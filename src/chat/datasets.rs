use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ChatValidationError, MAX_CHAT_TABLES};

pub const MAX_DATA_PAGE_ROWS: u32 = 1000;
pub const MAX_DATA_FILTERS: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatDataset {
    pub id: String,
    pub name: String,
    pub columns: Vec<String>,
    pub row_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatDataSource {
    pub query_url: String,
    pub lease_owner: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatDataFilter {
    pub column: String,
    pub operator: ChatDataOperator,
    pub value: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatDataOperator {
    Equal,
    Contains,
    GreaterOrEqual,
    LessOrEqual,
    IsEmpty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatDataQuery {
    pub table_id: String,
    pub filters: Vec<ChatDataFilter>,
    #[serde(default)]
    pub columns: Vec<String>,
    pub offset: u32,
    pub limit: u32,
    pub summary_column: Option<String>,
}

impl ChatDataQuery {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        if self.table_id.is_empty()
            || !(1..=MAX_DATA_PAGE_ROWS).contains(&self.limit)
            || self.columns.len() > super::MAX_CHAT_TABLE_COLUMNS
            || self.filters.len() > MAX_DATA_FILTERS
            || self
                .filters
                .iter()
                .any(|filter| filter.column.is_empty() || filter.value.len() > 1000)
        {
            return Err(ChatValidationError::InvalidTable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatDataPage {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub matching_rows: u64,
    pub next_offset: Option<u32>,
    pub summary: Option<ChatDataSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatDataSummary {
    pub column: String,
    pub numeric_rows: u64,
    pub empty_rows: u64,
    pub nonnumeric_rows: u64,
    pub sum: Option<String>,
    pub average: Option<String>,
    pub minimum: Option<String>,
    pub maximum: Option<String>,
}

pub fn validate_chat_datasets(
    datasets: &[ChatDataset],
    source: Option<&ChatDataSource>,
) -> Result<(), ChatValidationError> {
    if datasets.len() > MAX_CHAT_TABLES {
        return Err(ChatValidationError::TableLimit);
    }
    if datasets.is_empty() && source.is_some() {
        return Err(ChatValidationError::InvalidTable);
    }
    let mut ids = std::collections::BTreeSet::new();
    if datasets.iter().any(|table| {
        table.id.is_empty()
            || table.name.is_empty()
            || table.columns.is_empty()
            || table.columns.len() > super::MAX_CHAT_TABLE_COLUMNS
            || table.columns.iter().any(|column| column.trim().is_empty())
            || table
                .columns
                .iter()
                .map(|column| column.trim().to_lowercase())
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != table.columns.len()
            || !ids.insert(&table.id)
    }) || source.is_some_and(|source| source.lease_owner.is_nil() || source.query_url.is_empty())
    {
        return Err(ChatValidationError::InvalidTable);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_dataset_descriptors_do_not_contain_rows_and_queries_are_bounded() {
        let dataset = ChatDataset {
            id: "snapshot:1".into(),
            name: "Snapshot".into(),
            columns: vec!["Amount".into()],
            row_count: 1_000_000,
        };
        assert!(validate_chat_datasets(std::slice::from_ref(&dataset), None).is_ok());
        assert!(serde_json::to_string(&dataset).unwrap().len() < 150);
        let mut query = ChatDataQuery {
            table_id: dataset.id,
            filters: Vec::new(),
            columns: Vec::new(),
            offset: 0,
            limit: 100,
            summary_column: None,
        };
        assert!(query.validate().is_ok());
        query.limit = MAX_DATA_PAGE_ROWS + 1;
        assert!(query.validate().is_err());
        query.limit = 1;
        query.filters = vec![
            ChatDataFilter {
                column: "Amount".into(),
                operator: ChatDataOperator::Equal,
                value: "0".into()
            };
            MAX_DATA_FILTERS + 1
        ];
        assert!(query.validate().is_err());
    }
}
