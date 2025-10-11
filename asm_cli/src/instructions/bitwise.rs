use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;

pub fn execute_not(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // NOT SRC -> Bitwise NOT on SRC 
    if let Operand::Register(reg) = op1 {
        let value = cpu.registers.get(reg)?;
        let result = !value;
        cpu.registers.set(reg, result)?;
        
        // Update flags for integer operation
        cpu.registers.update_flags_u32(result, value, 0, false);
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
        
        // Update flags for integer operation
        cpu.registers.update_flags_u32(result, value1, value2, false);
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
        
        // Update flags for integer operation
        cpu.registers.update_flags_u32(result, value1, value2, false);
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
        
        // Update flags for integer operation
        cpu.registers.update_flags_u32(result, value1, value2, false);
        Ok(())
    } else {
        Err("XOR requires register first operand".to_string())
    }
}

#[cfg(test)]
mod bitwise_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::WorkMemory;
    use crate::utils::operands::Operand;
    use crate::memory::registers::Reg;

    #[test]
    fn not_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 0b10101010).unwrap();
        execute_not(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), !0b10101010);
    }

    #[test]
    fn and_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // AND AX, BX
        cpu.registers.set(&Reg::AX, 0b1100).unwrap();
        cpu.registers.set(&Reg::BX, 0b1010).unwrap();
        execute_and(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1000);

        // AND AX, 0b1111
        cpu.registers.set(&Reg::AX, 0b1010).unwrap();
        execute_and(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(0b1111), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1010);
    }

    #[test]
    fn or_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // OR AX, BX
        cpu.registers.set(&Reg::AX, 0b1100).unwrap();
        cpu.registers.set(&Reg::BX, 0b1010).unwrap();
        execute_or(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1110);

        // OR AX, 0b0001
        cpu.registers.set(&Reg::AX, 0b1010).unwrap();
        execute_or(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(0b0001), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1011);
    }

    #[test]
    fn xor_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // XOR AX, BX
        cpu.registers.set(&Reg::AX, 0b1100).unwrap();
        cpu.registers.set(&Reg::BX, 0b1010).unwrap();
        execute_xor(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b0110);

        // XOR AX, 0b1111
        cpu.registers.set(&Reg::AX, 0b1010).unwrap();
        execute_xor(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(0b1111), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b0101);
    }
}