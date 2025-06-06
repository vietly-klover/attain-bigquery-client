// use async_trait::async_trait;

/// Provides a trait for fetching configuration values.
pub trait ConfigProvider: Send + Sync {
    /// Fetch a configuration value by key. Returns None if not found.
    fn get_config(&self, key: &str) -> Option<String>;
}

pub mod chained_config_provider;
pub use chained_config_provider::ChainedConfigProvider;

pub mod environmental_config_provider;
pub mod file_config_provider;
pub mod imds_config_provider;

pub use environmental_config_provider::EnvironmentalConfigProvider;
pub use file_config_provider::FileConfigProvider;
pub use imds_config_provider::ImdsConfigProvider;
