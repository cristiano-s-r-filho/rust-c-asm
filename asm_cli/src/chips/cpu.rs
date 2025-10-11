use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::{Registers, Reg};

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
        }
    }

    pub fn load_program(&mut self, _program: &str) -> Result<(), String> {
        // TODO: Implement program loading logic
        Ok(())
    }

    // Execute a single instruction from memory
    pub fn execute_instruction(&mut self, _memory: &mut WorkMemory, instruction: u32) -> Result<(), String> {
        // Decode instruction (simplified for now)
        let opcode = (instruction >> 24) as u8;
        let _operand1_value = (instruction >> 16) as u16;
        let operand2_value = instruction as u16;
        
        // For now, we'll use a simple mapping
        // In a real implementation, you'd have a proper instruction decoder
        match opcode {
            0x01 => {
                // MOVI-like instruction
                let reg = Reg::AX; // Placeholder
                let value = operand2_value as u32;
                self.registers.set(&reg, value)
            },
            0x02 => {
                // ADD-like instruction
                let reg1 = Reg::AX; // Placeholder
                let reg2 = Reg::BX; // Placeholder
                
                let val1 = self.registers.get(&reg1)?;
                let val2 = self.registers.get(&reg2)?;
                let result = val1.wrapping_add(val2);
                
                self.registers.set(&reg1, result)?;
                self.registers.update_flags_u32(result, val1, val2, false);
                Ok(())
            },
            // Add more opcodes here...
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
        while self.registers.pc < memory.size as u32 {
            self.step(memory)?;
        }
        Ok(())
    }
}