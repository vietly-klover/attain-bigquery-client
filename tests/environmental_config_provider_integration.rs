//! Integration test for EnvironmentalConfigProvider.
//!
//! This test sets the CLOUDSDK_CORE_PROJECT environment variable to a hardcoded value
//! and verifies that EnvironmentalConfigProvider reads it correctly.

use attain_bigquery_client::config_provider::ConfigProvider;
use attain_bigquery_client::config_provider::environmental_config_provider::EnvironmentalConfigProvider;

#[test]
fn test_environmental_config_provider_reads_env_var() {
    // Set the environment variable
    unsafe {
        std::env::set_var("CLOUDSDK_CORE_PROJECT", "test-project-123");
    }

    let provider = EnvironmentalConfigProvider;
    let value = provider.get_config("core/project");
    assert_eq!(value.as_deref(), Some("test-project-123"));
}
