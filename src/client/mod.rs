pub mod datasets;
pub mod jobs;
pub mod models;
pub mod projects;
pub mod routines;
pub mod row_access_policies;
pub mod tabledata;
pub mod tables;

pub struct Client;

impl Client {
    pub fn datasets(&self) -> Datasets {
        Datasets
    }
    pub fn jobs(&self) -> Jobs {
        Jobs
    }
    pub fn models(&self) -> Models {
        Models
    }
    pub fn projects(&self) -> Projects {
        Projects
    }
    pub fn routines(&self) -> Routines {
        Routines
    }
    pub fn row_access_policies(&self) -> RowAccessPolicies {
        RowAccessPolicies
    }
    pub fn tabledata(&self) -> Tabledata {
        Tabledata
    }
    pub fn tables(&self) -> Tables {
        Tables
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
