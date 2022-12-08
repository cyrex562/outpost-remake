//
// Created by cyrex on 2022-05-22.
//

// #include "utils.h"

pub fn CONCAT22(a: u16, b: u16) -> u32
{
    // c: u32 = 0;
    // c = a << 16 | b;
    // return c;
    (a << 16 | b) as u32
}

pub unsafe fn CONCAT11(u8 a, u8 b) -> u16 {
    return a << 8 | b;
}

pub unsafe fn CARRY2(a: u16, b: u16) -> bool {
    // TODO: implement CARRY2 op
    return false;
}

pub fn SUB42(a: u32, b: u32) -> u32 {
    return a - b;
}
