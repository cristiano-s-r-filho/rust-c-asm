pub mod main_memory; 
pub mod registers; 
// USING CRATE's FUNCTIONALITYS; 
use crate::memory::main_memory::WorkMemory;
// DECLARING WORKING CONSTANTS - STARTS and ENDS for the segments.
// generate functions
fn _generate_work_memory() -> WorkMemory {
    let work_memory:WorkMemory = WorkMemory::new(1024);
    return work_memory;
}
