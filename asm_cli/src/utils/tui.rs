//! # TUI Application Core
//!
//! This module contains the core logic for the terminal user interface (TUI).
//! It manages the main application loop, state transitions, event handling,
//! and rendering of the different application screens.

use std::io;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, poll, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Terminal,
};

use super::{

    apps::{

        editor::{

            handle_text_editor_input, on_enter_text_editor, render_text_editor, TextEditorState,

        },

        emulator::{

            handle_emulator_input, on_enter_emulator, render_emulator, EmulatorState,

        },

        file_explorer::{

            handle_file_explorer_input, on_enter_file_explorer, render_file_explorer, FileExplorerState,

        },

        settings::{handle_settings_input, SettingsApp},

        start_menu::{handle_start_menu_input, on_enter_start_menu, render_start_menu, StartMenuState},

        help_guide::{

            handle_help_guide_input, HelpGuideState, render_help_guide,

        },

        io_devices::{

            handle_io_devices_input, render_io_devices, IoDevicesState,

        },

    },

    ui::{

        common::{handle_global_input, update_status_message, AppState, AppStatus, render_command_bar},

    },

    workspaces::Workspace,

};

use crate::utils::config::config_manager::ConfigManager;

/// The main application struct that holds the state of the TUI.
pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    app_state: AppState,
    prev_app_state: AppState,
    app_status: AppStatus,
    workspace: Workspace,
    config_manager: ConfigManager,
    emulator_state: EmulatorState,
    text_editor_state: TextEditorState,
    file_explorer_state: FileExplorerState,
    start_menu_state: StartMenuState,
    pub settings_app: SettingsApp,
    help_guide_state: HelpGuideState,
    _io_devices_state: IoDevicesState,
}

impl TuiApp {
    /// Creates a new `TuiApp`.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The `Workspace` to use for the application.
    /// * `config_manager` - The `ConfigManager` to use for the application.
    /// * `memory_size` - The total memory size for the emulator.
    ///
    /// # Returns
    ///
    /// * `TuiApp` - The new `TuiApp` instance.
    pub fn new(workspace: Workspace, config_manager: ConfigManager, memory_size: usize) -> TuiApp {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();

        let current_path_for_file_explorer = workspace.current_path.clone();

        TuiApp {
            terminal,
            app_state: AppState::StartMenu,
            prev_app_state: AppState::StartMenu,
            app_status: AppStatus::default(),
            workspace: Workspace::new(workspace.current_path, memory_size),
            settings_app: SettingsApp::new(config_manager.clone()),
            config_manager,
            emulator_state: EmulatorState::new(memory_size),
            text_editor_state: TextEditorState::new(),
            file_explorer_state: FileExplorerState::new(current_path_for_file_explorer),
            start_menu_state: StartMenuState::new(),
            help_guide_state: HelpGuideState::default(),
            _io_devices_state: IoDevicesState::default(),
        }
    }

    /// Runs the main application loop.
    ///
    /// This function initializes the terminal, enters the main event loop,
    /// and cleans up the terminal on exit.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - An empty `Result` on success, or an error on failure.
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        loop {
            let mut event = None;
            if poll(Duration::from_millis(250))? {
                event = Some(read()?);
            }

            if self.app_state != self.prev_app_state {
                match self.app_state {
                    AppState::StartMenu => on_enter_start_menu(&mut self.start_menu_state, &mut self.app_status),
                    AppState::TextEditor => on_enter_text_editor(&mut self.text_editor_state, &mut self.workspace, &mut self.app_status),
                    AppState::FileExplorer => on_enter_file_explorer(&mut self.file_explorer_state, &self.workspace, &mut self.app_status),
                    AppState::Emulator => {
                        on_enter_emulator(&mut self.emulator_state, &mut self.workspace, &mut self.app_status)
                    },
                    _ => {}
                }
                self.prev_app_state = self.app_state.clone();
            }

            let mut handled = false;
            if let Some(event) = event {
                match event {
                    Event::Key(key) => {
                        handle_global_input(key, &mut self.app_state, &mut handled);
                        if !handled {
                            match self.app_state {
                                AppState::TextEditor => handle_text_editor_input(key, &mut self.text_editor_state, &mut self.app_state, &mut self.workspace, &mut self.app_status, &mut handled),
                                AppState::FileExplorer => handle_file_explorer_input(key, &mut self.file_explorer_state, &mut self.app_state, &mut self.workspace, &mut self.app_status, &mut handled),
                                AppState::Emulator => handle_emulator_input(key, &mut self.emulator_state, &mut self.workspace, &mut self.app_state, &mut self.app_status, &mut handled),
                                AppState::Settings => handle_settings_input(key, &mut self.settings_app, &mut self.app_state, &mut handled),
                                AppState::HelpGuide => handle_help_guide_input(key, &mut self.help_guide_state, &mut self.app_state, &mut handled),
                                AppState::IoDevices => handle_io_devices_input(key, &mut self.workspace, &mut self.app_state, &mut handled),
                                AppState::StartMenu => handle_start_menu_input(key, &mut self.start_menu_state, &mut self.app_state, &mut self.app_status, &mut self.workspace, &mut handled),
                                _ => {}
                            }
                        }
                    },
                    Event::Resize(w, h) => {
                        self.terminal.resize(Rect::new(0, 0, w, h))?;
                    },
                    _ => {}
                }
            }

            if self.app_status.is_loading {
                // Advance loading progress for vibes
                self.app_status.loading_progress = (self.app_status.loading_progress + 5) % 101; // Increment by 5, wrap at 100
            } else {
                self.app_status.loading_progress = 0; // Reset when not loading
            }

            if update_status_message(&mut self.app_status) {
            }

            self.terminal.draw(|frame| {
                let size = frame.area();
                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(1),
                        Constraint::Length(1),
                    ])
                    .split(size);

                match self.app_state {
                    AppState::Emulator => {
                        if self.emulator_state.is_running {
                            match self.emulator_state.run_full_speed() {
                                Ok(_) => {},
                                Err(e) => {
                                    self.app_status.set_message(format!("Emulator error: {}", e));
                                    self.emulator_state.is_running = false;
                                }
                            }
                        }
                        render_emulator(frame, main_layout[0], &self.emulator_state, &self.workspace)
                    },
                    AppState::StartMenu => render_start_menu(frame, main_layout[0], &self.start_menu_state, &self.workspace, &self.config_manager.configs[self.config_manager.selected_config_index]),
                    AppState::Settings => self.settings_app.render(frame, main_layout[0]),
                    AppState::HelpGuide => render_help_guide(frame, main_layout[0], &mut self.help_guide_state),
                    AppState::IoDevices => render_io_devices(frame, main_layout[0], &self.workspace),
                    AppState::FileExplorer => render_file_explorer(frame, main_layout[0], &self.file_explorer_state, &self.workspace),
                    AppState::TextEditor => render_text_editor(frame, main_layout[0], &self.text_editor_state, &self.workspace),
                    _ => {}
                }

                let command_bar_area = if self.app_state == AppState::StartMenu {
                    let centered_area_for_start_menu = super::ui::common::centered_rect(80, 80, main_layout[0]);
                    Rect::new(centered_area_for_start_menu.x, main_layout[1].y, centered_area_for_start_menu.width, main_layout[1].height)
                } else {
                    main_layout[1]
                };
                render_command_bar(frame, command_bar_area, &self.app_state, &self.workspace, &self.app_status);
            })?;

            if self.app_state == AppState::Quit {
                break;
            }
        }

        self.terminal.show_cursor()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        disable_raw_mode()?;

        Ok(())
    }
}