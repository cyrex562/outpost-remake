use crate::block_1008::block_1008_5000;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1020::block_1020_6000::{pass1_1020_64d4, pass1_1020_6746};
use crate::win_ui::invalidate_rect_1020_735a;
use crate::block_1040::block_1040_4000::{pass1_1040_4d7e, pass1_1040_4dcc};
use crate::win_ui::{dialog_ui_fn_1040_78e2, move_win_1040_826c, msg_box_ui_op_1040_64ca, set_win_pos_1040_331a};
use crate::globals::DAT_1050_1050;
use crate::no_refs::s;
use crate::structs::struct_57::Struct57;
use crate::utils::CONCAT22;
use crate::winapi16::{DestroyWindow16, GetDlgItem16, GetWindowRect16, SetWindowPos16, ShowWindow16};
use crate::windef16::{HWND16, LPARAM, LRESULT, WPARAM16};

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
        post_win_msg_1040_7b3c
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
          post_win_msg_1038_dcb0(0x0,paVar5,CONCAT22(param_3,param_2));
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
      (**ppcVar1)(0x1008,iVar9,uVar3,&stack0xffea,&DAT_1050_1050);
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
    GetClientRect16(&local_1e, &DAT_1050_1050);
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
      var5 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_3,handle_1,win_handle_1,param_3);
    }
    else {
      if (l_param == 0x86) {
        fn_ptr_1 = (*var6 + 0x20);
        var1 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_3,handle_1,param_3);
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


pub unsafe fn create_window_1040_92dc(param_1: *mut astruct_65)

{
  let mut hwnd: HWND16;
   let mut pstruct65_2: *mut astruct_65;
   let mut pstruct65_1: *mut astruct_65;
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
            make_proc_inst_1040_a234(
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
        uVar12 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        uVar15 = SendMessage16(0x0, 0xf8, 0x111, (uStack6 + 0x14));
        paVar13 = (paVar13 & 0xffff0000 | uVar15 >> 0x10);
    } //
    // LAB_1010_7710:
    if (puStack10.is_null() == false) {
        uVar12 = SUB42(s_tile2_bmp_1050_1538, 0x0);
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

pub unsafe fn unk_destroy_win_op_1020_694c(
    mut param_1: u16,
    param_2: *mut StructA,
    mut param_3: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut HVar4: HWND16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut uVar6: u32;
    let iVar5: *mut StructA;
    let uVar4: *mut StructA;
    let mut unaff_CS: u16;
    let mut wparam: WPARAM16;

    uVar6 = CONCAT22(in_register_0000000a, param_1);
    uVar1 = param_3;
    if (param_3 != 0x12b) {
        iVar5 = param_2;
        uVar4 = (param_2 >> 0x10);
        if (param_3 < 0x12c) {
            if (param_3 == 0x6f) {
                uVar2 = FUN_1010_830a(0x0, uVar6, unaff_CS, _u16_1050_14cc, 0x1f8);
                BVar3 = WinHelp16(0x29, 0x1, CONCAT22(uVar6, uVar2), iVar5.field4_0x8);
                return BVar3;
            }
            if (param_3 == 0xeb) {
                uVar1 = GetDlgItem16(0x1797, iVar5.field4_0x8);
                uVar5 = uVar6;
                if (uVar1 != 0) {
                    //
                    // LAB_1020_6a6f:
                    win_ui_fn_1020_6e98(uVar5, param_2);
                    return uVar1;
                }
            } else {
                uVar1 = param_3 - 0xef;
                if (uVar1 == 0) {
                    pass1_1018_2e28(&iVar5[0x1].field20_0x26);
                    pass1_1008_3e0e(param_2);
                } else {
                    uVar1 = param_3 - 0x129;
                    if ((uVar1 != 0) && (uVar1 = param_3 - 0x12a, uVar1 == 0)) {
                        HVar4 = iVar5.field4_0x8;
                        wparam = 0xf012; //
                        // LAB_1020_69c3:
                        BVar3 = PostMessage16(0x0, wparam, 0x112, HVar4);
                        return BVar3;
                    }
                }
            }
        } else if (param_3 == 0xbb8) {
            HVar4 = GetDlgItem16(0x1797, iVar5.field4_0x8);
            if (HVar4 != 0) {
                DestroyWindow16(HVar4);
            }
            uVar1 = pass1_1018_31d0(&iVar5[0x1].field20_0x26);
            if (uVar1 != 0) {
                uVar1 = pass1_1018_2d9a(&iVar5[0x1].field20_0x26); //
                // LAB_1020_6a0b:
                invalidate_rect_1020_735a(&iVar5[0x1].field22_0x2a);
                return uVar1;
            }
        } else if (param_3 < 0xbb9) {
            if (param_3 == 0x12c) {
                HVar4 = iVar5.field4_0x8;
                wparam = 0xf020;
                // TODO: goto LAB_1020_69c3;
            }
            uVar1 = param_3 - 0x12d;
            if (param_3 != 0x12c) {
                uVar1 = param_3 - 0x12e;
            }
        } else if (param_3 == 0xbb9) {
            HVar4 = GetDlgItem16(0x1797, iVar5.field4_0x8);
            if (HVar4 != 0) {
                DestroyWindow16(HVar4);
            }
            uVar1 = pass1_1018_31d0(&iVar5[0x1].field20_0x26);
            if (uVar1 != 0) {
                uVar1 = pass1_1018_2dde(&iVar5[0x1].field20_0x26);
                // TODO: goto LAB_1020_6a0b;
            }
        } else {
            uVar1 = param_3 - 0xbba;
            if (uVar1 == 0) {
                uVar1 = GetDlgItem16(0x1797, iVar5.field4_0x8);
                uVar5 = uVar6;
                if (uVar1 != 0) {
                    BVar3 = DestroyWindow16(uVar1);
                    return BVar3;
                }
                // TODO: goto LAB_1020_6a6f;
            }
        }
    }
    return uVar1;
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

pub unsafe fn win_ui_reg_class_1008_96d2(param_1: *mut StructA) {
    let mut BVar1: bool;
    let mut AVar2: ATOM;
    let mut wndclass: WNDCLASS16;

    wndclass.lpsz_class_name = param_1 + 0x5b;
    BVar1 = GetClassInfo16(
        &wndclass,
        CONCAT22(wndclass.lpsz_class_name, 0x1050),
        param_1,
    );
    if (BVar1 == 0) {
        wndclass.style = (param_1 + 0xc8);
        wndclass.lpfn_wnd_proc = 0x5632;
        wndclass.lpfn_wnd_proc = 0x1008;
        wndclass._6_4_ = 0x40000;
        wndclass.h_instance = HINSTANCE16_1050_038c;
        wndclass.h_icon = (param_1 + 0xc2);
        wndclass.h_cursor = (param_1 + 0xc4);
        wndclass.hbr_background = (param_1 + 0xc6);
        wndclass.lpsz_menu_name = 0;
        wndclass.lpsz_class_name = param_1;
        AVar2 = RegisterClass16(&wndclass);
        if (AVar2 == 0) {
            fn_ptr_op_1000_24cd(0x0);
        }
    }
    return;
}


pub unsafe fn send_win_msg_1040_4a0a(struct_param_1: *mut astruct_48) -> LRESULT

{
    WPARAM16 * pWVar1;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut dlg_item: HWND16;
    let mut uVar5: u16;
    let mut iVar5: *mut astruct_48;
    let mut uVar6: *mut astruct_48;
    let mut lresult_6: LRESULT;
    let mut pcVar6: *mut c_char;
    let mut wparam: WPARAM16;
    let mut UVar7: u16;
    let mut HVar8: HWND16;
    let mut WStack10: WPARAM16;

    uVar6 = (struct_param_1 >> 0x10);
    iVar5 = struct_param_1;
    ppcVar2 = (struct_param_1 + 0x74);
    (**ppcVar2)();
    dlg_item = GetDlgItem16(0x1770, iVar5.hwnd_0x6);
    SendMessage16(0x0, 0x0, 0x40b, dlg_item);
    lresult_6 = SendMessage16(0x0, 0x0, 0xb, dlg_item);
    uVar5 = (lresult_6 >> 0x10);
//   for (WStack10 = 0; uVar3 = iVar5.field143_0x90, pWVar1 = (uVar3 + 0x10),
//       *pWVar1 != WStack10 && WStack10 <= *pWVar1; WStack10 += 1)
    WStack10 = 0;
    uVar3 = iVar5.field143_0x90;
    pWVar1 = uVar3 + 0x10;
    while *pWVar1 != WStack10 && WStack10 <= *pWVar1

    {
        wparam = 0;
        UVar7 = 0x403;
        uVar3 = iVar5.field143_0x90;
        uVar3 = (uVar3 + 0xc);
        HVar8 = dlg_item;
        pcVar6 = pass1_1040_4dcc(uVar5, struct_param_1, (uVar3 + WStack10 * 0x2));
        lresult_6 = SendMessage16(pcVar6, wparam, UVar7, HVar8);
        uVar5 = (lresult_6 >> 0x10);
    }
    pass1_1040_4d7e(struct_param_1);
    if (WStack10 == 0) {
        UVar7 = 0x40a;
        uVar4 = iVar5.field143_0x90;
        uVar3 = iVar5.field144_0x94;
        HVar8 = dlg_item;
        pcVar6 = string_op_1010_ada6(uVar5, uVar3, (uVar3 >> 0x10), 0x0, (uVar4 + 0xa));
        SendMessage16(pcVar6, WStack10, UVar7, HVar8);
    }
    lresult_6 = SendMessage16(0x0, 0x1, 0xb, dlg_item);
    return lresult_6;
}


pub unsafe fn pass1_1040_4cf4(mut param_1: u32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut hwnd: HWND16;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut LVar9: LRESULT;
    let mut uVar10: u32;
    let mut local_52: [u8; 0x50] = [0; 0x50];

    uVar8 = (in_EDX >> 0x10);
    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    hwnd = GetDlgItem16(0x1770, (iVar5 + 0x6));
    LVar9 = SendMessage16(0x0, 0x0, 0x407, hwnd);
    uVar4 = CONCAT22(uVar8, (LVar9 >> 0x10));
    if (LVar9 != 0xffff) {
        uVar10 = SendMessage16(CONCAT22(0x1050, local_52), LVar9, 0x408, hwnd);
        uVar4 = uVar4 & 0xffff0000 | uVar10 >> 0x10;
    }
    uVar2 = (iVar5 + 0x90);
    uVar1 = (iVar5 + 0x94);
    uVar3 = pass1_1010_ae12(uVar4, uVar1, (uVar1 >> 0x10), CONCAT22(0x1050, local_52),
                            (uVar2 + 0xa));
    if (uVar3 != 0xffff) {
        uVar1 = (iVar5 + 0x90);
        uVar8 = (uVar1 >> 0x10);
        iVar6 = uVar1;
        uVar10 = (iVar6 + 0x6);
        pass1_1010_ae92((iVar5 + 0x94), uVar10, (iVar6 + 0xa), uVar10, uVar4);
    }
    return;
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


pub unsafe fn send_msg_1038_ed8a(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u32;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut hwnd: HWND16;
    let mut in_stack_0000ffe2: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    hwnd = HWND16_1050_0396;
    if (param_4 != 0x1c8) {
        if (param_4 == 0x1c9) {
            puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe2, 0x2f), in_stack_0000fe8a,
                                     in_stack_0000ffae, in_stack_0000ffb4, in_stack_0000ffb8);
            uVar2 = (puVar7 >> 0x10);
            uVar5 = (puVar7 + 0x20);
            uVar1 = (puVar7 + 0x22);
            uVar8 = paVar6 & 0xffff0000 | uVar1;
            uVar3 = uVar1 | uVar5;
            if (uVar3 == 0) {
                return;
            }
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(uVar1, uVar5));
            uVar5 = uVar8 | uVar3;
            paVar6 = (uVar8 & 0xffff0000 | uVar5);
            if (uVar5 == 0) {
                return;
            }
            iVar4 = pass1_1030_5b00(CONCAT22(uVar8, uVar3));
            puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe2, iVar4), in_stack_0000fe8a,
                                     in_stack_0000ffae, in_stack_0000ffb4, in_stack_0000ffb8);
            if (((puVar7 >> 0x10) | puVar7) == 0) {
                return;
            }
            uVar8 = pass1_1018_0ad4(puVar7);
            uVar5 = (uVar8 >> 0x10);
            if ((uVar5 | uVar8) == 0) {
                return;
            }
            param_4 = 0x72;
            hwnd = (uVar8 + 0x8);
        } else if (param_4 != 0x1ca) {
            post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), (param_3 >> 0x10), param_4, param_4);
            return;
        }
    }
    SendMessage16(0x0, param_4, 0x111, hwnd);
    return;
}


pub unsafe fn def_win_proc_1008_5632(param_1: LPARAM, param_2: WPARAM, mut param_3: u16, param_4: HWND16)

{
  let mut ppcVar1: *mut *mut code;
  let mut puStack6: *mut u32;
  let mut uVar2: u16;

  uVar2 = &DAT_1050_1050;
  puStack6 = GetWindowLong16(0x0,param_4);
  if (((puStack6 >> 0x10) | puStack6) == 0) {
    if (param_3 != 1) {
      DefWindowProc16(param_1,param_2,param_3,param_4);
      return;
    }
    puStack6 = param_1;
    SetWindowLong16(puStack6,0x0,param_4);
    pass1_1008_9628(puStack6,param_4);
  }
  ppcVar1 = (*puStack6 + 0x1c);
  (**ppcVar1)(s_tile2_bmp_1050_1538,puStack6,(puStack6 >> 0x10),param_1,param_2,param_3,uVar2);
  return;
}


pub unsafe fn unk_win_msg_op_1008_9510(param_1: *mut i16) {
    let mut has_message: bool;
    let mut IVar1: i16;
    let mut local_14: MSG16; //
                             //
                             // LAB_1008_9578:
    if (*param_1 != 0) {
        has_message = GetMessage16(0x0, 0x0, 0x0, &local_14);
        if (has_message != 0) {
            //   if ((_u16_1050_5bc8 + 0x8) != 0) goto code_r0x10089538;
            // TODO: goto LAB_1008_9547;
        }
    }
    return;
    // code_r0x10089538:
    has_message = IsDialogMessage16(&local_14, &DAT_1050_1050);
    if (has_message == 0) {
        //
        // LAB_1008_9547:
        if (PTR_LOOP_1050_0398.is_null() == false) {
            IVar1 =
                TranslateAccelerator16(&local_14, (HACCEL16) & DAT_1050_1050, PTR_LOOP_1050_0398);
            //   if (IVar1 != 0) goto LAB_1008_9578;
        }
        TranslateMessage16(&local_14);
        DispatchMessage16(&local_14);
    }
    //   goto LAB_1008_9578;
}


pub unsafe fn post_win_msg_1008_a0e4(
    param_1: *mut astruct_67,
    param_2: i32,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: i16,
) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut bVar4: bool;
    let mut puVar4: *mut c_char;
    let mut uVar5: *mut astruct_66;
    let mut extraout_DX: u16;
    let mut uVar7: u16;
    let mut iVar7: *mut astruct_67;
    let mut uVar6: *mut astruct_67;
    let mut paStack14: *mut astruct_99;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar6 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), iVar7.field10_0xa);
    bVar4 = false;
    loop {
        puVar4 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, puVar4));
        // if ((extraout_DX | puVar4) == 0) goto LAB_1008_a146;
        if (puVar4 + 0x4) == param_6 {
            break;
        }
    }
    (puVar4 + 0xc) = (puVar4 + 0xc) + param_3;
    (puVar4 + 0xe) = (puVar4 + 0xe) + param_2;
    bVar4 = true; //
                  // LAB_1008_a146:
    if (!bVar4) {
        paStack14 = pass1_1000_07fc(_PTR_LOOP_1050_03a0);
        uVar7 = (paStack14 >> 0x10);
        uVar3 = paStack14;
        if ((uVar7 | uVar3) == 0) {
            paStack14 = null_mut();
        } else {
            paStack14.field0_0x0 = 0x389a;
            (uVar3 + 0x2) = 0x1008;
            (uVar3 + 0x4) = param_6;
            (uVar3 + 0x6) = param_5;
            (uVar3 + 0xa) = param_4;
            (uVar3 + 0xc) = param_3;
            (uVar3 + 0xe) = param_2;
            paStack14.field0_0x0 = 0xad8e;
            (uVar3 + 0x2) = 0x1008;
        }
        puVar1 = iVar7.field10_0xa;
        ppcVar2 = (*iVar7.field10_0xa + 0x8);
        (**ppcVar2)(
            0x1000,
            puVar1,
            (puVar1 >> 0x10),
            paStack14,
            (paStack14 >> 0x10),
        );
    }
    if (param_6 == 0x14) {
        PostMessage16(0x0, 0xfc, 0x111, HWND16_1050_0396);
    }
    return;
}

pub unsafe fn post_win_msg_1020_291a(mut param_1: u32) {
    PostMessage16(0x0, 0x0, 0x10, (param_1 + 0x8));
    return;
}


pub unsafe fn post_msg_1020_55b0(
    param_1: *mut StructD,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: i16,
) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut pcVar8: *mut c_char;
    let mut puVar6: *mut u32;
    let mut LVar9: LRESULT;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fd80: u16;
    let mut in_stack_0000fd82: u16;
    let mut in_stack_0000fea4: u16;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000feaa: u16;
    let mut in_stack_0000feac: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb0: u16;
    let mut paStack288: *mut Struct57;
    let mut puStack284: *mut u32;
    let mut local_114: WPARAM16;
    let mut local_112: [u8; 0x2] = [0; 0x2];
    let mut iStack272: i16;
    let mut local_10e: i16;
    let mut local_10c: [u8; 0x100] = [0; 0x100];
    let mut puStack12: *mut u32;
    let mut iStack8: i16;
    let mut puStack6: *mut u32;

    puStack6 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2),
        in_stack_0000fd80,
        in_stack_0000fea4,
        in_stack_0000feaa,
        in_stack_0000feae,
    );
    paVar4 = (param_1 & 0xffff0000 | puStack6 >> 0x10);
    iStack8 = (puStack6 + 0x20);
    puStack12 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x37),
        in_stack_0000fd80,
        in_stack_0000fea4,
        in_stack_0000feaa,
        in_stack_0000feae,
    );
    uVar6 = (paVar4 >> 0x10);
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x100,
        local_10c,
        &DAT_1050_1050,
    );
    puVar7 = pass1_1008_9436(CONCAT22(0x1050, local_112));
    pcVar8 = pass1_1008_a8f4(
        (puVar7 >> 0x10),
        puStack12,
        CONCAT22(0x1050, &local_114),
        CONCAT22(0x1050, local_112),
        CONCAT22(0x1050, &local_10e),
    );
    uVar2 = pcVar8;
    paVar4 = CONCAT22(uVar6, (pcVar8 >> 0x10) | uVar2);
    uVar5 = (param_2 >> 0x10);
    if ((pcVar8.is_null() == false) && (*pcVar8 != '\0')) {
        uVar6 = 0x1000;
        mem_op_1000_179c(0xb4, paVar4);
        paStack288 = CONCAT22(paVar4, uVar2);
        uVar2 = paVar4 | uVar2;
        paVar4 = (paVar4 & 0xffff0000);
        if (uVar2 == 0) {
            puVar6 = 0;
        } else {
            uVar6 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            puVar6 = pass1_1040_8478(
                uVar2,
                paStack288,
                0x0,
                CONCAT13(0x10, CONCAT12(0x50, local_10c)),
                pcVar8,
                (param_2 + 0x8),
            );
            paVar4 = (paVar4 & 0xffff0000 | puVar6 >> 0x10);
            puVar6 = SUB42(puVar6, 0x0);
        }
        uVar3 = SUB42(paVar4, 0x0);
        puStack284 = CONCAT22(uVar3, puVar6);
        if (iStack272 == 0) {
            ppcVar1 = (*puStack284 + 0x74);
            (**ppcVar1)(uVar6, puVar6, uVar3);
        } else {
            ppcVar1 = (*puStack284 + 0x6c);
            (**ppcVar1)(uVar6, puVar6, uVar3, local_112, &DAT_1050_1050);
        }
        if ((iStack8 == 0) || (local_114 == 0)) {
            PostMessage16(0x0, 0xfc, 0x111, HWND16_1050_0396);
        }
    }
    if ((iStack8 != 0) && (local_114 != 0)) {
        LVar9 = SendMessage16(0x0, local_114, 0x111, HWND16_1050_0396);
        (param_2 + 0x112) = 0x1;
        return (LVar9 >> 0x10);
    }
    if (local_10e == 0x6) {
        PostMessage16(0x0, 0xb0, 0x111, HWND16_1050_0396);
        puVar10 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(param_4, 0x2),
            in_stack_0000fd82,
            in_stack_0000fea6,
            in_stack_0000feac,
            in_stack_0000feb0,
        );
        paVar4 = (paVar4 & 0xffff0000 | puVar10 >> 0x10);
        param_4 = puVar10;
        (param_4 + 0x20) = 0x1;
    }
    if (local_10e == 0x15) {
        PostMessage16(0x0, 0x97, 0x111, HWND16_1050_0396);
        puVar10 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(param_4, 0x2),
            in_stack_0000fd82,
            in_stack_0000fea6,
            in_stack_0000feac,
            in_stack_0000feb0,
        );
        paVar4 = (puVar10 >> 0x10);
        (puVar10 + 0x20) = 0x1;
    }
    return paVar4;
}

pub unsafe fn post_win_msg_1020_79fc(
    param_1: *mut astruct_69,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut iVar3: i16;
    let mut iVar4: *mut astruct_69;
    let mut uVar4: u16;
    let mut uVar5: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar5 = (iVar4.field223_0xe0 >> 0x10);
    ppcVar2 = (*iVar4.field223_0xe0 + 0x24);
    iVar3 = (**ppcVar2)();
    if (iVar3 != param_4) {
        PostMessage16(0x0, 0x0, 0x85, iVar4.field8_0x8);
        puVar1 = iVar4.field223_0xe0;
        ppcVar2 = (*iVar4.field223_0xe0 + 0x28);
        (**ppcVar2)(
            s_tile2_bmp_1050_1538,
            puVar1,
            (puVar1 >> 0x10),
            param_4,
            uVar5,
        );
    }
    return;
}


pub unsafe fn post_msg_1028_76da() {
    let mut lVar1: i32;
    let mut uVar2: u16;
    let mut in_EDX: *mut Struct57;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffe4: u32;
    let mut uStack10: u16;
    let mut uStack8: u16;

    puVar3 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x2c),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    uVar2 = (puVar3 >> 0x10);
    lVar1 = (puVar3 + 0xc);
    uStack8 = (lVar1 >> 0x10);
    uStack10 = lVar1;
    if (((uStack8 | uStack10) != 0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
        PostMessage16(0x0, 0x106, 0x111, HWND16_1050_0396);
        (puVar3 + 0xc) = 0;
    }
    return;
}


pub unsafe fn post_win_msg_1038_dcb0(mut param_1: u16, param_2: *mut Struct57, mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut paVar7: *mut Struct57;
  let mut uVar8: u32;
  let mut puVar9: *mut u16;
  let mut paVar10: *mut astruct_67;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u8;
  let mut uVar15: u8;
  let mut local_18: u16;
  let mut uStack22: u16;
  let mut local_14: [u8;0x4] = [0;0x4];
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut local_a: [u8;0x4] = [0;0x4];
  let mut puStack6: *mut u32;

  mem_op_1000_179c(0xb4,param_2);
  uStack14 = param_2;
  uVar8 = param_2 & 0xffff0000;
  uVar6 = uVar8 | (uStack14 | param_1);
  iVar4 = param_3;
  uVar5 = (param_3 >> 0x10);
  uStack16 = param_1;
  if ((uStack14 | param_1) == 0) {
    iVar2 = 0;
  }
  else {
    iVar2 = string_1040_8520(uVar6,CONCAT22(uStack14,param_1),(iVar4 + 0x6),0x30004,0x7260634);
    uVar8 = uVar6;
  }
  puStack6 = CONCAT22(uVar8,iVar2);
  puVar9 = pass1_1008_941a(CONCAT22(0x1050,local_a),0x1,0x49);
  paVar7 = (uVar8 & 0xffff0000 | puVar9 >> 0x10);
  ppcVar1 = (*puStack6 + 0x6c);
  uVar3 = (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_a,&DAT_1050_1050);
  uStack12 = uVar3;
  if (uVar3 == 0x6) {
    mem_op_1000_179c(0xb4,paVar7);
    uStack14 = paVar7;
    uVar8 = paVar7 & 0xffff0000;
    uVar6 = uVar8 | (uStack14 | uVar3);
    uStack16 = uVar3;
    if ((uStack14 | uVar3) == 0) {
      iVar4 = 0;
    }
    else {
      iVar4 = string_1040_8520(uVar6,CONCAT13((paVar7 >> 0x8),CONCAT12(paVar7,uVar3)),
                               (iVar4 + 0x6),0x20000,0x7280634);
      uVar8 = uVar6;
    }
    puStack6 = CONCAT22(uVar8,iVar4);
    puVar9 = pass1_1008_941a(CONCAT22(0x1050,local_14),0x1,0x4a);
    paVar7 = (uVar8 & 0xffff0000 | puVar9 >> 0x10);
    ppcVar1 = (*puStack6 + 0x6c);
    (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_14,&DAT_1050_1050);
    uVar14 = 0;
    uVar15 = 0;
    iVar2 = 0x15;
    uVar12 = 0x1;
    uVar13 = 0;
    uVar11 = 0;
    iVar4 = 0;
    uVar5 = 0;
    paVar10 =
              mixed_1010_20ba(paVar7,_u16_1050_0ed0,0x37,in_stack_0000fe72,in_stack_0000ff96,
                              in_stack_0000ff9c,in_stack_0000ffa0);
    uStack22 = (paVar10 >> 0x10);
    local_18 = SUB42(paVar10,0x0);
    post_win_msg_1008_a0e4(paVar10,CONCAT22(uVar11,uVar5),iVar4,uVar12,CONCAT13(uVar15,CONCAT12(uVar14,uVar13)),iVar2);
    PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    return;
  }
  mem_op_1000_179c(0xb4,paVar7);
  uStack14 = paVar7;
  uVar8 = paVar7 & 0xffff0000 | (uStack14 | uVar3);
  uStack16 = uVar3;
  if ((uStack14 | uVar3) == 0) {
    iVar4 = 0;
    uVar5 = 0;
  }
  else {
    iVar4 = string_1040_8520(uVar8,CONCAT13((paVar7 >> 0x8),CONCAT12(paVar7,uVar3)),
                             (iVar4 + 0x6),0x20000,0x7290634);
    uVar5 = uVar8;
  }
  puStack6 = CONCAT22(uVar5,iVar4);
  pass1_1008_941a(CONCAT22(0x1050,&local_18),0x1,0x4b);
  ppcVar1 = (*puStack6 + 0x6c);
  (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),&local_18,&DAT_1050_1050);
  return;
}


pub unsafe fn post_win_msg_1040_7b3c(param_1: *mut StructC, mut param_2: u16, mut param_3: u16, mut param_4: i16) -> BOOL16

{
  let mut ppcVar1: *mut *mut code;

  if ((param_4 == 1) || (param_4 == 0x2)) {
    ppcVar1 = (param_1 + 0x14);
    (**ppcVar1)();
  }
  else if (param_4 == 0x6f) {
    ppcVar1 = (param_1 + 0x2c);
    (**ppcVar1)();
  }
  else {
    if (param_4 != 0x12e) {
      return 0x0;
    }
    PostMessage16(0x0,0xf060,0x112,(param_1 + 0x6));
  }
  return 0x1;
}

pub unsafe fn destroy_win_1038_a3d2(mut param_1: u32)

{
  let mut hwnd: HWND16;
  hwnd = GetWindowWord16(-0x8,(param_1 + 0x6));
  PostMessage16(0x0,0x105,0x111,hwnd);
  destroy_win_1040_7b98(param_1);
  return;
}

pub unsafe fn post_win_msg_1038_d840(param_1: *mut astruct_70, mut param_2: u16)

{
    let mut iVar1: *mut astruct_70;
    let mut uVar1: *mut astruct_70;

    iVar1 = param_1;
    uVar1 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if (iVar1.field142_0x8e != 0) {
            PostMessage16(0x0, iVar1.field142_0x8e, 0x111, &iVar1.field_0x6);
            iVar1.field142_0x8e = 0;
            return;
        }
    } else if (param_2 < 0x11) {
        if (param_2 == '\x01') {
            iVar1.field143_0x90 = 0;
            iVar1.field144_0x92 = 0;
            return;
        }
        if (param_2 == '\x03') {
            pass1_1038_e03e(param_1);
            return;
        }
    }
    return;
}


pub unsafe fn post_win_msg_1040_0d5e(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    if (param_3 != 0) {
        PostMessage16(0x0, 0x1, 0x111, (param_1 + 0x8));
    }
    return;
}


pub unsafe fn post_win_msg_1040_672e(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut iVar1: i16;

    if (param_5 == s_vrpal_bmp_1050_183a + 0x7) {
        msg_box_ui_op_1040_64ca(0x0, param_1, CONCAT22(param_3, param_2));
    } else {
        if (param_5 == 0x1851) {
            iVar1 = 0x2a;
        } else {
            if (param_5 != 0x1852) {
                post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
                return;
            }
            iVar1 = 0x29;
        }
        pass1_1038_af40(param_2, param_1, _PTR_LOOP_1050_5b7c, (param_2 + 0x8), iVar1);
        PostMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
    }
    return;
}


pub unsafe fn post_win_msg_1040_7f1c(mut param_1: u32, mut param_2: i16) -> BOOL16

{
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x78) != 0) {
        return 0x0;
    }
    if ((iVar1 + 0x60) != param_2) {
        (iVar1 + 0x60) = param_2;
        PostMessage16(0x0, 0x0, 0x85, (iVar1 + 0x6));
    }
    return 0x1;
}


pub unsafe fn post_win_msg_1040_7f56(mut param_1: u32, param_2: *mut c_char)

{
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x10)), param_2);
    PostMessage16(0x0, 0x0, 0x85, (param_1 + 0x6));
    return;
}


pub unsafe fn post_msg_1008_3d20(mut param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    PostMessage16(0x0, (param_1 + 0xcc), 0x111, (param_1 + 0xbc));
    return;
}

pub unsafe fn invalidate_rect_1018_58e2(param_1: *mut astruct_58, mut param_2: i16) {
    let mut piVar1: *mut i16;
    let mut iVar2: *mut astruct_58;
    let mut uVar2: u16;

    if (param_2 == 0x105) {
        uVar2 = (param_1 >> 0x10);
        iVar2 = param_1;
        piVar1 = &iVar2.field245_0xf6;
        *piVar1 = *piVar1 + 1;
        if (u16_1050_4240 <= iVar2.field245_0xf6) {
            PostMessage16(0x0, 0xca, 0x111, HWND16_1050_0396);
            return;
        }
        iVar2.field234_0xea = 0;
        InvalidateRect16(0x0, NULL, 0x0);
    }
    return;
}

pub unsafe fn post_win_msg_1018_ea0a(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    if (param_3 == 0xed) {
        PostMessage16(0x0, 0xc6, 0x111, HWND16_1050_0396);
    }
    return;
}

pub unsafe fn post_msg_1020_03d6(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn post_msg_1020_03fa(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn post_win_msg_1020_061c(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x6) = 0;
        return;
    }
    if (param_2 != 0x2) {
        return;
    }
    uVar1 = (param_1 + 0x6);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn win_help_op_1020_0ec4(mut param_1: u16, param_2: *mut u32, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut cVar2: u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let mut paVar8: *mut astruct_477;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut wparam: WPARAM16;
    let mut hwnd: HWND16;
    let mut iVar11: i16;
    let mut in_stack_0000fff4: u16;
    let mut uVar12: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    uVar7 = (param_2 >> 0x10);
    uVar3 = param_2;
    if (param_3 == 0xfb) {
        puVar9 = mixed_1010_20ba(
            paVar6,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fff4, 0x30),
            in_stack_0000fe9c,
            in_stack_0000ffc0,
            in_stack_0000ffc6,
            in_stack_0000ffca,
        );
        uVar12 = puVar9;
        pass1_1010_375e(puVar9);
        ppcVar1 = (*param_2 + 0x14);
        (**ppcVar1)();
        uVar10 = pass1_1010_375e(puVar9 & 0xffff0000 | uVar12);
        uVar12 = (uVar10 >> 0x10);
        pass1_1010_4788(
            uVar10,
            uVar12,
            (uVar3 + 0xf2),
            (uVar10 & 0xffff | uVar12 << 0x10),
        );
        return;
    }
    if (0xfb < param_3) {
        match param_3 {
            _ => {
                return;
            }
            0x12a => {
                hwnd = (uVar3 + 0x8);
                wparam = 0xf012;
            }
            //   break;
            0x12c => {
                hwnd = (uVar3 + 0x8);
                wparam = 0xf020;
            }
        }
        PostMessage16(0x0, wparam, 0x112, hwnd);
        return;
    }
    if (param_3 == 0xf3) {
        iVar11 = 0x3;
    } else {
        if (0xf3 < param_3) {
            return;
        }
        cVar2 = param_3;
        if ((cVar2 + 0x91) == 0) {
            uVar4 = FUN_1010_830a(
                param_3 & 0xff00 | (cVar2 + 0x91),
                paVar6,
                unaff_CS,
                _u16_1050_14cc,
                0x1f8,
            );
            WinHelp16(0x28, 0x1, CONCAT22(paVar6, uVar4), (uVar3 + 0x8));
            return;
        }
        if (cVar2 == 'r') {
            iVar11 = uVar3 + 0xa;
            uVar4 = uVar7;
            paVar8 = mixed_1010_20ba(
                paVar6,
                _u16_1050_0ed0,
                CONCAT22(iVar11, 0x30),
                in_stack_0000fe98,
                in_stack_0000ffbc,
                in_stack_0000ffc2,
                in_stack_0000ffc6,
            );
            uVar5 = (paVar8 >> 0x10);
            pass1_1010_3770(uVar5, paVar8, CONCAT22(uVar4, iVar11));
            pass1_1038_af40(uVar3, uVar5, _PTR_LOOP_1050_5b7c, (uVar3 + 0x8), 0x3);
            return;
        }
        if (cVar2 == -0xf) {
            iVar11 = 0x1;
        } else {
            if (cVar2 != -0xe) {
                return;
            }
            iVar11 = 0x2;
        }
    }
    pass1_1010_4674((uVar3 + 0xf2), iVar11, 0x1010, &DAT_1050_1050);
    return;
}

pub unsafe fn invalidate_rect_1020_2ae4(
    mut param_1: u16,
    param_2: *mut u32,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut cVar3: u8;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_CS: u16;
    let mut paVar10: *mut astruct_477;
    let mut puVar11: *mut u32;
    let mut uVar12: u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut wparam: WPARAM16;
    let mut hwnd: HWND16;

    paVar7 = CONCAT22(in_register_0000000a, param_1);
    if (param_3 != 0x129) {
        uVar8 = param_2;
        uVar9 = (param_2 >> 0x10);
        if (0x129 < param_3) {
            if (param_3 == 0x12a) {
                hwnd = (uVar8 + 0x8);
                wparam = 0xf012;
            } else {
                if (param_3 == 0x12b) {
                    return;
                }
                if (param_3 == 0x12c) {
                    hwnd = (uVar8 + 0x8);
                    wparam = 0xf020;
                } else {
                    if (param_3 == 0x12d) {
                        return;
                    }
                    if (param_3 != 0x12e) {
                        return;
                    }
                    hwnd = (uVar8 + 0x8);
                    wparam = 0xf060;
                }
            }
            PostMessage16(0x0, wparam, 0x112, hwnd);
            return;
        }
        if (param_3 == 0xfb) {
            puVar11 = mixed_1010_20ba(
                paVar7,
                _u16_1050_0ed0,
                CONCAT22(param_4, 0x30),
                in_stack_0000fe9c,
                in_stack_0000ffc0,
                in_stack_0000ffc6,
                in_stack_0000ffca,
            );
            pass1_1010_375e(puVar11);
            ppcVar1 = (*param_2 + 0x14);
            (**ppcVar1)();
            uVar12 = pass1_1010_375e(puVar11);
            uVar2 = (uVar12 >> 0x10);
            pass1_1018_181c(
                CONCAT22(uVar12, uVar2),
                (uVar8 + 0xf2),
                (uVar12 & 0xffff | uVar2 << 0x10),
            );
            return;
        }
        if (param_3 < 0xfc) {
            cVar3 = param_3;
            if ((cVar3 + 0x91) == 0) {
                uVar5 = FUN_1010_830a(
                    param_3 & 0xff00 | (cVar3 + 0x91),
                    paVar7,
                    unaff_CS,
                    _u16_1050_14cc,
                    0x1f8,
                );
                WinHelp16(0x2a, 0x1, CONCAT22(paVar7, uVar5), (uVar8 + 0x8));
                return;
            }
            if (cVar3 == 'r') {
                iVar4 = uVar8 + 0xa;
                uVar5 = uVar9;
                paVar10 = mixed_1010_20ba(
                    paVar7,
                    _u16_1050_0ed0,
                    CONCAT22(iVar4, 0x30),
                    in_stack_0000fe98,
                    in_stack_0000ffbc,
                    in_stack_0000ffc2,
                    in_stack_0000ffc6,
                );
                uVar6 = (paVar10 >> 0x10);
                pass1_1010_3770(uVar6, paVar10, CONCAT22(uVar5, iVar4));
                pass1_1038_af40(uVar8, uVar6, _PTR_LOOP_1050_5b7c, (uVar8 + 0x8), 0x3);
                return;
            }
            if (cVar3 == 'u') {
                pass1_1018_0a76((uVar8 + 0xf2));
                InvalidateRect16(0x0, NULL, 0x0);
                return;
            }
        }
    }
    return;
}

pub unsafe fn post_msg_1020_4394(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if ((iVar2 + 0x34) != 0) {
            PostMessage16(0x0, 0xf6, 0x111, HWND16_1050_0396);
            return;
        }
    } else if (param_2 < 0x11) {
        if (param_2 == '\x01') {
            (iVar2 + 0x18) = 0;
            return;
        }
        // if (param_2 == '\v') {
        //   uVar1 = (iVar2 + 0x2c);
        //   (uVar1 + 0xe) = (iVar2 -0xda);
        //   return;
        // }
    }
    return;
}

pub unsafe fn post_win_msg_1020_7308(mut param_1: u32, mut param_2: u16) {
    let mut cVar1: u8;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (param_2 != 0x12) {
        if (param_2 < 0x13) {
            cVar1 = param_2;
            if (cVar1 == '\x01') {
                (param_1 + 0x1c) = 0;
                return;
            }
            //      if (('\x03' < (cVar1 -1)) && ((cVar1 -0x5) < '\x02')) goto LAB_1020_7310;
        }
        return;
    } //
    // LAB_1020_7310:
    PostMessage16(0x0, 0xeb, 0x111, (param_1 + 0x4));
    invalidate_rect_1020_735a(param_1);
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

pub unsafe fn send_msg_1020_097e(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0xea) | (iVar2 + 0xe8)) != 0) {
        uVar1 = (iVar2 + 0xe8);
        SendMessage16(0x0, 0x1, 0x111, (uVar1 + 0x6));
        (iVar2 + 0xe8) = 0;
    }
    return;
}

pub unsafe fn send_msg_1028_e242(param_1: u32, mut param_2: i16) {
    let mut puVar1: *mut u8;
    let mut unaff_DI: i16;
    let mut LVar2: LRESULT;

    puVar1 = (*param_1 % 0x64);
    if (*param_1 % 0x64 == 0) {
        LVar2 = SendMessage16(0x0, 0x0, 0x41, HWND16_1050_0396);
        puVar1 = (LVar2 >> 0x10);
    }
    *param_1 = *param_1 + 1;
    if (param_2 != 0) {
        pass1_1028_e28a(puVar1, unaff_DI, &DAT_1050_1050);
    }
    return;
}


pub unsafe fn send_msg_1038_c228(mut param_1: u32) -> LRESULT

{
  let mut wparam: WPARAM16;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut wparam_00: WPARAM16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendMessage16(0x0,0x0,0x407,(iVar1 + 0x92));
  wparam = LVar3;
  SendMessage16(0x0,0x0,0x407,(iVar1 + 0x94));
  wparam_00 = 0x408;
  SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x9e),wparam,0x408,(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x19e),wparam_00,0x408,(iVar1 + 0x94));
  return LVar3;
}

pub unsafe fn send_msg_1038_c374(mut param_1: u32, param_2: *mut u32, mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut lparam: *mut c_char;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar5 = SUB42(s_tile2_bmp_1050_1538,0x0);
  LVar6 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar3 = LVar6;
  ppcVar2 = (*param_2 + 0x10);
  (**ppcVar2)(s_tile2_bmp_1050_1538,param_2);
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uStack10 = 0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar2 = (*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar2)(uVar5,param_2,uStack10,(uStack10 >> 0x10));
    uVar1 = (param_1 + 0x8e);
    lparam = string_1008_e586(uVar1,(uVar1 >> 0x10),
                                      CONCAT13((extraout_DX_00 >> 0x8),CONCAT12(extraout_DX_00,uVar4))
                                      ,uVar4,extraout_DX_00);
    LVar6 = SendMessage16(lparam,0x0,0x403,param_3);
    uVar5 = 0x1000;
    fn_ptr_1000_17ce;
    if (LVar6 == -1) { break; }
    if (LVar6 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}


pub unsafe fn send_msg_1040_1696(param_1: *mut StructB, mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut puVar3: *mut u16;
  let mut puVar4: *mut u8;
  let mut puVar5: *mut u8;
  let mut uVar6: u16;
  let mut LVar7: LRESULT;
  let mut pcVar8: *mut c_char;
  let mut WVar9: WPARAM16;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut uStack18: u16;
  let mut local_4: u16;

  SendMessage16(0x0,0x0,0x40b,param_2);
  LVar7 = SendMessage16(0x0,0x0,0xb,param_2);
  puVar4 = (LVar7 >> 0x10);
  local_4 = 0;
  puVar3 = &local_4;
  uVar6 = (param_1 >> 0x10);
  pass1_1010_519a(puVar4,(param_1 + 0x8e),CONCAT22(0x1050,puVar3));
  puVar5 = puVar4;
  for uStack18 in 0 .. local_4 {
    uVar1 = (puVar3 + uStack18 * 0x2);
    WVar9 = 0;
    UVar10 = 0x403;
    uVar2 = (param_1 + 0x8e);
    uVar11 = param_2;
    pcVar8 = string_1010_5286(uVar1,puVar5,uVar2,(uVar2 >> 0x10),uVar1);
    LVar7 = SendMessage16(pcVar8,WVar9,UVar10,uVar11);
    puVar5 = (LVar7 >> 0x10);
    fn_ptr_1000_17ce(pcVar8);
  }
  WVar9 = 0;
  UVar10 = 0x40a;
  uVar11 = param_2;
  pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
  SendMessage16(pcVar8,WVar9,UVar10,uVar11);
  SendMessage16(0x0,0x1,0xb,param_2);
  return;
}


pub unsafe fn send_msg_1040_323c(mut param_1: u32) -> LRESULT

{
  let mut wparam: WPARAM16;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut wparam_00: WPARAM16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendMessage16(0x0,0x0,0x407,(iVar1 + 0x92));
  wparam = LVar3;
  SendMessage16(0x0,0x0,0x407,(iVar1 + 0x94));
  wparam_00 = 0x408;
  SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x9a),wparam,0x408,(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x19a),wparam_00,0x408,(iVar1 + 0x94));
  return LVar3;
}

pub unsafe fn send_msg_1040_3374(mut param_1: u32, param_2: *mut u32, mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut lparam: *mut c_char;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar4 = SUB42(s_tile2_bmp_1050_1538,0x0);
  LVar5 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar2 = LVar5;
  ppcVar1 = (*param_2 + 0x10);
  (**ppcVar1)(s_tile2_bmp_1050_1538,param_2);
  uStack6 = CONCAT22(extraout_DX,uVar2);
  uStack10 = 0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (*param_2 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)(uVar4,param_2,uStack10,(uStack10 >> 0x10));
    lparam = pass1_1018_3a7a(uVar3,extraout_DX_00,(param_1 + 0x96),
                                     CONCAT13((extraout_DX_00 >> 0x8),CONCAT12(extraout_DX_00,uVar3)))
    ;
    LVar5 = SendMessage16(lparam,0x0,0x403,param_3);
    uVar4 = 0x1000;
    fn_ptr_1000_17ce;
    if (LVar5 == -1) { break; }
    if (LVar5 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}


pub unsafe fn win_msg_1040_a308(param_1: *mut astruct_49, mut param_2: u16, mut param_3: u16, mut param_4: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut puVar7: *mut u32;
  let mut pcVar8: *mut c_char;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut WVar9: WPARAM16;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000fff2: u16;
  let mut iStack12: i16;

  uVar3 = (in_EDX >> 0x10);
  SendMessage16(0x0,0x0,0x405,param_4);
  LVar6 = SendMessage16(0x0,0x0,0xb,param_4);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar2 = (iVar4 + 0x90);
  if ((uVar2 + 0x10) == 0) {
    WVar9 = 0;
    UVar10 = 0x401;
    uVar11 = param_4;
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
    SendMessage16(pcVar8,WVar9,UVar10,uVar11);
  }
  else {
    puVar7 = mixed_1010_20ba(CONCAT22(uVar3,(LVar6 >> 0x10)),_u16_1050_0ed0,
                             CONCAT22(in_stack_0000fff2,0x3),in_stack_0000fe9a,in_stack_0000ffbe,
                             in_stack_0000ffc4,in_stack_0000ffc8);
    // for (iStack12 = 0; uVar2 = (iVar4 + 0x90), piVar1 = (uVar2 + 0x10),
    //     *piVar1 != iStack12 && iStack12 <= *piVar1; iStack12 += 1)
    iStack12 = 0;
    uVar2 = iVar4 + 0x90;
    piVar1 = uVar2 + 0x10;
    while *piVar1 != iStack12 && iStack12 <= *piVar1
    {
      WVar9 = 0;
      UVar10 = 0x401;
      uVar2 = (iVar4 + 0x90);
      uVar2 = (uVar2 + 0xc);
      uVar11 = param_4;
      pcVar8 = load_string_1010_ac92
                         (puVar7,(puVar7 >> 0x10),(uVar2 + iStack12 * 0x2));
      SendMessage16(pcVar8,WVar9,UVar10,uVar11);
      iStack12 += 1;
    }
  }
  LVar6 = SendMessage16(0x0,0x1,0xb,param_4);
  return CONCAT22((LVar6 >> 0x10),1);
}


pub unsafe fn call_win_proc_1040_a40e(mut param_1: u16, param_2: HWND16, mut param_3: u32, param_4: LPARAM) -> u32

{
  let mut func: LPVOID = null_mut();
  let mut uVar1: u16;
  let mut ppcVar2: *mut *mut code;
  let mut wparam: WPARAM16;
  let mut uVar6: u16;
  let mut uVar3: u32;
  let mut uStack6: u32;
  let mut puVar3: *mut u32;
  let mut uVar5: u32;

  uStack6 = 0;
  wparam = (param_3 >> 0x10);
  if (param_4 == 0x19) {
    ppcVar2 = (_PTR_LOOP_1050_5ee0 + 0x34);
    uStack6 = (**ppcVar2)();
    param_1 = (uStack6 >> 0x10);
  }
  else {
    if (param_4 == 0x86) {
      ppcVar2 = (_PTR_LOOP_1050_5ee0 + 0x20);
      uVar3 = (**ppcVar2)();
      return uVar3;
    }
    if (param_4 == 0x110) {
      uVar3 = win_msg_1040_a308(_PTR_LOOP_1050_5ee0,param_2,param_3,wparam);
      return uVar3;
    }
  }
  if (uStack6 != 0) {
    return uStack6 & 0xffff | param_1 << 0x10;
  }
  uVar6 = (_u16_1050_5bc8 >> 0x10);
  func = (_u16_1050_5bc8 + 0x4);
  uVar1 = (_u16_1050_5bc8 + 0x6);
  if ((uVar1 | func) == 0) {
    return uVar1 << 0x10;
  }
  uVar3 = CallWindowProc16(CONCAT22(param_3,param_2),wparam,param_4,(param_4 >> 0x10),func);
  return uVar3;
}


pub unsafe fn send_msg_1040_4cb2(mut param_1: u16, mut param_2: u32) -> LRESULT

{
    let mut uVar1: u8;
    let mut HVar1: HWND16;
    let mut uVar2: u32;
    let mut LVar2: LRESULT;
    let mut hwnd: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    pass1_1040_b45e(param_2);
    HVar1 = GetDlgItem16(0x1770, (param_2 + 0x6));
    uVar3 = 0xffff;
    uVar4 = 0x40d;
    hwnd = HVar1;
    pass1_1040_4d7e(param_2);
    uVar2 = pass1_1040_4dcc(param_1, param_2, HVar1);
    LVar2 = SendMessage16(uVar2, uVar3, uVar4, hwnd);
    return LVar2;
}

pub unsafe fn pass1_1040_93e6(mut param_1: u32) -> LRESULT

{
    let mut uVar1: u16;
    let mut LVar2: LRESULT;

    uVar1 = (param_1 >> 0x10);
    LVar2 = SendMessage16(0x0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}


pub unsafe fn win_ui_op_1040_311a(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut in_EDX: u32;
    let mut uVar4: u16;
    let mut uVar3: u32;
    let mut LVar5: LRESULT;
    let mut paVar6: *mut Struct27;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut iVar7: i16;

    uVar4 = (in_EDX >> 0x10);
    send_msg_1040_323c(CONCAT22(param_2, param_1));
    load_string_1010_847e(_u16_1050_14cc, 0x531);
    if (param_4 == 0x181) {
        uVar3 = CONCAT22(uVar4, param_2);
        iVar1 = param_1 + 0x9a;
        iVar7 = iVar1;
        pass1_1018_3cda((param_1 + 0x96), CONCAT22(param_2, param_1 + 0x19a),
                        CONCAT22(param_2, iVar1));
        pass1_1018_3424(iVar7, uVar3, (param_1 + 0x96));
        if (iVar7 == 0) {
            iVar7 = 0x21;
        } else {
            pass1_1018_3a42(uVar3, (param_1 + 0x96), CONCAT22(param_2, iVar1));
            pass1_1030_8344(_u16_1050_5748, CONCAT22(uVar3, iVar7));
            uVar2 = (iVar7 + 0x10);
            pass1_1030_8344(_u16_1050_5748, uVar2);
            PTR_LOOP_1050_5f0c = uVar2;
            PTR_LOOP_1050_5f0e = uVar3;
            PTR_LOOP_1050_5f10 = (&PTR_LOOP_1050_0000 + 1);
            iVar7 = 0x25;
        }
        pass1_1038_af40(param_1, uVar3, _PTR_LOOP_1050_5b7c, (param_1 + 0x8), iVar7);
        uVar4 = (uVar3 >> 0x10);
        LVar5 = SendMessage16(0x0, 0x2, 0x111, (param_1 + 0x6));
        iVar7 = 0x1;
        paVar6 = mixed_1010_20ba(CONCAT22(uVar4, (LVar5 >> 0x10)), _u16_1050_0ed0,
                                 0x1002b, in_stack_0000fe8e, in_stack_0000ffb2, in_stack_0000ffb8,
                                 in_stack_0000ffbc);
        pass1_1010_038e(paVar6, iVar7);
    } else {
        if ((param_4 == 0x181) || (0x1 < param_4 - 0x182)) {
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4);
            return;
        }
        set_win_pos_1040_331a(CONCAT22(param_2, param_1), param_3, param_4);
    }
    return;
}

pub unsafe fn send_msg_1008_9640(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        SendMessage16(0x0, param_2, 0x86, (param_1 + 0x8));
    }
    return;
}


pub unsafe fn send_msg_1010_7c42(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x1e) | (iVar2 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar2 + 0x1c));
        loop {
            lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            if (lVar4 == 0) {
                break;
            }
            uVar1 = (lVar4 + 0x8);
            SendMessage16(0x0, 0xeb, 0x111, (uVar1 + 0x6));
        }
    }
    return;
}

pub unsafe fn send_msg_1020_29d8(
    mut param_1: u16,
    param_2: *mut Struct57,
    param_3: *mut astruct_69,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u16,
) -> u32 {
    let mut puVar1: *mut u8;
    let mut paVar2: *mut Struct27;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut iVar3: i16;

    iVar3 = (param_5 >> 0x10);
    post_win_msg_1020_79fc(param_3, param_4, param_5, iVar3);
    paVar2 = mixed_1010_20ba(
        param_2,
        _u16_1050_0ed0,
        CONCAT22(param_6, 0x29),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    puVar1 = (paVar2 >> 0x10);
    if (iVar3 == 0) {
        pass1_1018_270e(puVar1, paVar2, 0x1, (param_3 + 0xfc));
    } else {
        pass1_1018_270e(puVar1, paVar2, 0x0, (param_3 + 0xfc));
        SendMessage16(0x0, 0x69, 0x111, HWND16_1050_0396);
    }
    return CONCAT22(param_2, param_1);
}


pub unsafe fn win_help_1040_800c(mut param_1: u32)

{
    let mut in_AX: u16;
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut iVar4: i16;
    let mut w_command: u16;
    let mut hwnd: HWND16;

    uVar1 = FUN_1010_830a(in_AX, in_EDX, unaff_CS, _u16_1050_14cc, 0x1f8);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x8a) == 0) {
        hwnd = (iVar2 + 0x6);
        iVar4 = 0;
        w_command = 0x3;
        iVar2 = 0;
    } else {
        hwnd = (iVar2 + 0x6);
        w_command = 0x1;
        iVar2 = (iVar2 + 0x8a);
        iVar4 = iVar2 >> 0xf;
    }
    WinHelp16(CONCAT22(iVar4, iVar2), w_command, CONCAT22(in_EDX, uVar1), hwnd);
    return;
}
