//! # CPU Module
//! 
//! This module defines the `CPU` struct, which simulates the central processing unit
//! of the ARC computer. It handles register management, instruction fetching,
//! decoding, and execution.

use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::{Registers, Reg};
use crate::instructions::{moves, aritmethic, bitwise, compare, system, control, io};
use crate::utils::assembler::operands::Operand;

/// Converts a numeric representation to a `Reg` enum variant.
///
/// This helper function is used during instruction decoding to map
/// register numbers embedded in opcodes to their corresponding `Reg` enum.
///
/// # Arguments
///
/// * `num` - A `u8` representing the register number.
///
/// # Returns
///
/// * `Result<Reg, String>` - `Ok(Reg)` if the number maps to a valid register,
///   or `Err(String)` if the number is invalid.
fn number_to_register(num: u8) -> Result<Reg, String> {
    match num {
        0 => Ok(Reg::AX), 1 => Ok(Reg::BX), 2 => Ok(Reg::CX), 3 => Ok(Reg::DX),
        4 => Ok(Reg::EX), 5 => Ok(Reg::FX), 6 => Ok(Reg::GX), 7 => Ok(Reg::HX),
        8 => Ok(Reg::SP), 9 => Ok(Reg::BP), 10 => Ok(Reg::SI), 11 => Ok(Reg::DI),
        12 => Ok(Reg::PC), 13 => Ok(Reg::FLAGS),
        _ => Err(format!("Invalid register number: {}", num)),
    }
}

/// Decodes the 24-bit address field of an instruction, determining if it's a direct address
/// or a register-indirect address.
///
/// # Arguments
///
/// * `_cpu` - A reference to the `CPU` state, needed to resolve register values.
/// * `instruction_addr_field` - The 24-bit address field from the instruction.
///
/// # Returns
///
/// * `Result<Operand, String>` - `Ok(Operand::Address(u32))` for a direct address,
///   or `Ok(Operand::AddressRegister(Reg))` for a register-indirect address.
fn decode_address_operand(_cpu: &CPU, instruction_addr_field: u32) -> Result<Operand, String> {
    // Check if the highest bit (bit 23) is set, indicating register-indirect addressing
    if (instruction_addr_field >> 23) & 1 == 1 {
        let reg_num = ((instruction_addr_field >> 19) & 0xF) as u8; // Extract 4-bit register number
        let reg = number_to_register(reg_num)?;
        Ok(Operand::AddressRegister(reg))
    } else {
        // Direct address
        Ok(Operand::Address(instruction_addr_field & 0xFFFFFF))
    }
}

/// Decodes an 8-bit address field, determining if it's a direct 7-bit address
/// or a register-indirect address.
///
/// # Arguments
///
/// * `_cpu` - A reference to the `CPU` state (unused in this function).
/// * `instruction_addr_field` - The 8-bit address field from the instruction.
///
/// # Returns
///
/// * `Result<Operand, String>` - `Ok(Operand::Address(u32))` for a direct address,
///   or `Ok(Operand::AddressRegister(Reg))` for a register-indirect address.
fn decode_8bit_address_operand(_cpu: &CPU, instruction_addr_field: u32) -> Result<Operand, String> {
    // Check if the highest bit (bit 7) is set, indicating register-indirect addressing
    if (instruction_addr_field >> 7) & 1 == 1 {
        let reg_num = ((instruction_addr_field >> 3) & 0xF) as u8; // Extract 4-bit register number (bits 6-3)
        let reg = number_to_register(reg_num)?;
        Ok(Operand::AddressRegister(reg))
    } else {
        // Direct 7-bit address
        Ok(Operand::Address(instruction_addr_field & 0x7F))
    }
}

/// Represents the Central Processing Unit (CPU) of the ARC computer.
///
/// The CPU contains the registers and manages the execution flow of programs.
#[derive(Debug, Clone)]
pub struct CPU {
    /// The set of registers available to the CPU.
    pub registers: Registers,
    /// A flag indicating whether the CPU is halted.
    pub halted: bool,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    /// Creates a new `CPU` instance with default register values and not halted.
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            halted: false,
        }
    }

    /// Resets the CPU to its initial state.
    ///
    /// This includes resetting all registers and unhalting the CPU.
    pub fn reset(&mut self) {
        self.registers.reset();
        self.halted = false;
    }

    /// Loads a program into the CPU (currently a stub, actual loading is external).
    ///
    /// # Arguments
    ///
    /// * `_program` - A string slice representing the program to load.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - Always `Ok(())` as program loading is handled externally.
    pub fn load_program(&mut self, _program: &str) -> Result<(), String> {
        Ok(())
    }

    /// Executes a single instruction provided as a `u32` opcode.
    ///
    /// This function decodes the instruction and calls the appropriate
    /// instruction handler from the `instructions` module.
    ///
    /// # Arguments
    ///
    /// * `memory` - A mutable reference to the `WorkMemory` where the program and data reside.
    /// * `instruction` - The `u32` instruction to execute.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful execution, or `Err(String)` if an error occurs
    ///   (e.g., unknown opcode, invalid register).
    pub fn execute_instruction(&mut self, memory: &mut WorkMemory, instruction: u32) -> Result<(), String> {
        let opcode = (instruction >> 24) as u8;

        match opcode {
            0x01 | 0x03 => { // MOVI, LODI
                let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let imm = (instruction & 0xFFFF) as u32;
                let op1 = Operand::Register(reg);
                let op2 = Operand::Immediate(imm);
                let result = match opcode {
                    0x01 => moves::execute_movi(self, &op1, &op2, memory),
                    0x03 => moves::execute_lodi(self, &op1, &op2, memory),
                    _ => unreachable!(),
                };
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x02 | 0x10 | 0x11 | 0x12 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25 | 0x30 => { // MOVW, ADDW, SUBW, MUL, AND, OR, XOR, SHL, SHR, CMPW
                let reg1 = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let op1 = Operand::Register(reg1);
                let op2 = if (instruction & 1) == 1 {
                    let reg2 = number_to_register(((instruction >> 8) & 0xFF) as u8)?;
                    Operand::Register(reg2)
                } else {
                    let imm = (instruction & 0xFFFF) as u32; // Changed to u32
                    Operand::Immediate(imm)
                };
                let result = match opcode {
                    0x02 => moves::execute_movw(self, &op1, &op2, memory),
                    0x10 => aritmethic::execute_addw_instruction(self, &op1, &op2, memory),
                    0x11 => aritmethic::execute_subw_instruction(self, &op1, &op2, memory),
                    0x12 => aritmethic::execute_mul_instruction(self, &op1, &op2, memory),
                    0x21 => bitwise::execute_and_instruction(self, &op1, &op2, memory),
                    0x22 => bitwise::execute_or_instruction(self, &op1, &op2, memory),
                    0x23 => bitwise::execute_xor_instruction(self, &op1, &op2, memory),
                    0x24 => bitwise::execute_shl_instruction(self, &op1, &op2, memory),
                    0x25 => bitwise::execute_shr_instruction(self, &op1, &op2, memory),
                    0x30 => compare::execute_cmpw(self, &op1, &op2, memory),
                    _ => unreachable!(),
                };
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x04 => { // LODW
                let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let addr = instruction & 0xFFFF;
                let op1 = Operand::Register(reg);
                let op2 = Operand::Address(addr);
                let result = moves::execute_lodw(self, &op1, &op2, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x05 => { // STRI
                let addr = (instruction >> 16) & 0xFF;
                let imm = (instruction & 0xFFFF) as u32;
                let op1 = Operand::Address(addr);
                let op2 = Operand::Immediate(imm);
                let result = moves::execute_stri(self, &op1, &op2, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x06 => { // STRW
                let addr = (instruction >> 16) & 0xFF;
                let op1 = Operand::Address(addr);
                let op2 = if (instruction & 1) == 1 {
                    let reg = number_to_register(((instruction >> 8) & 0xFF) as u8)?;
                    Operand::Register(reg)
                } else {
                    let imm = (instruction & 0xFFFF) as u32;
                    Operand::Immediate(imm)
                };
                let result = moves::execute_strw(self, &op1, &op2, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x54 => { // INSW
                let addr_field = (instruction >> 16) & 0xFF; // 8-bit address field
                let op1 = decode_8bit_address_operand(self, addr_field)?; // Use new helper
                let imm = (instruction & 0xFFFF) as u32; // I/O slot, if applicable
                let op2 = Operand::Immediate(imm);
                let result = io::execute_insw(self, &op1, &op2, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x55 => { // OUTW
                let addr_field = (instruction >> 16) & 0xFF; // 8-bit address field
                let op1 = decode_8bit_address_operand(self, addr_field)?; // Use new helper
                let op2 = if (instruction & 1) == 1 {
                    let reg = number_to_register(((instruction >> 8) & 0xFF) as u8)?;
                    Operand::Register(reg)
                } else {
                    let imm = (instruction & 0xFFFF) as u32; // I/O slot, if applicable
                    Operand::Immediate(imm)
                };
                let result = io::execute_outw(self, &op1, &op2, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x07 => { // PUSH
                let op1 = if (instruction & 1) == 1 {
                    let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                    Operand::Register(reg)
                } else {
                    let imm = (instruction & 0xFFFF) as u32;
                    Operand::Immediate(imm)
                };
                let result = moves::execute_push(self, &op1, &Operand::None, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x51 => { // OUT
                let addr_field = instruction & 0xFFFFFF;
                let op1 = decode_address_operand(self, addr_field)?; // Use helper
                let result = io::execute_out(self, &op1, &Operand::None, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x08 | 0x13 | 0x14 | 0x15 | 0x20 => { // POP, INC, DEC, NEG, NOT
                let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let op1 = Operand::Register(reg);
                let result = match opcode {
                    0x08 => moves::execute_pop(self, &op1, &Operand::None, memory),
                    0x13 => aritmethic::execute_inc_instruction(self, &op1, &Operand::None, memory),
                    0x14 => aritmethic::execute_dec_instruction(self, &op1, &Operand::None, memory),
                    0x15 => aritmethic::execute_neg_instruction(self, &op1, &Operand::None, memory),
                    0x20 => bitwise::execute_not_instruction(self, &op1, &Operand::None, memory),
                    _ => unreachable!(),
                };
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x09 => { // XCGH
                let reg1 = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let reg2 = number_to_register(((instruction >> 8) & 0xFF) as u8)?;
                let op1 = Operand::Register(reg1);
                let op2 = Operand::Register(reg2);
                let result = moves::execute_xcgh(self, &op1, &op2, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x40 | 0x41 | 0x43 | 0x44 | 0x45 | 0x46 | 0x47 | 0x48 | 0x49 | 0x4A => { // JMP, CALL, JE, JNE, JGT, JGE, JLT, JLE, JS, JCO
                let addr_field = instruction & 0xFFFFFF;
                let op1 = decode_address_operand(self, addr_field)?;
                let result = match opcode {
                    0x40 => compare::execute_jmp(self, &op1, &Operand::None, memory),
                    0x41 => compare::execute_call(self, &op1, &Operand::None, memory),
                    0x43 => compare::execute_je(self, &op1, &Operand::None, memory),
                    0x44 => compare::execute_jne(self, &op1, &Operand::None, memory),
                    0x45 => compare::execute_jgt(self, &op1, &Operand::None, memory),
                    0x46 => compare::execute_jge(self, &op1, &Operand::None, memory),
                    0x47 => compare::execute_jlt(self, &op1, &Operand::None, memory),
                    0x48 => compare::execute_jle(self, &op1, &Operand::None, memory),
                    0x49 => compare::execute_js(self, &op1, &Operand::None, memory),
                    0x4A => compare::execute_jco(self, &op1, &Operand::None, memory),
                    _ => unreachable!(),
                };
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x50 => { // IN
                let addr_field = instruction & 0xFFFFFF;
                let op1 = decode_address_operand(self, addr_field)?;
                let result = io::execute_in(self, &op1, &Operand::None, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x52 => { // INSI
                let addr_field = instruction & 0xFFFFFF;
                let op1 = decode_address_operand(self, addr_field)?;
                let result = io::execute_insi(self, &op1, &Operand::None, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x42 => { // RET
                let result = compare::execute_ret(self, &Operand::None, &Operand::None, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0x60 | 0x61 => { // SETF, CLRF
                let flag_id = (instruction & 0xFF) as u8;
                let op1 = Operand::Flag(flag_id);
                let result = match opcode {
                    0x60 => control::execute_setf(self, &op1, &Operand::None, memory),
                    0x61 => control::execute_clrf(self, &op1, &Operand::None, memory),
                    _ => unreachable!(),
                };
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            0xFF => { // HALT
                let result = system::execute_halt(self, &Operand::None, &Operand::None, memory);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(())
            },
            _ => Err(format!("Unknown opcode: {:#04x}", opcode)),
        }
    }

    /// Fetches the next instruction from memory, increments the program counter, and executes it.
    ///
    /// # Arguments
    ///
    /// * `memory` - A mutable reference to the `WorkMemory`.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful step, or `Err(String)` if an error occurs.
    pub fn step(&mut self, memory: &mut WorkMemory) -> Result<(), String> {
        let instruction = memory.read_instruction(self.registers.pc)?;
        self.registers.pc += 4;
        self.execute_instruction(memory, instruction)
    }

    /// Runs the CPU continuously until a HALT instruction is encountered or an error occurs.
    ///
    /// # Arguments
    ///
    /// * `memory` - A mutable reference to the `WorkMemory`.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful program completion, or `Err(String)` if an error occurs.
    pub fn run(&mut self, memory: &mut WorkMemory) -> Result<(), String> {
        while !self.halted && self.registers.pc < memory.size as u32 {
            self.step(memory)?;
        }
        Ok(())
    }
}