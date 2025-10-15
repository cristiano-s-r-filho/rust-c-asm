extern crate asm_cli;
use std::error::Error;
use std::fs;
use clap::Parser;
use asm_cli::utils::{
    tui, 
};
#[derive(Parser, Debug)]
#[command(version, about = "A.R.C.S (Assembly Relay Command Scripts) Emulator", long_about = None)]
struct Cli {
    /// Program file to load
    program: Option<String>,
    
    /// Memory size in bytes
    #[arg(short, long, default_value_t = 65536)]
    memory: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let cli = Cli::parse();
      
    // Read program file if provided
    let program_content = if let Some(program_path) = cli.program {
        Some(fs::read_to_string(program_path)?)
    } else {
        None
    };

    // Run TUI
    let result = tui::run(cli.memory, program_content);
    
    // Restore terminal
    if let Err(e) = result {
        eprintln!("Error running TUI: {}", e);
    }
    
    Ok(())
}