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

pub fn CONCAT13(a: u8, b: u32) -> u32 {
    unimplemented!();
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

pub fn SUB42(a: u32, b: u16) -> u32 {
    todo!();
    0
}

pub fn SUB41(a: u32, b: u8) -> u32 {
    unimplemented!();
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

pub fn CONCAT210(local_3a: u16, u_stack68: ()) -> () {
    todo!()
}

pub fn CONCAT212(local_38: u16, u_stack68: ()) -> () {
    todo!()
}

pub fn CONCAT214(local_36: u16, u_stack68: ()) -> () {
    todo!()
}

pub fn CONCAT24(b_result_2: u16, arg: i32) -> () {
    todo!()
}



pub fn CONCAT26(local_4e: u16, b_result_2: ()) -> () {
    todo!()
}

pub fn CONCAT28(local_4c: u16, b_result_2: ()) -> () {
    todo!()
}

pub fn CONCAT610(u_stack74: (), b_result_2: ()) -> () {
    todo!()
}

pub(crate) fn CONCAT66(u_stack34: (), u_var1: ()) -> () {
    todo!()
}