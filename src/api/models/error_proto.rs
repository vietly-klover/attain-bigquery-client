use serde::{Deserialize, Serialize};

/// Error details.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ErrorProto {
    /// Debugging information. This property is internal to Google and should not be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<String>,

    /// Specifies where the error occurred, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// A human-readable description of the error.
    pub message: String,

    /// A short error code that summarizes the error.
    pub reason: String,
}
