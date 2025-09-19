use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;

pub fn execute_not(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // NOT SRC -> Bitwise NOT on SRC 
    if let Operand::Register(reg) = op1 {
        let value = cpu.registers.get(reg)?;
        let result = !value;
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value, 0, false);
        Ok(())
    } else {
        Err("NOT requires register operand".to_string())
    }
}

pub fn execute_and(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // AND SRC, DST -> Bitwise AND in DST and SRC; 
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)?;
        let value2 = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("AND requires register or immediate second operand".to_string()),
        };
        
        let result = value1 & value2;
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value1, value2, false);
        Ok(())
    } else {
        Err("AND requires register first operand".to_string())
    }
}

pub fn execute_or(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // OR DST, SRC -> Bitwise OR in DST and SRC;  
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)?;
        let value2 = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("OR requires register or immediate second operand".to_string()),
        };
        
        let result = value1 | value2;
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value1, value2, false);
        Ok(())
    } else {
        Err("OR requires register first operand".to_string())
    }
}

pub fn execute_xor(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // XOR DST, SRC -> Bitwise XOR in SRC and DST; 
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)?;
        let value2 = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("XOR requires register or immediate second operand".to_string()),
        };
        
        let result = value1 ^ value2;
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value1, value2, false);
        Ok(())
    } else {
        Err("XOR requires register first operand".to_string())
    }
}