use std::intrinsics::offset;

use crate::app_context::AppContext;
use crate::draw::{draw1, draw2, drawing_context, lines, paint, palette, polygon, rect, text};
use crate::draw::polygon::polygon_1020_3602;
use crate::err_ops::error_check_1000_17ce;
use crate::file_ops::close::close_file_1008_496c;
use crate::file_ops::read::read_from_file_1008_49e8;
use crate::list_funcs::modify_list_1010_2b50;
use crate::mem_funcs::mem_ops_1::{get_fn_ptr_at_address, get_string_from_address};
use crate::other_funcs::mixed_fn_1010_830a;
use crate::other_funcs::{modify_list_1008_3f62, zero_list_1008_3e38};
use crate::pass::pass13_funcs::pass1_1008_9f48;
use crate::pass::pass14_funcs::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3f32, pass1_1008_405c, pass1_1008_4480, pass1_1008_4d72, pass1_1008_4d84, pass1_1008_5236};
use crate::pass::pass15_funcs::pass1_1020_3540;
use crate::pass::pass20_funcs::pass1_1018_0a50;
use crate::pass::pass7_funcs::{draw_1018_6444, pass1_1018_0d9a, pass1_1018_1054, pass1_1018_108c, pass1_1018_1320, pass1_1018_15f6, pass1_1018_2d84, pass1_1018_2e5e, pass1_1018_642e, pass1_1018_6544};
use crate::pass::pass8_funcs::{pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_2b66, pass1_1010_2b78, pass1_1010_2b98, pass1_1010_4c2c, pass1_1010_4dc8, pass1_1010_4df0, pass1_1010_4f48, process_struct_1010_20ba};
use crate::pass::pass_funcs::pass1_fn_1000_484c;
use crate::sound_funcs::{mci_send_cmd_1008_5c5c, mci_send_command_1008_5c9e};
use crate::string_ops::misc::{copy_string_1000_3d3e, get_string_index_1000_3da4};
use crate::struct_ops::struct_ops_2::{pass1_1008_5068, process_struct_1000_179c, process_struct_1008_4772, process_struct_1008_47cc, process_struct_1008_4834, process_struct_1008_48fe, process_struct_1008_4c58, process_struct_1010_4d5c, set_struct_1008_4016};
use crate::struct_ops::struct_ops_2::process_struct_1040_9618;
use crate::structs::prog_structs_12::{Struct102, Struct409};
use crate::structs::prog_structs_1::Struct104;
use crate::structs::prog_structs_2::{Struct199, Struct7};
use crate::structs::prog_structs_20::{Struct133, Struct30};
use crate::structs::prog_structs_23::Struct134;
use crate::structs::prog_structs_24::{Struct103, Struct129};
use crate::structs::prog_structs_28::Struct357;
use crate::structs::prog_structs_29::Struct36;
use crate::structs::prog_structs_30::{Struct14, Struct35, Struct417};
use crate::structs::prog_structs_31::{Struct28, Struct29, Struct34, Struct45};
use crate::structs::prog_structs_3::Struct661;
use crate::structs::prog_structs_4::{Struct510, Struct650, Struct653};
use crate::structs::prog_structs_6::{Struct132, Struct622, Struct674};
use crate::structs::prog_structs_7::{Struct135, Struct31, Struct376, Struct44, Struct618, Struct631, Struct632};
use crate::structs::prog_structs_8::{Struct641, Struct643, Struct645, Struct646, Struct647};
use crate::structs::prog_structs_9::{Struct594, Struct604, Struct630};
use crate::sys_ops::get_sys_metrics_1020_7c1a;
use crate::sys_structs::{PAINTSTRUCT16, RECT16};
use crate::typedefs::{COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HGDIOBJ16, HICON16, HPALETTE16, HPEN16, HWND16};
use crate::ui_ops::misc::{pass1_1038_af40, win_gui_fn_1010_8170};
use crate::ui_ops::window;
use crate::ui_ops::window::{destroy_win_1008_628e, set_window_text_1018_6630};
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT210, CONCAT212, CONCAT214, CONCAT22, CONCAT24, CONCAT26, CONCAT28, CONCAT66, SUB42, ZEXT24};
use crate::winapi::{BeginPaint16, CreateDC16, CreatePen16, CreateSolidBrush16, DeleteDC16, DeleteObject16, DrawIcon16, DrawText16, Ellipse16, EndPaint16, FillRect16, FrameRect16, GetClientRect16, GetDC16, GetDlgCtrlID16, GetProp16, GetStockObject16, GetSystemMetrics16, GetTextExtent16, GetWindowDC16, GetWindowLong16, GetWindowRect16, GrayString16, InvalidateRect16, IsIconic16, LineTo16, lstrlen16, MoveTo16, MoveToEx16, Polygon16, PostMessage16, RealizePalette16, Rectangle16, ReleaseDC16, SelectObject16, SelectPalette16, SetBkColor16, SetCapture16, SetCursor16, SetMapMode16, SetTextColor16, TextOut16};

pub unsafe fn draw_1018_623e(ctx: &mut AppContext, param_1: &mut Struct604) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut h_var4: HDC16;
    let pu_var5: &u16;
    let mut obj_handle: HPEN16;
    let mut h_var6: HGDIOBJ16;
    let mut obj_handle_00: HBRUSH16;
    let mut obj_handle_01: HGDIOBJ16;
    let mut h_gdi_obj: HPALETTE16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: HWND16;
    let mut u_var11: u32;
    let mut u_var12: u32;
    let mut u_var13: u16;
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
    let mut local_22: PAINTSTRUCT16 = PAINTSTRUCT16::new();

    let mut hdc_var4: HDC16 = BeginPaint16(param_1.field_0x4, &local_22);
    let mut hdc_var24 = hdc_var4;
    pass1_1010_4c2c(param_1.field_0x6);
    let mut local_28 = CONCAT22(ctx.dx_reg, hdc_var4);
    let mut pu_var5 = &hdc_var24;
    let pp_var1 = get_fn_ptr_at_address(local_28 + 8);
    pp_var1(0x1010, hdc_var4, ctx.dx_reg, pu_var5, unaff_ss);
    (param_1 + 0x10) = pu_var5;
    let mut u_var2 = (param_1 + 6);
    let mut local_2a = (u_var2 + 0x30);
    u_var2 = (param_1 + 6);
    let mut local_2e = (u_var2 + 0x12);
    let mut local_32 = 0x140000;
    let mut u_var10 = &ctx.PTR_LOOP_1050_1008;
    zero_list_1008_3e38(CONCAT22(unaff_ss, local_38));
    let mut local_3a = 0;
    while local_3a < local_2a {
        u_var11 = process_struct_1008_4772((local_3a * 4 + local_2e));
        //// _var7 = (u_var11  >> 0x10);
        i_var3 = u_var11;
        pass1_1018_642e(
            param_1,
            u_var9,
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
        u_var10 = 0x1010;
        process_struct_1010_4d5c(
            (u_var8 + 6),
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
    (**pp_var1)(u_var10, local_28, (local_28 >> 0x10), 0, 0, 0xdc);
    obj_handle = CreatePen16(0x1000025, 1, 0);
    h_var6 = SelectObject16(obj_handle, hdc_var24);
    obj_handle_00 = CreateSolidBrush16(0x1000025);
    obj_handle_01 = SelectObject16(obj_handle_00, hdc_var24);
    draw_1018_6444(u_var8, (param_1 >> 0x10), hdc_var24);
    u_var13 = hdc_var24;
    u_var12 = pass1_1010_4dc8((u_var8 + 6));
    pass1_1018_6544(param_1, u_var12, u_var13);
    set_window_text_1018_6630(param_1);
    h_gdi_obj = SelectPalette16(0, (u_var8 + 0x10), hdc_var24);
    DeleteObject16(h_gdi_obj);
    h_var6 = SelectObject16(h_var6, hdc_var24);
    DeleteObject16(h_var6);
    h_var6 = SelectObject16(obj_handle_01, hdc_var24);
    DeleteObject16(h_var6);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_1018_5d6c(ctx: &mut AppContext, param_1: u32) {
    let pu_var1: &mut  u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: HWND16;
    let mut u_var6: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_22: PAINTSTRUCT16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var4 + 4));
    u_var3 = (i_var4 + 0x14);
    pu_var1 = (u_var3 + 10);
    u_var6 = pass1_1008_9f48((i_var4 + 0x14));
    pass1_1008_5236((i_var4 + 0x18));
    pass1_1008_4480(pu_var1, (param_1 & 0xffff0000 | (i_var4 + 0x1c)), u_var6);
    unsafe { ppc_var2 = (*pu_var1 + 4) };
    ppc_var2(
        &ctx.PTR_LOOP_1050_1008,
        pu_var1,
        (pu_var1 >> 0x10),
        0,
        param_1 & 0xffff0000 | (i_var4 + 10),
    );
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_func_1018_4f18(ctx: &mut AppContext, param_1: u32, param_2: u16) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u16;
    let mut i32_var6: i32;
    let mut in_eax: u32;

    let ctx.dx_reg: &mut  Struct199;
    let pa_var7: &mut  Struct199;
    let ctx.dx_reg: &mut  Struct199;

    let ctx.dx_reg: &mut  Struct199;
    let mut extraout_dx_04: u16;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, param_2);
    let local_6 = (in_eax & 0xffff | ctx.dx_reg << 0x10);
    ppc_var3 = (*local_6 + 0x14);
    (**ppc_var3)(0x1010, in_eax, ctx.dx_reg);
    pu_var4 = in_eax;
    let local_a = in_eax & 0xffff | ZEXT24(ctx.dx_reg) << 0x10;
  // u_var9 = (param_1  >> 0x10);
    i_var8 = param_1;
    pa_var7 = ctx.dx_reg;
    if ((i_var8 + 0xe) != 0) {
        u_var1 = (i_var8 + 0x10);
        pu_var4 = (i_var8 + 0xe);
        pa_var7 = (u_var1 | pu_var4);
        if (pa_var7 != 0x0) {
            ppc_var3 = *pu_var4;
            (**ppc_var3)(0x10, pu_var4, u_var1, 1);

            pa_var7 = ctx.dx_reg;
        }
    }
    process_struct_1000_179c(0x14, pa_var7);
    if ((pa_var7 | pu_var4) == 0) {
        pu_var4 = 0x0;
        u_var10 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(pa_var7, pu_var4));
        u_var10 = ctx.dx_reg;
    }
    (i_var8 + 0xe) = pu_var4;
    (i_var8 + 0x10) = u_var10;
    pass1_1008_4d84((i_var8 + 0xe), local_a);
    pu_var5 = &local_12;
    pa_var7 = ctx.dx_reg;
    GetClientRect16(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, pu_var5)),
        ctx.g_h_window,
    );
    u_var10 = 0x1000;
    process_struct_1000_179c(0x1e, pa_var7);
    if ((pa_var7 | pu_var5) == 0) {
        (i_var8 + 10) = 0;
    } else {
        i32_var6 = (local_c - local_10) + 1;
        u_var2 = (i_var8 + 0xe);
        u_var10 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_1008_405c(
            pu_var5,
            pa_var7,
            u_var2,
            (u_var2 >> 0x10),
            i32_var6,
            (local_e - local_12) + 1,
        );
        (i_var8 + 10) = i32_var6;
        (i_var8 + 0xc) = extraout_dx_04;
    }
    if (local_6 != 0x0) {
        ppc_var3 = *local_6;
        (**ppc_var3)(u_var10, local_6, (local_6 >> 0x10), 1);
    }
    return;
}

pub unsafe fn draw_1010_47ae(param_1: u32) {
    let local_struct_1: &mut  Struct30;
    local_struct_1 = 0x0;
    while {
        draw_1010_47d0(param_1, local_struct_1);
        local_struct_1 = local_struct_1.field_0x1;
        local_struct_1 < 0x10
    } {}
}

pub unsafe fn draw_1010_47d0(ctx: &mut AppContext, param_1: u32, param_2: &mut  Struct30) {
    let pu_var1: &mut  u16;
    let pu_var2: &mut  u32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let ppc_var5: fn();
    let pa_var6: &mut  Struct30;
    let h_palette: &mut  u16;
    let mut obj_handle: HGDIOBJ16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut i_var7: i32;
    let pu_var8: &mut  u16;
    let mut h_gdi_obj: HPALETTE16;
    let mut extraout_d_x: u16;
    let local_b_x_49: &mut  Struct29;
    let mut i_var9: i32;
    let local_s_i_120: &mut  Struct31;
    let local_s_i_293: &mut  Struct28;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_s_s: u16;
    let mut u_var12: u32;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let mut u_var17: u16;
    let mut u_var18: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut h_dc: u16;
    let mut dev_mode: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut stock_object: u16;
    let mut pen: u16;

    pen = CreatePen16(0x77d7fb, 1, 0);
    stock_object = GetStockObject16(5);
    let offset = 0;
    local_e = 0;
    local_c = 0;
    local_a = 1;
    local_8 = 1;
  // u_var10 = (param_1  >> 0x10);
    local_b_x_49 = param_1;
    pu_var2 = (&local_b_x_49.field_0x26 + param_2 * 4);
    u_var3 = (&local_b_x_49.field_0x26 + param_2 * 4 + 2);
    if ((u_var3 | pu_var2) != 0) {
        unsafe {
            ppc_var5 = *pu_var2;
            (**ppc_var5)(offset, pu_var2, u_var3, 1);
        }
    }
    pa_var6 = param_2 + 1;
    win_gui_fn_1010_8170(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        pa_var6,
    );
    local_s_i_120 = (param_2 * 4);
    (local_b_x_49 + local_s_i_120 + 0x26) = pa_var6;
    (local_b_x_49 + local_s_i_120 + 0x28) = extraout_d_x;
    u_var18 = 0x1380;
    u_var11 = 0;
    u_var17 = 0;
    u_var13 = 0;
    u_var14 = 0;
    u_var15 = 0;
    u_var16 = 0;
    u_var12 = process_struct_1008_4772((local_s_i_120 + &local_b_x_49.field_0x26));
  // local_10 = (u_var12  >> 0x10);
    dev_mode = u_var12;
    h_dc = CreateDC16(
        dev_mode,
        CONCAT13(u_var14, CONCAT12(u_var13, local_10)),
        CONCAT22(u_var11, CONCAT11(u_var16, u_var15)),
        CONCAT22(u_var18, u_var17),
    );
    u_var4 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    h_palette = &h_dc;
    palette::realize_palette_1008_4e08(u_var4, (u_var4 >> 0x10), h_palette, unaff_s_s);
    obj_handle = SelectObject16(pen, h_dc);
    obj_handle_00 = SelectObject16(stock_object, h_dc);
    local_20 = 0;
    while (true) {
        pu_var1 = &local_b_x_49.field_0x74;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_20 || pu_var1_val < local_20) {
            break;
        }
        i_var7 = (&param_2.field_0x0 + local_20 * 0x10) * 8;
        pu_var8 = &local_e;
        pass1_fn_1000_484c(
            CONCAT22(unaff_s_s, pu_var8),
            CONCAT22(
                (&local_b_x_49.field_0x70 + 2),
                i_var7 + &local_b_x_49.field_0x70,
            ),
            8,
        );
        if (pu_var8 != 0x0) {
            u_var12 = local_b_x_49.field_0x70;
          // u_var11 = (u_var12  >> 0x10);
            i_var9 = u_var12;
            local_s_i_293 = (i_var7 + i_var9);
            Rectangle16(
                local_s_i_293.field_0x6,
                local_s_i_293.field_0x4,
                local_s_i_293.field_0x2,
                (i_var9 + i_var7),
                h_dc,
            );
        }
        local_20 = local_20 + 1;
    }
    h_gdi_obj = SelectPalette16(0, h_palette, h_dc);
    DeleteObject16(h_gdi_obj);
    SelectObject16(obj_handle, h_dc);
    SelectObject16(obj_handle_00, h_dc);
    DeleteDC16(h_dc);
    DeleteObject16(pen);
    return;
}

pub unsafe fn draw_1040_b372(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: HWND16,
    param_3: i32,
    param_4: HDC16,
) {
    let mut iVar1: i32;
    let mut HVar2: HBRUSH16;
    let mut i_var3;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;

  // u_var4 = (param_1  >> 0x10);
    if ((param_1 + 0x8e) == 0) {
        HVar2 = CreateSolidBrush16(0);
        (param_1 + 0x8e) = HVar2;
    }
    if (ctx._PTR_LOOP_1050_5efa == 0) {
        u_var5 = pass1_1008_4d72((ctx._PTR_LOOP_1050_4230 + 0xe));
      // u_var4 = (u_var5  >> 0x10);
        iVar1 = u_var5;
        ctx._PTR_LOOP_1050_5efa =
            CONCAT12(*(iVar1 + 0x94), CONCAT11(*(iVar1 + 0x95), *(iVar1 + 0x96)));
    }
    if (param_3 < 4) {
        // LAB_1040_b3ea:
        i_var3 = GetDlgCtrlID16(param_2);
        if (i_var3 == 0x14c) {
            u_var4 = 0xffff;
            u_var6 = 0;
            // goto LAB_1040_b41a;
        }
        if (i_var3 == 0x175) {
            u_var4 = 0xff;
            u_var6 = 0;
            // goto LAB_1040_b41a;
        }
    } else {
        if (param_3 != 4) {
            if ((param_3 == 4) || (1 < param_3 - 5)) {
                return;
            }
            // goto LAB_1040_b3ea;
        }
    }
    u_var4 = ctx._PTR_LOOP_1050_5efa;
  // u_var6 = (ctx._PTR_LOOP_1050_5efa  >> 0x10);
    // LAB_1040_b41a:
    SetTextColor16(CONCAT22(u_var6, u_var4), param_4);
    SetBkColor16(0x1000000, param_4);
    return;
}

pub fn draw_1040_c74c(ctx: &mut AppContext, param_1: &mut  u32, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut h_gdi_obj: HGDIOBJ16;
    let mut h_gdi_obj_00: HPALETTE16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: u8;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let offset;

  // u_var4 = (ctx._PTR_LOOP_1050_4230  >> 0x10);
    local_6 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    local_8 = palette::realize_palette_1008_4e08(
        local_6,
        (ctx._PTR_LOOP_1050_4230 + 0x10),
        &param_2 + 2,
        unaff_ss,
    );
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x46) = 1;
    local_a = GetStockObject16(4);
    local_c = CreatePen16(0x1000002, 1, 0);
    local_e = SelectObject16(local_a, param_2);
    local_10 = SelectObject16(local_c, param_2);
    Rectangle16((i_var3 + 0x24), (i_var3 + 0x22), 0, 0, param_2);
    MoveTo16(0, (i_var3 + 0x36) * 2 + (i_var3 + 0x2a), param_2);
    LineTo16(
        (i_var3 + 0x24),
        (i_var3 + 0x36) * 2 + (i_var3 + 0x2a),
        param_2,
    );
    SelectObject16(local_e, param_2);
    h_gdi_obj = SelectObject16(local_10, param_2);
    DeleteObject16(h_gdi_obj);
    u_var2 = unsafe { *param_1 };
    pp_var1 = (u_var2 + 0x10);
    (**pp_var1)(offset, i_var3, u_var4, param_2);
    pp_var1 = (u_var2 + 0x14);
    (**pp_var1)(offset, i_var3, (param_1 >> 0x10), param_2);
    (i_var3 + 0x46) = 0;
    h_gdi_obj_00 = SelectPalette16(0, local_8, param_2);
    DeleteObject16(h_gdi_obj_00);
    return;
}

pub unsafe fn draw_1040_c226(ctx: &mut AppContext, struct_param_1: &mut Struct135) {
    let mut u_var1: Struct137;
    let mut hpen_var1: HPEN16;
    let mut hgdi_obj_var3: HGDIOBJ16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut h_dc: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut struct_var_32: RECT16 = RECT16::new();
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut hbrush_var_22: HBRUSH16;
    let mut hdc_var_24: HDC16;
    let mut struct_var_26: PAINTSTRUCT16;

    hdc_var_24 = BeginPaint16(
        struct_param_1.field_0x4,
        &struct_var_26);
    hbrush_var_22 = CreateSolidBrush16(0x8000);
    GetClientRect16(
        struct_param_1.field_0x4,
        &mut struct_var_32);
    u_var1 = (struct_param_1.field_0x6);
    local_28 = (u_var1 + 0x1a);
    u_var1 = (struct_param_1 + 6);
    local_2a = (u_var1 + 0x1c);
    local_30 = local_30 + 2;
    struct_var_32 = local_28 - 10;
    local_2e = local_2e - 2;
    local_2c = local_2c - 2;
    FrameRect16(hbrush_var_22, &struct_var_32, unaff_ss);
    DeleteObject16(hbrush_var_22);
    h_dc = hdc_var_24;
    hpen_var1 = CreatePen16(0x8080, 2, 0);
    hgdi_obj_var3 = SelectObject16(hpen_var1, h_dc);
    lines::draw_lines_1040_c302(struct_param_1, u_var4, hdc_var_24);
    lines::draw_lines_1040_c38e(struct_param_1, hdc_var_24);
    hgdi_obj_var3 = SelectObject16(hgdi_obj_var3, hdc_var_24);
    DeleteObject16(hgdi_obj_var3);
    EndPaint16(&struct_var_26, unaff_ss);
    return;
}

pub fn draw_1008_8288(ctx: &mut AppContext, param_1: HWND16, param_2: u32) {
    let mut HVar1: HGDIOBJ16;
    let mut obj_handle: HGDIOBJ16;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: PAINTSTRUCT16;
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
    let local_56: u16;
    let mut offset: u16;
    let mut _local_4c: u16;

    local_4a = BeginPaint16(param_1, CONCAT22(unaff_ss, &local_3c));
    local_4 = 0;
    local_4c = CreatePen16(ctx._PTR_LOOP_1050_0368, 1, 0);
    local_4e = CreatePen16(ctx.u32_1050_0364, 1, 0);
    local_50 = CreateSolidBrush16(ctx.u32_1050_0364);
    GetClientRect16(param_1, CONCAT22(unaff_ss, &local_58));
    local_3e = local_54;
    local_40 = local_52;
    local_42 = local_54 >> 1;
    local_44 = local_52 >> 1;
    local_46 = local_54 >> 2;
    local_48 = local_52 >> 2;
    HVar1 = GetStockObject16(7);
    HVar1 = SelectObject16(HVar1, local_5c);
    obj_handle = GetStockObject16(4);
    SelectObject16(obj_handle, HVar1);
    Rectangle16(local_52, local_54, local_56, local_58, local_4a);
    local_58 = local_4a;
    MoveTo16(local_44, 0, local_4a);
    local_56 = CONCAT22(local_54, local_4a);
    local_58 = local_3e;
    local_5c = offset;
    LineTo16(local_44, local_3e, local_4a);
  // u_var4 = (param_2  >> 0x10);
    if ((*(param_2 + 4) & 4) != 0) {
        local_4 = 1;
    }
    local_10 = local_42 + local_4;
    local_e = (local_48 + local_4) - 2;
    local_c = local_10 - 3;
    local_a = local_48 + local_4 + 1;
    local_8 = local_10 + 3;
    local_56 = local_4c;
    local_54 = local_4a;
    local_58 = offset;
    local_5a = 0x8395;
    local_6 = local_a;
    SelectObject16(local_4c, local_4a);
    if (local_4 == 0) {
        local_56 = CONCAT22(local_4a, 1);
        local_58 = local_44 - 2;
        local_5c = 0x83b0;
        MoveTo16(local_58, 1, local_4a);
        local_52 = local_4a;
        local_56 = 0x10001;
        local_58 = offset;
        local_5a = 0x83be;
        LineTo16(1, 1, local_4a);
        local_50 = local_4a;
        local_52 = local_3e - 1;
        local_54 = 1;
        local_56 = offset;
        local_58 = 0x83cd;
        LineTo16(1, local_52, local_4a);
    }
    local_4 = ((*(param_2 + 4) & 8) != 0);
    local_1c = local_42 + local_4;
    i_var3 = (local_40 - local_48) + local_4;
    local_1a = i_var3 + 1;
    local_18 = local_1c - 3;
    local_16 = i_var3 - 2;
    local_14 = local_1c + 3;
    local_12 = local_16;
    if (local_4 == 0) {
        local_4e = local_4a;
        local_50 = 1;
        local_52 = local_52 - 2;
        local_56 = 0x15388429;
        MoveTo16(local_52, 1, local_4a);
        local_4c = local_4a;
        local_4e = 1;
        u_var2 = local_44 + 1;
        local_52 = offset;
        local_54 = 0x843a;
        local_50 = u_var2;
        LineTo16(u_var2, 1, local_4a);
        local_4c = local_3e - 1;
        local_50 = offset;
        local_52 = 0x8448;
        local_4e = u_var2;
        LineTo16(u_var2, local_4c, local_4a);
    }
    local_4e = local_4a;
    local_50 = local_4a;
    local_52 = offset;
    local_56 = CONCAT22(0x8453, local_56);
    SelectObject16(local_4a, local_4a);
    local_4e = local_4a;
    local_52 = offset;
    local_56 = CONCAT22(0x845e, local_56);
    SelectObject16(local_50, local_4a);
    local_4e = local_4a;
    local_52 = &local_10;
    local_56 = 0x31538;
    local_58 = 0x846d;
    Polygon16(3, local_52, unaff_ss);
    local_4e = local_4a;
    local_52 = &local_1c;
    local_56 = 0x31538;
    local_58 = 0x847c;
    Polygon16(3, local_52, unaff_ss);
    local_4e = local_4a;
    local_50 = local_5c;
    local_52 = offset;
    local_56 = CONCAT22(0x8487, local_56);
    SelectObject16(local_5c, local_4a);
    local_4e = local_4a;
    local_50 = local_5a;
    local_52 = offset;
    local_56 = CONCAT22(0x8492, local_56);
    SelectObject16(local_5a, local_4a);
    local_4e = local_4c;
    local_50 = offset;
    local_52 = 0x849a;
    DeleteObject16(local_4c);
    u_var2 = local_4e;
    _local_4c = CONCAT22(local_4a, local_4e);
    local_4e = offset;
    local_50 = 0x84a2;
    DeleteObject16(u_var2);
    _local_4c = CONCAT22(local_50, 0x1538);
    local_4e = 0x84aa;
    DeleteObject16(local_50);
    local_48 = param_1;
    _local_4c = CONCAT22(unaff_ss, &local_3c);
    local_4e = offset;
    local_50 = 0x84b7;
    EndPaint16(&local_3c, unaff_ss);
    return;
}

pub unsafe fn draw_1008_4f20(
    ctx: &mut AppContext,
    param_1: &mut Struct103,
    param_2: u32,
    param_3: u16,
    param_4: u32,
) {
    let mut palette: u16;
    let mut count: i32;
    let mut palette_1: u16;
    let mut hdc_var_1: HDC16;
    let mut unaff_ss: u16;
    let mut u_var1: u32;
    let mut color_ref: u32;
    let u_var2: u8;
    let u_var3: u8;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    // let pa_var7: &mut  Struct102;
    let mut u_var8: u16;
    // let mut u_var9: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16 = 0;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut a_struct_7_26: Struct7 = Struct7::new();

    // pa_var7 = param_1;
  // u_var9 = (param_1  >> 0x10);
    set_struct_1008_4016(pa_var7);
    (pa_var7 + 1) = param_4;
    pa_var7[1].field_0x4 = param_3;
    pa_var7[1].field_0x6 = param_2;
    param_1.field_0x0 = (ctx.s_SinternalPutBldg2_site_0x_08lx__1050_5099 + 9);
    pa_var7.field_0x2 = ctx.PTR_LOOP_1050_1008;
    mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 2);
    process_struct_1008_48fe(
        ctx,
        &mut a_struct_7_26.field_0,
        1,
        &get_string_from_address(param_2 & 0xffff | ctx.dx_reg << 0x10),
    );
    read_from_file_1008_49e8(ctx, &mut a_struct_7_26);
    pass1_1008_5068(param_1, &mut a_struct_7_26);
    process_struct_1008_47cc(param_1);
    process_struct_1008_4834(param_1);
    u_var8 = 0x27e;
    u_var5 = 0;
    u_var6 = 0;
    u_var2 = 0;
    u_var3 = 0;
    u_var4 = 0;
    u_var1 = process_struct_1008_4772(param_1);
  // local_28 = (u_var1  >> 0x10);
    local_2a = u_var1;
    // local_eax_147._0_2_ = CreateDC16(
    //     local_2a,
    //     CONCAT13(u_var3, CONCAT12(u_var2, local_28)),
    //     CONCAT22(u_var5, u_var4),
    //     CONCAT22(u_var8, u_var6),
    // );
    hdc_var_1 = CreateDC16(CONCAT22(u_var8, uvar_6), CONCAT22(u_var5, u_var4), CONCAT13(u_var3, (CONCAT12(u_var2, local_28))), local_2a);
    palette = local_2c;
    palette::realize_palette_1008_46e4(param_1, &mut palette);
    // color_ref = SetBkColor16(0xffffff, local_eax_147);
    color_ref = SetBkColor16(hdc_var_1, 0xffffff);
    // SetTextColor16(CONCAT22(0x100, &pa_var7[1].field_0x4), local_eax_147);
    SetTextColor16(hdc_var_1, CONCAT22(0x100, pa_var7[1].field_0x4));
    count = get_string_index_1000_3da4((pa_var7[1].field_0x0) as i32);
    // TextOut16(count, (pa_var7 + 1), 0, 0, local_eax_147);
    TextOut16(hdc_var_1, 0, 0, pa_var7 + 1, count as i16);
    // SetBkColor16(color_ref, local_eax_147);
    SetBkColor16(hdc_var_1, color_ref);
    SetTextColor16(hdc_var_1, hdc_var_1 as u32);
    // palette_1 = SelectPalette16(0, palette, hdc_var_1);
    SelectPalette16(hdc_var_1, palette, 0);
    DeleteObject16(palette);
    DeleteDC16(hdc_var_1);
    close_file_1008_496c(ctx, &mut a_struct_7_26);
    return;
}

pub unsafe fn draw_1018_c578(ctx: &mut AppContext, param_1: u32) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let in_struct_104_ptr: &mut  Struct104;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let h_palette: &mut  u16;
    let mut i_var5: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut u_var6: u16;
    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let mut unaff_ss: HWND16;
    let mut u_var9: u32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset;

    let local_26 = process_struct_1010_20ba(ctx.g_struct_1050_0ed0, CONCAT22(unaff_si, 2));
    local_28 = (local_26 + 0x20);
    i_var7 = param_1;
  // u_var8 = (param_1  >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var7 + 8));
        EndPaint16(&local_22, unaff_ss);
        PostMessage16(0, (i_var7 + 0xea), 0x111, ctx.g_h_window);
        return;
    }
    if (((i_var7 + 0xf0) == 0) && ((i_var7 + 0xf4) != 0)) {
        (i_var7 + 0xf0) = 1;
        mci_send_command_1008_5c9e(
            ctx.g_struct_1050_02a0,
            (param_1 & 0xffff0000 | (i_var7 + 0xf2)),
        );
    }
    if ((ctx.g_struct_1050_02a0 + 0x12) == 0) {
        mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, 0x1d3);
    }
    local_2a = BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var7 + 8));
    local_2c = CreateSolidBrush16(0x2000000);
    local_34 = 0;
    local_30 = (i_var7 + 0xf6) - 1;
    local_2e = (i_var7 + 0xf8) - 1;
    FillRect16(local_2c, &local_34, unaff_ss);
    DeleteObject16(local_2c);
    u_var4 = (i_var7 + 0xe2);
    in_struct_104_ptr = (u_var4 + 0xe);
    h_palette = &local_2a;
    u_var4 = in_struct_104_ptr;
    ppc_var3 = (u_var4 + 8);
    (**ppc_var3)(
        offset,
        in_struct_104_ptr,
        (in_struct_104_ptr >> 0x10),
        h_palette,
    );
    u_var9 = process_struct_1008_4772(in_struct_104_ptr);
  // u_var6 = (u_var9  >> 0x10);
    iVar1 = (u_var9 + 4);
    i_var2 = (u_var9 + 8);
    i_var5 = (0x1e0 - i_var2) / 2;
    (i_var7 + 0x10c) = i_var5 + i_var2 + (i_var7 + 0x110);
    ppc_var3 = (u_var4 + 4);
    (**ppc_var3)(
        &ctx.PTR_LOOP_1050_1008,
        in_struct_104_ptr,
        (i_var7 + 0xfc) + (i_var7 + 0xfe) + i_var5,
        (i_var7 + 0xfa) + (0x280 - iVar1) / 2,
        0xd6,
    );
    text::draw_text_1018_c742(param_1, &local_2a, unaff_ss);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_1018_cda8(ctx: &mut AppContext, param_1: u32) {
    let pi_var1: &mut  i32;
    let mut i_var2: i32;
    let in_struct_104_ptr: &mut  Struct104;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let h_palette: &mut  u16;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut u_var7: u16;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: HWND16;
    let mut u_var11: u32;
    let mut in_stack_0000ffb0: u32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

    let local_26 = process_struct_1010_20ba(
        ctx.g_struct_1050_0ed0,
        CONCAT22((in_stack_0000ffb0 >> 0x10), 2),
    );
    local_28 = (local_26 + 0x20);
    i_var8 = param_1;
  // u_var9 = (param_1  >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 8));
        EndPaint16(&local_22, unaff_ss);
        PostMessage16(0, (i_var8 + 0xea), 0x111, ctx.g_h_window);
        return;
    }
    if ((i_var8 + 0xf0) == 0) {
        (i_var8 + 0xf0) = 1;
        mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, 499);
        if ((ctx.g_struct_1050_02a0 + 0x12) == 0) {
            mci_send_cmd_1008_5c5c(ctx._g_astruct_112_ag_struct_1050_02a0, 0x1d3);
        }
    }
    local_2a = BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 8));
    local_2c = CreateSolidBrush16(0x2000000);
    local_34 = 0;
    local_30 = (i_var8 + 0xf6) - 1;
    local_2e = (i_var8 + 0xf8) - 1;
    FillRect16(local_2c, &local_34, unaff_ss);
    DeleteObject16(local_2c);
    u_var4 = (i_var8 + 0xe2);
    in_struct_104_ptr = (u_var4 + 0xe);
    h_palette = &local_2a;
  // u_var10 = (in_struct_104_ptr  >> 0x10);
    ppc_var3 = (in_struct_104_ptr + 8);
    (**ppc_var3)(offset, in_struct_104_ptr, u_var10, h_palette);
    u_var11 = process_struct_1008_4772(in_struct_104_ptr);
  // u_var7 = (u_var11  >> 0x10);
    i_var5 = (0x280 - (u_var11 + 4)) / 2;
    i_var2 = (u_var11 + 8);
    i32_var6 = (0x1e0 - i_var2) / 2;
    (i_var8 + 0x10c) = i32_var6 + i_var2 + (i_var8 + 0x110);
    if (((i_var8 + 0xfa) == 0) && (i_var5 == 0)) {
        pi_var1 = (i_var8 + 0xfa);
        unsafe { *pi_var1 = *pi_var1 + 2 };
    }
    ppc_var3 = (in_struct_104_ptr + 4);
    (**ppc_var3)(
        &ctx.g_struct_1050_02a0PTR_LOOP_1050_1008,
        in_struct_104_ptr,
        u_var10,
        (i_var8 + 0xfc) + (i_var8 + 0xfe) + i32_var6,
        (i_var8 + 0xfa) + i_var5,
        0xd6,
    );
    text::draw_text_1018_c742(param_1, &local_2a, unaff_ss);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_1018_cfc0(ctx: &mut AppContext, param_1: u32) {
    let pi_var1: &mut  i32;
    let mut i_var2: i32;
    let in_struct_104_ptr: &mut  Struct104;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let h_palette: &mut  u16;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut u_var7: u16;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: HWND16;
    let mut u_var11: u32;
    let mut in_stack_0000ffb0: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

    let local_26 =
        process_struct_1010_20ba(ctx.g_struct_1050_0ed0, CONCAT22(in_stack_0000ffb0, 2));
    local_28 = (local_26 + 0x20);
    i_var8 = param_1;
  // u_var9 = (param_1  >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 8));
        EndPaint16(&local_22, unaff_ss);
        PostMessage16(0, (i_var8 + 0xea), 0x111, ctx.g_h_window);
        return;
    }
    if (((i_var8 + 0xf0) == 0) && ((i_var8 + 0xf4) != 0)) {
        (i_var8 + 0xf0) = 1;
        mci_send_command_1008_5c9e(
            ctx.g_struct_1050_02a0,
            (param_1 & 0xffff0000 | (i_var8 + 0xf2)),
        );
        if ((ctx.g_struct_1050_02a0 + 0x12) == 0) {
            mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, 0x1d3);
        }
    }
    local_2a = BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 8));
    local_2c = CreateSolidBrush16(0x2000000);
    local_34 = 0;
    local_30 = (i_var8 + 0xf6) - 1;
    local_2e = (i_var8 + 0xf8) - 1;
    FillRect16(local_2c, &local_34, unaff_ss);
    DeleteObject16(local_2c);
    u_var4 = (i_var8 + 0xe2);
    in_struct_104_ptr = (u_var4 + 0xe);
    h_palette = &local_2a;
  // u_var10 = (in_struct_104_ptr  >> 0x10);
    ppc_var3 = (in_struct_104_ptr + 8);
    (**ppc_var3)(offset, in_struct_104_ptr, u_var10, h_palette);
    u_var11 = process_struct_1008_4772(in_struct_104_ptr);
  // u_var7 = (u_var11  >> 0x10);
    i_var5 = (0x280 - (u_var11 + 4)) / 2;
    i_var2 = (u_var11 + 8);
    i32_var6 = (0x1e0 - i_var2) / 2;
    (i_var8 + 0x10c) = i32_var6 + i_var2 + (i_var8 + 0x110);
    if (((i_var8 + 0xfa) == 0) && (i_var5 == 0)) {
        pi_var1 = (i_var8 + 0xfa);
        unsafe { *pi_var1 = *pi_var1 + 2 };
    }
    ppc_var3 = (in_struct_104_ptr + 4);
    (**ppc_var3)(
        &ctx.PTR_LOOP_1050_1008,
        in_struct_104_ptr,
        u_var10,
        (i_var8 + 0xfc) + (i_var8 + 0xfe) + i32_var6,
        (i_var8 + 0xfa) + i_var5,
        0xd6,
    );
    text::draw_text_1018_c742(param_1, &local_2a, unaff_ss);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_1020_0000(ctx: &mut AppContext, param_1: u32) {
    let pu_var1: &mut  u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u32;

    let mut u_var6: i32;

    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: HWND16;
    let mut local_c4: [u8; 6];
    let mut local_be: u16;
    let mut local_b8: u16;

    let mut local_b4: u16;
    let mut local_b2: u16;
    let mut local_b0: [u16; 60];
    let mut local_38: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_22: PAINTSTRUCT16;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  // u_var8 = (param_1  >> 0x10);
    i_var7 = param_1;
    BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var7 + 4));
    u_var3 = (i_var7 + 0x14);
    local_26 = (u_var3 + 10);
    pass1_1008_3e94(
        (i_var7 + 0x18),
        CONCAT22(unaff_ss, &local_2a),
        CONCAT22(unaff_ss, &local_28),
    );
    u_var9 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    pass1_1008_4480(
        local_26,
        (param_1 & 0xffff0000 | ZEXT24((i_var7 + 0x18))),
        (i_var7 + 0x24),
    );
    local_2e = 0;
    local_30 = 0;
    while (local_30 < 6) {
        u_var3 = (i_var7 + 0x14);
        u_var9 = 0x1010;
        pass1_1010_2b78(
            u_var3,
            (u_var3 >> 0x10),
            local_30,
            CONCAT22(unaff_ss, &local_b4),
        );
        if (local_b4 == 0) {
            local_38 = 0;
            u_var6 = ctx.dx_reg;
            while (u_var4 = local_38, local_38 <= local_b2) {
                pu_var1 = local_b0 + local_38 * 3;
                local_b8 = pu_var1;
                if (local_b0[local_38 * 3 + 2] != 0) {
                    u_var5 = pass1_1010_2b98((i_var7 + 0x14), local_b0[local_38 * 3 + 2]);
                    local_2e = u_var5 & 0xffff | u_var6 << 0x10;
                    let pu_var1_val = unsafe { *pu_var1 };
                    pass1_1008_3e54(
                        CONCAT22(unaff_ss, &local_be),
                        0,
                        local_b0[u_var4 * 3 + 1] + local_2a,
                        pu_var1_val + local_28,
                    );
                    u_var9 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                    pass1_1008_4480(local_26, CONCAT22(unaff_ss, &local_be), local_2e);
                    u_var6 = ctx.dx_reg;
                }
                local_38 = local_38 + 1;
            }
        } else {
            let _local_be = CONCAT22(unaff_ss, local_b0 + local_b2 * 3);
            if (local_b0[local_b2 * 3 + 2] != 0) {
                u_var6 = ctx.dx_reg;
                u_var5 = pass1_1010_2b98((i_var7 + 0x14), local_b0[local_b2 * 3 + 2]);
                local_2e = u_var5 & 0xffff | u_var6 << 0x10;
                pass1_1008_3e54(
                    CONCAT22(unaff_ss, local_c4),
                    0,
                    (_local_be + 2) + local_2a,
                    *_local_be + local_28,
                );
                u_var9 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                pass1_1008_4480(local_26, CONCAT22(unaff_ss, local_c4), local_2e);
            }
        }
        local_30 = local_30 + 1;
    }
    ppc_var2 = (local_26 + 4);
    ppc_var2(
        u_var9,
        local_26,
        (local_26 >> 0x10),
        0,
        0,
        i_var7 + 10,
        u_var8,
    );
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_1020_0c3e(in_struct_1: &mut  Struct631) {
    let paVar1: &mut  Struct632;
    let h_palette: &mut  HDC16;
    let mut h_gdi_obj: HPALETTE16;
    let struct_a: &mut  Struct631;
    let struct_b: &mut  Struct632;
    let struct_a_hi: &mut  Struct631;
    let struct_c_hi: &mut  Struct632;
    let mut window_handle_a: HWND16;
    let mut local_30: u16;
    let local_28: &mut  Struct632;
    let mut hdc1: HDC16;
    let mut paint_struct_a: PAINTSTRUCT16;
    let struct_c: &mut  Struct632;
    let temp_5f82dd1a72: &mut  Struct632;
    let fn_ptr_1: fn();
    let mut offset: u16;

  // struct_a_hi = (in_struct_1  >> 0x10);
    struct_a = in_struct_1;
    hdc1 = BeginPaint16(
        CONCAT22(window_handle_a, &paint_struct_a),
        struct_a.hwnd_0x4,
    );
    struct_c = struct_a.astruct_632_0x6;
  // struct_c_hi = (struct_c  >> 0x10);
    struct_b = struct_c;
    paVar1 = struct_b.fn_ptr_0xa;
    local_28._0_2_ = paVar1;
    if ((struct_b.field_0xc | local_28) != 0) {
        h_palette = &hdc1;
        temp_5f82dd1a72 = paVar1;
        fn_ptr_1 = (temp_5f82dd1a72 + 8);
        (**fn_ptr_1)(offset, local_28, (paVar1 >> 0x10), h_palette);
        fn_ptr_1 = (temp_5f82dd1a72 + 4);
        (**fn_ptr_1)(
            offset,
            paVar1,
            &struct_a.field_0xc,
            struct_a.field_0xa,
            &hdc1,
        );
        h_gdi_obj = SelectPalette16(0, h_palette, hdc1);
        DeleteObject16(h_gdi_obj);
    }
    EndPaint16(&paint_struct_a, window_handle_a);
    return;
}

pub unsafe fn draw_1040_a67e(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: i32,
    param_3: i32,
    in_h_dc: HDC16,
) {
    let pu_var1: &mut  u16;
    let mut b_var2: bool;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut brush: u16;
    let local_bx_5: &mut  Struct36;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;

  // u_var5 = (param_1  >> 0x10);
    local_bx_5 = param_1;
    if (local_bx_5.brush == 0) {
        brush = CreateSolidBrush16(0);
        local_bx_5.brush = brush;
    }
    if (ctx._PTR_LOOP_1050_5ee8 == 0) {
        u_var6 = pass1_1008_4d72((ctx._PTR_LOOP_1050_4230 + 0xe));
      // u_var3 = (u_var6  >> 0x10);
        i_var4 = u_var6;
        ctx._PTR_LOOP_1050_5ee8 = CONCAT12(
            *(i_var4 + 0x94),
            CONCAT11(*(i_var4 + 0x95), *(i_var4 + 0x96)),
        );
        ctx.PTR_LOOP_1050_5eec = CONCAT11(*(i_var4 + 0x3e5), *(i_var4 + 0x3e6));
        ctx.PTR_LOOP_1050_5eee = *(i_var4 + 0x3e4);
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return;
        }
        b_var2 = false;
        local_e = 0;
        let pu_var1_val = unsafe { *pu_var1 };
        while (
            pu_var1 = &local_bx_5.field_0xea,
            pu_var1_val != local_e && local_e <= pu_var1_val,
        ) {
            if ((&local_bx_5.field_0x9a + local_e * 2) == param_2) {
                b_var2 = true;
                break;
            }
            local_e = local_e + 1;
        }
        if (b_var2) {
            ctx.PTR_LOOP_1050_5ee8 = ctx.PTR_LOOP_1050_5eec;
            ctx.PTR_LOOP_1050_5eea = ctx.PTR_LOOP_1050_5eee;
        }
    }
    SetTextColor16(
        CONCAT22(ctx.PTR_LOOP_1050_5eea, ctx.PTR_LOOP_1050_5ee8),
        in_h_dc,
    );
    SetBkColor16(0x1000000, in_h_dc);
    return;
}

pub fn draw_1040_82ee(param_1: u32) {
    let local_bx_6: &mut  Struct35;
    let mut hdc: HDC16;
    let mut unaff_ss: HDC16;
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

  // hdc = (param_1  >> 0x10);
    local_bx_6 = param_1;
    local_6 = (local_bx_6.field_0x80 - local_bx_6.field_0x7c) - 2;
    local_8 = (-(local_bx_6.field_0x60 == 0) & 0x1e) + 0x25;
    local_4 = local_6;
    local_a = CreateSolidBrush16(CONCAT22(0x100, local_8));
    if (local_bx_6.field_0x86 == 0) {
        let _local_1a = CONCAT22(local_bx_6.field_0x66 + 2, local_bx_6.field_0x64 + 2);
        local_16 = CONCAT22(local_4, local_6);
        &local_bx_6.field_0x82 = _local_1a;
        &local_bx_6.field_0x86 = local_16;
    }
    local_12 = local_bx_6.field_0x82 + 2;
    local_10 = ((local_bx_6.field_0x88 - local_bx_6.field_0x84) / 2 + local_bx_6.field_0x84) - 2;
    local_e = local_bx_6.field_0x86 - 2;
    local_c = (local_bx_6.field_0x88 - local_bx_6.field_0x84) / 2 + local_bx_6.field_0x84 + 2;
    FrameRect16(local_a, &local_bx_6.field_0x82, hdc);
    FrameRect16(local_a, &local_12, unaff_ss);
    DeleteObject16(local_a);
    local_bx_6.field_0x7a = local_bx_6.field_0x86 + 2;
    return;
}

pub unsafe fn draw_fn_1040_7e5e(
    ctx: &mut AppContext,
    param_1: &mut  u32,
    param_2: HWND16,
    param_3: i32,
    param_4: HDC16,
) {
    let pp_var1: fn();
    let mut HVar2: HGDIOBJ16;
    let mut i_var3: i32;
    let mut i_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    HVar2 = GetStockObject16(4);
    if (ctx._PTR_LOOP_1050_5df0 == 0) {
      // u_var5 = (param_1  >> 0x10);
        let param_1_val = unsafe { *param_1 };
        pp_var1 = (param_1_val + 0x68);
        u_var6 = (**pp_var1)(offset, param_1, u_var5, (param_1 + 0x6e));
        if (u_var6 == 0) {
            return 0;
        }
        u_var6 = pass1_1008_4d72(u_var6);
      // u_var5 = (u_var6  >> 0x10);
        i_var3 = u_var6;
        ctx._PTR_LOOP_1050_5df0 = CONCAT22(
            CONCAT11(2, *(i_var3 + 0x94)),
            CONCAT11(*(i_var3 + 0x95), *(i_var3 + 0x96)),
        );
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return 0;
        }
        u_var5 = 0x7ed5;
        i_var4 = GetDlgCtrlID16(param_2);
        if (i_var4 == 0x14c) {
            u_var7 = 0xffff;
            local_c = 0;
            // goto LAB_1040_7f00;
        }
        if (i_var4 == 0x175) {
            u_var7 = 0xff;
            local_c = 0;
            // goto LAB_1040_7f00;
        }
    }
    local_c = param_4;
    u_var5 = ctx._PTR_LOOP_1050_5df0;
  // u_var7 = (ctx._PTR_LOOP_1050_5df0  >> 0x10);
    // LAB_1040_7f00:
    SetTextColor16(CONCAT22(u_var7, u_var5), local_c);
    SetBkColor16(0x1000000, param_4);
    return CONCAT22(0x1050, HVar2);
}

pub fn draw_1040_7bb2(ctx: &mut AppContext, param_1: &mut  u32) {
    let pp_var1: fn();
    let mut hdc: u16;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut obj_handle: HPEN16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut h_var6: HGDIOBJ16;
    let mut obj_handle_01: HANDLE16;
    let mut count: u16;
    let mut i_var7: i32;
    let rect: &mut  RECT16;
    let mut unaff_ss: HWND16;
    let mut u_var8: u32;
    let mut dVar9: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
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
    let mut offset: u16;

  // rect = (param_1  >> 0x10);
    i_var7 = param_1;
    BVar2 = IsIconic16((i_var7 + 6));
    if (BVar2 == 0) {
        local_4 = GetWindowDC16((i_var7 + 6));
        let param_1_val = unsafe { *param_1 };
        pp_var1 = (param_1_val + 0x68);
        local_8 = (**pp_var1)(offset, i_var7, rect, (i_var7 + 0x6e));
        if (local_8 != 0) {
            local_a = palette::realize_palette_1008_4e08(local_8, 0xfc, unaff_ss);
            GetWindowRect16(CONCAT22(&local_12, 0x1008), unaff_ss);
            i_var3 = (local_e - local_12) + -1;
            i_var4 = (local_c - local_10) + -1;
            i_var5 = (-((i_var7 + 0x60) == 0) & 0x1e) + 0x25;
            obj_handle = CreatePen16(CONCAT22(0x100, i_var5), 0, 0);
            obj_handle_00 = SelectObject16(obj_handle, local_4);
            MoveTo16(0, 0, local_4);
            LineTo16(0, i_var3, local_4);
            LineTo16(i_var4, i_var3, local_4);
            LineTo16(i_var4, 0, local_4);
            LineTo16(0, 0, local_4);
            u_var8 = GetWindowLong16(-0x10, (i_var7 + 6));
            if (((u_var8 & 0x800000) != 0) && ((u_var8 & 0x400000) != 0)) {
                i_var4 = (i_var7 + 0x62) - (i_var7 + 0x66);
                MoveTo16(i_var4, 0, local_4);
                LineTo16(i_var4, i_var3, local_4);
                hdc = local_4;
                (i_var7 + 0x7a) = (i_var7 + 100);
                (i_var7 + 0x7c) = (i_var7 + 0x66);
                (i_var7 + 0x7e) = i_var3;
                (i_var7 + 0x80) = (i_var7 + 0x62) - (i_var7 + 0x66);
                h_var6 = GetStockObject16(4);
                FillRect16(h_var6, rect, hdc);
                if ((i_var7 + 0x76) != 0) {
                    draw_1040_82ee(i_var7, rect, local_4);
                }
                if (*(i_var7 + 0x10) != '\0') {
                    obj_handle_01 = GetProp16(ctx.s_hfont_1050_5de9, (i_var7 + 6));
                    if (obj_handle_01 != 0) {
                        SelectObject16(obj_handle_01, local_4);
                    }
                    SetBkColor16(0, local_4);
                    SetTextColor16(CONCAT22(0x100, i_var5), local_4);
                    count = lstrlen16((param_1 & 0xffff0000 | ZEXT24((i_var7 + 0x10))));
                    dVar9 = GetTextExtent16(count, (param_1 & 0xffff0000 | local_4), local_4);
                    i_var3 =
                        (((i_var7 + 0x7e) - (i_var7 + 0x7a)) / 2 - dVar9 / 2) + (i_var7 + 0x7a);
                  // h_var6 = ((i_var7 + 0x80) - (i_var7 + 0x7c)) / 2 - (dVar9  >> 0x10) / 2;
                  // local_2c = ((param_1 & 0xffff0000)  >> 0x10);
                    TextOut16(
                        local_2c,
                        (param_1 & 0xffff0000 | local_4),
                        h_var6,
                        i_var3,
                        local_4,
                    );
                    if (i_var3 != 0) {
                        SelectObject16(h_var6, local_4);
                    }
                }
            }
            pp_var1 = (param_1_val + 100);
            (**pp_var1)(offset, i_var7, rect, local_8, local_4);
            local_a = SelectPalette16(0, local_a, local_4);
            DeleteObject16(local_a);
            SelectObject16(obj_handle_00, local_4);
            DeleteObject16(obj_handle);
        }
        ReleaseDC16(local_4, (i_var7 + 6));
        return;
    }
    return;
}

pub fn draw_1040_5a06(ctx: &mut AppContext, param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let h_palette: &mut  u16;
    let mut i_var5: u16;
    let mut obj_handle: HPEN16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut h_var6: HGDIOBJ16;
    let mut y: i32;
    let mut y_00: i32;
    let mut x: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut u_var7: u32;




    let mut u_var8: u16;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_cs: u16;
    let mut u_var11: u16;
    let mut unaff_ss: HWND16;
    let pu_var12: &mut  u16;
    let mut local_62: u32;
    let mut local_5e: u32;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: PAINTSTRUCT16;
    let mut local_a: [u8; 8];
    let mut offset: u16;

  // u_var10 = (param_1  >> 0x10);
    i_var9 = param_1;
    GetWindowRect16(
        CONCAT13((local_a >> 8), CONCAT12(local_a, unaff_cs)),
        unaff_ss,
    );
    local_2c = BeginPaint16(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_2a)),
        (i_var9 + 6),
    );
    u_var2 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    h_palette = &local_2c;
    u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    palette::realize_palette_1008_4e08(u_var2, (u_var2 >> 0x10), h_palette, unaff_ss);
    u_var7 = 0;
    u_var8 = 0;
    local_36 = 0;
    if ((i_var9 + 0x98) != 0) {
        mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, (i_var9 + 0x98));
        local_36 = CONCAT22(ctx.dx_reg, u_var8);
        u_var7 = process_struct_1008_4772(CONCAT22(ctx.dx_reg, u_var8));
        if (((u_var7 >> 0x10) | (u_var7 & 0xffff)) == 0) {
            u_var7 = u_var7 & 0xffff;
            if (local_36 != 0) {
                u_var7 = local_36;
                if (local_36 != 0) {
                    ppcVar4 = local_36;
                    (**ppcVar4)(&ctx.PTR_LOOP_1050_1008, u_var8, ctx.dx_reg, 1);
                    u_var7 = local_36;
                }
            }
            u_var8 = u_var7;
            mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 0x4d);
            local_36 = CONCAT22(ctx.dx_reg, u_var8);
        }
        pu_var12 = &local_2c;
        u_var11 = SUB42(offset, 0);
        i_var5 = GetSystemMetrics16(4);
        u_var7 = -(i_var5 + -0x23);
        ppcVar4 = (local_36 + 4);
        (**ppcVar4)(
            0x38,
            local_36,
            (local_36 >> 0x10),
            -(i_var5 + -0x23),
            pu_var12,
        );
    }
    if (local_36 != 0) {
        u_var7 = local_36;
        if (local_36 != 0) {
            ppcVar4 = local_36;
            (**ppcVar4)(u_var11, local_36, (local_36 >> 0x10), 1);
            u_var7 = local_36;
        }
    }
    u_var8 = u_var7;
    mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, (i_var9 + 0x96));
    local_36 = CONCAT22(ctx.dx_reg, u_var8);
    pu_var12 = &local_2c;
    i_var5 = GetSystemMetrics16(4);
    u_var2 = local_36;
    ppcVar4 = u_var2 + 2;
    (**ppcVar4)(0x38, u_var8, ctx.dx_reg, -(i_var5 + -0x23), pu_var12);
    if (local_36 != 0) {
        if (local_36 != 0) {
            ppcVar4 = u_var2;
            (**ppcVar4)(offset, u_var8, ctx.dx_reg, 1);
        }
    }
    obj_handle = CreatePen16(0x1000025, 0, 0);
    obj_handle_00 = SelectObject16(obj_handle, local_2c);
    h_var6 = obj_handle_00;
    mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 0x4f);
    local_36 = CONCAT22(ctx.dx_reg, h_var6);
    u_var7 = process_struct_1008_4772(CONCAT13(
        (ctx.dx_reg >> 8),
        CONCAT12(ctx.dx_reg, h_var6),
    ));
  // u_var8 = (u_var7  >> 0x10);
    u_var2 = (u_var7 + 4);
    u_var3 = (u_var7 + 8);
    i_var5 = GetSystemMetrics16(4);
    y = -(i_var5 + -0xc1);
    i_var5 = GetSystemMetrics16(4);
    local_48._0_2_ = u_var3;
    y_00 = 0xc5 - (i_var5 - local_48);
    MoveTo16(y, 0x82, local_2c);
    local_44._0_2_ = u_var2;
    x = local_44 * 10 + 0x85;
    LineTo16(y, x, local_2c);
    LineTo16(y_00, x, local_2c);
    LineTo16(y_00, 0x82, local_2c);
    LineTo16(y, 0x82, local_2c);
    local_52 = 0;
    let pu_var1_val = unsafe { *pu_var1 };
    while (
        pu_var1 = (i_var9 + 0x94),
        local_52 <= pu_var1_val && pu_var1_val != local_52,
    ) {
        pu_var12 = &local_2c;
        i_var5 = GetSystemMetrics16(4);
        ppcVar4 = (local_36 + 4);
        (**ppcVar4)(offset, h_var6, ctx.dx_reg, -(i_var5 + -0xc4), pu_var12);
        local_52 = local_52 + 1;
    }
    if (local_36 != 0) {
        if (local_36 != 0) {
            ppcVar4 = local_36;
            (**ppcVar4)(offset, h_var6, ctx.dx_reg, 1, local_36, local_36);
        }
    }
    SelectObject16(obj_handle_00, local_2c);
    DeleteObject16(obj_handle);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2c);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_2a, unaff_ss);
    return;
}

pub unsafe fn draw_1040_27cc(
    ctx: &mut AppContext,
    param_1: &mut  u32,
    param_2: HWND16,
    param_3: i32,
    param_4: HDC16,
) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut HVar3: HBRUSH16;
    let mut i_var4: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_cs: u16;
    let mut u_var7: u32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut hdc: HDC16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut offset: u16;

  // u_var6 = (param_1  >> 0x10);
    i_var5 = param_1;
    if ((i_var5 + 4) == 0) {
        unaff_cs = SUB42(offset, 0);
        HVar3 = CreateSolidBrush16(0);
        (i_var5 + 4) = HVar3;
    }
    if (ctx._PTR_LOOP_1050_5cf8 == 0) {
        let param_1_val = unsafe { *param_1 };
        pp_var1 = (param_1_val + 0x68);
        u_var7 = (**pp_var1)(unaff_cs, i_var5, u_var6, (i_var5 + 0x6e));
        u_var7 = pass1_1008_4d72(u_var7);
      // u_var8 = (u_var7  >> 0x10);
        i_var2 = u_var7;
        ctx._PTR_LOOP_1050_5cf8 = CONCAT22(
            CONCAT11(2, *(i_var2 + 0x94)),
            CONCAT11(*(i_var2 + 0x95), *(i_var2 + 0x96)),
        );
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return 0;
        }
        u_var8 = SUB42(ctx.s_fem67_wav_1050_2842 + 8, 0);
        i_var4 = GetDlgCtrlID16(param_2);
        if (((i_var5 + 0x94) != 0) && (i_var4 == 0xfb2)) {
            u_var9 = 0xff;
            hdc = 0;
            // goto LAB_1040_286e;
        }
    }
    u_var8 = ctx._PTR_LOOP_1050_5cf8;
  // u_var9 = (ctx._PTR_LOOP_1050_5cf8  >> 0x10);
    hdc = param_4;
    // LAB_1040_286e:
    SetTextColor16(CONCAT22(u_var9, u_var8), hdc);
    SetBkColor16(0x1000000, param_4);
    return CONCAT22(0x1050, (i_var5 + 4));
}

pub unsafe fn draw_1040_21d6(ctx: &mut AppContext, param_1: u32) {
    let u_var1: u8;
    let u_var2: u8;
    let u_var3: u8;
    let mut u_var4: u32;
    let ppcVar5: fn();
    let mut u_var6: u16;
    let mut i_var7: i32;
    let h_palette: &mut  u16;
    let mut obj_handle: HANDLE16;
    let mut h_gdi_obj: HPALETTE16;
    let mut i_var8: i32;
    let mut count: u16;
    let mut unaff_ss: HWND16;
    let mut u_var9: u32;
    let mut color: COLORREF;
    let mut color_00: COLORREF;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

  // count = (param_1  >> 0x10);
    i_var8 = param_1;
    local_24 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 6));
    u_var9 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    h_palette = &local_24;
    palette::realize_palette_1008_4e08(u_var9, h_palette, unaff_ss);
    u_var4 = (i_var8 + 0x8e);
    ppcVar5 = ((i_var8 + 0x8e) + 4);
    (**ppcVar5)(
        &ctx.PTR_LOOP_1050_1008,
        u_var4,
        (u_var4 >> 0x10),
        0xffffffff,
        &local_24,
    );
    u_var9 = pass1_1008_4d72(u_var9);
  // u_var6 = (u_var9  >> 0x10);
    i_var7 = u_var9;
    u_var1 = *(i_var7 + 0x3e5);
    u_var2 = *(i_var7 + 0x3e6);
    u_var3 = *(i_var7 + 0x3e4);
    color = SetBkColor16(0, local_24);
    color_00 = SetTextColor16(
        CONCAT22(CONCAT11(2, u_var3), CONCAT11(u_var1, u_var2)),
        local_24,
    );
    local_3e = 0;
    obj_handle = GetProp16(ctx.s_hfont_1050_5ced, (i_var8 + 6));
    if (obj_handle != 0) {
        local_3e = SelectObject16(obj_handle, local_24);
    }
    u_var4 = (i_var8 + 0xa2);
    DrawText16(
        0x10,
        (i_var8 + 0x92),
        count,
        CONCAT13((u_var4 >> 8), CONCAT12(u_var4, 0xffff)),
        (u_var4 >> 0x10),
    );
    SetTextColor16(
        CONCAT13(
            2,
            CONCAT12(
                *(i_var7 + 0x94),
                CONCAT11(*(i_var7 + 0x95), *(i_var7 + 0x96)),
            ),
        ),
        local_24,
    );
    u_var4 = (i_var8 + 0xa6);
    DrawText16(
        0x10,
        (i_var8 + 0x9a),
        count,
        CONCAT13((u_var4 >> 8), CONCAT12(u_var4, 0xffff)),
        (u_var4 >> 0x10),
    );
    if (obj_handle != 0) {
        SelectObject16(local_3e, local_24);
    }
    SetBkColor16(color, local_24);
    SetTextColor16(color_00, local_24);
    h_gdi_obj = SelectPalette16(0, h_palette, local_24);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_1038_9dcc(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: HDC16,
) {
    let pu_var1: &mut  u32;
    let mut b_var2: bool;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut solid_brush: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;

  // u_var6 = (param_1  >> 0x10);
    i_var5 = param_1;
    if ((i_var5 + 0x8e) == 0) {
        solid_brush = CreateSolidBrush16(0);
        (i_var5 + 0x8e) = solid_brush;
    }
    if (ctx._PTR_LOOP_1050_5b64 == 0) {
        u_var7 = pass1_1008_4d72((ctx._PTR_LOOP_1050_4230 + 0xe));
      // u_var3 = (u_var7  >> 0x10);
        i_var4 = u_var7;
        ctx._PTR_LOOP_1050_5b64 = CONCAT12(
            *(i_var4 + 0x94),
            CONCAT11(*(i_var4 + 0x95), *(i_var4 + 0x96)),
        );
        ctx.PTR_LOOP_1050_5b68 = CONCAT11(*(i_var4 + 0x3e5), *(i_var4 + 0x3e6));
        ctx.PTR_LOOP_1050_5b6a = *(i_var4 + 0x3e4);
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return;
        }
        b_var2 = false;
        local_e = 0;
        let pu_var_1 = unsafe { *pu_var1 };
        while (
            pu_var1 = (i_var5 + 0x128),
            local_e <= pu_var_1 && pu_var_1 != local_e,
        ) {
            if ((i_var5 + 0x94 + local_e * 2) == param_2) {
                b_var2 = true;
                break;
            }
            local_e = local_e + 1;
        }
        if (b_var2) {
            ctx.PTR_LOOP_1050_5b64 = ctx.PTR_LOOP_1050_5b68;
            ctx.PTR_LOOP_1050_5b66 = ctx.PTR_LOOP_1050_5b6a;
        }
    }
    SetTextColor16(
        CONCAT22(ctx.PTR_LOOP_1050_5b66, ctx.PTR_LOOP_1050_5b64),
        param_4,
    );
    SetBkColor16(0x1000000, param_4);
    return;
}

pub fn draw_1020_9312(param_1: u32) {
    let pu_var1: &mut  u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: HWND16;
    let mut local_26: u32;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var4 + 4));
    u_var3 = (i_var4 + 6);
    pu_var1 = (u_var3 + 10);
    let pu_var1_val = unsafe { *pu_var1 };
    ppc_var2 = (pu_var1_val + 4);
    ppc_var2(
        offset,
        pu_var1,
        (pu_var1 >> 0x10),
        0,
        param_1 & 0xffff0000 | (i_var4 + 10),
    );
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub fn draw_1020_9364(ctx: &mut AppContext, param_1: &mut  Struct622) {
    let pi_var1: &mut  i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut h_var4: HBRUSH16;
    let mut x: i32;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut unaff_ss: HANDLE16;
    let mut u_var9: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_42: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 4];
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

  // u_var7 = (param_1  >> 0x10);
    i32_var6 = param_1;
    GetClientRect16(CONCAT22(unaff_ss, &local_a), (i32_var6 + 4));
    local_12 = local_a;
    local_e = local_6;
    local_14 = ctx.u16_1050_4216;
    local_16 = ctx.WORD_1050_422c;
    local_1a = ctx._PTR_PTR_BYTE_1050_0009_1050_4172_1050_4212;
    local_1e = ctx._PTR_PTR_s_ibr_1050_004f_1050_41b2_1050_4218;
    local_22 = ctx._PTR_PTR_s_ew_failed_in_Op_Op_1050_0021_1050_41da_1050_421c;
    local_26 = ctx._PTR_PTR_s_n_Op_Op__ResLibr_1050_0041_1050_4202_1050_4220;
    local_2a = ctx._PTR_WORD_1050_419a_1050_4224;
    local_2e = ctx._PTR_PTR_1050_4228;
    u_var3 = (i32_var6 + 6);
    local_30 = (u_var3 + 0x12);
    local_3a = 9;
    while {
        u_var9 = local_30;
        local_32 = CreatePen16((local_3a * 4 + local_22), 0, 0);
        local_34 = SelectObject16(local_32, u_var9);
        MoveToEx16(
            CONCAT22(unaff_ss, local_38),
            (local_3a * 2 + local_1a),
            local_a,
            local_30,
        );
        LineTo16(local_1a + local_3a * 2, local_6, local_30);
        i_var5 = (local_14 - local_3a) * 2;
        MoveToEx16(
            CONCAT22(unaff_ss, local_38),
            (i_var5 + local_1a),
            local_a,
            local_30,
        );
        LineTo16(local_1a + i_var5, local_6, local_30);
        SelectObject16(local_34, local_30);
        DeleteObject16(local_32);
        local_3a = local_3a - 1;
        local_3a < 0x8000
    } {}
    h_var4 = CreateSolidBrush16(0x2000000);
  // u_var8 = (local_1a  >> 0x10);
    local_a = CONCAT22((local_1a + 0x12) + 1, local_a);
    u_var2 = (local_1a + 0x14);
    local_e = local_e & 0xffff | u_var2 << 0x10;
    local_6 = CONCAT22(u_var2, local_6);
    FillRect16(h_var4, &local_a, unaff_ss);
    DeleteObject16(h_var4);
    local_3a = 1;
    local_3e = 8;
    while (local_3a < 10) {
        h_var4 = CreateSolidBrush16((local_1e + local_3e * 4 + 4));
        local_6 = local_6 & 0xffff | (local_a - 1) << 0x10;
        local_12 = local_12 & 0xffff | (local_e + 1) << 0x10;
      // u_var8 = (local_1a  >> 0x10);
        local_a = local_a & 0xffff | ((local_3e * 2 + local_1a) + 1) << 0x10;
        local_e = local_e & 0xffff | *(local_3a * 2 + local_1a + 0x14) << 0x10;
        FillRect16(h_var4, &local_a, unaff_ss);
        FillRect16(h_var4, &local_12, unaff_ss);
        DeleteObject16(h_var4);
        local_3a = local_3a + 1;
        local_3e = local_3e - 1;
    }
    h_var4 = CreateSolidBrush16(local_1e);
    local_a = local_a & 0xffff;
    local_6 = local_6 & 0xffff | *local_1a << 0x10;
    local_12 = local_12 & 0xffff | ((local_14 * 2 + local_1a) + 1) << 0x10;
    local_e = local_e & 0xffff | *(i32_var6 + 0xe) << 0x10;
    FillRect16(h_var4, &local_a, unaff_ss);
    FillRect16(h_var4, &local_12, unaff_ss);
    DeleteObject16(h_var4);
    local_3a = 3;
    while {
        u_var9 = local_30;
        local_32 = CreatePen16((local_3a * 4 + local_26), 0, 0);
        local_34 = SelectObject16(local_32, u_var9);
        i_var5 = local_3a * 2;
        x = (i_var5 + local_2a) + local_a;
      // u_var8 = (local_2e  >> 0x10);
        pi_var1 = (i_var5 + local_2e);
        MoveToEx16(
            CONCAT22(unaff_ss, local_38),
            ((i_var5 + local_2e) * 2 + local_1a),
            x,
            local_30,
        );
        let pi_var1_val = unsafe { *pi_var1 };
        LineTo16((local_14 - pi_var1) * 2 + local_1a, x, local_30);
        i_var5 = ((local_16 - local_3a) * 2 + local_2a) + local_a;
        MoveToEx16(
            CONCAT22(unaff_ss, local_38),
            (pi_var1_val * 2 + local_1a),
            i_var5,
            local_30,
        );
        LineTo16((local_14 - pi_var1_val) * 2 + local_1a, i_var5, local_30);
        SelectObject16(local_34, local_30);
        DeleteObject16(local_32);
        local_3a = local_3a - 1;
        local_3a < 0x8000
    } {}
    (i32_var6 + 0x10) = 0;
}

pub fn draw_func_1020_81c0(ctx: &mut AppContext, param_1: u32) {
    let mut u_var1: u32;
    let mut hdc: HDC16;
    let mut h_palette: HDC16;
    let mut h_palette_00: HPALETTE16;
    let mut u_var2: u16;
    let mut h_dc: HDC16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

  // u_var2 = (ctx._PTR_LOOP_1050_4230  >> 0x10);
    u_var1 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    local_6._0_2_ = u_var1;
    if (((ctx._PTR_LOOP_1050_4230 + 0x10) | local_6) == 0) {
        return;
    }
    hdc = GetDC16((param_1 + 8));
    h_palette = hdc;
    h_dc = hdc;
    palette::create_palette_1008_4e38(u_var1);
    h_palette_00 = SelectPalette16(0, h_palette, h_dc);
    RealizePalette16(hdc);
    SelectPalette16(1, h_palette_00, hdc);
    RealizePalette16(hdc);
    DeleteObject16(h_palette);
    if (0 < h_palette) {
        InvalidateRect16(1, 0x0, 0);
    }
    return;
}

pub fn draw_1040_8a06(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut h_dc: u16;
    let dev_ctx_handle: &mut  u16;
    let obj_handle: HANDLE16;
    let h_gdi_obj: HPALETTE16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let count: u16;
    let unaff_ss: HWND16;
    let mut u_var5: u32;
    let CVar6: COLORREF;
    let color: COLORREF;
    let u_stack68: u8;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let local_22: &mut  PAINTSTRUCT16;
    let iStack30: u16;

  // count = (param_1  >> 0x10);
    i_var4 = param_1;
    local_24 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var4 + 6));
    u_var5 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    local_28._0_2_ = u_var5;
    local_28 = (u_var5 >> 0x10);
    dev_ctx_handle = &local_24;
    palette::realize_palette_1008_4e08(local_28, local_28, dev_ctx_handle, unaff_ss);
    u_var5 = pass1_1008_4d72(u_var5);
  // u_var3 = (u_var5  >> 0x10);
    i_var2 = u_var5;
    DrawIcon16(
        (i_var4 + 0x8e),
        CONCAT214(
            local_3c,
            CONCAT212(local_3e, CONCAT66(u_stack68, CONCAT24(local_24, 0x14000a))),
        ),
        CONCAT214(
            u_var3,
            CONCAT212(
                i_var2,
                CONCAT210(
                    CONCAT11(2, *(i_var2 + 0x94)),
                    CONCAT28(
                        CONCAT11(*(i_var2 + 0x95), *(i_var2 + 0x96)),
                        CONCAT26(local_34, CONCAT24(local_36, CONCAT22(local_38, local_3a))),
                    ),
                ),
            ),
        ),
        dev_ctx_handle,
    );
    CVar6 = SetBkColor16(0, local_24);
    color = SetTextColor16(0x15388a8c, local_24);
    local_3e = 0;
    obj_handle = GetProp16(s_hfont_1050_5dfa, (i_var4 + 6));
    if (obj_handle != 0) {
        local_3e = SelectObject16(obj_handle, local_24);
    }
    u_var1 = (i_var4 + 0x90);
    local_28._0_2_ = u_var1;
    local_28 = (u_var1 >> 0x10);
    DrawText16(
        0x10,
        (i_var4 + 0x9e),
        count,
        CONCAT22(local_28, 0xffff),
        local_28,
    );
    if (obj_handle != 0) {
        SelectObject16(local_3e, local_24);
    }
    SetBkColor16(CONCAT22(0x8ae7, CVar6), local_24);
    SetTextColor16(color, local_24);
    h_dc = local_24;
    local_22 = local_24;
    local_24 = offset;
    h_gdi_obj = SelectPalette16(0, offset, h_dc);
    local_22 = offset;
    local_24 = 0x8b23;
    DeleteObject16(h_gdi_obj);
    iStack30 = (i_var4 + 6);
    local_22 = &local_22;
    local_24 = offset;
    EndPaint(local_22, unaff_ss);
    return;
}

pub fn draw_1020_30be(in_struct_1: &mut Struct653) {
    let paVar1: &mut Struct510;
    let is_iconic_result: bool;
    let local_struct_1: &mut Struct653;
    let local_struct_1_hi: &mut Struct653;
    let unaff_ss: HWND16;
    let local_struct_2_16: &mut Struct510;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let local_struct_2_16_2: &mut Struct510;
    let local_paint_struct_1: PAINTSTRUCT16;
    let fn_ptr_1: fn();

  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_2_16_2 = BeginPaint16(
        CONCAT22(unaff_ss, &local_paint_struct_1),
        local_struct_1.win_handle_0x4,
    );
    local_struct_2_16 = local_struct_1.win_handle_0x4;
    is_iconic_result = IsIconic16(local_struct_2_16);
    if (is_iconic_result == 0) {
        paVar1 = local_struct_1.struct_ptr_0x14;
        local_struct_2_16 = paVar1;
      // local_3a = (paVar1  >> 0x10);
        pass1_1018_0a50(paVar1);
        local_struct_2_16 = &local_struct_2_16_2;
        fn_ptr_1 = (CONCAT22(unaff_ss, local_struct_2_16) + 8);
        local_3a = unaff_ss;
        (**fn_ptr_1)(0x1018, local_struct_2_16, unaff_ss);
        paVar1 = local_struct_1.struct_ptr_0x14;
        if ((paVar1 + 0x84) == 1) {
            draw_1020_320e(in_struct_1, local_struct_2_16_2);
        }
        fn_ptr_1 = (CONCAT22(local_3a, local_struct_2_16) + 4);
        (**fn_ptr_1)(0x1018, local_struct_2_16, local_3a, 0, 0, 0xdc);
        paVar1 = local_struct_1.struct_ptr_0x14;
        if ((paVar1 + 0x84) != 1) {
            draw_1020_320e(in_struct_1, local_struct_2_16_2);
        }
        rect::draw_rect_1020_3488(in_struct_1, local_struct_2_16_2);
        fn_ptr_1 = (CONCAT22(local_3a, local_struct_2_16) + 0xc);
        (**fn_ptr_1)(0x1018, local_struct_2_16, local_3a, &local_struct_2_16_2);
    } else {
        if (ctx.PTR_LOOP_1050_0010 == 0x0) {
            paVar1 = local_struct_1.struct_ptr_0x14;
            local_struct_2_16 = paVar1;
          // local_3a = (paVar1  >> 0x10);
            fn_ptr_1 = (local_struct_1.struct_ptr_0x14 + 0x2c);
            local_26 = (**fn_ptr_1)(offset);
            if (local_26 != 0) {
                local_3a = 4;
                local_struct_2_16 = offset;
                local_28 = GetStockObject16(4);
                local_38 = local_struct_1.win_handle_0x4;
                local_struct_2_16 = &local_30;
                GetClientRect16(CONCAT22(unaff_ss, local_struct_2_16), local_38);
                local_32 = (local_2c - local_30) - 1;
                local_34 = (local_2a - local_2e) - 1;
                local_36 = local_struct_2_16_2;
                local_3a = &local_struct_2_16;
                local_struct_2_16 = local_28;
                FillRect16(local_28, local_3a, unaff_ss);
                local_36 = local_struct_2_16_2;
                local_3a = 2;
                local_38 = 2;
                local_struct_2_16 = local_26;
                DrawIcon16(
                    local_26,
                    CONCAT214(
                        local_2c,
                        CONCAT212(
                            local_2e,
                            CONCAT210(
                                local_30,
                                CONCAT28(
                                    local_32,
                                    CONCAT26(local_34, CONCAT24(local_struct_2_16_2, 0x20002)),
                                ),
                            ),
                        ),
                    ),
                    CONCAT88(
                        local_paint_struct_1._0_8_,
                        CONCAT26(
                            local_struct_2_16_2,
                            CONCAT24(local_26, CONCAT22(local_28, local_2a)),
                        ),
                    ),
                    local_paint_struct_1.rc_print.right,
                );
            }
        }
    }
    local_3a = local_struct_1.win_handle_0x4;
    EndPaint(&local_paint_struct_1, unaff_ss);
    return;
}

pub fn draw_1020_320e(param_1: &mut Struct653, param_2: u16) {
    let pu_var1: &mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let h_gdi_obj: HPALETTE16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut u_var8: u32;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_2;
  // u_var6 = (param_1  >> 0x10);
    i_var4 = param_1;
    u_var3 = (i_var4 + 0x14);
    if ((u_var3 + 0x84) == 1) {
        u_var3 = (i_var4 + 0x14);
      // u_var7 = (u_var3  >> 0x10);
        i_var5 = u_var3;
        pu_var1 = (i_var5 + 0x24);
        u_var14 = SUB42(s_dib_1050_42c6, 0);
        u_var12 = 0;
        u_var13 = 0;
        u_var9 = 0;
        u_var10 = 0;
        u_var11 = 0;
        u_var8 = process_struct_1008_4772((pu_var1 & 0xffff | *(i_var5 + 0x26) << 0x10));
        local_4 = CreateDC16(
            u_var8,
            CONCAT13(u_var10, CONCAT12(u_var9, (u_var8 >> 0x10))),
            CONCAT22(u_var12, u_var11),
            CONCAT22(u_var14, u_var13),
        );
        local_6 = &local_4;
        unsafe {
            ppc_var2 = (*pu_var1 + 8);
        }
        ppc_var2(offset, pu_var1, (pu_var1 >> 0x10), local_6);
    }
    pass1_1018_0d9a(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var3 = (i_var4 + 0x14);
    u_var3 = (u_var3 + 0x6c);
    draw_1020_33c0(
        param_1,
        u_var3,
        (u_var3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        1,
        local_4,
    );
    pass1_1018_1054(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var3 = (i_var4 + 0x14);
    u_var3 = (u_var3 + 0x74);
    draw_1020_33c0(
        param_1,
        u_var3,
        (u_var3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        2,
        local_4,
    );
    pass1_1018_1320(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var3 = (i_var4 + 0x14);
    u_var3 = (u_var3 + 0x68);
    draw_1020_33c0(
        param_1,
        u_var3,
        (u_var3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        1,
        local_4,
    );
    pass1_1018_15f6(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    if (local_c != 0) {
        u_var3 = (i_var4 + 0x14);
        u_var3 = (u_var3 + 0x70);
        draw_1020_33c0(
            param_1,
            u_var3,
            (u_var3 >> 0x10),
            local_c,
            local_a,
            (local_a >> 0x10),
            1,
            local_4,
        );
    }
    pass1_1018_108c(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    if (local_c != 0) {
        u_var3 = (i_var4 + 0x14);
        u_var3 = (u_var3 + 0x78);
        draw_1020_33c0(
            param_1,
            u_var3,
            (u_var3 >> 0x10),
            local_c,
            local_a,
            (local_a >> 0x10),
            0,
            local_4,
        );
    }
    u_var3 = (i_var4 + 0x14);
    if ((u_var3 + 0x84) == 1) {
        h_gdi_obj = SelectPalette16(0, local_6, local_4);
        DeleteObject16(h_gdi_obj);
        DeleteDC16(local_4);
    }
    return;
}

pub fn draw_1020_33c0(
    param_1: u32,
    color_ref: COLORREF,
    param_3: i32,
    param_4: u32,
    param_5: i32,
    in_dc_handle: HDC16,
) {
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let mut solid_brush: u16;
    let mut solid_brush_obj_handle: u16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let h_gdi_obj: HGDIOBJ16;

    let mut u_var3: u16;
    let h_var4: HDC16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3 != 0) {
        h_var4 = in_dc_handle;
        pen = CreatePen16(color_ref, 1, 0);
        pen_obj_handle = SelectObject16(pen, h_var4);
        h_var4 = in_dc_handle;
        solid_brush = CreateSolidBrush16(color_ref);
        solid_brush_obj_handle = SelectObject16(solid_brush, h_var4);
        local_e = param_4;
        local_14 = 0;
        while (local_14 < param_3) {
          // u_var3 = (param_1  >> 0x10);
            i_var1 = param_3;
            h_var4 = in_dc_handle;
            pass1_1020_3540(param_1, u_var3, param_5, local_e);
            if (param_5 < 1) {
                u_var2 = 3;
            } else {
                u_var2 = 4;
            }
            polygon_1020_3602(param_1, u_var3, u_var2, i_var1, ctx.dx_reg, h_var4);
            error_check_1000_17ce(CONCAT22(ctx.dx_reg, i_var1));
            local_14 = local_14 + 1;
            local_e = local_e & 0xffff0000 | (local_e + 6);
        }
        h_gdi_obj = SelectObject16(solid_brush_obj_handle, in_dc_handle);
        DeleteObject16(h_gdi_obj);
        SelectObject16(pen_obj_handle, in_dc_handle);
        DeleteObject16(pen);
    }
    return;
}

pub unsafe fn draw_fn_1018_dfd4(ctx: &mut AppContext, param_1: &mut  Struct618) -> u16 {
    let mut u_var1: u16;
    let local_Struct618_ptr_1: &mut  Struct618;
    let local_Struct618_ptr_1_hi: &mut  Struct618;
    let pu_var2: Vec<u8>;
    let local_char_ptr_1: String;
    let temp_5fe392f4b5: &mut Vec<u8>;

  // local_Struct618_ptr_1_hi = (param_1  >> 0x10);
    local_Struct618_ptr_1 = param_1;
    temp_5fe392f4b5 = local_Struct618_ptr_1.void_ptr_32_xe2;
  // u_var1 = draw_1018_e16c(temp_5fe392f4b5, (temp_5fe392f4b5  >> 0x10));
    if (local_Struct618_ptr_1.field_0xe6 == 0) {
        local_Struct618_ptr_1.field_0xe6 = 1;
        process_struct_1010_20ba(
            ctx.g_struct_1050_0ed0,
            CONCAT22(local_char_ptr_1, 0x36),
        );
        pu_var2 = pass1_1038_af40(ctx._g_astruct_112_a, local_Struct618_ptr_1.field_0x8, 8);
        u_var1 = SUB42(pu_var2, 0);
    }
    return u_var1;
}

pub fn draw_1018_e16c(param_1: u32) {
    let pu_var1: &mut  u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let h_palette: &mut  u16;
    let mut h_gdi_obj: HPALETTE16;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

  // u_var4 = (param_1  >> 0x10);
    local_24 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (param_1 + 4));
    u_var3 = (param_1 + 6);
    pu_var1 = (u_var3 + 10);
    h_palette = &local_24;
    u_var3 = unsafe { *pu_var1 };
    ppc_var2 = (u_var3 + 8);
    ppc_var2(offset, pu_var1, (pu_var1 >> 0x10), h_palette);
    ppc_var2 = (u_var3 + 4);
    ppc_var2(offset, pu_var1, 0, &local_24);
    h_gdi_obj = SelectPalette16(0, h_palette, local_24);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_fn_1018_e4f2(
    ctx: &mut AppContext,
    param_1: &mut Struct622,
    in_handle_ptr: &mut HANDLE16,
) {
    let pu_var1: &mut  u16;
    let ppVar2: &mut  Struct2551;
    let mut local_handle_1: HANDLE16;
    let mut uStack16: u16;
    let mut local_char_ptr: String = String::new();
    let mut local_6: u32;
    let temp_7e056a7c981: &mut  u32;
    let mut temp_5fb7f3034b: u32;
    let fn_ptr_1: fn();

    // local_handle_1 = in_handle_ptr;
    //// Stack16 = (in_handle_ptr  >> 0x10);
    drawing_context::get_dc_1020_921c(ctx, param_1);
    param_1.u32_0x14 = 0;
    param_1.field_0x0 = 0xe5d0;
    param_1.field_0x2 = 0x1018;
    process_struct_1010_20ba(&mut ctx.g_struct_1050_0ed0, &local_char_ptr);
    // ctx.dx_reg = (ppVar2 >> 0x10);
    param_1.u32_0x14 = ctx._g_struct_371_1050_0ed0;
    (param_1.u32_0x14 + 2) = ctx.dx_reg;
    param_1.u16_0x6 = &param_1.u32_0x14;
    param_1.u16_0x8 = ctx.dx_reg;
    temp_5fb7f3034b = param_1.u32_0x14;
    pu_var1 = &param_1.field_0xa;
    fn_ptr_1 = ((temp_5fb7f3034b + 10) + 8);
    (**fn_ptr_1)();
    param_1.i16_0x12 = pu_var1;
    draw_1020_9364(ctx, param_1);
}

pub unsafe fn draw_fn_1018_ec74(
    ctx: &mut AppContext,
    in_struct_1: &mut  Struct622,
    in_struct_1_hi: &mut  Struct622,
    param_3: u16,
) {
    let mut u_var1: u32;
    let pu_var2: &mut  u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: u16;

    let mut u_var6: u16;
    let mut unaff_si: u16;
    let pp_var7: &mut  Struct2551;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let pa_var10: &mut  Struct622;
    let pa_var11: &mut  Struct622;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    drawing_context::get_dc_1020_921c(ctx, CONCAT22(in_struct_1_hi, in_struct_1), param_3);
    in_struct_1.u32_0x14 = 0;
    zero_list_1008_3e38(CONCAT22(in_struct_1_hi, &in_struct_1.field_0x18));
    zero_list_1008_3e38(CONCAT22(in_struct_1_hi, &in_struct_1.field_0x1e));
    &in_struct_1.field_0x24 = 0;
    CONCAT22(in_struct_1_hi, in_struct_1) = 0x1cc;
    in_struct_1.field_0x2 = 0x1020;
    pp_var7 = process_struct_1010_20ba(ctx.g_struct_1050_0ed0, CONCAT22(unaff_si, 0x28));
  // u_var5 = (pp_var7  >> 0x10);
    &in_struct_1.u32_0x14 = pp_var7;
    (&in_struct_1.u32_0x14 + 2) = u_var5;
    u_var9 = 0;
    u_var8 = &in_struct_1.u32_0x14;
    fn_ptr_1 = (in_struct_1.u32_0x14 + 4);
    pa_var10 = in_struct_1;
    pa_var11 = in_struct_1_hi;
    (**fn_ptr_1)();
    u_var3 = in_struct_1.u32_0x14;
    &in_struct_1.u16_0x6 = u_var3;
    u_var1 = in_struct_1.u32_0x14;
    modify_list_1010_2b50(
        u_var1,
        (u_var1 >> 0x10),
        CONCAT22(in_struct_1_hi, &in_struct_1.field_0x18),
    );
    u_var6 = u_var3;
    pass1_1010_2b66(in_struct_1.u32_0x14);
    in_struct_1.field_0x24 = u_var6;
    in_struct_1.field_0x26 = ctx.dx_reg;
    u_var3 = in_struct_1.u32_0x14;
    pu_var2 = (u_var3 + 10);
    u_var4 = CONCAT22(in_struct_1_hi, &in_struct_1.field_0xa);
    u_var6 = SUB42(pu_var2, 0);
    unsafe {
        fn_ptr_1 = (*pu_var2 + 8);
        (**fn_ptr_1)(
            0x1010,
            u_var6,
            (pu_var2 >> 0x10),
            u_var4,
            u_var8,
            u_var5,
            u_var9,
            pa_var10,
            pa_var11,
        );
    }

    in_struct_1.i16_0x12 = u_var4;
    draw_1020_9364(CONCAT22(in_struct_1_hi, in_struct_1));
    pp_var7 = process_struct_1010_20ba(ctx.g_struct_1050_0ed0, CONCAT22(u_var6, 0x48));
    modify_list_1008_3f62(
        CONCAT22(in_struct_1_hi, &in_struct_1.field_0x1e),
        pp_var7 & 0xffff0000 | (pp_var7 + 0xe),
    );
    pass1_1008_3f32(
        &in_struct_1.field_0x18,
        in_struct_1_hi,
        in_struct_1 + '\x1e',
        in_struct_1_hi,
    );
    return;
}

pub unsafe fn draw_fn_1020_0a52(ctx: &mut AppContext, param_1: &mut  Struct630) {
    let local_bx_4: &mut  Struct630;
    let mut u_var1: u16;
    let pu_var2: Vec<u8>;

  // u_var1 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    draw_1020_0c3e(local_bx_4.field_0xe2);
    if (local_bx_4.field_0xe6 == 0) {
        local_bx_4.field_0xe6 = 1;
        (ctx._g_astruct_112_a + 0xae) = 0x99;
        pu_var2 = pass1_1038_af40(ctx._g_astruct_112_a, local_bx_4.field_0x8, 6);
        local_bx_4.field_0xe8 = pu_var2;
        local_bx_4.field_0xea = (pu_var2 >> 0x10);
    }
    return;
}

pub fn draw_1040_9948(ctx: &mut AppContext, param_1: HWND16, param_2: u32) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut hdc: HDC16;
    let mut mode: u16;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut obj_handle: HPEN16;
    let mut obj_handle_00: HPEN16;
    let mut obj_handle_01: HGDIOBJ16;
    let mut HVar5: HGDIOBJ16;
    let puVar6: Vec<u8>;
    let pu_var7: Vec<u8>;
    let mut i_var8: i32;
    let mut hdc_00: HDC16;
    let mut unaff_ss: HWND16;
    let mut u_var9: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u32;
    let mut local_52: u32;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: PAINTSTRUCT16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = 1;
    local_4 = 1;
    local_a = 0;
    local_8 = 0;
    local_1c = 0;
    local_1e = 0;
  // hdc_00 = (param_2  >> 0x10);
    i_var8 = param_2;
    local_20 = *(i_var8 + 4) & 8;
    local_22 = *(i_var8 + 0x56) & 1;
    hdc = BeginPaint16(CONCAT22(unaff_ss, &local_42), param_1);
    mode = SetMapMode16(1, hdc);
    GetClientRect16(CONCAT22(unaff_ss, &local_12), param_1);
  // i_var2 = (local_e  >> 0x10);
    iVar1 = i_var2 + -1;
    local_e = CONCAT22(iVar1, local_e - 1);
    if (local_22 != 0) {
        local_1a._0_2_ = local_12;
        local_1a = (local_12 >> 0x10);
        local_12 = CONCAT22(local_1a + 2, local_1a + 2);
        local_e = CONCAT22(i_var2 + -3, local_e - 3);
        local_16._0_2_ = local_e - 1;
        local_16 = iVar1;
    }
    if (*(i_var8 + 6) != '\0') {
        local_1c = 1;
        if ((i_var8 + 0x5a) != 0) {
            local_1e = SelectObject16((i_var8 + 0x5a), hdc);
        }
        u_var3 = i_var8 + 6;
        u_var4 = get_string_index_1000_3da4((param_2 & 0xffff0000 | u_var3));
        DrawText16(0x400, &local_a, unaff_ss, CONCAT22(u_var3, u_var4), hdc_00);
        local_8 = ((local_c - local_4) + local_8) / 2 + local_12;
        local_4 = local_4 + local_8;
        local_a = ((local_e - local_6) + local_a) / 2 + local_12;
        local_6 = local_6 + local_a;
    }
    obj_handle = CreatePen16(ctx.u32_1050_5ec2, 1, 0);
    obj_handle_00 = CreatePen16(ctx.u32_1050_5ebe, 1, 0);
    obj_handle_01 = SelectObject16(obj_handle, hdc);
    if (local_22 != 0) {
        local_4e = 0;
        while {
            MoveTo16(local_16, local_1a, hdc);
            LineTo16(local_16, local_16, hdc);
            LineTo16(local_1a, local_16, hdc);
            LineTo16(local_1a, local_1a, hdc);
            LineTo16(local_16, local_1a, hdc);
            local_1a = local_1a + 1;
            local_1a._0_2_ = local_1a + 1;
            local_16._0_2_ = local_16 + -1;
            local_16 = local_16 + -1;
            local_4e = local_4e + 1;
            local_4e < 1
        } {}
    }
    if ((*(i_var8 + 4) & 2) == 0) {
        local_56 = (local_12 >> 0x10);
        local_52._0_2_ = local_e;
        local_4e = 0;
        local_56._0_2_ = local_12;
        local_52 = local_c;
        while {
            SelectObject16(obj_handle, hdc);
            MoveTo16(local_52, local_56, hdc);
            LineTo16(local_52, local_52, hdc);
            LineTo16(local_56, local_52, hdc);
            while {
                SelectObject16(obj_handle_00, hdc);
                LineTo16(local_56, local_56, hdc);
                LineTo16(local_52, local_56, hdc);
                (hdc + 1) < 2
            } {}
            local_56 = local_56 + 1;
            local_56._0_2_ = local_56 + 1;
            local_52._0_2_ = local_52 + -1;
            local_52 = local_52 - 1;
            local_4e = local_4e + 1;
            local_4e < 2
        } {}
    } else {
        MoveTo16(local_c, local_12, hdc);
        LineTo16(local_12, local_12, hdc);
        LineTo16(local_12, local_e + 1, hdc);
        if (local_1c != 0) {
            local_8 = local_8 + 2;
            local_a = local_a + 2;
            local_6 = local_6 + 2;
            local_4 = local_4 + 2;
        }
    }
    MoveTo16(0, 0, hdc);
    if (local_1c != 0) {
        if ((*(i_var8 + 4) & 4) == 0) {
            pu_var7 = ctx.u16_1050_5ec8;
            puVar6 = ctx.PTR_LOOP_1050_5ec6;
            if (local_20 != 0) {
                pu_var7 = ctx.PTR_LOOP_1050_5ecc;
                puVar6 = ctx.u16_1050_5eca;
            }
            SetTextColor16(CONCAT22(pu_var7, puVar6), hdc);
            SetBkColor16(0x1000000, hdc);
            u_var3 = get_string_index_1000_3da4((param_2 & 0xffff0000 | (i_var8 + 6)));
            DrawText16(1, &local_a, unaff_ss, CONCAT22(i_var8 + 6, u_var3), hdc_00);
            SetTextColor16(CONCAT22(hdc, hdc), hdc);
            SetBkColor16(CONCAT22(hdc, hdc), hdc);
        } else {
            HVar5 = GetStockObject16(1);
            u_var9 = 0;
            u_var3 = i_var8 + 6;
            u_var4 = get_string_index_1000_3da4((param_2 & 0xffff0000 | u_var3));
            GrayString16(
                0x1000,
                local_4 - local_8,
                local_6 - local_a,
                local_8,
                local_a,
                u_var4,
                u_var3,
                hdc_00,
                u_var9,
                0,
                HVar5,
                hdc,
            );
        }
        if (local_1e != 0) {
            SelectObject16(local_1e, hdc);
        }
    }
    SelectObject16(obj_handle_01, hdc);
    SetMapMode16(mode, hdc);
    DeleteObject16(obj_handle);
    DeleteObject16(obj_handle_00);
    EndPaint16(&local_42, unaff_ss);
    return;
}

pub fn process_struct_1040_9252(param_1: &mut  Struct357) {
    let pi_var1: &mut  i32;
    let mut i_var2: i32;
    let local_bx_3: &mut  Struct357;
    let mut u_var3: i32;

  // u_var3 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    if (&local_bx_3.field_0x4 != 0) {
        text::draw_text_1040_9650(param_1);
    }
    if (local_bx_3.field_0x8 != 0x0) {
        process_struct_1040_9618((param_1 & 0xffff | u_var3 << 0x10));
    }
    i_var2 = &local_bx_3[1].field_0x4;
    if (&local_bx_3.field_0x22 < i_var2) {
        &local_bx_3.field_0x22 = i_var2;
    }
    i_var2 = &local_bx_3[1].field_0x6;
    if (&local_bx_3.field_0x24 < i_var2) {
        &local_bx_3.field_0x24 = i_var2;
    }
    i_var2 = &local_bx_3.field_0x26 + local_bx_3.field_0x2a;
    if (&local_bx_3.field_0x22 < i_var2) {
        &local_bx_3.field_0x22 = i_var2;
    }
    i_var2 = &local_bx_3.field_0x28 + local_bx_3.field_0x2c;
    if (&local_bx_3.field_0x24 < i_var2) {
        &local_bx_3.field_0x24 = i_var2;
    }
    pi_var1 = &local_bx_3.field_0x22;
    unsafe {
        *pi_var1 = *pi_var1 + &local_bx_3[1].field_0x8;
        pi_var1 = &local_bx_3.field_0x24;
        *pi_var1 = *pi_var1 + &local_bx_3[1].field_0x8;
    }
    return;
}

pub unsafe fn drawing_and_sound_fn_1018_742e(ctx: &mut AppContext, in_struct_129: &mut  Struct129) {
    let mut iVar1: i32;

    paint::begin_end_Paint_1018_6a7a(in_struct_129);
    if (ctx.PTR_LOOP_1050_4254 == 0x0) {
        iVar1 = mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, 0x1e9);
        if (iVar1 != 0) {
            ctx.PTR_LOOP_1050_4254 = (&ctx.PTR_LOOP_1050_0000 + 1);
        }
    }
    return;
}

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
    let pu_var7: &mut  u32;
    let mut y: i32;

    let local_bx_4: &mut  Struct34;
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

  // u_var9 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    b_result = IsIconic16(local_bx_4.h_window);
    if ((b_result == 0) || (ctx.PTR_LOOP_1050_0010 != 0x0)) {
        window_dc = GetWindowDC16(local_bx_4.h_window);
        local_a = (ctx._PTR_LOOP_1050_4230 + 0xe);
        local_c = &window_dc;
        palette::realize_palette_1008_4e08(local_a, (local_a >> 0x10), local_c, h_wnd);
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
      // u_var10 = (u_var3  >> 0x10);
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
      // uStack56 = (u_var3  >> 0x10);
        ppc_var2 = (local_bx_4.field_0x6 + 0x18);
        ppc_var2(offset, uStack58, uStack56);
        let pc_var5_val = unsafe { *pcVar5 };
        if (pc_var5_val != '\0') {
            SetBkColor16(0, window_dc);
            SetTextColor16(CONCAT22(0x100, i_var8), window_dc);
            i32_var6 = lstrlen16(CONCAT22(ctx.dx_reg, pcVar5));
            dVar11 = GetTextExtent16(i32_var6, CONCAT22(ctx.dx_reg, pcVar5), window_dc);
          // i_var8 = (dVar11  >> 0x10);
            if (b_result == 0) {
                local_42 = (i_var4 - iVar1) / 2 - i_var8 / 2;
                h_dc = x / 2 - dVar11 / 2;
            } else {
                local_42 = y / 2 - i_var8 / 2;
                h_dc = 2;
            }
            local_3e._0_1_ = ctx.dx_reg;
            local_3e = (ctx.dx_reg >> 8);
            TextOut16(
                local_42,
                CONCAT13(local_3e, CONCAT12(local_3e, pcVar5)),
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

pub fn call_draw_fn_1020_79b4(param_1: &mut  Struct674, param_2: u16, param_3: String) {
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

pub fn draw_1020_650c(param_1: &mut  Struct622) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut local_28: PAINTSTRUCT16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut offset: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    u_var2 = (i_var3 + 0x14);
    local_6 = (u_var2 + 10);
    if (((i_var3 + 0x10) != 0) || (u_var2 = (i_var3 + 0x14), (u_var2 + 0x24) != 0)) {
        draw_1020_9364(param_1);
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

pub unsafe fn draw_1020_40ce(
    param_1: u32,
    param_2: &mut  Struct132,
    param_3: &mut  Struct134,
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
    let temp_5f8e606965: &mut  Struct133;

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

pub unsafe fn draw_1020_3da4(ctx: &mut AppContext, param_1: &mut  u16, param_2: u32) {
    let pu_var1: &mut  u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: u16;
    let mut HVar5: HGDIOBJ16;
    let puVar6: &mut  u16;
    let mut h_dc: HDC16;
    let pp_var7: &mut  Struct2551;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut h_dc_00: HDC16;
    let mut local_16: u16;
    let mut local_8: u32;
    let mut local_4: u16;
    let mut offset: u16;

    i_var8 = param_1;
  // u_var9 = (param_1  >> 0x10);
    get_sys_metrics_1020_7c1a(i_var8, u_var9, param_2, (param_2 >> 0x10));
    (i_var8 + 0x14) = 0;
    unsafe { *param_1 = 0x408a };
    (i_var8 + 2) = 0x1020;
    pp_var7 = process_struct_1010_20ba(ctx.g_struct_1050_0ed0, CONCAT22(local_16, 6));
  // h_dc = (pp_var7  >> 0x10);
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

pub unsafe fn draw_1020_3e84(ctx: &mut AppContext, in_struct_1: &mut  Struct45) {
    let mut h_dc: HDC16;
    let pu_var1: Vec<u8>;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1: &mut  Struct45;
    let local_struct_1_hi: &mut  Struct45;
    let mut local_4: u16;

  // local_struct_1_hi = (in_struct_1  >> 0x10);
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

  // u_var4 = (param_1  >> 0x10);
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

pub unsafe fn call_draw_fn_1020_27b0(ctx: &mut AppContext, param_1: &mut  Struct650, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let ppVar5: &mut  Struct2551;
    let mut u_var6: u16;
    let mut in_stack_0000fff2: u16;
    let mut local_6: u32;

    u_var6 = param_2;
    drawing_context::get_dc_1020_921c(CONCAT22(u_var6, param_1), (param_2 >> 0x10));
    &param_1.field_0x14 = 0;
    CONCAT22(u_var6, param_1) = (ctx.s_fem74_wav_1050_2888 + 6);
    param_1.field_0x2 = 0x1020;
    ppVar5 = process_struct_1010_20ba(
        ctx.g_struct_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x2a),
    );
  // u_var4 = (ppVar5  >> 0x10);
    param_1.field_0x14 = ppVar5;
    &param_1.field_0x16 = u_var4;
    param_1.field_0x6 = param_1.field_0x14;
    param_1.field_0x8 = u_var4;
    u_var2 = &param_1.field_0x14;
    i_var3 = &param_1.field_0xa;
    pp_var1 = ((u_var2 + 10) + 8);
    (**pp_var1)();
    param_1.field_0x12 = i_var3;
    draw_1020_9364(CONCAT22(u_var6, param_1));
    return;
}

pub fn draw_icon_1020_1674(ctx: &mut AppContext, in_struct_1: &mut  Struct14) {
    let mut icon_handle: HICON16;
    let mut h_brush: HGDIOBJ16;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let local_struct_1: &mut  Struct14;
    let local_struct_1_hi: &mut  Struct14;
    let mut unaff_ss: HANDLE16;
    let uStack34;
    let mut local_1c: u16;
    let mut rect16_2: RECT16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut rect16_1: RECT16;
    let mut local_6: u16;
    let mut local_4: u16;
    // fn_ptr_1: &mut  Vec<u8>;
    let local_struct_2: &mut  Struct417;
    let mut u_var1: u16;
    let mut fn_ptr_1: fn();

    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
      // local_struct_1_hi = (in_struct_1  >> 0x10);
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

pub unsafe fn draw_1020_2020(ctx: &mut AppContext, param_1: &mut  Struct647) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut h_var4: HDC16;
    let pu_var5: &mut  u16;
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

  // u_var11 = (param_1  >> 0x10);
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
      // u_var9 = (u_var13  >> 0x10);
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
        window::win_gui_fn_1020_2488(param_1);
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

pub fn draw_1020_229c(param_1: u32, param_2: HDC16) {
    let mut iVar1: i32;
    let pi_var2: &mut  u16;
    let mut u_var3: u32;
    let mut x: i32;
    let mut i_var4: i32;
    let pi_var5: &mut  i32;
    let mut u_var6: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

  // u_var6 = (param_1  >> 0x10);
    u_var3 = (param_1 + 6);
    iVar1 = (u_var3 + 0x30);
    u_var3 = (param_1 + 6);
    pi_var2 = (u_var3 + 0x1a);
    let pi_var2_val = unsafe { *pi_var2 };
    MoveTo16(5, pi_var2_val, param_2);
  // u_var6 = (pi_var2  >> 0x10);
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
      // u_var1 = (param_1  >> 0x10);
        ctx.dx_ax_reg = pass1_1020_23f2(param_1, u_var1, CONCAT22(ctx.stack_seg_reg, local_a));
        polygon::polygon_1020_2474(
            param_1,
            u_var1,
            CONCAT22(ctx.dx_ax_reg, 3),
            (ctx.dx_ax_reg >> 0x10),
        );
    }
    return;
}

pub fn call_draw_1020_2c72(param_1: &mut Vec<u8>) {
    draw_1020_30be((param_1 + 0xf6));
    return;
}

pub fn pass1_1020_6e52(ctx: &mut AppContext, param_1: &mut  Struct674, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let lVar2: u32;
    let pcVar3: String;

    u_var1 = param_1.field_0xf2;
  // lVar2 = pass1_1018_2e5e(u_var1, (u_var1  >> 0x10));
    if (lVar2 == 0) {
        pcVar3 = load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
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

pub fn set_capture_1020_6184(in_struct_1: &mut  Struct661, param_2: u16) {
    let mut cursor: HCURSOR16;
    let local_struct_1: &mut  Struct661;
    let local_struct_1_hi: &mut  Struct661;

  // local_struct_1_hi = (in_struct_1  >> 0x10);
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

pub unsafe fn pass1_1020_52de(in_struct_1: &mut  Struct594) {
    // fn_ptr_2: &mut  Vec<u8>;
    let local_struct_1_2: &mut  Struct594;
    let local_struct_1: &mut  Struct594;
    let local_struct_1_hi: &mut  Struct594;
    let mut in_stack_0000fff2: u16;
    // fn_ptr_1: &mut  Vec<u8>;
    let mut fn_ptr_1: fn();
    let mut fn_ptr_2: fn();
    let temp_5f6e246310: &mut  u32;
    let mut local_u16_2: u16;

  // local_struct_1_hi = (in_struct_1  >> 0x10);
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

pub unsafe fn pass1_1020_3c74(param_1: u16, param_2: u32, param_3: u32) {
    rect::call_pt_in_rect_fn_1020_3c8c(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
    );
    return;
}

pub unsafe fn pass1_1020_1b68(param_1: &mut  Struct409) {
    let pu_var1: &mut  u32;
    let pvVar2: &mut Vec<u8>;
    let local_struct_1: &mut  Struct641;
    let local_struct_1_hi: &mut  Struct409;
    let fn_ptr_1: fn();

  // local_struct_1_hi = (param_1  >> 0x10);
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

pub unsafe fn pass1_1020_1da8(in_struct_1: &mut  Struct643) -> bool {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let local_struct_1: &mut  Struct643;
    let local_struct_1_hi: &mut  Struct643;
    let mut temp_5fb951b2a7: u32;

  // local_struct_1_hi = (in_struct_1  >> 0x10);
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

pub unsafe fn pass1_1020_1f74(ctx: &mut AppContext, param_1: &mut  Struct376) {
    let local_struct_1: &mut  Struct376;
    let local_struct_1_hi: &mut  Struct376;

  // local_struct_1_hi = (param_1  >> 0x10);
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

pub fn pass1_1020_2286(param_1: u16, param_2: u16, param_1_00: &mut  i32, param_2_00: i32) {
    unsafe {
        *param_1_00 = 100 - param_2_00 >> 1;
    }
    return;
}

pub unsafe fn pass1_1020_23f2(param_1: u16, param_2: u16, param_1_00: &mut  u16) {
    let mut u_var1: u16;
    let local_struct_1: &mut  Struct645;
    let mut unaff_ss: u16;
    let mut u_var2: u32;
    let local_12: &mut  Struct646;
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
  // u_var1 = (u_var2  >> 0x10);
    local_12 = 0x0;
    while (local_12 < 3) {
        local_struct_1 = (local_12 * 4);
        (local_struct_1 + u_var2) = (local_struct_1 + 0x4270) + local_4;
        (local_struct_1 + u_var2 + 2) = (local_struct_1 + 0x4272) + local_6;
        local_12 = &local_12.field_0x1;
    }
    return u_var2;
}
