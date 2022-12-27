use std::ffi::{CStr, CString};
use crate::mem_address::MemAddress;
use crate::winapp::fatal_app_exit_1000_3e9e;
use crate::utils::CONCAT22;
use crate::winapi16::{MessageBeep16, MessageBox16};

pub fn smart_heap_library_err_1000_214c(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: u16,
) -> bool {
    let mut i_var1: i16;
    let mut i_var2: i16;
    let mut msg_type: u16;

    // CONCAT22(param_4, param_3)
    let address = MemAddress::from_u16s(param_4, param_3);
    let text = addresss.cstring_mut();

    msg_type = 0x2 - (param_2 == 0) | 0x2110;
    MessageBeep16(0x0);
    let title_string = String::from("SmartHeap Library");
    let title_cstring = CString::from(title_string.into_bytes);
    loop {
        i_var1 = MessageBox16(
            msg_type as i16,
            title_cstring.into_raw(),
            text,
            0x0,
        );
        i_var2 = i_var1 - 0x1;
        if i_var2 == 0 {
            return false;
        }
        if (0x0 < i_var2) && (!SBORROW2(i_var2, 1)) {
            if i_var1 == 0x3 || i_var1 -0x2 < 1 {
                fatal_app_exit_1000_3e9e();
                return false;
            }
            if i_var1 == 0x4 {
                return true;
            }
            if i_var1 == 0x5 {
                return false;
            }
        }
        if (msg_type & 0x2000) == 0 {
            return false;
        }
        msg_type = msg_type & 0xdfef | 0x1010;
    }
}
