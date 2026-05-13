use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KnowledgeArtifact {
    pub dossier: DossierArtifact,
    pub compiled_knowledge_view: CompiledKnowledgeView,
    pub timeline_view: TimelineView,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DossierArtifact {
    pub id: Uuid,
    pub context_id: i64,
    pub name: String,
    pub role: DossierRole,
    pub scope: DossierScope,
    pub revision: i32,
    pub metadata: DossierMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DossierRole {
    #[serde(rename = "runtime_knowledge_compiler")]
    RuntimeKnowledgeCompiler,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DossierScope {
    Context,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DossierMetadata {
    pub require_conflicts_with: bool,
    pub max_conflict_nodes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TimelineView {
    pub nodes: Vec<TimelineNode>,
    pub edges: Vec<TimelineEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TimelineNode {
    pub id: String,
    pub kind: TimelineNodeKind,
    pub status: TimelineNodeStatus,
    pub effective_from: Option<NaiveDate>,
    pub effective_to: Option<NaiveDate>,
    pub evidence: Vec<EvidenceAnchor>,
    pub label: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TimelineNodeKind {
    Document,
    Change,
    Conflict,
    Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TimelineNodeStatus {
    Active,
    Historical,
    Superseded,
    Conflicting,
    Informational,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TimelineEdge {
    pub source: String,
    pub target: String,
    pub kind: TimelineEdgeKind,
    pub evidence: Vec<EvidenceAnchor>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TimelineEdgeKind {
    Amends,
    Supersedes,
    ConflictsWith,
    Supports,
    DerivedFrom,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct EvidenceAnchor {
    pub doc_id: Uuid,
    pub paragraph_id: String,
    pub quote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CompiledKnowledgeView {
    pub dossier_name: String,
    pub brief: String,
    pub active_surfaces: Vec<KnowledgeHint>,
    pub temporal_hints: Vec<KnowledgeHint>,
    pub conflict_hints: Vec<KnowledgeHint>,
    pub retrieval_hints: Vec<KnowledgeHint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeHint {
    pub label: String,
    pub summary: String,
    pub evidence: Vec<EvidenceAnchor>,
    pub timeline_node_id: String,
}
