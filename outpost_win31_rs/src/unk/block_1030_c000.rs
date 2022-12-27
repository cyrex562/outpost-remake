pub fn pass1_1030_c09c(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    (param_2 + 0x24) = 0;
    param_2.field0_0x0 = 0xc68e;
    (param_2 + 0x2) = 0x1030;
    return;
}










// WARNING: Restarted to delay deadcode elimination for space: stack
pub fn pass1_1030_c652(param_1: *mut u8, mut param_2: u16) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut astruct_250;
    let mut in_stack_0000fea8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd2: u16;
    let mut in_stack_0000ffd6: u16;

    paVar1 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x8),
        in_stack_0000fea8,
        in_stack_0000ffcc,
        in_stack_0000ffd2,
        in_stack_0000ffd6,
    );
    pass1_1010_9794(paVar1);
    return;
}



pub fn struct_1030_c6f6(param_1: *mut astruct_180) -> *mut astruct_180 {
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field0_0x0 = 0xc940;
    (param_1 + 0x2) = 0x1030;
    return param_1;
}

pub fn pass1_1030_c71e(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    param_2.field0_0x0 = 0xc940;
    (param_2 + 0x2) = 0x1030;
    return &param_2.field0_0x0;
}












pub fn struct_1030_c9a8(param_1: *mut astruct_180) -> *mut astruct_180 {
    let mut iVar1: *mut astruct_180;
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1[0x4].field16_0x18 = 0x1;
    param_1.field0_0x0 = 0xd88e;
    iVar1.field1_0x2 = 0x1030;
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(iVar1 + 1)), NULL, 0x78);
    return param_1;
}

pub fn pass1_1030_c9e4(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> u32 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x98) = 0x1;
    param_2.field0_0x0 = 0xd88e;
    (param_2 + 0x2) = 0x1030;
    pass1_1000_4906((param_2 & 0xffff0000 | (param_2 + 0x20)), NULL, 0x78);
    return param_2;
}




pub fn pass1_1030_cbf0(mut param_1: i16, mut param_2: u16, mut param_3: i16) -> u16 {
    let mut iVar1: *mut astruct_595;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        if (0x9 < iStack4) {
            return 0x0;
        }
        iVar1 = (iStack4 * 0xc + param_1);
        if ((iVar1.field36_0x24 == param_3) && (iVar1.field37_0x26 == 0x3)) {
            break;
        }
        iStack4 += 0x1;
    }
    iVar1.field37_0x26 = 0;
    iVar1.field38_0x28 = 0;
    return 0x1;
}



pub fn pass1_1030_cc44(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u32,
    mut param_5: i16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut puVar3: *mut u8;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut puVar7: *mut u8;
    let mut extraout_DX_01: u16;
    let mut iVar7: *mut astruct_304;
    let mut iVar8: *mut astruct_303;
    let mut uVar8: u8;
    let mut puVar9: *mut u32;
    let mut puVar10: *mut u32;
    let mut puVar11: *mut u8;
    let mut local_32: [u8; 0x8] = [0; 0x8];
    let mut puStack42: *mut u32;
    let mut uStack38: u32;
    let mut uStack34: u32;
    let mut puStack30: *mut u32;
    let mut uStack26: u16;
    let mut puStack24: *mut u8;
    let mut uStack22: u16;
    let mut puStack20: *mut u8;
    let mut puStack18: *mut u32;
    let mut iStack14: i16;
    let mut uStack12: u16;
    let mut iStack10: i16;
    let mut uStack8: u32;
    let mut iStack4: i16;

    iStack4 = 0;
    uStack8 = (param_4 + 0x4);
    iStack10 = 0;
    loop {
        if (0x9 < iStack10) {
            return;
        }
        iVar8 = (iStack10 * 0xc + param_1);
        if (((iVar8.field35_0x28 == uStack8) && (iVar8.field36_0x2a == uStack8))
            && (iVar8.field33_0x24 == param_5))
        {
            if (iVar8.field34_0x26 == 0x4) {
                iVar2 = param_5;
                pass1_1028_b58e(CONCAT22(param_2, param_1));
                iStack14 = iVar2;
                uStack12 = extraout_DX_00;
                pass1_1030_6e9c(
                    CONCAT13((extraout_DX_00 >> 0x8), CONCAT12(extraout_DX_00, iStack14)),
                    0x1,
                    iVar8.field33_0x24,
                );
                iVar8.field32_0x20 = 0;
                iVar8.field33_0x24 = 0;
                iVar8.field34_0x26 = 0;
                puStack42 = null_mut();
                puStack18 = null_mut();
                _DAT_0000_0006 = param_5;
                uRam0000000a = 0x1;
                uVar4 = switch_1020_c3b4(param_5);
                (puStack18 + 0xc) = uVar4;
                puVar10 = pass1_1008_c6fa(_u16_1050_06e0, 0x4);
                puVar7 = (puVar10 >> 0x10);
                uVar6 = puVar10;
                uVar5 = uVar6;
                puVar11 = puVar7;
                uStack22 = uVar6;
                puStack20 = puVar7;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000001);
                uVar8 = 0x38;
                uStack26 = uVar6;
                puStack24 = puVar7;
                pass1_1038_4d6e(
                    uVar6,
                    puVar7,
                    CONCAT22(puVar7, uVar6),
                    CONCAT22(puVar11, uVar5),
                );
                puStack30 = CONCAT22(puVar7, uVar6);
                ppcVar1 = (*puStack30 + 0x10);
                (**ppcVar1)(&u16_1050_1038, uVar6, puVar7);
                uStack34 = CONCAT22(extraout_DX_01, uVar6);
                uVar6 = extraout_DX_01;
                for uStack38 in 0..uStack34 {
                    puVar9 = pass1_1030_1d7c(uStack34, uVar6, puStack30);
                    uVar5 = (puVar9 >> 0x10);
                    uVar6 = uVar5 | puVar9;
                    if (uVar6 != 0) {
                        puVar3 = local_32;
                        ppcVar1 = (*puVar9 + 0x40);
                        (**ppcVar1)(0x38, puVar9, uVar5, puVar3, 0x1050);
                        uVar6 = extraout_DX;
                        if (puVar3.is_null()) {
                            uVar8 = 0x28;
                            pass1_1028_6408(puVar9, puStack18);
                            break;
                        }
                    }
                }
                puStack42 = puStack30;
                if (puStack30.is_null() == false) {
                    ppcVar1 = *puStack30;
                    (**ppcVar1)(uVar8, puStack30, (puStack30 >> 0x10), 1);
                }
            } else {
                iVar7 = (iStack10 * 0xc + param_1);
                iVar7.field38_0x26 = 0;
                iVar7.field39_0x28 = 0;
            }
            iStack4 += 0x1;
            param_3 += -0x1;
            if (param_3 == 0) {
                return;
            }
        }
        iStack10 += 0x1;
    }
}

pub fn pass1_1030_cde8(mut param_1: i16, mut param_2: u16, mut param_3: i16) -> i16 {
    let mut iVar1: i16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        if (0x9 < iStack4) {
            return -0x1;
        }
        iVar1 = iStack4 * 0xc + param_1;
        if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0)) {
            break;
        }
        iStack4 += 0x1;
    }
    return iStack4;
}



pub fn pass1_1030_ce2e(mut param_1: i16, mut param_2: u16, mut param_3: i16) -> i16 {
    let mut iVar1: i16;
    let mut uStack6: u32;

    uStack6 = 0;
    while uStack < 0xa {
        iVar1 = uStack6 * 0xc + param_1;
        if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0)) {
            uStack6 = uStack6 & 0xffff | (uStack6 + 1) << 0x10;
        }
        uStack6 = uStack6 & 0xffff0000 | (uStack6 + 1)
    }
    return uStack6;
}
pub fn pass1_1030_ce72(
    mut param_1: u32,
    mut param_2: i16,
    mut param_3: u32,
    mut param_4: i16,
) {
    let mut lVar1: i32;
    let mut iVar2: *mut astruct_300;
    let mut iStack10: i16;

    lVar1 = (param_3 + 0x4);
    iStack10 = 0;
    loop {
        if (0x9 < iStack10) {
            return;
        }
        iVar2 = (iStack10 * 0xc + param_1);
        if ((iVar2.field36_0x24 == param_4) && (iVar2.field38_0x28 == 0)) {
            iVar2.field38_0x28 = lVar1;
            if (param_4 == 0x4) {
                iVar2.field37_0x26 = 0x2;
            } else {
                (param_1 + iStack10 * 0xc + 0x26) = 0x1;
            }
            param_2 += -0x1;
            if (param_2 == 0) {
                return;
            }
        }
        iStack10 += 0x1;
    }
}
pub fn pass1_1030_cef8(
    mut param_1: u32,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + param_4 * 0xc + 0x26) = param_3;
    uVar4 = (param_2 >> 0x10);
    uVar1 = (param_2 + 0x6);
    (iVar2 + param_4 * 0xc + 0x28) = (param_2 + 0x4);
    (iVar2 + param_4 * 0xc + 0x2a) = uVar1;
    return;
}

pub fn pass1_1030_cf3a(mut param_1: u32, mut param_2: i16) -> u16 {
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        if (0x9 < iStack4) {
            return 0x0;
        }
        if ((param_1 + iStack4 * 0xc + 0x24) == param_2) {
            break;
        }
        iStack4 += 0x1;
    }
    return 0x1;
}
pub fn pass1_1030_cf78(param_1: *mut astruct_15, mut param_2: u16) {
    let mut uVar1: u32;
    let mut extraout_DX: u16;
    let mut iVar3: *mut astruct_680;
    let mut uVar2: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        if (0x9 < iStack4) {
            return;
        }
        uVar1 = param_2;
        uVar2 = (param_1 >> 0x10);
        if ((param_1 + iStack4 * 0xc + 0x24) == param_2) {
            break;
        }
        iStack4 += 0x1;
    }
    pass1_1028_b58e(param_1);
    if (param_2 == 0x5) {
        pass1_1038_4900((uVar1 + 0x2e));
    } else {
        pass1_1030_6e9c((uVar1 & 0xffff | extraout_DX << 0x10), 0x1, param_2);
    }
    iVar3 = (iStack4 * 0xc + param_1);
    iVar3.field32_0x20 = 0;
    iVar3.field33_0x24 = 0;
    iVar3.field34_0x26 = 0;
    return;
}
