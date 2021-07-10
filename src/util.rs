#![allow(non_snake_case)]

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

pub fn make_u16_ptr(address: u32) -> *mut u16 {
    unimplemented!()
}

pub fn make_u8_ptr(address: u32) -> *mut u8 {
    unimplemented!()
}

pub fn get_string_at_addr(address: u32) -> String {
    unimplemented!()
}

pub fn ZEXT24(a: u16) -> u32 {
    unimplemented!()
}

pub fn vec_from_addr<T>(addr: u32) -> Vec<T> {
    unimplemented!()
}

pub fn struct_from_addr<T>(addr: u32) -> T {
    unimplemented!()
}

pub fn get_mut_struct_ref_from_addr<T>(addr: u32) -> &mut T {unimplemented!()}

pub fn get_struct_ref_from_addr<T>(addr: u32) -> &T {unimplemented!()}
