use super::*;
use uuid::Uuid;

fn state() -> TaskState {
    TaskState {
        goal: "Compare the selected records".into(),
        completion_criteria: vec!["Return supported differences".into()],
        instructions: Vec::new(),
        findings: Vec::new(),
        hypotheses: Vec::new(),
        unresolved: Vec::new(),
        completed_steps: Vec::new(),
        next_step: "Read the selected source".into(),
    }
}

#[test]
fn unsupported_findings_cannot_enter_the_working_state() {
    let mut state = state();
    state.findings.push(TaskFinding {
        statement: "An agreement contains a term".into(),
        evidence: Vec::new(),
    });
    assert!(state.validate().is_err());
    state.findings[0].evidence.push(TaskEvidenceRef {
        run_id: Uuid::new_v4(),
        source_id: 1,
    });
    assert!(state.validate().is_ok());
}

#[test]
fn task_state_must_remain_bounded() {
    let mut state = state();
    state.completed_steps.push("x".repeat(MAX_TASK_STATE_BYTES));
    assert!(state.validate().is_err());
}

#[test]
fn user_rules_require_an_original_message_and_a_nonempty_quote() {
    let mut state = state();
    state.instructions.push(TaskInstruction {
        message_id: Uuid::nil(),
        quote: "Keep previous values".into(),
    });
    assert!(state.validate().is_err());
    state.instructions[0].message_id = Uuid::new_v4();
    state.instructions[0].quote = " ".into();
    assert!(state.validate().is_err());
}

#[test]
fn original_input_access_requires_bounded_pages_and_valid_identity() {
    assert!(
        TaskInputQuery::List {
            offset: 0,
            limit: MAX_TASK_INPUT_HEADERS
        }
        .validate()
        .is_ok()
    );
    assert!(
        TaskInputQuery::List {
            offset: 0,
            limit: MAX_TASK_INPUT_HEADERS + 1
        }
        .validate()
        .is_err()
    );
    assert!(
        TaskInputQuery::Read {
            message_id: Uuid::nil(),
            offset: 0,
            limit: 1
        }
        .validate()
        .is_err()
    );
    assert!(
        TaskInputQuery::Read {
            message_id: Uuid::new_v4(),
            offset: 0,
            limit: MAX_TASK_INPUT_PAGE + 1
        }
        .validate()
        .is_err()
    );
    let request = ChatTaskRequest::Input {
        task_id: Uuid::new_v4(),
        query: TaskInputQuery::Read {
            message_id: Uuid::new_v4(),
            offset: 0,
            limit: MAX_TASK_INPUT_PAGE,
        },
    };
    let encoded = serde_json::to_string(&request).unwrap();
    let decoded: ChatTaskRequest = serde_json::from_str(&encoded).unwrap();
    assert!(decoded.validate().is_ok());
}
