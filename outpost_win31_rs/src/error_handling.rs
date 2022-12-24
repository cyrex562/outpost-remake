use crate::winapp::fatal_app_exit_1000_3e9e;
use crate::utils::CONCAT22;
use crate::winapi16::{MessageBeep16, MessageBox16};

pub unsafe fn msg_box_op_1000_214c(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: u16,
) -> bool {
    let mut IVar1: i16;
    let mut iVar2: i16;
    let mut msg_type: u16;

    msg_type = 0x2 - (param_2 == 0) | 0x2110;
    MessageBeep16(0x0);
    loop {
        IVar1 = MessageBox16(
            msg_type,
            "SmartHeap Library",
            CONCAT22(param_4, param_3),
            0x0,
        );
        iVar2 = IVar1 -0x1;
        if iVar2 == 0 {
            return 0x0;
        }
        if (0x0 < iVar2) && (!SBORROW2(iVar2, 1)) {
            if IVar1 == 0x3 || IVar1 -0x2 < 1 {
                fatal_app_exit_1000_3e9e();
                return 0x0;
            }
            if (IVar1 == 0x4) {
                return 0x1;
            }
            if (IVar1 == 0x5) {
                return 0x0;
            }
        }
        if ((msg_type & 0x2000) == 0) {
            return 0x0;
        }
        msg_type = msg_type & 0xdfef | 0x1010;
    }
}
