use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PromptRuntimeError {
    #[error("unsupported prompt runtime manifest version: {0}")]
    InvalidManifestVersion(u16),
    #[error("prompt runtime bundle id must not be nil")]
    NilRuntimeBundleId,
    #[error("prompt runtime bundle version must be positive")]
    InvalidRuntimeBundleVersion,
    #[error("prompt runtime bundle must contain components")]
    EmptyRuntimeComponents,
    #[error("prompt runtime component id must use the inferencer namespace: {0}")]
    InvalidComponentId(String),
    #[error("prompt runtime component field is empty: {0}")]
    EmptyComponentField(&'static str),
    #[error("prompt runtime component id is duplicated: {0}")]
    DuplicateComponentId(String),
    #[error("prompt runtime hash is not a lowercase SHA-256 digest: {0}")]
    InvalidHash(&'static str),
    #[error("prompt runtime component content hash does not match: {0}")]
    ComponentContentHashMismatch(String),
    #[error("prompt runtime bundle hash does not match its components")]
    RuntimeHashMismatch,
    #[error("prompt runtime manifest must contain target bindings")]
    EmptyTargets,
    #[error("prompt runtime target key is invalid: {0}")]
    InvalidTargetKey(String),
    #[error("target prompt version id must not be nil")]
    NilPromptVersionId,
    #[error("target prompt version must be positive")]
    InvalidPromptVersion,
    #[error("prompt runtime composite hash does not match its contents")]
    CompositeHashMismatch,
}
