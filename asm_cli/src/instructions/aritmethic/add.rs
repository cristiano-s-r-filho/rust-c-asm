use crate::registers::*;
use crate::chips::mmu::*;  
use crate::memory::main_memory::*;
use crate::describe_working_states; 

pub fn add(work_env:(WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu: MMU, dst: u32, src: u32) {
    // ADD DST, SRC; Add SRC to DST
    // Mutable borrows here. 
    let mut work_env = work_env; 
    let mut mmu = mmu; 
    let mut flag = work_env.4;
    // 1. Increment program counter.
    // increment_program_counter 
    // get_from_data_bus
    // get_from_adress_bus
    // forward_to_data_bus
    // forward_to_adress_bus  
    work_env.2.increment_program_counter();
    describe_working_states(work_env, mmu, true, false);
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

    end1 = mmu.fisical_adress("ds", end1, work_env.4);
    mmu.forward_to_adress_bus(end1 as usize);
    // LER RAM EM END1 E POR EM DATA BUS !!

    let x = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("eax"), x);
    describe_working_states(work_env, mmu, true, true);

    adrr = work_env.2.read_from_register(String::from("eip"));
    adrr = mmu.fisical_adress("cs", adrr, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);

    // LER RAM EM ADRR E POR EM DATA BUS !!

    work_env.2.increment_program_counter();

    let mut end2 = mmu.get_from_data_bus();
    work_env.2.write_to_register(String::from("esi"), end2);
    describe_working_states(work_env, mmu, true, true);

    end2 = mmu.fisical_adress("ds", end2, work_env.4);
    mmu.forward_to_adress_bus(end2 as usize);
    describe_working_states(work_env, mmu, false, false);

    // LER RAM END2 E POR EM DATA BUS !!

    let y = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("ebx"), y);
    describe_working_states(work_env, mmu, true, true);

    let sum = x + y;
    work_env.1.write_to_register(String::from("eax"), sum);
    adrr = work_env.2.read_from_register(String::from("edi"));
    if sum == 0 {
        flag.zero = true;
    }
    else if sum < 0 {
        flag.negv = true;
    }

    mmu.fisical_adress("ds", adrr, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);
    mmu.foward_to_data_bus(work_env.1.eax);
    describe_working_states(work_env, mmu, true, false);
    // ESCREVER SUM EM ADRR !!
}