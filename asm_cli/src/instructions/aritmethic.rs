use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::assembler::operands::Operand;

/// Executes the `ADDW` instruction, adding a word value to a register.
///
/// The value from `op2` (register or immediate) is added to the value in `op1` (destination register).
/// Both values are treated as `f32` floating-point numbers. The result is stored back in `op1`.
/// CPU flags are updated based on the result.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which must be an `Operand::Register`.
/// * `op2` - The second operand, which can be an `Operand::Register` or `Operand::Immediate`.
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
///
/// # Examples
///
/// ```
/// use arc_emulator::chips::cpu::CPU;
/// use arc_emulator::memory::main_memory::WorkMemory;
/// use arc_emulator::utils::assembler::operands::Operand;
/// use arc_emulator::memory::registers::Reg;
/// use arc_emulator::instructions::aritmethic;
///
/// let mut cpu = CPU::new();
/// let mut memory = WorkMemory::new(1024);
///
/// // Initialize AX and BX
/// cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
/// cpu.registers.set(&Reg::BX, 5.5f32.to_bits()).unwrap();
///
/// // Add BX to AX (AX = AX + BX)
/// aritmethic::execute_addw_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
/// assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 15.5);
///
/// // Add immediate value 2.0 to AX (AX = AX + 2.0)
/// aritmethic::execute_addw_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(2), &mut memory).unwrap();
/// assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 17.5);
/// ```
pub fn execute_addw_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
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

pub fn execute_subw_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
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

pub fn execute_inc_instruction(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
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

pub fn execute_dec_instruction(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
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

pub fn execute_neg_instruction(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
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

pub fn execute_mul_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
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
    use crate::utils::assembler::operands::Operand;
    use crate::memory::registers::Reg;

    #[test]
    fn add_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // ADDW AX, BX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 5.5f32.to_bits()).unwrap();
        execute_addw_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 15.5);

        // ADDW AX, 10.0
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_addw_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(10), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 20.0);
    }

    #[test]
    fn sub_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // SUBW AX, BX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 5.5f32.to_bits()).unwrap();
        execute_subw_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 4.5);

        // SUBW AX, 10
        cpu.registers.set(&Reg::AX, 20.0f32.to_bits()).unwrap();
        execute_subw_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(10), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 10.0);
    }

    #[test]
    fn mul_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // MUL AX, BX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 5.0f32.to_bits()).unwrap();
        execute_mul_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 50.0);

        // MUL AX, 10
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_mul_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(10), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 100.0);
    }

    #[test]
    fn neg_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // NEG AX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_neg_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), -10.0);
    }

    #[test]
    fn inc_and_dec_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // INC AX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_inc_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 11.0);

        // DEC AX
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        execute_dec_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 9.0);
    }
}
