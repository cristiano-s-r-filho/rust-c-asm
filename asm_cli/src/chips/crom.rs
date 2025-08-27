use crate::{chips::{alu::ALU, mmu::MMU}, MainRegisters, OffsetRegisters, SegmentRegisters, EFLAG };
use crate::instructions::{
    aritmethic::*,
    moves::*,
    bit_logic::*, 
};
// Everything needed for the tables. 
// Gettin responses from terminal.S
pub fn get_response() -> String {
    let mut input: String = String::new(); 
    std::io::stdin().read_line(&mut input).expect("ERR: Cannot read terminal input");
    return input;
}


pub struct CPU {
    pub main_reg: MainRegisters, 
    pub segment_reg: SegmentRegisters,
    pub offsets:OffsetRegisters,
    pub flag:EFLAG, 
    pub alu: ALU,
    pub crom: CROM,
}
impl CPU {
    pub fn new() -> Self {
        CPU { 
            main_reg: MainRegisters::new(),
            segment_reg: SegmentRegisters::new(),
            offsets: OffsetRegisters::new(),
            flag: EFLAG::new(),
            alu: ALU::new(),
            crom: CROM::new() 
        }
    }

    pub fn initialize_cpu_state(&mut self, init_segment: &mut (u32, u32, u32, u32, u32), init_work_values: (u32, u32, u32, u32)) {
        // Destructurate all of the initial segment values
        let code_head:usize = init_segment.0 as usize;
        let code_tail:usize = (code_head as u32 + init_segment.1) as usize;  
        let stack_head:usize = (code_tail + 1) as usize; 
        let stack_tail:usize = (stack_head as u32 + init_segment.2) as usize; 
        let data_head:usize = (stack_tail + 1) as usize;
        let data_tail:usize = (data_head as u32 + init_segment.3) as usize;
        let extra_head:usize = (data_tail + 1) as usize; 
        // let extra_tail: usize = (extra_head as u32 + init_segment.4) as usize; 
        // Base adress calculations.: 
        let code_base_adrr:u32 = code_head as u32;
        let stack_base_adrr:u32 = stack_head as u32;
        let dats_base_adrr:u32 = data_head as u32;
        let extra_base_adrr:u32 = extra_head as u32;  
        // Writing this to the CPU.: 
        self.segment_reg.write_to_register("cs", code_base_adrr); 
        self.segment_reg.write_to_register("ss", stack_base_adrr); 
        self.segment_reg.write_to_register("ds", dats_base_adrr); 
        self.segment_reg.write_to_register("es", extra_base_adrr);  
        self.offsets.write_to_register("eip", self.segment_reg.cs as u32);
        self.offsets.write_to_register("esp", stack_tail as u32);
        self.main_reg.quick_start(init_work_values);
    }

    pub fn execute_instruction(&mut self, opcode: &str, operand1: u32, operand2: u32) {
        match opcode {
            "add" => add::add(self, operand1, operand2),
            "sub" => sub::sub(self, operand1, operand2),
            "dec" => dec::dec(self, operand1),
            "inc" => inc::inc(self, operand1),
            "mul" => mul::mul(self, operand1),
            "neg" => neg::neg(self, operand1),
            "and" => and::and(self, operand1, operand2),
            "not" => not::not(self, operand1),
            "or"  => or::or(self, operand1, operand2),
            "xor" => xor::xor(self, operand1, operand2),
            "mov" => mov::mov(self, operand1, operand2),
            "pop" => pop::pop(self),
            "push" => push::push(self, operand1),
            // "xchg" => xchg::xchg(self, operand1, operand2),
            _ => self.alu.gpf = true 
        }; 
    }
}
/// L1 and L2 Definitions for this particular architecture. Basically two big memory slots
pub struct L1 {
    pub slots: Vec<u32>,
}
impl L1 {
    pub fn new() -> Self {
        L1 { slots: vec![0;1024] }
    }

    pub fn set_slot(&mut self, adress:usize, value: u32) {
        self.slots[adress] = value;
    }

    pub fn get_slot(&mut self, adress:usize) -> u32 {
        self.slots[adress]
    }
}

pub struct L2 {
    pub slots: Vec<u32>,
}
impl L2 {
    pub fn new() -> Self {
        L2 {
            slots: vec![0;1024]
        }
    }

    pub fn set_slot(&mut self, adress:usize, value: u32) {
        self.slots[adress] = value;
    }

    pub fn get_slot(&mut self, adress:usize) -> u32 {
        self.slots[adress]
    }
}

pub struct CROM {
    pub mmu: MMU,
    pub l1: L1,
    pub l2: L2, 
    pub current_level_of_execution: AcessLevel, 
    pub gpf: bool,
}
impl CROM {
    pub fn new() -> Self {
        CROM { 
            mmu: MMU::new(),
            l1: L1::new(),
            l2: L2::new(),
            current_level_of_execution: AcessLevel::KERNEL,
            gpf: false 
        }
    }
}
// SIMULATING DTables BELLOW: 
pub const  MAX_TABLE_SIZE: usize = 0x2000 as usize;
#[derive(Clone, Copy,Debug)]
pub enum AcessLevel {
    KERNEL,
    SYSTEMCALL,
    SHELL,
    USER
}
#[derive(Clone,Copy)]
pub struct DTEntry {
    pub selector:&'static str,
    pub base: u32,
    pub limit: u32,
    pub acess_level: AcessLevel,
}  
pub struct DTable {
    pub name: &'static str,
    pub content: Vec<DTEntry>,
    pub capacity: u16,
}

impl DTable {
    pub fn new(table_name:&'static str) -> DTable {
        let d_table = DTable {
            name:table_name,
            content: vec![DTEntry{
                selector: "NULL",
                base: 0, 
                limit: 0, 
                acess_level: AcessLevel::USER,
            }; MAX_TABLE_SIZE],
            capacity: 0, 
        };
        return d_table;
    }
}

pub fn generate_gdt() -> DTable {
    let mut gd_table : DTable = DTable::new("GLOBAL_D_TABLE");
    gd_table.content[0] = DTEntry {
        selector:"CS",
        base:0,
        limit:u16::MAX as u32, 
        acess_level: AcessLevel::KERNEL
    };
    return gd_table;  
}

pub fn generate_idt() -> DTable {
    let id_table: DTable = DTable::new("INTERRUPT_D_TABLE");
    return id_table;
}

pub fn generate_ldt() -> DTable {
    let ld_table: DTable = DTable::new("LOCAL_D_TABLE");
    return ld_table;
} 

