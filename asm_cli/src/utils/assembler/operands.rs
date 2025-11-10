use crate::memory::registers::Reg;
// Removed: use crate::utils::logger; // Import the logger module

/// Represents the different types of operands an assembly instruction can have.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// A CPU register (e.g., AX, BX).
    Register(Reg),
    /// An immediate 16-bit unsigned integer value.
    Immediate(u32),
    /// A memory address (32-bit unsigned integer).
    Address(u32),
    /// A memory address whose value is held in a register.
    AddressRegister(Reg),
    /// A symbolic label that will be resolved to an address during assembly.
    Label(String),
    /// A string literal.
    String(String),
    /// A CPU flag (e.g., Carry, Zero).
    Flag(u8),
    /// No operand.
    None,
}

/// Parses a string slice into a `Reg` enum variant.
fn parse_register(input: &str) -> Result<Reg, String> {
    match input.to_lowercase().as_str() {
        "ax" => Ok(Reg::AX),
        "bx" => Ok(Reg::BX),
        "cx" => Ok(Reg::CX),
        "dx" => Ok(Reg::DX),
        "ex" => Ok(Reg::EX),
        "fx" => Ok(Reg::FX),
        "gx" => Ok(Reg::GX),
        "hx" => Ok(Reg::HX),
        "sp" => Ok(Reg::SP),
        "bp" => Ok(Reg::BP),
        "si" => Ok(Reg::SI),
        "di" => Ok(Reg::DI),
        "pc" => Ok(Reg::PC),
        "flags" => Ok(Reg::FLAGS),
        _ => Err(format!("Invalid register name: {}", input)),
    }
}

/// Parses a string slice into an `Operand` enum variant.
///
/// This function attempts to identify the type of operand based on its format:
/// - String literals (enclosed in double quotes)
/// - Register names (case-insensitive)
/// - Flag names (case-insensitive)
/// - Memory addresses (enclosed in square brackets, can be decimal or hexadecimal)
/// - Immediate values (decimal, hexadecimal with `0x` prefix, or binary with `0b` prefix)
/// - Labels (any other valid identifier)
///
/// # Arguments
///
/// * `input` - A string slice representing the operand.
///
/// # Returns
///
/// * `Result<Operand, String>` - `Ok(Operand)` on successful parsing, or `Err(String)` on failure.
pub fn parse_operand(input: &str) -> Result<Operand, String> {
    let input = input.trim();

    // 1. Check for String literal
    if input.starts_with('"') && input.ends_with('"') {
        let s = input[1..input.len() - 1].to_string();
        return Ok(Operand::String(s));
    }

    // 2. Check for Register
    if let Ok(reg) = parse_register(input) {
        return Ok(Operand::Register(reg));
    }

    // 3. Check for Flag names
    match input.to_lowercase().as_str() {
        "carry" => {
            return Ok(Operand::Flag(0));
        }
        "zero" => {
            return Ok(Operand::Flag(1));
        }
        "sign" => {
            return Ok(Operand::Flag(2));
        }
        "interrupt" => {
            return Ok(Operand::Flag(3));
        }
        "overflow" => {
            return Ok(Operand::Flag(5));
        }
        "macro" => {
            return Ok(Operand::Flag(6));
        }
        "stack_dir" => {
            return Ok(Operand::Flag(7));
        }
        _ => {} // Not a flag, continue
    };

    // 4. Check for Address (e.g., [123], [0x7B], or [AX])
    if input.starts_with('[') && input.ends_with(']') {
        let addr_str = &input[1..input.len() - 1];

        // Try to parse as a Register for AddressRegister
        if let Ok(reg) = parse_register(addr_str) {
            return Ok(Operand::AddressRegister(reg));
        }

        let addr = if let Some(stripped) = addr_str.strip_prefix("0x") {
            u32::from_str_radix(stripped, 16)
        } else if let Some(stripped) = addr_str.strip_prefix("0b") {
            u32::from_str_radix(stripped, 2)
        } else {
            addr_str.parse::<u32>()
        };
        if let Ok(num) = addr {
            return Ok(Operand::Address(num));
        } else {
            // If it's not a valid number, it could be a label inside the brackets
            if !addr_str.is_empty() && !addr_str.contains(|c: char| c.is_whitespace()) {
                return Ok(Operand::Label(addr_str.to_string()));
            }
            let error_msg = format!("Invalid address format: {}", input);
            panic!("{}", error_msg);
        }
    }

    // 5. Check for Immediate value (integer, hex, binary, or float)
    let imm_result = if let Some(stripped) = input.strip_prefix("0x") {
        u32::from_str_radix(stripped, 16).map_err(|e| e.to_string()) // Convert ParseIntError to String
    } else if let Some(stripped) = input.strip_prefix("0b") {
        u32::from_str_radix(stripped, 2).map_err(|e| e.to_string()) // Convert ParseIntError to String
    } else {
        // Try parsing as u32 first
        let parsed_u32 = input.parse::<u32>();
        if parsed_u32.is_ok() {
            parsed_u32.map_err(|e| e.to_string()) // Convert ParseIntError to String
        } else {
            // If u32 parsing fails, try parsing as f32 and truncate
            match input.parse::<f32>() {
                Ok(f_val) => Ok(f_val as u32),
                Err(e) => {
                    let error_msg = format!("Invalid immediate value format: {}", e);
                    Err(error_msg)
                }
            }
        }
    };

    if let Ok(num) = imm_result {
        return Ok(Operand::Immediate(num));
    }

    // 6. If all else fails, it's a Label
    // Basic validation: labels shouldn't contain whitespace or brackets.
    if !input.is_empty() && !input.contains(|c: char| c.is_whitespace() || c == '[' || c == ']') {
        return Ok(Operand::Label(input.to_string()));
    }

    let error_msg = format!("Invalid or unknown operand: {}", input);
    panic!("{}", error_msg);
}
