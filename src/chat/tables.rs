use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ChatValidationError;

pub const MAX_CHAT_TABLE_BYTES: usize = 20 * 1024 * 1024;
pub const MAX_CHAT_TABLE_ROWS: usize = 100_000;
pub const MAX_CHAT_TABLE_COLUMNS: usize = 256;
pub const MAX_CHAT_TABLES: usize = 32;
pub const MAX_CHAT_ARTIFACTS: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatTable {
    pub id: String,
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatArtifact {
    pub id: Uuid,
    pub filename: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub fn validate_chat_tables(tables: &[ChatTable]) -> Result<(), ChatValidationError> {
    if tables.len() > MAX_CHAT_TABLES {
        return Err(ChatValidationError::TableLimit);
    }
    let mut ids = BTreeSet::new();
    let mut size = TableSize::default();
    for table in tables {
        if table.id.trim().is_empty() || table.name.trim().is_empty() || !ids.insert(&table.id) {
            return Err(ChatValidationError::InvalidTable);
        }
        size.bytes += table.id.len() + table.name.len();
        size.add(&table.columns, &table.rows)?;
    }
    Ok(())
}

pub fn validate_chat_artifacts(artifacts: &[ChatArtifact]) -> Result<(), ChatValidationError> {
    if artifacts.len() > MAX_CHAT_ARTIFACTS {
        return Err(ChatValidationError::TableLimit);
    }
    let mut ids = BTreeSet::new();
    let mut size = TableSize::default();
    for artifact in artifacts {
        if artifact.id.is_nil()
            || !ids.insert(artifact.id)
            || !valid_csv_filename(&artifact.filename)
        {
            return Err(ChatValidationError::InvalidTable);
        }
        size.bytes += artifact.filename.len();
        size.add(&artifact.columns, &artifact.rows)?;
    }
    Ok(())
}

pub fn valid_csv_filename(filename: &str) -> bool {
    filename.len() > 4
        && filename.len() <= 180
        && filename.ends_with(".csv")
        && !filename.starts_with('.')
        && !filename
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\' | '"'))
}

#[derive(Default)]
struct TableSize {
    bytes: usize,
    rows: usize,
}

impl TableSize {
    fn add(&mut self, columns: &[String], rows: &[Vec<String>]) -> Result<(), ChatValidationError> {
        let unique: BTreeSet<_> = columns
            .iter()
            .map(|column| column.trim().to_lowercase())
            .collect();
        if columns.is_empty()
            || columns.iter().any(|column| column.trim().is_empty())
            || unique.len() != columns.len()
            || rows.iter().any(|row| row.len() != columns.len())
        {
            return Err(ChatValidationError::InvalidTable);
        }
        self.rows += rows.len();
        self.bytes += columns.iter().map(String::len).sum::<usize>();
        self.bytes += rows.iter().flatten().map(String::len).sum::<usize>();
        if columns.len() > MAX_CHAT_TABLE_COLUMNS
            || self.rows > MAX_CHAT_TABLE_ROWS
            || self.bytes > MAX_CHAT_TABLE_BYTES
        {
            return Err(ChatValidationError::TableLimit);
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "tables_tests.rs"]
mod tests;
