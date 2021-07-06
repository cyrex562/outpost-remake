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

}

impl Struct18 {

}

pub struct Struct27 {

}

pub struct Struct65 {
    
}