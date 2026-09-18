use serde_json::{Value, json};

use super::{ChatRunRequest, ChatRunResponse};

const RUN_ID: &str = "0cbd326c-61ea-4d65-9212-6a4e4b4df740";
const USER_ID: &str = "b5f358fc-a97c-4e28-a340-9189267aa048";

fn legacy_request() -> Value {
    json!({
        "run_id": RUN_ID,
        "user_id": USER_ID,
        "model": "gpt",
        "scope": { "type": "global" },
        "contracts": [{
            "contract_id": "b221ee59-5cdb-47c9-a24f-c9865e1b8f30",
            "context_id": 42,
            "name": "Parking agreement",
            "country_id": null,
            "metadata": {}
        }],
        "messages": [{ "role": "user", "content": "Summarize this contract" }]
    })
}

fn legacy_response() -> Value {
    json!({
        "run_id": RUN_ID,
        "model": "gpt",
        "actual_model": "configured-model",
        "answer": "The contract covers parking services.",
        "sources": [],
        "prompt_tokens": 100,
        "completion_tokens": 20,
        "tool_calls": 1
    })
}

#[test]
fn empty_extensions_preserve_legacy_chat_payloads() {
    let request: ChatRunRequest = serde_json::from_value(legacy_request()).unwrap();
    assert!(request.validate().is_ok());
    assert_eq!(serde_json::to_value(request).unwrap(), legacy_request());

    let response: ChatRunResponse = serde_json::from_value(legacy_response()).unwrap();
    assert_eq!(serde_json::to_value(response).unwrap(), legacy_response());
}

#[test]
fn populated_extensions_roundtrip_without_data_loss() {
    let mut request = legacy_request();
    request["tables"] = json!([{
        "id": "table:1", "name": "Amounts", "columns": ["Amount"], "rows": [["001.20"]]
    }]);
    request["datasets"] = json!([{
        "id": "dataset:1", "name": "Payments", "columns": ["Amount"], "row_count": 100
    }]);
    request["data_source"] = json!({
        "query_url": "https://worker.example/chat/runs/data", "lease_owner": USER_ID
    });
    let parsed: ChatRunRequest = serde_json::from_value(request.clone()).unwrap();
    assert!(parsed.validate().is_ok());
    assert_eq!(serde_json::to_value(parsed).unwrap(), request);

    let mut response = legacy_response();
    response["artifacts"] = json!([{
        "id": RUN_ID, "filename": "amounts.csv", "columns": ["Amount"], "rows": [["001.20"]]
    }]);
    let parsed: ChatRunResponse = serde_json::from_value(response.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), response);
}

#[test]
fn compatibility_does_not_allow_unknown_fields() {
    let mut request = legacy_request();
    request["unsupported"] = json!([]);
    assert!(serde_json::from_value::<ChatRunRequest>(request).is_err());

    let mut response = legacy_response();
    response["unsupported"] = json!([]);
    assert!(serde_json::from_value::<ChatRunResponse>(response).is_err());
}
