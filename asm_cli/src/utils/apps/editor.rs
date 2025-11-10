//! # Text Editor Application
//!
//! This module implements the text editor screen of the TUI. It allows
//! the user to edit `.arc` files, save them, open them, and assemble them.

use ratatui::{
    widgets::{Block, Borders, Paragraph, BorderType, Wrap},
    style::{Style, Color},
    text::{Text, Line, Span},
    layout::{Layout, Constraint, Direction, Rect},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind};
use tui_textarea::TextArea;
use std::path::PathBuf;
use crate::utils::ui::{
    common::{
        AppState, 
        AppStatus,
    },
};
use crate::utils::workspaces::Workspace;

/// Holds the state of the text editor.
pub struct TextEditorState {
    /// The `tui-textarea` widget.
    pub textarea: TextArea<'static>,
    /// The path to the file being edited.
    pub file_path: Option<PathBuf>,
    /// Whether the file has been modified since the last save.
    pub is_editing: bool,
    /// Whether the "save changes" dialog is being shown.
    pub show_save_dialog: bool,
    /// Whether the "save as" dialog is being shown.
    pub show_save_as_dialog: bool,
    /// The name of the new file being created.
    pub new_file_name: String,
    /// Whether the "open file" dialog is being shown.
    pub show_open_dialog: bool,
    /// The name of the file being opened.
    pub open_file_name: String,
    /// A list of assembly errors.
    pub assembly_errors: Vec<String>,
}

impl Default for TextEditorState {
    fn default() -> Self {
        let mut textarea = TextArea::default();
        
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(" ARC Editor (No File Loaded) ")
                .border_style(Style::default().fg(Color::LightBlue))
        );
        
        textarea.set_line_number_style(Style::default().fg(Color::White));
        textarea.set_cursor_line_style(Style::default().bg(Color::DarkGray));

        textarea.set_style(Style::default().fg(Color::White));

        Self {
            textarea,
            file_path: None,
            is_editing: true,
            show_save_dialog: false,
            show_save_as_dialog: false,
            new_file_name: String::new(),
            show_open_dialog: false,
            open_file_name: String::new(),
            assembly_errors: Vec::new(),
        }
    }
}

impl TextEditorState {
    /// Creates a new `TextEditorState`.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Loads a file into the text editor.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file to load.
    /// * `content` - The content of the file to load.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - An empty `Result` on success, or an error message on failure.
    pub fn load_file(&mut self, file_path: PathBuf, content: String) -> Result<(), String> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        self.textarea = TextArea::new(lines);
        let file_path_for_title = file_path.clone();
        self.file_path = Some(file_path);
        self.is_editing = true;
        
        let file_name = file_path_for_title.file_name().and_then(|s| s.to_str()).unwrap_or("Unknown File");
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" ARC Editor - {} ", file_name))
                .border_style(Style::default().fg(Color::LightBlue))
        );
        
        Ok(())
    }
    
    /// Returns the content of the text editor.
    pub fn get_content(&self) -> String {
        self.textarea.lines().join("\n")
    }
}

/// Handles key events for the text editor.
pub fn handle_text_editor_input(
    key: KeyEvent,
    editor_state: &mut TextEditorState,
    app_state: &mut AppState,
    workspace: &mut Workspace,
    app_status: &mut AppStatus,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    if editor_state.show_save_dialog {
        handle_save_dialog_input(key, editor_state, workspace, app_status);
        *handled = true;
        return;
    } else if editor_state.show_save_as_dialog {
        handle_save_as_dialog_input(key, editor_state, workspace, app_status);
        *handled = true;
        return;
    } else if editor_state.show_open_dialog {
        handle_open_dialog_input(key, editor_state, workspace, app_status);
        *handled = true;
        return;
    }
    
    match key.code {
        KeyCode::Char('n') if key.modifiers == KeyModifiers::ALT => {
            *editor_state = TextEditorState::new();
            workspace.active_file = None;
            app_status.set_message("New file created".to_string());
            *handled = true;
        }
        KeyCode::Char('s') if key.modifiers == KeyModifiers::ALT => {
            if let Some(ref path) = editor_state.file_path {
                match workspace.save_file(path, &editor_state.get_content()) {
                    Ok(()) => {
                        app_status.set_message("File saved successfully".to_string());
                        editor_state.is_editing = false;
                    }
                    Err(e) => {
                        app_status.set_message(format!("Save failed: {}", e));
                    }
                }
            } else {
                editor_state.show_save_as_dialog = true;
            }
            *handled = true;
        }
        KeyCode::Char('a') if key.modifiers == KeyModifiers::ALT => {
            editor_state.show_save_as_dialog = true;
            *handled = true;
        }
        KeyCode::Char('o') if key.modifiers == KeyModifiers::ALT => {
            editor_state.show_open_dialog = true;
            editor_state.open_file_name.clear();
            *handled = true;
        }
        KeyCode::Char('q') if key.modifiers == KeyModifiers::ALT => {
            if editor_state.is_editing {
                editor_state.show_save_dialog = true;
            } else {
                *app_state = AppState::StartMenu;
            }
            *handled = true;
        }
        KeyCode::Char('e') if key.modifiers == KeyModifiers::ALT => {
            editor_state.assembly_errors.clear();
            let content_to_assemble = editor_state.get_content();
            let current_file_path = editor_state.file_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "Unnamed Program".to_string());

            match workspace.try_assemble_program(&content_to_assemble, app_status) {
                Ok(_) => {
                    if let Err(e) = workspace.assemble_and_load_program(&content_to_assemble, app_status) {
                        let error_msg = format!("Assembly error in {}: {}", current_file_path, e);
                        app_status.set_message(error_msg.clone());
                    }
                    else {
                        app_status.set_message("Program assembled successfully! Switching to emulator...".to_string());
                        *app_state = AppState::Emulator;
                    }
                }
                Err(errors) => {
                    let formatted_errors: Vec<String> = errors.iter().map(|e| {
                        let error_msg = format!("Assembly error in {}: {}", current_file_path, e);
                        error_msg
                    }).collect();
                    editor_state.assembly_errors = formatted_errors.clone();
                    app_status.set_message(format!("Assembly failed with {} errors in {}", errors.len(), current_file_path));
                }
            }
            *handled = true;
        }
        _ => {
            if editor_state.textarea.input(key) {
                editor_state.is_editing = true;
                *handled = true;
            }
        }
    }
}

/// Handles key events for the "save changes" dialog.
fn handle_save_dialog_input(
    key: KeyEvent,
    editor_state: &mut TextEditorState,
    workspace: &mut Workspace,
    app_status: &mut AppStatus,
) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if let Some(ref path) = editor_state.file_path {
                if let Err(e) = workspace.save_file(path, &editor_state.get_content()) {
                    app_status.set_message(format!("Save failed: {}", e));
                } else {
                    app_status.set_message("File saved".to_string());
                    editor_state.is_editing = false;
                }
            } else {
                editor_state.show_save_as_dialog = true;
            }
            editor_state.show_save_dialog = false;
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            editor_state.show_save_dialog = false;
        }
        KeyCode::Esc => {
            editor_state.show_save_dialog = false;
        }
        _ => {}
    }
}

/// Handles key events for the "save as" dialog.
fn handle_save_as_dialog_input(
    key: KeyEvent,
    editor_state: &mut TextEditorState,
    workspace: &mut Workspace,
    app_status: &mut AppStatus,
) {
    match key.code {
        KeyCode::Char(c) if c.is_ascii() => {
            editor_state.new_file_name.push(c);
        }
        KeyCode::Backspace => {
            editor_state.new_file_name.pop();
        }
        KeyCode::Enter => {
            if !editor_state.new_file_name.is_empty() {
                let file_path = workspace.current_path.join(&editor_state.new_file_name);
                let file_path = if file_path.extension().is_none() {
                    file_path.with_extension("arc")
                } else {
                    file_path
                };
                match workspace.save_file_as(&file_path, &editor_state.get_content()) {
                    Ok(()) => {
                        app_status.set_message(format!("File saved as: {}", file_path.display()));
                        editor_state.file_path = Some(file_path);
                        editor_state.is_editing = false;
                    }
                    Err(e) => {
                        app_status.set_message(format!("Save failed: {}", e));
                    }
                }
            }
            editor_state.show_save_as_dialog = false;
        }
        KeyCode::Esc => {
            editor_state.show_save_as_dialog = false;
        }
        _ => {}
    }
}

/// Handles key events for the "open file" dialog.
fn handle_open_dialog_input(
    key: KeyEvent,
    editor_state: &mut TextEditorState,
    workspace: &mut Workspace,
    app_status: &mut AppStatus,
) {
    match key.code {
        KeyCode::Char(c) if c.is_ascii() => {
            editor_state.open_file_name.push(c);
        }
        KeyCode::Backspace => {
            editor_state.open_file_name.pop();
        }
        KeyCode::Enter => {
            if !editor_state.open_file_name.is_empty() {
                let file_path = workspace.current_path.join(&editor_state.open_file_name);
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => {
                        if let Err(e) = editor_state.load_file(file_path.clone(), content.clone()) {
                            app_status.set_message(format!("Failed to load file: {}", e));
                        } else {
                            match workspace.assemble_and_load_program(&content, app_status) {
                                Ok(_) => {
                                    app_status.set_message(format!("Loaded and assembled: {}", file_path.display()));
                                },
                                Err(e) => {
                                    app_status.set_message(format!("Assembly error after loading {}: {}", file_path.display(), e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        app_status.set_message(format!("Failed to open file: {}", e));
                    }
                }
            }
            editor_state.show_open_dialog = false;
        }
        KeyCode::Esc => {
            editor_state.show_open_dialog = false;
        }
        _ => {}
    }
}

/// Renders the text editor screen.
pub fn render_text_editor(
    frame: &mut Frame,
    area: Rect,
    editor_state: &TextEditorState,
    _workspace: &Workspace,
) {
    frame.render_widget(ratatui::widgets::Clear, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(if editor_state.assembly_errors.is_empty() { 0 } else { editor_state.assembly_errors.len() as u16 + 2 }),
            Constraint::Length(1),
        ])
        .split(area);

    let editor_area = chunks[0];
    let error_area = chunks[1];


    if editor_state.show_save_dialog {
        render_save_dialog(frame, editor_area);
    } else if editor_state.show_save_as_dialog {
        render_save_as_dialog(frame, editor_area, editor_state);
    } else if editor_state.show_open_dialog {
        render_open_dialog(frame, editor_area, editor_state);
    } else {
        render_editor(frame, editor_area, editor_state);
    }

    if !editor_state.assembly_errors.is_empty() {
        let error_text: Vec<Line> = editor_state.assembly_errors.iter()
            .map(|e| Line::from(Span::styled(e, Style::default().fg(Color::Red))))
            .collect();
        let error_block = Paragraph::new(Text::from(error_text))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .title(" Assembly Errors ")
                    .border_style(Style::default().fg(Color::Red))
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(error_block, error_area);
    }


}

/// Renders the text editor widget.
fn render_editor(frame: &mut Frame, area: Rect, editor_state: &TextEditorState) {
    frame.render_widget(&editor_state.textarea, area);
}

/// Renders the "save changes" dialog.
fn render_save_dialog(frame: &mut Frame, area: Rect) {
    let dialog = Paragraph::new(Text::from(vec![
        Line::from("Save changes before quitting?"),
        Line::from(Span::raw("")),
        Line::from(Span::styled("[Y]es", Style::default().fg(Color::Green))),
        Line::from(Span::styled("[N]o", Style::default().fg(Color::Red))),
        Line::from(Span::styled("[Esc] Cancel", Style::default().fg(Color::Gray))),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .title(" Save Changes ")
            .border_style(Style::default().fg(Color::Yellow))
    )
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center);
    
    frame.render_widget(dialog, area);
}

/// Renders the "save as" dialog.
fn render_save_as_dialog(frame: &mut Frame, area: Rect, editor_state: &TextEditorState) {
    let dialog_text = format!("Save as: {}.arc", editor_state.new_file_name);
    
    let dialog = Paragraph::new(Text::from(dialog_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(" Save As ")
                .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: true });
    
    frame.render_widget(dialog, area);
}

/// Renders the "open file" dialog.
fn render_open_dialog(frame: &mut Frame, area: Rect, editor_state: &TextEditorState) {
    let dialog_text = format!("Open file: {}", editor_state.open_file_name);
    
    let dialog = Paragraph::new(Text::from(dialog_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(" Open File ")
                .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: true });
    
    frame.render_widget(dialog, area);
}

/// Called when entering the text editor state.
pub fn on_enter_text_editor(
    editor_state: &mut TextEditorState,
    workspace: &mut Workspace,
    app_status: &mut AppStatus,
) {
    if let Some(active_file_path) = workspace.active_file.clone() {
        if let Ok(content) = std::fs::read_to_string(active_file_path.as_path()) {
            if let Err(e) = editor_state.load_file(active_file_path.clone(), content.clone()) {
                app_status.set_message(format!("Failed to load file: {}", e));
            } else {
                match workspace.assemble_and_load_program(&content, app_status) {
                    Ok(_) => {
                        app_status.set_message(format!("Loaded and assembled: {}", active_file_path.display()));
                    },
                    Err(e) => {
                        let error_msg = format!("Assembly error after loading {}: {}", active_file_path.display(), e);
                        editor_state.assembly_errors.push(error_msg.clone());
                        app_status.set_message(error_msg.clone());
                    }
                }
            }
        }
    } else {
        *editor_state = TextEditorState::new();
        app_status.set_message("New ARC file - Start editing your assembly code".to_string());
    }
}
