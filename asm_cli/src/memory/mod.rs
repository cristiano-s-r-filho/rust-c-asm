//! # Memory Module
//!
//! This module defines the memory-related components of the ARC CPU,
//! including the main working memory and CPU registers.

pub mod main_memory; 
pub mod registers; 
use crate::memory::main_memory::WorkMemory;
fn _generate_work_memory() -> WorkMemory {
    let work_memory:WorkMemory = WorkMemory::new(1024);
    work_memory
}
