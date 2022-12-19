use std::ptr::null_mut;
use crate::mem_address::MemAddress;
use crate::structs::struct_99::astruct_99;

#[derive(Default,Debug,Clone)]
pub struct MemContainer {
    pub address: MemAddress
    pub len: usize,
    pub ptr: *mut u8,
    pub buffer: Vec<u8>,
    pub data_type: MemContainerDataType
}

impl Default for MemContainer {
    fn default() -> Self {
        Self {
            address: MemAddress::default(),
            len: 0,
            ptr: null_mut(),
            buffer: vec![],
            data_type: MemContainerDataType::None
        }
    }
}

pub enum MemContainerDataType {
    None,
}
