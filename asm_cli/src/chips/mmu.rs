use crate::{chips::crom::*, comence_observations}; 

pub struct MMU {
    controlers: CRegisters, 
    observers: Vec<CRObserver>,
    rom: EEPROM, 
    data_bus: u32, 
    acess_bus: u32,
    control_bus: &'static str  
}
impl MMU {
    pub fn new() -> Self {
        let spawn_observers = comence_observations();
        MMU { 
            controlers: spawn_observers.0.0,
            observers: vec![spawn_observers.0.1.0,spawn_observers.0.1.1, spawn_observers.0.1.2],
            rom: EEPROM::new(),
            data_bus: 0x0,
            acess_bus: 0x0,
            control_bus: "null"  
        }
    }
}