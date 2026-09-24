use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::{ChatArtifact, ChatDataPage, ChatDataQuery, ChatSource, ChatValidationError};

const MAX_ACTIONS: usize = 32;
const MAX_ACTION_NAME_BYTES: usize = 64;
const MAX_ACTION_TEXT_BYTES: usize = 100_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatActionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grounding_instruction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatActionCall {
    pub action: String,
    pub arguments: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ChatSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, tag = "status", rename_all = "snake_case")]
pub enum ChatActionResult {
    Success {
        data: String,
        artifact: Option<ChatArtifact>,
        contract_id: Option<Uuid>,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatDataRequest {
    Query(ChatDataQuery),
    Action(ChatActionCall),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatDataResponse {
    Query(ChatDataPage),
    Action(ChatActionResult),
}

impl ChatDataRequest {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        match self {
            Self::Query(query) => query.validate(),
            Self::Action(call) => call.validate(),
        }
    }
}

impl ChatActionCall {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        if !valid_name(&self.action)
            || !self.arguments.is_object()
            || self.arguments.to_string().len() > MAX_ACTION_TEXT_BYTES
            || self.evidence.as_ref().is_some_and(|source| {
                source.id == 0
                    || source.contract_id.is_none_or(|id| id.is_nil())
                    || !valid_text(&source.excerpt)
            })
        {
            return Err(ChatValidationError::InvalidAction);
        }
        Ok(())
    }
}

pub fn validate_chat_actions(actions: &[ChatActionDefinition]) -> Result<(), ChatValidationError> {
    if actions.len() > MAX_ACTIONS {
        return Err(ChatValidationError::InvalidAction);
    }
    let mut names = BTreeSet::new();
    if actions.iter().any(|action| {
        !valid_name(&action.name)
            || !names.insert(&action.name)
            || !valid_text(&action.description)
            || !action.parameters.is_object()
            || action.parameters.to_string().len() > MAX_ACTION_TEXT_BYTES
            || action
                .grounding_instruction
                .as_ref()
                .is_some_and(|text| !valid_text(text))
    }) {
        return Err(ChatValidationError::InvalidAction);
    }
    Ok(())
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= MAX_ACTION_NAME_BYTES
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn valid_text(text: &str) -> bool {
    !text.trim().is_empty() && text.len() <= MAX_ACTION_TEXT_BYTES
}

#[cfg(test)]
#[path = "actions_tests.rs"]
mod tests;
