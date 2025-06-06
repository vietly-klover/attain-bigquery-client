use serde::{Deserialize, Serialize};

/// The status of a BigQuery Job.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    /// Output only. Final error result of the job. If present, indicates that the job has completed and was unsuccessful.
    pub error_result: Option<super::error_proto::ErrorProto>,

    /// Output only. The first errors encountered during the running of the job.
    pub errors: Option<Vec<super::error_proto::ErrorProto>>,

    /// Output only. Running state of the job. Valid states include 'PENDING', 'RUNNING', and 'DONE'.
    pub state: Option<String>,
}
