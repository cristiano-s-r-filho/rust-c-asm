//! # Settings Application
//!
//! This module implements the settings screen of the TUI. It allows
//! the user to view and modify various application settings, including
//! UI and theme, editor settings, keybindings, explorer settings, and workspace settings.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap, BorderType},
    text::{Text},
    Frame,
};

use crate::utils::config::config_manager::ConfigManager;
use crate::utils::ui::common::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

/// Represents the top-level sections in the settings menu.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SettingSection {
    /// UI and Theme settings.
    UiAndTheme,
    /// Editor-specific settings.
    EditorSettings,
    /// Keybinding configurations.
    Keybindings,
    /// File explorer settings.
    ExplorerSettings,
    /// Workspace-related settings.
    WorkspaceSettings,
    /// Action to save all current changes.
    SaveChanges,
    /// Action to discard all unsaved changes.
    DiscardChanges,
    /// Action to quit the settings menu.
    Quit,
}

/// Represents the current editing state within the settings menu.
pub enum SettingEditState {
    /// No setting is currently being edited or viewed in detail.
    None,
    /// Editing the theme setting.
    EditingTheme(String),
    /// Editing the tab width setting.
    EditingTabWidth(String),
    /// Editing the line numbers visibility setting.
    EditingLineNumbers(bool),
    /// Editing the keybinding for quitting.
    EditingKeybindingQuit(String),
    /// Editing the keybinding for saving.
    EditingKeybindingSave(String),
    /// Editing the keybinding for creating a new file.
    EditingKeybindingNewFile(String),
    /// Editing the setting for showing hidden files in the explorer.
    EditingExplorerShowHidden(bool),
    /// Editing the auto-save setting.
    EditingAutoSave(bool),
    /// Editing the recent files limit setting.
    EditingRecentFilesLimit(String),
    /// Viewing options within the editor settings section.
    ViewingEditorOptions,
    /// Viewing options within the keybindings section.
    ViewingKeybindingsOptions,
    /// Viewing options within the workspace settings section.
    ViewingWorkspaceOptions,
}

/// Manages the state and rendering of the settings application.
pub struct SettingsApp {
    /// The list of top-level setting sections.
    pub sections: Vec<SettingSection>,
    /// The state of the selected section in the list.
    pub selected_section: ListState,
    /// The state of the selected option within a section.
    pub selected_option_index: ListState,
    /// The current editing state.
    pub edit_state: SettingEditState,
    /// The configuration manager for loading and saving settings.
    pub config_manager: ConfigManager,
}

impl SettingsApp {
    /// Creates a new `SettingsApp` instance.
    ///
    /// # Arguments
    ///
    /// * `config_manager` - The `ConfigManager` to use for managing application configurations.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `SettingsApp` instance.
    pub fn new(config_manager: ConfigManager) -> Self {
        let mut app = Self {
            sections: vec![
                SettingSection::UiAndTheme,
                SettingSection::EditorSettings,
                SettingSection::Keybindings,
                SettingSection::ExplorerSettings,
                SettingSection::WorkspaceSettings,
                SettingSection::SaveChanges,
                SettingSection::DiscardChanges,
                SettingSection::Quit,
            ],
            selected_section: ListState::default(),
            selected_option_index: ListState::default(),
            edit_state: SettingEditState::None,
            config_manager,
        };
        app.selected_section.select(Some(0));
        app.selected_option_index.select(Some(0));
        app
    }
}

impl SettingsApp {
    /// Renders the settings application UI.
    ///
    /// # Arguments
    ///
    /// * `frame` - The `Frame` to render on.
    /// * `area` - The `Rect` in which to render the settings UI.
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(ratatui::widgets::Clear, area);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let nav_bar_area = main_chunks[0];
        let descriptor_area = main_chunks[1];

        self.render_nav_bar(frame, nav_bar_area);
        self.render_bottom_descriptor(frame, descriptor_area);
    }

    /// Renders the navigation bar for selecting setting sections.
    fn render_nav_bar(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.sections.iter()
            .map(|section| {
                let text = match section {
                    SettingSection::UiAndTheme => "UI & Theme",
                    SettingSection::EditorSettings => "Editor",
                    SettingSection::Keybindings => "Keybindings",
                    SettingSection::ExplorerSettings => "Explorer",
                    SettingSection::WorkspaceSettings => "Workspace",
                    SettingSection::SaveChanges => "Save Changes",
                    SettingSection::DiscardChanges => "Discard Changes",
                    SettingSection::Quit => "Quit",
                };
                ListItem::new(text)
            })
            .collect();
        
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .title("Settings Sections")
                    .border_style(Style::default().fg(Color::LightBlue))
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow));

        frame.render_stateful_widget(list, area, &mut self.selected_section.clone());
    }

    /// Renders the bottom descriptor area, showing details or options for the selected section.
    fn render_bottom_descriptor(&self, frame: &mut Frame, area: Rect) {
        let config = &self.config_manager.configs[self.config_manager.selected_config_index];
        let selected_section_index = self.selected_section.selected().unwrap_or(0);
        let current_section = self.sections[selected_section_index];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .title(format!("{} Details", match current_section {
                SettingSection::UiAndTheme => "UI & Theme",
                SettingSection::EditorSettings => "Editor",
                SettingSection::Keybindings => "Keybindings",
                SettingSection::ExplorerSettings => "Explorer",
                SettingSection::WorkspaceSettings => "Workspace",
                SettingSection::SaveChanges => "Save",
                SettingSection::DiscardChanges => "Discard",
                SettingSection::Quit => "Quit",
            }))
            .border_style(Style::default().fg(Color::Green));

        match current_section {
            SettingSection::EditorSettings => {
                let items = self.render_editor_settings(config);
                let list = List::new(items)
                    .block(block)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow));
                frame.render_stateful_widget(list, area, &mut self.selected_option_index.clone());
            },
            SettingSection::Keybindings => {
                let items = self.render_keybindings_settings(config);
                let list = List::new(items)
                    .block(block)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow));
                frame.render_stateful_widget(list, area, &mut self.selected_option_index.clone());
            },
            SettingSection::WorkspaceSettings => {
                let items = self.render_workspace_settings(config);
                let list = List::new(items)
                    .block(block)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow));
                frame.render_stateful_widget(list, area, &mut self.selected_option_index.clone());
            },
            _ => {
                let content = match current_section {
                    SettingSection::UiAndTheme => self.render_ui_and_theme_settings(config),
                    SettingSection::ExplorerSettings => self.render_explorer_settings(config),
                    SettingSection::SaveChanges => Text::from("Press Enter to save all configuration changes to file."),
                    SettingSection::DiscardChanges => Text::from("Press Enter to discard all unsaved changes and reload from file."),
                    SettingSection::Quit => Text::from("Press Enter to quit the settings menu and return to Start Menu."),
                    _ => Text::from(""),
                };
                let paragraph = Paragraph::new(content)
                    .block(block)
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, area);
            }
        }
    }

    /// Renders the UI and Theme settings options.
    fn render_ui_and_theme_settings(&self, config: &crate::utils::config::app_config::AppConfig) -> Text<'static> {
        match &self.edit_state {
            SettingEditState::EditingTheme(current_value) => {
                Text::from(format!("Enter new theme (current: {}): {}", config.ui.theme, current_value))
            },
            _ => Text::from(format!("Theme: {}", config.ui.theme)),
        }
    }

    /// Renders the Editor settings options.
    fn render_editor_settings(&self, config: &crate::utils::config::app_config::AppConfig) -> Vec<ListItem<'static>> {
        let items = vec![
            format!("Tab Width: {}", config.ui.editor_tab_width),
            format!("Line Numbers: {}", config.ui.editor_line_numbers),
        ];

        items.into_iter().enumerate().map(|(i, item)| {
            let style = if self.selected_option_index.selected() == Some(i) {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(item).style(style)
        }).collect()
    }

    /// Renders the Keybindings settings options.
    fn render_keybindings_settings(&self, config: &crate::utils::config::app_config::AppConfig) -> Vec<ListItem<'static>> {
        let items = vec![
            format!("Quit: {}", config.keybindings.quit.join(", ")),
            format!("Save: {}", config.keybindings.save.join(", ")),
            format!("New File: {}", config.keybindings.new_file.join(", ")),
        ];

        items.into_iter().enumerate().map(|(i, item)| {
            let style = if self.selected_option_index.selected() == Some(i) {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(item).style(style)
        }).collect()
    }

    /// Renders the Explorer settings options.
    fn render_explorer_settings(&self, config: &crate::utils::config::app_config::AppConfig) -> Text<'static> {
        match &self.edit_state {
            SettingEditState::EditingExplorerShowHidden(current_value) => {
                Text::from(format!("Toggle show hidden files (current: {}): {}", config.explorer_show_hidden, current_value))
            },
            _ => Text::from(format!("Show Hidden Files: {}", config.explorer_show_hidden)),
        }
    }

    /// Renders the Workspace settings options.
    fn render_workspace_settings(&self, config: &crate::utils::config::app_config::AppConfig) -> Vec<ListItem<'static>> {
         let items = vec![
            format!("Default Workspace: {}", config.default_workspace.display()),
            format!("Auto Save: {}", config.workspace.auto_save),
            format!("Recent Files Limit: {}", config.workspace.recent_files_limit),
        ];

        items.into_iter().enumerate().map(|(i, item)| {
            let style = if self.selected_option_index.selected() == Some(i) {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(item).style(style)
        }).collect()
    }

    /// Moves the selection to the next setting section.
    pub fn next_section(&mut self) {
        let i = match self.selected_section.selected() {
            Some(i) => {
                if i >= self.sections.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_section.select(Some(i));
        self.selected_option_index.select(Some(0));
    }

    /// Moves the selection to the previous setting section.
    pub fn previous_section(&mut self) {
        let i = match self.selected_section.selected() {
            Some(i) => {
                if i == 0 {
                    self.sections.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_section.select(Some(i));
        self.selected_option_index.select(Some(0));
    }

    /// Moves the selection to the next option within the current setting section.
    pub fn next_option(&mut self) {
        let selected_section_index = self.selected_section.selected().unwrap_or(0);
        let current_section = self.sections[selected_section_index];
        let num_options = match current_section {
            SettingSection::UiAndTheme => 1,
            SettingSection::EditorSettings => 2,
            SettingSection::Keybindings => 3,
            SettingSection::ExplorerSettings => 1,
            SettingSection::WorkspaceSettings => 3,
            _ => 0,
        };

        if num_options > 0 {
            let i = match self.selected_option_index.selected() {
                Some(i) => {
                    if i >= num_options - 1 {
                        0
                    } else {
                        i + 1
                    }
                },
                None => 0,
            };
            self.selected_option_index.select(Some(i));
        } else {
            self.selected_option_index.select(None);
        }
    }

    /// Moves the selection to the previous option within the current setting section.
    pub fn previous_option(&mut self) {
        let selected_section_index = self.selected_section.selected().unwrap_or(0);
        let current_section = self.sections[selected_section_index];
        let num_options = match current_section {
            SettingSection::UiAndTheme => 1,
            SettingSection::EditorSettings => 2,
            SettingSection::Keybindings => 3,
            SettingSection::ExplorerSettings => 1,
            SettingSection::WorkspaceSettings => 3,
            _ => 0,
        };

        if num_options > 0 {
            let i = match self.selected_option_index.selected() {
                Some(i) => {
                    if i == 0 {
                        num_options - 1
                    } else {
                        i - 1
                    }
                },
                None => 0,
            };
            self.selected_option_index.select(Some(i));
        } else {
            self.selected_option_index.select(None);
        }
    }
}

/// Handles key events for the settings application.
pub fn handle_settings_input(
    key: KeyEvent,
    settings_app: &mut SettingsApp,
    app_state: &mut AppState,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    let config = &mut settings_app.config_manager.configs[settings_app.config_manager.selected_config_index];

    match &mut settings_app.edit_state {
        SettingEditState::None | SettingEditState::ViewingEditorOptions | SettingEditState::ViewingKeybindingsOptions | SettingEditState::ViewingWorkspaceOptions => {
            let selected_section_index = settings_app.selected_section.selected().unwrap_or(0);
            let current_section = settings_app.sections[selected_section_index];

            let in_option_view = matches!(settings_app.edit_state, SettingEditState::ViewingEditorOptions | SettingEditState::ViewingKeybindingsOptions | SettingEditState::ViewingWorkspaceOptions);

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    if in_option_view {
                        settings_app.edit_state = SettingEditState::None;
                        settings_app.selected_option_index.select(None);
                    } else {
                        *app_state = AppState::StartMenu;
                    }
                    *handled = true;
                }
                KeyCode::Down => {
                    if in_option_view {
                        settings_app.next_option();
                    } else {
                        settings_app.next_section();
                    }
                    *handled = true;
                }
                KeyCode::Up => {
                    if in_option_view {
                        settings_app.previous_option();
                    } else {
                        settings_app.previous_section();
                    }
                    *handled = true;
                }
                KeyCode::Enter => {
                    if in_option_view {
                        if let Some(selected_option_index) = settings_app.selected_option_index.selected() {
                            match current_section {
                                SettingSection::EditorSettings => {
                                    match selected_option_index {
                                        0 => settings_app.edit_state = SettingEditState::EditingTabWidth(config.ui.editor_tab_width.to_string()),
                                        1 => settings_app.edit_state = SettingEditState::EditingLineNumbers(config.ui.editor_line_numbers),
                                        _ => {}
                                    }
                                },
                                SettingSection::Keybindings => {
                                    match selected_option_index {
                                        0 => settings_app.edit_state = SettingEditState::EditingKeybindingQuit(config.keybindings.quit[0].clone()),
                                        1 => settings_app.edit_state = SettingEditState::EditingKeybindingSave(config.keybindings.save[0].clone()),
                                        2 => settings_app.edit_state = SettingEditState::EditingKeybindingNewFile(config.keybindings.new_file[0].clone()),
                                        _ => {}
                                    }
                                },
                                SettingSection::WorkspaceSettings => {
                                    match selected_option_index {
                                        0 => settings_app.edit_state = SettingEditState::EditingAutoSave(config.workspace.auto_save),
                                        1 => settings_app.edit_state = SettingEditState::EditingRecentFilesLimit(config.workspace.recent_files_limit.to_string()),
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        }
                    } else {
                        match current_section {
                            SettingSection::UiAndTheme => {
                                settings_app.edit_state = SettingEditState::EditingTheme(config.ui.theme.clone());
                            },
                            SettingSection::EditorSettings => {
                                settings_app.edit_state = SettingEditState::ViewingEditorOptions;
                                settings_app.selected_option_index.select(Some(0));
                            },
                            SettingSection::Keybindings => {
                                settings_app.edit_state = SettingEditState::ViewingKeybindingsOptions;
                                settings_app.selected_option_index.select(Some(0));
                            },
                            SettingSection::ExplorerSettings => {
                                settings_app.edit_state = SettingEditState::EditingExplorerShowHidden(config.explorer_show_hidden);
                            },
                            SettingSection::WorkspaceSettings => {
                                settings_app.edit_state = SettingEditState::ViewingWorkspaceOptions;
                                settings_app.selected_option_index.select(Some(0));
                            },
                            SettingSection::SaveChanges => {
                                match settings_app.config_manager.save_current_config() {
                                    Ok(_) => { },
                                    Err(_e) => { }
                                }
                            },
                            SettingSection::DiscardChanges => {
                                *settings_app = SettingsApp::new(settings_app.config_manager.clone());
                            },
                            SettingSection::Quit => {
                                *app_state = AppState::StartMenu;
                            },
                        }
                    }
                    *handled = true;
                }
                _ => {}
            }
        },
        SettingEditState::EditingTheme(current_value) => {
            match key.code {
                KeyCode::Char(c) => {
                    current_value.push(c);
                    *handled = true;
                },
                KeyCode::Backspace => {
                    current_value.pop();
                    *handled = true;
                },
                KeyCode::Enter => {
                    config.ui.theme = current_value.clone();
                    settings_app.edit_state = SettingEditState::None;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::None;
                    *handled = true;
                },
                _ => {} 
            }
        },
        SettingEditState::EditingTabWidth(current_value) => {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    current_value.push(c);
                    *handled = true;
                },
                KeyCode::Backspace => {
                    current_value.pop();
                    *handled = true;
                },
                KeyCode::Enter => {
                    if let Ok(width) = current_value.parse::<usize>() {
                        config.ui.editor_tab_width = width;
                    }
                    settings_app.edit_state = SettingEditState::ViewingEditorOptions;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::ViewingEditorOptions;
                    *handled = true;
                },
                _ => {} 
            }
        },
        SettingEditState::EditingLineNumbers(current_value) => {
            match key.code {
                KeyCode::Enter => {
                    *current_value = !*current_value;
                    config.ui.editor_line_numbers = *current_value;
                    settings_app.edit_state = SettingEditState::ViewingEditorOptions;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::ViewingEditorOptions;
                    *handled = true;
                },
                _ => {}
            }
        },
        SettingEditState::EditingKeybindingQuit(current_value) => {
            match key.code {
                KeyCode::Char(c) => {
                    current_value.clear();
                    current_value.push(c);
                    *handled = true;
                },
                KeyCode::Enter => {
                    config.keybindings.quit = vec![current_value.clone()];
                    settings_app.edit_state = SettingEditState::ViewingKeybindingsOptions;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::ViewingKeybindingsOptions;
                    *handled = true;
                },
                _ => {}
            }
        },
        SettingEditState::EditingKeybindingSave(current_value) => {
            match key.code {
                KeyCode::Char(c) => {
                    current_value.clear();
                    current_value.push(c);
                    *handled = true;
                },
                KeyCode::Enter => {
                    config.keybindings.save = vec![current_value.clone()];
                    settings_app.edit_state = SettingEditState::ViewingKeybindingsOptions;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::ViewingKeybindingsOptions;
                    *handled = true;
                },
                _ => {}
            }
        },
        SettingEditState::EditingKeybindingNewFile(current_value) => {
            match key.code {
                KeyCode::Char(c) => {
                    current_value.clear();
                    current_value.push(c);
                    *handled = true;
                },
                KeyCode::Enter => {
                    config.keybindings.new_file = vec![current_value.clone()];
                    settings_app.edit_state = SettingEditState::ViewingKeybindingsOptions;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::ViewingKeybindingsOptions;
                    *handled = true;
                },
                _ => {} 
            }
        },
        SettingEditState::EditingExplorerShowHidden(current_value) => {
            match key.code {
                KeyCode::Enter => {
                    *current_value = !*current_value;
                    config.explorer_show_hidden = *current_value;
                    settings_app.edit_state = SettingEditState::None;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::None;
                    *handled = true;
                },
                _ => {}
            }
        },
        SettingEditState::EditingAutoSave(current_value) => {
            match key.code {
                KeyCode::Enter => {
                    *current_value = !*current_value;
                    config.workspace.auto_save = *current_value;
                    settings_app.edit_state = SettingEditState::ViewingWorkspaceOptions;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::ViewingWorkspaceOptions;
                    *handled = true;
                },
                _ => {} 
            }
        },
        SettingEditState::EditingRecentFilesLimit(current_value) => {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    current_value.push(c);
                    *handled = true;
                },
                KeyCode::Backspace => {
                    current_value.pop();
                    *handled = true;
                },
                KeyCode::Enter => {
                    if let Ok(limit) = current_value.parse::<usize>() {
                        config.workspace.recent_files_limit = limit;
                    }
                    settings_app.edit_state = SettingEditState::ViewingWorkspaceOptions;
                    *handled = true;
                },
                KeyCode::Esc => {
                    settings_app.edit_state = SettingEditState::ViewingWorkspaceOptions;
                    *handled = true;
                },
                _ => {} 
            }
        },
    }
}