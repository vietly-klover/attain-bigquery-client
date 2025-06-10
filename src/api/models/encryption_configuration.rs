use serde::{Deserialize, Serialize};

/// Configuration for Cloud KMS encryption settings.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionConfiguration {
    /// Optional. Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The
    /// BigQuery Service Account associated with your project requires access to this encryption key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_name: Option<String>,
}
