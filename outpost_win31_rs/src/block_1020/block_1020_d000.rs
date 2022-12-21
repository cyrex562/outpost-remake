pub unsafe fn struct_1020_d06c(param_1: *mut u16) -> *mut u16 {
    struct_1028_b354(param_1);
    *param_1 = 0xd314;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

pub unsafe fn pass1_1020_d08e(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0xd314;
    (param_2 + 0x2) = 0x1020;
    return &param_2.field0_0x0;
}



pub unsafe fn pass1_1020_d194(param_1: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut in_EDX: u32;
    let mut paVar13: *mut Struct57;
    let mut uVar14: u16;
    let mut uVar15: u32;
    let mut puVar16: *mut u32;
    let mut puVar17: *mut u32;
    let mut in_stack_0000fe70: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ff9e: u16;
    let mut uVar18: u8;
    let mut uVar19: u8;
    let mut uVar20: u16;
    let mut uVar21: u16;
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut puStack34: *mut u32;
    let mut local_4: [u8; 0x2] = [0; 0x2];
    let mut paVar12: *mut Struct57;

    uVar9 = (in_EDX >> 0x10);
    pass1_1030_bcae(local_4, &DAT_1050_1050);
    uVar15 = pass1_1028_b4f2(param_1);
    uVar8 = (uVar15 >> 0x10);
    paVar12 = CONCAT22(uVar9, uVar8);
    uVar7 = (uVar15 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7);
    uVar8 = uVar7;
    paVar13 = paVar12;
    pass1_1028_b58e(param_1);
    uVar9 = SUB42(paVar13, 0x0);
    puVar2 = local_4;
    pass1_1030_bd74(
        puVar2,
        &DAT_1050_1050,
        uVar7 & 0xffff | paVar12 << 0x10,
        CONCAT22(uVar9, uVar8),
    );
    if (puVar2 < 0x0) {
        return;
    }
    if (0x1e < puVar2) {
        uVar3 = 0x87;
        puVar16 = mixed_1010_20ba(
            paVar13,
            _u16_1050_0ed0,
            0x870009,
            in_stack_0000fe70,
            in_stack_0000ff94,
            in_stack_0000ff9a,
            in_stack_0000ff9e,
        );
        uVar10 = (paVar13 >> 0x10);
        uVar3 = pass1_1010_65d0(puVar16, uVar3);
        if (uVar3 == 0) {
            puVar17 = pass1_1008_c6fa(_u16_1050_06e0, 0x15);
            paVar12 = CONCAT22(uVar10, (puVar17 >> 0x10));
            uVar4 = puVar17;
            uVar14 = SUB42(&u16_1050_1038, 0x0);
            pass1_1038_4e78(uVar4, paVar12, uVar15, puVar17);
            uVar10 = SUB42(paVar12, 0x0);
            puStack34 = CONCAT22(uVar10, uVar4);
            ppcVar1 = (*puStack34 + 0x10);
            paVar13 = paVar12;
            uVar5 = uVar4;
            uVar21 = uVar4;
            (**ppcVar1)(&u16_1050_1038, uVar4, uVar10);
            uStack38 = CONCAT22(paVar13, uVar5);
            uStack42 = 0;
            loop {
                if (uStack38 <= uStack42) {
                    if (puStack34.is_null()) {
                        return;
                    }
                    ppcVar1 = *puStack34;
                    (**ppcVar1)(
                        uVar14, uVar4, paVar12, 0x1, uVar21, uVar10, puStack34, puStack34,
                    );
                    return;
                }
                uVar18 = uVar8;
                uVar19 = (uVar8 >> 0x8);
                uVar15 = uStack38;
                uVar20 = uVar9;
                pass1_1030_1d58(puStack34);
                uVar6 = uVar15;
                uVar11 = SUB42(paVar13, 0x0);
                puVar2 = local_4;
                uVar14 = 0x1030;
                pass1_1030_bd74(
                    puVar2,
                    &DAT_1050_1050,
                    uVar15 & 0xffff | paVar13 << 0x10,
                    CONCAT22(uVar20, CONCAT11(uVar19, uVar18)),
                );
                if ((0x0 < puVar2) && (puVar2 < 0x1f)) {
                    break;
                }
                uStack42 += 0x1;
            }
            if (puStack34.is_null()) {
                return;
            }
            ppcVar1 = *puStack34;
            (**ppcVar1)(
                0x1030, uVar4, paVar12, 0x1, uVar21, uVar10, puStack34, puStack34, uVar6, uVar11,
            );
            return;
        }
    }
    return;
}


pub unsafe fn struct_1020_d37c(param_1: *mut astruct_180) -> *mut u16 {
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field0_0x0 = 0xd53e;
    (param_1 + 0x2) = 0x1020;
    return &param_1.field0_0x0;
}

pub unsafe fn pass1_1020_d3a4(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u32,
) -> *mut u16 {
    let mut uVar1: u16;

    pass1_1028_b39e(param_1, param_2, param_4, param_5);
    uVar1 = (param_2 >> 0x10);
    (param_2 + 0x20) = param_3;
    param_2.field0_0x0 = 0xd53e;
    (param_2 + 0x2) = 0x1020;
    return &param_2.field0_0x0;
}








pub unsafe fn struct_1020_d5a6(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0xd7fe;
    (param_1 + 0x2) = 0x1020;
    return &param_1.field0_0x0;
}

pub unsafe fn pass1_1020_d5c8(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0xd7fe;
    (param_2 + 0x2) = 0x1020;
    return &param_2.field0_0x0;
}










pub unsafe fn struct_1020_d866(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0xd8ec;
    (param_1 + 0x2) = 0x1020;
    return &param_1.field0_0x0;
}

pub unsafe fn pass1_1020_d888(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0xd8ec;
    (param_2 + 0x2) = 0x1020;
    return &param_2.field0_0x0;
}



pub unsafe fn struct_1020_d954(param_1: *mut astruct_180) {
    let mut in_EDX: *mut Struct57;
    let mut iVar1: *mut astruct_180;
    let mut unaff_BP: u16;
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    struct_1030_dc96(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1[0x1].field_0x4 = 0;
    iVar1[0x1].field_0x6 = 0;
    iVar1[0x1].field_0x8 = 0;
    param_1.field0_0x0 = 0xe792;
    iVar1.field1_0x2 = 0x1020;
    puVar2 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x2f),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    iVar1[0x1].field_0x8 = puVar2;
    iVar1[0x1].field_0xa = (puVar2 >> 0x10);
    return;
}



pub unsafe fn struct_1020_d99e(
    param_1: *mut Struct57,
    param_2: *mut astruct_12,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u32,
) -> *mut u16 {
    let mut iVar1: *mut astruct_12;
    let mut unaff_BP: u16;
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    puVar2 = pass1_1030_dcc2(param_1, param_2, param_4, param_5);
    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    iVar1.field23_0x24 = param_3;
    (iVar1 + 1).field0_0x0 = 0;
    iVar1[0x1].field1_0x2 = 0;
    param_2.field0_0x0 = 0xe792;
    iVar1.field1_0x2 = 0x1020;
    puVar3 = mixed_1010_20ba(
        (param_1 & 0xffff0000 | puVar2 >> 0x10),
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x2f),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    iVar1[0x1].field1_0x2 = puVar3;
    iVar1[0x1].field_0x4 = (puVar3 >> 0x10);
    iVar1.field12_0x10 = 0x49;
    return &param_2.field0_0x0;
}







pub unsafe fn pass1_1020_dc1c(param_1: *mut astruct_15, param_2: *mut u16) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut puVar8: *mut u32;
    let mut bStack27: u8;
    let mut local_a: [u8; 0x4] = [0; 0x4];
    let mut uStack6: u32;

    puVar8 = CONCAT22(0x1050, local_a);
    uVar6 = pass1_1028_bb24(param_1);
    uVar5 = (uVar6 >> 0x10);
    puVar3 = uVar6;
    pass1_1030_64ce(
        puVar3,
        uVar5,
        _PTR_LOOP_1050_5740,
        param_2,
        uVar6 & 0xffff | uVar5 << 0x10,
        puVar8,
    );
    uStack6 = *puVar3;
    uVar5 = (puVar3 + 2);
    bStack27 = (uStack6 >> 0x18);
    uVar4 = bStack27;
    if (bStack27 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack6 & 0xffff | uVar5 << 0x10);
        puVar7 = struct_op_1030_73a8(CONCAT22(uVar5, uVar4), uVar4, uVar5);
        iVar1 = (puVar7 + 0xc);
        if (((iVar1 < 1) || (SBORROW2(iVar1, 1)))
            || (iVar1 != 0x9
                && 0x7 < iVar1 -0x1
                && (iVar1 -0x9 < 0x6a || (0x6 < iVar1 -0x73))))
        {
            ppcVar2 = (*puVar7 + 0x24);
            (**ppcVar2)();
        }
    }
    return;
}


pub unsafe fn pass1_1020_dca8(mut param_1: u16, param_2: *mut astruct_15) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut local_2e: [u8; 0xe] = [0; 0xe];
    let mut puStack32: *mut u32;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut uStack26: u16;
    let mut uStack24: u16;
    let mut uStack22: u16;
    let mut uStack20: u16;
    let mut uStack18: u16;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut uStack10: u32;
    let mut local_6: [u8; 0x2] = [0; 0x2];
    let mut local_4: i16;

    pass1_1028_c1f8(
        local_6,
        param_1,
        param_2,
        CONCAT22(0x1050, local_6),
        CONCAT22(0x1050, &local_4),
    );
    uStack10 = pass1_1028_b58e(param_2);
    uVar1 = (uStack10 >> 0x10);
    local_10 = (uStack10 + 0xc);
    uStack12 = (uStack10 + 0x10);
    puStack32 = &local_10;
    uStack18 = local_10;
    uStack20 = (local_10 >> 0x10);
    uStack24 = local_10 - 0x1;
    if (uStack24 < 0x0) {
        uStack24 = 0;
    }
    uVar2 = local_4 - 0x1;
    uStack26 = local_10 + 1;
    if (uVar2 < (local_10 + 1)) {
        uStack26 = uVar2;
    }
    uStack28 = uStack20 - 0x1;
    if (uStack28 < 0x0) {
        uStack28 = 0;
    }
    uStack30 = uStack20 + 1;
    if (uVar2 < (uStack20 + 1)) {
        uStack30 = uVar2;
    }
    uStack22 = uStack12;
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack12, uStack28, uStack24);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack22, uStack28, uStack18);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack22, uStack28, uStack26);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack22, uStack20, uStack24);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack22, uStack20, uStack26);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack22, uStack30, uStack24);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack22, uStack30, uStack18);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    pass1_1008_3e54(CONCAT22(0x1050, local_2e), uStack22, uStack30, uStack26);
    pass1_1020_dc1c(param_2, CONCAT22(0x1050, local_2e));
    return;
}


pub unsafe fn pass1_1020_de32(param_1: *mut u8, mut param_2: u32, mut param_3: u16) {
    let mut uVar1: u16;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000fff0: u16;

    puVar5 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff0, 0x5),
        in_stack_0000fe98,
        in_stack_0000ffbc,
        in_stack_0000ffc2,
        in_stack_0000ffc6,
    );
    uVar1 = (puVar5 >> 0x10);
    (puVar5 + 0x12) = param_3;
    uVar3 = uVar1;
    BVar2 = bring_win_to_top_1038_b72e(_PTR_LOOP_1050_5b7c, 0x4);
    if (BVar2 == 0) {
        pass1_1038_af40(
            _PTR_LOOP_1050_4230,
            uVar3,
            _PTR_LOOP_1050_5b7c,
            (_PTR_LOOP_1050_4230 + 0x16),
            0x4,
        );
    }
    PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 1);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5b80);
    uVar4 = (param_2 >> 0x10);
    (param_2 + 0x24) = (puVar5 + 0xa);
    if ((param_2 + 0x24) == 0) {
        PTR_LOOP_1050_50ca = 0x6b2;
    }
    return;
}

pub unsafe fn pass1_1020_deac(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    param_4: *mut u16,
    param_5: i32,
) -> BOOL16 {
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar1 = param_3;
    uVar2 = (param_3 >> 0x10);
    pass1_1028_c7b6(param_2, uVar1, uVar2, param_4, param_5);
    if (param_1 < 1) {
        return 0x0;
    }
    if (SBORROW2(param_1, 1)) {
        return 0x0;
    }
    if (param_1 != 0x3 && 0x0 < param_1 -0x2) {
        if (param_1 == 0x4) {
            pass1_1020_de32(param_2, param_3, 0x4);
            if ((uVar1 + 0x24) == 0x6) {
                return 0x1;
            }
            return 0x0;
        }
        if (param_1 != 0x5) {
            return 0x0;
        }
    }
    (uVar1 + 0x24) = 0x1;
    return 0x1;
}


pub unsafe fn pass1_1020_df10(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u32,
    param_4: *mut u16,
    param_5: i32,
) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut bStack31: u8;
    let mut local_e: u32;
    let mut uStack10: u32;
    let mut uStack6: u16;
    let mut uStack4: u16;

    uStack4 = 0;
    uVar6 = param_3;
    uVar7 = (param_3 >> 0x10);
    pass1_1028_c7b6(param_2, uVar6, uVar7, param_4, param_5);
    uStack6 = param_1;
    if (param_1 == 0) {
        puVar1 = &local_e;
        pass1_1030_64ce(
            puVar1,
            param_2,
            _PTR_LOOP_1050_5740,
            param_4,
            param_5,
            CONCAT22(0x1050, puVar1),
        );
        uStack10 = *puVar1;
        uVar4 = (puVar1 + 2);
        bStack31 = (uStack10 >> 0x18);
        uVar2 = bStack31;
        if (bStack31 != 0) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack10 & 0xffff | uVar4 << 0x10);
            uVar5 = struct_op_1030_73a8(CONCAT22(uVar4, uVar2), uVar2, uVar4);
            if ((uVar5 + 0xc) == 0x6a) {
                BVar3 = pass1_1020_e044(param_3);
                if (BVar3 == 0) {
                    (uVar6 + 0x24) = 0x1;
                } else {
                    PTR_LOOP_1050_50ca = 0x6ac;
                }
            }
        }
    } else if (((0x5 < param_1) && (!SBORROW2(param_1, 0x6))) && ((param_1 - 0x6) < 0x4)) {
        pass1_1020_de32(param_2, param_3, param_1);
        match (uVar6 + 0x24) {
            0x1 => {
                BVar3 = pass1_1020_e044(param_3);
                if (BVar3 != 0) {
                    PTR_LOOP_1050_50ca = 0x6ac;
                }
            }
            //   break;
            0x2 | 0x3 | 0x4 | 0x5 => {
                pass1_1020_e652(param_3, param_4, (param_4 >> 0x10), param_5);
            }
        }
    }
    return;
}
