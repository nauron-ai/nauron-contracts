use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::prompt::PromptLog;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReasoningTrace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptLog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<ReasoningNetwork>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasoning_steps: Vec<ReasoningStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReasoningNetwork {
    #[serde(default)]
    pub nodes: Vec<ReasoningNode>,
    #[serde(default)]
    pub edges: Vec<ReasoningEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReasoningNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReasoningEdge {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub edge_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReasoningStep {
    pub step: u32,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypotheses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_found: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
}
