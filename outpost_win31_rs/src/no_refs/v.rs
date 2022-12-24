use crate::win_ui::pt_in_rect_op_1020_58ce;
use crate::win_ui::{menu_ui_op_1020_5bf2, win_ui_op_1020_5de8, win_ui_op_1020_5e76};

pub unsafe fn pass1_1020_51c6(mut param_1: u32, mut param_2: u16, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut in_DX: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = (param_1 + 0xf4);
    uVar4 = param_3;
    if (iVar2 == 0x2) {
        win_ui_op_1020_5e76(param_1 & 0xffff | uVar3 << 0x10, param_2, uVar4);
        return;
    }
    iVar2 += -0x3;
    if (iVar2 != 0) {
        pt_in_rect_op_1020_58ce(
            in_DX,
            param_1 & 0xffff | uVar3 << 0x10,
            param_2,
            uVar4,
            (param_3 >> 0x10),
        );
        if (iVar2 == 0) {
            ppcVar1 = ((param_1 + 0x4) + 0x5c);
            (**ppcVar1)();
        }
        return;
    }
    win_ui_op_1020_5de8(param_1 & 0xffff | uVar3 << 0x10, param_2, uVar4);
    return;
}


pub unsafe fn pass1_1020_52de(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    puVar1 = (iVar6 + 0xf6);
    uVar2 = (iVar6 + 0xf8);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar6 + 0xf6) = 0;
    if ((iVar6 + 0xfa) != 0) {
        if (param_1 == 0) {
            iVar4 = 0;
            uVar5 = 0;
        } else {
            iVar4 = iVar6 + 0xe2;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6((iVar6 + 0xfa), CONCAT22(uVar5, iVar4));
    }
    destroy_win_1008_628e(param_1);
    if ((iVar6 + 0xfa) != 0) {
        pass1_1010_1dda((iVar6 + 0xfa));
    }
    (iVar6 + 0xfa) = 0;
    return;
}

pub unsafe fn pass1_1020_6208(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    destroy_cursor_1020_42f4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1020_6216(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    destroy_cursor_1020_42f4(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}
