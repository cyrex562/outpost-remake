use crate::block_1028::block_1028_3000::pass1_1028_388e;
use crate::block_1028::block_1028_4000::{pass1_1028_4530, pass1_1028_478a};

pub unsafe fn pass1_1028_2e84(
    param_1: *mut StructD,
    param_2: *mut astruct_15,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut paVar1: *mut Struct57;
    let mut paVar2: *mut astruct_67;
    let mut paVar3: *mut Struct27;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000fe90: u16;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut iVar11: i16;

    pass1_1028_09b8(param_2);
    if (param_4 == 0) {
        uVar10 = 0;
        iVar11 = 0x8;
        uVar8 = 0x1;
        uVar9 = 0;
        uVar6 = 0;
        iVar7 = 0;
        uVar5 = 0;
        paVar2 = mixed_1010_20ba(
            param_1,
            _u16_1050_0ed0,
            0x37,
            in_stack_0000fe88,
            in_stack_0000ffac,
            in_stack_0000ffb2,
            in_stack_0000ffb6,
        );
        paVar1 = (param_1 & 0xffff0000 | paVar2 >> 0x10);
        post_win_msg_1008_a0e4(
            paVar2,
            CONCAT22(uVar6, uVar5),
            iVar7,
            uVar8,
            CONCAT22(uVar10, uVar9),
            iVar11,
        );
        uVar6 = 0x400;
        iVar7 = 0x3;
        uVar5 = 0x1;
        paVar3 = mixed_1010_20ba(
            paVar1,
            _u16_1050_0ed0,
            0x1002b,
            in_stack_0000fe90,
            in_stack_0000ffb4,
            in_stack_0000ffba,
            in_stack_0000ffbe,
        );
        paVar1 = (paVar1 & 0xffff0000 | paVar3 >> 0x10);
        pass1_1010_043a(paVar3, CONCAT22(uVar6, uVar5), iVar7);
        pass1_1010_043a(paVar3, 0x4000001, 0x4);
        puVar4 = mixed_1010_20ba(
            paVar1,
            _u16_1050_0ed0,
            CONCAT22(param_3, 0x2f),
            in_stack_0000fe96,
            in_stack_0000ffba,
            in_stack_0000ffc0,
            in_stack_0000ffc4,
        );
        paVar1 = (paVar1 & 0xffff0000 | puVar4 >> 0x10);
        pass1_1010_ec84(puVar4);
        puVar4 = mixed_1010_20ba(
            paVar1,
            _u16_1050_0ed0,
            CONCAT22(param_3, 0x8),
            in_stack_0000fe96,
            in_stack_0000ffba,
            in_stack_0000ffc0,
            in_stack_0000ffc4,
        );
        pass1_1010_9766((puVar4 >> 0x10), puVar4);
    }
    return;
}

pub unsafe fn pass1_1028_33f6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_34d0(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_0138(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_35aa() -> u16 {
    return 0x1;
}

pub unsafe fn pass1_1028_35b0(param_1: *mut astruct_15, mut param_2: i16) {
    let mut paVar1: *mut astruct_397;
    let mut uVar2: u16;

    paVar1 = pass1_1028_b58e(param_1);
    if (param_2 == 0) {
        uVar2 = 0;
    } else {
        uVar2 = 0x32;
    }
    pass1_1030_7d1c(paVar1, (paVar1 >> 0x10), paVar1, uVar2, 0x230000);
    return;
}


pub unsafe fn pass1_1028_35e2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1028_36bc(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u32,
) -> u16 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut iStack4: i16;

    uVar5 = CONCAT22(param_2, param_1);
    *param_4 = 0;
    uVar4 = (param_3 >> 0x10);
    if ((param_3 + 0x28) != 0) {
        iStack4 = 0x4;
        loop {
            if (0x1c < iStack4) {
                break;
            }
            uVar3 = (param_3 + 0x28);
            uVar5 = pass1_1020_bae6(
                uVar5,
                (uVar5 >> 0x10),
                uVar3,
                CONCAT22(iStack4, (uVar3 >> 0x10)),
            );
            uVar2 = param_4;
            param_4 = param_4 + uVar5;
            piVar1 = (param_4 + 2);
            *piVar1 = *piVar1 + (uVar5 >> 0x10) + CARRY2(uVar2, uVar5);
            if (0xf9 < *param_4) {
                return 0x1;
            }
            iStack4 += 0x1;
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1028_3718(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_388e(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1028_38d4(
    mut param_1: i16,
    mut param_2: u16,
    param_3: *mut u32,
    param_4: *mut u16,
    mut param_5: u32,
    mut param_6: u32,
) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;

    uVar4 = param_3;
    uVar5 = (param_3 >> 0x10);
    pass1_1028_c7b6(param_2, uVar4, uVar5, param_4, param_6);
    if ((param_1 == 0x5) || (param_1 == 0x6)) {
        ppcVar1 = (*param_3 + 0x60);
        uVar3 = (**ppcVar1)();
        if (uVar3 != 0) {
            pass1_1028_c23e(
                uVar3,
                (uVar3 >> 0x10),
                uVar4,
                uVar5,
                param_4,
                param_5,
                param_6,
            );
            if (uVar3 != 0) {
                BVar2 = pass1_1028_c314(
                    uVar3,
                    (uVar3 >> 0x10),
                    uVar4,
                    uVar5,
                    param_4,
                    param_5,
                    (param_5 >> 0x10),
                    param_6,
                );
                if (BVar2 != 0) {
                    return 0x1;
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0x0;
}

pub unsafe fn pass1_1028_3958(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15) {
    let mut plVar1: *mut i32;
    let mut iVar2: i16;
    let mut qqVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uStack52: u32;
    let mut local_2c: [u16; 0x2] = [0; 0x2];
    let mut local_28: u32;
    let mut iStack36: i16;
    let mut uStack34: u32;
    let mut uStack30: u32;
    let mut uStack22: u16;
    let mut uStack20: u16;
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut puStack10: *mut u32;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b58e(param_3);
    puStack10 = (param_1 + 0x22);
    uVar6 = (param_1 + 0x24);
    uVar7 = uVar6;
    if ((uVar6 | puStack10) != 0) {
        iStack6 = param_1;
        uStack4 = param_2;
        if (u16_1050_574c != 0) {
            uStack30 = (puStack10 + 0x4);
            for uStack34 in 0..uStack30 {
                pass1_1020_bb16(
                    puStack10,
                    CONCAT22(0x1050, local_2c),
                    CONCAT22(0x1050, &local_28),
                    uStack34,
                );
            }
        }
        uStack14 = (iStack6 + 0x2e);
        uStack18 = *_PTR_LOOP_1050_65e2;
        uStack20 = uStack18 & 0x1;
        // for (uStack22 = 0x4; uStack22 < 0xe; uStack22 += 1)
        for uStack22 in 4..0xe {
            local_2c[0] = uStack22;
            local_28 = pass1_1020_bae6(
                uStack22,
                uVar7,
                puStack10,
                CONCAT22(uStack22, (puStack10 >> 0x10)),
            );
            uVar6 = (local_28 >> 0x10) | local_28;
            uVar7 = uVar6;
            if (uVar6 != 0) {
                pass1_1020_bb8a(puStack10, 0x0, local_2c[0] << 0x10);
                uVar6 = -(local_28 + (local_28 != 0));
                uVar7 = uVar6;
                uStack34 = CONCAT22(uVar6, -local_28);
                pass1_1038_5694(uStack14, CONCAT22(uVar6, -local_28), local_2c[0]);
                uStack30 = 0;
                iStack36 = 0;
                iVar8 = param_3;
                uVar9 = (param_3 >> 0x10);
                match uStack22 {
                    0x4 => {
                        uStack30 = local_28 >> 0x1;
                        if ((uStack30 == 0) && (uStack20 != 0)) {
                            uStack30 = 0x1;
                        }
                        iStack36 = 0x11;
                    }
                    //   break;
                    0x5 => {
                        uStack30 = local_28 >> 0x1;
                        if ((uStack30 == 0) && (uStack20 != 0)) {
                            uStack30 = 0x1;
                        }
                        iStack36 = 0x12;
                    }
                    //   break;
                    0x6 => {
                        uStack30 = local_28 >> 0x1;
                        if ((uStack30 == 0) && (uStack20 != 0)) {
                            uStack30 = 0x1;
                        }
                        iStack36 = 0x13;
                    }
                    //   break;
                    0x7 => {
                        uStack30 = local_28 >> 0x1;
                        if ((uStack30 == 0) && (uStack20 != 0)) {
                            uStack30 = 0x1;
                        }
                        iStack36 = 0x14;
                    }
                    //   break;
                    0x8 => {
                        uStack30 = local_28;
                        iStack36 = 0x1a;
                    }
                    //   break;
                    0x9 => {
                        uStack30 = local_28;
                        iStack36 = 0x1b;
                    }
                    //   break;
                    0xa => {
                        uStack30 = local_28;
                        iStack36 = 0x1c;
                    }
                    //   break;
                    0xb => {
                        uStack30 = local_28;
                        iStack36 = 0x17;
                    }
                    //   break;
                    0xc => {
                        iStack36 = 0x18;
                        uStack30 = local_28;
                        plVar1 = (iVar8 + 0x20);
                        *plVar1 = *plVar1 + local_28;
                        uVar6 = (iVar8 + 0x20);
                        uVar4 = (iVar8 + 0x22);
                        uVar5 = uVar6 >> 0x1 | ((uVar4 & 1) != 0) << 0xf;
                        uStack52 = CONCAT22(uVar4 >> 0x1, uVar5);
                        uVar5 = (uVar4 & 0xfffe) + CARRY2(uVar5, uVar5);
                        uVar7 = uVar5;
                        (iVar8 + 0x20) = uVar6 - (uVar6 & 0xfffe);
                        (iVar8 + 0x22) = (uVar4 - uVar5) - (uVar6 < (uVar6 & 0xfffe));
                        if (uStack52 != 0) {
                            uVar10 = 0x15; //
                            // LAB_1028_3b14:
                            uStack30 = local_28;
                            pass1_1020_bb8a(
                                (iVar8 + 0x28),
                                uStack52,
                                CONCAT22(uVar10, (uStack52 >> 0x10)),
                            );
                        }
                    }
                    //   break;
                    0xd => {
                        iStack36 = 0x19;
                        uStack30 = local_28;
                        plVar1 = (iVar8 + 0x24);
                        *plVar1 = *plVar1 + local_28;
                        uVar6 = (iVar8 + 0x24);
                        iVar2 = (iVar8 + 0x26);
                        qVar3 = (local_28 & 0xffff0000 | uVar6) / 0x3;
                        uStack52 = qVar3;
                        uStack52 = (qVar3 >> 0x10);
                        uVar4 = qVar3;
                        uVar5 = uStack52 * 0x3 + CARRY2(uVar4, uVar4) + CARRY2(uVar4 * 0x2, uVar4);
                        uVar7 = uVar5;
                        (iVar8 + 0x24) = uVar6 + uVar4 * -0x3;
                        (iVar8 + 0x26) = (iVar2 - uVar5) - (uVar6 < uVar4 * 0x3);
                        if (uStack52 != 0) {
                            uVar10 = 0x16;
                            // TODO: goto LAB_1028_3b14;
                        }
                    }
                };
                if (((uStack30 | uStack30) != 0) && (iStack36 != 0)) {
                    pass1_1020_bb70((iVar8 + 0x28), uStack30, CONCAT22(iStack36, uStack30));
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1028_3c60(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u32,
) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_10: i32;
    let mut local_c: [u8; 0x4] = [0; 0x4];
    let mut iStack8: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    uVar6 = CONCAT22(param_2, param_1);
    *param_4 = 0;
    uVar5 = (param_3 >> 0x10);
    iVar4 = param_3;
    if ((iVar4 + 0x28) != 0) {
        iStack8 = 0x4;
        loop {
            if (0x1c < iStack8) {
                break;
            }
            uVar3 = (iVar4 + 0x28);
            uVar6 = pass1_1020_bae6(
                uVar6,
                (uVar6 >> 0x10),
                uVar3,
                CONCAT22(iStack8, (uVar3 >> 0x10)),
            );
            uVar2 = param_4;
            param_4 = param_4 + uVar6;
            piVar1 = (param_4 + 2);
            *piVar1 = *piVar1 + (uVar6 >> 0x10) + CARRY2(uVar2, uVar6);
            if (0x3e7 < *param_4) {
                return;
            }
            iStack8 += 0x1;
        }
    }
    uVar6 = (iVar4 + 0x28);
    uStack4 = (uVar6 + 0x4);
    uStack6 = 0;
    loop {
        if (uStack4 <= uStack6) {
            return;
        }
        pass1_1020_bb16(
            (iVar4 + 0x28),
            CONCAT22(0x1050, &local_10),
            CONCAT22(0x1050, local_c),
            uStack6,
        );
        *param_4 = *param_4 + local_10;
        if (0x3e7 < *param_4) {
            break;
        }
        uStack6 += 0x1;
    }
    return;
}


pub unsafe fn write_to_file_1028_3d0e(
    mut param_1: u16,
    param_2: *mut astruct_731,
    param_3: *mut u8,
) {
    let mut BVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_stack_0000ffd8: HFILE16;
    let mut local_10: [u32; 0x2] = [0; 0x2];
    let mut local_8: u32;

    BVar1 = write_to_file_1028_b5ec(param_2, param_3);
    if (BVar1 != 0) {
        uVar3 = (param_2 >> 0x10);
        iVar2 = param_2;
        local_10[0] = (iVar2 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_10), 0x4, in_stack_0000ffd8);
        if (BVar1 != 0) {
            local_8 = (iVar2 + 0x24);
            BVar1 = write_to_file_1008_7e1c(
                param_3,
                CONCAT22(0x1050, &local_8),
                0x4,
                in_stack_0000ffd8,
            );
            if (BVar1 != 0) {
                write_to_file_1008_7a22(param_3, (iVar2 + 0x28));
                if (BVar1 != 0) {
                    return;
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}


pub unsafe fn pass1_1028_3d92(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut iVar1: i16;
    let mut BVar2: bool;
    let mut uVar3: u16;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        iVar1 = param_3;
        BVar2 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar1 + 0x20)), 0x4);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar1 + 0x24)), 0x4);
            if (BVar2 != 0) {
                uVar3 = pass1_1008_7ad4(param_4, (iVar1 + 0x28));
                if (uVar3 != 0) {
                    return;
                }
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}


pub unsafe fn pass1_1028_3e06(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_388e(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1028_3f04(mut param_1: u16, mut param_2: u16, param_3: *mut astruct_15) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut paStack6: *mut astruct_397;

    uVar6 = 0x1f;
    pass1_1028_b58e(param_3);
    paStack6 = CONCAT22(param_2, param_1);
    uStack10 = pass1_1030_7c28(param_1, param_2, CONCAT22(param_2, param_1), uVar6);
    uVar5 = (uStack10 >> 0x10);
    paVar3 = CONCAT22(in_register_0000000a, uVar5);
    uVar2 = uStack10 & 0xffff;
    pass1_1030_7d1c(uVar2, uVar5, paStack6, 0x0, 0x1f0000);
    uVar5 = (param_3 >> 0x10);
    iVar4 = param_3;
    if ((iVar4 + 0xc) != 0x22) {
        if ((iVar4 + 0xc) == 0x23) {
            uVar1 = 0xa;
        } else {
            uVar1 = 0x5;
        }
        uStack14 = uVar1;
        uStack10 += (iVar4 + 0x20);
        (iVar4 + 0x20) = uStack10 % uVar1;
        uVar2 = uStack10 / uStack14;
        paVar3 = (uStack10 % uStack14);
        uStack10 += uVar2;
    }
    pass1_1030_7ddc(uVar2, paVar3, paStack6, uStack10, 0x21);
    return;
}

pub unsafe fn pass1_1028_3fde(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_4194(param_1: *mut astruct_15) {
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut paVar4: *mut Struct27;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffea: u16;

    uVar2 = (in_EDX >> 0x10);
    pass1_1028_be9e(param_1);
    uVar3 = pass1_1028_b4f2(param_1);
    uVar1 = (uVar3 >> 0x10);
    if (((uVar3 + 0x200) != 0x8000002) && ((param_1 + 0x12) == 0x5)) {
        paVar4 = mixed_1010_20ba(
            CONCAT22(uVar2, uVar1),
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffea, 0x2b),
            in_stack_0000fe92,
            in_stack_0000ffb6,
            in_stack_0000ffbc,
            in_stack_0000ffc0,
        );
        pass1_1010_043a(paVar4, (uVar3 + 0x4), 0xe);
    }
    return;
}

pub unsafe fn pass1_1028_41ea(mut param_1: i16, param_2: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut uStack54: u32;
    let mut local_2c: [u8; 0x6] = [0; 0x6];
    let mut puStack38: *mut u32;
    let mut uStack34: u32;
    let mut puStack26: *mut u32;
    let mut uStack24: u32;
    let mut local_14: u32;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b514(param_2);
    pass1_1028_b58e(param_2);
    local_14 = (param_1 + 0xc);
    iStack8 = (param_1 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34, &local_14);
    iStack16 = iStack8 + 1;
    puVar7 = CONCAT22(0x1050, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    iStack6 = param_1;
    uStack4 = extraout_DX;
    uVar6 = pass1_1028_bb24(param_2);
    uVar5 = (uVar6 >> 0x10);
    puVar2 = &local_14;
    pass1_1030_64ce(
        puVar2,
        uVar5,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        uVar6 & 0xffff | uVar5 << 0x10,
        puVar7,
    );
    uStack34 = *puVar2;
    uVar5 = (puVar2 + 2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    uVar3 = uStack54._3_1_;
    if (uStack54._3_1_ != 0) {
        uStack24 = uStack34;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack34 & 0xffff | uVar5 << 0x10);
        uStack54 = CONCAT22(uVar5, uVar3);
        uVar4 = pass1_1030_6fa0(CONCAT22(uVar5, uVar3));
        if (uVar4 == 0x64) {
            puStack38 = struct_op_1030_73a8(uStack54, 0x64, uVar5);
            ppcVar1 = (*puStack38 + 0x24);
            (**ppcVar1)();
        }
    }
    return;
}

pub unsafe fn FUN_1028_42c2() {
    return;
}

pub unsafe fn FUN_1028_42c6(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1028_43a0(mut param_1: u32) -> u16 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (((param_1 + 0x12) != 0x6) && ((param_1 + 0x12) != 0x5)) {
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1028_43f6(param_1: *mut u8, param_2: *mut astruct_15, mut param_3: i16) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut uVar5: u16;

    uVar1 = 0x83;
    puVar4 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        0x830009,
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar3 = (puVar4 >> 0x10);
    uVar1 = pass1_1010_65d0(puVar4, uVar1);
    if (0x0 < uVar1) {
        uVar2 = pass1_1028_b58e(param_2);
        if (param_3 == 0) {
            uVar5 = 0;
        } else {
            uVar5 = 0x3e8;
        }
        pass1_1030_7d1c(uVar2, uVar3, CONCAT22(uVar3, uVar2), uVar5, 0x230000);
    }
    return;
}

pub unsafe fn pass1_1028_4444(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_456e(mut param_1: u16, param_2: *mut astruct_15, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    pass1_1028_b46e(param_1, param_2, param_3);
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    puVar1 = (iVar4 + 0x20);
    uVar2 = (iVar4 + 0x22);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0x20) = 0;
    return;
}


pub unsafe fn pass1_1028_45b0(param_1: *mut astruct_15) {
    let mut in_EDX: *mut Struct57;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;

    pass1_1028_be9e(param_1);
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        uVar3 = 0;
        iVar4 = 0x4;
        uVar2 = 0x2;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar1 << 0x10));
        pass1_1030_7c50(
            uVar1,
            in_EDX,
            CONCAT22(in_EDX, uVar1),
            CONCAT22(uVar3, uVar2),
            iVar4,
        );
    }
    return;
}


pub unsafe fn pass1_1028_45e2(mut param_1: u16, mut param_2: i16, mut param_3: u32) -> u32 {
    pass1_1028_478a(param_1, param_3);
    return CONCAT22(-(0x3e8 < param_1) - param_2, 0x3e8 - param_1);
}

pub unsafe fn pass1_1028_4768(mut param_1: u16, mut param_2: i16, mut param_3: u32) -> u16 {
    pass1_1028_478a(param_1, param_3);
    if ((param_2 == 0) && (param_1 < 0x3e8)) {
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1028_4810(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_4530(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_48fa(param_1: u32) {
    pass1_1028_bdac(param_1, 0x0);
    return;
}

pub unsafe fn FUN_1028_490c() {
    return;
}

pub unsafe fn FUN_1028_4910() {
    return;
}

pub unsafe fn FUN_1028_4914() {
    return;
}

pub unsafe fn FUN_1028_4918() {
    return;
}

pub unsafe fn FUN_1028_491c(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1028_4920() -> u32 {
    let mut unaff_BP: i16;

    pass1_1028_b418((unaff_BP + 0x6));
    if ((*(unaff_BP + 0xa) & 1) != 0) {
        fn_ptr_1000_17ce(*(unaff_BP + 0x6));
    }
    return CONCAT22((unaff_BP + 0x8), (unaff_BP + 0x6));
}

pub unsafe fn pass1_1028_4a1a(mut param_1: u32, mut param_2: u32) {
    let mut BVar1: bool;
    let mut in_stack_0000ffe8: HFILE16;

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if ((BVar1 != 0) && (
        BVar1 = write_to_file_1008_7e1c(
            param_2,
            param_1 & 0xffff0000 | (param_1 + 0x20),
            0xa,
            in_stack_0000ffe8,
        ),
        BVar1 == 0,
    )) {
        u16_1050_0310 = 0x6d0;
        return;
    }
    return;
}

pub unsafe fn pass1_1028_4a5a(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut BVar1: bool;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if ((param_1 != 0) && (
        BVar1 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x20)), 0xa),
        BVar1 == 0,
    )) {
        u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}
