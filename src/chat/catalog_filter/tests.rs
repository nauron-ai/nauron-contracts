use super::*;
use serde_json::json;

fn predicate(field: &str, operator: &str, value: serde_json::Value) -> ChatCatalogFilter {
    serde_json::from_value(json!({"field":field,"operator":operator,"value":value})).unwrap()
}

#[test]
fn alternatives_match_text_list_members_and_combine_with_required_filters() {
    let all = vec![predicate("country", "equal", json!("DK"))];
    let any = vec![
        predicate("type", "equal", json!("saas")),
        predicate("services", "contains", json!("saas")),
    ];
    let values = |field: &str| match field {
        "country" => ChatValue::Text("DK".into()),
        "type" => ChatValue::Text("other".into()),
        "services" => ChatValue::List(vec![ChatValue::Text("SaaS_solution".into())]),
        _ => ChatValue::Null,
    };
    assert!(matches_catalog_filters(&all, &any, values));
    let excluded = vec![predicate("country", "equal", json!("DE"))];
    assert!(!matches_catalog_filters(&excluded, &any, values));
    assert!(matches_catalog_filters(&all, &[], values));
    assert!(!matches_catalog_filters(
        &[],
        &[predicate("missing", "contains", json!("saas"))],
        values
    ));
}

#[test]
fn invalid_alternative_fields_operands_and_combined_budget_are_rejected() {
    let fields = BTreeSet::from(["type".to_owned()]);
    assert!(
        validate_catalog_filters(
            &[],
            &[predicate("unknown", "equal", json!("saas"))],
            &fields
        )
        .is_err()
    );
    assert!(
        validate_catalog_filters(&[], &[predicate("type", "contains", json!(false))], &fields)
            .is_err()
    );
    let predicates = vec![predicate("type", "equal", json!("saas")); MAX_DATA_FILTERS];
    assert!(
        validate_catalog_filters(
            &predicates,
            &[predicate("type", "equal", json!("other"))],
            &fields
        )
        .is_err()
    );
}

#[test]
fn missing_alternative_values_remain_visible_even_when_another_predicate_matches() {
    let any = vec![
        predicate("type", "equal", json!("saas")),
        predicate("services", "contains", json!("digital")),
    ];
    let value = |field: &str| {
        if field == "type" {
            ChatValue::Text("saas".into())
        } else {
            ChatValue::Null
        }
    };
    assert!(matches_catalog_filters(&[], &any, value));
    assert!(missing_catalog_filter_values(&[], &any, value));
}

#[test]
fn null_filter_is_unary_and_comparisons_require_explicit_operands() {
    for input in [
        json!({"field":"type","operator":"is_null"}),
        json!({"field":"type","operator":"is_null","value":null}),
    ] {
        let filter: ChatCatalogFilter = serde_json::from_value(input).unwrap();
        assert!(filter.matches(&ChatValue::Null));
        assert!(!filter.matches(&ChatValue::Text(String::new())));
    }
    assert!(
        serde_json::from_value::<ChatCatalogFilter>(json!({"field":"type","operator":"equal"}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<ChatCatalogFilter>(
            json!({"field":"type","operator":"is_null","value":0})
        )
        .is_err()
    );
}
