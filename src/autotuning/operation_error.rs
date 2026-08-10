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
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AutotuningOperationErrorResponse {
    pub error: String,
    pub code: AutotuningOperationErrorCode,
}
