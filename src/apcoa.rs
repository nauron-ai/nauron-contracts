use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::conditions::{ConditionEvaluationOptions, ConditionSpec, ConditionsEvaluateEvent};
use crate::{IngestEvent, IngestSchemaField, MirEvent, SchemaVersion};

pub const NAURON_CALLBACK_RECEIVED_MESSAGE_TYPE: &str = "nauron_callback_received";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NauronCallbackStatus {
    InProgress,
    Success,
    Failure,
    Retryable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NauronCallbackEventType {
    MirProgress,
    MirResult,
    IngestProgress,
    IngestResult,
    ConditionsProgress,
    ConditionsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "engine", rename_all = "snake_case")]
pub enum NauronCallback {
    Mir {
        event_type: NauronCallbackEventType,
        status: NauronCallbackStatus,
        job_id: Uuid,
        context_id: i64,
        event: MirEvent,
    },
    Ingest {
        event_type: NauronCallbackEventType,
        status: NauronCallbackStatus,
        job_id: Uuid,
        context_id: i64,
        event: IngestEvent,
    },
    Conditions {
        event_type: NauronCallbackEventType,
        status: NauronCallbackStatus,
        job_id: Uuid,
        context_id: i64,
        event: ConditionsEvaluateEvent,
    },
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerResultStatus {
    InProgress,
    Completed,
    Failed,
}

impl WorkerResultStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

impl std::fmt::Display for WorkerResultStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerResultError {
    pub code: String,
    pub message: String,
    pub retriable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerResultEventMessage {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub event_id: Uuid,
    pub command_id: Uuid,
    pub correlation_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub contract_id: Uuid,
    #[serde(flatten)]
    pub action: WorkerResultAction,
    pub status: WorkerResultStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<WorkerResultError>,
}

impl WorkerResultEventMessage {
    pub fn in_progress(command: &WorkerCommandMessage, result: WorkerResultAction) -> Self {
        Self {
            schema_version: SchemaVersion::default(),
            event_id: Uuid::new_v4(),
            command_id: command.command_id,
            correlation_id: command.correlation_id,
            occurred_at: Utc::now(),
            contract_id: command.contract_id,
            action: result,
            status: WorkerResultStatus::InProgress,
            error: None,
        }
    }

    pub fn completed(command: &WorkerCommandMessage, result: WorkerResultAction) -> Self {
        Self {
            schema_version: SchemaVersion::default(),
            event_id: Uuid::new_v4(),
            command_id: command.command_id,
            correlation_id: command.correlation_id,
            occurred_at: Utc::now(),
            contract_id: command.contract_id,
            action: result,
            status: WorkerResultStatus::Completed,
            error: None,
        }
    }

    pub fn failed(
        command: &WorkerCommandMessage,
        code: &str,
        message: String,
        retriable: bool,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::default(),
            event_id: Uuid::new_v4(),
            command_id: command.command_id,
            correlation_id: command.correlation_id,
            occurred_at: Utc::now(),
            contract_id: command.contract_id,
            action: WorkerResultAction::empty_for(command.action_type()),
            status: WorkerResultStatus::Failed,
            error: Some(WorkerResultError {
                code: code.to_string(),
                message,
                retriable,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action_type", content = "result", rename_all = "snake_case")]
pub enum WorkerResultAction {
    IngestContract(Option<NauronJobResult<IngestEvent>>),
    EvaluateClauses(Option<NauronJobResult<ConditionsEvaluateEvent>>),
    ProcessDocumentOcr(Option<ProcessDocumentOcrResult>),
    FullReprocess(Option<FullReprocessResult>),
    RetryAction(Option<RetryActionResult>),
}

impl WorkerResultAction {
    pub fn action_type(&self) -> WorkerActionType {
        match self {
            Self::IngestContract(_) => WorkerActionType::IngestContract,
            Self::EvaluateClauses(_) => WorkerActionType::EvaluateClauses,
            Self::ProcessDocumentOcr(_) => WorkerActionType::ProcessDocumentOcr,
            Self::FullReprocess(_) => WorkerActionType::FullReprocess,
            Self::RetryAction(_) => WorkerActionType::RetryAction,
        }
    }

    pub fn empty_for(action_type: WorkerActionType) -> Self {
        match action_type {
            WorkerActionType::IngestContract => Self::IngestContract(None),
            WorkerActionType::EvaluateClauses => Self::EvaluateClauses(None),
            WorkerActionType::ProcessDocumentOcr => Self::ProcessDocumentOcr(None),
            WorkerActionType::FullReprocess => Self::FullReprocess(None),
            WorkerActionType::RetryAction => Self::RetryAction(None),
        }
    }

    pub fn result_value(&self) -> Result<Option<Value>, serde_json::Error> {
        match self {
            Self::IngestContract(value) => value.as_ref().map(serde_json::to_value).transpose(),
            Self::EvaluateClauses(value) => value.as_ref().map(serde_json::to_value).transpose(),
            Self::ProcessDocumentOcr(value) => value.as_ref().map(serde_json::to_value).transpose(),
            Self::FullReprocess(value) => value.as_ref().map(serde_json::to_value).transpose(),
            Self::RetryAction(value) => value.as_ref().map(serde_json::to_value).transpose(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NauronJobResult<E> {
    pub nauron_job_id: Uuid,
    pub context_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<E>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDocumentOcrResult {
    pub document_id: Uuid,
    pub context_id: i32,
    pub pipeline_id: Uuid,
    pub file_id: i64,
    pub doc_id: Uuid,
    pub nauron_job_id: Uuid,
    pub nauron_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullReprocessResult {
    pub kind: FullReprocessKind,
    pub context_id: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryActionResult {
    pub kind: RetryActionKind,
    pub document_id: Uuid,
    pub pipeline_id: Uuid,
    pub nauron_state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nauron_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NauronCallbackEventMessage {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub message_type: String,
    pub event_id: Uuid,
    pub received_at: DateTime<Utc>,
    pub nauron_job_id: Uuid,
    pub status: NauronCallbackStatus,
    pub callback: NauronCallback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkerResultQueueMessage {
    NauronCallback(NauronCallbackEventMessage),
    Result(WorkerResultEventMessage),
}

#[derive(Debug)]
pub enum QueueCommandEnvelope {
    Valid {
        receipt_handle: String,
        command: WorkerCommandMessage,
    },
    Malformed {
        receipt_handle: String,
        raw_body: String,
    },
}
