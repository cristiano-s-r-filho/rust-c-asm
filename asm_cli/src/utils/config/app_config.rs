//! # Application Configuration Module
//!
//! This module defines the data structures for managing the application's
//! configuration, including UI settings, workspace preferences, and keybindings.
//! It also provides functionality for loading and saving these configurations.

use serde::{Deserialize, Serialize};
use std::path::{PathBuf, Path};
use std::env::{
    current_dir,
};

/// Represents the overall application configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    /// The name of the configuration profile.
    pub name: Option<String>,
    /// The default workspace directory.
    pub default_workspace: PathBuf,
    /// A list of file extensions recognized as ARC assembly files.
    pub arc_extensions: Vec<String>,
    /// Whether hidden files should be shown in the file explorer.
    pub explorer_show_hidden: bool,
    /// Workspace-specific settings.
    pub workspace: WorkspaceConfig,
    /// User interface settings.
    pub ui: UiConfig,
    /// Configurable keybindings.
    pub keybindings: KeybindingsConfig,
}

impl Default for AppConfig {
    /// Provides a default `AppConfig` instance.
    fn default() -> Self {
        Self {
            name: Some("default".to_string()),
            default_workspace: current_dir().unwrap().join("asm_cli").join("programs"),
            arc_extensions: vec![".arc".to_string(), ".asm".to_string()],
            explorer_show_hidden: false,
            workspace: WorkspaceConfig::default(),
            ui: UiConfig::default(),
            keybindings: KeybindingsConfig::default(),
        }
    }
}

impl AppConfig {
    /// Saves the current `AppConfig` to a specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The `PathBuf` where the configuration should be saved.
    ///
    /// # Returns
    ///
    /// * `Result<(), ConfigError>` - `Ok(())` on success, or a `ConfigError` on failure.
    pub fn save_to_path(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)
            .map_err(ConfigError::SerializeError)?;
        
        std::fs::write(path, content)
            .map_err(ConfigError::IoError)?;
        
        Ok(())
    }
    
    /// Checks if a given file path has an ARC assembly file extension.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to the `Path` to check.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the file has a recognized ARC extension, `false` otherwise.
    pub fn is_arc_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.arc_extensions.iter().any(|e| e == ext))
            .unwrap_or(false)
    }
    
    /// Resets the current configuration to its default values.
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }
}

/// Represents possible errors that can occur during configuration operations.
#[derive(Debug)]
pub enum ConfigError {
    /// An I/O error occurred.
    IoError(std::io::Error),
    /// An error occurred during parsing of the configuration file.
    ParseError(toml::de::Error),
    /// An error occurred during serialization of the configuration.
    SerializeError(toml::ser::Error),
    /// The configuration directory could not be found.
    NoConfigDir,
    /// A generic error occurred during configuration loading.
    LoadError(String),
}

impl std::fmt::Display for ConfigError {
    /// Formats the `ConfigError` for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::ParseError(e) => write!(f, "Config parse error: {}", e),
            ConfigError::SerializeError(e) => write!(f, "Config serialize error: {}", e),
            ConfigError::NoConfigDir => write!(f, "Could not find config directory"),
            ConfigError::LoadError(e) => write!(f, "Config load error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Represents user interface-specific configuration settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiConfig {
    /// The name of the active UI theme.
    pub theme: String,
    /// Whether line numbers should be displayed in the editor.
    pub editor_line_numbers: bool,
    /// The width of a tab character in the editor.
    pub editor_tab_width: usize,
}

impl Default for UiConfig {
    /// Provides a default `UiConfig` instance.
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            editor_line_numbers: true,
            editor_tab_width: 4,
        }
    }
}

/// Represents workspace-specific configuration settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceConfig {
    /// Whether auto-save is enabled.
    pub auto_save: bool,
    /// The maximum number of recent files to remember.
    pub recent_files_limit: usize,
}

impl Default for WorkspaceConfig {
    /// Provides a default `WorkspaceConfig` instance.
    fn default() -> Self {
        Self {
            auto_save: true,
            recent_files_limit: 10,
        }
    }
}

/// Represents configurable keybindings for various actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeybindingsConfig {
    /// Keybindings for quitting the application or current mode.
    pub quit: Vec<String>,
    /// Keybindings for saving files.
    pub save: Vec<String>,
    /// Keybindings for creating a new file.
    pub new_file: Vec<String>,
}

impl Default for KeybindingsConfig {
    /// Provides a default `KeybindingsConfig` instance.
    fn default() -> Self {
        Self {
            quit: vec!["q".to_string(), "Esc".to_string()],
            save: vec!["Ctrl+s".to_string()],
            new_file: vec!["Ctrl+n".to_string()],
        }
    }
}