//
#![allow(non_upper_case_globals)]
// Created by cyrex on 2022-05-22.
//

// #ifndef OUTPOST_C_PROJ__GLOBALS_H_
// #define OUTPOST_C_PROJ__GLOBALS_H_

use std::os::raw::c_char;
use std::ptr;
// #include "types.h"
// #include "func_ptrs.h"
use crate::prog_types::{HINSTANCE16};

pub static mut HINSTANCE16_1050_5f4c: HINSTANCE16 = 0;
pub static mut u16_1050_0008: Option<code> = None;
pub static mut PTR_LOOP_1050_000a: *mut u8 = ptr::null_mut();
pub static mut PTR_LOOP_1050_000c: u16 = 0;
pub static mut PTR_LOOP_1050_5f26: u16 = 0;
pub static mut PTR_LOOP_1050_5f4a: u16 = 0;
pub static mut PTR_LOOP_1050_5f50: u16 = 0;
pub static mut PTR_LOOP_1050_5f7e: u16 = 0;
pub static mut WIN_VERSION_1050_5f80: u32 = 0;
pub static mut DAT_1050_5f82: u16 = 0;
pub static mut PTR_LOOP_1050_5f84: u32 = 0;
pub static mut DAT_1050_5f87: u16 = 0;
pub static mut u16_1050_0002: u16 = 0;
pub static mut u32_1050_0004: u16 = 0;
pub static mut u32_1050_0006: u16 = 0;
pub static mut DAT_1050_5f30: u8 = 0;
pub static mut PTR_LOOP_1050_000e: *mut u8 = ptr::null_mut();
pub static mut PTR_LOOP_1050_5f48: *mut u8 = ptr::null_mut();
pub static mut PTR_LOOP_1050_5f4e: *mut u8 = ptr::null_mut();
pub static mut PTR_LOOP_1050_63fe: *mut c_char = ptr::null_mut();
pub static mut DAT_1050_422c: u16 = 0;
pub static mut DAT_1050_4216: u16 = 0;


pub static mut REG_BP: u16 = 0;
pub static mut REG_CS: u16 = 0;
pub static mut REG_DS: u16 = 0;
pub static mut REG_SS: u16 = 0;
pub static mut REG_AF: u8 = 0;
pub static mut REG_SI: u16 = 0;
pub static mut REG_DI: u16 = 0;
pub static mut REG_AH: u8 = 0;



// #endif //OUTPOST_C_PROJ__GLOBALS_H_
