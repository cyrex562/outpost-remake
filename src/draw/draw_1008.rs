use crate::{fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708}, mem_1000::mem_op_1000_160a, pass::pass_1008::{pass1_1008_3e38, pass1_1008_3e76, pass1_1008_818c}, string::string_1000::unk_str_op_1000_3d3e, util::{CONCAT22, ZEXT24}, win_struct::{HBRUSH16, HCURSOR16, HDC16, HGDIOBJ16, HPEN16, HWND16, PAINTSTRUCT16, PALETTEENTRY, RECT16}, winapi::{BeginPaint16, CreatePen16, CreateSolidBrush16, DeleteObject16, EndPaint16, GetClientRect16, GetDC16, GetDeviceCaps16, GetStockObject16, GetSystemPaletteEntries, LineTo16, MoveTo16, Polygon16, Rectangle16, ReleaseDC16, SelectObject16}};

pub fn draw_op_1008_1230(param_1: HWND16) {
    fill_rect_1008_39ac(param_1);
    return;
}

pub fn unk_draw_op_1008_61b2(
    ctx: &mut AppContext,
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u16,
    param_4: i32,
    param_5: u16,
) -> *mut Struct20 {
    let l_hgdiobj_1: HGDIOBJ16;
    let l_hcursor_1: HCURSOR16;
    let extraout_DX: *mut u8;
    let puVar1: *mut u8;
    let unaff_DI: i16;
    let l_struct_2: *mut u16;
    let uVar5: &mut Struct20;
    let iVar1: &mut Struct20;
    let iVar4: &mut Struct20;
    let uVar1: *mut u16;

    iVar1 = param_1;
    uVar5 = (param_1 >> 0x10);
    set_struct_1008_687a(param_1, param_4);
    iVar1.field_0xde = param_2;
    iVar1.field_0xe0 = 0x0;
    param_1.field_0x0 = 0x6378;
    iVar1.field_0x2 = 0x1008;
    puVar1 = extraout_DX;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.field_0x5b)),
        s_DanBrotherton_1050_0302,
    );
    l_hgdiobj_1 = GetStockObject16(0x1000);
    iVar1.hgdiobj_field_0xc6 = l_hgdiobj_1;
    l_hcursor_1 = LoadCursor16(s_tile2_bmp_1050_1538, 0x7f00);
    iVar1.hcursor_field_0xc4 = l_hcursor_1;
    iVar1.field_0xc8 = 0x200b;
    iVar1.field_0xac = 0x45000000;
    iVar1.field_0xbc = (param_4 + 0x8);
    l_struct_2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, param_5, puVar1, unaff_DI);
    uVar1 = (l_struct_2 >> 0x10);
    iVar1.field_0xb4 = 0x0;
    iVar1.field_0xb6 = 0x0;
    iVar1.field_0xb8 = (l_struct_2 + 0xa);
    iVar1.field_0xba = (l_struct_2 + 0xc);
    iVar1.field_0xca = param_3;
    win_ui_reg_class_1008_96d2(param_1, 0x1010, param_5);
    return param_1;
}

pub fn fill_rect_1008_62c0(ctx: &mut AppContext, param_1: HWND16) {
    let local_2e: [RECT16; 0x2];
    let pRStack38: *mut RECT16;
    let HStack36: HDC16;
    let local_22: PAINTSTRUCT16;

    HStack36 = BeginPaint16(param_1, &local_22);
    pRStack38 = CreateSolidBrush16(ctx.s_tile2_bmp_1050_1538);
    GetClientRect16(ctx.s_tile2_bmp_1050_1538, local_2e);
    FillRect16(ctx.s_tile2_bmp_1050_1538, pRStack38, local_2e);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    return;
}

pub fn unk_draw_op_1008_7f62(
    ctx: &mut AppContext,
    param_1: &mut Struct20,
    param_2: u16,
    Uparam_3: i32,
    param_4: u16,
) -> *mut Struct20 {
    let HVar1: HGDIOBJ16;
    let HVar2: HCURSOR16;
    let uVar4: &mut Struct20;
    let iVar3: &mut Struct20;

    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    set_struct_1008_687a(param_1, param_3);
    iVar3.field_0xde = param_2;
    param_1.field_0x0 = 0x8042;
    iVar3.field_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x5b)),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(0x1000);
    iVar3.hgdiobj_field_0xc6 = HVar1;
    HVar2 = LoadCursor16(s_tile2_bmp_1050_1538, 0x7f00);
    iVar3.hcursor_field_0xc4 = HVar2;
    iVar3.field_0xc8 = 0x2008;
    iVar3.field_0xac = 0x44000000;
    iVar3.field_0xbc = (param_3 + 0x8);
    iVar3.field_0xca = iVar3.field_0xde;
    win_ui_reg_class_1008_96d2(param_1, s_tile2_bmp_1050_1538, param_4);
    return param_1;
}

pub fn unk_draw_op_1008_80ee(
    ctx: &mut AppContext,
    param_1: &mut Struct23,
    param_2: u16,
) -> *mut Struct23 {
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let iVar3: &mut Struct23;
    let uVar3: &mut Struct23;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    param_1.field_0x0 = 0x389a;
    iVar3.field_0x2 = 0x1008;
    iVar3.field_0x54 = 0x0;
    iVar3.field_0x56 = 0x0;
    iVar3.field_0x58 = 0x0;
    param_1.field_0x0 = 0x87c8;
    iVar3.field_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x4)),
        ctx.s_MicroSpinControl_1050_0370,
    );
    iVar3.field_0x54 = 0x3;
    HVar1 = LoadCursor16(0x1000, 0x7f00);
    iVar3.field_0x58 = HVar1;
    HVar2 = GetStockObject16(s_tile2_bmp_1050_1538);
    iVar3.field_0x56 = HVar2;
    pass1_1008_818c(
        (param_1 & 0xffff | ZEXT24(uVar3) << 0x10),
        ctx.s_tile2_bmp_1050_1538,
        param_2,
    );
    return param_1;
}

pub fn draw_op_1008_8288(ctx: &mut AppContext, param_1: u16, param_2: u32, param_3: HWND16) {
    let HVar1: HGDIOBJ16;
    let HVar2: HGDIOBJ16;
    let x: i16;
    let uVar3: u16;
    let local_58: RECT16;
    let uStack84: u16;
    let uStack82: u16;
    let HStack80: HBRUSH16;
    let HStack78: HPEN16;
    let HStack76: HPEN16;
    let HStack74: HDC16;
    let uStack72: u16;
    let uStack70: u16;
    let uStack68: u16;
    let uStack66: u16;
    let uStack64: u16;
    let uStack62: u16;
    let local_3c: PAINTSTRUCT16;
    let local_1c: i16;
    let iStack26: i16;
    let iStack24: i16;
    let iStack22: i16;
    let iStack20: i16;
    let iStack18: i16;
    let local_10: i16;
    let iStack14: i16;
    let iStack12: i16;
    let uStack10: i16;
    let iStack8: i16;
    let iStack6: i16;
    let uStack4: u16;

    HStack74 = BeginPaint16(param_3, &local_3c);
    uStack4 = 0x0;
    HStack76 = CreatePen16(
        s_tile2_bmp_1050_1538,
        ctx._PTR_LOOP_1050_0368,
        (ctx.PTR__LOOP_1050_0368 >> 0x10),
    );
    HStack78 = CreatePen16(
        s_tile2_bmp_1050_1538,
        ctx.DAT_1050_0364,
        (ctx.DAT_1050_0364 >> 0x10),
    );
    HStack80 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    GetClientRect16(s_tile2_bmp_1050_1538, &local_58);
    uStack62 = uStack84;
    uStack64 = uStack82;
    uStack66 = uStack84 >> 0x1;
    uStack68 = uStack82 >> 0x1;
    uStack70 = uStack84 >> 0x2;
    uStack72 = uStack82 >> 0x2;
    HVar1 = GetStockObject16(s_tile2_bmp_1050_1538);
    HVar1 = SelectObject16(ctx.s_tile2_bmp_1050_1538, HVar1);
    HVar2 = GetStockObject16(s_tile2_bmp_1050_1538);
    HVar2 = SelectObject16(ctx.s_tile2_bmp_1050_1538, HVar2);
    Rectangle16(
        ctx.s_tile2_bmp_1050_1538,
        uStack82,
        uStack84,
        local_58.y,
        local_58.x,
    );
    MoveTo16(ctx.s_tile2_bmp_1050_1538, uStack68, 0x0);
    LineTo16(ctx.s_tile2_bmp_1050_1538, uStack68, uStack62);
    uVar3 = (param_2 >> 0x10);
    if (((param_2 + 0x4) & 0x4) != 0x0) {
        uStack4 = 0x1;
    }
    local_10 = uStack66 + uStack4;
    iStack14 = uStack72 + uStack4 + -0x2;
    iStack12 = local_10 + -0x3;
    iStack10 = uStack72 + uStack4 + 0x1;
    iStack8 = local_10 + 0x3;
    iStack6 = iStack10;
    SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack76);
    if (uStack4 == 0x0) {
        MoveTo16(ctx.s_tile2_bmp_1050_1538, uStack68 - 0x2, 0x1);
        LineTo16(ctx.s_tile2_bmp_1050_1538, 0x1, 0x1);
        LineTo16(ctx.s_tile2_bmp_1050_1538, 0x1, uStack62 - 0x1);
    }
    uStack4 = (((param_2 + 0x4) & 0x8) != 0x0);
    local_1c = uStack66 + uStack4;
    iStack22 = (uStack64 - uStack72) + uStack4;
    iStack26 = iStack22 + 0x1;
    iStack24 = local_1c + -0x3;
    iStack22 += -0x2;
    iStack20 = local_1c + 0x3;
    iStack18 = iStack22;
    if (uStack4 == 0x0) {
        MoveTo16(ctx.s_tile2_bmp_1050_1538, uStack82 - 0x2, 0x1);
        x = uStack68 + 0x1;
        LineTo16(ctx.s_tile2_bmp_1050_1538, x, 0x1);
        LineTo16(ctx.s_tile2_bmp_1050_1538, x, uStack62 - 0x1);
    }
    SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack78);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, HStack80);
    Polygon16(
        ctx.s_tile2_bmp_1050_1538,
        (&ctx.PTR_LOOP_1050_0002 + 0x1),
        &local_10,
    );
    Polygon16(
        ctx.s_tile2_bmp_1050_1538,
        (&ctx.PTR_LOOP_1050_0002 + 0x1),
        &local_1c,
    );
    SelectObject16(ctx.s_tile2_bmp_1050_1538, HVar2);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, HVar1);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(s_tile2_bmp_1050_1538, &local_3c);
    return;
}

pub fn unk_draw_op_1008_da12(param_1: &mut Struct19, param_2: u16, param_3: u16) {
    let piVar1: *mut i16;
    let bVar2: u8;
    let uVar3: u32;
    let puVar4: *mut u16;
    let hdc: HDC16;
    let IVar6: i16;
    let iVar7: i16;
    let uVar8: u16;
    let IVar5: &mut Struct80;
    let start: u16;
    let uVar9: u16;
    let entries: PALETTEENTRY;
    let count: *mut u8;
    let iVar10: i16;
    let hwnd: HWND16;
    let puStack32: *mut u16;
    let iStack16: i16;
    let lStack8: i32;

    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xc = 0x0;
    pass1_1008_3e38(CONCAT22(param_2, &param_1.field_0xe));
    param_1.field_0x14 = 0x0;
    param_1.field_0x16 = 0x0;
    param_1.field_0x18 = 0x0;
    CONCAT22(param_2, param_1) = 0xdc80;
    param_1.field_0x2 = 0x1008;
    hdc = GetDC16(0x1010);
    IVar6 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538, 0x8);
    param_1.field_0xa = IVar6;
    IVar6 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538, 0xa);
    param_1.field_0xc = IVar6;
    iVar7 = param_1.field_0xc + -0x1e0;
    count = (iVar7 >> 0xf);
    pass1_1008_3e76(
        CONCAT22(param_2, &param_1.field_0xe),
        0x0,
        iVar7 / 0x2,
        (param_1.field_0xa + -0x280) / 0x2,
    );
    hwnd = ctx.s_tile2_bmp_1050_1538;
    uVar8 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538, 0x26);
    if ((uVar8 & 0x100) != 0x0) {
        IVar6 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538, 0x68);
        param_1.field_0x14 = IVar6;
        IVar5 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538, 0x6a);
        param_1.field_0x16 = IVar5;
        if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(count, 0x1000);
        } else {
            count = ctx.PTR_LOOP_1050_5f2e;
        }
        start = fn_ptr_op_1000_1708(
            (IVar5 + 0x1) * 0x4,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c,
            count,
            0x1000,
        );
        lStack8 = CONCAT22(count, start);
        iVar7 = param_1.field_0x16;
        if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2e = count;
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(count, 0x1000);
        } else {
        }
        uVar9 = fn_ptr_op_1000_1708(
            (iVar7 + 0x1) * 0x4,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c,
            ctx.PTR_LOOP_1050_5f2e,
            0x1000,
        );
        &param_1.field_0x18 = uVar9;
        (&param_1.field_0x18 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
        if (lStack8 != 0x0) {
            if (param_1.field_0x18 != 0x0) {
                entries = (param_1.field_0x16 / 0x2);
                GetSystemPaletteEntries(0x1000, start, count, entries);
                GetSystemPaletteEntries(
                    ctx.s_tile2_bmp_1050_1538,
                    entries * 0x4 + start,
                    count,
                    entries,
                );
                puStack32 = param_1.field_0x18;
                // for (iStack16 = 0x0; puVar4 = puStack32, piVar1 = &param_1.field_0x16,
                //     *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 0x1) {
                //   bVar2 = (iStack16 * 0x4 + start);
                //   iVar7 = iStack16 * 0x4 + start;
                //   uVar3 = puStack32 >> 0x10;
                //   iVar10 = puStack32;
                //   puStack32 =
                //               (puStack32 & 0xffff0000 | (iVar10 + 0x4));
                //   *puVar4 = CONCAT11(*(iVar7 + 0x1),*(iVar7 + 0x2));
                //   (iVar10 + 0x2) = bVar2;
                // }
            }
        }
        hwnd = 0x1000;
        fn_ptr_1000_17ce(CONCAT22(count, start), 0x1000);
    }
    ReleaseDC16(hwnd, hdc);
    return;
}
