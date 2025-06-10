use crate::api::models::error_proto::ErrorProto;
use crate::api::models::job_reference::JobReference;
use crate::api::models::table_row::TableRow;
use crate::api::models::table_schema::TableSchema;
use serde::{Deserialize, Serialize};

/// Response from getQueryResults API.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryResultsResponse {
    /// Whether the query result was fetched from the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_hit: Option<bool>,

    /// [Output-only] All errors and warnings encountered during the running of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorProto>>,

    /// A hash of this response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Whether the query has completed or not. If rows are present, this will always be true.
    pub job_complete: bool,

    /// Reference to the BigQuery job that created this query result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,

    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_dml_affected_rows: Option<String>,

    /// A token used for paging results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// An object with as many results as can be contained within the maximum permitted reply size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<TableRow>>,

    /// The schema of the results. Present only when the query completes successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,

    /// The total number of bytes processed for this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed: Option<String>,

    /// The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows: Option<String>,
}
