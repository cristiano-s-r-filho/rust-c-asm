use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::Operand;
use crate::memory::registers::Reg;

pub fn execute_movi(cpu: &mut CPU, op1: &Operand, op2: &Operand, _memory: &mut WorkMemory) -> Result<(), String> {
    // MOVI DST, SRC -> Move immediate value to register, storing as f32 bits
    if let Operand::Register(reg) = op1 {
        let value = match op2 {
            Operand::Immediate(imm) => (*imm as f32).to_bits(),
            _ => return Err("MOVI requires immediate second operand".to_string()),
        };
        cpu.registers.set(reg, value)
    } else {
        Err("MOVI requires register first operand".to_string())
    }
}

pub fn execute_movw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // MOVW DST, SRC -> Move full word value to register
    if let Operand::Register(dest_reg) = op1 {
        let value = match op2 {
            Operand::Register(src_reg) => cpu.registers.get(src_reg)?,
            Operand::Immediate(imm) => (*imm as f32).to_bits(),
            Operand::Address(addr) => memory.read_u32(*addr)?,
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
            Operand::Immediate(imm) => (*imm as f32).to_bits(),
            _ => return Err("LODI requires immediate second operand".to_string()),
        };
        cpu.registers.set(reg, value)
    } else {
        Err("LODI requires register first operand".to_string())
    }
}

pub fn execute_lodw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // LODW DST, SRC -> Load Word to register. 
    if let Operand::Register(reg) = op1 {
        let addr = match op2 {
            Operand::Address(addr) => *addr,
            Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
            _ => return Err("LODW requires address or register second operand".to_string()),
        };
        let value = memory.read_u32(addr)?;
        cpu.registers.set(reg, value)
    } else {
        Err("LODW requires register first operand".to_string())
    }
}

pub fn execute_stri(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // STRI DST, SRC -> Store Immediate Source into memory
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
        _ => return Err("STRI requires address or register first operand".to_string()),
    };
    
    let value = match op2 {
        Operand::Immediate(imm) => (*imm as f32).to_bits(),
        _ => return Err("STRI requires immediate second operand".to_string()),
    };
    
    memory.write_u32(addr, value)
}

pub fn execute_strw(cpu: &mut CPU, op1: &Operand, op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // STRW DST, SRC -> Store Word Source into memory
    let addr = match op1 {
        Operand::Address(addr) => *addr,
        Operand::Register(addr_reg) => cpu.registers.get(addr_reg)?,
        _ => return Err("STRW requires address or register first operand".to_string()),
    };
    
    let value = match op2 {
        Operand::Register(value_reg) => cpu.registers.get(value_reg)?,
        Operand::Immediate(imm) => (*imm as f32).to_bits(),
        _ => return Err("STRW requires register or immediate second operand".to_string()),
    };
    
    memory.write_u32(addr, value)
}

pub fn execute_push(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // PUSH SRC -> Push SRC to STACK
    let value = match op1 {
        Operand::Register(reg) => cpu.registers.get(reg)?,
        Operand::Immediate(imm) => (*imm as f32).to_bits(),
        _ => return Err("PUSH requires register or immediate operand".to_string()),
    };
    
    let sp = cpu.registers.get(&Reg::SP)?;
    memory.write_u32(sp, value)?;
    cpu.registers.set(&Reg::SP, sp.wrapping_sub(4))
}

pub fn execute_pop(cpu: &mut CPU, op1: &Operand, _op2: &Operand, memory: &mut WorkMemory) -> Result<(), String> {
    // POP DST -> Pop from STACK and load to DST
    if let Operand::Register(reg) = op1 {
        let sp = cpu.registers.get(&Reg::SP)?.wrapping_add(4);
        let value = memory.read_u32(sp)?;
        cpu.registers.set(reg, value)?;
        cpu.registers.set(&Reg::SP, sp)
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

#[cfg(test)]
mod moves_test {
    use super::*;
    use crate::chips::cpu::CPU;
    use crate::memory::main_memory::WorkMemory;
    use crate::utils::operands::Operand;
    use crate::memory::registers::Reg;

    #[test]
    fn movi_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        execute_movi(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(123), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 123.0);
    }

    #[test]
    fn movw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // MOVW AX, BX
        cpu.registers.set(&Reg::BX, 456.7f32.to_bits()).unwrap();
        execute_movw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 456.7f32.to_bits());

        // MOVW AX, 789
        execute_movw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(789), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 789.0);

        // MOVW AX, [100]
        memory.write_u32(100, 101.112f32.to_bits()).unwrap();
        execute_movw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Address(100), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 101.112f32.to_bits());
    }

    #[test]
    fn lodi_lodw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // LODI AX, 123
        execute_lodi(&mut cpu, &Operand::Register(Reg::AX), &Operand::Immediate(123), &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 123.0);

        // LODW AX, [200]
        memory.write_u32(200, 456.7f32.to_bits()).unwrap();
        execute_lodw(&mut cpu, &Operand::Register(Reg::AX), &Operand::Address(200), &mut memory).unwrap();
        assert_eq!(cpu.registers.get(&Reg::AX).unwrap(), 456.7f32.to_bits());
    }

    #[test]
    fn stri_strw_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // STRI [300], 123
        execute_stri(&mut cpu, &Operand::Address(300), &Operand::Immediate(123), &mut memory).unwrap();
        assert_eq!(f32::from_bits(memory.read_u32(300).unwrap()), 123.0);

        // STRW [400], AX
        cpu.registers.set(&Reg::AX, 789.1f32.to_bits()).unwrap();
        execute_strw(&mut cpu, &Operand::Address(400), &Operand::Register(Reg::AX), &mut memory).unwrap();
        assert_eq!(memory.read_u32(400).unwrap(), 789.1f32.to_bits());
    }

    #[test]
    fn push_pop_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        // Set initial stack pointer
        cpu.registers.set(&Reg::SP, 1020).unwrap();

        // PUSH 123.0
        execute_push(&mut cpu, &Operand::Immediate(123), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(memory.read_u32(cpu.registers.get(&Reg::SP).unwrap() + 4).unwrap()), 123.0);

        // POP AX
        execute_pop(&mut cpu, &Operand::Register(Reg::AX), &Operand::None, &mut memory).unwrap();
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 123.0);
    }

    #[test]
    fn xcgh_behavior() {
        let mut cpu = CPU::new();
        let mut memory = WorkMemory::new(1024);

        cpu.registers.set(&Reg::AX, 123.5f32.to_bits()).unwrap();
        cpu.registers.set(&Reg::BX, 456.5f32.to_bits()).unwrap();

        execute_xcgh(&mut cpu, &Operand::Register(Reg::AX), &Operand::Register(Reg::BX), &mut memory).unwrap();

        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::AX).unwrap()), 456.5);
        assert_eq!(f32::from_bits(cpu.registers.get(&Reg::BX).unwrap()), 123.5);
    }
}