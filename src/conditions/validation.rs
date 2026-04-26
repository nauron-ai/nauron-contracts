use serde_json::Value;
use std::fmt;

use super::types::{ConditionEvaluationOptions, ConditionEvaluationRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionValidationError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for ConditionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionEvaluationOptionsResolved {
    pub max_candidates: u32,
    pub max_hops: u8,
    pub evidence_lang: Option<String>,
}

pub const MAX_CONDITIONS_PER_REQUEST: usize = 1000;
pub const MAX_LABEL_LEN: usize = 256;
pub const MAX_DESCRIPTION_LEN: usize = 4000;
pub const MAX_PARAMETERS_SIZE_BYTES: usize = 16 * 1024;
pub const DEFAULT_MAX_CANDIDATES: u32 = 50;
pub const MAX_MAX_CANDIDATES: u32 = 200;
pub const DEFAULT_MAX_HOPS: u8 = 5;
pub const MAX_MAX_HOPS: u8 = 8;

#[derive(Debug, Clone, Copy)]
pub struct ConditionLimits {
    pub max_conditions_per_request: usize,
    pub max_label_len: usize,
    pub max_description_len: usize,
    pub max_parameters_size_bytes: usize,
    pub default_max_candidates: u32,
    pub max_max_candidates: u32,
    pub default_max_hops: u8,
    pub max_max_hops: u8,
}

impl Default for ConditionLimits {
    fn default() -> Self {
        Self {
            max_conditions_per_request: MAX_CONDITIONS_PER_REQUEST,
            max_label_len: MAX_LABEL_LEN,
            max_description_len: MAX_DESCRIPTION_LEN,
            max_parameters_size_bytes: MAX_PARAMETERS_SIZE_BYTES,
            default_max_candidates: DEFAULT_MAX_CANDIDATES,
            max_max_candidates: MAX_MAX_CANDIDATES,
            default_max_hops: DEFAULT_MAX_HOPS,
            max_max_hops: MAX_MAX_HOPS,
        }
    }
}

fn parameters_size_bytes(idx: usize, value: &Value) -> Result<usize, ConditionValidationError> {
    let bytes = serde_json::to_vec(value).map_err(|err| ConditionValidationError {
        field: format!("conditions[{}].parameters", idx),
        message: format!("failed to serialize: {}", err),
    })?;
    Ok(bytes.len())
}

pub fn normalize_options(
    options: Option<&ConditionEvaluationOptions>,
    limits: ConditionLimits,
) -> Result<ConditionEvaluationOptionsResolved, ConditionValidationError> {
    let (max_candidates_raw, max_hops_raw, evidence_lang) = match options {
        Some(opts) => (
            opts.max_candidates,
            opts.max_hops,
            opts.evidence_lang.clone().or(opts.lang.clone()),
        ),
        None => (None, None, None),
    };

    let max_candidates = max_candidates_raw.unwrap_or(limits.default_max_candidates);
    if max_candidates == 0 || max_candidates > limits.max_max_candidates {
        return Err(ConditionValidationError {
            field: "options.max_candidates".into(),
            message: format!(
                "must be between 1 and {} (got {})",
                limits.max_max_candidates, max_candidates
            ),
        });
    }

    let max_hops = max_hops_raw.unwrap_or(limits.default_max_hops);
    if max_hops == 0 || max_hops > limits.max_max_hops {
        return Err(ConditionValidationError {
            field: "options.max_hops".into(),
            message: format!(
                "must be between 1 and {} (got {})",
                limits.max_max_hops, max_hops
            ),
        });
    }

    Ok(ConditionEvaluationOptionsResolved {
        max_candidates,
        max_hops,
        evidence_lang,
    })
}

pub fn validate_request(
    req: &ConditionEvaluationRequest,
    limits: ConditionLimits,
) -> Result<ConditionEvaluationOptionsResolved, ConditionValidationError> {
    if req.conditions.is_empty() {
        return Err(ConditionValidationError {
            field: "conditions".into(),
            message: "must not be empty".into(),
        });
    }

    for (idx, condition) in req.conditions.iter().enumerate() {
        if condition.label.len() > limits.max_label_len {
            return Err(ConditionValidationError {
                field: format!("conditions[{}].label", idx),
                message: format!("must be at most {} chars", limits.max_label_len),
            });
        }
        if condition.description.len() > limits.max_description_len {
            return Err(ConditionValidationError {
                field: format!("conditions[{}].description", idx),
                message: format!("must be at most {} chars", limits.max_description_len),
            });
        }

        let params_size = parameters_size_bytes(idx, &condition.parameters)?;
        if params_size > limits.max_parameters_size_bytes {
            return Err(ConditionValidationError {
                field: format!("conditions[{}].parameters", idx),
                message: format!(
                    "parameters too large: {} bytes (max {})",
                    params_size, limits.max_parameters_size_bytes
                ),
            });
        }
    }

    normalize_options(req.options.as_ref(), limits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_options_prefers_evidence_lang_over_legacy_lang() {
        let options = ConditionEvaluationOptions {
            max_candidates: Some(5),
            max_hops: Some(2),
            evidence_lang: Some("pl".to_string()),
            lang: Some("en".to_string()),
        };

        let resolved = normalize_options(Some(&options), ConditionLimits::default()).unwrap();

        assert_eq!(resolved.evidence_lang.as_deref(), Some("pl"));
    }

    #[test]
    fn normalize_options_uses_legacy_lang_when_new_field_missing() {
        let options = ConditionEvaluationOptions {
            max_candidates: Some(5),
            max_hops: Some(2),
            evidence_lang: None,
            lang: Some("en".to_string()),
        };

        let resolved = normalize_options(Some(&options), ConditionLimits::default()).unwrap();

        assert_eq!(resolved.evidence_lang.as_deref(), Some("en"));
    }
}
