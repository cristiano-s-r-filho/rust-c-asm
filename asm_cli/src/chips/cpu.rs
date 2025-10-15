use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::{Registers, Reg};
use crate::instructions::{moves, aritmethic, bitwise, compare, system};
use crate::utils::operands::Operand;

// Helper to convert number to register
fn number_to_register(num: u8) -> Result<Reg, String> {
    match num {
        0 => Ok(Reg::AX), 1 => Ok(Reg::BX), 2 => Ok(Reg::CX), 3 => Ok(Reg::DX),
        4 => Ok(Reg::EX), 5 => Ok(Reg::FX), 6 => Ok(Reg::GX), 7 => Ok(Reg::HX),
        8 => Ok(Reg::SP), 9 => Ok(Reg::BP), 10 => Ok(Reg::SI), 11 => Ok(Reg::DI),
        12 => Ok(Reg::PC), 13 => Ok(Reg::FLAGS),
        _ => Err(format!("Invalid register number: {}", num)),
    }
}

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            halted: false,
        }
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.halted = false;
    }

    pub fn load_program(&mut self, _program: &str) -> Result<(), String> {
        // This is a stub. Program loading is handled in tui.rs
        Ok(())
    }

    // Execute a single instruction from memory
    pub fn execute_instruction(&mut self, memory: &mut WorkMemory, instruction: u32) -> Result<(), String> {
        let opcode = (instruction >> 24) as u8;
        
        // Decode operands based on opcode
        match opcode {
            // Register-Immediate instructions
            0x01 | 0x03 => { // MOVI, LODI
                let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let imm = (instruction & 0xFFFF) as u16;
                let op1 = Operand::Register(reg);
                let op2 = Operand::Immediate(imm);
                match opcode {
                    0x01 => moves::execute_movi(self, &op1, &op2, memory),
                    0x03 => moves::execute_lodi(self, &op1, &op2, memory),
                    _ => unreachable!(),
                }
            },
            // Register-Register or Register-Immediate
            0x02 | 0x10 | 0x11 | 0x12 | 0x21 | 0x22 | 0x23 | 0x30 => { // MOVW, ADDW, SUBW, MUL, AND, OR, XOR, CMPW
                let reg1 = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let op1 = Operand::Register(reg1);
                let op2 = if (instruction & 1) == 1 { // Reg-Reg
                    let reg2 = number_to_register(((instruction >> 8) & 0xFF) as u8)?;
                    Operand::Register(reg2)
                } else { // Reg-Imm
                    let imm = (instruction & 0xFFFF) as u16;
                    Operand::Immediate(imm)
                };
                match opcode {
                    0x02 => moves::execute_movw(self, &op1, &op2, memory),
                    0x10 => aritmethic::execute_addw(self, &op1, &op2, memory),
                    0x11 => aritmethic::execute_subw(self, &op1, &op2, memory),
                    0x12 => aritmethic::execute_mul(self, &op1, &op2, memory),
                    0x21 => bitwise::execute_and(self, &op1, &op2, memory),
                    0x22 => bitwise::execute_or(self, &op1, &op2, memory),
                    0x23 => bitwise::execute_xor(self, &op1, &op2, memory),
                    0x30 => compare::execute_cmpw(self, &op1, &op2, memory),
                    _ => unreachable!(),
                }
            },
            // Register-Address
            0x04 => { // LODW
                let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let addr = instruction & 0xFFFF;
                let op1 = Operand::Register(reg);
                let op2 = Operand::Address(addr);
                moves::execute_lodw(self, &op1, &op2, memory)
            },
            // Address-Immediate
            0x05 => { // STRI
                let addr = (instruction >> 16) & 0xFF;
                let imm = (instruction & 0xFFFF) as u16;
                let op1 = Operand::Address(addr);
                let op2 = Operand::Immediate(imm);
                moves::execute_stri(self, &op1, &op2, memory)
            },
            // Address-Register or Address-Immediate
            0x06 => { // STRW
                let addr = (instruction >> 16) & 0xFF;
                let op1 = Operand::Address(addr);
                let op2 = if (instruction & 1) == 1 { // Addr-Reg
                    let reg = number_to_register(((instruction >> 8) & 0xFF) as u8)?;
                    Operand::Register(reg)
                } else { // Addr-Imm
                    let imm = (instruction & 0xFFFF) as u16;
                    Operand::Immediate(imm)
                };
                moves::execute_strw(self, &op1, &op2, memory)
            },
            // Register or Immediate
            0x07 => { // PUSH
                let op1 = if (instruction & 1) == 1 { // Register
                    let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                    Operand::Register(reg)
                } else { // Immediate
                    let imm = (instruction & 0xFFFF) as u16;
                    Operand::Immediate(imm)
                };
                moves::execute_push(self, &op1, &Operand::None, memory)
            },
            // Register only
            0x08 | 0x13 | 0x14 | 0x15 | 0x20 => { // POP, INC, DEC, NEG, NOT
                let reg = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let op1 = Operand::Register(reg);
                match opcode {
                    0x08 => moves::execute_pop(self, &op1, &Operand::None, memory),
                    0x13 => aritmethic::execute_inc(self, &op1, &Operand::None, memory),
                    0x14 => aritmethic::execute_dec(self, &op1, &Operand::None, memory),
                    0x15 => aritmethic::execute_neg(self, &op1, &Operand::None, memory),
                    0x20 => bitwise::execute_not(self, &op1, &Operand::None, memory),
                    _ => unreachable!(),
                }
            },
            // Register-Register
            0x09 => { // XCGH
                let reg1 = number_to_register(((instruction >> 16) & 0xFF) as u8)?;
                let reg2 = number_to_register(((instruction >> 8) & 0xFF) as u8)?;
                let op1 = Operand::Register(reg1);
                let op2 = Operand::Register(reg2);
                moves::execute_xcgh(self, &op1, &op2, memory)
            },
            // Address
            0x40 | 0x41 | 0x43 | 0x44 | 0x45 | 0x46 | 0x47 | 0x48 | 0x49 | 0x4A => { // JMP, CALL, JE, JNE, JGT, JGE, JLT, JLE, JS, JCO
                let addr = instruction & 0xFFFFFF;
                let op1 = Operand::Address(addr);
                match opcode {
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
                }
            },
            // No operand
            0x42 => { // RET
                compare::execute_ret(self, &Operand::None, &Operand::None, memory)
            },
            0xFF => { // HALT
                system::execute_halt(self, &Operand::None, &Operand::None, memory)
            },
            _ => Err(format!("Unknown opcode: {:#04x}", opcode)),
        }
    }

    // Fetch and execute the next instruction
    pub fn step(&mut self, memory: &mut WorkMemory) -> Result<(), String> {
        let instruction = memory.read_instruction(self.registers.pc)?;
        self.registers.pc += 4; // Move to next instruction (4 bytes per instruction)
        self.execute_instruction(memory, instruction)
    }

    // Run until program completion or error
    pub fn run(&mut self, memory: &mut WorkMemory) -> Result<(), String> {
        while !self.halted && self.registers.pc < memory.size as u32 {
            self.step(memory)?;
        }
        Ok(())
    }
}