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
pub const MAX_EVENTS_LOGS: u16 = (u16::MAX);
pub struct  CRObserver {
    name:   &'static str,
    status: CRSTATUS,
    resume: CRCONTENT,
    uptime: u32, 
    logs: Logs
}
struct Logs {
    logs: vec![(&'static str,(CRSTATUS, u32, bool),u32);MAX_EVENTS_LOGS],
    current: u16
}
impl Logs {
    pub fn init_new() -> Self {
        Logs {
            logs: vec![0; MAX_EVENTS_LOGS],
            current: 0,  
        }
    }
    pub fn prim_log(&mut self) {
        self.logs[0] = ("PRIM",(CRSTATUS::BORN,0,false), 1);
        self.current += 1;   
    }
    pub fn search_log(&mut self, log_id: u32) -> ((&'static str, CRSTATUS, u32, bool),u32) {
        // Linear search for now seems enough...
        if (log_id != 1 && log_id != MAX_EVENTS_LOGS) {
            if self.logs[log_id] != () {
                return (self.logs[log_id],log_id+1); 
            } else {
                return ((),0); 
            }
        } else {
            return (self.logs[0],0);
        }
    } 
    pub fn update_logs(&mut self, log: (&'static str, CRSTATUS, u32, bool)) {
        let currents_logs = self.search_log(self.current);
        let next = currents_logs.1; 
        self.logs[next] = log;
    } 

}

impl CRObserver {
    fn inc_uptime(&mut self) {
        self.uptime += 1;  
    }
    fn forward_status(&mut self) {
       match CRCONTENT {
            CRCONTENT::BORN => self.status = CRSTATUS::BORN,
            CRCONTENT::CONTROLBIT([0,32]) => self.status = CRSTATUS::DECIDING, 
            CRCONTENT::ADRESS(_) => self.status = CRSTATUS::UP  
       } 
    }
    pub fn get_status(&mut self) -> (CRSTATUS,u32,bool) {
        let status = self.status;
        let uptime = self.uptime;
        let has_stopped:bool; 
        if self.status == CRSTATUS::DECIDING {
            has_stopped = true; 
        } else  {
            has_stopped = false;
        }
        return (status,uptime,has_stopped); 
    }
    // EVENT FUNCTION - PERIODIC STATUS
    // WARNING! THIS is a TEST! PLEASE DON'T USE IT for anything yet.   
    pub fn event_ctrl_status_sender(&mut self, period: u32, stop: bool) -> (&'static str,(CRSTATUS, u32, bool),u16) {
        let event_name = "STATUS_REPORT"; 
        let ctrl_id:u16; 
        for i in (0..period) {
            if (stop == false) {
                ctrl_id += 1; 
                return (event_name,self.get_status(),ctrl_id);
            } else {
                break;
            }
        }
    }

    pub fn initiate_events(&mut self, cycle_limit:u16, pause:bool) {
        // initate all of the powers at be.
        let mut counter:u16 = 0;
        while counter != cycle_limit && stop = false{
            self.logs.update_logs(self.event_ctrl_status_sender(cycle_limit, pause));
            counter += 1  
        }    
    }
}
enum CRSTATUS {
    BORN,
    UP,
    DOWN,
    DECIDING,
}
enum CRCONTENT {
    BORN,
    ADRESS(u32),
    CONTROLBIT([bool;32])
}

impl CRegisters {
    pub fn new() -> CRegisters {
        let new_cr = CRegisters {
            cr0:[0;32],
            cr1:0,
            cr2:0,
            cr3:[0;32],
            cr4:[0;32],
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
        cr_observer.uptime += 1;
        return cr_observer; 
    }

    pub fn cregisters_quick_start() -> (CRegisters,(CRObserver,CRObserver,CRObserver)) {
        let mut c_registers = CRegisters::new(); 
        let mut cr0_observer = c_registers.generate_cr_observer(0);
        let mut cr1_observer = c_registers.generate_cr_observer(1);
        let mut cr4_observer = c_registers.generate_cr_observer(4);
        let observer_block = (cr0_observer,cr1_observer,cr4_observer); 
        return (c_registers, observer_block);
    }
}

// SIMULATING DTables BELLOW: 
pub const  MAX_TABLE_SIZE: u16 = 0x2000;
enum AcessLeveL {
    KERNEL,
    SYSTEMCALL,
    SHELL,
    USER
}
struct DTEntry {
    selector:&'static str,
    base: u32,
    limit: u32,
    acess_level: AcessLeveL,
}  
pub struct DTable {
    name: &'static str,
    content: vec![DTEntry,MAX_FLASH_SIZE],
    capacity: u16,
}

impl DTable {
    pub fn new(table_name:&'static str) -> DTable {
        let mut d_table = DTable {
            name:table_name,
            content: vec![],
            capacity: 0, 
        };
    }
}

fn generate_gdt() -> DTable {
    let mut gd_table : DTable = DTable::new("GLOBAL_D_TABLE");
    gd_table.content[0] = DTEntry {
        selector:"CS",
        base:0,
        limit:u16::MAX as u32, 
        acess_level: AcessLeveL::KERNEL
    };
    return gd_table;  
}

fn generate_idt() -> DTable {
    let mut id_table: DTable = DTable::new("INTERRUPT_D_TABLE");
    return id_table;
}

pub fn generate_ldt() -> DTable {
    let mut ld_table: DTable = DTable::new("LOCAL_D_TABLE");
    return ld_table;
} 