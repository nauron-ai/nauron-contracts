use super::*;

#[test]
fn target_policy_has_one_allowed_component() {
    let policy = PromptTuningPolicy::target();

    assert!(policy.is_target());
    assert_eq!(policy.tuning_scope, PromptTuningScope::Target);
    assert_eq!(policy.allowed_component_ids, [TARGET_PROMPT_COMPONENT_ID]);
}
