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



pub unsafe fn pass1_1010_11c6(
    mut param_1: u16,
    param_2: *mut Struct57,
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








pub unsafe fn pass1_1010_195e(
    param_1: *mut Struct57,
    param_2: *mut Struct19,
    param_3: *mut Struct19,
    mut param_4: u16,
) -> *mut Struct19 {
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



















pub unsafe fn struct_op_1010_1d48(param_1: *mut Struct19, mut param_2: u16) -> *mut Struct19 {
    let mut iVar1: *mut Struct19;
    let mut uVar1: *mut Struct19;

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




pub unsafe fn pass1_1010_1dda(mut param_1: u32) {
    pass1_1010_209e(_u16_1050_0ed0, (param_1 + 0x8));
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
pub unsafe fn pass1_1010_1f62(param_1: *mut Struct27, mut param_2: i16) {
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
