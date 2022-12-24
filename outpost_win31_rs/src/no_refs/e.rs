
pub unsafe fn pass1_1008_bd02(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_afde(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd28(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_b08c(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd4e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_b08c(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd74(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_b08c(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd9a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_b08c(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_d6f4(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1008_caa0(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_d75a(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_d968(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_d7da(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn pass1_1008_d9d4(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    clenaup_win_ui_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_dc5a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_dc2c(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}



pub unsafe fn pass1_1008_dc80(
    mut param_1: u16,
    mut param_2: i16,
    param_3: u8,
    param_4: u8,
    mut param_5: u16,
    param_6: *mut u16,
    mut param_7: u32,
    mut param_8: u32,
    mut param_9: u16,
) {
    let mut pcVar1: *mut c_char;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut pcVar4: *mut code;
    let mut uVar5: u16;
    let mut cVar6: u8;
    let mut extraout_DL: u8;
    let mut bVar7: u8;
    let mut iVar8: i16;
    let mut unaff_SI: i16;
    let mut uVar9: u16;
    let mut bVar10: u8;

    bVar7 = (param_9 >> 0x8);
    bVar10 = param_9 + bVar7;
    cVar6 = bVar10 + param_3;
    uVar2 = (CARRY1(param_9, bVar7) || CARRY1(bVar10, param_3));
    uVar3 = param_1 + 0xeff0;
    bVar10 = param_1 < 0x1010 || uVar3 < uVar2;
    uVar5 = uVar3 - uVar2;
    pcVar4 = swi(0x4);
    if (SBORROW2(param_1, 0x1010) != SBORROW2(uVar3, uVar2)) {
        (*pcVar4)();
        cVar6 = extraout_DL;
    }
    pcVar1 = (param_2 + unaff_SI);
    *pcVar1 = *pcVar1 + cVar6 + (uVar5 < 0x1010 || uVar5 + 0xeff0 < bVar10);
    uVar9 = (param_6 >> 0x10);
    iVar8 = param_6;
    *param_6 = 0x389a;
    (iVar8 + 0x2) = 0x1008;
    (iVar8 + 0x4) = param_8;
    (iVar8 + 0x8) = param_7;
    (iVar8 + 0xc) = 0;
    (iVar8 + 0xe) = 0;
    (iVar8 + 0x12) = 0;
    *param_6 = 0xdd4a;
    (iVar8 + 0x2) = 0x1008;
    return;
}


pub unsafe fn pass1_1008_dd1e(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
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


pub unsafe fn pass1_1008_ea86(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1008_ddca(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_eaf4(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_ebda(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_ec3c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
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
        // for (cVar4 = (param_4 -1) * '\x04'; cVar4 != '\0'; cVar4 += -1)
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


pub unsafe fn pass1_1010_02a2(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_0052(param_1, &DAT_1050_1050);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1010_0ad2(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut puVar3: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000ffbc: HFILE16;
    let mut local_2a: [u32; 0x2] = [0; 0x2];
    let mut local_22: [u16; 0x2] = [0; 0x2];
    let mut local_1e: [u16; 0x3] = [0; 0x3];
    let mut local_18: [u16; 0x3] = [0; 0x3];
    let mut uStack18: u32;
    let mut local_e: [u8; 0x8] = [0; 0x8];
    let mut uStack6: u16;
    let mut iStack4: i16;

    BVar2 = write_to_file_1008_7cac(param_2);
    if (BVar2 == 0) {
        return;
    }
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0xa) == 0) {
        uStack6 = 0;
    } else {
        uVar1 = (iVar4 + 0xa);
        uStack6 = (uVar1 + 0x8);
    }
    local_1e[0] = uStack6;
    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_1e), 0x2, in_stack_0000ffbc);
    if (BVar2 != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_e), (iVar4 + 0xa));
        loop {
            puVar3 = local_e;
            pass1_1008_5b12(CONCAT22(0x1050, puVar3));
            uStack18 = CONCAT22(extraout_DX, puVar3);
            if ((extraout_DX | puVar3) == 0) {
                local_22[0] = (iVar4 + 0xe);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_22),
                    0x2,
                    in_stack_0000ffbc,
                );
                if (BVar2 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
                local_22[0] = (iVar4 + 0x10);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_22),
                    0x2,
                    in_stack_0000ffbc,
                );
                if (BVar2 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
                if ((iVar4 + 0x18) != 0) {
                    DAT_1050_0e28 = (iVar4 + 0x12);
                    PTR_LOOP_1050_0e30 = (iVar4 + 0x14);
                    PTR_LOOP_1050_0ea8 = (iVar4 + 0x16);
                }
                iStack4 = 0;
                loop {
                    if (0x9 < iStack4) {
                        iStack4 = 0;
                        loop {
                            if (0x2 < iStack4) {
                                if ((iVar4 + 0x18) != 0) {
                                    DAT_1050_0e28 = 0;
                                    PTR_LOOP_1050_0e30 = null_mut();
                                    PTR_LOOP_1050_0ea8 = null_mut();
                                }
                                return;
                            }
                            local_1e[0] = (iStack4 * 0x8 + 0xea8);
                            BVar2 = write_to_file_1008_7e1c(
                                param_2,
                                CONCAT22(0x1050, local_1e),
                                0x2,
                                in_stack_0000ffbc,
                            );
                            if (BVar2 == 0) {
                                break;
                            }
                            iStack4 += 0x1;
                        }
                        u16_1050_0310 = 0x6d0;
                        return;
                    }
                    local_1e[0] = (iStack4 * 0x8 + 0xe28);
                    BVar2 = write_to_file_1008_7e1c(
                        param_2,
                        CONCAT22(0x1050, local_1e),
                        0x2,
                        in_stack_0000ffbc,
                    );
                    if (BVar2 == 0) {
                        break;
                    }
                    iStack4 += 0x1;
                }
                u16_1050_0310 = 0x6d0;
                return;
            }
            local_18[0] = (puVar3 + 0x4);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_18),
                0x2,
                in_stack_0000ffbc,
            );
            if (BVar2 == 0) {
                u16_1050_0310 = 0x6d0;
                return;
            }
            local_2a[0] = (uStack18 + 0x6);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_2a),
                0x4,
                in_stack_0000ffbc,
            );
            if BVar2 == 0 {
                break;
            }
        }
    }
    u16_1050_0310 = 0x6d0;
    return;
}


pub unsafe fn file_1010_0c7c(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_228,
    mut param_4: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar4: *mut astruct_229;
    let mut uVar3: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar6: *mut astruct_228;
    let mut uVar7: u16;
    let mut local_2a: [u16; 0x2] = [0; 0x2];
    let mut uStack38: u16;
    let mut puStack26: *mut u32;
    let mut puStack22: *mut u32;
    let mut local_12: [u16; 0x5] = [0; 0x5];
    let mut paStack8: *mut astruct_229;
    let mut local_6: *mut astruct_229;
    let mut uStack4: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    uVar7 = (param_4 >> 0x10);
    read_file_1008_7cfe(param_4, uVar7, 0x6);
    if (param_1 == 0) {
        u16_1050_0310 = 0x6d4;
    } else {
        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_6), 0x2);
        paStack8 = null_mut();
        iVar6 = param_3;
        if (BVar2 != 0) {
            //   for (paStack8 = null_mut(); iVar6 = param_3, paStack8 < local_6;
            //   paStack8 = &paStack8.field_1)
            while paStack8 < local_6 {
                uVar4 = local_6;
                mem_op_1000_179c(0xa, paVar6);
                uVar5 = paVar6;
                puStack26 = CONCAT22(uVar5, uVar4);
                paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar4));
                if ((uVar5 | uVar4) == 0) {
                    puStack22 = null_mut();
                } else {
                    puStack26 = 0x389a;
                    uVar4.field2_0x2 = 0x1008;
                    puStack26 = 0xea8;
                    uVar4.field2_0x2 = 0x1010;
                    puStack22 = puStack26;
                }
                BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_12), 0x2);
                if ((BVar2 == 0)
                    || (
                        BVar2 = read_file_1008_7dee(
                            param_4,
                            (puStack22 & 0xffff0000 | (puStack22 + 0x6)),
                            0x4,
                        ),
                        BVar2 == 0,
                    ))
                {
                    puStack26 = puStack22;
                    if (puStack22.is_null() == false) {
                        ppcVar1 = *puStack22;
                        (**ppcVar1)(0x1008, puStack22, (puStack22 >> 0x10), 1);
                    }
                    // TODO: goto LAB_1010_0cb1;
                }
                (puStack22 + 0x4) = local_12[0];
                ppcVar1 = (*iVar6.field10_0xa + 0x8);
                (**ppcVar1)();
                paStack8 = paStack8.field_1;
            }
            BVar2 = read_file_1008_7dee(
                param_4,
                (param_3 & 0xffff0000 | ZEXT24(&iVar6.field_0xe)),
                0x2,
            );
            if ((BVar2 != 0)
                && (
                    BVar2 = read_file_1008_7dee(
                        param_4,
                        (param_3 & 0xffff0000 | ZEXT24(&iVar6.field_0x10)),
                        0x2,
                    ),
                    BVar2 != 0,
                ))
            {
                // for (uStack4 = 0; uStack4 < 0xa; uStack4 += 1)
                for uStack4 in 0..0xa {
                    BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_2a), 0x2);
                    //   if (BVar2 == 0) goto LAB_1010_0cb1;
                    uVar3 = uStack4;
                    if (u16_1050_0312 < 0x2) {
                        uVar3 = pass1_1008_738c(param_4, uVar7, uStack4);
                    }
                    (uVar3 * 0x8 + 0xe28) = local_2a[0];
                    uStack38 = uVar3;
                }
                if (0x2 < u16_1050_0312) {
                    uStack4 = 0;
                    loop {
                        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_2a), 0x2);
                        // if (BVar2 == 0) goto LAB_1010_0cb1;
                        (uStack4 * 0x8 + 0xea8) = local_2a[0];
                        uStack4 += 0x1;
                        if uStack4 >= 3 {
                            break;
                        }
                    }
                }
                return;
            }
        } //
          // LAB_1010_0cb1:
        u16_1050_0310 = 0x6d2;
    }
    return;
}
