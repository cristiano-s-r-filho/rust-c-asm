use crate::chips::crom::CPU;
use crate::describe_working_states;

pub fn xchg(cpu: &mut CPU, src: u32, dst: u32) {
    let mut mmu = cpu.crom.mmu; 

    cpu.offsets.increment_program_counter();
    // describe_working_states(work_env, mmu, true, true);

    let mut adrr = cpu.offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs, 0xffff, adrr, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    cpu.offsets.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!

    let end1 = mmu.get_from_data_bus(); 
    cpu.offsets.write_to_register("edi", end1);
    cpu.offsets.write_to_register("esi", end1);
    describe_working_states(&cpu, true, true);

    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xffff, end1, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    //LER RAM EM ADRR POR EM DATA BUS !!

    let x = src;
    cpu.main_reg.write_to_register("eax", x);
    describe_working_states(&cpu, true, true);

    adrr = cpu.offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs,0xffff, adrr, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    // LER RAM EM ADRR E POR EM DATA BUS !!

    cpu.offsets.increment_program_counter();

    let end2 = mmu.get_from_data_bus();
    cpu.offsets.write_to_register("esi", end2);
    describe_working_states(&cpu, true, true);

    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xffff, end2, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    //LER RAM EM ADRR E POR EM DATA BUS !!

    let y = dst;
    cpu.main_reg.write_to_register("ebx", y);
    describe_working_states(&cpu, true, true);

    mmu.forward_to_adress_bus(end1 as usize);
    describe_working_states(&cpu, false, false);
    mmu.foward_to_data_bus(y);

    // ESCREVER Y EM END 1
    describe_working_states(&cpu, false, true);
    mmu.forward_to_adress_bus(end2 as usize);
    describe_working_states(&cpu, false, false);
    mmu.foward_to_data_bus(x);
    describe_working_states(&cpu, false, true);
    // ESCREVER X EM END2
    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
}