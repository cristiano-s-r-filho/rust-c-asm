use crate::registers::*;
use crate::chips::mmu::*;  
use crate::memory::main_memory::*;
use crate::describe_working_states;

pub fn dec(work_env:&mut (WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu: &mut MMU){
    // DEC DST; Subtract 1 from DST
    // Enable Mutability 
    let work_env = work_env;
    let mmu = mmu; 

    mmu.foward_to_data_bus(0x48 as u32);
    work_env.2.increment_program_counter();
    describe_working_states(work_env, mmu, true, true);

    let mut adrr = work_env.2.read_from_register(String::from("eip"));
    adrr = mmu.fisical_adress("cs", adrr, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);

    work_env.2.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!

    let end1 = mmu.get_from_data_bus();
    work_env.2.write_to_register(String::from("edi"), end1);
    work_env.2.write_to_register(String::from("esi"), end1);
    describe_working_states(work_env, mmu, true, true);

    adrr = mmu.fisical_adress("ds", end1, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);
    
    // LER RAM EM ADRR E POR EM DATA BUS !!

    let x = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("eax"), x);
    describe_working_states(work_env, mmu, true, true);

    let dec = x - 1;

    work_env.1.write_to_register(String::from("eax"), dec);
    adrr = work_env.2.read_from_register(String::from("edi"));
    adrr = mmu.fisical_adress("ds", adrr, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);
    mmu.foward_to_data_bus(work_env.1.eax);
    describe_working_states(work_env, mmu, true, false);
    // ESCREVER DEC EM ADRR !!
}