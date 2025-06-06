//! Integration test for FileConfigProvider.
//!
//! This test assumes you have already configured gcloud locally (i.e., you have run `gcloud init` or similar),
//! and that a valid config file exists at ~/.config/gcloud.
//!
//! The test checks that at least one config value is loaded from the file.

use attain_bigquery_client::config_provider::FileConfigProvider;

#[test]
fn test_file_config_provider_parsing() {
    // This is a sync test for the internal parsing logic, not the async trait method.
    let config = FileConfigProvider::load_config(None).expect("Failed to load config");
    assert!(
        config.get_map().map(|m| !m.is_empty()).unwrap_or(false),
        "Expected at least one config value to be loaded from ~/.config/gcloud"
    );
}
