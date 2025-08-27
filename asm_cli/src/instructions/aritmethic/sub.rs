use crate::chips::crom::CPU;
// use crate::describe_working_states;
pub fn sub(cpu: &mut CPU, src: u32, dst: u32) {
    // SUB DST, SRC; Subtract SRC from DST
    // Ok, how does it work.: 
    //  
    // First, create usefull bounds - MMU, Flag, Main registers, segments and Offsets; 
    // let mmu = &mut cpu.crom.mmu; 
    let flag = &mut cpu.flag;
    let mains = &mut cpu.main_reg; 
    let offsets = &mut cpu.offsets; 
    // let mut segments = &mut cpu.segment_reg; 
    // 1. Increment program counter.
    offsets.increment_program_counter();
    flag.over_flow_test();
    // describe_working_states(&cpu, false, false);
    // 2. Take the code segments? 
    // 3. increase the program counter again! 
    offsets.increment_program_counter();
    flag.over_flow_test();
    // 4. take the value from SRC and load to EBX. 
    mains.write_to_register("ebx", src);
    // 5. Increase the program counter again; 
    offsets.increment_program_counter();
    flag.over_flow_test();
    // 6. Take the value of DST and load to EAX. 
    mains.write_to_register("eax", dst);
    // 7. Realize the operation.
    mains.eax -= mains.ebx; 
    mains.ebx = 0;
    flag.over_flow_test();
    // 8. Return the result to the memory. <--- NOT IMPLEMENTED YET.
}