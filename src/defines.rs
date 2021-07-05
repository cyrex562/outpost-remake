pub struct StructA {
    pub field_0x16: u16,
    pub field_0x36: u32,
    pub field_0x3e: u16,
    pub field_0x40: u16,
}

impl StructA {
    pub fn new() -> StructA {
        StructA{ field_0x16: 0, field_0x36: 0, field_0x3e: 0, field_0x40: 0 }
    }
}