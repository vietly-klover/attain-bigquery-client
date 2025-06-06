use crate::api::models::error_proto::ErrorProto;
use crate::api::models::job_configuration::JobConfiguration;
use crate::api::models::job_reference::JobReference;
use crate::api::models::job_statistics::JobStatistics;
use crate::api::models::job_status::JobStatus;
use serde::{Deserialize, Serialize};

/// List of jobs response.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    pub etag: Option<String>,
    pub jobs: Option<Vec<JobListJob>>,
    pub kind: Option<String>,
    pub next_page_token: Option<String>,
    pub unreachable: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobListJob {
    pub configuration: Option<JobConfiguration>,
    pub error_result: Option<ErrorProto>,
    pub id: Option<String>,
    pub job_reference: Option<JobReference>,
    pub kind: Option<String>,
    pub principal_subject: Option<String>,
    pub state: Option<String>,
    pub statistics: Option<JobStatistics>,
    pub status: Option<JobStatus>,
    pub user_email: Option<String>,
}
