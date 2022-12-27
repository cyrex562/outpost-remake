
pub fn pass1_1028_9908(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1028_9944(
    param_1: *mut astruct_97,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_4;
    iVar1.field262_0x10c = param_3;
    iVar1.field264_0x110 = param_2;
    iVar1.field265_0x114 = 0;
    param_1.offset_0x0 = 0x9c52;
    iVar1.segment_0x2 = 0x1028;
    return;
}
pub fn pass1_1028_9992(param_1: *mut u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x9c52;
    (iVar1 + 0x2) = 0x1028;
    fn_ptr_1000_17ce(*(iVar1 + 0x114));
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}

pub fn pass1_1028_99c4(mut param_1: i16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_3 >> 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x10c));
    pass1_1030_355c((param_1 + 0x1f6), (param_3 + 0x114));
    return;
}


pub fn pass1_1028_9a02(
    mut param_1: u32,
    mut param_2: i16,
    param_3: *mut StructD,
    mut param_4: u16,
) {
    let mut lVar1: i32;
    let mut bVar2: bool;
    let mut uVar3: u16;
    let mut paVar4: *mut astruct_92;
    let mut uVar5: u32;
    let mut puVar6: *mut u8;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut paVar12: *mut Struct27;
    let mut paVar13: *mut astruct_67;
    let mut in_stack_0000fe62: u16;
    let mut in_stack_0000fe70: u16;
    let mut in_stack_0000ff86: u16;
    let mut in_stack_0000ff8c: u16;
    let mut in_stack_0000ff90: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ff9e: u16;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut iVar18: i16;
    let mut local_30: *mut astruct_92;
    let mut paVar9: *mut Struct57;

    uVar11 = (param_1 >> 0x10);
    iVar10 = param_1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar10 + 0x108));
    puVar6 = param_3;
    uVar5 = (param_2 + 0x1f6);
    pass1_1030_3694(puVar6, uVar5, 0x0, (iVar10 + 0x110));
    uVar7 = uVar5;
    (iVar10 + 0x114) = uVar7;
    (iVar10 + 0x116) = param_3;
    pass1_1030_38b8();
    uVar7 = param_3 | uVar7;
    paVar9 = (param_3 & 0xffff0000 | uVar7);
    if (uVar7 == 0) {
        lVar1 = (param_2 + 0x200);
        paVar12 = mixed_1010_20ba(
            paVar9,
            _u16_1050_0ed0,
            CONCAT22(param_4, 0x2b),
            in_stack_0000fe70,
            in_stack_0000ff94,
            in_stack_0000ff9a,
            in_stack_0000ff9e,
        );
        uVar11 = (paVar9 >> 0x10);
        if (lVar1 == 0x8000002) {
            iVar10 = 0x1f;
        } else {
            iVar10 = 0xb;
        }
        pass1_1010_043a(paVar12, (param_2 + 0x4), iVar10);
        if (lVar1 == 0x8000001) {
            uVar3 = 0x2;
        } else {
            uVar3 = 0x1;
        }
        paVar9 = CONCAT22(uVar11, 0x800);
        pass1_1038_349e(CONCAT22(puVar6, param_2), CONCAT22(0x800, uVar3));
        bVar2 = false;
        pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_30)), 0x1, 0x0, 0x400);
        loop {
            paVar4 = &local_30;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
            uVar7 = paVar9;
            uVar8 = uVar7 | paVar4;
            paVar9 = (paVar9 & 0xffff0000 | uVar8);
            if (uVar8 == 0) {
                break;
            }
            if (paVar4[0x1c].field4_0x8 != 0x8000002) {
                bVar2 = true;
            }
        }
        if (!bVar2) {
            uVar17 = 0;
            iVar18 = 0x3c;
            uVar14 = 0x1;
            uVar15 = 0;
            uVar16 = 0;
            uVar3 = 0;
            iVar10 = 0;
            uVar11 = 0;
            paVar13 = mixed_1010_20ba(
                paVar9,
                _u16_1050_0ed0,
                0x37,
                in_stack_0000fe62,
                in_stack_0000ff86,
                in_stack_0000ff8c,
                in_stack_0000ff90,
            );
            post_win_msg_1008_a0e4(
                paVar13,
                CONCAT22(uVar3, uVar11),
                iVar10,
                CONCAT11(uVar15, uVar14),
                CONCAT22(uVar17, uVar16),
                iVar18,
            );
        }
    }
    return;
}


pub fn pass1_1028_9b48(
    param_1: *mut astruct_330,
    param_2: *mut u8,
    param_3: *mut astruct_331,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_331;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x118, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    iVar5 = param_3;
    uVar8 = (param_3 >> 0x10);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
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
        param_1.field258_0x10c = iVar5.field259_0x10c;
        param_1.field259_0x110 = iVar5.field260_0x110;
        param_1.field260_0x114 = iVar5.field261_0x114;
        *puStack10 = 0x9c52;
        param_1.field2_0x2 = 0x1028;
    }
    iVar5.field261_0x114 = 0;
    return;
}


pub fn pass1_1028_9c2c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_9992(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1028_9c90(mut param_1: u32) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar1 = (param_1 + 0x108) - 0x3e8;
    if ((uVar1 < 0x3a99) && (uVar1 % 0x3e8 == 0)) {
        // WARNING: Could not recover jumptable at 0x10289dc0. Too many branches
        // WARNING: Treating indirect jump as call
        uVar2 = ((uVar1 / 0x3e8) * 0x2 -0x623a);
        return uVar2;
    }
    return 0x1;
}


pub fn pass1_1028_9dee(
    param_1: *mut astruct_332,
    mut param_2: u16,
    param_3: *mut astruct_333,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_333;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x10a, paVar5);
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
        *puStack10 = 0x9eb6;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub fn pass1_1028_9e8a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_9ec6(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, s_noth_bmp_1050_2321 + 0x6);
    param_1.offset_0x0 = 0xa6f6;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)), 0x105050f0);
    return param_1;
}


pub fn pass1_1028_9efc(
    param_1: *mut u16,
    param_2: *mut StructD,
    mut param_3: u32,
    mut param_4: u16,
) {
    let mut lVar1: i32;
    let mut iVar2: i16;
    let mut paVar3: *mut astruct_92;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut lVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut paVar12: *mut Struct57;
    let mut paVar13: *mut astruct_67;
    let mut paVar14: *mut astruct_690;
    let mut paVar15: *mut Struct27;
    let mut in_stack_0000fe6e: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff98: u16;
    let mut in_stack_0000ff9c: u16;
    let mut uVar16: u16;
    let mut local_18: *mut astruct_92;
    let mut paVar11: *mut Struct57;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
    uVar8 = param_2 | param_1;
    paVar11 = (param_2 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_18)), 0x1, 0x0, 0x400);
        loop {
            paVar3 = &local_18;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar3));
            uVar8 = paVar11;
            paVar12 = (paVar11 & 0xffff0000 | (uVar8 | paVar3));
            if ((uVar8 | paVar3) == 0) {
                break;
            }
            lVar1 = paVar3[0x1c].field4_0x8;
            uVar4 = (&paVar3[0x1c].field4_0x8 + 2);
            paVar11 = (paVar11 & 0xffff0000 | uVar4);
            if ((&paVar3[0x1c].field3_0x4 + 0x2) != 0) {
                uVar16 = (param_3 >> 0x10);
                lVar7 = lVar1;
                if ((lVar1 != 0x2) || (uVar4 != 0x800)) {
                    pass1_1028_a3ae(lVar1, paVar11, param_3, uVar16, CONCAT22(uVar8, paVar3));
                }
                uVar4 = lVar7;
                pass1_1028_a28a(param_3, uVar16, CONCAT22(uVar8, paVar3));
                if ((paVar11 < 1) && (paVar11 < 0x0 || (uVar4 < 0x64))) {
                    pass1_1028_a4ee(param_3, CONCAT22(uVar8, paVar3));
                }
                if (lVar1 != 0x8000002) {
                    pass1_1038_42cc(CONCAT22(uVar8, paVar3));
                    uVar9 = paVar11 | uVar4;
                    paVar11 = (paVar11 & 0xffff0000 | uVar9);
                    if (uVar9 != 0) {
                        paVar13 = mixed_1010_20ba(
                            paVar11,
                            _u16_1050_0ed0,
                            CONCAT22(param_4, 0x37),
                            in_stack_0000fe6e,
                            in_stack_0000ff92,
                            in_stack_0000ff98,
                            in_stack_0000ff9c,
                        );
                        paVar11 = (paVar11 & 0xffff0000 | paVar13 >> 0x10);
                        post_win_msg_1008_a0e4(
                            paVar13,
                            0x0,
                            uVar4,
                            paVar3[0x1c].field6_0x10,
                            paVar3.field3_0x4,
                            0x2,
                        );
                    }
                }
            }
        }
        local_18 = 0x389a;
        local_18.field2_0x2 = 0x1008;
        paVar14 = mixed_1010_20ba(
            paVar12,
            _u16_1050_0ed0,
            CONCAT22(param_4, 0x8),
            in_stack_0000fe6e,
            in_stack_0000ff92,
            in_stack_0000ff98,
            in_stack_0000ff9c,
        );
        paVar11 = (paVar12 & 0xffff0000 | paVar14 >> 0x10);
        iVar2 = paVar14;
        iVar5 = iVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
        uVar10 = SUB42(paVar11, 0x0);
        iVar6 = iVar5;
        pass1_1010_9f72(paVar14, 0x3e);
        if (iVar6 != 0) {
            iVar6 = pass1_1010_96d0(paVar14);
            if (iVar6 < 1) {
                if (iVar6 < 0x0) {
                    iVar6 = (iVar5 + 0x1f6);
                    pass1_1030_38b8();
                    if ((paVar11 < 1) && (paVar11 < 0x0 || (iVar6 == 0))) {
                        paVar13 = mixed_1010_20ba(
                            paVar11,
                            _u16_1050_0ed0,
                            CONCAT22(iVar2, 0x37),
                            in_stack_0000fe6e,
                            in_stack_0000ff92,
                            in_stack_0000ff98,
                            in_stack_0000ff9c,
                        );
                        post_win_msg_1008_a0e4(paVar13, 0x0, 0x0, 0x1, (iVar5 + 0x4), 0x6);
                    }
                }
            } else {
                paVar13 = mixed_1010_20ba(
                    paVar11,
                    _u16_1050_0ed0,
                    CONCAT22(iVar2, 0x37),
                    in_stack_0000fe6e,
                    in_stack_0000ff92,
                    in_stack_0000ff98,
                    in_stack_0000ff9c,
                );
                paVar11 = (paVar11 & 0xffff0000 | paVar13 >> 0x10);
                post_win_msg_1008_a0e4(paVar13, 0x0, iVar6, (iVar5 + 0x208), 0x4000001, 0x2);
                paVar15 = mixed_1010_20ba(
                    paVar11,
                    _u16_1050_0ed0,
                    CONCAT22(iVar2, 0x2b),
                    in_stack_0000fe6e,
                    in_stack_0000ff92,
                    in_stack_0000ff98,
                    in_stack_0000ff9c,
                );
                pass1_1010_043a(paVar15, (iVar5 + 0x4), 0x14);
            }
        }
    }
    return;
}

pub fn pass1_1028_a0fa(param_1: *mut astruct_334, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xa6f6;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub fn pass1_1028_a6ca(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1028_a706(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0xbb7);
    param_1.offset_0x0 = 0xa856;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCPrelimAlloc_1050_50f6,
    );
    return param_1;
}


pub fn pass1_1028_a73c(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut paVar2: *mut astruct_92;
    let mut uVar3: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        uVar3 = param_1 | paVar1;
        if (uVar3 == 0) {
            break;
        }
        paVar2 = paVar1;
        pass1_1038_5464(paVar1, CONCAT22(param_1, paVar1));
        pass1_1038_56d6(CONCAT22(param_1, paVar1), 0x0);
        pass1_1038_518c(paVar2, CONCAT22(param_1, paVar1));
        param_1 = uVar3;
    }
    return 0x1;
}


pub fn pass1_1028_a79c(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        (param_1 + 0x2) = 0x1008;
        uVar8 = (param_3 >> 0x10);
        (param_1 + 0x4) = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = (param_1 + 0x8);
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        *puStack10 = 0xa856;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}


pub fn pass1_1028_a82a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a866(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x36af);
    param_1.offset_0x0 = 0xa9ae;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCProdSched_1050_5104,
    );
    return param_1;
}


pub fn pass1_1028_a89c(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut uVar2: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        uVar2 = param_1;
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        param_1 = uVar2 | paVar1;
        if (param_1 == 0) {
            break;
        }
        if (paVar1[0x1c].field4_0x8 != 0x8000002) {
            pass1_1038_3fca(paVar1, CONCAT22(uVar2, paVar1));
        }
    }
    return 0x1;
}


pub fn pass1_1028_a8f4(param_1: *mut astruct_335, mut param_2: u16, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xa9ae;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub fn pass1_1028_a982(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a9be(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x176f);
    param_1.offset_0x0 = 0xab22;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCPower_1050_5110,
    );
    return param_1;
}


pub fn pass1_1028_a9f4(mut param_1: u16) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_92;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut puStack24: *mut u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        puStack24 = CONCAT22(param_1, paVar2);
        uVar4 = param_1 | paVar2;
        if (uVar4 == 0) {
            break;
        }
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0, &paVar2.field5_0xc, 0xc);
        param_1 = uVar4;
        if (BVar3 != 0) {
            ppcVar1 = (*puStack24 + 0x34);
            (**ppcVar1)(0x1008, paVar2);
            param_1 = extraout_DX;
        }
    }
    return 0x1;
}


pub fn pass1_1028_aa68(param_1: *mut astruct_336, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xab22;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub fn pass1_1028_aaf6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_ab32(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x2edf);
    param_1.offset_0x0 = 0xaca6;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCRchSched_1050_5118,
    );
    return param_1;
}


pub fn pass1_1028_ab68(mut param_1: u16) -> u16 {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_92;
    let mut BVar4: bool;
    let mut uVar5: u16;
    let mut extraout_DX: u16;
    let mut puStack24: *mut u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_14)), 0x1, 0x0, 0x700); //
                                                                                 // LAB_1028_ab7e:
    uVar5 = param_1;
    paVar3 = &local_14;
    pass1_1028_e4ec(CONCAT22(0x1050, paVar3));
    puStack24 = CONCAT22(uVar5, paVar3);
    param_1 = uVar5 | paVar3;
    if (param_1 == 0) {
        return 0x1;
    }
    uVar1 = &paVar3.field5_0xc;
    BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x11);
    //  if (BVar4 == 0) goto code_r0x1028abad;
    //   goto LAB_1028_abc0;
    // code_r0x1028abad:
    BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x12);
    if (BVar4 != 0) {
        //
        // LAB_1028_abc0:
        if ((paVar3 + 1) == 0x5) {
            ppcVar2 = (*puStack24 + 0x30);
            (**ppcVar2)(0x1008);
            param_1 = extraout_DX;
        }
    }
    //   goto LAB_1028_ab7e;
}


pub fn pass1_1028_abec(param_1: *mut astruct_337, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xaca6;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub fn pass1_1028_ac7a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_acb6(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x3e7f);
    param_1.offset_0x0 = 0xae56;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCSetup_1050_5124,
    );
    return param_1;
}


pub fn pass1_1028_acec(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut paVar2: *mut astruct_92;
    let mut uVar3: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        uVar3 = param_1;
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        param_1 = uVar3 | paVar1;
        if (param_1 == 0) {
            break;
        }
        paVar2 = paVar1;
        vsprintf_op_1030_840a(param_1, s_SCSetup__calcMe_clearing_colony_0_1050_512c);
        if (paVar1[0x1c].field4_0x8 != 0x8000002) {
            pass1_1038_5464(paVar2, CONCAT22(uVar3, paVar1));
            pass1_1038_56d6(CONCAT22(uVar3, paVar1), 1);
        }
    }
    local_14 = 0x389a;
    local_14.field2_0x2 = 0x1008;
    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x800);
    loop {
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        uVar3 = param_1 | paVar1;
        if (uVar3 == 0) {
            break;
        }
        pass1_1030_2690(CONCAT22(param_1, paVar1));
        param_1 = uVar3;
    }
    return 0x1;
}
