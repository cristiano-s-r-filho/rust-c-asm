//! # File Explorer Application
//!
//! This module implements the file explorer screen of the TUI. It allows
//! the user to navigate the file system, open `.arc` files, create new files,
//! and delete files.

use ratatui::{
    widgets::{List, ListItem, Block, Borders, Paragraph, Wrap, BorderType},
    style::{Style, Color, Modifier},
    text::{Text, Line, Span},
    layout::{Layout, Constraint, Direction, Rect},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode, KeyEventKind, KeyModifiers};
use std::path::PathBuf;
use crate::utils::ui::{common::{AppState, AppStatus}};
use crate::utils::workspaces::Workspace;

/// Holds the state of the file explorer.
pub struct FileExplorerState {
    /// The directory currently being displayed.
    pub current_directory: PathBuf,
    /// The list of entries in the current directory.
    pub entries: Vec<FileEntry>,
    /// The index of the currently selected entry.
    pub selected_index: usize,
    /// The scroll offset for the list of entries.
    pub scroll_offset: usize,
    /// Whether the "new file" dialog is being shown.
    pub show_new_file_dialog: bool,
    /// The name of the new file being created.
    pub new_file_name: String,
}

/// Represents an entry in the file explorer, which can be a file or a directory.
#[derive(Clone)]
pub struct FileEntry {
    /// The name of the file or directory.
    pub name: String,
    /// The full path to the file or directory.
    pub path: PathBuf,
    /// Whether the entry is a directory.
    pub is_dir: bool,
    /// Whether the entry is an `.arc` or `.asm` file.
    pub is_arc_file: bool,
}

impl FileExplorerState {
    /// Creates a new `FileExplorerState`.
    ///
    /// # Arguments
    ///
    /// * `workspace_path` - The path to the initial workspace directory.
    ///
    /// # Returns
    ///
    /// * `Self` - The new `FileExplorerState` instance.
    pub fn new(workspace_path: PathBuf) -> Self {
        let mut state = Self {
            current_directory: workspace_path,
            entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            show_new_file_dialog: false,
            new_file_name: String::new(),
        };
        state.refresh_entries();
        state
    }
    
    /// Refreshes the list of entries in the current directory.
    pub fn refresh_entries(&mut self) {
        self.entries.clear();
        
        if let Some(parent) = self.current_directory.parent() {
            self.entries.push(FileEntry {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_dir: true,
                is_arc_file: false,
            });
        }
        
        if let Ok(entries) = std::fs::read_dir(&self.current_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                let is_dir = path.is_dir();
                let is_arc_file = path.extension()
                    .map(|ext| ext == "arc" || ext == "asm")
                    .unwrap_or(false);
                
                self.entries.push(FileEntry {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path,
                    is_dir,
                    is_arc_file,
                });
            }
        }
        
        self.entries.sort_by(|a, b| {
            a.is_dir.cmp(&b.is_dir).reverse().then(a.name.cmp(&b.name))
        });
    }
    
    /// Returns the currently selected entry, if any.
    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }

    /// Adjusts the scroll offset based on the selected index.
    pub fn scroll(&mut self) {
        let list_height = 10;
        if self.selected_index >= self.scroll_offset + list_height {
            self.scroll_offset = self.selected_index - list_height + 1;
        }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }
}

/// Handles key events for the file explorer.
pub fn handle_file_explorer_input(
    key: KeyEvent,
    explorer_state: &mut FileExplorerState,
    app_state: &mut AppState,
    workspace: &mut Workspace,
    app_status: &mut AppStatus,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    if explorer_state.show_new_file_dialog {
        handle_new_file_dialog_input(key, explorer_state, workspace, app_status);
        *handled = true;
        return;
    }
    
    let entry_count = explorer_state.entries.len();
    
    match key.code {
        KeyCode::Down => {
            explorer_state.selected_index = (explorer_state.selected_index + 1) % entry_count;
            explorer_state.scroll();
            *handled = true;
        }
        KeyCode::Up => {
            explorer_state.selected_index = if explorer_state.selected_index == 0 {
                entry_count - 1
            } else {
                explorer_state.selected_index - 1
            };
            explorer_state.scroll();
            *handled = true;
        }
        KeyCode::Enter => {
            if let Some(entry) = explorer_state.selected_entry().cloned() {
                if entry.is_dir {
                    explorer_state.current_directory = entry.path.clone();
                    explorer_state.selected_index = 0;
                    explorer_state.refresh_entries();
                } else if entry.is_arc_file {
                    match workspace.open_file_for_editing(&entry.path) {
                        Ok(()) => {
                            app_status.set_message(format!("Opened: {}", entry.name));
                            *app_state = AppState::TextEditor;
                        }
                        Err(e) => {
                            app_status.set_message(format!("Failed to open: {}", e));
                        }
                    }
                }
            }
            *handled = true;
        }
        KeyCode::Char('n') if key.modifiers == KeyModifiers::ALT => {
            explorer_state.show_new_file_dialog = true;
            explorer_state.new_file_name.clear();
            *handled = true;
        }
        KeyCode::Delete => {
            if let Some(entry) = explorer_state.selected_entry().cloned() {
                if !entry.is_dir || entry.name != ".." {
                    let _ = std::fs::remove_file(&entry.path);
                    explorer_state.refresh_entries();
                    app_status.set_message(format!("Deleted: {}", entry.name));
                }
            }
            *handled = true;
        }
        KeyCode::Char('q') => {
            *app_state = AppState::StartMenu;
            *handled = true;
        }
        _ => {}
    }
}

/// Handles key events for the "new file" dialog.
fn handle_new_file_dialog_input(
    key: KeyEvent,
    explorer_state: &mut FileExplorerState,
    workspace: &mut Workspace,
    app_status: &mut AppStatus,
) {
    match key.code {
        KeyCode::Char(c) if c.is_ascii() => {
            explorer_state.new_file_name.push(c);
        }
        KeyCode::Backspace => {
            explorer_state.new_file_name.pop();
        }
        KeyCode::Enter => {
            if !explorer_state.new_file_name.is_empty() {
                let file_path = explorer_state.current_directory
                    .join(&explorer_state.new_file_name);
                
                let file_path = if file_path.extension().is_none() {
                    file_path.with_extension("arc")
                } else {
                    file_path
                };
                
                match workspace.create_new_arc_file(&file_path) {
                    Ok(()) => {
                        app_status.set_message(format!("Created: {}", file_path.display()));
                        explorer_state.refresh_entries();
                    }
                    Err(e) => {
                        app_status.set_message(format!("Failed to create: {}", e));
                    }
                }
            }
            explorer_state.show_new_file_dialog = false;
        }
        KeyCode::Esc => {
            explorer_state.show_new_file_dialog = false;
        }
        _ => {}
    }
}

/// Renders the file explorer screen.
pub fn render_file_explorer(
    frame: &mut Frame,
    area: Rect,
    explorer_state: &FileExplorerState,
    _workspace: &Workspace,
) {
    frame.render_widget(ratatui::widgets::Clear, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    if explorer_state.show_new_file_dialog {
        render_new_file_dialog(frame, chunks[0], explorer_state);
    } else {
        render_file_list(frame, chunks[0], explorer_state);
    }


}

/// Renders the list of files and directories.
fn render_file_list(frame: &mut Frame, area: Rect, explorer_state: &FileExplorerState) {
    let items: Vec<ListItem> = explorer_state.entries
        .iter()
        .skip(explorer_state.scroll_offset)
        .enumerate()
        .map(|(index, entry)| {
            let icon = if entry.is_dir { 
                "🗀" 
            } else if entry.is_arc_file { 
                "🗎" 
            } else { 
                "🗋" 
            };
            
            let style = if index + explorer_state.scroll_offset == explorer_state.selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            let content = Line::from(vec![
                Span::from(icon),
                Span::from(" "),
                Span::from(entry.name.clone()).style(style),
            ]);
            
            ListItem::new(content).style(style)
        })
        .collect();
    
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(format!(" Path: {} ", explorer_state.current_directory.display()))
        )
        .highlight_symbol("▶ ");
    
    frame.render_widget(list, area);
}

/// Renders the "new file" dialog.
fn render_new_file_dialog(frame: &mut Frame, area: Rect, explorer_state: &FileExplorerState) {
    let dialog_text = format!("New file name: {}.arc", explorer_state.new_file_name);
    
    let dialog = Paragraph::new(Text::from(dialog_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(" New ARC File ")
                .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: true });
    
    frame.render_widget(dialog, area);
}

/// Called when entering the file explorer state.
pub fn on_enter_file_explorer(
    explorer_state: &mut FileExplorerState,
    _workspace: &Workspace,
    app_status: &mut AppStatus,
) {
    explorer_state.refresh_entries();
    app_status.set_message("File Explorer: Create and manage ARC files".to_string());
}
