// PROTECTED Mode MEMORY implementation
pub const MEMORY_MAX_SIZE: usize = ((u32::MAX)/1024) as usize; 

pub const TEXT_START: u32 = 0x0000;
pub const TEXT_SIZE: u32 = 0x8000; // 32KB
pub const DATA_START: u32 = TEXT_START + TEXT_SIZE; // 0x8000
pub const DATA_SIZE: u32 = 0x4000; // 16KB
pub const STACK_SIZE: u32 = 0x4000; // 16KB
pub const STACK_START: u32 = DATA_START + DATA_SIZE; // 0xC000
pub const STACK_END: u32 = STACK_START + STACK_SIZE - 1; // 0xFFFF

#[derive(Debug)]
pub struct WorkMemory {
    pub memory: Vec<u8>,
    pub size: usize,
    pub stack_pointer: u32,
}

impl WorkMemory {
    pub fn new(size: usize) -> Self {
        WorkMemory {
            memory: vec![0; size],
            size,
            stack_pointer: STACK_END, // Initialize SP to top of stack section
        }
    }

    pub fn read_u8(&self, address: u32) -> Result<u8, String> {
        if address as usize >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        Ok(self.memory[address as usize])
    }

    pub fn write_u8(&mut self, address: u32, value: u8) -> Result<(), String> {
        if address as usize >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        self.memory[address as usize] = value;
        Ok(())
    }

    pub fn read_u16(&self, address: u32) -> Result<u16, String> {
        if address as usize + 1 >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.memory[address as usize..address as usize + 2]);
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn write_u16(&mut self, address: u32, value: u16) -> Result<(), String> {
        if address as usize + 1 >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        let bytes = value.to_le_bytes();
        self.memory[address as usize..address as usize + 2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn read_u32(&self, address: u32) -> Result<u32, String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[address as usize..address as usize + 4]);
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn write_u32(&mut self, address: u32, value: u32) -> Result<(), String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        let bytes = value.to_le_bytes();
        self.memory[address as usize..address as usize + 4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn read_f32(&self, address: u32) -> Result<f32, String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory read error: Address {:#010x} out of bounds", address));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[address as usize..address as usize + 4]);
        Ok(f32::from_le_bytes(bytes))
    }

    pub fn write_f32(&mut self, address: u32, value: f32) -> Result<(), String> {
        if address as usize + 3 >= self.size {
            return Err(format!("Memory write error: Address {:#010x} out of bounds", address));
        }
        let bytes = value.to_le_bytes();
        self.memory[address as usize..address as usize + 4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn get_stack_pointer(&self) -> u32 {
        self.stack_pointer
    }

    pub fn set_stack_pointer(&mut self, value: u32) {
        self.stack_pointer = value;
    }

    // Load a program into memory starting at a specific address
    pub fn load_program(&mut self, start_address: u32, program: &[u32]) -> Result<(), String> {
        if start_address as usize + program.len() * 4 > self.size {
            return Err("Program too large for memory".to_string());
        }
        
        for (i, &instruction) in program.iter().enumerate() {
            self.write_u32(start_address + (i * 4) as u32, instruction)?;
        }
        
        Ok(())
    }

    // Load data into memory starting at a specific address
    pub fn load_data(&mut self, start_address: u32, data: &[u8]) -> Result<(), String> {
        if start_address as usize + data.len() > self.size {
            return Err("Data too large for memory".to_string());
        }
        
        for (i, &byte) in data.iter().enumerate() {
            self.write_u8(start_address + i as u32, byte)?;
        }
        
        Ok(())
    }

    // Read an instruction from memory
    pub fn read_instruction(&self, address: u32) -> Result<u32, String> {
        self.read_u32(address)
    }

    // Read a block of memory for display purposes
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