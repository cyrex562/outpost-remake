use crate::win_struct::HWND16;
use crate::winapi::PostMessage16;

pub fn post_win_msg_1040_0d5e(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16) {
    if param_3 != 0x0 {
        PostMessage16(param_4, 0x0, 0x0, 0x1110001);
    }
    return;
}
