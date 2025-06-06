use serde::{Deserialize, Serialize};

/// Statistics for a single job execution.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics {
    /// Creation time of this job, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,

    /// Start time of this job, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// End time of this job, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Total bytes processed for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed: Option<String>,

    /// Information about the job, including query statistics, load statistics, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<super::job_statistics2::JobStatistics2>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_ratio: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<super::job_statistics5::JobStatistics5>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_masking_statistics: Option<super::data_masking_statistics::DataMaskingStatistics>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<super::job_statistics4::JobStatistics4>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_execution_duration_ms: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<super::job_statistics3::JobStatistics3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_child_jobs: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_deferments: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_usage: Option<Vec<super::job_statistics2::ReservationUsage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_security_statistics:
        Option<super::row_level_security_statistics::RowLevelSecurityStatistics>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_statistics: Option<super::script_statistics::ScriptStatistics>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_info: Option<super::session_info::SessionInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_slot_ms: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_info: Option<super::transaction_info::TransactionInfo>,
}
