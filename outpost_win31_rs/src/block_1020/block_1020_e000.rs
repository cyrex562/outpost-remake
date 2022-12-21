//
// Created by cyrex on 2022-05-22.
//

// #include "block_1020.h"



pub unsafe fn pass1_1020_e044(mut param_1: u32) -> BOOL16 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut paVar4: *mut astruct_598;

    uVar3 = (param_1 >> 0x10);
    paVar4 = pass1_1018_04b8((param_1 + 0x28));
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, paVar4);
    uVar2 = pass1_1030_2fac(paVar4);
    uVar1 = (param_1 + 0x28);
    if (uVar2 <= (uVar1 + 0x1e)) {
        return 0x1;
    }
    return 0x0;
}



pub unsafe fn pass1_1020_e08e(mut param_1: u16, param_2: *mut astruct_15) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut iVar4: i16;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut extraout_DX: u16;
    let mut uVar7: u16;
    let mut pstruct15_7: *mut astruct_15;
    let mut pstruct15_7_seg: *mut astruct_15;
    let mut piVar8: *mut i16;
    let mut uVar9: u16;
    let mut puVar10: *mut u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut local_158: u16;
    let mut uStack342: u16;
    let mut uStack50: u32;
    let mut puStack42: *mut u32;
    let mut local_22: i16;
    let mut local_20: [u8; 0x2] = [0; 0x2];
    let mut local_1e: [u8; 0x2] = [0; 0x2];
    let mut uStack28: u16;
    let mut piStack26: *mut i16;
    let mut local_18: i16;
    let mut local_16: u16;
    let mut uStack20: u32;
    let mut local_10: u32;
    let mut iStack12: i16;
    let mut uStack10: u32;
    let mut uStack6: u32;

    pstruct15_7_seg = (param_2 >> 0x10);
    pstruct15_7 = param_2;
    iVar3 = pstruct15_7.field10_0xc;
    if (iVar3 == 0x74) {
        uVar1 = (pstruct15_7 + 1).field0_0x0;
        iVar3 = uVar1 - 0x1;
        //    if (iVar3 == 0) goto LAB_1020_e0ae;
        iVar3 = uVar1 - 0x6;
        //    if (iVar3 != 0) goto LAB_1020_e0b9;
        uVar11 = 0x1;
    } else if (iVar3 == 0x78) {
        uVar1 = (pstruct15_7 + 1).field0_0x0;
        iVar4 = uVar1 - 0x1;
        if (iVar4 != 0) {
            iVar3 = uVar1 - 0x2;
            if ((0x0 < iVar4) && (!SBORROW2(iVar4, 1))) {
                if (uVar1 - 0x5 == 0x0 || iVar3 < 0x3) {
                    iVar3 = uVar1 - 0x5;
                    pass1_1020_e49a(param_2);
                } else {
                    iVar3 = uVar1 - 0x6;
                    if (iVar3 == 0) {
                        pass1_1020_e39c(0x0, param_2, 0x6);
                        pass1_1020_dca8(param_1, param_2);
                    }
                }
            }
            // TODO: goto LAB_1020_e0b9;
        }
        uVar11 = 0x6a;
        iVar3 = iVar4;
    } else {
        iVar3 += -0x79;
        if (iVar3 == 0) {
            pass1_1020_e49a(param_2);
            return;
        } //
        // LAB_1020_e0ae:
        uVar11 = 0x47;
    }
    pass1_1020_e39c(iVar3, param_2, uVar11); //
    // LAB_1020_e0b9:
    pass1_1028_b58e(param_2);
    uStack6 = CONCAT22(extraout_DX, iVar3);
    uVar5 = (iVar3 + 0x2e);
    uVar7 = (iVar3 + 0x30);
    uStack10 = uVar5;
    if (pstruct15_7.field10_0xc != 0x79) {
        pass1_1038_5804(uVar5 & 0xffff | uVar7 << 0x10, 0x1, 0x2);
    }
    if ((pstruct15_7 + 1).field0_0x0 == 0x6) {
        pass1_1038_43cc(uVar5, uVar7, uStack10, (uStack10 >> 0x10), 0x1, 0x2);
    }
    local_10 = (uStack6 + 0xc);
    iStack12 = (uStack6 + 0x10);
    puStack42 = &local_10;
    if (((pstruct15_7 + 1).field0_0x0 == 0x6) && (iStack12 == 0)) {
        return;
    }
    uVar2 = &pstruct15_7[0x1].field_0x4;
    uVar6 = (uVar2 + 0x20);
    puVar10 = &local_16;
    uVar12 = SUB42(&DAT_1050_1050, 0x0);
    piStack26 = &local_18;
    uVar9 = SUB42(&DAT_1050_1050, 0x0);
    piVar8 = piStack26;
    uStack20 = uVar6;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6);
    uStack28 = uVar6;
    pass1_1030_5b1c(
        uVar6 & 0xffff | ZEXT24(piStack26) << 0x10,
        CONCAT22(uVar9, piVar8),
        CONCAT22(uVar12, puVar10),
    );
    pass1_1028_c8ee(
        param_2,
        (pstruct15_7 + 1).field0_0x0,
        CONCAT22(0x1050, &local_10),
    );
    pass1_1008_3eb4(
        CONCAT22(0x1050, &local_10),
        CONCAT22(0x1050, &local_22),
        CONCAT22(0x1050, local_20),
        CONCAT22(0x1050, local_1e),
    );
    if ((pstruct15_7 + 1).field0_0x0 == 1) {
        if (local_18 < local_22) {
            pass1_1030_5b3e(CONCAT22(piStack26, uStack28), local_22, local_16);
            pass1_1030_5b1c(
                CONCAT22(piStack26, uStack28),
                CONCAT22(0x1050, &local_18),
                CONCAT22(0x1050, &local_16),
            );
        }
        uStack50 = (uStack10 + 0x4);
        struct_op_1028_87f0(
            CONCAT22(0x1050, &local_158),
            0x0,
            0x0,
            0x6a,
            &local_10,
            &DAT_1050_1050,
            uStack50,
            uStack20,
        );
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &local_158));
        local_158 = 0x389a;
        uStack342 = 0x1008;
    }
    pass1_1028_ccd0(param_2, CONCAT22(0x1050, &local_10));
    return;
}


pub unsafe fn pass1_1020_e294(param_1: *mut astruct_15) {
    let mut uVar1: u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u32;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut pstruct15_5: *mut astruct_15;
    let mut uVar6: *mut astruct_15;
    let mut cStack347: u8;
    let mut local_150: [u8; 0xc] = [0; 0xc];
    let mut puStack324: *mut u32;
    let mut string_140: [u8; 0x124] = [0; 0x124];
    let mut iStack20: i16;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut uStack6: u32;

    uVar6 = (param_1 >> 0x10);
    pstruct15_5 = param_1;
    if ((0x1 < (pstruct15_5 + 1).field0_0x0) && ((pstruct15_5 + 1).field0_0x0 < 0x6)) {
        uVar1 = &pstruct15_5[0x1].field_0x4;
        uVar3 = (uVar1 + 0x20);
        uStack6 = uVar3;
        pass1_1028_b58e(param_1);
        iStack10 = uVar3;
        local_10 = (iStack10 + 0xc);
        uStack12 = (iStack10 + 0x10);
        puStack324 = &local_10;
        uVar4 = extraout_DX;
        uStack8 = extraout_DX;
        pass1_1028_c8ee(
            param_1,
            (pstruct15_5 + 1).field0_0x0,
            CONCAT22(0x1050, puStack324),
        );
        puVar2 = &local_10;
        pass1_1028_c89c(
            puVar2,
            param_1,
            CONCAT22(0x1050, puVar2),
            CONCAT22(0x1050, local_150),
        );
        uVar3 = *puVar2;
        cStack347 = (uVar3 >> 0x18);
        if ((cStack347 == '\0') && (iStack20 = uVar3, iStack20 == 0x9)) {
            pstruct15_5.field16_0x14 = 0x1;
        }
        uVar1 = (iStack10 + 0x2e);
        struct_op_1028_87f0(
            CONCAT22(0x1050, string_140),
            0x0,
            &pstruct15_5.field16_0x14 + 0x1,
            0x79,
            &local_10,
            &DAT_1050_1050,
            (uVar1 + 0x4),
            uStack6,
        );
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, string_140));
    }
    pstruct15_5[0x1].field1_0x2 = 0x1;
    return;
}

// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2

pub unsafe fn pass1_1020_e39c(mut param_1: i16, param_2: *mut astruct_15, mut param_3: u16) {
    let mut uVar3: u16;
    let mut extraout_DX: u16;
    let mut local_13c: [u8; 0x124] = [0; 0x124];
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut uStack6: u32;
    let mut uVar1: u32;
    let mut uVar2: u32;

    pass1_1028_b58e(param_2);
    uStack6 = CONCAT22(extraout_DX, param_1);
    local_c = (param_1 + 0xc);
    iStack8 = (param_1 + 0x10);
    if (iStack8 < 1) {
        uVar3 = 0x5;
    } else {
        uVar3 = 0x6;
    }
    (param_1 + 0x14) = uVar3;
    uVar2 = (param_2 + 0x28);
    uVar1 = (param_1 + 0x2e);
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_13c),
        0x0,
        0x1,
        param_3,
        &local_c,
        &DAT_1050_1050,
        (uVar1 + 0x4),
        (uVar2 + 0x20),
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_13c));
    return;
}

pub unsafe fn pass1_1020_e44c(param_1: u8, mut param_2: u16, mut param_3: u32) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_3 >> 0x10);
    iVar2 = param_3;
    if ((iVar2 + 0x12) == 0x2) {
        piVar1 = (iVar2 + 0x14);
        *piVar1 = *piVar1 - 0x1;
        if (((iVar2 + 0x26) == 0) && ((iVar2 + 0xc) == 0x78)) {
            pass1_1020_e294((param_3 & 0xffff | uVar3 << 0x10));
        }
        if ((iVar2 + 0x14) == 0) {
            pass1_1020_e08e(param_2, (param_3 & 0xffff | uVar3 << 0x10));
            return;
        }
        if ((iVar2 + 0x24) == 0x6) {
            (iVar2 + 0xe) = 0x49;
        }
    }
    return;
}

pub unsafe fn pass1_1020_e49a(param_1: *mut astruct_15) {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut uStack10: u16;

    uVar3 = pass1_1028_b58e(param_1);
    iVar1 = (uVar3 + 0x14);
    uStack10 = 0;
    iVar2 = iVar1 - 0x6;
    if (iVar2 == 0) {
        uStack10 = 0x9;
    } else {
        iVar2 = iVar1 - 0x7;
        if (iVar2 == 0) {
            uStack10 = 0x6;
        } else {
            iVar2 = iVar1 - 0x8;
            if (iVar2 == 0) {
                uStack10 = 0x7;
            } else {
                iVar2 = iVar1 - 0x9;
                if (iVar2 == 0) {
                    uStack10 = 0x8;
                }
            }
        }
    }
    pass1_1020_e39c(iVar2, param_1, uStack10);
    return;
}

pub unsafe fn pass1_1020_e4fa(param_1: *mut astruct_15, mut param_2: u16) -> i16 {
    let mut uVar1: u32;
    let mut iStack4: i16;

    match param_2 {
        0x2 | 0x5 | 0x6 | 0x7 => {
            iStack4 = 0x4;
        }

        0x3 | 0x8 => {
            iStack4 = 0x5;
        }

        _ => {
            uVar1 = pass1_1028_b58e(param_1);
            iStack4 = (uVar1 + 0x14) + 2;
        }
    }
    return iStack4;
}




pub unsafe fn pass1_1020_e652(
    mut param_1: u32,
    param_2: *mut u32,
    mut param_3: u16,
    param_4: i32,
) -> *mut u32 {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    local_8 = *param_2;
    uStack4 = (param_2 + 1);
    uVar2 = (param_1 >> 0x10);
    pass1_1028_c8ee(param_1, (param_1 + 0x24), CONCAT22(0x1050, &local_8));
    puVar1 = &local_8;
    pass1_1028_c7b6(param_3, param_1, uVar2, CONCAT22(0x1050, puVar1), param_4);
    if (puVar1.is_null() == false) {
        puVar1 = (&PTR_LOOP_1050_0000 + 1);
    }
    return puVar1;
}


pub unsafe fn struct_1020_e7fa(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0xe88e;
    (param_1 + 0x2) = 0x1020;
    return &param_1.field0_0x0;
}

pub unsafe fn pass1_1020_e81c(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0xe88e;
    (param_2 + 0x2) = 0x1020;
    return &param_2.field0_0x0;
}

pub unsafe fn pass1_1020_e846(param_1: *mut u16) {
    *param_1 = 0xe88e;
    (param_1 + 0x2) = 0x1020;
    pass1_1028_b418(param_1);
    return;
}


pub unsafe fn struct_1020_e8f6(param_1: *mut u16) -> *mut u16 {
    let mut uVar1: u16;

    struct_1030_dc96(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x24) = 0;
    *param_1 = 0xeef6;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

pub unsafe fn pass1_1020_e91e(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u32,
) -> *mut u16 {
    pass1_1030_dcc2(param_1, CONCAT22(param_3, param_2), param_4, param_5);
    (param_2 + 0x24) = 0;
    CONCAT22(param_3, param_2) = 0xeef6;
    (param_2 + 0x2) = 0x1020;
    return CONCAT22(param_3, param_2);
}











pub unsafe fn pass1_1020_ee3a(mut param_1: i16, param_2: *mut astruct_15, mut param_3: u16) {
    let mut uVar1: u32;
    let mut extraout_DX: u16;
    let mut uVar2: u32;
    let mut local_13c: [u8; 0x124] = [0; 0x124];
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b58e(param_2);
    local_c = (param_1 + 0xc);
    uStack8 = (param_1 + 0x10);
    iStack6 = param_1;
    uStack4 = extraout_DX;
    uVar2 = pass1_1028_bb24(param_2);
    uVar1 = (iStack6 + 0x2e);
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_13c),
        0x0,
        0x1,
        param_3,
        &local_c,
        &DAT_1050_1050,
        (uVar1 + 0x4),
        uVar2,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_13c));
    return;
}


pub unsafe fn pass1_1020_ef5e(param_1: *mut u16) {
    *param_1 = 0;
    (param_1 + 0x2) = 0x1028;
    pass1_1028_b418(param_1);
    return;
}
