use std::ptr::null_mut;
use std::os::raw::c_char;
use crate::draw_ops;
use crate::draw_ops::draw_f;
use crate::globals::u32_1050_0004;
use crate::no_refs::h::pass1_1010_5d9c;
use crate::no_refs::i::{pass1_1010_60b4, pass1_1010_60ba, pass1_1010_60c0, pass1_1010_60c6};
use crate::no_refs::k::unk_load_str_op_1010_8c96;
use crate::no_refs::l::pass1_1010_c3c2;
use crate::resources::load_string_1010_847e;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::mem_op_1000_179c;
use crate::unk::block_1000_4000::pass1_1000_4906;
use crate::unk::block_1008_5000::{pass1_1008_5784, pass1_1008_5b12, win_1008_5c7c};
use crate::unk::block_1008_8000;
use crate::unk::block_1008_b000::{load_string_1008_b1f0, pass1_1008_b146, pass1_1008_b1a6, pass1_1008_b200, pass1_1008_b366, pass1_1008_b38c, pass1_1008_b47a, pass1_1008_b4a0, pass1_1008_b61a, pass1_1008_b63a, pass1_1008_b820};
use crate::unk::block_1008_c000::pass1_1008_c79a;
use crate::unk::block_1010_0000::pass1_1010_038e;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_3000::pass1_1010_375e;
use crate::unk::block_1010_6000::pass1_1010_60a0;
use crate::unk::block_1018_3000::{pass1_1018_36e6, pass1_1018_3ab2, switch_1018_3b9e};
use crate::unk::block_1020_c000::string_op_1020_c222;
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1030_8000::pass1_1030_8326;
use crate::unk::block_1038_3000::pass1_1038_3aa6;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1040_b000::pass1_1040_b54a;
use crate::utils::CONCAT22;
use crate::gui::{cleanup, dialog, window};
use crate::gui::window::win_e;
use crate::winapi16::{CheckDlgButton16, CheckRadioButton16, DestroyWindow16, EnableWindow16, GetDlgCtrlID16, GetDlgItem16, GetDlgItemInt16, GetWindowLong16, GetWindowWord16, IsDlgButtonChecked, PostMessage16, SendDlgItemMessage16, SendMessage16, SetDlgItemInt16, SetDlgItemText16, SetWindowLong16, SetWindowPos16};
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::windef16::{HWND16, LRESULT, WPARAM16};

pub fn win_sys_op_1038_a9fa(mut param_1: u32, mut param_2: i16)

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

  if param_2 != 0 {
    puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000fff0,0x2),in_stack_0000fe98,
                             in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x116,(iVar1 + 0x6));
    if ((LVar4 >> 0x10) | LVar4) == 0 {
      LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x117,(iVar1 + 0x6));
      if ((LVar4 >> 0x10) | LVar4) == 0 {
        LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x118,(iVar1 + 0x6));
        if ((LVar4 >> 0x10) | LVar4) == 0 {
          LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x119,(iVar1 + 0x6));
          if ((LVar4 >> 0x10) | LVar4) != 0 {
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
    cleanup::destroy_win_1040_7b98(param_1);
  }
  return;
}

pub fn win_ui_op_1008_8214(
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

pub fn send_dlg_item_msg_1038_8d22(mut param_1: u32, param_2: u8)

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


pub fn send_dlg_item_msg_1038_87b2(mut param_1: u16, param_2: *mut Struct903) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut l_param: *mut c_char;
  let mut LVar3: LRESULT;
  let mut uVar4: u16;
  let mut local_102: [u8;0x100] = [0;0x100];

  uVar4 = param_2;
  uVar1 = (param_2 >> 0x10);
  uVar2 = send_dlg_item_msg_1038_8164(uVar4, uVar1, CONCAT22(0x1050, local_102), 0x1855);
  if (uVar2 != 0) {
    pass1_1008_b61a(local_102,param_1,(uVar4 + 0x94),CONCAT22(0x1050,local_102));
    l_param = load_string_1008_b1f0();
    LVar3 = SendDlgItemMessage16(l_param,0xffff,0x40d,0x1856,(uVar4 + 0x6));
    uVar2 = LVar3;
  }
  return uVar2;
}


pub fn send_dlg_item_msg_1038_8618s(mut param_1: u16, param_2: *mut Struct903) -> u16

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
    uVar1 = send_dlg_item_msg_1038_8164(uVar4, uVar5, CONCAT22(0x1050, local_106), 0x1854);
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
        send_dlg_item_msg_1038_8400(uVar4, uVar5, CONCAT22(puVar3, puVar2), 0x1855);
        l_param = pass1_1008_b366((uVar4 + 0x94));
        uVar7 = l_param & 0xffff | ((l_param >> 0x10) | l_param) << 0x10;
        if (l_param != 0) {
          uVar7 = SendDlgItemMessage16(l_param,0xffff,0x40d,0x1855,(uVar4 + 0x6));
        }
      }
      uVar7 = pass1_1008_b38c(CONCAT22(uVar7,(uVar7 >> 0x10)),(uVar4 + 0x94));
      if (uVar7 != 0) {
        send_dlg_item_msg_1038_8400(uVar4, uVar5, uVar7, 0x1856);
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

pub fn send_dlg_item_msg_1038_8400(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u16 )

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

pub fn send_dlg_item_msg_1038_8164(mut param_1: u16, mut param_2: u16, param_3: *mut u8, mut param_4: u16 ) -> u16

{
  let mut LVar1: LRESULT;

  *param_3 = '\0';
  LVar1 = SendDlgItemMessage16(0x0,0x0,0x409,param_4,(param_1 + 0x6));
  if (LVar1 != -1) &&
     (LVar1 = SendDlgItemMessage16(param_3,LVar1,0x40a,param_4,(param_1 + 0x6)),
     LVar1 != -1) {
    return 0x1;
  }
  return 0x0;
}

pub fn send_dlg_item_msg_1038_7fae(mut param_1: u16, mut param_2: u16, mut param_3: u32)

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


pub fn send_dlg_item_msg_1038_7eac(param_1: *mut Struct903) -> LRESULT

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
    win_e::enable_win_1038_806a((LVar4 >> 0x10), param_1);
  }
  LVar4 = SendDlgItemMessage16(0x0,0x1,0xb,0x1854,(iVar1 + 0x6));
  return LVar4;
}


pub fn unk_win_ui_op_1040_19ea(param_1: *mut astruct_32, mut param_2: i16, param_3: *mut u8)

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


pub fn chk_is_dlg_btn_checked_1038_e7a0(param_1: *mut astruct_62, mut param_2: i16)

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

pub fn check_dialog_btn_1040_1b8a(param_1: *mut StructC)

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

pub fn check_dlg_btn_checked_1038_cdd6(param_1: *mut astruct_61, mut param_2: i16)

{
  let mut uVar1: u32;
  let mut UVar2: u16;
  let mut iVar3: *mut astruct_61;
  let mut uVar3: u16;

  iVar3 = param_1;
  uVar3 = (param_1 >> 0x10);
  if param_2 == 0 {
    uVar1 = iVar3.field142_0x8e;
    (uVar1 + 0xa) = 0;
  }
  else {
    UVar2 = IsDlgButtonChecked(0x182e,&iVar3.field_0x6);
    if UVar2 == 0 {
      UVar2 = IsDlgButtonChecked(0x182f,&iVar3.field_0x6);
      if UVar2 == 0 {
        UVar2 = IsDlgButtonChecked(0x1829,&iVar3.field_0x6);
        if UVar2 == 0 {
          UVar2 = IsDlgButtonChecked(0x182a,&iVar3.field_0x6);
          if UVar2 == 0 {
            UVar2 = IsDlgButtonChecked(0x182c,&iVar3.field_0x6);
            if UVar2 == 0 {
              UVar2 = IsDlgButtonChecked(0x182d,&iVar3.field_0x6);
              if UVar2 != 0 {
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


pub fn check_dialog_btn_1040_1afe(param_1: *mut StructB)

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

pub fn win_dlg_item_1040_2d48(mut param_1: u32)

{
    let mut UVar1: u16;
    let mut value: u16;
    let mut local_4: bool;

    pass1_1040_b45e(param_1);
    UVar1 = GetDlgItemInt16(0x1, &local_4, 0x1050, 0x163);
    value = GetDlgItemInt16(0x1, &local_4, 0x1050, 0x165);
    if (UVar1 != 0) {
        value /= UVar1;
    }
    SetDlgItemInt16(0x1, value, 0x165, (param_1 + 0x6));
    return;
}

pub fn unk_win_ui_op_1040_3c64(mut param_1: u16, struct_c_param_1: *mut StructC, struct_c_param_2: *mut StructC, mut param_4: u16, mut param_5: u32)

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


pub fn win_dlg_op_1038_9294(mut param_1: u16, param_2: *mut StructB)

{
  let mut UVar1: u16;
  let mut uVar1: u16;
   let mut struct_b_1_hi: *mut StructB;
  let mut local_6: bool;
  let mut local_4: bool;

  window::unk_win_ui_op_1040_b230(param_1, param_2);
  struct_b_1_hi = (param_2 >> 0x10);
  UVar1 = GetDlgItemInt16(0x1,&local_4,0x1050,0xfa9);
  (param_2 + 0x94) = UVar1;
  uVar1 = GetDlgItemInt16(0x1,&local_6,0x1050,0xfa8);
  (param_2 + 0x96) = uVar1;
  win_ui_dlg_op_1038_98b4((param_2 & 0xffff | ZEXT24(struct_b_1_hi) << 0x10));
  win_1008_5c7c(uVar1,param_1,_u16_1050_02a0,0x950001);
  return;
}


pub fn unk_win_ui_op_1038_9820(param_1: *mut StructB, mut param_2: i16, mut param_3: i16, mut param_4: i16) -> i32

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
  UVar4 = GetDlgItemInt16(0x1,&local_4,0x1050,(param_4 * 0xe + 0x5a74));
  iVar5 = UVar4 * param_2 * param_3;
  UVar4 = GetDlgItemInt16(0x1,&local_6,0x1050,(param_4 * 0xe + 0x5a76));
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


pub fn pass1_1038_8966(mut param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: i16, mut param_5: u16 ) -> u16

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

pub fn send_dlg_item_msg_1040_3f12(struct_c_param_1: *mut StructC, struct_c_param_2: *mut StructC, mut param_3: u32)

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

pub fn win_ui_dlg_op_1038_98b4(param_1: *mut StructB)

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
    UVar1 = GetDlgItemInt16(0x1,&local_4,0x1050,(iStack8 * 0xe + 0x5a72));
    unk_win_ui_op_1038_9820(param_1, UVar1, pvVar2, iVar4);
    iStack8 += 1;
  }
  SetDlgItemInt16(0x1,iVar3[0x7].max_count_field_0x10,0xfa9,iVar3.lpvoid_field_0x8);
  SetDlgItemInt16(0x1,iVar3[0x7].field5_0xa,0xfa8,iVar3.lpvoid_field_0x8);
  return;
}

pub fn dialog_item_ui_op_1040_3e08(struct_c_param_1: *mut StructC)

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
  if LVar2 != -1 {
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
  draw_f::invalidate_rect_1040_3ddc(struct_c_param_1);
  return;
}


pub fn send_dlg_item_msg_1038_8b58(param_1: *mut Struct903)

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


pub fn pass1_1040_b45e(mut param_1: u32)

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


pub fn win_ui_op_1040_ae04(mut param_1: u16, mut param_2: u32)

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


pub fn enable_window_1038_9cec(param_1: *mut u8, param_2: *mut Struct903, mut param_3: u16, mut param_4: u16, mut param_5: i16)

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
      enable = 0x1538;
      GetDlgItem16(0x17d8,(param_2 + 0x6));
      HVar6 = 0;
    }
    EnableWindow16(enable,HVar6);
  }
  return;
}
