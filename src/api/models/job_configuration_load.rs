use serde::{Deserialize, Serialize};

/// JobConfigurationLoad contains the configuration properties for loading data into a destination table.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationLoad {
    /// [Required] The destination table to load the data into.
    pub destination_table: super::table_reference::TableReference,

    /// [Required] The fully-qualified URIs that point to your data in Google Cloud.
    pub source_uris: Vec<String>,

    /// Optional. The format of the data files.
    pub source_format: Option<String>,

    /// Optional. The schema for the destination table.
    pub schema: Option<super::table_schema::TableSchema>,

    /// Optional. Specifies whether the job is allowed to create new tables.
    pub create_disposition: Option<String>,

    /// Optional. Specifies the action that occurs if the destination table already exists.
    pub write_disposition: Option<String>,

    /// Optional. The character encoding of the data.
    pub encoding: Option<String>,

    /// Optional. The separator for fields in a CSV file.
    pub field_delimiter: Option<String>,

    /// Optional. The number of rows at the top of a CSV file that BigQuery will skip when loading the data.
    pub skip_leading_rows: Option<i32>,
    // Add more fields as needed
}
