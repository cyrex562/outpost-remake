use std::intrinsics::offset;

use crate::{struct_ops, struct_ops2, sys_ops, winapi};
use crate::app_context::AppContext;
use crate::other_funcs::switch_stmt_1008_d818;
use crate::pass::pass11_funcs::pass1_1008_c79a;
use crate::pass::pass12_funcs::{pass1_1008_b38c, pass1_1008_b47a, pass1_1008_b4a0, pass1_1008_b61a, pass1_1008_b63a, pass1_1008_b820};
use crate::pass::pass13_funcs::{pass1_1008_a930, pass1_1008_b146, pass1_1008_b1a6, pass1_1008_b200, pass1_1008_b366};
use crate::pass::pass14_funcs::{pass1_1008_5784, pass1_1008_5b12, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::pass1_1020_bae6;
use crate::pass::pass17_funcs::{pass1_1030_6c1a, pass1_1030_6ddc, pass1_1030_6e14, pass1_1030_73a8, pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass20_funcs::{pass1_1010_9044, pass1_1010_9130, pass1_1010_9210, pass1_1010_c320, pass1_1010_c3c2, pass1_1010_dd5e};
use crate::pass::pass6_funcs::{pass1_1038_387e, pass1_1038_3aa6, pass1_1038_993a};
use crate::pass::pass7_funcs::{pass1_1018_1c9a, pass1_1018_1e78, pass1_1018_3ab2, pass1_1018_5206};
use crate::pass::pass8_funcs::{pass1_1010_375e, process_struct_1010_20ba, return_1_1010_60b4, return_1_1010_60ba, return_1_1010_60c0, return_1_1010_60c6};
use crate::prog_structs::prog_structs_16::Struct588;
use crate::prog_structs::prog_structs_21::Struct25;
use crate::prog_structs::prog_structs_24::Struct103;
use crate::prog_structs::prog_structs_25::Struct70;
use crate::prog_structs::prog_structs_26::{Struct1096, Struct51};
use crate::prog_structs::prog_structs_27::pass1_struct_2;
use crate::prog_structs::prog_structs_28::{Struct19, Struct300, Struct351};
use crate::prog_structs::prog_structs_29::Struct49;
use crate::prog_structs::prog_structs_30::Struct347;
use crate::prog_structs::prog_structs_31::{Struct20, Struct21};
use crate::sound_funcs::mci_send_command_1008_5c7c;
use crate::string_ops::load_string_1008_b1f0;
use crate::struct_ops::process_struct_1000_179c;
use crate::sys_ops::win_msg;
use crate::typedefs::{HANDLE16, HWND16, LPARAM, LRESULT};
use crate::ui_ops::{misc, ui2, window};
use crate::ui_ops::cursor::set_cursor_1038_bc30;
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};
use crate::winapi::{DestroyWindow16, EnableWindow16, GetDlgItem16, SendMessage16, SetDlgItemText16, SetFocus16, SetWindowText16, ShowWindow16};

pub fn send_dialog_item_msg_1040_d79c(param_1: u32) {
    let lVar1: u32;
    let mut u_var2: u16;

    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let LVar5: LRESULT;
    let mut local_10e: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_10e, 3));
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1010_c3c2(
        _local_6,
        (_local_6 >> 0x10),
        CONCAT22(unaff_ss, local_106),
        (i_var3 + 0x98),
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_ss, local_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        (i_var3 + 6),
    );
    SendDlgItemMessage16(0, 0, 0xb, (s_dibtext_bmp_1050_1844 + 3), (i_var3 + 6));
    LVar5 = SendDlgItemMessage16(0, 0, 0x405, (s_dibtext_bmp_1050_1844 + 3), (i_var3 + 6));
    u_var2 = LVar5;
    pass1_1010_9044((i_var3 + 0x9c));
    _local_10a = CONCAT22(ctx.dx_reg, u_var2);
    local_10e._0_2_ = 0;
    local_10e._2_2_ = 0;
    while (CONCAT22(local_10e._2_2_, local_10e) < _local_10a) {
        pass1_1010_9130((i_var3 + 0x9c), CONCAT22(unaff_ss, local_106), local_10e);
        if (local_106[0] != '\0') {
            SendDlgItemMessage16(
                CONCAT22(unaff_ss, local_106),
                0,
                0x401,
                (s_dibtext_bmp_1050_1844 + 3),
                (i_var3 + 6),
            );
        }
        lVar1 = CONCAT22(local_10e._2_2_, local_10e) + 1;
        local_10e._0_2_ = lVar1;
        local_10e._2_2_ = (lVar1 >> 0x10);
    }
    SendDlgItemMessage16(0, 1, 0xb, (s_dibtext_bmp_1050_1844 + 3), (i_var3 + 6));
    return;
}

pub fn send_dialog_item_msg_1040_d20c(in_struct_588_ptr_1: *mut Struct588, param_2: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;
    let struct_588_ptr_2: *mut Struct588;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let ppVar3: *mut pass1_struct_1;
    let pu_var4: Vec<u8>;
    let mut u_var5: u16;
    let struct_588_ptr_1: *mut Struct588;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        window::enable_window_1040_d60e(in_struct_588_ptr_1);
        return;
    }
    u_var2 = (in_struct_588_ptr_1 >> 0x10);
    struct_588_ptr_2 = in_struct_588_ptr_1;
    if (struct_588_ptr_2.field_0xa0 != 0) {
        pass1_1010_9210(struct_588_ptr_2.field_0x9c);
        window::enable_window_1040_d60e(in_struct_588_ptr_1);
        ppVar1 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            param_2,
        );
        pu_var4 = local_106;
        u_var5 = unaff_ss;
        local_6 = ppVar1;
        ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var4, 3));
        pass1_1010_c3c2(
            ppVar3,
            (ppVar3 >> 0x10),
            CONCAT22(u_var5, pu_var4),
            CONCAT22(in_dx, ppVar1),
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_106),
            0,
            0x401,
            (s_dibtext_bmp_1050_1844 + 3),
            struct_588_ptr_2.field_0x6,
        );
    }
    return;
}

pub fn send_dlg_item_msg_1040_d29c(param_1: u32) {
    send_dialog_item_msg_1040_d79c(param_1);
    return;
}

pub fn send_dlg_item_int_1040_cdac(param_1: u32, param_2: u16, param_3: u16, param_4: i32) -> u16 {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let mut u8_var3: bool;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_4: u16;

    u8_var3 = false;
    i_var4 = param_1;
    u_var5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        i_var2 = (i_var4 + 0x9e);
        pi_var1 = (i_var4 + 0x9c);
        unsafe {
            if (*pi_var1 == i_var2 || *pi_var1 < i_var2) {}
            // goto LAB_1040_cdef;
            pi_var1 = (i_var4 + 0x9e);
            *pi_var1 = *pi_var1 + 1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1040_cdef;
        if ((i_var4 + 0x9e) < 1) {}
        // goto LAB_1040_cdef;
        pi_var1 = (i_var4 + 0x9e);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
    }
    u8_var3 = true;
    // LAB_1040_cdef:
    if (u8_var3) {
        (0, *(i_var4 + 0x9e), 0x18e, (i_var4 + 6));
    }
    return 0;
}

pub fn send_dialog_item_msg_1040_ce12(param_1: u32, param_2: u32, param_3: u16) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u32;
    let mut iStack262: i32;
    let HStack260: HWND16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar1 == 0) {
            break;
        }
        wsprintf16(
            &local_10a,
            CONCAT22(0x5f12, unaff_ss),
            CONCAT22((lVar1 + 4), 0x1050),
        );
        HStack260 = (param_1 + 6);
        iStack262 = param_3;
        local_10a = 0x4010000;
        _local_10e = CONCAT22(unaff_ss, &local_10a);
        SendDlgItemMessage16(_local_10e, 0, 0x401, param_3, HStack260);
    }
    return;
}

pub fn send_dlg_item_msg_1040_ce76(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let LVar3: LRESULT;
    let mut u_var4: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, (s_vrpal_bmp_1050_183a + 8), (i_var1 + 6));
    local_6 = LVar3;
    local_4 = local_6 >> 0xf;
    if (local_6 != 0xffff) {
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_106),
            local_6,
            0x40a,
            (s_vrpal_bmp_1050_183a + 8),
            (i_var1 + 6),
        );
        u_var4 = pass1_1018_5206((i_var1 + 0x98));
        if (u_var4 != 0) {
            (i_var1 + 0x9c) = (u_var4 + 8);
            (i_var1 + 0x9e) = 0;
            SetDlgItemint16(0, 0, 0x18e, (i_var1 + 6));
            SetDlgItemint16(0, *(i_var1 + 0x9c), 0x191, (i_var1 + 6));
        }
    }
    return;
}

pub fn set_dlg_item_txt_1040_b45e(param_1: u32) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x90) != 0) {
        u_var1 = (i_var3 + 0x90);
        (u_var1 + 0x14) = (i_var3 + 6);
        u_var1 = (i_var3 + 0x90);
        local_6 = (u_var1 + 2);
        local_8 = 0;
        while (
            pu_var2 = (i_var3 + 0x90),
            *pu_var2 != local_8 && local_8 <= *pu_var2,
        ) {
            u_var1 = (local_6 + 2);
            SetDlgItemText16(CONCAT22(u_var1, local_6), (u_var1 >> 0x10), (i_var3 + 6));
            local_8 = local_8 + 1;
            local_6 = local_6 & 0xffff0000 | (local_6 + 10);
        }
    }
    return;
}

pub fn set_dialog_item_txt_1040_ad14(in_struct_1: *mut Struct347) {
    set_dialog_item_text_1040_ae04(in_struct_1);
    return;
}

pub fn set_dialog_item_text_1040_ae04(param_1: *mut Struct347) {
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut u_var3: u16;
    let string_2: String;
    let string_3: String;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut local_AX_349: u16;
    let lVar6: u32;
    let local_DL_13: u8;
    let mut local_DX_81: u16;
    let mut local_DX_110: u16;

    let mut local_DX_373: u16;
    let local_struct_1: *mut Struct347;
    let plVar7: *mut long;
    let mut local_SI__1: u16;
    let mut local_DI__1: u16;
    let mut ctx.es_reg: u16;

    let pp_var8: *mut pass1_struct_1;
    let mut u_var9: u32;
    let mut u_var10: u32;
    let mut local_124: u32;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut string_1: [u8; 256];
    let mut temp_5f97b0c4d6: u32;

    pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_SI__1, 3));
    ctx.es_reg = (param_1 >> 0x10);
    local_struct_1 = param_1;
    pass1_1010_c3c2(
        pp_var8,
        (pp_var8 >> 0x10),
        CONCAT22(ctx.stack_seg_reg, string_1),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(ctx.stack_seg_reg, string_1),
        0x1778,
        local_struct_1.window_handle,
    );
    u_var9 = pass1_1030_73a8(local_struct_1.field_0x94);
    string_3 = (u_var9 + 0x20);
    u_var3 = (u_var9 >> 0x10);
    u_var10 = pass1_1030_8326();
    local_DX_110 = (u_var10 >> 0x10);
    u_var9 = 0;
    local_116 = 0;
    b_var2 = false;
    local_118 = 0;
    while (true) {
        local_AX_349 = u_var9;
        if (9 < local_118) {
            break;
        }
        u_var4 = (string_3 + local_118 * 0xc + 2) | (string_3 + local_118 * 0xc);
        u_var9 = u_var4;
        if (u_var4 != 0) {
            plVar7 = (string_3 + local_118 * 0xc);
            u_var5 = big_switch_statement_1020_c222((plVar7 + 1));
            SetDlgItemText16(
                CONCAT22(local_DX_110, u_var5),
                local_116 + 0x1d2,
                local_struct_1.window_handle,
            );
            unsafe {
                lVar6 = *plVar7 - (u_var10 & 0xffff);
            }
            wsprintf16(
                string_1,
                CONCAT13(0x5e, CONCAT12(0xf4, ctx.stack_seg_reg)),
                CONCAT13((lVar6 >> 8), CONCAT12(lVar6, 0x1050)),
            );
            _local_120 = CONCAT22(ctx.stack_seg_reg, string_1);
            SetDlgItemText16(_local_120, local_116 + 0x1dc, local_struct_1.window_handle);
            u_var1 = local_struct_1.field_0x98;
            local_124._0_2_ = u_var1;
            local_124._2_2_ = (u_var1 >> 0x10);
            wsprintf_1010_8c96(
                local_124,
                local_124._2_2_,
                string_1,
                ctx.stack_seg_reg,
                plVar7,
                u_var3,
            );
            u_var9 = ZEXT24(string_1);
            _local_120 = CONCAT22(ctx.stack_seg_reg, string_1);
            local_DX_110 = ctx.dx_reg;
            SetDlgItemText16(_local_120, local_116 + 0x1e6, local_struct_1.window_handle);
            local_116 = CONCAT22(1, local_116 + 1);
            b_var2 = true;
        }
        local_118 = local_118 + 1;
    }
    if (!b_var2) {
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        SetDlgItemText16(
            CONCAT22(local_DX_373, local_AX_349),
            0x1d2,
            local_struct_1.window_handle,
        );
    }
    return;
}

pub fn set_dialog_item_1040_a94a(in_struct_1: *mut Struct351) {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let lVar3: u32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let value: Vec<u8>;
    let mut u_var6: u16;
    let mut u_var7: i32;
    let HVar8: HWND16;
    let mut value_00: u16;

    let mut u_var10: i32;
    let local_struct_1: *mut Struct351;
    let mut i_var11: i32;
    let mut unaff_si: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let mut b_var14: bool;
    let ppVar15: *mut pass1_struct_1;
    let mut local_128: u16;
    let mut local_126: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u32;
    let mut local_116: u16;
    let mut local_114: u16;
    let mut local_112: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: [u8; 256];
    let mut u_var9: u32;

    ppVar15 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
    u_var5 = (ppVar15 >> 0x10);
    u_var4 = ppVar15;
    u_var12 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pass1_1010_c3c2(
        u_var4,
        u_var5,
        CONCAT22(unaff_ss, local_102),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(unaff_ss, local_102),
        0x1f2,
        local_struct_1.field_0x6,
    );
    pass1_1010_c320(
        u_var4,
        u_var5,
        CONCAT22(unaff_ss, local_102),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(unaff_ss, local_102),
        0x1774,
        local_struct_1.field_0x6,
    );
    str_fn_1010_c446(
        ppVar15,
        CONCAT22(unaff_ss, local_102),
        local_struct_1.field_0x94,
    );
    value = local_102;
    SetDlgItemText16(CONCAT22(unaff_ss, value), 0x1773, local_struct_1.field_0x6);
    u_var2 = local_struct_1.field_0x94;
    pass1_1030_6ddc(u_var2, (u_var2 >> 0x10));
    (0, value, 0x1f5, local_struct_1.field_0x6);
    pass1_1030_6e14(local_struct_1.field_0x94);
    (0, value, 0x1f6, local_struct_1.field_0x6);
    if (local_struct_1.field_0x98 != 0) {
        pass1_1010_dd5e(u_var4, u_var5, local_struct_1.field_0x94);
        if ((ctx.dx_reg | value) != 0) {
            u_var2 = local_struct_1.field_0x94;
            u_var13 = (u_var2 >> 0x10);
            i_var11 = u_var2;
            lVar3 = (i_var11 + 0x26);
            u_var10 = (i_var11 + 0x28);
            local_114 = 0x1798;
            local_116 = 0x17c3;
            local_struct_1.field_0xea = 0;
            local_120 = 1;
            while (local_120 < 0x25) {
                if (lVar3 == 0) {
                    value_00 = 0;
                    u_var7 = 0;
                } else {
                    u_var9 = pass1_1020_bae6(lVar3, CONCAT22(local_120, (lVar3 >> 0x10)));
                    value_00 = u_var9;
                    u_var7 = u_var10;
                }
                b_var14 = (value + local_120 * 4) != 0;
                u_var10 = u_var7;
                if (b_var14) {
                    u_var6 = value_00;
                    big_switch_statement_1020_c0d8(local_120);
                    SetDlgItemText16(
                        CONCAT22(u_var10, u_var6),
                        local_114,
                        local_struct_1.field_0x6,
                    );
                    SetDlgItemInt16(
                        0,
                        *(value + local_120 * 4),
                        local_116,
                        local_struct_1.field_0x6,
                    );
                }
                u_var7 = u_var7 | value_00;
                if (u_var7 != 0) {
                    if (!b_var14) {
                        big_switch_statement_1020_c0d8(local_120);
                        SetDlgItemText16(
                            CONCAT22(u_var10, u_var7),
                            local_114,
                            local_struct_1.field_0x6,
                        );
                    }
                    SetDlgItemInt16(0, value_00, local_116, local_struct_1.field_0x6);
                    i_var11 = local_struct_1.field_0xea;
                    HVar8 = GetDlgItem16(local_114, local_struct_1.field_0x6);
                    (&local_struct_1.field_0x9a + i_var11 * 2) = HVar8;
                    pi_var1 = &local_struct_1.field_0xea;
                    unsafe {
                        *pi_var1 = *pi_var1 + 1;
                    }
                    i_var11 = local_struct_1.field_0xea;
                    HVar8 = GetDlgItem16(local_116, local_struct_1.field_0x6);
                    (&local_struct_1.field_0x9a + i_var11 * 2) = HVar8;
                    pi_var1 = &local_struct_1.field_0xea;
                    unsafe {
                        *pi_var1 = *pi_var1 + 1;
                    }
                    b_var14 = true;
                }
                if (b_var14) {
                    local_114 = local_114 + 1;
                    local_116 = local_116 + 1;
                }
                local_120 = local_120 + 1;
            }
        }
    }
    return;
}

pub fn set_dialog_item_1040_a84a(param_1: *mut Struct351) {
    set_dialog_item_1040_a94a(param_1);
    return;
}

pub unsafe fn check_gui_dlg_1038_b922(ctx: &mut AppContext, param_1: &mut u32, param_2: &mut String, uparam_3: i32) {
    let pi_var1: i32;
    let mut i_var2: i32;
    let in_struct_1: Struct300;
    let ppc_var3: fn();
    let h_var4: HANDLE16;
    let mut u_var5: u16;
    let h_var6: HWND16;
    let mut u_var7: i32;
    let puVar8: Vec<u8>;
    let mut u_var9: u32;
    let pu_var10: u16;
    let mut struct_a: Struct103;
    let mut u_var11: u16;
    let mut i_var12: i32;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut unaff_ss: u16;
    let LVar15: LRESULT;
    let uVar16: u8;
    let mut local_470: u16;
    let mut local_46e: u16;
    let mut local_46c: u16;
    let mut local_468: u16;
    let mut local_466: u16;
    let mut local_464: [u8; 80];
    let mut local_414: [u8; 1024];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    i_var12 = param_1;
    u_var13 = (param_1 >> 0x10);
    if (param_3 < 0x1b5) {
        if (0x19f < param_3) {
            local_8 = winapi::GetDlgItem16(param_3, (i_var12 + 6));
            LVar15 = winapi::SendMessage16(0, 0, 0x400, local_8);
            local_a = LVar15;
            if (local_a == 2) {
                return;
            }
            winapi::SendMessage16(0, (local_a == 0), 0x401, local_8);
            u_var5 = IsDlgButtonChecked16(param_3, (i_var12 + 6));
            if (u_var5 != 0) {
                pi_var1 = (i_var12 + 0x96);
                unsafe {
                    *pi_var1 = *pi_var1 + -1;
                }
                h_var6 = winapi::GetDlgItem16(0xfb1, (i_var12 + 6));
                IsWindowEnabled16(offset, h_var6);
                u_var7 = ctx.dx_reg;
                if (h_var6 == 0) {
                    h_var6 = winapi::GetDlgItem16(0xfb1, (i_var12 + 6));
                    winapi::EnableWindow16(1, h_var6);
                }
                if ((i_var12 + 0x96) < 0) {
                    CheckDlgButton16(0, (i_var12 + 0x98), (i_var12 + 6));
                    (i_var12 + 0x96) = 0;
                }
                (i_var12 + 0x98) = param_3;
                pass1_1018_1c9a(*(i_var12 + 0x92), param_3);
                u_var9 = pass1_1018_1e78((i_var12 + 0x92), 0xffff);
                local_e = (u_var9 & 0xffff | u_var7 << 0x10);
                if ((u_var7 | u_var9) == 0) {
                    local_10 = 0;
                } else {
                    local_10 = (u_var9 + 0x1c);
                }
                mci_send_command_1008_5c7c(&ctx._g_struct_ptr_1050_02a0, CONCAT22(local_10, 1));
                return;
            }
            pi_var1 = (i_var12 + 0x96);
            unsafe {
                *pi_var1 = *pi_var1 + 1;
            }
            if ((i_var12 + 0x96) != 1) {
                return;
            }
            h_var6 = winapi::GetDlgItem16(0xfb1, (i_var12 + 6));
            winapi::EnableWindow16(0, h_var6);
            return;
        }
        if (param_3 == 2) {
            return;
        }
        // LAB_1038_bc20:
        uVar16 = param_3;
    } else {
        if (param_3 == 0xfb1) {
            local_46c._0_1_ = 0xa0;
            local_46c._1_1_ = 1;
            while (CONCAT11(local_46c._1_1_, local_46c) < 0x1b5) {
                u_var5 = IsDlgButtonChecked16(CONCAT11(local_46c._1_1_, local_46c), (i_var12 + 6));
                if (u_var5 == 1) {
                    in_struct_1 = (i_var12 + 0x8e);
                    uVar16 = (in_struct_1 >> 0x10);
                    switch_stmt_1008_d818(in_struct_1, CONCAT11(local_46c._1_1_, local_46c));
                    // goto LAB_1038_bba2;
                }
                i_var2 = CONCAT11(local_46c._1_1_, local_46c) + 1;
                local_46c._0_1_ = i_var2;
                local_46c._1_1_ = (i_var2 >> 8);
            }
        } else {
            if (param_3 != 0xfbe) {}
            // goto LAB_1038_bc20;
            local_e = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_470, 2));
            u_var7 = (local_e >> 0x10);
            local_10 = u16_1050_13ae;
            if (u16_1050_13ae == 1) {
                local_10 = 2;
            }
            local_a = (local_10 * 0xc + 0x5b84) - 1;
            pass1_fn_1008_612e(0, local_a);
            u_var9 = pass1_1018_1e78((i_var12 + 0x92), ((local_10 * 6 + local_a) * 2 + 0x5b86));
            _local_14 = (u_var9 & 0xffff | u_var7 << 0x10);
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x50,
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_464)),
                0x5f1,
            );
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                *_local_14,
            );
            struct_a = ctx.dx_reg;
            u_var7 = wsprintf16(
                local_414,
                CONCAT13(0x5b, CONCAT12(0xc0, unaff_ss)),
                CONCAT13((local_464 >> 8), CONCAT12(local_464, 0x1050)),
            );
            u_var14 = 0x1000;
            process_struct_1000_179c(0xb4, &mut struct_a);
            h_var4 = ctx.g_h_window;
            if ((struct_a | u_var7) == 0) {
                local_8 = 0;
                u_var11 = 0;
            } else {
                local_470 = ctx._g_struct_73_1050_14cc;
                local_46e = (ctx._g_struct_73_1050_14cc >> 0x10);
                puVar8 = local_414;
                load_string_1010_847e(ctx, local_470, local_46e, 0x7b);
                local_46c._0_1_ = ctx.dx_reg;
                local_46c._1_1_ = (ctx.dx_reg >> 8);
                u_var14 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                u_var11 = ctx.dx_reg;
                pu_var10 = sys_ops::process_struct_1040_8478(
                    CONCAT13((puVar8 >> 8), CONCAT12(puVar8, 0x41)),
                    0x41,
                    CONCAT13(local_46c._1_1_, CONCAT12(local_46c, puVar8)),
                    CONCAT22(unaff_ss, local_414),
                    h_var4,
                );
                local_8 = pu_var10;
            }
            ppc_var3 = (CONCAT22(u_var11, local_8) + 0x74);
            (**ppc_var3)(u_var14, local_8);
            if (local_8 != 1) {
                return;
            }
            switch_stmt_1008_d818((i_var12 + 0x8e), (_local_14 + 0x1a));
            uVar16 = *(_local_14 + 0x1a);
            // LAB_1038_bba2:
            set_cursor_1038_bc30(param_1, u_var13, uVar16);
        }
        winapi::PostMessage16(0, 0xce, 0x111, ctx.g_h_window);
        uVar16 = 1;
    }
    win_msg::post_win_msg_1040_7b3c(
        param_1,
        (param_1 >> 0x10),
        param_2,
        (param_2 >> 0x10),
        uVar16,
    );
    return;
}

pub unsafe fn get_dlg_item_int_1038_8afe(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let local_4: bool;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = GetDlgItemInt16(0, &local_4, unaff_ss, (s_dibtext_bmp_1050_1844 + 9));
    pass1_1030_6c1a((i_var3 + 0x94), u_var2);
    u_var1 = (i_var3 + 0x94);
    pass1_1038_387e((u_var1 + 0x2e), u_var2, (i_var3 + 0x9c), (i_var3 + 0x94));
    return;
}

pub unsafe fn send_dlg_item_msg_1038_87b2(param_1: Vec<u8>) -> i32 {
    let mut u_var1: u32;
    let l_param: LPARAM;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let LVar4: LRESULT;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: [u8; 256];

    i_var2 = send_dlg_item_msg_1038_8164(
        param_1,
        CONCAT22(unaff_ss, local_102),
        (s_logo_bmp_1050_1850 + 5),
    );
    if (i_var2 != 0) {
        u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        pass1_1008_b61a((i_var2 + 0x94), CONCAT22(unaff_ss, local_102));
        u_var1 = (i_var2 + 0x94);
        l_param = load_string_1008_b1f0(u_var1, (u_var1 >> 0x10));
        LVar4 = SendDlgItemMessage16(
            l_param,
            0xffff,
            0x40d,
            (s_logo_bmp_1050_1850 + 6),
            (i_var2 + 6),
        );
        i_var2 = LVar4;
    }
    return i_var2;
}

pub unsafe fn send_dlg_item_msg_1038_8618(param_1: Vec<u8>) -> i32 {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut u_var6: u32;
    let LVar7: LRESULT;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    _local_6 = pass1_1008_b820((i_var3 + 0x94));
    i_var2 = _local_6;
    if (_local_6 != 0) {
        i_var2 = send_dlg_item_msg_1038_8164(
            param_1,
            CONCAT22(unaff_ss, local_106),
            (s_logo_bmp_1050_1850 + 4),
        );
        if (i_var2 != 0) {
            SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            u_var1 = (i_var3 + 0x94);
            lVar5 = pass1_1008_b4a0(u_var1, (u_var1 >> 0x10), local_106, unaff_ss);
            pass1_1008_b200((i_var3 + 0x94));
            if (lVar5 != 0) {
                send_dialog_item_msg_1038_8400(i_var3, u_var4, lVar5, (s_logo_bmp_1050_1850 + 5));
                u_var6 = pass1_1008_b366((i_var3 + 0x94));
                if (u_var6 != 0) {
                    SendDlgItemMessage16(
                        u_var6,
                        0xffff,
                        0x40d,
                        (s_logo_bmp_1050_1850 + 5),
                        (i_var3 + 6),
                    );
                }
            }
            u_var6 = pass1_1008_b38c((i_var3 + 0x94));
            if (u_var6 != 0) {
                send_dialog_item_msg_1038_8400(i_var3, u_var4, u_var6, (s_logo_bmp_1050_1850 + 6));
                u_var6 = pass1_1008_b47a((i_var3 + 0x94));
                if (u_var6 != 0) {
                    SendDlgItemMessage16(
                        u_var6,
                        0xffff,
                        0x40d,
                        (s_logo_bmp_1050_1850 + 6),
                        (i_var3 + 6),
                    );
                }
            }
            SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            LVar7 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            i_var2 = LVar7;
        }
    }
    return i_var2;
}

pub unsafe fn send_dialog_item_msg_1038_8400(param_1: u32, param_2: u32, param_3: u16) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar1 == 0) {
            break;
        }
        SendDlgItemMessage16((lVar1 + 4), 0, 0x401, param_3, (param_1 + 6));
    }
    return;
}

pub fn send_dialog_item_msg_1040_3f12(param_1: u32, param_2: u32) {
    let pu_var1: Vec<u8>;

    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let LVar5: LRESULT;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, 400, (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, 400, (i_var3 + 6));
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_2);
    while (true) {
        pu_var1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            break;
        }
        LVar5 = SendDlgItemMessage16((pu_var1 + 4), 0, 0x401, 400, (i_var3 + 6));
        i_var2 = (LVar5 >> 0x10);
        if (((LVar5 == -1) && (i_var2 == -1)) || (LVar5 == -2 && (i_var2 == -1))) {
            break;
        }
    }
    SendDlgItemMessage16(0, 0, 0x407, 400, (i_var3 + 6));
    SendDlgItemMessage16(0, 1, 0xb, 400, (i_var3 + 6));
    return;
}

pub fn win_gui_dialog_func_1040_3e08(param_1: &mut Struct25) {
    let mut u_var1: u16;
    let local_bx_4: &mut Struct25;
    let mut u_var2: u16;
    let LVar3: LRESULT;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    CheckRadioButton16(local_bx_4.check_id, 0x18d, 0x188, local_bx_4.h_wnd);
    local_bx_4.dlg_item_id = 0;
    local_bx_4.dlg_item_id_2 = 0;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, 400, local_bx_4.h_wnd);
    if (LVar3 != -1) {
        u_var1 = pass1_1018_3ab2(local_bx_4.field_0x8e, LVar3, local_bx_4.check_id);
        local_bx_4.dlg_item_id_2 = u_var1;
    }
    SetDlgItemInt16(0, local_bx_4.dlg_item_id, 0x18e, local_bx_4.h_wnd);
    SetDlgItemInt16(0, local_bx_4.dlg_item_id_2, 0x191, local_bx_4.h_wnd);
    match (local_bx_4.check_id) {
        0x188 => local_bx_4.field_0xa4 = 5,
        0x189 => local_bx_4.field_0xa4 = 6,
        0x18a => local_bx_4.field_0xa4 = 7,
        0x18b => local_bx_4.field_0xa4 = 8,
        0x18c => local_bx_4.field_0xa4 = 9,
        0x18d => {
            local_bx_4.field_0xa4 = 10;
        }
    }
    misc::invalidate_rect_1040_3ddc(param_1);
    return;
}

pub fn check_dialog_btn_1040_1d7a(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_bx_8: &mut Struct51;
    let mut u_var3: u16;

    local_bx_8 = param_1;
    u_var3 = (param_1 >> 0x10);
    if ((param_2 != 0) && (u_var1 = local_bx_8.field_0x8e, (u_var1 + 0x72) != 0)) {
        u_var2 = IsDlgButtonChecked16(0xe1, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d5);
        }
        u_var2 = IsDlgButtonChecked16(0xe2, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d6);
        }
        u_var2 = IsDlgButtonChecked16(0xe3, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d7);
        }
        u_var2 = IsDlgButtonChecked16(0xe5, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d8);
        }
        u_var2 = IsDlgButtonChecked16(0xe6, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1e2);
        }
        u_var2 = IsDlgButtonChecked16(0xe7, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1dc);
        }
        return;
    }
    DestroyWindow16(local_bx_8.field_0x6);
    return;
}

pub fn check_dialog_func_1040_1afe(param_1: &mut Struct20) {
    let mut check: u16;
    let mut check_00: u16;
    let mut check_01: u16;
    let mut u_var1: u32;
    let mut u_var2: u32;
    let local_Struct20: &mut Struct20;
    let mut local_es_4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_Struct20 = param_1;
    u_var1 = local_Struct20.field_0x8e;
    u_var2 = local_Struct20.field_0x8e;
    check = *(u_var2 + 0x20);
    u_var2 = local_Struct20.field_0x8e;
    check_00 = *(u_var2 + 0x74);
    u_var2 = local_Struct20.field_0x8e;
    check_01 = *(u_var2 + 0x72);
    CheckDlgButton16(*(u_var1 + 0x1e), 0xfdb, local_Struct20.h_dialog_6);
    CheckDlgButton16(check_00, 0xfdd, local_Struct20.h_dialog_6);
    CheckDlgButton16(check_01, 0xfde, local_Struct20.h_dialog_6);
    CheckDlgButton16(check, 0xfdc, local_Struct20.h_dialog_6);
    return;
}

pub fn check_dialog_func_1040_1b8a(param_1: u32) {
    let mut u_var1: u16;
    let mut check: u16;
    let local_bx_4: &mut Struct21;
    let mut u_var2: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = return_1_1010_60b4(local_bx_4.field_0x8e);
    return_1_1010_60c6(local_bx_4.field_0x8e);
    check = return_1_1010_60c0();
    return_1_1010_60ba(local_bx_4.field_0x8e);
    CheckDlgButton16(u_var1, 0xfdb, local_bx_4.h_wnd);
    CheckDlgButton16(check, 0xfdd, local_bx_4.h_wnd);
    CheckDlgButton16(0xfde, 0xfde, local_bx_4.h_wnd);
    u_var1 = local_bx_4.h_wnd;
    CheckDlgButton16(u_var1, 0xfdc, u_var1);
    return;
}

pub fn dialog_button_checked_1038_e7a0(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0x10) = 1;
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 10) = 0;
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0xc) = 0;
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0xe) = 0;
    } else {
        u_var2 = IsDlgButtonChecked16(0x1827, (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16(0x1828, (i_var3 + 6));
            if (u_var2 == 0) {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 10) = 0;
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 10) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 10) = 1;
        }
        u_var2 = IsDlgButtonChecked16(s_vrpal_bmp_1050_183a, (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 1), (i_var3 + 6));
            if (u_var2 == 0) {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xc) = 0;
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xc) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 0xc) = 1;
        }
        u_var2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 2), (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 3), (i_var3 + 6));
            if (u_var2 == 0) {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xe) = 0;
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xe) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 0xe) = 1;
        }
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0x10) = 0;
    }
    (i_var3 + 0x92) = 0;
    return;
}

pub fn check_dlg_btn_1038_e1dc(in_struct_1: &mut Struct49, param_2: u16) {
    let mut u_var1: u16;
    let local_bx_7: &mut Struct49;
    let mut u_var2: u16;
    let mut w_param: u32;

    local_bx_7 = in_struct_1;
    u_var2 = (in_struct_1 >> 0x10);
    if (param_2 != 0) {
        u_var1 = IsDlgButtonChecked16(0x1807, local_bx_7.h_win);
        if (u_var1 == 0) {
            u_var1 = IsDlgButtonChecked16(0x1806, local_bx_7.h_win);
            if (u_var1 == 0) {}
            // goto LAB_1038_e229;
            w_param = 0x1110130;
        } else {
            w_param = 0x111012f;
        }
        SendMessage16(0, w_param, w_param._2_2_, ctx.g_h_window);
    }
    // LAB_1038_e229:
    DestroyWindow16(local_bx_7.h_win);
    return;
}

pub fn dialog_button_checked_1038_cdd6(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_bx_8: &mut Struct70;
    let mut u_var3: u16;

    local_bx_8 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (param_2 == 0) {
        u_var1 = local_bx_8.field_0x8e;
        (u_var1 + 10) = 0;
    } else {
        u_var2 = IsDlgButtonChecked16(0x182e, local_bx_8.field_0x6);
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16(0x182f, local_bx_8.field_0x6);
            if (u_var2 == 0) {
                u_var2 = IsDlgButtonChecked16(0x1829, local_bx_8.field_0x6);
                if (u_var2 == 0) {
                    u_var2 = IsDlgButtonChecked16(0x182a, local_bx_8.field_0x6);
                    if (u_var2 == 0) {
                        u_var2 = IsDlgButtonChecked16(0x182c, local_bx_8.field_0x6);
                        if (u_var2 == 0) {
                            u_var2 = IsDlgButtonChecked16(0x182d, local_bx_8.field_0x6);
                            if (u_var2 != 0) {
                                u_var1 = local_bx_8.field_0x8e;
                                (u_var1 + 10) = 7;
                            }
                        } else {
                            u_var1 = local_bx_8.field_0x8e;
                            (u_var1 + 10) = 6;
                        }
                    } else {
                        u_var1 = local_bx_8.field_0x8e;
                        (u_var1 + 10) = 4;
                    }
                } else {
                    u_var1 = local_bx_8.field_0x8e;
                    (u_var1 + 10) = 3;
                }
            } else {
                u_var1 = local_bx_8.field_0x8e;
                (u_var1 + 10) = 2;
            }
        } else {
            u_var1 = local_bx_8.field_0x8e;
            (u_var1 + 10) = 1;
        }
    }
    local_bx_8.field_0x92 = 0;
    return;
}

pub fn gui_dialog_1038_b81c(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: i32;
    let mut h_dialog: u16;
    let mut b_result: u16;
    let hwnd: HWND16;

    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let pp_var8: &mut pass1_struct_1;
    let local_1e: &mut Struct19;
    let mut uStack28: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut check_id: u16;
    let mut local_a: u16;
    let mut local_6: u32;
    let pu_var5: &mut u16;

    misc::win_gui_func_1040_78e2(param_1);
    pp_var8 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 6));
    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    (i32_var6 + 0x92) = pp_var8;
    (i32_var6 + 0x94) = (pp_var8 >> 0x10);
    u_var1 = (i32_var6 + 0x92);
    u_var4 = u_var1 + 0x4e;
    u_var1 = u_var1 & 0xffff0000;
    pu_var5 = (u_var1 | u_var4);
    local_a = 0;
    check_id = 0x1a0;
    while (check_id < 0x1b5) {
        if ((local_a * 2 + u_var4) == check_id) {
            local_a = local_a + 1;
        } else {
            CheckDlgButton16(2, check_id, (i32_var6 + 6));
        }
        check_id = check_id + 1;
    }
    h_dialog = GetDlgItem16(0xfb1, (i32_var6 + 6));
    b_result = EnableWindow16(0, h_dialog);
    u_var2 = (i32_var6 + 0x92);
    local_1e = u_var2;
    uStack28 = (u_var2 >> 0x10);
    ppc_var3 = ((i32_var6 + 0x92) + 0x10);
    (**ppc_var3)(offset, local_1e, uStack28);
    _local_10 = CONCAT22(ctx.dx_reg, b_result);
    window::move_window_1040_826c(
        i32_var6,
        u_var7,
        (b_result + 2) + -2,
        (b_result + 4) + *_local_10 + 3,
    );
    ShowWindow16(5, (i32_var6 + 6));
    unsafe {
        pass1_1018_1c9a(*(i32_var6 + 0x92), *pu_var5);
    }
    unsafe {
        hwnd = GetDlgItem16(*pu_var5, (i32_var6 + 6));
    }
    SetFocus16(hwnd);
    return;
}

pub fn get_dialog_int_1038_9820(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> libc::c_long {
    let pi_var1: &mut i32;
    let lVar2: u32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let unaff_ss: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_6: bool;
    let local_4: bool;

    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    u_var3 = GetDlgItemInt16(1, &local_4, unaff_ss, (param_4 * 0xe + 0x5a74));
    i_var4 = u_var3 * param_2 * param_3;
    u_var3 = GetDlgItemInt16(1, &local_6, unaff_ss, (param_4 * 0xe + 0x5a76));
    lVar2 = (u_var3 * param_2) * param_3;
    u_var6 = (lVar2 >> 0x10);
    i_var5 = lVar2;
    if ((i_var4 - (i_var7 + 0x94) < 1) && (-1 < (i_var7 + 0x96) - i_var5)) {
        pi_var1 = (i_var7 + 0x94);
        unsafe {
            *pi_var1 = *pi_var1 - i_var4;
        }
        pi_var1 = (i_var7 + 0x96);
        unsafe {
            *pi_var1 = *pi_var1 - i_var5;
        }
        return CONCAT22(u_var6, 1);
    }
    return u_var6 << 0x10;
}

pub fn dialog_item_func_1038_98b4(param_1: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let unaff_ss: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    local_8 = 0;
    while (i_var2 = param_1, u_var3 = (param_1 >> 0x10), local_8 < 0xf) {
        u_var4 = 1;
        u_var3 = (i_var2 + 6);
        u_var1 = GetDlgItemInt16(1, &local_4, unaff_ss, (local_8 * 0xe + 0x5a72));
        get_dialog_int32_1038_9820(param_1, u_var1, u_var3, u_var4);
        local_8 = local_8 + 1;
    }
    SetDlgItemInt16(1, *(i_var2 + 0x94), 0xfa9, (i_var2 + 6));
    SetDlgItemInt16(1, *(i_var2 + 0x96), 0xfa8, (i_var2 + 6));
    return;
}

pub fn dialog_fn_1038_94da(
    param_1: Vec<u8>,
    param_2: Vec<u8>,
    param_3: u16,
    param_4: u16,
    param_5: i32,
) -> u16 {
    let pu_var1: &mut u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let hwnd: HWND16;
    let mut i_var4: i32;
    let unaff_ss: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 1;
    local_8 = pass1_1038_993a(param_1, param_2, param_3_00);
    if ((-1 < local_8)
        && (
            u_var3 = GetDlgItemInt16(1, &local_c, unaff_ss, (local_8 * 0xe + 0x5a72)),
            local_c != 0,
        ))
    {
        if (param_3 == 0) {
            local_6 = u_var3 + 1;
        } else {
            local_4 = 0xffff;
            local_6 = u_var3 - 1;
        }
        local_a = (local_6 <= (local_8 * 0xe + 0x5a7a));
        pu_var1 = (local_8 * 0xe + 0x5a78);
        if (unsafe { *pu_var1 != local_6 } && unsafe { local_6 <= *pu_var1 }) {
            local_a = 0;
        }
        i_var2 = local_8 * 0xe;
        hwnd = GetDlgItem16((i_var2 + 0x5a72), (param_1 + 6));
        SetFocus16(hwnd);
        if ((local_a != 0)
            && (
                i_var4 =
                    get_dialog_int32_1038_9820(CONCAT22(param_2, param_1), 1, local_4, local_8),
                i_var4 != 0,
            ))
        {
            SetDlgItemInt16(1, local_6, (i_var2 + 0x5a72), (param_1 + 6));
            SetDlgItemInt16(1, *(param_1 + 0x94), 0xfa9, (param_1 + 6));
            SetDlgItemInt16(1, *(param_1 + 0x96), 0xfa8, (param_1 + 6));
        }
    }
    return 1;
}

pub fn send_dlg_item_msg_1038_8d22(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let LVar3: LRESULT;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, (s_650_bmp_1050_1859 + 2), (i_var1 + 6));
    local_6 = LVar3;
    local_4 = local_6 >> 0xf;
    if (local_6 != 0xffff) {
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_106),
            local_6,
            0x40a,
            (s_650_bmp_1050_1859 + 2),
            (i_var1 + 6),
        );
        pass1_1008_c79a((i_var1 + 0x94), CONCAT22(unaff_ss, local_106));
    }
    return;
}

pub fn send_dlg_item_msg_1038_8b58(param_1: Vec<u8>) -> u8 {
    let mut u_var1: u32;
    let paVar2: &mut Struct1096;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut in_stack_0000feee: u16;
    let mut local_10a: u32;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000feee, 3));
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1010_c3c2(
        _local_6,
        (_local_6 >> 0x10),
        CONCAT22(unaff_ss, local_106),
        (i_var3 + 0x94),
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_ss, local_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        (i_var3 + 6),
    );
    u_var1 = (i_var3 + 0x94);
    (i_var3 + 0x9c) = (u_var1 + 0x32);
    (i_var3 + 0x9a) = (i_var3 + 0x9c);
    SetDlgItemInt16(
        0,
        *(i_var3 + 0x9c),
        (s_dibtext_bmp_1050_1844 + 9),
        (i_var3 + 6),
    );
    u_var1 = (i_var3 + 0x94);
    paVar2 = (u_var1 + 0x2e);
    pass1_1038_3aa6(paVar2);
    *(i_var3 + 0x98) = paVar2;
    SetDlgItemInt16(0, paVar2, (s_dibtext_bmp_1050_1844 + 0xb), (i_var3 + 6));
    return paVar2;
}

pub fn set_dialog_item_1038_8966(param_1: u32, param_2: u16, param_3: u16, param_4: i32) -> u16 {
    let pi_var1: &mut i32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    b_var2 = false;
    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        if ((i_var3 + 0x98) < 1) {}
        // goto LAB_1038_89af;
        pi_var1 = (i_var3 + 0x9a);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
        pi_var1 = (i_var3 + 0x98);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1038_89af;
        if ((i_var3 + 0x9a) < 1) {}
        // goto LAB_1038_89af;
        pi_var1 = (i_var3 + 0x9a);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
        pi_var1 = (i_var3 + 0x98);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
    }
    b_var2 = true;
    // LAB_1038_89af:
    if (b_var2) {
        SetDlgItemInt16(
            0,
            *(i_var3 + 0x9a),
            (s_dibtext_bmp_1050_1844 + 9),
            (i_var3 + 6),
        );
        SetDlgItemInt16(
            0,
            *(i_var3 + 0x98),
            (s_dibtext_bmp_1050_1844 + 0xb),
            (i_var3 + 6),
        );
    }
    return 0;
}

pub fn send_dialog_item_msg_1038_844a(param_1: u32) -> LRESULT {
    let hwnd: HWND16;
    let b_var1: bool;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let LVar5: LRESULT;
    let mut local_10a: u16;
    let mut local_108: [u8; 258];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
    _local_6 = pass1_1008_b820((i_var3 + 0x94));
    if (_local_6 == 0) {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_108),
            0x448,
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_108),
            0,
            0x401,
            (s_logo_bmp_1050_1850 + 4),
            (i_var3 + 6),
        );
        SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
        SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
        LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
        u_var2 = (LVar5 >> 0x10);
        hwnd = GetDlgItem16((s_logo_bmp_1050_1850 + 7), (i_var3 + 6));
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_108),
            0x449,
        );
        b_var1 = SetWindowText16(CONCAT22(unaff_ss, local_108), hwnd);
        return CONCAT22(u_var2, b_var1);
    }
    send_dialog_item_msg_1038_8400(
        i_var3,
        (param_1 >> 0x10),
        _local_6,
        (s_logo_bmp_1050_1850 + 4),
    );
    window::set_window_text_1038_8358(i_var3, u_var4);
    SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
    SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
    LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
    return LVar5;
}

pub fn send_dlg_item_msg_1038_8164(param_1: Vec<u8>, param_2: Vec<u8>, param_3: u16) -> u16 {
    let mut u_var1: u16;
    let LVar2: LRESULT;

    unsafe {
        *param_2 = '\0';
    }
    u_var1 = (param_1 >> 0x10);
    LVar2 = SendDlgItemMessage16(0, 0, 0x409, param_3, (param_1 + 6));
    if ((LVar2 != -1)
        && (
            LVar2 = SendDlgItemMessage16(param_2, LVar2, 0x40a, param_3, (param_1 + 6)),
            LVar2 != -1,
        ))
    {
        return 1;
    }
    return 0;
}

pub fn send_dlg_item_msg_1038_7eac(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let ppVar3: &mut pass1_struct_1;
    let mut l_param: u32;
    let LVar4: LRESULT;
    let mut in_stack_0000fff2: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    ppVar3 = struct_ops::process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x30),
    );
    l_param = pass1_1010_375e(ppVar3);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    pass1_1008_b1a6((i_var1 + 0x94), l_param);
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
    LVar4 = SendDlgItemMessage16(0, 0, 0x409, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
    if (LVar4 != -1) {
        SendDlgItemMessage16(0, LVar4, 0x403, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
        SendDlgItemMessage16(l_param, 0, 0x401, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
        SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
        SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (i_var1 + 6));
        SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (i_var1 + 6));
        window::enable_window_1038_806a(i_var1, u_var2);
    }
    LVar4 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
    return LVar4;
}

pub fn send_dlg_item_msg_1038_7fae(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    pass1_1008_b146((i_var1 + 0x94));
    SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 5), (i_var1 + 6));
    SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 6), (i_var1 + 6));
    pass1_1008_b61a((i_var1 + 0x94), 0);
    pass1_1008_b63a(*(i_var1 + 0x94), 0x0);
    return;
}
