use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    pub fn target(component_id: &str) -> Self {
        Self {
            tuning_scope: PromptTuningScope::Target,
            allowed_component_ids: vec![component_id.to_string()],
        }
    }

    pub fn is_target(&self, component_id: &str) -> bool {
        self == &Self::target(component_id)
    }
}
