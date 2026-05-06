use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::types::{SchemaVersion, StageParseError};

const DEFAULT_LANGUAGE: &str = "en";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCompileStart {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub job_id: Uuid,
    pub context_id: i32,
    pub revision: i32,
    #[serde(default)]
    pub options: KnowledgeCompileOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCompileOptions {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub require_conflicts_with: bool,
    #[serde(default)]
    pub max_conflict_nodes: Option<u64>,
}

impl Default for KnowledgeCompileOptions {
    fn default() -> Self {
        Self {
            language: default_language(),
            require_conflicts_with: false,
            max_conflict_nodes: None,
        }
    }
}

fn default_language() -> String {
    DEFAULT_LANGUAGE.to_string()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "knowledge_compile_stage", rename_all = "snake_case")
)]
pub enum KnowledgeCompileStage {
    Queued,
    Received,
    Evidence,
    Timeline,
    Compiled,
    Persist,
    Completed,
}

impl std::fmt::Display for KnowledgeCompileStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Queued => "queued",
            Self::Received => "received",
            Self::Evidence => "evidence",
            Self::Timeline => "timeline",
            Self::Compiled => "compiled",
            Self::Persist => "persist",
            Self::Completed => "completed",
        };
        f.write_str(label)
    }
}

impl FromStr for KnowledgeCompileStage {
    type Err = StageParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "queued" => Ok(Self::Queued),
            "received" => Ok(Self::Received),
            "evidence" => Ok(Self::Evidence),
            "timeline" => Ok(Self::Timeline),
            "compiled" => Ok(Self::Compiled),
            "persist" => Ok(Self::Persist),
            "completed" => Ok(Self::Completed),
            _ => Err(StageParseError::new(value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCompileProgress {
    #[serde(default)]
    pub schema_version: SchemaVersion,
    pub job_id: Uuid,
    pub context_id: i32,
    pub stage: KnowledgeCompileStage,
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
pub enum KnowledgeCompileResult {
    Success {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        context_id: i32,
        dossier_id: Uuid,
        dossier_name: String,
        revision: i32,
        completed_at: DateTime<Utc>,
    },
    Failure {
        #[serde(default)]
        schema_version: SchemaVersion,
        job_id: Uuid,
        context_id: i32,
        stage: KnowledgeCompileStage,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        occurred_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KnowledgeCompileEvent {
    Progress(KnowledgeCompileProgress),
    Result(KnowledgeCompileResult),
}

#[cfg(test)]
mod tests {
    use super::{
        KnowledgeCompileEvent, KnowledgeCompileOptions, KnowledgeCompileResult,
        KnowledgeCompileStage, KnowledgeCompileStart,
    };
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn knowledge_compile_options_default_to_english() {
        let options = KnowledgeCompileOptions::default();

        assert_eq!(options.language, "en");
        assert!(!options.require_conflicts_with);
        assert_eq!(options.max_conflict_nodes, None);
    }

    #[test]
    fn knowledge_compile_start_defaults_options() {
        let value = serde_json::json!({
            "job_id": "11111111-1111-1111-1111-111111111111",
            "context_id": 12,
            "revision": 3
        });

        let start: KnowledgeCompileStart = serde_json::from_value(value).unwrap();

        assert_eq!(start.options.language, "en");
        assert_eq!(start.revision, 3);
        assert_eq!(start.context_id, 12);
    }

    #[test]
    fn knowledge_compile_stage_labels_roundtrip() {
        assert_eq!(KnowledgeCompileStage::Evidence.to_string(), "evidence");
        assert!(matches!(
            "compiled".parse::<KnowledgeCompileStage>().unwrap(),
            KnowledgeCompileStage::Compiled
        ));
        assert!("unknown".parse::<KnowledgeCompileStage>().is_err());
    }

    #[test]
    fn knowledge_compile_result_and_event_tags_are_stable() {
        let result = KnowledgeCompileResult::Failure {
            schema_version: Default::default(),
            job_id: Uuid::nil(),
            context_id: 12,
            stage: KnowledgeCompileStage::Evidence,
            message: "failed".to_string(),
            details: None,
            occurred_at: Utc::now(),
        };

        let result_json = serde_json::to_value(&result).unwrap();
        assert_eq!(
            result_json.get("status").and_then(|value| value.as_str()),
            Some("failure")
        );

        let event_json = serde_json::to_value(KnowledgeCompileEvent::Result(result)).unwrap();
        assert_eq!(
            event_json.get("type").and_then(|value| value.as_str()),
            Some("result")
        );
    }
}
