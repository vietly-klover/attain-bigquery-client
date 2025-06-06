use crate::api::models::error_proto::ErrorProto;
use crate::api::models::job_reference::JobReference;
use crate::api::models::table_row::TableRow;
use crate::api::models::table_schema::TableSchema;
use serde::{Deserialize, Serialize};

/// Response from getQueryResults API.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryResultsResponse {
    pub cache_hit: Option<bool>,
    pub errors: Option<Vec<ErrorProto>>,
    pub etag: Option<String>,
    pub job_complete: Option<bool>,
    pub job_reference: Option<JobReference>,
    pub kind: Option<String>,
    pub num_dml_affected_rows: Option<String>,
    pub page_token: Option<String>,
    pub rows: Option<Vec<TableRow>>,
    pub schema: Option<TableSchema>,
    pub total_bytes_processed: Option<String>,
    pub total_rows: Option<String>,
}
