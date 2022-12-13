use crate::structs::struct_99::astruct_99;

#[derive(Default,Debug,Clone)]
pub struct MemContainer {
    pub high_word: u16,
    pub low_word: u16,
    pub dword: u32,
    pub ptr: *mut MemType,
}

pub enum MemType {
    Raw = u8,
    Struct99 = astruct_99
}
