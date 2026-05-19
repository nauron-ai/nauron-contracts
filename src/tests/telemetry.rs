use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    JobStepComponent, JobStepSeverity, JobStepStatus, JobStepTelemetry, JobTelemetryEvent,
};

#[test]
fn step_telemetry_roundtrip() {
    let payload = JobTelemetryEvent::Step(JobStepTelemetry {
        event_id: Uuid::nil(),
        job_id: Uuid::nil(),
        pipeline_id: Some(Uuid::nil()),
        context_id: 42,
        file_id: Some(7),
        document_id: Some(Uuid::nil()),
        component: JobStepComponent::Rdf,
        step: "ner".to_string(),
        status: JobStepStatus::Running,
        severity: JobStepSeverity::Info,
        percent: Some(33),
        current: Some(1),
        total: Some(3),
        attempt: Some(2),
        worker_id: Some("worker-1".to_string()),
        started_at: Some(Utc::now()),
        finished_at: None,
        duration_ms: None,
        message: Some("extracting named entities".to_string()),
        error: None,
        metrics: [("segments".to_string(), json!(120))].into_iter().collect(),
        emitted_at: Utc::now(),
    });

    let encoded = serde_json::to_string(&payload).unwrap();
    let decoded: JobTelemetryEvent = serde_json::from_str(&encoded).unwrap();

    match decoded {
        JobTelemetryEvent::Step(step) => {
            assert_eq!(step.component, JobStepComponent::Rdf);
            assert_eq!(step.status, JobStepStatus::Running);
            assert_eq!(step.normalized_percent(), Some(33));
            assert_eq!(step.metrics["segments"], json!(120));
        }
    }
}
