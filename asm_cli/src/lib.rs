//! # ARC Emulator Library
//!
//! This crate provides the core components for the ARC (Accessible Reduced Computer)
//! emulator and assembler. It includes modules for CPU chips, instruction sets,
//! memory management, and various utilities for the terminal user interface (TUI).

pub mod chips;
pub mod instructions;
pub mod memory;
pub mod utils;
pub use crate::utils::tui::TuiApp;
pub use crate::utils::config::app_config::AppConfig;
pub use crate::utils::workspaces::Workspace;
