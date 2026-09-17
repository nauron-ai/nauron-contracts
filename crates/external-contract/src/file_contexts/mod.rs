mod summary;

use serde::{Deserialize, Serialize};
pub use summary::*;
use uuid::Uuid;

use crate::CallbackTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "file_work_state", rename_all = "snake_case")
)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum WorkState {
    Pending,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "file_context_error", rename_all = "snake_case")
)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum ContextError {
    InputInvalid,
    UnsupportedFormat,
    SourceUnavailable,
    ExtractionFailed,
    AnalysisFailed,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContextSnapshot {
    pub file_context_id: Uuid,
    pub file_id: Uuid,
    pub sha256: String,
    pub revision: u64,
    pub active: Option<AnalysisVersion>,
    pub pending: Option<AnalysisVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisVersion {
    pub analysis_id: Uuid,
    pub context_id: i64,
    pub generation: u32,
    pub profile_id: String,
    pub job_id: Uuid,
    pub context_state: WorkState,
    pub summary_state: WorkState,
    pub summary_revision: u64,
    pub context_error: Option<ContextError>,
    pub summary_error: Option<ContextError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReprocessFileRequest {
    pub request_id: Uuid,
    pub expected_analysis_id: Uuid,
    pub callback: CallbackTarget,
}

pub fn mime_type_for_extension(extension: &str) -> Option<&'static str> {
    Some(match extension {
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        "tif" | "tiff" => "image/tiff",
        _ => return None,
    })
}
