use crate::registers::*;
use crate::chips::mmu::*;  
use crate::memory::main_memory::*;

pub fn or(work_env:&mut(WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu:&mut MMU){

    let work_env:&mut (WorkMemory, MainRegisters, OffsetRegisters, SegmentRegisters, EFLAG) = work_env; 
    let mmu = mmu; 

    work_env.2.increment_program_counter(); // go to instruction address
    
    work_env.2.increment_program_counter(); // 1° arg address
    
    let mut addr:u32 = work_env.2.read_from_register(String::from("eip"));    
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(addr as usize);


    let end1: u32 = mmu.get_from_adress_bus();
    work_env.2.write_to_register(String::from("edi"),  end1);
    work_env.2.write_to_register(String::from("esi"),  end1);

    let mut val: u32 = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("eax"), val);

    work_env.2.increment_program_counter(); // 2° arg address

    addr = work_env.2.read_from_register(String::from("eip"));
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(addr as usize);


    let end2 = mmu.get_from_adress_bus();
    work_env.2.write_to_register(String::from("esi"), end2);

    val |= mmu.get_from_data_bus();

    work_env.1.write_to_register(String::from("eax"), val);

    addr = work_env.2.read_from_register(String::from("edi"));
    // (TRANSFORMAR EM FÍSICO) DS


    mmu.forward_to_adress_bus(addr as usize);
    mmu.foward_to_data_bus(work_env.1.eax);
    
}