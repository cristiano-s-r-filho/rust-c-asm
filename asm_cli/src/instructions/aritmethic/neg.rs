use crate::chips::crom::CPU;
// use crate::describe_working_states;

pub fn neg(cpu: &mut CPU, src: u32){
    // NEG SRC; Negate SRC (subtract it from 0)
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
    let mut value = mains.eax as i32; 
    value = -value;
    mains.eax = value as u32;
    // 6. Return the value. 
}