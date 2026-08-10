use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::hash::{calculate_composite_hash, calculate_runtime_hash, sha256_hex};
use super::validation::{
    validate_components, validate_hash, validate_runtime_identity, validate_targets,
};
use super::{
    INFERENCER_COMPONENT_PREFIX, PROMPT_RUNTIME_MANIFEST_VERSION, PromptActivationCondition,
    PromptRuntimeError, PromptRuntimeRole, PromptRuntimeStage,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PromptRuntimeComponent {
    pub id: String,
    pub stage: PromptRuntimeStage,
    pub role: PromptRuntimeRole,
    pub activation_condition: PromptActivationCondition,
    pub execution_order: i32,
    pub content: String,
    pub content_hash: String,
}

impl PromptRuntimeComponent {
    pub fn new(
        id: impl Into<String>,
        stage: PromptRuntimeStage,
        role: PromptRuntimeRole,
        activation_condition: PromptActivationCondition,
        execution_order: i32,
        content: impl Into<String>,
    ) -> Result<Self, PromptRuntimeError> {
        let content = content.into();
        let component = Self {
            id: id.into(),
            stage,
            role,
            activation_condition,
            execution_order,
            content_hash: sha256_hex(content.as_bytes()),
            content,
        };
        component.validate()?;
        Ok(component)
    }

    pub fn validate(&self) -> Result<(), PromptRuntimeError> {
        if self.id.trim() != self.id
            || !self.id.starts_with(INFERENCER_COMPONENT_PREFIX)
            || self.id.len() == INFERENCER_COMPONENT_PREFIX.len()
        {
            return Err(PromptRuntimeError::InvalidComponentId(self.id.clone()));
        }
        if self.content.trim().is_empty() {
            return Err(PromptRuntimeError::EmptyComponentField("content"));
        }
        validate_hash(&self.content_hash, "content_hash")?;
        if sha256_hex(self.content.as_bytes()) != self.content_hash {
            return Err(PromptRuntimeError::ComponentContentHashMismatch(
                self.id.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PromptRuntimeBundle {
    pub id: Uuid,
    pub version: i32,
    pub runtime_hash: String,
    pub components: Vec<PromptRuntimeComponent>,
}

impl PromptRuntimeBundle {
    pub fn new(
        id: Uuid,
        version: i32,
        mut components: Vec<PromptRuntimeComponent>,
    ) -> Result<Self, PromptRuntimeError> {
        validate_runtime_identity(id, version)?;
        validate_components(&components)?;
        components.sort_by(|left, right| {
            left.execution_order
                .cmp(&right.execution_order)
                .then_with(|| left.id.cmp(&right.id))
        });
        let runtime_hash = calculate_runtime_hash(&components);
        Ok(Self {
            id,
            version,
            runtime_hash,
            components,
        })
    }

    pub fn validate(&self) -> Result<(), PromptRuntimeError> {
        validate_runtime_identity(self.id, self.version)?;
        validate_components(&self.components)?;
        validate_hash(&self.runtime_hash, "runtime_hash")?;
        if calculate_runtime_hash(&self.components) != self.runtime_hash {
            return Err(PromptRuntimeError::RuntimeHashMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TargetPromptBinding {
    pub prompt_version_id: Uuid,
    pub prompt_version: i32,
    pub prompt_hash: String,
}

impl TargetPromptBinding {
    pub fn new(
        prompt_version_id: Uuid,
        prompt_version: i32,
        prompt_hash: impl Into<String>,
    ) -> Result<Self, PromptRuntimeError> {
        let binding = Self {
            prompt_version_id,
            prompt_version,
            prompt_hash: prompt_hash.into(),
        };
        binding.validate()?;
        Ok(binding)
    }

    pub fn from_content(
        prompt_version_id: Uuid,
        prompt_version: i32,
        content: impl AsRef<[u8]>,
    ) -> Result<Self, PromptRuntimeError> {
        Self::new(prompt_version_id, prompt_version, sha256_hex(content))
    }

    pub fn validate(&self) -> Result<(), PromptRuntimeError> {
        if self.prompt_version_id.is_nil() {
            return Err(PromptRuntimeError::NilPromptVersionId);
        }
        if self.prompt_version <= 0 {
            return Err(PromptRuntimeError::InvalidPromptVersion);
        }
        validate_hash(&self.prompt_hash, "prompt_hash")?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CompositePromptManifestV2 {
    pub manifest_version: u16,
    pub runtime_bundle: PromptRuntimeBundle,
    pub targets: BTreeMap<String, TargetPromptBinding>,
    pub composite_hash: String,
}

impl CompositePromptManifestV2 {
    pub fn new(
        runtime_bundle: PromptRuntimeBundle,
        targets: BTreeMap<String, TargetPromptBinding>,
    ) -> Result<Self, PromptRuntimeError> {
        runtime_bundle.validate()?;
        validate_targets(&targets)?;
        let composite_hash = calculate_composite_hash(&runtime_bundle, &targets);
        Ok(Self {
            manifest_version: PROMPT_RUNTIME_MANIFEST_VERSION,
            runtime_bundle,
            targets,
            composite_hash,
        })
    }

    pub fn validate(&self) -> Result<(), PromptRuntimeError> {
        if self.manifest_version != PROMPT_RUNTIME_MANIFEST_VERSION {
            return Err(PromptRuntimeError::InvalidManifestVersion(
                self.manifest_version,
            ));
        }
        self.runtime_bundle.validate()?;
        validate_targets(&self.targets)?;
        validate_hash(&self.composite_hash, "composite_hash")?;
        if calculate_composite_hash(&self.runtime_bundle, &self.targets) != self.composite_hash {
            return Err(PromptRuntimeError::CompositeHashMismatch);
        }
        Ok(())
    }
}
