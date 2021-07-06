#![allow(non_snake_case)]

pub struct AppContext {
    pub PTR_LOOP_1050_5f7e: u32,
    pub PTR_LOOP_1050_1000: u32,
    pub PTR_LOOP_1050_5f80: u32,
    pub PTR_LOOP_1050_63fe: u32,
    pub PTR_LOOP_1050_5f84: u32,
    pub PTR_LOOP_1050_5f4c: u32,
    pub PTR_LOOP_1050_5f48: u32,
    pub PTR_LOOP_1050_5f4a: u32,
    pub PTR_LOOP_1050_5f4e: u32,
    pub PTR_LOOP_1050_5f50: u32,
    pub PTR_LOOP_1050_5f52: u32,
    pub data_seg: u16,
    pub _DAT_1050_5f82: u16,
    pub DAT_1050_5f87: u16,
    pub s_tile2_bmp_1050_1538: u32,
    pub _PTR_LOOP_1050_65e2: u32
}

impl AppContext {
    pub fn new() -> AppContext {
        AppContext{
            PTR_LOOP_1050_5f7e: 0,
            PTR_LOOP_1050_1000: 0,
            PTR_LOOP_1050_5f80: 0,
            PTR_LOOP_1050_63fe: 0,
            PTR_LOOP_1050_5f84: 0,
            PTR_LOOP_1050_5f4c: 0,
            data_seg: 0x1050,
            _DAT_1050_5f82: 0,
            DAT_1050_5f87: 0,
            PTR_LOOP_1050_5f48: (),
            PTR_LOOP_1050_5f4a: (),
            PTR_LOOP_1050_5f4e: (),
            PTR_LOOP_1050_5f50: (),
            PTR_LOOP_1050_5f52: (),
            s_tile2_bmp_1050_1538: (),
            _PTR_LOOP_1050_65e2: (),
        }
    }
}
