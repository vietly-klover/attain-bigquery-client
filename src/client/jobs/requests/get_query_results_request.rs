pub struct GetQueryResultsRequest<'a> {
    pub project_id: &'a str,
    pub job_id: &'a str,
    pub location: Option<&'a str>,
    pub max_results: Option<u32>,
    pub page_token: Option<&'a str>,
    pub start_index: Option<&'a str>,
    pub timeout_ms: Option<u32>,
    pub format_options_use_int64_timestamp: Option<bool>,
}
