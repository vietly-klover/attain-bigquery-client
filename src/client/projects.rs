use crate::config_provider::ConfigProvider;
use crate::credential_provider::CredentialProvider;
use std::sync::Arc;

pub struct Projects {
    credential_provider: Arc<dyn CredentialProvider>,
    config_provider: Arc<dyn ConfigProvider>,
}

impl Projects {
    pub fn new(
        credential_provider: Arc<dyn CredentialProvider>,
        config_provider: Arc<dyn ConfigProvider>,
    ) -> Self {
        Self {
            credential_provider,
            config_provider,
        }
    }
}
