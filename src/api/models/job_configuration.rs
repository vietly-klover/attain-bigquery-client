use serde::{Deserialize, Serialize};

/// Describes the job configuration for a BigQuery Job.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobConfiguration {
    /// [Pick one] Configures a copy job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<super::job_configuration_table_copy::JobConfigurationTableCopy>,

    /// Optional. If set, don't actually run this job. A valid query will return a mostly empty response with some
    /// processing statistics, while an invalid query will return the same error it would if it wasn't a dry run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// [Pick one] Configures an extract job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<super::job_configuration_extract::JobConfigurationExtract>,

    /// Optional. Job timeout in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_timeout_ms: Option<String>,

    /// Output only. The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,

    /// The labels associated with this job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    /// [Pick one] Configures a load job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<super::job_configuration_load::JobConfigurationLoad>,

    /// [Pick one] Configures a query job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<super::job_configuration_query::JobConfigurationQuery>,

    /// Optional. The reservation that job would use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<String>,
}
