use serde::{Deserialize, Serialize};

/// Detailed statistics for DML statements
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DmlStatistics {
    /// Output only. Number of deleted Rows. populated by DML DELETE, MERGE and TRUNCATE statements.
    pub deleted_row_count: Option<String>,
    /// Output only. Number of inserted Rows. Populated by DML INSERT and MERGE statements
    pub inserted_row_count: Option<String>,
    /// Output only. Number of updated Rows. Populated by DML UPDATE and MERGE statements.
    pub updated_row_count: Option<String>,
}
