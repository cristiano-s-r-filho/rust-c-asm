use crate::chips::crom::CPU;

pub fn or(cpu: &mut CPU, src: u32, dst: u32){
    let mut mmu = cpu.crom.mmu; 

    cpu.offsets.increment_program_counter(); // go to instruction address
    
    cpu.offsets.increment_program_counter(); // 1° arg address
    
    let mut addr:u32 = cpu.offsets.read_from_register("eip");    
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(addr as usize);


    let end1: u32 = mmu.get_from_adress_bus();
    cpu.offsets.write_to_register("edi",  end1);
    cpu.offsets.write_to_register("esi",  end1);

    let mut val: u32 = src;
    cpu.main_reg.write_to_register("eax", val);

    cpu.offsets.increment_program_counter();  // 2° arg address

    addr = cpu.offsets.read_from_register("eip");
    // (TRANSFORMAR EM FISICO?)  CS !!
    mmu.forward_to_adress_bus(addr as usize);


    let end2 = mmu.get_from_adress_bus();
    cpu.offsets.write_to_register("esi", end2);

    cpu.main_reg.write_to_register("ebx", dst);
    val |= cpu.main_reg.ebx;

    cpu.main_reg.write_to_register("eax", val);

    addr = cpu.offsets.read_from_register("edi");
    // (TRANSFORMAR EM FÍSICO) DS


    mmu.forward_to_adress_bus(addr as usize);
    mmu.foward_to_data_bus(cpu.main_reg.eax);
    // Clean buses
    mmu.foward_to_data_bus(0);
    mmu.forward_to_adress_bus(0);
    
}