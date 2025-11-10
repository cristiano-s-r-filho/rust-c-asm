//! # Common UI Components and State
//!
//! This module provides common components, state management, and helper functions
//! used across the TUI applications. It includes the global application state,
//! input handling, status messages, and layout helpers.

use ratatui::{
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Paragraph, Block},
    style::{Style, Color},
    Frame,
};
pub use crate::utils::ui::resources::AppStatus;

use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind};
use crate::utils::workspaces::Workspace;

/// Represents the different states or "screens" of the application.
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    /// The default starting screen.
    #[default]
    StartMenu,
    /// The text editor screen.
    TextEditor,
    /// The file explorer screen.
    FileExplorer,
    /// The emulator screen for running assembled code.
    Emulator,
    /// The settings screen.
    Settings,
    /// The help guide screen.
    HelpGuide,
    /// The I/O devices screen.
    IoDevices,
    /// A terminal state that signals the application to quit.
    Quit,
}

/// Handles global key events that are common across all application states.
///
/// # Arguments
///
/// * `key` - The `KeyEvent` to process.
/// * `app_state` - A mutable reference to the current `AppState`.
/// * `handled` - A mutable boolean that is set to `true` if the key event was handled.
pub fn handle_global_input(
    key: KeyEvent,
    app_state: &mut AppState,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    if key.code == KeyCode::Esc {
        *app_state = AppState::Quit;
        *handled = true;
    }
    if key.modifiers == KeyModifiers::ALT && key.code == KeyCode::Char('q') {
        *app_state = AppState::StartMenu;
        *handled = true;
    }
}

/// Updates the timers for status messages and removes expired ones.
///
/// # Arguments
///
/// * `status` - A mutable reference to the `AppStatus`.
///
/// # Returns
///
/// * `bool` - `true` if any message timers were updated or messages were removed, `false` otherwise.
pub fn update_status_message(
    status: &mut AppStatus,
) -> bool {
    let mut changed = false;
    for (_, timer) in status.messages.iter_mut() {
        if *timer > 0 {
            *timer -= 1;
            changed = true;
        }
    }

    while let Some((_, timer)) = status.messages.front() {
        if *timer == 0 {
            status.messages.pop_front();
            changed = true;
        } else {
            break;
        }
    }
    changed
}

/// Renders the command bar at the bottom of the screen.
///
/// The command bar displays the current screen's icon and name, the active file,
/// and context-sensitive command hints.
///
/// # Arguments
///
/// * `frame` - The `Frame` to render on.
/// * `area` - The `Rect` in which to render the command bar.
/// * `app_state` - The current `AppState`.
/// * `workspace` - The current `Workspace`.
pub fn render_command_bar(frame: &mut Frame, area: Rect, app_state: &AppState, workspace: &Workspace, app_status: &AppStatus) {
    const VERSION_STATUS: &str = "version 1.0.5 | 0 errors";
    const LOADING_BAR_CHAR: char = '█';

    let icon = get_status_icon(app_state);
    let screen_name = format!("{:?}", app_state);
    let file_name = workspace.active_file.as_ref()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("No file open");

    let command_bar_hint = match app_state {
        AppState::TextEditor => "Alt+S:Save Alt+A:SaveAs Alt+O:Open Alt+N:New Alt+Q:Back Alt+E:Run",
        AppState::FileExplorer => "↓:Down ↑:Up Enter:Open/Enter Alt+N:New Del:Delete Q:Back",
        AppState::Emulator => "P:Play/Pause S:Step R:Reset F:Flags I:I/O Q:Back",
        AppState::Settings => "↑↓:Navigate Enter:Select Esc:Back",
        _ => "",
    };

    let mut left_part = format!("{} {} - {} 🠶 ", icon, screen_name, file_name);
    if !command_bar_hint.is_empty() {
        left_part = format!("{} | {}", left_part, command_bar_hint);
    }

    let right_part = VERSION_STATUS;
    let available_width = area.width as usize;
    let min_total_width = left_part.len() + right_part.len();

    let middle_part = if app_status.is_loading && available_width > min_total_width + 2 { // +2 for some spacing
        let loading_bar_width = available_width - min_total_width - 2;
        let filled_chars = (loading_bar_width as f32 * (app_status.loading_progress as f32 / 100.0)).round() as usize;
        let empty_chars = loading_bar_width - filled_chars;

        format!(" {}{}{} ",
            LOADING_BAR_CHAR.to_string().repeat(filled_chars),
            " ".repeat(empty_chars),
            // Add a small animation to the end of the bar
            if app_status.loading_progress % 20 < 10 { "▰" } else { "▱" }
        )
    } else {
        " ".repeat(available_width.saturating_sub(min_total_width))
    };

    let full_bar_text = format!("{}{}{}", left_part, middle_part, right_part);

    let command_bar = Paragraph::new(full_bar_text)
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .block(
            Block::default()
                .borders(ratatui::widgets::Borders::NONE) // No borders
        );

    frame.render_widget(command_bar, area);
}

/// Returns a status icon character based on the current application state.
///
/// # Arguments
///
/// * `app_state` - The current `AppState`.
///
/// # Returns
///
/// * `&str` - A string slice containing the icon.
fn get_status_icon(app_state: &AppState) -> &str {
    match app_state {
        AppState::StartMenu => "⌂",
        AppState::Emulator => "𝈉",
        AppState::Settings => "⌂",
        _ => "🟕 ",
    }
}

/// Creates a centered rectangle for pop-ups.
///
/// # Arguments
///
/// * `percent_x` - The percentage of the width of the containing rectangle.
/// * `percent_y` - The percentage of the height of the containing rectangle.
/// * `r` - The containing `Rect`.
///
/// # Returns
///
/// * `Rect` - The calculated centered rectangle.
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}