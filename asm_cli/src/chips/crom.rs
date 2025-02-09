// Everything needed for the tables. 
// Gettin responses from terminal.S
pub fn get_response() -> String {
    let mut input: String = String::new(); 
    std::io::stdin().read_line(&mut input).expect("ERR: Cannot read terminal input");
    return input;
    
}

// SIMULATING DTables BELLOW: 
pub const  MAX_TABLE_SIZE: usize = 0x2000 as usize;
#[derive(Clone, Copy,Debug)]
pub enum AcessLevel {
    KERNEL,
    SYSTEMCALL,
    SHELL,
    USER
}
#[derive(Clone,Copy)]
pub struct DTEntry {
    pub selector:&'static str,
    pub base: u32,
    pub limit: u32,
    pub acess_level: AcessLevel,
}  
pub struct DTable {
    pub name: &'static str,
    pub content: Vec<DTEntry>,
    pub capacity: u16,
}

impl DTable {
    pub fn new(table_name:&'static str) -> DTable {
        let d_table = DTable {
            name:table_name,
            content: vec![DTEntry{
                selector: "NULL",
                base: 0, 
                limit: 0, 
                acess_level: AcessLevel::USER,
            }; MAX_TABLE_SIZE],
            capacity: 0, 
        };
        return d_table;
    }
}

pub fn generate_gdt() -> DTable {
    let mut gd_table : DTable = DTable::new("GLOBAL_D_TABLE");
    gd_table.content[0] = DTEntry {
        selector:"CS",
        base:0,
        limit:u16::MAX as u32, 
        acess_level: AcessLevel::KERNEL
    };
    return gd_table;  
}

pub fn generate_idt() -> DTable {
    let id_table: DTable = DTable::new("INTERRUPT_D_TABLE");
    return id_table;
}

pub fn generate_ldt() -> DTable {
    let ld_table: DTable = DTable::new("LOCAL_D_TABLE");
    return ld_table;
} 

