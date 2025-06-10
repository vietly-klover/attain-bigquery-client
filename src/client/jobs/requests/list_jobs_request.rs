pub struct ListJobsRequest<'a> {
    pub project_id: &'a str,
    pub all_users: Option<bool>,
    pub max_creation_time: Option<&'a str>,
    pub max_results: Option<u32>,
    pub min_creation_time: Option<&'a str>,
    pub page_token: Option<&'a str>,
    pub parent_job_id: Option<&'a str>,
    pub projection: Option<&'a str>,
    pub state_filter: Option<Vec<&'a str>>,
}
