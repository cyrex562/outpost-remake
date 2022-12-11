use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr;
use crate::block_1000::block_1000_1000::fn_ptr_1000_17ce;
use crate::block_1008::block_1008_5000::win_proc_1008_5f44;
use crate::block_1018::block_1018_2000::{pass1_1018_2afa, pass1_1018_2d84};
use crate::block_1018::block_1018_3000::pass1_1018_30fc;
use crate::prog_types::{ATOM, HMENU16, HWND16, LRESULT, RECT16, WNDCLASS16, WPARAM16};
use crate::string_defs::{s__1050_4415, s_listbox_1050_4416, s_MciSound_registerClass_failed_1050_02cc, s_MciSoundWindow_1050_02bd};
use crate::structs_2_h::{Struct57, StructA};
use crate::sys_api::{CreateWindow16, DestroyWindow16, GetClassInfo16, GetClientRect16, GetDlgItem16, GetStockObject16, OutputDebugString16, RegisterClass16, SendMessage16, SetFocus16, ShowWindow16};
use crate::utils::{CONCAT22, SUB42};

pub unsafe fn create_window_1008_5e7e() ->HWND16
{
  let mut puVar2: *mut c_char;
  let mut BVar3: bool;
  // AVar4: ATOM;
    let mut AVar4: ATOM;
  let mut window_handle_1: HWND16;
  let mut iVar5: i16;
  let mut string_1: *mut c_char;
  let mut puVar5: *mut c_char;
  let mut wndclass_44: WNDCLASS16;
  // let mut local_12: [u32;0x4] = [0;0x4];
  let mut local_12: [c_char;16] = [0;16];
    let mut puVar1: *mut c_char;

  puVar5 = local_12.as_mut_ptr();
  string_1 = s_MciSoundWindow_1050_02bd.into_raw();
    iVar5 = 0x3;
  // for (iVar5 = 0x3; iVar5 != 0; iVar5 += -1) {
    while iVar5 != 0 {
        puVar2 = puVar5;
        puVar5 = puVar5 + 1;
        puVar1 = string_1;
        string_1 = (string_1 + 0x4);
        puVar2 = puVar1;
        iVar5 -= 1;
    }
  puVar5 = string_1;
  (puVar5 + 0x2) = (string_1 + 2);
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
  wndclass_44.lpsz_menu_name = ptr::null_mut();
  wndclass_44.lpsz_class_name = local_12.as_mut_ptr();
  // wndclass_44.lpsz_class_name = SUB42(&DAT_1050_1050,0x0);
    let mut wc_ptr: *mut WNDCLASS16 = &mut wndclass_44;
    let class_name: *const c_char = local_12.as_ptr();
  BVar3 = GetClassInfo16(wc_ptr,class_name, 0);
  if BVar3 == false {
    AVar4 = RegisterClass16(wc_ptr);
    if AVar4 == 0x0 {
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
  // astruct_878 **ppaVar1;
    let mut pstruct878_var1: *mut Struct878;
  let mut u32_var2: u32;
  let mut window_handle: HWND16;
  let mut u16_var3: u16;
  let mut u16_var5: u16;
  let pstructa_var6: *mut StructA;
  let mut u16_var7: u16;
  let mut lresult_var8: LRESULT;
  let mut string_var9: *mut c_char;
  let mut wparam_var10: WPARAM16;
  let mut u16_var11: u16;
  let mut hwnd_var12: HWND16;
  let mut win_style: u32;
  let mut rectangle: RECT16;
  let mut hwnd_var13: HWND16 = 0;
  let mut i16_var4: i16 = 0i16;
  // iVar9: *mut astruct_878;
    let mut iVar9: *mut Struct878;

  // uVar5 = (param_2 >> 0x10);
  pstructa_var6 = param_2;
    let rect_ptr: *mut RECT16 = &mut rectangle;
  GetClientRect16(rect_ptr,0);
  win_style = 0;
  window_handle = GetDlgItem16(0x1797, pstructa_var6.field4_0x8);
  if window_handle != 0x0 {
    DestroyWindow16(window_handle);
  }
  pass1_1018_30fc(param_1, pstructa_var6[0x1].field20_0x26, &mut win_style);
  if (win_style) != 0x0 {
      // CONCAT22(0x1797,HINSTANCE16_1050_038c)
      let mut hinst = HINSTANCE16_1050_038c;
      let mut data: *mut c_void = &pstructa_vart.field4_0x8 as *mut c_void;
      let mut cw_hmenu: HMENU16 = (i16_var4 - 0x19) as HMENU16;
    window_handle = CreateWindow16(win_style ,data, hinst, cw_hmenu,
                                   hwnd_var13, 0x0, 0x0, 0x103, 0x40a0, s__1050_4415.as_ptr(), s_listbox_1050_4416.as_ptr());
    u32_var2 = win_style;
    if window_handle == 0x0 {
      if (win_style) != 0x0 {
        pass1_1018_2afa(win_style, 0);
        fn_ptr_1000_17ce(u32_var2);
        return;
      }
    }
    else {
      lresult_var8 = SendMessage16(0x0, 0x0, 0xb, window_handle);
      u16_var5 = (lresult_var8 >> 0x10);
      if (win_style + 0x4) == 0x0 {
        wparam_var10 = 0;
        u16_var11 = 0x401;
        hwnd_var12 = window_handle;
        string_var9 = load_string_1010_847e(_u16_1050_14cc, 0x531);
        SendMessage16(string_var9, wparam_var10, u16_var11, hwnd_var12);
      }
      else {
        iVar9 = NULL;
        loop {
          ppaVar1 = (astruct_878 **)(win_style + 0x4);
          if *ppaVar1 == iVar9 || *ppaVar1 < iVar9 { break; };
          wparam_var10 = 0;
          u16_var11 = 0x401;
          hwnd_var12 = window_handle;
          u16_var3 = pass1_1020_bd80((win_style + iVar9 * 0x2));
          lresult_var8 = SendMessage16(u16_var3, wparam_var10, u16_var11, hwnd_var12);
          // u16_var5 = (lresult_var8 >> 0x10);
          iVar9 = (u16_var11 + 1);
        }
      }
      lresult_var8 = SendMessage16(0x0, 0x1, 0xb, window_handle);
      // u16_var5 = (lresult_var8 >> 0x10);
      u16_var3 = lresult_var8;
      wparam_var10 = 0xffff;
      u16_var11 = 0x40d;
      hwnd_var12 = window_handle;
      pass1_1018_2d84(u16_var3, &pstructa_var6[0x1].field20_0x26);
      lresult_var8 = SendMessage16(u16_var3, wparam_var10, u16_var11, hwnd_var12);
      wparam_var10 = lresult_var8;
      if (wparam_var10 != 0xffff) || ((lresult_var8 >> 0x10) != -1) {
        SendMessage16(0x0, wparam_var10, 0x407, window_handle);
        SendMessage16(0x0, wparam_var10, 0x418, window_handle);
      }
      if (win_style != 0) {
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
