use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;

pub fn execute_movi(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // MOVI SRC, DST -> Move immediate value to register
    if let Operand::Register(reg) = op1 {
        let value = match op2 {
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("MOVI requires immediate second operand".to_string()),
        };
        cpu.registers.set(reg, value)
    } else {
        Err("MOVI requires register first operand".to_string())
    }
}

pub fn execute_movw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // MOVW SRC, DST -> Move full word value to register
    if let Operand::Register(dest_reg) = op1 {
        let value = match op2 {
            Operand::Register(src_reg) => cpu.registers.get(src_reg)?,
            Operand::Immediate(imm) => *imm as u32,
            Operand::Address(addr) => memory.read(*addr)?,
            _ => return Err("Invalid second operand for MOVW".to_string()),
        };
        cpu.registers.set(dest_reg, value)
    } else {
        Err("MOVW requires register first operand".to_string())
    }
}

pub fn execute_lodi(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // LODI DST, SRC -> Load immediate value.   
    if let Operand::Register(reg) = op1 {
        let value = match op2 {
            Operand::Immediate(imm) => *imm as u32,
            _ => return Err("LODI requires immediate second operand".to_string()),
        };
        cpu.registers.set(reg, value)
    } else {
        Err("LODI requires register first operand".to_string())
    }
}

pub fn execute_lodw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // LODW DST, SRc -> Load Word to register. 
    if let Operand::Register(reg) = op1 {
        let addr = match op2 {
            Operand::Address(addr) => *addr,
            Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
            _ => return Err("LODW requires address or register second operand".to_string()),
        };
        let value = memory.read(addr)?;
        cpu.registers.set(reg, value)
    } else {
        Err("LODW requires register first operand".to_string())
    }
}

pub fn execute_stri(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // STRI SRC, DST -> Store Immediate Source into register; 
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
        _ => return Err("STRI requires address or register first operand".to_string()),
    };
    
    let value = match op2 {
        Operand::Immediate(imm) => *imm as u32,
        _ => return Err("STRI requires immediate second operand".to_string()),
    };
    
    memory.write(addr, value)
}

pub fn execute_strw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // STRW SRC, DST -> Store Immediate Source into register;
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
        _ => return Err("STRW requires address or register first operand".to_string()),
    };
    
    let value = match op2 {
        Operand::Register(value_reg) => cpu.registers.get(value_reg)?,
        Operand::Immediate(imm) => *imm as u32,
        _ => return Err("STRW requires register or immediate second operand".to_string()),
    };
    
    memory.write(addr, value)
}

pub fn execute_push(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // PUSH SRC -> Push SRC to STACK
    let value = match op1 {
        Operand::Register(reg) => cpu.registers.get(reg)?,
        Operand::Immediate(imm) => *imm as u32,
        _ => return Err("PUSH requires register or immediate operand".to_string()),
    };
    
    let sp = cpu.registers.get("sp")?;
    memory.write(sp, value)?;
    cpu.registers.set("sp", sp - 4)
}

pub fn execute_pop(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // POP DST -> Pop from STCK and load to DST
    if let Operand::Register(reg) = op1 {
        let sp = cpu.registers.get("sp")? + 4;
        let value = memory.read(sp)?;
        cpu.registers.set(reg, value)?;
        cpu.registers.set("sp", sp)
    } else {
        Err("POP requires register operand".to_string())
    }
}

pub fn execute_xcgh(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // XCGH OP1, OP2 -> Exchange values from 1 to 2, and 2 to 1; 
    if let (Operand::Register(reg1), Operand::Register(reg2)) = (op1, op2) {
        let val1 = cpu.registers.get(reg1)?;
        let val2 = cpu.registers.get(reg2)?;
        cpu.registers.set(reg1, val2)?;
        cpu.registers.set(reg2, val1)
    } else {
        Err("XCGH requires two register operands".to_string())
    }
}