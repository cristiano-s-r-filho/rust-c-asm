use crate::memory::registers::Reg;
use std::collections::VecDeque;
use std::io;
use std::time::Duration;
use std::fs;
use std::path::Path;

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
use crate::memory::main_memory::{WorkMemory, TEXT_START, DATA_START};
use crate::chips::io_device::IoDevice;
use crate::utils::command_processor::{self, parse_command, Command, AssembledProgram, Macro};
use crate::utils::widgets::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppMode {
    StartMenu,
    CommandMode,
    Running,
    Paused,
    IoScreen,
    IoInput,
    MacroDefinition,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub assembled_program: AssembledProgram,
}

#[derive(Debug)]
pub struct AppState {
    pub cpu: CPU,
    pub work_memory: WorkMemory,
    pub io_device: IoDevice,
    pub macros: Vec<Macro>,
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
    pub parsing_errors: Vec<String>,
    pub help_scroll: u16,
}

pub fn run(memory_size: usize, program_content: Option<String>) -> io::Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut initial_mode = AppMode::StartMenu;
    let mut initial_message: Option<String> = None;
    let mut initial_command_queue = VecDeque::new();
    let mut initial_loaded_program: Option<String> = None;
    let mut initial_loaded_program_size: usize = 0;
    let mut initial_parsing_errors: Vec<String> = Vec::new();
    let mut initial_assembled_program: Option<AssembledProgram> = None;

    if let Some(content) = program_content {
        let lines: Vec<&str> = content.lines().collect();
        let mut commands = VecDeque::new();
        let mut current_parsing_errors = Vec::new();

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            match parse_command(trimmed_line) {
                Ok(command) => commands.push_back(command),
                Err(e) => current_parsing_errors.push(format!("Line {}: {}", line_num + 1, e)),
            }
        }

        if !current_parsing_errors.is_empty() {
            initial_message = Some(format!("Parsing errors:\n{}", current_parsing_errors.join("\n")));
            initial_parsing_errors = current_parsing_errors;
        } else {
            let commands_vec: Vec<Command> = commands.clone().into();
            match command_processor::assemble_program(&commands_vec, &Vec::new()) {
                Ok(assembled_program) => {
                    initial_mode = AppMode::Paused;
                    initial_message = Some("Program loaded from file. Press 'Run All' to start.".to_string());
                    initial_command_queue = commands;
                    initial_loaded_program = Some("File Program".to_string());
                    initial_loaded_program_size = content.len();
                    initial_assembled_program = Some(assembled_program);
                },
                Err(e) => {
                    initial_message = Some(format!("Assembly failed: {}", e));
                }
            }
        }
    }

    // Initialize app state
    let mut app_state = AppState {
        cpu: CPU::new(),
        work_memory: WorkMemory::new(memory_size),
        io_device: IoDevice::new(),
        macros: Vec::new(),
        should_quit: false,
        mode: initial_mode,
        last_message: initial_message.clone(),
        message_is_error: initial_message.is_some() && initial_loaded_program.is_none(),
        input_buffer: String::new(),
        command_queue: initial_command_queue,
        loaded_program: initial_loaded_program,
        loaded_program_size: initial_loaded_program_size,
        cursor_position: 0,
        selected_menu_item: 0,
        show_command_queue: true,
        show_call_stack: true,
        settings_menu_active: false,
        settings_selected_item: 0,
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
        parsing_errors: initial_parsing_errors,
        help_scroll: 0,
    };

    if let Some(assembled_program) = initial_assembled_program {
        if app_state.work_memory.load_program(TEXT_START, &assembled_program.text).is_err() {
            app_state.last_message = Some("Failed to load text section into memory.".to_string());
            app_state.message_is_error = true;
        }
        if app_state.work_memory.load_data(DATA_START, &assembled_program.data).is_err() {
            app_state.last_message = Some("Failed to load data section into memory.".to_string());
            app_state.message_is_error = true;
        }
    }
    
    if !Path::new("programs").exists() {
        fs::create_dir("programs")?;
    }

    for entry in fs::read_dir("programs")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("arc") {
            let content = fs::read_to_string(&path)?;
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let lines: Vec<&str> = content.lines().collect();
            let mut commands = VecDeque::new();
            for line in lines {
                if let Ok(command) = parse_command(line) {
                    commands.push_back(command);
                }
            }
            let commands_vec: Vec<Command> = commands.clone().into();
            let assembled_program = command_processor::assemble_program(&commands_vec, &Vec::new()).unwrap();
            app_state.saved_programs.push(ProgramInfo {
                name,
                content,
                commands,
                assembled_program,
            });
        }
    }
    
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
                    if app_state.cpu.halted {
                        app_state.mode = AppMode::Paused;
                        app_state.last_message = Some("Program execution halted. Press Esc to return to the main menu.".to_string());
                        app_state.message_is_error = false;            } else {
                match app_state.cpu.step(&mut app_state.work_memory) {
                    Ok(_) => {
                        // Add delay between instructions
                        std::thread::sleep(Duration::from_millis(100));
                        
                        // Stop if we've reached the end of memory
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
        }
        
        // Small delay to prevent high CPU usage
        std::thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

fn ui(f: &mut Frame, app_state: &AppState) {
    if app_state.show_help {
        f.render_widget(Clear, f.area());
        render_help_guide(f, centered_rect(80, 80, f.area()), app_state);
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
    
    if app_state.mode == AppMode::IoScreen {
        f.render_widget(Clear, f.area());
        render_io_panel(f, f.area(), app_state);
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
    
    // Render CPU and code panels
    render_cpu_panel(f, top_chunks[0], &app_state.cpu);
    render_code_panel(f, top_chunks[1], app_state);
    
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
                        if app_state.settings_selected_item < 5 {
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
                                // I/O Screen
                                app_state.mode = AppMode::IoScreen;
                                app_state.settings_menu_active = false;
                            },
                            4 => {
                                // Show help guide
                                app_state.show_help = true;
                            },
                            5 => {
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
                            2 => {
                                app_state.editor_mode = EditorMode::Naming;
                                app_state.program_name_input.clear();
                                app_state.program_name_cursor = 0;
                            },
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
                    
                    KeyCode::Up if app_state.show_help => {
                        if app_state.help_scroll > 0 {
                            app_state.help_scroll -= 1;
                        }
                    },
                    KeyCode::Down if app_state.show_help => {
                        app_state.help_scroll += 1;
                    },

                    // Exit help guide
                    KeyCode::Up if app_state.show_help => {
                        if app_state.help_scroll > 0 {
                            app_state.help_scroll -= 1;
                        }
                    },
                    KeyCode::Down if app_state.show_help => {
                        app_state.help_scroll += 1;
                    },
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
                                    app_state.cpu.registers.set(&Reg::PC, TEXT_START).unwrap();
                                    app_state.mode = AppMode::Running;
                                } else {
                                    app_state.last_message = Some("No program loaded to run.".to_string());
                                    app_state.message_is_error = true;
                                }
                            },
                            1 => {
                                if let Some(command) = app_state.command_queue.pop_front() {
                                    if let Err(e) = command_processor::execute_command(command, &mut app_state.cpu, &mut app_state.work_memory, &mut app_state.io_device) {
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
                                if app_state.mode == AppMode::MacroDefinition {
                                    if command.opcode == ".endmacro" {
                                        app_state.mode = AppMode::Paused;
                                        let new_macro = Macro {
                                            name: app_state.macros.last().unwrap().name.clone(),
                                            args: app_state.macros.last().unwrap().args.clone(),
                                            body: app_state.macros.last().unwrap().body.clone(),
                                        };
                                        app_state.macros.pop();
                                        app_state.macros.push(new_macro);
                                    } else {
                                        app_state.macros.last_mut().unwrap().body.push(command);
                                    }
                                } else if command.opcode == ".macro" {
                                    app_state.mode = AppMode::MacroDefinition;
                                    let name = command.macro_name.unwrap_or_default();
                                    let args = command.macro_args.unwrap_or_default();
                                    app_state.macros.push(Macro {
                                        name,
                                        args,
                                        body: Vec::new(),
                                    });
                                } else {
                                    app_state.command_queue.push_back(command);
                                }
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
                        } else if app_state.mode == AppMode::IoScreen {
                            app_state.mode = AppMode::Paused;
                        }
                        // Clear input buffer when switching to command mode
                        if app_state.mode == AppMode::CommandMode {
                            app_state.input_buffer.clear();
                            app_state.cursor_position = 0;
                        }
                    },

                    // IO Screen button presses
                    KeyCode::Up if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.up = true,
                    KeyCode::Down if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.down = true,
                    KeyCode::Left if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.left = true,
                    KeyCode::Right if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.right = true,
                    KeyCode::Char('z') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.circle = true,
                    KeyCode::Char('x') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.triangle = true,
                    KeyCode::Char('c') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.square = true,
                    KeyCode::Char('v') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.cross = true,
                    KeyCode::Char('i') if app_state.mode == AppMode::IoScreen => app_state.mode = AppMode::IoInput,

                    // IO Screen input
                    KeyCode::Char(c) if app_state.mode == AppMode::IoInput => app_state.io_device.input_buffer.push(c),
                    KeyCode::Backspace if app_state.mode == AppMode::IoInput => { app_state.io_device.input_buffer.pop(); },
                    KeyCode::Enter if app_state.mode == AppMode::IoInput => app_state.mode = AppMode::IoScreen,
                    
                    _ => {}
                }
            } else if key.kind == KeyEventKind::Release {
                match key.code {
                    // IO Screen button releases
                    KeyCode::Up if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.up = false,
                    KeyCode::Down if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.down = false,
                    KeyCode::Left if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.left = false,
                    KeyCode::Right if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.right = false,
                    KeyCode::Char('z') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.circle = false,
                    KeyCode::Char('x') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.triangle = false,
                    KeyCode::Char('c') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.square = false,
                    KeyCode::Char('v') if app_state.mode == AppMode::IoScreen => app_state.io_device.button_state.cross = false,
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
    app_state.saved_programs.clear();
    for entry in fs::read_dir("programs").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("arc") {
            let content = fs::read_to_string(&path).unwrap();
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let lines: Vec<&str> = content.lines().collect();
            let mut commands = VecDeque::new();
            for line in lines {
                if let Ok(command) = parse_command(line) {
                    commands.push_back(command);
                }
            }
            let commands_vec: Vec<Command> = commands.clone().into();
            let assembled_program = command_processor::assemble_program(&commands_vec, &Vec::new()).unwrap();
            app_state.saved_programs.push(ProgramInfo {
                name,
                content,
                commands,
                assembled_program,
            });
        }
    }
    app_state.editor_mode = EditorMode::LoadList;
    app_state.program_list_selected = 0;
}

fn save_program_with_name(app_state: &mut AppState) {
    let program_name = app_state.program_name_input.trim().to_string();
    let file_path = Path::new("programs").join(format!("{}.arc", program_name));

    if let Err(e) = fs::write(&file_path, &app_state.editor_text) {
        app_state.last_message = Some(format!("Failed to save program: {}", e));
        app_state.message_is_error = true;
        return;
    }
}

fn load_selected_program(app_state: &mut AppState) {
    if app_state.saved_programs.is_empty() {
        app_state.last_message = Some("No programs available to load".to_string());
        app_state.message_is_error = true;
        return;
    }
    
    let selected_program = &app_state.saved_programs[app_state.program_list_selected];
    
    // Reset CPU and memory for the new program
    app_state.cpu.reset();
    app_state.work_memory = WorkMemory::new(app_state.work_memory.size);

    app_state.command_queue = selected_program.commands.clone();
    app_state.work_memory.load_program(TEXT_START, &selected_program.assembled_program.text).unwrap();
    app_state.work_memory.load_data(DATA_START, &selected_program.assembled_program.data).unwrap();
    
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