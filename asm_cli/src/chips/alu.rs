use crate::memory::main_memory::WorkMemory;
use crate::registers::*;
use crate::chips::mmu::MMU; 
use crate::instructions::aritmethic::*;
use crate::instructions::moves::*;
    
pub struct ALU {
    pub instruction: &'static str,
    pub lifetime:u32, 
    pub gpf: bool,  
} 
impl ALU {
    pub fn new() -> Self {
        ALU {instruction: "NULL", lifetime:0, gpf: false}
    } 
    pub fn execute_instruction(&mut self,working_env: &mut (WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), pro_mmu: &mut MMU ,instruction: &str, operand1:u32 , operand2:u32) {
        match instruction {
            "add" => add::add(working_env, pro_mmu, operand1, operand2),
            "sub" => sub::sub(working_env, pro_mmu),
            "dec" => dec::dec(working_env, pro_mmu),
            "inc" => inc::inc(working_env, pro_mmu),
            "mul" => mul::mul(working_env, pro_mmu),
            "neg" => neg::neg(working_env, pro_mmu),
            "mov" => mov::mov(working_env, pro_mmu), 
            _ => self.gpf = true 
        }
    }
}
