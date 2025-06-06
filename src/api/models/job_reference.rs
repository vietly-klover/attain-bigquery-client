use serde::{Deserialize, Serialize};

/// A job reference is a fully qualified identifier for referring to a job.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobReference {
    /// Required. The ID of the project containing this job.
    pub project_id: String,

    /// Required. The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).
    pub job_id: String,

    /// Optional. The geographic location of the job. The default value is US.
    pub location: Option<String>,
}
