use std::collections::{BTreeMap, HashSet};

use uuid::Uuid;

use super::{PromptRuntimeComponent, PromptRuntimeError, TargetPromptBinding};

pub(super) fn validate_runtime_identity(id: Uuid, version: i32) -> Result<(), PromptRuntimeError> {
    if id.is_nil() {
        return Err(PromptRuntimeError::NilRuntimeBundleId);
    }
    if version <= 0 {
        return Err(PromptRuntimeError::InvalidRuntimeBundleVersion);
    }
    Ok(())
}

pub(super) fn validate_components(
    components: &[PromptRuntimeComponent],
) -> Result<(), PromptRuntimeError> {
    if components.is_empty() {
        return Err(PromptRuntimeError::EmptyRuntimeComponents);
    }
    let mut ids = HashSet::with_capacity(components.len());
    for component in components {
        component.validate()?;
        if !ids.insert(component.id.as_str()) {
            return Err(PromptRuntimeError::DuplicateComponentId(
                component.id.clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_targets(
    targets: &BTreeMap<String, TargetPromptBinding>,
) -> Result<(), PromptRuntimeError> {
    if targets.is_empty() {
        return Err(PromptRuntimeError::EmptyTargets);
    }
    for (target_key, binding) in targets {
        if target_key.is_empty() || target_key.trim() != target_key {
            return Err(PromptRuntimeError::InvalidTargetKey(target_key.clone()));
        }
        binding.validate()?;
    }
    Ok(())
}

pub(super) fn validate_hash(value: &str, field: &'static str) -> Result<(), PromptRuntimeError> {
    let valid = value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !valid {
        return Err(PromptRuntimeError::InvalidHash(field));
    }
    Ok(())
}
