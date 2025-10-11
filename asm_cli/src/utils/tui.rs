use crate::memory::registers::Reg;
use std::collections::VecDeque;
use std::io;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;
use ratatui::Terminal;

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::command_processor::{self, execute_command, parse_command, Command};
use crate::utils::widgets::{
    render_cpu_panel, render_memory_panel, render_menu, render_input_panel, 
    render_status_bar, render_start_menu, render_settings_menu, 
    render_program_editor, render_help_guide, render_program_naming, 
    render_program_list,
};

#[derive(Debug, PartialEq)]
pub enum AppMode {
    StartMenu,
    CommandMode,
    Running,
    Paused,
}

#[derive(Debug, PartialEq)]
pub enum EditorMode {
    Normal,
    Edit,
    Naming,
    LoadList,
    EditingExisting,
}

#[derive(Debug, Clone)]
pub struct ProgramInfo {
    pub name: String,
    pub content: String,
    pub commands: VecDeque<Command>,
    pub machine_code: Vec<u32>,
}

#[derive(Debug)]
pub struct AppState {
    pub cpu: CPU,
    pub work_memory: WorkMemory,
    pub should_quit: bool,
    pub mode: AppMode,
    pub input_buffer: String,
    pub command_queue: VecDeque<Command>,
    pub last_message: Option<String>,
    pub message_is_error: bool,
    pub cursor_position: usize,
    pub selected_menu_item: usize,
    pub show_command_queue: bool,
    pub show_call_stack: bool,
    pub settings_menu_active: bool,
    pub settings_selected_item: usize,
    pub loaded_program: Option<String>,
    pub loaded_program_size: usize,
    pub editor_screen: bool,
    pub editor_text: String,
    pub editor_cursor_position: usize,
    pub editor_menu_selected: usize,
    pub current_file: Option<String>,
    pub show_help: bool,
    pub editor_mode: EditorMode,
    pub program_name_input: String,
    pub program_name_cursor: usize,
    pub saved_programs: Vec<ProgramInfo>,
    pub program_list_selected: usize,
    pub editing_program_index: Option<usize>,
}

pub fn run() -> io::Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Initialize app state
    let mut app_state = AppState {
        cpu: CPU::new(),
        work_memory: WorkMemory::new(65536),
        should_quit: false,
        mode: AppMode::StartMenu,
        input_buffer: String::new(),
        command_queue: VecDeque::new(),
        last_message: None,
        message_is_error: false,
        cursor_position: 0,
        selected_menu_item: 0,
        show_command_queue: true,
        show_call_stack: true,
        settings_menu_active: false,
        settings_selected_item: 0,
        loaded_program: None,
        loaded_program_size: 0,
        editor_screen: false,
        editor_text: String::new(),
        editor_cursor_position: 0,
        editor_menu_selected: 0,
        current_file: None,
        show_help: false,
        editor_mode: EditorMode::Normal,
        program_name_input: String::new(),
        program_name_cursor: 0,
        saved_programs: Vec::new(),
        program_list_selected: 0,
        editing_program_index: None,
    };
    
    // Add some sample programs
    let commands1: Vec<Command> = vec![
        parse_command("MOVI AX, 10").unwrap(),
        parse_command("MOVI BX, 20").unwrap(),
        parse_command("ADDW AX, BX").unwrap(),
    ];
    let machine_code1 = command_processor::assemble_program(&commands1).unwrap();
    app_state.saved_programs.push(ProgramInfo {
        name: "Example 1".to_string(),
        content: "MOVI AX, 10\nMOVI BX, 20\nADDW AX, BX".to_string(),
        commands: commands1.into(),
        machine_code: machine_code1,
    });
    
    let commands2: Vec<Command> = vec![
        parse_command("MOVI CX, 5").unwrap(),
        parse_command("MOVI DX, 3").unwrap(),
        parse_command("SUBW CX, DX").unwrap(),
    ];
    let machine_code2 = command_processor::assemble_program(&commands2).unwrap();
    app_state.saved_programs.push(ProgramInfo {
        name: "Example 2".to_string(),
        content: "MOVI CX, 5\nMOVI DX, 3\nSUBW CX, DX".to_string(),
        commands: commands2.into(),
        machine_code: machine_code2,
    });
    
    // Main loop
    let result = run_app(&mut terminal, &mut app_state);
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    result
}



fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app_state: &mut AppState) -> io::Result<()> {
    while !app_state.should_quit {
        terminal.draw(|f| ui(f, app_state))?;
        handle_events(app_state)?;
        
        // Process instructions if in running mode
        if app_state.mode == AppMode::Running {
            match app_state.cpu.step(&mut app_state.work_memory) {
                Ok(_) => {
                    // Add delay between instructions
                    std::thread::sleep(Duration::from_millis(100));
                    
                    // Stop if we've reached the end of memory or a HLT instruction
                    if app_state.cpu.registers.pc >= app_state.work_memory.size as u32 {
                        app_state.mode = AppMode::Paused;
                        app_state.last_message = Some("Program execution completed".to_string());
                        app_state.message_is_error = false;
                    }
                },
                Err(e) => {
                    app_state.last_message = Some(e);
                    app_state.message_is_error = true;
                    app_state.mode = AppMode::Paused;
                }
            }
        }
        
        // Small delay to prevent high CPU usage
        std::thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

fn ui(f: &mut Frame, app_state: &AppState) {
    if app_state.show_help {
        f.render_widget(Clear, f.area());
        render_help_guide(f, centered_rect(80, 80, f.area()));
        return;
    }
    
    if app_state.editor_screen {
        match app_state.editor_mode {
            EditorMode::Naming => {
                f.render_widget(Clear, f.area());
                render_program_naming(f, f.area(), app_state);
                return;
            },
            EditorMode::LoadList => {
                f.render_widget(Clear, f.area());
                render_program_list(f, f.area(), app_state);
                return;
            },
            _ => {
                f.render_widget(Clear, f.area());
                render_program_editor(f, f.area(), app_state);
                return;
            }
        }
    }
    
    if app_state.settings_menu_active {
        f.render_widget(Clear, f.area());
        render_settings_menu(f, f.area(), app_state);
        return;
    }
    
    if app_state.mode == AppMode::StartMenu {
        f.render_widget(Clear, f.area());
        render_start_menu(f, f.area(), app_state);
        return;
    }
    
    // Main work interface
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Status bar
            Constraint::Min(10),    // Main content
        ])
        .split(f.area());
    
    // Render status bar in its dedicated area
    render_status_bar(f, main_chunks[0], app_state);
    
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70),  // CPU and memory
            Constraint::Percentage(15),  // Input
            Constraint::Percentage(15),  // Menu
        ])
        .split(main_chunks[1]);
    
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(content_chunks[0]);
    
    // Render CPU and memory panels
    render_cpu_panel(f, top_chunks[0], &app_state.cpu);
    render_memory_panel(f, top_chunks[1], &app_state.work_memory, app_state);
    
    // Input area
    render_input_panel(f, content_chunks[1], app_state);
    
    // Menu
    render_menu(f, content_chunks[2], app_state);
    
    // Message (if any)
    if let Some(message) = &app_state.last_message {
        let message_area = centered_rect(60, 20, f.area());
        let color = if app_state.message_is_error { Color::Red } else { Color::Green };
        let title = if app_state.message_is_error { "Error" } else { "Info" };
        let message_paragraph = Paragraph::new(message.as_str())
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(Clear, message_area);
        f.render_widget(message_paragraph, message_area);
    }
}

fn handle_events(app_state: &mut AppState) -> io::Result<()> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app_state.should_quit = true,
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => app_state.should_quit = true,
                    
                    // Start menu navigation
                    KeyCode::Up if app_state.mode == AppMode::StartMenu && !app_state.settings_menu_active && !app_state.editor_screen => {
                        if app_state.selected_menu_item > 0 {
                            app_state.selected_menu_item -= 1;
                        }
                    },
                    KeyCode::Down if app_state.mode == AppMode::StartMenu && !app_state.settings_menu_active && !app_state.editor_screen => {
                        if app_state.selected_menu_item < 3 {
                            app_state.selected_menu_item += 1;
                        }
                    },
                    
                    // Start menu selection
                    KeyCode::Enter if app_state.mode == AppMode::StartMenu && !app_state.settings_menu_active && !app_state.editor_screen => {
                        match app_state.selected_menu_item {
                            0 => {
                                // Start Emulator
                                app_state.mode = AppMode::Paused;
                                
                                if app_state.loaded_program.is_some() {
                                    app_state.cpu.registers.set(&Reg::PC, 0).unwrap();
                                    app_state.last_message = Some(format!("Program '{}' loaded. Press 'Run All' to start.", 
                                        app_state.loaded_program.as_ref().unwrap()));
                                    app_state.message_is_error = false;
                                } else {
                                    app_state.last_message = Some("No program loaded. Use command mode to enter instructions.".to_string());
                                    app_state.message_is_error = false;
                                }
                            },
                            1 => {
                                // Load Program - open editor in load list mode
                                app_state.editor_screen = true;
                                app_state.editor_mode = EditorMode::LoadList;
                                app_state.program_list_selected = 0;
                            },
                            2 => {
                                // Settings
                                app_state.settings_menu_active = true;
                                app_state.settings_selected_item = 0;
                            },
                            3 => {
                                // Quit
                                app_state.should_quit = true;
                            },
                            _ => {}
                        }
                    },
                    
                    // Settings menu navigation
                    KeyCode::Up if app_state.settings_menu_active => {
                        if app_state.settings_selected_item > 0 {
                            app_state.settings_selected_item -= 1;
                        }
                    },
                    KeyCode::Down if app_state.settings_menu_active => {
                        if app_state.settings_selected_item < 4 {
                            app_state.settings_selected_item += 1;
                        }
                    },
                    
                    // Settings menu selection
                    KeyCode::Enter if app_state.settings_menu_active => {
                        match app_state.settings_selected_item {
                            0 => {
                                // Reset all states
                                app_state.command_queue.clear();
                                app_state.cpu = CPU::new();
                                app_state.work_memory = WorkMemory::new(65536);
                                app_state.last_message = None;
                                app_state.input_buffer.clear();
                                app_state.cursor_position = 0;
                                app_state.mode = AppMode::Paused;
                                app_state.settings_menu_active = false;
                            },
                            1 => {
                                // Toggle command queue visibility
                                app_state.show_command_queue = !app_state.show_command_queue;
                            },
                            2 => {
                                // Toggle call stack visibility
                                app_state.show_call_stack = !app_state.show_call_stack;
                            },
                            3 => {
                                // Show help guide
                                app_state.show_help = true;
                            },
                            4 => {
                                // Back to start menu
                                app_state.settings_menu_active = false;
                            },
                            _ => {}
                        }
                    },
                    
                    // Editor navigation (normal mode)
                    KeyCode::Left if app_state.editor_screen && app_state.editor_mode == EditorMode::Normal => {
                        if app_state.editor_menu_selected > 0 {
                            app_state.editor_menu_selected -= 1;
                        }
                    },
                    KeyCode::Right if app_state.editor_screen && app_state.editor_mode == EditorMode::Normal => {
                        if app_state.editor_menu_selected < 3 {
                            app_state.editor_menu_selected += 1;
                        }
                    },
                    
                    // Editor menu selection (normal mode)
                    KeyCode::Enter if app_state.editor_screen && app_state.editor_mode == EditorMode::Normal => {
                        match app_state.editor_menu_selected {
                            0 => create_new_program(app_state),
                            1 => load_program_list(app_state),
                            2 => save_program_prompt(app_state),
                            3 => exit_editor(app_state),
                            _ => {}
                        }
                    },
                    
                    // Switch to edit mode
                    KeyCode::Char('i') if app_state.editor_screen && app_state.editor_mode == EditorMode::Normal => {
                        app_state.editor_mode = EditorMode::Edit;
                    },
                    
                    // Switch to normal mode from edit mode
                    KeyCode::Esc if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit => {
                        app_state.editor_mode = EditorMode::Normal;
                    },
                    
                    // Editor text input (edit mode)
                    KeyCode::Char(c) if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit => {
                        app_state.editor_text.insert(app_state.editor_cursor_position, c);
                        app_state.editor_cursor_position += 1;
                    },
                    
                    KeyCode::Backspace if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit && app_state.editor_cursor_position > 0 => {
                        app_state.editor_text.remove(app_state.editor_cursor_position - 1);
                        app_state.editor_cursor_position -= 1;
                    },
                    
                    KeyCode::Delete if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit && app_state.editor_cursor_position < app_state.editor_text.len() => {
                        app_state.editor_text.remove(app_state.editor_cursor_position);
                    },
                    
                    KeyCode::Left if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit && app_state.editor_cursor_position > 0 => {
                        app_state.editor_cursor_position -= 1;
                    },
                    
                    KeyCode::Right if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit && app_state.editor_cursor_position < app_state.editor_text.len() => {
                        app_state.editor_cursor_position += 1;
                    },
                    
                    KeyCode::Up if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit => {
                        // Move cursor up one line
                        let current_line_start = app_state.editor_text[..app_state.editor_cursor_position]
                            .rfind('\n')
                            .map(|pos| pos + 1)
                            .unwrap_or(0);
                        
                        if current_line_start > 0 {
                            let prev_line_end = app_state.editor_text[..current_line_start - 1]
                                .rfind('\n')
                                .map(|pos| pos + 1)
                                .unwrap_or(0);
                            
                            let current_column = app_state.editor_cursor_position - current_line_start;
                            let prev_line_len = current_line_start - 1 - prev_line_end;
                            
                            app_state.editor_cursor_position = prev_line_end + current_column.min(prev_line_len);
                        }
                    },
                    
                    KeyCode::Down if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit => {
                        // Move cursor down one line
                        let current_line_start = app_state.editor_text[..app_state.editor_cursor_position]
                            .rfind('\n')
                            .map(|pos| pos + 1)
                            .unwrap_or(0);
                        
                        let next_line_start = app_state.editor_text[app_state.editor_cursor_position..]
                            .find('\n')
                            .map(|pos| app_state.editor_cursor_position + pos + 1)
                            .unwrap_or(app_state.editor_text.len());
                        
                        if next_line_start < app_state.editor_text.len() {
                            let next_line_end = app_state.editor_text[next_line_start..]
                                .find('\n')
                                .map(|pos| next_line_start + pos)
                                .unwrap_or(app_state.editor_text.len());
                            
                            let current_column = app_state.editor_cursor_position - current_line_start;
                            let next_line_len = next_line_end - next_line_start;
                            
                            app_state.editor_cursor_position = next_line_start + current_column.min(next_line_len);
                        } else {
                            app_state.editor_cursor_position = app_state.editor_text.len();
                        }
                    },
                    
                    KeyCode::Enter if app_state.editor_screen && app_state.editor_mode == EditorMode::Edit => {
                        app_state.editor_text.insert(app_state.editor_cursor_position, '\n');
                        app_state.editor_cursor_position += 1;
                    },
                    
                    // Program naming input
                    KeyCode::Char(c) if app_state.editor_screen && app_state.editor_mode == EditorMode::Naming => {
                        app_state.program_name_input.insert(app_state.program_name_cursor, c);
                        app_state.program_name_cursor += 1;
                    },
                    
                    KeyCode::Backspace if app_state.editor_screen && app_state.editor_mode == EditorMode::Naming && app_state.program_name_cursor > 0 => {
                        app_state.program_name_input.remove(app_state.program_name_cursor - 1);
                        app_state.program_name_cursor -= 1;
                    },
                    
                    KeyCode::Left if app_state.editor_screen && app_state.editor_mode == EditorMode::Naming && app_state.program_name_cursor > 0 => {
                        app_state.program_name_cursor -= 1;
                    },
                    
                    KeyCode::Right if app_state.editor_screen && app_state.editor_mode == EditorMode::Naming && app_state.program_name_cursor < app_state.program_name_input.len() => {
                        app_state.program_name_cursor += 1;
                    },
                    
                    KeyCode::Enter if app_state.editor_screen && app_state.editor_mode == EditorMode::Naming => {
                        if app_state.program_name_input.trim().is_empty() {
                            app_state.last_message = Some("Program name cannot be empty".to_string());
                            app_state.message_is_error = true;
                        } else {
                            save_program_with_name(app_state);
                            app_state.editor_mode = EditorMode::Normal;
                        }
                    },
                    
                    KeyCode::Esc if app_state.editor_screen && app_state.editor_mode == EditorMode::Naming => {
                        app_state.editor_mode = EditorMode::Normal;
                    },
                    
                    // Program list navigation
                    KeyCode::Up if app_state.editor_screen && app_state.editor_mode == EditorMode::LoadList => {
                        if app_state.program_list_selected > 0 {
                            app_state.program_list_selected -= 1;
                        }
                    },
                    
                    KeyCode::Down if app_state.editor_screen && app_state.editor_mode == EditorMode::LoadList => {
                        if app_state.program_list_selected < app_state.saved_programs.len() - 1 {
                            app_state.program_list_selected += 1;
                        }
                    },
                    
                    KeyCode::Enter if app_state.editor_screen && app_state.editor_mode == EditorMode::LoadList => {
                        load_selected_program(app_state);
                    },
                    
                    KeyCode::Char('e') if app_state.editor_screen && app_state.editor_mode == EditorMode::LoadList => {
                        edit_selected_program(app_state);
                    },
                    
                    KeyCode::Char('r') if app_state.editor_screen && app_state.editor_mode == EditorMode::LoadList => {
                        rename_selected_program(app_state);
                    },
                    
                    KeyCode::Char('d') if app_state.editor_screen && app_state.editor_mode == EditorMode::LoadList => {
                        delete_selected_program(app_state);
                    },
                    
                    KeyCode::Esc if app_state.editor_screen && app_state.editor_mode == EditorMode::LoadList => {
                        app_state.editor_mode = EditorMode::Normal;
                    },
                    
                    // ESC to return to start menu from work environment
                    KeyCode::Esc if !app_state.settings_menu_active && app_state.mode != AppMode::StartMenu && !app_state.editor_screen && !app_state.show_help => {
                        app_state.mode = AppMode::StartMenu;
                        app_state.selected_menu_item = 0;
                    },
                    
                    // ESC to exit settings menu
                    KeyCode::Esc if app_state.settings_menu_active => {
                        app_state.settings_menu_active = false;
                    },
                    
                    // ESC to exit editor
                    KeyCode::Esc if app_state.editor_screen && app_state.editor_mode == EditorMode::Normal => {
                        app_state.editor_screen = false;
                    },
                    
                    // Exit help guide
                    KeyCode::Esc if app_state.show_help => {
                        app_state.show_help = false;
                    },
                    KeyCode::Char(_) if app_state.show_help => {
                        app_state.show_help = false;
                    },
                    
                    // Work menu navigation
                    KeyCode::Left if app_state.mode != AppMode::CommandMode && !app_state.settings_menu_active && !app_state.editor_screen => {
                        if app_state.selected_menu_item > 0 {
                            app_state.selected_menu_item -= 1;
                        }
                    },
                    KeyCode::Right if app_state.mode != AppMode::CommandMode && !app_state.settings_menu_active && !app_state.editor_screen => {
                        if app_state.selected_menu_item < 4 {
                            app_state.selected_menu_item += 1;
                        }
                    },
                    
                    // Work menu selection
                    KeyCode::Enter if app_state.mode != AppMode::CommandMode && !app_state.settings_menu_active && !app_state.editor_screen => {
                        match app_state.selected_menu_item {
                            0 => {
                                if app_state.loaded_program.is_some() {
                                    app_state.cpu.registers.set(&Reg::PC, 0).unwrap();
                                    app_state.mode = AppMode::Running;
                                } else {
                                    app_state.last_message = Some("No program loaded to run.".to_string());
                                    app_state.message_is_error = true;
                                }
                            },
                            1 => {
                                if let Some(command) = app_state.command_queue.pop_front() {
                                    if let Err(e) = execute_command(command, &mut app_state.cpu, &mut app_state.work_memory) {
                                        app_state.last_message = Some(e);
                                        app_state.message_is_error = true;
                                    }
                                }
                            },
                            2 => { app_state.command_queue.pop_front(); },
                            3 => { app_state.command_queue.pop_back(); },
                            4 => app_state.should_quit = true,
                            _ => {}
                        }
                    },
                    
                    // Command input
                    KeyCode::Char(c) if app_state.mode == AppMode::CommandMode => {
                        app_state.input_buffer.push(c);
                        app_state.cursor_position = app_state.input_buffer.len();
                    },

                    KeyCode::Backspace if app_state.mode == AppMode::CommandMode && app_state.cursor_position > 0 => {
                        app_state.input_buffer.remove(app_state.cursor_position - 1);
                        app_state.cursor_position -= 1;
                    },

                    KeyCode::Enter if app_state.mode == AppMode::CommandMode => {
                        match parse_command(&app_state.input_buffer) {
                            Ok(command) => {
                                app_state.command_queue.push_back(command);
                                app_state.input_buffer.clear();
                                app_state.cursor_position = 0;
                            },
                            Err(e) => {
                                app_state.last_message = Some(e);
                                app_state.message_is_error = true;
                            }
                        }
                    },
                    
                    // Mode switching
                    KeyCode::Tab => {
                        if app_state.last_message.is_some() {
                            // Dismiss message
                            app_state.last_message = None;
                        } else if app_state.mode == AppMode::CommandMode {
                            app_state.mode = AppMode::Paused;
                        } else if app_state.mode == AppMode::Paused {
                            app_state.mode = AppMode::CommandMode;
                        }
                        // Clear input buffer when switching to command mode
                        if app_state.mode == AppMode::CommandMode {
                            app_state.input_buffer.clear();
                            app_state.cursor_position = 0;
                        }
                    },
                    
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn create_new_program(app_state: &mut AppState) {
    app_state.editor_text.clear();
    app_state.editor_cursor_position = 0;
    app_state.current_file = None;
    app_state.editor_mode = EditorMode::Edit;
    app_state.editing_program_index = None;
    app_state.last_message = Some("New program created. Press 'i' to start editing.".to_string());
    app_state.message_is_error = false;
}

fn load_program_list(app_state: &mut AppState) {
    app_state.editor_mode = EditorMode::LoadList;
    app_state.program_list_selected = 0;
}

fn save_program_prompt(app_state: &mut AppState) {
    if app_state.editor_text.is_empty() {
        app_state.last_message = Some("Cannot save empty program".to_string());
        app_state.message_is_error = true;
        return;
    }
    
    app_state.program_name_input.clear();
    app_state.program_name_cursor = 0;
    
    // If we're editing an existing program, pre-fill the name
    if let Some(index) = app_state.editing_program_index {
        app_state.program_name_input = app_state.saved_programs[index].name.clone();
        app_state.program_name_cursor = app_state.program_name_input.len();
    }
    
    app_state.editor_mode = EditorMode::Naming;
}

fn save_program_with_name(app_state: &mut AppState) {
    let program_name = app_state.program_name_input.trim().to_string();
    
    // Parse the program text into commands
    let lines: Vec<&str> = app_state.editor_text.lines().collect();
    let mut commands = VecDeque::new();
    
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        if let Ok(command) = parse_command(trimmed_line) {
            commands.push_back(command);
        }
    }

    let commands_vec: Vec<Command> = commands.clone().into();
    let machine_code = match command_processor::assemble_program(&commands_vec) {
        Ok(code) => code,
        Err(e) => {
            app_state.last_message = Some(e);
            app_state.message_is_error = true;
            return;
        }
    };
    
    // Check if we already have a program with this name (and it's not the one we're editing)
    if let Some(existing_index) = app_state.saved_programs.iter().position(|p| p.name == program_name) {
        if app_state.editing_program_index != Some(existing_index) {
            app_state.last_message = Some(format!("A program with name '{}' already exists", program_name));
            app_state.message_is_error = true;
            return;
        }
    }
    
    if let Some(index) = app_state.editing_program_index {
        // Update existing program
        app_state.saved_programs[index] = ProgramInfo {
            name: program_name.clone(),
            content: app_state.editor_text.clone(),
            commands,
            machine_code,
        };
    } else {
        // Add new program
        app_state.saved_programs.push(ProgramInfo {
            name: program_name.clone(),
            content: app_state.editor_text.clone(),
            commands,
            machine_code,
        });
    }
    
    app_state.loaded_program = Some(program_name.clone());
    app_state.loaded_program_size = app_state.editor_text.len();
    app_state.last_message = Some(format!("Program '{}' saved and assembled successfully", program_name));
    app_state.message_is_error = false;
    app_state.editing_program_index = None;
}

fn load_selected_program(app_state: &mut AppState) {
    if app_state.saved_programs.is_empty() {
        app_state.last_message = Some("No programs available to load".to_string());
        app_state.message_is_error = true;
        return;
    }
    
    let selected_program = &app_state.saved_programs[app_state.program_list_selected];
    
    app_state.command_queue = selected_program.commands.clone();
    app_state.work_memory.load_program(0, &selected_program.machine_code).unwrap();
    
    app_state.loaded_program = Some(selected_program.name.clone());
    app_state.loaded_program_size = selected_program.content.len();
    app_state.editor_screen = false;
    app_state.last_message = Some(format!("Program '{}' loaded with {} commands", selected_program.name, selected_program.commands.len()));
    app_state.message_is_error = false;
}

fn edit_selected_program(app_state: &mut AppState) {
    if app_state.saved_programs.is_empty() {
        app_state.last_message = Some("No programs available to edit".to_string());
        app_state.message_is_error = true;
        return;
    }
    
    let selected_program = &app_state.saved_programs[app_state.program_list_selected];
    app_state.editor_text = selected_program.content.clone();
    app_state.editor_cursor_position = 0;
    app_state.editor_mode = EditorMode::Edit;
    app_state.editing_program_index = Some(app_state.program_list_selected);
    app_state.last_message = Some(format!("Editing program '{}'", selected_program.name));
    app_state.message_is_error = false;
}

fn rename_selected_program(app_state: &mut AppState) {
    if app_state.saved_programs.is_empty() {
        app_state.last_message = Some("No programs available to rename".to_string());
        app_state.message_is_error = true;
        return;
    }
    
    let selected_program = &app_state.saved_programs[app_state.program_list_selected];
    app_state.program_name_input = selected_program.name.clone();
    app_state.program_name_cursor = app_state.program_name_input.len();
    app_state.editor_mode = EditorMode::Naming;
    app_state.editing_program_index = Some(app_state.program_list_selected);
}

fn delete_selected_program(app_state: &mut AppState) {
    if app_state.saved_programs.is_empty() {
        app_state.last_message = Some("No programs available to delete".to_string());
        app_state.message_is_error = true;
        return;
    }
    
    let program_name = app_state.saved_programs[app_state.program_list_selected].name.clone();
    app_state.saved_programs.remove(app_state.program_list_selected);
    
    // Adjust selection if needed
    if app_state.program_list_selected >= app_state.saved_programs.len() {
        app_state.program_list_selected = app_state.saved_programs.len().saturating_sub(1);
    }
    
    app_state.last_message = Some(format!("Program '{}' deleted", program_name));
    app_state.message_is_error = false;
}

fn exit_editor(app_state: &mut AppState) {
    app_state.editor_screen = false;
    app_state.editor_mode = EditorMode::Normal;
    app_state.editing_program_index = None;
}

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
