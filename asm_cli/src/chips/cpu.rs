use crate::memory::main_memory::WorkMemory;
use crate::memory::registers::Registers;
use super::instruction_queue::{InstructionQueue, Instruction}; 
use super::call_stack::CallStack; 

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub instruction_queue: InstructionQueue,
    pub call_stack: CallStack,
    pub symbol_table: std::collections::HashMap<String, u32>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: crate::memory::registers::Registers::new(),
            instruction_queue: InstructionQueue::new(),
            call_stack: CallStack::new(),
            symbol_table: std::collections::HashMap::new(),
        }
    }
    
    pub fn reset(&mut self) {
        self.registers.reset();
        self.instruction_queue.clear();
        self.call_stack.clear();
        self.symbol_table.clear();
    }
    
    pub fn step(&mut self, memory: &mut WorkMemory) -> Result<(), String> {
        if self.instruction_queue.is_empty() {
            return Err("No instructions in queue".to_string());
        }
        
        let pc = self.registers.pc as usize;
        if pc >= self.instruction_queue.len() {
            return Err("Program counter out of bounds".to_string());
        }
        
        // Get the instruction from the queue
        let instruction = self.instruction_queue.get(pc)
            .ok_or("Instruction not found".to_string())?;
        
        // Convert Instruction to Command for execution
        let command = crate::utils::command_processor::Command {
            opcode: instruction.opcode.clone(),
            operand1: instruction.operand1.clone(),
            operand2: instruction.operand2.clone(),
        };
        
        // Execute the command
        crate::utils::command_processor::execute_command(command, self, memory)?;
        
        // Increment program counter (unless it was modified by a jump)
        if self.registers.pc == pc as u32 {
            self.registers.pc += 1;
        }
        
        Ok(())
    }
    
    pub fn add_label(&mut self, label: String) {
        let address = self.instruction_queue.len() as u32;
        self.symbol_table.insert(label, address);
    }
    
    pub fn add_instruction_to_queue(&mut self, opcode: String, operand1: Option<String>, operand2: Option<String>) -> Result<(), String> {
        // Parse operands
        let op1 = operand1.and_then(|s| crate::utils::operands::parse_operand(&s).ok());
        let op2 = operand2.and_then(|s| crate::utils::operands::parse_operand(&s).ok());
        
        let address = self.instruction_queue.len() as u32;
        let instruction = Instruction {
            opcode,
            operand1: op1,
            operand2: op2,
            address,
        };
        
        self.instruction_queue.push(instruction);
        Ok(())
    }
    
    pub fn execute_all(&mut self, memory: &mut WorkMemory) -> Result<(), String> {
        while self.registers.pc < self.instruction_queue.len() as u32 {
            self.step(memory)?;
        }
        Ok(())
    }
    
    pub fn load_program(&mut self, program: &str) -> Result<(), String> {
        self.instruction_queue.clear();
        self.symbol_table.clear();
        
        let mut address = 0;
        for line in program.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            
            // Check if line is a label
            if line.ends_with(':') {
                let label_name = line[..line.len()-1].trim().to_string();
                self.symbol_table.insert(label_name, address);
                continue;
            }
            
            // Parse the instruction
            let command = crate::utils::command_processor::parse_command(line)?;
            let instruction = Instruction {
                opcode: command.opcode,
                operand1: command.operand1,
                operand2: command.operand2,
                address,
            };
            
            self.instruction_queue.push(instruction);
            address += 1;
        }
        
        self.registers.pc = 0;
        Ok(())
    }
    
    pub fn get_label_address(&self, label: &str) -> Option<u32> {
        self.symbol_table.get(label).cloned()
    }
}

// Symbol Table implementation
#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: std::collections::HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, label: String, address: u32) {
        self.symbols.insert(label, address);
    }
    
    pub fn get(&self, label: &str) -> Option<u32> {
        self.symbols.get(label).cloned()
    }
    
    pub fn clear(&mut self) {
        self.symbols.clear();
    }
    
    pub fn contains(&self, label: &str) -> bool {
        self.symbols.contains_key(label)
    }
}