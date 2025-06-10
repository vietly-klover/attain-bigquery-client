//! Integration test for FileConfigProvider.
//!
//! This test assumes you have already configured gcloud locally (i.e., you have run `gcloud init` or similar),
//! and that a valid config file exists at ~/.config/gcloud.
//!
//! The test checks that at least one config value is loaded from the file.

use attain_bigquery_client::config_provider::ConfigProvider;
use attain_bigquery_client::config_provider::FileConfigProvider;

#[test]
fn test_file_config_provider_parsing() {
    // This is a sync test for the internal parsing logic, not the async trait method.
    let provider = FileConfigProvider::load_default_config().expect("Failed to load config");
    let value = provider.get_config("core/project").unwrap_or_default();
    assert!(
        !value.is_empty(),
        "Expected at least one config value to be loaded from ~/.config/gcloud"
    );
}
