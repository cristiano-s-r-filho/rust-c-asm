use crate::memory::registers::EFLAG;
#[derive(Clone, Copy)] 
pub struct MMU {
    data_bus: u32, 
    adress_bus: u32  
}
impl MMU {
    pub fn new() -> Self {
        MMU {
            data_bus: 0x0,
            adress_bus: 0x0 
        }
    }
    pub fn foward_to_data_bus(&mut self, dt: u32){
        self.data_bus = dt;
    }
    pub fn get_from_data_bus(&mut self) -> u32 {
        return self.data_bus;
    }
    pub fn forward_to_adress_bus(&mut self, adrr: usize) {
        self.adress_bus = adrr as u32; 
    }
    pub fn get_from_adress_bus(&mut self) -> u32 {
        return self.adress_bus; 
    }

    pub fn fisical_adress(&mut self, base_adress:u32 , segment_limit: usize, offset: u32, flag: EFLAG ) -> u32 {
        let base_adrr:usize = base_adress as usize;
        let mut flag = flag;   
        let fisc_adrr = base_adrr + (offset as usize);
    
        if fisc_adrr > base_adrr + segment_limit {
            flag.set_true("ovfw");
        } else {
            return 0; 
        }

        return fisc_adrr as u32;
    }
}