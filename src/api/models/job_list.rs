use crate::api::models::error_proto::ErrorProto;
use crate::api::models::job_configuration::JobConfiguration;
use crate::api::models::job_reference::JobReference;
use crate::api::models::job_statistics::JobStatistics;
use crate::api::models::job_status::JobStatus;
use serde::{Deserialize, Serialize};

/// JobList is the response format for a jobs.list call.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    /// A hash of this page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// List of jobs that were requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobListJob>>,

    /// The resource type of the response.
    ///
    /// Default: "bigquery#jobList"
    pub kind: Option<String>,

    /// A token to request the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// A list of skipped locations that were unreachable. For more information about BigQuery locations, see:
    /// https://cloud.google.com/bigquery/docs/locations. Example: "europe-west5"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreachable: Option<Vec<String>>,
}

/// ListFormatJob is a partial projection of job information returned as part of a jobs.list response.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobListJob {
    /// Required. Describes the job configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<JobConfiguration>,

    /// A result object that will be present only if the job has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_result: Option<ErrorProto>,

    /// Unique opaque ID of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Unique opaque ID of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,

    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// [Full-projection-only] String representation of identity of requesting party. Populated for both first- and third-party identities. Only present for APIs that support third-party identities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_subject: Option<String>,

    /// Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Output only. Information about the job, including starting time and ending time of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<JobStatistics>,

    /// [Full-projection-only] Describes the status of this job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,

    /// [Full-projection-only] Email address of the user who ran the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}
