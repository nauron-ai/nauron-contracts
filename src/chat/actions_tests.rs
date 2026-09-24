use serde_json::json;

use super::*;

fn definition() -> ChatActionDefinition {
    ChatActionDefinition {
        name: "calculate_report".into(),
        description: "Calculate a report from authorized data".into(),
        parameters: json!({"type":"object","properties":{}}),
        grounding_instruction: None,
    }
}

#[test]
fn action_definitions_require_unique_names_and_bounded_schemas() {
    let mut action = definition();
    assert!(validate_chat_actions(&[action.clone()]).is_ok());
    assert!(validate_chat_actions(&[action.clone(), action.clone()]).is_err());
    action.parameters = json!({"value":"a".repeat(MAX_ACTION_TEXT_BYTES)});
    assert!(validate_chat_actions(&[action]).is_err());
}

#[test]
fn callback_envelope_preserves_legacy_queries_and_rejects_ambiguous_requests() {
    let mut value = json!({"table_id":"finance:one","filters":[],"offset":0,"limit":10});
    let request: ChatDataRequest = serde_json::from_value(value.clone()).unwrap();
    assert!(matches!(request, ChatDataRequest::Query(_)));
    assert!(request.validate().is_ok());
    value["action"] = json!("calculate_report");
    value["arguments"] = json!({});
    assert!(serde_json::from_value::<ChatDataRequest>(value).is_err());
    let request: ChatDataRequest = serde_json::from_value(json!({
        "action":"calculate_report","arguments":{}
    }))
    .unwrap();
    assert!(matches!(request, ChatDataRequest::Action(_)));
    assert!(request.validate().is_ok());
}

#[test]
fn action_evidence_requires_contract_identity_and_nonempty_excerpt() {
    let mut call = ChatActionCall {
        action: "calculate_report".into(),
        arguments: json!({}),
        evidence: Some(ChatSource {
            id: 1,
            contract_id: Some(Uuid::new_v4()),
            document_id: None,
            paragraph_id: None,
            excerpt: "Supporting contractual clause".into(),
        }),
    };
    assert!(call.validate().is_ok());
    call.evidence.as_mut().unwrap().contract_id = None;
    assert!(call.validate().is_err());
    call.evidence = None;
    call.arguments = json!([]);
    assert!(call.validate().is_err());
}

#[test]
fn action_business_errors_are_distinct_from_results_and_legacy_pages() {
    let response: ChatDataResponse = serde_json::from_value(json!({
        "status":"error","message":"No compatible reporting periods"
    }))
    .unwrap();
    assert!(matches!(
        response,
        ChatDataResponse::Action(ChatActionResult::Error { .. })
    ));
    assert!(
        serde_json::from_value::<ChatDataResponse>(json!({
            "status":"success","data":"{}","artifact":null,"contract_id":null,
            "message":"ambiguous error"
        }))
        .is_err()
    );
}
