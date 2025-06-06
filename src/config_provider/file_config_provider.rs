use super::ConfigProvider;
use anyhow;
use configparser::ini::Ini;
use dirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct FileConfigProvider {
    config: Mutex<Ini>,
}

impl FileConfigProvider {
    pub fn new() -> anyhow::Result<Self> {
        let config = Self::load_config(None)?;
        Ok(Self {
            config: Mutex::new(config),
        })
    }

    fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".config/gcloud/configurations")
    }

    pub fn load_config(profile: Option<&str>) -> anyhow::Result<Ini> {
        let profile_name = profile.unwrap_or("default");
        let path = Self::config_path().join(format!("config_{profile_name}"));
        println!("path: {}", path.display());

        let contents = fs::read_to_string(&path)?;

        let mut ini = Ini::new();
        ini.read(contents)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {e}"))?;
        Ok(ini)
    }

    pub fn load_default_config() -> anyhow::Result<Ini> {
        Self::load_config(None)
    }
}

impl ConfigProvider for FileConfigProvider {
    fn get_config(&self, key: &str) -> Option<String> {
        let config = self.config.lock().unwrap();
        // Try to find the key in the default section first
        if let Some(val) = config.get("default", key) {
            return Some(val);
        }
        // If not found, try to find the key in any section
        for (section, props) in config.get_map().unwrap_or_default().iter() {
            if let Some(val) = props.get(key) {
                if let Some(v) = val {
                    return Some(v.clone());
                }
            }
        }
        None
    }
}
