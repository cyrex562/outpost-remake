use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::globals::{PTR_LOOP_1050_1040, u32_1050_0004};
use crate::no_refs::i::{pass1_1010_659a, pass1_1010_6604};
use crate::no_refs::l::{pass1_1010_a568, pass1_1010_a58a, pass1_1010_a5ac, pass1_1010_a5ec};
use crate::resources::load_string_1010_847e;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_915::astruct_915;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1000_3000::pass1_1000_3cea;
use crate::unk::block_1008_3000::pass1_1008_3bd6;
use crate::unk::block_1008_5000::{win_1008_5c5c, win_1008_5c7c};
use crate::unk::block_1008_b000::{pass1_1008_b340, pass1_1008_b366, pass1_1008_b47a, pass1_1008_b820};
use crate::unk::block_1008_e000::{pass1_1008_e2a4, pass1_1008_e320};
use crate::unk::block_1010_2000::{mixed_1010_20ba, pass1_1010_2ee2};
use crate::unk::block_1010_3000::{pass1_1010_32c0, win_ui_op_1010_3202};
use crate::unk::block_1010_a000::pass1_1010_a5ca;
use crate::unk::block_1018_1000::pass1_1018_1c9a;
use crate::unk::block_1018_3000::{pass1_1018_3a5c, string_1018_39d8};
use crate::unk::block_1018_7000::pass1_1018_6198;
use crate::unk::block_1028_d000::pass1_1028_dc52;
use crate::unk::block_1028_e000::pass1_1028_e4ec;
use crate::unk::block_1030_2000::{pass1_1030_2f1a, pass1_1030_2fac};
use crate::unk::block_1030_5000::pass1_1030_532e;
use crate::unk::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e};
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_b000::pass1_1040_b54a;
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::gui::dialog::dlg_b;
use crate::gui::{cleanup, dialog, window};
use crate::gui::window::win_d;
use crate::winapi16::{CheckRadioButton16, DestroyWindow16, EnableWindow16, GetClientRect16, GetDlgItem16, GetSystemMetrics16, InvalidateRect16, IsWindow16, LoadCursor16, MapDialogRect16, PostMessage16, SendDlgItemMessage16, SendMessage16, SetCursor16, SetWindowPos16, SetWindowText16, ShowWindow16};
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::winapp::winapp_a::unk_win_op_1010_7300;
use crate::windef16::{BOOL16, HCURSOR16, HWND16, LRESULT, RECT16, WPARAM16};

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

  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
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
  window::move_win_1040_826c(struct_b_param_1, 0xc8, 0x0);
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

  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
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
    MapDialogRect16(rect,0x1050);
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
  window::move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
  BVar2 = ShowWindow16(0x5,struct_b_6.lpvoid_field_0x8);
  win_1008_5c7c(BVar2,uVar4,_u16_1050_02a0,0x9a0001);
  (struct_b_6 + 0x7).field0_0x0 = BVar2;
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
            dlg_b::enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0;
        }
        // break;
        0x168 => {
            dlg_b::enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x1;
        }
        // break;
        0x169 => {
            dlg_b::enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x2;
        }
        // break;
        0x16a => {
            dlg_b::enable_win_1040_060e(param_3, 0x3);
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
            uVar14 = SUB42(0x1050, 0x0);
            uVar12 = SUB21(&local_16, 0x0);
            uVar13 = (&local_16 >> 0x8);
            pHVar10 = &local_18;
            uVar11 = SUB42(0x1050, 0x0);
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
            win_d::set_window_text_1018_6086((param_3 + 0x96), uVar4, local_16);
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

    dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar3 = (struct_b_param_1 >> 0x10);
    struct_b_4 = struct_b_param_1;
    hwnd = GetDlgItem16(0xfb1, struct_b_4.lpvoid_field_0x8);
    EnableWindow16(0x0, hwnd);
    ppcVar1 = (*&struct_b_4[0x7].field1_0x2 + 0x10);
    piVar2 = (**ppcVar1)(0x1538, &struct_b_4[0x7].field1_0x2);
    piVar2 = (piVar2 >> 0x10);
    window::move_win_1040_826c(struct_b_param_1, (piVar2 + 0x2) - 0x2, (piVar2 + 0x4) + *piVar2 + 0x3);
    ShowWindow16(0x5, struct_b_4.lpvoid_field_0x8);
    pass1_1018_1c9a(&struct_b_4[0x7].field1_0x2, 0x1a0);
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
            GetClientRect16(&local_a, 0x1050);
            paVar2 = param_1.field144_0x94;
            local_a.y += 0x3;
            local_a.x = (paVar2 + 0x1a) - 0x9;
            iStack6 += -0x3;
            iStack4 = iStack4 - 0x3;
            InvalidateRect16(0x1, &local_a, 0x1050);
            cleanup::unk_destroy_win_op_1010_2fa0(param_1.field144_0x94);
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
                GetClientRect16(&local_a, 0x1050);
                puVar5 = param_3.field148_0x98;
                local_a.y += 0x3;
                local_a.x = (puVar5 + 0x1a) - 0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(0x1, &local_a, 0x1050);
                cleanup::unk_destroy_win_op_1010_2fa0(param_3.field148_0x98);
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
                    window::ui_op_1010_79aa(paVar17, 0xfc6, param_3.field169_0xb0);
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
        GetClientRect16(&local_a, 0x1050);
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
        GetClientRect16(&local_a, 0x1050);
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
    rect = 0x1050;//
// LAB_1040_c19d:
    InvalidateRect16(erase, rect, hwnd);
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
    dialog::dialog_ui_fn_1040_78e2(structb_param_1);
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
    GetClientRect16(&local_e, 0x1050);
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
