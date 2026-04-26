use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::rdf::{PipelineTimings, RdfStage};
use crate::types::{ArtifactRef, FailureKind, MirStage, MirStatus, SchemaVersion};

/// Progress update describing the current stage of a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirProgress {
    /// Schema revision for the payload.
    #[serde(default)]
    pub schema_version: SchemaVersion,
    /// Identifier of the job being tracked.
    pub job_id: Uuid,
    /// Context identifier the job belongs to.
    pub context_id: i32,
    /// Stage emitted by the worker.
    pub stage: MirStage,
    /// Percentage of completion (0-100).
    pub percent: u8,
    /// Optional human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Moment when the update was produced.
    pub timestamp: DateTime<Utc>,
}

/// Result message summarising terminal state of a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum MirResult {
    /// Job completed successfully and artifacts are available.
    Success {
        /// Schema revision for the payload.
        #[serde(default)]
        schema_version: SchemaVersion,
        /// Identifier of the processed job.
        job_id: Uuid,
        /// Context identifier where the job comes from.
        context_id: i32,
        /// Uploaded artifacts.
        artifacts: Vec<ArtifactRef>,
        /// Optional aggregate statistics.
        #[serde(skip_serializing_if = "Option::is_none")]
        stats: Option<MirStats>,
        /// Completion timestamp.
        completed_at: DateTime<Utc>,
    },
    /// Job failed to finish successfully.
    Failure {
        /// Schema revision for the payload.
        #[serde(default)]
        schema_version: SchemaVersion,
        /// Identifier of the job.
        job_id: Uuid,
        /// Context identifier where the job comes from.
        context_id: i32,
        /// Failure category.
        kind: FailureKind,
        /// Human-readable description of the error.
        message: String,
        /// Optional machine-readable details (stack trace, exit code...).
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        /// Timestamp when the failure was recorded.
        occurred_at: DateTime<Utc>,
    },
    /// Job failed but may be retried safely.
    Retryable {
        /// Schema revision for the payload.
        #[serde(default)]
        schema_version: SchemaVersion,
        /// Identifier of the job.
        job_id: Uuid,
        /// Context identifier where the job comes from.
        context_id: i32,
        /// Failure category.
        kind: FailureKind,
        /// Human-readable description of the error.
        message: String,
        /// Optional machine-readable details.
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        /// Timestamp when the retry recommendation was emitted.
        occurred_at: DateTime<Utc>,
    },
}

/// Aggregated counters produced on successful completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirStats {
    /// Total processing duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// Number of media assets detected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_count: Option<u32>,
    /// Number of OCR sections appended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_sections: Option<u32>,
    /// Number of textual sources combined in the final document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<u32>,
}

/// Envelope published by the worker onto progress/result topics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MirEvent {
    /// Incremental progress notification.
    Progress(MirProgress),
    /// Terminal job outcome.
    Result(MirResult),
}

impl MirResult {
    /// Returns the terminal status for the job result.
    pub fn status(&self) -> MirStatus {
        match self {
            MirResult::Success { .. } => MirStatus::Success,
            MirResult::Failure { .. } => MirStatus::Failure,
            MirResult::Retryable { .. } => MirStatus::Retryable,
        }
    }
}

/// Progress update describing the current RDF stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfProgress {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub job_id: Uuid,
    pub doc_id: Uuid,
    pub context_id: i32,
    pub stage: RdfStage,
    pub percent: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_current: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_percent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RdfResult {
    Success {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        doc_id: Uuid,
        context_id: i32,
        completed_at: DateTime<Utc>,
        timings: PipelineTimings,
        #[serde(skip_serializing_if = "Option::is_none")]
        stats: Option<RdfStats>,
    },
    Failure {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        doc_id: Uuid,
        context_id: i32,
        stage: RdfStage,
        kind: FailureKind,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        occurred_at: DateTime<Utc>,
    },
    Retryable {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        doc_id: Uuid,
        context_id: i32,
        stage: RdfStage,
        kind: FailureKind,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_after_seconds: Option<u32>,
        occurred_at: DateTime<Utc>,
    },
}

/// Envelope mirroring the dedicated RDF topics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RdfEvent {
    /// Incremental progress update.
    Progress(RdfProgress),
    /// Terminal RDF result.
    Result(RdfResult),
}

/// Reason why an inferred relation was discarded.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationMissingReason {
    InvalidSubject,
    UnknownPredicate,
    UnknownObjectEntity,
    MissingObjectSurface,
    MissingEvidenceMentions,
}

/// Counter for a missing-relation category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationMissingStat {
    /// Enumerated rejection reason.
    pub reason: RelationMissingReason,
    /// Number of relations rejected for this reason.
    pub count: u32,
}

/// Statistics for LLM-backed relation extraction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfRelationStats {
    /// Number of raw relations returned by the LLM.
    pub proposed: u32,
    /// Number of relations accepted after validation.
    pub accepted: u32,
    /// Breakdown of missing relations per reason.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<RelationMissingStat>,
}

/// Optional RDF statistics attached to success results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfStats {
    /// Relation-specific metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<RdfRelationStats>,
}
