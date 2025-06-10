use serde::{Deserialize, Serialize};

/// A table reference is a fully qualified identifier for referring to a table.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableReference {
    /// Required. The ID of the dataset containing this table.
    pub dataset_id: String,

    /// Required. The ID of the project containing this table.
    pub project_id: String,

    /// Required. The ID of the table. The ID can contain Unicode characters in category L (letter), M (mark), N
    /// (number), Pc (connector, including underscore), Pd (dash), and Zs (space).
    pub table_id: String,
}
