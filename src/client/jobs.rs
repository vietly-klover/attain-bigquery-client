pub struct Jobs;

use crate::api::models::{
    get_query_results_response::GetQueryResultsResponse, job::Job,
    job_cancel_response::JobCancelResponse, job_list::JobList, query_request::QueryRequest,
    query_response::QueryResponse,
};

use anyhow::Result;

impl Jobs {
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
    pub async fn get_query_results(
        &self,
        project_id: &str,
        job_id: &str,
        location: Option<&str>,
        max_results: Option<u32>,
        page_token: Option<&str>,
        start_index: Option<&str>,
        timeout_ms: Option<u32>,
        format_options_use_int64_timestamp: Option<bool>,
    ) -> Result<GetQueryResultsResponse> {
        todo!("Implement getQueryResults endpoint")
    }

    /// Starts a new asynchronous job.
    pub async fn insert_job(&self, project_id: &str, job: Job) -> Result<Job> {
        todo!("Implement job insert endpoint")
    }

    /// Lists all jobs that you started in the specified project.
    pub async fn list_jobs(
        &self,
        project_id: &str,
        all_users: Option<bool>,
        max_creation_time: Option<&str>,
        max_results: Option<u32>,
        min_creation_time: Option<&str>,
        page_token: Option<&str>,
        parent_job_id: Option<&str>,
        projection: Option<&str>,
        state_filter: Option<Vec<&str>>,
    ) -> Result<JobList> {
        todo!("Implement job list endpoint")
    }

    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.
    pub async fn query(&self, project_id: &str, request: QueryRequest) -> Result<QueryResponse> {
        todo!("Implement job query endpoint")
    }
}
