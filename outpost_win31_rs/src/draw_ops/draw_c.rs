use std::ptr::null_mut;
use std::os::raw::c_char;
use crate::draw_ops::{draw_a, draw_b};
use crate::globals::{DAT_1050_4216, DAT_1050_422c};
use crate::sound_ops;
use crate::structs::struct_57::Struct57;
use crate::unk::{block_1018_6000, block_1020_3000};
use crate::unk::block_1000_4000::pass1_1000_484c;
use crate::unk::block_1008_4000::pass1_1008_4772;
use crate::unk::block_1010_8000::{FUN_1010_830a, pass1_1010_8170};
use crate::utils::{CONCAT11, CONCAT22};
use crate::gui::cleanup;
use crate::winapi16::{CreateDC16, DeleteDC16, DeleteObject16, GetStockObject16, Rectangle16, SelectObject16, SelectPalette16};
use crate::windef16::{COLORREF, DEVMODEA, HDC16, HGDIOBJ16, HPALETTE16, HPEN16, RECT16};

pub unsafe fn draw_op_1018_6544(mut param_1: u32, param_2: *mut i16) {
    let mut puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut local_a: [u8; 0x6] = [0; 0x6];
    let mut uStack4: u16;

    if (param_2.is_null() == false) {
        uStack4 = ((param_2 + 0x4) - *param_2 >> 1) + *param_2;
        puVar1 = pass1_1008_3e54(CONCAT22(0x1050, local_a), 0x0, 0x57, uStack4);
        uVar3 = (param_1 >> 0x10);
        uVar2 = block_1018_6000::pass1_1018_659a((puVar1 >> 0x10), param_1, uVar3, CONCAT22(0x1050, local_a));
        draw_polygon_1018_661c(param_1, uVar3, 0x3, uVar2, (uVar2 >> 0x10));
    }
    return;
}

pub unsafe fn draw_polygon_1018_661c(
    mut param_1: u16,
    mut param_2: u16,
    count_param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    Polygon16(count_param_3, param_4, param_5);
    return;
}


pub unsafe fn mixed_draw_op_1018_6a7a(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_28,
) {
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut uVar1: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb0: u16;
    let mut local_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut in_stack_0000ffd8: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    uVar1 = (param_3 >> 0x10);
    BeginPaint16(CONCAT22(0x1050, &local_22), (param_3 + 0x8));
    EndPaint16(CONCAT22(0x1050, &local_22), (param_3 + 0x8));
    puVar4 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x2),
        in_stack_0000fe80,
        in_stack_0000ffa4,
        in_stack_0000ffaa,
        in_stack_0000ffae,
    );
    uVar2 = (puVar4 >> 0x10);
    if ((puVar4 + 0x20) == 0) {
        cleanup::unk_destroy_window_op_1018_6bb6(param_3);
        return;
    }
    sound_ops::mix_ui_op_1018_6adc(param_3, uVar2, in_stack_0000ffae, in_stack_0000ffb0);
    return;
}


pub unsafe fn draw_line_1020_229c(mut param_1: u32, mut param_2: u16) {
    let mut uVar2: u32;
    let mut x: i16;
    let mut iVar3: i16;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut x_00: u16;
    let mut iStack10: i16;
    let mut iVar1: i16;
    let mut pIVar1: *mut INT16 = null_mut();
    let mut uVar1: u32;

    uVar5 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x6);
    iVar1 = (uVar1 + 0x30);
    uVar2 = (param_1 + 0x6);
    pIVar1 = (uVar2 + 0x1a);
    MoveTo16(0x5, *pIVar1, param_2);
    uVar6 = (pIVar1 >> 0x10);
    iVar3 = pIVar1;
    LineTo16(0x5, (iVar3 + iVar1 * 0x8 - 0x4), param_2);
    for iStack10 in 0..iVar1 {
        piVar4 = (iStack10 * 0x8 + iVar3);
        x = (piVar4[0x2] - *piVar4 >> 1) + *piVar4;
        MoveTo16(0x5, x, param_2);
        LineTo16(0xa, x, param_2);
    }
    MoveTo16(0x5f, *pIVar1, param_2);
    LineTo16(0x5f, (iVar3 + iVar1 * 0x8 - 0x4), param_2);
    for iStack10 in 0..iVar1 {
        piVar4 = (iStack10 * 0x8 + iVar3);
        x_00 = param_2;
        MoveTo16(0x5f, (piVar4[0x2] - *piVar4 >> 1) + *piVar4, param_2);
        LineTo16(0x5a, x_00, param_2);
    }
    return;
}

pub unsafe fn draw_polygon_1020_2474(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    Polygon16(param_3, param_4, param_5);
    return;
}

pub unsafe fn palette_op_1020_92c4(struct_param_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct_1: *mut StructD;
    let mut uVar2: *mut StructD;

    uVar2 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    struct_param_1.address_offset_field_0x0 = 0x96c8;
    struct_1.address_offset_field_0x2 = 0x1020;
    if (struct_1.field11_0x12 != 0) {
        obj = SelectPalette16(0x0, struct_1.field11_0x12, struct_1.field6_0xa);
        DeleteObject16(obj);
    }
    struct_param_1.address_offset_field_0x0 = 0x3ab0;
    struct_1.address_offset_field_0x2 = 0x1008;
    struct_param_1.address_offset_field_0x0 = 0x389a;
    struct_1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn draw_op_1020_9364(param_1: *mut StructA) {
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut brush_handle_var6: HBRUSH16;
    let local_struct_1: *mut StructA;
    let mut var7: u16;
    let mut uVar7: u16;
    let mut uVar3: u16;
    let mut uVar8: u16;
    let mut HVar3: HDC16;
    let mut iStack62: i16;
    let mut uStack58: *mut astruct_737;
    let mut POpoint16_38: INT16;
    let mut hgdiobj16_var_52: HGDIOBJ16;
    let mut HStack50: HPEN16;
    let mut hdc16_var_48: HDC16;
    let mut uStack46: u32;
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut uStack34: u32;
    let mut puStack30: *mut u32;
    let mut puStack26: *mut u16;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut rect16_var_12: RECT16;
    let mut uStack14: u32;
    let mut rect16_a: RECT16;
    let mut x_var_6: u32;
    let mut uVar2: u16;
    let mut iVar2: i16;
    let mut piVar2: *mut i16;
    let mut uVar4: u32;
    let mut iVar4: i16;
    let mut uVar10: u8;
    let mut uVar11: u8;

    var7 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    GetClientRect16(&rect16_a, 0x1050);
    rect16_var_12 = rect16_a;
    uStack14 = x_var_6;
    uStack20 = DAT_1050_4216;
    iStack22 = DAT_1050_422c;
    puStack26 = PTR_u16_1050_4172_1050_4212;
    puStack30 = PTR_u16_1050_41b2_1050_4218;
    uStack34 = PTR_u16_1050_41da_1050_421c;
    uStack38 = PTR_u16_1050_4202_1050_4220;
    uStack42 = PTR_u16_1050_419a_1050_4224;
    uStack46 = PTR_u16_1050_41aa_1050_4228;
    uVar4 = &local_struct_1.field3_0x6;
    hdc16_var_48 = *(uVar4 + 0x12);
    uStack58 = (&u16_1050_0008 + 1);
    loop {
        HVar3 = hdc16_var_48;
        HStack50 = CreatePen16(*(uStack58 * 0x4 + uStack34), 0x0, 0x0);
        hgdiobj16_var_52 = SelectObject16(HStack50, HVar3);
        MoveToEx16(
            &point16_38,
            0x1050,
            (uStack58 * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16((puStack26 + uStack58 * 0x2), x_var_6, hdc16_var_48);
        iVar4 = (uStack20 - uStack58) * 0x2;
        MoveToEx16(&point16_38, 0x1050, (iVar4 + puStack26), rect16_a.x);
        LineTo16((puStack26 + iVar4), x_var_6, hdc16_var_48);
        SelectObject16(hgdiobj16_var_52, hdc16_var_48);
        DeleteObject16(HStack50);
        uStack58 = (&uStack58[-0x1].field0_0x0 + 1);
        if uStack58 >= 0x8000 {
            break;
        }
    }
    brush_handle_var6 = CreateSolidBrush16(0x2000000);
    uVar7 = (puStack26 >> 0x10);
    rect16_a = CONCAT22((puStack26 + 0x12) + 0x1, rect16_a.x);
    uVar2 = (puStack26 + 0x14);
    uStack14 = uStack14 & 0xffff | uVar2 << 0x10;
    x_var_6 = CONCAT22(uVar2, x_var_6);
    FillRect16(brush_handle_var6, &rect16_a, 0x1050);
    DeleteObject16(brush_handle_var6);
    iStack62 = 0x8;
    puVar2 = &PTR_LOOP_1050_0000;
    while (uStack58 = (puVar2 + 1), uStack58 < 0xa) {
        brush_handle_var6 = CreateSolidBrush16(*(puStack30 + iStack62 * 0x4 + 0x4));
        x_var_6 = x_var_6 & 0xffff | (rect16_a.y - 1) << 0x10;
        rect16_var_12 = (rect16_var_12 & 0xffff | (uStack14 + 1) << 0x10);
        uVar3 = (puStack26 >> 0x10);
        rect16_a = (rect16_a & 0xffff | ((iStack62 * 0x2 + puStack26) + 1) << 0x10);
        uStack14 = uStack14 & 0xffff | (uStack58 * 0x2 + puStack26 + 0x14) << 0x10;
        FillRect16(brush_handle_var6, &rect16_a, 0x1050);
        FillRect16(brush_handle_var6, &rect16_var_12, 0x1050);
        DeleteObject16(brush_handle_var6);
        iStack62 += -0x1;
        puVar2 = &uStack58.field0_0x0;
    }
    brush_handle_var6 = CreateSolidBrush16(*puStack30);
    rect16_a = (rect16_a & 0xffff);
    x_var_6 = x_var_6 & 0xffff | *puStack26 << 0x10;
    rect16_var_12 = (rect16_var_12 & 0xffff | ((uStack20 * 0x2 + puStack26) + 1) << 0x10);
    uStack14 = uStack14 & 0xffff | local_struct_1.field7_0xe << 0x10;
    FillRect16(brush_handle_var6, &rect16_a, 0x1050);
    FillRect16(brush_handle_var6, &rect16_var_12, 0x1050);
    DeleteObject16(brush_handle_var6);
    uStack58 = (&u16_1050_0002 + 1);
    loop {
        HVar3 = hdc16_var_48;
        HStack50 = CreatePen16(*(uStack58 * 0x4 + uStack38), 0x0, 0x0);
        hgdiobj16_var_52 = SelectObject16(HStack50, HVar3);
        iVar2 = uStack58 * 0x2;
        rect16_a.x = (iVar2 + uStack42) + rect16_a.x;
        uVar8 = (uStack46 >> 0x10);
        piVar1 = (iVar2 + uStack46);
        MoveToEx16(
            &point16_38,
            0x1050,
            ((iVar2 + uStack46) * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16(
            ((uStack20 - *piVar1) * 0x2 + puStack26),
            rect16_a.x,
            hdc16_var_48,
        );
        rect16_a.x = ((iStack22 - uStack58) * 0x2 + uStack42) + rect16_a.x;
        MoveToEx16(
            &point16_38,
            0x1050,
            (*piVar1 * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16(
            ((uStack20 - *piVar1) * 0x2 + puStack26),
            rect16_a.x,
            hdc16_var_48,
        );
        SelectObject16(hgdiobj16_var_52, hdc16_var_48);
        DeleteObject16(HStack50);
        uStack58 = (&uStack58[-0x1].field0_0x0 + 1);
        if uStack58 >= 0x8000 {
            break;
        }
    }

    local_struct_1.field8_0x10 = 0;
    return;
}


pub unsafe fn draw_line_1040_c302(param_1: *mut astruct_772, param_2: HDC16)

{
    let mut uVar3: u32;
    let mut uVar5: u16;
    let mut iVar7: *mut astruct_794;
    let mut iVar6: *mut astruct_793;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar2: u32;
    let mut uVar4: u32;
    let mut iVar1: i16;
    let mut uVar1: u32;

    uVar6 = (param_1 >> 0x10);
    uVar4 = (param_1 + 0x6);
    iVar1 = (uVar4 + 0x16);
    if (0x1 < iVar1) {
        uVar2 = (param_1 + 0x6);
        uVar5 = uVar2;
        uVar5 = uVar5 + 0x2a;
        uVar1 = (uVar2 & 0xffff0000 | uVar5);
        iVar7 = uVar1;
        iVar7 = &iVar7.field_0x1e;
        uVar7 = ((uVar1 & 0xffff0000) >> 0x10);
        MoveTo16(iVar7.field32_0x20 + iVar7.field34_0x24,
                 iVar7.field33_0x22 / 0x2 + (uVar1 & 0xffff0000 | ZEXT24(iVar7)), param_2);
        uVar3 = (uVar5 + iVar1 * 0x4 - 0x4);
        iVar6 = uVar3;
        iVar6 = &iVar6.field_0x1e;
        uVar3 &= 0xffff0000;
        uVar8 = (uVar3 >> 0x10);
        LineTo16(iVar6.field32_0x20, iVar6.field33_0x22 / 0x2 + (uVar3 | ZEXT24(iVar6)), param_2);
    }
    return;
}


pub unsafe fn draw_op_1040_c38e(param_1: *mut astruct_772)

{
    let mut iVar1: i16;
    let mut uVar8: u32;
    let mut iVar5: i16;
    let mut iVar11: i16;
    let mut y1: i16;
    let mut iVar12: i16;
    let mut in_DX: u16;
    let mut iVar10: *mut astruct_772;
    let mut uVar10: u16;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut DVar10: u32;
    let mut DVar9: u32;
    let mut in_stack_00000008: HDC16;
    let mut iStack26: i16;
    let mut x3: i16;
    let mut y5: i16;
    let mut x2: i16;
    let mut y4: i16;
    let mut uVar2: u32;
    let mut uVar1: u32;
    let mut uVar5: u32;
    let mut x1: *mut i16;
    let mut uVar4: u32;
    let mut uVar3: u32;
    let mut uVar6: u32;
    let mut uVar7: u32;

    uVar10 = (param_1 >> 0x10);
    iVar10 = param_1;
    uVar2 = iVar10.field5_0x6;
    iVar1 = (uVar2 + 0x18);
    if ((iVar1 != 0) && (uVar4 = iVar10.field5_0x6, (uVar4 + 0x16) != 0)) {
        iVar5 = iVar1;
        pass1_1010_2ee2(iVar10.field5_0x6);
        for iStack26 in 0..iVar1 {
            uVar3 = (iStack26 * 0x4 + iVar5);
            iVar11 = uVar3;
            iVar11 = iVar11 + 0x1e;
            x1 = (uVar3 & 0xffff0000 | iVar11);
            uVar9 = ((uVar3 & 0xffff0000) >> 0x10);
            y1 = (iVar11 + 0x24) / 0x2 + (iVar11 + 0x20);
            MoveTo16(y1, *x1, in_stack_00000008);
            LineTo16(y1, *x1 - 0xf, in_stack_00000008);
            DVar10 = GetCurrentPosition16(in_stack_00000008);
            y5 = (DVar10 >> 0x10);
            x3 = DVar10;
            if (iStack26 == 0) {
                x2 = x3;
                y4 = y5;
            }
        }
        uVar6 = iVar10.field5_0x6;
        if ((uVar6 + 0x24) != 0) {
            y4 += -0xd;
        }
        uVar7 = iVar10.field5_0x6;
        if ((uVar7 + 0x26) != 0) {
            y5 += 0xd;
        }
        uVar8 = iVar10.field5_0x6;
        uVar5 = iVar10.field5_0x6;
        uVar1 = (uVar8 + (uVar5 + 0x16) * 0x4 + 0x26);
        iVar12 = uVar1;
        iVar12 = iVar12 + 0x1e;
        uVar11 = ((uVar1 & 0xffff0000) >> 0x10);
        MoveTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20),
                 (iVar12 + 0x22) + (uVar1 & 0xffff0000 | iVar12), in_stack_00000008);
        LineTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20), x2, in_stack_00000008);
        DVar9 = GetCurrentPosition16(in_stack_00000008);
        DVar9 = (DVar9 >> 0x10);
        if (DVar9 < y4) {
            y4 = DVar9;
        }
        if (y5 < DVar9) {
            y5 = DVar9;
        }
        MoveTo16(y4, x2, in_stack_00000008);
        LineTo16(y5, x3, in_stack_00000008);
    }
    return;
}


pub unsafe fn draw_op_1040_5a06(mut param_1: u32, struct741_param_1: *mut astruct_741)

{
    let mut uVar1: u16;
    let mut caption_height_px: i16;
    let mut IVar2: i16;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut IVar3: i16;
    let mut y: i16;
    let mut IVar4: i16;
    let mut y_00: i16;
    let mut x: i16;
    let mut IVar5: i16;
    let mut in_DX: u16;
    let mut uVar6: u16;
    let mut palette_handle_7: HPALETTE16;
    let mut puVar2: *mut u32;
    let mut uVar8: u32;
    let mut struct_1: *mut astruct_741;
    let mut uVar9: u16;
    let mut base_addr: u16;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let mut uVar10: u8;
    let mut uVar14: u16;
    let mut uStack82: u16;
    let mut iStack72: i16;
    let mut iStack68: i16;
    let mut puStack54: *mut u32;
    let mut hdc16_2c: HDC16;
    let mut paint_struct_2a: [u8; 0x20] = [0; 0x20];
    let mut rect_array_local_a: [u8; 0x8] = [0; 0x8];
    let mut uVar13: u16;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut uVar4: u32;
    let mut puVar1: *mut u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar7: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar9 = (struct741_param_1 >> 0x10);
    struct_1 = struct741_param_1;
    GetWindowRect16(CONCAT13(0x10, CONCAT12(0x50, rect_array_local_a)), struct_1.field6_0x6);
    hdc16_2c = BeginPaint16(CONCAT13(0x10, CONCAT12(0x50, paint_struct_2a)), struct_1.field6_0x6);
    base_addr = 0x1008;
    palette_handle_7 = draw_a::palette_op_1008_4e08(&hdc16_2c, param_1, (_PTR_LOOP_1050_4230 + 0xe),
                                                    CONCAT13(0x10, CONCAT12(0x50, &hdc16_2c)));
    puVar2 = null_mut();
    puStack54 = null_mut();
    uVar7 = param_1;
    if (struct_1.field149_0x98 != 0) {
        uVar1 = FUN_1010_830a(0x0, param_1, 0x1008, _u16_1050_14cc, struct_1.field149_0x98);
        uVar14 = param_1;
        puStack54 = CONCAT22(uVar14, uVar1);
        uVar7 = param_1;
        uVar11 = pass1_1008_4772(CONCAT22(uVar14, uVar1));
        uVar6 = (uVar11 >> 0x10) | (uVar11 & 0xffff);
        uVar7 = uVar7 & 0xffff0000 | uVar6;
        if (uVar6 == 0) {
            puVar2 = (uVar11 & 0xffff);
            if (puStack54.is_null() == false) {
                puVar2 = puStack54;
                if (puStack54.is_null() == false) {
                    fn_ptr_1 = *puStack54;
                    (**fn_ptr_1)(0x8, uVar1, param_1, 0x1, uVar14);
                    puVar2 = puStack54;
                }
            }
            uVar1 = FUN_1010_830a(puVar2, uVar7, 0x1008, _u16_1050_14cc, 0x4d);
            puStack54 = CONCAT22(uVar7, uVar1);
        }
        uVar13 = 0x1050;
        uVar10 = SUB21(&hdc16_2c, 0x0);
        base_addr = 0x1538;
        caption_height_px = GetSystemMetrics16(SM_CYCAPTION);
        puVar2 = -(caption_height_px - 0x23);
        fn_ptr_1 = (*puStack54 + 0x4);
        (**fn_ptr_1)(0x38, puStack54, (puStack54 >> 0x10), -(caption_height_px - 0x23), uVar10, uVar13);
    }
    if (puStack54.is_null() == false) {
        uVar1 = (puStack54 >> 0x10);
        puVar2 = puStack54;
        if (puStack54.is_null() == false) {
            fn_ptr_1 = *puStack54;
            (**fn_ptr_1)(base_addr, puStack54, uVar1, 0x1, puStack54, uVar1);
            puVar2 = puStack54;
        }
    }
    uVar1 = FUN_1010_830a(puVar2, uVar7, base_addr, _u16_1050_14cc, struct_1.field148_0x96);
    puStack54 = CONCAT22(uVar7, uVar1);
    uVar14 = SUB42(0x1050, 0x0);
    uVar10 = SUB21(&hdc16_2c, 0x0);
    uVar8 = uVar7;
    IVar2 = GetSystemMetrics16(SM_CYCAPTION);
    uVar3 = *puStack54;
    fn_ptr_1 = uVar3 + 2;
    (**fn_ptr_1)(0x38, uVar1, uVar7, -(IVar2 - 0x23), uVar10, uVar14);
    if (puStack54.is_null() == false) {
        if (puStack54.is_null() == false) {
            fn_ptr_1 = uVar3;
            (**fn_ptr_1)(0x1538, uVar1, uVar7, 1);
        }
    }
    handle = CreatePen16(0x1000025, 0x0, 0x0);
    handle_00 = SelectObject16(handle, hdc16_2c);
    uVar14 = FUN_1010_830a(handle_00, uVar8, 0x1538, _u16_1050_14cc, 0x4f);
    puStack54 = CONCAT22(uVar8, uVar14);
    uVar12 = pass1_1008_4772(CONCAT13((uVar8 >> 0x8), CONCAT12(uVar8, uVar14)));
    uVar1 = (uVar12 >> 0x10);
    uVar4 = (uVar12 + 0x4);
    uVar2 = (uVar12 + 0x8);
    IVar3 = GetSystemMetrics16(SM_CYCAPTION);
    y = -(IVar3 - 0xc1);
    IVar4 = GetSystemMetrics16(SM_CYCAPTION);
    iStack72 = uVar2;
    y_00 = 0xc5 - (IVar4 - iStack72);
    MoveTo16(y, 0x82, hdc16_2c);
    iStack68 = uVar4;
    x = iStack68 * 0xa + 0x85;
    LineTo16(y, x, hdc16_2c);
    LineTo16(y_00, x, hdc16_2c);
    LineTo16(y_00, 0x82, hdc16_2c);
    LineTo16(y, 0x82, hdc16_2c);
//   for (uStack82 = 0; puVar1 = &struct_1.field147_0x94, uStack82 <= *puVar1 && *puVar1 != uStack82; uStack82 += 1)
    uStack82 = 0;
    puVar1 = &struct_1.field147_0x94;
    while uStack82 <= *puVar1 && *puVar1 != uStack82 {
        IVar5 = GetSystemMetrics16(SM_CYCAPTION);
        fn_ptr_1 = (*puStack54 + 0x4);
        (**fn_ptr_1)(0x1538, uVar14, uVar8, -(IVar5 - 0xc4));
        uStack82 += 1;
    }
    if (puStack54.is_null() == false) {
        if (puStack54.is_null() == false) {
            fn_ptr_1 = *puStack54;
            (**fn_ptr_1)(0x1538, uVar14);
        }
    }
    SelectObject16(handle_00, hdc16_2c);
    DeleteObject16(handle);
    palette_handle_7 = SelectPalette16(0x0, palette_handle_7, hdc16_2c);
    DeleteObject16(palette_handle_7);
    EndPaint16(CONCAT22(0x1050, paint_struct_2a), struct_1.field6_0x6);
    return;
}


pub unsafe fn draw_op_1040_c74c(param_1: *mut astruct_738, mut param_2: u16, hdc16_param_3: HDC16, mut param_4: u16)

{
    let mut uVar2: u16;
    let mut hdc_black_brush_1: HGDIOBJ16;
    let mut pen_handle_1: HPEN16;
    let mut handle: HGDIOBJ16;
    let mut hpalette_1: HPALETTE16;
    let mut struct_1: *mut astruct_738;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar3: u32;
    let mut uVar1: u16;
    let mut func_ptr_1: *mut *mut code;

    uVar4 = (_PTR_LOOP_1050_4230 >> 0x10);
    uVar2 = (_PTR_LOOP_1050_4230 + 0x10);
    hpalette_1 = draw_a::palette_op_1008_4e08(&hdc16_param_3, uVar2,
                                              CONCAT22(uVar2, (_PTR_LOOP_1050_4230 + 0xe)),
                                              CONCAT13(0x10, CONCAT12(0x50, &hdc16_param_3)));
    uVar5 = (param_1 >> 0x10);
    struct_1 = param_1;
    struct_1.field66_0x46 = 0x1;
    hdc_black_brush_1 = GetStockObject16(BLACK_BRUSH);
    pen_handle_1 = CreatePen16(0x1000002, 0x1, 0x0);
    hdc_black_brush_1 = SelectObject16(hdc_black_brush_1, hdc16_param_3);
    handle = SelectObject16(pen_handle_1, hdc16_param_3);
    Rectangle16(struct_1.field35_0x24, struct_1.field34_0x22, 0x0, 0x0, hdc16_param_3);
    MoveTo16(0x0, struct_1.field51_0x36 * 0x2 + struct_1.field40_0x2a, hdc16_param_3);
    LineTo16(struct_1.field35_0x24, struct_1.field51_0x36 * 0x2 + struct_1.field40_0x2a, hdc16_param_3);
    SelectObject16(hdc_black_brush_1, hdc16_param_3);
    hdc_black_brush_1 = SelectObject16(handle, hdc16_param_3);
    DeleteObject16(hdc_black_brush_1);
    uVar3 = param_1;
    func_ptr_1 = (uVar3 + 0x10);
    (**func_ptr_1)(0x1538, param_1, _param_2);
    func_ptr_1 = (uVar3 + 0x14);
    (**func_ptr_1)(0x1538, struct_1, (param_1 >> 0x10), hdc16_param_3);
    struct_1.field66_0x46 = 0;
    hpalette_1 = SelectPalette16(0x0, hpalette_1, hdc16_param_3);
    DeleteObject16(hpalette_1);
    return;
}


pub unsafe fn draw_rect_1020_40ce(
    mut param_1: u32,
    mut param_2: i16,
    mut param_3: i16,
    hdc16_param_4: HDC16,
    mut param_5: u16,
) {
    let mut pen_handle: HPEN16;
    let mut brush_handle_1: HGDIOBJ16;
    let mut right: i16;
    let mut bottom: i16;
    let mut unaff_SS: u16;
    let mut hdc: HDC16;
    let mut local_6: i16;
    let mut local_4: i16;
    let mut hdc16_var_fff2: HDC16;
    let mut iVar1: i16;

    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (param_1 + 0x10)),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    pass1_1008_3e94(
        param_1,
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    iVar1 = (param_1 + 0xa);
    Ellipse16(
        iVar1 + local_6 + param_2,
        iVar1 + local_4 + param_3,
        (local_6 - (param_1 + 0xa)) + param_2,
        (local_4 - (param_1 + 0xa)) + param_3,
        hdc16_param_4,
    );
    if ((*(param_1 + 0xe) & 1) != 0) {
        brush_handle_1 = GetStockObject16(HOLLOW_BRUSH);
        SelectObject16(brush_handle_1, hdc16_var_fff2);
        hdc = hdc16_param_4;
        pen_handle = CreatePen16(0x10000f9, 0x1, 0x0);
        SelectObject16(pen_handle, hdc);
        right = local_4 + param_3 + 0x5;
        bottom = local_6 + param_2 + 0x5;
        Rectangle16(
            bottom,
            right,
            local_6 + param_2 -0x5,
            local_4 + param_3 -0x5,
            hdc16_param_4,
        );
        brush_handle_1 = GetStockObject16(WHITE_BRUSH);
        SelectObject16(brush_handle_1, right);
        brush_handle_1 = GetStockObject16(WHITE_PEN);
        brush_handle_1 = SelectObject16(brush_handle_1, bottom);
        DeleteObject16(brush_handle_1);
    }
    return;
}

pub unsafe fn draw_op_1010_47d0(param_1: *mut Struct27, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut puVar3: *mut u32;
    let mut iVar5: i16;
    let mut hpalette16_var6: HPALETTE16;
    let mut handle: HGDIOBJ16;
    let mut hgdiobj16_00: HGDIOBJ16;
    let mut iVar4: *mut astruct_797;
    let mut uVar5: u16;
    let mut extraout_DX: *mut u8;
    let mut puVar5: *mut u8;
    let mut iVar7: i16;
    let mut iVar8: *mut astruct_739;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut iStack32: i16;
    let mut hdc16_var_1: HDC16;
    let mut devmodea_init_data: u16;
    let mut uStack16: u16;
    let mut local_e: u16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut stock_obj_handle: HGDIOBJ16;
    let mut pen_handle: HPEN16;
    let mut uVar4: u32;
    let mut puVar2: *mut u32;
    let mut uVar13: u16;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut offset_1: u16;
    let mut base_addr_1: u16;
    let mut fn_ptr_1: *mut *mut code;

    pen_handle = CreatePen16(0x77d7fb, 0x1, 0x0);
    stock_obj_handle = GetStockObject16(HOLLOW_BRUSH);
    local_e = 0;
    uStack12 = 0;
    uStack10 = 0x1;
    uStack8 = 0x1;
    puVar3 = (&param_1.field_0x26 + param_3 * 0x4);
    puVar5 = (&param_1.field33_0x28)[param_3 * 0x2];
    if ((puVar5 | puVar3) != 0) {
        fn_ptr_1 = *puVar3;
        (**fn_ptr_1)(0x1538, puVar3, puVar5, 1);
        puVar5 = extraout_DX;
    }
    iVar5 = param_3 + 0x105;
    pass1_1010_8170(puVar5, _u16_1050_14cc, iVar5);
    iVar8 = (param_3 * 0x4);
    (iVar8 + (&param_1.field_0x0 + 0x26)) = iVar5;
    (iVar8 + (&param_1.field_0x0 + 0x28)) = puVar5;
    base_addr_1 = 0x1050;
    offset_1 = 0x1380;
    uVar16 = 0;
    uVar17 = 0;
    uVar13 = 0;
    uVar14 = '\0';
    uVar15 = '\0';
    uVar12 = pass1_1008_4772((&param_1.field_0x26 + iVar8));
    uVar12 = (uVar12 >> 0x10);
    devmodea_init_data = uVar12;
    uStack16 = uVar12;
    hdc16_var_1 = CreateDC16(
        (uVar12 & 0xffff | uVar12 << 0x10),
        CONCAT13(uVar15, CONCAT12(uVar14, uVar13)),
        CONCAT22(uVar17, uVar16),
        CONCAT22(base_addr_1, offset_1),
    );
    hpalette16_var6 = draw_a::palette_op_1008_4e08(
        &hdc16_var_1,
        uVar12,
        (_PTR_LOOP_1050_4230 + 0xe),
        CONCAT22(0x1050, &hdc16_var_1),
    );
    handle = SelectObject16(pen_handle, hdc16_var_1);
    hgdiobj16_00 = SelectObject16(stock_obj_handle, hdc16_var_1);
    iStack32 = 0;
    loop {
        piVar1 = &param_1[0x1].field_0x1e;
        if (*piVar1 == iStack32 || *piVar1 < iStack32) {
            break;
        }
        iVar4 = ((iStack32 * 0x10 + param_3) * 0x8);
        uVar5 = pass1_1000_484c(
            CONCAT22(0x1050, &local_e),
            CONCAT22(&param_1[0x1].field_0x1c, iVar4 + param_1[0x1].field23_0x1a),
            0x8,
        );
        if (uVar5 != 0) {
            uVar4 = &param_1[0x1].field23_0x1a;
            uVar11 = (uVar4 >> 0x10);
            iVar7 = uVar4;
            Rectangle16(
                (iVar4 + iVar7 + 0x6),
                (iVar4 + iVar7 + 0x4),
                (iVar4 + iVar7 + 0x2),
                (iVar4 + iVar7),
                hdc16_var_1,
            );
        }
        iStack32 += 0x1;
    }
    hpalette16_var6 = SelectPalette16(0x0, hpalette16_var6, hdc16_var_1);
    DeleteObject16(hpalette16_var6);
    SelectObject16(handle, hdc16_var_1);
    SelectObject16(hgdiobj16_00, hdc16_var_1);
    DeleteDC16(hdc16_var_1);
    DeleteObject16(pen_handle);
    return;
}


pub unsafe fn draw_op_1020_30be(struct_param_1: *mut astruct_762) {
    let mut is_iconic: bool;
    let mut iVar5: *mut astruct_762;
    let mut uVar5: *mut astruct_762;
    let mut pHVar1: *mut HDC16;
    let mut pHVar2: *mut HDC16;
    let mut rect_30: [RECT16; 2] = [RECT16::default(); 2];
    let mut hbrush_40: HGDIOBJ16;
    let mut hicon_38: HICON16;
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut IVar4: i16;
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (struct_param_1 >> 0x10);
    iVar5 = struct_param_1;
    local_24 = BeginPaint16(CONCAT22(0x1050, local_22), iVar5.field4_0x4);
    is_iconic = IsIconic16(iVar5.field4_0x4);
    if (is_iconic == 0) {
        pass1_1018_0a50(iVar5.field19_0x14);
        pHVar2 = &local_24;
        IVar4 = 0x1050;
        fn_ptr_1 = (CONCAT22(0x1050, pHVar2) + 0x8);
        pHVar1 = pHVar2;
        (**fn_ptr_1)(0x1018, pHVar2, 0x1050);
        uVar2 = iVar5.field19_0x14;
        if ((uVar2 + 0x84) == 1) {
            unk_draw_op_1020_320e(struct_param_1, local_24);
        }
        fn_ptr_1 = (CONCAT22(IVar4, pHVar2) + 0x4);
        (**fn_ptr_1)(
            0x1018,
            pHVar2,
            IVar4,
            0x0,
            0x0,
            &local_24,
            0x1050,
            pHVar1,
        );
        uVar1 = iVar5.field19_0x14;
        if ((uVar1 + 0x84) != 1) {
            unk_draw_op_1020_320e(struct_param_1, local_24);
        }
        draw_op_1020_3488(struct_param_1);
        fn_ptr_1 = (CONCAT22(IVar4, pHVar2) + 0xc);
        (**fn_ptr_1)(0x1018, pHVar2, IVar4, &local_24, 0x1050);
    } else if (PTR_LOOP_1050_0010.is_null()) {
        fn_ptr_1 = (iVar5.field19_0x14 + 0x2c);
        hicon_38 = (**fn_ptr_1)(0x1538);
        if (hicon_38 != 0) {
            hbrush_40 = GetStockObject16(BLACK_BRUSH);
            GetClientRect16(rect_30, 0x1050);
            FillRect16(hbrush_40, &stack0xffc4, 0x1050);
            DrawIcon16(hicon_38, 0x2, 0x2, local_24);
        }
    }
    EndPaint16(CONCAT22(0x1050, local_22), iVar5.field4_0x4);
    return;
}


pub unsafe fn unk_draw_op_1020_320e(astruct762_param_1: *mut astruct_762, hdc_param_2: HDC16) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut obj: HPALETTE16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut struct_a: *mut astruct_762;
    let mut iVar7: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: *mut DEVMODEA;
    let mut device: *mut c_char;
    let mut driver: *mut c_char;
    let mut local_c: i16;
    let mut local_a: u32;
    let mut pHStack6: *mut HDC16;
    let mut hdc_var4: HDC16;
    let mut puVar2: *mut u32;
    let mut uVar4: u32;
    let mut uVar9: u8;
    let mut uVar10: u8;
    let mut uVar11: u16;

    hdc_var4 = hdc_param_2;
    uVar6 = (astruct762_param_1 >> 0x10);
    struct_a = astruct762_param_1;
    uVar4 = struct_a.field19_0x14;
    if ((uVar4 + 0x84) == 1) {
        uVar3 = struct_a.field19_0x14;
        uVar7 = (uVar3 >> 0x10);
        iVar7 = uVar3;
        puVar1 = (iVar7 + 0x24);
        driver = s_dib_1050_42c6;
        device = null_mut();
        uVar9 = '\0';
        uVar10 = '\0';
        uVar11 = 0;
        uVar8 = pass1_1008_4772((puVar1 & 0xffff | (iVar7 + 0x26) << 0x10));
        hdc_var4 = CreateDC16(
            uVar8,
            CONCAT22(uVar11, CONCAT11(uVar10, uVar9)),
            device,
            driver,
        );
        pHStack6 = &hdc_var4;
        ppcVar2 = (*puVar1 + 0x8);
        (**ppcVar2)(0x1538);
        in_DX = extraout_DX;
    }
    pass1_1018_0d9a(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x6c),
        local_c,
        local_a,
        0x1,
        hdc_var4,
    );
    pass1_1018_1054(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x74),
        local_c,
        local_a,
        0x2,
        hdc_var4,
    );
    pass1_1018_1320(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x68),
        local_c,
        local_a,
        0x1,
        hdc_var4,
    );
    pass1_1018_15f6(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    if (local_c != 0) {
        uVar3 = struct_a.field19_0x14;
        draw_op_1020_33c0(
            in_DX,
            astruct762_param_1,
            *(uVar3 + 0x70),
            local_c,
            local_a,
            0x1,
            hdc_var4,
        );
    }
    pass1_1018_108c(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    if (local_c != 0) {
        uVar3 = struct_a.field19_0x14;
        draw_op_1020_33c0(
            in_DX,
            astruct762_param_1,
            *(uVar3 + 0x78),
            local_c,
            local_a,
            0x0,
            hdc_var4,
        );
    }
    uVar3 = struct_a.field19_0x14;
    if ((uVar3 + 0x84) == 1) {
        obj = SelectPalette16(0x0, pHStack6, hdc_var4);
        DeleteObject16(obj);
        DeleteDC16(hdc_var4);
    }
    return;
}

pub unsafe fn draw_op_1020_33c0(
    mut param_1: u16,
    mut param_2: u32,
    colorref_param_2: COLORREF,
    mut param_4: i16,
    mut param_5: u32,
    mut param_6: i16,
    hdc16_param_6: HDC16,
) {
    let mut pen_handle: HPEN16;
    let mut object_handle: HGDIOBJ16;
    let mut brush_handle: HBRUSH16;
    let mut obj_handle_2: HGDIOBJ16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_DX: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u16;
    let mut HVar7: HDC16;
    let mut iStack20: i16;
    let mut puStack14: *mut u16;
    let mut uVar5: u16;

    if (param_4 != 0) {
        HVar7 = hdc16_param_6;
        pen_handle = CreatePen16(colorref_param_2, 0x1, 0x0);
        object_handle = SelectObject16(pen_handle, HVar7);
        HVar7 = hdc16_param_6;
        brush_handle = CreateSolidBrush16(colorref_param_2);
        obj_handle_2 = SelectObject16(brush_handle, HVar7);
        puStack14 = param_5;
        for iStack20 in 0..param_4 {
            uVar6 = (param_2 >> 0x10);
            uVar1 = param_4;
            block_1020_3000::pass1_1020_3540(param_1, param_2, uVar6, param_6, puStack14);
            if (param_6 < 1) {
                uVar2 = 0x3;
            } else {
                uVar2 = 0x4;
            }
            uVar3 = param_1;
            draw_b::draw_polygon_1020_3602(param_2, uVar6, uVar2, uVar1, param_1);
            fn_ptr_1000_17ce(CONCAT22(param_1, uVar1));
            puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x6));
            param_1 = uVar3;
        }
        obj_handle_2 = SelectObject16(obj_handle_2, hdc16_param_6);
        DeleteObject16(obj_handle_2);
        SelectObject16(object_handle, hdc16_param_6);
        DeleteObject16(pen_handle);
    }
    return;
}


pub unsafe fn draw_op_1020_3488(param_1: *mut astruct_762) {
    let mut uVar6: u32;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut obj_handle_7: HGDIOBJ16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut in_stack_00000008: HDC16;
    let mut hdc: HDC16;
    let mut local_a: u32;
    let mut puStack6: *mut u16;
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut left: i16;
    let mut hdc16_ffe2: HDC16;

    uVar5 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x14);
    puStack6 = (uVar2 & 0xffff0000 | (uVar2 + 0x36));
    pass1_1008_3e94(
        puStack6,
        CONCAT22(0x1050, &local_a),
        CONCAT22(0x1050, &local_a + 0x2),
    );
    uVar4 = (local_a - 0x3) << 0x10;
    if ((local_a - 0x3) < 0x0) {
        uVar4 = 0;
    }
    uVar1 = local_a - 0x3;
    uVar6 = uVar1;
    if (uVar1 < 0x0) {
        uVar6 = 0;
    }
    local_a = uVar4 | uVar6;
    uVar3 = (param_1 + 0x14);
    hdc = in_stack_00000008;
    handle = CreatePen16(*(uVar3 + 0x64), 0x1, 0x0);
    handle_00 = SelectObject16(handle, hdc);
    obj_handle_7 = GetStockObject16(HOLLOW_BRUSH);
    obj_handle_7 = SelectObject16(obj_handle_7, hdc16_ffe2);
    left = (local_a >> 0x10);
    Rectangle16(local_a + 0x6, left + 0x6, local_a, left, in_stack_00000008);
    SelectObject16(handle_00, in_stack_00000008);
    SelectObject16(obj_handle_7, in_stack_00000008);
    DeleteObject16(handle);
    return;
}
