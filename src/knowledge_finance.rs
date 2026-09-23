use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::knowledge_artifacts::{ContractDocumentStory, TranslatedEvidenceAnchor};

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ContractStory {
    pub language: String,
    pub documents: Vec<ContractDocumentStory>,
    #[serde(default)]
    pub atlas_source: bool,
    #[serde(default)]
    pub financial_flows: Vec<ContractFinancialFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atlas: Option<crate::knowledge_atlas::AtlasContractStory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ContractFinancialFlow {
    pub doc_id: Uuid,
    pub kind: FinancialFlowKind,
    pub payer: Option<String>,
    pub payee: Option<String>,
    pub purpose: String,
    pub amount_or_rule: String,
    pub period: Option<String>,
    pub status: FinancialFlowStatus,
    pub certainty: FinancialFlowCertainty,
    pub evidence: Vec<TranslatedEvidenceAnchor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FinancialFlowKind {
    Payment,
    OperatingReceipt,
    ContractPrice,
    Investment,
    PaymentChange,
    OtherFinancialTerm,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FinancialFlowStatus {
    Operative,
    Proposed,
    Conditional,
    Historical,
    Referenced,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FinancialFlowCertainty {
    Explicit,
    Inferred,
    Unknown,
}
