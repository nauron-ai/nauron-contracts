use crate::{IngestEvidenceSupportStatus, IngestFieldEvidence};
use serde_json::json;

#[test]
fn deserializes_historical_evidence_as_unassessed() {
    let evidence: IngestFieldEvidence = serde_json::from_value(json!({
        "path": "rent.amount",
        "reasoning": "Historical reasoning",
        "anchors": []
    }))
    .unwrap();

    assert_eq!(
        evidence.support_status,
        IngestEvidenceSupportStatus::Unassessed
    );
    assert!(evidence.evidence_gap.is_none());
}

#[test]
fn serializes_partial_support_with_evidence_gap() {
    let evidence: IngestFieldEvidence = serde_json::from_value(json!({
        "path": "rent.amount",
        "support_status": "partially_supported",
        "reasoning": "The amount appears once.",
        "evidence_gap": "The applicable period is not stated.",
        "anchors": []
    }))
    .unwrap();

    assert_eq!(
        evidence.support_status,
        IngestEvidenceSupportStatus::PartiallySupported
    );
    assert_eq!(
        evidence.evidence_gap.as_deref(),
        Some("The applicable period is not stated.")
    );
}
