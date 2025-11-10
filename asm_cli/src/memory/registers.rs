//! # Registers Module
//!
//! This module defines the `Reg` enum, representing the various CPU registers,
//! and the `Registers` struct, which manages the state of these registers
//! and provides methods for accessing and modifying them, including flag manipulation.

/// Represents the different types of CPU registers.
#[derive(Debug, Clone, PartialEq)]
pub enum Reg {
    /// Accumulator register.
    AX,
    /// Base register.
    BX,
    /// Counter register.
    CX,
    /// Data register.
    DX,
    /// Extended general-purpose register.
    EX,
    /// Extended general-purpose register.
    FX,
    /// Extended general-purpose register.
    GX,
    /// Extended general-purpose register.
    HX,
    /// Stack Pointer.
    SP,
    /// Base Pointer.
    BP,
    /// Source Index register.
    SI,
    /// Destination Index register.
    DI,
    /// Program Counter.
    PC,
    /// Flags register.
    FLAGS,
}

/// Manages the state of all CPU registers.
#[derive(Debug, Clone)]
pub struct Registers {
    /// Accumulator register.
    pub ax: u32,
    /// Base register.
    pub bx: u32,
    /// Counter register.
    pub cx: u32,
    /// Data register.
    pub dx: u32,
    /// Extended general-purpose register.
    pub ex: u32,
    /// Extended general-purpose register.
    pub fx: u32,
    /// Extended general-purpose register.
    pub gx: u32,
    /// Extended general-purpose register.
    pub hx: u32,
    /// Stack Pointer.
    pub sp: u32,
    /// Base Pointer.
    pub bp: u32,
    /// Source Index register.
    pub si: u32,
    /// Destination Index register.
    pub di: u32,
    /// Program Counter.
    pub pc: u32,
    /// Flags register, where individual bits represent different CPU flags.
    pub flags: u32,
}

impl Default for Registers {
    /// Creates a new `Registers` instance with default values.
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    /// Creates a new `Registers` instance, initializing all registers to 0.
    pub fn new() -> Self {
        Self {
            ax: 0, bx: 0, cx: 0, dx: 0, ex: 0, fx: 0, gx: 0, hx: 0,
            sp: 0, bp: 0, si: 0, di: 0,
            pc: 0, flags: 0,
        }
    }
    
    /// Resets all registers to their initial default values.
    /// Resets all registers to their initial default values.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
    
    /// Retrieves the 32-bit value of a specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - A reference to the `Reg` enum variant representing the desired register.
    ///
    /// # Returns
    ///
    /// * `Result<u32, String>` - The 32-bit value of the register on success, or an error message on failure.
    /// Retrieves the 32-bit value of a specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - A reference to the `Reg` enum variant representing the desired register.
    ///
    /// # Returns
    ///
    /// * `Result<u32, String>` - The 32-bit value of the register on success, or an error message on failure.
    pub fn get(&self, reg: &Reg) -> Result<u32, String> {
        match reg {
            Reg::AX => Ok(self.ax), Reg::BX => Ok(self.bx), Reg::CX => Ok(self.cx), Reg::DX => Ok(self.dx),
            Reg::EX => Ok(self.ex), Reg::FX => Ok(self.fx), Reg::GX => Ok(self.gx), Reg::HX => Ok(self.hx),
            Reg::SP => Ok(self.sp), Reg::BP => Ok(self.bp), Reg::SI => Ok(self.si), Reg::DI => Ok(self.di),
            Reg::PC => Ok(self.pc), Reg::FLAGS => Ok(self.flags),
        }
    }
    
    /// Sets the 32-bit value of a specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - A reference to the `Reg` enum variant representing the target register.
    /// * `value` - The `u32` value to set the register to.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on success, or an error message on failure.
    /// Sets the 32-bit value of a specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - A reference to the `Reg` enum variant representing the target register.
    /// * `value` - The `u32` value to set the register to.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - `Ok(())` on success, or an error message on failure.
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
    
    /// Updates the CPU flags based on the result of an integer (u32) operation.
    ///
    /// This method sets the zero, sign, carry, and overflow flags according to
    /// standard integer arithmetic rules.
    ///
    /// # Arguments
    ///
    /// * `result` - The `u32` result of the operation.
    /// * `op1` - The first `u32` operand of the operation.
    /// * `op2` - The second `u32` operand of the operation.
    /// * `is_subtraction` - A boolean indicating if the operation was a subtraction.
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

    /// Updates the CPU flags based on the result of a floating-point (f32) operation.
    ///
    /// This method sets the zero and sign flags. Carry and overflow flags are
    /// typically not set for floating-point operations in the same way as integers.
    ///
    /// # Arguments
    ///
    /// * `result` - The `f32` result of the operation.
    pub fn update_flags_f32(&mut self, result: f32) {
        self.set_flag("zero", result == 0.0);
        self.set_flag("sign", result.is_sign_negative());
        self.set_flag("carry", false);
        self.set_flag("overflow", false);
    }
    
    /// Sets the value of a specific CPU flag.
    ///
    /// # Arguments
    ///
    /// * `flag_name` - The name of the flag to set (e.g., "zero", "carry").
    /// * `value` - The boolean value to set the flag to (`true` for set, `false` for clear).
    pub fn set_flag(&mut self, flag_name: &str, value: bool) {
        let bit_position = match flag_name.to_lowercase().as_str() {
            "carry" => 0,
            "zero" => 6,
            "sign" => 7,
            "interrupt" => 9,
            "string" => 10,
            "overflow" => 11,
            "macro" => 12,
            "stack_dir" => 13,
            _ => return,
        };
        
        if value {
            self.flags |= 1 << bit_position;
        } else {
            self.flags &= !(1 << bit_position);
        }
    }
    
    /// Retrieves the boolean value of a specific CPU flag.
    ///
    /// # Arguments
    ///
    /// * `flag_name` - The name of the flag to retrieve (e.g., "zero", "carry").
    ///
    /// # Returns
    ///
    /// * `Result<bool, String>` - The boolean value of the flag on success, or an error message if the flag name is unknown.
    pub fn get_flag(&self, flag_name: &str) -> Result<bool, String> {
        let bit_position = match flag_name.to_lowercase().as_str() {
            "carry" => 0,
            "zero" => 6,
            "sign" => 7,
            "interrupt" => 9,
            "string" => 10,
            "overflow" => 11,
            "macro" => 12,
            "stack_dir" => 13,
            _ => return Err(format!("Unknown flag: {}", flag_name)),
        };
        
        Ok((self.flags & (1 << bit_position)) != 0)
    }
}