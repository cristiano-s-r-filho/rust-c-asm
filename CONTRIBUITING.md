# Contributing to ARC-0 Assembly Emulator

Thank you for your interest in contributing to the ARC-0 Assembly Emulator! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you are expected to uphold our Code of Conduct:

- Be respectful and inclusive
- Exercise consideration and respect in your speech and actions
- Attempt collaboration before conflict
- Refrain from demeaning, discriminatory, or harassing behavior

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates.

When creating a bug report, include:

- A clear, descriptive title
- Detailed steps to reproduce the issue
- Expected behavior vs actual behavior
- Screenshots if applicable
- Your environment (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are welcome. Please provide:

- A clear, descriptive title
- A detailed description of the proposed enhancement
- Examples of the proposed functionality
- Any additional context

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

1. Install Rust (1.60+)
2. Fork and clone the repository
3. Build the project: `cargo build`
4. Run tests: `cargo test`
5. Run the emulator: `cargo run`

## Project Structure

asm_cli
|-> src
|---> chips
|------> cpu.rs                    // Cpu Processor implementaition.
|------> call_stack.rs             // Call Stack H2 chip implementation.
|------> instruction_queue         // Instruction Cache H1 chip implementation.
|---> intructions
|------> aritmethic.rs             // Aritmethic Instructions implementation.
|------> compare.rs                // Comparison and Pointer Instructions implementation.
|------> bitwise.rs                // Bitwise Instructions implementation.
|------> moves.rs                  // Moves and Memory Allocation Instructions implementation.  
|------> io.rs                     // I/O Instructions implementations.
|------> system.rs                 // System Level Instructions implemenations.
|---> memory
|------> main_memory.rs            // Main RAM strip chip.
|------> register.rs               // Registers implementation.
|---> utils
|------> command_processor.rs      // Two-Pass Full Assembler implementation
|------> symbols.rs                // [FEATURE UNDER REVISION]
|------> operand.rs                // Operand Definitions for Assembler
|------> tui.rs                    // App State, Modes and Event handling definitions
|------> widgets.rs                // Widgets and Screens for rendering.
|--> lib.rs                        // Library definitions for when we post on Cargo (WIP)
|--> main.rs                       // Main Exit endpoint for execution.
|- Cargo.lock
|- Cargo.toml                      // Dependencies Management and Versioning.

## Coding Guidelines

- Follow Rust naming conventions
- Write documentation comments for public items
- Include tests for new functionality
- Format code with `cargo fmt`
- Check for warnings with `cargo clippy`

## Testing

Please ensure your code passes all tests:

```bash
cargo test
```

Add tests for new functionality to maintain test coverage.

## Documentation

- Update documentation when adding new features

- Ensure code comments are clear and helpful

- Keep the README.md up to date with new functionality.

- Keep the DOCUMENTATION.md up to date with new functionality.
