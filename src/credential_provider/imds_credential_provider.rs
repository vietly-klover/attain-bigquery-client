use serde::Deserialize;
use serde_json;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use ureq;

use super::{CredentialError, CredentialProvider};

/// Credential provider that fetches an access token from the GCE metadata server.
/// See: https://cloud.google.com/compute/docs/access/authenticate-workloads#authenticating-applications-directly-with-access-tokens
#[derive(Deserialize)]
struct ImdsTokenResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
}

pub struct ImdsCredentialProvider {
    cache: Mutex<Option<CachedToken>>,
}

struct CachedToken {
    token: String,
    expires_at: Instant,
}

impl Default for ImdsCredentialProvider {
    fn default() -> Self {
        Self {
            cache: Mutex::new(None),
        }
    }
}

impl CredentialProvider for ImdsCredentialProvider {
    fn fetch_credentials(&self) -> Result<String, CredentialError> {
        {
            let cache_guard = self.cache.lock().unwrap();
            if let Some(cached) = &*cache_guard {
                let now = Instant::now();
                if now < cached.expires_at {
                    return Ok(cached.token.clone());
                }
            }
        }
        // Token is missing, expired, or more than halfway expired, fetch a new one
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

        let token_response: ImdsTokenResponse = serde_json::from_str(&body)
            .map_err(|e| CredentialError::Other(format!("Failed to parse JSON: {}", e)))?;

        let expires_at = Instant::now() + Duration::from_secs(token_response.expires_in as u64 / 2);

        let mut cache_guard = self.cache.lock().unwrap();

        *cache_guard = Some(CachedToken {
            token: token_response.access_token.clone(),
            expires_at,
        });

        Ok(token_response.access_token)
    }
}
