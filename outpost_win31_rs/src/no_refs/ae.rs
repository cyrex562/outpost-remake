

pub unsafe fn pass1_1028_65e2(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_16: u16;
    let mut paStack20: *mut astruct_99;
    let mut local_10: [u16; 0x2] = [0; 0x2];
    let mut local_c: [u16; 0x3] = [0; 0x3];
    let mut uStack6: u16;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar3 != 0) {
            uStack6 = 0;
            loop {
                if (local_4 <= uStack6) {
                    return;
                }
                paStack20 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                uVar5 = (paStack20 >> 0x10);
                uVar2 = paStack20;
                if ((uVar5 | uVar2) == 0) {
                    paStack20 = null_mut();
                } else {
                    paStack20.field0_0x0 = 0x389a;
                    (uVar2 + 0x2) = 0x1008;
                    (uVar2 + 0x4) = 0;
                    (uVar2 + 0x6) = 0;
                    (uVar2 + 0x8) = 0;
                    (uVar2 + 0xa) = 0;
                    (uVar2 + 0xc) = 0;
                    paStack20.field0_0x0 = 0x56ce;
                    (uVar2 + 0x2) = 0x1018;
                }
                BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_10), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_c), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_16), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, (paStack20 & 0xffff0000 | (paStack20 + 0xa)), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, (paStack20 & 0xffff0000 | (paStack20 + 0xc)), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                (paStack20 + 0x4) = local_10[0];
                uVar4 = switch_1008_72bc(param_4, local_c[0]);
                uVar6 = (paStack20 >> 0x10);
                (paStack20 + 0x6) = uVar4;
                (paStack20 + 0x8) = local_16;
                ppcVar1 = ((param_3 + 0x20) + 0x8);
                (**ppcVar1)();
                uStack6 += 0x1;
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn pass1_1028_6822(mut param_1: u32, param_2: *mut u16) -> u16 {
    let mut iVar1: i16;
    let mut uVar2: u32;

    uVar2 = pass1_1028_67d4(param_1);
    iVar1 = (uVar2 >> 0x10);
    *param_2 = uVar2;
    (param_2 + 0x2) = iVar1;
    if ((iVar1 == 0) && (*param_2 < 0x64)) {
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1028_86f4(
    param_1: *mut astruct_320,
    param_2: *mut u8,
    param_3: *mut astruct_321,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_321;
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
        *puStack10 = 0x87e0;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_87b4(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_8920(mut param_1: u16, mut param_2: u32) {
    let mut ppuVar1: *mut *mut u32 = null_mut();
    let mut ppcVar2: *mut *mut code;
    let mut ppuVar3: *mut *mut u32 = null_mut();
    let mut iVar4: i16;
    let mut BVar5: bool;
    let mut uVar6: u32;
    let mut puVar7: *mut u8;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_register_0000000a: u16;
    let mut paVar10: *mut Struct57;
    let mut iVar11: i16;
    let mut iVar12: *mut astruct_684;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut in_stack_0000fd4e: u16;
    let mut in_stack_0000fe72: u16;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000fe7c: u16;
    let mut uVar14: u8;
    let mut uVar15: u16;
    let mut local_156: [*mut *mut u8; 0x43] = [null_mut(); 0x43];
    let mut local_4a: u32;
    let mut iStack70: i16;
    let mut uStack68: u32;
    let mut uStack56: u32;
    let mut puStack52: *mut u32;
    let mut uStack48: u16;
    let mut puStack46: *mut u32;
    let mut uStack42: u32;
    let mut local_26: [u8; 0x4] = [0; 0x4];
    let mut uStack34: u32;
    let mut uStack30: u32;
    let mut uStack26: u32;
    let mut uStack22: u32;
    let mut puStack18: *mut u32;
    let mut uStack14: u16;
    let mut local_c: [u8; 0x2] = [0; 0x2];
    let mut local_a: [u8; 0x2] = [0; 0x2];
    let mut local_8: [u8; 0x2] = [0; 0x2];
    let mut uStack6: u32;

    uVar12 = (param_2 >> 0x10);
    iVar11 = param_2;
    ppuVar1 = (iVar11 + 0x114);
    ppuVar3 = ppuVar1;
    pass1_1030_64ce(
        ppuVar1,
        param_1,
        _PTR_LOOP_1050_5740,
        (param_2 & 0xffff0000 | ZEXT24(ppuVar1)),
        (iVar11 + 0x108),
        CONCAT22(0x1050, local_26),
    );
    uStack6 = *ppuVar3;
    pass1_1008_3eb4(
        (param_2 & 0xffff0000 | ZEXT24(ppuVar1)),
        CONCAT22(0x1050, local_c),
        CONCAT13(0x10, CONCAT12(0x50, local_a)),
        CONCAT22(0x1050, local_8),
    );
    paVar10 = CONCAT22(in_register_0000000a, uStack6);
    puStack46 = uStack6;
    uStack56 = uStack6;
    uStack56._3_1_ = (uStack6 >> 0x18);
    uStack14 = (uStack56._3_1_ != '\0');
    if (uStack14 == 0) {
        uVar6 = (iVar11 + 0x114);
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x500);
        puStack18 = (uVar6 & 0xffff | paVar10 << 0x10);
        uVar13 = 0x1030;
        pass1_1030_61fe(
            uVar6,
            paVar10,
            _PTR_LOOP_1050_5740,
            uVar6 & 0xffff | paVar10 << 0x10,
            param_2 & 0xff000000 | CONCAT12((param_2 >> 0x10), iVar11 + 0x114),
            (iVar11 + 0x108),
        );
        uStack56 = null_mut();
        if (((iVar11 + 0x11a) == 0xa) || ((iVar11 + 0x11a) == 0x37)) {
            if ((iVar11 + 0x11a) == 0x37) {
                uStack56 = (iVar11 + 0x10c);
            }
            iVar4 = iVar11 + 0x114;
            pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x400);
            (iVar11 + 0x10c) = iVar4;
            (iVar11 + 0x10e) = paVar10;
            puStack46 = mixed_1010_20ba(
                paVar10,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x2f),
                in_stack_0000fd4e,
                in_stack_0000fe72,
                in_stack_0000fe78,
                in_stack_0000fe7c,
            );
            paVar10 = (paVar10 & 0xffff0000 | puStack46 >> 0x10);
            uVar6 = puStack46 & 0xffff;
            puVar7 = (puStack46 >> 0x10);
            uVar13 = 0x1018;
            pass1_1018_0196(
                uVar6,
                puVar7,
                uVar6 | ZEXT24(puVar7) << 0x10,
                (iVar11 + 0x10c),
                (iVar11 + 0x108),
            );
            iVar4 = uVar6;
            if ((iVar11 + 0x110) != 0) {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar11 + 0x10c));
                uStack42 = CONCAT22(paVar10, iVar4);
                uVar6 = (iVar11 + 0x110);
                (iVar4 + 0x200) = uVar6;
                uStack68 = uVar6;
            }
        }
        uStack6 = uVar6;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar11 + 0x10c));
        uVar8 = paVar10;
        puStack52 = CONCAT22(uVar8, uStack6);
        paVar10 = (paVar10 & 0xffff0000 | (uVar8 | uStack6));
        if ((uVar8 | uStack6) != 0) {
            ppcVar2 = (*puStack52 + 0x8);
            (**ppcVar2)(
                uVar13,
                uStack6,
                uVar8,
                0x0,
                puStack18,
                (puStack18 >> 0x10),
                0x0,
            );
        }
    } else {
        puStack18 = uStack6;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puStack18);
    uStack22 = CONCAT22(paVar10, uStack6);
    pass1_1030_73ee(
        paVar10,
        CONCAT13((paVar10 >> 0x8), CONCAT12(paVar10, uStack6)),
        (iVar11 + 0x10c),
    );
    BVar5 = pass1_1008_c6ae(_u16_1050_06e0, (iVar11 + 0x11a), 0x31);
    if ((BVar5 == 0) && ((iVar11 + 0x11c) == 0)) {
        paVar10 = (paVar10 & 0xffff0000);
        local_4a = (uStack22 + 0xc);
        iStack70 = (uStack22 + 0x10);
        uStack68 = uStack68 & 0xffff0000 | ZEXT24(&local_4a);
        if (iStack70 < 1) {
            uStack48 = 0x5;
        } else {
            uStack48 = 0x6;
        }
        (uStack22 + 0x14) = uStack48;
    }
    uStack26 = (uStack22 + 0x16);
    uVar8 = (uStack22 + 0x18);
    paVar10 = (paVar10 & 0xffff0000 | uVar8);
    if ((uVar8 | uStack26) != 0) {
        struct_1030_e4fa(
            CONCAT13(0x10, CONCAT12(0x50, local_156)),
            uStack26 & 0xffff | uVar8 << 0x10,
        );
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_156));
        local_156[0] = &PTR_pass1_1008_377e_1008_389a;
    }
    uStack30 = pass1_1028_e2e0(paVar10, _PTR_LOOP_1050_65e2, 0x7);
    uVar8 = uStack30;
    uVar9 = (uStack30 >> 0x10) | uVar8;
    if (uVar9 == 0) {
        return;
    }
    pass1_1030_7e5a(uVar9, uStack22, uStack30);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack30);
    uStack34 = CONCAT22(uVar9, uVar8);
    uVar13 = SUB42(puStack18, 0x0);
    uVar15 = (puStack18 >> 0x10);
    uVar14 = uVar9;
    iVar12 = *uStack34;
    ppcVar2 = &iVar12.field4_0x4;
    (**ppcVar2)();
    ppcVar2 = &iVar12.field28_0x20;
    (**ppcVar2)(0x1030, uStack34, uVar8, uVar14, uVar13, uVar15);
    ppcVar2 = &iVar12.field22_0x18;
    (**ppcVar2)(0x1030, uStack34, (uStack34 >> 0x10), 1);
    if ((iVar11 + 0x11a) == 0x37) {
        (uStack34 + 0x20) = (iVar11 + 0x10c);
    }
    (iVar11 + 0x120) = uStack34;
    return;
}


pub unsafe fn pass1_1028_8c46(
    param_1: *mut astruct_322,
    param_2: *mut u8,
    param_3: *mut astruct_323,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_323;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x124, paVar5);
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
        param_1.field261_0x118 = iVar5.field262_0x118;
        param_1.field262_0x11a = iVar5.field263_0x11a;
        param_1.field263_0x11c = iVar5.field264_0x11c;
        param_1.field264_0x11e = iVar5.field265_0x11e;
        param_1.field265_0x120 = iVar5.field266_0x120;
        *puStack10 = 0x8d8e;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}


pub unsafe fn pass1_1028_8d62(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_8e1e(mut param_1: i16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_3 >> 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x10c));
    pass1_1030_355c((param_1 + 0x1f6), (param_3 + 0x114));
    return;
}

pub unsafe fn pass1_1028_8e5c(mut param_1: u32, mut param_2: i16, param_3: *mut u8) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar2 + 0x108));
    uVar1 = (param_2 + 0x1f6);
    pass1_1030_35a4(param_3, uVar1, (iVar2 + 0x110));
    (iVar2 + 0x114) = uVar1;
    (iVar2 + 0x116) = param_3;
    return;
}


pub unsafe fn pass1_1028_8ea6(
    param_1: *mut astruct_324,
    param_2: *mut u8,
    param_3: *mut astruct_325,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_325;
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
        *puStack10 = 0x8fb0;
        param_1.field2_0x2 = 0x1028;
    }
    iVar5.field261_0x114 = 0;
    return;
}

pub unsafe fn pass1_1028_8f8a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_8dec(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_8fea(
    param_1: *mut astruct_326,
    param_2: *mut u8,
    param_3: *mut astruct_327,
) {
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_327;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;
    let mut puVar1: *mut u32;

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
        *puStack10 = 0x90d6;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_90aa(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_9114(param_1: *mut StructD, mut param_2: u32, mut param_3: u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut paVar4: *mut astruct_67;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut iVar9: i16;
    let mut uVar10: u16;
    let mut uStack10: u16;

    paVar4 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x37),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    uVar2 = (param_1 >> 0x10);
    uVar3 = param_2;
    iVar1 = (uVar3 + 0x108);
    if (iVar1 - 0x1 < 0x8) {
        uStack10 = *_PTR_LOOP_1050_65e2;
        iVar8 = (*_PTR_LOOP_1050_65e2 >> 0x10);
        match iVar1 {
            0x1 => {
                iVar1 = 0x16;
            }
            //   break;
            0x2 => {
                iVar1 = 0x17;
            }
            //   break;
            0x3 => {
                iVar1 = 0x18;
            }
            //   break;
            0x4 => {
                iVar1 = 0x1b;
            }
            //   break;
            0x5 => {
                iVar1 = 0x1f;
            }
            //   break;
            0x6 => {
                iVar1 = 0x24;
            }
            //   break;
            0x7 => {
                pass1_1008_612e(uVar3, 0x0, 0x14);
                iVar1 = (uVar3 >> 0xf) + (0xff91 < uVar3);
                uVar6 = uStack10 + uVar3 + 0x6e;
                uVar7 = iVar8 + iVar1 + CARRY2(uStack10, uVar3 + 0x6e);
                iVar8 = 0x7;
                puVar5 = mixed_1010_20ba(
                    CONCAT22(uVar2, iVar1),
                    _u16_1050_0ed0,
                    CONCAT22(uVar6, 0x2f),
                    in_stack_0000fe8c,
                    in_stack_0000ffb0,
                    in_stack_0000ffb6,
                    in_stack_0000ffba,
                );
                uVar2 = (puVar5 >> 0x10);
                uVar3 = puVar5;
                pass1_1010_ebf8(puVar5, uVar6, uVar7, iVar8);
                pass1_1008_612e(uVar3, 0x1, 0x64);
                if (0x32 < uVar3) {
                    return;
                }
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
                pass1_1038_4900(CONCAT22(uVar2, uVar3));
                iVar1 = 0x2c;
            }
            //   break;
            0x8 => {
                pass1_1008_612e(uVar3, 0x0, 0x14);
                iVar1 = (uVar3 >> 0xf) + (0xff9b < uVar3);
                uVar6 = uStack10 + uVar3 + 0x64;
                uVar7 = iVar8 + iVar1 + CARRY2(uStack10, uVar3 + 0x64);
                iVar9 = 0x8;
                puVar5 = mixed_1010_20ba(
                    CONCAT22(uVar2, iVar1),
                    _u16_1050_0ed0,
                    CONCAT22(uVar6, 0x2f),
                    in_stack_0000fe8c,
                    in_stack_0000ffb0,
                    in_stack_0000ffb6,
                    in_stack_0000ffba,
                );
                iVar1 = (puVar5 >> 0x10);
                iVar8 = puVar5;
                pass1_1010_ebf8(puVar5, uVar6, uVar7, iVar9);
                if (0x19 < uVar3) {
                    return;
                }
                uVar3 = 0x1;
                uVar10 = 0x2;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
                pass1_1038_43cc(iVar8, iVar1, iVar8, iVar1, uVar3, uVar10);
                iVar1 = 0x2d;
            }
        };
        post_win_msg_1008_a0e4(paVar4, 0x0, 0x0, 0x1, 0x0, iVar1);
    }
    return;
}


pub unsafe fn pass1_1028_9264(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar7: i16;
    let mut puVar8: *mut u32;
    let mut uVar9: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x10a, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        (param_1 + 0x2) = 0x1008;
        uVar9 = (param_3 >> 0x10);
        iVar7 = param_3;
        (param_1 + 0x4) = (iVar7 + 0x4);
        puVar3 = (iVar7 + 0x8);
        puVar8 = (param_1 + 0x8);
        for iVar4 in 0x40..0 {
            puVar2 = puVar8;
            puVar8 = puVar8 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        (param_1 + 0x108) = (iVar7 + 0x108);
        *puStack10 = 0x932c;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_9300(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1028_94e4(
    param_1: *mut astruct_328,
    param_2: *mut u8,
    param_3: *mut astruct_329,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_329;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x124, paVar5);
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
        param_1.field261_0x118 = iVar5.field262_0x118;
        param_1.field262_0x11a = iVar5.field263_0x11a;
        param_1.field263_0x11c = iVar5.field264_0x11c;
        param_1.field264_0x11e = iVar5.field265_0x11e;
        param_1.field265_0x122 = iVar5.field266_0x122;
        *puStack10 = 0x9934;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_9600(param_1: *mut u8, mut param_2: u32) {
    let mut puVar1: *mut u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    puVar1 = pass1_1020_a43e(param_1, CONCAT22(0x1050, local_6));
    pass1_1020_a80e(
        local_6,
        (puVar1 >> 0x10),
        local_6,
        &DAT_1050_1050,
        (param_2 + 0x11a),
    );
    return;
}

pub unsafe fn pass1_1028_9624(mut param_1: u16, param_2: *mut u8, param_3: *mut astruct_688) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut BVar5: bool;
    let mut uVar7: u32;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut iVar9: *mut astruct_688;
    let mut unaff_SI: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut in_stack_0000fd54: u16;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000fe82: u16;
    let mut puVar11: *mut u8;
    let mut puVar12: *mut u8;
    let mut uStack332: u16;
    let mut uStack330: u16;
    let mut uStack64: u16;
    let mut uStack62: u32;
    let mut iStack58: i16;
    let mut uStack56: u32;
    let mut puStack46: *mut u32;
    let mut uStack42: u32;
    let mut local_26: [u8; 0x4] = [0; 0x4];
    let mut uStack34: u16;
    let mut puStack32: *mut u8;
    let mut uStack30: u32;
    let mut uStack26: u32;
    let mut puStack22: *mut u32;
    let mut local_12: [u8; 0x2] = [0; 0x2];
    let mut local_10: [u8; 0x2] = [0; 0x2];
    let mut local_e: [u8; 0x2] = [0; 0x2];
    let mut uStack12: u16;
    let mut uStack10: u32;
    let mut puStack6: *mut u32;
    let mut puVar6: *mut u32;

    paVar8 = CONCAT22(in_register_0000000a, param_2);
    uVar9 = (param_3 >> 0x10);
    iVar9 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, iVar9.field265_0x10c);
    iVar9.field266_0x110 = param_1;
    (&iVar9.field266_0x110 + 0x2) = paVar8;
    puStack6 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2f),
        in_stack_0000fd54,
        in_stack_0000fe78,
        in_stack_0000fe7e,
        in_stack_0000fe82,
    );
    uStack10 = (puStack6 >> 0x10);
    puVar2 = &iVar9.field267_0x114;
    pass1_1030_64ce(
        puVar2,
        uStack10,
        _PTR_LOOP_1050_5740,
        (param_3 & 0xffff0000 | ZEXT24(puVar2)),
        iVar9.field264_0x108,
        CONCAT22(0x1050, local_26),
    );
    uStack56 = *puVar2;
    uStack56._3_1_ = (uStack56 >> 0x18);
    uStack12 = (uStack56._3_1_ != '\0');
    uVar10 = 0x1008;
    puStack46 = uStack56;
    uStack10 = uStack56;
    pass1_1008_3eb4(
        (param_3 & 0xffff0000 | ZEXT24(&iVar9.field267_0x114)),
        CONCAT22(0x1050, local_12),
        CONCAT22(0x1050, local_10),
        CONCAT22(0x1050, local_e),
    );
    if (uStack12 == 0) {
        puVar2 = &iVar9.field267_0x114;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x500);
        puStack22 = CONCAT22(uStack10, puVar2);
        uVar10 = 0x1030;
        pass1_1030_61fe(
            puVar2,
            uStack10,
            _PTR_LOOP_1050_5740,
            CONCAT22(uStack10, puVar2),
            param_3 & 0xffff0000 | ZEXT24(&iVar9.field267_0x114),
            iVar9.field264_0x108,
        );
        if ((iVar9.field270_0x11a == 0xa) || (iVar9.field270_0x11a == 0x37)) {
            if (iVar9.field270_0x11a == 0x37) {
                uStack56 = iVar9.field273_0x11e;
                uStack10 = (&iVar9.field273_0x11e + 2);
                uStack42 = iVar9.field265_0x10c;
                (uStack56 + 0x20) = uStack42;
            }
            puVar2 = &iVar9.field267_0x114;
            pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x400);
            iVar9.field265_0x10c = puVar2;
            (&iVar9.field265_0x10c + 0x2) = uStack10;
            uVar10 = 0x1018;
            pass1_1018_0196(
                puVar2,
                uStack10,
                puStack6,
                CONCAT22(uStack10, &iVar9.field265_0x10c),
                iVar9.field264_0x108,
            );
            if (iVar9.field270_0x11a == 0xa) {
                uVar10 = 0x1010;
                pass1_1010_ed22(puStack6, iVar9.field265_0x10c);
            }
        }
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, iVar9.field265_0x10c);
        iVar9.field266_0x110 = puVar2;
        (&iVar9.field266_0x110 + 0x2) = uStack10;
        uVar4 = uStack10 | &iVar9.field266_0x110;
        puVar6 = uVar4;
        //    if (uVar4 == 0) goto LAB_1028_9807;
        uVar3 = SUB42(puStack22, 0x0);
        puVar12 = (puStack22 >> 0x10);
        puVar11 = uStack10;
    } else {
        puStack22 = uStack10;
        puVar6 = uStack10;
        //    if (iVar9.field270_0x11a != 0x75) goto LAB_1028_9807;
        uVar3 = SUB42(uStack10, 0x0);
        puVar12 = uStack10;
        puVar11 = (&iVar9.field266_0x110 + 2);
    }
    ppcVar1 = (*iVar9.field266_0x110 + 0x8);
    (**ppcVar1)(
        uVar10,
        &iVar9.field266_0x110,
        puVar11,
        0x0,
        uVar3,
        puVar12,
        0x0,
    ); //
       // LAB_1028_9807:
    uVar10 = SUB42(puVar6, 0x0);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puStack22);
    uStack26 = CONCAT22(uStack10, uVar10);
    pass1_1030_73ee(uStack10, CONCAT22(uStack10, uVar10), iVar9.field265_0x10c);
    BVar5 = pass1_1008_c6ae(_u16_1050_06e0, iVar9.field270_0x11a, 0x31);
    if ((BVar5 == 0) && (iVar9.field274_0x122 == 0)) {
        uStack62 = (uStack26 + 0xc);
        iStack58 = (uStack26 + 0x10);
        uStack56 = (uStack56 & 0xffff0000 | ZEXT24(&uStack62));
        if (iStack58 < 1) {
            uStack64 = 0x5;
        } else {
            uStack64 = 0x6;
        }
        (uStack26 + 0x14) = uStack64;
        uStack10 = uStack26;
    }
    uVar7 = (uStack26 + 0x16);
    uStack30 = uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7);
    uStack34 = uVar7;
    puStack32 = uStack10;
    if (uStack30 != 0) {
        struct_1030_e4fa(CONCAT22(0x1050, &uStack332), uStack30);
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &uStack332));
        uStack332 = 0x389a;
        uStack330 = 0x1008;
    }
    ppcVar1 = (*iVar9.field273_0x11e + 0x4);
    (**ppcVar1)();
    puVar6 = iVar9.field273_0x11e;
    pass1_1030_7e5a(uStack10, uStack26, (puVar6 + 0x4));
    return;
}
