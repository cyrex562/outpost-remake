use crate::cleanup::ui_cleanup_op_1040_782c;
use crate::defines::{
    Struct13, Struct14, Struct15, Struct161, Struct17, Struct18, Struct37, Struct43, Struct71,
    Struct76, Struct_1040_8b92, U32Ptr,
};
use crate::file::file_1010::unk_io_op_1010_830a;
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::global::AppContext;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1008::{pass1_1008_4772, pass1_1008_4d72};
use crate::pass::pass_1010::{pass1_1010_2ee2, pass1_1010_7b8c};
use crate::pass::pass_1040::pass1_1040_a5d0;
use crate::string::string_1000::str_op_1000_3da4;
use crate::ui::ui_1008::palette_op_1008_4e08;
use crate::util::{read_string_from_rsrc, CONCAT11, CONCAT12, CONCAT22, SUB42, ZEXT24};
use crate::win_struct::{
    COLORREF, HANDLE16, HBRUSH16, HDC16, HGDIOBJ16, HICON16, HINSTANCE16, HPALETTE16, HPEN16,
    HWND16, PAINTSTRUCT16, RECT16,
};
use crate::winapi::{
    lstrlen16, BeginPaint16, CreatePen16, CreateSolidBrush16, DeleteObject16, DrawIcon16,
    DrawText16, EndPaint16, FillRect16, FrameRect16, GetClientRect16, GetCurrentPosition16,
    GetDC16, GetDlgCtrlID16, GetProp16, GetStockObject16, GetSystemMetrics16, GetTextExtent16,
    GetWindowDC16, GetWindowLong16, GetWindowRect16, GrayString16, InvalidateRect16, IsIconic16,
    LineTo16, LoadIcon16, MoveTo16, PtInRect16, Rectangle16, ReleaseDC16, SelectObject16,
    SelectPalette16, SetBkColor16, SetMapMode16, SetTextColor16, TextOut16,
};

pub fn mix_draw_op_1040_21d6(ctx: &mut AppContext, param_1: u32, param_2: HWND16, param_3: u16) {
    let u_var1: u8;
    let u_var2: u8;
    let pa_var3: &mut Struct13;
    let ppc_var4: u32;
    let i_var5: i16;
    let b_force_background: HPALETTE16;
    let color: COLORREF;
    let color_00: COLORREF;
    let handle: HANDLE16;
    let in_dx: u16;
    let i_var6: i16;
    let rect: *mut RECT16;
    let u_var7: u32;
    let u_var8: u16;
    let hstack62: HGDIOBJ16;
    let local_24: HDC16;
    let local_22: PAINTSTRUCT16;

    // rect = (param_1 >> 0x10);
    i_var6 = param_1;
    u_var8 = (i_var6 + 0x6);
    local_24 = BeginPaint16(param_2, &local_22);
    pa_var3 = (ctx.PTR_LOOP_1050_4230 + 0xe);
    b_force_background = palette_op_1008_4e08(pa_var3, &local_24, in_dx, 0x1008);
    ppc_var4 = ((i_var6 + 0x8e) + 0x4);
    (**ppc_var4)(
        0x1008,
        (i_var6 + 0x8e),
        0xffffffff,
        &local_24,
        param_3,
        u_var8,
    );
    u_var7 = pass1_1008_4d72(pa_var3);
    // u_var8 = (u_var7 >> 0x10);
    i_var5 = u_var7;
    u_var1 = *(i_var5 + 0x3e5);
    u_var2 = *(i_var5 + 0x3e6);
    color = SetBkColor16(0x1008, 0x0);
    color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538, CONCAT11(u_var1, u_var2));
    hstack62 = 0x0;
    handle = GetProp16(ctx.s_tile2_bmp_1050_1538, 0x5ced);
    if (handle != 0x0) {
        hstack62 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
    }
    DrawText16(
        ctx.s_tile2_bmp_1050_1538,
        &ctx.PTR_LOOP_1050_0010,
        i_var6 + 0x92,
        rect,
        0xffff,
    );
    SetTextColor16(
        ctx.s_tile2_bmp_1050_1538,
        CONCAT11(*(i_var5 + 0x95), *(i_var5 + 0x96)),
    );
    DrawText16(
        ctx.s_tile2_bmp_1050_1538,
        &ctx.PTR_LOOP_1050_0010,
        i_var6 + 0x9a,
        rect,
        0xffff,
    );
    if (handle != 0x0) {
        SelectObject16(ctx.s_tile2_bmp_1050_1538, hstack62);
    }
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, color);
    SetTextColor16(ctx.s_tile2_bmp_1050_1538, color_00);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn draw_ui_op_1040_27cc(
    ctx: &mut AppContext,
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: COLORREF,
) -> u32 {
    let ppcVar1: u32;
    let u_var2: u16;
    let i_var3: i16;
    let HVar4: HBRUSH16;
    let IVar5: i16;
    let iVar6: i16;
    let uVar7: u16;
    let CVar8: COLORREF;
    let hdc: HWND16;
    let uVar9: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    CVar8 = param_4;
    if ((iVar6 + 0x4) == 0x0) {
        CVar8 = ctx.s_tile2_bmp_1050_1538;
        HVar4 = CreateSolidBrush16(param_4);
        (iVar6 + 0x4) = HVar4;
    }
    if (ctx.PTR_LOOP_1050_5cf8 == 0x0) {
        ppcVar1 = (*param_1 + 0x68);
        uVar9 = (**ppcVar1)(CVar8, param_1, (iVar6 + 0x6e));
        CVar8 = 0x1008;
        uVar9 = pass1_1008_4d72(uVar9);
        // u_var2 = (uVar9 >> 0x10);
        i_var3 = uVar9;
        ctx.PTR_LOOP_1050_5cf8 = CONCAT22(
            CONCAT11(0x2, *(i_var3 + 0x94)),
            CONCAT11(*(i_var3 + 0x95), *(i_var3 + 0x96)),
        );
    }
    hdc = CVar8;
    if (0x5 < param_3) {
        if (param_3 != 0x6) {
            return 0x0;
        }
        hdc = ctx.s_tile2_bmp_1050_1538;
        IVar5 = GetDlgCtrlID16(CVar8);
        if (((iVar6 + 0x94) != 0x0) && (IVar5 == 0xfb2)) {
            CVar8 = 0x0;
            //       TODO: goto LAB_1040_286e;
        }
    }
    CVar8 = ctx.PTR_LOOP_1050_5cf8;
    //LAB_1040_286e:
    SetTextColor16(hdc, CVar8);
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
    return CONCAT22(0x1050, (iVar6 + 0x4));
}

pub fn draw_op_1040_5a06(ctx: &mut AppContext, param_1: u32, param_2: HWND16, param_3: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let ppc_var3: u32;
    let u_var4: u32;
    let b_force_background: HPALETTE16;
    let iVar5: i16;
    let handle: HPEN16;
    let handle_00: HGDIOBJ16;
    let x: i16;
    let y: i16;
    let in_DX: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u16;
    let IVar9: i16;
    let u_var10: u32;
    let paVar11: &mut Struct43;
    let paVar12: &mut Struct76;
    let uVar13: u16;
    let pHVar14: HDC16;
    let uVar15: u16;
    let HVar16: HDC16;
    let HVar17: HDC16;
    let HVar18: HDC16;
    let uVar19: u16;
    let uVar20: u16;
    let uVar21: u16;
    let uStack82: u16;
    let iStack72: i16;
    let i_stack68: i16;
    let paStack54: &mut Struct76;
    let local_2c: HDC16;
    let local_2a: PAINTSTRUCT16;
    let local_a: [RECT16; 0x2];

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar21 = (iVar6 + 0x6);
    GetWindowRect16(param_2, local_a);
    uVar13 = (iVar6 + 0x6);
    local_2c = BeginPaint16(ctx.s_tile2_bmp_1050_1538, &local_2a);
    uVar8 = 0x1008;
    b_force_background =
        palette_op_1008_4e08((ctx.PTR_LOOP_1050_4230 + 0xe), &local_2c, in_DX, 0x1008);
    paStack54 = 0x0;
    if ((iVar6 + 0x98) != 0x0) {
        paStack54 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, (iVar6 + 0x98), param_3);
        IVar9 = 0x1008;
        u_var10 = pass1_1008_4772(paStack54);
        if (((u_var10 >> 0x10) | u_var10) == 0x0) {
            if (paStack54 != 0x0) {
                if (paStack54 != 0x0) {
                    ppc_var3 = paStack54;
                    (**ppc_var3)(0x1008, paStack54, (paStack54 >> 0x10), 0x1, uVar13);
                }
            }
            IVar9 = 0x1010;
            paStack54 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x4d, param_3);
        }
        uVar8 = SUB42(ctx.s_tile2_bmp_1050_1538, 0x0);
        GetSystemMetrics16(IVar9);
        ppc_var3 = (paStack54 + 0x4);
        (**ppc_var3)();
    }
    if (paStack54 != 0x0) {
        if (paStack54 != 0x0) {
            ppc_var3 = paStack54;
            (**ppc_var3)(uVar8, paStack54, (paStack54 >> 0x10), 0x1, uVar13, uVar21);
        }
    }
    paVar11 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, (iVar6 + 0x96), param_3);
    // uVar21 = (paVar11 >> 0x10);
    pHVar14 = &local_2c;
    uVar19 = 0x4;
    uVar20 = 0xd;
    uVar15 = param_3;
    IVar9 = GetSystemMetrics16(0x1010);
    iVar5 = -(IVar9 + -0x23);
    u_var4 = paVar11;
    ppc_var3 = u_var4 + 0x2;
    uVar13 = paVar11;
    uVar8 = uVar21;
    (**ppc_var3)();
    if (paVar11 != 0x0) {
        if (paVar11 != 0x0) {
            ppc_var3 = u_var4;
            (**ppc_var3)(
                ctx.s_tile2_bmp_1050_1538,
                paVar11,
                uVar21,
                0x1,
                uVar13,
                uVar8,
                iVar5,
                uVar19,
                uVar20,
                pHVar14,
                uVar15,
            );
        }
    }
    handle = CreatePen16(ctx.s_tile2_bmp_1050_1538, 0x25, 0x100);
    handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
    paVar12 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x4f, param_3);
    // uVar21 = (paVar12 >> 0x10);
    u_var10 = pass1_1008_4772(paVar12);
    // uVar13 = (u_var10 >> 0x10);
    u_var4 = (u_var10 + 0x4);
    u_var2 = (u_var10 + 0x8);
    IVar9 = GetSystemMetrics16(0x1008);
    iVar5 = -(IVar9 + -0xc1);
    IVar9 = GetSystemMetrics16(ctx.s_tile2_bmp_1050_1538);
    iStack72 = u_var2;
    x = 0xc5 - (IVar9 - iStack72);
    MoveTo16(ctx.s_tile2_bmp_1050_1538, iVar5, 0x82);
    i_stack68 = u_var4;
    y = i_stack68 * 0xa + 0x85;
    LineTo16(ctx.s_tile2_bmp_1050_1538, iVar5, y);
    HVar18 = local_2c;
    LineTo16(ctx.s_tile2_bmp_1050_1538, x, y);
    HVar17 = local_2c;
    LineTo16(ctx.s_tile2_bmp_1050_1538, x, 0x82);
    HVar16 = local_2c;
    LineTo16(ctx.s_tile2_bmp_1050_1538, iVar5, 0x82);
    // TODO: refactor for loop
    // for (uStack82 = 0x0; pu_var1 = (iVar6 + 0x94),
    //     uStack82 <= *pu_var1 && *pu_var1 != uStack82; uStack82 += 0x1) {
    //   pHVar14 = &local_2c;
    //   iVar5 = i_stack68 * uStack82 + 0x84;
    //   uVar13 = 0x4;
    //   uVar15 = param_3;
    //   IVar9 = GetSystemMetrics16(ctx.s_tile2_bmp_1050_1538);
    //   ppc_var3 = (paVar12 + 0x4);
    //   (**ppc_var3)(ctx.s_tile2_bmp_1050_1538,paVar12,uVar21,-(IVar9 + -0xc4),uVar13,
    //               iVar5,pHVar14,uVar15,HVar16,HVar17);
    // }
    if (paVar12 != 0x0) {
        if (paVar12 != 0x0) {
            ppc_var3 = paVar12;
            (**ppc_var3)(
                ctx.s_tile2_bmp_1050_1538,
                paVar12,
                uVar21,
                0x1,
                HVar16,
                HVar17,
                HVar18,
            );
        }
    }
    SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_00);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_2a);
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn draw_op_1040_7bb2(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct14,
    in_win_handle_2: HWND16,
    param_3: u16,
) {
    let ppcVar1: u32;
    let BVar2: bool;
    let y: i16;
    let i_var3: i16;
    let color: COLORREF;
    let handle: HPEN16;
    let handle_00: HGDIOBJ16;
    let rect: *mut RECT16;
    let handle_01: HANDLE16;
    let mut str_a: String;
    let i_var4: &mut Struct14;
    let mut count: String;
    let mut str_00: String;
    let u_var4: u32;
    let DVar5: u32;
    let hbrush: HBRUSH16;
    let u_var6: u16;
    let uVar7: u16;
    let local_obj_handle_42: HGDIOBJ16;
    let local_rect_12: RECT16;
    let iStack14: i16;
    let iStack12: i16;
    let HStack10: HPALETTE16;
    let uStack8: u32;
    let local_hdc_4: HDC16;

    // str_00 = (in_struct_1 >> 0x10);
    i_var4 = in_struct_1;
    uVar7 = i_var4.field_0x6;
    BVar2 = IsIconic16(in_win_handle_2);
    if (BVar2 == 0x0) {
        u_var6 = i_var4.field_0x6;
        local_hdc_4 = GetWindowDC16(ctx.s_tile2_bmp_1050_1538);
        ppcVar1 = (in_struct_1 + 0x68);
        uStack8 = (**ppcVar1)(
            ctx.s_tile2_bmp_1050_1538,
            in_struct_1,
            i_var4.field_0x6e,
            u_var6,
            uVar7,
        );
        if (uStack8 != 0x0) {
            HStack10 =
                palette_op_1008_4e08(uStack8, &local_hdc_4, (uStack8 >> 0x10) | uStack8, 0x1008);
            GetWindowRect16(0x1008, &local_rect_12);
            y = (iStack14 - local_rect_12.x) + -0x1;
            i_var3 = (iStack12 - local_rect_12.y) + -0x1;
            color = (-(i_var4.field_0x60 == 0x0) & 0x1e) + 0x25;
            handle = CreatePen16(ctx.s_tile2_bmp_1050_1538, color, 0x100);
            handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
            MoveTo16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0);
            LineTo16(ctx.s_tile2_bmp_1050_1538, 0x0, y);
            LineTo16(ctx.s_tile2_bmp_1050_1538, i_var3, y);
            LineTo16(ctx.s_tile2_bmp_1050_1538, i_var3, 0x0);
            LineTo16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0);
            u_var4 = GetWindowLong16(ctx.s_tile2_bmp_1050_1538, -0x10);
            if (((u_var4 & 0x800000) != 0x0) && ((u_var4 & 0x400000) != 0x0)) {
                i_var3 = i_var4.field_0x62 - i_var4.field_0x66;
                MoveTo16(ctx.s_tile2_bmp_1050_1538, i_var3, 0x0);
                LineTo16(ctx.s_tile2_bmp_1050_1538, i_var3, y);
                i_var4.field_0x7a = i_var4.field_0x64;
                i_var4.field_0x7c = i_var4.field_0x66;
                i_var4.field_0x7e = y;
                i_var4.field_0x80 = i_var4.field_0x62 - i_var4.field_0x66;
                hbrush = 0x4;
                rect = GetStockObject16(ctx.s_tile2_bmp_1050_1538);
                FillRect16(ctx.s_tile2_bmp_1050_1538, rect, hbrush);
                if (i_var4.field_0x76 != 0x0) {
                    draw_op_1040_82ee(ctx, in_struct_1, ctx.s_tile2_bmp_1050_1538);
                }
                count = &i_var4.field_0x10;
                if (*count != '\0') {
                    local_obj_handle_42 = 0x0;
                    handle_01 = GetProp16(ctx.s_tile2_bmp_1050_1538, 0x5de9);
                    if (handle_01 != 0x0) {
                        local_obj_handle_42 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_01);
                    }
                    SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
                    SetTextColor16(ctx.s_tile2_bmp_1050_1538, color);
                    let str_a = lstrlen16(ctx.s_tile2_bmp_1050_1538);
                    DVar5 = GetTextExtent16(ctx.s_tile2_bmp_1050_1538, str_a, count);
                    TextOut16(
                        ctx.s_tile2_bmp_1050_1538,
                        str_a,
                        count,
                        &str_00,
                        (i_var4.field_0x80 - i_var4.field_0x7c) / 0x2 - (DVar5 >> 0x10) / 0x2,
                    );
                    if (handle_01 != 0x0) {
                        SelectObject16(ctx.s_tile2_bmp_1050_1538, local_obj_handle_42);
                    }
                }
            }
            ppcVar1 = (in_struct_1 + 0x64);
            (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, in_struct_1, uStack8, local_hdc_4);
            HStack10 = SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, HStack10);
            DeleteObject16(ctx.s_tile2_bmp_1050_1538);
            SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_00);
            DeleteObject16(ctx.s_tile2_bmp_1050_1538);
        }
        ReleaseDC16(ctx.s_tile2_bmp_1050_1538, local_hdc_4);
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_text_bk_color_1040_7e5e(
    ctx: &mut AppContext,
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: i16,
) -> u32 {
    let ppcVar1: u32;
    let i_var2: i16;
    let HVar3: HGDIOBJ16;
    let IVar4: i16;
    let hwnd: HWND16;
    let hdc: HWND16;
    let u_var5: u32;
    let color: COLORREF;
    let u_var6: u16;

    u_var6 = 0x4;
    hwnd = ctx.s_tile2_bmp_1050_1538;
    HVar3 = GetStockObject16(param_4);
    if (ctx.PTR_LOOP_1050_5df0 == 0x0) {
        ppcVar1 = (*param_1 + 0x68);
        u_var5 = (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, param_1, (param_1 + 0x6e), u_var6);
        if (u_var5 == 0x0) {
            return 0x0;
        }
        hwnd = 0x1008;
        u_var5 = pass1_1008_4d72(u_var5);
        // u_var6 = (u_var5 >> 0x10);
        i_var2 = u_var5;
        ctx.PTR_LOOP_1050_5df0 = CONCAT22(
            CONCAT11(0x2, *(i_var2 + 0x94)),
            CONCAT11(*(i_var2 + 0x95), *(i_var2 + 0x96)),
        );
    }
    hdc = hwnd;
    if (0x5 < param_3) {
        if (param_3 != 0x6) {
            return 0x0;
        }
        hdc = ctx.s_tile2_bmp_1050_1538;
        IVar4 = GetDlgCtrlID16(hwnd);
        if (IVar4 == 0x14c) {
            color = 0x0;
            //       TODO: goto LAB_1040_7f00;
        }
        if (IVar4 == 0x175) {
            color = 0x0;
            //       TODO: goto LAB_1040_7f00;
        }
    }
    color = ctx.PTR_LOOP_1050_5df0;
    //LAB_1040_7f00:
    SetTextColor16(hdc, color);
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
    return CONCAT22(0x1050, HVar3);
}

pub fn draw_op_1040_82ee(ctx: &mut AppContext, param_1: &mut Struct15, in_colorref_2: COLORREF) {
    let i_var1: &mut Struct15;
    let u_var1: u16;
    let local_1a: u32;
    let uStack22: u32;
    let local_12: i16;
    let iStack16: i16;
    let iStack14: i16;
    let iStack12: i16;
    let l_brush_handle: *mut RECT16;
    let iStack8: i16;
    let i_stack6: i16;
    let i_stack4: i16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_stack6 = (i_var1.field_0x80 - i_var1.field_0x7c) + -0x2;
    iStack8 = (-(i_var1.field_0x60 == 0x0) & 0x1e) + 0x25;
    i_stack4 = i_stack6;
    l_brush_handle = CreateSolidBrush16(in_colorref_2);
    if (i_var1.field_0x86 == 0x0) {
        local_1a = CONCAT22(i_var1.field_0x66 + 0x2, i_var1.field_0x64 + 0x2);
        uStack22 = CONCAT22(i_stack4, i_stack6);
        &i_var1.field_0x82 = local_1a;
        &i_var1.field_0x86 = uStack22;
    }
    local_12 = i_var1.field_0x82 + 0x2;
    iStack16 = (i_var1.field_0x88 - i_var1.field_0x84) / 0x2 + i_var1.field_0x84 + -0x2;
    iStack14 = i_var1.field_0x86 - 0x2;
    iStack12 = (i_var1.field_0x88 - i_var1.field_0x84) / 0x2 + i_var1.field_0x84 + 0x2;
    FrameRect16(
        ctx.s_tile2_bmp_1050_1538,
        l_brush_handle,
        &i_var1.field_0x82,
    );
    FrameRect16(ctx.s_tile2_bmp_1050_1538, l_brush_handle, &local_12);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    i_var1.field_0x7a = i_var1.field_0x86 + 0x2;
    return;
}

pub fn mixed_draw_op_1040_8a06(ctx: &mut AppContext, param_1: u32, param_2: HWND16, param_3: u16) {
    let u_var1: u8;
    let u_var2: u8;
    let paVar3: &mut Struct13;
    let u_var4: u16;
    let b_force_background: HPALETTE16;
    let color: COLORREF;
    let color_00: COLORREF;
    let handle: HANDLE16;
    let in_DX: u16;
    let rect: *mut RECT16;
    let u_var5: u32;
    let HStack62: HGDIOBJ16;
    let local_24: HDC16;
    let local_22: PAINTSTRUCT16;

    // rect = (param_1 >> 0x10);
    local_24 = BeginPaint16(param_2, &local_22);
    paVar3 = (ctx.PTR_LOOP_1050_4230 + 0xe);
    b_force_background = palette_op_1008_4e08(paVar3, &local_24, in_DX, 0x1008);
    u_var5 = pass1_1008_4d72(paVar3);
    // u_var4 = (u_var5 >> 0x10);
    u_var1 = *(u_var5 + 0x95);
    u_var2 = *(u_var5 + 0x96);
    DrawIcon16(0x1008, (param_1 + 0x8e), 0xa, 0x14);
    color = SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
    color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538, CONCAT11(u_var1, u_var2));
    HStack62 = 0x0;
    handle = GetProp16(ctx.s_tile2_bmp_1050_1538, 0x5dfa);
    if (handle != 0x0) {
        HStack62 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
    }
    DrawText16(
        ctx.s_tile2_bmp_1050_1538,
        &ctx.PTR_LOOP_1050_0010,
        param_1 + 0x9e,
        rect,
        0xffff,
    );
    if (handle != 0x0) {
        SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack62);
    }
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, color);
    SetTextColor16(ctx.s_tile2_bmp_1050_1538, color_00);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn load_icon_1040_8b92(param_1: &mut Struct_1040_8b92, instance_handle: HINSTANCE16) {
    let b_var1: u8;
    let icon_handle: HICON16;

    let mut name: String;

    b_var1 = (param_1.field_0x98) & 0xf0;
    if b_var1 == 0x30 {
        name = read_string_from_rsrc(0x7f03);
    } else {
        if (b_var1 == 0x10) || (b_var1 == 0x10) {
            name = read_string_from_rsrc(0x7f01);
        } else {
            if (b_var1 == 0x40) || (b_var1 == 0x40) {
                name = read_string_from_rsrc(0x7f04);
            } else {
                if b_var1 != 0x20 {
                    return;
                }
                name = read_string_from_rsrc(0x7f02);
            }
        }
    }
    icon_handle = LoadIcon16(instance_handle, &name);
    (param_1.field_0x8e) = icon_handle;
    return;
}

pub fn draw_text_1040_8d14(ctx: &mut AppContext, param_1: &mut Struct37, param_2: &mut HWND16) {
    let b_var1: u8;
    let ivar2: i16;
    let handle: HANDLE16;
    // let i_var3: &mut Struct37;
    let rect: RECT16;
    let obj_handle: HGDIOBJ16;

    // rect = (param_1 >> 0x10);
    //  i_var3 = param_1;
    b_var1 = param_1.field_0x98 & 0xf0;
    if (((b_var1 == 0x30) || (b_var1 == 0x10)) || (b_var1 == 0x40)) || (b_var1 == 0x20) {
        param_1.field_0xa0 = 0xa;
        ivar2 = GetSystemMetrics16(*param_2 as i16);
        param_1.field_0x9e = ivar2 + 0x28;
        param_2.field_0x0 = ctx.s_tile2_bmp_1050_1538 as u16;
    } else {
        param_1.field_0xa0 = 0xa;
        param_1.field_0x9e = 0x14;
    }
    obj_handle = 0x0;
    handle = GetProp16(*param_2, &read_string_from_rsrc(0x5e0f));
    if handle != 0x0 {
        obj_handle = SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, handle);
    }
    DrawText16(
        ctx.s_tile2_bmp_1050_1538 as HDC16,
        &read_string_from_rsrc(0x410),
        param_1.field_0x9e,
        &rect,
        0xffff,
    );
    if obj_handle != 0x0 {
        SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, obj_handle);
    }
    return;
}

pub fn unk_draw_op_1040_9458(
    ctx: &mut AppContext,
    param_1: &mut Struct17,
    param_2: u8,
    param_3: u16,
    param_4: HDC16,
) {
    let ppc_var1: u32;
    let uvar2: i32;
    let u_var3: u16;
    // let i_var4: &mut Struct17;
    let u_var4: u16;
    let pu_stack8: U32Ptr;
    let pu_stack6: &mut u32;

    // u_var4 = (param_1 >> 0x10);
    // i_var4 = param_1;
    if param_1.field_0x8 != 0x0 {
        pu_stack6 = param_1.field_0x8;
        u_var3 = (&param_1.field_0x8 + 0x2);
        if (((&param_1.field_0xc + 0x2) | &param_1.field_0xc) != 0x0) && ((param_2 & 0x1) != 0x0) {
            pu_stack6 = param_1.field_0xc;
            u_var3 = (&param_1.field_0xc + 0x2);
        }
        if (param_1.field_0x10 != 0x0) && ((param_2 & 0x4) != 0x0) {
            pu_stack6 = param_1.field_0x10;
            u_var3 = (&param_1.field_0x10 + 0x2);
        }
        pu_stack8 = &param_3;
        uvar2 = *pu_stack6;
        ppc_var1 = (uvar2 + 0x8);
        (**ppc_var1)(param_4, pu_stack6, u_var3, pu_stack8);
        ppc_var1 = (uvar2 + 0x4);
        (**ppc_var1)(
            param_4,
            pu_stack6,
            param_1.field_0x28,
            param_1.field_0x26,
            &param_3,
        );
        SelectPalette16(param_4, 0x0, pu_stack8);
        DeleteObject16(ctx.s_tile2_bmp_1050_1538 as HGDIOBJ16);
    }
    return;
}

pub fn draw_text_1040_94fc(ctx: &mut AppContext, param_1: &mut Struct37, draw_handle: HDC16) {
    let bk_color: COLORREF;
    let txt_color: COLORREF;
    let rect: RECT16;

    bk_color = SetBkColor16(draw_handle, param_1.field_0x3a);
    txt_color = SetTextColor16(ctx.s_tile2_bmp_1050_1538 as HDC16, param_1.field_0x3c);
    DrawText16(
        ctx.s_tile2_bmp_1050_1538 as HDC16,
        &ctx.PTR_LOOP_1050_0010,
        param_1.field_0x2e,
        &rect,
        0xffff,
    );
    SetBkColor16(ctx.s_tile2_bmp_1050_1538 as HDC16, bk_color);
    SetTextColor16(ctx.s_tile2_bmp_1050_1538 as HDC16, txt_color);
    return;
}

pub fn draw_text_1040_9650(ctx: &mut AppContext, param_1: &mut Struct161, param_2: HWND16) {
    let hdc: HDC16;

    hdc = GetDC16(param_2);
    DrawText16(
        ctx.s_tile2_bmp_1050_1538 as HDC16,
        0x410,
        param_1 + 0x2e,
        (param_1 >> 0x10),
        0xffff,
    );
    ReleaseDC16(ctx.s_tile2_bmp_1050_1538 as HDC16, hdc);
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn draw_op_1040_9948(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u32,
    param_3: HWND16,
    param_4: &RECT16,
) {
    let i_var1: i16;
    let i_var2: i16;
    let mode: i16;
    let u_var3: u16;
    let handle: HPEN16;
    let handle_00: HPEN16;
    let handle_01: HGDIOBJ16;
    let color: U32Ptr;
    let color_00: COLORREF;
    let color_01: COLORREF;
    let i_var4: &mut Struct71;
    let y: i16;
    let mut x: String;
    let cx: i16;
    let cy: i16;
    let iStack88: i16;
    let iStack86: i16;
    let iStack84: i16;
    let iStack82: i16;
    let iStack80: i16;
    let iStack78: i16;
    let local_42: PAINTSTRUCT16;
    let uStack34: u16;
    let uStack32: u16;
    let HStack30: HGDIOBJ16;
    let iStack28: i16;
    let iStack26: i16;
    let iStack24: i16;
    let iStack22: i16;
    let iStack20: i16;
    let local_12: RECT16;
    let uStack14: u32;
    let local_a: i16;
    let iStack8: i16;
    let i_stack6: i16;
    let i_stack4: i16;

    i_stack6 = 0x1;
    i_stack4 = 0x1;
    local_a = 0x0;
    iStack8 = 0x0;
    iStack28 = 0x0;
    HStack30 = 0x0;
    // y = (param_2 >> 0x10);
    i_var4 = param_2;
    uStack32 = i_var4.field_0x4 & 0x8;
    uStack34 = i_var4.field_0x56 & 0x1;
    BeginPaint16(param_3, &local_42);
    mode = SetMapMode16(ctx.s_tile2_bmp_1050_1538, 0x1);
    GetClientRect16(ctx.s_tile2_bmp_1050_1538, &local_12);
    // i_var2 = (uStack14 >> 0x10);
    i_var1 = i_var2 + -0x1;
    uStack14 = CONCAT22(i_var1, uStack14 + -0x1);
    if (uStack34 != 0x0) {
        iStack26 = local_12;
        // iStack24 = (local_12 >> 0x10);
        local_12 = CONCAT22(iStack24 + 0x2, iStack26 + 0x2);
        uStack14 = CONCAT22(i_var2 + -0x3, uStack14 + -0x3);
        iStack22 = uStack14 + -0x1;
        iStack20 = i_var1;
    }
    if (i_var4.field_0x6 != '\0') {
        iStack28 = 0x1;
        if (i_var4.field_0x5a != 0x0) {
            HStack30 = SelectObject16(ctx.s_tile2_bmp_1050_1538, i_var4.field_0x5a);
        }
        u_var3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(&i_var4.field_0x6)));
        DrawText16(0x1000, 0x400, &local_a, param_4, u_var3);
        iStack8 = ((uStack14._2_2_ - i_stack4) + iStack8) / 0x2 + local_12.y;
        i_stack4 += iStack8;
        local_a = ((uStack14 - i_stack6) + local_a) / 0x2 + local_12.x;
        i_stack6 += local_a;
    }
    handle = CreatePen16(
        ctx.s_tile2_bmp_1050_1538,
        ctx.DAT_1050_5ec2,
        (ctx.DAT_1050_5ec2 >> 0x10),
    );
    handle_00 = CreatePen16(
        ctx.s_tile2_bmp_1050_1538,
        ctx.DAT_1050_5ebe,
        (ctx.DAT_1050_5ebe >> 0x10),
    );
    handle_01 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
    if (uStack34 != 0x0) {
        iStack78 = 0x0;
        loop {
            MoveTo16(ctx.s_tile2_bmp_1050_1538, iStack20, iStack26);
            LineTo16(ctx.s_tile2_bmp_1050_1538, iStack20, iStack22);
            LineTo16(ctx.s_tile2_bmp_1050_1538, iStack24, iStack22);
            LineTo16(ctx.s_tile2_bmp_1050_1538, iStack24, iStack26);
            LineTo16(ctx.s_tile2_bmp_1050_1538, iStack20, iStack26);
            iStack24 += 0x1;
            iStack26 += 0x1;
            iStack22 += -0x1;
            iStack20 += -0x1;
            iStack78 += 0x1;
            if iStack78 >= 0x1 {
                break;
            }
        }
    }
    if ((i_var4.field_0x4 & 0x2) == 0x0) {
        // iStack84 = (local_12 >> 0x10);
        iStack82 = uStack14;
        iStack78 = 0x0;
        iStack86 = local_12.x;
        iStack80 = uStack14._2_2_;
        loop {
            SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
            MoveTo16(ctx.s_tile2_bmp_1050_1538, iStack80, iStack86);
            LineTo16(ctx.s_tile2_bmp_1050_1538, iStack80, iStack82);
            LineTo16(ctx.s_tile2_bmp_1050_1538, iStack84, iStack82);
            iStack88 = 0x0;
            loop {
                SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_00);
                LineTo16(ctx.s_tile2_bmp_1050_1538, iStack84, iStack86);
                LineTo16(ctx.s_tile2_bmp_1050_1538, iStack80, iStack86);
                iStack88 += 0x1;
                if iStack88 >= 0x2 {
                    break;
                }
            }
            iStack84 += 0x1;
            iStack86 += 0x1;
            iStack82 += -0x1;
            iStack80 += -0x1;
            iStack78 += 0x1;
            if iStack78 >= 0x2 {
                break;
            }
        }
    } else {
        MoveTo16(ctx.s_tile2_bmp_1050_1538, uStack14._2_2_, local_12.x);
        LineTo16(ctx.s_tile2_bmp_1050_1538, local_12.y, local_12.x);
        LineTo16(ctx.s_tile2_bmp_1050_1538, local_12.y, uStack14 + 0x1);
        if (iStack28 != 0x0) {
            iStack8 += 0x2;
            local_a += 0x2;
            i_stack6 += 0x2;
            i_stack4 += 0x2;
        }
    }
    MoveTo16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0);
    if (iStack28 != 0x0) {
        if ((i_var4.field_0x4 & 0x4) == 0x0) {
            color = ctx.PTR_LOOP_1050_5ec6;
            if (uStack32 != 0x0) {
                color = ctx.DAT_1050_5eca;
            }
            color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538, color);
            color_01 = SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
            u_var3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(&i_var4.field_0x6)));
            DrawText16(
                0x1000,
                (&ctx.PTR_LOOP_1050_0000 + 0x1),
                &local_a,
                param_4,
                u_var3,
            );
            SetTextColor16(ctx.s_tile2_bmp_1050_1538, color_00);
            SetBkColor16(ctx.s_tile2_bmp_1050_1538, color_01);
        } else {
            GetStockObject16(ctx.s_tile2_bmp_1050_1538);
            cx = 0x0;
            cy = 0x0;
            x = &i_var4.field_0x6;
            u_var3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(x)));
            GrayString16(
                0x1000,
                i_stack4 - iStack8,
                (i_stack6 - local_a),
                CONCAT22(local_a, iStack8),
                u_var3,
                x,
                y,
                cx,
                cy,
            );
        }
        if (HStack30 != 0x0) {
            SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack30);
        }
    }
    SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_01);
    SetMapMode16(ctx.s_tile2_bmp_1050_1538, mode);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_42);
    return;
}

pub fn draw_op_1040_a67e(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: i16,
    param_3: u16,
    param_4: COLORREF,
) {
    let pi_var1: U32Ptr;
    let bVar2: bool;
    let u_var3: u16;
    let i_var4: i16;
    let HVar5: HBRUSH16;
    let iVar6: i16;
    let uVar7: u16;
    let hdc: COLORREF;
    let uVar8: u32;
    let iStack14: i16;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    hdc = param_4;
    if ((iVar6 + 0x8e) == 0x0) {
        hdc = ctx.s_tile2_bmp_1050_1538;
        HVar5 = CreateSolidBrush16(param_4);
        (iVar6 + 0x8e) = HVar5;
    }
    if (ctx.PTR_LOOP_1050_5ee8 == 0x0) {
        hdc = 0x1008;
        uVar8 = pass1_1008_4d72((ctx.PTR_LOOP_1050_4230 + 0xe));
        // u_var3 = (uVar8 >> 0x10);
        i_var4 = uVar8;
        ctx.PTR_LOOP_1050_5ee8 = CONCAT12(
            *(i_var4 + 0x94),
            CONCAT11(*(i_var4 + 0x95), *(i_var4 + 0x96)),
        );
        ctx.PTR_LOOP_1050_5eec = CONCAT11(*(i_var4 + 0x3e5), *(i_var4 + 0x3e6));
        ctx.PTR_LOOP_1050_5eee = (i_var4 + 0x3e4);
    }
    if (0x5 < param_3) {
        if (param_3 != 0x6) {
            return;
        }
        bVar2 = false;
        // TODO: refactor for loop
        // for (iStack14 = 0x0; pi_var1 = (iVar6 + 0xea),
        //     *pi_var1 != iStack14 && iStack14 <= *pi_var1; iStack14 += 0x1) {
        //   if ((iVar6 + 0x9a + iStack14 * 0x2) == param_2) {
        //     bVar2 = true;
        //     break;
        //   }
        // }
        if (bVar2) {
            ctx.PTR_LOOP_1050_5ee8 = ctx.PTR_LOOP_1050_5eec;
        }
    }
    SetTextColor16(hdc, ctx.PTR_LOOP_1050_5ee8);
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_draw_op_1040_b0f8(ctx: &mut AppContext, param_1: &mut Struct18) {
    let u_var1: u16;
    let u_var2: u16;
    let in_DX: U32Ptr;
    let i_var3: &mut Struct18;
    let unaff_DI: i16;
    let u_var3: u16;
    let u_var4: u16;
    let unaff_SS: u16;
    let pu_var5: U32Ptr;
    let paStack10: &mut Struct18;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    param_1.field_0x0 = 0xb772;
    i_var3.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, unaff_SS, in_DX, unaff_DI);
    u_var4 = 0x1010;
    pass1_1010_7b8c(pu_var5, i_var3.field_0x6, unaff_SS);
    if (i_var3.field_0x8e != 0x0) {
        u_var4 = SUB42(ctx.s_tile2_bmp_1050_1538, 0x0);
        DeleteObject16(0x1010);
        i_var3.field_0x8e = 0x0;
    }
    u_var1 = i_var3.field_0x90;
    u_var2 = i_var3.field_0x92;
    paStack10 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1040_a5d0(CONCAT22(u_var2, u_var1));
        u_var4 = 0x1000;
        fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
    }
    ui_cleanup_op_1040_782c(param_1, u_var4);
    return;
}

pub fn invalidate_rect_1040_c028(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: i16,
    param_3: HWND16,
    param_4: &RECT16,
) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let i_var4: i16;
    let iVar5: i16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar9: u16;
    let rect: &mut RECT16;
    let mut rect_2: RECT16;
    let i_stack6: i16;
    let i_stack4: i16;
    let piVar8: U32Ptr;

    iVar7 = param_1;
    // uVar9 = (param_1 >> 0x10);
    if param_2 == 0x8 {
        GetClientRect16(param_3, rect);
        u_var1 = (iVar7 + 0x6);
        u_var3 = (iVar7 + 0x6);
        iVar5 = (u_var3 + 0x16);
        u_var3 = (iVar7 + 0x6);
        rect.x = (u_var3 + 0x1a);
        u_var3 = (iVar7 + 0x6);
        PtInRect16.y = (u_var3 + 0x1c);
        if (iVar5 != 0x0) {
            if (iVar5 < 0x2) {
                i_var4 = 0x1;
            } else {
                i_var4 = 0x2;
            }
            u_var2 = ((iVar5 - i_var4) * 0x4 + u_var1 + 0x2a);
            iVar5 = u_var2;
            u_var2 &= 0xffff0000;
            rect.x = (iVar5 + 0x22) + (u_var2 | iVar5 + 0x1e);
        }
        u_var1 = (iVar7 + 0x6);
        i_stack6 = (u_var1 + 0x1e);
        i_stack4 += -0x5;
    } else {
        if (param_2 != 0x9) {
            if (param_2 != 0xa) {
                return;
            }
            u_var1 = (iVar7 + 0x6);
            u_var6 = u_var1 + 0x2a;
            if (((iVar7 + 0x8) | u_var6) == 0x0) {
                return;
            }
            u_var3 = (iVar7 + 0x6);
            u_var2 = (((u_var3 + 0x16) + -0x1) * 0x4 + u_var6);
            iVar7 = u_var2;
            u_var2 &= 0xffff0000;
            piVar8 = (u_var2 | iVar7 + 0x1e);
            // uVar9 = (u_var2 >> 0x10);
            rect_2.y = (iVar7 + 0x20) + -0x8;
            rect_2.x = *piVar8;
            i_stack6 = (iVar7 + 0x22) + *piVar8;
            i_stack4 = (iVar7 + 0x20);
            param_4 = &rect_2;
            rect = 0x0;
            //       TODO: goto LAB_1040_c19d;
        }
        rect.x = 0x0;
        rect.y = 0x0;
        i_stack6 = 0x0;
        i_stack4 = 0x0;
        GetClientRect16(param_3, &rect_2);
        u_var1 = (iVar7 + 0x6);
        rect_2.x = (u_var1 + 0x1a);
        u_var1 = (iVar7 + 0x6);
        i_stack6 = (u_var1 + 0x1e);
        i_stack4 += -0x5;
        u_var1 = (iVar7 + 0x6);
        u_var3 = (iVar7 + 0x6);
        iVar7 = (u_var3 + 0x16);
        if (0x0 < iVar7) {
            u_var1 = (u_var1 + iVar7 * 0x4 + 0x26);
            iVar7 = u_var1;
            // uVar9 = (u_var1 >> 0x10);
            rect.y = (iVar7 + 0x20) + (iVar7 + 0x24);
        }
    }
    param_3 = ctx.s_tile2_bmp_1050_1538;
    rect = &rect_2;
    //LAB_1040_c19d:
    InvalidateRect16(param_3, rect, param_4);
    return;
}

pub fn unk_draw_op_1040_c226(ctx: &mut AppContext, param_1: u32, param_2: HWND16, param_3: u16) {
    let u_var1: u32;
    let handle: HPEN16;
    let handle_00: HGDIOBJ16;
    let u_var2: u16;
    let local_32: RECT16;
    let iStack46: i16;
    let iStack44: i16;
    let uStack42: u16;
    let iStack40: i16;
    let pRStack38: *mut RECT16;
    let HStack36: HDC16;
    let local_22: PAINTSTRUCT16;

    // u_var2 = (param_1 >> 0x10);
    HStack36 = BeginPaint16(param_2, &local_22);
    pRStack38 = CreateSolidBrush16(ctx.s_tile2_bmp_1050_1538);
    GetClientRect16(ctx.s_tile2_bmp_1050_1538, &local_32);
    u_var1 = (param_1 + 0x6);
    iStack40 = (u_var1 + 0x1a);
    u_var1 = (param_1 + 0x6);
    uStack42 = (u_var1 + 0x1c);
    local_32.y += 0x2;
    local_32.x = iStack40 + -0xa;
    iStack46 += -0x2;
    iStack44 += -0x2;
    FrameRect16(ctx.s_tile2_bmp_1050_1538, pRStack38, &local_32);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    handle = CreatePen16(ctx.s_tile2_bmp_1050_1538, -0x7f80, 0x0);
    handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
    draw_line_1040_c302(ctx, param_1, ctx.s_tile2_bmp_1050_1538);
    draw_op_1040_c38e(ctx, param_1);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_00);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn draw_line_1040_c302(ctx: &mut AppContext, param_1: u32, param_2: HDC16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let iVar5: i16;
    let iVar6: i16;
    let uVar7: u16;

    // uVar7 = (param_1 >> 0x10);
    u_var3 = (param_1 + 0x6);
    iVar6 = (u_var3 + 0x16);
    if (0x1 < iVar6) {
        u_var1 = (param_1 + 0x6);
        u_var4 = u_var1 + 0x2a;
        u_var1 &= 0xffff0000;
        u_var2 = (u_var1 | u_var4);
        iVar5 = u_var2;
        u_var2 &= 0xffff0000;
        // uVar7 = (u_var2 >> 0x10);
        MoveTo16(
            param_2,
            (iVar5 + 0x20) + (iVar5 + 0x24),
            (iVar5 + 0x22) / 0x2 + (u_var2 | iVar5 + 0x1e),
        );
        u_var1 = (u_var4 + iVar6 * 0x4 + -0x4);
        iVar6 = u_var1;
        u_var1 &= 0xffff0000;
        // uVar7 = (u_var1 >> 0x10);
        LineTo16(
            ctx.s_tile2_bmp_1050_1538,
            (iVar6 + 0x20),
            (iVar6 + 0x22) / 0x2 + (u_var1 | iVar6 + 0x1e),
        );
    }
    return;
}

pub fn draw_op_1040_c38e(ctx: &mut AppContext, param_1: u32) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let i_var4: i16;
    let iVar5: i16;
    let pIVar6: i16;
    let in_DX: u16;
    let iVar7: i16;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let hdc: HDC16;
    let unaff_SS: u16;
    let DVar11: u32;
    let iStack26: i16;
    let IStack20: i16;
    let iStack18: i16;
    let IStack16: i16;
    let iStack14: i16;

    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var2 = (i_var8 + 0x6);
    iVar7 = (u_var2 + 0x18);
    if ((iVar7 != 0x0) && (u_var2 = (i_var8 + 0x6), (u_var2 + 0x16) != 0x0)) {
        hdc = 0x1010;
        i_var4 = iVar7;
        pass1_1010_2ee2((i_var8 + 0x6), unaff_SS, 0x1010);
        // TODO: refactor for loop
        // for (iStack26 = 0x0; iStack26 < iVar7; iStack26 += 0x1) {
        //   u_var1 = (iStack26 * 0x4 + i_var4);
        //   iVar5 = u_var1;
        //   u_var1 &= 0xffff0000;
        //   pIVar6 = (u_var1 | iVar5 + 0x1e);
        //   u_var10 = (u_var1 >> 0x10);
        //   iVar5 = (iVar5 + 0x24) / 0x2 + (iVar5 + 0x20);
        //   MoveTo16(hdc,iVar5,*pIVar6);
        //   LineTo16(ctx.s_tile2_bmp_1050_1538,iVar5,*pIVar6 + -0xf);
        //   hdc = ctx.s_tile2_bmp_1050_1538;
        //   DVar11 = GetCurrentPosition16(ctx.s_tile2_bmp_1050_1538);
        //   iStack18 = (DVar11 >> 0x10);
        //   IStack20 = DVar11;
        //   if (iStack26 == 0x0) {
        //     IStack16 = IStack20;
        //     iStack14 = iStack18;
        //   }
        // }
        u_var2 = (i_var8 + 0x6);
        if ((u_var2 + 0x24) != 0x0) {
            iStack14 += -0xd;
        }
        u_var2 = (i_var8 + 0x6);
        if ((u_var2 + 0x26) != 0x0) {
            iStack18 += 0xd;
        }
        u_var2 = (i_var8 + 0x6);
        u_var3 = (i_var8 + 0x6);
        u_var1 = (u_var2 + (u_var3 + 0x16) * 0x4 + 0x26);
        iVar7 = u_var1;
        u_var1 &= 0xffff0000;
        // uVar9 = (u_var1 >> 0x10);
        MoveTo16(
            hdc,
            (iVar7 + 0x24) / 0x2 + (iVar7 + 0x20),
            (iVar7 + 0x22) + (u_var1 | iVar7 + 0x1e),
        );
        LineTo16(
            ctx.s_tile2_bmp_1050_1538,
            (iVar7 + 0x24) / 0x2 + (iVar7 + 0x20),
            IStack16,
        );
        DVar11 = GetCurrentPosition16(ctx.s_tile2_bmp_1050_1538);
        // iVar7 = (DVar11 >> 0x10);
        if (iVar7 < iStack14) {
            iStack14 = iVar7;
        }
        if (iStack18 < iVar7) {
            iStack18 = iVar7;
        }
        MoveTo16(ctx.s_tile2_bmp_1050_1538, iStack14, IStack16);
        LineTo16(ctx.s_tile2_bmp_1050_1538, iStack18, IStack20);
    }
    return;
}

pub fn draw_op_1040_c74c(ctx: &mut AppContext, param_1: U32Ptr, param_2: u32, param_3: u16) {
    let u_var1: u16;
    let ppcVar2: u32;
    let u_var3: u32;
    let HVar4: HGDIOBJ16;
    let iVar5: i16;
    let u_var6: u16;
    let HStack16: HGDIOBJ16;
    let HStack14: HGDIOBJ16;
    let HStack12: HPEN16;
    let HStack10: HGDIOBJ16;
    let HStack8: HPALETTE16;
    let uStack6: u16;

    u_var6 = (ctx.PTR_LOOP_1050_4230 >> 0x10);
    uStack6 = (ctx.PTR_LOOP_1050_4230 + 0xe);
    u_var1 = (ctx.PTR_LOOP_1050_4230 + 0x10);
    HStack8 = palette_op_1008_4e08(CONCAT22(u_var1, uStack6), &param_2 + 0x2, u_var1, 0x1008);
    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1;
    (iVar5 + 0x46) = 0x1;
    HStack10 = GetStockObject16(0x1008);
    HStack12 = CreatePen16(ctx.s_tile2_bmp_1050_1538, 0x2, 0x100);
    HStack14 = SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack10);
    HStack16 = SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack12);
    Rectangle16(
        ctx.s_tile2_bmp_1050_1538,
        (iVar5 + 0x24),
        (iVar5 + 0x22),
        0x0,
        0x0,
    );
    MoveTo16(
        ctx.s_tile2_bmp_1050_1538,
        0x0,
        (iVar5 + 0x36) * 0x2 + (iVar5 + 0x2a),
    );
    LineTo16(
        ctx.s_tile2_bmp_1050_1538,
        (iVar5 + 0x24),
        (iVar5 + 0x36) * 0x2 + (iVar5 + 0x2a),
    );
    SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack14);
    HVar4 = SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack16);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    u_var3 = *param_1;
    ppcVar2 = (u_var3 + 0x10);
    (**ppcVar2)(
        ctx.s_tile2_bmp_1050_1538,
        param_1,
        param_2,
        HVar4,
        param_2._2_2_,
    );
    ppcVar2 = (u_var3 + 0x14);
    (**ppcVar2)(ctx.s_tile2_bmp_1050_1538, param_1, param_2._2_2_);
    (iVar5 + 0x46) = 0x0;
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, HStack8);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    return;
}

pub fn palette_op_1040_c886(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u8,
    param_3: u16,
    param_4: HDC16,
) {
    let u_var1: u16;
    let ppcVar2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let uStack12: u16;
    let puStack8: u32;
    let HStack4: HPALETTE16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (((i_var3 + 0xa) | (i_var3 + 0x8)) != 0x0) {
        HStack4 = 0x0;
        if ((i_var3 + 0x46) == 0x0) {
            u_var5 = (ctx.PTR_LOOP_1050_4230 >> 0x10);
            uStack12 = (ctx.PTR_LOOP_1050_4230 + 0xe);
            u_var1 = (ctx.PTR_LOOP_1050_4230 + 0x10);
            param_4 = 0x1008;
            HStack4 = palette_op_1008_4e08(CONCAT22(u_var1, uStack12), &param_3, u_var1, 0x1008);
        }
        puStack8 = (i_var3 + 0x8);
        u_var5 = (i_var3 + 0xa);
        if ((((i_var3 + 0xe) | (i_var3 + 0xc)) != 0x0) && ((param_2 & 0x1) != 0x0)) {
            puStack8 = (i_var3 + 0xc);
            u_var5 = (i_var3 + 0xe);
        }
        if (((i_var3 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0)) {
            puStack8 = (i_var3 + 0x10);
            u_var5 = (i_var3 + 0x12);
        }
        ppcVar2 = (*puStack8 + 0x4);
        (**ppcVar2)(
            param_4,
            puStack8,
            u_var5,
            (i_var3 + 0x28),
            (i_var3 + 0x26),
            &param_3,
        );
        if ((i_var3 + 0x46) == 0x0) {
            SelectPalette16(param_4, 0x0, HStack4);
            DeleteObject16(ctx.s_tile2_bmp_1050_1538);
        }
    }
    return;
}
