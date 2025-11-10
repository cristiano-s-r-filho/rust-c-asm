//! # Command Processor Module
//!
//! This module provides functionalities for parsing, assembling, and executing
//! assembly commands for the ARC CPU. It handles directives, macros, and
//! translates assembly instructions into machine code.


use crate::utils::assembler::operands::{Operand, parse_operand};
use crate::memory::registers::Reg;

use std::collections::HashMap;

// Default segment values if not specified by directives
const DEFAULT_TEXT_START: u32 = 0x0000;
const DEFAULT_STACK_SIZE: u32 = 0x1000; // 4KB

/// Represents a macro definition in the assembly code.
#[derive(Debug, Clone)]
pub struct Macro {
    /// The name of the macro.
    pub name: String,
    /// The arguments the macro accepts.
    pub args: Vec<String>,
    /// The body of the macro, consisting of a sequence of commands.
    pub body: Vec<Command>,
}

/// Represents a single assembly command or directive.
#[derive(Debug, Clone)]
pub struct Command {
    /// The opcode or directive name (e.g., "MOVI", ".word").
    pub opcode: String,
    /// The first operand, if present.
    pub operand1: Option<Operand>,
    /// The second operand, if present.
    pub operand2: Option<Operand>,
    /// An optional label associated with this command.
    pub label: Option<String>,
    /// The name of the macro if this command is a macro call.
    pub macro_name: Option<String>,
    /// The arguments passed to the macro if this command is a macro call.
    pub macro_args: Option<Vec<String>>,
    /// Optional: The starting address for the text segment, if specified by a directive.
    pub text_start_address: Option<u32>,
    /// Optional: The starting address for the stack segment, if specified by a directive.
    pub stack_start_address: Option<u32>,
    /// Optional: The size of the stack segment, if specified by a directive.
    pub stack_segment_size: Option<u32>,
}

/// Represents the current section of the assembly code (e.g., `.text` or `.data`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Section {
    /// The text (code) section.
    Text,
    /// The data section.
    Data,
}

/// Represents the assembled program, containing machine code and data.
#[derive(Debug, Default, Clone)]
pub struct AssembledProgram {
    /// The machine code instructions.
    pub text: Vec<u32>,
    /// The initialized data.
    pub data: Vec<u8>,
    /// The actual starting address of the text segment.
    pub actual_text_start: u32,
    /// The actual starting address of the data segment.
    pub actual_data_start: u32,
    /// The actual starting address of the stack segment.
    pub actual_stack_start: u32,
    /// The actual size of the stack segment.
    pub actual_stack_size: u32,
}

/// Parses a single line of assembly code into a `Command` structure.
///
/// This function handles comments, labels, opcodes, and operands,
/// including directives and macro definitions.
///
/// # Arguments
///
/// * `input` - A string slice representing a line of assembly code.
///
/// # Returns
///
/// * `Result<Command, String>` - `Ok(Command)` on successful parsing, or `Err(String)` on failure.
pub fn parse_command(input: &str) -> Result<Command, String> {

    let comment_start = input.find(';');
    let without_comment = if let Some(index) = comment_start {
        &input[..index]
    } else {
        input
    };
    let trimmed_input = without_comment.trim();

    if trimmed_input.is_empty() {
        return Ok(Command {
            opcode: "".to_string(),
            operand1: None,
            operand2: None,
            label: None,
            macro_name: None,
            macro_args: None,
            text_start_address: None,
            stack_start_address: None,
            stack_segment_size: None,
        });
    }

    let mut parts = trimmed_input.splitn(2, char::is_whitespace);
    let first_part = parts.next().unwrap_or("").to_string();

    let (label, remaining) = if first_part.ends_with(':') {
        (Some(first_part[..first_part.len()-1].to_string()), parts.next().unwrap_or(""))
    } else {
        (None, trimmed_input)
    };

    let mut command_parts = remaining.splitn(2, char::is_whitespace);
    let opcode = command_parts.next().unwrap_or("").to_string();
    let operands_str = command_parts.next();

    let mut command = Command {
        opcode: opcode.clone(),
        operand1: None,
        operand2: None,
        label: label.clone(), // Clone the label here
        macro_name: None,
        macro_args: None,
        text_start_address: None,
        stack_start_address: None,
        stack_segment_size: None,
    };

    if opcode.starts_with('.') {
        match opcode.as_str() {
            ".text_start" => {
                if let Some(op_str) = operands_str {
                    let parsed_op = parse_operand(op_str)?;
                    if let Operand::Immediate(addr) = parsed_op {
                        command.text_start_address = Some(addr as u32);
                    } else {
                        return Err(format!("Invalid operand for .text_start directive. Expected immediate address."));
                    }
                } else {
                    return Err(format!("Missing operand for .text_start directive. Expected immediate address."));
                }
            },
            ".stack_start" => {
                if let Some(op_str) = operands_str {
                    let parsed_op = parse_operand(op_str)?;
                    if let Operand::Immediate(addr) = parsed_op {
                        command.stack_start_address = Some(addr as u32);
                    } else {
                        return Err(format!("Invalid operand for .stack_start directive. Expected immediate address."));
                    }
                } else {
                    return Err(format!("Missing operand for .stack_start directive. Expected immediate address."));
                }
            },
            ".stack_size" => {
                if let Some(op_str) = operands_str {
                    let parsed_op = parse_operand(op_str)?;
                    if let Operand::Immediate(size) = parsed_op {
                        command.stack_segment_size = Some(size as u32);
                    } else {
                        return Err(format!("Invalid operand for .stack_size directive. Expected immediate size."));
                    }
                } else {
                    return Err(format!("Missing operand for .stack_size directive. Expected immediate size."));
                }
            },
            _ => {
                // Handle existing directives
                if let Some(op_str) = operands_str {
                    if !op_str.is_empty() {
                        let parsed_op = parse_operand(op_str)?;
                        command.operand1 = Some(parsed_op);
                    }
                }
            }
        }
        return Ok(command);
    }

    if opcode == ".macro" {
        let mut parts = operands_str.unwrap_or("").split_whitespace();
        let name = parts.next().unwrap_or("").to_string();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
        command.macro_name = Some(name);
        command.macro_args = Some(args);
        return Ok(command);
    } else if opcode == ".endmacro" {
        return Ok(command);
    }

    if opcode.is_empty() && label.is_some() {
        return Ok(command);
    }
    
    if let Some(operands_str) = operands_str {
        let trimmed_operands_str = operands_str.trim();
        let mut comma_split = trimmed_operands_str.splitn(2, ',').map(|s| s.trim());

        let first_part = comma_split.next().unwrap_or("");
        let second_part = comma_split.next();

        if let Some(op2_str_after_comma) = second_part {
            if !first_part.is_empty() {
                command.operand1 = Some(parse_operand(first_part)?);
            }
            if !op2_str_after_comma.is_empty() {
                command.operand2 = Some(parse_operand(op2_str_after_comma)?);
            }
        } else {
            let mut space_split = first_part.splitn(2, char::is_whitespace).map(|s| s.trim());
            if let Some(op1_str_space) = space_split.next() {
                if !op1_str_space.is_empty() {
                    command.operand1 = Some(parse_operand(op1_str_space)?);
                }
            }
            if let Some(op2_str_space) = space_split.next() {
                if !op2_str_space.is_empty() {
                    command.operand2 = Some(parse_operand(op2_str_space)?);
                }
            }
        }
    }
    
    Ok(command)
}

// ... (rest of the file)


pub fn assemble_program(commands: &[Command], macros: &[Macro], total_memory_size: usize) -> Result<AssembledProgram, String> {
    let mut symbol_table = HashMap::new();
    let mut text_address_counter = DEFAULT_TEXT_START; // Default text start
    let mut data_address_counter = 0; // Data address counter will be relative to actual_data_start
    let mut current_section = Section::Text;

    // Configured segment values (from directives)
    let mut configured_text_start: Option<u32> = None;
    let mut configured_stack_start: Option<u32> = None;
    let mut configured_stack_size: Option<u32> = None;

    // Pass 1: Build symbol table and process directives
    for command in commands {
        if command.opcode == ".equ" {
            if let (Some(label), Some(Operand::Immediate(value))) = (&command.label, &command.operand1) {
                if symbol_table.contains_key(label) {
                    return Err(format!("Duplicate label: {}", label));
                }
                symbol_table.insert(label.clone(), *value as u32);
            } else {
                return Err(".equ directive requires a label and an immediate value".to_string());
            }
            continue;
        }

        if let Some(ts_addr) = command.text_start_address {
            if configured_text_start.is_some() {
                return Err("Multiple .text_start directives found. Only one is allowed.".to_string());
            }
            configured_text_start = Some(ts_addr);
        }
        if let Some(ss_addr) = command.stack_start_address {
            if configured_stack_start.is_some() {
                return Err("Multiple .stack_start directives found. Only one is allowed.".to_string());
            }
            configured_stack_start = Some(ss_addr);
        }
        if let Some(ss_size) = command.stack_segment_size {
            if configured_stack_size.is_some() {
                return Err("Multiple .stack_size directives found. Only one is allowed.".to_string());
            }
            configured_stack_size = Some(ss_size);
        }

        if command.opcode == ".text" {
            current_section = Section::Text;
            continue;
        } else if command.opcode == ".data" {
            current_section = Section::Data;
            continue;
        }

        if let Some(label) = &command.label {
            if symbol_table.contains_key(label) {
                return Err(format!("Duplicate label: {}", label));
            }
            let address = match current_section {
                Section::Text => text_address_counter,
                Section::Data => data_address_counter,
            };
            symbol_table.insert(label.clone(), address);
        }

        if !command.opcode.is_empty() && !command.opcode.starts_with('.') { // Only count instructions for text_address_counter
            let (size, padding) = get_instruction_or_data_size(command, data_address_counter)?;
            match current_section {
                Section::Text => text_address_counter += size,
                Section::Data => {
                    data_address_counter += padding;
                    data_address_counter += size;
                }
            }
        }
    }

    // Apply configured values or defaults after Pass 1
    let actual_text_start = configured_text_start.unwrap_or(DEFAULT_TEXT_START);
    let actual_stack_size = configured_stack_size.unwrap_or(DEFAULT_STACK_SIZE);
    let actual_stack_start = configured_stack_start.unwrap_or_else(|| {
        // Default stack starts at total_memory_size - actual_stack_size
        (total_memory_size as u32).checked_sub(actual_stack_size).unwrap_or(0)
    });

    // Validate segment boundaries
    if actual_text_start as usize >= total_memory_size {
        return Err(format!(".text_start address (0x{:04X}) is outside total memory (0x{:04X}).", actual_text_start, total_memory_size));
    }
    if actual_stack_start as usize >= total_memory_size {
        return Err(format!(".stack_start address (0x{:04X}) is outside total memory (0x{:04X}).", actual_stack_start, total_memory_size));
    }
    if (actual_stack_start as usize + actual_stack_size as usize) > total_memory_size {
        return Err(format!("Stack segment (0x{:04X} - 0x{:04X}) exceeds total memory (0x{:04X}).", actual_stack_start, actual_stack_start + actual_stack_size, total_memory_size));
    }

    // Data segment starts right after the stack segment (growing downwards)
    let actual_data_start = actual_stack_start.checked_sub(data_address_counter).unwrap_or(0);

    // Ensure text and data don't overlap
    if actual_text_start + text_address_counter > actual_data_start {
        return Err(format!("Text segment (0x{:04X} - 0x{:04X}) overlaps with Data segment (0x{:04X}).", actual_text_start, actual_text_start + text_address_counter, actual_data_start));
    }

    // Pass 2: Assemble
    let mut expanded_commands = Vec::new();
    for command in commands {
        if let Some(macro_to_expand) = macros.iter().find(|m| m.name == command.opcode) {
            let mut expanded_macro = macro_to_expand.body.clone();
            for (i, arg) in macro_to_expand.args.iter().enumerate() {
                let value = match i {
                    0 => command.operand1.clone(),
                    1 => command.operand2.clone(),
                    _ => None,
                };
                for cmd in &mut expanded_macro {
                    if let Some(op1) = &mut cmd.operand1 {
                        if let Operand::Label(l) = op1 {
                            if l == arg {
                                *op1 = value.clone().ok_or_else(|| format!("Missing argument for macro parameter: {}", arg))?;
                            }
                        }
                    }
                    if let Some(op2) = &mut cmd.operand2 {
                        if let Operand::Label(l) = op2 {
                            if l == arg {
                                *op2 = value.clone().ok_or_else(|| format!("Missing argument for macro parameter: {}", arg))?;
                            }
                        }
                    }
                }
            }
            expanded_commands.extend(expanded_macro);
        } else {
            expanded_commands.push(command.clone());
        }
    }

    let mut assembled_program = AssembledProgram::default();
    assembled_program.actual_text_start = actual_text_start;
    assembled_program.actual_data_start = actual_data_start;
    assembled_program.actual_stack_start = actual_stack_start;
    assembled_program.actual_stack_size = actual_stack_size;

    current_section = Section::Text;
    let mut data_address_counter_pass2 = actual_data_start;

    for command in &expanded_commands {
        if command.opcode == ".text" {
            current_section = Section::Text;
            continue;
        } else if command.opcode == ".data" {
            current_section = Section::Data;
            continue;
        } else if command.opcode == ".equ" || command.opcode == ".text_start" || command.opcode == ".stack_start" || command.opcode == ".stack_size" {
            continue;
        }

        if command.opcode == ".align" {
            if current_section == Section::Data {
                if let Some(Operand::Immediate(boundary)) = command.operand1 {
                    let boundary = boundary as u32;
                    let padding = (boundary - (data_address_counter_pass2 % boundary)) % boundary;
                    for _ in 0..padding {
                        assembled_program.data.push(0);
                    }
                    data_address_counter_pass2 += padding;
                }
            }
            continue;
        }

        if command.opcode.is_empty() {
            continue;
        }

        match current_section {
            Section::Text => {
                if !command.opcode.starts_with('.') {
                    let instruction = assemble_instruction(command, &symbol_table)?;
                    assembled_program.text.push(instruction);
                }
            }
            Section::Data => {
                let data_bytes = assemble_data(command, &symbol_table)?;
                data_address_counter_pass2 += data_bytes.len() as u32;
                assembled_program.data.extend(data_bytes);
            }
        }
    }

    Ok(assembled_program)
}

/// Determines the size of an instruction or data directive.
///
/// This is used during the first pass of assembly to calculate addresses.
///
/// # Arguments
///
/// * `command` - The `Command` to evaluate.
/// * `current_address` - The current address counter, used for `.align` directive.
///
/// # Returns
///
/// * `Result<(u32, u32), String>` - A tuple containing `(size, padding)` on success,
///   or `Err(String)` if an unknown directive or invalid operand is encountered.
fn get_instruction_or_data_size(command: &Command, current_address: u32) -> Result<(u32, u32), String> {
    if command.opcode.starts_with('.') { // Directive
        match command.opcode.as_str() {
            ".text" | ".data" | ".equ" => Ok((0, 0)),
            ".word" => Ok((4, 0)),
            ".byte" => Ok((1, 0)),
            ".string" => {
                if let Some(Operand::String(s)) = &command.operand1 {
                    Ok(((s.len() + 1) as u32, 0))
                } else {
                    Err(".string directive requires a string operand".to_string())
                }
            }
            ".space" => {
                if let Some(Operand::Immediate(value)) = command.operand1 {
                    Ok((value as u32, 0))
                } else {
                    Err(".space directive requires an immediate value".to_string())
                }
            }
            ".align" => {
                if let Some(Operand::Immediate(boundary)) = command.operand1 {
                    let boundary = boundary as u32;
                    if boundary == 0 || !boundary.is_power_of_two() {
                        return Err(".align boundary must be a power of two".to_string());
                    }
                    let padding = (boundary - (current_address % boundary)) % boundary;
                    Ok((0, padding))
                } else {
                    Err(".align directive requires an immediate value".to_string())
                }
            }
            ".bitv" => {
                if let (Some(Operand::Address(begin_addr)), Some(Operand::Address(end_addr))) = (&command.operand1, &command.operand2) {
                    if begin_addr > end_addr {
                        return Err(".bitv begin address cannot be greater than end address".to_string());
                    }
                    Ok((end_addr - begin_addr + 1, 0))
                } else {
                    Err(".bitv directive requires two address operands".to_string())
                }
            }
            _ => Err(format!("Unknown directive: {}", command.opcode)),
        }
    } else { // Instruction
        Ok((4, 0)) // All instructions are 4 bytes
    }
}

/// Assembles data directives into a vector of bytes.
///
/// # Arguments
///
/// * `command` - The `Command` representing the data directive.
/// * `_symbol_table` - The symbol table (unused for data assembly, but required by signature).
///
/// # Returns
///
/// * `Result<Vec<u8>, String>` - A vector of bytes representing the assembled data,
///   or `Err(String)` if an unknown directive or invalid operand is encountered.
fn assemble_data(command: &Command, symbol_table: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
    match command.opcode.as_str() {
        ".word" => {
            let value = match command.operand1 {
                Some(Operand::Immediate(value)) => value as u32,
                Some(Operand::Label(ref label)) => {
                    *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?
                }
                _ => return Err(".word directive requires an immediate value or a label".to_string()),
            };
            Ok(value.to_le_bytes().to_vec())
        }
        ".byte" => {
            if let Some(Operand::Immediate(value)) = command.operand1 {
                Ok(vec![value as u8])
            } else {
                Err(".byte directive requires an immediate value".to_string())
            }
        }
        ".string" => {
            if let Some(Operand::String(s)) = &command.operand1 {
                let mut bytes = s.as_bytes().to_vec();
                bytes.push(0x00); // Append null terminator
                Ok(bytes)
            } else {
                Err(".string directive requires a string operand".to_string())
            }
        }
        ".space" => {
            if let Some(Operand::Immediate(value)) = command.operand1 {
                Ok(vec![0; value as usize])
            } else {
                Err(".space directive requires an immediate value".to_string())
            }
        }
        ".bitv" => {
            if let (Some(Operand::Address(begin_addr)), Some(Operand::Address(end_addr))) = (&command.operand1, &command.operand2) {
                if begin_addr > end_addr {
                    return Err(".bitv begin address cannot be greater than end address".to_string());
                }
                let size = (end_addr - begin_addr + 1) as usize;
                Ok(vec![0; size]) // Initialize with null bytes
            } else {
                Err(".bitv directive requires two address operands".to_string())
            }
        }
        _ => Err(format!("Unknown directive in data section: {}", command.opcode)),
    }
}

/// Assembles a single assembly instruction into its 32-bit machine code representation.
///
/// # Arguments
///
/// * `command` - The `Command` representing the instruction.
/// * `symbol_table` - The symbol table for resolving labels.
///
/// # Returns
///
/// * `Result<u32, String>` - The 32-bit machine code instruction on success,
///   or `Err(String)` if the instruction is unsupported or has invalid operands.
fn assemble_instruction(command: &Command, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if command.opcode.starts_with('.') {
        return Ok(0);
    }
    let opcode = command.opcode.to_lowercase();
    let op1 = command.operand1.as_ref();
    let op2 = command.operand2.as_ref();

    match opcode.as_str() {
        "movi" => assemble_reg_imm(0x01, op1, op2, symbol_table),
        "movw" => assemble_reg_reg_or_reg_imm(0x02, op1, op2),
        "lodi" => assemble_reg_imm(0x03, op1, op2, symbol_table),
        "lodw" => assemble_reg_addr(0x04, op1, op2, symbol_table),
        "stri" => assemble_addr_imm(0x05, op1, op2, symbol_table),
        "strw" => assemble_addr_reg_or_addr_imm(0x06, op1, op2, symbol_table),
        "push" => assemble_reg_or_imm(0x07, op1, symbol_table),
        "pop" => assemble_reg(0x08, op1),
        "xcgh" => assemble_reg_reg(0x09, op1, op2),
        "addw" => assemble_reg_reg_or_reg_imm(0x10, op1, op2),
        "subw" => assemble_reg_reg_or_reg_imm(0x11, op1, op2),
        "mul" => assemble_reg_reg_or_reg_imm(0x12, op1, op2),
        "inc" => assemble_reg(0x13, op1),
        "dec" => assemble_reg(0x14, op1),
        "neg" => assemble_reg(0x15, op1),
        "not" => assemble_reg(0x20, op1),
        "and" => assemble_reg_reg_or_reg_imm(0x21, op1, op2),
        "or" => assemble_reg_reg_or_reg_imm(0x22, op1, op2),
        "xor" => assemble_reg_reg_or_reg_imm(0x23, op1, op2),
        "shl" => assemble_reg_reg_or_reg_imm(0x24, op1, op2),
        "shr" => assemble_reg_reg_or_reg_imm(0x25, op1, op2),
        "cmpw" => assemble_reg_reg_or_reg_imm(0x30, op1, op2),
        "jmp" => assemble_addr(0x40, op1, symbol_table),
        "call" => assemble_addr(0x41, op1, symbol_table),
        "ret" => Ok(0x42 << 24),
        "je" => assemble_addr(0x43, op1, symbol_table),
        "jne" => assemble_addr(0x44, op1, symbol_table),
        "jgt" => assemble_addr(0x45, op1, symbol_table),
        "jge" => assemble_addr(0x46, op1, symbol_table),
        "jlt" => assemble_addr(0x47, op1, symbol_table),
        "jle" => assemble_addr(0x48, op1, symbol_table),
        "js" => assemble_addr(0x49, op1, symbol_table),
        "jco" => assemble_addr(0x4A, op1, symbol_table),
        "in" => assemble_addr(0x50, op1, symbol_table),
        "out" => assemble_addr(0x51, op1, symbol_table),
        "insi" => assemble_addr(0x52, op1, symbol_table), // For INSI, op1 is dest addr, op2 is optional immediate for I/O slot
        "outi" => assemble_reg_or_imm(0x53, op1, symbol_table), // For OUTI, op1 is value, op2 is optional immediate for I/O slot
        "insw" => assemble_addr_imm(0x54, op1, op2, symbol_table), // For INSW, op1 is dest addr, op2 is optional immediate for I/O slot
        "outw" => assemble_addr_reg_or_addr_imm(0x55, op1, op2, symbol_table), // For OUTW, op1 is src addr, op2 is optional immediate for I/O slot
        "setf" => assemble_flag_op(0x60, op1),
        "clrf" => assemble_flag_op(0x61, op1),
        "halt" => Ok(0xFF << 24),
        _ => Err(format!("Unsupported instruction: {}", opcode)),
    }
}

/// Assembles an instruction that operates on a flag.
///
/// # Arguments
///
/// * `opcode` - The base opcode for the instruction.
/// * `op1` - The operand, which must be an `Operand::Flag`.
///
/// # Returns
///
/// * `Result<u32, String>` - The assembled 32-bit instruction, or an error if `op1` is not a flag.
fn assemble_flag_op(opcode: u32, op1: Option<&Operand>) -> Result<u32, String> {
    if let Some(Operand::Flag(flag_id)) = op1 {
        Ok((opcode << 24) | (*flag_id as u32))
    } else {
        Err("Instruction requires a flag name operand".to_string())
    }
}

/// Converts a `Reg` enum variant to its numeric representation.
///
/// # Arguments
///
/// * `reg` - A reference to the `Reg` enum variant.
///
/// # Returns
///
/// * `Result<u8, String>` - The numeric representation of the register, or an error if the register is invalid.
fn register_to_number(reg: &Reg) -> Result<u8, String> {
    match reg {
        Reg::AX => Ok(0), Reg::BX => Ok(1), Reg::CX => Ok(2), Reg::DX => Ok(3),
        Reg::EX => Ok(4), Reg::FX => Ok(5), Reg::GX => Ok(6), Reg::HX => Ok(7),
        Reg::SP => Ok(8), Reg::BP => Ok(9), Reg::SI => Ok(10), Reg::DI => Ok(11),
        Reg::PC => Ok(12), Reg::FLAGS => Ok(13),
    }
}

/// Assembles a register-immediate instruction.
fn assemble_reg_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if let Some(Operand::Register(reg)) = op1 {
        let reg_num = register_to_number(reg)?;
        let imm_value = match op2 {
            Some(Operand::Immediate(imm)) => *imm as u32,
            Some(Operand::Label(label)) => *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
            _ => return Err("Invalid second operand for register-immediate instruction".to_string()),
        };
        if imm_value > 0xFFFF {
            return Err(format!("Immediate value {} is too large for this instruction. Maximum is 65535.", imm_value));
        }
        Ok((opcode << 24) | ((reg_num as u32) << 16) | (imm_value & 0xFFFF))
    } else {
        Err("Invalid first operand for register-immediate instruction".to_string())
    }
}

/// Assembles a register-register instruction.
fn assemble_reg_reg(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>) -> Result<u32, String> {
    if let (Some(Operand::Register(reg1)), Some(Operand::Register(reg2))) = (op1, op2) {
        let reg1_num = register_to_number(reg1)?;
        let reg2_num = register_to_number(reg2)?;
        Ok((opcode << 24) | ((reg1_num as u32) << 16) | ((reg2_num as u32) << 8))
    } else {
        Err("Invalid operands for register-register instruction".to_string())
    }
}

/// Assembles a register-register or register-immediate instruction.
fn assemble_reg_reg_or_reg_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>) -> Result<u32, String> {
    if let Some(Operand::Register(reg1)) = op1 {
        let reg1_num = register_to_number(reg1)?;
        match op2 {
            Some(Operand::Register(reg2)) => {
                let reg2_num = register_to_number(reg2)?;
                Ok((opcode << 24) | ((reg1_num as u32) << 16) | ((reg2_num as u32) << 8) | 1)
            }
            Some(Operand::Immediate(imm)) => {
                Ok((opcode << 24) | ((reg1_num as u32) << 16) | (*imm as u32 & 0xFFFF))
            }
            _ => Err("Invalid second operand".to_string()),
        }
    } else {
        Err("Invalid first operand".to_string())
    }
}

/// Assembles an instruction that operates on a single register.
fn assemble_reg(opcode: u32, op1: Option<&Operand>) -> Result<u32, String> {
    if let Some(Operand::Register(reg)) = op1 {
        let reg_num = register_to_number(reg)?;
        Ok((opcode << 24) | ((reg_num as u32) << 16))
    } else {
        Err("Invalid operand for register instruction".to_string())
    }
}

/// Assembles an instruction that operates on an address.
fn assemble_addr(opcode: u32, op1: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if let Some(operand) = op1 {

        let addr_val: u32 = match operand {
            Operand::Address(addr) => *addr,
            Operand::Immediate(imm) => *imm,
            Operand::Label(label) => *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
            Operand::AddressRegister(reg) => {
                let reg_num = register_to_number(reg)?;
                (1 << 23) | ((reg_num as u32) << 19)
            }
                            Operand::Register(reg) => {
                                return Err(format!("Instruction does not support register direct addressing. Use an address or label, or define a new instruction for register-indirect I/O. Encountered register: {:?}", reg));
                            }            _ => return Err("Invalid operand for address instruction".to_string()),
        };
        Ok((opcode << 24) | (addr_val & 0xFFFFFF))
    } else {
        Err("Missing operand for address instruction".to_string())
    }
}

/// Assembles a register-address instruction.
fn assemble_reg_addr(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if let Some(Operand::Register(reg)) = op1 {
        let reg_num = register_to_number(reg)?;
        if let Some(operand) = op2 {
            let addr = match operand {
                Operand::Address(addr) => *addr,
                Operand::Label(label) => *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
                _ => return Err("Invalid second operand for register-address instruction".to_string()),
            };
            Ok((opcode << 24) | ((reg_num as u32) << 16) | (addr & 0xFFFF))
        } else {
            Err("Missing second operand for register-address instruction".to_string())
        }
    } else {
        Err("Invalid first operand for register-address instruction".to_string())
    }
}

/// Assembles an address-immediate instruction.
fn assemble_addr_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    let addr_encoded = assemble_8bit_addr_or_reg_indirect(op1, symbol_table)?;
    let imm_value = match op2 {
        Some(Operand::Immediate(imm)) => *imm as u32,
        None => 0, // Default to I/O slot 0 if not provided
        _ => return Err("Invalid second operand for address-immediate instruction. Expected immediate or none.".to_string()),
    };
    Ok((opcode << 24) | ((addr_encoded as u32) << 16) | (imm_value & 0xFFFF))
}

/// Assembles an address-register or address-immediate instruction.
fn assemble_addr_reg_or_addr_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    let addr_encoded = assemble_8bit_addr_or_reg_indirect(op1, symbol_table)?;
    match op2 {
        Some(Operand::Register(reg)) => {
            let reg_num = register_to_number(reg)?;
            Ok((opcode << 24) | ((addr_encoded as u32) << 16) | ((reg_num as u32) << 8) | 1)
        }
        Some(Operand::Immediate(imm)) => {
            Ok((opcode << 24) | ((addr_encoded as u32) << 16) | (*imm as u32 & 0xFFFF))
        }
        None => { // Default to I/O slot 0 if not provided
            Ok((opcode << 24) | ((addr_encoded as u32) << 16) | (0 & 0xFFFF))
        }
        _ => Err("Invalid second operand. Expected register, immediate, or none.".to_string()),
    }
}

/// Assembles a register or immediate operand instruction.
fn assemble_reg_or_imm(opcode: u32, op1: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if let Some(operand) = op1 {

        match operand {
            Operand::Register(reg) => {
                let reg_num = register_to_number(reg)?;
                Ok((opcode << 24) | ((reg_num as u32) << 16) | 1)
            }
            Operand::Immediate(imm) => {
                Ok((opcode << 24) | (*imm as u32 & 0xFFFF))
            }
            Operand::Label(label) => {
                let imm_value = *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?;
                Ok((opcode << 24) | (imm_value & 0xFFFF))
            }
            Operand::AddressRegister(reg) => { // NEW
                let reg_num = register_to_number(reg)?;
                // Encode register-indirect address: highest bit (bit 23) set, next 4 bits for register number
                Ok((opcode << 24) | ( (1 << 23) | ((reg_num as u32) << 19) ) )
            }
            _ => Err("Invalid operand".to_string()),
        }
    } else {
        Err("Missing operand".to_string())
    }
}

/// Assembles an 8-bit address or register-indirect operand.
///
/// This helper function is used for instructions that have an 8-bit address field
/// and need to support both direct 8-bit addresses and register-indirect addressing.
///
/// Encoding for register-indirect:
///   - Bit 7 (MSB) is set to 1 to indicate register-indirect.
///   - Bits 6-3 encode the 4-bit register number.
///   - Bits 2-0 are unused (or can be used for a small offset if needed).
/// Encoding for direct address:
///   - Bit 7 (MSB) is 0.
///   - Bits 6-0 encode the 7-bit direct address.
///
/// # Arguments
///
/// * `operand` - The `Operand` to assemble.
/// * `symbol_table` - The symbol table for resolving labels.
///
/// # Returns
///
/// * `Result<u8, String>` - The assembled 8-bit value, or an error if the operand is invalid.
fn assemble_8bit_addr_or_reg_indirect(operand: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u8, String> {
    if let Some(op) = operand {

        match op {
            Operand::Address(addr) => Ok((*addr & 0x7F) as u8), // Direct 7-bit address
            Operand::Label(label) => {
                let addr = *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?;
                Ok((addr & 0x7F) as u8) // Direct 7-bit address from label
            }
            Operand::AddressRegister(reg) => {
                let reg_num = register_to_number(reg)?;
                // Encode register-indirect: bit 7 is flag, bits 6-3 are reg_num
                Ok(((1 << 7) | (reg_num << 3)) as u8)
            }
            Operand::Register(reg) => {
                return Err(format!("Instruction does not support register direct addressing. Use an address or label, or define a new instruction for register-indirect I/O. Encountered register: {:?}", reg));
            }
            _ => Err("Invalid operand for 8-bit address or register indirect".to_string()),
        }
    } else {
        Err("Missing operand for 8-bit address or register indirect".to_string())
    }
}



#[cfg(test)]
mod command_processor_tests {
    use super::*;
    use crate::memory::registers::Reg;

    #[test]
    fn test_parse_command_with_immediate() {
        let command = parse_command("MOVI AX, 123").unwrap();
        assert_eq!(command.opcode, "MOVI");
        assert_eq!(command.operand1, Some(Operand::Register(Reg::AX)));
        assert_eq!(command.operand2, Some(Operand::Immediate(123)));
    }

    #[test]
    fn test_parse_command_with_hex_immediate() {
        let command = parse_command("MOVI AX, 0xFF").unwrap();
        assert_eq!(command.opcode, "MOVI");
        assert_eq!(command.operand1, Some(Operand::Register(Reg::AX)));
        assert_eq!(command.operand2, Some(Operand::Immediate(255)));
    }

    #[test]
    fn test_parse_command_with_address() {
        let command = parse_command("LODW AX, [1024]").unwrap();
        assert_eq!(command.opcode, "LODW");
        assert_eq!(command.operand1, Some(Operand::Register(Reg::AX)));
        assert_eq!(command.operand2, Some(Operand::Address(1024)));
    }

    #[test]
    fn test_parse_command_with_label() {
        let command = parse_command("JMP my_label").unwrap();
        assert_eq!(command.opcode, "JMP");
        assert_eq!(command.operand1, Some(Operand::Label("my_label".to_string())));
        assert_eq!(command.operand2, None);
    }

    #[test]
    fn test_parse_command_with_two_registers() {
        let command = parse_command("MOVW AX, BX").unwrap();
        assert_eq!(command.opcode, "MOVW");
        assert_eq!(command.operand1, Some(Operand::Register(Reg::AX)));
        assert_eq!(command.operand2, Some(Operand::Register(Reg::BX)));
    }

    #[test]
    fn test_parse_label() {
        let command = parse_command("my_label:").unwrap();
        assert_eq!(command.label, Some("my_label".to_string()));
        assert_eq!(command.opcode, "");
    }

    #[test]
    fn test_parse_label_and_command() {
        let command = parse_command("my_label: MOVW AX, BX").unwrap();
        assert_eq!(command.label, Some("my_label".to_string()));
        assert_eq!(command.opcode, "MOVW");
        assert_eq!(command.operand1, Some(Operand::Register(Reg::AX)));
        assert_eq!(command.operand2, Some(Operand::Register(Reg::BX)));
    }
}