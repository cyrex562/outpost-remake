pub unsafe fn pass1_1010_0e46(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_0350(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_0e6c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_0ee6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    clenaup_win_ui_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn struct_1010_0f9c(
    mut param_1: u16,
    param_2: *mut astruct_232,
    param_3: *mut Struct57,
) {
    let mut puVar1: *mut u8;
    let mut ppcVar2: *mut *mut code;
    let mut iVar3: i16;
    let mut paVar4: *mut astruct_92;
    let mut paVar5: *mut astruct_92;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut iVar8: *mut astruct_232;
    let mut iVar9: *mut astruct_231;
    let mut iVar10: *mut astruct_230;
    let mut iVar11: *mut astruct_233;
    let mut uVar9: *mut astruct_232;
    let mut uVar10: u16;
    let mut paVar11: *mut astruct_232;
    let mut uVar12: u8;
    let mut uStack36: u32;
    let mut iStack32: i16;
    let mut uStack30: u16;
    astruct_92 * *ppaStack28;
    let mut paStack24: *mut astruct_15;
    let mut local_14: *mut astruct_92;

    uVar9 = (param_2 >> 0x10);
    iVar8 = param_2;
    ppcVar2 = (param_2 + 0x38);
    (**ppcVar2)();
    iVar8.field100_0x68 = param_1;
    if ((&iVar8.field96_0x60 != 0) && (iVar8.field100_0x68 == 1)) {
        return;
    }
    if (iVar8.field100_0x68 == 0) {
        return;
    }
    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    iVar3 = iVar8.field100_0x68 * 0x18;
    mem_op_1000_179c(iVar3, param_3);
    iVar8.field96_0x60 = iVar3;
    iVar8.field97_0x62 = param_3;
    ppaStack28 = CONCAT22(param_3, iVar8.field96_0x60);
    uStack30 = iVar8.field100_0x68;
    loop {
        loop {
            paVar4 = &local_14;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
            uVar8 = param_3;
            paStack24 = CONCAT22(uVar8, paVar4);
            param_3 = (param_3 & 0xffff0000 | (uVar8 | paVar4));
            //   if ((uVar8 | paVar4) == 0) goto LAB_1010_10ca;
            iVar9 = param_2;
            ppcVar2 = &iVar9.field58_0x40;
            paVar5 = paVar4;
            (**ppcVar2)(0x1028, param_2);
            if paVar5.is_null() == false {
                break;
            }
        }
        pass1_1028_b58e(paStack24);
        uStack36 = CONCAT22(param_3, paVar5);
        ppcVar2 = &iVar9.field44_0x2c;
        (**ppcVar2)(0x1028, param_2);
        uVar10 = (ppaStack28 >> 0x10);
        iVar10 = ppaStack28;
        *ppaStack28 = paVar5;
        iVar10.field2_0x2 = param_3;
        uVar12 = SUB21(paVar4, 0x0);
        ppcVar2 = &iVar9.field46_0x30;
        paVar11 = param_2;
        (**ppcVar2)();
        iVar10.field7_0x8 = paVar5;
        iVar10.field8_0xa = param_3;
        iVar10.field9_0xc = uStack36;
        ppcVar2 = &iVar9.field56_0x3c;
        uVar7 = uStack36;
        (**ppcVar2)(0x1028, param_2, paStack24, paVar11, uVar12, uVar8);
        iVar10.field10_0x10 = uVar7;
        iVar10.field11_0x12 = param_3;
        iVar10.field12_0x14 = uStack36;
        ppaStack28 = (ppaStack28 & 0xffff0000 | ZEXT24(iVar10 + 1));
        uStack30 -= 1;
        if uStack30 == 0 {
            break;
        }
    }
    // LAB_1010_10ca:
    uVar6 = iVar8.field100_0x68 << 0x2;
    mem_op_1000_179c(uVar6, param_3);
    iVar8.field98_0x64 = uVar6;
    iVar8.field99_0x66 = param_3;
    iStack32 = 0;
    uStack30 = 0;
    loop {
        if ((iVar8.field100_0x68 * 0x3) <= uStack30) {
            break;
        }
        puVar1 = iVar8.field97_0x62;
        uVar7 = &iVar8.field98_0x64;
        uVar10 = (uVar7 >> 0x10);
        iVar11 = uVar7;
        (iVar11 + iStack32 * 0x4) = iVar8.field96_0x60 + uStack30 * 0x8;
        (iVar11 + iStack32 * 0x4 + 0x2) = puVar1;
        uStack30 += 0x3;
        iStack32 += 0x1;
    }
    return;
}

pub unsafe fn pass1_1010_116c(param_1: u32, mut param_2: i16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut in_EDX: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uStack4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x56) != 0) {
        ppcVar1 = (*param_1 + 0x34);
        (**ppcVar1)();
    }
    ppcVar1 = (*param_1 + 0x28);
    iVar2 = (**ppcVar1)();
    if (iVar2 != 0) {
        uStack4 = DAT_1050_0ecc;
        iVar2 = DAT_1050_0ecc + 1;
        if (iVar2 == 0) {
            uStack4 = 0;
        }
        pass1_1010_1146(param_1, uStack4);
        pass1_1010_11c6(iVar2, in_EDX, param_1);
        (iVar3 + 0x56) = iVar2;
        (iVar3 + 0x58) = in_EDX;
    }
    return;
}

pub unsafe fn pass1_1010_1656(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u16,
    param_4: *mut Struct27,
    mut param_5: u16,
    mut param_6: u32,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut paVar9: *mut astruct_15;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    unk_destroy_win_op_1010_305a(param_1, param_4, param_5, param_6);
    if ((param_4 + 0x16) == 0x3) {
        paVar8 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(param_3, 0x32),
            in_stack_0000fe88,
            in_stack_0000ffac,
            in_stack_0000ffb2,
            in_stack_0000ffb6,
        );
        uVar5 = paVar4 & 0xffff0000;
        uVar1 = (param_4 + 0x32);
        uVar1 = (uVar1 + 0x42);
        uVar7 = (uVar1 >> 0x10);
        iVar6 = uVar1;
        uVar1 = (iVar6 + 0x16);
        paVar9 = struct_op_1030_73a8((uVar1 + 0x4), iVar6, (paVar8 >> 0x10));
        uVar5 = uVar5 & 0xffff0000 | paVar9 >> 0x10;
        uVar2 = pass1_1010_7818(paVar8, paVar9);
        uVar1 = (iVar6 + 0x16);
        uVar3 = uVar2;
        ui_op_1010_79aa(paVar8, 0x0, (uVar1 + 0x4));
        if (uVar3 == 0) {
            uVar1 = (iVar6 + 0x16);
            unk_win_op_1010_7300(uVar5, paVar8, 0x0, uVar2, (uVar1 + 0x4));
        }
    }
    return;
}


pub unsafe fn pass1_1010_16ee(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut in_stack_0000ffc0: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_6);
    mem_op_1000_179c(0x4a, paVar2);
    uVar1 = paVar2 | param_5;
    if (uVar1 != 0) {
        pass1_1040_c54a(
            CONCAT22(paVar2, param_5),
            0x0,
            CONCAT22(param_4, param_3),
            in_stack_0000ffc0,
            paVar2 & 0xffff0000 | uVar1,
        );
        return;
    }
    return;
}

pub unsafe fn string_1010_1722(mut param_1: u16, mut param_2: u16, mut param_3: u16, mut param_4: u32) {
    let mut extraout_DX: u16;
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut uVar3: u16;
    let mut local_52: [u8; 0x50] = [0; 0x50];

    pass1_1028_b58e(param_4);
    if ((extraout_DX | param_1) == 0) {
        pcVar2 = load_string_1010_847e(_u16_1050_14cc, 0x424);
        uVar1 = (pcVar2 >> 0x10);
        unk_str_op_1000_3d3e(CONCAT22(0x1050, local_52), pcVar2);
        pcVar2 = CONCAT22(uVar1, local_52);
        uVar3 = &DAT_1050_1050;
    } else {
        pcVar2 = pass1_1038_4d28(*(param_1 + 0x2e));
        uVar3 = (pcVar2 >> 0x10);
    }
    str_op_1008_60e8((pcVar2 >> 0x10), (pcVar2 & 0xffff | uVar3 << 0x10));
    return;
}

pub unsafe fn pass1_1010_1788(
    param_1: *mut c_char,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut astruct_15,
) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut puVar5: *mut u8;
    let mut in_stack_0000fff4: u32;
    u8 * *ppuVar6;
    let mut iVar7: i16;

    ppuVar6 = CONCAT22((in_stack_0000fff4 >> 0x10), 0x3);
    puVar4 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        ppuVar6,
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    uVar3 = (puVar4 >> 0x10);
    iVar7 = (ppuVar6 >> 0x10);
    puVar5 = 0x1778;
    uVar1 = pass1_1028_b58e(param_4);
    pcVar2 = pass1_1010_b038(puVar4, uVar1, uVar3, puVar5, iVar7);
    str_op_1008_60e8(uVar3, CONCAT22(uVar3, pcVar2));
    return;
}

pub unsafe fn pass1_1010_184a(param_1: u32, param_2: *mut u32) {
    let mut iVar1: i16;
    let mut iVar2: i16;

    iVar2 = DAT_1050_0ecc;
    iVar1 = (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    iVar1 = pass1_1000_475e((iVar1 + *param_1), (iVar1 + *param_2));
    if (iVar1 == 0) {
        iVar1 = (iVar2 * 0x6 + 0xebc) * 0x8;
        iVar1 = pass1_1000_475e((iVar1 + *param_1), (iVar1 + *param_2));
        if (iVar1 == 0) {
            iVar2 = (iVar2 * 0x6 + 0xebe) * 0x8;
            pass1_1000_475e((iVar2 + *param_1), (iVar2 + *param_2));
        }
    }
    return;
}

pub unsafe fn FUN_1010_18e8() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1010_18ee() -> u16 {
    return 0x1;
}

pub unsafe fn pass1_1010_18f4(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_19a4(mut param_1: u16, param_2: *mut u32) {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_92;
    let mut extraout_DX: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        if ((param_1 | paVar2) == 0) {
            break;
        }
        ppcVar1 = (*param_2 + 0x40);
        (**ppcVar1)(0x1028, param_2);
        param_1 = extraout_DX;
    }
    return;
}

pub unsafe fn pass1_1010_1a06(
    mut param_1: u32,
    param_2: *mut astruct_15,
    mut param_3: i16,
    mut param_4: u16,
) {
    let mut pcVar1: *mut c_char;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar2: u16;
    let mut in_EDX: *mut Struct57;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar3: *mut u32;
    let mut pcVar4: *mut c_char;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffee: i16;
    let mut uVar1: u32;

    uVar6 = pass1_1028_b58e(param_2);
    uVar5 = (param_1 >> 0x10);
    pcVar1 = pass1_1010_b038((param_1 + 0x6e), uVar6, in_EDX, 0x1770, in_stack_0000ffee);
    iVar2 = pass1_1000_3e2c(CONCAT22(in_EDX, pcVar1));
    puVar3 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x32),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    uVar2 = (puVar3 >> 0x10);
    uVar3 = pass1_1010_7818(puVar3, param_2);
    uVar1 = (param_1 + 0x6e);
    pcVar4 = string_op_1010_ada6(uVar2, uVar1, (uVar1 >> 0x10), iVar2, uVar3);
    str_op_1008_60e8((pcVar4 >> 0x10), pcVar4);
    return;
}


pub unsafe fn pass1_1010_1a66(mut param_1: u32, param_2: *mut astruct_15) -> u8 {
    let mut uVar2: u32;
    let mut uVar3: u8;
    let mut uVar4: u16;
    let mut BVar4: bool;
    let mut uVar5: *mut astruct_15;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar1: u32;

    uVar5 = (param_2 >> 0x10);
    if (((param_2 + 0x1c) != 0x2) || (((param_2 + 0x1e) & 0xff) != 0)) {
        uVar7 = pass1_1028_b58e((param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
        uVar6 = (param_1 >> 0x10);
        uVar1 = (param_1 + 0x6e);
        pass1_1010_c2d8(uVar1, (uVar1 >> 0x10), uVar7);
        if ((uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0)) {
            uVar2 = (param_1 + 0x6e);
            uVar4 = pass1_1010_b028(uVar2, (uVar2 >> 0x10), param_2);
            BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar4, 0x5);
            if ((BVar4 == 0)
                && (
                    BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar4, 0x6),
                    BVar4 == 0,
                ))
            {
                uVar3 = '\0';
            } else {
                uVar3 = '\x01';
            }
            return uVar3;
        }
    }
    return '\0';
}


pub unsafe fn pass1_1010_1b04(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1010_1b6e(
    param_1: *mut StructD,
    param_2: *mut Struct19,
    param_3: *mut Struct19,
    mut param_4: u16,
) -> *mut Struct19 {
    let mut unaff_BP: u16;
    let mut puVar1: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    pass1_1010_0f24(param_1, CONCAT22(param_3, param_2), param_4);
    param_2[0x1].field_0xe = 0;
    CONCAT22(param_3, param_2) = 0x1d04;
    param_2.segment_0x2 = 0x1010;
    puVar1 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x3),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    param_2[0x1].field_0xe = puVar1;
    param_2[0x1].field8_0x10 = (puVar1 >> 0x10);
    return CONCAT22(param_3, param_2);
}

pub unsafe fn pass1_1010_1bb4(mut param_1: u16, param_2: *mut u32) {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_92;
    let mut extraout_DX: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        if ((param_1 | paVar2) == 0) {
            break;
        }
        ppcVar1 = (*param_2 + 0x40);
        (**ppcVar1)(0x1028, param_2);
        param_1 = extraout_DX;
    }
    return;
}

pub unsafe fn pass1_1010_1c16(mut param_1: u32, param_2: *mut astruct_15, mut param_3: i16) {
    let mut pcVar1: *mut c_char;
    let mut uVar3: *mut astruct_15;
    let mut uVar2: *mut astruct_15;
    let mut uVar4: u32;

    uVar4 = pass1_1028_b58e(param_2);
    uVar3 = (uVar4 >> 0x10);
    uVar2 = uVar3;
    pcVar1 = pass1_1010_b038((param_1 + 0x6e), uVar4, uVar3, 0x178a, param_3);
    str_op_1008_60e8(uVar2, CONCAT22(uVar2, pcVar1));
    return;
}


pub unsafe fn pass1_1010_1c40(mut param_1: u32, param_2: *mut astruct_15) -> u8 {
    let mut uVar4: u32;
    let mut uVar3: u16;
    let mut BVar5: bool;
    let mut uVar5: *mut astruct_15;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar2: u8;
    let mut uVar1: u32;

    uVar5 = (param_2 >> 0x10);
    if (((param_2 + 0x1c) != 0x2) || (((param_2 + 0x1e) & 0xff) != 0)) {
        uVar7 = pass1_1028_b58e((param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
        uVar6 = (param_1 >> 0x10);
        uVar1 = (param_1 + 0x6e);
        pass1_1010_c2d8(uVar1, (uVar1 >> 0x10), uVar7);
        if ((uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0)) {
            uVar4 = (param_1 + 0x6e);
            uVar3 = pass1_1010_b028(uVar4, (uVar4 >> 0x10), param_2);
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar3, 0x11);
            if ((BVar5 == 0)
                && (
                    BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar3, 0x12),
                    BVar5 == 0,
                ))
            {
                uVar2 = '\0';
            } else {
                uVar2 = '\x01';
            }
            return uVar2;
        }
    }
    return '\0';
}

pub unsafe fn pass1_1010_1cde(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_1dce() -> u16 {
    return 0x0;
}

pub unsafe fn pass1_1010_1dd4() -> u16 {
    return 0x0;
}

pub unsafe fn pass1_1010_1df2(
    param_1: *mut astruct_242,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
    param_5: *mut Struct57,
) {
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: *mut astruct_241;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar3: *mut astruct_242;
    let mut uVar4: u16;
    let mut puStack10: *mut u16;
    let mut puStack6: *mut u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field4_0x4.is_null()) {
        mem_op_1000_179c(0xc, param_5);
        uVar2 = param_5;
        uVar3 = uVar2 | in_AX;
        param_5 = (param_5 & 0xffff0000 | uVar3);
        if (uVar3 == 0) {
            iVar3.field4_0x4 = null_mut();
        } else {
            set_struct_1008_574a(CONCAT22(uVar2, in_AX));
            iVar3.field4_0x4 = in_AX;
            (iVar3.field4_0x4 + 0x2) = param_5;
        }
    }
    mem_op_1000_179c(0xa, param_5);
    uVar2 = param_5;
    puStack10 = CONCAT22(uVar2, in_AX);
    if ((uVar2 | in_AX) == 0) {
        puStack6 = null_mut();
    } else {
        *puStack10 = 0x389a;
        in_AX.field2_0x2 = 0x1008;
        in_AX.field3_0x4 = param_3;
        in_AX.field4_0x8 = param_2;
        *puStack10 = 0x2010;
        in_AX.field2_0x2 = 0x1010;
        puStack6 = puStack10;
    }
    ppcVar1 = (*iVar3.field4_0x4 + 0x4);
    (**ppcVar1)(0x1000, iVar3.field4_0x4, puStack6);
    return;
}


pub unsafe fn pass1_1010_1fbe(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1010_1fea(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_2bb9() {
    pass1_1010_286c();
    return;
}

pub unsafe fn pass1_1010_2bbe(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1010_29c6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_2c9c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1010_394a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: i16,
) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;

    paVar1 = CONCAT22(in_register_0000000a, param_2);
    if (param_5 != 0) {
        mem_op_1000_179c(param_5 << 0x2, paVar1);
        return;
    }
    mem_op_1000_179c(0x16, paVar1);
    if ((paVar1 | param_1) != 0) {
        struct_1010_383a(CONCAT22(paVar1, param_1));
        return;
    }
    return;
}

pub unsafe fn pass1_1010_3a86(mut param_1: u32) -> u16 {
    return (param_1 + 0x10);
}

pub unsafe fn pass1_1010_3a94(mut param_1: u32, mut param_2: u16) {
    (param_1 + 0x12) = param_2;
    return;
}

pub unsafe fn FUN_1010_3aa6() {
    return;
}

pub unsafe fn pass1_1010_3aaa(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x6), (param_1 + 0x4));
}

pub unsafe fn FUN_1010_3abc() -> u16 {
    return 0x0;
}

pub unsafe fn pass1_1010_3ac2(mut param_1: u32, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x12) = param_2;
    return;
}

pub unsafe fn pass1_1010_3adc(mut param_1: u32) -> u32 {
    let mut puVar1: *mut u16;

    puVar1 = (param_1 + 0x16);
    return CONCAT22((puVar1 + 0x2), *puVar1);
}


pub unsafe fn pass1_1010_3af2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1010_3800(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1010_3b18(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1010_3880(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_3d0a(mut param_1: i16, mut param_2: u16, mut param_3: i16, mut param_4: u16) {
    if (param_3 == 0x2) {
        pass1_1010_3cd0(CONCAT22(param_2, param_1 -0xa));
        pass1_1010_1f62(CONCAT22(param_2, param_1 -0xa), 0x2);
    }
    return;
}

pub unsafe fn pass1_1010_3d38(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
    pass1_1010_3bde(param_1, &DAT_1050_1050);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1010_3d44(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1010_3bde(param_2, &DAT_1050_1050);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn pass1_1010_3e06(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1010_3dc8(param_1, &DAT_1050_1050);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn FUN_1010_3fc2(mut param_1: u16, mut param_2: u32, param_3: *mut u8) -> u16 {
    let mut BVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_stack_0000ffda: HFILE16;
    let mut local_c: [u16; 0x3] = [0; 0x3];
    let mut local_6: [u16; 0x2] = [0; 0x2];

    BVar1 = write_to_file_1008_7cac(param_3);
    if (BVar1 != 0) {
        uVar3 = (param_2 >> 0x10);
        iVar2 = param_2;
        local_c[0] = (iVar2 + 0x24);
        BVar1 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffda);
        if (BVar1 != 0) {
            local_6[0] = (iVar2 + 0x6a);
            BVar1 =
                write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffda);
            if (BVar1 != 0) {
                local_6[0] = (iVar2 + 0x7e);
                BVar1 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, local_6),
                    0x2,
                    in_stack_0000ffda,
                );
                if (BVar1 != 0) {
                    return 0x1;
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}

pub unsafe fn pass1_1010_404a(mut param_1: i16, mut param_2: u32, mut param_3: u32) {
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut local_4: u16;

    read_file_1008_7cfe(param_3, (param_3 >> 0x10), 0x5);
    if (param_1 == 0) {
        u16_1050_0310 = 0x6d4;
    } else {
        iVar1 = param_2;
        BVar2 = read_file_1008_7dee(param_3, (param_2 & 0xffff0000 | (iVar1 + 0x24)), 0x2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_3, CONCAT22(0x1050, &local_4), 0x2);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(param_3, (param_2 & 0xffff0000 | (iVar1 + 0x7e)), 0x2);
                if (BVar2 != 0) {
                    (iVar1 + 0x6a) = local_4;
                    return;
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn pass1_1010_40cc(mut param_1: i16, mut param_2: u16, mut param_3: u32) -> u32 {
    pass1_1030_8344(_u16_1050_5748, (param_3 + 0x6c));
    return CONCAT22((param_1 + 0xe), (param_1 + 0xc));
}

pub unsafe fn pass1_1010_4566(mut param_1: i16, mut param_2: u16, mut param_3: i16) {
    if (param_3 != 0x2) {
        return;
    }
    pass1_1010_4956(CONCAT22(param_2, param_1 -0x20));
    pass1_1010_1f62(CONCAT22(param_2, param_1 -0x20), 0x2);
    return;
}

pub unsafe fn get_sys_metrics_1010_46f6(mut param_1: u32, param_2: *mut Struct57) {
    let mut IVar1: i16;
    let mut IVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut uVar6: u32;
    let mut in_stack_0000fe7c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffaa: u16;
    let mut piVar7: *mut i16;
    let mut uVar8: u16;
    let mut pcVar9: *mut c_char;
    let mut local_6: i16;
    let mut local_4: i16;

    pcVar9 = CONCAT22(0x1050, &local_4);
    piVar7 = &local_6;
    uVar8 = SUB42(&DAT_1050_1050, 0x0);
    puVar5 = mixed_1010_20ba(
        param_2,
        _u16_1050_0ed0,
        CONCAT22(piVar7, 0x48),
        in_stack_0000fe7c,
        in_stack_0000ffa0,
        in_stack_0000ffa6,
        in_stack_0000ffaa,
    );
    pass1_1008_3e94(
        (puVar5 & 0xffff0000 | (puVar5 + 0xe)),
        CONCAT22(uVar8, piVar7),
        pcVar9,
    );
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar6 = pass1_1008_4772((iVar3 + 0x66));
    uVar8 = (uVar6 >> 0x10);
    (iVar3 + 0x18) = local_4 + 0x8;
    (iVar3 + 0x1a) = local_6 + 0x9;
    IVar1 = GetSystemMetrics16(SM_CXBORDER);
    (iVar3 + 0x1c) = IVar1 * 0x2 + (uVar6 + 0x4);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    IVar2 = GetSystemMetrics16(SM_CYBORDER);
    (iVar3 + 0x1e) = IVar2 + IVar1 + (uVar6 + 0x8);
    return;
}
