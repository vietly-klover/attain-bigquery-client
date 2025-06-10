pub mod datasets;
pub mod jobs;
pub mod models;
pub mod projects;
pub mod routines;
pub mod row_access_policies;
pub mod tabledata;
pub mod tables;

use crate::config_provider::ConfigProvider;
use crate::credential_provider::CredentialProvider;
use std::sync::Arc;

pub struct Client {
    credential_provider: Arc<dyn CredentialProvider>,
    config_provider: Arc<dyn ConfigProvider>,
}

impl Client {
    pub fn new(
        credential_provider: Arc<dyn CredentialProvider>,
        config_provider: Arc<dyn ConfigProvider>,
    ) -> Self {
        Self {
            credential_provider,
            config_provider,
        }
    }
    pub fn datasets(&self) -> Datasets {
        Datasets::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
    pub fn jobs(&self) -> Jobs {
        Jobs::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
    pub fn models(&self) -> Models {
        Models::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
    pub fn projects(&self) -> Projects {
        Projects::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
    pub fn routines(&self) -> Routines {
        Routines::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
    pub fn row_access_policies(&self) -> RowAccessPolicies {
        RowAccessPolicies::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
    pub fn tabledata(&self) -> Tabledata {
        Tabledata::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
    pub fn tables(&self) -> Tables {
        Tables::new(
            Arc::clone(&self.credential_provider),
            Arc::clone(&self.config_provider),
        )
    }
}

use datasets::Datasets;
use jobs::Jobs;
use models::Models;
use projects::Projects;
use routines::Routines;
use row_access_policies::RowAccessPolicies;
use tabledata::Tabledata;
use tables::Tables;
