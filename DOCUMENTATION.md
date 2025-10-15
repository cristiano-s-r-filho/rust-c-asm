# A.R.C.S. Emulator Documentation

## 1. Overview

This document provides an overview of the ARC-0 Assembly Emulator, a terminal-based application for writing, running, and debugging ARC-0 assembly language programs. The emulator is built in Rust and uses `ratatui` for its terminal user interface, with `crossterm` as a backend layer for cross terminal compatibility.

## 2. How to Run

### Prerequisites

- Rust and Cargo must be installed.

### Building and Running

1. Navigate to the `asm_cli` directory:

```bash
    cd asm_cli
```

1. Build and run the application (without any options):

```bash
    cargo run
```

### Command-Line Arguments

The application accepts the following command-line arguments:

-> **`program` (optional):** A path to a program file to load into the emulator at startup.

```bash
    cargo run -- /path/to/your/program.arc
```

-> **`--memory <SIZE>` (optional):** The size of the emulator's working memory in bytes. Defaults to 65536 bytes (64 KiB).

```bash
    cargo run -- --memory 131072
```

## 3. Architecture

The emulator is composed of several key components:

### 3.1. Core Emulator

The core of the emulator simulates a simple computer architecture.

-> **CPU (`cpu.rs`):** The `CPU` struct contains the registers and is responsible for fetching, decoding, and executing instructions. It contains 8 general purpose registers, AX -> HX;
-> **Memory (`main_memory.rs`):** The `WorkMemory` struct represents the main memory of the emulated machine. It provides methods for reading and writing 8, 16, and 32-bit values.
-> **Registers (`registers.rs`):** The `Registers` struct holds the state of the CPU registers. All general-purpose registers are 32-bit wide and are used for both integer and floating-point operations.

### 3.2. Instruction Set (ISA)

The emulator supports a custom instruction set, ARC-0. The instructions are implemented in the `instructions/` directory and are categorized as follows:

-> **Arithmetic    (`aritmethic.rs`):** `ADDW`, `SUBW`, `MUL`, `INC`, `DEC`, `NEG`. These instructions perform 32-bit floating-point arithmetic.
-> **Bitwise       (`bitwise.rs`):** `NOT`, `AND`, `OR`, `XOR`. These instructions perform bitwise operations on 32-bit unsigned integers.
-> **Data Movement (`moves.rs`):** `MOVI`, `MOVW`, `LODI`, `LODW`, `STRI`, `STRW`, `PUSH`, `POP`, `XCGH`. These instructions move data from and to memory, and switch data between registers and between addresses.
-> **Control Flow  (`compare.rs`):** `CMPW`, `JMP`, `CALL`, `RET`, and conditional jumps (`JE`, `JNE`, `JGT`, `JGE`, `JLT`, `JLE`, `JS`, `JCO`).
-> **System        (`systems.rs`):**  `HALT`. These instructions are system instructions that make it so the code execution cycle is not permanent.
-> **I/O           (`io.rs`):** `IN`, `OUT`. Input and output instructions for interactive programs.

### 3.3. Assembler

The emulator includes a simple two-pass assembler implemented in `command_processor.rs`.

-> **Pass 1:** Builds a symbol table by mapping labels to memory addresses.
-> **Pass 2:** Assembles the instructions into 32-bit machine code.

### 3.4. Terminal User Interface (TUI)

The TUI is built using the `ratatui` library and provides an interactive environment for the emulator.

-> **Modes:** The TUI has several modes:
---> `StartMenu`: The initial screen with options to start the emulator, load a program, or change settings.
---> `CommandMode`: Allows the user to type and execute assembly commands one by one.
---> `Paused`: The state when a program is loaded but not running, or when execution has been paused.
---> `Running`: The state when the emulator is actively executing a program.
-> **Panels:** The main interface is divided into several panels:
---> **CPU:** Displays the current state of the CPU registers.
---> **Code:** Shows a view of the code currently in execution.
---> **Input:** A text box for entering commands in `CommandMode`.
---> **Menu:** A menu of actions available in the current mode.
---> **Status Bar:** Displays the current mode and other information.
-> **Program Editor:** A built-in text editor for writing and editing programs. It supports basic text manipulation and can save programs to an paste in the project,  `/programs` , allowing for programs to be reused.

## 4. Current State and Future Work

### 4.1. Recent Fixes

As of October 12, 2025, several key bugs have been resolved, improving the emulator's stability and functionality:

-> **Program Loading:** An issue where assembly programs provided via command-line arguments were not being correctly loaded has been fixed. The parser now correctly handles comments and empty lines, allowing for successful loading and assembly of external program files.

-> **HALT Instruction:** A bug where the `HALT` instruction did not properly stop program execution has been addressed. The CPU now includes a `halted` flag that is set by the `HALT` instruction, ensuring that the emulator gracefully terminates program execution instead of attempting to run from uninitialized memory.

These fixes provide a more robust foundation for future development.

### Recently Implemented

-> **Program Loading from File:** The emulator can now load a program from a file specified as a command-line argument.

### Next Steps

The following features are planned for future development:

-> **VGA and other graphical instructions**. The implementation of graphical instructions for a new screen for graphical output is in order.
-> **Better Macro definitions**. Evolving from our current macro definitions into more modern approaches is also in order.
-> **Layered Execution and Multicore Support**. Allowing in the future, for users to deploy multiple emulated cores, and get feed back from it, is also in order, to allow for good layered execution.
