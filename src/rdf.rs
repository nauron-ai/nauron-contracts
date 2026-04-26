use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::StageParseError;

/// Logical stages executed by the RDF enrichment worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "rdf_job_stage", rename_all = "snake_case")
)]
pub enum RdfStage {
    /// Job accepted by the worker.
    Received,
    /// Fetching UTF-8 text from the provided URI.
    FetchText,
    /// Segmenting markdown into analysis-ready chunks.
    Segment,
    /// Running information extraction models.
    InformationExtraction,
    /// Validating facts against SHACL shapes.
    ShaclValidate,
    /// Materialising OWL 2 RL inferences.
    Reasoning,
    /// Writing facts/inferences to Fuseki.
    Persist,
    /// All done.
    Completed,
}

impl std::fmt::Display for RdfStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Received => "received",
            Self::FetchText => "fetch_text",
            Self::Segment => "segment",
            Self::InformationExtraction => "information_extraction",
            Self::ShaclValidate => "shacl_validate",
            Self::Reasoning => "reasoning",
            Self::Persist => "persist",
            Self::Completed => "completed",
        };
        f.write_str(label)
    }
}

impl FromStr for RdfStage {
    type Err = StageParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "received" => Ok(Self::Received),
            "fetch_text" => Ok(Self::FetchText),
            "segment" => Ok(Self::Segment),
            "information_extraction" => Ok(Self::InformationExtraction),
            "shacl_validate" => Ok(Self::ShaclValidate),
            "reasoning" => Ok(Self::Reasoning),
            "persist" => Ok(Self::Persist),
            "completed" => Ok(Self::Completed),
            _ => Err(StageParseError::new(value)),
        }
    }
}

/// Execution timings emitted for successful RDF jobs.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PipelineTimings {
    /// Duration of the HTTP/S3 fetch stage in milliseconds.
    pub fetch_ms: f64,
    /// Duration of the segmentation stage in milliseconds.
    pub segment_ms: f64,
    /// Duration of language detection in milliseconds.
    #[serde(default)]
    pub language_ms: f64,
    /// Duration of the information extraction stage in milliseconds.
    pub ie_ms: f64,
    /// Duration of the SHACL validation stage in milliseconds.
    pub shacl_ms: f64,
    /// Duration of the reasoning stage in milliseconds.
    pub reasoning_ms: f64,
    /// Duration of SHACL-AF execution in milliseconds.
    #[serde(default)]
    pub shacl_af_ms: f64,
    /// Duration of the SPARQL persistence stage in milliseconds.
    pub sparql_ms: f64,
}
