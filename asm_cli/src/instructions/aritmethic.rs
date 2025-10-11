use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;

// Note: Per user request, these instructions now perform floating-point arithmetic.
// They interpret the u32 bits in registers as f32 values.

pub fn execute_addw(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let value1_bits = cpu.registers.get(reg)?;
        let value2_bits = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => (*imm as f32).to_bits(),
            _ => return Err("ADDW requires register or immediate second operand".to_string()),
        };
        
        let value1_float = f32::from_bits(value1_bits);
        let value2_float = f32::from_bits(value2_bits);
        
        let result_float = value1_float + value2_float;
        cpu.registers.set(reg, result_float.to_bits())?;
        
        cpu.registers.update_flags_f32(result_float);
        Ok(())
    } else {
        Err("ADDW requires register first operand".to_string())
    }
}

pub fn execute_subw(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let value1_bits = cpu.registers.get(reg)?;
        let value2_bits = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => (*imm as f32).to_bits(),
            _ => return Err("SUBW requires register or immediate second operand".to_string()),
        };

        let value1_float = f32::from_bits(value1_bits);
        let value2_float = f32::from_bits(value2_bits);

        let result_float = value1_float - value2_float;
        cpu.registers.set(reg, result_float.to_bits())?;

        cpu.registers.update_flags_f32(result_float);
        Ok(())
    } else {
        Err("SUBW requires register first operand".to_string())
    }
}

pub fn execute_inc(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let value_bits = cpu.registers.get(reg)?;
        let value_float = f32::from_bits(value_bits);
        let result_float = value_float + 1.0;
        cpu.registers.set(reg, result_float.to_bits())?;

        cpu.registers.update_flags_f32(result_float);
        Ok(())
    } else {
        Err("INC requires register operand".to_string())
    }
}

pub fn execute_dec(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let value_bits = cpu.registers.get(reg)?;
        let value_float = f32::from_bits(value_bits);
        let result_float = value_float - 1.0;
        cpu.registers.set(reg, result_float.to_bits())?;

        cpu.registers.update_flags_f32(result_float);
        Ok(())
    } else {
        Err("DEC requires register operand".to_string())
    }
}

pub fn execute_neg(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let value_bits = cpu.registers.get(reg)?;
        let value_float = f32::from_bits(value_bits);
        let result_float = -value_float;
        cpu.registers.set(reg, result_float.to_bits())?;

        cpu.registers.update_flags_f32(result_float);
        Ok(())
    } else {
        Err("NEG requires register operand".to_string())
    }
}

pub fn execute_mul(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let value1_bits = cpu.registers.get(reg)?;
        let value2_bits = match op2 {
            Operand::Register(reg2) => cpu.registers.get(reg2)?,
            Operand::Immediate(imm) => (*imm as f32).to_bits(),
            _ => return Err("MUL requires register or immediate second operand".to_string()),
        };
        
        let value1_float = f32::from_bits(value1_bits);
        let value2_float = f32::from_bits(value2_bits);

        let result_float = value1_float * value2_float;
        cpu.registers.set(reg, result_float.to_bits())?;

        cpu.registers.update_flags_f32(result_float);
        Ok(())
    } else {
        Err("MUL requires register first operand".to_string())
    }
}

#[cfg(test)]
mod aritmetics_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::WorkMemory;
    use crate::utils::operands::Operand;
    use crate::memory::registers::Reg;

    #[test]
    fn add_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // ADDW AX, BX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 5.5f32.to_bits()).unwrap();
        execute_addw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 15.5);

        // ADDW AX, 10.0
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_addw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(10), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 20.0);
    }

    #[test]
    fn sub_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // SUBW AX, BX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 5.5f32.to_bits()).unwrap();
        execute_subw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 4.5);

        // SUBW AX, 10
        cpu.registers.set(&Reg::AX, 20.0f32.to_bits()).unwrap();
        execute_subw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(10), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 10.0);
    }

    #[test]
    fn mul_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // MUL AX, BX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 5.0f32.to_bits()).unwrap();
        execute_mul(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 50.0);

        // MUL AX, 10
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_mul(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(10), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 100.0);
    }

    #[test]
    fn neg_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // NEG AX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_neg(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), -10.0);
    }

    #[test]
    fn inc_and_dec_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // INC AX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_inc(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 11.0);

        // DEC AX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_dec(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 9.0);
    }
}