pub mod dlg_a;
pub mod dlg_b;

use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::globals::u32_1050_0004;
use crate::no_refs::i::{pass1_1010_5fd8, pass1_1010_60b4, pass1_1010_60ba, pass1_1010_60c0, pass1_1010_60c6, pass1_1010_659a, pass1_1010_6604};
use crate::no_refs::k::{pass1_1010_9044, pass1_1010_9130, unk_load_str_op_1010_8c96};
use crate::no_refs::l::{pass1_1010_c3c2, string_op_1010_c446};
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1000_3000::str_op_1000_3da4;
use crate::unk::block_1008_3000::pass1_1008_3bd6;
use crate::unk::block_1008_5000::{pass1_1008_5784, pass1_1008_5b12, win_1008_5c5c, win_1008_5c7c, win_1008_5c9e};
use crate::unk::block_1008_9000::pass1_1008_941a;
use crate::unk::block_1008_a000::pass1_1008_a930;
use crate::unk::block_1008_b000::{load_string_1008_b1f0, pass1_1008_b146, pass1_1008_b1a6, pass1_1008_b200, pass1_1008_b366, pass1_1008_b38c, pass1_1008_b47a, pass1_1008_b4a0, pass1_1008_b61a, pass1_1008_b63a, pass1_1008_b820};
use crate::unk::block_1008_c000::{pass1_1008_c79a, pass1_1008_c83a, pass1_1008_c85e};
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_c000::pass1_1010_c320;
use crate::unk::block_1010_d000::struct_1010_dd5e;
use crate::unk::block_1018_1000::{pass1_1018_1e78, pass1_1018_1f1a};
use crate::unk::block_1018_3000::{pass1_1018_36e6, pass1_1018_3ab2, pass1_1018_3d44, switch_1018_3b9e};
use crate::unk::block_1018_5000::{pass1_1018_50ea, pass1_1018_5206, pass1_1018_526a};
use crate::unk::block_1020_b000::pass1_1020_bae6;
use crate::unk::block_1020_c000::{string_1020_c0d8, string_op_1020_c222};
use crate::unk::block_1028_4000::pass1_1028_4a9a;
use crate::unk::block_1030_6000::{pass1_1030_6ddc, pass1_1030_6e14};
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1030_8000::pass1_1030_8326;
use crate::unk::block_1038_3000::pass1_1038_3aa6;
use crate::unk::block_1038_9000::pass1_1038_993a;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_b000::pass1_1040_b54a;
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::{draw_ops, gui};
use crate::no_refs::h::pass1_1010_5d9c;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_4000::{pass1_1000_472c, pass1_1000_4906};
use crate::unk::block_1008_8000;
use crate::unk::block_1008_e000::{pass1_1008_e3ec, string_1008_e586};
use crate::unk::block_1010_0000::pass1_1010_038e;
use crate::unk::block_1010_3000::pass1_1010_375e;
use crate::unk::block_1010_6000::pass1_1010_60a0;
use crate::gui::{cleanup, window};
use crate::winapi16::{CheckDlgButton16, CheckRadioButton16, CreateDialog16, DestroyWindow16, EnableWindow16, GetDlgCtrlID16, GetDlgItem16, GetDlgItemInt16, GetMessage16, GetWindowLong16, GetWindowText16, GetWindowWord16, InvalidateRect16, IsDialogMessage16, IsDlgButtonChecked, IsWindow16, MapDialogRect16, MessageBox16, PostMessage16, SendDlgItemMessage16, SendMessage16, SetDlgItemInt16, SetDlgItemText16, SetFocus16, SetProp16, SetWindowLong16, SetWindowPos16, SetWindowText16, ShowWindow16};
use crate::winapp::winapp_c::send_msg_1038_c374;
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::windef16::{HANDLE16, HWND16, LRESULT, RECT16, WPARAM16};

pub fn send_dlg_item_msg_1040_d79c(mut param_1: u16, param_2: *mut Struct903)

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


pub fn win_dlg_op_1038_bea4(mut param_1: u16, mut param_2: u32)

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


pub fn win_ui_op_1040_cace(mut param_1: u16, mut param_2: u32)

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
    uStack6 = GetDlgItemInt16(0x0, &local_4, 0x1050, 0x18e);
    UStack8 = GetDlgItemInt16(0x0, &local_4, 0x1050, 0x191);
    if (uStack6 == 0) {
        return;
    }
    pass1_1018_50ea((uVar5 + 0x98), uStack6, (uVar5 + 0x94));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_208, 0x1050);
    uVar1 = (uVar5 + 0x94);
    uVar8 = (_u16_1050_14cc >> 0x10);
    if ((uVar1 + 0x36) == 0) {
        load_string_1010_84e0(_u16_1050_14cc, uVar8, 0x3ff, local_108, 0x1050);
        iVar3 = MessageBox16(0x34, CONCAT22(0x1050, local_208), CONCAT22(0x1050, local_108),
                             (uVar5 + 0x8));
    } else {
        load_string_1010_84e0(_u16_1050_14cc, uVar8, 0x3ff, local_108, 0x1050);
        iVar3 = MessageBox16(0x34, CONCAT22(0x1050, local_208), CONCAT22(0x1050, local_108),
                             (uVar5 + 0x8));
    }
    bVar2 = iVar3 == 0x6;
    bVar7 = false;
    if ((!bVar2) && (uVar1 = (uVar5 + 0x94), (uVar1 + 0x34) < 1)) {
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_108, 0x1050);
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

pub fn send_dlg_item_msg_1040_ce12(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u16 )

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

pub fn send_dlg_item_1040_ce76(param_1: *mut Struct903)

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

pub fn send_dlg_msg_1040_cf1c(mut param_1: u16, param_2: *mut Struct903) -> LRESULT

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
  send_dlg_item_msg_1040_ce12(uVar1, uVar2, uVar4, uVar6);
  LVar5 = SendDlgItemMessage16(0x0,0x0,0x40c,s_vrpal_bmp_1050_183a + 0x8,uVar1.field6_0x6);
  if (((LVar5 >> 0x10) < 1) && ((LVar5 < 0x0 || (LVar5 == 0)))) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,&stack0xfaf4,0x1050);
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


pub fn pass1_1040_cdac(mut param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: i16) -> u16

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


pub fn send_dlg_item_int_1038_94da(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> BOOL16

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
     (UStack6 = GetDlgItemInt16(0x1,&local_c,0x1050,(iStack8 * 0xe + 0x5a72)), local_c != 0)
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
       (lVar3 = dlg_a::unk_win_ui_op_1038_9820(CONCAT22(param_2, param_1), 0x1, iStack4, iStack8), lVar3 != 0)) {
      SetDlgItemInt16(0x1,UStack6,(iVar2 + 0x5a72),(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x94),0xfa9,(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x96),0xfa8,(param_1 + 0x6));
    }
  }
  return 0x1;
}

pub fn win_ui_op_1020_36f6(mut param_1: u32, mut param_2: i16) {
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
        0x1050,
    );
    uVar3 = (iVar9 + 0x18);
    ppcVar2 = ((iVar9 + 0x18) + 0x18);
    (**ppcVar2)(0x1010, uVar3, (uVar3 >> 0x10), local_406, 0x1050);
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


pub fn dialog_ui_fn_1040_78e2(in_struct_1: *mut StructB)

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
// pub fn *pvStack6;
  let mut fn_ptr_1: *mut *mut code;

  local_string_1 = (in_struct_1 >> 0x10);
  struct_b_1 = in_struct_1;
  if &struct_b_1.field6_0xc == 0 {
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
  (**fn_ptr_1)(0x1538,in_struct_1);
  return;
}

pub fn unk_win_ui_op_1038_d400(mut param_1: u16, param_2: *mut astruct_885, param_3: u8, param_4: u8, mut param_5: u16, mut param_6: u16,
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

pub fn dlg_ui_op_1040_2a64(mut param_1: u16, struct_b_param_1: *mut StructB)

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

    window::unk_win_ui_op_1040_b230(param_1, struct_b_param_1);
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
        MapDialogRect16(&local_16, 0x1050);
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

pub fn win_ui_op_1038_a788(mut param_1: u16, mut param_2: u32, mut param_3: i16)

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
      uVar5 = SUB42(0x1050,0x0);
      pUVar2 =
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_5fd8((pUVar2 >> 0x10),pUVar2,CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,(param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      cleanup::destroy_win_1040_7b98(param_2);
    }
  }
  return;
}


pub fn win_ui_op_1040_3b1e(mut param_1: u16, struct_c_param_1: *mut StructC)

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
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x80, local_10e, 0x1050,
    );
    uVar1 = struct_c_4.field142_0x92;
    wsprintf16(local_8c, CONCAT22(local_10e, 0x1050), CONCAT22(*puStack282, 0x1050),
               (*puStack282 >> 0x10), uVar1, (uVar1 >> 0x10));
    SetDlgItemText16(CONCAT22(0x1050, local_8c), 0x187, struct_c_4.field6_0x6);
    BVar2 = CheckRadioButton16(0x188, 0x18d, 0x188, struct_c_4.field6_0x6);
    struct_c_4.field149_0xa0 = 0x188;
    uVar5 = switch_1018_3b9e(BVar2, pSVar4, struct_c_4.field141_0x8e, struct_c_4.field149_0xa0);
    dlg_a::send_dlg_item_msg_1040_3f12(struct_c_4, struct_c_param_2, uVar5);
    dlg_a::dialog_item_ui_op_1040_3e08(struct_c_param_1);
    HVar3 = GetDlgItem16(0x186, struct_c_4.field6_0x6);
    struct_c_4.field146_0x9a = HVar3;
    return;
}


pub fn show_win_1040_18a2(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u32;
   let mut struct_b_2: *mut StructB;
  let mut uVar2: u16;
  let mut local_304: [u16;0x80] = [0;0x80];
  let mut local_204: [u8;0x100] = [0;0x100];
  let mut local_104: [u8;0x100] = [0;0x100];
  let mut uStack4: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  dlg_a::check_dialog_btn_1040_1afe(struct_b_param_1);
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
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_204,0x1050);
    wsprintf16(local_304,0x5cda1050,CONCAT22(local_204,0x1050),0x1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfe0,struct_b_2.lpvoid_field_0x8);
    uVar1 = &struct_b_2[0x7].field1_0x2;
    if ((uVar1 + 0x82) == 0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_204,0x1050);
    wsprintf16(local_304,0x5cdf1050,CONCAT22(local_204,0x1050),0x1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfdf,struct_b_2.lpvoid_field_0x8);
  }
  window::move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
  ShowWindow16(0x5,struct_b_2.lpvoid_field_0x8);
  return;
}


pub fn check_dialog_msg_1040_81b6(mut param_1: u32)

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
        local_14.hwnd = 0x1050;
        BVar1 = GetMessage16(0x0, 0x0, 0x0, &local_14);
        if (BVar1 == 0) { break; }
        IsDialogMessage16(&local_14, 0x1050);
    }
    return;
}

pub fn win_dlg_op_1038_c95e(struct_param_1: *mut astruct_882, mut param_2: i16)

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
