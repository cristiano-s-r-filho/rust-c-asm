# ARC - 0  | Rust-Based Assembly EMULATOR
<div align="center">
```
█████╗   ██████╗    ██████╗  ██████╗ 
██╔══██╗ ██╔══██╗ ██╔════╝  ██╔═══██╗
███████║ ██████╔╝ ██║       ██║   ██║
██╔══██║ ██╔══██╗ ██║       ██║   ██║
██║  ██║ ██║  ██║ ╚██████╗  ╚██████╔╝
╚═╝  ╚═╝ ╚═╝  ╚═╝  ╚═════╝   ╚═════╝
´´´

    **A custom x86-inspired assembly architecture emulator with ARC-0 assembly language support**

[![Rust](https://img.shields.io/badge/Rust-1.60%2B-orange?logo=rust)](https://www.rust-lang.org/)

[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue)](#license)

[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/arc0-emulator/rust.yml?branch=main)](https://github.com/yourusername/arc0-emulator/actions)

</div>``` 

<div align="center">
**Explore the world of assembly programming with ARC-0!**
</div> ```

## Overview

ARC-0 Assembly Emulator is a terminal-based emulator for a custom 8086-inspired architecture with its own assembly language. It features a rich TUI interface built with Ratatui and Crossterm, providing an interactive environment for writing, debugging, and executing ARC-0 assembly programs.

## Features

- **Full CPU Emulation**: Complete implementation of an 8086-like architecture with custom extensions
- **Interactive TUI**: Terminal user interface with multiple panels for registers, memory, and program editing
- **Program Editor**: Built-in text editor with syntax support for ARC-0 assembly
- **Memory Management**: Virtual memory system with support for different memory segments
- **Debugging Tools**: Step-by-step execution, breakpoints, and register monitoring
- **Save/Load Programs**: Persistent storage of assembly programs with .arco extension

## Architecture

The emulator implements a custom architecture with:

- 16-bit registers (AX, BX, CX, DX, SP, BP, SI, DI)
- Segment registers (CS, DS, SS, ES)
- Control registers (PC, FLAGS)
- 64KB addressable memory space
- Stack-based subroutine calls
- Flag-based conditional jumps

## Installation

### Prerequisites

- Rust 1.60 or newer
- Cargo (Rust package manager)

### Getting Started

To build and run this project:

1.  Clone the repository:
    ```bash
    git clone https://github.com/cristiano-s-r-filho/rust-c-asm.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd rust-c-asm
    cd asm_cli
    ```
3.  Build the project using Cargo:
    ```bash
    cargo build
    ```
4.  Execute the compiled program:
    ```bash
    cargo run
    ```

## Future Applications

The knowledge acquired from this project forms a strong basis for understanding how software interacts with hardware, which is critical for advanced cybersecurity roles, including vulnerability analysis, reverse engineering, and developing secure operating systems components. I aim to apply these insights to contribute to robust security strategies and tech implementations.
