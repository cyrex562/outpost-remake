
pub unsafe fn file_1030_778c(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar3: *mut astruct_387;
    let mut BVar2: bool;
    let mut iVar6: i16;
    let mut plVar7: *mut i32;
    let mut paVar8: *mut astruct_169;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut uVar10: *mut astruct_99;
    let mut in_register_0000000a: u16;
    let mut paVar12: *mut Struct57;
    let mut iVar4: *mut astruct_99;
    let mut iVar5: *mut astruct_99;
    let mut uVar13: u16;
    let mut uVar15: u16;
    let mut uVar14: *mut astruct_99;
    let mut local_56: [u16; 0x2] = [0; 0x2];
    let mut uStack82: u16;
    let mut paStack74: *mut astruct_99;
    let mut local_46: [u16; 0x2] = [0; 0x2];
    let mut local_42: [u16; 0x2] = [0; 0x2];
    let mut local_3e: [u32; 0x3] = [0; 0x3];
    let mut paStack50: *mut astruct_99;
    let mut local_2e: u16;
    let mut paStack44: *mut astruct_99;
    let mut local_28: [u16; 0x2] = [0; 0x2];
    let mut local_24: [u16; 0x2] = [0; 0x2];
    let mut local_20: [u16; 0x9] = [0; 0x9];
    let mut uStack14: u16;
    let mut local_4: u16;
    let mut uVar5: *mut astruct_388;
    let mut uVar8: *mut astruct_99;

    paVar12 = CONCAT22(in_register_0000000a, param_2);
    file_1030_1730(param_3, param_4);
    if (param_1 != 0) {
        iVar3 = param_3;
        iVar3 = &iVar3.field_0xc;
        BVar2 = read_file_1008_7bc8(param_4, (param_3 & 0xffff0000 | ZEXT24(iVar3)));
        if ((BVar2 != 0)
            && (
                BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2),
                BVar2 != 0,
            ))
        {
            uVar13 = (param_3 >> 0x10);
            iVar3.field18_0x12 = local_4;
            BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
            if (BVar2 != 0) {
                iVar3.field19_0x14 = local_4;
                BVar2 = read_file_1008_7dee(
                    param_4,
                    (param_3 & 0xffff0000 | ZEXT24(&iVar3.field_0x16)),
                    0x4,
                );
                if (BVar2 != 0) {
                    plVar7 = (param_3 & 0xffff0000 | ZEXT24(&iVar3.field_0x1e));
                    file_1008_76e4(paVar12, param_4, plVar7);
                    if ((((plVar7 != 0)
                        && (
                            iVar6 = file_1008_77cc(
                                paVar12,
                                param_4,
                                (param_3 & 0xffff0000 | ZEXT24(&iVar3.field_0x22)),
                            ),
                            iVar6 != 0x0,
                        ))
                        && (
                            iVar6 = file_1008_77cc(
                                paVar12,
                                param_4,
                                (param_3 & 0xffff0000 | ZEXT24(&iVar3.field_0x26)),
                            ),
                            iVar6 != 0,
                        ))
                        && (
                            BVar2 = read_file_1008_7dee(
                                param_4,
                                (param_3 & 0xffff0000 | ZEXT24(&iVar3.field40_0x2a)),
                                0x4,
                            ),
                            BVar2 != 0,
                        ))
                    {
                        if (iVar3.field40_0x2a != 0) {
                            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, iVar3.field40_0x2a);
                            iVar3.field41_0x2e = BVar2;
                            iVar3.field42_0x30 = paVar12;
                        }
                        if (u16_1050_0312 < 0x2) {
                            return;
                        }
                        BVar2 = read_file_1008_7dee(
                            param_4,
                            (param_3 & 0xffff0000 | ZEXT24(&iVar3.field_0x32)),
                            0x2,
                        );
                        if ((BVar2 != 0)
                            && (
                                BVar2 = read_file_1008_7dee(
                                    param_4,
                                    (param_3 & 0xffff0000 | ZEXT24(&iVar3.field_0x34)),
                                    0x2,
                                ),
                                BVar2 != 0,
                            ))
                        {
                            paVar8 = (param_3 & 0xffff0000 | ZEXT24(&iVar3.field_0x36));
                            pass1_1008_766e(paVar12, param_4, paVar8);
                            if ((paVar8 != 0)
                                && (
                                    BVar2 = read_file_1008_7dee(
                                        param_4,
                                        CONCAT22(0x1050, local_20),
                                        0x2,
                                    ),
                                    BVar2 != 0,
                                ))
                            {
                                // for (uStack14 = 0; uVar15 = (paVar12 >> 0x10), uStack14 < local_20[0];
                                //     uStack14 += 1)
                                uStack14 = 0;
                                uVar15 = pavar12 >> 0x10;
                                while uStack14 < local_20[0] {
                                    local_3e[0] = _PTR_LOOP_1050_68a2;
                                    paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                                    uVar11 = (paStack50 >> 0x10);
                                    uVar5 = paStack50;
                                    paVar12 = CONCAT22(uVar15, uVar11 | uVar5);
                                    if ((uVar11 | uVar5) == 0) {
                                        paStack44 = null_mut();
                                    } else {
                                        paStack50.field0_0x0 = 0x389a;
                                        uVar5.field2_0x2 = 0x1008;
                                        uVar5.field3_0x4 = 0;
                                        uVar5.field4_0x6 = 0;
                                        uVar5.field5_0x8 = 0;
                                        uVar5.field6_0xa = 0;
                                        uVar5.field7_0xc = 0;
                                        paStack50.field0_0x0 = 0x56ce;
                                        uVar5.field2_0x2 = 0x1018;
                                        paStack44 = paStack50;
                                    }
                                    BVar2 = read_file_1008_7dee(
                                        param_4,
                                        CONCAT22(0x1050, local_28),
                                        0x2,
                                    );
                                    if (((BVar2 == 0)
                                        || (
                                            BVar2 = read_file_1008_7dee(
                                                param_4,
                                                CONCAT22(0x1050, local_24),
                                                0x2,
                                            ),
                                            BVar2 == 0,
                                        ))
                                        || ((
                                            BVar2 = read_file_1008_7dee(
                                                param_4,
                                                CONCAT22(0x1050, &local_2e),
                                                0x2,
                                            ),
                                            BVar2 == 0x0
                                                || ((
                                                    BVar2 = read_file_1008_7dee(
                                                        param_4,
                                                        (paStack44 & 0xffff0000
                                                            | (paStack44 + 0xa)),
                                                        0x2,
                                                    ),
                                                    BVar2 == 0x0
                                                        || (
                                                            BVar2 = read_file_1008_7dee(
                                                                param_4,
                                                                (paStack44 & 0xffff0000
                                                                    | (paStack44 + 0xc)),
                                                                0x2,
                                                            ),
                                                            BVar2 == 0,
                                                        ),
                                                )),
                                        )))
                                    {
                                        // goto LAB_1030_77be;
                                    }
                                    uVar15 = (paStack44 >> 0x10);
                                    iVar4 = paStack44;
                                    iVar4.field2_0x4 = local_28[0];
                                    (&iVar4.field2_0x4 + 0x2) = local_24[0];
                                    iVar4.field3_0x8 = local_2e;
                                    if (iVar3.field51_0x3a.is_null()) {
                                        uVar11 = local_2e;
                                        mem_op_1000_179c(0xc, paVar12);
                                        uVar9 = paVar12;
                                        paStack50 = CONCAT22(uVar9, uVar11);
                                        paVar12 = (paVar12 & 0xffff0000 | (uVar9 | uVar11));
                                        if ((uVar9 | uVar11) == 0) {
                                            iVar3.field51_0x3a = null_mut();
                                        } else {
                                            set_struct_1008_574a(CONCAT22(uVar9, uVar11));
                                            iVar3.field51_0x3a = uVar11;
                                            (&iVar3.field51_0x3a + 0x2) = paVar12;
                                        }
                                    }
                                    ppcVar1 = (*iVar3.field51_0x3a + 0x8);
                                    (**ppcVar1)();
                                }
                                BVar2 =
                                    read_file_1008_7dee(param_4, CONCAT22(0x1050, local_56), 0x2);
                                if (BVar2 != 0) {
                                    uStack82 = 0;
                                    loop {
                                        uVar15 = (paVar12 >> 0x10);
                                        if (local_56[0] <= uStack82) {
                                            return;
                                        }
                                        paStack44 = _PTR_LOOP_1050_68a2;
                                        paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                                        uVar10 = (paStack50 >> 0x10);
                                        uVar8 = paStack50;
                                        paVar12 = CONCAT22(uVar15, uVar10 | uVar8);
                                        if ((uVar10 | uVar8) == 0) {
                                            paStack74 = null_mut();
                                        } else {
                                            paStack50.field0_0x0 = 0x389a;
                                            uVar8.field1_0x2 = 0x1008;
                                            uVar8.field2_0x4 = 0;
                                            (&uVar8.field2_0x4 + 0x2) = 0;
                                            uVar8.field3_0x8 = 0;
                                            (&uVar8.field3_0x8 + 0x2) = 0;
                                            uVar8.field4_0xc = 0;
                                            paStack50.field0_0x0 = 0x56ce;
                                            uVar8.field1_0x2 = 0x1018;
                                            paStack74 = paStack50;
                                        }
                                        BVar2 = read_file_1008_7dee(
                                            param_4,
                                            CONCAT22(0x1050, local_46),
                                            0x2,
                                        );
                                        if ((((BVar2 == 0)
                                            || (
                                                BVar2 = read_file_1008_7dee(
                                                    param_4,
                                                    CONCAT22(0x1050, local_42),
                                                    0x2,
                                                ),
                                                BVar2 == 0,
                                            ))
                                            || (
                                                BVar2 = read_file_1008_7dee(
                                                    param_4,
                                                    CONCAT22(0x1050, local_3e),
                                                    0x2,
                                                ),
                                                BVar2 == 0,
                                            ))
                                            || ((
                                                BVar2 = read_file_1008_7dee(
                                                    param_4,
                                                    (paStack74 & 0xffff0000 | (paStack74 + 0xa)),
                                                    0x2,
                                                ),
                                                BVar2 == 0x0
                                                    || (
                                                        BVar2 = read_file_1008_7dee(
                                                            param_4,
                                                            (paStack74 & 0xffff0000
                                                                | (paStack74 + 0xc)),
                                                            0x2,
                                                        ),
                                                        BVar2 == 0,
                                                    ),
                                            )))
                                        {
                                            break;
                                        }
                                        uVar14 = (paStack74 >> 0x10);
                                        iVar5 = paStack74;
                                        iVar5.field2_0x4 = local_46[0];
                                        (&iVar5.field2_0x4 + 0x2) = local_42[0];
                                        iVar5.field3_0x8 = local_3e[0];
                                        if (iVar3.field52_0x3e.is_null()) {
                                            mem_op_1000_179c(0xc, paVar12);
                                            uVar11 = paVar12;
                                            paStack50 = CONCAT22(uVar11, local_3e[0]);
                                            paVar12 =
                                                (paVar12 & 0xffff0000 | (uVar11 | local_3e[0]));
                                            if ((uVar11 | local_3e[0]) == 0) {
                                                iVar3.field52_0x3e = null_mut();
                                            } else {
                                                set_struct_1008_574a(CONCAT22(uVar11, local_3e[0]));
                                                iVar3.field52_0x3e = local_3e[0];
                                                (&iVar3.field52_0x3e + 0x2) = paVar12;
                                            }
                                        }
                                        ppcVar1 = (*iVar3.field52_0x3e + 0x8);
                                        (**ppcVar1)();
                                        uStack82 += 0x1;
                                    }
                                }
                                uStack14 += 1;
                            }
                        }
                    }
                }
            }
        } //
          // LAB_1030_77be:
        u16_1050_0310 = 0x6d2;
    }
    return;
}



pub unsafe fn pass1_1030_7ea0(mut param_1: u32) -> BOOL16 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut BVar3: bool;

    uVar2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0xb);
    if (BVar3 != 0) {
        uVar1 = (param_1 + 0x1a);
        if ((uVar1 + 0x12) == 0x5) {
            return 0x1;
        }
        BVar3 = 0;
    }
    return BVar3;
}


pub unsafe fn pass1_1030_7fd6(mut param_1: u16, mut param_2: u32) {
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut in_AX: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u32;

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if ((iVar3 + 0x1a) == 0) {
        uVar5 = struct_op_1030_73a8((param_2 & 0xffff | uVar4 << 0x10), in_AX, param_1);
        param_1 = (uVar5 >> 0x10);
    }
    uVar2 = (iVar3 + 0x1a);
    iVar1 = (uVar2 + 0xc);
    if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
        && (iVar1 == 0x34
            || iVar1 -0x33 < 0x1
            || (0x2b < iVar1 -0x34 && (iVar1 -0x60 < 0x2))))
    {
        pass1_1028_1416(param_1, (iVar3 + 0x1a));
    }
    return;
}

pub unsafe fn pass1_1030_8030(mut param_1: u16, mut param_2: u32) {
    let mut iVar1: i16;
    let mut paVar2: *mut astruct_15;
    let mut uVar3: u32;
    let mut in_AX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;

    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    if ((iVar4 + 0x1a) == 0) {
        uVar6 = struct_op_1030_73a8((param_2 & 0xffff | uVar5 << 0x10), in_AX, param_1);
        param_1 = (uVar6 >> 0x10);
    }
    uVar3 = (iVar4 + 0x1a);
    iVar1 = (uVar3 + 0xc);
    if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
        && (iVar1 == 0x34
            || iVar1 -0x33 < 0x1
            || (0x2b < iVar1 -0x34 && (iVar1 -0x60 < 0x2))))
    {
        paVar2 = (iVar4 + 0x1a);
        pass1_1028_1106(paVar2, param_1, paVar2);
    }
    return;
}

pub unsafe fn pass1_1030_80ee(param_1: *mut astruct_611, param_2: u8) -> *mut astruct_611 {
    pass1_1030_68dc(param_1);
    if ((param_2 & 1) != 0) {
        pass1_1000_093a(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1030_888e(mut param_1: u32, mut param_2: u32) -> u16 {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = (param_1 + 0x4);
    uVar4 = (param_2 >> 0x10);
    piVar1 = (param_2 + 0x4);
    if (*piVar1 != iVar2 && iVar2 <= *piVar1) {
        return 0xffff;
    }
    if ((param_2 + 0x4) < (param_1 + 0x4)) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn pass1_1030_8e12(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_8a2c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_9e9c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        pass1_1000_093a(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_b90c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_afa6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_b9da(
    param_1: *mut Struct57,
    param_2: *mut astruct_172,
    param_3: *mut astruct_402,
    mut param_4: u32,
) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut uVar3: *mut Struct57;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut paVar7: *mut Struct57;
    let mut iVar7: *mut astruct_402;
    let mut uVar6: *mut astruct_402;
    let mut uVar8: u32;
    let mut uStack12: u16;
    let mut uStack4: u16;
    let mut paVar6: *mut Struct57;

    uVar6 = (param_3 >> 0x10);
    iVar7 = param_3;
    paVar6 = param_1;
    if (&iVar7.field14_0xe == 0) {
        mem_op_1000_179c(0xa, param_1);
        uVar3 = (param_1 | param_2);
        paVar6 = ZEXT24(uVar3);
        if (uVar3.is_null()) {
            iVar7.field14_0xe = 0;
        } else {
            pass1_1020_ba3e((param_2 & 0xffff | param_1 << 0x10), 0x5, 0x5);
            iVar7.field14_0xe = param_2;
            iVar7.field15_0x10 = paVar6;
        }
        iVar7.field16_0x12 = 0;
    }
    //   for (uStack4 = 0x4; uStack4 < 0xe; uStack4 += 1)
    for uStack4 in 0x4..0xe {
        uVar8 = pass1_1030_7c28(param_2, paVar6, param_4, uStack4);
        uVar2 = (uVar8 >> 0x10);
        param_2 = (uVar8 & 0xffff);
        uVar5 = uVar2 | param_2;
        paVar6 = uVar5;
        if (uVar5 != 0) {
            uVar4 = 0x64 - iVar7.field16_0x12;
            paVar7 = (uVar4 >> 0x10);
            uStack12 = uVar8;
            if (uVar8 < uVar4) {
                uVar4 = uVar8 & 0xffff;
                paVar7 = uVar2;
            }
            uVar5 = uVar4;
            param_2 = (uVar4 & 0xffff | paVar7 << 0x10);
            paVar6 = paVar7;
            pass1_1030_7d1c(
                uVar5,
                paVar7,
                param_4,
                uStack12 - uVar5,
                CONCAT22(uStack4, (uVar2 - paVar7) - (uStack12 < uVar5)),
            );
            pass1_1020_bb8a(&iVar7.field14_0xe, uVar5, paVar7 | uStack4 << 0x10);
            puVar1 = &iVar7.field16_0x12;
            *puVar1 = &param_2.field0_0x0 + *puVar1;
            string_1020_c0ca(uStack4);
            vsprintf_op_1030_840a(paVar6, s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c);
            if (0x63 < iVar7.field16_0x12) {
                break;
            }
        }
    }
    if (iVar7.field16_0x12 != 0) {
        return;
    }
    return;
}


pub unsafe fn pass1_1030_bb0e(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: u16,
) {
    let mut puVar1: *mut u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut uVar7: u32;
    let mut uStack8: u16;
    let mut paVar6: *mut Struct57;

    uVar3 = pass1_1030_7bee(param_3);
    uVar4 = uVar3;
    if (uVar3 != 0) {
        return;
    }
    pass1_1030_b9b2(param_2);
    uVar2 = uVar4 & 0xffff;
    puVar1 = (uVar2 | param_4 << 0x10);
    uVar5 = param_4 | uVar4;
    paVar6 = CONCAT22(in_register_0000000a, uVar5);
    if (uVar5 != 0) {
        // for (uStack8 = 0x4; uStack8 < 0x25; uStack8 += 1)
        for uStack8 in 0x4..0x25 {
            uVar7 = pass1_1020_bae6(uVar4, paVar6, uVar2, CONCAT22(uStack8, param_4));
            uVar4 = uVar7 & 0xffff;
            uVar5 = (uVar7 >> 0x10) | uVar4;
            paVar6 = (paVar6 & 0xffff0000 | uVar5);
            if (uVar5 != 0) {
                pass1_1030_7ddc(uVar4, paVar6, param_3, uVar7, uStack8);
                uVar3 = pass1_1030_7bee(param_3);
                uVar4 = uVar3;
                if (uVar3 != 0) {
                    return;
                }
                string_1020_c0ca(uStack8);
                vsprintf_op_1030_840a(paVar6, s_truck_0x_08lx_unloaded__ld_of__s_1050_5798);
                pass1_1020_bb8a(puVar1, 0x0, uStack8 << 0x10);
            }
        }
        if (puVar1.is_null() == false) {
            fn_ptr_1020_ba7e(puVar1);
            fn_ptr_1000_17ce(puVar1);
        }
    }
    return;
}

pub unsafe fn pass1_1030_bbe6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_b96c(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1030_bc6c() {
    return;
}

pub unsafe fn pass1_1030_bc70(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_bc4e(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_be80(param_1: *mut u8, param_2: *mut astruct_15) {
    let mut piVar1: *mut i16;
    let mut pSVar2: *mut StructD;
    let mut iVar3: i16;
    let mut BVar4: bool;
    let mut uVar5: u32;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut pstruct15_7: *mut astruct_15;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut uVar2: *mut StructD;

    pass1_1028_bf22(param_1, param_2);
    uVar7 = (param_2 >> 0x10);
    pstruct15_7 = param_2;
    if (pstruct15_7.field15_0x12 == 0x5) {
        pSVar2 = pstruct15_7.field16_0x14;
        (pSVar2 + 0xa4) = 0x1e;
        uVar2 = pstruct15_7.field16_0x14;
        (uVar2 + 0xac) = 0x1;
        iVar8 = pstruct15_7.field10_0xc;
        iVar3 = iVar8 -0x1b;
        if (iVar3 == 0) {
            pSVar2 = pstruct15_7.field16_0x14;
            (pSVar2 + 0xaa) = 0xa;
        } else {
            iVar3 = iVar8 -0x1c;
            if (iVar3 == 0) {
                pSVar2 = pstruct15_7.field16_0x14;
                (pSVar2 + 0xaa) = 0xb;
            } else {
                iVar3 = iVar8 -0x1d;
                if (iVar3 == 0) {
                    pSVar2 = pstruct15_7.field16_0x14;
                    (pSVar2 + 0xaa) = 0xc;
                }
            }
        }
        pass1_1028_b58e(param_2);
        uVar5 = (iVar3 + 0x2e);
        iVar8 = 0xc;
        uVar6 = extraout_DX;
        pass1_1038_3fb0(uVar5);
        BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10, iVar8);
        if (BVar4 != 0) {
            pSVar2 = pstruct15_7.field16_0x14;
            piVar1 = (pSVar2 + 0xaa);
            *piVar1 = *piVar1 + 1;
        }
        BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10, 0xe);
        if (BVar4 != 0) {
            pSVar2 = pstruct15_7.field16_0x14;
            piVar1 = (pSVar2 + 0xaa);
            *piVar1 = *piVar1 + 1;
        }
        BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10, 0x76);
        if (BVar4 != 0) {
            pSVar2 = pstruct15_7.field16_0x14;
            piVar1 = (pSVar2 + 0xaa);
            *piVar1 = *piVar1 + 1;
        }
    }
    return;
}


pub unsafe fn pass1_1030_bf6e(mut param_1: u32) {
    let mut puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut uStack6: u32;

    uVar8 = 0x1e;
    uVar3 = pass1_1028_b58e(param_1);
    uVar4 = in_EDX;
    uStack6 = CONCAT22(uVar4, uVar3);
    uVar7 = pass1_1030_7c28(uVar3, uVar4, CONCAT22(uVar4, uVar3), uVar8);
    uVar4 = 0x3e8 - uVar7;
    uVar2 = (param_1 + 0x14);
    uVar6 = (uVar2 >> 0x10);
    iVar5 = uVar2;
    puVar1 = (iVar5 + 0xaa);
    uVar3 = -(uVar4 < *puVar1);
    pass1_1030_7ddc(
        uVar3,
        (in_EDX & 0xffff0000 | uVar7 >> 0x10),
        uStack6,
        ((uVar4 - *puVar1 & uVar3) + (iVar5 + 0xaa)),
        0x1e,
    );
    return;
}

pub unsafe fn pass1_1030_bfe0(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_c0d2(mut param_1: u32) -> u16 {
    if (0x0 < (param_1 + 0x24)) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn pass1_1030_c0ec(mut param_1: u32) -> u16 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (((param_1 + 0xc) != 0xb) && ((param_1 + 0x24) < 1)) {
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1030_c10e(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (0x0 < (iVar2 + 0x24)) {
        piVar1 = (iVar2 + 0x24);
        *piVar1 = *piVar1 -0x1;
        return;
    }
    (iVar2 + 0xc) = 0x37;
    return;
}


pub unsafe fn pass1_1030_c12e(mut param_1: i16, mut param_2: u32, mut param_3: i16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut extraout_DX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut paStack6: *mut astruct_397;

    pass1_1028_b58e(param_2);
    paStack6 = CONCAT22(extraout_DX, param_1);
    uVar2 = (param_1 + 0x2e);
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    iVar3 = uVar2;
    if ((iVar4 + 0x24) < 1) {
        pass1_1030_7d1c(iVar3, extraout_DX, paStack6, 0x0, 0x230000);
    } else {
        if (param_3 == 0) {
            uVar6 = 0;
        } else {
            uVar6 = 0x32;
        }
        pass1_1030_7d1c(iVar3, extraout_DX, paStack6, uVar6, 0x230000);
        piVar1 = (iVar4 + 0x24);
        *piVar1 = *piVar1 -0x1;
    }
    if ((0x0 < (iVar4 + 0x24)) && ((iVar4 + 0x24) < 0x19)) {
        (iVar3 + 0x1fe) = 0x1;
    }
    return;
}


pub unsafe fn pass1_1030_c1b2(param_1: *mut u8, param_2: *mut astruct_695) {
    let mut iVar1: i16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut astruct_695;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc4: u16;
    let mut iVar5: i16;
    let mut in_stack_0000ffee: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    pass1_1028_be9e(param_2);
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    if (iVar2.field17_0x12 == 0x5) {
        if (iVar2.field12_0xc == 0xb) {
            pass1_1030_c652(paVar2, 0xc1d7);
            iVar5 = 0x82;
            puVar4 = mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                0x820008,
                in_stack_0000fe94,
                in_stack_0000ffb8,
                in_stack_0000ffbe,
                in_stack_0000ffc2,
            );
            paVar2 = (paVar2 & 0xffff0000 | puVar4 >> 0x10);
            iVar1 = puVar4;
            pass1_1010_9f8c(puVar4, iVar5);
            iVar2.field34_0x24 = iVar1 * 0x3;
            mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000ffee, 0x2),
                in_stack_0000fe96,
                in_stack_0000ffba,
                in_stack_0000ffc0,
                in_stack_0000ffc4,
            );
            if (PTR_LOOP_1050_13ae < 0x3) {
                iVar1 = iVar2.field34_0x24;
                if (iVar1 < 0x32) {
                    iVar1 = 0x32;
                }
                iVar2.field34_0x24 = iVar1;
                return;
            }
        } else {
            iVar2.field34_0x24 = 0x64;
        }
    }
    return;
}


pub unsafe fn pass1_1030_c230(mut param_1: u32, mut param_2: u32) {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut in_stack_0000ffd8: HFILE16;
    let mut local_10: [u32; 0x2] = [0; 0x2];
    let mut local_8: [u16; 0x3] = [0; 0x3];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        uVar2 = (param_1 >> 0x10);
        local_10[0] = (param_1 + 0x20);
        BVar1 =
            write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_10), 0x4, in_stack_0000ffd8);
        if (BVar1 != 0) {
            local_8[0] = (param_1 + 0x24);
            BVar1 =
                write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_8), 0x2, in_stack_0000ffd8);
            if (BVar1 != 0) {
                return;
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}


pub unsafe fn pass1_1030_c29c(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut BVar1: bool;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar1 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x20)), 0x4);
        if (BVar1 != 0) {
            BVar1 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x24)), 0x2);
            if (BVar1 != 0) {
                return;
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}
