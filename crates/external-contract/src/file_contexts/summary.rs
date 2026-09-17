use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum DocumentKind {
    Agreement,
    Amendment,
    Invoice,
    Correspondence,
    Report,
    Drawing,
    Photo,
    Other,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum ContractMentionRole {
    Governs,
    Amends,
    References,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum CountryRole {
    SubjectLocation,
    PartyAddress,
    GoverningLaw,
    Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSummary {
    pub analysis_id: Uuid,
    pub revision: u64,
    pub language: String,
    pub title: String,
    pub brief: String,
    pub document_kind: DocumentKind,
    pub tags: Vec<String>,
    pub contract_mentions: Vec<ContractMention>,
    pub parties: Vec<FileParty>,
    pub countries: Vec<FileCountry>,
    pub evidence: Vec<FileEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMention {
    pub identifier: String,
    pub role: ContractMentionRole,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileParty {
    pub name: String,
    pub registration_id: Option<String>,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCountry {
    pub code: String,
    pub role: CountryRole,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEvidence {
    pub id: String,
    pub doc_id: String,
    pub paragraph_id: String,
    pub quote: String,
}
