use crate::memory::initiate_working_env;
use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::{MainRegisters, OffsetRegisters, SegmentRegisters, EFLAG};
use crate::memory::{CODE_HEAD,CODE_TAIL,STACK_HEAD,STACK_TAIL,DATA_HEAD,DATA_TAIL};
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

    pub fn fisical_adress(&mut self, segment_register: &str ,offset: u32, flag: EFLAG ) -> u32 {
        let mut base_adrr = 0x10;
        let mut flag = flag; 
        if segment_register == "cs" {
            base_adrr = CODE_HEAD;
        } else if segment_register == "ss" {
            base_adrr = STACK_HEAD;
        } else if segment_register == "ds" {
            base_adrr = DATA_HEAD;
        } else {
            return 0; 
        }
        
        let fisc_adrr = base_adrr + (offset as usize);
        
        if segment_register == "cs" {
            if fisc_adrr > CODE_TAIL {
                flag.ovfw = true;
            }
        } else if segment_register == "ss" {
            if fisc_adrr > STACK_TAIL {
                flag.ovfw = true;
            }
        } else if segment_register == "ds" {
            if fisc_adrr > DATA_TAIL {
                flag.ovfw = true;
            }
        } else {
            return 0; 
        }

        return fisc_adrr as u32;
    }
}