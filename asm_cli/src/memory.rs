pub mod main_memory; 
pub mod registers; 
// USING CRATE's FUNCTIONALITYS; 
use crate::memory::main_memory::MainMemory;
use crate::memory::registers::MainRegisters; 
use crate::memory::registers::OffsetRegisters;
use crate::memory::registers::SegmentRegisters;
use crate::memory::registers::EFLAG;
// DECLARING WORKING CONSTANTS 
pub const SEGMENT_SIZE:usize = u16::MAX as usize;
pub const CODE_BEGIN: usize = 0;
pub const STACK_BEGIN: usize = CODE_BEGIN + SEGMENT_SIZE; 
pub const DATA_BEGIN: usize = STACK_BEGIN + SEGMENT_SIZE; 
pub const EXTRA_BEGIN: usize = DATA_BEGIN + SEGMENT_SIZE; 
pub const END_OF_TASK_SPACE: usize = EXTRA_BEGIN + SEGMENT_SIZE;
// generate functions
fn generate_main_memory() -> MainMemory {
    let work_memory:MainMemory = MainMemory::new();
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
    // Main memory e Register declaration and initialization; 
    let work_memory:MainMemory = generate_main_memory();
    let mut main_registers: MainRegisters = generate_main_registers(); 
    let mut segment_selectors: SegmentRegisters = generate_segment_selector(); 
    let mut offsets: OffsetRegisters = generate_offsets();
    let mut flag: EFLAG = EFLAG::new(); 
    // Define memory blocks for task - Code(STATICS TEXT), STACK, DATA e EXTRA    
    // For now it will be a hard-coded begin, but that MUST be changed after
    let code_segment:&[u32] = &work_memory.cells[0..SEGMENT_SIZE]; 
    let stack_segment:&[u32] = &work_memory.cells[STACK_BEGIN..DATA_BEGIN-1];
    let data_segment:&[u32] = &work_memory.cells[DATA_BEGIN..EXTRA_BEGIN-1];
    let extra_segment:&[u32] = &work_memory.cells[EXTRA_BEGIN..END_OF_TASK_SPACE];
    segment_selectors.write_to_register(String::from("cs"), code_segment[0] as u16); 
    segment_selectors.write_to_register(String::from("ss"), stack_segment[0] as u16); 
    segment_selectors.write_to_register(String::from("ds"), data_segment[0] as u16);
    segment_selectors.write_to_register(String::from("es"), extra_segment[0] as u16);  
    offsets.write_to_register(String::from("eip"), segment_selectors.cs as u32);
    main_registers.quick_start((0,20,32,15));
    flag.over_flow_test();
    // PRE-TEST -> SEING THE CONTENTS of MEMORY: 
    return (segment_selectors.cs,segment_selectors.ss,segment_selectors.ds,segment_selectors.es); 
}