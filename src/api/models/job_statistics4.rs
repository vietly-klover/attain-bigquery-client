use serde::{Deserialize, Serialize};

/// Statistics for an extract job.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics4 {
    /// Output only. Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the 'destinationUris' field.
    pub destination_uri_file_counts: Vec<String>,

    /// Output only. Number of user bytes extracted into the result. This is the byte count as computed by BigQuery for billing purposes and doesn't have any relationship with the number of actual result bytes extracted in the desired format.
    pub input_bytes: String,

    /// Output only. Describes a timeline of job execution.
    pub timeline: Vec<QueryTimelineSample>,
}

/// Summary of the state of query execution at a given time.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTimelineSample {
    /// Total number of active workers. This does not correspond directly to slot usage. This is the largest value observed since the last sample.
    pub active_units: String,

    /// Total parallel units of work completed by this query.
    pub completed_units: String,

    /// Milliseconds elapsed since the start of query execution.
    pub elapsed_ms: String,

    /// Units of work that can be scheduled immediately. Providing additional slots for these units of work will accelerate the query, if no other query in the reservation needs additional slots.
    pub estimated_runnable_units: String,

    /// Total units of work remaining for the query. This number can be revised (increased or decreased) while the query is running.
    pub pending_units: String,

    /// Total shuffle usage ratio in shuffle RAM per reservation of this query. This will be provided for reservation customers only.
    pub shuffle_ram_usage_ratio: f64,

    /// Cumulative slot-ms consumed by the query.
    pub total_slot_ms: String,
}
