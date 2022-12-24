pub mod window;
pub mod dialog;
pub mod msg_box;
pub mod cursor;
pub mod cleanup;
pub mod menu;

use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708, mem_op_1000_160a, mem_op_1000_179c};
use crate::unk::block_1000_3000::{pass1_1000_3cea, pass1_1000_3e2c, str_op_1000_3da4, str_op_1000_3dbe, sys_1000_3f9c, unk_str_op_1000_3d3e};
use crate::unk::block_1000_4000::{pass1_1000_4906, pass1_1000_4f2e, str_1000_4d58};
use crate::unk::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e0e, pass1_1008_3e38, pass1_1008_3e94, pass1_1008_3f62, struct_op_1008_3f92};
use crate::unk::block_1008_5000::{pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_1008_5fd8, set_struct_1008_574a, win_1008_5c5c, win_1008_5c7c, win_1008_5c9e};
use crate::unk::{block_1008_8000, block_1038_8000, block_1038_9000, block_1038_c000, block_1040_1000, block_1040_3000};
use crate::unk::block_1010_2000::{mixed_1010_20ba, pass1_1010_209e, pass1_1010_2ee2, unk_load_str_op_1010_2c34};
use crate::unk::block_1010_3000;
use crate::unk::block_1010_5000::struct_1010_5f1e;
use crate::unk::block_1010_7000::show_window_1010_7ace;
use crate::unk::block_1010_9000::{pass1_1010_9044, pass1_1010_9130, pass1_1010_91cc, pass1_1010_9210, struct_1010_9172};
use crate::unk::block_1010_a000::{pass1_1010_a50c, pass1_1010_a5ca, pass1_1010_ac62};
use crate::unk::block_1010_c000::{pass1_1010_c320, pass1_1010_c3c2};
use crate::unk::block_1018_1000;
use crate::unk::block_1018_2000::{pass1_1018_2504, pass1_1018_2580, pass1_1018_25d2, pass1_1018_2678, pass1_1018_2862, pass1_1018_2afa, pass1_1018_2d22, pass1_1018_2d84, pass1_1018_2fe8};
use crate::unk::block_1018_3000::{pass1_1018_30ca, pass1_1018_30fc, pass1_1018_31d0, pass1_1018_36e6, pass1_1018_3710, pass1_1018_3a5c, pass1_1018_3a7a, pass1_1018_3a94, pass1_1018_3ab2, pass1_1018_3d44, sprintf_op_1018_34b6, string_1018_39d8, switch_1018_3b9e, unk_str_op_1018_35b0};
use crate::unk::block_1018_5000::{pass1_1018_50ea, pass1_1018_57d2, pass1_1018_57e6};
use crate::winapp::{enum_child_windows_1010_01be, fatal_app_exit_1000_3e9e};
use crate::unk::block_1020_1000::pass1_1020_1d8e;
use crate::unk::block_1020_5000;
use crate::unk::block_1020_b000::{pass1_1020_bae6, pass1_1020_bd80};
use crate::unk::block_1030_2000::{pass1_1030_2f1a, pass1_1030_2fac};
use crate::unk::block_1030_5000::{pass1_1030_532e, pass1_1030_5b00};
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8308, pass1_1030_8326, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e};
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::winapp::winapp_c::{send_msg_1038_c228, send_msg_1040_3374};
use crate::winapp::winapp_c::send_msg_1040_1696;
use crate::unk::block_1040_5000::{pass1_1040_5cd6, pass1_1040_5dc4, pass1_1040_5eaa};
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::unk::block_1040_9000::pass1_1040_9824;
use crate::unk::block_1040_b000::{pass1_1040_b0bc, pass1_1040_b54a, pass1_1040_bfde, struct_1040_bf3e};
use crate::{draw_ops, winapp};
use crate::draw_ops::unk_win_ui_op_1020_67ce;
use crate::draw_ops::draw_a::{draw_op_1040_9948, palette_op_1008_4e08, set_sys_color_1008_357e};
use crate::draw_ops::draw_d::load_draw_op_1020_2ede;
use crate::draw_ops::draw_e::ui_cleanup_op_1040_782c;
use crate::draw_ops::draw_f::{get_sys_metrics_1040_8c66, unk_win_ui_op_1020_717e};
use crate::file_ops::{close_file_1008_496c, close_file_1008_6dd0, read_file_1008_6e78};
use crate::globals::{PTR_LOOP_1050_1040, u32_1050_0004};
use crate::no_refs::f::pass1_1010_40cc;
use crate::no_refs::h::{pass1_1010_4f30, pass1_1010_5d9c};
use crate::no_refs::i::{pass1_1010_5f7a, pass1_1010_5fb0, pass1_1010_5fd8, pass1_1010_6006, pass1_1010_60b4, pass1_1010_60ba, pass1_1010_60c0, pass1_1010_60c6, pass1_1010_60cc, pass1_1010_6566, pass1_1010_659a, pass1_1010_6604};
use crate::no_refs::k::unk_load_str_op_1010_8c96;
use crate::no_refs::l::{pass1_1010_a568, pass1_1010_a58a, pass1_1010_a5ac, pass1_1010_a5ec, pass1_1010_af66, pass1_1010_c234, pass1_1010_c25e, pass1_1010_c3c2, string_op_1010_c446};
use crate::no_refs::m::pass1_1010_ecc6;
use crate::resources::{load_string_1010_847e, load_string_1010_84ac, load_string_1010_84e0};
use crate::sound_ops::mci_send_command_1008_53ae;
use crate::string_defs::{s__1050_4415, s_listbox_1050_4416, s_MciSound_registerClass_failed_1050_02cc, s_MciSoundWindow_1050_02bd};
use crate::structs::struct_19::Struct19;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_37::Struct37;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_72::Struct72;
use crate::structs::struct_878::Struct878;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_915::astruct_915;
use crate::structs::struct_929::Struct929;
use crate::structs::struct_a::StructA;
use crate::structs::struct_d::StructD;
use crate::sys_ops::debug_print_1008_6048;
use crate::unk::block_1000_5000::pass1_1000_5586;
use crate::unk::block_1008_4000::{pass1_1008_405c, pass1_1008_4772, pass1_1008_4d84, struct_1008_4c58, struct_op_1008_48fe};
use crate::unk::block_1008_6000::{pass1_1008_6978, str_1008_6d8a, str_op_1008_60e8};
use crate::unk::block_1008_8000::empty_1008_8fc4;
use crate::unk::block_1008_9000::{create_window_ex_1008_9760, make_def_wnd_proc_1008_9ce6, pass1_1008_932a, pass1_1008_941a};
use crate::unk::block_1008_a000::{pass1_1008_a930, pass1_1008_ab54};
use crate::unk::block_1008_b000::{load_string_1008_b1f0, load_string_1008_b65a, pass1_1008_b146, pass1_1008_b200, pass1_1008_b340, pass1_1008_b366, pass1_1008_b38c, pass1_1008_b47a, pass1_1008_b4a0, pass1_1008_b544, pass1_1008_b61a, pass1_1008_b63a, pass1_1008_b820};
use crate::unk::block_1008_c000::{pass1_1008_c6ae, pass1_1008_c83a, pass1_1008_c85e};
use crate::unk::block_1008_e000::{pass1_1008_e2a4, pass1_1008_e320, pass1_1008_eb5c, pass1_1008_eb6e};
use crate::unk::block_1010_0000::{pass1_1010_01f8, pass1_1010_038e, pass1_1010_043a, pass1_1010_0886, pass1_1010_088c, pass1_1010_0892, pass1_1010_0898, pass1_1010_089e, pass1_1010_08e2, pass1_1010_091e, pass1_1010_0932, pass1_1010_0946};
use crate::unk::block_1010_1000::{pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_1f62, struct_op_1010_1d48};
use crate::unk::block_1010_3000::{pass1_1010_32c0, pass1_1010_375e, pass1_1010_3770, win_ui_op_1010_3202};
use crate::unk::block_1010_4000::{pass1_1010_41d6, pass1_1010_451a, pass1_1010_459e};
use crate::unk::block_1010_6000::pass1_1010_60a0;
use crate::unk::block_1010_8000::{FUN_1010_830a, pass1_1010_8096};
use crate::unk::block_1010_d000::struct_1010_dd5e;
use crate::unk::block_1018_0000::{pass1_1018_017c, pass1_1018_04b8, pass1_1018_0ac0, pass1_1018_0ad4, pass1_1018_0afa, pass1_1018_0b08};
use crate::unk::block_1018_1000::{pass1_1018_161c, pass1_1018_1662, pass1_1018_179e, pass1_1018_1a8e, pass1_1018_1c9a, pass1_1018_1e78, pass1_1018_1f1a};
use crate::unk::block_1018_7000::pass1_1018_6198;
use crate::unk::block_1020_0000::{pass1_1020_022c, pass1_1020_0dc4};
use crate::unk::block_1020_2000::{pass1_1020_289a, pass1_1020_294a, pass1_1020_2a94};
use crate::unk::block_1020_6000::{pass1_1020_61c4, pass1_1020_6498, pass1_1020_64d4, pass1_1020_68de};
use crate::unk::block_1020_c000::{string_1020_c0d8, string_op_1020_c222};
use crate::unk::block_1028_4000::pass1_1028_4a9a;
use crate::unk::block_1028_8000::{pass1_1028_837e, pass1_1028_84ca};
use crate::unk::block_1028_d000::pass1_1028_dc52;
use crate::unk::block_1028_e000::pass1_1028_e4ec;
use crate::unk::block_1030_6000::{pass1_1030_6c1a, pass1_1030_6ddc, pass1_1030_6e14};
use crate::unk::block_1030_e000::pass1_1030_e63e;
use crate::unk::block_1038_3000::{pass1_1038_387e, pass1_1038_3aa6};
use crate::unk::block_1038_9000::pass1_1038_993a;
use crate::unk::block_1038_b000::pass1_1038_b6e0;
use crate::unk::block_1040_8000::{pass1_1040_8478, string_1040_8520};
use crate::unk::block_1040_a000::{pass1_1040_a5d0, struct_1040_a598};
use crate::unk::block_1040_c000::pass1_1040_c60e;
use crate::winapi16::{BringWindowToTop16, CallWindowProc16, CheckDlgButton16, CheckMenuItem16, CheckRadioButton16, ClientToScreen16, CreateDialog16, CreateWindow16, DefWindowProc16, DeleteMenu16, DestroyIcon16, DestroyMenu16, DestroyWindow16, DispatchMessage16, EnableMenuItem16, EnableWindow16, GetClassInfo16, GetClientRect16, GetCursorPos16, GetDC16, GetDlgCtrlID16, GetDlgItem16, GetDlgItemInt16, GetMenuState16, GetMessage16, GetNextDlgTabItem16, GetOpenFileName16, GetProp16, GetSaveFileName16, GetStockObject16, GetSubMenu16, GetSystemMetrics16, GetWindowLong16, GetWindowPlacement16, GetWindowRect16, GetWindowText16, GetWindowWord16, InsertMenu16, InvalidateRect16, IsDialogMessage16, IsDlgButtonChecked, IsIconic16, IsWindow16, LoadCursor16, LoadIcon16, LoadMenu16, MapDialogRect16, MessageBeep16, MessageBox16, ModifyMenu16, MoveWindow16, OutputDebugString16, PostMessage16, PtInRect16, RegisterClass16, ReleaseCapture16, ScreenToClient16, SendDlgItemMessage16, SendMessage16, SetCapture16, SetCursor16, SetDlgItemInt16, SetDlgItemText16, SetFocus16, SetProp16, SetSysModalWindow, SetWindowLong16, SetWindowPlacement16, SetWindowPos16, SetWindowText16, ShowCursor16, ShowWindow16, TrackPopupMenu16, TranslateAccelerator16, TranslateMessage16, UpdateWindow16, WinHelp16};
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::winapp::winapp_a::{create_window_1040_6eae, create_window_1040_7620, create_window_1040_8bea, unk_win_op_1010_7300, win_proc_1008_5f44};
use crate::winapp::winapp_b::{post_msg_1020_55b0, unk_win_msg_op_1008_9510};
use crate::winapp::winapp_c::send_msg_1020_097e;
use crate::windef16::{ATOM, BOOL16, HANDLE16, HCURSOR16, HDC16, HICON16, HMENU16, HWND16, LPARAM, LRESULT, MSG16, POINT16, RECT16, WINDOWPLACEMENT16, WNDCLASS16, WPARAM16};

pub unsafe fn get_sys_metrics_1018_1ea0(param_1: *mut Struct19) {
    let mut IVar1: i16;
    let mut IVar2: i16;
    let mut iVar3: *mut Struct19;
    let mut uVar3: *mut Struct19;

    IVar1 = GetSystemMetrics16(SM_CXBORDER);
    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar3.field23_0x2e = IVar1 * 0x2 + iVar3.field26_0x36;
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    IVar2 = GetSystemMetrics16(SM_CYBORDER);
    iVar3.field24_0x30 = IVar1 + iVar3.field27_0x38 + IVar2;
    return;
}


pub unsafe fn get_sys_metrics_1018_4b1e(
    param_1: *mut Struct19,
    mut param_2: u16,
    mut param_3: u16,
) -> *mut u16 {
    let mut pstruct19_1: *mut Struct19;
    let mut pstruct19_param_1_hi: *mut Struct19;

    struct_op_1010_1d48(param_1, param_3);
    pstruct19_param_1_hi = (param_1 >> 0x10);
    pstruct19_1 = param_1;
    pstruct19_1.field9_0x12 = param_2;
    pstruct19_1.field10_0x14 = 0;
    // 0x4c9e val
    param_1.offset_0x0 = &PTR_LOOP_1050_4c9e;
    pstruct19_1.segment_0x2 = 0x1018;
    if G_SM_CYCAPTION_1050_416c == 0 {
        G_SM_CYCAPTION_1050_416c = GetSystemMetrics16(SM_CYCAPTION);
        G_SM_CXBORDER_1050_416e = GetSystemMetrics16(SM_CXBORDER);
        G_SM_CYBORDER_1050_4170 = GetSystemMetrics16(SM_CYBORDER);
    }
    return &param_1.offset_0x0;
}


pub unsafe fn get_sys_metrics_1020_7c1a(param_1: *mut astruct_40, param_2: *mut StructA) {
    let mut IVar1: i16;
    let mut iVar3: *mut astruct_40;
    let mut uVar3: u16;
    let mut uVar4: *mut astruct_40;
    let mut uVar1: u16;

    uVar3 = (param_2 >> 0x10);
    uVar1 = (param_2 + 0x8);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar3.field1_0x2 = 0x1008;
    param_1.field0_0x0 = 0x3aa8;
    iVar3.field1_0x2 = 0x1008;
    iVar3.hwnd_0x4 = uVar1;
    param_1.field0_0x0 = 0x3ab0;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field_0x6 = param_2;
    iVar3.field_0xa = 0;
    iVar3.field_0xe = 0;
    iVar3.field_0x10 = 0;
    iVar3.field_0x12 = 0;
    param_1.field0_0x0 = 0x7f72;
    iVar3.field1_0x2 = 0x1020;
    iVar3.field_0xa = (param_2 + 0xe4);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    iVar3.field_0xe = IVar1;
    IVar1 = GetSystemMetrics16(SM_CXBORDER);
    iVar3.field_0x10 = IVar1;
    IVar1 = GetSystemMetrics16(SM_CYBORDER);
    iVar3.field_0x12 = IVar1;
    return;
}


pub unsafe fn get_sys_metrics_1040_7728(param_1: *mut Struct57, mut param_2: u16, mut param_3: u32, mut param_4: u16, mut param_5: u16 )

{
  let mut IVar1: i16;
  let mut iVar2: *mut Struct57;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar2.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x3aa8;
  iVar2.field1_0x2 = 0x1008;
  iVar2.field2_0x4 = 0;
  iVar2.field3_0x6 = 0;
  iVar2.field4_0x8 = param_5;
  iVar2.field5_0xa = param_4;
  iVar2.field6_0xc = 0;
  iVar2.field78_0x60 = 0;
  iVar2.field79_0x62 = 0;
  iVar2.field80_0x64 = 0;
  iVar2.field81_0x66 = 0;
  iVar2.field82_0x68 = 0;
  iVar2.field83_0x6a = param_3;
  iVar2.field84_0x6e = param_2;
  iVar2.field85_0x70 = 0;
  iVar2.field86_0x74 = 0;
  iVar2.field87_0x76 = 0;
  iVar2.field88_0x78 = 0;
  iVar2.field105_0x8a = 0;
  iVar2.field106_0x8c = 0;
  param_1.field0_0x0 = 0x840c;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&iVar2.field8_0x10)),0x10505db0);
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x7a)),NULL,0x8);
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x82)),NULL,0x8);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  iVar2.field79_0x62 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  iVar2.field80_0x64 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CYBORDER);
  iVar2.field81_0x66 = IVar1;
  return;
}

pub unsafe fn get_sys_metrics_1010_46f6(mut param_1: u32, param_2: *mut Struct57) {
    let mut IVar1: i16;
    let mut IVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut uVar6: u32;
    let mut in_stack_0000fe7c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffaa: u16;
    let mut piVar7: *mut i16;
    let mut uVar8: u16;
    let mut pcVar9: *mut c_char;
    let mut local_6: i16;
    let mut local_4: i16;

    pcVar9 = CONCAT22(0x1050, &local_4);
    piVar7 = &local_6;
    uVar8 = SUB42(0x1050, 0x0);
    puVar5 = mixed_1010_20ba(
        param_2,
        _u16_1050_0ed0,
        CONCAT22(piVar7, 0x48),
        in_stack_0000fe7c,
        in_stack_0000ffa0,
        in_stack_0000ffa6,
        in_stack_0000ffaa,
    );
    pass1_1008_3e94(
        (puVar5 & 0xffff0000 | (puVar5 + 0xe)),
        CONCAT22(uVar8, piVar7),
        pcVar9,
    );
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar6 = pass1_1008_4772((iVar3 + 0x66));
    uVar8 = (uVar6 >> 0x10);
    (iVar3 + 0x18) = local_4 + 0x8;
    (iVar3 + 0x1a) = local_6 + 0x9;
    IVar1 = GetSystemMetrics16(SM_CXBORDER);
    (iVar3 + 0x1c) = IVar1 * 0x2 + (uVar6 + 0x4);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    IVar2 = GetSystemMetrics16(SM_CYBORDER);
    (iVar3 + 0x1e) = IVar2 + IVar1 + (uVar6 + 0x8);
    return;
}

pub unsafe fn get_sys_metrics_1018_09a8(mut param_1: u16, mut param_2: u32) {
    let mut uVar1: u32;
    let mut IVar2: i16;
    let mut IVar3: i16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe7c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffaa: u16;
    let mut piVar8: *mut i16;
    let mut uVar9: u16;
    let mut pcVar10: *mut c_char;
    let mut local_a: i16;
    let mut local_8: i16;
    let mut iStack6: i16;
    let mut IStack4: i16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    IStack4 = GetSystemMetrics16(SM_CYCAPTION);
    uVar6 = (param_2 >> 0x10);
    iVar5 = param_2;
    iStack6 = (iVar5 + 0x12) - 0x2;
    pcVar10 = CONCAT22(0x1050, &local_8);
    piVar8 = &local_a;
    uVar9 = SUB42(0x1050, 0x0);
    puVar7 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        CONCAT22(piVar8, 0x48),
        in_stack_0000fe7c,
        in_stack_0000ffa0,
        in_stack_0000ffa6,
        in_stack_0000ffaa,
    );
    pass1_1008_3e94(
        (puVar7 & 0xffff0000 | (puVar7 + 0xe)),
        CONCAT22(uVar9, piVar8),
        pcVar10,
    );
    (iVar5 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
    (iVar5 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
    IVar2 = GetSystemMetrics16(SM_CXBORDER);
    uVar1 = (iVar5 + 0x5a);
    (iVar5 + 0x1c) = IVar2 * 0x2 + (uVar1 + 0x4);
    IVar2 = GetSystemMetrics16(SM_CYCAPTION);
    IVar3 = GetSystemMetrics16(SM_CYBORDER);
    uVar1 = (iVar5 + 0x5a);
    (iVar5 + 0x1e) = IVar3 + IVar2 + (uVar1 + 0x8);
    return;
}

pub unsafe fn get_sys_metrics_1018_2f56(mut param_1: u32) {
    let mut IVar1: i16;
    let mut IVar2: i16;
    let mut in_EDX: *mut Struct57;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut uVar6: u32;
    let mut in_stack_0000fe7c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffaa: u16;
    let mut piVar7: *mut i16;
    let mut uVar8: u16;
    let mut pcVar9: *mut c_char;
    let mut local_6: i16;
    let mut local_4: i16;

    pcVar9 = CONCAT22(0x1050, &local_4);
    piVar7 = &local_6;
    uVar8 = SUB42(0x1050, 0x0);
    puVar5 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(piVar7, 0x48),
        in_stack_0000fe7c,
        in_stack_0000ffa0,
        in_stack_0000ffa6,
        in_stack_0000ffaa,
    );
    pass1_1008_3e94(
        (puVar5 & 0xffff0000 | (puVar5 + 0xe)),
        CONCAT22(uVar8, piVar7),
        pcVar9,
    );
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar6 = pass1_1008_4772((iVar3 + 0x24));
    uVar8 = (uVar6 >> 0x10);
    (iVar3 + 0x18) = local_4 + 0xb5;
    (iVar3 + 0x1a) = local_6 + 0x9;
    IVar1 = GetSystemMetrics16(SM_CXBORDER);
    (iVar3 + 0x1c) = IVar1 * 0x2 + (uVar6 + 0x4);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    IVar2 = GetSystemMetrics16(SM_CYBORDER);
    (iVar3 + 0x1e) = IVar2 + IVar1 + (uVar6 + 0x8);
    return;
}
