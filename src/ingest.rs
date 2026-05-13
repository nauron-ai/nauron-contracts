use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::types::{SchemaVersion, StageParseError};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IngestSchemaField {
    pub key: String,
    #[serde(default)]
    pub name: Option<String>,
    pub description: String,
    #[serde(
        default = "default_type_spec",
        deserialize_with = "deserialize_type_spec"
    )]
    pub r#type: Value,
    #[serde(default)]
    pub required: bool,
}

fn default_type_spec() -> Value {
    Value::String("string".to_string())
}

fn deserialize_type_spec<'de, D>(deserializer: D) -> Result<Value, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(_) | Value::Object(_) => Ok(value),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::Array(_) => Err(
            serde::de::Error::custom("ingest schema field type must be a string or object"),
        ),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestTokensUsed {
    pub prompt: Option<u32>,
    pub completion: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestStart {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub job_id: Uuid,
    pub context_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub schema: Vec<IngestSchemaField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_revision: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "ingest_job_stage", rename_all = "snake_case")
)]
pub enum IngestStage {
    Queued,
    Received,
    Llm,
    Persist,
    Completed,
}

impl std::fmt::Display for IngestStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Queued => "queued",
            Self::Received => "received",
            Self::Llm => "llm",
            Self::Persist => "persist",
            Self::Completed => "completed",
        };
        f.write_str(label)
    }
}

impl FromStr for IngestStage {
    type Err = StageParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "queued" => Ok(Self::Queued),
            "received" => Ok(Self::Received),
            "llm" => Ok(Self::Llm),
            "persist" => Ok(Self::Persist),
            "completed" => Ok(Self::Completed),
            _ => Err(StageParseError::new(value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestProgress {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub job_id: Uuid,
    pub context_id: i64,
    pub stage: IngestStage,
    pub percent: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progress_current: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progress_total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progress_pct: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IngestResult {
    Success {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        context_id: i64,
        data: Value,
        language: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tokens_used: Option<IngestTokensUsed>,
        completed_at: DateTime<Utc>,
    },
    Failure {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        context_id: i64,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        occurred_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IngestEvent {
    Progress(IngestProgress),
    Result(IngestResult),
}
