pub fn add() {
    increment_program_counter();

    let mut adrr = read_from_register("eip");
    // (TRANSFORMAR EM FISICO?)  CS !!
    forward_to_adress_bus(adrr);

    increment_program_counter();

    // LER RAM EM ADRR E POR EM DATA BUS !!
    let end1 = get_from_data_bus();
    write_to_register("edi", end1);
    write_to_register("esi", end1);

    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!

    let x = get_from_data_bus();
    write_to_register("eax", x);

    adrr = read_from_register("eip");
    // (TRANSFORMAR EM FISICO?)  CS !!
    forward_to_adress_bus(adrr);

    increment_program_counter();

    let end2 = get_from_data_bus();
    write_to_register("esi", end2);

    // (TRANSFORMAR EM FÍSICO?)  DS !!
    // POR EM ADRR BUS, LER RAM, POR EM DATA BUS !!

    let y = get_from_data_bus();
    write_to_register("ebx", y);

    let sum = x + y;
    write_to_register("eax", sum);
    adrr = read_from_register("edi");
    // (TRANSFORMAR EM FÍSICO?)  DS !!
    forward_to_adress_bus(adrr);
    foward_to_data_bus(read_from_register("eax"));
    // ESCREVER SUM EM ADRR !!

}