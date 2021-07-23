use crate::{
    fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708},
    mem_1000::mem_op_1000_160a,
    pass::pass_1008::{pass1_1008_3e76, pass1_1008_818c},
    string::string_1000::string_1000_3d3e,
    util::{CONCAT22, ZEXT24},
    win_struct::{
        HBRUSH16, HCURSOR16, HDC16, HGDIOBJ16, HPEN16, HWND16, PAINTSTRUCT16, PALETTEENTRY, RECT16,
    },
    winapi::{
        BeginPaint16, CreatePen16, CreateSolidBrush16, DeleteObject16, EndPaint16, GetClientRect16,
        GetDC16, GetDeviceCaps16, GetStockObject16, GetSystemPaletteEntries, LineTo16, MoveTo16,
        Polygon16, Rectangle16, ReleaseDC16, SelectObject16,
    },
};
use crate::defines::{Struct19, Struct79, Struct80, U32Ptr, Struct20};
use crate::global::AppContext;
use crate::mixed::mixed_1010_20ba;
use crate::struct_ops::struct_1008::{clear_struct_1008_3e38, set_struct_1008_687a};
use crate::struct_ops::struct_1010::set_struct_fields_1010_1d48;
use crate::ui::ui_1008::{fill_rect_1008_39ac, win_ui_reg_class_1008_96d2};
use crate::util::{read_vec_from_addr, struct_from_addr, read_string_from_rsrc};
use crate::win_struct::{COLORREF, HINSTANCE16, WNDCLASS16};
use crate::winapi::{FillRect16, LoadCursor16};

pub fn draw_op_1008_1230(param_1: HWND16) {
    fill_rect_1008_39ac(param_1);
    return;
}

pub unsafe fn unk_draw_op_1008_61b2(
    ctx: &mut AppContext,
    struct_4: &mut Struct20,
    param_2: u16,
    param_3: u16,
    param_4: i32,
    wnd_class: &mut WNDCLASS16,
    extraout_dx: U32Ptr,
    unaff_di: i16,
)  {
    let gdi_obj_1: HGDIOBJ16;
    let cursor_1: HCURSOR16;
    let mut struct_1: Struct19;
    let struct_2: &mut Struct19;
    let struct_3: &mut Struct20;
    let u_var1: u16;

    set_struct_1008_687a(struct_4, param_4);
    struct_4.field_0xde = param_2;
    struct_4.field_0xe0 = 0x0;
    struct_4.field_0x0 = 0x6378;
    struct_4.field_0x2 = 0x1008;
    struct_1 = struct_from_addr::<Struct79>(extraout_dx);
    string_1000_3d3e(
        (struct_4 & 0xffff0000 | &i_var1.field_0x5b),
        &mut ctx.s_DanBrotherton_1050_0302,
    );
    gdi_obj_1 = GetStockObject16(0x1000);
    struct_4.hgdiobj_field_0xc6 = gdi_obj_1;
    cursor_1 = LoadCursor16(ctx.s_tile2_bmp_1050_1538 as HINSTANCE16, &read_string_from_rsrc(0x7f00));
    struct_4.hcursor_field_0xc4 = cursor_1;
    struct_4.field_0xc8 = 0x200b;
    struct_4.field_0xac = 0x45000000;
    struct_4.field_0xbc = (param_4 + 0x8);
    struct_2 = mixed_1010_20ba(
        ctx,
        ctx.PTR__LOOP_1050_0ed0,
        0x48,
        wnd_class,
        &mut struct_1,
        unaff_di,
        extraout_dx as u16,
    );
    struct_4.field_0xb4 = 0x0;
    struct_4.field_0xb6 = 0x0;
    struct_4.field_0xb8 = (struct_2 + 0xa);
    struct_4.field_0xba = (struct_2 + 0xc);
    struct_4.field_0xca = param_3;
    win_ui_reg_class_1008_96d2(ctx, struct_4, 0x1010, wnd_class, 0);
}

pub fn fill_rect_1008_62c0(ctx: &mut AppContext, win_handle: HWND16) {
    let mut rect: RECT16;
    let mut paint: PAINTSTRUCT16;

    let hstack36 = BeginPaint16(win_handle, &mut paint);
    let brush = CreateSolidBrush16(ctx.s_tile2_bmp_1050_1538 as COLORREF);
    GetClientRect16(ctx.s_tile2_bmp_1050_1538 as HDC16, &mut rect);
    FillRect16(ctx.s_tile2_bmp_1050_1538 as HDC16, &rect, brush);
    EndPaint16(ctx.s_tile2_bmp_1050_1538 as HDC16, &mut paint);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538 as HDC16);
    return;
}

pub fn unk_draw_op_1008_7f62(
    ctx: &mut AppContext,
    param_1: &mut Struct20,
    param_2: u16,
    param_3: i32,
    param_4: &mut WNDCLASS16,
) -> *mut Struct20 {
    let HVar1: HGDIOBJ16;
    let HVar2: HCURSOR16;
    let u_var4: &mut Struct20;
    let iVar3: &mut Struct20;

    iVar3 = param_1;
   // u_var4 = (param_1 >> 0x10);
    set_struct_1008_687a(param_1, param_3);
    iVar3.field_0xde = param_2;
    param_1.field_0x0 = 0x8042;
    iVar3.field_0x2 = 0x1008;
    string_1000_3d3e(
        (param_1 & 0xffff0000 | &iVar3.field_0x5b),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(0x1000);
    iVar3.hgdiobj_field_0xc6 = HVar1;
    HVar2 = LoadCursor16(s_tile2_bmp_1050_1538, &read_string_from_rsrc(0x7f00));
    iVar3.hcursor_field_0xc4 = HVar2;
    iVar3.field_0xc8 = 0x2008;
    iVar3.field_0xac = 0x44000000;
    iVar3.field_0xbc = (param_3 + 0x8);
    iVar3.field_0xca = iVar3.field_0xde;
    win_ui_reg_class_1008_96d2(ctx, param_1, s_tile2_bmp_1050_1538, param_4, 0);
    return param_1;
}

pub fn unk_draw_op_1008_80ee(
    ctx: &mut AppContext,
    param_1: &mut Struct23,
    param_2: &WNDCLASS16,
    stack_0xfffe: u16
) -> *mut Struct23 {
    let hvar1: HCURSOR16;
    let hvar2: HGDIOBJ16;
    let i_var3: &mut Struct23;
    let u_var3: &mut Struct23;

   // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    param_1.field_0x0 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    i_var3.field_0x54 = 0x0;
    i_var3.field_0x56 = 0x0;
    i_var3.field_0x58 = 0x0;
    param_1.field_0x0 = 0x87c8;
    i_var3.field_0x2 = 0x1008;
    string_1000_3d3e(
        (param_1 & 0xffff0000 | &i_var3.field_0x4),
        ctx.s_MicroSpinControl_1050_0370,
    );
    i_var3.field_0x54 = 0x3;
    hvar1 = LoadCursor16(0x1000, &read_string_from_rsrc(0x7f00));
    i_var3.field_0x58 = hvar1;
    hvar2 = GetStockObject16(s_tile2_bmp_1050_1538);
    i_var3.field_0x56 = hvar2;
    pass1_1008_818c(
        ctx,
        param_1,
        ctx.s_tile2_bmp_1050_1538 as HINSTANCE16,
        param_2,
        stack_0xfffe
    );
    return param_1;
}

pub fn draw_op_1008_8288(ctx: &mut AppContext, param_1: u16, param_2: u32, param_3: HWND16) {
    let gdi_obj_1: HGDIOBJ16;
    let gid_obj_2: HGDIOBJ16;
    let x: i16;
    let u_var3: u16;
    let mut rect: RECT16 = RECT16::new();
    let rect_top: i16 = 0;
    let rect_left: i16 = 0;
    let brush: HBRUSH16;
    let pen_1: HPEN16;
    let pen_2: HPEN16;
    let draw_ctx: HDC16;
    let mut u_stack72: i16;
    let mut u_stack70: i16;
    let mut x_coord: i16 = 0;
    let mut u_stack66: i16;
    let mut u_stack64: i16;
    let mut y_coord: i16 = 0;
    let mut paint: PAINTSTRUCT16;
    let local_1c: i16;
    let i_stack26: i16;
    let i_stack24: i16;
    let i_stack22: i16;
    let i_stack20: i16;
    let i_stack18: i16;
    let local_10: i16;
    let i_stack14: i16;
    let i_stack12: i16;
    let u_stack10: i16;
    let i_stack8: i16;
    let i_stack6: i16;
    let mut b_var1: bool = false;

    draw_ctx = BeginPaint16(param_3, &mut paint);
    b_var1 = false;
    pen_2 = CreatePen16(
        ctx.s_tile2_bmp_1050_1538 as i16,
        ctx._PTR_LOOP_1050_0368 as i16,
        (ctx.PTR__LOOP_1050_0368 >> 0x10),
    );
    pen_1 = CreatePen16(
        s_tile2_bmp_1050_1538,
        ctx.DAT_1050_0364,
        (ctx.DAT_1050_0364 >> 0x10),
    );
    brush = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    GetClientRect16(s_tile2_bmp_1050_1538, &mut rect);
    y_coord = rect_top;
    u_stack64 = rect_left;
    u_stack66 = rect_top >> 0x1;
    x_coord = rect_left >> 0x1;
    u_stack70 = rect_top >> 0x2;
    u_stack72 = rect_left >> 0x2;
    gdi_obj_1 = GetStockObject16(s_tile2_bmp_1050_1538);
    gdi_obj_1 = SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, gdi_obj_1);
    gid_obj_2 = GetStockObject16(s_tile2_bmp_1050_1538);
    gid_obj_2 = SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, gid_obj_2);
    Rectangle16(
        ctx.s_tile2_bmp_1050_1538 as HDC16,
        rect_left,
        rect_top,
        rect.y,
        rect.x,
    );
    MoveTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, x_coord, 0x0);
    LineTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, x_coord, y_coord);
    u_var3 = (param_2 >> 0x10) as u16;
    if ((param_2 + 0x4) & 0x4) != 0x0 {
        b_var1 = true;
    }
    local_10 = u_stack66 + b_var1;
    i_stack14 = u_stack72 + b_var1 + -0x2;
    i_stack12 = local_10 + -0x3;
    iStack10 = u_stack72 + b_var1 + 0x1;
    i_stack8 = local_10 + 0x3;
    i_stack6 = iStack10;
    SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, pen_2);
    if b_var1 == false {
        MoveTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, x_coord - 0x2, 0x1);
        LineTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0x1, 0x1);
        LineTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0x1, y_coord - 0x1);
    }
    b_var1 = (((param_2 + 0x4) & 0x8) != 0x0);
    local_1c = u_stack66 + b_var1;
    i_stack22 = (u_stack64 - u_stack72) + b_var1;
    i_stack26 = i_stack22 + 0x1;
    i_stack24 = local_1c + -0x3;
    i_stack22 += -0x2;
    i_stack20 = local_1c + 0x3;
    i_stack18 = i_stack22;
    if b_var1 == false {
        MoveTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, rect_left - 0x2, 0x1);
        x = x_coord + 0x1;
        LineTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, x, 0x1);
        LineTo16(ctx.s_tile2_bmp_1050_1538 as HDC16, x, y_coord - 0x1);
    }
    SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, pen_1);
    SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, brush);
    Polygon16(
        ctx.s_tile2_bmp_1050_1538 as HDC16,
        (&ctx.PTR_LOOP_1050_0002 + 0x1),
        local_10,
    );
    Polygon16(
        ctx.s_tile2_bmp_1050_1538 as HDC16,
        (&ctx.PTR_LOOP_1050_0002 + 0x1),
        local_1c,
    );
    SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, gid_obj_2);
    SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, gdi_obj_1);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538 as HDC16);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538 as HDC16);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538 as HDC16);
    EndPaint16(s_tile2_bmp_1050_1538, &mut paint);
    return;
}

pub unsafe fn unk_draw_op_1008_da12(
    ctx: &mut AppContext,
    param_1: &mut Struct19,
    param_2: u16,
    param_3: u16,
) {
    let pi_var1: U32Ptr;
    let b_var2: u8;
    let u_var3: u32;
    let pu_var4: U32Ptr;
    let hdc: HDC16;
    let i16_var6: i16;
    // let i_var7: i16;
    let u_var8: i16;
    let i16_var5: i16;
    let start: u16;
    let u_var9: u16;
    let mut entries: Vec<PALETTEENTRY>;
    let count: i16;
    let i_var10: i16;
    let hwnd: HWND16;
    let pu_stack32: u16;
    let i_stack16: i16;
    let l_stack8: i32;

    set_struct_fields_1010_1d48(&mut struct_from_addr::<Struct79>(CONCAT22(param_2, param_1.field_0x0)), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xc = 0x0;
    clear_struct_1008_3e38(CONCAT22(param_2, param_1.field_0xe));
    param_1.field_0x14 = 0x0;
    param_1.field_0x16 = 0x0;
    param_1.field_0x18 = 0x0;
    CONCAT22(param_2, param_1.field_0x0) = 0xdc80;
    param_1.field_0x2 = 0x1008;
    let hdc = GetDC16(0x1010);
    let mut dev_caps = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0x8);
    param_1.field_0xa = dev_caps;
    dev_caps = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0xa);
    param_1.field_0xc = dev_caps;
    let mut i_var7 = param_1.field_0xc + -0x1e0;
    count = (i_var7 >> 0xf);
    pass1_1008_3e76(
        &mut read_vec_from_addr::<u16>(CONCAT22(param_2, param_1.field_0xe)),
        0x0,
        i_var7 / 0x2,
        ((param_1.field_0xa + -0x280) / 0x2) as u16,
    );
    let mut hwnd = ctx.s_tile2_bmp_1050_1538 as HWND16;
    let u_var8 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0x26);
    if (u_var8 & 0x100) != 0x0 {
        dev_caps = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0x68);
        param_1.field_0x14 = dev_caps;
        let i16_var5 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538 as HDC16, 0x6a);
        param_1.field_0x16 = i16_var5;
        if ctx.PTR__LOOP_1050_5f2c == 0x0 {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx, count, 0x1000);
        } else {
            count = ctx.PTR_LOOP_1050_5f2e as i16;
        }
        start = fn_ptr_op_1000_1708(
            ctx,
            (i16_var5 + 0x1) * 0x4,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c as u16,
            count,
            0x1000,
        );
        l_stack8 = CONCAT22(count as u16, start) as i32;
        i_var7 = param_1.field_0x16;
        if ctx.PTR__LOOP_1050_5f2c == 0x0 {
            ctx.PTR_LOOP_1050_5f2e = count as u32;
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx, count, 0x1000);
        } else {
        }
        u_var9 = fn_ptr_op_1000_1708(
            ctx,
            (i_var7 + 0x1) * 0x4,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c as u16,
            ctx.PTR_LOOP_1050_5f2e as u16,
            0x1000,
        );
        param_1.field_0x18 = u_var9;
        (&param_1.field_0x18 + 0x2) = ctx.PTR_LOOP_1050_5f2e as u16;
        if l_stack8 != 0x0 {
            if param_1.field_0x18 != 0x0 {
                entries = read_vec_from_addr((param_1.field_0x16 / 0x2) as u32);
                GetSystemPaletteEntries(0x1000, start, count as u16, entries.as_mut_slice());
                GetSystemPaletteEntries(
                    ctx.s_tile2_bmp_1050_1538 as HDC16,
                    entries * 0x4 + start,
                    count as u16,
                    entries.as_mut_slice(),
                );
                pu_stack32 = param_1.field_0x18;
                // TODO: refactor for loop
                // for (i_stack16 = 0x0; pu_var4 = pu_stack32, pi_var1 = &param_1.field_0x16,
                //     *pi_var1 != i_stack16 && i_stack16 <= *pi_var1; i_stack16 += 0x1) {
                //   b_var2 = (i_stack16 * 0x4 + start);
                //   i_var7 = i_stack16 * 0x4 + start;
                //   u_var3 = pu_stack32 >> 0x10;
                //   i_var10 = pu_stack32;
                //   pu_stack32 =
                //               (pu_stack32 & 0xffff0000 | (i_var10 + 0x4));
                //   *pu_var4 = CONCAT11(*(i_var7 + 0x1),*(i_var7 + 0x2));
                //   (i_var10 + 0x2) = b_var2;
                // }
            }
        }
        hwnd = 0x1000;
        fn_ptr_1000_17ce(ctx, struct_from_addr(CONCAT22(count as u16, start)), 0x1000);
    }
    ReleaseDC16(hwnd, hdc);
    return;
}
