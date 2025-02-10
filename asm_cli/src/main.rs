extern crate asm_cli;
use asm_cli::*;
use chips::{crom::{generate_gdt, AcessLevel}, mmu::MMU};
use colored::Colorize;
use inline_colorization::*;
use std::env; 
use std::fs; 
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
    
    // Loop - Execution cicle - 
    // Take a instruction from the top of execution queue. 
    // Pass it for the ALU. 
    // Wait for execution. 
    // If GPF: STOP.  
    // Else : Continue till end of queue. 
    // GENERATE MMU - GENERAL ENV. - and describe general initial CPU STATE.
    let mut mmu = MMU::new(); 
    let general_env = mmu.start_process_manager(vec![20, 25, 14, 17, 15,10]);
    let work_enviroment = (general_env.0, general_env.1, general_env.3,general_env.2, general_env.4);
    describe_cpu_state(work_enviroment);

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
