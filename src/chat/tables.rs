use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ChatValidationError;

pub const MAX_CHAT_TABLE_BYTES: usize = 20 * 1024 * 1024;
pub const MAX_CHAT_TABLE_ROWS: usize = 100_000;
pub const MAX_CHAT_TABLE_COLUMNS: usize = 256;
pub const MAX_CHAT_TABLES: usize = 32;
pub const MAX_CHAT_ARTIFACTS: usize = 8;
const MAX_XLSX_WORKSHEETS: usize = 32;
const MAX_XLSX_CELL_CHARACTERS: usize = 32_767;
const MAX_XLSX_SHEET_CHARACTERS: usize = 31;
const RESERVED_WORKSHEET: &str = "History";
pub const CHAT_PRIMARY_WORKSHEET: &str = "Report";

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sheets: Vec<ChatWorksheet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatWorksheet {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatArtifactFormat {
    Csv,
    Xlsx,
}

impl ChatArtifactFormat {
    pub fn from_filename(filename: &str) -> Option<Self> {
        if !valid_filename(filename) {
            return None;
        }
        if filename.ends_with(".csv") {
            return Some(Self::Csv);
        }
        if filename.ends_with(".xlsx") {
            return Some(Self::Xlsx);
        }
        None
    }

    pub fn content_type(self) -> &'static str {
        match self {
            Self::Csv => "text/csv; charset=utf-8",
            Self::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        }
    }
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
        let format = ChatArtifactFormat::from_filename(&artifact.filename)
            .ok_or(ChatValidationError::InvalidTable)?;
        if artifact.id.is_nil()
            || !ids.insert(artifact.id)
            || (format == ChatArtifactFormat::Csv && !artifact.sheets.is_empty())
        {
            return Err(ChatValidationError::InvalidTable);
        }
        size.bytes += artifact.filename.len();
        size.add(&artifact.columns, &artifact.rows)?;
        if format == ChatArtifactFormat::Xlsx {
            validate_xlsx_cells(&artifact.columns, &artifact.rows)?;
        }
        if artifact.sheets.len() >= MAX_XLSX_WORKSHEETS {
            return Err(ChatValidationError::TableLimit);
        }
        validate_sheet_names(&artifact.sheets)?;
        for sheet in &artifact.sheets {
            size.bytes += sheet.name.len();
            size.add(&sheet.columns, &sheet.rows)?;
            validate_xlsx_cells(&sheet.columns, &sheet.rows)?;
        }
    }
    Ok(())
}

pub fn valid_csv_filename(filename: &str) -> bool {
    ChatArtifactFormat::from_filename(filename) == Some(ChatArtifactFormat::Csv)
}

fn valid_filename(filename: &str) -> bool {
    filename
        .rsplit_once('.')
        .is_some_and(|(stem, _)| !stem.is_empty())
        && filename.len() <= 180
        && !filename.starts_with('.')
        && !filename
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\' | '"'))
}

fn validate_sheet_names(sheets: &[ChatWorksheet]) -> Result<(), ChatValidationError> {
    let mut names = BTreeSet::from([CHAT_PRIMARY_WORKSHEET.to_lowercase()]);
    if sheets
        .iter()
        .any(|sheet| !valid_sheet_name(&sheet.name) || !names.insert(sheet.name.to_lowercase()))
    {
        return Err(ChatValidationError::InvalidTable);
    }
    Ok(())
}

fn valid_sheet_name(name: &str) -> bool {
    !name.trim().is_empty()
        && !name.eq_ignore_ascii_case(RESERVED_WORKSHEET)
        && name.encode_utf16().count() <= MAX_XLSX_SHEET_CHARACTERS
        && !name.starts_with('\'')
        && !name.ends_with('\'')
        && !name
            .chars()
            .any(|c| c.is_control() || matches!(c, ':' | '/' | '\\' | '?' | '*' | '[' | ']'))
}

fn validate_xlsx_cells(
    columns: &[String],
    rows: &[Vec<String>],
) -> Result<(), ChatValidationError> {
    if columns
        .iter()
        .chain(rows.iter().flatten())
        .any(|cell| cell.encode_utf16().count() > MAX_XLSX_CELL_CHARACTERS)
    {
        return Err(ChatValidationError::TableLimit);
    }
    Ok(())
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
