use serde::{Deserialize, Serialize};

/// JobConfigurationExtract configures a job that exports data from a BigQuery table into Google Cloud Storage.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationExtract {
    /// [Pick one] A reference to the table being exported.
    pub source_table: Option<super::table_reference::TableReference>,

    /// [Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    pub destination_uris: Option<Vec<String>>,

    /// Optional. The exported file format.
    pub destination_format: Option<String>,

    /// Optional. Whether to print out a header row in the results. Default is true.
    pub print_header: Option<bool>,
    // Add more fields as needed
}
