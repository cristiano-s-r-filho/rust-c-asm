//! # Control Instructions Module
//!
//! This module provides the implementation for control-related instructions
//! for the ARC CPU, primarily focusing on manipulating CPU flags.

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::assembler::operands::Operand;

/// Converts a numeric flag ID to its corresponding string name.
///
/// This helper function is used to map the flag ID embedded in instructions
/// to the flag names used by the `Registers` struct.
///
/// # Arguments
///
/// * `id` - A `u8` representing the flag's numeric identifier.
///
/// # Returns
///
/// * `Result<&'static str, String>` - The string name of the flag on success,
///   or an error message if the ID is unknown.
fn flag_id_to_name(id: u8) -> Result<&'static str, String> {
    match id {
        0 => Ok("carry"),
        1 => Ok("zero"),
        2 => Ok("sign"),
        3 => Ok("interrupt"),
        4 => Ok("string"),
        5 => Ok("overflow"),
        6 => Ok("macro"),
        7 => Ok("stack_dir"),
        _ => Err(format!("Unknown flag id: {}", id)),
    }
}

/// Executes the `SETF` instruction, setting a specified CPU flag to true.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The operand, which must be an `Operand::Flag` containing the flag ID.
/// * `_op2` - The second operand (unused in this instruction).
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
/// Executes the `SETF` instruction, setting a specified CPU flag to true.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The operand, which must be an `Operand::Flag` containing the flag ID.
/// * `_op2` - The second operand (unused in this instruction).
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
/// use arc_emulator::instructions::control;
///
/// let mut cpu = CPU::new();
/// let mut memory = WorkMemory::new(1024);
///
/// // Initially, the zero flag should be false
/// assert!(!cpu.registers.get_flag("zero").unwrap());
///
/// // Set the zero flag (flag ID 1)
/// control::execute_setf(&mut cpu, &Operand::Flag(1), &Operand::None, &mut memory).unwrap();
/// assert!(cpu.registers.get_flag("zero").unwrap());
///
/// // Set the carry flag (flag ID 0)
/// assert!(!cpu.registers.get_flag("carry").unwrap());
/// control::execute_setf(&mut cpu, &Operand::Flag(0), &Operand::None, &mut memory).unwrap();
/// assert!(cpu.registers.get_flag("carry").unwrap());
/// ```
pub fn execute_setf(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Flag(flag_id) = op1 {
        let flag_name = flag_id_to_name(*flag_id)?;
        cpu.registers.set_flag(flag_name, true);
        Ok(())
    } else {
        Err("SETF requires a flag name as operand".to_string())
    }
}

/// Executes the `CLRF` instruction, clearing (setting to false) a specified CPU flag.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The operand, which must be an `Operand::Flag` containing the flag ID.
/// * `_op2` - The second operand (unused in this instruction).
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_clrf(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    if let Operand::Flag(flag_id) = op1 {
        let flag_name = flag_id_to_name(*flag_id)?;
        cpu.registers.set_flag(flag_name, false);
        Ok(())
    } else {
        Err("CLRF requires a flag name as operand".to_string())
    }
}
