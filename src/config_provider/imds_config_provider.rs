use super::ConfigProvider;
use anyhow::Result;
use std::time::Duration;
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
        
        match self.agent
            .get(&metadata_path)
            .set("Metadata-Flavor", "Google")
            .call() {
                Ok(response) => {
                    if response.status() == 200 {
                        response.into_string().ok()
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
    }
}
