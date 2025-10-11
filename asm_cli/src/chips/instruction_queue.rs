use std::vec::Vec;
use crate::utils::operands::Operand;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: String,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
    pub address: u32, // Memory address where this instruction is stored
}

#[derive(Debug)]
pub struct InstructionQueue {
    pub queue: Vec<Instruction>,
    pub current_index: usize,
}

impl InstructionQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            current_index: 0,
        }
    }
    
    pub fn clear(&mut self) {
        self.queue.clear();
        self.current_index = 0;
    }
    
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    pub fn push(&mut self, instruction: Instruction) {
        self.queue.push(instruction);
    }
    
    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.queue.get(index)
    }
    
    pub fn next(&mut self) -> Option<&Instruction> {
        if self.current_index < self.queue.len() {
            let instruction = &self.queue[self.current_index];
            self.current_index += 1;
            Some(instruction)
        } else {
            None
        }
    }
    
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}