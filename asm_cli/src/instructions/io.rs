//! # I/O Instructions
//! 
//! This module provides the implementation for input/output (I/O) instructions
//! for the ARC CPU. These instructions allow the CPU to interact with external
//! devices or memory-mapped I/O regions.

use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;

// Fixed I/O segment addresses
const IO_START: u32 = 0xE000;
const IO_SIZE: u32 = 0x1000;
use crate::utils::assembler::operands::Operand;

/// Executes the `IN` instruction, which reads data from an I/O port into memory.
///
/// The `IN` instruction reads a sequence of bytes from the I/O region
/// (starting at `IO_START`) until a null byte (0x00) is encountered,
/// and then loads this data into the memory address specified by `op1`.
///
/// # Arguments
///
/// * `_cpu` - A mutable reference to the CPU state (unused in this instruction).
/// * `op1` - The destination operand, which must be an `Operand::Address`.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or `Err(String)` if
///   `op1` is not an address operand or memory access fails.
pub fn execute_in(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::AddressRegister(reg) => cpu.registers.get(reg)?,
        _ => return Err("IN requires an address or address register operand".to_string()),
    };

    let mut buffer = Vec::new();
    for i in 0..IO_SIZE {
        let byte = memory.read_u8(IO_START + i)?;
        if byte == 0 {
            break;
        }
        buffer.push(byte);
    }
    memory.load_data(addr, &buffer)
}

/// Executes the `INSI` instruction, reading an immediate value from the I/O input buffer.
///
/// The first immediate value from the `io_device.input_buffer` is read and
/// stored into the memory address specified by `op1`.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The destination operand, which must be an `Operand::Address`.
/// * `op2` - The second operand (optional, currently unused).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_insi(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::AddressRegister(reg) => cpu.registers.get(reg)?,
        _ => return Err("INSI requires an address or address register operand".to_string()),
    };

    // Placeholder: In a real scenario, this would parse an immediate from io_device.input_buffer
    // For now, let's simulate reading a fixed value or from a specific part of the buffer.
    // This needs to be refined when io_device.input_buffer is structured for immediates.
    let immediate_value: u32 = 0xDEADBEEF; // Placeholder value
    memory.write_u32(addr, immediate_value)?;
    Ok(())
}

/// Executes the `OUTI` instruction, writing an immediate value to the I/O output buffer.
///
/// An immediate value from `op1` is written to the `io_device.output_buffer`.
///
/// # Arguments
///
/// * `cpu` - A mutable reference to the `CPU` state.
/// * `op1` - The source operand, which must be an `Operand::Immediate`.
/// * `op2` - The second operand (optional, currently unused).
/// * `memory` - A mutable reference to the `WorkMemory` (unused in this instruction).
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or an error message on failure.
pub fn execute_outi(_cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // For now, assume OUTI appends a single immediate to output_buffer
    // and op2 is ignored.
    if let Operand::Immediate(imm) = op1 {
        // Placeholder: In a real scenario, this would append to io_device.output_buffer
        // This needs to be refined when io_device.output_buffer is structured for immediates.
        println!("OUTI: {}", *imm); // For debugging, will integrate with io_device later
        Ok(())
    } else {
        Err("OUTI requires an immediate operand".to_string())
    }
}

/// Executes the `INSW` instruction, reading a word (32-bit) from an I/O port into memory.
///
/// The `INSW` instruction reads a 32-bit word from the I/O region
/// (starting at `IO_START`) and stores it into the memory address specified by `op1`.
///
/// # Arguments
///
/// * `_cpu` - A mutable reference to the CPU state (unused in this instruction).
/// * `op1` - The destination operand, which must be an `Operand::Address`.
/// * `op2` - The second operand (optional, currently unused).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or `Err(String)` if
///   `op1` is not an address operand or memory access fails.
pub fn execute_insw(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::AddressRegister(reg) => cpu.registers.get(reg)?,
        _ => return Err("INSW requires an address or address register operand".to_string()),
    };

    // Placeholder: In a real scenario, this would read a word from an I/O device.
    // For now, let's simulate reading a fixed value or from a specific part of the buffer.
    // This needs to be refined when io_device.input_buffer is structured for immediates.
    let word_value: u32 = 0xCAFEBABE; // Placeholder value
    memory.write_u32(addr, word_value)?;
    Ok(())
}

/// Executes the `OUTW` instruction, writing a word (32-bit) from memory to an I/O port.
///
/// The `OUTW` instruction reads a 32-bit word from the memory address
/// specified by `op1` and writes it to an I/O port.
///
/// # Arguments
///
/// * `_cpu` - A mutable reference to the CPU state (unused in this instruction).
/// * `op1` - The source operand, which must be an `Operand::Address`.
/// * `op2` - The second operand (optional, currently unused).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or `Err(String)` if
///   `op1` is not an address operand or memory access fails.
pub fn execute_outw(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::AddressRegister(reg) => cpu.registers.get(reg)?,
        _ => return Err("OUTW requires an address or address register operand".to_string()),
    };

    let word_value = memory.read_u32(addr)?;
    // Placeholder: In a real scenario, this would write to an I/O device.
    println!("OUTW: 0x{:X}", word_value); // For debugging, will integrate with io_device later
    Ok(())
}

/// Executes the `OUT` instruction, which writes data from memory to an I/O port.
///
/// The `OUT` instruction reads a sequence of bytes from the memory address
/// specified by `op1` until a null byte (0x00) is encountered. This data
/// would typically be sent to an output device.
///
/// # Arguments
///
/// * `_cpu` - A mutable reference to the CPU state (unused in this instruction).
/// * `op1` - The source operand, which can be an `Operand::Address` or `Operand::Register`.
/// * `_op2` - The second operand (unused in this instruction).
/// * `memory` - A mutable reference to the `WorkMemory`.
///
/// # Returns
///
/// * `Result<(), String>` - `Ok(())` on successful execution, or `Err(String)` if
///   `op1` is not an address or register operand, or memory access fails.
pub fn execute_out(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::AddressRegister(reg) => cpu.registers.get(reg)?,
        _ => return Err("OUT requires an address or address register operand".to_string()),
    };

    let mut current_addr = addr;
    let mut output_bytes = Vec::new();
    loop {
        let byte = memory.read_u8(current_addr)?;
        if byte == 0x00 {
            break;
        }
        output_bytes.push(byte);
        current_addr += 1;
    }
    // Placeholder: In a real scenario, this would write to an I/O device.
    println!("OUT: {}", String::from_utf8_lossy(&output_bytes)); // For debugging
    Ok(())
}

#[cfg(test)]
mod io_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::{WorkMemory, IO_START, MEMORY_MAX_SIZE};
    use crate::utils::assembler::operands::Operand;

    #[test]
    fn in_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(MEMORY_MAX_SIZE);
        let dest_addr = 0x100;

        // Prepare some data in the I/O region
        memory.write_u8(IO_START, 0x41).unwrap(); // 'A'
        memory.write_u8(IO_START + 1, 0x42).unwrap(); // 'B'
        memory.write_u8(IO_START + 2, 0x00).unwrap(); // Null terminator

        execute_in(&mut cpu, &Operand::Address(dest_addr), &Operand::None, &mut memory).unwrap();

        assert_eq!(memory.read_u8(dest_addr).unwrap(), 0x41);
        assert_eq!(memory.read_u8(dest_addr + 1).unwrap(), 0x42);
        assert_eq!(memory.read_u8(dest_addr + 2).unwrap(), 0x00);
    }

    #[test]
    fn out_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(MEMORY_MAX_SIZE);
        let src_addr = 0x100;

        // Prepare some data in memory to be "outputted"
        memory.write_u8(src_addr, 0x43).unwrap(); // 'C'
        memory.write_u8(src_addr + 1, 0x44).unwrap(); // 'D'
        memory.write_u8(src_addr + 2, 0x00).unwrap(); // Null terminator

        // This test primarily checks for no errors during execution
        // Actual output would be to a device, which is mocked by println!
        execute_out(&mut cpu, &Operand::Address(src_addr), &Operand::None, &mut memory).unwrap();
        // No direct assertion on output, as it's a side effect (println!)
        // We just ensure it runs without error.
    }

    #[test]
    fn insi_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(MEMORY_MAX_SIZE);
        let dest_addr = 0x200;

        execute_insi(&mut cpu, &Operand::Address(dest_addr), &Operand::None, &mut memory).unwrap();
        assert_eq!(memory.read_u32(dest_addr).unwrap(), 0xDEADBEEF); // Placeholder value
    }

    #[test]
    fn outi_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(MEMORY_MAX_SIZE);
        let immediate_val = 12345;

        // This test primarily checks for no errors during execution
        // Actual output would be to a device, which is mocked by println!
        execute_outi(&mut cpu, &Operand::Immediate(immediate_val), &Operand::None, &mut memory).unwrap();
        // No direct assertion on output, as it's a side effect (println!)
        // We just ensure it runs without error.
    }

    #[test]
    fn insw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(MEMORY_MAX_SIZE);
        let dest_addr = 0x300;

        execute_insw(&mut cpu, &Operand::Address(dest_addr), &Operand::None, &mut memory).unwrap();
        assert_eq!(memory.read_u32(dest_addr).unwrap(), 0xCAFEBABE); // Placeholder value
    }

    #[test]
    fn outw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(MEMORY_MAX_SIZE);
        let src_addr = 0x300;
        let word_val = 0x12345678;

        memory.write_u32(src_addr, word_val).unwrap();

        // This test primarily checks for no errors during execution
        // Actual output would be to a device, which is mocked by println!
        execute_outw(&mut cpu, &Operand::Address(src_addr), &Operand::None, &mut memory).unwrap();
        // No direct assertion on output, as it's a side effect (println!)
        // We just ensure it runs without error.
    }
}