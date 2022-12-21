use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr;
use std::ptr::null_mut;
use crate::block_1000::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::block_1000::block_1000_3000::{pass1_1000_3cea, str_op_1000_3da4, unk_str_op_1000_3d3e};
use crate::block_1000::block_1000_4000::str_1000_4d58;
use crate::block_1008::block_1008_1000::win_ui_op_1008_1414;
use crate::block_1008::block_1008_5000::win_proc_1008_5f44;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1010::block_1010_5000::struct_1010_5f1e;
use crate::block_1010::block_1010_7000::show_window_1010_7ace;
use crate::block_1010::block_1010_8000::{load_string_1010_847e, load_string_1010_84e0};
use crate::block_1010::block_1010_9000::{pass1_1010_9044, pass1_1010_9130, pass1_1010_91cc, pass1_1010_9210, struct_1010_9172};
use crate::block_1010::block_1010_c000::pass1_1010_c3c2;
use crate::block_1018::block_1018_2000::{pass1_1018_2afa, pass1_1018_2d84};
use crate::block_1018::block_1018_3000::pass1_1018_30fc;
use crate::block_1018::block_1018_5000::{pass1_1018_50ea, pass1_1018_57d2};
use crate::block_1020::block_1020_0000::send_msg_1020_097e;
use crate::block_1020::block_1020_b000::pass1_1020_bd80;
use crate::block_1030::block_1030_8000::pass1_1030_8344;
use crate::block_1038::block_1038_a000::pass1_1038_af40;
use crate::block_1040::block_1040_b000::pass1_1040_b54a;
use crate::string_defs::{s__1050_4415, s_listbox_1050_4416, s_MciSound_registerClass_failed_1050_02cc, s_MciSoundWindow_1050_02bd};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_72::Struct72;
use crate::structs::struct_878::Struct878;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_929::Struct929;
use crate::structs::struct_a::StructA;
use crate::winbase::{CreateWindow16, DestroyWindow16, EnableWindow16, GetClassInfo16, GetClientRect16, GetDlgItem16, GetDlgItemInt16, GetStockObject16, LoadCursor16, MessageBox16, OutputDebugString16, PostMessage16, RegisterClass16, SendDlgItemMessage16, SendMessage16, SetCursor16, SetDlgItemInt16, SetFocus16, ShowWindow16, UpdateWindow16};
use crate::utils::{CONCAT22, SUB42};
use crate::windef::{ATOM, HMENU16, HWND16, LPARAM, LRESULT, RECT16, WNDCLASS16, WPARAM16};

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
