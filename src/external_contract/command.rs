use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::requests::{CreateConditionsEvaluateJobRequest, CreateIngestJobRequest};
use crate::SchemaVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerActionType {
    IngestContract,
    EvaluateClauses,
    ProcessDocumentOcr,
    FullReprocess,
    RetryAction,
}

impl WorkerActionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IngestContract => "ingest_contract",
            Self::EvaluateClauses => "evaluate_clauses",
            Self::ProcessDocumentOcr => "process_document_ocr",
            Self::FullReprocess => "full_reprocess",
            Self::RetryAction => "retry_action",
        }
    }
}

impl std::str::FromStr for WorkerActionType {
    type Err = WorkerActionTypeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "ingest_contract" => Ok(Self::IngestContract),
            "evaluate_clauses" => Ok(Self::EvaluateClauses),
            "process_document_ocr" => Ok(Self::ProcessDocumentOcr),
            "full_reprocess" => Ok(Self::FullReprocess),
            "retry_action" => Ok(Self::RetryAction),
            _ => Err(WorkerActionTypeParseError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkerActionTypeParseError;

impl std::fmt::Display for WorkerActionTypeParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("invalid worker action type")
    }
}

impl std::error::Error for WorkerActionTypeParseError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCommandMessage {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub command_id: Uuid,
    pub correlation_id: Uuid,
    pub requested_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by_user_id: Option<String>,
    pub contract_id: Uuid,
    #[serde(flatten)]
    pub action: WorkerCommandAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action_type", content = "payload", rename_all = "snake_case")]
pub enum WorkerCommandAction {
    IngestContract(NauronJobCommand<CreateIngestJobRequest>),
    EvaluateClauses(NauronJobCommand<CreateConditionsEvaluateJobRequest>),
    ProcessDocumentOcr(ProcessDocumentOcrCommand),
    FullReprocess(FullReprocessCommand),
    RetryAction(RetryActionCommand),
}

impl WorkerCommandAction {
    pub fn action_type(&self) -> WorkerActionType {
        match self {
            Self::IngestContract(_) => WorkerActionType::IngestContract,
            Self::EvaluateClauses(_) => WorkerActionType::EvaluateClauses,
            Self::ProcessDocumentOcr(_) => WorkerActionType::ProcessDocumentOcr,
            Self::FullReprocess(_) => WorkerActionType::FullReprocess,
            Self::RetryAction(_) => WorkerActionType::RetryAction,
        }
    }

    pub fn payload_value(&self) -> Result<Value, serde_json::Error> {
        match self {
            Self::IngestContract(value) => serde_json::to_value(value),
            Self::EvaluateClauses(value) => serde_json::to_value(value),
            Self::ProcessDocumentOcr(value) => serde_json::to_value(value),
            Self::FullReprocess(value) => serde_json::to_value(value),
            Self::RetryAction(value) => serde_json::to_value(value),
        }
    }
}

impl WorkerCommandMessage {
    pub fn action_type(&self) -> WorkerActionType {
        self.action.action_type()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NauronJobCommand<T> {
    pub context_id: i32,
    pub request: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDocumentOcrCommand {
    pub document_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullReprocessCommand {
    pub kind: FullReprocessKind,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata_keys: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullReprocessKind {
    #[serde(rename = "full_reprocess_document")]
    Document,
    #[serde(rename = "ocr_prepare_contract_context")]
    OcrPrepareContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryActionCommand {
    pub kind: RetryActionKind,
    pub document_id: Uuid,
    pub pipeline_id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryActionKind {
    #[serde(rename = "ocr_reconcile_pipeline")]
    OcrReconcilePipeline,
}

#[derive(Debug)]
pub enum QueueCommandEnvelope {
    Valid {
        receipt_handle: String,
        command: Box<WorkerCommandMessage>,
    },
    Malformed {
        receipt_handle: String,
        raw_body: String,
    },
}
