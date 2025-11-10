//! # Call Stack Module
//!
//! This module defines the `CallStack` struct, which simulates a call stack
//! for the ARC CPU. It provides basic stack operations like push, pop, and peek,
//! with checks for overflow and underflow.

use std::vec::Vec; 
/// Represents a fixed-size call stack for storing return addresses.
#[derive(Debug, Clone)]
pub struct CallStack {
    /// The underlying vector used to store stack elements.
    pub stack: Vec<u32>,
    /// The maximum capacity of the call stack.
    max_size: usize,
}

impl Default for CallStack {
    /// Creates a new `CallStack` with default capacity.
    fn default() -> Self {
        Self::new()
    }
}

impl CallStack {
    /// Creates a new `CallStack` with a default maximum size.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            max_size: 256, // Reasonable default
        }
    }
    
    /// Creates a new `CallStack` with a specified maximum capacity.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum number of elements the stack can hold.
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    /// Pushes an address onto the call stack.
    ///
    /// # Arguments
    ///
    /// * `address` - The `u32` address to push.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on success, or an error message if the stack overflows.
    pub fn push(&mut self, address: u32) -> Result<(), String> {
        if self.stack.len() >= self.max_size {
            return Err("Call stack overflow".to_string());
        }
        self.stack.push(address);
        Ok(())
    }
    
    /// Pops an address from the call stack.
    ///
    /// # Returns
    ///
    /// * `Result<u32, String>` - The popped `u32` address on success, or an error message if the stack underflows.
    pub fn pop(&mut self) -> Result<u32, String> {
        self.stack.pop().ok_or("Call stack underflow".to_string())
    }
    
    /// Peeks at the top address of the call stack without removing it.
    ///
    /// # Returns
    ///
    /// * `Result<u32, String>` - The `u32` address at the top of the stack on success, or an error message if the stack is empty.
    pub fn peek(&self) -> Result<u32, String> {
        self.stack.last().cloned().ok_or("Call stack empty".to_string())
    }
    
    /// Clears all elements from the call stack.
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    /// Returns the current number of elements in the call stack.
    ///
    /// # Returns
    ///
    /// * `usize` - The current depth of the stack.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    
    /// Checks if the call stack is empty.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the stack is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}