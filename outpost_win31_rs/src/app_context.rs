use std::collections::HashMap;
use std::hash::Hash;
use crate::mem_address::MemAddress;
use crate::mem_container::MemContainer;
use crate::windef16::HINSTANCE16;

#[allow(non_snake_case)]
#[derive(Default, Debug,Clone)]
pub struct Globals {
    pub PTR_LOOP_1050_000c: u16,
    pub HINSTANCE16_1050_5f4c: HINSTANCE16,
    pub DAT_1050_5f82: u32,
    pub DAT_1050_5f87: u16,
    pub PTR_LOOP_1050_5f84: u32,
    pub WIN_VERSION_1050_5f80: u32,
    pub PTR_LOOP_1050_5f4e: u32,
    pub PTR_LOOP_1050_5f4a: u16,
    pub PTR_LOOP_1050_5f48: u16,
    pub PTR_LOOP_1050_5f50: u16,
    pub u16_1050_02a0: u16,
    pub u16_1050_5bc8: u16,
    pub PTR_LOOP_1050_1040: u16,
}

//noinspection ALL
#[derive(Default,Debug,Clone,Copy)]
pub struct AppContext {
    pub globals: Globals,
    pub AH_REG: u8,
    pub AL_REG: u8,
    pub AX_REG: u16,
    pub BH_REG: u8,
    pub BL_REG: u8,
    pub BX_REG: u16,
    pub CH_REG: u8,
    pub CL_REG: u8,
    pub CX_REG: u16,
    pub DH_REG: u8,
    pub DL_REG: u8,
    pub DX_REG: u16,
    pub CS_REG: u16,
    pub SI_REG: u16,
    pub DI_REG: u16,
    pub ES_REG: u16,
    pub BP_REG: u16,
    pub DS_REG: u16,
    pub SS_REG: u16,

    // pub data_map: HashMap<MemAddress, MemContainer>,
    // pub data_map: Vec<u16,Vec<u16,MemContainer>>,
    pub data_map: HashMap<u16, HashMap<u16, MemContainer>>;
    pub function_address_table: HashMap<MemAddress, fn(&mut AppContext)>

}
