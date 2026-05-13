use serde_json::json;

use crate::ingest::{IngestEvent, IngestResult, IngestSchemaField, IngestStage, IngestStart};

#[test]
fn ingest_schema_field_accepts_legacy_scalar_type() {
    let field: IngestSchemaField = serde_json::from_value(json!({
        "key": "summary",
        "name": "Summary",
        "description": "summary",
        "type": "string",
        "required": true
    }))
    .unwrap();

    assert_eq!(field.r#type, json!("string"));
    assert!(field.required);
    assert_eq!(field.name.as_deref(), Some("Summary"));
}

#[test]
fn ingest_schema_field_accepts_object_type_spec() {
    let field: IngestSchemaField = serde_json::from_value(json!({
        "key": "operational_start",
        "description": "operational start",
        "type": {
            "type": "object",
            "properties": {
                "original": { "type": "string" },
                "translated": { "type": "string" }
            },
            "required": ["original", "translated"]
        }
    }))
    .unwrap();

    assert!(field.r#type.is_object());
}

#[test]
fn ingest_schema_field_defaults_type_to_string() {
    let field: IngestSchemaField = serde_json::from_value(json!({
        "key": "summary",
        "description": "summary"
    }))
    .unwrap();

    assert_eq!(field.r#type, json!("string"));
}

#[test]
fn ingest_schema_field_rejects_invalid_type_spec_shape() {
    let error = serde_json::from_value::<IngestSchemaField>(json!({
        "key": "summary",
        "description": "summary",
        "type": 42
    }))
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("ingest schema field type must be a string or object")
    );
}

#[test]
fn ingest_start_accepts_optional_knowledge_revision() {
    let start: IngestStart = serde_json::from_value(json!({
        "job_id": "00000000-0000-0000-0000-000000000001",
        "context_id": 42,
        "schema": [{
            "key": "summary",
            "description": "summary"
        }],
        "knowledge_revision": 7
    }))
    .unwrap();

    assert_eq!(start.knowledge_revision, Some(7));
}

#[test]
fn ingest_start_omits_empty_knowledge_revision() {
    let start: IngestStart = serde_json::from_value(json!({
        "job_id": "00000000-0000-0000-0000-000000000001",
        "context_id": 42,
        "schema": [{
            "key": "summary",
            "description": "summary"
        }]
    }))
    .unwrap();
    let value = serde_json::to_value(start).unwrap();

    assert!(value.get("knowledge_revision").is_none());
}

#[test]
fn ingest_stage_labels_roundtrip() {
    assert_eq!(IngestStage::Queued.to_string(), "queued");
    assert!(matches!(
        "persist".parse::<IngestStage>().unwrap(),
        IngestStage::Persist
    ));
    assert!("unknown".parse::<IngestStage>().is_err());
}

#[test]
fn ingest_result_success_carries_evidence_and_knowledge() {
    let event: IngestEvent = serde_json::from_str(include_str!(
        "fixtures/ingest_result_success_with_knowledge.json"
    ))
    .unwrap();

    let result = match event {
        IngestEvent::Result(result) => Some(result),
        IngestEvent::Progress(_) => None,
    }
    .unwrap();
    let (context_id, evidence, knowledge) = match result.as_ref() {
        IngestResult::Success {
            context_id,
            evidence,
            knowledge,
            ..
        } => Some((context_id, evidence, knowledge)),
        IngestResult::Failure { .. } => None,
    }
    .unwrap();

    assert_eq!(*context_id, 42);
    assert_eq!(evidence[0].path, "rent_amount");
    assert_eq!(evidence[0].anchors[0].paragraph_id, "p1");
    let knowledge = knowledge.as_ref().unwrap();
    assert_eq!(knowledge.dossier.name, "Agreement");
    assert_eq!(knowledge.dossier.revision, 1);
    assert!(!knowledge.dossier.metadata.require_conflicts_with);
    assert_eq!(knowledge.compiled_knowledge_view.dossier_name, "Agreement");
    assert_eq!(knowledge.compiled_knowledge_view.active_surfaces.len(), 1);
    assert_eq!(
        knowledge.compiled_knowledge_view.active_surfaces[0].timeline_node_id,
        "node-1"
    );
    assert_eq!(knowledge.timeline_view.nodes.len(), 1);
    assert_eq!(knowledge.timeline_view.nodes[0].id, "node-1");
    assert_eq!(knowledge.timeline_view.nodes[0].evidence.len(), 1);
}
