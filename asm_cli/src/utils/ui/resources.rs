//! # UI Resources Module
//!
//! This module defines data structures that act as resources for the user interface,
//! such as wrappers for `Workspace` and `AppConfig`, and a system for managing
//! application-wide status messages.

use crate::utils::{workspaces::Workspace, config::app_config::AppConfig};
use std::collections::VecDeque;

/// A wrapper struct for the `Workspace` to be used as a UI resource.
pub struct WorkspaceResource {
    /// The inner `Workspace` instance.
    pub inner: Workspace,
}

/// A wrapper struct for the `AppConfig` to be used as a UI resource.
pub struct ConfigResource {
    /// The inner `AppConfig` instance.
    pub inner: AppConfig,
}

/// The maximum number of status messages to display concurrently.
const MAX_STATUS_MESSAGES: usize = 3;
/// The duration (in frames) for which each status message is displayed.
const MESSAGE_DURATION_FRAMES: u32 = 100;

/// Manages application-wide status messages.
///
/// Status messages are displayed for a limited duration and a maximum number
/// can be shown at once.
pub struct AppStatus {
    /// A queue of messages, each paired with its remaining display timer.
    pub messages: VecDeque<(String, u32)>, // (message, timer)
    pub is_loading: bool,
    pub loading_progress: u8, // 0-100
}

impl Default for AppStatus {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
            is_loading: false,
            loading_progress: 0,
        }
    }
}

impl AppStatus {
    /// Sets a new status message.
    ///
    /// If the message queue is full, the oldest message is removed.
    /// The new message is added to the end of the queue with a fresh timer.
    ///
    /// # Arguments
    ///
    /// * `message` - The string content of the status message.
    pub fn set_message(&mut self, message: String) {
        // Remove oldest message if queue is full
        if self.messages.len() >= MAX_STATUS_MESSAGES {
            self.messages.pop_front();
        }
        self.messages.push_back((message, MESSAGE_DURATION_FRAMES));
    }
}