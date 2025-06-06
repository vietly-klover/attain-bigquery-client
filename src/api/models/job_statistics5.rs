use serde::{Deserialize, Serialize};

/// Statistics for a copy job.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics5 {
    /// Output only. Number of logical bytes copied to the destination table.
    pub copied_logical_bytes: String,

    /// Output only. Number of rows copied to the destination table.
    pub copied_rows: String,
}
