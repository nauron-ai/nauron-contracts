use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use super::PromptTuningPolicy;
use crate::{CompositePromptManifestV2, PromptRuntimeBundle};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievalChunk {
    pub point_id: String,
    pub document_id: String,
    pub paragraph_id: Option<String>,
    pub score: f64,
    pub content: String,
    pub content_hash: String,
    pub retrieval_query: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct FrozenTuningExample {
    pub golden_set_id: String,
    pub golden_set_version: i32,
    pub target_key: String,
    pub expected_value: Value,
    pub expected_value_hash: String,
    pub expected_value_content_hash: String,
    pub baseline_value: Value,
    pub baseline_value_hash: String,
    pub baseline_score: f64,
    pub baseline_differences: Vec<String>,
    pub comparator_version: String,
    pub source_contract_id: String,
    pub source_contract_version: i32,
    pub source_contract_hash: String,
    pub context_id: String,
    pub document_ids: Vec<String>,
    pub source_snapshot_hash: String,
    pub prompt_snapshot_refs: Vec<String>,
    pub source_ingest_job_id: String,
    pub rdf_graph_id: String,
    pub rdf_version: String,
    pub retrieval_queries: Vec<String>,
    pub chunks: Vec<RetrievalChunk>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GenerationContext {
    pub target_key: String,
    pub qdrant_collection: String,
    pub retrieval_snapshot_hash: String,
    pub context_char_count: i64,
    pub estimated_context_tokens: i64,
    pub chars_per_token: i64,
    pub max_context_tokens: i64,
    pub examples: Vec<FrozenTuningExample>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GeneratePromptCandidatesRequest {
    pub cycle_id: String,
    pub target_key: String,
    pub tuning_policy: PromptTuningPolicy,
    pub prompt_family: String,
    pub prompt_type: String,
    pub source_prompt_id: String,
    pub source_prompt_version: String,
    pub source_prompt_content: String,
    pub source_prompt_hash: String,
    pub composite_manifest: CompositePromptManifestV2,
    pub type_spec: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    pub candidate_count: u32,
    pub actor: String,
    pub idempotency_key: String,
    pub generation_context: GenerationContext,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GeneratedPromptCandidate {
    pub id: String,
    pub candidate_index: u32,
    pub content: String,
    pub prompt_hash: String,
    pub change_description: String,
    pub generator_input_hash: String,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GeneratePromptCandidatesResponse {
    pub generator_model: String,
    pub generator_route: String,
    pub generator_system_prompt: String,
    pub generator_system_prompt_hash: String,
    pub duration_ms: u64,
    pub context_snapshot_hash: String,
    pub runtime_bundle: PromptRuntimeBundle,
    pub candidates: Vec<GeneratedPromptCandidate>,
}
