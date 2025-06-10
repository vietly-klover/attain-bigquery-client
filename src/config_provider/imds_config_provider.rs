use super::ConfigProvider;
use std::time::Duration;
use tracing::error;
use ureq::{Agent, AgentBuilder};

pub struct ImdsConfigProvider {
    agent: Agent,
}

impl ImdsConfigProvider {
    pub fn new() -> Self {
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(1))
            .timeout_write(Duration::from_secs(1))
            .build();

        Self { agent }
    }
}

impl ConfigProvider for ImdsConfigProvider {
    fn get_config(&self, key: &str) -> Option<String> {
        // Transform the key to match Google Cloud metadata format
        // e.g. "project/project-id" -> "project/project-id"
        let metadata_path = format!("http://metadata.google.internal/computeMetadata/v1/{key}");

        match self
            .agent
            .get(&metadata_path)
            .set("Metadata-Flavor", "Google")
            .call()
            .inspect_err(|e| error!(message = "IMDS request failed", ?e))
        {
            Ok(response) => {
                if response.status() == 200 {
                    response
                        .into_string()
                        .inspect_err(|e| error!(message = "Failed to read IMDS response", ?e))
                        .ok()
                } else {
                    error!(
                        message = "IMDS returned non-200 status",
                        status = response.status()
                    );
                    None
                }
            }
            Err(e) => {
                error!(message = "IMDS request error", ?e);
                None
            }
        }
    }
}
