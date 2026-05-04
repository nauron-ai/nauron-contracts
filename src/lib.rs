//! Shared contracts for Nauron services and workers.

pub mod common;
pub mod conditions;
pub mod events;
pub mod health;
pub mod ingest;
pub mod rdf;
pub mod requests;
pub mod topics;
pub mod types;
pub mod vector;

pub use events::{
    MirEvent, MirProgress, MirResult, RdfEvent, RdfProgress, RdfRelationStats, RdfResult, RdfStats,
    RelationMissingReason, RelationMissingStat,
};
pub use ingest::{
    IngestEvent, IngestProgress, IngestResult, IngestSchemaField, IngestStage, IngestStart,
    IngestTokensUsed,
};
pub use rdf::{PipelineTimings, RdfStage};
pub use requests::{MirRequest, RdfStart};
pub use topics::{
    CONDITIONS_EVALUATE_PROGRESS_TOPIC, CONDITIONS_EVALUATE_RESULT_TOPIC,
    CONDITIONS_EVALUATE_START_TOPIC, INGEST_PROGRESS_TOPIC, INGEST_RESULT_TOPIC,
    INGEST_START_TOPIC, MIR_PROGRESS_TOPIC, MIR_REQUEST_TOPIC, MIR_RESULT_TOPIC, MIR_RETRY_TOPIC,
    ONTOLOGY_UPDATED_TOPIC, RDF_PROGRESS_TOPIC, RDF_RESULT_TOPIC, RDF_START_TOPIC,
    REPROCESS_CONTEXT_TOPIC,
};
pub use types::{
    ArtifactRef, FailureKind, MirStage, MirStatus, OutputTarget, SchemaVersion, SourceRef,
    StageParseError,
};

#[cfg(test)]
mod tests;
