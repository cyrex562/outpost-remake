use crate::defines::HWND16;

pub fn cleanup_1040_abe2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn win_cleanup_1010_0ee6(param_1: u32, param_2: u8) {
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn cleanup_1010_17c0(param_1: *mut astruct_340) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_BX_14: *mut astruct_371;
    let mut uvar3: u16;
    let fn_ptr_1: fn();

    destroy_win_1010_2fa0(param_1);
    uVar3 = (param_1 >> 0x10);
    local_BX_14 = param_1;
    puVar1 = local_BX_14.field_0x56;
    uVar2 = &local_BX_14.field_0x58;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
            (**fn_ptr_1)();
        }
    }
    &local_BX_14.field_0x56 = 0;
    error_check_1000_17ce(local_BX_14.field_0x60);
    pass1_1000_4906(local_BX_14.field_0x64, 0, local_BX_14.field_0x68 << 2);
    error_check_1000_17ce(local_BX_14.field_0x64);
    local_BX_14.field_0x60 = 0;
    local_BX_14.field_0x64 = 0;
    return;
}

pub fn win_cleanup_1010_305a(param_1: *mut astruct_318, param_2: i32, param_3: u32) {
    let puVar1: *mut u16;
    let piVar2: *mut i32;
    let mut uVar3: u32;
    let mut bVar4: bool;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar10 = (param_3 >> 0x10);
    uVar6 = pass1_1040_c60e(param_3);
    uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    (iVar7 + 0x12) = uVar6;
    (iVar7 + 0x14) = 0;
    local_6._0_2_ = 0;
    bVar4 = false;
    (iVar7 + 0x28) = 0;
    local_8 = 0;
    loop {
        puVar1 = (iVar7 + 0x16);
        unsafe {
            if (*puVar1 == local_8 || *puVar1 < local_8) {
                // LAB_1010_30ad:
                local_8 = local_6;
                if (bVar4) {
                    while (
                        local_8 = local_8 + 1,
                        puVar1 = (iVar7 + 0x16),
                        *puVar1 != local_8 && local_8 <= *puVar1,
                    ) {
                        uVar3 = (iVar7 + 0x2a + local_8 * 4);
                        DestroyWindow16((uVar3 + 0x18));
                        (iVar7 + local_8 * 4 + 0x2a) = 0;
                    }
                    (iVar7 + 0x16) = local_6 + 1;
                    pass1_1010_1f62(param_1, 9);
                } else {
                    iVar8 = (iVar7 + 0x16) * 4;
                    (iVar7 + iVar8 + 0x2a) = param_3;
                    (iVar7 + iVar8 + 0x2c) = uVar10;
                    local_a = 10;
                    piVar2 = (iVar7 + 0x16);
                    *piVar2 = *piVar2 + 1;
                    if (1 < (iVar7 + 0x16)) {
                        uVar3 = (iVar7 + (iVar7 + 0x16) * 4 + 0x22);
                        iVar8 = uVar3;
                        uVar5 = (uVar3 >> 0x10);
                        local_a = (iVar8 + 0x20) + (iVar8 + 0x24) + 8;
                    }
                    update_window_1040_93aa(param_3, uVar10, local_a, (iVar7 + 0x1a));
                }
                if (!bVar4) {
                    pass1_1010_1f62(param_1, 10);
                }
                if (param_2 == 0) {
                    if ((iVar7 + 0x52) != 0) {
                        local_8 = 0;
                        while {
                            uVar3 = (iVar7 + 0x52);
                            uVar10 = (uVar3 >> 0x10);
                            iVar8 = uVar3;
                            if (((iVar8 + local_8 * 4) != 0) && ((iVar8 + local_8 * 4) != param_3))
                            {
                                uVar3 = (iVar7 + 0x52);
                                uVar3 = (uVar3 + local_8 * 4);
                                DestroyWindow16((uVar3 + 0x18));
                            }
                            uVar3 = (iVar7 + 0x52);
                            (uVar3 + local_8 * 4) = 0;
                            local_8 = local_8 + 1;
                            local_8 < 10
                        } {}
                    }
                    pass1_1010_32da(param_1, param_3);
                    pass1_1010_1f62(param_1, 8);
                }
                return;
            }
        }
        if ((iVar7 + 0x2a + local_8 * 4) == param_3) {
            local_6._0_2_ = local_8;
            bVar4 = true;
            // goto LAB_1010_30ad;
        }
        local_8 = local_8 + 1;
    }
}

pub fn destroy_win_1010_3202(param_1: *mut astruct_387, param_2: i32) {
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_4: u16;

    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    unsafe {
        if (param_2 == 0) {
            piVar1 = (iVar3 + 0x28);
            *piVar1 = *piVar1 + -10;
            if (*piVar1 < 0) {
                (iVar3 + 0x28) = 0;
            }
        } else {
            piVar1 = (iVar3 + 0x28);
            *piVar1 = *piVar1 + (iVar3 + 0x18);
        }
        if ((iVar3 + 0x52) != 0) {
            local_4 = 0;
            while {
                uVar2 = (iVar3 + 0x52);
                if ((uVar2 + local_4 * 4) != 0) {
                    uVar2 = (iVar3 + 0x52);
                    uVar2 = (uVar2 + local_4 * 4);
                    DestroyWindow16((uVar2 + 0x18));
                    uVar2 = (iVar3 + 0x52);
                    (uVar2 + local_4 * 4) = 0;
                }
                local_4 = local_4 + 1;
                local_4 < 10
            } {}
        }
        if ((iVar3 + 0x16) == 0) {
            win_gui_fn_1010_32f4(param_1, (iVar3 + 0x56));
        } else {
            pass1_1010_32da(param_1, (iVar3 + (iVar3 + 0x16) * 4 + 0x26));
        }
    }
    pass1_1010_1f62(param_1, 8);
    return;
}

pub fn cleanup_1040_97da(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    free_proc_inst_1040_911e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_83e6(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_782c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn destroy_window_1040_8212(param_1: *mut astruct_53) {
    let is_window_result: bool;
    let local_struct_1: *mut astruct_53;
    let local_struct_1_hi: *mut astruct_53;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.win_handle_0x8c != 0) {
        is_window_result = IsWindow16(local_struct_1.win_handle_0x8c);
        if (is_window_result != 0) {
            DestroyWindow16(local_struct_1.win_handle_0x8c);
            local_struct_1.win_handle_0x8c = 0;
        }
    }
    return;
}

pub fn win_cleanup_func_1040_782c(param_1: *mut astruct_599) -> u8 {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let HVar4: HANDLE16;
    let local_BX_5: *mut astruct_46;
    let mut uVar5: u16;
    let mut unaff_CS: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.field_0x0 = 0x840c;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1040;
    puVar1 = local_BX_5.field_0x70;
    uVar2 = local_BX_5.field_0x72;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    if (local_BX_5.field_0x4 != 0) {
        unaff_CS = offset;
        DeleteObject16(local_BX_5.field_0x4);
        local_BX_5.field_0x4 = 0;
    }
    if (local_BX_5.field_0x68 != 0) {
        DestroyMenu16(unaff_CS);
    }
    RemoveProp16(s_thisLo_1050_5db1, local_BX_5.h_wnd);
    RemoveProp16(s_thisHi_1050_5db8, local_BX_5.h_wnd);
    RemoveProp16(s_procLo_1050_5dbf, local_BX_5.h_wnd);
    HVar4 = RemoveProp16(s_procHi_1050_5dc6, local_BX_5.h_wnd);
    param_1.field_0x0 = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return HVar4;
}

pub fn ret_1040_78de() -> u8 {
    let in_AL: u8;

    return in_AL;
}

pub fn destroy_win_1040_52c0(
    param_1: *mut astruct_124,
    param_2: u16_00,
    param_3: u16,
    param_4: u32,
) {
    let mut uVar1: u16;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut iVar4: i32;
    let BVar5: bool;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut h_wnd: HWND16;
    let mut uVar8: u16;
    let mut unaff_SS: HWND16;
    let ppVar9: *mut pass1_struct_1;
    let puVar10: *mut u8;
    let uVar11: u8;
    let uVar12: u8;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ != 0x10c) {
        if (param_2._2_2_ < 0x10d) {
            if (param_2._2_2_ == 0xfa) {
                ppcVar2 = (&param_1[1].field_0x4 + 0x18);
                ppcVar2();
                return;
            }
            if (param_2._2_2_ == 0x10a) {
                GetClientRect16(CONCAT22(unaff_SS, &local_a), &param_1.field_0x6);
                uVar3 = &param_1[1].field_0x4;
                local_8 = local_8 + 3;
                local_a = (uVar3 + 0x1a) - 9;
                local_6 = local_6 - 3;
                local_4 = local_4 - 3;
                InvalidateRect16(1, &local_a, unaff_SS);
                destroy_win_1010_2fa0(&param_1[1].field_0x4);
                pass1_1010_32c0(&param_1[1].field_0x4, 0);
                uVar3 = &param_1[1].field_0x4;
                local_22 = (uVar3 >> 0x10);
                pass1_1010_2ee2(uVar3, local_22);
                return;
            }
            if (param_2._2_2_ != 0x10b) {
                // LAB_1040_5560:
                win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                return;
            }
            uVar3 = &param_1[1].field_0x4;
            uVar6 = (uVar3 + 0x12);
            uVar7 = uVar6;
            ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar6, 3));
            uVar8 = (ppVar9 >> 0x10);
            iVar4 = pass1_1010_a5ca(ppVar9, uVar8, uVar7);
            if ((uVar6 != 0x70) && (iVar4 == 0)) {
                return;
            }
            uVar3 = &param_1[1].field_0x1c;
            uVar13 = uVar3;
            uVar14 = (uVar3 >> 0x10);
            uVar3 = &param_1[1].field_0x4;
            uVar1 = (uVar3 + 0x12);
            uVar11 = uVar1;
            uVar12 = (uVar1 >> 8);
        } else {
            if (param_2._2_2_ != 0x10d) {
                if (param_2._2_2_ == 0x10e) {
                    ppVar9 = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(local_22, 0x32),
                    );
                    iVar4 = win_gui_fn_1010_79aa(ppVar9, 0xfc6, &param_1[1].field_0x1c);
                    if (iVar4 != 0) {
                        return;
                    }
                    uVar3 = &param_1[1].field_0x1c;
                    window_msg_func_1010_7300(ppVar9, 0, 0, 0x13, uVar3, (uVar3 >> 0x10));
                    return;
                }
                if (param_2._2_2_ != 0xbbb) {
                    if (param_2._2_2_ == 0xbbc) {
                        ppVar9 = process_struct_1010_20ba(
                            _g_astruct_372_1050_0ed0,
                            CONCAT22(local_22, 3),
                        );
                        uVar8 = (ppVar9 >> 0x10);
                        uVar6 = ppVar9;
                        uVar7 = pass1_1010_a5ac(uVar6, uVar8, &param_1[1].field_0x1c);
                        iVar4 = pass1_1010_a58a(uVar6, uVar8, uVar7);
                        if (iVar4 == 0) {
                            pass1_1010_a568(uVar6, uVar8, uVar7);
                        }
                        h_wnd = GetDlgItem16(0xbbc, &param_1.field_0x6);
                        EnableWindow16(0, h_wnd);
                        return;
                    }
                    // goto LAB_1040_5560;
                }
                if ((&param_1[1].field_0x22 == 0)
                    || (BVar5 = IsWindow16(&param_1[1].field_0x22), BVar5 == 0))
                {
                    puVar10 = pass1_1038_af40(_g_astruct_112_a, *&param_1.field_0x6, 0x1b);
                    &param_1[1].field_0x22 = (puVar10 + 6);
                    set_window_pos_1038_abdc(puVar10);
                    ShowWindow16(1, &param_1[1].field_0x22);
                    return;
                }
                local_22 = &param_1[1].field_0x22;
                // goto LAB_1040_5417;
            }
            ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_22, 3));
            uVar8 = (ppVar9 >> 0x10);
            uVar3 = &param_1[1].field_0x1c;
            uVar13 = uVar3;
            uVar14 = (uVar3 >> 0x10);
            uVar11 = 0x71;
            uVar12 = 0;
        }
        local_1e = ppVar9;
        pass1_1010_a5ec(
            local_1e,
            uVar8,
            CONCAT11(uVar12, uVar11),
            CONCAT22(uVar14, uVar13),
        );
        if ((&param_1[1].field_0x20 != 0)
            && (BVar5 = IsWindow16(&param_1[1].field_0x20), BVar5 != 0))
        {
            SendMessage16(0, 0xeb, 0x111, &param_1[1].field_0x20);
        }
    }
    local_22 = &param_1.field_0x6;
    // LAB_1040_5417:
    DestroyWindow16(local_22);
    return;
}

pub fn destroy_win_1040_5256(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let BVar4: bool;
    let local_BX_5: *mut astruct_52;
    let mut uVar5: u16;
    let mut unaff_CS: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (local_BX_5.h_wnd != 0) {
        unaff_CS = SUB42(offset, 0);
        BVar4 = IsWindow16(local_BX_5.h_wnd);
        if (BVar4 != 0) {
            unaff_CS = SUB42(offset, 0);
            DestroyWindow16(local_BX_5.h_wnd);
        }
    }
    local_BX_5.h_wnd = 0;
    puVar1 = local_BX_5.field_0x94;
    uVar2 = &local_BX_5.field_0x96;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
        }
    }
    &local_BX_5.field_0x94 = 0;
    local_BX_5.field_0x98 = 0;
    return;
}

pub fn pass1_1040_3506(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = (s_Null_Ptr_1050_38f3 + 7);
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_3532(param_1: *mut u8, param_2: *mut u8) {
    let local_DXAX_12: *mut pass1_struct_1;
    let BVar1: bool;

    BVar1 = 0;
    local_DXAX_12 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x2b);
    pass1_1010_038e(local_DXAX_12, BVar1);
    destroy_win_1040_7b98(param_1, param_2);
    return;
}

pub fn pass1_1040_2f06(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x3436;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_2f32(param_1: *mut u8, param_2: *mut u8) {
    let paVar1: *mut astruct_318;
    let BVar2: bool;

    BVar2 = 0;
    paVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x2b);
    pass1_1010_038e(paVar1, BVar2);
    destroy_win_1040_7b98(param_1, param_2);
    return;
}

pub fn pass1_1040_2a22(param_1: *mut astruct_44) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.ptr_a_lo = (s_add74_wav_1050_2e20 + 6);
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    error_check_1000_17ce((iVar1 + 0x94));
    error_check_1000_17ce((iVar1 + 0x98));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn pass1_1040_2464(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = (s_fem94_wav_1050_2950 + 6);
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_205e(param_1: *mut astruct_599) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field_0x0 = (s_alarm_m_1050_2377 + 7);
    (iVar4 + 2) = &PTR_LOOP_1050_1040;
    puVar1 = (iVar4 + 0x8e);
    uVar2 = (iVar4 + 0x90);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    error_check_1000_17ce((iVar4 + 0xa2));
    error_check_1000_17ce((iVar4 + 0xa6));
    pass1_1038_b6e0(_g_astruct_112_a, *(iVar4 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_1876(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = (s_202_flc_1050_1c46 + 2);
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_1290(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x17b0;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_1152(param_1: i32, param_2: *mut u8, param_3: *mut u8) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000fff4: u16;
    let mut local_4: u16;

    if ((param_1 + 0x92) != 0) {
        uVar2 = (param_1 + 0x8e);
        uVar1 = (uVar2 + 10);
        ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff4, 3));
        uVar2 = (param_1 + 0x92);
        uVar4 = (uVar2 >> 0x10);
        iVar3 = uVar2;
        pass1_1010_ae92(ppVar5, uVar1, (iVar3 + 10), (iVar3 + 6));
    }
    destroy_win_1040_7b98(param_1, param_2, param_3);
    PTR_LOOP_1050_5b80 = 0x0;
    return;
}

pub fn pass1_1040_0e86(param_1: *mut astruct_599) {
    let mut uVar1: i32;
    let in_struct_1: *mut astruct_44;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000ffee: u16;
    let mut local_6: u32;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    param_1.field_0x0 = (s_overflow_on_node__d_1050_11ca + 8);
    (iVar2 + 2) = &PTR_LOOP_1050_1040;
    in_struct_1 = (iVar2 + 0x92);
    uVar1 = (iVar2 + 0x94);
    if ((uVar1 | in_struct_1) != 0) {
        pass1_1040_a5d0((in_struct_1 & 0xffff | uVar1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    PTR_LOOP_1050_5b82 = (iVar2 + 0x96);
    if ((iVar2 + 0x92) == 0) {
        pass1_1038_b6e0(_g_astruct_112_a, *(iVar2 + 6));
    } else {
        ppVar4 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffee, 0x32));
        pass1_1010_7b8c(ppVar4, (iVar2 + 6));
    }
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_0c54(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xdb0;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    (param_1 + 0x8e) = 0;
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_073a(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xb90;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1040_0656(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    win_cleanup_1038_ef3a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn win_cleanup_1038_ef3a(param_1: *mut astruct_599) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    param_1.field_0x0 = 0x67c;
    (iVar2 + 2) = &PTR_LOOP_1050_1040;
    if ((iVar2 + 0x96) != 0) {
        uVar1 = (iVar2 + 0x96);
        DestroyWindow16((uVar1 + 6));
        (iVar2 + 0x96) = 0;
    }
    pass1_1038_b6e0(_g_astruct_112_a, *(iVar2 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_ebd6(param_1: *mut astruct_599) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field_0x0 = 0xee6e;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(iVar1 + 6));
    error_check_1000_17ce((iVar1 + 0x8e));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_e308(param_1: *mut astruct_599) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field_0x0 = 0xe62e;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(iVar1 + 6));
    error_check_1000_17ce((iVar1 + 0x8e));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_e16e(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xe264;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_d7d0(param_1: *mut astruct_45) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.u16_0x0 = 0xe0d4;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    if ((iVar1 + 0x90) != 0) {
        pass1_1010_1ea6(_g_struct_ptr_1050_02a0, param_1);
    }
    if ((iVar1 + 0x92) != 0) {
        pass1_1010_1ea6(*(iVar1 + 0x92), param_1);
    }
    pass1_1038_b6e0(_g_astruct_112_a, *(iVar1 + 6));
    error_check_1000_17ce((iVar1 + 0x96));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_d276(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd6ea;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_d218(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    free_proc_inst_1038_cfda(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn free_proc_inst_1038_cfda(param_1: *mut u16) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut unaff_CS: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    unsafe { *param_1 = 0xd23e };
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    FreeProcInstance16(CONCAT22((iVar1 + 4), unaff_CS));
    FreeProcInstance16(CONCAT22(_PTR_LOOP_1050_5bcc, 0x1538));
    (iVar1 + 4) = 0;
    unsafe { *param_1 = s_1_1050_389a };
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1038_cd5c(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xcf00;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_cb30(param_1: *mut astruct_599) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xcc9a;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn cleanup_fn_1020_0e2c(param_1: *mut astruct_48) {
    get_sys_metrics_1020_7a50(param_1);
    destroy_icon_func_1020_1038(param_1);
    return;
}

pub fn pass1_1038_ae08(param_1: *mut astruct_44) {
    param_1.ptr_a_lo = 0xae4e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_a156(param_1: *mut astruct_599) {
    param_1.field_0x0 = 0xa2d0;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn destroy_win_1038_a072(param_1: u32, param_2: i32) {
    if (param_2 != 0) {
        DestroyWindow16((param_1 + 6));
    }
    return;
}

pub fn pass1_1038_9ed4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_9a48(param_1: *mut astruct_44) {
    param_1.ptr_a_lo = 0x9af6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_997c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_7d5c(param_1: *mut astruct_44) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x8876;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn destroy_win_1038_7d88(param_1: u32, param_2: i32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1008_b544((param_1 + 0x94), param_2);
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn cleanup_fn_1020_96a2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    select_and_delete_palette_1020_92c4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn cleanup_fn_1020_7b60(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn cleanup_fn_1020_78ac(param_1: *mut astruct_44) {
    let local_struct_1: *mut astruct_44;
    let local_struct_1_hi: *mut astruct_44;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x7902;
    local_struct_1.ptr_a_hi = 0x1020;
    if (local_struct_1.field_0x14 != 0) {
        pass1_1010_1dda(local_struct_1.field_0x14);
    }
    select_and_delete_palette_1020_92c4(param_1);
    return;
}

pub fn cleanup_fn_1020_78dc(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    cleanup_fn_1020_78ac(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn win_cleanup_fn_1020_770e(in_struct_1: *mut astruct_594) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_struct_1: *mut astruct_594;
    let local_struct_1_hi: *mut astruct_594;
    let mut in_stack_0000fff6: u16;
    let fn_ptr_1_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    puVar1 = local_struct_1.u8_ptr_16_xee;
    uVar2 = local_struct_1.field_0xf0;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1_1 = *puVar1;
            (**fn_ptr_1_1)();
        }
    }
    &local_struct_1.u8_ptr_16_xee = 0;
    destroy_win_1008_628e(
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        in_stack_0000fff6,
    );
    return;
}

pub fn call_win_cleanup_fn_1020_3616(in_struct_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    win_cleanup_func_1020_2fea(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}
