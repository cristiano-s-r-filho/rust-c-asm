use arc_emulator::utils::tui::TuiApp;
use arc_emulator::utils::workspaces::Workspace;
use arc_emulator::utils::config::config_manager::ConfigManager;
use arc_emulator::memory::main_memory::DEFAULT_MEMORY_SIZE;
use std::env;
use clap::Parser;

/// ARC CPU Emulator and Assembler CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set the total memory size for the emulator (e.g., 64KB, 1MB, 8MB).
    /// Minimum: 64KB, Maximum: 8MB.
    #[arg(long, short, value_parser = parse_memory_size, help = "Set the total memory size (e.g., 64KB, 1MB, 8MB). Min: 64KB, Max: 8MB)")]
    memory_size: Option<usize>,
}

fn parse_memory_size(s: &str) -> Result<usize, String> {
    let s_upper = s.to_uppercase();
    let (value_str, unit) = if s_upper.ends_with("KB") {
        (&s_upper[0..s_upper.len() - 2], "KB")
    } else if s_upper.ends_with("MB") {
        (&s_upper[0..s_upper.len() - 2], "MB")
    } else {
        return Err(format!("Invalid memory size format: {}. Use KB or MB suffix (e.g., 64KB, 1MB).", s));
    };

    let value = value_str.parse::<f64>()
        .map_err(|_| format!("Invalid number in memory size: {}", value_str))?;

    let bytes = match unit {
        "KB" => (value * 1024.0) as usize,
        "MB" => (value * 1024.0 * 1024.0) as usize,
        _ => unreachable!(), // Should be caught by earlier checks
    };

    const MIN_MEMORY_SIZE: usize = 64 * 1024; // 64KB
    const MAX_MEMORY_SIZE: usize = 8 * 1024 * 1024; // 8MB

    if bytes < MIN_MEMORY_SIZE || bytes > MAX_MEMORY_SIZE {
        return Err(format!(
            "Memory size {} is out of range. Minimum: {}KB, Maximum: {}MB.",
            s, MIN_MEMORY_SIZE / 1024, MAX_MEMORY_SIZE / (1024 * 1024)
        ));
    }

    Ok(bytes)
}

fn main() {
    let cli = Cli::parse();
    let memory_size = cli.memory_size.unwrap_or(DEFAULT_MEMORY_SIZE);

    let workspace = Workspace::new(env::current_dir().expect("Failed to get current directory"), memory_size);
    let config_manager = ConfigManager::new().expect("Failed to create ConfigManager");
    let mut app = TuiApp::new(workspace, config_manager, memory_size);

    if let Err(e) = app.run() {
        eprintln!("Error running TUI: {}", e);
    }
}