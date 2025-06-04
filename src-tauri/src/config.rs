use crate::log_function;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use std::sync::Mutex;

const CONFIG_DIR: &str = ".datasquirrel";

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigType {
    Config,
    Connections,
}

pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> io::Result<Self> {
        log_function!(new);
        let home_dir = dirs::home_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find home directory"))?;
        let config_dir = home_dir.join(CONFIG_DIR);

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        Ok(Self { config_dir })
    }

    pub fn get_config_path(&self, config_type: &ConfigType) -> PathBuf {
        log_function!(get_config_path, "config_type" => config_type);
        let filename = match config_type {
            ConfigType::Config => "config.json",
            ConfigType::Connections => "connections.json",
        };
        self.config_dir.join(filename)
    }

    pub fn read_config<T: for<'de> Deserialize<'de>>(
        &self,
        config_type: &ConfigType,
    ) -> io::Result<T> {
        log_function!(read_config, "config_type" => config_type);
        let config_path = self.get_config_path(config_type);

        if !config_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Config file not found: {:?}", config_path),
            ));
        }

        let contents = fs::read_to_string(config_path)?;
        serde_json::from_str(&contents).map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse config file: {}", e),
            )
        })
    }

    pub fn write_config<T: Serialize>(&self, config_type: &ConfigType, data: &T) -> io::Result<()> {
        log_function!(write_config, "config_type" => config_type);
        let config_path = self.get_config_path(config_type);
        let json = serde_json::to_string_pretty(data)?;
        fs::write(config_path, json)
    }

    pub fn ensure_config_exists<T: Serialize + Default>(
        &self,
        config_type: &ConfigType,
    ) -> io::Result<()> {
        log_function!(ensure_config_exists, "config_type" => config_type);
        let config_path = self.get_config_path(config_type);

        if !config_path.exists() {
            let default_config = T::default();
            self.write_config(config_type, &default_config)?;
        }

        Ok(())
    }
}

static CONFIG_MANAGER: Lazy<Mutex<ConfigManager>> =
    Lazy::new(|| Mutex::new(ConfigManager::new().expect("Failed to initialize ConfigManager")));

pub fn get_config_manager() -> &'static Mutex<ConfigManager> {
    &CONFIG_MANAGER
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_config_operations() {
        let config_manager = ConfigManager::new().unwrap();

        // Test writing and reading config
        let test_data = json!({
            "test_key": "test_value"
        });

        config_manager
            .write_config(&ConfigType::Config, &test_data)
            .unwrap();
        let read_data: serde_json::Value = config_manager.read_config(&ConfigType::Config).unwrap();

        assert_eq!(read_data, test_data);
    }
}
