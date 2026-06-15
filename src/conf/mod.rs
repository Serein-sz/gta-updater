use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub github_owner: String,
    pub global_path: String,
    pub apps: Vec<App>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub name: String,
    pub alias: Option<String>,
    pub version: String,
    pub path: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from_dir(config_dir())
    }

    pub fn load_from_dir(config_dir: impl AsRef<Path>) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::from(config_dir.as_ref().join("config.toml")))
            .add_source(
                Environment::with_prefix("GTA")
                    .separator("__")
                    .prefix_separator("__"),
            )
            .build()?
            .try_deserialize()
    }

    pub fn rewrite(&self) -> anyhow::Result<()> {
        let toml = toml::to_string_pretty(self)?;
        
        std::fs::write(config_dir().join("config.toml"), toml)?;
        Ok(())
    }
}

pub fn config_dir() -> PathBuf {
    if let Some(path) = std::env::var_os("GTA_CONFIG_DIR") {
        return PathBuf::from(path);
    }

    #[cfg(any(target_os = "macos", windows))]
    if let Some(path) = dirs::home_dir() {
        return path.join(".config").join("gta-updater");
    }

    if let Some(path) = dirs::config_dir() {
        return path.join("gta-updater");
    }

    PathBuf::from("config")
}

#[cfg(test)]
mod tests {
    use super::App;
    use super::AppConfig;

    #[test]
    fn loads_default_config_file() {
        let config = AppConfig {
            github_owner: "Serein-sz".to_string(),
            global_path: "D:\\code_sorfwares\\bin".to_string(),
            apps: vec![App {
                name: "body-recorder".to_string(),
                alias: None,
                version: "v0.1.0".to_string(),
                path: None,
            }],
        };
        assert!(!config.apps.is_empty());
        assert!(!config.global_path.is_empty());
    }
}
