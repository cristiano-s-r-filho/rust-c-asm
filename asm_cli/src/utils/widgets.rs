use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use ratatui::layout::Alignment;

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::tui::{AppState, EditorMode, AppMode};

pub fn render_cpu_panel(f: &mut Frame, area: Rect, cpu: &CPU) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),  // General Purpose (increased from 6)
            Constraint::Length(7),  // Segment (increased from 5)
            Constraint::Length(7),  // Stack/Pointer (increased from 5)
            Constraint::Length(6),  // Control (increased from 4)
        ])
        .split(area);

    // General Purpose Registers - now with red color
    let general_purpose = format!(
        "AX: {:#010x}    BX: {:#010x}\n\
         CX: {:#010x}    DX: {:#010x}",
        cpu.registers.ax, cpu.registers.bx, cpu.registers.cx, cpu.registers.dx
    );
    let general_paragraph = Paragraph::new(general_purpose)
        .block(Block::default().borders(Borders::ALL).title("General Purpose"))
        .style(Style::default().fg(Color::Red));  // Changed to red
    f.render_widget(general_paragraph, chunks[0]);

    // Segment Registers - now with red color
    let segment = format!(
        "CS: {:#06x}    DS: {:#06x}\n\
         SS: {:#06x}    ES: {:#06x}",
        cpu.registers.cs, cpu.registers.ds, cpu.registers.ss, cpu.registers.es
    );
    let segment_paragraph = Paragraph::new(segment)
        .block(Block::default().borders(Borders::ALL).title("Segment"))
        .style(Style::default().fg(Color::Blue));  // Changed to red
    f.render_widget(segment_paragraph, chunks[1]);

    // Stack/Pointer Registers - now with red color
    let stack = format!(
        "SP: {:#010x}    BP: {:#010x}\n\
         SI: {:#010x}    DI: {:#010x}",
        cpu.registers.sp, cpu.registers.bp, cpu.registers.si, cpu.registers.di
    );
    let stack_paragraph = Paragraph::new(stack)
        .block(Block::default().borders(Borders::ALL).title("Stack/Pointer"))
        .style(Style::default().fg(Color::Yellow));  // Changed to red
    f.render_widget(stack_paragraph, chunks[2]);

    // Control Registers - now with red color
    let control = format!(
        "PC: {:#010x}    FLAGS: {:#010x}",
        cpu.registers.pc, cpu.registers.flags
    );
    let control_paragraph = Paragraph::new(control)
        .block(Block::default().borders(Borders::ALL).title("Control"))
        .style(Style::default().fg(Color::Green));  // Changed to red
    f.render_widget(control_paragraph, chunks[3]);
}

pub fn render_memory_panel(f: &mut Frame, area: Rect, memory: &WorkMemory, _app_state: &AppState) {
    let mut content = String::new();
    
    // Show memory around the stack pointer
    let sp = memory.get_stack_pointer();
    let start_addr = sp.saturating_sub(32);
    
    for i in 0..16 {
        let addr = start_addr + i * 4;
        if addr as usize + 3 < memory.size {
            if let Ok(value) = memory.read(addr) {
                content.push_str(&format!("{:#010x}: {:#010x}", addr, value));
                if addr == sp {
                    content.push_str("  ← SP");
                }
                content.push('\n');
            }
        }
    }
    
    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Memory"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(paragraph, area);
}

pub fn render_menu(f: &mut Frame, area: Rect, app_state: &AppState) {
    let menu_items = vec![
        "▶ Run All", 
        "⏩ Execute Next", 
        "🗑️ Delete Top", 
        "🗑️ Delete Bottom", 
        "⏏ Quit"
    ];
    
    let items: Vec<Span> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app_state.selected_menu_item {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            Span::styled(*item, style)
        })
        .collect();
    
    // Create a horizontal line with spacing
    let mut line_content = Vec::new();
    for (i, span) in items.iter().enumerate() {
        if i > 0 {
            line_content.push(Span::raw(" | "));
        }
        line_content.push(span.clone());
    }
    
    let menu = Paragraph::new(Line::from(line_content))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Work Menu (←/→/Enter)"))
        .alignment(Alignment::Center);
    
    f.render_widget(menu, area);
}

pub fn render_input_panel(f: &mut Frame, area: Rect, app_state: &AppState) {
    let input_text = if app_state.mode == AppMode::CommandMode {
        format!("> {}", app_state.input_buffer)
    } else {
        "Press TAB to enter command mode".to_string()
    };
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Input")
        .style(match app_state.mode {
            AppMode::CommandMode => Style::default().fg(Color::Green),
            _ => Style::default().fg(Color::Gray),
        });
    
    let input_paragraph = Paragraph::new(input_text)
        .block(input_block)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(input_paragraph, area);
    
    // Show cursor in command mode
    if app_state.mode == AppMode::CommandMode {
        f.set_cursor_position((area.x + 2 + app_state.cursor_position as u16, area.y + 1));
    }
}

pub fn render_status_bar(f: &mut Frame, area: Rect, app_state: &AppState) {
    let status = match app_state.mode {
        AppMode::StartMenu => "START MENU - Use arrow keys and Enter to navigate",
        AppMode::CommandMode => "COMMAND MODE - Type commands and press Enter",
        AppMode::Running => "RUNNING - Executing commands",
        AppMode::Paused => "PAUSED - Use menu to control execution",
    };
    
    let status_bar = Paragraph::new(status)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    
    f.render_widget(status_bar, area);
}

pub fn render_start_menu(f: &mut Frame, area: Rect, app_state: &AppState) {
    // Split the area into three sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),  // Top section (art)
            Constraint::Percentage(40),  // Middle section (info)
            Constraint::Percentage(30),  // Bottom section (menu)
        ])
        .split(area);
    
    // Top section with ANSI art
    let art = r#"
     █████╗ ██████╗  ██████╗     ██████╗ 
    ██╔══██╗██╔══██╗██╔════╝    ██╔═══██╗
    ███████║██████╔╝██║         ██║   ██║
    ██╔══██║██╔══██╗██║         ██║   ██║
    ██║  ██║██║  ██║╚██████╗    ╚██████╔╝
    ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝     ╚═════╝ 
    "#;
    
    let art_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::Cyan));
    
    let art_paragraph = Paragraph::new(art)
        .block(art_block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));
    
    f.render_widget(art_paragraph, chunks[0]);
    
    // Middle section with info
    let version_info = "ARC-0 Assembly Emulator v0.1.0";
    let repo_info = "GitHub: https://github.com/yourusername/arc0-emulator";
    
    let program_info = if app_state.loaded_program.is_some() {
        format!("Loaded: {} ({} commands, {:.1} KiB)", 
                app_state.loaded_program.as_ref().unwrap(),
                app_state.command_queue.len(),
                app_state.loaded_program_size as f32 / 1024.0)
    } else {
        "No program loaded".to_string()
    };
    
    let middle_art = r#"
    ┌──────────────────────────────────────┐
    │      ARC-0 Custom Assembly           │
    └──────────────────────────────────────┘
    "#;
    
    let info_text = format!("{}\n{}\n{}\n\n{}", version_info, repo_info, program_info, middle_art);
    
    let info_paragraph = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(info_paragraph, chunks[1]);
    
    // Bottom section with menu
    let menu_items = vec![
        "Start Emulator",
        "Load Program",
        "Settings",
        "Quit"
    ];
    
    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app_state.selected_menu_item {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            ListItem::new(*item).style(style)
        })
        .collect();
    
    let menu = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Main Menu")
            .style(Style::default().fg(Color::Cyan)))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    
    // Center the menu in the bottom section
    let menu_area = centered_rect(50, 70, chunks[2]);
    f.render_widget(menu, menu_area);
    
    // Add instructions at the bottom
    let instructions = Paragraph::new("Use ↑/↓ to navigate, Enter to select, ESC to return to this menu")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    
    let instruction_area = Rect::new(
        area.x,
        area.y + area.height - 1,
        area.width,
        1
    );
    
    f.render_widget(instructions, instruction_area);
}

pub fn render_settings_menu(f: &mut Frame, area: Rect, app_state: &AppState) {
    let menu_items = vec![
        "Reset All States",
        "Toggle Command Queue Visibility",
        "Toggle Call Stack Visibility",
        "Help Guide",
        "Back to Start Menu",
    ];
    
    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app_state.settings_selected_item {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            ListItem::new(*item).style(style)
        })
        .collect();
    
    let menu = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Settings")
            .style(Style::default().fg(Color::Cyan)))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    
    // Center the menu
    let area = centered_rect(50, 60, area);
    f.render_widget(menu, area);
}

pub fn render_program_editor(f: &mut Frame, area: Rect, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),  // Editor area
            Constraint::Length(3), // Editor menu
        ])
        .split(area);
    
    // Editor text area with line numbers
    let mode_indicator = match app_state.editor_mode {
        EditorMode::Normal => "[NORMAL]",
        EditorMode::Edit => "[EDIT]",
        EditorMode::EditingExisting => "[EDITING]",
        _ => "",
    };
    
    let editor_title = format!("ARC-0 Program Editor {}", mode_indicator);
    
    // Add line numbers to the editor content
    let text_with_line_numbers = add_line_numbers(&app_state.editor_text);
    
    let editor_paragraph = Paragraph::new(text_with_line_numbers)
        .block(Block::default().borders(Borders::ALL).title(editor_title))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(editor_paragraph, chunks[0]);
    
    // Editor menu (only show in normal mode)
    if app_state.editor_mode == EditorMode::Normal || app_state.editor_mode == EditorMode::EditingExisting {
        let menu_items = vec!["Create Program", "Load Program", "Save Program", "Exit Editor"];
        
        let items: Vec<Span> = menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == app_state.editor_menu_selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                
                Span::styled(*item, style)
            })
            .collect();
        
        let mut line_content = Vec::new();
        for (i, span) in items.iter().enumerate() {
            if i > 0 {
                line_content.push(Span::raw(" | "));
            }
            line_content.push(span.clone());
        }
        
        let menu = Paragraph::new(Line::from(line_content))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Editor Menu (←/→/Enter)"))
            .alignment(Alignment::Center);
        
        f.render_widget(menu, chunks[1]);
    } else {
        // Show instructions in edit mode
        let instructions = Paragraph::new("Press ESC to return to normal mode, Enter to add new line")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        
        f.render_widget(instructions, chunks[1]);
    }
    
    // Show cursor in edit mode - fixed positioning
    if app_state.editor_mode == EditorMode::Edit || app_state.editor_mode == EditorMode::EditingExisting {
        // Calculate the correct cursor position
        let (cursor_line, cursor_col) = calculate_cursor_position(&app_state.editor_text, app_state.editor_cursor_position);
        
        // Add 5 characters for line numbers (4 digits + space)
        let cursor_x = chunks[0].x + 1 + 5 + cursor_col as u16;
        let cursor_y = chunks[0].y + 1 + cursor_line as u16;
        
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

// Helper function to add line numbers to text
fn add_line_numbers(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let max_line_num_width = (lines.len() + 1).to_string().len().max(3);
    
    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        let line_num = format!("{:width$} ", i + 1, width = max_line_num_width);
        result.push_str(&line_num);
        result.push_str(line);
        result.push('\n');
    }
    
    // Remove the last newline if the text ends with one
    if text.ends_with('\n') {
        let line_num = format!("{:width$} ", lines.len() + 1, width = max_line_num_width);
        result.push_str(&line_num);
    }
    
    result
}

// Helper function to calculate cursor position (line and column)
fn calculate_cursor_position(text: &str, cursor_pos: usize) -> (usize, usize) {
    if cursor_pos == 0 {
        return (0, 0);
    }
    
    if cursor_pos > text.len() {
        return calculate_cursor_position(text, text.len());
    }
    
    let text_before_cursor = &text[..cursor_pos];
    let line = text_before_cursor.chars().filter(|&c| c == '\n').count();
    let last_newline = text_before_cursor.rfind('\n').map(|pos| pos + 1).unwrap_or(0);
    let column = cursor_pos - last_newline;
    
    (line, column)
}

pub fn render_program_naming(f: &mut Frame, area: Rect, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);
    
    let title = Paragraph::new("Name Your Program")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    
    f.render_widget(title, chunks[0]);
    
    let input_text = format!("> {}", app_state.program_name_input);
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Program Name")
        .style(Style::default().fg(Color::Green));
    
    let input_paragraph = Paragraph::new(input_text)
        .block(input_block)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(input_paragraph, chunks[1]);
    
    let instructions = Paragraph::new("Enter a name for your program and press Enter to save, or ESC to cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    
    f.render_widget(instructions, chunks[2]);
    
    // Show cursor
    f.set_cursor_position((chunks[1].x + 2 + app_state.program_name_cursor as u16, chunks[1].y + 1));
}

pub fn render_program_list(f: &mut Frame, area: Rect, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);
    
    let title = Paragraph::new("Select a Program")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    
    f.render_widget(title, chunks[0]);
    
    if app_state.saved_programs.is_empty() {
        let no_programs = Paragraph::new("No programs saved yet")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        
        f.render_widget(no_programs, chunks[0]);
    } else {
        let program_items: Vec<ListItem> = app_state.saved_programs
            .iter()
            .enumerate()
            .map(|(i, program)| {
                let style = if i == app_state.program_list_selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                
                let content = format!("{} ({} commands)", program.name, program.command_count);
                ListItem::new(content).style(style)
            })
            .collect();
        
        let program_list = List::new(program_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Saved Programs"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        
        f.render_widget(program_list, chunks[0]);
    }
    
    let instructions = Paragraph::new("↑/↓: Navigate | Enter: Load | E: Edit | R: Rename | D: Delete | ESC: Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    
    f.render_widget(instructions, chunks[1]);
}

pub fn render_help_guide(f: &mut Frame, area: Rect) {
    let help_text = r#"
ARC-0 Assembly Emulator Help Guide

=== BASIC OPERATIONS ===
- Use arrow keys to navigate menus
- Press Enter to select menu items
- Press TAB to switch between command mode and paused mode
- Press ESC to return to the start menu

=== COMMAND SYNTAX ===
ARC-0 assembly uses a simple syntax:
- One command per line
- Operands can be registers, immediates, or memory addresses

=== REGISTERS ===
The emulator supports the following registers:
- General Purpose: AX, BX, CX, DX
- Segment: CS, DS, SS, ES
- Stack/Pointer: SP, BP, SI, DI
- Control: PC, FLAGS

=== MOVEMENT COMMANDS ===
MOVI reg, imm     Move immediate value to register
MOVW reg, src     Move word (register, immediate, or memory) to register
LODI reg, imm     Load immediate value to register
LODW reg, addr    Load word from memory to register
STRI addr, imm    Store immediate value to memory
STRW addr, src    Store word (register or immediate) to memory
PUSH src          Push value (register or immediate) onto the stack
POP reg           Pop value from stack to register
XCGH reg1, reg2   Exchange values between two registers

=== ARITHMETIC COMMANDS ===
ADDW reg, src     Add word (register or immediate) to register
ADDI reg, imm     Add immediate to register (16-bit)
SUBW reg, src     Subtract word (register or immediate) from register
SUBI reg, imm     Subtract immediate from register (16-bit)
INC reg           Increment register
DEC reg           Decrement register
NEG reg           Negate register
MUL reg, src      Multiply register by word (register or immediate)

=== BITWISE COMMANDS ===
NOT reg           Bitwise NOT of register
AND reg, src      Bitwise AND of register with word (register or immediate)
OR reg, src       Bitwise OR of register with word (register or immediate)
XOR reg, src      Bitwise XOR of register with word (register or immediate)

=== COMPARE AND JUMP COMMANDS ===
CMPW op1, op2     Compare two operands (register or immediate)
JMP target        Jump to address (label, immediate, or address)
CALL target        Call subroutine
RET               Return from subroutine
JE target         Jump if equal
JNE target        Jump if not equal
JGT target        Jump if greater (signed)
JGE target        Jump if greater or equal (signed)
JLT target        Jump if less (signed)
JLE target        Jump if less or equal (signed)
JS target         Jump if sign (negative)
JCO target        Jump if carry or overflow

=== EXAMPLES ===
MOVI AX, 10       ; Move immediate value 10 to AX
MOVW BX, AX       ; Move AX to BX
ADDW AX, 5        ; Add 5 to AX
STRW [100], AX    ; Move AX to memory address 100

Press any key to return.
"#;
    
    let help_block = Block::default()
        .borders(Borders::ALL)
        .title("ARC-0 Assembly Help Guide")
        .style(Style::default().fg(Color::Cyan));
    
    let help_paragraph = Paragraph::new(help_text)
        .block(help_block)
        .style(Style::default().fg(Color::White))
        .scroll((0, 0));
    
    f.render_widget(help_paragraph, area);
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