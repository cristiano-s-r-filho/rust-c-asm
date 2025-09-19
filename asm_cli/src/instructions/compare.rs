use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;

pub fn execute_cmpw(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    let value1 = match op1 {
        Operand::Register(reg) => cpu.registers.get(reg)?,
        Operand::Immediate(imm) => *imm as u32,
        _ => return Err("CMPW requires register or immediate first operand".to_string()),
    };
    
    let value2 = match op2 {
        Operand::Register(reg) => cpu.registers.get(reg)?,
        Operand::Immediate(imm) => *imm as u32,
        _ => return Err("CMPW requires register or immediate second operand".to_string()),
    };
    
    // Compare by subtracting and updating flags without storing result
    let result = value1.wrapping_sub(value2);
    cpu.registers.update_flags(result, value1, value2, true);
    Ok(())
}

pub fn execute_jmp(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    let address = match op1 {
        Operand::Label(label) => cpu.symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
        Operand::Immediate(imm) => &(*imm as u32),
        Operand::Address(addr) => addr,
        _ => return Err("JMP requires label, immediate, or address operand".to_string()),
    };
    
    cpu.registers.set("pc", *address)
}

pub fn execute_call(cpu: &mut CPU, op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    let address = match op1 {
        Operand::Label(label) => cpu.symbol_table.get(label).ok_or(format!("Unknown label: {}", label))?,
        Operand::Immediate(imm) => &(*imm as u32),
        Operand::Address(addr) => addr,
        _ => return Err("CALL requires label, immediate, or address operand".to_string()),
    };
    
    // Push return address (current PC + 1) onto call stack
    let return_addr = cpu.registers.get("pc")? + 1;
    cpu.call_stack.push(return_addr)?;
    
    // Jump to subroutine
    cpu.registers.set("pc", *address)
}

pub fn execute_ret(cpu: &mut CPU, _op1: &Operand, _op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // Pop return address from call stack
    let return_addr = cpu.call_stack.pop()?;
    
    // Jump to return address
    cpu.registers.set("pc", return_addr)
}

pub fn execute_je(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    if cpu.registers.get_flag("zero")? {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}

pub fn execute_jne(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    if !cpu.registers.get_flag("zero")? {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}

pub fn execute_jgt(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if greater than (signed)
    let zero = cpu.registers.get_flag("zero")?;
    let sign = cpu.registers.get_flag("sign")?;
    let overflow = cpu.registers.get_flag("overflow")?;
    
    // For signed comparison: (ZF = 0) and (SF = OF)
    if !zero && (sign == overflow) {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}

pub fn execute_jge(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if greater than or equal (signed)
    let _zero = cpu.registers.get_flag("zero")?;
    let sign = cpu.registers.get_flag("sign")?;
    let overflow = cpu.registers.get_flag("overflow")?;
    
    // For signed comparison: (SF = OF)
    if sign == overflow {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}

pub fn execute_jlt(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if less than (signed)
    let sign = cpu.registers.get_flag("sign")?;
    let overflow = cpu.registers.get_flag("overflow")?;
    
    // For signed comparison: (SF != OF)
    if sign != overflow {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}

pub fn execute_jle(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if less than or equal (signed)
    let zero = cpu.registers.get_flag("zero")?;
    let sign = cpu.registers.get_flag("sign")?;
    let overflow = cpu.registers.get_flag("overflow")?;
    
    // For signed comparison: (ZF = 1) or (SF != OF)
    if zero || (sign != overflow) {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}

pub fn execute_js(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if sign (negative)
    if cpu.registers.get_flag("sign")? {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}

pub fn execute_jco(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // Jump if carry or overflow
    let carry = cpu.registers.get_flag("carry")?;
    let overflow = cpu.registers.get_flag("overflow")?;
    
    if carry || overflow {
        execute_jmp(cpu, op1, &Operand::Immediate(0), memory)
    } else {
        Ok(())
    }
}
