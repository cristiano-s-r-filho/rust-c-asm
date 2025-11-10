//! # Workspaces Module
//!
//! This module defines the `Workspace` struct, which manages the application's
//! current working directory, open files, active file, and interacts with
//! the emulator and assembler states. It provides functionalities for file
//! operations, program assembly, and state management.

use std::path::{PathBuf, Path};
use std::fs;
use crate::utils::assembler::command_processor::{AssembledProgram, parse_command, Command, Macro, assemble_program};
use walkdir::WalkDir;
use crate::utils::apps::emulator::EmulatorState as AppEmulatorState;
use crate::utils::ui::resources::AppStatus;

/// Represents the state of the assembler within the workspace.
#[derive(Debug, Clone, Default)]
pub struct AssemblerState {
    /// A list of defined macros.
    pub macros: Vec<Macro>,
    /// The result of the last assembly operation.
    pub last_assembly_result: Option<Result<AssembledProgram, String>>,
}

use crate::chips::io_device::IoDevice;

/// Manages the current workspace, including files, emulator, and assembler states.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// The current directory of the workspace.
    pub current_path: PathBuf,
    /// A list of files currently open in the workspace.
    pub open_files: Vec<PathBuf>,
    /// The currently active file being edited or viewed.
    pub active_file: Option<PathBuf>,
    /// The state of the emulator associated with this workspace.
    pub emulator: Option<AppEmulatorState>,
    /// The state of the assembler associated with this workspace.
    pub assembler: AssemblerState,
    /// The I/O device for the emulator.
    pub io_device: IoDevice,
    /// A flag indicating if there are unsaved changes in the active file.
    pub unsaved_changes: bool,
}

impl Workspace {
    /// Creates a new `Workspace` instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The initial path for the workspace.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `Workspace` instance.
    pub fn new<P: AsRef<Path>>(path: P, memory_size: usize) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        std::fs::create_dir_all(&path_buf).ok();
        
        Self {
            current_path: path_buf,
            open_files: Vec::new(),
            active_file: None,
            emulator: Some(AppEmulatorState::new(memory_size)),
            assembler: AssemblerState::default(),
            io_device: IoDevice::new(),
            unsaved_changes: false,
        }
    }
    
    /// Returns a mutable reference to the emulator state.
    ///
    /// # Panics
    ///
    /// Panics if the emulator state is `None`.
    pub fn get_emulator(&mut self) -> &mut AppEmulatorState {
        self.emulator.as_mut().unwrap()
    }
    
    /// Saves the given content to a specified file path within the workspace.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path where the content should be saved.
    /// * `content` - The string content to write to the file.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful save, or an error message on failure.
    pub fn save_file<P: AsRef<Path>>(&mut self, file_path: P, content: &str) -> Result<(), String> {
        let path = file_path.as_ref();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        self.active_file = Some(path.to_path_buf());
        if !self.open_files.contains(&path.to_path_buf()) {
            self.open_files.push(path.to_path_buf());
        }
        self.unsaved_changes = false;
        
        Ok(())
    }
    
    /// Saves the content of the currently active file.
    ///
    /// # Arguments
    ///
    /// * `content` - The string content to write to the active file.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful save, or an error message if
    ///   no file is active or saving fails.
    pub fn save_current_file(&mut self, content: &str) -> Result<(), String> {
        if let Some(ref path) = self.active_file.clone() {
            self.save_file(path, content)
        } else {
            Err("No active file to save".to_string())
        }
    }
    
    /// Saves the given content to a new file path, updating the active file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The new path where the content should be saved.
    /// * `content` - The string content to write to the new file.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful save, or an error message on failure.
    pub fn save_file_as<P: AsRef<Path>>(&mut self, file_path: P, content: &str) -> Result<(), String> {
        self.save_file(file_path, content)
    }
    
    /// Opens a specified file for editing, setting it as the active file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path of the file to open.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful opening, or an error message on failure.
    pub fn open_file_for_editing<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), String> {
        self.active_file = Some(file_path.as_ref().to_path_buf());
        if !self.open_files.contains(&file_path.as_ref().to_path_buf()) {
            self.open_files.push(file_path.as_ref().to_path_buf());
        }
        self.unsaved_changes = false;
        Ok(())
    }

    /// Creates a new `.arc` assembly file with default content.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path for the new `.arc` file.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful creation, or an error message on failure.
    pub fn create_new_arc_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), String> {
        let default_content = r#"; New ARC Program
.text
    ; Your code goes here
    MOVI AX, 0x0042    ; Example: Load 42 into AX
    HALT               ; Stop execution

.data
    ; Data section (optional)
    .word 0x0000
"#.to_string();
        
        self.save_file(file_path.as_ref(), &default_content)?;
        self.open_file_for_editing(file_path)?;
        self.unsaved_changes = true;
        
        Ok(())
    }
    
    /// Assembles the provided source code and loads the resulting program into the emulator.
    ///
    /// # Arguments
    ///
    /// * `source` - The assembly source code as a string.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on successful assembly and loading, or an error message on failure.
    pub fn assemble_and_load_program(&mut self, source: &str, app_status: &mut AppStatus) -> Result<(), String> {
        app_status.is_loading = true;
        let commands = self.parse_source_to_commands(source)?;
        let total_memory_size = self.emulator.as_ref().unwrap().memory.size;
        let assembled_program = assemble_program(&commands, &self.assembler.macros, total_memory_size)?;
        
        let emulator = self.emulator.as_mut().unwrap();
        emulator.load_assembled_program(&assembled_program)?;
        emulator.program_source = Some(source.to_string());
        emulator.assembled_program = Some(assembled_program.clone());
        emulator.last_assembly_errors.clear();
        
        self.assembler.last_assembly_result = Some(Ok(assembled_program));
        app_status.is_loading = false;
        
        Ok(())
    }
    
    /// Attempts to assemble the provided source code and returns the assembled program
    /// or a list of errors, without loading it into the emulator.
    ///
    /// # Arguments
    ///
    /// * `source` - The assembly source code as a string.
    ///
    /// # Returns
    ///
    /// * `Result<AssembledProgram, Vec<String>>` - `Ok(AssembledProgram)` on successful assembly,
    ///   or a `Vec<String>` containing error messages on failure.
    pub fn try_assemble_program(&mut self, source: &str, app_status: &mut AppStatus) -> Result<AssembledProgram, Vec<String>> {
        app_status.is_loading = true;
        let result = match self.parse_source_to_commands(source) {
            Ok(commands) => {
                let total_memory_size = self.emulator.as_ref().unwrap().memory.size;
                match assemble_program(&commands, &self.assembler.macros, total_memory_size) {
                    Ok(program) => {
                        let emulator = self.get_emulator();
                        emulator.last_assembly_errors.clear();
                        Ok(program)
                    }
                    Err(e) => {
                        let emulator = self.get_emulator();
                        emulator.last_assembly_errors = vec![e.clone()];
                        Err(vec![e])
                    }
                }
            }
            Err(e) => {
                let emulator = self.get_emulator();
                emulator.last_assembly_errors = vec![e.clone()];
                Err(vec![e])
            }
        };
        app_status.is_loading = false;
        result
    }
    
    /// Parses the given assembly source code into a vector of `Command`s.
    ///
    /// This function handles macro definitions and expansions during the parsing process.
    ///
    /// # Arguments
    ///
    /// * `source` - The assembly source code as a string.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Command>, String>` - A vector of parsed `Command`s on success,
    ///   or an error message if parsing fails (e.g., syntax error, unclosed macro).
    fn parse_source_to_commands(&self, source: &str) -> Result<Vec<Command>, String> {
        let mut commands = Vec::new();
        let mut current_macro: Option<Macro> = None;
        
        for (line_num, line) in source.lines().enumerate() {
            let command = parse_command(line)
                .map_err(|e| format!("Line {}: {}", line_num + 1, e))?;
            
            if command.opcode == ".macro" {
                if current_macro.is_some() {
                    return Err(format!("Line {}: Nested macro definition not allowed", line_num + 1));
                }
                current_macro = Some(Macro {
                    name: command.macro_name.clone().unwrap_or_default(),
                    args: command.macro_args.clone().unwrap_or_default(),
                    body: Vec::new(),
                });
                continue;
            }
            
            if let Some(ref mut macro_def) = current_macro {
                if command.opcode == ".endmacro" {
                    current_macro = None;
                } else {
                    macro_def.body.push(command);
                }
            } else {
                commands.push(command);
            }
        }
        
        if current_macro.is_some() {
            return Err("Unclosed macro definition".to_string());
        }
        
        Ok(commands)
    }
    
    /// Checks if the workspace currently has unsaved changes.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if there are unsaved changes, `false` otherwise.
    pub fn has_unsaved_changes(&self) -> bool {
        self.unsaved_changes
    }
    
    /// Returns the path of the currently selected program as an `Option<String>`.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The path of the active file if set, otherwise `None`.
    pub fn get_selected_program_path(&self) -> Option<String> {
        self.active_file.as_ref().map(|p| p.to_string_lossy().into_owned())
    }

    /// Marks the workspace as having unsaved changes.
    pub fn mark_unsaved(&mut self) {
        self.unsaved_changes = true;
    }
    
    /// Lists all `.arc` and `.asm` files within the current workspace directory and its subdirectories.
    ///
    /// The returned list of paths is sorted alphabetically.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<PathBuf>, String>` - A sorted vector of `PathBuf`s on success,
    ///   or an error message if directory traversal fails.
    pub fn list_arc_files(&self) -> Result<Vec<PathBuf>, String> {
        let mut arc_files = Vec::new();
        
        for entry in WalkDir::new(&self.current_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "arc" || ext == "asm" {
                        arc_files.push(path.to_path_buf());
                    }
                }
            }
        }
        
        arc_files.sort();
        Ok(arc_files)
    }
}

impl Default for Workspace {
    /// Provides a default `Workspace` instance, initialized to a "default" path.
    fn default() -> Self {
        Self::new("default", crate::memory::main_memory::DEFAULT_MEMORY_SIZE)
    }
}

