// cpu/instructions/aritmetics.rs
use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;

pub fn execute_addw(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // ADDW DST, SRC -> Add SRC to DST; 
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)?;
        let value2 = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("ADDW requires register or immediate second operand".to_string()),
        };
        
        let result = value1.wrapping_add(value2);
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value1, value2, false);
        Ok(())
    } else {
        Err("ADDW requires register first operand".to_string())
    }
}

pub fn execute_addi(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // ADDI DST, SRC -> ADD immediate SRC to DST 
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)? as u16;
        let value2 = match op2 {
            Operand::Immediate(imm) => *imm,
            _ => return Err("ADDI requires immediate second operand".to_string()),
        };
        
        let result = value1.wrapping_add(value2) as u32;
        cpu.registers.set(reg, result)?;
        
        // Update flags with 16-bit values
        cpu.registers.update_flags(result, value1 as u32, value2 as u32, false);
        Ok(())
    } else {
        Err("ADDI requires register first operand".to_string())
    }
}

pub fn execute_subw(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // SUBW DST, SRC -> Subtract SRC from DST; 
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)?;
        let value2 = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("SUBW requires register or immediate second operand".to_string()),
        };
        
        let result = value1.wrapping_sub(value2);
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value1, value2, true);
        Ok(())
    } else {
        Err("SUBW requires register first operand".to_string())
    }
}

pub fn execute_subi(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // SUBI DST, SRC -> Subtracte immediate value from source; 
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)? as u16;
        let value2 = match op2 {
            Operand::Immediate(imm) => *imm,
            _ => return Err("SUBI requires immediate second operand".to_string()),
        };
        
        let result = value1.wrapping_sub(value2) as u32;
        cpu.registers.set(reg, result)?;
        
        // Update flags with 16-bit values
        cpu.registers.update_flags(result, value1 as u32, value2 as u32, true);
        Ok(())
    } else {
        Err("SUBI requires register first operand".to_string())
    }
}

pub fn execute_inc(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // INC SRC -> Increment SRC; 
    if let Operand::Register(reg) = op1 {
        let value = cpu.registers.get(reg)?;
        let result = value.wrapping_add(1);
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value, 1, false);
        Ok(())
    } else {
        Err("INC requires register operand".to_string())
    }
}

pub fn execute_dec(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // DEC SRC -> Decrement SRC; 
    if let Operand::Register(reg) = op1 {
        let value = cpu.registers.get(reg)?;
        let result = value.wrapping_sub(1);
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value, 1, true);
        Ok(())
    } else {
        Err("DEC requires register operand".to_string())
    }
}

pub fn execute_neg(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // NEG SRC -> Negate register value, 
    if let Operand::Register(reg) = op1 {
        let value = cpu.registers.get(reg)?;
        let result = value.wrapping_neg();
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value, 0, true);
        Ok(())
    } else {
        Err("NEG requires register operand".to_string())
    }
}

pub fn execute_mul(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // MUL DST, SRC -> Multiply reg by immediate value. 
    if let Operand::Register(reg) = op1 {
        let value1 = cpu.registers.get(reg)?;
        let value2 = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("MUL requires register or immediate second operand".to_string()),
        };
        
        let result = value1.wrapping_mul(value2);
        cpu.registers.set(reg, result)?;
        
        // Update flags
        cpu.registers.update_flags(result, value1, value2, false);
        Ok(())
    } else {
        Err("MUL requires register first operand".to_string())
    }
}