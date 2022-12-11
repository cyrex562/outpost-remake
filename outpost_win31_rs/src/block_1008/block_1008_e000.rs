pub unsafe fn pass1_1008_e01c(mut param_1: u32, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x1a) = param_2;
    return;
}
pub unsafe fn pass1_1008_e038(mut param_1: u32, param_2: *mut u32, param_3: *mut u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x16);
    *param_2 = (param_1 + 0x1a);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_e05e(
    param_1: *mut astruct_102,
    mut param_2: u16,
    param_3: *mut c_char,
    param_4: *mut c_char,
) {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut lVar3: i32;
    let mut uVar4: u32;
    let mut local_122: [u8; 0x112] = [0; 0x112];
    let mut iStack16: i16;
    let mut local_e: [u8; 0x8] = [0; 0x8];

    lVar3 = pass1_1008_e8cc(param_1, param_3, param_4);
    uVar1 = (lVar3 >> 0x10);
    iVar2 = lVar3;
    if (lVar3 != 0) {
        uVar4 = pass1_1030_8326();
        (iVar2 + 0xe) = uVar4;
        (iVar2 + 0x10) = (uVar4 >> 0x10);
        (iVar2 + 0xc) = param_2;
    }
    pass1_1008_5784(CONCAT22(0x1050, local_e), (param_1 + 0xa));
    iStack16 = 0;
    loop {
        lVar3 = pass1_1008_5b12(CONCAT22(0x1050, local_e));
        // if (lVar3 == 0) goto LAB_1008_e0d3;
        if ((lVar3 + 0xc) != 1) == false {
            break;
        }
    }
    iStack16 = 0x1; //
                    // LAB_1008_e0d3:
    if (iStack16 == 0) {
        struct_1030_e2be(CONCAT22(0x1050, local_122), 0x0, 0x0, 0x0);
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_122));
    }
    return;
}

pub unsafe fn pass1_1008_e10c(
    param_1: *mut astruct_102,
    param_2: *mut c_char,
    param_3: *mut c_char,
    mut param_4: i16,
    mut param_5: u16,
) -> i16 {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u32;

    uVar3 = pass1_1008_e8cc(param_1, param_2, param_3);
    if (uVar3 == 0) {
        return 0x1;
    }
    iVar1 = (uVar3 + 0xc);
    if (-0x1 < iVar1) {
        if (iVar1 < 0x2) {
            return 0x1;
        }
        if ((0x0 < iVar1 + -1) && (iVar2 = iVar1 + -0x3, iVar2 == 0x0 || iVar1 + -0x2 < 1)) {
            pass1_1008_e9a4(param_1, (param_1 >> 0x10), uVar3);
            return iVar2;
        }
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_e164(param_1: *mut astruct_102) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar5: u16;
    let mut paVar2: *mut astruct_216;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut paVar4: *mut Struct57;
    let mut uVar11: *mut astruct_102;
    let mut iVar12: *mut astruct_213;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uStack292: u32;
    let mut uStack288: u32;
    let mut uStack284: u32;
    let mut local_118: [u8; 0x112] = [0; 0x112];
    let mut lStack6: i32;
    let mut iVar1: *mut astruct_216;

    uVar7 = (in_EDX >> 0x10);
    uVar6 = (param_1 >> 0x10);
    uVar11 = param_1;
    // WARNING: Load size is inaccurate
    // WARNING: Load size is inaccurate
    lStack6 = pass1_1008_e8cc(param_1, uVar11.field25_0x1a, uVar11.field22_0x16);
    uVar3 = (lStack6 >> 0x10);
    uVar5 = lStack6;
    paVar4 = CONCAT22(uVar7, uVar3 | uVar5);
    if (lStack6 == 0) {
        // WARNING: Load size is inaccurate
        pass1_1008_e852(uVar3 | uVar5, uVar11, uVar6, uVar11.field22_0x16);
        uStack288 = CONCAT22(paVar4, uVar5);
        // WARNING: Load size is inaccurate
        pass1_1008_e852(paVar4, uVar11, uVar6, uVar11.field25_0x1a);
        uStack292 = CONCAT22(paVar4, uVar5);
        mem_op_1000_179c(0x14, paVar4);
        uVar3 = paVar4 | uVar5;
        if (uVar3 == 0) {
            uVar5 = 0;
            uVar3 = 0;
        } else {
            struct_1008_dc90(CONCAT22(paVar4, uVar5), uStack292, uStack288);
        }
        lStack6 = CONCAT22(uVar3, uVar5);
        (uVar5 + 0xc) = 0x1;
        uVar8 = pass1_1030_8326();
        uVar7 = (lStack6 >> 0x10);
        iVar12 = lStack6;
        iVar12.field14_0xe = uVar8;
        iVar12.field15_0x10 = (uVar8 >> 0x10);
        ppcVar1 = (*&uVar11.field_0xa + 0x4);
        (**ppcVar1)(0x1030, &uVar11.field_0xa, iVar12, uVar7);
    } else {
        iVar1 = (uVar5 + 0xc);
        paVar2 = iVar1 + -0x1;
        if (paVar2.is_null()) {
            return;
        }
        if (((0x0 < paVar2) && (!SBORROW2(paVar2, 1))) && ((iVar1 + -0x2) < 0x2)) {
            (uVar5 + 0x12) = 0x1;
        }
        (uVar5 + 0xc) = 0x1;
    }
    uVar7 = (lStack6 >> 0x10);
    struct_1030_e2be(
        CONCAT13(0x10, CONCAT12(0x50, local_118)),
        0x1,
        (lStack6 + 0x8),
        (lStack6 + 0x4),
    );
    uVar8 = pass1_1030_8326();
    uStack284 = CONCAT22((uVar8 >> 0x10) + (0xfffe < uVar8), uVar8 + 1);
    pass1_1030_8372(_u16_1050_5748, uStack284, CONCAT22(0x1050, local_118));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_e2a4(
    param_1: *mut astruct_102,
    param_2: *mut c_char,
    param_3: *mut c_char,
) -> u16 {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut string_2: *mut c_char;
    let mut lVar3: i32;
    let mut string_1: *mut c_char;

    string_1 = param_2;
    string_2 = load_string_1010_847e(_u16_1050_14cc, 0x531);
    iVar1 = pass1_1000_3d7a(string_2, string_1);
    if ((iVar1 == 0) || (iVar1 = pass1_1000_3d7a(param_3, param_2), iVar1 == 0)) {
        return 0x0;
    }
    lVar3 = pass1_1008_e8cc(param_1, param_2, param_3);
    if (lVar3 != 0) {
        iVar1 = (lVar3 + 0xc);
        iVar2 = iVar1 + -0x1;
        if (iVar2 == 0) {
            return 0x2;
        }
        if (iVar2 < 1) {
            return 0x0;
        }
        if (SBORROW2(iVar2, 1)) {
            return 0x0;
        }
        if (0x1 < iVar1 + -0x2) {
            return 0x0;
        }
    }
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_e320(
    param_1: *mut astruct_102,
    param_2: *mut c_char,
    param_3: *mut c_char,
) {
    let mut uVar2: i16;
    let mut uVar1: u16;
    let mut uVar3: u16;
    let mut struct_1: *mut astruct_102;
    let mut struct_1_hi: *mut astruct_102;
    let mut string_1: *mut c_char;
    let mut lVar4: i32;
    let mut uStack12: u16;
    let mut string_2: *mut c_char;

    struct_1_hi = (param_1 >> 0x10);
    struct_1 = param_1;
    fn_ptr_1000_17ce(*&struct_1.field28_0x1e);
    struct_1.field28_0x1e = 0;
    string_2 = param_2;
    string_1 = load_string_1010_847e(_u16_1050_14cc, 0x531);
    uVar3 = (string_1 >> 0x10);
    uVar2 = pass1_1000_3d7a(string_1, string_2);
    if ((uVar2 == 0) || (uVar2 = pass1_1000_3d7a(param_3, param_2), uVar2 == 0)) {
        uStack12 = 0x443;
    } else {
        lVar4 = pass1_1008_e8cc(param_1, param_2, param_3);
        uVar1 = (lVar4 >> 0x10);
        uVar2 = lVar4;
        uVar3 = uVar1 | uVar2;
        if (uVar3 == 0) {
            uStack12 = 0x444;
        } else {
            uStack12 = 0x443;
            uVar1 = (uVar2 + 0xc);
            uVar2 = uVar1;
            if (uVar1 != 0) {
                uVar2 = uVar1 - 0x1;
                if (uVar2 == 0) {
                    uStack12 = 0x445;
                    // TODO: goto LAB_1008_e378;
                }
                uVar2 = uVar1 - 0x2;
                if (uVar2 != 0) {
                    uVar2 = uVar1 - 0x3;
                    if (uVar2 == 0) {
                        uStack12 = 0x446;
                    }
                    // TODO: goto LAB_1008_e378;
                }
            }
            uStack12 = 0x444;
        }
    } //
      // LAB_1008_e378:
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), uStack12);
    struct_1.field28_0x1e = uVar2;
    struct_1.field29_0x20 = uVar3;
    return;
}
pub unsafe fn pass1_1008_e3ec(param_1: *mut astruct_218, param_2: *mut u32, param_3: *mut u32) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut pstruct92_1: *mut astruct_92;
    let mut puVar3: *mut u32;
    let mut puVar4: *mut astruct_92;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_EDX: u32;
    let mut paVar6: *mut Struct57;
    let mut iVar10: *mut astruct_218;
    let mut uVar9: u16;
    let mut struct92_1: *mut astruct_92;
    let mut uVar7: u32;
    let mut uVar8: u32;

    uVar9 = (param_1 >> 0x10);
    iVar10 = param_1;
    // WARNING: Load size is inaccurate
    puVar3 = iVar10.field14_0xe;
    uVar4 = (&iVar10.field14_0xe + 2);
    paVar6 = (in_EDX & 0xffff0000 | uVar4);
    if ((uVar4 | puVar3) != 0) {
        ppcVar2 = *puVar3;
        (**ppcVar2)();
    }
    mem_op_1000_179c(0x18, paVar6);
    uVar4 = paVar6 | puVar3;
    uVar8 = paVar6 & 0xffff0000;
    uVar7 = uVar8 | uVar4;
    if (uVar4 == 0) {
        puVar3 = NULL;
    } else {
        struct_op_1030_1cd8(
            CONCAT13((paVar6 >> 0x8), CONCAT12(paVar6, puVar3)),
            0x5,
            0x5,
        );
        uVar8 = uVar7;
    }
    iVar10.field14_0xe = puVar3;
    (&iVar10.field14_0xe + 0x2) = uVar8;
    pass1_1028_dc52(CONCAT22(0x1050, &struct92_1), 0x1, 0x0, 0x400);
    loop {
        pstruct92_1 = &struct92_1;
        pass1_1028_e4ec(CONCAT22(0x1050, pstruct92_1));
        uVar4 = uVar8;
        uVar5 = uVar4 | pstruct92_1;
        uVar7 = uVar8 & 0xffff0000;
        uVar8 = uVar7 | uVar5;
        if (uVar5 == 0) {
            break;
        }
        if (pstruct92_1[0x1c].field4_0x8 != 0x8000002) {
            puVar1 = iVar10.field14_0xe;
            ppcVar2 = (*iVar10.field14_0xe + 0xc);
            (**ppcVar2)(0x28, puVar1, (puVar1 >> 0x10));
        }
    }
    *param_3 = iVar10.field14_0xe;
    uVar4 = (&iVar10.field15_0x12 + 2);
    puVar3 = iVar10.field15_0x12;
    uVar5 = uVar4 | puVar3;
    paVar6 = (uVar7 | uVar5);
    if (uVar5 != 0) {
        ppcVar2 = *puVar3;
        (**ppcVar2)(0x1028, puVar3);
    }
    mem_op_1000_179c(0x18, paVar6);
    uVar4 = paVar6 | puVar3;
    if (uVar4 == 0) {
        puVar3 = NULL;
        uVar4 = 0;
    } else {
        struct_op_1030_1cd8(
            CONCAT13((paVar6 >> 0x8), CONCAT12(paVar6, puVar3)),
            0x5,
            0x5,
        );
    }
    iVar10.field15_0x12 = puVar3;
    (&iVar10.field15_0x12 + 0x2) = uVar4;
    if (struct92_1.field6_0x10 != 0) {
        struct92_1.field5_0xc = 0x1;
        struct92_1.field5_0xc = 0;
    }
    uVar8 = struct92_1.field5_0xc;
    struct92_1.field4_0x8 = struct92_1.field5_0xc;
    struct92_1.field4_0x8 = struct92_1.field5_0xc;
    loop {
        uVar4 = uVar8;
        puVar4 = &struct92_1;
        pass1_1028_e4ec(CONCAT22(0x1050, puVar4));
        uVar8 = (uVar4 | puVar4);
        if ((uVar4 | puVar4) == 0) {
            break;
        }
        puVar1 = iVar10.field15_0x12;
        ppcVar2 = (*iVar10.field15_0x12 + 0xc);
        (**ppcVar2)(0x28, puVar1, (puVar1 >> 0x10));
    }
    *param_2 = iVar10.field15_0x12;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn string_1008_e586(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u16,
) -> u32 {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut in_string_2: *mut c_char;
    let mut pcStack10: *mut c_char;
    let mut pcStack6: *mut c_char;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3);
    pcStack6 = CONCAT22(param_5, param_4);
    paVar1 = CONCAT22(in_register_0000000a, param_5 | param_4);
    if ((param_5 | param_4) == 0) {
        return 0x0;
    }
    mem_op_1000_179c(0x80, paVar1);
    pcStack10 = CONCAT22(paVar1, param_4);
    in_string_2 = pass1_1038_4d28(pcStack6);
    unk_str_op_1000_3d3e(pcStack10, in_string_2);
    return CONCAT22(paVar1, param_4);
}
pub unsafe fn pass1_1008_e5da(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut puVar3: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000ffba: HFILE16;
    let mut local_30: [u32; 0x2] = [0; 0x2];
    let mut local_28: u32;
    let mut local_24: [u32; 0x2] = [0; 0x2];
    let mut local_1c: [u16; 0x3] = [0; 0x3];
    let mut local_16: [u16; 0x3] = [0; 0x3];
    let mut uStack16: u32;
    let mut local_c: [u8; 0x8] = [0; 0x8];
    let mut uStack4: u16;

    BVar2 = write_to_file_1008_7cac(param_2);
    if (BVar2 != 0) {
        uVar5 = (param_1 >> 0x10);
        iVar4 = param_1;
        if ((iVar4 + 0xa) == 0) {
            uStack4 = 0;
        } else {
            uVar1 = (iVar4 + 0xa);
            uStack4 = (uVar1 + 0x8);
        }
        local_1c[0] = uStack4;
        BVar2 =
            write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_1c), 0x2, in_stack_0000ffba);
        if (BVar2 != 0) {
            pass1_1008_5784(CONCAT22(0x1050, local_c), (iVar4 + 0xa));
            loop {
                puVar3 = local_c;
                pass1_1008_5b12(CONCAT22(0x1050, puVar3));
                uStack16 = CONCAT22(extraout_DX, puVar3);
                if ((extraout_DX | puVar3) == 0) {
                    return;
                }
                local_24[0] = (puVar3 + 0x4);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_24),
                    0x4,
                    in_stack_0000ffba,
                );
                if (BVar2 == 0) {
                    break;
                }
                local_28 = (uStack16 + 0x8);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, &local_28),
                    0x4,
                    in_stack_0000ffba,
                );
                if (BVar2 == 0) {
                    break;
                }
                local_16[0] = (uStack16 + 0xc);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_16),
                    0x2,
                    in_stack_0000ffba,
                );
                if (BVar2 == 0) {
                    break;
                }
                local_30[0] = (uStack16 + 0xe);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_30),
                    0x4,
                    in_stack_0000ffba,
                );
                if (BVar2 == 0) {
                    break;
                }
                local_16[0] = (uStack16 + 0x12);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_16),
                    0x2,
                    in_stack_0000ffba,
                );
                if BVar2 == 0 {
                    break;
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}
pub unsafe fn file_1008_e70e(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut local_12: [u16; 0x2] = [0; 0x2];
    let mut puStack14: *mut u32;
    let mut uStack10: u16;
    let mut local_4: u16;
    let mut paVar6: *mut Struct57;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    if (u16_1050_0312 < 0x2) {
        return;
    }
    read_file_1008_7cfe(param_4, (param_4 >> 0x10), 0x14);
    if (param_1 != 0) {
        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar2 != 0) {
            if (local_4 == 0) {
                return;
            }
            uStack10 = 0;
            loop {
                if (local_4 <= uStack10) {
                    return;
                }
                uVar3 = local_4;
                mem_op_1000_179c(0x14, paVar5);
                uVar4 = paVar5 | uVar3;
                paVar6 = (paVar5 & 0xffff0000 | uVar4);
                if (uVar4 == 0) {
                    uVar3 = 0;
                    paVar5 = (paVar5 & 0xffff0000);
                } else {
                    struct_1008_dcdc(CONCAT22(paVar5, uVar3));
                    paVar5 = paVar6;
                }
                puStack14 = CONCAT22(paVar5, uVar3);
                BVar2 = read_file_1008_7dee(param_4, CONCAT22(paVar5, uVar3 + 0x4), 0x4);
                if ((((BVar2 == 0)
                    || (
                        BVar2 = read_file_1008_7dee(
                            param_4,
                            (puStack14 & 0xffff0000 | (puStack14 + 0x8)),
                            0x4,
                        ),
                        BVar2 == 0,
                    ))
                    || (
                        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_12), 0x2),
                        BVar2 == 0,
                    ))
                    || ((
                        BVar2 = read_file_1008_7dee(
                            param_4,
                            (puStack14 & 0xffff0000 | (puStack14 + 0xe)),
                            0x4,
                        ),
                        BVar2 == 0x0
                            || (
                                BVar2 = read_file_1008_7dee(
                                    param_4,
                                    (puStack14 & 0xffff0000 | (puStack14 + 0x12)),
                                    0x2,
                                ),
                                BVar2 == 0,
                            ),
                    )))
                {
                    break;
                }
                (puStack14 + 0xc) = local_12[0];
                ppcVar1 = ((param_3 + 0xa) + 0x4);
                (**ppcVar1)();
                uStack10 += 0x1;
            }
            if (puStack14.is_null() == false) {
                ppcVar1 = *puStack14;
                (**ppcVar1)(0x1000, puStack14, (puStack14 >> 0x10), 0x1, puStack14);
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}
pub unsafe fn pass1_1008_e852(mut param_1: u16, mut param_2: u16, mut param_3: u16, param_4: *mut c_char) {
    let mut pstruct92_1: *mut astruct_92;
    let mut iVar1: i16;
    let mut pcVar2: *mut c_char;
    let mut struct92_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &struct92_14), 0x1, 0x0, 0x400);
    loop {
        pstruct92_1 = &struct92_14;
        pass1_1028_e4ec(CONCAT22(0x1050, pstruct92_1));
        if ((param_1 | pstruct92_1) == 0) {
            return;
        }
        pcVar2 = pass1_1038_4d28(CONCAT22(param_1, pstruct92_1));
        param_1 = (pcVar2 >> 0x10);
        iVar1 = pass1_1000_3d7a(param_4, (pcVar2 & 0xffff | param_1 << 0x10));
        if iVar1 == 0 {
            break;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_e8cc(
    param_1: *mut astruct_102,
    param_2: *mut c_char,
    param_3: *mut c_char,
) -> i32 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut lVar6: i32;
    let mut pcVar7: *mut c_char;
    let mut pcVar8: *mut c_char;
    let mut pcStack22: *mut c_char;
    let mut pcStack18: *mut c_char;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0xa));
    loop {
        lVar6 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        uVar4 = (lVar6 >> 0x10);
        uVar1 = lVar6;
        uVar5 = uVar4 | uVar1;
        if (lVar6 == 0) {
            return 0x0;
        }
        uVar2 = uVar1;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (uVar1 + 0x4));
        pcStack18 = CONCAT22(uVar5, uVar2);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (uVar1 + 0x8));
        pcStack22 = CONCAT22(uVar5, uVar2);
        pcVar7 = pass1_1038_4d28(pcStack18);
        pcVar8 = pass1_1038_4d28(pcStack22);
        iVar3 = pass1_1000_3d7a(param_3, pcVar7);
        if ((iVar3 == 0) && (iVar3 = pass1_1000_3d7a(param_2, pcVar8), iVar3 == 0)) {
            break;
        }
        iVar3 = pass1_1000_3d7a(param_2, pcVar7);
        if ((iVar3 == 0) && (iVar3 = pass1_1000_3d7a(param_3, pcVar8), iVar3 == 0)) {
            return lVar6;
        }
    }
    return lVar6;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_e9a4(
    param_1: *mut astruct_102,
    param_2: *mut astruct_102,
    mut param_3: u32,
) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut in_stack_0000fe90: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffe8: u16;
    let mut iStack20: i16;
    let mut uStack16: u32;
    let mut uStack6: u32;

    uVar5 = (in_EDX >> 0x10);
    uVar8 = pass1_1030_8326();
    uVar7 = (param_3 >> 0x10);
    iVar6 = param_3;
    puVar1 = (iVar6 + 0xe);
    uVar2 = uVar8 - *puVar1;
    iVar4 = ((uVar8 >> 0x10) - (iVar6 + 0x10)) - (uVar8 < *puVar1);
    uStack6 = CONCAT22(iVar4, uVar2);
    mixed_1010_20ba(
        CONCAT22(uVar5, iVar4),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x2),
        in_stack_0000fe90,
        in_stack_0000ffb4,
        in_stack_0000ffba,
        in_stack_0000ffbe,
    );
    uStack16 = 0;
    //   if ((PTR_LOOP_1050_13ae < 1) || (SBORROW2(PTR_LOOP_1050_13ae,1))) goto LAB_1008_ea2b;
    if (PTR_LOOP_1050_13ae == &u16_1050_0002 || (PTR_LOOP_1050_13ae + -1) < 1) {
        if ((iVar6 + 0x12) == 0) {
            //
            // LAB_1008_ea20:
            uVar3 = 0x1e;
        } else {
            uVar3 = 0xa;
        }
    } else if (PTR_LOOP_1050_13ae == (&u16_1050_0002 + 1)) {
        if ((iVar6 + 0x12) == 0) {
            uVar3 = 0x28;
        } else {
            uVar3 = 0x14;
        }
    } else {
        // if (PTR_LOOP_1050_13ae != &u32_1050_0004) goto LAB_1008_ea2b;
        // if ((iVar6 + 0x12) != 0) goto LAB_1008_ea20;
        uVar3 = 0x32;
    }
    uStack16 = uVar3; //
                      // LAB_1008_ea2b:
    if (uStack16 < uStack6) {
        pass1_1008_612e(uVar2, 0x1, 0x64);
        iStack20 = 0;
        iVar4 = (iVar6 + 0xc);
        if (iVar4 == 0x2) {
            iStack20 = 0x32;
        } else if (iVar4 == 0x3) {
            iStack20 = 0x19;
        }
        if (uStack6 < iStack20) {
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1008_ea86(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1008_ddca(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_eabc(param_1: *mut astruct_19, mut param_2: u16) -> *mut u16 {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    pass1_1008_3e38((param_1 & 0xffff0000 | (param_1 + 0xc)));
    param_1.offset_0x0 = 0xeb1a;
    (param_1 + 0x2) = 0x1008;
    return &param_1.offset_0x0;
}

pub unsafe fn pass1_1008_eaf4(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn pass1_1008_eb2a(param_1: *mut astruct_19, mut param_2: u16) {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xc) = 0;
    param_1.offset_0x0 = 0xec00;
    (param_1 + 0x2) = 0x1008;
    return;
}

pub unsafe fn pass1_1008_eb5c(mut param_1: u16, mut param_2: u16, mut param_3: i16) -> u32 {
    return CONCAT22(0x1050, param_3 * 0x10 + 0xd0e);
}

pub unsafe fn pass1_1008_eb6e() -> u16 {
    return 0x5;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_eb74(param_1: *mut u8, mut param_2: u32, mut param_3: i16) {
    let mut in_register_0000000a: u16;
    let mut unaff_SI: u16;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;

    (param_2 + 0xa) = param_3;
    if (param_3 != 0) {
        mixed_1010_20ba(
            CONCAT22(in_register_0000000a, param_1),
            _u16_1050_0ed0,
            CONCAT22(unaff_SI, 0x3),
            in_stack_0000fe98,
            in_stack_0000ffbc,
            in_stack_0000ffc2,
            in_stack_0000ffc6,
        );
        pass1_1010_c312();
    }
    return;
}

pub unsafe fn pass1_1008_ebda(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ec10(mut param_1: i16, mut param_2: u16, mut param_3: u16) -> *mut u16 {
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa) = 0;
    CONCAT22(param_2, param_1) = 0xec62;
    (param_1 + 0x2) = 0x1008;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1008_ec3c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn struct_1008_ec72(param_1: *mut astruct_223) -> *mut astruct_223 {
    struct_1010_383a(param_1);
    param_1.field0_0x0 = 0xefc4;
    (param_1 + 0x2) = 0x1008;
    return param_1;
}
pub unsafe fn pass1_1008_ec94(param_1: *mut StructD) {
    param_1.address_offset_field_0x0 = 0xefc4;
    (param_1 + 0x2) = 0x1008;
    pass1_1010_3880(param_1);
    return;
}

pub unsafe fn struct_1008_ecb2(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_19,
    mut param_4: u16,
) -> *mut astruct_19 {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;

    paVar1 = CONCAT22(in_register_0000000a, param_2);
    struct_1010_2cd2(param_3, param_4);
    param_3.offset_0x0 = 0xef9c;
    (param_3 + 0x2) = 0x1008;
    mem_op_1000_179c(0x20c, paVar1);
    (param_3 + 0x5c) = param_1;
    (param_3 + 0x5e) = paVar1;
    pass1_1000_4906(CONCAT22(paVar1, (param_3 + 0x5c)), NULL, 0x20c);
    return param_3;
}
pub unsafe fn pass1_1008_ed00(param_1: *mut u16) {
    *param_1 = 0xef9c;
    (param_1 + 0x2) = 0x1008;
    pass1_1010_2db2(param_1);
    return;
}
pub unsafe fn mem_1008_ed1e(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;

    paVar1 = CONCAT22(in_register_0000000a, param_5);
    if (param_3 != 0) {
        mem_op_1000_179c(param_3 << 0x2, paVar1);
        return;
    }
    mem_op_1000_179c(0x1a, paVar1);
    if ((paVar1 | param_4) != 0) {
        struct_1008_ec72(CONCAT22(paVar1, param_4));
        return;
    }
    return;
}
pub unsafe fn pass1_1008_ed62(mut param_1: u32, mut param_2: i16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x16) = param_2 * 0x8 + 0xd5e;
    (iVar1 + 0x18) = &DAT_1050_1050;
    (iVar1 + 0x12) = (param_2 * 0x8 + 0xd64);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_ed8a(
    param_1: u32,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
    mut param_5: i16,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut cVar4: u8;
    let mut in_EDX: *mut Struct57;
    let mut uVar5: u16;
    let mut unaff_SI: u16;
    let mut uVar6: u16;
    let mut bVar7: bool;
    let mut uVar8: u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;

    if (0x0 < param_4) {
        if (_PTR_LOOP_1050_0df6.is_null()) {
            ppcVar1 = (*param_1 + 0x18);
            uVar3 = (**ppcVar1)();
            _PTR_LOOP_1050_0df6 = mixed_1010_20ba(
                in_EDX,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, uVar3),
                in_stack_0000fe9a,
                in_stack_0000ffbe,
                in_stack_0000ffc4,
                in_stack_0000ffc8,
            );
        }
        uVar2 = (param_1 + 0xc);
        uVar8 = pass1_1010_2e02(_PTR_LOOP_1050_0df6, (uVar2 + 0x12));
        uVar5 = param_2 + 1;
        uVar6 = param_3 + (0xfffe < param_2);
        // for (cVar4 = (param_4 + -1) * '\x04'; cVar4 != '\0'; cVar4 += -1)
        for cVar4 in (param_4 - 1) * 0x4..0 {
            bVar7 = CARRY2(uVar5, uVar5);
            uVar5 *= 0x2;
            uVar6 = uVar6 * 0x2 + bVar7;
        }
        pass1_1010_2e30(
            _PTR_LOOP_1050_0df6,
            uVar5 | uVar8,
            uVar6 | (uVar8 >> 0x10),
            (param_5 * 0x8 + 0xd64),
        );
    }
    return;
}
pub unsafe fn pass1_1008_ee14(mut param_1: u32, mut param_2: u16) {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut paVar5: *mut astruct_223;
    let mut local_1c: [u8; 0x1a] = [0; 0x1a];

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x56) == 0) {
        paVar5 = struct_1008_ec72(CONCAT22(0x1050, local_1c));
        uVar2 = (paVar5 >> 0x10);
        puVar1 = local_1c;
        pass1_1010_398e(puVar1, CONCAT22(0x1050, puVar1), 0x0, 0x0, 0x0);
        (iVar3 + 0x56) = puVar1;
        (iVar3 + 0x58) = uVar2;
        pass1_1008_ec94(CONCAT22(0x1050, local_1c));
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn load_string_1008_ee56() -> *mut c_char {
    let mut pcVar1: *mut c_char;
    let mut in_stack_00000004: u32;

    pcVar1 = load_string_1010_847e(_u16_1050_14cc, *(in_stack_00000004 + 0x16));
    return pcVar1;
}
pub unsafe fn pass1_1008_ee72(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;

    if ((param_1 + 0x56) == 0) {
        ppcVar1 = (CONCAT22(param_2, param_1) + 0x10);
        (**ppcVar1)();
    }
    uVar2 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, uVar2);
    return;
}

pub unsafe fn pass1_1008_eea6() -> u16 {
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_eeac(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) -> bool {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut cVar3: u8;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffee: u32;

    uVar5 = (param_4 + 0x12);
    puVar7 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22((in_stack_0000ffee >> 0x10), 0x3),
        in_stack_0000fe98,
        in_stack_0000ffbc,
        in_stack_0000ffc2,
        in_stack_0000ffc6,
    );
    uVar1 = (puVar7 >> 0x10);
    uVar2 = puVar7;
    uVar6 = uVar1;
    if (uVar5 == 0x7d) {
        pass1_1010_a5ca(0x7d, uVar1, uVar2, uVar1, 0x7c);
        if (uVar5 != 0) {
            return false;
        }
        pass1_1010_a5ca(0x0, uVar6, uVar2, uVar1, 0x7d);
        if (uVar5 != 0) {
            return false;
        }
        uVar4 = uVar5;
        uVar5 = 0x78;
    } else {
        uVar4 = uVar5;
        if (uVar5 < 0x7e) {
            cVar3 = uVar5;
            uVar4 = uVar5 & 0xff00;
            if ((cVar3 + 0x8d) == 0) {
                uVar5 = 0x9;
                uVar4 = uVar4 | (cVar3 + 0x8d);
            } else if ((cVar3 + 0x89) == 0) {
                uVar5 = 0x2e;
                uVar4 = uVar4 | (cVar3 + 0x89);
            } else {
                uVar4 |= (cVar3 + 0x87);
                if ((cVar3 + 0x87) == 0) {
                    uVar5 = 0x5b;
                }
            }
        }
    }
    pass1_1010_a5ca(uVar4, uVar6, uVar2, uVar1, uVar5);
    return uVar4 == 0;
}

pub unsafe fn pass1_1008_ef38(mut param_1: u32) -> u16 {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0x16);
    return (uVar1 + 2);
}

pub unsafe fn pass1_1008_ef4a() -> u16 {
    return 0x41;
}

pub unsafe fn pass1_1008_ef50(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_ec94(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ef76(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_ed00(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
