use crate::unk::block_1020_d000::{pass1_1020_deac, pass1_1020_df10};
use crate::unk::block_1020_e000::{pass1_1020_e4fa, pass1_1020_e846, pass1_1020_ee3a, pass1_1020_ef5e};
use crate::unk::block_1028_0000::{pass1_1028_0138, pass1_1028_081e};

pub fn pass1_1020_d7d8(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn FUN_1020_d8b2() {
    return;
}

pub fn FUN_1020_d8b6() {
    return;
}

pub fn FUN_1020_d8ba() {
    return;
}

pub fn FUN_1020_d8be() {
    return;
}

pub fn FUN_1020_d8c2() {
    return;
}

pub fn FUN_1020_d8c6(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub fn pass1_1020_d9fa(mut param_1: u16, param_2: *mut astruct_15) {
    let mut extraout_DX: u16;

    if ((param_2 + 0xc) != 0x79) {
        pass1_1030_df0c(param_1, param_2);
        pass1_1028_b58e(param_2);
        pass1_1038_57dc((param_1 + 0x2e), 0x1, 0x2);
    }
    return;
}

pub fn pass1_1020_da3c(param_1: u32) {
    pass1_1028_bdac(param_1, 0x2);
    return;
}

pub fn pass1_1020_da4e(
    mut param_1: u16,
    param_2: *mut u32,
    param_3: *mut u16,
    mut param_4: u32,
    mut param_5: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut BVar4: bool;
    let mut extraout_DX: *mut u8;
    let mut puVar5: *mut u8;
    let mut extraout_DX_00: *mut u8;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut puVar10: *mut u16;
    let mut uVar12: u32;
    let mut bStack31: u8;
    let mut local_e: u32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u32;

    puVar2 = &local_e;
    pass1_1030_64ce(
        puVar2,
        param_1,
        _PTR_LOOP_1050_5740,
        param_3,
        param_5,
        CONCAT22(0x1050, puVar2),
    );
    uStack6 = *puVar2;
    uVar6 = (puVar2 + 2);
    bStack31 = (uStack6 >> 0x18);
    uVar3 = bStack31;
    if (bStack31 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack6 & 0xffff | uVar6 << 0x10);
        uVar7 = struct_op_1030_73a8(CONCAT22(uVar6, uVar3), uVar3, uVar6);
        uVar6 = (uVar7 >> 0x10);
        uVar3 = uVar7;
        if ((uVar3 + 0xc) == 0x10) {
            PTR_LOOP_1050_50ca = 0x6a9;
            return;
        }
    }
    uVar9 = param_2;
    uVar11 = (param_2 >> 0x10);
    pass1_1028_c7b6(uVar6, uVar9, uVar11, param_3, param_5);
    uVar8 = param_2 & 0xffff | uVar11 << 0x10;
    ppcVar1 = (*param_2 + 0x60);
    puVar10 = param_3;
    uVar7 = param_4;
    uVar12 = param_5;
    uStack8 = uVar3;
    (**ppcVar1)();
    if (((uVar3 != 0) && (
        puVar5 = extraout_DX,
        pass1_1028_c23e(uVar3, extraout_DX, uVar9, uVar11, param_3, param_4, param_5),
        uVar3 != 0x0,
    )) && (
        BVar4 = pass1_1028_c314(
            uVar3,
            puVar5,
            uVar9,
            uVar11,
            param_3,
            param_4,
            (param_4 >> 0x10),
            param_5,
        ),
        BVar4 != 0,
    )) {
        uVar6 = (param_3 >> 0x10);
        if ((((param_3 + 0x4) == 0) && (uStack8 != 0x4)) && (
            ppcVar1 = (*param_2 + 0x5c),
            (**ppcVar1)(
                0x1028, param_2, param_3, param_4, param_5, uVar8, puVar10, uVar7, uVar12,
            ),
            puVar5 = extraout_DX_00,
            BVar4 == 0,
        )) {
            return;
        }
        uStack10 = (param_3 + 0x4);
        if (uStack10 != 0) {
            pass1_1020_df10(
                uStack10,
                puVar5,
                param_2,
                (param_3 & 0xffff | uVar6 << 0x10),
                param_5,
            );
            return;
        }
        pass1_1020_deac(
            0x0,
            puVar5,
            param_2,
            (param_3 & 0xffff | uVar6 << 0x10),
            param_5,
        );
        return;
    }
    return;
}

pub fn pass1_1020_db86(
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
    let mut puVar6: *mut u16;
    let mut local_4: [u8; 0x2] = [0; 0x2];

    uVar5 = pass1_1030_bcae(local_4, 0x1050);
    uVar4 = (uVar5 >> 0x10);
    iVar1 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4);
    uVar3 = (iVar1 + 0x10);
    puVar6 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3);
    puVar2 = local_4;
    pass1_1030_bcde(
        puVar2,
        0x1050,
        uVar3 & 0xffff | uVar4 << 0x10,
        puVar6,
        param_5,
    );
    if (puVar2 < 0x0) {
        PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if ((puVar2 < 0x1f) || ((param_3 + 0x4) < 1)) {
            return;
        }
        PTR_LOOP_1050_50ca = 0x6b6;
        PTR_LOOP_1050_50cc = puVar2 - 0x1e;
    }
    return;
}

pub fn pass1_1020_e558(mut param_1: i16, param_2: *mut astruct_15) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut bStack45: u8;
    let mut local_24: [u8; 0xc] = [0; 0xc];
    let mut uStack24: u32;
    let mut uStack20: u32;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    uVar7 = (param_2 >> 0x10);
    iVar6 = param_2;
    if ((iVar6 + 0xc) == 0x79) {
        param_1 = (iVar6 + 0x24);
        (iVar6 + 0x14) = param_1;
        (iVar6 + 0x24) = 0;
    }
    if ((iVar6 + 0x24) != 0x6) {
        pass1_1028_b58e(param_2);
        uStack8 = (param_1 + 0x14);
        iStack6 = param_1;
        uStack4 = extraout_DX;
        iStack10 = pass1_1020_e4fa(param_2, uStack8);
        local_10 = (iStack6 + 0xc);
        uStack12 = (iStack6 + 0x10);
        uStack24 = CONCAT22(uStack24, &local_10);
        uVar4 = uStack4;
        pass1_1028_c8ee(param_2, (iVar6 + 0x24), CONCAT22(0x1050, &local_10));
        puVar1 = &local_10;
        pass1_1028_c89c(
            puVar1,
            param_2,
            CONCAT22(0x1050, puVar1),
            CONCAT22(0x1050, local_24),
        );
        uStack24 = *puVar1;
        uVar5 = (puVar1 + 2);
        bStack45 = (uStack24 >> 0x18);
        uVar2 = bStack45;
        uStack20 = uStack24;
        uStack20 = uStack24;
        if (bStack45 != 0) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack24 & 0xffff | uVar5 << 0x10);
            uStack20 = (uVar2 + 0x14);
        }
        uStack8 = uStack20;
        iVar3 = pass1_1020_e4fa(param_2, uStack20);
        (iVar6 + 0x14) = iStack10 + iVar3;
        return;
    }
    (iVar6 + 0x14) = 0x1;
    return;
}


pub fn write_to_file_1020_e6a4(mut param_1: u32, param_2: *mut u8) -> BOOL16 {
    let mut in_AX: i16;
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut in_stack_0000ffdc: HFILE16;
    let mut local_c: [u16; 0x3] = [0; 0x3];
    let mut local_6: [u16; 0x2] = [0; 0x2];

    pass1_1030_de7c(param_1, param_2);
    if (in_AX != 0) {
        uVar2 = (param_1 >> 0x10);
        local_c[0] = (param_1 + 0x24);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffdc);
        if (BVar1 != 0) {
            local_6[0] = (param_1 + 0x26);
            BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffdc);
            if (BVar1 != 0) {
                return 0x1;
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn pass1_1020_e70e(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut BVar1: bool;

    pass1_1030_dec4(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar1 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x24)), 0x2);
        if (BVar1 != 0) {
            BVar1 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x26)), 0x2);
            if (BVar1 != 0) {
                return;
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn pass1_1020_e76c(
    mut param_1: u16,
    param_2: *mut StructD,
    param_3: u8,
) -> *mut StructD {
    pass1_1030_dcf4(param_1, param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub fn FUN_1020_e864() {
    return;
}

pub fn pass1_1020_e868(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_e846(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_e94e(param_1: BOOL16, mut param_2: u32, mut param_3: u32) -> BOOL16 {
    let mut BVar1: bool;
    let mut in_stack_0000ffde: HFILE16;
    let mut local_c: [u16; 0x5] = [0; 0x5];

    pass1_1030_de7c(param_2, param_3);
    if (param_1 != 0) {
        local_c[0] = (param_2 + 0x24);
        BVar1 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffde);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        param_1 = 0x1;
    }
    return param_1;
}

pub fn pass1_1020_e994(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut BVar1: bool;

    pass1_1030_dec4(param_1, param_2, param_3, param_4);
    if ((param_1 != 0) && (
        BVar1 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x24)), 0x2),
        BVar1 == 0,
    )) {
        u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn pass1_1020_e9d4(mut param_1: u16, param_2: *mut astruct_15) {
    let mut extraout_DX: u16;

    pass1_1030_df0c(param_1, param_2);
    pass1_1028_b58e(param_2);
    pass1_1038_57dc((param_1 + 0x2e), 0x1, 1);
    return;
}

pub fn pass1_1020_ea0e(param_1: u32) {
    pass1_1028_bdac(param_1, 1);
    return;
}

pub fn pass1_1020_ea20(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
    mut param_5: u32,
    mut param_6: u32,
) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut cVar3: u8;
    let mut puVar4: *mut u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut uVar9: u32;
    let mut uVar10: u32;
    let mut uVar11: u16;
    let mut in_stack_0000fd5e: u16;
    let mut in_stack_0000fe82: u16;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000fe8c: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut iVar14: i16;
    let mut local_146: [u8; 0x10c] = [0; 0x10c];
    let mut uStack58: u16;
    let mut uStack56: u16;
    let mut paStack50: *mut astruct_419;
    let mut puStack46: *mut u32;
    let mut puStack42: *mut u32;
    let mut uStack38: u32;
    let mut uStack34: u32;
    let mut uStack28: u32;
    let mut local_12: u32;
    let mut iStack14: i16;
    let mut puStack12: *mut u32;
    let mut uStack8: u32;
    let mut BStack4: bool;

    uVar12 = param_3;
    uVar13 = (param_3 >> 0x10);
    pass1_1028_c3aa(uVar12, uVar13, param_4, param_5, param_6);
    if (param_1 == 0) {
        return;
    }
    pass1_1028_c23e(param_1, param_2, uVar12, uVar13, param_4, param_5, param_6);
    if (param_1 == 0) {
        return;
    }
    BStack4 = pass1_1028_c314(
        param_1,
        param_2,
        uVar12,
        uVar13,
        param_4,
        param_5,
        (param_5 >> 0x10),
        param_6,
    );
    if (BStack4 == 0) {
        return;
    }
    pass1_1028_c7b6(param_2, uVar12, uVar13, param_4, param_6);
    if ((((BStack4 == 0x5) || (BStack4 == 0x4)) || (BStack4 == 0x6)) || (BStack4 == 0x9)) {
        PTR_LOOP_1050_50ca = 0x6a8;
        return;
    }
    if (BStack4 != 0) {
        return;
    }
    puVar4 = &local_12;
    pass1_1030_64ce(
        puVar4,
        param_2,
        _PTR_LOOP_1050_5740,
        param_4,
        param_6,
        CONCAT22(0x1050, puVar4),
    );
    uStack38 = *puVar4;
    uVar1 = (puVar4 + 2);
    paVar8 = CONCAT22(in_register_0000000a, uVar1);
    uStack38._3_1_ = (uStack38 >> 0x18);
    uStack58 = uStack38._3_1_;
    uStack28 = uStack38;
    uStack8 = uStack38;
    //  if (uStack38._3_1_ == 0) goto LAB_1020_eb4e;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack38 & 0xffff | uVar1 << 0x10);
    uVar11 = SUB42(paVar8, 0x0);
    uStack38 = CONCAT22(uVar11, uStack58);
    uStack34 = (uStack58 + 0x2e);
    if ((uStack34 + 0x4) != param_5) {
        PTR_LOOP_1050_50ca = 0x6b7;
        return;
    }
    uStack28 = struct_op_1030_73a8(CONCAT22(uVar11, uStack58), param_5, uVar11);
    paVar8 = (paVar8 & 0xffff0000 | uStack28 >> 0x10);
    uVar11 = (uStack28 >> 0x10);
    uVar1 = (uStack28 + 0xc);
    uStack58 = uVar1;
    if (uVar1 != 0x41) {
        if (0x41 < uVar1) {
            if (uVar1 == 0x6b) {
                PTR_LOOP_1050_50ca = 0x6b1;
                return;
            }
            if (uVar1 < 0x6c) {
                if (uVar1 == 0x42) {
                    PTR_LOOP_1050_50ca = 0x6b1;
                    return;
                }
                uStack58 = uVar1 - 0x4b;
                if (uStack58 == 0) {
                    PTR_LOOP_1050_50ca = 0x6b1;
                    return;
                }
            } else {
                if (uVar1 == 0x6e) {
                    return;
                }
                uStack58 = uVar1 - 0x73;
                if ((0x4 < (uVar1 - 0x6e)) && (
                    uStack58 = uVar1 - 0x79,
                    uStack58 == 0x0 || (uVar1 - 0x73) < 0x6,
                )) {
                    PTR_LOOP_1050_50ca = 0x6b0;
                    return;
                }
            }
            // TODO: goto LAB_1020_eb4e;
        }
        if (uVar1 != 0x3e) {
            if (uVar1 < 0x3f) {
                cVar3 = uVar1;
                //         if (cVar3 != '\v') {
                //           if (cVar3 == '\x10') {
                //             return;
                //           }
                //           uStack58 = uVar1 & 0xff00 | (cVar3 - 0x37U);
                // //          if ((cVar3 - 0x37U) != 0) goto LAB_1020_eb4e;
                //         }
                PTR_LOOP_1050_50ca = 0x6b4;
                return;
            }
            // TODO: goto LAB_1020_eb4e;
        }
    }
    if ((uStack28 + 0x12) == 0x4) {
        PTR_LOOP_1050_50ca = 0x6b1;
        return;
    } //
    // LAB_1020_eb4e:
    uVar11 = 0x1000;
    mem_op_1000_179c(0xb4, paVar8);
    uStack56 = paVar8;
    uVar10 = paVar8 & 0xffff0000;
    uVar9 = uVar10 | (uStack56 | uStack58);
    if ((uStack56 | uStack58) == 0) {
        iStack14 = 0;
    } else {
        uVar11 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        iStack14 = string_1040_8520(
            uVar9,
            CONCAT13((paVar8 >> 0x8), CONCAT12(paVar8, uStack58)),
            HWND16_1050_0396,
            0x20024,
            0x5e8057b,
        );
        uVar10 = uVar9;
    }
    puStack12 = CONCAT22(uVar10, iStack14);
    ppcVar2 = (*puStack12 + 0x74);
    iVar14 = iStack14;
    (**ppcVar2)(uVar11, iStack14, uVar10);
    if (iStack14 != 0x7) {
        paVar8 = (uVar10 & 0xffff0000 | uStack8 >> 0x10);
        puStack46 = uStack8;
        uStack34 = uStack8;
        uStack34._3_1_ = (uStack8 >> 0x18);
        uVar5 = uStack34._3_1_;
        if (uStack34._3_1_ != 0) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack8);
            paStack50 = CONCAT22(paVar8, uVar5);
            fn_ptr_1030_7296(CONCAT22(paVar8, uVar5));
            pass1_1030_730a(uVar5, paStack50);
            puStack46 = mixed_1010_20ba(
                paVar8,
                _u16_1050_0ed0,
                CONCAT22(iVar14, 0x2f),
                in_stack_0000fd5e,
                in_stack_0000fe82,
                in_stack_0000fe88,
                in_stack_0000fe8c,
            );
            uVar6 = (puStack46 >> 0x10);
            uVar11 = SUB42(puStack46, 0x0);
            pass1_1010_ec68(puStack46, paStack50);
            puStack42 = struct_op_1030_73a8(paStack50, uVar11, uVar6);
            puVar7 = (puStack42 >> 0x10);
            puVar4 = puStack42;
            ppcVar2 = (*puStack42 + 0x24);
            (**ppcVar2)(0x1030, puStack42, puVar7);
            uVar5 = pass1_1028_bc4a(puVar4, puVar7, puStack42);
            (uVar12 + 0x24) = uVar5;
            struct_1030_e4fa(CONCAT22(0x1050, local_146), (paStack50 + 0x16));
            fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_146));
        }
        return;
    }
    PTR_LOOP_1050_50ca = (&PTR_LOOP_1050_0000 + 1);
    return;
}

pub fn pass1_1020_ecb0(mut param_1: i16, mut param_2: u16, mut param_3: u32) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uStack8: u16;

    uVar2 = (param_3 >> 0x10);
    iVar1 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar1 + 0x8));
    if ((iVar1 + 0x12) == 1) {
        match (param_1 + 0x14) {
            0x2 | 0x7 => {
                uStack8 = 0x2;
            }
            //   break;
            0x3 | 0x8 => {
                uStack8 = 0x3;
            }
            //   break;
            _ => {
                uStack8 = (param_1 + 0x14);
            }
            //   break;
            0x5 | 0x6 => {
                uStack8 = 0x1;
            }
        }
        (iVar1 + 0x14) = uStack8;
        return;
    }
    pass1_1028_bf22(param_2, param_3 & 0xffff | uVar2 << 0x10);
    return;
}

pub fn pass1_1020_ed3c(mut param_1: i16, param_2: *mut astruct_15) {
    StructD * *ppSVar1;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut pstruct15_5_1: *mut astruct_15;
    let mut pstruct15_5: *mut astruct_15;
    let mut local_138: [u8; 0x112] = [0; 0x112];
    let mut local_12: i16;
    let mut local_10: [u8; 0x2] = [0; 0x2];
    let mut local_e: [u8; 0x2] = [0; 0x2];
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pstruct15_5 = (param_2 >> 0x10);
    pstruct15_5_1 = param_2;
    ppSVar1 = &pstruct15_5_1.field16_0x14;
    ppSVar1 = ppSVar1 - 0x1;
    if (ppSVar1 == 0) {
        pstruct15_5_1.field15_0x12 = 0;
        pass1_1028_b58e(param_2);
        local_c = (param_1 + 0xc);
        uStack8 = (param_1 + 0x10);
        iStack6 = param_1;
        uStack4 = extraout_DX;
        pass1_1008_3eb4(
            CONCAT22(0x1050, &local_c),
            CONCAT22(0x1050, &local_12),
            CONCAT22(0x1050, local_10),
            CONCAT22(0x1050, local_e),
        );
        if (local_12 < 1) {
            uVar3 = 0x5;
        } else {
            uVar3 = 0x6;
        }
        (iStack6 + 0x14) = uVar3;
        if (local_12 < 1) {
            iVar4 = 0x5;
        } else {
            iVar4 = 0x9;
        }
        pass1_1020_ee3a(iVar4, param_2, iVar4);
        pass1_1028_b58e(param_2);
        uVar2 = (iVar4 + 0x2e);
        pass1_1038_5804(uVar2, 0x1, 1);
        if (0x0 < (pstruct15_5_1 + 1).field0_0x0) {
            pass1_1028_68de(
                CONCAT22(0x1050, local_138),
                (pstruct15_5_1 + 1).field0_0x0,
                (uVar2 + 0x4),
            );
            fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_138));
        }
    }
    return;
}

pub fn pass1_1020_eed0(
    mut param_1: u16,
    param_2: *mut StructD,
    param_3: u8,
) -> *mut StructD {
    pass1_1030_dcf4(param_1, param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub fn FUN_1020_ef7c() {
    return;
}

pub fn FUN_1020_ef80() {
    return;
}

pub fn FUN_1020_ef84() {
    return;
}

pub fn FUN_1020_ef88() {
    return;
}

pub fn FUN_1020_ef8c() {
    return;
}

pub fn FUN_1020_ef90() {
    return;
}

pub fn pass1_1020_ef94(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_ef5e(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0176(mut param_1: u16, param_2: *mut astruct_15, mut param_3: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_EDX: u32;
    let mut uVar7: u16;
    let mut iVar9: *mut astruct_15;
    let mut uVar8: *mut astruct_15;
    let mut iVar8: *mut astruct_298;
    let mut paVar6: *mut Struct57;

    uVar3 = (in_EDX >> 0x10);
    iVar9 = param_2;
    uVar8 = (param_2 >> 0x10);
    pass1_1028_b46e(param_1, param_2, param_3);
    puVar2 = iVar9.field25_0x22;
    uVar5 = (iVar9 + 1).field0_0x0;
    uVar4 = uVar5 | puVar2;
    paVar6 = CONCAT22(uVar3, uVar4);
    if (uVar4 != 0) {
        ppcVar1 = *puVar2;
        puVar2 = (**ppcVar1)();
    }
    mem_op_1000_179c(0xc, paVar6);
    uVar5 = paVar6 | puVar2;
    if (uVar5 == 0) {
        uVar4 = 0;
        uVar5 = 0;
    } else {
        uVar4 = set_struct_1008_574a(CONCAT22(paVar6, puVar2));
    }
    iVar9.field25_0x22 = uVar4;
    (iVar9 + 1).field0_0x0 = uVar5;
    uVar7 = 0x14;
    uVar3 = pass1_1028_b58e(param_2);
    pass1_1030_7f1a(CONCAT22(uVar5, uVar3), uVar7);
    return;
}

pub fn pass1_1028_01ec(param_1: u32)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x12) == 0x6) || ((iVar2 + 0x12) == 0x5)) {
        uVar1 = (iVar2 + 0x14);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        if (((iVar2 + 0xa6) == 0x14) || ((iVar2 + 0xa8) == 0x10)) {
            pass1_1028_bdac(param_1, 0x6);
            return;
        }
        pass1_1028_be2a(param_1);
    }
    return;
}


pub fn write_to_file_1028_0234(mut param_1: u32, mut param_2: u32) -> u16

{
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_stack_0000ffba: HFILE16;
    let mut local_1a: [u16; 0x3] = [0; 0x3];
    let mut local_14: [u16; 0x2] = [0; 0x2];
    let mut uStack16: u16;
    let mut lStack14: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar2 != 0) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        local_1a[0] = (iVar3 + 0x20);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_1a), 0x2, in_stack_0000ffba);
        if (BVar2 != 0) {
            pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar3 + 0x22));
            uVar1 = (iVar3 + 0x22);
            local_14[0] = (uVar1 + 0x8);
            uStack16 = local_14[0];
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_14), 0x2, in_stack_0000ffba);
            while (BVar2 != 0) {
                lStack14 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
                if (lStack14 == 0) {
                    return 0x1;
                }
                local_14[0] = (lStack14 + 0x4);
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_14), 0x2, in_stack_0000ffba);
                if (BVar2 == 0) { break; }
                local_14[0] = (lStack14 + 0x6);
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_14), 0x2, in_stack_0000ffba);
                if (BVar2 == 0) { break; }
                local_14[0] = (lStack14 + 0x8);
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_14), 0x2, in_stack_0000ffba);
                if (BVar2 == 0) { break; }
                local_14[0] = (lStack14 + 0xa);
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_14), 0x2, in_stack_0000ffba);
                if (BVar2 == 0) { break; }
                local_14[0] = (lStack14 + 0xc);
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_14), 0x2, in_stack_0000ffba);
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}
