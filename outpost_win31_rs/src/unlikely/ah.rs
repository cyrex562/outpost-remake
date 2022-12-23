pub unsafe fn pass1_1030_10b0(
    param_1: *mut astruct_12,
    param_2: *mut astruct_12,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u32,
    mut param_7: u32,
) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut uStack8: u16;

    puVar6 = switch_1030_07ac(
        param_1,
        param_2,
        param_3,
        param_4,
        param_5,
        param_6,
        (param_6 >> 0x10),
        param_7,
    );
    uVar3 = (puVar6 >> 0x10);
    uVar1 = (puVar6 + 0x4);
    uVar2 = uVar1;
    uVar4 = uVar3;
    pass1_1028_e1ec(CONCAT22(param_4, param_3), param_7);
    uVar5 = uVar4 | uVar2;
    if (uVar5 != 0) {
        pass1_1030_7e5a(uVar5, (uVar2 & 0xffff | uVar4 << 0x10), uVar1);
    }
    uStack8 = (uVar1 >> 0x10);
    pass1_1030_1358(
        (param_3 + 0x26),
        puVar6,
        uVar3,
        uVar1 & 0xffff | (uStack8 & 0xff) << 0x10,
    );
    return;
}

pub unsafe fn pass1_1030_1120(mut param_1: u16, param_2: *mut Struct57, mut param_3: u32) {
    let mut puVar1: *mut u8;

    mem_op_1000_179c(0x3b2, param_2);
    puVar1 = (param_2 | param_1);
    if (puVar1.is_null()) {
        param_1 = 0;
        puVar1 = null_mut();
    } else {
        struct_1030_2112(param_1, puVar1, CONCAT22(param_2, param_1), 0x0);
    }
    pass1_1030_1358(
        (param_3 + 0x2a),
        param_1,
        puVar1,
        (param_1 + 0x4) & 0xffff | ((param_1 + 0x6) & 0xff) << 0x10,
    );
    return;
}


pub unsafe fn pass1_1030_117a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_1794(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_16b2(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_18f0(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: i16,
    mut param_4: i16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uStack10: u32;
    let mut uStack6: u32;

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if ((iVar3 + 0xc) != 0) {
        ppcVar1 = ((iVar3 + 0xc) + 0x10);
        (**ppcVar1)();
        uStack6 = CONCAT22(extraout_DX, param_1);
        for uStack10 in 0..uStack6 {
            ppcVar1 = ((iVar3 + 0xc) + 0x4);
            uVar2 = uStack6;
            (**ppcVar1)();
            if ((uVar2 == param_3) && (extraout_DX_00 == param_4)) {
                ppcVar1 = ((iVar3 + 0xc) + 0x8);
                (**ppcVar1)();
            }
        }
    }
    return;
}


pub unsafe fn pass1_1030_1972() -> u16 {
    return 0x1;
}


pub unsafe fn pass1_1030_19f0(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_18b2(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_1a9c(mut param_1: u32, mut param_2: u32) -> u16 {
    let mut piVar1: *mut i16;
    let mut in_AX: u16;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000ffde: HFILE16;
    let mut local_c: [u16; 0x5] = [0; 0x5];

    uVar2 = pass1_1030_1978(in_AX, param_1, param_2);
    if (uVar2 != 0) {
        uVar5 = (param_1 >> 0x10);
        iVar4 = param_1;
        local_c[0] = (iVar4 + 0x10);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffde);
        if (BVar3 != 0) {
            if (*(iVar4 + 0x10) == 0) {
                return 0x1;
            }
            piVar1 = (iVar4 + 0x10);
            BVar3 = write_to_file_1008_7e1c(
                param_2,
                (piVar1 + 0x2),
                (*piVar1 * 0x2),
                in_stack_0000ffde,
            );
            if (BVar3 != 0) {
                return 0x1;
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub unsafe fn file_1030_1b18(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
) -> u16 {
    let mut uVar1: u16;
    let mut piVar2: *mut i16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut BVar5: bool;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut pSVar7: *mut StructD;
    let mut paVar8: *mut Struct57;
    let mut iVar9: i16;
    let mut iVar7: *mut astruct_368;
    let mut uVar10: u16;
    let mut uVar11: u16;

    pSVar7 = CONCAT22(in_register_0000000a, param_2);
    file_1030_19b4(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
        } else {
            pSVar7 = (pSVar7 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
        }
        uVar4 = fn_ptr_op_1000_1708(0x6, 0x0, 0x1, PTR_LOOP_1050_5f2c, pSVar7);
        uVar10 = (param_3 >> 0x10);
        iVar9 = param_3;
        (iVar9 + 0x10) = uVar4;
        (iVar9 + 0x12) = pSVar7;
        uVar1 = (iVar9 + 0x12);
        paVar8 = (pSVar7 & 0xffff0000 | uVar1);
        BVar5 = read_file_1008_7dee(param_4, CONCAT22(uVar1, (iVar9 + 0x10)), 0x2);
        if (BVar5 != 0) {
            piVar2 = (iVar9 + 0x10);
            if (*piVar2 == 0) {
                return 0x1;
            }
            uVar1 = *piVar2 * 0x2;
            uVar6 = uVar1;
            mem_op_1000_179c(uVar1, paVar8);
            uVar3 = (iVar9 + 0x10);
            uVar11 = (uVar3 >> 0x10);
            iVar7 = uVar3;
            iVar7.field2_0x2 = uVar6;
            iVar7.field3_0x4 = paVar8;
            uVar3 = (iVar9 + 0x10);
            BVar5 = read_file_1008_7dee(param_4, (uVar3 + 0x2), uVar1);
            if (BVar5 != 0) {
                return 0x1;
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return 0x0;
}



pub unsafe fn pass1_1030_1be2(mut param_1: u16, param_2: *mut Struct57, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u16;
    let mut uVar3: *mut Struct57;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut uStack4: u16;

    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    if ((iVar3 + 0xc) == 0) {
        mem_op_1000_179c(0x18, param_2);
        uVar3 = param_2;
        param_2 = (param_2 & 0xffff0000 | (uVar3 | param_1));
        if ((uVar3 | param_1) == 0) {
            (iVar3 + 0xc) = 0;
        } else {
            struct_op_1030_1cd8(CONCAT22(uVar3, param_1), 0x5, 0x5);
            (iVar3 + 0xc) = param_1;
            (iVar3 + 0xe) = param_2;
        }
    }
    //   for (uStack4 = 0; puVar2 = (iVar3 + 0x10), uStack4 <= *puVar2 && *puVar2 != uStack4; uStack4 += 1)
    uStack4 = 0;
    puVar2 = iVar3 + 0x10;

    while uStack4 <= *puVar2 && *puVar2 != uStack4 {
        uVar5 = pass1_1028_e2e0(param_2, _PTR_LOOP_1050_65e2, 1);
        param_2 = (param_2 & 0xffff0000 | uVar5 >> 0x10);
        ppcVar1 = ((iVar3 + 0xc) + 0x8);
        (**ppcVar1)(0x1028, (iVar3 + 0xc), uVar5, (uVar5 >> 0x10), uStack4, 0x0);
        uStack4 += 1;
    }
    return;
}


pub unsafe fn pass1_1030_1c96(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_1a74(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1030_1daa(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xa), (param_1 + 0x8));
}


pub unsafe fn pass1_1030_1dbc() {
    return;
}


pub unsafe fn pass1_1030_1dfc(param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut bVar7: bool;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    puVar1 = (iVar5 + 0x8);
    if ((*puVar1 < param_4 || *puVar1 == param_4) || ((iVar5 + 0x4) == 0)) {
        puVar2 = (iVar5 + 0x12);
        bVar7 = *puVar2 < param_4;
        if ((bVar7 || *puVar2 == param_4)
            && (bVar7
                || (
                    puVar2 = (iVar5 + 0x10),
                    *puVar2 < param_4 || *puVar2 == param_4,
                )))
        {
            ppcVar3 = (*param_1 + 0x20);
            (**ppcVar3)();
        }
        puVar1 = (iVar5 + 0x10);
        if ((*puVar1 < param_4 || *puVar1 == param_4) || ((iVar5 + 0x4) == 0)) {
            return;
        }
        puVar2 = (iVar5 + 0xa);
        bVar7 = *puVar2 < param_4;
        if ((bVar7 || *puVar2 == param_4)
            && (bVar7
                || (
                    puVar2 = (iVar5 + 0x8),
                    *puVar2 < param_4 || *puVar2 == param_4,
                )))
        {
            (iVar5 + 0x8) = (param_4 + 1);
            (iVar5 + 0xa) = (param_4 + 0x1 >> 0x10);
        }
    }
    uVar4 = (iVar5 + 0x4);
    uVar6 = (uVar4 >> 0x10);
    iVar5 = uVar4;
    (iVar5 + param_4 * 0x4) = param_2;
    (iVar5 + param_4 * 0x4 + 0x2) = param_3;
    return;
}


pub unsafe fn pass1_1030_1e96(param_1: u32) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uStack6: u32;

    uStack6 = 0;
    loop {
        uVar4 = (param_1 >> 0x10);
        puVar1 = (param_1 + 0x8);
        if ((*puVar1 < uStack6 || *puVar1 == uStack6)
            || (uVar3 = (param_1 + 0x4), (uVar3 + uStack6 * 0x4) == 0))
        {
            break;
        }
        uStack6 += 0x1;
    }
    ppcVar2 = (*param_1 + 0x8);
    (**ppcVar2)();
    return;
}


pub unsafe fn pass1_1030_1eee(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0xc);
    param_2 = (iVar2 + 0xe);
    if (uVar1 < param_2) {
        uVar1 = param_2 & 0xffff;
    }
    (iVar2 + 0xc) = uVar1;
    (iVar2 + 0xe) = param_2;
    return;
}


pub unsafe fn pass1_1030_1f16(param_1: u32, mut param_2: u32) {
    let mut plVar1: *mut i32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (((iVar4 + 0x4) == 0) || ((iVar4 + 0x10) <= (iVar4 + 0x8))) {
        ppcVar2 = (*param_1 + 0x20);
        (**ppcVar2)();
    }
    uVar3 = (iVar4 + 0x4);
    ((iVar4 + 0x8) * 0x4 + uVar3) = param_2;
    plVar1 = (iVar4 + 0x8);
    *plVar1 = *plVar1 + 1;
    return;
}

pub unsafe fn FUN_1030_1f6c() {
    return;
}


pub unsafe fn FUN_1030_1f70(mut param_1: u16, mut param_2: u32) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut pSVar5: *mut StructD;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut lVar8: i32;
    let mut uStack10: u32;
    let mut uStack6: u32;

    uVar7 = (param_2 >> 0x10);
    iVar6 = param_2;
    if ((iVar6 + 0x10) == 0) {
        uVar4 = (iVar6 + 0xc);
        pSVar5 = (in_EDX & 0xffff0000 | (iVar6 + 0xe));
    } else {
        uVar2 = (iVar6 + 0x10);
        puVar1 = (iVar6 + 0x14);
        uVar4 = uVar2 + *puVar1;
        pSVar5 = (in_EDX & 0xffff0000 | ((iVar6 + 0x12) + (iVar6 + 0x16) + CARRY2(uVar2, *puVar1)));
    }
    uStack6 = CONCAT22(pSVar5, uVar4);
    if ((iVar6 + 0x4) == 0) {
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
            PTR_LOOP_1050_5f2e = pSVar5;
        } else {
        }
        uVar4 = fn_ptr_op_1000_1708(
            uVar4 << 0x2,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            PTR_LOOP_1050_5f2e,
        );
    } else {
        uVar3 = (iVar6 + 0x4);
        lVar8 = pass1_1000_0ed4(
            0x1,
            uVar4 * 0x4,
            (pSVar5 * 0x2 + CARRY2(uVar4, uVar4)) * 0x2 + CARRY2(uVar4 * 0x2, uVar4 * 0x2),
            uVar3,
            (uVar3 >> 0x10),
        );
        PTR_LOOP_1050_5f2e = (lVar8 >> 0x10);
        uVar4 = lVar8;
    }
    uStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    if ((PTR_LOOP_1050_5f2e | uVar4) != 0) {
        (iVar6 + 0x10) = uStack6;
        (iVar6 + 0x4) = uStack10;
    }
    return;
}


pub unsafe fn pass1_1030_227a(param_1: u16, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut BVar3: bool;
    let mut in_stack_0000ffe8: HFILE16;

    uVar1 = pass1_1030_1978(param_1, param_2, param_3);
    if (uVar1 != 0) {
        iVar2 = param_2;
        BVar3 = write_to_file_1008_7e1c(
            param_3,
            param_2 & 0xffff0000 | (iVar2 + 0x10),
            0x106,
            in_stack_0000ffe8,
        );
        if (BVar3 != 0) {
            BVar3 = write_to_file_1008_7e1c(
                param_3,
                param_2 & 0xffff0000 | (iVar2 + 0x116),
                0x86,
                in_stack_0000ffe8,
            );
            if (BVar3 != 0) {
                BVar3 = write_to_file_1008_7e1c(
                    param_3,
                    param_2 & 0xffff0000 | (iVar2 + 0x19c),
                    0xa,
                    in_stack_0000ffe8,
                );
                if (BVar3 != 0) {
                    BVar3 = write_to_file_1008_7e1c(
                        param_3,
                        param_2 & 0xffff0000 | (iVar2 + 0x1a6),
                        0x106,
                        in_stack_0000ffe8,
                    );
                    if (BVar3 != 0) {
                        BVar3 = write_to_file_1008_7e1c(
                            param_3,
                            param_2 & 0xffff0000 | (iVar2 + 0x2ac),
                            0x106,
                            in_stack_0000ffe8,
                        );
                        if (BVar3 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}


pub unsafe fn pass1_1030_232e(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut iVar1: i16;
    let mut BVar2: bool;

    file_1030_19b4(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        iVar1 = param_3;
        BVar2 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar1 + 0x10)), 0x106);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar1 + 0x116)), 0x86);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar1 + 0x19c)), 0xa);
                if (BVar2 != 0) {
                    BVar2 = read_file_1008_7dee(
                        param_4,
                        (param_3 & 0xffff0000 | (iVar1 + 0x1a6)),
                        0x106,
                    );
                    if (BVar2 != 0) {
                        BVar2 = read_file_1008_7dee(
                            param_4,
                            (param_3 & 0xffff0000 | (iVar1 + 0x2ac)),
                            0x106,
                        );
                        if (BVar2 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn pass1_1030_2916(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_18b2(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1030_2aca(param_1: u16, param_2: *mut astruct_730, mut param_3: u32) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut iVar6: *mut astruct_730;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_stack_0000ffc8: HFILE16;
    let mut local_18: [u32; 0x3] = [0; 0x3];
    let mut local_c: [u16; 0x3] = [0; 0x3];
    let mut local_6: [u16; 0x2] = [0; 0x2];

    uVar2 = pass1_1030_1978(param_1, param_2, param_3);
    if (uVar2 == 0) {
        return;
    }
    uVar5 = (param_2 >> 0x10);
    iVar6 = param_2;
    local_c[0] = *iVar6.field16_0x10;
    BVar3 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffc8);
    if (((BVar3 != 0) && (
        puVar1 = iVar6.field16_0x10,
        BVar3 = pass1_1008_7c2a(param_3, *(puVar1 + 0x2)),
        BVar3 != 0,
    )) && (
        puVar1 = iVar6.field16_0x10,
        iVar4 = write_to_file_1008_7b4c(param_3, (puVar1 & 0xffff0000 | (puVar1 + 0x6))),
        iVar4 != 0,
    )) {
        puVar1 = iVar6.field16_0x10;
        local_6[0] = (puVar1 + 0xc);
        BVar3 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffc8);
        if (BVar3 != 0) {
            puVar1 = iVar6.field16_0x10;
            local_18[0] = (puVar1 + 0xe);
            BVar3 = write_to_file_1008_7e1c(
                param_3,
                CONCAT22(0x1050, local_18),
                0x4,
                in_stack_0000ffc8,
            );
            if ((BVar3 != 0) && (
                puVar1 = iVar6.field16_0x10,
                BVar3 = write_to_file_1008_7e1c(
                    param_3,
                    puVar1 & 0xffff0000 | (puVar1 + 0x12),
                    0x10,
                    in_stack_0000ffc8,
                ),
                BVar3 != 0,
            )) {
                puVar1 = iVar6.field16_0x10;
                local_c[0] = (puVar1 + 0x22);
                BVar3 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, local_c),
                    0x2,
                    in_stack_0000ffc8,
                );
                if ((BVar3 != 0) && ((
                    puVar1 = iVar6.field16_0x10,
                    (puVar1 + 0x22) == 0x0 || (
                        puVar1 = iVar6.field16_0x10,
                        uVar6 = (puVar1 >> 0x10),
                        iVar4 = puVar1,
                        BVar3 = write_to_file_1008_7e1c(
                            param_3,
                            (iVar4 + 0x24),
                            ((iVar4 + 0x22) * 0x2),
                            in_stack_0000ffc8,
                        ),
                        BVar3 != 0,
                    ),
                ))) {
                    local_c[0] = iVar6.field17_0x14;
                    BVar3 = write_to_file_1008_7e1c(
                        param_3,
                        CONCAT22(0x1050, local_c),
                        0x2,
                        in_stack_0000ffc8,
                    );
                    if (BVar3 != 0) {
                        local_c[0] = iVar6.field18_0x16;
                        BVar3 = write_to_file_1008_7e1c(
                            param_3,
                            CONCAT22(0x1050, local_c),
                            0x2,
                            in_stack_0000ffc8,
                        );
                        if (BVar3 != 0) {
                            local_c[0] = iVar6.field19_0x18;
                            BVar3 = write_to_file_1008_7e1c(
                                param_3,
                                CONCAT22(0x1050, local_c),
                                0x2,
                                in_stack_0000ffc8,
                            );
                            if (BVar3 != 0) {
                                local_c[0] = iVar6.field20_0x1a;
                                BVar3 = write_to_file_1008_7e1c(
                                    param_3,
                                    CONCAT22(0x1050, local_c),
                                    0x2,
                                    in_stack_0000ffc8,
                                );
                                if (BVar3 != 0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    u16_1050_0310 = 0x6d0;
    return;
}


pub unsafe fn pass1_1030_2c8a(
    mut param_1: i16,
    param_2: *mut StructD,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut puVar4: *mut u8;
    let mut uVar5: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar7: *mut astruct_374;
    let mut iVar8: *mut astruct_371;
    let mut iVar9: *mut astruct_372;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let mut puVar8: *mut u16;
    let mut puVar9: *mut u32;
    let mut in_stack_0000fa72: u16;
    let mut in_stack_0000fb96: u16;
    let mut in_stack_0000fb9c: u16;
    let mut in_stack_0000fba0: u16;
    let mut puStack1038: *mut u16;
    let mut local_406: *mut astruct_430;
    let mut local_404: u16;
    let mut local_402: [u8; 0x400] = [0; 0x400];
    let mut uVar9: *mut astruct_373;
    let mut iVar14: *mut astruct_373;

    iVar14 = param_3;
    uVar9 = (param_3 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4);
    if (param_1 == 0) {
        return;
    }
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_2);
    } else {
        param_2 = (param_2 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar2 = fn_ptr_op_1000_1708(0x28, 0x0, 0x1, PTR_LOOP_1050_5f2c, param_2);
    uVar5 = param_2;
    puStack1038 = CONCAT22(uVar5, uVar2);
    paVar6 = (param_2 & 0xffff0000 | (uVar5 | uVar2));
    if ((uVar5 | uVar2) == 0) {
        iVar14.field13_0x10 = null_mut();
    } else {
        puVar8 = pass1_1008_3e38(CONCAT22(uVar5, uVar2 + 0x6));
        paVar6 = (param_2 & 0xffff0000 | puVar8 >> 0x10);
        iVar14.field13_0x10 = puStack1038;
    }
    BVar3 = read_file_1008_7dee(param_4, iVar14.field13_0x10, 0x2);
    if (BVar3 != 0) {
        puVar4 = local_402;
        read_file_1008_7c6e(param_4, (param_4 >> 0x10), CONCAT22(0x1050, puVar4));
        if (puVar4.is_null() == false) {
            uVar2 = str_op_1008_60e8(paVar6, CONCAT22(0x1050, local_402));
            puVar1 = iVar14.field13_0x10;
            uVar7 = (puVar1 >> 0x10);
            iVar7 = puVar1;
            iVar7.field2_0x2 = uVar2;
            iVar7.field3_0x4 = paVar6;
            puVar1 = iVar14.field13_0x10;
            BVar3 = read_file_1008_7bc8(param_4, (puVar1 & 0xffff0000 | (puVar1 + 0x6)));
            if ((((BVar3 != 0) && (
                puVar1 = iVar14.field13_0x10,
                BVar3 = read_file_1008_7dee(param_4, (puVar1 & 0xffff0000 | (puVar1 + 0xc)), 0x2),
                BVar3 != 0,
            )) && (
                puVar1 = iVar14.field13_0x10,
                BVar3 = read_file_1008_7dee(param_4, (puVar1 & 0xffff0000 | (puVar1 + 0xe)), 0x4),
                BVar3 != 0,
            )) && ((
                puVar1 = iVar14.field13_0x10,
                BVar3 = read_file_1008_7dee(param_4, (puVar1 & 0xffff0000 | (puVar1 + 0x12)), 0x10),
                BVar3 != 0x0 && (
                    puVar1 = iVar14.field13_0x10,
                    BVar3 = read_file_1008_7dee(
                        param_4,
                        (puVar1 & 0xffff0000 | (puVar1 + 0x22)),
                        0x2,
                    ),
                    BVar3 != 0,
                ),
            ))) {
                puVar1 = iVar14.field13_0x10;
                if ((puVar1 + 0x22) != 0) {
                    puVar1 = iVar14.field13_0x10;
                    uVar7 = (puVar1 >> 0x10);
                    iVar8 = puVar1;
                    uVar2 = iVar8.field34_0x22 * 0x2;
                    mem_op_1000_179c(uVar2, paVar6);
                    iVar8.field35_0x24 = uVar2;
                    iVar8.field36_0x26 = paVar6;
                    puVar1 = iVar14.field13_0x10;
                    uVar7 = (puVar1 >> 0x10);
                    iVar9 = puVar1;
                    BVar3 = read_file_1008_7dee(
                        param_4,
                        iVar9.field35_0x24,
                        (iVar9.field34_0x22 * 0x2),
                    );
                    if (BVar3 == 0) {
                        u16_1050_0310 = 0x6d2;
                        return;
                    }
                }
                BVar3 = read_file_1008_7dee(
                    param_4,
                    (param_3 & 0xffff0000 | ZEXT24(&iVar14.field_0x14)),
                    0x2,
                );
                if (((BVar3 != 0) && (
                    BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_404), 0x2),
                    BVar3 != 0,
                )) && ((
                    BVar3 = read_file_1008_7dee(
                        param_4,
                        (param_3 & 0xffff0000 | ZEXT24(&iVar14.field_0x18)),
                        0x2,
                    ),
                    BVar3 != 0x0 && (
                        BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_406), 0x2),
                        BVar3 != 0,
                    ),
                ))) {
                    iVar14.field16_0x16 = local_404;
                    iVar14.field19_0x1a = local_406;
                    puVar9 = mixed_1010_20ba(
                        paVar6,
                        _u16_1050_0ed0,
                        CONCAT22(unaff_SI, 0x2f),
                        in_stack_0000fa72,
                        in_stack_0000fb96,
                        in_stack_0000fb9c,
                        in_stack_0000fba0,
                    );
                    pass1_1018_04a4(puVar9, iVar14.field4_0x4);
                    pass1_1010_82f8(_u16_1050_14cc, *iVar14.field13_0x10);
                    return;
                }
            }
        }
    }
    u16_1050_0310 = 0x6d2;
    return;
}



pub unsafe fn pass1_1030_51f0(mut param_1: u32) -> u32 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xa4) = 0;
    (iVar1 + 0xa6) = 0;
    (iVar1 + 0xa8) = 0;
    (iVar1 + 0xaa) = 0;
    (iVar1 + 0xac) = 0;
    return param_1;
}

pub unsafe fn pass1_1030_51eb() {
    pass1_1030_3b28();
    return;
}


pub unsafe fn pass1_1030_5260(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut puStack6: *mut u32;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x108));
    puStack6 = CONCAT22(param_2, param_1);
    ppcVar1 = (*puStack6 + 0x14);
    (**ppcVar1)();
    return 0x1;
}
