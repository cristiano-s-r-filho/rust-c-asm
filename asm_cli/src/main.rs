extern crate asm_cli;
use asm_cli::*;
use chips::{crom::{generate_gdt, AcessLevel}, mmu::MMU};
use colored::Colorize;
use inline_colorization::*;
use std::io; 
use chips::alu::*; 
fn main() {
    // BEGINNING - NAME PRINTING
    println!("{color_cyan} ------ RUST ASM SIMULATOR ------- {color_reset}");
    println!("{color_cyan} MADE WITH SWEAT AND TEARS BY: "); 
    let names = vec!["Cristiano S.R. Filho", "Thales de Carvalho","Cauã M. Rosa", "Carlos Rafael", "Pedro Antonio"];    
    let colors = vec![color_bright_red, color_bright_blue, color_bright_green, color_bright_yellow, color_bright_magenta];
    let mut index = 0;  
    for i in names {
        let chosen_color = colors[index];
        println!("{color_cyan}[-] - {chosen_color}{}{color_reset} {color_reset}", i);
        index += 1;
    }
    // Prompt User for: 
    println!("Olá! Preencha o formulário abaixo:");    
    // - Initial code adress. 
    println!("ENDEREÇO INICIAL DO SEGMENTO DE CÓDIGO:"); 
    let mut code_base = String::new(); 
    io::stdin().read_line(&mut code_base).expect("Failed to get segment's initial value");
    let code_base= code_base.trim().trim_start_matches("0x").trim_start_matches("0X");
    let code_based  = u32::from_str_radix(code_base, 16); 
    // - Segment size for code. 
    println!("INDIQUE O TAMANHO DOS SEGMENTOS: ");
    println!("CODE -> "); 
    let mut code_size = String::new(); 
    io::stdin().read_line(&mut code_size).expect("Failed to get input for CODE");
    let code_size= code_size.trim().trim_start_matches("0x").trim_start_matches("0X"); 
    let code_sized = u32::from_str_radix(code_size, 16); 
    // - Segment size for stack. 
    println!("STACK -> ");
    let mut stack_size = String::new(); 
    io::stdin().read_line(&mut stack_size).expect("Failed to get input for STACK"); 
    let stack_size= stack_size.trim().trim_start_matches("0x").trim_start_matches("0X");
    let stack_sized = u32::from_str_radix(stack_size, 16);   
    // - Segment size for data. 
    println!("DATA -> "); 
    let mut data_size = String::new(); 
    io::stdin().read_line(&mut data_size).expect("Failed to get input for DATA"); 
    let data_size = data_size.trim().trim_start_matches("0x").trim_start_matches("0X");
    let data_sized = u32::from_str_radix(&data_size, 16); 
    // - Segment size for extra.  
    println!("EXTRA -> ");
    let mut extra_size = String::new(); 
    io::stdin().read_line(&mut extra_size).expect("Failed to get input for EXTRA");
    let extra_size= extra_size.trim().trim_start_matches("0x").trim_start_matches("0X");
    let extra_sized = u32::from_str_radix(extra_size, 16);    
    // For now, see if it works. 
    let init_segment:u32; 
    let code_init:u32; 
    let stack_init:u32; 
    let data_init:u32; 
    let extra_init:u32; 
    // Getting a feasible u32 value. 
    match code_based {
        Ok(value) => {
            init_segment = value;  
        }, 
        Err(_) => {
            init_segment = 0x0; 
        } 
    };
    match code_sized {
        Ok(value) => code_init = value, 
        Err(_) => code_init = 0xFFFFF 
    };
    match stack_sized {
        Ok(value) => stack_init = value, 
        Err(_) => stack_init = 0xFFFFF 
    };
    match data_sized {
        Ok(value) => data_init = value, 
        Err(_) => data_init = 0xFFFFF
    } 
    match extra_sized {
        Ok(value) => extra_init = value, 
        Err(_) => extra_init = 0xFFFFF
    }
    let mut process = (init_segment,code_init,stack_init,data_init,extra_init);
    let mut _alu = ALU::new();
    // Generate Enviroment: 
    let mut mmu = MMU::new(); 
    let general_env = mmu.start_process_manager(&mut process);
    let work_enviroment = &mut (general_env.0, general_env.1, general_env.3,general_env.2, general_env.4);
    describe_cpu_state(work_enviroment, mmu);

    // - Full instruction: 4byte intruction adress - instruction - operand 1 - operand2
    loop {
        println!("QUAL A INSTRUÇÃO A SER EXECUTADA?");
        let mut instruction = String::new(); 
        io::stdin().read_line(&mut instruction).expect("Failed to get instruction!!");  
        // parse input:
        let input_vec: Vec<&str> = instruction.split_whitespace().collect(); 
        // Loop - Execution cicle - 
        let instruction_adress = input_vec[0]; 
        let hex_adrr = u32::from_str_radix(instruction_adress, 16);
        let inst_adrr:u32; 
        match hex_adrr {
            Ok(value) => inst_adrr = value,
            Err(_) => inst_adrr = 0x0001
        }
        let opcode = input_vec[1]; 
        let operands = &input_vec[2..];
        let num_operands: & mut[u32] = &mut [0,0];
        let mut index = 0; 
        for item in operands {
            let hex_operand = u32::from_str_radix(&item, 16); 
            match hex_operand {
                Ok(values) => num_operands[index] = values,
                Err(_) => num_operands[index] = 0x0
            }
            index += 1; 
        }
        work_enviroment.2.write_to_register(String::from("eip"), inst_adrr);
        let mut alu = ALU::new();
        alu.execute_instruction(work_enviroment, &mut mmu, opcode, num_operands[0], num_operands[1]);
        println!("Continue? (Y) for yes and (N) for No"); 
        let mut response = String::new(); 
        io::stdin().read_line(&mut response).expect("Generate a valid response");
        let str_response = response.trim();
        let keep_going = if str_response == "Y" {true} else {false}; 
        
        if work_enviroment.4.ovfw == true || keep_going == false {
            break; 
        }
    }
    // END of LOOP. 
    // Global Descriptor Table; 
    let mut global_table =  generate_gdt();
    let mut indexing = 0; 
    let segment_vec = vec![mmu.code_summary, mmu.stack_summary, mmu.data_summary, mmu.extra_sumary]; 
    for item in &mut global_table.content[0..4] {
        item.selector = segment_vec[indexing].0; 
        item.base = segment_vec[indexing].2; 
        item.limit = segment_vec[indexing].3; 
        item.acess_level = AcessLevel::USER;
        indexing += 1; 
    }
    println!("{}", "--------------------------- GLOBAL DESCRIPTOR TABLE ----------------------------".cyan().bold());
    for item in &global_table.content[0..20]{
        let selects = item.selector;
        let base  = item.base;
        let limit = item.limit;
        let level:AcessLevel = item.acess_level; 
        println!("SELECTOR: {color_red}{}{color_reset} | BASE-ADRR: {color_blue}{:#x}{color_reset} | LIMIT-ADRR: {color_yellow}{:#x}{color_reset} | ACESS-LEVEL: {color_green}{:?}{color_reset}", selects,base,limit,level);
    }
    println!("{}", "-------------------------------- END OF TABLE ----------------------------------".cyan().bold());
}    
