use crate::app_context::AppContext;
use crate::draw::draw2;
use crate::err_funcs::error_check_1000_17ce;
use crate::file_funcs::file1::{close_file_1008_496c, read_from_file_1008_49e8};
use crate::list_funcs::modify_list_1010_2b50;
use crate::mixed_fn_1010_830a;
use crate::other_funcs::{modify_list_1008_3f62, zero_list_1008_3e38};
use crate::pass::pass13_funcs::pass1_1008_9f48;
use crate::pass::pass14_funcs::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3f32, pass1_1008_3f92, pass1_1008_405c, pass1_1008_43cc, pass1_1008_4480, pass1_1008_4d72, pass1_1008_4d84, pass1_1008_5068, pass1_1008_5134, pass1_1008_5236, pass1_1008_57a4, pass1_1008_5b12};
use crate::pass::pass20_funcs::{pass1_1010_a5ca, pass1_1018_0afa};
use crate::pass::pass7_funcs::{draw_1018_6444, pass1_1018_1eda, pass1_1018_265c, pass1_1018_266a, pass1_1018_268e, pass1_1018_642e, pass1_1018_6544};
use crate::pass::pass8_funcs::{pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_209e, pass1_1010_2b66, pass1_1010_2b78, pass1_1010_2b98, pass1_1010_2ee2, pass1_1010_4c2c, pass1_1010_4dc8, process_struct_1010_20ba};
use crate::pass::pass_funcs::pass1_fn_1000_484c;
use crate::prog_structs::prog_structs_1::{Struct104, Struct197, Struct538, Struct693, Struct694};
use crate::prog_structs::prog_structs_12::Struct102;
use crate::prog_structs::prog_structs_16::{Struct115, Struct13};
use crate::prog_structs::prog_structs_17::Struct590;
use crate::prog_structs::prog_structs_2::{Struct199, Struct318, Struct7};
use crate::prog_structs::prog_structs_20::Struct30;
use crate::prog_structs::prog_structs_22::Struct69;
use crate::prog_structs::prog_structs_23::Struct56;
use crate::prog_structs::prog_structs_24::Struct129;
use crate::prog_structs::prog_structs_25::Struct65;
use crate::prog_structs::prog_structs_26::{Struct54, Struct55};
use crate::prog_structs::prog_structs_28::Struct357;
use crate::prog_structs::prog_structs_29::{Struct136, Struct36};
use crate::prog_structs::prog_structs_30::{Struct137, Struct14, Struct35};
use crate::prog_structs::prog_structs_31::{Struct138, Struct139, Struct28, Struct29, Struct33, Struct334};
use crate::prog_structs::prog_structs_6::{Struct622, Struct670};
use crate::prog_structs::prog_structs_7::{Struct135, Struct31, Struct376, Struct44, Struct618, Struct631, Struct632};
use crate::prog_structs::prog_structs_8::Struct642;
use crate::prog_structs::prog_structs_9::{Struct604, Struct630, Struct633, Struct634};
use crate::sound_funcs::{mci_send_cmd_1008_5c5c, mci_send_command_1008_5c9e};
use crate::string_ops1::{copy_string_1000_3d3e, get_string_index_1000_3da4, string_fn_1000_3f9c};
use crate::struct_ops1::{process_struct_1000_179c, process_struct_1008_41bc, process_struct_1008_4772, process_struct_1008_47cc, process_struct_1008_4834, process_struct_1008_48fe, process_struct_1008_4c58, process_struct_1008_50c2, process_struct_1010_4d5c, set_struct_1008_4016};
use crate::struct_ops2::process_struct_1040_9618;
use crate::sys1::{reg_class_1008_96d2, win_func_1018_6bb6};
use crate::sys_structs::{PAINTSTRUCT16, POINT16, RECT16};
use crate::typedefs::{COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HGDIOBJ16, HPALETTE16, HPEN16, HWND16};
use crate::ui_funcs::ui1::{load_cursor_1008_61b2, mixed_1040_8520, set_window_text_1018_6630, win_fn_1018_6adc, win_gui_fn_1010_8170};
use crate::ui_funcs::ui2::pass1_1038_af40;
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};
use crate::winapi_funcs::{BeginPaint16, CreateDC16, CreatePalette16, CreatePen16, CreateSolidBrush16, DeleteDC16, DeleteObject16, DestroyWindow16, DrawText16, EndPaint16, FillRect16, FrameRect16, GetClientRect16, GetCurrentPosition16, GetDC16, GetDlgCtrlID16, GetProp16, GetStockObject16, GetSystemMetrics16, GetTextExtent16, GetWindowDC16, GetWindowLong16, GetWindowRect16, GrayString16, InvalidateRect16, IsIconic16, IsWindow16, LineTo16, LoadCursor16, lstrlen16, MoveTo16, MoveToEx16, Polygon16, PostMessage16, PtInRect16, RealizePalette16, Rectangle16, ReleaseDC16, ScreenToClient16, SelectObject16, SelectPalette16, SetBkColor16, SetMapMode16, SetTextColor16, TextOut16, UnrealizeObject16};

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
    let mut local_22: PAINTSTRUCT16;

    // u_var9 = (param_1 >> 0x10);
    // u_var8 = param_1;
    h_var4 = BeginPaint16(&local_22, param_1.field_0x4);
    local_24 = h_var4;
    pass1_1010_4c2c(param_1.field_0x6);
    let local_28 = CONCAT22(ctx.dx_reg, h_var4);
    pu_var5 = &local_24;
    // pp_var1 = (*local_28 + 8);
    pp_var1(0x1010, h_var4, ctx.dx_reg, pu_var5, unaff_ss);
    (u_var8 + 0x10) = pu_var5;
    u_var2 = (u_var8 + 6);
    local_2a = (u_var2 + 0x30);
    u_var2 = (u_var8 + 6);
    local_2e = (u_var2 + 0x12);
    let local_32 = 0x140000;
    u_var10 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    zero_list_1008_3e38(CONCAT22(unaff_ss, local_38));
    local_3a = 0;
    while (local_3a < local_2a) {
        u_var11 = process_struct_1008_4772((local_3a * 4 + local_2e));
        u_var7 = (u_var11 >> 0x10);
        i_var3 = u_var11;
        pass1_1018_642e(
            u_var8,
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
    h_var6 = SelectObject16(obj_handle, local_24);
    obj_handle_00 = CreateSolidBrush16(0x1000025);
    obj_handle_01 = SelectObject16(obj_handle_00, local_24);
    draw_1018_6444(u_var8, (param_1 >> 0x10), local_24);
    u_var13 = local_24;
    u_var12 = pass1_1010_4dc8((u_var8 + 6));
    pass1_1018_6544(param_1, u_var12, u_var13);
    set_window_text_1018_6630(param_1);
    h_gdi_obj = SelectPalette16(0, (u_var8 + 0x10), local_24);
    DeleteObject16(h_gdi_obj);
    h_var6 = SelectObject16(h_var6, local_24);
    DeleteObject16(h_var6);
    h_var6 = SelectObject16(obj_handle_01, local_24);
    DeleteObject16(h_var6);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub fn polygon_1018_661c(param_1: u16, param_2: u16, in_count: u16, in_Paint16: *mut POINT16) {
    let mut hdc: HDC16;

    Polygon16(in_count, in_Paint16, hdc);
    return;
}

pub fn invalidate_rect_1018_5d32(param_1: u32, param_2: i32) {
    let mut hwnd: HWND16;

    hwnd = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 2) {
        return;
    }
    InvalidateRect16(0, (param_1 + 0x22), hwnd);
    return;
}

pub unsafe fn draw_1018_5d6c(ctx: &mut AppContext, param_1: u32) {
    let pu_var1: *mut u32;
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

    u_var5 = (param_1 >> 0x10);
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
    let pu_var4: *mut u32;
    let pu_var5: *mut u16;
    let mut i32_var6: i32;
    let mut in_eax: u32;

    let ctx.dx_reg: *mut Struct199;
    let pa_var7: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;

    let ctx.dx_reg: *mut Struct199;
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

    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, param_2);
    let local_6 = (in_eax & 0xffff | ctx.dx_reg << 0x10);
    ppc_var3 = (*local_6 + 0x14);
    (**ppc_var3)(0x1010, in_eax, ctx.dx_reg);
    pu_var4 = in_eax;
    let local_a = in_eax & 0xffff | ZEXT24(ctx.dx_reg) << 0x10;
    u_var9 = (param_1 >> 0x10);
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

pub unsafe fn pt_in_rect_1018_1bda(param_1: *mut Struct318, param_2: u16, param_3: u16) {
    let pu_var1: *mut u16;
    let mut i_var2: i32;
    let b_var3: bool;
    let paVar4: *mut Struct199;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_e = 0;
    i_var2 = param_1;
    pass1_1008_3e94(
        (i_var2 + 0x3a),
        CONCAT22(unaff_ss, &local_14),
        CONCAT22(unaff_ss, &local_12),
    );
    let local_18 = CONCAT22(param_2, param_3);
    local_10 = 0;
    local_1a = 0;
    while (true) {
        u_var5 = (param_1 >> 0x10);
        pu_var1 = (i_var2 + 0x44);
        let var = unsafe { *pu_var1 };
        if (var == local_1a || var < local_1a) {
            return;
        }
        paVar4 = ((i_var2 + 0x40) + local_1a * 0x18);
        local_e = CONCAT22((i_var2 + 0x42), paVar4);
        pass1_1008_3e94(
            paVar4,
            CONCAT22(unaff_ss, &local_8),
            CONCAT22(unaff_ss, &local_a),
        );
        local_a = local_a + (local_12 - 6);
        local_6 = local_a + 0xc;
        local_8 = local_8 + (local_14 - 6);
        local_4 = local_8 + 0xc;
        b_var3 = PtInRect16(local_18, &local_a);
        if (b_var3 != 0) {
            break;
        }
        local_1a = local_1a + 1;
    }
    pass1_1018_1eda(param_1, local_e);
    return;
}

pub fn get_gui_dc_1018_4db0(in_struct_1: Vec<u8>, in_win_handle: HWND16) -> *mut Struct199 {
    let dev_ctx_handle: *mut Struct199;
    let local_struct_1_hi: Vec<u8>;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    (in_struct_1 + 0x16) = in_win_handle;
    dev_ctx_handle = GetDC16(in_win_handle);
    (in_struct_1 + 0x14) = dev_ctx_handle;
    return dev_ctx_handle;
}

pub unsafe fn create_drawing_dc_1018_4e04(
    ctx: &mut AppContext,
    param_1: *mut Struct115,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) {
    let pp_var1: fn();
    let mut dc_handle: u16;
    let mut hdc_ptr: u16;
    let local_bx_8: *mut Struct115;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut func_ptr: u32;
    let mut inner_struct: u32;
    let mut offset: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_8 = param_1;
    pp_var1 = (param_1 + 0x14);
    (**pp_var1)();
    u_var3 = process_struct_1008_4772(local_bx_8.field_0xa);
    pass1_1008_43cc(local_bx_8.field_0xa);
    dc_handle = CreateDC16(u_var3, (u_var3 >> 0x10), 0x0, 0x42340000);
    local_bx_8.dc_handle = dc_handle;
    hdc_ptr = &local_bx_8.dc_handle;
    inner_struct = local_bx_8.field_0xa;
    func_ptr = (local_bx_8.field_0xa + 8);
    (**func_ptr)(
        offset,
        inner_struct,
        (inner_struct >> 0x10),
        hdc_ptr,
        u_var2,
        param_1,
        param_2,
    );
    local_bx_8.field_0x1a = hdc_ptr;
    if ((ctx.WORD_1050_422e != 0) && (0x280 < param_4)) {
        local_10 = 0;
        while (local_10 < (ctx.u16_1050_4216 + 1)) {
            (&ctx.PTR_BYTE_1050_0009_1050_4172 + local_10 * 2) =
                (((&ctx.PTR_BYTE_1050_0009_1050_4172 + local_10 * 2) * (param_4 + 1)) / 0x280);
            local_10 = local_10 + 1;
        }
        local_10 = 0;
        while (local_10 < (ctx.WORD_1050_422c + 1)) {
            (&ctx.WORD_1050_419a + local_10 * 2) =
                (((&ctx.WORD_1050_419a + local_10 * 2) * (param_3 + 1)) / 0x1e0);
            local_10 = local_10 + 1;
        }
    }
    ctx.WORD_1050_422e = 0;
    return;
}

pub unsafe fn pt_in_rect_1010_40f8(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: *mut POINT16,
) -> u16 {
    let pi_var1: *mut i32;
    let ppc_var2: fn();
    let b_var3: bool;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    let pu_var9: *mut u16;
    let mut in_stack_0000ffec: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    loop {
        u_var7 = (param_1 >> 0x10);
        i32_var6 = param_1;
        pi_var1 = (i32_var6 + 0x74);
        let pi_var1_val = unsafe { *pi_var1 };
        if (pi_var1_val == local_6 || pi_var1_val < local_6) {
            // LAB_1010_413e:
            if ((local_6._2_2_ != 0) && (3 < ctx.PTR_LOOP_1050_3960)) {
                pp_var8 = process_struct_1010_20ba(
                    ctx._g_Struct372_1050_0ed0,
                    CONCAT22(in_stack_0000ffec, local_6 + 0xc),
                );
                pu_var9 = pass1_1018_0afa(pp_var8);
                if (pu_var9 == 0) {
                    local_6 = local_6 & 0xffff;
                    u_var7 = 0x1000;
                    process_struct_1000_179c(0xb4, (pu_var9 >> 0x10));
                    u_var5 = (pu_var9 >> 0x10) | pu_var9;
                    if (pu_var9 == 0x0) {
                        u_var4 = 0;
                        u_var5 = 0;
                    } else {
                        u_var7 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                        u_var4 = mixed_1040_8520(pu_var9, ctx.g_h_window, 0x30, 2, 0x643, 0x645);
                    }
                    let local_10 = CONCAT22(u_var5, u_var4);
                    ppc_var2 = (*local_10 + 0x74);
                    ppc_var2(u_var7, u_var4, u_var5);
                    pass1_1010_209e(ctx._g_Struct372_1050_0ed0, local_6 + 0xc);
                }
            }
            if (local_6._2_2_ != 0) {
                return local_6;
            }
            return 0xffff;
        }
        let param_2_val = unsafe { param_2 };
        b_var3 = PtInRect16(
            param_2_val,
            ((local_6 * 0x10 + (i32_var6 + 0x24)) * 8 + (i32_var6 + 0x70)),
        );
        if (b_var3 != 0) {
            local_6 = CONCAT22(1, local_6);
            // goto LAB_1010_413e;
        }
        local_6 = (local_6 + 1);
    }
}

pub unsafe fn draw_1010_47ae(param_1: u32) {
    let local_struct_1: *mut Struct30;
    local_struct_1 = 0x0;
    while {
        draw_1010_47d0(param_1, local_struct_1);
        local_struct_1 = local_struct_1.field_0x1;
        local_struct_1 < 0x10
    } {}
}

pub unsafe fn draw_1010_47d0(ctx: &mut AppContext, param_1: u32, param_2: *mut Struct30) {
    let pu_var1: *mut u16;
    let pu_var2: *mut u32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let ppc_var5: fn();
    let pa_var6: *mut Struct30;
    let h_palette: *mut u16;
    let mut obj_handle: HGDIOBJ16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut i_var7: i32;
    let pu_var8: *mut u16;
    let mut h_gdi_obj: HPALETTE16;
    let mut extraout_d_x: u16;
    let local_b_x_49: *mut Struct29;
    let mut i_var9: i32;
    let local_s_i_120: *mut Struct31;
    let local_s_i_293: *mut Struct28;
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
    u_var10 = (param_1 >> 0x10);
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
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
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
    local_10 = (u_var12 >> 0x10);
    dev_mode = u_var12;
    h_dc = CreateDC16(
        dev_mode,
        CONCAT13(u_var14, CONCAT12(u_var13, local_10)),
        CONCAT22(u_var11, CONCAT11(u_var16, u_var15)),
        CONCAT22(u_var18, u_var17),
    );
    u_var4 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    h_palette = &h_dc;
    realize_palette_1008_4e08(u_var4, (u_var4 >> 0x10), h_palette, unaff_s_s);
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
            u_var11 = (u_var12 >> 0x10);
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

pub fn pt_in_rect_1010_4e08(in_struct_1: *mut Struct642, param_2: u16, param_3: u16) {
    let pi_var1: *mut i32;
    let mut b_var2: bool;
    let mut point_in_rect_result: bool;
    let local_struct_1: *mut Struct642;
    let local_struct_1_hi: *mut Struct642;
    let mut local_c: u32;
    let mut point_1: POINT16;
    let mut local_4: u16;

    point_1 = CONCAT22(param_2, param_3);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    &local_struct_1.field_0x22 = local_struct_1.field_0x20;
    b_var2 = false;
    local_struct_1.field_0x24 = 0;
    local_c = 0;
    loop {
        pi_var1 = local_struct_1.field_0x30;
        if (pi_var1 == local_c || pi_var1 < local_c) {
            // LAB_1010_4e67:
            if (local_c._2_2_ != 0) {
                local_struct_1.field_0x20 = local_c._2_2_;
            }
            if (b_var2) {
                return;
            }
            return;
        }
        point_in_rect_result = PtInRect16(point_1, (local_struct_1.field_0x1a + local_c * 8));
        if (point_in_rect_result != 0) {
            local_c = local_c << 0x10;
            b_var2 = true;
            // goto LAB_1010_4e67;
        }
        local_c = (local_c + 1);
    }
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

    u_var4 = (param_1 >> 0x10);
    if ((param_1 + 0x8e) == 0) {
        HVar2 = CreateSolidBrush16(0);
        (param_1 + 0x8e) = HVar2;
    }
    if (ctx._PTR_LOOP_1050_5efa == 0) {
        u_var5 = pass1_1008_4d72((ctx._PTR_LOOP_1050_4230 + 0xe));
        u_var4 = (u_var5 >> 0x10);
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
    u_var6 = (ctx._PTR_LOOP_1050_5efa >> 0x10);
    // LAB_1040_b41a:
    SetTextColor16(CONCAT22(u_var6, u_var4), param_4);
    SetBkColor16(0x1000000, param_4);
    return;
}

pub unsafe fn draw_text_1040_c94a(
    ctx: &mut AppContext,
    param_1: *mut Struct334,
    param_2: u16,
    param_3: u16,
) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000fff0: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    if (param_1.field_0x48 != 0) {
        ppVar4 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff0, 3));
        u_var2 = param_1.field_0x42;
        u_var1 = (u_var2 + 0x12);
        i_var3 = pass1_1010_a5ca(ppVar4, (ppVar4 >> 0x10), u_var1);
        if (i_var3 == -1) {
            param_1.field_0x3c = 0xf9;
        } else {
            if (i_var3 == 0) {
                param_1.field_0x3c = 0x25;
                if ((u_var1 == 0x5b) || (u_var1 == 9)) {
                    param_1.field_0x3c = 0xfe;
                }
            } else {
                param_1.field_0x3c = 0xfb;
            }
        }
    }
    draw_text_1040_94fc(param_1, param_2, param_3);
    return;
}

pub fn draw_1040_c74c(ctx: &mut AppContext, param_1: *mut u32, param_2: u32) {
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

    u_var4 = (ctx._PTR_LOOP_1050_4230 >> 0x10);
    local_6 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    local_8 = realize_palette_1008_4e08(
        local_6,
        (ctx._PTR_LOOP_1050_4230 + 0x10),
        &param_2 + 2,
        unaff_ss,
    );
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x46) = 1;
    local_a = GetStockObject16(4);
    local_c = CreatePen16(0x1000002, 1, 0);
    local_e = SelectObject16(local_a, param_2._2_2_);
    local_10 = SelectObject16(local_c, param_2._2_2_);
    Rectangle16((i_var3 + 0x24), (i_var3 + 0x22), 0, 0, param_2._2_2_);
    MoveTo16(0, (i_var3 + 0x36) * 2 + (i_var3 + 0x2a), param_2._2_2_);
    LineTo16(
        (i_var3 + 0x24),
        (i_var3 + 0x36) * 2 + (i_var3 + 0x2a),
        param_2._2_2_,
    );
    SelectObject16(local_e, param_2._2_2_);
    h_gdi_obj = SelectObject16(local_10, param_2._2_2_);
    DeleteObject16(h_gdi_obj);
    u_var2 = unsafe { *param_1 };
    pp_var1 = (u_var2 + 0x10);
    (**pp_var1)(offset, i_var3, u_var4, param_2);
    pp_var1 = (u_var2 + 0x14);
    (**pp_var1)(offset, i_var3, (param_1 >> 0x10), param_2._2_2_);
    (i_var3 + 0x46) = 0;
    h_gdi_obj_00 = SelectPalette16(0, local_8, param_2._2_2_);
    DeleteObject16(h_gdi_obj_00);
    return;
}

pub unsafe fn draw_lines_1040_c38e(in_Struct135: *mut Struct135, in_hdc_2: u16) {
    let mut i_var1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let local_a_x_262: *mut Struct137;
    let pi_var6: *mut u16;
    let mut extraout_d_x: u16;
    let local_Struct135: *mut Struct135;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut d_var9: u32;
    let mut curr_pos_10: u32;
    let mut local_1e: u16;
    let mut counter: u16;
    let mut local_18: u16;
    let mut x_coord: u16;
    let mut y_coord: u16;
    let mut local_10: u16;
    let local_e: *mut Struct136;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (in_Struct135 >> 0x10);
    local_Struct135 = in_Struct135;
    u_var2 = local_Struct135.field_0x6;
    i_var1 = (u_var2 + 0x18);
    if ((i_var1 != 0) && (u_var2 = local_Struct135.field_0x6, (u_var2 + 0x16) != 0)) {
        i_var4 = i_var1;
        pass1_1010_2ee2(local_Struct135.field_0x6);
        counter = 0;
        while (counter < i_var1) {
            u_var2 = (counter * 4 + i_var4);
            i_var5 = u_var2;
            u_var2 = u_var2 & 0xffff0000;
            pi_var6 = (u_var2 | i_var5 + 0x1e);
            u_var8 = (u_var2 >> 0x10);
            i_var5 = (i_var5 + 0x24) / 2 + (i_var5 + 0x20);
            let pi_var6_val = unsafe { *pi_var6 };
            MoveTo16(i_var5, pi_var6_val, in_hdc_2);
            LineTo16(i_var5, pi_var6_val + -0xf, in_hdc_2);
            d_var9 = GetCurrentPosition16(in_hdc_2);
            y_coord = (d_var9 >> 0x10);
            x_coord = d_var9;
            if (counter == 0) {
                local_10 = x_coord;
                local_e = y_coord;
            }
            counter = counter + 1;
        }
        u_var2 = local_Struct135.field_0x6;
        if ((u_var2 + 0x24) != 0) {
            local_e = local_e + -0xd;
        }
        u_var2 = local_Struct135.field_0x6;
        if ((u_var2 + 0x26) != 0) {
            y_coord = y_coord + 0xd;
        }
        u_var2 = local_Struct135.field_0x6;
        u_var3 = local_Struct135.field_0x6;
        u_var2 = (u_var2 + (u_var3 + 0x16) * 4 + 0x26);
        local_a_x_262 = u_var2;
        local_a_x_262 = &local_a_x_262.field_0x1e;
        u_var2 = u_var2 & 0xffff0000;
        u_var7 = (u_var2 >> 0x10);
        MoveTo16(
            local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20,
            local_a_x_262.field_0x22 + (u_var2 | ZEXT24(local_a_x_262)),
            in_hdc_2,
        );
        LineTo16(
            local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20,
            local_10,
            in_hdc_2,
        );
        curr_pos_10 = GetCurrentPosition16(in_hdc_2);
        curr_pos_10._2_2_ = (curr_pos_10 >> 0x10);
        if (curr_pos_10._2_2_ < local_e) {
            local_e = curr_pos_10._2_2_;
        }
        if (y_coord < curr_pos_10._2_2_) {
            y_coord = curr_pos_10._2_2_;
        }
        MoveTo16(local_e, local_10, in_hdc_2);
        LineTo16(y_coord, x_coord, in_hdc_2);
    }
    return;
}

pub fn draw_lines_1040_c302(param_1: u32, in_hdc_2: HDC16) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: i32;
    let local_AX_46: *mut Struct138;
    let local_AX_100: *mut Struct139;
    let mut u_var6: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    u_var4 = (param_1 + 6);
    iVar1 = (u_var4 + 0x16);
    if (1 < iVar1) {
        u_var2 = (param_1 + 6);
        u_var5 = u_var2 + 0x2a;
        u_var2 = u_var2 & 0xffff0000;
        u_var3 = (u_var2 | u_var5);
        local_AX_46 = u_var3;
        local_AX_46 = &local_AX_46.field_0x1e;
        u_var3 = u_var3 & 0xffff0000;
        u_var6 = (u_var3 >> 0x10);
        MoveTo16(
            local_AX_46.field_0x20 + local_AX_46.field_0x24,
            local_AX_46.field_0x22 / 2 + (u_var3 | ZEXT24(local_AX_46)),
            in_hdc_2,
        );
        u_var2 = (u_var5 + iVar1 * 4 + -4);
        local_AX_100 = u_var2;
        local_AX_100 = &local_AX_100.field_0x1e;
        u_var2 = u_var2 & 0xffff0000;
        u_var6 = (u_var2 >> 0x10);
        LineTo16(
            local_AX_100.field_0x20,
            local_AX_100.field_0x22 / 2 + (u_var2 | ZEXT24(local_AX_100)),
            in_hdc_2,
        );
    }
    return;
}

pub unsafe fn draw_1040_c226(param_1: *mut Struct135) {
    let mut u_var1: u32;
    let mut obj_handle: HPEN16;
    let mut HVar2: HGDIOBJ16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut h_dc: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    local_24 = BeginPaint16(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_22)),
        (i_var3 + 4),
    );
    local_26 = CreateSolidBrush16(0x8000);
    GetClientRect16(CONCAT22(unaff_ss, &local_32), (i_var3 + 4));
    u_var1 = (i_var3 + 6);
    local_28 = (u_var1 + 0x1a);
    u_var1 = (i_var3 + 6);
    local_2a = (u_var1 + 0x1c);
    local_30 = local_30 + 2;
    local_32 = local_28 - 10;
    local_2e = local_2e - 2;
    local_2c = local_2c - 2;
    FrameRect16(local_26, &local_32, unaff_ss);
    DeleteObject16(local_26);
    h_dc = local_24;
    obj_handle = CreatePen16(0x8080, 2, 0);
    HVar2 = SelectObject16(obj_handle, h_dc);
    draw_lines_1040_c302(i_var3, u_var4, local_24);
    draw_lines_1040_c38e(param_1, local_24);
    HVar2 = SelectObject16(HVar2, local_24);
    DeleteObject16(HVar2);
    EndPaint16(&local_22, unaff_ss);
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
    u_var4 = (param_2 >> 0x10);
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

pub fn realize_palette_1008_46e4(param_1: u32, param_2: *mut HDC16) -> u16 {
    let mut bVar1: bool;
    let mut u_var2: u16;
    let mut HVar3: HPALETTE16;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 6) == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | u_var5 << 0x10));
    }
    if ((i_var4 + 6) == 0) {
        bVar1 = false;
    } else {
        if ((i_var4 + 10) == 0) {
            process_struct_1008_4834((param_1 & 0xffff | u_var5 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0;
    }
    u_var2 = create_palette_1008_4e38((i_var4 + 10));
    (i_var4 + 0xe) = u_var2;
    let param_2_val = unsafe { *param_2 };
    HVar3 = SelectPalette16(0, (i_var4 + 0xe), param_2_val);
    (i_var4 + 4) = HVar3;
    RealizePalette16(param_2_val);
    return (i_var4 + 4);
}

pub fn realize_palette_1008_4e08(param_1: u32, param_2: *mut HDC16) -> HDC16 {
    let mut h_palette: HPALETTE16;
    let mut HVar1: HDC16;
    let mut local_4: u16;

    unsafe { HVar1 = *param_2 };
    h_palette = create_palette_1008_4e38(param_1);
    SelectPalette16(0, h_palette, HVar1);
    unsafe { HVar1 = *param_2 };
    RealizePalette16(HVar1);
    return HVar1;
}

pub fn create_palette_1008_4e38(param_1: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let in_dx: *mut Struct199;
    let local_bx_4: *mut Struct33;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var4 = (local_bx_4.field_0xc + 2) * 4;
    if (&local_bx_4.palette == 0) {
        process_struct_1000_179c(u_var4, in_dx);
        &local_bx_4.palette = u_var4;
        local_bx_4.field_0x10 = in_dx;
        *&local_bx_4.palette = 0x300;
        u_var3 = &local_bx_4.palette;
        (u_var3 + 2) = local_bx_4.field_0xc;
        u_var2 = &local_bx_4.palette;
        local_8 = u_var2 & 0xffff0000 | (u_var2 + 4);
        local_c = local_bx_4.field_0x4;
        local_e = 0;
        loop {
            pu_var1 = &local_bx_4.field_0xc;
            let pu_var_1_val = unsafe { *pu_var1 };
            if (pu_var_1_val == local_e || pu_var_1_val < local_e) {
                break;
            }
            u_var8 = (local_c >> 0x10);
            i_var5 = local_c;
            *local_8 = *(i_var5 + 2);
            u_var9 = (local_8 >> 0x10);
            i32_var6 = local_8;
            *(i32_var6 + 1) = *(i_var5 + 1);
            *(i32_var6 + 2) = *local_c;
            *(i32_var6 + 3) = 0;
            local_e = local_e + 1;
            local_8 = local_8 & 0xffff0000 | (i32_var6 + 4);
            local_c = local_c & 0xffff0000 | (i_var5 + 4);
        }
    }
    CreatePalette16(&local_bx_4.palette);
    return;
}

pub unsafe fn draw_1008_4f20(
    ctx: &mut AppContext,
    param_1: *mut Struct104,
    param_2: u32,
    param_3: u16,
    param_4: u32,
) {
    let mut palette: u16;
    let mut count: i32;
    let mut palette_1: u16;
    let mut local_eax_147: u32;

    let mut unaff_ss: u16;
    let mut u_var1: u32;
    let mut color_ref: u32;
    let u_var2: u8;
    let u_var3: u8;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let pa_var7: *mut Struct102;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let a_struct_7_26: *mut Struct7;

    pa_var7 = param_1;
    u_var9 = (param_1 >> 0x10);
    set_struct_1008_4016(pa_var7);
    (pa_var7 + 1) = param_4;
    &pa_var7[1].field_0x4 = param_3;
    pa_var7[1].field_0x6 = param_2;
    param_1 = (ctx.s_SinternalPutBldg2_site_0x_08lx__1050_5099 + 9);
    pa_var7.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 2);
    process_struct_1008_48fe(
        CONCAT22(unaff_ss, &a_struct_7_26),
        1,
        param_2 & 0xffff | ctx.dx_reg << 0x10,
    );
    read_from_file_1008_49e8(&a_struct_7_26);
    pass1_1008_5068(pa_var7, u_var9, &a_struct_7_26, unaff_ss);
    process_struct_1008_47cc(param_1);
    process_struct_1008_4834(param_1);
    u_var8 = 0x27e;
    u_var5 = 0;
    u_var6 = 0;
    u_var2 = 0;
    u_var3 = 0;
    u_var4 = 0;
    u_var1 = process_struct_1008_4772(param_1);
    local_28 = (u_var1 >> 0x10);
    local_2a = u_var1;
    local_eax_147._0_2_ = CreateDC16(
        local_2a,
        CONCAT13(u_var3, CONCAT12(u_var2, local_28)),
        CONCAT22(u_var5, u_var4),
        CONCAT22(u_var8, u_var6),
    );
    palette = &local_2c;
    realize_palette_1008_46e4(pa_var7, u_var9, palette, unaff_ss);
    color_ref = SetBkColor16(0xffffff, local_eax_147);
    SetTextColor16(CONCAT22(0x100, &pa_var7[1].field_0x4), local_eax_147);
    count = get_string_index_1000_3da4(*(pa_var7 + 1));
    TextOut16(count, (pa_var7 + 1), 0, 0, local_eax_147);
    SetBkColor16(color_ref, local_eax_147);
    SetTextColor16(CONCAT22(local_eax_147, local_eax_147), local_eax_147);
    palette_1 = SelectPalette16(0, palette, local_eax_147);
    DeleteObject16(palette_1);
    DeleteDC16(local_eax_147);
    close_file_1008_496c(&a_struct_7_26);
    return;
}

pub unsafe fn set_palette_fn_1018_69ac(in_Struct376: *mut Struct376) {
    let local_Struct376: *mut Struct376;
    let mut u16_1: u16;

    u16_1 = (in_Struct376 >> 0x10);
    local_Struct376 = in_Struct376;
    in_Struct376.offset = 0x6a02;
    local_Struct376.segment = 0x1018;
    if (&local_Struct376.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_Struct376.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(in_Struct376);
    return;
}

pub unsafe fn set_palette_fn_1018_69dc(
    in_Struct376: *mut Struct376,
    param_2: u8,
) -> *mut Struct376 {
    set_palette_fn_1018_69ac(in_Struct376);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_Struct376);
    }
    return in_Struct376;
}

pub fn load_cursor_fn_1018_6a0e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: *mut libc::c_void,
) -> *mut Struct65 {
    let local_bx_25: *mut Struct65;
    let mut local_es_25: u16;

    load_cursor_1008_61b2(param_1, param_3, param_6, param_7);
    local_es_25 = (param_1 >> 0x10);
    local_bx_25 = param_1;
    local_bx_25.u16_xea = param_5;
    local_bx_25.u16_xec = param_4;
    local_bx_25.u16_xee = param_2;
    local_bx_25.u16_xf0 = 0;
    param_1.ptr_a_lo = 0x6c66;
    local_bx_25.ptr_a_hi = 0x1018;
    local_bx_25.u16_xe0 = 1;
    local_bx_25.ptr_b_lo = 0;
    local_bx_25.ptr_b_hi = 0;
    &local_bx_25.u16_xe6 = 0x1df027f;
    return param_1;
}

pub unsafe fn begin_end_Paint_1018_6a7a(ctx: &mut AppContext, param_1: *mut Struct129) {
    let local_p_Struct129: *mut Struct129;
    let mut u_var1: u16;
    let mut unaff_ss: HWND16;
    let ppVar2: *mut pass1_struct_1;
    let mut HVar3: HWND16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut string_22: [u8; 32];

    u_var1 = (param_1 >> 0x10);
    local_p_Struct129 = param_1;
    BeginPaint16(CONCAT22(unaff_ss, string_22), local_p_Struct129.h_wnd_0x8);
    HVar3 = local_p_Struct129.h_wnd_0x8;
    EndPaint16(string_22, unaff_ss);
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(HVar3, 2));
    if ((ppVar2 + 0x20) == 0) {
        win_func_1018_6bb6(param_1);
        return;
    }
    win_fn_1018_6adc(param_1);
    return;
}

pub fn load_cursor_fn_1018_6d02(
    in_struct_65: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(in_struct_65, 0, 0xb, 0x9c, 0x8b, param_2, param_3);
    in_struct_65.ptr_a_lo = 0xa27e;
    (in_struct_65 + 2) = 0x1018;
    return in_struct_65;
}

pub fn load_cursor_fn_1018_6d38(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xc, 0x9d, 0xd0, param_2, param_3);
    param_1.ptr_a_lo = 0xb562;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6d6e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xd, 0x9e, 0xd1, param_2, param_3);
    param_1.ptr_a_lo = 0x9822;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6da4(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xe, 0x9f, 0xd2, param_2, param_3);
    param_1.ptr_a_lo = 0xab06;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6dda(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xf, 0xa0, 0xd4, param_2, param_3);
    param_1.ptr_a_lo = 0xbdea;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e10(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x10, 0xa1, 0xda, param_2, param_3);
    param_1.ptr_a_lo = 0xa0aa;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e46(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x11, 0xa2, 0xdc, param_2, param_3);
    param_1.ptr_a_lo = 0xb38e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e7c(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x12, 0xa3, 0xd3, param_2, param_3);
    param_1.ptr_a_lo = 0x964e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6eb2(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x13, 0xa4, 0xdb, param_2, param_3);
    param_1.ptr_a_lo = 0xa932;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6ee8(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x14, 0xa5, 0xa5, param_2, param_3);
    param_1.ptr_a_lo = 0xbc16;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f1e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x15, 0xa7, 0xb2, param_2, param_3);
    param_1.ptr_a_lo = 0x9e3a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f54(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x16, 0xa8, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb11e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f8a(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x17, 0xaf, 0xc0, param_2, param_3);
    param_1.ptr_a_lo = 0x93de;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6fc0(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x18, 0xb0, 0xc1, param_2, param_3);
    param_1.ptr_a_lo = 0xa6c2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6ff6(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x19, 0xb1, 0x80, param_2, param_3);
    param_1.ptr_a_lo = 0xb9a6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_702c(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ec, 0x1a, 0xb2, 0xc3, param_2, param_3);
    param_1.ptr_a_lo = 0x9c66;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7062(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1b, 0xb3, 0xc4, param_2, param_3);
    param_1.ptr_a_lo = 0xaf4a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7098(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1c, 0xb4, 0xd8, param_2, param_3);
    param_1.ptr_a_lo = 0xc22e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_70ce(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1d, 0xb5, 0x7b, param_2, param_3);
    param_1.ptr_a_lo = 0xa4ee;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7104(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1e, 0xb6, 0xd9, param_2, param_3);
    param_1.ptr_a_lo = 0xb7d2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_713a(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1f, 0xb7, 0x7d, param_2, param_3);
    param_1.ptr_a_lo = 0x9a92;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7170(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x21, 0xb9, 0xdd, param_2, param_3);
    param_1.ptr_a_lo = 0xad76;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_71a6(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x23, 0xd3, 0xd6, param_2, param_3);
    param_1.ptr_a_lo = 0xb69a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_71dc(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ed, 0x24, 0xd4, 0xd7, param_2, param_3);
    param_1.ptr_a_lo = 0x995a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7212(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x25, 0xe9, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0xa452;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_7248(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 99, 0xa6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc05a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_727e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 100, 0xa9, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa31a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_72b4(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x65, 0xaa, 0xbb, param_2, param_3);
    param_1.ptr_a_lo = 0xb5fe;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_72ea(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x66, 0xab, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x98be;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7320(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x67, 0xac, 0xbd, param_2, param_3);
    param_1.ptr_a_lo = 0xaba2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7356(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x68, 0xad, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbe86;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_738c(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x69, 0xae, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xac3e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_73c2(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x35, 0xba, 0x81, param_2, param_3);
    param_1.ptr_a_lo = 0xbf22;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_73f8(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x39, 0xbb, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa146;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub unsafe fn drawing_and_sound_fn_1018_742e(ctx: &mut AppContext, in_struct_129: *mut Struct129) {
    let mut iVar1: i32;

    begin_end_Paint_1018_6a7a(in_struct_129);
    if (ctx.PTR_LOOP_1050_4254 == 0x0) {
        iVar1 = mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1e9);
        if (iVar1 != 0) {
            ctx.PTR_LOOP_1050_4254 = (&ctx.PTR_LOOP_1050_0000 + 1);
        }
    }
    return;
}

pub fn load_cursor_fn_1018_745e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x22, 0xbc, 0xd5, param_2, param_3);
    param_1.ptr_a_lo = 0xb42a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7494(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x36, 0xbd, 0xcd, param_2, param_3);
    param_1.ptr_a_lo = 0x96ea;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_74ca(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x37, 0xbe, 0x83, param_2, param_3);
    param_1.ptr_a_lo = 0xa9ce;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7500(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x38, 0xbf, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbcb2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7536(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3a, 0xc0, 0x85, param_2, param_3);
    param_1.ptr_a_lo = 0x9f72;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_756c(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e2, 0x3b, 0xc1, 0x86, param_2, param_3);
    param_1.ptr_a_lo = 0xb256;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_75a2(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3c, 0xc2, 0x87, param_2, param_3);
    param_1.ptr_a_lo = 0x9516;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_75d8(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3d, 0xc3, 0x88, param_2, param_3);
    param_1.ptr_a_lo = 0xa7fa;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_760e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3e, 0xc4, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbade;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7644(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3f, 0xc5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9d02;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_767a(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x40, 0xc6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xafe6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_76b0(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x41, 199, 0x8d, param_2, param_3);
    param_1.ptr_a_lo = 0xc2ca;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_76e6(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x42, 200, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa58a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_771c(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x43, 0xc9, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb86e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7752(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x44, 0xcc, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9b2e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7788(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x45, 0xcd, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xae12;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_77be(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x46, 0xd1, 0x92, param_2, param_3);
    param_1.ptr_a_lo = 0xc0f6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_77f4(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x47, 0xd2, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa3b6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_782a(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x48, 0xd5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xacda;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x49, 0xd6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbfbe;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7896(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 500, 0x4a, 0xd7, 0x98, param_2, param_3);
    param_1.ptr_a_lo = 0xa1e2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_78cc(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4b, 0xd8, 0x99, param_2, param_3);
    param_1.ptr_a_lo = 0xb4c6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7902(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4c, 0xd9, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0x9786;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7938(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4d, 0xda, 0x9c, param_2, param_3);
    param_1.ptr_a_lo = 0xaa6a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_796e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4e, 0xdb, 0x9d, param_2, param_3);
    param_1.ptr_a_lo = 0xbd4e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_79a4(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4f, 0xdc, 0x9e, param_2, param_3);
    param_1.ptr_a_lo = 0xa00e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_79da(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x50, 0xdd, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb2f2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a10(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1d9, 0x51, 0xde, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x95b2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a46(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x52, 0xdf, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa896;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a7c(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x53, 0xe0, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbb7a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7ab2(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e4, 0x55, 0xe2, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb082;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7ae8(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e4, 0x56, 0xe3, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc366;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b1e(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1da, 0x57, 0xe4, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa626;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b54(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1d8, 0x58, 0xe5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb90a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b8a(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x59, 0xe6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9bca;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7bc0(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ef, 0x5a, 0xe7, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xaeae;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7bf6(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5b, 0xe8, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc192;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c2c(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5c, 0xea, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb736;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c62(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5d, 0xeb, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x99f6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c98(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e6, 0x5e, 0xec, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0xba42;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7cce(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1da, 0x5f, 0xed, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9ed6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d04(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x60, 0xee, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb1ba;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d3a(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1f0, 0x61, 0xef, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x947a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d70(
    param_1: *mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> *mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1f7, 0x62, 0xf0, 0xcc, param_2, param_3);
    param_1.ptr_a_lo = 0xa75e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub unsafe fn draw_1018_c578(ctx: &mut AppContext, param_1: u32) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let in_struct_104_ptr: *mut Struct104;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let h_palette: *mut u16;
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

    let local_26 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(unaff_si, 2));
    local_28 = (local_26 + 0x20);
    i_var7 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var7 + 8));
        EndPaint16(&local_22, unaff_ss);
        PostMessage16(0, (i_var7 + 0xea), 0x111, ctx.g_h_window);
        return;
    }
    if (((i_var7 + 0xf0) == 0) && ((i_var7 + 0xf4) != 0)) {
        (i_var7 + 0xf0) = 1;
        mci_send_command_1008_5c9e(
            ctx._g_struct_ptr_1050_02a0,
            (param_1 & 0xffff0000 | (i_var7 + 0xf2)),
        );
    }
    if ((ctx._g_struct_ptr_1050_02a0 + 0x12) == 0) {
        mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1d3);
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
    u_var6 = (u_var9 >> 0x10);
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
    draw_text_1018_c742(param_1, &local_2a, unaff_ss);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_text_1018_c742(param_1: u32, param_2: *mut HDC16, param_3: &u16) {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let local_bx_4: *mut Struct54;
    let mut u_var4: u16;
    let mut unaf_SS: u16;
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
    let mut local_6: u16;
    let mut local_a: u16;
    let mut unaff_ss: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if ((local_bx_4.field_0x108 != 0) && (*local_bx_4.field_0x108 != '\0')) {
        let param_2_val = *param_2;
        let param_3_val = *param_3;
        local_6 = SetTextColor16(0x8000, param_3_val);
        local_a = SetBkColor16(0x2000000, param_2_val);
        if (local_bx_4.field_0x106 == 0) {
            local_18 = 0;
            local_16 = &local_bx_4.field_0x10e;
            local_14 = 0;
            local_1a = param_2_val;
            u_var2 = local_bx_4.field_0x108;
            local_1c = (u_var2 >> 0x10);
            DrawText16(
                0x410,
                &local_1a,
                unaff_ss,
                CONCAT22(u_var2, 0xffff),
                local_1c,
            );
            local_bx_4.field_0x100 = (0x280 - local_16) / 2;
            local_bx_4.field_0x102 = local_bx_4.field_0x10c;
            local_bx_4.field_0x104 = local_bx_4.field_0x100 + local_16;
            i_var3 = local_bx_4.field_0x102 + local_14;
            local_bx_4.field_0x106 = i_var3;
            pi_var1 = &local_bx_4.field_0xf8;
            let pi_var1_val = unsafe { *pi_var1 };
            if (pi_var1_val == i_var3 || pi_var1_val < i_var3) {
                i_var3 = i_var3 - local_bx_4.field_0xf8;
                pi_var1 = &local_bx_4.field_0x102;
                unsafe {
                    *pi_var1 = *pi_var1 - i_var3;
                    pi_var1 = &local_bx_4.field_0x106;
                    *pi_var1 = *pi_var1 - i_var3;
                }
            }
        }
        local_12 = local_bx_4.field_0xfa + local_bx_4.field_0x100;
        local_10 = local_bx_4.field_0xfc + local_bx_4.field_0x102;
        local_e = local_bx_4.field_0xfa + local_bx_4.field_0x104;
        local_c = local_bx_4.field_0xfc + local_bx_4.field_0x106;
        u_var2 = local_bx_4.field_0x108;
        DrawText16(
            0x10,
            &local_12,
            unaff_ss,
            CONCAT22(u_var2, 0xffff),
            (u_var2 >> 0x10),
        );
        SetTextColor16(local_6, param_2_val);
        SetBkColor16(local_a, param_2_val);
    }
    return;
}

pub unsafe fn draw_1018_cda8(ctx: &mut AppContext, param_1: u32) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let in_struct_104_ptr: *mut Struct104;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let h_palette: *mut u16;
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
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22((in_stack_0000ffb0 >> 0x10), 2),
    );
    local_28 = (local_26 + 0x20);
    i_var8 = param_1;
    u_var9 = (param_1 >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 8));
        EndPaint16(&local_22, unaff_ss);
        PostMessage16(0, (i_var8 + 0xea), 0x111, ctx.g_h_window);
        return;
    }
    if ((i_var8 + 0xf0) == 0) {
        (i_var8 + 0xf0) = 1;
        mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 499);
        if ((ctx._g_struct_ptr_1050_02a0 + 0x12) == 0) {
            mci_send_cmd_1008_5c5c(ctx._g_astruct_112_a_g_struct_ptr_1050_02a0, 0x1d3);
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
    u_var10 = (in_struct_104_ptr >> 0x10);
    ppc_var3 = (in_struct_104_ptr + 8);
    (**ppc_var3)(offset, in_struct_104_ptr, u_var10, h_palette);
    u_var11 = process_struct_1008_4772(in_struct_104_ptr);
    u_var7 = (u_var11 >> 0x10);
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
        &ctx._g_struct_ptr_1050_02a0PTR_LOOP_1050_1008,
        in_struct_104_ptr,
        u_var10,
        (i_var8 + 0xfc) + (i_var8 + 0xfe) + i32_var6,
        (i_var8 + 0xfa) + i_var5,
        0xd6,
    );
    draw_text_1018_c742(param_1, &local_2a, unaff_ss);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_1018_cfc0(ctx: &mut AppContext, param_1: u32) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let in_struct_104_ptr: *mut Struct104;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let h_palette: *mut u16;
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
        process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffb0, 2));
    local_28 = (local_26 + 0x20);
    i_var8 = param_1;
    u_var9 = (param_1 >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 8));
        EndPaint16(&local_22, unaff_ss);
        PostMessage16(0, (i_var8 + 0xea), 0x111, ctx.g_h_window);
        return;
    }
    if (((i_var8 + 0xf0) == 0) && ((i_var8 + 0xf4) != 0)) {
        (i_var8 + 0xf0) = 1;
        mci_send_command_1008_5c9e(
            ctx._g_struct_ptr_1050_02a0,
            (param_1 & 0xffff0000 | (i_var8 + 0xf2)),
        );
        if ((ctx._g_struct_ptr_1050_02a0 + 0x12) == 0) {
            mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1d3);
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
    u_var10 = (in_struct_104_ptr >> 0x10);
    ppc_var3 = (in_struct_104_ptr + 8);
    (**ppc_var3)(offset, in_struct_104_ptr, u_var10, h_palette);
    u_var11 = process_struct_1008_4772(in_struct_104_ptr);
    u_var7 = (u_var11 >> 0x10);
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
    draw_text_1018_c742(param_1, &local_2a, unaff_ss);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_ss);
    return;
}

pub unsafe fn draw_fn_1018_dfd4(ctx: &mut AppContext, param_1: *mut Struct618) -> u16 {
    let mut u_var1: u16;
    let local_Struct618_ptr_1: *mut Struct618;
    let local_Struct618_ptr_1_hi: *mut Struct618;
    let pu_var2: Vec<u8>;
    let local_char_ptr_1: String;
    let temp_5fe392f4b5: &mut Vec<u8>;

    local_Struct618_ptr_1_hi = (param_1 >> 0x10);
    local_Struct618_ptr_1 = param_1;
    temp_5fe392f4b5 = local_Struct618_ptr_1.void_ptr_32_xe2;
    u_var1 = draw_1018_e16c(temp_5fe392f4b5, (temp_5fe392f4b5 >> 0x10));
    if (local_Struct618_ptr_1.field_0xe6 == 0) {
        local_Struct618_ptr_1.field_0xe6 = 1;
        process_struct_1010_20ba(
            ctx._g_astruct_372_1050_0ed0,
            CONCAT22(local_char_ptr_1, 0x36),
        );
        pu_var2 = pass1_1038_af40(ctx._g_astruct_112_a, local_Struct618_ptr_1.field_0x8, 8);
        u_var1 = SUB42(pu_var2, 0);
    }
    return u_var1;
}

pub fn draw_1018_e16c(param_1: u32) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let h_palette: *mut u16;
    let mut h_gdi_obj: HPALETTE16;
    let mut u_var4: u16;
    let mut unaff_ss: HWND16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

    u_var4 = (param_1 >> 0x10);
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
    param_1: *mut Struct622,
    in_handle_ptr: *mut HANDLE16,
) {
    let pu_var1: *mut u16;

    let ppVar2: *mut pass1_struct_1;
    let mut local_handle_1: HANDLE16;
    let mut uStack16: u16;
    let local_char_ptr: String;
    let mut local_6: u32;
    let temp_7e056a7c981: *mut u32;
    let mut temp_5fb7f3034b: u32;
    let fn_ptr_1: fn();

    local_handle_1 = in_handle_ptr;
    uStack16 = (in_handle_ptr >> 0x10);
    get_dc_1020_921c(CONCAT22(local_handle_1, param_1), uStack16);
    param_1.u32_0x14 = 0;
    CONCAT22(local_handle_1, param_1) = 0xe5d0;
    param_1.u16_0x2 = 0x1018;
    ppVar2 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_char_ptr, 0x26));
    ctx.dx_reg = (ppVar2 >> 0x10);
    &param_1.u32_0x14 = ppVar2;
    (&param_1.u32_0x14 + 2) = ctx.dx_reg;
    param_1.u16_0x6 = &param_1.u32_0x14;
    param_1.u16_0x8 = ctx.dx_reg;
    temp_5fb7f3034b = param_1.u32_0x14;
    pu_var1 = &param_1.field_0xa;
    fn_ptr_1 = ((temp_5fb7f3034b + 10) + 8);
    (**fn_ptr_1)();
    param_1.i16_0x12 = pu_var1;
    draw_1020_9364(CONCAT22(local_handle_1, param_1));
    return;
}

pub unsafe fn select_and_delete_palette_fn_1018_e57a(
    ctx: &mut AppContext,
    in_struct_ptr_1: &mut Struct44,
) {
    let local_struct_ptr_1: *mut Struct376;
    let mut u_var1: u16;

    u_var1 = (in_struct_ptr_1 >> 0x10);
    local_struct_ptr_1 = in_struct_ptr_1;
    in_struct_ptr_1.ptr_a_lo = 0xe5d0;
    local_struct_ptr_1.segment = 0x1018;
    if (&local_struct_ptr_1.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_struct_ptr_1.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(ctx, in_struct_ptr_1);
    return;
}

pub unsafe fn draw_fn_1018_ec74(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct622,
    in_struct_1_hi: *mut Struct622,
    param_3: u16,
) {
    let mut u_var1: u32;
    let pu_var2: *mut u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: u16;

    let mut u_var6: u16;
    let mut unaff_si: u16;
    let pp_var7: *mut pass1_struct_1;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let pa_var10: *mut Struct622;
    let pa_var11: *mut Struct622;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    get_dc_1020_921c(ctx, CONCAT22(in_struct_1_hi, in_struct_1), param_3);
    in_struct_1.u32_0x14 = 0;
    zero_list_1008_3e38(CONCAT22(in_struct_1_hi, &in_struct_1.field_0x18));
    zero_list_1008_3e38(CONCAT22(in_struct_1_hi, &in_struct_1.field_0x1e));
    &in_struct_1.field_0x24 = 0;
    CONCAT22(in_struct_1_hi, in_struct_1) = 0x1cc;
    in_struct_1.u16_0x2 = 0x1020;
    pp_var7 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(unaff_si, 0x28));
    u_var5 = (pp_var7 >> 0x10);
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
    pp_var7 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(u_var6, 0x48));
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

pub unsafe fn invalidate_rect_1018_edd8(param_1: u32, param_2: i32) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: HWND16;
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

    iVar1 = param_1;
    u_var2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xc) {
        return;
    }
    local_8 = iVar1 + 0x18;
    pass1_1008_3e94(
        local_8,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    local_a = pass1_1010_2b66((iVar1 + 0x14));
    local_e = process_struct_1008_4772(CONCAT22(local_8, local_a));
    u_var2 = (local_e >> 0x10);
    local_16 = local_4;
    local_14 = local_6;
    local_12 = local_4 + (local_e + 4);
    local_10 = local_6 + (local_e + 8);
    InvalidateRect16(0, &local_16, unaff_ss);
    return;
}

pub unsafe fn draw_1020_0000(ctx: &mut AppContext, param_1: u32) {
    let pu_var1: *mut u16;
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
    u_var8 = (param_1 >> 0x10);
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

pub fn call_fill_rect_1020_041e(in_struct_1: *mut Struct13) {
    fill_rect_1020_065e((in_struct_1 + 0xe6));
    return;
}

pub fn fill_rect_1020_065e(param_1: *mut Struct13) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u32;
    let local_bx_6: *mut Struct13;
    let mut u_var4: u16;
    let mut h_dc: u16;
    let mut rectangle: RECT16;
    let mut brush: u16;
    let mut palette: u16;
    let mut local_28: u32;
    let mut result: u16;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    result = BeginPaint16(CONCAT22(h_dc, &local_22), local_bx_6.field_0x4);
    if (0x280 < local_bx_6.field_0xa) {
        brush = CreateSolidBrush16(0x210070b);
        rectangle.left = 0;
        rectangle.top = 0;
        rectangle.right = local_bx_6.field_0xa + -1;
        rectangle.bottom = local_bx_6.field_0xc + -1;
        FillRect16(brush, &rectangle, h_dc);
        DeleteObject16(brush);
    }
    u_var2 = local_bx_6.field_0x6;
    local_28 = (u_var2 + 0xe);
    palette = &result;
    u_var3 = local_28;
    pp_var1 = (u_var3 + 8);
    (**pp_var1)(offset, local_28, (local_28 >> 0x10), palette);
    pp_var1 = (u_var3 + 4);
    (**pp_var1)(
        offset,
        local_28,
        local_bx_6.field_0x10,
        local_bx_6.field_0xe,
        &result,
    );
    palette = SelectPalette16(0, palette, result);
    DeleteObject16(palette);
    EndPaint16(&local_22, h_dc);
    return;
}

pub unsafe fn draw_fn_1020_0a52(ctx: &mut AppContext, param_1: *mut Struct630) {
    let local_bx_4: *mut Struct630;
    let mut u_var1: u16;
    let pu_var2: Vec<u8>;

    u_var1 = (param_1 >> 0x10);
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

pub fn palette_fn_1020_0aa6(param_1: &mut Vec<u8>) {
    palette_fn_1020_0cd2((param_1 + 0xe2));
    return;
}

pub unsafe fn draw_1020_0c3e(in_struct_1: *mut Struct631) {
    let paVar1: *mut Struct632;
    let h_palette: *mut HDC16;
    let mut h_gdi_obj: HPALETTE16;
    let struct_a: *mut Struct631;
    let struct_b: *mut Struct632;
    let struct_a_hi: *mut Struct631;
    let struct_c_hi: *mut Struct632;
    let mut window_handle_a: HWND16;
    let mut local_30: u16;
    let local_28: *mut Struct632;
    let mut hdc1: HDC16;
    let mut paint_struct_a: PAINTSTRUCT16;
    let struct_c: *mut Struct632;
    let temp_5f82dd1a72: *mut Struct632;
    let fn_ptr_1: fn();
    let mut offset: u16;

    struct_a_hi = (in_struct_1 >> 0x10);
    struct_a = in_struct_1;
    hdc1 = BeginPaint16(
        CONCAT22(window_handle_a, &paint_struct_a),
        struct_a.hwnd_0x4,
    );
    struct_c = struct_a.astruct_632_0x6;
    struct_c_hi = (struct_c >> 0x10);
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

pub fn palette_fn_1020_0cd2(ctx: &mut AppContext, in_struct_1: *mut Struct633) {
    let paVar1: *mut Struct634;
    let mut u_var2: i32;
    let mut hdc: HDC16;
    let mut h_palette: HDC16;
    let mut h_palette_00: HPALETTE16;
    let mut u_var3: u16;

    let local_struct_1: *mut Struct633;
    let local_struct_2: *mut Struct634;
    let local_struct_1_hi: *mut Struct633;
    let local_struct_2_hi: *mut Struct634;
    let mut h_dc: HDC16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut local_a: u32;
    let temp_5f7b277b00: *mut u32;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = local_struct_1.field_0x6;
    local_struct_2_hi = (paVar1 >> 0x10);
    local_struct_2 = paVar1;
    temp_5f7b277b00 = &local_struct_2.field_0xa;
    local_6._0_2_ = temp_5f7b277b00;
    u_var2 = &local_struct_2.field_0xc | local_6;
    if (u_var2 != 0) {
        let tmp_5f7b_val = unsafe { *temp_5f7b277b00 };
        fn_ptr_1 = (tmp_5f7b_val + 0x14);
        (**fn_ptr_1)();
        local_a = CONCAT22(ctx.dx_reg, u_var2);
        if ((ctx.dx_reg | u_var2) != 0) {
            hdc = GetDC16(local_struct_1.win_handle_0x4);
            h_palette = hdc;
            h_dc = hdc;
            create_palette_1008_4e38(local_a);
            h_palette_00 = SelectPalette16(0, h_palette, h_dc);
            u_var3 = RealizePalette16(hdc);
            SelectPalette16(1, h_palette_00, hdc);
            DeleteObject16(h_palette);
            if (0 < u_var3) {
                InvalidateRect16(1, 0x0, 0);
            }
            ReleaseDC16(hdc, local_struct_1.win_handle_0x4);
            return;
        }
    }
    return;
}

pub fn realize_palette_1020_0e46(param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u32;
    let mut u_var4: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
        u_var4 = (param_1 >> 0x10);
        u_var2 = (param_1 + 0xf2);
        pu_var3 = (u_var2 + 0x66);
        let pu_var3_val = unsafe { *pu_var3 };
        pp_var1 = (pu_var3_val + 0x18);
        (**pp_var1)();
        UnrealizeObject16(pu_var3_val);
        u_var2 = (param_1 + 0xf2);
        RealizePalette16((u_var2 + 0x7c));
    }
    return;
}

pub unsafe fn call_Paint_fn_1020_1022(in_struct_1: *mut Struct14) {
    draw2::paint_func_1020_15de((in_struct_1 + 0xf6));
    return;
}

pub unsafe fn palette_func_1020_150e(ctx: &mut AppContext, in_struct_1: *mut Struct376) {
    let mut u_var1: u32;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1_low: *mut Struct376;
    let local_struct_1: *mut Struct376;

    local_struct_1 = (in_struct_1 >> 0x10);
    local_struct_1_low = in_struct_1;
    in_struct_1.offset = 0x1730;
    local_struct_1_low.segment = 0x1020;
    if (&local_struct_1_low.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1ea6(
            local_struct_1_low.u8_ptr_x14,
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1) << 0x10),
        );
    }
    u_var1 = &local_struct_1_low.u8_ptr_x14;
    h_gdi_obj = SelectPalette16(0, &local_struct_1_low.field_0x1c, (u_var1 + 0x7c));
    &local_struct_1_low.field_0x1c = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    in_struct_1.offset = ctx.s_0_020_1050_3ab0;
    local_struct_1_low.segment = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.offset = ctx.s_1_1050_389a;
    local_struct_1_low.segment = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn draw_1040_a67e(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: i32,
    param_3: i32,
    in_h_dc: HDC16,
) {
    let pu_var1: *mut u16;
    let mut b_var2: bool;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut brush: u16;
    let local_bx_5: *mut Struct36;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (local_bx_5.brush == 0) {
        brush = CreateSolidBrush16(0);
        local_bx_5.brush = brush;
    }
    if (ctx._PTR_LOOP_1050_5ee8 == 0) {
        u_var6 = pass1_1008_4d72((ctx._PTR_LOOP_1050_4230 + 0xe));
        u_var3 = (u_var6 >> 0x10);
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
    hdc_00 = (param_2 >> 0x10);
    i_var8 = param_2;
    local_20 = *(i_var8 + 4) & 8;
    local_22 = *(i_var8 + 0x56) & 1;
    hdc = BeginPaint16(CONCAT22(unaff_ss, &local_42), param_1);
    mode = SetMapMode16(1, hdc);
    GetClientRect16(CONCAT22(unaff_ss, &local_12), param_1);
    i_var2 = (local_e >> 0x10);
    iVar1 = i_var2 + -1;
    local_e = CONCAT22(iVar1, local_e - 1);
    if (local_22 != 0) {
        local_1a._0_2_ = local_12;
        local_1a._2_2_ = (local_12 >> 0x10);
        local_12 = CONCAT22(local_1a._2_2_ + 2, local_1a + 2);
        local_e = CONCAT22(i_var2 + -3, local_e - 3);
        local_16._0_2_ = local_e - 1;
        local_16._2_2_ = iVar1;
    }
    if (*(i_var8 + 6) != '\0') {
        local_1c = 1;
        if ((i_var8 + 0x5a) != 0) {
            local_1e = SelectObject16((i_var8 + 0x5a), hdc);
        }
        u_var3 = i_var8 + 6;
        u_var4 = get_string_index_1000_3da4((param_2 & 0xffff0000 | u_var3));
        DrawText16(0x400, &local_a, unaff_ss, CONCAT22(u_var3, u_var4), hdc_00);
        local_8 = ((local_c - local_4) + local_8) / 2 + local_12._2_2_;
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
            MoveTo16(local_16._2_2_, local_1a, hdc);
            LineTo16(local_16._2_2_, local_16, hdc);
            LineTo16(local_1a._2_2_, local_16, hdc);
            LineTo16(local_1a._2_2_, local_1a, hdc);
            LineTo16(local_16._2_2_, local_1a, hdc);
            local_1a._2_2_ = local_1a._2_2_ + 1;
            local_1a._0_2_ = local_1a + 1;
            local_16._0_2_ = local_16 + -1;
            local_16._2_2_ = local_16._2_2_ + -1;
            local_4e = local_4e + 1;
            local_4e < 1
        } {}
    }
    if ((*(i_var8 + 4) & 2) == 0) {
        local_56._2_2_ = (local_12 >> 0x10);
        local_52._0_2_ = local_e;
        local_4e = 0;
        local_56._0_2_ = local_12;
        local_52._2_2_ = local_c;
        while {
            SelectObject16(obj_handle, hdc);
            MoveTo16(local_52._2_2_, local_56, hdc);
            LineTo16(local_52._2_2_, local_52, hdc);
            LineTo16(local_56._2_2_, local_52, hdc);
            while {
                SelectObject16(obj_handle_00, hdc);
                LineTo16(local_56._2_2_, local_56, hdc);
                LineTo16(local_52._2_2_, local_56, hdc);
                (hdc + 1) < 2
            } {}
            local_56._2_2_ = local_56._2_2_ + 1;
            local_56._0_2_ = local_56 + 1;
            local_52._0_2_ = local_52 + -1;
            local_52._2_2_ = local_52._2_2_ - 1;
            local_4e = local_4e + 1;
            local_4e < 2
        } {}
    } else {
        MoveTo16(local_c, local_12, hdc);
        LineTo16(local_12._2_2_, local_12, hdc);
        LineTo16(local_12._2_2_, local_e + 1, hdc);
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

pub fn draw_text_1040_9650(param_1: u32) {
    let mut u_var1: u32;
    let mut hdc: HDC16;
    let mut count: u16;
    let mut local_4: u16;

    hdc = GetDC16(0);
    count = (param_1 >> 0x10);
    u_var1 = (param_1 + 4);
    DrawText16(
        0x410,
        (param_1 + 0x2e),
        count,
        CONCAT22(u_var1, 0xffff),
        (u_var1 >> 0x10),
    );
    ReleaseDC16(hdc, 0);
    return;
}

pub fn draw_text_1040_94fc(param_1: u32, param_2: HDC16) -> COLORREF {
    let mut u_var1: u32;
    let local_bx_7: *mut Struct56;
    let mut count: u16;
    let mut CVar2: COLORREF;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    count = (param_1 >> 0x10);
    local_bx_7 = param_1;
    CVar2 = SetBkColor16(CONCAT22(0x100, local_bx_7.field_0x3a), param_2);
    SetTextColor16(CONCAT22(0x100, local_bx_7.field_0x3c), param_2);
    u_var1 = local_bx_7.field_0x4;
    local_a = (u_var1 >> 0x10);
    DrawText16(
        0x10,
        &local_bx_7.field_0x2e,
        count,
        CONCAT22(u_var1, 0xffff),
        local_a,
    );
    SetBkColor16(CVar2, param_2);
    CVar2 = SetTextColor16(CONCAT22(param_2, param_2), param_2);
    return CVar2;
}

pub fn delete_palette_func_1040_9458(param_1: u32, param_2: u8, param_3: HDC16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut h_gdi_obj: HPALETTE16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut uStack28: u16;
    let mut uStack26: u16;
    let puStack24: *mut u16;
    let mut uStack20: u16;
    let mut uStack18: u16;
    let puStack16: *mut u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 8) != 0) {
        local_6 = (i_var3 + 8);
        uStack18 = (i_var3 + 10);
        if ((((i_var3 + 0xe) | (i_var3 + 0xc)) != 0) && ((param_2 & 1) != 0)) {
            local_6 = (i_var3 + 0xc);
            uStack18 = (i_var3 + 0xe);
        }
        if (((i_var3 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_6 = (i_var3 + 0x10);
            uStack18 = (i_var3 + 0x12);
        }
        local_8 = &param_3;
        uStack20 = local_6;
        u_var2 = local_6;
        puStack24 = 0x94c5;
        pp_var1 = (u_var2 + 8);
        puStack16 = local_8;
        (**pp_var1)();
        puStack24 = &param_3;
        uStack26 = (i_var3 + 0x26);
        uStack28 = (i_var3 + 0x28);
        pp_var1 = (u_var2 + 4);
        (**pp_var1)();
        h_gdi_obj = SelectPalette16(0, local_8, param_3);
        DeleteObject16(h_gdi_obj);
    }
    return;
}

pub fn process_struct_1040_9252(param_1: *mut Struct357) {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let local_bx_3: *mut Struct357;
    let mut u_var3: i32;

    u_var3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (&local_bx_3.field_0x4 != 0) {
        draw_text_1040_9650(param_1);
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

pub fn get_sys_metrics_1040_8c66(param_1: u32) {
    let pi_var1: *mut i32;
    let mut b_var2: u8;
    let mut HVar3: HDC16;
    let mut i_var4: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    HVar3 = GetDC16((i_var5 + 6));
    draw_text_1040_8d14(param_1, HVar3);
    (i_var5 + 0xa6) = (i_var5 + 0x9e);
    (i_var5 + 0xaa) = (i_var5 + 0xa2);
    i_var4 = GetSystemMetrics16(4);
    pi_var1 = (i_var5 + 0xac);
    unsafe { *pi_var1 = *pi_var1 + i_var4 };
    b_var2 = *(i_var5 + 0x98) & 0xf0;
    if ((((b_var2 == 0x30) || (b_var2 == 0x10)) || (b_var2 == 0x40)) || (b_var2 == 0x20)) {
        i_var4 = GetSystemMetrics16(0xc);
        if ((i_var5 + 0xac) < i_var4) {
            i_var4 = GetSystemMetrics16(0xc);
            (i_var5 + 0xac) = i_var4;
        }
    }
    pi_var1 = (i_var5 + 0xaa);
    unsafe {
        *pi_var1 = *pi_var1 + 0x14;
        pi_var1 = (i_var5 + 0xac);
        *pi_var1 = *pi_var1 + 10;
        (i_var5 + 0xb0) = (i_var5 + 0xac);
        pi_var1 = (i_var5 + 0xac);
        *pi_var1 = *pi_var1 + 0x30;
    }
    HVar3 = (i_var5 + 6);
    ReleaseDC16(HVar3, HVar3);
    return;
}

pub fn draw_text_1040_8d14(ctx: &mut AppContext, param_1: u32, param_2: HGDIOBJ16) {
    let mut u_var1: u32;
    let mut b_var2: u8;
    let mut i_var3: u16;
    let mut obj_handle: HANDLE16;
    let local_bx_4: *mut Struct55;
    let mut count: u16;
    let mut local_8: u16;

    count = (param_1 >> 0x10);
    local_bx_4 = param_1;
    b_var2 = local_bx_4.field_0x98 & 0xf0;
    if ((((b_var2 == 0x30) || (b_var2 == 0x10)) || (b_var2 == 0x40)) || (b_var2 == 0x20)) {
        local_bx_4.field_0xa0 = 10;
        i_var3 = GetSystemMetrics16(0xb);
        local_bx_4.field_0x9e = i_var3 + 0x28;
    } else {
        local_bx_4.field_0xa0 = 10;
        local_bx_4.field_0x9e = 0x14;
    }
    obj_handle = GetProp16(ctx.s_hfont_1050_5e0f, local_bx_4.field_0x6);
    if (obj_handle != 0) {
        SelectObject16(obj_handle, param_2);
    }
    u_var1 = local_bx_4.field_0x90;
    local_8 = (u_var1 >> 0x10);
    DrawText16(
        0x410,
        &local_bx_4.field_0x9e,
        count,
        CONCAT22(u_var1, 0xffff),
        local_8,
    );
    if (local_8 != 0) {
        SelectObject16(param_2, param_2);
    }
    return;
}

pub fn draw_1040_82ee(param_1: u32) {
    let local_bx_6: *mut Struct35;
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

    hdc = (param_1 >> 0x10);
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

pub fn pt_in_rect_1040_8158(param_1: *mut Struct69, param_2: u32, param_3: u16) {
    let pp_var1: fn();
    let mut i_var2: u16;
    let b_var3: bool;
    let local_bx_10: *mut Struct69;
    let mut unaff_cs: u16;
    let mut unaff_ss: HWND16;
    let mut local_6: u32;
    let mut offset: u16;

    if (param_3 == 2) {
        local_bx_10 = param_1;
        if (local_bx_10.field_0x76 != 0) {
            local_6 = param_2;
            ScreenToClient16(CONCAT22(&local_6, unaff_cs), unaff_ss);
            i_var2 = GetSystemMetrics16(4);
            local_6 = local_6 & 0xffff | (local_6._2_2_ + i_var2) << 0x10;
            b_var3 = PtInRect16(local_6, (local_bx_10 + 1));
            if (b_var3 != 0) {
                pp_var1 = (param_1 + 0x14);
                (**pp_var1)(offset, param_1, 0);
            }
        }
    }
    return;
}

pub unsafe fn draw_fn_1040_7e5e(
    ctx: &mut AppContext,
    param_1: *mut u32,
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
        u_var5 = (param_1 >> 0x10);
        let param_1_val = unsafe { *param_1 };
        pp_var1 = (param_1_val + 0x68);
        u_var6 = (**pp_var1)(offset, param_1, u_var5, (param_1 + 0x6e));
        if (u_var6 == 0) {
            return 0;
        }
        u_var6 = pass1_1008_4d72(u_var6);
        u_var5 = (u_var6 >> 0x10);
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
    u_var7 = (ctx._PTR_LOOP_1050_5df0 >> 0x10);
    // LAB_1040_7f00:
    SetTextColor16(CONCAT22(u_var7, u_var5), local_c);
    SetBkColor16(0x1000000, param_4);
    return CONCAT22(0x1050, HVar2);
}

pub fn draw_1040_7bb2(ctx: &mut AppContext, param_1: *mut u32) {
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
    let rect: *mut RECT16;
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

    rect = (param_1 >> 0x10);
    i_var7 = param_1;
    BVar2 = IsIconic16((i_var7 + 6));
    if (BVar2 == 0) {
        local_4 = GetWindowDC16((i_var7 + 6));
        let param_1_val = unsafe { *param_1 };
        pp_var1 = (param_1_val + 0x68);
        local_8 = (**pp_var1)(offset, i_var7, rect, (i_var7 + 0x6e));
        if (local_8 != 0) {
            local_a = realize_palette_1008_4e08(local_8, 0xfc, unaff_ss);
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
                    h_var6 = ((i_var7 + 0x80) - (i_var7 + 0x7c)) / 2 - (dVar9 >> 0x10) / 2;
                    local_2c = ((param_1 & 0xffff0000) >> 0x10);
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
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let h_palette: *mut u16;
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
    let pu_var12: *mut u16;
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

    u_var10 = (param_1 >> 0x10);
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
    realize_palette_1008_4e08(u_var2, (u_var2 >> 0x10), h_palette, unaff_ss);
    u_var7 = 0;
    u_var8 = 0;
    local_36 = 0;
    if ((i_var9 + 0x98) != 0) {
        mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, (i_var9 + 0x98));
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
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x4d);
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
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, (i_var9 + 0x96));
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
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x4f);
    local_36 = CONCAT22(ctx.dx_reg, h_var6);
    u_var7 = process_struct_1008_4772(CONCAT13(
        (ctx.dx_reg >> 8),
        CONCAT12(ctx.dx_reg, h_var6),
    ));
    u_var8 = (u_var7 >> 0x10);
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
    param_1: *mut u32,
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

    u_var6 = (param_1 >> 0x10);
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
        u_var8 = (u_var7 >> 0x10);
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
    u_var9 = (ctx._PTR_LOOP_1050_5cf8 >> 0x10);
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
    let h_palette: *mut u16;
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

    count = (param_1 >> 0x10);
    i_var8 = param_1;
    local_24 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var8 + 6));
    u_var9 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    h_palette = &local_24;
    realize_palette_1008_4e08(u_var9, h_palette, unaff_ss);
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
    u_var6 = (u_var9 >> 0x10);
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
    let pu_var1: *mut u32;
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

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    if ((i_var5 + 0x8e) == 0) {
        solid_brush = CreateSolidBrush16(0);
        (i_var5 + 0x8e) = solid_brush;
    }
    if (ctx._PTR_LOOP_1050_5b64 == 0) {
        u_var7 = pass1_1008_4d72((ctx._PTR_LOOP_1050_4230 + 0xe));
        u_var3 = (u_var7 >> 0x10);
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

pub unsafe fn get_dc_1020_921c(ctx: &mut AppContext, param_1: *mut Struct622, param_2: u16) {
    let local_AX_99: *mut Struct670;
    let mut u_var1: u16;
    let local_struct_1: *mut Struct622;
    let local_es_4: *mut Struct622;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffe2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (in_stack_0000ffe2 >> 0x10);
    local_es_4 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.u16_0x0 = ctx.s_1_1050_389a;
    local_struct_1.u16_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1.u16_0x0 = (ctx.ctx.s_18_2_1050_3aa5 + 3);
    local_struct_1.u16_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.window_handle_0x4 = param_2;
    param_1.u16_0x0 = ctx.s_0_020_1050_3ab0;
    local_struct_1.u16_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_struct_1.u16_0x6 = 0;
    local_struct_1.field_0xa = 0;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0xe = 0;
    local_struct_1.field_0x10 = 0;
    local_struct_1.i16_0x12 = 0;
    param_1.u16_0x0 = 0x96c8;
    local_struct_1.u16_0x2 = 0x1020;
    local_AX_99 = GetDC16(local_struct_1.window_handle_0x4);
    local_struct_1.field_0xa = local_AX_99;
    ppVar2 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(u_var1, 0x48));
    u_var1 = (ppVar2 >> 0x10);
    local_struct_1.field_0xc = (ppVar2 + 10);
    local_struct_1.field_0xe = (ppVar2 + 0xc);
    return;
}

pub fn select_and_delete_palette_1020_92c4(ctx: &mut AppContext, struct_param_1: &mut Struct7) {
    let mut h_gdi_obj: HPALETTE16;
    let local_Struct376: *mut Struct376;
    let local_struct_376_hi: *mut Struct376;

    local_struct_376_hi = (struct_param_1 >> 0x10);
    local_Struct376 = struct_param_1;
    struct_param_1.ptr_a_lo = 0x96c8;
    local_Struct376.segment = 0x1020;
    if (local_Struct376.palette_handle_x12 != 0) {
        h_gdi_obj = SelectPalette16(
            0,
            local_Struct376.palette_handle_x12,
            local_Struct376.dc_handle_x0a,
        );
        DeleteObject16(h_gdi_obj);
    }
    struct_param_1.ptr_a_lo = ctx.s_0_020_1050_3ab0;
    local_Struct376.segment = &ctx.PTR_LOOP_1050_1008;
    struct_param_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_Struct376.segment = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn draw_1020_9312(param_1: u32) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: HWND16;
    let mut local_26: u32;
    let mut local_22: PAINTSTRUCT16;
    let mut offset: u16;

    u_var5 = (param_1 >> 0x10);
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

pub fn draw_1020_9364(ctx: &mut AppContext, param_1: *mut Struct622) {
    let pi_var1: *mut i32;
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

    u_var7 = (param_1 >> 0x10);
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
    u_var8 = (local_1a >> 0x10);
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
        local_6 = local_6 & 0xffff | (local_a._2_2_ - 1) << 0x10;
        local_12 = local_12 & 0xffff | (local_e._2_2_ + 1) << 0x10;
        u_var8 = (local_1a >> 0x10);
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
        u_var8 = (local_2e >> 0x10);
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

pub unsafe fn call_invalidate_rect_1020_8bcc(ctx: &mut AppContext, in_struct_1: *mut Struct693) {
    let paVar1: *mut Struct197;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let paVar4: *mut Struct199;



    let local_struct_1: *mut Struct693;
    let local_bx_310: *mut Struct694;
    let local_struct_1_hi: *mut Struct693;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 30];
    let mut local_3a: [u8; 38];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let local_struct_2: *mut Struct538;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x4 != 0) {
        local_struct_2 = local_struct_1.struct538_ptr_0x22;
        local_6 = (local_struct_2 + 10);
        local_a = pass1_1018_268e(local_struct_1.struct538_ptr_0x22);
        local_struct_2 = local_struct_1.struct538_ptr_0x22;
        local_c = (local_struct_2 + 0x16);
        if (local_struct_1.field_0xc == 0) {
            local_14 = process_struct_1008_4772(local_a);
            paVar4 = (local_14 >> 0x10);
            u_var2 = local_14;
            process_struct_1000_179c(0x14, paVar4);
            local_5c = CONCAT22(paVar4, u_var2);
            if ((paVar4 | u_var2) == 0) {
                local_struct_1.field_0xc = 0;
            } else {
                u_var3 = in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16;
                u_var5 = (local_14 >> 0x10);
                process_struct_1008_50c2(local_5c, (local_14 + 8), (local_14 + 4), u_var3, local_6);
                paVar1 = local_struct_1.field_0xc;
                u_var2 = u_var3;
                paVar1 = u_var2;
                (paVar1 + 2) = ctx.dx_reg;
            }
            pass1_1008_5134(local_struct_1.field_0xc);
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 2);
            process_struct_1008_48fe(
                CONCAT22(unaff_ss, local_3a),
                1,
                CONCAT22(ctx.dx_reg, u_var2),
            );
            pass1_1008_3f92(CONCAT22(unaff_ss, local_58), CONCAT22(unaff_ss, local_3a));
            local_14 = process_struct_1008_4772(CONCAT22(unaff_ss, local_58));
            paVar4 = (local_14 >> 0x10);
            u_var2 = local_14;
            process_struct_1000_179c(0x14, paVar4);
            local_5c = CONCAT22(paVar4, u_var2);
            if ((paVar4 | u_var2) == 0) {
                paVar1 = local_struct_1.field_0xc;
                (paVar1 + 4) = 0;
            } else {
                u_var3 = in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16;
                u_var5 = (local_14 >> 0x10);
                process_struct_1008_50c2(local_5c, (local_14 + 8), (local_14 + 4), u_var3, local_6);
                paVar1 = local_struct_1.field_0xc;
                u_var5 = (paVar1 >> 0x10);
                local_bx_310 = paVar1;
                local_bx_310.field_0x4 = u_var3;
                local_bx_310.field_0x6 = ctx.dx_reg;
            }
            paVar1 = local_struct_1.field_0xc;
            pass1_1008_5134((paVar1 + 4));
            process_struct_1008_41bc(CONCAT22(unaff_ss, local_58));
            close_file_1008_496c(local_3a);
        }
        paVar1 = local_struct_1.field_0xc;
        pass1_1008_5236((paVar1 + 4));
        pass1_1008_5236(local_struct_1.field_0xc);
        pass1_1008_4480(
            local_6,
            (in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16),
            local_a,
        );
        invalidate_rect_1020_8d90(in_struct_1, local_c, local_6, (local_6 >> 0x10));
    }
    return;
}

pub unsafe fn invalidate_rect_1020_8d90(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_3: u32,
) {
    let mut u_var1: u32;
    let mut in_dx: i32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let unaff_ss: String;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 40];
    let mut local_10: [u8; 10];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    local_6 = pass1_1018_266a((i_var2 + 0x22));
    if (local_6 != 0x0) {
        pass1_1018_265c((i_var2 + 0x22));
        if (in_dx | local_6) != 0 {
            string_fn_1000_3f9c(
                local_10,
                unaff_ss,
                ctx.s__03ld_1050_442a,
                &ctx.g_alloc_addr_1050_1050,
                local_6,
            );
            u_var1 = (i_var2 + 0x22);
            u_var1 = (u_var1 + 0xe);
            draw_1008_4f20(
                local_38,
                unaff_ss,
                u_var1,
                (u_var1 >> 0x10),
                0x25,
                local_10,
                unaff_ss,
            );
            pass1_1008_4480(
                param_3,
                (param_1 & 0xffff0000 | (i_var2 + 0x1c)),
                CONCAT22(unaff_ss, local_38),
            );
            local_3c = process_struct_1008_4772(CONCAT22(unaff_ss, local_38));
            pass1_1008_3e94(
                (i_var2 + 0x1c),
                CONCAT22(unaff_ss, &local_40),
                CONCAT22(unaff_ss, &local_3e),
            );
            local_48 = local_3e;
            local_46 = local_40;
            u_var3 = (local_3c >> 0x10);
            local_44 = local_3e + (local_3c + 4);
            local_42 = local_40 + (local_3c + 8);
            InvalidateRect16(0, &local_48, unaff_ss);
            process_struct_1008_41bc(CONCAT22(unaff_ss, local_38));
        }
    }
    return;
}

pub unsafe fn realize_palette_1020_8128(ctx: &mut AppContext, param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: Vec<u8>;
    let pu_var4: *mut u32;
    let pu_var5: *mut u32;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_22: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: [u8; 8];
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
        u_var7 = (param_1 >> 0x10);
        i32_var6 = param_1;
        u_var2 = (i32_var6 + 0xe6);
        pu_var5 = (u_var2 + 10);
        let pu_var5_val = unsafe { *pu_var5 };
        pp_var1 = (pu_var5_val + 0x18);
        local_6 = pu_var5;
        (**pp_var1)();
        local_8 = pu_var5;
        UnrealizeObject16(local_8);
        u_var2 = (i32_var6 + 0xe6);
        local_a = (u_var2 + 0x14);
        RealizePalette16(local_a);
        pass1_1008_57a4(
            CONCAT22(unaff_ss, local_12),
            param_1 & 0xffff0000 | (i32_var6 + 0xd2),
        );
        loop {
            pu_var3 = local_12;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
            if ((ctx.dx_reg | pu_var3) == 0) {
                break;
            }
            u_var2 = (pu_var3 + 4);
            u_var7 = (pu_var3 + 6);
            pu_var4 = u_var2;
            let pu_var4_1 = unsafe { *pu_var4 };
            pp_var1 = (pu_var4_1 + 0x90);
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008, pu_var4, u_var7, 1, u_var2);
        }
    }
    return;
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

    u_var2 = (ctx._PTR_LOOP_1050_4230 >> 0x10);
    u_var1 = (ctx._PTR_LOOP_1050_4230 + 0xe);
    local_6._0_2_ = u_var1;
    if (((ctx._PTR_LOOP_1050_4230 + 0x10) | local_6) == 0) {
        return;
    }
    hdc = GetDC16((param_1 + 8));
    h_palette = hdc;
    h_dc = hdc;
    create_palette_1008_4e38(u_var1);
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

pub unsafe fn load_cursor_1020_7f7a(
    ctx: &mut AppContext,
    in_struct_65_1: *mut Struct65,
    param_2: u32,
    param_3: u32,
) {
    let mut u_var1: u16;
    let mut h_gdi_obj: HGDIOBJ16;
    let mut h_cursor: HCURSOR16;
    let local_AX_192: *mut Struct590;
    let struct_65_1: *mut Struct65;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut u_var4: u16;
    let mut in_stack_0000ffe8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_2 >> 0x10);
    load_cursor_1008_61b2(in_struct_65_1, param_2, u_var4, param_3);
    u_var2 = (in_struct_65_1 >> 0x10);
    struct_65_1 = in_struct_65_1;
    struct_65_1.ptr_b_lo = ctx.s_1_1050_389a;
    struct_65_1.ptr_b_hi = &ctx.PTR_LOOP_1050_1008;
    struct_65_1.ptr_b_lo = (ctx.ctx.s_18_2_1050_3aa5 + 3);
    struct_65_1.ptr_b_hi = &ctx.PTR_LOOP_1050_1008;
    &struct_65_1.u16_xe6 = 0;
    struct_65_1.u16_xea = 0;
    struct_65_1.u16_xec = 0;
    in_struct_65_1.ptr_a_lo = 0x82bc;
    struct_65_1.ptr_a_hi = 0x1020;
    struct_65_1.ptr_b_lo = 0x8358;
    struct_65_1.ptr_b_hi = 0x1020;
    copy_string_1000_3d3e(
        (in_struct_65_1 & 0xffff0000 | ZEXT24(&struct_65_1.char_ptr_x5b)),
        ctx.s_VrMode_1050_4422,
    );
    h_gdi_obj = GetStockObject16(5);
    struct_65_1.h_gdi_obj_xc6 = h_gdi_obj;
    h_cursor = LoadCursor16(0x7f00, 0);
    struct_65_1.h_cursor_xc4 = h_cursor;
    struct_65_1.u16_xc8 = (ctx.s_576_bmp_1050_2027 + 1);
    struct_65_1.u32_xac = 0x47000000;
    struct_65_1.astruct_590_ptr_xbc = (param_3 + 8);
    ppVar3 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x48),
    );
    u_var1 = (ppVar3 >> 0x10);
    struct_65_1.u16_xb4 = 0;
    struct_65_1.u16_xb6 = 0;
    struct_65_1.u16_xb8 = (ppVar3 + 10);
    struct_65_1.u16_xba = (ppVar3 + 0xc);
    &struct_65_1.u32_xca = u_var4;
    reg_class_1008_96d2(in_struct_65_1, in_stack_0000ffe8);
    return;
}
