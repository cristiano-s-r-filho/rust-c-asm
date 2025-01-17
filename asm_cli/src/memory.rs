pub mod main_memory; 
pub mod registers; 
// USING CRATE's FUNCTIONALITYS; 
use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::MainRegisters; 
use crate::memory::registers::OffsetRegisters;
use crate::memory::registers::SegmentRegisters;
use crate::memory::registers::EFLAG;
// DECLARING WORKING CONSTANTS 
// generate functions
fn generate_work_memory() -> WorkMemory {
    let work_memory:WorkMemory = WorkMemory::new();
    return work_memory;
}
fn generate_main_registers() -> MainRegisters {
    let main_registers: MainRegisters = MainRegisters::new(); 
    return main_registers;
}
fn generate_segment_selector() -> SegmentRegisters {
    let segment_selectors: SegmentRegisters = SegmentRegisters::new(); 
    return segment_selectors; 
}
fn generate_offsets() -> OffsetRegisters {
    let offsets: OffsetRegisters = OffsetRegisters::new();
    return offsets; 
}
pub fn initiate_working_env() -> (u16,u16,u16,u16) {
    // Register declaration and initialization; 
    let mut main_registers: MainRegisters = generate_main_registers(); 
    let mut segment_selectors: SegmentRegisters = generate_segment_selector(); 
    let mut offsets: OffsetRegisters = generate_offsets();
    let mut flag: EFLAG = EFLAG::new(); 
    // Define memory blocks for task - Code(STATICS TEXT), STACK, DATA e EXTRA    
    // For now it will be a hard-coded begin, but that MUST be changed after
    let mut code_segment:WorkMemory = generate_work_memory(); 
    let mut stack_segment:WorkMemory = generate_work_memory(); 
    let mut data_segment:WorkMemory= generate_work_memory(); 
    let mut extra_segment:WorkMemory = generate_work_memory(); 
    code_segment.write(0,20);
    stack_segment.write(0,21);
    data_segment.write(0,23);
    extra_segment.write(0,24);
    // SEGMENT SELECTOR INIT
    segment_selectors.write_to_register(String::from("cs"), code_segment.cells[0] as u16); 
    segment_selectors.write_to_register(String::from("ss"), stack_segment.cells[0] as u16); 
    segment_selectors.write_to_register(String::from("ds"), data_segment.cells[0] as u16);
    segment_selectors.write_to_register(String::from("es"), extra_segment.cells[0] as u16);  
    offsets.write_to_register(String::from("eip"), segment_selectors.cs as u32);
    main_registers.quick_start((0,20,32,15));
    flag.over_flow_test();
    // PRE-TEST -> SEING THE CONTENTS of MEMORY: 
    return (segment_selectors.cs,segment_selectors.ss,segment_selectors.ds,segment_selectors.es); 
}