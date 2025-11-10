use std::fs::File;

const LOG_FILE_PATH: &str = "asm_cli/debug.log";

pub fn log_message(_message: &str) {
    // Logging is disabled.
}

pub fn clear_log_file() {
    if let Err(e) = File::create(LOG_FILE_PATH) {
        eprintln!("Failed to clear log file: {}", e);
    }
}

