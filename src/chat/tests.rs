use super::*;

fn request() -> ChatRunRequest {
    ChatRunRequest {
        datasets: Vec::new(),
        data_source: None,
        action_tools: Vec::new(),
        run_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        model: ChatModel::Gpt,
        scope: ChatScope::Global,
        contracts: vec![ChatContract {
            contract_id: Uuid::new_v4(),
            context_id: Some(42),
            name: "Lease".into(),
            country_id: Some(Uuid::new_v4()),
            metadata: BTreeMap::new(),
        }],
        tables: Vec::new(),
        messages: vec![ChatMessage {
            role: ChatRole::User,
            content: "What expires next year?".into(),
        }],
    }
}

#[test]
fn global_scope_must_not_become_unrestricted_when_empty() {
    let mut input = request();
    assert!(input.validate().is_ok());
    input.contracts.clear();
    assert_eq!(input.validate(), Err(ChatValidationError::EmptyScope));
}

#[test]
fn contract_scope_cannot_include_another_contract() {
    let mut input = request();
    input.scope = ChatScope::Contract {
        contract_id: Uuid::new_v4(),
    };
    assert_eq!(
        input.validate(),
        Err(ChatValidationError::ContractScopeMismatch)
    );
}

#[test]
fn scope_rejects_ambiguous_context_mapping() {
    let mut input = request();
    let mut duplicate = input.contracts[0].clone();
    duplicate.contract_id = Uuid::new_v4();
    input.contracts.push(duplicate);
    assert_eq!(input.validate(), Err(ChatValidationError::InvalidScope));
}

#[test]
fn followup_requires_completed_conversation_order() {
    let mut input = request();
    input.messages.push(ChatMessage {
        role: ChatRole::Assistant,
        content: "Two contracts.".into(),
    });
    assert_eq!(input.validate(), Err(ChatValidationError::InvalidHistory));
    input.messages.push(ChatMessage {
        role: ChatRole::User,
        content: "Which ones?".into(),
    });
    assert!(input.validate().is_ok());
}

#[cfg(feature = "sqlx")]
#[test]
fn models_use_standard_sql_text() {
    use sqlx::{Postgres, Type};

    assert_eq!(
        <ChatModel as Type<Postgres>>::type_info(),
        <str as Type<Postgres>>::type_info()
    );
}

#[test]
fn action_descriptors_work_before_and_after_the_worker_attaches_its_callback() {
    let mut input = request();
    input.action_tools.push(ChatActionDefinition {
        name: "calculate_report".into(),
        description: "Calculate a report from authorized data".into(),
        parameters: serde_json::json!({"type":"object","properties":{}}),
        grounding_instruction: None,
    });
    assert!(input.validate().is_ok());
    input.data_source = Some(ChatDataSource {
        query_url: "https://worker.example.test/data".into(),
        lease_owner: Uuid::new_v4(),
    });
    assert!(input.validate().is_ok());
    input.action_tools.clear();
    assert_eq!(input.validate(), Err(ChatValidationError::InvalidTable));
}
