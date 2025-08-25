pub mod main_memory; 
pub mod registers; 
// USING CRATE's FUNCTIONALITYS; 
use crate::memory::main_memory::WorkMemory;
use crate::chips::crom::CPU; 
// DECLARING WORKING CONSTANTS - STARTS and ENDS for the segments.
// generate functions
fn generate_work_memory() -> WorkMemory {
    let work_memory:WorkMemory = WorkMemory::new();
    return work_memory;
}

pub fn initiate_working_env(init_segments:&mut (u32,u32,u32,u32,u32)) -> (WorkMemory, CPU) { 
    let mut cpu = CPU::new();
    cpu.initialize_cpu_state(init_segments, (10, 11, 14, 15));
    // Define memory blocks for task - Code(STATICS TEXT), STACK, DATA e EXTRA    
    // For now it will be a hard-coded begin, but that MUST be changed after
    let work_memory: WorkMemory = generate_work_memory();
    // WARNING!! THIS FEATURE IS NOT ACTUALLY IMPLEMENTED, THIS IS A OBJECTIVE THOUGH!
    cpu.flag.over_flow_test();
    // PRE-TEST -> SEING THE CONTENTS of MEMORY: 
    return (work_memory, cpu); 
}