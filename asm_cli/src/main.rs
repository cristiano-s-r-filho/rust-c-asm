extern crate asm_cli;
use asm_cli::*;
use chips::crom::AcessLevel;
use colored::Colorize;
use inline_colorization::*;
fn main() {
    describe_cpu_registers();
    let obsevations = comence_observations();
    let global_table = obsevations.1; 
    println!("{}", "--------------------------- GLOBAL DESCRIPTOR TABLE ----------------------------".cyan().bold());
    for item in &global_table.content[0..20]{
        let selects = item.selector;
        let base  = item.base;
        let limit = item.limit;
        let level:AcessLevel = item.acess_level; 
        println!("SELECTOR: {color_red}{}{color_reset} | BASE-ADRR: {color_blue}{}{color_reset} | LIMIT-ADRR: {color_yellow}{}{color_reset} | ACESS-LEVEL: {color_green}{:?}{color_reset}", selects,base,limit,level);
    }
    println!("{}", "-------------------------------- END OF TABLE ----------------------------------".cyan().bold());
}    
