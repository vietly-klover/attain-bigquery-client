use serde::{Deserialize, Serialize};

/// A connection-level property to customize query behavior.
///
/// Under JDBC, these correspond directly to connection properties passed to the DriverManager.
/// Under ODBC, these correspond to properties in the connection string.
///
/// Currently supported connection properties include:
/// - dataset_project_id
/// - time_zone
/// - session_id
/// - query_label
/// - service_account
///
/// Additional properties are allowed, but ignored. Specifying multiple connection properties with the same key returns an error.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionProperty {
    /// The key of the property to set.
    pub key: Option<String>,
    /// The value of the property to set.
    pub value: Option<String>,
}
