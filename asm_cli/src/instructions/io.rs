use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;
use crate::chips::io_device::IoDevice;

pub fn execute_in(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory, io_device: &mut IoDevice) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        if let Some(char_to_read) = io_device.input_buffer.pop() {
            cpu.registers.set(reg, char_to_read as u32)?;
        } else {
            // Set register to 0 if buffer is empty
            cpu.registers.set(reg, 0)?;
        }
        Ok(())
    } else {
        Err("IN requires a register operand".to_string())
    }
}

pub fn execute_out(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory, io_device: &mut IoDevice) -> Result<(), String> {
    if let Operand::Register(reg) = op1 {
        let value = cpu.registers.get(reg)? as u8 as char;
        io_device.output_buffer.push(value);
        Ok(())
    } else {
        Err("OUT requires a register operand".to_string())
    }
}
