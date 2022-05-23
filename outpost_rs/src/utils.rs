//
// Created by cyrex on 2/22/2022.
//

pub fn CONCAT22(a: u16, b: u16) -> u32 {
    (a << 16 | b) as u32
}

pub fn CONCAT11(a: u8, b: u8) -> u16 {
    (a << 8 | b) as u16
}

pub fn CONCAT13(a: u8, b: u32) -> u32 {
    let mut c = 0u32;
    (a << 24 | b) as u32
}

pub fn CONCAT12(a: u8, b: u16) -> u32 {
    u32 c = 0;
    c |= b;
    c | (a << 16)
}

pub fn SUB42(a: u32, b: u16) -> u32 {
    a - b
}

