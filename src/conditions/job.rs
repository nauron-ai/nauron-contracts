use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use super::types::{
    ConditionContextMode, ConditionEvaluationOptions, ConditionEvaluationResponse, ConditionSpec,
};
use crate::types::{SchemaVersion, StageParseError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionsEvaluateStart {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub job_id: Uuid,
    pub context_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_doc_id: Option<Uuid>,
    pub conditions: Vec<ConditionSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ConditionEvaluationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_mode: Option<ConditionContextMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "conditions_job_stage", rename_all = "snake_case")
)]
pub enum ConditionsEvaluateStage {
    Queued,
    Received,
    Retrieve,
    Reason,
    Completed,
}

impl std::fmt::Display for ConditionsEvaluateStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Queued => "queued",
            Self::Received => "received",
            Self::Retrieve => "retrieve",
            Self::Reason => "reason",
            Self::Completed => "completed",
        };
        f.write_str(label)
    }
}

impl FromStr for ConditionsEvaluateStage {
    type Err = StageParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "queued" => Ok(Self::Queued),
            "received" => Ok(Self::Received),
            "retrieve" => Ok(Self::Retrieve),
            "reason" => Ok(Self::Reason),
            "completed" => Ok(Self::Completed),
            _ => Err(StageParseError::new(value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionsEvaluateProgress {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub job_id: Uuid,
    pub context_id: i32,
    pub stage: ConditionsEvaluateStage,
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
pub enum ConditionsEvaluateResult {
    Success {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        context_id: i32,
        response: ConditionEvaluationResponse,
        completed_at: DateTime<Utc>,
    },
    Failure {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        context_id: i32,
        error: super::types::ConditionErrorResponse,
        occurred_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConditionsEvaluateEvent {
    Progress(ConditionsEvaluateProgress),
    Result(ConditionsEvaluateResult),
}

#[cfg(test)]
mod tests {
    use super::ConditionsEvaluateStage;

    #[test]
    fn conditions_stage_labels_roundtrip() {
        assert_eq!(ConditionsEvaluateStage::Retrieve.to_string(), "retrieve");
        assert!(matches!(
            "reason".parse::<ConditionsEvaluateStage>().unwrap(),
            ConditionsEvaluateStage::Reason
        ));
        assert!("unknown".parse::<ConditionsEvaluateStage>().is_err());
    }
}
