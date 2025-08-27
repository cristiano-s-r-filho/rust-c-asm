use crate::chips::crom::CPU;

pub fn or(cpu: &mut CPU, src: u32, dst: u32){
    // OR SRC, DST -> Operate Bitwise OR and return to DST
    // 
    // First -> Usefull bounds.:
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
    // 4. Take SRC and put on EBX; 
    mains.write_to_register("ebx", src);
    // 5. Increment P.C. again.
    offsets.increment_program_counter();
    flag.over_flow_test();
    // 6. Take DST and put it on EAX; 
    mains.write_to_register("eax", dst);
    // 7. Operate
    mains.eax |= mains.ebx;
    // 8. Return.
    
}