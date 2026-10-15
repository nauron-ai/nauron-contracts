use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    MAX_TASK_EVIDENCE_BYTES, TaskEvidence, TaskEvidenceRef, TaskMessage, TaskSnapshot, TaskState,
};
use crate::chat::ChatValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskTransition {
    Start,
    Continue,
    Resume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "juliette_task_stop_reason", rename_all = "snake_case")
)]
pub enum TaskStopReason {
    Completed,
    NeedsInput,
    NoProgress,
    BudgetExhausted,
    Running,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "task_operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChatTaskRequest {
    Load,
    Inspect {
        task_id: Uuid,
    },
    Select {
        transition: TaskTransition,
        task_id: Option<Uuid>,
        expected_revision: Option<i64>,
        state: TaskState,
    },
    Checkpoint {
        task_id: Uuid,
        expected_revision: i64,
        operation_id: Uuid,
        state: TaskState,
        evidence: Vec<TaskEvidence>,
        stop_reason: TaskStopReason,
    },
    Evidence {
        task_id: Uuid,
        reference: TaskEvidenceRef,
    },
    Search {
        query: String,
    },
}

impl ChatTaskRequest {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        match self {
            Self::Select { state, .. } | Self::Checkpoint { state, .. } => state.validate()?,
            Self::Search { query } if query.trim().is_empty() || query.len() > 200 => {
                return Err(ChatValidationError::InvalidAction);
            }
            _ => {}
        }
        if serde_json::to_vec(self)
            .map_err(|_| ChatValidationError::InvalidAction)?
            .len()
            > MAX_TASK_EVIDENCE_BYTES
        {
            return Err(ChatValidationError::InvalidAction);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskHeader {
    pub id: Uuid,
    pub goal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "task_result", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChatTaskResponse {
    Loaded {
        active: Option<TaskSnapshot>,
        scope_changed: bool,
        current_message: TaskMessage,
        recent_messages: Vec<TaskMessage>,
    },
    Selected {
        snapshot: TaskSnapshot,
        messages: Vec<TaskMessage>,
    },
    Saved {
        snapshot: TaskSnapshot,
    },
    Evidence {
        evidence: TaskEvidence,
    },
    Tasks {
        tasks: Vec<TaskHeader>,
    },
}
