use serde::{Deserialize, Serialize};

/// Statistics for data-masking.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataMaskingStatistics {
    /// Whether any accessed data was protected by the data masking.
    pub data_masking_applied: bool,
}
