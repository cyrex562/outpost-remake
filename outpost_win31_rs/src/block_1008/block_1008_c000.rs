// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_c626(param_1: u32) {
    _u16_1050_06e0 = 0;
    fn_ptr_1000_17ce(*param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_c646(mut param_1: u16, mut param_2: u32) -> i16 {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut uVar4: u16;
    let mut unaff_SI: u16;
    let mut puVar5: *mut u32;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut iStack18: i16;
    let mut iStack16: i16;

    uVar4 = (in_EDX >> 0x10);
    puVar5 = pass1_1008_c6fa(CONCAT22(param_2, param_1), (param_2 >> 0x10));
    uVar3 = (puVar5 >> 0x10);
    puVar6 = mixed_1010_20ba(
        CONCAT22(uVar4, uVar3),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x35),
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    iStack18 = 0;
    iStack16 = 0;
    while ((
        piVar1 = (puVar5 + 0x4),
        iVar2 = iStack16,
        *piVar1 != iStack18
            && iStack18 <= *piVar1
            && (
                iVar2 = (*puVar5 + iStack18 * 0x2),
                (iVar2 * 0x2 + puVar6 + 0xa) == 0,
            ),
    )) {
        iStack18 += 0x1;
    }
    iStack16 = iVar2;
    return iStack16;
}

pub unsafe fn pass1_1008_c6ae(mut param_1: u32, mut param_2: i16, mut param_3: i16) -> BOOL16 {
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u32;
    let mut iStack8: i16;

    puVar2 = pass1_1008_c6fa(param_1, param_3);
    iStack8 = 0;
    loop {
        piVar1 = (puVar2 + 0x4);
        if (*piVar1 == iStack8 || *piVar1 < iStack8) {
            return 0x0;
        }
        if ((*puVar2 + iStack8 * 0x2) == param_2) {
            break;
        }
        iStack8 += 0x1;
    }
    return 0x1;
}

pub unsafe fn pass1_1008_c6fa(param_1: *mut i16, mut param_2: i16) -> *mut u32 {
    if ((0x0 < param_2) && (param_2 < 0x47)) {
        return CONCAT22((param_1 + 0x2), param_2 * 0x6 + *param_1);
    }
    return NULL;
}
pub unsafe fn pass1_1008_c72a(param_1: *mut astruct_19, mut param_2: u16, mut param_3: u16) {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xe) = 0;
    param_1.offset_0x0 = 0xca4a;
    (param_1 + 0x2) = 0x1008;
    return;
}
pub unsafe fn pass1_1008_c75c(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field0_0x0 = 0xca4a;
    iVar4.field1_0x2 = 0x1008;
    puVar1 = iVar4[0x1].field1_0x2;
    puVar2 = iVar4[0x1].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_c79a(mut param_1: u32, param_2: *mut c_char) {
    let mut string_1: *mut c_char;
    let mut iVar1: i16;
    let mut pstruct117_2: *mut astruct_117;
    let mut extraout_DX: u16;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;
    let mut in_stack_0000fe70: u16;
    let mut local_12: [u8; 0x4] = [0; 0x4];
    let mut uStack14: u32;
    let mut string_a: [u8; 0x8] = [0; 0x8];

    uVar4 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(0x1050, string_a), (param_1 + 0xa));
    loop {
        string_1 = string_a;
        pass1_1008_5b12(CONCAT22(0x1050, string_1));
        uStack14 = CONCAT22(extraout_DX, string_1);
        puVar2 = (extraout_DX | string_1);
        if (puVar2.is_null()) {
            break;
        }
        iVar1 = pass1_1000_3d7a(*(string_1 + 0x4), param_2);
        if (iVar1 == 0) {
            puVar5 = pass1_1020_a43e(puVar2, CONCAT22(0x1050, local_12));
            uVar3 = (puVar5 >> 0x10);
            pass1_1020_a6ee(
                local_12,
                uVar3,
                in_stack_0000fe70,
                CONCAT22(0x1050, local_12),
                (uStack14 + 0x12),
            );
            pstruct117_2 = (_PTR_LOOP_1050_65e2 + 0x52);
            pass1_1030_4bbe(uVar3, pstruct117_2, (uStack14 + 0x12));
            (param_1 + 0xe) = (pstruct117_2 + 0x94) + *_PTR_LOOP_1050_65e2;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_c83a(param_1: *mut astruct_201) {
    if (*_PTR_LOOP_1050_65e2 <= (param_1 + 0xe)) {
        return;
    }
    return;
}

pub unsafe fn pass1_1008_c85e(param_1: *mut astruct_201) -> u32 {
    let mut iVar1: *mut astruct_201;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    if (iVar1.field10_0xa.is_null()) {
        pass1_1008_c882((param_1 & 0xffff | uVar1 << 0x10));
    }
    return CONCAT22((&iVar1.field10_0xa + 0x2), &iVar1.field10_0xa);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_c882(param_1: *mut astruct_201) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut ppcVar4: *mut *mut code;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_EDX: u32;
    let mut paVar10: *mut Struct57;
    let mut uVar12: u32;
    let mut paVar13: *mut Struct57;
    let mut iVar9: *mut astruct_201;
    let mut unaff_SI: u16;
    let mut uVar14: u16;
    let mut uVar15: u8;
    let mut puVar16: *mut u32;
    let mut puVar17: *mut u32;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut iStack16: i16;
    let mut paVar11: *mut Struct57;

    uVar14 = (param_1 >> 0x10);
    iVar9 = param_1;
    // WARNING: Load size is inaccurate
    puVar5 = iVar9.field10_0xa;
    uVar9 = (&iVar9.field10_0xa + 2);
    paVar10 = (in_EDX & 0xffff0000 | uVar9);
    if ((uVar9 | puVar5) != 0) {
        ppcVar4 = *puVar5;
        puVar5 = (**ppcVar4)();
    }
    mem_op_1000_179c(0xc, paVar10);
    uVar9 = paVar10 | puVar5;
    paVar13 = (paVar10 & 0xffff0000);
    paVar11 = (paVar13 | uVar9);
    if (uVar9 == 0) {
        uVar6 = 0;
    } else {
        uVar6 = set_struct_1008_574a(CONCAT22(paVar10, puVar5));
        paVar13 = paVar11;
    }
    iVar9.field10_0xa = uVar6;
    (&iVar9.field10_0xa + 0x2) = paVar13;
    puVar16 = mixed_1010_20ba(
        paVar13,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x35),
        in_stack_0000fe7e,
        in_stack_0000ffa2,
        in_stack_0000ffa8,
        in_stack_0000ffac,
    );
    uVar12 = paVar13 & 0xffff0000;
    puVar17 = pass1_1008_c6fa(_u16_1050_06e0, 0x44);
    paVar10 = (uVar12 & 0xffff0000 | puVar17 >> 0x10);
    iStack16 = 0;
    loop {
        piVar1 = (puVar17 + 0x4);
        if (*piVar1 == iStack16 || *piVar1 < iStack16) {
            break;
        }
        uVar2 = (*puVar17 + iStack16 * 0x2);
        if ((uVar2 * 0x2 + puVar16 + 0xa) != 0) {
            uVar7 = pass1_1020_bd80(uVar2);
            uVar8 = str_op_1008_60e8(paVar10, CONCAT22(paVar10, uVar7));
            uVar6 = SUB42(paVar10, 0x0);
            uVar15 = 0;
            uVar7 = uVar8;
            mem_op_1000_179c(0x14, paVar10);
            uVar9 = paVar10 | uVar7;
            paVar13 = (paVar10 & 0xffff0000 | uVar9);
            if (uVar9 == 0) {
                uVar7 = 0;
                paVar10 = (paVar10 & 0xffff0000);
            } else {
                uVar15 = 0x18;
                struct_1018_47c8(
                    CONCAT22(paVar10, uVar7),
                    0x1,
                    CONCAT22(uVar6, uVar8),
                    uVar2,
                    0x0,
                );
                paVar10 = paVar13;
            }
            puVar3 = iVar9.field10_0xa;
            ppcVar4 = (*iVar9.field10_0xa + 0x4);
            (**ppcVar4)(uVar15, puVar3, (puVar3 >> 0x10), uVar7, paVar10);
        }
        iStack16 += 0x1;
    }
    return;
}
pub unsafe fn pass1_1008_c98e(mut param_1: u32, mut param_2: u32) {
    let mut BVar1: bool;
    let mut in_stack_0000ffda: HFILE16;
    let mut local_10: [u32; 0x3] = [0; 0x3];

    BVar1 = write_to_file_1008_7cac(param_2);
    if (BVar1 != 0) {
        local_10[0] = (param_1 + 0xe);
        BVar1 =
            write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_10), 0x4, in_stack_0000ffda);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}
pub unsafe fn pass1_1008_c9d4(mut param_1: i16, mut param_2: u32, mut param_3: u32) {
    let mut BVar1: bool;

    if (0x1 < u16_1050_0312) {
        read_file_1008_7cfe(param_3, (param_3 >> 0x10), 0x15);
        if (param_1 == 0) {
            u16_1050_0310 = 0x6d4;
            return;
        }
        BVar1 = read_file_1008_7dee(param_3, (param_2 & 0xffff0000 | (param_2 + 0xe)), 0x4);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1008_ca24(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_c75c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn pass1_1008_ca5a(param_1: *mut astruct_19, mut param_2: u16) {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x1a) = 0;
    (param_1 + 0x1e) = 0;
    param_1.offset_0x0 = 0xd71a;
    (param_1 + 0x2) = 0x1008;
    return;
}
pub unsafe fn pass1_1008_caa0(param_1: *mut astruct_455) {
    let mut uVar1: *mut astruct_455;

    uVar1 = (param_1 >> 0x10);
    param_1.field0_0x0 = 0xd71a;
    (param_1 + 0x2) = 0x1008;
    pass1_1008_cac6((param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
    pass1_1010_1d80(param_1);
    return;
}
pub unsafe fn pass1_1008_cac6(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar4: *mut astruct_455;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = iVar4[0x1].field1_0x2;
    puVar2 = iVar4[0x1].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    iVar4[0x1].field1_0x2 = 0;
    puVar1 = iVar4[0x1].field3_0x6;
    uVar3 = (iVar4 + 0x2).field0_0x0;
    if ((uVar3 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    iVar4[0x1].field3_0x6 = 0;
    puVar1 = iVar4[0x2].field1_0x2;
    puVar2 = iVar4[0x2].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    iVar4[0x2].field1_0x2 = 0;
    puVar1 = iVar4[0x2].field3_0x6;
    uVar3 = (iVar4 + 0x3).field0_0x0;
    if ((uVar3 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    iVar4[0x2].field3_0x6 = 0;
    puVar1 = iVar4[0x3].field1_0x2;
    puVar2 = iVar4[0x3].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    iVar4[0x3].field1_0x2 = 0;
    puVar1 = iVar4[0x3].field3_0x6;
    uVar3 = (iVar4 + 0x4).field0_0x0;
    if ((uVar3 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    iVar4[0x3].field3_0x6 = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_cbc4(param_1: *mut astruct_263, mut param_2: u32) {
    let mut lVar1: i32;
    let mut lVar2: i32;
    let mut ppcVar3: *mut *mut code;
    let mut bVar4: bool;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut paVar7: *mut astruct_92;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut in_EDX: u32;
    let mut paVar13: *mut Struct57;
    let mut iVar10: *mut astruct_263;
    let mut uVar16: *mut astruct_263;
    let mut uVar17: u8;
    let mut pcVar18: *mut c_char;
    let mut uStack54: u16;
    let mut iStack30: i16;
    let mut local_18: *mut astruct_92;
    let mut uVar8: u32;
    let mut paVar14: *mut Struct57;
    let mut paVar15: *mut Struct57;

    uVar16 = (param_1 >> 0x10);
    iVar10 = param_1;
    puVar5 = &iVar10.field17_0x1e;
    uVar9 = (&iVar10.field17_0x1e + 2);
    paVar13 = (in_EDX & 0xffff0000 | uVar9);
    if ((uVar9 | puVar5) != 0) {
        ppcVar3 = *puVar5;
        (**ppcVar3)();
    }
    mem_op_1000_179c(0xc, paVar13);
    uVar9 = paVar13 | puVar5;
    paVar15 = (paVar13 & 0xffff0000);
    paVar14 = (paVar15 | uVar9);
    if (uVar9 == 0) {
        puVar5 = NULL;
    } else {
        set_struct_1008_574a(CONCAT22(paVar13, puVar5));
        paVar15 = paVar14;
    }
    iVar10.field17_0x1e = puVar5;
    (&iVar10.field17_0x1e + 0x2) = paVar15;
    lVar1 = (param_2 + 0x200);
    pass1_1028_dc52(CONCAT22(0x1050, &local_18), 0x1, 0x0, 0x400);
    iStack30 = 0;
    loop {
        paVar7 = &local_18;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar7));
        uVar9 = paVar15;
        uVar10 = uVar9 | paVar7;
        paVar13 = (paVar15 & 0xffff0000);
        paVar15 = (paVar13 | uVar10);
        if (uVar10 == 0) {
            break;
        }
        if (paVar7[0x1c].field4_0x8 == lVar1) {
            iStack30 += 0x1;
        }
    }
    bVar4 = false;
    if (0x1 < iStack30) {
        if (local_18.field6_0x10 == 0) {
            paVar15 = (paVar13 | local_18.field5_0xc);
        } else {
            local_18.field5_0xc = 0x1;
            paVar15 = paVar13;
        }
        local_18.field4_0x8 = SUB42(paVar15, 0x0);
        local_18.field4_0x8 = local_18.field5_0xc;
        loop {
            paVar7 = &local_18;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar7));
            uVar9 = paVar15;
            paVar15 = (paVar15 & 0xffff0000 | (uVar9 | paVar7));
            if ((uVar9 | paVar7) == 0) {
                break;
            }
            if ((paVar7[0x1c].field4_0x8 == lVar1) && (paVar7.field3_0x4 != 0x4000001)) {
                pcVar18 = pass1_1038_4d28(CONCAT22(uVar9, paVar7));
                paVar13 = (paVar15 & 0xffff0000 | pcVar18 >> 0x10);
                uVar6 = str_op_1008_60e8((pcVar18 >> 0x10), pcVar18);
                uVar8 = uVar6;
                uVar11 = SUB42(paVar13, 0x0);
                uVar17 = 0;
                mem_op_1000_179c(0x12, paVar13);
                uVar10 = uVar8;
                uVar12 = paVar13 | uVar10;
                paVar15 = (paVar13 & 0xffff0000 | uVar12);
                if (uVar12 == 0) {
                    uVar10 = 0;
                    uStack54 = 0;
                } else {
                    uVar17 = 0x18;
                    struct_1018_4920(
                        (uVar8 & 0xffff | paVar13 << 0x10),
                        0x1,
                        CONCAT22(uVar11, uVar6),
                        paVar7.field3_0x4,
                    );
                    uStack54 = SUB42(paVar15, 0x0);
                }
                lVar2 = iVar10.field17_0x1e;
                ppcVar3 = (iVar10.field17_0x1e + 0x4);
                (**ppcVar3)(uVar17, lVar2, (lVar2 >> 0x10), uVar10, uStack54);
                bVar4 = true;
            }
        }
    }
    if (!bVar4) {
        load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x43d);
        uVar8 = CONCAT22(paVar15, paVar7);
        uVar17 = 0;
        mem_op_1000_179c(0x12, paVar15);
        uVar9 = paVar15 | paVar7;
        if (uVar9 == 0) {
            paVar7 = NULL;
            uVar9 = 0;
        } else {
            uVar17 = 0x18;
            struct_1018_4920(CONCAT22(paVar15, paVar7), 0x0, uVar8, 0x0);
        }
        lVar1 = iVar10.field17_0x1e;
        ppcVar3 = (iVar10.field17_0x1e + 0x4);
        (**ppcVar3)(
            uVar17,
            lVar1,
            (lVar1 >> 0x10),
            paVar7,
            uVar9,
            uVar8,
            paVar7,
            uVar9,
        );
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_cda2(param_1: *mut astruct_263, mut param_2: u32) {
    let mut plVar1: *mut i32;
    let mut lVar2: i32;
    let mut ppcVar3: *mut *mut code;
    let mut puVar4: *mut u32;
    let mut uVar5: u16;
    let mut pcVar6: *mut c_char;
    let mut puVar9: *mut astruct_206;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_EDX: u32;
    let mut paVar10: *mut Struct57;
    let mut uVar11: u32;
    let mut iVar15: *mut astruct_263;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u8;
    let mut paVar15: *mut astruct_203;
    let mut local_2e: [u8; 0xa] = [0; 0xa];
    let mut uStack36: u16;
    let mut uStack34: u32;
    let mut uStack30: u32;
    let mut uStack26: u32;
    let mut puStack18: *mut u32;
    let mut uStack16: u16;
    let mut paStack14: *mut astruct_203;
    let mut uStack10: u16;
    let mut uStack8: u32;
    let mut iStack4: i16;

    uVar12 = (param_1 >> 0x10);
    iVar15 = param_1;
    puVar4 = &iVar15.field16_0x1a;
    uStack16 = (&iVar15.field16_0x1a + 2);
    paVar10 = (in_EDX & 0xffff0000 | uStack16);
    paStack14 = CONCAT22(uStack16, puVar4);
    puStack18 = puVar4;
    if ((uStack16 | puVar4) != 0) {
        ppcVar3 = *puVar4;
        (**ppcVar3)();
    }
    mem_op_1000_179c(0xc, paVar10);
    uStack16 = paVar10;
    uVar7 = paVar10 & 0xffff0000;
    uVar11 = uVar7 | (uStack16 | puVar4);
    puStack18 = puVar4;
    if ((uStack16 | puVar4) == 0) {
        puVar4 = NULL;
    } else {
        set_struct_1008_574a(CONCAT22(uStack16, puVar4));
        uVar7 = uVar11;
    }
    iVar15.field16_0x1a = puVar4;
    (&iVar15.field16_0x1a + 0x2) = uVar7;
    iStack4 = 0;
    uVar13 = (param_2 >> 0x10);
    uStack8 = (param_2 + 0x210);
    uVar5 = (param_2 + 0x212);
    paVar10 = (uVar7 & 0xffff0000 | uVar5);
    uVar5 |= uStack8;
    uVar7 = uVar5;
    if (uVar5 != 0) {
        uStack26 = (uStack8 + 0xa);
        uStack30 = 0;
        loop {
            paVar10 = (paVar10 & 0xffff0000 | uStack26 >> 0x10);
            uVar7 = uStack26;
            if (uStack26 <= uStack30) {
                break;
            }
            bad_1030_1312();
            uStack34 = uVar7 & 0xffff | paVar10 << 0x10;
            uVar5 = paVar10 | uVar7;
            paVar10 = (paVar10 & 0xffff0000 | uVar5);
            if (uVar5 != 0) {
                // for (uStack36 = 0x1; uStack36 < 0x15; uStack36 += 1)
                for uStack36 in 1..0x15 {
                    local_2e._8_2_ = pass1_1030_ce2e(uStack34, (uStack34 >> 0x10), uStack36);
                    if (local_2e._8_2_ != 0) {
                        pass1_1008_5784(CONCAT22(0x1050, local_2e), iVar15.field16_0x1a);
                        loop {
                            puVar9 = local_2e;
                            pass1_1008_5b12(CONCAT22(0x1050, puVar9));
                            uVar5 = paVar10;
                            paVar10 = (paVar10 & 0xffff0000 | (uVar5 | puVar9));
                            if ((uVar5 | puVar9) == 0) {
                                break;
                            }
                            if puVar9.field_0xe == uStack36 {
                                break;
                            }
                        }
                        if (CONCAT22(uVar5, puVar9) == 0) {
                            pcVar6 = string_op_1020_c222(uStack36);
                            uVar5 = str_op_1008_60e8(paVar10, CONCAT22(paVar10, pcVar6));
                            uVar7 = CONCAT22(paVar10, uVar5);
                            uVar14 = 0;
                            mem_op_1000_179c(0x10, paVar10);
                            uVar8 = paVar10;
                            paStack14 = CONCAT22(uVar8, uVar5);
                            paVar10 = (paVar10 & 0xffff0000);
                            if ((uVar8 | uVar5) == 0) {
                                uVar13 = 0;
                            } else {
                                uVar14 = 0x18;
                                paVar15 = struct_1018_48b0(
                                    paStack14,
                                    CONCAT22(
                                        local_2e._8_2_ >> 0xf,
                                        local_2e._8_2_ & 0xff | (local_2e._8_2_ >> 0x8) << 0x8,
                                    ),
                                    uVar7,
                                    uStack36,
                                );
                                paVar10 = (paVar10 & 0xffff0000 | paVar15 >> 0x10);
                                uVar13 = SUB42(paVar15, 0x0);
                            }
                            lVar2 = iVar15.field16_0x1a;
                            ppcVar3 = (iVar15.field16_0x1a + 0x4);
                            (**ppcVar3)(uVar14, lVar2, (lVar2 >> 0x10), uVar13, paVar10);
                        } else {
                            plVar1 = &puVar9.field5_0x8;
                            *plVar1 = *plVar1 + local_2e._8_2_;
                        }
                        iStack4 = 0x1;
                    }
                }
            }
            uStack30 += 0x1;
        }
    }
    uVar5 = uVar7;
    uStack10 = 0;
    if (iStack4 == 0) {
        load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x43f);
        uVar7 = CONCAT22(paVar10, uVar5);
        uVar14 = 0;
        mem_op_1000_179c(0x10, paVar10);
        uStack16 = paVar10;
        puStack18 = uVar5;
        if ((uStack16 | uVar5) == 0) {
            uVar13 = 0;
            uVar9 = 0;
        } else {
            uVar14 = 0x18;
            paVar15 = struct_1018_48b0(CONCAT22(uStack16, uVar5), 0x0, uVar7, 0x0);
            uVar9 = (paVar15 >> 0x10);
            uVar13 = SUB42(paVar15, 0x0);
        }
        lVar2 = iVar15.field16_0x1a;
        ppcVar3 = (iVar15.field16_0x1a + 0x4);
        (**ppcVar3)(
            uVar14,
            lVar2,
            (lVar2 >> 0x10),
            uVar13,
            uVar9,
            uVar7,
            uVar13,
            uVar9,
        );
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_cfa0(param_1: *mut astruct_263, mut param_2: u32) {
    let mut uVar1: u32;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut bVar4: bool;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut in_EDX: u32;
    let mut paVar14: *mut Struct57;
    let mut paVar16: *mut Struct57;
    let mut iVar15: *mut astruct_4;
    let mut uVar17: u16;
    let mut uVar18: u8;
    let mut paVar19: *mut astruct_203;
    let mut paVar15: *mut Struct57;

    uVar17 = (param_1 >> 0x10);
    iVar15 = param_1;
    // WARNING: Load size is inaccurate
    puVar5 = iVar15.field22_0x16;
    uVar10 = (&iVar15.field22_0x16 + 2);
    paVar14 = (in_EDX & 0xffff0000 | uVar10);
    if ((uVar10 | puVar5) != 0) {
        ppcVar3 = *puVar5;
        (**ppcVar3)();
    }
    mem_op_1000_179c(0xc, paVar14);
    uVar10 = paVar14 | puVar5;
    paVar16 = (paVar14 & 0xffff0000);
    paVar15 = (paVar16 | uVar10);
    if (uVar10 == 0) {
        puVar5 = NULL;
    } else {
        set_struct_1008_574a(CONCAT22(paVar14, puVar5));
        paVar16 = paVar15;
    }
    iVar15.field22_0x16 = puVar5;
    (&iVar15.field22_0x16 + 0x2) = paVar16;
    bVar4 = false;
    uVar1 = (param_2 + 0x1f6);
    uVar9 = uVar1;
    pass1_1030_38f2(uVar1, 0x2);
    uVar10 = uVar9;
    if ((-0x1 < paVar16) && (0x0 < paVar16 || (uVar10 != 0))) {
        paVar15 = paVar16;
        load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x430);
        uVar13 = SUB42(paVar15, 0x0);
        uVar18 = 0;
        uVar6 = uVar10;
        mem_op_1000_179c(0x14, paVar15);
        paVar14 = (paVar15 & 0xffff0000);
        if ((paVar15 | uVar6) == 0) {
            uVar10 = 0;
            paVar16 = paVar14;
        } else {
            uVar18 = 0x18;
            paVar19 = struct_1018_4842(
                CONCAT22(paVar15, uVar6),
                uVar9 & 0xffff | paVar16 << 0x10,
                CONCAT22(uVar13, uVar10),
                0x2,
            );
            uVar10 = paVar19;
            paVar16 = (paVar14 & 0xffff0000 | paVar19 >> 0x10);
        }
        puVar2 = iVar15.field22_0x16;
        ppcVar3 = (*iVar15.field22_0x16 + 0x4);
        (**ppcVar3)(uVar18, puVar2, (puVar2 >> 0x10), uVar10, paVar16);
        bVar4 = true;
    }
    pass1_1030_38f2(uVar1, 0x3);
    iVar11 = paVar16;
    if ((-0x1 < iVar11) && (0x0 < iVar11 || (uVar10 != 0))) {
        uVar7 = uVar10;
        load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x431);
        uVar13 = SUB42(paVar16, 0x0);
        uVar18 = 0;
        uVar6 = uVar7;
        mem_op_1000_179c(0x14, paVar16);
        uVar12 = paVar16;
        paVar16 = (paVar16 & 0xffff0000);
        if ((uVar12 | uVar6) == 0) {
            uVar10 = 0;
        } else {
            uVar18 = 0x18;
            paVar19 = struct_1018_4842(
                CONCAT22(uVar12, uVar6),
                CONCAT22(iVar11, uVar10),
                CONCAT22(uVar13, uVar7),
                0x3,
            );
            paVar16 = (paVar16 & 0xffff0000 | paVar19 >> 0x10);
            uVar10 = paVar19;
        }
        puVar2 = iVar15.field22_0x16;
        ppcVar3 = (*iVar15.field22_0x16 + 0x4);
        (**ppcVar3)(uVar18, puVar2, (puVar2 >> 0x10), uVar10, paVar16);
        bVar4 = true;
    }
    pass1_1030_38f2(uVar1, 0x4);
    iVar11 = paVar16;
    if ((-0x1 < iVar11) && (0x0 < iVar11 || (uVar10 != 0))) {
        uVar7 = uVar10;
        load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x432);
        uVar13 = SUB42(paVar16, 0x0);
        uVar18 = 0;
        uVar6 = uVar7;
        mem_op_1000_179c(0x14, paVar16);
        uVar12 = paVar16;
        paVar16 = (paVar16 & 0xffff0000);
        if ((uVar12 | uVar6) == 0) {
            uVar10 = 0;
        } else {
            uVar18 = 0x18;
            paVar19 = struct_1018_4842(
                CONCAT22(uVar12, uVar6),
                CONCAT22(iVar11, uVar10),
                CONCAT22(uVar13, uVar7),
                0x4,
            );
            paVar16 = (paVar16 & 0xffff0000 | paVar19 >> 0x10);
            uVar10 = paVar19;
        }
        puVar2 = iVar15.field22_0x16;
        ppcVar3 = (*iVar15.field22_0x16 + 0x4);
        (**ppcVar3)(uVar18, puVar2, (puVar2 >> 0x10), uVar10, paVar16);
        bVar4 = true;
    }
    if (!bVar4) {
        load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x440);
        uVar13 = SUB42(paVar16, 0x0);
        uVar18 = 0;
        uVar6 = uVar10;
        mem_op_1000_179c(0x14, paVar16);
        if ((paVar16 | uVar6) == 0) {
            uVar8 = 0;
            uVar13 = 0;
        } else {
            uVar18 = 0x18;
            paVar19 =
                struct_1018_4842(CONCAT22(paVar16, uVar6), 0x0, CONCAT22(uVar13, uVar10), 0x0);
            uVar13 = (paVar19 >> 0x10);
            uVar8 = SUB42(paVar19, 0x0);
        }
        puVar2 = iVar15.field22_0x16;
        ppcVar3 = (*iVar15.field22_0x16 + 0x4);
        (**ppcVar3)(uVar18, puVar2, (puVar2 >> 0x10), uVar8, uVar13);
    }
    return;
}
