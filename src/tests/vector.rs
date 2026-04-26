use serde_json::json;

use crate::vector::{RdfFactsResponse, RdfRelation};

use super::common::parse_uuid;

#[test]
fn rdf_facts_response_backwards_compatible_defaults() {
    let value = json!({
        "entities": [],
        "relations": [],
        "paragraphs": [],
    });

    let decoded: RdfFactsResponse = serde_json::from_value(value).expect("decode rdf facts");
    assert!(decoded.claims.is_empty());
    assert!(decoded.contradictions.is_empty());
    assert!(decoded.similarity_edges.is_empty());
}

#[test]
fn rdf_relation_accepts_evidence_pointer_lang() {
    let doc_id = parse_uuid("55555555-aaaa-bbbb-cccc-555555555555");
    let value = json!({
        "subject": "s",
        "predicate": "p",
        "object": "o",
        "justification": null,
        "confidence": 0.9,
        "evidence_paragraphs": [
            { "doc_id": doc_id, "paragraph_id": "p1", "text": "t", "lang": "en" }
        ]
    });

    let decoded: RdfRelation = serde_json::from_value(value).expect("decode rdf relation");
    assert_eq!(decoded.evidence_paragraphs.len(), 1);
    assert_eq!(decoded.evidence_paragraphs[0].lang.as_deref(), Some("en"));
}
