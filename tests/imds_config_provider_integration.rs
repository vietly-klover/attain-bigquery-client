use attain_bigquery_client::config_provider::ConfigProvider;
use attain_bigquery_client::config_provider::imds_config_provider::ImdsConfigProvider;

#[test]
#[ignore]
fn test_grab_default_project_id() {
    // This test will only work in a GCP environment with metadata server access
    // It is ignored by default
    let provider = ImdsConfigProvider::new();
    let project_id = provider.get_config("project/project-id");

    assert!(
        project_id.is_some(),
        "Should be able to fetch project id from metadata server"
    );
    let project_id = project_id.unwrap();

    assert!(!project_id.is_empty(), "Project id should not be empty");
}
