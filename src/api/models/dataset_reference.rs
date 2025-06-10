use serde::{Deserialize, Serialize};

/// Identifier for a dataset.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DatasetReference {
    /// Required. A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z),
    /// numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    pub dataset_id: String,

    /// Optional. The ID of the project containing this dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
