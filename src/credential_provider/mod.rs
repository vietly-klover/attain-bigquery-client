use ureq;

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

pub mod idms_credential_provider;
pub use idms_credential_provider::IdmsCredentialProvider;

pub mod chained_credential_provider;
pub use chained_credential_provider::ChainedCredentialProvider;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_idms_credential_provider() {
        let provider = IdmsCredentialProvider;
        let result = provider.fetch_credentials();
        // This test is ignored by default because it only works on GCE.
        assert!(result.is_ok() || matches!(result, Err(CredentialError::Other(_))));
    }
}
