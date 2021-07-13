use crate::util::{CONCAT22, ZEXT24};
use crate::struct_ops::struct_1018::struct_1020_0baa;
use crate::mem_1000::mem_op_1000_179c;
use crate::ui::ui_1008::{create_window_ex_1008_9760, win_1008_5c5c};
use crate::winapi::{SendMessage16, IsWindow16, PostMessage16, ShowCursor16, SetWindowText16, GetDlgItem16, wsprintf16, SetWindowPos16, GetSystemMetrics16, GetClientRect16};
use crate::win_struct::{HWND16, WNDCLASS16, SEGPTR, RECT16};
use crate::pass::pass_1008::{pass1_1008_57c4, pass1_1008_405c, pass1_1008_4d84};
use crate::pass::pass_1018::{pass1_1020_04f6, pass1_1018_ec74, pass1_1018_e834, pass1_1018_e4f2, struct_1018_e100, pass1_1018_6924, pass1_1018_6198, pass1_1018_5b06};
use crate::struct_ops::struct_1010::struct_1010_3c52;
use crate::mixed::mixed_1010_20ba;
use crate::draw::draw_1018::{get_dc_1018_4db0, create_dc_1018_4e04};
use crate::cleanup::unk_destroy_window_op_1018_6bb6;
use crate::mci::mci_send_command_1008_53ae;
use crate::defines::{Struct28, Struct76};
use crate::pass::pass_1000::pass1_1000_3cea;
use crate::pass::pass_1028::{pass1_1028_e4ec, pass1_1028_dc52};
use crate::ui::ui_1040::dialog_ui_fn_1040_78e2;
use crate::struct_ops::struct_1008::struct_1008_4c58;

pub fn
win_op_1018_294a(param_1: i16,param_2: u16,param_3: u16,param_4: i32,param_5: u16,
                in_string_6: &String)

{
  if (((param_1 + 0x18) != 0x0) && (param_4._2_2_ == 0x280)) {
    (param_1 + 0x18) = 0x0;
  }
  create_dc_1018_4e04((astruct_8 **)CONCAT22(param_2,param_1),param_3,param_4,
                      param_4._2_2_,in_string_6,param_5);
  return;
}


pub fn unk_win_ui_op_1018_4f18(param_1: &mut Struct39,param_2: u16,param_3: u32)
{
  let ppcVar1: u32;
  let puVar2: u32;
  let rect: *mut RECT16;
  let iVar3: i16;
  let uVar4: u32;
  let extraout_DX: *mut u8;
  let puVar5: *mut u8;
  let extraout_DX_00: *mut u8;
  let puVar6: *mut u8;
  let uVar7: u16;
  let iVar6: &mut Struct39;
  let uVar8: u16;
  let uVar9: u16;
  let unaff_SS: u16;
  let paStack22: &mut Struct76;
  let local_12: RECT16;
  let iStack14: i16;
  let iStack12: i16;
  let uStack10: u32;
  let paStack6: &mut Struct43;
  
  paStack6 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,param_2,unaff_SS);
  uVar4 = paStack6 & 0xffff;
  ppcVar1 = (paStack6 + 0x14);
  (**ppcVar1)(0x1010,uVar4,(paStack6 >> 0x10));
  puVar2 = uVar4;
  uStack10 = uVar4 & 0xffff | ZEXT24(extraout_DX) << 0x10;
 // uVar8 = (param_1 >> 0x10);
  iVar6 = param_1;
  puVar5 = extraout_DX;
  if (&iVar6.field_0xe != 0x0) {
    uVar7 = iVar6.field_0x10;
    puVar2 = &iVar6.field_0xe;
    puVar5 = (uVar7 | puVar2);
    if (puVar5 != 0x0) {
      ppcVar1 = *puVar2;
      (**ppcVar1)();
      puVar5 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar5,0x1000);
  puVar6 = (puVar5 | puVar2);
  if (puVar6 == 0x0) {
    puVar2 = 0x0;
    puVar6 = 0x0;
  }
  else {
    struct_1008_4c58(CONCAT22(puVar5,puVar2));
  }
  iVar6.field_0xe = puVar2;
  iVar6.field_0x10 = puVar6;
  pass1_1008_4d84(&iVar6.field_0xe,uStack10,puVar6);
  rect = &local_12;
  GetClientRect16(0x1008,rect);
  uVar9 = 0x1000;
  mem_op_1000_179c(0x1e,puVar6,0x1000);
  paStack22 = CONCAT22(puVar6,rect);
  uVar7 = puVar6 | rect;
  if (uVar7 == 0x0) {
    &iVar6.field_0xa = 0x0;
  }
  else {
    iVar3 = (iStack12 - local_12.y) + 0x1;
    uVar9 = 0x1008;
    pass1_1008_405c(paStack22,&iVar6.field_0xe,iVar3,
                    (iStack14 - local_12.x) + 0x1);
    iVar6.field_0xa = iVar3;
    iVar6.field_0xc = uVar7;
  }
  if (paStack6 != 0x0) {
    ppcVar1 = paStack6;
    (**ppcVar1)(uVar9,paStack6,(paStack6 >> 0x10),0x1);
  }
  return;
}


pub fn win_1018_598c(astruct *param_1,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  let iVar1: &mut Struct131;
  let uVar2: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  get_dc_1018_4db0(iVar1.field_0xf2,iVar1.field_0x8,0x1008);
  mem_op_1000_179c(0x2a,param_3,0x1000);
  uVar1 = param_3 | param_2;
  if (uVar1 != 0x0) {
    pass1_1018_5b06(param_2,param_3,iVar1.field_0x8,unaff_SS);
    iVar1.field_0xee = param_2;
    iVar1.field_0xf0 = uVar1;
    return;
  }
  &iVar1.field_0xee = 0x0;
  return;
}


pub fn win_ui_op_1018_5e9a(param_1: &mut Struct1,param_2: u16)
{
  let uVar1: u32;
  let pUVar2: &mut u32;
  let IVar3: i16;
  let puVar4: *mut u8;
  let in_DX: *mut u8;
  let puVar5: *mut u8;
  let puVar6: *mut u8;
  let uVar7: u16;
  let uVar8: u16;
  let iVar9: i16;
  let unaff_DI: i16;
  let uVar10: u16;
  let puVar11: *mut u16;
  let local_28: [u8;12];
  let iStack22: i16;
  let uStack20: u16;
  let iStack18: i16;
  let iStack16: i16;
  let local_e: RECT16;
  let iStack8: i16;
  INT16 *pIStack6;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  puVar11 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x39,param_2,in_DX,unaff_DI);
 // puVar5 = (puVar11 >> 0x10);
  uVar7 = puVar11;
 // uVar10 = (param_1 >> 0x10);
  iVar9 = param_1;
  (iVar9 + 0x8e) = uVar7;
  (iVar9 + 0x90) = puVar5;
  mem_op_1000_179c(0x12,puVar5,0x1000);
  puVar6 = (puVar5 | uVar7);
  if (puVar6 == 0x0) {
    (iVar9 + 0x92) = 0x0;
  }
  else {
    pass1_1018_6198(CONCAT22(puVar5,uVar7),param_1,
                    (iVar9 + 0x6),puVar6,unaff_DI,param_2);
    (iVar9 + 0x92) = uVar7;
    (iVar9 + 0x94) = puVar6;
  }
  uVar1 = (iVar9 + 0x8e);
  pIStack6 = (INT16 *)(uVar1 & 0xffff0000 | (uVar1 + 0xa));
  GetClientRect16(0x1000,&local_e);
  IVar3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
  (pIStack6 + 0x6) = IVar3 + iStack8;
  puVar11 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x48,param_2,puVar6,unaff_DI);
 // uStack20 = (puVar11 >> 0x10);
  iStack22 = puVar11;
  iStack16 = (iStack22 + 0xa);
  iStack18 = (iStack22 + 0xc);
 // uVar10 = (pIStack6 >> 0x10);
  (pIStack6 + 0x2) = (iStack18 - (pIStack6 + 0x6)) / 0x2;
  uVar7 = iStack16 >> 0xf;
  *pIStack6 = iStack16 / 0x2 + 0x3;
  pass1_1028_dc52(CONCAT22(param_2,local_28),0x1,0x0,0x100);
  loop {
    puVar4 = local_28;
    pass1_1028_e4ec(CONCAT22(param_2,puVar4));
    uVar8 = uVar7 | puVar4;
    if (uVar8 == 0x0) { break; }
    pUVar2 = (puVar4 + 0x10);
    uVar7 = uVar8;
    if (pUVar2 != (ULONG *)0x0) {
      pass1_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10),*pUVar2);
      uVar7 = uVar8;
    }
  }
 // uVar10 = (pIStack6 >> 0x10);
  iVar9 = pIStack6;
  SetWindowPos16(&USHORT_1050_1028,0x44,(iVar9 + 0x6),
                 (iVar9 + 0x4),(iVar9 + 0x2),*pIStack6,0x0);
  return;
}


pub fn
set_window_text_1018_6066
          (param_1: u16,param_2: u16,in_win_text_3: SEGPTR,param_4: u16,
          dialog_id_5: i16,in_hwnd_6: HWND16)

{
  GetDlgItem16(in_hwnd_6,dialog_id_5);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,in_win_text_3);
  return;
}



pub fn set_window_text_1018_6086(param_1: i32,param_2: &mut String,param_3: *mut u16)
{
  let HStack8: HWND16;
  
  wsprintf16(param_2,&stack0xfff4,param_3);
  GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1be);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,&stack0xfff4);
  wsprintf16(s_tile2_bmp_1050_1538,&stack0xfff4,param_3);
  HStack8 = (param_1 + 0x6);
  HStack8 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1bf);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,&stack0xfff4);
  return;
}


pub fn window_op_1018_67b6(astruct *param_1)
{
  let in_AX: &mut Struct658;
  let in_DX: *mut u8;
  let uVar1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0((iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_6924(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    (iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


pub fn mix_ui_op_1018_6adc(param_1: &mut Struct28)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let in_DX: *mut u8;
  let uVar4: u16;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  unaff_SS: &WNDCLASS16;
  let puVar7: *mut u16;
  let paVar8: &mut Struct43;
  
  puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
 // uVar4 = (puVar7 >> 0x10);
  iVar1 = (puVar7 + 0xa);
  iVar2 = (puVar7 + 0xc);
 // uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if (0x1 < iVar1 - (iVar5 + 0xe6)) {
    uVar4 = iVar1 >> 0xf;
    (iVar5 + 0xe2) = iVar1 / 0x2 - ((iVar5 + 0xe6) + 0x1) / 0x2;
  }
  if (0x1 < iVar2 - (iVar5 + 0xe8)) {
    uVar4 = iVar2 >> 0xf;
    (iVar5 + 0xe4) = iVar2 / 0x2 - ((iVar5 + 0xe8) + 0x1) / 0x2;
  }
  uVar3 = ShowCursor16(0x1010);
  if ((iVar5 + 0xee) != 0x0) {
    win_1008_5c5c(unaff_SS,uVar3,uVar4,_PTR_LOOP_1050_02a0,(iVar5 + 0xee));
    (iVar5 + 0xf0) = uVar3;
  }
  paVar8 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,(iVar5 + 0xec),
                               unaff_SS);
  mci_send_command_1008_53ae
            (paVar8,(iVar5 + 0x8),0x1008,unaff_SS);
  ShowCursor16(0x1008);
  unk_destroy_window_op_1018_6bb6(param_1,s_tile2_bmp_1050_1538);
  return;
}


pub fn win_1018_df40(astruct *param_1,param_2: u16,param_3: *mut u8,param_4: u16)
{
  let iVar1: &mut Struct267;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0xa,param_3,0x1000);
  iVar1 = param_1;
 // uVar1 = (param_1 >> 0x10);
  if ((param_3 | param_2) != 0x0) {
    puVar2 = struct_1018_e100(CONCAT22(param_3,param_2),iVar1.field_0x8,
                              (param_3 | param_2),param_4);
    iVar1.field_0xe2 = puVar2;
    iVar1.field_0xe4 = (puVar2 >> 0x10);
    return;
  }
  &iVar1.field_0xe2 = 0x0;
  return;
}


pub fn window_op_1018_e384(astruct *param_1)
{
  let in_AX: &mut Struct659;
  let in_DX: *mut u8;
  let uVar1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0((iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_e4f2(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    (iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


pub fn window_op_1018_e6c6(astruct *param_1)
{
  let in_AX: &mut Struct660;
  let in_DX: *mut u8;
  let uVar1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0((iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_e834(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    (iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


pub fn post_win_msg_1018_ea0a(param_1: u16,param_2: u16,param_3: i16,param_4: HWND16)
{
  if (param_3 == 0xed) {
    PostMessage16(param_4,0x0,0x0,0x11100c6);
  }
  return;
}


pub fn window_op_1018_eada(astruct *param_1)
{
  let in_AX: &mut Struct661;
  let in_DX: *mut u8;
  let uVar1: u16;
  let iVar2: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0((iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x28,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_ec74(in_AX,in_DX,(iVar2 + 0x8),unaff_SS);
    (iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


pub fn win_1020_0316(astruct *param_1,param_2: *mut u8,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8;
  let puVar4: *mut u8;
  let iVar1: &mut Struct273;
  let unaff_DI: i16;
  let uVar5: u16;
  let puVar6: *mut u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x1,param_3,param_2,unaff_DI);
 // puVar3 = (puVar6 >> 0x10);
 // uVar5 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field_0xe2 = puVar6;
  iVar1.field_0xe4 = puVar3;
  uVar1 = &iVar1.field_0xe2;
  (uVar1 + 0x16) = iVar1.field_0xea;
  uVar2 = iVar1.field_0xee;
  uVar1 = &iVar1.field_0xe2;
  (uVar1 + 0x12) = uVar2;
  struct_1010_3c52(&iVar1.field_0xe2,iVar1.field_0xec,param_3);
  mem_op_1000_179c(0x12,puVar3,0x1000);
  puVar4 = (puVar3 | uVar2);
  if (puVar4 != 0x0) {
    pass1_1020_04f6(CONCAT22(puVar3,uVar2),iVar1.field_0x8,puVar4,unaff_DI,
                    param_3);
    iVar1.field_0xe6 = uVar2;
    iVar1.field_0xe8 = puVar4;
    return;
  }
  &iVar1.field_0xe6 = 0x0;
  return;
}



pub fn post_msg_1020_03b2(param_1: u32,param_2: HWND16)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}



pub fn post_msg_1020_03d6(param_1: u32,param_2: HWND16)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}



pub fn post_msg_1020_03fa(param_1: u32,param_2: HWND16)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}


pub fn post_win_msg_1020_061c(param_1: u32,param_2: i16,param_3: HWND16)
{
  let uVar1: u32;
  let uVar2: u16;
  
 // uVar2 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  uVar1 = (param_1 + 0x6);
  PostMessage16(param_3,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}


pub fn send_win_msg_1020_08fe(param_1: &mut Struct63,param_2: HWND16)
{
  let BVar1: bool;
  let iVar2: &mut Struct63;
  let uVar2: &mut Struct63;
  
 // uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1 = 0xb0e;
  iVar2.field_0x2 = 0x1020;
  if (iVar2.field_0xe8 != 0x0) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 != 0x0) {
      SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1110001);
    }
    iVar2.field_0xe8 = 0x0;
  }
  pass1_1008_57c4(
                  (param_1 & 0xffff0000 | &iVar2.field_0xd2));
  param_1 = 0x380a;
  iVar2.field_0x2 = 0x1008;
  param_1 = 0x389a;
  iVar2.field_0x2 = 0x1008;
  return;
}



pub fn send_msg_1020_097e(param_1: u32,param_2: HWND16)
{
  let iVar1: i16;
  let uVar2: u16;
  
 // uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((iVar1 + 0xea) | (iVar1 + 0xe8)) != 0x0) {
    SendMessage16(param_2,0x0,0x0,0x1110001);
    (iVar1 + 0xe8) = 0x0;
  }
  return;
}



pub fn win_1020_09ba(astruct *param_1,param_2: u16,param_3: *mut u8,param_4: u16)
{
  let puVar1: *mut u8;
  let iVar1: &mut Struct275;
  let uVar2: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0xe,param_3,0x1000);
  puVar1 = (param_3 | param_2);
  iVar1 = param_1;
 // uVar2 = (param_1 >> 0x10);
  if (puVar1 != 0x0) {
    struct_1020_0baa(CONCAT22(param_3,param_2),iVar1.field_0x8,puVar1,param_4);
    iVar1.field_0xe2 = param_2;
    iVar1.field_0xe4 = puVar1;
    return;
  }
  &iVar1.field_0xe2 = 0x0;
  return;
}
