use super::*;

fn table() -> ChatTable {
    ChatTable {
        id: "table:example".into(),
        name: "data.csv".into(),
        columns: vec!["Amount".into(), "Record ID".into()],
        rows: vec![vec!["001.20".into(), Uuid::new_v4().to_string()]],
    }
}

#[test]
fn tables_preserve_strings_and_require_rectangular_unique_headers() {
    let mut input = table();
    assert!(validate_chat_tables(std::slice::from_ref(&input)).is_ok());
    let roundtrip: ChatTable =
        serde_json::from_str(&serde_json::to_string(&input).unwrap()).unwrap();
    assert_eq!(roundtrip.rows[0][0], "001.20");
    input.rows[0].pop();
    assert_eq!(
        validate_chat_tables(&[input]),
        Err(ChatValidationError::InvalidTable)
    );
    let mut input = table();
    input.columns[1] = " amount ".into();
    assert_eq!(
        validate_chat_tables(&[input]),
        Err(ChatValidationError::InvalidTable)
    );
}

#[test]
fn combined_limits_and_duplicate_ids_are_rejected() {
    let first = table();
    assert_eq!(
        validate_chat_tables(&[first.clone(), first]),
        Err(ChatValidationError::InvalidTable)
    );
    let mut input = table();
    input.rows = vec![vec![String::new(), String::new()]; MAX_CHAT_TABLE_ROWS + 1];
    assert_eq!(
        validate_chat_tables(&[input]),
        Err(ChatValidationError::TableLimit)
    );
    let mut input = table();
    input.rows[0][0] = "x".repeat(MAX_CHAT_TABLE_BYTES);
    assert_eq!(
        validate_chat_tables(&[input]),
        Err(ChatValidationError::TableLimit)
    );
}

#[test]
fn artifacts_reject_paths_and_ambiguous_identifiers() {
    for filename in [
        "../secret.csv",
        "dir/export.csv",
        "dir\\export.csv",
        ".csv",
        "file.xlsx",
        "file\n.csv",
    ] {
        assert!(!valid_csv_filename(filename));
    }
    assert!(valid_csv_filename("Expanded data.csv"));
    let input = table();
    let mut artifact = ChatArtifact {
        id: Uuid::new_v4(),
        filename: "expanded.csv".into(),
        columns: input.columns,
        rows: input.rows,
    };
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_ok());
    artifact.id = Uuid::nil();
    assert_eq!(
        validate_chat_artifacts(&[artifact]),
        Err(ChatValidationError::InvalidTable)
    );
}
