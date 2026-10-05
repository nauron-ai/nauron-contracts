use serde::{Deserialize, Serialize};

use super::ChatValidationError;

pub const MAX_CHAT_PROGRESS_ROUND: u16 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "graphql", graphql(rename_items = "snake_case"))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "juliette_chat_progress_stage", rename_all = "snake_case")
)]
pub enum ChatProgressStage {
    Queued,
    Preparing,
    Analyzing,
    Retrieving,
    GeneratingReport,
    Verifying,
    Saving,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatProgress {
    pub stage: ChatProgressStage,
    pub round: u16,
}

impl ChatProgress {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        if self.round > MAX_CHAT_PROGRESS_ROUND {
            return Err(ChatValidationError::InvalidAction);
        }
        Ok(())
    }
}
