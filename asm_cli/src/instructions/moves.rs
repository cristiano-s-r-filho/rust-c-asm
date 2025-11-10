//! # Move Instructions Module
//!
//! This module provides the implementation for various data movement instructions
//! for the ARC CPU. These instructions handle transferring data between registers,
//! immediate values, and memory locations.

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::assembler::operands::Operand;
use crate::memory::registers::Reg;

/// Executes the `MOVI` instruction, moving an immediate value to a register.
///
/// The immediate value is stored in the destination register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which must be an `Operand::Register`.
/// * `op2` - The second operand, which must be an `Operand::Immediate`.
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_movi(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // MOVI DST, SRC -> Move immediate value to register
    if let Operand::Register(reg) = op1 {
        let value = match op2 {
            Operand::Immediate(imm) => *imm,
            _ => return Err("MOVI requires immediate second operand".to_string()),
        };
        cpu.registers.set(reg, value)
    } else {
        Err("MOVI requires register first operand".to_string())
    }
}

/// Executes the `MOVW` instruction, moving a word value to a register.
///
/// The value from `op2` (register, immediate, or memory address) is moved
/// to `op1` (destination register). Values are treated as `u32` bit patterns.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which must be an `Operand::Register`.
/// * `op2` - The second operand, which can be an `Operand::Register`, `Operand::Immediate`, or `Operand::Address`.
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_movw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // MOVW DST, SRC -> Move full word value to register
    if let Operand::Register(dest_reg) = op1 {
        let value = match op2 {
            Operand::Register(src_reg) => cpu.registers.get(src_reg)?,
            Operand::Immediate(imm) => *imm,
            Operand::Address(addr) => memory.read_u32(*addr)?,
            _ => return Err("Invalid second operand for MOVW".to_string()),
        };
        cpu.registers.set(dest_reg, value)
    } else {
        Err("MOVW requires register first operand".to_string())
    }
}

/// Executes the `LODI` instruction, loading an immediate value into a register.
///
/// This instruction is functionally similar to `MOVI`, loading an immediate
/// value into the specified register.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which must be an `Operand::Register`.
/// * `op2` - The second operand, which must be an `Operand::Immediate`.
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_lodi(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // LODI DST, SRC -> Load immediate value.
    if let Operand::Register(reg) = op1 {
        let value = match op2 {
            Operand::Immediate(imm) => *imm,
            _ => return Err("LODI requires immediate second operand".to_string()),
        };
        cpu.registers.set(reg, value)
    } else {
        Err("LODI requires register first operand".to_string())
    }
}

/// Executes the `LODW` instruction, loading a word from memory into a register.
///
/// The 32-bit value at the memory address specified by `op2` is loaded
/// into the destination register `op1`.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which must be an `Operand::Register`.
/// * `op2` - The second operand, which can be an `Operand::Address` or `Operand::Register` (containing an address).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_lodw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // LODW DST, SRC -> Load Word to register.
    if let Operand::Register(reg) = op1 {
        let addr = match op2 {
            Operand::Address(addr) => *addr,
            Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
            _ => return Err("LODW requires address or register second operand".to_string()),
        };
        let value = memory.read_u32(addr)?;
        cpu.registers.set(reg, value)
    } else {
        Err("LODW requires register first operand".to_string())
    }
}

/// Executes the `STRI` instruction, storing an immediate value into memory.
///
/// The immediate value from `op2` is stored into the memory address specified by `op1`.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which can be an `Operand::Address` or `Operand::Register` (containing an address).
/// * `op2` - The second operand, which must be an `Operand::Immediate`.
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_stri(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // STRI DST, SRC -> Store Immediate Source into memory
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
        _ => return Err("STRI requires address or register first operand".to_string()),
    };

    let value = match op2 {
        Operand::Immediate(imm) => *imm,
        _ => return Err("STRI requires immediate second operand".to_string()),
    };

    memory.write_u32(addr, value)
}

/// Executes the `STRW` instruction, storing a word from a register or immediate value into memory.
///
/// The value from `op2` (register or immediate) is stored into the memory
/// address specified by `op1`.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which can be an `Operand::Address` or `Operand::Register` (containing an address).
/// * `op2` - The second operand, which can be an `Operand::Register` or `Operand::Immediate`.
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_strw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // STRW DST, SRC -> Store Word Source into memory
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
        _ => return Err("STRW requires address or register first operand".to_string()),
    };

    let value = match op2 {
        Operand::Register(value_reg) => cpu.registers.get(value_reg)?,
        Operand::Immediate(imm) => *imm,
        _ => return Err("STRW requires register or immediate second operand".to_string()),
    };

    memory.write_u32(addr, value)
}

/// Executes the `PUSH` instruction, pushing a value onto the stack.
///
/// The value from `op1` (register or immediate) is pushed onto the stack.
/// The stack pointer (`SP`) is updated accordingly based on the stack direction flag.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which can be an `Operand::Register` or `Operand::Immediate`.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory` for stack operations.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_push(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // PUSH SRC -> Push SRC to STACK
    let value = match op1 {
        Operand::Register(reg) => cpu.registers.get(reg)?,
        Operand::Immediate(imm) => *imm,
        _ => return Err("PUSH requires register or immediate operand".to_string()),
    };

    let sp = cpu.registers.get(&Reg::SP)?;
    let stack_grows_upward = cpu.registers.get_flag("stack_dir")?;

    if stack_grows_upward {
        memory.write_u32(sp, value)?;
        cpu.registers.set(&Reg::SP, sp.wrapping_add(4))
    } else {
        let new_sp = sp.wrapping_sub(4);
        memory.write_u32(new_sp, value)?;
        cpu.registers.set(&Reg::SP, new_sp)
    }
}

/// Executes the `POP` instruction, popping a value from the stack into a register.
///
/// A value is popped from the stack into the destination register `op1`.
/// The stack pointer (`SP`) is updated accordingly based on the stack direction flag.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which must be an `Operand::Register`.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory` for stack operations.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_pop(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // POP DST -> Pop from STACK and load to DST
    if let Operand::Register(reg) = op1 {
        let sp = cpu.registers.get(&Reg::SP)?;
        let stack_grows_upward = cpu.registers.get_flag("stack_dir")?;

        if stack_grows_upward {
            let new_sp = sp.wrapping_sub(4);
            let value = memory.read_u32(new_sp)?;
            cpu.registers.set(reg, value)?;
            cpu.registers.set(&Reg::SP, new_sp)
        } else {
            let value = memory.read_u32(sp)?;
            let new_sp = sp.wrapping_add(4);
            cpu.registers.set(reg, value)?;
            cpu.registers.set(&Reg::SP, new_sp)
        }
    } else {
        Err("POP requires register operand".to_string())
    }
}

/// Executes the `XCGH` instruction, exchanging values between two registers.
///
/// The values in `op1` and `op2` (both of which must be registers) are swapped.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The first operand, which must be an `Operand::Register`.
/// * `op2` - The second operand, which must be an `Operand::Register`.
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_xcgh(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // XCGH OP1, OP2 -> Exchange values from 1 to 2, and 2 to 1; 
    if let (Operand::Register(reg1), Operand::Register(reg2)) = (op1, op2) {
        let val1 = cpu.registers.get(reg1)?;
        let val2 = cpu.registers.get(reg2)?;
        cpu.registers.set(reg1, val2)?;
        cpu.registers.set(reg2, val1)
    } else {
        Err("XCGH requires two register operands".to_string())
    }
}

#[cfg(test)]
mod moves_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::WorkMemory;
use crate::utils::assembler::operands::Operand;
    use crate::memory::registers::Reg;

    #[test]
    fn movi_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        execute_movi(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(123), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 123);
    }

    #[test]
    fn movw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // MOVW AX, BX
        cpu.registers.set(&Reg::BX, 456).unwrap();
        execute_movw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 456);

        // MOVW AX, 789
        execute_movw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(789), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 789);

        // MOVW AX, [100]
        memory.write_u32(100, 101).unwrap();
        execute_movw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Address(100), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 101);
    }

    #[test]
    fn lodi_lodw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // LODI AX, 123
        execute_lodi(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(123), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 123);

        // LODW AX, [200]
        memory.write_u32(200, 456).unwrap();
        execute_lodw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Address(200), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 456);
    }

    #[test]
    fn stri_strw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // STRI [300], 123
        execute_stri(&mut cpu, &Operand::Address(300), &Operand::Immediate(123), &mut memory).unwrap();
        assert_eq!(memory.read_u32(300).unwrap(), 123);

        // STRW [400], AX
        cpu.registers.set(&Reg::AX, 789).unwrap();
        execute_strw(&mut cpu, &Operand::Address(400), &Operand::Register(Reg::AX), &mut memory).unwrap();
        assert_eq!(memory.read_u32(400).unwrap(), 789);
    }

    #[test]
    fn push_pop_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // Set initial stack pointer
        let initial_sp = 1020;
        cpu.registers.set(&Reg::SP, initial_sp).unwrap();

        // PUSH 123
        execute_push(&mut cpu, &Operand::Immediate(123), &Operand::None, &mut memory).unwrap();
        assert_eq!(memory.read_u32(cpu.registers.get(&Reg::SP).unwrap()).unwrap(), 123);

        // POP AX
        execute_pop(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 123);
        assert_eq!(cpu.registers.get(&Reg::SP).unwrap(), initial_sp);
    }

    #[test]
    fn xcgh_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 123).unwrap();
        cpu.registers.set(&Reg::BX, 456).unwrap();

        execute_xcgh(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();

        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 456);
        assert_eq!(cpu.registers.get(&Reg::BX).unwrap(), 123);
    }
}