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
    // - Full instruction: 4byte intruction adress - instruction - operand 1 - operand2
    println!("QUAL A INSTRUÇÃO A SER EXECUTADA?");
    let mut instruction = String::new(); 
    io::stdin().read_line(&mut instruction).expect("Failed to get instruction!!");    
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

    // Loop - Execution cicle - 
    // Take a instruction from the top of execution queue. 
    // Pass it for the ALU. 
    let mut _alu = ALU::new(); 
    // Wait for execution. 
    // If GPF: STOP.  
    // Else : Continue till end of queue. 
    // GENERATE MMU - GENERAL ENV. - and describe general initial CPU STATE.
    let mut mmu = MMU::new(); 
    let general_env = mmu.start_process_manager(&mut process);
    let work_enviroment = (general_env.0, general_env.1, general_env.3,general_env.2, general_env.4);
    describe_cpu_state(work_enviroment, mmu);

    // Global Descriptor Table; 
    let global_table =  generate_gdt();
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
