use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;

pub fn execute_halt(cpu: &mut CPU, _op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    cpu.halted = true;
    Ok(())
}
