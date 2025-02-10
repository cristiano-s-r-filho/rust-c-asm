use crate::registers::*;
use crate::chips::mmu::*;  
use crate::memory::main_memory::*;
use crate::describe_working_states; 

pub fn add(work_env:(WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu: MMU, dst: u32, src: u32) {
    // ADD DST, SRC; Add SRC to DST
    // Mutable borrows here. 
    let mut work_env = work_env; 
    let mut mmu = mmu; 
    // 1. Increment program counter.
    // increment_program_counter 
    // get_from_data_bus
    // get_from_adress_bus
    // forward_to_data_bus
    // forward_to_adress_bus  
    work_env.2.increment_program_counter();

    let mut adrr = work_env.2.read_from_register(String::from("eip"));
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(adrr as usize);

    work_env.2.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!
    let end1 = mmu.get_from_adress_bus(); 
    work_env.2.write_to_register(String::from("edi"), end1);
    work_env.2.write_to_register(String::from("esi"), end1);

    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!

    let x = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("eax"), x);

    adrr = work_env.2.read_from_register(String::from("eip"));
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(adrr as usize);

    work_env.2.increment_program_counter();

    let end2 = mmu.get_from_data_bus();
    work_env.2.write_to_register(String::from("esi"), end2);

    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!

    let y = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("ebx"), y);

    let sum = x + y;
    work_env.1.write_to_register(String::from("eax"), sum);
    adrr = work_env.2.read_from_register(String::from("edi"));
    // (TRANSFORMAR EM FÍSICO?)  DS !!
    mmu.forward_to_adress_bus(adrr as usize);
    mmu.foward_to_data_bus(work_env.1.eax);
    // ESCREVER SUM EM ADRR !!
}