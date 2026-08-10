use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum AutotuningOperationErrorCode {
    #[serde(rename = "autotuning_operation_in_progress")]
    InProgress,
    #[serde(rename = "autotuning_operation_indeterminate")]
    Indeterminate,
    #[serde(rename = "autotuning_idempotency_binding_mismatch")]
    BindingMismatch,
    #[serde(rename = "autotuning_operation_lease_changed")]
    LeaseChanged,
    #[serde(rename = "autotuning_candidate_generation_exhausted")]
    CandidateGenerationExhausted,
    #[serde(rename = "autotuning_candidate_generation_failed")]
    CandidateGenerationFailed,
    #[serde(rename = "autotuning_operation_timeout")]
    OperationTimeout,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AutotuningOperationErrorResponse {
    pub error: String,
    pub code: AutotuningOperationErrorCode,
}
