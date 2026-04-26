use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct VectorSearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<Uuid>,
    #[serde(default = "default_target_paragraph")]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

pub struct VectorTarget;
impl VectorTarget {
    pub const PARAGRAPH: &'static str = "paragraph";
    pub const DOCUMENT: &'static str = "document";
    pub const CHAT: &'static str = "chat_history";
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct VectorSearchResponse {
    pub items: Vec<VectorSearchItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct VectorSearchItem {
    pub doc_id: Uuid,
    pub paragraph_id: Option<String>,
    pub snippet: Option<String>,
    pub lang: Option<String>,
    pub score: f32,
    pub iri: String,
    pub context_id: String,
    pub title: Option<String>,
    pub point_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SemanticSearchFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SemanticSearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SemanticSearchFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_entity_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SemanticSearchResponse {
    pub results: Vec<SemanticSearchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SemanticSearchResult {
    pub id: String,
    pub score: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_links: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfFactsRequest {
    pub context_id: i64,
    pub doc_id: Option<Uuid>,
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_entities: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_relations: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_claims: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contradictions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_similarity_edges: Option<u32>,
    pub max_paragraphs: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfFactsResponse {
    pub entities: Vec<RdfEntity>,
    pub relations: Vec<RdfRelation>,
    #[serde(default)]
    pub claims: Vec<RdfClaim>,
    #[serde(default)]
    pub contradictions: Vec<RdfContradiction>,
    #[serde(default)]
    pub similarity_edges: Vec<RdfSimilarityEdge>,
    pub paragraphs: Vec<RdfParagraph>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfClaimCondition {
    pub predicate: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfClaim {
    pub iri: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub conditions: Vec<RdfClaimCondition>,
    #[serde(default)]
    pub evidence_paragraphs: Vec<EvidencePointer>,
    #[serde(default)]
    pub doc_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfContradiction {
    pub iri: String,
    pub source_claim: String,
    pub target_claim: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(default)]
    pub evidence_paragraphs: Vec<EvidencePointer>,
    #[serde(default)]
    pub doc_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfSimilarityEdge {
    pub iri: String,
    pub source_entity: String,
    pub target_entity: String,
    pub score: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[serde(default)]
    pub evidence_paragraphs: Vec<EvidencePointer>,
    #[serde(default)]
    pub doc_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfEntity {
    pub iri: String,
    pub label: String,
    pub types: Vec<String>,
    pub doc_ids: Vec<Uuid>,
    pub popularity: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EvidencePointer {
    pub doc_id: Uuid,
    pub paragraph_id: String,
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfRelation {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub justification: Option<String>,
    pub confidence: Option<f32>,
    pub evidence_paragraphs: Vec<EvidencePointer>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RdfParagraph {
    pub doc_id: Uuid,
    pub paragraph_id: String,
    pub text: String,
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ParagraphContextRequest {
    pub context_id: i64,
    pub doc_id: Uuid,
    pub paragraph_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ParagraphContextResponse {
    pub paragraphs: Vec<RdfParagraph>,
}

fn default_target_paragraph() -> String {
    "paragraph".to_string()
}
