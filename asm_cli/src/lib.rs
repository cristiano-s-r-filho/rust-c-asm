pub mod memory; 
pub mod instructions; 
pub mod chips; 

use chips::crom::generate_gdt;
use chips::crom::generate_idt;
use colored::Colorize;
use inline_colorization::*;
// Memory description at initialization: 
use memory::main_memory::WorkMemory;
use memory::registers::MainRegisters;
use memory::registers::OffsetRegisters;
use memory::registers::EFLAG;
use memory::CODE_HEAD;
pub fn describe_cpu_registers() {
    println!("Functioning as usual");     
    let init_stats: ((u16,u16,u16,u16),((&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32)), WorkMemory, MainRegisters, OffsetRegisters,EFLAG) = memory::initiate_working_env(); 
    let initial_status_code:(u16,u16,u16,u16) = init_stats.0;
    // Process work memory blocks that have been statically allocaded.
    let init_code_block: (&'static str,bool,u32,u32) = init_stats.1.0;
    let init_stack_block:(&'static str,bool,u32,u32) = init_stats.1.1;
    let init_data_block: (&'static str,bool,u32,u32) = init_stats.1.2;
    let init_extra_block: (&'static str,bool,u32,u32) = init_stats.1.3; 
    // GENERAL CONFIGS ON INITIALIZATION;
    let memory_stats: WorkMemory = init_stats.2; 
    let workbench: MainRegisters = init_stats.3; 
    let work_offsets: OffsetRegisters = init_stats.4; 
    let flag_state: EFLAG = init_stats.5; 
    println!("{}","MEMORY SUMMARY: ".cyan().bold());
    for item in &memory_stats.cells[CODE_HEAD..CODE_HEAD+10] {
        println!("{color_cyan}[+] - {color_reset} {color_white}{}{color_reset}", item);
    }
    println!("{}","WHAT GOES BELLOW IS THE STATE OF THE REGISTERS AFTER INITIALIZATION: ".cyan().bold());
    println!("{}","MAIN REGISTERS: ".cyan().bold()); 
    println!(" {color_cyan}EAX:{color_reset} {color_red}{}{color_reset}", workbench.eax);
    println!(" {color_cyan}EBX:{color_reset} {color_blue}{}{color_reset}",workbench.ebx);
    println!(" {color_cyan}ECX:{color_reset} {color_yellow}{}{color_reset}",workbench.ecx);
    println!(" {color_cyan}EDX:{color_reset} {color_green}{}{color_reset}",workbench.edx);
    let code_status= if init_code_block.1 == false {"UP"} else {"DOWN"};
    let stack_status = if init_stack_block.1 == false {"UP"} else {"DOWN"};
    let data_status = if init_data_block.1 == false {"UP"} else{"DOWN"};
    let extra_status = if init_extra_block.1 == false {"UP"} else{"DOWN"}; 
    println!("{}","SEGMENTS SELECTED: ".cyan().bold()); 
    println!(" NAME: {color_red}{}{color_reset} | STATUS: {color_blue}{}{color_reset} | PRIM: {color_yellow}{:#x}{color_reset} | BLOCK_END: {color_green}{:#x}{color_reset}",init_code_block.0, code_status, init_code_block.2, init_code_block.3);
    println!(" NAME: {color_red}{}{color_reset} | STATUS: {color_blue}{}{color_reset} | PRIM: {color_yellow}{:#x}{color_reset} | BLOCK_END: {color_green}{:#x}{color_reset}",init_stack_block.0, stack_status, init_stack_block.2, init_stack_block.3);
    println!(" NAME: {color_red}{}{color_reset} | STATUS: {color_blue}{}{color_reset} | PRIM: {color_yellow}{:#x}{color_reset} | BLOCK_END: {color_green}{:#x}{color_reset}",init_data_block.0, data_status, init_data_block.2, init_data_block.3);
    println!(" NAME: {color_red}{}{color_reset} | STATUS: {color_blue}{}{color_reset} | PRIM: {color_yellow}{:#x}{color_reset} | BLOCK_END: {color_green}{:#x}{color_reset}",init_extra_block.0, extra_status, init_extra_block.2, init_extra_block.3);
    println!("{color_cyan} PROGRAM-COUNTER-STATE:{color_reset} {color_magenta}{:#x}{color_reset}", work_offsets.eip);
    println!("{}","FLAGS: ".cyan().bold());
    println!("{color_cyan} OVERFLOW? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.ovfw);
    println!("{color_cyan} ZERO? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.zero);
    println!("{color_cyan} NEGATIVE? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.negv);
    println!("{style_bold}{color_bright_cyan}INITIALIZATION COMPLETED! CODE -- {color_reset}{style_reset}{color_white}{:#x}:{:#x}:{:#x}:{:#x}{color_reset}", initial_status_code.0, initial_status_code.1, initial_status_code.2, initial_status_code.3);
}     

use chips::crom::CRObserver; 
use chips::crom::CRegisters;
use chips::crom::DTable;

pub fn comence_observations() -> ((CRegisters,(CRObserver,CRObserver,CRObserver)),DTable,DTable){
    let start = chips::crom::CRegisters::cregisters_quick_start(); 
   let gdt =  generate_gdt();
   let idt=  generate_idt();
    return (start,gdt,idt); 

}
