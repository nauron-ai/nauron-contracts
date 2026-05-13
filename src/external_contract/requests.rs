use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::IngestSchemaField;
use crate::conditions::{ConditionEvaluationOptions, ConditionSpec};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CallbackTarget {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateIngestJobRequest {
    pub schema: Vec<IngestSchemaField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback: Option<CallbackTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateConditionsEvaluateJobRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_doc_id: Option<Uuid>,
    pub conditions: Vec<ConditionSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ConditionEvaluationOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback: Option<CallbackTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateJobResponse {
    pub job_id: Uuid,
    pub job_status_url: String,
}
