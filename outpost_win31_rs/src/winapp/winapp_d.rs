use std::ptr::null_mut;
use std::os::raw::c_char;
use crate::resources::load_string_1010_84e0;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_a::StructA;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1018_5000::pass1_1018_57e6;
use crate::utils::{CONCAT22, SUB42};
use crate::winapi16::{CallWindowProc16, GetClientRect16, GetDC16, GetProp16, GetWindowText16, IsWindow16, PostMessage16, WinAPI16_PtInRect16, ReleaseCapture16, SendMessage16, SetCapture16, SetCursor16, SetDlgItemText16, SetWindowLong16, SetWindowText16, WinHelp16};
use crate::{gui, winapp};
use crate::draw_ops::draw_a::palette_op_1008_4e08;
use crate::gui::cursor;
use crate::gui::window::win_c;
use crate::no_refs::l::{pass1_1010_af66, pass1_1010_c234, pass1_1010_c25e};
use crate::structs::struct_d::StructD;
use crate::sys_ops::debug_print_1008_6048;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1008_3000::pass1_1008_3e38;
use crate::unk::block_1008_4000::pass1_1008_4d84;
use crate::unk::block_1008_5000::{pass1_1008_57c4, win_1008_5c5c};
use crate::unk::block_1008_9000::{make_def_wnd_proc_1008_9ce6, pass1_1008_932a, pass1_1008_941a};
use crate::unk::block_1010_1000::pass1_1010_1ea6;
use crate::unk::block_1010_3000::{pass1_1010_375e, pass1_1010_3770};
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::unk::block_1018_2000::pass1_1018_2580;
use crate::unk::block_1020_2000::pass1_1020_2a94;
use crate::unk::block_1020_6000::{pass1_1020_61c4, pass1_1020_64d4, pass1_1020_68de};
use crate::unk::block_1028_8000::{pass1_1028_837e, pass1_1028_84ca};
use crate::unk::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8344};
use crate::unk::block_1030_e000::pass1_1030_e63e;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_8000::{pass1_1040_8478, string_1040_8520};
use crate::winapp::{winapp_a, winapp_b, winapp_e};
use crate::windef16::{HANDLE16, HCURSOR16, HDC16, HWND16, LPARAM, RECT16, WPARAM16};

pub fn win_sys_op_1020_493c(
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
                winapp::enum_child_windows_1010_01be();
                return;

                cursor::set_cursor_1020_5764(param_3, iVar6);
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
                    win_c::ui_op_1020_536e(param_2, param_3, 0x0, -0x1, 1);
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
                    winapp_a::unk_win_op_1010_7300(param_2, &uVar9[0x1].field19_0x20, 0x0, 0x9, 0x0);
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
                    winapp_b::post_msg_1020_55b0(param_2, param_3, param_5, param_6);
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


pub fn win_ui_op_1020_5e76(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u16;
    let mut puVar3: *mut u8;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_EDX: u32;
    let mut iVar8: i16;
    let mut puVar9: *mut u32;
    let mut puVar10: *mut u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut puVar13: *mut u32;
    let mut uVar14: u8;
    let mut in_AF: u8;
    let mut paVar15: *mut Struct57;
    let mut puVar16: *mut u32;
    let mut in_stack_0000fc00: u16;
    let mut in_stack_0000fd24: u16;
    let mut in_stack_0000fd2a: u16;
    let mut in_stack_0000fd2e: u16;
    let mut pcVar17: *mut c_char;
    let mut uVar18: u16;
    let mut in_stack_0000fd58: u16;
    let mut local_1aa: [*mut u8; 0x80] = [null_mut(); 0x80];
    let mut local_aa: [u8; 0x80] = [0; 0x80];
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut local_22: [u8; 0x10] = [0; 0x10];
    let mut puStack18: *mut u8;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut uStack10: u32;
    let mut local_6: u16;
    let mut uStack4: u16;
    let mut uVar7: u32;

    ReleaseCapture16();
    uVar11 = (param_1 >> 0x10);
    iVar8 = param_1;
    SetCursor16((iVar8 + 0xee));
    (iVar8 + 0xee) = 0;
    (iVar8 + 0xf4) = 0x1;
    local_6 = param_3;
    uStack4 = param_2;
    puVar2 = &local_6;
    winapp_e::pt_in_rect_1020_5856(puVar2, in_EDX, param_1, CONCAT22(0x1050, puVar2));
    uVar5 = in_EDX;
    uStack10 = CONCAT22(uVar5, puVar2);
    paVar15 = (in_EDX & 0xffff0000 | (uVar5 | puVar2));
    //  if ((uVar5 | puVar2) == 0) goto LAB_1020_6176;
    uStack12 = puVar2[0x6];
    uStack14 = puVar2[0x7];
    uStack16 = 0;
    puVar3 = pass1_1018_2580(
        in_AF,
        (iVar8 + 0xfa),
        0x0,
        CONCAT22(uStack12, uStack14),
        (iVar8 + 0x10c),
    );
    //  if (puVar3 == 0x6b2) goto LAB_1020_6176;
    puStack18 = puVar3;
    if (puVar3 == 0x6b8) {
        mem_op_1000_179c(0xb4, paVar15);
        uStack42 = CONCAT22(paVar15, puVar3);
        uVar5 = paVar15 | puVar3;
        uVar7 = paVar15 & 0xffff0000 | uVar5;
        if (uVar5 == 0) {
            iVar4 = 0;
            uVar12 = 0;
        } else {
            iVar4 = string_1040_8520(
                uVar7,
                CONCAT13((paVar15 >> 0x8), CONCAT12(paVar15, puVar3)),
                HWND16_1050_0396,
                0x20040,
                0x6ad06b8,
            );
            uVar12 = uVar7;
        }
        uStack38 = CONCAT22(uVar12, iVar4);
        uVar18 = 0xa5; //
        // LAB_1020_5f84:
        pass1_1008_941a(CONCAT13(0x10, CONCAT12(0x50, local_22)), 0x1, uVar18);
        pcVar17 = local_22;
        uVar12 = (uStack38 >> 0x10);
        puVar9 = uStack38;
    } else {
        if (puVar3 == 0x6b4) {
            mem_op_1000_179c(0xb4, paVar15);
            uStack42 = CONCAT22(paVar15, puVar3);
            uVar5 = paVar15 | puVar3;
            uVar7 = paVar15 & 0xffff0000 | uVar5;
            if (uVar5 == 0) {
                iVar4 = 0;
                uVar12 = 0;
            } else {
                iVar4 = string_1040_8520(
                    uVar7,
                    CONCAT13((paVar15 >> 0x8), CONCAT12(paVar15, puVar3)),
                    HWND16_1050_0396,
                    0x20040,
                    CONCAT22(puStack18, 0x57b),
                );
                uVar12 = uVar7;
            }
            uStack38 = CONCAT22(uVar12, iVar4);
            uVar18 = 0xab;
            // TODO: goto LAB_1020_5f84;
        }
        if (puVar3 == 0x6b6) {
            load_string_1010_84e0(
                _u16_1050_14cc,
                (_u16_1050_14cc >> 0x10),
                0x3ff,
                local_aa,
                0x1050,
            );
            load_string_1010_84e0(
                _u16_1050_14cc,
                (_u16_1050_14cc >> 0x10),
                0x3ff,
                local_1aa,
                0x1050,
            );
            uVar5 = sys_1000_3f9c(
                CONCAT13(0x10, CONCAT12(0x50, &stack0xfd56)),
                CONCAT22(0x1050, local_1aa),
                PTR_LOOP_1050_50cc,
            );
            uVar14 = 0;
            mem_op_1000_179c(0xb4, paVar15);
            uVar6 = paVar15;
            uStack42 = CONCAT22(uVar6, uVar5);
            if ((uVar6 | uVar5) == 0) {
                puVar9 = null_mut();
                paVar15 = null_mut();
            } else {
                uVar14 = 0x40;
                paVar15 = pass1_1040_8478(
                    uVar6 | uVar5,
                    CONCAT22(uVar6, uVar5),
                    0x40,
                    CONCAT13(0x10, CONCAT12(0x50, local_aa)),
                    CONCAT22(0x1050, &stack0xfd56),
                    HWND16_1050_0396,
                );
                puVar9 = paVar15;
            }
            uStack38 = paVar15 & 0xffff0000 | ZEXT24(puVar9);
            puVar10 = puVar9;
            puVar13 = ((paVar15 & 0xffff0000) >> 0x10); //
            // LAB_1020_6027:
            ppcVar1 = (*puVar10 + 0x74);
            (**ppcVar1)(uVar14, puVar9);
            // TODO: goto LAB_1020_6176;
        }
        if (puVar3 < 0x6a7) {
            if (((iVar8 + 0x10c) == 0x78) || ((iVar8 + 0x10c) == 0x74)) {
                puVar16 = mixed_1010_20ba(
                    paVar15,
                    _u16_1050_0ed0,
                    CONCAT22(in_stack_0000fd58, 0x5),
                    in_stack_0000fc00,
                    in_stack_0000fd24,
                    in_stack_0000fd2a,
                    in_stack_0000fd2e,
                );
                paVar15 = (paVar15 & 0xffff0000 | puVar16 >> 0x10);
                in_stack_0000fd58 = (puVar16 >> 0x10);
                if ((puVar16 + 0xa) == 0) {
                    return;
                }
            }
            if ((((((iVar8 + 0x10c) == 0x6c) || ((iVar8 + 0x10c) == 0x6d)) || ((iVar8 + 0x10c) == 0x31)) || ((iVar8 + 0x10c) == 0x32)) && (
                puVar16 = mixed_1010_20ba(
                    paVar15,
                    _u16_1050_0ed0,
                    CONCAT22(in_stack_0000fd58, 0x3a),
                    in_stack_0000fc00,
                    in_stack_0000fd24,
                    in_stack_0000fd2a,
                    in_stack_0000fd2e,
                ),
                (puVar16 + 0xa) == 0,
            )) {
                return;
            }
            pass1_1020_68de((iVar8 + 0xfe));
            // TODO: goto LAB_1020_6176;
        }
        if (0x6b1 < puVar3) {
            uVar14 = 0;
            mem_op_1000_179c(0xb4, paVar15);
            uStack42 = CONCAT22(paVar15, puVar3);
            uVar5 = paVar15 | puVar3;
            uVar7 = paVar15 & 0xffff0000 | uVar5;
            if (uVar5 == 0) {
                puVar9 = null_mut();
                puVar10 = null_mut();
                puVar13 = puVar10;
            } else {
                uVar14 = 0x40;
                puVar9 = string_1040_8520(
                    uVar7,
                    CONCAT13((paVar15 >> 0x8), CONCAT12(paVar15, puVar3)),
                    HWND16_1050_0396,
                    0x20040,
                    CONCAT22(puStack18, 0x57b),
                );
                puVar10 = uVar7;
                puVar13 = puVar10;
            }
            // TODO: goto LAB_1020_6027;
        }
        mem_op_1000_179c(0xb4, paVar15);
        uStack42 = CONCAT22(paVar15, puVar3);
        uVar5 = paVar15 | puVar3;
        uVar7 = paVar15 & 0xffff0000 | uVar5;
        if (uVar5 == 0) {
            uVar12 = 0;
        } else {
            string_1040_8520(
                uVar7,
                CONCAT13((paVar15 >> 0x8), CONCAT12(paVar15, puVar3)),
                HWND16_1050_0396,
                0x20040,
                CONCAT22(puStack18, 0x57b),
            );
            uVar12 = uVar7;
        }
        local_1aa[0] = puStack18 - 0x608;
        pass1_1008_941a(CONCAT13(0x10, CONCAT12(0x50, local_aa)), 0x1, local_1aa[0]);
        pcVar17 = local_aa;
        puVar9 = 0x1050;
    }
    ppcVar1 = (*puVar9 + 0x6c);
    (**ppcVar1)(0x1008, puVar9, uVar12, pcVar17); //
    // LAB_1020_6176:
    (iVar8 + 0x10c) = 0;
    return;
}

pub fn win_ui_get_prop_op_1040_9566(param_1: *mut i16)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut ppcVar3: *mut *mut code;
  let mut HVar4: HANDLE16;
  let mut HVar5: HANDLE16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut puStack12: *mut u32;

  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if *param_1 == 0x4 {
    uVar1 = (iVar6 + 0xc);
    HVar4 = GetProp16(s_thisHi_1050_5e6f,(iVar6 + 0xa));
    HVar5 = GetProp16(s_thisLo_1050_5e68,(iVar6 + 0xa));
    if (HVar4 | HVar5) != 0 {
      iVar2 = (iVar6 + 0x6);
      if iVar2 == 1 {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0xc);
        (**ppcVar3)(0x1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if iVar2 == 0x2 {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0x10);
        (**ppcVar3)(0x1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if iVar2 == 0x4 {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0x18);
        (**ppcVar3)(0x1538,uVar1,HVar4,*(iVar6 + 0x8) & 0x10);
        return;
      }
    }
  }
  return;
}

pub fn call_win_proc_1040_9684(win_handle_1: HWND16, mut param_2: u16, w_param_1: WPARAM16, l_param_1: LPARAM)

{
  let mut handle_1: HANDLE16;
  let mut handle_2: HANDLE16;
  let mut bool_1: bool;
  let mut uVar2: u16;
  let mut local_1a: [RECT16;0x2] = [RECT16::default();2];
  let mut var18: *mut u32;
  let mut var14: *mut u32;
  let mut var10: *mut u32;
  let mut var6: i32;
  let mut var2: u32;
  let mut var4: u16;
  let mut var11: u16;
  let mut var7: u16;
  let mut var8: u16;
  let mut var9: u16;
  let mut uVar1: u16;
  let mut fn_ptr_1: *mut *mut code;

  handle_1 = GetProp16(s_procHi_1050_5e7d,l_param_1);
  handle_2 = GetProp16(s_procLo_1050_5e76,l_param_1);
  var6 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16(s_thisHi_1050_5e8b,l_param_1);
  handle_2 = GetProp16(s_thisLo_1050_5e84,l_param_1);
  var10 = CONCAT22(handle_1,handle_2);
  if ((handle_1 | handle_2) != 0) {
    if (l_param_1 == 0x2) {
      var18 = var10;
      var14 = var10;
      if (var10.is_null() == false) {
        fn_ptr_1 = *var10;
        (**fn_ptr_1)(0x1538,handle_2,handle_1,1);
      }
    }
    else if (l_param_1 == 0x201) {
      handle_1 = GetProp16(s_IsDlg_1050_5e92,l_param_1);
      if (handle_1 == 0) {
        uVar2 = (var10 + 0x18);
        GetClientRect16(local_1a,0x1050);
        bool_1 = WinAPI16_PtInRect16(CONCAT22(param_2, win_handle_1), local_1a);
        if (bool_1 == 0) {
          return;
        }
        debug_print_1008_6048(uVar1,s_button__08lx_ldown_1050_5e98);
        fn_ptr_1 = (*var10 + 0x1c);
        (**fn_ptr_1)(0x1008,var10,(var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
        return;
      }
    }
    else if (l_param_1 == 0x204) {
      uVar2 = (handle_2 + 0x18);
      var4 = uVar1;
      GetClientRect16(local_1a,0x1050);
      bool_1 = WinAPI16_PtInRect16(CONCAT22(param_2, win_handle_1), local_1a);
      if (bool_1 == 0) {
        return;
      }
      debug_print_1008_6048(var4,s_button__08lx_rdown_1050_5eab);
      fn_ptr_1 = (*var10 + 0x20);
      (**fn_ptr_1)(0x8,var10,(var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
      return;
    }
  }
  if (var6 != 0) {
    CallWindowProc16(CONCAT13((param_2 >> 0x8),CONCAT12(param_2,win_handle_1)),w_param_1,l_param_1,
                     l_param_1,var6);
  }
  return;
}


pub fn win_dlg_op_1038_c58e(mut param_1: u16, mut param_2: u32)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut iVar2: i16;
  let mut unaff_SI: u16;
  let mut uVar3: u16;
  let mut in_stack_0000f68e: u16;
  let mut in_stack_0000f7b2: u16;
  let mut in_stack_0000f7b8: u16;
  let mut in_stack_0000f7bc: u16;
  let mut puStack2070: *mut u32;
  let mut local_80e: [u16;0x201] = [0;0x201];
  let mut local_40c: [u16;0x201] = [0;0x201];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             CONCAT22(unaff_SI,0x2),in_stack_0000f68e,in_stack_0000f7b2,in_stack_0000f7b8,
                             in_stack_0000f7bc);
  uStack10 = (puStack6 + 0x68);
  uVar3 = (param_2 >> 0x10);
  iVar2 = param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_40c),(iVar2 + 0x6));
  wsprintf16(local_80e,CONCAT22(local_40c,0x1050),CONCAT22(uStack10,0x1050),
             (uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_80e),(iVar2 + 0x6));
  puStack2070 = (param_2 & 0xffff0000 | (iVar2 + 0x96));
  pass1_1008_e038((iVar2 + 0x8e),(param_2 & 0xffff0000 | ZEXT24((iVar2 + 0x92))),
                  (param_2 & 0xffff0000 | (iVar2 + 0x96)));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x400,local_80e,
             0x1050);
  uVar1 = (iVar2 + 0x92);
  wsprintf16(local_40c,CONCAT22(local_80e,0x1050),CONCAT22(*puStack2070,0x1050),
             (*puStack2070 >> 0x10),uVar1,(uVar1 >> 0x10));
  SetDlgItemText16(CONCAT22(0x1050,local_40c),0x17f,(iVar2 + 0x6));
  return;
}

pub fn send_win_msg_1020_08fe(param_1: *mut astruct_63) {
    let mut hwnd: HWND16;
    let mut lVar1: i32;
    let mut BVar2: bool;
    let mut iVar2: *mut astruct_63;
    let mut uVar2: *mut astruct_63;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    param_1.field0_0x0 = 0xb0e;
    iVar2.field1_0x2 = 0x1020;
    if (iVar2.field229_0xe8 != 0) {
        lVar1 = iVar2.field229_0xe8;
        hwnd = (lVar1 + 0x6);
        BVar2 = IsWindow16(hwnd);
        if (BVar2 != 0) {
            SendMessage16(0x0, 0x1, 0x111, hwnd);
        }
        iVar2.field229_0xe8 = 0;
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar2.field208_0xd2)));
    param_1.field0_0x0 = 0x380a;
    iVar2.field1_0x2 = 0x1008;
    param_1.field0_0x0 = 0x389a;
    iVar2.field1_0x2 = 0x1008;
    return;
}


pub fn unk_win_op_1008_97f2(
    param_1: u32,
    param_2: *mut i16,
    param_3: WPARAM16,
    param_4: *mut u8,
    mut param_5: u16,
) -> u32 {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut paVar6: *mut astruct_864;
    let mut UVar7: u16;
    let mut unaff_CS: u8;
    let mut uVar8: u32;
    let mut uVar9: u8;
    let mut uVar10: u8;

    paVar6 = param_1;
    UVar7 = (param_1 >> 0x10);
    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2));
        } else {
            ppcVar1 = (*param_1 + 0x70);
            (**ppcVar1)();
        }
        uVar5 = 0x1;
        // TODO: goto LAB_1008_9a95;
    }
    uVar10 = (param_1 >> 0x10);
    uVar9 = SUB41(param_1, 0x0);
    if param_5 < 0x2c {
        unaff_CS = 0x8;
        // switch(param_5) {
        match param_5 {
            // case 0x1:
            1 => {}
            //   break;
            // case 0x2:
            2 => {
                ppcVar1 = (*param_1 + 0x3c);
                (**ppcVar1)(0x1008);
                SetWindowLong16(0x0, 0x0, paVar6.hwnd_0x8);
                BVar2 = IsWindow16(paVar6[0x12].hwnd_0x8);
                if (BVar2 != 0) {
                    PostMessage16(param_1, 0xc7, 0x111, paVar6[0x12].hwnd_0x8);
                }
            }
            //   break;
            // case 0x3:
            3 => {
                ppcVar1 = (*param_1 + 0x54);
                (**ppcVar1)(0x8, uVar9, UVar7, param_3, param_2);
            }
            //   break;
            // _ =>
            _ => {}
            // TODO: goto switchD_1008_9b30_caseD_4;
            // case 0x5:
            5 => {
                ppcVar1 = (*param_1 + 0x58);
                (**ppcVar1)(0x8, uVar9, uVar10, param_3, param_2, param_4);
            }
            //   break;
            // case 0x7:
            7 => {
                ppcVar1 = (*param_1 + 0x50);
                (**ppcVar1)(0x8, param_1, param_4);
            }
            //   break;
            // case 0x8:
            8 => {
                ppcVar1 = (*param_1 + 0x74);
                (**ppcVar1)(0x8, param_1, param_4);
            }
            //   break;
            // case 0xd:
            0xd => {
                ppcVar1 = (*param_1 + 0x84);
                iVar4 = (**ppcVar1)(
                    0x8,
                    uVar9,
                    uVar10,
                    param_2,
                    CONCAT12(param_4._0_1_, param_3),
                );
            }
            // TODO: goto LAB_1008_9ada;
            // case 0xf:
            0xf => {
                ppcVar1 = (*param_1 + 0x34);
                (**ppcVar1)(0x1008, param_1);
            }
            //   break;
            // case 0x10:
            0x10 => {
                ppcVar1 = (*param_1 + 0x38);
                uVar8 = (**ppcVar1)(0x1008, param_1);
                return uVar8;
            }
            // case 0x19:
            0x19 => {
                ppcVar1 = (*param_1 + 0x78);
                uVar3 = (**ppcVar1)(
                    0x8,
                    uVar9,
                    uVar10,
                    param_2,
                    CONCAT12(param_4._0_1_, param_3),
                );
                return CONCAT22(0x1050, uVar3);
            }
            // case 0x1c:
            0x1c => {
                ppcVar1 = (*param_1 + 0x30);
                (**ppcVar1)(0x8, param_1, param_4);
            }
        };
    } else if param_5 == 0x112 {
        if (PTR_LOOP_1050_039a.is_null())
            && (
                ppcVar1 = (*param_1 + 0x48),
                iVar4 = (**ppcVar1)(),
                iVar4 != 0,
            )
        {
            make_def_wnd_proc_1008_9ce6(
                paVar6,
                UVar7,
                CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)),
                param_4,
                0x112,
            );
        }
    } else if param_5 < 0x113 {
        if param_5 == 0x86 {
            ppcVar1 = (*param_1 + 0x80);
            uVar8 = (**ppcVar1)();
            return uVar8;
        }
        if param_5 < 0x87 {
            if param_5 == 0x85 {
                ppcVar1 = (*param_1 + 0x7c);
                uVar8 = (**ppcVar1)();
                return uVar8;
            }
            if param_5 < 0x86 {
                if param_5 == '7' {
                    return &paVar6[0x13].field_0x4;
                }
                if param_5 == 'A' {
                    ppcVar1 = (*param_1 + 0x2c);
                    (**ppcVar1)();
                    // TODO: goto switchD_1008_9b30_caseD_1;
                }
            }
            // switchD_1008_9b30_caseD_4:
            if (param_5 < 0x400) || (0x7ffe < param_5) {
                uVar8 = make_def_wnd_proc_1008_9ce6(
                    paVar6,
                    UVar7,
                    CONCAT22(param_3, param_2),
                    param_4,
                    param_5,
                );
                return uVar8;
            }
            ppcVar1 = (*param_1 + 0x28);
            (**ppcVar1)(
                unaff_CS,
                uVar9,
                uVar10,
                param_2,
                param_3,
                CONCAT22(param_5, param_4),
            );
        } else if param_5 == 0x100 {
            if PTR_LOOP_1050_039a.is_null() {
                ppcVar1 = (*param_1 + 0x6c);
                (**ppcVar1)();
            }
        } else if param_5 == 0x102 {
            if PTR_LOOP_1050_039a.is_null() {
                ppcVar1 = (*param_1 + 0x68);
                (**ppcVar1)();
            }
        } else {
            //   if (param_5 != 0x111) goto switchD_1008_9b30_caseD_4;
            if (param_4 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a.is_null()) {
                if param_2.is_null() {
                    ppcVar1 = (*param_1 + 0x40);
                    (**ppcVar1)();
                } else {
                    ppcVar1 = (*param_1 + 0x44);
                    (**ppcVar1)();
                }
            }
        }
    } else if param_5 == 0x204 {
        if PTR_LOOP_1050_039a.is_null() {
            ppcVar1 = (*param_1 + 0x60);
            (**ppcVar1)();
        }
    } else if (param_5 < 0x205) {
        if (param_5 == 0x113) {
            if _PTR_LOOP_1050_0388 != 0 {
                pass1_1008_932a(_PTR_LOOP_1050_0388);
            }
        } else if (param_5 == 0x117) {
            if (param_3 == 0) {
                ppcVar1 = (*param_1 + 0x4c);
                (**ppcVar1)();
            } else {
                ppcVar1 = (*param_1 + 0x20);
                (**ppcVar1)();
            }
        } else {
            //   if (param_5 != 0x201) goto switchD_1008_9b30_caseD_4;
            if PTR_LOOP_1050_039a.is_null() {
                ppcVar1 = (*param_1 + 0x5c);
                (**ppcVar1)();
            }
        }
    } else if param_5 == 0x210 {
        ppcVar1 = (*param_1 + 0x64);
        (**ppcVar1)();
    } else {
        if param_5 == 0x30f {
            //
            // LAB_1008_9af8:
            ppcVar1 = (*param_1 + 0x8c);
            iVar4 = (**ppcVar1)(); //
                                   // LAB_1008_9ada:
            return iVar4;
        }
        if param_5 == 0x311 {
            ppcVar1 = (*param_1 + 0x88);
            iVar4 = (**ppcVar1)();
        //   if (iVar4 != 0) goto LAB_1008_9af8;
        } else {
            //   if (param_5 != 0x3b9) goto switchD_1008_9b30_caseD_4;
            ppcVar1 = (*param_1 + 0x24);
            (**ppcVar1)();
        }
    }
    // switchD_1008_9b30_caseD_1:
    uVar5 = 0; //
               // LAB_1008_9a95:
    return uVar5;
}


pub fn unk_win_ui_op_1020_1418(
    param_1: *mut astruct_40,
    param_2: *mut StructA,
    mut param_3: u16,
) {
    let mut uVar1: u32;
    let mut paVar2: *mut astruct_13;
    let mut ppcVar3: *mut *mut code;
    let mut pHVar4: *mut HDC16;
    let pSVar5: *mut StructA;
    let mut puVar6: *mut u8;
    let mut in_EDX: u32;
    let mut paVar7: *mut Struct57;
    let mut iVar5: *mut astruct_40;
    let mut uVar9: u16;
    let mut puVar10: *mut u16;
    let mut puVar11: *mut u32;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_8: HDC16;
    let mut puStack6: *mut u32;
    let mut uVar8: u16;

    uVar8 = (in_EDX >> 0x10);
    gui::get_sys_metrics_1020_7c1a(param_1, param_2);
    uVar9 = (param_1 >> 0x10);
    iVar5 = param_1;
    iVar5.field17_0x14 = 0;
    iVar5.field20_0x18 = null_mut();
    puVar10 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x1e)));
    paVar7 = CONCAT22(uVar8, (puVar10 >> 0x10));
    param_1.field0_0x0 = 0x1730;
    iVar5.field1_0x2 = 0x1020;
    puVar11 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2d),
        in_stack_0000fe84,
        in_stack_0000ffa8,
        in_stack_0000ffae,
        in_stack_0000ffb2,
    );
    iVar5.field17_0x14 = puVar11;
    iVar5.field_0x16 = (puVar11 >> 0x10);
    puStack6 = mixed_1010_20ba(
        (paVar7 & 0xffff0000 | puVar11 >> 0x10),
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x29),
        in_stack_0000fe84,
        in_stack_0000ffa8,
        in_stack_0000ffae,
        in_stack_0000ffb2,
    );
    puVar6 = (puStack6 >> 0x10);
    uVar1 = &iVar5.field17_0x14;
    ppcVar3 = (*&iVar5.field17_0x14 + 0x4);
    (**ppcVar3)(0x1010, uVar1, (uVar1 >> 0x10), 0x0, param_1);
    local_8 = GetDC16(iVar5.hwnd_0x4);
    uVar1 = &iVar5.field17_0x14;
    *(uVar1 + 0x7c) = local_8;
    uVar1 = &iVar5.field17_0x14;
    pSVar5 = (uVar1 + 0x66);
    iVar5.field20_0x18 = pSVar5;
    ppcVar3 = (iVar5.field20_0x18 + 0x14);
    (**ppcVar3)();
    paVar2 = (puStack6 + 0xe);
    pass1_1008_4d84(puVar6, (pSVar5 & 0xffff | ZEXT24(puVar6) << 0x10), paVar2);
    pHVar4 = palette_op_1008_4e08(&local_8, puVar6, paVar2, CONCAT22(0x1050, &local_8));
    iVar5.field21_0x1c = pHVar4;
    return;
}


pub fn win_ui_op_1020_5de8(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut uVar1: u32;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_EDX: *mut Struct57;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe4: u16;
    let mut uStack18: u16;
    let mut bStack15: u8;
    let mut local_6: u16;
    let mut uStack4: u16;

    ReleaseCapture16();
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    SetCursor16((iVar5 + 0xee));
    (iVar5 + 0xee) = 0;
    (iVar5 + 0xf4) = 0x1;
    local_6 = param_3;
    uStack4 = param_2;
    puVar7 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x47),
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    uVar3 = (puVar7 >> 0x10);
    puVar2 = &local_6;
    winapp_e::pt_in_rect_1020_5856(puVar2, uVar3, param_1, CONCAT22(0x1050, puVar2));
    uVar4 = uVar3 | puVar2;
    if (uVar4 != 0) {
        uVar1 = (puVar2 + 0x21);
        uVar4 = puVar2[0x22];
        bStack15 = (uVar1 >> 0x18);
        puVar2 = bStack15;
        if (bStack15 == 0x5) {
            uStack18 = uVar1;
            uVar3 = uVar4;
            // TODO: goto LAB_1020_5e62;
        }
    }
    uStack18 = 0;
    uVar3 = 0; //
    // LAB_1020_5e62:
    pass1_1018_57e6(puVar7, CONCAT22(uVar3, uStack18), puVar2, uVar4);
    return;
}
