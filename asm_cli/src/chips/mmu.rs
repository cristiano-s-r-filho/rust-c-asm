use crate::memory::initiate_working_env;
use crate::memory::main_memory::{WorkMemory, MEMORY_MAX_SIZE};
use crate::memory::registers::{MainRegisters, OffsetRegisters, SegmentRegisters, EFLAG};
use crate::chips::crom::*; 

pub struct MMU {
    controlers: CRegisters, 
    observers: Vec<CRObserver>,
    rom: EEPROM, 
    data_bus: u32, 
    adress_bus: u32  
}
impl MMU {
    pub fn new() -> Self {
        let spawn_observers = CRegisters::cregisters_quick_start();
        MMU { 
            controlers: spawn_observers.0,
            observers: vec![spawn_observers.1.0,spawn_observers.1.1, spawn_observers.1.2],
            rom: EEPROM::new(),
            data_bus: 0x0,
            adress_bus: 0x0 
        }
    }
    pub fn generate_process_id(segment_select: &SegmentRegisters) -> u32 {
        let mut process_id:u32 = 0; 
        let mut vector = [1;32];
        for i in vector  {
            match (i as f32)%2.0 {
                0.0 => process_id += segment_select.cs as u32, 
                _ => process_id *= 2
            }
            let inter = [1.0,2.0,4.0,6.0,8.0,10.0,12.0];
            let index = 0; 
            if (inter[index + i]%2.0) == 0.0 {
                vector[index + i] = 1 
            } else {
                vector[index + i] = 0;
            }
        }
        return process_id;
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
    pub fn start_process_manager(&mut self, segment_or_paging: bool, initial_data:Vec<u32>) -> (WorkMemory, MainRegisters, SegmentRegisters, OffsetRegisters, EFLAG, CRegisters, Vec<CRObserver>) {
        let mut mmu = MMU::new(); 
        // First, we need to be able to determine: SEGMENTATION OR PAGINATION?
        // if segment_or_paging == true -> SEGMENTATION ON.  
        if segment_or_paging == true {
            let work_env = initiate_working_env(&initial_data); 
            // define process management for segmentation: 
            let controlers = mmu.controlers;
            let mut observers = mmu.observers;
            let work_memory = work_env.1; 
            let main_registers = work_env.2;
            let segment_registers = work_env.3;
            let offsets = work_env.4;
            let flag = work_env.5; 
            // Initiate events: z
            observers[0].initiate_events(10);
            observers[1].initiate_events(10);
            observers[2].initiate_events(10);
            return (work_memory,main_registers, segment_registers, offsets,flag, controlers, observers);
        } else {
            // Declare working variables.
            let mut observers = mmu.observers;
            let controlers = mmu.controlers; 
            let mut page_dir = PageDir::new(); 
            let work_env = initiate_working_env(&initial_data); 
            let mut work_memory = work_env.1; 
            let main_registers = work_env.2;
            let segment_registers = work_env.3;
            let offsets = work_env.4;
            let flag = work_env.5; 
            // First: generate process_id for the first process based on initial segment
            let process_id = MMU::generate_process_id(&segment_registers); 
            // println!("PROCESS ID: {}", process_id);
            // Second: Introduce on  Page Table Directory
            page_dir.content[0].add_page(process_id, segment_registers.cs as u32, self.rom.cells[0], true, &initial_data); 
            // Third: Start process on paging mode 
            let mut adress = 0; 
            for _i in [0..initial_data.len()] {
                let d = initial_data[adress];
                work_memory.write(adress,&d);
                adress += 1; 
            }
            // Fourth: Initiate events on observers here:
            observers[0].initiate_events(10);
            observers[1].initiate_events(10);
            observers[2].initiate_events(10);
            // Fifth: Start functionality for ROM reading and writing
            let mut tab_adrr:usize = 0; 
            if work_memory.cells.len() == MEMORY_MAX_SIZE as usize {
                if mmu.rom.acess == true {
                     for _i in [0..initial_data.len()] {
                        mmu.rom.write_to_rom(tab_adrr as u32, initial_data[tab_adrr]);
                        tab_adrr += 1; 
                     } 
                } else {
                    mmu.rom.change_acess();
                    for _j in [0..initial_data.len()] {
                        mmu.rom.write_to_rom(tab_adrr as u32, initial_data[tab_adrr]);
                        tab_adrr += 1; 
                    }
                    mmu.rom.change_acess();
                }
            }
            
            return (work_memory,main_registers, segment_registers, offsets,flag, controlers, observers, )
        } 

    }
}