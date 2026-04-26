use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageParseError {
    value: String,
}

impl StageParseError {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl std::fmt::Display for StageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown stage label: {}", self.value)
    }
}

impl std::error::Error for StageParseError {}

/// Schema identifier carried by every public contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub u16);

impl SchemaVersion {
    /// Initial schema version for public contracts.
    pub const V1: SchemaVersion = SchemaVersion(1);
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

/// Location of the source document that should be processed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SourceRef {
    /// Object stored in an S3 compatible bucket.
    S3 {
        /// Bucket name that hosts the object.
        bucket: String,
        /// Object key inside the bucket.
        key: String,
        /// Optional version identifier (S3 versioned buckets).
        #[serde(skip_serializing_if = "Option::is_none")]
        version_id: Option<String>,
    },
    /// Absolute path on the worker filesystem (useful in tests).
    LocalPath {
        /// Fully-qualified filesystem path.
        path: String,
    },
}

/// Destination bucket/prefix pair for produced artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputTarget {
    /// Bucket where artifacts should be stored.
    pub bucket: String,
    /// Optional key prefix under which artifacts will be saved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl OutputTarget {
    /// Creates a new target with the provided bucket and optional prefix.
    pub fn new(bucket: impl Into<String>, prefix: Option<String>) -> Self {
        Self {
            bucket: bucket.into(),
            prefix,
        }
    }
}

/// Logical stages published in progress events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "mir_job_stage", rename_all = "snake_case")
)]
pub enum MirStage {
    /// Job accepted by the worker.
    Received,
    /// Input classification phase.
    Detect,
    /// Running Pandoc.
    PandocRun,
    /// Assembling Pandoc artifacts.
    PandocAssemble,
    /// Running document processing.
    ProcessingRun,
    /// Assembling processing artifacts.
    ProcessingAssemble,
    /// Uploading binaries to storage.
    Upload,
    /// Finished processing.
    Completed,
}

impl std::fmt::Display for MirStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Received => "received",
            Self::Detect => "detect",
            Self::PandocRun => "pandoc_run",
            Self::PandocAssemble => "pandoc_assemble",
            Self::ProcessingRun => "processing_run",
            Self::ProcessingAssemble => "processing_assemble",
            Self::Upload => "upload",
            Self::Completed => "completed",
        };
        f.write_str(label)
    }
}

impl FromStr for MirStage {
    type Err = StageParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "received" => Ok(Self::Received),
            "detect" => Ok(Self::Detect),
            "pandoc_run" => Ok(Self::PandocRun),
            "pandoc_assemble" => Ok(Self::PandocAssemble),
            "processing_run" => Ok(Self::ProcessingRun),
            "processing_assemble" => Ok(Self::ProcessingAssemble),
            "upload" => Ok(Self::Upload),
            "completed" => Ok(Self::Completed),
            _ => Err(StageParseError::new(value)),
        }
    }
}

/// Terminal status for MIR jobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirStatus {
    /// Job finished successfully.
    Success,
    /// Job failed and should not be retried automatically.
    Failure,
    /// Job failed but may be retried safely.
    Retryable,
}

/// High-level failure category used in result messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    Input,
    Pandoc,
    Processing,
    Storage,
    Internal,
    Upstream,
}

/// Reference to an artifact stored after successful processing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRef {
    /// Bucket that contains the artifact.
    pub bucket: String,
    /// Key of the artifact.
    pub key: String,
    /// Optional media type (e.g. `application/gzip`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Optional size hint in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}
