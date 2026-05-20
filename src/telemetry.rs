use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

const MAX_PERCENT: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "job_step_component", rename_all = "snake_case")
)]
pub enum JobStepComponent {
    Gateway,
    Queue,
    Sharepoint,
    Ocr,
    Mir,
    Rdf,
    Ner,
    Inferencer,
    Ingest,
    Conditions,
}

impl std::fmt::Display for JobStepComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Gateway => "gateway",
            Self::Queue => "queue",
            Self::Sharepoint => "sharepoint",
            Self::Ocr => "ocr",
            Self::Mir => "mir",
            Self::Rdf => "rdf",
            Self::Ner => "ner",
            Self::Inferencer => "inferencer",
            Self::Ingest => "ingest",
            Self::Conditions => "conditions",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "job_step_status", rename_all = "snake_case")
)]
pub enum JobStepStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Retrying,
    Skipped,
}

impl std::fmt::Display for JobStepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Retrying => "retrying",
            Self::Skipped => "skipped",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum JobStepSeverity {
    #[default]
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JobStepError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub message: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schema(value_type = Object)]
    pub details: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JobStepTelemetry {
    pub event_id: Uuid,
    pub job_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<Uuid>,
    pub context_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<Uuid>,
    pub component: JobStepComponent,
    pub step: String,
    pub status: JobStepStatus,
    #[serde(default)]
    pub severity: JobStepSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JobStepError>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schema(value_type = Object)]
    pub metrics: BTreeMap<String, Value>,
    pub emitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JobTelemetryEvent {
    Step(JobStepTelemetry),
}

impl JobStepTelemetry {
    pub fn normalized_percent(&self) -> Option<u8> {
        self.percent.map(|value| value.min(MAX_PERCENT))
    }
}
