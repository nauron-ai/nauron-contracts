use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::CompositePromptManifestV2;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrozenPromptExecutionMode {
    BaselineExecution,
    SelectionEvaluation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrozenPromptRole {
    Baseline,
    Candidate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrozenPromptExecutionStatus {
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct FrozenPromptSourceIdentity {
    pub source_ingest_job_id: String,
    pub context_id: String,
    pub document_ids: Vec<String>,
    pub source_snapshot_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct FrozenPrompt {
    pub role: FrozenPromptRole,
    pub prompt_id: String,
    pub prompt_version: String,
    pub content: String,
    pub prompt_hash: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecuteFrozenPromptsRequest {
    pub mode: FrozenPromptExecutionMode,
    pub target_key: String,
    pub type_spec: Value,
    pub language: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    pub source: FrozenPromptSourceIdentity,
    pub composite_manifest: CompositePromptManifestV2,
    pub prompts: Vec<FrozenPrompt>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct FrozenPromptExecutionResult {
    pub role: FrozenPromptRole,
    pub prompt_id: String,
    pub prompt_version: String,
    pub prompt_hash: String,
    pub composite_hash: String,
    pub status: FrozenPromptExecutionStatus,
    pub value: Value,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecuteFrozenPromptsResponse {
    pub mode: FrozenPromptExecutionMode,
    pub source_snapshot_hash: String,
    pub model_route: String,
    pub adjudication_model_route: String,
    pub language: String,
    pub duration_ms: u64,
    pub executions: Vec<FrozenPromptExecutionResult>,
}
