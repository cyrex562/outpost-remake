use crate::winapi::{SetBkColor16, SetTextColor16, CreateSolidBrush16, GetDlgItemInt16};
use crate::util::{CONCAT11, CONCAT12, CONCAT22, ZEXT24};
use crate::pass::pass_1008::pass1_1008_4d72;
use crate::win_struct::{COLORREF, HBRUSH16, HWND16, HDC16, WNDCLASS16};
use crate::draw::draw_1010::draw_fn_1010_2a32;
use crate::pass::pass_1040::{pass1_1040_b54a, pass1_1040_a5d0};
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::pass::pass_1010::pass1_1010_a50c;
use crate::pass::pass_1000::pass1_1000_5586;
use crate::mem_1000::mem_op_1000_179c;
use crate::struct_ops::struct_1040::struct_1040_a598;
use crate::mixed::mixed_1010_20ba;
use crate::defines::{Struct20, Struct18, Struct10, Struct19};
use crate::global::AppContext;

pub unsafe fn draw_op_1038_92f6(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: HWND16,
    param_6: &mut WNDCLASS16,
    in_dx: &mut Struct19,
    unaff_di: i16)
{
    let u_var1: u32;
    let ppc_var2: u32;
    let u_var3: u16;
    let i_var4: i16;
    let pa_var5: &mut Struct18;
    // let in_dx: *mut u8;
    // let pu_var6: &mut Struct19;
    let pu_var7: *mut u8;
    // let unaff_di: i16;
    let u_var8: u16;
    let local_1a: [bool; 0x2];
    let ustack22: u16;
    let pa_stack20: &mut Struct18;
    let pa_stack16: &mut Struct18;
    let i_stack12: i16;
    let pa_stack10: &mut Struct18;
    let pa_stack6: &mut Struct20;

    if param_4._2_2_ == 0xeb {
        pa_stack6 = mixed_1010_20ba(ctx, ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, in_dx, unaff_di);
        // pu_var6 = (pa_stack6 >> 0x10);
        pa_var5 = (param_1 + 0x90);
        if pa_var5 != 0x0 {
            pa_stack10 = pa_var5;
            mem_op_1000_179c(ctx, 0x18, pa_stack6, 0x1000);
            u_var3 = pa_var5;
            pa_stack16 = (pa_var5 & 0xffff | ZEXT24(pa_stack6) << 0x10);
            pu_var7 = (pa_stack6 | u_var3);
            if pu_var7 == 0x0 {
                u_var3 = 0x0;
                pu_var7 = 0x0;
            } else {
                struct_1040_a598((pa_var5 & 0xffff | ZEXT24(pa_stack6) << 0x10));
            }
            (param_1 + 0x90) = u_var3;
            (param_1 + 0x92) = pu_var7;
            (param_1 + 0x90) = 0x11;
            i_stack12 = *(param_1 + 0x90);
            u_var3 = i_stack12 * 0xa + 0x2;
            mem_op_1000_179c(ctx,u_var3, pu_var7, 0x1000);
            pa_stack16 = CONCAT22(pu_var7, u_var3);
            if (pu_var7 | u_var3) == 0x0 {
                u_var1 = (param_1 + 0x90);
                (u_var1 + 0x2) = 0x0;
            } else {
                pa_stack16 = i_stack12;
                pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, i_stack12, 0xa,
                                u_var3 + 0x2, pu_var7);
                u_var1 = (param_1 + 0x90);
                // u_var8 = (u_var1 >> 0x10);
                i_var4 = u_var1;
                (i_var4 + 0x2) = u_var3 + 0x2;
                (i_var4 + 0x4) = pu_var7;
            }
            // u_var8 = (pa_stack10 >> 0x10);
            u_var1 = (param_1 + 0x90);
            (u_var1 + 0x6) = (pa_stack10 + 0x6);
            u_var1 = (param_1 + 0x90);
            (u_var1 + 0xa) = (pa_stack10 + 0xa);
            u_var1 = (param_1 + 0x90);
            (u_var1 + 0x12) = (param_1 + 0xa);
            u_var8 = 0x1010;
            pass1_1010_a50c(pa_stack6, 0x10505b42, (param_1 + 0x90));
            pa_stack20 = pa_stack10;
            pa_stack16 = pa_stack10;
            if pa_stack10 != 0x0 {
                pass1_1040_a5d0(pa_stack10);
                u_var8 = 0x1000;
                fn_ptr_1000_17ce(ctx,pa_stack10, 0x1000);
            }
            ppc_var2 = (CONCAT22(param_2, param_1) + 0x70);
            (**ppc_var2)(u_var8, param_1, param_2);
        }
    } else {
        if param_4._2_2_ != 0xf9 {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, in_dx, &ctx.PTR_LOOP_1050_1040,
                            param_6);
            return;
        }
        i_var4 = pass1_1038_993a(param_1, param_2, param_3);
        if -0x1 < i_var4 {
            u_var8 = (param_1 + 0x6);
            ustack22 = GetDlgItemInt16(param_5, 0x1, local_1a, param_6);
            if local_1a[0] != 0x0 {
                u_var1 = (param_1 + 0x98);
                draw_fn_1010_2a32(0x94be, ctx.s_tile2_bmp_1050_1538, u_var1,
                                  (u_var1 >> 0x10), ustack22,
                                  CONCAT22(u_var8, (i_var4 * 0xe + 0x5a72)),
                                  in_dx, param_1, &stack0xfffe, param_2);
            }
        }
    }
    return;
}


pub fn draw_op_1038_9dcc(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct10,
    param_2: i16,
    param_3: u16,
    in_colorref_4: COLORREF,
    param_5: u16)
{
    let pu_var1: *mut u16;
    let b_var2: bool;
    let u_var3: u16;
    let i_var4: i16;
    let local_brush_handle: HBRUSH16;
    let u_var5: i32;
    let extraout_dx: u16;
    let local_struct_5: &mut Struct10;
    let var5: &mut Struct10;
    let color: COLORREF;
    let u_var6: u32;
    let u_stack14: u16;

    // var5 = (in_struct_1 >> 0x10);
    local_struct_5 = in_struct_1;
    color = in_colorref_4;
    if local_struct_5.brush_handle_field_0x8e == 0x0 {
        color = ctx.s_tile2_bmp_1050_1538;
        local_brush_handle = CreateSolidBrush16(in_colorref_4);
        local_struct_5.brush_handle_field_0x8e = local_brush_handle;
    }
    if ctx.PTR__LOOP_1050_5b64 == 0x0 {
        color = 0x1008;
        u_var6 = pass1_1008_4d72((ctx.PTR__LOOP_1050_4230 + 0xe));
        // u_var3 = (u_var6 >> 0x10);
        i_var4 = u_var6;
        ctx._PTR_LOOP_1050_5b64 = CONCAT12(*(i_var4 + 0x94),
                                           CONCAT11(*(i_var4 + 0x95),
                                                    *(i_var4 + 0x96)));
        ctx.PTR_LOOP_1050_5b68 = CONCAT11(*(i_var4 + 0x3e5), *(i_var4 + 0x3e6));
        ctx.PTR_LOOP_1050_5b6a = (i_var4 + 0x3e4);
    }
    if 0x5 < param_3 {
        if param_3 != 0x6 {
            return;
        }
        b_var2 = false;
        // TODO: refactor for loop
        // for (u_stack14 = 0x0; pu_var1 = &local_struct_5.field_0x128,
        //     u_stack14 <= *pu_var1 && *pu_var1 != u_stack14; u_stack14 += 0x1) {
        //   if ((&local_struct_5.field_0x94 + u_stack14 * 0x2) == param_2) {
        //     b_var2 = true;
        //     break;
        //   }
        // }
        if b_var2 {
            ctx.PTR_LOOP_1050_5b64 = ctx.PTR_LOOP_1050_5b68;
        }
    }
    SetTextColor16(ctx.PTR_LOOP_1050_5b64 as HDC16, color);
    SetBkColor16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0x0);
    return;
}


