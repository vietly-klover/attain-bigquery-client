use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics3 {
    /// Output only. The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data.
    pub bad_records: String,

    /// Output only. Number of bytes of source data in a load job.
    pub input_file_bytes: String,

    /// Output only. Number of source files in a load job.
    pub input_files: String,

    /// Output only. Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change.
    pub output_bytes: String,

    /// Output only. Number of rows imported in a load job. Note that while an import job is in the running state, this value may change.
    pub output_rows: String,

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
