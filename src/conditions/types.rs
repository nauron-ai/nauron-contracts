use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

pub type ConditionParameters = Value;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConditionContextMode {
    Emb,
    Rdf,
    Lpg,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Negligible,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionVerdict {
    Satisfied,
    Unsatisfied,
    Unknown,
    Conflict,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionSpec {
    pub id: String,
    pub label: String,
    pub description: String,
    #[serde(default)]
    pub parameters: ConditionParameters,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionEvaluationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_candidates: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hops: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionEvaluationRequest {
    pub context_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_doc_id: Option<Uuid>,
    pub conditions: Vec<ConditionSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ConditionEvaluationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_mode: Option<ConditionContextMode>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionMatch {
    pub doc_id: Uuid,
    pub paragraph_id: String,
    pub snippet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionRawEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdf_entities: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdf_relations: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionEvaluationResult {
    pub condition_id: String,
    pub verdict: ConditionVerdict,
    pub satisfied: bool,
    pub confidence: f32,
    pub evidence_strength: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<RiskLevel>,
    #[serde(default)]
    pub matches: Vec<ConditionMatch>,
    pub reasoning: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_evidence: Option<ConditionRawEvidence>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionEvaluationResponse {
    pub context_id: i64,
    pub results: Vec<ConditionEvaluationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConditionEvaluationErrorCode {
    ValidationError,
    ContextNotFound,
    VectorApiError,
    LlmError,
    InternalError,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionErrorPayload {
    pub code: ConditionEvaluationErrorCode,
    pub message: String,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub details: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ConditionErrorResponse {
    pub error: ConditionErrorPayload,
}
