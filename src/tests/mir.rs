use chrono::Utc;
use serde_json::json;

use crate::{
    ArtifactRef, FailureKind, MirEvent, MirProgress, MirRequest, MirResult, MirStage, SchemaVersion,
};

use super::common::parse_uuid;

#[test]
fn mir_request_defaults_are_applied() {
    let job_id = parse_uuid("11111111-1111-1111-1111-111111111111");
    let value = json!({
        "job_id": job_id,
        "context_id": 42,
        "source": {
            "kind": "s3",
            "bucket": "documents",
            "key": "uploads/doc.pdf"
        },
        "output": {
            "bucket": "artifacts"
        }
    });

    let request: MirRequest = serde_json::from_value(value).expect("valid request json");
    assert_eq!(request.schema_version, SchemaVersion::V1);
    assert_eq!(request.attempt, 1);
    assert_eq!(request.context_id, 42);
    assert!(!request.dry_run);
    assert!(request.user_id.is_none());
    assert!(request.submitted_at.is_none());
}

#[test]
fn mir_progress_roundtrip() {
    let job_id = parse_uuid("22222222-2222-2222-2222-222222222222");
    let progress = MirProgress {
        schema_version: SchemaVersion::V1,
        job_id,
        context_id: 7,
        stage: MirStage::ProcessingRun,
        percent: 65,
        message: Some("processing is running".into()),
        timestamp: Utc::now(),
    };

    let json = serde_json::to_string(&progress).expect("serialize progress");
    let decoded: MirProgress = serde_json::from_str(&json).expect("deserialize progress");

    assert_eq!(decoded.job_id, progress.job_id);
    assert_eq!(decoded.context_id, progress.context_id);
    assert_eq!(decoded.stage, progress.stage);
    assert_eq!(decoded.percent, progress.percent);
    assert_eq!(decoded.message, progress.message);
    assert_eq!(decoded.schema_version, SchemaVersion::V1);
}

#[test]
fn mir_result_failure_roundtrip() {
    let job_id = parse_uuid("33333333-3333-3333-3333-333333333333");
    let event = MirEvent::Result(MirResult::Failure {
        schema_version: SchemaVersion::V1,
        job_id,
        kind: FailureKind::Pandoc,
        message: "pandoc exited with code 1".into(),
        details: Some("stderr lines...".into()),
        occurred_at: Utc::now(),
        context_id: 9,
    });

    let json = serde_json::to_value(&event).expect("serialize result");
    let decoded: MirEvent = serde_json::from_value(json).expect("deserialize result");

    match decoded {
        MirEvent::Result(MirResult::Failure {
            job_id,
            kind,
            message,
            details,
            schema_version,
            context_id,
            ..
        }) => {
            assert_eq!(job_id, parse_uuid("33333333-3333-3333-3333-333333333333"));
            assert_eq!(kind, FailureKind::Pandoc);
            assert_eq!(message, "pandoc exited with code 1");
            assert_eq!(details.as_deref(), Some("stderr lines..."));
            assert_eq!(schema_version, SchemaVersion::V1);
            assert_eq!(context_id, 9);
        }
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn mir_result_retryable_roundtrip() {
    let job_id = parse_uuid("55555555-5555-5555-5555-555555555555");
    let event = MirEvent::Result(MirResult::Retryable {
        schema_version: SchemaVersion::V1,
        job_id,
        kind: FailureKind::Processing,
        message: "processing throttled".into(),
        details: None,
        occurred_at: Utc::now(),
        context_id: 11,
    });

    let value = serde_json::to_value(&event).expect("serialize retryable");
    let decoded: MirEvent = serde_json::from_value(value).expect("deserialize retryable");

    match decoded {
        MirEvent::Result(MirResult::Retryable {
            job_id,
            kind,
            context_id,
            ..
        }) => {
            assert_eq!(job_id, parse_uuid("55555555-5555-5555-5555-555555555555"));
            assert_eq!(kind, FailureKind::Processing);
            assert_eq!(context_id, 11);
        }
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn mir_result_success_contains_artifacts() {
    let job_id = parse_uuid("44444444-4444-4444-4444-444444444444");
    let event = MirEvent::Result(MirResult::Success {
        schema_version: SchemaVersion::V1,
        job_id,
        artifacts: vec![
            ArtifactRef {
                bucket: "artifacts".into(),
                key: format!("jobs/{job_id}/final-artifact.tar.gz"),
                content_type: Some("application/gzip".into()),
                size_bytes: Some(12_345),
            },
            ArtifactRef {
                bucket: "artifacts".into(),
                key: format!("jobs/{job_id}/document.md"),
                content_type: Some("text/markdown".into()),
                size_bytes: None,
            },
        ],
        stats: None,
        completed_at: Utc::now(),
        context_id: 3,
    });

    let value = serde_json::to_value(&event).expect("serialize success");
    let decoded: MirEvent = serde_json::from_value(value).expect("deserialize success");

    match decoded {
        MirEvent::Result(MirResult::Success {
            artifacts,
            context_id,
            ..
        }) => {
            assert_eq!(artifacts.len(), 2);
            assert!(
                artifacts
                    .iter()
                    .any(|a| a.key.ends_with("final-artifact.tar.gz"))
            );
            assert_eq!(context_id, 3);
        }
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn mir_stage_labels_roundtrip() {
    assert_eq!(MirStage::ProcessingRun.to_string(), "processing_run");
    assert_eq!(
        "processing_assemble".parse::<MirStage>().unwrap(),
        MirStage::ProcessingAssemble
    );
    assert!("unknown".parse::<MirStage>().is_err());
}
