use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::draw_ops::draw_f::get_sys_metrics_1040_8c66;
use crate::draw_ops::draw_a::draw_op_1040_9948;
use crate::globals::PTR_LOOP_1050_1040;
use crate::resources::load_string_1010_84e0;
use crate::structs::struct_37::Struct37;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_915::astruct_915;
use crate::structs::struct_a::StructA;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1000_3000::unk_str_op_1000_3d3e;
use crate::unk::block_1000_5000::pass1_1000_5586;
use crate::unk::block_1008_3000::pass1_1008_3bd6;
use crate::unk::block_1008_5000::win_1008_5c7c;
use crate::unk::block_1008_9000::create_window_ex_1008_9760;
use crate::unk::block_1008_a000::pass1_1008_ab54;
use crate::unk::block_1010_0000::pass1_1010_01f8;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_4000::{pass1_1010_41d6, pass1_1010_451a, pass1_1010_459e};
use crate::unk::block_1010_a000::pass1_1010_a50c;
use crate::unk::block_1018_0000::pass1_1018_04b8;
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_9000::pass1_1040_9824;
use crate::unk::block_1040_a000::{pass1_1040_a5d0, struct_1040_a598};
use crate::unk::block_1040_b000::pass1_1040_b54a;
use crate::utils::{CONCAT11, CONCAT22};
use crate::gui::{cleanup, dialog, window};
use crate::gui::window::win_e;
use crate::winapi16::{CheckDlgButton16, DefWindowProc16, EnableWindow16, GetClientRect16, GetCursorPos16, GetDlgCtrlID16, GetDlgItem16, GetNextDlgTabItem16, GetWindowLong16, GetWindowPlacement16, GetWindowRect16, InvalidateRect16, IsWindow16, LoadIcon16, MapDialogRect16, MoveWindow16, PostMessage16, PtInRect16, ReleaseCapture16, SendDlgItemMessage16, SendMessage16, SetCapture16, SetFocus16, SetSysModalWindow, SetWindowLong16, SetWindowPlacement16, SetWindowPos16, SetWindowText16, ShowWindow16, UpdateWindow16};
use crate::winapp;
use crate::winapp::winapp_b::unk_win_msg_op_1008_9510;
use crate::winapp::winapp_a::create_window_1040_8bea;
use crate::winapp::winapp_d;
use crate::windef16::{HWND16, LPARAM, LRESULT, RECT16, WINDOWPLACEMENT16, WPARAM16};

pub unsafe fn win_ui_op_1038_a6f4(mut param_1: u16, param_2: *mut StructB)

{
  let mut lp_string: u32;
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u32;
   let mut struct_b_3: *mut StructB;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut LVar7: LRESULT;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;
  let mut uVar4: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  dialog::dialog_ui_fn_1040_78e2(param_2);
  puVar6 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(in_stack_0000ffec,0x2),in_stack_0000fe94,
                           in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
  uVar4 = (paVar2 >> 0x10);
  lp_string = (puVar6 + 0x68);
  uVar5 = (param_2 >> 0x10);
  struct_b_3 = param_2;
  hwnd = GetDlgItem16(0x115,struct_b_3.lpvoid_field_0x8);
  SetWindowText16(lp_string,hwnd);
  SetFocus16(hwnd);
  LVar7 = SendMessage16(-0x10000,0x0,0x401,hwnd);
  uVar3 = CONCAT22(uVar4,(LVar7 >> 0x10));
  uVar1 = LVar7;
  window::unk_win_ui_op_1038_a18c(uVar3, param_2, in_stack_0000ff9e);
  win_1008_5c7c(uVar1,uVar3,_u16_1050_02a0,0x30001);
  (struct_b_3 + 0x7).field0_0x0 = uVar1;
  ShowWindow16(0x5,struct_b_3.lpvoid_field_0x8);
  return;
}


pub unsafe fn FUN_1040_0f0c(mut param_1: u16, param_2: *mut StructB) {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut HVar2: HWND16;
    let mut in_EDX: u32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe6e: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff98: u16;
    let mut in_stack_0000ff9c: u16;
    let mut id: i16;
    let mut in_stack_0000ffc6: u16;
    let mut local_2e: [u8; 0x2] = [0; 0x2];
    let mut iStack44: i16;
    let mut local_26: i16;
    let mut iStack36: i16;
    let mut iStack34: i16;
    let mut iStack32: i16;
    let mut iStack30: i16;
    let mut uStack28: u16;
    let mut iStack26: i16;
    let mut iStack24: i16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut local_e: i16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut POlocal_6: INT16;

    dialog::dialog_ui_fn_1040_78e2(param_2);
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    if ((iVar4 + 0x98) == 0) {
        GetWindowRect16(CONCAT22(0x1050, &local_26), (iVar4 + 0x6));
        uVar3 = (in_EDX >> 0x10);
        HVar2 = GetDlgItem16(0x1830, (iVar4 + 0x6));
        GetWindowRect16(CONCAT22(0x1050, local_2e), HVar2);
        iStack34 -= local_26;
        iStack32 = (iStack44 - iStack36) - 0x2;
        SetWindowPos16(0x6, iStack32, iStack34, 0x0, 0x0, 0x0, (iVar4 + 0x6));
        CheckDlgButton16(0x1, 0x1c1, (iVar4 + 0x6));
        uVar1 = (iVar4 + 0x8e);
        (uVar1 + 0xa) = 0x2;
        HVar2 = GetDlgItem16(0x1830, (iVar4 + 0x6));
        in_stack_0000ffc6 = 0;
        EnableWindow16(0x0, HVar2);
    } else {
        uVar1 = (iVar4 + 0x92);
        uVar6 = struct_op_1030_73a8((uVar1 + 0x6), in_AX, in_EDX);
        uVar3 = (in_EDX >> 0x10);
        if ((uVar6 + 0x20) == 0x2) {
            HVar2 = (iVar4 + 0x6);
            id = 0x1c1;
        } else {
            HVar2 = (iVar4 + 0x6);
            id = 0x1c2;
        }
        CheckDlgButton16(0x1, id, HVar2);
    }
    GetCursorPos16(&local_6);
    GetWindowRect16(CONCAT22(0x1050, &local_e), (iVar4 + 0x6));
    iStack20 = iStack10 - local_e;
    iStack16 = -(iStack20 / 0x2 - local_6.x);
    iStack22 = iStack8 - iStack12;
    iStack18 = -(iStack22 / 0x2 - local_6.y);
    puVar7 = mixed_1010_20ba(
        CONCAT22(uVar3, iStack22 >> 0xf),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffc6, 0x48),
        in_stack_0000fe6e,
        in_stack_0000ff92,
        in_stack_0000ff98,
        in_stack_0000ff9c,
    );
    uStack28 = (puVar7 >> 0x10);
    iStack30 = puVar7;
    iStack24 = (iStack30 + 0xa);
    iStack26 = (iStack30 + 0xc);
    if (iStack24 < iStack20 + iStack16) {
        iStack16 = iStack24 - iStack20;
    }
    if (iStack26 < iStack22 + iStack18) {
        iStack18 = iStack26 - iStack22;
    }
    SetWindowPos16(0x45, 0x0, 0x0, iStack18, iStack16, 0x0, (iVar4 + 0x6));
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
            GetClientRect16(rect_a, 0x1050);
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
                GetClientRect16(rect_a, 0x1050);
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


pub unsafe fn win_ui_op_1040_8718(param_1: *mut Struct37) -> *mut u8

{
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_EDX: u32;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fd88: u16;
    let mut in_stack_0000fd8a: u16;
    let mut in_stack_0000feac: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb2: u16;
    let mut in_stack_0000feb4: u16;
    let mut in_stack_0000feb6: u16;
    let mut in_stack_0000feb8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut in_stack_0000fee0: u16;
    let mut in_stack_0000fee2: u16;
    let mut local_104: [i16; 0x80] = [0; 0x80];
    let mut uStack4: u16;
    let mut uVar8: u16;
    let mut paVar12: *mut Struct37;
    let mut uVar12: *mut Struct37;

    uVar5 = (in_EDX >> 0x10);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    paVar12 = param_1;
    uVar12 = (param_1 >> 0x10);
    dialog::dialog_ui_fn_1040_78e2(param_1);
    PTR_LOOP_1050_5df6 = (&paVar12.field1_0x4 + 2);
    if (&paVar12.field_0x94 != 0) {
        unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&paVar12.field_0x10)), *&paVar12.field_0x94);
    }
    get_sys_metrics_1040_8c66(param_1);
    uStack4 = paVar12.field138_0x98 & 0xf;
    if (uStack4 == 1) {
        iVar3 = &paVar12[0x1].field_0x8 - 0xc4;
        paVar6 = CONCAT22(uVar5, iVar3 >> 0xf);
        paVar12[0x1].field_0xc = iVar3 / 0x2;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              0x1050);
        uVar2 = &paVar12[0x1].field_0xc;
        create_window_1040_8bea(paVar12, uVar12, 0x1, 0x1, uVar2, (uVar2 >> 0x10), CONCAT22(0x1050, local_104));
        piVar1 = &paVar12[0x1].field_0xc;
        *piVar1 = *piVar1 + 0x6c;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              0x1050);
        uVar2 = &paVar12[0x1].field_0xc;
        uVar10 = uVar2;
        uVar11 = (uVar2 >> 0x10);
        uVar9 = 0x2;
    } else {
        if uStack4 != 0x4 {
            iVar3 = &paVar12[0x1].field_0x8 - 0x58;
            paVar6 = CONCAT22(uVar5, iVar3 >> 0xf);
            paVar12[0x1].field_0xc = iVar3 / 0x2;
            load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                                  0x1050);
            uVar2 = &paVar12[0x1].field_0xc;
            uVar10 = uVar2;
            uVar11 = (uVar2 >> 0x10);
            uVar5 = 0x1;
            uVar9 = 0x1;
            // TODO: goto LAB_1040_88a5;
        }
        iVar3 = &paVar12[0x1].field_0x8 - 0xc4;
        paVar6 = CONCAT22(uVar5, iVar3 >> 0xf);
        paVar12[0x1].field_0xc = iVar3 / 0x2;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              0x1050);
        uVar2 = &paVar12[0x1].field_0xc;
        create_window_1040_8bea(paVar12, uVar12, 0x1, 0x6, uVar2, (uVar2 >> 0x10), CONCAT22(0x1050, local_104));
        piVar1 = &paVar12[0x1].field_0xc;
        *piVar1 = *piVar1 + 0x6c;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              0x1050);
        uVar2 = &paVar12[0x1].field_0xc;
        uVar10 = uVar2;
        uVar11 = (uVar2 >> 0x10);
        uVar9 = 0x7;
    }
    uVar5 = 0;//
// LAB_1040_88a5:
    create_window_1040_8bea(paVar12, uVar12, uVar5, uVar9, uVar10, uVar11, CONCAT22(0x1050, local_104));
    puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000fee0, 0x48), in_stack_0000fd88,
                             in_stack_0000feac, in_stack_0000feb2, in_stack_0000feb6);
    uVar5 = (puVar7 >> 0x10);
    local_104[0] = (puVar7 + 0xa);
    uStack4 = (puVar7 + 0xc);
    iVar3 = uStack4 - &paVar12[0x1].field_0xa;
    paVar6 = (paVar6 & 0xffff0000 | (iVar3 >> 0xf));
    SetWindowPos16(0x40, &paVar12[0x1].field_0xa, &paVar12[0x1].field_0x8, iVar3 / 0x2,
                   (local_104[0] - &paVar12[0x1].field_0x8) / 0x2, 0x0, (&paVar12.field1_0x4 + 0x2),
    );
    PTR_LOOP_1050_5df4 = (&PTR_LOOP_1050_0000 + 1);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    cleanup::destroy_win_1040_8b7e(param_1);
    PTR_LOOP_1050_5df6 = null_mut();
    if (&paVar12[0x1].field_0x10 != 0) {
        puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000fee2, 0x37), in_stack_0000fd8a,
                                 in_stack_0000feae, in_stack_0000feb4, in_stack_0000feb8);
        uVar4 = pass1_1008_ab54(puVar7);
        if (uVar4 != 0) {
            PostMessage16(0x0, 0xfc, 0x111, HWND16_1050_0396);
        }
    }
    return PTR_LOOP_1050_5df8;
}


pub unsafe fn set_win_pos_1040_4ae4(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut pSVar4: *mut StructD;
    let mut uVar5: u16;
    let mut in_EDX: *mut Struct57;
    let mut paVar6: *mut Struct57;
    let mut paVar8: *mut Struct57;
    let mut iVar9: i16;
    let mut unaff_SI: u16;
    let mut uVar10: u16;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut local_24: [i16; 0x2] = [0; 0x2];
    let mut iStack32: i16;
    let mut pSStack20: *mut StructD;
    let mut pSStack16: *mut StructD;
    let mut iStack12: i16;
    let mut pSStack10: *mut StructD;
    let mut paStack6: *mut astruct_20;
    let mut paVar7: *mut Struct57;

    if param_4 == 0xeb {
        paStack6 = mixed_1010_20ba(in_EDX, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x3), in_stack_0000fe80,
                                   in_stack_0000ffa4, in_stack_0000ffaa, in_stack_0000ffae);
        paVar6 = (in_EDX & 0xffff0000 | paStack6 >> 0x10);
        pSVar4 = (param_1 + 0x90);
        if pSVar4.is_null() == false {
            pSStack10 = pSVar4;
            mem_op_1000_179c(0x18, paVar6);
            uVar3 = pSVar4;
            pSStack16 = (pSVar4 & 0xffff | paVar6 << 0x10);
            uVar5 = paVar6 | uVar3;
            paVar8 = (paVar6 & 0xffff0000);
            paVar7 = (paVar8 | uVar5);
            if (uVar5 == 0) {
                uVar3 = 0;
            } else {
                struct_1040_a598((pSVar4 & 0xffff | paVar6 << 0x10));
                paVar8 = paVar7;
            }
            (param_1 + 0x90) = uVar3;
            (param_1 + 0x92) = paVar8;
            (param_1 + 0x90) = 0x7;
            iStack12 = *(param_1 + 0x90);
            uVar3 = iStack12 * 0xa + 2;
            mem_op_1000_179c(uVar3, paVar8);
            uVar5 = paVar8;
            pSStack16 = CONCAT22(uVar5, uVar3);
            if ((uVar5 | uVar3) == 0) {
                uVar2 = (param_1 + 0x90);
                (uVar2 + 0x2) = 0;
            } else {
                pSStack16 = iStack12;
                pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iStack12, 0xa, uVar3 + 0x2, uVar5);
                uVar2 = (param_1 + 0x90);
                uVar10 = (uVar2 >> 0x10);
                iVar9 = uVar2;
                (iVar9 + 0x2) = uVar3 + 2;
                (iVar9 + 0x4) = uVar5;
            }
            uVar10 = (pSStack10 >> 0x10);
            iVar9 = pSStack10;
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x6) = (iVar9 + 0x6);
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0xa) = (iVar9 + 0xa);
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x12) = (iVar9 + 0x12);
            uVar10 = 0x1010;
            pass1_1010_a50c(paStack6, 0x10505d6a, (param_1 + 0x90));
            pSStack20 = pSStack10;
            pSStack16 = pSStack10;
            if pSStack10.is_null() == false {
                pass1_1040_a5d0(pSStack10);
                uVar10 = 0x1000;
                fn_ptr_1000_17ce(pSStack10);
            }
            ppcVar1 = (CONCAT22(param_2, param_1) + 0x70);
            (**ppcVar1)(uVar10, param_1, param_2);
        }
    } else {
        if param_4 != 0x1770 {
            pass1_1040_b54a(in_EDX, CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)),
                            param_3, param_4);
            return;
        }
        if param_4 == 0x7 {
            GetWindowRect16(CONCAT22(0x1050, local_24), param_3);
            iStack32 -= local_24[0];
            SetWindowPos16(0x2, 0x50, iStack32, 0x0, 0x0, 0x0, param_3);
        }
    }
    return;
}


pub unsafe fn win_ui_op_1040_81fe(mut param_1: u32)

{
    SetSysModalWindow((param_1 + 0x6));
    return;
}


pub unsafe fn set_win_placement_1010_010e(mut param_1: u16, mut param_2: u16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut piVar3: *mut i16;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut extraout_DX: u16;
    let mut lVar6: i32;
    let mut uVar7: u16;
    let mut local_18: WINDOWPLACEMENT16;
    let mut iStack6: i16;
    let mut iStack4: i16;

    local_18.length = 0x16;
    local_18.flags = 0;
    local_18.show_cmd = 0;
    local_18.pt_min_position.x = 0;
    local_18.pt_min_position.y = 0;
    local_18.pt_max_position.x = 0;
    local_18.pt_max_position.y = 0;
    local_18.rc_normal_position.x = 0;
    local_18.rc_normal_position.y = 0;
    iStack6 = 0;
    iStack4 = 0;
    uVar7 = param_3;
    GetWindowPlacement16(&local_18, 0x1050);
    if local_18.rc_normal_position.x == -1 {
        lVar6 = GetWindowLong16(0x0, param_3);
        uVar4 = (lVar6 >> 0x10);
        puVar5 = (lVar6 + 0xe0);
        ppcVar1 = (*puVar5 + 0x1c);
        (**ppcVar1)(0x1538, puVar5, (lVar6 + 0xe2), uVar7);
        iVar2 = puVar5;
        piVar3 = (puVar5 & 0xffff | extraout_DX << 0x10);
        local_18.show_cmd = 0x9;
        local_18.rc_normal_position.x = *piVar3;
        local_18.rc_normal_position.y = (iVar2 + 2);
        iStack6 = (iVar2 + 0x4) + *piVar3;
        iStack4 = (iVar2 + 0x2) + (iVar2 + 0x6);
        SetWindowPlacement16(&local_18, 0x1050);
    }
    return;
}

pub unsafe fn set_window_placement_1010_0070(mut param_1: u32, mut param_2: i16, mut param_3: u16) {
    let mut ppc_var1: *mut *mut code;
    let mut u_var2: u16;
    let mut pu_var3: *mut u32;
    let mut l_var4: i32;
    let mut u_var5: u16;
    let mut local_18: [u8; 0x6] = [0; 0x6];
    let mut istack18: i16;
    let mut i_stack16: i16;
    let mut istack14: i16;
    let mut istack12: i16;
    let mut istack10: i16;
    let mut istack8: i16;
    let mut u_stack6: u16;
    let mut u_stack4: u16;

    local_18 = 0x16;
    local_18._2_4_ = 0;
    istack18 = 0;
    i_stack16 = 0;
    istack14 = 0;
    istack12 = 0;
    istack10 = 0;
    istack8 = 0;
    u_stack6 = 0;
    u_stack4 = 0;
    u_var5 = param_3;
    GetWindowPlacement16(local_18, 0x1050);
    if (i_stack16 == -1) || (param_2 != 0) {
        local_18._2_4_ = 0x50001;
        l_var4 = GetWindowLong16(0x0, param_3);
        u_var2 = (l_var4 >> 0x10);
        pu_var3 = (l_var4 + 0xe0);
        ppc_var1 = (*pu_var3 + 0x38);
        (**ppc_var1)(0x1538, pu_var3, (l_var4 + 0xe2), u_var5);
        pass1_1010_01f8(param_1, CONCAT22(0x1050, local_18), pu_var3);
        SetWindowPlacement16(local_18, 0x1050);
    }
    return;
}


pub unsafe fn window_op_1020_10a0(
    param_1: *mut Struct57,
    param_2: *mut Struct57,
    struct_param_1: *mut StructA,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
    mut param_9: u16,
    mut param_10: u16,
    mut param_11: u16,
    mut param_12: u16,
    mut param_13: u16,
    mut param_14: u16,
    mut param_15: u16,
    mut param_16: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_160;
    let mut pIVar4: *mut INT16 = null_mut();
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar9: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar10: *mut u32;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let struct_1: *mut StructA;
    let mut paVar8: *mut Struct57;

    struct_1 = struct_param_1;
    uVar14 = (struct_param_1 >> 0x10);
    create_window_ex_1008_9760(struct_param_1);
    mem_op_1000_179c(0x42, param_2);
    uVar5 = param_2 | param_1;
    paVar8 = (param_2 & 0xffff0000 | uVar5);
    if (uVar5 != 0) {
        pass1_1008_3bd6(
            paVar8,
            param_1,
            param_2,
            0x0,
            0x1f009b,
            0x0,
            0x740075,
            CONCAT22(struct_1.field4_0x8, 0xf1),
            param_4,
            param_6,
            param_7,
            param_10,
            param_11,
            param_12,
        );
    }
    mem_op_1000_179c(0x42, paVar8);
    uVar5 = paVar8 | param_1;
    paVar9 = (paVar8 & 0xffff0000 | uVar5);
    if (uVar5 != 0) {
        pass1_1008_3bd6(
            paVar9,
            param_1,
            paVar8,
            0x0,
            0x31009b,
            0x0,
            0x760077,
            CONCAT22(struct_1.field4_0x8, 0xf2),
            param_4,
            param_6,
            param_7,
            param_10,
            param_11,
            param_12,
        );
    }
    mem_op_1000_179c(0x42, paVar9);
    uVar5 = paVar9 | param_1;
    paVar8 = (paVar9 & 0xffff0000 | uVar5);
    if (uVar5 != 0) {
        pass1_1008_3bd6(
            paVar8,
            param_1,
            paVar9,
            0x0,
            0x77009b,
            0x0,
            0x780079,
            CONCAT22(struct_1.field4_0x8, 0xf3),
            param_4,
            param_6,
            param_7,
            param_10,
            param_11,
            param_12,
        );
    }
    puVar10 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2d),
        param_9,
        param_5,
        param_14,
        param_15,
    );
    paVar8 = (paVar8 & 0xffff0000 | puVar10 >> 0x10);
    struct_1[0x1].field20_0x26 = puVar10;
    uVar6 = (puVar10 >> 0x10);
    struct_1[0x1].field21_0x28 = uVar6;
    struct_1[0x1].field10_0x14 = struct_1[0x1].field20_0x26;
    struct_1[0x1].field11_0x16 = uVar6;
    paVar3 = LoadIcon16(s_PLNTICON_1050_4267, HINSTANCE16_1050_038c);
    struct_1.field_0xc2 = paVar3;
    uVar1 = &struct_1[0x1].field20_0x26;
    uVar7 = uVar1;
    ppcVar2 = (*&struct_1[0x1].field20_0x26 + 0x30);
    (**ppcVar2)(0x1538, uVar7, (uVar1 >> 0x10), paVar3);
    mem_op_1000_179c(0x24, paVar8);
    uVar5 = paVar8 | paVar3;
    paVar9 = (paVar8 & 0xffff0000 | uVar5);
    if (uVar5 == 0) {
        struct_1[0x1].field22_0x2a = 0;
    } else {
        winapp_d::unk_win_ui_op_1020_1418(CONCAT22(paVar8, paVar3), struct_param_1, param_5);
        struct_1[0x1].field22_0x2a = paVar3;
        struct_1[0x1].field_0x2c = paVar9;
    }
    struct_1[0x1].field14_0x1c = &struct_1[0x1].field22_0x2a;
    puVar10 = mixed_1010_20ba(
        paVar9,
        _u16_1050_0ed0,
        CONCAT22(uVar7, 0x2f),
        param_16,
        param_4,
        param_13,
        param_14,
    );
    uVar12 = paVar9 & 0xffff0000;
    uVar11 = pass1_1018_04b8(puVar10);
    paVar8 = (uVar12 & 0xffff0000 | uVar11 >> 0x10);
    // WARNING: Load size is inaccurate
    pass1_1010_41d6(paVar8, struct_1[0x1].field20_0x26, uVar11);
    uVar12 = pass1_1010_451a(paVar8, &struct_1[0x1].field20_0x26);
    uVar7 = (uVar12 >> 0x10);
    pIVar4 = uVar12;
    uVar1 = struct_param_1;
    ppcVar2 = (uVar1 + 0x14);
    (**ppcVar2)(0x1010, struct_param_1, 0x0, uVar12, uVar7);
    uVar13 = 0x1;
    ppcVar2 = (uVar1 + 0x10);
    (**ppcVar2)();
    pass1_1010_459e(&struct_1[0x1].field20_0x26);
    ppcVar2 = (*&struct_1[0x1].field20_0x26 + 0x10);
    (**ppcVar2)(0x1010, &struct_1[0x1].field20_0x26, struct_param_1, uVar13);
    MoveWindow16(
        0x1,
        pIVar4[0x3],
        pIVar4[0x2],
        pIVar4[0x1],
        *pIVar4,
        struct_1.field4_0x8,
    );
    UpdateWindow16(struct_1.field4_0x8);
    return;
}

pub unsafe fn win_ui_op_1040_0000(
    ctx: &mut AppContext,
    pstruct57_param_1: *mut Struct57,
    pstructb_param_2: *mut StructB,
    mut param_3: u16,
) {
    let mut rect: *mut Struct57;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    // let mut unaff_SI: u16;
    let mut uVar5: u16;
    let mut puVar8: *mut u32;
    let mut uVar9: u32;
    let mut in_stack_0000fe16: u16;
    let mut in_stack_0000fe1a: u16;
    let mut in_stack_0000fe6a: u16;
    let mut in_stack_0000ff40: u16;
    let mut in_stack_0000ff44: u16;
    let mut in_stack_0000ff48: u16;
    let mut in_stack_0000ff8e: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff98: u16;
    let mut local_24: u16;
    let mut uStack34: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut local_1a: u16;
    let mut uStack24: u16;
    let mut uStack22: u16;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut iStack12: i16;
    let mut paStack10: *mut astruct_915;
    let mut paStack8: *mut Struct57;
    let mut uStack6: u16;
    let mut iStack4: i16;
    let mut iVar1: u16;
    let mut uVar4: u32;

    dialog::dialog_ui_fn_1040_78e2(pstructb_param_2);
    let iStack4 = 0x8u16;
    //   for (paStack10 = null_mut(); uVar5 = struct_b_param_1, uVar6 = (struct_b_param_1 >> 0x10),
    //   paStack10 < iStack4; paStack10 = paStack10 + 1)
    paStack10 = null_mut();
    // uVar5 = pstructb_param_2;
    // uVar6 = pstructb_param_2 >> 0x10;
    while paStack10 < iStack4 {
        iVar1 = paStack10 * 0xe;
        local_24 = (iVar1 + 0x5c60);
        uStack34 = (iVar1 + 0x5c62);
        uStack32 = 0x1;
        uStack30 = 0x1;
        rect = &local_24;
        MapDialogRect16(rect, 0x1050);
        mem_op_1000_179c(0x42, pstruct57_param_1);
        uVar2 = pstruct57_param_1 | rect;
        uVar4 = pstruct57_param_1 & 0xffff0000 | uVar2;
        if uVar2 == 0 {
            rect = null_mut();
            uVar4 = pstruct57_param_1 & 0xffff0000;
        } else {
            pass1_1008_3bd6(
                uVar4,
                rect,
                pstruct57_param_1,
                0x1,
                CONCAT22(local_24, uStack34),
                0x104,
                0x1020103,
                CONCAT22((uVar5 + 0x6), (iVar1 + 0x5c64)),
                param_3,
                in_stack_0000fe16,
                in_stack_0000fe1a,
                in_stack_0000ff40,
                in_stack_0000ff44,
                in_stack_0000ff48,
            );
        }
        uStack6 = uVar4;
        paStack8 = rect;
        let uVar7 = win_e::win_ui_op_1040_0558(pstructb_param_2, paStack10);
        pstruct57_param_1 = (uVar4 & 0xffff0000 | uVar7 >> 0x10);
        paStack10 += 1;
    }
    window::move_win_1040_826c(pstructb_param_2, -0x1, 0xffff);
    puVar8 = mixed_1010_20ba(
        pstruct57_param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x48),
        in_stack_0000fe6a,
        in_stack_0000ff8e,
        in_stack_0000ff94,
        in_stack_0000ff98,
    );
    uStack16 = (puVar8 >> 0x10);
    uStack18 = puVar8;
    iStack12 = (uStack18 + 0xa);
    uStack14 = (uStack18 + 0xc);
    GetWindowRect16(CONCAT22(0x1050, &local_1a), (uVar5 + 0x6));
    uVar3 = iStack12 >> 0xf;
    uStack28 = uStack22 - local_1a;
    local_1a = (iStack12 / 0x2 - uStack28) - 0x3;
    if (local_1a < 0x0) {
        local_1a = 0;
    }
    SetWindowPos16(0x41, 0x0, 0x0, uStack24, local_1a, 0x0, (uVar5 + 0x6));
    uVar9 = pass1_1038_af40(uVar5, uVar3, _PTR_LOOP_1050_5b7c, (uVar5 + 0x6), 0x17);
    uVar3 = (uVar9 >> 0x10);
    uVar1 = uVar9;
    (uVar5 + 0x96) = uVar1;
    (uVar5 + 0x98) = uVar3;
    win_1008_5c7c(uVar1, uVar3, _u16_1050_02a0, 0x9e0001);
    (uVar5 + 0x8c) = uVar1;
    return;
}
