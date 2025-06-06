use async_trait::async_trait;
use std::sync::Arc;

/// Provides an async trait for fetching credentials and a dummy implementation for testing purposes.

#[derive(Debug)]
pub enum CredentialError {
    NotFound,
    Other(String),
}

#[async_trait]
pub trait CredentialProvider: Send + Sync {
    async fn fetch_credentials(&self) -> Result<String, CredentialError>;
}

pub struct DummyCredentialProvider;

#[async_trait]
impl CredentialProvider for DummyCredentialProvider {
    async fn fetch_credentials(&self) -> Result<String, CredentialError> {
        Ok("hardcoded-credential".to_string())
    }
}

pub mod chained_credential_provider;
pub use chained_credential_provider::ChainedCredentialProvider;
