use crate::chips::io_device::IoDevice;
use crate::chips::cpu::CPU;
use crate::memory::main_memory::{WorkMemory, TEXT_START, DATA_START};
use crate::utils::operands::{Operand, parse_operand};
use crate::memory::registers::Reg;
use crate::instructions::io;
use crate::instructions::{
    moves,
    aritmethic, 
    bitwise, 
    compare, 
    system, 
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Command>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub opcode: String,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
    pub label: Option<String>,
    pub macro_name: Option<String>,
    pub macro_args: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Section {
    Text,
    Data,
}

#[derive(Debug, Default, Clone)]
pub struct AssembledProgram {
    pub text: Vec<u32>,
    pub data: Vec<u8>,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let comment_start = input.find(';');
    let without_comment = if let Some(index) = comment_start {
        &input[..index]
    } else {
        input
    };
    let mut parts = without_comment.trim().splitn(2, char::is_whitespace);
    let first_part = parts.next().unwrap_or("").to_string();
    
    if first_part.is_empty() {
                return Ok(Command { opcode: "".to_string(), operand1: None, operand2: None, label: None, macro_name: None, macro_args: None });    }

    let (label, remaining) = if first_part.ends_with(':') {
        (Some(first_part[..first_part.len()-1].to_string()), parts.next().unwrap_or(""))
    } else {
        (None, without_comment.trim())
    };

    let mut command_parts = remaining.splitn(2, char::is_whitespace);
    let opcode = command_parts.next().unwrap_or("").to_string();
    let operands_str = command_parts.next();

    if opcode == ".macro" {
        let mut parts = operands_str.unwrap_or("").split_whitespace();
        let name = parts.next().unwrap_or("").to_string();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
        return Ok(Command { opcode, operand1: None, operand2: None, label, macro_name: Some(name), macro_args: Some(args) });
    } else if opcode == ".endmacro" {
        return Ok(Command { opcode, operand1: None, operand2: None, label, macro_name: None, macro_args: None });
    }

    

    if opcode.starts_with('.') {
        let mut operand1 = None;
        if let Some(operands_str) = operands_str {
            if !operands_str.is_empty() {
                operand1 = parse_operand(operands_str.trim()).ok();
            }
        }
        return Ok(Command { opcode, operand1, operand2: None, label, macro_name: None, macro_args: None });
    }

    if opcode.is_empty() && label.is_some() {
        return Ok(Command { opcode: "".to_string(), operand1: None, operand2: None, label, macro_name: None, macro_args: None });
    }
    
    let mut operand1 = None;
    let mut operand2 = None;

    if let Some(operands_str) = operands_str {
        let mut operand_parts = operands_str.split(',').map(|s| s.trim());
        
        if let Some(op1_str) = operand_parts.next() {
            if !op1_str.is_empty() {
                operand1 = parse_operand(op1_str).ok();
            }
        }
        
        if let Some(op2_str) = operand_parts.next() {
            if !op2_str.is_empty() {
                operand2 = parse_operand(op2_str).ok();
            }
        }
    }
    
    Ok(Command { opcode, operand1, operand2, label, macro_name: None, macro_args: None })
}

pub fn execute_command(command: Command, cpu: &mut CPU, memory: &mut WorkMemory, io_device: &mut IoDevice) -> Result<(), String> {
    let op1 = command.operand1.clone().unwrap_or(Operand::None);
    let op2 = command.operand2.clone().unwrap_or(Operand::None);
    
    match command.opcode.to_lowercase().as_str() {
        "movi" => moves::execute_movi(cpu, &op1, &op2, memory),
        "movw" => moves::execute_movw(cpu, &op1, &op2, memory),
        "lodi" => moves::execute_lodi(cpu, &op1, &op2, memory),
        "lodw" => moves::execute_lodw(cpu, &op1, &op2, memory),
        "stri" => moves::execute_stri(cpu, &op1, &op2, memory),
        "strw" => moves::execute_strw(cpu, &op1, &op2, memory),
        "push" => moves::execute_push(cpu, &op1, &op2, memory),
        "pop" =>  moves::execute_pop(cpu, &op1, &op2, memory),
        "xcgh" => moves::execute_xcgh(cpu, &op1, &op2, memory),
        "addw" => aritmethic::execute_addw(cpu, &op1, &op2, memory),
        "dec" => aritmethic::execute_dec(cpu, &op1, &op2, memory),
        "inc" => aritmethic::execute_inc(cpu, &op1, &op2, memory),
        "neg" => aritmethic::execute_neg(cpu, &op1, &op2, memory),
        "mul" => aritmethic::execute_mul(cpu, &op1, &op2, memory),
        "subw" => aritmethic::execute_subw(cpu, &op1, &op2, memory),
        "not" => bitwise::execute_not(cpu, &op1, &op2, memory),
        "and" => bitwise::execute_and(cpu, &op1, &op2, memory),
        "or" => bitwise::execute_or(cpu, &op1, &op2, memory),
        "xor" => bitwise::execute_xor(cpu, &op1, &op2, memory),
        "cmpw" => compare::execute_cmpw(cpu, &op1, &op2, memory),
        "jmp" => compare::execute_jmp(cpu, &op1, &op2, memory),
        "call" => compare::execute_call(cpu, &op1, &op2, memory),
        "ret" => compare::execute_ret(cpu, &op1, &op2, memory),
        "je" => compare::execute_je(cpu, &op1, &op2, memory),
        "jne" => compare::execute_jne(cpu, &op1, &op2, memory),
        "jgt" => compare::execute_jgt(cpu, &op1, &op2, memory),
        "jge" => compare::execute_jge(cpu, &op1, &op2, memory),
        "jlt" => compare::execute_jlt(cpu, &op1, &op2, memory),
        "jle" => compare::execute_jle(cpu, &op1, &op2, memory),
        "js" => compare::execute_js(cpu, &op1, &op2, memory),
        "jco" => compare::execute_jco(cpu, &op1, &op2, memory),
        "in" => io::execute_in(cpu, &op1, &op2, memory, io_device),
        "out" => io::execute_out(cpu, &op1, &op2, memory, io_device),
        "halt" => system::execute_halt(cpu, &op1, &op2, memory),
        "" => Ok(()), // Label-only line
        _ => Err(format!("Unknown opcode: {}", command.opcode)),
    }
}

pub fn assemble_program(commands: &[Command], macros: &[Macro]) -> Result<AssembledProgram, String> {
    let mut symbol_table = HashMap::new();
    let mut text_address_counter = TEXT_START;
    let mut data_address_counter = DATA_START;
    let mut current_section = Section::Text;

    // Pass 1: Build symbol table
    for command in commands {
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

        if !command.opcode.is_empty() {
            let size = get_instruction_or_data_size(command)?;
            match current_section {
                Section::Text => text_address_counter += size,
                Section::Data => data_address_counter += size,
            }
        }
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
    current_section = Section::Text;
    for command in &expanded_commands {
        if command.opcode == ".text" {
            current_section = Section::Text;
            continue;
        } else if command.opcode == ".data" {
            current_section = Section::Data;
            continue;
        }

        if command.opcode.is_empty() {
            continue;
        }

        match current_section {
            Section::Text => {
                let instruction = assemble_instruction(command, &symbol_table)?;
                assembled_program.text.push(instruction);
            }
            Section::Data => {
                let data_bytes = assemble_data(command, &symbol_table)?;
                assembled_program.data.extend(data_bytes);
            }
        }
    }

    Ok(assembled_program)
}

fn get_instruction_or_data_size(command: &Command) -> Result<u32, String> {
    if command.opcode.starts_with('.') { // Directive
        match command.opcode.as_str() {
            ".word" => Ok(4),
            ".byte" => Ok(1),
            ".string" => {
                if let Some(Operand::String(s)) = &command.operand1 {
                    Ok(s.len() as u32)
                } else {
                    Err(".string directive requires a string operand".to_string())
                }
            }
            _ => Err(format!("Unknown directive: {}", command.opcode)),
        }
    } else { // Instruction
        Ok(4) // All instructions are 4 bytes
    }
}

fn assemble_data(command: &Command, _symbol_table: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
    match command.opcode.as_str() {
        ".word" => {
            if let Some(Operand::Immediate(value)) = command.operand1 {
                Ok((value as u32).to_le_bytes().to_vec())
            } else {
                Err(".word directive requires an immediate value".to_string())
            }
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
                Ok(s.as_bytes().to_vec())
            } else {
                Err(".string directive requires a string operand".to_string())
            }
        }
        _ => Err(format!("Unknown directive in data section: {}", command.opcode)),
    }
}

fn assemble_instruction(command: &Command, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    let opcode = command.opcode.to_lowercase();
    let op1 = command.operand1.as_ref();
    let op2 = command.operand2.as_ref();

    match opcode.as_str() {
        "movi" => assemble_reg_imm(0x01, op1, op2),
        "movw" => assemble_reg_reg_or_reg_imm(0x02, op1, op2),
        "lodi" => assemble_reg_imm(0x03, op1, op2),
        "lodw" => assemble_reg_addr(0x04, op1, op2, symbol_table),
        "stri" => assemble_addr_imm(0x05, op1, op2, symbol_table),
        "strw" => assemble_addr_reg_or_addr_imm(0x06, op1, op2, symbol_table),
        "push" => assemble_reg_or_imm(0x07, op1),
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
        "in" => assemble_reg(0x50, op1),
        "out" => assemble_reg_or_imm(0x51, op1),
        "halt" => Ok(0xFF << 24),
        _ => Err(format!("Unsupported instruction: {}", opcode)),
    }
}

fn register_to_number(reg: &Reg) -> Result<u8, String> {
    match reg {
        Reg::AX => Ok(0), Reg::BX => Ok(1), Reg::CX => Ok(2), Reg::DX => Ok(3),
        Reg::EX => Ok(4), Reg::FX => Ok(5), Reg::GX => Ok(6), Reg::HX => Ok(7),
        Reg::SP => Ok(8), Reg::BP => Ok(9), Reg::SI => Ok(10), Reg::DI => Ok(11),
        Reg::PC => Ok(12), Reg::FLAGS => Ok(13),
    }
}

// Helper functions for assembling different operand combinations

fn assemble_reg_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>) -> Result<u32, String> {
    if let (Some(Operand::Register(reg)), Some(Operand::Immediate(imm))) = (op1, op2) {
        let reg_num = register_to_number(reg)?;
        Ok((opcode << 24) | ((reg_num as u32) << 16) | (*imm as u32 & 0xFFFF))
    } else {
        Err("Invalid operands for register-immediate instruction".to_string())
    }
}

fn assemble_reg_reg(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>) -> Result<u32, String> {
    if let (Some(Operand::Register(reg1)), Some(Operand::Register(reg2))) = (op1, op2) {
        let reg1_num = register_to_number(reg1)?;
        let reg2_num = register_to_number(reg2)?;
        Ok((opcode << 24) | ((reg1_num as u32) << 16) | ((reg2_num as u32) << 8))
    } else {
        Err("Invalid operands for register-register instruction".to_string())
    }
}

fn assemble_reg_reg_or_reg_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>) -> Result<u32, String> {
    if let Some(Operand::Register(reg1)) = op1 {
        let reg1_num = register_to_number(reg1)?;
        match op2 {
            Some(Operand::Register(reg2)) => {
                let reg2_num = register_to_number(reg2)?;
                Ok((opcode << 24) | ((reg1_num as u32) << 16) | ((reg2_num as u32) << 8) | 1) // Set a bit to indicate reg-reg
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

fn assemble_reg(opcode: u32, op1: Option<&Operand>) -> Result<u32, String> {
    if let Some(Operand::Register(reg)) = op1 {
        let reg_num = register_to_number(reg)?;
        Ok((opcode << 24) | ((reg_num as u32) << 16))
    } else {
        Err("Invalid operand for register instruction".to_string())
    }
}

fn assemble_addr(opcode: u32, op1: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if let Some(operand) = op1 {
        let addr = match operand {
            Operand::Address(addr) => *addr,
            Operand::Immediate(imm) => *imm as u32,
            Operand::Label(label) => *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
            _ => return Err("Invalid operand for address instruction".to_string()),
        };
        Ok((opcode << 24) | (addr & 0xFFFFFF))
    } else {
        Err("Missing operand for address instruction".to_string())
    }
}

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

fn assemble_addr_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if let Some(operand) = op1 {
        let addr = match operand {
            Operand::Address(addr) => *addr,
            Operand::Label(label) => *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
            _ => return Err("Invalid first operand for address-immediate instruction".to_string()),
        };
        if let Some(Operand::Immediate(imm)) = op2 {
            Ok((opcode << 24) | ((addr & 0xFF) << 16) | (*imm as u32 & 0xFFFF))
        } else {
            Err("Invalid second operand for address-immediate instruction".to_string())
        }
    } else {
        Err("Missing first operand for address-immediate instruction".to_string())
    }
}

fn assemble_addr_reg_or_addr_imm(opcode: u32, op1: Option<&Operand>, op2: Option<&Operand>, symbol_table: &HashMap<String, u32>) -> Result<u32, String> {
    if let Some(operand) = op1 {
        let addr = match operand {
            Operand::Address(addr) => *addr,
            Operand::Label(label) => *symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
            _ => return Err("Invalid first operand".to_string()),
        };
        match op2 {
            Some(Operand::Register(reg)) => {
                let reg_num = register_to_number(reg)?;
                Ok((opcode << 24) | ((addr & 0xFF) << 16) | ((reg_num as u32) << 8) | 1)
            }
            Some(Operand::Immediate(imm)) => {
                Ok((opcode << 24) | ((addr & 0xFF) << 16) | (*imm as u32 & 0xFFFF))
            }
            _ => Err("Invalid second operand".to_string()),
        }
    } else {
        Err("Invalid first operand".to_string())
    }
}

fn assemble_reg_or_imm(opcode: u32, op1: Option<&Operand>) -> Result<u32, String> {
    if let Some(operand) = op1 {
        match operand {
            Operand::Register(reg) => {
                let reg_num = register_to_number(reg)?;
                Ok((opcode << 24) | ((reg_num as u32) << 16) | 1)
            }
            Operand::Immediate(imm) => {
                Ok((opcode << 24) | (*imm as u32 & 0xFFFF))
            }
            _ => Err("Invalid operand".to_string()),
        }
    } else {
        Err("Missing operand".to_string())
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
