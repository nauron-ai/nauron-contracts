/// Kafka / Redpanda topic carrying MIR job requests.
pub const MIR_REQUEST_TOPIC: &str = "mir.request";
/// Topic receiving progress updates for ongoing jobs.
pub const MIR_PROGRESS_TOPIC: &str = "mir.progress";
/// Topic receiving terminal results (success/failure) for jobs.
pub const MIR_RESULT_TOPIC: &str = "mir.result";
/// Topic used to reschedule retryable jobs.
pub const MIR_RETRY_TOPIC: &str = "mir.retry";
/// Topic instructing the RDF worker to start enrichment.
pub const RDF_START_TOPIC: &str = "rdf.start";
/// Topic receiving progress updates for RDF jobs.
pub const RDF_PROGRESS_TOPIC: &str = "rdf.progress";
/// Topic receiving terminal RDF results.
pub const RDF_RESULT_TOPIC: &str = "rdf.result";
pub const INGEST_START_TOPIC: &str = "ingest.start";
pub const INGEST_PROGRESS_TOPIC: &str = "ingest.progress";
pub const INGEST_RESULT_TOPIC: &str = "ingest.result";
pub const CONDITIONS_EVALUATE_START_TOPIC: &str = "conditions.evaluate.start";
pub const CONDITIONS_EVALUATE_PROGRESS_TOPIC: &str = "conditions.evaluate.progress";
pub const CONDITIONS_EVALUATE_RESULT_TOPIC: &str = "conditions.evaluate.result";
/// Topic emitted whenever ontologies are refreshed.
pub const ONTOLOGY_UPDATED_TOPIC: &str = "ontology.updated";
/// Topic requesting a context-level reprocessing cycle.
pub const REPROCESS_CONTEXT_TOPIC: &str = "reprocess.context";
