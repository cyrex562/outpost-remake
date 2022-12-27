use crate::unk::block_1020_0000::{draw_op_1020_041e, pass1_1020_022c, struct_1020_0baa};
use crate::unk::block_1020_1000::{pass1_1020_1d8e, pass1_1020_1da8, pass1_1020_1eea};
use crate::unk::block_1020_2000::{pass1_1020_2286, pass1_1020_239c, pass1_1020_2488};
use crate::draw_ops::draw_f::win_ui_palette_op_1020_0cd2;
use crate::draw_ops::draw_c::draw_line_1020_229c;
use crate::draw_ops::draw_e::{draw_op_1020_15de, unk_draw_op_1020_0c3e, win_ui_op_1020_150e};
use crate::utils::CONCAT22;
use crate::winapp::winapp_d::unk_win_ui_op_1020_1418;
use crate::gui::cleanup::{cleanup_ui_op_1020_1038, destroy_win_1020_1dea, destroy_win_1020_1e1e};
use crate::gui::cursor::win_ui_cursor_op_1020_1294;
use crate::gui::window::get_win_ui_info_op_1020_7a50;
use crate::winapp::winapp_d::send_win_msg_1020_08fe;
use crate::windef16::{HDC16, HGDIOBJ16, HPEN16};

pub fn pass1_1020_07aa(mut param_1: u16, param_2: *mut Struct19) {
    let mut iVar1: *mut Struct19;
    let mut uVar2: *mut Struct19;
    let mut local_16: [u8; 0x14] = [0; 0x14];

    draw_op_1020_041e(param_2);
    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2;
    if (iVar1[0x2].field24_0x30 == 0) {
        iVar1[0x2].field24_0x30 = 0x1;
        pass1_1008_5bdc(CONCAT22(0x1050, local_16));
        win_1008_5c9e(
            local_16,
            param_1,
            CONCAT22(0x1050, local_16),
            (param_2 & 0xffff0000 | ZEXT24(&iVar1[0x2].field25_0x32)),
        );
        pass1_1008_5c34(CONCAT22(0x1050, local_16));
    }
    return;
}

pub fn pass1_1020_07f4(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29 {
    pass1_1020_022c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn win_1020_09ba(mut param_1: u16, mut param_2: u16, param_3: *mut StructA) {
    let mut puVar1: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let iVar1: *mut StructA;
    let mut uVar3: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_2);
    create_window_ex_1008_9760(param_3);
    mem_op_1000_179c(0xe, paVar2);
    puVar1 = (paVar2 | param_1);
    iVar1 = param_3;
    uVar3 = (param_3 >> 0x10);
    if (puVar1.is_null() == false) {
        struct_1020_0baa(puVar1, CONCAT22(paVar2, param_1), iVar1.field4_0x8);
        iVar1[0x1].field11_0x16 = param_1;
        iVar1[0x1].field12_0x18 = puVar1;
        return;
    }
    iVar1[0x1].field11_0x16 = 0;
    return;
}

pub fn pass1_1020_0a0c(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    destroy_win_1008_628e(param_1);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0xe2);
    uVar2 = (iVar4 + 0xe4);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 1);
    }
    (iVar4 + 0xe2) = 0;
    (iVar4 + 0xe6) = 0;
    return;
}

pub fn pass1_1020_0a52(mut param_1: u16, mut param_2: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u32;

    uVar2 = (param_2 >> 0x10);
    uVar1 = param_2;
    unk_draw_op_1020_0c3e((uVar1 + 0xe2));
    if ((uVar1 + 0xe6) == 0) {
        (uVar1 + 0xe6) = 0x1;
        (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        uVar3 = pass1_1038_af40(uVar1, param_1, _PTR_LOOP_1050_5b7c, (uVar1 + 0x8), 0x6);
        (uVar1 + 0xe8) = uVar3;
        (uVar1 + 0xea) = (uVar3 >> 0x10);
    }
    return;
}

pub fn pass1_1020_0aa6(mut param_1: u32) {
    win_ui_palette_op_1020_0cd2((param_1 + 0xe2));
    return;
}

pub fn pass1_1020_0abc(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe6) != 0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x10);
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1020_0ae8(param_1: *mut astruct_63, param_2: u8) -> *mut astruct_63 {
    send_win_msg_1020_08fe(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_0e2c(param_1: *mut astruct_868) {
    get_win_ui_info_op_1020_7a50(param_1);
    cleanup_ui_op_1020_1038(param_1);
    return;
}

pub fn pass1_1020_0e8e(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: i16,
    mut param_6: i16,
) {
    let mut ppc_var1: fn();

    win_ui_cursor_op_1020_1294(param_2, CONCAT22(param_4, param_3), param_5, param_6);
    if param_1 == 0 {
        ppc_var1 = ((param_3 + 0x4) + 0x5c);
        (**ppc_var1)();
    }
    return;
}

pub fn FUN_1020_101f() {
    return;
}

pub fn pass1_1020_1022(mut param_1: u32) {
    draw_op_1020_15de((param_1 + 0xf6));
    return;
}

pub fn pass1_1020_135e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    cleanup_menu_ui_op_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_170a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    win_ui_op_1020_150e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_1780(param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x6c);
    (**ppcVar1)();
    destroy_win_1040_8212(param_1);
    return;
}

pub fn pass1_1020_1b68(param_1: *mut astruct) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct;
    let mut uVar4: *mut astruct;

    uVar4 = param_1 >> 0x10;
    iVar4 = param_1;
    puVar1 = iVar4.field143_0x92;
    uVar2 = iVar4.field144_0x94;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    iVar4.field143_0x92 = 0;
    pass1_1010_4f48(iVar4.field142_0x8e);
    iVar4.field142_0x8e = null_mut();
    return;
}

pub fn pass1_1020_1bb6(mut param_1: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x92) + 0x8);
    (**ppcVar1)();
    return 0x0;
}


pub fn post_win_msg_1020_1ca4(
    mut param_1: u32,
    param_2: *mut Struct57,
    mut param_3: u16,
) -> BOOL16 {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar6: u16;
    let mut puStack10: *mut u32;
    let mut uVar5: u32;

    uVar6 = (param_1 >> 0x10);
    if ((param_1 + 0x96) == 0) {
        pass1_1010_4df0(param_2, (param_1 + 0x8e));
        if (param_3 == 0) {
            uVar6 = 0x1000;
            mem_op_1000_179c(0xb4, param_2);
            uVar3 = param_2 | param_3;
            uVar5 = param_2 & 0xffff0000 | uVar3;
            if (uVar3 == 0) {
                iVar2 = 0;
                uVar4 = 0;
            } else {
                uVar6 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                iVar2 = string_1040_8520(
                    uVar5,
                    CONCAT22(param_2, param_3),
                    HWND16_1050_0396,
                    0x20030,
                    0x62a057b,
                );
                uVar4 = uVar5;
            }
            puStack10 = CONCAT22(uVar4, iVar2);
            ppcVar1 = (*puStack10 + 0x74);
            (**ppcVar1)(uVar6, iVar2, uVar4);
            return 0x0;
        }
        PostMessage16(0x0, 0xde, 0x111, HWND16_1050_0396);
    }
    return 0x1;
}
