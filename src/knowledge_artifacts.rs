use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeArtifact {
    pub dossier: DossierArtifact,
    pub compiled_knowledge_view: CompiledKnowledgeView,
    pub timeline_view: TimelineView,
    #[serde(default)]
    pub analysis_view: AnalysisView,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AnalysisView {
    pub findings: Vec<AnalysisFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AnalysisFinding {
    pub profile: AnalysisProfile,
    pub title: String,
    pub summary: String,
    pub score: u8,
    pub parties: Vec<String>,
    pub signals: Vec<String>,
    pub assumptions: Vec<String>,
    pub next_step: String,
    pub evidence: Vec<EvidenceAnchor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisProfile {
    Opportunity,
    SecurityPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DossierArtifact {
    pub id: Uuid,
    pub context_id: i64,
    pub name: String,
    pub role: DossierRole,
    pub scope: DossierScope,
    pub revision: i32,
    pub metadata: DossierMetadata,
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
    #[serde(default)]
    pub analysis_profiles: Vec<AnalysisProfile>,
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
    UnresolvedChange,
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
    #[serde(default)]
    pub contract_story: ContractStory,
    pub active_surfaces: Vec<KnowledgeHint>,
    pub temporal_hints: Vec<KnowledgeHint>,
    pub conflict_hints: Vec<KnowledgeHint>,
    pub retrieval_hints: Vec<KnowledgeHint>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ContractStory {
    pub language: String,
    pub documents: Vec<ContractDocumentStory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ContractDocumentStory {
    pub doc_id: Uuid,
    pub parties: Vec<ContractParty>,
    pub relationships: Vec<ContractRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ContractParty {
    pub id: String,
    pub canonical_name: String,
    pub aliases: Vec<String>,
    pub roles: Vec<String>,
    pub evidence: Vec<TranslatedEvidenceAnchor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ContractRelationship {
    pub subject_party_id: String,
    pub predicate: String,
    pub object_party_id: Option<String>,
    pub object: String,
    pub modality: ContractRelationshipModality,
    pub conditions: Vec<String>,
    pub evidence: Vec<TranslatedEvidenceAnchor>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContractRelationshipModality {
    Definition,
    Grant,
    Permission,
    Obligation,
    Prohibition,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TranslatedEvidenceAnchor {
    pub doc_id: Uuid,
    pub paragraph_id: String,
    pub source_quote: String,
    pub source_language: Option<String>,
    pub english_translation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeHint {
    pub label: String,
    pub summary: String,
    pub evidence: Vec<EvidenceAnchor>,
    pub timeline_node_id: String,
}

#[cfg(test)]
mod tests {
    use super::CompiledKnowledgeView;

    #[test]
    fn legacy_compiled_view_defaults_contract_story() {
        let value = serde_json::json!({
            "dossier_name": "Agreement",
            "brief": "Current state",
            "active_surfaces": [],
            "temporal_hints": [],
            "conflict_hints": [],
            "retrieval_hints": []
        });

        let parsed: CompiledKnowledgeView = serde_json::from_value(value).unwrap();

        assert!(parsed.contract_story.documents.is_empty());
    }
}
