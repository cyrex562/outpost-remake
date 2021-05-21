use std::intrinsics::offset;

use crate::{draw, mixed_fn_1010_830a, struct_ops, struct_ops2};
use crate::app_context::AppContext;
use crate::cleanup::{destroy_win_1010_3202, destroy_window_1040_8212, window_msg_func_1010_7300};
use crate::draw::{draw2, drawing_context, rect};
use crate::err_ops::error_check_1000_17ce;
use crate::func_ptr_funcs::call_fn_ptr_1000_5586;
use crate::other_funcs::{set_timer_1008_91ba, zero_list_1008_3e38};
use crate::pass::pass10_funcs::pass1_1040_bfde;
use crate::pass::pass12_funcs::{pass1_1008_b47a, pass1_1008_b820};
use crate::pass::pass13_funcs::{pass1_1008_8ce4, pass1_1008_941a, pass1_1008_9628, pass1_1008_b340, pass1_1008_b366};
use crate::pass::pass14_funcs::{pass1_1008_3e0e, pass1_1008_3e54, pass1_1008_3e94, pass1_1008_5118, pass1_1008_5236, pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_1008_6978, pass1_1008_6a04};
use crate::pass::pass15_funcs::{pass1_1020_294a, pass1_1020_2a94, pass1_1020_5d56, pass1_1020_61c4, pass1_1020_6498, pass1_1020_64d4};
use crate::pass::pass16_funcs::{pass1_1028_4a9a, pass1_1028_84ca};
use crate::pass::pass17_funcs::{pass1_1030_2f1a, pass1_1030_2fac, pass1_1030_73a8, pass1_1030_8334, pass1_1030_8344, pass1_1030_835a};
use crate::pass::pass19_funcs::{pass1_1018_e100, pass1_1020_022c, pass1_1040_4d7e, pass1_1040_4dcc, pass1_1040_a5d0};
use crate::pass::pass20_funcs::{pass1_1010_a50c, pass1_1010_a568, pass1_1010_a58a, pass1_1010_a5ac, pass1_1010_a5ca, pass1_1010_a5ec, pass1_1010_ac62, pass1_1010_acc0, pass1_1010_ad64, pass1_1010_af66, pass1_1010_c234, pass1_1010_c25e, pass1_1010_c3c2, pass1_1010_ecc6, pass1_1018_04b8, pass1_1018_0ad4, pass1_1018_0ae8, pass1_1018_0afa};
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e4ec};
use crate::pass::pass6_funcs::pass1_1038_b6e0;
use crate::pass::pass7_funcs::{pass1_1018_161c, pass1_1018_1662, pass1_1018_169e, pass1_1018_1a8e, pass1_1018_1c9a, pass1_1018_2580, pass1_1018_25d2, pass1_1018_2afa, pass1_1018_2d22, pass1_1018_2d84, pass1_1018_2d9a, pass1_1018_2dde, pass1_1018_2e28, pass1_1018_2e5e, pass1_1018_2fe8, pass1_1018_30ca, pass1_1018_30fc, pass1_1018_31d0, pass1_1018_39d8, pass1_1018_3a5c, pass1_1018_526a, pass1_1018_57e6};
use crate::pass::pass8_funcs::{pass1_1008_e320, pass1_1008_eb5c, pass1_1010_01f8, pass1_1010_038e, pass1_1010_089e, pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_2ee2, pass1_1010_32c0, pass1_1010_375e, pass1_1010_3770, pass1_1010_3c52, pass1_1010_3cd0, pass1_1010_41d6, pass1_1010_451a, pass1_1010_459e, pass1_1010_4df0, pass1_1010_4e8c, pass1_1010_4f20, pass1_1010_4f30, pass1_1010_5fb0, pass1_1010_659a, pass1_1010_65d0, pass1_1010_6604, pass1_1010_7d38, process_struct_1010_20ba, ret_5_1008_eb6e};
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_1008_392e, pass1_fn_1000_3e2c};
use crate::prog_structs::prog_structs_12::Struct57;
use crate::prog_structs::prog_structs_14::Struct39;
use crate::prog_structs::prog_structs_16::Struct588;
use crate::prog_structs::prog_structs_2::{Struct199, Struct318, Struct665};
use crate::prog_structs::prog_structs_21::Struct12;
use crate::prog_structs::prog_structs_23::win_struct_42;
use crate::prog_structs::prog_structs_24::Struct38;
use crate::prog_structs::prog_structs_25::Struct71;
use crate::prog_structs::prog_structs_26::{Struct339, Struct340, Struct53, Struct58, Struct59};
use crate::prog_structs::prog_structs_27::pass1_struct_2;
use crate::prog_structs::prog_structs_28::{Struct37, Struct40};
use crate::prog_structs::prog_structs_29::{Struct123, Struct41};
use crate::prog_structs::prog_structs_30::{Struct119, Struct124, Struct18};
use crate::prog_structs::prog_structs_31::{Struct15, Struct20, Struct47, Struct5};
use crate::prog_structs::prog_structs_3::Struct663;
use crate::prog_structs::prog_structs_4::{Struct651, Struct652, Struct656};
use crate::prog_structs::prog_structs_6::{Struct673, Struct675};
use crate::prog_structs::prog_structs_7::{Struct376, Struct44, Struct628, Struct629};
use crate::prog_structs::prog_structs_8::{Struct60, Struct647};
use crate::prog_structs::prog_structs_9::{Struct594, Struct604};
use crate::sound_funcs::{mci_fn_1020_08b6, mci_send_cmd_1008_5c5c, mci_send_command_1008_5c7c, mci_send_command_1008_5c9e};
use crate::string_ops::{copy_string_1000_3d3e, get_string_index_1000_3da4, load_string_1008_b65a, process_string_1000_3cea, string_fn_1000_3f9c};
use crate::struct_ops::{process_struct_1000_179c, process_struct_1008_4772, process_struct_1010_6118, process_struct_1020_04f6, process_struct_1020_808e, process_struct_1040_7728, process_struct_1040_a598, process_struct_1040_bf3e};
use crate::sys_ops::{metrics, pass1_1030_838e, process_struct_1040_8478, win_msg};
use crate::sys_ops::metrics::get_sys_metrics_1020_7a50;
use crate::sys_ops::rsrc::load_rsrc_1010_4e9e;
use crate::sys_ops::win::{create_win_1008_9760, win_cleanup_func_1040_b0f8};
use crate::sys_ops::win_msg::{post_win_msg_1020_1ca4, post_win_msg_1040_7b3c, process_win_msg_1008_9510};
use crate::sys_structs::{PAINTSTRUCT16, RECT16};
use crate::typedefs::{ATOM, HANDLE16, HCURSOR16, HINSTANCE16, HPALETTE16, HWND16, LPARAM, LRESULT, SEGPTR, WPARAM16, HDC16, HBRUSH16};
use crate::ui_ops::{color, cursor, dialog, icon, menu, misc, msg_box, ui2};
use crate::ui_ops::misc::{mixed_1040_8520, pass1_1020_289a, pass1_1038_af40, win_fn_1008_3bd6, win_fn_1020_0dc4, win_gui_func_1040_78e2};
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT210, CONCAT212, CONCAT214, CONCAT22, CONCAT24, CONCAT26, CONCAT28, CONCAT31, CONCAT610, SBORROW2, SUB21, SUB41, SUB42, ZEXT24};
use crate::winapi::{BeginPaint16, BringWindowToTop16, CreateSolidBrush16, CreateWindow16, DeleteDC16, DeleteObject16, DestroyIcon16, DestroyMenu16, DestroyWindow16, DrawIcon16, EnableWindow16, EndPaint16, EnumChildWindows16, FillRect16, FreeProcInstance16, GetClassInfo16, GetClientRect16, GetDC16, GetDlgItem16, GetProp16, GetStockObject16, GetSystemMetrics16, GetTextExtent16, GetWindowLong16, GetWindowRect16, GetWindowWord16, InvalidateRect16, IsIconic16, IsWindow16, LoadCursor16, LoadIcon16, MakeProcInstance16, MoveWindow16, PostMessage16, PtInRect16, RegisterClass16, ReleaseCapture16, ReleaseDC16, SelectObject16, SelectPalette16, SendMessage16, SetCapture16, SetCursor16, SetDlgItemText16, SetFocus16, SetWindowPos16, SetWindowText16, ShowWindow16, UpdateWindow16, WinHelp16};

use crate::winapi;

pub fn fill_client_window_1008_39ac(ctx: &mut AppContext, struct_param1: &mut Struct12) {
    let mut unaff_ss: HWND16;
    let mut window_handle_44: HWND16;
    let mut brush: HBRUSH16;
    let mut hdc_var21: HDC16;
    let mut paint_struct: PAINTSTRUCT16 = PAINTSTRUCT16::new();
    let mut rect_var20: RECT16 = RECT16::new();
    hdc_var21 = winapi::BeginPaint(struct_param1.h_window, paint_struct);
    brush = CreateSolidBrush16(0x210070b);
    GetClientRect16(struct_param1.h_window, &mut rect_var20);
    window_handle_44 = hdc_var21;
    FillRect16(  unaff_ss, &rect_var20,brush);
    window_handle_44 = struct_param1.h_window;
    EndPaint16(unaff_ss, &mut paint_struct);
    DeleteObject16(brush);
    return;
}

pub fn set_win_1008_5634(
    param_1: &mut u32,
    param_2: u16,
    param_3_00: WPARAM16,
    param_3: u16,
    param_4: i32,
) {
    let pp_var1: fn();

    let lVar2: u32;
    let mut u_var3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    lVar2 = param_4 << 0x10;
    _local_6 = GetWindowLong16(0, param_4);
    if (((_local_6 >> 0x10) | _local_6) == 0) {
        if (param_3 != 1) {
            DefWindowProc16(CONCAT22(param_2, param_1), param_3_00, param_3, param_4);
            return;
        }
        unsafe {
            _local_6 = *param_1;
        }
        lVar2 = param_4 << 0x10;
        SetWindowLong16(_local_6, (_local_6 >> 0x10));
        pass1_1008_9628(_local_6, param_4);
    }
    pp_var1 = (*_local_6 + 0x1c);
    (**pp_var1)(
        offset,
        _local_6,
        (_local_6 >> 0x10),
        param_1,
        param_2,
        param_3_00,
        param_3,
        lVar2,
        u_var3,
    );
    return;
}

pub fn create_win_1008_5e7e() -> u16 {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let BVar3: bool;
    let AVar4: ATOM;
    let mut window_handle: u16;
    let mut i_var5: i32;
    let mut local_ESI_9: string;
    let pu_var6: *mut u32;
    let unaff_ss: HINSTANCE16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut stock_obj: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let local_12: [u32; 4];

    pu_var6 = local_12;
    local_ESI_9._0_2_ = s_MciSoundWindow_1050_02bd;
    i_var5 = 3;
    while (i_var5 != 0) {
        i_var5 = i_var5 + -1;
        pu_var2 = pu_var6;
        pu_var6 = pu_var6 + 1;
        pu_var1 = local_ESI_9;
        local_ESI_9._0_2_ = local_ESI_9 + 1;
        unsafe {
            *pu_var2 = *pu_var1;
        }
    }
    pu_var6 = local_ESI_9;
    *(pu_var6 + 2) = *(local_ESI_9 + 2);
    local_2c = 0x2000;
    local_2a = &u16_1050_5f44;
    local_28 = &ctx.PTR_LOOP_1050_1008;
    local_24 = 2;
    local_22 = ctx.g_h_instance_1050_038c;
    local_20 = 0;
    local_1e = 0;
    local_26 = 0;
    stock_obj = GetStockObject16(0);
    local_1a = 0;
    local_16 = local_12;
    BVar3 = GetClassInfo16(&local_2c, CONCAT22(local_16, unaff_ss), unaff_ss);
    if (BVar3 == 0) {
        AVar4 = RegisterClass16(&local_2c);
        if (AVar4 == 0) {
            OutputDebugString16(s_MciSound_registerClass_failed_1050_02cc);
            return 0;
        }
    }
    window_handle = CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0,
        ctx.g_h_window,
        1,
        1,
        -0x8000,
        -0x8000,
        0xcf0000,
        s_MciSound_registerClass_failed_1050_02cc + 0x1e,
        CONCAT22(unaff_ss, local_12),
    );
    return window_handle;
}

pub fn destroy_win_1008_628e(param_1: &mut Struct594, param_2: u16) {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 0xd2) + 0x14);
    (**pp_var1)();
    DestroyWindow16((param_1 + 8));
    (param_1 + 8) = 0;
    return;
}

pub fn show_window_1008_68c6(param_1: &mut Struct628, param_2: u16, param_3: u16) -> u16 {
    let mut local_AX_13: u16;
    let mut local_4: u16;

    local_AX_13 = show_window_1008_96ae(CONCAT22(param_2, param_1), param_3);
    pass1_1008_6a04(param_1, param_2);
    return local_AX_13;
}

pub fn set_window_text_1008_9664(param_1: u32, param_2: u16, param_3: String) {
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 10)), param_3);
    SetWindowText16(param_1 & 0xffff0000 | (param_1 + 10), (param_1 + 8));
    return;
}

pub fn destroy_wiin_1008_9698(param_1: u32) {
    DestroyWindow16((param_1 + 8));
    return;
}

pub fn show_window_1008_96ae(param_1: u32, param_2: u16) -> bool {
    let b_var1: bool;
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    if ((param_1 + 8) != 0) {
        b_var1 = ShowWindow16(param_2, (param_1 + 8));
        return b_var1;
    }
    return 0;
}

pub fn enable_window_1040_d6be(param_1: u32) {
    let HVar1: HWND16;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    HVar1 = GetDlgItem16(1, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16(2, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_vrpal_bmp_1050_183a + 7), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 4), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 5), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 6), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 7), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    (i_var2 + 0xa0) = 1;
    return;
}

pub fn enable_window_1040_d60e(in_Struct588_ptr_1: &mut Struct588) -> u8 {
    let HVar1: HWND16;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;

    u_var4 = (in_Struct588_ptr_1 >> 0x10);
    i_var3 = in_Struct588_ptr_1;
    HVar1 = GetDlgItem16(1, (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16(2, (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_vrpal_bmp_1050_183a + 7), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 4), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 5), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 6), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 7), (i_var3 + 6));
    BVar2 = EnableWindow16(1, HVar1);
    (i_var3 + 0xa0) = 0;
    return BVar2;
}

pub fn enable_window_1040_cf1c(in_Struct123: &mut Struct123) -> LRESULT {
    let h_wnd: HWND16;
    let local_Struct123: Struct123;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut u_var3: u32;
    let LVar4: LRESULT;
    let enable: bool;
    let mut u_var5: u16;
    let mut buffer_50c: [u8; 1026];
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut buffer_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(buffer_50c._0_2_, 3));
    u_var1 = (in_Struct123 >> 0x10);
    local_Struct123 = in_Struct123;
    pass1_1010_c3c2(
        ppVar2,
        (ppVar2 >> 0x10),
        CONCAT22(unaff_ss, buffer_106),
        local_Struct123.field_0x94,
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_ss, buffer_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        local_Struct123.h_wnd_0x6,
    );
    // msg = WM_SETREDRAW
    SendDlgItemMessage16(
        0,
        0,
        0xb,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    // WM_PSD_ENVSTAMPRECT
    SendDlgItemMessage16(
        0,
        0,
        0x405,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    u_var5 = SUB42(s_vrpal_bmp_1050_183a + 8, 0);
    u_var3 = pass1_1018_526a(local_Struct123.field_0x98, local_Struct123.field_0x94);
    dialog::send_dialog_item_msg_1040_ce12(in_Struct123, u_var3, u_var5);
    // 040c   1036   SB_GETTEXTLENGTHW
    // 040c   1036   TB_ISBUTTONHIDDEN
    LVar4 = SendDlgItemMessage16(
        0,
        0,
        0x40c,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    if (((LVar4 >> 0x10) != 0 && -1 < LVar4) || (-1 < LVar4 && (LVar4 != 0))) {
        h_wnd = GetDlgItem16(1, local_Struct123.h_wnd_0x6);
        enable = 1;
    } else {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, buffer_50c),
            0x44a,
        );
        // 0401   1025   DDM_DRAW
        // 0401   1025   HKM_SETHOTKEY
        // 0401   1025   TB_ENABLEBUTTON
        // 0401   1025   WM_CHOOSEFONT_GETLOGFONT
        // 0401   1025   WM_PSD_FULLPAGERECT
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, buffer_50c),
            0,
            0x401,
            (s_vrpal_bmp_1050_183a + 8),
            local_Struct123.h_wnd_0x6,
        );
        h_wnd = GetDlgItem16(1, local_Struct123.h_wnd_0x6);
        enable = 0;
    }
    EnableWindow16(enable, h_wnd);
    // WM_SETREDRAW
    LVar4 = SendDlgItemMessage16(
        0,
        1,
        0xb,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    return LVar4;
}

pub fn enable_window_1040_cc66(param_1: &mut Struct123) {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 0x98) + 0x10);
    (**pp_var1)();
    enable_window_1040_cf1c(param_1);
    return;
}

pub fn win_gui_fn_1040_cc8c(param_1: &mut Struct124, param_2: u16, param_3: u16, param4: u32) {
    if (param_3._2_2_ == 0xeb) {
        enable_window_1040_cf1c(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ == (s_vrpal_bmp_1050_183a + 7)) {
            msg_box::display_msg_box_1040_cce4(CONCAT22(param_2, param_1));
        } else {
            if (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 8)) {
                win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
                return;
            }
            if (param_3 == 1) {
                dialog::send_dlg_item_msg_1040_ce76(CONCAT22(param_2, param_1));
            }
        }
    }
    return;
}

pub fn win_fn_1040_bbe2(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: i32;
    let b_var5: bool;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let h_wnd: HWND16;
    let mut u_var8: u16;
    let unaff_ss: HWND16;
    let pp_var9: *mut pass1_struct_1;
    let pu_var10: Vec<u8>;
    let u_var11: u8;
    let u_var12: u8;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ != 0x10c) {
        if (param_2._2_2_ < 0x10d) {
            if (param_2._2_2_ == 0xfa) {
                ppc_var2 = (&param_1[1].field_0x4 + 0x18);
                ppc_var2();
                return;
            }
            if (param_2._2_2_ == 0x10a) {
                GetClientRect16(CONCAT22(unaff_ss, &local_a), &param_1.field_0x6);
                u_var3 = &param_1[1].field_0x4;
                local_8 = local_8 + 3;
                local_a = (u_var3 + 0x1a) - 9;
                local_6 = local_6 - 3;
                local_4 = local_4 - 3;
                InvalidateRect16(1, &local_a, unaff_ss);
                destroy_win_1010_2fa0(&param_1[1].field_0x4);
                pass1_1010_32c0(&param_1[1].field_0x4, 0);
                u_var3 = &param_1[1].field_0x4;
                local_22 = (u_var3 >> 0x10);
                pass1_1010_2ee2(u_var3, local_22);
                return;
            }
            if (param_2._2_2_ != 0x10b) {
                // LAB_1040_be78:
                win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                return;
            }
            u_var3 = &param_1[1].field_0x4;
            u_var6 = (u_var3 + 0x12);
            u_var7 = u_var6;
            pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var6, 3));
            u_var8 = (pp_var9 >> 0x10);
            i_var4 = pass1_1010_a5ca(pp_var9, u_var8, u_var7);
            if ((u_var6 != 0x70) && (i_var4 == 0)) {
                return;
            }
            u_var3 = &param_1[1].field_0x1c;
            u_var13 = u_var3;
            u_var14 = (u_var3 >> 0x10);
            u_var3 = &param_1[1].field_0x4;
            u_var1 = (u_var3 + 0x12);
            u_var11 = u_var1;
            u_var12 = (u_var1 >> 8);
        } else {
            if (param_2._2_2_ != 0x10d) {
                if (param_2._2_2_ == 0x10e) {
                    pp_var9 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(local_22, 0x32),
                    );
                    i_var4 = misc::win_gui_fn_1010_79aa(pp_var9, 0xfc6, &param_1[1].field_0x1c);
                    if (i_var4 != 0) {
                        return;
                    }
                    u_var3 = &param_1[1].field_0x1c;
                    window_msg_func_1010_7300(pp_var9, 0, 0, 0x13, u_var3, (u_var3 >> 0x10));
                    return;
                }
                if (param_2._2_2_ != 0xbbb) {
                    if (param_2._2_2_ == 0xbbc) {
                        pp_var9 = process_struct_1010_20ba(
                            ctx._g_Struct372_1050_0ed0,
                            CONCAT22(local_22, 3),
                        );
                        u_var8 = (pp_var9 >> 0x10);
                        u_var6 = pp_var9;
                        u_var7 = pass1_1010_a5ac(u_var6, u_var8, &param_1[1].field_0x1c);
                        i_var4 = pass1_1010_a58a(u_var6, u_var8, u_var7);
                        if (i_var4 == 0) {
                            pass1_1010_a568(u_var6, u_var8, u_var7);
                        }
                        h_wnd = GetDlgItem16(0xbbc, &param_1.field_0x6);
                        EnableWindow16(0, h_wnd);
                        return;
                    }
                    // goto LAB_1040_be78;
                }
                if ((&param_1[1].field_0x22 == 0)
                    || (b_var5 = IsWindow16(&param_1[1].field_0x22), b_var5 == 0))
                {
                    pu_var10 = misc::pass1_1038_af40(ctx._g_Struct112_a, *&param_1.field_0x6, 0x1b);
                    &param_1[1].field_0x22 = (pu_var10 + 6);
                    ShowWindow16(1, &param_1[1].field_0x22);
                    return;
                }
                local_22 = &param_1[1].field_0x22;
                // goto LAB_1040_bd39;
            }
            pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_22, 3));
            u_var8 = (pp_var9 >> 0x10);
            u_var3 = &param_1[1].field_0x1c;
            u_var13 = u_var3;
            u_var14 = (u_var3 >> 0x10);
            u_var11 = 0x71;
            u_var12 = 0;
        }
        local_1e = pp_var9;
        pass1_1010_a5ec(
            local_1e,
            u_var8,
            CONCAT11(u_var12, u_var11),
            CONCAT22(u_var14, u_var13),
        );
        if ((&param_1[1].field_0x20 != 0)
            && (b_var5 = IsWindow16(&param_1[1].field_0x20), b_var5 != 0))
        {
            SendMessage16(0, 0xeb, 0x111, &param_1[1].field_0x20);
        }
    }
    local_22 = &param_1.field_0x6;
    // LAB_1040_bd39:
    DestroyWindow16(local_22);
    return;
}

pub fn destroy_window_1040_bb78(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let b_var4: bool;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_cs: u16;

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    if ((i_var5 + 0xb6) != 0) {
        unaff_cs = SUB42(offset, 0);
        b_var4 = IsWindow16((i_var5 + 0xb6));
        if (b_var4 != 0) {
            unaff_cs = SUB42(offset, 0);
            DestroyWindow16((i_var5 + 0xb6));
        }
    }
    (i_var5 + 0xb6) = 0;
    pu_var1 = (i_var5 + 0x94);
    u_var2 = (i_var5 + 0x96);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    (i_var5 + 0x94) = 0;
    (i_var5 + 0x98) = 0;
    return;
}

pub fn set_window_pos_1040_b8d2(ctx: &mut AppContext, param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut i32_var6: i32;

    let struct_a: Struct199;
    let pa_var7: Struct199;
    // let ctx.dx_reg: Struct199;
    let pa_var8: Struct199;
    // let ctx.dx_reg: Struct199;
    // let ctx.dx_reg: Struct199;
    // let ctx.dx_reg: Struct199;
    let extraout_dx_04: Struct199;
    let extraout_dx_05: Struct199;
    let mut extraout_dx_06: u16;
    let mut u_var9: u16;
    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let ppVar13: pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    ppVar13 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x31),
    );
    pa_var8 = (ppVar13 >> 0x10);
    u_var3 = ppVar13;
    u_var11 = (param_1 >> 0x10);
    i_var10 = param_1;
    (i_var10 + 0x98) = u_var3;
    (i_var10 + 0x9a) = pa_var8;
    process_struct_1000_179c(10, pa_var8);
    if (pa_var8 | u_var3) == 0 {
        (i_var10 + 0x94) = 0;
    } else {
        process_struct_1040_bf3e(CONCAT22(pa_var8, u_var3), (i_var10 + 6));
        (i_var10 + 0x94) = u_var3;
        (i_var10 + 0x96) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var10 + 0x94), (i_var10 + 0x98));
    pa_var8 = struct_a;
    process_struct_1000_179c(0x42, struct_a);
    pa_var7 = (pa_var8 | u_var3);
    if pa_var7 != 0x0 {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var7 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var7);
    pa_var8 = (pa_var7 | u_var3);
    if (pa_var8 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var7,
            1,
            0xa0028,
            0x850000,
            0x10b0084,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var8 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var8);
    pa_var7 = (pa_var8 | u_var3);
    if (pa_var7 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa0046,
            0x870000,
            0x10d0086,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var7 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var7);
    pa_var8 = (pa_var7 | u_var3);
    if (pa_var8 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var7,
            1,
            0xa0064,
            0x890000,
            0x10e0088,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var8 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var8);
    pa_var7 = (pa_var8 | u_var3);
    if (pa_var7 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa0082,
            0x830000,
            0x10c0082,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var7 = extraout_dx_04;
    }
    process_struct_1000_179c(0x42, pa_var7);
    pa_var8 = (pa_var7 | u_var3);
    if (pa_var8 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var7,
            1,
            0xa00d2,
            0x8b0000,
            0xbbb008a,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var8 = extraout_dx_05;
    }
    process_struct_1000_179c(0x42, pa_var8);
    if ((pa_var8 | u_var3) == 0) {
        u_var3 = 0;
        u_var12 = 0;
    } else {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa00a0,
            0x8d008e,
            0xbbc008c,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        u_var12 = extraout_dx_06;
    }
    ppVar13 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffe4, 3));
    u_var9 = (ppVar13 >> 0x10);
    u_var2 = ppVar13;
    u_var4 = pass1_1010_a5ac(u_var2, u_var9, (i_var10 + 0xb0));
    u_var5 = pass1_1010_ac62(u_var2, u_var9, 0x1e);
    if (u_var5 != 0) {
        pass1_1010_a5ca(u_var2, u_var9, u_var4);
        if (0 < u_var5) {
            pass1_1010_a58a(u_var2, u_var9, u_var4);
            if (u_var5 == 0) {}
            // goto LAB_1040_bb26;
        }
    }
    enable_window_1040_9234(u_var3, u_var12, 0);
    // LAB_1040_bb26:
    u_var1 = (i_var10 + 0x98);
    i32_var6 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var12 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i32_var6 + 0x10),
        (i32_var6 + 0xe),
        (i32_var6 + 0xc),
        (u_var1 | i32_var6 + 10),
        0,
        (i_var10 + 6),
    );
    return;
}

pub fn destroy_window_1040_b726(param_1: &mut u32, param_2: i32) {
    let pp_var1: fn();

    if (param_2 != 0) {
        unsafe {
            pp_var1 = (*param_1 + 0x78);
        }
        (**pp_var1)();
    }
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn win_gui_fn_1040_b54a(param_1: &mut Struct124, param_2: u16, param_3: u16, param_2_00: u32) {
    let in_struct_1: &mut Struct44;
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let pa_var5: &mut Struct44;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    let u_var9: u8;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut fn_ptr_2: u32;

    if (param_2_00._2_2_ == 0xea) {
        fn_ptr_2 = (CONCAT22(param_2, param_1) + 0x5c);
        (**fn_ptr_2)();
    } else {
        if (param_2_00._2_2_ == 0xeb) {
            pp_var8 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffe6, 3),
            );
            in_struct_1 = &param_1.field_0x90;
            if (in_struct_1 != 0x0) {
                u_var7 = (in_struct_1 >> 0x10);
                u_var10 = 0x1010;
                pa_var5 = in_struct_1;
                pass1_1010_ad64(
                    pp_var8,
                    CONCAT22((in_struct_1 + 10), (pp_var8 >> 0x10)),
                    (in_struct_1 + 6),
                );
                param_1.field_0x90 = pa_var5;
                &param_1.field_0x92 = ctx.dx_reg;
                if ((ctx.dx_reg | param_1.field_0x90) == 0) {
                    &param_1.field_0x90 = in_struct_1;
                } else {
                    if (in_struct_1 != 0x0) {
                        pass1_1040_a5d0(in_struct_1);
                        u_var10 = 0x1000;
                        error_check_1000_17ce(in_struct_1);
                    }
                    pp_var1 = (CONCAT22(param_2, param_1) + 0x70);
                    (**pp_var1)(u_var10, param_1, param_2);
                }
            }
        } else {
            if (param_2_00._2_2_ == 0x1790) {
                pp_var8 = process_struct_1010_20ba(
                    ctx._g_Struct372_1050_0ed0,
                    CONCAT22(in_stack_0000ffe6, 0x32),
                );
                u_var3 = pp_var8;
                u_var2 = &param_1.field_0x90;
                u_var2 = (u_var2 + 6);
                pass1_1010_7d38(u_var3, (pp_var8 >> 0x10), u_var2, (u_var2 >> 0x10));
                u_var4 = u_var3;
                misc::win_gui_fn_1010_79aa(pp_var8, 0xfab, 0);
                if (u_var4 != 0) {
                    return;
                }
                if (u_var3 == 0) {
                    u_var2 = &param_1.field_0x90;
                    u_var7 = (u_var2 >> 0x10);
                    i32_var6 = u_var2;
                    u_var2 = (i32_var6 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 0x14;
                } else {
                    u_var2 = &param_1.field_0x90;
                    u_var7 = (u_var2 >> 0x10);
                    i32_var6 = u_var2;
                    u_var2 = (i32_var6 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 9;
                }
                u_var9 = u_var7;
            } else {
                if (param_2_00._2_2_ == 0x1824) {
                    pp_var8 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffe6, 0x32),
                    );
                    i32_var6 = pp_var8;
                    u_var2 = &param_1.field_0x90;
                    misc::win_gui_fn_1010_79aa(pp_var8, 0xfc5, (u_var2 + 6));
                    if (i32_var6 != 0) {
                        return;
                    }
                    u_var2 = &param_1.field_0x90;
                    u_var2 = (u_var2 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 0x12;
                    u_var9 = 0;
                } else {
                    if (param_2_00._2_2_ != 0x1830) {
                        post_win_msg_1040_7b3c(param_1, param_2, param_3, param_2_00);
                        return;
                    }
                    pp_var8 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffe6, 0x32),
                    );
                    i32_var6 = pp_var8;
                    u_var2 = &param_1.field_0x90;
                    misc::win_gui_fn_1010_79aa(pp_var8, 0xfb6, (u_var2 + 6));
                    if (i32_var6 != 0) {
                        return;
                    }
                    u_var2 = &param_1.field_0x90;
                    u_var2 = (u_var2 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 0xc;
                    u_var9 = 0;
                }
            }
            window_msg_func_1010_7300(pp_var8, i32_var6, u_var9, u_var10, u_var11, u_var12);
        }
    }
    return;
}

pub fn show_win_1040_b43c(param_1: &mut u32) {
    let pp_var1: fn();

    unsafe {
        pp_var1 = (*param_1 + 0x70);
    }
    (**pp_var1)();
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn win_gui_fn_1040_b4c8(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x90) != 0) {
        ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_6, 0x32));
        u_var1 = (param_1 + 0x90);
        i_var2 = (u_var1 + 10);
        if (i_var2 == 4) {
            i_var2 = misc::win_gui_fn_1010_79aa(ppVar4, 0xfd9, 0);
            if (i_var2 == 0) {
                u_var3 = 0xe;
                // LAB_1040_b50f:
                window_msg_func_1010_7300(ppVar4, i_var2, i_var2, u_var3, i_var2, i_var2);
                return;
            }
        } else {
            if (((0 < i_var2 + -5) && (!SBORROW2(i_var2 + -5, 1))) && (i_var2 + -6 < 2)) {
                i_var2 = misc::win_gui_fn_1010_79aa(ppVar4, 0xfda, 0);
                if (i_var2 == 0) {
                    u_var3 = 0xd;
                    // goto LAB_1040_b50f;
                }
            }
        }
    }
    return;
}

pub fn set_window_pos_1040_b230(param_1: &mut Struct20) {
    let pp_var1: fn();
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: HWND16;
    let pu_var5: *mut u16;
    let u_var6: u8;
    let u_var7: u8;
    let pu_var8: *mut u16;
    let HVar9: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    if (PTR_LOOP_1050_5ef8 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        PTR_LOOP_1050_5ef8 = 0x0;
    }
    pu_var8 = &local_4;
    pu_var5 = &local_6;
    u_var6 = unaff_ss;
    u_var7 = (unaff_ss >> 8);
    HVar9 = unaff_ss;
    _local_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var5, 0x48));
    pass1_1008_3e94(
        (_local_a + 0xe),
        CONCAT13(u_var7, CONCAT12(u_var6, pu_var5)),
        CONCAT22(HVar9, pu_var8),
    );
    u_var4 = (_local_a >> 0x10);
    local_c = (_local_a + 10);
    local_e = (_local_a + 0xc);
    i_var2 = GetSystemMetrics16(4);
    i_var3 = i_var2 * PTR_LOOP_1050_5ef8 + 10;
    PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
    _local_12 = CONCAT22(i_var3 + local_4, i_var3 + local_6);
    u_var4 = (param_1 >> 0x10);
    GetWindowRect16(CONCAT22(&local_1a, 0x1538), unaff_ss);
    if (local_e < ((local_14 - local_18) + local_12)) {
        _local_12 = _local_12 & 0xffff0000 | (0xfffe - ((local_14 - local_18) - local_e));
    }
    if (local_c < ((local_16 - local_1a) + local_10)) {
        _local_12 = _local_12 & 0xffff | (0xfffe - ((local_16 - local_1a) - local_c)) << 0x10;
    }
    SetWindowPos16(1, 0, 0, _local_12, (_local_12 >> 0x10), 0, (param_1 + 6));
    pp_var1 = (param_1 + 0x6c);
    (**pp_var1)(offset, param_1, u_var4);
    return;
}

pub fn set_win_placement_1010_0070(param_1: u32, param_2: i32, param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let pu_var3: *mut u32;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let pu_var5: *mut u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_18 = 0x16;
    _local_16 = 0;
    local_12 = 0;
    local_10 = 0;
    local_e = 0;
    local_c = 0;
    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    pu_var5 = &local_18;
    GetWindowPlacement16();
    if ((local_10 == 0xffff) || (param_2 != 0)) {
        _local_16 = 0x50001;
        lVar4 = GetWindowLong16(0, param_3);
        u_var2 = (lVar4 >> 0x10);
        pu_var3 = (lVar4 + 0xe0);
        unsafe {
            pp_var1 = (*pu_var3 + 0x38);
        }
        (**pp_var1)(offset, pu_var3, (lVar4 + 0xe2), pu_var5);
        pass1_1010_01f8(param_1, CONCAT22(unaff_ss, &local_18), pu_var3);
        SetWindowPlacement16();
    }
    return;
}

pub fn set_win_placement_1010_010e(param_1: u16, param_2: u16, param_3: HWND16) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let pu_var3: *mut u16;
    let mut u_var4: u16;
    let pu_var5: *mut u32;

    let lVar6: u32;
    let pu_var7: *mut u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_18 = 0x16;
    local_16 = 0;
    local_14 = 0;
    local_12 = 0;
    local_10 = 0;
    local_e = 0;
    local_c = 0;
    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    pu_var7 = &local_18;
    GetWindowPlacement16();
    if (local_a == 0xffff) {
        lVar6 = GetWindowLong16(0, param_1);
        u_var4 = (lVar6 >> 0x10);
        pu_var5 = (lVar6 + 0xe0);
        unsafe {
            pp_var1 = (*pu_var5 + 0x1c);
        }
        (**pp_var1)(offset, pu_var5, (lVar6 + 0xe2), pu_var7);
        i_var2 = pu_var5;
        pu_var3 = (pu_var5 & 0xffff | ctx.dx_reg << 0x10);
        local_14 = 9;
        unsafe {
            local_a = *pu_var3;
        }
        local_8 = (i_var2 + 2);
        unsafe {
            local_6 = (i_var2 + 4) + *pu_var3;
        }
        local_4 = (i_var2 + 2) + (i_var2 + 6);
        SetWindowPlacement16(offset, &local_18);
    }
    return;
}

pub fn destroy_win_1010_2fa0(param_1: &mut Struct340) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let local_bx_7: Struct340;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_7 = param_1;
    local_bx_7.field_0x28 = 0;
    local_4 = 0;
    loop {
        pu_var1 = &local_bx_7.field_0x16;
        if (unsafe { *pu_var1 == local_4 } || unsafe { *pu_var1 < local_4 }) {
            break;
        }
        u_var2 = (&local_bx_7.field_0x2a + local_4 * 4);
        DestroyWindow16((u_var2 + 0x18));
        local_4 = local_4 + 1;
    }
    local_bx_7.field_0x16 = 0;
    if ((&local_bx_7.field_0x54 | local_bx_7.field_0x52) != 0) {
        local_4 = 0;
        while {
            u_var2 = &local_bx_7.field_0x52;
            if ((u_var2 + local_4 * 4) != 0) {
                u_var2 = &local_bx_7.field_0x52;
                u_var2 = (u_var2 + local_4 * 4);
                DestroyWindow16((u_var2 + 0x18));
                u_var2 = &local_bx_7.field_0x52;
                (u_var2 + local_4 * 4) = 0;
            }
            local_4 = local_4 + 1;
            local_4 < 10
        } {}
        error_check_1000_17ce(&local_bx_7.field_0x52);
        &local_bx_7.field_0x52 = 0;
    }
    return;
}

pub fn show_window_1010_7a76(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x20) == 0) {
        (i_var2 + 0x20) = 1;
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            u_var1 = (lVar4 + 8);
            ShowWindow16(0, (u_var1 + 6));
        }
    }
    return;
}

pub fn show_window_1010_7ace(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x20) != 0) {
        (i_var2 + 0x20) = 0;
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            u_var1 = (lVar4 + 8);
            ShowWindow16(1, (u_var1 + 6));
        }
    }
    return;
}

pub fn destroy_win_1010_7b26(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;
    let pu_var2: Vec<u8>;

    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (((i_var3 + 0x1e) | (i_var3 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var3 + 0x1c));
        while {
            pu_var2 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
            if ((ctx.dx_reg | pu_var2) == 0) {
                break;
            }
            (pu_var2 + 4) != param_2
        } {}
        if ((ctx.dx_reg | pu_var2) != 0) {
            u_var1 = (pu_var2 + 8);
            DestroyWindow16((u_var1 + 6));
        }
    }
    return;
}

pub fn set_window_text_1018_6066(param_1: u32, in_window_text: u32, in_dlg_item_id: u16) {
    let hwnd: HWND16;

    hwnd = GetDlgItem16(in_dlg_item_id, (param_1 + 6));
    SetWindowText16(in_window_text, hwnd);
    return;
}

pub fn set_window_text_1018_6630(in_struct_604_ptr: &mut Struct604) {
    let mut in_dlg_item_id: u16;
    let mut u_var1: i32;
    let struct_604_ptr_1: Struct604;
    let struct_60_ptr_hi: Struct604;
    let mut local_c: u16;
    let window_text: SEGPTR;
    let mut local_4: u16;
    let struct_60_ptr_1: Struct60;

    struct_60_ptr_hi = (in_struct_604_ptr >> 0x10);
    struct_604_ptr_1 = in_struct_604_ptr;
    window_text = load_rsrc_1010_4e9e(struct_604_ptr_1.Struct60_ptr);
    if (window_text != 0) {
        local_c = 0;
        while (local_c < 9) {
            struct_60_ptr_1 = struct_604_ptr_1.Struct60_ptr;
            in_dlg_item_id = pass1_1010_4f20(struct_60_ptr_1, (struct_60_ptr_1 >> 0x10), local_c);
            set_window_text_1018_6066(struct_604_ptr_1.field_0xa, window_text, in_dlg_item_id);
            u_var1 = get_string_index_1000_3da4(window_text);
            window_text = window_text & 0xffff0000 | (window_text + u_var1 + 1);
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn destroy_win_1018_c518(in_struct_376_1: &mut Struct376) {
    let b_rc: bool;
    let struct_a_1: Struct376;
    let struct_a_2: Struct376;
    let temp_5fa5d31fd0: Struct376;

    struct_a_2 = (in_struct_376_1 >> 0x10);
    struct_a_1 = in_struct_376_1;
    in_struct_376_1.ptr_a_lo = 0xc8bc;
    struct_a_1.ptr_a_hi = 0x1018;
    error_check_1000_17ce(&struct_a_1.struct_44_a);
    if (struct_a_1.window_handle_a != 0) {
        b_rc = IsWindow16(struct_a_1.window_handle_a);
        if (b_rc != 0) {
            DestroyWindow16(struct_a_1.window_handle_a);
            struct_a_1.window_handle_a = 0;
        }
    }
    pass1_1020_022c(in_struct_376_1);
    return;
}

pub fn destroy_win_fn_1018_c896(param_1: &mut Struct376, param_2: u8) -> &mut Struct376 {
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn create_win_fn_1018_df40(in_struct_42_ptr_1: &mut win_struct_42) {
    let local_struct_42_1: *mut win_struct_42;
    let mut local_struct_42_hi_1: u16;
    let create_win_result: Struct199;
    let mut local_u32_43: u32;
    let mut local_4: u16;

    create_win_result = create_win_1008_9760(in_struct_42_ptr_1);
    process_struct_1000_179c(10, (create_win_result >> 0x10));
    local_struct_42_1 = in_struct_42_ptr_1;
    local_struct_42_hi_1 = (in_struct_42_ptr_1 >> 0x10);
    if (create_win_result != 0x0) {
        local_u32_43 = pass1_1018_e100(create_win_result, local_struct_42_1.win_handle_0x8);
        local_struct_42_1.u16_xe2 = local_u32_43;
        local_struct_42_1.u16_xe4 = (local_u32_43 >> 0x10);
        return;
    }
    &local_struct_42_1.u16_xe2 = 0;
    return;
}

pub fn destroy_win_fn_1018_df92(param_1: &mut Struct594) {
    let pu_var1: *mut u32;
    let pvVar2: &mut Vec<u8>;
    let local_struct_594_ptr_1: Struct594;
    let mut unaff_si: u16;
    let mut u_var3: u16;
    let temp_862fc4681f0: *mut u32;
    let local_fn_ptr_1: fn();

    destroy_win_1008_628e(param_1, unaff_si);
    u_var3 = (param_1 >> 0x10);
    local_struct_594_ptr_1 = param_1;
    pu_var1 = local_struct_594_ptr_1.u32_xE2;
    pvVar2 = local_struct_594_ptr_1.vptr_xE4;
    if ((pvVar2 | pu_var1) != 0) {
        unsafe {
            local_fn_ptr_1 = *pu_var1;
        }
        (**local_fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, pu_var1, pvVar2, 1);
    }
    &local_struct_594_ptr_1.u32_xE2 = 0;
    return;
}

pub fn destroy_win_fn_1018_e72a(param_1: &mut Struct594) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut in_stack_0000fff6: u16;
    Struct594 * *temp_86277dbc59c;
    let fn_ptr_1: fn();

    u_var3 = (param_1 >> 0x10);
    pu_var1 = (param_1 + 0xee);
    u_var2 = (param_1 + 0xf0);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    destroy_win_1008_628e(param_1, in_stack_0000fff6);
    return;
}

pub fn update_window_1020_10a0(param_1: &mut win_struct_42) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let pi_var4: *mut u16;
    let pa_var5: Struct199;

    let paVar6: Struct199;
    let ctx.dx_reg: Struct199;
    let struct_a: Struct199;


    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let paVar9: Struct199;
    let ppVar10: *mut pass1_struct_1;
    let mut u_var11: u32;
    let mut u_var12: u16;
    let local_3a: Struct71;
    let mut local_8: u16;

    paVar9 = create_win_1008_9760(param_1);
    paVar6 = (paVar9 >> 0x10);
    u_var3 = paVar9;
    process_struct_1000_179c(0x42, paVar6);
    pa_var5 = (paVar6 | u_var3);
    i_var7 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (pa_var5 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            paVar6,
            0,
            0x1f009b,
            0x750000,
            0xf10074,
            CONCAT22(unaff_si, (i_var7 + 8)),
        );
        pa_var5 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var5);
    paVar6 = (pa_var5 | u_var3);
    if (paVar6 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            pa_var5,
            0,
            0x31009b,
            0x770000,
            0xf20076,
            CONCAT22(unaff_si, (i_var7 + 8)),
        );
        paVar6 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | u_var3) != 0) {
        misc::win_fn_1008_3bd6(
            u_var3,
            paVar6,
            0,
            0x77009b,
            0x790000,
            0xf30078,
            CONCAT22(unaff_si, (i_var7 + 8)),
        );
    }
    ppVar10 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2d));
    u_var12 = (ppVar10 >> 0x10);
    (i_var7 + 0xf2) = ppVar10;
    (i_var7 + 0xf4) = u_var12;
    u_var3 = (i_var7 + 0xf2);
    (i_var7 + 0xe0) = u_var3;
    (i_var7 + 0xe2) = u_var12;
    LoadIcon16(
        0x1010,
        s_PLNTICON_1050_4267,
        &ctx.g_alloc_addr_1050_1050,
        ctx.g_h_instance_1050_038c,
    );
    (i_var7 + 0xc2) = u_var3;
    u_var1 = (i_var7 + 0xf2);
    u_var12 = u_var1;
    ppc_var2 = ((i_var7 + 0xf2) + 0x30);
    ppc_var2(offset, u_var12, (u_var1 >> 0x10), u_var3);
    pa_var5 = struct_a;
    process_struct_1000_179c(0x24, struct_a);
    if ((pa_var5 | u_var3) == 0) {
        (i_var7 + 0xf6) = 0;
    } else {
        misc::get_dc_1020_1418(u_var3, pa_var5, param_1, u_var8);
        (i_var7 + 0xf6) = u_var3;
        (i_var7 + 0xf8) = ctx.dx_reg;
    }
    (i_var7 + 0xe8) = (i_var7 + 0xf6);
    ppVar10 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var12, 0x2f));
    u_var11 = pass1_1018_04b8(ppVar10);
    pass1_1010_41d6((i_var7 + 0xf2), u_var11);
    u_var11 = pass1_1010_451a((i_var7 + 0xf2));
    pi_var4 = u_var11;
    u_var1 = param_1;
    ppc_var2 = (u_var1 + 0x14);
    ppc_var2(0x1010, i_var7, u_var8, 0, u_var11, (u_var11 >> 0x10));
    u_var12 = 1;
    ppc_var2 = (u_var1 + 0x10);
    ppc_var2();
    pass1_1010_459e((i_var7 + 0xf2));
    ppc_var2 = ((i_var7 + 0xf2) + 0x10);
    ppc_var2(0x1010, (i_var7 + 0xf2), param_1, u_var12);
    MoveWindow16(
        1,
        pi_var4[3],
        pi_var4[2],
        pi_var4[1],
        unsafe { *pi_var4 },
        (i_var7 + 8),
    );
    UpdateWindow16((i_var7 + 8));
    return;
}

pub fn create_window_1008_0af8(in_win_struct: &mut win_struct_42) -> u16 {
    let pu_var1: *mut u16;
    let ppc_var2: fn();
    let local_win_handle: HANDLE16;
    let struct_a: Struct199;
    let paVar3: Struct199;

    let mut local_DX_85: i32;
    let struct_a_00: Struct199;
    let mut local_DX_175: i32;

    let local_win_struct_42: Struct119;
    let mut local_es_13: u16;
    let mut local_CS_138: u16;
    let paVar4: Struct199;
    let local_16: u32;
    let mut local_6: u16;
    let mut temp_5fdbebe016: i32;
    char * *fn_ptr_1;
    let mut temp_5fc4833446: u32;

    paVar4 = create_win_1008_9760(in_win_struct);
    struct_a = (paVar4 >> 0x10);
    local_es_13 = (in_win_struct >> 0x10);
    local_win_struct_42 = in_win_struct;
    local_win_handle = local_win_struct_42.win_handle_0x8;
    ctx.g_h_window = local_win_handle;
    process_struct_1000_179c(0x12, struct_a);
    paVar3 = (struct_a | local_win_handle);
    if (paVar3 != 0x0) {
        set_timer_1008_91ba(local_win_handle, struct_a);
        paVar3 = ctx.dx_reg;
    }
    process_struct_1000_179c(6, paVar3);
    if ((paVar3 | local_win_handle) == 0) {
        &local_win_struct_42.field_0xe0 = 0;
    } else {
        pass1_1008_392e(
            CONCAT22(paVar3, local_win_handle),
            local_win_struct_42.win_handle_0x8,
        );
        local_win_struct_42.field_0xe0 = local_win_handle;
        local_win_struct_42.field_0xe2 = local_DX_85;
    }
    fn_ptr_1 = (in_win_struct + 0x14);
    (**fn_ptr_1)(0x1000, in_win_struct, 0, 0x15a, &ctx.g_alloc_addr_1050_1050);
    local_CS_138 = 0x1000;
    paVar3 = struct_a_00;
    process_struct_1000_179c(0xec, struct_a_00);
    _local_6 = CONCAT22(paVar3, local_win_handle);
    if ((paVar3 | local_win_handle) == 0) {
        &local_win_struct_42.field_0xe4 = 0;
    } else {
        pu_var1 = &local_win_struct_42.field_0xcc;
        unsafe {
            *pu_var1 = *pu_var1 + 1;
        }
        local_CS_138 = 0x1020;
        mci_fn_1020_08b6(_local_6, local_win_struct_42.field_0xcc, in_win_struct);
        local_win_struct_42.field_0xe4 = local_win_handle;
        local_win_struct_42.field_0xe6 = local_DX_175;
    }
    if (local_win_struct_42.field_0xce != 0) {
        ppc_var2 = (local_win_struct_42.field_0xce + 0x10);
        ppc_var2();
    }
    local_win_struct_42.field_0xce = &local_win_struct_42.field_0xe4;
    temp_5fc4833446 = &local_win_struct_42.field_0xe4;
    ppc_var2 = (&local_win_struct_42.field_0xe4 + 0x10);
    ppc_var2();
    temp_5fdbebe016 = local_win_struct_42.field_0xe6;
    local_win_struct_42.field_0xe8 = &local_win_struct_42.field_0xe4;
    ppc_var2 = (local_win_struct_42.field_0xe8 + 8);
    ppc_var2(
        local_CS_138,
        &local_win_struct_42.field_0xe8,
        temp_5fdbebe016,
        temp_5fc4833446,
        1,
    );
    ppc_var2 = (local_win_struct_42.field_0xe8 + 0xc);
    ppc_var2();
    pass1_1008_6978(
        (in_win_struct & 0xffff | local_es_13 << 0x10),
        0,
        local_win_struct_42.field_0xe8,
    );
    return ctx.dx_reg;
}

pub fn call_fill_client_window_1008_1230(param_1: &mut Vec<u8>) {
    fill_client_window_1008_39ac((param_1 + 0xe0));
    return;
}

pub fn update_window_1040_93aa(param_1: u32, param_2: u16, param_3: u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x1e) = param_3;
    (i_var1 + 0x20) = param_2;
    MoveWindow16(
        1,
        (i_var1 + 0x24),
        (i_var1 + 0x22),
        param_2,
        (i_var1 + 0x1e),
        (i_var1 + 0x18),
    );
    UpdateWindow16((i_var1 + 0x18));
    return;
}

pub fn create_win_1040_92dc(param_1: &mut Struct41) {
    let mut window: u16;
    let handle: HANDLE16;
    let local_bx_4: Struct41;
    let handle_00: HANDLE16;
    let lVar1: u32;

    handle_00 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x18 == 0) {
        window = CreateWindow16(
            0,
            ctx.g_h_instance_1050_038c,
            local_bx_4.field_0x1c,
            local_bx_4.field_0x1a,
            0,
            0,
            local_bx_4.field_0x20,
            local_bx_4.field_0x1e,
            0x4000000b,
            s__1050_5e3e,
            s_button_1050_5e3f,
        );
        local_bx_4.field_0x18 = window;
        lVar1 = SetWindowLong16(_g_proc_inst_1050_5e18, (_g_proc_inst_1050_5e18 >> 0x10));
        handle = (lVar1 >> 0x10);
        local_bx_4.field_0x14 = lVar1;
        local_bx_4.field_0x16 = handle;
        SetProp16(handle, s_procHi_1050_5e46, local_bx_4.field_0x18);
        SetProp16(
            local_bx_4.field_0x14,
            s_procLo_1050_5e4d,
            local_bx_4.field_0x18,
        );
        SetProp16(handle_00, s_thisHi_1050_5e54, local_bx_4.field_0x18);
        SetProp16(local_bx_4, s_thisLo_1050_5e5b, local_bx_4.field_0x18);
        if (local_bx_4.field_0x40 != 0) {
            SetProp16(1, s_IsDlg_1050_5e62, local_bx_4.field_0x18);
        }
        ShowWindow16(5, local_bx_4.field_0x18);
    }
    return;
}

pub fn enable_window_1040_9234(param_1: u32, param_2: bool) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x18) != 0) {
        EnableWindow16(param_2, (param_1 + 0x18));
    }
    return;
}

pub fn enable_window_1040_8ea0(param_1: &mut Struct59, param_2: u32, param_3: u32) {
    let enable: bool;
    let h_wnd: HWND16;

    if (param_3._2_2_ == 0xf8) {
        h_wnd = GetDlgItem16(0x17d8, param_1.field_0x6);
        enable = 1;
    } else {
        if (param_3._2_2_ != 0x17d8) {
            win_gui_fn_1040_b54a(param_1, param_2, (param_2 >> 0x10), param_3);
            return;
        }
        SetWindowPos16(6, 0xf6, 0x269, 0, 0, 0, param_1.field_0x6);
        enable = offset;
        GetDlgItem16(0x17d8, param_1.field_0x6);
        h_wnd = 0;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn win_fn_1040_8b3c(param_1: u16, param_2: u32, param_3: u32) {
    if ((param_3._2_2_ != 0x0)
        && (param_3._2_2_ == (&ctx.PTR_LOOP_1050_0000 + 1)
            || param_3._2_2_ == &dos_alloc_addr_1050_0002
            || ((&dos_alloc_addr_1050_0002 + 1) < param_3._2_2_ + -2
                && (param_3._2_2_ + -6 < &dos_alloc_addr_1050_0002))))
    {
        PTR_LOOP_1050_5df4 = 0x0;
        PTR_LOOP_1050_5df8 = param_3._2_2_;
        return;
    }
    post_win_msg_1040_7b3c(param_1, param_2, param_3);
    return;
}

pub fn destroy_window_1040_8b7e(param_1: u32) {
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn create_win_1040_8bea(
    param_1: &mut Struct40,
    param_2: u16,
    menu: u16,
    x_coord: u16,
    y_coord: u16,
    window_name: u32,
) -> HANDLE16 {
    let mut window_1: u16;
    let w_param: HANDLE16;
    let ctx.bx_reg: Struct40;
    let mut u_var1: u16;
    let LVar2: LRESULT;
    let mut local_8: u16;
    let mut style: u32;

    style = 0x50010000;
    if (param_2 != 0) {
        style = 0x50010001;
    }
    u_var1 = (param_1 >> 0x10);
    ctx.bx_reg = param_1;
    if (ctx.bx_reg.field_0x74 != 0) {
        style = style | 0x8000000;
    }
    window_1 = CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        menu,
        ctx.bx_reg.parent,
        0x17,
        0x58,
        y_coord,
        x_coord,
        style,
        window_name,
        s_OPButton_1050_5e00,
    );
    w_param = GetProp16(s_hfont_1050_5e09, ctx.bx_reg.parent);
    if (w_param != 0) {
        LVar2 = SendMessage16(1, w_param, 0x30, window_1);
        w_param = LVar2;
    }
    return w_param;
}

pub fn enable_window_1040_86dc(param_1: u32) {
    let HVar1: HWND16;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    HVar1 = GetDlgItem16(1, (param_1 + 6));
    if (HVar1 != 0) {
        EnableWindow16(1, HVar1);
        HVar1 = GetDlgItem16(2, (param_1 + 6));
        if (HVar1 != 0) {
            EnableWindow16(1, HVar1);
        }
    }
    return;
}

pub fn move_window_1040_826c(param_1: u32, param_2: u16, param_3: u16) {
    let i_var1: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
    if ((param_3 == 0xffff) || (param_2 == 0xffff)) {
        i_var1 = GetSystemMetrics16(0);
        local_4 = (i_var1 - (local_a - local_e)) / 2;
        i_var1 = GetSystemMetrics16(1);
        param_2 = (i_var1 - (local_8 - local_c)) / 2;
    } else {
        local_4 = param_3;
    }
    local_6 = param_2;
    MoveWindow16(
        0,
        local_8 - local_c,
        local_a - local_e,
        param_2,
        local_4,
        (param_1 + 6),
    );
    return;
}

pub fn destroy_win_1040_bf92(param_1: &mut Struct339) {
    let local_bx_4: Struct339;
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0xc53e;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1010_1ea6(local_bx_4.field_0x6, (param_1 & 0xffff | u_var1 << 0x10));
    destroy_win_1010_2fa0(local_bx_4.field_0x6);
    param_1 = ctx.s_0_020_1050_3ab0;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn enum_child_windows_1010_01be() {
    let unaff_cs: HANDLE16;
    let pvVar1: &mut Vec<u8>;
    let mut local_6: u16;
    let mut local_4: u16;

    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
        pvVar1 = MakeProcInstance16(unaff_cs, CONCAT22(0x240, ctx.g_h_instance_1050_038c));
        EnumChildWindows16(0, pvVar1, (pvVar1 >> 0x10));
        FreeProcInstance16(CONCAT22(pvVar1, 0x1538));
    }
    return;
}

pub fn pass1_1020_02ae(in_struct_1: &mut Struct594) {
    let local_struct_1: Struct594;
    let mut unaff_si: u16;
    let local_struct_1_hi: Struct594;
    let temp_5f77ded944: Vec<u8>;
    let temp_5f51233cf1: Vec<u8>;
    // fn_ptr_1: *mut Vec<u8>;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pass1_1010_3cd0(&local_struct_1.u32_xE2);
    destroy_win_1008_628e(in_struct_1, unaff_si);
    // WARNING: Load size is inaccurate
    temp_5f77ded944 = local_struct_1.u8_ptr_32_xE6;
    temp_5f51233cf1 = *(&local_struct_1.u8_ptr_32_xE6 + 2);
    if ((temp_5f51233cf1 | temp_5f77ded944) != 0) {
        fn_ptr_1 = temp_5f77ded944;
        (**fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, temp_5f77ded944, temp_5f51233cf1, 1);
    }
    local_struct_1.u8_ptr_32_xE6 = 0x0;
    // WARNING: Load size is inaccurate
    pass1_1010_1dda(local_struct_1.u32_xE2);
    &local_struct_1.u32_xE2 = 0;
    return;
}

pub fn create_win_fn_1020_0316(in_struct_1: &mut win_struct_42) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let ppVar1: *mut pass1_struct_1;
    let in_struct_1_00: Struct629;
    let mut u_var2: u32;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;

    create_win_1008_9760(in_struct_1);
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff2, 1));
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.u16_xe2 = ppVar1;
    local_struct_1.u16_xe4 = (ppVar1 >> 0x10);
    u_var2 = &local_struct_1.u16_xe2;
    (u_var2 + 0x16) = &local_struct_1.field_0xea;
    u_var2 = &local_struct_1.u16_xe2;
    *(u_var2 + 0x12) = local_struct_1.char_ptr_16_0xee;
    in_struct_1_00 = pass1_1010_3c52(&local_struct_1.u16_xe2, &local_struct_1.field_0xec);
    process_struct_1000_179c(0x12, (in_struct_1_00 >> 0x10));
    if (in_struct_1_00 != 0x0) {
        u_var2 = process_struct_1020_04f6(in_struct_1_00, local_struct_1.win_handle_0x8);
        &local_struct_1.field_0xe6 = u_var2;
        &local_struct_1.field_0xe8 = (u_var2 >> 0x10);
        return;
    }
    &local_struct_1.field_0xe6 = 0;
    return;
}

pub fn win_cleanup_1040_d1bc(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.ptr_a_lo = 0xd8c4;
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(i_var4 + 6));
    pu_var1 = (i_var4 + 0x9c);
    u_var2 = (i_var4 + 0x9e);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(&PTR_LOOP_1050_1038, pu_var1, u_var2, 1);
    }
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn destroy_win_1040_caa6(param_1: u16, param_2: u32) {
    let paVar1: Struct318;
    let BVar2: bool;

    BVar2 = 0;
    paVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x2b);
    pass1_1010_038e(paVar1, BVar2);
    destroy_window_1040_b726(param_1, param_2);
    return;
}

pub fn win_cleanup_1008_0618(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let u_var4: u8;
    let extraout_var: u32;
    let mut u_var5: i32;

    let local_bx_5: Struct47;
    let mut u_var6: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1 = s_0_1050_389e;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    color::sys_color_func_1008_357e(param_1, 0);
    error_check_1000_17ce(local_bx_5.field_0xf8);
    if (local_bx_5.field_0xec != 0) {
        DestroyMenu16(0x1000);
    }
    DestroyIcon16(local_bx_5.field_0xc2);
    local_bx_5.field_0xc2 = 0;
    pu_var1 = local_bx_5.field_0xe0;
    u_var2 = local_bx_5.field_0xe2;
    u_var5 = u_var2 | pu_var1;
    if (u_var5 != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(offset, pu_var1, u_var2, 1);
        u_var5 = ctx.dx_reg;
    }
    u_var4 = pass1_1008_57c4((param_1 & 0xffff0000 | &local_bx_5.field_0xd2));
    param_1 = 0x380a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return CONCAT31(extraout_var, u_var4) & 0xffff | u_var5 << 0x10;
}

pub fn win_cleanup_fn_1040_a294(param_1: &mut Struct44) {
    let mut local_CS__1: u16;

    param_1.ptr_a_lo = 0xa4e8;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + -1;
    if (PTR_LOOP_1050_5eda == 0x0) {
        FreeProcInstance16(CONCAT22(_PTR_LOOP_1050_5edc, local_CS__1));
        _PTR_LOOP_1050_5edc = 0;
    }
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn destroy_win_1040_7b98(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x74) == 0) {
        DestroyWindow16((param_1 + 6));
    }
    return;
}

pub fn create_win_1040_7620(
    param_1: u32,
    param_2: i32,
    param_3: &mut u16,
    param_4: u16,
    HMENmenu: u16,
) {
    let mut window_name: string;
    let local_bx_49: &mut Struct39;
    let mut u_var1: u16;
    let mut local_a: u32;

    window_name = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        param_4,
    );
    local_a = 0x50000009;
    if (param_2 != 0) {
        local_a = 0x50020009;
    }
    u_var1 = (param_3 >> 0x10);
    local_bx_49 = param_3;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        menu,
        (param_1 + 6),
        local_bx_49.field_0x6,
        local_bx_49.field_0x4,
        local_bx_49.field_0x2,
        unsafe { *param_3 },
        local_a,
        window_name,
        s_button_1050_5da8,
    );
    return;
}

pub fn create_win_1040_70b4(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: i32;
    let h_wnd: &mut u32;
    let mut i_var5: i32;
    let struct_a: &mut Struct199;

    let struct_a_00: &mut Struct199;
    let struct_a_01: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;
    let paVar6: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;

    let struct_a_02: &mut Struct199;

    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let ppVar10: &mut pass1_struct_1;
    let dVar11: u32;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let u_var17: u8;
    let mut hdc: u16;
    let mut in_stack_0000ff8a: u16;
    let mut local_6e: u16;
    let mut local_6c: u16;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 80];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    ppVar10 = struct_ops::process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ff8a, 0x34),
    );
    u_var4 = ppVar10;
    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    (i_var7 + 0x94) = u_var4;
    (i_var7 + 0x96) = (ppVar10 >> 0x10);
    u_var9 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 4);
    (**ppc_var3)(0x10, u_var9, (ppVar10 >> 0x10), 0, i_var7, u_var8);
    paVar6 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar6 | u_var4) == 0) {
        (i_var7 + 0x98) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((paVar6 >> 8), CONCAT12(paVar6, u_var4)),
            (i_var7 + 6),
        );
        (i_var7 + 0x98) = u_var4;
        (i_var7 + 0x9a) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var7 + 0x98), (i_var7 + 0x94));
    paVar6 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    struct_a_01 = (paVar6 | u_var4);
    if (struct_a_01 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        struct_a_01 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, struct_a_01);
    paVar6 = (struct_a_01 | u_var4);
    if (paVar6 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            struct_a_01,
            1,
            0xa0028,
            0x830000,
            0x10c0082,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        paVar6 = ctx.dx_reg;
    }
    u_var12 = 0;
    u_var16 = 0;
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        misc::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00aa,
            0x1000101,
            0x10700ff,
            CONCAT13(u_var16, CONCAT12(u_var12, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    enable_window_1040_9234(u_var4, local_4, u_var12);
    u_var12 = 0;
    u_var16 = 0;
    paVar6 = struct_a_02;
    process_struct_1000_179c(0x42, struct_a_02);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        misc::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00c2,
            0x1000101,
            0x10800ff,
            CONCAT13(u_var16, CONCAT12(u_var12, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    enable_window_1040_9234(u_var4, local_4, u_var12);
    local_8 = GetDC16((i_var7 + 6));
    u_var15 = unaff_ss;
    u_var17 = (unaff_ss >> 8);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        0x7cd,
    );
    u_var13 = SUB21(local_58, 0);
    u_var14 = (local_58 >> 8);
    u_var12 = u_var15;
    u_var16 = u_var17;
    hdc = local_8;
    u_var4 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_58));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT13(u_var16, CONCAT12(u_var12, CONCAT11(u_var14, u_var13))),
        hdc,
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7cd,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xad,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5d9a,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT22(unaff_ss, local_58),
        0x7ce,
    );
    u_var13 = local_8;
    u_var14 = (local_8 >> 8);
    u_var12 = SUB21(local_58, 0);
    u_var16 = (local_58 >> 8);
    u_var4 = get_string_index_1000_3da4(CONCAT13(u_var17, CONCAT12(u_var15, local_58)));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT22(unaff_ss, CONCAT11(u_var16, u_var12)),
        CONCAT11(u_var14, u_var13),
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    ReleaseDC16(local_8, (i_var7 + 6));
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7ce,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xc5,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5da1,
    );
    local_64 = 0x5a000a;
    local_60 = 0x140050;
    h_wnd = &local_64;
    u_var12 = SUB41(param_1, 0);
    create_win_1040_7620(
        u_var12,
        u_var8,
        1,
        h_wnd,
        u_var15,
        0x5eb,
        0xfd,
        (i_var7 + 6),
    );
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_7620(u_var12, u_var8, 0, 0x9c, u_var15, 0x5ed, 0xfe, (i_var7 + 6));
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_7620(u_var12, u_var8, 0, 0x9c, u_var15, 0x5ef, 0xff, (i_var7 + 6));
    SendMessage16(0, 1, 0x401, h_wnd);
    u_var1 = (i_var7 + 0x94);
    i_var5 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var9 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i_var5 + 0x10),
        (i_var5 + 0xe),
        (i_var5 + 0xc),
        (u_var1 | i_var5 + 10),
        0,
        (i_var7 + 6),
    );
    u16_1050_0ecc = 0;
    u_var2 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 0x10);
    (**ppc_var3)(offset, u_var2, (u_var2 >> 0x10));
    u_var2 = (i_var7 + 0x94);
    pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
    PostMessage16(0, 0x10a, 0x111, (i_var7 + 6));
    return;
}

pub fn enable_window_1040_6ff2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    if (param_2 == 8) {
        u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        HVar2 = GetDlgItem16(0x107, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x26), HVar2);
    }
    return;
}

pub fn create_win_1040_6eae(
    param_1: u32,
    param_2: u16,
    param_3: &mut Struct38,
    param_4: u16,
    menu: u16,
) {
    let mut window_name: string;
    let local_bx_49: &mut Struct38;
    let mut in_stack_0000000c: u16;
    let mut local_a: u32;

    window_name = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        param_4,
    );
    local_a = 0x50000009;
    if (param_2 != 0) {
        local_a = 0x50020009;
    }
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        menu,
        (param_1 + 6),
        param_3.field_0x6,
        param_3.field_0x4,
        param_3.field_0x2,
        *_param_3,
        local_a,
        window_name,
        s_button_1050_5d92,
    );
    return;
}

pub fn destroy_win_1040_6d1a(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let unaff_ss: HWND16;
    let mut u_var3: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2._2_2_) {
        0xfa => {
            pp_var1 = ((param_1 + 1) + 0x18);
            (**pp_var1)()
        }
        _ => {
            win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        0xfd => {
            if (u16_1050_0ecc == 0) {
                return;
            }
            u16_1050_0ecc = 0;
            // goto LAB_1040_6deb;
        }
        0xfe => {
            if (u16_1050_0ecc == 1) {
                return;
            }
            u16_1050_0ecc = 1;
            // goto LAB_1040_6deb;
        }
        0xff => {
            if (u16_1050_0ecc == 2) {
                return;
            }
            u16_1050_0ecc = 2;
            // LAB_1040_6deb:
            u_var2 = (param_1 + 1);
            pp_var1 = ((param_1 + 1) + 0x10);
            (**pp_var1)(0x40, u_var2, (u_var2 >> 0x10));
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
            PostMessage16(0, 0x10a, 0x111, &param_1.field_0x6)
        }
        0x107 => {
            u_var3 = 0;
            // goto LAB_1040_6e48;
        }
        0x108 => {
            u_var3 = 1;
            // LAB_1040_6e48:
            u_var2 = (param_1 + 1);
            destroy_win_1010_3202(u_var2, (u_var2 >> 0x10), u_var3)
        }
        0x10a => {
            GetClientRect16(CONCAT22(unaff_ss, &local_a), &param_1.field_0x6);
            u_var2 = (param_1 + 1);
            local_8 = local_8 + 3;
            local_a = (u_var2 + 0x1a) - 9;
            local_6 = local_6 - 3;
            local_4 = local_4 - 3;
            InvalidateRect16(1, &local_a, unaff_ss);
            destroy_win_1010_2fa0((param_1 + 1));
            pass1_1010_32c0((param_1 + 1), 0);
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10))
        }
        0x10c => {
            DestroyWindow16(&param_1.field_0x6);
        }
    }
    return;
}

pub fn create_win_1040_6942(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: i32;
    let h_wnd: &mut Struct38;
    let mut i_var5: i32;
    let struct_a: &mut Struct199;

    let struct_a_00: &mut Struct199;
    let struct_a_01: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;
    let paVar6: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;

    let struct_a_02: &mut Struct199;

    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let ppVar10: &mut pass1_struct_1;
    let dVar11: u32;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let u_var17: u8;
    let mut hdc: u16;
    let mut in_stack_0000ff8a: u16;
    let mut local_6e: u16;
    let mut local_6c: u16;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 80];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    ppVar10 = struct_ops::process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ff8a, 0x33),
    );
    u_var4 = ppVar10;
    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    (i_var7 + 0x94) = u_var4;
    (i_var7 + 0x96) = (ppVar10 >> 0x10);
    u_var9 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 4);
    (**ppc_var3)(0x10, u_var9, (ppVar10 >> 0x10), 0, i_var7, u_var8);
    paVar6 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar6 | u_var4) == 0) {
        (i_var7 + 0x98) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((paVar6 >> 8), CONCAT12(paVar6, u_var4)),
            (i_var7 + 6),
        );
        (i_var7 + 0x98) = u_var4;
        (i_var7 + 0x9a) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var7 + 0x98), (i_var7 + 0x94));
    paVar6 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    struct_a_01 = (paVar6 | u_var4);
    if (struct_a_01 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        struct_a_01 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, struct_a_01);
    paVar6 = (struct_a_01 | u_var4);
    if (paVar6 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            struct_a_01,
            1,
            0xa0028,
            0x830000,
            0x10c0082,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        paVar6 = ctx.dx_reg;
    }
    u_var14 = 0;
    u_var16 = 0;
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        misc::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00aa,
            0x1000101,
            0x10700ff,
            CONCAT13(u_var16, CONCAT12(u_var14, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    enable_window_1040_9234(u_var4, local_4, u_var14);
    u_var14 = 0;
    u_var16 = 0;
    paVar6 = struct_a_02;
    process_struct_1000_179c(0x42, struct_a_02);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        misc::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00c2,
            0x1000101,
            0x10800ff,
            CONCAT13(u_var16, CONCAT12(u_var14, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    enable_window_1040_9234(u_var4, local_4, u_var14);
    local_8 = GetDC16((i_var7 + 6));
    u_var15 = unaff_ss;
    u_var17 = (unaff_ss >> 8);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        0x7cd,
    );
    u_var12 = SUB21(local_58, 0);
    u_var13 = (local_58 >> 8);
    u_var14 = u_var15;
    u_var16 = u_var17;
    hdc = local_8;
    u_var4 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_58));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT13(u_var16, CONCAT12(u_var14, CONCAT11(u_var13, u_var12))),
        hdc,
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7cd,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xad,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5d84,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT22(unaff_ss, local_58),
        0x7ce,
    );
    u_var12 = local_8;
    u_var13 = (local_8 >> 8);
    u_var14 = SUB21(local_58, 0);
    u_var16 = (local_58 >> 8);
    u_var4 = get_string_index_1000_3da4(CONCAT13(u_var17, CONCAT12(u_var15, local_58)));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT22(unaff_ss, CONCAT11(u_var16, u_var14)),
        CONCAT11(u_var13, u_var12),
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    ReleaseDC16(local_8, (i_var7 + 6));
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7ce,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xc5,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5d8b,
    );
    local_64 = 0x5a000a;
    local_60 = 0x140050;
    h_wnd = &local_64;
    create_win_1040_6eae(param_1, 1, h_wnd, 0x5eb, 0xfd);
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_6eae(param_1, 0, &local_64, 0x5ec, 0xfe);
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_6eae(param_1, 0, &local_64, 0x5ee, 0xff);
    SendMessage16(0, 1, 0x401, h_wnd);
    u_var1 = (i_var7 + 0x94);
    i_var5 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var9 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i_var5 + 0x10),
        (i_var5 + 0xe),
        (i_var5 + 0xc),
        (u_var1 | i_var5 + 10),
        0,
        (i_var7 + 6),
    );
    u16_1050_0ecc = 0;
    u_var2 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 0x10);
    (**ppc_var3)(offset, u_var2, (u_var2 >> 0x10));
    u_var2 = (i_var7 + 0x94);
    pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
    PostMessage16(0, 0x10a, 0x111, (i_var7 + 6));
    return;
}

pub fn enable_window_1040_6880(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    if (param_2 == 8) {
        u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        HVar2 = GetDlgItem16(0x107, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x26), HVar2);
    }
    return;
}

pub fn enable_window_1040_5780(param_1: &mut u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u16;
    let h_wnd: HWND16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let ppVar6: &mut pass1_struct_1;
    let mut i_var7: i32;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    unsafe {
        pp_var1 = (*param_1 + 0x74);
    }
    i_var7 = i_var4;
    (**pp_var1)();
    ppVar6 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(i_var7, 3));
    u_var2 = (i_var4 + 0x90);
    u_var3 = pass1_1010_acc0(ppVar6, (ppVar6 >> 0x10), (u_var2 + 6));
    if (u_var3 != 0) {
        h_wnd = GetDlgItem16(0x1790, (i_var4 + 6));
        EnableWindow16(1, h_wnd);
    }
    return;
}

pub fn set_window_pos_1040_4f96(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut i_var7: i32;
    let extraout_DL: u8;
    let u_var8: u8;
    let mut u_var9: u16;
    let struct_a: &mut Struct199;

    let struct_a_00: &mut Struct199;
    let pa_var10: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;
    let pa_var11: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;
    let ctx.dx_reg: &mut Struct199;
    let extraout_dx_04: &mut Struct199;
    let extraout_dx_05: &mut Struct199;
    let mut u_var12: u16;
    let mut i_var13: i32;
    let mut u_var14: u16;
    let mut u_var15: u16;
    let ppVar16: &mut pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    ppVar16 = struct_ops::process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x41),
    );
    u_var9 = (ppVar16 >> 0x10);
    u_var4 = ppVar16;
    u_var14 = (param_1 >> 0x10);
    i_var13 = param_1;
    (i_var13 + 0x98) = u_var4;
    (i_var13 + 0x9a) = u_var9;
    u_var15 = (i_var13 + 0x98);
    ppc_var2 = ((i_var13 + 0x98) + 0x10);
    ppc_var2(0x1010, u_var15, u_var9);
    pa_var11 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((pa_var11 | u_var4) == 0) {
        (i_var13 + 0x94) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((pa_var11 >> 8), CONCAT12(pa_var11, u_var4)),
            (i_var13 + 6),
        );
        (i_var13 + 0x94) = u_var4;
        (i_var13 + 0x96) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var13 + 0x94), (i_var13 + 0x98));
    pa_var11 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    pa_var10 = (pa_var11 | u_var4);
    if (pa_var10 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var10 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var10);
    pa_var11 = (pa_var10 | u_var4);
    if (pa_var11 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            pa_var10,
            1,
            0xa0028,
            0x850000,
            0x10b0084,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var11 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var11);
    pa_var10 = (pa_var11 | u_var4);
    if (pa_var10 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa0046,
            0x870000,
            0x10d0086,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var10 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var10);
    pa_var11 = (pa_var10 | u_var4);
    if (pa_var11 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            pa_var10,
            1,
            0xa0064,
            0x890000,
            0x10e0088,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var11 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var11);
    pa_var10 = (pa_var11 | u_var4);
    if (pa_var10 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa0082,
            0x830000,
            0x10c0082,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var10 = extraout_dx_04;
    }
    process_struct_1000_179c(0x42, pa_var10);
    pa_var11 = (pa_var10 | u_var4);
    if (pa_var11 != 0x0) {
        misc::win_fn_1008_3bd6(
            u_var4,
            pa_var10,
            1,
            0xa00d2,
            0x8b0000,
            0xbbb008a,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var11 = extraout_dx_05;
    }
    u_var9 = 0;
    process_struct_1000_179c(0x42, pa_var11);
    if ((pa_var11 | u_var4) == 0) {
        u_var4 = 0;
        u_var8 = 0;
    } else {
        misc::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa00a0,
            0x8d008e,
            0xbbc008c,
            CONCAT22(u_var9, (i_var13 + 6)),
        );
        u_var8 = extraout_DL;
    }
    enable_window_1040_9234(u_var4, u_var8, u_var9);
    ppVar16 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var15, 3));
    u_var12 = (ppVar16 >> 0x10);
    u_var3 = ppVar16;
    u_var5 = pass1_1010_a5ac(u_var3, u_var12, (i_var13 + 0xb0));
    u_var6 = pass1_1010_ac62(u_var3, u_var12, 0x1e);
    if (u_var6 != 0) {
        pass1_1010_a5ca(u_var3, u_var12, u_var5);
        if (0 < u_var6) {
            pass1_1010_a58a(u_var3, u_var12, u_var5);
            if (u_var6 == 0) {
                enable_window_1040_9234(u_var4, u_var8, 1);
            }
        }
    }
    u_var1 = (i_var13 + 0x98);
    i_var7 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var15 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i_var7 + 0x10),
        (i_var7 + 0xe),
        (i_var7 + 0xc),
        (u_var1 | i_var7 + 10),
        0,
        (i_var13 + 6),
    );
    return;
}

pub fn set_win_pos_1040_4ae4(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: u32;

    let pa_var5: &mut Struct199;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0xeb) {
        _local_6 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
        pa_var5 = (_local_6 >> 0x10);
        u_var4 = &param_1.field_0x90;
        if (u_var4 != 0) {
            local_a = u_var4;
            process_struct_1000_179c(0x18, pa_var5);
            u_var3 = u_var4;
            _local_10 = (u_var4 & 0xffff | ZEXT24(pa_var5) << 0x10);
            if ((pa_var5 | u_var3) == 0) {
                u_var3 = 0;
                pa_var5 = 0x0;
            } else {
                process_struct_1040_a598((u_var4 & 0xffff | ZEXT24(pa_var5) << 0x10));
                pa_var5 = ctx.dx_reg;
            }
            param_1.field_0x90 = u_var3;
            &param_1.field_0x92 = pa_var5;
            *&param_1.field_0x90 = 7;
            local_c = *&param_1.field_0x90;
            u_var3 = local_c * 10 + 2;
            process_struct_1000_179c(u_var3, pa_var5);
            _local_10 = CONCAT22(pa_var5, u_var3);
            if ((pa_var5 | u_var3) == 0) {
                u_var2 = &param_1.field_0x90;
                (u_var2 + 2) = 0;
            } else {
                *_local_10 = local_c;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &ctx.PTR_LOOP_1050_1040,
                    local_c,
                    10,
                    u_var3 + 2,
                    pa_var5,
                );
                u_var2 = &param_1.field_0x90;
                u_var7 = (u_var2 >> 0x10);
                i32_var6 = u_var2;
                (i32_var6 + 2) = u_var3 + 2;
                (i32_var6 + 4) = pa_var5;
            }
            u_var7 = (local_a >> 0x10);
            i32_var6 = local_a;
            u_var2 = &param_1.field_0x90;
            (u_var2 + 6) = (i32_var6 + 6);
            u_var2 = &param_1.field_0x90;
            (u_var2 + 10) = (i32_var6 + 10);
            u_var2 = &param_1.field_0x90;
            (u_var2 + 0x12) = (i32_var6 + 0x12);
            u_var7 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505d6a, &param_1.field_0x90);
            local_14 = local_a;
            _local_10 = local_a;
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                u_var7 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            pp_var1 = (CONCAT22(param_2, param_1) + 0x70);
            (**pp_var1)(u_var7, param_1, param_2);
        }
    } else {
        if (param_3._2_2_ != 6000) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        if (param_3 == 7) {
            GetWindowRect16(CONCAT22(&local_24, unaff_cs), unaff_ss);
            local_20 = local_20 - local_24;
            SetWindowPos16(2, 0x50, local_20, 0, 0, 0, param_3_00);
        }
    }
    return;
}

pub fn enable_window_1040_32a8(param_1: &mut Vec<u8>) {
    let lp_string: SEGPTR;
    let b_var1: bool;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    _local_c = param_1 & 0xffff0000 | (param_1 + 0x19a);
    lp_string = pass1_1018_3a5c(
        (param_1 + 0x96),
        param_1 & 0xffff0000 | (param_1 + 0x9a),
        param_1 & 0xffff0000 | (param_1 + 0x19a),
    );
    SetWindowText16(lp_string, (param_1 + 0x90));
    b_var1 = pass1_1018_39d8(
        (param_1 + 0x96),
        param_1 & 0xffff0000 | (param_1 + 0x9a),
        _local_c,
    );
    EnableWindow16(b_var1 & 1, (param_1 + 0x8e));
    return;
}

pub fn set_window_pos_1040_331a(param_1: &mut Vec<u8>, param_2: u16, param_3: u16) -> u16 {
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_3;
    local_6 = param_2;
    if (param_3 == 1) {
        enable_window_1040_32a8(param_1);
    } else {
        if (param_3 != 7) {
            return 0;
        }
        GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
        local_a = local_a - local_e;
        SetWindowPos16(2, 0x50, local_a, 0, 0, 0, local_6);
    }
    return 1;
}

pub fn enable_window_1040_2a64(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: i32;
    let struct_a: &mut Struct199;
    let mut i_var4: i32;
    let local_bx_215: &mut Struct58;
    let mut u_var5: u16;
    let mut h_wnd: u16;
    let local_28: &mut Struct57;
    let mut uStack38: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_28 = param_1;
    uStack38 = (param_1 >> 0x10);
    set_window_pos_1040_b230(local_28, uStack38);
    local_4 = 5;
    u_var1 = local_28.field_0x90;
    _local_c = pass1_1030_73a8((u_var1 + 6));
    struct_a = (_local_c >> 0x10);
    u_var5 = SUB42(&PTR_LOOP_1050_1028, 0);
    PTR_LOOP_1050_5d04 = pass1_1028_4a9a(_local_c, 0);
    local_e = 0;
    while (local_e < local_4) {
        if (local_e != 0) {
            (&PTR_LOOP_1050_5d04 + local_e * 0xc) = 0;
        }
        local_bx_215 = (local_e * 0xc);
        local_16 = (local_bx_215 + 0x5cfc);
        local_14 = (local_bx_215 + 0x5cfe);
        u_var3 = 1;
        local_12 = 1;
        local_10 = 1;
        u_var2 = local_28.field_0x6;
        MapDialogRect16(CONCAT22(&local_16, u_var5), h_wnd);
        u_var5 = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | u_var3) == 0) {
            _local_8 = 0;
        } else {
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            _local_8 = misc::win_fn_1008_3bd6(
                u_var3,
                struct_a,
                1,
                CONCAT22(local_16, local_14),
                0x1000101,
                CONCAT22((local_bx_215 + 0x5d00), 0xff),
                CONCAT22(u_var2, local_28.field_0x6),
            );
        }
        struct_a = (_local_8 >> 0x10);
        if (PTR_LOOP_1050_5d04 == 0x0) {
            if ((local_e != 0) && (_local_8 != 0)) {
                u_var5 = SUB42(offset, 0);
                EnableWindow16(0, (_local_8 + 0x18));
            }
        } else {
            i_var4 = local_e * 0xc;
            u_var5 = SUB42(&PTR_LOOP_1050_1028, 0);
            u_var2 = pass1_1028_4a9a(_local_c, (i_var4 + 0x5d02));
            if (u_var2 != 0) {
                (&PTR_LOOP_1050_5d04 + i_var4) = 1;
                u_var5 = SUB42(offset, 0);
                SetDlgItemText16(
                    local_28.field_0x98,
                    (&PTR_s_post_1050_015d_1050_5d06 + i_var4),
                    local_28.field_0x6,
                );
            }
        }
        local_e = local_e + 1;
    }
    return;
}

pub fn enable_window_1040_2bb2(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: i32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let lp_string: SEGPTR;
    let id: Vec<u8>;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0x158) {
        PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04 == 0x0);
        if (PTR_LOOP_1050_5d04 == 0x0) {
            local_8 = 1;
            while (local_8 < 5) {
                i_var3 = local_8 * 0xc;
                HVar2 = GetDlgItem16((i_var3 + 0x5d00), &param_1.field_0x6);
                EnableWindow16(0, HVar2);
                (&PTR_LOOP_1050_5d04 + i_var3) = 0;
                SetDlgItemText16(
                    (param_1 + 1),
                    (&PTR_s_post_1050_015d_1050_5d06 + i_var3),
                    &param_1.field_0x6,
                );
                local_8 = local_8 + 1;
            }
            HVar2 = &param_1.field_0x6;
            lp_string = (param_1 + 1);
            id = PTR_s_post_1050_015d_1050_5d06;
            // goto LAB_1040_2ccc;
        }
        local_8 = 1;
        while (local_8 < 5) {
            i_var3 = local_8 * 0xc;
            HVar2 = GetDlgItem16((i_var3 + 0x5d00), &param_1.field_0x6);
            EnableWindow16(1, HVar2);
            (&PTR_LOOP_1050_5d04 + i_var3) = 0;
            SetDlgItemText16(
                (param_1 + 1),
                (&PTR_s_post_1050_015d_1050_5d06 + i_var3),
                &param_1.field_0x6,
            );
            local_8 = local_8 + 1;
        }
        HVar2 = &param_1.field_0x6;
        id = PTR_s_post_1050_015d_1050_5d06;
    } else {
        if (param_3._2_2_ == 0x159) {
            local_4 = 1;
        } else {
            if (param_3._2_2_ == 0x15a) {
                local_4 = 2;
            } else {
                if (param_3._2_2_ == 0x15b) {
                    local_4 = 3;
                } else {
                    if (param_3._2_2_ != 0x15c) {
                        win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
                        return;
                    }
                    local_4 = 4;
                }
            }
        }
        if (local_4 == 0) {
            return;
        }
        i_var3 = local_4 * 0xc;
        u_var1 = ((&PTR_LOOP_1050_5d04 + i_var3) == 0);
        (&PTR_LOOP_1050_5d04 + i_var3) = u_var1;
        if (u_var1 == 0) {
            HVar2 = &param_1.field_0x6;
            lp_string = (param_1 + 1);
            id = (&PTR_s_post_1050_015d_1050_5d06 + i_var3);
            // goto LAB_1040_2ccc;
        }
        HVar2 = &param_1.field_0x6;
        id = (&PTR_s_post_1050_015d_1050_5d06 + local_4 * 0xc);
    }
    lp_string = &param_1[1].field_0x4;
    // LAB_1040_2ccc:
    SetDlgItemText16(lp_string, id, HVar2);
    return;
}

pub fn enable_window_1040_2490(param_1: &mut Struct20) {
    let pp_var1: fn();
    let h_wnd: HWND16;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let pi_var5: &mut i32;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    h_wnd = GetDlgItem16(0xfb1, (i_var3 + 6));
    EnableWindow16(0, h_wnd);
    pp_var1 = ((i_var3 + 0x8e) + 0x10);
    pi_var5 = (**pp_var1)(offset, (i_var3 + 0x8e));
    u_var2 = (pi_var5 >> 0x10);
    move_window_1040_826c(
        i_var3,
        u_var4,
        (pi_var5 + 2) + -2,
        (pi_var5 + 4) + unsafe { *pi_var5 } + 3,
    );
    ShowWindow16(5, (i_var3 + 6));
    pass1_1018_1c9a(*(i_var3 + 0x8e), 0x1a0);
    return;
}

pub fn create_win_1040_20d4(param_1: &mut Struct20) {
    let mut cx: i32;
    let pu_var1: Vec<u8>;

    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let ppVar4: &mut pass1_struct_1;
    let mut in_stack_0000ffca: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: [u8; 4];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    ppVar4 = struct_ops::process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffca, 0x48),
    );
    local_c = (ppVar4 >> 0x10);
    local_e = ppVar4;
    local_8 = (local_e + 10);
    local_a = (local_e + 0xc);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    _local_12 = process_struct_1008_4772((i_var2 + 0x8e));
    cx = (_local_12 + 4);
    local_4 = (local_8 - cx) / 2;
    local_6 = 5;
    SetWindowPos16(6, 0x1d6, cx, 5, local_4, 0, (i_var2 + 6));
    pu_var1 = local_1e;
    GetClientRect16(CONCAT22(unaff_ss, pu_var1), (i_var2 + 6));
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x592,
    );
    local_16 = 0x50010001;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        1,
        (i_var2 + 6),
        0x19,
        0x58,
        local_18 - 0x28,
        (local_1a - 0x58) / 2,
        0x50010001,
        CONCAT22(ctx.dx_reg, pu_var1),
        s_OPButton_1050_5ce4,
    );
    SetWindowPos16(
        0x45,
        local_a - 10,
        (_local_12 + 4),
        5,
        local_4,
        0,
        (i_var2 + 6),
    );
    return;
}

pub fn show_window_1040_1d50(param_1: &mut Struct20) {
    misc::win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn set_win_pos_1040_162a(param_1: u16, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut u_var2: u32;
    let mut local_a: u16;
    let mut local_6: u16;

    if ((param_3._2_2_ != (s_vrpal_bmp_1050_183a + 5))
        && (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 4)))
    {
        u_var2 = post_win_msg_1040_7b3c(param_1, param_2, param_3);
        return u_var2;
    }
    if (param_3 == 7) {
        GetWindowRect16(CONCAT22(&local_a, unaff_cs), unaff_ss);
        local_6 = local_6 - local_a;
        SetWindowPos16(2, 0x50, local_6, 0, 0, 0, param_2._2_2_);
    } else {
        if ((param_3 != 9) && (param_3 != 10)) {
            u_var1 = 0;
            // goto LAB_1040_164d;
        }
    }
    u_var1 = 1;
    // LAB_1040_164d:
    return u_var1;
}

pub fn set_win_pos_1040_0f0c(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut u_var6: u32;
    let id: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_2e: [u8; 2];
    let mut local_2c: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x98) == 0) {
        GetWindowRect16(
            CONCAT13((&local_26 >> 8), CONCAT12(&local_26, unaff_cs)),
            unaff_ss,
        );
        GetDlgItem16(0x1830, (i_var4 + 6));
        GetWindowRect16(CONCAT22(local_2e, 0x1538), unaff_ss);
        local_22 = local_22 - local_26;
        local_20 = (local_2c - local_24) - 2;
        SetWindowPos16(6, local_20, local_22, 0, 0, 0, (i_var4 + 6));
        CheckDlgButton16(1, 0x1c1, (i_var4 + 6));
        u_var1 = (i_var4 + 0x8e);
        (u_var1 + 10) = 2;
        HVar2 = GetDlgItem16(0x1830, (i_var4 + 6));
        EnableWindow16(0, HVar2);
    } else {
        u_var1 = (i_var4 + 0x92);
        u_var6 = pass1_1030_73a8((u_var1 + 6));
        if ((u_var6 + 0x20) == 2) {
            HVar2 = (i_var4 + 6);
            id = 0x1c1;
        } else {
            HVar2 = (i_var4 + 6);
            id = 0x1c2;
        }
        CheckDlgButton16(1, id, HVar2);
    }
    GetCursor16(offset, &local_6);
    u_var3 = (i_var4 + 6);
    GetWindowRect16(CONCAT22(&local_e, 0x1538), unaff_ss);
    local_14 = local_a - local_e;
    local_10 = -(local_14 / 2 - local_6);
    local_16 = local_8 - local_c;
    local_12 = -(local_16 / 2 - local_4);
    _local_1e = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var3, 0x48));
    u_var3 = (_local_1e >> 0x10);
    local_18 = (_local_1e + 10);
    local_1a = (_local_1e + 0xc);
    if (local_18 < (local_14 + local_10)) {
        local_10 = local_18 - local_14;
    }
    if (local_1a < (local_16 + local_12)) {
        local_12 = local_1a - local_16;
    }
    SetWindowPos16(0x45, 0, 0, local_12, local_10, 0, (i_var4 + 6));
    return;
}

pub fn show_window_1040_0c7c(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_6: u32;

    misc::win_gui_func_1040_78e2(param_1);
    u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x8e);
    pass1_1010_4f30(
        u_var1,
        (u_var1 >> 0x10),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_6 + 2),
    );
    move_window_1040_826c(param_1, local_6);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn enable_win_1040_0acc(param_1: u32, param_2: bool) {
    let b_var1: bool;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    b_var1 = IsWindow16((i_var3 + 6));
    if (b_var1 != 0) {
        HVar2 = GetDlgItem16(100, (i_var3 + 6));
        b_var1 = IsWindow16(HVar2);
        if (b_var1 != 0) {
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x74, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x73, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x6e, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0xee, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
        }
    }
    return;
}

pub fn show_win_1040_0766(param_1: &mut Struct20) {
    let mut unaff_ss: u16;
    let ppVar1: &mut pass1_struct_1;
    let p_uvar2: &mut u16;
    let mut u_var3: u16;
    let pu_var4: &mut u16;
    let mut in_stack_0000ffde: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    _local_6 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffde, 2));
    process_struct_1010_6118(_local_6);
    pu_var4 = &local_8;
    pu_var2 = &local_a;
    u_var3 = unaff_ss;
    ppVar1 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(unaff_ss, pu_var2),
        CONCAT22(u_var3, pu_var4),
    );
    u_var3 = (param_1 >> 0x10);
    move_window_1040_826c(param_1, u_var3, local_a + 0x8c, local_8 + 0xb9);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn enable_window_1040_060e(param_1: u32, param_2: u16) {
    let pi_var1: &mut u16;
    let h_wnd: HWND16;
    let mut unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    _local_8 = CONCAT22(unaff_ss, &stack0x000a);
    local_a = param_2;
    while (true) {
        pi_var1 = _local_8;
        if (local_a == 0) {
            break;
        }
        _local_8 = (_local_8 & 0xffff0000 | (local_8 + 2));
        local_a = local_a - 1;
        h_wnd = GetDlgItem16(unsafe { *pi_var1 }, (param_1 + 6));
        EnableWindow16(0, h_wnd);
    }
    return;
}

pub fn enable_window_1040_0558(param_1: u32, param_2: i32) -> LRESULT {
    let mut i_var1: i32;
    let l_param: LPARAM;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let LVar6: LRESULT;
    let w_param: WPARAM16;
    let mut msg: u16;
    let id: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    i_var1 = param_2 * 0xe;
    HVar2 = GetDlgItem16((i_var1 + 0x5c64), (i_var4 + 6));
    i_var3 = pass1_1010_659a((i_var4 + 0x8e), (i_var1 + 0x5c66));
    if ((i_var3 == 0) && ((i_var1 + 0x5c66) != 10)) {
        EnableWindow16(0, HVar2);
        HVar2 = (i_var4 + 6);
        id = (param_2 * 0xe + 0x5c68);
        u_var5 = 0x531;
    } else {
        EnableWindow16(1, HVar2);
        HVar2 = (i_var4 + 6);
        id = (param_2 * 0xe + 0x5c68);
        u_var5 = 0x649;
    }
    msg = 0xc;
    w_param = 0;
    l_param = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        u_var5,
    );
    LVar6 = SendDlgItemMessage16(l_param, w_param, msg, id, HVar2);
    return LVar6;
}

pub fn destroy_win_1040_0170(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let h_var4: HWND16;
    let ppVar5: &mut pass1_struct_2;
    let mut u_var6: u16;

    let mut unaff_si: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let pp_var8: &mut pass1_struct_1;
    let u_var9: u8;
    let u_var10: u8;
    let u_var11: u8;
    let u_var12: u8;
    let mut u_var13: u16;
    let mut local_12a: u16;
    let mut local_128: u16;
    let HStack294: HWND16;
    let mut uStack292: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 8;
    local_6 = 0;
    match (param_2._2_2_) {
        0x167 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x168, 0x69, 0x16a);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 0
        }
        0x168 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x69, 0x16a);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 1
        }
        0x169 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x68, 0x16a);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 2
        }
        0x16a => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x68, 0x169);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 3
        }
        0x16b => {
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            u_var7 = offset;
            EnableWindow16(0, h_var4);
            if ((param_1 + 0x92) != 3) {
                u_var7 = &ctx.PTR_LOOP_1050_1008;
                mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1de);
            }
            if ((param_1 + 0x92) != 8) {
                i_var3 = (param_1 + 0x92) * 0xe;
                local_6 = (i_var3 + 0x5c6c);
                u_var7 = 0x1010;
                pass1_1010_6604((param_1 + 0x8e), (i_var3 + 0x5c66));
                (param_1 + 0x92) = 8;
            }
            local_8 = 0;
            while (local_8 < 4) {
                enable_window_1040_0558(param_1, param_2_00, local_8);
                local_8 = local_8 + 1;
            }
            // goto LAB_1040_04da;
        }
        0x16c => {
            h_var4 = GetDlgItem16(0x16d, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 5;
            (param_1 + 0x94) = 5;
            // goto LAB_1040_01bf;
        }
        0x16d => {
            h_var4 = GetDlgItem16(0x16d, (param_1 + 6));
            EnableWindow16(0, h_var4);
            mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1de);
            if ((param_1 + 0x94) != 8) {
                i_var3 = (param_1 + 0x94) * 0xe;
                local_6 = (i_var3 + 0x5c6c);
                pass1_1010_6604((param_1 + 0x8e), (i_var3 + 0x5c66));
                (param_1 + 0x94) = 8;
            }
            enable_window_1040_0558(param_1, param_2_00, 5);
            _local_c =
                struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x39));
            local_10 = (_local_c + 0x20);
            u_var11 = SUB21(&local_16, 0);
            u_var12 = (&local_16 >> 8);
            u_var9 = SUB21(&local_18, 0);
            u_var10 = (&local_18 >> 8);
            u_var7 = (local_10 >> 0xf) + 0x200;
            u_var13 = unaff_ss;
            local_e = u_var7;
            local_8 = local_10;
            ppVar5 = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                CONCAT22(u_var7, local_10),
            );
            _local_14 = CONCAT22(u_var7, ppVar5);
            pass1_1030_2f1a(
                CONCAT22(u_var7, ppVar5),
                CONCAT22(unaff_ss, CONCAT11(u_var10, u_var9)),
                CONCAT22(u_var13, CONCAT11(u_var12, u_var11)),
            );
            local_16 = local_16 + (local_18 - local_16) / 2;
            local_1a = pass1_1030_2fac(_local_14);
            u_var2 = (param_1 + 0x96);
            u_var7 = 0x1018;
            win_fn_1018_6086(u_var2, (u_var2 >> 0x10), local_1a, local_16);
            // goto LAB_1040_04da;
        }
        0x16e => {
            _local_1e =
                struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x39));
            local_1a = (_local_1e + 0x20);
            local_18 = LoadCursor16(0x7f02, 0);
            local_16 = SetCursor16(local_18);
            local_12a = (local_1a + 0x2000000 >> 0x10);
            pass1_1030_532e(CONCAT22(unaff_ss, &local_12a), local_1a + 0x2000000);
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_12a));
            local_12a = (ctx._g_bool_1050_5748 >> 0x10);
            pass1_1030_838e(ctx._g_bool_1050_5748);
            local_12a = (ctx._g_bool_1050_5748 >> 0x10);
            pass1_1030_8334();
            local_12a = local_16;
            SetCursor16(local_16);
            local_128 = ctx.g_h_window;
            local_12a = 0x111;
            PostMessage16(0, 0x7e, 0x111, ctx.g_h_window);
            HStack294 = (param_1 + 6);
            local_128 = offset;
            u_var7 = offset;
            local_12a = 0x495;
            DestroyWindow16(HStack294);
            // goto LAB_1040_04da;
        }
        // default:
        _ => {
            post_win_msg_1040_7b3c();
            return;
        }
    }
    (param_1 + 0x92) = local_4;
    // LAB_1040_01bf:
    u_var7 = offset;
    // LAB_1040_04da:
    if (local_4 != 8) {
        uStack292 = (uStack292 & 0xffff0000 | *(param_1 + 6));
        HStack294 = (local_4 * 0xe + 0x5c68);
        local_12a = 0;
        local_128 = 0xc;
        u_var6 = local_4;
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            (local_4 * 0xe + 0x5c6a),
        );
        u_var7 = offset;
        SendDlgItemMessage16(
            CONCAT22(ctx.dx_reg, u_var6),
            local_12a,
            local_128,
            HStack294,
            uStack292,
        );
    }
    if (local_6 != 0) {
        uStack292 = CONCAT22(uStack292._2_2_, 2);
        local_128 = ctx._g_Struct372_1050_0ed0;
        HStack294 = (ctx._g_Struct372_1050_0ed0 >> 0x10);
        local_12a = u_var7;
        pp_var8 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, uStack292);
        u_var1 = (pp_var8 + 0x20);
        _local_1e = (_local_1e & 0xffff0000 | u_var1);
        if (u_var1 != 0) {
            uStack292 = (uStack292 & 0xffff0000 | ctx.g_h_window);
            HStack294 = 0x111;
            local_128 = local_6;
            local_12a = 0;
            PostMessage16(0, local_6, 0x111, ctx.g_h_window);
        }
    }
    return;
}

pub fn show_window_1038_ea18(param_1: &mut Struct20) {
    let i_var1: u16;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: HWND16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    _local_6 = pass1_1010_375e((i_var3 + 0x8e));
    local_8 = GetDlgItem16(0xfa5, (i_var3 + 6));
    SendMessage16(_local_6, 0, 0xc, local_8);
    GetWindowRect16(CONCAT22(&local_10, 0x1538), unaff_ss);
    i_var1 = GetSystemMetrics16(4);
    i_var2 = i_var1 + local_e + 5;
    move_window_1040_826c(i_var3, u_var4, i_var2, i_var2);
    ShowWindow16(5, (i_var3 + 6));
    return;
}

pub fn destroy_win_1038_eaa2(param_1: u32, param_2: i32) {
    let h_wnd: HWND16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_56: u16;
    let local_54: [HWND16; 41];

    i_var1 = param_1;
    u_var2 = (param_1 >> 0x10);
    if (param_2 != 0) {
        h_wnd = GetDlgItem16(0xfa5, (i_var1 + 6));
        SendMessage16(CONCAT22(unaff_ss, local_54), 0x50, 0xd, h_wnd);
        pass1_1010_3770((i_var1 + 0x8e), CONCAT22(unaff_ss, local_54));
        PostMessage16(0, 0xfb, 0x111, (i_var1 + 8));
    }
    local_54[0] = (i_var1 + 6);
    DestroyWindow16(local_54[0]);
    return;
}

pub fn show_destroy_win_1038_e71c(param_1: &mut Struct20) {


    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    string_fn_1010_2c34((i_var1 + 0x8e));
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var1 + 0x10)),
        CONCAT22(ctx.dx_reg, in_ax),
    );
    error_check_1000_17ce(_local_6);
    move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (i_var1 + 6));
    (i_var1 + 0x92) = 1;
    process_win_msg_1008_9510();
    DestroyWindow16((i_var1 + 6));
    return;
}

pub fn show_window_1038_cb5c(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let struct_a: &mut Struct199;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let pu_var5: &mut u16;
    let pu_var6: &mut u16;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = ret_5_1008_eb6e();
    local_a = 0;
    while (local_a < u_var2) {
        u_var1 = (i_var3 + 0x8e);
        pu_var5 = pass1_1008_eb5c(u_var1, (u_var1 >> 0x10), local_a);
        struct_a = (pu_var5 >> 0x10);
        pu_var6 = pu_var5;
        process_struct_1000_179c(0x42, struct_a);
        if (pu_var6 != 0x0) {
            misc::win_fn_1008_3bd6(
                pu_var6,
                (pu_var6 >> 0x10),
                0,
                unsafe { CONCAT22(*pu_var5, (pu_var5 + 2)) },
                0x1000101,
                CONCAT22((pu_var5 + 4), 0xff),
                CONCAT22(in_stack_0000fff2, (i_var3 + 6)),
            );
        }
        local_a = local_a + 1;
    }
    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x90001);
    ShowWindow16(5, (i_var3 + 6));
    return;
}

pub fn destroy_win_1038_c95e(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 10) = 0;
    } else {
        u_var2 = IsDlgButtonChecked16(0xfac, (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16(0xfad, (i_var3 + 6));
            if (u_var2 == 0) {
                u_var2 = IsDlgButtonChecked16(0xfae, (i_var3 + 6));
                if (u_var2 == 0) {
                    u_var2 = IsDlgButtonChecked16(0xfaf, (i_var3 + 6));
                    if (u_var2 == 0) {
                        u_var2 = IsDlgButtonChecked16(0xfb0, (i_var3 + 6));
                        if (u_var2 != 0) {
                            u_var1 = (i_var3 + 0x8e);
                            (u_var1 + 10) = 5;
                        }
                    } else {
                        u_var1 = (i_var3 + 0x8e);
                        (u_var1 + 10) = 4;
                    }
                } else {
                    u_var1 = (i_var3 + 0x8e);
                    (u_var1 + 10) = 3;
                }
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 10) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 10) = 1;
        }
    }
    DestroyWindow16((i_var3 + 6));
    PTR_LOOP_1050_5b80 = 0x0;
    return;
}

pub fn enable_window_1038_c294(param_1: u32) {
    let lp_string: SEGPTR;
    let mut u_var1: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    _local_c = param_1 & 0xffff0000 | (param_1 + 0x9e);
    lp_string = pass1_1008_e320(
        (param_1 + 0x8e),
        param_1 & 0xffff0000 | (param_1 + 0x19e),
        param_1 & 0xffff0000 | (param_1 + 0x9e),
    );
    SetWindowText16(lp_string, (param_1 + 0x9a));
    u_var1 = string_fn_1008_e2a4(
        (param_1 + 0x8e),
        param_1 & 0xffff0000 | (param_1 + 0x19e),
        _local_c,
    );
    EnableWindow16(u_var1 & 1, (param_1 + 0x96));
    EnableWindow16(u_var1 & 2, (param_1 + 0x98));
    return;
}

pub fn set_window_pos_1038_c31a(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_3;
    local_6 = param_2;
    if (param_3 == 1) {
        enable_window_1038_c294(param_1);
    } else {
        if (param_3 != 7) {
            return 0;
        }
        GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
        local_a = local_a - local_e;
        SetWindowPos16(2, 0x50, local_a, 0, 0, 0, local_6);
    }
    return 1;
}

pub fn show_window_1038_b634(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xac) == 0) {
        (i_var2 + 0xac) = 1;
        local_4 = 1;
        while (local_4 < 0x2b) {
            if (((local_4 * 4 + i_var2 + 2) | (local_4 * 4 + i_var2)) != 0) {
                u_var1 = (local_4 * 4 + i_var2);
                ShowWindow16(0, (u_var1 + 6));
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn show_win_1038_b68a(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xac) != 0) {
        (i_var2 + 0xac) = 0;
        local_4 = 1;
        while (local_4 < 0x2b) {
            if (((local_4 * 4 + i_var2 + 2) | (local_4 * 4 + i_var2)) != 0) {
                u_var1 = (local_4 * 4 + i_var2);
                ShowWindow16(1, (u_var1 + 6));
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn set_window_pos_1038_abdc(param_1: u32) {
    let mut u_var1: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_12: [u8; 2];
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (param_1 >> 0x10);
    GetWindowRect16(CONCAT22(&local_a, unaff_cs), unaff_ss);
    GetDlgItem16(0xfd7, (param_1 + 6));
    GetWindowRect16(CONCAT22(local_12, 0x1538), unaff_ss);
    local_6 = local_6 - local_a;
    local_4 = (local_10 - local_8) - 2;
    SetWindowPos16(6, local_4, local_6, 0, 0, 0, (param_1 + 6));
    return;
}

pub fn enable_window_1038_a8f8(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let h_wnd: HWND16;
    let enable: bool;

    if (param_3._2_2_ == 0x116) {
        SendDlgItemMessage16(0, 1, 0x401, 0x11a, (param_1 + 6));
        h_wnd = GetDlgItem16(0x11a, (param_1 + 6));
        enable = 0;
    } else {
        if ((param_3._2_2_ == 0x116) || (2 < param_3._2_2_ - 0x117)) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        h_wnd = GetDlgItem16(0x11a, (param_1 + 6));
        enable = 1;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn enable_window_1038_a972(param_1: &mut Struct20) {
    let h_wnd: HWND16;
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;

    misc::win_gui_func_1040_78e2(param_1);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    SendDlgItemMessage16(0, 1, 0x401, 0x116, (i_var2 + 6));
    SendDlgItemMessage16(0, 1, 0x401, 0x11a, (i_var2 + 6));
    h_wnd = GetDlgItem16(0x11a, (i_var2 + 6));
    EnableWindow16(0, h_wnd);
    u_var1 = mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x40001);
    (i_var2 + 0x8c) = u_var1;
    metrics::get_system_metrics_1038_a18c(i_var2, u_var3);
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub fn show_window_1038_a6f4(param_1: &mut Struct20) {
    let lp_string: SEGPTR;
    let hwnd: HWND16;
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let ppVar4: &mut pass1_struct_1;
    let LVar5: LRESULT;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    ppVar4 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffec, 2));
    lp_string = (ppVar4 + 0x68);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    hwnd = GetDlgItem16(0x115, (i_var2 + 6));
    SetWindowText16(lp_string, hwnd);
    SetFocus16(hwnd);
    LVar5 = SendMessage16(-0x10000, 0, 0x401, hwnd);
    u_var1 = LVar5;
    metrics::get_system_metrics_1038_a18c(param_1);
    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x30001);
    (i_var2 + 0x8c) = u_var1;
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub fn show_window_1038_a4ee(param_1: &mut Struct20) {
    let lp_string: SEGPTR;

    let hwnd: HWND16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let ppVar3: &mut pass1_struct_1;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    misc::win_gui_func_1040_78e2(param_1);
    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x20001);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x8c) = in_ax;
    ppVar3 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffec, 2));
    lp_string = (ppVar3 + 0x6c);
    hwnd = GetDlgItem16(0x114, (i_var1 + 6));
    SetWindowText16(lp_string, hwnd);
    SetFocus16(hwnd);
    SendMessage16(-0x10000, 0, 0x401, hwnd);
    metrics::get_system_metrics_1038_a18c(param_1);
    ShowWindow16(5, (i_var1 + 6));
    return;
}

pub fn show_window_1038_a396(param_1: &mut Struct20) {
    let mut u_var1: u16;
    let mut u_var2: u16;

    misc::win_gui_func_1040_78e2(param_1);
    metrics::get_system_metrics_1038_a18c(param_1);
    u_var1 = mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x10001);
    u_var2 = (param_1 >> 0x10);
    (param_1 + 0x8c) = u_var1;
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn show_window_1038_9fd0(param_1: &mut Struct20) {
    misc::win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn enable_window_1038_9cec(
    param_1: &mut Struct124,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i32,
) {
    let paVar1: &mut Struct124;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut unaff_si: u16;
    let ppVar5: &mut pass1_struct_1;
    let enable: bool;
    let h_var6: HWND16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    if (param_5 == 0xeb) {
        win_gui_fn_1040_b54a(param_1, param_2, param_3_00, CONCAT22(0xeb, param_3));
        ppVar5 = struct_ops::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
        i_var4 = ppVar5 + 0xa4;
        u_var3 = (ppVar5 >> 0x10);
        local_c = 0;
        while (i_var2 = local_c * 2, (i_var2 + i_var4) != 0) {
            h_var6 = GetDlgItem16((i_var2 + i_var4), &param_1.field_0x6);
            (&param_1[1].field_0x0 + i_var2) = h_var6;
            local_c = local_c + 1;
            paVar1 = (param_1 + 2);
            paVar1 = paVar1 + 1;
        }
    } else {
        if (param_5 == 0xf8) {
            h_var6 = GetDlgItem16(0x17d8, &param_1.field_0x6);
            enable = 1;
        } else {
            if (param_5 != 0x17d8) {
                win_gui_fn_1040_b54a(param_1, param_2, param_3_00, CONCAT22(param_5, param_3));
                return;
            }
            SetWindowPos16(6, 0xed, 0x237, 0, 0, 0, &param_1.field_0x6);
            enable = offset;
            GetDlgItem16(0x17d8, &param_1.field_0x6);
            h_var6 = 0;
        }
        EnableWindow16(enable, h_var6);
    }
    return;
}

pub fn enable_window_1038_9a66(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let enable: bool;
    let h_wnd: HWND16;

    if (param_3._2_2_ == 0xf8) {
        h_wnd = GetDlgItem16(0x17d9, &param_1.field_0x6);
        enable = 1;
    } else {
        if (param_3._2_2_ != 0x17d9) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        enable = 0;
        SetWindowPos16(6, 0x1a0, 300, 0, 0, 0, &param_1.field_0x6);
        h_wnd = 0;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn set_window_text_1038_8358(param_1: Vec<u8>) {
    let pu_var1: Vec<u8>;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_30a: [u8; 258];
    let mut local_208: [u8; 256];
    let mut local_108: [u8; 256];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    local_4 = GetDlgItem16((s_logo_bmp_1050_1850 + 7), (i_var3 + 6));
    _local_8 = pass1_1008_b820((i_var3 + 0x94));
    if (_local_8 == 0) {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_30a),
            0x449,
        );
        pu_var1 = local_30a;
    } else {
        i_var2 = dialog::send_dlg_item_msg_1038_8164(
            param_1,
            CONCAT22(unaff_ss, local_108),
            (s_logo_bmp_1050_1850 + 4),
        );
        if (i_var2 == 0) {
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x100,
                CONCAT22(unaff_ss, local_208),
                0x447,
            );
        } else {
            load_string_1008_b65a(
                (i_var3 + 0x94),
                CONCAT22(unaff_ss, local_208),
                CONCAT22(unaff_ss, local_108),
            );
        }
        pu_var1 = local_208;
    }
    SetWindowText16(CONCAT22(unaff_ss, pu_var1), local_4);
    return;
}

pub fn enable_window_1038_806a(param_1: u32) {
    let HVar1: HWND16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    HVar1 = GetDlgItem16(1, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_logo_bmp_1050_1850 + 8), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16(s_650_bmp_1050_1859, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    u_var4 = pass1_1008_b820((i_var2 + 0x94));
    if (u_var4 != 0) {
        u_var4 = pass1_1008_b340((i_var2 + 0x94));
        u_var5 = pass1_1008_b366((i_var2 + 0x94));
        u_var6 = pass1_1008_b47a((i_var2 + 0x94));
        if (((u_var4 != 0) && (u_var5 != 0)) && (u_var6 != 0)) {
            HVar1 = GetDlgItem16(1, (i_var2 + 6));
            EnableWindow16(1, HVar1);
            HVar1 = GetDlgItem16((s_logo_bmp_1050_1850 + 8), (i_var2 + 6));
            EnableWindow16(1, HVar1);
        }
        if (u_var4 != 0) {
            HVar1 = GetDlgItem16(s_650_bmp_1050_1859, (i_var2 + 6));
            EnableWindow16(1, HVar1);
        }
    }
    return;
}

pub fn destroy_win_1020_42f4(in_struct_1: &mut Struct44) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_cs: u16;
    let mut HVar3: u16;

    u_var2 = (in_struct_1 >> 0x10);
    i_var1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x623c;
    (i_var1 + 2) = 0x1020;
    (i_var1 + 0xe2) = 0x62d8;
    (i_var1 + 0xe4) = 0x1020;
    HVar3 = unaff_cs;
    if ((i_var1 + 0x106) != 0) {
        HVar3 = offset;
        DestroyMenu16(unaff_cs);
    }
    DestroyCursor16(HVar3, (i_var1 + 0xf0));
    DestroyCursor16();
    process_struct_1020_808e(in_struct_1);
    return;
}

pub fn bring_window_to_top_1020_2aae(param_1: &mut Vec<u8>, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    pass1_1008_3e0e(param_1);
    BringWindowToTop16((param_1 + 8));
    pass1_1018_169e((param_1 + 0xf2), param_2);
    return;
}

pub fn win_cleanup_func_1020_2fea(param_1: &mut Struct44) {
    let h_gdi_obj: HPALETTE16;
    let local_bx_4: &mut Struct44;
    let mut in_stack_00000006: u16;

    *_param_1 = 0x363c;
    param_1.ptr_a_hi = 0x1020;
    if (param_1.field_0x14 != 0) {
        pass1_1010_1ea6(param_1.field_0x14, CONCAT22(in_stack_00000006, param_1));
    }
    SelectObject16(param_1.field_0x1c, param_1.field_0x18);
    SelectObject16(param_1.field_0x1e, param_1.field_0x18);
    DeleteObject16(param_1.field_0x1a);
    h_gdi_obj = SelectPalette16(0, param_1.field_0x20, param_1.field_0x18);
    DeleteObject16(h_gdi_obj);
    DeleteDC16(param_1.field_0x18);
    *_param_1 = ctx.s_0_020_1050_3ab0;
    param_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    *_param_1 = ctx.s_1_1050_389a;
    param_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn cleanup_fn_1020_2a6a(in_struct_1: &mut Struct652) {
    let local_struct_1: &mut Struct652;
    let local_struct_1_hi: &mut Struct652;

    get_sys_metrics_1020_7a50(in_struct_1);
    pass1_1018_0ae8((in_struct_1 + 0xf2), 0);
    icon::destroy_icon_1020_2c88(in_struct_1);
    return;
}

pub fn send_win_fn_1040_4cb2(param_1: Vec<u8>) -> LRESULT {
    let h_wnd: HWND16;
    let l_param: LPARAM;
    let LVar1: LRESULT;
    let w_param: WPARAM16;
    let mut msg: u16;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    dialog::set_dlg_item_txt_1040_b45e(param_1, u_var2);
    h_wnd = GetDlgItem16(6000, (param_1 + 6));
    w_param = 0xffff;
    msg = 0x40d;
    l_param = pass1_1040_4d7e(param_1);
    pass1_1040_4dcc(param_1, l_param);
    LVar1 = SendMessage16(l_param, w_param, msg, h_wnd);
    return LVar1;
}

pub fn enable_win_1040_42b2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let pu_var3: Vec<u8>;
    let mut u_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let LVar7: LRESULT;
    let mut local_66: u32;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    i_var5 = param_1;
    u_var6 = (param_1 >> 0x10);
    if (param_2 == 0) {
        (i_var5 + 0x9a) = 1;
        DestroyWindow16((i_var5 + 6));
        return;
    }
    pass1_1000_4906(CONCAT22(unaff_ss, local_54), 0, 0x51);
    HVar2 = GetDlgItem16(0xfb5, (i_var5 + 6));
    LVar7 = SendMessage16(CONCAT22(unaff_ss, local_54), 0x51, 0xd, HVar2);
    u_var4 = (LVar7 >> 0x10);
    pu_var3 = local_54;
    pass1_fn_1000_3e2c(CONCAT22(unaff_ss, pu_var3));
    if ((u_var4 | pu_var3) != 0) {
        (i_var5 + 0x92) = pu_var3;
        (i_var5 + 0x94) = u_var4;
    }
    if (u_var4 < 0) {
        u_var1 = (i_var5 + 0x8e);
        wsprintf16(
            local_54,
            CONCAT22(0x5d3c, unaff_ss),
            CONCAT22((u_var1 + 0x76), 0x1050),
        );
        SendMessage16(CONCAT22(unaff_ss, local_54), 0, 0xc, HVar2);
        SetFocus16(HVar2);
        SendMessage16(-0x10000, 0, 0x401, HVar2);
        return;
    }
    HVar2 = GetDlgItem16(1, (i_var5 + 6));
    EnableWindow16(0, HVar2);
    u_var1 = (i_var5 + 0x8e);
    (u_var1 + 0x76) = (i_var5 + 0x92);
    PostMessage16((i_var5 + 0x92), 0, 0x400, ctx.g_h_window);
    HVar2 = GetDlgItem16(1, (i_var5 + 6));
    EnableWindow16(1, HVar2);
    return;
}

pub fn get_win_rect_1040_43ea(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    GetWindowRect16(CONCAT22(&local_a, unaff_cs), unaff_ss);
    local_6 = local_6 - local_a;
    local_4 = local_4 - local_8;
    pass1_1010_5fb0((i_var2 + 0x8e), 0, &local_a, unaff_ss, 7);
    u_var1 = (i_var2 + 0x8e);
    (u_var1 + 0x7a) = ((i_var2 + 0x9a) == 0);
    return;
}

pub fn enable_window_1040_3a36(param_1: u32, param_2: u16, param_3: u16, param_4: i32) -> u16 {
    let pi_var1: &mut i32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    b_var2 = false;
    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        if ((i_var3 + 0x9e) <= (i_var3 + 0x9c)) {}
        // goto LAB_1040_3a79;
        pi_var1 = (i_var3 + 0x9c);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1040_3a79;
        if ((i_var3 + 0x9c) == 0) {}
        // goto LAB_1040_3a79;
        pi_var1 = (i_var3 + 0x9c);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
    }
    b_var2 = true;
    // LAB_1040_3a79:
    if (b_var2) {
        SetDlgItemInt16(0, *(i_var3 + 0x9c), 0x18e, (i_var3 + 6));
    }
    if (((i_var3 + 0x9c) != 0) && ((i_var3 + 0xa2) == 0)) {
        (i_var3 + 0xa2) = 1;
        EnableWindow16(1, (i_var3 + 0x9a));
    }
    if (((i_var3 + 0x9c) == 0) && ((i_var3 + 0xa2) != 0)) {
        (i_var3 + 0xa2) = 0;
        EnableWindow16(0, (i_var3 + 0x9a));
    }
    return 0;
}

pub fn gui_window_func_1038_b72e(param_1: u32, param_2: i32) -> u16 {
    let hwnd: HWND16;
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    if ((param_2 * 4 + param_1) != 0) {
        u_var1 = (param_2 * 4 + param_1);
        hwnd = (u_var1 + 6);
        SetFocus16(hwnd);
        BringWindowToTop16(hwnd);
        return 1;
    }
    return 0;
}

pub fn destroy_win_1020_8250(param_1: u32) {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xec) != 0) {
        b_var1 = IsWindow16((i_var2 + 0xec));
        if (b_var1 != 0) {
            DestroyWindow16((i_var2 + 0xec));
            (i_var2 + 0xec) = 0;
        }
    }
    return;
}

pub unsafe fn win_fn_1020_694c(ctx: &mut AppContext, param_1: &mut Struct37, param_2: i32) -> u32 {
    let mut u_var1: u32;
    let mut lp_help_file: String;
    let mut u_var2: i32;
    let mut HVar3: HWND16;
    let local_bx_14: Struct5;
    let mut u_var4: u16;
    let u_var5: u8;

    u_var2 = param_2;
    if (param_2 != 299) {
        local_bx_14 = param_1;
        u_var4 = (param_1 >> 0x10);
        if (param_2 < 300) {
            if (param_2 == 0x6f) {
                lp_help_file = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
                HVar3 = WinHelp16(0x29, 1, lp_help_file, local_bx_14.field_0x8);
                return HVar3;
            }
            if (param_2 == 0xeb) {
                u_var2 = GetDlgItem16(0x1797, local_bx_14.field_0x8);
                if (u_var2 != 0) {
                    // LAB_1020_6a6f:
                    HVar3 = create_win_1020_6e98(param_1);
                    return HVar3;
                }
            } else {
                if (param_2 == 0xef) {
                    pass1_1018_2e28(local_bx_14.field_0xf2);
                    u_var2 = pass1_1008_3e0e(param_1);
                } else {
                    u_var2 = param_2 - 0x129;
                    if ((u_var2 != 0) && (u_var2 = param_2 - 0x12a, u_var2 == 0)) {
                        HVar3 = local_bx_14.field_0x8;
                        u_var5 = 0x12;
                        // LAB_1020_69c3:
                        HVar3 = PostMessage16(0, CONCAT11(0xf0, u_var5), 0x112, HVar3);
                        return HVar3;
                    }
                }
            }
        } else {
            if (param_2 == 3000) {
                HVar3 = GetDlgItem16(0x1797, local_bx_14.field_0x8);
                if (HVar3 != 0) {
                    DestroyWindow16(HVar3);
                }
                u_var2 = pass1_1018_31d0(local_bx_14.field_0xf2);
                if (u_var2 != 0) {
                    u_var1 = local_bx_14.field_0xf2;
                    pass1_1018_2d9a(u_var1, (u_var1 >> 0x10));
                    // LAB_1020_6a0b:
                    u_var1 = local_bx_14.field_0xf6;
                    HVar3 = rect::invalidate_rect_1020_735a(u_var1, (u_var1 >> 0x10));
                    return HVar3;
                }
            } else {
                if (param_2 < 0xbb9) {
                    if (param_2 == 300) {
                        HVar3 = local_bx_14.field_0x8;
                        u_var5 = 0x20;
                        // goto LAB_1020_69c3;
                    }
                    u_var2 = param_2 - 0x12d;
                    if (param_2 != 300) {
                        u_var2 = param_2 - 0x12e;
                    }
                } else {
                    if (param_2 == 0xbb9) {
                        HVar3 = GetDlgItem16(0x1797, local_bx_14.field_0x8);
                        if (HVar3 != 0) {
                            DestroyWindow16(HVar3);
                        }
                        u_var2 = pass1_1018_31d0(local_bx_14.field_0xf2);
                        if (u_var2 != 0) {
                            u_var1 = local_bx_14.field_0xf2;
                            pass1_1018_2dde(u_var1, (u_var1 >> 0x10));
                            // goto LAB_1020_6a0b;
                        }
                    } else {
                        u_var2 = param_2 - 0xbba;
                        if (u_var2 == 0) {
                            HVar3 = GetDlgItem16(0x1797, local_bx_14.field_0x8);
                            if (HVar3 != 0) {
                                HVar3 = DestroyWindow16(HVar3);
                                return HVar3;
                            }
                            // goto LAB_1020_6a6f;
                        }
                    }
                }
            }
        }
    }
    return u_var2;
}

pub fn destroy_win_1020_6ae6(param_1: &mut u32, param_2: u16, param_4: i32, param_3: i32) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut stack0xffa8: u16;

    if (param_3 == 0x1797) {
        u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        local_4 = GetDlgItem16(0x1797, (i_var3 + 8));
        if (local_4 != 0) {
            if (param_2 == 2) {
                local_8 = SendMessage16(0, 0, 0x409, local_4);
                if (local_8 != -1) {
                    SendMessage16(
                        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &stack0xffa8)),
                        local_8,
                        0x40a,
                        local_4,
                    );
                    pass1_1018_30ca((i_var3 + 0xf2), CONCAT22(unaff_ss, &stack0xffa8));
                    i_var2 = pass1_1018_2fe8();
                    if (i_var2 != 0) {
                        rect::invalidate_rect_1020_735a();
                        let param_1_val = unsafe { *param_1 };
                        pp_var1 = (param_1_val + 0x40);
                        (**pp_var1)(0x18, i_var3);
                    }
                }
            } else {
                if (param_2 != 3) {
                    return;
                }
            }
            DestroyWindow16(local_4);
        }
    }
    return;
}

pub unsafe fn call_win_fn_1020_6bbc(in_struct_1: Vec<u8>) {
    win_fn_1020_737a((in_struct_1 + 0xf6));
    return;
}

pub unsafe fn update_window_1020_6c3a(ctx: &mut AppContext, param_1: &mut win_struct_42) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: i32;
    let pi_var5: *mut u16;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let struct_a: Struct199;
    let pa_var8: Struct199;
    let struct_a_00: Struct199;
    let struct_a_01: Struct199;
    let struct_a_02: Struct199;



    let mut i_var9: i32;
    let mut unaff_si: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let pp_var11: *mut pass1_struct_1;
    let u_var12: u8;
    let u_var13: u8;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut offset: u16;

    create_win_1008_9760(param_1);
    pp_var11 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(unaff_si, 4));
    u_var7 = (pp_var11 >> 0x10);
    u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    (i_var9 + 0xf2) = pp_var11;
    (i_var9 + 0xf4) = u_var7;
    u_var3 = (i_var9 + 0xf2);
    (i_var9 + 0xe0) = u_var3;
    (i_var9 + 0xe2) = u_var7;
    LoadIcon16(
        0x1010,
        ctx.s_TILEICON_1050_440c,
        &ctx.g_alloc_addr_1050_1050,
        ctx.g_h_instance_1050_038c,
    );
    (i_var9 + 0xc2) = u_var3;
    u_var6 = (i_var9 + 0xf2);
    u_var7 = u_var6;
    ppc_var2 = ((i_var9 + 0xf2) + 0x30);
    ppc_var2(offset, u_var7, (u_var6 >> 0x10), u_var3);
    u_var4 = &local_6 + 2;
    u_var12 = unaff_ss;
    u_var13 = (unaff_ss >> 8);
    pass1_1018_2d22(
        (i_var9 + 0xf2),
        CONCAT22(unaff_ss, &local_6),
        CONCAT13(u_var13, CONCAT12(u_var12, u_var4)),
        3000,
    );
    pa_var8 = struct_a;
    process_struct_1000_179c(0x42, struct_a);
    if ((pa_var8 | u_var4) != 0) {
        win_fn_1008_3bd6(
            u_var4,
            pa_var8,
            0,
            local_6,
            0x7d0000,
            0xbb8007c,
            CONCAT22(u_var7, (i_var9 + 8)),
        );
    }
    u_var4 = &local_6 + 2;
    pass1_1018_2d22(
        (i_var9 + 0xf2),
        CONCAT22(unaff_ss, &local_6),
        CONCAT13(u_var13, CONCAT12(u_var12, u_var4)),
        0xbb9,
    );
    pa_var8 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    if ((pa_var8 | u_var4) != 0) {
        win_fn_1008_3bd6(
            u_var4,
            pa_var8,
            0,
            local_6,
            0x7f0000,
            0xbb9007e,
            CONCAT22(u_var7, (i_var9 + 8)),
        );
    }
    u_var4 = &local_6 + 2;
    pass1_1018_2d22(
        (i_var9 + 0xf2),
        CONCAT22(unaff_ss, &local_6),
        CONCAT13(u_var13, CONCAT12(u_var12, u_var4)),
        0xbba,
    );
    pa_var8 = struct_a_01;
    process_struct_1000_179c(0x42, struct_a_01);
    struct_a_02 = (pa_var8 | u_var4);
    if (struct_a_02 != 0x0) {
        win_fn_1008_3bd6(
            u_var4,
            pa_var8,
            0,
            local_6,
            0x1b101b2,
            0xbba01b0,
            CONCAT22(u_var7, (i_var9 + 8)),
        );
        struct_a_02 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x22, struct_a_02);
    if ((struct_a_02 | u_var4) == 0) {
        (i_var9 + 0xf6) = 0;
    } else {
        drawing_context::get_dc_1020_717e();
        (i_var9 + 0xf6) = u_var4;
        (i_var9 + 0xf8) = ctx.dx_reg;
    }
    u_var6 = (i_var9 + 0xf6);
    (i_var9 + 0xe8) = u_var6;
    u_var1 = (i_var9 + 0xf2);
    ppc_var2 = ((i_var9 + 0xf2) + 0x10);
    ppc_var2(0x1000, u_var1, (u_var1 >> 0x10));
    pi_var5 = u_var6;
    let pi_var5_val = unsafe { *pi_var5 };
    MoveWindow16(
        1,
        pi_var5[3],
        pi_var5[2],
        pi_var5[1],
        pi_var5_val,
        (i_var9 + 8),
    );
    u_var6 = param_1;
    ppc_var2 = (u_var6 + 0x94);
    ppc_var2(0x38, i_var9, u_var10, 0);
    ppc_var2 = (u_var6 + 0x10);
    ppc_var2(offset, i_var9, (param_1 >> 0x10), 1);
    UpdateWindow16((i_var9 + 8));
    return;
}

pub unsafe fn create_win_1020_6e98(ctx: &mut AppContext, param_1: &mut Struct37) {
    let pu_var1: *mut u16;
    let w_param: WPARAM16;
    let mut in_struct_1: u32;
    let mut h_var2: HWND16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut extraout_d_x: u16;

    let local_bx_5: Struct37;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut send_msg_result: u32;
    let mut l_var6: LRESULT;
    let mut local_24: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 4];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    GetClientRect16(CONCAT22(unaff_ss, local_a), local_bx_5.hwnd_dlg);
    local_e = 0;
    h_var2 = GetDlgItem16(0x1797, local_bx_5.hwnd_dlg);
    if (h_var2 != 0) {
        DestroyWindow16(h_var2);
    }
    pass1_1018_30fc(local_bx_5.field_0xf2, CONCAT22(unaff_ss, &local_e));
    if ((local_e._2_2_ | local_e) != 0) {
        h_var2 = CreateWindow16(
            local_e,
            ctx.g_h_instance_1050_038c,
            0x1797,
            local_bx_5.hwnd_dlg,
            local_4 - 0x19,
            local_6,
            0,
            0,
            0x40a00103,
            0x10504415,
            ctx.s_listbox_1050_4416,
        );
        in_struct_1 = local_e;
        if (h_var2 == 0) {
            if ((local_e._2_2_ | local_e) != 0) {
                pass1_1018_2afa(local_e);
                error_check_1000_17ce(in_struct_1);
                return;
            }
        } else {
            send_msg_result = SendMessage16(0, 0, 0xb, h_var2);
            send_msg_result._0_2_ = (send_msg_result & 0xffff);
            if ((local_e + 4) == 0) {
                load_string_1010_847e(
                    ctx._g_struct_73_1050_14cc,
                    (ctx._g_struct_73_1050_14cc >> 0x10),
                    0x531,
                );
                SendMessage16(CONCAT22(extraout_d_x, send_msg_result), 0, 0x401, h_var2);
            } else {
                local_24 = 0;
                i_var4 = (send_msg_result >> 0x10);
                while (true) {
                    pu_var1 = (local_e + 4);
                    let pu_var1_val = unsafe { *pu_var1 };
                    if (pu_var1_val == local_24 || pu_var1_val < local_24) {
                        break;
                    }
                    big_switch_statement_1020_bd80((local_e + local_24 * 2));
                    l_var6 = SendMessage16(CONCAT22(i_var4, local_24), 0, 0x401, h_var2);
                    local_24 = i_var4 + 1;
                    i_var4 = (l_var6 >> 0x10);
                }
            }
            l_var6 = SendMessage16(0, 1, 0xb, h_var2);
            u_var3 = l_var6;
            pass1_1018_2d84(local_bx_5.field_0xf2);
            l_var6 = SendMessage16(CONCAT22(ctx.dx_reg, u_var3), 0xffff, 0x40d, h_var2);
            w_param = l_var6;
            if ((w_param != 0xffff) || ((l_var6 >> 0x10) != -1)) {
                SendMessage16(0, w_param, 0x407, h_var2);
                SendMessage16(0, w_param, 0x418, h_var2);
            }
            if (local_e != 0) {
                pass1_1018_2afa(local_e);
                error_check_1000_17ce(0x1538704d);
            }
            ShowWindow16(1, h_var2);
            SetFocus16(h_var2);
        }
    }
    return;
}

pub unsafe fn win_fn_1020_737a(ctx: &mut AppContext, in_struct_1: &mut Struct15) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut b_result: u16;
    let pu_var4: Vec<u8>;
    let mut u_var5: u32;


    let local_bx_4: Struct15;
    let mut u_var6: u16;
    let mut unaff_ss: HWND16;
    let u_var7: u8;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut u_stack74;
    let mut u_stack68;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut h_dialog: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut b_result_2: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

    u_var6 = (in_struct_1 >> 0x10);
    local_bx_4 = in_struct_1;
    u_var7 = (unaff_ss >> 8);
    b_result_2 = BeginPaint16(
        CONCAT13(u_var7, CONCAT12(unaff_ss, &local_22)),
        local_bx_4.h_window,
    );
    b_result = IsIconic16(local_bx_4.h_window);
    if (b_result == 0) {
        u_var5 = local_bx_4.field_0x1c;
        u_var5 = (u_var5 + 0x24);
        local_3c = u_var5;
        local_3a = (u_var5 >> 0x10);
        u_var2 = local_bx_4.field_0x1c;
        pass1_1018_2e5e(u_var2, (u_var2 >> 0x10));
        local_30 = (u_var5 & 0xffff | ctx.dx_reg << 0x10);
        pass1_1008_3e54(
            CONCAT13(u_var7, CONCAT12(unaff_ss, &u_stack68 + 2)),
            0,
            0x35,
            0xc,
        );
        if (&local_bx_4.field_0x14 != 0) {
            pass1_1008_5236(&local_bx_4.field_0x14);
        }
        if (local_30 != 0x0) {
            pu_var4 = local_bx_4.field_0x14;
            u_var1 = local_bx_4.field_0x16;
            if ((u_var1 | pu_var4) != 0) {
                pass1_1008_5118(CONCAT22(u_var1, pu_var4));
                error_check_1000_17ce(CONCAT22(u_var1, pu_var4));
            }
            pu_var4 = (&u_stack68 + 2);
            pass1_1008_8ce4(
                local_30,
                CONCAT22(unaff_ss, pu_var4),
                CONCAT22(local_3a, local_3c),
            );
            local_bx_4.field_0x14 = pu_var4;
            local_bx_4.field_0x16 = ctx.dx_reg;
        }
        ppc_var3 = (CONCAT22(local_3a, local_3c) + 4);
        (**ppc_var3)(
            &ctx.PTR_LOOP_1050_1008,
            local_3c,
            local_3a,
            0,
            0,
            &b_result_2,
        );
        ppc_var3 = (local_bx_4.field_0x18 + 0x94);
        (**ppc_var3)(&ctx.PTR_LOOP_1050_1008, local_bx_4.field_0x18, 1);
        h_dialog = GetDlgItem16(0x1797, local_bx_4.h_window);
        if (h_dialog != 0) {
            ShowWindow16(1, h_dialog);
        }
    } else {
        if (ctx.PTR_LOOP_1050_0010 == 0x0) {
            u_var5 = local_bx_4.field_0x1c;
            ppc_var3 = (local_bx_4.field_0x1c + 0x2c);
            (**ppc_var3)(offset, u_var5, (u_var5 >> 0x10));
            local_26 = b_result;
            if (b_result != 0) {
                local_28 = GetStockObject16(4);
                GetClientRect16(CONCAT22(unaff_ss, &local_30), local_bx_4.h_window);
                local_3c = 0;
                local_3a = 0;
                local_38 = (local_2c - local_30) - 1;
                local_36 = (local_2a - local_2e) - 1;
                h_dialog = local_36;
                local_32 = local_38;
                FillRect16(local_28, &local_3c, unaff_ss);
                DrawIcon16(
                    local_26,
                    CONCAT610(
                        u_stack74,
                        CONCAT28(local_4c, CONCAT26(local_4e, CONCAT24(b_result_2, 0x20002))),
                    ),
                    CONCAT214(
                        local_36,
                        CONCAT212(local_38, CONCAT210(local_3a, CONCAT28(local_3c, u_stack68))),
                    ),
                    h_dialog,
                );
            }
        }
    }
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn win_fn_1020_5e76(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct663,
    param_2: u16,
    param_3: u16,
) {
    let pu_var1: Vec<u8>;
    let ppc_var2: fn();
    let u_var3: u8;
    let pu_var4: *mut u32;
    let extraout_var;
    let pu_var5: *mut u16;

    let struct_a: Struct199;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let pu_var10: Vec<u8>;
    let mut u_var11: u16;
    let unaff_ss: String;
    let ppVar12: *mut pass1_struct_1;
    let u_var13: u8;
    let mut local_2aa: u16;
    let mut local_2a8: u16;
    let mut uStack676: u16;
    let mut local_1aa: u16;
    let mut local_aa: [u8; 128];
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8; 16];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    ReleaseCapture16(local_2aa);
    u_var9 = (in_struct_1 >> 0x10);
    i_var8 = in_struct_1;
    local_2a8 = (i_var8 + 0xee);
    local_2aa = offset;
    SetCursor16(local_2a8);
    (i_var8 + 0xee) = 0;
    (i_var8 + 0xf4) = 1;
    local_6 = param_3;
    local_4 = param_2;
    local_2a8 = &local_6;
    local_2aa = u_var9;
    rect::pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_ss, local_2a8));
    local_a = CONCAT22(ctx.dx_reg, local_2a8);
    struct_a = (ctx.dx_reg | local_2a8);
    if (struct_a == 0x0) {}
    // goto LAB_1020_6176;
    local_c = (local_2a8 + 0xc);
    local_2aa = (local_2a8 + 0xe);
    local_10 = 0;
    u_var11 = 0x1018;
    local_2a8 = local_c;
    local_e = local_2aa;
    pu_var4 = pass1_1018_2580(
        (i_var8 + 0xfa),
        0,
        CONCAT22(local_c, local_2aa),
        (i_var8 + 0x10c),
    );
    if (pu_var4 == 0x6b2) {}
    // goto LAB_1020_6176;
    local_12 = pu_var4;
    if (pu_var4 == 0x6b8) {
        local_2a8 = 0x1018;
        local_2aa = &ctx.PTR_LOOP_1050_5f06;
        process_struct_1000_179c(0xb4, struct_a);
        local_2a = CONCAT22(struct_a, pu_var4);
        u_var6 = struct_a | pu_var4;
        if (u_var6 == 0) {
            pu_var4 = 0x0;
            u_var6 = 0;
        } else {
            local_2a8 = 0x6b8;
            local_2aa = 2;
            mixed_1040_8520(
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, pu_var4)),
                ctx.g_h_window,
                0x40,
                2,
                0x6b8,
                0x6ad,
            );
        }
        local_26 = CONCAT22(u_var6, pu_var4);
        u_var11 = 0xa5;
        // LAB_1020_5f84:
        local_2a8 = 1;
        pass1_1008_941a(CONCAT22(unaff_ss, local_22), 1, u_var11);
        pu_var10 = (local_26 >> 0x10);
        local_2aa = local_26;
        local_2a8 = local_22;
    } else {
        if (pu_var4 == 0x6b4) {
            local_2a8 = 0x1018;
            local_2aa = 0x5f4d;
            process_struct_1000_179c(0xb4, struct_a);
            local_2a = CONCAT22(struct_a, pu_var4);
            u_var6 = struct_a | pu_var4;
            if (u_var6 == 0) {
                pu_var4 = 0x0;
                u_var6 = 0;
            } else {
                local_2aa = 2;
                local_2a8 = 0x57b;
                mixed_1040_8520(
                    CONCAT13((struct_a >> 8), CONCAT12(struct_a, pu_var4)),
                    ctx.g_h_window,
                    0x40,
                    2,
                    0x57b,
                    local_12,
                );
            }
            local_26 = CONCAT22(u_var6, pu_var4);
            u_var11 = 0xab;
            // goto LAB_1020_5f84;
        }
        if (pu_var4 == 0x6b6) {
            local_2aa = local_aa;
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_ss, local_2aa),
                0x57b,
            );
            local_2aa = &local_1aa;
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_ss, local_2aa),
                0x6b6,
            );
            local_2aa = &local_1aa;
            u_var3 = string_fn_1000_3f9c(
                &local_2aa,
                unaff_ss,
                local_2aa,
                unaff_ss,
                ctx.PTR_LOOP_1050_50cc,
            );
            u_var6 = CONCAT31(extraout_var, u_var3);
            local_2a8 = 0x1000;
            u_var11 = 0x1000;
            local_2aa = 0x5fef;
            process_struct_1000_179c(0xb4, struct_a);
            local_2a = CONCAT22(struct_a, u_var6);
            u_var7 = struct_a | u_var6;
            if (u_var7 == 0) {
                pu_var4 = 0x0;
                u_var7 = 0;
            } else {
                local_2aa = &local_2aa;
                u_var11 = &ctx.PTR_LOOP_1050_1040;
                pu_var5 = process_struct_1040_8478(
                    CONCAT22(struct_a, u_var6),
                    0x40,
                    CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_aa)),
                    CONCAT22(unaff_ss, local_2aa),
                    ctx.g_h_window,
                );
                pu_var4 = pu_var5;
            }
            local_26 = CONCAT22(u_var7, pu_var4);
            // LAB_1020_6027:
            let pu_var4_val = unsafe { *pu_var4 };
            ppc_var2 = (pu_var4_val + 0x74);
            local_2aa = u_var11;
            local_2a8 = pu_var4;
            ppc_var2();
            // goto LAB_1020_6176;
        }
        if (pu_var4 < 0x6a7) {
            if (((i_var8 + 0x10c) == 0x78) || ((i_var8 + 0x10c) == 0x74)) {
                local_2aa = ctx._g_astruct_372_1050_0ed0;
                local_2a8 = (ctx._g_astruct_372_1050_0ed0 >> 0x10);
                u_var11 = 0x1010;
                ppVar12 =
                    process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(uStack676, 5));
                if ((ppVar12 + 10) == 0) {
                    return;
                }
            }
            if (((((i_var8 + 0x10c) == 0x6c) || ((i_var8 + 0x10c) == 0x6d))
                || ((i_var8 + 0x10c) == 0x31))
                || ((i_var8 + 0x10c) == 0x32))
            {
                local_2aa = ctx._g_astruct_372_1050_0ed0;
                local_2a8 = (ctx._g_astruct_372_1050_0ed0 >> 0x10);
                u_var11 = 0x1010;
                ppVar12 = process_struct_1010_20ba(
                    ctx._g_astruct_372_1050_0ed0,
                    CONCAT22(uStack676, 0x3a),
                );
                if ((ppVar12 + 10) == 0) {
                    return;
                }
            }
            pu_var1 = *(i_var8 + 0xfe);
            local_2a8 = pu_var1;
            local_2aa = u_var11;
            rect::call_invalidate_rect_1020_68de(pu_var1);
            // goto LAB_1020_6176;
        }
        if (0x6b1 < pu_var4) {
            local_2a8 = 0x1018;
            u_var11 = 0x1000;
            local_2aa = 0x60cf;
            process_struct_1000_179c(0xb4, struct_a);
            local_2a = CONCAT22(struct_a, pu_var4);
            u_var7 = struct_a | pu_var4;
            if (u_var7 == 0) {
                pu_var4 = 0x0;
                u_var7 = 0;
            } else {
                local_2aa = 2;
                local_2a8 = 0x57b;
                u_var11 = &ctx.PTR_LOOP_1050_1040;
                mixed_1040_8520(
                    CONCAT13((struct_a >> 8), CONCAT12(struct_a, pu_var4)),
                    ctx.g_h_window,
                    0x40,
                    2,
                    0x57b,
                    local_12,
                );
            }
            // goto LAB_1020_6027;
        }
        local_2a8 = 0x1018;
        local_2aa = 0x6116;
        process_struct_1000_179c(0xb4, struct_a);
        local_2a = CONCAT22(struct_a, pu_var4);
        if ((struct_a | pu_var4) != 0) {
            local_2aa = 2;
            local_2a8 = 0x57b;
            mixed_1040_8520(
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, pu_var4)),
                ctx.g_h_window,
                0x40,
                2,
                0x57b,
                local_12,
            );
        }
        local_1aa = local_12 - 0x608;
        local_2a8 = 1;
        local_2aa = unaff_ss;
        pass1_1008_941a(CONCAT22(unaff_ss, local_aa), 1, local_1aa);
        pu_var10 = local_aa;
        local_2a8 = pu_var10;
    }
    u_var13 = local_2aa;
    ppc_var2 = (local_2aa + 0x6c);
    local_2aa = pu_var10;
    ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var13);
    // LAB_1020_6176:
    (i_var8 + 0x10c) = 0;
    return;
}

pub unsafe fn win_fn_1020_58ce(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct663,
    param_2: u16,
    param_3: u16,
    param_4: u8,
) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let rect: *mut RECT16;
    let pu_var3: *mut u16;
    let b_var4: bool;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;




    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut w_param: WPARAM16;
    let mut in_stack_0000ffcc: u16;
    let mut stack0xffcc: u16;
    let mut local_2e: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_3;
    local_4 = param_2;
    local_8 = param_4 & 8;
    rect = (param_4 & 4);
    u_var9 = (in_struct_1 >> 0x10);
    i_var7 = in_struct_1;
    local_a = rect;
    pass1_1020_64d4(*(i_var7 + 0xf6), 2);
    if (rect == 0x0) {
        // LAB_1020_5942:
        pass1_1020_64d4(*(i_var7 + 0xf6), 4);
        if (rect == 0x0) {
            // LAB_1020_5a16:
            pass1_1020_64d4(*(i_var7 + 0xf6), 1);
            u_var6 = ZEXT24(rect);
            if (rect != 0x0) {
                pass1_1020_6498(*(i_var7 + 0xf6), 1);
                local_1e = u_var6 & 0xffff | ctx.dx_reg << 0x10;
                local_1a = 0;
                loop {
                    rect = u_var6;
                    if (3 < local_1a) {
                        break;
                    }
                    b_var4 = PtInRect16(CONCAT22(local_4, local_6), (local_1a * 8 + local_1e));
                    u_var6 = b_var4;
                    if (b_var4 != 0) {
                        local_18 = 0;
                        local_14 = 0;
                        if (local_1a == 0) {
                            local_14 = (-(local_a == 0) & 4) - 5;
                        } else {
                            if (local_1a == 1) {
                                local_14 = (-(local_a == 0) & 0xfffc) + 5;
                            } else {
                                if (local_1a == 2) {
                                    local_18 = (-(local_a == 0) & 4) - 5;
                                } else {
                                    if (local_1a == 3) {
                                        local_18 = (-(local_a == 0) & 0xfffc) + 5;
                                    }
                                }
                            }
                        }
                        pass1_1020_2a94((i_var7 + 0xce), CONCAT22(local_18, local_14));
                        return;
                    }
                    local_1a = local_1a + 1;
                }
            }
            pass1_1020_64d4(*(i_var7 + 0xf6), 3);
            if (rect != 0x0) {
                local_1e = &local_6;
                rect::pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_ss, local_1e));
                if ((ctx.dx_reg | local_1e) != 0) {
                    local_1a = (local_1e + 0x2e);
                    if (((local_8 == 0) || (local_a == 0)) && (local_a == 0)) {
                        local_18 = 1;
                    } else {
                        local_18 = 2;
                    }
                    local_14 = (local_1e + 0xc);
                    pu_var5 = (local_1e + 0xe);
                    local_12 = CONCAT22(local_10, pu_var5);
                    local_1c = ctx.dx_reg;
                    if ((local_1a == 0xb) || (local_1a == 0x37)) {
                        u_var2 = (i_var7 + 0xfa);
                        u_var10 = (u_var2 >> 0x10);
                        i_var8 = u_var2;
                        local_2e = (i_var8 + 0x20);
                        pu_var5 = ((i_var8 + 0x22) | local_2e);
                        if (pu_var5 != 0x0) {
                            zero_list_1008_3e38(CONCAT22(unaff_ss, &stack0xffcc));
                            pass1_1018_161c(
                                local_2e,
                                (local_2e >> 0x10),
                                &stack0xffcc,
                                unaff_ss,
                                local_12,
                                local_14,
                            );
                            local_26 = process_struct_1010_20ba(
                                ctx._g_astruct_372_1050_0ed0,
                                CONCAT22(in_stack_0000ffcc, 0x2f),
                            );
                            pu_var5 = &stack0xffcc;
                            pass1_1010_ecc6(
                                local_26,
                                CONCAT22(unaff_ss, pu_var5),
                                (local_2e + 0x3c),
                            );
                        }
                    }
                    pass1_1018_25d2(
                        (i_var7 + 0xfa),
                        local_18,
                        local_12 & 0xffff | local_14 << 0x10,
                    );
                    if (pu_var5 != 0x0) {
                        return;
                    }
                    b_var4 = pass1_1020_5d56(in_struct_1, CONCAT22(local_1c, local_1e));
                    if (b_var4 != 0) {
                        return;
                    }
                }
            }
            return;
        }
        pass1_1020_6498(*(i_var7 + 0xf6), 4);
        local_e = rect;
        local_c = ctx.dx_reg;
        rect = PtInRect16(CONCAT22(local_4, local_6), rect);
        if (rect == 0x0) {}
        // goto LAB_1020_5a16;
        local_12 =
            process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffcc, 2));
        if ((local_12 + 0x72) != 0) {
            (i_var7 + 0x116) = 1;
            if (in_struct_1 == 0x0) {
                i_var7 = 0;
                u_var9 = 0;
            } else {
                i_var7 = i_var7 + 0xe2;
            }
            local_1e = CONCAT22(u_var9, i_var7);
            pp_var1 = (*ctx._g_struct_ptr_1050_02a0 + 4);
            (**pp_var1)(
                0x1010,
                ctx._g_struct_ptr_1050_02a0,
                (ctx._g_struct_ptr_1050_02a0 >> 0x10),
                0x10,
                i_var7,
                u_var9,
            );
            pass1_1008_941a(CONCAT22(unaff_ss, &local_18), 1, 0x86);
            pu_var3 = &local_18;
            mci_send_command_1008_5c9e(ctx._g_struct_ptr_1050_02a0, CONCAT22(unaff_ss, pu_var3));
            if (pu_var3 != 0x0) {
                return;
            }
            w_param = 0xf6;
            local_1a = pu_var3;
            // goto LAB_1020_5936;
        }
        w_param = 0xf6;
    } else {
        pass1_1020_6498(*(i_var7 + 0xf6), 2);
        local_e = rect;
        local_c = ctx.dx_reg;
        rect = PtInRect16(CONCAT22(local_4, local_6), rect);
        if (rect == 0x0) {}
        // goto LAB_1020_5942;
        w_param = 0x68;
    }
    pu_var3 = 0x0;
    // LAB_1020_5936:
    PostMessage16(CONCAT22(pu_var3, pu_var3), w_param, 0x111, ctx.g_h_window);
    return;
}

pub unsafe fn win_fn_1020_493c(ctx: &mut AppContext, in_struct_1: &mut Struct673, param_2: u16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let in_bool_1: bool;
    let mut HVar3: HCURSOR16;
    let pu_var4: *mut u32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let in_dx: Struct199;
    let pu_var7: Vec<u8>;

    let struct_a: Struct199;
    let mut u_var8: i32;


    let mut in_bx: i32;
    let local_struct_1: Struct673;
    let local_struct_1_hi: Struct673;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let ppVar10: *mut pass1_struct_1;
    let in_bool_2: bool;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut local_356: u16;
    let mut local_354: u16;
    let mut local_24e: u16;
    let mut local_24c: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u32;
    let mut local_13c: u16;
    let mut local_13a: u16;
    let mut local_2a: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u32;

    if (param_2 == 0xe9) {
        return;
    }
    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    if (param_2 < 0xea) {
        match param_2 {
            0x69 => u_var12 = 0,
            0x6a => u_var12 = 1,
            0x6b => u_var12 = 2,
            0x6c => u_var12 = 3,
            0x6d => u_var12 = 4,
            0x77 => {
                if ((&local_struct_1.field_0x10a | local_struct_1.field_0x108) == 0) {
                    return;
                }
                u_var2 = &local_struct_1.field_0x108;
                u_var8 = ((ctx.s_VrMode_1050_42ca + 8) + (u_var2 + 0x2e) * 2);
                local_1a = local_1a & 0xffff0000 | u_var8;
                if (u_var8 == 0) {
                    return;
                }
                mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
                local_12 = CONCAT22(ctx.dx_reg, u_var8);
                WinHelp16(
                    CONCAT13(
                        (local_1a >> 0xf),
                        CONCAT12((local_1a >> 0xf), local_1a & 0xff | (local_1a >> 8) << 8),
                    ),
                    1,
                    CONCAT22(ctx.dx_reg, u_var8),
                    local_struct_1.field_0x8,
                );
                return;
            }
            0x78 => {
                ppVar10 = process_struct_1010_20ba(
                    ctx._g_astruct_372_1050_0ed0,
                    CONCAT22(local_356, 0x45),
                );
                local_24c = (ppVar10 >> 0x10);
                local_24e = ppVar10;
                enum_child_windows_1010_01be(local_24e, local_24c, local_struct_1.field_0x8);
                return;
            }
            _ => {
                return;
            }
        }
        cursor::set_cursor_1020_5764(in_struct_1, u_var12);
        return;
    }
    if (param_2 == 0x132) {
        pass1_1020_64d4(local_struct_1.field_0xf6, 3);
        if (param_2 == 0) {
            return;
        }
        u_var9 = 0xffff;
        // goto LAB_1020_4ef8;
    }
    if (param_2 < 0x133) {
        if (param_2 == 0x102) {
            u_var9 = 0x1000;
            process_struct_1000_179c(0xb4, in_dx);
            u_var8 = in_dx | param_2;
            local_22 = param_2;
            if (u_var8 == 0) {
                u_var5 = 0;
                u_var8 = 0;
            } else {
                u_var9 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                u_var5 = param_2;
                mixed_1040_8520(
                    CONCAT13((in_dx >> 8), CONCAT12(in_dx, param_2)),
                    ctx.g_h_window,
                    0x31,
                    2,
                    0x57b,
                    0x62b,
                );
            }
            local_144 = CONCAT22(u_var8, u_var5);
            pp_var1 = (local_144 + 0x74);
            (**pp_var1)(u_var9, u_var5, u_var8);
            local_140 = CONCAT22(local_140._2_2_, u_var5);
            if (u_var5 != 1) {
                return;
            }
            pass1_1028_837e(&local_13c, unaff_ss);
            // LAB_1020_4b6c:
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13c));
            return;
        }
        if (param_2 < 0x103) {
            match param_2 {
                0xf0 => {
                    gui_window_func_1020_536e(in_struct_1, (in_struct_1 >> 0x10), 0, 0, 0xffff, 1);
                    return;
                }
                _ => {
                    return;
                }
                0xf6 => {
                    if (local_struct_1.field_0x116 != 0) {
                        if (in_struct_1 == 0x0) {
                            local_356 = 0;
                            local_354 = 0;
                        } else {
                            local_356 = &local_struct_1.field_0xe2;
                            local_354 = local_struct_1_hi;
                        }
                        pass1_1010_1ea6(
                            ctx._g_struct_ptr_1050_02a0,
                            CONCAT22(local_354, local_356),
                        );
                        local_struct_1.field_0x116 = 0;
                    }
                    u_var12 = 0x12;
                }
                0xf7 => {
                    u_var2 = local_struct_1.field_0x10e;
                    window_msg_func_1010_7300(u_var2, (u_var2 >> 0x10), 0, 0, 9, 0, 0);
                    return;
                }
                0xfb => {
                    local_144 = process_struct_1010_20ba(
                        ctx._g_astruct_372_1050_0ed0,
                        CONCAT22(local_356, 3),
                    );
                    local_140 = process_struct_1010_20ba(
                        ctx._g_astruct_372_1050_0ed0,
                        CONCAT22(local_356, 0x30),
                    );
                    u_var9 = local_140;
                    pass1_1010_375e(local_140);
                    pass1_1010_c25e(local_144, CONCAT22(ctx.dx_reg, u_var9));
                    return;
                }
                0xfc => {
                    win_msg::post_msg_1020_55b0(in_struct_1);
                    return;
                }
                0x101 => {
                    local_1a = process_struct_1010_20ba(
                        ctx._g_astruct_372_1050_0ed0,
                        CONCAT22(local_356, 0x2f),
                    );
                    u_var9 = (local_1a >> 0x10);
                    local_16 = (local_1a + 0x24);
                    local_20 = (local_1a + 0x26);
                    pu_var4 = (local_20 | local_16);
                    if (pu_var4 == 0x0) {
                        u_var9 = 0x1000;
                        process_struct_1000_179c(0xb4, local_20);
                        u_var8 = local_20 | pu_var4;
                        local_22 = pu_var4;
                        if (u_var8 == 0) {
                            pu_var4 = 0x0;
                            u_var8 = 0;
                        } else {
                            u_var9 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                            mixed_1040_8520(
                                CONCAT13((local_20 >> 8), CONCAT12(local_20, pu_var4)),
                                ctx.g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x730,
                            );
                        }
                        local_1e = CONCAT22(u_var8, pu_var4);
                        // LAB_1020_4c5f:
                        let pu_var4_val = unsafe { *pu_var4 };
                        pp_var1 = (pu_var4_val + 0x74);
                        (**pp_var1)(u_var9, pu_var4, u_var8);
                        return;
                    }
                    pass1_1038_af40(ctx._g_astruct_112_a, local_struct_1.field_0x8, 0xe);
                    local_12 = process_struct_1010_20ba(
                        ctx._g_astruct_372_1050_0ed0,
                        CONCAT22(local_356, 0x43),
                    );
                    u_var9 = (local_12 >> 0x10);
                    i32_var6 = local_12;
                    local_e = (i32_var6 + 10);
                    local_a = (i32_var6 + 0xc);
                    u_var12 = (i32_var6 + 0xe);
                    local_6 = CONCAT22(local_6._2_2_, u_var12);
                    if ((i32_var6 + 0x10) != 0) {
                        return;
                    }
                    pass1_1028_84ca(
                        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_13c)),
                        local_16,
                        u_var12,
                        local_a,
                        local_e,
                    );
                    // goto LAB_1020_4b6c;
                }
            }
        } else {
            if (param_2 != 0x106) {
                if (param_2 < 0x107) {
                    if (param_2 == 0x103) {
                        local_144 = process_struct_1010_20ba(
                            ctx._g_astruct_372_1050_0ed0,
                            CONCAT22(local_356, 0x2f),
                        );
                        u_var9 = (local_144 >> 0x10);
                        local_140 = (local_144 + 0x24);
                        local_20 = (local_144 + 0x26);
                        local_22 = local_20 | local_140;
                        if (local_22 != 0x0) {
                            pass1_1038_af40(ctx._g_astruct_112_a, local_struct_1.field_0x8, 0xf);
                            local_13c = process_struct_1010_20ba(
                                ctx._g_astruct_372_1050_0ed0,
                                CONCAT22(local_356, 0x42),
                            );
                            local_2a = (local_13c + 10);
                            if (local_2a == 0) {
                                return;
                            }
                            pass1_1030_e63e(CONCAT22(unaff_ss, &local_24e), local_2a);
                            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_24e));
                            return;
                        }
                        u_var9 = 0x1000;
                        process_struct_1000_179c(0xb4, local_20);
                        u_var8 = local_20 | local_22;
                        if (u_var8 == 0) {
                            pu_var4 = 0x0;
                            u_var8 = 0;
                        } else {
                            u_var9 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                            pu_var4 = local_22;
                            mixed_1040_8520(
                                CONCAT13((local_20 >> 8), CONCAT12(local_20, local_22)),
                                ctx.g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x730,
                            );
                        }
                        local_26 = CONCAT22(u_var8, pu_var4);
                    } else {
                        if (param_2 != 0x104) {
                            return;
                        }
                        ppVar10 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x220003);
                        local_24c = (ppVar10 >> 0x10);
                        local_22 = ppVar10;
                        local_24e = local_22;
                        pass1_1010_af66(ppVar10);
                        local_144 = CONCAT22(local_142, local_22);
                        local_20 = struct_a;
                        if (local_22 != 0x0) {
                            u_var9 = 0x1000;
                            process_struct_1000_179c(0xb4, struct_a);
                            u_var8 = local_20 | local_22;
                            if (u_var8 == 0) {
                                pu_var4 = 0x0;
                                u_var8 = 0;
                            } else {
                                u_var9 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                                pu_var4 = local_22;
                                mixed_1040_8520(
                                    CONCAT13((local_20 >> 8), CONCAT12(local_20, local_22)),
                                    ctx.g_h_window,
                                    0x31,
                                    2,
                                    0x57b,
                                    0x62c,
                                );
                            }
                            local_140 = CONCAT22(u_var8, pu_var4);
                            pp_var1 = (local_140 + 0x74);
                            (**pp_var1)(u_var9, pu_var4, u_var8);
                            local_13c = CONCAT22(local_13a, pu_var4);
                            if (pu_var4 != (&ctx.PTR_LOOP_1050_0000 + 1)) {
                                return;
                            }
                            pass1_1030_e79a(0xaa, unaff_ss);
                            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_356));
                            mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1e6);
                            return;
                        }
                        u_var9 = 0x1000;
                        process_struct_1000_179c(0xb4, struct_a);
                        u_var8 = local_20 | local_22;
                        if (u_var8 == 0) {
                            pu_var4 = 0x0;
                            u_var8 = 0;
                            local_356 = pu_var4;
                            local_354 = u_var8;
                        } else {
                            u_var9 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                            pu_var4 = local_22;
                            mixed_1040_8520(
                                CONCAT13((local_20 >> 8), CONCAT12(local_20, local_22)),
                                ctx.g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x731,
                            );
                            local_356 = pu_var4;
                            local_354 = u_var8;
                        }
                    }
                    // goto LAB_1020_4c5f;
                }
                if (param_2 == 0x12f) {
                    pass1_1020_61c4(
                        local_struct_1,
                        local_struct_1_hi,
                        CONCAT22(unaff_ss, &local_144),
                        CONCAT22(unaff_ss, &local_24e),
                    );
                    i32_var6 = local_24e + 0x6a;
                } else {
                    if (param_2 != 0x130) {
                        i32_var6 = param_2 - 0x131;
                        if (i32_var6 != 0) {
                            return;
                        }
                        pass1_1020_64d4(local_struct_1.field_0xf6, 2);
                        if (i32_var6 == 0) {
                            return;
                        }
                        u_var12 = 7;
                        // goto LAB_1020_49b7;
                    }
                    pass1_1020_61c4(
                        local_struct_1,
                        local_struct_1_hi,
                        CONCAT22(unaff_ss, &local_144),
                        CONCAT22(unaff_ss, &local_24e),
                    );
                    i32_var6 = local_24e + 0x68;
                }
                local_140 = CONCAT22(local_140._2_2_, i32_var6);
                if (0x6d < i32_var6) {
                    return;
                }
                if (i32_var6 < 0x69) {
                    return;
                }
                pp_var1 = (in_struct_1 + 0x40);
                (**pp_var1)();
                return;
            }
            u_var12 = 0x13;
        }
        // LAB_1020_49b7:
        pass1_1038_af40(ctx._g_astruct_112_a, local_struct_1.field_0x8, u_var12);
        return;
    }
    if (param_2 == 0x1c8) {
        u_var2 = local_struct_1.field_0x102;
        SendMessage16(0, 0x72, 0x111, (u_var2 + 8));
        return;
    }
    if (0x1c8 < param_2) {
        if (param_2 == 0x1ca) {
            local_144 =
                process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_356, 3));
            u_var8 = local_144;
            pass1_1010_c234(local_144);
            local_140 = CONCAT22(ctx.dx_reg, u_var8);
            if ((ctx.dx_reg | u_var8) == 0) {
                return;
            }
            u_var5 = ctx.dx_reg;
            local_13c =
                process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(u_var8, 0x30));
            pass1_1010_3770(local_13c, CONCAT22(u_var5, u_var8));
            u_var12 = 3;
        } else {
            in_bool_2 = (ctx._g_bool_1050_5748 >> 0x10);
            in_bool_1 = ctx._g_bool_1050_5748;
            if (param_2 == 0x200) {
                u_var2 = &local_struct_1.field_0x108;
                u_var9 = (u_var2 >> 0x10);
                i32_var6 = u_var2;
                local_1a = (i32_var6 + 0x42);
                local_c = (i32_var6 + 0x44);
                local_1a._3_1_ = (local_1a >> 0x18);
                if (local_1a._3_1_ != '\x05') {
                    return;
                }
                ctx.PTR_LOOP_1050_5f0c =
                    pass1_1030_8344(in_bool_1, in_bool_2, local_1a & 0xffff | local_c << 0x10);
                u_var12 = 0x25;
                ctx.PTR_LOOP_1050_5f0e = local_c;
                local_e = ctx.PTR_LOOP_1050_5f0c;
            } else {
                if (param_2 == 0x201) {
                    u_var2 = &local_struct_1.field_0x108;
                    u_var9 = (u_var2 >> 0x10);
                    i32_var6 = u_var2;
                    local_1a = (i32_var6 + 0x42);
                    local_c = (i32_var6 + 0x44);
                    local_1a._3_1_ = (local_1a >> 0x18);
                    if (local_1a._3_1_ != '\x05') {
                        return;
                    }
                    ctx.PTR_LOOP_1050_5f16 =
                        pass1_1030_8344(in_bool_1, in_bool_2, local_1a & 0xffff | local_c << 0x10);
                    u_var12 = 0x26;
                    ctx.PTR_LOOP_1050_5f18 = local_c;
                    local_e = ctx.PTR_LOOP_1050_5f16;
                } else {
                    if (param_2 != 0x202) {
                        if (param_2 != 0x203) {
                            return;
                        }
                        if (local_struct_1.field_0xf4 != 1) {
                            return;
                        }
                        HVar3 = SetCursor16(local_struct_1.field_0xf2);
                        local_struct_1.field_0xee = HVar3;
                        local_struct_1.field_0xf4 = 3;
                        local_356 = local_struct_1.field_0x8;
                        SetCapture16(local_356);
                        return;
                    }
                    u_var2 = &local_struct_1.field_0x108;
                    u_var9 = (u_var2 >> 0x10);
                    i32_var6 = u_var2;
                    local_6 = (i32_var6 + 0x42);
                    pu_var7 = (i32_var6 + 0x44);
                    local_6._3_1_ = (local_6 >> 0x18);
                    if (local_6._3_1_ != '\x05') {
                        return;
                    }
                    ctx.PTR_LOOP_1050_5a68 = pass1_1030_8344(
                        in_bool_1,
                        in_bool_2,
                        local_6 & 0xffff | ZEXT24(pu_var7) << 0x10,
                    );
                    local_16 = CONCAT22(pu_var7, ctx.PTR_LOOP_1050_5a68);
                    u_var12 = 0x27;
                    ctx.PTR_LOOP_1050_5a6a = pu_var7;
                }
            }
        }
        // goto LAB_1020_49b7;
    }
    match param_2 {
        0x133 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_bx == 0) {
                return;
            }
            u_var11 = 0xffff;
            u_var9 = 0;
        }
        0x134 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_bx == 0) {
                return;
            }
            u_var9 = 1;
            // goto LAB_1020_4ef8;
        }
        0x135 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_bx == 0) {
                return;
            }
            u_var11 = 1;
            u_var9 = 0;
        }
        0x136 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_bx == 0) {
                return;
            }
            u_var9 = 0xfffb;
            // goto LAB_1020_4ef8;
        }
        0x137 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_bx == 0) {
                return;
            }
            u_var11 = 0xfffb;
            u_var9 = 0;
        }
        0x138 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_bx == 0) {
                return;
            }
            u_var9 = 5;
            // LAB_1020_4ef8:
            u_var11 = 0;
        }
        0x139 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_bx == 0) {
                return;
            }
            u_var11 = 5;
            u_var9 = 0;
        }
        // default:
        // goto switchD_1020_518a_caseD_13a;
        0x13c => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 2);
            if (in_bx != 0) {
                u_var12 = 0x1a;
                // goto LAB_1020_49b7;
            }
            // goto switchD_1020_518a_caseD_13a;
        }
    }
    pass1_1020_2a94(local_struct_1.field_0xce, CONCAT22(u_var9, u_var11));
    // switchD_1020_518a_caseD_13a:
    return;
}

pub unsafe fn win_fn_1020_51c6(in_struct_1: &mut Struct663, param_2: u16, param_3: u32) {
    let mut iVar1: i32;
    let local_struct_1: Struct663;
    let local_struct_1_hi: Struct663;
    let in_struct_2: Struct665;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    iVar1 = local_struct_1.field_0xf4;
    in_struct_2 = param_3;
    if (iVar1 == 2) {
        win_fn_1020_5e76(
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
            param_2,
            in_struct_2,
        );
        return;
    }
    if (iVar1 != 3) {
        iVar1 = win_fn_1020_58ce(
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
            param_2,
            in_struct_2,
            (param_3 >> 0x10),
        );
        if (iVar1 == 0) {
            fn_ptr_1 = (local_struct_1.field_0x4 + 0x5c);
            (**fn_ptr_1)();
        }
        return;
    }
    cursor::set_cursor_1020_5de8(
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        param_2,
        in_struct_2,
    );
    return;
}

pub unsafe fn win_func_1020_522e(ctx: &mut AppContext, param_1: u32, param_2: u16, param_3: u16) {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let b_var3: bool;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let ppVar6: *mut pass1_struct_1;
    let mut in_stack_0000fffc: HWND16;
    let mut u_var7: u16;
    let mut u_var8: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    iVar1 = (i_var4 + 0xf4);
    if (iVar1 == 2) {
        SetCursor16((i_var4 + 0xee));
        (i_var4 + 0xee) = 0;
        (i_var4 + 0xf4) = 1;
        (i_var4 + 0x10c) = 0;
        ReleaseCapture16(in_stack_0000fffc);
        return;
    }
    if (iVar1 == 3) {
        SetCursor16((i_var4 + 0xee));
        (i_var4 + 0xee) = 0;
        (i_var4 + 0xf4) = 1;
        (i_var4 + 0x10c) = 0;
        ReleaseCapture16(in_stack_0000fffc);
        u_var7 = 0;
        u_var8 = 0;
        ppVar6 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x47);
        pass1_1018_57e6(ppVar6, CONCAT22(u_var8, u_var7));
        return;
    }
    b_var3 = menu::track_popup_menu_1020_5bf2((param_1 & 0xffff | u_var5 << 0x10), param_2, param_3);
    if (b_var3 == 0) {
        ppc_var2 = ((i_var4 + 4) + 0x60);
        ppc_var2();
    }
    return;
}

pub unsafe fn gui_window_func_1020_536e(
    ctx: &mut AppContext,
    param_1: &mut Struct675,
    param_2: u32,
    param_3: i32,
    param_4: i32,
) {
    let pu_var1: *mut u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let in_dx: Struct199;
    let struct_a: Struct199;
    let pu_var7: *mut u32;
    let local_bx_557: Struct18;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let pp_var9: *mut pass1_struct_1;
    let in_struct_1: Struct651;
    // ppu_var10: *mut Vec<u8>;
    let mut ppu_var10: u16;
    let mut u_var11: u32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = param_4 - 1;
    u_var8 = (param_1 >> 0x10);
    local_bx_557 = param_1;
    if (u_var6 == 0) {
        if (&local_bx_557.field_0xfe == 0) {
            process_struct_1000_179c(0xfc, in_dx);
            if ((in_dx | u_var6) == 0) {
                &local_bx_557.field_0xfe = 0;
            } else {
                pu_var1 = &local_bx_557.field_0xcc;
                *pu_var1 = *pu_var1 + 1;
                u_var11 = cursor::load_cursor_1020_67ce(
                    u_var6,
                    in_dx,
                    local_bx_557.field_0xcc,
                    local_bx_557,
                    u_var8,
                );
                &local_bx_557.field_0xfe = u_var11;
                local_bx_557.field_0x100 = (u_var11 >> 0x10);
            }
            pass1_1008_6978(param_1, 0, &local_bx_557.field_0xfe);
            u_var11 = &local_bx_557.field_0xfe;
            u_var12 = u_var11;
            u_var13 = (u_var11 >> 0x10);
            u_var11 = &local_bx_557.field_0xfe;
            u_var8 = (u_var11 >> 0x10);
            pu_var7 = u_var11;
            // goto LAB_1020_53f3;
        }
    } else {
        if (param_4 == 2) {
            u_var4 = param_3 + 0xc;
            pp_var9 =
                process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(unaff_si, u_var4));
            u_var11 = pass1_1018_0afa(pp_var9);
            struct_a = (u_var11 >> 0x10);
            if (u_var11 == 0) {
                pu_var1 = &local_bx_557.field_0xcc;
                unsafe { *pu_var1 = *pu_var1 + 1 };
                u_var14 = local_bx_557.field_0xcc;
                u_var12 = param_2;
                u_var13 = (param_2 >> 0x10);
                u_var5 = u_var14;
                process_struct_1000_179c(0xfe, struct_a);
                if ((struct_a | u_var5) == 0) {
                    in_struct_1 = 0x0;
                } else {
                    in_struct_1 =
                        pass1_1020_289a(u_var5, struct_a, u_var14, param_1, (param_1 >> 0x10));
                }
                pass1_1020_294a(in_struct_1, CONCAT22(u_var13, u_var12), u_var4);
                u_var11 = in_struct_1;
                ppc_var2 = (u_var11 + 8);
                ppc_var2(0x1000, in_struct_1);
                pass1_1008_3e0e(in_struct_1);
                pass1_1008_6978(param_1, u_var14, in_struct_1);
                ppc_var2 = (u_var11 + 0xc);
                ppc_var2(8, in_struct_1, (in_struct_1 >> 0x10), 1);
            } else {
                ppu_var10 = pass1_1018_0ad4(pp_var9);
                pass1_1008_3e0e(ppu_var10);
            }
            pass1_1018_1662(pp_var9, (pp_var9 >> 0x10), 0, 0);
            u_var3 = local_bx_557.field_0xce;
            BringWindowToTop16((u_var3 + 8));
            u_var4 = 1;
            u_var14 = 4;
            pp_var9 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x1002b);
            pass1_1010_089e(pp_var9, u_var4, u_var14);
            pass1_1010_089e(pp_var9, 1, 3);
            return;
        }
        u_var6 = param_4 - 3;
        if ((u_var6 == 0) && (&local_bx_557.field_0x102 == 0)) {
            process_struct_1000_179c(0xfc, in_dx);
            if ((in_dx | u_var6) == 0) {
                &local_bx_557.field_0x102 = 0;
            } else {
                pu_var1 = &local_bx_557.field_0xcc;
                unsafe { *pu_var1 = *pu_var1 + 1 };
                u_var11 = win_fn_1020_0dc4(
                    CONCAT13((in_dx >> 8), CONCAT12(in_dx, u_var6)),
                    local_bx_557.field_0xcc,
                    param_1,
                );
                local_bx_557.field_0x102 = u_var11;
                &local_bx_557.field_0x104 = (u_var11 >> 0x10);
            }
            pass1_1008_6978(param_1, 0, &local_bx_557.field_0x102);
            u_var11 = &local_bx_557.field_0x102;
            u_var12 = u_var11;
            u_var13 = (u_var11 >> 0x10);
            u_var11 = &local_bx_557.field_0x102;
            u_var8 = (u_var11 >> 0x10);
            pu_var7 = u_var11;
            // LAB_1020_53f3:
            unsafe { ppc_var2 = (*pu_var7 + 0xc) };
            ppc_var2(8, u_var12, u_var13, 5);
            return;
        }
    }
    return;
}

pub unsafe fn set_window_pos_1020_38aa(ctx: &mut AppContext, param_1: &mut win_struct_42) {
    let mut hwnd: HWND16;
    let pp_var1: fn();
    let u_var2: u8;
    let mut u_var3: i32;
    let piVar4: *mut u16;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let mut extraout_var;
    let struct_a: Struct199;
    let struct_a_00: Struct199;

    let pa_var7: Struct199;
    let ctx.dx_reg: Struct199;


    let mut unaff_si: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: HWND16;
    let ppVar10: *mut pass1_struct_1;
    let mut u_var11: i32;
    let mut u_var12: i32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var11 = param_1;
    u_var12 = (param_1 >> 0x10);
    create_win_1008_9760(param_1);
    ppVar10 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(unaff_si, 6));
    u_var9 = (ppVar10 >> 0x10);
    (u_var11 + 0xfa) = ppVar10;
    (u_var11 + 0xfc) = u_var9;
    (u_var11 + 0xe0) = (u_var11 + 0xfa);
    (u_var11 + 0xe2) = u_var9;
    if ((u_var12 | u_var11) == 0) {
        u_var3 = 0;
        u_var5 = 0;
    } else {
        u_var3 = u_var11 + 0xf2;
        u_var5 = u_var12;
    }
    u_var6 = (u_var11 + 0xfa);
    u_var9 = u_var6;
    pp_var1 = ((u_var11 + 0xfa) + 4);
    (**pp_var1)(0x1010, u_var9, (u_var6 >> 0x10), 0, u_var3, u_var5);
    pass1_1018_1a8e((u_var11 + 0xfa));
    pa_var7 = struct_a;
    process_struct_1000_179c(0x20, struct_a);
    struct_a_00 = (pa_var7 | u_var3);
    if (struct_a_00 == 0x0) {
        (u_var11 + 0xf6) = 0;
    } else {
        draw::misc::draw_1020_3da4(u_var3, pa_var7, param_1, u_var12);
        (u_var11 + 0xf6) = u_var3;
        (u_var11 + 0xf8) = ctx.dx_reg;
        struct_a_00 = ctx.dx_reg;
    }
    u_var6 = (u_var11 + 0xf6);
    (u_var11 + 0xe8) = u_var6;
    process_struct_1000_179c(0x42, struct_a_00);
    piVar4 = u_var6;
    pa_var7 = (struct_a_00 | piVar4);
    if (pa_var7 == 0x0) {
        (u_var11 + 0x102) = 0;
    } else {
        win_fn_1008_3bd6(
            piVar4,
            struct_a_00,
            0,
            0xf1,
            0x1ad0000,
            0xf401ac,
            CONCAT22(u_var9, (u_var11 + 8)),
        );
        (u_var11 + 0x102) = piVar4;
        (u_var11 + 0x104) = ctx.dx_reg;
        pa_var7 = ctx.dx_reg;
    }
    u_var8 = 0x1000;
    process_struct_1000_179c(0x42, pa_var7);
    if ((pa_var7 | piVar4) == 0) {
        (u_var11 + 0x106) = 0;
    } else {
        u_var8 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        win_fn_1008_3bd6(
            piVar4,
            pa_var7,
            0,
            0xf500f1,
            0x1af0000,
            0xf501ae,
            CONCAT22(u_var9, (u_var11 + 8)),
        );
        (u_var11 + 0x106) = piVar4;
        (u_var11 + 0x108) = ctx.dx_reg;
    }
    u_var6 = (u_var11 + 0xfa);
    pp_var1 = ((u_var11 + 0xfa) + 0x10);
    (**pp_var1)(u_var8, u_var6, (u_var6 >> 0x10));
    pa_var7 = piVar4[1];
    let pi_var4_val = unsafe { *piVar4 };
    u_var5 = MoveWindow16(1, piVar4[3], piVar4[2], pa_var7, pi_var4_val, (u_var11 + 8));
    u_var9 = 0x1000;
    process_struct_1000_179c(0x8e, pa_var7);
    u_var3 = pa_var7 | u_var5;
    if (u_var3 == 0) {
        (u_var11 + 0x10a) = 0;
    } else {
        u_var9 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
        u_var2 = process_struct_1040_7728(
            CONCAT22(pa_var7, u_var5),
            (&ctx.PTR_LOOP_1050_0000 + 1),
            0,
            0xfc0,
            *(u_var11 + 8),
        );
        (u_var11 + 0x10a) = CONCAT31(extraout_var, u_var2);
        (u_var11 + 0x10c) = u_var3;
    }
    u_var6 = (u_var11 + 0x10a);
    (u_var6 + 0x74) = 1;
    u_var6 = (u_var11 + 0x10a);
    pp_var1 = ((u_var11 + 0x10a) + 8);
    (**pp_var1)(u_var9, u_var6, (u_var6 >> 0x10));
    if (((u_var11 + 0x10c) | (u_var11 + 0x10a)) != 0) {
        u_var6 = (u_var11 + 0x10a);
        hwnd = (u_var6 + 6);
        GetWindowRect16(CONCAT22(&local_a, u_var9), unaff_ss);
        GetWindowRect16(CONCAT22(&local_12, 0x1538), unaff_ss);
        local_c = local_c - local_10;
        local_e = local_6 - local_a;
        local_12 = local_a;
        local_10 = local_4 + 3;
        SetWindowPos16(0x44, local_c, local_e, local_10, local_a, 0, hwnd);
    }
    return;
}

pub unsafe fn destroy_win_1020_3b3e(in_struct_1: &mut Struct656) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;
    let local_struct_1: Struct656;
    let mut local_es_4: u16;
    let mut local_CS__1: u16;
    let mut temp_5f994c26c9: u32;
    let temp_8626ab7427f: *mut u32;
    let mut offset: u16;

    local_es_4 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x10e = 0;
    if (local_struct_1.field_0x10a != 0) {
        temp_5f994c26c9 = local_struct_1.field_0x10a;
        local_CS__1 = offset;
        DestroyWindow16((temp_5f994c26c9 + 6));
    }
    pu_var1 = local_struct_1.field_0xf6;
    u_var2 = &local_struct_1.field_0xf8;
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)(local_CS__1, pu_var1);
    }
    &local_struct_1.field_0xf6 = 0;
    if (local_struct_1.field_0xfa != 0x0) {
        if (in_struct_1 == 0x0) {
            i_var4 = 0;
            u_var5 = 0;
        } else {
            i_var4 = &local_struct_1.field_0xf2;
            u_var5 = local_es_4;
        }
        pass1_1010_1ea6(local_struct_1.field_0xfa, CONCAT22(u_var5, i_var4));
    }
    local_struct_1.field_0xfa = 0x0;
    return;
}

pub unsafe fn win_gui_fn_1020_2488(in_struct_1: &mut Struct647) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let local_struct_1: Struct647;
    let local_struct_1_hi: Struct647;
    let mut u_var3: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_5f808f12c5: Struct60;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    u_var3 = load_rsrc_1010_4e9e(local_struct_1.struct_ptr_0x6);
    local_a = u_var3;
    if (u_var3 != 0) {
        local_c = 0;
        while (local_c < 9) {
            temp_5f808f12c5 = local_struct_1.struct_ptr_0x6;
            u_var1 = pass1_1010_4f20(temp_5f808f12c5, (temp_5f808f12c5 >> 0x10), local_c);
            set_window_text_1020_1d2a(local_struct_1.field_0xa, local_a, (u_var3 >> 0x10), u_var1);
            u_var2 = get_string_index_1000_3da4((u_var3 & 0xffff0000 | local_a));
            local_a = local_a + u_var2 + 1;
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn win_fn_1020_157c(param_1: u32, param_2: i32) {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: HWND16;
    let mut i_stack8: u16;
    let mut local_4: u16;
    let mut stack0xfff6: u16;

    i_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (i_var2 + 0x14) = 0;
        return;
    }
    if (param_2 == 2) {
        b_var1 = IsIconic16((i_var2 + 4));
        if (b_var1 == 0) {
            GetClientRect16(CONCAT22(unaff_ss, &stack0xfff6), (i_var2 + 4));
            local_4 = 0x9a;
            i_stack8 = (i_var2 + 4);
            InvalidateRect16(0, &stack0xfff6, unaff_ss);
            return;
        }
    }
    return;
}

pub fn call_destroy_win_fn_1020_1780(param_1: &mut Struct53) {
    let fn_ptr_1: fn();

    fn_ptr_1 = (param_1 + 0x6c);
    (**fn_ptr_1)();
    destroy_window_1040_8212(param_1);
    return;
}

pub unsafe fn win_fn_1020_179c(ctx: &mut AppContext, param_1: &mut Struct20) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut i_var5: u16;
    let puVar6: *mut u16;
    let BVar7: bool;
    let mut hwnd: HWND16;
    let struct_a: Struct199;


    let mut i_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_ss: HWND16;
    let ppVar13: *mut pass1_struct_1;
    let pu_var14: Vec<u8>;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let uVar18: u8;
    let mut uVar19: u16;
    let mut in_stack_0000fe88: u16;
    let mut local_76: u32;
    let mut local_72: u32;
    let mut local_6e: u32;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_62: u16;
    let mut local_5e: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut stack0xfe88: u16;
    let mut offset: u16;

    win_gui_func_1040_78e2(param_1);
    uVar19 = 0x89;
    local_6 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x890009);
    i_var8 = local_6;
    pass1_1010_65d0(local_6, uVar19);
    u_var4 = (i_var8 == 0);
    local_8 = u_var4;
    pass1_1010_65d0(local_6, 0x86);
    if (u_var4 != 0) {
        local_8 = 0;
    }
    ppVar13 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000fe88, 0x39),
    );
    u_var12 = (ppVar13 >> 0x10);
    u_var4 = ppVar13;
    u_var10 = (param_1 >> 0x10);
    i_var8 = param_1;
    (i_var8 + 0x8e) = u_var4;
    (i_var8 + 0x90) = u_var12;
    u_var11 = (i_var8 + 0x8e);
    ppc_var2 = ((i_var8 + 0x8e) + 0x10);
    ppc_var2(0x1010, u_var11, u_var12, local_8);
    local_4c = struct_a;
    process_struct_1000_179c(0x12, struct_a);
    local_4e = u_var4;
    if ((local_4c | u_var4) == 0) {
        (i_var8 + 0x92) = 0;
    } else {
        struct_ops::process_struct_1020_1eea(CONCAT22(local_4c, u_var4), param_1, (i_var8 + 6));
        (i_var8 + 0x92) = u_var4;
        (i_var8 + 0x94) = ctx.dx_reg;
    }
    u_var1 = (i_var8 + 0x8e);
    local_c = u_var1 & 0xffff0000 | (u_var1 + 10);
    local_10 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(u_var11, 0x48));
    uVar15 = unaff_ss;
    uVar17 = (unaff_ss >> 8);
    GetClientRect16(CONCAT13(uVar17, CONCAT12(uVar15, &local_1c)), (i_var8 + 6));
    i_var5 = GetSystemMetrics16(4);
    u_var11 = (local_c >> 0x10);
    i_var9 = local_c;
    (i_var9 + 6) = i_var5 + local_18._2_2_;
    u_var12 = (local_10 >> 0x10);
    local_12 = (local_10 + 10);
    local_14 = (local_10 + 0xc);
    (i_var9 + 2) = (local_14 - (i_var9 + 6)) / 2;
    local_c = (local_12 - (i_var9 + 4)) / 2;
    pass1_1028_dc52(
        CONCAT13(uVar17, CONCAT12(uVar15, &local_2e)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x100,
    );
    local_38 = 0;
    while (true) {
        puVar6 = &local_2e;
        pass1_1028_e4ec(CONCAT22(unaff_ss, puVar6));
        local_32 = CONCAT22(ctx.dx_reg, puVar6);
        if ((ctx.dx_reg | puVar6) == 0) {
            break;
        }
        local_36 = (puVar6 + 8);
        if (local_36 != 0x0) {
            process_string_1000_3cea((param_1 & 0xffff0000 | (i_var8 + 0x10)), *local_36);
        }
    }
    BVar7 = draw::misc::pass1_1020_1da8(param_1);
    (i_var8 + 0x96) = BVar7;
    pass1_1010_65d0(local_6, 0x86);
    if ((BVar7 == 0) || ((i_var8 + 0x96) != 0)) {
        u_var3 = (i_var8 + 0x8e);
        (u_var3 + 0x2c) = 0;
        local_66 = GetDlgItem16(0x175, (i_var8 + 6));
        if (local_8 != 0) {
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x100,
                CONCAT22(unaff_ss, &stack0xfe88),
                0x72d,
            );
            SetWindowText16(CONCAT22(unaff_ss, &stack0xfe88), local_66);
        }
        local_3c = MakeProcInstance16(offset, CONCAT22(0x1e1e, ctx.g_h_instance_1050_038c));
        GetWindowRect16(
            CONCAT13((&local_6e >> 8), CONCAT12(&local_6e, 0x1538)),
            unaff_ss,
        );
        local_72 = local_6a;
        local_62 = local_6a - local_6e;
        local_5e = local_6a._2_2_ - local_6e._2_2_;
        local_76 = local_6e & 0xffff0000 | (-(local_62 - (local_c + 4)) / 2);
        i_var5 = GetSystemMetrics16(4);
        u_var1 = local_76 & 0xffff;
        local_76 = u_var1 | (local_76._2_2_ - i_var5) << 0x10;
        local_76._0_2_ = u_var1;
        MoveWindow16(
            0,
            local_5e,
            local_62,
            local_76._2_2_ - i_var5,
            local_76,
            local_66,
        );
    } else {
        mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x9d0001);
        (i_var8 + 0x8c) = BVar7;
        local_3c = MakeProcInstance16(
            &ctx.PTR_LOOP_1050_1008,
            CONCAT22(0x1dea, ctx.g_h_instance_1050_038c),
        );
    }
    EnumChildWindows16(0, local_3c, (local_3c >> 0x10));
    FreeProcInstance16(CONCAT13((local_3c >> 8), CONCAT12(local_3c, 0x1538)));
    local_46 = GetDlgItem16(1, (i_var8 + 6));
    GetWindowRect16(CONCAT22(&local_1c, 0x1538), unaff_ss);
    local_40 = local_18;
    i_var9 = local_18 - local_1c;
    local_4a = CONCAT22(i_var9, local_18._2_2_ - local_1c._2_2_);
    local_44 = local_1c & 0xffff0000 | (-(i_var9 - (local_c + 4)) / 2);
    i_var5 = GetSystemMetrics16(4);
    local_44 = local_44 & 0xffff | (local_44._2_2_ - i_var5) << 0x10;
    if ((i_var8 + 0x96) == 0) {
        if (local_8 == 0) {}
        // goto LAB_1020_1b24;
        unaff_ss = 0x72e;
        pu_var14 = &stack0xfe88;
        i_var5 = 0x100;
        uVar16 = uVar15;
        uVar18 = uVar17;
    } else {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT13(uVar17, CONCAT12(uVar15, &stack0xfe88)),
            0x72f,
        );
        hwnd = GetDlgItem16(0x175, (i_var8 + 6));
        i_var5 = offset;
        SetWindowText16(CONCAT13(uVar17, CONCAT12(uVar15, &stack0xfe88)), hwnd);
        pu_var14 = 0x3ff;
        uVar16 = &stack0xfe88;
        uVar18 = (&stack0xfe88 >> 8);
    }
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        i_var5,
        CONCAT13(uVar18, CONCAT12(uVar16, pu_var14)),
        unaff_ss,
    );
    SetWindowText16(CONCAT13(uVar17, CONCAT12(uVar15, &stack0xfe88)), local_46);
    // LAB_1020_1b24:
    MoveWindow16(
        0,
        local_4a,
        (local_4a >> 0x10),
        local_44._2_2_,
        local_44,
        local_46,
    );
    u_var11 = (local_c >> 0x10);
    i_var9 = local_c;
    SetWindowPos16(
        0x44,
        (i_var9 + 6),
        (i_var9 + 4),
        (i_var9 + 2),
        local_c,
        0,
        (i_var8 + 6),
    );
    return;
}

pub unsafe fn enable_window_1020_1bd4(
    ctx: &mut AppContext,
    param_1: i32,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut b_var3: bool;
    let mut h_wnd: HWND16;
    let b_var4: bool;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let mut u_var7: u16;
    let puVar8: *mut u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    b_var3 = false;
    puVar8 = rect::call_pt_in_rect_fn_1020_1d8e(
        CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
        CONCAT22(param_3, param_4),
    );
    if (puVar8 != 0) {
        if (puVar8 < 2) {
            b_var3 = true;
        } else {
            h_wnd = GetDlgItem16(1, (param_1 + 6));
            u_var1 = (param_1 + 0x8e);
            pass1_1010_4e8c(u_var1, (u_var1 >> 0x10));
            EnableWindow16(1, h_wnd);
            puVar8 = pass1_1010_4df0((param_1 + 0x8e));
            u_var7 = (puVar8 >> 0x10);
            if ((puVar8 == 0) && (b_var3 = true, (param_1 + 0x96) == 0)) {
                b_var4 = EnableWindow16(0, h_wnd);
                puVar8 = CONCAT22(u_var7, b_var4);
            }
        }
    }
    if (b_var3) {
        u_var7 = 0x1000;
        process_struct_1000_179c(0xb4, (puVar8 >> 0x10));
        u_var6 = (puVar8 >> 0x10) | puVar8;
        if (puVar8 == 0x0) {
            u_var5 = 0;
            u_var6 = 0;
        } else {
            u_var7 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            u_var5 = mixed_1040_8520(puVar8, (param_1 + 6), 0x30, 2, 0x57b, 0x62a);
        }
        local_c = CONCAT22(u_var6, u_var5);
        ppc_var2 = (*local_c + 0x74);
        ppc_var2(u_var7, u_var5, u_var6);
    }
    return;
}

pub fn set_window_text_1020_1d2a(param_1: u32, param_2: SEGPTR, param_3: u16) {
    let mut hwnd: HWND16;
    hwnd = GetDlgItem16(param_3, (param_1 + 6));
    SetWindowText16(param_2, hwnd);
    return;
}

pub fn win_func_1020_1d4a(ctx: &mut AppContext, param_1: u32, param_2: i32) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    if (param_2 != 0) {
        i_var2 = param_1;
        u_var3 = (param_1 >> 0x10);
        iVar1 = post_win_msg_1020_1ca4(i_var2, u_var3);
        if (iVar1 != 0) {
            if ((i_var2 + 0x96) != 0) {
                PostMessage16(0, 0xee, 0x111, ctx.g_h_window);
            }
            DestroyWindow16((i_var2 + 6));
        }
    }
    return;
}

pub fn destroy_win_1020_1dea(param_1: u16, param_2: u16, param_3: HWND16) -> u16 {
    let b_var1: bool;
    let wVar2: u16;

    b_var1 = IsWindow16(param_1);
    if (b_var1 != 0) {
        wVar2 = GetWindowWord16(-0xc, param_1);
        if (wVar2 == 0x175) {
            DestroyWindow16(param_1);
            return 0;
        }
    }
    return 1;
}

pub fn destroy_win_1020_1e1e(param_1: u16, param_2: u16, 2: HWND16) -> u16 {
    let b_var1: bool;
    let wVar2: u16;
    let mut local_4: u16;

    b_var1 = IsWindow16(param_1);
    if (b_var1 != 0) {
        wVar2 = GetWindowWord16(-0xc, param_1);
        if ((wVar2 != 1) && (wVar2 != 0x175)) {
            DestroyWindow16(param_1);
        }
    }
    return 1;
}

pub unsafe fn win_fn_1020_7270(ctx: &mut AppContext, param_1: &mut Struct7) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1: &mut Struct44;
    let local_struct_1_hi: &mut Struct44;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut temp_5f4bfe04bb: u32;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x754c;
    local_struct_1.base_fld_2 = 0x1020;
    if (&local_struct_1.field_0x1c != 0) {
        pass1_1010_1ea6(
            *&local_struct_1.field_0x1c,
            (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        );
    }
    u_var1 = &local_struct_1.field_0x14;
    u_var2 = (&local_struct_1.field_0x14 + 2);
    local_8 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        pass1_1008_5118(CONCAT22(u_var2, u_var1));
        error_check_1000_17ce(local_8);
    }
    temp_5f4bfe04bb = &local_struct_1.field_0x1c;
    h_gdi_obj = SelectPalette16(0, local_struct_1.field_0x20, (temp_5f4bfe04bb + 0x178));
    local_struct_1.field_0x20 = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    param_1.ptr_a_lo = ctx.s_0_020_1050_3ab0;
    local_struct_1.base_fld_2 = &ctx.PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_struct_1.base_fld_2 = &ctx.PTR_LOOP_1050_1008;
    return;
}
