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
    /// Optional. Connection properties which can modify the query behavior.
    pub connection_properties: Option<Vec<ConnectionProperty>>,

    /// [Optional] Specifies whether the query should be executed as a continuous query. The default value is false.
    pub continuous: Option<bool>,

    /// Optional. If true, creates a new session using a randomly generated session_id. If false, runs query with an existing session_id passed in ConnectionProperty, otherwise runs query in non-session mode. The session location will be set to QueryRequest.location if it is present, otherwise it's set to the default location based on existing routing logic.
    pub create_session: Option<bool>,

    /// Optional. Specifies the default datasetId and projectId to assume for any unqualified table names in the query. If not set, all table names in the query string must be qualified in the format 'datasetId.tableId'.
    pub default_dataset: Option<DatasetReference>,

    /// Optional. Custom encryption configuration (e.g., Cloud KMS keys)
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,

    /// Optional. If set to true, BigQuery doesn't run the job. Instead, if the query is valid, BigQuery returns statistics about the job such as how many bytes would be processed. If the query is invalid, an error returns. The default value is false.
    pub dry_run: Option<bool>,

    /// Optional. Output format adjustments.
    pub format_options: Option<DataFormatOptions>,

    /// Optional. If not set, jobs are always required. If set, the query request will follow the behavior described JobCreationMode. [Preview](https://cloud.google.com/products/#product-launch-stages)
    pub job_creation_mode: Option<String>,

    /// Optional. Job timeout in milliseconds. If this time limit is exceeded, BigQuery will attempt to stop a longer job, but may not always succeed in canceling it before the job completes. For example, a job that takes more than 60 seconds to complete has a better chance of being stopped than a job that takes 10 seconds to complete. This timeout applies to the query even if a job does not need to be created.
    pub job_timeout_ms: Option<String>,

    /// The resource type of the request.
    pub kind: Option<String>,

    /// Optional. The labels associated with this query. Labels can be used to organize and group query jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label keys must start with a letter and each label in the list must have a different key.
    pub labels: Option<std::collections::HashMap<String, String>>,

    /// The geographic location where the job should run. For more information, see how to specify locations: https://cloud.google.com/bigquery/docs/locations#specify_locations
    pub location: Option<String>,

    /// Optional. The maximum number of rows of data to return per page of results. Setting this flag to a small value such as 1000 and then paging through results might improve reliability when the query result set is large. In addition to this limit, responses are also limited to 10 MB. By default, there is no maximum row count, and only the byte limit applies.
    pub max_results: Option<i32>,

    /// Optional. Limits the bytes billed for this query. Queries with bytes billed above this limit will fail (without incurring a charge). If unspecified, the project default is used.
    pub maximum_bytes_billed: Option<String>,

    /// GoogleSQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    pub parameter_mode: Option<String>,

    /// This property is deprecated.
    pub preserve_nulls: Option<bool>,

    /// Required. A query string to execute, using Google Standard SQL or legacy SQL syntax. Example: "SELECT COUNT(f1) FROM myProjectId.myDatasetId.myTableId".
    pub query: Option<String>,

    /// Query parameters for GoogleSQL queries.
    pub query_parameters: Option<Vec<QueryParameter>>,

    /// Optional. A unique user provided identifier to ensure idempotent behavior for queries. Note that this is different from the job_id. It is case-sensitive, limited to up to 36 ASCII characters. A UUID is recommended. For the purposes of idempotency ensured by the request_id, a request is considered duplicate of another only if they have the same request_id and are actually duplicates. When determining whether a request is a duplicate of another request, all parameters in the request that may affect the result are considered. For example, query, connection_properties, query_parameters, use_legacy_sql are parameters that affect the result and are considered when determining whether a request is a duplicate, but properties like timeout_ms don't affect the result and are thus not considered. Dry run query requests are never considered duplicate of another request. When a duplicate mutating query request is detected, it returns: a. the results of the mutation if it completes successfully within the timeout. b. the running operation if it is still in progress at the end of the timeout. Its lifetime is limited to 15 minutes. In other words, if two requests are sent with the same request_id, but more than 15 minutes apart, idempotency is not guaranteed.
    pub request_id: Option<String>,

    /// Optional. The reservation that jobs.query request would use. User can specify a reservation to execute the job.query. The expected format is `projects/{project}/locations/{location}/reservations/{reservation}`.
    pub reservation: Option<String>,

    /// Optional. Specifies the maximum amount of time, in milliseconds, that the client is willing to wait for the query to complete. By default, this limit is 10 seconds (10,000 milliseconds). If the query is complete, the jobComplete field in the response is true. If the query has not yet completed, jobComplete is false. You can request a longer timeout period in the timeoutMs field. However, the call is not guaranteed to wait for the specified timeout; it typically returns after around 200 seconds (200,000 milliseconds), even if the query is not complete. If jobComplete is false, you can continue to wait for the query to complete by calling the getQueryResults method until the jobComplete field in the getQueryResults response is true.
    pub timeout_ms: Option<i32>,

    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's GoogleSQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    pub use_legacy_sql: Option<bool>,

    /// Optional. Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. The default value is true.
    pub use_query_cache: Option<bool>,

    /// Optional. This is only supported for SELECT query. If set, the query is allowed to write results incrementally to the temporary result table. This may incur a performance penalty. This option cannot be used with Legacy SQL. This feature is not yet available.
    pub write_incremental_results: Option<bool>,
}
