
pub unsafe fn pass1_1028_ad9c(param_1: *mut astruct_338, mut param_2: u16, mut param_3: u32) {
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
        *puStack10 = 0xae56;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_ae2a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_aec0(mut param_1: i16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_3 >> 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x108));
    pass1_1030_375a((param_1 + 0x1f6), 0x0, (param_3 + 0x114));
    return;
}


pub unsafe fn pass1_1028_af08(
    param_1: *mut Struct57,
    param_2: *mut astruct_693,
    mut param_3: u16,
) -> u16 {
    let mut puVar1: *mut u8;
    let mut puVar2: *mut u8;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar6: *mut astruct_693;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut paVar8: *mut astruct_67;
    let mut paVar9: *mut Struct27;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;
    let mut iStack12: i16;
    let mut iStack10: i16;

    puVar7 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    paVar5 = (param_1 & 0xffff0000 | puVar7 >> 0x10);
    puVar1 = PTR_LOOP_1050_13ae -0x1;
    if ((PTR_LOOP_1050_13ae < 1) || (SBORROW2(PTR_LOOP_1050_13ae, 1))) {
        //
        // LAB_1028_af27:
        iStack10 = 0x1;
    } else {
        puVar2 = PTR_LOOP_1050_13ae -0x2;
        if (puVar2.is_null() || puVar1 < 1) {
            iStack12 = 0x1;
            iStack10 = 0x1;
            // TODO: goto LAB_1028_af42;
        }
        puVar1 = PTR_LOOP_1050_13ae -0x4;
        //    if (puVar1.is_null() == false) goto LAB_1028_af27;
        iStack10 = 0x2;
    }
    iStack12 = 0x3;
    puVar2 = puVar1; //
                     // LAB_1028_af42:
    pass1_1008_612e(puVar2, iStack10, iStack12);
    uVar6 = (param_2 >> 0x10);
    iVar6 = param_2;
    iVar6.field273_0x114 = puVar2;
    paVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x37),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    paVar5 = (paVar5 & 0xffff0000 | paVar8 >> 0x10);
    iVar3 = paVar8;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, iVar6.field264_0x108);
    uVar4 = SUB42(paVar5, 0x0);
    post_win_msg_1008_a0e4(
        paVar8,
        0x0,
        iVar6.field273_0x114,
        (iVar3 + 0x208),
        iVar6.field264_0x108,
        0x2,
    );
    paVar9 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2b),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    pass1_1010_043a(paVar9, (iVar3 + 0x4), 0xd);
    return 0x1;
}


pub unsafe fn pass1_1028_afce(
    param_1: *mut astruct_339,
    mut param_2: u16,
    param_3: *mut astruct_340,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_340;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x116, paVar5);
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
        param_1.field258_0x10c = iVar5.field259_0x10c;
        param_1.field259_0x110 = iVar5.field260_0x110;
        param_1.field260_0x114 = iVar5.field261_0x114;
        *puStack10 = 0xb0ce;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_b0a2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_b108(
    param_1: *mut astruct_341,
    mut param_2: u16,
    param_3: *mut astruct_342,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_342;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x110, paVar5);
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
        param_1.field258_0x10c = iVar5.field259_0x10c;
        *puStack10 = 0x6e50;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xb1f4;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_b1c8(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1028_b27e() {
    return;
}


pub unsafe fn FUN_1028_b282(mut param_1: u16, mut param_2: u32, mut param_3: u32) -> BOOL16 {
    let mut in_AX: bool;
    let mut BVar1: bool;
    let mut in_stack_0000ffde: HFILE16;
    let mut auStack12: [u16; 0x5] = [0; 0x5];

    pass1_1030_16d6(param_2, param_3);
    if (in_AX != 0) {
        auStack12[0] = (param_2 + 0xc);
        BVar1 =
            write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, auStack12), 0x2, in_stack_0000ffde);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        in_AX = 0x1;
    }
    return in_AX;
}


pub unsafe fn pass1_1028_b2c8(
    param_1: BOOL16,
    param_2: *mut astruct_373,
    param_3: *mut HFILE16,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut local_4: u16;

    file_1030_1730(param_2, param_3);
    if (param_1 != 0) {
        BVar1 = read_file_1008_7dee(param_3, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d2;
            return BVar1;
        }
        uVar2 = switch_1008_72bc(param_3, local_4);
        (param_2 + 0xc) = uVar2;
        param_1 = 0x1;
    }
    return param_1;
}

pub unsafe fn pass1_1028_b316(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b260(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn struct_1028_b354(param_1: *mut astruct_180) {
    let mut iVar1: *mut astruct_180;
    let mut uVar1: u16;

    struct_1030_1628(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field10_0xc = 0;
    iVar1.field11_0xe = 0;
    iVar1.field12_0x10 = 0;
    iVar1.field13_0x12 = 0;
    iVar1.field16_0x18 = 0;
    iVar1.field17_0x1a = 0;
    iVar1.field18_0x1c = 0;
    param_1.field0_0x0 = 0xcf6a;
    iVar1.field1_0x2 = 0x1028;
    iVar1.field15_0x16 = 0;
    iVar1.field14_0x14 = 0;
    return;
}

pub unsafe fn FUN_1028_b4e6() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1028_b4ec() -> u16 {
    return 0x0;
}

pub unsafe fn pass1_1028_b5a8(mut param_1: u32) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 0x5) {
        return 0x0;
    }
    uVar1 = (param_1 + 0x14);
    return (uVar1 + 0x94);
}

pub unsafe fn pass1_1028_b5ca(mut param_1: u32) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 0x5) {
        return 0x0;
    }
    uVar1 = (param_1 + 0x14);
    return (uVar1 + 0x9c);
}

pub unsafe fn pass1_1028_bb56(mut param_1: u32, mut param_2: u32) {
    pass1_1030_177a(param_1, param_2);
    return;
}

pub unsafe fn pass1_1028_bc02(param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x40);
    (**ppcVar1)();
    return;
}

pub unsafe fn pass1_1028_bc7e(param_1: u32) {
    pass1_1028_bdac(param_1, 0x4);
    return;
}

pub unsafe fn FUN_1028_bf16() {
    return;
}
pub unsafe fn FUN_1028_bf1a() {
    return;
}
pub unsafe fn FUN_1028_bf1e() {
    return;
}


pub unsafe fn pass1_1028_c522(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut u16,
    mut param_4: u32,
    param_5: i32,
) {
    let mut iVar1: i16;
    let mut puVar2: *mut u8;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_4: [u8; 0x2] = [0; 0x2];

    uVar5 = pass1_1030_bcae(local_4, 0x1050);
    uVar4 = (uVar5 >> 0x10);
    iVar1 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4);
    uVar3 = (iVar1 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3);
    puVar2 = local_4;
    pass1_1030_bcde(
        puVar2,
        0x1050,
        uVar3 & 0xffff | uVar4 << 0x10,
        param_3,
        param_5,
    );
    if (puVar2 < 0x0) {
        PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if (puVar2 < 0x1f) {
            return;
        }
        PTR_LOOP_1050_50ca = 0x6b6;
        PTR_LOOP_1050_50cc = puVar2 -0x1e;
    }
    return;
}


pub unsafe fn pass1_1028_c64a(
    mut param_1: u32,
    param_2: *mut u32,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    param_6: i32,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut local_e: [u8; 0x2] = [0; 0x2];
    let mut local_c: i16;
    let mut local_a: i16;
    let mut local_8: u32;
    let mut uStack4: u16;

    local_8 = *param_2;
    uStack4 = (param_2 + 1);
    pass1_1008_3eb4(
        CONCAT22(0x1050, &local_8),
        CONCAT22(0x1050, local_e),
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    local_8 = local_8 & 0xffff | (local_c - 1) << 0x10;
    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    BVar1 = pass1_1028_c5a6(
        &local_8,
        param_3,
        uVar2,
        uVar3,
        0x7b,
        CONCAT22(0x1050, &local_8),
        param_6,
    );
    if (BVar1 == 0) {
        local_8 = local_8 & 0xffff | (local_c + 1) << 0x10;
        BVar1 = pass1_1028_c5a6(
            &local_8,
            param_3,
            uVar2,
            uVar3,
            0x7b,
            CONCAT22(0x1050, &local_8),
            param_6,
        );
        if (BVar1 == 0) {
            local_8 = local_a -0x1;
            local_8 = local_c;
            BVar1 = pass1_1028_c5a6(
                &local_8,
                param_3,
                uVar2,
                uVar3,
                0x7c,
                CONCAT22(0x1050, &local_8),
                param_6,
            );
            if (BVar1 == 0) {
                local_8 = CONCAT22(local_8, local_a + 1);
                BVar1 = pass1_1028_c5a6(
                    &local_8,
                    param_3,
                    uVar2,
                    uVar3,
                    0x7c,
                    CONCAT22(0x1050, &local_8),
                    param_6,
                );
                if (BVar1 == 0) {
                    return BVar1;
                }
            }
        }
    }
    return 0x1;
}


pub unsafe fn pass1_1028_ced2(param_1: *mut astruct_15) -> u16 {
    let mut uVar1: *mut astruct_15;
    let mut bVar1: bool;
    let mut bVar2: bool;
    let mut paVar3: *mut astruct_398;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;

    uVar1 = (param_1 >> 0x10);
    bVar1 = (*(param_1 + 0x1a) & 0x2) == 0;
    if (bVar1) {
        uVar5 = 0;
        uVar6 = 0x23;
        uVar4 = 0x1;
        paVar3 = pass1_1028_b58e((param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
        pass1_1030_7d7c(
            paVar3,
            (paVar3 >> 0x10),
            paVar3,
            uVar4,
            CONCAT22(uVar6, uVar5),
        );
    }
    bVar2 = (*(param_1 + 0x1a) & 1) == 0;
    if (bVar2) {
        uVar5 = 0;
        uVar6 = 0xe;
        uVar4 = 0x1;
        paVar3 = pass1_1028_b58e((param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
        pass1_1030_7d7c(
            paVar3,
            (paVar3 >> 0x10),
            paVar3,
            uVar4,
            CONCAT22(uVar6, uVar5),
        );
    }
    if (bVar2 || bVar1) {
        pass1_1028_bdac(param_1, 0x6);
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1028_cf44(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1028_d222() -> u16 {
    return 0x1;
}

pub unsafe fn FUN_1028_d228() -> u16 {
    return 0x1;
}


pub unsafe fn pass1_1028_d7de(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_57c4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_ec36(
    mut param_1: u16,
    param_2: *mut Struct57,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: i16,
    mut param_6: u16,
    mut param_7: u32,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;

    mem_op_1000_179c(0x14, param_2);
    puVar4 = (param_2 | param_1);
    if (puVar4.is_null()) {
        uVar2 = 0;
        puVar4 = null_mut();
    } else {
        puVar7 = pass1_1030_5d3c(param_1, puVar4, CONCAT22(param_2, param_1), param_7);
        puVar4 = (puVar7 >> 0x10);
        uVar2 = puVar7;
    }
    uVar6 = (param_3 >> 0x10);
    uVar1 = (param_3 + 0x52);
    puVar5 = puVar4;
    uVar3 = uVar2;
    pass1_1030_4594(puVar4, uVar1, (uVar1 >> 0x10), param_5);
    pass1_1030_5fe2(CONCAT22(puVar4, uVar2), CONCAT22(puVar5, uVar3));
    pass1_1030_1358(
        (param_3 + 0xe),
        uVar2,
        puVar4,
        (uVar2 + 0x4) & 0xffff | ((uVar2 + 0x6) & 0xff) << 0x10,
    );
    return;
}


pub unsafe fn pass1_1028_ecac(
    mut param_1: u16,
    param_2: *mut Struct57,
    mut param_3: u32,
    mut param_4: u16,
    param_5: *mut i16,
    mut param_6: u16,
    mut param_7: u32,
) {
    let mut uVar1: u32;
    i16 * *ppiVar2;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut uVar7: u16;
    let mut paVar6: *mut Struct57;

    mem_op_1000_179c(0x1c, param_2);
    uVar3 = param_2 | param_1;
    paVar6 = (param_2 & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
        param_1 = 0;
        puVar4 = null_mut();
    } else {
        struct_1030_299a(param_1, paVar6, CONCAT22(param_2, param_1), param_7);
        puVar4 = paVar6;
    }
    uVar7 = (param_3 >> 0x10);
    uVar1 = (param_3 + 0x52);
    puVar5 = puVar4;
    ppiVar2 = param_5;
    pass1_1030_4628(puVar4, uVar1, (uVar1 >> 0x10), param_5);
    *ppiVar2 = param_5;
    pass1_1030_3006(CONCAT22(puVar4, param_1), CONCAT22(puVar5, ppiVar2));
    pass1_1030_1358(
        (param_3 + 0x12),
        param_1,
        puVar4,
        (param_1 + 0x4) & 0xffff | ((param_1 + 0x6) & 0xff) << 0x10,
    );
    return;
}


pub unsafe fn pass1_1028_ed2c(
    mut param_1: u16,
    param_2: *mut Struct57,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: i16,
    mut param_6: u16,
    mut param_7: u32,
) {
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut puVar11: *mut u16;
    let mut in_stack_0000fef8: u16;
    let mut uVar1: u32;
    let mut paVar8: *mut Struct57;

    mem_op_1000_179c(0x1e, param_2);
    uVar4 = param_2 | param_1;
    uVar9 = param_2 & 0xffff0000;
    paVar8 = (uVar9 | uVar4);
    if (uVar4 == 0) {
        uVar2 = 0;
    } else {
        puVar11 = struct_1030_565a(param_1, paVar8, CONCAT22(param_2, param_1), param_7);
        uVar9 = paVar8 & 0xffff0000 | puVar11 >> 0x10;
        uVar2 = puVar11;
    }
    uVar5 = uVar9;
    uVar10 = (param_3 >> 0x10);
    uVar1 = (param_3 + 0x52);
    uVar3 = uVar2;
    pass1_1030_4782(
        uVar9,
        uVar1,
        (uVar1 >> 0x10),
        0x1,
        0x1,
        param_5,
        in_stack_0000fef8,
    );
    puVar6 = uVar9;
    puVar7 = puVar6;
    pass1_1030_5a80(CONCAT22(uVar5, uVar2), CONCAT22(puVar6, uVar3));
    uVar9 = (uVar2 + 0x4);
    pass1_1030_6222(
        uVar9,
        puVar7,
        _PTR_LOOP_1050_5740,
        0x1,
        CONCAT22(puVar6, uVar3),
        uVar9,
    );
    pass1_1030_1358((param_3 + 0x16), uVar2, uVar5, uVar9 & 0xffffff);
    return;
}


pub unsafe fn pass1_1028_edc4(
    param_1: *mut Struct57,
    mut param_2: u32,
    mut param_3: u16,
    param_4: *mut u16,
    param_5: i32,
    param_6: u8,
) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_1a: [u8; 0x4] = [0; 0x4];
    let mut uStack22: u32;
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut puStack6: *mut u16;
    let mut paVar5: *mut Struct57;

    puStack6 = param_4;
    pass1_1030_64ce(
        param_4,
        param_1,
        _PTR_LOOP_1050_5740,
        param_4,
        param_5,
        CONCAT22(0x1050, local_1a),
    );
    uVar2 = param_4;
    uStack14 = uVar2;
    uStack10 = uVar2;
    mem_op_1000_179c(0x21e, param_1);
    uVar1 = uVar2;
    uVar3 = param_1 | uVar1;
    paVar5 = (param_1 & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
        uVar1 = 0;
        uVar4 = 0;
    } else {
        pass1_1038_3222(
            uVar1,
            paVar5,
            (uVar2 & 0xffff | param_1 << 0x10),
            uStack14,
            param_5,
        );
        uVar4 = paVar5;
    }
    uStack18 = CONCAT22(uVar4, uVar1);
    uStack22 = (uVar1 + 0x4);
    pass1_1030_1358(
        (param_2 + 0x1a),
        uVar1,
        uVar4,
        uStack22 & 0xffff | ((uVar1 + 0x6) & 0xff) << 0x10,
    );
    return;
}


pub unsafe fn pass1_1028_ee54(
    mut param_1: u32,
    mut param_2: u16,
    param_3: *mut u16,
    mut param_4: u32,
) {
    let mut in_DX: u16;
    let mut uVar1: u16;
    let mut paVar2: *mut astruct_99;
    let mut local_16: [u8; 0x4] = [0; 0x4];
    let mut uStack18: u32;
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut uStack10: u32;
    let mut puStack6: *mut u16;

    puStack6 = param_3;
    pass1_1030_64ce(
        param_3,
        in_DX,
        _PTR_LOOP_1050_5740,
        param_3,
        param_4,
        CONCAT22(0x1050, local_16),
    );
    uStack10 = param_3;
    paVar2 = pass1_1000_07fc(_PTR_LOOP_1050_5744);
    uVar1 = (paVar2 >> 0x10);
    uStack14 = paVar2;
    uStack12 = uVar1 | uStack14;
    if (uStack12 == 0) {
        uStack14 = 0;
        uStack12 = 0;
    } else {
        pass1_1030_684c(
            (paVar2 & 0xffff | uVar1 << 0x10),
            puStack6,
            (puStack6 >> 0x10),
            uStack10,
            (uStack10 >> 0x10),
            param_4,
        );
    }
    uStack18 = (uStack14 + 0x4);
    pass1_1030_61fe(
        uStack18,
        uStack12,
        _PTR_LOOP_1050_5740,
        uStack18,
        puStack6,
        param_4,
    );
    pass1_1030_1358(
        (param_1 + 0x1e),
        uStack14,
        uStack12,
        uStack18 & 0xffff | (uStack18 & 0xff) << 0x10,
    );
    return;
}


pub unsafe fn pass1_1028_ef00(
    param_1: *mut Struct57,
    mut param_2: u32,
    mut param_3: u16,
    param_4: *mut astruct_365,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut paVar1: *mut astruct_365;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;

    if (param_4 == &u32_1050_0004) {
        mem_op_1000_179c(0x16, param_1);
        uVar2 = param_1 | param_4;
        if (uVar2 != 0) {
            pass1_1030_b936(uVar2, param_4, param_1, 0x4, _param_5);
            // TODO: goto LAB_1028_ef8b;
        }
    } else if (param_4 == &PTR_LOOP_1050_000c) {
        mem_op_1000_179c(0xe, param_1);
        uVar3 = param_1 | param_4;
        if (uVar3 != 0) {
            puVar4 = pass1_1030_bc24(uVar3, param_4, param_1, 0xc, _param_5);
            uVar2 = (puVar4 >> 0x10);
            param_4 = puVar4;
            // TODO: goto LAB_1028_ef8b;
        }
    } else {
        paVar1 = param_4;
        mem_op_1000_179c(0xe, param_1);
        uVar3 = param_1 | paVar1;
        if (uVar3 != 0) {
            puVar4 = pass1_1028_b22c(uVar3, CONCAT22(param_1, paVar1), param_4, _param_5);
            uVar2 = (puVar4 >> 0x10);
            param_4 = puVar4;
            // TODO: goto LAB_1028_ef8b;
        }
    }
    param_4 = null_mut();
    uVar2 = 0; //
               // LAB_1028_ef8b:
    pass1_1030_1358(
        (param_2 + 0x22),
        param_4,
        uVar2,
        &(param_4).field_0x4 & 0xffff | (&(param_4).field_0x6 & 0xff) << 0x10,
    );
    return;
}
