pub unsafe fn pass1_1028_1024(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15) -> i16 {
    let mut BVar1: bool;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut iStack26: i16;
    let mut iStack24: i16;
    let mut local_16: u32;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut uStack12: u32;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_bab6(param_1, param_2, param_3);
    iStack6 = param_1;
    uStack4 = param_2;
    uStack8 = pass1_1030_2fac(CONCAT22(param_2, param_1));
    uStack12 = pass1_1028_bb24(param_3);
    uVar6 = pass1_1028_b58e(param_3);
    uStack14 = (uVar6 >> 0x10);
    local_16 = (uVar6 + 0xc);
    iStack26 = 0;
    iStack24 = 0;
    loop {
        if (uStack8 < iStack26) {
            return iStack24;
        }
        iStack18 = iStack26;
        puVar2 = &local_16;
        pass1_1030_627e(
            puVar2,
            (uVar6 >> 0x10),
            _PTR_LOOP_1050_5740,
            CONCAT22(0x1050, puVar2),
            uStack12,
        );
        uStack16 = uVar6;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6 & 0xffff0000 | ZEXT24(puVar2));
        uStack16 = uVar6;
        uVar3 = (uVar6 >> 0x10) | puVar2;
        if (uVar3 == 0) {
            break;
        }
        uVar6 = struct_op_1030_73a8((uVar6 & 0xffff0000 | ZEXT24(puVar2)), puVar2, uVar3);
        uVar4 = (uVar6 >> 0x10);
        uVar3 = uVar6;
        uVar5 = uVar4 | uVar3;
        if (uVar6 == 0) {
            return iStack24;
        }
        BVar1 = pass1_1008_c6ae(_u16_1050_06e0, (uVar3 + 0xc), 0x13);
        uVar6 = CONCAT22(uVar5, uStack16);
        if ((BVar1 != 0) && ((uVar3 + 0x12) == 0x5)) {
            iStack24 += 0x1;
        }
        iStack26 += 0x1;
    }
    return iStack24;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_1106(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15) -> i16 {
    let mut BVar1: bool;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut iStack26: i16;
    let mut iStack24: i16;
    let mut local_16: u32;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut uStack12: u32;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_bab6(param_1, param_2, param_3);
    iStack6 = param_1;
    uStack4 = param_2;
    uStack8 = pass1_1030_2fac(CONCAT22(param_2, param_1));
    uStack12 = pass1_1028_bb24(param_3);
    uVar5 = pass1_1028_b58e(param_3);
    uStack14 = (uVar5 >> 0x10);
    local_16 = (uVar5 + 0xc);
    iStack26 = 0;
    iStack24 = 0;
    loop {
        if (uStack8 < iStack26) {
            return iStack24;
        }
        iStack18 = iStack26;
        puVar2 = &local_16;
        pass1_1030_627e(
            puVar2,
            (uVar5 >> 0x10),
            _PTR_LOOP_1050_5740,
            CONCAT22(0x1050, puVar2),
            uStack12,
        );
        uStack16 = uVar5;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5 & 0xffff0000 | ZEXT24(puVar2));
        uStack16 = uVar5;
        uVar3 = (uVar5 >> 0x10) | puVar2;
        if (uVar3 == 0) {
            break;
        }
        uVar5 = struct_op_1030_73a8((uVar5 & 0xffff0000 | ZEXT24(puVar2)), puVar2, uVar3);
        uVar4 = (uVar5 >> 0x10);
        uVar3 = uVar4 | uVar5;
        if (uVar5 == 0) {
            return iStack24;
        }
        BVar1 = pass1_1008_c6ae(_u16_1050_06e0, (uVar5 + 0xc), 0x13);
        uVar5 = CONCAT22(uVar3, uStack16);
        if (BVar1 != 0) {
            iStack24 += 0x1;
        }
        iStack26 += 0x1;
    }
    return iStack24;
}

pub unsafe fn pass1_1028_11de(param_1: *mut astruct_15) -> bool {
    let mut uVar1: u32;

    uVar1 = pass1_1028_b58e(param_1);
    return (uVar1 + 0x10) == 0;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_121e(mut param_1: u16, param_2: *mut astruct_15) -> *mut astruct_15 {
    let mut bVar1: bool;
    let mut extraout_AH: u8;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut paVar5: *mut astruct_15;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut uStack6: u32;

    bVar1 = pass1_1028_11de(param_2);
    if (CONCAT11(extraout_AH, bVar1) != 0) {
        return param_2;
    }
    uStack6 = pass1_1028_b58e(param_2);
    local_c = (uStack6 + 0xc);
    uStack8 = 0;
    uVar4 = pass1_1028_bb24(param_2);
    uVar3 = (uVar4 >> 0x10);
    puVar2 = &local_c;
    pass1_1030_627e(
        puVar2,
        uVar3,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        uVar4 & 0xffff | uVar3 << 0x10,
    );
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(uVar3, puVar2));
    if ((uVar3 | puVar2) == 0) {
        return NULL;
    }
    paVar5 = struct_op_1030_73a8(CONCAT22(uVar3, puVar2), puVar2, uVar3 | puVar2);
    return paVar5;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_1416(mut param_1: u16, mut param_2: u32) -> i16 {
    let mut bVar1: bool;
    let mut extraout_AH: u8;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut paVar4: *mut astruct_15;

    bVar1 = pass1_1028_11de(param_2);
    if (CONCAT11(extraout_AH, bVar1) == 0) {
        paVar4 = pass1_1028_121e(&DAT_1050_1050, param_2);
        uVar3 = (paVar4 >> 0x10);
        iVar2 = pass1_1028_1416(uVar3, paVar4 & 0xffff | uVar3 << 0x10);
        return iVar2;
    }
    iVar2 = pass1_1028_1024(CONCAT11(extraout_AH, bVar1), param_1, param_2);
    return iVar2 * 0xf;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_1556(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15) -> u16 {
    let mut iVar1: i16;
    let mut puVar2: *mut u32;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut iStack26: i16;
    let mut local_16: u32;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut uStack12: u32;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_bab6(param_1, param_2, param_3);
    iStack6 = param_1;
    uStack4 = param_2;
    uStack8 = pass1_1030_2fac(CONCAT22(param_2, param_1));
    uStack12 = pass1_1028_bb24(param_3);
    uVar7 = pass1_1028_b58e(param_3);
    uStack14 = (uVar7 >> 0x10);
    local_16 = (uVar7 + 0xc);
    iStack26 = 0x1;
    loop {
        if (uStack8 < iStack26) {
            return 0x0;
        }
        iStack18 = iStack26;
        puVar2 = &local_16;
        pass1_1030_627e(
            puVar2,
            (uVar7 >> 0x10),
            _PTR_LOOP_1050_5740,
            CONCAT22(0x1050, puVar2),
            uStack12,
        );
        uStack16 = uVar7;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7 & 0xffff0000 | ZEXT24(puVar2));
        uStack16 = uVar7;
        uVar4 = (uVar7 >> 0x10) | puVar2;
        if (uVar4 == 0) {
            return 0x0;
        }
        uVar7 = struct_op_1030_73a8((uVar7 & 0xffff0000 | ZEXT24(puVar2)), puVar2, uVar4);
        uVar5 = (uVar7 >> 0x10);
        uVar4 = uVar7;
        uVar6 = uVar5 | uVar4;
        if (uVar7 == 0) {
            return 0x0;
        }
        iVar1 = (uVar4 + 0xc);
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0, iVar1, 0x13);
        uVar7 = CONCAT22(uVar6, uStack16);
        if ((BVar3 == 0) && (iVar1 != 0x75)) {
            break;
        }
        if ((uVar4 + 0x12) != 0x9) {
            return 0x1;
        }
        iStack26 += 0x1;
    }
    return 0x0;
}

pub unsafe fn pass1_1028_1646(param_1: *mut astruct_409) -> *mut astruct_409 {
    let mut paVar1: *mut astruct_409;
    let mut uVar2: *mut astruct_409;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    uVar2 = param_1;
    paVar1 = (uVar2.field32_0x20 -0x4);
    if (paVar1 < (&u16_1050_0008 + 1)) {
        match *paVar1 {
            0 => {
                uVar2.field32_0x20 = 0x5;
            }
            //   break;
            0x1 => {
                uVar2.field32_0x20 = 0x6;
            }
            //   break;
            0x2 => {
                uVar2.field32_0x20 = 0x7;
            }
            //   break;
            0x3 => {
                uVar2.field32_0x20 = 0x8;
            }
            //   break;
            0x4 => {
                uVar2.field32_0x20 = 0x9;
            }
            //   break;
            0x5 => {
                uVar2.field32_0x20 = 0xa;
                return uVar2;
            }
            0x6 => {
                uVar2.field32_0x20 = 0xb;
                return uVar2;
            }
            0x7 => {
                uVar2.field32_0x20 = 0xc;
                return uVar2;
            }
            0x8 => {
                uVar2.field32_0x20 = 0xd;
                return uVar2;
            }
        };
        return uVar2;
    }
    uVar2.field32_0x20 = 0x4;
    return paVar1;
}

pub unsafe fn struct_1028_178c(param_1: *mut u16) -> *mut u16 {
    struct_1030_dc96(param_1);
    *param_1 = 0x1b54;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}

pub unsafe fn pass1_1028_17ae(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u32,
) -> *mut u16 {
    pass1_1030_dcc2(param_1, CONCAT22(param_3, param_2), param_4, param_5);
    CONCAT22(param_3, param_2) = 0x1b54;
    (param_2 + 0x2) = 0x1028;
    return CONCAT22(param_3, param_2);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address


pub unsafe fn struct_1028_1bbc(param_1: *mut astruct_180) -> *mut u16 {
    let mut iVar1: *mut astruct_180;
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 1).field0_0x0 = 0;
    iVar1[0x1].field1_0x2 = 0;
    param_1.field0_0x0 = 0x1eee;
    iVar1.field1_0x2 = 0x1028;
    return &param_1.field0_0x0;
}

pub unsafe fn pass1_1028_1be8(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    (param_2 + 0x22) = 0;
    param_2.field0_0x0 = 0x1eee;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}

pub unsafe fn pass1_1028_1c1c() -> u16 {
    return 0x0;
}




// WARNING: Could not reconcile some variable overlaps


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_1e14(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: i16,
    param_6: *mut u16,
    param_7: i32,
) -> u16 {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u32;

    pass1_1030_627e(param_1, param_2, _PTR_LOOP_1050_5740, param_6, param_7);
    uVar2 = param_2 | param_1;
    if (uVar2 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(param_2, param_1));
        if ((uVar2 | param_1) != 0) {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_1), uVar2 | param_1, uVar2);
            if (uVar3 != 0) {
                iVar1 = (uVar3 + 0xc);
                if (((iVar1 == 0x18) || (iVar1 == 0x3f)) || (iVar1 == param_5)) {
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}


// WARNING: Unable to use type for symbol uVar1
pub unsafe fn struct_1028_1f56(param_1: *mut Struct57, param_2: *mut astruct_180) {
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut iVar3: *mut astruct_180;
    let mut uVar3: u16;
    let mut uVar1: u32;

    struct_1028_b354(param_2);
    uVar3 = (param_2 >> 0x10);
    iVar3 = param_2;
    uVar2 = 0;
    (iVar3 + 1) = 0;
    iVar3[0x1].field_0x4 = 0;
    param_2.field0_0x0 = 0x2572;
    iVar3.field1_0x2 = 0x1028;
    mem_op_1000_179c(0xc, param_1);
    extraout_DX = param_1 | uVar2;
    if (extraout_DX == 0) {
        (iVar3 + 1) = 0;
    } else {
        set_struct_1008_574a(CONCAT22(param_1, uVar2));
        (iVar3 + 1).field0_0x0 = uVar2;
        iVar3[0x1].field1_0x2 = extraout_DX;
    }
    uVar1 = (iVar3 + 1);
    (uVar1 + 0xa) = 0;
    return;
}
pub unsafe fn pass1_1028_1fc8(
    param_1: *mut u8,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    uVar2 = 0;
    (param_2 + 0x20) = 0;
    (param_2 + 0x24) = 0;
    param_2.field0_0x0 = 0x2572;
    (param_2 + 0x2) = 0x1028;
    mem_op_1000_179c(0xc, paVar4);
    uVar3 = paVar4 | uVar2;
    if (uVar3 == 0) {
        (param_2 + 0x20) = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar4, uVar2));
        (param_2 + 0x20) = uVar2;
        (param_2 + 0x22) = uVar3;
    }
    uVar1 = (param_2 + 0x20);
    (uVar1 + 0xa) = 0;
    return;
}
