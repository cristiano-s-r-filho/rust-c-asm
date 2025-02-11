// PROTECTED Mode MEMORY implementation
pub const MEMORY_MAX_SIZE: u32 = (u32::MAX)/64; 
pub struct WorkMemory {
    pub cells: Vec<u32>
}
impl WorkMemory{
    pub fn new() -> Self {
       WorkMemory {
        cells: vec![0;MEMORY_MAX_SIZE as usize]
       }
    }
    // Writing with PHYSICAL Adresses
    pub fn write(&mut self, adress:usize, data: &u32) {
        self.cells[adress] = *data; 
    }
    // Reading from PHYSICAL Adresses - Work in progress 
    pub fn read(&mut self, adress:usize) -> u32 {
        let read_value:u32 = self.cells[adress];
        return read_value;  
    }
}

    