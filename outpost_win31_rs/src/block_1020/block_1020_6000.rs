pub unsafe fn pass1_1020_6184(mut param_1: u32, mut param_2: u16) {
    let mut HVar1: HCURSOR16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xf4) == 1) {
        HVar1 = SetCursor16((iVar2 + 0xf0));
        (iVar2 + 0xee) = HVar1;
        (iVar2 + 0x10c) = param_2;
        SetCapture16((iVar2 + 0x8));
        (iVar2 + 0xf4) = 0x2;
    }
    return;
}


pub unsafe fn pass1_1020_61c4(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut in_EDX: *mut Struct57;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000fff2: u16;

    puVar3 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x2f),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar2 = (puVar3 >> 0x10);
    uVar1 = (puVar3 + 0x20);
    pass1_1030_8308(
        uVar1,
        uVar2,
        _u16_1050_5748,
        (_u16_1050_5748 >> 0x10),
        param_3,
        param_4,
        uVar1,
    );
    *param_4 = (puVar3 + 0x1e);
    return;
}



pub unsafe fn pass1_1020_62e0(mut param_1: i16, mut param_2: u16, mut param_3: u16) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_EDX: *mut Struct57;
    let mut paVar8: *mut Struct57;
    let mut paVar9: *mut Struct57;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fe3e: u16;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6c: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc6: u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut iVar14: i16;
    let mut uVar15: u16;
    let mut in_stack_0000ffee: u16;
    let mut paVar7: *mut Struct57;

    set_struct_op_1020_921c(
        in_EDX,
        CONCAT22(param_2, param_1),
        param_3,
        in_stack_0000ffc6,
    );
    (param_1 + 0x14) = 0;
    (param_1 + 0x2c) = 0;
    CONCAT22(param_2, param_1) = 0x67c2;
    (param_1 + 0x2) = 0x1020;
    puVar3 = pass1_1000_4906(CONCAT22(param_2, param_1 + 0x18), NULL, 0x14);
    mem_op_1000_179c(0x3c, in_EDX);
    uVar5 = in_EDX | puVar3;
    paVar7 = (in_EDX & 0xffff0000 | uVar5);
    if (uVar5 == 0) {
        (param_1 + 0x1c) = 0;
    } else {
        pass1_1020_87c2(CONCAT22(in_EDX, puVar3));
        (param_1 + 0x1c) = puVar3;
        (param_1 + 0x1e) = paVar7;
    }
    mem_op_1000_179c(0x26, paVar7);
    uVar5 = paVar7 | puVar3;
    paVar9 = (paVar7 & 0xffff0000);
    paVar8 = (paVar9 | uVar5);
    if (uVar5 == 0) {
        puVar3 = null_mut();
    } else {
        pass1_1020_8a9c(
            CONCAT22(paVar7, puVar3),
            paVar8,
            in_stack_0000ff68,
            in_stack_0000ff6c,
            in_stack_0000ff62,
            in_stack_0000fe3e,
        );
        paVar9 = paVar8;
    }
    (param_1 + 0x20) = puVar3;
    (param_1 + 0x22) = paVar9;
    mem_op_1000_179c(0xbe, paVar9);
    uVar5 = paVar9 | puVar3;
    paVar7 = (paVar9 & 0xffff0000);
    paVar8 = (paVar7 | uVar5);
    if (uVar5 == 0) {
        puVar3 = null_mut();
    } else {
        pass1_1020_8eaa(CONCAT22(paVar9, puVar3), paVar8);
        paVar7 = paVar8;
    }
    (param_1 + 0x24) = puVar3;
    (param_1 + 0x26) = paVar7;
    mem_op_1000_179c(0x20, paVar7);
    uVar5 = paVar7 | puVar3;
    paVar9 = (paVar7 & 0xffff0000);
    paVar8 = (paVar9 | uVar5);
    if (uVar5 == 0) {
        puVar3 = null_mut();
    } else {
        pass1_1020_8360(CONCAT22(paVar7, puVar3));
        paVar9 = paVar8;
    }
    (param_1 + 0x28) = puVar3;
    (param_1 + 0x2a) = paVar9;
    pass1_1020_6746(CONCAT22(param_2, param_1), 0x1, 0x4);
    puVar10 = mixed_1010_20ba(
        paVar9,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffee, 0x29),
        in_stack_0000fe96,
        in_stack_0000ffba,
        in_stack_0000ffc0,
        in_stack_0000ffc4,
    );
    paVar7 = (paVar9 & 0xffff0000 | puVar10 >> 0x10);
    (param_1 + 0x14) = puVar10;
    uVar6 = (puVar10 >> 0x10);
    (param_1 + 0x16) = uVar6;
    uVar13 = 0;
    uVar12 = (param_1 + 0x14);
    ppcVar2 = ((param_1 + 0x14) + 0x4);
    iVar14 = param_1;
    uVar15 = param_2;
    (**ppcVar2)();
    (param_1 + 0x6) = (param_1 + 0x14);
    uVar4 = (param_1 + 0x14);
    puVar1 = (uVar4 + 0xa);
    uVar4 = CONCAT22(param_2, param_1 + 0xa);
    uVar11 = SUB42(puVar1, 0x0);
    ppcVar2 = (*puVar1 + 0x8);
    (**ppcVar2)(
        0x1010,
        uVar11,
        (puVar1 >> 0x10),
        uVar4,
        uVar12,
        uVar6,
        uVar13,
        iVar14,
        uVar15,
    );
    (param_1 + 0x12) = uVar4;
    (param_1 + 0x10) = 0x1;
    puVar10 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT22(uVar11, 0x2),
        in_stack_0000fe84,
        in_stack_0000ffa8,
        in_stack_0000ffae,
        in_stack_0000ffb2,
    );
    (param_1 + 0x2c) = puVar10;
    (param_1 + 0x2e) = (puVar10 >> 0x10);
    return;
}

pub unsafe fn pass1_1020_6466(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x67c2;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1ea6(iVar1.field12_0x14, (param_1 & 0xffff | uVar1 << 0x10));
    }
    palette_op_1020_92c4(param_1);
    return;
}

pub unsafe fn pass1_1020_6498(mut param_1: u32, mut param_2: i16) -> u32 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 0x4) != 0) {
        uVar1 = (param_1 + 0x18 + param_2 * 0x4);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0xa), (iVar2 + 0x8));
    }
    return 0x0;
}

pub unsafe fn pass1_1020_64d4(mut param_1: u32, mut param_2: i16) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 0x4) != 0) {
        uVar1 = (param_1 + 0x18 + param_2 * 0x4);
        return (uVar1 + 0x4);
    }
    return 0x0;
}

pub unsafe fn pass1_1020_6746(mut param_1: u32, mut param_2: i16, mut param_3: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;

    if (param_3 != 0) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        if ((iVar3 + 0x18 + param_3 * 0x4) != 0) {
            uVar2 = (iVar3 + 0x18 + param_3 * 0x4);
            (uVar2 + 0x4) = param_2;
            (iVar3 + 0x10) = 0x1;
            if (param_2 == 0) {
                ppcVar1 = ((iVar3 + 0x18 + param_3 * 0x4) + 0x14);
                (**ppcVar1)();
            }
        }
    }
    return;
}


pub unsafe fn unk_win_ui_op_1020_67ce(
    in_struct_1: *mut StructA,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut HVar1: HGDIOBJ16;
    let mut hcursor_2: HCURSOR16;
    let struct_a_1_lo: *mut StructA;
    let struct_a_1_hi: *mut StructA;
    let mut in_stack_0000fe10: u16;
    let mut in_stack_0000fe14: u16;
    let mut in_stack_0000fe66: u16;
    let mut in_stack_0000ff3a: u16;
    let mut in_stack_0000ff3e: u16;
    let mut in_stack_0000ff42: u16;
    let mut in_stack_0000ff82: u16;
    let mut in_stack_0000ff8a: u16;
    let mut in_stack_0000ff90: u16;
    let mut in_stack_0000ff94: u16;

    struct_1020_790e(
        &in_struct_1.field0_0x0,
        s_TPPOPMENU_1050_43fa,
        param_2,
        param_3,
    );
    struct_a_1_hi = (in_struct_1 >> 0x10);
    struct_a_1_lo = in_struct_1;
    struct_a_1_lo[0x1].field20_0x26 = 0;
    struct_a_1_lo[0x1].field22_0x2a = 0;
    in_struct_1.field0_0x0 = 0x70e6;
    struct_a_1_lo.field1_0x2 = 0x1020;
    unk_str_op_1000_3d3e(
        (in_struct_1 & 0xffff0000 | ZEXT24(&struct_a_1_lo.field60_0x5b)),
        s_VrMode2_1050_4404,
    );
    HVar1 = GetStockObject16(HOLLOW_BRUSH);
    struct_a_1_lo.field157_0xc6 = HVar1;
    hcursor_2 = LoadCursor16(0x7f00, 0x0);
    struct_a_1_lo.field156_0xc4 = hcursor_2;
    struct_a_1_lo.field140_0xac = 0x44c00000;
    struct_a_1_lo.field158_0xc8 = 0x2020;
    struct_a_1_lo.field149_0xbc = (param_3 + 0x8);
    struct_a_1_lo.field159_0xca = param_2;
    win_ui_reg_class_1008_96d2(in_struct_1);
    window_op_1020_6c3a(
        param_4,
        in_struct_1,
        in_stack_0000fe66,
        in_stack_0000ff82,
        in_stack_0000ff8a,
        in_stack_0000ff90,
        in_stack_0000ff94,
        in_stack_0000fe10,
        in_stack_0000fe14,
        in_stack_0000ff3a,
        in_stack_0000ff3e,
        in_stack_0000ff42,
    );
    return;
}


// WARNING: Unable to use type for symbol uVar2

pub unsafe fn pass1_1020_68de(mut param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0xf6) != 0) {
        invalidate_rect_1020_735a((param_1 + 0xf6));
    }
    return;
}





pub unsafe fn destroy_icon_1020_6bd2(param_1: *mut astruct_868, param_2: u8) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut struct_1: *mut astruct_868;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    struct_1 = param_1;
    DestroyIcon16(struct_1.hicon_0xc2);
    struct_1.hicon_0xc2 = 0;
    struct_1.field8_0x8 = 0;
    puVar1 = struct_1.field241_0xf6;
    uVar2 = struct_1.field242_0xf8;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(s_tile2_bmp_1050_1538, puVar1, uVar2, 1);
    }
    struct_1.field241_0xf6 = 0;
    pass1_1010_1dda(struct_1.field240_0xf2);
    struct_1.field240_0xf2 = 0;
    return;
}


pub unsafe fn window_op_1020_6c3a(
    param_1: *mut Struct57,
    struct_param_1: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
    mut param_9: u16,
    mut param_10: u16,
    mut param_11: u16,
    mut param_12: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut HVar3: HICON16;
    let mut paVar4: *mut Struct57;
    let mut pIVar5: *mut INT16 = null_mut();
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut paVar9: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar11: *mut u32;
    let mut uVar12: u8;
    let mut local_6: u32;
    let struct_a_1: *mut StructA;
    let mut paVar10: *mut Struct57;
    let mut struct_a_1_hi: u16;

    struct_a_1 = struct_param_1;
    struct_a_1_hi = (struct_param_1 >> 0x10);
    create_window_ex_1008_9760(struct_param_1);
    puVar11 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x4),
        param_3,
        param_5,
        param_6,
        param_7,
    );
    paVar9 = (param_1 & 0xffff0000 | puVar11 >> 0x10);
    struct_a_1[0x1].field20_0x26 = puVar11;
    uVar7 = (puVar11 >> 0x10);
    struct_a_1[0x1].field21_0x28 = uVar7;
    struct_a_1[0x1].field10_0x14 = struct_a_1[0x1].field20_0x26;
    struct_a_1[0x1].field11_0x16 = uVar7;
    HVar3 = LoadIcon16(s_TILEICON_1050_440c, HINSTANCE16_1050_038c);
    struct_a_1.field_0xc2 = HVar3;
    uVar6 = &struct_a_1[0x1].field20_0x26;
    ppcVar2 = (*&struct_a_1[0x1].field20_0x26 + 0x30);
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar6, (uVar6 >> 0x10), HVar3);
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbb8,
    );
    mem_op_1000_179c(0x42, paVar9);
    uVar8 = paVar9 | paVar4;
    paVar10 = (paVar9 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar9,
            0x0,
            local_6,
            0x0,
            0x7c007d,
            CONCAT22(struct_a_1.field4_0x8, 0xbb8),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbb9,
    );
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar9 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar9,
            paVar4,
            paVar10,
            0x0,
            local_6,
            0x0,
            0x7e007f,
            CONCAT22(struct_a_1.field4_0x8, 0xbb9),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbba,
    );
    mem_op_1000_179c(0x42, paVar9);
    uVar8 = paVar9 | paVar4;
    paVar10 = (paVar9 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar9,
            0x0,
            local_6,
            0x1b2,
            0x1b001b1,
            CONCAT22(struct_a_1.field4_0x8, 0xbba),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    mem_op_1000_179c(0x22, paVar10);
    uVar8 = paVar10 | paVar4;
    if (uVar8 == 0) {
        struct_a_1[0x1].field22_0x2a = 0;
    } else {
        unk_win_ui_op_1020_717e(uVar8, param_5, CONCAT22(paVar10, paVar4), struct_param_1);
        struct_a_1[0x1].field22_0x2a = paVar4;
        struct_a_1[0x1].field_0x2c = uVar8;
    }
    uVar6 = &struct_a_1[0x1].field22_0x2a;
    struct_a_1[0x1].field14_0x1c = uVar6;
    uVar1 = &struct_a_1[0x1].field20_0x26;
    ppcVar2 = (*&struct_a_1[0x1].field20_0x26 + 0x10);
    (**ppcVar2)(0x1000, uVar1, (uVar1 >> 0x10));
    pIVar5 = uVar6;
    MoveWindow16(
        0x1,
        pIVar5[0x3],
        pIVar5[0x2],
        pIVar5[0x1],
        *pIVar5,
        struct_a_1.field4_0x8,
    );
    uVar12 = (struct_param_1 >> 0x10);
    uVar6 = struct_param_1;
    ppcVar2 = (uVar6 + 0x94);
    (**ppcVar2)(s_tile2_bmp_1050_1538, struct_a_1, uVar12, 0x0);
    ppcVar2 = (uVar6 + 0x10);
    (**ppcVar2)(s_tile2_bmp_1050_1538, struct_a_1, uVar12, 1);
    UpdateWindow16(struct_a_1.field4_0x8);
    return;
}
