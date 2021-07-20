#![allow(non_snake_case)]

use crate::defines::U32Ptr;

pub fn CONCAT11(a: u8, b: u8) -> u16 {
    unimplemented!()
}

pub fn CARRY2(a: u16, b: u16) -> u32 {
    unimplemented!()
}

pub fn CONCAT22(a: u16, b: u16) -> u32 {
    unimplemented!()
}

pub fn SUB42(a: u16, b: u16) -> u32 {
    unimplemented!()
}

pub fn CONCAT13(a: u16, b: u32) -> u32 {
    unimplemented!()
}

pub fn CONCAT12(a: u8, b: u16) -> u32 {
    unimplemented!()
}

pub fn make_u16_ptr(address: u32) -> U32Ptr {
    unimplemented!()
}

pub fn make_u8_ptr(address: u32) -> U32Ptr {
    unimplemented!()
}

pub fn read_string_from_addr(address: u32) -> &mut String {
    unimplemented!()
}

pub fn ZEXT24(a: u16) -> u32 {
    unimplemented!()
}

pub fn read_vec_from_addr<T>(addr: u32) -> &mut Vec<T> {
    unimplemented!()
}

// pub fn struct_from_addr<T>(addr: u32) -> T {
//     unimplemented!()
// }

pub fn read_struct_from_addr<T>(addr: u32) -> &mut T {unimplemented!()}

// pub fn get_struct_ref_from_addr<T>(addr: u32) -> &T {unimplemented!()}

pub fn read_string_from_rsrc(rsrc_id: u16) -> String {
    unimplemented!()
}

pub fn CARRY4(a: u32, b: u32) -> u32 {
    unimplemented!()
}

pub fn SBORROW2(a: u32, b: u32) -> u32 { unimplemented!()}

pub fn SEXT24(a: u16) -> u32 {unimplemented!()}

pub fn address_of(a: u32) -> u32 {unimplemented!()}

pub fn write_string_to_addr(address: U32Ptr, src_string: &String) {unimplemented!()}

pub fn write_bool_to_addr(address: U32Ptr, value: bool) { unimplemented!()}
