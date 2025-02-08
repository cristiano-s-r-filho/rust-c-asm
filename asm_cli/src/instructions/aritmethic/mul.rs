pub fn mul() {
    // MUL SRC; Multiply EAX by SRC (unsigned)
    increment_program_counter();

    let mut adrr = OffsetRegisters::read_from_register("eip");
    // (TRANSFORMAR EM FISICO?)  CS !!
    forward_to_adress_bus(adrr);

    increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!
    let end1 = get_from_data_bus();
    OffsetRegisters::write_to_register("esi", end1);

    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!

    let val = get_from_data_bus();
    MainRegisters::write_to_register("ebx", val);

    let x = MainRegisters::read_from_register("eax");

    let mul = x * val;
    MainRegisters::write_to_register("eax", mul);
}