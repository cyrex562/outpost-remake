use crate::app_context::AppContext;
use crate::pass::pass20_funcs::pass1_1010_a5ca;
use crate::pass::pass8_funcs::process_struct_1010_20ba;
use crate::prog_structs::prog_structs_23::Struct56;
use crate::prog_structs::prog_structs_26::{Struct54, Struct55};
use crate::prog_structs::prog_structs_31::Struct334;
use crate::typedefs::{COLORREF, HANDLE16, HDC16, HGDIOBJ16};
use crate::util::CONCAT22;
use crate::winapi::{DrawText16, GetDC16, GetProp16, GetSystemMetrics16, ReleaseDC16, SelectObject16, SetBkColor16, SetTextColor16};

pub unsafe fn draw_text_1040_c94a(
    ctx: &mut AppContext,
    param_1: &mut Struct334,
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
