use crate::memory::initiate_working_env;
use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::{MainRegisters, OffsetRegisters, SegmentRegisters, EFLAG};
#[derive(Clone, Copy)] 
pub struct MMU {
    pub code_summary: (&'static str, bool, u32, u32),
    pub stack_summary:(&'static str, bool, u32, u32),
    pub data_summary:(&'static str, bool, u32, u32), 
    pub extra_sumary: (&'static str, bool, u32, u32), 
    data_bus: u32, 
    adress_bus: u32  
}
impl MMU {
    pub fn new() -> Self {
        MMU { 
            code_summary: ("null", false, 0x0,0x0),
            stack_summary:("null", false, 0x0,0x0),
            data_summary:("null", false, 0x0,0x0),
            extra_sumary:("null", false, 0x0,0x0),
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
    pub fn start_process_manager(&mut self, initial_data:&mut (u32,u32,u32,u32,u32)) -> (WorkMemory, MainRegisters, SegmentRegisters, OffsetRegisters, EFLAG) {
        // First, we need to be able to determine: SEGMENTATION OR PAGINATION?
        // if segment_or_paging == true -> SEGMENTATION ON.  
            let work_env = initiate_working_env(initial_data); 
            self.code_summary = work_env.0.0; 
            self.stack_summary = work_env.0.1; 
            self.data_summary = work_env.0.2; 
            self.extra_sumary = work_env.0.3;
            // define process management for Segmentation: 
            let work_memory = work_env.1; 
            let main_registers = work_env.2;
            let segment_registers = work_env.3;
            let offsets = work_env.4;
            let flag = work_env.5;             
            return (work_memory,main_registers, segment_registers, offsets,flag)
    }

    pub fn fisical_adress(&mut self, segment_register: &str ,offset: u32, flag: EFLAG ) -> u32 {
        let base_adrr:usize;
        let mut flag = flag; 
        if segment_register == "cs" {
            base_adrr = self.code_summary.2 as usize;
        } else if segment_register == "ss" {
            base_adrr = self.stack_summary.2 as usize;
        } else if segment_register == "ds" {
            base_adrr = self.data_summary.2 as usize;
        } else {
            return 0; 
        }
        
        let fisc_adrr = base_adrr + (offset as usize);
        
        if segment_register == "cs" {
            if fisc_adrr > self.code_summary.3 as usize {
                flag.ovfw = true;
            }
        } else if segment_register == "ss" {
            if fisc_adrr > self.stack_summary.3 as usize {
                flag.ovfw = true;
            }
        } else if segment_register == "ds" {
            if fisc_adrr > self.data_summary.3  as usize{
                flag.ovfw = true;
            }
        } else {
            return 0; 
        }

        return fisc_adrr as u32;
    }
}