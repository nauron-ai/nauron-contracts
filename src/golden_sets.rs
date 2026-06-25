use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoldenSetPromptScope {
    Contract,
    Datapoint,
    Clause,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoldenSetPromptEvaluationStatus {
    Passed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateGoldenSetPromptCandidateRequest {
    pub golden_set_id: String,
    pub golden_set_version: i32,
    pub source_datapoint_refs: Vec<String>,
    pub prompt_family: String,
    pub prompt_type: String,
    pub prompt_scope: GoldenSetPromptScope,
    pub source_prompt_id: String,
    pub source_prompt_version: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetPromptCandidate {
    pub id: String,
    pub golden_set_id: String,
    pub golden_set_version: i32,
    pub source_datapoint_refs: Vec<String>,
    pub prompt_family: String,
    pub prompt_type: String,
    pub prompt_scope: GoldenSetPromptScope,
    pub source_prompt_id: String,
    pub source_prompt_version: String,
    pub generated_prompt_id: String,
    pub generated_prompt_version: String,
    pub content: String,
    pub content_hash: String,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateGoldenSetPromptCandidateRequest {
    pub candidate_id: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetPromptEvaluation {
    pub id: String,
    pub candidate_id: String,
    pub golden_set_id: String,
    pub status: GoldenSetPromptEvaluationStatus,
    pub metrics: Value,
    pub failed_datapoint_examples: Value,
    pub warnings: Vec<String>,
    pub blocking_issues: Vec<String>,
    pub evaluated_at: DateTime<Utc>,
    pub evaluator: String,
    pub evaluator_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateGoldenSetPromptCandidateRequest {
    pub candidate_id: String,
    pub idempotency_key: String,
    pub activated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSetPromptActivation {
    pub id: String,
    pub candidate_id: String,
    pub evaluation_id: String,
    pub golden_set_id: String,
    pub activated_prompt_id: String,
    pub activated_prompt_version: String,
    pub previous_active_version: Option<String>,
    pub activated_by: String,
    pub activated_at: DateTime<Utc>,
    pub golden_set_version: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoldenSetPromptHistory {
    pub candidates: Vec<GoldenSetPromptCandidate>,
    pub evaluations: Vec<GoldenSetPromptEvaluation>,
    pub activations: Vec<GoldenSetPromptActivation>,
}
