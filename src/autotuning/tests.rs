use super::*;

#[test]
fn target_policy_has_one_allowed_component() {
    let policy = PromptTuningPolicy::target();

    assert!(policy.is_target());
    assert_eq!(policy.tuning_scope, PromptTuningScope::Target);
    assert_eq!(policy.allowed_component_ids, [TARGET_PROMPT_COMPONENT_ID]);
}

#[test]
fn operation_error_codes_have_stable_wire_values() {
    for (code, expected) in [
        (
            AutotuningOperationErrorCode::InProgress,
            "\"autotuning_operation_in_progress\"",
        ),
        (
            AutotuningOperationErrorCode::Indeterminate,
            "\"autotuning_operation_indeterminate\"",
        ),
        (
            AutotuningOperationErrorCode::BindingMismatch,
            "\"autotuning_idempotency_binding_mismatch\"",
        ),
        (
            AutotuningOperationErrorCode::LeaseChanged,
            "\"autotuning_operation_lease_changed\"",
        ),
        (
            AutotuningOperationErrorCode::CandidateGenerationExhausted,
            "\"autotuning_candidate_generation_exhausted\"",
        ),
        (
            AutotuningOperationErrorCode::CandidateGenerationFailed,
            "\"autotuning_candidate_generation_failed\"",
        ),
        (
            AutotuningOperationErrorCode::OperationTimeout,
            "\"autotuning_operation_timeout\"",
        ),
    ] {
        assert_eq!(
            serde_json::to_string(&code).expect("serialize code"),
            expected
        );
    }
}

#[test]
fn operation_error_response_rejects_unknown_fields() {
    let payload = serde_json::json!({
        "error": "operation is in progress",
        "code": "autotuning_operation_in_progress",
        "unexpected": true,
    });

    assert!(serde_json::from_value::<AutotuningOperationErrorResponse>(payload).is_err());
}
