pub mod memory; 
pub mod instructions; 
pub mod chips; 
use chips::mmu::MMU;
use colored::Colorize;
use inline_colorization::*;
// Memory description at initialization: 
use memory::main_memory::WorkMemory;
use memory::registers::MainRegisters;
use memory::registers::OffsetRegisters;
use memory::registers::SegmentRegisters;
use memory::registers::EFLAG;
use memory::*; 
pub fn describe_cpu_state(work_env:(WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu: MMU) {
    println!("BELLOW HERE GOES A SNAPSHOT THE STATE OF THE CPU:"); 
    let initial_status_code:(u16,u16,u16,u16) = (work_env.3.cs, work_env.3.ss,work_env.3.ds, work_env.3.es);
    // Process work memory blocks that have been statically allocaded.
    let init_code_block: (&'static str,bool,u32,u32) = slice_segment_data(&"CODE", mmu.code_summary.2 as usize,mmu.code_summary.3 , &work_env.0);
    let init_stack_block:(&'static str,bool,u32,u32) = slice_segment_data(&"STCK", mmu.stack_summary.2 as usize, mmu.stack_summary.3, &work_env.0);
    let init_data_block: (&'static str,bool,u32,u32) = slice_segment_data(&"DATA", mmu.data_summary.2 as usize, mmu.data_summary.3,&work_env.0);
    let init_extra_block: (&'static str,bool,u32,u32) = slice_segment_data(&"EXTR", mmu.extra_sumary.2 as usize, mmu.extra_sumary.3,&work_env.0); 
    // GENERAL CONFIGS ON INITIALIZATION;
    let memory_stats: WorkMemory = work_env.0; 
    let workbench: MainRegisters = work_env.1; 
    let work_offsets: OffsetRegisters = work_env.2; 
    let flag_state: EFLAG = work_env.4; 
    println!("{}","MEMORY SUMMARY: ".cyan().bold());
    let pt = init_code_block.2 as usize;
    for item in &memory_stats.cells[pt..pt+10] {
        println!("{color_cyan}[+] - {color_reset} {color_white}{:#x}{color_reset}", item);
    }
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
    println!("{color_cyan} STACK-POINTER-STATE:{color_reset} {color_magenta}{:#x}{color_reset}", work_offsets.esi);
    println!("{}","FLAGS: ".cyan().bold());
    println!("{color_cyan} OVERFLOW? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.ovfw);
    println!("{color_cyan} ZERO? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.zero);
    println!("{color_cyan} NEGATIVE? - {color_reset} {color_bright_white}{}{color_reset}", flag_state.negv);
    println!("{style_bold}{color_bright_cyan}ANALYSES COMPLETED! CODE -- {color_reset}{style_reset}{color_white}{:#x}:{:#x}:{:#x}:{:#x}{color_reset}", initial_status_code.0, initial_status_code.1, initial_status_code.2, initial_status_code.3);
}     

pub fn describe_working_states(work_env:&mut (WorkMemory,MainRegisters,OffsetRegisters,SegmentRegisters,EFLAG), mmu: &mut MMU, data_or_adress: bool, get_or_send: bool) -> bool {
    let mmu_acess = mmu; 
    let data_bus = mmu_acess.get_from_data_bus(); 
    let adress_bus = mmu_acess.get_from_adress_bus(); 
    let eax =  work_env.1.eax;
    let ebx = work_env.1.ebx; 
    let ecx = work_env.1.ecx; 
    let edx = work_env.1.edx; 
    let eip = work_env.2.eip; 
    let esp = work_env.2.esp;
    let ebp = work_env.2.ebp;  
    let cs = work_env.3.cs;
    let ss = work_env.3.ss; 
    let ds = work_env.3.ds; 
    let es = work_env.3.es; 
    let flag = if work_env.4.ovfw == true {"YES"} else {"NO"}; 
    //Conditional Prints. 
    println!("{color_cyan} <-----------------------> {color_reset}");
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
    if work_env.4.ovfw == false {
        println!("{color_cyan}GPF?:{color_bright_magenta}{}{color_reset}", flag);
        println!("{color_cyan}--PROCEED AS USUAL--{color_reset}"); 
        let gpf = false; 
        return gpf; 
    } else {
        println!("{color_cyan}GPF?:{color_bright_magenta}{}{color_reset}", flag);
        println!("{color_red}-- THERE HAS BEEN A GPF!!! TERMINATING PROCESS IMEDIATELY");
        let gpf = true; 
        return gpf;  
    }  
}



