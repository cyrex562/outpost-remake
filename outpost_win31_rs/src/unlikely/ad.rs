use crate::block_1028::block_1028_7000::{pass1_1028_75bc, pass1_1028_767e, pass1_1028_78b8, pass1_1028_7c4e, pass1_1028_7dfc, pass1_1028_7fb6, post_msg_1028_76da};

pub unsafe fn pass1_1028_6e60(param_1: u8, param_2: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_2, 0x32c7);
    param_2.offset_0x0 = 0x6fb0;
    (param_2 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | (param_2 + 0x8)),
        s_SCConstruct_1050_4fdc,
    );
    return param_2;
}


pub unsafe fn pass1_1028_6e96(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut ppcVar2: *mut *mut code;
    let mut ppaVar3: *mut *mut astruct_92 = null_mut();
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut puStack24: *mut u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        ppaVar3 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, ppaVar3));
        puStack24 = CONCAT22(param_1, ppaVar3);
        uVar4 = param_1 | ppaVar3;
        if (uVar4 == 0) {
            break;
        }
        paVar1 = ppaVar3[0x9];
        param_1 = uVar4;
        if (((0x0 < paVar1) && (!SBORROW2(paVar1, 1))) && ((&paVar1[-0x1].field6_0x10 + 1) < 0x4)) {
            ppcVar2 = (*puStack24 + 0x38);
            (**ppcVar2)();
            param_1 = extraout_DX;
        }
    }
    return 0x1;
}


pub unsafe fn pass1_1028_6ef6(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
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
        // for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1)
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        *puStack10 = 0x6fb0;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}


pub unsafe fn pass1_1028_6f84(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1028_6fc0(param_1: u8, param_2: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_2, 0x3e7);
    param_2.offset_0x0 = 0x749e;
    (param_2 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | (param_2 + 0x8)),
        s_SCEndSim_1050_4fea,
    );
    return param_2;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_6ff6(param_1: *mut StructD, mut param_2: u32, mut param_3: u16) {
    let mut bVar1: bool;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut paVar4: *mut astruct_92;
    let mut paVar5: *mut astruct_92;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut uVar9: u32;
    let mut unaff_DI: i16;
    let mut paVar10: *mut Struct27;
    let mut paVar11: *mut astruct_67;
    let mut paVar12: *mut astruct_102;
    let mut paVar13: *mut astruct_679;
    let mut in_stack_0000fe52: u16;
    let mut in_stack_0000fe5c: u16;
    let mut in_stack_0000fe60: u16;
    let mut in_stack_0000ff76: u16;
    let mut in_stack_0000ff7c: u16;
    let mut in_stack_0000ff80: u16;
    let mut in_stack_0000ff84: u16;
    let mut in_stack_0000ff86: u16;
    let mut in_stack_0000ff8a: u16;
    let mut in_stack_0000ff8e: u16;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut iVar16: i16;
    let mut uVar17: u8;
    let mut uVar18: u8;
    let mut uVar19: u16;
    let mut uVar20: u16;
    let mut uVar21: u16;
    let mut iVar22: i16;
    let mut local_46: *mut astruct_92;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_14)), 0x1, 0x0, 0x400);
    bVar3 = true;
    bVar2 = false;
    loop {
        loop {
            paVar4 = &local_14;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
            uVar7 = param_1;
            param_1 = (param_1 & 0xffff0000 | (uVar7 | paVar4));
            //      if ((uVar7 | paVar4) == 0) goto LAB_1028_7066;
            if !(((&paVar4[0x1c].field3_0x4 + 0x2) == 0) || (paVar4[0x1c].field4_0x8 == 0x8000002)) {
                break;
            }
        }
        bVar2 = true;
        iVar16 = &paVar4[0x1b].field6_0x10;
        pass1_1030_38b8();
        if !((param_1 < 0x0) || (param_1 < 0x1 && (iVar16 == 0))) {
            break;
        }
    }
    bVar3 = false; //
    // LAB_1028_7066:
    if (local_14.field6_0x10 == 0) {
        paVar8 = (param_1 & 0xffff0000 | local_14.field5_0xc);
        local_14.field4_0x8 = local_14.field5_0xc;
    } else {
        paVar8 = (param_1 & 0xffff0000);
        local_14.field4_0x8 = 0x1;
    }
    local_14.field4_0x8 = SUB42(paVar8, 0x0);
    bVar1 = false;
    loop {
        paVar4 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
        uVar7 = paVar8;
        uVar6 = uVar7 | paVar4;
        paVar8 = (paVar8 & 0xffff0000 | uVar6);
        if (uVar6 == 0) {
            break;
        }
        if (paVar4[0x1c].field4_0x8 == 0x8000001) {
            bVar1 = true;
        }
    }
    if (!bVar1) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
        uVar7 = paVar8 | paVar4;
        paVar8 = (paVar8 & 0xffff0000 | uVar7);
        if (uVar7 != 0) {
            PTR_LOOP_1050_4fe8 = (&PTR_LOOP_1050_0000 + 1);
            uVar20 = 0;
            iVar16 = 0x1;
            paVar10 = mixed_1010_20ba(
                paVar8,
                _u16_1050_0ed0,
                0x2b,
                in_stack_0000fe5c,
                in_stack_0000ff80,
                in_stack_0000ff86,
                in_stack_0000ff8a,
            );
            paVar8 = (paVar8 & 0xffff0000 | paVar10 >> 0x10);
            paVar4 = paVar10;
            pass1_1010_089e(paVar10, uVar20, iVar16);
            pass1_1010_089e(paVar10, 0x0, 0x2);
            pass1_1010_089e(paVar10, 0x0, 0x3);
            pass1_1010_089e(paVar10, 0x0, 0x4);
            pass1_1010_089e(paVar10, 0x0, 0x5);
            pass1_1010_089e(paVar10, 0x0, 0x7);
            pass1_1010_089e(paVar10, 0x0, 0x8);
            pass1_1010_089e(paVar10, 0x0, 0xa);
        }
    }
    if ((bVar2) && (bVar3)) {
        uVar21 = 0;
        iVar22 = 0x6;
        uVar17 = 0x1;
        uVar18 = 0;
        uVar19 = 0;
        uVar15 = 0;
        iVar16 = 0;
        uVar14 = 0;
        paVar11 = mixed_1010_20ba(
            paVar8,
            _u16_1050_0ed0,
            0x37,
            in_stack_0000fe52,
            in_stack_0000ff76,
            in_stack_0000ff7c,
            in_stack_0000ff80,
        );
        paVar8 = (paVar8 & 0xffff0000 | paVar11 >> 0x10);
        paVar4 = paVar11;
        post_win_msg_1008_a0e4(
            paVar11,
            CONCAT22(uVar15, uVar14),
            iVar16,
            CONCAT11(uVar18, uVar17),
            CONCAT22(uVar21, uVar19),
            iVar22,
        );
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x8000001);
    uVar7 = paVar8;
    paVar8 = (paVar8 & 0xffff0000 | (uVar7 | paVar4));
    paVar5 = paVar4;
    if ((((((uVar7 | paVar4) != 0) && (
        paVar5 = pass1_1030_2242(CONCAT22(uVar7, paVar4), 0x4),
        paVar5.is_null(),
    )) && (
        paVar5 = pass1_1030_2242(CONCAT22(uVar7, paVar4), 0x2a),
        paVar5.is_null(),
    )) && ((
        paVar5 = pass1_1030_2242(CONCAT22(uVar7, paVar4), 0x4b),
        paVar5.is_null() && (
            paVar5 = pass1_1030_2242(CONCAT22(uVar7, paVar4), 0x54),
            paVar5.is_null(),
        ),
    ))) && ((
        paVar5 = pass1_1030_2242(CONCAT22(uVar7, paVar4), 0x2c),
        paVar5.is_null() && ((
            paVar5 = pass1_1030_2242(CONCAT22(uVar7, paVar4), 0x3c),
            paVar5.is_null() && (
                paVar5 = pass1_1030_2242(CONCAT22(uVar7, paVar4), 0x3d),
                paVar5.is_null(),
            ),
        )),
    ))) {
        if (local_14.field6_0x10 == 0) {
            paVar8 = (paVar8 & 0xffff0000 | local_14.field5_0xc);
        } else {
            local_14.field5_0xc = 0x1;
            paVar8 = (paVar8 & 0xffff0000);
        }
        local_14.field4_0x8 = SUB42(paVar8, 0x0);
        bVar2 = false;
        bVar3 = false;
        local_14.field4_0x8 = local_14.field5_0xc;
        loop {
            loop {
                paVar5 = &local_14;
                pass1_1028_e4ec(CONCAT22(0x1050, paVar5));
                uVar7 = paVar8;
                paVar8 = (paVar8 & 0xffff0000 | (uVar7 | paVar5));
                //        if ((uVar7 | paVar5) == 0) goto LAB_1028_72d3;
                if !(paVar5[0x1c].field4_0x8 == 0x8000002) {
                    break;
                }
            }
            uVar20 = (param_2 >> 0x10);
            paVar4 = paVar5;
            if ((!bVar2) && (
                pass1_1028_740c(param_2, uVar20, 0x22, CONCAT22(uVar7, paVar5)),
                paVar4.is_null() == false,
            )) {
                bVar2 = true;
            }
            if ((!bVar3) && (
                pass1_1028_740c(param_2, uVar20, 0x24, CONCAT22(uVar7, paVar5)),
                paVar4.is_null() == false,
            )) {
                bVar3 = true;
            }
            if !((!bVar2) || (!bVar3)) {
                break;
            }
        }
        uVar21 = 0;
        iVar22 = 0x14;
        uVar17 = 0x1;
        uVar18 = 0;
        uVar19 = 0;
        uVar15 = 0;
        iVar16 = 0;
        uVar14 = 0;
        paVar11 = mixed_1010_20ba(
            paVar8,
            _u16_1050_0ed0,
            0x37,
            in_stack_0000fe52,
            in_stack_0000ff76,
            in_stack_0000ff7c,
            in_stack_0000ff80,
        );
        paVar8 = (paVar8 & 0xffff0000 | paVar11 >> 0x10);
        paVar5 = paVar11;
        post_win_msg_1008_a0e4(
            paVar11,
            CONCAT22(uVar15, uVar14),
            iVar16,
            CONCAT11(uVar18, uVar17),
            CONCAT22(uVar21, uVar19),
            iVar22,
        );
    } //
    // LAB_1028_72d3:
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
    uVar7 = paVar8 | paVar5;
    paVar8 = (paVar8 & 0xffff0000 | uVar7);
    if (uVar7 != 0) {
        paVar12 = mixed_1010_20ba(
            paVar8,
            _u16_1050_0ed0,
            CONCAT22(param_3, 0x3b),
            in_stack_0000fe60,
            in_stack_0000ff84,
            in_stack_0000ff8a,
            in_stack_0000ff8e,
        );
        paVar8 = (paVar8 & 0xffff0000 | paVar12 >> 0x10);
        pass1_1008_df4a(paVar12, unaff_DI, &DAT_1050_1050);
        paVar13 = mixed_1010_20ba(
            paVar8,
            _u16_1050_0ed0,
            CONCAT22(param_3, 0x3c),
            in_stack_0000fe60,
            in_stack_0000ff84,
            in_stack_0000ff8a,
            in_stack_0000ff8e,
        );
        uVar9 = paVar13 >> 0x10;
        pass1_1018_34a6(paVar13);
        pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_46)), 0x1, 0x0, 0x400);
        loop {
            uVar7 = uVar9;
            paVar4 = &local_46;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
            uVar9 = (uVar7 | paVar4);
            if ((uVar7 | paVar4) == 0) {
                break;
            }
            if (paVar4[0x1c].field4_0x8 != 0x8000002) {
                pass1_1038_3ba0(CONCAT22(uVar7, paVar4));
            }
        }
    }
    return;
}

pub unsafe fn pass1_1028_737e(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
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
        // for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1)
        for iVar4 in 0x4..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        *puStack10 = 0x749e;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_7472(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_74ae(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x1387);
    param_1.offset_0x0 = 0x819a;
    // just 0x1028
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCEvent_1050_4ff4,
    );
    return param_1;
}

pub unsafe fn pass1_1028_74e4(param_1: u8, param_2: *mut Struct57, mut param_3: u32) -> u16 {
    let mut iVar1: i16;

    pass1_1028_7fb6(param_1, param_3);
    pass1_1028_7c4e(param_2, param_3);
    pass1_1028_7dfc(param_1, param_2, param_3);
    iVar1 = post_msg_1028_76da();
    pass1_1028_767e(iVar1, param_2);
    pass1_1028_75bc();
    pass1_1028_78b8(param_1, param_2, param_3);
    return 0x1;
}


pub unsafe fn pass1_1028_752e(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
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
        // for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1)
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        *puStack10 = 0x819a;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_816e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_81aa(param_1: u8, param_2: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_2, 0x1b57);
    param_2.offset_0x0 = 0x836e;
    (param_2 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | (param_2 + 0x8)),
        s_SCFactory_1050_5002,
    );
    return param_2;
}


pub unsafe fn pass1_1028_81e0(mut param_1: u16) -> u16 {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_92;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut unaff_CS: u16;
    let mut puStack24: *mut u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    // switchD_1028_8225_caseD_0:
    loop {
        loop {
            uVar4 = param_1;
            paVar3 = &local_14;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar3));
            puStack24 = CONCAT22(uVar4, paVar3);
            param_1 = uVar4 | paVar3;
            if (param_1 == 0) {
                return 0x1;
            }
            iVar1 = &paVar3.field5_0xc;
            //      if (iVar1 < 0x35) goto code_r0x10288222;
            if (0x61 < iVar1) {
                break;
            }
            //      if ((iVar1 < 0x5d) && ((iVar1 != 0x37 && (iVar1 != 0x47)))) goto switchD_1028_8225_caseD_1;
        }
        if !((iVar1 == 0x6a) || (0x8 < iVar1 - 0x6a && (iVar1 == 0x75 || iVar1 - 0x74 < 0x1 || (0x0 < iVar1 - 0x76 && (iVar1 - 0x78 < 0x2))))) {
            break;
        }
    }
    //   goto switchD_1028_8225_caseD_1;
    // code_r0x10288222:
    unaff_CS = 0x1028;
    match iVar1 {
        0x1 | 0x2 | 0x3 | 0x4 | 0x6 | 0x7 | 0x8 | 0xa | 0xb | 0xc | 0xd | 0xe | 0xf | 0x11 => {
            // switchD_1028_8225_caseD_1:
            if ((paVar3 + 1) == 0x5) {
                ppcVar2 = (*puStack24 + 0x30);
                (**ppcVar2)(unaff_CS);
                param_1 = extraout_DX;
            }
        }
    };
    //   goto switchD_1028_8225_caseD_0;
}


pub unsafe fn pass1_1028_82b4(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
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
        // for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1)
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        *puStack10 = 0x836e;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}


pub unsafe fn pass1_1028_8342(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_83b4(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        if ((param_1 | paVar1) == 0) {
            break;
        }
        (&paVar1[0x1c].field5_0xc + 0x2) = 0x1;
        param_1 = param_1 | paVar1;
    }
    return 0x1;
}


pub unsafe fn pass1_1028_8400(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
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
        *puStack10 = 0x84ba;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}


pub unsafe fn pass1_1028_848e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1028_853e(param_1: *mut astruct_685, mut param_2: u32) -> u16 {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    if ((iVar2 + 0x108) == 0) {
        return 0x0;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar2 + 0x10e));
    if ((iVar2 + 0x108) == 1) {
        uVar1 = 0x3e8;
    } else {
        uVar1 = 0;
    }
    pass1_1038_4d0e(CONCAT22(param_1, (param_1 >> 0x10)), uVar1);
    return 0x1;
}


pub unsafe fn pass1_1028_858c(
    param_1: *mut astruct_318,
    param_2: *mut u8,
    param_3: *mut astruct_319,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_319;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x112, paVar5);
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
        param_1.field258_0x10a = iVar5.field259_0x10a;
        param_1.field259_0x10c = iVar5.field260_0x10c;
        param_1.field260_0x10e = iVar5.field261_0x10e;
        *puStack10 = 0x8688;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_865c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
