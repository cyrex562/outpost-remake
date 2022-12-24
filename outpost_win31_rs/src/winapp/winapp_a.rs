use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::gui::dialog::dialog_ui_fn_1040_78e2;
use crate::gui::window::move_win_1040_826c;
use crate::no_refs::s;
use crate::resources::load_string_1010_847e;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1008_5000;
use crate::unk::block_1008_5000::{set_struct_1008_574a, win_1008_5c5c};
use crate::unk::block_1008_9000::pass1_1008_941a;
use crate::unk::block_1010_1000::pass1_1010_1ea6;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_3000::{pass1_1010_375e, pass1_1010_3770};
use crate::unk::block_1010_8000::FUN_1010_830a;
use crate::unk::block_1020_6000::{pass1_1020_64d4, pass1_1020_6746};
use crate::unk::block_1030_8000::fn_ptr_1030_835a;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_8000::string_1040_8520;
use crate::utils::{CONCAT22, SUB42};
use crate::winapi16::{CallWindowProc16, DestroyWindow16, GetClientRect16, GetDlgItem16, GetProp16, GetWindowRect16, IsWindow16, PostMessage16, PtInRect16, ReleaseCapture16, SendMessage16, SetCapture16, SetWindowLong16, SetWindowPos16, ShowWindow16, WinHelp16};
use crate::winapp;
use crate::winapp::{winapp_b, winapp_c};
use crate::windef16::{HANDLE16, HWND16, LPARAM, LRESULT, RECT16, WPARAM16};

pub unsafe fn unk_win_sys_op_1038_da68(param_1: *mut StructD, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut in_BX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u8;
  let mut iVar9: i16;
  let mut puStack14: *mut u32;
  let mut uStack8: u16;
  let mut iStack4: i16;

  paVar5 = CONCAT22(in_register_0000000a,param_1);
  uVar8 = (param_3 >> 0x8);
  if (param_4 == 0x204) {
    pass1_1038_de20(in_BX,param_1,CONCAT13(uVar8,CONCAT12(param_3,param_2)),0x204,param_5,
                    param_5);
    return;
  }
  iStack4 = 0;
  uStack8 = 0;
  if (param_5 == 0x121) {
    iStack4 = 0x6ec;
    uStack8 = 0x15;
// TODO: goto LAB_1038_dac3;
  }
  if (param_5 < 0x1220000) {
    uVar2 = param_5 - 0x100;
    if (uVar2 == 0) {
      param_5 = uVar2;
      if ((param_2 + 0x8e) == 0) {
        pass1_1010_1ea6(_u16_1050_02a0,CONCAT22(param_3,param_2));
        (param_2 + 0x90) = 0;
      }
      iStack4 = 0x72c;
      uStack8 = 0x48;
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 - 0x11c == 0) {
      param_5 = param_5 - 0x11c;
      pass1_1038_df86(param_1,CONCAT22(param_3,param_2));
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 == 0x11d) {
      uVar7 = pass1_1038_df5c(param_1,CONCAT22(param_3,param_2));
      paVar5 = (paVar5 & 0xffff0000 | uVar7 >> 0x10);
      param_5 = uVar7;
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 == 0x11e) {
      iVar9 = 0x1d;
    }
    else {
      if (param_5 != 0x120) {//
// LAB_1038_dc20:
        winapp_b::post_win_msg_1040_7b3c
                  (CONCAT13(uVar8,CONCAT12(param_3,param_2)),param_4,param_5,param_5);
        return;
      }
      iVar9 = 0x1c;
    }
  }
  else if (param_5 == 0x122) {
    iVar9 = 0x14;
  }
  else {
    if (param_5 != 0x123) {
      if (param_5 - 0x125 == 0) {
        ppcVar1 = (*_u16_1050_02a0 + 0x4);
        param_5 = param_5 - 0x125;
        (**ppcVar1)();
        (param_2 + 0x90) = 0x1;
        win_1008_5c5c(param_5,paVar5,_u16_1050_02a0,0x1db);
        (param_2 + 0x8e) = 0x100;
      }
      else {
        iVar9 = param_5 - 0x126;
        if (iVar9 == 0) {
          (param_2 + 0x8e) = 0;
          win_1008_5c7c(0x0,param_1,_u16_1050_02a0,0xcb0001);
          uVar3 = FUN_1010_830a(iVar9,paVar5,0x1008,_u16_1050_14cc,0x1f8);
          param_5 = WinHelp16(0x0,0x3,CONCAT22(paVar5,uVar3),(param_2 + 0x6));
        }
        else {
//          if (param_5 - 0x127 != 0) goto LAB_1038_dc20;
          param_5 = param_5 - 0x127;
          winapp_b::post_win_msg_1038_dcb0(0x0, paVar5, CONCAT22(param_3, param_2));
        }
      }
  // TODO: goto LAB_1038_dac3;
    }
    iVar9 = 0x28;
  }
  uVar7 = pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),iVar9);
  paVar5 = (paVar5 & 0xffff0000 | uVar7 >> 0x10);
  param_5 = uVar7;//
// LAB_1038_dac3:
  if (iStack4 != 0) {
    mem_op_1000_179c(0xb4,paVar5);
    uVar4 = paVar5 | param_5;
    uVar7 = paVar5 & 0xffff0000 | uVar4;
    if (uVar4 == 0) {
      uVar6 = 0x1000;
      iVar9 = 0;
      uVar3 = 0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar9 = string_1040_8520(uVar7,
                                     CONCAT13((paVar5 >> 0x8),CONCAT12(paVar5,param_5)),
                               (param_2 + 0x6),0x20000,CONCAT22(iStack4,0x634));
      uVar3 = uVar7;
    }
    puStack14 = CONCAT22(uVar3,iVar9);
    if (uStack8 == 0) {
      ppcVar1 = (*puStack14 + 0x74);
      (**ppcVar1)(uVar6,iVar9,uVar3);
    }
    else {
      pass1_1008_941a(CONCAT22(0x1050,&stack0xffea),0x1,uStack8);
      ppcVar1 = (*puStack14 + 0x6c);
      (**ppcVar1)(0x1008,iVar9,uVar3,&stack0xffea,0x1050);
    }
  }
  return;
}


pub unsafe fn create_win_1040_20d4(mut param_1: u32, struct_b_param_2: *mut StructB, mut param_3: u16)

{
    let mut cx: i16;
    let mut struct_b_1: *mut StructB;
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut window_name: *mut c_char;
    let mut in_stack_0000fe72: u16;
    let mut in_stack_0000ff96: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut local_1e: RECT16;
    let mut iStack26: i16;
    let mut iStack24: *mut astruct_858;
    let mut uStack22: u32;
    let mut uStack18: u32;
    let mut iStack14: i16;
    let mut uStack12: u16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut uStack6: u16;
    let mut iStack4: i16;

    dialog_ui_fn_1040_78e2(struct_b_param_2);
    puVar2 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_3, 0x48), in_stack_0000fe72,
                             in_stack_0000ff96, in_stack_0000ff9c, in_stack_0000ffa0);
    uStack12 = (puVar2 >> 0x10);
    iStack14 = puVar2;
    iStack8 = (iStack14 + 0xa);
    iStack10 = (iStack14 + 0xc);
    uVar1 = (struct_b_param_2 >> 0x10);
    struct_b_1 = struct_b_param_2;
    uStack18 = pass1_1008_4772(&struct_b_1[0x7].field1_0x2);
    cx = (uStack18 + 0x4);
    iStack4 = (iStack8 - cx) / 0x2;
    uStack6 = 0x5;
    SetWindowPos16(0x6, 0x1d6, cx, 0x5, iStack4, 0x0, struct_b_1.lpvoid_field_0x8);
    GetClientRect16(&local_1e, 0x1050);
    window_name = load_string_1010_847e(_u16_1050_14cc, 0x592);
    uStack22 = 0x50010001;
    CreateWindow16(0x0, CONCAT22(0x1, HINSTANCE16_1050_038c), struct_b_1.lpvoid_field_0x8, 0x19, 0x58,
                   (iStack24 - 0x28), (iStack26 - 0x58) / 0x2, 0x1, s_Rebel_1050_4ffc + 0x5, window_name,
                   s_OPButton_1050_5ce4);
    SetWindowPos16(0x45, iStack10 - 0xa, (uStack18 + 0x4), 0x5, iStack4, 0x0,
                   struct_b_1.lpvoid_field_0x8);
    return;
}


pub unsafe fn call_win_proc_1038_d020
               (base_param_6: u16,win_handle_1: HWND16,mut param_3: u32,l_param: HWND16,hwnd_param_4: HWND16,
               win_handle_2: HWND16) -> i32

{
  let mut handle_1: HANDLE16;
  let mut handle_2: HANDLE16;
  let mut handle_3: HANDLE16;
  let mut var1: u16;
  let mut lresult: LRESULT;
  let mut var5: i32;
  let mut var6: *mut u32;
  let mut var7: i32;
  let mut var8: u16;
  let mut fn_ptr_1: *mut *mut code;

  handle_1 = GetProp16(CONCAT22(base_param_6,0x5bd7),hwnd_param_4);
  handle_2 = GetProp16(CONCAT13((base_param_6 >> 0x8),CONCAT12(base_param_6,0x5bd0)),
                       hwnd_param_4);
  var7 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16(CONCAT22(base_param_6,0x5be5),hwnd_param_4);
  handle_3 = GetProp16(CONCAT22(base_param_6,0x5bde),hwnd_param_4);
  var6 = CONCAT22(handle_1,handle_3);
  if ((handle_1 | handle_3) != 0) {
    var5 = 0;
    if (l_param == 0x19) {
      fn_ptr_1 = (*var6 + 0x34);
      var5 = (**fn_ptr_1)(0x1538,handle_3,handle_1,win_handle_1,param_3);
    }
    else {
      if (l_param == 0x86) {
        fn_ptr_1 = (*var6 + 0x20);
        var1 = (**fn_ptr_1)(0x1538,handle_3,handle_1,param_3);
    // TODO: goto LAB_1038_d10e;
      }
      if ((l_param == 0x112) && ((param_3 & 0xfff0) == 0xf140)) {
        lresult = SendMessage16(0x0,0xf140,0x112,&HWND16_1050_0396);
        var1 = (lresult == 0);
    // TODO: goto LAB_1038_d10e;
      }
    }
    if (var5 != 0) {
      return var5;
    }
  }
  if (var7 != 0) {
    lresult = CallWindowProc16(CONCAT22(param_3,win_handle_1),param_3,l_param,hwnd_param_4,handle_2);
    return lresult;
  }
  var1 = 0;//
// LAB_1038_d10e:
  return var1;
}

pub unsafe fn reg_class_1040_98c0(mut param_1: u32)

{
  let mut BVar1: bool;
  let mut AVar2: ATOM;
  let mut wndclass: WNDCLASS16;

  wndclass.lpsz_class_name = param_1 + 0x4;
  BVar1 = GetClassInfo16(&wndclass,CONCAT22(wndclass.lpsz_class_name,0x1050),param_1);
  if (BVar1 == 0) {
    wndclass.style = (param_1 + 0x54);
    wndclass.lpfn_wnd_proc = 0x9cde;
    wndclass.lpfn_wnd_proc = SUB42(&PTR_LOOP_1050_1040,0x0);
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

pub unsafe fn post_quit_msg_1008_3af4() {
    PostQuitMessage16(0x0);
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
            GetClientRect16(&rect_a, 0x1050);
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
            winapp_c::send_msg_1008_84ba(hwnd_param_4, win_long_1);
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
        GetClientRect16(&rect_a, 0x1050);
        if (lparam_param_1 < (iStack4 >> 1)) {
            puVar1 = &iVar3.field3_0x4;
            *puVar1 = *puVar1 | 0x4;
        } else {
            puVar1 = &iVar3.field3_0x4;
            *puVar1 = *puVar1 | 0x8;
        }
        winapp_c::send_msg_1008_84ba(hwnd_param_4, win_long_1);
        SetTimer16(NULL, 0x12c, 0xfa6, hwnd_param_4);
        InvalidateRect16(0x1, NULL, 0x0);
        UpdateWindow16(hwnd_param_4);
        SetCapture16(hwnd_param_4);
    }
    return;
}


pub unsafe fn create_window_1040_8bea
                   (param_1: *mut Struct37,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 ,param_5: INT16,param_6: INT16,param_7: *mut c_char) -> HANDLE16

{
  let mut hwnd: HWND16;
  let mut wparam: HANDLE16;
  let mut LVar1: LRESULT;
  let mut uStack6: u32;

  uStack6 = 0x50010000;
  if (param_3 != 0) {
    uStack6 = 0x50010001;
  }
  if (&param_1.field_0x74 != 0) {
    uStack6 |= 0x8000000;
  }
  hwnd = CreateWindow16(0x0,CONCAT22(param_4,HINSTANCE16_1050_038c),
                        (&param_1.field1_0x4 + 0x2),0x17,0x58,param_6,param_5,uStack6,
                        (uStack6 >> 0x10),param_7,s_OPButton_1050_5e00);
  wparam = GetProp16(s_hfont_1050_5e09,(&param_1.field1_0x4 + 0x2));
  if (wparam != 0) {
    LVar1 = SendMessage16(0x1,wparam,0x30,hwnd);
    wparam = LVar1;
  }
  return wparam;
}


pub unsafe fn create_window_1040_92dc(param_1: *mut Struct65)

{
  let mut hwnd: HWND16;
   let mut pstruct65_2: *mut Struct65;
   let mut pstruct65_1: *mut Struct65;
  let mut lVar1: i32;

  pstruct65_1 = (param_1 >> 0x10);
  pstruct65_2 = param_1;
  if (pstruct65_2.field11_0x18 == 0) {
    hwnd = CreateWindow16(0x0,CONCAT22(pstruct65_2.field13_0x1c,HINSTANCE16_1050_038c),
                          pstruct65_2.field12_0x1a,0x0,0x0,pstruct65_2.field15_0x20,pstruct65_2.field14_0x1e,0xb,
                          0x4000,s__1050_5e3e,s_button_1050_5e3f);
    pstruct65_2.field11_0x18 = hwnd;
    lVar1 = SetWindowLong16(_u16_1050_5e18,-0x4,hwnd);
    lVar1 = (lVar1 >> 0x10);
    pstruct65_2.field10_0x14 = lVar1;
   (&pstruct65_2.field10_0x14 + 0x2) = lVar1;
    SetProp16(lVar1,s_procHi_1050_5e46,pstruct65_2.field11_0x18);
    SetProp16(&pstruct65_2.field10_0x14,s_procLo_1050_5e4d,pstruct65_2.field11_0x18);
    SetProp16(pstruct65_1,s_thisHi_1050_5e54,pstruct65_2.field11_0x18);
    SetProp16(pstruct65_2,s_thisLo_1050_5e5b,pstruct65_2.field11_0x18);
    if (pstruct65_2[0x1].field12_0x1a != 0) {
      SetProp16(0x1,s_IsDlg_1050_5e62,pstruct65_2.field11_0x18);
    }
    ShowWindow16(0x5,pstruct65_2.field11_0x18);
  }
  return;
}

pub unsafe fn unk_win_msg_op_1008_0a3c(mut param_1: u32, mut param_2: u16) -> u16 {
    let mut BVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if ((param_2 & 0xfff0) == 0xf140) {
        return (iVar2 + 0xde);
    }
    if ((param_2 & 0xfff0) == 0xf060) {
        BVar1 = IsIconic16((iVar2 + 0x8));
        if (BVar1 == 0) {
            PostMessage16(0x0, 0x67, 0x111, (iVar2 + 0x8));
        }
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn unk_win_op_1020_65cc(param_1: *mut astruct_60, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut iVar4: *mut astruct_59;
    let mut iVar5: *mut astruct_60;
    let mut iVar6: i16;
    let mut uVar7: *mut astruct_60;
    let mut iStack4: i16;

    iVar5 = param_1;
    uVar7 = (param_1 >> 0x10);
    if (param_2 == 1) {
        iVar5.field20_0x14 = 0;
        return;
    }
    if (param_2 == 0x2) {
        for iStack4 in 0..0x5 {
            iVar6 = iStack4 * 0x4;
            if (((&iVar5.field_0x1a + iVar6) | (&iVar5.field_0x18 + iVar6)) != 0) {
                ppcVar1 = (*(&iVar5.field_0x18 + iVar6) + 0x4);
                (**ppcVar1)();
            }
        }
    } else if (((0x0 < param_2 - 0x3) && (!SBORROW2(param_2 - 0x3, 1))) && (param_2 - 0x4 < 0x4)) {
        BVar3 = IsIconic16(HWND16_1050_0396);
        if (BVar3 == 0) {
            BVar3 = IsIconic16(&iVar5.field_0x4);
            if ((BVar3 == 0) && (uVar2 = iVar5.field20_0x14, (uVar2 + 0x24) != 0)) {
                InvalidateRect16(0x0, NULL, 0x0);
                uVar4 = pass1_1020_64d4(param_1, 0x2);
                if (uVar4 == 0) {
                    pass1_1020_6746(param_1, 0x1, 0x2);
                }
                uVar4 = pass1_1020_64d4(param_1, 0x3);
                if (uVar4 == 0) {
                    pass1_1020_6746(param_1, 0x1, 0x3);
                }
                uVar4 = pass1_1018_255e(iVar5.field20_0x14);
                if (uVar4 == 0) {
                    SendMessage16(0x0, 0x69, 0x111, &iVar5.field_0x4);
                } else {
                    uVar4 = pass1_1020_64d4(param_1, 1);
                    if (uVar4 == 0) {
                        pass1_1020_6746(param_1, 0x1, 1);
                    }
                }
                SendMessage16(0x0, 0xf0, 0x111, &iVar5.field_0x4);
                uVar2 = iVar5.field41_0x2c;
                if ((uVar2 + 0x7a) != 0) {
                    uVar2 = iVar5.field41_0x2c;
                    (uVar2 + 0x7a) = 0;
                    SendMessage16(0x0, 0x131, 0x111, &iVar5.field_0x4);
                    return;
                }
            }
        }
    }
    return;
}

pub unsafe fn win_ui_op_1038_ea18(param_1: *mut StructB)

{
    let mut hwnd: HWND16;
    let mut IVar1: i16;
    let mut iVar2: *mut StructB;
    let mut uVar2: u16;
    let mut lparam: u32;
    let mut in_stack_0000fff0: bool;
    let mut iStack14: i16;

    dialog_ui_fn_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    lparam = pass1_1010_375e(&iVar2[0x7].field1_0x2);
    hwnd = GetDlgItem16(0xfa5, iVar2.lpvoid_field_0x8);
    SendMessage16(lparam, 0x0, 0xc, hwnd);
    GetWindowRect16(CONCAT22(0x1050, &stack0xfff0), iVar2.max_count_field_0x10);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    move_win_1040_826c(param_1, IVar1 + iStack14 + 0x5, in_stack_0000fff0);
    ShowWindow16(0x5, iVar2.lpvoid_field_0x8);
    return;
}


pub unsafe fn create_window_1040_6eae(mut param_1: u32, mut param_2: i16, pstruct_param_3: *mut astruct_859, mut param_4: u16, mut param_5: u16 )

{
   let mut pstruct_1: *mut astruct_859;
  let mut uVar1: u16;
  let mut window_name: *mut c_char;
  let mut h_instance: HISTANCE16;

  window_name = load_string_1010_847e(_u16_1050_14cc,param_4);
  _h_instance = 0x50000009;
  if (param_2 != 0) {
    _h_instance = 0x50020009;
  }
  uVar1 = (pstruct_param_3 >> 0x10);
  pstruct_1 = pstruct_param_3;
  CreateWindow16(0x0,CONCAT22(param_5,HINSTANCE16_1050_038c),(param_1 + 0x6),
                 pstruct_1.field4_0x6,pstruct_1.field3_0x4,pstruct_1.field2_0x2,pstruct_param_3,
                 _h_instance,(_h_instance >> 0x10),window_name,s_button_1050_5d92);
  return;
}


pub unsafe fn create_window_1040_7620(mut param_1: u32, mut param_2: i16, pstruct_param_3: *mut astruct_860, mut param_4: u16, mut param_5: u16 )

{
  let mut iVar1: *mut astruct_860;
  let mut uVar1: u16;
  let mut window_name: *mut c_char;
  let mut h_instance: HISTANCE16;

  window_name = load_string_1010_847e(_u16_1050_14cc,param_4);
  _h_instance = 0x50000009;
  if (param_2 != 0) {
    _h_instance = 0x50020009;
  }
  uVar1 = (pstruct_param_3 >> 0x10);
  iVar1 = pstruct_param_3;
  CreateWindow16(0x0,CONCAT22(param_5,HINSTANCE16_1050_038c),(param_1 + 0x6),
                 iVar1.field4_0x6,iVar1.field3_0x4,iVar1.field2_0x2,pstruct_param_3,_h_instance,
                 (_h_instance >> 0x10),window_name,s_button_1050_5da8);
  return;
}

pub unsafe fn unk_win_op_1010_7300(
    param_1: *mut Struct57,
    param_2: *mut Struct57,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut ppcVar3: *mut *mut code;
    let mut cVar4: u8;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u16;
    let mut pSVar7: *mut StructD;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut paVar10: *mut Struct57;
    let mut paVar11: *mut Struct57;
    let mut uVar13: *mut Struct57;
    let mut uVar12: u16;
    let mut paVar13: *mut Struct57;
    let mut puVar14: *mut u16;
    let mut uVar15: u32;
    let mut puStack20: *mut u16;
    let mut puStack14: *mut u16;
    let mut puStack10: *mut u32;
    let mut uStack6: u32;

    if (param_4 == 0) {
        return;
    }
    uStack6 = param_3;
    paVar11 = param_2;
    uVar13 = (param_2 >> 0x10);
    if (param_3 == 0) {
        uVar1 = paVar11.field10_0x14;
        pass1_1010_ad64(
            0x0,
            param_1,
            uVar1,
            CONCAT22(param_4, (uVar1 >> 0x10)),
            param_5,
        );
        uStack6 = param_3 & 0xffff | param_1 << 0x10;
    }
    match param_4 {
        0x1 | 0x4 | 0x6 | 0x7 | 0x8 | 0x9 | 0xd | 0xe | 0x14 | 0x18 => {}
        _ => {
            if ((uStack6 | uStack6) == 0) {
                return;
            }
        }
    };
    pass1_1010_1f62(param_2, 0xb);
    if (paVar11.field7_0xe == 0) {
        return;
    }
    paVar5 = paVar11;
    match (param_4 - 1) {
        0x0 => {
            mem_op_1000_179c(0x94, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            if (uVar8 == 0) {
                //
                // LAB_1010_73fe:
                uVar12 = 0x1000;
                paVar5 = null_mut();
                paVar10 = (paVar10 & 0xffff0000);
            } else {
                uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                pass1_1040_44d2(
                    paVar5,
                    paVar10,
                    CONCAT13((param_1 >> 0x8), CONCAT12(param_1, paVar5)),
                    param_5,
                    paVar11.field7_0xe,
                );
            }
        }

        _ => {
            mem_op_1000_179c(0x94, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_b040(
                CONCAT13((param_1 >> 0x8), CONCAT12(param_1, paVar5)),
                uStack6,
                paVar11.field7_0xe,
            );
        }

        0x3 => {
            mem_op_1000_179c(0x9e, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_5626(
                paVar10,
                CONCAT13((param_1 >> 0x8), CONCAT12(param_1, paVar5)),
                param_5,
                paVar11.field7_0xe,
            );
        }

        0x4 => {
            mem_op_1000_179c(0x94, param_1);
            paVar10 = (param_1 & 0xffff0000);
            //    if ((param_1 | paVar5) == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            puVar14 = pass1_1040_8e58(
                paVar5,
                param_1,
                uStack6,
                CONCAT22(paVar11.field7_0xe, uStack6),
            );
            paVar10 = (paVar10 & 0xffff0000 | puVar14 >> 0x10);
            paVar5 = puVar14;
        }

        0x5 | 0x6 => {
            mem_op_1000_179c(0x98, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_48a0(
                paVar10,
                CONCAT22(param_1, paVar5),
                param_4,
                param_5,
                paVar11.field7_0xe,
            );
        }

        0x7 => {
            mem_op_1000_179c(0x9c, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&u16_1050_1038, 0x0);
            pass1_1038_9144(uVar8, CONCAT22(param_1, paVar5), paVar11.field7_0xe);
        }

        0x8 => {
            mem_op_1000_179c(0xb8, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_b7ee(
                CONCAT13((param_1 >> 0x8), CONCAT12(param_1, paVar5)),
                uStack6,
                paVar11.field7_0xe,
            );
        }

        0x9 | 0xa => {
            mem_op_1000_179c(0x94, param_1);
            paVar10 = (param_1 & 0xffff0000);
            //    if ((param_1 | paVar5) == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&u16_1050_1038, 0x0);
            puVar14 = pass1_1038_9a1e(
                paVar5,
                param_1,
                uStack6,
                CONCAT22(paVar11.field7_0xe, uStack6),
            );
            paVar10 = (paVar10 & 0xffff0000 | puVar14 >> 0x10);
            paVar5 = puVar14;
        }

        0xb => {
            mem_op_1000_179c(0x12a, param_1);
            paVar10 = (param_1 & 0xffff0000);
            //    if ((param_1 | paVar5) == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&u16_1050_1038, 0x0);
            uVar15 = pass1_1038_9b72(
                paVar5,
                param_1,
                uStack6,
                CONCAT22(paVar11.field7_0xe, uStack6),
            );
            paVar10 = (paVar10 & 0xffff0000 | uVar15 >> 0x10);
            paVar5 = uVar15;
        }

        0xc => {
            mem_op_1000_179c(0x9c, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_6826(CONCAT22(param_1, paVar5), paVar11.field7_0xe);
        }

        0xd => {
            mem_op_1000_179c(0x9c, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_6fb6(CONCAT22(param_1, paVar5), paVar11.field7_0xe);
        }

        0x12 => {
            mem_op_1000_179c(0x94, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            winapp::make_proc_inst_1040_a234(
                paVar5,
                param_1,
                uStack6,
                CONCAT22(paVar11.field7_0xe, uStack6),
            );
        }

        0x13 => {
            mem_op_1000_179c(0xb8, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_4e94(
                CONCAT13((param_1 >> 0x8), CONCAT12(param_1, paVar5)),
                uStack6,
                paVar11.field7_0xe,
            );
        }

        0x14 => {
            mem_op_1000_179c(0x9a, param_1);
            pSVar7 = (param_1 | paVar5);
            paVar10 = (param_1 & 0xffff0000 | ZEXT24(pSVar7));
            //    if (pSVar7.is_null()) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_0e1c(
                pSVar7,
                CONCAT22(param_1, paVar5),
                0x1,
                uStack6,
                paVar11.field7_0xe,
            );
        }

        0x15 => {
            mem_op_1000_179c(0x9c, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            paVar13 = pas1_1040_29c2(
                paVar5,
                uVar8,
                CONCAT13((param_1 >> 0x8), CONCAT12(param_1, paVar5)),
                uStack6,
                paVar11.field7_0xe,
            );
            paVar10 = (paVar10 & 0xffff0000 | paVar13 >> 0x10);
            paVar5 = paVar13;
        }

        0x16 => {
            mem_op_1000_179c(0x12a, param_1);
            paVar10 = (param_1 & 0xffff0000);
            //    if ((param_1 | paVar5) == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&u16_1050_1038, 0x0);
            puVar14 = pass1_1038_adde(
                paVar5,
                param_1,
                uStack6,
                CONCAT22(paVar11.field7_0xe, uStack6),
            );
            paVar10 = (paVar10 & 0xffff0000 | puVar14 >> 0x10);
            paVar5 = puVar14;
        }

        0x17 => {
            mem_op_1000_179c(0xec, param_1);
            uVar8 = param_1 | paVar5;
            paVar10 = (param_1 & 0xffff0000 | uVar8);
            //    if (uVar8 == 0) goto LAB_1010_73fe;
            uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_a640(
                CONCAT13((param_1 >> 0x8), CONCAT12(param_1, paVar5)),
                param_5,
                paVar11.field7_0xe,
            );
        }
    };
    uVar8 = paVar10;
    puStack10 = CONCAT22(uVar8, paVar5);
    ppcVar3 = (*puStack10 + 0x8);
    paVar13 = paVar10;
    (**ppcVar3)(uVar12, paVar5, uVar8);
    if (param_4 != 0x17) {
        //    if (0x17 < param_4) goto LAB_1010_7710;
        cVar4 = param_4;
        //    if ((cVar4 != '\x05') && (((cVar4 -0x5) < '\x05' || ('\x02' < (cVar4 -0xa))))) goto LAB_1010_7710;
    }
    if ((uStack6 + 0x16) != 0) {
        uVar12 = SUB42(0x1538, 0x0);
        uVar15 = SendMessage16(0x0, 0xf8, 0x111, (uStack6 + 0x14));
        paVar13 = (paVar13 & 0xffff0000 | uVar15 >> 0x10);
    } //
    // LAB_1010_7710:
    if (puStack10.is_null() == false) {
        uVar12 = SUB42(0x1538, 0x0);
        uVar6 = IsWindow16(paVar5.field3_0x6);
        if (uVar6 != 0) {
            if (&paVar11.field_0x1c == 0) {
                mem_op_1000_179c(0xc, paVar13);
                uVar8 = paVar13;
                uVar9 = uVar8 | uVar6;
                paVar13 = (paVar13 & 0xffff0000 | uVar9);
                if (uVar9 == 0) {
                    paVar11.field_0x1c = 0;
                } else {
                    set_struct_1008_574a(CONCAT22(uVar8, uVar6));
                    paVar11.field_0x1c = uVar6;
                    paVar11.field_0x1e = paVar13;
                }
            }
            mem_op_1000_179c(0xc, paVar13);
            uVar8 = paVar13;
            puStack14 = CONCAT22(uVar8, uVar6);
            if ((uVar8 | uVar6) == 0) {
                puStack20 = null_mut();
            } else {
                *puStack14 = 0x389a;
                (uVar6 + 0x2) = 0x1008;
                (uVar6 + 0x4) = param_5;
                (uVar6 + 0x8) = puStack10;
                *puStack14 = 0x7e24;
                (uVar6 + 0x2) = 0x1010;
                puStack20 = puStack14;
            }
            uVar2 = &paVar11.field_0x1c;
            ppcVar3 = (*&paVar11.field_0x1c + 0x4);
            (**ppcVar3)(
                0x1000,
                uVar2,
                (uVar2 >> 0x10),
                puStack20,
                (puStack20 >> 0x10),
            );
            return;
        }
    }
    if ((uVar8 | paVar5) != 0) {
        ppcVar3 = *puStack10;
        (**ppcVar3)(uVar12, paVar5, paVar10, 1);
    }
    return;
}


pub unsafe fn win_proc_1008_5f44(mut param_1: u16, mut param_2: u16, param_3: LPARAM, in_wparam_2: WPARAM16, mut param_5: u16, param_6: HWND16) -> LRESULT

{
  let mut WVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut LVar3: LRESULT;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0x2) {
    WVar1 = GetWindowWord16(0x0,param_6);
    block_1008_5000::mci_send_command_1008_5cb6(_u16_1050_02a0, WVar1);
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(param_2,0x37),in_stack_0000fea0,
                             in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1008_aa28(puVar4,puVar4);
  }
  else {
    if (param_5 != 0x3b9) {
      LVar3 = DefWindowProc16(param_3,in_wparam_2,param_5,param_6);
      return LVar3;
    }
    DestroyWindow16(param_6);
  }
  return 0x0;
}

pub unsafe fn win_ui_op_1038_eaa2(param_1: *mut astruct_888, mut param_2: i16)

{
    let mut hwnd: HWND16;
    let mut struct_1: *mut astruct_888;
    let mut struct_1_lo: u16;
    let mut LVar1: LRESULT;

    struct_1 = param_1;
    struct_1_lo = (param_1 >> 0x10);
    if (param_2 != 0) {
        hwnd = GetDlgItem16(0xfa5, struct_1.field6_0x6);
        LVar1 = SendMessage16(CONCAT22(0x1050, &stack0xffac), 0x50, 0xd, hwnd);
        pass1_1010_3770((LVar1 >> 0x10), struct_1.field140_0x8e,
                        CONCAT22(0x1050, &stack0xffac));
        PostMessage16(0x0, 0xfb, 0x111, struct_1.field7_0x8);
    }
    DestroyWindow16(struct_1.field6_0x6);
    return;
}


pub unsafe fn win_msg_op_1040_13b2(param_1: *mut astruct_892, mut param_2: i16)

{
    let mut HVar1: HWND16;
    let mut uVar4: u16;
    let mut iVar4: i16;
    let mut puVar5: *mut u8;
    let mut iVar5: i16;
    let mut puVar4: *mut c_char;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut paVar2: *mut Struct57;
    let mut struct_7: *mut astruct_892;
    let mut iVar6: i16;
    let mut struct_5_lo: u16;
    let mut uVar6: u16;
    let mut lresult_4: LRESULT;
    let mut pcVar6: *mut c_char;
    let mut puStack266: *mut u32;
    let mut local_100: [u8; 0x52] = [0; 0x52];
    let mut local_aa: [u8; 0x52] = [0; 0x52];
    let mut uStack88: u16;
    let mut handle_86: HWND16;
    let mut local_54: [u8; 0x52] = [0; 0x52];
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (in_EDX >> 0x10);
    struct_7 = param_1;
    struct_5_lo = (param_1 >> 0x10);
    if (param_2 != 0) {
        handle_86 = GetDlgItem16(0xfd2, struct_7.hwnd_0x6);
        SendMessage16(CONCAT22(0x1050, local_54), 0x51, 0xd, handle_86);
        uStack88 = pass1_1000_3e2c(CONCAT22(0x1050, local_54));
        HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x4, struct_7.hwnd_0x6);
        lresult_4 = SendMessage16(0x0, 0x0, 0x407, HVar1);
        if (lresult_4 != 0xffff) {
            SendMessage16(CONCAT22(0x1050, local_aa), lresult_4, 0x408, HVar1);
        }
        HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x5, struct_7.hwnd_0x6);
        lresult_4 = SendMessage16(0x0, 0x0, 0x407, HVar1);
        if (lresult_4 != 0xffff) {
            SendMessage16(CONCAT22(0x1050, local_100), lresult_4, 0x408, HVar1);
        }
        pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x531);
        paVar2 = CONCAT22(uVar5, local_aa);
        uVar4 = pass1_1000_3d7a(CONCAT22(0x1050, local_aa), CONCAT22(0x1050, local_100));
        if (uVar4 != 0) {
            uVar4 = pass1_1000_3d7a(CONCAT22(0x1050, local_aa), pcVar6);
            if (uVar4 != 0) {
                uVar4 = pass1_1000_3d7a(CONCAT22(0x1050, local_100), pcVar6);
                if (uVar4 != 0) {
                    pass1_1010_531c(local_aa, paVar2, struct_7.field141_0x8e, CONCAT22(0x1050, local_aa));
                    puVar5 = local_100;
                    pass1_1010_52fc(puVar5, paVar2, struct_7.field141_0x8e, CONCAT22(0x1050, puVar5));
                    pass1_1010_5120(puVar5, paVar2, struct_7.field141_0x8e, uStack88);
                    if (puVar5.is_null()) {
                        mem_op_1000_179c(0xb4, paVar2);
                        puVar7 = (paVar2 | puVar5);
                        uVar3 = paVar2 & 0xffff0000 | ZEXT24(puVar7);
                        if (puVar7.is_null()) {
                            iVar5 = 0;
                            uVar5 = 0;
                        } else {
                            iVar5 = string_1040_8520(uVar3, CONCAT22(paVar2, puVar5), HWND16_1050_0396, 0x20030,
                                                     0x7d2057b);
                            uVar5 = uVar3;
                        }
                        fn_ptr_1 = (CONCAT13((uVar5 >> 0x8), CONCAT12(uVar5, iVar5)) + 0x74);
                        (**fn_ptr_1)();
                        return;
                    }
                    uVar1 = struct_7.field141_0x8e;
                    uVar2 = struct_7.field141_0x8e;
                    uVar6 = (uVar2 >> 0x10);
                    iVar6 = uVar2;
                    uVar3 = struct_7.field141_0x8e;
                    pass1_1028_8d9e(CONCAT22(0x1050, &stack0xfdd2), (uVar3 + 0xa),
                                    (uVar1 + 0x12),
                                    (iVar6 + 0x16) & 0xffff | (iVar6 + 0x18) << 0x10);
                    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &stack0xfdd2));
                    pass1_1028_8dec(CONCAT22(0x1050, &stack0xfdd2));
                    // TODO: goto LAB_1040_1619;
                }
            }
        }
        mem_op_1000_179c(0xb4, paVar2);
        puVar6 = (paVar2 | uVar4);
        uVar3 = paVar2 & 0xffff0000 | ZEXT24(puVar6);
        if (puVar6.is_null()) {
            iVar4 = 0;
            uVar5 = 0;
        } else {
            iVar4 = string_1040_8520(uVar3, CONCAT22(paVar2, uVar4), HWND16_1050_0396, 0x20030, 0x755057b);
            uVar5 = uVar3;
        }
        puStack266 = CONCAT22(uVar5, iVar4);
        fn_ptr_1 = (*puStack266 + 0x74);
        (**fn_ptr_1)();
    }//
// LAB_1040_1619:
    DestroyWindow16(struct_7.hwnd_0x6);
    return;
}

pub unsafe fn destroy_window_1020_1d4a(mut param_1: u32, mut param_2: i16) {
    let mut in_AX: u16;
    let mut BVar1: bool;
    let mut in_EDX: u32;
    let mut uVar2: u16;

    if (param_2 != 0) {
        BVar1 = s::post_win_msg_1020_1ca4(param_1, in_EDX, in_AX);
        if (BVar1 != 0) {
            uVar2 = (param_1 >> 0x10);
            if ((param_1 + 0x96) != 0) {
                PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
            }
            DestroyWindow16((param_1 + 0x6));
        }
    }
    return;
}
