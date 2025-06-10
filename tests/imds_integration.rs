use attain_bigquery_client::config_provider::ConfigProvider;
use attain_bigquery_client::config_provider::imds_config_provider::ImdsConfigProvider;
use attain_bigquery_client::credential_provider::CredentialProvider;
use attain_bigquery_client::credential_provider::ImdsCredentialProvider;

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

#[test]
#[ignore]
fn test_imds_credential_provider() {
    let provider = ImdsCredentialProvider::default();
    let result = provider.fetch_credentials();
    // This test is ignored by default because it only works on GCE.
    assert!(
        result.is_ok()
            || matches!(
                result,
                Err(attain_bigquery_client::credential_provider::CredentialError::Other(_))
            )
    );
}
