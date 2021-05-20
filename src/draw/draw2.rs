use crate::{draw, mixed_fn_1010_830a};
use crate::app_context::AppContext;
use crate::cleanup::{destroy_window_1040_8212, win_cleanup_func_1040_782c, window_msg_func_1010_7300};
use crate::draw::draw1;
use crate::err_funcs::error_check_1000_17ce;
use crate::file_funcs::file2::pass1_1020_8a9c;
use crate::func_ptr_funcs::call_fn_ptr_1_1020_6746;
use crate::other_funcs::zero_list_1008_3e38;
use crate::pass::pass12_funcs::pass1_1008_c6ae;
use crate::pass::pass13_funcs::{bad_func_1008_8fc4, pass1_1008_8ce4, pass1_1008_941a, pass1_1008_9436};
use crate::pass::pass14_funcs::{pass1_1008_3e0e, pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_4480, pass1_1008_4d84, pass1_1008_5118, pass1_1008_5236, pass1_1008_6978};
use crate::pass::pass15_funcs::{pass1_1020_294a, pass1_1020_2a94, pass1_1020_5d56, pass1_1020_61c4, pass1_1020_6498, pass1_1020_64d4, pass1_1020_87c2, pass1_1020_8eaa, process_struct_1020_8360};
use crate::pass::pass16_funcs::pass1_1028_84ca;
use crate::pass::pass17_funcs::{pass1_1030_69cc, pass1_1030_6fa0, pass1_1030_73a8, pass1_1030_8308, pass1_1030_8344, pass1_1030_835a};
use crate::pass::pass20_funcs::{pass1_1010_af66, pass1_1010_c234, pass1_1010_c25e, pass1_1010_c3c2, pass1_1010_ecc6, pass1_1018_017c, pass1_1018_0ad4, pass1_1018_0afa};
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e4ec};
use crate::pass::pass7_funcs::{pass1_1018_161c, pass1_1018_1662, pass1_1018_1a8e, pass1_1018_2504, pass1_1018_255e, pass1_1018_2580, pass1_1018_25d2, pass1_1018_2862, pass1_1018_2afa, pass1_1018_2d22, pass1_1018_2d84, pass1_1018_2d9a, pass1_1018_2dde, pass1_1018_2e28, pass1_1018_2e5e, pass1_1018_2fe8, pass1_1018_30ca, pass1_1018_30fc, pass1_1018_31d0, pass1_1018_57e6};
use crate::pass::pass8_funcs::{pass1_1010_089e, pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_375e, pass1_1010_3770, pass1_1010_454a, pass1_1010_4c2c, pass1_1010_4dc8, pass1_1010_4df0, pass1_1010_4e8c, pass1_1010_4f20, pass1_1010_4f48, pass1_1010_65d0, process_struct_1010_20ba};
use crate::pass::pass_funcs::pass1_1000_4906;
use crate::prog_structs::prog_structs_12::Struct409;
use crate::prog_structs::prog_structs_15::Struct26;
use crate::prog_structs::prog_structs_2::{Struct199, Struct665, Struct668};
use crate::prog_structs::prog_structs_20::Struct133;
use crate::prog_structs::prog_structs_23::{Struct134, win_struct_42};
use crate::prog_structs::prog_structs_25::Struct65;
use crate::prog_structs::prog_structs_26::Struct53;
use crate::prog_structs::prog_structs_27::pass1_struct_2;
use crate::prog_structs::prog_structs_28::Struct37;
use crate::prog_structs::prog_structs_3::{Struct661, Struct663, Struct664};
use crate::prog_structs::prog_structs_30::{Struct14, Struct18, Struct417};
use crate::prog_structs::prog_structs_31::{Struct15, Struct20, Struct34, Struct45, Struct5};
use crate::prog_structs::prog_structs_4::{Struct650, Struct651, Struct652, Struct656};
use crate::prog_structs::prog_structs_5::Struct659;
use crate::prog_structs::prog_structs_6::{Struct132, Struct622, Struct672, Struct673, Struct674, Struct675};
use crate::prog_structs::prog_structs_7::{Struct376, Struct44};
use crate::prog_structs::prog_structs_8::{Struct60, Struct641, Struct643, Struct644, Struct645, Struct646, Struct647, Struct68};
use crate::prog_structs::prog_structs_9::Struct594;
use crate::sound_funcs::{mci_send_cmd_1008_5c5c, mci_send_command_1008_5c7c, mci_send_command_1008_5c9e};
use crate::string_ops1::{copy_string_1000_3d3e, get_string_index_1000_3da4, load_string_1008_a8f4, process_string_1000_3cea, string_fn_1000_3f9c};
use crate::struct_ops1::{process_struct_1000_179c, process_struct_1008_4772, process_struct_1010_4d5c};
use crate::struct_ops2::process_struct_1040_7728;
use crate::sys1::{create_win_1008_9760, load_rsrc_1010_4e9e, reg_class_1008_96d2};
use crate::sys2::{get_sys_metrics_1020_7c1a, post_win_msg_1020_1ca4, process_struct_1040_8478};
use crate::sys_structs::{PAINTSTRUCT16, POINT16, RECT16};
use crate::typedefs::{COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HGDIOBJ16, HICON16, HMENU16, HPALETTE16, HPEN16, HWND16, LRESULT, SEGPTR, WPARAM16};
use crate::ui_funcs::ui1::{destroy_win_1008_628e, enum_child_windows_1010_01be, mixed_1040_8520, win_fn_1008_3bd6, win_fn_1020_0dc4, win_gui_func_1040_78e2};
use crate::ui_funcs::ui2::{call_load_cursor_1020_790e, destroy_menu_func_1020_795c, destroy_win_1020_42f4, pass1_1020_289a, pass1_1038_af40};
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT210, CONCAT212, CONCAT214, CONCAT22, CONCAT24, CONCAT26, CONCAT28, CONCAT31, CONCAT610, CONCAT66, SBORROW2, SUB41, SUB42, ZEXT24};
use crate::winapi_funcs::{BeginPaint16, BringWindowToTop16, CheckMenuItem16, ClientToScreen16, CreatePen16, CreateSolidBrush16, CreateWindow16, DeleteMenu16, DeleteObject16, DestroyIcon16, DestroyWindow16, DrawIcon16, Ellipse16, EnableMenuItem16, EnableWindow16, EndPaint16, EnumChildWindows16, FillRect16, FreeProcInstance16, GetClientRect16, GetDC16, GetDlgItem16, GetMenuState16, GetStockObject16, GetSubMenu16, GetSystemMetrics16, GetTextExtent16, GetWindowDC16, GetWindowRect16, GetWindowWord16, InsertMenu16, InvalidateRect16, IsIconic16, IsWindow16, LineTo16, LoadCursor16, LoadIcon16, LoadMenu16, lstrlen16, MakeProcInstance16, ModifyMenu16, MoveTo16, MoveWindow16, Polygon16, PostMessage16, PtInRect16, RealizePalette16, Rectangle16, ReleaseCapture16, ReleaseDC16, SelectObject16, SelectPalette16, SendMessage16, SetBkColor16, SetCapture16, SetCursor16, SetFocus16, SetMapMode16, SetTextColor16, SetWindowPos16, SetWindowText16, ShowWindow16, TextOut16, TrackPopupMenu16, UnrealizeObject16, UpdateWindow16, ValidateRect16, WinHelp16};

pub fn draw_1020_7cc8(ctx: &mut AppContext, param_1: Vec<u8>) {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut hdc: u16;
    let mut x: i32;
    let mut i_var4: i32;
    let mut h_brush: HGDIOBJ16;
    let mut width: u16;
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let pcVar5: String;
    let mut i32_var6: u16;
    let pu_var7: *mut u32;
    let mut y: i32;

    let local_bx_4: *mut Struct34;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut h_wnd: u16;
    let mut dVar11: u32;
    let mut h_dc: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut uStack58: u16;
    let mut uStack56: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
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
    let mut local_a: u32;
    let mut window_dc: u16;
    let mut b_result: u16;
    let mut offset: u16;

    u_var9 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    b_result = IsIconic16(local_bx_4.h_window);
    if ((b_result == 0) || (ctx.PTR_LOOP_1050_0010 != 0x0)) {
        window_dc = GetWindowDC16(local_bx_4.h_window);
        local_a = (ctx._PTR_LOOP_1050_4230 + 0xe);
        local_c = &window_dc;
        draw1::realize_palette_1008_4e08(local_a, (local_a >> 0x10), local_c, h_wnd);
        GetWindowRect16(CONCAT22(&local_14, 0x1008), h_wnd);
        hdc = window_dc;
        x = (local_10 - local_14) + -1;
        y = (local_e - local_12) + -1;
        iVar1 = local_bx_4.field_0x12;
        i_var4 = y;
        if (b_result == 0) {
            i_var4 = local_bx_4.field_0xe - local_bx_4.field_0x12;
        }
        h_brush = GetStockObject16(4);
        FillRect16(h_brush, h_wnd, hdc);
        u_var3 = local_bx_4.field_0x6;
        u_var10 = (u_var3 >> 0x10);
        i_var8 = u_var3;
        pu_var7 = (i_var8 + 0xe0);
        i32_var6 = (i_var8 + 0xe2);
        width = pu_var7;
        let pu_var7_val = unsafe { *pu_var7 };
        ppc_var2 = (pu_var7_val + 0x24);
        ppc_var2(offset, width, i32_var6, 0, 0);
        i_var8 = (-(pu_var7 == 0) & 0x1e) + 0x25;
        pen = CreatePen16(CONCAT22(0x100, i_var8), width, i32_var6);
        pen_obj_handle = SelectObject16(pen, window_dc);
        MoveTo16(0, 0, window_dc);
        LineTo16(0, x, window_dc);
        LineTo16(y, x, window_dc);
        LineTo16(y, 0, window_dc);
        pcVar5 = LineTo16(0, 0, window_dc);
        if (b_result == 0) {
            MoveTo16(local_bx_4.field_0xe - local_bx_4.field_0x12, 0, window_dc);
            pcVar5 = LineTo16(x, x, window_dc);
        }
        u_var3 = local_bx_4.field_0x6;
        uStack58 = u_var3;
        uStack56 = (u_var3 >> 0x10);
        ppc_var2 = (local_bx_4.field_0x6 + 0x18);
        ppc_var2(offset, uStack58, uStack56);
        let pc_var5_val = unsafe { *pcVar5 };
        if (pc_var5_val != '\0') {
            SetBkColor16(0, window_dc);
            SetTextColor16(CONCAT22(0x100, i_var8), window_dc);
            i32_var6 = lstrlen16(CONCAT22(ctx.dx_reg, pcVar5));
            dVar11 = GetTextExtent16(i32_var6, CONCAT22(ctx.dx_reg, pcVar5), window_dc);
            i_var8 = (dVar11 >> 0x10);
            if (b_result == 0) {
                local_42 = (i_var4 - iVar1) / 2 - i_var8 / 2;
                h_dc = x / 2 - dVar11 / 2;
            } else {
                local_42 = y / 2 - i_var8 / 2;
                h_dc = 2;
            }
            local_3e._0_1_ = ctx.dx_reg;
            local_3e._1_1_ = (ctx.dx_reg >> 8);
            TextOut16(
                local_42,
                CONCAT13(local_3e._1_1_, CONCAT12(local_3e, pcVar5)),
                local_42,
                h_dc,
                window_dc,
            );
        }
        local_c = SelectPalette16(0, local_c, window_dc);
        DeleteObject16(local_c);
        SelectObject16(pen_obj_handle, window_dc);
        DeleteObject16(pen);
        ReleaseDC16(window_dc, local_bx_4.h_window);
    }
    return;
}

pub fn get_sys_metrics_1020_7a50(param_1: *mut Struct652) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let b_var3: bool;
    let mut i_var4: u16;
    let mut i_var5: u16;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: HWND16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    b_var3 = IsIconic16((i32_var6 + 8));
    if (b_var3 == 0) {
        GetWindowRect16(CONCAT22(&local_a, 0x1538), unaff_ss);
        local_6 = local_6 - local_a;
        local_4 = local_4 - local_8;
        i_var4 = GetSystemMetrics16(5);
        i_var5 = GetSystemMetrics16(6);
        local_a = local_a + i_var4 * 2;
        local_8 = local_8 + i_var5 * 2;
    }
    u_var1 = (i32_var6 + 0xe0);
    ppc_var2 = ((i32_var6 + 0xe0) + 0x14);
    ppc_var2(offset, u_var1, (u_var1 >> 0x10), &local_a);
    return;
}

pub fn track_popup_menu_1020_7ad2(ctx: &mut AppContext, param_1: u32, param_2: u16) {
    let mut HVar1: HMENU16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: HWND16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0xee) != 0) && ((i_var2 + 0xec) == 0)) {
        HVar1 = LoadMenu16((i_var2 + 0xee), ctx.g_h_instance_1050_038c);
        (i_var2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
        local_6 = (i_var2 + 0xec);
        unaff_cs = SUB42(offset, 0);
        HVar1 = GetSubMenu16(0, local_6);
        (i_var2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    local_4 = param_2;
    local_6 = (i_var2 + 8);
    ClientToScreen16(CONCAT22(&local_6, unaff_cs), unaff_ss);
    TrackPopupMenu16(0x0, 0, (i_var2 + 8), 0, local_4, local_6, 0);
    return;
}

pub fn call_draw_fn_1020_79b4(param_1: *mut Struct674, param_2: u16, param_3: String) {
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 10)), param_3);
    if (param_2 != 0) {
        draw_1020_7cc8(*(param_1 + 0xe8));
    }
    return;
}

pub fn call_draw_fn_1020_79e4(ctx: &mut AppContext, param_1: Vec<u8>) {
    draw_1020_7cc8(ctx, *(param_1 + 0xe8));
    return;
}

pub unsafe fn call_load_cursor_fn_1020_7554(
    ctx: &mut AppContext,
    param_1: *mut Struct65,
    param_2: u16,
    param_3: u32,
) {
    let mut local_AX_24: u16;
    let mut local_DX_71: u16;
    let mut iVar1: i32;
    let mut local_BP__1: u16;
    let local_struct_1_hi: *mut Struct65;
    let ppVar2: *mut pass1_struct_1;

    draw1::load_cursor_1020_7f7a(ctx, param_1, CONCAT22(param_2, 5), param_3);
    local_struct_1_hi = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xee) = 0;
    (iVar1 + 0xf2) = 0;
    param_1.ptr_a_lo = 0x7780;
    (iVar1 + 2) = 0x1020;
    (iVar1 + 0xe2) = 0x781c;
    (iVar1 + 0xe4) = 0x1020;
    ppVar2 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_BP__1, 0x25));
    local_DX_71 = (ppVar2 >> 0x10);
    (iVar1 + 0xf2) = ppVar2;
    (iVar1 + 0xf4) = local_DX_71;
    (iVar1 + 0xe6) = (iVar1 + 0xf2);
    (iVar1 + 0xe8) = local_DX_71;
    return;
}

pub unsafe fn call_palette_fn_1020_679c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    call_palette_fn_1020_6466(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn load_cursor_1020_67ce(
    ctx: &mut AppContext,
    param_1: *mut Struct65,
    param_2: u16,
    param_3: u32,
) {
    let mut HVar1: HGDIOBJ16;
    let mut HVar2: HCURSOR16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut in_stack_0000fffc: u16;

    call_load_cursor_1020_790e(param_1, ctx.s_TPPOPMENU_1050_43fa, param_2, param_3);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0xf2) = 0;
    (i_var3 + 0xf6) = 0;
    param_1.ptr_a_lo = 0x70e6;
    (i_var3 + 2) = 0x1020;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var3 + 0x5b)),
        ctx.s_VrMode2_1050_4404,
    );
    HVar1 = GetStockObject16(5);
    (i_var3 + 0xc6) = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0xc4) = HVar2;
    (i_var3 + 0xac) = 0x44c00000;
    (i_var3 + 200) = (ctx.s_575_bmp_1050_201f + 1);
    (i_var3 + 0xbc) = (param_3 + 8);
    (i_var3 + 0xca) = param_2;
    reg_class_1008_96d2(param_1, in_stack_0000fffc);
    update_window_1020_6c3a(i_var3, u_var4);
    return;
}

pub unsafe fn cleanup_fn_1020_687c(param_1: *mut Struct652) {
    get_sys_metrics_1020_7a50(param_1);
    destroy_icon_1020_6bd2(param_1);
    return;
}

pub fn realize_palette_1020_6896(param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u32;
    let mut u_var4: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
        u_var4 = (param_1 >> 0x10);
        u_var2 = (param_1 + 0xf2);
        pu_var3 = (u_var2 + 0x24);
        let pu_var3_val = unsafe { *pu_var3 };
        pp_var1 = (pu_var3_val + 0x18);
        (**pp_var1)();
        UnrealizeObject16(pu_var3);
        u_var2 = (param_1 + 0xf2);
        RealizePalette16((u_var2 + 0x178));
    }
    return;
}

pub fn call_invalidate_rect_1020_68de(param_1: Vec<u8>) {
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    if ((param_1 + 0xf6) != 0) {
        invalidate_rect_1020_735a((param_1 + 0xf6));
    }
    return;
}

pub fn pt_in_rect_1020_68fc(param_1: *mut u32, param_2: i32, param_3: u16) {
    let pp_var1: fn();
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    local_6 = CONCAT22(param_2, param_3);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1018_31d0(*(i_var3 + 0xf2));
    if (param_2 != 0) {
        BVar2 = PtInRect16(local_6, ((i_var3 + 0xf2) + 0x16c));
        if (BVar2 != 0) {
            let param_1_val = unsafe { *param_1 };
            pp_var1 = (param_1_val + 0x40);
            (**pp_var1)(offset, i_var3, u_var4, 0xef);
        }
    }
    return;
}

pub unsafe fn win_fn_1020_694c(ctx: &mut AppContext, param_1: *mut Struct37, param_2: i32) -> u32 {
    let mut u_var1: u32;
    let mut lp_help_file: String;
    let mut u_var2: i32;
    let mut HVar3: HWND16;
    let local_bx_14: *mut Struct5;
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
                    HVar3 = invalidate_rect_1020_735a(u_var1, (u_var1 >> 0x10));
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

pub fn destroy_win_1020_6ae6(param_1: *mut u32, param_2: u16, param_4: i32, param_3: i32) {
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
                        invalidate_rect_1020_735a();
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

pub fn enable_menu_item_1020_6b9a() {
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;

    if (in_stack_0000000a != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0, in_stack_0000000c);
    return;
}

pub unsafe fn call_win_fn_1020_6bbc(in_struct_1: Vec<u8>) {
    win_fn_1020_737a((in_struct_1 + 0xf6));
    return;
}

pub unsafe fn destroy_icon_1020_6bd2(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut offset: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    DestroyIcon16((i_var4 + 0xc2));
    (i_var4 + 0xc2) = 0;
    (i_var4 + 8) = 0;
    pu_var1 = (i_var4 + 0xf6);
    u_var2 = (i_var4 + 0xf8);
    if ((u_var2 | pu_var1) != 0) {
        let pu_var1_val = unsafe { *pu_var1 };
        ppc_var3 = pu_var1_val;
        (**ppc_var3)(offset, pu_var1, u_var2, 1);
    }
    (i_var4 + 0xf6) = 0;
    pass1_1010_1dda(*(i_var4 + 0xf2));
    (i_var4 + 0xf2) = 0;
    return;
}

pub unsafe fn update_window_1020_6c3a(ctx: &mut AppContext, param_1: *mut win_struct_42) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: i32;
    let pi_var5: *mut u16;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let struct_a: *mut Struct199;
    let pa_var8: *mut Struct199;
    let struct_a_00: *mut Struct199;
    let struct_a_01: *mut Struct199;
    let struct_a_02: *mut Struct199;



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
        get_dc_1020_717e();
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

pub fn pass1_1020_6e52(ctx: &mut AppContext, param_1: *mut Struct674, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let lVar2: u32;
    let pcVar3: String;

    u_var1 = param_1.field_0xf2;
    lVar2 = pass1_1018_2e5e(u_var1, (u_var1 >> 0x10));
    if (lVar2 == 0) {
        pcVar3 = load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x5a1,
        );
    } else {
        pcVar3 = pass1_1018_2d84(param_1.field_0xf2);
    }
    call_draw_fn_1020_79b4(
        CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
        param_3,
        pcVar3,
    );
    return;
}

pub unsafe fn create_win_1020_6e98(ctx: &mut AppContext, param_1: *mut Struct37) {
    let pu_var1: *mut u16;
    let w_param: WPARAM16;
    let mut in_struct_1: u32;
    let mut h_var2: HWND16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut extraout_d_x: u16;

    let local_bx_5: *mut Struct37;
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

pub fn set_colors_1020_7070(ctx: &mut AppContext) {
    let mut in_dx: i32;
    let mut CVar1: COLORREF;
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: HDC16;
    let mut local_4: u16;

    GetStockObject16(4);
    if (ctx._PTR_LOOP_1050_441e == 0) {
        ctx._PTR_LOOP_1050_441e = 0x1000002;
    }
    if (6 < in_stack_0000000a) {
        return in_dx << 0x10;
    }
    SetTextColor16(ctx._PTR_LOOP_1050_441e, in_stack_0000000c);
    CVar1 = SetBkColor16(0x1000000, in_stack_0000000c);
    return CVar1 & 0xffff0000 | 0x100;
}

pub unsafe fn cleanup_fn_1020_70c0(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn get_dc_1020_717e(ctx: &mut AppContext, param_1: *mut u16, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u32;
    let pu_var4: *mut u16;
    let pu_var5: *mut u32;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let unaff_ss: u8;
    let pp_var8: *mut pass1_struct_1;
    let u_var9: u8;
    let u_var10: u8;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut in_stack_0000ffdc: u16;
    let mut local_1c: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    i_var11 = param_1;
    u_var12 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(i_var11, u_var12, param_2, (param_2 >> 0x10));
    (i_var11 + 0x14) = 0;
    (i_var11 + 0x18) = param_2;
    (i_var11 + 0x1c) = 0;
    (i_var11 + 0x20) = 0;

    *param_1 = 0x754c;

    (i_var11 + 2) = 0x1020;
    pp_var8 =
        process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffdc, 4));
    u_var7 = (pp_var8 >> 0x10);
    (i_var11 + 0x1c) = pp_var8;
    (i_var11 + 0x1e) = u_var7;
    pp_var1 = ((i_var11 + 0x1c) + 4);
    (**pp_var1)(0x10, (i_var11 + 0x1c), u_var7, 0, i_var11, u_var12);
    local_4 = GetDC16((i_var11 + 4));
    u_var2 = (i_var11 + 0x1c);
    (u_var2 + 0x178) = local_4;
    u_var2 = (i_var11 + 0x1c);
    u_var7 = (u_var2 >> 0x10);
    i32_var6 = u_var2;
    pu_var5 = (i32_var6 + 0x24);
    u_var9 = SUB41(pu_var5, 0);
    u_var10 = (pu_var5 >> 8);
    pp_var1 = (*pu_var5 + 0x14);
    (**pp_var1)(0x38, u_var9, (i32_var6 + 0x26));
    pp_var8 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT13(u_var10, CONCAT12(u_var9, 0x29)),
    );
    u_var2 = (pp_var8 + 0xe);
    pu_var3 = u_var2;
    pass1_1008_4d84((pu_var5 & 0xffff | ctx.dx_reg << 0x10), pu_var3);
    pu_var4 = &local_4;
    draw1::realize_palette_1008_4e08(pu_var3, u_var2, pu_var4, ctx.stack_seg_reg);
    (i_var11 + 0x20) = pu_var4;
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

pub fn post_win_msg_1020_7308(param_1: u32, param_2: i32) {
    let mut cVar1: u8;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if (param_2 != 0x12) {
        if (param_2 < 0x13) {
            cVar1 = param_2;
            if (cVar1 == 0x1) {
                (param_1 + 0x1c) = 0;
                return;
            }
            if (('\x03' < (cVar1 + -1)) && ((cVar1 + -5) < 0x2)) {}
            // goto LAB_1020_7310;
        }
        return;
    }
    // LAB_1020_7310:
    PostMessage16(0, 0xeb, 0x111, (param_1 + 4));
    invalidate_rect_1020_735a(param_1);
    return;
}

pub fn invalidate_rect_1020_735a(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x1c);
    InvalidateRect16(0, (u_var1 + 0x16c), (u_var1 >> 0x10));
    return;
}

pub unsafe fn win_fn_1020_737a(ctx: &mut AppContext, in_struct_1: *mut Struct15) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut b_result: u16;
    let pu_var4: Vec<u8>;
    let mut u_var5: u32;


    let local_bx_4: *mut Struct15;
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

pub fn draw_1020_650c(param_1: *mut Struct622) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut local_28: PAINTSTRUCT16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut offset: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = (i_var3 + 0x14);
    local_6 = (u_var2 + 10);
    if (((i_var3 + 0x10) != 0) || (u_var2 = (i_var3 + 0x14), (u_var2 + 0x24) != 0)) {
        draw1::draw_1020_9364(param_1);
        if ((i_var3 + 0x24) != 0) {
            pp_var1 = ((i_var3 + 0x24) + 0x14);
            (**pp_var1)();
        }
    }
    local_8 = 0;
    while {
        if ((i_var3 + 0x18 + local_8 * 4) != 0) {
            pp_var1 = ((i_var3 + 0x18 + local_8 * 4) + 8);
            (**pp_var1)();
        }
        local_8 = local_8 + 1;
        local_8 < 5
    } {}
    BeginPaint16(CONCAT22(unaff_ss, &local_28), (i_var3 + 4));
    pp_var1 = (local_6 + 4);
    (**pp_var1)(
        offset,
        local_6,
        (local_6 >> 0x10),
        0,
        0,
        i_var3 + 10,
        u_var4,
    );
    EndPaint16(&local_28, unaff_ss);
    return;
}

pub fn send_win_msg_1020_65cc(ctx: &mut AppContext, param_1: *mut Struct672, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let b_var3: bool;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut local_4: u16;

    i32_var6 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (i32_var6 + 0x14) = 0;
        return;
    }
    if (param_2 == 2) {
        local_4 = 0;
        while (local_4 < 5) {
            i_var5 = i32_var6 + 0x18;
            i_var7 = local_4 * 4;
            if (((i_var5 + i_var7 + 2) | (i_var5 + i_var7)) != 0) {
                pp_var1 = ((i_var5 + i_var7) + 4);
                (**pp_var1)();
            }
            local_4 = local_4 + 1;
        }
    } else {
        if (((0 < param_2 + -3) && (!SBORROW2(param_2 + -3, 1))) && (param_2 + -4 < 4)) {
            b_var3 = IsIconic16(ctx.g_h_window);
            if (b_var3 == 0) {
                b_var3 = IsIconic16((i32_var6 + 4));
                if ((b_var3 == 0) && (u_var2 = (i32_var6 + 0x14), (u_var2 + 0x24) != 0)) {
                    InvalidateRect16(0, 0x0, 0);
                    pass1_1020_64d4(param_1, 2);
                    if (b_var3 == 0) {
                        call_fn_ptr_1_1020_6746(param_1, 1, 2);
                    }
                    pass1_1020_64d4(param_1, 3);
                    if (b_var3 == 0) {
                        call_fn_ptr_1_1020_6746(param_1, 1, 3);
                    }
                    u_var4 = pass1_1018_255e((i32_var6 + 0x14));
                    if (u_var4 == 0) {
                        SendMessage16(0, 0x69, 0x111, (i32_var6 + 4));
                    } else {
                        pass1_1020_64d4(param_1, 1);
                        if (u_var4 == 0) {
                            call_fn_ptr_1_1020_6746(param_1, 1, 1);
                        }
                    }
                    SendMessage16(0, 0xf0, 0x111, (i32_var6 + 4));
                    u_var2 = (i32_var6 + 0x2c);
                    if ((u_var2 + 0x7a) != 0) {
                        u_var2 = (i32_var6 + 0x2c);
                        (u_var2 + 0x7a) = 0;
                        SendMessage16(0, 0x131, 0x111, (i32_var6 + 4));
                        return;
                    }
                }
            }
        }
    }
    return;
}

pub unsafe fn call_palette_fn_1020_6466(in_struct_1: &mut Struct44) {
    let local_struct_1: &mut Struct44;
    let local_struct_1_hi: &mut Struct44;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x67c2;
    local_struct_1.base_fld_2 = 0x1020;
    if (local_struct_1.field_0x14 != 0) {
        pass1_1010_1ea6(
            local_struct_1.field_0x14,
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        );
    }
    draw1::select_and_delete_palette_1020_92c4(in_struct_1);
    return;
}

pub unsafe fn cleanup_fn_1020_6216(in_struct_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    destroy_win_1020_42f4(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub unsafe fn process_struct_1020_62e0(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct668,
    param_2: u32,
) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let u_var3: u8;
    let mut u_var4: i32;
    let mut extraout_var;
    let mut u_var5: u32;

    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let paVar6: *mut Struct199;

    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    // ppu_var9: *mut Vec<u8>;
    let ppu_var9: u16;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let paVar13: *mut Struct668;
    let mut u_var14: u16;
    let mut in_stack_0000ffee: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    // fn_ptr_3: *mut Vec<u8>;
    let fn_ptr_3: fn();
    // fn_ptr_1: *mut Vec<u8>;
    let fn_ptr_1: fn();

    u_var12 = param_2;
    draw1::get_dc_1020_921c(CONCAT22(u_var12, in_struct_1), (param_2 >> 0x10));
    &in_struct_1.fn_ptr_1_0x14 = 0;
    &in_struct_1.field_0x2c = 0;
    CONCAT22(u_var12, in_struct_1) = 0x67c2;
    in_struct_1.field_0x2 = 0x1020;
    paVar6 = ctx.dx_reg;
    u_var3 = pass1_1000_4906(CONCAT22(u_var12, &in_struct_1.field_0x18), 0, 0x14);
    u_var4 = CONCAT31(extraout_var, u_var3);
    process_struct_1000_179c(0x3c, paVar6);
    struct_a = (paVar6 | u_var4);
    if (struct_a == 0x0) {
        &in_struct_1.field_0x1c = 0;
    } else {
        pass1_1020_87c2(u_var4, paVar6);
        in_struct_1.field_0x1c = u_var4;
        in_struct_1.field_0x1e = ctx.dx_reg;
        struct_a = ctx.dx_reg;
    }
    process_struct_1000_179c(0x26, struct_a);
    if ((struct_a | u_var4) == 0) {
        u_var4 = 0;
        paVar6 = 0x0;
    } else {
        pass1_1020_8a9c(u_var4, struct_a);
        paVar6 = ctx.dx_reg;
    }
    in_struct_1.field_0x20 = u_var4;
    in_struct_1.field_0x22 = paVar6;
    process_struct_1000_179c(0xbe, paVar6);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        paVar6 = 0x0;
    } else {
        pass1_1020_8eaa(u_var4, paVar6);
        paVar6 = ctx.dx_reg;
    }
    in_struct_1.field_0x24 = u_var4;
    in_struct_1.field_0x26 = paVar6;
    process_struct_1000_179c(0x20, paVar6);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        u_var7 = 0;
    } else {
        process_struct_1020_8360(CONCAT22(paVar6, u_var4));
        u_var7 = ctx.dx_reg;
    }
    in_struct_1.field_0x28 = u_var4;
    in_struct_1.field_0x2a = u_var7;
    call_fn_ptr_1_1020_6746(CONCAT22(u_var12, in_struct_1), 1, 4);
    pp_var8 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffee, 0x29),
    );
    in_struct_1.fn_ptr_1_0x14 = pp_var8;
    &in_struct_1.field_0x16 = (pp_var8 >> 0x10);
    u_var11 = 0;
    u_var10 = (pp_var8 >> 0x10);
    ppu_var9 = in_struct_1.fn_ptr_1_0x14;
    fn_ptr_1 = (&in_struct_1.fn_ptr_1_0x14 + 4);
    paVar13 = in_struct_1;
    u_var14 = u_var12;
    (**fn_ptr_1)();
    in_struct_1.field_0x6 = &in_struct_1.fn_ptr_1_0x14;
    u_var2 = &in_struct_1.fn_ptr_1_0x14;
    pu_var1 = (u_var2 + 10);
    u_var5 = param_2 << 0x10 | &in_struct_1.field_0xa;
    u_var7 = SUB42(pu_var1, 0);
    let pu_var1_val = unsafe { *pu_var1 };
    fn_ptr_3 = (pu_var1_val + 8);
    (**fn_ptr_3)(
        0x1010,
        u_var7,
        (pu_var1 >> 0x10),
        u_var5,
        ppu_var9,
        u_var10,
        u_var11,
        paVar13,
        u_var14,
    );
    in_struct_1.field_0x12 = u_var5;
    in_struct_1.field_0x10 = 1;
    pp_var8 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(u_var7, 2));
    in_struct_1.field_0x2c = pp_var8;
    in_struct_1.field_0x2e = (pp_var8 >> 0x10);
    return;
}

pub unsafe fn set_cursor_1020_5de8(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct664,
    param_2: u16,
    in_struct_2: *mut Struct665,
) {
    let mut u_var1: u32;
    let local_struct_2_1: *mut Struct665;

    let local_struct_1: *mut Struct664;
    let mut local_struct_1_hi: u16;
    let mut unaff_ss: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffe2: HWND16;
    let mut in_stack_0000ffe8: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_struct_2: *mut Struct665;
    let mut local_4: u16;

    ReleaseCapture16(in_stack_0000ffe2);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    SetCursor16(local_struct_1.cursor_handle_0xee);
    local_struct_1.cursor_handle_0xee = 0;
    local_struct_1.field_0xf4 = 1;
    local_struct_2 = in_struct_2;
    local_4 = param_2;
    ppVar2 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x47),
    );
    local_struct_2_1 = &local_struct_2;
    pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_ss, local_struct_2_1));
    if ((ctx.dx_reg | local_struct_2_1) != 0) {
        u_var1 = &local_struct_2_1.field_0x42;
        local_struct_1_hi = &local_struct_2_1.field_0x44;
        local_12._3_1_ = (u_var1 >> 0x18);
        if (local_12._3_1_ == '\x05') {
            local_12._0_2_ = u_var1;
            // goto LAB_1020_5e62;
        }
    }
    local_12._0_2_ = 0;
    local_struct_1_hi = 0;
    // LAB_1020_5e62:
    pass1_1018_57e6(ppVar2, CONCAT22(local_struct_1_hi, local_12));
    return;
}

pub unsafe fn win_fn_1020_5e76(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct663,
    param_2: u16,
    param_3: u16,
) {
    let pu_var1: Vec<u8>;
    let ppc_var2: fn();
    let u_var3: u8;
    let pu_var4: *mut u32;
    let extraout_var;
    let pu_var5: *mut u16;

    let struct_a: *mut Struct199;
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
    pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_ss, local_2a8));
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
            call_invalidate_rect_1020_68de(pu_var1);
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

pub fn set_capture_1020_6184(in_struct_1: *mut Struct661, param_2: u16) {
    let mut cursor: HCURSOR16;
    let local_struct_1: *mut Struct661;
    let local_struct_1_hi: *mut Struct661;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0xf4 == 1) {
        cursor = SetCursor16(local_struct_1.cursor_handle_0xf0);
        local_struct_1.cursor_handle_0xee = cursor;
        local_struct_1.field_0x10c = param_2;
        SetCapture16(local_struct_1.window_handle_0x8);
        local_struct_1.field_0xf4 = 2;
    }
    return;
}

pub unsafe fn post_msg_1020_55b0(ctx: &mut AppContext, param_1: u32) {
    let pp_var1: fn();
    let pu_var2: *mut u16;
    let pu_var3: *mut u16;
    let mut u_var4: u16;
    let pu_var5: *mut u16;

    let struct_a: *mut Struct199;
    let mut u_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let pp_var9: *mut pass1_struct_1;
    let mut in_stack_0000fed8: u16;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u16;
    let mut local_114: u16;
    let mut local_112: [u8; 2];
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: [u8; 256];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 =
        process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fed8, 2));
    local_8 = (local_6 + 0x20);
    local_c = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000fed8, 0x37),
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, local_10c),
        0x59c,
    );
    pass1_1008_9436(CONCAT22(unaff_ss, local_112));
    pu_var2 = &local_114;
    load_string_1008_a8f4(
        local_c, pu_var2, unaff_ss, 0xee, unaff_ss, &local_10e, unaff_ss,
    );
    local_118 = CONCAT22(ctx.dx_reg, pu_var2);
    struct_a = (ctx.dx_reg | pu_var2);
    u_var7 = (param_1 >> 0x10);
    if ((struct_a != 0x0) && (*local_118 != '\0')) {
        u_var8 = 0x1000;
        pu_var3 = pu_var2;
        process_struct_1000_179c(0xb4, struct_a);
        local_120 = CONCAT22(struct_a, pu_var3);
        u_var6 = struct_a | pu_var3;
        if (u_var6 == 0) {
            u_var4 = 0;
            u_var6 = 0;
        } else {
            u_var8 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pu_var5 = process_struct_1040_8478(
                local_120,
                0,
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_10c)),
                CONCAT22(ctx.dx_reg, pu_var2),
                (param_1 + 8),
            );
            u_var4 = SUB42(pu_var5, 0);
        }
        local_11c = CONCAT22(u_var6, u_var4);
        if (local_110 == 0) {
            pp_var1 = (*local_11c + 0x74);
            (**pp_var1)(u_var8, u_var4, u_var6);
        } else {
            pp_var1 = (*local_11c + 0x6c);
            (**pp_var1)(u_var8, u_var4, u_var6, local_112);
        }
        if ((local_8 == 0) || (local_114 == 0)) {
            PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        }
    }
    if ((local_8 != 0) && (local_114 != 0)) {
        SendMessage16(0, local_114, 0x111, ctx.g_h_window);
        (param_1 + 0x112) = 1;
        return;
    }
    if (local_10e == 6) {
        PostMessage16(0, 0xb0, 0x111, ctx.g_h_window);
        pp_var9 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_126, 2));
        local_126 = pp_var9;
        (local_126 + 0x20) = 1;
    }
    if (local_10e == 0x15) {
        PostMessage16(0, 0x97, 0x111, ctx.g_h_window);
        pp_var9 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_126, 2));
        (pp_var9 + 0x20) = 1;
    }
    return;
}

pub unsafe fn set_cursor_1020_5764(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct673,
    param_2: u16,
) {
    let mut u_var1: i32;
    let pu_var2: Vec<u8>;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut h_cursor: HCURSOR16;
    let local_struct_1: *mut Struct673;
    let local_struct_1_hi: *mut Struct673;
    let mut unaff_ss: u16;
    let mut in_stack_0000ffe6: u16;
    let mut in_stack_0000ffe8: HWND16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 2];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffe6, 0x2f),
    );
    u_var4 = (local_6 >> 0x10);
    local_a = (local_6 + 0x20);
    u_var1 = (local_6 + 0x22);
    if ((u_var1 | local_a) != 0) {
        pass1_1030_8308(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, local_c),
            local_a & 0xffff | u_var1 << 0x10,
        );
        if (param_2 <= local_e) {
            local_struct_1_hi = (in_struct_1 >> 0x10);
            local_struct_1 = in_struct_1;
            if (local_struct_1.field_0xf4 != 1) {
                SetCursor16(local_struct_1.field_0xee);
                local_struct_1.field_0xee = 0;
                local_struct_1.field_0xf4 = 1;
                &local_struct_1.field_0x10c = 0;
                ReleaseCapture16(in_stack_0000ffe8);
            }
            h_cursor = LoadCursor16(0x7f02, 0);
            SetCursor16(h_cursor);
            local_12 = param_2;
            pass1_1018_017c(local_6, param_2);
            pu_var2 = local_struct_1.field_0xf6;
            (pu_var2 + 0x10) = 1;
            if (&local_struct_1.field_0xfe != 0) {
                call_invalidate_rect_1020_68de(*&local_struct_1.field_0xfe);
                u_var3 = &local_struct_1.field_0xfe;
                local_12 = (u_var3 + 8);
                PostMessage16(0, 0xeb, 0x111, local_12);
            }
            SetCursor16(local_12);
        }
    }
    return;
}

pub unsafe fn pt_in_rect_1020_5856(in_struct_1: *mut Struct664, in_struct_2: *mut Struct665) {
    let pu_var1: *mut u32;
    let mut in_a_x: i32;
    let b_var2: bool;
    let mut u_var3: u32;
    let mut extraout_d_x: i32;
    let mut extraout_d_x_00: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1018_2862((in_struct_1 + 0xfa));
    if ((extraout_d_x | in_a_x) != 0) {
        local_a = 0;
        while (true) {
            pu_var1 = (in_a_x + 10);
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val < local_a || pu_var1_val == local_a) {
                break;
            }

            u_var3 = local_a;
            bad_func_1008_8fc4(CONCAT22(extraout_d_x, in_a_x), local_a);
            if ((extraout_d_x_00 | u_var3) != 0) {
                b_var2 = PtInRect16(in_struct_2.field_0x0, (u_var3 + 0x14));
                if (b_var2 != 0) {
                    return;
                }
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub unsafe fn win_fn_1020_58ce(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct663,
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
                pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_ss, local_1e));
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

pub unsafe fn track_popup_menu_1020_5bf2(
    ctx: &mut AppContext,
    param_1: *mut Struct26,
    param_2: u16,
    param_3: u16,
) -> bool {
    let mut iVar1: i32;
    let mut pt_in_rect: u16;
    let mut tile_menu_handle: u16;
    let mut HVar2: HMENU16;
    let mut u_var3: i32;
    let local_bx_18: *mut Struct26;
    let mut u_var4: u16;
    let mut unaff_cs: u16;
    let mut h_window: u16;
    let mut u_var5: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut rect: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;
    let mut tile_menu_name: String;

    local_6 = param_3;
    local_4 = param_2;
    u_var4 = (param_1 >> 0x10);
    local_bx_18 = param_1;
    iVar1 = pass1_1020_64d4(local_bx_18.field_0xf6, 2);
    if (iVar1 != 0) {
        rect = pass1_1020_6498(local_bx_18.field_0xf6, 2);
        unaff_cs = SUB42(offset, 0);
        pt_in_rect = PtInRect16(CONCAT22(local_4, local_6), rect);
        if (pt_in_rect != 0) {
            PostMessage16(0, 0x131, 0x111, ctx.g_h_window);
            return 1;
        }
    }
    iVar1 = pass1_1020_64d4(local_bx_18.field_0xf6, 3);
    if (iVar1 == 0) {
        return 0;
    }
    u_var5 = pt_in_rect_1020_5856(param_1, CONCAT22(h_window, &local_6));
    u_var3 = (u_var5 >> 0x10);
    local_bx_18.field_0x108 = u_var5;
    &local_bx_18.field_0x10a = u_var3;
    if ((u_var3 | local_bx_18.field_0x108) == 0) {
        return 0;
    }
    if (local_bx_18.menu_handle == 0) {
        tile_menu_handle = LoadMenu16(tile_menu_name, ctx.g_h_instance_1050_038c);
        local_bx_18.menu_handle = tile_menu_handle;
        if (tile_menu_handle == 0) {
            return 1;
        }
        unaff_cs = SUB42(offset, 0);
        HVar2 = GetSubMenu16(0, local_bx_18.menu_handle);
        local_bx_18.menu_handle = HVar2;
        if (HVar2 == 0) {
            return 1;
        }
    }
    u_var5 = &local_bx_18.field_0x108;
    rect._0_2_ = (u_var5 + 0x2e);
    local_c = 0;
    if (rect == 0x42) {
        local_c = 0xc9;
    } else {
        if (((ctx.s_VrMode_1050_42ca + 8) + rect * 2) == 0) {
            local_c = 200;
        }
    }
    if (local_c != 0) {
        unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, CONCAT22(local_c, 1));
    }
    local_10 = param_3;
    local_e = param_2;
    ClientToScreen16(CONCAT22(&local_10, unaff_cs), h_window);
    TrackPopupMenu16(0x0, 0, local_bx_18.field_0x8, 0, local_e, local_10, 0);
    return 1;
}

pub unsafe fn enable_menu_item_1020_44ec(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_4: i32,
    param_3: u16,
) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let ppVar4: *mut pass1_struct_2;
    let pu_var5: *mut u16;
    let b_var6: bool;
    let extraout_var;
    let mut in_dx: i32;
    let mut u_var7: i32;

    let mut i_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let u_var12: u8;
    let mut unaff_ss: u16;
    let mut h_var13: HMENU16;
    let mut h_menu: u16;
    let mut local_138: u16;
    let pp_stack_310: *mut pass1_struct_2;
    let mut local_134: u16;
    let mut local_130: u16;
    let mut local_12e: u16;
    let mut local_12c: u32;
    let mut local_128: u32;
    let mut local_124: u32;
    let mut local_11e: u32;
    let mut local_116: u32;
    let mut local_10e: u16;
    let mut local_10c: u32;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    u_var10 = (param_1 >> 0x10);
    i_var8 = param_1;
    if ((i_var8 + 0x106) != 0) {
        if (*(i_var8 + 0x106) == param_3) {
            local_6 =
                process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_138, 3));
            u_var1 = (i_var8 + 0x108);
            local_8 = (u_var1 + 0x2e);
            u_var1 = (i_var8 + 0x108);
            u_var11 = (u_var1 >> 0x10);
            i_var9 = u_var1;
            local_128 = (i_var9 + 0x42);
            u_var7 = (i_var9 + 0x44);
            local_128._3_1_ = (local_128 >> 0x18);
            u_var3 = local_128._3_1_;
            local_11e = local_128;
            local_10c = local_128;
            if (local_128._3_1_ == 0) {
                big_switch_statement_1020_bd80(local_8);
                u_var11 = 0x1000;
                copy_string_1000_3d3e(CONCAT22(unaff_ss, &local_108), CONCAT22(u_var7, u_var3));
            } else {
                ppVar4 = pass1_1030_8344(
                    ctx._g_bool_1050_5748,
                    (ctx._g_bool_1050_5748 >> 0x10),
                    local_128 & 0xffff | u_var7 << 0x10,
                );
                local_128 = CONCAT22(u_var7, ppVar4);
                u_var11 = 0x1010;
                pass1_1010_c3c2(
                    local_6,
                    (local_6 >> 0x10),
                    CONCAT22(unaff_ss, &local_108),
                    CONCAT22(u_var7, ppVar4),
                );
            }
            pu_var5 = &local_108;
            ModifyMenu16(u_var11, pu_var5, unaff_ss, 0x76, 0, 0x76, (i_var8 + 0x106));
            local_10e = pu_var5;
            GetMenuState16(offset, 0, 0x3c, (i_var8 + 0x106));
            if (pu_var5 != 0xffff) {
                DeleteMenu16(0x38, 0, 0x13c, (i_var8 + 0x106));
            }
            b_var6 = pass1_1008_c6ae(ctx.ctx._PTR_LOOP_1050_06e0, local_8, 0x20);
            if (b_var6 != 0) {
                load_string_1010_847e(
                    ctx._g_struct_73_1050_14cc,
                    (ctx._g_struct_73_1050_14cc >> 0x10),
                    0x74b,
                );
                local_128 = CONCAT22(ctx.dx_reg, b_var6);
                InsertMenu16(
                    0x1010,
                    b_var6,
                    ctx.dx_reg,
                    0x3c,
                    0x400,
                    0xffff,
                    (i_var8 + 0x106),
                );
            }
            if (((ctx.s_VrMode_1050_42ca + 8) + local_8 * 2) == 0) {
                local_128._0_2_ = *(i_var8 + 0x106);
                h_var13 = 1;
                h_menu = 0x77;
                // goto LAB_1020_464c;
            }
            local_128._0_2_ = *(i_var8 + 0x106);
            h_menu = 0x77;
        } else {
            h_var13 = GetSubMenu16(1, (i_var8 + 0x106));
            local_128 = local_128 & 0xffff0000 | h_var13;
            if (h_var13 != param_3) {}
            // goto LAB_1020_479e;
            EnableMenuItem16(1, 0x200, h_var13);
            local_138 = local_128;
            EnableMenuItem16(1, 0x201, local_128);
            pp_stack_310 = local_128;
            local_138 = 0x202;
            EnableMenuItem16(1, 0x202, local_128);
            u_var1 = (i_var8 + 0x108);
            local_124 = (u_var1 + 0x42);
            pp_stack_310 = local_124;
            local_134 = (local_124 >> 0x10);
            local_138 = (ctx._g_bool_1050_5748 >> 0x10);
            pp_stack_310 = pass1_1030_8344(ctx._g_bool_1050_5748, local_138, local_124);
            local_11e = CONCAT22(in_dx, pp_stack_310);
            if ((in_dx | pp_stack_310) == 0) {
                return;
            }
            local_116 = &pp_stack_310.field_0x2e;
            if ((&pp_stack_310.field_0x30 | local_116) == 0) {
                return;
            }
            local_10c = (local_116 + 0x200);
            local_138 = 0x1030;
            local_108 = pass1_1030_73a8(CONCAT22(in_dx, pp_stack_310));
            u_var10 = (local_108 >> 0x10);
            local_6 = (local_108 + 0x1c);
            u_var7 = (local_108 + 0x1e);
            if ((u_var7 | local_6) != 0) {
                local_10c = local_6 & 0xffff | u_var7 << 0x10;
            }
            if (local_10c != 1) {
                return;
            }
            if ((local_10c & 0xff0000) != 0) {
                return;
            }
            pp_stack_310 = local_11e;
            local_134 = (local_11e >> 0x10);
            local_138 = 0x1030;
            u_var2 = pass1_1030_6fa0(local_11e);
            pp_stack_310 = CONCAT31(extraout_var, u_var2);
            local_134 = 0x3f;
            local_138 = (ctx.ctx._PTR_LOOP_1050_06e0 >> 0x10);
            b_var6 = pass1_1008_c6ae(ctx.ctx._PTR_LOOP_1050_06e0, pp_stack_310, 0x3f);
            if (b_var6 != 0) {
                local_134 = local_128;
                local_138 = 0;
                pp_stack_310 = 0x201;
                b_var6 = EnableMenuItem16(0, 0x201, local_128);
            }
            if ((local_11e + 0x36) != 0) {
                b_var6 = EnableMenuItem16(0, 0x202, local_128);
            }
            pass1_1030_69cc(local_11e);
            if (b_var6 == 0) {
                return;
            }
            h_menu = 0x200;
        }
        h_var13 = 0;
        // goto LAB_1020_464c;
    }
    // LAB_1020_479e:
    i_var9 = param_2 + -1;
    if (i_var9 == 0) {
        u_var1 = (i_var8 + 0xfa);
        pass1_1018_2504(u_var1, (u_var1 >> 0x10));
        if (i_var9 == 0) {
            h_menu = 0;
            EnableMenuItem16(0x401, 0, param_3);
            local_138 = param_3;
            local_128._0_2_ = 1;
            // LAB_1020_47e3:
            h_var13 = 0x401;
            // goto LAB_1020_464c;
        }
        h_menu = 0;
        EnableMenuItem16(0x400, 0, param_3);
        local_138 = param_3;
        local_128._0_2_ = 1;
    } else {
        i_var9 = param_2 + -2;
        if (i_var9 == 0) {
            pass1_1020_64d4(*(i_var8 + 0xf6), 2);
            if (i_var9 == 0) {
                EnableMenuItem16(0x401, 0, param_3);
                h_menu = 0x401;
            } else {
                EnableMenuItem16(0x400, 0, param_3);
                h_menu = 0x400;
            }
            local_128._0_2_ = 1;
            local_138 = param_3;
            EnableMenuItem16(h_menu, 1, param_3);
            if ((ctx.PTR_LOOP_1050_0010 != 0x0) || ((i_var8 + 0x102) == 0)) {
                pp_stack_310 = param_3;
                local_138 = 5;
                // goto LAB_1020_47e3;
            }
            pp_stack_310 = param_3;
            local_138 = 5;
        } else {
            if (param_2 == 3) {
                local_138 = 0;
                local_134 = 0;
                u_var12 = 0x10;
                local_130 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x2f);
                u_var10 = (local_130 >> 0x10);
                local_12c = (local_130 + 0x20);
                u_var7 = (local_130 + 0x22);
                if ((u_var7 | local_12c) != 0) {
                    u_var12 = 0x30;
                    pass1_1030_8308(
                        ctx._g_bool_1050_5748,
                        (ctx._g_bool_1050_5748 >> 0x10),
                        CONCAT22(unaff_ss, &local_134),
                        CONCAT22(unaff_ss, &local_138),
                        local_12c & 0xffff | u_var7 << 0x10,
                    );
                    local_138 = (local_130 + 0x1e);
                }
                local_128 = local_128 & 0xffff0000;
                while {
                    CheckMenuItem16(u_var12, 0x400, local_128, param_3);
                    u_var12 = 0x38;
                    EnableMenuItem16(0x401, local_128, param_3);
                    local_128 = local_128 & 0xffff0000 | (local_128 + 1);
                    (local_128 + 1) < 5
                } {}
                CheckMenuItem16(0x38, 0x408, local_138, param_3);
                local_128 = local_128 & 0xffff0000;
                while (local_128 <= local_134) {
                    EnableMenuItem16(0x400, local_128, param_3);
                    local_128 = local_128 & 0xffff0000 | (local_128 + 1);
                }
                return;
            }
            if (param_2 != 4) {
                return;
            }
            h_menu = 2;
            local_128._0_2_ = param_3;
        }
    }
    h_var13 = 0x400;
    // LAB_1020_464c:
    EnableMenuItem16(h_var13, h_menu, local_128);
    return;
}

pub unsafe fn win_fn_1020_493c(ctx: &mut AppContext, in_struct_1: *mut Struct673, param_2: u16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let in_bool_1: bool;
    let mut HVar3: HCURSOR16;
    let pu_var4: *mut u32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let in_dx: *mut Struct199;
    let pu_var7: Vec<u8>;

    let struct_a: *mut Struct199;
    let mut u_var8: i32;


    let mut in_bx: i32;
    let local_struct_1: *mut Struct673;
    let local_struct_1_hi: *mut Struct673;
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
        set_cursor_1020_5764(in_struct_1, u_var12);
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
                    post_msg_1020_55b0(in_struct_1);
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

pub unsafe fn win_fn_1020_51c6(in_struct_1: *mut Struct663, param_2: u16, param_3: u32) {
    let mut iVar1: i32;
    let local_struct_1: *mut Struct663;
    let local_struct_1_hi: *mut Struct663;
    let in_struct_2: *mut Struct665;
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
    set_cursor_1020_5de8(
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
    b_var3 = track_popup_menu_1020_5bf2((param_1 & 0xffff | u_var5 << 0x10), param_2, param_3);
    if (b_var3 == 0) {
        ppc_var2 = ((i_var4 + 4) + 0x60);
        ppc_var2();
    }
    return;
}

pub unsafe fn pass1_1020_52de(in_struct_1: *mut Struct594) {
    // fn_ptr_2: *mut Vec<u8>;
    let local_struct_1_2: *mut Struct594;
    let local_struct_1: *mut Struct594;
    let local_struct_1_hi: *mut Struct594;
    let mut in_stack_0000fff2: u16;
    // fn_ptr_1: *mut Vec<u8>;
    let mut fn_ptr_1: fn();
    let mut fn_ptr_2: fn();
    let temp_5f6e246310: *mut u32;
    let mut local_u16_2: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    temp_5f6e246310 = local_struct_1.field_0xf6;
    local_u16_2 = local_struct_1.field_0xf8;
    if ((local_u16_2 | temp_5f6e246310) != 0) {
        fn_ptr_1 = unsafe { *temp_5f6e246310 };
        (**fn_ptr_1)();
    }
    &local_struct_1.field_0xf6 = 0;
    if (local_struct_1.u8_ptr_32_0xfa != 0x0) {
        if ((local_struct_1_hi | local_struct_1) == 0) {
            fn_ptr_2 = 0x0;
            local_struct_1_2 = 0x0;
        } else {
            fn_ptr_2 = &local_struct_1.u32_xE2;
            local_struct_1_2 = local_struct_1_hi;
        }
        pass1_1010_1ea6(
            local_struct_1.u8_ptr_32_0xfa,
            CONCAT22(local_struct_1_2, fn_ptr_2),
        );
    }
    destroy_win_1008_628e(in_struct_1, in_stack_0000fff2);
    if (local_struct_1.u8_ptr_32_0xfa != 0x0) {
        pass1_1010_1dda(local_struct_1.u8_ptr_32_0xfa);
    }
    local_struct_1.u8_ptr_32_0xfa = 0x0;
    return;
}

pub unsafe fn gui_window_func_1020_536e(
    ctx: &mut AppContext,
    param_1: *mut Struct675,
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
    let in_dx: *mut Struct199;
    let struct_a: *mut Struct199;
    let pu_var7: *mut u32;
    let local_bx_557: *mut Struct18;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let pp_var9: *mut pass1_struct_1;
    let in_struct_1: *mut Struct651;
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
                u_var11 = load_cursor_1020_67ce(
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

pub unsafe fn draw_1020_40ce(
    param_1: u32,
    param_2: *mut Struct132,
    param_3: *mut Struct134,
    in_hdc_4: u16,
) {
    let mut stock_obj_5: u16;
    let mut obj_handle: HPEN16;
    let mut stock_obj: u16;
    let mut HVar1: HGDIOBJ16;
    let mut rect_right: u16;
    let mut rect_bottom: u16;
    let mut unaff_ss: u16;
    let mut hdc_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut hdc_a: u16;
    let temp_5f8e606965: *mut Struct133;

    pass1_1008_3e94(
        &(param_1)[1].field_0x4,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    pass1_1008_3e94(
        param_1,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    temp_5f8e606965 = (param_1).field_0xa;
    Ellipse16(
        param_2 + (temp_5f8e606965 + local_6),
        (param_3 + (temp_5f8e606965 + local_4)),
        (param_2 + (local_6 - (param_1).field_0xa)),
        (param_3 + (local_4 - (param_1).field_0xa)),
        in_hdc_4,
    );
    if ((*&(param_1)[1].field_0x2 & 1) != 0) {
        stock_obj_5 = GetStockObject16(5);
        SelectObject16(stock_obj_5, hdc_e);
        obj_handle = CreatePen16(0x10000f9, 1, 0);
        SelectObject16(obj_handle, in_hdc_4);
        rect_right = (param_3 + local_4 + 5);
        rect_bottom = (param_2 + local_6 + 5);
        Rectangle16(
            rect_bottom,
            rect_right,
            (param_2 + local_6 + -5),
            (param_3 + local_4 + -5),
            in_hdc_4,
        );
        stock_obj = GetStockObject16(0);
        SelectObject16(stock_obj, rect_right);
        HVar1 = GetStockObject16(6);
        HVar1 = SelectObject16(HVar1, rect_bottom);
        DeleteObject16(HVar1);
    }
    return;
}

pub unsafe fn pass1_1020_3c74(param_1: u16, param_2: u32, param_3: u32) {
    call_pt_in_rect_fn_1020_3c8c(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
    );
    return;
}

pub unsafe fn call_pt_in_rect_fn_1020_3c8c(param_1: &mut Vec<u8>, param_2: u32) {
    draw1::pt_in_rect_1018_1bda((param_1 + 0xfa), param_2);
    return;
}

pub unsafe fn cleanup_fn_1020_3cb8(
    ctx: &mut AppContext,
    param_1: &mut Struct44,
    param_2: u8,
) -> &mut Struct44 {
    let local_struct_1: *mut Struct659;
    let mut local_a: u16;
    let mut local_8: u16;

    if (param_1 == 0x0) {
        local_struct_1 = 0x0;
        param_1._2_2_ = 0;
    } else {
        local_struct_1 = (param_1 + 0xf2);
    }
    local_a = CONCAT22(param_1._2_2_, local_struct_1);
    *local_a = ctx.s_1_1050_389a;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn draw_1020_3da4(ctx: &mut AppContext, param_1: *mut u16, param_2: u32) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: u16;
    let mut HVar5: HGDIOBJ16;
    let puVar6: *mut u16;
    let mut h_dc: HDC16;
    let pp_var7: *mut pass1_struct_1;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut h_dc_00: HDC16;
    let mut local_16: u16;
    let mut local_8: u32;
    let mut local_4: u16;
    let mut offset: u16;

    i_var8 = param_1;
    u_var9 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(i_var8, u_var9, param_2, (param_2 >> 0x10));
    (i_var8 + 0x14) = 0;
    unsafe { *param_1 = 0x408a };
    (i_var8 + 2) = 0x1020;
    pp_var7 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_16, 6));
    h_dc = (pp_var7 >> 0x10);
    (i_var8 + 0x14) = pp_var7;
    (i_var8 + 0x16) = h_dc;
    h_dc_00 = 0;
    ppc_var2 = ((i_var8 + 0x14) + 4);
    ppc_var2(0x1010, (i_var8 + 0x14), h_dc, 0, i_var8, u_var9);
    local_4 = GetDC16((i_var8 + 4));
    i_var4 = SetMapMode16(1, local_4);
    (i_var8 + 0x1e) = i_var4;
    HVar5 = GetStockObject16(0);
    HVar5 = SelectObject16(HVar5, h_dc);
    (i_var8 + 0x18) = HVar5;
    HVar5 = GetStockObject16(6);
    HVar5 = SelectObject16(HVar5, h_dc_00);
    (i_var8 + 0x1a) = HVar5;
    u_var3 = (i_var8 + 0x14);
    pu_var1 = (u_var3 + 0x24);
    puVar6 = &local_4;
    unsafe { ppc_var2 = (*pu_var1 + 8) };
    ppc_var2(offset, pu_var1, (pu_var1 >> 0x10), puVar6);
    (i_var8 + 0x1c) = puVar6;
    u_var3 = (i_var8 + 0x14);
    (u_var3 + 0x4c) = local_4;
    return;
}

pub unsafe fn draw_1020_3e84(ctx: &mut AppContext, in_struct_1: *mut Struct45) {
    let mut h_dc: HDC16;
    let pu_var1: Vec<u8>;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1: *mut Struct45;
    let local_struct_1_hi: *mut Struct45;
    let mut local_4: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.u16_fld_0 = 0x408a;
    local_struct_1.u16_fld_2 = 0x1020;
    pass1_1010_1ea6(
        local_struct_1.field_0x14,
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
    );
    pu_var1 = local_struct_1.field_0x14;
    h_dc = (pu_var1 + 0x4c);
    SelectObject16(local_struct_1.object_0x18, h_dc);
    SelectObject16(local_struct_1.object_0x1a, h_dc);
    h_gdi_obj = SelectPalette16(0, local_struct_1.palette, h_dc);
    DeleteObject16(h_gdi_obj);
    SetMapMode16(local_struct_1.mode, h_dc);
    in_struct_1.u16_fld_0 = ctx.s_0_020_1050_3ab0;
    local_struct_1.u16_fld_2 = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.u16_fld_0 = ctx.s_1_1050_389a;
    local_struct_1.u16_fld_2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn validate_rect_1020_3f12(param_1: u32, param_2: i32) {
    let mut h_wnd: u16;
    let mut rect: u32;
    let mut local_6: u32;

    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    rect = 0x8000e;
    local_6 = 0x1100116;
    InvalidateRect16(0, &rect, h_wnd);
    rect = 0xf10000;
    local_6 = 0x1220030;
    ValidateRect16(&rect, h_wnd);
    rect = 0xf100f5;
    local_6 = 0x1220127;
    ValidateRect16(&rect, h_wnd);
    return;
}

pub unsafe fn draw_1020_3fa0(param_1: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var3 + 4));
    u_var2 = (i_var3 + 0x14);
    local_24 = (u_var2 + 0x4c);
    u_var2 = (i_var3 + 0x14);
    local_28 = (u_var2 + 0x24);
    pp_var1 = (local_28 + 4);
    (**pp_var1)(offset, local_28, (local_28 >> 0x10), 0, &local_24);
    u_var2 = (i_var3 + 0x14);
    local_2e = (u_var2 + 0x44);
    u_var2 = (i_var3 + 0x14);
    local_2c = (u_var2 + 0x40);
    pass1_1008_3e94(
        ((i_var3 + 0x14) + 0x3a),
        CONCAT22(unaff_ss, &local_32),
        CONCAT22(unaff_ss, &local_32 + 2),
    );
    local_36 = local_2c;
    local_38 = 0;
    while (local_38 < local_2e) {
        draw_1020_40ce(local_36, local_32, (local_32 >> 0x10), local_24);
        local_38 = local_38 + 1;
        local_36 = local_36 & 0xffff0000 | (local_36 + 0x18);
    }
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn call_draw_fn_1020_4064(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    draw_1020_3e84(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn polygon_1020_3602(param_1: u16, param_2: u16, param_3: u32, param_4: HDC16) {
    Polygon16(param_1, (param_1 >> 0x10), param_2);
    return;
}

pub unsafe fn cleanup_fn_1020_3898(param_1: *mut Struct656) {
    destroy_win_1020_3b3e(param_1);
    return;
}

pub unsafe fn set_window_pos_1020_38aa(ctx: &mut AppContext, param_1: *mut win_struct_42) {
    let mut hwnd: HWND16;
    let pp_var1: fn();
    let u_var2: u8;
    let mut u_var3: i32;
    let piVar4: *mut u16;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let mut extraout_var;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;

    let pa_var7: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;


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
        draw_1020_3da4(u_var3, pa_var7, param_1, u_var12);
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

pub unsafe fn destroy_win_1020_3b3e(in_struct_1: *mut Struct656) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;
    let local_struct_1: *mut Struct656;
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

pub unsafe fn call_draw_fn_1020_27b0(ctx: &mut AppContext, param_1: *mut Struct650, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut u_var6: u16;
    let mut in_stack_0000fff2: u16;
    let mut local_6: u32;

    u_var6 = param_2;
    draw1::get_dc_1020_921c(CONCAT22(u_var6, param_1), (param_2 >> 0x10));
    &param_1.field_0x14 = 0;
    CONCAT22(u_var6, param_1) = (ctx.s_fem74_wav_1050_2888 + 6);
    param_1.field_0x2 = 0x1020;
    ppVar5 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x2a),
    );
    u_var4 = (ppVar5 >> 0x10);
    param_1.field_0x14 = ppVar5;
    &param_1.field_0x16 = u_var4;
    param_1.field_0x6 = param_1.field_0x14;
    param_1.field_0x8 = u_var4;
    u_var2 = &param_1.field_0x14;
    i_var3 = &param_1.field_0xa;
    pp_var1 = ((u_var2 + 10) + 8);
    (**pp_var1)();
    param_1.field_0x12 = i_var3;
    draw1::draw_1020_9364(CONCAT22(u_var6, param_1));
    return;
}

pub unsafe fn cleanup_fn_1020_2838(ctx: &mut AppContext, param_1: &mut Struct44) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.ptr_a_lo = (ctx.s_fem74_wav_1050_2888 + 6);
    (iVar1 + 2) = 0x1020;
    if ((iVar1 + 0x14) != 0) {
        pass1_1010_1dda(*(iVar1 + 0x14));
    }
    draw1::select_and_delete_palette_1020_92c4(param_1);
    return;
}

pub unsafe fn call_cleanup_fn_1020_2868(
    in_struct_1: *mut Struct376,
    param_2: u8,
) -> *mut Struct376 {
    cleanup_fn_1020_2838(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn polygon_1020_2474(
    param_1: u16,
    param_2: u16,
    count: *mut POINT16,
    device_ctx_handle: HDC16,
) {
    Polygon16(count >> 0x10, device_ctx_handle, 0);
    return;
}

pub unsafe fn win_gui_fn_1020_2488(in_struct_1: *mut Struct647) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let local_struct_1: *mut Struct647;
    let local_struct_1_hi: *mut Struct647;
    let mut u_var3: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_5f808f12c5: *mut Struct60;

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

pub unsafe fn paint_func_1020_15de(ctx: &mut AppContext, in_struct_1: *mut Struct14) {
    let pvVar1: &mut Vec<u8>;
    let bool_result_1: bool;
    let mut u_var2: i32;
    let local_struct_1: *mut Struct14;
    let local_struct_1_hi: *mut Struct14;
    let mut u_var3: u16;
    let mut h_window: u16;
    let mut u_var4: u32;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut Paint_struct: PAINTSTRUCT16;
    let fn_ptr_1: fn();
    let local_struct_2: *mut Struct417;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    BeginPaint16(CONCAT22(h_window, Paint_struct), local_struct_1.h_wnd);
    bool_result_1 = IsIconic16(local_struct_1.h_wnd);
    if (bool_result_1 == 0) {
        u_var3 = 0x1010;
        u_var4 = pass1_1010_454a(local_struct_1.struct_ptr_0x14);
        u_var2 = (u_var4 >> 0x10);
        if ((u_var2 | u_var4) != 0) {
            local_struct_2 = local_struct_1.struct_ptr_0x14;
            u_var3 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            pass1_1008_4480(
                local_struct_1.field_0x18,
                (local_struct_2 & 0xffff0000 | (local_struct_2 + 0x76)),
                u_var4 & 0xffff | u_var2 << 0x10,
            );
        }
        pvVar1 = local_struct_1.field_0x18;
        // WARNING: Load size is inaccurate
        fn_ptr_1 = (*local_struct_1.field_0x18 + 4);
        (**fn_ptr_1)(u_var3, pvVar1, (pvVar1 >> 0x10), 0, 0);
    } else {
        draw_icon_1020_1674(in_struct_1);
    }
    EndPaint16(Paint_struct, h_window);
    return;
}

pub fn draw_icon_1020_1674(ctx: &mut AppContext, in_struct_1: *mut Struct14) {
    let mut icon_handle: HICON16;
    let mut h_brush: HGDIOBJ16;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let local_struct_1: *mut Struct14;
    let local_struct_1_hi: *mut Struct14;
    let mut unaff_ss: HANDLE16;
    let uStack34;
    let mut local_1c: u16;
    let mut rect16_2: RECT16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut rect16_1: RECT16;
    let mut local_6: u16;
    let mut local_4: u16;
    // fn_ptr_1: *mut Vec<u8>;
    let local_struct_2: *mut Struct417;
    let mut u_var1: u16;
    let mut fn_ptr_1: fn();

    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
        local_struct_1_hi = (in_struct_1 >> 0x10);
        local_struct_1 = in_struct_1;
        fn_ptr_1 = (local_struct_1.struct_ptr_0x14 + 0x2c);
        icon_handle = (**fn_ptr_1)();
        if (icon_handle != 0) {
            h_brush = GetStockObject16(4);
            GetClientRect16(CONCAT22(unaff_ss, &rect16_1), local_struct_1.h_wnd);
            rect16_2.left = 0;
            rect16_2.top = 0;
            i_var2 = (rect16_1.right - rect16_1.left) + -1;
            i_var3 = (rect16_1.bottom - rect16_1.top) + -1;
            local_struct_2 = local_struct_1.struct_ptr_0x14;
            u_var1 = (local_struct_2 + 0x7c);
            rect16_2.right = i_var2;
            rect16_2.bottom = i_var3;
            FillRect16(h_brush, &rect16_2, unaff_ss);
            DrawIcon16(
                icon_handle,
                CONCAT214(
                    rect16_2.left,
                    CONCAT212(u_var1, CONCAT66(uStack34, CONCAT24(u_var1, 0x20002))),
                ),
                CONCAT214(
                    rect16_1.right,
                    CONCAT212(
                        rect16_1.top,
                        CONCAT210(
                            rect16_1.left,
                            CONCAT28(
                                i_var2,
                                CONCAT26(
                                    i_var3,
                                    CONCAT24(
                                        rect16_2.bottom,
                                        CONCAT22(rect16_2.right, rect16_2.top),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                rect16_1.bottom,
            );
        }
    }
    return;
}

pub unsafe fn call_palette_fn_1020_170a(
    in_struct_1: *mut Struct376,
    param_2: u8,
) -> *mut Struct376 {
    draw1::palette_func_1020_150e(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn process_struct_1020_1738(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct68,
    param_2: u16,
    param_2_00: u32,
) {
    let local_struct_1: *mut Struct68;
    let local_struct_1_hi: *mut Struct68;

    process_struct_1040_7728(
        in_struct_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0xfcd,
        *(param_2_00 + 8),
    );
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x8e = 0;
    local_struct_1.field_0x92 = 0;
    local_struct_1.field_0x96 = 0;
    in_struct_1.field_0x0 = (ctx.s_512_bmp_1050_1e77 + 3);
    local_struct_1.field_0x2 = 0x1020;
    return;
}

pub fn call_destroy_win_fn_1020_1780(param_1: *mut Struct53) {
    let fn_ptr_1: fn();

    fn_ptr_1 = (param_1 + 0x6c);
    (**fn_ptr_1)();
    destroy_window_1040_8212(param_1);
    return;
}

pub unsafe fn win_fn_1020_179c(ctx: &mut AppContext, param_1: *mut Struct20) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut i_var5: u16;
    let puVar6: *mut u16;
    let BVar7: bool;
    let mut hwnd: HWND16;
    let struct_a: *mut Struct199;


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
        process_struct_1020_1eea(CONCAT22(local_4c, u_var4), param_1, (i_var8 + 6));
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
    BVar7 = pass1_1020_1da8(param_1);
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

pub unsafe fn pass1_1020_1b68(param_1: *mut Struct409) {
    let pu_var1: *mut u32;
    let pvVar2: &mut Vec<u8>;
    let local_struct_1: *mut Struct641;
    let local_struct_1_hi: *mut Struct409;
    let fn_ptr_1: fn();

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    pu_var1 = local_struct_1.field_0x92;
    pvVar2 = local_struct_1.field_0x94;
    if ((pvVar2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)();
        }
    }
    &local_struct_1.field_0x92 = 0;
    pass1_1010_4f48(local_struct_1.field_0x8e);
    local_struct_1.field_0x8e = 0;
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
    puVar8 = call_pt_in_rect_fn_1020_1d8e(
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

pub fn call_pt_in_rect_fn_1020_1d8e(param_1: &mut Vec<u8>, param_2: u32) {
    draw1::pt_in_rect_1010_4e08((param_1 + 0x8e), param_2, (param_2 >> 0x10));
    return;
}

pub unsafe fn pass1_1020_1da8(in_struct_1: *mut Struct643) -> bool {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let local_struct_1: *mut Struct643;
    let local_struct_1_hi: *mut Struct643;
    let mut temp_5fb951b2a7: u32;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    temp_5fb951b2a7 = local_struct_1.field_0x8e;
    if ((temp_5fb951b2a7 + 0x30) == 1) {
        return 1;
    }
    u_var1 = local_struct_1.field_0x8e;
    if (((u_var1 + 0x30) < 3)
        && (
            i_var2 = pass1_1010_4df0(local_struct_1.field_0x8e),
            i_var2 == 0,
        ))
    {
        return 1;
    }
    return 0;
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

pub unsafe fn call_cleanup_fn_1020_1e54(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    win_cleanup_func_1040_782c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn process_struct_1020_1eea(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct644,
    param_2: u32,
    param_3: u16,
) {
    let pp_var1: fn();
    let mut local_AX_92: u16;
    let mut local_DX_92: u16;
    let local_bx_4: *mut Struct644;
    let mut local_es_4: u16;
    let ppVar2: *mut pass1_struct_1;
    let string_1: String;

    local_es_4 = (in_struct_1 >> 0x10);
    local_bx_4 = in_struct_1;
    in_struct_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    in_struct_1 = (ctx.s_18_2_1050_3aa5 + 3);
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = param_3;
    in_struct_1 = ctx.s_0_020_1050_3ab0;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_bx_4.field_0x6 = 0;
    local_bx_4.field_0xa = param_2;
    in_struct_1 = (ctx.s_218_bmp_1050_2516 + 2);
    local_bx_4.field_0x2 = 0x1020;
    string_1 = CONCAT22(string_1._2_2_, 0x39);
    ppVar2 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, string_1);
    local_DX_92 = (ppVar2 >> 0x10);
    local_bx_4.field_0x6 = ppVar2;
    &local_bx_4.field_0x8 = local_DX_92;
    pp_var1 = (&local_bx_4.field_0x6 + 4);
    (**pp_var1)(
        0x1010,
        local_bx_4.field_0x6,
        local_DX_92,
        0,
        in_struct_1,
        string_1._2_2_,
    );
    return;
}

pub unsafe fn pass1_1020_1f74(ctx: &mut AppContext, param_1: *mut Struct376) {
    let local_struct_1: *mut Struct376;
    let local_struct_1_hi: *mut Struct376;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.offset = (ctx.s_218_bmp_1050_2516 + 2);
    local_struct_1.segment = 0x1020;
    pass1_1010_1ea6(
        *&local_struct_1.u16_x6,
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
    );
    param_1.offset = ctx.s_0_020_1050_3ab0;
    local_struct_1.segment = &ctx.PTR_LOOP_1050_1008;
    param_1.offset = ctx.s_1_1050_389a;
    1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn invalidate_window_rect_1020_1fb2(param_1: u32, param_2: i32) {
    let mut unaff_cs: u16;
    let mut unaff_ss: HWND16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 1) {
        (param_1 + 6) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
    local_16 = 0;
    local_6 = 0x46;
    local_14 = 0x46;
    local_12 = local_a - local_e;
    local_4 = 0x5f;
    local_10 = 0x5f;
    InvalidateRect16(0, &local_16, unaff_ss);
    return;
}

pub unsafe fn draw_1020_2020(ctx: &mut AppContext, param_1: *mut Struct647) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut h_var4: HDC16;
    let pu_var5: *mut u16;
    let mut h_var6: HPEN16;
    let mut obj_handle: HGDIOBJ16;
    let mut HVar7: HBRUSH16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut HVar8: HGDIOBJ16;
    let mut h_gdi_obj: HPALETTE16;

    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_ss: HWND16;
    let mut u_var13: u32;
    let mut u_var14: u32;
    let mut uVar15: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 6];
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    u_var11 = (param_1 >> 0x10);
    u_var10 = param_1;
    h_var4 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (u_var10 + 4));
    local_24 = h_var4;
    pass1_1010_4c2c((u_var10 + 6));
    local_28 = CONCAT22(ctx.dx_reg, h_var4);
    pu_var5 = &local_24;
    pp_var1 = (*local_28 + 8);
    (**pp_var1)(0x1010, h_var4, ctx.dx_reg, pu_var5, unaff_ss);
    (u_var10 + 0x10) = pu_var5;
    u_var2 = (u_var10 + 6);
    local_2a = (u_var2 + 0x30);
    u_var2 = (u_var10 + 6);
    local_2e = (u_var2 + 0x12);
    local_32 = 0x140000;
    u_var12 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    zero_list_1008_3e38(CONCAT22(unaff_ss, local_38));
    local_3a = 0;
    while (local_3a < local_2a) {
        u_var13 = process_struct_1008_4772((local_3a * 4 + local_2e));
        u_var9 = (u_var13 >> 0x10);
        i_var3 = u_var13;
        pass1_1020_2286(
            u_var10,
            u_var11,
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_32)),
            (i_var3 + 8),
        );
        pass1_1008_3e76(
            CONCAT22(unaff_ss, local_38),
            0,
            local_32,
            (local_32 >> 0x10),
        );
        pass1_1008_4480(
            local_28,
            CONCAT22(unaff_ss, local_38),
            (local_2e + local_3a * 4),
        );
        u_var12 = 0x1010;
        process_struct_1010_4d5c(
            (u_var10 + 6),
            (i_var3 + 8) + local_32,
            (i_var3 + 4) + local_30,
            local_32,
            local_30,
            local_3a,
        );
        local_32 =
            local_32 & 0xffff | (local_30 + (-(local_3a == 0) & 5) + 0x14 + (i_var3 + 4)) << 0x10;
        local_3a = local_3a + 1;
    }
    pp_var1 = (*local_28 + 4);
    (**pp_var1)(u_var12, local_28, (local_28 >> 0x10), 0, 0, 0xdc);
    h_var6 = CreatePen16(0x1000025, 1, 0);
    obj_handle = SelectObject16(h_var6, local_24);
    HVar7 = CreateSolidBrush16(0x1000025);
    obj_handle_00 = SelectObject16(HVar7, local_24);
    HVar8 = obj_handle_00;
    draw_1020_229c(u_var10, (param_1 >> 0x10), local_24);
    pass1_1010_4df0((u_var10 + 6));
    if (HVar8 == 0) {
        HVar8 = SelectObject16(obj_handle, local_24);
        DeleteObject16(HVar8);
        HVar8 = SelectObject16(obj_handle_00, local_24);
        DeleteObject16(HVar8);
        HVar7 = CreateSolidBrush16(0xff);
        h_var6 = CreatePen16(0xff, 1, 0);
        SelectObject16(HVar7, local_24);
        SelectObject16(h_var6, local_24);
    }
    uVar15 = local_24;
    u_var14 = pass1_1010_4dc8((u_var10 + 6));
    draw_fn_1020_239c(param_1, u_var14, uVar15);
    u_var2 = (u_var10 + 6);
    if ((u_var2 + 0x2c) != 0) {
        win_gui_fn_1020_2488(param_1);
    }
    h_gdi_obj = SelectPalette16(0, (u_var10 + 0x10), local_24);
    DeleteObject16(h_gdi_obj);
    HVar8 = SelectObject16(obj_handle, local_24);
    DeleteObject16(HVar8);
    HVar8 = SelectObject16(obj_handle_00, local_24);
    DeleteObject16(HVar8);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub fn pass1_1020_2286(param_1: u16, param_2: u16, param_1_00: *mut i32, param_2_00: i32) {
    unsafe {
        *param_1_00 = 100 - param_2_00 >> 1;
    }
    return;
}

pub fn draw_1020_229c(param_1: u32, param_2: HDC16) {
    let mut iVar1: i32;
    let pi_var2: *mut u16;
    let mut u_var3: u32;
    let mut x: i32;
    let mut i_var4: i32;
    let pi_var5: *mut i32;
    let mut u_var6: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    u_var3 = (param_1 + 6);
    iVar1 = (u_var3 + 0x30);
    u_var3 = (param_1 + 6);
    pi_var2 = (u_var3 + 0x1a);
    let pi_var2_val = unsafe { *pi_var2 };
    MoveTo16(5, pi_var2_val, param_2);
    u_var6 = (pi_var2 >> 0x10);
    i_var4 = pi_var2;
    LineTo16(5, (i_var4 + iVar1 * 8 + -4), param_2);
    local_a = 0;
    while (local_a < iVar1) {
        pi_var5 = (local_a * 8 + i_var4);
        let pi_var_5_val = unsafe { *pi_var5 };
        x = (pi_var5[2] - pi_var_5_val >> 1) + pi_var_5_val;
        MoveTo16(5, x, param_2);
        LineTo16(10, x, param_2);
        local_a = local_a + 1;
    }
    let pi_var_2_val = unsafe { *pi_var2 };
    MoveTo16(0x5f, pi_var_2_val, param_2);
    LineTo16(0x5f, (i_var4 + iVar1 * 8 + -4), param_2);
    local_a = 0;
    while (local_a < iVar1) {
        pi_var5 = (local_a * 8 + i_var4);
        let pi_var5_val = unsafe { *pi_var5 };
        MoveTo16(0x5f, (pi_var5[2] - pi_var5_val >> 1) + pi_var5_val, param_2);
        LineTo16(0x5a, param_2, param_2);
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn draw_fn_1020_239c(ctx: &mut AppContext, param_1: u32, param_2: u32, param_3: u16) {
    let mut u_var1: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 6];
    let mut local_4: u16;

    if (param_2 != 0) {
        pass1_1008_3e54(
            CONCAT22(ctx.stack_seg_reg, local_a),
            0,
            0x57,
            ((param_2 + 4) - param_2 >> 1) + param_2,
        );
        u_var1 = (param_1 >> 0x10);
        ctx.dx_ax_reg = pass1_1020_23f2(param_1, u_var1, CONCAT22(ctx.stack_seg_reg, local_a));
        polygon_1020_2474(
            param_1,
            u_var1,
            CONCAT22(ctx.dx_ax_reg, 3),
            (ctx.dx_ax_reg >> 0x10),
        );
    }
    return;
}

pub unsafe fn pass1_1020_23f2(param_1: u16, param_2: u16, param_1_00: *mut u16) {
    let mut u_var1: u16;
    let local_struct_1: *mut Struct645;
    let mut unaff_ss: u16;
    let mut u_var2: u32;
    let local_12: *mut Struct646;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass1_1008_3e94(
        param_1_00,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    process_struct_1000_179c(0xc, (u_var2 >> 0x10));
    u_var1 = (u_var2 >> 0x10);
    local_12 = 0x0;
    while (local_12 < 3) {
        local_struct_1 = (local_12 * 4);
        (local_struct_1 + u_var2) = (local_struct_1 + 0x4270) + local_4;
        (local_struct_1 + u_var2 + 2) = (local_struct_1 + 0x4272) + local_6;
        local_12 = &local_12.field_0x1;
    }
    return u_var2;
}
