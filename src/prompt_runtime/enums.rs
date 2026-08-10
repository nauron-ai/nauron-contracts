use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PromptRuntimeStage {
    Ingest,
    ValueRules,
    StructuredCandidates,
    CandidateReview,
    CandidatePrune,
    Retrieval,
    RetrievalLocalization,
    RetrievalRanking,
    IdentityCoverage,
    Repair,
    EvidenceRepair,
    Consistency,
    Reasoning,
    FinalResponse,
}

impl AsRef<str> for PromptRuntimeStage {
    fn as_ref(&self) -> &str {
        match self {
            Self::Ingest => "ingest",
            Self::ValueRules => "value_rules",
            Self::StructuredCandidates => "structured_candidates",
            Self::CandidateReview => "candidate_review",
            Self::CandidatePrune => "candidate_prune",
            Self::Retrieval => "retrieval",
            Self::RetrievalLocalization => "retrieval_localization",
            Self::RetrievalRanking => "retrieval_ranking",
            Self::IdentityCoverage => "identity_coverage",
            Self::Repair => "repair",
            Self::EvidenceRepair => "evidence_repair",
            Self::Consistency => "consistency",
            Self::Reasoning => "reasoning",
            Self::FinalResponse => "final_response",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PromptRuntimeRole {
    System,
    User,
}

impl AsRef<str> for PromptRuntimeRole {
    fn as_ref(&self) -> &str {
        match self {
            Self::System => "system",
            Self::User => "user",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum PromptActivationCondition {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "structured record path")]
    StructuredRecordPath,
    #[serde(rename = "repair required")]
    RepairRequired,
    #[serde(rename = "retrieval required")]
    RetrievalRequired,
    #[serde(rename = "post-extraction validation enabled")]
    PostExtractionValidationEnabled,
}

impl AsRef<str> for PromptActivationCondition {
    fn as_ref(&self) -> &str {
        match self {
            Self::Always => "always",
            Self::StructuredRecordPath => "structured record path",
            Self::RepairRequired => "repair required",
            Self::RetrievalRequired => "retrieval required",
            Self::PostExtractionValidationEnabled => "post-extraction validation enabled",
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::{PromptActivationCondition, PromptRuntimeRole, PromptRuntimeStage};

    #[test]
    fn runtime_enum_labels_are_stable() {
        let stages = [
            (PromptRuntimeStage::Ingest, "ingest"),
            (PromptRuntimeStage::ValueRules, "value_rules"),
            (
                PromptRuntimeStage::StructuredCandidates,
                "structured_candidates",
            ),
            (PromptRuntimeStage::CandidateReview, "candidate_review"),
            (PromptRuntimeStage::CandidatePrune, "candidate_prune"),
            (PromptRuntimeStage::Retrieval, "retrieval"),
            (
                PromptRuntimeStage::RetrievalLocalization,
                "retrieval_localization",
            ),
            (PromptRuntimeStage::RetrievalRanking, "retrieval_ranking"),
            (PromptRuntimeStage::IdentityCoverage, "identity_coverage"),
            (PromptRuntimeStage::Repair, "repair"),
            (PromptRuntimeStage::EvidenceRepair, "evidence_repair"),
            (PromptRuntimeStage::Consistency, "consistency"),
            (PromptRuntimeStage::Reasoning, "reasoning"),
            (PromptRuntimeStage::FinalResponse, "final_response"),
        ];
        for (stage, label) in stages {
            assert_serialized_label(stage, label);
        }
        assert_serialized_label(PromptRuntimeRole::System, "system");
        assert_serialized_label(PromptRuntimeRole::User, "user");
        assert_serialized_label(PromptActivationCondition::Always, "always");
        assert_serialized_label(
            PromptActivationCondition::StructuredRecordPath,
            "structured record path",
        );
        assert_serialized_label(PromptActivationCondition::RepairRequired, "repair required");
        assert_serialized_label(
            PromptActivationCondition::RetrievalRequired,
            "retrieval required",
        );
        assert_serialized_label(
            PromptActivationCondition::PostExtractionValidationEnabled,
            "post-extraction validation enabled",
        );
    }

    fn assert_serialized_label(value: impl Serialize, expected: &str) {
        assert_eq!(
            serde_json::to_value(value).expect("serializable runtime enum"),
            expected
        );
    }
}
