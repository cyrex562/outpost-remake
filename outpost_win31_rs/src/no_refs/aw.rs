use crate::block_1040::block_1040_8000::{pass1_1040_869a, pass1_1040_8e82};
use crate::block_1040::block_1040_9000::{draw_op_1040_9948, pass1_1040_9824};
use crate::block_1040::block_1040_a000::{free_proc_inst_1040_a294, msg_box_op_1040_a85a, msg_box_ui_op_1040_ad66, pass1_1040_ace8, win_msg_1040_a308, win_ui_dlg_op_1040_a94a, win_ui_op_1040_ae04};
use crate::block_1040::block_1040_b000::unk_draw_op_1040_b0f8;
use crate::windef::RECT16;

pub unsafe fn pass1_1040_89a4(param_1: *mut u8, param_2: *mut u32, param_3: *mut u16)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000fff0: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    uVar1 = (param_3 + 2);
    uVar2 = *param_3;
    uVar7 = 0x1010;
    puVar8 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000fff0, 0x2), in_stack_0000fe98,
                             in_stack_0000ffbc, in_stack_0000ffc2, in_stack_0000ffc6);
    uVar4 = (puVar8 >> 0x10);
    uVar5 = puVar8;
    if ((uVar5 + 0x72) != 0) {
        uVar7 = 0x1008;
        win_1008_5c7c(uVar5, uVar4, _u16_1050_02a0, CONCAT22(uVar1, uVar2));
        (param_2 + 0x8c) = uVar5;
    }
    ppcVar3 = (*param_2 + 0x74);
    (**ppcVar3)(uVar7, param_2);
    return;
}


pub unsafe fn mixed_draw_op_1040_8a06(mut param_1: u16, param_2: *mut astruct_765)

{
    let mut paVar1: *mut astruct_13;
    let mut uVar6: u8;
    let mut HVar7: HPALETTE16;
    let mut handle: HANDLE16;
    let mut extraout_var: u32;
    let mut extraout_DX: u16;
    let mut iVar10: *mut astruct_765;
    let mut count: i16;
    let mut uVar8: u32;
    let mut color: COLORREF;
    let mut color_00: u32;
    let mut hdc_local_24: HDC16;
    let mut paintstruct_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut uVar1: u8;
    let mut uVar2: u8;
    let mut uVar3: u8;
    let mut uVar4: LPCSTR;
    let mut uVar5: u16;
    let mut iVar2: *mut astruct_766;

    count = (param_2 >> 0x10);
    iVar10 = param_2;
    hdc_local_24 = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.hwnd_field6_0x6);
    paVar1 = (_PTR_LOOP_1050_4230 + 0xe);
    HVar7 = palette_op_1008_4e08(&hdc_local_24, param_1, paVar1, CONCAT22(0x1050, &hdc_local_24));
    uVar8 = pass1_1008_4d72(paVar1);
    uVar5 = (uVar8 >> 0x10);
    iVar2 = uVar8;
    uVar1 = iVar2.field149_0x95;
    uVar2 = iVar2.field150_0x96;
    uVar3 = iVar2.field148_0x94;
    DrawIcon16(iVar10.field141_0x8e, 0xa, 0x14, hdc_local_24);
    color = SetBkColor16(0x0, hdc_local_24);
    extraout_DX = (color >> 0x10);
    uVar6 = SetTextColor16(CONCAT22(CONCAT11(0x2, uVar3), CONCAT11(uVar1, uVar2)), hdc_local_24);
    color_00 = CONCAT31(extraout_var, uVar6) & 0xffff | extraout_DX << 0x10;
    handle = GetProp16(s_hfont_1050_5dfa, iVar10.hwnd_field6_0x6);
    if (handle != 0) {
        SelectObject16(handle, hdc_local_24);
    }
    DrawText16(0x10, (param_2 & 0xffff0000 | ZEXT24(&iVar10.rect_0x9e)), -0x1,
               iVar10.field142_0x90, hdc_local_24);
    if (handle != 0) {
        SelectObject16(hdc_local_24, hdc_local_24);
    }
    SetBkColor16(color, hdc_local_24);
    SetTextColor16(color_00, hdc_local_24);
    HVar7 = SelectPalette16(0x0, HVar7, hdc_local_24);
    DeleteObject16(HVar7);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.hwnd_field6_0x6);
    return;
}


pub unsafe fn pass1_1040_8b3c(mut param_1: u16, mut param_2: u32, mut param_3: u32)

{
    if ((param_3.is_null() == false) && ((param_3 == (&PTR_LOOP_1050_0000 + 1) || param_3 == &u16_1050_0002 || (((&u16_1050_0002 + 1) < param_3 - 0x2 && (param_3 - 0x6 < &u16_1050_0002)))))) {
        PTR_LOOP_1050_5df4 = null_mut();
        PTR_LOOP_1050_5df8 = param_3;
        return;
    }
    post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3, param_3);
    return;
}


pub unsafe fn pass1_1040_8db6(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_869a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn enable_window_1040_8ea0(param_1: *mut u8, param_2: *mut Struct903, mut param_3: u16, mut param_4: u32)

{
    let mut enable: bool;
    let mut hwnd: HWND16;

    if (param_4 == 0xf8) {
        hwnd = GetDlgItem16(0x17d8, (param_2 + 0x6));
        enable = 0x1;
    } else {
        if (param_4 != 0x17d8) {
            pass1_1040_b54a(param_1, param_2, param_3, param_4);
            return;
        }
        SetWindowPos16(0x6, 0xf6, 0x269, 0x0, 0x0, 0x0, (param_2 + 0x6));
        enable = s_tile2_bmp_1050_1538;
        GetDlgItem16(0x17d8, (param_2 + 0x6));
        hwnd = 0;
    }
    EnableWindow16(enable, hwnd);
    return;
}


pub unsafe fn pass1_1040_8f16(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_8e82(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_93e6(mut param_1: u32) -> LRESULT

{
    let mut uVar1: u16;
    let mut LVar2: LRESULT;

    uVar1 = (param_1 >> 0x10);
    LVar2 = SendMessage16(0x0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}


pub unsafe fn send_msg_1040_9404(mut param_1: u32) -> LRESULT

{
    let mut uVar1: u16;
    let mut LVar2: LRESULT;

    uVar1 = (param_1 >> 0x10);
    LVar2 = SendMessage16(0x0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}


pub unsafe fn pass1_1040_9422(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        ppcVar1 = (*param_1 + 0x10);
        (**ppcVar1)();
    }
    if ((param_1 + 0x4) != 0) {
        ppcVar1 = (*param_1 + 0x14);
        (**ppcVar1)();
    }
    return;
}


pub unsafe fn unk_draw_op_1040_9458(param_1: *mut astruct_17, param_2: u8, mut param_3: u16)

{
    let mut ppcVar1: *mut *mut code;
    let mut hpal: *mut u16;
    let mut obj: HPALETTE16;
    let mut uVar3: u16;
    let mut iVar4: *mut astruct_17;
    let mut uVar4: u16;
    let mut puStack8: *mut u16;
    let mut puStack6: *mut u32;
    let mut UVar2: u32;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (iVar4.field8_0x8.is_null() == false) {
        puStack6 = iVar4.field8_0x8;
        if ((((&iVar4.field9_0xc + 0x2) | &iVar4.field9_0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack6 = iVar4.field9_0xc;
        }
        if ((iVar4.field10_0x10.is_null() == false) && ((param_2 & 0x4) != 0)) {
            puStack6 = iVar4.field10_0x10;
        }
        hpal = &param_3;
        UVar2 = *puStack6;
        ppcVar1 = (UVar2 + 0x8);
        (**ppcVar1)();
        ppcVar1 = (UVar2 + 0x4);
        (**ppcVar1)();
        obj = SelectPalette16(0x0, hpal, param_3);
        DeleteObject16(obj);
    }
    return;
}

pub unsafe fn win_op_1040_9cde(lparam_param_1: LPARAM, wparam_param_2: WPARAM16, msg_param_3: u16, hwnd_param_4: HWND16,
                               mut param_5: u16, mut param_6: u16, mut param_7: u32)

{
    let mut pbVar1: *mut u8;
    let mut iVar2: i16;
    let mut bVar3: u8;
    let mut WVar4: WPARAM16;
    let mut BVar5: bool;
    let mut uVar9: u32;
    let mut uVar6: u16;
    let mut uVar8: i16;
    let mut uVar10: u16;
    let mut win_long_11: i32;
    WPARAM16 * pWVar11;
    let mut LVar12: LRESULT;
    let mut uVar13: u32;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut rect_a: [RECT16; 0x2] = [RECT16::default(); 2];
    let mut iVar3: i16;
    let mut paVar7: *mut Struct57;
    let mut hwnd_dlg_8: HWND16;

    uVar10 = (param_7 >> 0x10);
    win_long_11 = GetWindowLong16(0x0, hwnd_param_4);
    paVar7 = CONCAT22(uVar10, (win_long_11 >> 0x10));
    iVar3 = win_long_11;
    uVar8 = ((win_long_11 & 0xffff0000) >> 0x10);
    if (msg_param_3 == 0x30) {
        (iVar3 + 0x5a) = wparam_param_2;
    } else {
        if (msg_param_3 < 0x31) {
            if (msg_param_3 == 0x1f) {
                (iVar3 + 0x4) = 0;
                ReleaseCapture16();
                return;
            }
//      if (0x1f < msg_param_3) goto LAB_1040_a1ae;
            bVar3 = msg_param_3;
            if (bVar3 == 0x8) {
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 & 0xf7;
                uVar6 = 0;
                BVar5 = IsWindow16(wparam_param_2);
                if (BVar5 != 0) {
                    uVar13 = SendMessage16(0x0, 0x0, 0x87, wparam_param_2);
                    uVar6 = ((uVar13 & 0x20) == 0);
                }
                (iVar3 + 0x56) = 0;
                SendMessage16(0x0, (iVar3 + 0x5c), 0x401, (iVar3 + 0x2));
                if (((iVar3 + 0x5c) != 0) && ((iVar3 + 0x5c) != win_long_11)) {
                    SendDlgItemMessage16(uVar6, 0x1, 0x404, (iVar3 + 0x5c), (iVar3 + 0x2));
                }
                (iVar3 + 0x5c) = 0;
            } else if (bVar3 < 0x9) {
                if (bVar3 == 1) {
                    pWVar11 = GetWindowLong16(0x0, hwnd_param_4);
                    iVar2 = pWVar11;
                    uVar10 = ((pWVar11 & 0xffff0000) >> 0x10);
                    (iVar2 + 0x2) = (lparam_param_1 + 0x8);
                    WVar4 = GetDlgCtrlID16(hwnd_param_4);
                    *pWVar11 = WVar4;
                    (iVar2 + 0x56) = (lparam_param_1 + 0x12);
                    unk_str_op_1000_3d3e((pWVar11 & 0xffff0000 | (iVar2 + 0x6)), *(lparam_param_1 + 0x16),
                    );
                    if ((*(lparam_param_1 + 0x12) & 1) != 0) {
                        SendMessage16(0x0, *pWVar11, 0x401, (lparam_param_1 + 0x8));
                    }
                    if (((lparam_param_1 + 0x14) & 0x800) == 0) {
                        return;
                    }
                    pbVar1 = (iVar2 + 0x4);
                    *pbVar1 = *pbVar1 | 0x4;
                    return;
                }
                if (bVar3 == 0x2) {
                    fn_ptr_1000_17ce(win_long_11);
                    SetWindowLong16(0x0, 0x0, hwnd_param_4);
                    return;
                }
//        if (bVar3 != 0x7) goto LAB_1040_a1ae;
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 | 0x8;
                LVar12 = SendMessage16(0x0, 0x0, 0x400, (iVar3 + 0x2));
                iVar2 = LVar12;
                if (((LVar12 >> 0x10) == 0x534b) && ((iVar3 + 0x5c) = iVar2, iVar2 != win_long_11)) {
                    SendDlgItemMessage16(0x1, 0x0, 0x404, iVar2, (iVar3 + 0x2));
                }
                SendMessage16(0x0, win_long_11, 0x401, (iVar3 + 0x2));
                (iVar3 + 0x56) = 0x1;
            } else if (bVar3 == 0xa) {
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 & 0xfb;
                if (wparam_param_2 == 0) {
                    pbVar1 = (iVar3 + 0x4);
                    *pbVar1 = *pbVar1 | 0x4;
                }
            } else {
                if (bVar3 != 0xc) {
                    if (bVar3 == 0xf) {
                        draw_op_1040_9948(hwnd_param_4, win_long_11);
                        return;
                    }
                    // TODO: goto LAB_1040_a1ae;
                }
                if (lparam_param_1 != 0) {
                    unk_str_op_1000_3d3e((win_long_11 & 0xffff0000 | (iVar3 + 0x6)), lparam_param_1);
                }
            }
            // TODO: goto LAB_1040_9e20;
        }
        if (msg_param_3 == 0x200) {
            if ((*(iVar3 + 0x4) & 1) == 0) {
                return;
            }
            GetClientRect16(rect_a, &DAT_1050_1050);
            iVar2 = (iVar3 + 0x4);
            BVar5 = PtInRect16(lparam_param_1, rect_a);
            if (BVar5 == 0) {
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 & 0xfd;
            } else {
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
            }
            lparam_param_1 = (iVar3 + 0x4) - iVar2;
        } else {
            if (msg_param_3 < 0x201) {
                uVar9 = msg_param_3 - 0x81;
                if (uVar9 == 0) {
                    uVar14 = 0;
                    uVar15 = 0;
                    mem_op_1000_179c(0x5e, paVar7);
                    uVar6 = paVar7 | uVar9;
                    if (uVar6 == 0) {
                        uVar9 = 0;
                        uVar6 = 0;
                    } else {
                        pass1_1040_9824(CONCAT22(paVar7, uVar9));
                    }
                    SetWindowLong16(CONCAT22(uVar6, uVar9), CONCAT11(uVar15, uVar14), hwnd_param_4);
                    return;
                }
                if (msg_param_3 == 0x87) {
                    return;
                }
                if (msg_param_3 == 0x100) {
                    if ((wparam_param_2 == 0x26) || (wparam_param_2 == 0x25)) {
                        hwnd_dlg_8 = (iVar3 + 2);
                        BVar5 = 0x1;
                    } else {
                        if ((wparam_param_2 != 0x28) && (wparam_param_2 != 0x27)) {
                            if (((wparam_param_2 == 0x20) || (wparam_param_2 == 0xd)) && (PTR_LOOP_1050_5ed8.is_null())) {
                                PTR_LOOP_1050_5ed8 = (&PTR_LOOP_1050_0000 + 1);
                                pbVar1 = (iVar3 + 0x4);
                                *pbVar1 = *pbVar1 | 0x2;
                                // TODO: goto LAB_1040_9e20;
                            }
                            // TODO: goto LAB_1040_a1ae;
                        }
                        hwnd_dlg_8 = (iVar3 + 2);
                        BVar5 = 0;
                    }
                    hwnd_dlg_8 = GetNextDlgTabItem16(BVar5, hwnd_param_4, hwnd_dlg_8);
                    SetFocus16(hwnd_dlg_8);
                    return;
                }
                if ((msg_param_3 == 0x101) && (PTR_LOOP_1050_5ed8.is_null() == false)) {
                    PTR_LOOP_1050_5ed8 = null_mut();
                    pbVar1 = (iVar3 + 0x4);
                    *pbVar1 = *pbVar1 & 0xfd;
                    InvalidateRect16(0x1, NULL, 0x0);
                    UpdateWindow16(hwnd_param_4);
                    SendMessage16(0x0, win_long_11, 0x111, (iVar3 + 0x2));
                    return;
                }//
// LAB_1040_a1ae:
                DefWindowProc16(lparam_param_1, wparam_param_2, msg_param_3, hwnd_param_4);
                return;
            }
            if (msg_param_3 == 0x201) {//
// LAB_1040_9e74:
                SetFocus16(hwnd_param_4);
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 | 0x3;
                InvalidateRect16(0x1, NULL, 0x0);
                UpdateWindow16(hwnd_param_4);
                SetCapture16(hwnd_param_4);
                return;
            }
            if (msg_param_3 == 0x202) {
                ReleaseCapture16();
                GetClientRect16(rect_a, &DAT_1050_1050);
                if ((*(iVar3 + 0x4) & 1) == 0) {
                    return;
                }
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 & 0xfc;
                InvalidateRect16(0x1, NULL, 0x0);
                UpdateWindow16(hwnd_param_4);
                BVar5 = PtInRect16(lparam_param_1, rect_a);
                if (BVar5 == 0) {
                    return;
                }
                PostMessage16(0x0, win_long_11, 0x111, (iVar3 + 0x2));
                return;
            }
//      if (msg_param_3 == 0x203) goto LAB_1040_9e74;
//      if (msg_param_3 != 0x404) goto LAB_1040_a1ae;
            if (wparam_param_2 == 1) {
                (iVar3 + 0x56) = 0x1;
            } else {
                (iVar3 + 0x56) = 0;
            }
        }
    }
    if (lparam_param_1 == 0) {
        return;
    }//
// LAB_1040_9e20:
    InvalidateRect16(0x1, NULL, 0x0);
    UpdateWindow16(hwnd_param_4);
    return;
}


pub unsafe fn pass1_1040_a204(param_1: *mut u16, param_2: u8) -> *mut u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_a2cc(mut param_1: u16, param_2: *mut u8, mut param_3: i16, mut param_4: u32, mut param_5: u32) -> u32

{
    let mut uVar1: u16;

    if (param_5 == 0x1826) {
        if ((param_5 == 1) || ((0x1 < param_5 - 0x1 && (param_5 - 0x3 < 0x2)))) {
            uVar1 = 0x1;
        } else {
            uVar1 = 0;
        }
        return uVar1;
    }
    pass1_1040_b54a(param_2, CONCAT22(param_4, param_3), (param_4 >> 0x10), param_5);
    return CONCAT22(param_2, param_1);
}

pub unsafe fn get_dlg_item_1040_a3d0(param_1: *mut astruct_49)

{
    let mut lVar1: i32;
    let mut dlg_item: HWND16;
    let mut iVar3: *mut astruct_49;
    let mut uVar2: *mut astruct_49;

    uVar2 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field143_0x90 != 0) {
        lVar1 = iVar3.field143_0x90;
        (lVar1 + 0x14) = iVar3.field6_0x6;
        dlg_item = GetDlgItem16(0x1826, iVar3.field6_0x6);
        win_msg_1040_a308(param_1, 0x0, 0x0, dlg_item);
    }
    return;
}


pub unsafe fn pass1_1040_a4c2(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    free_proc_inst_1040_a294(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_a564(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = 0;
    (param_1 + 0x4) = 0;
    (param_1 + 0x6) = 0;
    return;
}

pub unsafe fn pass1_1040_a582(param_1: u32)

{
    fn_ptr_1000_17ce(*param_1);
    return;
}

pub unsafe fn draw_op_1040_a67e(struct750_param_1: *mut astruct_750, mut param_2: i16, mut param_3: u16, mut param_4: u16)

{
    let mut bVar1: bool;
    let mut brush_handle_var2: HBRUSH16;
    let mut struct750_var4: *mut astruct_750;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut iStack14: i16;
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut iVar2: *mut astruct_751;

    uVar3 = (struct750_param_1 >> 0x10);
    struct750_var4 = struct750_param_1;
    if (struct750_var4.hbrush16_field142_0x8e == 0) {
        brush_handle_var2 = CreateSolidBrush16(WHITE_BRUSH);
        struct750_var4.hbrush16_field142_0x8e = brush_handle_var2;
    }
    if (_u16_1050_5ee8 == 0) {
        uVar4 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar2 = (uVar4 >> 0x10);
        iVar2 = uVar4;
        _u16_1050_5ee8 = CONCAT12(iVar2.field_0x94, CONCAT11(iVar2.field_0x95, iVar2.field_0x96));
        u16_1050_5eec = CONCAT11(iVar2.field_0x3e5, iVar2.field_0x3e6);
        u16_1050_5eee = iVar2.field996_0x3e4;
    }
    if (0x5 < param_3) {
        if (param_3 != 0x6) {
            return;
        }
        bVar1 = false;
        // for (iStack14 = 0; piVar1 = &struct750_var4.field233_0xea, *piVar1 != iStack14 && iStack14 <= *piVar1;
        //     iStack14 += 1)
        iStack14 = 0;
        piVar1 = struct750_param_1.field233_0xea;
        while *piVar1 != iStack14 && iStack14 <= *piVar1 {
            if ((&struct750_var4.field_0x9a + iStack14 * 0x2) == param_2) {
                bVar1 = true;
                break;
            }
            iStack14 += 1;
        }
        if (bVar1) {
            u16_1050_5ee8 = u16_1050_5eec;
            u16_1050_5eea = u16_1050_5eee;
        }
    }
    SetTextColor16(CONCAT22(u16_1050_5eea, u16_1050_5ee8), param_4);
    SetBkColor16(0x1000000, param_4);
    return;
}


pub unsafe fn win_ui_op_1040_a784(param_1: u8, mut param_2: u16, mut param_3: i16, mut param_4: i16, mut param_5: u16, mut param_6: u32)

{
    let mut hwnd: HWND16;
    let mut iVar1: i16;

    iVar1 = param_3;
    if (param_6 != 0xeb) {
        if (param_6 == 0x1f4) {
            msg_box_op_1040_a85a(0x0, param_2, CONCAT22(param_4, param_3));
            return;
        }
        if (param_6 == 0x1f7) {
            _PTR_LOOP_1050_5ef0 = (param_3 + 0x94);
            pass1_1038_af40(param_3, param_2, _PTR_LOOP_1050_5b7c, (param_3 + 0x8), 0x23);
            PostMessage16(0x0, 0x2, 0x111, (param_3 + 0x6));
            return;
        }
        if (param_6 != 0x17d8) {
            pass1_1040_b54a(param_2, CONCAT22(param_4, param_3), param_5, param_6);
            return;
        }
        SetWindowPos16(0x6, 0xed, 0x237, 0x0, 0x0, 0x0, (param_3 + 0x6));
        hwnd = GetDlgItem16(0x17d8, (param_3 + 0x6));
        iVar1 = s_tile2_bmp_1050_1538;
        EnableWindow16(0x0, hwnd);
        (param_3 + 0x98) = 0x1;
        param_4 = param_3;
    }
    win_ui_dlg_op_1040_a94a(param_2, CONCAT22(param_4, iVar1));
    return;
}


pub unsafe fn pass1_1040_a84a(param_1: u8, mut param_2: u16, mut param_3: u32)

{
    win_ui_dlg_op_1040_a94a(param_2, param_3);
    return;
}


pub unsafe fn pass1_1040_abe2(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_ad14(mut param_1: u32)

{
    let mut in_DX: u16;

    win_ui_op_1040_ae04(in_DX, param_1);
    return;
}

pub unsafe fn pass1_1040_ad24(param_1: *mut u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    if (param_5 == 0xeb) {
        win_ui_op_1040_ae04(param_1, CONCAT22(param_3, param_2));
    } else {
        if (param_5 != 0x1f0) {
            pass1_1040_b54a(param_1, CONCAT22(param_3, param_2), param_4, param_5);
            return;
        }
        msg_box_ui_op_1040_ad66(0x0, param_1, CONCAT22(param_3, param_2));
    }
    return;
}


pub unsafe fn pass1_1040_af9e(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_ace8(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_b17c(param_1: *mut u8, mut param_2: u32, mut param_3: u32)

{
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut pcVar3: *mut c_char;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut iVar6: i16;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc2: u16;
    let mut ppuVar9: *mut *mut u8;
    let mut puStack12: *mut u16;
    let mut iStack4: i16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    iStack4 = 0;
    loop {
        uVar7 = (param_2 >> 0x10);
        iVar6 = param_2;
        piVar1 = (iVar6 + 0x90);
        if (*piVar1 == iStack4 || *piVar1 < iStack4) { break; }
        paVar4 = (paVar4 & 0xffff0000 | piVar1 >> 0x10);
        uVar2 = (piVar1 + 2);
        (iStack4 * 0xa + uVar2 + 0x4) = (iStack4 * 0x2 + param_3);
        iStack4 += 0x1;
    }
    ppuVar9 = CONCAT22(unaff_SI, 0x3);
    puVar8 = mixed_1010_20ba(paVar4, _u16_1050_0ed0, ppuVar9, in_stack_0000fe94, in_stack_0000ffb8, in_stack_0000ffbe,
                             in_stack_0000ffc2);
    uVar5 = puVar8 >> 0x10;
    uVar2 = (iVar6 + 0x90);
    puStack12 = (uVar2 + 2);
//   for (iStack4 = 0; piVar1 = (iVar6 + 0x90), *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 1)
    iStack4 = 0;
    piVar1 = iVar6 + 0x90;
    while *piVar1 != iStack4 && iStack4 <= *piVar1 {
        ppuVar9 = (ppuVar9 & 0xffff0000);
        uVar2 = (iVar6 + 0x90);
        uVar2 = (uVar2 + 0x6);
        pcVar3 = pass1_1010_b038(puVar8, uVar2, (uVar2 >> 0x10),
                                 (puStack12 + 0x4), (ppuVar9 >> 0x10));
        string_1040_a626(uVar5, puStack12, CONCAT22(uVar5, pcVar3));
        puStack12 = (puStack12 & 0xffff0000 | (puStack12 + 0xa));
        iStack4 += 1;
    }
    return;
}


pub unsafe fn win_ui_op_1040_b372(mut param_1: u32, hwnd_param_2: HWND16, mut param_3: u16, hdc_param_4: HDC16)

{
    let mut uVar1: u16;
    let mut dlg_ctrl_id: i16;
    let mut local_brush_handle: HBRUSH16;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut iVar1: *mut astruct_798;
    let mut uVar3: u16;
    let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    if ((param_1 + 0x8e) == 0) {
        local_brush_handle = CreateSolidBrush16(WHITE_BRUSH);
        (param_1 + 0x8e) = local_brush_handle;
    }
    if (_PTR_LOOP_1050_5efa == 0) {
        uVar2 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar1 = (uVar2 >> 0x10);
        iVar1 = uVar2;
        _PTR_LOOP_1050_5efa = CONCAT12(iVar1.field_0x94, CONCAT11(iVar1.field_0x95, iVar1.field_0x96));
    }
    if (param_3 < 0x4) {//
// LAB_1040_b3ea:
        dlg_ctrl_id = GetDlgCtrlID16(hwnd_param_2);
        if (dlg_ctrl_id == 0x14c) {
            uVar3 = 0xffff;
            uVar6 = 0;
            // TODO: goto LAB_1040_b41a;
        }
        if (dlg_ctrl_id == 0x175) {
            uVar3 = 0xff;
            uVar6 = 0;
            // TODO: goto LAB_1040_b41a;
        }
    } else if (param_3 != 0x4) {
        if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
            return;
        }
// TODO: goto LAB_1040_b3ea;
    }
    uVar3 = _PTR_LOOP_1050_5efa;
    uVar6 = (_PTR_LOOP_1050_5efa >> 0x10);//
// LAB_1040_b41a:
    SetTextColor16(CONCAT22(uVar6, uVar3), hdc_param_4);
    SetBkColor16(0x1000000, hdc_param_4);
    return;
}


pub unsafe fn show_win_1040_b43c(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x70);
    (**ppcVar1)();
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}


pub unsafe fn pass1_1040_b4c8(param_1: *mut u8, mut param_2: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000fffa: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    uVar7 = (param_2 >> 0x10);
    if ((param_2 + 0x90) != 0) {
        puVar8 = mixed_1010_20ba(paVar5, _u16_1050_0ed0, CONCAT22(in_stack_0000fffa, 0x32), in_stack_0000fea2,
                                 in_stack_0000ffc6, in_stack_0000ffcc, in_stack_0000ffd0);
        uVar6 = paVar5 & 0xffff0000 | puVar8 >> 0x10;
        uVar3 = puVar8;
        uVar2 = (param_2 + 0x90);
        iVar1 = (uVar2 + 0xa);
        iVar4 = iVar1 - 0x4;
        if (iVar4 == 0) {
            ui_op_1010_79aa(puVar8, 0xfd9, 0x0);
            if (iVar4 == 0) {
                uVar7 = 0xe;//
// LAB_1040_b50f:
                unk_win_op_1010_7300(uVar6, (puVar8 & 0xffff0000 | uVar3), CONCAT22(iVar4, iVar4), uVar7,
                                     CONCAT22(iVar4, iVar4));
                return;
            }
        } else if (((0x0 < iVar1 - 0x5) && (!SBORROW2(iVar1 - 0x5, 1))) && (iVar4 = iVar1 - 0x7, iVar4 == 0x0 || iVar1 - 0x6 < 1)) {
            ui_op_1010_79aa(puVar8, 0xfda, 0x0);
            if (iVar4 == 0) {
                uVar7 = 0xd;
                // TODO: goto LAB_1040_b50f;
            }
        }
    }
    return;
}


pub unsafe fn pass1_1040_b74c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffde: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffde, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
