use crate::api::models::connection_property::ConnectionProperty;
use crate::api::models::data_format_options::DataFormatOptions;
use crate::api::models::dataset_reference::DatasetReference;
use crate::api::models::encryption_configuration::EncryptionConfiguration;
use crate::api::models::query_parameter::QueryParameter;
use serde::{Deserialize, Serialize};

/// Request for running a query synchronously.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    pub connection_properties: Option<Vec<ConnectionProperty>>,
    pub continuous: Option<bool>,
    pub create_session: Option<bool>,
    pub default_dataset: Option<DatasetReference>,
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    pub dry_run: Option<bool>,
    pub format_options: Option<DataFormatOptions>,
    pub job_creation_mode: Option<String>,
    pub job_timeout_ms: Option<String>,
    pub kind: Option<String>,
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub location: Option<String>,
    pub max_results: Option<i32>,
    pub maximum_bytes_billed: Option<String>,
    pub parameter_mode: Option<String>,
    pub preserve_nulls: Option<bool>,
    pub query: Option<String>,
    pub query_parameters: Option<Vec<QueryParameter>>,
    pub request_id: Option<String>,
    pub reservation: Option<String>,
    pub timeout_ms: Option<i32>,
    pub use_legacy_sql: Option<bool>,
    pub use_query_cache: Option<bool>,
    pub write_incremental_results: Option<bool>,
}
