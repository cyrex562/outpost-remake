use crate::{
    prog_types::{COLORREF, DEVMODEA, HDC16, HGDIOBJ16, HPEN16},
    utils::CONCAT22,
};

// WARNING: Inlined function: struct_1010_4d5c
// WARNING: Unable to use type for symbol puVar4
// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar5
pub unsafe fn unk_draw_op_1020_2020(param_1: *mut astruct_743) {
    let mut piVar1: *mut i16;
    let mut uVar3: u32;
    let mut uVar6: u32;
    let mut iVar8: i16;
    let mut pHVar9: *mut HDC16;
    let mut iVar7: i16;
    let mut iVar9: i16;
    let mut hgdiobj16_var8: HGDIOBJ16;
    let mut hgdiobj16_var9: HGDIOBJ16;
    let mut hgdiobj16_var10: HGDIOBJ16;
    let mut HVar10: HBRUSH16;
    let mut HVar11: HPEN16;
    let mut uVar11: u16;
    let mut obj: HPALETTE16;
    let mut uVar13: u16;
    let mut extraout_DX: u16;
    let mut in_EDX: u32;
    let mut paVar14: *mut Struct57;
    let mut iVar10: *mut astruct_743;
    let mut iVar11: *mut astruct_744;
    let mut iVar12: *mut astruct_745;
    let mut puVar14: *mut astruct_735;
    let mut iVar13: i16;
    let mut uVar12: u16;
    let mut uVar15: u16;
    let mut uVar14: u16;
    let mut puVar15: *mut u16;
    let mut uVar16: u32;
    let mut piVar17: *mut i16;
    let mut local_38: [u8; 0x6] = [0; 0x6];
    let mut local_32: u16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u32;
    let mut hdc_24: HDC16;
    let mut paintstruct16_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2: u32;
    let mut puVar4: *mut u32;
    let mut uVar18: u8;
    let mut uVar19: u8;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut uVar24: u16;
    let mut iVar14: *mut astruct_746;
    let mut iVar15: *mut astruct_748;
    let mut uVar1: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut iVar16: i16;
    let mut fn_ptr_1: *mut *mut code;

    uVar24 = (in_EDX >> 0x10);
    puVar14 = &stack0xfffe;
    uVar12 = (param_1 >> 0x10);
    iVar10 = param_1;
    hdc_24 = BeginPaint16(CONCAT22(0x1050, paintstruct16_22), iVar10.field4_0x4);
    puStack40 = pass1_1010_4c2c(iVar10.field5_0x6);
    pHVar9 = &hdc_24;
    fn_ptr_1 = (*puStack40 + 0x8);
    (**fn_ptr_1)(
        0x1010,
        puStack40,
        (puStack40 >> 0x10),
        pHVar9,
        &DAT_1050_1050,
    );
    iVar10.field12_0x10 = pHVar9;
    uVar2 = iVar10.field5_0x6;
    uStack42 = (uVar2 + 0x30);
    uVar16 = iVar10.field5_0x6;
    uStack46 = *(uVar16 + 0x12);
    uStack48 = 0x14;
    local_32 = 0;
    uVar14 = 0x1008;
    puVar15 = pass1_1008_3e38(CONCAT22(0x1050, local_38));
    paVar14 = CONCAT22(uVar24, (puVar15 >> 0x10));
    while (&puVar14[-0x6].field_0x4 < (puVar14 + -0x4)) {
        iVar11 = (&puVar14[-0x6].field_0x4 * 0x4);
        uVar6 = puVar14[-0x5].field6_0x6;
        uVar16 = pass1_1008_4772((iVar11 + uVar6));
        paVar14 = (paVar14 & 0xffff0000 | uVar16 >> 0x10);
        puVar14[-0x7].field_0x2 = uVar16;
        uVar13 = (uVar16 >> 0x10);
        puVar14[-0x7].field_0x4 = uVar13;
        uVar6 = puVar14.field6_0x6;
        pass1_1020_2286(
            uVar6,
            (uVar6 >> 0x10),
            CONCAT13(0x10, CONCAT12(0x50, &puVar14[-0x5].field_0x2)),
            (uVar16 + 0x8),
        );
        uVar3 = &puVar14[-0x5].field_0x2;
        pass1_1008_3e76(
            CONCAT22(0x1050, &puVar14[-0x6].field6_0x6),
            0x0,
            uVar3,
            (uVar3 >> 0x10),
        );
        uVar6 = puVar14[-0x5].field6_0x6;
        pass1_1008_4480(
            &puVar14[-0x4].field_0x2,
            CONCAT22(0x1050, &puVar14[-0x6].field6_0x6),
            (iVar11 + uVar6),
        );
        iVar16 = &puVar14[-0x6].field_0x4;
        uVar3 = &puVar14[-0x5].field_0x2;
        uVar23 = uVar3;
        uVar20 = (uVar3 >> 0x10);
        uVar21 = (uVar3 >> 0x18);
        uVar3 = &puVar14[-0x7].field_0x2;
        uVar15 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        iVar7 = iVar12.field4_0x4 + &puVar14[-0x5].field_0x4;
        iVar9 = iVar12.field7_0x8 + &puVar14[-0x5].field_0x2;
        uVar6 = puVar14.field6_0x6;
        uVar3 = (uVar6 + 0x6);
        iVar14 = uVar3;
        uVar22 = (uVar3 >> 0x10);
        uVar18 = '\x0b';
        uVar19 = '\x10';
        if (&iVar14.field_0x1a == 0) {
            iVar8 = iVar14.field47_0x30 << 0x3;
            mem_op_1000_179c(iVar8, paVar14);
            iVar14.field_0x1a = iVar8;
            iVar14.field28_0x1c = paVar14;
        }
        uVar3 = &iVar14.field_0x1a;
        iVar15 = (iVar16 * 0x8);
        (iVar15 + uVar3) = CONCAT11(uVar21, uVar20);
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x2) = uVar23;
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x4) = iVar7;
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x6) = iVar9;
        uVar14 = CONCAT11(uVar19, uVar18);
        uVar3 = &puVar14[-0x7].field_0x2;
        piVar1 = &puVar14[-0x5].field_0x4;
        *piVar1 = *piVar1 + (-(&puVar14[-0x6].field_0x4 == 0) & 0x5) + 0x14 + (uVar3 + 0x4);
        piVar1 = &puVar14[-0x6].field_0x4;
        *piVar1 = *piVar1 + 1;
    }
    puVar4 = &puVar14[-0x4].field_0x2;
    fn_ptr_1 = (*puVar4 + 0x4);
    (**fn_ptr_1)(
        uVar14,
        puVar4,
        (puVar4 >> 0x10),
        0x0,
        0x0,
        puVar14 + -0x22,
        &DAT_1050_1050,
    );
    extraout_DX = paVar14;
    hgdiobj16_var8 = CreatePen16(0x1000025, 0x1, 0x0);
    puVar14[-0x6].field_0x2 = hgdiobj16_var8;
    hgdiobj16_var10 = SelectObject16(hgdiobj16_var8, *&puVar14[-0x4].field6_0x6);
    (puVar14 + -0x6) = hgdiobj16_var10;
    hgdiobj16_var9 = CreateSolidBrush16(0x1000025);
    (&puVar14[-0x7].field6_0x6 + 0x2) = hgdiobj16_var9;
    hgdiobj16_var10 = SelectObject16(hgdiobj16_var9, *&puVar14[-0x4].field6_0x6);
    puVar14[-0x7].field6_0x6 = hgdiobj16_var10;
    draw_line_1020_229c(puVar14.field6_0x6, &puVar14[-0x4].field6_0x6);
    uVar1 = puVar14.field6_0x6;
    pass1_1010_4df0(extraout_DX, (uVar1 + 0x6));
    if (hgdiobj16_var10 == 0) {
        hgdiobj16_var10 = SelectObject16((puVar14 + -0x6), *&puVar14[-0x4].field6_0x6);
        DeleteObject16(hgdiobj16_var10);
        hgdiobj16_var10 = SelectObject16(&puVar14[-0x7].field6_0x6, *&puVar14[-0x4].field6_0x6);
        DeleteObject16(hgdiobj16_var10);
        HVar10 = CreateSolidBrush16(0xff);
        (&puVar14[-0x7].field6_0x6 + 0x2) = HVar10;
        HVar11 = CreatePen16(0xff, 0x1, 0x0);
        puVar14[-0x6].field_0x2 = HVar11;
        SelectObject16(
            (&puVar14[-0x7].field6_0x6 + 0x2),
            *&puVar14[-0x4].field6_0x6,
        );
        SelectObject16(&puVar14[-0x6].field_0x2, *&puVar14[-0x4].field6_0x6);
    }
    uVar5 = puVar14.field6_0x6;
    piVar17 = pass1_1010_4dc8((uVar5 + 0x6));
    uVar13 = (piVar17 >> 0x10);
    uVar11 = piVar17;
    pass1_1020_239c(puVar14.field6_0x6, piVar17);
    uVar6 = puVar14.field6_0x6;
    uVar4 = (uVar6 + 0x6);
    if ((uVar4 + 0x2c) != 0) {
        pass1_1020_2488(uVar11, uVar13, puVar14.field6_0x6);
    }
    uVar6 = puVar14.field6_0x6;
    obj = SelectPalette16(0x0, (uVar6 + 0x10), *&puVar14[-0x4].field6_0x6);
    DeleteObject16(obj);
    hgdiobj16_var10 = SelectObject16((puVar14 + -0x6), *&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    hgdiobj16_var10 = SelectObject16(&puVar14[-0x7].field6_0x6, *&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    uVar6 = puVar14.field6_0x6;
    EndPaint16(
        CONCAT22(0x1050, (&puVar14[-0x4].field6_0x6 + 0x2)),
        (uVar6 + 0x4),
    );
    return;
}
pub unsafe fn pass1_1020_2286(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut i16,
    mut param_4: i16,
) {
    *param_3 = 0x64 - param_4 >> 0x1;
    return;
}

// WARNING: Unable to use type for symbol pIVar1
// WARNING: Unable to use type for symbol uVar1
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
    LineTo16(0x5, (iVar3 + iVar1 * 0x8 + -0x4), param_2);
    for iStack10 in 0..iVar1 {
        piVar4 = (iStack10 * 0x8 + iVar3);
        x = (piVar4[0x2] - *piVar4 >> 1) + *piVar4;
        MoveTo16(0x5, x, param_2);
        LineTo16(0xa, x, param_2);
    }
    MoveTo16(0x5f, *pIVar1, param_2);
    LineTo16(0x5f, (iVar3 + iVar1 * 0x8 + -0x4), param_2);
    for iStack10 in 0..iVar1 {
        piVar4 = (iStack10 * 0x8 + iVar3);
        x_00 = param_2;
        MoveTo16(0x5f, (piVar4[0x2] - *piVar4 >> 1) + *piVar4, param_2);
        LineTo16(0x5a, x_00, param_2);
    }
    return;
}
pub unsafe fn pass1_1020_239c(mut param_1: u32, param_2: *mut i16) {
    let mut puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut local_a: [u8; 0x6] = [0; 0x6];
    let mut uStack4: u16;

    if (param_2.is_null() == false) {
        uStack4 = ((param_2 + 0x4) - *param_2 >> 1) + *param_2;
        puVar1 = pass1_1008_3e54(CONCAT22(0x1050, local_a), 0x0, 0x57, uStack4);
        uVar3 = (param_1 >> 0x10);
        uVar2 = pass1_1020_23f2((puVar1 >> 0x10), param_1, uVar3, CONCAT22(0x1050, local_a));
        draw_polygon_1020_2474(param_1, uVar3, 0x3, uVar2, (uVar2 >> 0x10));
    }
    return;
}

pub unsafe fn pass1_1020_23f2(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u16,
) -> u32 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iStack18: i16;
    let mut local_6: i16;
    let mut local_4: i16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    piVar1 = &local_6;
    pass1_1008_3e94(
        param_4,
        CONCAT22(0x1050, piVar1),
        CONCAT22(0x1050, &local_4),
    );
    mem_op_1000_179c(0xc, paVar3);
    uVar2 = SUB42(paVar3, 0x0);
    for iStack18 in 0..0x3 {
        piVar1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4270) + local_4;
        piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x4272) + local_6;
    }
    return CONCAT22(uVar2, piVar1);
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
pub unsafe fn pass1_1020_2488(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000fff2: i16;
    let mut iStack12: i16;
    let mut uStack10: u16;

    uVar5 = (param_3 >> 0x10);
    iVar4 = param_3;
    find_n_load_rsrc_1010_4e9e((iVar4 + 0x6));
    if ((param_2 | param_1) != 0) {
        uStack10 = param_1;
        for iStack12 in 0..0x9 {
            uVar1 = (iVar4 + 0x6);
            uVar2 = pass1_1010_4f20(uVar1, (uVar1 >> 0x10), iStack12);
            uVar1 = (iVar4 + 0xa);
            set_win_tet_1020_1d2a(
                uVar1,
                (uVar1 >> 0x10),
                CONCAT22(param_2, uStack10),
                uVar2,
                in_stack_0000fff2,
            );
            uVar3 = str_op_1000_3da4(CONCAT22(param_2, uStack10));
            uStack10 += uVar3 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1020_24f2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_1f74(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn struct_1020_2524(
    mut param_1: u16,
    param_2: *mut astruct_20,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut astruct_20;
    let mut unaff_BP: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    unk_draw_op_1020_7f7a(param_2, 0x7, CONCAT22(param_4, param_3), param_5);
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field5_0xc = 0;
    iVar2[0x1].field7_0x10 = null_mut();
    param_2.offset_0x0 = 0x270c;
    iVar2.base_0x2 = 0x1020;
    (iVar2 + 1).offset_0x0 = 0x27a8;
    iVar2[0x1].base_0x2 = 0x1020;
    puVar4 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x2a),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = (puVar4 >> 0x10);
    iVar2[0x1].field7_0x10 = puVar4;
    (&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
    iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
    (&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
    return;
}
pub unsafe fn pass1_1020_2594(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x270c;
    iVar1.address_offset_field_0x2 = 0x1020;
    iVar1.field_0xe2 = 0x27a8;
    iVar1.field_0xe4 = 0x1020;
    pass1_1020_808e(param_1);
    return;
}
pub unsafe fn pass1_1020_25c0(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar3: *mut astruct_277;
    let mut uVar5: u16;
    let mut paStack10: *mut Struct57;
    let mut puStack6: *mut u32;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    uVar5 = (param_3 >> 0x10);
    iVar3 = param_3;
    if (iVar3.field236_0xee.is_null() == false) {
        ppcVar2 = (*iVar3.field236_0xee + 0x8);
        param_1 = (**ppcVar2)();
    }
    if (iVar3.field233_0xea == 0) {
        iVar3.field233_0xea = 0x1;
        mem_op_1000_179c(0x98, paVar4);
        paStack10 = CONCAT22(paVar4, param_1);
        uVar3 = paVar4 | param_1;
        if (uVar3 == 0) {
            puStack6 = null_mut();
        } else {
            piVar1 = &iVar3.field204_0xcc;
            *piVar1 = *piVar1 + 1;
            struct_1020_1738(
                paStack10,
                iVar3.field204_0xcc,
                param_3 & 0xffff | uVar5 << 0x10,
            );
            puStack6 = CONCAT22(uVar3, param_1);
        }
        ppcVar2 = (*puStack6 + 0x8);
        (**ppcVar2)(0x1000, puStack6, (puStack6 >> 0x10));
    }
    return;
}
pub unsafe fn window_op_1020_2642(
    param_1: *mut astruct_664,
    param_2: *mut astruct_57,
    param_3: *mut StructA,
) {
    let mut uVar1: u16;
    let iVar2: *mut StructA;
    let mut uVar2: u16;

    create_window_ex_1008_9760(param_3);
    uVar2 = (param_3 >> 0x10);
    iVar2 = param_3;
    get_dc_1018_4db0(&iVar2[0x1].field20_0x26, iVar2.field4_0x8);
    mem_op_1000_179c(0x18, param_2);
    uVar1 = param_2 | param_1;
    if (uVar1 != 0) {
        pass1_1020_27b0(uVar1, param_1, param_2, iVar2.field4_0x8);
        iVar2[0x1].field18_0x22 = param_1;
        iVar2[0x1].field19_0x24 = uVar1;
        return;
    }
    iVar2[0x1].field18_0x22 = 0;
    return;
}
pub unsafe fn pass1_1020_26a6(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1);
    return;
}

pub unsafe fn pass1_1020_26d8(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_2594(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1020_26e6(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1020_2594(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_27b0(
    mut param_1: u16,
    param_2: *mut astruct_664,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffca: u32;
    let mut in_stack_0000fff2: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    set_struct_op_1020_921c(
        param_1,
        CONCAT22(param_3, param_2),
        param_4,
        in_stack_0000ffca,
    );
    param_2.field16_0x14 = 0;
    CONCAT22(param_3, param_2) = 0x288e;
    param_2.field2_0x2 = 0x1020;
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x2a),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar3 = (puVar6 >> 0x10);
    param_2.field16_0x14 = puVar6;
    param_2.field17_0x16 = uVar3;
    param_2.field5_0x6 = param_2.field16_0x14;
    param_2.field6_0x8 = uVar3;
    uVar2 = &param_2.field16_0x14;
    puVar4 = &param_2.field_0xa;
    ppcVar1 = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_2.field15_0x12 = puVar4;
    draw_op_1020_9364(CONCAT22(param_3, param_2));
    return;
}
pub unsafe fn pass1_1020_2838(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x288e;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1dda(iVar1.field12_0x14);
    }
    palette_op_1020_92c4(param_1);
    return;
}

pub unsafe fn pass1_1020_2868(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_2838(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn pass1_1020_289a(param_1: *mut u16, mut param_2: u16, mut param_3: u32) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    struct_1020_790e(param_1, s_SCPOPMENU_1050_427c, param_2, param_3);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xf2) = 0;
    (iVar1 + 0xf6) = 0;
    (iVar1 + 0xfa) = 0;
    (iVar1 + 0xfc) = 0;
    *param_1 = 0x2e4a;
    (iVar1 + 0x2) = 0x1020;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (iVar1 + 0x5b)), s_VrMode_1050_4286);
    (iVar1 + 0xac) = 0x44c00000;
    return;
}
pub unsafe fn pass1_1020_28fc(param_1: *mut StructD) {
    param_1.address_offset_field_0x0 = 0x2e4a;
    (param_1 + 0x2) = 0x1020;
    cleanup_menu_ui_op_1020_795c(param_1);
    return;
}
pub unsafe fn post_win_msg_1020_291a(mut param_1: u32) {
    PostMessage16(0x0, 0x0, 0x10, (param_1 + 0x8));
    return;
}
pub unsafe fn pass1_1020_2936() {
    pass1_1020_79ae();
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_294a(
    param_1: *mut u8,
    param_2: *mut astruct_665,
    mut param_3: u32,
    mut param_4: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut iVar3: *mut astruct_665;
    let mut unaff_BP: u16;
    let mut uVar2: *mut astruct_665;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    uVar2 = (param_2 >> 0x10);
    iVar3 = param_2;
    iVar3.field248_0xfc = param_4;
    puVar2 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, param_4),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = (puVar2 >> 0x10);
    iVar3.field240_0xf2 = puVar2;
    iVar3.field241_0xf4 = uVar1;
    iVar3.field224_0xe0 = iVar3.field240_0xf2;
    iVar3.field225_0xe2 = uVar1;
    pass1_1018_0902(&iVar3.field240_0xf2, param_3);
    return;
}
pub unsafe fn realize_palette_1020_2992(mut param_1: u32, mut param_2: i16) {
    let mut obj: HGDIOBJ16;
    let mut hdc: HDC16;
    let mut uVar1: u16;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (param_2 != 0) {
        uVar1 = (param_1 >> 0x10);
        puVar1 = pass1_1018_0a50((param_1 + 0xf2));
        fn_ptr_1 = (*puVar1 + 0x18);
        obj = (**fn_ptr_1)(0x1018);
        UnrealizeObject16(obj);
        hdc = GetDC16((param_1 + 0x8));
        RealizePalette16(hdc);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn send_msg_1020_29d8(
    mut param_1: u16,
    param_2: *mut astruct_57,
    param_3: *mut astruct_69,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u16,
) -> u32 {
    let mut puVar1: *mut u8;
    let mut paVar2: *mut astruct_27;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut iVar3: i16;

    iVar3 = (param_5 >> 0x10);
    post_win_msg_1020_79fc(param_3, param_4, param_5, iVar3);
    paVar2 = mixed_1010_20ba(
        param_2,
        _u16_1050_0ed0,
        CONCAT22(param_6, 0x29),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    puVar1 = (paVar2 >> 0x10);
    if (iVar3 == 0) {
        pass1_1018_270e(puVar1, paVar2, 0x1, (param_3 + 0xfc));
    } else {
        pass1_1018_270e(puVar1, paVar2, 0x0, (param_3 + 0xfc));
        SendMessage16(0x0, 0x69, 0x111, HWND16_1050_0396);
    }
    return CONCAT22(param_2, param_1);
}
pub unsafe fn pass1_1020_2a46(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    let mut in_DX: u16;

    pass1_1018_0ae8((param_1 + 0xf2), 1);
    pass1_1008_68c6(in_DX, param_1, param_2, param_3);
    return;
}
pub unsafe fn pass1_1020_2a6a(mut param_1: u32) {
    get_win_ui_info_op_1020_7a50(param_1);
    pass1_1018_0ae8((param_1 + 0xf2), 0x0);
    destroy_icon_1020_2c88(param_1);
    return;
}
pub unsafe fn pass1_1020_2a94(mut param_1: u32, mut param_2: u32) {
    pass1_1018_1662((param_1 + 0xf2), param_2, (param_2 >> 0x10));
    return;
}
pub unsafe fn bring_window_to_top_1020_2aae(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u16;
    let mut unaff_SS: u16;

    pass1_1008_3e0e(param_1);
    uVar1 = (param_1 >> 0x10);
    BringWindowToTop16((param_1 + 0x8));
    pass1_1018_169e((param_1 + 0xf2), param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn invalidate_rect_1020_2ae4(
    mut param_1: u16,
    param_2: *mut u32,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut cVar3: u8;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_CS: u16;
    let mut paVar10: *mut astruct_477;
    let mut puVar11: *mut u32;
    let mut uVar12: u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut wparam: WPARAM16;
    let mut hwnd: HWND16;

    paVar7 = CONCAT22(in_register_0000000a, param_1);
    if (param_3 != 0x129) {
        uVar8 = param_2;
        uVar9 = (param_2 >> 0x10);
        if (0x129 < param_3) {
            if (param_3 == 0x12a) {
                hwnd = (uVar8 + 0x8);
                wparam = 0xf012;
            } else {
                if (param_3 == 0x12b) {
                    return;
                }
                if (param_3 == 0x12c) {
                    hwnd = (uVar8 + 0x8);
                    wparam = 0xf020;
                } else {
                    if (param_3 == 0x12d) {
                        return;
                    }
                    if (param_3 != 0x12e) {
                        return;
                    }
                    hwnd = (uVar8 + 0x8);
                    wparam = 0xf060;
                }
            }
            PostMessage16(0x0, wparam, 0x112, hwnd);
            return;
        }
        if (param_3 == 0xfb) {
            puVar11 = mixed_1010_20ba(
                paVar7,
                _u16_1050_0ed0,
                CONCAT22(param_4, 0x30),
                in_stack_0000fe9c,
                in_stack_0000ffc0,
                in_stack_0000ffc6,
                in_stack_0000ffca,
            );
            pass1_1010_375e(puVar11);
            ppcVar1 = (*param_2 + 0x14);
            (**ppcVar1)();
            uVar12 = pass1_1010_375e(puVar11);
            uVar2 = (uVar12 >> 0x10);
            pass1_1018_181c(
                CONCAT22(uVar12, uVar2),
                (uVar8 + 0xf2),
                (uVar12 & 0xffff | uVar2 << 0x10),
            );
            return;
        }
        if (param_3 < 0xfc) {
            cVar3 = param_3;
            if ((cVar3 + 0x91) == 0) {
                uVar5 = FUN_1010_830a(
                    param_3 & 0xff00 | (cVar3 + 0x91),
                    paVar7,
                    unaff_CS,
                    _u16_1050_14cc,
                    0x1f8,
                );
                WinHelp16(0x2a, 0x1, CONCAT22(paVar7, uVar5), (uVar8 + 0x8));
                return;
            }
            if (cVar3 == 'r') {
                iVar4 = uVar8 + 0xa;
                uVar5 = uVar9;
                paVar10 = mixed_1010_20ba(
                    paVar7,
                    _u16_1050_0ed0,
                    CONCAT22(iVar4, 0x30),
                    in_stack_0000fe98,
                    in_stack_0000ffbc,
                    in_stack_0000ffc2,
                    in_stack_0000ffc6,
                );
                uVar6 = (paVar10 >> 0x10);
                pass1_1010_3770(uVar6, paVar10, CONCAT22(uVar5, iVar4));
                pass1_1038_af40(uVar8, uVar6, _PTR_LOOP_1050_5b7c, (uVar8 + 0x8), 0x3);
                return;
            }
            if (cVar3 == 'u') {
                pass1_1018_0a76((uVar8 + 0xf2));
                InvalidateRect16(0x0, NULL, 0x0);
                return;
            }
        }
    }
    return;
}

pub unsafe fn enable_menu_item_1020_2c2a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: HMENU16,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut w_flags: u16;

    if (param_4 != 0) {
        return param_4 - 0x1;
    }
    EnableMenuItem16(0x400, 0x3, param_5);
    if (PTR_LOOP_1050_3960 < 0x2) {
        w_flags = 0x401;
    } else {
        w_flags = 0x400;
    }
    BVar1 = EnableMenuItem16(w_flags, 0x5, param_5);
    return BVar1;
}
pub unsafe fn pass1_1020_2c72(mut param_1: u32) {
    draw_op_1020_30be((param_1 + 0xf6));
    return;
}
pub unsafe fn destroy_icon_1020_2c88(param_1: *mut astruct_869) {
    let mut ppcVar1: *mut *mut code;
    let mut struct_1: *mut astruct_869;
    let mut uVar3: u16;
    let mut uVar1: u16;
    let mut puVar1: *mut u32;

    uVar3 = (param_1 >> 0x10);
    struct_1 = param_1;
    DestroyIcon16(struct_1.field193_0xc2);
    struct_1.field193_0xc2 = 0;
    struct_1.field8_0x8 = 0;
    puVar1 = struct_1.field241_0xf6;
    uVar1 = struct_1.field242_0xf8;
    if ((uVar1 | puVar1) != 0) {
        ppcVar1 = *puVar1;
        (**ppcVar1)(s_tile2_bmp_1050_1538, puVar1, uVar1, 1);
    }
    struct_1.field241_0xf6 = 0;
    pass1_1010_1dda(struct_1.field240_0xf2);
    struct_1.field240_0xf2 = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_op_1020_2cf0(
    param_1: *mut astruct_57,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_160;
    let mut pIVar4: *mut INT16 = null_mut();
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let pSVar11: *mut StructA;
    let mut uVar12: u16;
    let iVar10: *mut StructA;

    pSVar11 = param_2;
    uVar12 = (param_2 >> 0x10);
    create_window_ex_1008_9760(param_2);
    puVar9 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, pSVar11[0x1].field26_0x30),
        param_3,
        param_4,
        param_5,
        param_6,
    );
    paVar8 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    pSVar11[0x1].field20_0x26 = puVar9;
    uVar5 = (puVar9 >> 0x10);
    pSVar11[0x1].field21_0x28 = uVar5;
    pSVar11[0x1].field10_0x14 = pSVar11[0x1].field20_0x26;
    pSVar11[0x1].field11_0x16 = uVar5;
    paVar3 = LoadIcon16(s_SITEICON_1050_428d, HINSTANCE16_1050_038c);
    pSVar11.field_0xc2 = paVar3;
    uVar1 = &pSVar11[0x1].field20_0x26;
    ppcVar2 = (*&pSVar11[0x1].field20_0x26 + 0x30);
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar1, (uVar1 >> 0x10), paVar3);
    mem_op_1000_179c(0x22, paVar8);
    uVar6 = paVar8 | paVar3;
    uVar10 = paVar8 & 0xffff0000 | uVar6;
    if (uVar6 == 0) {
        pSVar11[0x1].field22_0x2a = 0;
    } else {
        load_draw_op_1020_2ede(uVar10, param_7, CONCAT22(paVar8, paVar3), pSVar11, uVar12);
        pSVar11[0x1].field22_0x2a = paVar3;
        pSVar11[0x1].field_0x2c = uVar10;
    }
    pSVar11[0x1].field14_0x1c = &pSVar11[0x1].field22_0x2a;
    pass1_1018_0ac0(&pSVar11[0x1].field20_0x26, param_2);
    uVar10 = pass1_1018_0b08(&pSVar11[0x1].field20_0x26);
    uVar7 = (uVar10 >> 0x10);
    pIVar4 = uVar10;
    ppcVar2 = (param_2 + 0x14);
    (**ppcVar2)();
    ppcVar2 = (*&pSVar11[0x1].field20_0x26 + 0x10);
    (**ppcVar2)();
    MoveWindow16(
        0x1,
        pIVar4[0x3],
        pIVar4[0x2],
        pIVar4[0x1],
        *pIVar4,
        pSVar11.field4_0x8,
    );
    pass1_1008_3e0e(param_2);
    UpdateWindow16(pSVar11.field4_0x8);
    return;
}

pub unsafe fn pass1_1020_2e24(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_28fc(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Unable to use type for symbol puVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn load_draw_op_1020_2ede(
    param_1: *mut astruct_57,
    mut param_2: u16,
    param_3: *mut astruct_40,
    param_4: *mut StructA,
    mut param_5: u16,
) {
    let mut hdc_dev_ctx_1: HDC16;
    StructA * *ppSVar1;
    let mut handle: HPEN16;
    let mut pHVar2: *mut HDC16;
    let mut h_null_brush: HGDIOBJ16;
    let mut in_DX: *mut u8;
    let mut struct_1: *mut astruct_40;
    let mut unaff_DI: i16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut puVar5: *mut u16;
    let mut paVar3: *mut astruct_76;
    let mut devmodea_ptr_var11: *mut DEVMODEA;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut device: *mut c_char;
    let mut driver: *mut c_char;
    let mut in_stack_0000ffea: u16;
    let mut puVar1: *mut u32;
    let mut puVar2: u32;
    let mut uVar10: COLORREF;
    let mut output: LPCSTR = null_mut();
    let mut fn_ptr_1: *mut *mut code;

    get_sys_metrics_1020_7c1a(param_3, CONCAT22(param_5, param_4));
    uVar5 = (param_3 >> 0x10);
    struct_1 = param_3;
    struct_1.field17_0x14 = 0;
    struct_1.field20_0x18 = 0;
    (&struct_1.field20_0x18 + 0x2) = 0;
    struct_1.field21_0x1c = null_mut();
    struct_1.field_0x1e = 0;
    (&struct_1[0x1].field0_0x0 + 1) = 0;
    param_3.field0_0x0 = 0x363c;
    struct_1.field1_0x2 = 0x1020;
    puVar5 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_2, param_4[0x1].field26_0x30),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    struct_1.field17_0x14 = puVar5;
    struct_1.field_0x16 = (puVar5 >> 0x10);
    fn_ptr_1 = (*&struct_1.field17_0x14 + 0x4);
    (**fn_ptr_1)();
    driver = s_dib_1050_42c2;
    device = null_mut();
    output = null_mut();
    paVar3 = pass1_1018_0a50(&struct_1.field17_0x14);
    devmodea_ptr_var11 = pass1_1008_4772(paVar3);
    hdc_dev_ctx_1 = CreateDC16(devmodea_ptr_var11, output, device, driver);
    *&struct_1.field20_0x18 = hdc_dev_ctx_1;
    ppSVar1 = &struct_1.field20_0x18;
    fn_ptr_1 = (paVar3 + 0x8);
    (**fn_ptr_1)();
    (&struct_1[0x1].field0_0x0 + 1) = ppSVar1;
    puVar2 = &struct_1.field17_0x14;
    handle = CreatePen16(*(puVar2 + 0x64), 0x1, 0x0);
    (&struct_1.field20_0x18 + 0x2) = handle;
    pHVar2 = SelectObject16(handle, *&struct_1.field20_0x18);
    struct_1.field21_0x1c = pHVar2;
    h_null_brush = GetStockObject16(HOLLOW_BRUSH);
    h_null_brush = SelectObject16(h_null_brush, *&struct_1.field20_0x18);
    struct_1.field_0x1e = h_null_brush;
    return;
}
pub unsafe fn cleanup_win_ui_1020_2fea(in_struct_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct_1: *mut StructD;
    let mut var2: u16;
    let mut unaff_SS: u16;

    var2 = (in_struct_1 >> 0x10);
    struct_1 = in_struct_1;
    in_struct_1.address_offset_field_0x0 = 0x363c;
    struct_1.address_offset_field_0x2 = 0x1020;
    if (struct_1.field12_0x14 != 0) {
        pass1_1010_1ea6(struct_1.field12_0x14, (in_struct_1 & 0xffff | var2 << 0x10));
    }
    SelectObject16(&struct_1.field_0x1c, struct_1.field13_0x18);
    SelectObject16(&struct_1.field_0x1e, struct_1.field13_0x18);
    DeleteObject16(struct_1.field14_0x1a);
    obj = SelectPalette16(0x0, struct_1.field19_0x20, struct_1.field13_0x18);
    DeleteObject16(obj);
    DeleteDC16(struct_1.field13_0x18);
    in_struct_1.address_offset_field_0x0 = 0x3ab0;
    struct_1.address_offset_field_0x2 = 0x1008;
    in_struct_1.address_offset_field_0x0 = 0x389a;
    struct_1.address_offset_field_0x2 = 0x1008;
    return;
}
