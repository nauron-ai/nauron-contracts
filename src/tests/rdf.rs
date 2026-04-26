use chrono::Utc;
use serde_json::json;

use crate::{
    FailureKind, PipelineTimings, RdfEvent, RdfProgress, RdfResult, RdfStage, RdfStart,
    SchemaVersion,
};

use super::common::parse_uuid;

#[test]
fn rdf_start_defaults() {
    let job_id = parse_uuid("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa");
    let doc_id = parse_uuid("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb");
    let value = json!({
        "job_id": job_id,
        "doc_id": doc_id,
        "text_uri": "https://example.com/doc.md",
        "source_id": "src-1"
    });

    let request: RdfStart = serde_json::from_value(value).expect("valid rdf start");
    assert_eq!(request.context_id, 0);
    assert_eq!(request.schema_version, SchemaVersion::V1);
    assert_eq!(request.source_id, "src-1");
    assert!(request.requested_at.is_none());
}

#[test]
fn rdf_progress_roundtrip() {
    let job_id = parse_uuid("cccccccc-cccc-cccc-cccc-cccccccccccc");
    let doc_id = parse_uuid("dddddddd-dddd-dddd-dddd-dddddddddddd");
    let progress = RdfProgress {
        schema_version: SchemaVersion::V1,
        job_id,
        doc_id,
        context_id: 12,
        stage: RdfStage::InformationExtraction,
        percent: 45,
        stage_current: Some(3),
        stage_total: Some(8),
        stage_percent: Some(37),
        message: Some("running IE".into()),
        timestamp: Utc::now(),
    };

    let json = serde_json::to_string(&progress).expect("serialize rdf progress");
    let decoded: RdfProgress = serde_json::from_str(&json).expect("deserialize rdf progress");

    assert_eq!(decoded.job_id, progress.job_id);
    assert_eq!(decoded.doc_id, progress.doc_id);
    assert_eq!(decoded.context_id, progress.context_id);
    assert_eq!(decoded.stage, progress.stage);
    assert_eq!(decoded.percent, 45);
    assert_eq!(decoded.stage_current, Some(3));
    assert_eq!(decoded.stage_total, Some(8));
    assert_eq!(decoded.stage_percent, Some(37));
}

#[test]
fn rdf_result_success_roundtrip() {
    let job_id = parse_uuid("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee");
    let doc_id = parse_uuid("ffffffff-ffff-ffff-ffff-ffffffffffff");
    let event = RdfEvent::Result(RdfResult::Success {
        schema_version: SchemaVersion::V1,
        job_id,
        doc_id,
        context_id: 5,
        completed_at: Utc::now(),
        timings: PipelineTimings {
            fetch_ms: 12.0,
            segment_ms: 8.5,
            language_ms: 3.0,
            ie_ms: 42.1,
            shacl_ms: 6.0,
            reasoning_ms: 11.2,
            shacl_af_ms: 2.5,
            sparql_ms: 9.8,
        },
        stats: None,
    });

    let value = serde_json::to_value(&event).expect("serialize rdf success");
    let decoded: RdfEvent = serde_json::from_value(value).expect("deserialize rdf success");

    match decoded {
        RdfEvent::Result(RdfResult::Success {
            job_id,
            doc_id,
            context_id,
            timings,
            ..
        }) => {
            assert_eq!(job_id, parse_uuid("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee"));
            assert_eq!(doc_id, parse_uuid("ffffffff-ffff-ffff-ffff-ffffffffffff"));
            assert_eq!(context_id, 5);
            assert!((timings.language_ms - 3.0).abs() < f64::EPSILON);
            assert!((timings.ie_ms - 42.1).abs() < f64::EPSILON);
            assert!((timings.shacl_af_ms - 2.5).abs() < f64::EPSILON);
        }
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn rdf_result_failure_roundtrip() {
    let job_id = parse_uuid("11111111-aaaa-bbbb-cccc-111111111111");
    let doc_id = parse_uuid("22222222-aaaa-bbbb-cccc-222222222222");
    let event = RdfEvent::Result(RdfResult::Failure {
        schema_version: SchemaVersion::V1,
        job_id,
        doc_id,
        context_id: 6,
        stage: RdfStage::ShaclValidate,
        kind: FailureKind::Upstream,
        message: "upstream provider throttled".into(),
        details: Some("provider=embedding_service status=429".into()),
        occurred_at: Utc::now(),
    });

    let json = serde_json::to_string(&event).expect("serialize rdf failure");
    let decoded: RdfEvent = serde_json::from_str(&json).expect("deserialize rdf failure");

    match decoded {
        RdfEvent::Result(RdfResult::Failure {
            job_id,
            doc_id,
            stage,
            context_id,
            kind,
            details,
            ..
        }) => {
            assert_eq!(job_id, parse_uuid("11111111-aaaa-bbbb-cccc-111111111111"));
            assert_eq!(doc_id, parse_uuid("22222222-aaaa-bbbb-cccc-222222222222"));
            assert_eq!(context_id, 6);
            assert_eq!(stage, RdfStage::ShaclValidate);
            assert_eq!(kind, FailureKind::Upstream);
            assert_eq!(
                details.as_deref(),
                Some("provider=embedding_service status=429")
            );
        }
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn rdf_result_retryable_roundtrip() {
    let job_id = parse_uuid("33333333-aaaa-bbbb-cccc-333333333333");
    let doc_id = parse_uuid("44444444-aaaa-bbbb-cccc-444444444444");
    let event = RdfEvent::Result(RdfResult::Retryable {
        schema_version: SchemaVersion::V1,
        job_id,
        doc_id,
        context_id: 7,
        stage: RdfStage::Persist,
        kind: FailureKind::Upstream,
        message: "provider throttled request".into(),
        details: Some("provider=embedding_service operation=embedding_request status=429".into()),
        retry_after_seconds: Some(31),
        occurred_at: Utc::now(),
    });

    let json = serde_json::to_string(&event).expect("serialize rdf retryable");
    let decoded: RdfEvent = serde_json::from_str(&json).expect("deserialize rdf retryable");

    match decoded {
        RdfEvent::Result(RdfResult::Retryable {
            job_id,
            doc_id,
            context_id,
            stage,
            kind,
            details,
            retry_after_seconds,
            ..
        }) => {
            assert_eq!(job_id, parse_uuid("33333333-aaaa-bbbb-cccc-333333333333"));
            assert_eq!(doc_id, parse_uuid("44444444-aaaa-bbbb-cccc-444444444444"));
            assert_eq!(context_id, 7);
            assert_eq!(stage, RdfStage::Persist);
            assert_eq!(kind, FailureKind::Upstream);
            assert_eq!(
                details.as_deref(),
                Some("provider=embedding_service operation=embedding_request status=429")
            );
            assert_eq!(retry_after_seconds, Some(31));
        }
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn rdf_stage_labels_roundtrip() {
    assert_eq!(
        RdfStage::InformationExtraction.to_string(),
        "information_extraction"
    );
    assert_eq!(
        "shacl_validate".parse::<RdfStage>().unwrap(),
        RdfStage::ShaclValidate
    );
    assert!("unknown".parse::<RdfStage>().is_err());
}
