use crate::chips::crom::CPU;
// use crate::describe_working_states;

pub fn push(cpu: &mut CPU, src: u32) {
    // PUSH SRC; Put SRC on the top of the stack.
    // 
    // First, as always, 
    let mut mmu = cpu.crom.mmu; 
    let offsets = &mut cpu.offsets;
    let mains = &mut cpu.main_reg;

    offsets.increment_program_counter();
    
    
    let mut adrr = offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs, 0xffff, adrr, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
    

    offsets.increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!

    let end1 = mmu.get_from_data_bus(); 
    offsets.write_to_register("edi", end1);
    offsets.write_to_register("esi", end1);
    
    adrr = mmu.fisical_adress(cpu.segment_reg.ds,0xfff,  end1, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);    

    //LER RAM EM ADRR POR EM DATA BUS !!
    // Operate.: 
    let x = src;
    mains.write_to_register("eax", x);

    let mut top = offsets.read_from_register("esp");
    top = top - 2;
    offsets.write_to_register("esp", top);
    top = mmu.fisical_adress(cpu.segment_reg.ss,0xffff, top, cpu.flag);
    mmu.forward_to_adress_bus(top as usize);
    mmu.foward_to_data_bus(x);
    
}