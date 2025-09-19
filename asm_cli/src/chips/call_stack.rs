use std::vec::Vec; 
#[derive(Debug, Clone)]
pub struct CallStack {
    pub stack: Vec<u32>,
    max_size: usize,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            max_size: 256, // Reasonable default
        }
    }
    
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn push(&mut self, address: u32) -> Result<(), String> {
        if self.stack.len() >= self.max_size {
            return Err("Call stack overflow".to_string());
        }
        self.stack.push(address);
        Ok(())
    }
    
    pub fn pop(&mut self) -> Result<u32, String> {
        self.stack.pop().ok_or("Call stack underflow".to_string())
    }
    
    pub fn peek(&self) -> Result<u32, String> {
        self.stack.last().cloned().ok_or("Call stack empty".to_string())
    }
    
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}