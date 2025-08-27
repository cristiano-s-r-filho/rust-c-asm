use crate::chips::crom::CPU;
// use crate::describe_working_states;

pub fn mov(cpu: &mut CPU, src: u32, dst: u32) {
    // MOV SRC, DST; Move SRC to DST. 
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
    // 4. Take the first value. Load do EBX
    mains.write_to_register("ebx", src); 
    // 5. Increase P.C. again; 
    offsets.increment_program_counter();
    flag.over_flow_test();
    // 6. Take the second value. Load to EAX
    mains.write_to_register("eax", dst); 
    // 7. Operate; <- Cant do right now. 
    // 8. Return;  

}