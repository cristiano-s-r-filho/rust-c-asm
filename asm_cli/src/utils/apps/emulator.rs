use crate::utils::assembler::command_processor::AssembledProgram;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState, BorderType},
    Frame,
};

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::workspaces::Workspace;
use crate::utils::ui::common::{AppStatus, AppState};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct EmulatorState {
    pub cpu: CPU,
    pub memory: WorkMemory,
    pub is_running: bool,
    pub show_flags: bool,
    pub selected_register: ListState,
    pub selected_memory_page: usize,
    pub selected_memory_address: ListState,
    pub memory_page_size: usize,
    pub current_program_path: Option<String>,
    pub program_source: Option<String>,
    pub assembled_program: Option<AssembledProgram>,
    pub last_assembly_errors: Vec<String>,
    pub program_loaded: bool,
    pub current_instruction: u32,
    pub breakpoints: HashSet<u32>,
}

impl Default for EmulatorState {
    fn default() -> Self {
        Self::new(crate::memory::main_memory::DEFAULT_MEMORY_SIZE)
    }
}

impl EmulatorState {
    pub fn new(memory_size: usize) -> Self {
        let mut state = Self {
            cpu: CPU::new(),
            memory: WorkMemory::new(memory_size),
            is_running: false,
            show_flags: false,
            selected_register: ListState::default(),
            selected_memory_address: ListState::default(),
            memory_page_size: 16, // 16 words per page
            selected_memory_page: 0,
            current_program_path: None,
            program_source: None,
            assembled_program: None,
            last_assembly_errors: Vec::new(),
            program_loaded: false,
            current_instruction: 0,
            breakpoints: HashSet::new(),
        };
        state.selected_register.select(Some(0));
        state.selected_memory_address.select(Some(0));
        state
    }

    pub fn reset(&mut self, memory_size: usize) {
        self.cpu = CPU::new();
        self.memory = WorkMemory::new(memory_size);
        self.is_running = false;
        self.show_flags = false;
        self.selected_register.select(Some(0));
        self.selected_memory_address.select(Some(0));
        self.selected_memory_page = 0;
    }

    pub fn load_assembled_program(&mut self, assembled_program: &AssembledProgram) -> Result<(), String> {
        // Load text section
        self.memory.load_program(assembled_program.actual_text_start, &assembled_program.text)?;
        
        // Load data section
        if !assembled_program.data.is_empty() {
            self.memory.load_data(assembled_program.actual_data_start, &assembled_program.data)?;
        }
        
        self.cpu.registers.pc = assembled_program.actual_text_start; // Start at beginning of text segment
        self.cpu.registers.sp = assembled_program.actual_stack_start; // Reset stack pointer to configured start
        self.program_loaded = true;
        self.current_instruction = if !assembled_program.text.is_empty() { 
            self.memory.read_u32(assembled_program.actual_text_start)?
        } else { 
            0 
        };
        
        Ok(())
    }

    pub fn toggle_running(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.is_running {
            return Err("Cannot step while emulator is running. Pause first.".to_string());
        }
        self.cpu.step(&mut self.memory)
    }

    pub fn run_full_speed(&mut self) -> Result<(), String> {
        if !self.is_running {
            return Err("Emulator is paused. Start first.".to_string());
        }
        // In a real application, this would run in a separate thread
        // For a TUI, we'll just step once per frame while running
        self.cpu.step(&mut self.memory)
    }

    pub fn next_register(&mut self) {
        let i = match self.selected_register.selected() {
            Some(i) => {
                if i >= 13 { // Changed from self.cpu.registers.len() - 1
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_register.select(Some(i));
    }

    pub fn previous_register(&mut self) {
        let i = match self.selected_register.selected() {
            Some(i) => {
                if i == 0 {
                    13 // Changed from self.cpu.registers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_register.select(Some(i));
    }

    pub fn next_memory_address(&mut self) {
        let i = match self.selected_memory_address.selected() {
            Some(i) => {
                if i >= self.memory_page_size - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_memory_address.select(Some(i));
    }

    pub fn previous_memory_address(&mut self) {
        let i = match self.selected_memory_address.selected() {
            Some(i) => {
                if i == 0 {
                    self.memory_page_size - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_memory_address.select(Some(i));
    }

    pub fn next_memory_page(&mut self) {
        self.selected_memory_page = (self.selected_memory_page + 1) % self.memory.get_num_pages(self.memory_page_size);
        self.selected_memory_address.select(Some(0)); // Reset address selection on page change
    }

    pub fn previous_memory_page(&mut self) {
        let num_pages = self.memory.get_num_pages(self.memory_page_size);
        self.selected_memory_page = (self.selected_memory_page + num_pages - 1) % num_pages;
        self.selected_memory_address.select(Some(0)); // Reset address selection on page change
    }
}

pub fn handle_emulator_input(
    key: KeyEvent,
    emulator_state: &mut EmulatorState,
    _workspace: &mut Workspace,
    app_state: &mut AppState,
    status: &mut AppStatus,
    handled: &mut bool,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match key.code {
        KeyCode::Char('q') => {
            *app_state = AppState::StartMenu;
            *handled = true;
        }
        KeyCode::Char('p') => {
            emulator_state.toggle_running();
            status.set_message(format!("Emulator {}", if emulator_state.is_running { "running" } else { "paused" }));
            *handled = true;
        }
        KeyCode::Char('s') => {
            match emulator_state.step() {
                Ok(_) => status.set_message("Stepped one instruction.".to_string()),
                Err(e) => status.set_message(format!("Error stepping: {}", e)),
            }
            *handled = true;
        }
        KeyCode::Char('r') => {
            emulator_state.reset(emulator_state.memory.size);
            status.set_message("Emulator reset.".to_string());
            *handled = true;
        }
        KeyCode::Char('f') => {
            emulator_state.show_flags = !emulator_state.show_flags;
            *handled = true;
        }
        KeyCode::Char('i') => {
            *app_state = AppState::IoDevices;
            *handled = true;
        }
        KeyCode::Up => {
            emulator_state.previous_register();
            *handled = true;
        }
        KeyCode::Down => {
            emulator_state.next_register();
            *handled = true;
        }
        KeyCode::Left => {
            emulator_state.previous_memory_page();
            *handled = true;
        }
        KeyCode::Right => {
            emulator_state.next_memory_page();
            *handled = true;
        }
        KeyCode::PageUp => {
            // Scroll memory view up
            emulator_state.previous_memory_address();
            *handled = true;
        }
        KeyCode::PageDown => {
            // Scroll memory view down
            emulator_state.next_memory_address();
            *handled = true;
        }
        _ => {}
    }
}

pub fn on_enter_emulator(
    emulator_state: &mut EmulatorState,
    workspace: &mut Workspace,
    status: &mut AppStatus,
) {
    if let Some(path) = workspace.get_selected_program_path() {
        match std::fs::read_to_string(&path) {
            Ok(source) => {
                match workspace.assemble_and_load_program(&source, status) {
                    Ok(_) => {
                        if let Some(emu) = &mut workspace.emulator {
                            *emulator_state = emu.clone();
                        }
                        status.set_message(format!("Assembled and loaded program: {}", path));
                    }
                    Err(e) => {
                        status.set_message(format!("Assembly error: {}", e));
                    }
                }
            }
            Err(e) => {
                status.set_message(format!("Error reading file: {}", e));
            }
        }
    } else {
        status.set_message("No program selected. Load one from File Explorer.".to_string());
    }
}

pub fn on_exit_emulator(workspace: &mut Workspace) {
    if let Some(emulator) = &mut workspace.emulator {
        emulator.is_running = false;
    }
}

pub fn render_emulator(
    frame: &mut Frame,
    area: Rect,
    emulator_state: &EmulatorState,
    _workspace: &Workspace, // No longer need workspace directly for rendering
) {
    frame.render_widget(ratatui::widgets::Clear, area);
    // if let Some(emulator) = &workspace.emulator { // Remove this check
        let layout = create_emulator_layout(area, emulator_state);

        render_general_registers(frame, layout.registers, emulator_state, &emulator_state.cpu);
        render_pointer_registers(frame, layout.pointer_registers, emulator_state, &emulator_state.cpu);
        render_control_registers(frame, layout.control_registers, emulator_state, &emulator_state.cpu);
        render_memory_view(frame, layout.memory, emulator_state, &emulator_state.memory);
        render_control_panel(frame, layout.controls, emulator_state, &emulator_state.cpu);
    // } // Remove this closing brace
}

struct EmulatorLayout {
    registers: Rect,
    pointer_registers: Rect,
    control_registers: Rect,
    memory: Rect,
    controls: Rect,
}

fn create_emulator_layout(area: Rect, _emulator_state: &EmulatorState) -> EmulatorLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), // Top: General and Pointer Registers
            Constraint::Percentage(10), // Middle: Control Registers
            Constraint::Percentage(60), // Bottom: Memory and Controls
        ])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // General Registers
            Constraint::Percentage(50), // Pointer Registers
        ])
        .split(chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80), // Memory Page
            Constraint::Percentage(20), // App Control
        ])
        .split(chunks[2]);

    EmulatorLayout {
        registers: top_chunks[0],
        pointer_registers: top_chunks[1],
        control_registers: chunks[1],
        memory: bottom_chunks[0],
        controls: bottom_chunks[1],
    }
}

fn render_general_registers(frame: &mut Frame, area: Rect, emulator_state: &EmulatorState, emulator: &CPU) {
    let registers_data = vec![
        ("AX", emulator.registers.ax),
        ("BX", emulator.registers.bx),
        ("CX", emulator.registers.cx),
        ("DX", emulator.registers.dx),
        ("EX", emulator.registers.ex),
        ("FX", emulator.registers.fx),
        ("GX", emulator.registers.gx),
        ("HX", emulator.registers.hx),
    ];

    let register_items: Vec<ListItem> = registers_data.iter().enumerate()
        .map(|(i, (name, val))| {
            let is_selected = emulator_state.selected_register.selected() == Some(i);
            let style = if is_selected {
                Style::default().fg(Color::Black).bg(Color::LightYellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{:<4}: 0x{:04X}", name, val)).style(style)
        })
        .collect();

    let registers_list = List::new(register_items)
        .block(Block::default().borders(Borders::ALL).title("General Registers").border_type(BorderType::Double).border_style(Style::default().fg(Color::Green)));

    frame.render_stateful_widget(registers_list, area, &mut emulator_state.selected_register.clone());
}

fn render_pointer_registers(frame: &mut Frame, area: Rect, emulator_state: &EmulatorState, emulator: &CPU) {
    let registers_data = vec![
        ("SP", emulator.registers.sp),
        ("BP", emulator.registers.bp),
        ("SI", emulator.registers.si),
        ("DI", emulator.registers.di),
    ];

    let register_items: Vec<ListItem> = registers_data.iter().enumerate()
        .map(|(i, (name, val))| {
            let is_selected = emulator_state.selected_register.selected() == Some(i + 8);
            let style = if is_selected {
                Style::default().fg(Color::Black).bg(Color::LightYellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{:<4}: 0x{:04X}", name, val)).style(style)
        })
        .collect();

    let registers_list = List::new(register_items)
        .block(Block::default().borders(Borders::ALL).title("Pointer Registers").border_type(BorderType::Double).border_style(Style::default().fg(Color::Cyan)));

    frame.render_stateful_widget(registers_list, area, &mut emulator_state.selected_register.clone());
}

fn render_control_registers(frame: &mut Frame, area: Rect, emulator_state: &EmulatorState, emulator: &CPU) {
    let registers_data = vec![
        ("PC", emulator.registers.pc),
        ("FLAGS", emulator.registers.flags),
    ];

    let register_items: Vec<ListItem> = registers_data.iter().enumerate()
        .map(|(i, (name, val))| {
            let is_selected = emulator_state.selected_register.selected() == Some(i + 12);
            let style = if is_selected {
                Style::default().fg(Color::Black).bg(Color::LightYellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{:<4}: 0x{:04X}", name, val)).style(style)
        })
        .collect();

    let registers_list = List::new(register_items)
        .block(Block::default().borders(Borders::ALL).title("Control Registers").border_type(BorderType::Double).border_style(Style::default().fg(Color::Magenta)));

    frame.render_stateful_widget(registers_list, area, &mut emulator_state.selected_register.clone());
}

fn render_memory_view(frame: &mut Frame, area: Rect, emulator_state: &EmulatorState, memory: &WorkMemory) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Raw Memory
            Constraint::Percentage(50), // Disassembly
        ])
        .split(area);

    let raw_memory_area = chunks[0];
    let disassembly_area = chunks[1];

    // Render Raw Memory (existing logic)
    let start_address = emulator_state.selected_memory_page * emulator_state.memory_page_size;
    let end_address = (start_address + emulator_state.memory_page_size).min(memory.memory.len());

    let memory_items: Vec<ListItem> = memory.memory[start_address..end_address].iter().enumerate()
        .map(|(i, &val)| {
            let current_address = start_address + i;
            let is_selected = emulator_state.selected_memory_address.selected() == Some(i);
            let style = if is_selected {
                Style::default().fg(Color::Black).bg(Color::LightYellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("0x{:04X}: 0x{:04X}", current_address, val)).style(style)
        })
        .collect();

    let memory_list = List::new(memory_items)
        .block(Block::default().borders(Borders::ALL).title(format!("Memory (Page {})", emulator_state.selected_memory_page)).border_type(BorderType::Double));

    frame.render_stateful_widget(memory_list, raw_memory_area, &mut emulator_state.selected_memory_address.clone());

    // Render Disassembly
    let mut disassembly_items: Vec<ListItem> = Vec::new();
    if let Some(assembled_program) = &emulator_state.assembled_program {
        for (i, &instruction) in assembled_program.text.iter().enumerate() {
            let address = i * 4; // Corrected for 32-bit instructions (4 bytes per instruction)
            let disassembled_line = disassemble_instruction(instruction);
            let is_current_instruction = emulator_state.cpu.registers.pc == address as u32;
            let style = if is_current_instruction {
                Style::default().fg(Color::Black).bg(Color::LightGreen).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            disassembly_items.push(ListItem::new(format!("0x{:04X}: {}", address, disassembled_line)).style(style));
        }
    }

    let disassembly_list = List::new(disassembly_items)
        .block(Block::default().borders(Borders::ALL).title("Disassembly").border_type(BorderType::Double));

    frame.render_widget(disassembly_list, disassembly_area);
}

fn disassemble_instruction(instruction: u32) -> String {
    let opcode = (instruction >> 24) as u8;

    match opcode {
        0x01 => { // MOVI reg, imm
            let reg_num = (instruction >> 16) & 0xFF;
            let imm = (instruction & 0xFFFF) as u16;
            format!("MOVI {}, 0x{:04X}", reg_num_to_name(reg_num as u8), imm)
        },
        0x02 => { // MOVW reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("MOVW {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("MOVW {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x03 => { // LODI reg, imm/label
            let reg_num = (instruction >> 16) & 0xFF;
            let imm = (instruction & 0xFFFF) as u16;
            format!("LODI {}, 0x{:04X}", reg_num_to_name(reg_num as u8), imm)
        },
        0x04 => { // LODW reg, [addr/label]
            let reg_num = (instruction >> 16) & 0xFF;
            let addr = instruction & 0xFFFF;
            format!("LODW {}, [0x{:04X}]", reg_num_to_name(reg_num as u8), addr)
        },
        0x05 => { // STRI [addr/label], imm
            let addr = (instruction >> 16) & 0xFF;
            let imm = (instruction & 0xFFFF) as u16;
            format!("STRI [0x{:02X}], 0x{:04X}", addr, imm)
        },
        0x06 => { // STRW [addr/label], reg/imm
            let addr = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Addr-Reg
                let reg_num = (instruction >> 8) & 0xFF;
                format!("STRW [0x{:02X}], {}", addr, reg_num_to_name(reg_num as u8))
            } else { // Addr-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("STRW [0x{:02X}], 0x{:04X}", addr, imm)
            }
        },
        0x07 => { // PUSH reg/imm
            if (instruction & 1) == 1 { // Register
                let reg_num = (instruction >> 16) & 0xFF;
                format!("PUSH {}", reg_num_to_name(reg_num as u8))
            } else { // Immediate
                let imm = (instruction & 0xFFFF) as u16;
                format!("PUSH 0x{:04X}", imm)
            }
        },
        0x08 => { // POP reg
            let reg_num = (instruction >> 16) & 0xFF;
            format!("POP {}", reg_num_to_name(reg_num as u8))
        },
        0x09 => { // XCGH reg, reg
            let reg1_num = (instruction >> 16) & 0xFF;
            let reg2_num = (instruction >> 8) & 0xFF;
            format!("XCGH {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
        },
        0x10 => { // ADDW reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("ADDW {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("ADDW {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x11 => { // SUBW reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("SUBW {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("SUBW {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x12 => { // MUL reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("MUL {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("MUL {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x13 => { // INC reg
            let reg_num = (instruction >> 16) & 0xFF;
            format!("INC {}", reg_num_to_name(reg_num as u8))
        },
        0x14 => { // DEC reg
            let reg_num = (instruction >> 16) & 0xFF;
            format!("DEC {}", reg_num_to_name(reg_num as u8))
        },
        0x15 => { // NEG reg
            let reg_num = (instruction >> 16) & 0xFF;
            format!("NEG {}", reg_num_to_name(reg_num as u8))
        },
        0x20 => { // NOT reg
            let reg_num = (instruction >> 16) & 0xFF;
            format!("NOT {}", reg_num_to_name(reg_num as u8))
        },
        0x21 => { // AND reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("AND {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("AND {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x22 => { // OR reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("OR {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("OR {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x23 => { // XOR reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("XOR {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("XOR {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x30 => { // CMPW reg, reg/imm
            let reg1_num = (instruction >> 16) & 0xFF;
            if (instruction & 1) == 1 { // Reg-Reg
                let reg2_num = (instruction >> 8) & 0xFF;
                format!("CMPW {}, {}", reg_num_to_name(reg1_num as u8), reg_num_to_name(reg2_num as u8))
            } else { // Reg-Imm
                let imm = (instruction & 0xFFFF) as u16;
                format!("CMPW {}, 0x{:04X}", reg_num_to_name(reg1_num as u8), imm)
            }
        },
        0x40 => { // JMP addr
            let addr = instruction & 0xFFFFFF;
            format!("JMP 0x{:06X}", addr)
        },
        0x41 => { // CALL addr
            let addr = instruction & 0xFFFFFF;
            format!("CALL 0x{:06X}", addr)
        },
        0x42 => { // RET
            format!("RET")
        },
        0x43 => { // JE addr
            let addr = instruction & 0xFFFFFF;
            format!("JE 0x{:06X}", addr)
        },
        0x44 => { // JNE addr
            let addr = instruction & 0xFFFFFF;
            format!("JNE 0x{:06X}", addr)
        },
        0x45 => { // JGT addr
            let addr = instruction & 0xFFFFFF;
            format!("JGT 0x{:06X}", addr)
        },
        0x46 => { // JGE addr
            let addr = instruction & 0xFFFFFF;
            format!("JGE 0x{:06X}", addr)
        },
        0x47 => { // JLT addr
            let addr = instruction & 0xFFFFFF;
            format!("JLT 0x{:06X}", addr)
        },
        0x48 => { // JLE addr
            let addr = instruction & 0xFFFFFF;
            format!("JLE 0x{:06X}", addr)
        },
        0x49 => { // JS addr
            let addr = instruction & 0xFFFFFF;
            format!("JS 0x{:06X}", addr)
        },
        0x4A => { // JCO addr
            let addr = instruction & 0xFFFFFF;
            format!("JCO 0x{:06X}", addr)
        },
        0x50 => { // IN reg
            let reg_num = (instruction >> 16) & 0xFF;
            format!("IN {}", reg_num_to_name(reg_num as u8))
        },
        0x51 => { // OUT reg/addr
            if (instruction & 1) == 1 { // Register
                let reg_num = (instruction >> 16) & 0xFF;
                format!("OUT {}", reg_num_to_name(reg_num as u8))
            } else { // Address (from assembler, this is the label address)
                let addr = instruction & 0xFFFFFF;
                format!("OUT 0x{:06X}", addr)
            }
        },
        0x60 => { // SETF flag
            let flag_id = instruction & 0xFF;
            format!("SETF {}", flag_id_to_name(flag_id as u8))
        },
        0x61 => { // CLRF flag
            let flag_id = instruction & 0xFF;
            format!("CLRF {}", flag_id_to_name(flag_id as u8))
        },
        0xFF => { // HALT
            format!("HALT")
        },
        _ => format!("UNKNOWN 0x{:08X}", instruction),
    }
}

// Helper to convert register number to name
fn reg_num_to_name(num: u8) -> &'static str {
    match num {
        0 => "AX", 1 => "BX", 2 => "CX", 3 => "DX",
        4 => "EX", 5 => "FX", 6 => "GX", 7 => "HX",
        8 => "SP", 9 => "BP", 10 => "SI", 11 => "DI",
        12 => "PC", 13 => "FLAGS",
        _ => "R??",
    }
}

// Helper to convert flag ID to name
fn flag_id_to_name(id: u8) -> &'static str {
    match id {
        0 => "ZERO",
        1 => "SIGN",
        2 => "CARRY",
        3 => "OVERFLOW",
        _ => "FLAG??",
    }
}

fn render_flags_display(frame: &mut Frame, area: Rect, emulator: &CPU) {
    let flags_text = format!(
        "Z: {} | N: {} | C: {} | V: {}",
        emulator.registers.get_flag("zero").unwrap_or(false) as u8,
        emulator.registers.get_flag("sign").unwrap_or(false) as u8, // Assuming 'sign' is negative_flag
        emulator.registers.get_flag("carry").unwrap_or(false) as u8,
        emulator.registers.get_flag("overflow").unwrap_or(false) as u8,
    );
    let flags_paragraph = Paragraph::new(flags_text)
        .block(Block::default().borders(Borders::ALL).title("Flags").border_type(BorderType::Double));
    frame.render_widget(flags_paragraph, area);
}

fn render_control_panel(frame: &mut Frame, area: Rect, emulator_state: &EmulatorState, emulator: &CPU) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let status_text = if emulator_state.is_running {
        "Running (P: Pause, S: Step, R: Reset, F: Toggle Flags)"
    } else {
        "Paused (P: Run, S: Step, R: Reset, F: Toggle Flags)"
    };

    let pc_text = format!("PC: 0x{:04X}", emulator.registers.pc);

    let control_panel_text = format!("{}\n{}", status_text, pc_text);

    let control_panel = Paragraph::new(control_panel_text)
        .block(Block::default().borders(Borders::ALL).title("Controls").border_type(BorderType::Double));

    frame.render_widget(control_panel, chunks[0]);

    if emulator_state.show_flags {
        render_flags_display(frame, chunks[1], emulator);
    }
}