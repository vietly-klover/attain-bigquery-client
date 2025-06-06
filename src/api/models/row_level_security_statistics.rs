use serde::{Deserialize, Serialize};

/// Statistics for row-level security.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowLevelSecurityStatistics {
    /// Whether any accessed data was protected by row access policies.
    pub row_level_security_applied: bool,
}
