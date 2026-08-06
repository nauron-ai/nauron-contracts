use std::collections::BTreeMap;

use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use super::*;

const RUNTIME_BUNDLE_ID: Uuid = Uuid::from_u128(0x11111111111111111111111111111111);
const PROMPT_VERSION_ID: Uuid = Uuid::from_u128(0x22222222222222222222222222222222);
const SYSTEM_COMPONENT_ID: &str = "inferencer.ingest.system";
const USER_COMPONENT_ID: &str = "inferencer.ingest.user";
const TARGET_KEY: &str = "revenue";
const SYSTEM_CONTENT: &str = "system";
const EXPECTED_RUNTIME_HASH: &str =
    "29059d0a03b3e930868486e71a02674a5f612784ed5839f3eba012eb0b0dd392";
const EXPECTED_COMPOSITE_HASH: &str =
    "585ed533951d0e7ac80083953b1b885cc4fd1346e96046583d3d8725864b9cb6";

#[test]
fn target_change_preserves_runtime_hash() {
    let runtime = runtime_bundle(vec![component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT)]);
    let first = manifest(runtime.clone(), targets(&[(TARGET_KEY, "first")]));
    let second = manifest(runtime, targets(&[(TARGET_KEY, "second")]));

    assert_eq!(
        first.runtime_bundle.runtime_hash,
        second.runtime_bundle.runtime_hash
    );
    assert_ne!(first.composite_hash, second.composite_hash);
}

#[test]
fn global_change_preserves_target_hash() {
    let binding = target_binding("target");
    let first = manifest(
        runtime_bundle(vec![component(SYSTEM_COMPONENT_ID, 10, "first")]),
        BTreeMap::from([(TARGET_KEY.to_string(), binding.clone())]),
    );
    let second = manifest(
        runtime_bundle(vec![component(SYSTEM_COMPONENT_ID, 10, "second")]),
        BTreeMap::from([(TARGET_KEY.to_string(), binding)]),
    );

    assert_eq!(
        first.targets[TARGET_KEY].prompt_hash,
        second.targets[TARGET_KEY].prompt_hash
    );
    assert_ne!(
        first.runtime_bundle.runtime_hash,
        second.runtime_bundle.runtime_hash
    );
    assert_ne!(first.composite_hash, second.composite_hash);
}

#[test]
fn component_and_target_order_is_deterministic() {
    let first_runtime = runtime_bundle(vec![
        component(USER_COMPONENT_ID, 20, "user"),
        component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT),
    ]);
    let second_runtime = runtime_bundle(vec![
        component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT),
        component(USER_COMPONENT_ID, 20, "user"),
    ]);
    let mut first_targets = BTreeMap::new();
    first_targets.insert(TARGET_KEY.to_string(), target_binding(TARGET_KEY));
    first_targets.insert("costs".to_string(), target_binding("costs"));
    let mut second_targets = BTreeMap::new();
    second_targets.insert("costs".to_string(), target_binding("costs"));
    second_targets.insert(TARGET_KEY.to_string(), target_binding(TARGET_KEY));

    let first = manifest(first_runtime, first_targets);
    let second = manifest(second_runtime, second_targets);

    assert_eq!(
        first.runtime_bundle.runtime_hash,
        second.runtime_bundle.runtime_hash
    );
    assert_eq!(first.composite_hash, second.composite_hash);
}

#[test]
fn malformed_hashes_are_rejected() {
    let mut runtime = runtime_bundle(vec![component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT)]);
    runtime.runtime_hash = "ABC".repeat(21) + "A";
    assert_eq!(
        runtime.validate(),
        Err(PromptRuntimeError::InvalidHash("runtime_hash"))
    );

    let binding = TargetPromptBinding::new(Uuid::new_v4(), 1, "0".repeat(64), Some("A".repeat(64)));
    assert_eq!(
        binding,
        Err(PromptRuntimeError::InvalidHash("source_bundle_hash"))
    );

    let mut component = component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT);
    component.content_hash = "0".repeat(64);
    assert!(matches!(
        component.validate(),
        Err(PromptRuntimeError::ComponentContentHashMismatch(_))
    ));
}

#[test]
fn duplicate_components_are_rejected() {
    let duplicate = component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT);
    let result = PromptRuntimeBundle::new(Uuid::new_v4(), 1, vec![duplicate.clone(), duplicate]);

    assert_eq!(
        result,
        Err(PromptRuntimeError::DuplicateComponentId(
            SYSTEM_COMPONENT_ID.to_string()
        ))
    );
}

#[test]
fn non_inferencer_components_are_rejected() {
    let result = PromptRuntimeComponent::new(
        "apcoa.datapoint.normalized_prompt",
        PromptRuntimeStage::Ingest,
        PromptRuntimeRole::User,
        PromptActivationCondition::Always,
        10,
        "target",
    );

    assert_eq!(
        result,
        Err(PromptRuntimeError::InvalidComponentId(
            "apcoa.datapoint.normalized_prompt".to_string()
        ))
    );
}

#[test]
fn invalid_identity_versions_targets_and_composite_are_rejected() {
    let runtime_component = component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT);
    assert_eq!(
        PromptRuntimeBundle::new(Uuid::nil(), 1, vec![runtime_component.clone()]),
        Err(PromptRuntimeError::NilRuntimeBundleId)
    );
    assert_eq!(
        PromptRuntimeBundle::new(Uuid::new_v4(), 0, vec![runtime_component]),
        Err(PromptRuntimeError::InvalidRuntimeBundleVersion)
    );
    assert_eq!(
        TargetPromptBinding::from_content(Uuid::nil(), 1, "target", None),
        Err(PromptRuntimeError::NilPromptVersionId)
    );
    assert_eq!(
        TargetPromptBinding::from_content(PROMPT_VERSION_ID, 0, "target", None),
        Err(PromptRuntimeError::InvalidPromptVersion)
    );

    let runtime = runtime_bundle(vec![component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT)]);
    assert_eq!(
        CompositePromptManifestV2::new(
            runtime.clone(),
            BTreeMap::from([(" revenue".to_string(), target_binding("target"))]),
        ),
        Err(PromptRuntimeError::InvalidTargetKey(" revenue".to_string()))
    );

    let mut manifest = manifest(runtime, targets(&[("revenue", "target")]));
    manifest.manifest_version = 0;
    assert_eq!(
        manifest.validate(),
        Err(PromptRuntimeError::InvalidManifestVersion(0))
    );
    manifest.manifest_version = PROMPT_RUNTIME_MANIFEST_VERSION;
    manifest.composite_hash = "0".repeat(64);
    assert_eq!(
        manifest.validate(),
        Err(PromptRuntimeError::CompositeHashMismatch)
    );
}

#[test]
fn canonical_hash_vectors_are_stable() {
    let runtime = runtime_bundle(vec![component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT)]);
    let binding =
        TargetPromptBinding::from_content(PROMPT_VERSION_ID, 7, "target", Some("a".repeat(64)))
            .expect("valid target binding");
    let manifest = manifest(runtime, BTreeMap::from([(TARGET_KEY.to_string(), binding)]));

    assert_eq!(manifest.runtime_bundle.runtime_hash, EXPECTED_RUNTIME_HASH);
    assert_eq!(manifest.composite_hash, EXPECTED_COMPOSITE_HASH);
}

#[test]
fn unknown_contract_fields_are_rejected_at_every_level() {
    let component = component(SYSTEM_COMPONENT_ID, 10, SYSTEM_CONTENT);
    let runtime = runtime_bundle(vec![component.clone()]);
    let binding = target_binding("target");
    let manifest = manifest(runtime.clone(), targets(&[(TARGET_KEY, "target")]));

    assert_unknown_field(&component);
    assert_unknown_field(&runtime);
    assert_unknown_field(&binding);
    assert_unknown_field(&manifest);
}

fn assert_unknown_field<T: Serialize + DeserializeOwned>(value: &T) {
    let mut value = serde_json::to_value(value).expect("serialize contract value");
    value["unknown"] = serde_json::Value::Bool(true);
    assert!(serde_json::from_value::<T>(value).is_err());
}

fn component(id: &str, execution_order: i32, content: &str) -> PromptRuntimeComponent {
    PromptRuntimeComponent::new(
        id,
        PromptRuntimeStage::Ingest,
        PromptRuntimeRole::System,
        PromptActivationCondition::Always,
        execution_order,
        content,
    )
    .expect("valid runtime component")
}

fn runtime_bundle(components: Vec<PromptRuntimeComponent>) -> PromptRuntimeBundle {
    PromptRuntimeBundle::new(RUNTIME_BUNDLE_ID, 1, components).expect("valid runtime bundle")
}

fn target_binding(content: &str) -> TargetPromptBinding {
    TargetPromptBinding::from_content(PROMPT_VERSION_ID, 1, content, None)
        .expect("valid target binding")
}

fn targets(values: &[(&str, &str)]) -> BTreeMap<String, TargetPromptBinding> {
    values
        .iter()
        .map(|(key, content)| (key.to_string(), target_binding(content)))
        .collect()
}

fn manifest(
    runtime_bundle: PromptRuntimeBundle,
    targets: BTreeMap<String, TargetPromptBinding>,
) -> CompositePromptManifestV2 {
    CompositePromptManifestV2::new(runtime_bundle, targets).expect("valid composite manifest")
}
