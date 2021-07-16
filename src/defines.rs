use crate::win_struct::{HMENU16, HICON16};

pub type U32Ptr = u32;

pub struct StructA {
    pub field_0x16: u16,
    pub field_0x36: u32,
    pub field_0x3e: u16,
    pub field_0x40: u16,
}

pub struct StructB {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0xf8: Struct18
}

impl StructA {
    pub fn new() -> StructA {
        StructA{ field_0x16: 0, field_0x36: 0, field_0x3e: 0, field_0x40: 0 }
    }
}

pub struct Struct18 {
    pub field_0x0: String,
    pub field_0x1: u16,
    pub field_0x2: U32Ptr,
    pub field_0x6: i16,
    pub field_0x8e: u16,
    pub field_0x92: U32Ptr,
    pub field_0x94: u16
}

pub struct Struct10 {

}

pub struct Struct27 {
 pub field_0x16: u16
}

pub struct Struct65 {
    
}

pub struct Struct11 {

}

pub struct Struct13 {

}

pub struct Struct28 {

}

pub struct Struct194 {
    pub field_0xe: u16,
}

pub struct Struct29 {
    pub field_0x0: u16,
    pub field_0x188: Struct18,
}

pub struct Struct57 {
    pub field_0x0: String,
    pub field_0x2: U32Ptr
    pub field_0x8e: Struct20,
    pub field_0x92: u32,
    pub field_0x96: u16,
    pub field_0x98: u16,

}

pub struct Struct19 {
    pub field_0x0: u16,
    pub field_0xa: i16,
    pub field_0xc: i16,
    pub field_0xe: u16,
    pub field_0x14: i16,
    pub field_0x16: i16,
    pub field_0x18: u16
}

pub struct Struct20 {

}

pub struct Struct76 {
    pub field_0x1e: u16,
    pub field_0x22: u16,
    pub field_0x24: u16,
    pub field_0x0: String,
    pub field_0x2: u16,
}

pub struct Struct79 {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x4: u16,
    pub field_0x8: u16,
    pub field_0xa: u16,
    pub field_0xc: u16,
    pub field_0xe: u16,
    pub field_0x12: u16,
    pub field_0x14: u16,
    pub field_0x16: u16
}

pub struct Struct80 {

}

pub struct Struct449 {
    pub field_0x0: u16,
    pub field_0x2: u16,
}

pub struct Struct648 {

}

pub struct Struct_1008_09ba {
    pub field_0xec: HMENU16,
}

pub struct Struct_1008_0a3c {
    pub field_0xde: bool
}

pub struct Struct_1008_496c {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x4: u16,
    pub field_0x6: u16,
    pub field_0x8: Struct18,
    pub field_0xc: u16,
    pub field_0x1a: Struct18
}

pub struct Struct_1008_49e8 {
    pub field_0x4: u16,
    pub field_0x8: u16,
    pub field_0xc: u16,
    pub field_0xe: u16,
    pub field_0x1a: u16,
    pub field_0x1c: u16,
    pub field_0x1e: u16,
    pub field_0x12: u16,
}

pub struct Struct_1008_4cdc {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x4: Struct18,
    pub field_0xe: Struct18,
    pub field_0x12: u16
}

pub struct Struct_1008_4d26 {
    pub field_0x2: u16,
    pub field_0x4: u16,
    pub field_0xc: u16,
}

pub struct Struct_1008_4d84 {

}

pub struct Struct_1000_05e2 {

}

pub struct Struct_1040_8b92 {
    pub field_0x90: u8,
    pub field_0x8e: HICON16
}

pub struct Struct17 {

}

pub struct Struct161 {

}

pub struct Struct37 {

}

pub struct Struct99 {

}

pub struct Struct110 {

}

pub struct Struct_1000_2cb0 {
    pub field_0x0: u16,
    pub field_0x1: u16,
    pub field_0x2: u16,
    pub field_0x3: Struct18,
    pub field_0x4: u16,
    pub field_0x5: u16,
    pub field_0xb: u16,
    pub field_0x78: u16
}


pub struct Struct_1000_34cf {
    pub field_0xe: Struct197,

}

pub struct Struct197 {
    pub field_0x2: u16
}

pub struct Struct_1000_09ca {
    pub field_0x0: Struct_211,
    pub field_0x1: u16,
    pub field_0x2: u16,
    pub field_0x7: U32Ptr,
    pub field_0x8: Struct_160,
    pub field_0x9: u16,
    pub field_0xa: u16,
}

pub struct Struct_211 {

}

pub struct Struct_160 {

}

pub struct Struct_1000_0c32 {
    pub field_0xe: u16,
}

pub struct Pointer<T> {

}

impl Pointer<T> {
    pub fn new<T>() -> Pointer<T> {
        Pointer::<T> {

        }
    }
}

pub struct Struct87 {
    pub field_0x680: i16,
    pub field_0x67c: u16,
    pub field_0x67e: Struct18
}

pub struct Struct_1010_2fa0 {

}

pub struct Struct_1008_628e {
    pub field_0x2: u16,
    pub field_0x8: u16,
    pub field_0xd2: Struct_257,
}

pub struct Struct_257 {
    pub field_0x14: U32Ptr
}

pub struct Struct_1010_7b26 {

}
