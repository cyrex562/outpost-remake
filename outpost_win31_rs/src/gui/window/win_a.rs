use std::ptr::null_mut;
use std::os::raw::c_char;
use std::ffi::c_void;
use crate::draw_ops::draw_a::draw_op_1040_9948;
use crate::globals::PTR_LOOP_1050_1040;
use crate::resources;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::string_defs::{s__1050_4415, s_listbox_1050_4416, s_MciSound_registerClass_failed_1050_02cc, s_MciSoundWindow_1050_02bd};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_72::Struct72;
use crate::structs::struct_878::Struct878;
use crate::structs::struct_a::StructA;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1000_3000::{str_op_1000_3da4, unk_str_op_1000_3d3e};
use crate::unk::block_1008_3000::pass1_1008_3bd6;
use crate::unk::block_1008_5000::win_1008_5c7c;
use crate::unk::block_1010_2000::{mixed_1010_20ba, pass1_1010_2ee2};
use crate::unk::block_1018_1000::{pass1_1018_1c9a, pass1_1018_1e78};
use crate::unk::block_1018_2000::pass1_1018_2d84;
use crate::unk::block_1018_3000::pass1_1018_30fc;
use crate::unk::block_1020_b000::pass1_1020_bd80;
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_9000::pass1_1040_9824;
use crate::unk::block_1040_b000::{pass1_1040_b54a, pass1_1040_bfde, struct_1040_bf3e};
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::gui::{cursor, dialog, window};
use crate::gui::dialog::dlg_b;
use crate::gui::window::{win_c, win_e};
use crate::winapi16::{BringWindowToTop16, CheckDlgButton16, CreateWindow16, DefWindowProc16, DestroyWindow16, EnableWindow16, GetClassInfo16, GetClientRect16, GetCursorPos16, GetDlgCtrlID16, GetDlgItem16, GetNextDlgTabItem16, GetStockObject16, GetWindowLong16, GetWindowRect16, InvalidateRect16, IsDlgButtonChecked, IsWindow16, LoadCursor16, OutputDebugString16, PostMessage16, PtInRect16, RegisterClass16, ReleaseCapture16, SendDlgItemMessage16, SendMessage16, SetCapture16, SetCursor16, SetFocus16, SetWindowLong16, SetWindowPos16, SetWindowText16, ShowWindow16, UpdateWindow16};
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::winapp::winapp_a::{create_window_1040_6eae, create_window_1040_7620, win_proc_1008_5f44};
use crate::windef16::{ATOM, BOOL16, HCURSOR16, HMENU16, HWND16, LPARAM, LRESULT, RECT16, WNDCLASS16, WPARAM16};

pub fn win_ui_fn_1020_6e98(mut param_1: *mut Struct57, param_2: *mut StructA)

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

pub fn show_win_1038_b634(mut param_1: *mut c_void)

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

pub fn show_win_1038_b68a(param_1: *mut c_void)

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

pub fn ui_op_1008_2c4e(mut param_1: u16, mut param_2: u16, param_3: *mut Struct72, mut param_4: u16 )

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
    uVar5 = (**fn_ptr_1)(0x1538,uVar2,(uVar2 >> 0x10));
  }
  big_switch_1008_15d4(paVar2,param_3,CONCAT22(param_2,param_4));
  uVar3 = (uVar5 >> 0x10);
  (param_3)[0x1].field5_0xc = uVar5;
  (param_3)[0x1].field6_0xe = uVar3;
  fn_ptr_1 = (*&(param_3)[0x1].field5_0xc + 0x8);
  (**fn_ptr_1)(0x1538,(param_3)[0x1].field5_0xc,uVar3);
  if ((&(param_3)[0x1].field2_0x4 + 0x2) != 0) {
    uVar1 = (&(param_3)[0x1].field2_0x4 + 2);
    fn_ptr_1 = ((&(param_3)[0x1].field2_0x4 + 0x2) + 0xc)
    ;
    (**fn_ptr_1)(0x1538,uVar1,(uVar1 >> 0x10),0x0);
  }
  show_win_1038_b634(_PTR_LOOP_1050_5b7c);
  win_c::show_win_1010_7a76((&(param_3)[0x1].field7_0x10 + 0x2));
  uVar1 = &(param_3)[0x1].field5_0xc;
  fn_ptr_1 = (*&(param_3)[0x1].field5_0xc + 0xc);
  (**fn_ptr_1)(0x1010,uVar1,(uVar1 >> 0x10),0x5);
  uVar1 = &(param_3)[0x1].field5_0xc;
  BringWindowToTop16((uVar1 + 0x8));
  SetCursor16(hcursor_5);
  return;
}


pub fn post_msg_1008_2d22(param_1: *mut Struct72)

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


pub fn bring_win_to_top_1038_b72e(mut param_1: u32, mut param_2: i16) -> BOOL16

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


pub fn win_ui_op_1038_b81c(mut param_1: u16, struct_b_param_1: *mut StructB)

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
  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
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
  (**ppcVar3)(0x1538,uVar2,(uVar2 >> 0x10));
  piStack16 = CONCAT22(extraout_DX,win_enabled);
  window::move_win_1040_826c(struct_b_param_1, win_enabled.field1_0x2 -0x2, win_enabled.field2_0x4 + *piStack16 + 0x3);
  ShowWindow16(0x5,struct_b_8.lpvoid_field_0x8);
  pass1_1018_1c9a(&struct_b_8[0x7].lpvoid_field_0x8,*piVar6);
  HVar5 = GetDlgItem16(*piVar6,struct_b_8.lpvoid_field_0x8);
  SetFocus16(HVar5);
  return;
}

pub fn win_ui_op_1038_b922(param_1: *mut StructD, param_2: *mut StructC, mut param_3: u32, mut param_4: u16, mut param_5: u16 ) -> u32

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
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x50,local_464,0x1050);
      pcVar11 = load_string_1010_847e(_u16_1050_14cc,*puStack20);
      uVar15 = (pcVar11 >> 0x10);
      paVar12 = CONCAT22(uVar9,uVar15);
      uVar13 = SUB21(local_464,0x0);
      uVar6 = wsprintf16(local_414,0x5bc01050,
                         CONCAT13((local_464 >> 0x8),CONCAT12(uVar13,0x1050)),uVar13,
                         0x1050,pcVar11,uVar15);
      uVar9 = 0x1000;
      mem_op_1000_179c(0xb4,paVar12);
      uVar7 = paVar12;
      if ((uVar7 | uVar6) == 0) {
        uVar6 = 0;
        paVar12 = null_mut();
      }
      else {
        pWVar14 = local_414;
        uVar15 = SUB42(0x1050,0x0);
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
      cursor::win_ui_cursor_op_1038_bc30(param_2);
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


pub fn win_ui_op_1040_a784(param_1: u8, mut param_2: u16, mut param_3: i16, mut param_4: i16, mut param_5: u16, mut param_6: u32)

{
    let mut hwnd: HWND16;
    let mut iVar1: i16;

    iVar1 = param_3;
    if (param_6 != 0xeb) {
        if (param_6 == 0x1f4) {
            resources::msg_box_op_1040_a85a(0x0, param_2, CONCAT22(param_4, param_3));
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
        iVar1 = 0x1538;
        EnableWindow16(0x0, hwnd);
        (param_3 + 0x98) = 0x1;
        param_4 = param_3;
    }
    dlg_b::win_ui_dlg_op_1040_a94a(param_2, CONCAT22(param_4, iVar1));
    return;
}

pub fn create_window_1008_5e7e() ->HWND16
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
  // wndclass_44.lpsz_class_name = SUB42(0x1050,0x0);
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


pub fn mixed_win_ui_op_1040_6942(param_1: *mut Struct57, mut param_2: u16, struct_b_param_1: *mut StructB, mut param_4: u16, mut param_5: u16)

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

    dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
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
    win_e::enable_win_1040_9234(CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, paVar3)), BVar21);
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
    win_e::enable_win_1040_9234(CONCAT13((uStack4 >> 0x8), CONCAT12(uStack4, paVar3)), BVar21);
    hdc_8 = GetDC16(struct_b_6.lpvoid_field_0x8);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, 0x1050);
    uVar22 = SUB42(0x1050, 0x0);
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
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, 0x1050);
    uVar18 = hdc_8;
    uVar19 = (hdc_8 >> 0x8);
    pcVar23 = local_58;
    uVar22 = SUB42(0x1050, 0x0);
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
    (**ppcVar2)(0x1538, uVar2, (uVar2 >> 0x10));
    pass1_1010_2ee2(&struct_b_6[0x7].max_count_field_0x10);
    PostMessage16(0x0, 0x10a, 0x111, struct_b_6.lpvoid_field_0x8);
    return;
}


pub fn mixed_win_ui_op_1040_70b4(param_1: *mut Struct57, mut param_2: u16, struct_b_param_1: *mut StructB, mut param_4: u16, mut param_5: u16)

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

    dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
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
    win_e::enable_win_1040_9234(CONCAT13((paVar5 >> 0x8), CONCAT12(paVar5, paVar2)), BVar13);
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
    win_e::enable_win_1040_9234(CONCAT13((uStack4 >> 0x8), CONCAT12(uStack4, paVar2)), BVar13);
    HStack8 = GetDC16(struct_b_5.lpvoid_field_0x8);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, 0x1050);
    uVar16 = SUB42(0x1050, 0x0);
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
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, 0x1050);
    uVar14 = HStack8;
    uVar15 = (HStack8 >> 0x8);
    pcVar17 = local_58;
    uVar16 = SUB42(0x1050, 0x0);
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
    (**fn_ptr_1)(0x1538, uVar2, (uVar2 >> 0x10));
    pass1_1010_2ee2(&struct_b_5[0x7].max_count_field_0x10);
    PostMessage16(0x0, 0x10a, 0x111, struct_b_5.lpvoid_field_0x8);
    return;
}
