use serde::{Deserialize, Serialize};

/// This is used for defining User Defined Function (UDF) resources only when using legacy SQL.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedFunctionResource {
    /// [Pick one] An inline resource that contains code for a user-defined function (UDF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_code: Option<String>,

    /// [Pick one] A code resource to load from a Google Cloud Storage URI (gs://bucket/path).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
