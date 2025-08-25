pub struct ALU {
    pub instruction: &'static str,
    pub lifetime:u32, 
    pub gpf: bool,  
} 
impl ALU {
    pub fn new() -> Self {
        ALU {instruction: "NULL", lifetime:0, gpf: false}
    } 
}
