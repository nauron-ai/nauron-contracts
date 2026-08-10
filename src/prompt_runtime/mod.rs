mod enums;
mod error;
mod hash;
#[cfg(test)]
mod tests;
mod transition;
#[cfg(test)]
mod transition_tests;
mod types;
mod validation;

pub use enums::{PromptActivationCondition, PromptRuntimeRole, PromptRuntimeStage};
pub use error::PromptRuntimeError;
pub use hash::{calculate_composite_hash, calculate_runtime_hash, sha256_hex};
pub use transition::{
    PromptRuntimeCatalog, PromptRuntimeHead, PromptRuntimeTransitionRequest,
    PromptRuntimeTransitionResponse,
};
pub use types::{
    CompositePromptManifestV2, PromptRuntimeBundle, PromptRuntimeComponent, TargetPromptBinding,
};

pub const PROMPT_RUNTIME_MANIFEST_VERSION: u16 = 2;
pub const PROMPT_RUNTIME_METADATA_KEY: &str = "_nauron_prompt_bundle";
pub const INFERENCER_COMPONENT_PREFIX: &str = "inferencer.";
