use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromptLog {
    pub model: String,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<PromptMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<PromptToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<PromptToolChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromptMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<PromptToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromptToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: PromptFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromptFunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromptToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: PromptFunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromptFunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum PromptToolChoice {
    Mode(String),
    Specific {
        r#type: String,
        function: PromptToolChoiceFunction,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromptToolChoiceFunction {
    pub name: String,
}
