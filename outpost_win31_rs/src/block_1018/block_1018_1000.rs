pub unsafe fn pass1_1018_1054(mut param_1: u32, param_2: *mut u16, param_3: *mut u32) {
    let mut in_EDX: u32;
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x94) == 0) {
        pass1_1018_0dc6(in_EDX, (param_1 & 0xffff | uVar2 << 0x10));
    }
    *param_3 = (iVar1 + 0x94);
    *param_2 = (iVar1 + 0x92);
    return;
}
pub unsafe fn pass1_1018_108c(mut param_1: u32, param_2: *mut u16, param_3: *mut u32) {
    let mut in_EDX: u32;
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x9a) == 0) {
        pass1_1018_0dc6(in_EDX, (param_1 & 0xffff | uVar2 << 0x10));
    }
    *param_3 = (iVar1 + 0x9a);
    *param_2 = (iVar1 + 0x98);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_10c4(mut param_1: u16, mut param_2: u32) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut paVar5: *mut astruct_92;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_register_0000000a: u16;
    let mut paVar10: *mut Struct57;
    let mut uVar11: u32;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut uVar14: u8;
    let mut bVar15: bool;
    let mut puVar16: *mut u32;
    let mut uStack60: u32;
    let mut uStack56: u32;
    let mut uStack52: u32;
    let mut puStack48: *mut u32;
    let mut puStack40: *mut u32;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut local_16: *mut astruct_92;

    paVar10 = CONCAT22(in_register_0000000a, param_1);
    uVar13 = (param_2 >> 0x10);
    iVar12 = param_2;
    iVar1 = (iVar12 + 0x86);
    fn_ptr_1000_17ce(*(iVar12 + 0x88));
    (iVar12 + 0x86) = 0;
    (iVar12 + 0x88) = 0;
    pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0;
    uStack28 = 0;
    loop {
        paVar5 = &local_16;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar5));
        uVar7 = paVar10;
        paVar10 = (paVar10 & 0xffff0000 | (uVar7 | paVar5));
        if ((uVar7 | paVar5) == 0) {
            break;
        }
        if ((iVar12 + 0x3c) == paVar5.field4_0x8) {
            puVar16 = pass1_1008_c6fa(_u16_1050_06e0, 0x2);
            paVar10 = (paVar10 & 0xffff0000 | puVar16 >> 0x10);
            uVar9 = puVar16;
            pass1_1038_4e78(uVar9, paVar10, CONCAT22(uVar7, paVar5), puVar16);
            uVar8 = SUB42(paVar10, 0x0);
            puStack48 = CONCAT22(uVar8, uVar9);
            uVar3 = *puStack48;
            ppcVar2 = uVar3 + 0x8;
            uVar7 = uVar9;
            (**ppcVar2)(&u16_1050_1038, uVar9, uVar8);
            bVar15 = CARRY2(uStack30, uVar7);
            uStack30 += uVar7;
            uStack28 = uStack28 + paVar10 + bVar15;
            if (puStack48.is_null() == false) {
                ppcVar2 = uVar3;
                (**ppcVar2)(0x38, uVar9, uVar8, 1);
            }
        }
    }
    if ((uStack28 | uStack30) != 0) {
        (iVar12 + 0x86) = uStack30;
        uVar7 = uStack30 * 0x6;
        mem_op_1000_179c(uVar7, paVar10);
        uVar9 = paVar10;
        uStack52 = CONCAT22(uVar9, uVar7);
        uVar11 = paVar10 & 0xffff0000;
        if ((uVar9 | uVar7) == 0) {
            (iVar12 + 0x88) = 0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack30, 0x6, uVar7, uVar9);
            (iVar12 + 0x88) = uStack52;
        }
        if (local_16.field6_0x10 == 0) {
            paVar10 = (uVar11 & 0xffff0000 | local_16.field5_0xc);
        } else {
            local_16.field5_0xc = 0x1;
            paVar10 = (uVar11 & 0xffff0000);
        }
        local_16.field4_0x8 = SUB42(paVar10, 0x0);
        iVar4 = 0;
        local_16.field4_0x8 = local_16.field5_0xc;
        loop {
            paVar5 = &local_16;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar5));
            uVar7 = paVar10;
            paVar10 = (paVar10 & 0xffff0000 | (uVar7 | paVar5));
            if ((uVar7 | paVar5) == 0) {
                break;
            }
            if ((iVar12 + 0x3c) == paVar5.field4_0x8) {
                puVar16 = pass1_1008_c6fa(_u16_1050_06e0, 0x2);
                paVar10 = (paVar10 & 0xffff0000 | puVar16 >> 0x10);
                uVar9 = puVar16;
                uVar14 = 0x38;
                pass1_1038_4e78(uVar9, paVar10, CONCAT22(uVar7, paVar5), puVar16);
                uVar8 = SUB42(paVar10, 0x0);
                puStack40 = CONCAT22(uVar8, uVar9);
                ppcVar2 = (*puStack40 + 0x10);
                uVar7 = uVar9;
                (**ppcVar2)(&u16_1050_1038, uVar9, uVar8);
                uStack56 = CONCAT22(paVar10, uVar7);
                // for (uStack60 = 0; uStack60 < uStack56; uStack60 += 1)
                for uStack60 in 0..uStack56 {
                    uVar6 = uStack56;
                    pass1_1030_1d58(puStack40);
                    uVar11 = (iVar12 + 0x88);
                    uVar14 = 0x8;
                    pass1_1008_3f62(
                        (uVar11 & 0xffff0000 | (uVar11 + iVar4 * 0x6)),
                        CONCAT22(paVar10, uVar6 + 0xc),
                    );
                    iVar4 += 0x1;
                }
                if (puStack40.is_null() == false) {
                    ppcVar2 = *puStack40;
                    (**ppcVar2)(uVar14, uVar9, uVar8, 1);
                }
            }
        }
        if ((iVar12 + 0x86) != iVar1) {
            pass1_1010_1f62(param_2, 0x6);
        }
    }
    return;
}
pub unsafe fn pass1_1018_1320(mut param_1: u32, param_2: *mut u16, param_3: *mut u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x88);
    *param_2 = (param_1 + 0x86);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_1346(mut param_1: u16, param_2: *mut astruct_93) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    astruct_92 * *ppaVar3;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut iVar9: *mut astruct_93;
    let mut uVar9: u16;
    let mut uVar10: u8;
    let mut puVar11: *mut u32;
    let mut uVar12: u32;
    let mut uVar13: u32;
    let mut uStack70: u32;
    let mut puStack56: *mut u32;
    let mut uStack52: u32;
    let mut puStack48: *mut u32;
    let mut uStack30: u32;
    let mut local_16: *mut astruct_92;
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    uVar9 = (param_2 >> 0x10);
    iVar9 = param_2;
    uStack4 = iVar9.field137_0x8c;
    fn_ptr_1000_17ce(iVar9.field138_0x8e);
    iVar9.field137_0x8c = 0;
    iVar9.field138_0x8e = 0;
    pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0;
    loop {
        ppaVar3 = &local_16;
        pass1_1028_e4ec(CONCAT22(0x1050, ppaVar3));
        uVar5 = paVar8;
        paVar8 = (paVar8 & 0xffff0000 | (uVar5 | ppaVar3));
        if ((uVar5 | ppaVar3) == 0) {
            break;
        }
        if (iVar9.field60_0x3c == (ppaVar3 + 0x4)) {
            puVar11 = pass1_1008_c6fa(_u16_1050_06e0, 0x2);
            paVar8 = (paVar8 & 0xffff0000 | puVar11 >> 0x10);
            uVar7 = puVar11;
            uVar10 = 0x38;
            pass1_1038_4e78(uVar7, paVar8, CONCAT22(uVar5, ppaVar3), puVar11);
            uVar6 = SUB42(paVar8, 0x0);
            puStack48 = CONCAT22(uVar6, uVar7);
            ppcVar1 = (*puStack48 + 0x10);
            uVar5 = uVar7;
            (**ppcVar1)(&u16_1050_1038, uVar7, uVar6);
            uStack52 = CONCAT22(paVar8, uVar5);
            //   for (puStack56 = null_mut(); puStack56 < uStack52; puStack56 = (puStack56 + 1))
            puStack56 = null_mut();

            while puStack56 < uStack52 {
                uVar10 = 0x30;
                uVar12 = pass1_1030_1d7c(uVar5, paVar8, puStack48);
                paVar8 = (paVar8 & 0xffff0000 | uVar12 >> 0x10);
                if ((uVar12 + 0x12) == 0x9) {
                    uStack30 += 0x1;
                }
                puStack56 += 1;
            }
            if (puStack48.is_null() == false) {
                ppcVar1 = *puStack48;
                (**ppcVar1)(uVar10, uVar7, uVar6, 1);
            }
        }
    }
    if ((uStack30 | uStack30) != 0) {
        iVar9.field137_0x8c = uStack30;
        uVar5 = uStack30 * 0x6;
        mem_op_1000_179c(uVar5, paVar8);
        uVar7 = paVar8;
        uStack70 = CONCAT22(uVar7, uVar5);
        uVar12 = paVar8 & 0xffff0000;
        if ((uVar7 | uVar5) == 0) {
            iVar9.field138_0x8e = 0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack30, 0x6, uVar5, uVar7);
            iVar9.field138_0x8e = uStack70;
        }
        if (iStack6 == 0) {
            paVar8 = (uVar12 & 0xffff0000 | uStack8);
        } else {
            uStack10 = 0x1;
            paVar8 = (uVar12 & 0xffff0000);
        }
        uStack12 = SUB42(paVar8, 0x0);
        iVar2 = 0;
        uStack14 = uStack10;
        loop {
            ppaVar3 = &local_16;
            pass1_1028_e4ec(CONCAT22(0x1050, ppaVar3));
            uVar5 = paVar8;
            paVar8 = (paVar8 & 0xffff0000 | (uVar5 | ppaVar3));
            if ((uVar5 | ppaVar3) == 0) {
                break;
            }
            if (iVar9.field60_0x3c == (ppaVar3 + 0x4)) {
                puVar11 = pass1_1008_c6fa(_u16_1050_06e0, 0x2);
                paVar8 = (paVar8 & 0xffff0000 | puVar11 >> 0x10);
                uVar7 = puVar11;
                uVar10 = 0x38;
                pass1_1038_4e78(uVar7, paVar8, CONCAT22(uVar5, ppaVar3), puVar11);
                uVar6 = SUB42(paVar8, 0x0);
                puStack56 = CONCAT22(uVar6, uVar7);
                ppcVar1 = (*puStack56 + 0x10);
                uVar5 = uVar7;
                (**ppcVar1)(&u16_1050_1038, uVar7, uVar6);
                uStack52 = CONCAT22(paVar8, uVar5);
                // for (puStack48 = null_mut(); puStack48 < uStack52; puStack48 = (puStack48 + 1))
                puStack48 = null_mut();
                while puStack48 < uStack52 {
                    uVar4 = uStack52;
                    pass1_1030_1d58(puStack56);
                    uVar12 = paVar8 << 0x10;
                    uVar10 = 0x30;
                    uVar13 = struct_op_1030_73a8((uVar4 & 0xffff | paVar8 << 0x10), uVar4, paVar8);
                    paVar8 = (paVar8 & 0xffff0000 | uVar13 >> 0x10);
                    if ((uVar13 + 0x12) == 0x9) {
                        uVar13 = iVar9.field138_0x8e;
                        uVar10 = 0x8;
                        pass1_1008_3f62(
                            (uVar13 & 0xffff0000 | (uVar13 + iVar2 * 0x6)),
                            (uVar12 | uVar4 + 0xc),
                        );
                        iVar2 += 0x1;
                    }
                    puStack48 += 1;
                }
                if (puStack56.is_null() == false) {
                    ppcVar1 = *puStack56;
                    (**ppcVar1)(uVar10, uVar7, uVar6, 1);
                }
            }
        }
        if (iVar9.field137_0x8c != uStack4) {
            pass1_1010_1f62(param_2, 0x6);
        }
    }
    return;
}
pub unsafe fn pass1_1018_15f6(mut param_1: u32, param_2: *mut u16, param_3: *mut u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x8e);
    *param_2 = (param_1 + 0x8c);
    return;
}
pub unsafe fn pass1_1018_161c(
    mut param_1: u32,
    param_2: *mut u16,
    mut param_3: i16,
    mut param_4: i16,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut local_6: u32;

    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (param_1 + 0x36)),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_6 + 0x2),
    );
    uVar1 = local_6 + param_4 + -0x3;
    uVar2 = local_6 + param_3 + -0x3;
    local_6 = CONCAT22(uVar1, uVar2);
    pass1_1008_3e76(param_2, (param_1 + 0x44), uVar2, uVar1);
    return;
}
pub unsafe fn pass1_1018_1662(mut param_1: u32, mut param_2: i16, mut param_3: i16) {
    let mut local_6: i16;
    let mut local_4: i16;

    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (param_1 + 0x36)),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    pass1_1018_16b8(
        param_1,
        (param_1 + 0x44),
        CONCAT22(local_4 + param_3, local_6 + param_2),
    );
    return;
}
pub unsafe fn pass1_1018_169e(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1018_16b8(param_1 & 0xffff | uVar1 << 0x10, (param_1 + 0x44), param_2);
    return;
}

// WARNING: Unable to use type for symbol uVar2
pub unsafe fn pass1_1018_16b8(mut param_1: u32, mut param_2: u16, mut param_3: u32) {
    let mut iVar1: i16;
    let mut uVar3: u32;
    let mut lVar4: i32;
    let mut in_EDX: u32;
    let mut uVar6: u16;
    let mut pSVar5: *mut StructD;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut in_stack_0000ffb0: u16;
    let mut uVar9: u16;
    let mut local_6: [u8; 0x2] = [0; 0x2];
    let mut local_4: [u8; 0x2] = [0; 0x2];
    let mut uVar2: u32;

    uVar6 = (in_EDX >> 0x10);
    if (param_3 + -0x3 < 1) {
        param_3 = CONCAT22(0x3, param_3);
    }
    if (param_3 + -0x3 < 1) {
        param_3 = CONCAT22(param_3, 0x3);
    }
    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    uVar2 = (iVar7 + 0x5a);
    iVar1 = (uVar2 + 0x4);
    if (iVar1 <= param_3 + 0x2) {
        param_3 = param_3 & 0xffff | (iVar1 - 0x3) << 0x10;
    }
    uVar3 = (iVar7 + 0x5a);
    iVar1 = (uVar3 + 0x8);
    if (iVar1 <= param_3 + 0x2) {
        param_3 = param_3 & 0xffff0000 | (iVar1 - 0x3);
    }
    uVar9 = (param_3 >> 0x10);
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | (iVar7 + 0x30)),
        param_2,
        param_3,
        uVar9,
    );
    pSVar5 = CONCAT22(uVar6, uVar8);
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (iVar7 + 0x36)),
        CONCAT22(0x1050, local_6),
        CONCAT22(0x1050, local_4),
    );
    pass1_1008_3e76((param_1 & 0xffff0000 | (iVar7 + 0x36)), 0x0, param_3, uVar9);
    (iVar7 + 0x4c) = 0;
    lVar4 = (iVar7 + 0x3c);
    uVar3 = (iVar7 + 0x2c);
    if ((uVar3 + 0x20) == lVar4) {
        pass1_1018_028c(
            lVar4,
            pSVar5,
            (iVar7 + 0x2c),
            (iVar7 + 0x3c),
            in_stack_0000ffb0,
        );
        (iVar7 + 0x4c) = lVar4;
        (iVar7 + 0x4e) = pSVar5;
        pass1_1010_1f62(param_1, 0x4);
    }
    return;
}
pub unsafe fn pass1_1018_179e(mut param_1: u32, mut param_2: u32) {
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1008_3eb4(
        param_2,
        CONCAT22(0x1050, &local_8),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_6 + 0x2),
    );
    pass1_1018_16b8(param_1, local_8, local_6);
    return;
}
pub unsafe fn pass1_1018_17ce(mut param_1: u32, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1018_0412(
        (param_1 + 0x2c),
        param_2,
        CONCAT22(param_3, (param_2 >> 0x10)),
        (param_3 >> 0x10),
        (param_1 + 0x3c),
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_17f0() -> i16 {
    let mut iStack4: i16;

    iStack4 = 0;
    while (iStack4 < 0x4 && ((iStack4 * 0x2 + _PTR_LOOP_1050_3962) != 0)) {
        iStack4 += 0x1;
    }
    return iStack4;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_181c(param_1: *mut astruct_610, mut param_2: u32, param_3: *mut c_char) {
    pass1_1030_8344(_u16_1050_5748, (param_2 + 0x3c));
    pass1_1030_5b6c(param_1, CONCAT22(param_1, (param_1 >> 0x10)), param_3);
    return;
}

pub unsafe fn pass1_1018_1842(param_1: *mut u16, param_2: u8) -> *mut u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x20));
    pass1_1018_078e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_184e(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_078e(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_18b8(param_1: *mut astruct_19, mut param_2: u16) {
    let mut uVar2: u16;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u32;
    let mut iVar3: *mut astruct_19;
    let mut uVar3: *mut astruct_19;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u32;
    let mut uVar9: u32;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut piVar10: *mut i16;
    let mut uVar11: u16;
    let mut piVar12: *mut i16;
    let mut uVar13: u16;
    let mut local_6: i16;
    let mut local_4: i16;
    let mut uVar1: u16;

    uVar11 = (in_EDX >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, 0x0, param_2);
    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar3.field17_0x20 = 0x389a;
    iVar3.field18_0x22 = 0x1008;
    iVar3.field17_0x20 = 0x3aa8;
    iVar3.field18_0x22 = 0x1008;
    iVar3.field19_0x24 = 0;
    (&iVar3.field20_0x26 + 0x2) = 0x4;
    puVar7 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x3a)));
    paVar5 = CONCAT22(uVar11, (puVar7 >> 0x10));
    iVar3.field34_0x40 = 0;
    iVar3.field35_0x44 = 0;
    iVar3.field36_0x46 = 0;
    iVar3.field37_0x4a = 0;
    iVar3.field47_0x56 = 0;
    param_1.offset_0x0 = 0x1fb0;
    iVar3.segment_0x2 = 0x1018;
    iVar3.field17_0x20 = 0x1fec;
    iVar3.field18_0x22 = 0x1018;
    pass1_1000_4906(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x4e)),
        NULL,
        0x8,
    );
    piVar12 = &local_4;
    uVar13 = SUB42(&DAT_1050_1050, 0x0);
    piVar10 = &local_6;
    uVar11 = SUB42(&DAT_1050_1050, 0x0);
    puVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(piVar10, 0x48),
        in_stack_0000fe7e,
        in_stack_0000ffa2,
        in_stack_0000ffa8,
        in_stack_0000ffac,
    );
    uVar6 = paVar5 & 0xffff0000 | puVar8 >> 0x10;
    uVar2 = puVar8 + 0xe;
    pass1_1008_3e94(
        (puVar8 & 0xffff0000 | uVar2),
        CONCAT22(uVar11, piVar10),
        CONCAT22(uVar13, piVar12),
    );
    uVar4 = FUN_1010_830a(uVar2, uVar6, 0x1008, _u16_1050_14cc, 0x9a);
    iVar3.field19_0x24 = uVar4;
    iVar3.field20_0x26 = uVar6;
    uVar9 = pass1_1008_4772(CONCAT22(uVar6, iVar3.field19_0x24));
    uVar1 = (uVar9 >> 0x10);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field25_0x32)),
        NULL,
        0x8,
    );
    iVar3.field26_0x36 = (uVar9 + 0x4);
    iVar3.field27_0x38 = (uVar9 + 0x8);
    iVar3.field21_0x2a = local_4 + 0x14;
    iVar3.field22_0x2c = local_6 + 0x14;
    get_sys_metrics_1018_1ea0(param_1);
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x3a)),
        0x0,
        0x88,
        0x99,
    );
    return;
}
pub unsafe fn pass1_1018_1a04(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut puVar4: *mut u16;
    let mut iVar5: *mut StructD;
    let mut uVar5: *mut StructD;
    let mut puStack14: *mut u16;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.address_offset_field_0x0 = 0x1fb0;
    iVar5.address_offset_field_0x2 = 0x1018;
    iVar5.field19_0x20 = 0x1fec;
    iVar5.field20_0x22 = 0x1018;
    puVar1 = &iVar5.field_0x24;
    uVar2 = &iVar5.field_0x26;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*&iVar5.field_0x40);
    if (param_1.is_null()) {
        puVar4 = null_mut();
        uVar5 = null_mut();
    } else {
        puVar4 = &iVar5.field19_0x20;
    }
    puStack14 = CONCAT22(uVar5, puVar4);
    *puStack14 = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_1a8e(param_1: *mut u8, param_2: *mut astruct_653) {
    let mut piVar1: *mut i16;
    let mut lVar2: i32;
    let mut in_register_0000000a: u16;
    let mut iVar2: *mut astruct_653;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut piVar5: *mut i16;
    let mut uVar6: u16;
    let mut local_8: i16;
    let mut uStack6: u32;

    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    if (iVar2.field65_0x44 != 0) {
        if (iVar2.field66_0x46 != 0) {
            lVar2 = iVar2.field66_0x46;
            (lVar2 + 0xe) = 0;
            iVar2.field66_0x46 = 0;
        }
        piVar1 = &iVar2.field67_0x4a;
        *piVar1 = *piVar1 + 1;
        return;
    }
    piVar5 = &local_8;
    uVar6 = SUB42(&DAT_1050_1050, 0x0);
    puVar4 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(piVar5, 0x3),
        in_stack_0000fe98,
        in_stack_0000ffbc,
        in_stack_0000ffc2,
        in_stack_0000ffc6,
    );
    pass1_1010_bf1e(puVar4, (puVar4 >> 0x10), puVar4, CONCAT22(uVar6, piVar5));
    iVar2.field65_0x44 = local_8;
    iVar2.field64_0x40 = uStack6;
    pass1_1018_1ce8(param_2);
    return;
}

// WARNING: Unable to use type for symbol uVar2
pub unsafe fn pass1_1018_1b02(param_1: *mut astruct_95, mut param_2: i16) {
    let mut piVar1: *mut i16;
    let mut uVar3: u32;
    let mut pstruct96_4: *mut astruct_96;
    let mut pstruct95_5: *mut astruct_95;
    let mut uVar4: u16;
    let mut iStack12: i16;
    let mut local_6: u16;
    let mut local_4: [u8; 0x2] = [0; 0x2];
    let mut uVar2: u32;
    let mut pstruct96_2: *mut astruct_96;

    iStack12 = 0;
    loop {
        uVar4 = (param_1 >> 0x10);
        pstruct95_5 = param_1;
        piVar1 = &pstruct95_5.field65_0x44;
        if (*piVar1 == iStack12 || *piVar1 < iStack12) {
            break;
        }
        pstruct96_2 = pstruct95_5.field64_0x40;
        pstruct96_4 = pstruct96_2;
        pstruct96_4 = pstruct96_4 + iStack12;
        uVar2 = pstruct96_2 & 0xffff0000;
        uVar3 = ZEXT24(pstruct96_4);
        piVar1 = &pstruct96_4.field6_0x6;
        *piVar1 = *piVar1 + param_2 * 0x2 + -0x1;
        uVar4 = (uVar2 >> 0x10);
        if (0x23 < pstruct96_4.field6_0x6) {
            pstruct96_4.field6_0x6 = 0;
        }
        if (pstruct96_4.field6_0x6 < 0x0) {
            pstruct96_4.field6_0x6 = 0x23;
        }
        pass1_1008_3f62((uVar2 | ZEXT24(&pstruct96_4.field_0x10)), (uVar2 | uVar3));
        pstruct96_4.field19_0x16 = pstruct96_4.field8_0xa;
        pass1_1008_3e94(
            (uVar2 | uVar3),
            CONCAT22(0x1050, &local_6),
            CONCAT22(0x1050, local_4),
        );
        pass1_1008_3e76(
            (uVar2 | uVar3),
            0x0,
            local_6,
            ((pstruct96_4.field7_0x8 * 0x24 + pstruct96_4.field6_0x6) * 0x2 + 0x3c20),
        );
        pstruct96_4.field8_0xa = (pstruct96_4.field6_0x6 * 0x2 + 0x3966);
        iStack12 += 0x1;
    }
    pass1_1010_1f62(param_1, 0xd);
    return;
}

// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pt_in_rect_1018_1bda(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut BVar4: bool;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut iStack26: i16;
    let mut POPStack24: INT16;
    let mut local_14: i16;
    let mut local_12: i16;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut local_a: u32;
    let mut iStack6: i16;
    let mut iStack4: i16;

    uStack14 = 0;
    iVar3 = param_1;
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (iVar3 + 0x3a)),
        CONCAT22(0x1050, &local_14),
        CONCAT22(0x1050, &local_12),
    );
    PStack24 = CONCAT22(param_2, param_3);
    uStack16 = 0;
    iStack26 = 0;
    loop {
        uVar6 = (param_1 >> 0x10);
        piVar1 = (iVar3 + 0x44);
        if (*piVar1 == iStack26 || *piVar1 < iStack26) {
            return;
        }
        uVar2 = (iVar3 + 0x42);
        iVar5 = (iVar3 + 0x40) + iStack26 * 0x18;
        uStack14 = CONCAT22(uVar2, iVar5);
        pass1_1008_3e94(
            CONCAT22(uVar2, iVar5),
            CONCAT22(0x1050, &local_a + 0x2),
            CONCAT22(0x1050, &local_a),
        );
        local_a += local_12 + -0x6;
        iStack6 = local_a + 0xc;
        local_a += local_14 + -0x6;
        iStack4 = local_a + 0xc;
        BVar4 = PtInRect16(PStack24, &local_a);
        if (BVar4 != 0) {
            break;
        }
        iStack26 += 0x1;
    }
    pass1_1018_1eda(param_1, uStack14);
    return;
}

pub unsafe fn pass1_1018_1c9a(param_1: *mut astruct_263, mut param_2: i16) -> u16 {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut iStack10: i16;

    iStack10 = 0;
    loop {
        uVar4 = (param_1 >> 0x10);
        piVar1 = (param_1 + 0x44);
        if (*piVar1 == iStack10 || *piVar1 < iStack10) {
            return 0x0;
        }
        uVar2 = (param_1 + 0x40);
        uVar3 = uVar2 + iStack10 * 0x18;
        if (((uVar3 + 0xc) * 0x1e + 0x3c32) == param_2) {
            break;
        }
        iStack10 += 0x1;
    }
    pass1_1018_1eda(param_1, uVar2 & 0xffff0000 | uVar3);
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1018_1ce8(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iStack26: i16;
    let mut local_18: [u8; 0x2] = [0; 0x2];
    let mut local_16: [u8; 0x2] = [0; 0x2];
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut local_e: u16;
    let mut local_c: i16;
    let mut local_a: i16;
    let mut iStack8: i16;
    let mut uStack6: u32;

    uVar5 = (param_1 >> 0x10);
    iVar3 = param_1;
    uStack6 = (iVar3 + 0x40);
    iStack8 = 0;
    loop {
        piVar1 = (iVar3 + 0x44);
        if (*piVar1 == iStack8 || *piVar1 < iStack8) {
            return;
        }
        pass1_1008_3eb4(
            (uStack6 & 0xffff0000 | (iStack8 * 0x18 + uStack6)),
            CONCAT22(0x1050, &local_e),
            CONCAT22(0x1050, &local_c),
            CONCAT22(0x1050, &local_a),
        );
        local_a /= 0xa;
        iStack16 = local_c % 0xa;
        if (iStack16 != 0) {
            if (iStack16 < 0x6) {
                local_c -= iStack16;
            } else {
                local_c += 0xa - iStack16;
            }
        }
        iStack18 = pass1_1000_49b2(local_e);
        iStack18 /= 0x5;
        if (0x14 < iStack18) {
            iStack18 = 0x14;
            iVar2 = pass1_1000_49b2(local_e);
            local_e = (local_e / iVar2) * 0x64;
        }
        iStack16 = pass1_1000_49b2(local_e);
        iStack16 %= 0x5;
        if (iStack16 != 0) {
            if (local_e < 0x0) {
                iVar2 = iStack16;
                if (0x2 < iStack16) {
                    iVar2 = iStack16 + -0x5;
                }
                local_e += iVar2;
            } else if (iStack16 < 0x3) {
                local_e -= iStack16;
            } else {
                local_e += 0x5 - iStack16;
            }
        }
        iStack20 = (iStack18 * 0x48 + 0x3c20);
        if (local_c < iStack20) {
            //   for (iStack26 = iStack18; iStack26 < 0x15; iStack26 += 1)
            for iStack26 in iStack18..0x15 {
                piVar1 = (iStack26 * 0x48 + 0x3c20);
                if (*piVar1 == local_c || *piVar1 < local_c) {
                    iStack18 = iStack26;
                    break;
                }
                iStack26 += 1;
            }
        }
        pass1_1008_3e94(
            (param_1 & 0xffff0000 | (iVar3 + 0x3a)),
            CONCAT22(0x1050, local_18),
            CONCAT22(0x1050, local_16),
        );
        uVar4 = iStack8 * 0x18 + uStack6;
        (uVar4 + 0x6) = local_a;
        (uVar4 + 0x8) = iStack18;
        pass1_1008_3e76(
            (uStack6 & 0xffff0000 | uVar4),
            0x0,
            local_e,
            ((iStack18 * 0x24 + local_a) * 0x2 + 0x3c20),
        );
        (uVar4 + 0xa) = (local_a * 0x2 + 0x3966);
        iStack8 += 0x1;
    }
}

pub unsafe fn pass1_1018_1e78(mut param_1: u32, mut param_2: i16) -> u32 {
    let mut uVar1: u32;

    if (param_2 == -1) {
        uVar1 = (param_1 + 0x46);
        param_2 = (uVar1 + 0xc);
    }
    return CONCAT22(0x1050, param_2 * 0x1e + 0x3c18);
}
pub unsafe fn get_sys_metrics_1018_1ea0(param_1: *mut astruct_19) {
    let mut IVar1: i16;
    let mut IVar2: i16;
    let mut iVar3: *mut astruct_19;
    let mut uVar3: *mut astruct_19;

    IVar1 = GetSystemMetrics16(SM_CXBORDER);
    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar3.field23_0x2e = IVar1 * 0x2 + iVar3.field26_0x36;
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    IVar2 = GetSystemMetrics16(SM_CYBORDER);
    iVar3.field24_0x30 = IVar1 + iVar3.field27_0x38 + IVar2;
    return;
}
pub unsafe fn pass1_1018_1eda(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x46) != 0) {
        uVar1 = (iVar2 + 0x46);
        (uVar1 + 0xe) = 0x2;
    }
    (iVar2 + 0x46) = param_2;
    (param_2 + 0xe) = 0x1;
    pass1_1010_1f62(param_1, 0xd);
    return;
}

pub unsafe fn pass1_1018_1f1a(mut param_1: u32, mut param_2: i16) -> u16 {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut iStack6: i16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x56) == 0) {
        return 0x0;
    }
    iStack6 = 0;
    loop {
        piVar1 = (iVar2 + 0x56);
        if (*piVar1 == iStack6 || *piVar1 < iStack6) {
            return 0x0;
        }
        if ((iVar2 + 0x4e + iStack6 * 0x2) == param_2) {
            break;
        }
        iStack6 += 0x1;
    }
    return 0x1;
}

pub unsafe fn pass1_1018_1f6a(
    mut param_1: u16,
    param_2: *mut StructD,
    param_3: u8,
) -> *mut StructD {
    param_2 = (param_2 & 0xffff0000 | (param_2 - 0x20));
    pass1_1018_1a04(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}
pub unsafe fn FUN_1018_1f76() {
    return;
}

pub unsafe fn pass1_1018_1f7a(mut param_1: i16, mut param_2: u16) -> u32 {
    return CONCAT22(param_2, param_1 + 0x2a);
}

pub unsafe fn pass1_1018_1f8a(param_1: *mut StructD, param_2: u8) -> *mut u16 {
    pass1_1018_1a04(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return &param_1.address_offset_field_0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_1ff4(param_1: *mut astruct_19, mut param_2: u16) -> u32 {
    let mut piVar1: *mut i16;
    let mut in_EDX: u32;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_19;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe82: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut piVar5: *mut i16;
    let mut uVar6: u16;
    let mut piVar7: *mut i16;
    let mut uVar8: u16;
    let mut local_a: i16;
    let mut local_8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    uVar2 = (in_EDX >> 0x10);
    paVar3 = struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0xb9010b;
    (param_1 + 0xe) = 0x170035;
    param_1.offset_0x0 = 0x21e8;
    (param_1 + 0x2) = 0x1018;
    piVar7 = &local_8;
    uVar8 = SUB42(&DAT_1050_1050, 0x0);
    piVar5 = &local_a;
    uVar6 = SUB42(&DAT_1050_1050, 0x0);
    puVar4 = mixed_1010_20ba(
        CONCAT22(uVar2, (paVar3 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(piVar5, 0x48),
        in_stack_0000fe82,
        in_stack_0000ffa6,
        in_stack_0000ffac,
        in_stack_0000ffb0,
    );
    uStack4 = (puVar4 >> 0x10);
    iStack6 = puVar4;
    pass1_1008_3e94(
        (puVar4 & 0xffff0000 | (iStack6 + 0xe)),
        CONCAT22(uVar6, piVar5),
        CONCAT22(uVar8, piVar7),
    );
    piVar1 = (param_1 + 0xa);
    *piVar1 = *piVar1 + local_8;
    piVar1 = (param_1 + 0xc);
    *piVar1 = *piVar1 + local_a;
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x12)), NULL, 0x7f4);
    return param_1;
}
