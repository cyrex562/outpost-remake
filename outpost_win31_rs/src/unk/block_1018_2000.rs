pub unsafe fn pass1_1018_2076(param_1: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = 0x21e8;
    (param_1 + 0x2) = 0x1018;
    pass1_1018_209c(param_1 & 0xffff | uVar1 << 0x10);
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1018_209c(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        uVar5 = (param_1 >> 0x10);
        iVar4 = param_1 + 0x12;
        puVar1 = (iVar4 + iStack4 * 0x4);
        uVar2 = (iVar4 + iStack4 * 0x4 + 2);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        (param_1 + iStack4 * 0x4 + 0x12) = 0;
        iStack4 += 0x1;
        if iStack4 >= 0x1fd {
            break;
        }
    }
    return;
}

pub unsafe fn pass1_1018_20ee(mut param_1: u32, param_2: *mut i16) {
    let mut BVar1: bool;
    let mut in_DX: u16;
    let mut uVar2: u16;

    BVar1 = pass1_1008_aed8(param_2);
    if (BVar1 == 0) {
        return;
    }
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + *param_2 * 0x4 + 0x12) == 0) {
        pass1_1018_216e(in_DX, param_1 & 0xffff | uVar2 << 0x10, param_2);
    }
    pass1_1008_ae26(param_2);
    return;
}

pub unsafe fn pass1_1018_214e(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut u16,
    mut param_4: i16,
) {
    pass1_1008_3e76(
        param_3,
        0x0,
        (param_4 * 0x4 + 0x3e90),
        (param_4 * 0x4 + 0x3e8e),
    );
    return;
}


pub unsafe fn pass1_1018_216e(mut param_1: u16, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uStack8: u16;

    uStack8 = pass1_1008_adf2(param_3);
    uVar1 = pass1_1008_ae0c(param_3);
    while uStack8 <= uVar1 {
        uVar2 = uVar1;
        pass1_1010_8018(param_1, _u16_1050_14cc, uStack8);
        uVar3 = (param_2 >> 0x10);
        (param_2 + uStack8 * 0x4 + 0x12) = uVar2;
        (param_2 + uStack8 * 0x4 + 0x14) = param_1;
        uStack8 += 1;
    }
    return;
}


pub unsafe fn pass1_1018_21f8() -> u16 {
    let mut puVar1: *mut u16;

    pass1_1008_3e54(&u16_1048_4210, 0x0, 0x195, 1);
    pass1_1008_3e54(&u16_1050_65ca, 0x0, 0xe0, 0x1b1);
    pass1_1008_3e54(&u16_1050_65d0, 0x0, 0x17a, 0x72);
    pass1_1008_3e54(&u16_1050_65d6, 0x0, 0xde, 0x93);
    pass1_1008_3e54(&u16_1050_65dc, 0x0, 0x177, 0x1da);
    pass1_1008_3e54(&u16_1048_4216, 0x0, 0x195, 0x21c);
    pass1_1008_3e54(&u32_1048_421c, 0x0, 0x1b6, 0x22c);
    pass1_1008_3e54((&u32_1048_4220 + 0x2), 0x0, 0x109, 0x5);
    puVar1 = pass1_1008_3e54(&u32_1048_4228, 0x0, 0xfd, 0x1fd);
    return puVar1;
}


pub unsafe fn struct_1018_229c(param_1: *mut u8, param_2: *mut Struct19, mut param_3: u16) {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut paVar3: *mut Struct57;
    let mut in_register_0000000a: u16;
    let mut uVar4: u32;
    let mut paVar5: *mut Struct57;
    let mut unaff_CS: u16;
    let mut iVar6: i16;

    uVar4 = CONCAT22(in_register_0000000a, param_1);
    struct_op_1018_4cda(param_2, param_3);
    (param_2 + 0x1c) = 0x389a;
    (param_2 + 0x1e) = 0x1008;
    (param_2 + 0x1c) = 0x3aa8;
    (param_2 + 0x1e) = 0x1008;
    uVar1 = 0;
    (param_2 + 0x20) = 0;
    (param_2 + 0x24) = 0;
    (param_2 + 0x26) = 0;
    (param_2 + 0x2a) = 0;
    (param_2 + 0x3e) = 0;
    (param_2 + 0x40) = 0;
    (param_2 + 0x42) = 0;
    (param_2 + 0x44) = 0;
    (param_2 + 0x6e) = 0;
    param_2.offset_0x0 = 0x2ada;
    (param_2 + 0x2) = 0x1018;
    (param_2 + 0x1c) = s_fem132_wav_1050_2aec + 0x6;
    (param_2 + 0x1e) = 0x1018;
    _PTR_LOOP_1050_4230 = param_2;
    pass1_1018_4dce(uVar4, param_2, 0x105);
    uVar1 = FUN_1010_830a(uVar1, uVar4, unaff_CS, _u16_1050_14cc, 0x1a8);
    (param_2 + 0x2a) = uVar1;
    (param_2 + 0x2c) = uVar4;
    pass1_1000_4906(
        (param_2 & 0xffff0000 | ZEXT24((param_2 + 0x2e))),
        NULL,
        0x10,
    );
    puVar2 = pass1_1000_4906((param_2 & 0xffff0000 | (param_2 + 0x46)), NULL, 0x28);
    uVar1 = FUN_1010_830a(puVar2, uVar4, 0x1000, _u16_1050_14cc, 0x6c);
    (param_2 + 0x2e) = uVar1;
    (param_2 + 0x30) = uVar4;
    uVar1 = FUN_1010_830a(uVar1, uVar4, 0x1010, _u16_1050_14cc, 0x68);
    (param_2 + 0x32) = uVar1;
    (param_2 + 0x34) = uVar4;
    uVar1 = FUN_1010_830a(uVar1, uVar4, 0x1010, _u16_1050_14cc, 0x66);
    (param_2 + 0x36) = uVar1;
    (param_2 + 0x38) = uVar4;
    uVar1 = FUN_1010_830a(uVar1, uVar4, 0x1010, _u16_1050_14cc, 0x6a);
    (param_2 + 0x3a) = uVar1;
    (param_2 + 0x3c) = uVar4;
    uVar1 = FUN_1010_830a(uVar1, uVar4, 0x1010, _u16_1050_14cc, 0x1cd);
    (param_2 + 0x6e) = uVar1;
    (param_2 + 0x70) = uVar4;
    iVar6 = 0;
    loop {
        uVar1 = FUN_1010_830a(iVar6 + 0x8f, uVar4, 0x1010, _u16_1050_14cc, iVar6 + 0x8f);
        (param_2 + iVar6 * 0x4 + 0x46) = uVar1;
        (param_2 + iVar6 * 0x4 + 0x48) = uVar4;
        iVar6 += 0x1;
        if iVar6 >= 0xa {
            break;
        }
    }
    if (param_2.is_null()) {
        paVar3 = null_mut();
        paVar5 = (uVar4 & 0xffff0000);
    } else {
        paVar5 = (uVar4 & 0xffff0000 | param_2);
        paVar3 = (param_2 + 0x1c);
    }
    pass1_1008_9262(
        paVar3,
        paVar5,
        _PTR_LOOP_1050_0388,
        (_PTR_LOOP_1050_0388 >> 0x10),
        0x73,
        CONCAT22(paVar5, paVar3),
    );
    return;
}


pub unsafe fn pass1_1018_2440(param_1: *mut StructD) {
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut puVar5: *mut u8;
    let mut puVar4: *mut u8;
    let mut uVar6: u16;
    let mut uVar5: *mut StructD;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let mut puStack6: *mut u16;
    let mut uVar2: u16;
    let mut puVar1: *mut u32;

    uVar7 = (param_1 >> 0x10);
    uVar5 = param_1;
    param_1.address_offset_field_0x0 = 0x2ada;
    uVar5.address_offset_field_0x2 = 0x1018;
    uVar5.field_0x1c = s_fem132_wav_1050_2aec + 0x6;
    uVar5.field_0x1e = 0x1018;
    if (_PTR_LOOP_1050_0388 != 0) {
        if (param_1.is_null()) {
            puVar5 = null_mut();
            uVar6 = 0;
        } else {
            puVar5 = &uVar5.field_0x1c;
            uVar6 = uVar7;
        }
        unaff_CS = 0x1008;
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x73, CONCAT22(uVar6, puVar5));
    }
    puVar1 = &uVar5.field_0x2a;
    uVar2 = &uVar5.field29_0x2c;
    if ((uVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)(unaff_CS, puVar1, uVar2, 1);
    }
    puVar2 = &uVar5.field_0x6e;
    uVar3 = &uVar5.field_0x70;
    if ((uVar3 | puVar2) != 0) {
        ppcVar4 = *puVar2;
        (**ppcVar4)(unaff_CS, puVar2, uVar3, 1);
    }
    if (param_1.is_null()) {
        puVar4 = null_mut();
        uVar7 = 0;
    } else {
        puVar4 = &uVar5.field_0x1c;
    }
    puStack6 = CONCAT22(uVar7, puVar4);
    *puStack6 = 0x389a;
    (puVar4 + 0x2) = 0x1008;
    clenaup_win_ui_1018_4d22(param_1);
    return;
}


pub unsafe fn pass1_1018_2504(mut param_1: u16, mut param_2: u16) {
    let mut uVar1: u16;

    pass1_1030_8344(_u16_1050_5748, 0x4000001);
    if ((param_2 | param_1) != 0) {
        uVar1 = pass1_1028_d69e(**_u16_1050_5748);
        if (uVar1 == 0) {
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1018_2548(mut param_1: u16, mut param_2: u16, param_3: *mut u16) {
    pass1_1008_3f62(param_3, &u32_1048_4228);
    return;
}

pub unsafe fn pass1_1018_255e(mut param_1: u32) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x26) != 0) {
        uVar1 = (param_1 + 0x26);
        return (uVar1 + 0xa);
    }
    return 0x0;
}

pub unsafe fn pass1_1018_2580(
    param_1: u8,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u16,
) -> *mut u8 {
    let mut puVar1: *mut u8;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut local_8: [u8; 0x6] = [0; 0x6];

    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    if ((iVar2 + 0x20) == 0) {
        return 0x6ad;
    }
    pass1_1008_3e38(CONCAT22(0x1050, local_8));
    pass1_1018_161c(
        (iVar2 + 0x20),
        CONCAT22(0x1050, local_8),
        param_4,
        (param_4 >> 0x10),
    );
    puVar1 = local_8;
    pass1_1018_17ce(
        (iVar2 + 0x20),
        CONCAT22(puVar1, param_3),
        CONCAT22(param_5, 0x1050),
    );
    return puVar1;
}



pub unsafe fn pass1_1018_25d2(mut param_1: u32, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut uVar3: u16;
    let mut paVar2: *mut Struct57;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut puVar7: *mut u16;
    let mut local_8: [u8; 0x6] = [0; 0x6];

    uVar3 = (in_EDX >> 0x10);
    uVar4 = (param_1 >> 0x10);
    if ((param_1 + 0x20) == 0) {
        return 0x0;
    }
    puVar5 = pass1_1008_3e38(CONCAT22(0x1050, local_8));
    paVar2 = CONCAT22(uVar3, (puVar5 >> 0x10));
    pass1_1018_161c(
        (param_1 + 0x20),
        CONCAT22(0x1050, local_8),
        param_3,
        (param_3 >> 0x10),
    );
    puVar7 = CONCAT22(0x1050, local_8);
    puVar6 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x32),
        in_stack_0000fe96,
        in_stack_0000ffba,
        in_stack_0000ffc0,
        in_stack_0000ffc4,
    );
    uVar1 = puVar6;
    pass1_1010_71d6(
        uVar1,
        (puVar6 >> 0x10),
        puVar6,
        param_2,
        puVar7,
        0x1050,
    );
    return uVar1;
}

pub unsafe fn pass1_1018_262e(mut param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x44) = 0x1;
    (param_1 + 0x3e) = 0;
    return;
}

pub unsafe fn pass1_1018_2646(mut param_1: u16, mut param_2: u16, param_3: *mut u16) {
    pass1_1008_3f62(param_3, (&u32_1048_4220 + 0x2));
    return;
}



pub unsafe fn pass1_1018_265c() -> u32 {
    let mut uVar1: u32;

    uVar1 = pass1_1030_8326();
    return uVar1;
}

pub unsafe fn pass1_1018_266a(mut param_1: u32) -> u16 {
    return (param_1 + 0x44);
}

pub unsafe fn pass1_1018_2678(mut param_1: u16, mut param_2: u16, param_3: *mut u16) {
    pass1_1008_3f62(param_3, &u16_1048_4216);
    return;
}

pub unsafe fn pass1_1018_268e(param_1: *mut astruct_287) -> u32 {
    let mut iVar1: *mut astruct_287;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar1 = param_1;
    if (iVar1.field65_0x42 != 0) {
        iVar1.field_0x40 = 0;
        iVar1.field66_0x44 = 0x1;
    }
    iVar2 = iVar1.field62_0x3e * 0x4;
    return CONCAT22(
        (&iVar1[0x1].field_0x2 + iVar2),
        (&iVar1[0x1].field_0x0 + iVar2),
    );
}

pub unsafe fn pass1_1018_26c2(mut param_1: u16, mut param_2: u16, param_3: *mut u16) {
    pass1_1008_3f62(param_3, &u32_1048_421c);
    return;
}

pub unsafe fn pass1_1018_26d8(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    param_4: *mut u16,
) {
    pass1_1008_3f62(param_4, CONCAT22(0x1050, &u16_1050_65ca + param_3 * 0x6));
    return;
}

pub unsafe fn pass1_1018_26f8(mut param_1: u16, mut param_2: u16, param_3: *mut u16) {
    pass1_1008_3f62(param_3, &u16_1048_4210);
    return;
}


pub unsafe fn pass1_1018_270e(
    param_1: *mut u8,
    param_2: *mut Struct27,
    mut param_3: i16,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut puVar3: *mut u8;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut Struct27;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000fff4: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    iVar5 = param_2;
    uVar6 = (param_2 >> 0x10);
    if (param_3 == 0) {
        if ((&iVar5.field_0x20 == 0) || (uVar2 = &iVar5.field_0x20, (uVar2 + 0x8) != param_4)) {
            puVar7 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000fff4, param_4),
                in_stack_0000fe9c,
                in_stack_0000ffc0,
                in_stack_0000ffc6,
                in_stack_0000ffca,
            );
            if (&iVar5.field_0x20 != 0) {
                if (param_2.is_null()) {
                    puVar3 = null_mut();
                    uVar4 = 0;
                } else {
                    puVar3 = &iVar5.field_0x1c;
                    uVar4 = uVar6;
                }
                pass1_1010_1ea6(&iVar5.field_0x20, CONCAT22(uVar4, puVar3));
            }
            iVar5.field_0x20 = puVar7;
            if (param_2.is_null()) {
                param_4 = 0;
                uVar4 = 0;
            } else {
                param_4 = &iVar5.field_0x1c;
                uVar4 = uVar6;
            }
            paVar5 = uVar4;
            uVar2 = &iVar5.field_0x20;
            ppcVar1 = (*&iVar5.field_0x20 + 0x4);
            (**ppcVar1)(0x1010, uVar2, (uVar2 >> 0x10), 0x0, param_4, uVar4);
        }
        uVar4 = paVar5;
        pass1_1018_2862(param_2);
        if ((uVar4 | param_4) != 0) {
            (&iVar5.field30_0x22 + 0x2) = 0x1;
        }
        pass1_1010_1f62(param_2, 0x7);
    } else if ((&iVar5.field30_0x22 | &iVar5.field_0x20) != 0) {
        if (param_2.is_null()) {
            puVar3 = null_mut();
            uVar4 = 0;
        } else {
            puVar3 = &iVar5.field_0x1c;
            uVar4 = uVar6;
        }
        pass1_1010_1ea6(&iVar5.field_0x20, CONCAT22(uVar4, puVar3));
        iVar5.field_0x20 = 0;
        return;
    }
    return;
}

pub unsafe fn pass1_1018_280c(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x24) == 0) {
        return;
    }
    (iVar2 + 0x24) = 0;
    if ((iVar2 + 0x20) == 0) {
        (iVar2 + 0x26) = 0;
    } else {
        uVar1 = (iVar2 + 0x20);
        (iVar2 + 0x26) = (uVar1 + 0x4c);
    }
    return;
}

pub unsafe fn pass1_1018_2862(param_1: *mut astruct_654) {
    let mut lVar1: i32;
    let mut iVar2: *mut astruct_654;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field32_0x20 == 0) {
        iVar2.field35_0x26 = 0;
    } else {
        lVar1 = iVar2.field32_0x20;
        iVar2.field35_0x26 = (lVar1 + 0x4c);
    }
    return;
}

pub unsafe fn pass1_1018_2922(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x40) != 0) && (
        piVar1 = (iVar2 + 0x3e),
        *piVar1 = *piVar1 + 0x1,
        0x9 < (iVar2 + 0x3e),
    )) {
        (iVar2 + 0x3e) = 0;
        (iVar2 + 0x42) = 0x1;
    }
    return;
}




pub unsafe fn struct_1018_2b10(param_1: *mut Struct19, mut param_2: u16) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u16;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut uVar7: u16;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u32;
    let mut puVar8: *mut u16;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut uVar12: u16;
    let mut in_stack_0000fff0: u32;
    let mut uVar14: u16;
    let mut ppuVar13: *mut *mut u8;
    let mut uVar11: *mut Struct19;
    let mut uVar9: *mut Struct19;

    uVar7 = (in_EDX >> 0x10);
    uVar14 = (in_stack_0000fff0 >> 0x10);
    uVar9 = param_1;
    uVar12 = (param_1 >> 0x10);
    puVar8 = get_sys_metrics_1018_4b1e(param_1, 0x1, param_2);
    paVar5 = CONCAT22(uVar7, (puVar8 >> 0x10));
    uVar9.field17_0x20 = 0x389a;
    uVar9.field18_0x22 = 0x1008;
    uVar9.field17_0x20 = 0x3aa8;
    uVar9.field18_0x22 = 0x1008;
    uVar9.field19_0x24 = 0;
    uVar9[0x3].field_0x54 = 0;
    uVar9[0x3].field47_0x56 = 0;
    uVar9[0x3].field_0x58 = 0;
    uVar9[0x3].field_0x5a = 0;
    uVar9[0x3].field53_0x5e = 0;
    uVar9[0x4].segment_0x2 = 0;
    (uVar9[0x4].field2_0x4 + 0x2) = 0;
    param_1.offset_0x0 = 0x32d8;
    uVar9.segment_0x2 = 0x1018;
    uVar9.field17_0x20 = 0x3314;
    uVar9.field18_0x22 = 0x1018;
    ppuVar13 = CONCAT22(uVar14, 0x2f);
    puVar9 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        ppuVar13,
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar6 = paVar5 & 0xffff0000;
    uVar9[0x4].segment_0x2 = puVar9;
    uVar9[0x4].field2_0x4 = (puVar9 >> 0x10);
    if (param_1.is_null()) {
        puVar3 = null_mut();
    } else {
        uVar6 |= uVar12;
        puVar3 = &uVar9.field17_0x20;
    }
    uVar1 = &uVar9[0x4].segment_0x2;
    ppcVar2 = (*&uVar9[0x4].segment_0x2 + 0x4);
    (**ppcVar2)(
        0x1010,
        uVar1,
        (uVar1 >> 0x10),
        0x0,
        puVar3,
        uVar6,
        (ppuVar13 >> 0x10),
    );
    uVar1 = &uVar9[0x4].segment_0x2;
    uVar1 = (uVar1 + 0x24);
    uVar9[0x3].field_0x5a = uVar1;
    uVar4 = FUN_1010_830a(uVar1, uVar6, 0x1010, _u16_1050_14cc, 0x6e);
    uVar9.field19_0x24 = uVar4;
    uVar9.field20_0x26 = uVar6;
    (uVar9.field20_0x26 + 0x2) = 0;
    uVar10 = pass1_1008_4772(&uVar9.field19_0x24);
    uVar14 = (uVar10 >> 0x10);
    pass1_1018_4b78(param_1);
    uVar9[0x3].field_0x4c = 0x1;
    uVar9[0x3].field_0x4e = 0x1;
    uVar9[0x3].field_0x50 = (uVar10 + 0x4) + &uVar9[0x3].field_0x4c;
    uVar9[0x3].field44_0x52 = (uVar10 + 0x8) - 0x19;
    return;
}

pub unsafe fn pass1_1018_2c60(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut puVar4: *mut u16;
    let mut uVar5: u16;
    let mut uVar6: *mut StructD;
    let mut uVar7: u16;
    let mut puStack6: *mut u16;

    uVar7 = (param_1 >> 0x10);
    uVar6 = param_1;
    param_1.address_offset_field_0x0 = 0x32d8;
    uVar6.address_offset_field_0x2 = 0x1018;
    uVar6.field19_0x20 = 0x3314;
    uVar6.field20_0x22 = 0x1018;
    if (&uVar6[0x1].field_0x94 != 0) {
        if (param_1.is_null()) {
            puVar4 = null_mut();
            uVar5 = 0;
        } else {
            puVar4 = &uVar6.field19_0x20;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6(&uVar6[0x1].field_0x94, CONCAT22(uVar5, puVar4));
    }
    fn_ptr_1000_17ce(*&uVar6[0x1].field_0x98);
    puVar1 = &uVar6.field_0x24;
    uVar2 = &uVar6.field_0x26;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1000, puVar1, uVar2, 1);
    }
    if (param_1.is_null()) {
        puVar4 = null_mut();
        uVar7 = 0;
    } else {
        puVar4 = &uVar6.field19_0x20;
    }
    puStack6 = CONCAT22(uVar7, puVar4);
    *puStack6 = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1018_2d22(
    mut param_1: u32,
    param_2: *mut i16,
    param_3: *mut u16,
    mut param_4: i16,
) {
    let mut uVar1: u32;

    *param_3 = 0;
    *param_2 = 0;
    uVar1 = pass1_1008_4772((param_1 + 0x24));
    *param_2 = (uVar1 + 0x8) - 0x14;
    if (param_4 == 0xbb8) {
        *param_3 = 0x5;
    }
    if (param_4 == 0xbba) {
        *param_3 = 0x23;
    }
    if (param_4 == 0xbb9) {
        *param_3 = 0x75;
    }
    return;
}

pub unsafe fn pass1_1018_2d84(mut param_1: u16, param_2: *mut astruct_126) {
    pass1_1018_2e28(param_2);
    pass1_1020_bd80(param_1);
    return;
}

pub unsafe fn pass1_1018_2d9a(param_1: *mut astruct_126) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut iVar4: *mut astruct_126;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar3 = iVar4.field375_0x180 | iVar4.field374_0x17e;
    if (uVar3 != 0) {
        piVar1 = &iVar4.field369_0x174;
        *piVar1 = *piVar1 - 0x1;
        if (*piVar1 < 0x0) {
            uVar2 = &iVar4.field374_0x17e;
            uVar3 = (uVar2 + 0xa) - 0x1;
            iVar4.field369_0x174 = uVar3;
        }
        pass1_1018_2e28(param_1);
        iVar4.field370_0x176 = uVar3;
    }
    return;
}

pub unsafe fn pass1_1018_2dde(param_1: *mut astruct_126) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut iVar4: *mut astruct_126;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4.field375_0x180 | iVar4.field374_0x17e) != 0) {
        piVar1 = &iVar4.field369_0x174;
        *piVar1 = *piVar1 + 1;
        iVar3 = iVar4.field369_0x174;
        uVar2 = &iVar4.field374_0x17e;
        piVar1 = (uVar2 + 0xa);
        if (*piVar1 == iVar3 || *piVar1 < iVar3) {
            iVar4.field369_0x174 = 0;
        }
        pass1_1018_2e28(param_1);
        iVar4.field370_0x176 = iVar3;
    }
    return;
}

pub unsafe fn pass1_1018_2e28(param_1: *mut astruct_126) {
    let mut uVar1: u16;
    let mut extraout_DX: u16;

    uVar1 = (param_1 + 0x174);
    empty_1008_8fc4();
    if ((extraout_DX | uVar1) != 0) {
        return;
    }
    return;
}


pub unsafe fn pass1_1018_2e5e(mut param_1: u16, mut param_2: u16, param_3: *mut astruct_126) {
    let mut uVar3: u32;
    let mut uVar1: u16;
    let mut iVar5: i16;
    let mut iVar4: *mut astruct_126;
    let mut uVar2: *mut astruct_126;

    uVar2 = (param_3 >> 0x10);
    iVar4 = param_3;
    if (&iVar4.field374_0x17e == 0) {
        pass1_1030_82f0(_u16_1050_5748, iVar4.field373_0x17a);
        iVar4.field374_0x17e = param_1;
        iVar4.field375_0x180 = param_2;
    }
    if ((&iVar4.field374_0x17e != 0) && (uVar3 = &iVar4.field374_0x17e, (uVar3 + 0xa) != 0)) {
        iVar5 = iVar4.field369_0x174;
        empty_1008_8fc4();
        pass1_1018_2e28(param_3);
        iVar4.field370_0x176 = iVar5;
        return;
    }
    return;
}




pub unsafe fn pass1_1018_2fe8(param_1: *mut astruct_126, param_2: u16, param_3: u16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut extraout_DX: u16;
    let mut uVar8: u16;
    let mut pstruct126_9: *mut astruct_126;
    let mut uVar9: *mut astruct_126;

    uVar9 = (param_1 >> 0x10);
    pstruct126_9 = param_1;
    iVar6 = pstruct126_9.field369_0x174;
    uVar2 = &pstruct126_9.field374_0x17e;
    iVar7 = (uVar2 + 0xa);
    if (iVar7 != 0) {
        if (pstruct126_9[0x1].field4_0x4.is_null() == false) {
            uVar3 = str_op_1000_3da4(pstruct126_9[0x1].field4_0x4);
            pstruct126_9.field369_0x174 = 0;
            loop {
                if (iVar7 <= pstruct126_9.field369_0x174) {
                    break;
                }
                uVar4 = pstruct126_9.field369_0x174;
                empty_1008_8fc4();
                uVar8 = extraout_DX;
                pass1_1018_2e28(param_1);
                uVar4 = pass1_1020_bd80(uVar4);
                uVar5 = pass1_1000_3de8(
                    CONCAT22(uVar8, uVar4),
                    pstruct126_9[0x1].field4_0x4,
                    uVar3,
                    param_2,
                    param_3,
                );
                if (uVar5 == 0) {
                    break;
                }
                piVar1 = &pstruct126_9.field369_0x174;
                *piVar1 = *piVar1 + 1;
            }
            if (pstruct126_9.field369_0x174 < iVar7) {
                pass1_1018_2e28(param_1);
                pstruct126_9.field370_0x176 = iVar7;
                return;
            }
            pstruct126_9.field369_0x174 = iVar6;
            pass1_1018_2e28(param_1);
            pstruct126_9.field370_0x176 = iVar6;
        }
    }
    return;
}