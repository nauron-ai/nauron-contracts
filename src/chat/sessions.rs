use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::reasoning::ReasoningTrace;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SessionsQuery {
    pub context_id: Option<i64>,
    pub limit: Option<i64>,
    pub cursor_updated_at: Option<DateTime<Utc>>,
    pub cursor_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AdminSessionsQuery {
    pub context_id: Option<i64>,
    pub limit: Option<i64>,
    pub cursor_updated_at: Option<DateTime<Utc>>,
    pub cursor_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ReasoningQuery {
    pub message_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionResponse {
    pub session_id: Uuid,
    pub context_id: i64,
    pub kind: String,
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_message_at: DateTime<Utc>,
    pub message_count: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionCursor {
    pub updated_at: DateTime<Utc>,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsResponse {
    pub sessions: Vec<SessionResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<SessionCursor>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReasoningResponse {
    pub session_id: Uuid,
    pub message_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningTrace>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MessageMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningTrace>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionDetailResponse {
    pub session_id: Uuid,
    pub context_id: i64,
    pub kind: String,
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<MessageResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SourceReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SourceReference {
    pub doc_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}
