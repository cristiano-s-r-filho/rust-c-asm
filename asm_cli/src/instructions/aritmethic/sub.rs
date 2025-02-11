use crate::registers::*;
use crate::chips::mmu::*;  
use crate::memory::main_memory::*;
use crate::describe_working_states;
pub fn sub(work_env:&mut (WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu:&mut MMU) {
    // SUB DST, SRC; Subtract SRC from DST
    let work_env = work_env; 
    let mmu = mmu;
    let mut flag = work_env.4;
    
    // mmu.foward_to_data_bus(0x29D8 as u32);
    work_env.2.increment_program_counter();
    // describe_working_states(work_env, mmu, true, true);

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

    //LER RAM EM ADRR POR EM DATA BUS !!

    let x = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("eax"), x);
    describe_working_states(work_env, mmu, true, true);

    adrr = work_env.2.read_from_register(String::from("eip"));
    adrr = mmu.fisical_adress("cs", adrr, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);

    // LER RAM EM ADRR E POR EM DATA BUS !!

    work_env.2.increment_program_counter();

    let end2 = mmu.get_from_data_bus();
    work_env.2.write_to_register(String::from("esi"), end2);
    describe_working_states(work_env, mmu, true, true);

    adrr = mmu.fisical_adress("ds", end2, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);

    //LER RAM EM ADRR E POR EM DATA BUS !!

    let y = mmu.get_from_data_bus();
    work_env.1.write_to_register(String::from("ebx"), y);
    describe_working_states(work_env, mmu, true, true);

    let sub = (x as i32) - (y as i32);
    if sub == 0 {
        flag.zero = true;
    }
    else if sub < 0 {
        flag.negv = true;
    }

    work_env.1.write_to_register(String::from("eax"), sub as u32);
    adrr = work_env.2.read_from_register(String::from("edi"));
    adrr = mmu.fisical_adress("ds", adrr, work_env.4);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(work_env, mmu, false, false);
    mmu.foward_to_data_bus(work_env.1.eax);
    describe_working_states(work_env, mmu, true, false);
    // ESCREVER SUB EM ADRR !!
    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
}