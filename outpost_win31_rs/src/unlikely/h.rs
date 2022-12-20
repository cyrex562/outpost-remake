use crate::block_1010::block_1010_4000::{draw_op_1010_47d0, pass1_1010_459e, pass1_1010_45d6};
use crate::block_1010::block_1010_5000::{pass1_1010_519a, pass1_1010_533c, string_1010_5286};

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


pub unsafe fn pass1_1010_4674(
    param_1: *mut Struct27,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut piVar1: *mut i16;
    let mut paVar2: *mut Struct27;
    let mut uVar2: u16;

    paVar2 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        piVar1 = (&paVar2.field30_0x22 + 2);
        *piVar1 = *piVar1 + 1;
        if (0xf < (&paVar2.field30_0x22 + 0x2)) {
            (&paVar2.field30_0x22 + 0x2) = 0;
        } //
        // LAB_1010_469a:
        draw_op_1010_47d0(paVar2, uVar2, (&paVar2.field30_0x22 + 0x2));
    } else if (param_2 != 0x2) {
        if (param_2 != 0x3) {
            if ((paVar2[0x1].field19_0x14 != 0) && (paVar2[0x1].field19_0x14 != 0x4)) {
                pass1_1010_459e(param_1);
            }
            // TODO: goto LAB_1010_46e8;
        }
        piVar1 = (&paVar2.field30_0x22 + 2);
        *piVar1 = *piVar1 - 0x1;
        if (*piVar1 < 0x0) {
            (&paVar2.field30_0x22 + 0x2) = 0xf;
        }
        // TODO: goto LAB_1010_469a;
    }
    pass1_1010_1f62(param_1, 0x2);
    pass1_1010_45d6(param_1); //
    // LAB_1010_46e8:
    paVar2[0x1].field19_0x14 = param_2;
    return;
}

pub unsafe fn pass1_1010_4788(mut param_1: u16, mut param_2: u16, mut param_3: u32, param_4: *mut c_char) {
    pass1_1030_8344(_u16_1050_5748, (param_3 + 0x6c));
    pass1_1030_301a(param_2, CONCAT22(param_2, param_1), param_4);
    return;
}

pub unsafe fn pass1_1010_4956(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar2 = (iVar3 + 0x6a);
    if (iVar2 == 0) {
        piVar1 = (iVar3 + 0x24);
        *piVar1 = *piVar1 + 1;
        if (0xf < (iVar3 + 0x24)) {
            (iVar3 + 0x24) = 0;
            return;
        }
    } else {
        if (iVar2 != 0x4) {
            return;
        }
        piVar1 = (iVar3 + 0x24);
        *piVar1 = *piVar1 - 0x1;
        if (*piVar1 < 0x0) {
            (iVar3 + 0x24) = 0xf;
        }
    }
    return;
}

pub unsafe fn free_rsrc_1010_4b3e(param_1: *mut StructD) {
    let mut puVar3: *mut u32;
    let mut uVar5: u32;
    let mut BVar6: bool;
    let mut pstructd_1: *mut StructD;
    let mut iVar7: *mut astruct_818;
    let mut pstructd_1_hi: *mut StructD;
    let mut uVar4: u16;
    let mut unaff_CS: u16;
    let mut iStack4: i16;
    let mut puVar2: *mut u32;
    let mut piVar1: *mut i16;
    let mut uVar1: u16;
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut fn_ptr_1: *mut *mut code;

    pstructd_1_hi = (param_1 >> 0x10);
    pstructd_1 = param_1;
    // really just 0x5024
    param_1.address_offset_field_0x0 = s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
    pstructd_1.address_offset_field_0x2 = 0x1010;
    if (pstructd_1.field_0x2a != 0) {
        unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
        BVar6 = GlobalUnlock16(&pstructd_1.field_0x2a);
        if (BVar6 == 0) {
            unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
            FreeResource16(&pstructd_1.field_0x2a);
        }
    }
    pstructd_1.field_0x2a = 0;
    if (&pstructd_1.field11_0x12 != 0) {
        iStack4 = 0;
        loop {
            puVar3 = &pstructd_1.field11_0x12;
            piVar1 = (puVar3 + 0x8);
            if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                break;
            }
            uVar4 = (*puVar3 >> 0x10);
            iVar7 = *puVar3;
            puVar2 = (iVar7 + iStack4 * 0x4);
            uVar1 = (iVar7 + iStack4 * 0x4 + 2);
            if ((uVar1 | puVar2) != 0) {
                fn_ptr_1 = *puVar2;
                (**fn_ptr_1)(unaff_CS, puVar2, uVar1, 1);
            }
            iStack4 += 0x1;
        }
    }
    uVar5 = &pstructd_1.field11_0x12;
    fn_ptr_1000_17ce(*(uVar5 + 0x4));
    fn_ptr_1000_17ce(*&pstructd_1.field11_0x12);
    puVar1 = (&pstructd_1.field12_0x14 + 2);
    uVar2 = pstructd_1.field13_0x18;
    if ((uVar2 | puVar1) != 0) {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)(0x1000, puVar1, uVar2, 1);
    }
    fn_ptr_1000_17ce(*&pstructd_1.field14_0x1a);
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_4c2c(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16));
}


pub unsafe fn pass1_1010_4dc8(mut param_1: u32) -> u32 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x20) == 0) {
        return 0x0;
    }
    return CONCAT22((iVar1 + 0x1c), (iVar1 + 0x20) * 0x8 + (iVar1 + 0x1a));
}


pub unsafe fn pass1_1010_4df0(mut param_1: u16, mut param_2: u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_2 >> 0x10);
    uVar1 = (param_2 + 0x26);
    pass1_1010_c1ba(param_1, uVar1, (uVar1 >> 0x10), (param_2 + 0x20));
    return;
}


pub unsafe fn pt_in_rect_1010_4e08(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut bVar2: bool;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut POPStack8: INT16;

    PStack8 = CONCAT22(param_2, param_3);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    (iVar4 + 0x22) = (iVar4 + 0x20);
    bVar2 = false;
    (iVar4 + 0x24) = 0;
    iStack12 = 0;
    iStack10 = 0;
    loop {
        piVar1 = (iVar4 + 0x30);
        if (*piVar1 == iStack12 || *piVar1 < iStack12) {
            //
            // LAB_1010_4e67:
            if (iStack10 != 0) {
                (iVar4 + 0x20) = iStack10;
            }
            if (bVar2) {
                return;
            }
            return;
        }
        BVar3 = PtInRect16(PStack8, ((iVar4 + 0x1a) + iStack12 * 0x8));
        if (BVar3 != 0) {
            iStack10 = iStack12;
            bVar2 = true;
            // TODO: goto LAB_1010_4e67;
        }
        iStack12 += 0x1;
    }
}


pub unsafe fn pass1_1010_4e8c(mut param_1: u32) {
    pass1_1010_1f62(param_1, 0xd);
    return;
}

pub unsafe fn find_n_load_rsrc_1010_4e9e(struct_param_1: *mut astruct_812) {
    let mut BVar1: bool;
    let mut h_rsrc: HRSRC16;
    let mut handle: HGLOBAL16;
    let mut struct_1: *mut astruct_812;
    let mut uVar3: u16;
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar3 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    if (struct_1.field29_0x20 != 0) {
        if (struct_1.hglobal_0x2a != 0) {
            BVar1 = GlobalUnlock16(struct_1.hglobal_0x2a);
            if (BVar1 == 0) {
                FreeResource16(struct_1.hglobal_0x2a);
            }
        }
        uVar1 = struct_1.field18_0x12;
        uVar2 = (uVar1 + 0x4);
        h_rsrc = FindResource16(
            0xa,
            ((uVar2 + struct_1.field29_0x20 * 0x2) * 0x2 + 0x1384),
            HINSTANCE16_1050_038c,
        );
        handle = LoadResource16(h_rsrc, HINSTANCE16_1050_038c);
        struct_1.hglobal_0x2a = handle;
        if (handle != 0) {
            WIN16_LockResource16(handle);
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1010_4f20(mut param_1: u16, mut param_2: u16, mut param_3: i16) -> u16 {
    return (param_3 * 0x2 + 0x139a);
}

pub unsafe fn pass1_1010_4f30(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut u16,
    param_4: *mut u16,
) {
    *param_4 = 0xa;
    *param_3 = 0x73;
    return;
}

pub unsafe fn pass1_1010_4f48(param_1: *mut astruct_482) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u32;
    let mut puVar4: *mut u32;
    let mut in_EDX: u32;
    let mut uVar6: u16;
    let mut uVar5: u32;
    let mut iVar6: *mut astruct_482;
    let mut iVar7: *mut astruct_483;
    let mut uVar7: *mut astruct_482;
    let mut uVar8: u16;
    let mut unaff_CS: u16;

    uVar6 = (in_EDX >> 0x10);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    puVar3 = iVar6.field18_0x12;
    iVar6.field39_0x30 = (puVar3 + 0x8);
    if (iVar6.field40_0x32 != 0) {
        uVar5 = *iVar6.field18_0x12;
        uVar8 = (uVar5 >> 0x10);
        iVar7 = uVar5;
        puVar3 = iVar7.field4_0x4;
        iVar7.field4_0x4 = iVar6.field40_0x32;
        if puVar3.is_null() == false {
            ppcVar2 = *puVar3;
            (**ppcVar2)();
        }
        iVar6.field40_0x32 = 0;
    }
    puVar4 = iVar6.field19_0x16;
    uVar1 = iVar6.field20_0x18;
    uVar5 = CONCAT22(uVar6, uVar1);
    if ((uVar1 | puVar4) != 0) {
        ppcVar2 = *puVar4;
        (**ppcVar2)();
    }
    puVar4 = FUN_1010_830a(puVar4, uVar5, unaff_CS, _u16_1050_14cc, 0x1b3);
    iVar6.field19_0x16 = puVar4;
    iVar6.field20_0x18 = uVar5;
    fn_ptr_1000_17ce(iVar6.field21_0x1a);
    iVar6.field21_0x1a = 0;
    iVar6.field38_0x2e = 0;
    return;
}

pub unsafe fn pass1_1010_50f2(param_1: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = 0x53f4;
    (param_1 + 0x2) = 0x1010;
    fn_ptr_1000_17ce(*(param_1 + 0xc));
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_5120(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u16) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut iVar8: i16;
    let mut uVar9: u16;

    uVar9 = (param_3 >> 0x10);
    iVar8 = param_3;
    if ((iVar8 + 0x16) != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar8 + 0x16));
        uVar5 = param_2 | param_1;
        if (uVar5 != 0) {
            uVar1 = (param_1 + 0x1f6);
            uVar4 = uVar1;
            pass1_1030_38f2(uVar1, 0x3);
            uVar2 = uVar4;
            uVar6 = uVar5;
            uVar3 = uVar2;
            pass1_1030_38f2(uVar1, 0x4);
            iVar7 = uVar6 + uVar5 + CARRY2(uVar3, uVar2);
            if ((0x0 < iVar7) || (-0x1 < iVar7 && (param_4 <= uVar3 + uVar2))) {
                (iVar8 + 0xa) = param_4;
                return;
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_52fc(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut uVar1: u16;

    pass1_1010_533c(param_2, param_3, param_4);
    uVar1 = (param_3 >> 0x10);
    (param_3 + 0x12) = param_1;
    (param_3 + 0x14) = param_2;
    return;
}

pub unsafe fn pass1_1010_531c(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut uVar1: u16;

    pass1_1010_533c(param_2, param_3, param_4);
    uVar1 = (param_3 >> 0x10);
    (param_3 + 0x16) = param_1;
    (param_3 + 0x18) = param_2;
    return;
}

pub unsafe fn pass1_1010_533c(param_1: *mut u8, mut param_2: u32, param_3: *mut c_char) {
    let mut puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut pcVar7: *mut c_char;
    let mut uStack6: u16;
    let mut local_4: [u8; 0x2] = [0; 0x2];

    pass1_1010_519a(param_1, param_2, CONCAT22(0x1050, local_4));
    uStack6 = 0;
    loop {
        uVar6 = (param_2 >> 0x10);
        uVar5 = param_2;
        puVar1 = (uVar5 + 0x10);
        if (*puVar1 < uStack6 || *puVar1 == uStack6) {
            return;
        }
        uVar3 = (uVar5 + 0xc);
        uVar2 = (uVar3 + uStack6 * 0x4);
        pcVar7 = string_1010_5286(uVar2, param_1, uVar5, uVar6, uVar2);
        param_1 = (pcVar7 >> 0x10);
        iVar4 = pass1_1000_3d7a(param_3, (pcVar7 & 0xffff | ZEXT24(param_1) << 0x10));
        if (iVar4 == 0) {
            break;
        }
        fn_ptr_1000_17ce(pcVar7);
        uStack6 += 0x1;
    }
    return;
}

pub unsafe fn pass1_1010_5d9c(param_1: *mut u8, mut param_2: u32, mut param_3: i16) {
    let mut in_register_0000000a: u16;
    let mut puVar1: *mut u32;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000fffa: u16;

    (param_2 + 0x1e) = param_3;
    if (param_3 == 0) {
        puVar1 = mixed_1010_20ba(
            CONCAT22(in_register_0000000a, param_1),
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fffa, 0x2e),
            in_stack_0000fea2,
            in_stack_0000ffc6,
            in_stack_0000ffcc,
            in_stack_0000ffd0,
        );
        pass1_1018_209c(puVar1);
    }
    return;
}
