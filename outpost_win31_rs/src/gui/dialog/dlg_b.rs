use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::no_refs::l::{pass1_1010_c3c2, string_op_1010_c446};
use crate::resources::load_string_1010_84e0;
use crate::structs::struct_903::Struct903;
use crate::unk::block_1008_5000::{pass1_1008_5784, pass1_1008_5b12};
use crate::unk::block_1008_a000::pass1_1008_a930;
use crate::unk::block_1008_b000::pass1_1008_b820;
use crate::unk::block_1008_c000::{pass1_1008_c83a, pass1_1008_c85e};
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_c000::pass1_1010_c320;
use crate::unk::block_1010_d000::struct_1010_dd5e;
use crate::unk::block_1020_b000::pass1_1020_bae6;
use crate::unk::block_1020_c000::string_1020_c0d8;
use crate::unk::block_1030_6000::{pass1_1030_6ddc, pass1_1030_6e14};
use crate::utils::CONCAT22;
use crate::gui::dialog::dlg_a;
use crate::gui::window;
use crate::gui::window::win_d;
use crate::winapi16::{DestroyWindow16, EnableWindow16, GetDlgItem16, IsDlgButtonChecked, SendDlgItemMessage16, SetDlgItemInt16, SetDlgItemText16, SetWindowText16};
use crate::windef16::{HWND16, LRESULT};

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


pub unsafe fn enable_win_1040_060e(mut param_1: u32, mut param_2: i16) {
    let mut pIVar1: *mut INT16 = null_mut();
    let mut hwnd: HWND16;
    let mut iStack10: i16;
    let mut pIStack8: *mut INT16 = null_mut();

    pIStack8 = CONCAT22(0x1050, &stack0x000a);
    iStack10 = param_2;
    loop {
        pIVar1 = pIStack8;
        if iStack10 == 0 {
            break;
        }
        pIStack8 = (pIStack8 & 0xffff0000 | (pIStack8 + 0x2));
        hwnd = GetDlgItem16(*pIVar1, (param_1 + 0x6));
        EnableWindow16(0x0, hwnd);
        iStack10 = iStack10 - 0x1;
    }
    return;
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
               0x1050);
    SendDlgItemMessage16(CONCAT22(0x1050,&local_404),0x0,0x401,0x185b,iVar2.field6_0x6);
    hwnd = GetDlgItem16(0x1,iVar2.field6_0x6);
    enable = 0;
  }
  EnableWindow16(enable,hwnd);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0xb,0x185b,iVar2.field6_0x6);
  return LVar5;
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
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_108,0x1050);
    SendDlgItemMessage16(CONCAT22(0x1050,local_108),0x0,0x401,0x1854,uVar4.field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4.field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4.field6_0x6);
    LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4.field6_0x6);
    uVar3 = (LVar6 >> 0x10);
    hwnd = GetDlgItem16(0x1857,uVar4.field6_0x6);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_108,0x1050);
    BVar1 = SetWindowText16(CONCAT22(0x1050,local_108),hwnd);
    return CONCAT22(uVar3,BVar1);
  }
  dlg_a::send_dlg_item_msg_1038_8400(uVar4, uVar5, uStack6, 0x1854);
  win_d::set_win_text_1038_8358(uVar2, param_1);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4.field6_0x6);
  LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4.field6_0x6);
  return LVar6;
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
