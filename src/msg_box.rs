use crate::defines::U32Ptr;
use crate::exit::fatal_app_exit_1000_3e9e;
use crate::global::AppContext;
use crate::util::SBORROW2;
use crate::winapi::{MessageBeep16, MessageBox16};

pub fn msg_box_op_1000_1f24(ctx: &mut AppContext, param_1: i16, param_2: u16, param_3: u16, param_4: u16) -> bool {
    let pi_var1: U32Ptr;
    let unaff_CS: u16;

    if param_3 < (param_1 + 0xc) {
        msg_box_op_1000_214c(ctx, 0x0, 0x0, 0xd940, param_4);
        return 0x1;
    }
    pi_var1 = (param_1 + 0xc);
    *pi_var1 = *pi_var1 + 0x1;
    return 0x0;
}

pub fn msg_box_op_1000_214c(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: i16,
    param_3: &String,
    param_5: u16,
) -> bool {
    let IVar1: i16;
    let i_var2: i16;
    let mut text: String;

    text = (0x2 - (param_2 == 0x0) | 0x2110);
    MessageBeep16(param_5);
    loop {
        IVar1 = MessageBox16(ctx.s_tile2_bmp_1050_1538, &text, 0x1de8, 0x1000);
        i_var2 = IVar1 + -0x1;
        if (i_var2 == 0x0) {
            return 0x0;
        }
        if ((0x0 < i_var2) && (!SBORROW2(i_var2, 0x1))) {
            if (IVar1 == 0x3 || IVar1 + -0x2 < 0x1) {
                fatal_app_exit_1000_3e9e(ctx.s_tile2_bmp_1050_1538);
                return 0x0;
            }
            if (IVar1 == 0x4) {
                return 0x1;
            }
            if (IVar1 == 0x5) {
                return 0x0;
            }
        }
        if ((text & 0x2000) == 0x0) {
            return 0x0;
        }
        text = (text & 0xdfef | 0x1010);
    }
}
