use crate::api::models::{
    get_query_results_response::GetQueryResultsResponse, job::Job,
    job_cancel_response::JobCancelResponse, job_list::JobList, query_request::QueryRequest,
    query_response::QueryResponse,
};

use crate::client::jobs::requests::get_query_results_request::GetQueryResultsRequest;
use crate::client::jobs::requests::list_jobs_request::ListJobsRequest;

use crate::config_provider::ConfigProvider;
use crate::credential_provider::CredentialProvider;
use anyhow::Result;
use std::sync::Arc;

pub struct Jobs {
    credential_provider: Arc<dyn CredentialProvider>,
    config_provider: Arc<dyn ConfigProvider>,
}

impl Jobs {
    pub fn new(
        credential_provider: Arc<dyn CredentialProvider>,
        config_provider: Arc<dyn ConfigProvider>,
    ) -> Self {
        Self {
            credential_provider,
            config_provider,
        }
    }

    /// Requests that a job be cancelled.
    pub async fn cancel_job(
        &self,
        project_id: &str,
        job_id: &str,
        location: Option<&str>,
    ) -> Result<JobCancelResponse> {
        todo!("Implement job cancel endpoint")
    }

    /// Requests the deletion of the metadata of a job.
    pub async fn delete_job(&self, project_id: &str, job_id: &str, location: &str) -> Result<()> {
        todo!("Implement job delete endpoint")
    }

    /// Returns information about a specific job.
    pub async fn get_job(
        &self,
        project_id: &str,
        job_id: &str,
        location: Option<&str>,
    ) -> Result<Job> {
        todo!("Implement job get endpoint")
    }

    /// RPC to get the results of a query job.
    pub async fn get_query_results<'a>(
        &self,
        request: GetQueryResultsRequest<'a>,
    ) -> Result<GetQueryResultsResponse> {
        todo!("Implement getQueryResults endpoint")
    }

    /// Starts a new asynchronous job.
    pub async fn insert_job(&self, project_id: &str, job: Job) -> Result<Job> {
        todo!("Implement job insert endpoint")
    }

    /// Lists all jobs that you started in the specified project.
    pub async fn list_jobs<'a>(&self, request: ListJobsRequest<'a>) -> Result<JobList> {
        todo!("Implement job list endpoint")
    }

    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.
    pub async fn query(&self, project_id: &str, request: QueryRequest) -> Result<QueryResponse> {
        todo!("Implement job query endpoint")
    }
}
