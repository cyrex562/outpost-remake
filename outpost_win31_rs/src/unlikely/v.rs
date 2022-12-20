use crate::block_1020::block_1020_5000::{menu_ui_op_1020_5bf2, pt_in_rect_op_1020_58ce, win_ui_op_1020_5de8, win_ui_op_1020_5e76};

pub unsafe fn win_sys_op_1020_493c(
    mut param_1: u16,
    param_2: *mut StructD,
    param_3: *mut StructD,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    param_7: *mut u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut lVar3: i32;
    let mut HVar4: HCURSOR16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut paVar15: *mut Struct57;
    let mut uVar9: *mut StructD;
    let mut uVar16: u16;
    let mut uVar17: u32;
    let mut puVar18: *mut u32;
    let mut paVar19: *mut astruct_97;
    let mut pcVar20: *mut c_char;
    let mut in_stack_0000fb4e: u16;
    let mut in_stack_0000fb50: u16;
    let mut in_stack_0000fb52: u16;
    let mut in_stack_0000fc72: u16;
    let mut in_stack_0000fc74: u16;
    let mut in_stack_0000fc76: u16;
    let mut in_stack_0000fc78: u16;
    let mut in_stack_0000fc7a: u16;
    let mut in_stack_0000fc7c: u16;
    let mut in_stack_0000fc7e: u16;
    let mut in_stack_0000fc80: u16;
    let mut uStack852: u16;
    let mut local_24e: u16;
    let mut uStack588: u16;
    let mut local_144: u32;
    let mut uStack320: u32;
    let mut local_13c: u32;
    let mut uStack42: u16;
    let mut uStack38: u32;
    let mut uStack34: u16;
    let mut uStack32: u16;
    let mut uStack30: u32;
    let mut uStack26: u32;
    let mut uStack22: u32;
    let mut puStack18: *mut u32;
    let mut puStack14: *mut u8;
    let mut puStack12: *mut u8;
    let mut uStack10: u16;
    let mut uStack6: u32;

    if (param_4 == 0xe9) {
        return;
    }
    uVar9 = param_3;
    uVar13 = (param_3 >> 0x10);
    if (param_4 < 0xea) {
        match param_4 {
            0x69 => {
                iVar6 = 0;
            }
            //   break;
            0x6a => {
                iVar6 = 0x1;
            }
            //   break;
            0x6b => {
                iVar6 = 0x2;
            }
            //   break;
            0x6c => {
                iVar6 = 0x3;
            }
            //   break;
            0x6d => {
                iVar6 = 0x4;
            }
            //   break;
            _ => {
                return;
            }
            0x77 => {
                if ((&uVar9[0x1].field_0x1c | uVar9[0x1].field14_0x1a) == 0) {
                    return;
                }
                uVar2 = &uVar9[0x1].field14_0x1a;
                uVar11 = (s_VrMode_1050_42ca + 0x8 + (uVar2 + 0x2e) * 0x2);
                uStack26 = (uStack26 & 0xffff0000 | uVar11);
                if (uVar11 == 0) {
                    return;
                }
                uVar16 = FUN_1010_830a(uVar11, param_2, 0x1020, _u16_1050_14cc, 0x1f8);
                puStack18 = CONCAT22(param_2, uVar16);
                param_7 = uVar9.field5_0x8;
                WinHelp16(
                    CONCAT13(
                        (uStack26 >> 0xf),
                        CONCAT12(
                            (uStack26 >> 0xf),
                            uStack26 & 0xff | (uStack26 >> 0x8) << 0x8,
                        ),
                    ),
                    0x1,
                    CONCAT22(param_2, uVar16),
                    param_7,
                );
                return;
            }
            0x78 => {
                puVar18 = mixed_1010_20ba(
                    param_2,
                    _u16_1050_0ed0,
                    CONCAT22(param_7, 0x45),
                    in_stack_0000fb52,
                    in_stack_0000fc76,
                    in_stack_0000fc7c,
                    in_stack_0000fc80,
                );
                uStack588 = (puVar18 >> 0x10);
                local_24e = puVar18;
                enum_child_windows_1010_01be();
                return;

                set_cursor_1020_5764(param_3, iVar6);
                return;
            }
        };
    }
    if (param_4 == 0x132) {
        uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
        if (uVar10 == 0) {
            return;
        }
        uVar16 = 0xffff;
        // TODO: goto LAB_1020_4ef8;
    }
    if (param_4 < 0x133) {
        if (param_4 == 0x102) {
            uVar16 = 0x1000;
            mem_op_1000_179c(0xb4, param_2);
            uStack32 = param_2;
            uVar17 = param_2 & 0xffff0000 | (uStack32 | param_4);
            uStack34 = param_4;
            if ((uStack32 | param_4) == 0) {
                iVar6 = 0;
                uVar12 = 0;
            } else {
                uVar16 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                iVar6 = string_1040_8520(
                    uVar17,
                    CONCAT22(uStack32, param_4),
                    HWND16_1050_0396,
                    0x20031,
                    0x62b057b,
                );
                uVar12 = uVar17;
            }
            local_144 = CONCAT22(uVar12, iVar6);
            ppcVar1 = (*local_144 + 0x74);
            (**ppcVar1)(uVar16, iVar6, uVar12);
            uStack320 = CONCAT22(uStack320, iVar6);
            if (iVar6 != 1) {
                return;
            }
            pass1_1028_837e(CONCAT22(0x1050, &local_13c)); //
            // LAB_1020_4b6c:
            fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &local_13c));
            return;
        }
        if (param_4 < 0x103) {
            match param_4 {
                0xf0 => {
                    ui_op_1020_536e(param_2, param_3, 0x0, -0x1, 1);
                    return;
                }
                _ => {
                    return;
                }
                0xf6 => {
                    if (&uVar9[0x1].field_0x28 != 0) {
                        if (param_3.is_null()) {
                            param_7 = null_mut();
                            uStack852 = 0;
                        } else {
                            param_7 = &uVar9.field_0xe2;
                            uStack852 = uVar13;
                        }
                        param_2 = uStack852;
                        pass1_1010_1ea6(_u16_1050_02a0, CONCAT22(uStack852, param_7));
                        uVar9[0x1].field_0x28 = 0;
                    }
                    iVar6 = 0x12;
                }
                // break;
                0xf7 => {
                    unk_win_op_1010_7300(param_2, &uVar9[0x1].field19_0x20, 0x0, 0x9, 0x0);
                    return;
                }
                0xfb => {
                    local_144 = mixed_1010_20ba(
                        param_2,
                        _u16_1050_0ed0,
                        CONCAT22(param_7, 0x3),
                        in_stack_0000fb52,
                        in_stack_0000fc76,
                        in_stack_0000fc7c,
                        in_stack_0000fc80,
                    );
                    uStack320 = mixed_1010_20ba(
                        (param_2 & 0xffff0000 | local_144 >> 0x10),
                        _u16_1050_0ed0,
                        CONCAT22(param_7, 0x30),
                        in_stack_0000fb52,
                        in_stack_0000fc76,
                        in_stack_0000fc7c,
                        in_stack_0000fc80,
                    );
                    pcVar20 = pass1_1010_375e(uStack320);
                    pass1_1010_c25e(
                        pcVar20,
                        (pcVar20 >> 0x10),
                        local_144,
                        (local_144 >> 0x10),
                        pcVar20,
                    );
                    return;
                }
                0xfc => {
                    post_msg_1020_55b0(param_2, param_3, param_5, param_6);
                    return;
                }
                0x101 => {
                    uStack26 = mixed_1010_20ba(
                        param_2,
                        _u16_1050_0ed0,
                        CONCAT22(param_7, 0x2f),
                        in_stack_0000fb52,
                        in_stack_0000fc76,
                        in_stack_0000fc7c,
                        in_stack_0000fc80,
                    );
                    uVar16 = (uStack26 >> 0x10);
                    uStack22 = (uStack26 + 0x24);
                    uVar11 = (uStack26 + 0x26);
                    paVar15 = (param_2 & 0xffff0000 | uVar11);
                    uStack22 = uVar11 | uStack22;
                    if (uStack22 == 0) {
                        uVar16 = 0x1000;
                        mem_op_1000_179c(0xb4, paVar15);
                        uStack32 = paVar15;
                        uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack22);
                        uStack34 = uStack22;
                        if ((uStack32 | uStack22) == 0) {
                            puVar8 = null_mut();
                            uVar11 = 0;
                        } else {
                            uVar16 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                            puVar8 = string_1040_8520(
                                uVar17,
                                CONCAT22(uStack32, uStack22),
                                HWND16_1050_0396,
                                0x20030,
                                0x730057b,
                            );
                            uVar11 = uVar17;
                        }
                        uStack30 = CONCAT22(uVar11, puVar8); //
                        // LAB_1020_4c5f:
                        ppcVar1 = (*puVar8 + 0x74);
                        (**ppcVar1)(uVar16, puVar8, uVar11);
                        return;
                    }
                    uVar17 = pass1_1038_af40(uVar9, uVar11, _PTR_LOOP_1050_5b7c, uVar9.field5_0x8, 0xe);
                    puStack18 = mixed_1010_20ba(
                        (paVar15 & 0xffff0000 | uVar17 >> 0x10),
                        _u16_1050_0ed0,
                        CONCAT22(param_7, 0x43),
                        in_stack_0000fb52,
                        in_stack_0000fc76,
                        in_stack_0000fc7c,
                        in_stack_0000fc80,
                    );
                    uVar16 = (puStack18 >> 0x10);
                    iVar6 = puStack18;
                    puStack14 = (iVar6 + 0xa);
                    uStack10 = (iVar6 + 0xc);
                    uVar13 = (iVar6 + 0xe);
                    uStack6 = CONCAT22(uStack6, uVar13);
                    if ((iVar6 + 0x10) != 0) {
                        return;
                    }
                    pass1_1028_84ca(
                        CONCAT22(0x1050, &local_13c),
                        uStack22,
                        uVar13,
                        uStack10,
                        puStack14,
                    );
                } // TODO: goto LAB_1020_4b6c;
            };
        } else {
            if (param_4 != 0x106) {
                if (param_4 < 0x107) {
                    if (param_4 == 0x103) {
                        local_144 = mixed_1010_20ba(
                            param_2,
                            _u16_1050_0ed0,
                            CONCAT22(param_7, 0x2f),
                            in_stack_0000fb52,
                            in_stack_0000fc76,
                            in_stack_0000fc7c,
                            in_stack_0000fc80,
                        );
                        uVar16 = (local_144 >> 0x10);
                        uStack320 = *(local_144 + 0x24);
                        uVar11 = (local_144 + 0x26);
                        paVar15 = (param_2 & 0xffff0000 | uVar11);
                        uStack34 = uVar11 | uStack320;
                        if (uStack34 != 0) {
                            uVar17 = pass1_1038_af40(
                                uVar9,
                                uVar11,
                                _PTR_LOOP_1050_5b7c,
                                uVar9.field5_0x8,
                                0xf,
                            );
                            local_13c = mixed_1010_20ba(
                                (paVar15 & 0xffff0000 | uVar17 >> 0x10),
                                _u16_1050_0ed0,
                                CONCAT22(param_7, 0x42),
                                in_stack_0000fb52,
                                in_stack_0000fc76,
                                in_stack_0000fc7c,
                                in_stack_0000fc80,
                            );
                            uStack42 = (local_13c + 0xa);
                            if (uStack42 == 0) {
                                return;
                            }
                            pass1_1030_e63e(CONCAT22(0x1050, &local_24e), uStack42);
                            fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &local_24e));
                            return;
                        }
                        uVar16 = 0x1000;
                        mem_op_1000_179c(0xb4, paVar15);
                        uStack32 = paVar15;
                        uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack34);
                        if ((uStack32 | uStack34) == 0) {
                            puVar8 = null_mut();
                            uVar11 = 0;
                        } else {
                            uVar16 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                            puVar8 = string_1040_8520(
                                uVar17,
                                CONCAT22(uStack32, uStack34),
                                HWND16_1050_0396,
                                0x20030,
                                0x730057b,
                            );
                            uVar11 = uVar17;
                        }
                        uStack38 = CONCAT22(uVar11, puVar8);
                    } else {
                        if (param_4 != 0x104) {
                            return;
                        }
                        uVar16 = 0x22;
                        puVar18 = mixed_1010_20ba(
                            param_2,
                            _u16_1050_0ed0,
                            0x220003,
                            in_stack_0000fb50,
                            in_stack_0000fc74,
                            in_stack_0000fc7a,
                            in_stack_0000fc7e,
                        );
                        paVar15 = (param_2 & 0xffff0000 | puVar18 >> 0x10);
                        uStack34 = puVar18;
                        uStack588 = (puVar18 >> 0x10);
                        local_24e = uStack34;
                        pass1_1010_af66(uStack588, puVar18, uVar16);
                        local_144 = CONCAT22(local_144, uStack34);
                        if (uStack34 != 0) {
                            uVar16 = 0x1000;
                            mem_op_1000_179c(0xb4, paVar15);
                            uStack32 = paVar15;
                            uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack34);
                            if ((uStack32 | uStack34) == 0) {
                                iVar6 = 0;
                                uVar12 = 0;
                            } else {
                                uVar16 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                                iVar6 = string_1040_8520(
                                    uVar17,
                                    CONCAT22(uStack32, uStack34),
                                    HWND16_1050_0396,
                                    0x20031,
                                    0x62c057b,
                                );
                                uVar12 = uVar17;
                            }
                            uStack320 = CONCAT22(uVar12, iVar6);
                            ppcVar1 = (*uStack320 + 0x74);
                            (**ppcVar1)(uVar16, iVar6, uVar12);
                            local_13c = CONCAT22(local_13c, iVar6);
                            if (iVar6 != 1) {
                                return;
                            }
                            paVar19 = pass1_1030_e79a(CONCAT22(0x1050, &param_7));
                            uVar13 = (paVar19 >> 0x10);
                            puVar7 = &param_7;
                            fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, puVar7));
                            win_1008_5c5c(puVar7, uVar13, _u16_1050_02a0, 0x1e6);
                            return;
                        }
                        uVar16 = 0x1000;
                        mem_op_1000_179c(0xb4, paVar15);
                        uStack32 = paVar15;
                        uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack34);
                        if ((uStack32 | uStack34) == 0) {
                            puVar8 = null_mut();
                            uVar11 = 0;
                            param_7 = puVar8;
                            uStack852 = uVar11;
                        } else {
                            uVar16 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                            puVar8 = string_1040_8520(
                                uVar17,
                                CONCAT22(uStack32, uStack34),
                                HWND16_1050_0396,
                                0x20030,
                                0x731057b,
                            );
                            uVar11 = uVar17;
                            param_7 = puVar8;
                            uStack852 = uVar11;
                        }
                    }
                    // TODO: goto LAB_1020_4c5f;
                }
                if (param_4 == 0x12f) {
                    pass1_1020_61c4(
                        uVar9,
                        uVar13,
                        CONCAT22(0x1050, &local_144),
                        CONCAT22(0x1050, &local_24e),
                    );
                    iVar6 = local_24e + 0x6a;
                } else {
                    if (param_4 != 0x130) {
                        if (param_4 != 0x131) {
                            return;
                        }
                        uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x2);
                        if (uVar10 == 0) {
                            return;
                        }
                        iVar6 = 0x7;
                        // TODO: goto LAB_1020_49b7;
                    }
                    pass1_1020_61c4(
                        uVar9,
                        uVar13,
                        CONCAT22(0x1050, &local_144),
                        CONCAT22(0x1050, &local_24e),
                    );
                    iVar6 = local_24e + 0x68;
                }
                uStack320 = CONCAT22(uStack320, iVar6);
                if (0x6d < iVar6) {
                    return;
                }
                if (iVar6 < 0x69) {
                    return;
                }
                ppcVar1 = (param_3 + 0x40);
                (**ppcVar1)();
                return;
            }
            iVar6 = 0x13;
        } //
        // LAB_1020_49b7:
        pass1_1038_af40(uVar9, param_2, _PTR_LOOP_1050_5b7c, uVar9.field5_0x8, iVar6);
        return;
    }
    if (param_4 == 0x1c8) {
        lVar3 = uVar9[0x1].field12_0x14;
        SendMessage16(0x0, 0x72, 0x111, (lVar3 + 0x8));
        return;
    }
    if (0x1c8 < param_4) {
        if (param_4 == 0x1ca) {
            local_144 = mixed_1010_20ba(
                param_2,
                _u16_1050_0ed0,
                CONCAT22(param_7, 0x3),
                in_stack_0000fb52,
                in_stack_0000fc76,
                in_stack_0000fc7c,
                in_stack_0000fc80,
            );
            uVar17 = param_2 & 0xffff0000;
            uStack320 = pass1_1010_c234(local_144, (local_144 >> 0x10));
            uVar11 = uStack320;
            uVar14 = (uStack320 >> 0x10);
            if ((uVar14 | uVar11) == 0) {
                return;
            }
            local_13c = mixed_1010_20ba(
                (uVar17 & 0xffff0000 | (uVar14 | uVar11)),
                _u16_1050_0ed0,
                CONCAT22(uVar11, 0x30),
                in_stack_0000fb4e,
                in_stack_0000fc72,
                in_stack_0000fc78,
                in_stack_0000fc7c,
            );
            param_2 = (local_13c >> 0x10);
            pass1_1010_3770((local_13c >> 0x10), local_13c, CONCAT22(uVar14, uVar11));
            iVar6 = 0x3;
        } else if (param_4 == 0x200) {
            uVar2 = &uVar9[0x1].field14_0x1a;
            uVar16 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            uStack26 = (iVar6 + 0x42);
            uVar11 = (iVar6 + 0x44);
            param_2 = uVar11;
            uStack26._3_1_ = (uStack26 >> 0x18);
            puStack14 = uStack26._3_1_;
            if (uStack26._3_1_ != 0x5) {
                return;
            }
            pass1_1030_8344(_u16_1050_5748, uStack26 & 0xffff | uVar11 << 0x10);
            PTR_LOOP_1050_5f0e = param_2;
            iVar6 = 0x25;
            PTR_LOOP_1050_5f0c = puStack14;
            puStack12 = PTR_LOOP_1050_5f0e;
        } else if (param_4 == 0x201) {
            uVar2 = &uVar9[0x1].field14_0x1a;
            uVar16 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            uStack26 = (iVar6 + 0x42);
            uVar11 = (iVar6 + 0x44);
            param_2 = uVar11;
            uStack26._3_1_ = (uStack26 >> 0x18);
            puStack14 = uStack26._3_1_;
            if (uStack26._3_1_ != 0x5) {
                return;
            }
            pass1_1030_8344(_u16_1050_5748, uStack26 & 0xffff | uVar11 << 0x10);
            PTR_LOOP_1050_5f18 = param_2;
            iVar6 = 0x26;
            PTR_LOOP_1050_5f16 = puStack14;
            puStack12 = PTR_LOOP_1050_5f18;
        } else {
            if (param_4 != 0x202) {
                if (param_4 != 0x203) {
                    return;
                }
                if (&uVar9[0x1].field_0x6 != 1) {
                    return;
                }
                HVar4 = SetCursor16(uVar9[0x1].hfile_0x4);
                (uVar9 + 1).address_offset_field_0x0 = HVar4;
                uVar9[0x1].field_0x6 = 0x3;
                param_7 = uVar9.field5_0x8;
                SetCapture16(param_7);
                return;
            }
            uVar2 = &uVar9[0x1].field14_0x1a;
            uVar16 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            uStack6 = (iVar6 + 0x42);
            uVar11 = (iVar6 + 0x44);
            param_2 = uVar11;
            uStack6._3_1_ = (uStack6 >> 0x18);
            uVar5 = uStack6._3_1_;
            if (uStack6._3_1_ != 0x5) {
                return;
            }
            pass1_1030_8344(_u16_1050_5748, uStack6 & 0xffff | uVar11 << 0x10);
            PTR_LOOP_1050_5a6a = param_2;
            uStack22 = CONCAT22(PTR_LOOP_1050_5a6a, uVar5);
            iVar6 = 0x27;
            u16_1050_5a68 = uVar5;
        }
        // TODO: goto LAB_1020_49b7;
    }
    match param_4 {
        0x133 => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
            if (uVar10 == 0) {
                return;
            }
            uVar12 = 0xffff;
            uVar16 = 0;
        }
        // break;
        0x134 => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
            if (uVar10 == 0) {
                return;
            }
            uVar16 = 0x1;
        }
        // TODO: goto LAB_1020_4ef8;
        0x135 => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
            if (uVar10 == 0) {
                return;
            }
            uVar12 = 0x1;
            uVar16 = 0;
        }
        // break;
        0x136 => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
            if (uVar10 == 0) {
                return;
            }
            uVar16 = 0xfffb;
        }
        // TODO: goto LAB_1020_4ef8;
        0x137 => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
            if (uVar10 == 0) {
                return;
            }
            uVar12 = 0xfffb;
            uVar16 = 0;
        }
        // break;
        0x138 => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
            if (uVar10 == 0) {
                return;
            }
            uVar16 = 0x5; //
            // LAB_1020_4ef8:
            uVar12 = 0;
        }
        // break;
        0x139 => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x3);
            if (uVar10 == 0) {
                return;
            }
            uVar12 = 0x5;
            uVar16 = 0;
        }
        // break;
        _ => {}
        // TODO: goto switchD_1020_518a_caseD_13a;
        0x13c => {
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8, 0x2);
            if (uVar10 != 0) {
                iVar6 = 0x1a;
                // TODO: goto LAB_1020_49b7;
            }
        } // TODO: goto switchD_1020_518a_caseD_13a;
    };
    pass1_1020_2a94(&uVar9.field_0xce, CONCAT22(uVar16, uVar12));
    // switchD_1020_518a_caseD_13a:
    return;
}

pub unsafe fn pass1_1020_51c6(mut param_1: u32, mut param_2: u16, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut in_DX: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = (param_1 + 0xf4);
    uVar4 = param_3;
    if (iVar2 == 0x2) {
        win_ui_op_1020_5e76(param_1 & 0xffff | uVar3 << 0x10, param_2, uVar4);
        return;
    }
    iVar2 += -0x3;
    if (iVar2 != 0) {
        pt_in_rect_op_1020_58ce(
            in_DX,
            param_1 & 0xffff | uVar3 << 0x10,
            param_2,
            uVar4,
            (param_3 >> 0x10),
        );
        if (iVar2 == 0) {
            ppcVar1 = ((param_1 + 0x4) + 0x5c);
            (**ppcVar1)();
        }
        return;
    }
    win_ui_op_1020_5de8(param_1 & 0xffff | uVar3 << 0x10, param_2, uVar4);
    return;
}

pub unsafe fn win_ui_cursor_op_1020_522e(
    mut param_1: u16,
    param_2: *mut astruct_52,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut BVar3: bool;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar4: *mut astruct_52;
    let mut uVar4: *mut astruct_52;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut uVar6: u8;
    let mut uVar7: u8;
    let mut uVar8: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    iVar1 = iVar4.field242_0xf4;
    if (iVar1 == 0x2) {
        SetCursor16(iVar4.field237_0xee);
        iVar4.field237_0xee = 0;
        iVar4.field242_0xf4 = 0x1;
        (iVar4 + 1) = 0;
        ReleaseCapture16();
        return;
    }
    if (iVar1 == 0x3) {
        SetCursor16(iVar4.field237_0xee);
        iVar4.field237_0xee = 0;
        iVar4.field242_0xf4 = 0x1;
        (iVar4 + 1) = 0;
        ReleaseCapture16();
        uVar6 = 0;
        uVar7 = 0;
        uVar8 = 0;
        puVar5 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            0x47,
            in_stack_0000fea0,
            in_stack_0000ffc4,
            in_stack_0000ffca,
            in_stack_0000ffce,
        );
        pass1_1018_57e6(
            puVar5,
            CONCAT22(uVar8, CONCAT11(uVar7, uVar6)),
            puVar5,
            (puVar5 >> 0x10),
        );
        return;
    }
    BVar3 = menu_ui_op_1020_5bf2(param_2, param_3, param_4);
    if (BVar3 == 0) {
        ppcVar2 = (*&iVar4.field_0x4 + 0x60);
        (**ppcVar2)();
    }
    return;
}


pub unsafe fn pass1_1020_52de(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    puVar1 = (iVar6 + 0xf6);
    uVar2 = (iVar6 + 0xf8);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar6 + 0xf6) = 0;
    if ((iVar6 + 0xfa) != 0) {
        if (param_1 == 0) {
            iVar4 = 0;
            uVar5 = 0;
        } else {
            iVar4 = iVar6 + 0xe2;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6((iVar6 + 0xfa), CONCAT22(uVar5, iVar4));
    }
    destroy_win_1008_628e(param_1);
    if ((iVar6 + 0xfa) != 0) {
        pass1_1010_1dda((iVar6 + 0xfa));
    }
    (iVar6 + 0xfa) = 0;
    return;
}

pub unsafe fn pass1_1020_6208(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    destroy_cursor_1020_42f4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1020_6216(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    destroy_cursor_1020_42f4(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn mix_draw_op_1020_650c(param_1: *mut StructA) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let struct_1: *mut StructA;
    let mut uVar3: u16;
    let mut local_28: [u8; 0x20] = [0; 0x20];
    let mut iStack8: i16;
    let mut puStack6: *mut u32;

    uVar3 = (param_1 >> 0x10);
    struct_1 = param_1;
    uVar2 = &struct_1.field10_0x14;
    puStack6 = (uVar2 + 0xa);
    if ((struct_1.field8_0x10 != 0) || (uVar2 = &struct_1.field10_0x14, (uVar2 + 0x24) != 0)) {
        draw_op_1020_9364(param_1);
        if (&struct_1.field19_0x24 != 0) {
            uVar2 = &struct_1.field19_0x24;
            ppcVar1 = (*&struct_1.field19_0x24 + 0x14);
            (**ppcVar1)(0x1020, uVar2, (uVar2 >> 0x10));
        }
    }
    iStack8 = 0;
    loop {
        if ((&struct_1.field12_0x18 + iStack8 * 0x2) != 0) {
            uVar2 = (&struct_1.field12_0x18 + iStack8 * 0x2);
            ppcVar1 = (*(&struct_1.field12_0x18 + iStack8 * 0x2) + 0x8);
            (**ppcVar1)(0x1020, uVar2, (uVar2 >> 0x10), puStack6, (puStack6 >> 0x10));
        }
        iStack8 += 0x1;
        if iStack8 >= 0x5 {
            break;
        }
    }
    BeginPaint16(CONCAT22(0x1050, local_28), struct_1.field2_0x4);
    ppcVar1 = (*puStack6 + 0x4);
    (**ppcVar1)(
        s_tile2_bmp_1050_1538,
        puStack6,
        (puStack6 >> 0x10),
        0x0,
        0x0,
        &struct_1.field5_0xa,
        uVar3,
    );
    EndPaint16(CONCAT22(0x1050, local_28), struct_1.field2_0x4);
    return;
}
