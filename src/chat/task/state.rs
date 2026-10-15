use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::{ChatSource, ChatValidationError};

pub const MAX_TASK_STATE_BYTES: usize = 8 * 1024;
pub const MAX_TASK_EVIDENCE_BYTES: usize = 512 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskMessage {
    pub id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskEvidenceRef {
    pub run_id: Uuid,
    pub source_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskInstruction {
    pub message_id: Uuid,
    pub quote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskFinding {
    pub statement: String,
    pub evidence: Vec<TaskEvidenceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskState {
    pub goal: String,
    pub completion_criteria: Vec<String>,
    pub instructions: Vec<TaskInstruction>,
    pub findings: Vec<TaskFinding>,
    pub hypotheses: Vec<String>,
    pub unresolved: Vec<String>,
    pub completed_steps: Vec<String>,
    pub next_step: String,
}

impl TaskState {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        if self.goal.trim().is_empty()
            || self.completion_criteria.is_empty()
            || self
                .instructions
                .iter()
                .any(|rule| rule.message_id.is_nil() || rule.quote.trim().is_empty())
            || self
                .findings
                .iter()
                .any(|finding| finding.statement.trim().is_empty() || finding.evidence.is_empty())
            || serde_json::to_vec(self)
                .map_err(|_| ChatValidationError::InvalidAction)?
                .len()
                > MAX_TASK_STATE_BYTES
        {
            return Err(ChatValidationError::InvalidAction);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskSnapshot {
    pub id: Uuid,
    pub revision: i64,
    pub state: TaskState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskEvidence {
    pub reference: TaskEvidenceRef,
    pub tool: String,
    pub source: ChatSource,
}
