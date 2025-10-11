use crate::memory::registers::Reg;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Reg),
    Immediate(u16),
    Address(u32),
    Label(String),
    None,
}

pub fn parse_operand(input: &str) -> Result<Operand, String> {
    let input = input.trim();

    // 1. Check for Register
    match input.to_lowercase().as_str() {
        "ax" => return Ok(Operand::Register(Reg::AX)),
        "bx" => return Ok(Operand::Register(Reg::BX)),
        "cx" => return Ok(Operand::Register(Reg::CX)),
        "dx" => return Ok(Operand::Register(Reg::DX)),
        "ex" => return Ok(Operand::Register(Reg::EX)),
        "fx" => return Ok(Operand::Register(Reg::FX)),
        "gx" => return Ok(Operand::Register(Reg::GX)),
        "hx" => return Ok(Operand::Register(Reg::HX)),
        "sp" => return Ok(Operand::Register(Reg::SP)),
        "bp" => return Ok(Operand::Register(Reg::BP)),
        "si" => return Ok(Operand::Register(Reg::SI)),
        "di" => return Ok(Operand::Register(Reg::DI)),
        "pc" => return Ok(Operand::Register(Reg::PC)),
        "flags" => return Ok(Operand::Register(Reg::FLAGS)),
        _ => {} // Not a register, continue
    };

    // 2. Check for Address (e.g., [123] or [0x7B])
    if input.starts_with('[') && input.ends_with(']') {
        let addr_str = &input[1..input.len() - 1];
        let addr = if addr_str.starts_with("0x") {
            u32::from_str_radix(&addr_str[2..], 16)
        } else if addr_str.starts_with("0b") {
            u32::from_str_radix(&addr_str[2..], 2)
        } else {
            addr_str.parse::<u32>()
        };
        if let Ok(num) = addr {
            return Ok(Operand::Address(num));
        } else {
            return Err(format!("Invalid address format: {}", input));
        }
    }

    // 3. Check for Immediate value
    let imm = if input.starts_with("0x") {
        u16::from_str_radix(&input[2..], 16)
    } else if input.starts_with("0b") {
        u16::from_str_radix(&input[2..], 2)
    } else {
        input.parse::<u16>()
    };
    if let Ok(num) = imm {
        return Ok(Operand::Immediate(num));
    }

    // 4. If all else fails, it's a Label
    // Basic validation: labels shouldn't contain whitespace or brackets.
    if !input.is_empty() && !input.contains(|c: char| c.is_whitespace() || c == '[' || c == ']') {
        return Ok(Operand::Label(input.to_string()));
    }

    Err(format!("Invalid or unknown operand: {}", input))
}
