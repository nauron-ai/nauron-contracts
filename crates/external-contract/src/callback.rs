use serde::{Deserialize, Serialize};
use uuid::Uuid;

use nauron_contracts::conditions::ConditionsEvaluateEvent;
use nauron_contracts::{IngestEvent, MirEvent};

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
    FileContextChanged {
        request_id: Uuid,
        snapshot: crate::file_contexts::FileContextSnapshot,
    },
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

impl NauronCallback {
    pub fn job_status(&self) -> Option<(uuid::Uuid, NauronCallbackStatus)> {
        match self {
            Self::Mir { job_id, status, .. }
            | Self::Ingest { job_id, status, .. }
            | Self::Conditions { job_id, status, .. } => Some((*job_id, *status)),
            Self::FileContextChanged { .. } => None,
        }
    }
}
