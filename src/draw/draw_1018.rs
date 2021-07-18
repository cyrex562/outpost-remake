use crate::{cleanup::unk_destroy_window_op_1018_6bb6, global::AppContext, mixed::mixed_1010_20ba, pass::{pass_1008::pass1_1008_3e54, pass_1018::pass1_1018_659a}, ui::{ui_1008::{win_1008_5c5c, win_1008_5c9e}, ui_1018::mix_ui_op_1018_6adc}, util::{CONCAT22, SUB42}, win_struct::{COLORREF, HDC16, HWND16, PAINTSTRUCT16, RECT16}, winapi::{BeginPaint16, CreateSolidBrush16, DeleteObject16, DrawText16, EndPaint16, FillRect16, InvalidateRect16, PostMessage16, SelectPalette16, SetBkColor16, SetTextColor16}};

pub unsafe fn pt_in_rect_1018_1bda(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let i_var3: i16;
    let bvar4: bool;
    let i_var5: i16;
    let u_var6: u16;
    let i_stack26: i16;
    let p_stack24: i16;
    let local_14: i16;
    let local_12: i16;
    let u_stack16: u16;
    let u_stack14: u32;
    let local_a: i16;
    let local_8: i16;
    let i_stack6: i16;
    let i_stack4: i16;

    u_stack14 = 0x0;
    i_var3 = param_1;
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (i_var3 + 0x3a)),
        CONCAT22(param_4, &local_14),
        CONCAT22(param_4, &local_12),
    );
    p_stack24 = CONCAT22(param_2, param_3);
    u_stack16 = 0x0;
    i_stack26 = 0x0;
    loop {
        // uVar6 = (param_1 >> 0x10);
        pi_var1 = (i_var3 + 0x44);
        if (*pi_var1 == i_stack26 || *pi_var1 < i_stack26) {
            return;
        }
        u_var2 = (i_var3 + 0x42);
        i_var5 = (i_var3 + 0x40) + i_stack26 * 0x18;
        u_stack14 = CONCAT22(u_var2, i_var5);
        pass1_1008_3e94(
            CONCAT22(u_var2, i_var5),
            CONCAT22(param_4, &local_8),
            CONCAT22(param_4, &local_a),
        );
        local_a += local_12 + -0x6;
        i_stack6 = local_a + 0xc;
        local_8 += local_14 + -0x6;
        i_stack4 = local_8 + 0xc;
        BVar4 = PtInRect16(0x1008, p_stack24);
        if (BVar4 != 0x0) {
            break;
        }
        i_stack26 += 0x1;
    }
    pass1_1018_1eda(param_1, u_stack14, param_4);
    return;
}

pub fn get_dc_1018_4db0(param_1: i32, param_2: u16, param_3: HWND16) {
    let HVar1: HDC16;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_2;
    HVar1 = GetDC16(param_3);
    *(param_1 + 0x14) = HVar1;
    return;
}

pub unsafe fn create_dc_1018_4e04(
    param_1: *mut *mut astruct_8,
    param_2: u16,
    param_3: i16,
    param_4: i16,
    in_string_5: &String,
    in_string_6: u16,
) {
    let ppc_var1: u32;
    let pa_var2: &mut Struct8;
    let i_var4: &mut Struct9;
    let u_var3: u16;
    let u_var4: u32;
    let i_stack16: i16;

    // uVar3 = (param_1 >> 0x10);
    i_var4 = param_1;
    ppc_var1 = (*param_1 + 0x14);
    (**ppc_var1)();
    u_var4 = pass1_1008_4772(i_var4.field_0xa);
    pass1_1008_43cc(i_var4.field_0xa);
    pa_var2 = CreateDC16(0x1008, u_var4, (u_var4 >> 0x10), 0x0);
    i_var4.field_0x12 = pa_var2;
    pa_var2 = &i_var4.field_0x12;
    ppc_var1 = (*i_var4.field_0xa + 0x8);
    (**ppc_var1)();
    i_var4.field_0x1a = pa_var2;
    if ((ctx.DAT_1050_422e != 0x0) && (0x280 < param_4)) {
        // TODO: refactor for loop
        // for (iStack16 = 0x0; iStack16 < ctx.DAT_1050_4216 + 0x1; iStack16 += 0x1) {
        //   (&ctx.PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) =
        //        (((&ctx.PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) *
        //              (param_4 + 0x1)) / 0x280);
        // }
        // TODO: refactor for loop
        // for (iStack16 = 0x0; iStack16 < ctx.DAT_1050_422c + 0x1; iStack16 += 0x1) {
        //   (&DAT_1050_419a + iStack16 * 0x2) =
        //        (((&DAT_1050_419a + iStack16 * 0x2) *
        //              (param_3 + 0x1)) / 0x1e0);
        // }
    }
    ctx.DAT_1050_422e = 0x0;
    return;
}

pub unsafe fn invalidate_rect_1018_58e2(
    ctx: &mut AppContext,
    param_1: &mut Struct58,
    param_2: i16,
    win_handle: HWND16,
) {
    let pi_var1: U32Ptr;
    let i_var2: &mut Struct58;
    let u_var2: u16;

    if (param_2 == 0x105) {
        i_var2 = param_1;
        pi_var1 = &i_var2.field_0xf6;
        *pi_var1 = *pi_var1 + 0x1;
        if (ctx.PTR_DAT_1050_0004_1050_4240 <= i_var2.field_0xf6) {
            PostMessage16(win_handle, 0x0, 0x0, 0x11100ca);
            return;
        }
        i_var2.field_0xea = 0x0;
        InvalidateRect16(win_handle, 0x0, 0x0);
    }
    return;
}

pub fn invalidate_rect_1018_5d32(param_1: u32, param_2: i16, param_3: HWND16) {
    if (param_2 == 0x1) {
        (param_1 + 0x14) = 0x0;
        return;
    }
    if (param_2 != 0x2) {
        return;
    }
    InvalidateRect16(param_3, 0x0, param_1 + 0x22);
    return;
}

pub fn misc_draw_op_1018_5d6c(param_1: u32, param_2: HWND16) {
    let puVar1: u32;
    let ppcVar2: u32;
    let uVar3: u32;
    let iVar4: i16;
    let uVar5: u16;
    let unaff_SS: u16;
    let paVar6: &mut Struct76;
    let uVar7: u16;
    let local_22: PAINTSTRUCT16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar7 = (iVar4 + 0x4);
    BeginPaint16(param_2, &local_22);
    uVar3 = (iVar4 + 0x14);
    puVar1 = (uVar3 + 0xa);
    paVar6 = pass1_1008_9f48((iVar4 + 0x14));
    pass1_1008_5236((iVar4 + 0x18));
    pass1_1008_4480(
        puVar1,
        (param_1 & 0xffff0000 | (iVar4 + 0x1c)),
        paVar6,
        unaff_SS,
    );
    ppcVar2 = (*puVar1 + 0x4);
    (**ppcVar2)(
        0x1008,
        puVar1,
        (puVar1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | (iVar4 + 0xa),
        uVar7,
    );
    EndPaint16(0x1008, &local_22);
    return;
}

pub unsafe fn unk_draw_op_1018_623e(param_1: u32, param_2: HWND16, param_3: u16) {
    let piVar1: U32Ptr;
    let ppcVar2: u32;
    let uVar3: u32;
    let puVar4: u32;
    let uVar5: u16;
    let pHVar6: HDC16;
    let iVar7: i16;
    let handle: HPEN16;
    let HVar8: HGDIOBJ16;
    let handle_00: HBRUSH16;
    let puVar9: U32Ptr;
    let uVar10: u16;
    let iVar11: i16;
    let iVar12: i16;
    let puVar13: U32Ptr;
    let uVar14: u16;
    let uVar15: u16;
    let style: i16;
    let uVar16: u32;
    let uVar17: u8;
    let uVar18: u8;
    let iVar19: i16;
    let uVar20: u8;
    let uVar21: u8;
    let local_38: [u8; 6];
    let local_32: u16;
    let uStack48: u16;
    let uStack46: u32;
    let uStack42: u16;
    let puStack40: u32;
    let local_24: HDC16;
    let local_22: PAINTSTRUCT16;

    puVar13 = &stack0xfffe;
    // uVar14 = (param_1 >> 0x10);
    iVar11 = param_1;
    uVar15 = (iVar11 + 0x4);
    local_24 = BeginPaint16(param_2, &local_22);
    puStack40 = pass1_1010_4c2c((iVar11 + 0x6));
    pHVar6 = &local_24;
    ppcVar2 = (*puStack40 + 0x8);
    (**ppcVar2)(
        0x1010,
        puStack40,
        (puStack40 >> 0x10),
        pHVar6,
        param_3,
        uVar15,
    );
    (iVar11 + 0x10) = pHVar6;
    uVar3 = (iVar11 + 0x6);
    uStack42 = (uVar3 + 0x30);
    uVar3 = (iVar11 + 0x6);
    uStack46 = (uVar3 + 0x12);
    uStack48 = 0x14;
    local_32 = 0x0;
    style = 0x1008;
    pass1_1008_3e38(CONCAT22(param_3, local_38));
    while ((puVar13 + -0x38) < (puVar13 + -0x28)) {
        iVar11 = (puVar13 + -0x38) * 0x4;
        uVar3 = (puVar13 + -0x2c);
        uVar16 = pass1_1008_4772((iVar11 + uVar3));
        // puVar9 = (uVar16 >> 0x10);
        (puVar13 + -0x44) = uVar16;
        (puVar13 + -0x42) = puVar9;
        uVar3 = (puVar13 + 0x6);
        pass1_1018_642e(
            uVar3,
            (uVar3 >> 0x10),
            CONCAT13((param_3 >> 0x8), CONCAT12(param_3, puVar13 + -0x30)),
            (uVar16 + 0x8),
        );
        uVar3 = (puVar13 + -0x30);
        pass1_1008_3e76(
            CONCAT22(param_3, puVar13 + -0x36),
            0x0,
            uVar3,
            (uVar3 >> 0x10),
        );
        uVar3 = (puVar13 + -0x2c);
        pass1_1008_4480(
            (puVar13 + -0x26),
            CONCAT22(param_3, puVar13 + -0x36),
            (uVar3 + iVar11),
            param_3,
        );
        iVar11 = (puVar13 + -0x38);
        uVar3 = (puVar13 + -0x30);
        uVar14 = uVar3;
        // uVar20 = (uVar3 >> 0x10);
        uVar21 = (uVar3 >> 0x18);
        uVar3 = (puVar13 + -0x44);
        // uVar15 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        iVar7 = (iVar12 + 0x4) + (puVar13 + -0x2e);
        iVar12 = (iVar12 + 0x8) + (puVar13 + -0x30);
        uVar3 = (puVar13 + 0x6);
        uVar3 = (uVar3 + 0x6);
        iVar19 = uVar3;
        // uVar15 = (uVar3 >> 0x10);
        uVar17 = 0x8;
        uVar18 = 0x10;
        if ((iVar19 + 0x1a) == 0x0) {
            uVar5 = (iVar19 + 0x30) << 0x3;
            mem_op_1000_179c(uVar5, puVar9, 0x1000);
            (iVar19 + 0x1a) = uVar5;
            (iVar19 + 0x1c) = puVar9;
        }
        uVar3 = (iVar19 + 0x1a);
        iVar11 *= 0x8;
        (uVar3 + iVar11) = CONCAT11(uVar21, uVar20);
        uVar3 = (iVar19 + 0x1a);
        (uVar3 + iVar11 + 0x2) = uVar14;
        uVar3 = (iVar19 + 0x1a);
        (uVar3 + iVar11 + 0x4) = iVar7;
        uVar3 = (iVar19 + 0x1a);
        (uVar3 + iVar11 + 0x6) = iVar12;
        style = CONCAT11(uVar18, uVar17);
        uVar3 = (puVar13 + -0x44);
        piVar1 = (puVar13 + -0x2e);
        *piVar1 = *piVar1 + (-((puVar13 + -0x38) == 0x0) & 0x5) + 0x14 + (uVar3 + 0x4);
        piVar1 = (puVar13 + -0x38);
        *piVar1 = *piVar1 + 0x1;
    }
    puVar4 = (puVar13 + -0x26);
    ppcVar2 = (*puVar4 + 0x4);
    (**ppcVar2)(
        style,
        puVar4,
        (puVar4 >> 0x10),
        0x0,
        0x0,
        puVar13 + -0x22,
        param_3,
    );
    handle = CreatePen16(style, 0x25, 0x100);
    (puVar13 + -0x3a) = handle;
    HVar8 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
    (puVar13 + -0x3c) = HVar8;
    handle_00 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    (puVar13 + -0x3e) = handle_00;
    HVar8 = SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_00);
    (puVar13 + -0x40) = HVar8;
    draw_line_1018_6444((puVar13 + 0x6), s_tile2_bmp_1050_1538);
    uVar3 = (puVar13 + 0x6);
    uVar16 = pass1_1010_4dc8((uVar3 + 0x6));
    // uVar10 = (uVar16 >> 0x10);
    uVar5 = uVar16;
    draw_op_1018_6544((puVar13 + 0x6), (uVar16 & 0xffff | uVar10 << 0x10), param_3);
    pass1_1018_6630((puVar13 + 0x6), uVar5, uVar10);
    uVar3 = (puVar13 + 0x6);
    SelectPalette16(0x1010, 0x0, (uVar3 + 0x10));
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, (puVar13 + -0x3c));
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, (puVar13 + -0x40));
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(s_tile2_bmp_1050_1538, (puVar13 + -0x20));
    return;
}

pub fn draw_line_1018_6444(param_1: u32, param_2: HDC16) {
    let iVar1: i16;
    INT16 * pIVar2;
    let uVar3: u32;
    let iVar4: i16;
    let iVar5: i16;
    let piVar6: U32Ptr;
    let uVar7: u16;
    let uStack10: i16;

    // uVar7 = (param_1 >> 0x10);
    uVar3 = (param_1 + 0x6);
    iVar1 = (uVar3 + 0x30);
    uVar3 = (param_1 + 0x6);
    pIVar2 = (uVar3 + 0x1a);
    MoveTo16(param_2, 0x5, *pIVar2);
    // uVar7 = (pIVar2 >> 0x10);
    iVar5 = pIVar2;
    LineTo16(ctx.s_tile2_bmp_1050_1538, 0x5, (iVar5 + iVar1 * 0x8 + -0x4));
    // TODO: refactor
    // for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    //   piVar6 = (iStack10 * 0x8 + iVar5);
    //   iVar4 = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
    //   MoveTo16(ctx.s_tile2_bmp_1050_1538,0x5,iVar4);
    //   LineTo16(ctx.s_tile2_bmp_1050_1538,0xa,iVar4);
    // }
    MoveTo16(ctx.s_tile2_bmp_1050_1538, 0x5f, *pIVar2);
    LineTo16(
        ctx.s_tile2_bmp_1050_1538,
        0x5f,
        (iVar5 + iVar1 * 0x8 + -0x4),
    );
    // TODO: refactor
    // for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    //   piVar6 = (iStack10 * 0x8 + iVar5);
    //   iVar4 = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
    //   MoveTo16(ctx.s_tile2_bmp_1050_1538,0x5f,iVar4);
    //   LineTo16(ctx.s_tile2_bmp_1050_1538,0x5a,iVar4);
    // }
    return;
}

pub fn draw_op_1018_6544(param_1: u32, param_2: &mut i16, param_3: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let local_a: [u8; 6];
    let u_stack4: u16;

    if (param_2 != 0x0) {
        u_stack4 = ((param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
        pu_var1 = pass1_1008_3e54(CONCAT22(param_3, local_a), 0x0, 0x57, u_stack4);
        u_var2 = pass1_1018_659a(
            param_1,
            u_var3,
            CONCAT22(param_3, local_a),
            (pu_var1 >> 0x10),
            param_3,
        );
        draw_polygon_1018_661c(param_1, u_var3, CONCAT22(u_var2, 0x3), 0x1008);
    }
    return;
}

pub fn draw_polygon_1018_661c(param_1: u16, param_2: u16, param_3: u32, param_4: HDC16) {
    Polygon16(param_4, param_3, (param_3 >> 0x10));
    return;
}

pub unsafe fn mixed_draw_op_1018_6a7a(
    ctx: &mut AppContext,
    param_1: &mut Struct28,
    win_handle: HWND16,
    in_dx: u16,
    unaff_di: u16,
    unaff_ss: u16,
) {
    // let in_DX: U32Ptr;
    // let unaff_DI: i16;
    // let unaff_SS: u16;
    let pu_var1: U32Ptr;
    let local_22: PAINTSTRUCT16;

    BeginPaint16(win_handle, &local_22);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    pu_var1 = mixed_1010_20ba(ctx, ctx.PTR__LOOP_1050_0ed0, 0x2, unaff_ss, in_dx, unaff_di);
    if ((pu_var1 + 0x20) == 0x0) {
        unk_destroy_window_op_1018_6bb6(ctx, param_1, 0x1010);
        return;
    }
    mix_ui_op_1018_6adc(param_1);
    return;
}

pub unsafe fn unk_draw_op_1018_c578(
    ctx: &mut AppContext,
    param_1: &mut Struct36,
    param_2: u16,
    in_dx: u16,
    extraout_dx: u16,
    unaff_di: u16,
) {
    let u_var1: u16;
    let i_var2: i16;
    let i_var3: i16;
    let pa_var4: &mut Struct76;
    let ppc_var5: u32;
    let u_var6: u32;
    let u_var7: u16;
    let b_force_background: HDC16;
    let i_var8: i16;
    // let in_d_x: U32Ptr;
    let u_var9: u16;
    let u_var10: u16;
    // let extraout_d_x: u16;
    let i_var11: i16;
    let u_var12: u16;
    // let unaff_d_i: i16;
    let u_var13: u16;
    let u_var14: u16;
    let hwnd: HWND16;
    let u_var15: u32;
    let p_h_var16: HDC16;
    let p_r_var18: *mut RECT16;
    let u_var17: u32;
    let h_var19: HDC16;
    let local_34: u32;
    let i_stack48: i16;
    let i_stack46: i16;
    let p_r_stack44: *mut RECT16;
    let local_2a: HDC16;
    let u_stack40: u16;
    let pu_stack38: U32Ptr;
    let local_22: PAINTSTRUCT16;

    hwnd = 0x1010;
    pu_stack38 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_2, in_dx, unaff_di);
    // uVar9 = (puStack38 >> 0x10);
    u_var7 = (pu_stack38 + 0x20);
    i_var11 = param_1;
    // uVar13 = (param_1 >> 0x10);
    u_stack40 = u_var7;
    if (u_var7 == 0x0) {
        BeginPaint16(0x1010, &local_22);
        EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
        PostMessage16(
            ctx.s_tile2_bmp_1050_1538,
            0x0,
            0x0,
            CONCAT22(0x111, (i_var11 + 0xea)),
        );
        return;
    }
    if (((i_var11 + 0xf0) == 0x0) && ((i_var11 + 0xf4) != 0x0)) {
        (i_var11 + 0xf0) = 0x1;
        u_var1 = i_var11 + 0xf2;
        hwnd = 0x1008;
        win_1008_5c9e(
            ctx.PTR__LOOP_1050_02a0,
            (param_1 & 0xffff0000 | u_var1),
            u_var1,
            u_var9,
            param_2,
        );
        u_var7 = u_var1;
    }
    if ((ctx.PTR__LOOP_1050_02a0 + 0x12) == 0x0) {
        hwnd = 0x1008;
        win_1008_5c5c(param_2, u_var7, u_var9, ctx._PTR_LOOP_1050_02a0, 0x1d3);
    }
    local_2a = BeginPaint16(hwnd, &local_22);
    p_r_stack44 = CreateSolidBrush16(ctx.s_tile2_bmp_1050_1538);
    local_34 = 0x0;
    i_stack48 = (i_var11 + 0xf6) + -0x1;
    i_stack46 = (i_var11 + 0xf8) + -0x1;
    u_var7 = param_2;
    h_var19 = local_2a;
    FillRect16(ctx.s_tile2_bmp_1050_1538, p_r_stack44, &local_34);
    p_r_var18 = p_r_stack44;
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    u_var6 = (i_var11 + 0xe2);
    pa_var4 = (u_var6 + 0xe);
    b_force_background = &local_2a;
    u_var17 = CONCAT22(p_r_var18, param_2);
    // uVar14 = (paVar4 >> 0x10);
    u_var12 = SUB42(pa_var4, 0x0);
    u_var6 = pa_var4;
    ppc_var5 = (u_var6 + 0x8);
    p_h_var16 = b_force_background;
    (**ppc_var5)();
    u_var15 = pass1_1008_4772(pa_var4);
    // uVar10 = (uVar15 >> 0x10);
    i_var2 = (u_var15 + 0x4);
    i_var3 = (u_var15 + 0x8);
    i_var8 = (0x1e0 - i_var3) / 0x2;
    (i_var11 + 0x10c) = i_var8 + i_var3 + (i_var11 + 0x110);
    ppc_var5 = (u_var6 + 0x4);
    (**ppc_var5)(
        0x1008,
        pa_var4,
        (i_var11 + 0xfc) + (i_var11 + 0xfe) + i_var8,
        (i_var11 + 0xfa) + (0x280 - i_var2) / 0x2,
        &local_2a,
        param_2,
        u_var12,
        u_var14,
        p_h_var16,
        u_var17,
        u_var7,
        h_var19,
    );
    draw_text_1018_c742(param_1, 0x1008, param_2, extraout_dx);
    SelectPalette16(0x1008, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub unsafe fn draw_text_1018_c742(
    ctx: &mut AppContext,
    param_1: &mut Struct36,
    param_2: HDC16,
    param_3: &RECT16,
    param_4: u16,
) {
    let pi_var1: U32Ptr;
    let c_var2: COLORREF;
    let i_var3: i16;
    let i_var4: &mut Struct36;
    let u_var4: &mut Struct36;
    let local_1a: u16;
    let u_stack24: u16;
    let i_stack22: i16;
    let i_stack20: i16;
    let local_12: i16;
    let i_stack16: i16;
    let i_stack14: i16;
    let i_stack12: i16;
    let c_stack10: COLORREF;
    let u_stack8: u16;
    let u_stack6: u32;

    // uVar4 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4.field_0x108 != 0x0) && (*i_var4.field_0x108 != '\0')) {
        c_var2 = SetTextColor16(param_2, 0x8000);
        u_stack6 = CONCAT22(param_4, c_var2);
        c_stack10 = SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
        u_stack8 = param_4;
        if (i_var4.field_0x106 == 0x0) {
            local_1a = 0x0;
            u_stack24 = 0x0;
            i_stack22 = i_var4.field_0x10e;
            i_stack20 = 0x0;
            DrawText16(ctx.s_tile2_bmp_1050_1538, 0x410, &local_1a, param_3, 0xffff);
            i_var4.field_0x100 = (0x280 - i_stack22) / 0x2;
            i_var4.field_0x102 = i_var4.field_0x10c;
            i_var4.field_0x104 = i_var4.field_0x100 + i_stack22;
            i_var3 = i_var4.field_0x102 + i_stack20;
            i_var4.field_0x106 = i_var3;
            pi_var1 = &i_var4.field_0xf8;
            if (*pi_var1 == i_var3 || *pi_var1 < i_var3) {
                i_var3 -= i_var4.field_0xf8;
                pi_var1 = &i_var4.field_0x102;
                *pi_var1 = *pi_var1 - i_var3;
                pi_var1 = &i_var4.field_0x106;
                *pi_var1 = *pi_var1 - i_var3;
            }
        }
        local_12 = i_var4.field_0xfa + i_var4.field_0x100;
        i_stack16 = i_var4.field_0xfc + i_var4.field_0x102;
        i_stack14 = i_var4.field_0xfa + i_var4.field_0x104;
        i_stack12 = i_var4.field_0xfc + i_var4.field_0x106;
        DrawText16(
            ctx.s_tile2_bmp_1050_1538,
            &ctx.PTR_LOOP_1050_0010,
            &local_12,
            param_3,
            0xffff,
        );
        SetTextColor16(ctx.s_tile2_bmp_1050_1538, u_stack6);
        SetBkColor16(ctx.s_tile2_bmp_1050_1538, c_stack10);
    }
    return;
}

pub unsafe fn unk_draw_op_1018_cda8(param_1: &mut Struct36, param_2: u16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let pa_var3: &mut Struct76;
    let ppc_var4: u32;
    let u_var5: u16;
    let b_force_background: HDC16;
    let i_var6: i16;
    let i_var7: i16;
    let in_dx: U32Ptr;
    let u_var8: u16;
    let uVar9: u16;
    let extraout_DX: u16;
    let iVar10: i16;
    let unaff_DI: i16;
    let uVar11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let hwnd: HWND16;
    let uVar14: u32;
    let uVar15: u16;
    let uVar16: u16;
    let pHVar17: HDC16;
    let pRVar19: *mut RECT16;
    let uVar18: u32;
    let local_34: u32;
    let iStack48: i16;
    let iStack46: i16;
    let pRStack44: *mut RECT16;
    let local_2a: HDC16;
    let uStack40: u16;
    let puStack38: U32Ptr;
    let local_22: PAINTSTRUCT16;

    hwnd = 0x1010;
    puStack38 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_2, in_dx, unaff_DI);
    // uVar8 = (puStack38 >> 0x10);
    u_var5 = (puStack38 + 0x20);
    iVar10 = param_1;
    // uVar11 = (param_1 >> 0x10);
    uStack40 = u_var5;
    if (u_var5 == 0x0) {
        BeginPaint16(0x1010, &local_22);
        EndPaint16(s_tile2_bmp_1050_1538, &local_22);
        PostMessage16(
            s_tile2_bmp_1050_1538,
            0x0,
            0x0,
            CONCAT22(0x111, (iVar10 + 0xea)),
        );
        return;
    }
    if ((iVar10 + 0xf0) == 0x0) {
        (iVar10 + 0xf0) = 0x1;
        hwnd = 0x1008;
        win_1008_5c5c(param_2, u_var5, u_var8, _PTR_LOOP_1050_02a0, 0x1f3);
        uVar12 = (ctx.PTR__LOOP_1050_02a0 >> 0x10);
        if ((ctx.PTR__LOOP_1050_02a0 + 0x12) == 0x0) {
            hwnd = 0x1008;
            win_1008_5c5c(
                param_2,
                u_var5,
                u_var8,
                ctx._PTR_LOOP_1050_02a0 & 0xffff | uVar12 << 0x10,
                0x1d3,
            );
        }
    }
    local_2a = BeginPaint16(hwnd, &local_22);
    pRStack44 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    local_34 = 0x0;
    iStack48 = (iVar10 + 0xf6) + -0x1;
    iStack46 = (iVar10 + 0xf8) + -0x1;
    FillRect16(ctx.s_tile2_bmp_1050_1538, pRStack44, &local_34);
    pRVar19 = pRStack44;
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    uVar18 = (iVar10 + 0xe2);
    pa_var3 = (uVar18 + 0xe);
    b_force_background = &local_2a;
    uVar18 = CONCAT22(pRVar19, param_2);
    // uVar13 = (paVar3 >> 0x10);
    ppc_var4 = (pa_var3 + 0x8);
    uVar15 = pa_var3;
    uVar16 = uVar13;
    pHVar17 = b_force_background;
    (**ppc_var4)();
    uVar14 = pass1_1008_4772(pa_var3);
    // uVar9 = (uVar14 >> 0x10);
    i_var6 = (0x280 - (uVar14 + 0x4)) / 0x2;
    i_var2 = (uVar14 + 0x8);
    i_var7 = (0x1e0 - i_var2) / 0x2;
    (iVar10 + 0x10c) = i_var7 + i_var2 + (iVar10 + 0x110);
    if (((iVar10 + 0xfa) == 0x0) && (i_var6 == 0x0)) {
        pi_var1 = (iVar10 + 0xfa);
        *pi_var1 = *pi_var1 + 0x2;
    }
    ppc_var4 = (pa_var3 + 0x4);
    (**ppc_var4)(
        0x1008,
        pa_var3,
        uVar13,
        (iVar10 + 0xfc) + (iVar10 + 0xfe) + i_var7,
        (iVar10 + 0xfa) + i_var6,
        &local_2a,
        param_2,
        uVar15,
        uVar16,
        pHVar17,
        uVar18,
    );
    draw_text_1018_c742(param_1, 0x1008, param_2, extraout_DX);
    SelectPalette16(0x1008, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn unk_draw_op_1018_cfc0(param_1: &mut Struct36, param_2: u16) {
    let piVar1: U32Ptr;
    let iVar2: i16;
    let paVar3: &mut Struct76;
    let ppcVar4: u32;
    let uVar5: u16;
    let b_force_background: HDC16;
    let iVar6: i16;
    let iVar7: i16;
    let in_DX: U32Ptr;
    let uVar8: u16;
    let uVar9: u16;
    let extraout_DX: u16;
    let iVar10: i16;
    let unaff_DI: i16;
    let uVar11: u16;
    let uVar12: u16;
    let hwnd: HWND16;
    let uVar13: u32;
    let uVar14: u16;
    let uVar15: u16;
    let pHVar16: HDC16;
    let pRVar18: *mut RECT16;
    let uVar17: u32;
    let HVar19: HDC16;
    let local_34: u32;
    let iStack48: i16;
    let iStack46: i16;
    let pRStack44: *mut RECT16;
    let local_2a: HDC16;
    let iStack40: i16;
    let puStack38: U32Ptr;
    let local_22: PAINTSTRUCT16;

    hwnd = 0x1010;
    puStack38 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    // uVar8 = (puStack38 >> 0x10);
    iStack40 = (puStack38 + 0x20);
    iVar10 = param_1;
    // uVar11 = (param_1 >> 0x10);
    if (iStack40 == 0x0) {
        BeginPaint16(0x1010, &local_22);
        EndPaint16(s_tile2_bmp_1050_1538, &local_22);
        PostMessage16(
            s_tile2_bmp_1050_1538,
            0x0,
            0x0,
            CONCAT22(0x111, (iVar10 + 0xea)),
        );
        return;
    }
    if (((iVar10 + 0xf0) == 0x0) && ((iVar10 + 0xf4) != 0x0)) {
        (iVar10 + 0xf0) = 0x1;
        uVar5 = iVar10 + 0xf2;
        hwnd = 0x1008;
        win_1008_5c9e(
            ctx.PTR__LOOP_1050_02a0,
            (param_1 & 0xffff0000 | uVar5),
            uVar5,
            uVar8,
            param_2,
        );
        if ((ctx.PTR__LOOP_1050_02a0 + 0x12) == 0x0) {
            hwnd = 0x1008;
            win_1008_5c5c(param_2, uVar5, uVar8, _PTR_LOOP_1050_02a0, 0x1d3);
        }
    }
    local_2a = BeginPaint16(hwnd, &local_22);
    pRStack44 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    local_34 = 0x0;
    iStack48 = (iVar10 + 0xf6) + -0x1;
    iStack46 = (iVar10 + 0xf8) + -0x1;
    uVar8 = param_2;
    HVar19 = local_2a;
    FillRect16(ctx.s_tile2_bmp_1050_1538, pRStack44, &local_34);
    pRVar18 = pRStack44;
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    uVar17 = (iVar10 + 0xe2);
    paVar3 = (uVar17 + 0xe);
    b_force_background = &local_2a;
    uVar17 = CONCAT22(pRVar18, param_2);
    // uVar12 = (paVar3 >> 0x10);
    ppcVar4 = (paVar3 + 0x8);
    uVar14 = paVar3;
    uVar15 = uVar12;
    pHVar16 = b_force_background;
    (**ppcVar4)();
    uVar13 = pass1_1008_4772(paVar3);
    // uVar9 = (uVar13 >> 0x10);
    iVar6 = (0x280 - (uVar13 + 0x4)) / 0x2;
    iVar2 = (uVar13 + 0x8);
    iVar7 = (0x1e0 - iVar2) / 0x2;
    (iVar10 + 0x10c) = iVar7 + iVar2 + (iVar10 + 0x110);
    if (((iVar10 + 0xfa) == 0x0) && (iVar6 == 0x0)) {
        piVar1 = (iVar10 + 0xfa);
        *piVar1 = *piVar1 + 0x2;
    }
    ppcVar4 = (paVar3 + 0x4);
    (**ppcVar4)(
        0x1008,
        paVar3,
        uVar12,
        (iVar10 + 0xfc) + (iVar10 + 0xfe) + iVar7,
        (iVar10 + 0xfa) + iVar6,
        &local_2a,
        param_2,
        uVar14,
        uVar15,
        pHVar16,
        uVar17,
        uVar8,
        HVar19,
    );
    draw_text_1018_c742(param_1, 0x1008, param_2, extraout_DX);
    SelectPalette16(0x1008, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn invalidate_rect_1018_edd8(param_1: u32, param_2: i16, param_3: u16) {
    let iVar1: i16;
    let uVar2: u16;
    let uVar3: u32;
    let local_16: i16;
    let iStack20: i16;
    let iStack18: i16;
    let iStack16: i16;
    let uStack14: u32;
    let uStack10: u16;
    let uStack8: u16;
    let local_6: i16;
    let local_4: i16;

    iVar1 = param_1;
    // uVar2 = (param_1 >> 0x10);
    if (param_2 == 0x1) {
        (iVar1 + 0x14) = 0x0;
        return;
    }
    if (param_2 != 0xc) {
        return;
    }
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (iVar1 + 0x18)),
        CONCAT22(param_3, &local_6),
        CONCAT22(param_3, &local_4),
    );
    uVar3 = pass1_1010_2b66((iVar1 + 0x14));
    // uStack8 = (uVar3 >> 0x10);
    uStack10 = uVar3;
    uStack14 = pass1_1008_4772((uVar3 & 0xffff | uStack8 << 0x10));
    // uVar2 = (uStack14 >> 0x10);
    local_16 = local_4;
    iStack20 = local_6;
    iStack18 = local_4 + (uStack14 + 0x4);
    iStack16 = local_6 + (uStack14 + 0x8);
    InvalidateRect16(0x1008, 0x0, &local_16);
    return;
}
