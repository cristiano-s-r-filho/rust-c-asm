extern crate asm_cli;
use std::error::Error;
use clap::Parser;
use asm_cli::utils::{
    tui, 
};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Program file to load
    #[arg(short, long)]
    program: Option<String>,
    
    /// Memory size in bytes
    #[arg(short, long, default_value_t = 65536)]
    memory: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let _cli = Cli::parse();
      
    // Run TUI
    let result = tui::run();
    
    // Restore terminal
    if let Err(e) = result {
        eprintln!("Error running TUI: {}", e);
    }
    
    Ok(())
}