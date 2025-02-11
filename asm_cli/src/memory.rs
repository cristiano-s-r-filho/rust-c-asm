pub mod main_memory; 
pub mod registers; 
// USING CRATE's FUNCTIONALITYS; 
use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::MainRegisters; 
use crate::memory::registers::OffsetRegisters;
use crate::memory::registers::SegmentRegisters;
use crate::memory::registers::EFLAG;
// DECLARING WORKING CONSTANTS - STARTS and ENDS for the segments.
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

pub fn slice_segment_data(slice_name: &'static str, cursor:usize, end:u32, workplace: &WorkMemory) -> (&'static str,bool,u32,u32) {
    let isfull: bool;
    if &end != &workplace.cells[cursor] {
        isfull = false; 
    } else {
        isfull = true; 
    }    
    let sliced_segment:(&str,bool,u32,u32) = (slice_name,isfull,cursor as u32,end); 
    return sliced_segment;
}
pub fn initiate_working_env(init_segments:&mut (u32,u32,u32,u32,u32)) -> (((&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32)), WorkMemory, MainRegisters,SegmentRegisters, OffsetRegisters,EFLAG) { 
    let code_head:usize = init_segments.0 as usize;
    let code_tail:usize = (code_head as u32 + init_segments.1) as usize;  
    let stack_head:usize = (code_tail + 1) as usize; 
    let stack_tail:usize = (stack_head as u32 + init_segments.2) as usize; 
    let data_head:usize = (stack_tail + 1) as usize;
    let data_tail:usize = (data_head as u32 + init_segments.3) as usize;
    let extra_head:usize = (data_tail + 1) as usize; 
    let extra_tail: usize = (extra_head as u32 + init_segments.4) as usize; 
    // Register declaration and initialization; 
    let mut main_registers: MainRegisters = generate_main_registers(); 
    let mut segment_selectors: SegmentRegisters = generate_segment_selector(); 
    let mut offsets: OffsetRegisters = generate_offsets();
    let mut flag: EFLAG = EFLAG::new(); 
    // Define memory blocks for task - Code(STATICS TEXT), STACK, DATA e EXTRA    
    // For now it will be a hard-coded begin, but that MUST be changed after
    let work_memory: WorkMemory = generate_work_memory();
    let code_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"CODE", code_head , code_tail as u32, &work_memory);
    let stack_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"STCK", stack_head , stack_tail as u32, &work_memory); 
    let data_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"DATA", data_head , data_tail as u32,&work_memory);
    let extra_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"EXTR", extra_head, extra_tail as u32,&work_memory); 
    // BASE ADRESS CALCULATION 
    let code_base_adrr:u16 = (code_head / 0x10) as u16;
    let stack_base_adrr:u16 = (stack_head/ 0x10) as u16;
    let dats_base_adrr:u16 = (data_head/ 0x10) as u16;
    let extra_base_adrr:u16 = (extra_head/ 0x10) as u16;   
    // SEGMENT SELECTOR INIT
    segment_selectors.write_to_register(String::from("cs"), code_base_adrr); 
    segment_selectors.write_to_register(String::from("ss"), stack_base_adrr); 
    segment_selectors.write_to_register(String::from("ds"), dats_base_adrr); 
    segment_selectors.write_to_register(String::from("es"), extra_base_adrr);  
    offsets.write_to_register(String::from("eip"), segment_selectors.cs as u32);
    offsets.write_to_register(String::from("esi"), stack_tail as u32);
    main_registers.quick_start((10,11,14,15));
    // WARNING!! THIS FEATURE IS NOT ACTUALLY IMPLEMENTED, THIS IS A OBJECTIVE THOUGH!
    flag.over_flow_test();
    // PRE-TEST -> SEING THE CONTENTS of MEMORY: 
    return ((code_segment_data,stack_segment_data,data_segment_data,extra_segment_data),work_memory, main_registers,segment_selectors, offsets, flag); 
}

