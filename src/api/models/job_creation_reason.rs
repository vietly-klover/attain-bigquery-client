use serde::{Deserialize, Serialize};

/// Reason about why a Job was created from a jobs.query method when used with JOB_CREATION_OPTIONAL Job creation mode.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobCreationReason {
    /// Output only. Specifies the high level reason why a Job was created.
    pub code: Option<JobCreationReasonCode>,
}

/// Enum for JobCreationReason code field.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobCreationReasonCode {
    CodeUnspecified,
    Requested,
    LongRunning,
    LargeResults,
    Other,
}

impl Default for JobCreationReasonCode {
    fn default() -> Self {
        JobCreationReasonCode::CodeUnspecified
    }
}
