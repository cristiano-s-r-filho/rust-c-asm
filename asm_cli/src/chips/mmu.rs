use crate::memory::initiate_working_env;
use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::{MainRegisters, OffsetRegisters, SegmentRegisters, EFLAG};
pub struct MMU {
    data_bus: u32, 
    adress_bus: u32  
}
impl MMU {
    pub fn new() -> Self {
        MMU { 
            data_bus: 0x0,
            adress_bus: 0x0 
        }
    }
    pub fn foward_to_data_bus(&mut self, dt: u32){
        self.data_bus = dt;
    }
    pub fn get_from_data_bus(&mut self) -> u32 {
        return self.data_bus;
    }
    pub fn forward_to_adress_bus(&mut self, adrr: usize) {
        self.adress_bus = adrr as u32; 
    }
    pub fn get_from_adress_bus(&mut self) -> u32 {
        return self.adress_bus; 
    }
    pub fn start_process_manager(&mut self, initial_data:Vec<u32>) -> (WorkMemory, MainRegisters, SegmentRegisters, OffsetRegisters, EFLAG, MMU) {
        let mmu = MMU::new(); 
        // First, we need to be able to determine: SEGMENTATION OR PAGINATION?
        // if segment_or_paging == true -> SEGMENTATION ON.  
            let work_env = initiate_working_env(&initial_data); 
            // define process management for Segmentation: 
            let mut work_memory = work_env.1; 
            let main_registers = work_env.2;
            let segment_registers = work_env.3;
            let offsets = work_env.4;
            let flag = work_env.5; 
           // Write Data ot Memory... 
            let mut adress = 0; 
            for _i in [0..initial_data.len()] {
                let d = initial_data[adress];
                work_memory.write(adress,&d);
                adress += 1; 
            }
            
            return (work_memory,main_registers, segment_registers, offsets,flag, mmu)
    }
}