use crate::chips::crom::CPU;
// use crate::describe_working_states;

pub fn dec(cpu: &mut CPU, src: u32){
    // DEC DST; Subtract 1 from DST
    // First, create usefull bounds.: 
    let flag = &mut cpu.flag;
    let mains = &mut cpu.main_reg; 
    let offsets = &mut cpu.offsets;  
    // 1. Increment the Program Counter. 
    offsets.increment_program_counter();
    flag.over_flow_test();
    // 2. Tecnically take the code segments? 
    // 3. Increment P.C. again. 
    offsets.increment_program_counter();
    flag.over_flow_test();
    // 4. Take SRC and load into eax; 
    mains.write_to_register("eax", src);
    // 5. Increment P.C. again
    mains.eax -= 1; 
    // 6. Return the value. 
    
}