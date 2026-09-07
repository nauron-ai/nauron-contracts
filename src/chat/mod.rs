use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod search;
pub use search::*;

const MAX_HISTORY_MESSAGES: usize = 60;
const MAX_HISTORY_CHARACTERS: usize = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "graphql", graphql(rename_items = "lowercase"))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "juliette_chat_model", rename_all = "lowercase")
)]
#[serde(rename_all = "kebab-case")]
pub enum ChatModel {
    Gpt,
    Claude,
    #[serde(rename = "glm-5")]
    #[cfg_attr(feature = "sqlx", sqlx(rename = "glm-5"))]
    Glm5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    List(Vec<ChatValue>),
    Object(BTreeMap<String, ChatValue>),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatContract {
    pub contract_id: Uuid,
    pub context_id: Option<i64>,
    pub name: String,
    pub country_id: Option<Uuid>,
    pub metadata: BTreeMap<String, ChatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, tag = "type", rename_all = "snake_case")]
pub enum ChatScope {
    Contract { contract_id: Uuid },
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatRunRequest {
    pub run_id: Uuid,
    pub user_id: Uuid,
    pub model: ChatModel,
    pub scope: ChatScope,
    pub contracts: Vec<ChatContract>,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatSource {
    pub id: u32,
    pub contract_id: Option<Uuid>,
    pub document_id: Option<Uuid>,
    pub paragraph_id: Option<String>,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatRunResponse {
    pub run_id: Uuid,
    pub model: ChatModel,
    pub actual_model: String,
    pub answer: String,
    pub sources: Vec<ChatSource>,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub tool_calls: u32,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChatValidationError {
    #[error("chat scope contains no authorized contracts")]
    EmptyScope,
    #[error("chat scope contains invalid or duplicate identifiers")]
    InvalidScope,
    #[error("contract chat must contain exactly its selected contract")]
    ContractScopeMismatch,
    #[error("chat history must alternate user and assistant messages and end with a user message")]
    InvalidHistory,
    #[error("chat history exceeds the supported limit")]
    HistoryLimit,
}

impl ChatRunRequest {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        if self.contracts.is_empty() {
            return Err(ChatValidationError::EmptyScope);
        }
        let mut contract_ids = BTreeSet::new();
        let mut context_ids = BTreeSet::new();
        for contract in &self.contracts {
            if contract.contract_id.is_nil()
                || !contract_ids.insert(contract.contract_id)
                || contract
                    .context_id
                    .is_some_and(|id| id <= 0 || !context_ids.insert(id))
            {
                return Err(ChatValidationError::InvalidScope);
            }
        }
        if let ChatScope::Contract { contract_id } = self.scope
            && (self.contracts.len() != 1 || !contract_ids.contains(&contract_id))
        {
            return Err(ChatValidationError::ContractScopeMismatch);
        }
        if self.messages.len() > MAX_HISTORY_MESSAGES
            || self
                .messages
                .iter()
                .map(|message| message.content.chars().count())
                .sum::<usize>()
                > MAX_HISTORY_CHARACTERS
        {
            return Err(ChatValidationError::HistoryLimit);
        }
        if self.run_id.is_nil()
            || self.user_id.is_nil()
            || self.messages.is_empty()
            || self.messages.len().is_multiple_of(2)
            || self.messages.iter().enumerate().any(|(index, message)| {
                let role = if index.is_multiple_of(2) {
                    ChatRole::User
                } else {
                    ChatRole::Assistant
                };
                message.role != role || message.content.trim().is_empty()
            })
        {
            return Err(ChatValidationError::InvalidHistory);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
