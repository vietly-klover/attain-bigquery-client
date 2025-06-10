use serde::{Deserialize, Serialize};

/// JobConfigurationQuery configures a BigQuery query job.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationQuery {
    /// [Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses
    /// legacy SQL or GoogleSQL.
    pub query: String,

    /// Optional. Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,

    /// Optional. Specifies whether the job is allowed to create new tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,

    /// Optional. Specifies the action that occurs if the destination table already exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,

    /// Optional. Describes the table where the query results should be stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table: Option<super::table_reference::TableReference>,

    /// Optional. If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables
    /// at a slight cost in performance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_large_results: Option<bool>,

    /// Optional. If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query
    /// results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten_results: Option<bool>,

    /// Optional. Maximum bytes billed for this job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bytes_billed: Option<String>,

    /// Optional. Whether to look for the result in the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_query_cache: Option<bool>,

    /// Optional. User-defined function resources used in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_function_resources:
        Option<Vec<super::user_defined_function_resource::UserDefinedFunctionResource>>,

    /// Specifies whether the job is a dry run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    // Add more fields as needed
}
