// PROTECTED Mode MEMORY implementation
pub const MEMORY_MAX_SIZE: usize = ((u32::MAX)/1024) as usize; 
// memory/main_memory.rs
#[derive(Debug)]
pub struct WorkMemory {
    data: Vec<u8>,
    pub size: usize,
}

impl WorkMemory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            size,
        }
    }
    pub fn new_max() -> Self {
        Self {
            data: vec![0; MEMORY_MAX_SIZE],
            size: MEMORY_MAX_SIZE,
        }
    }
    
    pub fn read(&self, address: u32) -> Result<u32, String> {
        if address as usize + 3 >= self.size {
            return Err("Memory read out of bounds".to_string());
        }
        
        let addr = address as usize;
        let value = (self.data[addr] as u32) |
                   ((self.data[addr + 1] as u32) << 8) |
                   ((self.data[addr + 2] as u32) << 16) |
                   ((self.data[addr + 3] as u32) << 24);
        
        Ok(value)
    }
    
    pub fn write(&mut self, address: u32, value: u32) -> Result<(), String> {
        if address as usize + 3 >= self.size {
            return Err("Memory write out of bounds".to_string());
        }
        
        let addr = address as usize;
        self.data[addr] = value as u8;
        self.data[addr + 1] = (value >> 8) as u8;
        self.data[addr + 2] = (value >> 16) as u8;
        self.data[addr + 3] = (value >> 24) as u8;
        
        Ok(())
    }
    
    pub fn read_byte(&self, address: u32) -> Result<u8, String> {
        if address as usize >= self.size {
            return Err("Memory read out of bounds".to_string());
        }
        
        Ok(self.data[address as usize])
    }
    
    pub fn write_byte(&mut self, address: u32, value: u8) -> Result<(), String> {
        if address as usize >= self.size {
            return Err("Memory write out of bounds".to_string());
        }
        
        self.data[address as usize] = value;
        Ok(())
    }
    pub fn get_stack_pointer(&self) -> u32 {
        // In a real implementation, this would get the current stack pointer
        // For now, we'll return a fixed value for demonstration
        0xFFFF
    }
}
    