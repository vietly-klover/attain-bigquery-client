use super::ConfigProvider;
use std::sync::Arc;

pub struct ChainedConfigProvider {
    providers: Vec<Arc<dyn ConfigProvider>>,
}

impl ChainedConfigProvider {
    pub fn new(providers: Vec<Arc<dyn ConfigProvider>>) -> Self {
        Self { providers }
    }
}

impl ConfigProvider for ChainedConfigProvider {
    fn get_config(&self, key: &str) -> Option<String> {
        for provider in &self.providers {
            if let Some(value) = provider.get_config(key) {
                return Some(value);
            }
        }
        None
    }
}
