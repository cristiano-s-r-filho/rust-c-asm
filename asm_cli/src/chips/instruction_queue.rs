//! # Instruction Queue Module
//!
//! This module defines the `Instruction` struct, representing a single decoded
//! instruction, and the `InstructionQueue` struct, which manages a sequence
//! of these instructions for execution.

use std::vec::Vec;
use crate::utils::assembler::operands::Operand;

/// Represents a single decoded instruction.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// The opcode of the instruction (e.g., "MOVI", "ADDW").
    pub opcode: String,
    /// The first operand of the instruction, if present.
    pub operand1: Option<Operand>,
    /// The second operand of the instruction, if present.
    pub operand2: Option<Operand>,
    /// The memory address where this instruction is stored.
    pub address: u32,
}

/// Manages a queue of `Instruction`s for sequential processing.
#[derive(Debug)]
pub struct InstructionQueue {
    /// The underlying vector storing the instructions.
    pub queue: Vec<Instruction>,
    /// The index of the next instruction to be retrieved.
    pub current_index: usize,
}

impl Default for InstructionQueue {
    /// Creates a new empty `InstructionQueue`.
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionQueue {
    /// Creates a new empty `InstructionQueue`.
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            current_index: 0,
        }
    }
    
    /// Clears the instruction queue and resets the current index.
    pub fn clear(&mut self) {
        self.queue.clear();
        self.current_index = 0;
    }
    
    /// Returns the number of instructions in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    
    /// Checks if the instruction queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Adds an `Instruction` to the end of the queue.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The `Instruction` to add.
    pub fn push(&mut self, instruction: Instruction) {
        self.queue.push(instruction);
    }
    
    /// Retrieves a reference to the instruction at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the instruction to retrieve.
    ///
    /// # Returns
    ///
    /// * `Option<&Instruction>` - A reference to the instruction if found, otherwise `None`.
    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.queue.get(index)
    }
    
    /// Retrieves a reference to the next instruction in the queue and advances the current index.
    ///
    /// # Returns
    ///
    /// * `Option<&Instruction>` - A reference to the next instruction if available, otherwise `None`.
    pub fn get_next(&mut self) -> Option<&Instruction> {
        if self.current_index < self.queue.len() {
            let instruction = &self.queue[self.current_index];
            self.current_index += 1;
            Some(instruction)
        } else {
            None
        }
    }
    
    /// Resets the current index to the beginning of the queue.
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}