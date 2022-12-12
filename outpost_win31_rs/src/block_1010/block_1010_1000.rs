pub unsafe fn pass1_1010_1146(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    DAT_1050_0ecc = param_2;
    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x66);
    pass1_1000_4aea(
        (param_1 + 0x64),
        uVar1,
        (uVar1 >> 0x10),
        0x4,
        (s_dibtext_bmp_1050_1844 + 0x6),
    );
    return;
}
pub unsafe fn pass1_1010_116c(param_1: u32, mut param_2: i16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut in_EDX: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uStack4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x56) != 0) {
        ppcVar1 = (*param_1 + 0x34);
        (**ppcVar1)();
    }
    ppcVar1 = (*param_1 + 0x28);
    iVar2 = (**ppcVar1)();
    if (iVar2 != 0) {
        uStack4 = DAT_1050_0ecc;
        iVar2 = DAT_1050_0ecc + 1;
        if (iVar2 == 0) {
            uStack4 = 0;
        }
        pass1_1010_1146(param_1, uStack4);
        pass1_1010_11c6(iVar2, in_EDX, param_1);
        (iVar3 + 0x56) = iVar2;
        (iVar3 + 0x58) = in_EDX;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1010_11c6(
    mut param_1: u16,
    param_2: *mut astruct_57,
    param_3: *mut astruct_234,
) {
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut iVar6: *mut astruct_239;
    let mut iVar5: i16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut paVar15: *mut Struct57;
    let mut uVar16: u32;
    let mut puVar17: *mut u32;
    let mut iVar18: *mut astruct_234;
    let mut iVar19: i16;
    let mut iVar21: i16;
    let mut iVar20: *mut astruct_238;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut paVar24: *mut astruct_223;
    let mut paVar25: *mut Struct57;
    let mut puStack50: *mut u32;
    let mut iStack42: i16;
    let mut iStack40: i16;
    let mut pSStack38: *mut StructD;
    let mut iStack28: i16;
    let mut puStack26: *mut u32;
    let mut puStack22: *mut u32;
    let mut uStack14: u32;
    let mut uStack10: u32;

    if (DAT_1050_0ecc == -1) {
        return;
    }
    mem_op_1000_179c(0x1a, param_2);
    paVar15 = (param_2 & 0xffff0000);
    if ((param_2 | param_1) == 0) {
        iVar6 = null_mut();
    } else {
        paVar24 = pass1_1010_37d4(CONCAT22(param_2, param_1));
        paVar15 = (paVar15 & 0xffff0000 | paVar24 >> 0x10);
        iVar6 = paVar24;
    }
    uVar11 = SUB42(paVar15, 0x0);
    uStack10 = 0x10500ece;
    uStack14 = 0;
    loop {
        uVar22 = (param_3 >> 0x10);
        iVar18 = param_3;
        piVar1 = &iVar18.field101_0x68;
        if (*piVar1 == uStack14 || *piVar1 < uStack14) {
            break;
        }
        uVar4 = iVar18.field100_0x64;
        uVar16 = (uVar4 + uStack14 * 0x4);
        puVar17 = (uVar16 + DAT_1050_0ecc * 0x8);
        puStack50 = (uVar16 & 0xffff0000 | ZEXT24(puVar17));
        iVar5 = pass1_1000_475e(uStack10, *puVar17);
        if (iVar5 != 0) {
            uStack10 = *puStack50;
            uStack14 = uStack14 & 0xffff | (uStack14 + 1) << 0x10;
        }
        uStack14 = uStack14 & 0xffff0000 | (uStack14 + 1);
    }
    iVar6.field13_0x10 = uStack14;
    paVar25 = struct_1010_38f8(uStack14, paVar15, CONCAT22(uVar11, iVar6), uStack14);
    paVar15 = (paVar15 & 0xffff0000 | paVar25 >> 0x10);
    iVar7 = 0;
    mem_op_1000_179c(0x400, paVar15);
    uVar12 = SUB42(paVar15, 0x0);
    iVar5 = iVar7;
    mem_op_1000_179c(0x400, paVar15);
    uVar13 = SUB42(paVar15, 0x0);
    pSStack38 = CONCAT22(uVar13, iVar5);
    iStack28 = 0;
    pass1_1000_4906(CONCAT22(uVar12, iVar7), NULL, 0x400);
    pass1_1000_4906(CONCAT22(uVar13, iVar5), NULL, 0x400);
    iStack42 = 0;
    uVar10 = 0;
    loop {
        puVar2 = &iVar6.field13_0x10;
        if (*puVar2 == uVar10 || *puVar2 < uVar10) {
            return;
        }
        uVar4 = iVar18.field100_0x64;
        uVar23 = (uVar4 >> 0x10);
        iVar19 = uVar4;
        iVar21 = (iVar19 + iStack28 * 0x4);
        uVar9 = (iVar19 + iStack28 * 0x4 + 2);
        paVar25 = (paVar15 & 0xffff0000 | uVar9);
        iVar19 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
        puStack22 = CONCAT22(uVar9, iVar19);
        uVar8 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
        mem_op_1000_179c(0x1a, paVar25);
        uVar14 = paVar25 | uVar8;
        paVar15 = (paVar25 & 0xffff0000 | uVar14);
        if (uVar14 == 0) {
            uVar4 = iVar6.field8_0x8;
            (uVar4 + uVar10 * 0x4) = 0;
        } else {
            paVar24 = pass1_1010_37d4(CONCAT22(paVar25, uVar8));
            paVar15 = (paVar15 & 0xffff0000 | paVar24 >> 0x10);
            uVar4 = iVar6.field8_0x8;
            uVar23 = (uVar4 >> 0x10);
            iVar21 = uVar4;
            (iVar21 + uVar10 * 0x4) = paVar24;
            (iVar21 + uVar10 * 0x4 + 0x2) = (paVar24 >> 0x10);
        }
        iStack42 += 0x1;
        uVar4 = iVar6.field8_0x8;
        uVar23 = (uVar4 >> 0x10);
        iVar21 = uVar4;
        uVar4 = (iVar21 + uVar10 * 0x4);
        ppcVar3 = ((iVar21 + uVar10 * 0x4) + 0x1c);
        (**ppcVar3)(0x1000, uVar4, (uVar4 >> 0x10), iStack42, iVar19, uVar9);
        uStack14 = uVar10;
        loop {
            piVar1 = &iVar18.field101_0x68;
            if (*piVar1 == iStack28 || *piVar1 < iStack28) {
                break;
            }
            iVar19 = iStack28 * 0x4;
            uVar4 = iVar18.field100_0x64;
            uVar4 = (uVar4 + iVar19);
            iVar21 = pass1_1000_475e(*puStack22, (uVar4 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
            if (iVar21 != 0) {
                break;
            }
            uVar4 = iVar18.field100_0x64;
            uVar23 = (uVar4 >> 0x10);
            iVar21 = uVar4;
            uVar10 = (iVar21 + iVar19 + 2);
            paVar15 = (paVar15 & 0xffff0000 | uVar10);
            uVar9 = (iVar21 + iVar19) + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
            puStack26 = CONCAT22(uVar10, uVar9);
            mem_op_1000_179c(0x1a, paVar15);
            uVar16 = paVar15 & 0xffff0000;
            if ((paVar15 | uVar9) == 0) {
                uVar23 = 0;
            } else {
                paVar24 = pass1_1010_37d4(CONCAT22(paVar15, uVar9));
                uVar16 = uVar16 & 0xffff0000 | paVar24 >> 0x10;
                uVar23 = SUB42(paVar24, 0x0);
            }
            (uStack14 * 0x4 + iVar7) = uVar23;
            (uStack14 * 0x4 + iVar7 + 0x2) = uVar16;
            uVar4 = iVar18.field100_0x64;
            uVar23 = (uVar4 >> 0x10);
            iVar21 = uVar4;
            uVar10 = (iVar21 + iStack28 * 0x4 + 2);
            paVar15 = (uVar16 & 0xffff0000 | uVar10);
            iStack42 += 0x1;
            uVar4 = (uStack14 * 0x4 + iVar7);
            ppcVar3 = ((uStack14 * 0x4 + iVar7) + 0x1c);
            (**ppcVar3)(
                0x1000,
                uVar4,
                (uVar4 >> 0x10),
                iStack42,
                (iVar21 + iStack28 * 0x4) + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8,
                uVar10,
            );
            iStack40 = 0;
            loop {
                piVar1 = &iVar18.field101_0x68;
                if (*piVar1 == iStack28 || *piVar1 < iStack28) {
                    break;
                }
                uVar4 = iVar18.field100_0x64;
                uVar4 = (uVar4 + iStack28 * 0x4);
                iVar21 = pass1_1000_475e(*puStack26, (uVar4 + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8));
                if (iVar21 != 0) {
                    break;
                }
                uVar4 = iVar18.field100_0x64;
                uVar4 = (uVar4 + iStack28 * 0x4);
                uVar10 = pass1_1000_475e(*puStack22, (uVar4 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
                if (uVar10 != 0) {
                    break;
                }
                mem_op_1000_179c(0x1a, paVar15);
                uVar16 = paVar15 & 0xffff0000;
                if ((paVar15 | uVar10) == 0) {
                    uVar23 = 0;
                } else {
                    paVar24 = pass1_1010_37d4(CONCAT22(paVar15, uVar10));
                    uVar16 = uVar16 & 0xffff0000 | paVar24 >> 0x10;
                    uVar23 = SUB42(paVar24, 0x0);
                }
                (iStack40 * 0x4 + iVar5) = uVar23;
                (iStack40 * 0x4 + iVar5 + 0x2) = uVar16;
                uVar4 = iVar18.field100_0x64;
                uVar23 = (uVar4 >> 0x10);
                iVar20 = uVar4;
                uVar10 = (iVar20 + iStack28 * 0x4 + 2);
                paVar15 = (uVar16 & 0xffff0000 | uVar10);
                iStack42 += 0x1;
                uVar4 = (iStack40 * 0x4 + iVar5);
                ppcVar3 = ((iStack40 * 0x4 + iVar5) + 0x1c);
                (**ppcVar3)(
                    0x1000,
                    uVar4,
                    (uVar4 >> 0x10),
                    iStack42,
                    (iVar20 + iStack28 * 0x4) + (DAT_1050_0ecc * 0x6 + 0xebe) * 0x8,
                    uVar10,
                );
                iStack28 += 0x1;
                iStack40 += 0x1;
            }
            uVar4 = (uStack14 * 0x4 + iVar7);
            (uVar4 + 0x10) = iStack40;
            uVar10 = iStack40 << 0x2;
            iVar21 = iVar5;
            uVar23 = uVar13;
            paVar25 = struct_1010_38f8(uVar10, paVar15, (uStack14 * 0x4 + iVar7), iStack40);
            paVar15 = (paVar15 & 0xffff0000 | paVar25 >> 0x10);
            pass1_1000_48a8(paVar25, CONCAT22(uVar23, iVar21), uVar10);
            pass1_1000_4906(pSStack38, NULL, 0x400);
            uStack14 = uStack14 & 0xffff | (uStack14 + 1) << 0x10;
        }
        uVar4 = iVar6.field8_0x8;
        uVar4 = (uVar4 + uStack14 * 0x4);
        (uVar4 + 0x10) = uStack14;
        uVar10 = uStack14 << 0x2;
        uVar4 = iVar6.field8_0x8;
        iVar21 = iVar7;
        uVar23 = uVar12;
        paVar25 = struct_1010_38f8(uVar10, paVar15, (uVar4 + uStack14 * 0x4), uStack14);
        paVar15 = (paVar15 & 0xffff0000);
        pass1_1000_48a8(paVar25, CONCAT22(uVar23, iVar21), uVar10);
        pass1_1000_4906(CONCAT22(uVar12, iVar7), NULL, 0x400);
        uVar10 = uStack14 + 1;
    }
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_1656(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u16,
    param_4: *mut astruct_27,
    mut param_5: u16,
    mut param_6: u32,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut paVar9: *mut astruct_15;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    unk_destroy_win_op_1010_305a(param_1, param_4, param_5, param_6);
    if ((param_4 + 0x16) == 0x3) {
        paVar8 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(param_3, 0x32),
            in_stack_0000fe88,
            in_stack_0000ffac,
            in_stack_0000ffb2,
            in_stack_0000ffb6,
        );
        uVar5 = paVar4 & 0xffff0000;
        uVar1 = (param_4 + 0x32);
        uVar1 = (uVar1 + 0x42);
        uVar7 = (uVar1 >> 0x10);
        iVar6 = uVar1;
        uVar1 = (iVar6 + 0x16);
        paVar9 = struct_op_1030_73a8((uVar1 + 0x4), iVar6, (paVar8 >> 0x10));
        uVar5 = uVar5 & 0xffff0000 | paVar9 >> 0x10;
        uVar2 = pass1_1010_7818(paVar8, paVar9);
        uVar1 = (iVar6 + 0x16);
        uVar3 = uVar2;
        ui_op_1010_79aa(paVar8, 0x0, (uVar1 + 0x4));
        if (uVar3 == 0) {
            uVar1 = (iVar6 + 0x16);
            unk_win_op_1010_7300(uVar5, paVar8, 0x0, uVar2, (uVar1 + 0x4));
        }
    }
    return;
}
pub unsafe fn pass1_1010_16ee(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut in_stack_0000ffc0: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_6);
    mem_op_1000_179c(0x4a, paVar2);
    uVar1 = paVar2 | param_5;
    if (uVar1 != 0) {
        pass1_1040_c54a(
            CONCAT22(paVar2, param_5),
            0x0,
            CONCAT22(param_4, param_3),
            in_stack_0000ffc0,
            paVar2 & 0xffff0000 | uVar1,
        );
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn string_1010_1722(mut param_1: u16, mut param_2: u16, mut param_3: u16, mut param_4: u32) {
    let mut extraout_DX: u16;
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut uVar3: u16;
    let mut local_52: [u8; 0x50] = [0; 0x50];

    pass1_1028_b58e(param_4);
    if ((extraout_DX | param_1) == 0) {
        pcVar2 = load_string_1010_847e(_u16_1050_14cc, 0x424);
        uVar1 = (pcVar2 >> 0x10);
        unk_str_op_1000_3d3e(CONCAT22(0x1050, local_52), pcVar2);
        pcVar2 = CONCAT22(uVar1, local_52);
        uVar3 = &DAT_1050_1050;
    } else {
        pcVar2 = pass1_1038_4d28(*(param_1 + 0x2e));
        uVar3 = (pcVar2 >> 0x10);
    }
    str_op_1008_60e8((pcVar2 >> 0x10), (pcVar2 & 0xffff | uVar3 << 0x10));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_1788(
    param_1: *mut c_char,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut astruct_15,
) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut puVar5: *mut u8;
    let mut in_stack_0000fff4: u32;
    u8 * *ppuVar6;
    let mut iVar7: i16;

    ppuVar6 = CONCAT22((in_stack_0000fff4 >> 0x10), 0x3);
    puVar4 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        ppuVar6,
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    uVar3 = (puVar4 >> 0x10);
    iVar7 = (ppuVar6 >> 0x10);
    puVar5 = 0x1778;
    uVar1 = pass1_1028_b58e(param_4);
    pcVar2 = pass1_1010_b038(puVar4, uVar1, uVar3, puVar5, iVar7);
    str_op_1008_60e8(uVar3, CONCAT22(uVar3, pcVar2));
    return;
}
pub unsafe fn pass1_1010_17c0(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut struct_1: *mut astruct_455;
    let mut struct_1_hi: *mut astruct_455;
    let mut fn_ptr_1: *mut *mut code;

    unk_destroy_win_op_1010_2fa0(param_1);
    struct_1_hi = (param_1 >> 0x10);
    struct_1 = param_1;
    puVar1 = struct_1[0xa].field3_0x6;
    uVar2 = (struct_1 + 0xb).field0_0x0;
    if ((uVar2 | puVar1) != 0) {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)();
    }
    struct_1[0xa].field3_0x6 = 0;
    fn_ptr_1000_17ce(*(struct_1 + 0xc));
    pass1_1000_4906(
        &struct_1[0xc].field2_0x4,
        NULL,
        (struct_1 + 0xd).field0_0x0 << 0x2,
    );
    fn_ptr_1000_17ce(*&struct_1[0xc].field2_0x4);
    (struct_1 + 0xc) = 0;
    struct_1[0xc].field2_0x4 = 0;
    return;
}
pub unsafe fn pass1_1010_184a(param_1: u32, param_2: *mut u32) {
    let mut iVar1: i16;
    let mut iVar2: i16;

    iVar2 = DAT_1050_0ecc;
    iVar1 = (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    iVar1 = pass1_1000_475e((iVar1 + *param_1), (iVar1 + *param_2));
    if (iVar1 == 0) {
        iVar1 = (iVar2 * 0x6 + 0xebc) * 0x8;
        iVar1 = pass1_1000_475e((iVar1 + *param_1), (iVar1 + *param_2));
        if (iVar1 == 0) {
            iVar2 = (iVar2 * 0x6 + 0xebe) * 0x8;
            pass1_1000_475e((iVar2 + *param_1), (iVar2 + *param_2));
        }
    }
    return;
}

pub unsafe fn FUN_1010_18e8() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1010_18ee() -> u16 {
    return 0x1;
}

pub unsafe fn pass1_1010_18f4(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_195e(
    param_1: *mut astruct_57,
    param_2: *mut astruct_19,
    param_3: *mut astruct_19,
    mut param_4: u16,
) -> *mut astruct_19 {
    let mut unaff_BP: u16;
    let mut puVar1: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    pass1_1010_0f24(param_1, CONCAT22(param_3, param_2), param_4);
    param_2[0x1].field_0xe = 0;
    CONCAT22(param_3, param_2) = 0x1b2a;
    param_2.segment_0x2 = 0x1010;
    puVar1 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x3),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    param_2[0x1].field_0xe = puVar1;
    param_2[0x1].field8_0x10 = (puVar1 >> 0x10);
    return CONCAT22(param_3, param_2);
}
pub unsafe fn pass1_1010_19a4(mut param_1: u16, param_2: *mut u32) {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_92;
    let mut extraout_DX: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        if ((param_1 | paVar2) == 0) {
            break;
        }
        ppcVar1 = (*param_2 + 0x40);
        (**ppcVar1)(0x1028, param_2);
        param_1 = extraout_DX;
    }
    return;
}

// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_1a06(
    mut param_1: u32,
    param_2: *mut astruct_15,
    mut param_3: i16,
    mut param_4: u16,
) {
    let mut pcVar1: *mut c_char;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar2: u16;
    let mut in_EDX: *mut Struct57;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar3: *mut u32;
    let mut pcVar4: *mut c_char;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffee: i16;
    let mut uVar1: u32;

    uVar6 = pass1_1028_b58e(param_2);
    uVar5 = (param_1 >> 0x10);
    pcVar1 = pass1_1010_b038((param_1 + 0x6e), uVar6, in_EDX, 0x1770, in_stack_0000ffee);
    iVar2 = pass1_1000_3e2c(CONCAT22(in_EDX, pcVar1));
    puVar3 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x32),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    uVar2 = (puVar3 >> 0x10);
    uVar3 = pass1_1010_7818(puVar3, param_2);
    uVar1 = (param_1 + 0x6e);
    pcVar4 = string_op_1010_ada6(uVar2, uVar1, (uVar1 >> 0x10), iVar2, uVar3);
    str_op_1008_60e8((pcVar4 >> 0x10), pcVar4);
    return;
}

// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1a66(mut param_1: u32, param_2: *mut astruct_15) -> u8 {
    let mut uVar2: u32;
    let mut uVar3: u8;
    let mut uVar4: u16;
    let mut BVar4: bool;
    let mut uVar5: *mut astruct_15;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar1: u32;

    uVar5 = (param_2 >> 0x10);
    if (((param_2 + 0x1c) != 0x2) || (((param_2 + 0x1e) & 0xff) != 0)) {
        uVar7 = pass1_1028_b58e((param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
        uVar6 = (param_1 >> 0x10);
        uVar1 = (param_1 + 0x6e);
        pass1_1010_c2d8(uVar1, (uVar1 >> 0x10), uVar7);
        if ((uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0)) {
            uVar2 = (param_1 + 0x6e);
            uVar4 = pass1_1010_b028(uVar2, (uVar2 >> 0x10), param_2);
            BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar4, 0x5);
            if ((BVar4 == 0)
                && (
                    BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar4, 0x6),
                    BVar4 == 0,
                ))
            {
                uVar3 = '\0';
            } else {
                uVar3 = '\x01';
            }
            return uVar3;
        }
    }
    return '\0';
}

pub unsafe fn pass1_1010_1b04(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1b6e(
    param_1: *mut StructD,
    param_2: *mut astruct_19,
    param_3: *mut astruct_19,
    mut param_4: u16,
) -> *mut astruct_19 {
    let mut unaff_BP: u16;
    let mut puVar1: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    pass1_1010_0f24(param_1, CONCAT22(param_3, param_2), param_4);
    param_2[0x1].field_0xe = 0;
    CONCAT22(param_3, param_2) = 0x1d04;
    param_2.segment_0x2 = 0x1010;
    puVar1 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x3),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    param_2[0x1].field_0xe = puVar1;
    param_2[0x1].field8_0x10 = (puVar1 >> 0x10);
    return CONCAT22(param_3, param_2);
}
pub unsafe fn pass1_1010_1bb4(mut param_1: u16, param_2: *mut u32) {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_92;
    let mut extraout_DX: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        if ((param_1 | paVar2) == 0) {
            break;
        }
        ppcVar1 = (*param_2 + 0x40);
        (**ppcVar1)(0x1028, param_2);
        param_1 = extraout_DX;
    }
    return;
}
pub unsafe fn pass1_1010_1c16(mut param_1: u32, param_2: *mut astruct_15, mut param_3: i16) {
    let mut pcVar1: *mut c_char;
    let mut uVar3: *mut astruct_15;
    let mut uVar2: *mut astruct_15;
    let mut uVar4: u32;

    uVar4 = pass1_1028_b58e(param_2);
    uVar3 = (uVar4 >> 0x10);
    uVar2 = uVar3;
    pcVar1 = pass1_1010_b038((param_1 + 0x6e), uVar4, uVar3, 0x178a, param_3);
    str_op_1008_60e8(uVar2, CONCAT22(uVar2, pcVar1));
    return;
}

// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1c40(mut param_1: u32, param_2: *mut astruct_15) -> u8 {
    let mut uVar4: u32;
    let mut uVar3: u16;
    let mut BVar5: bool;
    let mut uVar5: *mut astruct_15;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar2: u8;
    let mut uVar1: u32;

    uVar5 = (param_2 >> 0x10);
    if (((param_2 + 0x1c) != 0x2) || (((param_2 + 0x1e) & 0xff) != 0)) {
        uVar7 = pass1_1028_b58e((param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
        uVar6 = (param_1 >> 0x10);
        uVar1 = (param_1 + 0x6e);
        pass1_1010_c2d8(uVar1, (uVar1 >> 0x10), uVar7);
        if ((uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0)) {
            uVar4 = (param_1 + 0x6e);
            uVar3 = pass1_1010_b028(uVar4, (uVar4 >> 0x10), param_2);
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar3, 0x11);
            if ((BVar5 == 0)
                && (
                    BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar3, 0x12),
                    BVar5 == 0,
                ))
            {
                uVar2 = '\0';
            } else {
                uVar2 = '\x01';
            }
            return uVar2;
        }
    }
    return '\0';
}

pub unsafe fn pass1_1010_1cde(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn struct_op_1010_1d48(param_1: *mut astruct_19, mut param_2: u16) -> *mut astruct_19 {
    let mut iVar1: *mut astruct_19;
    let mut uVar1: *mut astruct_19;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.offset_0x0 = 0x389a;
    iVar1.segment_0x2 = 0x1008;
    iVar1.field2_0x4 = 0;
    iVar1.field3_0x8 = param_2;
    param_1.offset_0x0 = 0x2014;
    iVar1.segment_0x2 = 0x1010;
    return param_1;
}
pub unsafe fn pass1_1010_1d80(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar4: *mut astruct_455;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.address_offset_field_0x0 = 0x2014;
    iVar4.field1_0x2 = 0x1010;
    pass1_1010_1f62((param_1 & 0xffff | ZEXT24(uVar4) << 0x10), 1);
    puVar1 = iVar4.field2_0x4;
    uVar2 = iVar4.field3_0x6;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    param_1.address_offset_field_0x0 = 0x389a;
    iVar4.field1_0x2 = 0x1008;
    return;
}

pub unsafe fn pass1_1010_1dce() -> u16 {
    return 0x0;
}

pub unsafe fn pass1_1010_1dd4() -> u16 {
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_1dda(mut param_1: u32) {
    pass1_1010_209e(_u16_1050_0ed0, (param_1 + 0x8));
    return;
}
pub unsafe fn pass1_1010_1df2(
    param_1: *mut astruct_242,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
    param_5: *mut astruct_57,
) {
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: *mut astruct_241;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar3: *mut astruct_242;
    let mut uVar4: u16;
    let mut puStack10: *mut u16;
    let mut puStack6: *mut u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field4_0x4.is_null()) {
        mem_op_1000_179c(0xc, param_5);
        uVar2 = param_5;
        uVar3 = uVar2 | in_AX;
        param_5 = (param_5 & 0xffff0000 | uVar3);
        if (uVar3 == 0) {
            iVar3.field4_0x4 = null_mut();
        } else {
            set_struct_1008_574a(CONCAT22(uVar2, in_AX));
            iVar3.field4_0x4 = in_AX;
            (iVar3.field4_0x4 + 0x2) = param_5;
        }
    }
    mem_op_1000_179c(0xa, param_5);
    uVar2 = param_5;
    puStack10 = CONCAT22(uVar2, in_AX);
    if ((uVar2 | in_AX) == 0) {
        puStack6 = null_mut();
    } else {
        *puStack10 = 0x389a;
        in_AX.field2_0x2 = 0x1008;
        in_AX.field3_0x4 = param_3;
        in_AX.field4_0x8 = param_2;
        *puStack10 = 0x2010;
        in_AX.field2_0x2 = 0x1010;
        puStack6 = puStack10;
    }
    ppcVar1 = (*iVar3.field4_0x4 + 0x4);
    (**ppcVar1)(0x1000, iVar3.field4_0x4, puStack6);
    return;
}
pub unsafe fn pass1_1010_1ea6(mut param_1: u32, param_2: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar6: *mut astruct_498;
    let mut uVar6: u16;
    let mut local_c: [u8; 0x4] = [0; 0x4];
    let mut uStack8: u32;
    let mut uStack4: u16;

    uVar6 = (param_1 >> 0x10);
    iVar6 = param_1;
    if (iVar6.field4_0x4.is_null()) {
        return;
    }
    uStack4 = 0;
    pass1_1008_5784(CONCAT22(0x1050, local_c), iVar6.field4_0x4);
    loop {
        puVar5 = local_c;
        pass1_1008_5b12(CONCAT22(0x1050, puVar5));
        if ((extraout_DX | puVar5) == 0) {
            break;
        }
        if ((puVar5 + 0x4) == param_2) {
            uStack4 = 0x1;
            ppcVar3 = (*iVar6.field4_0x4 + 0xc);
            (**ppcVar3)(0x1008);
            uStack8 = 0;
        }
    }
    puVar4 = iVar6.field4_0x4;
    if ((puVar4 + 0x8) == 0) {
        // WARNING: Load size is inaccurate
        puVar1 = iVar6.field4_0x4;
        uVar2 = (&iVar6.field4_0x4 + 2);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)(0x1008, puVar1, uVar2, 0x1, puVar1, uVar2, puVar1, uVar2);
        }
        iVar6.field4_0x4 = null_mut();
    }
    return;
}
pub unsafe fn pass1_1010_1f62(param_1: *mut astruct_27, mut param_2: i16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];
    let mut fn_ptr_1: *mut *mut code;

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x4));
    loop {
        lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        uVar3 = (lVar4 >> 0x10);
        iVar2 = lVar4;
        if (lVar4 == 0) {
            break;
        }
        if ((((iVar2 + 0x8) == 0) || (param_2 == 0)) || ((iVar2 + 0x8) == param_2)) {
            uVar1 = (iVar2 + 0x4);
            fn_ptr_1 = ((iVar2 + 0x4) + 0x4);
            (**fn_ptr_1)(0x1008, uVar1, (uVar1 >> 0x10), param_2);
        }
    }
    return;
}

pub unsafe fn pass1_1010_1fbe(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_1fea(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
