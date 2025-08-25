use crate::chips::crom::CPU;
use crate::describe_working_states;

pub fn neg(cpu: &mut CPU, src: u32){
    // NEG DST; Negate DST (subtract it from 0)
    // Mutability enable; 
    let mut mmu = cpu.crom.mmu; 

    // mmu.foward_to_data_bus(0xF7D8 as u32);
    cpu.offsets.increment_program_counter();
    // describe_working_states(work_env, mmu, true, true);

    let mut adrr = cpu.offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs,0xfff, adrr, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);
 
    cpu.offsets.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!

    let end1 = mmu.get_from_data_bus();
    cpu.offsets.write_to_register("edi", end1);
    cpu.offsets.write_to_register("esi", end1);
    describe_working_states(&cpu, true, true);


    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xffff, adrr, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);

    // LER RAM EM ADRR E POR EM DATA BUS !!

    let x = src;
    cpu.main_reg.write_to_register("eax", x);
    describe_working_states(&cpu, true, true);

    let neg = x as i32;
    let neg = -neg;
    cpu.main_reg.write_to_register("eax", neg as u32);
    adrr = cpu.offsets.read_from_register("edi");
    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xffff, adrr, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    describe_working_states(&cpu, false, false);
    mmu.foward_to_data_bus(cpu.main_reg.eax);
    describe_working_states(&cpu, true, false);
    // ESCREVER NEG EM ADRR !!
    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
}