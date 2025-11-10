# A.R.C.S. Emulator Documentation

## 1. Overview

This document provides an overview of the A.R.C.S. Emulator, a terminal-based application for writing, running, and debugging ARC assembly language programs. The emulator is built in Rust and uses `ratatui` for its terminal user interface, with `crossterm` as a backend layer for cross terminal compatibility.

## 2. How to Run

### Prerequisites

*   Rust and Cargo must be installed.

### Building and Running

1.  Navigate to the `asm_cli` directory:

    ```bash
    cd asm_cli
    ```

2.  Install the `arcs` command:

    ```bash
    cargo install --path .
    ```
    This command compiles the project and installs the `arcs` executable to your Cargo bin directory (usually `~/.cargo/bin` or `%USERPROFILE%\.cargo/bin`). Ensure this directory is in your system's PATH.

3.  Run the A.R.C.S emulator:

    ```bash
    arcs
    ```

### Command-Line Arguments

The `arcs` application accepts the following command-line arguments:

*   **`program` (optional):** A path to a program file to load into the emulator at startup.

    ```bash
    arcs /path/to/your/program.arc
    ```

*   **`--memory-size <SIZE>` (optional):** The total size of the emulator's working memory.
    *   **Supported Units:** KB, MB (case-insensitive, e.g., `64KB`, `2MB`).
    *   **Range:** Minimum 64KB, Maximum 8MB.
    *   **Default:** If not specified, the memory size defaults to 64KB.

    ```bash
    arcs --memory-size 1MB
    ```

### Assembler Directives

The ARC assembler supports the following directives for configuring memory segments:

*   **`.text_start <address>`:** Sets the starting address for the code/text segment.
    *   **Example:** `.text_start 0x1000`
*   **`.stack_start <address>`:** Sets the starting address for the stack segment.
    *   **Example:** `.stack_start 0xF000`
*   **`.stack_size <size_in_bytes>`:** Sets the size of the stack segment in bytes.
    *   **Example:** `.stack_size 0x1000` (for 4KB stack)

**Important Notes on Directives:**
*   These directives can only be specified once per assembly file.
*   The assembler validates that segments do not overlap and fit within the total configured memory size.
*   The data segment automatically starts immediately after the stack segment (growing downwards from the stack start).
*   If `.text_start`, `.stack_start`, or `.stack_size` are not specified, default values will be used (text starts at 0x0000, stack starts at `total_memory_size - default_stack_size`, with a default stack size of 4KB).

## 3. Architecture

The emulator is composed of several key components:

### 3.1. Core Emulator

The core of the emulator simulates a simple computer architecture.

*   **CPU (`cpu.rs`):** The `CPU` struct contains the registers and is responsible for fetching, decoding, and executing instructions. It contains 8 general purpose registers, AX -> HX;
*   **Memory (`main_memory.rs`):** The `WorkMemory` struct represents the main memory of the emulated machine. It provides methods for reading and writing 8, 16, and 32-bit values.
*   **Registers (`registers.rs`):** The `Registers` struct holds the state of the CPU registers. All general-purpose registers are 32-bit wide and are used for both integer and floating-point operations.

### 3.2. Instruction Set (ISA)

The emulator supports a custom instruction set, ARC. The instructions are implemented in the `instructions/` directory and are categorized as follows:

*   **Arithmetic    (`aritmethic.rs`):** `ADDW`, `SUBW`, `MUL`, `INC`, `DEC`, `NEG`. These instructions perform 32-bit floating-point arithmetic.
*   **Bitwise       (`bitwise.rs`):** `NOT`, `AND`, `OR`, `XOR`. These instructions perform bitwise operations on 32-bit unsigned integers.
*   **Data Movement (`moves.rs`):** `MOVI`, `MOVW`, `LODI`, `LODW`, `STRI`, `STRW`, `PUSH`, `POP`, `XCGH`. These instructions move data from and to memory, and switch data between registers and between addresses.
*   **Control Flow  (`compare.rs`):** `CMPW`, `JMP`, `CALL`, `RET`, and conditional jumps (`JE`, `JNE`, `JGT`, `JGE`, `JLT`, `JLE`, `JS`, `JCO`).
*   **System        (`systems.rs`):**  `HALT`. These instructions are system instructions that make it so the code execution cycle is not permanent.
*   **I/O           (`io.rs`):** `IN`, `OUT`. Input and output instructions for interactive programs.

### 3.3. Assembler

The emulator includes a simple two-pass assembler implemented in `command_processor.rs`.

*   **Pass 1:** Builds a symbol table by mapping labels to memory addresses.
*   **Pass 2:** Assembles the instructions into 32-bit machine code.

### 3.4. Terminal User Interface (TUI)

The TUI is built using the `ratatui` library and provides an interactive environment for the emulator.

*   **Modes:** The TUI has several modes:
    *   `StartMenu`: The initial screen with options to start the emulator, load a program, or change settings.
    *   `CommandMode`: Allows the user to type and execute assembly commands one by one.
    *   `Paused`: The state when a program is loaded but not running, or when execution has been paused.
    *   `Running`: The state when the emulator is actively executing a program.
*   **Panels:** The main interface is divided into several panels:
    *   **CPU:** Displays the current state of the CPU registers.
    *   **Code:** Shows a view of the code currently in execution.
    *   **Input:** A text box for entering commands in `CommandMode`.
    *   **Menu:** A menu of actions available in the current mode.
    *   **Status Bar:** Displays the current mode and other information.
*   **Program Editor:** A built-in text editor for writing and editing programs. It supports basic text manipulation and can save programs to an paste in the project,  `/programs` , allowing for programs to be reused.

## 4. Current State and Future Work

### Recent Improvements

As of the latest update, several key improvements have been made, enhancing the emulator's stability, functionality, and configurability:

*   **Configurable Memory Size:** The emulator now supports configurable total memory size via a CLI option (`--memory-size`), allowing users to specify memory from 64KB to 8MB.
*   **Assembler Segment Directives:** New assembler directives (`.text_start`, `.stack_start`, `.stack_size`) enable precise control over memory segment placement and size within assembly programs.
*   **Debug Message Elimination:** Unwanted debug messages during the assembly process have been identified and removed, providing a cleaner user experience.
*   **Program Loading:** An issue where assembly programs provided via command-line arguments were not being correctly loaded has been fixed. The parser now correctly handles comments and empty lines, allowing for successful loading and assembly of external program files.
*   **HALT Instruction:** A bug where the `HALT` instruction did not properly stop program execution has been addressed. The CPU now includes a `halted` flag that is set by the `HALT` instruction, ensuring that the emulator gracefully terminates program execution instead of attempting to run from uninitialized memory.

### Recently Implemented

*   **Program Loading from File:** The emulator can now load a program from a file specified as a command-line argument.
*   **Editor Compilation Fixes:** Resolved compilation errors in `editor.rs` by removing incompatible `tui-textarea` syntax highlighting methods.
*   **Basic Editor Styling:** Implemented basic text styling in the editor using `set_style` for improved readability.
*   **New File and Save As Functionality:** Re-implemented 'New File' (Ctrl+N) and 'Save As' (Ctrl+A) features in the editor, including input handling and dialog rendering.

### Next Steps

The following features are planned for future development:

*   **Enhanced Syntax Highlighting:** Implement a more robust and accurate syntax highlighting solution for the editor.
*   **VGA and other graphical instructions**. The implementation of graphical instructions for a new screen for graphical output is in order.
*   **Better Macro definitions**. Evolving from our current macro definitions into more modern approaches is also in order.
*   **Layered Execution and Multicore Support**. Allowing in the future, for users to deploy multiple emulated cores, and get feed back from it, is also in order, to allow for good layered execution.
