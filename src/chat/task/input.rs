use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::chat::{ChatRole, ChatValidationError};

pub const MAX_TASK_INPUT_PAGE: u32 = 12_000;
pub const MAX_TASK_INPUT_HEADERS: u32 = 5;
pub const MAX_TASK_INPUT_PREVIEW: i32 = 160;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "input_operation",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum TaskInputQuery {
    List {
        offset: u32,
        limit: u32,
    },
    Read {
        message_id: Uuid,
        offset: u32,
        limit: u32,
    },
}

impl TaskInputQuery {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        let valid = match self {
            Self::List { limit, .. } => (1..=MAX_TASK_INPUT_HEADERS).contains(limit),
            Self::Read {
                message_id, limit, ..
            } => !message_id.is_nil() && (1..=MAX_TASK_INPUT_PAGE).contains(limit),
        };
        if !valid {
            return Err(ChatValidationError::InvalidAction);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskInputHeader {
    pub message_id: Uuid,
    pub preview: String,
    pub total_characters: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskInputPage {
    pub message_id: Uuid,
    pub role: ChatRole,
    pub content: String,
    pub offset: usize,
    pub total_characters: usize,
    pub has_more: bool,
}
