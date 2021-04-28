pub fn pass1_fn_1000_0c32(param_1: u16, astruct_126_: *mut astruct_126, param_3: u16) -> u16 {
    let puVar1: *mut u32;
    let paVar2: *mut astruct_145;
    let pbVar3: *mut byte;
    let piVar4: *mut i32;
    let mut uVar5: u32;
    let mut uVar6: i32;
    let local_BX__1: *mut astruct_126;
    let paVar7: *mut astruct_145;
    let mut iVar8: i32;
    let puVar9: *mut u32;
    let puVar10: *mut u32;
    let paVar11: *mut astruct_145;
    let mut uVar12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let local_c: u8;
    let mut local_a: u16;
    let local_8: *mut astruct_145;
    let mut local_6: u16;

    paVar11 = local_BX__1.field_0xe;
    local_6 = 0;
    paVar7 = paVar11;
    while (true) {
        while {
            uVar6 = paVar7;
            if (param_1 <= uVar6) {
                uVar6 = (uVar6 & 0xfffc) - param_1;
                puVar1 = &local_BX__1.field_0x12;
                local_8 = paVar7;
                let pu_var1_val = unsafe { *puVar1 };
                if (pu_var1_val < uVar6 || pu_var1_val == uVar6) {
                    local_e = param_1;
                    if ((param_3 & 6) == 0) {
                        local_8 = (uVar6 + paVar7);
                        local_8[-1].field_0x4 = uVar6;
                        paVar7 = uVar6 | 2;
                        puVar9 = paVar7.field_0x2;
                        pbVar3 = (&local_8.field_0x0 + param_1);
                        unsafe {
                            unsafe { *pbVar3 = *pbVar3 | 2 };
                            unsafe { *local_8 = param_1 | 1 };
                        }
                    } else {
                        unsafe {
                            unsafe { *paVar7 = param_1 & 0xff00 | *paVar7 & 2 | param_1 & 0xff | 1 }
                        };
                        (paVar7.field_0x4 + 2) = paVar7.field_0x2;
                        (paVar7.field_0x2 + 4) = paVar7.field_0x4;
                        puVar9 = (&paVar7.field_0x0 + param_1);
                        (puVar9 + (uVar6 - 2)) = uVar6;
                        unsafe { unsafe { *puVar9 = uVar6 | 2 } };
                        uVar6 = local_BX__1.field_0x10;
                        puVar9[2] = uVar6;
                        puVar9[1] = (uVar6 + 2);
                        *((uVar6 + 2) + 4) = puVar9;
                        *(uVar6 + 2) = puVar9;
                    }
                } else {
                    puVar9 = paVar7.field_0x2;
                    *(paVar7.field_0x4 + 2) = puVar9;
                    (paVar7.field_0x2 + 4) = paVar7.field_0x4;
                    paVar2 = paVar7;
                    unsafe { unsafe { *paVar2 = *paVar2 | 1 } };
                    local_e = paVar7 & 0xfffc;
                    puVar10 = (&paVar7.field_0x0 + local_e);
                    unsafe { unsafe { *puVar10 = *puVar10 | 2 } };
                }
                local_BX__1.field_0xe = puVar9;
                if ((param_3 & 1) != 0) {
                    uVar6 = local_e - 2 >> 1;
                    paVar11 = local_8;
                    while (paVar11 = &paVar11.field_0x2, uVar6 != 0) {
                        uVar6 = uVar6 - 1;
                        paVar11 = 0;
                    }
                    if ((local_e - 2 & 1) != 0) {
                        unsafe { unsafe { *paVar11 = 0 } };
                    }
                }
                if (((param_3 & 2) != 0) && (puVar9[1] == puVar9[2])) {
                    local_BX__1.field_0x4 = *(local_BX__1.field_0x10 + 2) & 0xfffc;
                    uVar5 = local_BX__1.field_0x4;
                    pbVar3 = (uVar5 + 3);
                    unsafe { unsafe { *pbVar3 = *pbVar3 | 0x80 } };
                }
                piVar4 = &local_BX__1.field_0xa;
                unsafe { unsafe { *piVar4 = *piVar4 + 1 } };
                return CONCAT22(0x1050, &local_8.field_0x2);
            }
            if (local_6 < uVar6) {
                local_6 = uVar6;
            }
            paVar7 = paVar7.field_0x2;
            (paVar7 != paVar11)
        } {}
        if (((param_3 & 2) == 0) || ((param_3 & 0x40) != 0)) {
            break;
        }
        uVar5 = (local_BX__1).field_0x0;
        uVar12 = (uVar5 >> 0x10);
        iVar8 = uVar5;
        if ((iVar8 + 0x34) == 0) {
            break;
        }
        local_6 = (**(iVar8 + 0x34))();
        if ((local_6 < param_1) || (paVar7 = local_BX__1.field_0xe, paVar7 == 0x0)) {
            break;
        }
    }
    local_BX__1.field_0x4 = local_6 & 0xfffc;
    return 0;
}

pub fn pass1_fn_1000_1dfa(param_1: i32, param_2: u8, uparam_3: i32, uparam_4: i32) -> bool {
    let uVar1: u32;
    let mut uVar2: i32;

    if ((param_2 & 4) == 0) {
        uVar2 = (((-((param_2 & 2) == 0) >> 8) & 0xfe) + 0x92) << 8;
    } else {
        uVar2 = 0x1800;
    }
    if ((param_4 == 0)
        || ((param_4 & 0xff00 & (((-((param_2 & 4) == 0) >> 8) & 0x82) + 0x18) << 8) != uVar2))
    {
        return 1;
    }
    if (param_1 != 0) {
        uVar1 = SegmentLimit(param_4);
        if (CARRY2(param_3, param_1 - 1)) {
            return 1;
        }
        if (uVar1 < param_3 + (param_1 - 1)) {
            return 1;
        }
    }
    return 0;
}

pub fn pass1_fn_1000_206c(param_1: *mut astruct_148, param_2: u16) -> u16 {
    let mut iVar1: i32;

    iVar1 = pass1_fn_1000_21d2(0x102, 0x42, 0, param_1, param_2);
    if ((iVar1 != 0) && (param_1.field_0x14 == -0x4153)) {
        return 1;
    }
    return 0;
}

pub fn pass1_fn_1000_20a2(param_1: *mut astruct_147, param_2: u16) {
    let mut uVar1: u16;
    let paVar2: *mut astruct_147;
    let mut uvar3: u16;
    let paVar4: *mut astruct_147;
    let local_DI_58: *mut astruct_147;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_5fbe0711d8: *mut astruct_146;

    temp_5fbe0711d8 = (param_1 + 1);
    uVar1 = &param_1[1].field_0x2;
    local_8 = 0;
    paVar2 = temp_5fbe0711d8.field_0x4;
    local_4 = temp_5fbe0711d8.field_0x6;
    local_DI_58 = 0x0;
    if ((local_4 | paVar2) != 0) {
        while ((
            paVar4 = paVar2,
            uVar3 = local_4,
            paVar4 != param_1 || (local_4 != param_2),
        )) {
            paVar2 = paVar4.field_0x2a;
            local_4 = paVar4.field_0x2c;
            local_DI_58 = paVar4;
            local_8 = uVar3;
            if ((local_4 | paVar2) == 0) {
                return;
            }
        }
        if ((local_8 | local_DI_58) != 0) {
            uVar3 = paVar4.field_0x2c;
            local_DI_58.field_0x2a = paVar4.field_0x2a;
            local_DI_58.field_0x2c = uVar3;
            return;
        }
        uVar3 = paVar4.field_0x2c;
        temp_5fbe0711d8.field_0x4 = paVar4.field_0x2a;
        temp_5fbe0711d8.field_0x6 = uVar3;
    }
    return;
}

pub fn pass1_fn_1000_21b6(uparam_1: i32, uparam_2: i32) -> u32 {
    let BVar1: bool;

    BVar1 = pass1_fn_1000_1dfa(0, 4, param_1, param_2);
    return (BVar1 == 0);
}

// WARNING: Removing unreachable block (ram,0x100021de)

pub fn pass1_fn_1000_21d2(
    uparam_1: i32,
    param_2: libc::c_long,
    uparam_3: i32,
    uparam_4: i32,
) -> u16 {
    let uVar1: u32;
    let BVar2: bool;

    BVar2 = pass1_fn_1000_1dfa(0, param_1, param_3, param_4);
    if (BVar2 == 0) {
        if ((param_1 & 4) == 0) {
            uVar1 = SegmentLimit(param_4);
            if ((uVar1 >> 0x10) & 1) {
                if (param_2 == 0) {
                    return 1;
                }
                if ((!CARRY4(param_3, param_2 - 1)) && (param_3 + (param_2 - 1) <= uVar1)) {
                    return 1;
                }
            }
        } else {
            BVar2 = pass1_fn_1000_22c0(param_3, param_4, param_2, param_1, param_2._2_2_);
            if (BVar2 != 0) {
                return 1;
            }
        }
    }
    return 0;
}

pub fn pass1_fn_1000_2242(
    uparam_1: i32,
    uparam_2: i32,
    uparam_3: i32,
    param_4: i32,
    param_5: u16,
    param_6: *mut void,
) {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut bVar3: bool;
    let mut local_c: u16;
    let mut local_a: u16;

    uVar1 = param_2 | param_1;
    while (true) {
        if (uVar1 == 0) {
            return 0;
        }
        uVar1 = param_1;
        if (param_2 != 0) {
            uVar1 = 0xffff;
        }
        if (CARRY2(param_3, uVar1) != false) {
            uVar1 = -param_3;
        }
        bVar3 = param_1 < uVar1;
        param_1 = param_1 - uVar1;
        param_2 = param_2 - bVar3;
        uVar2 = (uVar1, param_5, param_3, param_4);
        if (uVar2 != 0) {
            break;
        }
        bVar3 = CARRY2(param_3, uVar1);
        param_3 = param_3 + uVar1;
        param_4 = param_4 + bVar3 * 0x100;
        uVar1 = param_2 | param_1;
    }
    return CONCAT22(param_2 + CARRY2(uVar2, param_1), uVar2 + param_1);
}

pub fn pass1_fn_1000_22c0(
    uparam_1: i32,
    param_2: i32,
    uparam_3: i32,
    param_4: u16,
    uparam_5: i32,
) -> bool {
    let mut u_var1: u32;

    uVar1 = pass1_fn_1000_2242(
        param_3,
        param_5,
        param_1,
        param_2,
        param_4,
        (s_1037a_bmp_1050_1df1 + 9),
    );
    if (uVar1 == 0) {
        return 1;
    }
    return 0;
}

pub fn pass1_fn_1000_23c1(uparam_1: i32) {
    process_win_msg_1008_54aa(
        &PTR_LOOP_1050_5f52,
        &PTR_LOOP_1050_5f4e,
        &PTR_LOOP_1050_5f50,
        &PTR_LOOP_1050_5f4a,
        &g_h_instance,
    );
    return;
}

// WARNING: Removing unreachable block (ram,0x10002557)

pub fn pass1_fn_1000_24ee(param_1: u16, uparam_2: i32) {
    let pcVar1: *mut code;
    let mut cVar2: u8;

    *&PTR_LOOP_1050_5fc9 = 1;
    cVar2 = 0x1;
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_2594();
    empty_fn_1000_55ac(param_1);
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_256b();
    if (cVar2 == '\0') {
        unsafe {
            pcVar1 = swi(0x21);
            (*pcVar1)();
        }
    }
    return;
}

pub fn pass1_fn_1000_25a8() {
    pass1_fn_1000_2913(0xfc);
    pass1_fn_1000_2913(0xff);
    return;
}

pub fn pass1_fn_1000_25ac() {
    pass1_fn_1000_2913(0xfc);
    pass1_fn_1000_2913(0xff);
    return;
}

pub fn pass1_fn_1000_2913(param_1: *mut char) {
    let pcVar1: *mut char;
    let pcVar2: *mut char;
    let mut iVar3: i32;
    let mut unaff_ES: u16;

    if (PTR_LOOP_1050_61ec != 0x0) {
        pcVar2 = process_string_1000_28dc(param_1);
        if (pcVar2 != 0x0) {
            iVar3 = -1;
            while {
                if (iVar3 == 0) {
                    break;
                }
                iVar3 = iVar3 + -1;
                pcVar1 = pcVar2;
                pcVar2 = pcVar2 + 1;
                let pc_var1_val = unsafe { *pcVar1 };
                pv_var1_val != '\0'
            } {}
            process_string_1000_55b1();
        }
    }
    return;
}

pub fn pass1_fn_1000_29af() {
    let mut in_AX: i32;

    pass1_fn_1000_29b5(in_AX & 0xff);
    return;
}

pub fn pass1_fn_1000_29b5(param_1: u16) {
    let char1: u8;

    PTR_LOOP_1050_5f88._0_1_ = param_1;
    char1 = (param_1 >> 8);
    if (char1 != '\0') {}
    // goto LAB_1000_29d2;
    if (PTR_LOOP_1050_5f88 < 0x22) {
        if (PTR_LOOP_1050_5f88 < 0x20) {
            if (0x13 < PTR_LOOP_1050_5f88) {}
            // goto LAB_1000_29cc;
        } else {
            param_1 = 5;
        }
    } else {
        // LAB_1000_29cc:
        param_1 = 0x13;
    }
    unsafe { char1 = *((param_1 & 0xff) + 0x5fd6) };
    // LAB_1000_29d2:
    PTR_LOOP_1050_5f78 = char1;
    return;
}

pub fn pass1_fn_1000_2b02(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> u16 {
    let paVar1: *mut astruct_153;
    let piVar2: *mut i32;
    let mut local_6: u16;

    paVar1 = pass1_fn_1000_35aa();
    if ((param_6 | paVar1) == 0) {
        piVar2 = 0x0;
    } else {
        piVar2 = pass1_fn_1000_2d34(
            param_1,
            param_2,
            CONCAT22(param_4, param_3),
            param_5,
            paVar1,
        );
    }
    return piVar2;
}

pub fn pass1_fn_1000_2b3c(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    pass1_fn_1000_2b02(param_1, param_2, param_3, param_4, 0, param_5);
    return;
}

pub fn pass1_fn_1000_2b5c(param_1: u16, param_2: u16, param_2: u16_00, param_4: u16) -> u16 {
    let mut local_AX_15: u16;
    let mut uVar1: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    local_AX_15 = process_struct_1000_2e74(param_1);
    uVar1 = pass1_fn_1000_30b4(
        param_1,
        &g_alloc_addr_1050_1050,
        CONCAT22(param_4, param_2_00),
    );
    process_struct_1000_2f00(local_AX_15, param_1);
    return uVar1;
}

pub fn pass1_fn_1000_2d34(
    param_1: i32,
    param_2: i32,
    param_3: *mut byte,
    param_4: i32,
    param_5: *mut astruct_153,
) -> *mut i32 {
    let mut bVar1: u8;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut local_e: u8;
    let mut local_c: u16;
    let mut local_a: u16;
    let local_8: u8;
    let mut local_6: u8;

    local_8 = PTR_LOOP_1050_6062;
    bVar3 = false;
    unsafe { unsafe { bVar1 = *param_3 } };
    if (bVar1 == 0x77) {
        uVar5 = 0x301;
    } else {
        if (0x77 < bVar1) {
            return 0x0;
        }
        if (bVar1 != 0x61) {
            if (bVar1 != 0x72) {
                return 0x0;
            }
            uVar5 = 0;
            local_6 = 1;
            // goto LAB_1000_2d6c;
        }
        uVar5 = 0x109;
    }
    local_6 = 2;
    // LAB_1000_2d6c:
    bVar2 = true;
    // LAB_1000_2d71:
    param_3 = (param_3 & 0xffff0000 | (param_3 + 1));
    let param_3_val = unsafe { *param_3 };
    if ((param_3_val == 0) || (!bVar2)) {
        iVar4 = dos3_call_1000_370a(param_1, param_2, uVar5, param_4, 0x1a4);
        if (iVar4 < 0) {
            return 0x0;
        }
        PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 1;
        *&param_5.field_0xa = local_6;
        param_5.field_0x2 = 0;
        (param_5).field_0x0 = 0;
        param_5.field_0x8 = 0;
        param_5.field_0x6 = 0;
        local_e = iVar4;
        *&param_5.field_0xb = local_e;
        param_5.field_0xf0 = local_8;
        param_5.field_0x4 = 0;
        param_5.field_0xf4 = 0;
        return param_5;
    }
    unsafe { unsafe { bVar1 = *param_3 } };
    if (bVar1 == 0x74) {
        if ((uVar5 & 0xc000) == 0) {
            uVar5 = uVar5 | 0x4000;
            // goto LAB_1000_2d71;
        }
    } else {
        if (0x74 < bVar1) {}
        // goto LAB_1000_2da4;
        if (bVar1 == 0x2b) {
            if ((uVar5 & 2) != 0) {}
            // goto LAB_1000_2da4;
            uVar5 = uVar5 & 0xfffe | 2;
            local_6 = 0x80;
            // goto LAB_1000_2d71;
        }
        if (bVar1 == 0x62) {
            if ((uVar5 & 0xc000) == 0) {
                uVar5 = uVar5 | 0x8000;
                // goto LAB_1000_2d71;
            }
        } else {
            if (bVar1 != 99) {
                if ((bVar1 != 0x6e) || (bVar3)) {}
                // goto LAB_1000_2da4;
                bVar3 = true;
                local_8 = local_8 & 0xbf;
                // goto LAB_1000_2d71;
            }
            if (!bVar3) {
                bVar3 = true;
                local_8 = local_8 | 0x40;
                // goto LAB_1000_2d71;
            }
        }
    }
    // LAB_1000_2da4:
    bVar2 = false;
    // goto LAB_1000_2d71;
}

pub fn pass1_fn_1000_2f48(param_1: *mut astruct_156, param_2: u16) -> i32 {
    let mut iVar1: i32;

    if ((param_2 | param_1) == 0) {
        iVar1 = pass1_fn_1000_3038(0);
    } else {
        iVar1 = pass1_fn_1000_2fa4(param_1);
        if (iVar1 == 0) {
            if ((param_1.field_0xf0 & 0x40) != 0) {
                iVar1 = pass1_fn_1000_400a(param_1.field_0xb);
                iVar1 = -(iVar1 != 0);
            }
        } else {
            iVar1 = -1;
        }
    }
    return iVar1;
}

pub fn pass1_fn_1000_2fa4(param_1: *mut astruct_157) -> u16 {
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut local_DI_8: u16;
    let mut local_4: u16;

    local_DI_8 = 0;
    bVar2 = param_1.field_0xa;
    if (((bVar2 & 3) == 2) && ((bVar2 & 8) != 0 || ((param_1.field_0xf0 & 1) != 0))) {
        iVar3 = (param_1).field_0x0 - param_1.field_0x6;
        if (0 < iVar3) {
            iVar4 = dos3_call_1000_39f2(
                param_1.field_0xb,
                param_1.field_0x6,
                param_1.field_0x8,
                iVar3,
            );
            if (iVar4 == iVar3) {
                if ((param_1.field_0xa & 0x80) != 0) {
                    pbVar1 = &param_1.field_0xa;
                    unsafe { *pbVar1 = *pbVar1 & 0xfd };
                }
            } else {
                pbVar1 = &param_1.field_0xa;
                unsafe { *pbVar1 = *pbVar1 | 0x20 };
                local_DI_8 = 0xffff;
            }
        }
    }
    iVar3 = param_1.field_0x8;
    (param_1).field_0x0 = param_1.field_0x6;
    param_1.field_0x2 = iVar3;
    param_1.field_0x4 = 0;
    return local_DI_8;
}

pub fn pass1_fn_1000_3024() {
    let mut unaff_BP: i32;

    pass1_fn_1000_3038(1, &g_alloc_addr_1050_1050, unaff_BP + 1);
    return;
}

pub fn pass1_fn_1000_3038(param_1: i32, param_2: u32, param_3: u32) -> u16 {
    let mut iVar1: i32;
    let paVar2: *mut astruct_156;
    let mut uvar3: u16;
    let mut local_4: u16;

    paVar2 = &PTR_LOOP_1050_6210;
    uVar3 = 0;
    local_4 = 0;
    while (paVar2 <= PTR_LOOP_1050_5ff0) {
        if ((param_1 == 1) && ((paVar2.field_0xa & 0x83) != 0)) {
            iVar1 = pass1_fn_1000_2f48(paVar2, &g_alloc_addr_1050_1050);
            if (iVar1 != -1) {
                uVar3 = uVar3 + 1;
            }
        } else {
            if ((param_1 == 0)
                && ((paVar2.field_0xa & 2) != 0
                    && (
                        iVar1 = pass1_fn_1000_2f48(paVar2, &g_alloc_addr_1050_1050),
                        iVar1 == -1,
                    )))
            {
                local_4 = 0xffff;
            }
        }
        paVar2 = &paVar2.field_0xc;
    }
    if (param_1 == 1) {
        local_4 = uVar3;
    }
    return local_4;
}

// WARNING: Variable defined which should be unmapped: local_c
// WARNING: Variable defined which should be unmapped: local_6
// WARNING: Could not reconcile some variable overlaps

pub fn pass1_fn_1000_30b4(param_1: u16, param_2: u16, param_1_00: *mut byte) -> u16 {
    let mut bVar1: u8;
    let mut uVar2: u16;
    let mut unaff_BP: i32;
    let mut unaff_SI: i32;
    let mut local_c: u16;
    let local_9: u8;
    let local_6: u8;

    bad_1000_25f2(&g_alloc_addr_1050_1050, unaff_BP + 1);
    unsafe { bVar1 = *param_1_00 };
    _local_6 = unaff_SI & 0xff00 | bVar1;
    if (bVar1 == 0) {
        return 0;
    }
    if ((bVar1 - 0x20) < 0x59) {
        unsafe { bVar1 = *((bVar1 - 0x20) + 0x5ffe) & 0xf };
    } else {
        bVar1 = 0;
    }
    // WARNING: Could not emulate address calculation at 0x10003101
    // WARNING: Treating indirect jump as call
    uVar2 = (**((*((bVar1 * 0x8) + 0x5ffe) >> 4) * 2 + 0x30a4))(_local_6);
    return uVar2;
}

pub fn pass1_fn_1000_34cf() -> u16 {
    let mut uVar1: u16;
    let p_uvar2: *mut u16;
    let local_BP__1: *mut astruct_158;
    let mut unaff_SS: u16;

    puVar2 = &local_BP__1.field_0xe;
    unsafe { uVar1 = *puVar2 };
    local_BP__1.field_0xe = puVar2 + 2;
    return uVar1;
}

pub fn pass1_fn_1000_34d8() -> u32 {
    let uVar1: u32;
    let puVar2: *mut u3232_t;
    let local_BP__1: *mut astruct_159;
    let mut unaff_SS: u16;

    unsafe { puVar2 = &local_BP__1.field_0xe };
    unsafe { uVar1 = *puVar2 };
    local_BP__1.field_0xe = puVar2 + 4;
    return uVar1;
}

pub fn pass1_fn_1000_34e6() {
    let mut iVar1: i32;
    let local_BP__1: *mut astruct_160;
    let mut unaff_SS: u16;

    if ((*(local_BP__1 + -6) & 0x20) != 0) {
        pass1_fn_1000_34d8();
        return;
    }
    iVar1 = pass1_fn_1000_34cf();
    if (iVar1 == 0) {
        return;
    }
    return;
}

pub fn pass1_fn_1000_3503(param_1: u32) -> u16 {
    let piVar1: *mut i32;
    let puVar2: *mut u328_t;
    **ppuVar3;
    let mut iVar4: i32;
    let local_BX_3: *mut astruct_161;
    let local_BP__1: *mut astruct_162;
    let mut uVar5: u16;
    let mut unaff_SS: u16;

    ppuVar3 = local_BP__1.field_0x6;
    uVar5 = (ppuVar3 >> 0x10);
    local_BX_3 = ppuVar3;
    piVar1 = &local_BX_3.field_0x4;
    unsafe { *piVar1 = *piVar1 + -1 };
    let pi_var1_val = unsafe { *piVar1 };
    if (pi_var1_val < 0) {
        iVar4 = dos3_call_1000_2bb6(param_1, local_BX_3, uVar5);
        if (iVar4 == -1) {
            return 0xffff;
        }
    } else {
        unsafe { puVar2 = *ppuVar3 };
        unsafe { *ppuVar3 = *ppuVar3 + 1 };
        unsafe { *puVar2 = param_1 };
    }
    return 0;
}

pub fn pass1_fn_1000_3534(param1: u16) {
    let piVar1: *mut i32;
    let puVar2: *mut u328_t;
    let mut uvar3: u16;
    let local_BP__1: *mut astruct_163;
    let unaff_DI: *mut u328_t;
    let mut uVar4: i32;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;

    if (param1 != 0) {
        piVar1 = (local_BP__1 + -10);
        unsafe { *piVar1 = *piVar1 + param1 };
        uVar4 = 0;
        while {
            puVar2 = unaff_DI;
            unaff_DI = unaff_DI + 1;
            let pu_var2_val = unsafe { *puVar2 };
            uVar3 = pass1_fn_1000_3503(pu_var2_val);
            uVar4 = uVar4 | uVar3;
            param1 = param1 - 1;
            param1 != 0
        } {}
        if (uVar4 != 0) {
            (local_BP__1 + -10) = 0xffff;
        }
    }
    return;
}

pub fn pass1_fn_1000_3552(param_1: u16, param_2: u32) {
    let piVar1: *mut i32;
    let mut uVar2: u16;
    let mut unaff_BP: i32;
    let mut uVar3: i32;
    let mut unaff_SS: u16;

    if (param_1 != 0) {
        piVar1 = (unaff_BP + -10);
        unsafe { *piVar1 = *piVar1 + param_1 };
        uVar3 = 0;
        while {
            uVar2 = pass1_fn_1000_3503(param_2);
            uVar3 = uVar3 | uVar2;
            param_1 = param_1 - 1;
            param_1 != 0
        } {}
        if (uVar3 != 0) {
            (unaff_BP + -10) = 0xffff;
        }
    }
    return;
}

pub fn pass1_fn_1000_356e(param_1: u16, param_2: u16, param_3: u16) {
    let pbVar1: *mut byte;
    let mut uVar2: u32;
    let mut bVar3: u8;
    let mut unaff_BP: i32;
    let mut unaff_SI: i32;
    let unaff_DI: *mut byte;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;

    while ((0 < unaff_SI || (param_1 != 0)) || (param_3 != 0)) {
        uVar2 = param_3;
        param_3 = param_3 / param_2;
        uVar2 = uVar2 % param_2 << 0x10 | param_1;
        param_1 = (uVar2 / param_2);
        bVar3 = (uVar2 % param_2) + 0x30;
        if (0x39 < bVar3) {
            bVar3 = bVar3 + *(unaff_BP + -3);
        }
        pbVar1 = unaff_DI;
        unaff_DI = unaff_DI + -1;
        unsafe { *pbVar1 = bVar3 };
        unaff_SI = unaff_SI + -1;
    }
    return;
}

pub fn pass1_fn_1000_35aa() -> *mut astruct_164 {
    let local_SI_9: *mut astruct_164;
    let mut local_8: u16;
    let mut local_6: u16;

    local_SI_9 = &PTR_LOOP_1050_6210;
    while (true) {
        if (PTR_LOOP_1050_5ff0 < local_SI_9) {
            return 0x0;
        }
        if ((*&local_SI_9.field_0xa & 0x83) == 0) {
            break;
        }
        local_SI_9 = &local_SI_9.field_0xc;
    }
    *&local_SI_9.field_0xa = 0;
    local_SI_9.field_0x4 = 0;
    local_SI_9.field_0x8 = 0;
    local_SI_9.field_0x6 = 0;
    local_SI_9.field_0x2 = 0;
    (local_SI_9).field_0x0 = 0;
    *&local_SI_9.field_0xb = 0xff;
    return local_SI_9;
}

pub fn pass1_fn_1000_3bac() -> i32 {
    let mut iVar1: i32;

    if (PTR_LOOP_1050_5f48 < &stack0x0004) {
        iVar1 = -(PTR_LOOP_1050_5f48 + -&stack0x0004);
    } else {
        iVar1 = 0;
    }
    return iVar1;
}

pub fn pass1_fn_1000_3cb7(param_1: *mut astruct_165) {
    let mut uVar1: i32;
    let puVar2: *mut u32;

    puVar2 = param_1.field_0xa;
    if (puVar2 == param_1.field_0xc) {
        puVar2 = param_1.field_0x8;
    }
    while (true) {
        unsafe { uVar1 = *puVar2 };
        if (uVar1 == 0xfffe) {
            break;
        }
        puVar2 = (puVar2 + (uVar1 & 0xfffe) + 2);
    }
    return;
}

pub fn pass1_1000_3d7a(param_1: u32, param_2: u32) -> i32 {
    let pbVar1: *mut byte;
    let paVar2: *mut astruct_168;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let paVar5: *mut astruct_168;
    let pbVar6: *mut byte;
    let local_DI_11: *mut astruct_168;
    let paVar7: *mut astruct_168;
    let mut uVar8: u16;
    let mut bVar9: bool;
    let mut bVar10: bool;

    pbVar6 = param_1;
    uVar8 = (param_2 >> 0x10);
    local_DI_11 = param_2;
    iVar3 = 0;
    uVar4 = 0xffff;
    while {
        if (uVar4 == 0) {
            break;
        }
        uVar4 = uVar4 - 1;
        paVar2 = local_DI_11;
        local_DI_11 = &local_DI_11.field_0x1;
        paVar2.field_0x0 != '\0'
    } {}
    // paVar5 = ~uVar4;
    bVar9 = local_DI_11 < paVar5;
    paVar7 = (local_DI_11 - paVar5);
    bVar10 = paVar7 == 0x0;
    while {
        if (paVar5 == 0x0) {
            break;
        }
        paVar5 = &paVar5[-1].field_0x1;
        paVar2 = paVar7;
        paVar7 = &paVar7.field_0x1;
        pbVar1 = pbVar6;
        pbVar6 = pbVar6 + 1;
        unsafe { bVar9 = *pbVar1 < paVar2.field_0x0 };
        unsafe { bVar10 = *pbVar1 == paVar2.field_0x0 };
        bVar10
    } {}
    if (!bVar10) {
        iVar3 = (1 - bVar9) - (bVar9 != 0);
    }
    return iVar3;
}

pub fn pass1_fn_1000_3e2c(long_byte_ptr: *mut u328_t) -> i32 {
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut iVar4: i32;
    let pbVar5: *mut byte;
    let mut uVar6: u16;

    uVar6 = (long_byte_ptr >> 0x10);
    pbVar5 = long_byte_ptr;
    iVar4 = 0;
    while {
        while {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 1;
            unsafe { bVar2 = *pbVar1 };
            bVar2 == 0x20
        } {}
        bVar2 == 9
    } {}
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {}
    // goto LAB_1000_3e4d;
    while (true) {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 1;
        unsafe { bVar3 = *pbVar1 };
        // LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 10 + (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

// pub fn pass1_fn_1000_3e2c(long_byte_ptr: *mut u328_t) -> i32 {
//     let pbVar1: *mut byte;
//     let mut bVar2: u8;
//     let mut bVar3: u8;
//     let mut iVar4: i32;
//     let pbVar5: *mut byte;
//     let mut uVar6: u16;

//     uVar6 = (long_byte_ptr >> 0x10);
//     pbVar5 = long_byte_ptr;
//     iVar4 = 0;
//     while {
//         while {
//             pbVar1 = pbVar5;
//             pbVar5 = pbVar5 + 1;
//             unsafe { bVar2 = *pbVar1 };
//             bVar2 == 0x20
//         } {}
//         bVar2 == 9
//     } {}
//     if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {}
//     // goto LAB_1000_3e4d;
//     loop {
//         pbVar1 = pbVar5;
//         pbVar5 = pbVar5 + 1;
//         unsafe { bVar3 = *pbVar1 };
//         // LAB_1000_3e4d:
//         if ((0x39 < bVar3) || (bVar3 < 0x30)) {
//             break;
//         }
//         iVar4 = iVar4 * 10 + (bVar3 - 0x30);
//     }
//     if (bVar2 == 0x2d) {
//         iVar4 = -iVar4;
//     }
//     return iVar4;
// }

pub fn pass1_fn_1000_3e2c(long_byte_ptr: *mut u328_t) -> i32 {
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut iVar4: i32;
    let pbVar5: *mut byte;
    let mut uVar6: u16;

    uVar6 = (long_byte_ptr >> 0x10);
    pbVar5 = long_byte_ptr;
    iVar4 = 0;
    while {
        while {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 1;
            unsafe { bVar2 = *pbVar1 };
            bVar2 == 0x20
        } {}
        bVar2 == 9
    } {}
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {}
    // goto LAB_1000_3e4d;
    loop {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 1;
        unsafe { bVar3 = *pbVar1 };
        // LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 10 + (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

pub fn pass1_fn_1000_3e82(uparam_1: i32, param_2: *mut u8, uparam_3: i32) -> *mut astruct_169 {
    let paVar1: *mut astruct_169;
    let mut uVar2: u32;
    let mut bVar3: u8;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let local_SI_2839: *mut astruct_169;
    let paVar8: *mut astruct_169;
    let paVar9: *mut astruct_169;
    let paVar10: *mut astruct_169;
    let mut uVar11: u16;
    let mut bVar12: bool;
    let mut cVar4: u8;

    uVar6 = 0;
    if (param_3 == 10) {
        uVar6 = param_1 >> 0xf;
    }
    uVar11 = (param_2 >> 0x10);
    paVar8 = param_2;
    paVar9 = paVar8;
    local_SI_2839 = paVar8;
    if ((param_3 == 10) && (uVar6 < 0)) {
        paVar9 = &paVar8.field_0x1;
        unsafe { *param_2 = 0x2d };
        bVar12 = param_1 != 0;
        param_1 = -param_1;
        uVar6 = -(uVar6 + bVar12);
        local_SI_2839 = paVar9;
    }
    while {
        uVar7 = 0;
        uVar5 = uVar6;
        if (uVar6 != 0) {
            uVar5 = uVar6 / param_3;
            uVar7 = uVar6 % param_3;
        }
        uVar2 = CONCAT22(uVar7, param_1);
        param_1 = (uVar2 / param_3);
        cVar4 = (uVar2 % param_3);
        bVar3 = cVar4 + 0x30;
        if (0x39 < bVar3) {
            bVar3 = cVar4 + 0x57;
        }
        paVar10 = &paVar9.field_0x1;
        (paVar9).field_0x0 = bVar3;
        uVar6 = uVar5;
        paVar9 = paVar10;
        (uVar5 | param_1) != 0
    } {}
    (paVar10).field_0x0 = 0;
    while {
        paVar10 = &paVar10[-1].field_0x3;
        paVar1 = paVar10;
        bVar3 = paVar1.field_0x0;
        paVar1.field_0x0 = (local_SI_2839).field_0x0;
        (local_SI_2839).field_0x0 = bVar3;
        paVar9 = &local_SI_2839.field_0x2;
        local_SI_2839 = &local_SI_2839.field_0x1;
        paVar9 < paVar10
    } {}
    return paVar8;
}

pub fn pass1_fn_1000_3f5c() -> i32 {
    let mut iVar1: i32;
    let paVar2: *mut astruct_150;
    let mut iVar3: i32;

    iVar3 = 0;
    if (PTR_LOOP_1050_61ec == 0x0) {
        paVar2 = &PTR_LOOP_1050_6210;
    } else {
        paVar2 = 0x6234;
    }
    while (paVar2 <= PTR_LOOP_1050_5ff0) {
        iVar1 = process_string_1000_2a00(paVar2);
        if (iVar1 != -1) {
            iVar3 = iVar3 + 1;
        }
        paVar2 = &paVar2.field_0xc;
    }
    return iVar3;
}

pub fn pass1_fn_1000_400a(param_1: i32) -> u16 {
    let paVar1: *mut astruct_149;
    let mut astruct_149_1: u16;

    if ((param_1 < 0) || (PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
        PTR_LOOP_1050_5f78 = &BYTE_1050_0009;
        astruct_149_1 = 0xffff;
    } else {
        if (((PTR_LOOP_1050_61ec == 0x0) || (param_1 < u16_1050_5f8a && (2 < param_1)))
            && (0x31d < CONCAT11(u8_1050_5f83, u8_1050_5f82)))
        {
            paVar1 = PTR_LOOP_1050_5f88;
            if (((*(param_1 + 0x5f90) & 1) == 0)
                || (
                    astruct_149_1 = dos3_call_1000_5174(param_1),
                    paVar1 = astruct_149_1,
                    astruct_149_1 != 0x0,
                ))
            {
                PTR_LOOP_1050_5f88 = paVar1;
                PTR_LOOP_1050_5f78 = &BYTE_1050_0009;
                astruct_149_1 = 0xffff;
            }
        } else {
            astruct_149_1 = 0;
        }
    }
    return astruct_149_1;
}

pub fn pass1_fn_1000_41e0(param_1: i32) -> u16 {
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = CONCAT22(PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    while (true) {
        if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= local_6) {
            return 0;
        }
        if (*_local_6 == param_1) {
            break;
        }
        _local_6 = (_local_6 & 0xffff0000 | (local_6 + 4));
    }
    unsafe { *_local_6 = 0 };
    return (local_6 + 2);
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_fn_1000_422a(param_1: i32, param_2: u16) -> i32 {
    let puVar1: *mut u8;
    let puVar2: *mut u8;
    let puVar3: *mut u8;
    let puVar4: *mut u8;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = CONCAT22(PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    while (true) {
        if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= local_6) {
            puVar2 = PTR_LOOP_1050_6194 + 0x28;
            puVar4 = PTR_LOOP_1050_6192;
            puVar3 = alloc_mem_1000_16aa();
            if ((puVar4 | puVar3) == 0) {
                param_1 = 0;
            } else {
                puVar1 = puVar3 + (PTR_LOOP_1050_6194 & 0xfffc);
                _local_6 = CONCAT22(puVar4, puVar1);
                PTR_LOOP_1050_6190 = puVar3;
                PTR_LOOP_1050_6192 = puVar4;
                unsafe { *_local_6 = param_1 };
                (puVar1 + 2) = param_2;
                PTR_LOOP_1050_6194 = puVar2;
                pass1_1000_4906(CONCAT22(puVar4, puVar1 + 4), 0, 0x24);
            }
            return param_1;
        }
        if (*_local_6 == 0) {
            break;
        }
        _local_6 = (_local_6 & 0xffff0000 | (local_6 + 4));
    }
    (local_6 + 2) = param_2;
    unsafe { *_local_6 = param_1 };
    return param_1;
}

pub fn pass1_fn_1000_43f0() {
    let mut in_DX: u16;

    if (PTR_LOOP_1050_68b4 == 0x0) {
        process_string_1000_440c(in_DX);
        PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 1;
    }
    return;
}

pub fn pass1_fn_1000_455a(param_1: *mut astruct_170) -> u16 {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut uVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let local_BX_9: *mut astruct_170;
    let mut uVar6: u16;
    let mut local_6: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_9 = param_1;
    if (((local_BX_9.field_0xa < 0x43) || (local_BX_9.field_0x8 < 3)) || (9 < local_BX_9.field_0x8))
    {
    }
    // goto LAB_1000_4623;
    if ((local_BX_9.field_0x8 < 4) || (8 < local_BX_9.field_0x8)) {
        uVar3 = local_BX_9.field_0xa;
        if ((uVar3 < 0x57) || (local_BX_9.field_0x8 != 3)) {
            local_6 = (local_BX_9.field_0x8 * 2 + 0x61b2);
        } else {
            local_6 = (local_BX_9.field_0x8 * 2 + 0x61b0) + 7;
        }
        if ((uVar3 & 3) == 0) {
            local_6 = local_6 + 1;
        }
        uVar3 = (uVar3 - 0x46) * 0x16d + ((uVar3 - 1) >> 2) + local_6;
        iVar4 = pass1_fn_1000_52f0(uVar3 - 0xd, (uVar3 >> 0xf) - (uVar3 < 0xd), 7, 0);
        iVar4 = iVar4 - local_6;
        iVar5 = -iVar4;
        if (local_BX_9.field_0x8 == 3) {
            iVar2 = local_BX_9.field_0xe;
            if ((iVar5 < iVar2) || (-iVar2 == iVar4 && (1 < local_BX_9.field_0x4))) {}
            // goto LAB_1000_460e;
        } else {
            piVar1 = &local_BX_9.field_0xe;
            unsafe { iVar2 = *piVar1 };
            if ((SBORROW2(iVar2, iVar5) != iVar2 + (iVar4 < 0))
                || (iVar2 == iVar5 && (local_BX_9.field_0x4 < 1)))
            {}
            // goto LAB_1000_460e;
        }
        // LAB_1000_4623:
        uVar6 = 0;
    } else {
        // LAB_1000_460e:
        uVar6 = 1;
    }
    return uVar6;
}

pub fn pass1_fn_1000_462e(
    uparam_1: i32,
    param_2: i32,
    uparam_3: i32,
    uparam_4: i32,
    uparam_5: i32,
    param_6: i32,
) {
    uVar1;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let extraout_AH: u8;
    let extraout_AH_00: u8;
    let extraout_AH_01: u8;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut unaff_BP: i32;
    let mut uVar9: i32;
    let mut unaff_SS: u16;
    let mut in_u16_3: u16;
    let mut in_u16_4: u16;
    let mut in_u16_3_00: u16;
    let mut in_u16_4_00: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_BP + 1;
    local_4 = SUB42(&g_alloc_addr_1050_1050, 0);
    uVar9 = (param_2 * 2 + 0x61ae);
    if (((param_1 & 3) == 0) && (2 < param_2)) {
        uVar9 = uVar9 + 1;
    }
    pass1_fn_1000_43f0();
    in_u16_4_00 = 0;
    in_u16_3_00 = 0x3c;
    in_u16_4 = 0;
    in_u16_3 = 0x3c;
    uVar2 = (param_1 * 0x16d);
    uVar3 = (param_1 + 3) >> 2;
    uVar4 = uVar3 + param_3;
    uVar5 = uVar2 + uVar4;
    iVar7 = uVar9 >> 0xf;
    uVar6 = uVar5 + uVar9;
    uVar1 = pass1_fn_1000_52be(
        uVar6 + 0xe44,
        ((param_1 * 0x16d) >> 0x10)
            + ((param_1 + 3) >> 0xf)
            + (param_3 >> 0xf)
            + CARRY2(uVar3, param_3)
            + CARRY2(uVar2, uVar4)
            + iVar7
            + CARRY2(uVar5, uVar9)
            + (0xf1bb < uVar6),
        0x18,
        0,
    );
    iVar8 = param_4 >> 0xf;
    uVar1 = pass1_fn_1000_52be(
        CONCAT11(extraout_AH, uVar1) + param_4,
        iVar7 + iVar8 + CARRY2(CONCAT11(extraout_AH, uVar1), param_4),
        in_u16_3,
        in_u16_4,
    );
    uVar1 = pass1_fn_1000_52be(
        CONCAT11(extraout_AH_00, uVar1) + param_5,
        iVar8 + (param_5 >> 0xf) + CARRY2(CONCAT11(extraout_AH_00, uVar1), param_5),
        in_u16_3_00,
        in_u16_4_00,
    );
    local_1a = CONCAT11(extraout_AH_01, uVar1) + param_6 + u16_1050_61ce;
    local_8 = param_3 + uVar9;
    local_c = param_1 + 0x50;
    local_e = param_2 - 1;
    local_12 = param_4;
    if (u16_1050_61d2 != 0) {
        iVar7 = pass1_fn_1000_455a(CONCAT22(unaff_SS, local_16));
        if (iVar7 != 0) {
            local_1a = local_1a - 0xe10;
        }
    }
    return local_1a;
}

pub fn pass1_fn_1000_47a4(param_1: u32, param_2: *mut byte) -> uint {
    let pbVar1: *mut byte;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut byte_ptr_1: u16;
    let mut iVar4: i32;
    let mut byte_ptr_3: u16;
    let puVar5: *mut u16;
    let mut unaff_SS: u16;
    let mut uVar6: u16;
    let mut u16_array_1: [u8; 32];
    let mut byte_ptr_2: u32;
    let byte_1: u8;
    let temp_87f577f7f21: *mut u16;

    iVar4 = 0x10;
    puVar5 = u16_array_1;
    while (iVar4 != 0) {
        iVar4 = iVar4 + -1;
        temp_87f577f7f21 = puVar5;
        puVar5 = puVar5 + 1;
        unsafe { *temp_87f577f7f21 = 0 };
    }
    byte_ptr_3 = param_2;
    while (true) {
        byte_ptr_2 = byte_ptr_3;
        byte_ptr_3 = byte_ptr_3 + 1;
        unsafe { byte_1 = *byte_ptr_2 };
        if (byte_1 == '\0') {
            break;
        }
        pbVar1 = u16_array_1 + (byte_1 >> 3);
        unsafe { *pbVar1 = *pbVar1 | 0x1 << (byte_1 & 7) };
    }
    pbVar1 = param_1;
    if (param_1 == 0) {
        pbVar1 = _UINT_1050_61e4;
    }
    while {
        _UINT_1050_61e4 = pbVar1;
        uVar6 = (_UINT_1050_61e4 >> 0x10);
        pbVar2 = (_UINT_1050_61e4 + 1);
        unsafe { bVar3 = *_UINT_1050_61e4 };
        if (bVar3 == 0) {
            return 0;
        }
        pbVar1 = (_UINT_1050_61e4 & 0xffff0000 | ZEXT24(pbVar2));
        (0x1 << (bVar3 & 7) & u16_array_1[bVar3 >> 3]) != 0
    } {}
    while {
        byte_ptr_1 = pbVar2;
        unsafe { bVar3 = *byte_ptr_1 };
        if (bVar3 == 0) {}
        // goto LAB_1000_483c;
        pbVar2 = (byte_ptr_1 + 1);
        (0x1 << (bVar3 & 7) & u16_array_1[bVar3 >> 3]) == 0
    } {}
    unsafe { *byte_ptr_1 = 0 };
    byte_ptr_1 = (byte_ptr_1 + 1);
    // LAB_1000_483c:
    _UINT_1050_61e4 = (_UINT_1050_61e4 & 0xffff0000 | byte_ptr_1);
    return UINT_1050_61e4;
}

pub fn pass1_fn_1000_484c(param_1: *mut byte, param_2: *mut byte, param_3: u16) -> uint {
    let pbVar1: *mut byte;
    let pbVar2: *mut byte;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let pbVar6: *mut byte;
    let pbVar7: *mut byte;
    let local_ES_16: *mut astruct_172;
    let mut iVar8: i32;
    let local_DS_13: *mut astruct_173;
    let mut bVar9: bool;
    let mut bVar10: bool;

    if (param_3 == 0) {
        return 0;
    }
    loop {
        iVar8 = (param_2 >> 0x10);
        pbVar7 = param_2;
        iVar3 = (param_1 >> 0x10);
        pbVar6 = param_1;
        // uVar4 = ~pbVar7;
        uVar4 = ((param_3 - 1) - uVar4 & -(param_3 - 1 < uVar4)) + uVar4;
        // uVar5 = ~pbVar6;
        uVar4 = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 1;
        bVar9 = param_3 < uVar4;
        param_3 = param_3 - uVar4;
        bVar10 = param_3 == 0;
        while {
            if (uVar4 == 0) {
                break;
            }
            uVar4 = uVar4 - 1;
            pbVar2 = pbVar7;
            pbVar7 = pbVar7 + 1;
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 1;
            unsafe { bVar9 = *pbVar1 < *pbVar2 };
            unsafe { bVar10 = *pbVar1 == *pbVar2 };
            bVar10
        } {}
        param_2 = (param_2 & 0xffff0000 | ZEXT24(pbVar7));
        if (!bVar10) {
            return (1 - bVar9) - (bVar9 != 0);
        }
        if (param_3 == 0) {
            return uVar4;
        }
        if (pbVar6 == 0x0) {
            iVar3 = iVar3 + 0x6c;
        }
        param_1 = CONCAT22(iVar3, pbVar6);
        if (pbVar7 == 0x0) {
            param_2 = ((iVar8 + 0x6c) << 0x10);
            param_1 = CONCAT22(iVar3, pbVar6);
        }
    }
}

pub fn pass1_fn_1000_48a8(param_1: *mut astruct_174, param_2: u32, param_3: u16) -> u16 {
    let puVar1: *mut u16;
    let p_uvar2: *mut u16;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let puVar6: *mut u16;
    let puVar7: *mut u16;
    let local_ES_14: *mut astruct_174;
    let mut iVar8: i32;
    let local_DS_11: *mut astruct_175;

    if (param_3 != 0) {
        while (true) {
            iVar3 = (param_2 >> 0x10);
            puVar6 = param_2;
            iVar8 = (param_1 >> 0x10);
            puVar7 = param_1;
            // uVar4 = ~puVar7;
            uVar4 = ((param_3 - 1) - uVar4 & -(param_3 - 1 < uVar4)) + uVar4;
            // uVar5 = ~puVar6;
            uVar4 = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 1;
            param_3 = param_3 - uVar4;
            uVar5 = uVar4 >> 1;
            while (uVar5 != 0) {
                uVar5 = uVar5 - 1;
                puVar2 = puVar7;
                puVar7 = puVar7 + 1;
                puVar1 = puVar6;
                puVar6 = puVar6 + 1;
                unsafe { *puVar2 = *puVar1 };
            }
            uVar4 = ((uVar4 & 1) != 0);
            while (uVar4 != 0) {
                uVar4 = uVar4 - 1;
                puVar2 = puVar7;
                puVar7 = (puVar7 + 1);
                puVar1 = puVar6;
                puVar6 = (puVar6 + 1);
                unsafe { *puVar2 = *puVar1 };
            }
            if (param_3 == 0) {
                break;
            }
            if (puVar6 == 0x0) {
                iVar3 = iVar3 + 0x6c;
            }
            param_1 = (param_1 & 0xffff0000 | ZEXT24(puVar7));
            param_2 = CONCAT22(iVar3, puVar6);
            if (puVar7 == 0x0) {
                param_1 = ((iVar8 + 0x6c) << 0x10);
                param_2 = CONCAT22(iVar3, puVar6);
            }
        }
    }
    return param_1._0_2_;
}

pub fn pass1_1000_4906(param_1: *mut astruct_65, param_2: u16, param_3: u16) {
    let puVar1: *mut u32;
    let uVar2: u8;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut uVar6: u16;
    let mut iVar7: i32;

    if (param_3 != 0) {
        iVar7 = (param_1 >> 0x10);
        uVar5 = -param_1;
        uVar6 = param_3;
        if (uVar5 != 0) {
            uVar6 = (uVar5 - param_3 & -(uVar5 < param_3)) + param_3;
            uVar5 = param_3 - uVar6;
        }
        uVar3 = param_2 & 0xff | param_2 << 8;
        uVar4 = uVar6 >> 1;
        while (uVar4 != 0) {
            uVar4 = uVar4 - 1;
            puVar1 = param_1;
            param_1._0_2_ = param_1 + 1;
            unsafe { *puVar1 = uVar3 };
        }
        uVar4 = ((uVar6 & 1) != 0);
        while (uVar2 = (param_2 & 0xff), uVar4 != 0) {
            uVar4 = uVar4 - 1;
            puVar1 = param_1;
            param_1._0_2_ = (param_1 + 1);
            unsafe { *puVar1 = uVar2 };
        }
        if (uVar5 != 0) {
            uVar4 = uVar5 >> 1;
            while (uVar4 != 0) {
                uVar4 = uVar4 - 1;
                puVar1 = param_1;
                param_1._0_2_ = param_1 + 1;
                unsafe { *puVar1 = uVar3 };
            }
            uVar5 = ((uVar5 & 1) != 0);
            while (uVar5 != 0) {
                uVar5 = uVar5 - 1;
                puVar1 = param_1;
                param_1._0_2_ = (param_1 + 1);
                unsafe { *puVar1 = uVar2 };
            }
        }
    }
    return param_1;
}

pub fn pass1_1000_49c6(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    fn_ptr_7: u16,
) -> i32 {
    uVar1;
    let extraout_AH: u8;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut iVar5: i32;
    let mut unaff_BP: i32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_BP + 1;
    local_4 = SUB42(&g_alloc_addr_1050_1050, 0);
    local_14 = param_3;
    local_12 = param_4;
    uVar4 = param_4;
    uVar1 = pass1_fn_1000_52be(param_5 - 1, -(param_5 == 0), param_6, 0);
    local_8 = CONCAT11(extraout_AH, uVar1) + param_3;
    local_6 = (uVar4 + CARRY2(CONCAT11(extraout_AH, uVar1), param_3)) * 0x100 + param_4;
    while (true) {
        if (local_6 < local_12) {
            return 0;
        }
        if ((local_6 <= local_12) && (local_8 < local_14)) {
            return 0;
        }
        local_e = param_5 >> 1;
        if (local_e == 0) {
            if ((param_5 != 0) && (iVar5 = (*fn_ptr_7)(), iVar5 == 0)) {
                return local_14;
            }
            return 0;
        }
        uVar2 = local_e;
        if ((param_5 & 1) == 0) {
            uVar2 = local_e - 1;
        }
        uVar3 = (uVar2 * param_6);
        uVar4 = uVar3 + local_14;
        local_a = ((uVar2 * param_6 >> 0x10) + CARRY2(uVar3, local_14)) * 0x100 + local_12;
        local_c = uVar4;
        iVar5 = (*fn_ptr_7)();
        if (iVar5 == 0) {
            break;
        }
        if (iVar5 < 0) {
            local_8 = -param_6 + local_c;
            local_6 = (CARRY2(-param_6, local_c) - (param_6 != 0)) * 0x100 + local_a;
            uVar2 = param_5 & 1;
            param_5 = local_e;
            if (uVar2 == 0) {
                param_5 = local_e - 1;
            }
        } else {
            local_14 = param_6 + local_c;
            local_12 = CARRY2(param_6, local_c) * 0x100 + local_a;
            param_5 = local_e;
        }
    }
    return uVar4;
}

pub fn pass1_1000_4aea(
    uparam_1: i32,
    uparam_2: i32,
    param_3: i32,
    uparam_4: i32,
    in_fn_ptr_1: *mut code,
) -> i32 {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let lVar3: u32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let puVar8: *mut u8;
    let mut unaff_BP: i32;
    let mut uVar9: i32;
    let mut uVar10: i32;
    let mut uVar11: i32;
    let mut uVar12: i32;
    let mut unaff_SS: u16;
    let mut uVar13: u16;
    let mut bVar14: bool;
    let mut uStack44: u16;
    let mut uStack40: i32;
    let mut uStack38: i32;
    let mut uStack36: i32;
    let mut uStack34: i32;
    let mut uStack32: i32;
    let mut uStack30: i32;
    let mut uStack28: i32;
    let mut uStack26: i32;
    let mut uStack24: i32;
    let mut uStack22: i32;
    let mut uVar15: i32;
    let mut uVar16: i32;
    let mut uStack18: i32;
    let mut uStack16: i32;
    let mut uStack14: i32;
    let mut uStack12: i32;
    let mut uStack4: u16;
    let mut iStack2: i32;
    let mut fn_ptr_1: u32;

    iStack2 = unaff_BP + 1;
    uStack4 = SUB42(&g_alloc_addr_1050_1050, 0);
    uStack28 = &g_alloc_addr_1050_1050;
    if ((param_4 != 0) && (param_3 != 0)) {
        iVar7 = param_3 + -1;
        uStack12 = param_2;
        uStack14 = param_1;
        while (iVar7 != 0) {
            uVar9 = uStack14 + param_4;
            uVar12 = uStack12 + (-CARRY2(uStack14, param_4) & 0x6c);
            uStack22 = 0x4b1c;
            uStack18 = uVar9;
            uStack16 = uVar12;
            unsafe {
                iVar6 = (*in_fn_ptr_1)();
            }
            if (iVar6 < 0) {
                uVar9 = param_3 - 1;
                iVar7 = 0;
                while {
                    uVar9 = uVar9 >> 1;
                    iVar7 = iVar7 + -1;
                    iVar7 != 0 && uVar9 != 0
                } {}
                if ((-iVar7 * 8 >> 0x10) == 0) {
                    uStack12 = 0x4b4b;
                    uVar9 = pass1_fn_1000_3bac();
                    if ((-iVar7 * 8) <= uVar9) {
                        puVar8 = &stack0xfff6;
                        lVar3 = (param_3 - 1) * param_4;
                        uVar9 = lVar3;
                        uStack14 = uVar9 + param_1;
                        uStack12 = ((lVar3 >> 0x10) + CARRY2(uVar9, param_1)) * 0x100 + param_2;
                        uStack16 = param_2;
                        uStack18 = param_1;
                        // LAB_1000_4b7d:
                        if (puVar8 <= &uStack18) {
                            return;
                        }
                        // LAB_1000_4b81:
                        if ((uStack16 < uStack12)
                            || (uStack16 <= uStack12 && (uStack18 < uStack14)))
                        {
                            uStack22 = uStack14;
                            puVar1 = (puVar8 + 0x14);
                            uVar12 = uStack16;
                            uVar9 = uStack18;
                            let pu_var1_val = unsafe { *puVar1 };
                            uStack30 = uStack14 + pu_var1_val;
                            uVar4 = uStack12 + (-CARRY2(uStack14, pu_var1_val) & 0x6c);
                            uStack26 = uStack18;
                            uStack24 = uStack16;
                            uVar15 = uStack12;
                            uVar13 = uStack28;
                            // LAB_1000_4bbc:
                            loop {
                                uStack28 = uVar4;
                                puVar1 = (puVar8 + 0x14);
                                let pu_var1_val = unsafe { *puVar1 };
                                uVar10 = uVar9 + pu_var1_val;
                                uStack38 = uVar12 + (-CARRY2(uVar9, pu_var1_val) & 0x6c);
                                if ((uVar10 != uStack14)
                                    || (
                                        uVar12 = uStack28,
                                        uVar11 = uStack30,
                                        uVar4 = uStack38,
                                        uVar5 = uStack22,
                                        uStack38 != uStack12,
                                    ))
                                {
                                    uStack34 = uStack16;
                                    uStack36 = uStack18;
                                    uStack44 = SUB42(&PTR_LOOP_1050_4bde, 0);
                                    ppcVar2 = (puVar8 + 0x16);
                                    uStack40 = uVar10;
                                    uStack32 = uStack38;
                                    iVar7 = ppcVar2();
                                    uVar12 = uStack28;
                                    uVar11 = uStack30;
                                    uVar4 = uStack32;
                                    uVar5 = uStack22;
                                    if (iVar7 < 1) {
                                        uVar12 = uStack32;
                                        uVar9 = uVar10;
                                        uVar4 = uStack28;
                                        if (iVar7 != 0) {
                                            uStack26 = uVar10;
                                            uStack24 = uStack32;
                                        }
                                        // goto LAB_1000_4bbc;
                                    }
                                }
                                while {
                                    uVar16 = uVar15;
                                    uStack22 = uVar5;
                                    uStack28 = uVar4;
                                    uStack30 = uVar10;
                                    puVar1 = (puVar8 + 0x14);
                                    let pu_var1_val = unsafe { *puVar1 };
                                    bVar14 = uVar11 < pu_var1_val;
                                    uVar11 = uVar11 - pu_var1_val;
                                    uStack34 = uVar12 - (-bVar14 & 0x6c);
                                    uStack38 = uStack16;
                                    uStack40 = uStack18;
                                    uStack44 = SUB42(&PTR_LOOP_1050_4c0e, 0);
                                    ppcVar2 = (puVar8 + 0x16);
                                    uStack36 = uVar11;
                                    uStack32 = uStack34;
                                    iVar7 = ppcVar2();
                                    uVar9 = uStack30;
                                    if (0 < iVar7) {
                                        break;
                                    }
                                    uVar12 = uStack32;
                                    uVar10 = uStack30;
                                    uVar4 = uStack28;
                                    uVar5 = uVar11;
                                    uVar15 = uStack32;
                                    ((iVar7 != 0)
                                        || (uVar5 = uStack22, uVar15 = uVar16, uVar11 != uStack18))
                                        || (uStack32 != uStack16)
                                } {}
                                if ((uStack32 < uStack28)
                                    || (uStack32 <= uStack28 && (uVar11 <= uStack30)))
                                {
                                }
                                // goto LAB_1000_4c58;
                                uStack30 = &PTR_LOOP_1050_4c46;
                                uVar12 = uStack28;
                                uStack28 = uVar13;
                                pass1_fn_1000_4ceb(*(puVar8 + 0x14));
                                uStack30 = uVar11;
                                uVar4 = uStack32;
                                uStack26 = uVar9;
                                uStack24 = uVar12;
                                uStack22 = uVar11;
                                uVar15 = uStack32;
                                uVar13 = uStack28;
                            }
                        }
                        // goto LAB_1000_4b7d;
                    }
                }
                uStack12 = 0x4b7b;
                bad_1000_25f2();
                return;
            }
            iVar7 = iVar7 + -1;
            uStack12 = uVar12;
            uStack14 = uVar9;
        }
    }
    return;
    // LAB_1000_4c58:
    uStack32 = &PTR_LOOP_1050_4c68;
    uStack28 = uVar13;
    pass1_fn_1000_4ceb(*(puVar8 + 0x14));
    uVar12 = ((uStack12 - (-(uStack14 < uStack22) & 0x6c)) - uVar16)
        + (-CARRY2(uStack14 - uStack22, uStack18) & 0x6c)
        + uStack16;
    uVar9 = -((uStack14 - uStack22) + uStack18 < uStack26) & 0x6c;
    if ((uVar12 < uVar9) || (uVar12 - uVar9 < uStack24)) {
        uStack12 = uStack24;
        uStack14 = uStack26;
    } else {
        uStack18 = uStack22;
        uStack16 = uVar16;
    }
    // goto LAB_1000_4b81;
}

pub fn pass1_fn_1000_4ceb(param_1: u16) {
    uVar1;
    let mut uVar2: u16;
    let mut unaff_SI: i32;
    let mut unaff_DI: i32;
    let mut local_ES__1: u16;
    let temp_87f72e626cf: *mut u328_t;
    let temp_87f9aad3c2a: *mut u16;

    if ((param_1 & 1) != 0) {
        param_1 = param_1 - 1;
        temp_87f72e626cf = (param_1 + unaff_DI);
        unsafe { uVar1 = *temp_87f72e626cf };
        unsafe { *temp_87f72e626cf = *(param_1 + unaff_SI) };
        *(param_1 + unaff_SI) = uVar1;
        if (param_1 == 0) {
            return;
        }
    }
    while {
        param_1 = param_1 - 2;
        temp_87f9aad3c2a = (param_1 + unaff_DI);
        unsafe { uVar2 = *temp_87f9aad3c2a };
        unsafe { *temp_87f9aad3c2a = *(param_1 + unaff_SI) };
        unsafe { *(param_1 + unaff_SI) = uVar2 };
        param_1 != 0
    } {}
    return;
}

pub fn pass1_fn_1000_4d24() -> uint {
    let local_AL_23: u8;
    let local_AH_23: u8;
    let mut iVar1: i32;

    iVar1 = 3;
    local_AL_23 = pass1_fn_1000_52be(
        UINT_1050_61e8,
        PTR_LOOP_1050_61ea,
        (s_TPPOPMENU_1050_43fa + 3),
        3,
    );
    PTR_LOOP_1050_61ea = (iVar1 + 0x26 + (0x613c < CONCAT11(local_AH_23, local_AL_23)));
    UINT_1050_61e8 = CONCAT11(local_AH_23, local_AL_23) + 0x9ec3;
    return PTR_LOOP_1050_61ea & 0x7fff;
}

pub fn pass1_fn_1000_5008(param_1: u16, param_2: u16, param_3: u16) {
    let mut unaff_BP: i32;

    pass1_fn_1000_5026(
        0,
        param_1,
        param_2,
        param_3,
        &g_alloc_addr_1050_1050,
        unaff_BP + 1,
    );
    return;
}

pub fn pass1_fn_1000_5026(param_1: i32, in_mem_buf_ptr: u32, uparam_3: i32, param_4: u16) {
    let mut uVar1: i32;
    let mut uVar2: u16;
    let mut local_AX_282: u16;
    let mut unaff_BP: i32;
    let mut local_SS__1: u16;
    let mut local_132: u16;
    let mut mem_buf_130: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_118: u16;
    local_116;
    let local_115: u8;
    let local_110: u8;
    let mut local_10e: u16;
    local_108;
    let uStack263: u8;
    let uStack262: u8;
    let mut auStack261: [u8; 257];
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_BP + 1;
    local_4 = SUB42(&g_alloc_addr_1050_1050, 0);
    _mem_buf_130 = CONCAT22(local_SS__1, &local_108);
    if (param_1 == 0) {
        param_1 = dos3_call_1000_4f94();
    }
    unsafe { *_mem_buf_130 = param_1 + '@' };
    uStack263 = 0x3a;
    local_10e = auStack261;
    uStack262 = 0x5c;
    local_115 = 'G';
    local_110 = param_1;
    dos3_call_1000_42de(&local_116);
    if (local_118 == 0) {
        uVar1 = get_string_index_1000_3da4(CONCAT22(local_SS__1, &local_108));
        uVar2 = uVar1 + 1;
        mem_buf_130 = in_mem_buf_ptr;
        local_12e = param_3;
        param_3 = param_3 | in_mem_buf_ptr;
        if (param_3 == 0) {
            if (param_4 < uVar2) {
                param_4 = uVar2;
            }
            mem_buf_130 = alloc_mem_1000_167a(param_4, 0);
            local_12e = param_3;
            if ((param_3 | mem_buf_130) == 0) {
                PTR_LOOP_1050_5f78 = &PTR_LOOP_1050_000c;
                return;
            }
        }
        if (param_4 < uVar2) {
            PTR_LOOP_1050_5f78 = (s_New_failed_in_Op__Op_1050_0020 + 2);
        } else {
            copy_string_1000_3d3e(
                CONCAT22(local_12e, mem_buf_130),
                CONCAT22(local_SS__1, &local_108),
            );
        }
    } else {
        PTR_LOOP_1050_5f78 = (&PTR_LOOP_1050_000c + 1);
        PTR_LOOP_1050_5f88 = local_124;
    }
    return;
}

pub fn pass1_fn_1000_52be(in_i16_1: u16, in_i16_2: u16, in_u16_3: u16, in_u16_4: u16) {
    if ((in_u16_4 | in_i16_2) == 0) {
        return (in_i16_1 * in_u16_3);
    }
    return (in_i16_1 * in_u16_3);
}

pub fn pass1_fn_1000_52f0(uparam_1: i32, uparam_2: i32, uparam_3: i32, uparam_4: i32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let mut uVar9: i32;
    let mut uVar10: i32;
    let mut uVar11: i32;
    let mut bVar12: bool;
    let mut bVar13: bool;

    bVar13 = param_2 < 0;
    if (bVar13) {
        bVar12 = param_1 != 0;
        param_1 = -param_1;
        param_2 = -bVar12 - param_2;
    }
    uVar11 = bVar13;
    if (param_4 < 0) {
        bVar13 = param_3 != 0;
        param_3 = -param_3;
        param_4 = -bVar13 - param_4;
    }
    uVar3 = param_1;
    uVar4 = param_3;
    uVar8 = param_2;
    uVar9 = param_4;
    if (param_4 == 0) {
        iVar5 = ((param_2 % param_3 << 0x10 | param_1) % param_3);
        iVar6 = 0;
        if ((uVar11 - 1) < 0) {}
        // goto LAB_1000_538a;
    } else {
        while {
            uVar10 = uVar9 >> 1;
            uVar4 = uVar4 >> 1 | ((uVar9 & 1) != 0) << 0xf;
            uVar7 = uVar8 >> 1;
            uVar3 = uVar3 >> 1 | ((uVar8 & 1) != 0) << 0xf;
            uVar8 = uVar7;
            uVar9 = uVar10;
            uVar10 != 0
        } {}
        uVar1 = CONCAT22(uVar7, uVar3) / uVar4;
        uVar3 = uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) * param_3;
        uVar8 = (lVar2 >> 0x10);
        uVar4 = lVar2;
        uVar9 = uVar8 + uVar3;
        if (((CARRY2(uVar8, uVar3)) || (param_2 < uVar9))
            || (param_2 <= uVar9 && (param_1 < uVar4)))
        {
            bVar13 = uVar4 < param_3;
            uVar4 = uVar4 - param_3;
            uVar9 = (uVar9 - param_4) - bVar13;
        }
        iVar5 = uVar4 - param_1;
        iVar6 = (uVar9 - param_2) - (uVar4 < param_1);
        if (-1 < (uVar11 - 1)) {}
        // goto LAB_1000_538a;
    }
    bVar13 = iVar5 != 0;
    iVar5 = -iVar5;
    iVar6 = -bVar13 - iVar6;
    // LAB_1000_538a:
    return CONCAT22(iVar6, iVar5);
}

pub fn pass1_fn_1000_5390(uparam_1: i32, uparam_2: i32, uparam_3: i32, uparam_4: i32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut uVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let mut uVar9: i32;

    uVar3 = param_1;
    uVar8 = param_4;
    uVar6 = param_2;
    uVar9 = param_3;
    if (param_4 == 0) {
        uVar3 = param_2 / param_3;
        iVar4 = ((param_2 % param_3 << 0x10 | param_1) / param_3);
    } else {
        while {
            uVar5 = uVar8 >> 1;
            uVar9 = uVar9 >> 1 | ((uVar8 & 1) != 0) << 0xf;
            uVar7 = uVar6 >> 1;
            uVar3 = uVar3 >> 1 | ((uVar6 & 1) != 0) << 0xf;
            uVar8 = uVar5;
            uVar6 = uVar7;
            uVar5 != 0
        } {}
        uVar1 = CONCAT22(uVar7, uVar3) / uVar9;
        iVar4 = uVar1;
        lVar2 = param_3 * (uVar1 & 0xffff);
        uVar3 = (lVar2 >> 0x10);
        uVar8 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3, iVar4 * param_4)) || (param_2 < uVar8))
            || (param_2 <= uVar8 && (param_1 < lVar2)))
        {
            iVar4 = iVar4 + -1;
        }
        uVar3 = 0;
    }
    return CONCAT22(uVar3, iVar4);
}

pub fn pass1_fn_1000_53f0(uparam_1: i32, uparam_2: i32, uparam_3: i32, uparam_4: i32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let mut uVar8: i32;
    let mut uVar9: i32;
    let mut uVar10: i32;
    let mut bVar11: bool;

    uVar3 = param_1;
    uVar4 = param_4;
    uVar9 = param_2;
    uVar10 = param_3;
    if (param_4 == 0) {
        iVar6 = ((param_2 % param_3 << 0x10 | param_1) % param_3);
        iVar7 = 0;
    } else {
        while {
            uVar5 = uVar4 >> 1;
            uVar10 = uVar10 >> 1 | ((uVar4 & 1) != 0) << 0xf;
            uVar8 = uVar9 >> 1;
            uVar3 = uVar3 >> 1 | ((uVar9 & 1) != 0) << 0xf;
            uVar4 = uVar5;
            uVar9 = uVar8;
            uVar5 != 0
        } {}
        uVar1 = CONCAT22(uVar8, uVar3) / uVar10;
        uVar3 = uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) * param_3;
        uVar9 = (lVar2 >> 0x10);
        uVar4 = lVar2;
        uVar10 = uVar9 + uVar3;
        if (((CARRY2(uVar9, uVar3)) || (param_2 < uVar10))
            || (param_2 <= uVar10 && (param_1 < uVar4)))
        {
            bVar11 = uVar4 < param_3;
            uVar4 = uVar4 - param_3;
            uVar10 = (uVar10 - param_4) - bVar11;
        }
        iVar6 = -(uVar4 - param_1);
        iVar7 = -(uVar4 - param_1 != 0) - ((uVar10 - param_2) - (uVar4 < param_1));
    }
    return CONCAT22(iVar7, iVar6);
}

pub fn pass1_fn_1000_54a0(param_1: u32, param_2: u16, param_3: u16) -> u32 {
    let puVar1: *mut u32;
    local_AL_41;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let puVar6: *mut u32;
    let mut in_stack_00000006: i32;

    if (param_3 != 0) {
        uVar4 = -param_1;
        uVar5 = param_3;
        if (uVar4 != 0) {
            uVar5 = (uVar4 - param_3 & -(uVar4 < param_3)) + param_3;
            uVar4 = param_3 - uVar5;
        }
        uVar2 = param_2 & 0xff | param_2 << 8;
        uVar3 = uVar5 >> 1;
        puVar6 = param_1;
        while (uVar3 != 0) {
            uVar3 = uVar3 - 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            unsafe { *puVar1 = uVar2 };
        }
        uVar3 = ((uVar5 & 1) != 0);
        while (local_AL_41 = (param_2 & 0xff), uVar3 != 0) {
            uVar3 = uVar3 - 1;
            puVar1 = puVar6;
            puVar6 = (puVar6 + 1);
            unsafe { *puVar1 = local_AL_41 };
        }
        if (uVar4 != 0) {
            uVar3 = uVar4 >> 1;
            while (uVar3 != 0) {
                uVar3 = uVar3 - 1;
                puVar1 = puVar6;
                puVar6 = puVar6 + 1;
                unsafe { *puVar1 = uVar2 };
            }
            uVar4 = ((uVar4 & 1) != 0);
            while (uVar4 != 0) {
                uVar4 = uVar4 - 1;
                puVar1 = puVar6;
                puVar6 = (puVar6 + 1);
                unsafe { *puVar1 = local_AL_41 };
            }
        }
    }
    return param_1;
}

pub fn pass1_fn_1008_04d2(param_1: u32, param_2: u8) {
    handle_error_1008_9466(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_fn_1008_07d8(in_u16_1: u16, in_u16_ptr_1: *mut u16) {
    let mut bVar1: bool;
    let mut uVar2: u16;
    let mut local_4: u16;

    if (_g_bool_1050_5748 == 0x0) {
        process_struct_1000_179c(10, in_u16_ptr_1);
        uVar2 = in_u16_ptr_1 | in_u16_1;
        if (uVar2 != 0) {
            bVar1 = pass1_1030_8128(CONCAT22(in_u16_ptr_1, in_u16_1));
            in_u16_1 = bVar1;
        }
        if (_g_bool_1050_5748 == 0x0) {
            fn_1008_6048(
                s_New_failed_in_Op__Op__Simulator_1050_0110,
                uVar2,
                SUB21(in_u16_1, 0),
            );
            call_fn_ptr_1000_24cd(1);
        }
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        pass1_1030_838e(_g_bool_1050_5748);
        pass1_1030_8334();
    }
    return;
}

pub fn pass1_1008_0a92(param_1: u32) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut local_ES_4: i32;
    void * *fn_ptr_1;
    void * *temp_87f6e103bce;

    local_ES_4 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xee) != 0) {
        fn_ptr_1 = ((iVar2 + 0xee) + 0x90);
        (**fn_ptr_1)();
    }
    if ((iVar2 + 0xe8) != 0) {
        temp_87f6e103bce = ((iVar2 + 0xe8) + 0x90);
        (**temp_87f6e103bce)();
    }
    if (_PTR_LOOP_1050_0388 != 0x0) {
        unsafe { ppcVar1 = *_PTR_LOOP_1050_0388 };
        (**ppcVar1)();
    }
    post_quit_msg_1008_3af4(param_1);
    return;
}

pub fn pass1_1008_1272(param_1: u32, uparam_2: i32) {
    let mut uint_1: i32;
    let mut fn_ptr_1: u32;

    uint_1 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0) {
        fn_ptr_1 = ((param_1 + 0xe8) + 0x88);
        (**fn_ptr_1)();
        return;
    }
    pass1_1008_9cc4(param_1 & 0xffff | uint_1 << 0x10, param_2);
    return;
}

pub fn pass1_1008_12aa(param_1: u32) {
    let mut local_ES_3: i32;
    char * *fn_ptr_1;

    local_ES_3 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0) {
        fn_ptr_1 = ((param_1 + 0xe8) + 0x8c);
        (**fn_ptr_1)();
        return;
    }
    pass1_1008_9ce0();
    return;
}

pub fn pass1_1008_3714(param_1: *mut *mut u8) {
    pass1_1008_3e0e(param_1);
    return;
}

pub fn pass1_1008_372c(param_1: i32, uparam_2: i32) {
    return CONCAT22(param_2, param_1 + 10);
}

pub fn pass1_1008_37aa(in_list_1: *mut u32, param_2: u8) -> *mut u32 {
    let mut uVar1: u16;

    uVar1 = (in_list_1 >> 0x10);
    unsafe { *in_list_1 = 0x380a };
    (in_list_1 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *in_list_1 = s_1_1050_389a };
    (in_list_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_list_1);
    }
    return in_list_1;
}

pub fn pass1_1008_37e4(param_1: u32, param_2: u8) {
    win_cleanup_1008_0618(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_392e(param_1: *mut u16, param_2: u16) {
    let local_BX_4: i16;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    unsafe { *param_1 = s_1_1050_389a };
    (local_BX_4 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *param_1 = (s_18_2_1050_3aa5 + 3) };
    (local_BX_4 + 2) = &PTR_LOOP_1050_1008;
    (local_BX_4 + 4) = param_2;
    unsafe { *param_1 = s_0_020_1050_3ab0 };
    (local_BX_4 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *param_1 = s_0_76_1050_3aa0 };
    (local_BX_4 + 2) = &PTR_LOOP_1050_1008;
    return param_1;
}

pub fn pass1_1008_397a(param_1: *mut u16) {
    let mut local_BX_4: u16;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    unsafe { *param_1 = s_0_76_1050_3aa0 };
    (local_BX_4 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *param_1 = s_0_020_1050_3ab0 };
    (local_BX_4 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *param_1 = s_1_1050_389a };
    (local_BX_4 + 2) = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_3afe(param_1: *mut astruct_181, param_2: u8) {
    let local_AX_8: *mut astruct_181;
    let mut uVar1: u16;

    local_AX_8 = param_1;
    local_AX_8 = local_AX_8 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(local_AX_8)));
    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    local_AX_8.field_0x2 = &PTR_LOOP_1050_1008;
    param_1.field_0x0 = s_1_1050_389a;
    local_AX_8.field_0x2 = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1008_3b7a(in_char_1: u8, in_char_2: u8, in_u16_3: u16) {
    let pbVar1: *mut byte;
    let local_SP: *mut u16;
    let local_BP__1: *mut u16;
    let mut local_SS__1: u16;
    let char_1: u8;

    local_SP = &stack0xfffe;
    char_1 = '\x0f';
    while {
        local_BP__1 = local_BP__1 + -1;
        local_SP = local_SP + -1;
        unsafe { *local_SP = *local_BP__1 };
        char_1 = char_1 + -1;
        '\0' < char_1
    } {}
    pbVar1 = (in_u16_3 + in_char_1);
    unsafe { *pbVar1 = *pbVar1 | in_char_2 };
    loop {
        // WARNING: Do nothing block with infinite loop
    }
}

pub fn pass1_1008_3d44(param_1: u16, param_2: u8) {
    let mut uVar1: u16;
    let local_res7: u8;
    let mut in_stack_00000008: u8;

    _param_1 = CONCAT13(local_res7, CONCAT12(param_2, param_1));
    uVar1 = (_param_1 >> 0x10);
    _param_1.ptr_a_lo = 0x380a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    _param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        error_check_1000_17ce(_param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}

pub fn pass1_1008_3db2(
    param_1: u8,
    param_2: u16,
    param_3: u16,
    param_4: bool,
    param_5: u8,
    param_6: u8,
    param_7: u8,
    param_8: u8,
) {
    let pbVar1: *mut byte;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut bVar5: u8;
    let ppcVar6: fn();
    let mut cVar7: u8;
    let mut uVar8: u32;
    let mut bVar9: u8;
    let mut bVar10: u8;
    let mut iVar11: i32;
    let mut bVar12: u8;
    let mut bVar13: u8;
    let local_BX__1: *mut u32;
    let puVar14: *mut u32;
    u32 * *local_SP;
    let unaff_BP: *mut u32;
    let unaff_SI: *mut byte;
    let pbVar15: *mut byte;
    let unaff_DI: *mut byte;
    let unaff_ES: *mut byte;
    let mut local_SS__1: u16;
    let puStack2: *mut u32;
    let temp_179f29f37356: *mut u32;
    let mut temp_5fdbfafefb: u32;

    loop {
        local_SP = &puStack2;
        local_SP = &puStack2;
        cVar7 = '\x0f';
        temp_179f29f37356 = unaff_BP;
        while {
            temp_179f29f37356 = (temp_179f29f37356 + -2);
            local_SP = local_SP + -1;
            unsafe { *local_SP = temp_179f29f37356 };
            cVar7 = cVar7 + -1;
            '\0' < cVar7
        } {}
        iVar11 = param_1;
        pbVar1 = (local_BX__1 + iVar11);
        bVar12 = param_3;
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        bVar2 = 9 < (unaff_SI & 0xf);
        bVar5 = bVar2 | param_5;
        pbVar1 = (local_BX__1 + iVar11);
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        bVar3 = 9 < (unaff_SI + bVar5 * '\x06' & 0xf);
        pbVar1 = (local_BX__1 + iVar11);
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        let pb_var1_val = unsafe { *pbVar1 };
        bVar9 = ((POPCOUNT(pb_var1_val) & 1) == 0) * 0x4;
        pbVar1 = (local_BX__1 + iVar11);
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        bVar5 = 9 < bVar9 | bVar3 | bVar5;
        bVar10 = bVar9 + bVar5 * '\x06' & 0xf;
        pbVar1 = (local_BX__1 + iVar11);
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        bVar5 = 9 < bVar10 | bVar5;
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        pbVar15 = unaff_DI + -1;
        pbVar1 = (local_BX__1 + pbVar15);
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        pbVar1 = (local_BX__1 + pbVar15);
        unsafe { *pbVar1 = *pbVar1 | bVar12 };
        param_3 = param_3 - 1;
        pbVar1 = (local_BX__1 + pbVar15);
        bVar13 = param_3;
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        pbVar1 = unaff_DI;
        unsafe { *pbVar1 = *pbVar1 & (local_BX__1 >> 8) };
        pbVar1 = (local_BX__1 + pbVar15);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        bVar12 = (bVar10 + bVar5 * '\x06' & 0xf) + 1;
        let pv_var15_val = unsafe { *pbVar15 };
        out(pv_var15_val, param_3);
        bVar4 = 9 < (bVar12 & 0xf);
        bVar5 = bVar4 | bVar5;
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        puVar14 = local_BX__1;

        if (pb_var1_val == 0) {}
        // goto code_r0x10083e29;
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        if ((POPCOUNT(pb_var1_val) & 1) == 0) {
            break;
        }
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        puVar14 = ((param_8 & 1) * 0x4000
            | (param_7 & 1) * 0x200
            | (param_6 & 1) * 0x100
            | (pb_var1_val < '\0') * 0x80
            | (pb_var1_val == 0) * 0x40
            | ((bVar4 | 9) < bVar10 | (9 < bVar9) | bVar3 | bVar2 | param_5 & 1) * 0x10
            | ((POPCOUNT(pb_var1_val) & 1) == 0) * 4);
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        pbVar1 = unaff_DI + 0x1008;
        unsafe { *pbVar1 = *pbVar1 ^ local_BX__1 };
        unsafe { bVar9 = *pbVar1 };
        unaff_DI[0x1008] = local_BX__1;
        temp_5fdbfafefb = (unaff_DI + 0x1008);
        unaff_ES = (temp_5fdbfafefb >> 0x10);
        local_BX__1 = temp_5fdbfafefb;
        param_2 = param_2 - 1;
        if (param_2 == 0 || bVar9 == 0) {
            pbVar1 = (local_BX__1 + unaff_DI);
            unsafe { *pbVar1 = *pbVar1 | bVar13 };
            unaff_ES = unaff_DI;
            if (puVar14[1] != 0) {
                // code_r0x10083e29:
                uVar8 = puVar14[1];
                unaff_ES = (uVar8 >> 0x10);
                local_BX__1 = uVar8;
                // code_r0x10083e2d:
                let local_bx_1_val = unsafe { *local_BX__1 };
                ppcVar6 = (local_bx_1_val + 4);
                puStack2 = unaff_BP;
                (**ppcVar6)();
            }
            return;
        }
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        bVar9 = (bVar12 + bVar5 * '\x06' & 0xf) - 1;
        bVar5 = 9 < (bVar9 & 0xf) | bVar5;
        bVar9 = bVar9 + bVar5 * '\x06' & 0xf;
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        param_5 = 9 < bVar9 | bVar5;
        param_1 = bVar9 + param_5 * '\x06' & 0xf;
        pbVar1 = (local_BX__1 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar13 };
        unaff_BP = local_SP;
        unaff_SI = unaff_DI;
    }
    let pb_var1_val = unsafe { *pbVar1 };
    if (pb_var1_val == 0) {}
    // goto code_r0x10083e29;
    // goto code_r0x10083e2d;
}

pub fn pass1_1008_3e0e(param_1: *mut *mut u8) {
    let mut local_ES_3: u16;
    // fn_ptr_1: *mut *mut u8;

    local_ES_3 = (param_1 >> 0x10);
    if ((param_1 + 4) != 0) {
        fn_ptr_1 = ((param_1 + 4) + 4);
        (**fn_ptr_1)();
    }
    return;
}

pub fn pass1_1008_3e54(param_1: *mut u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut local_ES_6: u16;

    local_ES_6 = (param_1 >> 0x10);
    unsafe { *param_1 = param_4 };
    (param_1 + 2) = param_3;
    (param_1 + 4) = param_2;
    return param_1;
}

pub fn pass1_1008_3e76(param_1: *mut u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut local_ES_6: u16;

    local_ES_6 = (param_1 >> 0x10);
    unsafe { *param_1 = param_4 };
    (param_1 + 2) = param_3;
    (param_1 + 4) = param_2;
    return;
}

pub fn pass1_1008_3e94(param_1: *mut astruct_199, param_2: *mut u8, param_3: *mut u8) {
    let mut in_stack_00000006: u16;

    unsafe { param_3 = *_param_1 };
    param_2 = param_1.field_0x2;
    return;
}

pub fn pass1_1008_3eb4(param_1: *mut u16, param_2: u32, param_3: u32, param_4: u32) {
    let mut local_ES_15: u16;

    unsafe { param_4 = *param_1 };
    local_ES_15 = (param_1 >> 0x10);
    param_3 = (param_1 + 2);
    param_2 = (param_1 + 4);
    return;
}

pub fn pass1_1008_3ee2(param_1: *mut i32, param_2: *mut i32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_ES_26: u16;
    let mut local_ES_33: u16;

    unsafe { iVar1 = *param_2 - *param_1 };
    if (iVar1 < 0) {
        iVar1 = -iVar1;
    }
    unsafe { *param_1 = iVar1 + 1 };
    local_ES_26 = (param_2 >> 0x10);
    local_ES_33 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar1 = (param_2 + 2) - (iVar2 + 2);
    if (iVar1 < 0) {
        iVar1 = -iVar1;
    }
    (iVar2 + 2) = iVar1 + 1;
    iVar1 = (param_2 + 4) - (iVar2 + 4);
    if (iVar1 < 0) {
        iVar1 = -iVar1;
    }
    (iVar2 + 4) = iVar1 + 1;
    return;
}

pub fn pass1_1008_3f32(param_1: *mut i32, param_2: *mut i32) {
    let piVar1: *mut i32;
    let mut local_ES_15: u16;
    let mut local_ES_22: u16;

    unsafe { *param_1 = *param_1 + *param_2 };
    local_ES_15 = (param_2 >> 0x10);
    local_ES_22 = (param_1 >> 0x10);
    piVar1 = (param_1 + 2);
    unsafe { *piVar1 = *piVar1 + (param_2 + 2) };
    piVar1 = (param_1 + 4);
    unsafe { *piVar1 = *piVar1 + (param_2 + 4) };
    return;
}

pub fn pass1_1008_3f92(in_struct_a: *mut astruct_103, param_2: *mut astruct_183) {
    let struct_a_hi: *mut astruct_103;
    let struct_a: *mut astruct_103;
    // fn_ptr_1: *mut *mut u8;

    struct_a = in_struct_a;
    set_struct_1008_56b4(struct_a);
    struct_a_hi = (in_struct_a >> 0x10);
    struct_a.field_0x6 = 0;
    struct_a.field_0xa = 0;
    struct_a.field_0xe = 0;
    struct_a.field_0x10 = 0;
    struct_a.field_0x14 = 0;
    struct_a.field_0x18 = 0;
    struct_a.field_0x1c = 0;
    in_struct_a.ptr_1_lo = &PTR_LOOP_1050_48de;
    struct_a.ptr_1_hi = &PTR_LOOP_1050_1008;
    if (param_2 == 0x0) {
        return;
    }
    fn_ptr_1 = (param_2 + 8);
    (**fn_ptr_1)();
    pass1_1008_4214(struct_a, param_2);
    process_struct_1008_47cc(in_struct_a);
    process_struct_1008_4834(in_struct_a);
    return;
}

pub fn pass1_1008_405c(
    param_1: *mut astruct_103,
    param_2: *mut u32,
    param_3: u16_00,
    param_3: i32,
    param_4: i32,
) -> i32 {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut local_AX_116: u16;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let paVar6: *mut astruct_103;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut temp_5fb8438c55: u32;
    let lVar3: u32;

    paVar6 = param_1;
    set_struct_1008_56b4(paVar6);
    uVar5 = (param_1 >> 0x10);
    paVar6.field_0x6 = 0;
    paVar6.field_0xa = 0;
    paVar6.field_0xe = 0;
    paVar6.field_0x10 = 0;
    paVar6.field_0x14 = 0;
    paVar6.field_0x18 = 0;
    paVar6.field_0x1c = 0;
    param_1.ptr_1_lo = &PTR_LOOP_1050_48de;
    paVar6.ptr_1_hi = &PTR_LOOP_1050_1008;
    iVar2 = param_4 * 8 + 0x1f;
    iVar2 = ((iVar2 + (iVar2 >> 0xf & 0x1f)) >> 5) << 2;
    _local_a = param_3;
    uVar4 = ((iVar2 * param_3) >> 0x20);
    lVar3 = (iVar2 * param_3) + 0x436;
    alloc_mem_1000_0a48(1, lVar3, __g_astruct_94_ptr_1);
    &paVar6.field_0x6 = lVar3;
    (&paVar6.field_0x6 + 2) = uVar4;
    process_struct_1008_47cc((param_1 & 0xffff | uVar5 << 0x10));
    &paVar6.field_0x18 = iVar2;
    (&paVar6.field_0x18 + 2) = iVar2 >> 0xf;
    paVar6.field_0x10 = 0x28;
    temp_5fb8438c55 = paVar6.field_0x10;
    (temp_5fb8438c55 + 4) = param_4;
    uVar1 = paVar6.field_0x10;
    (uVar1 + 8) = _local_a;
    uVar1 = paVar6.field_0x10;
    (uVar1 + 0xc) = 1;
    uVar1 = paVar6.field_0x10;
    (uVar1 + 0xe) = 8;
    uVar1 = paVar6.field_0x10;
    (uVar1 + 0x10) = 0;
    uVar1 = paVar6.field_0x10;
    (uVar1 + 0x14) = paVar6.field_0x18 * _local_a;
    uVar1 = paVar6.field_0x10;
    (uVar1 + 0x20) = 0x100;
    uVar1 = paVar6.field_0x10;
    (uVar1 + 0x24) = 0x100;
    process_struct_1008_4834(param_1);
    pass1_1008_4d84(paVar6.field_0xa, param_2);
    return;
}

pub fn pass1_1008_4214(param_1: *mut astruct_103, in_astruct_183: *mut astruct_183) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_astruct_183: *mut astruct_183;
    let mut local_ES_5: u16;
    let mut in_stack_00000006: u16;
    let temp_862d998a63d: *mut u32;
    // fn_ptr_1: *mut *mut u8;

    local_ES_5 = (in_astruct_183 >> 0x10);
    local_astruct_183 = in_astruct_183;
    param_1.field_0x6 = local_astruct_183.field_0x1a;
    local_astruct_183.field_0x1a = 0;
    puVar1 = local_astruct_183.field_0x4;
    uVar2 = &local_astruct_183.field_0x6;
    if ((uVar2 | puVar1) != 0) {
        unsafe { fn_ptr_1 = *puVar1 };
        (**fn_ptr_1)();
    }
    &local_astruct_183.field_0x4 = 0;
    local_astruct_183.field_0xe = 0;
    local_astruct_183.field_0x12 = 0;
    local_astruct_183.field_0x16 = 0;
    local_astruct_183.field_0x1e = 0;
    return '\0';
}

pub fn pass1_1008_431c(in_astruct_184: *mut astruct_184, param_2: u8) {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut local_AX_134: u16;
    let mut local_DX_134: u16;
    let local_astruct_184: *mut astruct_184;
    let mut uVar5: i32;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fa7e01195: u32;
    let mut uVar4: u32;

    uVar5 = (in_astruct_184 >> 0x10);
    local_astruct_184 = in_astruct_184;
    if (&local_astruct_184.field_0x6 == 0) {
        process_struct_1008_47cc((in_astruct_184 & 0xffff | uVar5 << 0x10));
    }
    if ((local_astruct_184.field_0x8 | local_astruct_184.field_0x6) == 0) {
        bVar3 = false;
    } else {
        if ((local_astruct_184.field_0xc | local_astruct_184.field_0xa) == 0) {
            process_struct_1008_4834((in_astruct_184 & 0xffff | uVar5 << 0x10));
        }
        bVar3 = true;
    }
    if (bVar3) {
        if ((local_astruct_184.field_0x16 | local_astruct_184.field_0x14) == 0) {
            return;
        }
        local_6 = 0;
        while (true) {
            temp_5fa7e01195 = local_astruct_184.field_0x10;
            puVar1 = (temp_5fa7e01195 + 8);
            let pu_var1_val = unsafe { *puVar1 };
            if (pu_var1_val == local_6 || pu_var1_val < local_6) {
                break;
            }
            uVar4 = local_6;
            process_struct_1008_4544(in_astruct_184);
            uVar2 = local_astruct_184.field_0x10;
            pass1_1000_4906(
                (uVar4 & 0xffff | local_DX_134 << 0x10),
                param_2,
                (uVar2 + 4),
            );
            local_6 = local_6 + 1;
        }
    }
    return;
}

pub fn pass1_1008_43cc(param1: *mut i32) {
    let mut bVar1: bool;
    let num_list: *mut i32;
    let mut uVar2: i32;
    let mut local_4: u16;

    uVar2 = (i32_list >> 0x10);
    num_list = i32_list;
    if ((num_list + 3) == 0) {
        process_struct_1008_47cc((i32_list & 0xffff | uVar2 << 0x10));
    }
    if ((num_list + 3) == 0) {
        bVar1 = false;
    } else {
        if ((num_list + 5) == 0) {
            process_struct_1008_4834((i32_list & 0xffff | uVar2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0;
    }
    return CONCAT22(num_list[0xb], num_list[10]);
}

pub fn pass1_1008_4426(param_1: *mut astruct_104) {
    let mut bVar1: bool;
    let local_BX_4: *mut astruct_104;
    let mut uVar2: i32;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.a == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | uVar2 << 0x10));
    }
    if (&local_BX_4.a == 0) {
        bVar1 = false;
    } else {
        if (&local_BX_4.field_0xa == 0) {
            process_struct_1008_4834((param_1 & 0xffff | uVar2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0;
    }
    return CONCAT22(&local_BX_4.field_0xc, &local_BX_4.field_0xa);
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_4480(param_1: u32, in_struct_1: *mut astruct_417, param_3: u32) {
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut local_DX_29: u16;
    let mut local_DX_97: u16;
    let mut local_DX_122: u16;
    let mut local_SS__1: u16;
    let mut uVar4: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94(
        in_struct_1,
        CONCAT22(local_SS__1, &local_6),
        CONCAT22(local_SS__1, &local_4),
    );
    uVar4 = process_struct_1008_4772(param_3);
    local_DX_29 = (uVar4 >> 0x10);
    uVar1 = (uVar4 + 4);
    iVar2 = (uVar4 + 8);
    local_10 = 0;
    while (local_10 < iVar2) {
        uVar3 = local_6;
        local_6 = local_6 + 1;
        process_struct_1008_4544(param_1);
        _local_14 = CONCAT22(local_DX_97, uVar3);
        uVar4 = SEXT24(local_10);
        process_struct_1008_4544(param_3);
        _local_18 = (uVar4 & 0xffff | local_DX_122 << 0x10);
        local_1a = uVar1;
        while (local_1a != 0) {
            if (*_local_18 != -1) {
                unsafe { *_local_14 = *_local_18 };
            }
            _local_18 = CONCAT22(
                (_local_18 >> 0x10) + (-(0xfffe < local_18) & 0x6c),
                local_18 + 1,
            );
            _local_14 = CONCAT22(
                (_local_14 >> 0x10) + (-(0xfffe < local_14) & 0x6c),
                local_14 + 1,
            );
            local_1a = local_1a - 1;
        }
        local_10 = local_10 + 1;
    }
    return;
}

pub fn pass1_1008_4b5e(param_1: *mut astruct_189) {
    let mut iVar1: i32;
    let local_struct_1: *mut astruct_189;
    let mut local_ES_3: u16;
    // fn_ptr: *mut *mut u8;

    local_ES_3 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0x1e == 0) {
        fn_ptr = (param_1 + 8);
        iVar1 = (**fn_ptr)();
        if (iVar1 == 0) {
            return 0;
        }
    }
    return CONCAT22(local_struct_1.field_0x6, local_struct_1.field_0x4);
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_4b8e(in_astruct_7_1: *mut astruct_7, param_2: *mut astruct_131) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut local_DX_11: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_res6: u16;
    let mut in_stack_0000ffe4: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar4 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x48),
    );
    uVar2 = (ppVar4 >> 0x10);
    uVar1 = (ppVar4 + 0x18);
    iVar3 = (ppVar4 + 0x16) / 2;
    local_10 = 0;
    while (local_a._0_2_ = uVar1, local_10 < iVar3) {
        pass1_1008_4d26(
            &in_astruct_7_1.func_ptr_0x4,
            uVar1 & 0xffff0000 | (local_10 * 4 + local_a),
            local_10,
        );
        local_10 = local_10 + 1;
    }
    local_12 = 0x100 - iVar3;
    while (local_12 < 0x100) {
        pass1_1008_4d26(
            &in_astruct_7_1.func_ptr_0x4,
            uVar1 & 0xffff0000 | (local_10 * 4 + local_a),
            local_12,
        );
        local_12 = local_12 + 1;
        local_10 = local_10 + 1;
    }
    return;
}

pub fn pass1_1008_4cdc(param_1: *mut astruct_192) {
    let local_BX_4: *mut astruct_192;
    let mut local_ES_4: u16;
    let mut temp_5f3eb4e5e7: u32;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = 0x4f1c;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    error_check_1000_17ce(local_BX_4.field_0xe);
    if (local_BX_4.field_0x12 != 0) {
        error_check_1000_17ce(local_BX_4.field_0x4);
    }
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_4d26(param_1: *mut astruct_193, param_2: u32, param_3: u16) {
    let puVar1: *mut u16;
    let local_BX_4: *mut astruct_193;
    let mut iVar2: i32;
    let mut local_ES_4: u16;
    let mut local_ES_51: u16;
    let mut temp_5ffd109e1a: u16;
    let mut temp_5fce800a58: u32;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    let pu_var1_val = unsafe { *puVar1 };
    if (((local_BX_4.field_0x4 != 0) && (-1 < param_3))
        && (
            puVar1 = &local_BX_4.field_0xc,
            pu_var1_val != param_3 && param_3 <= pu_var1_val,
        ))
    {
        temp_5ffd109e1a = (param_2 + 2);
        temp_5fce800a58 = local_BX_4.field_0x4;
        local_ES_51 = (temp_5fce800a58 >> 0x10);
        iVar2 = temp_5fce800a58;
        (iVar2 + param_3 * 4) = param_2;
        (iVar2 + param_3 * 4 + 2) = temp_5ffd109e1a;
        return 1;
    }
    return 0;
}

pub fn pass1_1008_4d72(param_1: u32) {
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 6), (param_1 + 4));
}

pub fn pass1_1008_4d84(param_1: *mut astruct_194, param_2: *mut u32) {
    let puVar1: *mut u16;
    let in_DX: *mut astruct_199;
    let local_BX_4: *mut astruct_194;
    let mut local_ES_4: u16;
    let mut local_ES_12: u16;
    let mut in_stack_0000000a: u16;
    let mut temp_5fed13c9c0: u32;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x12 != 0) {
        local_BX_4.field_0xc = (param_2 + 3);
        error_check_1000_17ce(&local_BX_4.field_0x4);
        &local_BX_4.field_0x4 = 0;
        puVar1 = (local_BX_4.field_0xc << 2);
        process_struct_1000_179c(puVar1, in_DX);
        local_BX_4.field_0x4 = puVar1;
        &local_BX_4.field_0x6 = in_DX;
    }
    if (local_BX_4.field_0xc != 0x100) {
        return;
    }
    pass1_fn_1000_48a8(&local_BX_4.field_0x4, param_2[1], 0x400);
    return;
}

// WARNING: Variable defined which should be unmapped: local_4

pub fn pass1_1008_5068(param_1: *mut astruct_103, param_2: u16_00, param_2: *mut astruct_183) {
    pass1_1008_4214(param_1, param_2);
    return;
}

pub fn pass1_1008_507c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    process_struct_1008_41bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_5118(param_1: *mut astruct_376) {
    let mut local_ES_3: u16;
    let mut temp_5f496a68b0: u32;

    local_ES_3 = (param_1 >> 0x10);
    if ((param_1 + 0x10) != 0) {
        temp_5f496a68b0 = (param_1 + 0x10);
        error_check_1000_0dc6(temp_5f496a68b0, (temp_5f496a68b0 >> 0x10));
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_5134(param_1: u32) {
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut var4: u16;
    let mut local_AX_21: u16;
    let mut uVar5: i32;
    let lVar6: u32;
    let mut iVar7: i32;
    let local_BX_6: *mut astruct_196;
    let mut local_ES_6: u16;
    let mut bVar8: bool;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    local_ES_6 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    sVar4 = &local_BX_6.field_0x4 * &local_BX_6.field_0x8;
    uVar5 = (sVar4 >> 0x20);
    local_AX_21 = sVar4;
    alloc_mem_1000_0a48(
        1,
        local_AX_21,
        (sVar4 >> 0x10),
        __g_astruct_94_ptr_1,
        (__g_astruct_94_ptr_1 >> 0x10),
    );
    local_BX_6.field_0x10 = local_AX_21;
    local_BX_6.field_0x12 = uVar5;
    if ((uVar5 | local_BX_6.field_0x10) == 0) {
        return;
    }
    iVar7 = local_BX_6.field_0x8;
    iVar2 = local_BX_6.field_0xa;
    lVar6 = CONCAT22(iVar2 - (iVar7 == 0), iVar7 + -1) * &local_BX_6.field_0x4;
    puVar1 = &local_BX_6.field_0x10;
    uVar5 = lVar6;
    let pu_var1_val = unsafe { *puVar1 };
    local_a = uVar5 + pu_var1_val;
    local_8 = ((lVar6 >> 0x10) + CARRY2(uVar5, pu_var1_val)) * 0x100 + local_BX_6.field_0x12;
    _local_e = CONCAT22(iVar2, iVar7);
    local_10 = local_BX_6.field_0x2;
    while (_local_e != 0) {
        uVar3 = local_10 + 1;
        iVar7 = local_10 >> 0xf;
        process_struct_1008_4544(local_BX_6.field_0xc);
        pass1_fn_1000_48a8(
            CONCAT22(local_8, local_a),
            CONCAT22(iVar7, local_10),
            local_BX_6.field_0x4,
        );
        iVar7 = local_BX_6.field_0x4;
        uVar5 = -iVar7;
        bVar8 = CARRY2(local_a, uVar5);
        local_a = local_a + uVar5;
        local_8 = local_8 + (bVar8 - (local_BX_6.field_0x6 + (iVar7 != 0))) * 0x100;
        local_10 = uVar3;
        _local_e = _local_e + -1;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_5236(param_1: *mut astruct_197) {
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let lVar5: u32;
    let mut iVar6: i32;
    let local_BX_6: *mut astruct_197;
    let mut local_ES_6: u16;
    let mut bVar7: bool;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_6 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    iVar6 = local_BX_6.field_0x8;
    iVar2 = local_BX_6.field_0xa;
    lVar5 = CONCAT22(iVar2 - (iVar6 == 0), iVar6 + -1) * &local_BX_6.field_0x4;
    puVar1 = &local_BX_6.field_0x10;
    uVar4 = lVar5;
    let pu_var1_val = unsafe { *puVar1 };
    local_6 = uVar4 + pu_var1_val;
    local_4 = ((lVar5 >> 0x10) + CARRY2(uVar4, pu_var1_val)) * 0x100 + local_BX_6.field_0x12;
    _local_a = CONCAT22(iVar2, iVar6);
    local_c = local_BX_6.field_0x2;
    while (_local_a != 0) {
        uVar3 = local_c + 1;
        iVar6 = local_c >> 0xf;
        process_struct_1008_4544(local_BX_6.field_0xc);
        pass1_fn_1000_48a8(
            CONCAT22(iVar6, local_c),
            CONCAT22(local_4, local_6),
            local_BX_6.field_0x4,
        );
        iVar6 = local_BX_6.field_0x4;
        uVar4 = -iVar6;
        bVar7 = CARRY2(local_6, uVar4);
        local_6 = local_6 + uVar4;
        local_4 = local_4 + (bVar7 - (local_BX_6.field_0x6 + (iVar6 != 0))) * 0x100;
        local_c = uVar3;
        _local_a = _local_a + -1;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_52fc(param_1: *mut astruct_195) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let lVar4: u32;
    let mut local_DX_15: u16;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let local_BX_5: *mut astruct_195;
    let mut local_ES_5: u16;
    let mut uVar7: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let temp_87f1a411929: *mut u32;

    local_ES_5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    uVar7 = process_struct_1008_4772(local_BX_5.field_0xc);
    local_DX_15 = (uVar7 >> 0x10);
    iVar6 = uVar7;
    iVar5 = (iVar6 + 4);
    uVar3 = iVar5 - 1;
    iVar5 = (iVar6 + 6) - (iVar5 == 0);
    lVar4 = (iVar6 + 8) + -1;
    uVar2 = param_1.field_0x0;
    temp_87f1a411929 = &local_BX_5.field_0x4;
    iVar6 = (uVar2 >> 0xf) + (&local_BX_5.field_0x4 + 2) + CARRY2(uVar2, temp_87f1a411929);
    if ((iVar5 <= iVar6) && (iVar5 < iVar6 || (uVar3 < uVar2 + temp_87f1a411929))) {
        &local_BX_5.field_0x4 = uVar3 - uVar2;
        (&local_BX_5.field_0x4 + 2) = (iVar5 - (uVar2 >> 0xf)) - (uVar3 < uVar2);
    }
    uVar2 = local_BX_5.field_0x2;
    puVar1 = &local_BX_5.field_0x8;
    iVar5 = (uVar2 >> 0xf) + (&local_BX_5.field_0x8 + 2) + CARRY2(uVar2, puVar1);
    local_e._2_2_ = (lVar4 >> 0x10);
    if ((local_e._2_2_ <= iVar5)
        && ((
            local_e._0_2_ = lVar4,
            local_e._2_2_ < iVar5 || (local_e < uVar2 + puVar1),
        )))
    {
        &local_BX_5.field_0x8 = local_e - uVar2;
        (&local_BX_5.field_0x8 + 2) = (local_e._2_2_ - (uVar2 >> 0xf)) - (local_e < uVar2);
    }
    return;
}

pub fn pass1_1008_570e(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = s_1_1050_389a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_5784(param_1: *mut *mut u8, param_2: u32) {
    param_1 = param_2;
    (param_1 + 4) = 0;
    return;
}

pub fn pass1_1008_57a4(param_1: *mut u32, param_2: u32) {
    unsafe {
        unsafe { *param_1 = param_2 };
        (param_1 + 4) = 0;
    }
}

pub fn pass1_1008_57c4(param_1: *mut astruct_44) {
    let uVar1: u8;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    param_1.ptr_a_lo = (s__s__s__1050_5bc0 + 4);
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    uVar1 = pass1_1008_5830((param_1 & 0xffff | local_ES_4 << 0x10));
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    return uVar1;
}

pub fn pass1_1008_57f0(param_1: u32, param_2: u16) -> libc::c_long {
    let mut local_SS__1: u16;
    let mut bVar1: bool;
    let lVar2: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(local_SS__1, local_a), param_1);
    local_c = 0;
    while {
        lVar2 = pass1_1008_5b12(CONCAT22(local_SS__1, local_a));
        if (lVar2 == 0) {
            return 0;
        }
        bVar1 = local_c != param_2;
        local_c = local_c + 1;
        bVar1
    } {}
    return lVar2;
}

pub fn pass1_1008_5830(param_1: *mut astruct_200) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let puVar4: *mut u32;
    let local_BX_5: *mut astruct_200;
    let mut iVar5: i32;
    let mut local_ES_5: u16;
    let mut local_ES_23: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7e01ca0f901: *mut u32;
    let mut temp_5f0adb14fb: u32;
    // temp_87fa1582f55: *mut *mut u8;
    let temp_8622404347c: *mut u32;

    while (true) {
        local_ES_5 = (param_1 >> 0x10);
        local_BX_5 = param_1;
        if (local_BX_5.field_0x4 == 0) {
            break;
        }
        if (local_BX_5.field_0xa != 0) {
            temp_5f0adb14fb = local_BX_5.field_0x4;
            local_ES_23 = (temp_5f0adb14fb >> 0x10);
            iVar5 = temp_5f0adb14fb;
            puVar1 = (iVar5 + 8);
            uVar2 = (iVar5 + 10);
            if ((uVar2 | puVar1) != 0) {
                unsafe { temp_87fa1582f55 = *puVar1 };
                (**temp_87fa1582f55)();
            }
        }
        puVar4 = local_BX_5.field_0x4;
        local_BX_5.field_0x4 = (puVar4 + 4);
        if (puVar4 != 0x0) {
            unsafe { ppcVar3 = *puVar4 };
            (**ppcVar3)();
        }
    }
    local_BX_5.field_0x8 = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_fn_1008_58a6(param_1: u32, param_2: u32) {
    let piVar1: *mut i32;
    let local_AX_10: *mut astruct_201;
    let mut uVar2: i32;
    let mut extraout_DX: i32;
    let mut iVar4: i32;
    let mut local_ES_102: u16;
    let mut local_ES_110: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let puVar3: *mut u8;

    puVar3 = _PTR_LOOP_1050_029c;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_029c);
    uVar2 = puVar3;
    local_6 = puVar3 & 0xffff | extraout_DX << 0x10;
    if ((extraout_DX | uVar2) == 0) {
        local_6 = 0;
    } else {
        local_6 = s_1_1050_389a;
        (uVar2 + 2) = &PTR_LOOP_1050_1008;
        (uVar2 + 4) = 0;
        (uVar2 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (uVar2 + 2) = &PTR_LOOP_1050_1008;
    }
    if (local_6 == 0) {
        return;
    }
    local_ES_102 = (local_6 >> 0x10);
    (local_6 + 8) = param_2;
    local_ES_110 = (param_1 >> 0x10);
    iVar4 = param_1;
    (local_6 + 4) = (iVar4 + 4);
    (iVar4 + 4) = local_6;
    piVar1 = (iVar4 + 8);
    unsafe { *piVar1 = *piVar1 + 1 };
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_593c(param_1: *mut u32, param_2: u32) {
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let mut uVar3: i32;
    let puVar4: *mut u8;
    let mut extraout_DX: i32;
    let mut iVar5: i32;
    let mut local_ES_4: u16;
    let mut local_ES_150: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    let mut temp_5f74faf803: u32;

    local_ES_4 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 8) == 0) {
        let param_1_val = unsafe { *param_1 };
        ppcVar2 = (param_1_val + 4);
        ppcVar2();
        return;
    }
    puVar4 = _PTR_LOOP_1050_029c;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_029c);
    uVar3 = puVar4;
    local_6 = puVar4 & 0xffff | extraout_DX << 0x10;
    if ((extraout_DX | uVar3) == 0) {
        local_6 = 0;
    } else {
        local_6 = s_1_1050_389a;
        (uVar3 + 2) = &PTR_LOOP_1050_1008;
        (uVar3 + 4) = 0;
        (uVar3 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (uVar3 + 2) = &PTR_LOOP_1050_1008;
    }
    if (local_6 == 0) {
        return;
    }
    (local_6 + 8) = param_2;
    while {
        param_1 = (param_1 + 4);
        local_ES_150 = (param_1 >> 0x10);
        (param_1 + 4) != 0
    } {}
    todo!();
    //(param_1 + 4) = local_6;
    piVar1 = (iVar5 + 8);
    unsafe { *piVar1 = *piVar1 + 1 };
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_59f4(param_1: u32, param_2: u32) {
    let piVar1: *mut i32;
    let puVar2: *mut u32;
    let mut uVar3: i32;
    let mut local_AX_110: u16;
    let mut local_DX_110: u16;
    let mut iVar4: i32;
    let mut local_ES_12: u16;
    let mut local_ES_24: u16;
    let mut local_ES_42: u16;
    let mut local_ES_110: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    let temp_8623d993e9d: *mut u32;
    let temp_79fa913f4bf: *mut u32;
    let mut temp_79f326fe867: u32;
    let fn_ptr_1: fn();

    local_6 = 0;
    local_ES_12 = (param_1 >> 0x10);
    temp_79f326fe867 = local_6;
    temp_79fa913f4bf = param_1;
    while {
        local_6 = temp_79f326fe867;
        local_ES_24 = (temp_79fa913f4bf >> 0x10);
        iVar4 = temp_79fa913f4bf;
        temp_79fa913f4bf = (iVar4 + 4);
        local_a._0_2_ = temp_79fa913f4bf;
        local_ES_42 = (temp_79fa913f4bf >> 0x10);
        if (((iVar4 + 6) | local_a) == 0) {
            break;
        }
        temp_79f326fe867 = temp_79fa913f4bf;
        (local_a + 8) != param_2
    } {}
    if (temp_79fa913f4bf != 0x0) {
        if (local_6 == 0) {
            local_AX_110 = (local_a + 4);
            local_DX_110 = (local_a + 6);
            local_6 = param_1;
        } else {
            local_AX_110 = (local_a + 4);
            local_DX_110 = (local_a + 6);
        }
        local_ES_110 = (local_6 >> 0x10);
        (local_6 + 4) = local_AX_110;
        (local_6 + 6) = local_DX_110;
        if ((param_1 + 10) != 0) {
            puVar2 = (local_a + 8);
            uVar3 = (local_a + 10);
            if ((uVar3 | puVar2) != 0) {
                unsafe { fn_ptr_1 = *puVar2 };
                (**fn_ptr_1)();
            }
        }
        if (temp_79fa913f4bf != 0x0) {
            unsafe { fn_ptr_1 = *temp_79fa913f4bf };
            (**fn_ptr_1)();
        }
        piVar1 = (param_1 + 8);
        unsafe { *piVar1 = *piVar1 + -1 };
        return;
    }
    return;
}

pub fn pass1_fn_1008_5ab8(param_1: u32) {
    let piVar1: *mut i32;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut local_ES_4: u16;
    let mut uVar4: i32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7e05aed193b: *mut u32;
    let fn_ptr_1: fn();

    local_ES_4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 4) == 0) {
        return;
    }
    puVar2 = (iVar3 + 4);
    uVar4 = (puVar2 >> 0x10);
    (iVar3 + 4) = (puVar2 + 4);
    if ((uVar4 | puVar2) != 0) {
        unsafe { fn_ptr_1 = *puVar2 };
        (**fn_ptr_1)();
    }
    piVar1 = (iVar3 + 8);
    unsafe { *piVar1 = *piVar1 + -1 };
    return;
}

pub fn pass1_1008_5b12(param_1: *mut astruct_306) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_ES_23: u16;
    let mut local_ES_34: u16;
    let mut temp_5fb237ca1c: u32;

    if ((param_1.field_0x0 != 0) && ((param_1.field_0x0 + 8) != 0)) {
        local_ES_23 = (param_1 >> 0x10);
        iVar1 = param_1;
        if ((iVar1 + 4) == 0) {
            local_ES_34 = (param_1.field_0x0 >> 0x10);
            iVar2 = param_1.field_0x0;
        } else {
            temp_5fb237ca1c = (iVar1 + 4);
            local_ES_34 = (temp_5fb237ca1c >> 0x10);
            iVar2 = temp_5fb237ca1c;
        }
        (iVar1 + 4) = (iVar2 + 4);
        if ((iVar1 + 4) != 0) {
            return;
        }
    }
    return;
}

pub fn pass1_fn_1008_5b9a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_57c4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_fn_1008_60e8(param_1: *mut char) -> uint {
    let mut uVar1: i32;
    let in_DX: *mut astruct_199;
    let mut local_8: u16;
    let mut local_6: u16;

    if (param_1 != 0x0) {
        uVar1 = get_string_index_1000_3da4(param_1);
        uVar1 = uVar1 + 1;
        process_struct_1000_179c(uVar1, in_DX);
        if ((in_DX | uVar1) != 0) {
            copy_string_1000_3d3e(CONCAT22(in_DX, uVar1), param_1);
            return uVar1;
        }
    }
    return 0;
}

pub fn pass1_fn_1008_612e(param_1: u16, param_2: i32) {
    let mut in_AX: i32;
    let mut uVar1: i32;
    let lVar2: u32;
    let mut iVar3: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_fn_1000_4d24();
    uVar1 = (param_2 - param_1) + 1;
    if ((uVar1 >> 0xf | uVar1) == 0) {
        return;
    }
    local_10 = 1;
    local_12 = param_1;
    loop {
        if (param_2 < local_12) {
            return;
        }
        lVar2 = local_10 * (0x7fff / uVar1);
        iVar3 = (lVar2 >> 0x10);
        if (in_AX >> 0xf <= iVar3) {
            if (in_AX >> 0xf < iVar3) {
                return;
            }
            if (in_AX <= lVar2) {
                return;
            }
        }
        local_12 = local_12 + 1;
        local_10 = local_10 + 1;
    }
}

pub fn pass1_1008_6330(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x380a };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *param_1 = s_1_1050_389a };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1008_6562(
    param_1: *mut *mut astruct_104,
    param_2: u32,
    param_3: i32,
    uparam_4: i32,
    param_5: *mut u16,
) -> i32 {
    let mut iVar1: i32;
    let mut local_DX_57: u16;
    let mut uVar2: u16;
    let mut local_DX_103: u16;
    let mut local_DX_129: u16;
    let mut uvar3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f1d1f0836: u32;

    let param_1_val = unsafe { *param_1 };
    if (param_1_val == 0x0) {
        return;
    }
    process_struct_1000_179c(0x1e, param_5);
    if ((param_5 | param_4) == 0) {
        param_4 = 0;
        uVar2 = 0;
    } else {
        temp_5f1d1f0836 = (param_1 + 4);
        pass1_1008_405c(
            param_4,
            param_5,
            temp_5f1d1f0836,
            (temp_5f1d1f0836 >> 0x10),
            param_2,
        );
        uVar2 = local_DX_57;
    }
    _local_6 = CONCAT22(uVar2, param_4);
    local_8 = 0;
    while (param_2 = param_2 & 0xffff0000 | (param_2 - 1), param_2 != 0) {
        iVar1 = param_3 + 1;
        process_struct_1008_4544(param_1_val);
        uVar2 = local_8 + 1;
        uVar3 = local_DX_103;
        process_struct_1008_4544(_local_6);
        pass1_fn_1000_48a8(
            CONCAT22(local_DX_129, local_8),
            CONCAT22(uVar3, param_3),
            param_2._2_2_,
        );
        param_3 = iVar1;
        local_8 = uVar2;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_6604(param_1: *mut u16, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let mut local_AX__1: u16;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut local_DX__1: u16;
    let mut local_DX_105: u16;
    let mut local_ES_15: u16;
    let mut local_ES_177: u16;
    let paVar4: *mut astruct_102;

    paVar4 = param_1;
    set_struct_1008_4016(paVar4);
    local_ES_15 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x685a };
    paVar4.field_0x2 = &PTR_LOOP_1050_1008;
    alloc_mem_1000_0a48(1, 0x28, __g_astruct_94_ptr_1);
    &paVar4.field_0x10 = local_AX__1;
    (&paVar4.field_0x10 + 2) = local_DX__1;
    iVar2 = param_3 * 8 + 0x1f;
    iVar2 = ((iVar2 + (iVar2 >> 0xf & 0x1f)) >> 5) << 2;
    &paVar4.field_0x18 = iVar2;
    (&paVar4.field_0x18 + 2) = iVar2 >> 0xf;
    local_DX_105 = ((iVar2 * param_2) >> 0x20);
    uVar3 = (iVar2 * param_2);
    alloc_mem_1000_0a48(1, uVar3, __g_astruct_94_ptr_1);
    &paVar4.field_0x6 = uVar3;
    (&paVar4.field_0x6 + 2) = local_DX_105;
    &paVar4.field_0x14 = &paVar4.field_0x6;
    (&paVar4.field_0x14 + 2) = local_DX_105;
    paVar4.field_0x10 = 0x28;
    uVar1 = paVar4.field_0x10;
    (uVar1 + 4) = param_3;
    uVar1 = paVar4.field_0x10;
    local_ES_177 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    (iVar2 + 8) = param_2;
    (iVar2 + 10) = param_2 >> 0xf;
    uVar1 = paVar4.field_0x10;
    (uVar1 + 0xc) = 1;
    uVar1 = paVar4.field_0x10;
    (uVar1 + 0xe) = 8;
    uVar1 = paVar4.field_0x10;
    (uVar1 + 0x10) = 0;
    uVar1 = paVar4.field_0x10;
    (uVar1 + 0x14) = paVar4.field_0x18 * param_2;
    uVar1 = paVar4.field_0x10;
    (uVar1 + 0x20) = 0x100;
    uVar1 = paVar4.field_0x10;
    (uVar1 + 0x24) = 0x100;
    return;
}

pub fn pass1_1008_6732(param_1: *mut astruct_182) {
    let mut iVar1: i32;
    let mut local_ES_3: u16;
    let mut temp_5f5866abc5: u32;

    local_ES_3 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1 = 0x685a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((iVar1 + 0x10) != 0) {
        temp_5f5866abc5 = (iVar1 + 0x10);
        error_check_1000_0dc6(temp_5f5866abc5, (temp_5f5866abc5 >> 0x10));
    }
    (iVar1 + 0x10) = 0;
    process_struct_1008_41bc(param_1);
    return;
}

pub fn pass1_1008_6834(param_1: u32, param_2: u8) {
    pass1_1008_6732(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_687a(param_1: *mut astruct_65, param_2: u32) {
    let mut iVar1: i32;
    let mut local_ES_17: u16;

    set_struct_1008_9584(param_1, param_2);
    local_ES_17 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xcc) = 0;
    (iVar1 + 0xce) = 0;
    process_struct_1008_574a((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    param_1.ptr_a_lo = 0x6bfc;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    (iVar1 + 0xdc) = 0;
    return;
}

pub fn pass1_1008_68ea(
    param_1: i32,
    param_2: u16,
    param_2_00: *mut u32,
    param_4: u16,
    param_5: u16,
    param_6: i32,
) -> i32 {
    let ppcVar1: fn();
    let mut local_6: u32;
    // temp_87f4899415d: *mut *mut u8;

    if (param_6 == 0) {
        if ((param_1 + 0xce) != CONCAT22(param_4, param_2_00)) {
            if ((param_1 + 0xce) != 0) {
                temp_87f4899415d = ((param_1 + 0xce) + 0x10);
                (**temp_87f4899415d)();
            }
            (param_1 + 0xce) = CONCAT22(param_4, param_2_00);
            let param_2_val = unsafe { *param_2_00 };
            ppcVar1 = (param_2_val + 0x10);
            (**ppcVar1)();
            ppcVar1 = ((param_1 + 0xce) + 0xc);
            (**ppcVar1)();
            return;
        }
    } else {
        pass1_1008_3e0e(CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)));
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_6978(param_1: *mut astruct_675, param_2: u16, param_3: u32) {
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    // func_ptr_1: *mut *mut u8;

    process_struct_1000_179c(10, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) == 0) {
        local_6 = 0;
    } else {
        if (param_2 == 0) {
            param_2 = (param_1 + 0xcc);
        }
        unsafe { *_local_a = s_1_1050_389a };
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        (in_AX + 4) = param_3;
        (in_AX + 8) = param_2;
        unsafe { *_local_a = 0x6c8c };
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        local_6 = _local_a;
    }
    func_ptr_1 = ((param_1 + 0xd2) + 4);
    (**func_ptr_1)(0x1000, (param_1 + 0xd2), param_1._2_2_, local_6);
    return;
}

pub fn pass1_1008_6a04(param_1: u32) {
    let local_AX_30: *mut u8;
    let mut extraout_DX: i32;
    let mut local_SS__1: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];
    // temp_87f48b94a86: *mut *mut u8;

    pass1_1008_57a4(
        CONCAT22(local_SS__1, local_a),
        param_1 & 0xffff0000 | (param_1 + 0xd2),
    );
    while (true) {
        local_AX_30 = local_a;
        pass1_1008_5b12(CONCAT22(local_SS__1, local_AX_30));
        if ((extraout_DX | local_AX_30) == 0) {
            break;
        }
        temp_87f48b94a86 = ((local_AX_30 + 4) + 0xc);
        (**temp_87f48b94a86)();
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_6a4a(param_1: u32, param_2: i32, param_3: u16, param_3_00: i32) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let local_AX_38: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut local_SS__1: u16;
    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    // temp_87fb77c20c3: *mut *mut u8;

    if (param_3_00 == 2) {
        iVar2 = param_1;
        pass1_1008_57a4(
            CONCAT22(local_SS__1, local_e),
            param_1 & 0xffff0000 | (iVar2 + 0xd2),
        );
        while {
            local_AX_38 = local_e;
            pass1_1008_5b12(CONCAT22(local_SS__1, local_AX_38));
            _local_6 = CONCAT22(extraout_DX, local_AX_38);
            if ((extraout_DX | local_AX_38) == 0) {
                break;
            }
            (local_AX_38 + 8) != param_2
        } {}
        if (_local_6 != 0) {
            temp_87fb77c20c3 = ((iVar2 + 0xd2) + 0xc);
            (**temp_87fb77c20c3)();
            local_a = 0;
            local_6 = local_e;
            pass1_1008_5b12(CONCAT22(local_SS__1, local_6));
            if ((extraout_DX_00 | local_6) != 0) {
                ppcVar1 = ((local_6 + 4) + 0x10);
                local_4 = extraout_DX_00;
                (**ppcVar1)();
                (iVar2 + 0xce) = (local_6 + 4);
                return;
            }
            (iVar2 + 0xce) = 0;
        }
    }
    return;
}

pub fn pass1_1008_6b02(param_1: u32) {
    let mut iVar1: i32;
    let mut local_ES_3: u16;
    // fn_ptr_1: *mut *mut u8;

    local_ES_3 = (param_1 >> 0x10);
    iVar1 = param_1;
    if (((iVar1 + 0xd0) | (iVar1 + 0xce)) != 0) {
        fn_ptr_1 = ((iVar1 + 0xce) + 0x6c);
        (**fn_ptr_1)();
    }
    return;
}

pub fn pass1_1008_6b5a(param_1: *mut u16, param_2: u8) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let temp_862bb3f10cd: *mut u32;
    // fn_ptr_1: *mut *mut u8;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    unsafe { *param_1 = 0x6c8c };
    (iVar3 + 2) = &PTR_LOOP_1050_1008;
    puVar1 = (iVar3 + 4);
    uVar2 = (iVar3 + 6);
    if ((uVar2 | puVar1) != 0) {
        unsafe { fn_ptr_1 = *puVar1 };
        (**fn_ptr_1)();
    }
    unsafe { *param_1 = s_1_1050_389a };
    (iVar3 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_6bb4(param_1: *mut astruct_203, param_2: u8) {
    let local_AX_8: *mut astruct_203;
    let mut uVar1: u16;

    local_AX_8 = param_1;
    local_AX_8 = local_AX_8 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(local_AX_8)));
    uVar1 = (param_1 >> 0x10);
    param_1 = 0x380a;
    local_AX_8.field_0x2 = &PTR_LOOP_1050_1008;
    param_1 = s_1_1050_389a;
    local_AX_8.field_0x2 = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1008_6c30(
    param_1: u8,
    param_2: i32,
    param_3: u8,
    param_4: i32,
    param_5: bool,
    param_6: bool,
    param_7: bool,
    param_8: bool,
    param_9: bool,
) -> i32 {
    let pcVar1: *mut char;
    let pbVar2: *mut byte;
    let mut uVar3: i32;
    let mut bVar4: bool;
    let mut bVar5: bool;
    let mut bVar6: u8;
    let mut bVar7: u8;
    let mut bVar8: u8;
    let mut bVar9: u8;
    let mut bVar10: u8;
    let mut iVar11: i32;
    let mut bVar12: u8;
    u16 * *local_SP;
    u16 * *unaff_BP;
    let mut unaff_SI: i32;
    let mut unaff_DI: i32;
    let mut local_SS__1: u16;
    let mut bVar13: bool;
    let in_stack_0000f71d: *mut astruct_1;
    let paStack2265: *mut astruct_1;
    let mut cStack3: u8;
    let puStack2: *mut u16;
    let temp_179f295966e9: *mut u16;
    let mut byte3: u8;

    loop {
        local_SP = &puStack2;
        local_SP = &puStack2;
        byte3 = '\x0f';
        temp_179f295966e9 = unaff_BP;
        while {
            temp_179f295966e9 = temp_179f295966e9 + -1;
            local_SP = local_SP + -1;
            unsafe { *local_SP = *temp_179f295966e9 };
            byte3 = byte3 + -1;
            '\0' < byte3
        } {}
        iVar11 = param_1;
        pbVar2 = (param_4 + iVar11);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        bVar4 = 9 < (unaff_SI & 0xf);
        bVar6 = bVar4 | param_7;
        pbVar2 = (param_4 + iVar11);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        bVar5 = 9 < (unaff_SI + bVar6 * '\x06' & 0xf);
        pbVar2 = (param_4 + iVar11);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        let pb_var2_val = unsafe { *pbVar2 };
        bVar7 = ((POPCOUNT(pb_var2_val) & 1) == 0) * 0x4;
        pbVar2 = (param_4 + iVar11);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        bVar6 = 9 < bVar7 | bVar5 | bVar6;
        bVar8 = bVar7 + bVar6 * '\x06' & 0xf;
        pbVar2 = (param_4 + iVar11);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        bVar6 = 9 < bVar8 | bVar6;
        bVar9 = bVar8 + bVar6 * '\x06' & 0xf;
        pbVar2 = (param_4 + unaff_DI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        unaff_SI = unaff_DI + -1;
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        bVar12 = param_3 - 1;
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | bVar12 };
        bVar6 = 9 < bVar9 | bVar6;
        bVar10 = bVar9 + bVar6 * '\x06' & 0xf;
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | bVar12 };
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | bVar12 };
        param_3 = param_3 - 2;
        bVar12 = (param_2 >> 8);
        bVar13 = CARRY1(u8_1050_086b, bVar12);
        u8_1050_086b = u8_1050_086b + bVar12;
        pcVar1 = &cStack3 + unaff_DI;
        unsafe { *pcVar1 = *pcVar1 + bVar10 + bVar13 };
        iVar11 = (param_4 + unaff_SI);
        puStack2 = unaff_BP;
        let pc_var1_val = unsafe { *pcVar1 };
        if ((POPCOUNT(pc_var1_val) & 1) == 0) {}
        // goto code_r0x10086ca6;
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        let pb_var2_val = unsafe { *pbVar2 };
        uVar3 = (param_9 & 1) * 0x4000
            | (param_8 & 1) * 0x200
            | (param_5 & 1) * 0x100
            | (pb_var2_val < '\0') * 0x80
            | (pb_var2_val == 0) * 0x40
            | ((9 < bVar9) | (9 < bVar8) | (9 < bVar7) | bVar5 | bVar4 | param_7 & 1) * 0x10
            | ((POPCOUNT(pb_var2_val) & 1) == 0) * 4;
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        paStack2265 = CONCAT22(uVar3, unaff_SI);
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        pbVar2 = (unaff_DI + 0x1007);
        unsafe { *pbVar2 = *pbVar2 ^ param_4 };
        unsafe { bVar7 = *pbVar2 };
        *(unaff_DI + 0x1007) = param_4;
        param_4 = (unaff_DI + 0x1007);
        param_2 = iVar11 * 0x10 + -1;
        if (param_2 == 0 || bVar7 == 0) {
            break;
        }
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        bVar10 = bVar10 - 1;
        bVar6 = 9 < (bVar10 & 0xf) | bVar6;
        bVar7 = bVar10 + bVar6 * '\x06' & 0xf;
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        param_7 = (9 < bVar7 | bVar6);
        param_1 = bVar7 + param_7 * '\x06' & 0xf;
        pbVar2 = (param_4 + unaff_SI);
        unsafe { *pbVar2 = *pbVar2 | param_3 };
        unaff_BP = local_SP;
    }
    pbVar2 = (param_4 + unaff_SI);
    unsafe { *pbVar2 = *pbVar2 | param_3 };
    zero_list_1008_3e38(paStack2265);
    in_stack_0000f71d = CONCAT22(uVar3, unaff_DI + 5);
    // code_r0x10086ca6:
    zero_list_1008_3e38(in_stack_0000f71d);
    return;
}

pub fn pass1_1008_6cec(param_1: u32, param_2: u16, param_3: u32, param_4: u16, param_5: u32) {
    pass1_1008_3e76(param_1, param_4, param_5, (param_5 >> 0x10));
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | (param_1 + 6)),
        param_2,
        param_3,
        (param_3 >> 0x10),
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_6d8a(param_1: u32, param_2: u32) {
    let mut local_AX_49: u16;
    let mut local_DX__1: u16;
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    param_1 = 0;
    (param_1 + 4) = 0xffff;
    PTR_LOOP_1050_0312 = &PTR_DAT_0005_0000_1050_0004;
    string_fn_1000_3f9c(
        s__1050_65a0,
        &g_alloc_addr_1050_1050,
        _PTR_s_SC_03d_1050_0314_1050_031c,
        (_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
        &PTR_DAT_0005_0000_1050_0004,
    );
    local_AX_49 = pass1_fn_1008_60e8(param_2);
    param_1 = local_AX_49;
    (param_1 + 2) = local_DX__1;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_6eee(param_1: u16, param_2: u16, param_1_00: u32) {
    let local_AX_45: *mut u8;
    let BVar1: bool;
    let puVar2: *mut u8;
    let mut local_SS__1: u16;
    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u32;

    unsafe { local_6 = *_g_bool_1050_5748 };
    local_a = local_6;
    pass1_1020_a43e(CONCAT22(local_SS__1, local_e));
    local_AX_45 = write_to_file_1028_d7a0(local_a, (local_a >> 0x10), param_1_00);
    if (local_AX_45 != 0x0) {
        BVar1 = write_to_file_1030_5c1a(_PTR_LOOP_1050_5736, param_1_00);
        if (BVar1 != 0) {
            write_to_file_1028_dce2(_PTR_LOOP_1050_65e2, param_1_00);
            if (BVar1 != 0) {
                write_to_file_1038_7b20(_PTR_LOOP_1050_5a64, param_1_00);
                if (BVar1 != 0) {
                    puVar2 = local_e;
                    call_write_to_file_1020_a644(puVar2, local_SS__1, param_1_00);
                    if (puVar2 != 0x0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_6f7a(param_1: u16, param_2: u16, param_1_00: u32) {
    let local_AX_45: *mut u8;
    let BVar1: bool;
    let puVar2: *mut u8;
    let mut local_SS__1: u16;
    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u32;

    unsafe { local_6 = *_g_bool_1050_5748 };
    local_a = local_6;
    pass1_1020_a43e(CONCAT22(local_SS__1, local_e));
    local_AX_45 = read_file_1028_d7ba(local_a, (local_a >> 0x10), param_1_00);
    if (local_AX_45 != 0x0) {
        BVar1 = read_file_1030_5c52(_PTR_LOOP_1050_5736, param_1_00);
        if (BVar1 != 0) {
            read_file_1028_def2(_PTR_LOOP_1050_65e2, param_1_00);
            if (BVar1 != 0) {
                read_from_file_1038_7c02(_PTR_LOOP_1050_5a64, param_1_00);
                if (BVar1 != 0) {
                    puVar2 = local_e;
                    call_read_file_1020_a65e(puVar2, local_SS__1, param_1_00);
                    if (puVar2 != 0x0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_8
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_7006(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let local_DXAX_37: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        if (PTR_LOOP_1050_0334 <= local_4) {
            return 1;
        }
        local_DXAX_37 = process_struct_1010_20ba(
            _g_astruct_372_1050_0ed0,
            CONCAT22(local_8, (local_4 * 2 + 800)),
        );
        unsafe { ppcVar1 = (*local_DXAX_37 + 8) };
        iVar2 = (**ppcVar1)(0x1010, local_DXAX_37, param_1_00);
        local_8 = local_DXAX_37;
        if (iVar2 == 0) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return iVar2;
}

// WARNING: Variable defined which should be unmapped: local_8
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_7056(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let local_DXAX_37: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        if (PTR_LOOP_1050_0334 <= local_4) {
            return 1;
        }
        local_DXAX_37 = process_struct_1010_20ba(
            _g_astruct_372_1050_0ed0,
            CONCAT22(local_8, (local_4 * 2 + 800)),
        );
        unsafe { ppcVar1 = (*local_DXAX_37 + 0xc) };
        iVar2 = (**ppcVar1)(0x1010, local_DXAX_37, param_1_00);
        local_8 = local_DXAX_37;
        if (iVar2 == 0) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return iVar2;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_766e(param_1: u32, param_2: u32) {
    let puVar1: *mut u32;
    let struct_a: *mut astruct_199;
    let paVar2: *mut astruct_199;
    let mut local_DX_74: u16;
    let mut uvar3: u16;
    let mut local_SS__1: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    param_2 = 0;
    local_6 = 0;
    puVar1 = &local_6;
    read_file_1008_76e4(param_1, puVar1, local_SS__1);
    if (puVar1 != 0x0) {
        if (local_6 != 0) {
            paVar2 = struct_a;
            process_struct_1000_179c(0xc, struct_a);
            if ((paVar2 | puVar1) == 0) {
                puVar1 = 0x0;
                uVar3 = 0;
            } else {
                pass1_1010_8ef2(CONCAT22(paVar2, puVar1));
                uVar3 = local_DX_74;
            }
            param_2 = puVar1;
            (param_2 + 2) = uVar3;
            pass1_1010_905e(param_2, local_6);
        }
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_7e98(param_1: *mut u16, param_2: u8) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x380a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *param_1 = s_1_1050_389a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Instruction at (ram,0x10087f7b) overlaps instruction at (ram,0x10087f79)
//

pub fn pass1_1008_7f06(
    param_1: u8,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: bool,
    param_6: u8,
    param_7: u8,
    param_8: u8,
    param_9: u8,
) {
    let pbVar1: *mut byte;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut bVar5: bool;
    let mut bVar6: u8;
    let mut bVar7: u8;
    let mut uVar8: u32;
    let paVar9: *mut astruct_65;
    let mut bVar10: u8;
    u16 * *ppuVar11;
    let puVar12: *mut u8;
    let mut bVar13: u8;
    let mut bVar14: u8;
    let mut bVar15: u8;
    let HVar16: HGDIOBJ16;
    let HVar17: HCURSOR16;
    let mut bVar18: u8;
    let mut bVar19: u8;
    let mut uVar20: u16;
    let mut uVar21: u16;
    let mut iVar22: i32;
    u16 * *local_SP;
    u16 * *unaff_BP;
    let mut unaff_SI: i32;
    let local_SI_28: *mut u8;
    let mut unaff_DI: i32;
    let mut local_ES_112: u16;
    let mut local_ES_174: u16;
    let local_ES_214: *mut u8;
    let mut local_SS__1: u16;
    let mut uVar23: i32;
    let mut iStack2251: i32;
    u16 * *ppuStack2249;
    // local_8c7: *mut *mut u8;
    let mut uStack2245: u16;
    let mut uStack2243: u16;
    let mut uStack2241: u16;
    let mut uStack2239: i32;
    let puStack2237: *mut u8;
    let mut auStack2235: [u8; 2201];
    u16 * *ppuStack34;
    let puStack2: *mut u16;
    let temp_179f862c9b4a: *mut u16;
    let temp_5f36b4e76f: *mut u16;
    let mut temp_5f3d48f035: u32;
    let mut temp_5fd4f68048: u32;
    // temp_32379f6118dbc5: *mut *mut u8;
    let temp_36379f6118dbc5: *mut u8;
    let mut char_8: u8;

    while (true) {
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        char_8 = '\x0f';
        temp_179f862c9b4a = unaff_BP;
        while {
            temp_179f862c9b4a = temp_179f862c9b4a + -1;
            local_SP = local_SP + -1;
            unsafe { *local_SP = *temp_179f862c9b4a };
            char_8 = char_8 + -1;
            '\0' < char_8
        } {}
        iStack2251 = param_1;
        pbVar1 = (param_4 + iStack2251);
        bVar18 = param_3;
        unsafe { *bVar1 = *pbVar1 | bVar18 };
        puStack2237 = auStack2235;
        bVar2 = 9 < (unaff_SI & 0xf);
        bVar13 = bVar2 | param_6;
        bVar10 = unaff_SI + bVar13 * '\x06' & 0xf;
        pbVar1 = (param_4 + iStack2251);
        unsafe { *pbVar1 = *pbVar1 | bVar18 };
        local_8c7 = &puStack2237;
        bVar3 = 9 < bVar10;
        bVar6 = bVar3 | bVar13;
        uStack2239 = CONCAT11((unaff_SI >> 8) + bVar13 + bVar6, bVar10 + bVar6 * '\x06') & 0xff0f;
        pbVar1 = (param_4 + iStack2251);
        unsafe { *pbVar1 = *pbVar1 | bVar18 };
        unsafe { bVar13 = *pbVar1 };
        bVar10 = ((POPCOUNT(bVar13) & 1) == 0) * 0x4;
        pbVar1 = (param_4 + iStack2251);
        unsafe { *pbVar1 = *pbVar1 | bVar18 };
        bVar4 = 9 < bVar10;
        bVar6 = bVar4 | bVar6;
        bVar10 = bVar10 + bVar6 * '\x06' & 0xf;
        pbVar1 = (param_4 + iStack2251);
        unsafe { *pbVar1 = *pbVar1 | bVar18 };
        bVar5 = 9 < bVar10;
        bVar7 = bVar5 | bVar6;
        bVar14 = bVar10 + bVar7 * '\x06' & 0xf;
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar18 };
        local_SI_28 = (unaff_DI + -1);
        uVar23 = (param_9 & 1) * 0x4000
            | SBORROW2(unaff_DI, 1) * 0x800
            | (param_8 & 1) * 0x200
            | (param_7 & 1) * 0x100
            | (local_SI_28 < 0) * 0x80
            | (local_SI_28 == 0x0) * 0x40
            | (bVar5 | bVar4 | bVar3 | bVar2 | param_6 & 1) * 0x10
            | ((POPCOUNT(local_SI_28 & 0xff) & 1) == 0) * 4;
        pbVar1 = local_SI_28 + param_4;
        unsafe { *pbVar1 = *pbVar1 | bVar18 };
        pbVar1 = local_SI_28 + param_4;
        unsafe { *pbVar1 = *pbVar1 | bVar18 };
        uVar20 = param_3 - 1;
        pbVar1 = local_SI_28 + param_4;
        bVar19 = uVar20;
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        bVar18 = 9 < bVar14 | bVar7;
        bVar15 = bVar14 + bVar18 * '\x06' & 0xf;
        pbVar1 = local_SI_28 + param_4;
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        pbVar1 = local_SI_28 + param_4;
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        let local_si_28_val = unsafe { *local_SI_28 };
        out(local_si_28_val, uVar20);
        bVar10 = 9 < bVar15 | bVar18;
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        ppuVar11 = local_SP;
        temp_32379f6118dbc5 = &puStack2237;
        uStack2245 = param_4;
        uStack2243 = param_3;
        uStack2241 = param_2;
        puVar12 = auStack2235;
        temp_36379f6118dbc5 = local_SP;
        puStack2 = unaff_BP;
        let pb_var1_val = unsafe { *pbVar1 };
        if (pb_var1_val == 0) {
            // goto LAB_1008_7f73;
        }
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        let pb_var1_val = unsafe { *pbVar1 };
        if ((POPCOUNT(pb_var1_val) & 1) == 0) {
            break;
        }
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        unsafe { bVar13 = *pbVar1 };
        unsafe { bVar6 = *pbVar1 };
        unsafe { bVar7 = *pbVar1 };
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        pbVar1 = (param_4 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        pbVar1 = (&PTR_LOOP_1050_1008 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 ^ param_4 };
        unsafe { bVar18 = *pbVar1 };
        *(&PTR_LOOP_1050_1008 + unaff_DI) = param_4;
        uVar21 = *(&PTR_LOOP_1050_1008 + unaff_DI);
        if (param_2 - 1 == 0 || bVar18 == 0) {
            pbVar1 = (uVar21 + unaff_DI);
            unsafe { *pbVar1 = *pbVar1 | bVar19 };
            pass1_1008_687a(
                CONCAT22(
                    unaff_DI,
                    (param_9 & 1) * 0x4000
                        | (param_8 & 1) * 0x200
                        | (param_7 & 1) * 0x100
                        | (bVar13 < '\0') * 0x80
                        | (bVar6 == 0) * 0x40
                        | ((9 < bVar15)
                            | (9 < bVar14)
                            | bVar5
                            | bVar4
                            | bVar3
                            | bVar2
                            | param_6 & 1)
                            * 0x10
                        | ((POPCOUNT(bVar7) & 1) == 0) * 4,
                ),
                CONCAT22(0x37, uVar20),
            );
            local_SP = &stack0xf721;
            ppuVar11 = local_SP;
            temp_32379f6118dbc5 = local_8c7;
            puVar12 = puStack2237;
            temp_36379f6118dbc5 = local_SP;
            // LAB_1008_7f73:
            local_SP = temp_36379f6118dbc5;
            puStack2237 = puVar12;
            local_8c7 = temp_32379f6118dbc5;
            local_SP = ppuVar11;
            temp_5f36b4e76f = (local_SP + 6);
            local_ES_112 = (temp_5f36b4e76f >> 0x10);
            iVar22 = temp_5f36b4e76f;
            (iVar22 + 0xde) = (local_SP + 10);
            unsafe { *temp_5f36b4e76f = 0x8042 };
            (iVar22 + 2) = &PTR_LOOP_1050_1008;
            copy_string_1000_3d3e(
                (temp_5f36b4e76f & 0xffff0000 | (iVar22 + 0x5b)),
                s_SOLChildPar_1050_0358,
            );
            HVar16 = GetStockObject16(5);
            temp_5f3d48f035 = (local_SP + 6);
            (temp_5f3d48f035 + 0xc6) = HVar16;
            HVar17 = LoadCursor16(0x7f00, 0);
            uVar8 = (local_SP + 6);
            local_ES_174 = (uVar8 >> 0x10);
            iVar22 = uVar8;
            (iVar22 + 0xc4) = HVar17;
            (iVar22 + 200) = (s_572_bmp_1050_2007 + 1);
            (iVar22 + 0xac) = 0x44000000;
            uVar8 = (local_SP + 0xc);
            temp_5fd4f68048 = (local_SP + 6);
            (temp_5fd4f68048 + 0xbc) = (uVar8 + 8);
            paVar9 = (local_SP + 6);
            local_ES_214 = (paVar9 >> 0x10);
            (paVar9 + 0xca) = (paVar9 + 0xde);
            reg_class_1008_96d2(paVar9, uVar23);
            return CONCAT22((local_SP + 8), (local_SP + 6));
        }
        pbVar1 = (uVar21 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        bVar13 = (bVar15 + bVar10 * '\x06' & 0xf) - 1;
        bVar10 = 9 < (bVar13 & 0xf) | bVar10;
        bVar13 = bVar13 + bVar10 * '\x06' & 0xf;
        pbVar1 = (uVar21 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        param_6 = 9 < bVar13 | bVar10;
        param_1 = bVar13 + param_6 * '\x06' & 0xf;
        pbVar1 = (uVar21 + unaff_DI);
        unsafe { *pbVar1 = *pbVar1 | bVar19 };
        param_2 = param_2 - 1;
        param_3 = uVar20;
        param_4 = uVar21;
        unaff_BP = local_SP;
        unaff_SI = unaff_DI;
    }
    cRam105007c7 = cRam105007c7
        + ((param_9 & 1) * '@' | (param_8 & 1) * 0x2 | param_7 & 1 | ((bVar13 < '\0') * 0x80 >> 8))
        + bVar6
        + bVar7
        + bVar18
        + bVar10;
    bRam105047c7 = bRam105047c7 & 2;
    pbVar1 = (unaff_DI + param_4);
    unsafe { *pbVar1 = *pbVar1 | bVar19 + 1 };
    local_SP = local_SP;
    ppuVar11 = local_SP;
    temp_32379f6118dbc5 = &puStack2237;
    puVar12 = auStack2235;
    temp_36379f6118dbc5 = local_SP;
    // goto LAB_1008_7f73;
}

pub fn pass1_1008_7ffa(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    unsafe { *param_1 = 0x380a };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    unsafe { *param_1 = s_1_1050_389a };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1008_8aa2(param_1: *mut u16) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let ppcVar4: fn();
    let mut iVar5: i32;
    let mut local_ES_4: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let temp_8628f43bde0: *mut u32;
    let mut temp_5fc30efdd1: u32;

    local_ES_4 = (param_1 >> 0x10);
    iVar5 = param_1;
    unsafe { *param_1 = 0x8e9a };
    (iVar5 + 2) = &PTR_LOOP_1050_1008;
    temp_5fc30efdd1 = (iVar5 + 4);
    if ((temp_5fc30efdd1 + 0x1c) != 0) {
        puVar1 = (iVar5 + 4);
        uVar2 = (iVar5 + 6);
        if ((uVar2 | puVar1) != 0) {
            unsafe { ppcVar4 = *puVar1 };
            (**ppcVar4)();
        }
    }
    uVar2 = (iVar5 + 0x3a);
    uVar3 = (iVar5 + 0x3c);
    _local_10 = CONCAT22(uVar3, uVar2);
    if ((uVar3 | uVar2) != 0) {
        pass1_1008_5118(CONCAT22(uVar3, uVar2));
        error_check_1000_17ce(_local_10);
    }
    unsafe { *param_1 = s_1_1050_389a };
    (iVar5 + 2) = &PTR_LOOP_1050_1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_8b20(param_1: *mut astruct_206) {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut local_AX_71: u16;
    let mut local_DX_71: u16;
    let local_struct_1: *mut astruct_206;
    let mut local_ES_6: u16;
    let mut local_SS__1: u16;
    let mut local_1c: u32;
    let mut local_c: u16;
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f83cd8e8d: u32;
    let mut uVar3: u32;

    local_ES_6 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0x8 != 0) {
        iVar2 = local_struct_1.field_0x40;
        piVar1 = &local_struct_1.field_0x40;
        unsafe { *piVar1 = *piVar1 + 1 };
        if (iVar2 % local_struct_1.field_0x3e == 0) {
            local_struct_1.field_0x40 = 1;
            uVar3 = local_struct_1.field_0x8;
            pass1_1018_20ee(_PTR_LOOP_1050_0382, uVar3);
            uVar3 = uVar3 & 0xffff | local_DX_71 << 0x10;
            pass1_1008_3e94(
                &local_struct_1.field_0x28,
                CONCAT22(local_SS__1, local_a),
                CONCAT22(local_SS__1, local_8),
            );
            process_struct_1008_8d8a(
                (param_1 & 0xffff | local_ES_6 << 0x10),
                uVar3,
                local_struct_1.field_0x4,
            );
            pass1_1008_4480(
                local_struct_1.field_0x4,
                (param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x28)),
                uVar3,
            );
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_8bc6(param_1: u32) {
    let mut local_AX_42: u16;
    let mut local_DX_42: u16;
    let mut iVar2: i32;
    let mut local_ES_6: u16;
    let mut local_SS__1: u16;
    let mut local_1a: u32;
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f9f6c97b2: u32;
    let mut u_var1: u32;

    local_ES_6 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 8) == 0) {
        return;
    }
    uVar1 = (iVar2 + 8);
    pass1_1018_20ee(_PTR_LOOP_1050_0382, uVar1);
    uVar1 = uVar1 & 0xffff | local_DX_42 << 0x10;
    pass1_1008_3e94(
        (iVar2 + 0x28),
        CONCAT22(local_SS__1, local_a),
        CONCAT22(local_SS__1, local_8),
    );
    process_struct_1008_8d8a((param_1 & 0xffff | local_ES_6 << 0x10), uVar1, (iVar2 + 4));
    pass1_1008_4480(
        (iVar2 + 4),
        (param_1 & 0xffff0000 | ZEXT24((iVar2 + 0x28))),
        uVar1,
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_8c4e(in_struct: *mut astruct_207, param_2: u32) {
    let mut uVar1: i32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let struct_a: *mut astruct_199;
    let paVar4: *mut astruct_199;
    let mut local_DX_97: u16;
    let mut uVar5: u16;
    let struct_1: *mut astruct_207;
    let mut local_ES_4: u16;
    let mut uVar6: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (in_struct >> 0x10);
    struct_1 = in_struct;
    uVar6 = process_struct_1008_4772(struct_1.astruct_104_field_4);
    struct_a = (uVar6 >> 0x10);
    uVar1 = 0;
    if ((struct_1.field_0xc == 0) || (struct_1.field_0xe == 0)) {
        paVar4 = struct_a;
        process_struct_1000_179c(0x14, struct_a);
        _local_e = CONCAT22(paVar4, uVar1);
        if ((paVar4 | uVar1) == 0) {
            uVar2 = 0;
            uVar5 = 0;
        } else {
            uVar3 = in_struct & 0xffff0000 | ZEXT24(struct_1 + 1);
            process_struct_1008_50c2(_local_e, (uVar6 + 8), (uVar6 + 4), uVar3, param_2);
            uVar2 = uVar3;
            uVar5 = local_DX_97;
        }
        pass1_1008_5134(CONCAT22(uVar5, uVar2));
    }
    pass1_1008_4480(
        param_2,
        (in_struct & 0xffff0000 | ZEXT24(struct_1 + 1)),
        struct_1.astruct_104_field_4,
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_8ce4(param_1: *mut astruct_207, param_2: u32, param_3: u32) {
    let local_AX_120: *mut u8;
    let struct_a: *mut astruct_199;
    let paVar1: *mut astruct_199;
    let mut local_DX_113: u16;
    let mut uVar2: u16;
    let local_BX_4: *mut astruct_207;
    let mut local_ES_4: u16;
    let mut local_SS__1: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 6];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_7ffc9379903: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    _local_6 = process_struct_1008_4772(local_BX_4.astruct_104_field_4);
    local_a = 0;
    pass1_1008_3e54(
        CONCAT22(local_SS__1, local_10),
        0,
        &local_BX_4.field_0x12,
        &local_BX_4.field_0x10,
    );
    local_AX_120 = local_10;
    pass1_1008_3f32(param_2, local_AX_120, local_SS__1);
    paVar1 = struct_a;
    process_struct_1000_179c(0x14, struct_a);
    if ((paVar1 | local_AX_120) == 0) {
        local_AX_120 = 0x0;
        uVar2 = 0;
    } else {
        temp_7ffc9379903 = (_local_6 >> 0x10);
        process_struct_1008_50c2(
            CONCAT22(paVar1, local_AX_120),
            (_local_6 + 8),
            (_local_6 + 4),
            param_2,
            param_3,
        );
        uVar2 = local_DX_113;
    }
    local_a = CONCAT22(uVar2, local_AX_120);
    pass1_1008_5134(CONCAT22(uVar2, local_AX_120));
    pass1_1008_4480(param_3, param_2, local_BX_4.astruct_104_field_4);
    return;
}

pub fn pass1_1008_8f24(param_1: *mut astruct_211) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_BX_5: *mut astruct_211;
    let local_BX_59: *mut astruct_212;
    let local_SI_56: *mut astruct_213;
    let mut local_ES_5: u16;
    let mut local_ES_59: u16;
    let mut local_6: u32;
    let mut temp_5f9d841f90: u32;
    let temp_5f5a60757f: *mut u32;
    let fn_ptr_1: fn();

    local_ES_5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1 = 0x9170;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    if (local_BX_5.field_0x1a != 0) {
        local_6 = 0;
        while (true) {
            puVar1 = &local_BX_5.field_0xa;
            let pu_var1_val = unsafe { *puVar1 };
            if (pu_var1_val < local_6 || pu_var1_val == local_6) {
                break;
            }
            local_SI_56 = (local_6 * 4);
            temp_5f9d841f90 = local_BX_5.field_0x6;
            local_ES_59 = (temp_5f9d841f90 >> 0x10);
            local_BX_59 = temp_5f9d841f90;
            temp_5f5a60757f = (local_BX_59 + local_SI_56);
            uVar2 = (local_BX_59 + local_SI_56 + 2);
            if ((uVar2 | temp_5f5a60757f) != 0) {
                unsafe { fn_ptr_1 = *temp_5f5a60757f };
                (**fn_ptr_1)();
            }
            local_6 = local_6 + 1;
        }
    }
    error_check_1000_17ce(local_BX_5.field_0x6);
    param_1 = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_8faa(param_1: u32, param_2: u32) {
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    pass1_1008_9004(
        param_1 & 0xffff | uVar1 << 0x10,
        param_2,
        (param_2 >> 0x10),
        (param_1 + 10),
    );
    return;
}

pub fn bad_func_1008_8fc4(param_1: *mut u8, param_2: u32) {
    let mut local_6: u32;

    return;
}

pub fn pass1_1008_9004(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut local_ES_126: u16;
    let mut bVar5: bool;
    let mut temp_5f0a5228db: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    puVar1 = (iVar3 + 10);
    let pu_var1_val = unsafe { *puVar1 };
    if ((pu_var1_val < param_4 || pu_var1_val == param_4) || ((iVar3 + 6) == 0)) {
        puVar2 = (iVar3 + 0x14);
        unsafe { bVar5 = *puVar2 < param_4._2_2_ };
        let pu_var2_val = unsafe { *puVar2 };
        if ((bVar5 || pu_var2_val == param_4._2_2_)
            && (bVar5
                || (
                    puVar2 = (iVar3 + 0x12),
                    pu_var2_val < param_4 || pu_var2_val == param_4,
                )))
        {
            alloc_mem_1008_909c((param_1 & 0xffff | uVar4 << 0x10));
        }
        puVar1 = (iVar3 + 0x12);
        let pu_var1_val = unsafe { *puVar1 };
        if ((pu_var1_val < param_4 || pu_var1_val == param_4) || ((iVar3 + 6) == 0)) {
            return;
        }
        puVar2 = (iVar3 + 0xc);
        unsafe { bVar5 = *puVar2 < param_4._2_2_ };
        let pu_var2_val = unsafe { *puVar2 };
        if ((bVar5 || pu_var2_val == param_4._2_2_)
            && (bVar5
                || (
                    puVar2 = (iVar3 + 10),
                    pu_var2_val < param_4 || pu_var2_val == param_4,
                )))
        {
            (iVar3 + 10) = (param_4 + 1);
            (iVar3 + 0xc) = (param_4 + 1 >> 0x10);
        }
    }
    temp_5f0a5228db = (iVar3 + 6);
    local_ES_126 = (temp_5f0a5228db >> 0x10);
    iVar3 = temp_5f0a5228db;
    (iVar3 + param_4 * 4) = param_2;
    (iVar3 + param_4 * 4 + 2) = param_3;
    return;
}

pub fn pass1_1008_914a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_8f24(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_92b2(param_1: u32, param_2: libc::c_long, param_3: libc::c_long) {
    let ppcVar1: fn();
    let local_AX_35: *mut u8;
    let mut extraout_DX: i32;
    let mut local_SS__1: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1008_57a4(
        CONCAT22(local_SS__1, local_c),
        param_1 & 0xffff0000 | (param_1 + 6),
    );
    while (true) {
        local_AX_35 = local_c;
        pass1_1008_5b12(CONCAT22(local_SS__1, local_AX_35));
        if ((extraout_DX | local_AX_35) == 0) {
            break;
        }
        if (((local_AX_35 + 4) == param_3) && ((local_AX_35 + 8) == param_2)) {
            local_4 = 1;
            ppcVar1 = ((param_1 + 6) + 0xc);
            (**ppcVar1)();
            local_8 = 0;
        }
    }
    return;
}

pub fn pass1_1008_932a(param_1: u32) {
    let mut uVar1: i32;
    let local_AX_44: *mut u8;
    let mut extraout_DX: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_ES_4: u16;
    let mut local_SS__1: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];
    let fn_ptr_1: fn();

    local_ES_4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 4) == 0) {
        (iVar3 + 4) = 1;
        pass1_1008_57a4(
            CONCAT22(local_SS__1, local_a),
            param_1 & 0xffff0000 | (iVar3 + 6),
        );
        while (true) {
            local_AX_44 = local_a;
            pass1_1008_5b12(CONCAT22(local_SS__1, local_AX_44));
            if ((extraout_DX | local_AX_44) == 0) {
                break;
            }
            uVar1 = (local_AX_44 + 0xc);
            iVar2 = (local_AX_44 + 0xe) - (uVar1 < 0x37);
            (local_AX_44 + 0xc) = uVar1 - 0x37;
            (local_AX_44 + 0xe) = iVar2;
            if ((iVar2 < 1)
                && ((iVar2 < 0 || ((local_AX_44 + 0xc) == 0)) && ((local_AX_44 + 0x10) == 0)))
            {
                fn_ptr_1 = ((local_AX_44 + 4) + 4);
                (**fn_ptr_1)();
                (local_AX_44 + 0xc) = (local_AX_44 + 8);
            }
        }
        (iVar3 + 4) = 0;
    }
    return;
}

pub fn pass1_1008_93c0(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = s_1_1050_389a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_941a(param_1: *mut u16, param_2: u16, param_3: u16) {
    unsafe { *param_1 = param_2 };
    (param_1 + 2) = param_3;
    return param_1;
}

pub fn pass1_1008_9436(param_1: *mut u16) {
    unsafe { *param_1 = 0 };
    (param_1 + 2) = 0;
    return param_1;
}

pub fn pass1_1008_944e(param_1: *mut u16, param_2: u16, param_3: u16) {
    (param_1 + 2) = param_3;
    unsafe { *param_1 = param_2 };
    return;
}

pub fn pass1_1008_9628(param_1: u32, param_2: u16) {
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    if ((param_1 + 8) == 0) {
        (param_1 + 8) = param_2;
    }
    return;
}

pub fn pass1_1008_9c60(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: i32) {
    let fn_ptr_1: fn();

    if ((param_2_00 == 199) && (param_1_00 != 0x0)) {
        unsafe { fn_ptr_1 = *param_1_00 };
        (**fn_ptr_1)();
    }
    return;
}

pub fn pass1_1008_9cc4(param_1: u32, param_2: u16) {
    if ((param_1 + 8) != param_2) {
        return 1;
    }
    return 0;
}

pub fn pass1_1008_9ce0() {
    return 0;
}

pub fn pass1_1008_9d02(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = s_1_1050_389a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_9e5a(param_1: *mut u16) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut local_DX_58: u16;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_862d4416fbd: *mut u32;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    unsafe { *param_1 = 0x9fb2 };
    (iVar5 + 2) = &PTR_LOOP_1050_1008;
    (iVar5 + 0x1c) = 0x9fca;
    (iVar5 + 0x1e) = &PTR_LOOP_1050_1008;
    if (_PTR_LOOP_1050_0388 != 0) {
        if (param_1 == 0x0) {
            iVar4 = 0;
            local_DX_58 = 0;
        } else {
            iVar4 = iVar5 + 0x1c;
            local_DX_58 = uVar6;
        }
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x50, iVar4, local_DX_58);
    }
    local_4 = 0;
    while {
        puVar1 = (iVar5 + 0x22 + local_4 * 4);
        uVar2 = (iVar5 + 0x22 + local_4 * 4 + 2);
        if ((uVar2 | puVar1) != 0) {
            unsafe { ppcVar3 = *puVar1 };
            (**ppcVar3)();
        }
        local_4 = local_4 + 1;
        local_4 < 0xc
    } {}
    if (param_1 == 0x0) {
        iVar5 = 0;
        uVar6 = 0;
    } else {
        iVar5 = iVar5 + 0x1c;
    }
    _local_8 = CONCAT22(uVar6, iVar5);
    unsafe { *_local_8 = s_1_1050_389a };
    (iVar5 + 2) = &PTR_LOOP_1050_1008;
    win_cleanup_1018_4d22(param_1);
    return;
}

pub fn pass1_1008_9f18(param_1: u16, param_2: u16, param_3: u16) {
    if (param_3 == 2) {
        pass1_1008_9f64(CONCAT22(param_2, param_1 - 0x1c));
        pass1_1010_1f62(CONCAT22(param_2, param_1 - 0x1c), 2);
    }
    return;
}

pub fn pass1_1008_9f48(param_1: u32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar2 = (iVar1 + 0x20) * 4;
    return CONCAT22((iVar1 + iVar2 + 0x24), (iVar1 + iVar2 + 0x22));
}

pub fn pass1_1008_9f64(param_1: u32) {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    iVar2 = param_1;
    piVar1 = (iVar2 + 0x20);
    unsafe { *piVar1 = *piVar1 + 1 };
    if (0xb < (iVar2 + 0x20)) {
        (iVar2 + 0x20) = 0;
    }
    return;
}

pub fn pass1_1008_9f8c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_9e5a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_a086(param_1: *mut astruct_376) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.ptr_a_lo = 0xad92;
    (iVar4 + 2) = &PTR_LOOP_1050_1008;
    puVar1 = (iVar4 + 10);
    uVar2 = (iVar4 + 0xc);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    puVar1 = (iVar4 + 0x410);
    uVar2 = (iVar4 + 0x412);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1008_a930(param_1: u32, param_2: u16) {
    let ppcVar1: fn();
    let puVar2: *mut u8;
    let mut extraout_DX: i32;
    let mut uVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar4 + 0x410));
    while {
        puVar2 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar2));
        uVar3 = extraout_DX | puVar2;
        if (uVar3 == 0) {
            process_struct_1000_179c(6, 0x0);
            _local_18 = CONCAT22(uVar3, puVar2);
            if ((uVar3 | puVar2) == 0) {
                local_12 = 0;
            } else {
                unsafe { *_local_18 = s_1_1050_389a };
                (puVar2 + 2) = &PTR_LOOP_1050_1008;
                (puVar2 + 4) = param_2;
                unsafe { *_local_18 = 0xad8a };
                (puVar2 + 2) = &PTR_LOOP_1050_1008;
                local_12 = _local_18;
            }
            ppcVar1 = ((iVar4 + 0x410) + 8);
            (**ppcVar1)(0x1000, (iVar4 + 0x410), local_12);
            return;
        }
        (puVar2 + 4) != param_2
    } {}
    return;
}

pub fn pass1_1008_a9ec(param_1: *mut astruct_218) {
    let mut u_var1: u32;
    let mut in_AX: u16;
    let local_BX_9: *mut astruct_218;
    let mut uVar2: i32;
    let mut local_4: u16;

    local_4 = 0;
    uVar2 = (param_1 >> 0x10);
    local_BX_9 = param_1;
    if ((local_BX_9.field_0x414 == 0) && (uVar1 = local_BX_9.field_0x410, (uVar1 + 8) != 0)) {
        local_BX_9.field_0x414 = 1;
        pass1_1008_aa28((param_1 & 0xffff | uVar2 << 0x10), in_AX);
        local_4 = in_AX;
    }
    return local_4;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_aa28(param_1: *mut astruct_219, param_2: u16) {
    let mut extraout_DX: i32;
    let local_BX_4: *mut astruct_219;
    let mut local_ES_4: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5ffab03f5c: u32;
    let fn_ptr_1: fn();

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x414 != 0) {
        temp_5ffab03f5c = local_BX_4.field_0x410;
        if ((temp_5ffab03f5c + 8) == 0) {
            local_BX_4.field_0x414 = 0;
            return;
        }
        fn_ptr_1 = (local_BX_4.field_0x410 + 0x10);
        (**fn_ptr_1)();
        _local_6 = CONCAT22(extraout_DX, param_2);
        if ((extraout_DX | param_2) != 0) {
            mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, (param_2 + 4));
            if (_local_6 != 0x0) {
                unsafe { fn_ptr_1 = *_local_6 };
                (**fn_ptr_1)();
            }
            return;
        }
    }
    return;
}

pub fn pass1_1008_ab12(param_1: *mut pass1_struct_1, param_2: u16, param_1_00: u8) {
    let in_stack_00000009: u8;
    let mut local_4: u16;

    if (_param_1_00 == 0x37) {
        return 0x22;
    }
    if (_param_1_00 < 0x38) {
        if (param_1_00 == '\r') {
            return 0xf;
        }
        if (param_1_00 == '*') {
            return 0x2b;
        }
    }
    return 0;
}

pub fn pass1_1008_ab54(param_1: u32) {
    let mut local_ES_9: u16;
    let mut local_4: u16;
    let mut temp_5f94569d31: u32;

    local_4 = 0;
    local_ES_9 = (param_1 >> 0x10);
    if (((param_1 + 10) != 0) && (temp_5f94569d31 = (param_1 + 10), (temp_5f94569d31 + 8) != 0)) {
        local_4 = 1;
    }
    return local_4;
}

pub fn pass1_1008_ad38(param_1: u16, param_2: u8) {
    let in_stack_00000007: u8;
    let mut in_stack_00000008: u8;

    _param_1 = CONCAT13(in_stack_00000007, CONCAT12(param_2, param_1));
    _param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        error_check_1000_17ce(_param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}

pub fn pass1_1008_ada2(param_1: *mut u16, param_2: *mut astruct_220) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0 };
    (param_1 + 2) = 0;
    (param_1 + 4) = param_2;
    unsafe { *param_1 = (param_2 * 6 + 0x3a4) };
    return param_1;
}

pub fn pass1_1008_add2(param_1: *mut u16) {
    unsafe { *param_1 = ((param_1 + 4) * 6 + 0x3a4) };
    return;
}

pub fn pass1_1008_adf2(param_1: u32) {
    return ((param_1 + 4) * 6 + 0x3a4);
}

pub fn pass1_1008_ae0c(param_1: u32) {
    return ((param_1 + 4) * 6 + 0x3a6);
}

pub fn pass1_1008_ae26(param_1: *mut i32) {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let local_BX_4: *mut astruct_221;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    iVar2 = (local_BX_4.field_0x4 * 6 + 0x3a8);
    if (iVar2 == 2) {
        if (local_BX_4.field_0x2 == 1) {
            unsafe { *param_1 = *param_1 + -1 };
            iVar2 = local_BX_4.field_0x4 * 6;
            piVar1 = (iVar2 + 0x3a4);
            let pi_var1_val = unsafe { *piVar1 };
            let param_1_val = unsafe { *param_1 };
            if (pi_var1_val != param_1_val && param_1_val <= pi_var1_val) {
                unsafe { *param_1 = (iVar2 + 0x3a4) + 1 };
                local_BX_4.field_0x2 = 0;
                return;
            }
        } else {
            unsafe { *param_1 = *param_1 + 1 };
            let param_1_val = unsafe { *param_1 };
            iVar2 = local_BX_4.field_0x4 * 6;
            if ((iVar2 + 0x3a6) < param_1_val) {
                unsafe { *param_1 = (iVar2 + 0x3a6) + -1 };
                local_BX_4.field_0x2 = 1;
                return;
            }
        }
    } else {
        if ((iVar2 != 3) && (iVar2 != 4)) {
            unsafe { *param_1 = *param_1 + 1 };
            iVar2 = local_BX_4.field_0x4 * 6;
            let param_1_val = unsafe { *param_1 };
            if ((iVar2 + 0x3a6) < param_1_val) {
                unsafe { *param_1 = (iVar2 + 0x3a4) };
            }
        }
    }
    return;
}

pub fn pass1_1008_aed8(param_1: u32) {
    if (((param_1 + 4) * 6 + 0x3a4) != 0) {
        return 1;
    }
    return 0;
}

pub fn pass1_1008_aefe(param_1: *mut u8, param_2: *mut u8, param_3: u16) {
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = 0xaf7c;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1b3);
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1008_af38(param_1: *mut u16) {
    unsafe { *param_1 = 0xaf7c };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    win_cleanup_1018_4d22(param_1);
    return;
}

pub fn pass1_1008_af56(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_af38(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_af94(param_1: *mut astruct_222, param_2: u32) {
    let mut uVar1: u16;

    uVar1 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    param_1.field_0x22 = 0;
    CONCAT22(uVar1, param_1) = 0xbdcc;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_afde(param_1: *mut u16) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    unsafe { *param_1 = 0xbdcc };
    (iVar4 + 2) = &PTR_LOOP_1050_1008;
    puVar1 = (iVar4 + 10);
    uVar2 = (iVar4 + 0xc);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    puVar1 = (iVar4 + 0xe);
    uVar2 = (iVar4 + 0x10);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    puVar1 = (iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1008_b05a(param_1: *mut u16) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    unsafe { *param_1 = s_1_1050_389a };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    (iVar1 + 4) = 0;
    unsafe { *param_1 = 0xbdc8 };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    return param_1;
}

pub fn pass1_1008_b08c(param_1: *mut u16) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    unsafe { *param_1 = 0xbdc8 };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    error_check_1000_17ce((iVar1 + 4));
    unsafe { *param_1 = s_1_1050_389a };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_b0bc(param_1: *mut astruct_199) {
    let local_BX_12: *mut astruct_223;
    let mut uVar1: u16;

    pass1_1008_b05a(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    local_BX_12.field_0x8 = 0;
    local_BX_12.field_0xa = 0;
    local_BX_12.field_0xe = 0;
    local_BX_12.field_0x10 = 0;
    param_1.field_0x0 = 0xbdc4;
    local_BX_12.field_0x2 = &PTR_LOOP_1050_1008;
    return param_1;
}

pub fn pass1_1008_b0f2(param_1: *mut u16) {
    let mut uVar1: u16;

    pass1_1008_b05a(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 8) = 0;
    unsafe { *param_1 = 0xbdc0 };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    return param_1;
}

pub fn pass1_1008_b11e(param_1: *mut u16) {
    let mut uVar1: u16;

    pass1_1008_b05a(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 8) = 0;
    unsafe { *param_1 = 0xbddc };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b146(param_1: *mut astruct_224) {
    let mut u_var1: u32;
    let ppVar2: *mut pass1_struct_2;
    let mut in_DX: u16;
    let local_BX_4: *mut astruct_224;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x16 != 0) {
        uVar1 = local_BX_4.field_0x16;
        ppVar2 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), (uVar1 + 10));
        pass1_1038_3608(CONCAT22(in_DX, ppVar2));
        uVar1 = local_BX_4.field_0x16;
        (uVar1 + 8) = 0;
        uVar1 = local_BX_4.field_0x16;
        (uVar1 + 10) = 0;
        uVar1 = local_BX_4.field_0x16;
        (uVar1 + 0xe) = 0;
        uVar1 = local_BX_4.field_0x16;
        (uVar1 + 0x10) = 0;
    }
    return;
}

pub fn pass1_1008_b1a6(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let uVar2: u8;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uvar3: u16;

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x16) != 0) {
        uVar1 = (iVar4 + 0x16);
        uVar2 = error_check_1000_17ce((uVar1 + 4));
        uVar3 = CONCAT31(extraout_var, uVar2);
        pass1_fn_1008_60e8(param_2);
        uVar1 = (iVar4 + 0x16);
        uVar7 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        (iVar5 + 4) = uVar3;
        (iVar5 + 6) = in_DX;
        (iVar4 + 0x16) = 0;
    }
    return;
}

pub fn pass1_1008_b200(param_1: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let puVar3: *mut u32;
    let paVar4: *mut astruct_199;
    let puVar5: *mut u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let extraout_DX: *mut astruct_199;
    let mut uVar8: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut iVar11: i32;
    let mut uVar12: u16;
    let mut unaff_SS: u16;
    let mut uVar13: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    uVar12 = (param_1 >> 0x10);
    iVar11 = param_1;
    if ((iVar11 + 0xe) != 0) {
        return;
    }
    puVar3 = (iVar11 + 0xe);
    paVar4 = (iVar11 + 0x10);
    if ((paVar4 | puVar3) != 0) {
        unsafe { ppcVar2 = *puVar3 };
        ppcVar2();
        paVar4 = extraout_DX;
    }
    process_struct_1000_179c(0xc, paVar4);
    uVar8 = paVar4 | puVar3;
    if (uVar8 == 0) {
        paVar4 = 0x0;
        uVar8 = 0;
    } else {
        paVar4 = process_struct_1008_574a(CONCAT22(paVar4, puVar3));
    }
    (iVar11 + 0xe) = paVar4;
    (iVar11 + 0x10) = uVar8;
    pass1_1028_dc52(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_14)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        puVar5 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar5));
        _local_18 = CONCAT22(extraout_DX_00, puVar5);
        paVar4 = (extraout_DX_00 | puVar5);
        if (paVar4 == 0x0) {
            break;
        }
        uVar1 = (puVar5 + 2);
        if ((puVar5 + 0x100) == 0x8000001) {
            uVar7 = uVar1;
            process_struct_1000_179c(0xc, paVar4);
            uVar8 = uVar7;
            if ((paVar4 | uVar8) == 0) {
                uVar8 = 0;
                uVar9 = 0;
            } else {
                pass1_1008_b0f2(uVar8, paVar4);
                uVar9 = extraout_DX_01;
            }
            uVar13 = pass1_1038_4d28(_local_18);
            uVar10 = (uVar13 >> 0x10);
            uVar6 = uVar13;
            pass1_fn_1008_60e8();
            (uVar8 + 4) = uVar6;
            (uVar8 + 6) = uVar10;
            (uVar8 + 8) = uVar1;
            uVar1 = (iVar11 + 0xe);
            ppcVar2 = ((iVar11 + 0xe) + 8);
            ppcVar2(0x38, uVar1, (uVar1 >> 0x10));
        }
    }
    return;
}

pub fn pass1_1008_b340(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x16) != 0) {
        uVar1 = (param_1 + 0x16);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 6), (iVar2 + 4));
    }
    return 0;
}

pub fn pass1_1008_b366(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) != 0) {
        uVar1 = (param_1 + 0x1a);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 6), (iVar2 + 4));
    }
    return 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b38c(param_1: u32) {
    let ppcVar1: fn();
    let in_AX: *mut astruct_199;
    let mut iVar2: i32;
    let ppVar3: *mut pass1_struct_2;
    let in_DX: *mut astruct_199;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let lVar9: u32;
    let mut uVar10: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0x12) == 0) {
        process_struct_1000_179c(0xc, in_DX);
        uVar4 = in_DX | in_AX;
        if (uVar4 == 0) {
            (iVar6 + 0x12) = 0;
        } else {
            in_AX = process_struct_1008_574a(CONCAT22(in_DX, in_AX));
            (iVar6 + 0x12) = in_AX;
            (iVar6 + 0x14) = uVar4;
        }
        lVar9 = CONCAT22(uVar4, in_AX);
        local_4 = 0x6d9;
        while (uVar5 = (lVar9 >> 0x10), local_4 < 0x6e7) {
            if (local_4 == 0x6e3) {
                ppVar3 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), 0x8000001);
                lVar9 = CONCAT22(uVar5, ppVar3);
                if (&ppVar3.field_0x136 != 0) {
                    // goto LAB_1008_b44a;
                }
            } else {
                // LAB_1008_b44a:
                process_struct_1000_179c(10, (lVar9 >> 0x10));
                if (lVar9 == 0) {
                    uVar10 = 0;
                } else {
                    uVar10 = pass1_1008_b11e(lVar9);
                }
                uVar5 = (uVar10 >> 0x10);
                iVar2 = uVar10;
                uVar8 = load_str_1010_84ac(
                    _g_struct_73_1050_14cc,
                    (_g_struct_73_1050_14cc >> 0x10),
                    local_4,
                );
                (iVar2 + 4) = uVar8;
                (iVar2 + 6) = (uVar8 >> 0x10);
                (iVar2 + 8) = local_4 - 0x6d8;
                uVar8 = (iVar6 + 0x12);
                ppcVar1 = ((iVar6 + 0x12) + 8);
                lVar9 = (**ppcVar1)(0x1010, uVar8, (uVar8 >> 0x10), uVar10);
            }
            local_4 = local_4 + 1;
        }
    }
    return CONCAT22((iVar6 + 0x14), (iVar6 + 0x12));
}

pub fn pass1_1008_b47a(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x1e) != 0) {
        uVar1 = (param_1 + 0x1e);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 6), (iVar2 + 4));
    }
    return 0;
}

pub fn pass1_1008_b4a0(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;
    let mut in_AX: u16;
    let mut uVar2: u16;
    let mut in_DX: u16;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let lVar7: u32;

    iVar4 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        (iVar4 + 0x16) = 0;
    } else {
        pass1_1008_b9ce(param_1, param_2);
        (iVar4 + 0x16) = in_AX;
        (iVar4 + 0x18) = in_DX;
    }
    uVar1 = (iVar4 + 0x16);
    if ((uVar1 + 8) != 0) {
        pass1_1008_b200(param_1);
        uVar6 = pass1_1008_b38c(param_1);
        uVar3 = (uVar6 >> 0x10);
        uVar2 = uVar6;
        uVar1 = (iVar4 + 0x16);
        pass1_1008_b85c(param_1, (uVar1 + 10));
        (iVar4 + 0x1a) = uVar2;
        (iVar4 + 0x1c) = uVar3;
        uVar1 = (iVar4 + 0x16);
        lVar7 = pass1_1008_b8ac(param_1, (uVar1 + 0xe));
        (iVar4 + 0x1e) = lVar7;
        (iVar4 + 0x20) = (lVar7 >> 0x10);
        return;
    }
    (iVar4 + 0x1a) = 0;
    (iVar4 + 0x1e) = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b544(param_1: u32, param_2: i32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let mut uVar5: u32;
    let ppVar6: *mut pass1_struct_2;
    let mut in_DX: u16;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut unaff_CS: u16;

    iVar7 = param_1;
    uVar8 = (param_1 >> 0x10);
    if (param_2 != 0) {
        if ((iVar7 + 0x1a) != 0) {
            uVar4 = (iVar7 + 0x16);
            (uVar4 + 8) = 1;
            uVar4 = (iVar7 + 0x1a);
            uVar5 = (iVar7 + 0x16);
            (uVar5 + 10) = (uVar4 + 8);
            uVar4 = (iVar7 + 0x1e);
            uVar5 = (iVar7 + 0x16);
            (uVar5 + 0xe) = (uVar4 + 8);
            uVar4 = (iVar7 + 0x16);
            ppVar6 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), (uVar4 + 10));
            unaff_CS = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_3608(CONCAT22(in_DX, ppVar6));
        }
    }
    (iVar7 + 0x1e) = 0;
    (iVar7 + 0x1a) = 0;
    (iVar7 + 0x16) = 0;
    puVar1 = (iVar7 + 0xe);
    uVar2 = (iVar7 + 0x10);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    (iVar7 + 0xe) = 0;
    puVar1 = (iVar7 + 0x12);
    uVar2 = (iVar7 + 0x14);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    (iVar7 + 0x12) = 0;
    return;
}

pub fn pass1_1008_b61a(param_1: u32, param_2: u32) {
    let mut uVar1: u16;
    let mut uVar2: u32;

    uVar2 = pass1_1008_b8fa(param_1, param_2);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x1a) = uVar2;
    (param_1 + 0x1c) = (uVar2 >> 0x10);
    return;
}

pub fn pass1_1008_b63a(param_1: *mut u8, param_2: *mut u8) {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar1: u16;

    pass1_1008_b964(param_1, param_2);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = in_AX;
    (param_1 + 0x20) = in_DX;
    return;
}

pub fn pass1_1008_b820(param_1: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_DX: u16;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar1 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), 0x8000001);
    if (&ppVar1.field_0x152 == 0) {
        return 0;
    }
    uVar2 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xc), (param_1 + 10));
}

pub fn pass1_1008_b85c(param_1: u32, param_2: libc::c_long) {
    let puVar1: *mut u8;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0xe));
    while {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            return;
        }
        (puVar1 + 8) != param_2
    } {}
    return;
}

pub fn pass1_1008_b8ac(param_1: u32, param_2: i32) -> libc::c_long {
    let mut unaff_SS: u16;
    let lVar1: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x12));
    while {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        if (lVar1 == 0) {
            return 0;
        }
        (lVar1 + 8) != param_2
    } {}
    return lVar1;
}

// WARNING: Variable defined which should be unmapped: local_12

pub fn pass1_1008_b8fa(param_1: u32, param_2: u32) {
    let puVar1: *mut u8;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0xe));
    while {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            return;
        }
        pass1_1000_3d7a((puVar1 + 4), param_2, 0);
        puVar1 != 0x0
    } {}
    return;
}

// WARNING: Variable defined which should be unmapped: local_12

pub fn pass1_1008_b964(param_1: u32, param_2: libc::c_long) {
    let puVar1: *mut u8;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x12));
    while {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            return;
        }
        pass1_1000_3d7a((puVar1 + 4), param_2, 0);
        puVar1 != 0x0
    } {}
    return;
}

// WARNING: Variable defined which should be unmapped: local_12

pub fn pass1_1008_b9ce(param_1: u32, param_2: libc::c_long) {
    let puVar1: *mut u8;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 10));
    while {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            return;
        }
        pass1_1000_3d7a((puVar1 + 4), param_2, 0);
        puVar1 != 0x0
    } {}
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_ba38(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let uVar2: u8;
    let BVar3: bool;
    let puVar4: *mut u8;
    let extraout_var: u32;
    let mut extraout_DX: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut local_2a: u32;
    let mut local_1e: u16;
    let mut local_14: [u8; 8];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    uVar2 = write_to_file_1008_7cac(param_2, 0x16);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        uVar6 = (param_1 >> 0x10);
        iVar5 = param_1;
        local_c = (iVar5 + 0x22);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar3 != 0) {
            if ((iVar5 + 10) == 0) {
                local_c = 0;
            } else {
                uVar1 = (iVar5 + 10);
                local_c = (uVar1 + 8);
            }
            local_1e = local_c;
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1e), 2);
            if (BVar3 != 0) {
                pass1_1008_5784(CONCAT22(unaff_SS, local_14), (iVar5 + 10));
                while {
                    puVar4 = local_14;
                    pass1_1008_5b12(CONCAT22(unaff_SS, puVar4));
                    _local_a = CONCAT22(extraout_DX, puVar4);
                    if ((extraout_DX | puVar4) == 0) {
                        return;
                    }
                    uVar1 = (puVar4 + 4);
                    write_to_file_1008_7c2a(param_2, uVar1, (uVar1 >> 0x10));
                    if (puVar4 == 0x0) {
                        break;
                    }
                    local_6 = (_local_a + 8);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_6), 2);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_2a = (_local_a + 10);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_2a), 4);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_6 = (_local_a + 0xe);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_6), 2);
                    BVar3 != 0
                } {}
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn pass1_1008_bd02(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_afde(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_bd28(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_bd4e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_bd74(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_bd9a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// void  pass1_1008_bde0(param_1: u32)
pub fn pass1_1008_bde0(param_1: u32) {
    let mut u_var1: u32;
    let in_DX: *mut u16;
    let local_BX_64: *mut astruct_225;
    let local_BX_92: *mut astruct_226;
    let local_BX_120: *mut astruct_227;
    let local_BX_148: *mut astruct_228;
    let local_BX_176: *mut astruct_229;
    let local_BX_204: *mut astruct_230;
    let local_BX_232: *mut astruct_231;
    let local_BX_260: *mut astruct_232;
    let local_BX_288: *mut astruct_233;
    let local_BX_316: *mut astruct_234;
    let local_BX_344: *mut astruct_235;
    let local_BX_372: *mut astruct_236;
    let local_BX_400: *mut astruct_237;
    let local_BX_428: *mut astruct_238;
    let local_BX_456: *mut astruct_239;
    let local_BX_484: *mut astruct_240;
    let local_BX_512: *mut astruct_241;
    let local_BX_540: *mut astruct_242;
    let local_BX_568: *mut astruct_243;
    let local_BX_596: *mut astruct_244;
    let local_BX_624: *mut astruct_245;
    let local_BX_654: *mut astruct_246;
    let local_BX_685: *mut astruct_247;
    let local_BX_716: *mut astruct_248;
    let local_BX_747: *mut astruct_249;
    let local_BX_778: *mut astruct_250;
    let local_BX_809: *mut astruct_251;
    let local_BX_840: *mut astruct_252;
    let local_BX_871: *mut astruct_253;
    let local_BX_902: *mut astruct_254;
    let local_BX_933: *mut astruct_255;
    let local_BX_964: *mut astruct_256;
    let local_BX_995: *mut astruct_257;
    let local_BX_1026: *mut astruct_258;
    let local_BX_1057: *mut astruct_259;
    let local_BX_1088: *mut astruct_260;
    let local_BX_1119: *mut astruct_261;
    let local_BX_1150: *mut astruct_262;
    let local_BX_1181: *mut astruct_263;
    let local_BX_1212: *mut astruct_264;
    let local_BX_1243: *mut astruct_265;
    let local_BX_1274: *mut astruct_266;
    let local_BX_1305: *mut astruct_267;
    let local_BX_1336: *mut astruct_268;
    let local_BX_1367: *mut astruct_269;
    let local_BX_1398: *mut astruct_270;
    let local_BX_1429: *mut astruct_271;
    let local_BX_1460: *mut astruct_272;
    let local_BX_1491: *mut astruct_273;
    let local_BX_1522: *mut astruct_274;
    let local_BX_1553: *mut astruct_275;
    let local_BX_1584: *mut astruct_276;
    let local_BX_1615: *mut astruct_277;
    let local_BX_1646: *mut astruct_278;
    let local_BX_1677: *mut astruct_279;
    let local_BX_1708: *mut astruct_280;
    let local_BX_1739: *mut astruct_281;
    let local_BX_1770: *mut astruct_282;
    let local_BX_1801: *mut astruct_283;
    let local_BX_1832: *mut astruct_284;
    let local_BX_1863: *mut astruct_285;
    let local_BX_1894: *mut astruct_286;
    let local_BX_1925: *mut astruct_287;
    let local_BX_1956: *mut astruct_288;
    let local_BX_1987: *mut astruct_289;
    let local_BX_2018: *mut astruct_290;
    let local_BX_2049: *mut astruct_291;
    let local_BX_2080: *mut astruct_292;
    let mut uVar2: u16;

    _PTR_LOOP_1050_06e0 = param_1;
    if (__g_astruct_94_ptr_1 == 0) {
        uVar1 = param_1;
        struct_fn_1000_160a();
        _g_astruct_94_ptr_1 = uVar1;
        g_u16_ptr_1050_5f2e = in_DX;
    } else {
    }
    alloc_mem_1000_1708(0x1aa, 0x10000, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
    param_1 = _g_astruct_94_ptr_1;
    (param_1 + 2) = g_u16_ptr_1050_5f2e;
    uVar2 = (*param_1 >> 0x10);
    local_BX_64 = param_1;
    local_BX_64.field_0x6 = 0x6e4;
    local_BX_64.field_0x8 = &g_alloc_addr_1050_1050;
    (param_1 + 10) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_92 = param_1;
    local_BX_92.field_0xc = 0x6ea;
    local_BX_92.field_0xe = &g_alloc_addr_1050_1050;
    (param_1 + 0x10) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_120 = param_1;
    local_BX_120.field_0x12 = 0x6ee;
    local_BX_120.field_0x14 = &g_alloc_addr_1050_1050;
    (param_1 + 0x16) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_148 = param_1;
    local_BX_148.field_0x18 = 0x6f2;
    local_BX_148.field_0x1a = &g_alloc_addr_1050_1050;
    (param_1 + 0x1c) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_176 = param_1;
    local_BX_176.field_0x1e = 0x6f6;
    local_BX_176.field_0x20 = &g_alloc_addr_1050_1050;
    (param_1 + 0x22) = 4;
    uVar2 = (*param_1 >> 0x10);
    local_BX_204 = param_1;
    local_BX_204.field_0x24 = 0x6fe;
    local_BX_204.field_0x26 = &g_alloc_addr_1050_1050;
    (param_1 + 0x28) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_232 = param_1;
    local_BX_232.field_0x2a = 0x702;
    local_BX_232.field_0x2c = &g_alloc_addr_1050_1050;
    (param_1 + 0x2e) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_260 = param_1;
    local_BX_260.field_0x30 = 0x708;
    local_BX_260.field_0x32 = &g_alloc_addr_1050_1050;
    (param_1 + 0x34) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_288 = param_1;
    local_BX_288.field_0x36 = 0x70e;
    local_BX_288.field_0x38 = &g_alloc_addr_1050_1050;
    (param_1 + 0x3a) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_316 = param_1;
    local_BX_316.field_0x3c = 0x714;
    local_BX_316.field_0x3e = &g_alloc_addr_1050_1050;
    (param_1 + 0x40) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_344 = param_1;
    local_BX_344.field_0x42 = 0x71a;
    local_BX_344.field_0x44 = &g_alloc_addr_1050_1050;
    (param_1 + 0x46) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_372 = param_1;
    local_BX_372.field_0x48 = 0x71e;
    local_BX_372.field_0x4a = &g_alloc_addr_1050_1050;
    (param_1 + 0x4c) = 7;
    uVar2 = (*param_1 >> 0x10);
    local_BX_400 = param_1;
    local_BX_400.field_0x4e = 0x72c;
    local_BX_400.field_0x50 = &g_alloc_addr_1050_1050;
    (param_1 + 0x52) = 6;
    uVar2 = (*param_1 >> 0x10);
    local_BX_428 = param_1;
    local_BX_428.field_0x54 = 0x738;
    local_BX_428.field_0x56 = &g_alloc_addr_1050_1050;
    (param_1 + 0x58) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_456 = param_1;
    local_BX_456.field_0x5a = 0x73e;
    local_BX_456.field_0x5c = &g_alloc_addr_1050_1050;
    (param_1 + 0x5e) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_484 = param_1;
    local_BX_484.field_0x60 = 0x744;
    local_BX_484.field_0x62 = &g_alloc_addr_1050_1050;
    (param_1 + 100) = 4;
    uVar2 = (*param_1 >> 0x10);
    local_BX_512 = param_1;
    local_BX_512.field_0x66 = 0x74c;
    local_BX_512.field_0x68 = &g_alloc_addr_1050_1050;
    (param_1 + 0x6a) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_540 = param_1;
    local_BX_540.field_0x6c = 0x750;
    local_BX_540.field_0x6e = &g_alloc_addr_1050_1050;
    (param_1 + 0x70) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_568 = param_1;
    local_BX_568.field_0x72 = 0x756;
    local_BX_568.field_0x74 = &g_alloc_addr_1050_1050;
    (param_1 + 0x76) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_596 = param_1;
    local_BX_596.field_0x78 = 0x75a;
    local_BX_596.field_0x7a = &g_alloc_addr_1050_1050;
    (param_1 + 0x7c) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_624 = param_1;
    local_BX_624.field_0x7e = 0x75e;
    local_BX_624.field_0x80 = &g_alloc_addr_1050_1050;
    (param_1 + 0x82) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_654 = param_1;
    local_BX_654.field_0x84 = 0x764;
    local_BX_654.field_0x86 = &g_alloc_addr_1050_1050;
    (param_1 + 0x88) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_685 = param_1;
    local_BX_685.field_0x8a = 0x76a;
    local_BX_685.field_0x8c = &g_alloc_addr_1050_1050;
    (param_1 + 0x8e) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_716 = param_1;
    local_BX_716.field_0x90 = 0x770;
    local_BX_716.field_0x92 = &g_alloc_addr_1050_1050;
    (param_1 + 0x94) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_747 = param_1;
    local_BX_747.field_0x96 = 0x774;
    local_BX_747.field_0x98 = &g_alloc_addr_1050_1050;
    (param_1 + 0x9a) = 4;
    uVar2 = (*param_1 >> 0x10);
    local_BX_778 = param_1;
    local_BX_778.field_0x9c = 0x77c;
    local_BX_778.field_0x9e = &g_alloc_addr_1050_1050;
    (param_1 + 0xa0) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_809 = param_1;
    local_BX_809.field_0xa2 = 0x780;
    local_BX_809.field_0xa4 = &g_alloc_addr_1050_1050;
    (param_1 + 0xa6) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_840 = param_1;
    local_BX_840.field_0xa8 = 0x782;
    local_BX_840.field_0xaa = &g_alloc_addr_1050_1050;
    (param_1 + 0xac) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_871 = param_1;
    local_BX_871.field_0xae = 0x786;
    local_BX_871.field_0xb0 = &g_alloc_addr_1050_1050;
    (param_1 + 0xb2) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_902 = param_1;
    local_BX_902.field_0xb4 = 0x78a;
    local_BX_902.field_0xb6 = &g_alloc_addr_1050_1050;
    (param_1 + 0xb8) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_933 = param_1;
    local_BX_933.field_0xba = 0x78e;
    local_BX_933.field_0xbc = &g_alloc_addr_1050_1050;
    (param_1 + 0xbe) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_964 = param_1;
    local_BX_964.field_0xc0 = 0x792;
    local_BX_964.field_0xc2 = &g_alloc_addr_1050_1050;
    (param_1 + 0xc4) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_995 = param_1;
    local_BX_995.field_0xc6 = 0x796;
    local_BX_995.field_0xc8 = &g_alloc_addr_1050_1050;
    (param_1 + 0xca) = 4;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1026 = param_1;
    local_BX_1026.field_0xcc = 0x79e;
    local_BX_1026.field_0xce = &g_alloc_addr_1050_1050;
    (param_1 + 0xd0) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1057 = param_1;
    local_BX_1057.field_0xd2 = 0x7a0;
    local_BX_1057.field_0xd4 = &g_alloc_addr_1050_1050;
    (param_1 + 0xd6) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1088 = param_1;
    local_BX_1088.field_0xd8 = 0x7a4;
    local_BX_1088.field_0xda = &g_alloc_addr_1050_1050;
    (param_1 + 0xdc) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1119 = param_1;
    local_BX_1119.field_0xde = 0x7a6;
    local_BX_1119.field_0xe0 = &g_alloc_addr_1050_1050;
    (param_1 + 0xe2) = 6;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1150 = param_1;
    local_BX_1150.field_0xe4 = 0x7b2;
    local_BX_1150.field_0xe6 = &g_alloc_addr_1050_1050;
    (param_1 + 0xe8) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1181 = param_1;
    local_BX_1181.field_0xea = 0x7b4;
    local_BX_1181.field_0xec = &g_alloc_addr_1050_1050;
    (param_1 + 0xee) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1212 = param_1;
    local_BX_1212.field_0xf0 = 0x7ba;
    local_BX_1212.field_0xf2 = &g_alloc_addr_1050_1050;
    (param_1 + 0xf4) = 0x2d;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1243 = param_1;
    local_BX_1243.field_0xf6 = 0x814;
    local_BX_1243.field_0xf8 = &g_alloc_addr_1050_1050;
    (param_1 + 0xfa) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1274 = param_1;
    local_BX_1274.field_0xfc = 0x81a;
    local_BX_1274.field_0xfe = &g_alloc_addr_1050_1050;
    (param_1 + 0x100) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1305 = param_1;
    local_BX_1305.field_0x102 = 0x81c;
    local_BX_1305.field_0x104 = &g_alloc_addr_1050_1050;
    (param_1 + 0x106) = 0x4b;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1336 = param_1;
    local_BX_1336.field_0x108 = 0x8b2;
    local_BX_1336.field_0x10a = &g_alloc_addr_1050_1050;
    (param_1 + 0x10c) = 6;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1367 = param_1;
    local_BX_1367.field_0x10e = 0x8be;
    local_BX_1367.field_0x110 = &g_alloc_addr_1050_1050;
    (param_1 + 0x112) = 4;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1398 = param_1;
    local_BX_1398.field_0x11a = 0x8c6;
    local_BX_1398.field_0x11c = &g_alloc_addr_1050_1050;
    (param_1 + 0x11e) = 0x35;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1429 = param_1;
    local_BX_1429.field_0x120 = 0x930;
    local_BX_1429.field_0x122 = &g_alloc_addr_1050_1050;
    (param_1 + 0x124) = 0x2e;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1460 = param_1;
    local_BX_1460.field_0x114 = 0x98c;
    local_BX_1460.field_0x116 = &g_alloc_addr_1050_1050;
    (param_1 + 0x118) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1491 = param_1;
    local_BX_1491.field_0x126 = 0x98e;
    local_BX_1491.field_0x128 = &g_alloc_addr_1050_1050;
    (param_1 + 0x12a) = 9;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1522 = param_1;
    local_BX_1522.field_0x12c = 0x9a0;
    local_BX_1522.field_0x12e = &g_alloc_addr_1050_1050;
    (param_1 + 0x130) = 0x1a;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1553 = param_1;
    local_BX_1553.field_0x132 = 0x9d4;
    local_BX_1553.field_0x134 = &g_alloc_addr_1050_1050;
    (param_1 + 0x136) = 8;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1584 = param_1;
    local_BX_1584.field_0x138 = 0x9e4;
    local_BX_1584.field_0x13a = &g_alloc_addr_1050_1050;
    (param_1 + 0x13c) = 0x4a;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1615 = param_1;
    local_BX_1615.field_0x144 = 0xa78;
    local_BX_1615.field_0x146 = &g_alloc_addr_1050_1050;
    (param_1 + 0x148) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1646 = param_1;
    local_BX_1646.field_0x14a = 0xa7c;
    local_BX_1646.field_0x14c = &g_alloc_addr_1050_1050;
    (param_1 + 0x14e) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1677 = param_1;
    local_BX_1677.field_0x150 = 0xa7e;
    local_BX_1677.field_0x152 = &g_alloc_addr_1050_1050;
    (param_1 + 0x154) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1708 = param_1;
    local_BX_1708.field_0x156 = 0xa80;
    local_BX_1708.field_0x158 = &g_alloc_addr_1050_1050;
    (param_1 + 0x15a) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1739 = param_1;
    local_BX_1739.field_0x15c = 0xa86;
    local_BX_1739.field_0x15e = &g_alloc_addr_1050_1050;
    (param_1 + 0x160) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1770 = param_1;
    local_BX_1770.field_0x168 = 0xa8a;
    local_BX_1770.field_0x16a = &g_alloc_addr_1050_1050;
    (param_1 + 0x16c) = 0x1b;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1801 = param_1;
    local_BX_1801.field_0x16e = 0xac0;
    local_BX_1801.field_0x170 = &g_alloc_addr_1050_1050;
    (param_1 + 0x172) = 0x16;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1832 = param_1;
    local_BX_1832.field_0x174 = 0xaec;
    local_BX_1832.field_0x176 = &g_alloc_addr_1050_1050;
    (param_1 + 0x178) = 0x3e;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1863 = param_1;
    local_BX_1863.field_0x17a = 0xb68;
    local_BX_1863.field_0x17c = &g_alloc_addr_1050_1050;
    (param_1 + 0x17e) = 0x46;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1894 = param_1;
    local_BX_1894.field_0x180 = 0xbf4;
    local_BX_1894.field_0x182 = &g_alloc_addr_1050_1050;
    (param_1 + 0x184) = 1;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1925 = param_1;
    local_BX_1925.field_0x186 = 0xbf6;
    local_BX_1925.field_0x188 = &g_alloc_addr_1050_1050;
    (param_1 + 0x18a) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1956 = param_1;
    local_BX_1956.field_0x18c = 0xbfc;
    local_BX_1956.field_0x18e = &g_alloc_addr_1050_1050;
    (param_1 + 400) = 3;
    uVar2 = (*param_1 >> 0x10);
    local_BX_1987 = param_1;
    local_BX_1987.field_0x192 = 0xc02;
    local_BX_1987.field_0x194 = &g_alloc_addr_1050_1050;
    (param_1 + 0x196) = 10;
    uVar2 = (*param_1 >> 0x10);
    local_BX_2018 = param_1;
    local_BX_2018.field_0x198 = 0xc16;
    local_BX_2018.field_0x19a = &g_alloc_addr_1050_1050;
    (param_1 + 0x19c) = 0x24;
    uVar2 = (*param_1 >> 0x10);
    local_BX_2049 = param_1;
    local_BX_2049.field_0x19e = 0xc5e;
    local_BX_2049.field_0x1a0 = &g_alloc_addr_1050_1050;
    (param_1 + 0x1a2) = 2;
    uVar2 = (*param_1 >> 0x10);
    local_BX_2080 = param_1;
    local_BX_2080.field_0x1a4 = 0xc62;
    local_BX_2080.field_0x1a6 = &g_alloc_addr_1050_1050;
    (param_1 + 0x1a8) = 0x44;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_c626(param_1: *mut u32) {
    _PTR_LOOP_1050_06e0 = 0;
    let param_1_val = unsafe { *param_1 };
    error_check_1000_17ce(param_1_val);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_c646(param_1: u16, param_2: u32) -> i32 {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let puVar4: *mut u8;
    let mut in_DX: i32;
    let mut unaff_SI: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    puVar4 = pass1_1008_c6fa(CONCAT22(param_2, param_1), (param_2 >> 0x10));
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x35));
    local_12 = 0;
    loop {
        piVar1 = (puVar4 + 4);
        let pi_var1_val = unsafe { *piVar1 };
        if (pi_var1_val == local_12 || pi_var1_val < local_12) {
            // LAB_1008_c6a5:
            return local_12._2_2_;
        }
        uVar3 = (puVar4 & 0xffff | in_DX << 0x10);
        uVar2 = (uVar3 + local_12 * 2);
        if ((uVar2 * 2 + ppVar5 + 10) != 0) {
            local_12 = uVar2 << 0x10;
            // goto LAB_1008_c6a5;
        }
        local_12 = (local_12 + 1);
    }
}

pub fn pass1_1008_c6ae(param_1: u32, param_2: i32, param_3: u16) -> bool {
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let puVar3: *mut u8;
    let mut in_DX: i32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    puVar3 = pass1_1008_c6fa(param_1, param_3);
    local_8 = 0;
    while (true) {
        puVar1 = (puVar3 + 4);
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val == local_8 || pu_var1_val < local_8) {
            return 0;
        }
        uVar2 = (puVar3 & 0xffff | in_DX << 0x10);
        if ((uVar2 + local_8 * 2) == param_2) {
            break;
        }
        local_8 = local_8 + 1;
    }
    return 1;
}

pub fn pass1_1008_c6fa(param_1: *mut u8, param_2: u16) {
    let mut in_EAX: u32;

    if ((0 < param_2) && (param_2 < 0x47)) {
        return (in_EAX & 0xffff0000 | (param_2 * 6 + param_1));
    }
    return (in_EAX & 0xffff0000);
}

pub fn pass1_1008_c72a(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    (param_1 + 0xe) = 0;
    CONCAT22(param_2, param_1) = 0xca4a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_c75c(param_1: *mut astruct_293) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut astruct_293;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1 = 0xca4a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    puVar1 = local_BX_5.field_0xa;
    uVar2 = local_BX_5.field_0xc;
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_c79a(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let puVar2: *mut u8;
    let mut uVar3: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: [u8; 4];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    uVar5 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 10));
    while (true) {
        puVar2 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar2));
        _local_e = CONCAT22(extraout_DX, puVar2);
        if ((extraout_DX | puVar2) == 0) {
            break;
        }
        uVar1 = (puVar2 + 4);
        pass1_1000_3d7a(uVar1, (uVar1 >> 0x10), param_2);
        if (puVar2 == 0x0) {
            pass1_1020_a43e(CONCAT22(unaff_SS, local_12));
            pass1_1020_a6ee(CONCAT22(unaff_SS, local_12), (_local_e + 0x12));
            uVar3 = (_PTR_LOOP_1050_65e2 + 0x52);
            uVar4 = extraout_DX_00;
            pass1_1030_4bbe(uVar3, (_local_e + 0x12));
            (param_1 + 0xe) = (uVar3 + 0x94) + *_PTR_LOOP_1050_65e2;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_c83a(param_1: u32) {
    if (*_PTR_LOOP_1050_65e2 <= (param_1 + 0xe)) {
        return;
    }
    return;
}

pub fn pass1_1008_c85e(param_1: u32) {
    let local_BX_3: *mut astruct_294;
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (&local_BX_3.field_0xa == 0) {
        process_struct_1008_c882(param_1 & 0xffff | uVar1 << 0x10);
    }
    return CONCAT22(local_BX_3.field_0xc, local_BX_3.field_0xa);
}

pub fn pass1_1008_ca24(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_c75c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_ca5a(param_1: *mut astruct_295, param_2: u32) {
    let mut uVar1: u16;

    uVar1 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    CONCAT22(uVar1, param_1) = 0xd71a;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_caa0(param_1: *mut u16) {
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0xd71a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    pass1_1008_cac6(param_1 & 0xffff | uVar1 << 0x10);
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1008_cac6(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 10);
    uVar2 = (iVar4 + 0xc);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    (iVar4 + 10) = 0;
    puVar1 = (iVar4 + 0xe);
    uVar2 = (iVar4 + 0x10);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    (iVar4 + 0xe) = 0;
    puVar1 = (iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    (iVar4 + 0x12) = 0;
    puVar1 = (iVar4 + 0x16);
    uVar2 = (iVar4 + 0x18);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    (iVar4 + 0x16) = 0;
    puVar1 = (iVar4 + 0x1a);
    uVar2 = (iVar4 + 0x1c);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    (iVar4 + 0x1a) = 0;
    puVar1 = (iVar4 + 0x1e);
    uVar2 = (iVar4 + 0x20);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    (iVar4 + 0x1e) = 0;
    return;
}

pub fn pass1_1008_cfa0(param_1: *mut astruct_298, param_2: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut bVar3: bool;
    let puVar4: *mut u32;
    let paVar5: *mut astruct_199;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let mut uVar8: u32;
    let extraout_DX: *mut astruct_199;
    let mut uVar9: i32;
    let mut extraout_DX_00: i32;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let struct_a_00: *mut astruct_199;
    let mut extraout_DX_03: u16;
    let mut uVar10: u16;
    let mut extraout_DX_04: i32;
    let struct_a_01: *mut astruct_199;
    let mut extraout_DX_05: u16;
    let struct_a_02: *mut astruct_199;
    let mut extraout_DX_06: u16;
    let mut uVar11: u16;
    let mut iVar12: i32;
    let mut uVar13: u16;
    let uVar14: u8;
    let mut uVar15: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar13 = (param_1 >> 0x10);
    iVar12 = param_1;
    puVar4 = (iVar12 + 0x16);
    paVar5 = (iVar12 + 0x18);
    if ((paVar5 | puVar4) != 0) {
        unsafe { ppcVar2 = *puVar4 };
        ppcVar2();
        paVar5 = extraout_DX;
    }
    process_struct_1000_179c(0xc, paVar5);
    uVar9 = paVar5 | puVar4;
    if (uVar9 == 0) {
        paVar5 = 0x0;
        uVar9 = 0;
    } else {
        paVar5 = process_struct_1008_574a(CONCAT22(paVar5, puVar4));
    }
    (iVar12 + 0x16) = paVar5;
    (iVar12 + 0x18) = uVar9;
    bVar3 = false;
    uVar8 = (param_2 + 0x1f6);
    uVar11 = uVar8;
    uVar15 = (uVar8 >> 0x10);
    pass1_1030_38f2(uVar11, uVar15, 2);
    uVar9 = uVar8;
    if ((-1 < extraout_DX_00) && (0 < extraout_DX_00 || (uVar9 != 0))) {
        uVar7 = uVar9;
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x430,
        );
        uVar14 = 0;
        paVar5 = struct_a;
        uVar9 = uVar7;
        process_struct_1000_179c(0x14, struct_a);
        if ((paVar5 | uVar9) == 0) {
            uVar9 = 0;
            uVar10 = 0;
        } else {
            uVar14 = 0x18;
            pass1_1018_4842(
                CONCAT22(paVar5, uVar9),
                uVar8 & 0xffff | extraout_DX_00 << 0x10,
                CONCAT22(struct_a, uVar7),
                2,
            );
            uVar10 = extraout_DX_01;
        }
        uVar1 = (iVar12 + 0x16);
        ppcVar2 = ((iVar12 + 0x16) + 4);
        ppcVar2(uVar14, uVar1, (uVar1 >> 0x10), uVar9, uVar10);
        bVar3 = true;
    }
    pass1_1030_38f2(uVar11, uVar15, 3);
    if ((-1 < extraout_DX_02) && (0 < extraout_DX_02 || (uVar9 != 0))) {
        uVar6 = uVar9;
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x431,
        );
        uVar14 = 0;
        paVar5 = struct_a_00;
        uVar7 = uVar6;
        process_struct_1000_179c(0x14, struct_a_00);
        if ((paVar5 | uVar7) == 0) {
            uVar9 = 0;
            uVar10 = 0;
        } else {
            uVar14 = 0x18;
            uVar8 = CONCAT22(extraout_DX_02, uVar9);
            uVar9 = uVar7;
            pass1_1018_4842(
                CONCAT22(paVar5, uVar7),
                uVar8,
                CONCAT22(struct_a_00, uVar6),
                3,
            );
            uVar10 = extraout_DX_03;
        }
        uVar1 = (iVar12 + 0x16);
        ppcVar2 = ((iVar12 + 0x16) + 4);
        ppcVar2(uVar14, uVar1, (uVar1 >> 0x10), uVar9, uVar10);
        bVar3 = true;
    }
    pass1_1030_38f2(uVar11, uVar15, 4);
    if ((-1 < extraout_DX_04) && (0 < extraout_DX_04 || (uVar9 != 0))) {
        uVar6 = uVar9;
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x432,
        );
        uVar14 = 0;
        paVar5 = struct_a_01;
        uVar7 = uVar6;
        process_struct_1000_179c(0x14, struct_a_01);
        if ((paVar5 | uVar7) == 0) {
            uVar9 = 0;
            uVar11 = 0;
        } else {
            uVar14 = 0x18;
            uVar8 = CONCAT22(extraout_DX_04, uVar9);
            uVar9 = uVar7;
            pass1_1018_4842(
                CONCAT22(paVar5, uVar7),
                uVar8,
                CONCAT22(struct_a_01, uVar6),
                4,
            );
            uVar11 = extraout_DX_05;
        }
        uVar1 = (iVar12 + 0x16);
        ppcVar2 = ((iVar12 + 0x16) + 4);
        ppcVar2(uVar14, uVar1, (uVar1 >> 0x10), uVar9, uVar11);
        bVar3 = true;
    }
    if (!bVar3) {
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x440,
        );
        uVar14 = 0;
        paVar5 = struct_a_02;
        uVar7 = uVar9;
        process_struct_1000_179c(0x14, struct_a_02);
        if ((paVar5 | uVar7) == 0) {
            uVar7 = 0;
            uVar11 = 0;
        } else {
            uVar14 = 0x18;
            pass1_1018_4842(CONCAT22(paVar5, uVar7), 0, CONCAT22(struct_a_02, uVar9), 0);
            uVar11 = extraout_DX_06;
        }
        uVar1 = (iVar12 + 0x16);
        ppcVar2 = ((iVar12 + 0x16) + 4);
        ppcVar2(uVar14, uVar1, (uVar1 >> 0x10), uVar7, uVar11);
    }
    return;
}

pub fn pass1_1008_d6f4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_caa0(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_d72e(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    CONCAT22(param_2, param_1) = 0xd780;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1008_d75a(param_1: u32, param_2: u8) {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Variable defined which should be unmapped: local_6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_d790(in_struct_1: *mut astruct_299, param_2: u32) {
    let mut local_AX_18: u16;
    let mut local_DX_49: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    local_6 = param_2;
    uStack4 = (param_2 >> 0x10);
    process_struct_1010_1d48(CONCAT22(local_6, in_struct_1), uStack4);
    local_AX_18 = 0;
    &in_struct_1.field_0xa = 0;
    in_struct_1.field_0xe = 0;
    CONCAT22(local_6, in_struct_1) = 0xd98e;
    in_struct_1.field_0x2 = &PTR_LOOP_1050_1008;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x9b);
    in_struct_1.field_0xa = local_AX_18;
    in_struct_1.field_0xc = local_DX_49;
    return;
}

pub fn pass1_1008_d7da(param_1: *mut u16) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    unsafe { *param_1 = 0xd98e };
    (iVar4 + 2) = &PTR_LOOP_1050_1008;
    puVar1 = (iVar4 + 10);
    uVar2 = (iVar4 + 0xc);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1008_d968(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_d7da(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_d99e(param_1: *mut astruct_534, param_2: u16, param_3: u16) {
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = 0xd9fa;
    param_1.u16_x02 = &PTR_LOOP_1050_1008;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x9a);
    _PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

pub fn pass1_1008_d9d4(param_1: u32, param_2: u8) {
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_dc2c(param_1: *mut u16) {
    let mut local_ES_4: u16;
    let mut temp_5f04f790ee: u32;

    local_ES_4 = (param_1 >> 0x10);
    unsafe { *param_1 = 0xdc80 };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    error_check_1000_17ce((param_1 + 0x18));
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1008_dc5a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1008_dc2c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_dc90(in_struct_1: *mut astruct_301, param_2: u32, param_3: u32) {
    let local_BX_4: *mut astruct_301;
    let mut uVar1: u16;

    uVar1 = (in_struct_1 >> 0x10);
    local_BX_4 = in_struct_1;
    in_struct_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = param_3;
    local_BX_4.field_0x8 = param_2;
    local_BX_4.field_0xc = 0;
    local_BX_4.field_0xe = 0;
    local_BX_4.field_0x12 = 0;
    in_struct_1 = 0xdd4a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1008_dd1e(param_1: *mut u16, param_2: u8) {
    unsafe { *param_1 = s_1_1050_389a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_14f0_01b8(param_1: u8, param_2: i32) {
    let mut cVar1: u8;
    let mut in_CL: u8;
    let local_SP: *mut u16;
    let local_BP__1: *mut u16;
    let unaff_SI: *mut char;
    let mut local_SS__1: u16;
    let mut local_DS__1: u16;
    let string_1: *mut char;

    local_SP = &stack0xfffe;
    cVar1 = '\x1e';
    while {
        local_BP__1 = local_BP__1 + -1;
        local_SP = local_SP + -1;
        unsafe { *local_SP = *local_BP__1 };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    string_1 = unaff_SI + param_2;
    unsafe { *string_1 = *string_1 - in_CL };
    string_1 = unaff_SI + param_2;
    unsafe { *string_1 = *string_1 + param_1 };
    string_1 = unaff_SI + param_2;
    unsafe { *string_1 = *string_1 + param_1 };
    string_1 = unaff_SI;
    unsafe { *string_1 = *string_1 + in_CL };
    return;
}

pub fn pass1_11a0_03fl4(param_1: *mut char, param_2: u16, param_3: *mut char) {
    let piVar1: *mut i32;
    let puVar2: *mut u8;
    let pbVar3: *mut byte;
    let mut bVar4: u8;
    let mut bVar5: u8;
    let mut uVar6: i32;
    let mut cVar7: u8;
    let mut bVar8: u8;
    let string_2: *mut char;
    let mut cVar9: u8;
    let string_5: *mut char;
    let string_6: *mut char;
    let local_BP_188: *mut u8;
    let string_4: *mut char;
    let string_1: *mut char;
    let mut local_SS__1: u16;
    let mut local_DS__1: u16;
    let mut bVar10: bool;
    let in_stack_000003fa: *mut char;
    let in_stack_000003fc: *mut char;
    let mut in_stack_000003fe: u16;
    let mut in_stack_00000400: i32;
    let local_76ef: u8;
    let mut temp_13fb229f94a: u16;
    let mut temp_5f472f0e69: u16;
    let bytes_2: *mut byte;
    let string_3: *mut char;

    bytes_2 = (param_3 + string_4);
    bVar4 = param_1;
    unsafe { *bytes_2 = *bytes_2 | bVar4 };
    string_3 = param_3;
    unsafe { *string_3 = *string_3 + 0x2 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + bVar4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    local_BP_188 = &stack0x86fa;
    bytes_2 = (param_3 + string_4);
    unsafe { *bytes_2 = *bytes_2 | bVar4 };
    string_3 = param_3;
    unsafe { *string_3 = *string_3 + 0x2 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + bVar4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    param_3[string_4] = -0x5e;
    string_3 = string_4;
    cVar9 = (param_2 >> 8);
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    cVar7 = param_2;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = (param_3 + string_1 + 0x217);
    unsafe { *string_3 = *string_3 + bVar4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + bVar4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    param_3[string_4] = -0x51;
    string_3 = string_4;
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + cVar7 };
    puVar2 = &local_76ef + string_1;
    unsafe { *puVar2 = *puVar2 + bVar4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + bVar4 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 };
    temp_5f472f0e69 = (param_3 + string_4);
    string_3 = param_3 + string_1;
    cVar9 = (param_3 >> 8);
    unsafe { *string_3 = *string_3 + cVar9 };
    string_3 = param_3 + string_4;
    unsafe { *string_3 = *string_3 + cVar7 };
    piVar1 = (&stack0x86fa + string_4);
    unsafe { *piVar1 = *piVar1 + 1 };
    bytes_2 = &stack0x86fa + string_4;
    unsafe { bVar5 = *bytes_2 };
    unsafe { *bytes_2 = *bytes_2 + bVar4 };
    string_3 = &stack0x875b + string_4;
    unsafe { *string_3 = *string_3 + 'w' + CARRY1(bVar5, bVar4) };
    bytes_2 = (string_4 + 0x6f);
    unsafe { *bytes_2 = *bytes_2 & bVar4 };
    string_2 = param_1;
    string_5 = param_1;
    string_6 = (param_3 & 0xff | (cVar9 * 0x2) << 8);
    let bytes_2_val = unsafe { *bytes_2 };
    if (bytes_2_val != 0) {
        cVar7 = _in(param_1);
        unsafe { *string_1 = cVar7 };
        temp_13fb229f94a = _in(param_1);
        (string_1 + 1) = temp_13fb229f94a;
        local_BP_188 = ((param_1 + 0x69) * 0x676e);
        string_3 = in_stack_000003fa;
        unsafe { *string_3 = *string_3 + in_stack_00000400 };
        uVar6 = in_stack_00000400 & (in_stack_000003fa + param_1);
        bVar5 = uVar6 ^ in_stack_000003fa[param_1];
        string_2 = (uVar6 & 0xff00 | bVar5);
        bytes_2 = (in_stack_000003fa + param_1);
        unsafe { *bytes_2 = *bytes_2 | bVar5 };
        string_3 = param_1 + -0x3a00;
        unsafe { *string_3 = *string_3 + bVar5 };
        puVar2 = local_BP_188 + param_1;
        unsafe { *puVar2 = *puVar2 + (in_stack_000003fc >> 8) };
        param_2 = in_stack_000003fe;
        string_5 = in_stack_000003fc;
        string_6 = in_stack_000003fa;
        string_4 = param_1;
        string_1 = param_1;
    }
    string_3 = (local_BP_188 + string_1 + 0x217);
    unsafe { *string_3 = *string_3 + (string_6 >> 8) };
    puVar2 = local_BP_188 + string_4;
    bVar5 = string_2;
    unsafe { *puVar2 = *puVar2 + bVar5 };
    string_3 = string_6 + string_4;
    unsafe { *string_3 = *string_3 };
    string_6[string_4] = '2';
    string_3 = 0x100;
    cVar7 = param_2;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = string_6 + string_1;
    unsafe { *string_3 = *string_3 + bVar5 };
    string_3 = string_6 + string_1;
    unsafe { *string_3 = *string_3 + bVar5 };
    if (string_5 != 0xffff) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    if (string_5 == 0xffff) {
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    string_5 = string_4 + 1;
    let string_4_val = unsafe { *string_4 };
    out(string_4_val, 0);
    string_3 = string_6 + 0x6b;
    unsafe { *string_3 = *string_3 + cVar7 };
    string_3 = string_6 + string_5;
    unsafe { *string_3 = *string_3 + bVar5 };
    bVar8 = cVar7 - 1;
    puVar2 = local_BP_188 + string_5;
    unsafe { *puVar2 = *puVar2 + bVar5 };
    bytes_2 = 0x3000;
    unsafe { bVar4 = *bytes_2 };
    unsafe { *bytes_2 = *bytes_2 + bVar8 };
    string_3 = string_6 + string_5;
    unsafe { *string_3 = (*string_3 - bVar5) - CARRY1(bVar4, bVar8) };
    string_3 = string_6 + (string_1 + -1);
    unsafe { *string_3 = *string_3 + bVar5 };
    bVar10 = string_2 * 0x100 == -1;
    if (bVar10) {
        if (!bVar10) {
            pbVar3 = (string_4 + 2);
            let string_5_val = unsafe { *string_5 };
            out(string_5_val, 0);
            puVar2 = local_BP_188 + 0x65;
            unsafe { *puVar2 = *puVar2 + bVar8 };
            string_3 = string_6;
            bVar8 = (string_2 >> 8);
            unsafe { *string_3 = *string_3 + bVar8 };
            puVar2 = local_BP_188 + pbVar3;
            unsafe { *puVar2 = *puVar2 };
            bVar5 = bVar5 ^ string_6[pbVar3];
            bytes_2 = pbVar3;
            unsafe { bVar4 = *bytes_2 };
            unsafe { *bytes_2 = *bytes_2 + bVar8 };
            string_3 = string_6 + pbVar3;
            unsafe { *string_3 = (*string_3 - bVar5) - CARRY1(bVar4, bVar8) };
            string_3 = string_6 + (string_1 + -2);
            unsafe { *string_3 = *string_3 + bVar5 };
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_
}

pub fn pass1_1050_3552() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub fn pass1_1050_3654() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub fn pass1_1050_37d4() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar2 = &stack0xfffe;
    cVar1 = '\x16';
    while {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    return;
}

pub fn pass1_1050_309c() {
    let pbVar1: *mut byte;
    let puVar2: *mut u32;
    let pcVar3: *mut char;
    let piVar4: *mut i32;
    let mut bVar5: u8;
    let mut bVar6: u8;
    let mut in_AX: u16;
    let mut bVar11: u8;
    let local_AX_15: *mut astruct_329;
    let mut cVar12: u8;
    let local_AX_111: *mut astruct_327;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let mut uVar9: i32;
    let pcVar10: *mut char;
    let mut bVar13: u8;
    let mut in_CX: i32;
    let mut bVar14: u8;
    let mut bVar15: u8;
    let mut cVar16: u8;
    let mut bVar17: u8;
    let mut bVar18: u8;
    let mut in_DX: i32;
    let mut iVar19: i32;
    let mut bVar20: u8;
    let mut cVar21: u8;
    let mut bVar22: u8;
    let local_BX__1: *mut astruct_328;
    let mut bVar23: u8;
    let puVar24: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut char;
    let pcVar25: *mut char;
    let unaff_DI: *mut byte;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;
    let mut in_CF: u8;
    let mut bVar26: bool;
    let mut bVar27: bool;
    let mut bStack002a: u8;
    let mut in_stack_0000002a: u16;
    let mut bStack002b: u8;
    let mut in_stack_0000502a: u8;
    let mut in_stack_0000502b: u8;
    let mut bStack3: u8;

    puVar24 = &stack0xfffe;
    cVar12 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar24 = puVar24 + -1;
        unsafe { *puVar24 = *unaff_BP };
        cVar12 = cVar12 + -1;
        '\0' < cVar12
    } {}
    pbVar1 = &stack0xfffe + unaff_DI;
    bVar13 = in_CX;
    unsafe { bVar6 = *pbVar1 };
    unsafe { *pbVar1 = *pbVar1 >> (bVar13 & 0x1f) };
    bVar26 = (in_CX & 0x1f) != 0;
    bVar22 = local_BX__1;
    bVar11 = (in_AX >> 8)
        + bVar22
        + (!bVar26 & in_CF | (bVar26 && (bVar6 >> (bVar13 & 0x1f) - 1 & 1) != 0));
    puVar2 = (local_BX__1 + unaff_SI + 0x10);
    let pu_var2_val = unsafe { *puVar2 };
    iVar19 = in_DX - pu_var2_val;
    out(0x2b, in_AX);
    local_AX_15._0_1_ = in_AX + (iVar19 >> 8) + (in_DX < pu_var2_val);
    iVar19 = (iVar19 - (local_BX__1 + unaff_SI + 0x10)) - (local_BX__1 + unaff_SI + 0x10);
    pbVar1 = 0x502c;
    unsafe { bVar6 = *pbVar1 };
    unsafe { bVar18 = *pbVar1 };
    unsafe { *pbVar1 = bVar18 + bVar13 + (0xd3 < local_AX_15) };
    pcVar3 = unaff_SI + &local_BX__1.field_0x0;
    let pc_var3_val = unsafe { *pcVar3 };
    unsafe {
        *pcVar3 = *pcVar3
            + bVar22
            + (CARRY1(bVar6, bVar13) || CARRY1(bVar18 + bVar13, 0xd3 < local_AX_15))
    };
    pcVar3 = &stack0xfffe + unaff_SI;
    unsafe { *pcVar3 = *pcVar3 + bVar11 + ((local_AX_15 + 0x2c) < 0x50) };
    pcVar3 = unaff_SI;
    bVar14 = (in_CX >> 8);
    unsafe { *pcVar3 = *pcVar3 + bVar14 + ((local_AX_15 - 0x24) < 0x50) };
    bVar26 = (local_AX_15 + 0x8c) < 0x50;
    bVar5 = local_AX_15 + 0x3c;
    pbVar1 = 0x502c;
    bVar20 = (iVar19 >> 8);
    unsafe {
        bVar27 = CARRY1(*pbVar1, bVar20) || CARRY1(*pbVar1 + bVar20, bVar26);
        unsafe { *pbVar1 = *pbVar1 + bVar20 + bVar26 };
        pbVar1 = (local_BX__1 + unaff_SI + 0x2c);
        bVar26 = CARRY1(*pbVar1, bVar5) || CARRY1(*pbVar1 + bVar5, bVar27);
        unsafe { *pbVar1 = *pbVar1 + bVar5 + bVar27 };
        pbVar1 = ((register0x00000010 + 0x2a) + unaff_SI);
        bVar27 = CARRY1(*pbVar1, bVar13) || CARRY1(*pbVar1 + bVar13, bVar26);
    }
    unsafe { *pbVar1 = *pbVar1 + bVar13 + bVar26 };
    pbVar1 = (unaff_SI + 0x2c);
    unsafe { bVar6 = *pbVar1 };
    bVar15 = iVar19;
    unsafe { bVar18 = *pbVar1 };
    unsafe { *pbVar1 = bVar18 + bVar15 + bVar27 };
    bVar26 = CARRY1(bStack002a, bVar22)
        || CARRY1(
            bStack002a + bVar22,
            CARRY1(bVar6, bVar15) || CARRY1(bVar18 + bVar15, bVar27),
        );
    pbVar1 = (local_BX__1 + unaff_SI + 0x2c);
    unsafe {
        bVar27 = CARRY1(*pbVar1, bVar14) || CARRY1(*pbVar1 + bVar14, bVar26);
        unsafe { *pbVar1 = *pbVar1 + bVar14 + bVar26 };
        pbVar1 = ((register0x00000010 + 0x2a) + unaff_SI);
        bVar26 = CARRY1(*pbVar1, bVar20) || CARRY1(*pbVar1 + bVar20, bVar27);
        unsafe { *pbVar1 = *pbVar1 + bVar20 + bVar27 };
        pbVar1 = (unaff_SI + 0x2c);
        bVar23 = (local_BX__1 >> 8);
        bVar27 = CARRY1(*pbVar1, bVar23) || CARRY1(*pbVar1 + bVar23, bVar26);
        unsafe { *pbVar1 = *pbVar1 + bVar23 + bVar26 };
        bVar26 = CARRY1(in_stack_0000502a, bVar5) || CARRY1(in_stack_0000502a + bVar5, bVar27);
        bVar6 = in_stack_0000502a + bVar5 + bVar27;
        pbVar1 = (local_BX__1 + unaff_SI + 0x502c);
        bVar27 = CARRY1(*pbVar1, bVar15) || CARRY1(*pbVar1 + bVar15, bVar26);
        unsafe { *pbVar1 = *pbVar1 + bVar15 + bVar26 };
        pbVar1 = &stack0x502a + unaff_SI;
        bVar26 = CARRY1(*pbVar1, bVar22) || CARRY1(*pbVar1 + bVar22, bVar27);
    }
    unsafe { *pbVar1 = *pbVar1 + bVar22 + bVar27 };
    pbVar1 = (unaff_SI + 0x502c);
    unsafe { bVar18 = *pbVar1 };
    unsafe { bVar17 = *pbVar1 };
    unsafe { *pbVar1 = bVar17 + bVar11 + bVar26 };
    bVar26 = CARRY1(bVar6, bVar14)
        || CARRY1(
            bVar6 + bVar14,
            CARRY1(bVar18, bVar11) || CARRY1(bVar17 + bVar11, bVar26),
        );
    pbVar1 = (local_BX__1 + unaff_SI + 0x502c);
    unsafe { bVar6 = *pbVar1 };
    unsafe { bVar18 = *pbVar1 };
    unsafe { *pbVar1 = bVar18 + bVar23 + bVar26 };
    cVar16 = bVar15 + bVar5 + (CARRY1(bVar6, bVar23) || CARRY1(bVar18 + bVar23, bVar26));
    cVar12 = bVar11 + bVar13 + (bVar5 < 0x50);
    cVar21 = bVar20 + cVar16 + ((local_AX_15 - 0x14) < 0x50);
    local_AX_111._0_1_ = local_AX_15 + 0x4c + cVar12 + ((local_AX_15 + 0x9c) < 0x50);
    bVar17 = cVar16 + bVar14 + (local_AX_111 < 0x50);
    bVar11 = cVar21 + bVar23 + ((local_AX_111 + 0x60) < 0x50);
    uVar7 = CONCAT11(
        cVar12 + cVar21 + ((local_AX_111 + 0xb0) < 0x50),
        local_AX_111 - 0x40,
    );
    pcVar3 = unaff_SI + &local_BX__1.field_0x0;
    unsafe { *pcVar3 = *pcVar3 + bVar13 + ((local_AX_111 + 0x10) < 0x50) };
    pbVar1 = unaff_DI;
    unsafe {
        bVar6 = bVar14 + *pbVar1;
    }
    bVar5 = bVar6 + (uVar7 < 0x1050);
    pcVar3 = unaff_SI;
    unsafe {
        *pcVar3 = *pcVar3 + bVar22 + (CARRY1(bVar14, *pbVar1) || CARRY1(bVar6, uVar7 < 0x1050))
    };
    pbVar1 = unaff_DI;
    unsafe { *pbVar1 = *pbVar1 ^ bVar5 };
    pcVar3 = &stack0xfffe + unaff_SI;
    unsafe { *pcVar3 = *pcVar3 + bVar23 };
    pcVar25 = unaff_SI + -1;
    uVar7 = CONCAT11(local_AX_111 + 0x70, (uVar7 + 0xcf10 >> 8));
    uVar8 = uVar7 + 0xefb0;
    bVar14 = uVar8;
    cVar12 = (uVar8 >> 8);
    pbVar1 = (unaff_SI + 0x2c);
    unsafe { bVar6 = *pbVar1 };
    unsafe { bVar18 = *pbVar1 };
    unsafe { *pbVar1 = bVar18 + bVar5 + (uVar7 < 0x1050) };
    bVar26 = CARRY1(bStack002b, bVar11)
        || CARRY1(
            bStack002b + bVar11,
            CARRY1(bVar6, bVar5) || CARRY1(bVar18 + bVar5, uVar7 < 0x1050),
        );
    pbVar1 = (local_BX__1 + pcVar25 + 0x502d);
    unsafe {
        bVar27 = CARRY1(*pbVar1, bVar14) || CARRY1(*pbVar1 + bVar14, bVar26);
        unsafe { *pbVar1 = *pbVar1 + bVar14 + bVar26 };
        pbVar1 = &stack0x502b + pcVar25;
        bVar26 = CARRY1(*pbVar1, bVar13) || CARRY1(*pbVar1 + bVar13, bVar27);
        unsafe { *pbVar1 = *pbVar1 + bVar13 + bVar27 };
        pbVar1 = (unaff_SI + 0x502c);
        unsafe { bVar6 = *pbVar1 };
        unsafe { bVar18 = *pbVar1 };
        unsafe { *pbVar1 = bVar18 + bVar17 + bVar26 };
        bVar26 = CARRY1(in_stack_0000502b, bVar22)
            || CARRY1(
                in_stack_0000502b + bVar22,
                CARRY1(bVar6, bVar17) || CARRY1(bVar18 + bVar17, bVar26),
            );
        pbVar1 = (local_BX__1 + pcVar25 + 0x502d);
        bVar27 = CARRY1(*pbVar1, bVar5) || CARRY1(*pbVar1 + bVar5, bVar26);
    }
    unsafe { *pbVar1 = *pbVar1 + bVar5 + bVar26 };
    pbVar1 = &stack0x502b + pcVar25;
    unsafe { bVar6 = *pbVar1 };
    unsafe { bVar18 = *pbVar1 };
    unsafe { *pbVar1 = bVar18 + bVar11 + bVar27 };
    pcVar3 = unaff_SI + 0x502c;
    unsafe {
        *pcVar3 = *pcVar3 + bVar23 + (CARRY1(bVar6, bVar11) || CARRY1(bVar18 + bVar11, bVar27))
    };
    iVar19 = uVar7 + 0xdf60;
    pbVar1 = unaff_DI;
    unsafe { bVar6 = *pbVar1 };
    unsafe { *pbVar1 = *pbVar1 >> 1 };
    bVar18 = bVar17 + bVar22 + (bVar6 & 1);
    bVar6 = _in(0x2d);
    uVar9 = (uVar7 + 0xcf10 & 0xff00 | bVar6) + 0xdf60;
    bVar5 = bVar5 + *0x1050;
    pcVar10 = (uVar9 | 0x2e);
    pbVar1 = 0x502e;
    unsafe {
        bVar26 = CARRY1(*pbVar1, bVar18);
        unsafe { *pbVar1 = *pbVar1 + bVar18 };
        pbVar1 = (pcVar25 + &local_BX__1.field_0x0);
        bVar17 = (uVar9 >> 8);
        bVar27 = CARRY1(*pbVar1, bVar17) || CARRY1(*pbVar1 + bVar17, bVar26);
    }
    unsafe { *pbVar1 = *pbVar1 + bVar17 + bVar26 };
    pbVar1 = &stack0xfffe + pcVar25;
    unsafe { bVar6 = *pbVar1 };
    unsafe { bVar18 = *pbVar1 + bVar5 };
    unsafe { *pbVar1 = bVar18 + bVar27 };
    pcVar3 = &stack0xfffe + pcVar25;
    unsafe { *pcVar3 = *pcVar3 + bVar17 + (CARRY1(bVar6, bVar5) || CARRY1(bVar18, bVar27)) };
    pcVar3 = pcVar10 + uVar7 + 0xe364;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    pcVar3 = pcVar10 + uVar7 + 0xdf64;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pcVar10;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    puVar2 = (pcVar10 + iVar19);
    unsafe { *puVar2 = *puVar2 | uVar8 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    iVar19 = CONCAT11((iVar19 >> 8), 4);
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    bVar18 = bVar14 ^ pcVar10[iVar19];
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar18 };
    pbVar1 = (pcVar10 + iVar19);
    unsafe { bVar6 = *pbVar1 };
    unsafe { *pbVar1 = *pbVar1 + bVar18 };
    piVar4 = (pcVar10 + iVar19);
    unsafe { *piVar4 = (*piVar4 - CONCAT11(4, bVar18)) - CARRY1(bVar6, bVar18) };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar18 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar18 };
    bVar18 = bVar18 ^ pcVar10[iVar19];
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + bVar18 };
    pbVar1 = (pcVar10 + 0x404);
    unsafe { bVar6 = *pbVar1 };
    unsafe { *pbVar1 = *pbVar1 + bVar18 };
    piVar4 = (pcVar10 + 0x404);
    unsafe { *piVar4 = (*piVar4 - CONCAT11(4, bVar18)) - CARRY1(bVar6, bVar18) };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + bVar18 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x508;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pcVar10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pcVar10 * 2 + 0x104);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pcVar10 * 2 + 4);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pcVar10 * 2);
    unsafe { *pcVar3 = *pcVar3 + bVar14 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    cVar12 = bVar14 + 4;
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pcVar10 + 0x104;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pcVar10 + 4;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    cVar12 = cVar12 + bVar14;
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = 0x608;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    cVar12 = cVar12 * 0x2;
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = (pcVar10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    cVar21 = cVar12 + 0x4;
    iVar19 = CONCAT11(4, cVar21);
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar21 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = (pcVar10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar21 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 + 0x1 };
    cVar16 = cVar12 + 0x4;
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar21 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    iVar19 = CONCAT11(cVar12 + 0x8, cVar21);
    pcVar3 = (pcVar10 * 2);
    unsafe { *pcVar3 = *pcVar3 + bVar14 + cVar12 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    unsafe { *0x2 = pcVar10 };
    iVar19 = iVar19 + -1;
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    pcVar3 = pcVar10;
    unsafe { *pcVar3 = *pcVar3 + cVar12 + 0x8 };
    pcVar3 = pcVar10 + iVar19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    return;
}

pub fn pass1_1050_3344() {
    let pcVar1: *mut char;
    let pbVar2: *mut byte;
    let piVar3: *mut i32;
    let mut bVar4: u8;
    let mut bVar5: u8;
    let mut in_AX: i32;
    let mut uVar6: u16;
    let mut extraout_DH: u8;
    let mut in_BX: i32;
    let unaff_SI: *mut char;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    bVar4 = in_AX;
    out(4, bVar4);
    pcVar1 = unaff_SI + in_BX;
    unsafe { *pcVar1 = *pcVar1 + bVar4 };
    bVar4 = bVar4 ^ unaff_SI[in_BX];
    pcVar1 = unaff_SI + in_BX;
    unsafe { *pcVar1 = *pcVar1 + bVar4 };
    out(4, in_AX & 0xff00 | bVar4);
    pbVar2 = (unaff_SI + in_BX);
    unsafe { bVar5 = bVar4 + *pbVar2 };
    piVar3 = (unaff_SI + in_BX);
    unsafe { *piVar3 = (*piVar3 - (in_AX & 0xff00 | bVar5)) - CARRY1(bVar4, *pbVar2) };
    pcVar1 = unaff_SI + in_BX;
    unsafe { *pcVar1 = *pcVar1 + bVar5 };
    uVar6 = bad_fn_1050_335f();
    pcVar1 = &stack0xfffe + unaff_SI;
    unsafe { *pcVar1 = *pcVar1 + extraout_DH };
    pcVar1 = unaff_SI + in_BX;
    unsafe { *pcVar1 = *pcVar1 + uVar6 };
    pcVar1 = unaff_SI;
    unsafe { *pcVar1 = *pcVar1 + (uVar6 >> 8) };
    pcVar1 = unaff_SI + in_BX;
    unsafe { *pcVar1 = *pcVar1 + uVar6 };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn pass1_1048_1e02() -> i32 {
    let pcVar1: *mut char;
    let mut cVar2: u8;
    let mut in_AX: i32;
    let mut in_BX: i32;
    let puVar3: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar3 = &stack0xfffe;
    cVar2 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar3 = puVar3 + -1;
        unsafe { *puVar3 = *unaff_BP };
        cVar2 = cVar2 + -1;
        '\0' < cVar2
    } {}
    pcVar1 = (in_BX + unaff_SI);
    cVar2 = in_AX;
    unsafe { *pcVar1 = *pcVar1 + cVar2 };
    pcVar1 = (in_BX + unaff_SI);
    unsafe { *pcVar1 = *pcVar1 + cVar2 };
    pcVar1 = (in_BX + unaff_SI);
    unsafe { *pcVar1 = *pcVar1 + cVar2 };
    pcVar1 = (in_BX + unaff_SI);
    unsafe { *pcVar1 = *pcVar1 + cVar2 };
    pcVar1 = (in_BX + unaff_SI);
    unsafe { *pcVar1 = *pcVar1 + cVar2 };
    return in_AX + (in_BX + unaff_SI);
}

pub fn pass1_1048_3c50() {
    let piVar1: *mut i32;
    let pcVar2: *mut char;
    let mut cVar3: u8;
    let mut in_AX: u16;
    let mut iVar4: i32;
    let mut in_CX: u16;
    let mut in_DX: u16;
    let in_BX: *mut char;
    let mut cVar5: u8;
    let puVar6: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut char;
    let unaff_DI: *mut char;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar6 = &stack0xfffe;
    cVar3 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar6 = puVar6 + -1;
        unsafe { *puVar6 = *unaff_BP };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    out(in_DX, in_AX);
    piVar1 = (in_BX + unaff_DI);
    unsafe { *piVar1 = *piVar1 + 1 };
    pcVar2 = unaff_DI;
    unsafe { *pcVar2 = *pcVar2 + in_DX };
    pcVar2 = in_BX + unaff_SI;
    unsafe { *pcVar2 = *pcVar2 + (in_CX >> 8) };
    pcVar2 = &stack0xfffe + unaff_SI;
    cVar5 = (in_BX >> 8);
    unsafe { *pcVar2 = *pcVar2 + cVar5 };
    pcVar2 = &stack0xfffe + unaff_DI;
    unsafe { *pcVar2 = *pcVar2 + in_CX };
    pcVar2 = unaff_SI;
    unsafe { *pcVar2 = *pcVar2 + (in_AX >> 8) };
    cVar3 = _in(in_DX);
    unsafe { *unaff_DI = cVar3 };
    pcVar2 = in_BX + (unaff_DI + 1);
    unsafe { *pcVar2 = *pcVar2 + (in_DX >> 8) };
    iVar4 = (in_BX + unaff_SI) * 0x62;
    pcVar2 = in_BX;
    unsafe { *pcVar2 = *pcVar2 + in_DX };
    pcVar2 = in_BX + unaff_SI;
    unsafe { *pcVar2 = *pcVar2 + cVar5 };
    pcVar2 = 0x1300;
    unsafe { *pcVar2 = *pcVar2 + (iVar4 >> 8) };
    pcVar2 = in_BX + unaff_SI;
    unsafe { *pcVar2 = *pcVar2 + iVar4 };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn pass1_1048_3da0() {
    let ppcVar1: fn();
    let mut cVar2: u8;
    let in_AL: u8;
    let mut in_BX: i32;
    let puVar3: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let unaff_DI: *mut u8;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;
    let mut local_6e03: u16;
    let mut local_6703: u16;
    let puStack64: *mut u8;

    puStack64 = &stack0xfffe;
    puVar3 = &stack0xfffe;
    cVar2 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar3 = puVar3 + -1;
        unsafe { *puVar3 = *unaff_BP };
        cVar2 = cVar2 + -1;
        '\0' < cVar2
    } {}
    unsafe { *unaff_DI = in_AL };
    ppcVar1 = (in_BX + -0x6801);
    (**ppcVar1)();
    (**(&local_6e03 + unaff_SI))();
    (**(&local_6703 + unaff_SI))();
    // WARNING: Could not recover jumptable at 0x10483db1. Too many branches
    // WARNING: Treating indirect jump as call
    (**(in_BX + unaff_SI + -0x5401))();
    return;
}

pub fn pass1_1048_3de8() {
    let mut cVar1: u8;
    let mut in_AX: u16;
    let mut in_BX: i32;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let unaff_DI: *mut u16;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;

    puVar2 = &stack0xfffe;
    cVar1 = '\x1e';
    while {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    } {}
    unsafe { *unaff_DI = in_AX };
    // WARNING: Could not recover jumptable at 0x10483ded. Too many branches
    // WARNING: Treating indirect jump as call
    (**(in_BX + unaff_SI + -0x6701))();
    return;
}

pub fn pass1_1048_4072() {
    let mut cVar1: u8;
    let p_uvar2: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_DI: i32;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;
    let mut local_5503: u16;
    let puStack64: *mut u8;

    puStack64 = &stack0xfffe;
    puVar2 = &stack0xfffe;
    cVar1 = '\x1e';
    unsafe {
        unaff_BP = unaff_BP + -1;
        puVar2 = puVar2 + -1;
        unsafe { *puVar2 = *unaff_BP };
        cVar1 = cVar1 + -1;
        '\0' < cVar1
    }
    []
    // WARNING: Could not recover jumptable at 0x10484079. Too many branches
    // WARNING: Treating indirect jump as call
    (* * (&stack0xaaff + unaff_DI))();
    return;
}

pub fn pass1_1040_d89e(param_1: u32, param_2: u8) {
    win_cleanup_1040_d1bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_d76e(param_1: *mut astruct_330) {
    let mut u_var1: u32;
    let local_BX_3: *mut astruct_330;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    uVar1 = local_BX_3.field_0x94;
    pass1_1018_5742(
        uVar1,
        (uVar1 >> 0x10),
        local_BX_3.field_0x9c,
        local_BX_3.field_0x98,
    );
    local_BX_3.field_0x9c = 0;
    return;
}

pub fn pass1_1040_d056(param_1: u32, param_2: u8) {
    pass1_1040_ca74(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_d0d8(param_1: u32) {
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut cVar4: u8;
    let mut uVar5: i32;
    let mut in_AL: u8;
    let mut in_CL: u8;
    let mut bVar6: u8;
    let mut in_DX: i32;
    let mut bVar7: u8;
    let mut in_BX: i32;
    let mut bVar8: u8;
    let puVar9: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut bVar11: bool;
    let local_4e: u8;

    puVar9 = &stack0xfffe;
    cVar4 = '\x0f';
    unsafe {
        unaff_BP = unaff_BP + -1;
        puVar9 = puVar9 + -1;
        unsafe { *puVar9 = *unaff_BP };
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    }
    {}
    bVar8 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar8;
    bVar7 = (in_DX + 1 >> 8);
    bVar2 = bVar7 + bVar8;
    bVar10 = CARRY1(bVar7, bVar8) || CARRY1(bVar2, in_CF);
    uVar5 = in_DX + 1 & 0xff;
    pbVar1 = unaff_SI + in_BX;
    bVar6 = uVar5;
    unsafe { bVar7 = *pbVar1 };
    unsafe { bVar3 = *pbVar1 - bVar6 };
    unsafe { bVar11 = *pbVar1 < bVar6 || bVar3 < bVar10 };
    unsafe { *pbVar1 = bVar3 - bVar10 };
    let pb_var1_val = unsafe { *pbVar1 };
    if (((pb_var1_val != 0) && SBORROW1(bVar7, bVar6))
        != (SBORROW1(bVar3, bVar10) == (pb_var1_val < '\0')))
    {
        pbVar1 = unaff_SI;
        unsafe { bVar7 = *pbVar1 };
        unsafe { bVar3 = *pbVar1 };
        unsafe { *pbVar1 = bVar3 + bVar8 + bVar11 };
        bVar10 = CARRY1(local_4e, in_BX)
            || CARRY1(
                local_4e + in_BX,
                CARRY1(bVar7, bVar8) || CARRY1(bVar3 + bVar8, bVar11),
            );
        pbVar1 = unaff_SI + -0x4f;
        unsafe { bVar7 = *pbVar1 };
        unsafe { bVar3 = *pbVar1 };
        unsafe { *pbVar1 = bVar3 + bVar8 + bVar10 };
        return CONCAT22(
            uVar5
                | (bVar2 + in_CF + in_CL + (CARRY1(bVar7, bVar8) || CARRY1(bVar3 + bVar8, bVar10)))
                    << 8,
            CONCAT11(0x40, in_AL + 0x1) + 2,
        );
    }
    let pb_var1_val = unsafe { *pbVar1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_d0f8(param_1: *mut u16, param_2: u16) {
    let mut uVar1: u16;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let struct_a: *mut astruct_199;
    let paVar4: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let ppVar7: *mut pass1_struct_1;
    let mut in_stack_0000fffa: u16;
    let mut local_4: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1845));
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    (iVar5 + 0x94) = 0;
    (iVar5 + 0x98) = _PTR_LOOP_1050_5f16;
    (iVar5 + 0x9c) = 0;
    (iVar5 + 0xa0) = 0;
    unsafe { *param_1 = 0xd8c4 };
    (iVar5 + 2) = &PTR_LOOP_1050_1040;
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fffa, 0x47));
    uVar3 = (ppVar7 >> 0x10);
    uVar1 = SUB42(ppVar7, 0);
    (iVar5 + 0x94) = uVar1;
    (iVar5 + 0x96) = uVar3;
    pass1_1018_5732((iVar5 + 0x94), uVar3, (iVar5 + 0x98));
    (iVar5 + 0x9c) = uVar1;
    (iVar5 + 0x9e) = struct_a;
    uVar2 = struct_a | (iVar5 + 0x9c);
    if (uVar2 == 0) {
        paVar4 = struct_a;
        process_struct_1000_179c(0xc, struct_a);
        if ((paVar4 | uVar2) == 0) {
            (iVar5 + 0x9c) = 0;
        } else {
            pass1_1010_8ef2(CONCAT22(paVar4, uVar2));
            (iVar5 + 0x9c) = uVar2;
            (iVar5 + 0x9e) = extraout_DX;
        }
    }
    return;
}

pub fn pass1_1040_ca74(param_1: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    unsafe { *param_1 = 0xd07c };
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    PTR_LOOP_1050_5f10 = 0x0;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn pass1_1040_ca16(param_1: *mut astruct_331, param_2: u16) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let local_a: *mut astruct_331;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1840));
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = _PTR_LOOP_1050_5f0c;
    (iVar1 + 0x98) = 0;
    (iVar1 + 0x9c) = 0;
    (iVar1 + 0x9e) = 0;
    param_1.field_0x0 = 0xd07c;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x3e));
    (iVar1 + 0x98) = ppVar3;
    (iVar1 + 0x9a) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_c9cc(param_1: u32, param_2: u8) {
    pass1_1040_c5ac(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

pub fn pass1_1040_ca03() {
    let pbVar1: *mut byte;
    let pcVar2: *mut char;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let puVar5: *mut u16;
    let mut cVar6: u8;
    let puVar7: *mut u16;
    let mut uVar8: u32;
    let mut in_AL: u8;
    let mut bVar9: u8;
    let mut in_DX: i32;
    let mut in_BL: u8;
    let local_BX_41: *mut astruct_332;
    let local_BX_92: *mut astruct_333;
    let puVar10: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_DI: i32;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar12: bool;
    let mut bVar13: bool;
    let ppVar14: *mut pass1_struct_1;
    let mut in_stack_00000000: u16;

    puVar10 = &stack0xfffe;
    cVar6 = '\t';
    puVar5 = unaff_BP;
    while {
        puVar5 = puVar5 + -1;
        puVar10 = puVar10 + -1;
        unsafe { *puVar10 = *puVar5 };
        cVar6 = cVar6 + -1;
        '\0' < cVar6
    } {}
    bVar9 = in_DX * 0x2 + in_CF;
    bVar12 = (in_AL + 1) < 8;
    pbVar1 = (unaff_BP + unaff_SI + -0x38);
    unsafe {
        bVar13 = CARRY1(*pbVar1, in_BL) || CARRY1(*pbVar1 + in_BL, bVar12);
    }
    unsafe { *pbVar1 = *pbVar1 + in_BL + bVar12 };
    pbVar1 = (unaff_BP + unaff_SI + 0x40c8);
    unsafe { bVar3 = *pbVar1 };
    unsafe { bVar4 = *pbVar1 + in_AL + 2 };
    unsafe { *pbVar1 = bVar4 + bVar13 };
    pcVar2 = (unaff_DI + -0x75);
    unsafe { *pcVar2 = *pcVar2 + bVar9 + (CARRY1(bVar3, in_AL + 2) || CARRY1(bVar4, bVar13)) };
    _in(in_DX & 0xff00 | bVar9);
    process_struct_1040_b082((unaff_BP + 3), CONCAT22(unaff_BP[5], 0x1840));
    puVar7 = (unaff_BP + 3);
    uVar11 = (puVar7 >> 0x10);
    local_BX_41 = puVar7;
    local_BX_41.field_0x94 = _PTR_LOOP_1050_5f0c;
    local_BX_41.field_0x98 = 0;
    local_BX_41.field_0x9c = 0;
    local_BX_41.field_0x9e = 0;
    unsafe { *puVar7 = 0xd07c };
    local_BX_41.field_0x2 = &PTR_LOOP_1050_1040;
    ppVar14 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_00000000, 0x3e));
    uVar8 = (unaff_BP + 3);
    uVar11 = (uVar8 >> 0x10);
    local_BX_92 = uVar8;
    local_BX_92.field_0x98 = ppVar14;
    local_BX_92.field_0x9a = (ppVar14 >> 0x10);
    return;
}

pub fn pass1_1040_c71e(param_1: *mut astruct_335) {
    let local_BX_12: *mut astruct_335;
    let mut uVar1: u16;

    process_struct_1040_9252(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    local_BX_12.field_0x28 = local_BX_12.field_0x24 / 2 - local_BX_12.field_0x2c / 2;
    return;
}

pub fn pass1_1040_c54a(param_1: u32, param_2: u16, in_astruct_3: *mut astruct_114) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut extraout_DX: u16;
    let local_BX_7: *mut astruct_114;
    let local_BX_49: *mut astruct_113;
    let mut uVar3: i32;
    let in_astruct_1: *mut astruct_74;
    let paVar4: *mut astruct_114;
    let mut uVar5: u16;
    let mut uVar6: u32;

    local_BX_7 = in_astruct_3;
    iVar2 = local_BX_7.field_0x12 + 200;
    uVar6 = 0;
    uVar5 = 0;
    ppcVar1 = (in_astruct_3 + 0x14);
    paVar4 = in_astruct_3;
    (**ppcVar1)();
    in_astruct_1 = param_1;
    make_proc_inst_1040_8fb8(
        in_astruct_1,
        0,
        CONCAT22(extraout_DX, iVar2),
        paVar4,
        (paVar4 >> 0x10),
        uVar5,
        uVar6,
        (uVar6 >> 0x10),
    );
    uVar3 = (param_1 >> 0x10);
    (in_astruct_1 + 1) = in_astruct_3;
    in_astruct_1[1].field_0x4 = 0;
    in_astruct_1[1].field_0x6 = param_2;
    param_1 = 0xc9f2;
    in_astruct_1.field_0x2 = &PTR_LOOP_1050_1040;
    process_struct_1040_c630((param_1 & 0xffff | uVar3 << 0x10));
    return;
}

pub fn pass1_1040_c5ac(param_1: *mut astruct_337) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut astruct_337;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1 = 0xc9f2;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1040;
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + -1;
    if (PTR_LOOP_1050_5f02 == 0x0) {
        puVar1 = local_BX_5.field_0x8;
        uVar2 = local_BX_5.field_0xa;
        if ((uVar2 | puVar1) != 0) {
            unsafe { ppcVar3 = *puVar1 };
            (**ppcVar3)();
        }
        puVar1 = local_BX_5.field_0xc;
        uVar2 = local_BX_5.field_0xe;
        if ((uVar2 | puVar1) != 0) {
            unsafe { ppcVar3 = *puVar1 };
            (**ppcVar3)();
        }
    }
    free_proc_inst_1040_911e(param_1);
    return;
}

pub fn pass1_1040_c60e(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x42) != 0) {
        uVar1 = (param_1 + 0x42);
        return (uVar1 + 0x12);
    }
    return 0;
}

pub fn pass1_1040_bfde(param_1: *mut astruct_338, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let local_BX_15: *mut astruct_338;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_15 = param_1;
    local_BX_15.field_0x6 = param_2;
    ppcVar1 = (param_2 + 4);
    (**ppcVar1)();
    uVar2 = local_BX_15.field_0x6;
    (uVar2 + 0x22) = local_BX_15.field_0x4;
    pass1_1010_2ee2(local_BX_15.field_0x6);
    return;
}

pub fn pass1_1040_be94(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_bb5a(param_1: u32) {
    let fn_ptr_1: fn();

    fn_ptr_1 = ((param_1 + 0x94) + 8);
    (**fn_ptr_1)();
    return 0;
}

pub fn pass1_1040_b864(
    param_1: *mut u32,
    param_2: *mut i32,
    param_3: u16,
    param_3: u16_00,
    param_5: *mut u8,
) {
    let ppcVar1: fn();
    let uVar2: u8;
    let extraout_AH: u8;

    if (param_5 == (s_New_failed_in_Op__Op_1050_0020 + 0xb)) {
        let param_2_val = unsafe { *param_2 };
        if (param_2_val == 4) {
            get_prop_1040_9566(param_2, param_3);
        }
    } else {
        if (param_5 != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1)) {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_3_00, param_5);
            return CONCAT11(extraout_AH, uVar2);
        }
        let param_1_val = unsafe { *param_1 };
        ppcVar1 = (param_1_val + 0x7c);
        (**ppcVar1)();
    }
    return 1;
}

pub fn pass1_1040_b74c(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_b7ce(param_1: u32) {
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut cVar3: u8;
    let mut uVar4: i32;
    let mut in_CX: u16;
    let mut bVar5: u8;
    let mut in_DX: i32;
    let mut bVar6: u8;
    let mut in_BX: i32;
    let mut bVar7: u8;
    let local_BX_55: *mut astruct_343;
    let puVar8: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let paVar13: *mut astruct_346;
    let lVar14: u32;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar8 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar8 = puVar8 + -1;
        unsafe { *puVar8 = *unaff_BP };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar7 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar7;
    bVar6 = (in_DX + 1 >> 8);
    bVar2 = bVar6 + bVar7;
    bVar11 = CARRY1(bVar6, bVar7) || CARRY1(bVar2, in_CF);
    uVar4 = in_DX + 1 & 0xff;
    lVar14 = CONCAT22(uVar4 | (bVar2 + in_CF) << 8, in_BX);
    paVar13 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar1 = unaff_SI + in_BX;
    bVar5 = uVar4;
    unsafe { bVar2 = *pbVar1 };
    unsafe { bVar6 = *pbVar1 - bVar5 };
    unsafe { bVar12 = *pbVar1 < bVar5 || bVar6 < bVar11 };
    unsafe { *pbVar1 = bVar6 - bVar11 };
    let pb_var1_val = unsafe { *pbVar1 };
    if (pb_var1_val != 0
        && (SBORROW1(bVar2, bVar5) != SBORROW1(bVar6, bVar11)) == (pb_var1_val < '\0'))
    {
        pbVar1 = unaff_SI;
        bVar11 = CARRY1(pb_var1_val, bVar7) || CARRY1(pb_var1_val + bVar7, bVar12);
        unsafe { *pbVar1 = *pbVar1 + bVar7 + bVar12 };
        bVar2 = local_4e + in_BX;
        bVar12 = CARRY1(local_4e, in_BX) || CARRY1(bVar2, bVar11);
        local_4e = bVar2 + bVar11;
        pbVar1 = unaff_SI + -0x4f;
        unsafe { bVar2 = *pbVar1 };
        unsafe { bVar6 = *pbVar1 };
        unsafe { *pbVar1 = bVar6 + bVar7 + bVar12 };
        pbVar1 = unaff_SI + -0x78;
        unsafe {
            *pbVar1 = *pbVar1 + in_CX + (CARRY1(bVar2, bVar7) || CARRY1(bVar6 + bVar7, bVar12))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_b0bc(paVar13, 0, CONCAT22(in_CX, 0xfab));
        uVar9 = (paVar13 >> 0x10);
        local_BX_55 = paVar13;
        local_BX_55.field_0x94 = 0;
        local_BX_55.field_0x98 = 0;
        local_BX_55.field_0xb0 = 0;
        local_BX_55.field_0xb4 = 0;
        local_BX_55.field_0xb6 = 0;
        paVar13 = 0xbeba;
        local_BX_55.field_0x2 = &PTR_LOOP_1050_1040;
        if (lVar14 != 0) {
            uVar10 = (lVar14 >> 0x10);
            local_BX_55.field_0xb0 = (lVar14 + 6);
            local_BX_55.field_0xb4 = (lVar14 + 0x14);
        }
        return;
    }
    let pb_var1_val = unsafe { *pbVar1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1040_b316(
    param_1: *mut u32,
    param_2: *mut u8,
    param_3: *mut u8,
    param_3_00: *mut u8,
    param_5: *mut u8,
) {
    let ppcVar1: fn();
    let mut uVar2: u16;
    let mut local_4: u16;

    if (param_5 == (&PTR_LOOP_1050_000e + 1)) {
        let param_1_val = unsafe { *param_1 };
        ppcVar1 = (param_1_val + 0x60);
        local_4._0_1_ = (**ppcVar1)();
    } else {
        if (param_5 == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1)) {
            let param_1_val = unsafe { *param_1 };
            ppcVar1 = (param_1_val + 0x10);
            (**ppcVar1)();
            local_4._0_1_ = 0x1;
        } else {
            uVar2 = pass1_1040_79c0(param_1, param_2, param_3, param_3_00, param_5);
            local_4._0_1_ = uVar2;
        }
    }
    return local_4;
}

pub fn pass1_1040_af9e(param_1: u32, param_2: u8) {
    pass1_1040_ace8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_b020(param_1: u32) {
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut cVar3: u8;
    let mut uVar4: i32;
    let in_CX: *mut u8;
    let mut bVar5: u8;
    let mut in_DX: i32;
    let mut uVar6: i32;
    let mut in_BX: i32;
    let mut bVar9: u8;
    let mut iVar8: i32;
    let puVar10: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar12: bool;
    let mut bVar13: bool;
    let in_struct_1: *mut astruct_68;
    let mut uVar14: u32;
    let local_4e: u8;
    let puStack34: *mut u8;
    let mut bVar7: u8;

    puStack34 = &stack0xfffe;
    puVar10 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar10 = puVar10 + -1;
        unsafe { *puVar10 = *unaff_BP };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar9 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar9;
    bVar7 = (in_DX + 1 >> 8);
    bVar2 = bVar7 + bVar9;
    bVar12 = CARRY1(bVar7, bVar9) || CARRY1(bVar2, in_CF);
    uVar4 = in_DX + 1 & 0xff;
    uVar6 = uVar4 | (bVar2 + in_CF) << 8;
    uVar14 = CONCAT22(uVar6, in_BX);
    in_struct_1 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar1 = unaff_SI + in_BX;
    bVar5 = uVar4;
    unsafe { bVar2 = *pbVar1 };
    unsafe { bVar7 = *pbVar1 - bVar5 };
    unsafe { bVar13 = *pbVar1 < bVar5 || bVar7 < bVar12 };
    unsafe { *pbVar1 = bVar7 - bVar12 };
    let pb_var1_val = unsafe { *pbVar1 };
    if (pb_var1_val != 0
        && (SBORROW1(bVar2, bVar5) != SBORROW1(bVar7, bVar12)) == (pb_var1_val < '\0'))
    {
        pbVar1 = unaff_SI;
        bVar12 = CARRY1(pb_var1_val, bVar9) || CARRY1(pb_var1_val + bVar9, bVar13);
        unsafe { *pbVar1 = *pbVar1 + bVar9 + bVar13 };
        bVar2 = local_4e + in_BX;
        bVar13 = CARRY1(local_4e, in_BX) || CARRY1(bVar2, bVar12);
        local_4e = bVar2 + bVar12;
        pbVar1 = unaff_SI + -0x4f;
        unsafe { bVar2 = *pbVar1 };
        unsafe { bVar7 = *pbVar1 };
        unsafe { *pbVar1 = bVar7 + bVar9 + bVar13 };
        pbVar1 = unaff_SI + -0x78;
        unsafe {
            *pbVar1 = *pbVar1 + in_CX + (CARRY1(bVar2, bVar9) || CARRY1(bVar7 + bVar9, bVar13))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_7728(
            in_struct_1,
            (&PTR_LOOP_1050_0000 + 1),
            0,
            *(in_BX + 0x12),
            in_CX,
        );
        uVar11 = (in_struct_1 >> 0x10);
        iVar8 = in_struct_1;
        (iVar8 + 0x8e) = 0;
        (iVar8 + 0x90) = uVar14;
        in_struct_1.field_0x0 = 0xb772;
        (iVar8 + 2) = &PTR_LOOP_1050_1040;
        return;
    }
    let pb_var1_val = unsafe { *pbVar1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1040_b040(param_1: *mut u16, param_2: u32, param_3: u16) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        *(param_2 + 0x12),
        param_3,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x90) = param_2;
    unsafe { *param_1 = 0xb772 };
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_ac64(param_1: u32) {
    byte * *ppbVar1;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut cVar4: u8;
    let mut bVar5: u8;
    let mut in_CL: u8;
    let mut in_DX: i32;
    let mut bVar6: u8;
    let mut in_BX: i32;
    let mut bVar8: u8;
    let mut iVar7: i32;
    let puVar9: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let ppVar13: *mut pass1_struct_1;
    let puVar14: *mut u8;
    let puVar15: *mut u16;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar9 = &stack0xfffe;
    puVar14 = &stack0xfffe;
    cVar4 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar9 = puVar9 + -1;
        unsafe { *puVar9 = *unaff_BP };
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    bVar8 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar8;
    bVar6 = ((in_DX + 1) >> 8);
    bVar11 = CARRY1(bVar6, bVar8) || CARRY1(bVar6 + bVar8, in_CF);
    bVar5 = (in_DX + 1);
    puVar15 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar2 = unaff_SI + in_BX;
    unsafe { bVar6 = *pbVar2 };
    unsafe { bVar3 = *pbVar2 - bVar5 };
    unsafe { bVar12 = *pbVar2 < bVar5 || bVar3 < bVar11 };
    unsafe { *pbVar2 = bVar3 - bVar11 };
    let pb_var2_val = unsafe { *pbVar2 };
    if (pb_var2_val != 0
        && (SBORROW1(bVar6, bVar5) != SBORROW1(bVar3, bVar11)) == (pb_var2_val < '\0'))
    {
        pbVar2 = unaff_SI;
        bVar11 = CARRY1(pb_var2_val, bVar8) || CARRY1(pb_var2_val + bVar8, bVar12);
        unsafe { *pbVar2 = *pbVar2 + bVar8 + bVar12 };
        bVar6 = local_4e + in_BX;
        bVar12 = CARRY1(local_4e, in_BX) || CARRY1(bVar6, bVar11);
        local_4e = bVar6 + bVar11;
        pbVar2 = unaff_SI + -0x4f;
        unsafe { bVar6 = *pbVar2 };
        unsafe { bVar3 = *pbVar2 };
        unsafe { *pbVar2 = bVar3 + bVar8 + bVar12 };
        pbVar2 = unaff_SI + -0x78;
        unsafe {
            *pbVar2 = *pbVar2 + in_CL + (CARRY1(bVar6, bVar8) || CARRY1(bVar3 + bVar8, bVar12))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_b082(CONCAT22(&stack0xbf2a, &stack0xfffe), CONCAT22(in_BX, 499));
        uVar10 = (puVar15 >> 0x10);
        iVar7 = puVar15;
        (iVar7 + 0x94) = 0;
        (iVar7 + 0x98) = 0;
        unsafe { *puVar15 = 0xafc4 };
        (iVar7 + 2) = &PTR_LOOP_1050_1040;
        (iVar7 + 0x94) = _PTR_LOOP_1050_5ef0;
        _PTR_LOOP_1050_5ef0 = 0;
        ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar14, 0x3d));
        uVar10 = (puVar15 >> 0x10);
        (puVar15 + 0x98) = ppVar13;
        (puVar15 + 0x9a) = (ppVar13 >> 0x10);
        return;
    }
    ppbVar1 = (unaff_SI + 9);
    unsafe { *ppbVar1 = unaff_SI + *ppbVar1 };
    error_check_1000_17ce(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ac84(param_1: *mut astruct_350, param_2: u16) {
    let local_BX_18: *mut astruct_350;
    let mut unaff_BP: u16;
    let mut uVar1: u16;
    let ppVar2: *mut pass1_struct_1;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 499));
    uVar1 = (param_1 >> 0x10);
    local_BX_18 = param_1;
    local_BX_18.field_0x94 = 0;
    &local_BX_18.field_0x98 = 0;
    param_1 = 0xafc4;
    local_BX_18.field_0x2 = &PTR_LOOP_1050_1040;
    local_BX_18.field_0x94 = _PTR_LOOP_1050_5ef0;
    _PTR_LOOP_1050_5ef0 = 0;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x3d));
    local_BX_18.field_0x98 = ppVar2;
    local_BX_18.field_0x9a = (ppVar2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ace8(in_struct_1: *mut astruct_348) {
    let local_struct_1: *mut astruct_348;
    let mut uVar1: u16;

    uVar1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.field_0x0 = 0xafc4;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, local_struct_1.field_0x6);
    win_cleanup_func_1040_b0f8(in_struct_1);
    return;
}

pub fn pass1_1008_de58(param_1: u32, param_2: u32, param_3: u32) {
    let ppcVar1: fn();
    let mut bVar2: bool;
    let local_AX_39: *mut astruct_305;
    let mut extraout_DX: i32;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_00: u16;
    let paVar3: *mut astruct_305;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    uVar4 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 10));
    bVar2 = false;
    while {
        local_AX_39 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_39));
        struct_a = (extraout_DX | local_AX_39);
        paVar3 = local_AX_39;
        if (struct_a == 0x0) {}
        // goto LAB_1008_dedb;
        ((local_AX_39.field_0x4 != param_3) || (local_AX_39.field_0x8 != param_2))
            && (local_AX_39.field_0x8 != param_3 || (local_AX_39.field_0x4 != param_2))
    } {}
    local_AX_39.field_0xc = 1;
    uVar6 = pass1_1030_8326();
    struct_a = (uVar6 >> 0x10);
    paVar3 = uVar6;
    local_AX_39.field_0xe = paVar3;
    local_AX_39.field_0x10 = struct_a;
    bVar2 = true;
    // LAB_1008_dedb:
    if (!bVar2) {
        process_struct_1000_179c(0x14, struct_a);
        if ((struct_a | paVar3) == 0) {
            paVar3 = 0x0;
            uVar5 = 0;
        } else {
            pass1_1008_dc90(CONCAT22(struct_a, paVar3), param_2, param_3);
            uVar5 = extraout_DX_00;
        }
        paVar3.field_0xc = 1;
        uVar6 = pass1_1030_8326();
        paVar3.field_0xe = uVar6;
        paVar3.field_0x10 = (uVar6 >> 0x10);
        ppcVar1 = ((param_1 + 10) + 4);
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1008_df4a(param_1: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut uVar3: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    uVar2 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 10));
    while (true) {
        uVar3 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        uVar1 = (uVar3 >> 0x10);
        if (uVar3 == 0) {
            break;
        }
        if (((uVar3 + 0xc) == 2) || ((uVar3 + 0xc) == 3)) {
            pass1_1008_e9a4(param_1, uVar2, uVar3);
        }
    }
    return;
}

pub fn pass1_1008_dfa6(param_1: u32, param_2: u32, param_3: u32) {
    let local_AX_37: *mut astruct_306;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 10));
    while {
        local_AX_37 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_37));
        if ((extraout_DX | local_AX_37) == 0) {
            return;
        }
        ((local_AX_37.field_0x4 != param_3) || (local_AX_37.field_0x8 != param_2))
            && (local_AX_37.field_0x8 != param_3 || (local_AX_37.field_0x4 != param_2))
    } {}
    if (local_AX_37.field_0xc != 1) {
        return;
    }
    return;
}

pub fn pass1_1008_e01c(param_1: u32, param_2: u32, param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x1a) = param_2;
    return;
}

pub fn pass1_1008_e038(param_1: u32, param_2: u32, param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x16);
    param_2 = (param_1 + 0x1a);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
