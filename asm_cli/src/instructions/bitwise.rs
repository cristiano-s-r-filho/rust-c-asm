use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::assembler::operands::Operand;

/// Executes the `AND` instruction, performing a bitwise AND operation.
///
/// The value from `op2` (register or immediate) is bitwise ANDed with the value in `op1` (destination register).
/// The result is stored back in `op1`.
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
/// use arc_emulator::instructions::bitwise;
///
/// let mut cpu = CPU::new();
/// let mut memory = WorkMemory::new(1024);
///
/// // Initialize AX and BX
/// cpu.registers.set(&Reg::AX, 0b1100).unwrap(); // AX = 12
/// cpu.registers.set(&Reg::BX, 0b1010).unwrap(); // BX = 10
///
/// // Perform AX = AX AND BX
/// bitwise::execute_and_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
/// assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1000); // AX = 8
///
/// // Reset AX and perform AX = AX AND 0b0011
/// cpu.registers.set(&Reg::AX, 0b1100).unwrap(); // AX = 12
/// bitwise::execute_and_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(0b0011), &mut memory).unwrap();
/// assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b0000); // AX = 0
/// ```
pub fn execute_and_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let val1 = cpu.registers.get(reg)?;
        let val2 = match op2 {
            Operand::Register(reg) => cpu.registers.get(reg)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("Invalid second operand for AND".to_string()),
        };
        cpu.registers.set(reg, val1 & val2)
    } else {
        Err("AND requires register first operand".to_string())
    }
}

pub fn execute_or_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let val1 = cpu.registers.get(reg)?;
        let val2 = match op2 {
            Operand::Register(reg) => cpu.registers.get(reg)?,
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("Invalid second operand for OR".to_string()),
        };
        cpu.registers.set(reg, val1 | val2)
    } else {
        Err("OR requires register first operand".to_string())
    }
}

pub fn execute_xor_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let val1 = cpu.registers.get(reg)?;
        let val2 = match op2 {
            Operand::Register(ref src_reg) => cpu.registers.get(src_reg)?,
            &Operand::Immediate(imm) => (imm).into(),
            _ => return Err("XOR requires register or immediate second operand".to_string()),
        };
        cpu.registers.set(reg, val1 ^ val2)
    } else {
        Err("XOR requires register first operand".to_string())
    }
}

pub fn execute_not_instruction(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let val = cpu.registers.get(reg)?;
        cpu.registers.set(reg, !val)
    } else {
        Err("NOT requires register operand".to_string())
    }
}

pub fn execute_shl_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let val = cpu.registers.get(reg)?;
        let shift_amount = match op2 {
            Operand::Register(ref src_reg) => cpu.registers.get(src_reg)?,
            &Operand::Immediate(imm) => (imm).into(),
            _ => return Err("SHL requires register or immediate second operand".to_string()),
        };
        cpu.registers.set(reg, val.wrapping_shl(shift_amount))
    } else {
        Err("SHL requires register first operand".to_string())
    }
}

pub fn execute_shr_instruction(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let val = cpu.registers.get(reg)?;
        let shift_amount = match op2 {
            Operand::Register(ref src_reg) => cpu.registers.get(src_reg)?,
            &Operand::Immediate(imm) => (imm).into(),
            _ => return Err("SHR requires register or immediate second operand".to_string()),
        };
        cpu.registers.set(reg, val.wrapping_shr(shift_amount))
    } else {
        Err("SHR requires register first operand".to_string())
    }
}

#[cfg(test)]
mod bitwise_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::WorkMemory;
    use crate::utils::assembler::operands::Operand;
    use crate::memory::registers::Reg;

    #[test]
    fn and_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 0b1100).unwrap();
        execute_and_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(0b1010), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1000);

        cpu.registers.set(&Reg::BX, 0b0110).unwrap();
        execute_and_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b0000);
    }

    #[test]
    fn or_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 0b1100).unwrap();
        execute_or_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(0b0011), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1111);

        cpu.registers.set(&Reg::BX, 0b0101).unwrap();
        execute_or_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b1111);
    }

    #[test]
    fn xor_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 0b1100).unwrap();
        execute_xor_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(0b1010), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b0110);

        cpu.registers.set(&Reg::BX, 0b0110).unwrap();
        execute_xor_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b0000);
    }

    #[test]
    fn not_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 0b1100).unwrap();
        execute_not_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), !0b1100);
    }

    #[test]
    fn shl_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 0b00000000000000000000000000000001).unwrap();
        execute_shl_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(2), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b00000000000000000000000000000100);

        cpu.registers.set(&Reg::BX, 3).unwrap();
        execute_shl_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 32);
    }

    #[test]
    fn shr_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 0b00000000000000000000000000000100).unwrap();
        execute_shr_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(2), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b00000000000000000000000000000001);

        cpu.registers.set(&Reg::BX, 1).unwrap();
        execute_shr_instruction(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 0b00000000000000000000000000000000);
    }
}