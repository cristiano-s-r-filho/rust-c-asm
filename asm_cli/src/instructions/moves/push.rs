use crate::chips::crom::CPU;
use crate::describe_working_states;

pub fn push(cpu: &mut CPU, src: u32) {
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

    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xfff,  end1, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    //LER RAM EM ADRR POR EM DATA BUS !!

    let x = src;
    cpu.main_reg.write_to_register("eax", x);
    describe_working_states(&cpu, true, true);

    let mut top = cpu.offsets.read_from_register("esp");
    top = top - 2;
    cpu.offsets.write_to_register("esp", top);
    top = mmu.fisical_adress(cpu.segment_reg.ss,0xffff, top, cpu.flag);
    mmu.forward_to_adress_bus(top as usize);
    describe_working_states(&cpu, false, false);
    mmu.foward_to_data_bus(x);
    describe_working_states(&cpu, true, false);
    // ESCREVER X EM TOP
    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
}