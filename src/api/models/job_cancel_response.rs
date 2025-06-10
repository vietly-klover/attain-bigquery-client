use serde::{Deserialize, Serialize};

use super::job::Job;

/// Response from cancelling a job.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobCancelResponse {
    /// The job resource, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
