//! # Configuration Manager Module
//!
//! This module provides the `ConfigManager` struct, responsible for loading,
//! saving, and managing application configurations. It handles multiple
//! configuration profiles and ensures a default configuration exists.

use crate::utils::config::app_config::{AppConfig, ConfigError};
use std::path::PathBuf;
use std::fs;

/// Manages the loading, saving, and selection of application configurations.
#[derive(Clone)]
pub struct ConfigManager {
    /// A list of all loaded application configurations.
    pub configs: Vec<AppConfig>,
    /// The index of the currently selected configuration in the `configs` vector.
    pub selected_config_index: usize,
}

impl ConfigManager {
    /// Determines the application's configuration directory.
    ///
    /// This typically resolves to a platform-specific configuration directory
    /// (e.g., `~/.config/arc-editor` on Linux, `C:\Users\<user>\AppData\Roaming\arc-editor` on Windows).
    ///
    /// # Returns
    ///
    /// * `Result<PathBuf, ConfigError>` - The path to the configuration directory on success,
    ///   or a `ConfigError::NoConfigDir` if the directory cannot be determined.
    fn config_dir() -> Result<PathBuf, ConfigError> {
        dirs::config_dir()
            .map(|mut path| {
                path.push("arc-editor");
                path
            })
            .ok_or(ConfigError::NoConfigDir)
    }

    /// Creates a new `ConfigManager` instance, loading existing configurations
    /// or creating a default one if none exist.
    ///
    /// This function ensures that a `profiles` subdirectory exists within the
    /// configuration directory and that at least a "default.toml" configuration
    /// is available.
    ///
    /// # Returns
    ///
    /// * `Result<Self, ConfigError>` - A new `ConfigManager` instance on success,
    ///   or a `ConfigError` if any I/O or parsing errors occur during loading.
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = Self::config_dir()?;
        let profiles_dir = config_dir.join("profiles");

        if !profiles_dir.exists() {
            fs::create_dir_all(&profiles_dir).map_err(ConfigError::IoError)?;
            let default_config = AppConfig::default();
            let default_path = profiles_dir.join("default.toml");
            default_config.save_to_path(&default_path)?;
        }

        let mut configs = Vec::new();
        for entry in fs::read_dir(&profiles_dir).map_err(ConfigError::IoError)? {
            let entry = entry.map_err(ConfigError::IoError)?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                let content = fs::read_to_string(&path).map_err(ConfigError::IoError)?;
                let config: AppConfig = toml::from_str(&content).map_err(ConfigError::ParseError)?;
                configs.push(config);
            }
        }

        if configs.is_empty() {
            let default_config = AppConfig::default();
            let default_path = profiles_dir.join("default.toml");
            default_config.save_to_path(&default_path)?;
            configs.push(default_config);
        }

        Ok(Self {
            configs,
            selected_config_index: 0,
        })
    }

    /// Saves the currently selected application configuration to its respective file.
    ///
    /// The configuration is saved to a file named after its `name` field (or "default.toml"
    /// if `name` is `None`) within the `profiles` subdirectory of the configuration directory.
    ///
    /// # Returns
    ///
    /// * `Result<(), ConfigError>` - `Ok(())` on successful save, or a `ConfigError` if
    ///   no configuration is selected or an I/O/serialization error occurs.
    pub fn save_current_config(&self) -> Result<(), ConfigError> {
        if let Some(config) = self.configs.get(self.selected_config_index) {
            let config_dir = Self::config_dir()?;
            let profiles_dir = config_dir.join("profiles");
            let config_name = config.name.clone().unwrap_or_else(|| "default".to_string());
            let config_path = profiles_dir.join(format!("{}.toml", config_name));
            config.save_to_path(&config_path)?;
            Ok(())
        } else {
            Err(ConfigError::LoadError("No config selected to save".to_string()))
        }
    }

    /// Adds a new `AppConfig` to the manager and sets it as the currently selected one.
    ///
    /// # Arguments
    ///
    /// * `config` - The `AppConfig` to add.
    pub fn add_config(&mut self, config: AppConfig) {
        self.configs.push(config);
        self.selected_config_index = self.configs.len() - 1;
    }
}
