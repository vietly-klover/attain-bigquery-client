use serde::{Deserialize, Serialize};

/// Options for data format adjustments.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataFormatOptions {
    /// Optional. Output timestamp as usec int64. Default is false.
    pub use_int64_timestamp: Option<bool>,
}
