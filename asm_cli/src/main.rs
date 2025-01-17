extern crate asm_cli;
use asm_cli::memory::main_memory::WorkMemory;
use asm_cli::memory::registers::MainRegisters;
use asm_cli::memory::registers::OffsetRegisters;
use asm_cli::memory::registers::EFLAG;
use asm_cli::memory::CODE_HEAD;
fn main() {
    println!("Functioning as usual");     
    let init_stats: ((u16,u16,u16,u16),((&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32),(&'static str,bool,u32,u32)), WorkMemory, MainRegisters, OffsetRegisters,EFLAG) = asm_cli::memory::initiate_working_env(); 
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
    println!("MEMORY SUMMARY: ");
    for item in &memory_stats.cells[CODE_HEAD..CODE_HEAD+10] {
        println!("[+] - {}", item);
    }
    println!("WHAT GOES BELLOW IS THE STATE OF THE REGISTERS AFTER INITIALIZATION: ");
    println!("MAIN REGISTERS: "); 
    println!("EAX: {}",workbench.eax);
    println!("EBX: {}",workbench.ebx);
    println!("ECX: {}",workbench.ecx);
    println!("EDX: {}",workbench.edx);
    let code_status= if init_code_block.1 == false {"up"} else {"down"};
    let stack_status = if init_stack_block.1 == false {"up"} else {"down"};
    let data_status = if init_data_block.1 == false {"up"} else{"down"};
    let extra_status = if init_extra_block.1 == false {"up"} else{"down"}; 
    println!("SEGMENTS SELECTED: "); 
    println!("NAME: {} | STATUS: {} | PRIM: {:#x} | BLOCK_END: {:#x}",init_code_block.0, code_status, init_code_block.2, init_code_block.3);
    println!("NAME: {} | STATUS: {} | PRIM: {:#x} | BLOCK_END: {:#x}",init_stack_block.0, stack_status, init_stack_block.2, init_stack_block.3);
    println!("NAME: {} | STATUS: {} | PRIM: {:#x} | BLOCK_END: {:#x}",init_data_block.0, data_status, init_data_block.2, init_data_block.3);
    println!("NAME: {} | STATUS: {} | PRIM: {:#x} | BLOCK_END: {:#x}",init_extra_block.0, extra_status, init_extra_block.2, init_extra_block.3);
    println!("PROGRAM-COUNTER-STATE: {:#x}", work_offsets.eip);
    println!("FLAGS: ");
    println!("OVERFLOW? - {}", flag_state.ovfw);
    println!("ZERO? - {}",flag_state.zero);
    println!("NEGATIVE? - {}",flag_state.negv);
    println!("INITIALIZATION COMPLETED! CODE -- {:#x}:{:#x}:{:#x}:{:#x}", initial_status_code.0, initial_status_code.1, initial_status_code.2, initial_status_code.3); 
}
