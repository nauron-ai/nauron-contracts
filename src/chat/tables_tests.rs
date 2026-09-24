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
        sheets: Vec::new(),
    };
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_ok());
    artifact.id = Uuid::nil();
    assert_eq!(
        validate_chat_artifacts(&[artifact]),
        Err(ChatValidationError::InvalidTable)
    );
}

#[test]
fn workbooks_validate_every_sheet_without_silently_discarding_csv_sheets() {
    let mut artifact = ChatArtifact {
        id: Uuid::new_v4(),
        filename: "finance.xlsx".into(),
        columns: vec!["Country".into(), "GOPBD".into()],
        rows: vec![vec!["DE".into(), "100.25".into()]],
        sheets: vec![ChatWorksheet {
            name: "Sources".into(),
            columns: vec!["Source".into()],
            rows: vec![vec!["finance:version-id".into()]],
        }],
    };
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_ok());
    artifact.filename = "finance.csv".into();
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_err());
    artifact.filename = "finance.xlsx".into();
    for name in [
        "report", "History", "bad/name", "'Source", "", "Source'", "[Source]",
    ] {
        artifact.sheets[0].name = name.into();
        assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_err());
    }
    artifact.sheets[0].name = "Sources".into();
    artifact.sheets[0].rows[0].push("extra cell".into());
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_err());
    artifact.sheets[0].rows = vec![vec!["x".repeat(MAX_XLSX_CELL_CHARACTERS + 1)]];
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_err());
}

#[test]
fn existing_csv_artifacts_remain_readable_without_new_fields() {
    let artifact: ChatArtifact = serde_json::from_value(serde_json::json!({
        "id": Uuid::new_v4(), "filename": "old.csv", "columns": ["Revenue"], "rows": [["10"]]
    }))
    .unwrap();
    assert!(artifact.sheets.is_empty());
    assert!(validate_chat_artifacts(&[artifact]).is_ok());
}

#[test]
fn xlsx_cells_enforce_the_utf16_limit_for_supplementary_characters() {
    let mut artifact = ChatArtifact {
        id: Uuid::new_v4(),
        filename: "unicode.xlsx".into(),
        columns: vec!["Value".into()],
        rows: vec![vec!["🅰".repeat(MAX_XLSX_CELL_CHARACTERS / 2)]],
        sheets: Vec::new(),
    };
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_ok());
    artifact.rows[0][0].push('🅰');
    assert_eq!(
        validate_chat_artifacts(&[artifact]),
        Err(ChatValidationError::TableLimit)
    );
}

#[test]
fn workbook_limit_includes_the_primary_report_sheet() {
    let mut artifact = ChatArtifact {
        id: Uuid::new_v4(),
        filename: "many-sheets.xlsx".into(),
        columns: vec!["Value".into()],
        rows: Vec::new(),
        sheets: (1..MAX_XLSX_WORKSHEETS)
            .map(|number| ChatWorksheet {
                name: format!("Sheet {number}"),
                columns: vec!["Value".into()],
                rows: Vec::new(),
            })
            .collect(),
    };
    assert!(validate_chat_artifacts(std::slice::from_ref(&artifact)).is_ok());
    artifact.sheets.push(ChatWorksheet {
        name: "One too many".into(),
        columns: vec!["Value".into()],
        rows: Vec::new(),
    });
    assert_eq!(
        validate_chat_artifacts(&[artifact]),
        Err(ChatValidationError::TableLimit)
    );
}
