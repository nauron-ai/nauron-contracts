use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatSearchMode {
    Semantic,
    Exact,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatSearchRequest {
    pub context_ids: Vec<i64>,
    pub query: String,
    pub mode: ChatSearchMode,
    pub limit: u16,
    pub offset: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatSearchHit {
    pub context_id: i64,
    pub document_id: Uuid,
    pub paragraph_id: String,
    pub excerpt: String,
    pub score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatSearchResponse {
    pub hits: Vec<ChatSearchHit>,
    pub total_matches: Option<u64>,
}
