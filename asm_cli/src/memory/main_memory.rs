// PROTECTED Mode MEMORY implementation
const MEMORY_MAX_SIZE: usize = u32::MAX as usize + 1; 
pub struct MainMemory {
    pub cells: [u32;MEMORY_MAX_SIZE] 
}
impl MainMemory {
    pub fn new() -> Self {
       MainMemory {
        cells:[0;MEMORY_MAX_SIZE]
       }
    }
    // Writing with PHYSICAL Adresses
    pub fn write(&mut self, adress:usize, data:u32) {
        self.cells[adress] = data; 
    }
    // Reading from PHYSICAL Adresses - Work in progress 
    pub fn read(&mut self, adress:usize) -> u32 {
        let read_value:u32 = self.cells[adress];
        return read_value;  
    }
}

    