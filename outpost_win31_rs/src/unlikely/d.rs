
pub unsafe fn pass1_1008_6b5a(param_1: *mut astruct_458, param_2: u8) -> *mut u16 {
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


pub unsafe fn pass1_1008_6bb4(param_1: *mut astruct_459, param_2: u8) {
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

pub unsafe fn pass1_1008_6d18(param_1: *mut u16, param_2: *mut u16, param_3: *mut u16) {
    pass1_1008_3f62(param_1, param_3);
    pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x6)), param_2);
    return;
}


pub unsafe fn pass1_1008_7e98(param_1: *mut astruct_460, param_2: u8) -> *mut u16 {
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


pub unsafe fn pass1_1008_7ffa(param_1: *mut astruct_461, param_2: u8) {
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

pub unsafe fn win_sys_op_1008_84f2(
    lparam_param_1: LPARAM,
    wparam_param_2: WPARAM16,
    msg_param_3: u16,
    hwnd_param_4: HWND16,
) {
    let mut puVar1: *mut u16;
    let mut cVar2: u8;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut win_long_1: i32;
    let mut in_stack_0000ff7c: u32;
    let mut in_stack_0000ff84: u16;
    let mut rect_a: RECT16;
    let mut iStack4: i16;
    let mut iVar3: *mut astruct_863;

    win_long_1 = GetWindowLong16(0x0, hwnd_param_4);
    win_long_1 = (win_long_1 >> 0x10);
    iVar3 = win_long_1;
    if (msg_param_3 == 0x1f) {
        iVar3.field3_0x4 = 0;
        KillTimer16(0xfa6, hwnd_param_4);
        KillTimer16(0xfa7, hwnd_param_4);
        ReleaseCapture16();
    } else if (msg_param_3 < 0x20) {
        if (msg_param_3 != 0x14) {
            //   if (0x14 < msg_param_3) goto LAB_1008_8771;
            cVar2 = msg_param_3;
            uVar4 = msg_param_3 & 0xff00 | (cVar2 - 1);
            if ((cVar2 - 1) == 0) {
                //
                // LAB_1008_8560:
                win_ui_op_1008_8214(
                    uVar4,
                    win_long_1,
                    hwnd_param_4,
                    msg_param_3,
                    wparam_param_2,
                    lparam_param_1,
                );
                return;
            }
            if (cVar2 == '\x02') {
                fn_ptr_1000_17ce(win_long_1);
            } else if (cVar2 != '\x0f') {
                // if (cVar2 != '\x0f') goto LAB_1008_8771;
                draw_op_1008_8288(
                    in_stack_0000ff84,
                    in_stack_0000ff7c,
                    hwnd_param_4,
                    win_long_1,
                );
            }
        }
    } else if (msg_param_3 == 0x200) {
        if ((*&iVar3.field3_0x4 & 1) != 0) {
            GetClientRect16(&rect_a, &DAT_1050_1050);
            uVar4 = iVar3.field3_0x4;
            puVar1 = &iVar3.field3_0x4;
            *puVar1 = *puVar1 & 0xf3;
            BVar3 = PtInRect16(lparam_param_1, &rect_a);
            if (BVar3 == 0) {
                puVar1 = &iVar3.field3_0x4;
                *puVar1 = *puVar1 | 0x2;
            } else {
                if (lparam_param_1 < iStack4 >> 1) {
                    puVar1 = &iVar3.field3_0x4;
                    *puVar1 = *puVar1 | 0x4;
                } else {
                    puVar1 = &iVar3.field3_0x4;
                    *puVar1 = *puVar1 | 0x8;
                }
                puVar1 = &iVar3.field3_0x4;
                *puVar1 = *puVar1 & 0xfd;
            }
            if (iVar3.field3_0x4 != uVar4) {
                InvalidateRect16(0x1, NULL, 0x0);
                UpdateWindow16(hwnd_param_4);
            }
        }
    } else if (msg_param_3 < 0x201) {
        uVar4 = msg_param_3 - 0x81;
        // if (uVar4 == 0) goto LAB_1008_8560;
        if (msg_param_3 != 0x113) {
            //
            // LAB_1008_8771:
            DefWindowProc16(lparam_param_1, wparam_param_2, msg_param_3, hwnd_param_4);
            return;
        }
        if (wparam_param_2 == 0xfa6) {
            KillTimer16(0xfa6, hwnd_param_4);
            SetTimer16(NULL, 0x1, 0xfa7, hwnd_param_4);
        }
        if ((*&iVar3.field3_0x4 & 0x2) == 0) {
            send_msg_1008_84ba(hwnd_param_4, win_long_1);
        }
    } else {
        if (msg_param_3 != 0x201) {
            if (msg_param_3 == 0x202) {
                KillTimer16(0xfa6, hwnd_param_4);
                KillTimer16(0xfa7, hwnd_param_4);
                ReleaseCapture16();
                uVar4 = iVar3.field3_0x4;
                if (((uVar4 & 1) != 0) && ((uVar4 & 0xfffd) != 0)) {
                    puVar1 = &iVar3.field3_0x4;
                    *puVar1 = *puVar1 & 0xf2;
                    InvalidateRect16(0x1, NULL, 0x0);
                    UpdateWindow16(hwnd_param_4);
                }
                SendMessage16(iVar3.field2_0x2, 0xf9, 0x111, win_long_1);
                return;
            }
            //   if (msg_param_3 != 0x203) goto LAB_1008_8771;
        }
        puVar1 = &iVar3.field3_0x4;
        *puVar1 = *puVar1 | 0x1;
        GetClientRect16(&rect_a, &DAT_1050_1050);
        if (lparam_param_1 < (iStack4 >> 1)) {
            puVar1 = &iVar3.field3_0x4;
            *puVar1 = *puVar1 | 0x4;
        } else {
            puVar1 = &iVar3.field3_0x4;
            *puVar1 = *puVar1 | 0x8;
        }
        send_msg_1008_84ba(hwnd_param_4, win_long_1);
        SetTimer16(NULL, 0x12c, 0xfa6, hwnd_param_4);
        InvalidateRect16(0x1, NULL, 0x0);
        UpdateWindow16(hwnd_param_4);
        SetCapture16(hwnd_param_4);
    }
    return;
}

pub unsafe fn send_msg_1008_9640(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        SendMessage16(0x0, param_2, 0x86, (param_1 + 0x8));
    }
    return;
}

pub unsafe fn set_win_text_1008_9664(mut param_1: u32, mut param_2: u16, param_3: *mut c_char) {
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)), param_3);
    SetWindowText16(param_1 & 0xffff0000 | (param_1 + 0xa), (param_1 + 0x8));
    return;
}

pub unsafe fn destroy_win_1008_9698(param_1: *mut astruct_871, mut param_2: u16) {
    DestroyWindow16(param_1.hwnd_0x8);
    return;
}

pub unsafe fn begin_end_paint_1008_97c8(param_1: *mut astruct_837, mut param_2: u16) {
    BeginPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    EndPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    return;
}


pub unsafe fn unk_win_op_1008_97f2(
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
    if (param_5 < 0x2c) {
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
    } else if (param_5 == 0x112) {
        if ((PTR_LOOP_1050_039a.is_null())
            && (
                ppcVar1 = (*param_1 + 0x48),
                iVar4 = (**ppcVar1)(),
                iVar4 != 0,
            ))
        {
            make_def_wnd_proc_1008_9ce6(
                paVar6,
                UVar7,
                CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)),
                param_4,
                0x112,
            );
        }
    } else if (param_5 < 0x113) {
        if (param_5 == 0x86) {
            ppcVar1 = (*param_1 + 0x80);
            uVar8 = (**ppcVar1)();
            return uVar8;
        }
        if (param_5 < 0x87) {
            if (param_5 == 0x85) {
                ppcVar1 = (*param_1 + 0x7c);
                uVar8 = (**ppcVar1)();
                return uVar8;
            }
            if (param_5 < 0x86) {
                if (param_5 == '7') {
                    return &paVar6[0x13].field_0x4;
                }
                if (param_5 == 'A') {
                    ppcVar1 = (*param_1 + 0x2c);
                    (**ppcVar1)();
                    // TODO: goto switchD_1008_9b30_caseD_1;
                }
            }
            // switchD_1008_9b30_caseD_4:
            if ((param_5 < 0x400) || (0x7ffe < param_5)) {
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
        } else if (param_5 == 0x100) {
            if (PTR_LOOP_1050_039a.is_null()) {
                ppcVar1 = (*param_1 + 0x6c);
                (**ppcVar1)();
            }
        } else if (param_5 == 0x102) {
            if (PTR_LOOP_1050_039a.is_null()) {
                ppcVar1 = (*param_1 + 0x68);
                (**ppcVar1)();
            }
        } else {
            //   if (param_5 != 0x111) goto switchD_1008_9b30_caseD_4;
            if ((param_4 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a.is_null())) {
                if (param_2.is_null()) {
                    ppcVar1 = (*param_1 + 0x40);
                    (**ppcVar1)();
                } else {
                    ppcVar1 = (*param_1 + 0x44);
                    (**ppcVar1)();
                }
            }
        }
    } else if (param_5 == 0x204) {
        if (PTR_LOOP_1050_039a.is_null()) {
            ppcVar1 = (*param_1 + 0x60);
            (**ppcVar1)();
        }
    } else if (param_5 < 0x205) {
        if (param_5 == 0x113) {
            if (_PTR_LOOP_1050_0388 != 0) {
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
            if (PTR_LOOP_1050_039a.is_null()) {
                ppcVar1 = (*param_1 + 0x5c);
                (**ppcVar1)();
            }
        }
    } else if (param_5 == 0x210) {
        ppcVar1 = (*param_1 + 0x64);
        (**ppcVar1)();
    } else {
        if (param_5 == 0x30f) {
            //
            // LAB_1008_9af8:
            ppcVar1 = (*param_1 + 0x8c);
            iVar4 = (**ppcVar1)(); //
                                   // LAB_1008_9ada:
            return iVar4;
        }
        if (param_5 == 0x311) {
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


pub unsafe fn pass1_1008_9c16(mut param_1: u16, mut param_2: u32, mut param_3: u32) -> LRESULT {
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


pub unsafe fn pass1_1008_9c30(mut param_1: u16, mut param_2: u32, mut param_3: u32) -> LRESULT {
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

pub unsafe fn pass1_1008_9c4a() {
    return;
}
pub unsafe fn pass1_1008_9c4e() {
    return;
}
pub unsafe fn pass1_1008_9c52() {
    return;
}

pub unsafe fn get_stock_obj_1008_9c56() {
    GetStockObject16(HOLLOW_BRUSH);
    return;
}

pub unsafe fn pass1_1008_9c60(
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

pub unsafe fn pass1_1008_9c86(mut param_1: u32, param_2: *mut c_char, mut param_3: i16) {
    let mut uVar1: u16;

    uVar1 = str_op_1000_3da4((param_1 & 0xffff0000 | (param_1 + 0xa)));
    if (param_3 < uVar1) {
        uVar1 = param_3 - 0x1;
    }
    str_op_1000_3dbe(param_2, (param_1 & 0xffff0000 | (param_1 + 0xa)), uVar1);
    return;
}


pub unsafe fn pass1_1008_9d02(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_9f18(mut param_1: i16, mut param_2: u16, mut param_3: i16) {
    if (param_3 == 0x2) {
        pass1_1008_9f64(CONCAT22(param_2, param_1 -0x1c));
        pass1_1010_1f62(CONCAT22(param_2, param_1 -0x1c), 0x2);
    }
    return;
}


pub unsafe fn pass1_1008_9f80(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x1c));
    pass1_1008_9e5a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_9fb2(
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
    (param_2 + 0x1008) = &DAT_1050_1050;
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

pub unsafe fn pass1_1008_ad0c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        pass1_1000_093a(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ad38(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ad64(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_a086(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_af56(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_af38(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ba38(mut param_1: u32, mut param_2: u32) {
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


pub unsafe fn file_1008_bb5e(
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
