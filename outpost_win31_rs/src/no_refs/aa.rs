use crate::block_1028::block_1028_1000::pass1_1028_1e14;
use crate::block_1028::block_1028_2000::{pass1_1028_2042, pass1_1028_21ba, pass1_1028_2220, pass1_1028_297c, pass1_1028_2f18};

pub unsafe fn pass1_1028_1b1e(mut param_1: u32) {
    (param_1 + 0x14) = 0x7;
    return;
}


pub unsafe fn pass1_1028_1b2e(
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

pub unsafe fn pass1_1028_1c22(mut param_1: u32) -> u16 {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x20) != 0) && ((iVar2 + 0x12) == 0x5 || ((iVar2 + 0x12) == 0x6))) {
        if ((iVar2 + 0xc) == 0x16) {
            return 0x19;
        }
        if ((iVar2 + 0xc) == 0x17) {
            return 0x1a;
        }
    }
    uVar1 = pass1_1028_bc1c(param_1 & 0xffff | uVar3 << 0x10);
    return uVar1;
}

pub unsafe fn pass1_1028_1c66(param_1: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u32;

    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    uVar3 = pass1_1028_b4f2(param_1);
    if ((uVar3 + 0x200) != 0x8000002) {
        ppcVar1 = (param_1 + 0x64);
        iVar2 = (**ppcVar1)();
        if (iVar2 == 0) {
            return;
        }
        pass1_1028_cb04(param_1);
        if (iVar2 == 0) {
            iVar2 = 0x6;
            // TODO: goto LAB_1028_1cbd;
        }
        pass1_1028_c952(param_1);
    }
    iVar2 = 0x5; //
    // LAB_1028_1cbd:
    pass1_1028_bdac(param_1, iVar2);
    return;
}

pub unsafe fn pass1_1028_1cca(
    mut param_1: u32,
    param_2: *mut u32,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    param_6: i32,
) -> u16 {
    let mut uVar1: u16;
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
    uVar1 = pass1_1028_1e14(
        &local_8,
        param_3,
        uVar2,
        uVar3,
        0x16,
        CONCAT22(0x1050, &local_8),
        param_6,
    );
    if (uVar1 == 0) {
        local_8 = local_8 & 0xffff | (local_c + 1) << 0x10;
        uVar1 = pass1_1028_1e14(
            &local_8,
            param_3,
            uVar2,
            uVar3,
            0x16,
            CONCAT22(0x1050, &local_8),
            param_6,
        );
        if (uVar1 == 0) {
            local_8 = local_a - 0x1;
            local_8 = local_c;
            uVar1 = pass1_1028_1e14(
                &local_8,
                param_3,
                uVar2,
                uVar3,
                0x17,
                CONCAT22(0x1050, &local_8),
                param_6,
            );
            if (uVar1 == 0) {
                local_8 = CONCAT22(local_8, local_a + 1);
                uVar1 = pass1_1028_1e14(
                    &local_8,
                    param_3,
                    uVar2,
                    uVar3,
                    0x17,
                    CONCAT22(0x1050, &local_8),
                    param_6,
                );
                if (uVar1 == 0) {
                    return uVar1;
                }
            }
        }
    }
    return 0x1;
}

pub unsafe fn pass1_1028_1da4(
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

    uVar5 = pass1_1030_bcae(local_4, &DAT_1050_1050);
    uVar4 = (uVar5 >> 0x10);
    iVar1 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4);
    uVar3 = (iVar1 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3);
    puVar2 = local_4;
    pass1_1030_bcde(
        puVar2,
        &DAT_1050_1050,
        uVar3 & 0xffff | uVar4 << 0x10,
        param_3,
        param_5,
    );
    if (puVar2 < 0x0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

pub unsafe fn pass1_1028_1e8a(param_1: *mut astruct_15) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar3: u16;

    uVar1 = (param_1 >> 0x10);
    if ((*(param_1 + 0x1a) & 0x2) == 0) {
        uVar4 = 0;
        uVar5 = 0x23;
        uVar3 = 0x1;
        uVar2 = pass1_1028_b58e((param_1 & 0xffff | uVar1 << 0x10));
        pass1_1030_7d7c(uVar2, (uVar2 >> 0x10), uVar2, uVar3, CONCAT22(uVar5, uVar4));
        pass1_1028_bdac(param_1, 0x6);
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1028_1ec8(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_20b6(param_1: *mut astruct_15) {
    let mut uVar1: u16;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut auStack22: [u8; 0x2] = [0; 0x2];
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut uStack16: u32;
    let mut uStack12: u16;
    let mut uStack10: u32;
    let mut uStack6: u32;

    pass1_1028_be9e(param_1);
    uVar4 = (param_1 >> 0x10);
    uVar3 = param_1;
    if ((uVar3 + 0x12) == 0x5) {
        uStack6 = pass1_1028_bb24((param_1 & 0xffff | uVar4 << 0x10));
        uStack10 = pass1_1028_b58e(param_1);
        puVar2 = (uStack10 >> 0x10);
        uStack16 = (uStack10 + 0xc);
        uStack12 = (uStack10 + 0x10);
        pass1_1008_3eb4(
            CONCAT22(0x1050, &uStack16),
            CONCAT22(0x1050, auStack22),
            CONCAT22(0x1050, &iStack20),
            CONCAT22(0x1050, &iStack18),
        );
        uStack16 = uStack16 & 0xffff | (iStack20 - 1) << 0x10;
        uVar1 = pass1_1028_21ba(
            &uStack16,
            puVar2,
            uVar3,
            uVar4,
            CONCAT22(0x1050, &uStack16),
            uStack6,
        );
        if (uVar1 == 0) {
            uStack16 = uStack16 & 0xffff | (iStack20 + 1) << 0x10;
            uVar1 = pass1_1028_21ba(
                &uStack16,
                puVar2,
                uVar3,
                uVar4,
                CONCAT22(0x1050, &uStack16),
                uStack6,
            );
            if (uVar1 == 0) {
                uStack16 = CONCAT22(iStack20, iStack18 - 1);
                uVar1 = pass1_1028_21ba(
                    &uStack16,
                    puVar2,
                    uVar3,
                    uVar4,
                    CONCAT22(0x1050, &uStack16),
                    uStack6,
                );
                if (uVar1 == 0) {
                    uStack16 = uStack16 & 0xffff0000 | (iStack18 + 1);
                    uVar1 = pass1_1028_21ba(
                        &uStack16,
                        puVar2,
                        uVar3,
                        uVar4,
                        CONCAT22(0x1050, &uStack16),
                        uStack6,
                    );
                    if (uVar1 == 0) {
                        return;
                    }
                }
            }
        }
        pass1_1038_79b2(uVar1, puVar2, _PTR_LOOP_1050_5a64, uStack10);
    }
    return;
}


pub unsafe fn pass1_1028_2290(
    mut param_1: u32,
    param_2: *mut u32,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    param_6: i32,
) -> i16 {
    let mut iVar1: i16;
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
    iVar1 = pass1_1028_2220(
        &local_8,
        param_3,
        uVar2,
        uVar3,
        0x16,
        CONCAT22(0x1050, &local_8),
        param_6,
    );
    if (iVar1 == 0) {
        local_8 = local_8 & 0xffff | (local_c + 1) << 0x10;
        iVar1 = pass1_1028_2220(
            &local_8,
            param_3,
            uVar2,
            uVar3,
            0x16,
            CONCAT22(0x1050, &local_8),
            param_6,
        );
        if (iVar1 == 0) {
            local_8 = local_a - 0x1;
            local_8 = local_c;
            iVar1 = pass1_1028_2220(
                &local_8,
                param_3,
                uVar2,
                uVar3,
                0x17,
                CONCAT22(0x1050, &local_8),
                param_6,
            );
            if (iVar1 == 0) {
                local_8 = CONCAT22(local_8, local_a + 1);
                iVar1 = pass1_1028_2220(
                    &local_8,
                    param_3,
                    uVar2,
                    uVar3,
                    0x17,
                    CONCAT22(0x1050, &local_8),
                    param_6,
                );
                if (iVar1 == 0) {
                    return iVar1;
                }
            }
        }
    }
    return 0x1;
}


pub unsafe fn pass1_1028_236a(param_1: *mut astruct_15) -> u16 {
    let mut uVar1: u16;
    let mut paVar2: *mut astruct_398;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;

    uVar1 = (param_1 >> 0x10);
    if ((*(param_1 + 0x1a) & 0x2) == 0) {
        uVar4 = 0;
        uVar5 = 0x23;
        uVar3 = 0x1;
        paVar2 = pass1_1028_b58e((param_1 & 0xffff | uVar1 << 0x10));
        pass1_1030_7d7c(
            paVar2,
            (paVar2 >> 0x10),
            paVar2,
            uVar3,
            CONCAT22(uVar5, uVar4),
        );
        pass1_1028_bdac(param_1, 0x6);
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1028_23a8(
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

    uVar5 = pass1_1030_bcae(local_4, &DAT_1050_1050);
    uVar4 = (uVar5 >> 0x10);
    iVar1 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4);
    uVar3 = (iVar1 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3);
    puVar2 = local_4;
    pass1_1030_bcde(
        puVar2,
        &DAT_1050_1050,
        uVar3 & 0xffff | uVar4 << 0x10,
        param_3,
        param_5,
    );
    if (puVar2 < 0x0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}


pub unsafe fn pass1_1028_2418(mut param_1: u32, mut param_2: u32) -> BOOL16 {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut in_stack_0000ffce: HFILE16;
    let mut local_1c: [u16; 0x6] = [0; 0x6];
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut uStack12: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar2 != 0) {
        uVar3 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x20));
        uVar1 = (param_1 + 0x20);
        local_1c[0] = (uVar1 + 0x8);
        uStack16 = local_1c[0];
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_1c), 0x2, in_stack_0000ffce);
        if (BVar2 == 0) {
            u16_1050_0310 = 0x6d0;
            return BVar2;
        }
        loop {
            uVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            iStack14 = uVar4;
            if (uVar4 == 0) {
                break;
            }
            pass1_1038_75ca(iStack14, uVar4, param_2);
            uStack12 = (uVar4 >> 0x10);
            if (uVar4 == 0) {
                return uVar4;
            }
        }
        BVar2 = 0x1;
    }
    return BVar2;
}


pub unsafe fn file_1028_24a2(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) -> BOOL16 {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut pHVar7: *mut HFILE16;
    let mut uStack6: u16;
    let mut local_4: u16;
    let mut paVar6: *mut Struct57;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 == 0) {
        return 0x0;
    }
    if (0x1 < u16_1050_0312) {
        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar2 == 0) {
            u16_1050_0310 = 0x6d2;
            return 0x0;
        }
        for uStack6 in 0..local_4 {
            uVar3 = local_4;
            pHVar7 = param_4;
            mem_op_1000_179c(0x2a, paVar5);
            uVar4 = paVar5 | uVar3;
            paVar6 = (paVar5 & 0xffff0000 | uVar4);
            if (uVar4 == 0) {
                uVar3 = 0;
                paVar5 = (paVar5 & 0xffff0000);
            } else {
                struct_1038_6520(CONCAT22(paVar5, uVar3));
                paVar5 = paVar6;
            }
            file_1038_774e(paVar5, CONCAT22(paVar5, uVar3), pHVar7);
            if (uVar3 == 0) {
                return 0x0;
            }
            ppcVar1 = ((param_3 + 0x20) + 0x8);
            (**ppcVar1)();
        }
    }
    return 0x1;
}

pub unsafe fn pass1_1028_254c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_2042(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_2626(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_2700(param_1: *mut astruct_15) {
    let mut uVar1: u16;
    let mut uVar2: u32;

    pass1_1028_be2a(param_1);
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        uVar2 = pass1_1028_b4f2((param_1 & 0xffff | uVar1 << 0x10));
        (uVar2 + 0x204) = 0x1;
    }
    return;
}

pub unsafe fn pass1_1028_272e(param_1: *mut astruct_15) {
    let mut uVar1: u32;

    pass1_1028_be9e(param_1);
    uVar1 = pass1_1028_b4f2(param_1);
    if ((param_1 + 0x12) == 0x5) {
        (uVar1 + 0x204) = 0x1;
    }
    return;
}

pub unsafe fn pass1_1028_2762(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_2844(
    mut param_1: u32,
    param_2: *mut u32,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    param_6: i32,
) -> u16 {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
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
    uVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    BVar1 = pass1_1028_c5a6(
        &local_8,
        param_3,
        uVar3,
        uVar4,
        0x7b,
        CONCAT22(0x1050, &local_8),
        param_6,
    );
    if (BVar1 == 0) {
        uVar2 = pass1_1028_297c(
            &local_8,
            param_3,
            param_1,
            CONCAT22(0x1050, &local_8),
            param_6,
        );
        if (uVar2 == 0) {
            local_8 = local_8 & 0xffff | (local_c + 1) << 0x10;
            BVar1 = pass1_1028_c5a6(
                &local_8,
                param_3,
                uVar3,
                uVar4,
                0x7b,
                CONCAT22(0x1050, &local_8),
                param_6,
            );
            if (BVar1 == 0) {
                uVar2 = pass1_1028_297c(
                    &local_8,
                    param_3,
                    param_1,
                    CONCAT22(0x1050, &local_8),
                    param_6,
                );
                if (uVar2 == 0) {
                    local_8 = local_a - 0x1;
                    local_8 = local_c;
                    BVar1 = pass1_1028_c5a6(
                        &local_8,
                        param_3,
                        uVar3,
                        uVar4,
                        0x7c,
                        CONCAT22(0x1050, &local_8),
                        param_6,
                    );
                    if (BVar1 == 0) {
                        uVar2 = pass1_1028_297c(
                            &local_8,
                            param_3,
                            param_1,
                            CONCAT22(0x1050, &local_8),
                            param_6,
                        );
                        if (uVar2 == 0) {
                            local_8 = CONCAT22(local_8, local_a + 1);
                            BVar1 = pass1_1028_c5a6(
                                &local_8,
                                param_3,
                                uVar3,
                                uVar4,
                                0x7c,
                                CONCAT22(0x1050, &local_8),
                                param_6,
                            );
                            if (BVar1 == 0) {
                                uVar3 = pass1_1028_297c(
                                    &local_8,
                                    param_3,
                                    param_1,
                                    CONCAT22(0x1050, &local_8),
                                    param_6,
                                );
                                if (uVar3 == 0) {
                                    return uVar3;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0x1;
}

pub unsafe fn pass1_1028_2a6c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1028_2b46() {
    return;
}

pub unsafe fn FUN_1028_2b4a() {
    return;
}

pub unsafe fn pass1_1028_2b4e(
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


pub unsafe fn pass1_1028_2c28(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u32,
    mut param_5: u32,
    mut param_6: u32,
) -> u16 {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    pass1_1028_09d4(param_1, param_2, param_3, param_4, param_5, param_6);
    if (param_1 != 0) {
        local_8 = *param_4;
        uStack4 = (param_4 + 0x4);
        puVar2 = &local_e;
        pass1_1008_3eb4(
            CONCAT22(0x1050, &local_8),
            CONCAT22(0x1050, puVar2),
            CONCAT22(0x1050, &local_c),
            CONCAT22(0x1050, &local_a),
        );
        pass1_1008_3e76(
            CONCAT22(0x1050, &local_8),
            local_e,
            local_c - 0x1,
            local_a - 1,
        );
        puVar1 = &local_8;
        uVar3 = param_3;
        uVar4 = (param_3 >> 0x10);
        pass1_1028_c7b6(puVar2, uVar3, uVar4, CONCAT22(0x1050, puVar1), param_6);
        if (puVar1.is_null() == false) {
            pass1_1008_3e76(CONCAT22(0x1050, &local_8), local_e, local_c - 0x1, local_a);
            puVar1 = &local_8;
            pass1_1028_c7b6(puVar2, uVar3, uVar4, CONCAT22(0x1050, puVar1), param_6);
            if (puVar1.is_null() == false) {
                pass1_1008_3e76(
                    CONCAT22(0x1050, &local_8),
                    local_e,
                    local_c - 0x1,
                    local_a + 1,
                );
                puVar1 = &local_8;
                pass1_1028_c7b6(puVar2, uVar3, uVar4, CONCAT22(0x1050, puVar1), param_6);
                if (puVar1.is_null() == false) {
                    pass1_1008_3e76(CONCAT22(0x1050, &local_8), local_e, local_c, local_a - 1);
                    puVar1 = &local_8;
                    pass1_1028_c7b6(puVar2, uVar3, uVar4, CONCAT22(0x1050, puVar1), param_6);
                    if (puVar1.is_null() == false) {
                        pass1_1008_3e76(CONCAT22(0x1050, &local_8), local_e, local_c, local_a);
                        puVar1 = &local_8;
                        pass1_1028_c7b6(puVar2, uVar3, uVar4, CONCAT22(0x1050, puVar1), param_6);
                        if (puVar1.is_null() == false) {
                            pass1_1008_3e76(
                                CONCAT22(0x1050, &local_8),
                                local_e,
                                local_c,
                                local_a + 1,
                            );
                            puVar1 = &local_8;
                            pass1_1028_c7b6(
                                puVar2,
                                uVar3,
                                uVar4,
                                CONCAT22(0x1050, puVar1),
                                param_6,
                            );
                            if (puVar1.is_null() == false) {
                                pass1_1008_3e76(
                                    CONCAT22(0x1050, &local_8),
                                    local_e,
                                    local_c + 0x1,
                                    local_a - 1,
                                );
                                puVar1 = &local_8;
                                pass1_1028_c7b6(
                                    puVar2,
                                    uVar3,
                                    uVar4,
                                    CONCAT22(0x1050, puVar1),
                                    param_6,
                                );
                                if (puVar1.is_null() == false) {
                                    pass1_1008_3e76(
                                        CONCAT22(0x1050, &local_8),
                                        local_e,
                                        local_c + 0x1,
                                        local_a,
                                    );
                                    puVar1 = &local_8;
                                    pass1_1028_c7b6(
                                        puVar2,
                                        uVar3,
                                        uVar4,
                                        CONCAT22(0x1050, puVar1),
                                        param_6,
                                    );
                                    if (puVar1.is_null() == false) {
                                        pass1_1008_3e76(
                                            CONCAT22(0x1050, &local_8),
                                            local_e,
                                            local_c + 0x1,
                                            local_a + 1,
                                        );
                                        puVar1 = &local_8;
                                        pass1_1028_c7b6(
                                            puVar2,
                                            uVar3,
                                            uVar4,
                                            CONCAT22(0x1050, puVar1),
                                            param_6,
                                        );
                                        if (puVar1.is_null() == false) {
                                            return 0x1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0x0;
}

pub unsafe fn pass1_1028_2e40(param_1: u8, mut param_2: i16, param_3: *mut u8, param_4: *mut u32) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut paVar2: *mut Struct27;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;

    paVar1 = CONCAT22(in_register_0000000a, param_3);
    pass1_1028_be9e(param_4);
    if ((param_4 + 0x12) == 0x5) {
        pass1_1028_2f18(param_1, param_2, param_4);
        pass1_1028_3246(param_2, paVar1, param_4);
        uVar3 = 0x1;
        iVar4 = 0x5;
        paVar2 = mixed_1010_20ba(
            paVar1,
            _u16_1050_0ed0,
            0x1002b,
            in_stack_0000fe9e,
            in_stack_0000ffc2,
            in_stack_0000ffc8,
            in_stack_0000ffcc,
        );
        pass1_1010_089e(paVar2, uVar3, iVar4);
    }
    return;
}
