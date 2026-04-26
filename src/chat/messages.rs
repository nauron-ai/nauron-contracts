use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::vector::RdfFactsResponse;
use crate::vector::VectorSearchItem;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ChatMessageRequest {
    pub session_id: Option<Uuid>,
    pub context_id: i64,
    pub message: String,
    pub k: Option<u16>,
    pub lang: Option<String>,
    #[serde(default = "default_stream_true")]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatCompletionResponse {
    pub session_id: Uuid,
    pub context_id: i64,
    pub user_id: String,
    pub kind: ChatKind,
    pub answer: String,
    pub confidence: f32,
    pub sources: Vec<VectorSearchItem>,
    pub messages: Vec<MessageTurn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_facts: Option<RdfFactsResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatKind {
    Emb,
    RdfEmb,
    BnRdfEmb,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct MessageTurn {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ChatCreatedEvent {
    pub session_id: Uuid,
    pub kind: ChatKind,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct VectorSourcesEvent {
    pub items: Vec<VectorSearchItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TokenDelta {
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ChatCompletedEvent {
    pub answer: String,
    pub confidence: f32,
    pub sources: Vec<VectorSearchItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_facts: Option<RdfFactsResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SseExample {
    #[schema(
        example = "event: chat.created\\ndata: {\"session_id\":\"11111111-2222-3333-4444-555555555555\",\"kind\":\"emb\"}\\n\\nevent: llm.delta\\ndata: {\"delta\":\"partial answer\"}\\n\\nevent: chat.completed\\ndata: {\"answer\":\"...\",\"confidence\":0.72,\"sources\":[]}"
    )]
    pub example: String,
}

fn default_stream_true() -> Option<bool> {
    Some(true)
}
