use super::ConfigProvider;
use configparser::ini::Ini;
use dirs;
use std::fs;
use std::path::PathBuf;
use tracing::error;

pub struct FileConfigProvider {
    config: Ini,
}

impl FileConfigProvider {
    fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".config/gcloud/configurations")
    }

    pub fn load_config(profile: Option<&str>) -> anyhow::Result<Self> {
        let profile_name = profile.unwrap_or("default");
        let path = Self::config_path().join(format!("config_{profile_name}"));

        let contents = fs::read_to_string(&path)
            .inspect_err(|e| error!(message = "Failed to read config file", ?path, ?e))?;

        let mut ini = Ini::new();
        ini.read(contents).map_err(|e| {
            error!("Failed to parse config file: {e}");
            anyhow::anyhow!("Failed to read config file: {e}")
        })?;

        Ok(Self { config: ini })
    }

    pub fn load_default_config() -> anyhow::Result<Self> {
        Self::load_config(None)
    }
}

impl ConfigProvider for FileConfigProvider {
    fn get_config(&self, key: &str) -> Option<String> {
        let (section, key) = key.split_once('/').unwrap_or(("default", key));

        self.config.get(section, key)
    }
}
