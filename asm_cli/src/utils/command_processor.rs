use crate::chips::cpu::CPU;
use crate::memory::main_memory::WorkMemory;
use crate::utils::operands::{Operand, parse_operand};
use crate::instructions::{
    moves,
    aritmethic, 
    bitwise, 
    compare, 
};

#[derive(Debug)]
pub struct Command {
    pub opcode: String,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }
    
    let opcode = parts[0].to_string();
    let operands: Vec<Option<Operand>> = parts[1..].iter()
        .map(|s| parse_operand(s).ok())
        .collect();
    
    let operand1 = operands.get(0).cloned().flatten();
    let operand2 = operands.get(1).cloned().flatten();
    
    Ok(Command { opcode, operand1, operand2 })
}

pub fn execute_command(command: Command, cpu: &mut CPU, memory: &mut WorkMemory) -> Result<(), String> {
    let op1 = command.operand1.clone().unwrap_or(Operand::Register("ax".to_string()));
    let op2 = command.operand2.clone().unwrap_or(Operand::Immediate(0));
    
    match command.opcode.to_lowercase().as_str() {
        "movi" => moves::execute_movi(cpu, &op1, &op2, memory),
        "movw" => moves::execute_movw(cpu, &op1, &op2, memory),
        "lodi" => moves::execute_lodi(cpu, &op1, &op2, memory),
        "lodw" => moves::execute_lodw(cpu, &op1, &op2, memory),
        "stri" => moves::execute_stri(cpu, &op1, &op2, memory),
        "strw" => moves::execute_strw(cpu, &op1, &op2, memory),
        "push" => moves::execute_push(cpu, &op1, &op2, memory),
        "pop" =>  moves::execute_pop(cpu, &op1, &op2, memory),
        "xcgh" => moves::execute_xcgh(cpu, &op1, &op2, memory),
        "addw" => aritmethic::execute_addw(cpu, &op1, &op2, memory),
        "addi" => aritmethic::execute_addi(cpu, &op1, &op2, memory),
        "dec" => aritmethic::execute_dec(cpu, &op1, &op2, memory),
        "inc" => aritmethic::execute_inc(cpu, &op1, &op2, memory),
        "neg" => aritmethic::execute_neg(cpu, &op1, &op2, memory),
        "mul" => aritmethic::execute_mul(cpu, &op1, &op2, memory),
        "subw" => aritmethic::execute_subw(cpu, &op1, &op2, memory),
        "subi" => aritmethic::execute_subi(cpu, &op1, &op2, memory),
        "not" => bitwise::execute_not(cpu, &op1, &op2, memory),
        "and" => bitwise::execute_and(cpu, &op1, &op2, memory),
        "or" => bitwise::execute_or(cpu, &op1, &op2, memory),
        "xor" => bitwise::execute_xor(cpu, &op1, &op2, memory),
        "cmpw" => compare::execute_cmpw(cpu, &op1, &op2, memory),
        "jmp" => compare::execute_jmp(cpu, &op1, &op2, memory),
        "call" => compare::execute_call(cpu, &op1, &op2, memory),
        "ret" => compare::execute_ret(cpu, &op1, &op2, memory),
        "je" => compare::execute_je(cpu, &op1, &op2, memory),
        "jne" => compare::execute_jne(cpu, &op1, &op2, memory),
        "jgt" => compare::execute_jgt(cpu, &op1, &op2, memory),
        "jge" => compare::execute_jge(cpu, &op1, &op2, memory),
        "jlt" => compare::execute_jlt(cpu, &op1, &op2, memory),
        "jle" => compare::execute_jle(cpu, &op1, &op2, memory),
        "js" => compare::execute_js(cpu, &op1, &op2, memory),
        "jco" => compare::execute_jco(cpu, &op1, &op2, memory),
        _ => Err(format!("Unknown opcode: {}", command.opcode)),
    }
}