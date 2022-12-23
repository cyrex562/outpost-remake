


pub unsafe fn struct_1030_11aa(param_1: *mut astruct_156, param_2: i32, param_3: i32) {
    let mut iVar1: *mut astruct_156;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar1.field1_0x2 = 0x1008;
    iVar1.field2_0x4 = 0;
    iVar1.field3_0x6 = null_mut();
    iVar1.field4_0xa = 0;
    iVar1.field5_0xe = param_3;
    iVar1.field6_0x12 = 0;
    iVar1.field7_0x16 = param_2;
    iVar1.field8_0x1a = 0x1;
    // just 0x1624
    param_1.field0_0x0 = s_462_bmp_1050_1620 + 0x4;
    iVar1.field1_0x2 = 0x1030;
    if (iVar1.field5_0xe == 0) {
        iVar1.field5_0xe = 0x5;
    }
    if (iVar1.field7_0x16 == 0) {
        iVar1.field7_0x16 = 0x5;
    }
    struct_1030_1550(param_1);
    *iVar1.field3_0x6 = 0;
    return;
}
pub unsafe fn pass1_1030_1244(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u32;
    let mut iVar6: *mut StructD;
    let mut iVar7: i16;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uStack6: u32;

    uVar9 = (param_1 >> 0x10);
    iVar6 = param_1;
    param_1.address_offset_field_0x0 = s_462_bmp_1050_1620 + 0x4;
    iVar6.address_offset_field_0x2 = 0x1030;
    if (iVar6.field14_0x1a != 0) {
        uStack6 = 0x1;
        loop {
            puVar1 = &iVar6.field6_0xa;
            if (*puVar1 < uStack6 || *puVar1 == uStack6) {
                break;
            }
            iVar8 = uStack6 * 0x4;
            uVar5 = &iVar6.field_0x6;
            uVar10 = (uVar5 >> 0x10);
            iVar7 = uVar5;
            puVar2 = (iVar7 + iVar8);
            uVar3 = (iVar7 + iVar8 + 2);
            if ((uVar3 | puVar2) != 0) {
                ppcVar4 = *puVar2;
                (**ppcVar4)();
            }
            uStack6 += 0x1;
        }
    }
    fn_ptr_1000_17ce(*&iVar6.field_0x6);
    param_1.address_offset_field_0x0 = 0x389a;
    iVar6.address_offset_field_0x2 = 0x1008;
    return;
}
pub unsafe fn pass1_1030_12ca(param_1: *mut astruct_176) {
    let mut puVar1: *mut u32;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_176;
    let mut uVar3: u16;
    let mut uStack6: u32;

    uStack6 = 0x1;
    loop {
        uVar3 = (param_1 >> 0x10);
        iVar3 = param_1;
        puVar1 = &iVar3.field6_0xa;
        if (*puVar1 < uStack6 || *puVar1 == uStack6) {
            iVar3.field4_0x4 = 0;
            return;
        }
        uVar2 = iVar3.field5_0x6;
        if ((uVar2 + uStack6 * 0x4) == 0) {
            break;
        }
        uStack6 += 0x1;
    }
    return;
}
pub unsafe fn bad_1030_1312() {
    return;
}
pub unsafe fn pass1_1030_1358(
    param_1: *mut astruct_291,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u16;
    let mut lVar3: i32;
    let mut pstruct_291_2: *mut astruct_291;
    let mut iVar4: i16;
    let mut pstruct_291_1: *mut astruct_291;
    let mut uVar5: u16;
    let mut bVar6: bool;

    if (param_4 == 0) {
        return;
    }
    pstruct_291_1 = (param_1 >> 0x10);
    pstruct_291_2 = param_1;
    puVar1 = &pstruct_291_2.field7_0xa;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_291_2.field6_0x6 == 0)) {
        puVar2 = (&pstruct_291_2.field13_0x12 + 2);
        bVar6 = *puVar2 < param_4;
        if ((bVar6 || *puVar2 == param_4)
            && (bVar6
                || (
                    puVar1 = &pstruct_291_2.field13_0x12,
                    puVar1 < param_4 || puVar1 == param_4,
                )))
        {
            struct_1030_1550((param_1 & 0xffff | ZEXT24(pstruct_291_1) << 0x10));
        }
        puVar1 = &pstruct_291_2.field13_0x12;
        if (*puVar1 < param_4 || *puVar1 == param_4) {
            return;
        }
        if (pstruct_291_2.field6_0x6 == 0) {
            return;
        }
        puVar2 = &pstruct_291_2.field8_0xc;
        bVar6 = *puVar2 < param_4;
        if ((bVar6 || *puVar2 == param_4)
            && (bVar6
                || (
                    puVar2 = &pstruct_291_2.field7_0xa,
                    *puVar2 < param_4 || *puVar2 == param_4,
                )))
        {
            pstruct_291_2.field7_0xa = (param_4 + 1);
            pstruct_291_2.field8_0xc = (param_4 + 0x1 >> 0x10);
        }
    }
    lVar3 = pstruct_291_2.field6_0x6;
    uVar5 = (lVar3 >> 0x10);
    iVar4 = lVar3;
    (iVar4 + param_4 * 0x4) = param_2;
    (iVar4 + param_4 * 0x4 + 0x2) = param_3;
    return;
}

pub unsafe fn pass1_1030_13f6(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_291,
    mut param_4: u32,
) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut puStack8: *mut u32;
    let mut uStack4: u16;

    uStack4 = 0;
    bad_1030_1312();
    puStack8 = CONCAT22(param_2, param_1);
    if ((param_2 | param_1) != 0) {
        uStack4 = 0x1;
        uVar2 = (param_3 >> 0x10);
        if (((param_3 + 0x1a) != 0) && ((param_2 | param_1) != 0)) {
            ppcVar1 = *puStack8;
            (**ppcVar1)();
        }
        pass1_1030_1358(param_3, 0x0, 0x0, param_4);
        (param_3 + 0x4) = 0x1;
    }
    return uStack4;
}
pub unsafe fn pass1_1030_145a(param_1: *mut astruct_346, param_2: i32) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut pstruct_1: *mut astruct_346;
    let mut pstruct_1_hi: *mut astruct_346;

    pstruct_1_hi = (param_1 >> 0x10);
    pstruct_1 = param_1;
    fn_ptr_1000_17ce(pstruct_1.field6_0x6);
    pstruct_1.field6_0x6 = 0;
    pstruct_1.field7_0xa = 0;
    uVar1 = pstruct_1.field10_0x16 + param_2;
    uVar2 = (uVar1 >> 0x10);
    if (uVar1 < pstruct_1.field8_0xe) {
        uVar1 = &pstruct_1.field8_0xe;
        uVar2 = (&pstruct_1.field8_0xe + 2);
    }
    pstruct_1.field8_0xe = uVar1;
    (&pstruct_1.field8_0xe + 0x2) = uVar2;
    pstruct_1.field9_0x12 = 0;
    return;
}
pub unsafe fn pass1_1030_14b4(
    param_1: *mut astruct_156,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u16;
    let mut puVar3: *mut u32;
    let mut pstruct_1: *mut astruct_156;
    let mut iVar4: *mut astruct_344;
    let mut pstruct_1_hi: *mut astruct_156;
    let mut uVar4: u16;
    let mut bVar5: bool;

    pstruct_1_hi = (param_1 >> 0x10);
    pstruct_1 = param_1;
    puVar1 = &pstruct_1.field4_0xa;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_1.field3_0x6.is_null())) {
        puVar2 = (&pstruct_1.field6_0x12 + 2);
        bVar5 = *puVar2 < param_4;
        if ((bVar5 || *puVar2 == param_4)
            && (bVar5
                || (
                    puVar3 = &pstruct_1.field6_0x12,
                    puVar3 < param_4 || puVar3 == param_4,
                )))
        {
            struct_1030_1550((param_1 & 0xffff | ZEXT24(pstruct_1_hi) << 0x10));
        }
        puVar1 = &pstruct_1.field6_0x12;
        if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_1.field3_0x6.is_null())) {
            return;
        }
        puVar2 = (&pstruct_1.field4_0xa + 2);
        bVar5 = *puVar2 < param_4;
        if ((bVar5 || *puVar2 == param_4)
            && (bVar5
                || (
                    puVar3 = &pstruct_1.field4_0xa,
                    puVar3 < param_4 || puVar3 == param_4,
                )))
        {
            pstruct_1.field4_0xa = (param_4 + 1);
            (&pstruct_1.field4_0xa + 0x2) = (param_4 + 0x1 >> 0x10);
        }
    }
    puVar3 = pstruct_1.field3_0x6;
    uVar4 = (puVar3 >> 0x10);
    iVar4 = puVar3;
    (iVar4 + param_4 * 0x4) = param_2;
    (iVar4 + param_4 * 0x4 + 0x2) = param_3;
    return;
}
pub unsafe fn pass1_1030_154c() {
    return;
}


pub unsafe fn struct_1030_1550(param_1: *mut astruct_156) {
    let mut plVar1: *mut i32;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut pSVar5: *mut StructD;
    let mut iVar5: *mut astruct_156;
    let mut uVar6: u16;
    let mut lVar7: i32;
    let mut puStack10: *mut u32;
    let mut uStack6: u32;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (iVar5.field6_0x12 == 0) {
        uVar4 = &iVar5.field5_0xe;
        pSVar5 = (in_EDX & 0xffff0000 | (&iVar5.field5_0xe + 0x2));
    } else {
        uVar2 = &iVar5.field6_0x12;
        plVar1 = &iVar5.field7_0x16;
        uVar4 = uVar2 + plVar1;
        pSVar5 = (in_EDX & 0xffff0000
            | ((&iVar5.field6_0x12 + 0x2) + (&iVar5.field7_0x16 + 0x2) + CARRY2(uVar2, plVar1)));
    }
    uStack6 = CONCAT22(pSVar5, uVar4);
    if (iVar5.field3_0x6.is_null()) {
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
        puVar3 = iVar5.field3_0x6;
        lVar7 = pass1_1000_0ed4(
            0x1,
            uVar4 * 0x4,
            (pSVar5 * 0x2 + CARRY2(uVar4, uVar4)) * 0x2 + CARRY2(uVar4 * 0x2, uVar4 * 0x2),
            puVar3,
            (puVar3 >> 0x10),
        );
        PTR_LOOP_1050_5f2e = (lVar7 >> 0x10);
        uVar4 = lVar7;
    }
    puStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    if ((PTR_LOOP_1050_5f2e | uVar4) != 0) {
        iVar5.field6_0x12 = uStack6;
        iVar5.field3_0x6 = puStack10;
    }
    return;
}

pub unsafe fn pass1_1030_15fe(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1030_1244(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn struct_1030_1628(param_1: *mut astruct_180) {
    let mut iVar1: *mut astruct_180;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar1.field1_0x2 = 0x1008;
    iVar1.field_0x4 = 0;
    iVar1.field_0x8 = 0;
    param_1.field0_0x0 = 0x17ba;
    iVar1.field1_0x2 = 0x1030;
    return;
}


pub unsafe fn pass1_1030_165e(
    param_1: *mut Struct57,
    param_2: *mut astruct_175,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut uVar1: u16;
    let mut iVar1: *mut astruct_175;
    let mut uVar2: u16;

    uVar1 = SUB42(param_1, 0x0);
    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2;
    param_2.field0_0x0 = 0x389a;
    iVar1.field1_0x2 = 0x1008;
    iVar1.field2_0x4 = 0;
    iVar1.field4_0x8 = param_4;
    param_2.field0_0x0 = 0x17ba;
    iVar1.field1_0x2 = 0x1030;
    pass1_1030_5c8a(_PTR_LOOP_1050_5736, param_3);
    iVar1.field2_0x4 = param_4;
    iVar1.field3_0x6 = uVar1;
    return;
}
pub unsafe fn pass1_1030_16b2(param_1: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = 0x17ba;
    (param_1 + 0x2) = 0x1030;
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    return;
}
pub unsafe fn pass1_1030_16d6(param_1: *mut astruct_731, mut param_2: u32) {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut in_stack_0000ffd8: HFILE16;
    let mut local_10: [u32; 0x2] = [0; 0x2];
    let mut local_8: u32;

    uVar2 = (param_1 >> 0x10);
    local_10[0] = (param_1 + 0x4);
    BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_10), 0x4, in_stack_0000ffd8);
    if (BVar1 != 0) {
        local_8 = (param_1 + 0x8);
        BVar1 =
            write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, &local_8), 0x4, in_stack_0000ffd8);
        if (BVar1 != 0) {
            return;
        }
    }
    u16_1050_0310 = 0x6d0;
    return;
}
pub unsafe fn file_1030_1730(param_1: *mut astruct_373, param_2: *mut HFILE16) {
    let mut BVar1: bool;

    BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x4)), 0x4);
    if (BVar1 != 0) {
        BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x8)), 0x4);
        if (BVar1 != 0) {
            return;
        }
    }
    u16_1050_0310 = 0x6d2;
    return;
}
pub unsafe fn pass1_1030_177a(mut param_1: u32, mut param_2: u32) {
    (param_1 + 0x8) = param_2;
    return;
}
pub unsafe fn FUN_1030_178e() {
    return;
}



pub unsafe fn struct_1030_17ce(
    param_1: *mut astruct_180,
    mut param_2: u32,
    mut param_3: u32,
    param_4: *mut Struct57,
) -> *mut u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: *mut astruct_180;
    let mut uVar4: *mut astruct_180;

    uVar1 = struct_1030_1628(param_1);
    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar4.field10_0xc = 0;
    param_1.field0_0x0 = 0x1a16;
    iVar4.field1_0x2 = 0x1030;
    if ((param_3 != 0) || (param_2 != 0)) {
        mem_op_1000_179c(0x18, param_4);
        uVar3 = param_4 | uVar1;
        if (uVar3 == 0) {
            uVar2 = 0;
            uVar3 = 0;
        } else {
            uVar2 = struct_op_1030_1cd8(CONCAT22(param_4, uVar1), param_2, param_3);
        }
        iVar4.field10_0xc = uVar2;
        iVar4.field11_0xe = uVar3;
    }
    return &param_1.field0_0x0;
}

pub unsafe fn pass1_1030_183c(
    mut param_1: u16,
    param_2: *mut Struct57,
    param_3: *mut u16,
    mut param_4: u32,
    mut param_5: u32,
    mut param_6: u32,
    mut param_7: u32,
) -> *mut u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;

    pass1_1030_165e(param_2, param_3, param_6, param_7);
    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    (iVar3 + 0xc) = 0;
    *param_3 = 0x1a16;
    (iVar3 + 0x2) = 0x1030;
    if ((param_5 != 0) || (param_4 != 0)) {
        mem_op_1000_179c(0x18, param_2);
        uVar2 = param_2 | param_1;
        if (uVar2 == 0) {
            uVar1 = 0;
            uVar2 = 0;
        } else {
            uVar1 = struct_op_1030_1cd8(CONCAT22(param_2, param_1), param_4, param_5);
        }
        (iVar3 + 0xc) = uVar1;
        (iVar3 + 0xe) = uVar2;
    }
    return param_3;
}
pub unsafe fn pass1_1030_18b2(param_1: *mut u16) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    *param_1 = 0x1a16;
    (iVar4 + 0x2) = 0x1030;
    puVar1 = (iVar4 + 0xc);
    uVar2 = (iVar4 + 0xe);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1030_16b2(param_1);
    return;
}


pub unsafe fn pass1_1030_1978(param_1: u16, param_2: *mut astruct_731, mut param_3: u32) -> u16 {
    pass1_1030_16d6(param_2, param_3);
    if (param_1 != 0) {
        write_to_file_1008_7954(param_1, param_3, (param_2 + 0xc));
        if (param_1 == 0) {
            u16_1050_0310 = 0x6d0;
            return param_1;
        }
        param_1 = 0x1;
    }
    return param_1;
}
pub unsafe fn file_1030_19b4(
    mut param_1: i16,
    mut param_2: u16,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut plVar1: *mut i32;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;

    paVar2 = CONCAT22(in_register_0000000a, param_2);
    file_1030_1730(param_3, param_4);
    if (param_1 != 0) {
        plVar1 = (param_3 & 0xffff0000 | (param_3 + 0xc));
        file_1008_76e4(paVar2, param_4, plVar1);
        if (plVar1 == 0) {
            u16_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}


pub unsafe fn pass1_1030_1a32(param_1: *mut u16, mut param_2: u16, param_3: *mut u8) -> *mut u16 {
    let mut in_register_0000000a: u16;

    pass1_1030_183c(
        param_2,
        CONCAT22(in_register_0000000a, param_3),
        param_1,
        0x1,
        0x16,
        0xff000000,
        0x0,
    );
    PTR_LOOP_1050_5168 = (param_1 >> 0x10);
    PTR_LOOP_1050_5166 = param_1;
    (PTR_LOOP_1050_5166 + 0x10) = 0;
    *param_1 = 0x1cbc;
    (PTR_LOOP_1050_5166 + 0x2) = 0x1030;
    return param_1;
}


pub unsafe fn pass1_1030_1a74(param_1: *mut u16) {
    *param_1 = 0x1cbc;
    (param_1 + 0x2) = 0x1030;
    _PTR_LOOP_1050_5166 = 0;
    pass1_1030_18b2(param_1);
    return;
}





pub unsafe fn struct_op_1030_1cd8(param_1: *mut astruct_75, mut param_2: u32, mut param_3: u32) {
    let mut struct_var1: *mut astruct_75;
    let mut struct_var2: *mut astruct_75;

    struct_var2 = (param_1 >> 0x10);
    struct_var1 = param_1;
    param_1.field0_0x0 = 0x389a;
    struct_var1.field1_0x2 = 0x1008;
    struct_var1.field2_0x4 = 0;
    struct_var1.field3_0x8 = 0;
    struct_var1.field4_0xc = param_3;
    struct_var1.field5_0x10 = 0;
    struct_var1.field6_0x14 = param_2;
    param_1.field0_0x0 = 0x2044;
    struct_var1.field1_0x2 = 0x1030;
    return;
}
pub unsafe fn pass1_1030_1d28(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x2044;
    iVar1.address_offset_field_0x2 = 0x1030;
    fn_ptr_1000_17ce(*&iVar1.hfile_0x4);
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    return;
}


pub unsafe fn pass1_1030_1d58(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;

    ppcVar1 = (param_1 + 0x4);
    uVar2 = (**ppcVar1)();
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2);
    return;
}

pub unsafe fn pass1_1030_1d7c(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u32 {
    let mut uVar1: u32;

    pass1_1030_1d58(param_3);
    if ((param_2 | param_1) != 0) {
        uVar1 = struct_op_1030_73a8(CONCAT22(param_2, param_1), param_1, param_2 | param_1);
        return uVar1;
    }
    return 0x0;
}
