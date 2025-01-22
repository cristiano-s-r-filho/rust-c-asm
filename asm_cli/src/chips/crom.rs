use inline_colorization::*;
use colored::Colorize;

// IMPLEMENTATION OF A SIMULATION OF VIRTUAL CRegisters, Basic Tables AND ROM - NOT actual virtual memory
// PLEASE DO NOT TRY TO USE IT AS SUCH -> IT'S ALL A SIMULATION.
pub struct CRegisters {
    pub cr0:[bool; 32],
    pub cr1:u32,
    pub cr2:u32, 
    pub cr3:u32,
    pub cr4:[bool; 32],
    pub cr5:u32,
    pub cr6:u32,
    pub cr7:u32     
}
// Observer for a specific CRegister.
pub const MAX_EVENTS_LOGS: usize = (0x400) as usize;
pub struct  CRObserver {
    name:   &'static str,
    status: CRSTATUS,
    resume: CRCONTENT,
    uptime: u32, 
    logs: Logs
}
struct Logs {
    logs:Vec<(&'static str, CRSTATUS, u32,bool)>,
    current: u16
}
impl Logs {
    pub fn init_new() -> Self {
        Logs {
            logs: vec![("NULL",CRSTATUS::BORN,0,false); MAX_EVENTS_LOGS],
            current: 0,  
        }
    }
    pub fn prim_log(&mut self) {
        self.logs[0] = ("PRIM",CRSTATUS::BORN,0,false);
        self.current += 1;   
    }
    pub fn search_log(&mut self, log_id: usize) -> ((&'static str, CRSTATUS, u32, bool),u32) {
        // Linear search for now seems enough...
        if log_id != 1 && log_id != MAX_EVENTS_LOGS {
            if self.logs[log_id] != ("NULL",CRSTATUS::BORN,0,false) {
                // We only clone the value here because tuples don't implement copy trait.
                // However we may change into a structure based comunication format... 
                return (self.logs[log_id].clone(),(log_id as u32) + 1); 
            } else {
                return (("NULL",CRSTATUS::BORN,0,false),0); 
            }
        } else {
            return (self.logs[0].clone(),0);
        }
    } 
    pub fn update_logs(&mut self, log: (&'static str, CRSTATUS, u32, bool)) {
        let currents_logs = self.search_log(self.current as usize);
        let next = (currents_logs.1) as usize; 
        self.logs[next] = log;
    } 

}
// Gettin responses from terminal.S
pub fn get_response() -> bool {
    let mut input: String = String::new(); 
    std::io::stdin().read_line(&mut input).expect("ERR: Cannot read terminal input"); 
    let response:bool; 
    if &input == &String::from("Y") {
        response = true;
    } else if &input == &String::from("N") {
        response = false; 
    } else {
        response = true;  
    } 
    return  response;
    
}

impl CRObserver {
    pub fn inc_uptime(&mut self) {
        self.uptime += 1;  
    }
    pub fn forward_status(&mut self) {
       let contact = &self.resume; 
       match contact {
            CRCONTENT::BORN => self.status = CRSTATUS::BORN,
            CRCONTENT::CONTROLBIT(_) => self.status = CRSTATUS::DECIDING, 
            CRCONTENT::ADRESS(_) => self.status = CRSTATUS::UP  
       } 
    }
    pub fn get_status(&mut self) -> (CRSTATUS,u32,bool) {
        let status = &self.status;
        let uptime = self.uptime;
        let has_stopped:bool; 
        if self.status == CRSTATUS::DECIDING {
            has_stopped = true; 
        } else  {
            has_stopped = false;
        }
        return (*status,uptime,has_stopped); 
    }
    // EVENT FUNCTION - PERIODIC STATUS
    // WARNING! THIS is a TEST! PLEASE DON'T USE IT for anything yet.   
    pub fn event_ctrl_status_sender(&mut self) -> ((&'static str,CRSTATUS, u32, bool),u16) {
        let event_name = self.name; 
        let mut ctrl_id:u16 = 
        self.uptime as u16; 
        ctrl_id += 1; 
        let response = (event_name, self.get_status().0, self.get_status().1, self.get_status().2); 
        return (response,ctrl_id);
    }
    pub fn initiate_events(&mut self, cycle_limit:u16) {
        // initate all of the powers at be.
        let mut counter:u16 = 0;
        let mut pause: bool = false;
        let event_result = self.event_ctrl_status_sender().0;
        let current_log = self.logs.current as usize; 
            let process_log = self.logs.logs[current_log]; 
            let log_name = process_log.0; 
            let log_status = process_log.1;
            let log_number = process_log.2; 
            let log_state = if process_log.3 == false {"UP"} else {"DOWN"};
        while (counter != cycle_limit) && (pause == false){
            self.logs.update_logs(event_result);
            counter += 1;
            println!("{}","CR STATUS LOG: ".cyan().bold());
            println!("{color_cyan}NAME:{color_reset}   {}",log_name); 
            println!("{color_cyan}STATUS:{color_reset} {:?}",log_status);
            println!("{color_cyan}UPTIME:{color_reset} {}",log_number);
            println!("{color_cyan}STATE:{color_reset}  {}",log_state);
            println!("Continue with log display? (Y) | (N)"); 
            pause = get_response();
        } 
        println!("{style_bold}{color_cyan}+-------------------- LOG ENTRANCE - {color_reset} {color_white}CR:{}{color_reset} {color_cyan}------------------+ {color_reset} {style_reset}",self.name);
        let mut log_id:u16 = 0;
        for item in &mut self.logs.logs[0..20]{
            println!("{style_bold}{color_cyan}LOG-ID:{color_reset}{style_reset} {}", log_id);
            println!("{color_cyan}NAME:{color_reset}   {}",item.0); 
            println!("{color_cyan}STATUS:{color_reset} {:?}",item.1);
            println!("{color_cyan}UPTIME:{color_reset} {}",item.2);
            println!("{color_cyan}STATE:{color_reset}  {}",item.3);
            println!("{}", "--------- end ---------".cyan());
            log_id += 1;
        }   
    }
}
#[derive(Clone,PartialEq,PartialOrd,Copy,Debug)]
pub enum CRSTATUS {
    BORN,
    UP,
    DOWN,
    DECIDING,
}
pub enum CRCONTENT {
    BORN,
    ADRESS(u32),
    CONTROLBIT([bool;32])
}

impl CRegisters {
    pub fn new() -> CRegisters {
        let new_cr = CRegisters {
            cr0:[false;32],
            cr1:0,
            cr2:0,
            cr3:0,
            cr4:[false;32],
            cr5:0,
            cr6:0,
            cr7:0,
        };
        return new_cr; 
    }

    pub fn generate_cr_observer(&mut self, code: u8) -> CRObserver {
        let mut cr_observer = CRObserver {
            name: "null",
            status: CRSTATUS::BORN,
            resume: CRCONTENT::BORN,
            uptime:0,
            logs:Logs::init_new()     
        };
        if code == 0 {
            cr_observer.name = "CR0";
            cr_observer.status = CRSTATUS::UP; 
            cr_observer.resume = CRCONTENT::CONTROLBIT(self.cr0);      
        } else if code == 1 {
            cr_observer.name = "CR1"; 
            cr_observer.status = CRSTATUS::DOWN; 
            cr_observer.resume = CRCONTENT::ADRESS(self.cr1); 
        } else if code == 2 {
            cr_observer.name = "CR2";
            cr_observer.resume = CRCONTENT::ADRESS(self.cr2); 
        } else if code == 3 {
            cr_observer.name = "CR3";
            cr_observer.resume = CRCONTENT::ADRESS(self.cr3);
        } else if code == 4 {
            cr_observer.name = "CR4"; 
            cr_observer.status = CRSTATUS::UP;
            cr_observer.resume = CRCONTENT::CONTROLBIT(self.cr4);   
        } else {
            cr_observer.name = "UNAUTH";
            cr_observer.status = CRSTATUS::UP;
            if code == 5 {
                cr_observer.resume = CRCONTENT::ADRESS(self.cr5);
            } else if code == 6 {
                cr_observer.resume =  CRCONTENT::ADRESS(self.cr6);
            } else {
                cr_observer.resume = CRCONTENT::ADRESS(self.cr7); 
            } 
        }
        cr_observer.logs.prim_log();
        cr_observer.uptime += 1;
        return cr_observer; 
    }

    pub fn cregisters_quick_start() -> (CRegisters,(CRObserver,CRObserver,CRObserver)) {
        let mut c_registers = CRegisters::new(); 
        let mut cr0_observer = c_registers.generate_cr_observer(0);
        let mut cr1_observer = c_registers.generate_cr_observer(1);
        let mut cr4_observer = c_registers.generate_cr_observer(4);
        cr0_observer.initiate_events(10);
        cr1_observer.initiate_events(10);
        cr4_observer.initiate_events(10);
        let observer_block = (cr0_observer,cr1_observer,cr4_observer); 
        return (c_registers, observer_block);
    }
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

#[derive(PartialEq,Clone)]
pub struct Page {
    process_id: u32,
    real_adress: u32,
    virtual_adress: u32,
    control_bit: bool, 
    content: Vec<u32>,
}

pub const PAGE_SIZE: usize = 0x1000 as usize;
#[derive(Clone)]
pub struct PageTable {
    content: Vec<Page>,
    capacity: u32,  
}

impl Page {
    pub fn new(process:u32,ram_adress:u32, rom_adress:u32, read_only:bool, data: Vec<u32>) -> Self{
        Page {
            process_id: process,
            real_adress: ram_adress,
            virtual_adress: rom_adress, 
            control_bit: read_only,
            content: data,
        }
    }
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            content: vec![Page::new(0,0,0,false, vec![0;PAGE_SIZE]);1],
            capacity: 1
        }
    }
    pub fn search_page(&mut self, proc:u32, ram_adr: u32, rom_adr: u32, c_bit:bool) -> (Page, u32) {
        let mut adrr = 0;
        let mut mark: Page = Page::new(proc, ram_adr, rom_adr, c_bit, vec![0;1]); 
        let reference = proc; 
        for _i in &self.content[0..self.content.len()]{
            if self.content[adrr].process_id == reference {
                if self.content[adrr].real_adress == ram_adr {
                    if self.content[adrr].virtual_adress == rom_adr {
                        mark.content = self.content[adrr].content.clone();
                        break;
                    }    
                }
            } else {
                adrr += 1;
            }
        }
        return (mark,adrr as u32);
    }
    pub fn add_page(&mut self,proc_id: u32, ram_adrr: u32, rom_adrr: u32, ctrl_bit:bool, stuff: Vec<u32>) -> bool{
        let new_page = Page::new(proc_id, ram_adrr, rom_adrr, ctrl_bit, stuff);
        let search_page = self.search_page(proc_id, ram_adrr, rom_adrr, ctrl_bit);
        if new_page == search_page.0 {
            return false; 
        } else {
            let add_adrr = (search_page.1 + 1) as usize; 
            self.content[add_adrr] = new_page;
            return true; 
        } 
    }
    pub fn rmv_page(&mut self, proc_id: u32, ram_adrr: u32, rom_adrr: u32, ctrl_bit:bool,) -> bool {
        let entry_to_rmv = self.search_page(proc_id, ram_adrr, rom_adrr, ctrl_bit);
        if entry_to_rmv.0 == self.content[entry_to_rmv.1 as usize] {
            self.content[entry_to_rmv.1 as usize] = Page::new(0, 0, 0, true, vec![0;1]);
            self.capacity -= 1; 
            return true; 
        } else {
            return false; 
        }
    }
}

pub const MAX_ROM_SIZE: usize = (u32::MAX/32) as usize;
pub struct EEPROM{
    acess: bool,
    cells: Vec<u32>, 
}
impl EEPROM {
    pub fn new() -> Self {
        EEPROM { acess: true, cells: vec![0;MAX_ROM_SIZE] }
    }

    pub fn read_from_rom(&mut self, adress: u32) -> u32 {
        if self.acess == true {
            let resolve = self.cells[adress as usize];
            return resolve 
        } else {
            return 0; 
        }
    }

    pub fn write_to_rom(&mut self, adress: u32, data: u32) {
        if self.acess == true {
            self.cells[adress as usize] = data;
        }
    }

    pub fn change_acess(&mut self) {
        if self.acess == false {
            self.acess = true; 
        } else {
            self.acess = false; 
        }
    }
}