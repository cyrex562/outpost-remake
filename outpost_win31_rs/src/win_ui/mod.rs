use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::block_1000::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::block_1000::block_1000_3000::{pass1_1000_3cea, str_op_1000_3da4, unk_str_op_1000_3d3e};
use crate::block_1000::block_1000_4000::str_1000_4d58;
use crate::block_1008::block_1008_3000::pass1_1008_3bd6;
use crate::block_1008::block_1008_5000::{pass1_1008_5fd8, win_1008_5c5c, win_1008_5c7c};
use crate::block_1008::block_1008_8000;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1010::block_1010_3000;
use crate::block_1010::block_1010_5000::struct_1010_5f1e;
use crate::block_1010::block_1010_7000::show_window_1010_7ace;
use crate::block_1010::block_1010_9000::{pass1_1010_9044, pass1_1010_9130, pass1_1010_91cc, pass1_1010_9210, struct_1010_9172};
use crate::block_1010::block_1010_a000::pass1_1010_a5ca;
use crate::block_1010::block_1010_c000::pass1_1010_c3c2;
use crate::block_1018::block_1018_1000;
use crate::block_1018::block_1018_2000::{pass1_1018_2afa, pass1_1018_2d84};
use crate::block_1018::block_1018_3000::pass1_1018_30fc;
use crate::block_1018::block_1018_5000::{pass1_1018_50ea, pass1_1018_57d2};
use crate::winapp::send_msg_1020_097e;
use crate::block_1020::block_1020_1000::pass1_1020_1d8e;
use crate::block_1020::block_1020_5000;
use crate::block_1020::block_1020_b000::pass1_1020_bd80;
use crate::block_1030::block_1030_2000::{pass1_1030_2f1a, pass1_1030_2fac};
use crate::block_1030::block_1030_5000::pass1_1030_532e;
use crate::block_1030::block_1030_7000::struct_op_1030_73a8;
use crate::block_1030::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e};
use crate::block_1038::block_1038_a000::pass1_1038_af40;
use crate::block_1038::{block_1038_8000, block_1038_9000, block_1038_c000};
use crate::winapp::send_msg_1038_c228;
use crate::winapp::send_msg_1040_1696;
use crate::block_1040::{block_1040_1000, block_1040_3000};
use crate::block_1040::block_1040_5000::{pass1_1040_5cd6, pass1_1040_5dc4, pass1_1040_5eaa};
use crate::winapp::{create_window_1040_6eae, create_window_1040_7620, win_proc_1008_5f44};
use crate::winapp::post_win_msg_1040_7b3c;
use crate::block_1040::block_1040_9000::pass1_1040_9824;
use crate::block_1040::block_1040_b000::pass1_1040_b54a;
use crate::draw_ops;
use crate::draw_ops::{draw_op_1040_9948, load_draw_op_1020_2ede};
use crate::globals::u32_1050_0004;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::string_defs::{s__1050_4415, s_listbox_1050_4416, s_MciSound_registerClass_failed_1050_02cc, s_MciSoundWindow_1050_02bd};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_72::Struct72;
use crate::structs::struct_878::Struct878;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_915::astruct_915;
use crate::structs::struct_929::Struct929;
use crate::structs::struct_a::StructA;
use crate::structs::struct_d::StructD;
use crate::winapi16::{CreateWindow16, DestroyWindow16, EnableWindow16, GetClassInfo16, GetClientRect16, GetDlgItem16, GetDlgItemInt16, GetStockObject16, GetWindowRect16, LoadCursor16, MapDialogRect16, MessageBox16, OutputDebugString16, PostMessage16, RegisterClass16, SendDlgItemMessage16, SendMessage16, SetCursor16, SetDlgItemInt16, SetFocus16, SetWindowPos16, ShowWindow16, UpdateWindow16};
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::windef16::{ATOM, HCURSOR16, HMENU16, HWND16, LPARAM, LRESULT, RECT16, WNDCLASS16, WPARAM16};

pub unsafe fn create_window_1008_5e7e() ->HWND16
{
  let mut var2: *mut c_char;
  let mut var3: bool;
  // AVar4: ATOM;
    let mut var4: ATOM;
  let mut window_handle_1: HWND16;
  let mut var5: i16;
  let mut string_1: *mut c_char;
  let mut var6: *mut c_char;
  let mut wndclass_44: WNDCLASS16;
  // let mut local_12: [u32;0x4] = [0;0x4];
  let mut var12: [c_char;16] = [0;16];
    let mut var1: *mut c_char;

  var6 = var12.as_mut_ptr();
  string_1 = s_MciSoundWindow_1050_02bd.into_raw();
    var5 = 0x3;
  // for (iVar5 = 0x3; iVar5 != 0; iVar5 += -1) {
    while var5 != 0 {
        var2 = var6;
        var6 = var6 + 1;
        var1 = string_1;
        string_1 = (string_1 + 0x4);
        var2 = var1;
        var5 -= 1;
    }
  var6 = string_1;
  (var6 + 0x2) = (string_1 + 2);
  wndclass_44.style = 0x2000;
  // wndclass_44.lpfn_wnd_proc = SUB42(&DAT_1050_5f44,0x0);
  // wndclass_44.lpfn_wnd_proc = 0x1008;
    wndclass_44.lpfn_wnd_proc = win_proc_1008_5f44;
  wndclass_44.cb_wnd_extra = 0x2;
  wndclass_44.h_instance = HINSTANCE16_1050_038c;
  wndclass_44.h_icon = 0;
  wndclass_44.h_cursor = 0;
  wndclass_44.cb_cls_extra = 0;
  wndclass_44.hbr_background = GetStockObject16(WHITE_BRUSH);
  wndclass_44.lpsz_menu_name = null_mut();
  wndclass_44.lpsz_class_name = local_12.as_mut_ptr();
  // wndclass_44.lpsz_class_name = SUB42(&DAT_1050_1050,0x0);
    let mut wc_ptr: *mut WNDCLASS16 = &mut wndclass_44;
    let class_name: *const c_char = local_12.as_ptr();
  var3 = GetClassInfo16(wc_ptr, class_name, 0);
  if var3 == false {
    var4 = RegisterClass16(wc_ptr);
    if var4 == 0x0 {
      OutputDebugString16(s_MciSound_registerClass_failed_1050_02cc.as_ptr());
      return 0x0;
    }
  }
  window_handle_1 =
       CreateWindow16(0x0,HINSTANCE16_1050_038c,HWND16_1050_0396,0x1,0x1,-0x8000,-0x8000,0x0,0xcf,
                      s_MciSound_registerClass_failed_1050_02cc + 0x1e,local_12.as_ptr());
  return window_handle_1;
}


pub unsafe fn win_ui_fn_1020_6e98(mut param_1: *mut Struct57 ,param_2: *mut StructA)

{
  let mut u16_var3: u16;
  let mut u16_var5: u16;
  let mut string_var9: *mut c_char;
  let mut wparam_var10: WPARAM16;
  let mut u16_var11: u16;
  let mut hwnd_var12: HWND16;
  let mut rectangle: RECT16;
  let mut hwnd_var13: HWND16 = 0;
  let mut i16_var4: i16 = 0i16;
    let mut var9: *mut Struct878;

  // uVar5 = (param_2 >> 0x10);
  // let mut pstructa_var6 = param_2;
    let client_rect: *mut RECT16 = &mut rectangle;
  GetClientRect16(client_rect, 0);
  let mut win_style = 0;
  let mut window_handle = GetDlgItem16(0x1797, param_2.win_handle_0x8);
  if window_handle != 0x0 {
    DestroyWindow16(window_handle);
  }
  pass1_1018_30fc(param_1, param_2[0x1].field20_0x26, &mut win_style);
  if (win_style) != 0x0 {
      // CONCAT22(0x1797,HINSTANCE16_1050_038c) --> 0x1797,0x038c
      let mut hinst = HINSTANCE16_1050_038c;
      let mut data: *mut c_void = &param_2.win_handle_0x8 as *mut c_void;
      let mut cw_hmenu: HMENU16 = (i16_var4 - 0x19) as HMENU16;
    window_handle = CreateWindow16(win_style ,data, hinst, cw_hmenu,
                                   hwnd_var13, 0x0, 0x0, 0x103, 0x40a0, s__1050_4415.as_ptr(), s_listbox_1050_4416.as_ptr());
    let u32_var2 = win_style;
    if window_handle == 0x0 {
      if (win_style) != 0x0 {
        pass1_1018_2afa(win_style);
        fn_ptr_1000_17ce(u32_var2);
        return;
      }
    }
    else {
      let mut lresult_var8 = SendMessage16(0x0, 0x0, 0xb, window_handle);
      u16_var5 = (lresult_var8 >> 0x10);
      if (win_style + 0x4) == 0x0 {
        wparam_var10 = 0;
        u16_var11 = 0x401;
        hwnd_var12 = window_handle;
        string_var9 = load_string_1010_847e(_u16_1050_14cc, 0x531);
        SendMessage16(string_var9 as LPARAM, wparam_var10, u16_var11, hwnd_var12);
      }
      else {
        var9 = null_mut();
        loop {
          ppaVar1 = (win_style + 0x4);
          if *ppaVar1 == var9 || *ppaVar1 < var9 { break; };
          wparam_var10 = 0;
          u16_var11 = 0x401;
          hwnd_var12 = window_handle;
          u16_var3 = pass1_1020_bd80((win_style + var9 * 0x2));
          lresult_var8 = SendMessage16(u16_var3 as LPARAM, wparam_var10, u16_var11, hwnd_var12);
          // u16_var5 = (lresult_var8 >> 0x10);
          var9 = (u16_var11 + 1);
        }
      }
      lresult_var8 = SendMessage16(0x0, 0x1, 0xb, window_handle);
      // u16_var5 = (lresult_var8 >> 0x10);
      u16_var3 = lresult_var8;
      wparam_var10 = 0xffff;
      u16_var11 = 0x40d;
      hwnd_var12 = window_handle;
      pass1_1018_2d84(u16_var3, &mut param_2[0x1].field20_0x26);
      lresult_var8 = SendMessage16(u16_var3, wparam_var10, u16_var11, hwnd_var12);
      wparam_var10 = lresult_var8;
      if (wparam_var10 != 0xffff) || ((lresult_var8 >> 0x10) != -1) {
        SendMessage16(0x0, wparam_var10, 0x407, window_handle);
        SendMessage16(0x0, wparam_var10, 0x418, window_handle);
      }
      if win_style != 0 {
        string_var9 = win_style;
        pass1_1018_2afa(win_style);
        fn_ptr_1000_17ce(string_var9);
      }
      ShowWindow16(0x1,window_handle);
      SetFocus16(window_handle);
    }
  }
  return;
}

pub unsafe fn disable_dialog_items_1040_d6be(param_1: *mut Struct903)

{
    let mut handle: HWND16;
    handle = GetDlgItem16(0x1, param_1.field6_0x6);
    EnableWindow16(0x0, handle);
    handle = GetDlgItem16(0x2, param_1.field6_0x6);
    EnableWindow16(0x0, handle);
    handle = GetDlgItem16(0x1841, param_1.field6_0x6);
    EnableWindow16(0x0, handle);
    handle = GetDlgItem16(0x1848, param_1.field6_0x6);
    EnableWindow16(0x0, handle);
    handle = GetDlgItem16(0x1849, param_1.field6_0x6);
    EnableWindow16(0x0, handle);
    handle = GetDlgItem16(0x184a, param_1.field6_0x6);
    EnableWindow16(0x0, handle);
    handle = GetDlgItem16(0x184b, param_1.field6_0x6);
    EnableWindow16(0x0, handle);
    param_1.field_0x8 = 0x1;
    return;
}



pub unsafe fn send_dlg_item_msg_1040_d20c(mut param_1: u16, mut param_2: u16, param_3: *mut Struct929, param_4: i32)

{
    let mut var1: *mut u8;
    // let mut var2: u16;
     let mut var12: u16;
    let mut var3: *mut Struct57;
    let mut var6: *mut u32;
    let mut var13: u16;
    let mut var14: u16;
    let mut var15: u16;
    let mut var16: u16;
    let mut var7: *mut u8;
    let mut var9: [u8; 0x100] = [0; 0x100];
    let mut var10: u16;
    let mut var11: u16;

    var3 = CONCAT22(var12, param_2);
    if param_4 == 0 {
        enable_win_1040_d60e(param_3);
        return;
    }
    // var5 = (param_3 >> 0x10);
    // var4 = param_3;
    if (param_3 + 0xa0) != 0 {
        pass1_1010_9210((param_3 + 0x9c));
        enable_win_1040_d60e(param_3);
        pass1_1030_8344(_u16_1050_5748, param_4);
        // var2 = SUB42(param_3, 0x0);
        var7 = var9;
        // var8 = SUB42(&DAT_1050_1050, 0x0);
        var10 = param_1;
        var11 = var2;
        var6 = mixed_1010_20ba(var3, _u16_1050_0ed0, CONCAT22(var7, 0x3), var13,
                               var14, var15, var16);
        var1 = (var6 >> 0x10);
        pass1_1010_c3c2(var1, var6, var1, &mut var9, CONCAT22(var2, param_1));
        SendDlgItemMessage16(&mut var9 as LPARAM, 0x0, 0x401, 0x1847, (param_3 + 0x6));
    }
    return;
}



pub unsafe fn win_ui_op_1040_d2ac(mut param_1: u16, pstruct_param_2: *mut Struct903, mut param_3: u16, mut param_4: u32)

{
    let mut in_register_0000000a: u16;
    let mut LVar1: LRESULT;

    if (param_4 == s_dibtext_bmp_1050_1844 + 0x4) {
        LVar1 = SendDlgItemMessage16(0x0, 0x0, 0x405, s_dibtext_bmp_1050_1844 + 0x3, (pstruct_param_2 + 0x6));
        struct_1010_9172((pstruct_param_2 + 0x9c),
                         CONCAT22(in_register_0000000a, (LVar1 >> 0x10)));
    } else if (s_dibtext_bmp_1050_1844 + 0x4 < param_4) {
        if (param_4 == s_dibtext_bmp_1050_1844 + 0x5) {
            LVar1 = SendDlgItemMessage16(0x0, 0x0, 0x40c, s_dibtext_bmp_1050_1844 + 0x3, (pstruct_param_2 + 0x6));
            if ((LVar1 != -1) || ((LVar1 >> 0x10) != -1)) {
                SendDlgItemMessage16(0x0, LVar1 - 0x1, 0x403, s_dibtext_bmp_1050_1844 + 0x3,
                                     (pstruct_param_2 + 0x6));
                pass1_1010_91cc((pstruct_param_2 + 0x9c));
            }
        } else if (param_4 == s_dibtext_bmp_1050_1844 + 0x6) {
            disable_dialog_items_1040_d6be(pstruct_param_2);
            pass1_1018_57d2((pstruct_param_2 + 0x94), pstruct_param_2);
            PostMessage16(0x0, 0x203, 0x111, HWND16_1050_0396);
        } else {
//      if (param_4 != s_dibtext_bmp_1050_1844 + 0x7U) goto LAB_1040_d3b3;
            _u16_1050_5a68 = (pstruct_param_2 + 0x98);
            pass1_1038_af40(pstruct_param_2, param_1, _PTR_LOOP_1050_5b7c, (pstruct_param_2 + 0x6),
                            0x27);
        }
    } else if (param_4 == 0xeb) {
        send_dlg_item_msg_1040_d79c(param_1, pstruct_param_2);
    } else {
        if (param_4 != s_vrpal_bmp_1050_183a + 0x7) {//
// LAB_1040_d3b3:
            pass1_1040_b54a(param_1, pstruct_param_2, param_3, param_4);
            return;
        }
        msg_box_op_1040_d3d0(0x0, param_1, pstruct_param_2);
    }
    return;
}



pub unsafe fn msg_box_op_1040_d3d0(param_1: *mut c_char, mut param_2: u16, mut param_3: u32)

{
    let mut in_buf_len_5: i16;
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut uVar2: u16;
    let mut local_206: [u8; 0x102] = [0; 0x102];
    let mut local_104: [u8; 0x102] = [0; 0x102];

    paVar1 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x1000, paVar1);
    in_buf_len_5 = paVar1;
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x100, local_206, &DAT_1050_1050);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, param_1, in_buf_len_5);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    uVar2 = (param_3 >> 0x10);
    MessageBox16(0x0, CONCAT22(0x1050, local_206), CONCAT22(in_buf_len_5, param_1),
                 (param_3 + 0x6));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, param_1, in_buf_len_5);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, &DAT_1050_1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    MessageBox16(0x0, CONCAT22(0x1050, local_206), CONCAT22(in_buf_len_5, param_1),
                 (param_3 + 0x6));
    fn_ptr_1000_17ce(CONCAT22(in_buf_len_5, param_1));
    return;
}

pub unsafe fn enable_win_1040_d60e(param_1: *mut Struct929)

{
    let mut HVar1: HWND16;
    let mut iVar2: *mut Struct929;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    HVar1 = GetDlgItem16(0x1, iVar2.field6_0x6);
    EnableWindow16(0x1, HVar1);
    HVar1 = GetDlgItem16(0x2, iVar2.field6_0x6);
    EnableWindow16(0x1, HVar1);
    HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x7, iVar2.field6_0x6);
    EnableWindow16(0x1, HVar1);
    HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x4, iVar2.field6_0x6);
    EnableWindow16(0x1, HVar1);
    HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x5, iVar2.field6_0x6);
    EnableWindow16(0x1, HVar1);
    HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x6, iVar2.field6_0x6);
    EnableWindow16(0x1, HVar1);
    HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x7, iVar2.field6_0x6);
    EnableWindow16(0x1, HVar1);
    iVar2.field159_0xa0 = 0;
    return;
}



pub unsafe fn send_dlg_item_msg_1040_d79c(mut param_1: u16, param_2: *mut Struct903)

{
    let mut lVar1: i32;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let mut in_stack_0000fd9a: u16;
    let mut in_stack_0000febe: u16;
    let mut in_stack_0000fec4: u16;
    let mut in_stack_0000fec8: u16;
    let mut in_stack_0000fef2: u16;
    let mut uStack268: u16;
    let mut uStack266: u32;
    let mut local_106: [u8; 0x100] = [0; 0x100];
    let mut puStack6: *mut u32;

    puStack6 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0,
                               CONCAT22(in_stack_0000fef2, 0x3), in_stack_0000fd9a, in_stack_0000febe,
                               in_stack_0000fec4, in_stack_0000fec8);
    puVar2 = (puStack6 >> 0x10);
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    pass1_1010_c3c2(puVar2, puStack6, puVar2, CONCAT22(0x1050, local_106), (iVar4 + 0x98));
    SendDlgItemMessage16(CONCAT22(0x1050, local_106), 0x0, 0xc, s_dibtext_bmp_1050_1844 + 0x2, (iVar4 + 0x6));
    SendDlgItemMessage16(0x0, 0x0, 0xb, s_dibtext_bmp_1050_1844 + 0x3, (iVar4 + 0x6));
    uVar6 = SendDlgItemMessage16(0x0, 0x0, 0x405, s_dibtext_bmp_1050_1844 + 0x3, (iVar4 + 0x6));
    uVar7 = uVar6 >> 0x10;
    uVar3 = uVar6;
    pass1_1010_9044((iVar4 + 0x9c));
    uStack266 = CONCAT22(uVar7, uVar3);
    uVar3 = 0;
    uStack268 = 0;
    while (CONCAT22(uStack268, uVar3) < uStack266) {
        pass1_1010_9130(local_106, uVar7, (iVar4 + 0x9c), CONCAT22(0x1050, local_106));
        if (local_106[0] != '\0') {
            uVar7 = SendDlgItemMessage16(CONCAT22(0x1050, local_106), 0x0, 0x401, s_dibtext_bmp_1050_1844 + 0x3,
                                         (iVar4 + 0x6));
            uVar7 >>= 0x10;
        }
        lVar1 = CONCAT22(uStack268, uVar3) + 1;
        uVar3 = lVar1;
        uStack268 = (lVar1 >> 0x10);
    }
    SendDlgItemMessage16(0x0, 0x1, 0xb, s_dibtext_bmp_1050_1844 + 0x3, (iVar4 + 0x6));
    return;
}

pub unsafe fn show_win_1038_b634(mut param_1: *mut c_void)

{
  let mut var1: *mut c_void;

  // uVar3 = (param_1 >> 0x10);
  // iVar2 = param_1;
  if (param_1 + 0xac) == 0 {
    (param_1 + 0xac) = 0x1;
    // for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 1)
    for i in 0x1 .. 0x2b
    {
      if (((i * 0x4 + param_1 + 0x2) | (i * 0x4 + param_1)) != 0) {
        var1 = (i * 0x4 + param_1);
        ShowWindow16(0x0,(var1 + 0x6) as HWND16);
      }
    }
  }
  return;
}

pub unsafe fn show_win_1038_b68a(param_1: *mut c_void)

{
  let mut sub_struct: *mut c_void;

  if (param_1 + 0xac) != 0 {
    (param_1 + 0xac) = 0;
    for i in 0x1 .. 0x2b
    {
      if ((i * 0x4 + param_1 + 0x2) | (i * 0x4 + param_1)) != 0 {
        sub_struct = (i * 0x4 + param_1);
        ShowWindow16(0x1,(sub_struct + 0x6) as HWND16);
      }
    }
  }
  return;
}

pub unsafe fn win_ui_op_1008_2b54(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> i16

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut uVar7: u16;
  let mut pcVar8: *mut c_char;
//   local_a6: *mut u32 [0x14];
let mut local_a6: [*mut u32;0x14] = [null_mut();0x14];
let mut local_56: [u8;0x50] = [0;0x50];
  let mut iStack6: i16;
  let mut iStack4: i16;
  let mut uVar6: u32;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0;
  if (_PTR_LOOP_1050_4230 == 0) {
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x5f2);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_56),pcVar8);
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_a6),pcVar8);
    iStack4 = MessageBox16(0x21,CONCAT13(0x10,CONCAT12(0x50,local_a6)),CONCAT22(0x1050,local_56),
                           HWND16_1050_0396);
  }
  else {
    uVar7 = 0x1000;
    mem_op_1000_179c(0xb4,paVar5);
    uVar3 = paVar5 | param_1;
    uVar6 = paVar5 & 0xffff0000 | uVar3;
    if (uVar3 == 0) {
      iVar2 = 0;
      uVar4 = 0;
    }
    else {
      uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520(uVar6,CONCAT22(paVar5,param_1),HWND16_1050_0396,0x20021,0x5f2057b);
      uVar4 = uVar6;
    }
    local_a6[0] = CONCAT22(uVar4,iVar2);
    ppcVar1 = (*local_a6[0] + 0x74);
    iStack4 = (**ppcVar1)(uVar7,iVar2,uVar4,param_1);
  }
  iStack6 = iStack4;
  if (iStack4 != 1) {
    iStack6 = 0;
  }
  if (((iStack6 != 0) && (_u16_1050_5748 != 0)) &&
     (uVar3 = (_u16_1050_5748 + 0x8),
     local_a6[0] = (local_a6[0] & 0xffff0000 | uVar3), uVar3 != 0)) {
    PostMessage16(0x0,0xb4,0x111,(param_3 + 0x8));
    iStack6 = 0;
  }
  return iStack6;
}


// WARNING: Unable to use type for symbol uVar2

pub unsafe fn ui_op_1008_2c4e(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut Struct72,mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut hcursor_5: HCURSOR16;
  let mut uVar3: u16;
  let mut paVar2: *mut astruct_20;
  let mut uVar5: *mut astruct_20;
  let mut in_stack_0000fff8: u16;
  let mut uVar2: u32;
  let mut ppaVar1: *mut astruct_20;
  let mut fn_ptr_1: *mut *mut code;

  hcursor_5 = LoadCursor16(0x7f02,0x0);
  hcursor_5 = SetCursor16(hcursor_5);
  uVar5 = CONCAT22(param_1,hcursor_5);
  ppaVar1 = &(param_3)[0x1].field7_0x10;
  ppaVar1.offset_0x0 = ppaVar1.offset_0x0 + 1;
  paVar2 = param_3;
  if (&(param_3)[0x1].field5_0xc != 0) {
    uVar2 = &(param_3)[0x1].field5_0xc;
    paVar2 = *&(param_3)[0x1].field5_0xc;
    fn_ptr_1 = &paVar2.field_0x90;
    uVar5 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10));
  }
  big_switch_1008_15d4(paVar2,param_3,CONCAT22(param_2,param_4));
  uVar3 = (uVar5 >> 0x10);
  (param_3)[0x1].field5_0xc = uVar5;
  (param_3)[0x1].field6_0xe = uVar3;
  fn_ptr_1 = (*&(param_3)[0x1].field5_0xc + 0x8);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,(param_3)[0x1].field5_0xc,uVar3);
  if ((&(param_3)[0x1].field2_0x4 + 0x2) != 0) {
    uVar1 = (&(param_3)[0x1].field2_0x4 + 2);
    fn_ptr_1 = ((&(param_3)[0x1].field2_0x4 + 0x2) + 0xc)
    ;
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10),0x0);
  }
  show_win_1038_b634(_PTR_LOOP_1050_5b7c);
  show_win_1010_7a76((&(param_3)[0x1].field7_0x10 + 0x2));
  uVar1 = &(param_3)[0x1].field5_0xc;
  fn_ptr_1 = (*&(param_3)[0x1].field5_0xc + 0xc);
  (**fn_ptr_1)(0x1010,uVar1,(uVar1 >> 0x10),0x5);
  uVar1 = &(param_3)[0x1].field5_0xc;
  BringWindowToTop16((uVar1 + 0x8));
  SetCursor16(hcursor_5);
  return;
}



pub unsafe fn post_msg_1008_2d22(param_1: *mut Struct72)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u32;
  let mut func_ptr: *mut *mut code;
  // let mut iVar4: *mut Struct72;
  let mut puVar5: *mut u32;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;

  // iVar4 = param_1;
    piVar1 = &mut param_1.field231_0xf2;
    *piVar1 = *piVar1 -0x1;
  if param_1.field230_0xee.is_null() == false && param_1.field231_0xf2 < 1 {
    puVar7 = param_1.field230_0xee;
    func_ptr = (*param_1.field230_0xee + 0x90);
    (**func_ptr)();
    param_1.field230_0xee = null_mut();
    param_1.field231_0xf2 = 0;
    if param_1.field227_0xe8.is_null() == false {
      uVar6 = 0x3;
      puVar5 = param_1.field227_0xe8;
      func_ptr = *param_1.field227_0xe8 + 0xc;
      (**func_ptr)();
      show_win_1038_b68a(_PTR_LOOP_1050_5b7c);
      show_window_1010_7ace(iVar4.field232_0xf4);
      puVar2 = param_1.field227_0xe8;
      func_ptr = *param_1.field227_0xe8 + 0x98;
      (**func_ptr)(0x1010, puVar2, (puVar2 >> 0x10), 0x1, puVar5, uVar6, puVar7);
      PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    }
  }
  return;
}

pub unsafe fn cursor_op_1008_2dcc(mut param_1: u16, param_2: *mut astruct_20, mut param_3: u16, mut param_4: u16, mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut cursor_handle: HCURSOR16;
  let mut hcursor: HCURSOR16;
  let mut extraout_DX: u16;
  let mut paVar3: *mut astruct_20;

  cursor_handle = LoadCursor16(0x7f02,0x0);
  hcursor = SetCursor16(cursor_handle);
  paVar3 = param_2;
  cursor_handle = hcursor;
  if ((&(param_2)[0x1].field2_0x4 + 0x2) != 0) {
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    paVar3 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = &paVar3.field_0x90;
    (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10));
    param_1 = extraout_DX;
  }
  big_switch_1008_15d4(paVar3,param_2,CONCAT22(param_5,param_3));
  (&(param_2)[0x1].field2_0x4 + 0x2) = cursor_handle;
  (param_2)[0x1].field3_0x8 = param_1;
  uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
  if ((uVar1 + 0xe0) == 0) {
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = ((&(param_2)[0x1].field2_0x4 + 0x2) + 0x8);
    (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10));
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = ((&(param_2)[0x1].field2_0x4 + 0x2) + 0xc);
    (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10),0x3);
    (param_2).field153_0xce = (&(param_2)[0x1].field2_0x4 + 2);
  }
  else {
    (&(param_2)[0x1].field2_0x4 + 0x2) = 0;
    ui_op_1008_2c4e(param_1,param_4,param_2,param_3);
    (param_2).field153_0xce = 0;
  }
  SetCursor16(hcursor);
  return;
}




pub unsafe fn win_ui_cursor_op_1008_2e9a
               (param_1: *mut Struct57,param_2: *mut Struct72,param_3: *mut WNDCLASS16 ,mut param_4: u16 ,mut param_5: u16 ,
               mut param_6: u16 )

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut HVar6: HCURSOR16;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut unaff_SI: u16;
  let mut uVar7: *mut Struct72;
  let mut in_stack_0000fc78: u16;
  let mut in_stack_0000fd9c: u16;
  let mut in_stack_0000fda2: u16;
  let mut in_stack_0000fda6: u16;
  let mut local_224: [u8;0x108] = [0;0x108];
  let mut uStack266: u16;
  let mut uStack262: u32;
  let mut local_102: [u8;0x100] = [0;0x100];

  local_102[0] = '\0';
  uStack262 =
              mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2),in_stack_0000fc78,
                              in_stack_0000fd9c,in_stack_0000fda2,in_stack_0000fda6);
  uVar3 = (uStack262 >> 0x10);
  iVar4 = uStack262;
  pcVar1 = *(iVar4 + 0x16);
  uVar5 = (iVar4 + 0x18);
  uVar9 = param_1 & 0xffff0000 | uVar5;
  uStack266 = pcVar1;
  uStack266 = uVar5 | uStack266;
  if uStack266 == 0 {
    save_file_1008_3178(uVar5,param_2,1);
    uVar5 = uVar9;
    uVar8 = uVar5 | uStack266;
    uVar9 = uVar9 & 0xffff0000 | uVar8;
    if uVar8 == 0 {
      PostMessage16(0x0,0x13d,0x111,HWND16_1050_0396);
      return;
    }
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),CONCAT22(uVar5,uStack266));
    str_1000_4d58(CONCAT22(0x1050,local_102),NULL,0x0,CONCAT22(0x1050,local_224),
                  CONCAT22(0x1050,&param_3));
    if (param_3 != '\0') {
        let cea = pass1_1000_3cea(CONCAT22(0x1050, local_224), CONCAT22(0x1050, &param_3));
    }
    struct_1010_5f1e(uVar9,uStack262,CONCAT22(0x1050,local_224));
  }
  else {
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),*(iVar4 + 0x1a));
    uVar5 = str_op_1000_3da4(CONCAT22(0x1050,local_102));
    if (local_102[uVar5 - 0x1] != '\\') {
      local_102[uVar5] = '\\';
      local_102[uVar5 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(0x1050,local_102),pcVar1);
  }
  if (local_102[0] != '\0') {
    uVar7 = (param_2 >> 0x10);
    send_msg_1020_097e((param_2 + 0xe8));
    uVar2 = (param_2 + 0xe8);
    UpdateWindow16((uVar2 + 0x8));
    HVar6 = LoadCursor16(0x7f02,0x0);
    param_3 = (param_3 & 0xffff0000 | HVar6);
    HVar6 = SetCursor16(HVar6);
    param_3 = CONCAT22(0x1050,local_102);
    win_ui_op_1008_1414(uVar9,param_2,param_3,param_6,param_5,param_4);
    param_3 = CONCAT22(HVar6,0x1538);
    SetCursor16(HVar6);
  }
  return;
}


pub unsafe fn bring_win_to_top_1038_b72e(mut param_1: u32, mut param_2: i16) -> BOOL16

{
  let mut hwnd: HWND16;
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_2 * 0x4 + param_1) != 0) {
    uVar1 = (param_2 * 0x4 + param_1);
    hwnd = (uVar1 + 0x6);
    SetFocus16(hwnd);
    BringWindowToTop16(hwnd);
    return 0x1;
  }
  return 0x0;
}



pub unsafe fn win_ui_op_1038_b81c(mut param_1: u16 ,struct_b_param_1: *mut StructB)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u16;
  let mut HVar5: HWND16;
   let mut win_enabled: *mut astruct_909;
  let mut extraout_DX: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut uVar9: u32;
   let mut struct_b_8: *mut StructB;
  let mut unaff_SI: u16;
  let mut uVar7: u16;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut piStack16: *mut i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
   let mut iVar7: *mut StructB;
  let mut uVar8: u16;
  let mut piVar6: *mut i16;

  paVar7 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(unaff_SI,0x6),in_stack_0000fe8e,
                            in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar9 = puVar10 >> 0x10;
  uVar7 = (struct_b_param_1 >> 0x10);
  struct_b_8 = struct_b_param_1;
  struct_b_8[0x7].lpvoid_field_0x8 = puVar10;
  struct_b_8[0x7].max_count_field_0x10 = (puVar10 >> 0x10);
  uVar1 = &struct_b_8[0x7].lpvoid_field_0x8;
  uVar4 = uVar1 + 0x4e;
  uVar1 &= 0xffff0000;
  piVar6 = (uVar1 | uVar4);
  iStack10 = 0;
//   for (iStack12 = 0x1a0; extraout_DX = uVar9, iStack12 < 0x1b5; iStack12 += 1)
  iStack12 = 0x1a0;
  extraout_DX = uVar9;
  while iStack12 < 0x1b5
  {
    if ((iStack10 * 0x2 + uVar4) == iStack12) {
      iStack10 += 0x1;
    }
    else {
      CheckDlgButton16(0x2,iStack12,struct_b_8.lpvoid_field_0x8);
    }
    iStack12 += 1;
  }
  HVar5 = GetDlgItem16(0xfb1,struct_b_8.lpvoid_field_0x8);
  win_enabled = EnableWindow16(0x0,HVar5);
  uVar2 = &struct_b_8[0x7].lpvoid_field_0x8;
  ppcVar3 = (*&struct_b_8[0x7].lpvoid_field_0x8 + 0x10);
  (**ppcVar3)(s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10));
  piStack16 = CONCAT22(extraout_DX,win_enabled);
  move_win_1040_826c(struct_b_param_1,win_enabled.field1_0x2 -0x2,win_enabled.field2_0x4 + *piStack16 + 0x3);
  ShowWindow16(0x5,struct_b_8.lpvoid_field_0x8);
  pass1_1018_1c9a(&struct_b_8[0x7].lpvoid_field_0x8,*piVar6);
  HVar5 = GetDlgItem16(*piVar6,struct_b_8.lpvoid_field_0x8);
  SetFocus16(HVar5);
  return;
}

pub unsafe fn win_ui_op_1038_b922(param_1: *mut StructD, param_2: *mut StructC, mut param_3: u32, mut param_4: u16, mut param_5: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  let mut ppcVar2: *mut *mut code;
  let mut UVar3: u16;
  let mut HVar4: HWND16;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: *mut StructC;
  let mut uVar8: *mut StructC;
  let mut uVar9: u16;
  let mut LVar10: LRESULT;
  let mut pcVar11: *mut c_char;
  let mut paVar12: *mut Struct57;
  let mut in_stack_0000fa38: u16;
  let mut in_stack_0000fb5c: u16;
  let mut in_stack_0000fb62: u16;
  let mut in_stack_0000fb66: u16;
  let mut uVar13: u8;
  let mut pWVar14: *mut WORD;
  let mut uVar15: u16;
  let mut puStack1128: *mut u32;
  let mut local_464: [u8;0x50] = [0;0x50];
  let mut local_414: [u16;0x200] = [0;0x200];
  let mut puStack20: *mut u16;
  let mut puStack16: *mut u8;
  let mut puStack14: *mut u32;
  let mut uStack10: u16;
  let mut HStack8: HWND16;
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0;
  uStack4 = 0;
  iVar8 = param_2;
  uVar8 = (param_2 >> 0x10);
  if (param_4 < 0x1b5) {
    if (param_4 < 0x1a0) {
//      if (param_4 != 0x2) goto LAB_1038_bbbf;
    }
    else {
      HStack8 = GetDlgItem16(param_4,iVar8.field6_0x6);
      LVar10 = SendMessage16(0x0,0x0,0x400,HStack8);
      uStack10 = LVar10;
      if (uStack10 == 0x2) {
        BStack6 = 0;
        uStack4 = 0;
    // TODO: goto LAB_1038_bc26;
      }
      SendMessage16(0x0,(uStack10 == 0),0x401,HStack8);
      UVar3 = IsDlgButtonChecked(param_4,iVar8.field6_0x6);
      if (UVar3 == 0) {
        piVar1 = &iVar8.field_0x96;
        *piVar1 = *piVar1 + 1;
        if (&iVar8.field_0x96 == 1) {
          HVar4 = GetDlgItem16(0xfb1,iVar8.field6_0x6);
          EnableWindow16(0x0,HVar4);
        }
      }
      else {
        piVar1 = &iVar8.field_0x96;
        *piVar1 = *piVar1 -0x1;
        HVar4 = GetDlgItem16(0xfb1,iVar8.field6_0x6);
        BVar5 = IsWindowEnabled16(HVar4);
        if (BVar5 == 0) {
          HVar4 = GetDlgItem16(0xfb1,iVar8.field6_0x6);
          EnableWindow16(0x1,HVar4);
        }
        if (&iVar8.field_0x96 < 0x0) {
          CheckDlgButton16(0x0,iVar8.field145_0x98,iVar8.field6_0x6);
          iVar8.field_0x96 = 0;
        }
        iVar8.field145_0x98 = param_4;
        pass1_1018_1c9a(iVar8.field142_0x92,param_4);
        puStack14 = pass1_1018_1e78(iVar8.field142_0x92,-1);
        uVar6 = (puStack14 >> 0x10);
        uVar7 = uVar6 | puStack14;
        if (uVar7 == 0) {
          puStack16 = null_mut();
        }
        else {
          puStack16 = (puStack14 + 0x1c);
        }
        win_1008_5c7c(puStack16,uVar7,_u16_1050_02a0,CONCAT22(puStack16,1));
      }
    }
    BStack6 = 0x1;
    uStack4 = 0;
  }
  else {
    if (param_4 == 0xfb1) {
    //   for (uVar6 = 0x1a0; uVar6 < 0x1b5; uVar6 += 1)
    for uVar6 in 0x1a0 .. 0x1b5
    {
        UVar3 = IsDlgButtonChecked(uVar6,iVar8.field6_0x6);
        if (UVar3 == 1) {
          pass1_1008_d818(iVar8.field141_0x8e,uVar6);
      // TODO: goto LAB_1038_bba2;
        }
      }
    }
    else {
//      if (param_4 != 0xfbe) goto LAB_1038_bbbf;
      puStack14 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_5,0x2),
                                  in_stack_0000fa38,in_stack_0000fb5c,in_stack_0000fb62,in_stack_0000fb66);
      uVar9 = (param_1 >> 0x10);
      puStack16 = PTR_LOOP_1050_13ae;
      if (PTR_LOOP_1050_13ae == (&PTR_LOOP_1050_0000 + 1)) {
        puStack16 = &u16_1050_0002;
      }
      uStack10 = (puStack16 * 0xc + 0x5b84) - 0x1;
      pass1_1008_612e(uStack10,0x0,uStack10);
      puStack20 = pass1_1018_1e78(iVar8.field142_0x92,((puStack16 * 0x6 + uStack10) * 0x2 + 0x5b86)
                                        );
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x50,local_464,&DAT_1050_1050);
      pcVar11 = load_string_1010_847e(_u16_1050_14cc,*puStack20);
      uVar15 = (pcVar11 >> 0x10);
      paVar12 = CONCAT22(uVar9,uVar15);
      uVar13 = SUB21(local_464,0x0);
      uVar6 = wsprintf16(local_414,0x5bc01050,
                         CONCAT13((local_464 >> 0x8),CONCAT12(uVar13,0x1050)),uVar13,
                         &DAT_1050_1050,pcVar11,uVar15);
      uVar9 = 0x1000;
      mem_op_1000_179c(0xb4,paVar12);
      uVar7 = paVar12;
      if ((uVar7 | uVar6) == 0) {
        uVar6 = 0;
        paVar12 = null_mut();
      }
      else {
        pWVar14 = local_414;
        uVar15 = SUB42(&DAT_1050_1050,0x0);
        HVar4 = HWND16_1050_0396;
        pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x57b);
        uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
        paVar12 = pass1_1040_8478((pcVar11 >> 0x10),CONCAT22(uVar7,uVar6),0x41,pcVar11,
                                  CONCAT22(uVar15,pWVar14),HVar4);
        uVar6 = paVar12;
      }
      param_1 = (paVar12 >> 0x10);
      puStack1128 = (paVar12 & 0xffff0000 | uVar6);
      ppcVar2 = (*puStack1128 + 0x74);
      HStack8 = (**ppcVar2)(uVar9,uVar6,(paVar12 >> 0x10));
//      if (HStack8 != 1) goto LAB_1038_bc26;
      pass1_1008_d818(iVar8.field141_0x8e,(puStack20 + 0x1a));//
// LAB_1038_bba2:
      win_ui_cursor_op_1038_bc30(param_2);
    }
    PostMessage16(0x0,0xce,0x111,HWND16_1050_0396);
    param_4 = 0x1;//
// LAB_1038_bbbf:
    uVar9 = SUB42(param_1,0x0);
    BStack6 = post_win_msg_1040_7b3c(param_2,param_3,(param_3 >> 0x10),param_4);
    uStack4 = uVar9;
  }//
// LAB_1038_bc26:
  return CONCAT22(uStack4,BStack6);
}



pub unsafe fn win_ui_cursor_op_1038_bc30(param_1: *mut StructC)

{
  let mut uVar1: u32;
  let mut local_112: u16;
  let mut uStack272: u16;
  let mut HStack6: HCURSOR16;
  let mut HStack4: HCURSOR16;

  HStack4 = LoadCursor16(0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  uVar1 = (param_1 + 0x8e);
  pass1_1030_532e(CONCAT22(0x1050,&local_112),(uVar1 + 0xe) + 0x1000000);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_112));
  pass1_1030_838e(_u16_1050_5748);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334();
  SetCursor16(HStack6);
  return;
}



pub unsafe fn win_dlg_op_1038_bea4(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut HVar3: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut uVar8: u32;
  let mut lparam: *mut c_char;
  let mut LVar9: LRESULT;
  let mut in_stack_0000fd7a: u16;
  let mut in_stack_0000fd7c: u16;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000fea4: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000fea8: u16;
  let mut in_stack_0000feaa: u16;
  let mut in_stack_0000fed2: u16;
  let mut in_stack_0000fed4: u16;
  let mut local_116: *mut u32;
  let mut local_112: *mut u32;
  let mut local_10e: [u16;0x41] = [0;0x41];
  let mut local_8c: [u8;0x82] = [0;0x82];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(in_stack_0000fed2,0x2),in_stack_0000fd7a,
                             in_stack_0000fe9e,in_stack_0000fea4,in_stack_0000fea8);
  paVar4 = (paVar4 & 0xffff0000 | puStack6 >> 0x10);
  uStack10 = (puStack6 + 0x68);
  uVar6 = (param_2 >> 0x10);
  iVar5 = param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),(iVar5 + 0x6));
  wsprintf16(local_10e,CONCAT22(local_8c,0x1050),CONCAT22(uStack10,0x1050),
             (uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),(iVar5 + 0x6));
  HVar3 = GetDlgItem16(0x179,(iVar5 + 0x6));
  (iVar5 + 0x92) = HVar3;
  pass1_1008_e3ec((iVar5 + 0x8e),CONCAT22(0x1050,&local_116),
                  CONCAT22(0x1050,&local_112));
  send_msg_1038_c374(param_2,local_112,(iVar5 + 0x92));
  puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(in_stack_0000fed4,0x2f),in_stack_0000fd7c,
                           in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
  uVar2 = (puVar7 >> 0x10);
  uVar8 = (puVar7 + 0x24);
  uVar1 = (iVar5 + 0x8e);
  uVar8 = string_1008_e586(uVar1,(uVar1 >> 0x10),uVar8,uVar8,uVar2);
  SendMessage16(uVar8,0xffff,0x40d,(iVar5 + 0x92));
  HVar3 = GetDlgItem16(0x17a,(iVar5 + 0x6));
  (iVar5 + 0x94) = HVar3;
  send_msg_1038_c374(param_2,local_116,HVar3);
  lparam = load_string_1010_847e(_u16_1050_14cc,0x531);
  LVar9 = SendMessage16(lparam,0x0,0x403,(iVar5 + 0x94));
  (iVar5 + 0x9c) = LVar9;
  SendMessage16(lparam,0xffff,0x40d,(iVar5 + 0x94));
  HVar3 = GetDlgItem16(0x178,(iVar5 + 0x6));
  (iVar5 + 0x96) = HVar3;
  HVar3 = GetDlgItem16(0x177,(iVar5 + 0x6));
  (iVar5 + 0x98) = HVar3;
  HVar3 = GetDlgItem16(0x184,(iVar5 + 0x6));
  (iVar5 + 0x9a) = HVar3;
  return;
}



pub unsafe fn win_ui_op_1040_cace(mut param_1: u16, mut param_2: u32)

{
    let mut uVar1: u32;
    let mut bVar2: bool;
    let mut iVar3: i16;
    let mut IVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut bVar7: bool;
    let mut uVar8: u16;
    let mut local_208: [u8; 0x100] = [0; 0x100];
    let mut local_108: [u8; 0x100] = [0; 0x100];
    let mut UStack8: u16;
    let mut uStack6: u16;
    let mut local_4: bool;

    uVar6 = (param_2 >> 0x10);
    uVar5 = param_2;
    uStack6 = GetDlgItemInt16(0x0, &local_4, &DAT_1050_1050, 0x18e);
    UStack8 = GetDlgItemInt16(0x0, &local_4, &DAT_1050_1050, 0x191);
    if (uStack6 == 0) {
        return;
    }
    pass1_1018_50ea((uVar5 + 0x98), uStack6, (uVar5 + 0x94));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_208, &DAT_1050_1050);
    uVar1 = (uVar5 + 0x94);
    uVar8 = (_u16_1050_14cc >> 0x10);
    if ((uVar1 + 0x36) == 0) {
        load_string_1010_84e0(_u16_1050_14cc, uVar8, 0x3ff, local_108, &DAT_1050_1050);
        iVar3 = MessageBox16(0x34, CONCAT22(0x1050, local_208), CONCAT22(0x1050, local_108),
                             (uVar5 + 0x8));
    } else {
        load_string_1010_84e0(_u16_1050_14cc, uVar8, 0x3ff, local_108, &DAT_1050_1050);
        iVar3 = MessageBox16(0x34, CONCAT22(0x1050, local_208), CONCAT22(0x1050, local_108),
                             (uVar5 + 0x8));
    }
    bVar2 = iVar3 == 0x6;
    bVar7 = false;
    if ((!bVar2) && (uVar1 = (uVar5 + 0x94), (uVar1 + 0x34) < 1)) {
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_108, &DAT_1050_1050);
        IVar4 = MessageBox16(0x34, CONCAT22(0x1050, local_208), CONCAT22(0x1050, local_108),
                             (uVar5 + 0x8));
        bVar7 = IVar4 == 0x6;
        bVar2 = false;
    }
    if (bVar2) {
        _PTR_LOOP_1050_5f16 = (uVar5 + 0x94);
        iVar3 = 0x26;
    } else {
        if (!bVar7) {
            return;
        }
        _u16_1050_5a68 = (uVar5 + 0x94);
        iVar3 = 0x27;
    }
    pass1_1038_af40(uVar5, param_1, _PTR_LOOP_1050_5b7c, (uVar5 + 0x8), iVar3);
    return;
}



pub unsafe fn msg_box_op_1040_cce4(param_1: *mut c_char,mut param_2: u16 ,param_3: *mut Struct903)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uStack522: u32;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];
  let mut uVar2: u16;
  let mut uVar3: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}

pub unsafe fn send_dlg_item_msg_1040_ce12(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut lVar2: i32;
  let mut local_10a: [u16;0x80] = [0;0x80];
  let mut local_a: [u8;0x8] = [0;0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),param_3);
  loop {
    lVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar2 == 0) { break; }
    uVar1 = (lVar2 + 0x4);
    wsprintf16(local_10a,0x5f121050,CONCAT22(uVar1,0x1050),(uVar1 >> 0x10));
    SendDlgItemMessage16(CONCAT22(0x1050,local_10a),0x0,0x401,param_4,(param_1 + 0x6));
  }
  return;
}

pub unsafe fn send_dlg_item_1040_ce76(param_1: *mut Struct903)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut uVar4: u32;
  let mut local_106: [u8;0x100] = [0;0x100];
  let mut WStack6: WPARAM16;
  let mut iStack4: i16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendDlgItemMessage16(0x0,0x0,0x409,s_vrpal_bmp_1050_183a + 0x8,(iVar1 + 0x6));
  WStack6 = LVar3;
  iStack4 = WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16
              (CONCAT22(0x1050,local_106),WStack6,0x40a,s_vrpal_bmp_1050_183a + 0x8,(iVar1 + 0x6));
    uVar4 = pass1_1018_5206((iVar1 + 0x98),CONCAT22(0x1050,local_106));
    if (uVar4 != 0) {
      (iVar1 + 0x9c) = (uVar4 + 0x8);
      (iVar1 + 0x9e) = 0;
      SetDlgItemInt16(0x0,0x0,0x18e,(iVar1 + 0x6));
      SetDlgItemInt16(0x0,(iVar1 + 0x9c),0x191,(iVar1 + 0x6));
    }
  }
  return;
}

pub unsafe fn send_dlg_msg_1040_cf1c(mut param_1: u16, param_2: *mut Struct903) -> LRESULT

{
  let mut puVar1: *mut u8;
  let mut hwnd: HWND16;
  let mut in_register_0000000a: u16;
  let mut uVar1: *mut astruct_928;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut uVar4: u32;
  let mut LVar5: LRESULT;
  let mut in_stack_0000f99c: u16;
  let mut in_stack_0000fac0: u16;
  let mut in_stack_0000fac6: u16;
  let mut in_stack_0000faca: u16;
  let mut enable: bool;
  let mut uVar6: u16;
  let mut in_stack_0000faf4: u16;
  let mut local_106: [u8;0x100] = [0;0x100];

  puVar3 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           CONCAT22(in_stack_0000faf4,0x3),in_stack_0000f99c,in_stack_0000fac0,
                           in_stack_0000fac6,in_stack_0000faca);
  puVar1 = (puVar3 >> 0x10);
  uVar2 = (param_2 >> 0x10);
  uVar1 = param_2;
  pass1_1010_c3c2(puVar1,puVar3,puVar1,CONCAT22(0x1050,local_106),uVar1.field147_0x94);
  SendDlgItemMessage16(CONCAT22(0x1050,local_106),0x0,0xc,s_dibtext_bmp_1050_1844 + 0x2,uVar1.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,s_vrpal_bmp_1050_183a + 0x8,uVar1.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,s_vrpal_bmp_1050_183a + 0x8,uVar1.field6_0x6);
  uVar6 = s_vrpal_bmp_1050_183a + 0x8;
  uVar4 = pass1_1018_526a(uVar1.field148_0x98,uVar1.field147_0x94);
  send_dlg_item_msg_1040_ce12(uVar1,uVar2,uVar4,uVar6);
  LVar5 = SendDlgItemMessage16(0x0,0x0,0x40c,s_vrpal_bmp_1050_183a + 0x8,uVar1.field6_0x6);
  if (((LVar5 >> 0x10) < 1) && ((LVar5 < 0x0 || (LVar5 == 0)))) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,&stack0xfaf4,&DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,&stack0xfaf4),0x0,0x401,s_vrpal_bmp_1050_183a + 0x8,uVar1.field6_0x6);
    hwnd = GetDlgItem16(0x1,uVar1.field6_0x6);
    enable = 0;
  }
  else {
    hwnd = GetDlgItem16(0x1,uVar1.field6_0x6);
    enable = 0x1;
  }
  EnableWindow16(enable,hwnd);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0xb,s_vrpal_bmp_1050_183a + 0x8,uVar1.field6_0x6);
  return LVar5;
}



pub unsafe fn send_msg_1040_c85a(mut param_1: u32)

{
    _PTR_LOOP_1050_5efe = param_1;
    SendMessage16(0x0, 0xfa, 0x111, (param_1 + 0x1a));
    return;
}


pub unsafe fn pass1_1040_cdac(mut param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: i16) -> u16

{
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut bVar3: bool;
    let mut iVar4: i16;
    let mut uVar5: u16;

    bVar3 = false;
    iVar4 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (param_4 == 0) {
        iVar2 = (iVar4 + 0x9e);
        piVar1 = (iVar4 + 0x9c);
//    if (*piVar1 == iVar2 || *piVar1 < iVar2) goto LAB_1040_cdef;
        piVar1 = (iVar4 + 0x9e);
        *piVar1 = *piVar1 + 1;
    } else {
//    if (param_4 != 1) goto LAB_1040_cdef;
//    if ((iVar4 + 0x9e) < 1) goto LAB_1040_cdef;
        piVar1 = (iVar4 + 0x9e);
        *piVar1 = *piVar1 - 0x1;
    }
    bVar3 = true;//
// LAB_1040_cdef:
    if (bVar3) {
        SetDlgItemInt16(0x0, (iVar4 + 0x9e), 0x18e, (iVar4 + 0x6));
    }
    return 0x0;
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

pub unsafe fn destroy_cursor_1020_42f4(param_1: *mut StructD) {
    let mut struct_1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    struct_1 = param_1;
    param_1.address_offset_field_0x0 = 0x623c;
    struct_1.address_offset_field_0x2 = 0x1020;
    struct_1.field_0xe2 = 0x62d8;
    struct_1.field_0xe4 = 0x1020;
    if (struct_1[0x1].field13_0x18 != 0) {
        DestroyMenu16(struct_1[0x1].field13_0x18);
    }
    DestroyCursor16(struct_1[0x1].address_offset_field_0x2);
    DestroyCursor16(struct_1[0x1].hfile_0x4);
    pass1_1020_808e(param_1);
    return;
}


pub unsafe fn mixed_win_ui_op_1040_6942(param_1: *mut Struct57, mut param_2: u16, struct_b_param_1: *mut StructB, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut Struct57;
    let mut hwnd: *mut u32;
    let mut iVar3: *mut astruct_790;
    let mut uVar4: u16;
    let mut uVar10: u16;
    let mut uVar5: u16;
    let mut paVar13: *mut Struct57;
    let mut struct_b_6: *mut StructB;
    let mut uVar6: u16;
    let mut uVar9: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut puVar15: *mut u32;
    let mut puVar14: *mut u16;
    let mut DVar16: u32;
    let mut in_stack_0000fdd4: u16;
    let mut in_stack_0000fdd6: u16;
    let mut in_stack_0000fdd8: u16;
    let mut in_stack_0000fdda: u16;
    let mut in_stack_0000fe32: u16;
    let mut in_stack_0000fefe: u16;
    let mut in_stack_0000ff00: u16;
    let mut in_stack_0000ff02: u16;
    let mut in_stack_0000ff04: u16;
    let mut in_stack_0000ff06: u16;
    let mut in_stack_0000ff08: u16;
    let mut in_stack_0000ff56: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ff60: u16;
    let mut uVar17: u8;
    let mut uVar20: u8;
    let mut BVar21: bool;
    let mut uVar22: u16;
    let mut pcVar23: *mut c_char;
    let mut hdc: HDC16;
    let mut local_64: u32;
    let mut uStack96: u32;
    let mut HStack92: HWND16;
//   HMENlet mut HStack90: u16;
    let mut HStack90: HMENU16;
    let mut local_58: [u8; 0x50] = [0; 0x50];
    let mut hdc_8: HDC16;
    let mut paStack6: *mut Struct57;
    let mut uStack4: u16;
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar18: u8;
    let mut uVar19: u8;
    let mut in_stack_0000ff8a: u16;
    let mut paVar11: *mut Struct57;
    let mut paVar12: *mut Struct57;
    let mut uVar14: u32;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar15 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_2, 0x33), in_stack_0000fe32,
                              in_stack_0000ff56, in_stack_0000ff5c, in_stack_0000ff60);
    paVar11 = (param_1 & 0xffff0000 | puVar15 >> 0x10);
    paVar3 = puVar15;
    uVar6 = (struct_b_param_1 >> 0x10);
    struct_b_6 = struct_b_param_1;
    struct_b_6[0x7].max_count_field_0x10 = paVar3;
    struct_b_6[0x7].field5_0xa = (puVar15 >> 0x10);
    ppcVar2 = (*&struct_b_6[0x7].max_count_field_0x10 + 0x4);
    (**ppcVar2)(0x1010, struct_b_6[0x7].max_count_field_0x10, (puVar15 >> 0x10), 0x0, struct_b_param_1);
    mem_op_1000_179c(0xa, paVar11);
    uVar10 = paVar11 | paVar3;
    paVar12 = (paVar11 & 0xffff0000 | uVar10);
    if (uVar10 == 0) {
        struct_b_6[0x7].field6_0xc = 0;
    } else {
        puVar14 = struct_1040_bf3e(CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, paVar3)),
                                   struct_b_6.lpvoid_field_0x8);
        paVar12 = (paVar12 & 0xffff0000 | puVar14 >> 0x10);
        paVar3 = puVar14;
        struct_b_6[0x7].field6_0xc = paVar3;
        struct_b_6[0x7].field7_0xe = (puVar14 >> 0x10);
    }
    // WARNING: Load size is inaccurate
    pass1_1040_bfde(struct_b_6[0x7].field6_0xc, &struct_b_6[0x7].max_count_field_0x10);
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar3;
    paVar11 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar11, paVar3, paVar12, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(struct_b_6.lpvoid_field_0x8, 0x10a), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar10 = paVar11 | paVar3;
    paVar12 = (paVar11 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar3, paVar11, 0x1, 0xa0028, 0x0, 0x820083,
                        CONCAT22(struct_b_6.lpvoid_field_0x8, 0x10c), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    BVar21 = 0;
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar3;
    paVar11 = (paVar12 & 0xffff0000);
    paVar13 = (paVar11 | uVar10);
    if (uVar10 == 0) {
        paVar3 = null_mut();
    } else {
        pvVar1 = struct_b_6.lpvoid_field_0x8;
        pass1_1008_3bd6(paVar13, paVar3, paVar12, 0x1, 0xa00aa, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x107)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        paVar11 = paVar13;
    }
    uStack4 = SUB42(paVar11, 0x0);
    paStack6 = paVar3;
    enable_win_1040_9234(CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, paVar3)), BVar21);
    BVar21 = 0;
    mem_op_1000_179c(0x42, paVar11);
    uVar5 = paVar11 | paVar3;
    uVar14 = paVar11 & 0xffff0000 | uVar5;
    if (uVar5 == 0) {
        paVar3 = null_mut();
        uStack4 = 0;
    } else {
        pvVar1 = struct_b_6.lpvoid_field_0x8;
        pass1_1008_3bd6(uVar14, paVar3, paVar11, 0x1, 0xa00c2, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x108)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        uStack4 = uVar14;
    }
    paStack6 = paVar3;
    enable_win_1040_9234(CONCAT13((uStack4 >> 0x8), CONCAT12(uStack4, paVar3)), BVar21);
    hdc_8 = GetDC16(struct_b_6.lpvoid_field_0x8);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar22 = SUB42(&DAT_1050_1050, 0x0);
    uVar17 = SUB21(local_58, 0x0);
    uVar20 = (local_58 >> 0x8);
    hdc = hdc_8;
    uVar10 = str_op_1000_3da4(CONCAT22(0x1050, local_58));
    DVar16 = GetTextExtent16(uVar10, CONCAT22(uVar22, CONCAT11(uVar20, uVar17)), hdc);
    HStack90 = (HMENU16)(DVar16 >> 0x10);
    HStack92 = DVar16;
    CreateWindow16(0x0, CONCAT22(0x7cd, HINSTANCE16_1050_038c), struct_b_6.lpvoid_field_0x8, HStack90,
                   HStack92, 0xad, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT13(0x10, CONCAT12(0x50, local_58)),
                   s_static_1050_5d84);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar18 = hdc_8;
    uVar19 = (hdc_8 >> 0x8);
    pcVar23 = local_58;
    uVar22 = SUB42(&DAT_1050_1050, 0x0);
    uVar10 = str_op_1000_3da4(CONCAT13(0x10, CONCAT12(0x50, pcVar23)));
    DVar16 = GetTextExtent16(uVar10, CONCAT22(uVar22, pcVar23), CONCAT11(uVar19, uVar18));
    HStack90 = (HMENU16)(DVar16 >> 0x10);
    HStack92 = DVar16;
    ReleaseDC16(hdc_8, struct_b_6.lpvoid_field_0x8);
    CreateWindow16(0x0, CONCAT22(0x7ce, HINSTANCE16_1050_038c), struct_b_6.lpvoid_field_0x8, HStack90,
                   HStack92, 0xc5, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT22(0x1050, local_58),
                   s_static_1050_5d8b);
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    hwnd = &local_64;
    create_window_1040_6eae(struct_b_param_1, 0x1, CONCAT22(0x1050, hwnd), 0x5eb, 0xfd);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_6eae(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ec, 0xfe);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_6eae(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ee, 0xff);
    SendMessage16(0x0, 0x1, 0x401, hwnd);
    uVar1 = &struct_b_6[0x7].max_count_field_0x10;
    iVar3 = uVar1;
    iVar3 = &iVar3.field_0xa;
    uVar9 = ((uVar1 & 0xffff0000) >> 0x10);
    SetWindowPos16(0x40, iVar3.field14_0x10, iVar3.field13_0xe, iVar3.field12_0xc,
                   (uVar1 & 0xffff0000 | ZEXT24(iVar3)), 0x0, struct_b_6.lpvoid_field_0x8);
    DAT_1050_0ecc = 0;
    uVar2 = &struct_b_6[0x7].max_count_field_0x10;
    ppcVar2 = (*&struct_b_6[0x7].max_count_field_0x10 + 0x10);
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10));
    pass1_1010_2ee2(&struct_b_6[0x7].max_count_field_0x10);
    PostMessage16(0x0, 0x10a, 0x111, struct_b_6.lpvoid_field_0x8);
    return;
}


pub unsafe fn mixed_win_ui_op_1040_70b4(param_1: *mut Struct57, mut param_2: u16, struct_b_param_1: *mut StructB, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut count: u16;
    let mut hwnd: *mut u32;
    let mut iVar3: *mut astruct_792;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut paVar5: *mut Struct57;
    let mut paVar7: *mut Struct57;
    let mut struct_b_5: *mut StructB;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut puVar10: *mut u32;
    let mut puVar11: *mut u16;
    let mut DVar11: u32;
    let mut DVar12: u32;
    let mut in_stack_0000fdd4: u16;
    let mut in_stack_0000fdd6: u16;
    let mut in_stack_0000fdd8: u16;
    let mut in_stack_0000fdda: u16;
    let mut in_stack_0000fe32: u16;
    let mut in_stack_0000fefe: u16;
    let mut in_stack_0000ff00: u16;
    let mut in_stack_0000ff02: u16;
    let mut in_stack_0000ff04: u16;
    let mut in_stack_0000ff06: u16;
    let mut in_stack_0000ff08: u16;
    let mut in_stack_0000ff56: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ff60: u16;
    let mut uVar11: u8;
    let mut uVar12: u8;
    let mut BVar13: bool;
    let mut uVar16: u16;
    let mut pcVar17: *mut c_char;
    let mut hdc: HDC16;
    let mut local_64: u32;
    let mut uStack96: u32;
    let mut HStack92: HWND16;
//   HMENlet mut HStack90: u16;
    let mut HStack90: HMENU16;
    let mut local_58: [u8; 0x50] = [0; 0x50];
    let mut HStack8: HDC16;
    let mut paStack6: *mut Struct57;
    let mut uStack4: u16;
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut in_stack_0000ff8a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar9: u32;
    let mut fn_ptr_1: *mut *mut code;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar10 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_2, 0x34), in_stack_0000fe32,
                              in_stack_0000ff56, in_stack_0000ff5c, in_stack_0000ff60);
    paVar5 = (param_1 & 0xffff0000 | puVar10 >> 0x10);
    paVar2 = puVar10;
    uVar6 = (struct_b_param_1 >> 0x10);
    struct_b_5 = struct_b_param_1;
    struct_b_5[0x7].max_count_field_0x10 = paVar2;
    struct_b_5[0x7].field5_0xa = (puVar10 >> 0x10);
    fn_ptr_1 = (*&struct_b_5[0x7].max_count_field_0x10 + 0x4);
    (**fn_ptr_1)(0x1010, struct_b_5[0x7].max_count_field_0x10, (puVar10 >> 0x10), 0x0, struct_b_param_1);
    mem_op_1000_179c(0xa, paVar5);
    uVar4 = paVar5 | paVar2;
    paVar6 = (paVar5 & 0xffff0000 | uVar4);
    if (uVar4 == 0) {
        struct_b_5[0x7].field6_0xc = 0;
    } else {
        puVar11 = struct_1040_bf3e(CONCAT13((paVar5 >> 0x8), CONCAT12(paVar5, paVar2)),
                                   struct_b_5.lpvoid_field_0x8);
        paVar6 = (paVar6 & 0xffff0000 | puVar11 >> 0x10);
        paVar2 = puVar11;
        struct_b_5[0x7].field6_0xc = paVar2;
        struct_b_5[0x7].field7_0xe = (puVar11 >> 0x10);
    }
    // WARNING: Load size is inaccurate
    pass1_1040_bfde(struct_b_5[0x7].field6_0xc, &struct_b_5[0x7].max_count_field_0x10);
    mem_op_1000_179c(0x42, paVar6);
    uVar4 = paVar6 | paVar2;
    paVar5 = (paVar6 & 0xffff0000 | uVar4);
    if (uVar4 != 0) {
        pass1_1008_3bd6(paVar5, paVar2, paVar6, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(struct_b_5.lpvoid_field_0x8, 0x10a), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    mem_op_1000_179c(0x42, paVar5);
    uVar4 = paVar5 | paVar2;
    paVar6 = (paVar5 & 0xffff0000 | uVar4);
    if (uVar4 != 0) {
        pass1_1008_3bd6(paVar6, paVar2, paVar5, 0x1, 0xa0028, 0x0, 0x820083,
                        CONCAT22(struct_b_5.lpvoid_field_0x8, 0x10c), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    BVar13 = 0;
    mem_op_1000_179c(0x42, paVar6);
    uVar4 = paVar6 | paVar2;
    paVar5 = (paVar6 & 0xffff0000);
    paVar7 = (paVar5 | uVar4);
    if (uVar4 == 0) {
        paVar2 = null_mut();
    } else {
        pvVar1 = struct_b_5.lpvoid_field_0x8;
        pass1_1008_3bd6(paVar7, paVar2, paVar6, 0x1, 0xa00aa, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x107)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        paVar5 = paVar7;
    }
    uStack4 = SUB42(paVar5, 0x0);
    paStack6 = paVar2;
    enable_win_1040_9234(CONCAT13((paVar5 >> 0x8), CONCAT12(paVar5, paVar2)), BVar13);
    BVar13 = 0;
    mem_op_1000_179c(0x42, paVar5);
    uVar5 = paVar5 | paVar2;
    uVar9 = paVar5 & 0xffff0000 | uVar5;
    if (uVar5 == 0) {
        paVar2 = null_mut();
        uStack4 = 0;
    } else {
        pvVar1 = struct_b_5.lpvoid_field_0x8;
        pass1_1008_3bd6(uVar9, paVar2, paVar5, 0x1, 0xa00c2, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x108)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        uStack4 = uVar9;
    }
    paStack6 = paVar2;
    enable_win_1040_9234(CONCAT13((uStack4 >> 0x8), CONCAT12(uStack4, paVar2)), BVar13);
    HStack8 = GetDC16(struct_b_5.lpvoid_field_0x8);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar16 = SUB42(&DAT_1050_1050, 0x0);
    uVar11 = SUB21(local_58, 0x0);
    uVar12 = (local_58 >> 0x8);
    hdc = HStack8;
    uVar3 = str_op_1000_3da4(CONCAT22(0x1050, local_58));
    DVar11 = GetTextExtent16(uVar3, CONCAT22(uVar16, CONCAT11(uVar12, uVar11)), hdc);
    HStack90 = (HMENU16)(DVar11 >> 0x10);
    HStack92 = DVar11;
    CreateWindow16(0x0, CONCAT22(0x7cd, HINSTANCE16_1050_038c), struct_b_5.lpvoid_field_0x8, HStack90,
                   HStack92, 0xad, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT13(0x10, CONCAT12(0x50, local_58)),
                   s_static_1050_5d9a);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar14 = HStack8;
    uVar15 = (HStack8 >> 0x8);
    pcVar17 = local_58;
    uVar16 = SUB42(&DAT_1050_1050, 0x0);
    count = str_op_1000_3da4(CONCAT13(0x10, CONCAT12(0x50, pcVar17)));
    DVar12 = GetTextExtent16(count, CONCAT22(uVar16, pcVar17), CONCAT11(uVar15, uVar14));
    HStack90 = (HMENU16)(DVar12 >> 0x10);
    HStack92 = DVar12;
    ReleaseDC16(HStack8, struct_b_5.lpvoid_field_0x8);
    CreateWindow16(0x0, CONCAT22(0x7ce, HINSTANCE16_1050_038c), struct_b_5.lpvoid_field_0x8, HStack90,
                   HStack92, 0xc5, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT22(0x1050, local_58),
                   s_static_1050_5da1);
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    hwnd = &local_64;
    create_window_1040_7620(struct_b_param_1, 0x1, CONCAT22(0x1050, hwnd), 0x5eb, 0xfd);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_7620(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ed, 0xfe);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_7620(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ef, 0xff);
    SendMessage16(0x0, 0x1, 0x401, hwnd);
    uVar1 = &struct_b_5[0x7].max_count_field_0x10;
    iVar3 = uVar1;
    iVar3 = &iVar3.field_0xa;
    uVar16 = ((uVar1 & 0xffff0000) >> 0x10);
    SetWindowPos16(0x40, iVar3.field14_0x10, iVar3.field13_0xe, iVar3.field12_0xc,
                   (uVar1 & 0xffff0000 | ZEXT24(iVar3)), 0x0, struct_b_5.lpvoid_field_0x8);
    DAT_1050_0ecc = 0;
    uVar2 = &struct_b_5[0x7].max_count_field_0x10;
    fn_ptr_1 = (*&struct_b_5[0x7].max_count_field_0x10 + 0x10);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10));
    pass1_1010_2ee2(&struct_b_5[0x7].max_count_field_0x10);
    PostMessage16(0x0, 0x10a, 0x111, struct_b_5.lpvoid_field_0x8);
    return;
}

pub unsafe fn msg_box_op_1000_214c(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: u16,
) -> bool {
    let mut IVar1: i16;
    let mut iVar2: i16;
    let mut msg_type: u16;

    msg_type = 0x2 - (param_2 == 0) | 0x2110;
    MessageBeep16(0x0);
    loop {
        IVar1 = MessageBox16(
            msg_type,
            "SmartHeap Library",
            CONCAT22(param_4, param_3),
            0x0,
        );
        iVar2 = IVar1 -0x1;
        if (iVar2 == 0) {
            return 0x0;
        }
        if ((0x0 < iVar2) && (!SBORROW2(iVar2, 1))) {
            if (IVar1 == 0x3 || IVar1 -0x2 < 1) {
                fatal_app_exit_1000_3e9e();
                return 0x0;
            }
            if (IVar1 == 0x4) {
                return 0x1;
            }
            if (IVar1 == 0x5) {
                return 0x0;
            }
        }
        if ((msg_type & 0x2000) == 0) {
            return 0x0;
        }
        msg_type = msg_type & 0xdfef | 0x1010;
    }
}

pub unsafe fn message_box_op_1008_12dc(param_1: *mut Struct72, mut param_2: u32) {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut in_DX: u16;
    let mut HVar3: HWND16;
    let mut uVar4: u16;
    let mut pcVar5: *mut c_char;
    let mut uVar6: u16;
    let mut hwnd: HWND16;
    let mut uStack16: u32;
    let mut local_c: [u8; 0x6] = [0; 0x6];
    let mut HStack6: HCURSOR16;
    let mut HStack4: HCURSOR16;

    HStack4 = LoadCursor16(0x7f02, 0x0);
    HStack6 = SetCursor16(HStack4);
    str_1008_6d8a(in_DX, CONCAT22(0x1050, local_c), param_2);
    BVar1 = file_fn_1008_6e02(CONCAT22(0x1050, local_c));
    uVar4 = (param_1 >> 0x10);
    if (BVar1 == 0) {
        SetCursor16(HStack6);
        pcVar5 = load_string_1010_847e(_u16_1050_14cc, u16_1050_0310);
        HVar3 = (pcVar5 >> 0x10);
        uVar2 = str_op_1008_60e8(HVar3, pcVar5);
        uStack16 = CONCAT22(HVar3, uVar2);
        pcVar5 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
        MessageBeep16(0x10);
        MessageBox16(0x10, pcVar5, CONCAT22(HVar3, uVar2), (param_1 + 0x8));
    } else {
        (_u16_1050_5748 + 0x8) = 0;
        SetCursor16(HStack6);
        pcVar5 = load_string_1010_847e(_u16_1050_14cc, 0x6d3);
        str_op_1008_60e8((pcVar5 >> 0x10), pcVar5);
        pcVar5 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
        uVar6 = 0;
        MessageBeep16(0x0);
        hwnd = (param_1 + 0x8);
        HVar3 = hwnd;
        MessageBox16(0x40, pcVar5, CONCAT22(hwnd, uVar6), hwnd);
        uStack16 = CONCAT22(HVar3, hwnd);
    }
    fn_ptr_1000_17ce((uStack16 & 0xffff | HVar3 << 0x10));
    close_file_1008_6dd0(CONCAT22(0x1050, local_c));
    return;
}


pub unsafe fn save_file_1008_3178(mut param_1: u16, param_2: *mut Struct72, mut param_3: i16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut puVar3: *mut u16;
    let mut uVar4: u16;
    let mut success: bool;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut pcVar9: *mut c_char;
    let mut in_stack_0000f720: u16;
    let mut in_stack_0000f844: u16;
    let mut in_stack_0000f84a: u16;
    let mut in_stack_0000f84e: u16;
    let mut local_782: [u8; 0x104] = [0; 0x104];
    //   let mut local_67e: [u16;0x4] = [0;0x4];
    let local_67e: [u16; 4] = [0; 4];
    let mut pcStack1654: *mut c_char;
    let mut atype: u16;
    let mut text: u16;
    let mut pcStack1646: *mut c_char;
    let mut local_666: [u8; 0x100] = [0; 0x100];
    let mut pcStack1382: *mut c_char;
    let mut ofn: SEGPTR;
    let mut uStack1374: u16;
    let mut pcStack1370: *mut c_char;
    let mut uStack1368: u16;
    let mut pcStack1354: *mut c_char;
    let mut uStack1350: u32;
    let mut puStack1346: *mut u8;
    let mut uStack1344: u16;
    let mut uStack1342: u32;
    let mut pcStack1338: *mut c_char;
    let mut uStack1336: u16;
    let mut puStack1334: *mut u8;
    let mut uStack1332: u16;
    let mut uStack1330: u32;
    let mut uStack1326: u16;
    let mut pcStack1322: *mut c_char;
    let mut uStack1320: u16;
    let mut cStack1306: u8;
    let mut acStack1305: [u8; 0x101] = [0; 0x101];
    let mut uStack1048: u16;
    let mut local_416: [u8; 0x8] = [0; 0x8];
    let mut uStack1038: u16;
    let mut local_40c: [u8; 0x102] = [0; 0x102];
    let mut uStack778: u32;
    let mut paStack774: *mut astruct_487;
    let mut local_302: [u8; 0x100] = [0; 0x100];
    let mut local_202: [u8; 0xff] = [0; 0xff];
    let mut acStack259: [u8; 0x101] = [0; 0x101];

    acStack259[1] = 0;
    local_302[0] = 0;
    local_202[0] = 0;
    paStack774 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2),
        in_stack_0000f720,
        in_stack_0000f844,
        in_stack_0000f84a,
        in_stack_0000f84e,
    );
    uVar8 = (paStack774 >> 0x10);
    iVar2 = paStack774;
    uStack778 = (iVar2 + 0x1a);
    uVar5 = (iVar2 + 0x1c);
    if ((uVar5 | uStack778) == 0) {
        pcStack1646 = *(iVar2 + 0x64);
        uVar5 = (iVar2 + 0x66);
        if ((uVar5 | pcStack1646) != 0) {
            pass1_1008_5784(
                CONCAT22(0x1050, local_67e),
                pcStack1646 & 0xffff | uVar5 << 0x10,
            );
            puVar3 = local_67e;
            pass1_1008_5b12(CONCAT22(0x1050, puVar3));
            pcStack1654 = CONCAT22(uVar5, puVar3);
            if ((uVar5 | puVar3) != 0) {
                uVar1 = (puVar3 + 2);
                uStack778 = uVar1;
                uVar5 = (uVar1 >> 0x10);
                // TODO: goto LAB_1008_3206;
            }
        }
    } else {
        //
        // LAB_1008_3206:
        unk_str_op_1000_3d3e(CONCAT22(0x1050, acStack259 + 1), CONCAT22(uVar5, uStack778));
    }
    pass1_1000_5008(local_40c, &DAT_1050_1050, 0x100);
    uStack1038 = str_op_1000_3da4(CONCAT22(0x1050, local_40c));
    if (local_40c[uStack1038 - 0x1] == '\\') {
        local_40c[uStack1038 - 0x1] = 0;
    }
    uStack1038 = str_op_1000_3da4(CONCAT22(0x1050, acStack259 + 1));
    if (acStack259[uStack1038] == '\\') {
        acStack259[uStack1038] = '\0';
    }
    pass1_1000_4f2e();
    uVar8 = (paStack774 >> 0x10);
    uStack778 = (paStack774 + 0x12);
    uVar5 = (paStack774 + 0x14);
    if ((uVar5 | uStack778) != 0) {
        unk_str_op_1000_3d3e(
            CONCAT22(0x1050, local_202),
            (uStack778 & 0xffff | uVar5 << 0x10),
        );
    }
    local_416[0] = '\0';
    pcVar9 = load_string_1010_847e(_u16_1050_14cc, 0x579);
    unk_str_op_1000_3d3e(CONCAT22(0x1050, local_416), pcVar9);
    uStack1048 = str_op_1000_3da4(CONCAT22(0x1050, local_416));
    uStack1038 = uStack1048;
    //   for (; -0x1 < uStack1048; uStack1048 -= 1)
    while -1 < uStack1048 {
        if (local_416[uStack1048] == '.') {
            unk_str_op_1000_3d3e(
                CONCAT22(0x1050, local_67e),
                CONCAT22(0x1050, local_416 + uStack1048 + 1),
            );
            unk_str_op_1000_3d3e(CONCAT22(0x1050, local_416), CONCAT22(0x1050, local_67e));
        }
        uStack1048 -= 1;
    }
    acStack1305[1] = 0;
    pcVar9 = load_string_1010_847e(_u16_1050_14cc, 0x74c);
    uVar7 = (pcVar9 >> 0x10);
    unk_str_op_1000_3d3e(CONCAT22(0x1050, acStack1305 + 1), pcVar9);
    uStack1038 = str_op_1000_3da4(CONCAT22(0x1050, acStack1305 + 1));
    cStack1306 = acStack1305[uStack1038];
    uStack1048 = 0;
    while (acStack1305[uStack1048 + 0x1] != '\0') {
        if (acStack1305[uStack1048 + 0x1] == cStack1306) {
            acStack1305[uStack1048 + 0x1] = '\0';
        }
        uStack1048 += 0x1;
    }
    pass1_1000_4906(CONCAT22(0x1050, &ofn), NULL, 0x48);
    _ofn = 0x48;
    uVar8 = (param_2 >> 0x10);
    uStack1374 = (param_2 + 0x8);
    pcStack1370 = acStack1305 + 1;
    uStack1368 = SUB42(&DAT_1050_1050, 0x0);
    pcStack1354 = CONCAT22(0x1050, local_202);
    puStack1346 = local_302;
    uStack1344 = SUB42(&DAT_1050_1050, 0x0);
    uStack1350 = 0x100;
    uStack1342 = 0x100;
    pcStack1338 = acStack259 + 1;
    uStack1336 = SUB42(&DAT_1050_1050, 0x0);
    pcStack1322 = local_416;
    uStack1320 = SUB42(&DAT_1050_1050, 0x0);
    pcStack1382 = null_mut();
    local_666[0] = 0;
    if (param_3 == 1) {
        uStack1330 = 0x1804;
        pcVar9 = load_string_1010_847e(_u16_1050_14cc, 0x74d);
        uVar7 = (pcVar9 >> 0x10);
        unk_str_op_1000_3d3e(CONCAT22(0x1050, local_666), pcVar9);
        puStack1334 = local_666;
        uStack1332 = SUB42(&DAT_1050_1050, 0x0);
        success = GetOpenFileName16(CONCAT22(0x1050, &ofn));
    } else {
        if (param_3 != 0x2) {
            debug_print_1008_6048(uVar7, s_Unsupported_FileStructType_in_Op_1050_01ca);
            // TODO: goto LAB_1008_3461;
        }
        uStack1330 = 0x6;
        pcVar9 = load_string_1010_847e(_u16_1050_14cc, 0x74e);
        uVar7 = (pcVar9 >> 0x10);
        unk_str_op_1000_3d3e(CONCAT22(0x1050, local_666), pcVar9);
        puStack1334 = local_666;
        uStack1332 = SUB42(&DAT_1050_1050, 0x0);
        success = GetSaveFileName16(CONCAT22(0x1050, &ofn));
    }
    if (success != 0) {
        pcStack1382 = pcStack1354;
    } //
      // LAB_1008_3461:
    if (pcStack1382.is_null() == false) {
        local_67e[0] = uStack1326;
        if (uStack1326 < 0x0) {
            pcStack1654 = load_string_1010_847e(_u16_1050_14cc, 0x3fd);
            uVar6 = (pcStack1654 >> 0x10);
            uVar4 = str_op_1008_60e8(uVar6, pcStack1654);
            pcStack1654 = CONCAT22(uVar6, uVar4);
            pcVar9 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
            text = (pcVar9 >> 0x10);
            atype = SUB42(pcVar9, 0x0);
            MessageBox16(0x10, pcVar9, pcStack1654, (param_2 + 0x8));
            pcStack1382 = null_mut();
            pcStack1646 = pcStack1654;
            fn_ptr_1000_17ce(pcStack1654);
        } else {
            str_op_1000_3dbe(
                CONCAT13(0x10, CONCAT12(0x50, local_782)),
                pcStack1354,
                uStack1326,
            );
            local_782[uStack1326] = '\0';
            if (local_782[0] != '\0') {
                pass1_1010_60cc(uVar7, paStack774, CONCAT22(0x1050, local_782));
            }
        }
    }
    pass1_1000_4f2e();
    return;
}


pub unsafe fn msg_box_op_1010_8bb4(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut pcVar1: *mut c_char;
    let mut local_402: [u8; 0x400] = [0; 0x400];

    pcVar1 = load_string_1010_847e(_u16_1050_14cc, 0x3fa);
    unk_str_op_1000_3d3e(CONCAT22(0x1050, local_402), pcVar1);
    pass1_1000_3cea(CONCAT22(0x1050, local_402), param_3);
    pcVar1 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
    MessageBox16(
        0x1010,
        pcVar1,
        CONCAT22(0x1050, local_402),
        HWND16_1050_0396,
    );
    PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
    return;
}


pub unsafe fn msg_box_op_1038_81be(param_1: *mut c_char, mut param_2: u16, param_3: *mut Struct903)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  uVar2 = (param_3 >> 0x10);
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub unsafe fn msg_box_op_1038_8dda(param_1: *mut c_char, mut param_2: u16, param_3: *mut Struct903)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  uVar2 = (param_3 >> 0x10);
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub unsafe fn msg_box_ui_op_1040_64ca(param_1: *mut c_char, mut param_2: u16, mut param_3: u32)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub unsafe fn msg_box_op_1040_a85a(param_1: *mut c_char, mut param_2: u16, mut param_3: u32)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub unsafe fn win_ui_cursor_op_1008_06c0(
    param_1: u32,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: u16;
    let mut in_EDX: u32;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_AF: u8;
    let mut pcVar6: *mut c_char;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000fe44: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6e: u16;
    let mut in_stack_0000ff72: u16;
    let mut iVar8: i16;
    let mut in_stack_0000ff9c: u16;
    let mut local_5a: [u8; 0x50] = [0; 0x50];
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut HStack6: HCURSOR16;
    let mut HStack4: HCURSOR16;

    if (param_4 == 0x400) {
        pass1_1030_8344(_u16_1050_5748, 0x4000001);
        in_AX = in_EDX | in_AX;
        paVar2 = (in_EDX & 0xffff0000 | in_AX);
        if (in_AX != 0) {
            iVar4 = param_1;
            uVar5 = (param_1 >> 0x10);
            if (PTR_LOOP_1050_4fe8.is_null() == false) {
                pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
                MessageBox16(
                    0x10,
                    pcVar6,
                    s_You_may_not_run_a_turn__The_game_1050_00df,
                    (iVar4 + 0x8),
                );
                return;
            }
            HStack4 = LoadCursor16(0x7f02, 0x0);
            HStack6 = SetCursor16(HStack4);
            pass1_1030_83ba(in_AF, _u16_1050_5748, param_2);
            (_u16_1050_5748 + 0x8) = 0x1;
            puVar7 = mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000ff9c, 0x29),
                in_stack_0000fe44,
                in_stack_0000ff68,
                in_stack_0000ff6e,
                in_stack_0000ff72,
            );
            uVar3 = (paVar2 >> 0x10);
            uStack10 = SUB42(puVar7, 0x0);
            uStack8 = (puVar7 >> 0x10);
            pass1_1018_262e(puVar7);
            pass1_1030_8326();
            pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x5dc);
            paVar2 = CONCAT22(uVar3, (pcVar6 >> 0x10));
            sys_1000_3f9c(
                CONCAT13(0x10, CONCAT12(0x50, local_5a)),
                s__s__ld_1050_0109,
                pcVar6,
            );
            ppcVar1 = (*param_1 + 0x14);
            iVar8 = iVar4;
            (**ppcVar1)(
                0x1000,
                iVar4,
                (param_1 >> 0x10),
                0x0,
                local_5a,
                &DAT_1050_1050,
            );
            puVar7 = mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                CONCAT22(iVar8, 0x37),
                in_stack_0000fe3a,
                in_stack_0000ff5e,
                in_stack_0000ff64,
                in_stack_0000ff68,
            );
            pass1_1008_a9ec(puVar7);
            SetCursor16(HStack6);
            PostMessage16(0x0, 0xfc, 0x111, (iVar4 + 0x8));
        }
    }
    return;
}


pub unsafe fn message_box_op_1038_c672(param_1: u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
  let mut uVar1: u16;
  let mut local_404: [u8;0x402] = [0;0x402];

  uVar1 = (_u16_1050_14cc >> 0x10);
  if (param_5 == 0x17d) {
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,&DAT_1050_1050);
    MessageBox16(0x30,*(param_2 + 0x92),CONCAT22(0x1050,local_404),(param_2 + 0x6));
  }
  else {
    if (param_5 != 0x17e) {
      post_win_msg_1040_7b3c(CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,&DAT_1050_1050);
    MessageBox16(0x30,*(param_2 + 0x92),CONCAT22(0x1050,local_404),(param_2 + 0x6));
    pass1_1008_e164((param_2 + 0x8e));
  }
  PostMessage16(0x0,0x2,0x111,(param_2 + 0x6));
  return;
}

pub unsafe fn msg_box_op_1038_c07a(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut local_70c: [u8; 0x200] = [0; 0x200];
    let mut local_50c: [u8; 0x100] = [0; 0x100];
    let mut local_40c: [u8; 0x402] = [0; 0x402];
    let mut uStack10: u32;
    let mut uStack6: u32;

    send_msg_1038_c228(CONCAT22(param_2, param_1));
    uStack6 = load_string_1010_847e(_u16_1050_14cc, 0x531);
    if (param_4 == 0x177) {
        pass1_1008_e05e((param_1 + 0x8e), 0x2, CONCAT22(param_2, param_1 + 0x19e),
                        CONCAT22(param_2, param_1 + 0x9e));
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x200, local_40c, &DAT_1050_1050);
        sys_1000_3f9c(CONCAT22(0x1050, local_70c), CONCAT22(0x1050, local_40c), param_1 + 0x19e);
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x100, local_50c, &DAT_1050_1050);
        MessageBox16(0x30, CONCAT22(0x1050, local_50c), CONCAT22(0x1050, local_70c), (param_1 + 0x6));
    } else {
        if (param_4 != 0x178) {
            if ((param_4 != 0x178) && (param_4 - 0x179 < 0x2)) {
                set_win_pos_1038_c31a(CONCAT22(param_2, param_1), param_3, param_4);
                return;
            }
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4);
            return;
        }
        uStack10 = CONCAT22(param_2, param_1 + 0x9e);
        uVar2 = param_2;
        iVar1 = pass1_1008_e10c((param_1 + 0x8e), CONCAT22(param_2, param_1 + 0x19e),
                                CONCAT22(param_2, param_1 + 0x9e), param_2, &DAT_1050_1050);
        if (iVar1 == 0) {
            load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_40c, &DAT_1050_1050);
            load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_50c, &DAT_1050_1050);
            MessageBox16(0x30, CONCAT22(0x1050, local_50c), CONCAT22(0x1050, local_40c), (param_1 + 0x6),
            );
            return;
        }
        pass1_1008_e01c((param_1 + 0x8e), CONCAT22(param_2, param_1 + 0x19e), uStack10);
        pass1_1038_af40(param_1, uVar2, _PTR_LOOP_1050_5b7c, (param_1 + 0x8), 0x1f);
    }
    PostMessage16(0x0, 0x2, 0x111, (param_1 + 0x6));
    return;
}


pub unsafe fn win_ui_op_1040_07dc(
    mut param_1: u16,
    pstruct_c_param_2: *mut StructC,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut IVar2: i16;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut puVar8: *mut u32;
    let mut in_stack_0000f69a: u16;
    let mut in_stack_0000f7be: u16;
    let mut in_stack_0000f7c4: u16;
    let mut in_stack_0000f7c8: u16;
    let mut BVar9: bool;
    let mut in_stack_0000f7f2: u16;
    let mut uStack2060: u32;
    let mut local_806: [u8; 0x400] = [0; 0x400];
    let mut local_406: [u32; 0x100] = [0; 0x100];
    let mut uStack6: u32;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    uStack6 = 0;
    if (param_5 == 0x73) {
        enable_window_1040_0acc(pstruct_c_param_2, 0x0);
        puVar4 = paVar6;
        puVar3 = pass1_1008_5fd8(puVar4);
        uStack2060 = CONCAT22(puVar4, puVar3);
        puVar5 = puVar4;
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_806,
            &DAT_1050_1050,
        );
        IVar2 = MessageBox16(
            0x34,
            CONCAT13(0x10, CONCAT12(0x50, local_806)),
            CONCAT22(puVar4, puVar3),
            HWND16_1050_0396,
        );
        local_406[0] = uStack2060;
        fn_ptr_1000_17ce(CONCAT22(puVar4, puVar3));
        if (IVar2 == 0x6) {
            PostMessage16(0x0, 0xcb, 0x111, HWND16_1050_0396);
            BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, 1);
            uStack6 = CONCAT22(puVar5, BVar9);
        }
    } else {
        if (param_5 < 0x74) {
            if (param_5 == 0x6e) {
                (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
                puVar8 = pass1_1038_af40(
                    pstruct_c_param_2,
                    param_1,
                    _PTR_LOOP_1050_5b7c,
                    (pstruct_c_param_2 + 0x6),
                    0x2,
                );
                ppcVar1 = (*puVar8 + 0x3c);
                (**ppcVar1)(&u16_1050_1038, puVar8, (puVar8 >> 0x10));
                SetFocus16((pstruct_c_param_2 + 0x6));
                return;
            }
            if (0x6e < param_5) {
                //
                // LAB_1040_09f9:
                post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, param_5);
                return;
            }
            if (param_5 == '\x02') {
                //
                // LAB_1040_09b4:
                post_win_msg_1040_7b3c(pstruct_c_param_2, 0x0, 0x0, 0x2);
                PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
                return;
            }
            //      if (param_5 != 'd') goto LAB_1040_09f9;
            PostMessage16(0x0, 0x64, 0x111, HWND16_1050_0396);
            BVar9 = 0;
            // TODO: goto LAB_1040_0821;
        }
        if (param_5 != 0x74) {
            //      if (param_5 == 0xee) goto LAB_1040_09b4;
            if (param_5 == 0x13d) {
                enable_window_1040_0acc(pstruct_c_param_2, 1);
                return;
            }
            // TODO: goto LAB_1040_09f9;
        }
        enable_window_1040_0acc(pstruct_c_param_2, 0x0);
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_406,
            &DAT_1050_1050,
        );
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_806,
            &DAT_1050_1050,
        );
        IVar2 = MessageBox16(
            0x34,
            CONCAT13(0x10, CONCAT12(0x50, local_406)),
            CONCAT22(0x1050, local_806),
            HWND16_1050_0396,
        );
        if (IVar2 == 0x6) {
            PostMessage16(0x0, 0x7a, 0x111, HWND16_1050_0396);
            BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, 1);
            uStack6 = CONCAT22(paVar6, BVar9);
            puVar7 = mixed_1010_20ba(
                paVar6,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000f7f2, 0x2),
                in_stack_0000f69a,
                in_stack_0000f7be,
                in_stack_0000f7c4,
                in_stack_0000f7c8,
            );
            pass1_1010_60fa(puVar7);
        }
    }
    BVar9 = 0x1; //
    // LAB_1040_0821:
    enable_window_1040_0acc(pstruct_c_param_2, BVar9);
    return;
}


pub unsafe fn message_box_op_1040_37f0(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut uVar4: u16;
    let mut LVar5: LRESULT;
    let mut in_stack_0000fa94: u16;
    let mut in_stack_0000fbb8: u16;
    let mut in_stack_0000fbbe: u16;
    let mut in_stack_0000fbc2: u16;
    let mut in_stack_0000fbec: u16;
    let mut iVar6: i16;
    let mut local_40c: [u8; 0x402] = [0; 0x402];
    let mut pcStack10: *mut c_char;
    let mut paStack6: *mut Struct27;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 == 0x193) {
        paStack6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fbec, 0x2), in_stack_0000fa94,
                                   in_stack_0000fbb8, in_stack_0000fbbe, in_stack_0000fbc2);
        uVar2 = (paStack6 >> 0x10);
        pcStack10 = *(paStack6 + 0x68);
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_40c, &DAT_1050_1050);
        uVar1 = MessageBox16(0x30, pcStack10, CONCAT22(0x1050, local_40c), (param_2 + 0x6));
        pass1_1018_3710(uVar1, uVar2, (param_2 + 0x8e));
        PostMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
    } else {
        if (param_5 != 0x194) {
            post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
            return;
        }
        pass1_1038_af40(param_2, param_1, _PTR_LOOP_1050_5b7c, (param_2 + 0x8), 0x21);
        uVar4 = (paVar3 >> 0x10);
        LVar5 = SendMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
        iVar6 = 0x1;
        paStack6 = mixed_1010_20ba(CONCAT22(uVar4, (LVar5 >> 0x10)), _u16_1050_0ed0,
                                   0x1002b, in_stack_0000fa94, in_stack_0000fbb8, in_stack_0000fbbe,
                                   in_stack_0000fbc2);
        pass1_1010_038e(paStack6, iVar6);
    }
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

    dialog_ui_fn_1040_78e2(param_2);
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


pub unsafe fn pass1_1020_6184(mut param_1: u32, mut param_2: u16) {
    let mut HVar1: HCURSOR16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xf4) == 1) {
        HVar1 = SetCursor16((iVar2 + 0xf0));
        (iVar2 + 0xee) = HVar1;
        (iVar2 + 0x10c) = param_2;
        SetCapture16((iVar2 + 0x8));
        (iVar2 + 0xf4) = 0x2;
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


pub unsafe fn set_cursor_1020_5764(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut HVar3: HCURSOR16;
    let mut in_EDX: *mut Struct57;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_e: i16;
    let mut local_c: [u8; 0x2] = [0; 0x2];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    puStack6 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe6, 0x2f),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    uVar5 = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x20);
    uVar1 = (puStack6 + 0x22);
    if ((uVar1 | uStack10) != 0) {
        pass1_1030_8308(
            &local_e,
            uVar1,
            _u16_1050_5748,
            (_u16_1050_5748 >> 0x10),
            CONCAT22(0x1050, &local_e),
            CONCAT22(0x1050, local_c),
            CONCAT13((uVar1 >> 0x8), CONCAT12(uVar1, uStack10)),
        );
        if (param_2 <= local_e) {
            uVar5 = (param_1 >> 0x10);
            iVar4 = param_1;
            if ((iVar4 + 0xf4) != 1) {
                SetCursor16((iVar4 + 0xee));
                (iVar4 + 0xee) = 0;
                (iVar4 + 0xf4) = 0x1;
                (iVar4 + 0x10c) = 0;
                ReleaseCapture16();
            }
            HVar3 = LoadCursor16(0x7f02, 0x0);
            HVar3 = SetCursor16(HVar3);
            pass1_1018_017c(puStack6, param_2);
            uVar2 = (iVar4 + 0xf6);
            (uVar2 + 0x10) = 0x1;
            if ((iVar4 + 0xfe) != 0) {
                pass1_1020_68de((iVar4 + 0xfe));
                uVar2 = (iVar4 + 0xfe);
                PostMessage16(0x0, 0xeb, 0x111, (uVar2 + 0x8));
            }
            SetCursor16(HVar3);
        }
    }
    return;
}


pub unsafe fn win_ui_op_1020_5e76(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
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
    pt_in_rect_1020_5856(puVar2, in_EDX, param_1, CONCAT22(0x1050, puVar2));
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
                &DAT_1050_1050,
            );
            load_string_1010_84e0(
                _u16_1050_14cc,
                (_u16_1050_14cc >> 0x10),
                0x3ff,
                local_1aa,
                &DAT_1050_1050,
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
        puVar9 = &DAT_1050_1050;
    }
    ppcVar1 = (*puVar9 + 0x6c);
    (**ppcVar1)(0x1008, puVar9, uVar12, pcVar17); //
    // LAB_1020_6176:
    (iVar8 + 0x10c) = 0;
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


pub unsafe fn send_dlg_item_int_1038_94da(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> BOOL16

{
  let mut pUVar1: *mut u16;
  let mut iVar2: i16;
  let mut hwnd: HWND16;
  let mut lVar3: i32;
  let mut local_c: bool;
  let mut uStack10: u16;
  let mut iStack8: i16;
  let mut UStack6: u16;
  let mut iStack4: i16;

  iStack4 = 0x1;
  iStack8 = pass1_1038_993a(param_1,param_2,param_3);
  if ((-0x1 < iStack8) &&
     (UStack6 = GetDlgItemInt16(0x1,&local_c,&DAT_1050_1050,(iStack8 * 0xe + 0x5a72)), local_c != 0)
     ) {
    if (param_5 == 0) {
      UStack6 += 0x1;
    }
    else {
      iStack4 = -0x1;
      UStack6 -= 1;
    }
    uStack10 = (UStack6 <= (iStack8 * 0xe + 0x5a7a));
    pUVar1 = (iStack8 * 0xe + 0x5a78);
    if (*pUVar1 != UStack6 && UStack6 <= *pUVar1) {
      uStack10 = 0;
    }
    iVar2 = iStack8 * 0xe;
    hwnd = GetDlgItem16((iVar2 + 0x5a72),(param_1 + 0x6));
    SetFocus16(hwnd);
    if ((uStack10 != 0) &&
       (lVar3 = unk_win_ui_op_1038_9820(CONCAT22(param_2,param_1),0x1,iStack4,iStack8), lVar3 != 0)) {
      SetDlgItemInt16(0x1,UStack6,(iVar2 + 0x5a72),(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x94),0xfa9,(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x96),0xfa8,(param_1 + 0x6));
    }
  }
  return 0x1;
}


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
  dialog_ui_fn_1040_78e2(param_2);
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
  unk_win_ui_op_1038_a18c(uVar3,param_2,in_stack_0000ff9e);
  win_1008_5c7c(uVar1,uVar3,_u16_1050_02a0,0x30001);
  (struct_b_3 + 0x7).field0_0x0 = uVar1;
  ShowWindow16(0x5,struct_b_3.lpvoid_field_0x8);
  return;
}


pub unsafe fn show_win_1038_c044(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
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

    dialog_ui_fn_1040_78e2(struct_b_param_1);
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
    dialog_ui_fn_1040_78e2(struct_b_param_1);
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
        uVar7 = SUB42(&DAT_1050_1050, 0x0);
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

    dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn show_win_1040_3ae8(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog_ui_fn_1040_78e2(param_1);
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

pub unsafe fn win_ui_op_1020_36f6(mut param_1: u32, mut param_2: i16) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut pcVar4: *mut c_char;
    let mut uVar5: u16;
    let mut HVar6: HWND16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iVar9: i16;
    let mut uVar10: u16;
    let mut pcVar11: *mut c_char;
    let mut uVar12: u8;
    let mut uVar13: u8;
    let mut puStack1034: *mut u8;
    let mut local_406: [u8; 0x400] = [0; 0x400];
    let mut uStack6: u32;

    iVar9 = param_1;
    uVar10 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar9 + 0x8) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    uStack6 = pass1_1018_1e78((iVar9 + 0x8), -1);
    uVar7 = (uStack6 >> 0x10);
    uVar3 = (iVar9 + 0x18);
    GetWindowText16(0x3ff, CONCAT22(0x1050, local_406), (uVar3 + 0x6));
    pcVar4 = pass1_1000_472c(CONCAT22(0x1050, local_406), ':');
    puStack1034 = CONCAT22(uVar7, pcVar4 + 2);
    *puStack1034 = 0;
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x3ff,
        local_406,
        &DAT_1050_1050,
    );
    uVar3 = (iVar9 + 0x18);
    ppcVar2 = ((iVar9 + 0x18) + 0x18);
    (**ppcVar2)(0x1010, uVar3, (uVar3 >> 0x10), local_406, &DAT_1050_1050);
    uVar3 = (iVar9 + 0x8);
    iVar1 = (uVar3 + 0x4a);
    uVar3 = (iVar9 + 0x18);
    HVar6 = (uVar3 + 0x6);
    SetDlgItemText16((uStack6 + 0x2), 0x10f, HVar6);
    SetDlgItemText16((uStack6 + 0xa), 0x110, HVar6);
    SetDlgItemText16((uStack6 + 0x12), 0x112, HVar6);
    SetDlgItemText16((uStack6 + 0xe), 0x113, HVar6);
    if (iVar1 != 0) {
        uVar5 = pass1_1018_1f1a((iVar9 + 0x8), (uStack6 + 0x1a));
        if (uVar5 != 0) {
            uVar12 = 0x11;
            uVar13 = 0x1;
            uVar3 = (uStack6 + 0x16);
            uVar7 = uVar3;
            uVar8 = (uVar3 >> 0x10);
            // TODO: goto LAB_1020_3846;
        }
    }
    uVar12 = 0x11;
    uVar13 = 0x1;
    pcVar11 = load_string_1010_847e(_u16_1050_14cc, 0x5d9);
    uVar8 = (pcVar11 >> 0x10);
    uVar7 = SUB42(pcVar11, 0x0); //
    // LAB_1020_3846:
    SetDlgItemText16(CONCAT22(uVar8, uVar7), CONCAT11(uVar13, uVar12), HVar6);
    if ((iVar9 + 0x1c) != 0) {
        uVar3 = (iVar9 + 0x1c);
        HVar6 = GetDlgItem16((uStack6 + 0x1a), (uVar3 + 0x6));
        SetFocus16(HVar6);
        return;
    }
    InvalidateRect16(0x0, NULL, 0x0);
    return;
}

pub unsafe fn win_ui_get_prop_op_1040_9566(param_1: *mut i16)

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
  if (*param_1 == 0x4) {
    uVar1 = (iVar6 + 0xc);
    HVar4 = GetProp16(s_thisHi_1050_5e6f,(iVar6 + 0xa));
    HVar5 = GetProp16(s_thisLo_1050_5e68,(iVar6 + 0xa));
    if ((HVar4 | HVar5) != 0) {
      iVar2 = (iVar6 + 0x6);
      if (iVar2 == 1) {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0xc);
        (**ppcVar3)(s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x2) {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0x10);
        (**ppcVar3)(s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x4) {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0x18);
        (**ppcVar3)(s_tile2_bmp_1050_1538,uVar1,HVar4,*(iVar6 + 0x8) & 0x10);
        return;
      }
    }
  }
  return;
}

pub unsafe fn call_win_proc_1040_9684(win_handle_1: HWND16, mut param_2: u16, w_param_1: WPARAM16, l_param_1: LPARAM)

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
        (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_2,handle_1,1);
      }
    }
    else if (l_param_1 == 0x201) {
      handle_1 = GetProp16(s_IsDlg_1050_5e92,l_param_1);
      if (handle_1 == 0) {
        uVar2 = (var10 + 0x18);
        GetClientRect16(local_1a,&DAT_1050_1050);
        bool_1 = PtInRect16(CONCAT22(param_2,win_handle_1),local_1a);
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
      GetClientRect16(local_1a,&DAT_1050_1050);
      bool_1 = PtInRect16(CONCAT22(param_2,win_handle_1),local_1a);
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


pub unsafe fn dialog_ui_fn_1040_78e2(in_struct_1: *mut StructB)

{
  let mut puVar1: *mut u8;
  let mut dialog_handle: LPVOID = null_mut();
  let mut uVar2: u16;
   let mut struct_b_1: *mut StructB;
   let mut local_string_1: *mut StructB;
  let mut uVar3: u16;
  let mut lVar4: i32;
  let mut local_string_2: HANDLE16;
  let mut HStack8: HANDLE16;
// pub unsafe fn *pvStack6;
  let mut fn_ptr_1: *mut *mut code;

  local_string_1 = (in_struct_1 >> 0x10);
  struct_b_1 = in_struct_1;
  if (&struct_b_1.field6_0xc == 0) {
    uVar3 = (_u16_1050_5bc8 >> 0x10);
    puVar1 = (_u16_1050_5bc8 + 0x4);
    uVar2 = (_u16_1050_5bc8 + 0x6);
  }
  else {
    puVar1 = struct_b_1.field6_0xc;
    uVar2 = struct_b_1.field7_0xe;
  }
  pvStack6 = CONCAT22(uVar2,puVar1);
  dialog_handle =
       CreateDialog16(pvStack6,struct_b_1.max_count_field_0x10,ZEXT24(struct_b_1.field5_0xa),
                              HINSTANCE16_1050_038c);
  struct_b_1.lpvoid_field_0x8 = dialog_handle;
  GetWindowText16(0x50,in_struct_1 & 0xffff0000 | ZEXT24(&struct_b_1.field8_0x10),dialog_handle);
  lVar4 = GetWindowLong16(-0x4,struct_b_1.lpvoid_field_0x8);
  SetWindowLong16(_u16_1050_5bcc,-0x4,struct_b_1.lpvoid_field_0x8);
  SetProp16(struct_b_1,s_thisLo_1050_5dcd,struct_b_1.lpvoid_field_0x8);
  SetProp16(local_string_1,s_thisHi_1050_5dd4,struct_b_1.lpvoid_field_0x8);
  local_string_2 = lVar4;
  SetProp16(local_string_2,s_procLo_1050_5ddb,struct_b_1.lpvoid_field_0x8);
  HStack8 = (lVar4 >> 0x10);
  SetProp16(HStack8,s_procHi_1050_5de2,struct_b_1.lpvoid_field_0x8);
  fn_ptr_1 = (in_struct_1 + 0x50);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,in_struct_1);
  return;
}


pub unsafe fn menu_ui_op_1020_5bf2(
    param_1: *mut astruct_52,
    param_2: INT16,
    param_3: INT16,
) -> BOOL16 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut pIVar4: *mut INT16 = null_mut();
    //   HMENlet mut HVar5: u16;
    let mut HVar5: HMENU16;
    let mut in_DX: u16;
    let mut iVar5: *mut astruct_52;
    let mut uVar6: u16;
    let mut local_10: i16;
    let mut IStack14: i16;
    let mut iStack12: i16;
    let mut uStack10: u32;
    let mut local_6: i16;
    let mut IStack4: i16;

    local_6 = param_3;
    IStack4 = param_2;
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    uVar2 = pass1_1020_64d4(iVar5.field243_0xf6, 0x2);
    if (uVar2 != 0) {
        uStack10 = pass1_1020_6498(iVar5.field243_0xf6, 0x2);
        in_DX = (uStack10 >> 0x10);
        BVar3 = PtInRect16(CONCAT22(IStack4, local_6), uStack10);
        if (BVar3 != 0) {
            PostMessage16(0x0, 0x131, 0x111, HWND16_1050_0396);
            return 0x1;
        }
    }
    uVar2 = pass1_1020_64d4(iVar5.field243_0xf6, 0x3);
    if (uVar2 == 0) {
        return 0x0;
    }
    pIVar4 = &local_6;
    pt_in_rect_1020_5856(pIVar4, in_DX, param_1, CONCAT22(0x1050, pIVar4));
    iVar5.field257_0x108 = pIVar4;
    iVar5.field258_0x10a = in_DX;
    if ((in_DX | iVar5.field257_0x108) == 0) {
        return 0x0;
    }
    if (iVar5.field256_0x106 == 0) {
        HVar5 = LoadMenu16(s_TILEMENU_1050_43f0, HINSTANCE16_1050_038c);
        iVar5.field256_0x106 = HVar5;
        if (HVar5 == 0) {
            return 0x1;
        }
        HVar5 = GetSubMenu16(0x0, iVar5.field256_0x106);
        iVar5.field256_0x106 = HVar5;
        if (HVar5 == 0) {
            return 0x1;
        }
    }
    uVar1 = &iVar5.field257_0x108;
    uStack10 = (uVar1 + 0x2e);
    iStack12 = 0;
    if (uStack10 == 0x42) {
        iStack12 = 0xc9;
    } else if ((s_VrMode_1050_42ca + 0x8 + uStack10 * 0x2) == 0) {
        iStack12 = 0xc8;
    }
    if (iStack12 != 0) {
        win_1008_5c7c(uStack10, in_DX, _u16_1050_02a0, CONCAT22(iStack12, 1));
    }
    local_10 = param_3;
    IStack14 = param_2;
    ClientToScreen16(CONCAT22(0x1050, &local_10), iVar5.field8_0x8);
    TrackPopupMenu16(
        NULL,
        iVar5.field8_0x8,
        0x0,
        IStack14,
        local_10,
        0x0,
        iVar5.field256_0x106,
    );
    return 0x1;
}


pub unsafe fn menu_ui_op_1008_09ba(param_1: *mut astruct_853, param_2: HWND16, param_3: *mut RECT16) {
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_853;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field235_0xec == 0) {
        HVar1 = LoadMenu16(s_OPPOPMENU_1050_0150, HINSTANCE16_1050_038c);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field235_0xec);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field8_0x8);
    HVar1 = iVar2.field235_0xec;
    TrackPopupMenu16(NULL, HWND16_1050_0396, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}


pub unsafe fn menu_ui_op_1040_7f86(param_1: *mut astruct_855)

{
//   HMENlet mut HVar1: u16;
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_855;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2.field104_0x6a.is_null() == false) && (iVar2.field103_0x68 == 0)) {
        HVar1 = LoadMenu16(iVar2.field104_0x6a, HINSTANCE16_1050_038c);
        iVar2.field103_0x68 = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field103_0x68);
        iVar2.field103_0x68 = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field6_0x6);
    HVar1 = iVar2.field103_0x68;
    TrackPopupMenu16(NULL, iVar2.field6_0x6, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}


pub unsafe fn unk_win_ui_op_1040_8158(param_1: u32, POparam_2: INT16, mut param_3: i16)

{
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut iVar3: i16;
    let mut uVar4: u16;

    if (param_3 == 0x2) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        if ((iVar3 + 0x76) != 0) {
            ScreenToClient16(CONCAT22(0x1050, &stack0xfffa), (iVar3 + 0x6));
            GetSystemMetrics16(SM_CYCAPTION);
            BVar2 = PtInRect16((param_1 & 0xffff0000 | ZEXT24((iVar3 + 0x82))),
                               (iVar3 + 0x82));
            if (BVar2 != 0) {
                ppcVar1 = (*param_1 + 0x14);
                (**ppcVar1)(s_tile2_bmp_1050_1538, iVar3);
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
                (**ppcVar1)(s_tile2_bmp_1050_1538, puVar5, (puVar5 >> 0x10), 1);
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
            GetClientRect16(&local_a, &DAT_1050_1050);
            uStack4 = 0x9a;
            InvalidateRect16(0x0, &local_a, &DAT_1050_1050);
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
        draw_ops::unk_draw_op_1020_3da4(uVar7, param_3, CONCAT22(paVar9, paVar3), param_2);
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
        get_sys_metrics_1040_7728(
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
  move_win_1040_826c(param_2,local_10,local_e);
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
    enable_win_1038_c294(param_1);
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
    enable_win_1040_32a8(param_1);
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
  dialog_ui_fn_1040_78e2(param_2);
  if (PTR_LOOP_1050_5ef8 == (&u32_1050_0004 + 1)) {
    PTR_LOOP_1050_5ef8 = null_mut();
  }
  pcVar5 = CONCAT22(0x1050,&local_4);
  piVar3 = &local_6;
  uVar4 = SUB42(&DAT_1050_1050,0x0);
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
  (**ppcVar1)(s_tile2_bmp_1050_1538,param_2);
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
    pass1_1010_5fb0((param_1 + 0x8e), 0x0, &local_a, &DAT_1050_1050, 0x7);
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
    InvalidateRect16(0x0, &local_16, &DAT_1050_1050);
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
                GetClientRect16(&local_a, &DAT_1050_1050);
                puVar2 = param_2.field148_0x98;
                local_a.y += 0x3;
                local_a.x = (puVar2 + 0x1a) - 0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
                unk_destroy_win_op_1010_2fa0(param_2.field148_0x98);
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
                    if (uVar3 != 0) {
                        return;
                    }
                    unk_win_op_1010_7300(uVar9, (puVar10 & 0xffff0000 | uVar15), 0x0, 0x13, param_2.field169_0xb0);
                    return;
                }
                if (param_5 != 0xbbb) {
                    if (param_5 == 0xbbc) {
                        puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x3),
                                                  in_stack_0000fe86, in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                        uVar12 = (puVar10 >> 0x10);
                        uVar4 = puVar10;
                        uVar7 = uVar12;
                        uVar5 = pass1_1010_a5ac(uVar4, uVar12, param_2.field169_0xb0);
                        uVar6 = uVar5;
                        pass1_1010_a58a(uVar5, uVar7, uVar4, uVar12, uVar5);
                        if (uVar6 == 0) {
                            pass1_1010_a568(0x0, uVar7, uVar4, uVar12, uVar5);
                        }
                        hwnd_8 = GetDlgItem16(0xbbc, param_2.hwnd_0x6);
                        EnableWindow16(0x0, hwnd_8);
                        return;
                    }
                    // TODO: goto LAB_1040_5560;
                }
                if ((param_2.field171_0xb6 == 0) || (is_window = IsWindow16(param_2.field171_0xb6), is_window == 0)) {
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
        if ((param_2.hwnd_0xb4 != 0) && (is_window = IsWindow16(param_2.hwnd_0xb4), is_window != 0)) {
            SendMessage16(0x0, 0xeb, 0x111, param_2.hwnd_0xb4);
        }
    }
    hwnd_8 = param_2.hwnd_0x6;//
// LAB_1040_5417:
    DestroyWindow16(hwnd_8);
    return;
}


pub unsafe fn win_ui_op_1040_6d1a(param_1: *mut astruct_897, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut Struct27;
    let mut in_DX: *mut u8;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: *mut astruct_896;
    let mut iVar3: *mut astruct_895;

    match param_4 {
        0xfa => {
            ppcVar1 = (param_1.field144_0x94 + 0x18);
            (**ppcVar1)();
        }
        // break;
        _ => {
            pass1_1040_b54a(in_DX, CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)), param_3,
                            param_4);
            return;
        }
        0xfd => {
            if (DAT_1050_0ecc == 0) {
                return;
            }
            DAT_1050_0ecc = 0;
        }
// TODO: goto LAB_1040_6deb;
        0xfe => {
            if (DAT_1050_0ecc == 1) {
                return;
            }
            DAT_1050_0ecc = 0x1;
        }
// TODO: goto LAB_1040_6deb;
        0xff => {
            if (DAT_1050_0ecc == 0x2) {
                return;
            }
            DAT_1050_0ecc = 0x2;//
// LAB_1040_6deb:
            paVar2 = param_1.field144_0x94;
            ppcVar1 = (param_1.field144_0x94 + 0x10);
            (**ppcVar1)(&PTR_LOOP_1050_1040, paVar2, (paVar2 >> 0x10));
            pass1_1010_2ee2(param_1.field144_0x94);
            PostMessage16(0x0, 0x10a, 0x111, param_1.field6_0x6);
        }
        // break;
        0x107 => {
            iVar3 = null_mut();
        }
// TODO: goto LAB_1040_6e48;
        0x108 => {
            iVar3 = (&PTR_LOOP_1050_0000 + 1);//
// LAB_1040_6e48:
            win_ui_op_1010_3202(param_1.field144_0x94, iVar3);
        }
        // break;
        0x10a => {
            GetClientRect16(&local_a, &DAT_1050_1050);
            paVar2 = param_1.field144_0x94;
            local_a.y += 0x3;
            local_a.x = (paVar2 + 0x1a) - 0x9;
            iStack6 += -0x3;
            iStack4 = iStack4 - 0x3;
            InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
            unk_destroy_win_op_1010_2fa0(param_1.field144_0x94);
            pass1_1010_32c0(param_1.field144_0x94, 0x0);
            pass1_1010_2ee2(param_1.field144_0x94);
        }
        // break;
        0x10c => {
            DestroyWindow16(param_1.field6_0x6);
        }
    }
    return;
}


pub unsafe fn win_ui_op_1040_bbe2(param_1: *mut u8, param_2: HWND16, param_3: *mut astruct_900, mut param_4: u16, mut param_5: u16, mut param_6: u32)

{
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut BVar7: bool;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar7: u16;
    let mut uVar9: u16;
    let mut hwnd: HWND16;
    let mut uVar10: u16;
    let mut uVar13: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut in_register_0000000a: u16;
    let mut paVar14: *mut Struct57;
    let mut uVar15: u32;
    let mut puVar16: *mut u32;
    let mut paVar17: *mut Struct57;
    let mut uVar16: u32;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb4: u16;
    let mut uVar21: u16;
    let mut uStack30: u16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut uVar1: u32;
    let mut puVar4: *mut u32;
    let mut uVar17: u16;
    let mut uVar18: u16;
    let mut uVar19: u16;
    let mut in_stack_0000ffde: u16;
    let mut uVar20: u16;

    paVar14 = CONCAT22(in_register_0000000a, param_1);
    if (param_6 != 0x10c) {
        if (param_6 < 0x10d) {
            if (param_6 == 0xfa) {
                ppcVar3 = (*param_3.field148_0x98 + 0x18);
                (**ppcVar3)();
                return;
            }
            if (param_6 == 0x10a) {
                GetClientRect16(&local_a, &DAT_1050_1050);
                puVar5 = param_3.field148_0x98;
                local_a.y += 0x3;
                local_a.x = (puVar5 + 0x1a) - 0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
                unk_destroy_win_op_1010_2fa0(param_3.field148_0x98);
                pass1_1010_32c0(param_3.field148_0x98, 0x0);
                pass1_1010_2ee2(param_3.field148_0x98);
                return;
            }
            if (param_6 != 0x10b) {//
// LAB_1040_be78:
                pass1_1040_b54a(param_1, CONCAT22(param_4, param_3), param_5, param_6);
                return;
            }
            puVar4 = param_3.field148_0x98;
            uVar2 = (puVar4 + 0x12);
            uVar21 = uVar2;
            puVar16 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(uVar2, 0x3), in_stack_0000fe84,
                                      in_stack_0000ffa8, in_stack_0000ffae, in_stack_0000ffb2);
            uVar8 = (puVar16 >> 0x10);
            uStack30 = puVar16;
            uVar6 = uStack30;
            uVar13 = uVar8;
            pass1_1010_a5ca(uStack30, uVar8, uStack30, uVar8, uVar21);
            if ((uVar2 != 0x70) && (uVar6 == 0)) {
                return;
            }
            uVar1 = param_3.field169_0xb0;
            uVar18 = uVar1;
            uVar19 = (uVar1 >> 0x10);
            puVar5 = param_3.field148_0x98;
            uVar17 = (puVar5 + 0x12);
        } else {
            if (param_6 != 0x10d) {
                if (param_6 == 0x10e) {
                    paVar17 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(param_2, 0x32), in_stack_0000fe86,
                                              in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                    uVar15 = paVar14 & 0xffff0000 | paVar17 >> 0x10;
                    iVar7 = paVar17;
                    ui_op_1010_79aa(paVar17, 0xfc6, param_3.field169_0xb0);
                    if (iVar7 != 0) {
                        return;
                    }
                    unk_win_op_1010_7300(uVar15, paVar17, 0x0, 0x13, param_3.field169_0xb0);
                    return;
                }
                if (param_6 != 0xbbb) {
                    if (param_6 == 0xbbc) {
                        puVar16 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(param_2, 0x3), in_stack_0000fe86,
                                                  in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                        uVar2 = (puVar16 >> 0x10);
                        uVar8 = puVar16;
                        uVar13 = uVar2;
                        uVar7 = pass1_1010_a5ac(uVar8, uVar2, param_3.field169_0xb0);
                        uVar9 = uVar7;
                        pass1_1010_a58a(uVar7, uVar13, uVar8, uVar2, uVar7);
                        if (uVar9 == 0) {
                            pass1_1010_a568(0x0, uVar13, uVar8, uVar2, uVar7);
                        }
                        hwnd = GetDlgItem16(0xbbc, param_3.field6_0x6);
                        EnableWindow16(0x0, hwnd);
                        return;
                    }
                    // TODO: goto LAB_1040_be78;
                }
                if ((param_3.field171_0xb6 == 0) || (BVar7 = IsWindow16(param_3.field171_0xb6), BVar7 == 0)) {
                    uVar16 = pass1_1038_af40(param_3, paVar14, _PTR_LOOP_1050_5b7c, param_3.field6_0x6, 0x1b);
                    param_3.field171_0xb6 = (uVar16 + 0x6);
                    ShowWindow16(0x1, param_3.field171_0xb6);
                    return;
                }
                param_2 = param_3.field171_0xb6;
                // TODO: goto LAB_1040_bd39;
            }
            puVar16 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(param_2, 0x3), in_stack_0000fe86,
                                      in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
            uVar13 = (puVar16 >> 0x10);
            uStack30 = puVar16;
            uVar15 = param_3.field169_0xb0;
            uVar18 = uVar15;
            uVar19 = (uVar15 >> 0x10);
            uVar17 = 0x71;
            uVar8 = uVar13;
        }
        pass1_1010_a5ec(uVar13, uStack30, uVar8, uVar17, CONCAT22(uVar19, uVar18));
        if ((param_3.field170_0xb4 != 0) && (BVar7 = IsWindow16(param_3.field170_0xb4), BVar7 != 0)) {
            SendMessage16(0x0, 0xeb, 0x111, param_3.field170_0xb4);
        }
    }
    param_2 = param_3.field6_0x6;//
// LAB_1040_bd39:
    DestroyWindow16(param_2);
    return;
}

pub unsafe fn invalidate_rect_1040_c028(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut uVar10: u16;
    let mut erase: *mut RECT16;
    let mut rect: *mut RECT16;
    let mut hwnd: HWND16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut piVar9: *mut i16;

    iVar8 = param_1;
    uVar10 = (param_1 >> 0x10);
    if (param_2 == 0x8) {
        GetClientRect16(&local_a, &DAT_1050_1050);
        uVar1 = (iVar8 + 0x6);
        uVar3 = (iVar8 + 0x6);
        iVar6 = (uVar3 + 0x16);
        uVar3 = (iVar8 + 0x6);
        local_a.x = (uVar3 + 0x1a);
        uVar3 = (iVar8 + 0x6);
        local_a.y = (uVar3 + 0x1c);
        if (iVar6 != 0) {
            if (iVar6 < 0x2) {
                iVar5 = 0x1;
            } else {
                iVar5 = 0x2;
            }
            uVar2 = ((iVar6 - iVar5) * 0x4 + uVar1 + 0x2a);
            iVar6 = uVar2;
            uVar2 &= 0xffff0000;
            local_a.x = (iVar6 + 0x22) + (uVar2 | iVar6 + 0x1e);
        }
        uVar1 = (iVar8 + 0x6);
        iStack6 = (uVar1 + 0x1e);
        iStack4 += -0x5;
    } else {
        if (param_2 != 0x9) {
            if (param_2 != 0xa) {
                return;
            }
            uVar1 = (iVar8 + 0x6);
            uVar7 = uVar1 + 0x2a;
            if (((iVar8 + 0x8) | uVar7) == 0) {
                return;
            }
            uVar3 = (iVar8 + 0x6);
            uVar2 = (((uVar3 + 0x16) - 1) * 0x4 + uVar7);
            iVar8 = uVar2;
            uVar2 &= 0xffff0000;
            piVar9 = (uVar2 | iVar8 + 0x1e);
            uVar10 = (uVar2 >> 0x10);
            local_a.y = (iVar8 + 0x20) - 0x8;
            local_a.x = *piVar9;
            iStack6 = (iVar8 + 0x22) + *piVar9;
            iStack4 = (iVar8 + 0x20);
            rect = &local_a;
            hwnd = DAT_1050_1050;
            erase = null_mut();
            // TODO: goto LAB_1040_c19d;
        }
        local_a.x = 0;
        local_a.y = 0;
        iStack6 = 0;
        iStack4 = 0;
        GetClientRect16(&local_a, &DAT_1050_1050);
        uVar1 = (iVar8 + 0x6);
        local_a.x = (uVar1 + 0x1a);
        uVar1 = (iVar8 + 0x6);
        iStack6 = (uVar1 + 0x1e);
        iStack4 += -0x5;
        uVar1 = (iVar8 + 0x6);
        uVar3 = (iVar8 + 0x6);
        iVar6 = (uVar3 + 0x16);
        if (0x0 < iVar6) {
            uVar1 = (uVar1 + iVar6 * 0x4 + 0x26);
            iVar6 = uVar1;
            uVar4 = (uVar1 >> 0x10);
            local_a.y = (iVar6 + 0x20) + (iVar6 + 0x24);
        }
    }
    hwnd = (iVar8 + 0x4);
    erase = &local_a;
    rect = &DAT_1050_1050;//
// LAB_1040_c19d:
    InvalidateRect16(erase, rect, hwnd);
    return;
}

pub unsafe fn mixed_sys_op_1018_2978(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_931,
) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut paVar2: *mut astruct_394;
    let rect: *mut RECT16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut paVar7: *mut Struct57;
    let mut iVar9: *mut astruct_931;
    let mut uVar8: u16;
    let mut uVar7: u16;
    let mut uVar9: u16;
    let mut HVar8: HWND16;
    let mut paVar9: *mut astruct_394;
    let mut local_3a: RECT16;
    let mut iStack54: i16;
    let mut iStack52: i16;
    let mut uStack50: u32;
    let mut puStack46: *mut u32;
    let mut local_2a: astruct_394 = astruct_394::default();
    let mut uStack6: u16;
    let mut uStack4: u16;
    let mut uVar10: u8;
    let mut paVar6: *mut Struct57;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    pass1_1010_8096(_u16_1050_14cc, 1);
    uStack4 = SUB42(paVar5, 0x0);
    puVar2 = &local_2a;
    uStack6 = param_1;
    struct_op_1008_48fe(
        paVar5,
        CONCAT13(0x10, CONCAT12(0x50, puVar2)),
        0x1,
        CONCAT22(uStack4, param_1),
    );
    uVar7 = 0x1000;
    mem_op_1000_179c(0x1e, paVar5);
    uVar5 = paVar5 | puVar2;
    paVar7 = (paVar5 & 0xffff0000);
    paVar6 = (paVar7 | uVar5);
    if (uVar5 == 0) {
        paVar2 = null_mut();
    } else {
        paVar2 = &local_2a;
        uVar7 = 0x1008;
        struct_op_1008_3f92(CONCAT22(paVar5, puVar2), CONCAT22(0x1050, paVar2));
        paVar7 = paVar6;
    }
    uVar3 = SUB42(paVar7, 0x0);
    puStack46 = CONCAT22(uVar3, paVar2);
    ppcVar1 = (*puStack46 + 0x14);
    paVar9 = paVar2;
    (**ppcVar1)(uVar7, paVar2, uVar3);
    uStack50 = CONCAT22(paVar7, paVar2);
    mem_op_1000_179c(0x14, paVar7);
    uVar4 = paVar7 | paVar2;
    paVar7 = (paVar7 & 0xffff0000);
    paVar5 = (paVar7 | uVar4);
    if (uVar4 == 0) {
        paVar2 = null_mut();
    } else {
        struct_1008_4c58(paVar2);
        paVar7 = paVar5;
    }
    uVar8 = (param_3 >> 0x10);
    iVar9 = param_3;
    iVar9.field12_0xe = paVar2;
    (iVar9.field12_0xe + 0x2) = paVar7;
    pass1_1008_4d84(paVar7, iVar9.field12_0xe, uStack50);
    rect = &local_3a;
    HVar8 = HWND16_1050_0396;
    GetClientRect16(rect, &DAT_1050_1050);
    uVar9 = 0x1000;
    mem_op_1000_179c(0x1e, paVar7);
    uVar6 = paVar7 | rect;
    if (uVar6 == 0) {
        iVar9.field10_0xa = 0;
    } else {
        iVar4 = (iStack52 - local_3a.y) + 1;
        uVar9 = 0x1008;
        pass1_1008_405c(
            CONCAT22(paVar7, rect),
            iVar9.field12_0xe,
            iVar4,
            (iStack54 - local_3a.x) + 1,
        );
        iVar9.field10_0xa = iVar4;
        iVar9.field11_0xc = uVar6;
    }
    if (puStack46.is_null() == false) {
        ppcVar1 = *puStack46;
        (**ppcVar1)(
            uVar9,
            puStack46,
            (puStack46 >> 0x10),
            0x1,
            HVar8,
            paVar9,
            uVar3,
            puStack46,
            puStack46,
        );
    }
    close_file_1008_496c(CONCAT13(0x10, CONCAT12(0x50, &local_2a)));
    return;
}

pub unsafe fn win_ui_op_1018_5e9a(mut param_1: u16, structb_param_1: *mut StructB) {
    let mut ppcVar1: *mut *mut c_char;
    let mut pvVar2: LPVOID = null_mut();
    let mut IVar3: i16;
    let mut ppaVar4: *mut *mut astruct_92 = null_mut();
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut uVar9: u32;
    let mut structb_9: *mut StructB;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut puVar13: *mut u32;
    let mut in_stack_0000fe5a: u16;
    let mut in_stack_0000ff7e: u16;
    let mut in_stack_0000ff84: u16;
    let mut in_stack_0000ff88: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_28: *mut astruct_92;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut local_e: RECT16;
    let mut iStack8: i16;
    let mut pIStack6: *mut INT16 = null_mut();
    let mut paVar8: *mut Struct57;

    paVar7 = CONCAT22(in_register_0000000a, param_1);
    dialog_ui_fn_1040_78e2(structb_param_1);
    puVar13 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffb2, 0x39),
        in_stack_0000fe5a,
        in_stack_0000ff7e,
        in_stack_0000ff84,
        in_stack_0000ff88,
    );
    paVar7 = (paVar7 & 0xffff0000 | puVar13 >> 0x10);
    pvVar2 = puVar13;
    uVar11 = (structb_param_1 >> 0x10);
    structb_9 = structb_param_1;
    structb_9[0x7].field1_0x2 = pvVar2;
    structb_9[0x7].hwnd_0x6 = (puVar13 >> 0x10);
    mem_op_1000_179c(0x12, paVar7);
    puVar5 = (paVar7 | pvVar2);
    paVar8 = (paVar7 & 0xffff0000 | ZEXT24(puVar5));
    if (puVar5.is_null()) {
        structb_9[0x7].lpvoid_field_0x8 = 0;
    } else {
        pass1_1018_6198(
            puVar5,
            CONCAT22(paVar7, pvVar2),
            structb_param_1,
            structb_9.lpvoid_field_0x8,
        );
        structb_9[0x7].lpvoid_field_0x8 = pvVar2;
        structb_9[0x7].max_count_field_0x10 = paVar8;
    }
    uVar9 = &structb_9[0x7].field1_0x2;
    pIStack6 = (uVar9 & 0xffff0000 | (uVar9 + 0xa));
    GetClientRect16(&local_e, &DAT_1050_1050);
    IVar3 = GetSystemMetrics16(SM_CYCAPTION);
    (pIStack6 + 0x6) = IVar3 + iStack8;
    puVar13 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffb2, 0x48),
        in_stack_0000fe5a,
        in_stack_0000ff7e,
        in_stack_0000ff84,
        in_stack_0000ff88,
    );
    uStack20 = (puVar13 >> 0x10);
    iStack22 = puVar13;
    iStack16 = (iStack22 + 0xa);
    iStack18 = (iStack22 + 0xc);
    uVar12 = (pIStack6 >> 0x10);
    (pIStack6 + 0x2) = (iStack18 - (pIStack6 + 0x6)) / 0x2;
    uVar9 = (iStack16 >> 0xf);
    *pIStack6 = iStack16 / 0x2 + 0x3;
    pass1_1028_dc52(CONCAT22(0x1050, &local_28), 0x1, 0x0, 0x100);
    loop {
        uVar6 = uVar9;
        ppaVar4 = &local_28;
        pass1_1028_e4ec(CONCAT22(0x1050, ppaVar4));
        uVar9 = (uVar6 | ppaVar4);
        if ((uVar6 | ppaVar4) == 0) {
            break;
        }
        ppcVar1 = (ppaVar4 + 0x8);
        if (ppcVar1.is_null() == false) {
            pass1_1000_3cea(
                structb_param_1 & 0xffff0000 | ZEXT24(&structb_9.field8_0x10),
                *ppcVar1,
            );
        }
    }
    uVar12 = (pIStack6 >> 0x10);
    iVar10 = pIStack6;
    SetWindowPos16(
        0x44,
        (iVar10 + 0x6),
        (iVar10 + 0x4),
        (iVar10 + 0x2),
        *pIStack6,
        0x0,
        structb_9.lpvoid_field_0x8,
    );
    return;
}

pub unsafe fn enable_win_1038_806a(mut param_1: u16, param_2: *mut astruct_902)

{
  let mut HVar1: HWND16;
  let mut BVar2: bool;
  let mut iVar3: *mut astruct_902;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut uVar6: u32;

  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  HVar1 = GetDlgItem16(0x1,iVar3.field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x1858,iVar3.field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x1859,iVar3.field6_0x6);
  BVar2 = EnableWindow16(0x0,HVar1);
  uVar4 = pass1_1008_b820(BVar2,param_1,iVar3.field147_0x94);
  if (uVar4 != 0) {
    uVar4 = pass1_1008_b340(iVar3.field147_0x94);
    uVar5 = pass1_1008_b366(iVar3.field147_0x94);
    uVar6 = pass1_1008_b47a(iVar3.field147_0x94);
    if (((uVar4 != 0) && (uVar5 != 0)) && (uVar6 != 0)) {
      HVar1 = GetDlgItem16(0x1,iVar3.field6_0x6);
      EnableWindow16(0x1,HVar1);
      HVar1 = GetDlgItem16(0x1858,iVar3.field6_0x6);
      EnableWindow16(0x1,HVar1);
    }
    if (uVar4 != 0) {
      HVar1 = GetDlgItem16(0x1859,iVar3.field6_0x6);
      EnableWindow16(0x1,HVar1);
    }
  }
  return;
}

pub unsafe fn enable_win_1038_c294(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut pcStack12: *mut c_char;

  uVar1 = param_1 + 0x9e;
  pcStack12 = (param_1 & 0xffff0000 | uVar1);
  uVar3 = param_1;
  pass1_1008_e320((param_1 + 0x8e),(param_1 & 0xffff0000 | (param_1 + 0x19e))
                  ,(param_1 & 0xffff0000 | uVar1));
  SetWindowText16(CONCAT22(uVar3,uVar1),(param_1 + 0x9a));
  uVar2 = pass1_1008_e2a4((param_1 + 0x8e),
                          (param_1 & 0xffff0000 | (param_1 + 0x19e)),pcStack12);
  EnableWindow16(uVar2 & 0x1,(param_1 + 0x96));
  EnableWindow16(uVar2 & 0x2,(param_1 + 0x98));
  return;
}


pub unsafe fn win_ui_op_1040_0558(param_1: *mut StructB, param_2: *mut astruct_915) -> LRESULT {
    let mut hwnd: HWND16;
    let mut iVar2: i16;
    let mut iVar3: *mut StructB;
    let mut uVar3: u16;
    let mut l_param: *mut c_char;
    let mut LVar4: LRESULT;
    let mut uVar5: u16;
    let mut w_param: WPARAM16;
    let mut msg: u16;
    let mut id: i16;
    let mut hwnd_00: LPVOID = null_mut();
    let mut iVar1: *mut astruct_913;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar1 = (param_2 * 0xe);
    hwnd = GetDlgItem16((iVar1 + 0x5c64), iVar3.lpvoid_field_0x8);
    iVar2 = pass1_1010_659a(&iVar3[0x7].field1_0x2, (iVar1 + 0x5c66));
    if ((iVar2 == 0) && ((iVar1 + 0x5c66) != 0xa)) {
        EnableWindow16(0x0, hwnd);
        hwnd_00 = iVar3.lpvoid_field_0x8;
        id = (param_2 * 0xe + 0x5c68);
        uVar5 = 0x531;
    } else {
        EnableWindow16(0x1, hwnd);
        hwnd_00 = iVar3.lpvoid_field_0x8;
        id = (param_2 * 0xe + 0x5c68);
        uVar5 = 0x649;
    }
    msg = 0xc;
    w_param = 0;
    l_param = load_string_1010_847e(_u16_1050_14cc, uVar5);
    LVar4 = SendDlgItemMessage16(l_param, w_param, msg, id, hwnd_00);
    return LVar4;
}


pub unsafe fn enable_window_1040_0acc(param_1: *mut StructC, enable_3: BOOL16) {
    let mut BVar1: bool;
    let mut HVar2: HWND16;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    BVar1 = IsWindow16((iVar3 + 0x6));
    if (BVar1 != 0) {
        HVar2 = GetDlgItem16(0x64, (iVar3 + 0x6));
        BVar1 = IsWindow16(HVar2);
        if (BVar1 != 0) {
            EnableWindow16(enable_3, HVar2);
            HVar2 = GetDlgItem16(0x74, (iVar3 + 0x6));
            EnableWindow16(enable_3, HVar2);
            HVar2 = GetDlgItem16(0x73, (iVar3 + 0x6));
            EnableWindow16(enable_3, HVar2);
            HVar2 = GetDlgItem16(0x6e, (iVar3 + 0x6));
            EnableWindow16(enable_3, HVar2);
            HVar2 = GetDlgItem16(0xee, (iVar3 + 0x6));
            EnableWindow16(enable_3, HVar2);
        }
    }
    return;
}

pub unsafe fn enable_win_1040_32a8(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uStack12: u32;

  uVar1 = param_1 + 0x19a;
  uStack12 = param_1 & 0xffff0000 | uVar1;
  uVar2 = param_1;
  pass1_1018_3a5c((param_1 + 0x96),(param_1 & 0xffff0000 | (param_1 + 0x9a)),
                  (param_1 & 0xffff0000 | uVar1));
  SetWindowText16(CONCAT22(uVar2,uVar1),(param_1 + 0x90));
  BVar1 = string_1018_39d8((param_1 + 0x96),
                           (param_1 & 0xffff0000 | (param_1 + 0x9a)),uStack12);
  EnableWindow16(BVar1 & 0x1,(param_1 + 0x8e));
  return;
}

pub unsafe fn enable_win_1040_86dc(mut param_1: u32)

{
  let mut HVar1: HWND16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  HVar1 = GetDlgItem16(0x1,(param_1 + 0x6));
  if (HVar1 != 0) {
    EnableWindow16(0x1,HVar1);
    HVar1 = GetDlgItem16(0x2,(param_1 + 0x6));
    if (HVar1 != 0) {
      EnableWindow16(0x1,HVar1);
    }
  }
  return;
}

pub unsafe fn enable_win_1040_9234(mut param_1: u32, param_2: BOOL16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x18) != 0) {
    EnableWindow16(param_2,(param_1 + 0x18));
  }
  return;
}

pub unsafe fn enable_win_1038_9a66(param_1: *mut u8, pstruct903_param_2: *mut Struct903, in_b_enable_3: u16, mut param_4: u32)

{
  let mut enable: bool;
  let mut hwnd: HWND16;

  if (param_4 == 0xf8) {
    hwnd = GetDlgItem16(0x17d9,(pstruct903_param_2 + 0x6));
    enable = 0x1;
  }
  else {
    if (param_4 != 0x17d9) {
      pass1_1040_b54a(param_1,pstruct903_param_2,in_b_enable_3,param_4);
      return;
    }
    enable = 0;
    SetWindowPos16(0x6,0x1a0,0x12c,0x0,0x0,0x0,(pstruct903_param_2 + 0x6));
    hwnd = 0;
  }
  EnableWindow16(enable,hwnd);
  return;
}

pub unsafe fn enable_win_1038_a8f8(param_1: *mut StructC, mut param_2: u16, param_3: TwoWords)

{
  let mut hwnd: HWND16;
  let mut enable: bool;

  if (param_3.b_0x2 == 0x116) {
    SendDlgItemMessage16(0x0,0x1,0x401,0x11a,(param_1 + 0x6));
    hwnd = GetDlgItem16(0x11a,(param_1 + 0x6));
    enable = 0;
  }
  else {
    if ((param_3.b_0x2 == 0x116) || (0x2 < param_3.b_0x2 - 0x117)) {
      post_win_msg_1040_7b3c(param_1,param_2,param_3,param_3.b_0x2);
      return;
    }
    hwnd = GetDlgItem16(0x11a,(param_1 + 0x6));
    enable = 0x1;
  }
  EnableWindow16(enable,hwnd);
  return;
}

pub unsafe fn win_ui_op_1038_c89c(struct_b_param_1: *mut StructB)

{
  let mut HVar1: HWND16;
   let mut struct_b_4: *mut StructB;
  let mut uVar3: u16;
  let mut enable: bool;
  let mut iVar1: *mut astruct_910;
  let mut uVar1: u32;
  let mut uVar2: u32;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = (struct_b_param_1 >> 0x10);
  struct_b_4 = struct_b_param_1;
  CheckRadioButton16(0xfac,0xfad,0xfac,struct_b_4.lpvoid_field_0x8);
  uVar1 = &struct_b_4[0x7].field1_0x2;
  (uVar1 + 0xa) = 0x1;
  uVar2 = &struct_b_4[0x7].field1_0x2;
  iVar1 = (uVar2 + 0x12);
  if (iVar1 == &u32_1050_0004) {//
// LAB_1038_c8da:
    HVar1 = GetDlgItem16(0xfce,struct_b_4.lpvoid_field_0x8);
    if (HVar1 != 0) {
      EnableWindow16(0x1,HVar1);
    }
    HVar1 = GetDlgItem16(0x1,struct_b_4.lpvoid_field_0x8);
//    if (HVar1 == 0) goto LAB_1038_c93c;
    enable = 0;
  }
  else {
//    if (((iVar1 -0x5) < 1) || (SBORROW2((iVar1 -0x5),1))) goto LAB_1038_c93c;
    if (iVar1 != &u16_1050_0008 && 0x0 < (iVar1 -0x7)) {
//      if (iVar1 != (&u16_1050_0008 + 1)) goto LAB_1038_c93c;
  // TODO: goto LAB_1038_c8da;
    }
    HVar1 = GetDlgItem16(0xfce,struct_b_4.lpvoid_field_0x8);
//    if (HVar1 == 0) goto LAB_1038_c93c;
    enable = 0x1;
  }
  EnableWindow16(enable,HVar1);//
// LAB_1038_c93c:
  move_win_1040_826c(struct_b_param_1,0xc8,0x0);
  ShowWindow16(0x5,struct_b_4.lpvoid_field_0x8);
  return;
}


pub unsafe fn win_ui_op_1038_d2a2(param_1: *mut Struct57, struct_b_param_1: *mut StructB, mut param_3: u16 )

{
  let mut rect: *mut Struct57;
  let mut iVar1: i16;
  let mut hwnd_2: HWND16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: *mut astruct_912;
   let mut struct_b_6: *mut StructB;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut l_param: *mut c_char;
  let mut LVar8: LRESULT;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe32: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut w_param: WPARAM16;
  let mut msg: u16;
  let mut id: i16;
  let mut uVar9: u16;
  let mut hwnd: LPVOID = null_mut();
  let mut local_16: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut uStack8: u32;
  let mut uStack4: u16;
  let mut paVar5: *mut Struct57;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uStack4 = 0x7;
//   for (uStack10 = 0; struct_b_6 = struct_b_param_1, uVar6 = (struct_b_param_1 >> 0x10),
//       uStack10 < uStack4; uStack10 += 1)
      uStack10 = 0;
      struct_b_6 = struct_b_param_1;
      uVar6 = struct_b_param_1 >> 0x10;
      while uStack10 < uStack4
      {
    iVar5 = (uStack10 * 0xc);
    local_16 = (iVar5 + 0x5c0c);
    uStack20 = (iVar5 + 0x5c0e);
    uStack18 = 0x1;
    uStack16 = 0x1;
    rect = &local_16;
    MapDialogRect16(rect,&DAT_1050_1050);
    mem_op_1000_179c(0x42,param_1);
    uVar3 = param_1 | rect;
    paVar5 = (param_1 & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
      rect = null_mut();
      param_1 = (param_1 & 0xffff0000);
    }
    else {
      pass1_1008_3bd6(paVar5,rect,param_1,0x1,CONCAT22(local_16,uStack20),0x104,0x1020103,
                      CONCAT22(struct_b_6.lpvoid_field_0x8,(iVar5 + 0x5c10)),param_3,in_stack_0000fe2e,
                      in_stack_0000fe32,in_stack_0000ff58,in_stack_0000ff5c,in_stack_0000ff60);
      param_1 = paVar5;
    }
    uStack8 = CONCAT22(param_1,rect);
    if ((uStack10 * 0xc + 0x5c12) == 0) {
      EnableWindow16(0x0,&rect.field11_0x18);
    }
    uStack10 += 1;
  }
  uVar9 = 0x86;
  puVar7 = mixed_1010_20ba(param_1,_u16_1050_0ed0,0x860009,in_stack_0000fe80,in_stack_0000ffa4,
                           in_stack_0000ffaa,in_stack_0000ffae);
  uVar4 = (puVar7 >> 0x10);
  uStack14 = puVar7;
  uStack12 = uVar4;
  iVar1 = pass1_1010_659a(puVar7,uVar9);
  if (iVar1 == 0) {
    hwnd_2 = GetDlgItem16(0x14a,struct_b_6.lpvoid_field_0x8);
    EnableWindow16(0x0,hwnd_2);
    hwnd = struct_b_6.lpvoid_field_0x8;
    msg = 0xc;
    id = 0x144;
    w_param = 0;
    l_param = load_string_1010_847e(_u16_1050_14cc,0x531);
    LVar8 = SendDlgItemMessage16(l_param,w_param,msg,id,hwnd);
    uVar4 = (LVar8 >> 0x10);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  BVar2 = ShowWindow16(0x5,struct_b_6.lpvoid_field_0x8);
  win_1008_5c7c(BVar2,uVar4,_u16_1050_02a0,0x9a0001);
  (struct_b_6 + 0x7).field0_0x0 = BVar2;
  return;
}

pub unsafe fn unk_win_ui_op_1038_d400(mut param_1: u16, param_2: *mut astruct_885, param_3: u8, param_4: u8, mut param_5: u16, mut param_6: u16,
                                      mut param_7: u32)

{
    let mut HVar1: HWND16;
    let mut iVar2: i16;
    let mut uVar2: u16;
    let mut BVar2: bool;
    let mut in_DX: u16;
    let mut in_register_0000000a: u16;
    let mut uVar4: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u32;
    let mut LVar6: LRESULT;
    let mut pcVar7: *mut c_char;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbc: u16;
    let mut WVar8: WPARAM16;
    let mut UVar9: u16;
    let mut IVar10: i16;
    let mut uVar11: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_c: [u8; 0x4] = [0; 0x4];
    let mut WStack8: WPARAM16;
    let mut uStack6: u32;
    let mut paVar3: *mut Struct57;

    uStack6 = 0;
    WStack8 = 0;
    match param_7 {
        0x145 => {
            HVar1 = GetDlgItem16(0x146, param_2.field6_0x6);
            uVar2 = EnableWindow16(0x1, HVar1);
            uStack6 = 0x13f0647;
            uVar11 = 0x1f1;
        }
// TODO: goto LAB_1038_d490;
        0x146 => {
            uStack6 = 0x1400648;
            puVar4 = pass1_1008_941a(CONCAT22(0x1050, local_c), 0x1, 0xc4);
            puVar4 = (puVar4 >> 0x10);
            paVar3 = CONCAT22(in_register_0000000a, puVar4);
            win_1008_5c9e(local_c, puVar4, _u16_1050_02a0, CONCAT22(0x1050, local_c));
            uVar11 = 0x86;
            puVar5 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, 0x860009, in_stack_0000fe8c, in_stack_0000ffb0,
                                     in_stack_0000ffb6, in_stack_0000ffba);
            uVar4 = (paVar3 >> 0x10);
            pass1_1010_6604(puVar5, uVar11);
            HVar1 = GetDlgItem16(0x145, param_2.field6_0x6);
            EnableWindow16(0x0, HVar1);
            HVar1 = param_2.field6_0x6;
            UVar9 = 0xc;
            IVar10 = 0x13f;
            WVar8 = 0;
            pcVar7 = load_string_1010_847e(_u16_1050_14cc, 0x649);
            LVar6 = SendDlgItemMessage16(pcVar7, WVar8, UVar9, IVar10, HVar1);
            paVar3 = CONCAT22(uVar4, (LVar6 >> 0x10));
            HVar1 = GetDlgItem16(0x146, param_2.field6_0x6);
            EnableWindow16(0x0, HVar1);
            iVar2 = pass1_1010_659a(puVar5, 0x86);
            if (iVar2 == 0) {
                HVar1 = GetDlgItem16(0x14a, param_2.field6_0x6);
                uVar4 = (paVar3 >> 0x10);
                EnableWindow16(0x0, HVar1);
                HVar1 = param_2.field6_0x6;
                UVar9 = 0xc;
                IVar10 = 0x144;
                WVar8 = 0;
                pcVar7 = load_string_1010_847e(_u16_1050_14cc, 0x531);
                LVar6 = SendDlgItemMessage16(pcVar7, WVar8, UVar9, IVar10, HVar1);
                paVar3 = CONCAT22(uVar4, (LVar6 >> 0x10));
            }
            puVar5 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe6, 0x2), in_stack_0000fe8e,
                                     in_stack_0000ffb2, in_stack_0000ffb8, in_stack_0000ffbc);
            if ((puVar5 + 0x20) != 0) {
                PostMessage16(0x0, 0xaf, 0x111, HWND16_1050_0396);
            }
        }
        // break;
        0x147 => {
            HVar1 = GetDlgItem16(0x148, param_2.field6_0x6);
            uVar2 = EnableWindow16(0x1, HVar1);
            uStack6 = 0x1410647;
            uVar11 = 0x1f5;
        }
// TODO: goto LAB_1038_d490;
        0x148 => {
            HVar1 = GetDlgItem16(0x149, param_2.field6_0x6);
            uVar2 = EnableWindow16(0x1, HVar1);
            uStack6 = 0x1420647;
            uVar11 = 0x1f2;//
// LAB_1038_d490:
            win_1008_5c5c(uVar2, param_1, _u16_1050_02a0, uVar11);
        }
        // break;
        0x149 => {
            uStack6 = 0x1430648;
            PostMessage16(0x0, 0xb8, 0x111, HWND16_1050_0396);
            DestroyWindow16(param_2.field6_0x6);
        }
        // break;
        0x14a => {
            HVar1 = GetDlgItem16(0x145, param_2.field6_0x6);
            EnableWindow16(0x1, HVar1);
            HVar1 = param_2.field6_0x6;
            UVar9 = 0xc;
            IVar10 = 0x140;
            WVar8 = 0;
            pcVar7 = load_string_1010_847e(_u16_1050_14cc, 0x649);
            SendDlgItemMessage16(pcVar7, WVar8, UVar9, IVar10, HVar1);
        }
        // break;
        0x14b => {
            HVar1 = GetDlgItem16(0x147, param_2.field6_0x6);
            EnableWindow16(0x1, HVar1);
        }
        // break;
        _ => {
            post_win_msg_1040_7b3c(CONCAT22(CONCAT11(param_4, param_3), param_2), param_5, param_6, param_7);
            return;
        }
    };
    if (((uStack6 != 0) && (uStack6 != 0)) && (BVar2 = IsWindow16(param_2.field6_0x6), BVar2 != 0)) {
        HVar1 = param_2.field6_0x6;
        WVar8 = 0;
        UVar9 = 0xc;
        pcVar7 = load_string_1010_847e(_u16_1050_14cc, uStack6);
        SendDlgItemMessage16(pcVar7, WVar8, UVar9, uStack6, HVar1);
    }
    if (WStack8 != 0) {
        PostMessage16(0x0, WStack8, 0x111, HWND16_1050_0396);
    }
    return;
}


pub unsafe fn win_ui_op_1040_0170(
    param_1: u8,
    mut param_2: u16,
    param_3: *mut StructD,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: i16,
) {
    let mut iVar1: i16;
    let mut hwnd_1: HWND16;
    let mut BVar2: bool;
    let mut paVar3: *mut astruct_915;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut unaff_SI: u16;
    let mut uVar6: u32;
    let mut LVar7: LRESULT;
    let mut puVar8: *mut u32;
    let mut l_param: *mut c_char;
    let mut uVar9: u32;
    let mut in_stack_0000fd7c: u16;
    let mut in_stack_0000fd86: u16;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000feaa: u16;
    let mut in_stack_0000feb0: u16;
    let mut in_stack_0000feb4: u16;
    let mut pHVar10: *mut HCURSOR16;
    let mut uVar11: u16;
    let mut uVar12: u8;
    let mut uVar13: u8;
    let mut uVar14: u16;
    let mut w_param: WPARAM16;
    let mut msg: u16;
    let mut id: i16;
    let mut in_stack_0000fedc: u32;
    let mut uVar15: u32;
    let mut local_18: HCURSOR16;
    let mut local_16: u16;
    let mut paStack20: *mut astruct_598;
    let mut paStack16: *mut astruct_915;
    let mut uStack14: u16;
    let mut puStack12: *mut u32;
    let mut paStack8: *mut astruct_915;
    let mut WStack6: WPARAM16;
    let mut iStack4: i16;
    let mut iVar2: *mut astruct_890;
    let mut iVar3: *mut astruct_891;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    iStack4 = 0x8;
    WStack6 = 0;
    match param_6 {
        0x167 => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0;
        }
        // break;
        0x168 => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x1;
        }
        // break;
        0x169 => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x2;
        }
        // break;
        0x16a => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x3;
        }
        // break;
        0x16b => {
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            BVar2 = EnableWindow16(0x0, hwnd_1);
            if ((param_3 + 0x92) != 0x3) {
                win_1008_5c5c(BVar2, paVar5, _u16_1050_02a0, 0x1de);
            }
            if ((param_3 + 0x92) != 0x8) {
                iVar2 = ((param_3 + 0x92) * 0xe);
                WStack6 = (iVar2 + 0x5c6c);
                pass1_1010_6604((param_3 + 0x8e), (iVar2 + 0x5c66));
                (param_3 + 0x92) = 0x8;
            }
            // for (paStack8 = null_mut(); paStack8 < 0x4; paStack8 = paStack8 + 1)

            {
                uVar6 = win_ui_op_1040_0558(param_3, paStack8);
                paVar5 = (paVar5 & 0xffff0000 | uVar6 >> 0x10);
            }
        }
        // TODO: goto LAB_1040_04da;
        0x16c => {
            hwnd_1 = GetDlgItem16(0x16d, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x5;
            (param_3 + 0x94) = 0x5;
        }
        // TODO: goto LAB_1040_04da;
        0x16d => {
            hwnd_1 = GetDlgItem16(0x16d, (param_3 + 0x6));
            BVar2 = EnableWindow16(0x0, hwnd_1);
            win_1008_5c5c(BVar2, paVar5, _u16_1050_02a0, 0x1de);
            uVar11 = (paVar5 >> 0x10);
            if ((param_3 + 0x94) != 0x8) {
                iVar3 = ((param_3 + 0x94) * 0xe);
                WStack6 = (iVar3 + 0x5c6c);
                pass1_1010_6604((param_3 + 0x8e), (iVar3 + 0x5c66));
                (param_3 + 0x94) = 0x8;
            }
            LVar7 = win_ui_op_1040_0558(param_3, (&u32_1050_0004 + 1));
            paVar5 = CONCAT22(uVar11, (LVar7 >> 0x10));
            puStack12 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x39),
                in_stack_0000fd7c,
                in_stack_0000fea0,
                in_stack_0000fea6,
                in_stack_0000feaa,
            );
            paVar3 = (puStack12 + 0x20);
            uVar14 = SUB42(&DAT_1050_1050, 0x0);
            uVar12 = SUB21(&local_16, 0x0);
            uVar13 = (&local_16 >> 0x8);
            pHVar10 = &local_18;
            uVar11 = SUB42(&DAT_1050_1050, 0x0);
            uStack14 = (paVar3 >> 0xf) + 0x200;
            uVar6 = paVar5 & 0xffff0000 | uStack14;
            paStack16 = paVar3;
            paStack8 = paVar3;
            pass1_1030_8344(_u16_1050_5748, CONCAT22(uStack14, paVar3));
            paStack20 = CONCAT22(uVar6, paVar3);
            pass1_1030_2f1a(
                CONCAT22(uVar6, paVar3),
                CONCAT22(uVar11, pHVar10),
                CONCAT22(uVar14, CONCAT11(uVar13, uVar12)),
            );
            paVar5 = (uVar6 & 0xffff0000 | ((local_18 - local_16) >> 0xf));
            local_16 += (local_18 - local_16) / 0x2;
            uVar4 = pass1_1030_2fac(paStack20);
            set_window_text_1018_6086((param_3 + 0x96), uVar4, local_16);
        }
        // TODO: goto LAB_1040_04da;
        0x16e => {
            puVar8 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x39),
                in_stack_0000fd7c,
                in_stack_0000fea0,
                in_stack_0000fea6,
                in_stack_0000feaa,
            );
            paVar5 = (paVar5 & 0xffff0000 | puVar8 >> 0x10);
            iVar1 = (puVar8 + 0x20);
            local_18 = LoadCursor16(0x7f02, 0x0);
            local_16 = SetCursor16(local_18);
            pass1_1030_532e(CONCAT22(0x1050, &stack0xfed6), iVar1 + 0x2000000);
            fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &stack0xfed6));
            pass1_1030_838e(_u16_1050_5748);
            pass1_1030_8334();
            SetCursor16(local_16);
            PostMessage16(0x0, 0x7e, 0x111, HWND16_1050_0396);
            DestroyWindow16((param_3 + 0x6));
        }
        // TODO: goto LAB_1040_04da;
        _ => {
            post_win_msg_1040_7b3c(param_3, param_4, param_5, param_6);
            return;
        }
    };
    (param_3 + 0x92) = iStack4; //
    // LAB_1040_04da:
    uVar11 = (in_stack_0000fedc >> 0x10);
    if (iStack4 != 0x8) {
        uVar15 = in_stack_0000fedc & 0xffff0000 | (param_3 + 0x6);
        id = (iStack4 * 0xe + 0x5c68);
        w_param = 0;
        msg = 0xc;
        l_param = load_string_1010_847e(_u16_1050_14cc, (iStack4 * 0xe + 0x5c6a));
        uVar6 = paVar5 & 0xffff0000;
        uVar9 = SendDlgItemMessage16(l_param, w_param, msg, id, uVar15);
        uVar11 = (uVar15 >> 0x10);
        paVar5 = (uVar6 & 0xffff0000 | uVar9 >> 0x10);
    }
    if ((WStack6 != 0) && (
        puVar8 = mixed_1010_20ba(
            paVar5,
            _u16_1050_0ed0,
            CONCAT22(uVar11, 0x2),
            in_stack_0000fd86,
            in_stack_0000feaa,
            in_stack_0000feb0,
            in_stack_0000feb4,
        ),
        (puVar8 + 0x20) != 0,
    )) {
        PostMessage16(0x0, WStack6, 0x111, HWND16_1050_0396);
    }
    return;
}


pub unsafe fn show_win_1040_2490(struct_b_param_1: *mut StructB)

{
    let mut ppcVar1: *mut *mut code;
    let mut hwnd: HWND16;
    let mut struct_b_4: *mut StructB;
    let mut uVar3: u16;
    let mut piVar2: *mut i16;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar3 = (struct_b_param_1 >> 0x10);
    struct_b_4 = struct_b_param_1;
    hwnd = GetDlgItem16(0xfb1, struct_b_4.lpvoid_field_0x8);
    EnableWindow16(0x0, hwnd);
    ppcVar1 = (*&struct_b_4[0x7].field1_0x2 + 0x10);
    piVar2 = (**ppcVar1)(s_tile2_bmp_1050_1538, &struct_b_4[0x7].field1_0x2);
    piVar2 = (piVar2 >> 0x10);
    move_win_1040_826c(struct_b_param_1, (piVar2 + 0x2) - 0x2, (piVar2 + 0x4) + *piVar2 + 0x3);
    ShowWindow16(0x5, struct_b_4.lpvoid_field_0x8);
    pass1_1018_1c9a(&struct_b_4[0x7].field1_0x2, 0x1a0);
    return;
}


pub unsafe fn win_ui_op_1040_2512(param_1: *mut Struct57, mut param_2: u16, param_3: *mut StructC, mut param_4: u32, mut param_5: u16) -> u32

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_263;
    let mut uVar4: u16;
    let mut UVar4: u16;
    let mut HVar5: HWND16;
    let mut BVar6: bool;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut UVar6: u16;
    let mut uVar7: u16;
    let mut uVar11: u16;
    let mut puVar8: *mut u8;
    let mut uVar12: u16;
    let mut puVar9: *mut u8;
    let mut iVar8: *mut StructC;
    let mut iVar9: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut puVar15: *mut u16;
    let mut uVar10: u32;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_1e: [u8; 0x4] = [0; 0x4];
    let mut uStack26: u16;
    let mut uStack24: u16;
    let mut local_16: [*mut u32; 0x2];
    let mut uStack12: u16;
    let mut puStack10: *mut u32;
    let mut BStack6: bool;
    let mut uStack4: u16;
    let mut piVar1: *mut i16;
    let mut in_stack_0000ffdc: u16;
    let mut uVar13: u32;
    let mut uVar14: u32;
    let mut fn_ptr_21: *mut *mut code;

    BStack6 = 0;
    uStack4 = 0;
    if (param_5 == 0x2) {//
// LAB_1040_266d:
        BStack6 = 0x1;
        uStack4 = 0;
    } else {
        iVar8 = param_3;
        if (param_5 - 0x2 < 0x19e) {//
// LAB_1040_2539:
            param_2 = param_5;
        } else {
            uVar8 = (param_3 >> 0x10);
            if (param_5 - 0x1a0 < 0x14 || param_5 == 0x1b4) {
                UVar4 = IsDlgButtonChecked(param_5, iVar8.field6_0x6);
                if (UVar4 == 0) {
                    puVar1 = &iVar8.field142_0x92;
                    puVar1 = puVar1 + 1;
                    if (0x0 < &iVar8.field142_0x92) {
                        (&iVar8.field142_0x92 + 0x2) = 0;
                    }
                    paVar3 = iVar8.field141_0x8e;
                    if ((paVar3 + 0x28) == &iVar8.field142_0x92) {
                        HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                        EnableWindow16(0x0, HVar5);
                    }
                } else {
                    puVar1 = &iVar8.field142_0x92;
                    puVar1 = puVar1 - 0x1;
                    HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                    BVar6 = IsWindowEnabled16(HVar5);
                    if (BVar6 == 0) {
                        HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                        EnableWindow16(0x1, HVar5);
                    }
                    if (&iVar8.field142_0x92 < 1) {
                        (&iVar8.field142_0x92 + 0x2) = 0x1;
                    }
                    pass1_1018_1c9a(iVar8.field141_0x8e, param_5);
                    puStack10 = pass1_1018_1e78(iVar8.field141_0x8e, -1);
                    uVar2 = (puStack10 >> 0x10);
                    uVar11 = uVar2 | puStack10;
                    if (uVar11 == 0) {
                        uStack12 = 0;
                    } else {
                        uStack12 = (puStack10 + 0x1c);
                    }
                    win_1008_5c7c(uStack12, uVar11, _u16_1050_02a0, CONCAT22(uStack12, 1));
                }
                if ((-0x1 < &iVar8.field142_0x92) && (paVar3 = iVar8.field141_0x8e, &iVar8.field142_0x92 <= (paVar3 + 0x28))) {
                    sys_1000_3f9c(CONCAT13(0x10, CONCAT12(0x50, local_16)), s__d_1050_5cf4, &iVar8.field142_0x92);
                    SetDlgItemText16(CONCAT22(0x1050, local_16), 0xfb2, iVar8.field6_0x6);
                }
                // TODO: goto LAB_1040_266d;
            }
            uVar4 = param_5 - 0xfb1;
//      if (uVar4 != 0) goto LAB_1040_2539;
            if (&iVar8.field142_0x92 < 0x0) {
                mem_op_1000_179c(0xb4, param_1);
                uStack24 = param_1;
                puVar8 = (uStack24 | uVar4);
                uVar13 = param_1 & 0xffff0000 | ZEXT24(puVar8);
                uStack26 = uVar4;
                if (puVar8.is_null()) {
                    iVar6 = 0;
                    uVar12 = 0;
                } else {
                    iVar6 = string_1040_8520(uVar13,
                                             CONCAT13((param_1 >> 0x8), CONCAT12(param_1, uVar4)),
                                             HWND16_1050_0396, 0x20030, 0x57c057b);
                    uVar12 = uVar13;
                }
                puStack10 = CONCAT22(uVar12, iVar6);
                fn_ptr_21 = (*puStack10 + 0x74);
                (**fn_ptr_21)(0x1000, iVar6, uVar12);
                // TODO: goto LAB_1040_27c0;
            }
            if (0x0 < &iVar8.field142_0x92) {
                mem_op_1000_179c(0xb4, param_1);
                uStack24 = param_1;
                puVar9 = (uStack24 | uVar4);
                uVar13 = param_1 & 0xffff0000;
                uVar14 = uVar13 | ZEXT24(puVar9);
                uStack26 = uVar4;
                if (puVar9.is_null()) {
                    iVar7 = 0;
                } else {
                    iVar7 = string_1040_8520(uVar14,
                                             CONCAT13((param_1 >> 0x8), CONCAT12(param_1, uVar4)),
                                             HWND16_1050_0396, 0x20021, 0x57d057b);
                    uVar13 = uVar14;
                }
                puStack10 = CONCAT22(uVar13, iVar7);
                puVar15 = pass1_1008_941a(CONCAT22(0x1050, local_1e), 0x1, 0xc2);
                param_1 = (uVar13 & 0xffff0000 | puVar15 >> 0x10);
                param_2 = &DAT_1050_1050;
                fn_ptr_21 = (*puStack10 + 0x6c);
                uVar10 = (**fn_ptr_21)(0x1008, puStack10, (puStack10 >> 0x10), local_1e);
//        if (uVar10 == 0x2) goto LAB_1040_27c0;
            }
            local_16[0] = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_2, 0x6), in_stack_0000fe84,
                                          in_stack_0000ffa8, in_stack_0000ffae, in_stack_0000ffb2);
            param_1 = (local_16[0] >> 0x10);
            uStack12 = 0x1a0;
            loop {
                UVar6 = IsDlgButtonChecked(uStack12, iVar8.field6_0x6);
                if (UVar6 == 1) {
                    uVar9 = (local_16[0] >> 0x10);
                    iVar9 = local_16[0];
                    (iVar9 + (iVar9 + 0x56) * 0x2 + 0x4e) = uStack12;
                    piVar1 = (iVar9 + 0x56);
                    *piVar1 = *piVar1 + 1;
                }
                uStack12 += 0x1;
                if uStack12 >= 0x1b5 { break; }
            }
            uVar2 = &iVar8.field142_0x92;
            puStack10 = (puStack10 & 0xffff0000 | uVar2);
            paVar3 = iVar8.field141_0x8e;
            (paVar3 + 0x28) = uVar2;
            PostMessage16(0x0, 0xc8, 0x111, HWND16_1050_0396);
            param_2 = 0x1;
        }
        uVar12 = SUB42(param_1, 0x0);
        BStack6 = post_win_msg_1040_7b3c(param_3, param_4, (param_4 >> 0x10), param_2);
        uStack4 = uVar12;
    }//
// LAB_1040_27c0:
    return CONCAT22(uStack4, BStack6);
}


pub unsafe fn win_ui_op_1040_42b2(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut iVar5: *mut astruct_893;
    let mut uVar5: u16;
    let mut LVar6: LRESULT;
    let mut local_54: [u16; 0x29] = [0; 0x29];

    iVar5 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        iVar5.field147_0x9a = 0x1;
        DestroyWindow16(iVar5.field6_0x6);
        return;
    }
    pass1_1000_4906(CONCAT22(0x1050, local_54), NULL, 0x51);
    HVar2 = GetDlgItem16(0xfb5, iVar5.field6_0x6);
    LVar6 = SendMessage16(CONCAT22(0x1050, local_54), 0x51, 0xd, HVar2);
    uVar4 = (LVar6 >> 0x10);
    uVar3 = pass1_1000_3e2c(CONCAT22(0x1050, local_54));
    if ((uVar4 | uVar3) != 0) {
        iVar5.field142_0x92 = uVar3;
        (&iVar5.field142_0x92 + 0x2) = uVar4;
    }
    if (uVar4 < 0x0) {
        uVar1 = iVar5.field141_0x8e;
        uVar1 = (uVar1 + 0x76);
        wsprintf16(local_54, 0x5d3c1050, CONCAT22(uVar1, 0x1050), (uVar1 >> 0x10));
        SendMessage16(CONCAT22(0x1050, local_54), 0x0, 0xc, HVar2);
        SetFocus16(HVar2);
        SendMessage16(-0x10000, 0x0, 0x401, HVar2);
        return;
    }
    HVar2 = GetDlgItem16(0x1, iVar5.field6_0x6);
    EnableWindow16(0x0, HVar2);
    uVar1 = iVar5.field141_0x8e;
    (uVar1 + 0x76) = iVar5.field142_0x92;
    PostMessage16(iVar5.field142_0x92, 0x0, 0x400, HWND16_1050_0396);
    HVar2 = GetDlgItem16(0x1, iVar5.field6_0x6);
    EnableWindow16(0x1, HVar2);
    return;
}


pub unsafe fn enable_win_1040_5780(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut hwnd: HWND16;
    let mut in_EDX: *mut Struct57;
    let mut iVar4: *mut astruct_945;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut paVar6: *mut astruct_945;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    ppcVar1 = (*param_1 + 0x74);
    paVar6 = iVar4;
    (**ppcVar1)();
    puVar5 = mixed_1010_20ba(in_EDX, _u16_1050_0ed0, CONCAT22(paVar6, 0x3), in_stack_0000fe98, in_stack_0000ffbc,
                             in_stack_0000ffc2, in_stack_0000ffc6);
    uVar2 = iVar4.field143_0x90;
    uVar3 = pass1_1010_acc0(puVar5, (puVar5 >> 0x10), (uVar2 + 0x6));
    if (uVar3 != 0) {
        hwnd = GetDlgItem16(0x1790, iVar4.field6_0x6);
        EnableWindow16(0x1, hwnd);
    }
    return;
}

pub unsafe fn enable_win_1040_6880(param_1: *mut astruct_925, mut param_2: i16)

{
    let mut uVar2: u32;
    let mut HVar3: HWND16;
    let mut iVar3: *mut astruct_925;
    let mut uVar4: u16;
    let mut uVar1: u32;

    if (param_2 == 0x8) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar3 = GetDlgItem16(0x107, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x24), HVar3);
        HVar3 = GetDlgItem16(0x108, iVar3.field6_0x6);
        uVar2 = iVar3.field147_0x94;
        EnableWindow16((uVar2 + 0x26), HVar3);
    }
    return;
}

pub unsafe fn enable_win_1040_6ff2(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut iVar3: *mut astruct_926;
    let mut uVar3: u16;

    if (param_2 == 0x8) {
        uVar3 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar2 = GetDlgItem16(0x107, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x26), HVar2);
    }
    return;
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

pub unsafe fn dlg_ui_op_1040_2a64(mut param_1: u16, struct_b_param_1: *mut StructB)

{
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut struct_b_6: *mut StructB;
    let mut iVar8: *mut astruct_918;
    let mut uVar7: u16;
    let mut in_stack_0000fe30: u16;
    let mut in_stack_0000fe34: u16;
    let mut in_stack_0000ff5a: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ffa2: u16;
    let mut iVar9: i16;
    let mut local_16: RECT16;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut uStack12: u32;
    let mut uStack8: u32;
    let mut iStack4: i16;
    let mut paVar6: *mut Struct57;

    unk_win_ui_op_1040_b230(param_1, struct_b_param_1);
    iStack4 = 0x5;
    iVar9 = 0;
    uVar7 = (struct_b_param_1 >> 0x10);
    struct_b_6 = struct_b_param_1;
    uVar1 = &struct_b_6[0x7].hwnd_0x6;
    uStack12 = struct_op_1030_73a8((uVar1 + 0x6), in_AX, param_1);
    paVar5 = CONCAT22(in_register_0000000a, (uStack12 >> 0x10));
    PTR_LOOP_1050_5d04 = pass1_1028_4a9a(uStack12, iVar9);
    for iStack14 in 0..iStack4 {
        if (iStack14 != 0) {
            (&PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0;
        }
        iVar9 = iStack14 * 0xc;
        local_16.x = (iVar9 + 0x5cfc);
        local_16.y = (iVar9 + 0x5cfe);
        paVar2 = (&PTR_LOOP_1050_0000 + 1);
        uStack18 = 0x1;
        uStack16 = 0x1;
        MapDialogRect16(&local_16, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar5);
        uVar4 = paVar5 | paVar2;
        paVar6 = (paVar5 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            paVar2 = null_mut();
            paVar5 = (paVar5 & 0xffff0000);
        } else {
            pass1_1008_3bd6(paVar6, paVar2, paVar5, 0x1, CONCAT22(local_16.x, local_16.y), 0x101, 0xff0100,
                            CONCAT22(struct_b_6.lpvoid_field_0x8, (iVar9 + 0x5d00)), in_stack_0000ffa2,
                            in_stack_0000fe30, in_stack_0000fe34, in_stack_0000ff5a, in_stack_0000ff5e, in_stack_0000ff62);
            paVar5 = paVar6;
        }
        uVar4 = paVar5;
        uStack8 = CONCAT22(uVar4, paVar2);
        if (PTR_LOOP_1050_5d04.is_null()) {
            if ((iStack14 != 0) && ((uVar4 | paVar2) != 0)) {
                EnableWindow16(0x0, &paVar2.field11_0x18);
            }
        } else {
            iVar8 = (iStack14 * 0xc);
            uVar3 = pass1_1028_4a9a(uStack12, (iVar8 + 0x5d02));
            if (uVar3 != 0) {
                (iVar8 + 0x5d04) = 0x1;
                SetDlgItemText16(&struct_b_6[0x7].field6_0xc, (iVar8 + 0x5d06),
                                 struct_b_6.lpvoid_field_0x8);
            }
        }
    }
    return;
}


pub unsafe fn win_ui_op_1040_2bb2(param_1: *mut u8, pstruct_903_param_2: *mut Struct903, mut param_3: u16, mut param_4: u32)

{
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut HVar3: HWND16;
    let mut iVar4: *mut astruct_922;
    let mut iVar5: i16;
    let mut iVar3: *mut astruct_920;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut id: *mut u8;
    let mut iStack8: i16;
    let mut iStack4: i16;

    if (param_4 == 0x158) {
        PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04.is_null());
        if (PTR_LOOP_1050_5d04.is_null()) {
            //   for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 1)
            for iStack8 in 1..5 {
                iVar5 = iStack8 * 0xc;
                HVar3 = GetDlgItem16((iVar5 + 0x5d00), (pstruct_903_param_2 + 0x6));
                EnableWindow16(0x0, HVar3);
                (&PTR_LOOP_1050_5d04 + iVar5) = 0;
                SetDlgItemText16((pstruct_903_param_2 + 0x94),
                                 (&PTR_s_post_1050_015d_1050_5d06 + iVar5),
                                 (pstruct_903_param_2 + 0x6));
            }
            HVar3 = (pstruct_903_param_2 + 0x6);
            uVar1 = (pstruct_903_param_2 + 0x94);
            uVar6 = uVar1;
            uVar7 = (uVar1 >> 0x10);
            id = PTR_s_post_1050_015d_1050_5d06;
            // TODO: goto LAB_1040_2ccc;
        }
        // for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 1)
        for iStack8 in 1..5 {
            iVar3 = (iStack8 * 0xc);
            HVar3 = GetDlgItem16((iVar3 + 0x5d00), (pstruct_903_param_2 + 0x6));
            EnableWindow16(0x1, HVar3);
            (iVar3 + 0x5d04) = 0;
            SetDlgItemText16((pstruct_903_param_2 + 0x94), (iVar3 + 0x5d06),
                             (pstruct_903_param_2 + 0x6));
        }
        HVar3 = (pstruct_903_param_2 + 0x6);
        id = PTR_s_post_1050_015d_1050_5d06;
    } else {
        if (param_4 == 0x159) {
            iStack4 = 0x1;
        } else if (param_4 == 0x15a) {
            iStack4 = 0x2;
        } else if (param_4 == 0x15b) {
            iStack4 = 0x3;
        } else {
            if (param_4 != 0x15c) {
                pass1_1040_b54a(param_1, pstruct_903_param_2, param_3, param_4);
                return;
            }
            iStack4 = 0x4;
        }
        if (iStack4 == 0) {
            return;
        }
        iVar4 = (iStack4 * 0xc);
        uVar2 = ((iVar4 + 0x5d04) == 0);
        (iVar4 + 0x5d04) = uVar2;
        if (uVar2 == 0) {
            HVar3 = (pstruct_903_param_2 + 0x6);
            uVar1 = (pstruct_903_param_2 + 0x94);
            uVar6 = uVar1;
            uVar7 = (uVar1 >> 0x10);
            id = (iVar4 + 0x5d06);
            // TODO: goto LAB_1040_2ccc;
        }
        HVar3 = (pstruct_903_param_2 + 0x6);
        id = (&PTR_s_post_1050_015d_1050_5d06 + iStack4 * 0xc);
    }
    uVar1 = (pstruct_903_param_2 + 0x98);
    uVar6 = uVar1;
    uVar7 = (uVar1 >> 0x10);//
// LAB_1040_2ccc:
    SetDlgItemText16(CONCAT22(uVar7, uVar6), id, HVar3);
    return;
}


pub unsafe fn set_win_text_1040_3590(mut param_1: u16, param_2: *mut astruct_923)

{
    let mut HVar1: HWND16;
    let mut HVar2: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut iVar5: *mut astruct_923;
    let mut uVar5: u16;
    let mut in_stack_0000f8f8: u16;
    let mut in_stack_0000fa1c: u16;
    let mut in_stack_0000fa22: u16;
    let mut in_stack_0000fa26: u16;
    let mut uVar6: u8;
    let mut in_stack_0000fa50: u16;
    let mut local_59a: u32;
    let mut local_596: u32;
    let mut BStack1426: bool;
    let mut uStack1424: u16;
    let mut local_58e: [u16; 0x41] = [0; 0x41];
    let mut local_50c: [u16; 0x80] = [0; 0x80];
    let mut uStack1036: u32;
    let mut puStack1032: *mut u32;
    let mut local_404: [u8; 0x402] = [0; 0x402];

    puStack1032 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0,
                                  CONCAT22(in_stack_0000fa50, 0x2), in_stack_0000f8f8, in_stack_0000fa1c,
                                  in_stack_0000fa22, in_stack_0000fa26);
    uVar4 = (puStack1032 >> 0x10);
    uStack1036 = (puStack1032 + 0x68);
    uVar5 = (param_2 >> 0x10);
    iVar5 = param_2;
    GetWindowText16(0x80, CONCAT22(0x1050, local_50c), iVar5.field6_0x6);
    uVar6 = SUB21(local_50c, 0x0);
    wsprintf16(local_58e, CONCAT13((local_50c >> 0x8), CONCAT12(uVar6, 0x1050)), uVar6,
               CONCAT22(uStack1036, 0x1050), (uStack1036 >> 0x10));
    BStack1426 = SetWindowText16(CONCAT22(0x1050, local_58e), iVar5.field6_0x6);
    sprintf_op_1018_34b6(BStack1426, uVar4, iVar5.field141_0x8e);
    uStack1424 = uVar4;
    pass1_1018_3d44(iVar5.field141_0x8e, CONCAT22(0x1050, &local_59a), CONCAT22(0x1050, &local_596));
    HVar1 = GetDlgItem16(0x193, iVar5.field6_0x6);
    iVar5.field148_0x98 = HVar1;
    EnableWindow16(0x1, HVar1);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
    wsprintf16(local_50c, 0x50, CONCAT22(local_404, 0x1050), CONCAT22(local_596, 0x1050),
               (local_596 >> 0x10));
    HVar1 = GetDlgItem16(0x195, iVar5.field6_0x6);
    SetWindowText16(CONCAT22(0x1050, local_50c), HVar1);
    HVar2 = GetDlgItem16(0x196, iVar5.field6_0x6);
    HVar1 = HVar2;
    sprintf_op_1018_34b6(HVar2, uVar4, iVar5.field141_0x8e);
    SetWindowText16(CONCAT22(uVar4, HVar2), HVar1);
    HVar1 = GetDlgItem16(0x197, iVar5.field6_0x6);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
    SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
    wsprintf16(local_50c, CONCAT22(local_404, 0x1050), CONCAT22(local_59a, 0x1050),
               (local_59a >> 0x10));
    HVar1 = GetDlgItem16(0x198, iVar5.field6_0x6);
    SetWindowText16(CONCAT22(0x1050, local_50c), HVar1);
    uVar3 = GetDlgItem16(0x199, iVar5.field6_0x6);
    HVar1 = uVar3;
    unk_str_op_1018_35b0(uVar3, iVar5.field141_0x8e);
    if ((uVar4 | uVar3) == 0) {
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
        SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
        GetDlgItem16(0x19a, iVar5.field6_0x6);
        HVar1 = _u16_1050_14cc;
        load_string_1010_84e0(HVar1, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
        SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
        EnableWindow16(0x0, iVar5.field148_0x98);
        return;
    }
    SetWindowText16(CONCAT22(uVar4, uVar3), HVar1);
    return;
}


pub unsafe fn enable_win_1040_3a36(param_1: *mut astruct_924, mut param_2: u16, mut param_3: u16, mut param_4: i16) -> u16

{
    let mut puVar1: *mut u16;
    let mut bVar2: bool;
    let mut iVar3: *mut astruct_924;
    let mut uVar3: u16;

    bVar2 = false;
    iVar3 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_4 == 0) {
//    if (iVar3.field155_0x9e <= iVar3.field154_0x9c) goto LAB_1040_3a79;
        puVar1 = &iVar3.field154_0x9c;
        *puVar1 = *puVar1 + 1;
    } else {
//    if (param_4 != 1) goto LAB_1040_3a79;
//    if (iVar3.field154_0x9c == 0) goto LAB_1040_3a79;
        puVar1 = &iVar3.field154_0x9c;
        *puVar1 = *puVar1 - 0x1;
    }
    bVar2 = true;//
// LAB_1040_3a79:
    if (bVar2) {
        SetDlgItemInt16(0x0, iVar3.field154_0x9c, 0x18e, iVar3.field6_0x6);
    }
    if ((iVar3.field154_0x9c != 0) && (iVar3.field158_0xa2 == 0)) {
        iVar3.field158_0xa2 = 0x1;
        EnableWindow16(0x1, iVar3.field153_0x9a);
    }
    if ((iVar3.field154_0x9c == 0) && (iVar3.field158_0xa2 != 0)) {
        iVar3.field158_0xa2 = 0;
        EnableWindow16(0x0, iVar3.field153_0x9a);
    }
    return 0x0;
}

pub unsafe fn enable_window_1020_1bd4(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_901,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut bVar2: bool;
    let mut hwnd: HWND16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar8: u16;
    let mut puStack12: *mut u32;
    let mut uVar7: u32;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    bVar2 = false;
    pass1_1020_1d8e(
        CONCAT13((param_4 >> 0x8), CONCAT12(param_4, param_3)),
        CONCAT22(param_6, param_5),
    );
    if (param_1 != 0) {
        if (param_1 < 0x2) {
            bVar2 = true;
        } else {
            hwnd = GetDlgItem16(0x1, param_3.field6_0x6);
            pass1_1010_4e8c(param_3.field141_0x8e);
            param_1 = EnableWindow16(0x1, hwnd);
            pass1_1010_4df0(paVar6, param_3.field141_0x8e);
            if ((param_1 == 0) && (bVar2 = true, param_3.field146_0x96 == 0)) {
                param_1 = EnableWindow16(0x0, hwnd);
            }
        }
    }
    if (bVar2) {
        uVar8 = 0x1000;
        mem_op_1000_179c(0xb4, paVar6);
        uVar4 = paVar6 | param_1;
        uVar7 = paVar6 & 0xffff0000 | uVar4;
        if (uVar4 == 0) {
            iVar3 = 0;
            uVar5 = 0;
        } else {
            uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar3 = string_1040_8520(
                uVar7,
                CONCAT22(paVar6, param_1),
                param_3.field6_0x6,
                0x20030,
                0x62a057b,
            );
            uVar5 = uVar7;
        }
        puStack12 = CONCAT22(uVar5, iVar3);
        ppcVar1 = (*puStack12 + 0x74);
        (**ppcVar1)(uVar8, iVar3, uVar5);
    }
    return;
}


pub unsafe fn win_ui_op_1038_a584(mut param_1: u16, mut param_2: u16, mut param_3: i16, mut param_4: i16)

{
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  let mut hwnd_00: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut paVar3: *mut astruct_486;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff70: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut local_52: [u8;0x50] = [0;0x50];

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_4 != 0) {
    hwnd = GetDlgItem16(0x114,(param_2 + 0x6));
    GetWindowText16(0x50,CONCAT22(0x1050,local_52),hwnd);
    uVar1 = str_op_1000_3da4(CONCAT22(0x1050,local_52));
    if (uVar1 != 0) {
      puVar4 = local_52;
      uVar5 = SUB42(&DAT_1050_1050,0x0);
      paVar3 =
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_6006((paVar3 >> 0x10),paVar3,CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,(param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      destroy_win_1040_7b98(CONCAT22(param_3,param_2));
    }
  }
  return;
}

pub unsafe fn win_ui_op_1038_a788(mut param_1: u16, mut param_2: u32, mut param_3: i16)

{
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  let mut hwnd_00: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u16;
  let mut pUVar2: *mut u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff70: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut local_52: [u8;0x50] = [0;0x50];
  let mut puVar3: *mut u8;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_3 != 0) {
    uVar3 = (param_2 >> 0x10);
    hwnd = GetDlgItem16(0x115,(param_2 + 0x6));
    GetWindowText16(0x50,CONCAT22(0x1050,local_52),hwnd);
    uVar1 = str_op_1000_3da4(CONCAT22(0x1050,local_52));
    if (uVar1 != 0) {
      puVar4 = local_52;
      uVar5 = SUB42(&DAT_1050_1050,0x0);
      pUVar2 =
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_5fd8((pUVar2 >> 0x10),pUVar2,CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,(param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      destroy_win_1040_7b98(param_2);
    }
  }
  return;
}


pub unsafe fn win_dlg_op_1040_2f90(mut param_1: u16, mut param_2: u32)

{
    let mut uVar1: u16;
    let mut HVar2: HWND16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: *mut astruct_943;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut uVar6: u32;
    let mut l_param: *mut c_char;
    let mut in_stack_0000fd7a: u16;
    let mut in_stack_0000fd7c: u16;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000fea4: u16;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000fea8: u16;
    let mut in_stack_0000feaa: u16;
    let mut in_stack_0000fed2: u16;
    let mut in_stack_0000fed4: u16;
    let mut local_116: *mut u32;
    let mut local_112: *mut u32;
    let mut local_10e: [u16; 0x41] = [0; 0x41];
    let mut local_8c: [u8; 0x82] = [0; 0x82];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    puStack6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fed2, 0x2), in_stack_0000fd7a,
                               in_stack_0000fe9e, in_stack_0000fea4, in_stack_0000fea8);
    paVar3 = (paVar3 & 0xffff0000 | puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x68);
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    GetWindowText16(0x80, CONCAT22(0x1050, local_8c), iVar4.field6_0x6);
    wsprintf16(local_10e, CONCAT22(local_8c, 0x1050), CONCAT22(uStack10, 0x1050),
               (uStack10 >> 0x10));
    SetWindowText16(CONCAT22(0x1050, local_10e), iVar4.field6_0x6);
    HVar2 = GetDlgItem16(0x182, iVar4.field6_0x6);
    iVar4.field143_0x92 = HVar2;
    pass1_1018_3a94(iVar4.field145_0x96, CONCAT22(0x1050, &local_116), CONCAT22(0x1050, &local_112));
    send_msg_1040_3374(param_2, local_112, iVar4.field143_0x92);
    puVar5 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fed4, 0x2f), in_stack_0000fd7c,
                             in_stack_0000fea0, in_stack_0000fea6, in_stack_0000feaa);
    uVar1 = (puVar5 >> 0x10);
    uVar6 = (puVar5 + 0x24);
    uVar6 = pass1_1018_3a7a(uVar6, uVar1, iVar4.field145_0x96, uVar6);
    SendMessage16(uVar6, 0xffff, 0x40d, iVar4.field143_0x92);
    HVar2 = GetDlgItem16(0x183, iVar4.field6_0x6);
    iVar4.field144_0x94 = HVar2;
    send_msg_1040_3374(param_2, local_116, HVar2);
    l_param = load_string_1010_847e(_u16_1050_14cc, 0x531);
    SendDlgItemMessage16(l_param, 0x0, 0x403, 0x183, iVar4.field6_0x6);
    SendDlgItemMessage16(l_param, 0xffff, 0x40d, 0x183, iVar4.field6_0x6);
    HVar2 = GetDlgItem16(0x181, iVar4.field6_0x6);
    iVar4.field141_0x8e = HVar2;
    HVar2 = GetDlgItem16(0x184, iVar4.field6_0x6);
    iVar4.field142_0x90 = HVar2;
    return;
}

pub unsafe fn set_window_text_1018_6066(
    param_1: *mut astruct_937,
    mut param_2: u16,
    in_win_text_3: *mut c_char,
    dialog_id_5: INT16,
) {
    let mut hwnd: HWND16;

    hwnd = GetDlgItem16(dialog_id_5, param_1.hwnd_field_0x6);
    SetWindowText16(in_win_text_3, hwnd);
    return;
}


pub unsafe fn set_window_text_1018_6086(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut hwnd_1: HWND16;
    let mut uVar2: u16;

    wsprintf16(&stack0xfff4, 0x42421050, CONCAT22(param_3, 0x1050));
    uVar2 = (param_1 >> 0x10);
    hwnd_1 = GetDlgItem16(0x1be, (param_1 + 0x6));
    SetWindowText16(CONCAT22(0x1050, &stack0xfff4), hwnd_1);
    wsprintf16(&stack0xfff4, 0x42451050, CONCAT22(param_2, 0x1050));
    hwnd_1 = GetDlgItem16(0x1bf, (param_1 + 0x6));
    SetWindowText16(CONCAT22(0x1050, &stack0xfff4), hwnd_1);
    return;
}


pub unsafe fn set_win_tet_1020_1d2a(
    param_1: *mut astruct_938,
    mut param_2: u16,
    in_win_text_3: *mut c_void,
    mut param_4: u16,
    in_dlg_id_5: INT16,
) {
    let mut hwnd: HWND16;

    hwnd = GetDlgItem16(param_4, param_1.field6_0x6);
    SetWindowText16(in_win_text_3, hwnd);
    return;
}


pub unsafe fn set_win_text_1038_8358(mut param_1: u16, param_2: *mut Struct903)

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut uVar4: u16;
  let mut uVar3: u16;
  let mut local_30a: [u8;0x102] = [0;0x102];
  let mut local_208: [u8;0x100] = [0;0x100];
  let mut local_108: [u8;0x100] = [0;0x100];
  let mut uStack8: u32;
  let mut HStack4: HWND16;
  let mut uVar1: u32;

  uVar3 = (param_2 >> 0x10);
  uVar4 = param_2;
  HStack4 = GetDlgItem16(0x1857,(uVar4 + 0x6));
  uStack8 = pass1_1008_b820(HStack4,param_1,(uVar4 + 0x94));
  if (uStack8 == 0) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_30a,&DAT_1050_1050);
    pcVar1 = local_30a;
  }
  else {
    uVar2 = send_dlg_item_msg_1038_8164(uVar4, uVar3, CONCAT22(0x1050, local_108), 0x1854);
    if (uVar2 == 0) {
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_208,&DAT_1050_1050);
    }
    else {
      load_string_1008_b65a((uVar4 + 0x94),local_208,CONCAT22(local_108,0x1050),&DAT_1050_1050);
    }
    pcVar1 = local_208;
  }
  SetWindowText16(CONCAT22(0x1050,pcVar1),HStack4);
  return;
}


pub unsafe fn win_ui_op_1038_a4ee(mut param_1: u16, mut param_2: u16, struct_b_param_1: *mut StructB)

{
  let mut lp_string: u32;
  let mut hwnd: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
   let mut struct_b_1: *mut StructB;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut LVar5: LRESULT;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  win_1008_5c7c(param_1,paVar1,_u16_1050_02a0,0x20001);
  uVar3 = (struct_b_param_1 >> 0x10);
  struct_b_1 = struct_b_param_1;
  (struct_b_1 + 0x7).field0_0x0 = param_1;
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(in_stack_0000ffec,0x2),in_stack_0000fe94,
                           in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
  uVar2 = (paVar1 >> 0x10);
  lp_string = (puVar4 + 0x6c);
  hwnd = GetDlgItem16(0x114,struct_b_1.lpvoid_field_0x8);
  SetWindowText16(lp_string,hwnd);
  SetFocus16(hwnd);
  LVar5 = SendMessage16(-0x10000,0x0,0x401,hwnd);
  unk_win_ui_op_1038_a18c(CONCAT22(uVar2,(LVar5 >> 0x10)),struct_b_param_1,in_stack_0000ff9e);
  ShowWindow16(0x5,struct_b_1.lpvoid_field_0x8);
  return;
}


pub unsafe fn win_dlg_op_1038_c58e(mut param_1: u16, mut param_2: u32)

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
             &DAT_1050_1050);
  uVar1 = (iVar2 + 0x92);
  wsprintf16(local_40c,CONCAT22(local_80e,0x1050),CONCAT22(*puStack2070,0x1050),
             (*puStack2070 >> 0x10),uVar1,(uVar1 >> 0x10));
  SetDlgItemText16(CONCAT22(0x1050,local_40c),0x17f,(iVar2 + 0x6));
  return;
}


pub unsafe fn FUN_1038_d8ae(param_1: *mut StructD, mut param_2: u16, struct_b_param_2: *mut StructB, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar2: u32;
    let mut puVar3: *mut u32;
    let mut rect: *mut Struct57;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut struct_b_1: *mut StructB;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar9: u16;
    let mut uVar8: u16;
    let mut in_stack_0000fe24: u16;
    let mut in_stack_0000fe28: u16;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000ff4e: u16;
    let mut in_stack_0000ff52: u16;
    let mut in_stack_0000ff56: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa6: u16;
    let mut local_26: u16;
    let mut uStack36: u16;
    let mut uStack34: u16;
    let mut uStack32: u16;
    let mut puStack30: *mut u16;
    let mut puStack14: *mut u32;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut HStack6: HCURSOR16;
    let mut HStack4: HCURSOR16;

    HStack4 = LoadCursor16(0x7f02, 0x0);
    HStack6 = SetCursor16(HStack4);
    dialog_ui_fn_1040_78e2(struct_b_param_2);
    uVar7 = (struct_b_param_2 >> 0x10);
    struct_b_1 = struct_b_param_2;
    uStack8 = pass1_1010_0886();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_1);
    } else {
        param_1 = (param_1 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack30 = CONCAT22(param_1, PTR_LOOP_1050_5f2c);
    puVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, param_1);
    struct_b_1[0x7].field5_0xa = puVar3;
    struct_b_1[0x7].field6_0xc = param_1;
//   for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1)
    for iStack10 in 1..uStack8 {
        uVar2 = &struct_b_1[0x7].lpvoid_field_0x8;
        puStack30 = pass1_1010_08e2(uVar2, (uVar2 >> 0x10), iStack10);
        paVar5 = (param_1 & 0xffff0000 | puStack30 >> 0x10);
        local_26 = *puStack30;
        uStack36 = (puStack30 + 2);
        uStack34 = 0x1;
        uStack32 = 0x1;
        rect = &local_26;
        MapDialogRect16(rect, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar5);
        uVar4 = paVar5 | rect;
        param_1 = (paVar5 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar2 = &struct_b_1[0x7].field5_0xa;
            (uVar2 + iStack10 * 0x4) = 0;
        } else {
            pvVar1 = struct_b_1.lpvoid_field_0x8;
            pass1_1008_3bd6(param_1, rect, paVar5, 0x0, CONCAT22(local_26, uStack36), 0x101, 0xff0100,
                            CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, (puStack30 + 0x4))), param_4, in_stack_0000fe24, in_stack_0000fe28, in_stack_0000ff4e, in_stack_0000ff52, in_stack_0000ff56,
            );
            uVar2 = &struct_b_1[0x7].field5_0xa;
            uVar8 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            (iVar6 + iStack10 * 0x4) = rect;
            (iVar6 + iStack10 * 0x4 + 0x2) = param_1;
        }
        uVar2 = &struct_b_1[0x7].field5_0xa;
        uVar9 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        if ((iVar6 + iStack10 * 0x4) != 0) {
            uVar2 = (iVar6 + iStack10 * 0x4);
            (uVar2 + 0x3e) = 0x1;
            uVar2 = &struct_b_1[0x7].field5_0xa;
            enable_win_1040_9234((uVar2 + iStack10 * 0x4), (puStack30 + 0x6));
        }
    }
    puStack14 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_5, 0x2), in_stack_0000fe78, in_stack_0000ff9c, in_stack_0000ffa2, in_stack_0000ffa6);
    SetWindowText16((puStack14 + 0x68), struct_b_1.lpvoid_field_0x8);
    ShowWindow16(0x5, struct_b_1.lpvoid_field_0x8);
    SetCursor16(HStack6);
    return;
}


pub unsafe fn win_ui_op_1040_3b1e(mut param_1: u16, struct_c_param_1: *mut StructC)

{
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut HVar3: HWND16;
    let mut pSVar4: *mut StructC;
    let mut in_register_0000000a: u16;
    let mut struct_c_4: *mut StructC;
    let mut unaff_SI: u16;
    let mut struct_c_param_2: *mut StructC;
    let mut uVar5: u32;
    let mut in_stack_0000fd8a: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb4: u16;
    let mut in_stack_0000feb8: u16;
    let mut puStack282: *mut u32;
    let mut local_10e: [u16; 0x41] = [0; 0x41];
    let mut local_8c: [u16; 0x41] = [0; 0x41];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    puStack6 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0,
                               CONCAT22(unaff_SI, 0x2), in_stack_0000fd8a, in_stack_0000feae, in_stack_0000feb4,
                               in_stack_0000feb8);
    uStack10 = (puStack6 + 0x68);
    struct_c_param_2 = (struct_c_param_1 >> 0x10);
    struct_c_4 = struct_c_param_1;
    GetWindowText16(0x80, CONCAT22(0x1050, local_8c), struct_c_4.field6_0x6);
    wsprintf16(local_10e, CONCAT22(local_8c, 0x1050), CONCAT22(uStack10, 0x1050),
               (uStack10 >> 0x10));
    SetWindowText16(CONCAT22(0x1050, local_10e), struct_c_4.field6_0x6);
    puStack282 = (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field_0x96));
    pSVar4 = struct_c_param_2;
    pass1_1018_3d44(struct_c_4.field141_0x8e,
                    (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field142_0x92)),
                    (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field_0x96)));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x80, local_10e, &DAT_1050_1050,
    );
    uVar1 = struct_c_4.field142_0x92;
    wsprintf16(local_8c, CONCAT22(local_10e, 0x1050), CONCAT22(*puStack282, 0x1050),
               (*puStack282 >> 0x10), uVar1, (uVar1 >> 0x10));
    SetDlgItemText16(CONCAT22(0x1050, local_8c), 0x187, struct_c_4.field6_0x6);
    BVar2 = CheckRadioButton16(0x188, 0x18d, 0x188, struct_c_4.field6_0x6);
    struct_c_4.field149_0xa0 = 0x188;
    uVar5 = switch_1018_3b9e(BVar2, pSVar4, struct_c_4.field141_0x8e, struct_c_4.field149_0xa0);
    send_dlg_item_msg_1040_3f12(struct_c_4, struct_c_param_2, uVar5);
    dialog_item_ui_op_1040_3e08(struct_c_param_1);
    HVar3 = GetDlgItem16(0x186, struct_c_4.field6_0x6);
    struct_c_4.field146_0x9a = HVar3;
    return;
}

pub unsafe fn set_win_text_1008_9664(mut param_1: u32, mut param_2: u16, param_3: *mut c_char) {
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)), param_3);
    SetWindowText16(param_1 & 0xffff0000 | (param_1 + 0xa), (param_1 + 0x8));
    return;
}


pub unsafe fn show_win_1008_96ae(mut param_1: u32, param_2: INT16) -> BOOL16 {
    let mut BVar1: bool;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        BVar1 = ShowWindow16(param_2, (param_1 + 0x8));
        return BVar1;
    }
    return 0x0;
}


pub unsafe fn show_win_1040_18a2(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u32;
   let mut struct_b_2: *mut StructB;
  let mut uVar2: u16;
  let mut local_304: [u16;0x80] = [0;0x80];
  let mut local_204: [u8;0x100] = [0;0x100];
  let mut local_104: [u8;0x100] = [0;0x100];
  let mut uStack4: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  check_dialog_btn_1040_1afe(struct_b_param_1);
  struct_b_2 = struct_b_param_1;
  uVar2 = (struct_b_param_1 >> 0x10);
  if (PTR_LOOP_1050_13ae.is_null() == false) {
    if (PTR_LOOP_1050_13ae == &u16_1050_0002) {
      uStack4 = 0x621;
    }
    else if (PTR_LOOP_1050_13ae == (&u16_1050_0002 + 1)) {
      uStack4 = 0x622;
    }
    else if (PTR_LOOP_1050_13ae == &u32_1050_0004) {
      uStack4 = 0x623;
    }
    else {
      uStack4 = 0x620;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_204,&DAT_1050_1050);
    wsprintf16(local_304,0x5cda1050,CONCAT22(local_204,0x1050),&DAT_1050_1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfe0,struct_b_2.lpvoid_field_0x8);
    uVar1 = &struct_b_2[0x7].field1_0x2;
    if ((uVar1 + 0x82) == 0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_204,&DAT_1050_1050);
    wsprintf16(local_304,0x5cdf1050,CONCAT22(local_204,0x1050),&DAT_1050_1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfdf,struct_b_2.lpvoid_field_0x8);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  ShowWindow16(0x5,struct_b_2.lpvoid_field_0x8);
  return;
}

pub unsafe fn show_win_1038_9fd0(param_1: *mut StructB)

{
  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,(param_1 + 0x6));
  return;
}


pub unsafe fn win_ui_op_1038_a972(struct_b_param_1: *mut StructB)

{
  let mut hwnd: HWND16;
  let mut BVar1: bool;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut uVar2: u32;
   let mut struct_b_3: *mut StructB;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut in_stack_0000ffaa: u16;

  uVar3 = (in_EDX >> 0x10);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar4 = (struct_b_param_1 >> 0x10);
  struct_b_3 = struct_b_param_1;
  SendDlgItemMessage16(0x0,0x1,0x401,0x116,struct_b_3.lpvoid_field_0x8);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0x401,0x11a,struct_b_3.lpvoid_field_0x8);
  uVar2 = CONCAT22(uVar3,(LVar5 >> 0x10));
  hwnd = GetDlgItem16(0x11a,struct_b_3.lpvoid_field_0x8);
  BVar1 = EnableWindow16(0x0,hwnd);
  win_1008_5c7c(BVar1,uVar2,_u16_1050_02a0,0x40001);
  (struct_b_3 + 0x7).field0_0x0 = BVar1;
  unk_win_ui_op_1038_a18c(uVar2,struct_b_param_1,in_stack_0000ffaa);
  ShowWindow16(0x5,struct_b_3.lpvoid_field_0x8);
  return;
}


pub unsafe fn show_win_1038_cb5c(mut param_1: u32, struct_b_param_1: *mut StructB, mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u16;
  let mut paVar7: *mut Struct57;
   let mut struct_b_5: *mut StructB;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut in_stack_0000fe48: u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut iStack10: i16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar8 = (struct_b_param_1 >> 0x10);
  struct_b_5 = struct_b_param_1;
  uVar3 = pass1_1008_eb6e();
  for iStack10 in 0 .. uVar3 {
    uVar1 = &struct_b_5[0x7].field1_0x2;
    puVar9 = pass1_1008_eb5c(uVar1,(uVar1 >> 0x10),iStack10);
    paVar7 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    paVar4 = puVar9;
    uVar2 = (puVar9 >> 0x10);
    paVar5 = paVar4;
    mem_op_1000_179c(0x42,paVar7);
    uVar6 = paVar7 | paVar5;
    param_1 = paVar7 & 0xffff0000 | uVar6;
    if (uVar6 != 0) {
      pass1_1008_3bd6(param_1,paVar5,paVar7,0x0,CONCAT22(*puVar9,paVar4.field1_0x2),0x101,0xff0100,
                      CONCAT22(struct_b_5.lpvoid_field_0x8,paVar4.field2_0x4),param_3,in_stack_0000fe48,
                      in_stack_0000fe4c,in_stack_0000ff72,in_stack_0000ff76,in_stack_0000ff7a);
    }
  }
  win_1008_5c7c(uVar3,param_1,_u16_1050_02a0,0x90001);
  ShowWindow16(0x5,struct_b_5.lpvoid_field_0x8);
  return;
}

pub unsafe fn check_radio_btn_show_win_1038_e19a(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog_ui_fn_1040_78e2(param_1);
    uVar1 = (param_1 >> 0x10);
    CheckRadioButton16(0x1807, 0x1807, 0x1807, (param_1 + 0x6));
    move_win_1040_826c(param_1, 0xc8, 0xc8);
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}

pub unsafe fn unk_win_ui_op_1038_e71c(mut param_1: u16, param_2: *mut StructB)

{
    let mut extraout_DX: u16;
    let mut struct_1: *mut StructB;
    let mut struct_1_lo: u16;
    let mut pcStack6: *mut c_char;

    dialog_ui_fn_1040_78e2(param_2);
    struct_1_lo = (param_2 >> 0x10);
    struct_1 = param_2;
    unk_load_str_op_1010_2c34(&struct_1[0x7].field1_0x2);
    pcStack6 = CONCAT22(extraout_DX, param_1);
    unk_str_op_1000_3d3e((param_2 & 0xffff0000 | ZEXT24(&struct_1.field8_0x10)),
                         CONCAT22(extraout_DX, param_1));
    fn_ptr_1000_17ce(pcStack6);
    move_win_1040_826c(param_2, -0x1, 0xffff);
    ShowWindow16(0x5, struct_1.lpvoid_field_0x8);
    struct_1[0x7].lpvoid_field_0x8 = (&PTR_LOOP_1050_0000 + 1);
    unk_win_msg_op_1008_9510((param_2 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
    DestroyWindow16(struct_1.lpvoid_field_0x8);
    return;
}


pub unsafe fn FUN_1038_ec16(mut param_1: u16, param_2: *mut StructB, param_3: *mut Struct57, mut param_4: u16)

{
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut rect: *mut Struct57;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut pSVar5: *mut StructD;
    let mut iVar7: i16;
    let mut iVar8: i16;
    let mut unaff_SI: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut in_stack_0000fe2a: u16;
    let mut in_stack_0000fe2e: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ff54: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut local_22: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut puStack26: *mut u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut puStack6: *mut u32;
    let mut paVar6: *mut Struct57;

    dialog_ui_fn_1040_78e2(param_2);
    puStack6 = mixed_1010_20ba(param_3, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b), in_stack_0000fe7e,
                               in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    pSVar5 = (param_3 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_0892();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
    } else {
        pSVar5 = (pSVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(pSVar5, PTR_LOOP_1050_5f2c);
    uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, pSVar5);
    uVar9 = (param_2 >> 0x10);
    iVar7 = param_2;
    (iVar7 + 0x8e) = uVar2;
    (iVar7 + 0x90) = pSVar5;
//   for (iStack10 = 0x1; uVar10 = (pSVar5 >> 0x10), iStack10 <= uStack8; iStack10 += 1)
    iStack10 = 1;
    uVar10 = pSVar5 >> 0x10;
    while iStack10 <= uStack8 {
        puStack26 = pass1_1010_0932(puStack6, (puStack6 >> 0x10), iStack10);
        uVar3 = (puStack26 >> 0x10);
        paVar6 = CONCAT22(uVar10, uVar3);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar6);
        uVar4 = paVar6 | rect;
        pSVar5 = (paVar6 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar1 = (iVar7 + 0x8e);
            (uVar1 + iStack10 * 0x4) = 0;
        } else {
            uVar10 = (iVar7 + 0x6);
            pass1_1008_3bd6(pSVar5, rect, paVar6, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((uVar10 >> 0x8), CONCAT12(uVar10, (puStack26 + 0x4))), param_4, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar1 = (iVar7 + 0x8e);
            uVar10 = (uVar1 >> 0x10);
            iVar8 = uVar1;
            (iVar8 + iStack10 * 0x4) = rect;
            (iVar8 + iStack10 * 0x4 + 0x2) = pSVar5;
        }
        uVar1 = (iVar7 + 0x8e);
        uVar10 = (uVar1 >> 0x10);
        iVar8 = uVar1;
        if ((iVar8 + iStack10 * 0x4) != 0) {
            enable_win_1040_9234((iVar8 + iStack10 * 0x4), (puStack26 + 0x6));
        }
        iStack10 += 1;
    }
    move_win_1040_826c(param_2, -0x1, 0xffff);
    ShowWindow16(0x5, (iVar7 + 0x6));
    return;
}

pub unsafe fn show_win_1040_0c7c(param_1: *mut StructB) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut local_6: u32;

    dialog_ui_fn_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x8e);
    pass1_1010_4f30(
        uVar1,
        (uVar1 >> 0x10),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_6 + 0x2),
    );
    move_win_1040_826c(param_1, local_6, (local_6 >> 0x10));
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}

pub unsafe fn show_win_1040_1d50(param_1: *mut StructB)

{
    dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}


pub unsafe fn show_win_1040_65ba(param_1: *mut StructD, struct_b_param_1: *mut StructB, mut param_3: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut rect: *mut Struct57;
    let mut uVar4: u16;
    let mut uVar5: *mut StructD;
    let mut paVar5: *mut Struct57;
    let mut struct_b_4: *mut StructB;
    let mut iVar6: i16;
    let mut unaff_SI: u16;
    let mut unaff_DI: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_stack_0000fe2a: u16;
    let mut in_stack_0000fe2e: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ff54: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut local_22: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut puStack26: *mut u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut puStack6: *mut u32;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar5 = param_1;
    puStack6 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b),
                               in_stack_0000fe7e, in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    uVar5 = (uVar5 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_0898();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar5);
    } else {
        uVar5 = (uVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(uVar5, PTR_LOOP_1050_5f2c);
    uVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, uVar5);
    uVar7 = (struct_b_param_1 >> 0x10);
    struct_b_4 = struct_b_param_1;
    struct_b_4[0x7].field1_0x2 = uVar3;
    struct_b_4[0x7].hwnd_0x6 = uVar5;
//   for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1)
    for iStack10 in 1..uStack8 {
        puStack26 = pass1_1010_0946(puStack6, (puStack6 >> 0x10), iStack10, uVar5, unaff_DI,
                                    &DAT_1050_1050);
        paVar5 = (uVar5 & 0xffff0000 | puStack26 >> 0x10);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar5);
        uVar4 = paVar5 | rect;
        uVar5 = (paVar5 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar2 = &struct_b_4[0x7].field1_0x2;
            (uVar2 + iStack10 * 0x4) = 0;
        } else {
            pvVar1 = struct_b_4.lpvoid_field_0x8;
            pass1_1008_3bd6(uVar5, rect, paVar5, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, (puStack26 + 0x4))), param_3, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar2 = &struct_b_4[0x7].field1_0x2;
            uVar8 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            (iVar6 + iStack10 * 0x4) = rect;
            (iVar6 + iStack10 * 0x4 + 0x2) = uVar5;
        }
        uVar2 = &struct_b_4[0x7].field1_0x2;
        uVar8 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        if ((iVar6 + iStack10 * 0x4) != 0) {
            unaff_DI = puStack26;
            enable_win_1040_9234((iVar6 + iStack10 * 0x4), (unaff_DI + 0x6));
        }
    }
    move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
    ShowWindow16(0x5, struct_b_4.lpvoid_field_0x8);
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


pub unsafe fn show_win_1040_355a(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn show_win_1010_7a76(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x20) == 0) {
        (iVar2 + 0x20) = 0x1;
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar2 + 0x1c));
        loop {
            lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            if (lVar4 == 0) {
                break;
            }
            uVar1 = (lVar4 + 0x8);
            ShowWindow16(0x0, (uVar1 + 0x6));
        }
    }
    return;
}


pub unsafe fn ui_op_1020_536e(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: i16,
    mut param_5: i16,
) {
    let mut piVar1: *mut i16;
    let mut UVar2: u16;
    let mut uVar3: u32;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut UVar7: u16;
    let mut uVar8: u16;
    let mut puVar9: *mut u8;
    let mut uVar10: u16;
    let mut in_register_0000000a: u16;
    let mut paVar11: *mut Struct57;
    let mut uVar12: u32;
    let mut paVar13: *mut Struct57;
    let mut paVar14: *mut Struct57;
    let mut puVar15: *mut u32;
    let mut unaff_SI: u16;
    let mut uVar16: u16;
    let mut puVar17: *mut u32;
    let pSVar18: *mut StructA;
    let mut paVar19: *mut Struct27;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ff60: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffb8: u16;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut iVar24: i16;
    let mut puStack16: *mut u32;

    paVar11 = CONCAT22(in_register_0000000a, param_1);
    uVar8 = param_5 - 0x1;
    uVar16 = (param_2 >> 0x10);
    iVar24 = param_2;
    if (uVar8 == 0) {
        if ((iVar24 + 0xfe) == 0) {
            mem_op_1000_179c(0xfc, paVar11);
            uVar10 = paVar11 | uVar8;
            uVar12 = paVar11 & 0xffff0000 | uVar10;
            if (uVar10 == 0) {
                (iVar24 + 0xfe) = 0;
            } else {
                piVar1 = (iVar24 + 0xcc);
                *piVar1 = *piVar1 + 1;
                unk_win_ui_op_1020_67ce(
                    CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, uVar8)),
                    (iVar24 + 0xcc),
                    param_2,
                    uVar12,
                );
                (iVar24 + 0xfe) = uVar8;
                (iVar24 + 0x100) = uVar12;
            }
            pass1_1008_6978(uVar8, uVar12, param_2, 0x0, (iVar24 + 0xfe));
            uVar3 = (iVar24 + 0xfe);
            uVar22 = uVar3;
            uVar23 = (uVar3 >> 0x10);
            uVar3 = (iVar24 + 0xfe);
            uVar16 = (uVar3 >> 0x10);
            puVar15 = uVar3;
            // TODO: goto LAB_1020_53f3;
        }
    } else {
        if (param_5 == 0x2) {
            uVar5 = param_4 + 0xc;
            puVar17 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, uVar5),
                in_stack_0000fe8a,
                in_stack_0000ffae,
                in_stack_0000ffb4,
                in_stack_0000ffb8,
            );
            paVar11 = (paVar11 & 0xffff0000 | puVar17 >> 0x10);
            uVar6 = pass1_1018_0afa(puVar17);
            if (uVar6 == 0) {
                piVar1 = (iVar24 + 0xcc);
                *piVar1 = *piVar1 + 1;
                UVar2 = (iVar24 + 0xcc);
                UVar7 = UVar2;
                mem_op_1000_179c(0xfe, paVar11);
                uVar8 = paVar11 | UVar7;
                paVar14 = (paVar11 & 0xffff0000);
                paVar13 = (paVar14 | uVar8);
                if (uVar8 == 0) {
                    UVar7 = 0;
                } else {
                    pass1_1020_289a(
                        CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, UVar7)),
                        UVar2,
                        param_2,
                    );
                    paVar14 = paVar13;
                }
                puVar9 = paVar14;
                puStack16 = CONCAT22(puVar9, UVar7);
                uVar20 = SUB41(paVar14, 0x0);
                uVar21 = (paVar14 >> 0x8);
                pass1_1020_294a(
                    puVar9,
                    CONCAT13(uVar21, CONCAT12(uVar20, UVar7)),
                    param_3,
                    uVar5,
                );
                uVar3 = *puStack16;
                ppcVar4 = (uVar3 + 0x8);
                uVar8 = (**ppcVar4)(0x1000, UVar7, puVar9);
                pass1_1008_3e0e(CONCAT13(uVar21, CONCAT12(uVar20, UVar7)));
                pass1_1008_6978(uVar8, paVar14, param_2, UVar2, CONCAT22(puVar9, UVar7));
                ppcVar4 = (uVar3 + 0xc);
                (**ppcVar4)(0x1008, UVar7, uVar20, 1);
            } else {
                pSVar18 = pass1_1018_0ad4(puVar17);
                paVar14 = (paVar11 & 0xffff0000 | pSVar18 >> 0x10);
                pass1_1008_3e0e(pSVar18);
            }
            pass1_1018_1662(puVar17, 0x0, 0x0);
            uVar3 = (iVar24 + 0xce);
            BringWindowToTop16((uVar3 + 0x8));
            uVar5 = 0x1;
            iVar24 = 0x4;
            paVar19 = mixed_1010_20ba(
                paVar14,
                _u16_1050_0ed0,
                0x1002b,
                in_stack_0000fe88,
                in_stack_0000ffac,
                in_stack_0000ffb2,
                in_stack_0000ffb6,
            );
            pass1_1010_089e(paVar19, uVar5, iVar24);
            pass1_1010_089e(paVar19, 0x1, 0x3);
            return;
        }
        uVar8 = param_5 - 0x3;
        if ((uVar8 == 0) && ((iVar24 + 0x102) == 0)) {
            mem_op_1000_179c(0xfc, paVar11);
            uVar10 = paVar11 | uVar8;
            uVar12 = paVar11 & 0xffff0000 | uVar10;
            if (uVar10 == 0) {
                (iVar24 + 0x102) = 0;
            } else {
                piVar1 = (iVar24 + 0xcc);
                *piVar1 = *piVar1 + 1;
                pass1_1020_0dc4(
                    CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, uVar8)),
                    (iVar24 + 0xcc),
                    param_2,
                    uVar12,
                    in_stack_0000ff5c,
                    in_stack_0000ff60,
                );
                (iVar24 + 0x102) = uVar8;
                (iVar24 + 0x104) = uVar12;
            }
            pass1_1008_6978(uVar8, uVar12, param_2, 0x0, (iVar24 + 0x102));
            uVar3 = (iVar24 + 0x102);
            uVar22 = uVar3;
            uVar23 = (uVar3 >> 0x10);
            uVar3 = (iVar24 + 0x102);
            uVar16 = (uVar3 >> 0x10);
            puVar15 = uVar3; //
            // LAB_1020_53f3:
            ppcVar4 = (*puVar15 + 0xc);
            (**ppcVar4)(0x8, uVar22, uVar23, 0x5);
            return;
        }
    }
    return;
}

pub unsafe fn unk_destroy_window_op_1018_6bb6(param_1: *mut astruct_28) {
    let mut BVar1: bool;
    let mut struct_1: *mut astruct_28;
    let mut uVar2: *mut astruct_28;

    uVar2 = (param_1 >> 0x10);
    struct_1 = param_1;
    if (struct_1.field229_0xea != 0) {
        PostMessage16(0x0, struct_1.field229_0xea, 0x111, HWND16_1050_0396);
    }
    PostMessage16(0x0, 0x79, 0x111, HWND16_1050_0396);
    if (struct_1.hwnd_0xf0 != 0) {
        BVar1 = IsWindow16(struct_1.hwnd_0xf0);
        if (BVar1 != 0) {
            DestroyWindow16(struct_1.hwnd_0xf0);
            struct_1.hwnd_0xf0 = 0;
        }
    }
    return;
}

pub unsafe fn destroy_window_1018_c518(param_1: *mut astruct_29) {
    let mut is_window: bool;
    let mut pstruct_29_1: *mut astruct_29;
    let mut pstruct_29_hi: *mut astruct_29;

    pstruct_29_hi = (param_1 >> 0x10);
    pstruct_29_1 = param_1;
    param_1.field0_0x0 = 0xc8bc;
    pstruct_29_1.field1_0x2 = 0x1018;
    fn_ptr_1000_17ce(pstruct_29_1.field259_0x108);
    if (pstruct_29_1.hwnd_0x112.is_null() == false) {
        is_window = IsWindow16(pstruct_29_1.hwnd_0x112);
        if (is_window != 0) {
            DestroyWindow16(pstruct_29_1.hwnd_0x112);
            pstruct_29_1.hwnd_0x112 = null_mut();
        }
    }
    pass1_1020_022c(param_1);
    return;
}

pub unsafe fn send_win_msg_1020_08fe(param_1: *mut astruct_63) {
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

pub unsafe fn destroy_win_1020_1dea(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut WVar2: u16;

    BVar1 = IsWindow16(param_3);
    if (BVar1 != 0) {
        WVar2 = GetWindowWord16(-0xc, param_3);
        if (WVar2 == 0x175) {
            DestroyWindow16(param_3);
            return 0x0;
        }
    }
    return 0x1;
}


pub unsafe fn destroy_win_1040_5256(param_1: *mut astruct_34)

{
    let mut pUVar1: *mut u32;
    let mut bool3: bool;
    let mut pstruct34_5: *mut astruct_34;
    let mut pstruct34_hi: *mut astruct_34;
    let mut unaff_CS: u16;
    let mut uVar2: u16;
    let mut fn_ptr_1: *mut *mut code;

    pstruct34_hi = (param_1 >> 0x10);
    pstruct34_5 = param_1;
    if (pstruct34_5.hwnd_0xb6 != 0) {
        // 0x1538
        unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
        bool3 = IsWindow16(pstruct34_5.hwnd_0xb6);
        if (bool3 != 0) {
            // 0x1538
            unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
            DestroyWindow16(pstruct34_5.hwnd_0xb6);
        }
    }
    pstruct34_5.hwnd_0xb6 = 0;
    pUVar1 = pstruct34_5.field148_0x94;
    uVar2 = pstruct34_5.field149_0x96;
    if ((uVar2 | pUVar1) != 0) {
        fn_ptr_1 = *pUVar1;
        (**fn_ptr_1)(unaff_CS, pUVar1, uVar2, 1);
    }
    pstruct34_5.field148_0x94 = 0;
    pstruct34_5.field150_0x98 = 0;
    return;
}


pub unsafe fn check_dialog_msg_1040_81b6(mut param_1: u32)

{
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut local_14: MSG16;

    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x78) = 0x1;
    loop {
        BVar1 = IsWindow16((param_1 + 0x6));
        if (BVar1 == 0) {
            return;
        }
        local_14.hwnd = &DAT_1050_1050;
        BVar1 = GetMessage16(0x0, 0x0, 0x0, &local_14);
        if (BVar1 == 0) { break; }
        IsDialogMessage16(&local_14, &DAT_1050_1050);
    }
    return;
}


pub unsafe fn destroy_win_1040_bb78(param_1: *mut astruct_35)

{
    let mut uVar1: u16;
    let mut is_window: bool;
    let mut pstruct35_5: *mut astruct_35;
    let mut pstruct35_hi: *mut astruct_35;
    let mut unaff_CS: u16;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    pstruct35_hi = (param_1 >> 0x10);
    pstruct35_5 = param_1;
    if (pstruct35_5.hwnd_0xb6 != 0) {
        // 0x1538
        unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
        is_window = IsWindow16(pstruct35_5.hwnd_0xb6);
        if (is_window != 0) {
            // 0x1538
            unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
            DestroyWindow16(pstruct35_5.hwnd_0xb6);
        }
    }
    pstruct35_5.hwnd_0xb6 = 0;
    puVar1 = pstruct35_5.field148_0x94;
    uVar1 = pstruct35_5.field149_0x96;
    if ((uVar1 | puVar1) != 0) {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)(unaff_CS, puVar1, uVar1, 1);
    }
    pstruct35_5.field148_0x94 = 0;
    pstruct35_5.field150_0x98 = 0;
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

pub unsafe fn destroy_window_1020_8250(param_1: *mut astruct_879) {
    let mut BVar1: bool;
    let mut iVar2: *mut astruct_879;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field236_0xec != 0) {
        BVar1 = IsWindow16(iVar2.field236_0xec);
        if (BVar1 != 0) {
            DestroyWindow16(iVar2.field236_0xec);
            iVar2.field236_0xec = 0;
        }
    }
    return;
}

pub unsafe fn destroy_win_1008_628e(mut param_1: u32) {
    let mut fn_ptr_1: *mut *mut code;

    fn_ptr_1 = ((param_1 + 0xd2) + 0x14);
    (**fn_ptr_1)();
    DestroyWindow16((param_1 + 0x8));
    (param_1 + 0x8) = 0;
    return;
}

// WARNING: Unable to use type for symbol uVar3
pub unsafe fn unk_destroy_win_op_1010_2fa0(param_1: *mut astruct_873) {
    astruct_872 * *ppaVar1;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_873;
    let mut uVar4: u16;
    let mut iStack4: *mut astruct_872;
    let mut uVar3: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar3.field39_0x28 = 0;
    iStack4 = null_mut();
    loop {
        ppaVar1 = &iVar3.field22_0x16;
        if (*ppaVar1 == iStack4 || *ppaVar1 < iStack4) {
            break;
        }
        uVar3 = (&iVar3.field_0x2a + iStack4 * 0x4);
        DestroyWindow16((uVar3 + 0x18));
        iStack4 = iStack4 + 1;
    }
    iVar3.field22_0x16 = null_mut();
    if ((iVar3.field82_0x54 | &iVar3.field_0x52) != 0) {
        iStack4 = null_mut();
        loop {
            uVar2 = &iVar3.field_0x52;
            if ((uVar2 + iStack4 * 0x4) != 0) {
                uVar2 = &iVar3.field_0x52;
                uVar2 = (uVar2 + iStack4 * 0x4);
                DestroyWindow16((uVar2 + 0x18));
                uVar2 = &iVar3.field_0x52;
                (uVar2 + iStack4 * 0x4) = 0;
            }
            iStack4 = iStack4 + 1;
            if iStack4 >= 0xa {
                break;
            }
        }
        fn_ptr_1000_17ce(*&iVar3.field_0x52);
        iVar3.field_0x52 = 0;
    }
    return;
}

pub unsafe fn unk_destroy_win_op_1010_305a(
    mut param_1: u16,
    param_2: *mut Struct27,
    mut param_3: i16,
    param_4: *mut astruct_65,
) {
    astruct_874 * *ppaVar1;
    let mut piVar2: *mut i16;
    let mut UVar3: u32;
    let mut lVar4: i32;
    let mut uVar5: u32;
    let mut bVar6: bool;
    let mut uVar8: u16;
    let mut iVar4: *mut Struct27;
    let mut iVar9: i16;
    let mut uVar7: *mut Struct27;
    let mut uVar10: u16;
    let mut iStack10: i16;
    let mut paStack8: *mut astruct_874;
    let mut paStack6: *mut astruct_874;
    let mut iVar7: *mut astruct_874;

    uVar8 = pass1_1040_c60e(param_4);
    uVar7 = (param_2 >> 0x10);
    iVar4 = param_2;
    iVar4.field18_0x12 = uVar8;
    iVar4.field19_0x14 = 0;
    paStack6 = null_mut();
    bVar6 = false;
    iVar4.field33_0x28 = 0;
    paStack8 = null_mut();
    loop {
        ppaVar1 = &iVar4.field20_0x16;
        if (*ppaVar1 == paStack8 || *ppaVar1 < paStack8) {
            //
            // LAB_1010_30ad:
            iVar7 = paStack6;
            if (bVar6) {
                while (
                    paStack8 = iVar7 + 0x1,
                    ppaVar1 = &iVar4.field20_0x16,
                    *ppaVar1 != paStack8 && paStack8 <= *ppaVar1,
                ) {
                    UVar3 = (&iVar4.field36_0x2e)[iVar7];
                    DestroyWindow16((UVar3 + 0x18));
                    (&iVar4.field36_0x2e)[iVar7] = 0;
                    iVar7 = paStack8;
                }
                iVar4.field20_0x16 = (paStack6 + 1);
                pass1_1010_1f62(param_2, 0x9);
            } else {
                iVar9 = iVar4.field20_0x16;
                (&iVar4.field34_0x2a)[iVar9 * 0x2] = param_4;
                (&iVar4.field35_0x2c)[iVar9 * 0x2] = (param_4 >> 0x10);
                iStack10 = 0xa;
                piVar2 = &iVar4.field20_0x16;
                *piVar2 = *piVar2 + 1;
                if (0x1 < iVar4.field20_0x16) {
                    UVar3 = (&iVar4.field30_0x22)[iVar4.field20_0x16];
                    iVar9 = UVar3;
                    uVar10 = (UVar3 >> 0x10);
                    iStack10 = (iVar9 + 0x20) + (iVar9 + 0x24) + 0x8;
                }
                mov_update_win_1040_93aa(param_4, iStack10, iVar4.field23_0x1a);
            }
            if (!bVar6) {
                pass1_1010_1f62(param_2, 0xa);
            }
            if (param_3 == 0) {
                if (iVar4.field69_0x52 != 0) {
                    paStack8 = null_mut();
                    loop {
                        lVar4 = iVar4.field69_0x52;
                        uVar10 = (lVar4 >> 0x10);
                        iVar9 = lVar4;
                        if (((iVar9 + paStack8 * 0x4) != 0)
                            && ((iVar9 + paStack8 * 0x4) != param_4))
                        {
                            lVar4 = iVar4.field69_0x52;
                            uVar5 = (lVar4 + paStack8 * 0x4);
                            DestroyWindow16((uVar5 + 0x18));
                        }
                        lVar4 = iVar4.field69_0x52;
                        (lVar4 + paStack8 * 0x4) = 0;
                        paStack8 = (paStack8 + 1);
                        if paStack8 >= 0xa {
                            break;
                        }
                    }
                }
                block_1010_3000::pass1_1010_32da(param_2, param_4);
                pass1_1010_1f62(param_2, 0x8);
            }
            return;
        }
        if ((&iVar4.field34_0x2a + paStack8 * 0x2) == param_4) {
            bVar6 = true;
            paStack6 = paStack8;
            // TODO: goto LAB_1010_30ad;
        }
        paStack8 = paStack8 + 1;
    }
}

pub unsafe fn destroy_win_1020_1e1e(mut param_1: u16, mut param_2: u16, param_3: HWND16) -> u16 {
    let mut BVar1: bool;
    let mut WVar2: u16;

    BVar1 = IsWindow16(param_3);
    if (BVar1 != 0) {
        WVar2 = GetWindowWord16(-0xc, param_3);
        if ((WVar2 != 1) && (WVar2 != 0x175)) {
            DestroyWindow16(param_3);
        }
    }
    return 0x1;
}

pub unsafe fn destroy_window_1020_3b3e(param_1: *mut astruct_30) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut lVar4: i32;
    let mut puVar5: *mut u8;
    let mut paVar6: *mut astruct_30;
    let mut struct_5: *mut astruct_30;
    let mut struct_6: *mut astruct_30;
    let mut unaff_CS: u16;

    struct_6 = (param_1 >> 0x10);
    struct_5 = param_1;
    struct_5.field262_0x10e = 0;
    if (struct_5.field261_0x10a != 0) {
        lVar4 = struct_5.field261_0x10a;
        // 0x1538
        unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
        DestroyWindow16((lVar4 + 0x6));
    }
    puVar1 = struct_5.field246_0xf6;
    uVar2 = struct_5.field247_0xf8;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(unaff_CS, puVar1);
    }
    struct_5.field246_0xf6 = 0;
    if (struct_5.field248_0xfa != 0) {
        puVar5 = (struct_6 | struct_5);
        if (param_1.is_null()) {
            paVar6 = null_mut();
        } else {
            puVar5 = &struct_5.field_0xf2;
            paVar6 = struct_6;
        }
        pass1_1010_1ea6(struct_5.field248_0xfa, CONCAT22(paVar6, puVar5));
    }
    struct_5.field248_0xfa = 0;
    return;
}


pub unsafe fn destroy_win_1038_ef3a(param_1: *mut StructD)

{
  let mut uVar2: u32;
  let mut iVar1: *mut StructD;
  let mut uVar1: *mut StructD;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x67c;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&iVar1.field_0x96 != 0) {
    uVar2 = &iVar1.field_0x96;
    DestroyWindow16((uVar2 + 0x6));
    iVar1.field_0x96 = 0;
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}

pub unsafe fn destroy_win_1040_7b98(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x74) == 0) {
    DestroyWindow16((param_1 + 0x6));
  }
  return;
}


pub unsafe fn destroy_win_1040_8b7e(mut param_1: u32)

{
  DestroyWindow16((param_1 + 0x6));
  return;
}

pub unsafe fn destroy_window_1040_b726(mut param_1: u32, mut param_2: i16)

{
  let mut fn_ptr_1: *mut *mut code;

  if (param_2 != 0) {
    fn_ptr_1 = (param_1 + 0x78);
    (**fn_ptr_1)();
  }
  DestroyWindow16((param_1 + 0x6));
  return;
}

pub unsafe fn destroy_window_1038_7d88(mut param_1: u32, mut param_2: u16, mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  pass1_1008_b544(param_3,(param_1 + 0x94),param_2);
  DestroyWindow16((param_1 + 0x6));
  return;
}

pub unsafe fn destroy_window_1038_a072(param_1: *mut astruct_880, mut param_2: u16, mut param_3: i16)

{
  if (param_3 != 0) {
    DestroyWindow16(param_1.field6_0x6);
  }
  return;
}

pub unsafe fn win_dlg_op_1038_c95e(struct_param_1: *mut astruct_882, mut param_2: i16)

{
  let mut uVar3: u32;
  let mut UVar4: u16;
  let mut UVar5: u16;
  let mut UVar6: u16;
  let mut iVar3: *mut astruct_882;
  let mut uVar7: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  iVar3 = struct_param_1;
  uVar7 = (struct_param_1 >> 0x10);
  if (param_2 == 0) {
    uVar3 = iVar3.field141_0x8e;
    (uVar3 + 0xa) = 0;
  }
  else {
    UVar4 = IsDlgButtonChecked(0xfac,iVar3.field6_0x6);
    if (UVar4 == 0) {
      UVar5 = IsDlgButtonChecked(0xfad,iVar3.field6_0x6);
      if (UVar5 == 0) {
        UVar6 = IsDlgButtonChecked(0xfae,iVar3.field6_0x6);
        if (UVar6 == 0) {
          UVar6 = IsDlgButtonChecked(0xfaf,iVar3.field6_0x6);
          if (UVar6 == 0) {
            UVar6 = IsDlgButtonChecked(0xfb0,iVar3.field6_0x6);
            if (UVar6 != 0) {
              uVar3 = iVar3.field141_0x8e;
              (uVar3 + 0xa) = 0x5;
            }
          }
          else {
            uVar3 = iVar3.field141_0x8e;
            (uVar3 + 0xa) = 0x4;
          }
        }
        else {
          uVar3 = iVar3.field141_0x8e;
          (uVar3 + 0xa) = 0x3;
        }
      }
      else {
        uVar2 = iVar3.field141_0x8e;
        (uVar2 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field141_0x8e;
      (uVar1 + 0xa) = 0x1;
    }
  }
  DestroyWindow16(iVar3.field6_0x6);
  PTR_LOOP_1050_5b80 = null_mut();
  return;
}

pub unsafe fn destroy_window_1038_cd88(struct_b_param_1: *mut StructB)

{
   let mut struct_1: *mut StructB;
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = (struct_b_param_1 >> 0x10);
  struct_1 = struct_b_param_1;
  ShowWindow16(0x5,struct_1.lpvoid_field_0x8);
  struct_1[0x7].lpvoid_field_0x8 = (&PTR_LOOP_1050_0000 + 1);
  unk_win_msg_op_1008_9510((struct_b_param_1 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
  DestroyWindow16(struct_1.lpvoid_field_0x8);
  return;
}

pub unsafe fn destroy_window_1038_c836(param_1: *mut astruct_881, mut param_2: u32, mut param_3: u32)

{
    let mut puVar1: *mut u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];
    let mut uVar1: u32;

    if (param_3 == 0xfce) {
        puVar1 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0xac);
        win_1008_5c9e(local_6, (puVar1 >> 0x10), _u16_1050_02a0, CONCAT22(0x1050, local_6));
        uVar1 = param_1.field141_0x8e;
        (uVar1 + 0xa) = 0x6;
        DestroyWindow16(param_1.field6_0x6);
        PTR_LOOP_1050_5b80 = null_mut();
        return;
    }
    post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3, param_3);
    return;
}

pub unsafe fn unk_win_ui_op_1040_1d7a(param_1: *mut astruct_33, mut param_2: i16)

{
    let mut UVar2: u16;
    let mut UVar1: u16;
    let mut iVar3: *mut astruct_33;
    let mut uVar3: *mut astruct_33;
    let mut uVar1: u32;

    iVar3 = param_1;
    uVar3 = (param_1 >> 0x10);
    if ((param_2 != 0) && (uVar1 = iVar3.field141_0x8e, (uVar1 + 0x72) != 0)) {
        UVar2 = IsDlgButtonChecked(0xe1, iVar3.hwnd_0x6);
        if (UVar2 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d5);
        }
        UVar1 = IsDlgButtonChecked(0xe2, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d6);
        }
        UVar1 = IsDlgButtonChecked(0xe3, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d7);
        }
        UVar1 = IsDlgButtonChecked(0xe5, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d8);
        }
        UVar1 = IsDlgButtonChecked(0xe6, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1e2);
        }
        UVar1 = IsDlgButtonChecked(0xe7, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1dc);
        }
        return;
    }
    DestroyWindow16(iVar3.hwnd_0x6);
    return;
}


pub unsafe fn win_cleanup_op_1040_748c(param_1: *mut u8, param_2: *mut astruct_898, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut puVar1: *mut u32;
    let mut iVar2: i16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut fn_ptr_1: *mut *mut code;

    match param_5 {
        0xfa => {
            fn_ptr_1 = (*param_2.field144_0x94 + 0x18);
            (**fn_ptr_1)();
        }
        // break;
        _ => {
            pass1_1040_b54a(param_1, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), param_4,
                            param_5);
            return;
        }
        0xfd => {
            if (DAT_1050_0ecc == 0) {
                return;
            }
            DAT_1050_0ecc = 0;
        }
// TODO: goto LAB_1040_755d;
        0xfe => {
            if (DAT_1050_0ecc == 1) {
                return;
            }
            DAT_1050_0ecc = 0x1;
        }
// TODO: goto LAB_1040_755d;
        0xff => {
            if (DAT_1050_0ecc == 0x2) {
                return;
            }
            DAT_1050_0ecc = 0x2;//
// LAB_1040_755d:
            puVar1 = param_2.field144_0x94;
            fn_ptr_1 = (*param_2.field144_0x94 + 0x10);
            (**fn_ptr_1)(&PTR_LOOP_1050_1040, puVar1, (puVar1 >> 0x10));
            pass1_1010_2ee2(param_2.field144_0x94);
            PostMessage16(0x0, 0x10a, 0x111, param_2.field6_0x6);
        }
        // break;
        0x107 => {
            iVar2 = 0;
        }
// TODO: goto LAB_1040_75ba;
        0x108 => {
            iVar2 = 0x1;//
// LAB_1040_75ba:
            win_ui_op_1010_3202(param_2.field144_0x94, iVar2);
        }
        // break;
        0x10a => {
            GetClientRect16(&local_a, &DAT_1050_1050);
            puVar1 = param_2.field144_0x94;
            local_a.y += 0x3;
            local_a.x = (puVar1 + 0x1a) - 0x9;
            iStack6 += -0x3;
            iStack4 += -0x3;
            InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
            unk_destroy_win_op_1010_2fa0(param_2.field144_0x94);
            pass1_1010_32c0(param_2.field144_0x94, 0x0);
            pass1_1010_2ee2(param_2.field144_0x94);
        }
        // break;
        0x10c => {
            DestroyWindow16(param_2.field6_0x6);
        }
    }
    return;
}

pub unsafe fn destroy_win_1008_9698(param_1: *mut astruct_871, mut param_2: u16) {
    DestroyWindow16(param_1.hwnd_0x8);
    return;
}


pub unsafe fn destroy_window_1010_7b26(mut param_1: u16, mut param_2: u32, param_3: i32) -> u32 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut UpuVar2: *mut c_char;
    let mut extraout_DX: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    uVar2 = (iVar3 + 0x1e) | (iVar3 + 0x1c);
    if (uVar2 != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar3 + 0x1c));
        loop {
            puVar2 = local_a;
            pass1_1008_5b12(CONCAT22(0x1050, puVar2));
            param_1 = extraout_DX | puVar2;
            if (param_1 == 0) {
                break;
            }
            if !((puVar2 + 0x4) != param_3) {
                break;
            }
        }
        uVar2 = extraout_DX | puVar2;
        if (uVar2 != 0) {
            uVar1 = (puVar2 + 0x8);
            uVar2 = DestroyWindow16((uVar1 + 0x6));
        }
    }
    return CONCAT22(uVar2, param_1);
}


pub unsafe fn unk_win_ui_op_1020_1418(
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
    get_sys_metrics_1020_7c1a(param_1, param_2);
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


pub unsafe fn window_op_1020_6c3a(
    param_1: *mut Struct57,
    struct_param_1: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
    mut param_9: u16,
    mut param_10: u16,
    mut param_11: u16,
    mut param_12: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut HVar3: HICON16;
    let mut paVar4: *mut Struct57;
    let mut pIVar5: *mut INT16 = null_mut();
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut paVar9: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar11: *mut u32;
    let mut uVar12: u8;
    let mut local_6: u32;
    let struct_a_1: *mut StructA;
    let mut paVar10: *mut Struct57;
    let mut struct_a_1_hi: u16;

    struct_a_1 = struct_param_1;
    struct_a_1_hi = (struct_param_1 >> 0x10);
    create_window_ex_1008_9760(struct_param_1);
    puVar11 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x4),
        param_3,
        param_5,
        param_6,
        param_7,
    );
    paVar9 = (param_1 & 0xffff0000 | puVar11 >> 0x10);
    struct_a_1[0x1].field20_0x26 = puVar11;
    uVar7 = (puVar11 >> 0x10);
    struct_a_1[0x1].field21_0x28 = uVar7;
    struct_a_1[0x1].field10_0x14 = struct_a_1[0x1].field20_0x26;
    struct_a_1[0x1].field11_0x16 = uVar7;
    HVar3 = LoadIcon16(s_TILEICON_1050_440c, HINSTANCE16_1050_038c);
    struct_a_1.field_0xc2 = HVar3;
    uVar6 = &struct_a_1[0x1].field20_0x26;
    ppcVar2 = (*&struct_a_1[0x1].field20_0x26 + 0x30);
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar6, (uVar6 >> 0x10), HVar3);
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbb8,
    );
    mem_op_1000_179c(0x42, paVar9);
    uVar8 = paVar9 | paVar4;
    paVar10 = (paVar9 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar9,
            0x0,
            local_6,
            0x0,
            0x7c007d,
            CONCAT22(struct_a_1.field4_0x8, 0xbb8),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbb9,
    );
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar9 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar9,
            paVar4,
            paVar10,
            0x0,
            local_6,
            0x0,
            0x7e007f,
            CONCAT22(struct_a_1.field4_0x8, 0xbb9),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbba,
    );
    mem_op_1000_179c(0x42, paVar9);
    uVar8 = paVar9 | paVar4;
    paVar10 = (paVar9 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar9,
            0x0,
            local_6,
            0x1b2,
            0x1b001b1,
            CONCAT22(struct_a_1.field4_0x8, 0xbba),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    mem_op_1000_179c(0x22, paVar10);
    uVar8 = paVar10 | paVar4;
    if (uVar8 == 0) {
        struct_a_1[0x1].field22_0x2a = 0;
    } else {
        unk_win_ui_op_1020_717e(uVar8, param_5, CONCAT22(paVar10, paVar4), struct_param_1);
        struct_a_1[0x1].field22_0x2a = paVar4;
        struct_a_1[0x1].field_0x2c = uVar8;
    }
    uVar6 = &struct_a_1[0x1].field22_0x2a;
    struct_a_1[0x1].field14_0x1c = uVar6;
    uVar1 = &struct_a_1[0x1].field20_0x26;
    ppcVar2 = (*&struct_a_1[0x1].field20_0x26 + 0x10);
    (**ppcVar2)(0x1000, uVar1, (uVar1 >> 0x10));
    pIVar5 = uVar6;
    MoveWindow16(
        0x1,
        pIVar5[0x3],
        pIVar5[0x2],
        pIVar5[0x1],
        *pIVar5,
        struct_a_1.field4_0x8,
    );
    uVar12 = (struct_param_1 >> 0x10);
    uVar6 = struct_param_1;
    ppcVar2 = (uVar6 + 0x94);
    (**ppcVar2)(s_tile2_bmp_1050_1538, struct_a_1, uVar12, 0x0);
    ppcVar2 = (uVar6 + 0x10);
    (**ppcVar2)(s_tile2_bmp_1050_1538, struct_a_1, uVar12, 1);
    UpdateWindow16(struct_a_1.field4_0x8);
    return;
}

pub unsafe fn mov_update_win_1040_93aa(param_1: *mut astruct_65, param_2: INT16, mut param_3: u16 )

{
  let mut iVar1: *mut astruct_65;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field14_0x1e = param_3;
  iVar1.field15_0x20 = param_2;
  MoveWindow16(0x1,iVar1.field17_0x24,iVar1.field16_0x22,param_2,iVar1.field14_0x1e,iVar1.field11_0x18);
  UpdateWindow16(iVar1.field11_0x18);
  return;
}

pub unsafe fn win_ui_op_1020_2cf0(
    param_1: *mut Struct57,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_160;
    let mut pIVar4: *mut INT16 = null_mut();
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let pSVar11: *mut StructA;
    let mut uVar12: u16;
    let iVar10: *mut StructA;

    pSVar11 = param_2;
    uVar12 = (param_2 >> 0x10);
    create_window_ex_1008_9760(param_2);
    puVar9 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, pSVar11[0x1].field26_0x30),
        param_3,
        param_4,
        param_5,
        param_6,
    );
    paVar8 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    pSVar11[0x1].field20_0x26 = puVar9;
    uVar5 = (puVar9 >> 0x10);
    pSVar11[0x1].field21_0x28 = uVar5;
    pSVar11[0x1].field10_0x14 = pSVar11[0x1].field20_0x26;
    pSVar11[0x1].field11_0x16 = uVar5;
    paVar3 = LoadIcon16(s_SITEICON_1050_428d, HINSTANCE16_1050_038c);
    pSVar11.field_0xc2 = paVar3;
    uVar1 = &pSVar11[0x1].field20_0x26;
    ppcVar2 = (*&pSVar11[0x1].field20_0x26 + 0x30);
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar1, (uVar1 >> 0x10), paVar3);
    mem_op_1000_179c(0x22, paVar8);
    uVar6 = paVar8 | paVar3;
    uVar10 = paVar8 & 0xffff0000 | uVar6;
    if (uVar6 == 0) {
        pSVar11[0x1].field22_0x2a = 0;
    } else {
        load_draw_op_1020_2ede(uVar10, param_7, CONCAT22(paVar8, paVar3), pSVar11, uVar12);
        pSVar11[0x1].field22_0x2a = paVar3;
        pSVar11[0x1].field_0x2c = uVar10;
    }
    pSVar11[0x1].field14_0x1c = &pSVar11[0x1].field22_0x2a;
    pass1_1018_0ac0(&pSVar11[0x1].field20_0x26, param_2);
    uVar10 = pass1_1018_0b08(&pSVar11[0x1].field20_0x26);
    uVar7 = (uVar10 >> 0x10);
    pIVar4 = uVar10;
    ppcVar2 = (param_2 + 0x14);
    (**ppcVar2)();
    ppcVar2 = (*&pSVar11[0x1].field20_0x26 + 0x10);
    (**ppcVar2)();
    MoveWindow16(
        0x1,
        pIVar4[0x3],
        pIVar4[0x2],
        pIVar4[0x1],
        *pIVar4,
        pSVar11.field4_0x8,
    );
    pass1_1008_3e0e(param_2);
    UpdateWindow16(pSVar11.field4_0x8);
    return;
}

pub unsafe fn win_ui_cursor_op_1020_1294(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: i16,
    mut param_4: i16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut in_AX: u16;
    let mut hcursor: HCURSOR16;
    let mut hcursor_00: HCURSOR16;
    let mut in_register_0000000a: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000ffc2: u16;
    let mut local_12: i16;
    let mut local_10: i16;
    let mut puStack14: *mut u16;
    let mut puStack10: *mut u32;
    let mut local_6: i16;
    let mut iStack4: i16;

    pass1_1030_8344(_u16_1050_5748, 0x4000001);
    if ((param_1 | in_AX) == 0) {
        local_6 = param_4;
        iStack4 = param_3;
        uVar5 = (param_2 >> 0x10);
        iVar4 = param_2;
        puStack10 = pass1_1010_40cc(param_3, 0x0, (iVar4 + 0xf2));
        uVar3 = CONCAT22(in_register_0000000a, (puStack10 >> 0x10));
        uVar1 = (iVar4 + 0xf2);
        puStack14 = (uVar1 & 0xffff0000 | (uVar1 + 0x76));
        pass1_1008_3e94(
            puStack14,
            CONCAT22(0x1050, &local_12),
            CONCAT22(0x1050, &local_10),
        );
        local_6 -= local_10;
        iStack4 -= local_12;
        hcursor = pt_in_rect_1010_40f8(
            uVar3,
            (iVar4 + 0xf2),
            CONCAT22(0x1050, &local_6),
            in_stack_0000ffc2,
        );
        if (hcursor != 0xffff) {
            hcursor_00 = LoadCursor16(0x7f02, 0x0);
            SetCursor16(hcursor_00);
            ppcVar2 = (*puStack10 + 0x4);
            (**ppcVar2)();
            pass1_1008_3e0e(param_2);
            SetCursor16(hcursor);
        }
    }
    return;
}


pub unsafe fn win_ui_op_1020_5de8(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
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
    pt_in_rect_1020_5856(puVar2, uVar3, param_1, CONCAT22(0x1050, puVar2));
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


pub unsafe fn mix_ui_op_1018_6adc(
    param_1: *mut astruct_28,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar6: u32;
    let mut pstruct28_1: *mut astruct_28;
    let mut uVar5: *mut astruct_28;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    puVar8 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x48),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    uVar6 = puVar8 >> 0x10;
    uVar7 = (puVar8 >> 0x10);
    iVar1 = (puVar8 + 0xa);
    iVar2 = (puVar8 + 0xc);
    uVar5 = (param_1 >> 0x10);
    pstruct28_1 = param_1;
    if (0x1 < iVar1 - pstruct28_1.field227_0xe6) {
        uVar6 = (iVar1 >> 0xf);
        pstruct28_1.field225_0xe2 = iVar1 / 0x2 - (pstruct28_1.field227_0xe6 + 1) / 0x2;
    }
    if (0x1 < iVar2 - pstruct28_1.field228_0xe8) {
        uVar6 = (iVar2 >> 0xf);
        pstruct28_1.field226_0xe4 = iVar2 / 0x2 - (pstruct28_1.field228_0xe8 + 1) / 0x2;
    }
    uVar6 = paVar4 & 0xffff0000 | uVar6;
    uVar7 = SUB42(s_tile2_bmp_1050_1538, 0x0);
    uVar3 = ShowCursor16(0x0);
    if (pstruct28_1.field231_0xee != 0) {
        uVar7 = 0x1008;
        win_1008_5c5c(uVar3, uVar6, _u16_1050_02a0, pstruct28_1.field231_0xee);
        pstruct28_1.hwnd_0xf0 = uVar3;
    }
    uVar7 = FUN_1010_830a(
        uVar3,
        uVar6,
        uVar7,
        _u16_1050_14cc,
        pstruct28_1.field230_0xec,
    );
    mci_send_command_1008_53ae(CONCAT22(uVar6, uVar7), pstruct28_1.field8_0x8);
    ShowCursor16(1);
    unk_destroy_window_op_1018_6bb6(param_1);
    return;
}

pub unsafe fn pt_in_rect_1010_40f8(
    param_1: *mut Struct57,
    mut param_2: u32,
    param_3: *mut POINT16,
    mut param_4: u16,
) -> i16 {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut puVar12: *mut u32;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc2: u16;
    let mut puStack16: *mut u32;
    let mut iStack6: i16;
    let mut uStack4: u16;
    let mut uVar9: u32;

    iStack6 = 0;
    uStack4 = 0;
    loop {
        uVar11 = (param_2 >> 0x10);
        iVar10 = param_2;
        piVar1 = (iVar10 + 0x74);
        if (*piVar1 == iStack6 || *piVar1 < iStack6) {
            //
            // LAB_1010_413e:
            if ((uStack4 != 0) && (0x3 < PTR_LOOP_1050_3960)) {
                puVar12 = mixed_1010_20ba(
                    param_1,
                    _u16_1050_0ed0,
                    CONCAT22(param_4, iStack6 + 0xc),
                    in_stack_0000fe94,
                    in_stack_0000ffb8,
                    in_stack_0000ffbe,
                    in_stack_0000ffc2,
                );
                paVar8 = (param_1 & 0xffff0000 | puVar12 >> 0x10);
                uVar4 = pass1_1018_0afa(puVar12);
                if (uVar4 == 0) {
                    uVar11 = 0x1000;
                    uVar5 = uVar4;
                    mem_op_1000_179c(0xb4, paVar8);
                    uVar6 = paVar8 | uVar5;
                    uVar9 = paVar8 & 0xffff0000 | uVar6;
                    if (uVar6 == 0) {
                        iVar10 = 0;
                        uVar7 = 0;
                    } else {
                        uVar11 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                        iVar10 = string_1040_8520(
                            uVar9,
                            CONCAT22(paVar8, uVar5),
                            HWND16_1050_0396,
                            0x20030,
                            0x6450643,
                        );
                        uVar7 = uVar9;
                    }
                    puStack16 = CONCAT22(uVar7, iVar10);
                    ppcVar2 = (*puStack16 + 0x74);
                    (**ppcVar2)(uVar11, iVar10, uVar7);
                    pass1_1010_209e(_u16_1050_0ed0, iStack6 + 0xc);
                    uStack4 = uVar4;
                }
            }
            if (uStack4 != 0) {
                return iStack6;
            }
            return -0x1;
        }
        param_1 = (param_1 & 0xffff0000 | (iVar10 + 0x72));
        BVar3 = PtInRect16(
            *param_3,
            ((iStack6 * 0x10 + (iVar10 + 0x24)) * 0x8 + (iVar10 + 0x70)),
        );
        if (BVar3 != 0) {
            uStack4 = 0x1;
            // TODO: goto LAB_1010_413e;
        }
        iStack6 += 0x1;
    }
}


pub unsafe fn pt_in_rect_1018_1bda(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut BVar4: bool;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut iStack26: i16;
    let mut POPStack24: INT16;
    let mut local_14: i16;
    let mut local_12: i16;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut local_a: u32;
    let mut iStack6: i16;
    let mut iStack4: i16;

    uStack14 = 0;
    iVar3 = param_1;
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (iVar3 + 0x3a)),
        CONCAT22(0x1050, &local_14),
        CONCAT22(0x1050, &local_12),
    );
    PStack24 = CONCAT22(param_2, param_3);
    uStack16 = 0;
    iStack26 = 0;
    loop {
        uVar6 = (param_1 >> 0x10);
        piVar1 = (iVar3 + 0x44);
        if (*piVar1 == iStack26 || *piVar1 < iStack26) {
            return;
        }
        uVar2 = (iVar3 + 0x42);
        iVar5 = (iVar3 + 0x40) + iStack26 * 0x18;
        uStack14 = CONCAT22(uVar2, iVar5);
        pass1_1008_3e94(
            CONCAT22(uVar2, iVar5),
            CONCAT22(0x1050, &local_a + 0x2),
            CONCAT22(0x1050, &local_a),
        );
        local_a += local_12 - 0x6;
        iStack6 = local_a + 0xc;
        local_a += local_14 - 0x6;
        iStack4 = local_a + 0xc;
        BVar4 = PtInRect16(PStack24, &local_a);
        if (BVar4 != 0) {
            break;
        }
        iStack26 += 0x1;
    }
    block_1018_1000::pass1_1018_1eda(param_1, uStack14);
    return;
}

pub unsafe fn pt_in_rect_1020_5856(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut POINT16,
) {
    let mut puVar1: *mut u32;
    let mut BVar2: bool;
    let mut uVar3: u32;
    let mut extraout_DX: u16;
    let mut uStack10: u32;

    pass1_1018_2862((param_3 + 0xfa));
    if ((param_2 | param_1) != 0) {
        uStack10 = 0;
        loop {
            puVar1 = (param_1 + 0xa);
            if (*puVar1 < uStack10 || *puVar1 == uStack10) {
                break;
            }
            uVar3 = uStack10;
            empty_1008_8fc4();
            if ((extraout_DX | uVar3) != 0) {
                BVar2 = PtInRect16(*param_4, (uVar3 + 0x14));
                if (BVar2 != 0) {
                    return;
                }
            }
            uStack10 += 0x1;
        }
    }
    return;
}


pub unsafe fn pt_in_rect_op_1020_58ce(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: u16,
    param_5: u8,
) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut BVar6: bool;
    let mut puVar7: *mut u16;
    let mut uVar8: u16;
    let mut in_register_0000000a: u16;
    let mut paVar9: *mut Struct57;
    let mut iVar10: i16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u32;
    let mut puVar15: *mut u16;
    let mut puVar16: *mut u16;
    let mut puVar17: *mut u32;
    let mut in_stack_0000fe74: u16;
    let mut in_stack_0000ff98: u16;
    let mut in_stack_0000ff9e: u16;
    let mut in_stack_0000ffa2: u16;
    let mut wparam: WPARAM16;
    let mut in_stack_0000ffcc: u16;
    let mut uStack46: u16;
    let mut iStack26: i16;
    let mut local_18: [u16; 0x2] = [0; 0x2];
    let mut uStack20: u16;
    let mut uStack18: u32;
    let mut pRStack14: *mut RECT16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    paVar9 = CONCAT22(in_register_0000000a, param_1);
    local_6 = param_4;
    uStack4 = param_3;
    uStack8 = param_5 & 0x8;
    uStack10 = param_5 & 0x4;
    uVar12 = (param_2 >> 0x10);
    iVar10 = param_2;
    uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 0x2);
    if (uVar5 == 0) {
        //
        // LAB_1020_5942:
        uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 0x4);
        if (uVar5 == 0) {
            //
            // LAB_1020_5a16:
            uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 1);
            if (uVar5 != 0) {
                uVar14 = pass1_1020_6498((iVar10 + 0xf6), 1);
                paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
                for iStack26 in 0..0x4 {
                    paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
                    BVar6 = PtInRect16(CONCAT22(uStack4, local_6), (iStack26 * 0x8 + uVar14));
                    if (BVar6 != 0) {
                        local_18[0] = 0;
                        uStack20 = 0;
                        if (iStack26 == 0) {
                            uStack20 = (-(uStack10 == 0) & 0x4) - 0x5;
                        } else if (iStack26 == 1) {
                            uStack20 = (-(uStack10 == 0) & 0xfffc) + 0x5;
                        } else if (iStack26 == 0x2) {
                            local_18[0] = (-(uStack10 == 0) & 0x4) - 0x5;
                        } else if (iStack26 == 0x3) {
                            local_18[0] = (-(uStack10 == 0) & 0xfffc) + 0x5;
                        }
                        pass1_1020_2a94((iVar10 + 0xce), CONCAT22(local_18[0], uStack20));
                        return;
                    }
                }
            }
            uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 0x3);
            if (uVar5 != 0) {
                puVar7 = &local_6;
                pt_in_rect_1020_5856(puVar7, paVar9, param_2, CONCAT22(0x1050, puVar7));
                uVar8 = paVar9;
                uVar14 = (uVar8 | puVar7);
                if ((uVar8 | puVar7) != 0) {
                    uVar5 = puVar7[0x17];
                    if (((uStack8 == 0) || (uStack10 == 0)) && (uStack10 == 0)) {
                        local_18[0] = 0x1;
                    } else {
                        local_18[0] = 0x2;
                    }
                    uStack20 = puVar7[0x6];
                    uStack18 = CONCAT22(uStack18, puVar7[0x7]);
                    if ((uVar5 == 0xb) || (uVar5 == 0x37)) {
                        uVar4 = (iVar10 + 0xfa);
                        uVar13 = (uVar4 >> 0x10);
                        iVar11 = uVar4;
                        uVar2 = (iVar11 + 0x20);
                        uVar1 = (iVar11 + 0x22);
                        uVar14 = paVar9 & 0xffff0000 | uVar1;
                        uStack46 = uVar2;
                        if ((uVar1 | uStack46) != 0) {
                            puVar16 = pass1_1008_3e38(CONCAT22(0x1050, &stack0xffcc));
                            paVar9 = (uVar14 & 0xffff0000 | puVar16 >> 0x10);
                            pass1_1018_161c(
                                uVar2,
                                CONCAT22(0x1050, &stack0xffcc),
                                uStack18,
                                uStack20,
                            );
                            puVar17 = mixed_1010_20ba(
                                paVar9,
                                _u16_1050_0ed0,
                                CONCAT22(in_stack_0000ffcc, 0x2f),
                                in_stack_0000fe74,
                                in_stack_0000ff98,
                                in_stack_0000ff9e,
                                in_stack_0000ffa2,
                            );
                            uVar14 = puVar17 >> 0x10;
                            pass1_1010_ecc6(
                                &stack0xffcc,
                                (puVar17 >> 0x10),
                                puVar17,
                                CONCAT22(0x1050, &stack0xffcc),
                                (uStack46 + 0x3c),
                            );
                        }
                    }
                    uVar13 = uVar14;
                    uVar5 = pass1_1018_25d2(
                        (iVar10 + 0xfa),
                        local_18[0],
                        uStack18 & 0xffff | uStack20 << 0x10,
                    );
                    if (uVar5 != 0) {
                        return;
                    }
                    uVar5 = block_1020_5000::pass1_1020_5d56(uVar13, param_2, CONCAT22(uVar8, puVar7));
                    if (uVar5 != 0) {
                        return;
                    }
                }
            }
            return;
        }
        uVar14 = pass1_1020_6498((iVar10 + 0xf6), 0x4);
        paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
        pRStack14 = uVar14;
        uStack12 = (uVar14 >> 0x10);
        BVar6 = PtInRect16(CONCAT22(uStack4, local_6), pRStack14);
        //    if (BVar6 == 0) goto LAB_1020_5a16;
        uStack18 = mixed_1010_20ba(
            paVar9,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffcc, 0x2),
            in_stack_0000fe74,
            in_stack_0000ff98,
            in_stack_0000ff9e,
            in_stack_0000ffa2,
        );
        if ((uStack18 + 0x72) != 0) {
            (iVar10 + 0x116) = 0x1;
            if (param_2 == 0) {
                iVar10 = 0;
                uVar12 = 0;
            } else {
                iVar10 += 0xe2;
            }
            ppcVar3 = (*_u16_1050_02a0 + 0x4);
            (**ppcVar3)(
                0x1010,
                _u16_1050_02a0,
                (_u16_1050_02a0 >> 0x10),
                0x10,
                iVar10,
                uVar12,
            );
            puVar15 = pass1_1008_941a(CONCAT22(0x1050, local_18), 0x1, 0x86);
            puVar7 = local_18;
            win_1008_5c9e(
                puVar7,
                (puVar15 >> 0x10),
                _u16_1050_02a0,
                CONCAT22(0x1050, puVar7),
            );
            if (puVar7.is_null() == false) {
                return;
            }
            wparam = 0xf6;
            // TODO: goto LAB_1020_5936;
        }
        wparam = 0xf6;
    } else {
        uVar14 = pass1_1020_6498((iVar10 + 0xf6), 0x2);
        paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
        pRStack14 = uVar14;
        uStack12 = (uVar14 >> 0x10);
        BVar6 = PtInRect16(CONCAT22(uStack4, local_6), pRStack14);
        //    if (BVar6 == 0) goto LAB_1020_5942;
        wparam = 0x68;
    }
    puVar7 = null_mut(); //
    // LAB_1020_5936:
    PostMessage16(CONCAT22(puVar7, puVar7), wparam, 0x111, HWND16_1050_0396);
    return;
}


pub unsafe fn pt_in_rect_1010_4e08(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut bVar2: bool;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut POPStack8: INT16;

    PStack8 = CONCAT22(param_2, param_3);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    (iVar4 + 0x22) = (iVar4 + 0x20);
    bVar2 = false;
    (iVar4 + 0x24) = 0;
    iStack12 = 0;
    iStack10 = 0;
    loop {
        piVar1 = (iVar4 + 0x30);
        if (*piVar1 == iStack12 || *piVar1 < iStack12) {
            //
            // LAB_1010_4e67:
            if (iStack10 != 0) {
                (iVar4 + 0x20) = iStack10;
            }
            if (bVar2) {
                return;
            }
            return;
        }
        BVar3 = PtInRect16(PStack8, ((iVar4 + 0x1a) + iStack12 * 0x8));
        if (BVar3 != 0) {
            iStack10 = iStack12;
            bVar2 = true;
            // TODO: goto LAB_1010_4e67;
        }
        iStack12 += 0x1;
    }
}

pub unsafe fn pt_in_rect_1020_68fc(param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut POPStack6: INT16;

    PStack6 = CONCAT22(param_2, param_3);
    uVar4 = (param_1 >> 0x10);
    uVar2 = pass1_1018_31d0((param_1 + 0xf2));
    if (uVar2 != 0) {
        BVar3 = PtInRect16(PStack6, ((param_1 + 0xf2) + 0x16c));
        if (BVar3 != 0) {
            ppcVar1 = (*param_1 + 0x40);
            (**ppcVar1)(s_tile2_bmp_1050_1538, param_1, 0xef);
        }
    }
    return;
}


pub unsafe fn win_msg_op_1008_9498() -> WPARAM16 {
    let mut BVar1: bool;
    let mut IVar2: i16;
    let mut local_msg_1: MSG16; //
                                //
                                // LAB_1008_949c:
    BVar1 = GetMessage16(0x0, 0x0, 0x0, &local_msg_1);
    if (BVar1 == 0) {
        return local_msg_1.wparam;
    }
    //   if ((_u16_1050_5bc8 + 0x8) != 0) goto code_r0x100894cd;
    //   goto LAB_1008_94dc;
    // code_r0x100894cd:
    BVar1 = IsDialogMessage16(&local_msg_1, &DAT_1050_1050);
    if (BVar1 == 0) {
        //
        // LAB_1008_94dc:
        if (PTR_LOOP_1050_0398.is_null() == false) {
            IVar2 = TranslateAccelerator16(
                &local_msg_1,
                (HACCEL16) & DAT_1050_1050,
                PTR_LOOP_1050_0398,
            );
            //   if (IVar2 != 0) goto LAB_1008_949c;
        }
        TranslateMessage16(&local_msg_1);
        DispatchMessage16(&local_msg_1);
    }
    //   goto LAB_1008_949c;
}


pub unsafe fn send_dlg_item_msg_1038_844a(param_1: *mut Struct903) -> LRESULT

{
  let mut hwnd: HWND16;
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: *mut Struct903;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut local_108: [u8;0x102] = [0;0x102];
  let mut uStack6: u32;

  uVar5 = (param_1 >> 0x10);
  uVar4 = param_1;
  SendDlgItemMessage16(0x0,0x0,0xb,0x1854,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1855,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1856,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x1854,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x1855,uVar4.field6_0x6);
  LVar6 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,uVar4.field6_0x6);
  uStack6 = pass1_1008_b820(LVar6,(LVar6 >> 0x10),uVar4.field147_0x94);
  uVar2 = (uStack6 >> 0x10) | uStack6;
  if (uStack6 == 0) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_108,&DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,local_108),0x0,0x401,0x1854,uVar4.field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4.field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4.field6_0x6);
    LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4.field6_0x6);
    uVar3 = (LVar6 >> 0x10);
    hwnd = GetDlgItem16(0x1857,uVar4.field6_0x6);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_108,&DAT_1050_1050);
    BVar1 = SetWindowText16(CONCAT22(0x1050,local_108),hwnd);
    return CONCAT22(uVar3,BVar1);
  }
  send_dlg_item_msg_1038_8400(uVar4, uVar5, uStack6, 0x1854);
  set_win_text_1038_8358(uVar2, param_1);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4.field6_0x6);
  LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4.field6_0x6);
  return LVar6;
}


pub unsafe fn send_dlg_item_msg_1038_8f74(param_1: *mut Struct903) -> LRESULT

{
  let mut uVar1: u32;
  let mut iVar3: i16;
  let mut hwnd: HWND16;
  let mut iVar2: *mut Struct903;
  let mut uVar4: u16;
  let mut lVar4: i32;
  let mut LVar5: LRESULT;
  let mut enable: bool;
  let mut local_50c: [u16;0x80] = [0;0x80];
  let mut local_40c: [u8;0x8] = [0;0x8];
  let mut local_404: u32;

  uVar4 = (param_1 >> 0x10);
  iVar2 = param_1;
  SendDlgItemMessage16(0x0,0x0,0xb,0x185b,iVar2.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x185b,iVar2.field6_0x6);
  iVar3 = pass1_1008_c83a(iVar2.field147_0x94);
  if (iVar3 == 0) {
    local_404 = pass1_1008_c85e(iVar2.field147_0x94);
    pass1_1008_5784(CONCAT22(0x1050,local_40c),local_404);
    loop {
      lVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_40c));
      if (lVar4 == 0) { break; }
      uVar1 = (lVar4 + 0x4);
      wsprintf16(local_50c,0x5a6c1050,CONCAT22(uVar1,0x1050),(uVar1 >> 0x10));
      SendDlgItemMessage16(CONCAT22(0x1050,local_50c),0x0,0x401,0x185b,iVar2.field6_0x6);
    }
    hwnd = GetDlgItem16(0x1,iVar2.field6_0x6);
    enable = 0x1;
  }
  else {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,&local_404,
               &DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,&local_404),0x0,0x401,0x185b,iVar2.field6_0x6);
    hwnd = GetDlgItem16(0x1,iVar2.field6_0x6);
    enable = 0;
  }
  EnableWindow16(enable,hwnd);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0xb,0x185b,iVar2.field6_0x6);
  return LVar5;
}


pub unsafe fn enable_win_1040_060e(mut param_1: u32, mut param_2: i16) {
    let mut pIVar1: *mut INT16 = null_mut();
    let mut hwnd: HWND16;
    let mut iStack10: i16;
    let mut pIStack8: *mut INT16 = null_mut();

    pIStack8 = CONCAT22(0x1050, &stack0x000a);
    iStack10 = param_2;
    loop {
        pIVar1 = pIStack8;
        if (iStack10 == 0) {
            break;
        }
        pIStack8 = (pIStack8 & 0xffff0000 | (pIStack8 + 0x2));
        hwnd = GetDlgItem16(*pIVar1, (param_1 + 0x6));
        EnableWindow16(0x0, hwnd);
        iStack10 = iStack10 - 0x1;
    }
    return;
}


pub unsafe fn win_ui_dlg_op_1040_a94a(mut param_1: u16, mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut value: *mut u8;
  let mut pcVar6: *mut c_char;
  let mut uVar7: u16;
  let mut HVar8: HWND16;
  let mut value_00: u16;
  let mut puVar9: *mut u8;
  let mut in_register_0000000a: u16;
  let mut uVar10: u32;
  let mut iVar11: i16;
  let mut iVar12: i16;
  let mut unaff_SI: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut bVar15: bool;
  let mut puVar16: *mut u32;
  let mut uVar17: u32;
  let mut in_stack_0000fd7c: u16;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000feaa: u16;
  let mut uStack288: u16;
  let mut iStack278: i16;
  let mut iStack276: i16;
  let mut local_102: [u8;0x100] = [0;0x100];

  puVar16 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            CONCAT22(unaff_SI,0x3),in_stack_0000fd7c,in_stack_0000fea0,in_stack_0000fea6,
                            in_stack_0000feaa);
  puVar4 = (puVar16 >> 0x10);
  uVar5 = puVar16;
  uVar13 = (param_2 >> 0x10);
  iVar11 = param_2;
  puVar9 = puVar4;
  pass1_1010_c3c2(puVar4,uVar5,puVar4,CONCAT22(0x1050,local_102),(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1f2,(iVar11 + 0x6));
  pass1_1010_c320(puVar9,uVar5,puVar4,CONCAT22(0x1050,local_102),(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1774,(iVar11 + 0x6));
  string_op_1010_c446(puVar9,puVar16,CONCAT22(0x1050,local_102),(iVar11 + 0x94));
  value = local_102;
  SetDlgItemText16(CONCAT22(0x1050,value),0x1773,(iVar11 + 0x6));
  pass1_1030_6ddc((iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f5,(iVar11 + 0x6));
  pass1_1030_6e14((iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f6,(iVar11 + 0x6));
  if ((iVar11 + 0x98) != 0) {
    struct_1010_dd5e(uVar5,puVar4,(iVar11 + 0x94));
    if ((puVar9 | value) != 0) {
      uVar3 = (iVar11 + 0x94);
      uVar14 = (uVar3 >> 0x10);
      iVar12 = uVar3;
      uVar2 = (iVar12 + 0x26);
      uVar10 = (iVar12 + 0x28);
      iStack276 = 0x1798;
      iStack278 = 0x17c3;
      (iVar11 + 0xea) = 0;
      uVar17 = uVar2;
    //   for (uStack288 = 0x1; uStack288 < 0x25; uStack288 += 1)
      for uStack288 in 1 .. 0x25
      {
        if (uVar2 == 0) {
          value_00 = 0;
          uVar10 = 0;
        }
        else {
          uVar17 = pass1_1020_bae6(uVar17,uVar10,uVar2,CONCAT22(uStack288,(uVar2 >> 0x10)));
          uVar10 = uVar17 >> 0x10;
          value_00 = uVar17;
        }
        uVar7 = uVar10;
        bVar15 = (value + uStack288 * 0x4) != 0;
        if (bVar15) {
          pcVar6 = string_1020_c0d8(uStack288);
          SetDlgItemText16(CONCAT22(uVar10,pcVar6),iStack276,(iVar11 + 0x6));
          SetDlgItemInt16(0x0,(value + uStack288 * 0x4),iStack278,(iVar11 + 0x6));
        }
        uVar7 |= value_00;
        if (uVar7 != 0) {
          if (!bVar15) {
            pcVar6 = string_1020_c0d8(uStack288);
            SetDlgItemText16(CONCAT22(uVar10,pcVar6),iStack276,(iVar11 + 0x6));
          }
          SetDlgItemInt16(0x0,value_00,iStack278,(iVar11 + 0x6));
          iVar12 = (iVar11 + 0xea);
          HVar8 = GetDlgItem16(iStack276,(iVar11 + 0x6));
          (iVar11 + iVar12 * 0x2 + 0x9a) = HVar8;
          piVar1 = (iVar11 + 0xea);
          *piVar1 = *piVar1 + 1;
          iVar12 = (iVar11 + 0xea);
          uVar7 = GetDlgItem16(iStack278,(iVar11 + 0x6));
          (iVar11 + iVar12 * 0x2 + 0x9a) = uVar7;
          piVar1 = (iVar11 + 0xea);
          *piVar1 = *piVar1 + 1;
          bVar15 = true;
        }
        uVar17 = uVar7;
        if (bVar15) {
          iStack276 += 0x1;
          iStack278 += 0x1;
        }
      }
    }
  }
  return;
}


pub unsafe fn enable_window_1038_9cec(param_1: *mut u8, param_2: *mut Struct903, mut param_3: u16, mut param_4: u16, mut param_5: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut enable: bool;
  let mut HVar6: HWND16;
  let mut iStack12: i16;
  let mut iVar2: *mut astruct_905;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(0xeb,param_4));
    puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(unaff_SI,0x3),in_stack_0000fe90,
                             in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
    iVar3 = puVar5 + 0xa4;
    uVar2 = (puVar5 >> 0x10);
    iStack12 = 0;
    while (iVar2 = (iStack12 * 0x2), (iVar2 + iVar3) != 0) {
      HVar6 = GetDlgItem16((iVar2 + iVar3),(param_2 + 0x6));
      (iVar2 + param_2 + 0x94) = HVar6;
      iStack12 += 0x1;
      piVar1 = (param_2 + 0x128);
      *piVar1 = *piVar1 + 1;
    }
  }
  else {
    if (param_5 == 0xf8) {
      HVar6 = GetDlgItem16(0x17d8,(param_2 + 0x6));
      enable = 0x1;
    }
    else {
      if (param_5 != 0x17d8) {
        pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(param_5,param_4));
        return;
      }
      SetWindowPos16(0x6,0xed,0x237,0x0,0x0,0x0,(param_2 + 0x6));
      enable = s_tile2_bmp_1050_1538;
      GetDlgItem16(0x17d8,(param_2 + 0x6));
      HVar6 = 0;
    }
    EnableWindow16(enable,HVar6);
  }
  return;
}


pub unsafe fn win_ui_op_1020_6ae6(
    param_1: *mut astruct_877,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
    param_5: u16,
    param_6: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut hwnd: HWND16;
    let mut puVar2: *mut u8;
    let mut iVar3: *mut astruct_877;
    let mut uVar3: u16;
    let mut LVar4: LRESULT;

    if (param_4 == 0x1797) {
        uVar3 = (param_1 >> 0x10);
        iVar3 = param_1;
        hwnd = GetDlgItem16(0x1797, iVar3.field8_0x8);
        if (hwnd != 0) {
            if (param_3 == 0x2) {
                LVar4 = SendMessage16(0x0, 0x0, 0x409, hwnd);
                if (LVar4 != -1) {
                    LVar4 = SendMessage16(
                        CONCAT13(0x10, CONCAT12(0x50, &stack0xffa8)),
                        LVar4,
                        0x40a,
                        hwnd,
                    );
                    puVar2 = &stack0xffa8;
                    pass1_1018_30ca(
                        (LVar4 >> 0x10),
                        iVar3.field241_0xf2,
                        CONCAT22(0x1050, puVar2),
                    );
                    pass1_1018_2fe8(iVar3.field241_0xf2, param_5, param_6);
                    if (puVar2.is_null() == false) {
                        invalidate_rect_1020_735a(iVar3.field242_0xf6);
                        ppcVar1 = (param_1 + 0x40);
                        (**ppcVar1)(0x1018, iVar3);
                    }
                }
            } else if (param_3 != 0x3) {
                return;
            }
            DestroyWindow16(hwnd);
        }
    }
    return;
}


pub unsafe fn enable_menu_item_1020_6b9a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: HMENU16,
) {
    if (param_4 != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0x0, param_5);
    return;
}


pub unsafe fn win_ui_op_1040_ae04(mut param_1: u16, mut param_2: u32)

{
  let mut iVar1: i16;
  let mut bVar2: bool;
  let mut pWVar3: *mut WORD;
  let mut iVar4: i16;
  let mut pcVar5: *mut c_char;
  let mut lVar6: i32;
  let mut puVar7: *mut u8;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut iVar9: i16;
   let mut plVar10: *mut i32;
  let mut unaff_SI: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut puVar13: *mut u32;
  let mut uVar14: u32;
  let mut uVar15: u32;
  let mut lp_string: *mut c_char;
  let mut uVar16: u32;
  let mut in_stack_0000fd84: u16;
  let mut in_stack_0000fea8: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb2: u16;
  let mut iStack280: i16;
  let mut local_102: [u16;0x80] = [0;0x80];

  puVar13 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            CONCAT22(unaff_SI,0x3),in_stack_0000fd84,in_stack_0000fea8,in_stack_0000feae,
                            in_stack_0000feb2);
  puVar7 = (puVar13 >> 0x10);
  uVar11 = (param_2 >> 0x10);
  iVar9 = param_2;
  pass1_1010_c3c2(puVar7,puVar13,puVar7,CONCAT22(0x1050,local_102),(iVar9 + 0x94));
  pWVar3 = local_102;
  SetDlgItemText16(CONCAT22(0x1050,pWVar3),0x1778,(iVar9 + 0x6));
  uVar14 = struct_op_1030_73a8((iVar9 + 0x94),pWVar3,puVar7);
  iVar4 = uVar14 + 0x20;
  uVar15 = pass1_1030_8326();
  uVar16 = uVar15 >> 0x10;
  iVar1 = 0;
  bVar2 = false;
//   for (iStack280 = 0; uVar8 = uVar16, iStack280 < 0xa; iStack280 += 1)
  iStack280 = 0;
  uVar8 = uVar16;
  while iStack280 < 0xa
  {
    uVar12 = ((uVar14 & 0xffff0000) >> 0x10);
    if (((iStack280 * 0xc + iVar4 + 0x2) | (iStack280 * 0xc + iVar4)) != 0) {
      plVar10 = (iStack280 * 0xc + iVar4);
      pcVar5 = string_op_1020_c222((plVar10 + 1));
      SetDlgItemText16(CONCAT22(uVar8,pcVar5),iVar1 + 0x1d2,(iVar9 + 0x6));
      lVar6 = *plVar10 - uVar15;
      wsprintf16(local_102,0x5ef41050,0xf4,CONCAT22(lVar6,0x1050),(lVar6 >> 0x10));
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1dc,(iVar9 + 0x6));
      uVar16 = unk_load_str_op_1010_8c96
                         (uVar8,(iVar9 + 0x98),CONCAT22(0x1050,local_102),
                          uVar14 & 0xffff0000 | ZEXT24(plVar10));
      uVar16 &= 0xffff;
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1e6,(iVar9 + 0x6));
      iVar1 += 0x1;
      bVar2 = true;
    }
    iStack280 += 1;
  }
  if (!bVar2) {
    lp_string = load_string_1010_847e(_u16_1050_14cc,0x531);
    SetDlgItemText16(lp_string,0x1d2,(iVar9 + 0x6));
  }
  return;
}


pub unsafe fn pass1_1040_b45e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut piVar2: *mut i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut iStack8: i16;
  let mut puStack6: *mut u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x90) != 0) {
    uVar1 = (iVar3 + 0x90);
    (uVar1 + 0x14) = (iVar3 + 0x6);
    uVar1 = (iVar3 + 0x90);
    puStack6 = (uVar1 + 2);
    // for (iStack8 = 0; piVar2 = (iVar3 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 += 1)
    iStack8 = 0;
    piVar2 = iVar3 + 0x90;
    while *piVar2 != iStack8 && iStack8 <= *piVar2
    {
      uVar1 = (puStack6 + 2);
      SetDlgItemText16(CONCAT22(uVar1,*puStack6),(uVar1 >> 0x10),(iVar3 + 0x6));
      puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
      iStack8 += 1;
    }
  }
  return;
}


pub unsafe fn send_dlg_item_msg_1038_8b58(param_1: *mut Struct903)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u8;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut LVar7: LRESULT;
  let mut in_stack_0000fd96: u16;
  let mut in_stack_0000feba: u16;
  let mut in_stack_0000fec0: u16;
  let mut in_stack_0000fec4: u16;
  let mut in_stack_0000feee: u16;
  let mut local_106: [u8;0x100] = [0;0x100];
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000feee,0x3),in_stack_0000fd96,
                             in_stack_0000feba,in_stack_0000fec0,in_stack_0000fec4);
  puVar2 = (puStack6 >> 0x10);
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  pass1_1010_c3c2(puVar2,puStack6,puVar2,CONCAT22(0x1050,local_106),(iVar5 + 0x94));
  LVar7 = SendDlgItemMessage16
                    (CONCAT22(0x1050,local_106),0x0,0xc,s_dibtext_bmp_1050_1844 + 0x2,(iVar5 + 0x6));
  uVar4 = (LVar7 >> 0x10);
  uVar1 = (iVar5 + 0x94);
  (iVar5 + 0x9c) = (uVar1 + 0x32);
  (iVar5 + 0x9a) = (iVar5 + 0x9c);
  SetDlgItemInt16(0x0,(iVar5 + 0x9c),s_dibtext_bmp_1050_1844 + 0x9,(iVar5 + 0x6));
  uVar1 = (iVar5 + 0x94);
  uVar3 = (uVar1 + 0x2e);
  pass1_1038_3aa6(uVar3,uVar4,uVar3);
  (iVar5 + 0x98) = uVar3;
  SetDlgItemInt16(0x0,uVar3,s_dibtext_bmp_1050_1844 + 0xb,(iVar5 + 0x6));
  return;
}


pub unsafe fn msg_box_ui_op_1038_8a3a(param_1: *mut c_char, mut param_2: u16, param_3: *mut Struct903, mut param_4: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_20a: [u8;0x102] = [0;0x102];
  let mut pcStack264: *mut c_char;
  let mut sStack262: i16;
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  sStack262 = paVar1;
  pcStack264 = param_1;
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,sStack262);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x101,local_20a,&DAT_1050_1050);
  MessageBox16(0x0,CONCAT22(0x1050,local_20a),CONCAT22(sStack262,pcStack264),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(sStack262,pcStack264));
  return;
}

pub unsafe fn win_ui_dlg_op_1038_98b4(param_1: *mut StructB)

{
  let mut UVar1: u16;
   let mut iVar3: *mut StructB;
   let mut uVar2: *mut StructB;
  let mut pvVar2: LPVOID = null_mut();
  let mut iVar4: i16;
  let mut iStack8: i16;
  let mut local_4: bool;

  local_4 = 0;
//   for (iStack8 = 0; iVar3 = param_1, uVar2 = (param_1 >> 0x10), iStack8 < 0xf;
//       iStack8 += 1)
      iStack8 = 0;
      iVar3 = param_1;
      uVar2 = param_1 >> 0x10;
      while iStack8 < 0xf
      {
    iVar4 = 0x1;
    pvVar2 = iVar3.lpvoid_field_0x8;
    UVar1 = GetDlgItemInt16(0x1,&local_4,&DAT_1050_1050,(iStack8 * 0xe + 0x5a72));
    unk_win_ui_op_1038_9820(param_1, UVar1, pvVar2, iVar4);
    iStack8 += 1;
  }
  SetDlgItemInt16(0x1,iVar3[0x7].max_count_field_0x10,0xfa9,iVar3.lpvoid_field_0x8);
  SetDlgItemInt16(0x1,iVar3[0x7].field5_0xa,0xfa8,iVar3.lpvoid_field_0x8);
  return;
}

pub unsafe fn dialog_item_ui_op_1040_3e08(struct_c_param_1: *mut StructC)

{
  let mut uVar1: u16;
  let mut struct_c_1: *mut StructC;
  let mut var3: u16;
  let mut LVar2: LRESULT;

  var3 = (struct_c_param_1 >> 0x10);
  struct_c_1 = struct_c_param_1;
  CheckRadioButton16(struct_c_1.field149_0xa0,0x18d,0x188,struct_c_1.field6_0x6);
  struct_c_1.field147_0x9c = 0;
  struct_c_1.field148_0x9e = 0;
  LVar2 = SendDlgItemMessage16(0x0,0x0,0x409,0x190,struct_c_1.field6_0x6);
  if (LVar2 != -1) {
    uVar1 = pass1_1018_3ab2(struct_c_1.field141_0x8e,LVar2,struct_c_1.field149_0xa0);
    struct_c_1.field148_0x9e = uVar1;
  }
  SetDlgItemInt16(0x0,struct_c_1.field147_0x9c,0x18e,struct_c_1.field6_0x6);
  SetDlgItemInt16(0x0,struct_c_1.field148_0x9e,0x191,struct_c_1.field6_0x6);
  match struct_c_1.field149_0xa0 {
  0x188 =>{
    struct_c_1.field152_0xa4 = 0x5;}
    // break;
  0x189 =>{
    struct_c_1.field152_0xa4 = 0x6;}
    // break;
  0x18a =>{
    struct_c_1.field152_0xa4 = 0x7;}
    // break;
  0x18b =>{
    struct_c_1.field152_0xa4 = 0x8;}
    // break;
  0x18c =>{
    struct_c_1.field152_0xa4 = 0x9;}
    // break;
  0x18d =>{
    struct_c_1.field152_0xa4 = 0xa;}
  }
  draw_ops::invalidate_rect_1040_3ddc(struct_c_param_1);
  return;
}

pub unsafe fn send_dlg_item_msg_1040_3f12(struct_c_param_1: *mut StructC, struct_c_param_2: *mut StructC, mut param_3: u32)

{
  let mut puVar1: *mut u8;
  let mut extraout_DX: u16;
  let mut iVar2: i16;
  let mut LVar3: LRESULT;
  let mut local_a: [u8;0x8] = [0;0x8];

  SendDlgItemMessage16(0x0,0x0,0xb,0x190,struct_c_param_1.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x190,struct_c_param_1.field6_0x6);
  pass1_1008_5784(CONCAT22(0x1050,local_a),param_3);
  loop {
    puVar1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0) { break; }
    LVar3 = SendDlgItemMessage16((puVar1 + 0x4),0x0,0x401,0x190,struct_c_param_1.field6_0x6);
    iVar2 = (LVar3 >> 0x10);
    if (((LVar3 == -1) && (iVar2 == -1)) || ((LVar3 == -0x2 && (iVar2 == -1)))) { break; }
  }
  SendDlgItemMessage16(0x0,0x0,0x407,0x190,struct_c_param_1.field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x190,struct_c_param_1.field6_0x6);
  return;
}


pub unsafe fn pass1_1038_8966(mut param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: i16, mut param_5: u16 ) -> u16

{
  let mut piVar1: *mut i16;
  let mut bVar2: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;

  bVar2 = false;
  iVar3 = param_1;
  uVar4 = (param_1 >> 0x10);
  if (param_4 == 0) {
//    if ((iVar3 + 0x98) < 1) goto LAB_1038_89af;
    piVar1 = (iVar3 + 0x9a);
    *piVar1 = *piVar1 + 1;
    piVar1 = (iVar3 + 0x98);
    *piVar1 = *piVar1 -0x1;
  }
  else {
//    if (param_4 != 1) goto LAB_1038_89af;
//    if ((iVar3 + 0x9a) < 1) goto LAB_1038_89af;
    piVar1 = (iVar3 + 0x9a);
    *piVar1 = *piVar1 -0x1;
    piVar1 = (iVar3 + 0x98);
    *piVar1 = *piVar1 + 1;
  }
  bVar2 = true;//
// LAB_1038_89af:
  if (bVar2) {
    SetDlgItemInt16(0x0,(iVar3 + 0x9a),s_dibtext_bmp_1050_1844 + 0x9,(iVar3 + 0x6));
    SetDlgItemInt16(0x0,(iVar3 + 0x98),s_dibtext_bmp_1050_1844 + 0xb,(iVar3 + 0x6));
  }
  return 0x0;
}


pub unsafe fn pas1_1040_29c2(mut param_1: u16, mut param_2: u16, param_3: *mut Struct57, mut param_4: u32, mut param_5: u16) -> *mut Struct57

{
    let mut iVar1: *mut Struct57;
    let mut uVar1: *mut Struct57;

    pass1_1040_b0bc(param_3, param_4, CONCAT22(param_5, 0x157));
    uVar1 = (param_3 >> 0x10);
    iVar1 = param_3;
    param_3.field0_0x0 = 0x2e26;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x64b);
    iVar1[0x1].field3_0x6 = param_1;
    iVar1[0x1].field4_0x8 = param_2;
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x64a);
    iVar1[0x1].field5_0xa = param_1;
    iVar1[0x1].field6_0xc = param_2;
    return param_3;
}


pub unsafe fn unk_win_ui_op_1038_9820(param_1: *mut StructB, mut param_2: i16, mut param_3: i16, mut param_4: i16) -> i32

{
  let mut puVar1: *mut u16;
  let mut ppuVar2: *mut *mut u32 = null_mut();
  let mut lVar3: i32;
  let mut UVar4: u16;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar8: u16;
   let mut iVar7: *mut StructB;
   let mut uVar7: *mut StructB;
  let mut local_6: bool;
  let mut local_4: bool;

  uVar7 = (param_1 >> 0x10);
  iVar7 = param_1;
  UVar4 = GetDlgItemInt16(0x1,&local_4,&DAT_1050_1050,(param_4 * 0xe + 0x5a74));
  iVar5 = UVar4 * param_2 * param_3;
  UVar4 = GetDlgItemInt16(0x1,&local_6,&DAT_1050_1050,(param_4 * 0xe + 0x5a76));
  lVar3 = (UVar4 * param_2) * param_3;
  uVar8 = (lVar3 >> 0x10);
  iVar6 = lVar3;
  if (((iVar5 - iVar7[0x7].max_count_field_0x10) < 1) && (-0x1 < iVar7[0x7].field5_0xa - iVar6)) {
    puVar1 = &iVar7[0x7].max_count_field_0x10;
    *puVar1 = *puVar1 - iVar5;
    ppuVar2 = &iVar7[0x7].field5_0xa;
    *ppuVar2 = (*ppuVar2 - iVar6);
    return CONCAT22(uVar8,1);
  }
  return uVar8 << 0x10;
}

pub unsafe fn unk_win_ui_op_1038_8afe(mut param_1: u16, param_2: *mut astruct_50)

{
  let mut uVar1: u32;
  let mut dlg_item: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar4: *mut astruct_50;
  let mut uVar4: *mut astruct_50;
  let mut local_4: bool;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  uVar4 = (param_2 >> 0x10);
  iVar4 = param_2;
  dlg_item = GetDlgItemInt16(0x0,&local_4,&DAT_1050_1050,s_dibtext_bmp_1050_1844 + 0x9);
  pass1_1030_6c1a(iVar4.field148_0x94,dlg_item);
  uVar1 = iVar4.field148_0x94;
  pass1_1038_387e(paVar2,(uVar1 + 0x2e),dlg_item,iVar4.field153_0x9c,iVar4.field148_0x94);
  return;
}


pub unsafe fn win_dlg_op_1038_9294(mut param_1: u16, param_2: *mut StructB)

{
  let mut UVar1: u16;
  let mut uVar1: u16;
   let mut struct_b_1_hi: *mut StructB;
  let mut local_6: bool;
  let mut local_4: bool;

  unk_win_ui_op_1040_b230(param_1,param_2);
  struct_b_1_hi = (param_2 >> 0x10);
  UVar1 = GetDlgItemInt16(0x1,&local_4,&DAT_1050_1050,0xfa9);
  (param_2 + 0x94) = UVar1;
  uVar1 = GetDlgItemInt16(0x1,&local_6,&DAT_1050_1050,0xfa8);
  (param_2 + 0x96) = uVar1;
  win_ui_dlg_op_1038_98b4((param_2 & 0xffff | ZEXT24(struct_b_1_hi) << 0x10));
  win_1008_5c7c(uVar1,param_1,_u16_1050_02a0,0x950001);
  return;
}


pub unsafe fn win_msg_op_1038_95fc(mut param_1: u16, mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut UVar3: u16;
  let mut UVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut paVar7: *mut Struct57;
  let mut iVar9: i16;
  let mut unaff_SI: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut puStack30: *mut u16;
  let mut puStack24: *mut u16;
  let mut iStack20: i16;
  let mut local_10: bool;
  let mut puStack14: *mut u32;
  let mut puStack10: *mut u32;
  let mut puStack6: *mut u32;
  let mut paVar8: *mut Struct57;

  paVar6 = CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(unaff_SI,0x8),in_stack_0000fe80,
                             in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar6 = (paVar6 & 0xffff0000 | puStack6 >> 0x10);
  puStack10 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(unaff_SI,0x9),in_stack_0000fe80,
                              in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar7 = (paVar6 & 0xffff0000 | puStack10 >> 0x10);
  uVar2 = puStack10;
  mem_op_1000_179c(0xc,paVar7);
  uVar5 = paVar7 | uVar2;
  paVar6 = (paVar7 & 0xffff0000);
  paVar8 = (paVar6 | uVar5);
  if (uVar5 == 0) {
    uVar2 = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(paVar7,uVar2));
    paVar6 = paVar8;
  }
  puStack14 = CONCAT22(paVar6,uVar2);
  for iStack20 in 0 .. 0xf {
    uVar13 = (param_2 + 0x6);
    UVar3 = GetDlgItemInt16(0x1,&local_10,&DAT_1050_1050,(iStack20 * 0xe + 0x5a72));
    if (UVar3 != 0) {
      if ((iStack20 * 0xe + 0x5a7c) < 0x83) {
        UVar4 = UVar3;
        mem_op_1000_179c(0x8,paVar6);
        uVar2 = paVar6;
        puStack24 = CONCAT22(uVar2,UVar4);
        paVar6 = (paVar6 & 0xffff0000 | (uVar2 | UVar4));
        if ((uVar2 | UVar4) == 0) {
          puStack30 = null_mut();
        }
        else {
          *puStack24 = 0x389a;
          (UVar4 + 0x2) = 0x1008;
          *puStack24 = 0xa1c4;
          (UVar4 + 0x2) = 0x1010;
          puStack30 = puStack24;
        }
        uVar10 = (puStack30 >> 0x10);
        iVar9 = puStack30;
        (iVar9 + 0x6) = UVar3;
        (iVar9 + 0x4) = (iStack20 * 0xe + 0x5a7c);
        ppcVar1 = (*puStack14 + 0x4);
        (**ppcVar1)(0x1000,puStack14,(puStack14 >> 0x10),iVar9,uVar10,uVar13);
      }
      else {
        if ((iStack20 * 0xe + 0x5a7c) == 0x89) {
          uVar12 = (iStack20 * 0xe + 0x5a7c);
          uVar11 = UVar3;
        }
        else {
          uVar12 = (iStack20 * 0xe + 0x5a7c);
          uVar11 = 0;
        }
        pass1_1010_6566(puStack10,uVar11,UVar3,uVar12);
      }
    }
  }
  (puStack6 + 0xa) = puStack14;
  PostMessage16(0x0,0xed,0x111,HWND16_1050_0396);
  return;
}

pub unsafe fn unk_win_ui_op_1040_3c64(mut param_1: u16, struct_c_param_1: *mut StructC, struct_c_param_2: *mut StructC, mut param_4: u16, mut param_5: u32)

{
    let mut UVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut uVar3: u32;
    let mut LVar4: LRESULT;
    let mut paVar5: *mut Struct27;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut iVar6: i16;

    if (param_5 == 0x186) {
        LVar4 = SendDlgItemMessage16(0x0, 0x0, 0x409, 0x190, struct_c_param_1.field6_0x6);
        uVar2 = (LVar4 >> 0x10);
        UVar1 = GetDlgItemInt16(0x0, NULL, 0x0, 0x18e);
        pass1_1018_36e6(struct_c_param_1.field141_0x8e, UVar1, LVar4, struct_c_param_1.field149_0xa0);
        pass1_1038_af40(struct_c_param_1, uVar2, _PTR_LOOP_1050_5b7c, &struct_c_param_1.field_0x8, 0x22);
        LVar4 = SendMessage16(0x0, 0x2, 0x111, struct_c_param_1.field6_0x6);
        iVar6 = 0x1;
        paVar5 = mixed_1010_20ba(CONCAT22(in_register_0000000a, (LVar4 >> 0x10)), _u16_1050_0ed0,
                                 0x1002b, in_stack_0000fe9a, in_stack_0000ffbe, in_stack_0000ffc4,
                                 in_stack_0000ffc8);
        pass1_1010_038e(paVar5, iVar6);
    } else {
        if (param_5 - 0x186 < 0x2) {//
// LAB_1040_3c7f:
            post_win_msg_1040_7b3c(CONCAT22(struct_c_param_2, struct_c_param_1), param_4, param_5, param_5);
            return;
        }
        if (param_5 - 0x188 < 0x5 || param_5 == 0x18d) {
            struct_c_param_1.field149_0xa0 = param_5;
            uVar3 = switch_1018_3b9e(param_5, param_1, struct_c_param_1.field141_0x8e, param_5);
            send_dlg_item_msg_1040_3f12(struct_c_param_1, struct_c_param_2, uVar3);
        } else {
//      if (param_5 - 0x188 != 0x8) goto LAB_1040_3c7f;
            if (param_5 != 1) {
                return;
            }
        }
        dialog_item_ui_op_1040_3e08(CONCAT22(struct_c_param_2, struct_c_param_1));
    }
    return;
}

pub unsafe fn win_dlg_item_1040_2d48(mut param_1: u32)

{
    let mut UVar1: u16;
    let mut value: u16;
    let mut local_4: bool;

    pass1_1040_b45e(param_1);
    UVar1 = GetDlgItemInt16(0x1, &local_4, &DAT_1050_1050, 0x163);
    value = GetDlgItemInt16(0x1, &local_4, &DAT_1050_1050, 0x165);
    if (UVar1 != 0) {
        value /= UVar1;
    }
    SetDlgItemInt16(0x1, value, 0x165, (param_1 + 0x6));
    return;
}


pub unsafe fn check_dialog_btn_1040_1afe(param_1: *mut StructB)

{
  let mut check: u16;
  let mut check_00: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;
   let mut iVar3: *mut StructB;
  let mut uVar3: u16;
  let mut check_01: u16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = &iVar3[0x7].field1_0x2;
  uVar2 = &iVar3[0x7].field1_0x2;
  check = (uVar2 + 0x20);
  uVar2 = &iVar3[0x7].field1_0x2;
  check_00 = (uVar2 + 0x74);
  uVar2 = &iVar3[0x7].field1_0x2;
  check_01 = (uVar2 + 0x72);
  CheckDlgButton16((uVar1 + 0x1e),0xfdb,iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check_00,0xfdd,iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check_01,0xfde,iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check,0xfdc,iVar3.lpvoid_field_0x8);
  return;
}

pub unsafe fn check_dialog_btn_1040_1b8a(param_1: *mut StructC)

{
  let mut check: u16;
  let mut check_00: u16;
  let mut check_01: u16;
  let mut iVar1: *mut StructC;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  check_00 = pass1_1010_60b4();
  pass1_1010_60c6();
  check_01 = pass1_1010_60c0();
  pass1_1010_60ba();
  CheckDlgButton16(check_00,0xfdb,iVar1.field6_0x6);
  CheckDlgButton16(check_01,0xfdd,iVar1.field6_0x6);
  CheckDlgButton16(0xfde,0xfde,iVar1.field6_0x6);
  check = iVar1.field6_0x6;
  CheckDlgButton16(check,0xfdc,check);
  return;
}

pub unsafe fn check_dlg_btn_checked_1038_cdd6(param_1: *mut astruct_61, mut param_2: i16)

{
  let mut uVar1: u32;
  let mut UVar2: u16;
  let mut iVar3: *mut astruct_61;
  let mut uVar3: u16;

  iVar3 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0) {
    uVar1 = iVar3.field142_0x8e;
    (uVar1 + 0xa) = 0;
  }
  else {
    UVar2 = IsDlgButtonChecked(0x182e,&iVar3.field_0x6);
    if (UVar2 == 0) {
      UVar2 = IsDlgButtonChecked(0x182f,&iVar3.field_0x6);
      if (UVar2 == 0) {
        UVar2 = IsDlgButtonChecked(0x1829,&iVar3.field_0x6);
        if (UVar2 == 0) {
          UVar2 = IsDlgButtonChecked(0x182a,&iVar3.field_0x6);
          if (UVar2 == 0) {
            UVar2 = IsDlgButtonChecked(0x182c,&iVar3.field_0x6);
            if (UVar2 == 0) {
              UVar2 = IsDlgButtonChecked(0x182d,&iVar3.field_0x6);
              if (UVar2 != 0) {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xa) = 0x7;
              }
            }
            else {
              uVar1 = iVar3.field142_0x8e;
              (uVar1 + 0xa) = 0x6;
            }
          }
          else {
            uVar1 = iVar3.field142_0x8e;
            (uVar1 + 0xa) = 0x4;
          }
        }
        else {
          uVar1 = iVar3.field142_0x8e;
          (uVar1 + 0xa) = 0x3;
        }
      }
      else {
        uVar1 = iVar3.field142_0x8e;
        (uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field142_0x8e;
      (uVar1 + 0xa) = 0x1;
    }
  }
  iVar3.field143_0x92 = 0;
  return;
}

pub unsafe fn destroy_win_1038_e1dc(param_1: *mut astruct_886, mut param_2: u16, mut param_3: i16)

{
    let mut UVar1: u16;
    let mut uVar2: u32;

    if (param_3 != 0) {
        UVar1 = IsDlgButtonChecked(0x1807, param_1.field6_0x6);
        if (UVar1 == 0) {
            UVar1 = IsDlgButtonChecked(0x1806, param_1.field6_0x6);
//      if (UVar1 == 0) goto LAB_1038_e229;
            uVar2 = 0x1110130;
        } else {
            uVar2 = 0x111012f;
        }
        SendMessage16(0x0, uVar2, (uVar2 >> 0x10), HWND16_1050_0396);
    }//
// LAB_1038_e229:
    DestroyWindow16(param_1.field6_0x6);
    return;
}


pub unsafe fn chk_is_dlg_btn_checked_1038_e7a0(param_1: *mut astruct_62, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut UVar2: u16;
    let mut iVar3: *mut astruct_62;
    let mut uVar3: u16;

    iVar3 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 0) {
        uVar1 = iVar3.field142_0x8e;
        (uVar1 + 0x10) = 0x1;
        uVar1 = iVar3.field142_0x8e;
        (uVar1 + 0xa) = 0;
        uVar1 = iVar3.field142_0x8e;
        (uVar1 + 0xc) = 0;
        uVar1 = iVar3.field142_0x8e;
        (uVar1 + 0xe) = 0;
    } else {
        UVar2 = IsDlgButtonChecked(0x1827, &iVar3.field_0x6);
        if (UVar2 == 0) {
            UVar2 = IsDlgButtonChecked(0x1828, &iVar3.field_0x6);
            if (UVar2 == 0) {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xa) = 0;
            } else {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xa) = 0x2;
            }
        } else {
            uVar1 = iVar3.field142_0x8e;
            (uVar1 + 0xa) = 0x1;
        }
        UVar2 = IsDlgButtonChecked(s_vrpal_bmp_1050_183a, &iVar3.field_0x6);
        if (UVar2 == 0) {
            UVar2 = IsDlgButtonChecked(s_vrpal_bmp_1050_183a + 0x1, &iVar3.field_0x6);
            if (UVar2 == 0) {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xc) = 0;
            } else {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xc) = 0x2;
            }
        } else {
            uVar1 = iVar3.field142_0x8e;
            (uVar1 + 0xc) = 0x1;
        }
        UVar2 = IsDlgButtonChecked(s_vrpal_bmp_1050_183a + 0x2, &iVar3.field_0x6);
        if (UVar2 == 0) {
            UVar2 = IsDlgButtonChecked(s_vrpal_bmp_1050_183a + 0x3, &iVar3.field_0x6);
            if (UVar2 == 0) {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xe) = 0;
            } else {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xe) = 0x2;
            }
        } else {
            uVar1 = iVar3.field142_0x8e;
            (uVar1 + 0xe) = 0x1;
        }
        uVar1 = iVar3.field142_0x8e;
        (uVar1 + 0x10) = 0;
    }
    iVar3.field143_0x92 = 0;
    return;
}


pub unsafe fn unk_win_ui_op_1040_19ea(param_1: *mut astruct_32, mut param_2: i16, param_3: *mut u8)

{
    let mut pSVar1: *mut StructD;
    let mut UVar2: u16;
    let mut pstruct32_1: *mut astruct_32;
    let mut pstruct_32_hi: *mut astruct_32;

    pstruct32_1 = param_1;
    pstruct_32_hi = (param_1 >> 0x10);
    if (param_2 != 0) {
        UVar2 = IsDlgButtonChecked(0xfdb, pstruct32_1.field6_0x6);
        pass1_1010_5d9c(param_3, pstruct32_1.pstructd_0x8e, UVar2);
        UVar2 = IsDlgButtonChecked(0xfdc, pstruct32_1.field6_0x6);
        pSVar1 = pstruct32_1.pstructd_0x8e;
        (pSVar1 + 0x20) = UVar2;
        UVar2 = IsDlgButtonChecked(0xfdd, pstruct32_1.field6_0x6);
        pSVar1 = pstruct32_1.pstructd_0x8e;
        (pSVar1 + 0x74) = UVar2;
        UVar2 = IsDlgButtonChecked(0xfde, pstruct32_1.field6_0x6);
        pSVar1 = pstruct32_1.pstructd_0x8e;
        (pSVar1 + 0x72) = UVar2;
        if (pstruct32_1.field142_0x92 != 0) {
            pSVar1 = pstruct32_1.pstructd_0x8e;
            pass1_1000_4906((pSVar1 & 0xffff0000 | (pSVar1 + 0x22)), NULL, 0x40);
        }
        if (pstruct32_1.field143_0x94 != 0) {
            pass1_1010_60a0(pstruct32_1.pstructd_0x8e);
        }
    }
    DestroyWindow16(pstruct32_1.field6_0x6);
    return;
}


pub unsafe fn send_dlg_item_msg_1038_7eac(param_1: *mut Struct903) -> LRESULT

{
  let mut in_EDX: *mut Struct57;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut l_param: *mut c_char;
  let mut LVar4: LRESULT;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000fff2: u16;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000fff2,0x30),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  l_param = pass1_1010_375e(puVar3);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  pass1_1008_b1a6((iVar1 + 0x94),l_param);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1854,(iVar1 + 0x6));
  LVar4 = SendDlgItemMessage16(0x0,0x0,0x409,0x1854,(iVar1 + 0x6));
  if ((LVar4 != 0xffff) || ((LVar4 >> 0x10) != -1)) {
    SendDlgItemMessage16(0x0,LVar4,0x403,0x1854,(iVar1 + 0x6));
    SendDlgItemMessage16(l_param,0x0,0x401,0x1854,(iVar1 + 0x6));
    SendDlgItemMessage16(0x0,0xffff,0x407,0x1854,(iVar1 + 0x6));
    SendDlgItemMessage16(0x0,0x0,0x405,0x1855,(iVar1 + 0x6));
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,(iVar1 + 0x6));
    enable_win_1038_806a((LVar4 >> 0x10),param_1);
  }
  LVar4 = SendDlgItemMessage16(0x0,0x1,0xb,0x1854,(iVar1 + 0x6));
  return LVar4;
}

pub unsafe fn send_dlg_item_msg_1038_7fae(mut param_1: u16, mut param_2: u16, mut param_3: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;

  uVar2 = (param_3 >> 0x10);
  iVar1 = param_3;
  pass1_1008_b146(param_1,param_2,(iVar1 + 0x94));
  SendDlgItemMessage16(0x0,0xffff,0x407,0x1855,(iVar1 + 0x6));
  LVar3 = SendDlgItemMessage16(0x0,0xffff,0x407,0x1856,(iVar1 + 0x6));
  pass1_1008_b61a(LVar3,(LVar3 >> 0x10),(iVar1 + 0x94),0x0);
  pass1_1008_b63a((iVar1 + 0x94),0x0);
  return;
}

pub unsafe fn send_dlg_item_msg_1038_8164(mut param_1: u16, mut param_2: u16, param_3: *mut u8, mut param_4: u16 ) -> u16

{
  let mut LVar1: LRESULT;

  *param_3 = '\0';
  LVar1 = SendDlgItemMessage16(0x0,0x0,0x409,param_4,(param_1 + 0x6));
  if ((LVar1 != -1) &&
     (LVar1 = SendDlgItemMessage16(param_3,LVar1,0x40a,param_4,(param_1 + 0x6)),
     LVar1 != -1)) {
    return 0x1;
  }
  return 0x0;
}

pub unsafe fn send_dlg_item_msg_1038_8400(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u16 )

{
  let mut lVar1: i32;
  let mut local_a: [u8;0x8] = [0;0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),param_3);
  loop {
    lVar1 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar1 == 0) { break; }
    SendDlgItemMessage16((lVar1 + 0x4),0x0,0x401,param_4,(param_1 + 0x6));
  }
  return;
}


pub unsafe fn send_dlg_item_msg_1038_8618s(mut param_1: u16, param_2: *mut Struct903) -> u16

{
  let mut in_AX: i16;
  let mut uVar1: u16;
  let mut puVar2: *mut u8;
  let mut puVar3: *mut u8;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut l_param: u32;
  let mut uVar7: u32;
  let mut local_106: [u8;0x100] = [0;0x100];
  let mut uStack6: u32;

  uVar5 = (param_2 >> 0x10);
  uVar4 = param_2;
  uStack6 = pass1_1008_b820(in_AX,param_1,(uVar4 + 0x94));
  uVar1 = uStack6;
  if (uStack6 != 0) {
    uVar1 = send_dlg_item_msg_1038_8164(uVar4,uVar5,CONCAT22(0x1050,local_106),0x1854);
    if (uVar1 != 0) {
      SendDlgItemMessage16(0x0,0x0,0xb,0x1855,(uVar4 + 0x6));
      SendDlgItemMessage16(0x0,0x0,0xb,0x1856,(uVar4 + 0x6));
      SendDlgItemMessage16(0x0,0x0,0x405,0x1855,(uVar4 + 0x6));
      LVar6 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,(uVar4 + 0x6));
      puVar3 = (LVar6 >> 0x10);
      puVar2 = local_106;
      pass1_1008_b4a0(puVar2,puVar3,(uVar4 + 0x94),CONCAT22(0x1050,puVar2));
      pass1_1008_b200((uVar4 + 0x94));
      uVar7 = CONCAT22(puVar3 | puVar2,puVar2);
      if ((puVar3 | puVar2) != 0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,CONCAT22(puVar3,puVar2),0x1855);
        l_param = pass1_1008_b366((uVar4 + 0x94));
        uVar7 = l_param & 0xffff | ((l_param >> 0x10) | l_param) << 0x10;
        if (l_param != 0) {
          uVar7 = SendDlgItemMessage16(l_param,0xffff,0x40d,0x1855,(uVar4 + 0x6));
        }
      }
      uVar7 = pass1_1008_b38c(CONCAT22(uVar7,(uVar7 >> 0x10)),(uVar4 + 0x94));
      if (uVar7 != 0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,uVar7,0x1856);
        uVar7 = pass1_1008_b47a((uVar4 + 0x94));
        if (uVar7 != 0) {
          SendDlgItemMessage16(uVar7,0xffff,0x40d,0x1856,(uVar4 + 0x6));
        }
      }
      SendDlgItemMessage16(0x0,0x1,0xb,0x1855,(uVar4 + 0x6));
      LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,(uVar4 + 0x6));
      uVar1 = LVar6;
    }
  }
  return uVar1;
}


pub unsafe fn send_dlg_item_msg_1038_87b2(mut param_1: u16, param_2: *mut Struct903) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut l_param: *mut c_char;
  let mut LVar3: LRESULT;
  let mut uVar4: u16;
  let mut local_102: [u8;0x100] = [0;0x100];

  uVar4 = param_2;
  uVar1 = (param_2 >> 0x10);
  uVar2 = send_dlg_item_msg_1038_8164(uVar4,uVar1,CONCAT22(0x1050,local_102),0x1855);
  if (uVar2 != 0) {
    pass1_1008_b61a(local_102,param_1,(uVar4 + 0x94),CONCAT22(0x1050,local_102));
    l_param = load_string_1008_b1f0();
    LVar3 = SendDlgItemMessage16(l_param,0xffff,0x40d,0x1856,(uVar4 + 0x6));
    uVar2 = LVar3;
  }
  return uVar2;
}

pub unsafe fn send_dlg_item_msg_1038_8d22(mut param_1: u32, param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut local_106: [u8;0x100] = [0;0x100];
  let mut WStack6: WPARAM16;
  let mut iStack4: i16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendDlgItemMessage16(0x0,0x0,0x409,0x185b,(iVar1 + 0x6));
  WStack6 = LVar3;
  iStack4 = WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16(CONCAT22(0x1050,local_106),WStack6,0x40a,0x185b,(iVar1 + 0x6));
    pass1_1008_c79a((iVar1 + 0x94),CONCAT22(0x1050,local_106));
  }
  return;
}


pub unsafe fn win_sys_op_1038_a9fa(mut param_1: u32, mut param_2: i16)

{
  let mut hwnd: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut LVar4: LRESULT;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;

  if (param_2 != 0) {
    puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000fff0,0x2),in_stack_0000fe98,
                             in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x116,(iVar1 + 0x6));
    if (((LVar4 >> 0x10) | LVar4) == 0) {
      LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x117,(iVar1 + 0x6));
      if (((LVar4 >> 0x10) | LVar4) == 0) {
        LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x118,(iVar1 + 0x6));
        if (((LVar4 >> 0x10) | LVar4) == 0) {
          LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x119,(iVar1 + 0x6));
          if (((LVar4 >> 0x10) | LVar4) != 0) {
            PTR_LOOP_1050_13ae = &u32_1050_0004;
          }
        }
        else {
          PTR_LOOP_1050_13ae = (&u16_1050_0002 + 1);
        }
      }
      else {
        PTR_LOOP_1050_13ae = &u16_1050_0002;
      }
    }
    else {
      PTR_LOOP_1050_13ae = (&PTR_LOOP_1050_0000 + 1);
    }
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x11a,(iVar1 + 0x6));
    (puVar3 + 0x82) = LVar4;
    hwnd = GetWindowWord16(-0x8,(iVar1 + 0x6));
    PostMessage16(0x0,0x105,0x111,hwnd);
    destroy_win_1040_7b98(param_1);
  }
  return;
}

pub unsafe fn win_ui_op_1038_e348(param_1: *mut StructB, param_2: u8, param_3: *mut StructD, mut param_4: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar3: u32;
    let mut uVar5: u16;
    let mut uVar4: u16;
    let mut rect: *mut Struct57;
    let mut uVar7: u16;
    let mut uVar6: *mut StructD;
    let mut uVar11: u16;
    let mut struct_b_5: *mut StructB;
    let mut iVar12: i16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut uVar10: u16;
    let mut uVar9: u16;
    let mut in_stack_0000fe2a: u16;
    let mut in_stack_0000fe2e: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ff54: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut local_22: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut puStack26: *mut u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut puStack6: *mut u32;
    let mut uVar2: u32;
    let mut paVar8: *mut Struct57;

    dialog_ui_fn_1040_78e2(param_1);
    uVar6 = param_3;
    puStack6 = mixed_1010_20ba(param_3, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b),
                               in_stack_0000fe7e, in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    uVar6 = (uVar6 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_088c();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar6);
    } else {
        uVar6 = (uVar6 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(uVar6, PTR_LOOP_1050_5f2c);
    uVar4 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, uVar6);
    uVar8 = (param_1 >> 0x10);
    struct_b_5 = param_1;
    struct_b_5[0x7].field1_0x2 = uVar4;
    struct_b_5[0x7].hwnd_0x6 = uVar6;
//   for (iStack10 = 0x1; uVar11 = (uVar6 >> 0x10), iStack10 <= uStack8; iStack10 += 1)
    iStack10 = 1;
    uVar11 = uVar6 >> 0x10;
    while iStack10 <= uStack8 {
        puStack26 = pass1_1010_091e(puStack6, (puStack6 >> 0x10), iStack10);
        uVar5 = (puStack26 >> 0x10);
        paVar8 = CONCAT22(uVar11, uVar5);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar8);
        uVar7 = paVar8 | rect;
        uVar6 = (paVar8 & 0xffff0000 | uVar7);
        if (uVar7 == 0) {
            uVar3 = &struct_b_5[0x7].field1_0x2;
            (uVar3 + iStack10 * 0x4) = 0;
        } else {
            pvVar1 = struct_b_5.lpvoid_field_0x8;
            pass1_1008_3bd6(uVar6, rect, paVar8, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, (puStack26 + 0x4))), param_4, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar2 = &struct_b_5[0x7].field1_0x2;
            uVar9 = (uVar2 >> 0x10);
            iVar12 = uVar2;
            (iVar12 + iStack10 * 0x4) = rect;
            (iVar12 + iStack10 * 0x4 + 0x2) = uVar6;
        }
        uVar3 = &struct_b_5[0x7].field1_0x2;
        uVar10 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        if ((iVar12 + iStack10 * 0x4) != 0) {
            enable_win_1040_9234((iVar12 + iStack10 * 0x4), (puStack26 + 0x6));
        }
        iStack10 += 1;
    }
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(0x5, struct_b_5.lpvoid_field_0x8);
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

    dialog_ui_fn_1040_78e2(pstructb_param_2);
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
        MapDialogRect16(rect, &DAT_1050_1050);
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
        let uVar7 = win_ui_op_1040_0558(pstructb_param_2, paStack10);
        pstruct57_param_1 = (uVar4 & 0xffff0000 | uVar7 >> 0x10);
        paStack10 += 1;
    }
    move_win_1040_826c(pstructb_param_2, -0x1, 0xffff);
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

pub unsafe fn win_ui_op_1008_1414(
    mut param_1: u32,
    param_2: *mut astruct_20,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut puVar5: *mut u32;
    let mut extraout_DX: u16;
    let mut uVar6: u32;
    let mut paVar7: *mut Struct57;
    let mut uVar8: u32;
    let mut iVar17: *mut astruct_20;
    let mut unaff_SI: u16;
    let mut unaff_DI: u16;
    let mut uVar16: *mut Struct72;
    let mut puVar9: *mut u32;
    let mut pcVar10: *mut c_char;
    let mut puVar11: *mut u32;
    let mut paVar12: *mut Struct27;
    let mut in_stack_0000fe3c: u16;
    let mut in_stack_0000fe3e: u16;
    let mut in_stack_0000fe4e: u16;
    let mut in_stack_0000ff60: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff66: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6a: u16;
    let mut in_stack_0000ff6c: u16;
    let mut in_stack_0000ff72: u16;
    let mut in_stack_0000ff78: u16;
    let mut in_stack_0000ff7c: u16;
    let mut uVar13: u8;
    let mut uVar14: u8;
    let mut uVar15: u16;
    let mut local_2a: u32;
    let mut uStack38: u16;
    let mut iStack36: i16;
    let mut uStack34: u16;
    let mut uStack32: u32;
    let mut uStack28: u32;
    let mut uStack24: u32;
    let mut uStack20: u32;
    let mut uStack16: u32;
    let mut puStack12: *mut u32;
    let mut local_8: [u8; 0x6] = [0; 0x6];
    let mut uVar10: u16;

    puVar9 = str_1008_6d8a(param_1, CONCAT22(0x1050, local_8), param_3);
    paVar7 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    BVar3 = read_file_1008_6e78(CONCAT22(0x1050, local_8), unaff_SI, unaff_DI, param_4);
    iVar17 = param_2;
    uVar16 = (param_2 >> 0x10);
    if (BVar3 == 0) {
        if (u16_1050_0310 == 0) {
            u16_1050_0310 = 0x6d4;
        }
        pcVar10 = load_string_1010_847e(_u16_1050_14cc, u16_1050_0310);
        uVar8 = paVar7 & 0xffff0000 | pcVar10 >> 0x10;
        uVar4 = str_op_1008_60e8((pcVar10 >> 0x10), pcVar10);
        uVar15 = uVar8;
        pcVar10 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
        paVar7 = (uVar8 & 0xffff0000 | pcVar10 >> 0x10);
        MessageBeep16(0x10);
        MessageBox16(0x10, pcVar10, CONCAT22(uVar15, uVar4), iVar17.field3_0x8);
        fn_ptr_1000_17ce(CONCAT22(uVar15, uVar4));
        fn_ptr_op_1000_24cd(1);
    }
    cursor_op_1008_2dcc(paVar7, param_2, 0x8, param_6, param_5);
    puStack12 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT22(unaff_DI, 0x2f),
        in_stack_0000fe4e,
        in_stack_0000ff72,
        in_stack_0000ff78,
        in_stack_0000ff7c,
    );
    paVar7 = (paVar7 & 0xffff0000 | puStack12 >> 0x10);
    uVar6 = (puStack12 + 0x20);
    uStack16 = uVar6;
    pass1_1030_8344(_u16_1050_5748, uVar6);
    uStack20 = uVar6 & 0xffff | paVar7 << 0x10;
    uStack24 = (uVar6 + 0x10);
    iVar5 = (uStack24 + 0x2) -0x1;
    uVar1 = (&iVar17[0x1].field2_0x4 + 2);
    ppcVar2 = ((&iVar17[0x1].field2_0x4 + 0x2) + 0x4);
    (**ppcVar2)(
        0x1030,
        uVar1,
        (uVar1 >> 0x10),
        uStack16,
        (uStack16 >> 0x10),
        iVar5,
        0x2,
    );
    pass1_1030_8344(_u16_1050_5748, 0x4000001);
    uStack28 = CONCAT22(paVar7, iVar5);
    uVar6 = (iVar5 + 0x10);
    uStack32 = uVar6;
    pass1_1030_8344(_u16_1050_5748, uVar6);
    iStack36 = uVar6;
    uStack34 = SUB42(paVar7, 0x0);
    local_2a = (iStack36 + 0xc);
    uStack38 = (iStack36 + 0x10);
    puVar5 = pass1_1030_5b00(uStack20);
    uVar15 = SUB42(&DAT_1050_1050, 0x0);
    uVar13 = SUB21(&local_2a, 0x0);
    uVar14 = (&local_2a >> 0x8);
    puVar11 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT13(uVar14, CONCAT12(uVar13, puVar5)),
        in_stack_0000fe3e,
        in_stack_0000ff62,
        in_stack_0000ff68,
        in_stack_0000ff6c,
    );
    paVar7 = (paVar7 & 0xffff0000 | puVar11 >> 0x10);
    pass1_1018_179e(puVar11, CONCAT22(uVar15, CONCAT11(uVar14, uVar13)));
    uVar13 = 0;
    uVar14 = 0x4;
    iVar5 = 0x1b;
    paVar12 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        0x1002b,
        in_stack_0000fe3c,
        in_stack_0000ff60,
        in_stack_0000ff66,
        in_stack_0000ff6a,
    );
    pass1_1010_043a(paVar12, CONCAT13(uVar14, CONCAT12(uVar13, 1)), iVar5);
    close_file_1008_6dd0(CONCAT22(0x1050, local_8));
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
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar7, (uVar1 >> 0x10), paVar3);
    mem_op_1000_179c(0x24, paVar8);
    uVar5 = paVar8 | paVar3;
    paVar9 = (paVar8 & 0xffff0000 | uVar5);
    if (uVar5 == 0) {
        struct_1[0x1].field22_0x2a = 0;
    } else {
        unk_win_ui_op_1020_1418(CONCAT22(paVar8, paVar3), struct_param_1, param_5);
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

pub unsafe fn invalidate_rect_1020_735a(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0x1c);
    InvalidateRect16(0x0, (uVar1 + 0x16c), (uVar1 >> 0x10));
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
            puVar4 = block_1008_8000::pass1_1008_80d2(CONCAT22(paVar3, param_1));
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

pub unsafe fn set_window_placement_1010_0070(mut param_1: u32, mut param_2: i16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut lVar4: i32;
    let mut uVar5: u16;
    let mut local_18: [u8; 0x6] = [0; 0x6];
    let mut IStack18: i16;
    let mut iStack16: i16;
    let mut IStack14: i16;
    let mut IStack12: i16;
    let mut IStack10: i16;
    let mut IStack8: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    local_18 = 0x16;
    local_18._2_4_ = 0;
    IStack18 = 0;
    iStack16 = 0;
    IStack14 = 0;
    IStack12 = 0;
    IStack10 = 0;
    IStack8 = 0;
    uStack6 = 0;
    uStack4 = 0;
    uVar5 = param_3;
    GetWindowPlacement16(local_18, &DAT_1050_1050);
    if ((iStack16 == -1) || (param_2 != 0)) {
        local_18._2_4_ = 0x50001;
        lVar4 = GetWindowLong16(0x0, param_3);
        uVar2 = (lVar4 >> 0x10);
        puVar3 = (lVar4 + 0xe0);
        ppcVar1 = (*puVar3 + 0x38);
        (**ppcVar1)(s_tile2_bmp_1050_1538, puVar3, (lVar4 + 0xe2), uVar5);
        pass1_1010_01f8(param_1, CONCAT22(0x1050, local_18), puVar3);
        SetWindowPlacement16(local_18, &DAT_1050_1050);
    }
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
    GetWindowPlacement16(&local_18, &DAT_1050_1050);
    if (local_18.rc_normal_position.x == -1) {
        lVar6 = GetWindowLong16(0x0, param_3);
        uVar4 = (lVar6 >> 0x10);
        puVar5 = (lVar6 + 0xe0);
        ppcVar1 = (*puVar5 + 0x1c);
        (**ppcVar1)(s_tile2_bmp_1050_1538, puVar5, (lVar6 + 0xe2), uVar7);
        iVar2 = puVar5;
        piVar3 = (puVar5 & 0xffff | extraout_DX << 0x10);
        local_18.show_cmd = 0x9;
        local_18.rc_normal_position.x = *piVar3;
        local_18.rc_normal_position.y = (iVar2 + 2);
        iStack6 = (iVar2 + 0x4) + *piVar3;
        iStack4 = (iVar2 + 0x2) + (iVar2 + 0x6);
        SetWindowPlacement16(&local_18, &DAT_1050_1050);
    }
    return;
}

pub unsafe fn win_ui_menu_op_1020_7ad2(
    param_1: *mut astruct_854,
    param_2: HWND16,
    param_3: *mut RECT16,
) {
    //   HMENlet mut HVar1: u16;
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_854;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2.field236_0xee.is_null() == false) && (iVar2.field235_0xec == 0)) {
        HVar1 = LoadMenu16(iVar2.field236_0xee, HINSTANCE16_1050_038c);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field235_0xec);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field8_0x8);
    HVar1 = iVar2.field235_0xec;
    TrackPopupMenu16(NULL, iVar2.field8_0x8, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}

pub unsafe fn cleanup_ui_op_1008_0618(param_1: *mut astruct_53) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut in_EDX: u32;
    let mut iVar4: *mut astruct_53;
    let mut uVar3: u16;
    let mut puVar1: *mut u32;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1 = 0x389e;
    iVar4.field_0x2 = 0x1008;
    set_sys_color_1008_357e(param_1, 0x0, in_EDX);
    fn_ptr_1000_17ce(*&iVar4.field248_0xf8);
    if (&iVar4.field_0xec != 0) {
        DestroyMenu16(&iVar4.field_0xec);
    }
    DestroyIcon16(&iVar4.field_0xc2);
    iVar4.field_0xc2 = 0;
    puVar1 = &iVar4.field_0xe0;
    uVar1 = &iVar4.field_0xe2;
    if ((uVar1 | puVar1) != 0) {
        ppcVar2 = *puVar1;
        (**ppcVar2)(s_tile2_bmp_1050_1538, puVar1, uVar1, 1);
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
    param_1 = 0x380a;
    iVar4.field_0x2 = 0x1008;
    param_1 = 0x389a;
    iVar4.field_0x2 = 0x1008;
    return;
}

pub unsafe fn cleanup_menu_ui_op_1020_795c(in_struct_1: *mut StructD) {
    let mut local_struct_1: *mut StructD;
    let mut uVar1: *mut StructD;

    uVar1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.address_offset_field_0x0 = 0x7b86;
    local_struct_1.address_offset_field_0x2 = 0x1020;
    if (local_struct_1.hmenu_0xec != 0) {
        DestroyMenu16(local_struct_1.hmenu_0xec);
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.field192_0xd2)));
    in_struct_1.address_offset_field_0x0 = 0x380a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    in_struct_1.address_offset_field_0x0 = 0x389a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn mixed_menu_op_1020_44ec(
    param_1: *mut astruct_850,
    mut param_2: u16,
    mut param_3: i16,
    param_4: HMENU16,
    mut param_5: u16,
    param_6: u8,
) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut UVar4: u16;
    let mut BVar5: bool;
    //   HMENlet mut HVar6: u16;
    let mut HVar6: HMENU16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut puVar9: *mut u8;
    let mut uVar10: u16;
    let mut in_register_0000000a: u16;
    let mut paVar11: *mut Struct57;
    let mut iVar9: *mut astruct_850;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut data: *mut c_char;
    let mut puVar15: *mut u32;
    let mut in_stack_0000fd70: u16;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000fe9e: u16;
    let mut w_flags: u16;
    let mut in_stack_0000fec8: u16;
    //   HMENlet mut w_item_id: u16;
    let mut w_ite_id: HMENU16;
    let mut uStack300: u16;
    let mut bStack293: u8;
    let mut uStack278: u16;
    let mut uStack268: u32;
    let mut local_108: [u32; 0x40] = [0; 0x40];
    let mut uStack8: u16;
    let mut puStack6: *mut u32;

    paVar11 = CONCAT22(in_register_0000000a, param_5);
    uVar13 = (param_1 >> 0x10);
    iVar9 = param_1;
    if (iVar9.hmenu_0x106 != 0) {
        if (iVar9.hmenu_0x106 == param_4) {
            puStack6 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000fec8, 0x3),
                in_stack_0000fd70,
                in_stack_0000fe94,
                in_stack_0000fe9a,
                in_stack_0000fe9e,
            );
            uVar2 = iVar9.field257_0x108;
            uStack8 = (uVar2 + 0x2e);
            uVar2 = iVar9.field257_0x108;
            uVar14 = (uVar2 >> 0x10);
            iVar12 = uVar2;
            uVar1 = (iVar12 + 0x42);
            puVar9 = (iVar12 + 0x44);
            bStack293 = (uVar1 >> 0x18);
            uVar7 = bStack293;
            if (bStack293 == 0) {
                uVar3 = pass1_1020_bd80(uStack8);
                unk_str_op_1000_3d3e(CONCAT22(0x1050, local_108), CONCAT22(puVar9, uVar3));
            } else {
                pass1_1030_8344(_u16_1050_5748, uVar1 & 0xffff | ZEXT24(puVar9) << 0x10);
                pass1_1010_c3c2(
                    puVar9,
                    puStack6,
                    (puStack6 >> 0x10),
                    CONCAT22(0x1050, local_108),
                    CONCAT22(puVar9, uVar7),
                );
            }
            ModifyMenu16(
                CONCAT22(0x1050, local_108),
                0x76,
                0x0,
                0x76,
                iVar9.hmenu_0x106,
            );
            UVar4 = GetMenuState16(0x0, 0x13c, iVar9.hmenu_0x106);
            if (UVar4 != 0xffff) {
                DeleteMenu16(0x0, 0x13c, iVar9.hmenu_0x106);
            }
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x20);
            if (BVar5 != 0) {
                data = load_string_1010_847e(_u16_1050_14cc, 0x74b);
                InsertMenu16(data, 0x13c, 0x400, 0xffff, iVar9.hmenu_0x106);
            }
            if ((s_VrMode_1050_42ca + 0x8 + uStack8 * 0x2) == 0) {
                HVar6 = iVar9.hmenu_0x106;
                w_flags = 0x1;
                UVar4 = 0x77;
                // TODO: goto LAB_1020_464c;
            }
            HVar6 = iVar9.hmenu_0x106;
            UVar4 = 0x77;
        } else {
            HVar6 = GetSubMenu16(0x1, iVar9.hmenu_0x106);
            //      if (HVar6 != param_4) goto LAB_1020_479e;
            EnableMenuItem16(0x1, 0x200, HVar6);
            uVar10 = paVar11;
            EnableMenuItem16(0x1, 0x201, HVar6);
            EnableMenuItem16(0x1, 0x202, HVar6);
            uVar2 = iVar9.field257_0x108;
            uVar8 = (uVar2 + 0x42);
            pass1_1030_8344(_u16_1050_5748, uVar8);
            uVar7 = uVar8;
            if ((uVar10 | uVar7) == 0) {
                return;
            }
            uVar2 = (uVar7 + 0x2e);
            uVar7 = (uVar7 + 0x30);
            uStack278 = uVar2;
            if ((uVar7 | uStack278) == 0) {
                return;
            }
            uStack268 = (uStack278 + 0x200);
            local_108[0] = struct_op_1030_73a8((uVar8 & 0xffff | uVar10 << 0x10), uStack268, uVar7);
            uVar13 = (local_108[0] >> 0x10);
            puStack6 = (local_108[0] + 0x1c);
            uVar7 = (local_108[0] + 0x1e);
            if ((uVar7 | puStack6) != 0) {
                uStack268 = puStack6 & 0xffff | uVar7 << 0x10;
            }
            uStack268 &= 0xff;
            if (uStack268 != 1) {
                return;
            }
            if ((uStack268 & 0xff0000) != 0) {
                return;
            }
            uVar3 = pass1_1030_6fa0(uVar8 & 0xffff | uVar10 << 0x10);
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar3, 0x3f);
            if (BVar5 != 0) {
                BVar5 = EnableMenuItem16(0x0, 0x201, HVar6);
            }
            if (((uVar8 & 0xffff) + 0x36) != 0) {
                BVar5 = EnableMenuItem16(0x0, 0x202, HVar6);
            }
            pass1_1030_69cc(BVar5, uStack268, uVar8 & 0xffff | uVar10 << 0x10);
            if (BVar5 == 0) {
                return;
            }
            UVar4 = 0x200;
        }
        w_flags = 0;
        // TODO: goto LAB_1020_464c;
    } //
    // LAB_1020_479e:
    iVar12 = param_3 - 0x1;
    if (iVar12 == 0) {
        pass1_1018_2504(0x0, paVar11);
        if (iVar12 == 0) {
            UVar4 = 0;
            EnableMenuItem16(0x401, 0x0, param_4);
            HVar6 = 0x1; //
            // LAB_1020_47e3:
            w_flags = 0x401;
            // TODO: goto LAB_1020_464c;
        }
        UVar4 = 0;
        EnableMenuItem16(0x400, 0x0, param_4);
        HVar6 = 0x1;
    } else if (param_3 == 0x2) {
        uVar3 = pass1_1020_64d4(iVar9.field246_0xf6, 0x2);
        if (uVar3 == 0) {
            EnableMenuItem16(0x401, 0x0, param_4);
            UVar4 = 0x401;
        } else {
            EnableMenuItem16(0x400, 0x0, param_4);
            UVar4 = 0x400;
        }
        HVar6 = 0x1;
        EnableMenuItem16(UVar4, 0x1, param_4);
        //    if ((PTR_LOOP_1050_0010.is_null() == false) || (iVar9.field255_0x102 == 0)) goto LAB_1020_47e3;
    } else {
        if (param_3 == 0x3) {
            HVar6 = 0;
            puVar15 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                0x2f,
                in_stack_0000fd70,
                in_stack_0000fe94,
                in_stack_0000fe9a,
                in_stack_0000fe9e,
            );
            uVar13 = (puVar15 >> 0x10);
            uVar1 = (puVar15 + 0x20);
            uVar7 = (puVar15 + 0x22);
            uStack300 = uVar1;
            if ((uVar7 | uStack300) != 0) {
                pass1_1030_8308(
                    &stack0xfecc,
                    uVar7,
                    _u16_1050_5748,
                    (_u16_1050_5748 >> 0x10),
                    CONCAT22(0x1050, &stack0xfecc),
                    CONCAT22(0x1050, &stack0xfec8),
                    uVar1 & 0xffff | uVar7 << 0x10,
                );
            }
            UVar4 = 0;
            loop {
                CheckMenuItem16(0x400, UVar4, param_4);
                w_item_id = param_4;
                EnableMenuItem16(0x401, UVar4, param_4);
                UVar4 += 0x1;
                if UVar4 >= 5 {
                    break;
                }
            }
            CheckMenuItem16(0x408, w_item_id, param_4);
            //   for (UVar4 = 0; UVar4 <= HVar6; UVar4 += 1)
            for UVar4 in 0..HVar6 {
                HVar6 = param_4;
                EnableMenuItem16(0x400, UVar4, param_4);
            }
            return;
        }
        if (param_3 != 0x4) {
            return;
        }
        UVar4 = 0x2;
        HVar6 = param_4;
    }
    w_flags = 0x400; //
    // LAB_1020_464c:
    EnableMenuItem16(w_flags, UVar4, HVar6);
    return;
}

pub unsafe fn enable_menu_1020_1000(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: HMENU,
) {
    if (param_4 != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0x3, param_5);
    return;
}


pub unsafe fn enable_menu_item_1020_2c2a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: HMENU16,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut w_flags: u16;

    if (param_4 != 0) {
        return param_4 - 0x1;
    }
    EnableMenuItem16(0x400, 0x3, param_5);
    if (PTR_LOOP_1050_3960 < 0x2) {
        w_flags = 0x401;
    } else {
        w_flags = 0x400;
    }
    BVar1 = EnableMenuItem16(w_flags, 0x5, param_5);
    return BVar1;
}

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
    if (G_SM_CYCAPTION_1050_416c == 0) {
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
    uVar8 = SUB42(&DAT_1050_1050, 0x0);
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
    uVar9 = SUB42(&DAT_1050_1050, 0x0);
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
    uVar8 = SUB42(&DAT_1050_1050, 0x0);
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


pub unsafe fn win_ui_op_1040_81fe(mut param_1: u32)

{
    SetSysModalWindow((param_1 + 0x6));
    return;
}
