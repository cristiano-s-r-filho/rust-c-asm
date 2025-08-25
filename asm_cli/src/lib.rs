pub mod memory; 
pub mod instructions; 
pub mod chips; 
// use chips::mmu::MMU;
use colored::Colorize;
use inline_colorization::*;
// Memory description at initialization: 
// use memory::main_memory::WorkMemory;
use memory::registers::MainRegisters;
use memory::registers::OffsetRegisters;
use memory::registers::SegmentRegisters;
use memory::registers::EFLAG;

use crate::chips::crom::CPU; 
pub fn describe_cpu_state(cpu: &CPU) {
    println!("BELLOW HERE GOES A SNAPSHOT THE STATE OF THE CPU:"); 
    let initial_status_code:(u32,u32,u32,u32) = (cpu.segment_reg.cs, cpu.segment_reg.ss,cpu.segment_reg.ds, cpu.segment_reg.es);
    // Process work memory blocks that have been statically allocaded.
    let init_code_block: (&'static str,bool,u32,u32) = ("CODE", false, cpu.segment_reg.cs, 0xFFFF);
    let init_stack_block:(&'static str,bool,u32,u32) = ("STCK", false, cpu.segment_reg.cs, 0xFFFF);
    let init_data_block: (&'static str,bool,u32,u32) = ("DATA", false, cpu.segment_reg.cs, 0xFFFF);
    let init_extra_block: (&'static str,bool,u32,u32) = ("EXTR", false, cpu.segment_reg.cs, 0xFFFF);
    // GENERAL CONFIGS ON INITIALIZATION;
    let workbench: &MainRegisters = &cpu.main_reg; 
    let work_offsets: &OffsetRegisters = &cpu.offsets; 
    let flag_state: EFLAG = cpu.flag; 
    println!("{}","WHAT GOES BELLOW IS THE STATE OF THE MAIN REGISTERS:".cyan().bold());
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
    println!("{color_cyan} STACK-POINTER-STATE:{color_reset} {color_magenta}{:#x}{color_reset}", work_offsets.esp);
    println!("{}","FLAGS: ".cyan().bold());
    println!("{color_cyan} OVERFLOW? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.ovfw);
    println!("{color_cyan} ZERO? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.zero);
    println!("{color_cyan} NEGATIVE? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.negv);
    println!("{style_bold}{color_bright_cyan}ANALYSES COMPLETED! CODE -- {color_reset}{style_reset}{color_white}{:#x}:{:#x}:{:#x}:{:#x}{color_reset}", initial_status_code.0, initial_status_code.1, initial_status_code.2, initial_status_code.3);
}     

pub fn describe_working_states(cpu: &CPU, data_or_adress: bool, get_or_send: bool) -> bool {
    let mut mmu_acess = cpu.crom.mmu; 
    let data_bus = mmu_acess.get_from_data_bus(); 
    let adress_bus = mmu_acess.get_from_adress_bus();

    let eax =  cpu.main_reg.eax;
    let ebx = cpu.main_reg.ebx; 
    let ecx = cpu.main_reg.ecx; 
    let edx = cpu.main_reg.edx;

    let eip = cpu.offsets.eip; 
    let esp = cpu.offsets.esp;
    let ebp = cpu.offsets.ebp; 

    let cs = cpu.segment_reg.cs;
    let ss = cpu.segment_reg.ss; 
    let ds = cpu.segment_reg.ds; 
    let es = cpu.segment_reg.es; 
    
    let flag = cpu.flag.ovfw; 
    //Conditional Prints. 
    println!("{color_cyan}>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {color_reset}");
    if data_or_adress == true && get_or_send == true {
        println!("{color_cyan} -- GOT {color_white}{}{color_reset} FROM DATA BUS -- {color_reset}", data_bus); 
    } else if data_or_adress == false && get_or_send == true {
        println!("{color_cyan} -- GOT {color_white}{:#x}{color_reset} FROM ADRESS BUS -- {color_reset}", adress_bus);
    } else if data_or_adress == true && get_or_send == false {
        println!("{color_cyan} -- SENT {color_white}{}{color_reset} TO DATA BUS -- {color_reset}", data_bus);
    } else if data_or_adress == false && get_or_send == false {
        println!("{color_cyan} -- SENT {color_white}{:#x}{color_reset} TO ADRESS BUS -- {color_reset}", adress_bus);
    }
    println!("{color_cyan}EAX: {color_bright_red}{}{color_reset} | EBX: {color_bright_blue}{}{color_reset} | ECX: {color_bright_yellow}{}{color_reset} | EDX: {color_bright_green}{}{color_reset}  {color_reset}", eax,ebx,ecx,edx); 
    println!("{color_cyan}CS: {color_bright_red}{:#x}{color_reset} | SS: {color_bright_blue}{:#x}{color_reset} | DS: {color_bright_yellow}{:#x}{color_reset} | ES: {color_bright_green}{:#x}{color_reset}  {color_reset}", cs,ss,ds,es);
    println!("{color_cyan}PROGRAM-COUNTER: {color_magenta}{:#x}{color_reset}| STACK-POINTER: {color_white}{:#x}{color_reset} | BASE-POINTER:{color_white}{:#x}{color_reset}{color_reset}", eip, esp, ebp); 
    if flag == false {
        println!("{color_cyan}GPF?:{color_bright_magenta} NO{color_reset}");
        println!("{color_cyan}--PROCEED AS USUAL--{color_reset}"); 
        let gpf = false; 
        return gpf; 
    } else {
        println!("{color_cyan}GPF?:{color_bright_magenta} YES{color_reset}");
        println!("{color_red}-- THERE HAS BEEN A GPF!!! TERMINATING PROCESS IMEDIATELY");
        let gpf = true; 
        return gpf;  
    }  
}



