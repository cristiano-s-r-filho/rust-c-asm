use crate::registers::*;
use crate::chips::mmu::*;  
use crate::memory::main_memory::*;
pub fn mul(work_env:(WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu: MMU) {
    // MUL SRC; Multiply EAX by SRC (unsigned)
    // Mutability enable.
    let mut work_env = work_env; 
    let mut mmu = mmu; 
    work_env.2.increment_program_counter();

    let adrr = work_env.2.read_from_register(String::from("eip"));
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(adrr as usize);

    work_env.2.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!
    let end1 = mmu.get_from_data_bus();
    work_env.2.write_to_register(String::from("esi"), end1);

    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!

    let val = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("ebx"), val);

    let x = work_env.1.eax;

    let mul = x * val;
    work_env.1.write_to_register(String::from("eax"), mul);
}