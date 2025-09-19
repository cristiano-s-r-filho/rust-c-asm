// REGISTERS FOR THE REAL MODE EXECUTION - Implementation 
// General Purpose Registers, Segments, Offsets and a Flags 
// cpu/registers.rs
#[derive(Debug, Clone)]
pub struct Registers {
    // General purpose registers (32-bit)
    pub ax: u32,
    pub bx: u32,
    pub cx: u32,
    pub dx: u32,
    
    // Pointer registers (32-bit)
    pub sp: u32,  // Stack pointer
    pub bp: u32,  // Base pointer
    
    // Index registers (32-bit)
    pub si: u32,  // Source index
    pub di: u32,  // Destination index
    
    // Control registers (32-bit)
    pub pc: u32,  // Program counter
    pub flags: u32,  // Flags register
    
    // Segment registers (16-bit)
    pub cs: u16,  // Code segment
    pub ds: u16,  // Data segment
    pub ss: u16,  // Stack segment
    pub es: u16,  // Extra segment
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            sp: 0xFFFF,  // Start stack at top of memory
            bp: 0,
            si: 0,
            di: 0,
            pc: 0,
            flags: 0,
            cs: 0,
            ds: 0,
            ss: 0,
            es: 0,
        }
    }
    
    pub fn reset(&mut self) {
        *self = Self::new();
    }
    
    pub fn get(&self, reg_name: &str) -> Result<u32, String> {
        match reg_name.to_lowercase().as_str() {
            "ax" => Ok(self.ax),
            "bx" => Ok(self.bx),
            "cx" => Ok(self.cx),
            "dx" => Ok(self.dx),
            "sp" => Ok(self.sp),
            "bp" => Ok(self.bp),
            "si" => Ok(self.si),
            "di" => Ok(self.di),
            "pc" => Ok(self.pc),
            "flags" => Ok(self.flags),
            _ => Err(format!("Unknown register: {}", reg_name)),
        }
    }
    
    pub fn set(&mut self, reg_name: &str, value: u32) -> Result<(), String> {
        match reg_name.to_lowercase().as_str() {
            "ax" => { self.ax = value; Ok(()) },
            "bx" => { self.bx = value; Ok(()) },
            "cx" => { self.cx = value; Ok(()) },
            "dx" => { self.dx = value; Ok(()) },
            "sp" => { self.sp = value; Ok(()) },
            "bp" => { self.bp = value; Ok(()) },
            "si" => { self.si = value; Ok(()) },
            "di" => { self.di = value; Ok(()) },
            "pc" => { self.pc = value; Ok(()) },
            "flags" => { self.flags = value; Ok(()) },
            _ => Err(format!("Unknown register: {}", reg_name)),
        }
    }
    
    pub fn get_segment(&self, seg_name: &str) -> Result<u16, String> {
        match seg_name.to_lowercase().as_str() {
            "cs" => Ok(self.cs),
            "ds" => Ok(self.ds),
            "ss" => Ok(self.ss),
            "es" => Ok(self.es),
            _ => Err(format!("Unknown segment register: {}", seg_name)),
        }
    }
    
    pub fn set_segment(&mut self, seg_name: &str, value: u16) -> Result<(), String> {
        match seg_name.to_lowercase().as_str() {
            "cs" => { self.cs = value; Ok(()) },
            "ds" => { self.ds = value; Ok(()) },
            "ss" => { self.ss = value; Ok(()) },
            "es" => { self.es = value; Ok(()) },
            _ => Err(format!("Unknown segment register: {}", seg_name)),
        }
    }
    
    // Flag manipulation methods
    pub fn update_flags(&mut self, result: u32, op1: u32, op2: u32, is_subtraction: bool) {
        // Zero flag: set if result is zero
        self.set_flag("zero", result == 0);
        
        // Sign flag: set if result is negative (MSB set)
        self.set_flag("sign", (result as i32) < 0);
        
        // Carry flag: set if unsigned overflow occurred
        if is_subtraction {
            self.set_flag("carry", op1 < op2);
        } else {
            self.set_flag("carry", result < op1 || result < op2);
        }
        
        // Overflow flag: set if signed overflow occurred
        if is_subtraction {
            let op1_signed = op1 as i32;
            let op2_signed = op2 as i32;
            let result_signed = result as i32;
            self.set_flag("overflow", (op1_signed < 0 && op2_signed > 0 && result_signed > 0) ||
                                     (op1_signed > 0 && op2_signed < 0 && result_signed < 0));
        } else {
            let op1_signed = op1 as i32;
            let op2_signed = op2 as i32;
            let result_signed = result as i32;
            self.set_flag("overflow", (op1_signed > 0 && op2_signed > 0 && result_signed < 0) ||
                                     (op1_signed < 0 && op2_signed < 0 && result_signed > 0));
        }
        
        // Parity flag: set if number of set bits in lower byte is even
        let lower_byte = (result & 0xFF) as u8;
        self.set_flag("parity", lower_byte.count_ones() % 2 == 0);
        
        // Auxiliary carry flag: set if carry from bit 3 to bit 4
        let op1_low = op1 as u8;
        let op2_low = op2 as u8;
        let _result_low = result as u8;
        
        if is_subtraction {
            self.set_flag("auxiliary", (op1_low & 0x0F) < (op2_low & 0x0F));
        } else {
            self.set_flag("auxiliary", (op1_low & 0x0F) + (op2_low & 0x0F) > 0x0F);
        }
    }
    
    pub fn set_flag(&mut self, flag_name: &str, value: bool) {
        let bit_position = match flag_name.to_lowercase().as_str() {
            "carry" => 0,
            "parity" => 2,
            "auxiliary" => 4,
            "zero" => 6,
            "sign" => 7,
            "overflow" => 11,
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
            "parity" => 2,
            "auxiliary" => 4,
            "zero" => 6,
            "sign" => 7,
            "overflow" => 11,
            _ => return Err(format!("Unknown flag: {}", flag_name)),
        };
        
        Ok((self.flags & (1 << bit_position)) != 0)
    }
}