use crate::registers::*;
use crate::chips::mmu::*;  
use crate::memory::main_memory::*;
pub fn neg(work_env:(WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu: MMU){
    // NEG DST; Negate DST (subtract it from 0)
    // Mutability enable; 
    let mut work_env = work_env; 
    let mut mmu = mmu; 
    work_env.2.increment_program_counter();

    let mut adrr = work_env.2.read_from_register(String::from("eip"));
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(adrr as usize); 

    work_env.2.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!
    let end1 = mmu.get_from_data_bus();
    work_env.2.write_to_register(String::from("edi"), end1);
    work_env.2.write_to_register(String::from("esi"), end1);
    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!
    let x = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("eax"), x);

    let neg = x as i32;
    let neg = -neg;
    work_env.1.write_to_register(String::from("eax"), neg as u32);
    adrr = work_env.2.read_from_register(String::from("edi"));
    // (TRANSFORMAR EM FÍSICO?)  DS !!
    mmu.forward_to_adress_bus(adrr as usize);
    mmu.foward_to_data_bus(work_env.1.eax);
    // ESCREVER NEG EM ADRR !!
}