//! # Comparison and Jump Instructions Module
//!
//! This module provides the implementation for various comparison and jump
//! instructions for the ARC CPU. These instructions allow for conditional
//! and unconditional control flow changes based on register values and CPU flags.

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::assembler::operands::Operand;
use crate::memory::registers::Reg;

/// Executes the `CMPW` instruction, performing a floating-point comparison.
///
/// This instruction compares the values of `op1` and `op2` by effectively
/// subtracting `op2` from `op1` and updating the CPU's flags (zero, sign)
/// based on the result, without storing the result itself.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which can be a register, immediate value, or memory address.
/// * `op2` - The second operand, which can be a register, immediate value, or memory address.
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
/// Executes the `CMPW` instruction, performing a floating-point comparison.
///
/// This instruction compares the values of `op1` and `op2` by effectively
/// subtracting `op2` from `op1` and updating the CPU's flags (zero, sign)
/// based on the result, without storing the result itself.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which can be a register, immediate value, or memory address.
/// * `op2` - The second operand, which can be a register, immediate value, or memory address.
/// * `memory` - A mutable reference to the `WorkMemory`.
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
/// use arc_emulator::instructions::compare;
///
/// let mut cpu = CPU::new();
/// let mut memory = WorkMemory::new(1024);
///
/// // Compare AX (10.0) and BX (5.0) -> AX > BX
/// cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
/// cpu.registers.set(&Reg::BX, 5.0f32.to_bits()).unwrap();
/// compare::execute_cmpw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
/// assert!(!cpu.registers.get_flag("zero").unwrap()); // Not equal
/// assert!(!cpu.registers.get_flag("sign").unwrap()); // Positive difference
///
/// // Compare AX (5.0) and BX (10.0) -> AX < BX
/// cpu.registers.set(&Reg::AX, 5.0f32.to_bits()).unwrap();
/// cpu.registers.set(&Reg::BX, 10.0f32.to_bits()).unwrap();
/// compare::execute_cmpw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
/// assert!(!cpu.registers.get_flag("zero").unwrap()); // Not equal
/// assert!(cpu.registers.get_flag("sign").unwrap());  // Negative difference
///
/// // Compare AX (10.0) and 10.0 (immediate) -> AX == 10.0
/// cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
/// compare::execute_cmpw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(10), &mut memory).unwrap();
/// assert!(cpu.registers.get_flag("zero").unwrap()); // Equal
/// ```
pub fn execute_cmpw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    let value1_bits = match op1 {
        Operand::Register(reg) => cpu.registers.get(reg)?,
        Operand::Immediate(imm) => (*imm as f32).to_bits(),
        Operand::Address(addr) => memory.read_u32(*addr)?,
        _ => return Err("CMPW requires register, immediate, or address first operand".to_string()),
    };
    
    let value2_bits = match op2 {
        Operand::Register(reg) => cpu.registers.get(reg)?,
        Operand::Immediate(imm) => (*imm as f32).to_bits(),
        Operand::Address(addr) => memory.read_u32(*addr)?,
        _ => return Err("CMPW requires register, immediate, or address second operand".to_string()),
    };
    
    let value1_float = f32::from_bits(value1_bits);
    let value2_float = f32::from_bits(value2_bits);

    // Compare by subtracting and updating flags without storing result
    let result_float = value1_float - value2_float;
    cpu.registers.update_flags_f32(result_float);
    Ok(())
}

/// Executes the `JMP` instruction, performing an unconditional jump.
///
/// The program counter (`PC`) is set to the address specified by `op1`.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand, which can be a label, immediate value, address, or register.
/// * `_op2` - The second operand (unused in this instruction).
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_jmp(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    let address = match op1 {
        Operand::Label(label) => {
            label.parse::<u32>().map_err(|_| format!("Invalid address: {}", label))?
        },
        Operand::Immediate(imm) => *imm as u32,
        Operand::Address(addr) => *addr,
        Operand::Register(reg) => cpu.registers.get(reg)?,
        _ => return Err("JMP requires a label, immediate, address, or register operand".to_string()),
    };
    
    cpu.registers.set(&Reg::PC, address)
}

/// Executes the `CALL` instruction, performing a subroutine call.
///
/// The current program counter (return address) is pushed onto the stack,
/// and then the program counter is set to the address specified by `op1`.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand, which can be a label, immediate value, address, or register.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory` for stack operations.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_call(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    let address = match op1 {
        Operand::Label(label) => {
            label.parse::<u32>().map_err(|_| format!("Invalid address: {}", label))?
        },
        Operand::Immediate(imm) => *imm as u32,
        Operand::Address(addr) => *addr,
        Operand::Register(reg) => cpu.registers.get(reg)?,
        _ => return Err("CALL requires a label, immediate, address, or register operand".to_string()),
    };
    
    // Push return address (current PC + 4) onto the stack
    let return_addr = cpu.registers.get(&Reg::PC)?.wrapping_add(4);
    let sp = cpu.registers.get(&Reg::SP)?.wrapping_sub(4);
    memory.write_u32(sp, return_addr)?;
    
    // Update stack pointer
    cpu.registers.set(&Reg::SP, sp)?;
    
    // Jump to subroutine
    cpu.registers.set(&Reg::PC, address)
}

/// Executes the `RET` instruction, returning from a subroutine.
///
/// The return address is popped from the stack, and the program counter (`PC`)
/// is set to this address.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `_op1` - The first operand (unused in this instruction).
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory` for stack operations.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_ret(cpu: &mut CPU, _op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Pop return address from the stack
    let sp = cpu.registers.get(&Reg::SP)?;
    let return_addr = memory.read_u32(sp)?;
    
    // Update stack pointer
    cpu.registers.set(&Reg::SP, sp.wrapping_add(4))?;
    
    // Jump to return address
    cpu.registers.set(&Reg::PC, return_addr)
}

// Conditional jump implementations
/// Executes the `JE` (Jump if Equal) instruction.
///
/// If the CPU's "zero" flag is set (indicating a previous comparison resulted in equality),
/// an unconditional jump to the address specified by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_je(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    if cpu.registers.get_flag("zero")? {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

/// Executes the `JNE` (Jump if Not Equal) instruction.
///
/// If the CPU's "zero" flag is not set (indicating a previous comparison resulted in inequality),
/// an unconditional jump to the address specified by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_jne(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    if !cpu.registers.get_flag("zero")? {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

/// Executes the `JGT` (Jump if Greater Than) instruction.
///
/// If the CPU's "zero" flag is not set AND the "sign" flag is not set
/// (indicating a previous comparison resulted in a positive difference),
/// an unconditional jump to the address specified by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_jgt(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if greater than (for floats: not zero and not sign)
    let zero = cpu.registers.get_flag("zero")?;
    let sign = cpu.registers.get_flag("sign")?;
    if !zero && !sign {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

/// Executes the `JGE` (Jump if Greater Than or Equal) instruction.
///
/// If the CPU's "sign" flag is not set (indicating a previous comparison resulted
/// in a non-negative difference), an unconditional jump to the address specified
/// by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_jge(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if greater than or equal (for floats: not sign)
    let sign = cpu.registers.get_flag("sign")?;
    if !sign {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

/// Executes the `JLT` (Jump if Less Than) instruction.
///
/// If the CPU's "zero" flag is not set AND the "sign" flag is set
/// (indicating a previous comparison resulted in a negative difference),
/// an unconditional jump to the address specified by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_jlt(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if less than (for floats: not zero and sign)
    let zero = cpu.registers.get_flag("zero")?;
    let sign = cpu.registers.get_flag("sign")?;
    if !zero && sign {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

/// Executes the `JLE` (Jump if Less Than or Equal) instruction.
///
/// If the CPU's "zero" flag is set OR the "sign" flag is set
/// (indicating a previous comparison resulted in a non-positive difference),
/// an unconditional jump to the address specified by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_jle(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if less than or equal (for floats: zero or sign)
    let zero = cpu.registers.get_flag("zero")?;
    let sign = cpu.registers.get_flag("sign")?;
    if zero || sign {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

/// Executes the `JS` (Jump if Sign) instruction.
///
/// If the CPU's "sign" flag is set (indicating a previous operation resulted in a negative value),
/// an unconditional jump to the address specified by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_js(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if sign (negative)
    if cpu.registers.get_flag("sign")? {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

/// Executes the `JCO` (Jump if Carry or Overflow) instruction.
///
/// If either the CPU's "carry" flag or "overflow" flag is set,
/// an unconditional jump to the address specified by `op1` is performed.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand for the jump.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_jco(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if carry or overflow (no longer standard for floats, but kept for compatibility if needed)
    let carry = cpu.registers.get_flag("carry")?;
    let overflow = cpu.registers.get_flag("overflow")?;
    
    if carry || overflow {
        execute_jmp(cpu, op1, &Operand::None, memory)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod compare_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::WorkMemory;
    use crate::utils::assembler::operands::Operand;
    use crate::memory::registers::Reg;

    #[test]
    fn cmpw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // CMPW AX, BX (AX > BX)
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 5.0f32.to_bits()).unwrap();
        execute_cmpw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert!(!cpu.registers.get_flag("zero").unwrap());
        assert!(!cpu.registers.get_flag("sign").unwrap());

        // CMPW AX, BX (AX < BX)
        cpu.registers.set(&Reg::AX, 5.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 10.0f32.to_bits()).unwrap();
        execute_cmpw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert!(!cpu.registers.get_flag("zero").unwrap());
        assert!(cpu.registers.get_flag("sign").unwrap());

        // CMPW AX, BX (AX == BX)
        cpu.registers.set(&Reg::AX, 10.0f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 10.0f32.to_bits()).unwrap();
        execute_cmpw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert!(cpu.registers.get_flag("zero").unwrap());
    }

    #[test]
    fn jmp_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        execute_jmp(&mut cpu, &Operand::Immediate(123), &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::PC).unwrap(), 123);
    }

    #[test]
    fn call_ret_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);
        cpu.registers.set(&Reg::SP, 1020).unwrap();
        cpu.registers.set(&Reg::PC, 100).unwrap();

        // CALL 200
        execute_call(&mut cpu, &Operand::Immediate(200), &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::PC).unwrap(), 200);
        assert_eq!(cpu.registers.get(&Reg::SP).unwrap(), 1016);
        assert_eq!(memory.read_u32(1016).unwrap(), 104);

        // RET
        execute_ret(&mut cpu, &Operand::None, &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::PC).unwrap(), 104);
        assert_eq!(cpu.registers.get(&Reg::SP).unwrap(), 1020);
    }

    #[test]
    fn conditional_jump_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // JE
        cpu.registers.set_flag("zero", true);
        execute_je(&mut cpu, &Operand::Immediate(300), &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::PC).unwrap(), 300);

        // JNE
        cpu.registers.set(&Reg::PC, 0).unwrap();
        cpu.registers.set_flag("zero", false);
        execute_jne(&mut cpu, &Operand::Immediate(400), &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::PC).unwrap(), 400);
    }
}