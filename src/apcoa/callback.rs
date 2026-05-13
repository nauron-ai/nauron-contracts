use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::conditions::ConditionsEvaluateEvent;
use crate::{IngestEvent, MirEvent, SchemaVersion};

pub const NAURON_CALLBACK_RECEIVED_MESSAGE_TYPE: &str = "nauron_callback_received";

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
