pub mod main_memory; 
pub mod registers; 
// USING CRATE's FUNCTIONALITYS; 
use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::MainRegisters; 
use crate::memory::registers::OffsetRegisters;
use crate::memory::registers::SegmentRegisters;
use crate::memory::registers::EFLAG;
// DECLARING WORKING CONSTANTS - STARTS and ENDS for the segments.
pub const SEGMENT_SIZE:usize = u16::MAX as usize; 
pub const CODE_HEAD:usize = 0;
pub const CODE_TAIL:usize = (CODE_HEAD + SEGMENT_SIZE) as usize;  
pub const STACK_HEAD:usize = (CODE_TAIL + 1 ) as usize; 
pub const STACK_TAIL:usize = (STACK_HEAD + SEGMENT_SIZE) as usize; 
pub const DATA_HEAD:usize = (STACK_TAIL + 1) as usize;
pub const DATA_TAIL:usize = (DATA_HEAD + SEGMENT_SIZE) as usize;
pub const EXTRA_HEAD:usize = (DATA_TAIL + 1) as usize; 
pub const EXTRA_TAIL: usize = (EXTRA_HEAD + SEGMENT_SIZE) as usize; 
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

fn slice_segment_data(slice_name: &'static str, cursor:usize, end:u32, workplace: &WorkMemory) -> (&'static str,bool,u32,u32) {
    let isfull: bool;
    if &end != &workplace.cells[cursor] {
        isfull = false; 
    } else {
        isfull = true; 
    }    
    let sliced_segment:(&str,bool,u32,u32) = (slice_name,isfull,cursor as u32,end); 
    return sliced_segment;
}
pub fn initiate_working_env() -> ((u16,u16,u16,u16),((&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32)), WorkMemory, MainRegisters, OffsetRegisters,EFLAG) {
    // Register declaration and initialization; 
    let mut main_registers: MainRegisters = generate_main_registers(); 
    let mut segment_selectors: SegmentRegisters = generate_segment_selector(); 
    let mut offsets: OffsetRegisters = generate_offsets();
    let mut flag: EFLAG = EFLAG::new(); 
    // Define memory blocks for task - Code(STATICS TEXT), STACK, DATA e EXTRA    
    // For now it will be a hard-coded begin, but that MUST be changed after
    let mut work_memory: WorkMemory = generate_work_memory();
    let code_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"CODE", CODE_HEAD , CODE_TAIL as u32, &work_memory);
    let stack_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"STCK", STACK_HEAD , STACK_TAIL as u32, &work_memory); 
    let data_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"DATA", DATA_HEAD , DATA_TAIL as u32,&work_memory);
    let extra_segment_data:(&'static str,bool,u32,u32) = slice_segment_data(&"EXTR", EXTRA_HEAD , EXTRA_TAIL as u32,&work_memory); 
    work_memory.write(CODE_HEAD, 20);
    work_memory.write(STACK_HEAD, 21);
    work_memory.write(DATA_HEAD, 23);
    work_memory.write(EXTRA_HEAD,24); 
    // BASE ADRESS CALCULATION 
    let code_base_adrr:u16 = (CODE_HEAD / 0x10) as u16;
    let stack_base_adrr:u16 = (STACK_HEAD/ 0x10) as u16;
    let dats_base_adrr:u16 = (DATA_HEAD/ 0x10) as u16;
    let extra_base_adrr:u16 = (EXTRA_HEAD/ 0x10) as u16;   
    // SEGMENT SELECTOR INIT
    segment_selectors.write_to_register(String::from("cs"), code_base_adrr); 
    segment_selectors.write_to_register(String::from("ss"), stack_base_adrr); 
    segment_selectors.write_to_register(String::from("ds"), dats_base_adrr); 
    segment_selectors.write_to_register(String::from("es"), extra_base_adrr);  
    offsets.write_to_register(String::from("eip"), segment_selectors.cs as u32);
    main_registers.quick_start((10,11,14,15));
    // WARNING!! THIS FEATURE IS NOT ACTUALLY IMPLEMENTED, THIS IS A OBJECTIVE THOUGH!
    flag.over_flow_test();
    // PRE-TEST -> SEING THE CONTENTS of MEMORY: 
    return ((segment_selectors.cs,segment_selectors.ss,segment_selectors.ds,segment_selectors.es),(code_segment_data
        ,stack_segment_data,data_segment_data,extra_segment_data),work_memory, main_registers, offsets, flag); 
}