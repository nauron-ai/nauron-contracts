use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "graphql", graphql(rename_items = "lowercase"))]
#[serde(rename_all = "snake_case")]
pub enum ChatVerificationStatus {
    Verified,
    Unverified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatVerification {
    pub status: ChatVerificationStatus,
    pub explanation: Option<String>,
}

impl ChatVerification {
    pub fn verified() -> Self {
        Self {
            status: ChatVerificationStatus::Verified,
            explanation: None,
        }
    }
}
