
pub unsafe fn pass1_1030_5044(param_1: u16, param_2: *mut astruct_117) {
    let mut pcVar1: *mut c_char;
    let mut plVar2: *mut i32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut pcVar7: *mut c_char;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut extraout_var: u16;
    let mut uStack28: u32;
    let mut uStack24: u16;
    let mut uStack22: u32;
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut local_a: i32;
    let mut local_6: *mut c_char;
    let mut uVar8: u32;
    let mut pSVar11: *mut StructD;
    let mut pstruct117_10: *mut astruct_117;
    let mut pstruct117_10_hi: *mut astruct_117;

    plVar2 = &local_a;
    uVar9 = read_file_1030_4e70(
        param_2,
        CONCAT22(0x1050, plVar2),
        CONCAT22(0x1050, &local_6),
        s_bldgops_dat_1050_5708,
        param_1,
    );
    pcVar1 = local_6;
    pSVar11 = CONCAT22(extraout_var, uVar9);
    if (plVar2.is_null() == false) {
        pstruct117_10 = param_2;
        pstruct117_10_hi = (param_2 >> 0x10);
        pcVar7 = local_6;
        pass1_1030_4e34(pstruct117_10, pstruct117_10_hi, local_a, local_6);
        uVar3 = pcVar7;
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar11);
            PTR_LOOP_1050_5f2e = pSVar11;
        } else {
        }
        uVar4 = fn_ptr_op_1000_1708(
            uVar3 * 0xae,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            PTR_LOOP_1050_5f2e,
        );
        uVar8 = uVar4;
        uStack28 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
        uVar10 = PTR_LOOP_1050_5f2e | uVar4;
        if (uVar10 == 0) {
            pstruct117_10[0xd].field10_0xa = 0;
        } else {
            pass1_1000_5586(0x51f0, 0x1030, uVar3, 0xae, uVar4, PTR_LOOP_1050_5f2e);
            pstruct117_10[0xd].field10_0xa = uStack28;
            uVar8 = uStack28;
        }
        uVar5 = uVar8;
        pass1_1030_4dbc(param_2, local_6, pcVar7 & 0xffff);
        uStack22 = CONCAT22(uVar10, uVar5);
        for uStack24 in 0..uVar3 {
            uVar10 = (&pstruct117_10[0xd].field10_0xa + 2);
            iVar6 = &pstruct117_10[0xd].field10_0xa + uStack24 * 0xae;
            pass1_1030_4c52(
                pstruct117_10,
                pstruct117_10_hi,
                CONCAT22(uVar10, iVar6),
                uStack22,
                uVar10,
            );
            pass1_1030_4dbc(param_2, 0x0, 0x0);
            uStack22 = CONCAT22(uVar10, iVar6);
        }
        uStack12 = (pcVar1 >> 0x10);
        uStack14 = pcVar1;
        if ((uStack12 | uStack14) != 0) {
            call_fn_ptr_1000_0dc6(pcVar1);
        }
    }
    return;
}

pub unsafe fn pass1_1030_5164(param_1: *mut astruct_117, mut param_2: u32) -> *mut c_char {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x568));
    loop {
        lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        if (lVar4 == 0) {
            return param_2;
        }
        uVar1 = param_1 + 0x168;
        unk_str_op_1000_3d3e((param_1 & 0xffff0000 | uVar1), *(lVar4 + 0x4));
        pass1_1000_3cea(param_1 & 0xffff0000 | uVar1, param_2);
        uVar2 = dos3_call_1000_51aa(uVar1, uVar3, 1);
        if uVar2 == 0 {
            break;
        }
    }
    return (param_1 & 0xffff0000 | uVar1);
}



pub unsafe fn pass1_1030_521c(param_1: *mut astruct_97, mut param_2: u32) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x32c7);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_2;
    param_1.offset_0x0 = 0x55fe;
    iVar1.segment_0x2 = 0x1030;
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCGenKids_0x_08lx_1050_5714,
        param_2,
    );
    return;
}




pub unsafe fn pass1_1030_532e(param_1: *mut astruct_97, mut param_2: u32) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: *mut astruct_97;

    struct_op_1028_d1dc(param_1, 0x32c7);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_2;
    param_1.offset_0x0 = 0x55ee;
    iVar1.segment_0x2 = 0x1030;
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCSelect__u___d_1050_5726,
        iVar1.field_0x4,
    );
    return;
}







pub unsafe fn pass1_1030_560e(param_1: *mut astruct_180) -> *mut u16 {
    let mut in_EDX: u32;
    let mut iVar1: *mut astruct_180;
    let mut uVar2: *mut astruct_180;

    struct_1030_17ce(param_1, 0x64, 0x1f4, in_EDX);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field12_0x10 = 0;
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar1.field14_0x14)));
    iVar1.field17_0x1a = 0;
    iVar1.field18_0x1c = 0;
    param_1.field0_0x0 = s_procLo_1050_5bd0;
    iVar1.field1_0x2 = 0x1030;
    return &param_1.field0_0x0;
}

pub unsafe fn struct_1030_565a(
    mut param_1: u16,
    param_2: *mut Struct57,
    param_3: *mut astruct_352,
    mut param_4: u32,
) -> *mut u16 {
    let mut pstruct_1: *mut astruct_353;
    let mut pstruct_1_hi: *mut astruct_352;

    pass1_1030_183c(
        param_1,
        param_2,
        &param_3.u16_field_0x0,
        0x64,
        0x1f4,
        0x3000000,
        param_4,
    );
    pstruct_1_hi = (param_3 >> 0x10);
    pstruct_1 = param_3;
    pstruct_1.field15_0x10 = 0;
    pass1_1008_3e38((param_3 & 0xffff0000 | ZEXT24(&pstruct_1.field16_0x14)));
    pstruct_1.field21_0x1a = 0;
    pstruct_1.field22_0x1c = 0;
    // 0x5bd0
    param_3.u16_field_0x0 = s_procLo_1050_5bd0;
    pstruct_1.field2_0x2 = 0x1030;
    return &param_3.u16_field_0x0;
}
pub unsafe fn pass1_1030_56b0(param_1: *mut u16) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    *param_1 = s_procLo_1050_5bd0;
    (iVar3 + 0x2) = 0x1030;
    pcVar2 = *(iVar3 + 0x10);
    uVar1 = (iVar3 + 0x12);
    if ((uVar1 | pcVar2) != 0) {
        fn_ptr_1030_84d0(pcVar2 & 0xffff | uVar1 << 0x10);
        fn_ptr_1000_17ce(pcVar2);
    }
    pass1_1030_18b2(param_1);
    return;
}

pub unsafe fn pass1_1030_5a52(mut param_1: u32, param_2: *mut u32, param_3: *mut u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x10);
    *param_3 = (uVar1 + 0xe);
    uVar1 = (param_1 + 0x10);
    *param_2 = (uVar1 + 0x12);
    return;
}
pub unsafe fn pass1_1030_5a80(mut param_1: u32, mut param_2: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_20: [u8; 0xc] = [0; 0xc];
    let mut local_14: u32;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut iStack6: i16;
    let mut uStack4: u16;

    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x10) = param_2;
    uVar3 = pass1_1008_4772((param_2 + 0xe));
    uStack4 = (uVar3 >> 0x10);
    iStack6 = uVar3;
    uStack10 = (iStack6 + 0x4);
    uStack14 = (iStack6 + 0x8);
    pass1_1008_3e54(
        CONCAT22(0x1050, &local_14),
        0x0,
        uStack14 - 0x1,
        uStack10 - 1,
    );
    puVar1 = (param_1 + 0x14);
    pass1_1008_6cb4(
        CONCAT22(0x1050, local_20),
        &local_14,
        0x1050,
        puVar1,
        uVar2,
    );
    pass1_1008_6d64(
        CONCAT22(0x1050, local_20),
        (param_1 & 0xffff0000 | ZEXT24(puVar1)),
    );
    return;
}

pub unsafe fn pass1_1030_5b00(mut param_1: u32) -> i16 {
    return (param_1 + 0x4) + 0xb;
}
pub unsafe fn pass1_1030_5b1c(mut param_1: u32, param_2: *mut u16, param_3: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x1a);
    *param_2 = (param_1 + 0x1c);
    return;
}
pub unsafe fn pass1_1030_5b3e(mut param_1: u32, mut param_2: i16, mut param_3: u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x1a) = param_3;
    if ((iVar1 + 0x1c) < param_2) {
        (iVar1 + 0x1c) = param_2;
    }
    return;
}

pub unsafe fn pass1_1030_5b5c(mut param_1: i16, mut param_2: u16) -> u32 {
    return CONCAT22(param_2, param_1 + 0x14);
}
pub unsafe fn pass1_1030_5b6c(mut param_1: u16, param_2: *mut astruct_610, param_3: *mut c_char) {
    let mut lVar1: i32;
    let mut uVar2: u16;
    let mut iVar4: *mut astruct_610;
    let mut iVar3: *mut astruct_609;
    let mut uVar3: u16;

    uVar3 = (param_2 >> 0x10);
    iVar4 = param_2;
    if (iVar4.field16_0x10 != 0) {
        lVar1 = iVar4.field16_0x10;
        fn_ptr_1000_17ce(*(lVar1 + 0x4));
        uVar2 = str_op_1008_60e8(param_1, param_3);
        lVar1 = iVar4.field16_0x10;
        uVar3 = (lVar1 >> 0x10);
        iVar3 = lVar1;
        iVar3.field4_0x4 = uVar2;
        iVar3.field5_0x6 = param_1;
    }
    return;
}




pub unsafe fn pass1_1030_5bec(mut param_1: u32) {
    _PTR_LOOP_1050_5736 = param_1;
    pass1_1000_54a0(param_1, 0x0, 0x24);
    return;
}


pub unsafe fn pass1_1030_5c0e() {
    _PTR_LOOP_1050_5736 = 0;
    return;
}

pub unsafe fn pass1_1030_5c1a(mut param_1: u32, mut param_2: u32) -> BOOL16 {
    let mut BVar1: bool;
    let mut in_stack_0000ffe8: HFILE16;

    BVar1 = write_to_file_1008_7cac(param_2);
    if (BVar1 != 0) {
        BVar1 = write_to_file_1008_7e1c(param_2, param_1, 0x24, in_stack_0000ffe8);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}

pub unsafe fn read_file_1030_5c52(param_1: u16, param_2: *mut u8, param_3: *mut HFILE16) -> BOOL16 {
    let mut BVar1: bool;

    read_file_1008_7cfe(param_3, (param_3 >> 0x10), 0x9);
    if (param_1 != 0) {
        BVar1 = read_file_1008_7dee(param_3, param_2, 0x24);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d2;
            return BVar1;
        }
        param_1 = 0x1;
    }
    return param_1;
}


pub unsafe fn pass1_1030_5c8a(mut param_1: u32, mut param_2: u32) {
    let mut plVar1: *mut i32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar5: *mut astruct_177;
    let mut uVar5: u16;
    let mut uStack6: u32;

    uStack6 = 0;
    uVar2 = param_2._3_1_;
    if (uVar2 == 0xff) {
        return;
    }
    uVar5 = (_PTR_LOOP_1050_65e2 >> 0x10);
    iVar5 = (_PTR_LOOP_1050_65e2 + 0xa);
    uVar3 = (iVar5 + uVar2 * 0x4);
    uVar4 = (iVar5 + uVar2 * 0x4 + 2);
    if ((uVar3 + 0x4) != 0) {
        pass1_1030_12ca((uVar3 & 0xffff | uVar4 << 0x10));
        uStack6 = uVar3 & 0xffff | uVar4 << 0x10;
    }
    if (uStack6 == 0) {
        plVar1 = (uVar2 * 0x4 + param_1);
        *plVar1 = *plVar1 + 1;
    }
    return;
}

pub unsafe fn pass1_1030_5d0a(param_1: *mut u16) -> *mut u16 {
    let mut in_EDX: u32;
    let mut uVar1: u16;

    struct_1030_17ce(param_1, 0x1, 0x4, in_EDX);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x10) = 0;
    *param_1 = 0x613e;
    (param_1 + 0x2) = 0x1030;
    return param_1;
}

pub unsafe fn pass1_1030_5d3c(
    mut param_1: u16,
    param_2: *mut u8,
    param_3: *mut u16,
    mut param_4: u32,
) -> *mut u16 {
    let mut in_register_0000000a: u16;
    let mut uVar1: u16;

    pass1_1030_183c(
        param_1,
        CONCAT22(in_register_0000000a, param_2),
        param_3,
        0x1,
        0x4,
        0x1000000,
        param_4,
    );
    uVar1 = (param_3 >> 0x10);
    (param_3 + 0x10) = 0;
    *param_3 = 0x613e;
    (param_3 + 0x2) = 0x1030;
    return param_3;
}
pub unsafe fn pass1_1030_5d78(param_1: *mut u16) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    *param_1 = 0x613e;
    (iVar3 + 0x2) = 0x1030;
    pcVar2 = *(iVar3 + 0x10);
    uVar1 = (iVar3 + 0x12);
    if ((uVar1 | pcVar2) != 0) {
        pass1_1030_8480((pcVar2 & 0xffff | uVar1 << 0x10));
        fn_ptr_1000_17ce(pcVar2);
    }
    pass1_1030_18b2(param_1);
    return;
}



pub unsafe fn file_1030_5e70(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut uVar1: u32;
    let mut puVar2: *mut u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut puVar5: *mut u8;
    let mut BVar6: bool;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_register_0000000a: u16;
    let mut pSVar9: *mut StructD;
    let mut paVar10: *mut Struct57;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut puVar13: *mut u16;
    let mut puVar14: *mut u32;
    let mut in_stack_0000fa88: u16;
    let mut in_stack_0000fbac: u16;
    let mut in_stack_0000fbb2: u16;
    let mut in_stack_0000fbb6: u16;
    let mut iVar15: i16;
    let mut uVar16: u16;
    let mut uStack1034: u32;
    let mut local_402: [u8; 0x400] = [0; 0x400];

    pSVar9 = CONCAT22(in_register_0000000a, param_2);
    iVar15 = param_3;
    uVar16 = (param_3 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar9);
        } else {
            pSVar9 = (pSVar9 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
        }
        uVar4 = fn_ptr_op_1000_1708(0x10, 0x0, 0x1, PTR_LOOP_1050_5f2c, pSVar9);
        uVar7 = pSVar9;
        uStack1034 = CONCAT22(uVar7, uVar4);
        paVar10 = (pSVar9 & 0xffff0000 | (uVar7 | uVar4));
        if ((uVar7 | uVar4) == 0) {
            (iVar15 + 0x10) = 0;
        } else {
            puVar13 = pass1_1008_3e38(CONCAT22(uVar7, uVar4 + 0x4));
            paVar10 = (pSVar9 & 0xffff0000 | puVar13 >> 0x10);
            (iVar15 + 0x10) = uStack1034;
        }
        puVar5 = local_402;
        read_file_1008_7c6e(param_4, (param_4 >> 0x10), CONCAT22(0x1050, puVar5));
        if (puVar5.is_null() == false) {
            uVar4 = str_op_1008_60e8(paVar10, CONCAT22(0x1050, local_402));
            puVar2 = (iVar15 + 0x10);
            *puVar2 = uVar4;
            (puVar2 + 0x2) = paVar10;
            uVar1 = (iVar15 + 0x10);
            BVar6 = read_file_1008_7bc8(param_4, (uVar1 & 0xffff0000 | (uVar1 + 0x4)));
            if (BVar6 != 0) {
                uVar1 = (iVar15 + 0x10);
                uVar7 = uVar1 + 0xa;
                BVar6 = read_file_1008_7dee(param_4, (uVar1 & 0xffff0000 | uVar7), 0x2);
                if (BVar6 != 0) {
                    uVar3 = (iVar15 + 0x10);
                    uVar12 = (uVar3 >> 0x10);
                    iVar11 = uVar3;
                    if ((iVar11 + 0xa) == 0) {
                        //
                        // LAB_1030_5fb7:
                        puVar14 = mixed_1010_20ba(
                            paVar10,
                            _u16_1050_0ed0,
                            CONCAT22(uVar7, 0x2f),
                            in_stack_0000fa88,
                            in_stack_0000fbac,
                            in_stack_0000fbb2,
                            in_stack_0000fbb6,
                        );
                        pass1_1018_04ca(puVar14, (iVar15 + 0x4));
                        return;
                    }
                    uVar8 = (iVar11 + 0xa) * 0x2;
                    uVar7 = uVar8;
                    mem_op_1000_179c(uVar8, paVar10);
                    uVar3 = (iVar15 + 0x10);
                    uVar12 = (uVar3 >> 0x10);
                    iVar11 = uVar3;
                    (iVar11 + 0xc) = uVar8;
                    (iVar11 + 0xe) = paVar10;
                    uVar3 = (iVar15 + 0x10);
                    BVar6 = read_file_1008_7dee(param_4, (uVar3 + 0xc), uVar7);
                    //          if (BVar6 != 0) goto LAB_1030_5fb7;
                }
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}
pub unsafe fn pass1_1030_5fe2(mut param_1: u32, mut param_2: u32) {
    (param_1 + 0x10) = param_2;
    return;
}


pub unsafe fn pass1_1030_5ff6(mut param_1: u16, param_2: *mut Struct57, mut param_3: u32) {
    let mut puVar1: *mut u16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_6c: [u8; 0x58] = [0; 0x58];
    let mut uStack20: u32;
    let mut uStack16: u32;
    let mut uStack12: u32;
    let mut uStack8: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    uVar7 = (param_3 >> 0x10);
    iVar6 = param_3;
    if ((iVar6 + 0xc) == 0) {
        mem_op_1000_179c(0x18, param_2);
        uStack6 = param_2;
        param_2 = (param_2 & 0xffff0000 | (uStack6 | param_1));
        uStack8 = param_1;
        if ((uStack6 | param_1) == 0) {
            (iVar6 + 0xc) = 0;
        } else {
            struct_op_1030_1cd8(CONCAT22(uStack6, param_1), 0x5, 0x5);
            (iVar6 + 0xc) = param_1;
            (iVar6 + 0xe) = param_2;
        }
    }
    //   for (uStack4 = 0; uVar3 = (iVar6 + 0x10), puVar1 = (uVar3 + 0xa),
    //   uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 += 1)
    uStack4 = 0;
    uVar3 = iVar6 + 0x10;
    puVar1 = uVar3 + 0xa;
    while uStack4 <= *puVar1 && *puVar1 != uStack4 {
        uStack12 = pass1_1028_e2e0(param_2, _PTR_LOOP_1050_65e2, 0x2);
        param_2 = (param_2 & 0xffff0000 | uStack12 >> 0x10);
        iVar4 = uStack12;
        uVar3 = (iVar6 + 0xc);
        ppcVar2 = ((iVar6 + 0xc) + 0x8);
        (**ppcVar2)(
            0x1028,
            uVar3,
            (uVar3 >> 0x10),
            iVar4,
            (uStack12 >> 0x10),
            uStack4,
            0x0,
        );
        pass1_1030_8344(_u16_1050_5748, uStack12);
        uStack16 = CONCAT22(param_2, iVar4);
        uStack20 = (iVar4 + 0x10);
        if ((uStack20 + 0x2) == 0) {
            sys_1000_3f9c(
                CONCAT22(0x1050, local_6c),
                s__s__d_1050_573a,
                (iVar6 + 0x10),
            );
            uVar5 = str_op_1008_60e8(param_2, CONCAT22(0x1050, local_6c));
            uVar8 = (uStack20 >> 0x10);
            (uStack20 + 0x2) = uVar5;
            (uStack20 + 0x4) = param_2;
        }
        uStack4 += 1;
    }
    return;
}
