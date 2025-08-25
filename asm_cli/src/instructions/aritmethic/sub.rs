use crate::chips::crom::CPU;
use crate::describe_working_states;
pub fn sub(cpu: &mut CPU, src: u32, dst: u32) {
    // SUB DST, SRC; Subtract SRC from DST
    let mut mmu = cpu.crom.mmu;
    let mut flag = cpu.flag;
    
    // mmu.foward_to_data_bus(0x29D8 as u32);
    cpu.offsets.increment_program_counter();
    // describe_working_states(work_env, mmu, true, true);

    let mut adrr = cpu.offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs,0xfff, adrr, flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    cpu.offsets.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!

    let end1 = mmu.get_from_adress_bus(); 
    cpu.offsets.write_to_register("edi", end1);
    cpu.offsets.write_to_register("esi", end1);
    describe_working_states(&cpu, true, true);

    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xffff, end1, flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    //LER RAM EM ADRR POR EM DATA BUS !!

    let x = src;
    cpu.main_reg.write_to_register("eax", x);
    describe_working_states(&cpu, true, true);

    adrr = cpu.offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs,0xffff, adrr, flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    // LER RAM EM ADRR E POR EM DATA BUS !!

    cpu.offsets.increment_program_counter();

    let end2 = mmu.get_from_adress_bus();
    cpu.offsets.write_to_register("esi", end2);
    describe_working_states(&cpu, true, true);

    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xfff,end2, flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    //LER RAM EM ADRR E POR EM DATA BUS !!

    let y = dst;
    cpu.main_reg.write_to_register("ebx", y);
    describe_working_states(&cpu, true, true);

    let sub = (x as i32) - (y as i32);
    if sub == 0 {
        flag.zero = true;
    }
    else if sub < 0 {
        flag.negv = true;
    }

    cpu.main_reg.write_to_register("eax", sub as u32);
    adrr = cpu.offsets.read_from_register("edi");
    adrr = mmu.fisical_adress(cpu.segment_reg.ds, 0xffff, adrr, flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);
    mmu.foward_to_data_bus(cpu.main_reg.eax);
    describe_working_states(&cpu, true, false);
    // ESCREVER SUB EM ADRR !!
    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
}