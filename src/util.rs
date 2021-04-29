pub fn CONCAT22(a: u16, b: u16) -> u32 {
    (a << 16) | b
}

pub fn CONCAT11(a: u8, b: u8) -> u16 {
    (a << 8) | b
}

pub fn CONCAT12(a: u8, b: u16) -> u32 {
    (a << 16) | b
}
pub fn CONCAT21(a: u16, b: u8) -> u32 {
    todo!();
    0
}

pub fn CONCAT31(a: u32, b: u8) -> u32 {
    todo!();
    0
}

pub fn CARRY1(a: u8, b: u8) -> u16 {
    todo!();
    0
}

pub fn SUB21(a: u16, b: u8) -> u16 {
    todo!();
    0
}
pub fn SCARRY1(a: i8, b: i8) -> i16 {
    todo!();
    0
}

pub fn SCARRY2(a: u16, b: u16) -> u32 {
    todo!();
    0
}

pub fn ZEXT24(a: u16) -> u32 {
    todo!();
    0
}

pub fn CARRY2(a: u16, b: u16) -> u32 {
    todo!();
    0
}

pub fn POPCOUNT(a: u16) -> u16 {
    todo!();
    0
}

pub fn LocalDescriptorTableRegister() -> u16 {
    todo!();
    0
}

pub fn SBORROW1(a: u16, b: u16) -> u32 {
    todo!();
    0
}

pub fn SBORROW2(a: u16, b: u16) -> u32 {
    todo!();
    0
}
