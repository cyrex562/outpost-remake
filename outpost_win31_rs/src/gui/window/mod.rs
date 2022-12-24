pub mod win_a;
pub mod win_b;
pub mod win_c;
pub mod win_d;
pub mod win_e;


use std::os::raw::c_char;
use std::ffi::c_void;
use std::ptr::null_mut;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::string_defs::{s__1050_4415, s_listbox_1050_4416, s_MciSound_registerClass_failed_1050_02cc, s_MciSoundWindow_1050_02bd};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_72::Struct72;
use crate::structs::struct_878::Struct878;
use crate::structs::struct_a::StructA;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708, mem_op_1000_160a, mem_op_1000_179c};
use crate::unk::block_1000_3000::{pass1_1000_3cea, pass1_1000_3e2c, str_op_1000_3da4, sys_1000_3f9c, unk_str_op_1000_3d3e};
use crate::unk::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e0e, pass1_1008_3e38, pass1_1008_3e94, pass1_1008_3f62};
use crate::unk::block_1008_5000::{pass1_1008_5784, pass1_1008_5b12, win_1008_5c5c, win_1008_5c7c, win_1008_5c9e};
use crate::unk::block_1010_2000::{mixed_1010_20ba, pass1_1010_2ee2, unk_load_str_op_1010_2c34};
use crate::unk::block_1018_2000::{pass1_1018_25d2, pass1_1018_2678, pass1_1018_2d22, pass1_1018_2d84};
use crate::unk::block_1018_3000::{pass1_1018_30fc, pass1_1018_3a5c, pass1_1018_3a7a, pass1_1018_3a94, pass1_1018_3d44, sprintf_op_1018_34b6, string_1018_39d8, unk_str_op_1018_35b0};
use crate::unk::block_1020_b000::pass1_1020_bd80;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_b000::{pass1_1040_b54a, pass1_1040_bfde, struct_1040_bf3e};
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::{draw_ops, gui, resources, winapp};
use crate::app_context::AppContext;
use crate::draw_ops::{draw_b, unk_win_ui_op_1020_67ce};
use crate::draw_ops::draw_a::draw_op_1040_9948;
use crate::draw_ops::draw_d::load_draw_op_1020_2ede;
use crate::draw_ops::draw_f::{get_sys_metrics_1040_8c66, unk_win_ui_op_1020_717e};
use crate::globals::{PTR_LOOP_1050_1040, u32_1050_0004};
use crate::no_refs::h::pass1_1010_4f30;
use crate::no_refs::i::{pass1_1010_5f7a, pass1_1010_5fb0, pass1_1010_6006, pass1_1010_659a, pass1_1010_6604};
use crate::no_refs::l::{pass1_1010_a568, pass1_1010_a58a, pass1_1010_a5ac, pass1_1010_a5ec};
use crate::no_refs::m::pass1_1010_ecc6;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_37::Struct37;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_915::astruct_915;
use crate::unk::block_1000_4000::pass1_1000_4906;
use crate::unk::block_1000_5000::pass1_1000_5586;
use crate::unk::block_1008_4000::pass1_1008_4772;
use crate::unk::block_1008_6000::pass1_1008_6978;
use crate::unk::block_1008_9000::{create_window_ex_1008_9760, pass1_1008_941a};
use crate::unk::block_1008_a000::pass1_1008_ab54;
use crate::unk::block_1008_b000::{load_string_1008_b65a, pass1_1008_b340, pass1_1008_b366, pass1_1008_b47a, pass1_1008_b820};
use crate::unk::block_1008_e000::{pass1_1008_e2a4, pass1_1008_e320, pass1_1008_eb5c, pass1_1008_eb6e};
use crate::unk::block_1010_0000::{pass1_1010_01f8, pass1_1010_088c, pass1_1010_0892, pass1_1010_0898, pass1_1010_089e, pass1_1010_091e, pass1_1010_0932, pass1_1010_0946};
use crate::unk::block_1010_3000::{pass1_1010_32c0, win_ui_op_1010_3202};
use crate::unk::block_1010_4000::{pass1_1010_41d6, pass1_1010_451a, pass1_1010_459e};
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::unk::block_1010_a000::{pass1_1010_a50c, pass1_1010_a5ca};
use crate::unk::block_1018_0000::{pass1_1018_04b8, pass1_1018_0ac0, pass1_1018_0ad4, pass1_1018_0afa, pass1_1018_0b08};
use crate::unk::block_1018_1000::{pass1_1018_161c, pass1_1018_1662, pass1_1018_1a8e, pass1_1018_1c9a, pass1_1018_1e78};
use crate::unk::block_1018_7000::pass1_1018_6198;
use crate::unk::block_1020_0000::pass1_1020_0dc4;
use crate::unk::block_1020_1000::pass1_1020_1d8e;
use crate::unk::block_1020_2000::{pass1_1020_289a, pass1_1020_294a, pass1_1020_2a94};
use crate::unk::block_1020_5000;
use crate::unk::block_1020_6000::{pass1_1020_6498, pass1_1020_64d4};
use crate::unk::block_1028_d000::pass1_1028_dc52;
use crate::unk::block_1028_e000::pass1_1028_e4ec;
use crate::unk::block_1030_2000::{pass1_1030_2f1a, pass1_1030_2fac};
use crate::unk::block_1030_5000::pass1_1030_532e;
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e};
use crate::unk::block_1040_5000::{pass1_1040_5cd6, pass1_1040_5dc4, pass1_1040_5eaa};
use crate::unk::block_1040_8000::string_1040_8520;
use crate::unk::block_1040_9000::pass1_1040_9824;
use crate::unk::block_1040_a000::{pass1_1040_a5d0, struct_1040_a598};
use crate::gui::{cleanup, cursor, dialog};
use crate::gui::dialog::{dlg_a, dlg_b};
use crate::winapi16::{BringWindowToTop16, CheckDlgButton16, CheckRadioButton16, CreateWindow16, DefWindowProc16, DestroyWindow16, EnableWindow16, GetClassInfo16, GetClientRect16, GetCursorPos16, GetDlgCtrlID16, GetDlgItem16, GetNextDlgTabItem16, GetStockObject16, GetSystemMetrics16, GetWindowLong16, GetWindowPlacement16, GetWindowRect16, GetWindowText16, GetWindowWord16, InvalidateRect16, IsDlgButtonChecked, IsIconic16, IsWindow16, LoadCursor16, LoadIcon16, MapDialogRect16, MoveWindow16, OutputDebugString16, PostMessage16, PtInRect16, RegisterClass16, ReleaseCapture16, ScreenToClient16, SendDlgItemMessage16, SendMessage16, SetCapture16, SetCursor16, SetDlgItemInt16, SetDlgItemText16, SetFocus16, SetSysModalWindow, SetWindowLong16, SetWindowPlacement16, SetWindowPos16, SetWindowText16, ShowWindow16, UpdateWindow16};
use crate::winapp::winapp_a::{create_window_1040_6eae, create_window_1040_7620, create_window_1040_8bea, unk_win_op_1010_7300, win_proc_1008_5f44};
use crate::winapp::winapp_b::{post_win_msg_1040_7b3c, unk_win_msg_op_1008_9510};
use crate::winapp::winapp_c::{send_msg_1040_1696, send_msg_1040_3374};
use crate::windef16::{ATOM, BOOL16, HCURSOR16, HICON16, HMENU16, HWND16, LPARAM, LRESULT, RECT16, WINDOWPLACEMENT16, WNDCLASS16, WPARAM16};


pub unsafe fn show_win_1038_c044(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u16;

  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
  uVar1 = (struct_b_param_1 >> 0x10);
  ShowWindow16(0x5,(struct_b_param_1 + 0x6));
  SetFocus16((struct_b_param_1 + 0x6));
  return;
}


pub unsafe fn win_ui_op_1040_12bc(param_1: u8, struct_b_param_1: *mut StructB)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut struct_b_3: *mut StructB;
    let mut uVar3: u16;
    let mut lparam: *mut c_char;
    let mut local_54: [u8; 0x52] = [0; 0x52];

    dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar3 = (struct_b_param_1 >> 0x10);
    struct_b_3 = struct_b_param_1;
    uVar1 = &struct_b_3[0x7].field1_0x2;
    sys_1000_3f9c(CONCAT22(0x1050, local_54), s__u_1050_5cd4, (uVar1 + 0xa));
    HVar2 = GetDlgItem16(0xfd2, struct_b_3.lpvoid_field_0x8);
    SendMessage16(CONCAT22(0x1050, local_54), 0x0, 0xc, HVar2);
    SetFocus16(HVar2);
    SendMessage16(-0x10000, 0x0, 0x401, HVar2);
    move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
    lparam = load_string_1010_847e(_u16_1050_14cc, 0x531);
    HVar2 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x5, struct_b_3.lpvoid_field_0x8);
    send_msg_1040_1696(struct_b_param_1, HVar2);
    SendMessage16(lparam, 0xffff, 0x40d, HVar2);
    HVar2 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x4, struct_b_3.lpvoid_field_0x8);
    send_msg_1040_1696(struct_b_param_1, HVar2);
    SendMessage16(lparam, 0xffff, 0x40d, HVar2);
    ShowWindow16(0x5, struct_b_3.lpvoid_field_0x8);
    return;
}


pub unsafe fn win_ui_op_1040_410e(param_1: u8, struct_b_param_1: *mut StructB)

{
    let mut uVar1: u32;
    let mut in_EDX: u32;
    let mut paVar2: *mut Struct57;
    let mut struct_b_3: *mut StructB;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe52: u16;
    let mut in_stack_0000ff76: u16;
    let mut in_stack_0000ff7c: u16;
    let mut in_stack_0000ff80: u16;
    let mut piVar6: *mut i16;
    let mut uVar7: u16;
    let mut pcVar8: *mut c_char;
    let mut local_36: i16;
    let mut local_34: i16;
    let mut local_32: i16;
    let mut local_30: [u8; 0x6] = [0; 0x6];
    let mut local_2a: [i16; 0x4] = [0; 0x4];
    let mut uStack34: u32;
    let mut local_1e: u32;
    let mut uStack26: u32;
    let mut local_16: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut HStack14: HWND16;
    let mut local_c: [u8; 0xa] = [0; 0xa];

    uVar7 = (in_EDX >> 0x10);
    dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
    pass1_1000_4906(CONCAT22(0x1050, local_c), NULL, 0xa);
    uVar3 = (struct_b_param_1 >> 0x10);
    struct_b_3 = struct_b_param_1;
    uVar1 = &struct_b_3[0x7].field1_0x2;
    sys_1000_3f9c(CONCAT22(0x1050, local_c), s__lu_1050_5d38, (uVar1 + 0x76));
    HStack14 = GetDlgItem16(0xfb5, struct_b_3.lpvoid_field_0x8);
    SendMessage16(CONCAT22(0x1050, local_c), 0x0, 0xc, HStack14);
    SetFocus16(HStack14);
    SendMessage16(-0x10000, 0x0, 0x401, HStack14);
    GetWindowRect16(CONCAT22(0x1050, &local_16), struct_b_3.lpvoid_field_0x8);
    pass1_1000_4906(CONCAT22(0x1050, &local_1e), NULL, 0x8);
    uVar1 = &struct_b_3[0x7].field1_0x2;
    uStack34 = pass1_1010_5f7a(uVar1, (uVar1 >> 0x10), 0x0, 0x7);
    if (uStack34.is_null() == false) {
        local_1e = *uStack34;
        uStack26 = (uStack34 + 0x4);
    }
    if ((local_1e == 0) && (local_1e == 0)) {
        puVar4 = pass1_1008_3e38(CONCAT22(0x1050, local_30));
        paVar2 = CONCAT22(uVar7, (puVar4 >> 0x10));
        uVar1 = &struct_b_3[0x7].field5_0xa;
        pass1_1018_2678(uVar1, (uVar1 >> 0x10), CONCAT22(0x1050, local_30));
        pass1_1008_3e94(CONCAT22(0x1050, local_30), CONCAT22(0x1050, &local_32), CONCAT22(0x1050, local_2a),
        );
        pcVar8 = CONCAT22(0x1050, &local_34);
        piVar6 = &local_36;
        uVar7 = SUB42(0x1050, 0x0);
        puVar5 = mixed_1010_20ba(paVar2, _u16_1050_0ed0, CONCAT22(piVar6, 0x48), in_stack_0000fe52,
                                 in_stack_0000ff76, in_stack_0000ff7c, in_stack_0000ff80);
        pass1_1008_3e94((puVar5 & 0xffff0000 | (puVar5 + 0xe)), CONCAT22(uVar7, piVar6),
                        pcVar8);
        uStack26 = CONCAT22(iStack16 - iStack20, iStack18 - local_16);
        local_1e = CONCAT22((((puVar5 + 0xc) * -0x14) / 0x258 - (iStack16 - iStack20)) + local_36 + local_32,
                            local_34 + local_2a[0]);
    }
    move_win_1040_826c(struct_b_param_1, local_1e, local_1e);
    ShowWindow16(0x5, struct_b_3.lpvoid_field_0x8);
    return;
}

pub unsafe fn show_win_1040_2f5a(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog::dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn show_win_1040_3ae8(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog::dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn ui_op_1010_79aa(mut param_1: u32, mut param_2: i16, param_3: i32) {
    let mut hwnd: HWND16;
    let mut uVar1: u32;
    let mut puVar2: *mut u8;
    let mut extraout_DX: u16;
    let mut uVar3: u16;
    let mut lStack18: i32;
    let mut lStack14: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    if (((param_1 + 0x1c) != 0) && (param_3 != 0x0 || (param_2 != 0))) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x1c));
        lStack18 = 0;
        loop {
            puVar2 = local_a;
            pass1_1008_5b12(CONCAT22(0x1050, puVar2));
            lStack14 = CONCAT22(extraout_DX, puVar2);
            //      if ((extraout_DX | puVar2) == 0) goto LAB_1010_7a49;
            if (((param_2 == 0) && ((puVar2 + 0x4) == param_3)) || (param_3 == 0x0 && (uVar1 = (puVar2 + 0x8), (uVar1 + 0xa) == param_2))) {
                break;
            }
            if !(((puVar2 + 0x4) != param_3) || (uVar1 = (puVar2 + 0x8), (uVar1 + 0xa) != param_2)) {
                break;
            }
        }
        lStack18 = lStack14; //
        // LAB_1010_7a49:
        if (lStack18 != 0) {
            uVar1 = (lStack18 + 0x8);
            hwnd = (uVar1 + 0x6);
            SetFocus16(hwnd);
            BringWindowToTop16(hwnd);
            return;
        }
    }
    return;
}


pub unsafe fn unk_win_ui_op_1040_8158(param_1: u32, param_2: INT16, mut param_3: i16)

{
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut iVar3: i16;
    let mut uVar4: u16;

    if param_3 == 0x2 {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        if (iVar3 + 0x76) != 0 {
            ScreenToClient16(CONCAT22(0x1050, &stack0xfffa), (iVar3 + 0x6));
            GetSystemMetrics16(SM_CYCAPTION);
            BVar2 = PtInRect16((param_1 & 0xffff0000 | ZEXT24((iVar3 + 0x82))),
                               (iVar3 + 0x82));
            if BVar2 != 0 {
                ppcVar1 = (*param_1 + 0x14);
                (**ppcVar1)(0x1538, iVar3);
            }
        }
    }
    return;
}


pub unsafe fn win_ui_op_1010_0240(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) -> BOOL16 {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut WVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000fff8: u32;
    let mut uVar6: u16;

    uVar6 = (in_stack_0000fff8 >> 0x10);
    paVar4 = CONCAT22(in_register_0000000a, param_1);
    BVar2 = IsWindow16(param_4);
    if (BVar2 != 0) {
        WVar3 = GetWindowWord16(-0x6, param_4);
        if (WVar3 == HINSTANCE16_1050_038c) {
            BVar2 = IsIconic16(param_4);
            if (BVar2 != 0) {
                puVar5 = mixed_1010_20ba(
                    paVar4,
                    _u16_1050_0ed0,
                    CONCAT22(uVar6, 0x45),
                    in_stack_0000fea2,
                    in_stack_0000ffc6,
                    in_stack_0000ffcc,
                    in_stack_0000ffd0,
                );
                ppcVar1 = ((puVar5 & 0xffff0000 | param_4) + 0x10);
                (**ppcVar1)(0x1538, puVar5, (puVar5 >> 0x10), 1);
            }
        }
    }
    return 0x1;
}

pub unsafe fn invalidate_rect_1020_157c(mut param_1: u32, mut param_2: i16) {
    let mut BVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut local_a: RECT16;
    let mut uStack4: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar2 + 0x14) = 0;
        return;
    }
    if (param_2 == 0x2) {
        BVar1 = IsIconic16((iVar2 + 0x4));
        if (BVar1 == 0) {
            local_a.x = (iVar2 + 0x4);
            GetClientRect16(&local_a, 0x1050);
            uStack4 = 0x9a;
            InvalidateRect16(0x0, &local_a, 0x1050);
            return;
        }
    }
    return;
}

pub unsafe fn get_win_ui_info_op_1020_7a50(param_1: *mut astruct_868) {
    let mut ppcVar1: *mut *mut code;
    let mut b_var2: bool;
    let mut iVar2: *mut astruct_868;
    let mut var5: u16;

    var5 = (param_1 >> 0x10);
    iVar2 = param_1;
    b_var2 = IsIconic16(iVar2.field8_0x8);
    if (b_var2 == 0) {
        GetWindowRect16(CONCAT22(0x1050, &stack0xfff6), iVar2.field8_0x8);
        GetSystemMetrics16(SM_CXBORDER);
        GetSystemMetrics16(SM_CYBORDER);
    }
    ppcVar1 = (*&iVar2.field_0xe0 + 0x14);
    (**ppcVar1)();
    return;
}


pub unsafe fn window_op_1020_38aa(
    param_1: *mut Struct57,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
) {
    let mut hwnd: HWND16;
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut paVar3: *mut astruct_126;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut paVar9: *mut Struct57;
    let mut uVar11: u32;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut puVar13: *mut u32;
    let mut in_stack_0000fe08: u16;
    let mut in_stack_0000fe0c: u16;
    let mut in_stack_0000ff32: u16;
    let mut in_stack_0000ff36: u16;
    let mut in_stack_0000ff3a: u16;
    let mut uVar14: u16;
    let mut local_12: i16;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut local_a: [i16; 0x2] = [0; 2];
    let mut iStack6: i16;
    let mut iStack4: i16;
    let pstructa_hi: *mut StructA;
    let pstructa_1: *mut StructA;
    let mut paVar10: *mut Struct57;

    pstructa_1 = param_2;
    uVar14 = (param_2 >> 0x10);
    create_window_ex_1008_9760(param_2);
    puVar13 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x6),
        param_7,
        param_8,
        param_5,
        param_6,
    );
    paVar9 = (param_1 & 0xffff0000);
    pstructa_1[0x1].field25_0x2e = puVar13;
    iVar6 = (puVar13 >> 0x10);
    pstructa_1[0x1].field26_0x30 = iVar6;
    pstructa_1[0x1].field10_0x14 = pstructa_1[0x1].field25_0x2e;
    pstructa_1[0x1].field11_0x16 = iVar6;
    if (param_2.is_null()) {
        paVar3 = null_mut();
    } else {
        paVar9 = (paVar9 | uVar14);
        paVar3 = &pstructa_1[0x1].field20_0x26;
    }
    ppcVar1 = (*&pstructa_1[0x1].field25_0x2e + 0x4);
    (**ppcVar1)(0x1010, &pstructa_1[0x1].field25_0x2e, 0x0, paVar3, paVar9);
    pass1_1018_1a8e(paVar9, &pstructa_1[0x1].field25_0x2e);
    mem_op_1000_179c(0x20, paVar9);
    uVar7 = paVar9 | paVar3;
    paVar10 = (paVar9 & 0xffff0000 | uVar7);
    if (uVar7 == 0) {
        pstructa_1[0x1].field22_0x2a = 0;
    } else {
        draw_b::unk_draw_op_1020_3da4(uVar7, param_3, CONCAT22(paVar9, paVar3), param_2);
        pstructa_1[0x1].field22_0x2a = paVar3;
        pstructa_1[0x1].field_0x2c = paVar10;
    }
    uVar5 = &pstructa_1[0x1].field22_0x2a;
    pstructa_1[0x1].field14_0x1c = uVar5;
    mem_op_1000_179c(0x42, paVar10);
    paVar4 = uVar5;
    uVar7 = paVar10 | paVar4;
    paVar9 = (paVar10 & 0xffff0000 | uVar7);
    if (uVar7 == 0) {
        pstructa_1[0x1].field_0x36 = 0;
    } else {
        pass1_1008_3bd6(
            paVar9,
            paVar4,
            paVar10,
            0x0,
            0xf1,
            0x0,
            0x1ac01ad,
            CONCAT22(pstructa_1.field4_0x8, 0xf4),
            param_4,
            in_stack_0000fe08,
            in_stack_0000fe0c,
            in_stack_0000ff32,
            in_stack_0000ff36,
            in_stack_0000ff3a,
        );
        pstructa_1[0x1].field_0x36 = paVar4;
        pstructa_1[0x1].field32_0x38 = paVar9;
    }
    uVar12 = 0x1000;
    mem_op_1000_179c(0x42, paVar9);
    uVar7 = paVar9 | paVar4;
    uVar11 = paVar9 & 0xffff0000 | uVar7;
    if (uVar7 == 0) {
        pstructa_1[0x1].field_0x3a = 0;
    } else {
        uVar12 = 0x1008;
        pass1_1008_3bd6(
            uVar11,
            paVar4,
            paVar9,
            0x0,
            0xf500f1,
            0x0,
            0x1ae01af,
            CONCAT22(pstructa_1.field4_0x8, 0xf5),
            param_4,
            in_stack_0000fe08,
            in_stack_0000fe0c,
            in_stack_0000ff32,
            in_stack_0000ff36,
            in_stack_0000ff3a,
        );
        pstructa_1[0x1].field_0x3a = paVar4;
        pstructa_1[0x1].field_0x3c = uVar11;
    }
    uVar5 = &pstructa_1[0x1].field25_0x2e;
    ppcVar1 = (*&pstructa_1[0x1].field25_0x2e + 0x10);
    (**ppcVar1)(uVar12, uVar5, (uVar5 >> 0x10));
    uVar12 = uVar11;
    uVar7 = paVar4.field1_0x2;
    paVar9 = (uVar11 & 0xffff0000 | uVar7);
    uVar7 = MoveWindow16(
        0x1,
        paVar4.field3_0x6,
        paVar4.field2_0x4,
        uVar7,
        paVar4.field0_0x0,
        pstructa_1.field4_0x8,
    );
    uVar12 = 0x1000;
    mem_op_1000_179c(0x8e, paVar9);
    uVar8 = paVar9 | uVar7;
    if (uVar8 == 0) {
        pstructa_1[0x1].field37_0x3e = 0;
    } else {
        uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        gui::get_sys_metrics_1040_7728(
            CONCAT22(paVar9, uVar7),
            0x1,
            0x0,
            0xfc0,
            pstructa_1.field4_0x8,
        );
        pstructa_1[0x1].field37_0x3e = uVar7;
        (&pstructa_1[0x1].field37_0x3e + 0x2) = uVar8;
    }
    uVar2 = pstructa_1[0x1].field37_0x3e;
    (uVar2 + 0x74) = 0x1;
    uVar2 = pstructa_1[0x1].field37_0x3e;
    ppcVar1 = (pstructa_1[0x1].field37_0x3e + 0x8);
    (**ppcVar1)(uVar12, uVar2, (uVar2 >> 0x10));
    if (((&pstructa_1[0x1].field37_0x3e + 0x2) | &pstructa_1[0x1].field37_0x3e) != 0) {
        uVar2 = pstructa_1[0x1].field37_0x3e;
        hwnd = (uVar2 + 0x6);
        GetWindowRect16(
            CONCAT13(0x10, CONCAT12(0x50, local_a)),
            pstructa_1.field4_0x8,
        );
        GetWindowRect16(CONCAT22(0x1050, &local_12), hwnd);
        iStack12 -= iStack16;
        iStack14 = iStack6 - local_a[0];
        local_12 = local_a[0];
        iStack16 = iStack4 + 0x3;
        SetWindowPos16(0x44, iStack12, iStack14, iStack16, local_a[0], 0x0, hwnd);
    }
    return;
}


pub unsafe fn unk_win_ui_op_1038_a18c(param_1: *mut Struct57, param_2: *mut StructB, mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u8;
  let mut uVar3: u16;
  let mut IVar4: i16;
  let mut uVar5: u32;
  let mut puVar6: *mut u16;
  let mut in_stack_0000fe56: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000ff7a: u16;
  let mut in_stack_0000ff80: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff86: u16;
  let mut in_stack_0000ff8a: u16;
  let mut piVar7: *mut i16;
  let mut uVar8: u8;
  let mut uVar9: u8;
  let mut local_2c: [i16;0x2] = [0;0x2];
  let mut iStack40: i16;
  let mut puStack36: *mut u32;
  let mut iStack32: i16;
  let mut uStack30: u16;
  let mut local_1c: i16;
  let mut local_1a: [u8;0x2] = [0;0x2];
  let mut uStack24: u32;
  let mut puStack20: *mut u32;
  let mut local_10: i16;
  let mut local_e: bool;
  let mut local_c: [u8;0x6] = [0;0x6];
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_3,0x27),in_stack_0000fe5c,
                             in_stack_0000ff80,in_stack_0000ff86,in_stack_0000ff8a);
  uVar5 = param_1 & 0xffff0000;
  puVar6 = pass1_1008_3e38(CONCAT22(0x1050,local_c));
  uVar5 = uVar5 & 0xffff0000 | puVar6 >> 0x10;
  pass1_1008_3f62(CONCAT22(0x1050,local_c),
                  (puStack6 & 0xffff0000 | (puStack6 + 0x52)));
  puVar2 = local_c;
  pass1_1008_3e94(CONCAT22(0x1050,puVar2),CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_e));
  uVar3 = FUN_1010_830a(puVar2,uVar5,0x1008,_u16_1050_14cc,0x1c0);
  puStack20 = CONCAT22(uVar5,uVar3);
  uStack24 = pass1_1008_4772(CONCAT22(uVar5,uVar3));
  puVar2 = local_1a;
  piVar7 = &local_1c;
  uVar8 = 0x50;
  uVar9 = 0x10;
  puStack36 = mixed_1010_20ba((uVar5 & 0xffff0000 | uStack24 >> 0x10),_u16_1050_0ed0,
                              CONCAT22(piVar7,0x48),in_stack_0000fe56,in_stack_0000ff7a,in_stack_0000ff80,
                              in_stack_0000ff84);
  pass1_1008_3e94((puStack36 & 0xffff0000 | (puStack36 + 0xe)),
                  CONCAT13(uVar9,CONCAT12(uVar8,piVar7)),CONCAT22(0x1050,puVar2));
  uVar3 = (puStack36 >> 0x10);
  uStack30 = (puStack36 + 0xa);
  iStack32 = (puStack36 + 0xc);
  local_10 += (iStack32 * 0xa) / 0x258 + (uStack24 + 0x8) + local_1c;
  GetWindowRect16(CONCAT22(0x1050,local_2c),(param_2 + 0x6));
  IVar4 = GetSystemMetrics16(SM_CXSCREEN);
  local_e = (IVar4 - (iStack40 - local_2c[0])) / 0x2;
  move_win_1040_826c(param_2, local_10, local_e);
  if (puStack20.is_null() == false) {
    uVar3 = (puStack20 >> 0x10);
    ppcVar1 = *puStack20;
    (**ppcVar1)(&PTR_LOOP_1050_1040,puStack20,uVar3,0x1,uVar3);
  }
  return;
}

pub unsafe fn set_win_pos_1038_abdc(param_1: *mut astruct_940)

{
  let mut hwnd: HWND16;
  let mut iVar1: *mut astruct_940;
  let mut uVar1: u16;
  let mut in_stack_0000fff0: i16;
  let mut local_a: i16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  GetWindowRect16(CONCAT22(0x1050,&local_a),iVar1.field6_0x6);
  hwnd = GetDlgItem16(0xfd7,iVar1.field6_0x6);
  GetWindowRect16(CONCAT22(0x1050,&stack0xffee),hwnd);
  iStack6 -= local_a;
  iStack4 = (in_stack_0000fff0 - iStack8) -0x2;
  SetWindowPos16(0x6,iStack4,iStack6,0x0,0x0,0x0,iVar1.field6_0x6);
  return;
}


pub unsafe fn set_win_pos_1038_c31a(mut param_1: u32, mut param_2: u16, mut param_3: i16) -> BOOL16

{
  let mut iStack10: i16;

  if (param_3 == 1) {
    win_e::enable_win_1038_c294(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16(CONCAT22(0x1050,&stack0xfff2),param_2);
    SetWindowPos16(0x2,0x50,iStack10 - param_2,0x0,0x0,0x0,param_2);
  }
  return 0x1;
}


pub unsafe fn set_win_pos_1040_331a(mut param_1: u32, mut param_2: u16, mut param_3: i16) -> BOOL16

{
  let mut iStack10: i16;

  if (param_3 == 1) {
    win_e::enable_win_1040_32a8(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16(CONCAT22(0x1050,&stack0xfff2),param_2);
    SetWindowPos16(0x2,0x50,iStack10 - param_2,0x0,0x0,0x0,param_2);
  }
  return 0x1;
}

pub unsafe fn move_win_1040_826c(param_1: *mut StructB, param_2: INT16, param_3: BOOL16)

{
  let mut IVar1: i16;
  let mut struct_b_1_hi: u16;
  let mut local_e: i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut IStack6: i16;
  let mut BStack4: bool;

  struct_b_1_hi = (param_1 >> 0x10);
  GetWindowRect16(CONCAT22(0x1050,&local_e),(param_1 + 0x6));
  if ((param_3 == 0xffff) || (param_2 == -1)) {
    IVar1 = GetSystemMetrics16(SM_CXSCREEN);
    BStack4 = (IVar1 - (iStack10 - local_e)) / 0x2;
    IVar1 = GetSystemMetrics16(SM_CYSCREEN);
    param_2 = (IVar1 - (iStack8 - iStack12)) / 0x2;
  }
  else {
    BStack4 = param_3;
  }
  IVar1 = (param_1 + 0x6);
  IStack6 = param_2;
  MoveWindow16(0x0,IVar1,iStack10 - local_e,param_2,BStack4,IVar1);
  return;
}


pub unsafe fn unk_win_ui_op_1040_b230(mut param_1: u16, param_2: *mut StructB)

{
  let mut ppcVar1: *mut *mut code;
  let mut cy_caption_1: i16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u16;
  let mut in_stack_0000fe6e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9c: u16;
  let mut piVar3: *mut i16;
  let mut uVar4: u16;
  let mut pcVar5: *mut c_char;
  let mut local_1a: i16;
  let mut iStack24: i16;
  let mut iStack22: i16;
  let mut iStack20: i16;
  let mut iStack18: i16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut puStack10: *mut u32;
  let mut local_6: i16;
  let mut local_4: i16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  dialog::dialog_ui_fn_1040_78e2(param_2);
  if (PTR_LOOP_1050_5ef8 == (&u32_1050_0004 + 1)) {
    PTR_LOOP_1050_5ef8 = null_mut();
  }
  pcVar5 = CONCAT22(0x1050,&local_4);
  piVar3 = &local_6;
  uVar4 = SUB42(0x1050,0x0);
  puStack10 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(piVar3,0x48),in_stack_0000fe6e,
                              in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
  pass1_1008_3e94((puStack10 & 0xffff0000 | (puStack10 + 0xe)),CONCAT22(uVar4,piVar3),
                  pcVar5);
  uVar3 = (puStack10 >> 0x10);
  iStack12 = (puStack10 + 0xa);
  iStack14 = (puStack10 + 0xc);
  cy_caption_1 = GetSystemMetrics16(SM_CYCAPTION);
  iStack16 = cy_caption_1 * PTR_LOOP_1050_5ef8 + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
  iStack18 = iStack16 + local_6;
  iStack16 += local_4;
  uVar4 = (param_2 >> 0x10);
  GetWindowRect16(CONCAT22(0x1050,&local_1a),(param_2 + 0x6));
  if (iStack14 < (iStack20 - iStack24) + iStack18) {
    iStack18 = -0x2 - ((iStack20 - iStack24) - iStack14);
  }
  if (iStack12 < (iStack22 - local_1a) + iStack16) {
    iStack16 = -0x2 - ((iStack22 - local_1a) - iStack12);
  }
  SetWindowPos16(0x1,0x0,0x0,iStack18,iStack16,0x0,(param_2 + 0x6));
  ppcVar1 = (param_2 + 0x6c);
  (**ppcVar1)(0x1538,param_2);
  return;
}


pub unsafe fn set_win_pos_1040_162a(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u32) -> u32

{
    let mut uVar1: u16;
    let mut BVar2: bool;
    let mut iStack6: i16;

    if ((param_4 != s_vrpal_bmp_1050_183a + 0x5) && (param_4 != s_vrpal_bmp_1050_183a + 0x4)) {
        BVar2 = post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_3, param_4, param_4);
        return CONCAT22(param_1, BVar2);
    }
    if (param_4 == 0x7) {
        GetWindowRect16(CONCAT22(0x1050, &stack0xfff6), param_3);
        SetWindowPos16(0x2, 0x50, iStack6 - param_3, 0x0, 0x0, 0x0, param_3);
    } else if ((param_4 != 0x9) && (param_4 != 0xa)) {
        uVar1 = 0;
// TODO: goto LAB_1040_164d;
    }
    uVar1 = 0x1;//
// LAB_1040_164d:
    return uVar1;
}


pub unsafe fn get_win_rect_1040_43ea(mut param_1: i16, mut param_2: u16)

{
    let mut uVar1: u32;
    let mut local_a: u32;
    let mut iStack6: i16;
    let mut iStack4: i16;

    GetWindowRect16(CONCAT22(0x1050, &local_a), (param_1 + 0x6));
    iStack6 -= local_a;
    iStack4 -= local_a;
    pass1_1010_5fb0((param_1 + 0x8e), 0x0, &local_a, 0x1050, 0x7);
    uVar1 = (param_1 + 0x8e);
    (uVar1 + 0x7a) = ((param_1 + 0x9a) == 0);
    return;
}


pub unsafe fn win_ui_op_1040_5800(param_1: *mut u8, param_2: *mut astruct_18, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut paVar5: *mut astruct_18;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut hwnd: HWND16;
    let mut uVar6: u16;
    let mut puVar7: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut paVar10: *mut Struct57;
    let mut iVar11: i16;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut piStack24: *mut i16;
    let mut local_14: [u8; 0x8] = [0; 0x8];
    let mut iStack12: i16;
    let mut pSStack10: *mut StructD;
    let mut paStack6: *mut astruct_20;
    let mut pSVar5: *mut StructD;
    let mut paVar9: *mut Struct57;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 == 0xeb) {
        paStack6 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x3), in_stack_0000fe80,
                                   in_stack_0000ffa4, in_stack_0000ffaa, in_stack_0000ffae);
        paVar8 = (paVar8 & 0xffff0000 | paStack6 >> 0x10);
        pSVar5 = &param_2.field138_0x90;
        if (pSVar5.is_null() == false) {
            pSStack10 = pSVar5;
            // 0x0018
            mem_op_1000_179c(0x18, paVar8);
            uVar3 = pSVar5;
            uVar6 = paVar8 | uVar3;
            paVar10 = (paVar8 & 0xffff0000);
            paVar9 = (paVar10 | uVar6);
            if (uVar6 == 0) {
                uVar3 = 0;
            } else {
                struct_1040_a598((pSVar5 & 0xffff | paVar8 << 0x10));
                paVar10 = paVar9;
            }
            param_2.field138_0x90 = uVar3;
            param_2.field139_0x92 = paVar10;
            *&param_2.field138_0x90 = 0x6;
            iStack12 = *&param_2.field138_0x90;
            uVar3 = iStack12 * 0xa + 2;
            mem_op_1000_179c(uVar3, paVar10);
            uVar6 = paVar10;
            piStack24 = CONCAT22(uVar6, uVar3);
            puVar7 = (uVar6 | uVar3);
            if (puVar7.is_null()) {
                uVar2 = &param_2.field138_0x90;
                (uVar2 + 0x2) = 0;
            } else {
                *piStack24 = iStack12;
                // &PTR_LOOP_1050_1040 actually 0x1040
                pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iStack12, 0xa, uVar3 + 0x2, uVar6);
                uVar2 = &param_2.field138_0x90;
                uVar12 = (uVar2 >> 0x10);
                iVar11 = uVar2;
                (iVar11 + 0x2) = uVar3 + 2;
                (iVar11 + 0x4) = uVar6;
            }
            uVar2 = &param_2.field138_0x90;
            (uVar2 + 0x6) = (pSStack10 + 0x6);
            uVar2 = &param_2.field138_0x90;
            (uVar2 + 0xa) = 0x4;
            uVar2 = &param_2.field138_0x90;
            (uVar2 + 0x12) = &param_2.field_0xa;
            uVar12 = 0x1010;
            pass1_1010_a50c(paStack6, &u32_1050_5d78, &param_2.field138_0x90);
            if (pSStack10.is_null() == false) {
                pass1_1040_a5d0(pSStack10);
                uVar12 = 0x1000;
                fn_ptr_1000_17ce(pSStack10);
            }
            ppcVar1 = (CONCAT22(param_3, param_2) + 0x70);
            (**ppcVar1)(uVar12, param_2, param_3);
            uVar4 = pass1_1040_5cd6(CONCAT22(param_3, param_2));
            if (uVar4 != 0) {
                pass1_1040_5eaa(CONCAT22(param_3, param_2));
                param_2.field_0x94 = 0;
            }
            pass1_1040_5dc4(puVar7, CONCAT22(param_3, param_2));
            GetWindowRect16(CONCAT13(0x10, CONCAT12(0x50, local_14)), param_2.hwnd_0x6);
            InvalidateRect16(param_2[0x1].base_0x0, NULL, 0x0);
            if (param_2[0x1].base_0x0 != 0) {
                param_2[0x1].base_0x0 = 0;
            }
        }
    } else {
        if (param_5 != 0x13b) {
            pass1_1040_b54a(param_1, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), param_4,
                            param_5);
            return;
        }
        hwnd = GetDlgItem16(0x1790, param_2.hwnd_0x6);
        EnableWindow16(0x1, hwnd);
    }
    return;
}

pub unsafe fn invalidate_rect_1020_1fb2(mut param_1: u32, mut param_2: i16) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut local_16: u16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut local_e: [i16; 0x2] = [0; 2];
    let mut iStack10: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar1 + 0x6) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    GetWindowRect16(CONCAT22(0x1050, local_e), (iVar1 + 0x4));
    uStack6 = 0x46;
    uStack20 = 0x46;
    iStack18 = iStack10 - local_e[0];
    uStack4 = 0x5f;
    uStack16 = 0x5f;
    local_16 = (iVar1 + 0x4);
    InvalidateRect16(0x0, &local_16, 0x1050);
    return;
}

pub unsafe fn win_ui_op_1040_52c0(param_1: *mut u8, param_2: *mut astruct_894, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut is_window: bool;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut uVar9: u32;
    let mut puVar10: *mut u32;
    let mut paVar11: *mut astruct_940;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb4: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut in_stack_0000ffde: u16;
    let mut uVar15: u16;
    let mut uStack30: u16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut hwnd_8: HWND16;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 != 0x10c) {
        if (param_5 < 0x10d) {
            if (param_5 == 0xfa) {
                ppcVar1 = (*param_2.field148_0x98 + 0x18);
                (**ppcVar1)();
                return;
            }
            if (param_5 == 0x10a) {
                GetClientRect16(&local_a, 0x1050);
                puVar2 = param_2.field148_0x98;
                local_a.y += 0x3;
                local_a.x = (puVar2 + 0x1a) - 0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(0x1, &local_a, 0x1050);
                cleanup::unk_destroy_win_op_1010_2fa0(param_2.field148_0x98);
                pass1_1010_32c0(param_2.field148_0x98, 0x0);
                pass1_1010_2ee2(param_2.field148_0x98);
                return;
            }
            if (param_5 != 0x10b) {//
// LAB_1040_5560:
                pass1_1040_b54a(param_1, CONCAT22(param_3, param_2), param_4, param_5);
                return;
            }
            puVar2 = param_2.field148_0x98;
            uVar12 = (puVar2 + 0x12);
            uVar5 = uVar12;
            puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(uVar12, 0x3), in_stack_0000fe84,
                                      in_stack_0000ffa8, in_stack_0000ffae, in_stack_0000ffb2);
            uVar7 = (puVar10 >> 0x10);
            uStack30 = puVar10;
            uVar4 = uStack30;
            uVar6 = uVar7;
            pass1_1010_a5ca(uStack30, uVar7, uStack30, uVar7, uVar12);
            if ((uVar5 != 0x70) && (uVar4 == 0)) {
                return;
            }
            uVar9 = param_2.field169_0xb0;
            uVar13 = uVar9;
            uVar14 = (uVar9 >> 0x10);
            puVar2 = param_2.field148_0x98;
            uVar12 = (puVar2 + 0x12);
        } else {
            if (param_5 != 0x10d) {
                if (param_5 == 0x10e) {
                    puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x32),
                                              in_stack_0000fe86, in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                    uVar9 = paVar8 & 0xffff0000 | puVar10 >> 0x10;
                    uVar3 = puVar10;
                    uVar15 = uVar3;
                    ui_op_1010_79aa(puVar10, 0xfc6, param_2.field169_0xb0);
                    if uVar3 != 0 {
                        return;
                    }
                    unk_win_op_1010_7300(uVar9, (puVar10 & 0xffff0000 | uVar15), 0x0, 0x13, param_2.field169_0xb0);
                    return;
                }
                if param_5 != 0xbbb {
                    if param_5 == 0xbbc {
                        puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x3),
                                                  in_stack_0000fe86, in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                        uVar12 = (puVar10 >> 0x10);
                        uVar4 = puVar10;
                        uVar7 = uVar12;
                        uVar5 = pass1_1010_a5ac(uVar4, uVar12, param_2.field169_0xb0);
                        uVar6 = uVar5;
                        pass1_1010_a58a(uVar5, uVar7, uVar4, uVar12, uVar5);
                        if uVar6 == 0 {
                            pass1_1010_a568(0x0, uVar7, uVar4, uVar12, uVar5);
                        }
                        hwnd_8 = GetDlgItem16(0xbbc, param_2.hwnd_0x6);
                        EnableWindow16(0x0, hwnd_8);
                        return;
                    }
                    // TODO: goto LAB_1040_5560;
                }
                if (param_2.field171_0xb6 == 0) || (is_window = IsWindow16(param_2.field171_0xb6), is_window == 0) {
                    paVar11 = pass1_1038_af40(param_2, paVar8, _PTR_LOOP_1050_5b7c, param_2.hwnd_0x6, 0x1b);
                    param_2.field171_0xb6 = (paVar11 + 0x6);
                    set_win_pos_1038_abdc(paVar11);
                    ShowWindow16(0x1, param_2.field171_0xb6);
                    return;
                }
                hwnd_8 = param_2.field171_0xb6;
                // TODO: goto LAB_1040_5417;
            }
            puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x3), in_stack_0000fe86,
                                      in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
            uVar6 = (puVar10 >> 0x10);
            uStack30 = puVar10;
            uVar9 = param_2.field169_0xb0;
            uVar13 = uVar9;
            uVar14 = (uVar9 >> 0x10);
            uVar12 = 0x71;
            uVar7 = uVar6;
        }
        pass1_1010_a5ec(uVar6, uStack30, uVar7, uVar12, CONCAT22(uVar14, uVar13));
        if (param_2.hwnd_0xb4 != 0) && (is_window = IsWindow16(param_2.hwnd_0xb4), is_window != 0) {
            SendMessage16(0x0, 0xeb, 0x111, param_2.hwnd_0xb4);
        }
    }
    hwnd_8 = param_2.hwnd_0x6;//
// LAB_1040_5417:
    DestroyWindow16(hwnd_8);
    return;
}
