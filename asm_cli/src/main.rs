extern crate asm_cli;
use asm_cli::*;
use chips::{crom::{generate_gdt, get_response, AcessLevel}, mmu::MMU};
use colored::Colorize;
use inline_colorization::*;
fn main() {
    println!("{color_cyan} ------ RUST ASM SIMULATOR ------- {color_reset}");
    println!("{color_cyan} MADE WITH SWEAT AND TEARS BY: "); 
    let names = vec!["Cristiano S.R. Filho", "Thalles de Carvalho","Cauã M. Rosa", "Carlos Rafael", "Pedro Antonio"];    
    let colors = vec![color_bright_red, color_bright_blue, color_bright_green, color_bright_yellow, color_bright_magenta];
    let mut index = 0;  
    for i in names {
        let chosen_color = colors[index];
        println!("{color_cyan}[-] - {chosen_color}{}{color_reset} {color_reset}", i);
        index += 1;
    }
    println!("{color_cyan} SHOULD WE USE SEGMENTATION(S) OR PAGINATION(P)?{color_reset}"); 
    let mut lever:bool = true; 
    let response_1 = get_response();
    if (response_1 == String::from("P")) | (response_1 == String::from("p")) | (response_1 == String::from("paging")) {
        lever = false; 
    } 
    let mut mmu = MMU::new(); 
    let general_env = mmu.start_process_manager(lever,vec![20, 25, 14, 17, 15,10]);
    let work_enviroment = (general_env.0, general_env.1, general_env.3,general_env.2, general_env.4);
    describe_cpu_state(work_enviroment);
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
