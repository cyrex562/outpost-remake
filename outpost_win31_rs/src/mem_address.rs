#[derive(Default,Debug,Clone)]
pub struct MemAddress {
    pub offset: u16,
    pub segment: u16,
}

impl MemAddress {
    pub fn dword(&self) -> u32 {
        (self.segment << 16 | self.offset) as u32
    }
}
