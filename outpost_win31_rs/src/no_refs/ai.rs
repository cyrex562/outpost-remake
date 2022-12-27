pub fn pass1_1030_5290(
    param_1: *mut astruct_376,
    param_2: *mut u8,
    param_3: *mut astruct_377,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_377;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: *mut astruct_377;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x10c, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        iVar5 = param_3;
        param_1.field3_0x4 = iVar5.field4_0x4;
        puVar6 = &iVar5.field5_0x8;
        puVar7 = &param_1.field4_0x8;
        for iVar3 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        param_1.field257_0x108 = iVar5.field258_0x108;
        *puStack10 = 0x55fe;
        param_1.field2_0x2 = 0x1030;
    }
    return;
}


pub fn pass1_1030_538a(param_1: *mut astruct_694) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut iVar4: *mut astruct_694;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000fff0: u16;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar1 = (&iVar4.field264_0x108 + 2);
    uVar2 = uVar1 >> 0x8;
    puVar4 = mixed_1010_20ba(
        (in_EDX & 0xffff0000 | uVar1),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff0, 0x2f),
        in_stack_0000fe98,
        in_stack_0000ffbc,
        in_stack_0000ffc2,
        in_stack_0000ffc6,
    );
    if (uVar2 == 1) {
        pass1_1018_04ca(puVar4, iVar4.field264_0x108);
    } else if (uVar2 == 0x2) {
        pass1_1018_04a4(puVar4, iVar4.field264_0x108);
    }
    return 0x1;
}


pub fn pass1_1030_53f4(mut param_1: u16, mut param_2: u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut bStack291: u8;
    let mut local_11e: [u8; 0x10e] = [0; 0x10e];
    let mut uStack16: u32;
    let mut uStack12: u32;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    uVar6 = (param_2 >> 0x10);
    iVar5 = param_2;
    uStack12 = (iVar5 + 0x108);
    uStack12._3_1_ = (uStack12 >> 0x18);
    if (uStack12._3_1_ == -1) {
        uVar7 = pass1_1028_e2e0(paVar4, _PTR_LOOP_1050_65e2, ((iVar5 + 0x108) >> 0x18));
        uVar3 = (uVar7 >> 0x10);
    } else {
        uStack16 = (iVar5 + 0x108);
        uStack16._3_1_ = (uStack16 >> 0x18);
        if (uStack16._3_1_ == '\x03') {
            pass1_1028_e44a(_PTR_LOOP_1050_65e2, (iVar5 + 0x108));
            uVar3 = SUB42(paVar4, 0x0);
        } else {
            uVar1 = (iVar5 + 0x108);
            pass1_1028_e372(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
            uVar3 = SUB42(paVar4, 0x0);
        }
    }
    uStack12 = (iVar5 + 0x108);
    uStack12._3_1_ = (uStack12 >> 0x18);
    if (uStack12._3_1_ != '\x03') {
        pass1_1030_521c(CONCAT13(0x10, CONCAT12(0x50, local_11e)), (iVar5 + 0x108));
        uStack16 = *_u16_1050_5748;
        fn_ptr_1028_d566(uStack16, CONCAT22(0x1050, local_11e));
        bStack291 = ((iVar5 + 0x108) >> 0x18);
        uVar2 = bStack291;
        if (bStack291 == 0x2) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar5 + 0x108));
            pass1_1010_82f8(_u16_1050_14cc, *(uVar2 + 0x10));
        }
    }
    return;
}


pub fn pass1_1030_54f8(
    param_1: *mut astruct_378,
    param_2: *mut u8,
    param_3: *mut astruct_379,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_379;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: *mut astruct_379;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x10c, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        iVar5 = param_3;
        param_1.field3_0x4 = iVar5.field4_0x4;
        puVar6 = &iVar5.field5_0x8;
        puVar7 = &param_1.field4_0x8;
        for iVar3 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        param_1.field257_0x108 = iVar5.field258_0x108;
        *puStack10 = 0x55ee;
        param_1.field2_0x2 = 0x1030;
    }
    return;
}


pub fn pass1_1030_5596(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_55c2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_56f6(param_1: u16, param_2: *mut astruct_731, mut param_3: u32) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut BVar5: bool;
    let mut iVar6: i16;
    let mut iVar7: *mut astruct_731;
    let mut uVar8: *mut astruct_731;
    let mut in_stack_0000ffd6: HFILE16;
    let mut local_e: [u16; 0x3] = [0; 0x3];
    let mut local_8: [u16; 0x2] = [0; 0x2];
    let mut iStack4: i16;

    uVar4 = pass1_1030_1978(param_1, param_2, param_3);
    if (uVar4 != 0) {
        uVar8 = (param_2 >> 0x10);
        iVar7 = param_2;
        local_e[0] = *&iVar7.field_0x10;
        BVar5 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_e), 0x2, in_stack_0000ffd6);
        if (BVar5 != 0) {
            uVar3 = &iVar7.field_0x10;
            local_8[0] = (uVar3 + 2);
            BVar5 =
                write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_8), 0x2, in_stack_0000ffd6);
            if ((BVar5 != 0)
                && (
                    uVar3 = &iVar7.field_0x10,
                    BVar5 = pass1_1008_7c2a(param_3, *(uVar3 + 0x4)),
                    BVar5 != 0,
                ))
            {
                uVar3 = &iVar7.field_0x10;
                local_8[0] = (uVar3 + 0x1a);
                BVar5 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, local_8),
                    0x2,
                    in_stack_0000ffd6,
                );
                if (BVar5 != 0) {
                    //   for (iStack4 = 0; uVar3 = &iVar7.field_0x10, piVar1 = (uVar3 + 0x1a),
                    //   *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 1)
                    iStack4 = 0;
                    uVar3 = &iVar7.field_0x10;
                    piVar1 = uVar3 + 0x1a;
                    while *piVar1 != iStack4 && iStack4 <= *piVar1 {
                        uVar3 = &iVar7.field_0x10;
                        uVar2 = (uVar3 + 0x16);
                        iVar6 = write_to_file_1008_7b4c(
                            param_3,
                            (uVar2 & 0xffff0000 | (uVar2 + iStack4 * 0x6)),
                        );
                        //            if (iVar6 == 0) goto LAB_1030_5734;
                        iStack4 += 1;
                    }
                    iVar6 = write_to_file_1008_7b4c(
                        param_3,
                        (param_2 & 0xffff0000 | ZEXT24(&iVar7.field19_0x14)),
                    );
                    if (iVar6 != 0) {
                        local_8[0] = &iVar7.field_0x1c;
                        BVar5 = write_to_file_1008_7e1c(
                            param_3,
                            CONCAT22(0x1050, local_8),
                            0x2,
                            in_stack_0000ffd6,
                        );
                        if (BVar5 != 0) {
                            return;
                        }
                    }
                }
            }
        } //
          // LAB_1030_5734:
        u16_1050_0310 = 0x6d0;
    }
    return;
}


pub fn file_1030_581e(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_381,
    mut param_4: u32,
) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut paVar4: *mut astruct_380;
    let mut BVar5: bool;
    let mut puVar6: *mut u8;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut in_register_0000000a: u16;
    let mut pSVar11: *mut StructD;
    let mut paVar13: *mut Struct57;
    let mut iVar9: *mut astruct_380;
    let mut uVar14: u16;
    let mut in_stack_0000fae2: u16;
    let mut uStack1040: u32;
    let mut iStack1036: i16;
    let mut local_408: [u8; 0x400] = [0; 0x400];
    let mut uStack8: u32;
    let mut local_4: i16;
    let mut uVar15: *mut astruct_381;
    let mut iVar12: *mut astruct_381;
    let mut paVar12: *mut Struct57;

    pSVar11 = CONCAT22(in_register_0000000a, param_2);
    iVar12 = param_3;
    uVar15 = (param_3 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar11);
        } else {
            pSVar11 = (pSVar11 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
        }
        paVar4 = fn_ptr_op_1000_1708(0x20, 0x0, 0x1, PTR_LOOP_1050_5f2c, pSVar11);
        uVar9 = pSVar11 | paVar4;
        paVar13 = (pSVar11 & 0xffff0000);
        paVar12 = (paVar13 | uVar9);
        if (uVar9 == 0) {
            paVar4 = null_mut();
        } else {
            pass1_1030_84ae(CONCAT22(pSVar11, paVar4));
            paVar13 = paVar12;
        }
        iVar12.field16_0x10 = paVar4;
        iVar12.field17_0x12 = paVar13;
        BVar5 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar5 != 0) {
            uVar8 = (_PTR_LOOP_1050_65e2 + 0x52);
            uStack8 = uVar8;
            pass1_1030_4782(
                paVar13,
                uVar8,
                (uVar8 >> 0x10),
                0x0,
                0x1,
                local_4,
                in_stack_0000fae2,
            );
            iVar12.field16_0x10 = uVar8;
            iVar12.field17_0x12 = paVar13;
            BVar5 = read_file_1008_7dee(
                param_4,
                CONCAT22(paVar13, &iVar12.field16_0x10.field_0x2),
                0x2,
            );
            if (BVar5 != 0) {
                puVar6 = local_408;
                read_file_1008_7c6e(param_4, (param_4 >> 0x10), CONCAT22(0x1050, puVar6));
                if (puVar6.is_null() == false) {
                    uVar8 = &iVar12.field16_0x10;
                    fn_ptr_1000_17ce(*(uVar8 + 0x4));
                    uVar7 = str_op_1008_60e8(paVar13, CONCAT22(0x1050, local_408));
                    uVar8 = &iVar12.field16_0x10;
                    uVar14 = (uVar8 >> 0x10);
                    iVar9 = uVar8;
                    iVar9.field4_0x4 = uVar7;
                    iVar9.field5_0x6 = paVar13;
                    uVar3 = &iVar12.field16_0x10;
                    BVar5 =
                        read_file_1008_7dee(param_4, (uVar3 & 0xffff0000 | (uVar3 + 0x1a)), 0x2);
                    if (BVar5 != 0) {
                        uVar8 = &iVar12.field16_0x10;
                        iVar2 = (uVar8 + 0x1a);
                        uVar9 = iVar2 * 0x6;
                        mem_op_1000_179c(uVar9, paVar13);
                        uVar10 = paVar13;
                        uStack1040 = CONCAT22(uVar10, uVar9);
                        if ((uVar10 | uVar9) == 0) {
                            uVar8 = &iVar12.field16_0x10;
                            (uVar8 + 0x16) = 0;
                        } else {
                            pass1_1000_5586(0x3e38, 0x1008, iVar2, 0x6, uVar9, uVar10);
                            uVar8 = &iVar12.field16_0x10;
                            (uVar8 + 0x16) = uStack1040;
                        }
                        // for (iStack1036 = 0; uVar8 = &iVar12.field16_0x10, piVar1 = (uVar8 + 0x1a),
                        //     *piVar1 != iStack1036 && iStack1036 <= *piVar1; iStack1036 += 1)
                        iStack1036 = 0;
                        uVar8 = &iVar12.field16_0x10;
                        piVar1 = uVar8 + 0x1a;
                        while *piVar1 != iStack1036 && iStack1036 <= *piVar1 {
                            uVar8 = &iVar12.field16_0x10;
                            uVar3 = (uVar8 + 0x16);
                            BVar5 = read_file_1008_7bc8(
                                param_4,
                                (uVar3 & 0xffff0000 | (uVar3 + iStack1036 * 0x6)),
                            );
                            //              if (BVar5 == 0) goto LAB_1030_58a7;
                            iStack1036 += 1;
                        }
                        BVar5 = read_file_1008_7bc8(
                            param_4,
                            (param_3 & 0xffff0000 | ZEXT24(&iVar12.field_0x14)),
                        );
                        if ((BVar5 != 0)
                            && (
                                BVar5 = read_file_1008_7dee(
                                    param_4,
                                    (param_3 & 0xffff0000 | ZEXT24(&iVar12.field_0x1c)),
                                    0x2,
                                ),
                                BVar5 != 0,
                            ))
                        {
                            return;
                        }
                    }
                }
            }
        } //
          // LAB_1030_58a7:
        u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn pass1_1030_5baa(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_56b0(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_5dbe(param_1: u16, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut BVar4: bool;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut in_stack_0000ffde: HFILE16;
    let mut local_c: [u16; 0x5] = [0; 0x5];

    uVar3 = pass1_1030_1978(param_1, param_2, param_3);
    if (uVar3 != 0) {
        uVar7 = (param_2 >> 0x10);
        iVar6 = param_2;
        BVar4 = pass1_1008_7c2a(param_3, *(iVar6 + 0x10));
        if ((BVar4 != 0)
            && (
                uVar1 = (iVar6 + 0x10),
                iVar5 = write_to_file_1008_7b4c(param_3, (uVar1 & 0xffff0000 | (uVar1 + 0x4))),
                iVar5 != 0,
            ))
        {
            uVar2 = (iVar6 + 0x10);
            local_c[0] = (uVar2 + 0xa);
            BVar4 =
                write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffde);
            if (BVar4 != 0) {
                uVar2 = (iVar6 + 0x10);
                if ((uVar2 + 0xa) == 0) {
                    return;
                }
                uVar2 = (iVar6 + 0x10);
                uVar7 = (uVar2 >> 0x10);
                iVar6 = uVar2;
                BVar4 = write_to_file_1008_7e1c(
                    param_3,
                    (iVar6 + 0xc),
                    ((iVar6 + 0xa) * 0x2),
                    in_stack_0000ffde,
                );
                if (BVar4 != 0) {
                    return;
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn pass1_1030_6118(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_5d78(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_6f5a(mut param_1: u32) {
    let mut uVar1: u16;
    let mut BVar2: bool;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x4);
    if (BVar2 != 0) {
        pass1_1028_6302((param_1 + 0x1a));
    }
    return;
}

pub fn pass1_1030_6fd4(mut param_1: u32) {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar2 << 0x10), in_AX, in_DX);
    }
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 0x5) {
        return;
    }
    return;
}

pub fn pass1_1030_701c(mut param_1: u32) {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar2 << 0x10), in_AX, in_DX);
    }
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 0x5) {
        return;
    }
    return;
}


pub fn pass1_1030_7176(mut param_1: u32) {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut local_1a: i32;
    let mut local_16: [i16; 0x2] = [0; 0x2];
    let mut uStack18: u16;
    let mut uStack14: u16;
    let mut BStack10: bool;
    let mut uStack8: u16;
    let mut lStack6: i32;

    lStack6 = 0;
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x22) == 0) {
        return;
    }
    if ((iVar2 + 0x1a) == 0) {
        struct_op_1030_73a8(param_1, in_AX, in_DX);
    }
    uVar1 = (iVar2 + 0x1a);
    uStack8 = (uVar1 + 0xc);
    BStack10 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x3);
    if ((BStack10 != 0) && (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 0x5)) {
        uVar1 = (iVar2 + 0x22);
        uStack14 = (uVar1 + 0x4);
        for uStack18 in 0..uStack14 {
            pass1_1020_bb16(
                (iVar2 + 0x22),
                CONCAT22(0x1050, &local_1a),
                CONCAT22(0x1050, local_16),
                uStack18,
            );
            if (0x0 < local_16[0]) {
                lStack6 += local_1a;
            }
        }
    }
    return;
}



pub fn pass1_1030_7226(mut param_1: u32) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut in_AX: u16;
    let mut BVar3: bool;
    let mut in_DX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar5 << 0x10), in_AX, in_DX);
    }
    uVar2 = (iVar4 + 0x1a);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0, (uVar2 + 0xc), 0x10);
    if (((BVar3 != 0) && (uVar2 = (iVar4 + 0x1a), (uVar2 + 0x12) == 0x5))
        && (
            uVar1 = (iVar4 + 0x1a),
            uVar2 = (uVar1 & 0xffff0000 | (uVar1 + 0x14)),
            (uVar2 + 0xa4) == 0x1e,
        ))
    {
        return;
    }
    return;
}


pub fn pass1_1030_7418(mut param_1: i16, param_2: *mut astruct_731, mut param_3: u32) {
    let mut uVar1: u32;
    let mut iVar2: *mut astruct_731;
    let mut iVar3: i16;
    let mut BVar4: bool;
    let mut puVar5: *mut u8;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar6: u16;
    let mut in_stack_0000ffac: HFILE16;
    let mut uStack62: u16;
    let mut local_2a: [u16; 0x2] = [0; 0x2];
    let mut local_26: [u8; 0xe] = [0; 0xe];
    let mut local_18: u32;
    let mut local_14: [u32; 0x2] = [0; 0x2];
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: [u16; 0x2] = [0; 0x2];

    pass1_1030_16d6(param_2, param_3);
    if (param_1 == 0) {
        return;
    }
    iVar2 = param_2;
    iVar2 = &iVar2.field_0xc;
    iVar3 = write_to_file_1008_7b4c(param_3, (param_2 & 0xffff0000 | ZEXT24(iVar2)));
    if (iVar3 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    uVar6 = (param_2 >> 0x10);
    local_c = iVar2.field18_0x12;
    BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, &local_c), 0x2, in_stack_0000ffac);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    local_6[0] = iVar2.field19_0x14;
    BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffac);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    local_18 = iVar2.field20_0x16;
    BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, &local_18), 0x4, in_stack_0000ffac);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7954(BVar4, param_3, iVar2.field25_0x1e);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_3, iVar2.field26_0x22);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_3, iVar2.field27_0x26);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    local_a = iVar2.field28_0x2a;
    BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, &local_a), 0x4, in_stack_0000ffac);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    local_c = iVar2.field33_0x32;
    BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, &local_c), 0x2, in_stack_0000ffac);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    local_c = iVar2.field34_0x34;
    BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, &local_c), 0x2, in_stack_0000ffac);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_79f0(param_3, iVar2.field35_0x36);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    if (iVar2.field36_0x3a == 0) {
        local_18 &= 0xffff0000;
    } else {
        uVar1 = iVar2.field36_0x3a;
        local_18 = local_18 & 0xffff0000 | (uVar1 + 0x8);
    }
    local_6[0] = local_18;
    BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffac);
    if (BVar4 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_5784(CONCAT22(0x1050, local_26), iVar2.field36_0x3a);
    loop {
        puVar5 = local_26;
        pass1_1008_5b12(CONCAT22(0x1050, puVar5));
        local_14[0] = CONCAT22(extraout_DX, puVar5);
        if ((extraout_DX | puVar5) == 0) {
            if (iVar2.field37_0x3e == 0) {
                uStack62 = 0;
            } else {
                uVar1 = iVar2.field37_0x3e;
                uStack62 = (uVar1 + 0x8);
            }
            local_2a[0] = uStack62;
            BVar4 = write_to_file_1008_7e1c(
                param_3,
                CONCAT22(0x1050, local_2a),
                0x2,
                in_stack_0000ffac,
            );
            if (BVar4 == 0) {
                u16_1050_0310 = 0x6d0;
                return;
            }
            pass1_1008_5784(CONCAT22(0x1050, local_26), iVar2.field37_0x3e);
            loop {
                puVar5 = local_26;
                pass1_1008_5b12(CONCAT22(0x1050, puVar5));
                if ((extraout_DX_00 | puVar5) == 0) {
                    return;
                }
                local_18 = local_18 & 0xffff0000 | (puVar5 + 0x4);
                BVar4 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, &local_18),
                    0x2,
                    in_stack_0000ffac,
                );
                if (BVar4 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
                local_14[0] = local_14[0] & 0xffff0000 | (puVar5 + 0x6);
                BVar4 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, local_14),
                    0x2,
                    in_stack_0000ffac,
                );
                if (BVar4 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
                local_c = (puVar5 + 0x8);
                BVar4 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, &local_c),
                    0x2,
                    in_stack_0000ffac,
                );
                if (BVar4 == 0) {
                    break;
                }
                local_c = (puVar5 + 0xa);
                BVar4 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, &local_c),
                    0x2,
                    in_stack_0000ffac,
                );
                if (BVar4 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
                local_6[0] = (puVar5 + 0xc);
                BVar4 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, local_6),
                    0x2,
                    in_stack_0000ffac,
                );
                if (BVar4 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
            }
            u16_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (puVar5 + 0x4);
        BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffac);
        if (BVar4 == 0) {
            u16_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0x6);
        BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffac);
        if (BVar4 == 0) {
            break;
        }
        local_6[0] = (local_14[0] + 0x8);
        BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffac);
        if (BVar4 == 0) {
            u16_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0xa);
        BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffac);
        if (BVar4 == 0) {
            u16_1050_0310 = 0x6d0;
            return;
        }
        local_6[0] = (local_14[0] + 0xc);
        BVar4 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffac);
        if (BVar4 == 0) {
            u16_1050_0310 = 0x6d0;
            return;
        }
    }
    u16_1050_0310 = 0x6d0;
    return;
}
