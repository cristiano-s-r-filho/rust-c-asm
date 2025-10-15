use crate::memory::main_memory::{TEXT_START, STACK_END};

#[derive(Debug, Clone, PartialEq)]
pub enum Reg {
    AX, BX, CX, DX, EX, FX, GX, HX, // Added new GP registers
    SP, BP, SI, DI, PC, FLAGS,
}

#[derive(Debug, Clone)]
pub struct Registers {
    // All registers are 32-bit wide.
    // Instructions will determine how to interpret the bits (integer or float).
    pub ax: u32,
    pub bx: u32,
    pub cx: u32,
    pub dx: u32,
    pub ex: u32, // New
    pub fx: u32, // New
    pub gx: u32, // New
    pub hx: u32, // New
    pub sp: u32,
    pub bp: u32,
    pub si: u32,
    pub di: u32,
    pub pc: u32,
    pub flags: u32,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ax: 0, bx: 0, cx: 0, dx: 0, ex: 0, fx: 0, gx: 0, hx: 0,
            sp: STACK_END, bp: 0, si: 0, di: 0,
            pc: TEXT_START, flags: 0,
        }
    }
    
    pub fn reset(&mut self) {
        *self = Self::new();
    }
    
    pub fn get(&self, reg: &Reg) -> Result<u32, String> {
        match reg {
            Reg::AX => Ok(self.ax), Reg::BX => Ok(self.bx), Reg::CX => Ok(self.cx), Reg::DX => Ok(self.dx),
            Reg::EX => Ok(self.ex), Reg::FX => Ok(self.fx), Reg::GX => Ok(self.gx), Reg::HX => Ok(self.hx),
            Reg::SP => Ok(self.sp), Reg::BP => Ok(self.bp), Reg::SI => Ok(self.si), Reg::DI => Ok(self.di),
            Reg::PC => Ok(self.pc), Reg::FLAGS => Ok(self.flags),
        }
    }
    
    pub fn set(&mut self, reg: &Reg, value: u32) -> Result<(), String> {
        match reg {
            Reg::AX => self.ax = value,
            Reg::BX => self.bx = value,
            Reg::CX => self.cx = value,
            Reg::DX => self.dx = value,
            Reg::EX => self.ex = value,
            Reg::FX => self.fx = value,
            Reg::GX => self.gx = value,
            Reg::HX => self.hx = value,
            Reg::SP => self.sp = value,
            Reg::BP => self.bp = value,
            Reg::SI => self.si = value,
            Reg::DI => self.di = value,
            Reg::PC => self.pc = value,
            Reg::FLAGS => self.flags = value,
        }
        Ok(())
    }
    
    // This is the original, integer-based flag logic. It will be used by
    // integer instructions (like bitwise) and CMP.
    pub fn update_flags_u32(&mut self, result: u32, op1: u32, op2: u32, is_subtraction: bool) {
        // Zero flag
        self.set_flag("zero", result == 0);
        
        // Sign flag
        self.set_flag("sign", (result as i32) < 0);
        
        // Carry flag
        if is_subtraction {
            self.set_flag("carry", op1 < op2);
        } else {
            self.set_flag("carry", result < op1);
        }
        
        // Overflow flag
        if is_subtraction {
            let op1_signed = op1 as i32;
            let op2_signed = op2 as i32;
            let result_signed = result as i32;
            self.set_flag("overflow", (op1_signed.is_negative() && op2_signed.is_positive() && result_signed.is_positive()) ||
                                     (op1_signed.is_positive() && op2_signed.is_negative() && result_signed.is_negative()));
        } else {
            let op1_signed = op1 as i32;
            let op2_signed = op2 as i32;
            let result_signed = result as i32;
            self.set_flag("overflow", (op1_signed.is_positive() && op2_signed.is_positive() && result_signed.is_negative()) ||
                                     (op1_signed.is_negative() && op2_signed.is_negative() && result_signed.is_positive()));
        }
    }

    // This is a new, simplified flag update for float operations.
    pub fn update_flags_f32(&mut self, result: f32) {
        self.set_flag("zero", result == 0.0);
        self.set_flag("sign", result.is_sign_negative());
        // Float operations do not typically set carry or overflow in the same way.
        self.set_flag("carry", false);
        self.set_flag("overflow", false);
    }
    
    pub fn set_flag(&mut self, flag_name: &str, value: bool) {
        let bit_position = match flag_name.to_lowercase().as_str() {
            "carry" => 0,
            "zero" => 6,
            "sign" => 7,
            "interrupt" => 9,
            "string" => 10,
            "overflow" => 11,
            "macro" => 12,
            _ => return,
        };
        
        if value {
            self.flags |= 1 << bit_position;
        } else {
            self.flags &= !(1 << bit_position);
        }
    }
    
    pub fn get_flag(&self, flag_name: &str) -> Result<bool, String> {
        let bit_position = match flag_name.to_lowercase().as_str() {
            "carry" => 0,
            "zero" => 6,
            "sign" => 7,
            "interrupt" => 9,
            "string" => 10,
            "overflow" => 11,
            "macro" => 12,
            _ => return Err(format!("Unknown flag: {}", flag_name)),
        };
        
        Ok((self.flags & (1 << bit_position)) != 0)
    }
}