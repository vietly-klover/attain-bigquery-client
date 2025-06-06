use serde::{Deserialize, Serialize};

/// JobConfigurationTableCopy configures a job that copies data from one table to another.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationTableCopy {
    /// [Required] The destination table.
    pub destination_table: super::table_reference::TableReference,

    /// [Pick one] Source table to copy.
    pub source_table: Option<super::table_reference::TableReference>,

    /// [Pick one] Source tables to copy.
    pub source_tables: Option<Vec<super::table_reference::TableReference>>,

    /// Optional. Specifies whether the job is allowed to create new tables.
    pub create_disposition: Option<String>,

    /// Optional. Specifies the action that occurs if the destination table already exists.
    pub write_disposition: Option<String>,

    /// Optional. Supported operation types in table copy job.
    pub operation_type: Option<String>,
    // Add more fields as needed
}
