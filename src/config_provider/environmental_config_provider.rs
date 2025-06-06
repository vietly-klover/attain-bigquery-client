use super::ConfigProvider;
use std::env;

pub struct EnvironmentalConfigProvider;

impl ConfigProvider for EnvironmentalConfigProvider {
    fn get_config(&self, key: &str) -> Option<String> {
        // Transform 'core/project' into 'CLOUDSDK_CORE_PROJECT'
        let env_key = format!("CLOUDSDK_{}", key.replace('/', "_").to_uppercase());
        env::var(env_key).ok()
    }
}
