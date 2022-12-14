// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_b0aa(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut iVar3: i16;
    let mut puVar4: *mut u32;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uStack20: u32;

    uVar7 = (_PTR_LOOP_1050_4e74 >> 0x10);
    if ((_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) == 0) {
        return;
    }
    if ((_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) != -1) {
        if (PTR_LOOP_1050_4e78.is_null()) {
            iVar3 = param_4;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000002);
            puVar1 = (iVar3 + 0xc);
            ppcVar2 = (*puVar1 + 0x10);
            puVar4 = puVar1;
            (**ppcVar2)();
            uVar6 = extraout_DX;
            //   for (uStack20 = 0; uStack20 < ; uStack20 += 1)
            for uStack20 in 0..(puVar4 & 0xffff | extraout_DX << 0x10) {
                uVar8 = pass1_1030_1d7c((puVar4 & 0xffff), uVar6, puVar1);
                uVar5 = (uVar8 >> 0x10);
                uVar6 = uVar5 | uVar8;
                if ((uVar6 != 0) && ((iVar3 = (uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b)))) {
                    PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 1);
                    break;
                }
            }
            if (PTR_LOOP_1050_4e78.is_null()) {
                PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 1);
                return;
            }
        }
        uVar6 = (_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) - 0x1;
        pass1_1008_612e(uVar6, 0x0, uVar6);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1020_b1ae(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut u16,
    mut param_6: u32,
) -> u16 {
    let mut puVar1: *mut u32;
    let mut local_14: i16;
    let mut local_12: i16;
    let mut local_10: i16;
    let mut local_e: i16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6);
    iStack6 = param_1;
    uStack4 = param_2;
    puVar1 = pass1_1030_5b5c(param_1, param_2);
    local_c = *puVar1;
    uStack8 = (puVar1 + 0x4);
    pass1_1008_3e94(
        param_5,
        CONCAT22(0x1050, &local_10),
        CONCAT22(0x1050, &local_e),
    );
    pass1_1008_3e94(
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_14),
        CONCAT22(0x1050, &local_12),
    );
    if ((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 -0xb))
        && (local_10 < local_14 -0xb))
    {
        return 0x1;
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_b240(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut bStack31: u8;
    let mut local_a: u32;
    let mut uStack6: u32;

    puVar1 = &local_a;
    uVar6 = (param_4 >> 0x10);
    pass1_1030_64ce(
        puVar1,
        param_1,
        _PTR_LOOP_1050_5740,
        param_3,
        (param_4 + 0x4),
        CONCAT22(0x1050, puVar1),
    );
    uStack6 = *puVar1;
    uVar5 = (puVar1 + 2);
    bStack31 = (uStack6 >> 0x18);
    uVar2 = bStack31;
    if (bStack31 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack6 & 0xffff | uVar5 << 0x10);
        uVar7 = struct_op_1030_73a8(CONCAT22(uVar5, uVar2), uVar2, uVar5);
        uVar4 = (uVar7 >> 0x10);
        uVar2 = uVar7;
        uVar5 = uVar4 | uVar2;
        if ((uVar5 != 0) && (uVar2 = (uVar2 + 0xc), 0x9 < uVar2)) {
            return;
        }
    }
    uVar3 = pass1_1020_b1ae(
        uVar2,
        uVar5,
        param_2,
        (param_2 >> 0x10),
        param_3,
        (param_4 + 0x4),
    );
    if (uVar3 == 0) {
        return;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1020_b2da(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    param_4: *mut u16,
    mut param_5: u32,
) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut in_stack_0000fe78: u16;
    astruct_19 * *ppaVar7;
    let mut iStack28: i16;
    let mut local_1a: [u8; 0x6] = [0; 0x6];
    let mut uStack20: u16;
    let mut uStack18: u16;
    let mut piStack16: *mut i16;
    let mut piStack12: *mut i16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_3 == 0) {
        uVar2 = 0x4e6a;
    } else {
        uVar2 = 0x4e6e;
    }
    piStack12 = CONCAT22(0x1050, uVar2);
    if (param_3 == 0) {
        uStack20 = 0x4e68;
    } else {
        uStack20 = 0x4e6c;
    }
    uStack18 = SUB42(&DAT_1050_1050, 0x0);
    piStack16 = CONCAT22(0x1050, uStack20);
    loop {
        if (param_3 == 0) {
            ppaVar7 = &PTR_1048_4230;
        } else {
            ppaVar7 = 0x10484236;
        }
        pass1_1008_3eb4(
            ppaVar7,
            CONCAT22(0x1050, &local_8),
            CONCAT22(0x1050, &local_6),
            CONCAT22(0x1050, &local_6 + 0x2),
        );
        iVar1 = *piStack12;
        if (iVar1 == 0) {
            local_6 = CONCAT22(local_6 + *piStack16, local_6 -1);
        } else if (iVar1 == 1) {
            local_6 = CONCAT22(local_6 -0x1, local_6 + *piStack16);
        } else if (iVar1 == 0x2) {
            local_6 = CONCAT22(local_6 - *piStack16, local_6 -1);
        }
        puVar6 = pass1_1008_3e54(
            CONCAT22(0x1050, local_1a),
            local_8,
            local_6,
            (local_6 >> 0x10),
        );
        uVar5 = (puVar6 >> 0x10);
        uVar2 = (param_5 >> 0x10);
        uVar3 = pass1_1020_b1ae(
            local_1a,
            uVar5,
            param_1,
            param_2,
            CONCAT22(0x1050, local_1a),
            (param_5 + 0x4),
        );
        if (uVar3 != 0) {
            puVar4 = local_1a;
            pass1_1020_b240(
                uVar5,
                CONCAT22(param_2, param_1),
                CONCAT22(0x1050, puVar4),
                param_5,
            );
            if (puVar4.is_null() == false) {
                //
                // LAB_1020_b46e:
                pass1_1008_3e76(param_4, local_8, local_6, (local_6 >> 0x10));
                return;
            }
        }
        iVar1 = *piStack12;
        if (iVar1 == 0) {
            //
            // LAB_1020_b45e:
            local_6 = local_6 & 0xffff0000 | (local_6 + 2);
        } else if (iVar1 == 1) {
            local_6 = local_6 & 0xffff | (local_6 + 0x2) << 0x10;
        }
        // else if (iVar1 == 0x2) goto LAB_1020_b45e;
        pass1_1008_3e76(
            CONCAT22(0x1050, local_1a),
            local_8,
            local_6,
            (local_6 >> 0x10),
        );
        uVar3 = pass1_1020_b1ae(
            local_1a,
            uVar5,
            param_1,
            param_2,
            CONCAT22(0x1050, local_1a),
            (param_5 + 0x4),
        );
        if (uVar3 != 0) {
            puVar4 = local_1a;
            pass1_1020_b240(
                uVar5,
                CONCAT22(param_2, param_1),
                CONCAT22(0x1050, puVar4),
                param_5,
            );
            //      if (puVar4.is_null() == false) goto LAB_1020_b46e;
        }
        iStack28 = *piStack12 + 1;
        if (0x2 < iStack28) {
            iStack28 = 0;
            *piStack16 = *piStack16 + 1;
        }
        *piStack12 = iStack28;
        pass1_1020_ac6e(
            in_stack_0000fe78,
            CONCAT22(param_2, param_1),
            param_3,
            *piStack16,
            iStack28,
        );
    }
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_b482(mut param_1: u32, param_2: *mut u32, mut param_3: u32) {
    let mut puVar1: *mut u8;
    let mut ppaVar2: *mut *mut astruct_92 = null_mut();
    let mut puVar3: *mut u32;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut puVar11: *mut u32;
    let mut iStack46: i16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut uStack32: u16;
    let mut lStack30: i32;
    let mut uStack26: u32;
    let mut local_16: *mut astruct_92;
    let mut local_4: [u8; 0x2] = [0; 0x2];

    uVar8 = pass1_1030_bcae(local_4, &DAT_1050_1050);
    uVar5 = (uVar8 >> 0x10);
    pass1_1028_dc52(CONCAT22(0x1050, &local_16), 0x1, 0x0, 0x400);
    loop {
        ppaVar2 = &local_16;
        pass1_1028_e4ec(CONCAT22(0x1050, ppaVar2));
        uStack26 = CONCAT22(uVar5, ppaVar2);
        uVar6 = uVar5 | ppaVar2;
        if (uVar6 == 0) {
            pass1_1020_b240(0x0, param_1, param_2, param_3);
            if (ppaVar2.is_null() == false) {
                lStack30 = (param_3 + 0x4);
                local_24 = *param_2;
                uStack32 = (param_2 + 0x4);
                puVar7 = &local_2a;
                pass1_1008_3eb4(
                    CONCAT22(0x1050, &local_24),
                    CONCAT22(0x1050, puVar7),
                    CONCAT22(0x1050, &local_2a + 0x2),
                    CONCAT22(0x1050, &local_26),
                );
                pass1_1008_3e76(
                    CONCAT22(0x1050, &local_24),
                    local_2a,
                    local_2a - 0x1,
                    local_26 - 1,
                );
                puVar3 = &local_24;
                uVar9 = param_1;
                uVar10 = (param_1 >> 0x10);
                pass1_1020_afc4(puVar7, uVar9, uVar10, CONCAT22(0x1050, puVar3), lStack30);
                if (puVar3.is_null() == false) {
                    pass1_1008_3e76(
                        CONCAT22(0x1050, &local_24),
                        local_2a,
                        (local_2a >> 0x10),
                        local_26 - 1,
                    );
                    puVar3 = &local_24;
                    pass1_1020_afc4(puVar7, uVar9, uVar10, CONCAT22(0x1050, puVar3), lStack30);
                    if (puVar3.is_null() == false) {
                        pass1_1008_3e76(
                            CONCAT22(0x1050, &local_24),
                            local_2a,
                            local_2a + 0x1,
                            local_26 - 1,
                        );
                        puVar3 = &local_24;
                        pass1_1020_afc4(puVar7, uVar9, uVar10, CONCAT22(0x1050, puVar3), lStack30);
                        if (puVar3.is_null() == false) {
                            pass1_1008_3e76(
                                CONCAT22(0x1050, &local_24),
                                local_2a,
                                local_2a - 0x1,
                                local_26,
                            );
                            puVar3 = &local_24;
                            pass1_1020_afc4(
                                puVar7,
                                uVar9,
                                uVar10,
                                CONCAT22(0x1050, puVar3),
                                lStack30,
                            );
                            if (puVar3.is_null() == false) {
                                pass1_1008_3e76(
                                    CONCAT22(0x1050, &local_24),
                                    local_2a,
                                    local_2a + 0x1,
                                    local_26,
                                );
                                puVar3 = &local_24;
                                pass1_1020_afc4(
                                    puVar7,
                                    uVar9,
                                    uVar10,
                                    CONCAT22(0x1050, puVar3),
                                    lStack30,
                                );
                                if (puVar3.is_null() == false) {
                                    pass1_1008_3e76(
                                        CONCAT22(0x1050, &local_24),
                                        local_2a,
                                        local_2a + 0x1,
                                        local_26 + 1,
                                    );
                                    puVar3 = &local_24;
                                    pass1_1020_afc4(
                                        puVar7,
                                        uVar9,
                                        uVar10,
                                        CONCAT22(0x1050, puVar3),
                                        lStack30,
                                    );
                                    if (puVar3.is_null() == false) {
                                        pass1_1008_3e76(
                                            CONCAT22(0x1050, &local_24),
                                            local_2a,
                                            (local_2a >> 0x10),
                                            local_26 + 1,
                                        );
                                        puVar3 = &local_24;
                                        pass1_1020_afc4(
                                            puVar7,
                                            uVar9,
                                            uVar10,
                                            CONCAT22(0x1050, puVar3),
                                            lStack30,
                                        );
                                        if (puVar3.is_null() == false) {
                                            pass1_1008_3e76(
                                                CONCAT22(0x1050, &local_24),
                                                local_2a,
                                                local_2a - 0x1,
                                                local_26 + 1,
                                            );
                                            puVar3 = &local_24;
                                            pass1_1020_afc4(
                                                puVar7,
                                                uVar9,
                                                uVar10,
                                                CONCAT22(0x1050, puVar3),
                                                lStack30,
                                            );
                                            if (puVar3.is_null() == false) {
                                                pass1_1008_3e76(
                                                    CONCAT22(0x1050, &local_24),
                                                    local_2a,
                                                    local_2a - 0x2,
                                                    local_26 - 0x2,
                                                );
                                                puVar3 = &local_24;
                                                pass1_1020_afc4(
                                                    puVar7,
                                                    uVar9,
                                                    uVar10,
                                                    CONCAT22(0x1050, puVar3),
                                                    lStack30,
                                                );
                                                if (puVar3.is_null() == false) {
                                                    pass1_1008_3e76(
                                                        CONCAT22(0x1050, &local_24),
                                                        local_2a,
                                                        local_2a + 0x2,
                                                        local_26 - 0x2,
                                                    );
                                                    puVar3 = &local_24;
                                                    pass1_1020_afc4(
                                                        puVar7,
                                                        uVar9,
                                                        uVar10,
                                                        CONCAT22(0x1050, puVar3),
                                                        lStack30,
                                                    );
                                                    if (puVar3.is_null() == false) {
                                                        pass1_1008_3e76(
                                                            CONCAT22(0x1050, &local_24),
                                                            local_2a,
                                                            local_2a - 0x2,
                                                            local_26 + 2,
                                                        );
                                                        puVar3 = &local_24;
                                                        pass1_1020_afc4(
                                                            puVar7,
                                                            uVar9,
                                                            uVar10,
                                                            CONCAT22(0x1050, puVar3),
                                                            lStack30,
                                                        );
                                                        if (puVar3.is_null() == false) {
                                                            pass1_1008_3e76(
                                                                CONCAT22(0x1050, &local_24),
                                                                local_2a,
                                                                local_2a + 0x2,
                                                                local_26 + 2,
                                                            );
                                                            puVar3 = &local_24;
                                                            pass1_1020_afc4(
                                                                puVar7,
                                                                uVar9,
                                                                uVar10,
                                                                CONCAT22(0x1050, puVar3),
                                                                lStack30,
                                                            );
                                                            if (puVar3.is_null() == false) {
                                                                pass1_1008_3e76(
                                                                    CONCAT22(0x1050, &local_24),
                                                                    local_2a,
                                                                    local_2a - 0x1,
                                                                    local_26 + 2,
                                                                );
                                                                puVar3 = &local_24;
                                                                pass1_1020_afc4(
                                                                    puVar7,
                                                                    uVar9,
                                                                    uVar10,
                                                                    CONCAT22(0x1050, puVar3),
                                                                    lStack30,
                                                                );
                                                                if (puVar3.is_null() == false) {
                                                                    pass1_1008_3e76(
                                                                        CONCAT22(0x1050, &local_24),
                                                                        local_2a,
                                                                        local_2a - 0x1,
                                                                        local_26 + 0x3,
                                                                    );
                                                                    puVar3 = &local_24;
                                                                    pass1_1020_afc4(
                                                                        puVar7,
                                                                        uVar9,
                                                                        uVar10,
                                                                        CONCAT22(0x1050, puVar3),
                                                                        lStack30,
                                                                    );
                                                                    if (puVar3.is_null() == false) {
                                                                        iStack46 = 0x3;
                                                                        loop {
                                                                            if (0x9 < iStack46) {
                                                                                return;
                                                                            }
                                                                            pass1_1008_3e76(
                                                                                CONCAT22(
                                                                                    0x1050,
                                                                                    &local_24,
                                                                                ),
                                                                                0x0,
                                                                                local_2a - iStack46,
                                                                                local_26,
                                                                            );
                                                                            puVar3 = &local_24;
                                                                            pass1_1020_afc4(
                                                                                puVar7,
                                                                                uVar9,
                                                                                uVar10,
                                                                                CONCAT22(
                                                                                    0x1050, puVar3,
                                                                                ),
                                                                                lStack30,
                                                                            );
                                                                            if (puVar3.is_null()) {
                                                                                break;
                                                                            }
                                                                            iStack46 += 0x1;
                                                                        }
                                                                        return;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return;
        }
        uVar4 = (ppaVar2 + 0x8);
        puVar11 = param_2;
        uVar8 = param_3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4);
        puVar1 = local_4;
        pass1_1030_bcbc(
            puVar1,
            CONCAT22(uVar4, 0x1050),
            CONCAT22(puVar11, uVar6),
            (puVar11 >> 0x10),
            uVar8,
        );
        if (puVar1 < 0x0) {
            break;
        }
        uVar5 = uVar6;
        if (puVar1 < 0x65) {
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_b872(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u8;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u16;
    let mut uVar8: u16;
    let mut local_136: [u8; 0x124] = [0; 0x124];
    let mut local_12: u32;
    let mut local_c: i16;
    let mut local_a: i16;
    let mut local_8: u32;
    let mut uStack4: u16;

    uVar8 = (param_2 >> 0x10);
    puVar6 = pass1_1030_5b5c(param_2, uVar8);
    local_8 = *puVar6;
    uStack4 = (puVar6 + 0x4);
    pass1_1008_3e94(
        CONCAT22(0x1050, &local_8),
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar1 = local_a - 0xa;
    pass1_1008_612e(uVar1, 0xa, uVar1);
    uVar2 = local_c - 0xa;
    pass1_1008_612e(uVar2, 0xa, uVar2);
    puVar7 = pass1_1008_3e54(CONCAT22(0x1050, &local_12), 0x0, uVar2, uVar1);
    uVar2 = (puVar7 >> 0x10);
    loop {
        puVar4 = &local_12;
        pass1_1020_b482(param_1, CONCAT22(0x1050, puVar4), param_2);
        if (puVar4.is_null() == false) {
            break;
        }
        uVar1 = local_a - 0xa;
        pass1_1008_612e(uVar1, 0xa, uVar1);
        uVar3 = local_c - 0xa;
        pass1_1008_612e(uVar3, 0xa, uVar3);
        pass1_1008_3e76(CONCAT22(0x1050, &local_12), 0x0, uVar3, uVar1);
    }
    struct_op_1028_8888(
        CONCAT22(0x1050, local_136),
        0x0,
        0xa,
        &local_12,
        &DAT_1050_1050,
        0x8000002,
        0x0,
        (param_2 + 0x4),
    );
    puVar5 = local_136;
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, puVar5));
    pass1_1020_b97e(puVar5, uVar2, param_1, (param_1 >> 0x10), 1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_b97e(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: i16,
) {
    let mut uVar1: u32;
    let mut local_e: i16;
    let mut local_c: u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut uStack6: u32;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000002);
    _PTR_LOOP_1050_4e70 = CONCAT22(param_2, param_1);
    uVar1 = (param_1 + 0x10);
    uStack6 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1);
    iStack10 = uVar1;
    uStack8 = param_2;
    pass1_1008_3f62(&PTR_1048_4230, CONCAT22(param_2, iStack10 + 0xc));
    pass1_1008_3e94(
        &PTR_1048_4230,
        CONCAT22(0x1050, &local_e),
        CONCAT22(0x1050, &local_c),
    );
    if (param_5 == 0) {
        pass1_1008_3e76(&PTR_1048_4230, 0x0, local_e + 0x1, local_c - 1);
        pass1_1008_3e94(
            &PTR_1048_4230,
            CONCAT22(0x1050, &local_e),
            CONCAT22(0x1050, &local_c),
        );
    }
    pass1_1008_3e76(0x10484236, 0x1, local_e - 0x2, local_c);
    return;
}
pub unsafe fn pass1_1020_ba2b() {
    init_globals_1020_96d4();
    pass1_1020_a426();
    return;
}
pub unsafe fn pass1_1020_ba3e(param_1: *mut astruct_172, mut param_2: u16, mut param_3: i16) {
    let mut iVar1: *mut astruct_172;
    let mut uVar1: *mut astruct_172;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1 = 0;
    iVar1.field2_0x4 = 0;
    iVar1.field3_0x6 = param_3;
    iVar1.field4_0x8 = param_2;
    if (iVar1.field3_0x6 == 0) {
        iVar1.field3_0x6 = 0x5;
    }
    pass1_1020_bcc4(param_1);
    return;
}
pub unsafe fn fn_ptr_1020_ba7e(param_1: u32) {
    fn_ptr_1000_17ce(*param_1);
    return;
}
pub unsafe fn pass1_1020_ba94(param_1: *mut i32) {
    let mut puVar1: *mut u16;
    let mut uStack8: u16;

    if (*param_1 == 0) {
        return;
    }
    uStack8 = 0;
    loop {
        puVar1 = (param_1 + 0x4);
        if (*puVar1 < uStack8 || *puVar1 == uStack8) {
            break;
        }
        uStack8 += 0x1;
    }
    return;
}

pub unsafe fn pass1_1020_bae6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) -> u32 {
    let mut puStack6: *mut u16;

    pass1_1020_bc92(CONCAT22(param_4, param_3), (param_4 >> 0x10));
    puStack6 = CONCAT22(param_2, param_1);
    if ((param_2 | param_1) != 0) {
        return CONCAT22((param_1 + 0x2), *puStack6);
    }
    return 0x0;
}
pub unsafe fn pass1_1020_bb16(
    param_1: u32,
    param_2: *mut u32,
    param_3: *mut u16,
    mut param_4: u16,
) {
    if ((param_1 + 0x4) < param_4) {
        *param_3 = 0;
        *param_2 = 0;
        return;
    }
    *param_3 = (param_4 * 0x6 + *param_1 + 0x4);
    *param_2 = (*param_1 + param_4 * 0x6);
    return;
}
pub unsafe fn pass1_1020_bb70(param_1: *mut i32, mut param_2: u16, mut param_3: u32) {
    let mut in_AX: u16;
    let mut in_DX: u16;

    pass1_1020_bba4(
        param_1,
        0x1,
        param_2,
        param_3,
        (param_3 >> 0x10),
        in_AX,
        in_DX,
    );
    return;
}
pub unsafe fn pass1_1020_bb8a(param_1: *mut i32, mut param_2: u16, mut param_3: u32) {
    let mut in_AX: u16;
    let mut in_DX: u16;

    pass1_1020_bba4(
        param_1,
        0x0,
        param_2,
        param_3,
        (param_3 >> 0x10),
        in_AX,
        in_DX,
    );
    return;
}

pub unsafe fn pass1_1020_bba4(
    param_1: *mut astruct_172,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u16,
    param_6: *mut u16,
    mut param_7: u16,
) -> BOOL16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut bVar3: bool;
    let mut puStack6: *mut u16;

    pass1_1020_bc92(&param_1.field0_0x0, param_5);
    puStack6 = CONCAT22(param_7, param_6);
    uVar1 = param_7 | param_6;
    if (uVar1 == 0) {
        pass1_1020_bc92(&param_1.field0_0x0, 0x0);
        uVar2 = uVar1 | param_6;
        if (uVar2 == 0) {
            pass1_1020_bcc4(param_1);
            pass1_1020_bc92(&param_1.field0_0x0, 0x0);
            if ((uVar2 | param_6) == 0) {
                return 0x0;
            }
            param_6[0x2] = param_5;
            uVar1 = uVar2;
        } else {
            param_6[0x2] = param_5;
        }
        if (param_2 != 0) {
            uVar2 = *param_6;
            bVar3 = CARRY2(uVar2, param_3);
            param_3 = uVar2 + param_3;
            param_4 = param_6[0x1] + param_4 + bVar3;
        }
        *param_6 = param_3;
        param_6[0x1] = param_4;
        pass1_1020_bc72(param_1);
    } else {
        if (param_2 != 0) {
            bVar3 = CARRY2(*puStack6, param_3);
            param_3 = *puStack6 + param_3;
            param_4 = param_6[0x1] + param_4 + bVar3;
        }
        *param_6 = param_3;
        param_6[0x1] = param_4;
    }
    return 0x1;
}
pub unsafe fn pass1_1020_bc72(param_1: *mut astruct_172) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 2);
    pass1_1000_4aea(param_1.field0_0x0, uVar1, (uVar1 >> 0x10), 0x6, 0xbd6c);
    return;
}
pub unsafe fn pass1_1020_bc92(param_1: *mut u16, mut param_2: u16) {
    let mut uVar1: u32;
    let mut local_c: [u8; 0x4] = [0; 0x4];
    let mut uStack8: u16;

    uStack8 = param_2;
    uVar1 = (param_1 + 2);
    pass1_1000_49c6(
        local_c,
        &DAT_1050_1050,
        *param_1,
        uVar1,
        (uVar1 >> 0x10),
        0x6,
        0xbd6c,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_bcc4(param_1: *mut astruct_172) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut pSVar4: *mut StructD;
    let mut pstruct172_5: *mut astruct_172;
    let mut pstruct172_4: *mut astruct_172;
    let mut lVar5: i32;
    let mut uStack12: u32;

    pstruct172_4 = (param_1 >> 0x10);
    pstruct172_5 = param_1;
    if (pstruct172_5.field2_0x4 == 0) {
        pSVar4 = (in_EDX & 0xffff0000);
        uVar2 = pstruct172_5.field3_0x6;
    } else {
        uVar3 = pstruct172_5.field2_0x4;
        puVar1 = &pstruct172_5.field4_0x8;
        uVar2 = uVar3 + *puVar1;
        pSVar4 = (in_EDX & 0xffff0000 | CARRY2(uVar3, *puVar1));
    }
    if (pSVar4 == 0) {
        if (param_1 == 0) {
            if (_PTR_LOOP_1050_5f2c == 0) {
                PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar4);
                PTR_LOOP_1050_5f2e = pSVar4;
            } else {
            }
            uVar3 = fn_ptr_op_1000_1708(
                uVar2 * 0x6,
                0x0,
                0x1,
                PTR_LOOP_1050_5f2c,
                PTR_LOOP_1050_5f2e,
            );
        } else {
            lVar5 = pass1_1000_0ed4(0x1, uVar2 * 0x6, 0x0, param_1, (param_1 >> 0x10));
            PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
            uVar3 = lVar5;
        }
        uStack12 = CONCAT22(PTR_LOOP_1050_5f2e, uVar3);
        if ((PTR_LOOP_1050_5f2e | uVar3) != 0) {
            pstruct172_5.field2_0x4 = uVar2;
            param_1 = uStack12;
            pass1_1020_bc72((param_1 & 0xffff | ZEXT24(pstruct172_4) << 0x10));
        }
    }
    return;
}

pub unsafe fn pass1_1020_bd6c(mut param_1: u32, mut param_2: u32) -> i16 {
    return (param_1 + 0x4) - (param_2 + 0x4);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1020_bd80(mut param_1: u16) -> u16 {
    let mut pcVar1: *mut c_char;
    let mut uVar2: u16;
    let mut uStack6: u16;

    match param_1 {
        0x1 | 0x6 => {
            uVar2 = 0x427;
        }
        // break;
        0x2 => {
            uVar2 = 0x428;
        }
        // break;
        0x3 | 0x7 => {
            uVar2 = 0x429;
        }
        // break;
        0x4 | 0x8 => {
            uVar2 = 0x425;
        }
        // break;
        0x5 | 0x9 => {
            uVar2 = 0x426;
        }
        // break;
        0xa => {
            uVar2 = 0x402;
        }
        // break;
        0xb | 0x37 => {
            uVar2 = 0x418;
        }
        // break;
        0xc | 0x35 | 0x36 => {
            uVar2 = 0x42a;
        }
        // break;
        0xd => {
            uVar2 = 0x5f7;
        }
        // break;
        0xe => {
            uVar2 = 0x5f6;
        }
        // break;
        0xf => {
            uVar2 = 0x403;
        }
        // break;
        0x10 => {
            uVar2 = 0x5f8;
        }
        // break;
        0x11 => {
            uVar2 = 0x404;
        }
        // break;
        0x12 => {
            uVar2 = 0x405;
        }
        // break;
        0x13 | 0x14 | 0x15 => {
            uVar2 = 0x406;
        }
        // break;
        0x16 | 0x19 => {
            uVar2 = 0x5f9;
        }
        // break;
        0x17 | 0x1a => {
            uVar2 = 0x5fa;
        }
        // break;
        0x18 => {
            uVar2 = 0x5fb;
        }
        // break;
        0x1b | 0x1c | 0x1d => {
            uVar2 = 0x408;
        }
        // break;
        0x1e | 0x1f | 0x20 => {
            uVar2 = 0x409;
        }
        // break;
        0x21 => {
            uVar2 = 0x42c;
        }
        // break;
        0x22 | 0x23 | 0x24 => {
            uVar2 = 0x40a;
        }
        // break;
        0x25 | 0x26 | 0x27 => {
            uVar2 = 0x40b;
        }
        // break;
        0x28 | 0x29 => {
            uVar2 = 0x40c;
        }
        // break;
        0x2a | 0x2b => {
            uVar2 = 0x40d;
        }
        // break;
        0x2c => {
            uVar2 = 0x40e;
        }
        // break;
        0x2d | 0x2e => {
            uVar2 = 0x40f;
        }
        // break;
        0x2f | 0x30 => {
            uVar2 = 0x410;
        }
        // break;
        0x31 | 0x32 => {
            uVar2 = 0x411;
        }
        // break;
        0x33 | 0x34 => {
            uVar2 = 0x416;
        }
        // break;
        0x38 | 0x39 => {
            uVar2 = 0x5fc;
        }
        // break;
        0x3a | 0x3b => {
            uVar2 = 0x419;
        }
        // break;
        0x3c | 0x3d => {
            uVar2 = 0x5fd;
        }
        // break;
        0x3e => {
            uVar2 = 0x5fe;
        }
        // break;
        0x3f => {
            uVar2 = 0x5ff;
        }
        // break;
        0x40 => {
            uVar2 = 0x600;
        }
        // break;
        0x41 => {
            uVar2 = 0x601;
        }
        // break;
        0x42 | 0x46 | 0x6b => {
            uVar2 = 0x602;
        }
        // break;
        0x43 => {
            uStack6 = s_bidLRoadConst_1050_4e7a;
            return uStack6;
        }
        0x44 => {
            uStack6 = s_bidRRoadConst_1050_4e88;
            return uStack6;
        }
        0x45 => {
            uStack6 = s_bidXRoadConst_1050_4e96;
            return uStack6;
        }
        0x47 => {
            uVar2 = 0x42b;
        }
        // break;
        0x48 | 0x49 | 0x4a | 0x70 | 0x71 | 0x72 => {
            uVar2 = 0x603;
        }

        0x4b => {
            uVar2 = 0x42d;
        }
        // break;
        0x4c => {
            uVar2 = 0x604;
        }
        // break;
        0x4d => {
            uVar2 = 0x605;
        }
        // break;
        0x4e => {
            uVar2 = 0x606;
        }
        // break;
        0x4f | 0x50 | 0x51 => {
            uVar2 = 0x41a;
        }
        // break;
        0x52 | 0x53 => {
            uVar2 = 0x41b;
        }
        // break;
        0x54 | 0x55 | 0x56 => {
            uVar2 = 0x41d;
        }
        // break;
        0x57 | 0x58 | 0x59 => {
            uVar2 = 0x41e;
        }
        // break;
        0x5a => {
            uVar2 = 0x41f;
        }
        // break;
        0x5b | 0x5c => {
            uVar2 = 0x421;
        }
        // break;
        0x5d | 0x5e | 0x5f => {
            uVar2 = 0x420;
        }
        // break;
        0x60 | 0x61 => {
            uVar2 = 0x607;
        }
        // break;
        0x62 | 0x63 => {
            uVar2 = 0x608;
        }
        // break;
        0x64 => {
            uVar2 = 0x609;
        }
        // break;
        0x65 => {
            uVar2 = 0x422;
        }
        // break;
        0x66 | 0x67 => {
            uVar2 = 0x423;
        }
        // break;
        0x68 | 0x69 => {
            uVar2 = 0x60a;
        }
        // break;
        0x6a => {
            uVar2 = 0x60b;
        }
        // break;
        0x6c | 0x6d => {
            uVar2 = 0x41c;
        }
        // break;
        0x6e => {
            uVar2 = 0x60c;
        }
        // break;
        0x6f => {
            uVar2 = 0x60d;
        }
        // break;
        0x73 | 0x77 => {
            uVar2 = 0x415;
        }
        // break;
        0x74 | 0x78 | 0x79 => {
            uVar2 = 0x412;
        }
        // break;
        0x75 => {
            uVar2 = 0x413;
        }
        // break;
        0x76 => {
            uVar2 = 0x414;
        }
        // break;
        0x7a => {
            uVar2 = 0x60e;
        }
        // break;
        0x7b => {
            uVar2 = 0x60f;
        }
        // break;
        0x7c => {
            uVar2 = 0x610;
        }
        // break;
        0x7d => {
            uVar2 = 0x611;
        }
        // break;
        0x7e => {
            uVar2 = 0x612;
        }
        // break;
        0x7f => {
            uVar2 = 0x613;
        }
        // break;
        0x80 => {
            uVar2 = 0x614;
        }
        // break;
        0x81 => {
            uVar2 = 0x615;
        }
        // break;
        0x82 => {
            uVar2 = 0x616;
        }
        // break;
        0x83 => {
            uVar2 = 0x617;
        }
        // break;
        0x84 => {
            uVar2 = 0x618;
        }
        // break;
        0x85 => {
            uVar2 = 0x619;
        }
        // break;
        0x86 => {
            uVar2 = 0x61a;
        }
        // break;
        0x87 => {
            uVar2 = 0x61b;
        }
        // break;
        0x88 => {
            uVar2 = 0x61c;
        }
        // break;
        0x89 => {
            uVar2 = 0x61d;
        }
        // break;
        _ => {
            uVar2 = 0x424;
        }
    };
    pcVar1 = load_string_1010_847e(_u16_1050_14cc, uVar2);
    return pcVar1;
}
