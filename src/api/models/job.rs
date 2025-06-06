use serde::{Deserialize, Serialize};

/// Represents a BigQuery Job resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    /// Output only. The type of the resource.
    pub kind: Option<String>,

    /// Output only. A hash of this resource.
    pub etag: Option<String>,

    /// Output only. Opaque ID field of the job.
    pub id: Option<String>,

    /// Output only. A URL that can be used to access the resource again.
    pub self_link: Option<String>,

    /// Output only. Email address of the user who ran the job.
    pub user_email: Option<String>,

    /// Required. Describes the job configuration.
    pub configuration: super::job_configuration::JobConfiguration,

    /// Optional. Reference describing the unique-per-user name of the job.
    pub job_reference: Option<super::job_reference::JobReference>,

    /// Output only. Information about the job, including starting time and ending time of the job.
    pub statistics: Option<super::job_statistics::JobStatistics>,

    /// Output only. The status of this job. Examine this value when polling an asynchronous job to see if the job is complete.
    pub status: Option<super::job_status::JobStatus>,

    /// Output only. [Full-projection-only] String representation of identity of requesting party.
    pub principal_subject: Option<String>,

    /// Output only. The reason why a Job was created. [Preview]
    pub job_creation_reason: Option<String>,
}
