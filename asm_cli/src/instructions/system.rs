//! # System Instructions Module
//!
//! This module provides the implementation for system-level instructions
//! for the ARC CPU, such as halting the CPU's execution.

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::assembler::operands::Operand;

/// Executes the `HALT` instruction, stopping the CPU's execution.
///
/// This instruction sets the `halted` flag of the CPU to `true`,
/// which typically stops the main execution loop.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `_op1` - The first operand (unused in this instruction).
/// * `_op2` - The second operand (unused in this instruction).
/// * `_memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution.
pub fn execute_halt(cpu: &mut CPU, _op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    cpu.halted = true;
    Ok(())
}

#[cfg(test)]
mod system_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::WorkMemory;
    use crate::utils::assembler::operands::Operand;

    #[test]
    fn halt_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        assert!(!cpu.halted);
        execute_halt(&mut cpu, &Operand::None, &Operand::None, &mut memory).unwrap();
        assert!(cpu.halted);
    }
}
