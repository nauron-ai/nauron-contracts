use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const TARGET_PROMPT_COMPONENT_ID: &str = "apcoa.datapoint.normalized_prompt";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PromptTuningScope {
    Target,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PromptTuningPolicy {
    pub tuning_scope: PromptTuningScope,
    pub allowed_component_ids: Vec<String>,
}

impl PromptTuningPolicy {
    pub fn target() -> Self {
        Self {
            tuning_scope: PromptTuningScope::Target,
            allowed_component_ids: vec![TARGET_PROMPT_COMPONENT_ID.to_string()],
        }
    }

    pub fn is_target(&self) -> bool {
        self == &Self::target()
    }
}
