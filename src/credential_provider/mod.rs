use ureq;

mod chained_credential_provider;
mod imds_credential_provider;

pub use chained_credential_provider::ChainedCredentialProvider;
pub use imds_credential_provider::ImdsCredentialProvider;

#[derive(Debug)]
pub enum CredentialError {
    NotFound,
    Other(String),
}

pub trait CredentialProvider: Send + Sync {
    fn fetch_credentials(&self) -> Result<String, CredentialError>;
}

pub struct DummyCredentialProvider;

impl CredentialProvider for DummyCredentialProvider {
    fn fetch_credentials(&self) -> Result<String, CredentialError> {
        Ok("hardcoded-credential".to_string())
    }
}
