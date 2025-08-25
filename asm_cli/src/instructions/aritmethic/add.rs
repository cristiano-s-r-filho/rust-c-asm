use crate::chips::crom::CPU;
use crate::describe_working_states; 

pub fn add(cpu: &mut CPU, src: u32, dst: u32) {
    // ADD DST, SRC; Add SRC to DST
    // Mutable borrows here. 
    let mut mmu = cpu.crom.mmu; 
    let mut flag = cpu.flag;
    // 1. Increment program counter.
    // mmu.foward_to_data_bus(0x01D8 as u32);
    cpu.offsets.increment_program_counter();
    describe_working_states(&cpu, true, true);

    let mut adrr = cpu.offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs,0xFFFF, adrr, flag );
    mmu.forward_to_adress_bus(adrr as usize);

    describe_working_states(&cpu, false, false);

    cpu.offsets.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!
    mmu.foward_to_data_bus(dst);
    let end1 = mmu.get_from_data_bus(); 

    cpu.offsets.write_to_register("edi", end1);
    cpu.offsets.write_to_register("esi", end1);

    describe_working_states(&cpu, true, true);

    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xFFFF, end1, flag);
    mmu.forward_to_adress_bus(adrr as usize);

    describe_working_states(&cpu, false, false);

    //LER RAM EM ADRR POR EM DATA BUS !!

    let x = mmu.get_from_data_bus();
    cpu.main_reg.write_to_register("eax", x);

    describe_working_states(&cpu, true, true);

    adrr = cpu.offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs,0xFFFF, adrr, flag);
    mmu.forward_to_adress_bus(adrr as usize);

    describe_working_states(&cpu, false, false);

    // LER RAM EM ADRR E POR EM DATA BUS !!

    cpu.offsets.increment_program_counter();

    mmu.foward_to_data_bus(src);
    let end2 = mmu.get_from_data_bus();
    cpu.offsets.write_to_register("esi", end2);

    describe_working_states(&cpu, true, true);

    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xFFFF, end2, flag);
    mmu.forward_to_adress_bus(adrr as usize);
    
    describe_working_states(&cpu, false, false);

    //LER RAM EM ADRR E POR EM DATA BUS !!

    let y = mmu.get_from_data_bus();
    cpu.main_reg.write_to_register("ebx", y);

    describe_working_states(&cpu, true, true);

    let sum = (x as i32) + (y as i32);
    cpu.main_reg.write_to_register("eax", sum as u32);
    if sum == 0 {
        flag.zero = true;
    }
    else if sum < 0 {
        flag.negv = true;
    }

    cpu.main_reg.write_to_register("eax", sum as u32);
    adrr = cpu.offsets.read_from_register("edi");
    adrr = mmu.fisical_adress(cpu.segment_reg.ds, 0xFFFF, adrr, flag);
    mmu.forward_to_adress_bus(adrr as usize);

    describe_working_states(&cpu, false, false);

    mmu.foward_to_data_bus(cpu.main_reg.eax);

    describe_working_states(&cpu, true, false);

    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
    // ESCREVER SUM EM ADRR !!
}