
pub fn pass1_1008_6b5a(param_1: *mut astruct_458, param_2: u8) -> *mut u16 {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: *mut astruct_458;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    uVar4 = param_1;
    param_1 = 0x6c8c;
    uVar4.field2_0x2 = 0x1008;
    puVar1 = uVar4.field3_0x4;
    uVar2 = uVar4.field4_0x6;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    param_1 = 0x389a;
    uVar4.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1008_6bb4(param_1: *mut astruct_459, param_2: u8) {
    let mut uVar1: *mut astruct_459;
    let mut uVar2: u16;

    uVar1 = param_1;
    uVar1 = uVar1 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2 = (param_1 >> 0x10);
    param_1 = 0x380a;
    uVar1.field2_0x2 = 0x1008;
    param_1 = 0x389a;
    uVar1.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1008_6d18(param_1: *mut u16, param_2: *mut u16, param_3: *mut u16) {
    pass1_1008_3f62(param_1, param_3);
    pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x6)), param_2);
    return;
}


pub fn pass1_1008_7e98(param_1: *mut astruct_460, param_2: u8) -> *mut u16 {
    let mut uVar1: *mut astruct_460;
    let mut uVar2: *mut astruct_460;

    uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    param_1 = 0x380a;
    uVar1.field2_0x2 = 0x1008;
    param_1 = 0x389a;
    uVar1.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1008_7ffa(param_1: *mut astruct_461, param_2: u8) {
    let mut uVar1: *mut astruct_461;
    let mut uVar2: u16;

    uVar1 = param_1;
    uVar1 = uVar1 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2 = (param_1 >> 0x10);
    param_1 = 0x380a;
    uVar1.field2_0x2 = 0x1008;
    param_1 = 0x389a;
    uVar1.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}


pub fn pass1_1008_9c16(mut param_1: u16, mut param_2: u32, mut param_3: u32) -> LRESULT {
    let mut LVar1: LRESULT;

    LVar1 = make_def_wnd_proc_1008_9ce6(
        param_1,
        param_2,
        CONCAT22(param_3, (param_2 >> 0x10)),
        (param_3 >> 0x10),
        0x85,
    );
    return LVar1;
}


pub fn pass1_1008_9c30(mut param_1: u16, mut param_2: u32, mut param_3: u32) -> LRESULT {
    let mut LVar1: LRESULT;

    LVar1 = make_def_wnd_proc_1008_9ce6(
        param_1,
        param_2,
        CONCAT22(param_3, (param_2 >> 0x10)),
        (param_3 >> 0x10),
        0x86,
    );
    return LVar1;
}

pub fn pass1_1008_9c4a() {
    return;
}
pub fn pass1_1008_9c4e() {
    return;
}
pub fn pass1_1008_9c52() {
    return;
}

pub fn pass1_1008_9c60(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut u32,
    mut param_4: i16,
) {
    let mut ppcVar1: *mut *mut code;

    if ((param_4 == 0xc7) && (param_3.is_null() == false)) {
        ppcVar1 = *param_3;
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1008_9c86(mut param_1: u32, param_2: *mut c_char, mut param_3: i16) {
    let mut uVar1: u16;

    uVar1 = str_op_1000_3da4((param_1 & 0xffff0000 | (param_1 + 0xa)));
    if (param_3 < uVar1) {
        uVar1 = param_3 - 0x1;
    }
    str_op_1000_3dbe(param_2, (param_1 & 0xffff0000 | (param_1 + 0xa)), uVar1);
    return;
}


pub fn pass1_1008_9d02(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_9f18(mut param_1: i16, mut param_2: u16, mut param_3: i16) {
    if (param_3 == 0x2) {
        pass1_1008_9f64(CONCAT22(param_2, param_1 -0x1c));
        pass1_1010_1f62(CONCAT22(param_2, param_1 -0x1c), 0x2);
    }
    return;
}


pub fn pass1_1008_9f80(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x1c));
    pass1_1008_9e5a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1008_9fb2(
    mut param_1: u16,
    mut param_2: i16,
    param_3: u8,
    param_4: u8,
    param_5: u8,
    mut param_6: u16,
    mut param_7: i16,
    mut param_8: u16,
    mut param_9: u16,
) {
    let mut pcVar1: *mut c_char;
    let mut pbVar2: *mut u8;
    let mut bVar3: u8;
    let mut pcVar4: *mut code;
    let mut bVar5: u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut bVar8: u8;
    let mut uVar9: u16;
    let mut in_register_00000009: u32;
    let mut uVar10: u32;
    let mut paVar11: *mut Struct57;
    let mut unaff_SI: i16;
    let mut bVar13: u8;
    let mut bVar14: bool;
    let mut bVar15: bool;
    let mut paVar16: *mut Struct19;
    let mut paVar12: *mut Struct57;

    uVar10 = CONCAT31(in_register_00000009, param_5);
    (param_2 + 0x1008) = 0x1050;
    uVar7 = param_4;
    uVar9 = param_1 + 0xeff0;
    bVar13 = param_1 < 0x1010 || uVar9 < uVar7;
    uVar6 = uVar9 - uVar7;
    pcVar4 = swi(0x4);
    if (SBORROW2(param_1, 0x1010) != SBORROW2(uVar9, uVar7)) {
        (*pcVar4)();
    }
    bVar5 = ((uVar6 + 0xeff0) - bVar13) % 0x1d;
    pcVar1 = (param_2 + unaff_SI);
    bVar8 = uVar10;
    *pcVar1 = *pcVar1 + bVar8 + (uVar6 < 0x1010 || uVar6 + 0xeff0 < bVar13);
    pbVar2 = (param_2 + unaff_SI);
    bVar14 = *pbVar2 < bVar8 || (*pbVar2 - bVar8) < (0xb1 < bVar5);
    *pbVar2 = (*pbVar2 - bVar8) - (0xb1 < bVar5);
    pbVar2 = (param_2 + 0x18);
    bVar15 = *pbVar2 < param_3 || (*pbVar2 - param_3) < bVar14;
    *pbVar2 = (*pbVar2 - param_3) - bVar14;
    pbVar2 = (param_2 + unaff_SI + 0x89f);
    bVar13 = *pbVar2;
    bVar3 = *pbVar2 + bVar5 + 0x4e;
    *pbVar2 = bVar3 + bVar15;
    pcVar1 = (param_2 + unaff_SI);
    *pcVar1 = *pcVar1 + param_2 + (CARRY1(bVar13, bVar5 + 0x4e) || CARRY1(bVar3, bVar15));
    pbVar2 = (param_2 + unaff_SI);
    *pbVar2 = *pbVar2 | bVar8;
    paVar16 = struct_op_1010_1d48(CONCAT22(param_8, param_7), param_9);
    paVar11 = (uVar10 & 0xffff0000 | paVar16 >> 0x10);
    uVar7 = 0;
    (param_7 + 0xa) = 0;
    (param_7 + 0x410) = 0;
    (param_7 + 0x414) = 0;
    (param_7 + 0x416) = 0;
    (param_7 + 0x418) = 0;
    (param_7 + 0x41a) = 0;
    (param_7 + 0x41c) = 0;
    (param_7 + 0x41e) = 0;
    CONCAT22(param_8, param_7) = 0xad92;
    (param_7 + 0x2) = 0x1008;
    mem_op_1000_179c(0xc, paVar11);
    uVar9 = paVar11 | uVar7;
    paVar12 = (paVar11 & 0xffff0000 | uVar9);
    if (uVar9 == 0) {
        (param_7 + 0xa) = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar11, uVar7));
        (param_7 + 0xa) = uVar7;
        (param_7 + 0xc) = paVar12;
    }
    mem_op_1000_179c(0xc, paVar12);
    uVar9 = paVar12 | uVar7;
    if (uVar9 == 0) {
        uVar7 = 0;
        uVar9 = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar12, uVar7));
    }
    (param_7 + 0x410) = uVar7;
    (param_7 + 0x412) = uVar9;
    return;
}

pub fn pass1_1008_ad0c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        pass1_1000_093a(param_1);
    }
    return param_1;
}

pub fn pass1_1008_ad38(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_ad64(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_a086(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1008_af56(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_af38(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_ba38(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut puVar3: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000ffc0: HFILE16;
    let mut local_2a: [u32; 0x3] = [0; 0x3];
    let mut local_1e: [u16; 0x5] = [0; 0x5];
    let mut local_14: [u8; 0x8] = [0; 0x8];
    let mut local_c: u16;
    let mut uStack10: u32;
    let mut local_6: [u16; 0x2] = [0; 0x2];

    BVar2 = write_to_file_1008_7cac(param_2);
    if (BVar2 != 0) {
        uVar5 = (param_1 >> 0x10);
        iVar4 = param_1;
        local_c = (iVar4 + 0x22);
        BVar2 =
            write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, &local_c), 0x2, in_stack_0000ffc0);
        if (BVar2 != 0) {
            if ((iVar4 + 0xa) == 0) {
                local_c = 0;
            } else {
                uVar1 = (iVar4 + 0xa);
                local_c = (uVar1 + 0x8);
            }
            local_1e[0] = local_c;
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_1e),
                0x2,
                in_stack_0000ffc0,
            );
            if (BVar2 != 0) {
                pass1_1008_5784(CONCAT22(0x1050, local_14), (iVar4 + 0xa));
                loop {
                    puVar3 = local_14;
                    pass1_1008_5b12(CONCAT22(0x1050, puVar3));
                    uStack10 = CONCAT22(extraout_DX, puVar3);
                    if ((extraout_DX | puVar3) == 0) {
                        return;
                    }
                    BVar2 = pass1_1008_7c2a(param_2, *(puVar3 + 0x4));
                    if (BVar2 == 0) {
                        break;
                    }
                    local_6[0] = (uStack10 + 0x8);
                    BVar2 = write_to_file_1008_7e1c(
                        param_2,
                        CONCAT22(0x1050, local_6),
                        0x2,
                        in_stack_0000ffc0,
                    );
                    if (BVar2 == 0) {
                        break;
                    }
                    local_2a[0] = (uStack10 + 0xa);
                    BVar2 = write_to_file_1008_7e1c(
                        param_2,
                        CONCAT22(0x1050, local_2a),
                        0x4,
                        in_stack_0000ffc0,
                    );
                    if (BVar2 == 0) {
                        break;
                    }
                    local_6[0] = (uStack10 + 0xe);
                    BVar2 = write_to_file_1008_7e1c(
                        param_2,
                        CONCAT22(0x1050, local_6),
                        0x2,
                        in_stack_0000ffc0,
                    );

                    if BVar2 == 0 {
                        break;
                    }
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}


pub fn file_1008_bb5e(
    mut param_1: i16,
    param_2: *mut StructD,
    param_3: *mut astruct_199,
    mut param_4: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar3: *mut astruct_199;
    let mut BVar2: bool;
    let mut uVar3: *mut StructD;
    let mut uVar4: *mut astruct_200;
    let mut puVar3: *mut u8;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut paVar10: *mut Struct57;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut paStack286: *mut astruct_200;
    let mut puStack284: *mut u32;
    let mut local_118: [u8; 0x100] = [0; 0x100];
    let mut local_18: [u16; 0x2] = [0; 0x2];
    let mut local_14: [u16; 0x2] = [0; 0x2];
    let mut local_10: [*mut astruct_200; 0x4] = [null_mut(); 4];
    let mut local_8: u32;
    let mut uVar12: *mut astruct_199;
    let mut uVar11: *mut astruct_199;
    let mut uVar2: *mut astruct_199;
    let mut paVar9: *mut Struct57;

    paVar8 = CONCAT22(in_register_0000000a, param_2);
    if (u16_1050_0312 < 0x2) {
        return;
    }
    uVar14 = (param_4 >> 0x10);
    read_file_1008_7cfe(param_4, uVar14, 0x16);
    if (param_1 == 0) {
        u16_1050_0310 = 0x6d4;
    } else {
        iVar3 = param_3;
        iVar3 = &iVar3.field31_0x22;
        BVar2 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | ZEXT24(iVar3)), 0x2);
        if ((BVar2 != 0)
            && (
                uVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_10), 0x2),
                uVar3.is_null() == false,
            ))
        {
            if (local_10[0].is_null()) {
                return;
            }
            mem_op_1000_179c(0xc, paVar8);
            uVar6 = paVar8 | uVar3;
            paVar10 = (paVar8 & 0xffff0000);
            paVar9 = (paVar10 | uVar6);
            if (uVar6 == 0) {
                uVar3 = null_mut();
            } else {
                set_struct_1008_574a(CONCAT22(paVar8, uVar3));
                paVar10 = paVar9;
            }
            uVar13 = (param_3 >> 0x10);
            iVar3.field10_0xa = uVar3;
            (iVar3.field10_0xa + 0x2) = paVar10;
            paStack286 = null_mut();
            loop {
                if (local_10[0] <= paStack286) {
                    return;
                }
                uVar4 = local_10[0];
                mem_op_1000_179c(0x12, paVar10);
                uVar6 = paVar10 | uVar4;
                paVar8 = (paVar10 & 0xffff0000 | uVar6);
                if (uVar6 == 0) {
                    uVar4 = null_mut();
                    paVar10 = (paVar10 & 0xffff0000);
                } else {
                    set_stuct_1008_b0bc(CONCAT22(paVar10, uVar4));
                    paVar10 = paVar8;
                }
                uVar7 = SUB42(paVar10, 0x0);
                puStack284 = CONCAT22(uVar7, uVar4);
                puVar3 = local_118;
                read_file_1008_7c6e(param_4, uVar14, CONCAT22(0x1050, puVar3));
                if ((((puVar3.is_null())
                    || (
                        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_14), 0x2),
                        BVar2 == 0,
                    ))
                    || (
                        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_8), 0x4),
                        BVar2 == 0,
                    ))
                    || (
                        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_18), 0x2),
                        BVar2 == 0,
                    ))
                {
                    break;
                }
                uVar5 = str_op_1008_60e8(paVar10, CONCAT22(0x1050, local_118));
                uVar4.field3_0x4 = uVar5;
                uVar4.field4_0x6 = paVar10;
                uVar4.field5_0x8 = local_14[0];
                uVar4.field6_0xa = local_8;
                uVar4.field7_0xe = local_18[0];
                ppcVar1 = (iVar3.field10_0xa + 0x8);
                (**ppcVar1)();
                paStack286 = &paStack286.field1_0x1;
            }
            if (puStack284.is_null() == false) {
                ppcVar1 = *puStack284;
                (**ppcVar1)(0x1000, uVar4, uVar7, 0x1, puStack284);
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}
