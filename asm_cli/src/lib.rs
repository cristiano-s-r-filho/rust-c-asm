pub mod chips;
pub mod instructions;
pub mod memory;
pub mod utils;

use chips::cpu::CPU;
use memory::main_memory::WorkMemory;

#[derive(Debug)]
pub struct Emulator {
    pub cpu: CPU,
    pub memory: WorkMemory,
}

#[derive(Debug, PartialEq)]
pub enum EmulatorMode {
    Interactive,
    File,
    Running,
    Paused,
}

impl Emulator {
    pub fn new(memory_size: usize) -> Self {
        Self {
            cpu: CPU::new(),
            memory: WorkMemory::new(memory_size),
        }
    }
    
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory = WorkMemory::new(self.memory.size);
    }
    
    pub fn load_program(&mut self, program: &str) -> Result<(), String> {
        self.cpu.load_program(program)
    }
}
