use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::knowledge_artifacts::TranslatedEvidenceAnchor;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AtlasContractStory {
    pub documents: Vec<AtlasDocumentFacts>,
    pub parties: Vec<AtlasParty>,
    pub relationships: Vec<AtlasDocumentRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AtlasDocumentFacts {
    pub doc_id: Uuid,
    pub lifecycle: AtlasDocumentLifecycle,
    pub events: Vec<AtlasDocumentEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AtlasDocumentLifecycle {
    Draft,
    Issued,
    Executed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AtlasDocumentEvent {
    pub kind: AtlasEventKind,
    pub date: NaiveDate,
    pub certainty: AtlasCertainty,
    pub evidence: Vec<TranslatedEvidenceAnchor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AtlasEventKind {
    DocumentDate,
    Signature,
    Commencement,
    Expiry,
    Formation,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AtlasCertainty {
    Confirmed,
    Inferred,
    Uncertain,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AtlasParty {
    pub id: String,
    pub canonical_name: String,
    pub appearances: Vec<AtlasPartyAppearance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AtlasPartyAppearance {
    pub doc_id: Uuid,
    pub party_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AtlasDocumentRelationship {
    pub source_doc_id: Uuid,
    pub target_doc_id: Uuid,
    pub kind: AtlasDocumentRelationshipKind,
    pub certainty: AtlasCertainty,
    pub summary: String,
    pub evidence: Vec<TranslatedEvidenceAnchor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AtlasDocumentRelationshipKind {
    Amends,
    Supersedes,
    DerivedFrom,
    Implements,
    Supports,
}
