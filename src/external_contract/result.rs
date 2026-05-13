use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::callback::NauronCallbackEventMessage;
use super::command::{FullReprocessKind, RetryActionKind, WorkerActionType, WorkerCommandMessage};
use crate::conditions::ConditionsEvaluateEvent;
use crate::{IngestEvent, SchemaVersion};

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
#[serde(untagged)]
pub enum WorkerResultQueueMessage {
    NauronCallback(NauronCallbackEventMessage),
    Result(WorkerResultEventMessage),
}
