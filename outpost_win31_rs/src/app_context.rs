use std::collections::HashMap;
use crate::mem_address::MemAddress;
use crate::mem_container::MemContainer;

//noinspection ALL
#[derive(Default,Debug,Clone,Copy)]
pub struct AppContext {
    pub AH_REG: u8,
    pub CS_REG: u16,
    pub SI_REG: u16,
    pub DI_REG: u16,
    pub ES_REG: u16,
    pub BP_REG: u16,
    pub DS_REG: u16,
    // pub data_map: HashMap<MemAddress, MemContainer>,
    pub data_map: Vec<u16,Vec<u16,MemContainer>>,
    pub function_address_table: HashMap<MemAddress, fn(&mut AppContext)>

}
