pub unsafe fn pass1_1008_80d2(param_1: u32) -> *mut u32 {
    *param_1 = 0;
    (param_1 + 0x4) = 0;
    return param_1;
}

pub unsafe fn unk_draw_op_1008_80ee(param_1: *mut astruct_23) -> *mut astruct_23 {
    let mut HVar1: HCURSOR16;
    let mut HVar2: HGDIOBJ16;
    let mut iVar3: *mut astruct_23;
    let mut uVar3: *mut astruct_23;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field79_0x54 = 0;
    iVar3.field80_0x56 = 0;
    iVar3.field81_0x58 = 0;
    param_1.field0_0x0 = 0x87c8;
    iVar3.field1_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field2_0x4)),
        s_MicroSpinControl_1050_0370,
    );
    iVar3.field79_0x54 = 0x3;
    HVar1 = LoadCursor16(0x7f00, 0x0);
    iVar3.field81_0x58 = HVar1;
    HVar2 = GetStockObject16(0x4);
    iVar3.field80_0x56 = HVar2;
    pass1_1008_818c((param_1 & 0xffff | ZEXT24(uVar3) << 0x10));
    return param_1;
}
pub unsafe fn pass1_1008_8168(param_1: *mut c_char) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1 = 0x87c8;
    (param_1 + 0x2) = 0x1008;
    param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    return;
}
pub unsafe fn pass1_1008_818c(param_1: *mut astruct_23) {
    let mut BVar1: bool;
    let mut AVar2: ATOM;
    let mut wndclass: WNDCLASS16;

    wndclass.lpsz_class_name = param_1 + 0x4;
    BVar1 = GetClassInfo16(
        &wndclass,
        CONCAT22(wndclass.lpsz_class_name, 0x1050),
        param_1,
    );
    if (BVar1 == 0) {
        wndclass.style = (param_1 + 0x54);
        wndclass.lpfn_wnd_proc = 0x84f2;
        wndclass.lpfn_wnd_proc = 0x1008;
        wndclass._6_4_ = 0x40000;
        wndclass.h_instance = HINSTANCE16_1050_038c;
        wndclass.h_icon = 0;
        wndclass.h_cursor = (param_1 + 0x58);
        wndclass.hbr_background = (param_1 + 0x56);
        wndclass.lpsz_menu_name = 0;
        wndclass.lpsz_class_name = param_1;
        AVar2 = RegisterClass16(&wndclass);
        if (AVar2 == 0) {
            fn_ptr_op_1000_24cd(0x0);
        }
    }
    return;
}

pub unsafe fn win_ui_op_1008_8214(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u16,
    mut param_6: u32,
) -> u16 {
    let mut uVar1: u16;
    let mut IVar2: i16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u16;
    let mut offset: i16;
    let mut hwnd: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_2);
    if (param_4 == 0x81) {
        offset = 0;
        hwnd = param_3;
        mem_op_1000_179c(0x6, paVar3);
        if ((paVar3 | param_1) == 0) {
            uVar1 = 0;
            puVar4 = null_mut();
        } else {
            puVar4 = pass1_1008_80d2(CONCAT22(paVar3, param_1));
            uVar1 = puVar4;
        }
        SetWindowLong16(puVar4 & 0xffff0000 | uVar1, offset, hwnd);
    }
    if (param_4 == 1) {
        puVar5 = GetWindowLong16(0x0, param_3);
        *puVar5 = (param_6 + 0x8);
        IVar2 = GetDlgCtrlID16(param_3);
        (puVar5 + 0x2) = IVar2;
    }
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps
pub unsafe fn draw_op_1008_8288(
    mut param_1: u16,
    mut param_2: u32,
    hdc16_param_1: HDC16,
    mut param_4: u32,
) {
    let mut paint_handle_1: HDC16;
    let mut pen_handle_1: HPEN16;
    let mut pen_handle_3: HPEN16;
    let mut brush_handle_1: HBRUSH16;
    let mut hgdiobj16_var1: HGDIOBJ16;
    let mut y: u16;
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut hdc: HDC16;
    let mut right_00: u16;
    let mut obj: HGDIOBJ16;
    let mut paintstruct_3c: [u8; 0x20] = [0; 0x20];
    let mut POpoint_1c: INT16;
    let mut iStack24: i16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut POlocal_10: INT16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;
    let mut in_stack_0000ffa6: u32;
    let mut left_01: i16;
    let mut top: i16;
    let mut left: i16;
    let mut x2: u16;
    let mut uVar1: u32;
    let mut right: u16;
    let mut in_stack_0000ffae: u16;
    let mut bottom: u16;

    paint_handle_1 = BeginPaint16(CONCAT22(0x1050, paintstruct_3c), hdc16_param_1);
    uStack4 = 0;
    pen_handle_1 = CreatePen16(COLORREF_1050_0368, 0x1, 0x0);
    pen_handle_3 = CreatePen16(COLORREF_1050_0364, 0x1, 0x0);
    brush_handle_1 = CreateSolidBrush16(COLORREF_1050_0364);
    uVar1 = CONCAT22(pen_handle_3, brush_handle_1);
    hdc = hdc16_param_1;
    GetClientRect16((&param_2 + 0x2), &DAT_1050_1050);
    y = param_1 >> 0x1;
    right_00 = x2;
    hgdiobj16_var1 = GetStockObject16(BLACK_PEN);
    hgdiobj16_var1 = SelectObject16(hgdiobj16_var1, hdc);
    param_2 = param_2 & 0xffff0000 | hgdiobj16_var1;
    hgdiobj16_var1 = GetStockObject16(BLACK_BRUSH);
    SelectObject16(hgdiobj16_var1, left);
    Rectangle16(param_1, right_00, top, (param_2 >> 0x10), paint_handle_1);
    MoveTo16(y, 0x0, paint_handle_1);
    param_2 = param_2 & 0xffff0000 | paint_handle_1;
    LineTo16(y, x2, paint_handle_1);
    uVar3 = (param_4 >> 0x10);
    if ((*(param_4 + 0x4) & 0x4) != 0) {
        uStack4 = 0x1;
    }
    local_10.x = (x2 >> 1) + uStack4;
    iVar1 = (param_1 >> 0x2) + uStack4;
    local_10.y = iVar1 + -0x2;
    iStack12 = local_10.x + -0x3;
    iStack10 = iVar1 + 1;
    iStack8 = local_10.x + 0x3;
    iStack6 = iStack10;
    param_2 = pen_handle_1;
    param_2 = paint_handle_1;
    SelectObject16(pen_handle_1, paint_handle_1);
    if (uStack4 == 0) {
        param_2 = CONCAT22(paint_handle_1, 1);
        MoveTo16(y - 0x2, 0x1, paint_handle_1);
        param_2 = 0x10001;
        LineTo16(0x1, 0x1, paint_handle_1);
        param_2 = 0x1;
        param_2 = s_tile2_bmp_1050_1538;
        LineTo16(0x1, x2 - 0x1, paint_handle_1);
    }
    uStack4 = ((*(param_4 + 0x4) & 0x8) != 0);
    point_1c.x = (x2 >> 1) + uStack4;
    iVar2 = (param_1 - (param_1 >> 0x2)) + uStack4;
    point_1c.y = iVar2 + 1;
    iStack24 = point_1c.x + -0x3;
    iStack22 = iVar2 + -0x2;
    iStack20 = point_1c.x + 0x3;
    iStack18 = iStack22;
    if (uStack4 == 0) {
        param_2 = 0x15388429;
        MoveTo16(paint_handle_1 - 0x2, 0x1, paint_handle_1);
        param_2 = 0x843a;
        LineTo16(y + 0x1, 0x1, paint_handle_1);
        uVar1 = CONCAT22(paint_handle_1, x2 - 1);
        LineTo16(y + 0x1, x2 - 0x1, paint_handle_1);
    }
    param_2 = CONCAT22(0x8453, param_2);
    SelectObject16((uVar1 >> 0x10), paint_handle_1);
    param_2 = CONCAT22(0x845e, param_2);
    SelectObject16(uVar1, paint_handle_1);
    obj = (uVar1 >> 0x10);
    param_2 = 0x31538;
    Polygon16(0x3, &local_10, &DAT_1050_1050);
    param_2 = 0x31538;
    hgdiobj16_var1 = 0x847c;
    Polygon16(0x3, &point_1c, &DAT_1050_1050);
    param_2 = CONCAT22(0x8487, param_2);
    SelectObject16(hgdiobj16_var1, paint_handle_1);
    param_2 = CONCAT22(0x8492, param_2);
    SelectObject16(param_2, paint_handle_1);
    DeleteObject16(pen_handle_1);
    DeleteObject16(obj);
    DeleteObject16(obj);
    EndPaint16(CONCAT22(0x1050, paintstruct_3c), hdc16_param_1);
    return;
}
pub unsafe fn send_msg_1008_84ba(mut param_1: u16, mut param_2: u32) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut WStack4: WPARAM16;

    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2;
    if ((*(iVar1 + 0x4) & 0x4) == 0) {
        if ((*(iVar1 + 0x4) & 0x8) == 0) {
            return;
        }
        WStack4 = 0x1;
    } else {
        WStack4 = 0;
    }
    SendMessage16((iVar1 + 0x2), WStack4, 0x115, param_2);
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

pub unsafe fn pass1_1008_87a2(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_8168(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_87cc(
    param_1: *mut astruct_86,
    mut param_2: i16,
    mut param_3: i16,
    mut param_4: u16,
    param_5: *mut astruct_76,
    mut param_6: u32,
    mut param_7: u32,
) {
    let mut lVar1: i32;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar5: *mut astruct_86;
    let mut iVar7: i16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut puVar11: *mut u16;
    let mut puVar12: *mut u16;
    let mut in_stack_0000fe70: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ff9e: u16;
    let mut piStack48: *mut i16;
    let mut local_24: *mut astruct_19;
    let mut uStack32: u16;
    let mut uStack30: u32;
    let mut pcStack26: *mut c_char;
    let mut uStack18: u32;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut uStack6: u32;

    uVar10 = (param_7 >> 0x10);
    uVar8 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar5.field1_0x2 = 0x1008;
    iVar5.field2_0x4 = param_5;
    iVar5.field3_0x8 = 0;
    iVar5.field4_0xc = param_3;
    iVar5.field5_0xe = param_2;
    iVar5.field6_0x10 = 0;
    iVar5.field7_0x12 = 0;
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field12_0x1c)));
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field14_0x22)));
    puVar11 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field17_0x28)));
    paVar6 = CONCAT22(uVar10, (puVar11 >> 0x10));
    iVar5.field20_0x2e = param_4;
    iVar5.field21_0x30 = 0xffff;
    iVar5.field27_0x3a = 0;
    iVar5.field28_0x3e = 0x1;
    iVar5.field29_0x40 = 0x1;
    iVar5.field30_0x42 = param_6;
    param_1.field0_0x0 = 0x8e9a;
    iVar5.field1_0x2 = 0x1008;
    if (_PTR_LOOP_1050_0382.is_null()) {
        _PTR_LOOP_1050_0382 = mixed_1010_20ba(
            paVar6,
            _u16_1050_0ed0,
            CONCAT22(unaff_SI, 0x2e),
            in_stack_0000fe70,
            in_stack_0000ff94,
            in_stack_0000ff9a,
            in_stack_0000ff9e,
        );
        paVar6 = (paVar6 & 0xffff0000);
    }
    uVar10 = (paVar6 >> 0x10);
    uStack6 = pass1_1008_4772(iVar5.field2_0x4);
    iVar5.field7_0x12 = 0x2f - (uStack6 + 0x8);
    uVar9 = (_PTR_LOOP_1050_0382 >> 0x10);
    iVar7 = _PTR_LOOP_1050_0382;
    iStack8 = (iVar7 + 0xa);
    iStack10 = (iVar7 + 0xc);
    iStack12 = (iVar7 + 0xe);
    iStack14 = (iVar7 + 0x10);
    iVar7 = iVar5.field4_0xc;
    lVar1 = (iVar7 + iVar5.field5_0xe) * iStack14;
    paVar6 = CONCAT22(uVar10, (lVar1 >> 0x10));
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | ZEXT24(&iVar5.field12_0x1c)),
        0x0,
        lVar1 + iVar5.field7_0x12 + iStack10,
        (iVar7 - iVar5.field5_0xe) * iStack12 + iVar5.field6_0x10 + iStack8,
    );
    iVar5.field8_0x14 = &iVar5.field12_0x1c + 0x20;
    iVar5.field9_0x16 = (uStack6 + 0x8) + (&iVar5.field12_0x1c + 0x2) + -0x25;
    iVar5.field10_0x18 = iVar5.field8_0x14 + 0x32;
    uVar2 = iVar5.field9_0x16 + 0x19;
    iVar5.field11_0x1a = uVar2;
    mem_op_1000_179c(0x6, paVar6);
    uVar5 = paVar6;
    pcStack26 = CONCAT22(uVar5, uVar2);
    uStack18 = uVar5 | uVar2;
    if (uStack18 == 0) {
        iVar5.field3_0x8 = 0;
    } else {
        puVar12 = pass1_1008_ada2(CONCAT22(uVar5, uVar2), iVar5.field20_0x2e);
        uStack18 = (puVar12 >> 0x10);
        iVar5.field3_0x8 = puVar12;
        (iVar5.field3_0x8 + 0x2) = uStack18;
    }
    BVar3 = pass1_1008_aed8(iVar5.field3_0x8);
    if (BVar3 == 0) {
        pcStack26 = iVar5.field3_0x8;
        uStack18 = pcStack26;
        fn_ptr_1000_17ce(pcStack26);
        iVar5.field3_0x8 = 0;
    } else {
        piVar4 = iVar5.field3_0x8;
        pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar4);
        uStack18 = SUB42(piVar4, 0x0);
        pass1_1008_add2(iVar5.field3_0x8);
        uStack30 = pass1_1008_4772(CONCAT22(uStack18, uStack18));
        pass1_1018_214e(
            _PTR_LOOP_1050_0382,
            (_PTR_LOOP_1050_0382 >> 0x10),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field17_0x28)),
            iVar5.field20_0x2e,
        );
        local_24 = iVar5.field12_0x1c;
        uStack32 = iVar5.field13_0x20;
        pass1_1008_3f32(
            CONCAT22(0x1050, &local_24),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field17_0x28)),
        );
        piStack48 = (param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x32));
        pass1_1008_3e94(
            CONCAT22(0x1050, &local_24),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field24_0x34)),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x32)),
        );
        uVar10 = (uStack30 >> 0x10);
        iVar5.field25_0x36 = (uStack30 + 0x4) + *piStack48;
        uVar2 = (uStack30 + 0x8) + iVar5.field24_0x34;
        iVar5.field26_0x38 = uVar2;
        pass1_1008_612e(uVar2, 0x2, 0x5);
        iVar5.field28_0x3e = uVar2;
    }
    return;
}
pub unsafe fn pass1_1008_8aa2(param_1: *mut astruct_462) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u32;
    let mut iVar6: *mut astruct_462;
    let mut uVar6: u16;
    let mut pcStack16: *mut c_char;

    uVar6 = (param_1 >> 0x10);
    iVar6 = param_1;
    param_1 = 0x8e9a;
    iVar6.field2_0x2 = 0x1008;
    uVar5 = &iVar6.field3_0x4;
    if ((uVar5 + 0x1c) != 0) {
        puVar1 = iVar6.field3_0x4;
        uVar2 = iVar6.field4_0x6;
        if ((uVar2 | puVar1) != 0) {
            ppcVar4 = *puVar1;
            (**ppcVar4)();
        }
    }
    uVar2 = iVar6.field55_0x3a;
    uVar3 = iVar6.field56_0x3c;
    pcStack16 = CONCAT22(uVar3, uVar2);
    if ((uVar3 | uVar2) != 0) {
        pass1_1008_5118(CONCAT22(uVar3, uVar2));
        fn_ptr_1000_17ce(pcStack16);
    }
    param_1 = 0x389a;
    iVar6.field2_0x2 = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_8b20(mut param_1: u32) {
    let mut iVar1: i16;
    let mut piVar2: *mut i16;
    let mut in_EDX: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut local_a: [u8; 0x2] = [0; 0x2];
    let mut local_8: [u8; 0x2] = [0; 0x2];
    let mut paStack6: *mut astruct_76;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0x8) != 0) {
        iVar1 = (iVar5 + 0x40);
        piVar2 = (iVar5 + 0x40);
        *piVar2 = *piVar2 + 1;
        uVar4 = iVar1 % (iVar5 + 0x3e) & 0xffff;
        uVar3 = in_EDX & 0xffff0000 | uVar4;
        if (uVar4 == 0) {
            (iVar5 + 0x40) = 0x1;
            piVar2 = (iVar5 + 0x8);
            pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar2);
            paStack6 = (piVar2 & 0xffff | uVar3 << 0x10);
            uVar4 = uVar3 & 0xffff0000 | uVar6;
            pass1_1008_3e94(
                (param_1 & 0xffff0000 | (iVar5 + 0x28)),
                CONCAT22(0x1050, local_a),
                CONCAT22(0x1050, local_8),
            );
            pass1_1008_8d8a(
                (param_1 & 0xffff | uVar6 << 0x10),
                paStack6,
                (iVar5 + 0x4),
                uVar4,
            );
            pass1_1008_4480(
                (iVar5 + 0x4),
                (param_1 & 0xffff0000 | (iVar5 + 0x28)),
                paStack6,
            );
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_8bc6(mut param_1: u16, mut param_2: u32) {
    let mut piVar1: *mut i16;
    let mut in_register_0000000a: u16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut local_a: [u8; 0x2] = [0; 0x2];
    let mut local_8: [u8; 0x2] = [0; 0x2];
    let mut paStack6: *mut astruct_76;

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if ((iVar3 + 0x8) == 0) {
        return;
    }
    piVar1 = (iVar3 + 0x8);
    pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar1);
    paStack6 = (piVar1 & 0xffff | param_1 << 0x10);
    uVar2 = CONCAT22(in_register_0000000a, uVar4);
    pass1_1008_3e94(
        (param_2 & 0xffff0000 | (iVar3 + 0x28)),
        CONCAT22(0x1050, local_a),
        CONCAT22(0x1050, local_8),
    );
    pass1_1008_8d8a(
        (param_2 & 0xffff | uVar4 << 0x10),
        paStack6,
        (iVar3 + 0x4),
        uVar2,
    );
    pass1_1008_4480(
        (iVar3 + 0x4),
        (param_2 & 0xffff0000 | (iVar3 + 0x28)),
        paStack6,
    );
    return;
}
pub unsafe fn pass1_1008_8c4e(mut param_1: u32, mut param_2: u32, mut param_3: u32) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut paStack14: *mut astruct_110;
    let mut paVar4: *mut Struct57;

    uVar5 = (param_3 >> 0x10);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar8 = pass1_1008_4772((iVar6 + 0x4));
    uVar2 = (uVar8 >> 0x10);
    paVar4 = CONCAT22(uVar5, uVar2);
    uVar3 = 0;
    if (((iVar6 + 0xc) == 0) || ((iVar6 + 0xe) == 0)) {
        mem_op_1000_179c(0x14, paVar4);
        paStack14 = CONCAT22(paVar4, uVar3);
        uVar3 = paVar4 | uVar3;
        if (uVar3 == 0) {
            uVar2 = 0;
            uVar3 = 0;
        } else {
            puVar1 = (param_1 & 0xffff0000 | (iVar6 + 0x1c));
            pass1_1008_50c2(paStack14, (uVar8 + 0x8), (uVar8 + 0x4), puVar1, param_2);
            uVar2 = SUB42(puVar1, 0x0);
        }
        pass1_1008_5134(CONCAT22(uVar3, uVar2));
    }
    pass1_1008_4480(
        param_2,
        (param_1 & 0xffff0000 | (iVar6 + 0x1c)),
        (iVar6 + 0x4),
    );
    return;
}
pub unsafe fn pass1_1008_8ce4(mut param_1: u32, param_2: *mut u16, mut param_3: u32, mut param_4: u32) {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut local_10: [u8; 0x6] = [0; 0x6];
    let mut uStack10: u32;
    let mut uStack6: u32;

    uVar6 = (param_4 >> 0x10);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uStack6 = pass1_1008_4772((iVar4 + 0x4));
    uStack10 = 0;
    puVar7 = pass1_1008_3e54(
        CONCAT22(0x1050, local_10),
        0x0,
        (iVar4 + 0x12),
        (iVar4 + 0x10),
    );
    paVar3 = CONCAT22(uVar6, (puVar7 >> 0x10));
    puVar1 = local_10;
    pass1_1008_3f32(param_2, CONCAT22(0x1050, puVar1));
    mem_op_1000_179c(0x14, paVar3);
    uVar2 = paVar3 | puVar1;
    if (uVar2 == 0) {
        puVar1 = null_mut();
        uVar2 = 0;
    } else {
        uVar6 = (uStack6 >> 0x10);
        pass1_1008_50c2(
            CONCAT22(paVar3, puVar1),
            (uStack6 + 0x8),
            (uStack6 + 0x4),
            param_2,
            param_3,
        );
    }
    uStack10 = CONCAT22(uVar2, puVar1);
    pass1_1008_5134(CONCAT22(uVar2, puVar1));
    pass1_1008_4480(param_3, param_2, (iVar4 + 0x4));
    return;
}
pub unsafe fn pass1_1008_8d8a(
    param_1: *mut astruct_76,
    param_2: *mut astruct_76,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut uVar1: u16;
    let mut cVar2: u8;
    let mut puVar3: *mut u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar8: u16;
    let mut pstruct76_var7: *mut astruct_76;
    let mut uVar7: *mut astruct_76;
    let mut uVar9: u32;
    let mut paStack10: *mut astruct_110;
    let mut paVar6: *mut Struct57;

    uVar8 = (param_4 >> 0x10);
    uVar7 = (param_1 >> 0x10);
    pstruct76_var7 = param_1;
    uVar1 = pstruct76_var7[0x1].field3_0x6;
    if (uVar1 < 0x28) {
        if ((uVar1 < 0x25) && (uVar1 != 0x23)) {
            if (0x23 < uVar1) {
                return;
            }
            cVar2 = uVar1;
            // (cVar2 != '\v') &&
            if ((cVar2 != '\x0f') && (cVar2 != '!')) {
                return;
            }
        }
    } else if (uVar1 < 0x46) {
        if (uVar1 < 0x43) {
            if (uVar1 < 0x33) {
                return;
            }
            if ((uVar1 != 0x34 && 0x0 < (uVar1 - 0x33)) && (uVar1 != 0x37)) {
                return;
            }
        }
    } else if (uVar1 != 0x49) {
        if ((uVar1 - 0x49) < 0x2a) {
            return;
        }
        if (0x5 < (uVar1 - 0x73)) {
            return;
        }
    }
    if ((&pstruct76_var7[0x1].field8_0x10 + 0x2) == 0) {
        uVar9 = pass1_1008_4772(param_2);
        uVar4 = (uVar9 >> 0x10);
        paVar6 = CONCAT22(uVar8, uVar4);
        uVar1 = uVar9;
        uVar5 = uVar1;
        mem_op_1000_179c(0x14, paVar6);
        paStack10 = CONCAT22(paVar6, uVar5);
        uVar5 = paVar6 | uVar5;
        if (uVar5 == 0) {
            (&pstruct76_var7[0x1].field8_0x10 + 0x2) = 0;
        } else {
            puVar3 = (param_1 & 0xffff0000 | ZEXT24(pstruct76_var7 + 1));
            pass1_1008_50c2(paStack10, (uVar1 + 0x8), (uVar1 + 0x4), puVar3, param_3);
            (&pstruct76_var7[0x1].field8_0x10 + 0x2) = puVar3;
            pstruct76_var7[0x1].field9_0x14 = uVar5;
        }
        pass1_1008_5134((&pstruct76_var7[0x1].field8_0x10 + 0x2));
        return;
    }
    pass1_1008_5236((&pstruct76_var7[0x1].field8_0x10 + 0x2));
    return;
}

pub unsafe fn pass1_1008_8e74(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_8aa2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn struct_op_1008_8e9e(param_1: *mut astruct_78, mut param_2: u32, mut param_3: u32) {
    let mut iVar1: *mut astruct_78;
    let mut uVar1: *mut astruct_78;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar1.field1_0x2 = 0x1008;
    iVar1.field2_0x4 = 0;
    iVar1.field3_0x6 = null_mut();
    iVar1.field4_0xa = 0;
    iVar1.field5_0xe = param_3;
    iVar1.field6_0x12 = 0;
    iVar1.field7_0x16 = param_2;
    iVar1.field8_0x1a = 0x1;
    param_1.field0_0x0 = 0x9170;
    iVar1.field1_0x2 = 0x1008;
    if (iVar1.field5_0xe < 0x7) {
        iVar1.field5_0xe = 0x6;
    }
    pass1_1008_909c(param_1);
    iVar1.field3_0x6 = 0;
    return;
}
pub unsafe fn pass1_1008_8f24(param_1: *mut astruct_463) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u32;
    let mut iVar6: *mut astruct_463;
    let mut iVar7: i16;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uStack6: u32;

    uVar9 = (param_1 >> 0x10);
    iVar6 = param_1;
    param_1 = 0x9170;
    iVar6.field2_0x2 = 0x1008;
    if (iVar6.field19_0x1a != 0) {
        uStack6 = 0;
        loop {
            puVar1 = &iVar6.field6_0xa;
            if (*puVar1 < uStack6 || *puVar1 == uStack6) {
                break;
            }
            iVar8 = uStack6 * 0x4;
            uVar5 = iVar6.field5_0x6;
            uVar10 = (uVar5 >> 0x10);
            iVar7 = uVar5;
            puVar2 = (iVar7 + iVar8);
            uVar3 = (iVar7 + iVar8 + 2);
            if ((uVar3 | puVar2) != 0) {
                ppcVar4 = *puVar2;
                (**ppcVar4)();
            }
            uStack6 += 0x1;
        }
    }
    fn_ptr_1000_17ce(iVar6.field5_0x6);
    param_1 = 0x389a;
    iVar6.field2_0x2 = 0x1008;
    return;
}
pub unsafe fn pass1_1008_8faa(param_1: *mut astruct_78, mut param_2: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1008_9004(
        (param_1 & 0xffff | uVar1 << 0x10),
        param_2,
        (param_2 >> 0x10),
        (param_1 + 0xa),
    );
    return;
}
pub unsafe fn empty_1008_8fc4() {
    return;
}
