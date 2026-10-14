use super::{ChatValue, MAX_DATA_FILTERS};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "operator", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChatCatalogFilter {
    Equal {
        field: String,
        value: ChatValue,
    },
    Contains {
        field: String,
        value: ChatValue,
    },
    GreaterOrEqual {
        field: String,
        value: ChatValue,
    },
    LessOrEqual {
        field: String,
        value: ChatValue,
    },
    IsNull {
        field: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<()>,
    },
}

impl ChatCatalogFilter {
    pub fn field(&self) -> &str {
        match self {
            Self::Equal { field, .. }
            | Self::Contains { field, .. }
            | Self::GreaterOrEqual { field, .. }
            | Self::LessOrEqual { field, .. }
            | Self::IsNull { field, .. } => field,
        }
    }

    pub fn valid(&self) -> bool {
        match self {
            Self::Contains { value, .. } => matches!(value, ChatValue::Text(_)),
            Self::GreaterOrEqual { value, .. } | Self::LessOrEqual { value, .. } => {
                matches!(value, ChatValue::Text(_) | ChatValue::Number(_))
            }
            Self::Equal { .. } | Self::IsNull { .. } => true,
        }
    }

    pub fn matches(&self, actual: &ChatValue) -> bool {
        match self {
            Self::IsNull { .. } => matches!(actual, ChatValue::Null),
            Self::Equal { value, .. } => actual == value,
            Self::Contains { value, .. } => {
                match (actual, value) {
                    (ChatValue::Text(text), ChatValue::Text(fragment)) => text.to_lowercase().contains(&fragment.to_lowercase()),
                    (ChatValue::List(items), ChatValue::Text(fragment)) => items.iter().any(|item| matches!(item, ChatValue::Text(text) if text.to_lowercase().contains(&fragment.to_lowercase()))),
                    _ => false,
                }
            }
            Self::GreaterOrEqual { value, .. } => {
                ordered(actual, value).is_some_and(|order| !order.is_lt())
            }
            Self::LessOrEqual { value, .. } => {
                ordered(actual, value).is_some_and(|order| !order.is_gt())
            }
        }
    }
}

fn ordered(left: &ChatValue, right: &ChatValue) -> Option<std::cmp::Ordering> {
    match (left, right) {
        (ChatValue::Text(a), ChatValue::Text(b)) => Some(a.cmp(b)),
        (ChatValue::Number(a), ChatValue::Number(b)) => a.partial_cmp(b),
        _ => None,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChatCatalogFilterError {
    #[error(
        "At most {MAX_DATA_FILTERS} catalog predicates are allowed across filters and any_filters"
    )]
    Limit,
    #[error("unknown catalog field: {0}; use contract_fields first")]
    UnknownField(String),
    #[error("Filter value is incompatible with its operator for {0}")]
    InvalidValue(String),
}

pub fn validate_catalog_filters(
    filters: &[ChatCatalogFilter],
    any_filters: &[ChatCatalogFilter],
    fields: &BTreeSet<String>,
) -> Result<(), ChatCatalogFilterError> {
    if filters.len() + any_filters.len() > MAX_DATA_FILTERS {
        return Err(ChatCatalogFilterError::Limit);
    }
    for filter in filters.iter().chain(any_filters) {
        if !fields.contains(filter.field()) {
            return Err(ChatCatalogFilterError::UnknownField(filter.field().into()));
        }
        if !filter.valid() {
            return Err(ChatCatalogFilterError::InvalidValue(filter.field().into()));
        }
    }
    Ok(())
}

pub fn matches_catalog_filters(
    filters: &[ChatCatalogFilter],
    any_filters: &[ChatCatalogFilter],
    value: impl Fn(&str) -> ChatValue,
) -> bool {
    let all_match = filters
        .iter()
        .all(|filter| filter.matches(&value(filter.field())));
    let any_match = any_filters.is_empty()
        || any_filters
            .iter()
            .any(|filter| filter.matches(&value(filter.field())));
    all_match && any_match
}

pub fn missing_catalog_filter_values(
    filters: &[ChatCatalogFilter],
    any_filters: &[ChatCatalogFilter],
    value: impl Fn(&str) -> ChatValue,
) -> bool {
    filters
        .iter()
        .chain(any_filters)
        .any(|filter| matches!(value(filter.field()), ChatValue::Null))
}

mod schema;
pub use schema::catalog_filter_array_schema;

#[cfg(test)]
mod tests;
