use serde_json::json;

use crate::conditions::ConditionEvaluationResponse;

#[test]
fn condition_result_carries_nullable_token_usage() {
    let response: ConditionEvaluationResponse = serde_json::from_value(json!({
        "context_id": 42,
        "results": [{
            "condition_id": "termination",
            "verdict": "satisfied",
            "satisfied": true,
            "confidence": 0.9,
            "evidence_strength": 0.8,
            "matches": [],
            "reasoning": "supported",
            "tokens_used": {
                "prompt": 120,
                "completion": null
            }
        }]
    }))
    .unwrap();

    let usage = response.results[0].tokens_used.as_ref().unwrap();
    assert_eq!(usage.prompt, Some(120));
    assert_eq!(usage.completion, None);
}

#[test]
fn condition_result_accepts_legacy_payload_without_token_usage() {
    let response: ConditionEvaluationResponse = serde_json::from_value(json!({
        "context_id": 42,
        "results": [{
            "condition_id": "termination",
            "verdict": "unknown",
            "satisfied": false,
            "confidence": 0.2,
            "evidence_strength": 0.1,
            "matches": [],
            "reasoning": "missing evidence"
        }]
    }))
    .unwrap();

    assert!(response.results[0].tokens_used.is_none());
    let serialized = serde_json::to_value(response).unwrap();
    assert!(serialized["results"][0].get("tokens_used").is_none());
}
