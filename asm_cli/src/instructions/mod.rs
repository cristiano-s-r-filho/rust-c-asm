//! # Instructions Module
//!
//! This module defines and implements the various instructions that the ARC CPU
//! can execute. Instructions are categorized by their function, such as
//! arithmetic, data movement, bitwise operations, comparisons, system calls,
//! I/O operations, and control flow.

pub mod aritmethic;
pub mod moves; 
pub mod bitwise;
pub mod compare; 
pub mod system;
pub mod io;
pub mod control;