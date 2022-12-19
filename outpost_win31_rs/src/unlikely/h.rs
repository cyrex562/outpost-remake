
pub unsafe fn FUN_1010_9900(mut param_1: u16, mut param_2: u32, param_3: *mut u8) -> u16 {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut lVar6: i32;
    let mut in_stack_0000ffc0: HFILE16;
    let mut uStack36: u16;
    let mut local_1c: [u16; 0x3] = [0; 0x3];
    let mut local_16: [u16; 0x3] = [0; 0x3];
    let mut local_10: u32;
    let mut local_c: [u8; 0x8] = [0; 0x8];
    let mut local_4: u16;

    BVar2 = write_to_file_1008_7cac(param_3);
    if (BVar2 == 0) {
        return 0x0;
    }
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    if ((iVar4 + 0xa) == 0) {
        local_4 = 0;
    } else {
        uVar1 = (iVar4 + 0xa);
        local_4 = (uVar1 + 0x8);
    }
    local_1c[0] = local_4;
    BVar2 = write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_1c), 0x2, in_stack_0000ffc0);
    if (BVar2 == 0) {
        u16_1050_0310 = 0x6d0;
        return 0x0;
    }
    pass1_1008_5784(CONCAT22(0x1050, local_c), (iVar4 + 0xa));
    loop {
        local_10 = pass1_1008_5b12(CONCAT22(0x1050, local_c));
        if (local_10 == 0) {
            if ((iVar4 + 0xe) == 0) {
                uStack36 = 0;
            } else {
                uVar1 = (iVar4 + 0xe);
                uStack36 = (uVar1 + 0x8);
            }
            local_16[0] = uStack36;
            BVar2 = write_to_file_1008_7e1c(
                param_3,
                CONCAT22(0x1050, local_16),
                0x2,
                in_stack_0000ffc0,
            );
            if (BVar2 == 0) {
                u16_1050_0310 = 0x6d0;
                return 0x0;
            }
            pass1_1008_5784(CONCAT22(0x1050, local_c), (iVar4 + 0xe));
            loop {
                lVar6 = pass1_1008_5b12(CONCAT22(0x1050, local_c));
                uVar3 = (lVar6 >> 0x10);
                if (lVar6 == 0) {
                    if ((iVar4 + 0x12) == 0) {
                        uStack36 = 0;
                    } else {
                        uVar1 = (iVar4 + 0x12);
                        uStack36 = (uVar1 + 0x8);
                    }
                    local_16[0] = uStack36;
                    BVar2 = write_to_file_1008_7e1c(
                        param_3,
                        CONCAT22(0x1050, local_16),
                        0x2,
                        in_stack_0000ffc0,
                    );
                    if (BVar2 == 0) {
                        u16_1050_0310 = 0x6d0;
                        return 0x0;
                    }
                    pass1_1008_5784(CONCAT22(0x1050, local_c), (iVar4 + 0x12));
                    loop {
                        lVar6 = pass1_1008_5b12(CONCAT22(0x1050, local_c));
                        uVar3 = (lVar6 >> 0x10);
                        if (lVar6 == 0) {
                            local_1c[0] = (iVar4 + 0x1a);
                            BVar2 = write_to_file_1008_7e1c(
                                param_3,
                                CONCAT22(0x1050, local_1c),
                                0x2,
                                in_stack_0000ffc0,
                            );
                            if (BVar2 == 0) {
                                u16_1050_0310 = 0x6d0;
                                return 0x0;
                            }
                            local_1c[0] = (iVar4 + 0x1c);
                            BVar2 = write_to_file_1008_7e1c(
                                param_3,
                                CONCAT22(0x1050, local_1c),
                                0x2,
                                in_stack_0000ffc0,
                            );
                            if (BVar2 == 0) {
                                u16_1050_0310 = 0x6d0;
                                return 0x0;
                            }
                            local_1c[0] = (iVar4 + 0x1e);
                            BVar2 = write_to_file_1008_7e1c(
                                param_3,
                                CONCAT22(0x1050, local_1c),
                                0x2,
                                in_stack_0000ffc0,
                            );
                            if (BVar2 == 0) {
                                u16_1050_0310 = 0x6d0;
                                return 0x0;
                            }
                            return 0x1;
                        }
                        local_10 = local_10 & 0xffff0000 | (lVar6 + 0x4);
                        BVar2 = write_to_file_1008_7e1c(
                            param_3,
                            CONCAT22(0x1050, &local_10),
                            0x2,
                            in_stack_0000ffc0,
                        );
                        if (BVar2 == 0) {
                            u16_1050_0310 = 0x6d0;
                            return 0x0;
                        }
                        local_4 = (lVar6 + 0x6);
                        BVar2 = write_to_file_1008_7e1c(
                            param_3,
                            CONCAT22(0x1050, &local_4),
                            0x2,
                            in_stack_0000ffc0,
                        );
                        if BVar2 == 0 {
                            break;
                        }
                    }
                    u16_1050_0310 = 0x6d0;
                    return 0x0;
                }
                local_10 = local_10 & 0xffff0000 | (lVar6 + 0x4);
                BVar2 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, &local_10),
                    0x2,
                    in_stack_0000ffc0,
                );
                if (BVar2 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return 0x0;
                }
                local_4 = (lVar6 + 0x6);
                BVar2 = write_to_file_1008_7e1c(
                    param_3,
                    CONCAT22(0x1050, &local_4),
                    0x2,
                    in_stack_0000ffc0,
                );
                if BVar2 == 0 {
                    break;
                }
            }
            u16_1050_0310 = 0x6d0;
            return 0x0;
        }
        local_16[0] = (local_10 + 0x4);
        BVar2 =
            write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_16), 0x2, in_stack_0000ffc0);
        if (BVar2 == 0) {
            u16_1050_0310 = 0x6d0;
            return 0x0;
        }
        local_16[0] = (local_10 + 0x6);
        BVar2 =
            write_to_file_1008_7e1c(param_3, CONCAT22(0x1050, local_16), 0x2, in_stack_0000ffc0);
        if BVar2 == 0 {
            break;
        }
    }
    u16_1050_0310 = 0x6d0;
    return 0x0;
}


pub unsafe fn FUN_1010_9b72(
    mut param_1: u16,
    mut param_2: u32,
    param_3: *mut HFILE16,
    mut param_4: i16,
    mut param_5: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar7: i16;
    let mut puVar8: *mut u32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut local_36: [u16; 0x4] = [0; 0x4];
    let mut puStack46: *mut u16;
    let mut local_2a: [u16; 0x2] = [0; 0x2];
    let mut puStack38: *mut u16;
    let mut auStack34: [u16; 0x2] = [0; 0x2];
    let mut puStack30: *mut u16;
    let mut local_1a: [i16; 0x2];
    let mut puStack22: *mut u16;
    let mut uStack18: u16;
    let mut puStack14: *mut u16;
    let mut local_a: [i16; 0x3];
    let mut local_4: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_5);
    uVar10 = param_3;
    uVar11 = (param_3 >> 0x10);
    read_file_1008_7cfe(uVar10, uVar11, 1);
    if (param_4 != 0) {
        uVar2 = read_file_1008_7dee(param_3, CONCAT22(0x1050, &local_4), 0x2);
        if (uVar2 != 0) {
            iVar7 = param_2;
            uVar9 = (param_2 >> 0x10);
            if (local_4 != 0) {
                if ((iVar7 + 0xa) == 0) {
                    mem_op_1000_179c(0xc, paVar6);
                    uVar5 = paVar6;
                    puStack22 = CONCAT22(uVar5, uVar2);
                    paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar2));
                    if ((uVar5 | uVar2) == 0) {
                        (iVar7 + 0xa) = 0;
                    } else {
                        set_struct_1008_574a(CONCAT22(uVar5, uVar2));
                        (iVar7 + 0xa) = uVar2;
                        (iVar7 + 0xc) = paVar6;
                    }
                }
                // for (uStack18 = 0; uStack18 < local_4; uStack18 += 1)
                for uStack18 in 0..local_4 {
                    uVar2 = local_4;
                    mem_op_1000_179c(0x8, paVar6);
                    uVar5 = paVar6;
                    puStack22 = CONCAT22(uVar5, uVar2);
                    paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar2));
                    if ((uVar5 | uVar2) == 0) {
                        puStack14 = null_mut();
                    } else {
                        *puStack22 = 0x389a;
                        (uVar2 + 0x2) = 0x1008;
                        *puStack22 = 0xa1c4;
                        (uVar2 + 0x2) = 0x1010;
                        puStack14 = puStack22;
                    }
                    BVar3 = read_file_1008_7dee(param_3, CONCAT22(0x1050, local_a), 0x2);
                    if (BVar3 == 0) {
                        //
                        // LAB_1010_9c32:
                        puStack22 = puStack14;
                        //            if (puStack14.is_null()) goto LAB_1010_9ba1;
                        uVar9 = (puStack14 >> 0x10);
                        puVar8 = puStack14;
                        // TODO: goto LAB_1010_9e9e;
                    }
                    BVar3 = read_file_1008_7dee(
                        param_3,
                        (puStack14 & 0xffff0000 | (puStack14 + 0x6)),
                        0x2,
                    );
                    //          if (BVar3 == 0) goto LAB_1010_9c32;
                    iVar4 = switch_1008_73ea(uVar10, uVar11, local_a[0]);
                    (puStack14 + 0x4) = iVar4;
                    ppcVar1 = ((iVar7 + 0xa) + 0x4);
                    (**ppcVar1)();
                }
            }
            uVar2 = read_file_1008_7dee(param_3, CONCAT22(0x1050, local_2a), 0x2);
            if (uVar2 != 0) {
                if (local_2a[0] != 0) {
                    if ((iVar7 + 0xe) == 0) {
                        mem_op_1000_179c(0xc, paVar6);
                        uVar5 = paVar6;
                        puStack46 = CONCAT22(uVar5, uVar2);
                        paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar2));
                        if ((uVar5 | uVar2) == 0) {
                            (iVar7 + 0xe) = 0;
                        } else {
                            set_struct_1008_574a(CONCAT22(uVar5, uVar2));
                            (iVar7 + 0xe) = uVar2;
                            (iVar7 + 0x10) = paVar6;
                        }
                    }
                    //   for (auStack34[0] = 0; auStack34[0] < local_2a[0]; auStack34[0] += 1)
                    auStack34[0] = 0;
                    while auStack34[0] < local_2a[0] {
                        uVar2 = local_2a[0];
                        mem_op_1000_179c(0x8, paVar6);
                        uVar5 = paVar6;
                        puStack22 = CONCAT22(uVar5, uVar2);
                        paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar2));
                        if ((uVar5 | uVar2) == 0) {
                            puStack30 = null_mut();
                        } else {
                            *puStack22 = 0x389a;
                            (uVar2 + 0x2) = 0x1008;
                            *puStack22 = 0xa1c4;
                            (uVar2 + 0x2) = 0x1010;
                            puStack30 = puStack22;
                        }
                        BVar3 = read_file_1008_7dee(param_3, CONCAT22(0x1050, local_1a), 0x2);
                        if (BVar3 == 0) {
                            //
                            // LAB_1010_9d5c:
                            puStack22 = puStack30;
                            //              if (puStack30.is_null()) goto LAB_1010_9ba1;
                            uVar9 = (puStack30 >> 0x10);
                            puVar8 = puStack30;
                            // TODO: goto LAB_1010_9e9e;
                        }
                        BVar3 = read_file_1008_7dee(
                            param_3,
                            (puStack30 & 0xffff0000 | (puStack30 + 0x6)),
                            0x2,
                        );
                        //            if (BVar3 == 0) goto LAB_1010_9d5c;
                        iVar4 = switch_1008_73ea(uVar10, uVar11, local_1a[0]);
                        (puStack30 + 0x4) = iVar4;
                        ppcVar1 = ((iVar7 + 0xe) + 0x4);
                        (**ppcVar1)();
                        auStack34[0] += 1;
                    }
                }
                uVar2 = read_file_1008_7dee(param_3, CONCAT22(0x1050, local_36), 0x2);
                if (uVar2 != 0) {
                    if (local_36[0] != 0) {
                        if ((iVar7 + 0x12) == 0) {
                            mem_op_1000_179c(0xc, paVar6);
                            uVar5 = paVar6;
                            puStack22 = CONCAT22(uVar5, uVar2);
                            paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar2));
                            if ((uVar5 | uVar2) == 0) {
                                (iVar7 + 0x12) = 0;
                            } else {
                                set_struct_1008_574a(CONCAT22(uVar5, uVar2));
                                (iVar7 + 0x12) = uVar2;
                                (iVar7 + 0x14) = paVar6;
                            }
                        }
                        // for (local_2a[0] = 0; local_2a[0] < local_36[0]; local_2a[0] += 1)
                        local_2a[0] = 0;
                        while local_2a[0] < local_36[0] {
                            uVar2 = local_36[0];
                            mem_op_1000_179c(0x8, paVar6);
                            uVar5 = paVar6;
                            puStack46 = CONCAT22(uVar5, uVar2);
                            paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar2));
                            if ((uVar5 | uVar2) == 0) {
                                puStack38 = null_mut();
                            } else {
                                *puStack46 = 0x389a;
                                (uVar2 + 0x2) = 0x1008;
                                *puStack46 = 0xa1c4;
                                (uVar2 + 0x2) = 0x1010;
                                puStack38 = puStack46;
                            }
                            BVar3 = read_file_1008_7dee(param_3, CONCAT22(0x1050, auStack34), 0x2);
                            if (BVar3 == 0) {
                                //
                                // LAB_1010_9e86:
                                puStack22 = puStack38;
                                if (puStack38.is_null() == false) {
                                    uVar9 = (puStack38 >> 0x10);
                                    puVar8 = puStack38; //
                                                        // LAB_1010_9e9e:
                                    ppcVar1 = *puVar8;
                                    puStack46 = puStack22;
                                    (**ppcVar1)(0x1008, puStack22, uVar9, 1);
                                }
                                // TODO: goto LAB_1010_9ba1;
                            }
                            BVar3 = read_file_1008_7dee(
                                param_3,
                                (puStack38 & 0xffff0000 | (puStack38 + 0x6)),
                                0x2,
                            );
                            //              if (BVar3 == 0) goto LAB_1010_9e86;
                            iVar4 = switch_1008_73ea(uVar10, uVar11, auStack34[0]);
                            (puStack38 + 0x4) = iVar4;
                            ppcVar1 = ((iVar7 + 0x12) + 0x4);
                            (**ppcVar1)();
                            local_2a[0] += 1;
                        }
                    }
                    BVar3 =
                        read_file_1008_7dee(param_3, (param_2 & 0xffff0000 | (iVar7 + 0x1a)), 0x2);
                    if (BVar3 != 0) {
                        BVar3 = read_file_1008_7dee(
                            param_3,
                            (param_2 & 0xffff0000 | (iVar7 + 0x1c)),
                            0x2,
                        );
                        if (BVar3 != 0) {
                            BVar3 = read_file_1008_7dee(
                                param_3,
                                (param_2 & 0xffff0000 | (iVar7 + 0x1e)),
                                0x2,
                            );
                            if (BVar3 != 0) {
                                return;
                            }
                        }
                    }
                }
            }
        } //
          // LAB_1010_9ba1:
        u16_1050_0310 = 0x6d2;
    }
    return;
}


pub unsafe fn pass1_1010_a172(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_95f8(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_a198(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn load_str_1010_ddf6(mut param_1: u32, mut param_2: u32) {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut in_buf_len_5: i16;
    let mut uVar1: u32;

    in_buf_len_5 = (param_1 >> 0x10);
    *(param_1 + 0x13c) = 0;
    uVar1 = struct_op_1030_73a8(param_2, in_AX, in_DX);
    match (uVar1 + 0x12) {
        0x1 | 0x2 | 0x4 | 0x7 | 0x9 => {}
        // break;
        0x3 | 0x5 => {}
        // break;
        0x6 => {}
        // break;
        0x8 => {}
        // break;
        _ => {} // TODO: goto switchD_1010_de53_caseD_9;
    }
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x3ff,
        (param_1 + 0x13c),
        in_buf_len_5,
    );
    // switchD_1010_de53_caseD_9:
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_de78(mut param_1: u32, mut param_2: u32) {
    let mut in_buf_len_5: i16;

    in_buf_len_5 = (param_1 >> 0x10);
    *(param_1 + 0x13c) = 0;
    pass1_1030_809c(param_2);
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x3ff,
        (param_1 + 0x13c),
        in_buf_len_5,
    );
    return;
}
