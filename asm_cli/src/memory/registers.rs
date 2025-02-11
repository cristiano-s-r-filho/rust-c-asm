// REGISTERS FOR THE PROTECTED MODE EXECUTION - Implementation 
// General Registers 
pub struct MainRegisters {
    pub eax: u32, 
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32, 
}
impl MainRegisters {
    pub fn new() -> Self {
        MainRegisters {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
        }
    }
    pub fn write_to_register(&mut self, register:String, data:u32) -> &str {
        pub const ERR_MESSAGE: &str = "Stupid! Use a real register";
        pub const OK_MESSAGE: &str = "Everything OK! Data on the register";
        if register == "eax" {
            self.eax = data;  
            return OK_MESSAGE; 
        } else if register == "ebx" {
            self.ebx = data;
            return OK_MESSAGE;
        } else if register == "ecx" {
            self.ecx = data;
            return OK_MESSAGE;
        } else if register == "edx" {
            self.edx = data;
            return OK_MESSAGE;
        } else {
            return ERR_MESSAGE; 
        }
    }
    pub fn read_from_register(&mut self, register:String) -> u32 {
        if register == "eax" {
            return  self.eax;
        } else if register == "ebx" {
            return self.ebx;
        } else if register == "ecx" {
            return self.ecx;
        } else if register == "edx" {
            return self.edx;
        } else {
            return 0; 
        }
    }
    pub fn quick_start(&mut self, values_tuple: (u32,u32,u32,u32)){
        self.write_to_register(String::from("eax"), values_tuple.0);
        self.write_to_register(String::from("ebx"), values_tuple.1);
        self.write_to_register(String::from("ecx"), values_tuple.2);
        self.write_to_register(String::from("edx"), values_tuple.3);
    }
}
// Segment Selector Register
pub struct SegmentRegisters {
    pub cs: u32,
    pub ss: u32,
    pub ds: u32,
    pub es: u32,
    pub fs: u32,
    pub gs: u32,
}
impl SegmentRegisters {
    pub fn new() -> Self {
        SegmentRegisters {
            cs:0,
            ss:0,
            ds:0,
            es:0,
            fs:0,
            gs:0,
        }
    }
    pub fn write_to_register(&mut self, register: String, data: u32) {
        if register == "cs" {
            self.cs = data;
        } else if register == "ss" {
            self.ss = data;
        } else if register == "ds" {
            self.ds = data;
        } else if register == "es" {
            self.es = data;
        } else if register == "fs" {
            self.fs = data;
        } else if register == "gs" {
            self.gs = data; 
        } 
            
    }
    pub fn read_from_register(&mut self, register: String) -> u32 {
        if register == "cs" {
            return self.cs;
        } else if register == "ss" {
            return self.ss;
        } else if register == "ds" {
            return self.ds;
        } else if register == "es" {
            return self.es;
        } else if register == "fs" {
            return self.fs;
        } else if register == "gs" {
            return self.gs;
        } else {
            return 0; 
        }
    }
}
pub struct OffsetRegisters{
    pub eip: u32, 
    pub esp: u32,
    pub ebp: u32,
    pub edi: u32,
    pub esi: u32
}
impl OffsetRegisters {
    pub fn new() -> Self {
        OffsetRegisters {
            eip:0,
            esp:0,
            ebp:0,
            edi:0,
            esi:0,
        }
    }
    pub fn write_to_register(&mut self, register: String, data:u32) {
        if register == "eip" {
            self.eip = data;
        } else if register == "esp" {
            self.esp = data; 
        } else if register == "esi" {
            self.esi = data; 
        } else if register == "edi" {
            self.edi = data; 
        } else if register == "ebp" {
            self.ebp = data;
        }
    }
    pub fn read_from_register(&mut self, register: String) -> u32 {
        if register == "eip" {
            return self.eip
        } else if register == "esp" {
            return self.esp;
        } else if register == "ebp" {
            return self.ebp;
        } else if register == "edi" {
            return self.edi;
        } else if register == "esi" {
            return self.esi;
        } else {
            return 0; 
        } 
    } 
    pub fn increment_program_counter(&mut self){
        self.eip = &self.eip + 2;
    }
    pub fn decrease_stack_pointer(&mut self) {
        self.esp = &self.esp - 2; 
    } 
}    
#[derive(Clone,Copy)]
pub struct EFLAG { 
    pub ovfw:bool,
    pub zero:bool,
    pub negv:bool,  
}
impl EFLAG {
    pub fn new() -> Self {
        EFLAG { ovfw: false, zero: false, negv: false }
    }
    pub fn over_flow_test(&mut self) -> u8 {
        if self.ovfw == true {
            return 1; 
        } else {
            return 0; 
        }; 

    }
}