use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{OutputTarget, SchemaVersion, SourceRef};

/// Request created by upstream services to trigger a MIR processing job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirRequest {
    /// Schema revision for the payload.
    #[serde(default)]
    pub schema_version: SchemaVersion,
    /// Globally unique job identifier.
    pub job_id: Uuid,
    /// Identifier of the context the document belongs to.
    pub context_id: i32,
    /// Optional identifier of the user initiating the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Location of the document to process.
    pub source: SourceRef,
    /// Where the produced artifacts should be uploaded.
    pub output: OutputTarget,
    /// Whether the job should only report the planned commands.
    #[serde(default)]
    pub dry_run: bool,
    /// Sequential attempt number, starting from 1.
    #[serde(default = "default_attempt")]
    pub attempt: u16,
    /// Optional timestamp assigned by the producer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<DateTime<Utc>>,
}

fn default_attempt() -> u16 {
    1
}

/// Request emitted by the gateway to start RDF enrichment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfStart {
    /// Schema revision for the payload.
    #[serde(default)]
    pub schema_version: SchemaVersion,
    /// Identifier that ties progress/result updates together.
    pub job_id: Uuid,
    /// Identifier of the MIR document to process.
    pub doc_id: Uuid,
    /// Context graph the document belongs to.
    #[serde(default)]
    pub context_id: i32,
    /// URI pointing at the UTF-8 text to ingest (HTTP/S3/etc.).
    pub text_uri: String,
    /// Upstream/source identifier that uniquely describes the asset.
    pub source_id: String,
    /// Optional timestamp assigned by the orchestrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<DateTime<Utc>>,
}
