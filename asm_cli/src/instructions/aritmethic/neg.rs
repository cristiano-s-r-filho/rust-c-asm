pub fn neg(){
    // NEG DST; Negate DST (subtract it from 0)
    increment_program_counter();

    let mut adrr = OffsetRegisters::read_from_register("eip");
    // (TRANSFORMAR EM FISICO?)  CS !!
    forward_to_adress_bus(adrr);

    increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!
    let end1 = get_from_data_bus();
    OffsetRegisters::write_to_register("edi", end1);
    OffsetRegisters::write_to_register("esi", end1);

    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!

    let x = get_from_data_bus();
    MainRegisters::write_to_register("eax", x);

    let neg = -x;
    MainRegisters::write_to_register("eax", neg);
    adrr = OffsetRegisters::read_from_register("edi");
    // (TRANSFORMAR EM FÍSICO?)  DS !!
    forward_to_adress_bus(adrr);
    foward_to_data_bus(MainRegisters::read_from_register("eax"));
    // ESCREVER NEG EM ADRR !!
}