use std::ffi::c_void;
use crate::windef16::HWND16;

#[derive(Default,Debug,Clone)]
pub struct Struct1000 {
    pub win_handle_0x8: HWND16,
    pub field_0xd2: *mut c_void,
}
