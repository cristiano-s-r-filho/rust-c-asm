# A.R.C.S | Rust-Based Custom Assembly Emulator

A custom x86-inspired assembly architecture emulator with ARC assembly language support
[![Rust](https://img.shields.io/badge/Rust-1.60%2B-orange?logo=rust)](https://www.rust-lang.org/)

[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue)](#license)

[![Build Status](https://img.shields.io/github/actions/workflow/status/cristiano-s-r-filho/rust-c-asm/asm_cli/rust.yml?branch=main)](https://github.com/cristiano-s-r-filho/rust-c-asm/asm_cli/actions)

**Explore the world of assembly programming with A.R.C.S!**

**For detailed information about the emulator's architecture, usage, and features, please see the [full documentation](DOCUMENTATION.md).**

## Overview

A.R.C.S (Advanced RISC Computer System) is a terminal-based emulator for a custom ARC architecture with its own assembly language. It features a rich TUI interface built with Ratatui and Crossterm, providing an interactive environment for writing, debugging, and executing ARC assembly programs.

## Features

- **Full CPU Emulation**: Complete implementation of a custom ARC architecture, inspired by AVR, 8086-like, and ARM architectures with custom extensions. Supports configurable memory sizes.
- **Interactive TUI**: Terminal user interface with multiple panels for registers, memory, and program editing.
- **Program Editor**: Built-in text editor with syntax support for the ARC assembly language.
- **Memory Management**: Virtual memory system with configurable total memory size and support for custom segment directives (`.text_start`, `.stack_start`, `.stack_size`).
- **Debugging Tools**: Step-by-step execution, breakpoints, and register monitoring.
- **Save/Load Programs**: Persistent storage of assembly programs with `.arc` extension.

## Architecture

The emulator implements a custom ARC architecture with:

- 32-bit General and Pointer registers (AX, BX, CX, DX, EX, FX, GX, HX, SP, BP, SI, DI)
- Segment registers (CS, DS, SS, ES)
- Control registers (PC, FLAGS)
- Configurable addressable memory space (from 64KB to 8MB)
- Support for custom segment directives (`.text_start`, `.stack_start`, `.stack_size`)
- Stack-based subroutine calls
- Flag-based conditional jumps

## Installation

### Prerequisites

- Rust 1.60 or newer
- Cargo (Rust package manager)

### Getting Started

To build and run this project:

1. Clone the repository:

    ```bash
    git clone https://github.com/cristiano-s-r-filho/rust-c-asm.git
    ```

2. Navigate to the project directory:

    ```bash
    cd rust-c-asm
    cd asm_cli
    ```

3. Install the `arcs` command:

    ```bash
    cargo install --path .
    ```
    This command compiles the project and installs the `arcs` executable to your Cargo bin directory (usually `~/.cargo/bin` or `%USERPROFILE%\.cargo\bin`). Ensure this directory is in your system's PATH.

4. Run the A.R.C.S emulator:

    ```bash
    arcs
    ```
    You can also specify the memory size using the `--memory-size` option:
    ```bash
    arcs --memory-size 1MB
    ```
    (Supported sizes: 64KB to 8MB, e.g., `64KB`, `2MB`. Default is 64KB.)

## Future Applications

The knowledge acquired from this project forms a strong basis for understanding how software interacts with hardware, which is critical for advanced cybersecurity roles, including vulnerability analysis, reverse engineering, and developing secure operating systems components. I aim to apply these insights to contribute to robust security strategies and tech implementations.
