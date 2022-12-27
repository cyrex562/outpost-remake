pub fn pass1_1030_201e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_1d28(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_2068(param_1: *mut astruct_180, in_EDX: u32) {
    let mut in_EDX: u32;
    let mut iVar1: *mut astruct_180;
    let mut uVar3: *mut astruct_180;
    let mut iStack4: i16;

    struct_1030_17ce(param_1, 0x0, 0x0, in_EDX);
    uVar3 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field0_0x0 = 0x293c;
    iVar1.field1_0x2 = 0x1030;
    pass1_1000_4906(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.field12_0x10)),
        NULL,
        0x106,
    );
    pass1_1000_4906(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1[0x8].field15_0x16)),
        NULL,
        0x86,
    );
    pass1_1000_4906(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1[0xc].field18_0x1c)),
        NULL,
        0xa,
    );
    pass1_1000_4906(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1[0x15].field10_0xc)),
        NULL,
        0x106,
    );
    iStack4 = 0;
    loop {
        (&iVar1.field12_0x10)[iStack4] = 0xffff;
        (&iVar1[0xd].field_0x6 + iStack4 * 0x2) = 0x19;
        iStack4 += 0x1;
        if iStack4 >= 0x83 {
            break;
        }
    }
    return;
}

pub fn struct_1030_2112(
    mut param_1: u16,
    param_2: *mut u8,
    param_3: *mut astruct_366,
    mut param_4: u32,
) {
    let mut in_register_0000000a: u16;
    let mut pstruct_1: *mut astruct_366;
    let mut pstruct_1_hi: u16;
    let mut iStack4: i16;

    pass1_1030_183c(
        param_1,
        CONCAT22(in_register_0000000a, param_2),
        &param_3.field0_0x0,
        0x1,
        0x1,
        0x8000000,
        param_4,
    );
    pstruct_1_hi = (param_3 >> 0x10);
    pstruct_1 = param_3;
    param_3.field0_0x0 = 0x293c;
    pstruct_1.field1_0x2 = 0x1030;
    iStack4 = 0;
    loop {
        (&pstruct_1.field14_0x10)[iStack4] = 0xffff;
        (&pstruct_1.field405_0x1a6)[iStack4] = 0x19;
        iStack4 += 0x1;
        if iStack4 >= 0x83 {
            break;
        }
    }
    pass1_1000_4906(
        (param_3 & 0xffff0000 | ZEXT24(&pstruct_1.field_0x116)),
        NULL,
        0x86,
    );
    pass1_1000_4906(
        (param_3 & 0xffff0000 | ZEXT24(&pstruct_1.field_0x19c)),
        NULL,
        0xa,
    );
    pass1_1000_4906((param_3 & 0xffff0000 | ZEXT24(pstruct_1 + 1)), NULL, 0x106);
    pstruct_1.field14_0x10 = 0;
    pstruct_1.field17_0x14 = 0;
    pstruct_1.field18_0x16 = 0;
    pstruct_1.field27_0x20 = 0;
    pstruct_1.field62_0x44 = 0;
    pstruct_1.field73_0x50 = 0;
    pstruct_1.field98_0x6a = 0;
    pstruct_1.field123_0x84 = 0;
    pstruct_1.field190_0xc8 = 0;
    pstruct_1.field217_0xe4 = 0;
    pstruct_1.field228_0xf0 = 0;
    pstruct_1.field231_0xf4 = 0;
    pstruct_1.field232_0xf6 = 0;
    pstruct_1.field242_0x102 = 0;
    pstruct_1.field239_0xfe = 0;
    pstruct_1.field405_0x1a6 = 0;
    pstruct_1.field408_0x1aa = 0;
    pstruct_1.field409_0x1ac = 0;
    pstruct_1.field418_0x1b6 = 0;
    pstruct_1.field453_0x1da = 0;
    pstruct_1.field464_0x1e6 = 0;
    pstruct_1.field489_0x200 = 0;
    pstruct_1.field514_0x21a = 0;
    pstruct_1.field581_0x25e = 0;
    pstruct_1.field608_0x27a = 0;
    pstruct_1.field619_0x286 = 0;
    pstruct_1.field622_0x28a = 0;
    pstruct_1.field623_0x28c = 0;
    pstruct_1.field633_0x298 = 0;
    pstruct_1.field630_0x294 = 0;
    return;
}

pub fn pass1_1030_2242(param_1: *mut astruct_168, mut param_2: i16) -> i16 {
    let mut iVar1: i16;
    let mut iVar2: *mut astruct_168;
    let mut paVar2: *mut astruct_168;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    paVar2 = &iVar2.field_0x10;
    if (-0x1 < (paVar2 + param_2 * 0x2)) {
        iVar1 = (&iVar2.field_0x10 + param_2 * 0x2);
        paVar2 = iVar2 + 1;
        if ((&paVar2.field_0x0 + param_2 * 0x2) <= iVar1) {
            return iVar1;
        }
    }
    return (&paVar2.field_0x0 + param_2 * 0x2);
}


pub fn pass1_1030_23e2(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: i16,
    mut param_5: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut iVar5: i16;
    let mut extraout_var: u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_register_0000000a: u16;
    let mut paVar9: *mut Struct57;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut unaff_SI: u16;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut puVar14: *mut u32;
    let mut in_stack_0000fe90: u16;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc0: u16;
    let mut iStack8: i16;

    paVar9 = CONCAT22(in_register_0000000a, param_2);
    uVar13 = (param_3 >> 0x10);
    uVar10 = param_3;
    if ((uVar10 + 0x10 + param_5 * 0x2) < 0x0) {
        uVar7 = param_5;
        if (param_4 == 0) {
            puVar14 = mixed_1010_20ba(
                paVar9,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x31),
                in_stack_0000fe92,
                in_stack_0000ffb6,
                in_stack_0000ffbc,
                in_stack_0000ffc0,
            );
            paVar9 = (paVar9 & 0xffff0000 | puVar14 >> 0x10);
            ppcVar1 = (*puVar14 + 0x14);
            (**ppcVar1)(0x1010, puVar14, (puVar14 >> 0x10), param_5, param_5 >> 0xf);
            uVar6 = SUB42(paVar9, 0x0);
        } else {
            puVar14 = mixed_1010_20ba(
                paVar9,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x41),
                in_stack_0000fe92,
                in_stack_0000ffb6,
                in_stack_0000ffbc,
                in_stack_0000ffc0,
            );
            paVar9 = (paVar9 & 0xffff0000 | puVar14 >> 0x10);
            ppcVar1 = (*puVar14 + 0x14);
            (**ppcVar1)(0x1010, puVar14, (puVar14 >> 0x10), param_5, param_5 >> 0xf);
            uVar6 = SUB42(paVar9, 0x0);
        }
        uVar2 = (uVar7 + 0x16);
        param_1 = (uVar2 + 0x4);
        (uVar10 + param_5 * 0x2 + 0x10) = param_1;
    }
    if ((uVar10 + 0x10 + param_5 * 0x2) != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
        pass1_1030_2852();
        bVar3 = false;
        iStack8 = param_1;
        if ((uVar10 + 0x152) != 0) {
            bVar4 = pass1_1030_266c(uVar10, CONCAT22(param_5, uVar13));
            if (CONCAT31(extraout_var, bVar4) != 0) {
                iStack8 = param_1 + 1;
                bVar3 = true;
            }
        }
        iVar5 = param_5 * 0x2;
        iStack8 = (uVar10 + iVar5 + 0x10) - iStack8;
        (uVar10 + iVar5 + 0x10) = iStack8;
        if (iStack8 < 0x0) {
            (uVar10 + iVar5 + 0x10) = 0;
        }
        iVar5 = param_5 * 0x2;
        if ((uVar10 + 0x2ac + iVar5) == 0) {
            iVar12 = iVar5 + uVar10;
            (iVar12 + 0x2ac) = 0x1;
            uVar7 = (uVar10 + iVar5 + 0x1a6) - 0x1;
            paVar9 = (paVar9 & 0xffff0000 | uVar7);
            (iVar12 + 0x1a6) = uVar7;
            if ((uVar10 + iVar5 + 0x1a6) < 0x0) {
                (iVar12 + 0x1a6) = 0;
            }
        }
        if (((uVar10 + 0x10 + param_5 * 0x2) != 0) || (uVar11 = uVar10 + 0x1a6, (uVar11 + param_5 * 0x2) != 0)) {
            if ((uVar10 + 0x10 + param_5 * 0x2) == 0) {
                (uVar10 + param_5 * 0x2 + 0x10) = 0x1;
            }
            return;
        }
        uVar7 = param_5;
        puVar14 = mixed_1010_20ba(
            paVar9,
            _u16_1050_0ed0,
            CONCAT22(param_5, 0x32),
            in_stack_0000fe90,
            in_stack_0000ffb4,
            in_stack_0000ffba,
            in_stack_0000ffbe,
        );
        uVar8 = (puVar14 >> 0x10);
        pass1_1010_6cf8(uVar11, uVar8, 0x1010, puVar14, uVar7);
        pass1_1030_26ac(uVar8, param_3, param_5);
        if (bVar3) {
            iVar5 = pass1_1030_28dc(param_3, param_5);
            (uVar10 + iVar5 * 0x2 + 0x19c) = 0;
        }
    }
    return;
}

pub fn pass1_1030_25b2(mut param_1: u32, mut param_2: i16) -> BOOL16 {
    if ((param_1 + 0x10 + param_2 * 0x2) == 0) {
        return 0x1;
    }
    return 0x0;
}

pub fn pass1_1030_25d8(mut param_1: u32, mut param_2: u16, mut param_3: i16) {
    (param_1 + param_3 * 0x2 + 0x10) = param_2;
    return;
}

pub fn pass1_1030_25f0(mut param_1: u32, mut param_2: i16, mut param_3: i16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (param_2 == 0) {
        param_2 = (param_1 + 0x116 + param_3 * 0x2) + 1;
    }
    (param_1 + param_3 * 0x2 + 0x116) = param_2;
    return;
}

pub fn pass1_1030_2622(mut param_1: u32, mut param_2: i16) -> bool {
    let mut iVar1: i16;

    if ((param_2 != 0x70) && (param_2 != 1)) {
        iVar1 = pass1_1030_28dc(param_1, 0x0);
        if (-0x1 < iVar1) {
            (param_1 + iVar1 * 0x2 + 0x19c) = param_2;
        }
        return -0x1 < iVar1;
    }
    return false;
}

pub fn pass1_1030_266c(mut param_1: u16, mut param_2: u32) -> bool {
    let mut iVar1: i16;

    iVar1 = pass1_1030_28dc(CONCAT22(param_2, param_1), (param_2 >> 0x10));
    return iVar1 != -0x1;
}

pub fn pass1_1030_2690(mut param_1: u32) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x2ac)), NULL, 0x106);
    return;
}

pub fn pass1_1030_26ac(mut param_1: u16, mut param_2: u32, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut cVar5: u8;
    let mut paVar6: *mut astruct_92;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut iVar9: i16;
    let mut uVar10: u16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut iStack38: i16;
    let mut local_14: *mut astruct_92;

    iVar11 = param_2;
    uVar12 = (param_2 >> 0x10);
    if (param_3 != 0x13) {
        if (0x13 < param_3) {
            if (param_3 != 0x5f) {
                if ((param_3 - 0x5f) < 0x6) {
                    return;
                }
                if (param_3 != 0x66 && 0x0 < (param_3 - 0x65)) {
                    if ((param_3 - 0x66) < 0x5) {
                        return;
                    }
                    if (0x4 < (param_3 - 0x6b)) {
                        return;
                    }
                }
            }
            pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
            loop {
                uVar10 = param_1;
                paVar6 = &local_14;
                pass1_1028_e4ec(CONCAT22(0x1050, paVar6));
                param_1 = uVar10 | paVar6;
                if (param_1 == 0) {
                    break;
                }
                if ((iVar11 + 0x4) == paVar6[0x1c].field4_0x8) {
                    uVar7 = (&paVar6[0x1].field3_0x4 + 0x2) + 0x19;
                    if (0x3e8 < uVar7) {
                        uVar7 = 0x3e8;
                    }
                    pass1_1038_4d0e(CONCAT22(uVar10, paVar6), uVar7);
                }
            }
            return;
        }
        if (param_3 == 0x12) {
            pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
            loop {
                uVar10 = param_1;
                paVar6 = &local_14;
                pass1_1028_e4ec(CONCAT22(0x1050, paVar6));
                param_1 = uVar10 | paVar6;
                if (param_1 == 0) {
                    break;
                }
                if ((iVar11 + 0x4) == paVar6[0x1c].field4_0x8) {
                    uVar2 = &paVar6[0x1b].field6_0x10;
                    iVar9 = uVar2;
                    uVar4 = (uVar2 >> 0x10);
                    piVar1 = (iVar9 + 0x182);
                    *piVar1 = *piVar1 - 0x19;
                    iVar8 = (iVar9 + 0x182);
                    if (iVar8 < 1) {
                        iVar8 = 0x1;
                    }
                    (iVar9 + 0x182) = iVar8;
                }
            }
            return;
        }
        if (0x12 < param_3) {
            return;
        }
        cVar5 = param_3;
        if (cVar5 != '\n') {
            if ((cVar5 - 0xa) < '\x06') {
                return;
            }
            if ('\x01' < (cVar5 - 0x10)) {
                return;
            }
        }
    }
    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        uVar10 = param_1;
        paVar6 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar6));
        param_1 = uVar10 | paVar6;
        if (param_1 == 0) {
            break;
        }
        if ((iVar11 + 0x4) == paVar6[0x1c].field4_0x8) {
            uVar2 = &paVar6[0x1b].field6_0x10;
            iVar8 = uVar2 + 0x180;
            uVar4 = (uVar2 >> 0x10);
            iStack38 = 0x1;
            loop {
                iVar3 = iStack38 * 0x2;
                piVar1 = (iVar3 + iVar8);
                *piVar1 = *piVar1 - 0x1;
                iVar9 = (iVar3 + iVar8);
                if (iVar9 < 1) {
                    iVar9 = 0x1;
                }
                (iVar3 + iVar8) = iVar9;
                iStack38 += 0x1;
                if iStack38 >= 6 {
                    break;
                }
            }
        }
    }
    return;
}


pub fn pass1_1030_2852() {
    return;
}

pub fn pass1_1030_28dc(mut param_1: u32, mut param_2: i16) -> i16 {
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        if (0x4 < iStack4) {
            return -0x1;
        }
        if ((param_1 + 0x19c + iStack4 * 0x2) == param_2) {
            break;
        }
        iStack4 += 0x1;
    }
    return iStack4;
}



pub fn pass1_1030_2958(param_1: *mut astruct_180, mut param_2: u32) {
    let mut pstruct_1: *mut astruct_180;
    let mut pstruct_1_hi: *mut astruct_180;

    struct_1030_17ce(param_1, 0x5, 0xf, param_2);
    pstruct_1_hi = (param_1 >> 0x10);
    pstruct_1 = param_1;
    pstruct_1.field12_0x10 = 0;
    pstruct_1.field14_0x14 = 0;
    pstruct_1.field15_0x16 = 0;
    pstruct_1.field16_0x18 = 0x2710;
    pstruct_1.field17_0x1a = 0;
    param_1.field0_0x0 = 0x3130;
    pstruct_1.field1_0x2 = 0x1030;
    return;
}

pub fn struct_1030_299a(
    mut param_1: u16,
    param_2: *mut Struct57,
    param_3: *mut astruct_352,
    mut param_4: u32,
) {
    let mut pstruct_1: *mut astruct_352;
    let mut pstruct_1_hi: *mut astruct_352;

    pass1_1030_183c(
        param_1,
        param_2,
        &param_3.u16_field_0x0,
        0x5,
        0xf,
        0x2000000,
        param_4,
    );
    pstruct_1_hi = (param_3 >> 0x10);
    pstruct_1 = param_3;
    pstruct_1.field14_0x10 = 0;
    pstruct_1.field15_0x14 = 0;
    pstruct_1.field16_0x16 = 0;
    pstruct_1.field17_0x18 = 0x2710;
    pstruct_1.field18_0x1a = 0;
    param_3.u16_field_0x0 = 0x3130;
    pstruct_1.field1_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_29e6(param_1: *mut StructD) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut iVar4: *mut StructD;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.address_offset_field_0x0 = 0x3130;
    iVar4.address_offset_field_0x2 = 0x1030;
    pcVar2 = *&iVar4.field_0x10;
    uVar1 = iVar4.field11_0x12;
    if ((uVar1 | pcVar2) != 0) {
        pass1_1030_8496(pcVar2 & 0xffff | uVar1 << 0x10);
        fn_ptr_1000_17ce(pcVar2);
    }
    pass1_1030_18b2(&param_1.address_offset_field_0x0);
    return;
}


pub fn pass1_1030_2a2c(param_1: *mut StructD, param_2: *mut astruct_678) {
    let mut piVar1: *mut i16;
    let mut iVar2: *mut astruct_678;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_67;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc2: u16;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iVar9: i16;

    uVar2 = (param_2 >> 0x10);
    iVar2 = param_2;
    if (0x0 < iVar2.field23_0x18) {
        piVar1 = &iVar2.field23_0x18;
        *piVar1 = *piVar1 - 0x1;
    }
    if (iVar2.field22_0x16 == 0) {
        iVar2.field22_0x16 = 0x1;
    }
    if (iVar2.field24_0x1a == 0) {
        iVar2.field24_0x1a = 0x2;
    }
    if (iVar2.field23_0x18 < 1) {
        iVar2.field22_0x16 = 0x2;
        iVar2.field24_0x1a = 0x1;
        uVar8 = 0;
        iVar9 = 0x21;
        uVar6 = 0x1;
        uVar7 = 0;
        uVar4 = 0;
        iVar5 = 0;
        uVar2 = 0;
        paVar3 = mixed_1010_20ba(
            param_1,
            _u16_1050_0ed0,
            0x37,
            in_stack_0000fe94,
            in_stack_0000ffb8,
            in_stack_0000ffbe,
            in_stack_0000ffc2,
        );
        post_win_msg_1008_a0e4(
            paVar3,
            CONCAT22(uVar4, uVar2),
            iVar5,
            uVar6,
            CONCAT22(uVar8, uVar7),
            iVar9,
        );
    }
    return;
}

pub fn pass1_1030_2a98(mut param_1: u32) -> u16 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x14);
    *piVar1 = *piVar1 + 1;
    return (param_1 + 0x14);
}

pub fn pass1_1030_2aaa(mut param_1: u32) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x10) == 0) {
        return 0x0;
    }
    uVar1 = (param_1 + 0x10);
    return (uVar1 + 0xc);
}


pub fn pass1_1030_2f1a(mut param_1: u32, param_2: *mut u16, param_3: *mut u16) -> i16 {
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut iVar3: i16;

    uVar2 = (param_1 + 0x10);
    iVar3 = uVar2;
    iVar1 = (iVar3 + 0xc);
    if (iVar1 - 0x1 < 0x9) {
        match iVar1 {
            _ => {
                *param_3 = 0x19;
                *param_2 = 0x2d;
                return iVar3;
            }
            0x3 | 0x4 | 0x5 => {
                *param_3 = 0xa;
                *param_2 = 0xf;
                return iVar3;
            }
            0x6 => {
                *param_3 = 0xa;
                *param_2 = 0x19;
                return iVar3;
            }
            0x7 => {
                *param_3 = 0x19;
                *param_2 = 0x37;
                return iVar3;
            }
        };
    }
    *param_3 = 0;
    *param_2 = 0;
    return 0x0;
}

pub fn pass1_1030_2fac(param_1: *mut astruct_598) -> u16 {
    let mut lVar1: i32;
    let mut iVar2: *mut astruct_598;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field16_0x10 == 0) {
        return 0x0;
    }
    lVar1 = iVar2.field16_0x10;
    if ((lVar1 + 0xc) < 0x2) {
        return 0x4;
    }
    lVar1 = iVar2.field16_0x10;
    if ((lVar1 + 0xc) < 0x5) {
        return 0x3;
    }
    lVar1 = iVar2.field16_0x10;
    if ((lVar1 + 0xc) < 0x8) {
        return 0x2;
    }
    return 0x1;
}
