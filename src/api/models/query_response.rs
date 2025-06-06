use crate::api::models::dml_statistics::DmlStatistics;
use crate::api::models::error_proto::ErrorProto;
use crate::api::models::job_creation_reason::JobCreationReason;
use crate::api::models::job_reference::JobReference;
use crate::api::models::session_info::SessionInfo;
use crate::api::models::table_row::TableRow;
use crate::api::models::table_schema::TableSchema;
use serde::{Deserialize, Serialize};

/// Response from running a query synchronously.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    pub cache_hit: Option<bool>,
    pub creation_time: Option<String>,
    pub dml_stats: Option<DmlStatistics>,
    pub end_time: Option<String>,
    pub errors: Option<Vec<ErrorProto>>,
    pub job_complete: Option<bool>,
    pub job_creation_reason: Option<JobCreationReason>,
    pub job_reference: Option<JobReference>,
    pub kind: Option<String>,
    pub location: Option<String>,
    pub num_dml_affected_rows: Option<String>,
    pub page_token: Option<String>,
    pub query_id: Option<String>,
    pub rows: Option<Vec<TableRow>>,
    pub schema: Option<TableSchema>,
    pub session_info: Option<SessionInfo>,
    pub start_time: Option<String>,
    pub total_bytes_billed: Option<String>,
    pub total_bytes_processed: Option<String>,
    pub total_rows: Option<String>,
    pub total_slot_ms: Option<String>,
}
