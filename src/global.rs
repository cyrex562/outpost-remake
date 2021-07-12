#![allow(non_snake_case)]

use crate::defines::U32Ptr;

pub struct AppContext {
    pub PTR_LOOP_1050_5f7e: U32Ptr,
    pub PTR_LOOP_1050_1000: U32Ptr,
    pub PTR_LOOP_1050_5f80: U32Ptr,
    pub PTR_LOOP_1050_63fe: U32Ptr,
    pub PTR_LOOP_1050_5f84: U32Ptr,
    pub PTR_LOOP_1050_5f4c: U32Ptr,
    pub PTR_LOOP_1050_5f48: U32Ptr,
    pub PTR_LOOP_1050_5f4a: U32Ptr,
    pub PTR_LOOP_1050_5f4e: U32Ptr,
    pub PTR_LOOP_1050_5f50: U32Ptr,
    pub PTR_LOOP_1050_5f52: U32Ptr,
    pub data_seg: u16,
    pub _DAT_1050_5f82: u16,
    pub DAT_1050_5f87: u16,
    pub s_tile2_bmp_1050_1538: U32Ptr,
    pub _PTR_LOOP_1050_65e2: U32Ptr,
    pub _PTR_LOOP_1050_0368: U32Ptr,
    pub PTR__LOOP_1050_0368: U32Ptr,
    pub PTR__LOOP_1050_0ed0: U32Ptr,
    pub PTR__LOOP_1050_14cc: U32Ptr,
    pub PTR_LOOP_1050_038c: U32Ptr,
    pub PTR__LOOP_1050_5748: U32Ptr,
    pub USHORT_1050_1028: u16,
    pub PTR_LOOP_1050_0310: U32Ptr

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
            PTR_LOOP_1050_5f48: 0,
            PTR_LOOP_1050_5f4a: 0,
            PTR_LOOP_1050_5f4e: 0,
            PTR_LOOP_1050_5f50: 0,
            PTR_LOOP_1050_5f52: 0,
            s_tile2_bmp_1050_1538: 0,
            _PTR_LOOP_1050_65e2: 0,
            _PTR_LOOP_1050_0368: 0,
            PTR__LOOP_1050_0368: 0,
            PTR__LOOP_1050_0ed0: 0,
            PTR__LOOP_1050_14cc: 0,
            PTR_LOOP_1050_038c: 0,
            PTR__LOOP_1050_5748: 0,
            USHORT_1050_1028: 0,
            PTR_LOOP_1050_0310: 0
        }
    }
}
