use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{CompositePromptManifestV2, PromptRuntimeComponent, TargetPromptBinding};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PromptRuntimeCatalog {
    pub manifest_version: u16,
    pub runtime_hash: String,
    pub components: Vec<PromptRuntimeComponent>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PromptRuntimeHead {
    pub runtime_bundle_id: Uuid,
    pub runtime_bundle_version: i32,
    pub runtime_hash: String,
    pub revision: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PromptRuntimeTransitionRequest {
    pub expected_head: PromptRuntimeHead,
    pub target_catalog: PromptRuntimeCatalog,
    pub transition_reference: String,
    pub actor: String,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PromptRuntimeTransitionResponse {
    pub transition_id: Uuid,
    pub previous_head: PromptRuntimeHead,
    pub head: PromptRuntimeHead,
    pub active_targets: BTreeMap<String, TargetPromptBinding>,
    pub invalidated_candidate_ids: Vec<Uuid>,
    pub manifest: CompositePromptManifestV2,
    pub transitioned_at: DateTime<Utc>,
}
