use crate::chips::crom::CPU;
// use crate::describe_working_states;

pub fn pop(cpu: &mut CPU,) {
    // POP DST?; Take the top of STACK and push to EAX? or DST?
    // 
    // First the normal bound were used to.: 
    let mut mmu = cpu.crom.mmu; 
    let offsets = &mut cpu.offsets;
    let mains = &mut cpu.main_reg;
    // 1. Increment the P.C. 
    offsets.increment_program_counter();
    // 2. Operate? 
    let mut top = offsets.read_from_register("esp");
    top = mmu.fisical_adress(cpu.segment_reg.ss,0xffff, top, cpu.flag);
    top = top + 2; 
    mmu.forward_to_adress_bus(top as usize);
    // 3. Put it in EAX. 
    let x = mmu.get_from_data_bus();
    offsets.decrease_stack_pointer();
    mains.write_to_register("eax", x);
    // 4. Profit? 
    let mut adrr = offsets.read_from_register("eip");
    adrr = mmu.fisical_adress(cpu.segment_reg.cs, 0xffff, adrr, cpu.flag);
    mmu.forward_to_adress_bus(adrr as usize);
     
}