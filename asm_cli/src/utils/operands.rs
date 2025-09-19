#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(String),
    Immediate(u16),
    Address(u32),
    Label(String),
}

pub fn parse_operand(input: &str) -> Result<Operand, String> {
    let input = input.trim().to_lowercase();
    
    // Check if it's a register
    let registers = ["ax", "bx", "cx", "dx", "sp", "bp", "si", "di", "pc", "flags"];
    if registers.contains(&input.as_str()) {
        return Ok(Operand::Register(input));
    }
    
    // Check if it's a hexadecimal immediate
    if input.starts_with("0x") {
        let hex_val = u16::from_str_radix(&input[2..], 16)
            .map_err(|_| format!("Invalid hexadecimal value: {}", input))?;
        return Ok(Operand::Immediate(hex_val));
    }
    
    // Check if it's a decimal immediate
    if let Ok(val) = input.parse::<u16>() {
        return Ok(Operand::Immediate(val));
    }
    
    // Check if it's a character literal
    if input.starts_with('\'') && input.ends_with('\'') && input.len() == 3 {
        let ch = input.chars().nth(1).unwrap();
        return Ok(Operand::Immediate(ch as u16));
    }
    
    // Check if it's a memory address
    if input.starts_with('[') && input.ends_with(']') {
        let addr_str = &input[1..input.len()-1];
        let addr = u32::from_str_radix(addr_str, 16)
            .map_err(|_| format!("Invalid memory address: {}", addr_str))?;
        return Ok(Operand::Address(addr));
    }
    
    // Assume it's a label if none of the above
    Ok(Operand::Label(input))
}

