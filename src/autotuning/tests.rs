use std::collections::BTreeMap;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use uuid::Uuid;

use super::*;
use crate::{
    CompositePromptManifestV2, PromptActivationCondition, PromptRuntimeBundle,
    PromptRuntimeComponent, PromptRuntimeRole, PromptRuntimeStage, TargetPromptBinding, sha256_hex,
};

const HASHED_CONTENT: &str = "content";
const TARGET_KEY: &str = "revenue";

#[test]
fn target_policy_has_one_allowed_component() {
    let policy = PromptTuningPolicy::target();

    assert!(policy.is_target());
    assert_eq!(policy.tuning_scope, PromptTuningScope::Target);
    assert_eq!(policy.allowed_component_ids, [TARGET_PROMPT_COMPONENT_ID]);
}

#[test]
fn generation_contract_rejects_unknown_fields() {
    assert_unknown_field(&generation_request());
    assert_unknown_field(&generation_response());
}

#[test]
fn execution_contract_rejects_unknown_fields() {
    assert_unknown_field(&execution_request());
    assert_unknown_field(&execution_response());
}

fn generation_request() -> GeneratePromptCandidatesRequest {
    GeneratePromptCandidatesRequest {
        cycle_id: "cycle".to_string(),
        target_key: TARGET_KEY.to_string(),
        tuning_policy: PromptTuningPolicy::target(),
        prompt_family: "datapoint_extraction".to_string(),
        prompt_type: "normalized_prompt".to_string(),
        source_prompt_id: "prompt".to_string(),
        source_prompt_version: "1".to_string(),
        source_prompt_content: HASHED_CONTENT.to_string(),
        source_prompt_hash: sha256_hex(HASHED_CONTENT.as_bytes()),
        composite_manifest: manifest(),
        type_spec: json!({"type": "string"}),
        candidate_count: 1,
        actor: "actor".to_string(),
        idempotency_key: "generation".to_string(),
        generation_context: GenerationContext {
            target_key: TARGET_KEY.to_string(),
            qdrant_collection: "collection".to_string(),
            retrieval_snapshot_hash: sha256_hex(b"snapshot"),
            context_char_count: 7,
            estimated_context_tokens: 2,
            chars_per_token: 4,
            max_context_tokens: 10,
            examples: vec![example()],
        },
    }
}

fn example() -> FrozenTuningExample {
    FrozenTuningExample {
        golden_set_id: "set".to_string(),
        golden_set_version: 1,
        target_key: TARGET_KEY.to_string(),
        expected_value: json!("expected"),
        expected_value_hash: sha256_hex(b"expected"),
        expected_value_content_hash: sha256_hex(b"expected-content"),
        baseline_value: json!("baseline"),
        baseline_value_hash: sha256_hex(b"baseline"),
        baseline_score: 1.0,
        baseline_differences: Vec::new(),
        comparator_version: "1".to_string(),
        source_contract_id: "contract".to_string(),
        source_contract_version: 1,
        source_contract_hash: sha256_hex(b"contract"),
        context_id: "1".to_string(),
        document_ids: vec!["document".to_string()],
        source_snapshot_hash: sha256_hex(b"source"),
        prompt_snapshot_refs: vec!["prompt".to_string()],
        source_ingest_job_id: Uuid::new_v4().to_string(),
        rdf_graph_id: "graph".to_string(),
        rdf_version: "1".to_string(),
        retrieval_queries: vec!["query".to_string()],
        chunks: vec![RetrievalChunk {
            point_id: "point".to_string(),
            document_id: "document".to_string(),
            paragraph_id: Some("paragraph".to_string()),
            score: 1.0,
            content: HASHED_CONTENT.to_string(),
            content_hash: sha256_hex(HASHED_CONTENT.as_bytes()),
            retrieval_query: "query".to_string(),
        }],
    }
}

fn generation_response() -> GeneratePromptCandidatesResponse {
    GeneratePromptCandidatesResponse {
        generator_model: "model".to_string(),
        generator_route: "route".to_string(),
        generator_system_prompt: "system".to_string(),
        generator_system_prompt_hash: sha256_hex(b"system"),
        duration_ms: 1,
        context_snapshot_hash: sha256_hex(b"snapshot"),
        runtime_bundle: manifest().runtime_bundle,
        candidates: vec![GeneratedPromptCandidate {
            id: "candidate".to_string(),
            candidate_index: 0,
            content: HASHED_CONTENT.to_string(),
            prompt_hash: sha256_hex(HASHED_CONTENT.as_bytes()),
            change_description: "change".to_string(),
            generator_input_hash: sha256_hex(b"input"),
            prompt_tokens: Some(1),
            completion_tokens: Some(1),
            total_tokens: Some(2),
        }],
    }
}

fn execution_request() -> ExecuteFrozenPromptsRequest {
    ExecuteFrozenPromptsRequest {
        mode: FrozenPromptExecutionMode::BaselineExecution,
        target_key: TARGET_KEY.to_string(),
        type_spec: json!({"type": "string"}),
        language: "en".to_string(),
        source: FrozenPromptSourceIdentity {
            source_ingest_job_id: Uuid::new_v4().to_string(),
            context_id: "1".to_string(),
            document_ids: vec!["document".to_string()],
            source_snapshot_hash: sha256_hex(b"source"),
        },
        composite_manifest: manifest(),
        prompts: vec![FrozenPrompt {
            role: FrozenPromptRole::Baseline,
            prompt_id: "prompt".to_string(),
            prompt_version: "1".to_string(),
            content: HASHED_CONTENT.to_string(),
            prompt_hash: sha256_hex(HASHED_CONTENT.as_bytes()),
        }],
    }
}

fn execution_response() -> ExecuteFrozenPromptsResponse {
    ExecuteFrozenPromptsResponse {
        mode: FrozenPromptExecutionMode::BaselineExecution,
        source_snapshot_hash: sha256_hex(b"source"),
        model_route: "route".to_string(),
        language: "en".to_string(),
        duration_ms: 1,
        executions: vec![FrozenPromptExecutionResult {
            role: FrozenPromptRole::Baseline,
            prompt_id: "prompt".to_string(),
            prompt_version: "1".to_string(),
            prompt_hash: sha256_hex(HASHED_CONTENT.as_bytes()),
            composite_hash: manifest().composite_hash,
            status: FrozenPromptExecutionStatus::Succeeded,
            value: json!("value"),
            prompt_tokens: Some(1),
            completion_tokens: Some(1),
            total_tokens: Some(2),
            error: None,
        }],
    }
}

fn manifest() -> CompositePromptManifestV2 {
    let component = PromptRuntimeComponent::new(
        "inferencer.ingest.system",
        PromptRuntimeStage::Ingest,
        PromptRuntimeRole::System,
        PromptActivationCondition::Always,
        1,
        "system",
    )
    .expect("valid component");
    let runtime =
        PromptRuntimeBundle::new(Uuid::new_v4(), 1, vec![component]).expect("valid runtime bundle");
    let binding = TargetPromptBinding::from_content(Uuid::new_v4(), 1, HASHED_CONTENT)
        .expect("valid target binding");
    CompositePromptManifestV2::new(runtime, BTreeMap::from([(TARGET_KEY.to_string(), binding)]))
        .expect("valid manifest")
}

fn assert_unknown_field<T: Serialize + DeserializeOwned>(value: &T) {
    let mut value = serde_json::to_value(value).expect("serializable contract");
    value["unknown"] = json!(true);
    assert!(serde_json::from_value::<T>(value).is_err());
}
