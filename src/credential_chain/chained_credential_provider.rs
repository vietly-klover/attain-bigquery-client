use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use super::{CredentialError, CredentialProvider};

pub struct ChainedCredentialProvider {
    providers: Vec<Arc<dyn CredentialProvider>>,
    last_successful: Mutex<Option<Arc<dyn CredentialProvider>>>,
}

impl ChainedCredentialProvider {
    pub fn new(providers: Vec<Arc<dyn CredentialProvider>>) -> Self {
        Self {
            providers,
            last_successful: Mutex::new(None),
        }
    }

    pub fn last_successful_provider(&self) -> Option<Arc<dyn CredentialProvider>> {
        self.last_successful
            .lock()
            .unwrap()
            .as_ref()
            .map(Arc::clone)
    }
}

#[async_trait]
impl CredentialProvider for ChainedCredentialProvider {
    async fn fetch_credentials(&self) -> Result<String, CredentialError> {
        for provider in self.providers.iter() {
            match provider.fetch_credentials().await {
                Ok(cred) => {
                    let mut last = self.last_successful.lock().unwrap();
                    *last = Some(Arc::clone(provider));
                    return Ok(cred);
                }
                Err(CredentialError::NotFound) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(CredentialError::NotFound)
    }
}
