use super::*;

fn request() -> ChatRunRequest {
    ChatRunRequest {
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
