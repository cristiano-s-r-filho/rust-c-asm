use crate::chips::crom::CPU;

pub fn not(cpu: &mut CPU, src: u32){

    let mut mmu = cpu.crom.mmu; 

    cpu.offsets.increment_program_counter(); // go to instruction address
    
    cpu.offsets.increment_program_counter();

    let mut addr:u32 = cpu.offsets.read_from_register("eip");    
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(addr as usize);


    let end1: u32 = mmu.get_from_adress_bus();
    cpu.offsets.write_to_register("edi",  end1);
    cpu.offsets.write_to_register("esi",  end1);

    let mut val: u32 = src;
    cpu.main_reg.write_to_register("eax", val);

    val = !val;

    cpu.main_reg.write_to_register("eax", val);

    addr = cpu.offsets.read_from_register("edi");
    // (TRANSFORMAR EM FÍSICO) DS


    mmu.forward_to_adress_bus(addr as usize);
    mmu.foward_to_data_bus(cpu.main_reg.eax);
    // clean buses
    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
    
}