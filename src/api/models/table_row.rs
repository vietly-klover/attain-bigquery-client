use serde::{Deserialize, Serialize};

/// Represents a single row in the result set, consisting of one or more fields.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    /// The fields in the row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f: Option<Vec<TableCell>>,
}

/// Represents a single cell in a TableRow.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    /// The value of the cell. Can be any type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<serde_json::Value>,
}
