
pub fn pass1_1018_017c(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = param_2;
    pass1_1010_1f62((param_1 & 0xffff | uVar1 << 0x10), 0x4);
    return;
}


pub fn pass1_1018_0196(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
    mut param_5: u32,
) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut pSVar7: *mut StructD;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u32;

    pSVar7 = CONCAT22(in_register_0000000a, param_2);
    pass1_1030_8344(_u16_1050_5748, param_5);
    uVar9 = (param_3 >> 0x10);
    iVar8 = param_3;
    if ((iVar8 + 0x2c) == 0) {
        (iVar8 + 0x32) = 0x5;
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
        } else {
            pSVar7 = (_PTR_LOOP_1050_5f2c >> 0x10);
        }
        uVar4 = fn_ptr_op_1000_1708(0x1e, 0x0, 0x1, PTR_LOOP_1050_5f2c, pSVar7);
    } else {
        uVar4 = (iVar8 + 0x30) + 1;
        //    if (uVar4 < (iVar8 + 0x32)) goto LAB_1018_022a;
        piVar1 = (iVar8 + 0x32);
        *piVar1 = *piVar1 + 0x5;
        uVar3 = (iVar8 + 0x2c);
        uVar10 = pass1_1000_0ed4(0x1, (iVar8 + 0x32) * 0x6, 0x0, uVar3, (uVar3 >> 0x10));
        pSVar7 = (uVar10 >> 0x10);
        uVar4 = uVar10;
    }
    (iVar8 + 0x2c) = uVar4;
    (iVar8 + 0x2e) = pSVar7; //
                             // LAB_1018_022a:
    uVar6 = SUB42(pSVar7, 0x0);
    pass1_1030_8344(_u16_1050_5748, param_4);
    uVar5 = (uVar4 + 0x10);
    pass1_1030_8344(_u16_1050_5748, uVar5);
    iVar2 = (iVar8 + 0x30);
    piVar1 = (iVar8 + 0x30);
    *piVar1 = *piVar1 + 1;
    uVar10 = (iVar8 + 0x2c);
    pass1_1008_3f62(
        (uVar10 & 0xffff0000 | (uVar10 + iVar2 * 0x6)),
        CONCAT22(uVar6, uVar5 + 0xc),
    );
    return;
}


pub fn pass1_1018_028c(
    mut param_1: u16,
    param_2: *mut StructD,
    mut param_3: u32,
    mut param_4: u32,
    mut param_5: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u8;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar13: u16;
    let mut paVar10: *mut Struct57;
    let mut uVar12: u32;
    let mut iVar14: i16;
    let mut uVar15: u16;
    let mut in_stack_0000fe32: u16;
    let mut in_stack_0000fe76: u16;
    let mut in_stack_0000ff56: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ff60: u16;
    let mut in_stack_0000ff8a: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa4: u16;
    let mut iStack36: i16;
    let mut puStack28: *mut u32;
    let mut local_18: [u8; 0x4] = [0; 0x4];
    let mut uStack20: u16;
    let mut paStack12: *mut astruct_74;
    let mut iStack8: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;
    let mut uVar11: u32;

    pass1_1030_8344(_u16_1050_5748, param_4);
    uStack4 = SUB42(param_2, 0x0);
    uStack6 = param_1;
    iStack8 = pass1_1030_5b00(CONCAT22(uStack4, param_1));
    paStack12 = mixed_1010_20ba(
        param_2,
        _u16_1050_0ed0,
        CONCAT22(param_5, iStack8),
        in_stack_0000fe76,
        in_stack_0000ff9a,
        in_stack_0000ffa0,
        in_stack_0000ffa4,
    );
    uVar13 = (param_2 >> 0x10);
    pass1_1008_6c90(CONCAT22(0x1050, local_18));
    pass1_1018_0b1e(paStack12, CONCAT22(0x1050, local_18));
    paVar10 = CONCAT22(uVar13, uStack20 >> 0xf);
    if ((uStack20 >> 0xf | uStack20) == 0) {
        puVar3 = local_18;
        pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(0x1050, puVar3), param_4);
    } else {
        puVar3 = local_18;
        pass1_1030_62e4(_PTR_LOOP_1050_5740, CONCAT22(0x1050, puVar3), param_4);
    }
    uVar8 = paVar10;
    puStack28 = CONCAT22(uVar8, puVar3);
    uVar4 = uVar8 | puVar3;
    if (uVar4 == 0) {
        return;
    }
    pass1_1018_04f2(param_3);
    uVar15 = 0x1000;
    mem_op_1000_179c(0x1c, paVar10);
    uVar9 = paVar10 | uVar4;
    uVar11 = paVar10 & 0xffff0000 | uVar9;
    iVar14 = param_3;
    uVar13 = (param_3 >> 0x10);
    uVar5 = uVar4;
    if (uVar9 == 0) {
        (iVar14 + 0x12) = 0;
    } else {
        uVar15 = 0x1008;
        struct_op_1008_8e9e(CONCAT22(paVar10, uVar4), 0x6, 0x24);
        (iVar14 + 0x12) = uVar5;
        (iVar14 + 0x14) = uVar11;
    }
    ppcVar2 = (*puStack28 + 0x10);
    (**ppcVar2)(uVar15, puVar3, uVar8, uVar4);
    //   for (iStack36 = 0; iStack36 < uVar5; iStack36 += 1)
    for iStack36 in 0..uVar5 {
        uVar7 = iStack36;
        ppcVar2 = (*puStack28 + 0x4);
        (**ppcVar2)();
        uVar4 = uVar11 | uVar7;
        uVar12 = uVar11 & 0xffff0000 | uVar4;
        if (uVar4 != 0) {
            iVar6 = iStack36 / 0x6;
            uVar12 = uVar11 & 0xffff0000 | iStack36 % 0x6 & 0xffff;
            uVar1 = (iVar14 + 0xe);
            pass1_1018_dd7c(
                uVar12,
                uVar1,
                (uVar1 >> 0x10),
                CONCAT22(iStack36 % 0x6, iVar6),
                uVar7 & 0xffff | uVar11 << 0x10,
                in_stack_0000ff8a,
                in_stack_0000ff5c,
                in_stack_0000ff60,
                in_stack_0000ff56,
                in_stack_0000fe32,
            );
            pass1_1008_8faa((iVar14 + 0x12), CONCAT22(uVar12, iVar6));
        }
        uVar11 = uVar12;
    }
    return;
}



pub fn pass1_1018_0412(
    param_1: *mut Struct27,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut pstruct97_1: *mut astruct_97;
    let mut struct97_128: astruct_97 = astruct_97::default();
    let mut uStack4: u16;

    uStack4 = 0;
    if (((0x72 < param_4) && (!SBORROW2(param_4, 0x73)))
        && (param_4 == 0x75
            || (param_4 - 0x74) < 0x1
            || (0x0 < (param_4 - 0x76) && ((param_4 - 0x77) < 0x2))))
    {
        uStack4 = 0x1;
    }
    struct_op_1028_933c(
        CONCAT22(0x1050, &struct97_128),
        param_2,
        uStack4,
        param_4,
        param_3,
        (param_3 >> 0x10),
        (param_1 + 0x24),
        param_5,
    );
    pstruct97_1 = &struct97_128;
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, pstruct97_1));
    if (pstruct97_1.is_null() == false) {
        pass1_1010_1f62(param_1, 0x6);
    }
    return;
}
pub fn pass1_1018_04a4(mut param_1: u32, mut param_2: u32) {
    (param_1 + 0x16) = param_2;
    return;
}

pub fn pass1_1018_04b8(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16));
}
pub fn pass1_1018_04ca(mut param_1: u32, mut param_2: u32) {
    (param_1 + 0x1a) = param_2;
    return;
}
pub fn pass1_1018_04de(mut param_1: u32, mut param_2: u32) {
    (param_1 + 0x20) = param_2;
    return;
}
pub fn pass1_1018_04f2(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0x12) = 0;
    return;
}



pub fn struct_1018_0570(param_1: *mut Struct19, mut param_2: u16, mut param_3: u16) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u16;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut uVar6: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut uVar10: u16;
    let mut uVar9: *mut Struct19;
    let mut uVar11: *mut Struct19;

    uVar6 = (in_EDX >> 0x10);
    uVar9 = param_1;
    uVar10 = (param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, 0x0, param_2);
    uVar9.field17_0x20 = 0x389a;
    uVar9.field18_0x22 = 0x1008;
    uVar9.field17_0x20 = 0x3aa8;
    uVar9.field18_0x22 = 0x1008;
    uVar9.field19_0x24 = 0;
    uVar9.field22_0x2c = 0;
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&uVar9.field24_0x30)));
    puVar7 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&uVar9.field26_0x36)));
    paVar5 = CONCAT22(uVar6, (puVar7 >> 0x10));
    uVar9.field_0x3c = 0;
    pass1_1008_6c90((param_1 & 0xffff0000 | ZEXT24(&uVar9.field34_0x40)));
    uVar6 = 0;
    uVar9.field_0x4c = 0;
    uVar9.field_0x5a = 0;
    uVar9.field53_0x5e = 0;
    (uVar9 + 1) = 0;
    uVar9[0x1].field2_0x4 = 0xff00;
    (uVar9[0x1].field2_0x4 + 0x2) = 0;
    uVar9[0x1].field3_0x8 = 0x10000fb;
    uVar9[0x1].ver_res_0xc = 0x10000f9;
    uVar9[0x1].field8_0x10 = 0x10000ff;
    uVar9[0x1].field10_0x14 = 0x10000fe;
    uVar9[0x1].field12_0x18 = 0x10000fc;
    uVar9[0x1].field15_0x1c = 0;
    uVar9[0x1].field17_0x20 = 0;
    uVar9[0x1].field19_0x24 = 0x1;
    uVar9[0x1].field20_0x26 = 0;
    (uVar9[0x1].field20_0x26 + 0x2) = 0;
    uVar9[0x1].field22_0x2c = 0;
    uVar9[0x1].field23_0x2e = 0;
    uVar9[0x1].field25_0x32 = 0;
    (&uVar9[0x1].field25_0x32 + 0x2) = 0;
    uVar9[0x1].field27_0x38 = 0;
    uVar9[0x1].field_0x3a = 0;
    (uVar9[0x1].field34_0x40 + 0x2) = 0;
    uVar9[0x1].field36_0x46 = 0xffff;
    (uVar9[0x1].field36_0x46 + 0x2) = 0;
    param_1.offset_0x0 = 0x1874;
    uVar9.segment_0x2 = 0x1018;
    uVar9.field17_0x20 = 0x18b0;
    uVar9.field18_0x22 = 0x1018;
    if ((PTR_LOOP_1050_3960.is_null()) && (_PTR_LOOP_1050_3962 == 0)) {
        mem_op_1000_179c(0x8, paVar5);
        _PTR_LOOP_1050_3962 = CONCAT22(paVar5, uVar6);
        pass1_1000_4906(CONCAT22(paVar5, uVar6), NULL, 0x8);
    }
    PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + 1;
    puVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2f),
        in_stack_0000fe98,
        in_stack_0000ffbc,
        in_stack_0000ffc2,
        in_stack_0000ffc6,
    );
    paVar5 = (paVar5 & 0xffff0000);
    uVar9.field22_0x2c = puVar8;
    uVar9.field23_0x2e = (puVar8 >> 0x10);
    if (param_1.is_null()) {
        puVar3 = null_mut();
    } else {
        paVar5 = (paVar5 | uVar10);
        puVar3 = &uVar9.field17_0x20;
    }
    uVar1 = &uVar9.field22_0x2c;
    uVar6 = uVar1;
    ppcVar2 = (*&uVar9.field22_0x2c + 0x4);
    (**ppcVar2)(0x1010, uVar6, (uVar1 >> 0x10), 0x0, puVar3, paVar5);
    puVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(uVar6, 0x2),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    paVar5 = (paVar5 & 0xffff0000 | puVar8 >> 0x10);
    if ((puVar8 + 0x80) != 0) {
        uVar9[0x1].field19_0x24 = 0x2;
    }
    puVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(uVar6, 0x9),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    paVar5 = (paVar5 & 0xffff0000 | puVar8 >> 0x10);
    uVar9[0x1].field_0x3e = puVar8;
    uVar9[0x1].field34_0x40 = (puVar8 >> 0x10);
    uVar4 = pass1_1010_65d0(puVar8 & 0xffff0000 | &uVar9[0x1].field_0x3e, 0x88);
    if (uVar4 != 0) {
        (&uVar9[0x1].field36_0x46 + 0x2) = 0x1;
    }
    puVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(uVar6, 0x3b),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    (&uVar9[0x1].field34_0x40 + 0x2) = puVar8;
    uVar9[0x1].field35_0x44 = (puVar8 >> 0x10);
    return;
}


pub fn pass1_1018_078e(param_1: *mut StructD) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut puVar3: *mut u16;
    let mut pSVar4: *mut StructD;
    let mut pstruct_5: *mut StructD;
    let mut uVar6: *mut StructD;
    let mut puStack26: *mut u16;
    let mut pcStack6: *mut c_char;

    uVar6 = (param_1 >> 0x10);
    pstruct_5 = param_1;
    param_1.address_offset_field_0x0 = 0x1874;
    pstruct_5.address_offset_field_0x2 = 0x1018;
    pstruct_5.field19_0x20 = 0x18b0;
    pstruct_5.field20_0x22 = 0x1018;
    PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 -0x1;
    (_PTR_LOOP_1050_3962 + pstruct_5.field11_0x12 * 0x2 -0x4) = 0;
    if (PTR_LOOP_1050_3960.is_null()) {
        fn_ptr_1000_17ce(_PTR_LOOP_1050_3962);
        _PTR_LOOP_1050_3962 = null_mut();
    }
    fn_ptr_1000_17ce(*&pstruct_5.field_0x94);
    fn_ptr_1000_17ce(*&pstruct_5.field_0x9a);
    fn_ptr_1000_17ce(*&pstruct_5.field_0x88);
    fn_ptr_1000_17ce(*&pstruct_5.field_0x8e);
    if (pstruct_5.field29_0x2c.is_null() == false) {
        if (param_1.is_null()) {
            puVar3 = null_mut();
            pSVar4 = null_mut();
        } else {
            puVar3 = &pstruct_5.field19_0x20;
            pSVar4 = uVar6;
        }
        pass1_1010_1ea6(pstruct_5.field29_0x2c, CONCAT22(pSVar4, puVar3));
    }
    if (&pstruct_5.field_0x9e != 0) {
        if (param_1.is_null()) {
            puVar3 = null_mut();
            pSVar4 = null_mut();
        } else {
            puVar3 = &pstruct_5.field19_0x20;
            pSVar4 = uVar6;
        }
        pass1_1010_1ea6(&pstruct_5.field_0x9e, CONCAT22(pSVar4, puVar3));
    }
    uVar1 = &pstruct_5.field_0x60;
    uVar2 = &pstruct_5.field_0x62;
    pcStack6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1008_5118(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(pcStack6);
    }
    pstruct_5.field_0x4c = 0;
    if (param_1.is_null()) {
        puVar3 = null_mut();
        uVar6 = null_mut();
    } else {
        puVar3 = &pstruct_5.field19_0x20;
    }
    puStack26 = CONCAT22(uVar6, puVar3);
    *puStack26 = 0x389a;
    puVar3[0x1] = 0x1008;
    pass1_1010_1d80(param_1);
    return;
}


pub fn pass1_1018_0902(param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    astruct_76 * *ppaVar3;
    astruct_76 * *ppaVar4;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut puVar9: *mut u32;
    let mut puVar10: *mut u32;

    puVar10 = (param_1 & 0xffff0000 | (param_1 + 0x28));
    ppaVar3 = (param_1 + 0x24);
    puVar9 = (param_1 & 0xffff0000 | ZEXT24(ppaVar3));
    uVar6 = param_1;
    ppaVar4 = ppaVar3;
    pass1_1030_8344(_u16_1050_5748, param_2);
    pass1_1030_5a52(CONCAT22(uVar6, ppaVar4), puVar9, puVar10);
    uVar7 = pass1_1008_4772(*ppaVar3);
    (param_1 + 0x5a) = uVar7;
    (param_1 + 0x5c) = (uVar7 >> 0x10);
    iVar5 = pass1_1018_17f0();
    (param_1 + 0x12) = iVar5 + 2;
    (iVar5 * 0x2 + _PTR_LOOP_1050_3962) = 0x1;
    ppcVar2 = (*param_1 + 0x18);
    (**ppcVar2)();
    (param_1 + 0x3c) = param_2;
    uVar1 = (param_1 + 0x2c);
    uVar8 = pass1_1010_ec18(
        param_2,
        param_2,
        uVar1,
        (uVar1 >> 0x10),
        param_2 & 0xffff0000 | (param_1 + 0x3c),
    );
    (param_1 + 0x7c) = uVar8;
    (param_1 + 0x7e) = (uVar8 >> 0x10);
    return;
}



pub fn pass1_1018_0a50(mut param_1: u32) -> u32 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x84) == 0x2) {
        return CONCAT22((iVar1 + 0x2a), (iVar1 + 0x28));
    }
    return CONCAT22((iVar1 + 0x26), (iVar1 + 0x24));
}
pub fn pass1_1018_0a76(mut param_1: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x84) == 1) {
        uVar1 = 0x2;
    } else {
        uVar1 = 0x1;
    }
    (param_1 + 0x84) = uVar1;
    pass1_1010_1f62((param_1 & 0xffff | uVar2 << 0x10), 0x4);
    return;
}

pub fn pass1_1018_0ac0(mut param_1: u32, param_2: *mut StructA) {
    (param_1 + 0x80) = param_2;
    return;
}

pub fn pass1_1018_0ad4(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x82), (param_1 + 0x80));
}
pub fn pass1_1018_0ae8(mut param_1: u32, mut param_2: u16) {
    (param_1 + 0x5e) = param_2;
    return;
}

pub fn pass1_1018_0afa(mut param_1: u32) -> u16 {
    return (param_1 + 0x5e);
}

pub fn pass1_1018_0b08(mut param_1: u32) -> u32 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar1 = (param_1 + 0x7c);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0x6), (iVar2 + 0x4));
}
pub fn pass1_1018_0b1e(param_1: *mut astruct_74, param_2: *mut u16) {
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_74;
    let mut uVar3: u16;
    let mut local_8: u16;
    let mut local_6: i16;
    let mut local_4: i16;

    iVar3 = param_1;
    iVar3 = &iVar3.field_0x30;
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | ZEXT24(iVar3)),
        CONCAT22(0x1050, &local_8),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    if (local_4 -0x3 < 1) {
        local_4 = 0x3;
    }
    if (local_6 -0x3 < 1) {
        local_6 = 0x3;
    }
    uVar3 = (param_1 >> 0x10);
    uVar2 = iVar3.field90_0x5a;
    iVar1 = (uVar2 + 0x4);
    if (iVar1 <= local_4 + 0x2) {
        local_4 = iVar1 -0x3;
    }
    uVar2 = iVar3.field90_0x5a;
    iVar1 = (uVar2 + 0x8);
    if (iVar1 <= local_6 + 0x2) {
        local_6 = iVar1 -0x3;
    }
    pass1_1008_6cec(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x40)),
        local_8,
        CONCAT22(local_4 + 0x2, local_6 + 0x2),
        local_8,
        CONCAT22(local_4 -0x3, local_6 -0x3),
    );
    pass1_1008_3f62(param_2, (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x40)));
    pass1_1008_3f62(
        (param_2 & 0xffff0000 | (param_2 + 0x6)),
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x46)),
    );
    return;
}
pub fn pass1_1018_0bf4(
    mut param_1: i16,
    mut param_2: u32,
    mut param_3: i16,
    mut param_4: u32,
    mut param_5: u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut pSVar5: *mut StructD;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut local_14: [u8; 0xc] = [0; 0xc];
    let mut local_8: u16;
    let mut local_6: u32;

    uVar7 = (param_4 >> 0x10);
    match param_3 {
        0x1 => {
            (param_2 + 0xc) = 0;
            (param_2 + 0x7e) = 0;
            return;
        }
        0x4 => {
            pass1_1008_3eb4(
                (param_2 & 0xffff0000 | (param_2 + 0x10)),
                CONCAT22(0x1050, &local_8),
                CONCAT22(0x1050, &local_6),
                CONCAT22(0x1050, &local_6 + 0x2),
            );
            uVar1 = (param_2 + 0xc);
            local_8 = (uVar1 + 0x1e);
            pass1_1008_3e76(
                (param_2 & 0xffff0000 | (param_2 + 0x10)),
                local_8,
                local_6,
                (local_6 >> 0x10),
            );
            pass1_1008_6c90(CONCAT22(0x1050, local_14));
            pass1_1018_0b1e(
                (param_2 & 0xffff0000 | (param_2 - 0x20)),
                CONCAT22(0x1050, local_14),
            );
        }
        // TODO: goto LAB_1018_0c71;
        0x5 | 0x6 => {
            uVar6 = param_4 & 0xffff0000 | param_2;
            uVar2 = param_2 - 0x20;
            pass1_1018_0dc6(uVar6, (param_2 & 0xffff0000 | uVar2));
            pass1_1018_10c4(uVar6, param_2 & 0xffff0000 | uVar2);
            pass1_1018_1346(uVar6, (param_2 & 0xffff0000 | uVar2));
            uVar7 = (uVar6 >> 0x10); //
                                     // LAB_1018_0c71:
            (param_2 + 0x2c) = 0;
            lVar4 = (param_2 + 0x1c);
            pSVar5 = CONCAT22(uVar7, (param_2 + 0x1e));
            uVar1 = (param_2 + 0xc);
            if ((uVar1 + 0x20) == lVar4) {
                pass1_1018_028c(lVar4, pSVar5, (param_2 + 0xc), (param_2 + 0x1c), param_5);
                (param_2 + 0x2c) = lVar4;
                (param_2 + 0x2e) = pSVar5;
                pass1_1010_1f62((param_2 & 0xffff0000 | (param_2 - 0x20)), param_3);
                return;
            }
        }
        // break;
        0x14 => {
            uVar1 = (param_2 + 0xc);
            if ((uVar1 + 0x20) != (param_2 + 0x1c)) {
                post_win_msg_1020_291a((param_2 + 0x60));
                return;
            }
        }
        // break;
        0x15 => {
            uVar3 = pass1_1010_65d0((param_2 + 0x7e), 0x88);
            if (uVar3 != 0) {
                (param_2 + 0x88) = 0x1;
                return;
            }
        }
    };
    return;
}

pub fn pass1_1018_0d9a(mut param_1: u32, param_2: *mut u16, param_3: *mut u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x7c);
    *param_3 = (uVar1 + 0x16);
    uVar1 = (param_1 + 0x7c);
    *param_2 = (uVar1 + 0x1a);
    return;
}



pub fn pass1_1018_0dc6(param_1: *mut Struct57, param_2: *mut astruct_91) {
    let mut puVar1: *mut u16;
    let mut paVar2: *mut astruct_92;
    let mut iVar3: i16;
    let mut puVar4: *mut u32;
    let mut pcVar5: *mut c_char;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut iVar13: *mut astruct_91;
    let mut uVar9: u16;
    let mut local_32: u32;
    let mut uStack46: u16;
    let mut uStack44: u32;
    let mut pcStack40: *mut c_char;
    let mut pcStack36: *mut c_char;
    let mut pcStack32: *mut c_char;
    let mut uStack28: u32;
    let mut uStack24: u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    uVar9 = (param_2 >> 0x10);
    iVar13 = param_2;
    pcStack36 = iVar13.field147_0x94;
    fn_ptr_1000_17ce(pcStack36);
    pcStack40 = iVar13.field149_0x9a;
    pcStack32 = pcStack40;
    fn_ptr_1000_17ce(pcStack40);
    iVar13.field147_0x94 = 0;
    iVar13.field149_0x9a = 0;
    iVar13.field146_0x92 = 0;
    iVar13.field148_0x98 = 0;
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        uVar6 = param_1;
        uStack24 = CONCAT22(uVar6, paVar2);
        param_1 = (param_1 & 0xffff0000 | (uVar6 | paVar2));
        if ((uVar6 | paVar2) == 0) {
            break;
        }
        pcVar5 = paVar2[0x1c].field4_0x8;
        pcStack40 = pcVar5;
        if (pcVar5 == 0x8000001) {
            puVar1 = &iVar13.field146_0x92;
            *puVar1 = *puVar1 + 1;
        } else if ((iVar13.field157_0xa8 != 0)
            || (
                pass1_1008_dfa6(iVar13.field154_0xa2, paVar2.field3_0x4, 0x4000001),
                pcVar5 != 0,
            ))
        {
            puVar1 = &iVar13.field148_0x98;
            *puVar1 = *puVar1 + 1;
        }
    }
    if (iVar13.field146_0x92 != 0) {
        uVar6 = iVar13.field146_0x92;
        uStack44 = uStack44 & 0xffff0000 | uVar6;
        uVar6 *= 0x6;
        mem_op_1000_179c(uVar6, param_1);
        uVar7 = param_1;
        pcStack32 = CONCAT22(uVar7, uVar6);
        param_1 = (param_1 & 0xffff0000 | (uVar7 | uVar6));
        if ((uVar7 | uVar6) == 0) {
            iVar13.field147_0x94 = 0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack44, 0x6, uVar6, uVar7);
            iVar13.field147_0x94 = pcStack32;
        }
    }
    if (iVar13.field148_0x98 != 0) {
        uVar6 = iVar13.field148_0x98;
        uStack44 = uStack44 & 0xffff0000 | uVar6;
        uVar6 *= 0x6;
        mem_op_1000_179c(uVar6, param_1);
        uVar7 = param_1;
        pcStack32 = CONCAT22(uVar7, uVar6);
        if ((uVar7 | uVar6) == 0) {
            iVar13.field149_0x9a = 0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack44, 0x6, uVar6, uVar7);
            iVar13.field149_0x9a = pcStack32;
        }
    }
    if (local_14.field6_0x10 != 0) {
        local_14.field5_0xc = 0x1;
        local_14.field5_0xc = 0;
    }
    uVar8 = local_14.field5_0xc;
    uStack28 = 0;
    local_14.field4_0x8 = local_14.field5_0xc;
    local_14.field4_0x8 = local_14.field5_0xc; //
                                               // LAB_1018_0f74:
    uVar6 = uVar8;
    paVar2 = &local_14;
    pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
    uStack24 = CONCAT22(uVar6, paVar2);
    uVar8 = (uVar6 | paVar2);
    if ((uVar6 | paVar2) == 0) {
        return;
    }
    uStack44 = paVar2[0x1c].field4_0x8;
    pcVar5 = *&paVar2.field6_0x10;
    pcStack40 = pcVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, pcVar5);
    pcStack36 = (pcVar5 & 0xffff | uVar8 << 0x10);
    local_32 = (pcVar5 + 0xc);
    uStack46 = (pcVar5 + 0x10);
    puVar4 = &local_32;
    //  if (uStack44 != 0x8000001) goto LAB_1018_0ffc;
    iVar3 = &iVar13.field147_0x94;
    uVar6 = (&iVar13.field147_0x94 + 2);
    uStack28 = uStack28 & 0xffff | (uStack28 + 1) << 0x10;
    //   goto LAB_1018_0fe8;//
    // LAB_1018_0ffc:
    if ((iVar13.field157_0xa8 != 0)
        || (
            pass1_1008_dfa6(iVar13.field154_0xa2, (uStack24 + 0x4), 0x4000001),
            puVar4.is_null() == false,
        ))
    {
        iVar3 = &iVar13.field149_0x9a;
        uVar6 = (&iVar13.field149_0x9a + 2);
        uStack28 = uStack28 & 0xffff0000 | (uStack28 + 1);
        uStack28 = uStack28; //
                             // LAB_1018_0fe8:
        uVar8 = uVar6;
        pass1_1008_3f62(
            CONCAT22(uVar6, iVar3 + uStack28 * 0x6),
            CONCAT22(0x1050, &local_32),
        );
    }
    //   goto LAB_1018_0f74;
}
