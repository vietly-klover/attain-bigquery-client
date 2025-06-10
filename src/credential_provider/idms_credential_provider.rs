use serde_json;
use ureq;

use super::{CredentialError, CredentialProvider};

/// Credential provider that fetches an access token from the GCE metadata server.
/// See: https://cloud.google.com/compute/docs/access/authenticate-workloads#authenticating-applications-directly-with-access-tokens
pub struct IdmsCredentialProvider;

impl CredentialProvider for IdmsCredentialProvider {
    fn fetch_credentials(&self) -> Result<String, CredentialError> {
        let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
        let resp = ureq::get(url)
            .set("Metadata-Flavor", "Google")
            .call()
            .map_err(|e| CredentialError::Other(format!("HTTP error: {}", e)))?;
        if resp.status() != 200 {
            return Err(CredentialError::Other(format!(
                "Metadata server returned status {}",
                resp.status()
            )));
        }
        let body = resp
            .into_string()
            .map_err(|e| CredentialError::Other(format!("Failed to read response body: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| CredentialError::Other(format!("Failed to parse JSON: {}", e)))?;
        let access_token = json
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| CredentialError::Other("No access_token in response".to_string()))?;
        Ok(access_token.to_string())
    }
}
