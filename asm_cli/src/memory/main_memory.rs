//! # Main Memory Module
//!
//! This module defines the `WorkMemory` struct, which simulates the main memory
//! of the ARC CPU. It includes memory layout constants and methods for reading,
//! writing, and managing memory contents.

/// Default memory size if not specified (64KB).
pub const DEFAULT_MEMORY_SIZE: usize = 0x10000; // 64KB

/// Represents the simulated main memory of the ARC CPU.
#[derive(Debug, Clone)]
pub struct WorkMemory {
    /// The underlying vector of bytes representing the memory.
    pub memory: Vec<u8>,
    /// The total size of the memory in bytes.
    pub size: usize,
    /// The current value of the stack pointer.
    pub stack_pointer: u32,
}

impl WorkMemory {
    /// Creates a new `WorkMemory` instance with a specified size.
    ///
    /// All memory locations are initialized to 0, and the stack pointer
    /// is set to the top of the stack segment.
    ///
    /// # Arguments
    ///
    /// * `size` - The total size of the memory in bytes.
    pub fn new(size: usize) -> Self {
        WorkMemory {
            memory: vec![0; size],
            size,
            stack_pointer: (size - 1) as u32, // Initialize SP to the top of the allocated memory
        }
    }

    /// Reads a single byte (u8) from the specified memory address.
    ///
    /// # Arguments
    ///
    /// * `address` - The memory address to read from.
    ///
    /// # Returns
    ///
    /// * `Result<u8, String>` - The byte value on success, or an error message if the address is out of bounds.
    pub fn read_u8(&self, address: u32) -> Result<u8, String> {
        if address as usize >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        Ok(self.memory[address as usize])
    }

    /// Writes a single byte (u8) to the specified memory address.
    ///
    /// # Arguments
    ///
    /// * `address` - The memory address to write to.
    /// * `value` - The byte value to write.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on success, or an error message if the address is out of bounds.
    pub fn write_u8(&mut self, address: u32, value: u8) -> Result<(), String> {
        if address as usize >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        self.memory[address as usize] = value;
        Ok(())
    }

    /// Reads a 16-bit unsigned integer (u16) from the specified memory address.
    ///
    /// Values are read in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `address` - The starting memory address to read from.
    ///
    /// # Returns
    ///
    /// * `Result<u16, String>` - The u16 value on success, or an error message if the address is out of bounds.
    pub fn read_u16(&self, address: u32) -> Result<u16, String> {
        if address as usize + 1 >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.memory[address as usize..address as usize + 2]);
        Ok(u16::from_le_bytes(bytes))
    }

    /// Writes a 16-bit unsigned integer (u16) to the specified memory address.
    ///
    /// Values are written in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `address` - The starting memory address to write to.
    /// * `value` - The u16 value to write.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on success, or an error message if the address is out of bounds.
    pub fn write_u16(&mut self, address: u32, value: u16) -> Result<(), String> {
        if address as usize + 1 >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        let bytes = value.to_le_bytes();
        self.memory[address as usize..address as usize + 2].copy_from_slice(&bytes);
        Ok(())
    }

    /// Reads a 32-bit unsigned integer (u32) from the specified memory address.
    ///
    /// Values are read in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `address` - The starting memory address to read from.
    ///
    /// # Returns
    ///
    /// * `Result<u32, String>` - The u32 value on success, or an error message if the address is out of bounds.
    pub fn read_u32(&self, address: u32) -> Result<u32, String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[address as usize..address as usize + 4]);
        Ok(u32::from_le_bytes(bytes))
    }

    /// Writes a 32-bit unsigned integer (u32) to the specified memory address.
    ///
    /// Values are written in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `address` - The starting memory address to write to.
    /// * `value` - The u32 value to write.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on success, or an error message if the address is out of bounds.
    pub fn write_u32(&mut self, address: u32, value: u32) -> Result<(), String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        let bytes = value.to_le_bytes();
        self.memory[address as usize..address as usize + 4].copy_from_slice(&bytes);
        Ok(())
    }

    /// Reads a 32-bit floating-point number (f32) from the specified memory address.
    ///
    /// Values are read in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `address` - The starting memory address to read from.
    ///
    /// # Returns
    ///
    /// * `Result<f32, String>` - The f32 value on success, or an error message if the address is out of bounds.
    pub fn read_f32(&self, address: u32) -> Result<f32, String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[address as usize..address as usize + 4]);
        Ok(f32::from_le_bytes(bytes))
    }

    /// Writes a 32-bit floating-point number (f32) to the specified memory address.
    ///
    /// Values are written in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `address` - The starting memory address to write to.
    /// * `value` - The f32 value to write.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on success, or an error message if the address is out of bounds.
    pub fn write_f32(&mut self, address: u32, value: f32) -> Result<(), String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        let bytes = value.to_le_bytes();
        self.memory[address as usize..address as usize + 4].copy_from_slice(&bytes);
        Ok(())
    }

    /// Returns the current value of the stack pointer.
    ///
    /// # Returns
    ///
    /// * `u32` - The current stack pointer value.
    pub fn get_stack_pointer(&self) -> u32 {
        self.stack_pointer
    }

    /// Sets the value of the stack pointer.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for the stack pointer.
    pub fn set_stack_pointer(&mut self, value: u32) {
        self.stack_pointer = value;
    }

    /// Loads a program (a slice of 32-bit instructions) into memory starting at a specified address.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The memory address where the program should begin.
    /// * `program` - A slice of `u32` representing the program instructions.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful loading, or an error message if the program is too large.
    pub fn load_program(&mut self, start_address: u32, program: &[u32]) -> Result<(), String> {
        if start_address as usize + program.len() * 4 > self.size {
            return Err("Program too large for memory".to_string());
        }
        
        for (i, &instruction) in program.iter().enumerate() {
            self.write_u32(start_address + (i * 4) as u32, instruction)?;
        }
        
        Ok(())
    }

    /// Loads a block of data (a slice of bytes) into memory starting at a specified address.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The memory address where the data should begin.
    /// * `data` - A slice of `u8` representing the data bytes.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful loading, or an error message if the data is too large.
    pub fn load_data(&mut self, start_address: u32, data: &[u8]) -> Result<(), String> {
        if start_address as usize + data.len() > self.size {
            return Err("Data too large for memory".to_string());
        }
        
        for (i, &byte) in data.iter().enumerate() {
            self.write_u8(start_address + i as u32, byte)?;
        }
        
        Ok(())
    }

    /// Reads a 32-bit instruction from the specified memory address.
    ///
    /// This is a convenience function that calls `read_u32`.
    ///
    /// # Arguments
    ///
    /// * `address` - The memory address of the instruction.
    ///
    /// # Returns
    ///
    /// * `Result<u32, String>` - The instruction as a `u32` on success, or an error message on failure.
    pub fn read_instruction(&self, address: u32) -> Result<u32, String> {
        self.read_u32(address)
    }

    /// Calculates the number of pages required to display the entire memory.
    ///
    /// # Arguments
    ///
    /// * `page_size` - The size of each page in bytes.
    ///
    /// # Returns
    ///
    /// * `usize` - The total number of pages. Returns 0 if `page_size` is 0.
    pub fn get_num_pages(&self, page_size: usize) -> usize {
        if page_size == 0 {
            return 0;
        }
        self.size.div_ceil(page_size)
    }

    /// Reads a block of memory as 32-bit words for display purposes.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The starting memory address of the block.
    /// * `count` - The number of 32-bit words to read.
    ///
    /// # Returns
    ///
    /// * `Vec<(u32, u32)>` - A vector of tuples, where each tuple contains
    ///   the address and the 32-bit value at that address.
    pub fn read_memory_block(&self, start_address: u32, count: usize) -> Vec<(u32, u32)> {
        let mut result = Vec::new();
        
        for i in 0..count {
            let addr = start_address + (i * 4) as u32;
            if addr as usize + 3 < self.size {
                if let Ok(value) = self.read_u32(addr) {
                    result.push((addr, value));
                }
            }
        }
        
        result
    }
}