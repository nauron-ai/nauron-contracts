use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    Ok,
    Degraded,
    Down,
}

impl ServiceStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, ServiceStatus::Ok)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ServiceStatus::Ok => "ok",
            ServiceStatus::Degraded => "degraded",
            ServiceStatus::Down => "down",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse<T> {
    pub service: String,
    pub status: ServiceStatus,
    pub uptime_seconds: u64,
    #[serde(flatten)]
    pub components: T,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ComponentStatus {
    pub status: ServiceStatus,
    pub latency_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
