use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use super::*;

const RUNTIME_BUNDLE_ID: Uuid = Uuid::from_u128(0x11111111111111111111111111111111);
const NEXT_RUNTIME_BUNDLE_ID: Uuid = Uuid::from_u128(0x22222222222222222222222222222222);
const PROMPT_VERSION_ID: Uuid = Uuid::from_u128(0x33333333333333333333333333333333);
const TRANSITION_ID: Uuid = Uuid::from_u128(0x44444444444444444444444444444444);
const INVALIDATED_CANDIDATE_ID: Uuid = Uuid::from_u128(0x55555555555555555555555555555555);
const COMPONENT_ID: &str = "inferencer.ingest.system";
const TARGET_KEY: &str = "contract_type";
const COMPONENT_CONTENT: &str = "system";
const TARGET_CONTENT: &str = "target";

#[test]
fn transition_contract_preserves_complete_cas_and_result_snapshot() {
    let catalog = catalog();
    let expected_head = head(RUNTIME_BUNDLE_ID, 4, &catalog.runtime_hash, 8);
    let request = PromptRuntimeTransitionRequest {
        expected_head: expected_head.clone(),
        target_catalog: catalog.clone(),
        transition_reference: "global-cycle:42".to_string(),
        actor: "operator@example.com".to_string(),
        idempotency_key: "runtime-transition:42".to_string(),
    };
    let targets = targets();
    let next_bundle =
        PromptRuntimeBundle::new(NEXT_RUNTIME_BUNDLE_ID, 5, catalog.components.clone())
            .expect("valid runtime bundle");
    let manifest = CompositePromptManifestV2::new(next_bundle, targets.clone())
        .expect("valid composite manifest");
    let response = PromptRuntimeTransitionResponse {
        transition_id: TRANSITION_ID,
        previous_head: expected_head,
        head: head(NEXT_RUNTIME_BUNDLE_ID, 5, &catalog.runtime_hash, 9),
        active_targets: targets,
        invalidated_candidate_ids: vec![INVALIDATED_CANDIDATE_ID],
        manifest,
        transitioned_at: "2026-08-10T10:15:30Z"
            .parse::<DateTime<Utc>>()
            .expect("valid timestamp"),
    };

    assert_round_trip(&request);
    assert_round_trip(&response);
    assert_unknown_field(&request);
    assert_unknown_field(&response);
}

fn catalog() -> PromptRuntimeCatalog {
    let components = vec![
        PromptRuntimeComponent::new(
            COMPONENT_ID,
            PromptRuntimeStage::Ingest,
            PromptRuntimeRole::System,
            PromptActivationCondition::Always,
            10,
            COMPONENT_CONTENT,
        )
        .expect("valid runtime component"),
    ];
    PromptRuntimeCatalog {
        manifest_version: PROMPT_RUNTIME_MANIFEST_VERSION,
        runtime_hash: calculate_runtime_hash(&components),
        components,
    }
}

fn head(id: Uuid, version: i32, runtime_hash: &str, revision: i64) -> PromptRuntimeHead {
    PromptRuntimeHead {
        runtime_bundle_id: id,
        runtime_bundle_version: version,
        runtime_hash: runtime_hash.to_string(),
        revision,
    }
}

fn targets() -> BTreeMap<String, TargetPromptBinding> {
    let binding = TargetPromptBinding::from_content(PROMPT_VERSION_ID, 7, TARGET_CONTENT)
        .expect("valid target binding");
    BTreeMap::from([(TARGET_KEY.to_string(), binding)])
}

fn assert_round_trip<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let serialized = serde_json::to_value(value).expect("serialize contract value");
    let deserialized = serde_json::from_value(serialized).expect("deserialize contract value");
    assert_eq!(value, &deserialized);
}

fn assert_unknown_field<T: Serialize + DeserializeOwned>(value: &T) {
    let mut value = serde_json::to_value(value).expect("serialize contract value");
    value["unknown"] = serde_json::Value::Bool(true);
    assert!(serde_json::from_value::<T>(value).is_err());
}
