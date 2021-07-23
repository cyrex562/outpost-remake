

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

use std::default::default;

use crate::defines::Struct18;
use crate::draw::draw_1018::get_dc_1018_4db0;
use crate::draw::draw_1020::{invalidate_rect_1020_735a, load_draw_op_1020_2ede, pt_in_rect_1020_5856, unk_draw_op_1020_3da4};
use crate::fn_ptr::fn_ptr_1000::{call_fn_ptr_1008_3e0e, fn_ptr_1000_17ce};
use crate::mem_1000::mem_op_1000_179c;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::pass1_1000_472c;
use crate::pass::pass_1008::{pass1_1008_3e54, pass1_1008_4d84, pass1_1008_5118, pass1_1008_5236, pass1_1008_6978, pass1_1008_8ce4, pass1_1008_941a, pass1_1008_9436};
use crate::pass::pass_1010::{pass1_1010_089e, pass1_1010_1ea6, pass1_1010_375e, pass1_1010_3770, pass1_1010_41d6, pass1_1010_451a, pass1_1010_459e, pass1_1010_4674, pass1_1010_4788, pass1_1010_4df0, pass1_1010_65d0};
use crate::pass::pass_1018::{pass1_1018_04b8, pass1_1018_0ac0, pass1_1018_0ad4, pass1_1018_0afa, pass1_1018_0b08, pass1_1018_1662, pass1_1018_169e, pass1_1018_1a8e, pass1_1018_1e78, pass1_1018_1f1a, pass1_1018_255e, pass1_1018_2afa, pass1_1018_2d22, pass1_1018_2d84, pass1_1018_2e5e, pass1_1018_30fc, pass1_1018_57e6};
use crate::pass::pass_1020::{pass1_1020_0dc4, pass1_1020_1da8, pass1_1020_289a, pass1_1020_62e0, pass1_1020_6498, pass1_1020_64d4, pass1_1020_6746, pass1_1020_68de, pass1_1020_bd80};
use crate::pass::pass_1028::{pass1_1028_dc52, pass1_1028_e4ec};
use crate::string::string_1000::{string_1000_3d3e, string_1000_3cea};
use crate::string::string_1010::load_string_1010_847e;
use crate::string::string_1040::string_1040_8520;
use crate::struct_ops::struct_1008::clear_struct_1008_3e38;
use crate::struct_ops::struct_1020::struct_1020_3644;
use crate::sys_api::get_sys_metrics_1020_7c1a;
use crate::ui::ui_1008::create_window_ex_1008_9760;
use crate::ui::ui_1040::dialog_ui_fn_1040_78e2;
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};
use crate::win_struct::{HCURSOR16, HDC16, HGDIOBJ16, HMENU16, HPALETTE16, HWND16, PAINTSTRUCT16, POINT16, RECT16, SEGPTR, WNDCLASS16};
use crate::winapi::{BeginPaint16, BringWindowToTop16, ClientToScreen16, CreateWindow16, DeleteObject16, DestroyWindow16, DrawIcon16, EnableMenuItem16, EndPaint16, EnumChildWindows1, FillRect16, FreeProcInstance16, GetClientRect16, GetDC16, GetDlgItem16, GetStockObject16, GetSubMenu16, GetSystemMetrics16, GetWindowRect16, GetWindowText16, InvalidateRect16, IsIconic16, LoadMenu16, MakeProcInstance16, MoveWindow16, PostMessage16, PtInRect16, RealizePalette16, ReleaseCapture16, SelectPalette16, SendMessage16, SetCursor16, SetDlgItemText16, SetFocus16, SetWindowPos16, SetWindowText16, ShowWindow16, TrackPopupMenu16, UnrealizeObject16, UpdateWindow16};

pub fn win_help_op_1020_0ec4(param_1: U32Ptr, param_2: u16, param_3: u16)
{
  let ppcVar1: u32;
  let cVar2: u8;
  let uVar3: u16;
  let in_DX: U32Ptr;
  let u_var4: u16;
  let unaff_DI: i16;
  let puVar5: U32Ptr;
  let uVar6: u32;
  let paVar7: &mut Struct43;
  let uVar8: u16;
  let uVar9: u16;
  let iVar10: i16;
  
 // uVar8 = (param_1 >> 0x10);
  uVar3 = param_1;
  if (param_2 == 0xfb) {
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x30,param_3,in_DX,unaff_DI);
    pass1_1010_375e(puVar5);
    ppcVar1 = (*param_1 + 0x14);
    (**ppcVar1)();
    uVar6 = pass1_1010_375e(puVar5);
   // u_var4 = (uVar6 >> 0x10);
    pass1_1010_4788((uVar3 + 0xf2),
                    (uVar6 & 0xffff | u_var4 << 0x10),uVar6,u_var4);
    return;
  }
  if (param_2 < 0xfc) {
    if (param_2 == 0xf3) {
      iVar10 = 0x3;
    }
    else {
      if (0xf3 < param_2) {
        return;
      }
      cVar2 = param_2;
      if (cVar2 == 'o') {
        paVar7 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x1f8,param_3);
        WinHelp16(0x1010,(s_New_failed_in_Op__Op_1050_0020 + 0x8),0x0,
                  CONCAT22(paVar7,0x1));
        return;
      }
      if (cVar2 == 'r') {
        iVar10 = uVar3 + 0xa;
        uVar9 = uVar8;
        puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x30,param_3,in_DX,unaff_DI);
       // u_var4 = (puVar5 >> 0x10);
        pass1_1010_3770(puVar5,CONCAT22(uVar9,iVar10),u_var4);
        pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c,(uVar3 + 0x8),0x3,u_var4,uVar3,
                        &ctx.PTR_LOOP_1050_1038,param_3);
        return;
      }
      if (cVar2 == -0xf) {
        iVar10 = 0x1;
      }
      else {
        if (cVar2 != -0xe) {
          return;
        }
        iVar10 = 0x2;
      }
    }
    pass1_1010_4674((uVar3 + 0xf2),iVar10,0x1010,param_3);
    return;
  }
  if (true) {
    switch(param_2) {
    default:
//       TODO: goto switchD_1020_0feb_caseD_129;
    0x12a =>
      uVar8 = 0xf012;
      break;
    0x12c =>
      uVar8 = 0xf020;
    }
    PostMessage16(0x1020,0x0,0x0,CONCAT22(0x112,uVar8));
    return;
  }
switchD_1020_0feb_caseD_129:
  return;
}


pub fn enable_menu_1020_1000(HMENparam_1: u16,param_2: i16)
{
  if (param_2 != 0x0) {
    return;
  }
  EnableMenuItem16(param_1,0x400,0x3);
  return;
}


pub fn window_op_1020_10a0(astruct *param_1)
{
  let u_var1: u32;
  let ppcVar2: u32;
  let in_AX: &mut Struct160;
  let uVar3: u16;
  bool *pBVar4;
  let in_DX: U32Ptr;
  let puVar5: U32Ptr;
  let puVar6: U32Ptr;
  let extraout_dx: U32Ptr;
  let extraout_DX_00: u16;
  let unaff_DI: i16;
  let unaff_SS: u16;
  let in_AF: u8;
  let puVar7: U32Ptr;
  let uVar8: u32;
  let uVar9: u16;
  let puVar10: U32Ptr;
  let iVar11: i16;
  let uVar12: u16;
  
  iVar11 = param_1;
 // uVar12 = (param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0x42,in_DX,0x1000);
  puVar5 = (in_DX | in_AX);
  if (puVar5 != 0x0) {
    pass1_1008_3bd6(in_AX,in_DX,0x0,0x1f009b,0x0,0x740075,
                    CONCAT22((iVar11 + 0x8),0xf1),puVar5,unaff_SS);
  }
  mem_op_1000_179c(0x42,puVar5,0x1000);
  puVar6 = (puVar5 | in_AX);
  if (puVar6 != 0x0) {
    pass1_1008_3bd6(in_AX,puVar5,0x0,0x31009b,0x0,0x760077,
                    CONCAT22((iVar11 + 0x8),0xf2),puVar6,unaff_SS);
  }
  mem_op_1000_179c(0x42,puVar6,0x1000);
  puVar5 = (puVar6 | in_AX);
  if (puVar5 != 0x0) {
    pass1_1008_3bd6(in_AX,puVar6,0x0,0x77009b,0x0,0x780079,
                    CONCAT22((iVar11 + 0x8),0xf3),puVar5,unaff_SS);
  }
  puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2d,unaff_SS,puVar5,unaff_DI);
 // uVar9 = (puVar7 >> 0x10);
  (iVar11 + 0xf2) = puVar7;
  (iVar11 + 0xf4) = uVar9;
  (iVar11 + 0xe0) = (iVar11 + 0xf2);
  (iVar11 + 0xe2) = uVar9;
  puVar10 = ctx.PTR_LOOP_1050_038c;
  uVar3 = LoadIcon16(0x1010,s_PLNTICON_1050_4267);
  (iVar11 + 0xc2) = uVar3;
  u_var1 = (iVar11 + 0xf2);
  ppcVar2 = ((iVar11 + 0xf2) + 0x30);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,u_var1,(u_var1 >> 0x10),uVar3,
              puVar10);
  puVar5 = extraout_dx;
  mem_op_1000_179c(0x24,extraout_dx,0x1000);
  puVar6 = (puVar5 | uVar3);
  if (puVar6 == 0x0) {
    (iVar11 + 0xf6) = 0x0;
  }
  else {
    unk_win_ui_op_1020_1418(CONCAT22(puVar5,uVar3),param_1,unaff_SS);
    (iVar11 + 0xf6) = uVar3;
    (iVar11 + 0xf8) = puVar6;
  }
  (iVar11 + 0xe8) = (iVar11 + 0xf6);
  puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2f,unaff_SS,puVar6,unaff_DI);
  uVar8 = pass1_1018_04b8(puVar7);
 // puVar5 = (uVar8 >> 0x10);
  pass1_1010_41d6((iVar11 + 0xf2),uVar8,puVar5,unaff_SS,in_AF);
  uVar8 = pass1_1010_451a((iVar11 + 0xf2),puVar5,unaff_DI,unaff_SS);
  pBVar4 = (bool *)uVar8;
  u_var1 = param_1;
  ppcVar2 = (u_var1 + 0x14);
  (**ppcVar2)(0x1010,iVar11,uVar12,0x0,pBVar4,(uVar8 >> 0x10));
  uVar9 = 0x1;
  ppcVar2 = (u_var1 + 0x10);
  (**ppcVar2)();
  pass1_1010_459e((iVar11 + 0xf2));
  ppcVar2 = ((iVar11 + 0xf2) + 0x10);
  (**ppcVar2)(0x1010,(iVar11 + 0xf2),param_1,uVar9);
  MoveWindow16(0x1010,0x1,pBVar4[0x3],pBVar4[0x2],pBVar4[0x1],*pBVar4);
  UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
  return;
}


pub fn
win_ui_cursor_op_1020_1294
          (param_1: u32,param_2: i16,param_3: i16,param_4: u16,param_5: u16)

{
  let ppcVar1: u32;
  let in_AX: u16;
  let HVar2: HCURSOR16;
  let HVar3: HCURSOR16;
  let i_var4: i16;
  let u_var5: u16;
  let uVar6: u32;
  let local_12: i16;
  let local_10: i16;
  let puStack14: U32Ptr;
  let puStack10: u32;
  let local_6: i16;
  let iStack4: i16;
  
  pass1_1030_8344(ctx.PTR__LOOP_1050_5748,(ctx.PTR__LOOP_1050_5748 >> 0x10)
                  ,0x4000001);
  if ((param_4 | in_AX) == 0x0) {
    local_6 = param_3;
    iStack4 = param_2;
   // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    puStack10 = pass1_1010_40cc((i_var4 + 0xf2),param_2,0x0);
    uVar6 = (i_var4 + 0xf2);
    puStack14 = (uVar6 & 0xffff0000 | (uVar6 + 0x76));
    pass1_1008_3e94(puStack14,CONCAT22(param_5,&local_12),
                    CONCAT22(param_5,&local_10));
    local_6 -= local_10;
    iStack4 -= local_12;
    i_var4 = pt_in_rect_1010_40f8
                      ((i_var4 + 0xf2),CONCAT22(param_5,&local_6),
                       0x1010);
    if (i_var4 != -0x1) {
      uVar6 = 0x0;
      HVar2 = LoadCursor16(0x1010,0x7f02);
      uVar6 = uVar6 & 0xffff0000 | HVar2;
      HVar3 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
      ppcVar1 = (*puStack10 + 0x4);
      (**ppcVar1)(ctx.s_tile2_bmp_1050_1538,puStack10,
                  (puStack10 >> 0x10),i_var4,i_var4 >> 0xf,i_var4,0x2,uVar6,HVar3
                  ,HVar2);
      call_fn_ptr_1008_3e0e(param_1);
      SetCursor16(0x1008);
    }
  }
  return;
}


pub fn unk_win_ui_op_1020_1418(param_1: &mut Struct40,param_2: i32,param_3: u16)
{
  let u_var1: u32;
  let paVar2: &mut Struct13;
  let ppc_var3: u32;
  HDC16 *pHVar4;
  let puVar5: u32;
  let puVar6: U32Ptr;
  let extraout_dx: U32Ptr;
  let iVar5: &mut Struct40;
  let unaff_DI: i16;
  let uVar7: u16;
  let unaff_CS: u16;
  let puVar8: U32Ptr;
  let local_8: HDC16;
  let puStack6: U32Ptr;
  
  get_sys_metrics_1020_7c1a(param_1,param_2,unaff_CS);
 // uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  &iVar5.field_0x14 = 0x0;
  iVar5.field_0x18 = 0x0;
  puVar8 = clear_struct_1008_3e38(
                           (param_1 & 0xffff0000 | &iVar5.field_0x1e)
                          );
  param_1 = 0x1730;
  iVar5.field_0x2 = 0x1020;
  puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2d,param_3,
                           (puVar8 >> 0x10),unaff_DI);
 // puVar6 = (puVar8 >> 0x10);
  iVar5.field_0x14 = puVar8;
  &iVar5.field_0x16 = puVar6;
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x29,param_3,puVar6,unaff_DI);
  u_var1 = &iVar5.field_0x14;
  ppc_var3 = (*&iVar5.field_0x14 + 0x4);
  (**ppc_var3)(0x1010,u_var1,(u_var1 >> 0x10),0x0,param_1);
  local_8 = GetDC16(0x1010);
  u_var1 = &iVar5.field_0x14;
  *(u_var1 + 0x7c) = local_8;
  u_var1 = &iVar5.field_0x14;
  puVar5 = (u_var1 + 0x66);
  iVar5.field_0x18 = puVar5;
  ppc_var3 = (*iVar5.field_0x18 + 0x14);
  (**ppc_var3)();
  paVar2 = (puStack6 + 0xe);
  puVar6 = extraout_dx;
  pass1_1008_4d84((puVar5 & 0xffff | ZEXT24(extraout_dx) << 0x10),
                  paVar2,extraout_dx);
  pHVar4 = palette_op_1008_4e08(paVar2,&local_8,puVar6,0x1008);
  iVar5.field_0x1c = pHVar4;
  return;
}



pub fn win_ui_op_1020_150e(param_1: U32Ptr,param_2: HDC16)
{
  let HVar1: HPALETTE16;
  let iVar2: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  *param_1 = 0x1730;
  (iVar2 + 0x2) = 0x1020;
  if ((iVar2 + 0x14) != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6((iVar2 + 0x14),
                    param_1 & 0xffff | uVar3 << 0x10,unaff_SS);
  }
  HVar1 = SelectPalette16(param_2,0x0,(iVar2 + 0x1c));
  (iVar2 + 0x1c) = HVar1;
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  *param_1 = 0x3ab0;
  (iVar2 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar2 + 0x2) = 0x1008;
  return;
}


pub fn mixed_ui_op_1020_179c(param_1: &mut Struct1)
{
  let u_var1: u32;
  let ppcVar2: u32;
  let uVar3: u32;
  let u_var4: u16;
  let IVar5: i16;
  let puVar6: U32Ptr;
  let in_DX: U32Ptr;
  let extraout_dx: U32Ptr;
  let puVar7: U32Ptr;
  let uVar8: u16;
  let iVar9: i16;
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  unaff_SS: &WNDCLASS16;
  let puVar14: U32Ptr;
  in_resc_id_3: &WNDCLASS16;
  in_buffer_4: &WNDCLASS16;
  WNDCLASS16 local_178 [0xc];
  let uStack118: u32;
  let uStack114: u32;
  let local_6e: RECT16;
  let uStack106: u32;
  let HStack102: HWND16;
  let iStack98: i16;
  let iStack94: i16;
  let uStack78: u16;
  let puStack76: U32Ptr;
  let uStack74: u32;
  let HStack70: HWND16;
  let uStack68: u32;
  let uStack64: u32;
  pvStack60: U32Ptr;
  let uStack58: u16;
  let uStack56: u16;
  let pUStack54: &mut u32;
  let uStack50: u32;
  let local_2e: [u8;12];
  let local_1c: RECT16;
  let uStack24: u32;
  let iStack20: i16;
  let iStack18: i16;
  let puStack16: U32Ptr;
  INT16 *pIStack12;
  let uStack8: u16;
  let puStack6: U32Ptr;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  u_var4 = 0x89;
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x9,unaff_SS,in_DX,unaff_DI);
 // puVar7 = (puStack6 >> 0x10);
  u_var4 = pass1_1010_65d0(unaff_SS,puStack6,u_var4);
  uStack8 = (u_var4 == 0x0);
  u_var4 = pass1_1010_65d0(unaff_SS,puStack6,0x86);
  if (u_var4 != 0x0) {
    uStack8 = 0x0;
  }
  puVar14 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x39,unaff_SS,puVar7,unaff_DI);
 // uVar12 = (puVar14 >> 0x10);
  uVar8 = puVar14;
 // uVar11 = (param_1 >> 0x10);
  iVar9 = param_1;
  (iVar9 + 0x8e) = uVar8;
  (iVar9 + 0x90) = uVar12;
  ppcVar2 = ((iVar9 + 0x8e) + 0x10);
  (**ppcVar2)(0x1010,(iVar9 + 0x8e),uVar12,uStack8);
  puStack76 = extraout_dx;
  mem_op_1000_179c(0x12,extraout_dx,0x1000);
  puVar7 = (puStack76 | uVar8);
  uStack78 = uVar8;
  if (puVar7 == 0x0) {
    (iVar9 + 0x92) = 0x0;
  }
  else {
    pass1_1020_1eea(CONCAT22(puStack76,uVar8),param_1,
                    (iVar9 + 0x6),puVar7,unaff_DI,unaff_SS);
    (iVar9 + 0x92) = uVar8;
    (iVar9 + 0x94) = puVar7;
  }
  u_var1 = (iVar9 + 0x8e);
  pIStack12 = (INT16 *)(u_var1 & 0xffff0000 | (u_var1 + 0xa));
  puStack16 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x48,unaff_SS,puVar7,unaff_DI);
  GetClientRect16(0x1010,&local_1c);
  IVar5 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
 // uVar12 = (pIStack12 >> 0x10);
  iVar10 = pIStack12;
  (iVar10 + 0x6) = IVar5 + uStack24._2_2_;
 // uVar13 = (puStack16 >> 0x10);
  iStack18 = (puStack16 + 0xa);
  iStack20 = (puStack16 + 0xc);
  (iVar10 + 0x2) = (iStack20 - (iVar10 + 0x6)) / 0x2;
  iVar10 = iStack18 - (iVar10 + 0x4);
  uVar8 = iVar10 >> 0xf;
  *pIStack12 = iVar10 / 0x2;
  pass1_1028_dc52(CONCAT22(unaff_SS,local_2e),0x1,0x0,0x100);
  uStack56 = 0x0;
  loop {
    puVar6 = local_2e;
    pass1_1028_e4ec(CONCAT22(unaff_SS,puVar6));
    uStack50 = CONCAT22(uVar8,puVar6);
    uStack58 = uVar8 | puVar6;
    if (uStack58 == 0x0) { break; }
    pUStack54 = (puVar6 + 0x10);
    uVar8 = uStack58;
    if (pUStack54 != (ULONG *)0x0) {
      string_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10), *pUStack54);
      uVar8 = uStack58;
    }
  }
  u_var4 = pass1_1020_1da8(param_1,puVar6,0x0,unaff_SS);
  (iVar9 + 0x96) = u_var4;
  u_var4 = pass1_1010_65d0(unaff_SS,puStack6,0x86);
  if ((u_var4 == 0x0) || ((iVar9 + 0x96) != 0x0)) {
    uVar3 = (iVar9 + 0x8e);
    (uVar3 + 0x2c) = 0x0;
    HStack102 = GetDlgItem16(0x1010,0x175);
    if (uStack8 != 0x0) {
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_178,
                 (short)unaff_SS);
      SetWindowText16(0x1010,local_178);
    }
    pvStack60 = MakeProcInstance16(s_tile2_bmp_1050_1538,
                                   (HANDLE16)PTR_LOOP_1050_038c);
    GetWindowRect16(ctx.s_tile2_bmp_1050_1538,&local_6e);
    uStack114 = uStack106;
    iStack98 = uStack106 - local_6e.x;
    iStack94 = uStack106._2_2_ - local_6e.y;
    uStack118 = local_6e & 0xffff0000 |
                (-(iStack98 - (pIStack12 + 0x4)) / 0x2);
    IVar5 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    u_var1 = uStack118 & 0xffff;
    uStack118 = u_var1 | (uStack118._2_2_ - IVar5) << 0x10;
    uStack118._0_2_ = u_var1;
    MoveWindow16(ctx.s_tile2_bmp_1050_1538,0x0,iStack94,iStack98,
                 uStack118._2_2_ - IVar5,uStack118);
  }
  else {
    win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x9d0001,unaff_SS,u_var4,uStack58);
    (iVar9 + 0x8c) = u_var4;
    pvStack60 = MakeProcInstance16(0x1008,(HANDLE16)PTR_LOOP_1050_038c);
  }
  EnumChildWindows1(ctx.s_tile2_bmp_1050_1538,0x0,ZEXT24(pvStack60) << 0x10);
  FreeProcInstance16(s_tile2_bmp_1050_1538);
  HStack70 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1);
  GetWindowRect16(ctx.s_tile2_bmp_1050_1538,&local_1c);
  uStack64 = uStack24;
  local_1c.x = uStack24 - local_1c.x;
  uStack74 = CONCAT22(local_1c.x,uStack24._2_2_ - local_1c.y);
  uStack68 = local_1c & 0xffff0000 |
             (-(local_1c.x - (pIStack12 + 0x4)) / 0x2);
  IVar5 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
  uStack68 = uStack68 & 0xffff | (uStack68._2_2_ - IVar5) << 0x10;
  if ((iVar9 + 0x96) == 0x0) {
    if (uStack8 == 0x0) {
        // goto
        // LAB_1020_1b24;
    }
    in_buffer_4 = local_178;
    in_resc_id_3 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21);
  }
  else {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_178,
               (short)unaff_SS);
    GetDlgItem16(0x1010,0x175);
    SetWindowText16(ctx.s_tile2_bmp_1050_1538,local_178);
    in_resc_id_3 = local_178;
    in_buffer_4 = unaff_SS;
    unaff_SS = 0x3fe;
  }
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),in_resc_id_3,
             in_buffer_4,(short)unaff_SS);
  SetWindowText16(0x1010,local_178);
//LAB_1020_1b24:
  MoveWindow16(ctx.s_tile2_bmp_1050_1538,0x0,uStack74,
               (uStack74 >> 0x10),uStack68._2_2_,uStack68);
 // uVar12 = (pIStack12 >> 0x10);
  iVar9 = pIStack12;
  SetWindowPos16(ctx.s_tile2_bmp_1050_1538,0x44,(iVar9 + 0x6),
                 (iVar9 + 0x4),(iVar9 + 0x2),*pIStack12,0x0);
  return;
}


pub fn
enable_window_1020_1bd4
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: HWND16)

{
  let ppcVar1: u32;
  let bVar2: bool;
  let in_AX: u16;
  let iVar3: i16;
  let in_DX: U32Ptr;
  let puVar4: U32Ptr;
  let u_var5: u16;
  let unaff_SS: u16;
  let puStack12: u32;
  
  bVar2 = false;
  pass1_1020_1d8e(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3));
  if (in_AX != 0x0) {
    if (in_AX < 0x2) {
      bVar2 = true;
    }
    else {
      GetDlgItem16(param_5,0x1);
      pass1_1010_4e8c((param_1 + 0x8e),unaff_SS);
      in_AX = EnableWindow16(0x1010,0x1);
      pass1_1010_4df0((param_1 + 0x8e),in_DX,unaff_SS);
      if ((in_AX == 0x0) && (bVar2 = true, (param_1 + 0x96) == 0x0)) {
        in_AX = EnableWindow16(0x1010,0x0);
      }
    }
  }
  if (bVar2) {
    u_var5 = 0x1000;
    mem_op_1000_179c(0xb4,in_DX,0x1000);
    puVar4 = (in_DX | in_AX);
    if (puVar4 == 0x0) {
      iVar3 = 0x0;
      puVar4 = 0x0;
    }
    else {
      u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520(CONCAT22(in_DX,in_AX),
                               (param_1 + 0x6),0x30,0x2,0x57b,0x62a,puVar4,
                               unaff_SS);
    }
    puStack12 = CONCAT22(puVar4,iVar3);
    ppcVar1 = (*puStack12 + 0x74);
    (**ppcVar1)(u_var5,iVar3,puVar4);
  }
  return;
}


pub fn post_win_msg_1020_1ca4(param_1: u32) -> bool

{
  let ppcVar1: u32;
  let in_AX: u16;
  let iVar2: i16;
  let in_DX: U32Ptr;
  let puVar3: U32Ptr;
  let u_var4: u16;
  let unaff_SS: u16;
  let puStack10: u32;
  
 // u_var4 = (param_1 >> 0x10);
  if ((param_1 + 0x96) == 0x0) {
    pass1_1010_4df0((param_1 + 0x8e),in_DX,unaff_SS);
    if (in_AX == 0x0) {
      mem_op_1000_179c(0xb4,in_DX,0x1000);
      puVar3 = (in_DX | in_AX);
      if (puVar3 == 0x0) {
        iVar2 = 0x0;
        puVar3 = 0x0;
      }
      else {
        iVar2 = string_1040_8520(CONCAT22(in_DX,in_AX),
                                 ctx.PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x62a,puVar3,
                                 unaff_SS);
      }
      puStack10 = CONCAT22(puVar3,iVar2);
      ppcVar1 = (*puStack10 + 0x74);
      (**ppcVar1)();
      return 0x0;
    }
    PostMessage16(0x1010,0x0,0x0,0x11100de);
  }
  return 0x1;
}



pub fn
set_win_tet_1020_1d2a
          (param_1: u16,param_2: u16,in_win_text_3: SEGPTR,param_4: u16,
          in_dlg_id_5: i16,in_hwnd_6: HWND16)

{
  GetDlgItem16(in_hwnd_6,in_dlg_id_5);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,in_win_text_3);
  return;
}


pub fn window_op_1020_2642(astruct *param_1)
{
  let in_AX: &mut Struct664;
  let in_DX: U32Ptr;
  let u_var1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0((iVar2 + 0xf2),(iVar2 + 0x8),0x1018);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  u_var1 = in_DX | in_AX;
  if (u_var1 != 0x0) {
    pass1_1020_27b0(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    (iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = u_var1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


pub fn post_win_msg_1020_291a(param_1: HWND16)
{
  PostMessage16(param_1,0x0,0x0,0x100000);
  return;
}


u32 
send_msg_1020_29d8(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
                  param_6: u16,param_7: u16)

{
  let pu_var1: U32Ptr;
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar2: U32Ptr;
  let iVar3: i16;
  
 // iVar3 = (param_4 >> 0x10);
  post_win_msg_1020_79fc
            (CONCAT22(param_2,param_1),param_3,param_4,iVar3,param_7
            );
  puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x29,unaff_SS,param_6,unaff_DI);
 // pu_var1 = (puVar2 >> 0x10);
  if (iVar3 == 0x0) {
    pass1_1018_270e(puVar2,0x1,(param_1 + 0xfc),pu_var1,unaff_DI,unaff_SS);
  }
  else {
    pass1_1018_270e(puVar2,0x0,(param_1 + 0xfc),pu_var1,unaff_DI,unaff_SS);
    SendMessage16(0x1018,0x0,0x0,0x1110069);
  }
  return CONCAT22(param_6,param_5);
}


pub fn bring_window_to_top_1020_2aae(param_1: u32,param_2: u32)
{
  let unaff_SS: u16;
  
  call_fn_ptr_1008_3e0e(param_1);
  BringWindowToTop16(0x1008);
  pass1_1018_169e((param_1 + 0xf2),param_2,unaff_SS);
  return;
}


pub fn enable_menu_item_1020_2c2a(HMENparam_1: u16,param_2: i16) -> bool

{
  let BVar1: bool;
  let w_item_id: u16;
  
  if (param_2 != 0x0) {
    return param_2 - 0x1;
  }
  EnableMenuItem16(param_1,0x400,0x3);
  if (ctx.PTR_LOOP_1050_3960 < 0x2) {
    w_item_id = 0x401;
  }
  else {
    w_item_id = 0x400;
  }
  BVar1 = EnableMenuItem16(ctx.s_tile2_bmp_1050_1538,w_item_id,0x5);
  return BVar1;
}


pub fn win_ui_op_1020_2cf0(astruct *param_1)
{
  let u_var1: u32;
  let ppcVar2: u32;
  let uVar3: u16;
  bool *pBVar4;
  let in_DX: U32Ptr;
  let u_var5: u16;
  let extraout_dx: U32Ptr;
  let puVar6: U32Ptr;
  let uVar7: u16;
  let extraout_DX_00: u16;
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  let unaff_SS: u16;
  let puVar10: U32Ptr;
  let uVar11: u32;
  let puVar12: U32Ptr;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  puVar10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,(iVar8 + 0xfc),unaff_SS,in_DX,
                            unaff_DI);
 // u_var5 = (puVar10 >> 0x10);
  (iVar8 + 0xf2) = puVar10;
  (iVar8 + 0xf4) = u_var5;
  (iVar8 + 0xe0) = (iVar8 + 0xf2);
  (iVar8 + 0xe2) = u_var5;
  puVar12 = ctx.PTR_LOOP_1050_038c;
  uVar3 = LoadIcon16(0x1010,s_SITEICON_1050_428d);
  (iVar8 + 0xc2) = uVar3;
  u_var1 = (iVar8 + 0xf2);
  ppcVar2 = ((iVar8 + 0xf2) + 0x30);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,u_var1,(u_var1 >> 0x10),uVar3,
              puVar12);
  puVar6 = extraout_dx;
  mem_op_1000_179c(0x22,extraout_dx,0x1000);
  uVar7 = puVar6 | uVar3;
  if (uVar7 == 0x0) {
    (iVar8 + 0xf6) = 0x0;
  }
  else {
    load_draw_op_1020_2ede(CONCAT22(puVar6,uVar3),param_1,0x1000);
    (iVar8 + 0xf6) = uVar3;
    (iVar8 + 0xf8) = uVar7;
  }
  (iVar8 + 0xe8) = (iVar8 + 0xf6);
  pass1_1018_0ac0((iVar8 + 0xf2),param_1 & 0xffff | uVar9 << 0x10)
  ;
  uVar11 = pass1_1018_0b08((iVar8 + 0xf2));
  pBVar4 = (bool *)uVar11;
  ppcVar2 = (param_1 + 0x14);
  (**ppcVar2)();
  ppcVar2 = ((iVar8 + 0xf2) + 0x10);
  (**ppcVar2)();
  MoveWindow16(0x1018,0x1,pBVar4[0x3],pBVar4[0x2],pBVar4[0x1],*pBVar4);
  call_fn_ptr_1008_3e0e(param_1);
  UpdateWindow16(0x1008);
  return;
}


pub fn win_ui_op_1020_36f6(param_1: u32,param_2: i16,param_3: i16)
{
  let i_var1: i16;
  let ppcVar2: u32;
  let uVar3: u32;
  let mut pcVar4: String; 
  let u_var5: u16;
  let uVar6: u16;
  lp_string: SEGPTR;
  let iVar7: i16;
  let uVar8: u16;
  let hwnd: HWND16;
  let mut pcVar9: String; 
  let id: i16;
  let puStack1034: U32Ptr;
  local_406: u8 [0x400];
  let uStack6: u32;
  
  iVar7 = param_1;
 // uVar8 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (iVar7 + 0x8) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  uStack6 = pass1_1018_1e78((iVar7 + 0x8),-0x1);
 // uVar6 = (uStack6 >> 0x10);
  GetWindowText16(0x1018,0x3ff,local_406);
  pcVar4 = pass1_1000_472c(CONCAT22(param_3,local_406),':');
  puStack1034 = CONCAT22(uVar6,pcVar4 + 0x2);
  *puStack1034 = 0x0;
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_406,param_3);
  ppcVar2 = ((iVar7 + 0x18) + 0x18);
  (**ppcVar2)();
  uVar3 = (iVar7 + 0x8);
  i_var1 = (uVar3 + 0x4a);
  uVar3 = (uStack6 + 0x2);
  SetDlgItemText16(0x1010,uVar3,(uVar3 >> 0x10));
  uVar3 = (uStack6 + 0xa);
  SetDlgItemText16(ctx.s_tile2_bmp_1050_1538,uVar3,
                   (uVar3 >> 0x10));
  uVar3 = (uStack6 + 0x12);
  SetDlgItemText16(ctx.s_tile2_bmp_1050_1538,uVar3,
                   (uVar3 >> 0x10));
  uVar3 = (uStack6 + 0xe);
  SetDlgItemText16(ctx.s_tile2_bmp_1050_1538,uVar3,
                   (uVar3 >> 0x10));
  if (i_var1 != 0x0) {
    hwnd = 0x1018;
    u_var5 = pass1_1018_1f1a((iVar7 + 0x8),(uStack6 + 0x1a));
    if (u_var5 != 0x0) {
      uVar3 = (uStack6 + 0x16);
      id = uVar3;
     // lp_string = (uVar3 >> 0x10);
//       TODO: goto LAB_1020_3846;
    }
  }
  hwnd = 0x1010;
  pcVar9 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,0x1010);
 // lp_string = (pcVar9 >> 0x10);
  id = pcVar9;
//LAB_1020_3846:
  SetDlgItemText16(hwnd,id,lp_string);
  if ((iVar7 + 0x1c) != 0x0) {
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538,(uStack6 + 0x1a));
    SetFocus16(ctx.s_tile2_bmp_1050_1538);
    return;
  }
  InvalidateRect16(ctx.s_tile2_bmp_1050_1538,0x0,0x0);
  return;
}


pub fn window_op_1020_38aa(astruct *param_1)
{
  let ppcVar1: u32;
  let u_var2: u16;
  let paVar3: &mut Struct160;
  let u_var4: u32;
  let in_DX: U32Ptr;
  let u_var5: u16;
  let extraout_dx: U32Ptr;
  let puVar6: U32Ptr;
  let extraout_DX_00: U32Ptr;
  let puVar7: U32Ptr;
  let uVar8: u16;
  let extraout_DX_01: u16;
  let unaff_DI: i16;
  let HVar9: HWND16;
  let unaff_SS: u16;
  let puVar10: U32Ptr;
  let uVar11: u16;
  let uVar12: u16;
  let local_12: RECT16;
  let iStack14: i16;
  let iStack12: i16;
  let local_a: RECT16;
  let iStack6: i16;
  let iStack4: i16;
  
  uVar11 = param_1;
 // uVar12 = (param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  puVar10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x6,unaff_SS,in_DX,unaff_DI);
 // u_var5 = (puVar10 >> 0x10);
  (uVar11 + 0xfa) = puVar10;
  (uVar11 + 0xfc) = u_var5;
  (uVar11 + 0xe0) = (uVar11 + 0xfa);
  (uVar11 + 0xe2) = u_var5;
  if ((uVar12 | uVar11) == 0x0) {
    u_var2 = 0x0;
    uVar8 = 0x0;
  }
  else {
    u_var2 = uVar11 + 0xf2;
    uVar8 = uVar12;
  }
  ppcVar1 = ((uVar11 + 0xfa) + 0x4);
  (**ppcVar1)(0x1010,(uVar11 + 0xfa),0x0,u_var2,uVar8);
  puVar7 = extraout_dx;
  pass1_1018_1a8e((uVar11 + 0xfa),extraout_dx,unaff_DI,unaff_SS);
  mem_op_1000_179c(0x20,puVar7,0x1000);
  puVar6 = (puVar7 | u_var2);
  if (puVar6 == 0x0) {
    (uVar11 + 0xf6) = 0x0;
  }
  else {
    unk_draw_op_1020_3da4(CONCAT22(puVar7,u_var2),param_1);
    (uVar11 + 0xf6) = u_var2;
    (uVar11 + 0xf8) = extraout_DX_00;
    puVar6 = extraout_DX_00;
  }
  u_var4 = (uVar11 + 0xf6);
  (uVar11 + 0xe8) = u_var4;
  mem_op_1000_179c(0x42,puVar6,0x1000);
  paVar3 = u_var4;
  puVar7 = (puVar6 | paVar3);
  if (puVar7 == 0x0) {
    (uVar11 + 0x102) = 0x0;
  }
  else {
    pass1_1008_3bd6(paVar3,puVar6,0x0,0xf1,0x0,0x1ac01ad,
                    CONCAT22((uVar11 + 0x8),0xf4),puVar7,unaff_SS);
    (uVar11 + 0x102) = paVar3;
    (uVar11 + 0x104) = puVar7;
  }
  HVar9 = 0x1000;
  mem_op_1000_179c(0x42,puVar7,0x1000);
  uVar8 = puVar7 | paVar3;
  if (uVar8 == 0x0) {
    (uVar11 + 0x106) = 0x0;
  }
  else {
    HVar9 = 0x1008;
    pass1_1008_3bd6(paVar3,puVar7,0x0,0xf500f1,0x0,0x1ae01af,
                    CONCAT22((uVar11 + 0x8),0xf5),uVar8,unaff_SS);
    (uVar11 + 0x106) = paVar3;
    (uVar11 + 0x108) = uVar8;
  }
  u_var4 = (uVar11 + 0xfa);
  ppcVar1 = ((uVar11 + 0xfa) + 0x10);
  (**ppcVar1)(HVar9,u_var4,(u_var4 >> 0x10));
  puVar7 = paVar3.field_0x2;
  uVar8 = MoveWindow16(HVar9,0x1,&paVar3.field_0x6,&paVar3.field_0x4
                       ,puVar7,paVar3);
  HVar9 = 0x1000;
  mem_op_1000_179c(0x8e,puVar7,0x1000);
  u_var2 = puVar7 | uVar8;
  if (u_var2 == 0x0) {
    (uVar11 + 0x10a) = 0x0;
  }
  else {
    HVar9 = &ctx.PTR_LOOP_1050_1040;
    get_sys_metrics_1040_7728
              (CONCAT22(puVar7,uVar8),0x1,0x0,0xfc0,
               (uVar11 + 0x8));
    (uVar11 + 0x10a) = uVar8;
    (uVar11 + 0x10c) = u_var2;
  }
  u_var4 = (uVar11 + 0x10a);
  (u_var4 + 0x74) = 0x1;
  u_var4 = (uVar11 + 0x10a);
  ppcVar1 = ((uVar11 + 0x10a) + 0x8);
  (**ppcVar1)(HVar9,u_var4,(u_var4 >> 0x10));
  if (((uVar11 + 0x10c) | (uVar11 + 0x10a)) != 0x0) {
    GetWindowRect16(HVar9,&local_a);
    GetWindowRect16(ctx.s_tile2_bmp_1050_1538,&local_12);
    iStack12 -= local_12.y;
    iStack14 = iStack6 - local_a.x;
    local_12.x = local_a.x;
    local_12.y = iStack4 + 0x3;
    SetWindowPos16(ctx.s_tile2_bmp_1050_1538,0x44,iStack12,iStack14,local_12.y,
                   local_a.x,0x0);
  }
  return;
}


pub fn post_msg_1020_4394(param_1: u32,param_2: u16,param_3: HWND16)
{
  let u_var1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  iVar2 = param_1;
 // uVar3 = (param_1 >> 0x10);
  if (param_2 == 0x10) {
    if ((iVar2 + 0x34) != 0x0) {
      PostMessage16(param_3,0x0,0x0,0x11100f6);
      return;
    }
  }
  else {
    if (param_2 < 0x11) {
      if (param_2 == '\x01') {
        (iVar2 + 0x18) = 0x0;
        return;
      }
      if (param_2 == '\v') {
        u_var1 = (iVar2 + 0x2c);
        (u_var1 + 0xe) = (iVar2 + -0xda);
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_1020_43f6(astruct *param_1,param_2: U32Ptr,param_3: u16)
{
  let ppcVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  let u_var4: u16;
  let u_var5: u16;
  let uVar6: u16;
  let unaff_DI: i16;
  let puVar7: U32Ptr;
  let lVar8: i32;
  let uVar9: u16;
  let iVar9: &mut Struct282;
  
  iVar9 = param_1;
 // uVar9 = (param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  get_dc_1018_4db0(iVar9.field_0xfa,iVar9.field_0x8,0x1018);
  puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x32,param_3,param_2,unaff_DI);
  &iVar9.field_0x10e = puVar7;
  (&iVar9.field_0x10e + 0x2) = (puVar7 >> 0x10);
  if (param_1 == (astruct *)0x0) {
    iVar2 = 0x0;
    u_var4 = 0x0;
  }
  else {
    iVar2 = &iVar9.field_0xe2;
    u_var4 = uVar9;
  }
  ppcVar1 = (*iVar9.field_0x10e + 0x4);
  lVar8 = (**ppcVar1)(0x1010,iVar9.field_0x10e,0xb,iVar2,u_var4);
  mem_op_1000_179c(0x30,(lVar8 >> 0x10),0x1000);
 // u_var5 = (lVar8 >> 0x10);
  uVar3 = lVar8;
  uVar6 = u_var5 | uVar3;
  if (lVar8 == 0x0) {
    &iVar9.field_0xf6 = 0x0;
  }
  else {
    pass1_1020_62e0(uVar3,u_var5,iVar9.field_0x8,param_3);
    iVar9.field_0xf6 = uVar3;
    iVar9.field_0xf8 = uVar6;
  }
  ui_op_1020_536e(param_1,0x0,-0x1,0x3,uVar6);
  return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
mixed_menu_op_1020_44ec
          (param_1: u32,param_2: u16,param_3: i16,HMENparam_4: u16,HMENparam_5: u16,
          param_6: u16)

{
  let u_var1: u32;
  let u_var2: u16;
  let UVar3: u16;
  let Bvar4: bool;
  let HVar5: HMENU16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u32;
  let in_DX: U32Ptr;
  let puVar9: U32Ptr;
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  let HVar12: HMENU16;
  let in_AF: u8;
  let local_138: [u16;0x2];
  let local_134: [u16;0x2];
  let puStack304: U32Ptr;
  let uStack300: u32;
  let uStack296: u32;
  let uStack292: u32;
  let mut pcStack286: String; 
  let uStack278: u32;
  let BStack270: bool;
  let uStack268: u32;
  let local_108: [u32;0x40];
  let uStack8: u16;
  let puStack6: U32Ptr;
  
 // uVar11 = (param_1 >> 0x10);
  iVar10 = param_1;
  HVar12 = param_5;
  if ((iVar10 + 0x106) != 0x0) {
    if ((iVar10 + 0x106) == param_4) {
      puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      u_var1 = (iVar10 + 0x108);
      uStack8 = (u_var1 + 0x2e);
      u_var1 = (iVar10 + 0x108);
     // uVar11 = (u_var1 >> 0x10);
      iVar10 = u_var1;
      uStack296 = (iVar10 + 0x42);
      puVar9 = (iVar10 + 0x44);
      uStack296._3_1_ = (uStack296 >> 0x18);
      uVar6 = uStack296._3_1_;
      pcStack286 = uStack296;
      uStack268 = uStack296;
      if (uStack296._3_1_ == 0x0) {
        u_var2 = pass1_1020_bd80(uStack8);
        HVar12 = 0x1000;
        string_1000_3d3e
                  (CONCAT22(param_6,local_108),CONCAT22(puVar9,u_var2));
      }
      else {
        pass1_1030_8344(ctx.PTR__LOOP_1050_5748,
                        (ctx.PTR__LOOP_1050_5748 >> 0x10),
                        uStack296 & 0xffff | ZEXT24(puVar9) << 0x10);
        uStack296 = CONCAT22(puVar9,uVar6);
        HVar12 = 0x1010;
        pass1_1010_c3c2(puStack6,(puStack6 >> 0x10),
                        CONCAT22(param_6,local_108),CONCAT22(puVar9,uVar6),puVar9,in_AF,
                        param_6);
      }
      BStack270 = ModifyMenu16(HVar12,local_108,param_6,0x76,0x0);
      UVar3 = GetMenuState16(ctx.s_tile2_bmp_1050_1538,0x0,0x13c);
      if (UVar3 != 0xffff) {
        DeleteMenu16(ctx.s_tile2_bmp_1050_1538,0x0,0x13c);
      }
      HVar12 = 0x1008;
      BVar4 = pass1_1008_c6ae(ctx.PTR__LOOP_1050_06e0,uStack8,0x20);
      if (BVar4 != 0x0) {
        uStack296 = load_string_1010_847e
                              (ctx.PTR__LOOP_1050_14cc,
                               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
        HVar12 = ctx.s_tile2_bmp_1050_1538;
        InsertMenu16(0x1010,uStack296,(uStack296 >> 0x10),0x13c,
                     0x400);
      }
      if (((s_VrMode_1050_42ca + 0x8) + uStack8 * 0x2) == 0x0) {
        UVar3 = 0x1;
        param_4 = 0x77;
//         TODO: goto LAB_1020_464c;
      }
      param_4 = 0x77;
    }
    else {
      HVar12 = ctx.s_tile2_bmp_1050_1538;
      HVar5 = GetSubMenu16(param_5,0x1);
      uStack296 = (uStack296 & 0xffff0000 | HVar5);
      if (HVar5 != param_4) {
          // goto
          // LAB_1020_479e;
      }
      EnableMenuItem16(ctx.s_tile2_bmp_1050_1538,0x1,0x200);
      EnableMenuItem16(ctx.s_tile2_bmp_1050_1538,0x1,0x201);
      EnableMenuItem16(ctx.s_tile2_bmp_1050_1538,0x1,0x202);
      u_var1 = (iVar10 + 0x108);
      uVar8 = (u_var1 + 0x42);
      uStack292 = uVar8;
      pass1_1030_8344(ctx.PTR__LOOP_1050_5748,
                      (ctx.PTR__LOOP_1050_5748 >> 0x10),uVar8);
      uVar6 = uVar8;
      pcStack286 = (uVar8 & 0xffff | ZEXT24(in_DX) << 0x10);
      if ((in_DX | uVar6) == 0x0) {
        return;
      }
      uStack278 = (uVar6 + 0x2e);
      if (((uVar6 + 0x30) | uStack278) == 0x0) {
        return;
      }
      uStack268 = (uStack278 + 0x200);
      local_108[0] = struct_op_1030_73a8(CONCAT13((in_DX >> 0x8),
                                                  CONCAT12(in_DX,uVar6)));
      uVar11 = (local_108[0] >> 0x10);
      puStack6 = (local_108[0] + 0x1c);
      uVar6 = (local_108[0] + 0x1e);
      if ((uVar6 | puStack6) != 0x0) {
        uStack268 = (puStack6 & 0xffff | uVar6 << 0x10);
      }
      uStack268._2_2_ &= 0xff;
      if (uStack268 != 0x1) {
        return;
      }
      if ((uStack268 & 0xff0000) != 0x0) {
        return;
      }
      local_134[0] = pass1_1030_6fa0(pcStack286);
      HVar12 = 0x1008;
      BVar4 = pass1_1008_c6ae(ctx.PTR__LOOP_1050_06e0,local_134[0],0x3f);
      if (BVar4 != 0x0) {
        HVar12 = ctx.s_tile2_bmp_1050_1538;
        BVar4 = EnableMenuItem16(0x1008,0x0,0x201);
      }
      if ((pcStack286 + 0x36) != 0x0) {
        BVar4 = EnableMenuItem16(HVar12,0x0,0x202);
      }
      HVar12 = 0x1030;
      pass1_1030_69cc(pcStack286,BVar4,uStack268._2_2_,param_6);
      if (BVar4 == 0x0) {
        return;
      }
      param_4 = 0x200;
    }
    UVar3 = 0x0;
//     TODO: goto LAB_1020_464c;
  }
//LAB_1020_479e:
  iVar7 = param_3 + -0x1;
  if (iVar7 == 0x0) {
    pass1_1018_2504(0x0,in_DX);
    if (iVar7 == 0x0) {
      EnableMenuItem16(0x1018,0x401,0x0);
//LAB_1020_47e3:
      HVar12 = ctx.s_tile2_bmp_1050_1538;
      UVar3 = 0x401;
//       TODO: goto LAB_1020_464c;
    }
    HVar12 = ctx.s_tile2_bmp_1050_1538;
    EnableMenuItem16(0x1018,0x400,0x0);
  }
  else {
    if (param_3 == 0x2) {
      u_var2 = pass1_1020_64d4((iVar10 + 0xf6),0x2);
      if (u_var2 == 0x0) {
        EnableMenuItem16(HVar12,0x401,0x0);
      }
      else {
        EnableMenuItem16(HVar12,0x400,0x0);
      }
      HVar12 = ctx.s_tile2_bmp_1050_1538;
      EnableMenuItem16(ctx.s_tile2_bmp_1050_1538,CONCAT11(0x4,u_var2 == 0x0),0x1);
      if ((ctx.PTR_LOOP_1050_0010 != 0x0) || ((iVar10 + 0x102) == 0x0))
//       TODO: goto LAB_1020_47e3;
    }
    else {
      if (param_3 == 0x3) {
        local_138[0] = 0x0;
        local_134[0] = 0x0;
        HVar12 = 0x1010;
        puStack304 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2f,param_6,in_DX,unaff_DI);
       // uVar11 = (puStack304 >> 0x10);
        uStack300 = (puStack304 + 0x20);
        uVar6 = (puStack304 + 0x22);
        if ((uVar6 | uStack300) != 0x0) {
          HVar12 = 0x1030;
          pass1_1030_8308(ctx.PTR__LOOP_1050_5748,
                          (ctx.PTR__LOOP_1050_5748 >> 0x10),
                          CONCAT22(param_6,local_134),
                          CONCAT22(param_6,local_138),
                          uStack300 & 0xffff | uVar6 << 0x10,local_134,
                          uVar6);
          local_138[0] = (puStack304 + 0x1e);
        }
        uStack296 = (uStack296 & 0xffff0000);
        looo {
          CheckMenuItem16(HVar12,0x400,uStack296);
          HVar12 = ctx.s_tile2_bmp_1050_1538;
          EnableMenuItem16(ctx.s_tile2_bmp_1050_1538,0x401,uStack296);
          uStack296 = (uStack296 & 0xffff0000 |
                              (uStack296 + 0x1));
        if ((uStack296 + 0x1) < 0x5) == false { break; }
          }
        CheckMenuItem16(ctx.s_tile2_bmp_1050_1538,0x408,local_138[0]);
        // for (uStack296 = (uStack296 & 0xffff0000);
        //     uStack296 <= local_134[0];
        //     uStack296 = (uStack296 & 0xffff0000 |
        //                         (uStack296 + 0x1))) {
        //   EnableMenuItem16(ctx.s_tile2_bmp_1050_1538,0x400,uStack296);
        // }
        return;
      }
      if (param_3 != 0x4) {
        return;
      }
      param_4 = 0x2;
    }
  }
  UVar3 = 0x400;
//LAB_1020_464c:
  EnableMenuItem16(HVar12,UVar3,param_4);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
win_sys_op_1020_493c
          (param_1: U32Ptr,param_2: u16,param_3: U32Ptr,param_4: u16,HCURSOR16 param_5
          ,param_6: &WNDCLASS16)

{
  let ppcVar1: u32;
  let HVar2: HCURSOR16;
  let puVar3: U32Ptr;
  let i_var4: i16;
  let puVar5: u32;
  let uVar6: u16;
  let puVar7: U32Ptr;
  let puVar8: U32Ptr;
  let uVar9: u16;
  let uVar10: u16;
  let unaff_DI: i16;
  let uVar11: u16;
  let in_AF: u8;
  let uVar12: u32;
  let puVar13: U32Ptr;
  let paVar14: &mut Struct100;
  let mut pcVar15: String; 
  let uVar16: u8;
  let uVar17: u16;
  let uVar18: u16;
  let uVar19: u16;
  let local_356: [u32;0x42];
  let local_24e: u16;
  let puStack588: U32Ptr;
  let local_144: u32;
  let uStack320: u32;
  let local_13c: u32;
  let uStack42: u16;
  let uStack38: u32;
  let uStack34: u16;
  let puStack32: U32Ptr;
  let uStack30: u32;
  let uStack26: u32;
  let uStack22: u32;
  let paStack18: &mut Struct43;
  let puStack14: U32Ptr;
  let puStack12: U32Ptr;
  let uStack10: u16;
  let uStack6: u32;
  
  if (param_2 == 0xe9) {
    return;
  }
  uVar9 = param_1;
 // puVar8 = (param_1 >> 0x10);
  if (param_2 < 0xea) {
    if (false) {
      return;
    }
    switch(param_2) {
    0x69 =>
      i_var4 = 0x0;
      break;
    0x6a =>
      i_var4 = 0x1;
      break;
    0x6b =>
      i_var4 = 0x2;
      break;
    0x6c =>
      i_var4 = 0x3;
      break;
    0x6d =>
      i_var4 = 0x4;
      break;
    default:
      return;
    0x77 =>
      if (((uVar9 + 0x10a) | (uVar9 + 0x108)) == 0x0) {
        return;
      }
      uVar12 = (uVar9 + 0x108);
      uVar19 = ((s_VrMode_1050_42ca + 0x8) +
                        (uVar12 + 0x2e) * 0x2);
      uStack26 = (uStack26 & 0xffff0000 | uVar19);
      if (uVar19 == 0x0) {
        return;
      }
      paStack18 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x1f8,param_6);
      WinHelp16(0x1010,uStack26,
                CONCAT11((uStack26 >> 0xf),
                         (uStack26 >> 0xf)),
                CONCAT22(paStack18,0x1));
      return;
    0x78 =>
      puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x45,param_6,param_3,unaff_DI)
      ;
     // puStack588 = (puVar13 >> 0x10);
      local_24e = puVar13;
      enum_child_windows_1010_01be(0x1010);
      return;
    }
    set_cursor_1020_5764(param_1,i_var4,param_6);
    return;
  }
  if (param_2 == 0x132) {
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0xffff;
//     TODO: goto LAB_1020_4ef8;
  }
  if (param_2 < 0x133) {
    if (param_2 == 0x102) {
      uVar16 = 0x0;
      mem_op_1000_179c(0xb4,param_3,0x1000);
      puVar8 = (param_3 | param_2);
      uStack34 = param_2;
      puStack32 = param_3;
      if (puVar8 == 0x0) {
        i_var4 = 0x0;
        puVar8 = 0x0;
      }
      else {
        uVar16 = 0x40;
        i_var4 = string_1040_8520(CONCAT22(param_3,param_2),
                                 ctx.PTR_LOOP_1050_0396,0x31,0x2,0x57b,0x62b,puVar8,
                                 param_6);
      }
      local_144 = CONCAT22(puVar8,i_var4);
      ppcVar1 = (*local_144 + 0x74);
      (**ppcVar1)(uVar16,i_var4,puVar8);
      uStack320 = CONCAT22(uStack320._2_2_,i_var4);
      if (i_var4 != 0x1) {
        return;
      }
      pass1_1028_837e(CONCAT22(param_6,&local_13c),param_6,in_AF);
//LAB_1020_4b6c:
      fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748,CONCAT22(param_6,&local_13c));
      return;
    }
    if (param_2 < 0x103) {
      if (false) {
        return;
      }
      switch(param_2) {
      0xf0 =>
        ui_op_1020_536e(param_1,0x0,-0x1,0x1,param_3);
        return;
      default:
        return;
      0xf6 =>
        if ((uVar9 + 0x116) != 0x0) {
          if (param_1 == 0x0) {
            i_var4 = 0x0;
            param_3 = 0x0;
          }
          else {
            i_var4 = uVar9 + 0xe2;
            param_3 = puVar8;
          }
          local_356[0] = CONCAT22(param_3,i_var4);
          pass1_1010_1ea6(ctx.PTR__LOOP_1050_02a0,CONCAT22(param_3,i_var4),param_6);
          (uVar9 + 0x116) = 0x0;
        }
        i_var4 = 0x12;
        break;
      0xf7 =>
        unk_win_op_1010_7300((uVar9 + 0x10e),0x0,0x9,0x0);
        return;
      0xfb =>
        local_144 = 
                    mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,param_6,param_3,
                                    unaff_DI);
        uStack320 = 
                    mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x30,param_6,
                                    (local_144 >> 0x10),unaff_DI);
        pcVar15 = pass1_1010_375e(uStack320);
        pass1_1010_c25e(local_144,(local_144 >> 0x10),pcVar15,
                        pcVar15,(pcVar15 >> 0x10),unaff_DI,
                        param_6);
        return;
      0xfc =>
        post_msg_1020_55b0(param_1,param_6);
        return;
      0x101 =>
        uStack26 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2f,param_6,param_3,
                                   unaff_DI);
       // uVar11 = (uStack26 >> 0x10);
        uStack22 = (uStack26 + 0x24);
        puVar7 = (uStack26 + 0x26);
        uStack22._0_2_ = puVar7 | uStack22;
        if (uStack22 == 0x0) {
          uVar16 = 0x0;
          mem_op_1000_179c(0xb4,puVar7,0x1000);
          puVar8 = (puVar7 | uStack22);
          uStack34 = uStack22;
          puStack32 = puVar7;
          if (puVar8 == 0x0) {
            puVar5 = 0x0;
            puVar8 = 0x0;
          }
          else {
            uVar16 = 0x40;
            puVar5 = 
                     string_1040_8520(CONCAT22(puVar7,uStack22),
                                      ctx.PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x730,
                                      puVar8,param_6);
          }
          uStack30 = CONCAT22(puVar8,puVar5);
//LAB_1020_4c5f:
          ppcVar1 = (*puVar5 + 0x74);
          (**ppcVar1)(uVar16,puVar5,puVar8);
          return;
        }
        uVar12 = pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c,(uVar9 + 0x8),0xe,
                                 puVar7,uVar9,&ctx.PTR_LOOP_1050_1038,
                                 param_6);
        paStack18 = 
                    mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x43,param_6,
                                    (uVar12 >> 0x10),unaff_DI);
       // uVar11 = (paStack18 >> 0x10);
        i_var4 = paStack18;
        puStack14 = (i_var4 + 0xa);
        uStack10 = (i_var4 + 0xc);
        uVar9 = (i_var4 + 0xe);
        uStack6 = CONCAT22(uStack6._2_2_,uVar9);
        if ((i_var4 + 0x10) != 0x0) {
          return;
        }
        pass1_1028_84ca(CONCAT22(param_6,&local_13c),uStack22,uVar9,
                        uStack10,puStack14,param_6,in_AF);
//         TODO: goto LAB_1020_4b6c;
      }
    }
    else {
      if (param_2 != 0x106) {
        if (param_2 < 0x107) {
          if (param_2 == 0x103) {
            local_144 = 
                        mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2f,param_6,param_3,
                                        unaff_DI);
           // uVar11 = (local_144 >> 0x10);
            uStack320 = (local_144 + 0x24);
            puStack32 = (local_144 + 0x26);
            uStack34 = puStack32 | uStack320;
            if (uStack34 != 0x0) {
              uVar12 = pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c,(uVar9 + 0x8),0xf,
                                       puStack32,uVar9,&ctx.PTR_LOOP_1050_1038
                                       ,param_6);
              local_13c = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x42,param_6,
                                          (uVar12 >> 0x10),unaff_DI);
              uStack42 = (local_13c + 0xa);
              if (uStack42 == 0x0) {
                return;
              }
              pass1_1030_e63e(CONCAT22(param_6,&local_24e),uStack42,param_6
                              ,in_AF);
              fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748,CONCAT22(param_6,&local_24e));
              return;
            }
            uVar16 = 0x0;
            mem_op_1000_179c(0xb4,puStack32,0x1000);
            puVar8 = (puStack32 | uStack34);
            if (puVar8 == 0x0) {
              puVar5 = 0x0;
              puVar8 = 0x0;
            }
            else {
              uVar16 = 0x40;
              puVar5 = 
                       string_1040_8520(CONCAT22(puStack32,uStack34),
                                        ctx.PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x730,
                                        puVar8,param_6);
            }
            uStack38 = CONCAT22(puVar8,puVar5);
          }
          else {
            if (param_2 != 0x104) {
              return;
            }
            puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,param_6,param_3,
                                      unaff_DI);
           // puStack32 = (puVar13 >> 0x10);
            uStack34 = puVar13;
            local_24e = uStack34;
            puStack588 = puStack32;
            pass1_1010_af66(puVar13,puStack32);
            local_144 = CONCAT22(local_144._2_2_,uStack34);
            if (uStack34 != 0x0) {
              uVar16 = 0x0;
              mem_op_1000_179c(0xb4,puStack32,0x1000);
              puVar8 = (puStack32 | uStack34);
              if (puVar8 == 0x0) {
                i_var4 = 0x0;
                puVar8 = 0x0;
              }
              else {
                uVar16 = 0x40;
                i_var4 = string_1040_8520(CONCAT22(puStack32,uStack34),
                                         ctx.PTR_LOOP_1050_0396,0x31,0x2,0x57b,0x62c,
                                         puVar8,param_6);
              }
              uStack320 = CONCAT22(puVar8,i_var4);
              ppcVar1 = (*uStack320 + 0x74);
              (**ppcVar1)(uVar16,i_var4,puVar8);
              local_13c = CONCAT22(local_13c._2_2_,i_var4);
              if (i_var4 != 0x1) {
                return;
              }
              uVar16 = (param_6 >> 0x8);
              paVar14 = pass1_1030_e79a(
                                        CONCAT13(uVar16,CONCAT12(param_6,local_356))
                                        ,param_6,in_AF);
             // uVar9 = (paVar14 >> 0x10);
              puVar5 = local_356;
              fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748,
                               CONCAT13(uVar16,CONCAT12(param_6,puVar5)));
              win_1008_5c5c(param_6,puVar5,uVar9,_PTR_LOOP_1050_02a0,0x1e6);
              return;
            }
            uVar16 = 0x0;
            mem_op_1000_179c(0xb4,puStack32,0x1000);
            puVar8 = (puStack32 | uStack34);
            if (puVar8 == 0x0) {
              puVar5 = 0x0;
              puVar8 = 0x0;
            }
            else {
              uVar16 = 0x40;
              puVar5 = 
                       string_1040_8520(CONCAT22(puStack32,uStack34),
                                        ctx.PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x731,
                                        puVar8,param_6);
            }
            local_356[0] = CONCAT22(puVar8,puVar5);
          }
//           TODO: goto LAB_1020_4c5f;
        }
        if (param_2 == 0x12f) {
          pass1_1020_61c4(uVar9,puVar8,CONCAT22(param_6,&local_144),
                          CONCAT22(param_6,&local_24e),param_3,unaff_DI,
                          param_6);
          i_var4 = local_24e + 0x6a;
        }
        else {
          if (param_2 != 0x130) {
            if (param_2 != 0x131) {
              return;
            }
            uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x2);
            if (uVar6 == 0x0) {
              return;
            }
            i_var4 = 0x7;
//             TODO: goto LAB_1020_49b7;
          }
          pass1_1020_61c4(uVar9,puVar8,CONCAT22(param_6,&local_144),
                          CONCAT22(param_6,&local_24e),param_3,unaff_DI,
                          param_6);
          i_var4 = local_24e + 0x68;
        }
        uStack320 = CONCAT22(uStack320._2_2_,i_var4);
        if (0x6d < i_var4) {
          return;
        }
        if (i_var4 < 0x69) {
          return;
        }
        ppcVar1 = (*param_1 + 0x40);
        (**ppcVar1)();
        return;
      }
      i_var4 = 0x13;
    }
//LAB_1020_49b7:
    pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c,(uVar9 + 0x8),i_var4,param_3,
                    uVar9,&ctx.PTR_LOOP_1050_1038,param_6);
    return;
  }
  if (param_2 == 0x1c8) {
    SendMessage16(param_5,0x0,0x0,0x1110072);
    return;
  }
  if (0x1c8 < param_2) {
    if (param_2 == 0x1ca) {
      local_144 = 
                  mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,param_6,param_3,unaff_DI
                                 );
      uStack320 = 
                  pass1_1010_c234(local_144,(local_144 >> 0x10),
                                  unaff_DI,param_6);
     // uVar10 = (uStack320 >> 0x10);
      uVar19 = uStack320;
      if ((uVar10 | uVar19) == 0x0) {
        return;
      }
      local_13c = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x30,param_6,
                                  (uVar10 | uVar19),unaff_DI);
     // param_3 = (local_13c >> 0x10);
      pass1_1010_3770(local_13c,CONCAT22(uVar10,uVar19),param_3);
      i_var4 = 0x3;
    }
    else {
      uVar17 = (ctx.PTR__LOOP_1050_5748 >> 0x10);
      uVar6 = ctx._PTR_LOOP_1050_5748;
      if (param_2 == 0x200) {
        uVar12 = (uVar9 + 0x108);
       // uVar11 = (uVar12 >> 0x10);
        i_var4 = uVar12;
        uStack26 = (i_var4 + 0x42);
        param_3 = (i_var4 + 0x44);
        uStack26._3_1_ = (uStack26 >> 0x18);
        puStack14 = uStack26._3_1_;
        if (uStack26._3_1_ != 0x5) {
          return;
        }
        pass1_1030_8344(uVar6,uVar17,uStack26 & 0xffff | ZEXT24(param_3) << 0x10);
        i_var4 = 0x25;
        ctx.PTR_LOOP_1050_5f0c = puStack14;
        ctx.PTR_LOOP_1050_5f0e = param_3;
        puStack12 = param_3;
      }
      else {
        if (param_2 == 0x201) {
          uVar12 = (uVar9 + 0x108);
         // uVar11 = (uVar12 >> 0x10);
          i_var4 = uVar12;
          uStack26 = (i_var4 + 0x42);
          param_3 = (i_var4 + 0x44);
          uStack26._3_1_ = (uStack26 >> 0x18);
          puStack14 = uStack26._3_1_;
          if (uStack26._3_1_ != 0x5) {
            return;
          }
          pass1_1030_8344(uVar6,uVar17,uStack26 & 0xffff | ZEXT24(param_3) << 0x10)
          ;
          i_var4 = 0x26;
          ctx.PTR_LOOP_1050_5f16 = puStack14;
          ctx.PTR_LOOP_1050_5f18 = param_3;
          puStack12 = param_3;
        }
        else {
          if (param_2 != 0x202) {
            if (param_2 != 0x203) {
              return;
            }
            if ((uVar9 + 0xf4) != 0x1) {
              return;
            }
            HVar2 = SetCursor16(param_5);
            (uVar9 + 0xee) = HVar2;
            (uVar9 + 0xf4) = 0x3;
            SetCapture16(ctx.s_tile2_bmp_1050_1538);
            return;
          }
          uVar12 = (uVar9 + 0x108);
         // uVar11 = (uVar12 >> 0x10);
          i_var4 = uVar12;
          uStack6 = (i_var4 + 0x42);
          param_3 = (i_var4 + 0x44);
          uStack6._3_1_ = (uStack6 >> 0x18);
          puVar3 = uStack6._3_1_;
          if (uStack6._3_1_ != 0x5) {
            return;
          }
          pass1_1030_8344(uVar6,uVar17,uStack6 & 0xffff | ZEXT24(param_3) << 0x10);
          uStack22 = CONCAT22(param_3,puVar3);
          i_var4 = 0x27;
          ctx.PTR_LOOP_1050_5a68 = puVar3;
          ctx.PTR_LOOP_1050_5a6a = param_3;
        }
      }
    }
//     TODO: goto LAB_1020_49b7;
  }
  if (false) {
    return;
  }
  switch(param_2) {
  0x133 =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0xffff;
    uVar11 = 0x0;
    break;
  0x134 =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0x1;
//     TODO: goto LAB_1020_4ef8;
  0x135 =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0x1;
    uVar11 = 0x0;
    break;
  0x136 =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0xfffb;
//     TODO: goto LAB_1020_4ef8;
  0x137 =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0xfffb;
    uVar11 = 0x0;
    break;
  0x138 =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0x5;
//LAB_1020_4ef8:
    uVar18 = 0x0;
    break;
  0x139 =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0x5;
    uVar11 = 0x0;
    break;
  default:
//     TODO: goto switchD_1020_518a_caseD_13a;
  0x13c =>
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x2);
    if (uVar6 != 0x0) {
      i_var4 = 0x1a;
//       TODO: goto LAB_1020_49b7;
    }
//     TODO: goto switchD_1020_518a_caseD_13a;
  }
  pass1_1020_2a94((uVar9 + 0xce),CONCAT22(uVar11,uVar18),param_6);
switchD_1020_518a_caseD_13a:
  return;
}


pub fn win_ui_cursor_op_1020_522e(param_1: &mut Struct52,param_2: u16,param_3: u16)
{
  let i_var1: i16;
  let ppcVar2: u32;
  let BVar3: bool;
  let in_DX: U32Ptr;
  let i_var4: i16;
  let unaff_DI: i16;
  let u_var5: u16;
  let unaff_CS: HCURSOR16;
  let unaff_SS: u16;
  let puVar6: U32Ptr;
  let uVar7: u8;
  let uVar8: u8;
  
 // u_var5 = (param_1 >> 0x10);
  i_var4 = param_1;
  i_var1 = (i_var4 + 0xf4);
  if (i_var1 == 0x2) {
    SetCursor16(unaff_CS);
    (i_var4 + 0xee) = 0x0;
    (i_var4 + 0xf4) = 0x1;
    (i_var4 + 0x10c) = 0x0;
    ReleaseCapture16();
    return;
  }
  if (i_var1 == 0x3) {
    SetCursor16(unaff_CS);
    (i_var4 + 0xee) = 0x0;
    (i_var4 + 0xf4) = 0x1;
    (i_var4 + 0x10c) = 0x0;
    ReleaseCapture16();
    uVar7 = 0x0;
    uVar8 = 0x0;
    u_var5 = 0x0;
    puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x47,unaff_SS,in_DX,unaff_DI);
    pass1_1018_57e6(puVar6,CONCAT22(u_var5,CONCAT11(uVar8,uVar7)),unaff_SS);
    return;
  }
  BVar3 = menu_ui_op_1020_5bf2(param_1,param_2,param_3);
  if (BVar3 == 0x0) {
    ppcVar2 = ((i_var4 + 0x4) + 0x60);
    (**ppcVar2)();
  }
  return;
}


pub fn ui_op_1020_536e(param_1: u32,param_2: u32,param_3: i16,param_4: i16,param_5: U32Ptr)
{
  let piVar1: U32Ptr;
  let UVar2: u16;
  let ppc_var3: u32;
  let u_var4: u16;
  let u_var5: u16;
  let UVar6: u16;
  let uVar7: u16;
  let puVar8: U32Ptr;
  let extraout_dx: U32Ptr;
  let puVar9: U32Ptr;
  let iVar10: i16;
  let puVar11: u32;
  let unaff_DI: i16;
  let uVar12: u16;
  let unaff_SS: u16;
  let puVar13: U32Ptr;
  let uVar14: u32;
  let uVar15: u32;
  let uVar16: u8;
  let uVar17: u8;
  let uVar18: u16;
  let uVar19: u16;
  let puStack16: u32;
  
  uVar7 = param_4 - 0x1;
 // uVar12 = (param_1 >> 0x10);
  iVar10 = param_1;
  if (uVar7 == 0x0) {
    if ((iVar10 + 0xfe) == 0x0) {
      mem_op_1000_179c(0xfc,param_5,0x1000);
      uVar14 = CONCAT22(param_5 | uVar7,uVar7);
      if ((param_5 | uVar7) == 0x0) {
        (iVar10 + 0xfe) = 0x0;
      }
      else {
        piVar1 = (iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        uVar14 = unk_win_ui_op_1020_67ce
                           (CONCAT13((param_5 >> 0x8),
                                     CONCAT12(param_5,uVar7)),
                            (iVar10 + 0xcc),param_1);
        (iVar10 + 0xfe) = uVar14;
        (iVar10 + 0x100) = (uVar14 >> 0x10);
      }
      pass1_1008_6978(param_1,0x0,(iVar10 + 0xfe),uVar14,
                      (uVar14 >> 0x10));
      uVar14 = (iVar10 + 0xfe);
      uVar18 = uVar14;
     // uVar19 = (uVar14 >> 0x10);
      uVar14 = (iVar10 + 0xfe);
     // uVar12 = (uVar14 >> 0x10);
      puVar11 = uVar14;
//       TODO: goto LAB_1020_53f3;
    }
  }
  else {
    if (param_4 == 0x2) {
      u_var4 = param_3 + 0xc;
      puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,u_var4,unaff_SS,param_5,unaff_DI);
     // puVar9 = (puVar13 >> 0x10);
      u_var5 = pass1_1018_0afa(puVar13);
      if (u_var5 == 0x0) {
        piVar1 = (iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        UVar2 = (iVar10 + 0xcc);
        uVar12 = 0xfe;
        UVar6 = UVar2;
        mem_op_1000_179c(0xfe,puVar9,0x1000);
        puVar8 = (puVar9 | UVar6);
        if (puVar8 == 0x0) {
          UVar6 = 0x0;
          puVar8 = 0x0;
        }
        else {
          pass1_1020_289a(
                          CONCAT13((puVar9 >> 0x8),
                                   CONCAT12(puVar9,UVar6)),UVar2,param_1,unaff_SS);
        }
        puStack16 = CONCAT22(puVar8,UVar6);
        uVar16 = SUB21(puVar8,0x0);
        uVar17 = (puVar8 >> 0x8);
        pass1_1020_294a(CONCAT13(uVar17,CONCAT12(uVar16,UVar6)),
                        CONCAT22(param_2,uVar12),(param_2 >> 0x10),
                        puVar8,unaff_DI,unaff_SS);
        unaff_DI = (*puStack16 >> 0x10);
        iVar10 = *puStack16;
        ppc_var3 = (iVar10 + 0x8);
        uVar14 = (**ppc_var3)(0x1000,UVar6,puVar8,u_var4);
        call_fn_ptr_1008_3e0e(CONCAT13(uVar17, CONCAT12(uVar16, UVar6)));
        pass1_1008_6978(param_1,UVar2,CONCAT22(puVar8,UVar6),uVar14,
                        (uVar14 >> 0x10));
        ppc_var3 = (iVar10 + 0xc);
        (**ppc_var3)(0x1008,UVar6,uVar16,0x1);
        puVar9 = extraout_dx;
      }
      else {
        uVar15 = pass1_1018_0ad4(puVar13);
       // puVar9 = (uVar15 >> 0x10);
        call_fn_ptr_1008_3e0e(uVar15);
      }
      pass1_1018_1662(puVar13,0x0,0x0,unaff_SS);
      BringWindowToTop16(0x1018);
      u_var4 = 0x1;
      iVar10 = 0x4;
      puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2b,unaff_SS,puVar9,unaff_DI);
      pass1_1010_089e(unaff_SS,puVar13,u_var4,iVar10);
      pass1_1010_089e(unaff_SS,puVar13,0x1,0x3);
      return;
    }
    uVar7 = param_4 - 0x3;
    if ((uVar7 == 0x0) && ((iVar10 + 0x102) == 0x0)) {
      mem_op_1000_179c(0xfc,param_5,0x1000);
      puVar9 = (param_5 | uVar7);
      if (puVar9 == 0x0) {
        (iVar10 + 0x102) = 0x0;
      }
      else {
        piVar1 = (iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        pass1_1020_0dc4(
                        CONCAT13((param_5 >> 0x8),
                                 CONCAT12(param_5,uVar7)),(iVar10 + 0xcc)
                        ,param_1,unaff_SS);
        (iVar10 + 0x102) = uVar7;
        (iVar10 + 0x104) = puVar9;
      }
      pass1_1008_6978(param_1,0x0,(iVar10 + 0x102),uVar7,puVar9);
      uVar14 = (iVar10 + 0x102);
      uVar18 = uVar14;
     // uVar19 = (uVar14 >> 0x10);
      uVar14 = (iVar10 + 0x102);
     // uVar12 = (uVar14 >> 0x10);
      puVar11 = uVar14;
//LAB_1020_53f3:
      ppc_var3 = (*puVar11 + 0xc);
      (**ppc_var3)(0x8,uVar18,uVar19,0x5);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn post_msg_1020_55b0(param_1: u32,param_2: u16) -> u16

{
  let ppcVar1: u32;
  let u_var2: u16;
  let in_DX: U32Ptr;
  let puVar3: U32Ptr;
  let u_var4: u16;
  let extraout_dx: U32Ptr;
  let extraout_DX_00: U32Ptr;
  let unaff_DI: i16;
  let u_var5: u16;
  let hwnd: HWND16;
  let hwnd_00: HWND16;
  let in_AF: u8;
  let puVar5: U32Ptr;
  let mut pcVar6: String; 
  let puVar6: u32;
  let LVar7: LRESULT;
  let uVar8: u8;
  let local_114: i16;
  let local_112: [u8;2];
  let iStack272: i16;
  let local_10e: i16;
  local_10c: u8 [0x100];
  let puStack12: U32Ptr;
  let iStack8: i16;
  let puStack6: U32Ptr;
  
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
 // puVar3 = (puStack6 >> 0x10);
  iStack8 = (puStack6 + 0x20);
  puStack12 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x37,param_2,puVar3,unaff_DI);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_10c,param_2);
  puVar5 = pass1_1008_9436(CONCAT22(param_2,local_112));
  uVar8 = (param_2 >> 0x8);
  pcVar6 = pass1_1008_a8f4(puStack12,

                                   CONCAT13(uVar8,CONCAT12(param_2,&local_114)),
                                   CONCAT22(param_2,local_112),
                                   CONCAT22(param_2,&local_10e),
                                   (puVar5 >> 0x10),0x1008,param_2,in_AF);
  u_var2 = pcVar6;
  puVar3 = ((pcVar6 >> 0x10) | u_var2);
 // u_var5 = (param_1 >> 0x10);
  hwnd_00 = 0x1008;
  if ((pcVar6 != 0x0) && (*pcVar6 != '\0')) {
    hwnd = 0x1000;
    mem_op_1000_179c(0xb4,puVar3,0x1000);
    if ((puVar3 | u_var2) == 0x0) {
      puVar6 = 0x0;
    }
    else {
      hwnd = &ctx.PTR_LOOP_1050_1040;
      puVar6 = pass1_1040_8478(CONCAT22(puVar3,u_var2),0x0,
                                        CONCAT13(uVar8,CONCAT12(param_2,
                                                                        local_10c)),pcVar6
                                        ,(param_1 + 0x8),
                                        puVar3 | u_var2);
    }
   // u_var4 = (puVar6 >> 0x10);
    if (iStack272 == 0x0) {
      ppcVar1 = (*puVar6 + 0x74);
      (**ppcVar1)(hwnd,(puVar6 & 0xffff),u_var4);
      puVar3 = extraout_DX_00;
    }
    else {
      ppcVar1 = (*puVar6 + 0x6c);
      (**ppcVar1)(hwnd,(puVar6 & 0xffff),u_var4,local_112,param_2);
      puVar3 = extraout_dx;
    }
    if ((iStack8 == 0x0) || (hwnd_00 = hwnd, local_114 == 0x0)) {
      hwnd_00 = ctx.s_tile2_bmp_1050_1538;
      PostMessage16(hwnd,0x0,0x0,0x11100fc);
    }
  }
  if ((iStack8 != 0x0) && (local_114 != 0x0)) {
    LVar7 = SendMessage16(hwnd_00,0x0,0x0,CONCAT22(0x111,local_114));
    (param_1 + 0x112) = 0x1;
    return (LVar7 >> 0x10);
  }
  if (local_10e == 0x6) {
    PostMessage16(hwnd_00,0x0,0x0,0x11100b0);
    hwnd_00 = 0x1010;
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_2,puVar3,unaff_DI);
   // puVar3 = (puVar5 >> 0x10);
    (puVar5 + 0x20) = 0x1;
  }
  if (local_10e == 0x15) {
    PostMessage16(hwnd_00,0x0,0x0,0x1110097);
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_2,puVar3,unaff_DI);
   // puVar3 = (puVar5 >> 0x10);
    (puVar5 + 0x20) = 0x1;
  }
  return puVar3;
}


pub fn menu_ui_op_1020_5bf2(param_1: &mut Struct52,param_2: HWND16,param_3: &RECT16) -> bool

{
  let u_var1: u32;
  let u_var2: u16;
  let BVar3: bool;
  RECT16 **ppRVar4;
  let HVar5: HMENU16;
  let in_DX: u16;
  let uVar6: u16;
  let iVar5: &mut Struct52;
  let uVar7: u16;
  let unaff_CS: *mut RECT16;
  let instance: *mut RECT16;
  unaff_SS: &WNDCLASS16;
  let uVar8: u32;
  POlet local_10: i16;
  let iStack12: i16;
  let uStack10: u32;
  let local_6: *mut RECT16;
  let HStack4: HWND16;
  
  local_6 = param_3;
  HStack4 = param_2;
 // uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  u_var2 = pass1_1020_64d4(iVar5.field_0xf6,0x2);
  uVar8 = in_DX << 0x10;
  instance = unaff_CS;
  if (u_var2 != 0x0) {
    uStack10 = pass1_1020_6498(iVar5.field_0xf6,0x2);
    instance = s_tile2_bmp_1050_1538;
    uVar8 = uStack10;
    BVar3 = PtInRect16(unaff_CS,CONCAT22(HStack4,local_6));
    if (BVar3 != 0x0) {
      PostMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1110131);
      return 0x1;
    }
  }
  u_var2 = pass1_1020_64d4(iVar5.field_0xf6,0x3);
  if (u_var2 == 0x0) {
    return 0x0;
  }
  ppRVar4 = &local_6;
  pt_in_rect_1020_5856(param_1,CONCAT22(unaff_SS,ppRVar4),ppRVar4);
 // uVar6 = (uVar8 >> 0x10);
  iVar5.field_0x108 = ppRVar4;
  iVar5.field_0x10a = uVar6;
  if ((uVar6 | iVar5.field_0x108) == 0x0) {
    return 0x0;
  }
  if (iVar5.field_0x106 == 0x0) {
    HVar5 = LoadMenu16(instance,s_TILEMENU_1050_43f0);
    iVar5.field_0x106 = HVar5;
    if (HVar5 == 0x0) {
      return 0x1;
    }
    instance = s_tile2_bmp_1050_1538;
    HVar5 = GetSubMenu16(ctx.s_tile2_bmp_1050_1538,0x0);
    iVar5.field_0x106 = HVar5;
    if (HVar5 == 0x0) {
      return 0x1;
    }
  }
  u_var1 = &iVar5.field_0x108;
  uStack10._0_2_ = (u_var1 + 0x2e);
  iStack12 = 0x0;
  if (uStack10 == 0x42) {
    iStack12 = 0xc9;
  }
  else {
    if (((s_VrMode_1050_42ca + 0x8) + uStack10 * 0x2) == 0x0) {
      iStack12 = 0xc8;
    }
  }
  if (iStack12 != 0x0) {
    instance = 0x1008;
    win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,CONCAT22(iStack12,0x1),unaff_SS,uStack10,
                  (uVar8 >> 0x10));
  }
  local_10.x = param_3;
  local_10.y = param_2;
  ClientToScreen16(instance,&local_10);
  TrackPopupMenu16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,iVar5.field_0x8,0x0,local_10.y,
                   local_10.x);
  return 0x1;
}


pub fn win_ui_op_1020_5de8(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let u_var1: u16;
  let u_var2: u32;
  let puVar3: U32Ptr;
  let in_DX: U32Ptr;
  let u_var4: u16;
  let unaff_DI: i16;
  let u_var5: u16;
  let puVar6: U32Ptr;
  let uVar7: u8;
  let uVar8: u8;
  let uStack18: u16;
  let cStack15: u8;
  let local_6: u16;
  let uStack4: u16;
  
  ReleaseCapture16();
 // u_var5 = (param_1 >> 0x10);
  SetCursor16(ctx.s_tile2_bmp_1050_1538);
  (param_1 + 0xee) = 0x0;
  (param_1 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x47,param_4,in_DX,unaff_DI);
 // u_var4 = (puVar6 >> 0x10);
  puVar3 = &local_6;
  pt_in_rect_1020_5856
            (param_1,
                     CONCAT13((param_4 >> 0x8),CONCAT12(param_4,puVar3)),
             puVar3);
  if ((u_var4 | puVar3) != 0x0) {
    u_var2 = (puVar3 + 0x21);
    u_var1 = puVar3[0x22];
    cStack15 = (u_var2 >> 0x18);
    if (cStack15 == '\x05') {
      uVar7 = u_var1;
      uVar8 = (u_var1 >> 0x8);
      uStack18 = u_var2;
//       TODO: goto LAB_1020_5e62;
    }
  }
  uStack18 = 0x0;
  uVar7 = 0x0;
  uVar8 = 0x0;
//LAB_1020_5e62:
  pass1_1018_57e6(puVar6,CONCAT13(uVar8,CONCAT12(uVar7,uStack18)),param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1020_5e76(param_1: u32,param_2: u16,param_3: u16)
{
  let ppcVar1: u32;
  let paVar2: &mut Struct57;
  let puVar3: U32Ptr;
  let puVar4: U32Ptr;
  let iVar5: i16;
  let uVar6: u16;
  let in_DX: u16;
  let puVar7: U32Ptr;
  let puVar8: U32Ptr;
  let iVar9: i16;
  let puVar10: u32;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let unaff_SS: U32Ptr;
  let in_AF: u8;
  let mut pcVar14: String; 
  let uVar15: u8;
  local_2aa: U32Ptr [0x40];
  local_1aa: U32Ptr [0x80];
  local_aa: u8 [0x80];
  let uStack42: u32;
  let paStack38: &mut Struct57;
  local_22: u8 [0x10];
  let puStack18: U32Ptr;
  let uStack16: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let local_6: u16;
  let uStack4: u16;
  
  ReleaseCapture16();
 // uVar11 = (param_1 >> 0x10);
  iVar9 = param_1;
  SetCursor16(ctx.s_tile2_bmp_1050_1538);
  (iVar9 + 0xee) = 0x0;
  (iVar9 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar3 = &local_6;
  uVar15 = (unaff_SS >> 0x8);
  pt_in_rect_1020_5856
            (param_1,CONCAT13(uVar15,CONCAT12(unaff_SS,puVar3)),puVar3);
  uStack10 = CONCAT22(in_DX,puVar3);
  puVar7 = (in_DX | puVar3);
  if (puVar7 == 0x0) {
      // goto
      // LAB_1020_6176;
  }
  uStack12 = puVar3[0x6];
  uStack14 = puVar3[0x7];
  uStack16 = 0x0;
  uVar13 = 0x1018;
  puVar4 = pass1_1018_2580((iVar9 + 0xfa),0x0,CONCAT22(uStack12,uStack14),
                           (iVar9 + 0x10c),unaff_SS,in_AF);
  if (puVar4 == 0x6b2) {
      // goto
      // LAB_1020_6176;
  }
  puStack18 = puVar4;
  if (puVar4 == 0x6b8) {
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    uStack42 = CONCAT22(puVar7,puVar4);
    puVar8 = (puVar7 | puVar4);
    if (puVar8 == 0x0) {
      iVar5 = 0x0;
      puVar8 = 0x0;
    }
    else {
      iVar5 = string_1040_8520(
                               CONCAT13((puVar7 >> 0x8),
                                        CONCAT12(puVar7,puVar4)),
                               ctx.PTR_LOOP_1050_0396,0x40,0x2,0x6b8,0x6ad,puVar8,
                               unaff_SS);
    }
    paStack38 = CONCAT22(puVar8,iVar5);
    uVar13 = 0xa5;
//LAB_1020_5f84:
    pass1_1008_941a(CONCAT22(unaff_SS,local_22),0x1,uVar13);
    pcVar14 = local_22;
   // uVar12 = (paStack38 >> 0x10);
    puVar10 = paStack38;
  }
  else {
    if (puVar4 == 0x6b4) {
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,puVar4);
      puVar8 = (puVar7 | puVar4);
      if (puVar8 == 0x0) {
        iVar5 = 0x0;
        puVar8 = 0x0;
      }
      else {
        iVar5 = string_1040_8520(
                                 CONCAT13((puVar7 >> 0x8),
                                          CONCAT12(puVar7,puVar4)),
                                 ctx.PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,
                                 puVar8,unaff_SS);
      }
      paStack38 = CONCAT22(puVar8,iVar5);
      uVar13 = 0xab;
//       TODO: goto LAB_1020_5f84;
    }
    if (puVar4 == 0x6b6) {
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_aa,
                 (short)unaff_SS);
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_1aa,
                 (short)unaff_SS);
      uVar6 = sys_1000_3f9c(local_2aa,unaff_SS,local_1aa,unaff_SS
                            ,PTR_LOOP_1050_50cc,&stack0xfffe,uVar11,0x1000,
                            unaff_SS,in_AF);
      uVar12 = 0x1000;
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,uVar6);
      if ((puVar7 | uVar6) == 0x0) {
        paStack38 = 0x0;
      }
      else {
        uVar12 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
        paStack38 = pass1_1040_8478(CONCAT22(puVar7,uVar6),0x40,
                                    CONCAT13(uVar15,CONCAT12(unaff_SS,
                                                                     local_aa)),
                                    CONCAT22(unaff_SS,local_2aa),
                                    ctx.PTR_LOOP_1050_0396,puVar7 | uVar6);
      }
     // puVar8 = (paStack38 >> 0x10);
      puVar10 = paStack38;
      paVar2 = paStack38;
//LAB_1020_6027:
      ppcVar1 = (*puVar10 + 0x74);
      (**ppcVar1)(uVar12,paVar2);
//       TODO: goto LAB_1020_6176;
    }
    if (puVar4 < 0x6a7) {
      if (((iVar9 + 0x10c) == 0x78) || ((iVar9 + 0x10c) == 0x74)) {
        uVar13 = 0x1010;
        local_2aa[0] = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x5,unaff_SS,puVar7,
                                       unaff_DI);
        puVar7 = (local_2aa[0] >> 0x10);
        if ((local_2aa[0] + 0xa) == 0x0) {
          return;
        }
      }
      if (((((iVar9 + 0x10c) == 0x6c) || ((iVar9 + 0x10c) == 0x6d)) ||
          ((iVar9 + 0x10c) == 0x31)) || ((iVar9 + 0x10c) == 0x32)) {
        uVar13 = 0x1010;
        local_2aa[0] = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3a,unaff_SS,puVar7,
                                       unaff_DI);
        if ((local_2aa[0] + 0xa) == 0x0) {
          return;
        }
      }
      pass1_1020_68de((iVar9 + 0xfe),uVar13);
//       TODO: goto LAB_1020_6176;
    }
    if (0x6b1 < puVar4) {
      uVar12 = 0x1000;
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,puVar4);
      puVar8 = (puVar7 | puVar4);
      if (puVar8 == 0x0) {
        puVar10 = 0x0;
        puVar8 = 0x0;
      }
      else {
        uVar12 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
        puVar10 = 
                  string_1040_8520(
                                   CONCAT13((puVar7 >> 0x8),
                                            CONCAT12(puVar7,puVar4)),
                                   ctx.PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,
                                   puVar8,unaff_SS);
      }
      local_2aa[0] = CONCAT22(puVar8,puVar10);
      paVar2 = CONCAT22(puVar8,puVar10);
//       TODO: goto LAB_1020_6027;
    }
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    uStack42 = CONCAT22(puVar7,puVar4);
    puVar8 = (puVar7 | puVar4);
    if (puVar8 == 0x0) {
      iVar5 = 0x0;
      puVar8 = 0x0;
    }
    else {
      iVar5 = string_1040_8520(
                               CONCAT13((puVar7 >> 0x8),
                                        CONCAT12(puVar7,puVar4)),
                               ctx.PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,puVar8,
                               unaff_SS);
    }
    local_2aa[0] = CONCAT22(puVar8,iVar5);
    local_1aa[0] = puStack18 + -0x608;
    pass1_1008_941a(CONCAT22(unaff_SS,local_aa),0x1,local_1aa[0]);
    pcVar14 = local_aa;
    uVar12 = (local_2aa[0] >> 0x10);
    puVar10 = local_2aa[0];
  }
  ppcVar1 = (*puVar10 + 0x6c);
  (**ppcVar1)(0x1008,puVar10,uVar12,pcVar14);
//LAB_1020_6176:
  (iVar9 + 0x10c) = 0x0;
  return;
}


pub fn unk_win_op_1020_65cc(param_1: &mut Struct60,param_2: i16,param_3: u16)
{
  let ppcVar1: u32;
  let u_var2: u32;
  let BVar3: bool;
  let u_var4: u16;
  let i_var4: &mut Struct59;
  let iVar5: &mut Struct60;
  let iVar6: i16;
  let uVar7: &mut Struct60;
  let hwnd: HWND16;
  let iStack4: i16;
  
  iVar5 = param_1;
 // uVar7 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    iVar5.field_0x14 = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    // for (iStack4 = 0x0; iStack4 < 0x5; iStack4 += 0x1) {
    //   i_var4 = &iVar5.field_0x18;
    //   iVar6 = iStack4 * 0x4;
    //   if (((i_var4 + iVar6 + 0x2) | (i_var4 + iVar6)) != 0x0) {
    //     ppcVar1 = (**(u32 **)(i_var4 + iVar6) + 0x4);
    //     (**ppcVar1)(param_3,(i_var4 + iVar6));
    //   }
    // }
  }
  else {
    if (((0x0 < param_2 + -0x3) && (!SBORROW2(param_2 + -0x3,0x1))) &&
       (param_2 + -0x4 < 0x4)) {
      BVar3 = IsIconic16(param_3);
      if (BVar3 == 0x0) {
        BVar3 = IsIconic16(ctx.s_tile2_bmp_1050_1538);
        if ((BVar3 == 0x0) &&
           (u_var2 = iVar5.field_0x14, (u_var2 + 0x24) != 0x0)) {
          InvalidateRect16(ctx.s_tile2_bmp_1050_1538,0x0,0x0);
          u_var4 = pass1_1020_64d4(param_1,0x2);
          if (u_var4 == 0x0) {
            pass1_1020_6746(param_1,0x1,0x2);
          }
          u_var4 = pass1_1020_64d4(param_1,0x3);
          if (u_var4 == 0x0) {
            pass1_1020_6746(param_1,0x1,0x3);
          }
          hwnd = 0x1018;
          u_var4 = pass1_1018_255e(iVar5.field_0x14);
          if (u_var4 == 0x0) {
            hwnd = ctx.s_tile2_bmp_1050_1538;
            SendMessage16(0x1018,0x0,0x0,0x1110069);
          }
          else {
            u_var4 = pass1_1020_64d4(param_1,0x1);
            if (u_var4 == 0x0) {
              pass1_1020_6746(param_1,0x1,0x1);
            }
          }
          SendMessage16(hwnd,0x0,0x0,0x11100f0);
          u_var2 = iVar5.field_0x2c;
          if ((u_var2 + 0x7a) != 0x0) {
            u_var2 = iVar5.field_0x2c;
            (u_var2 + 0x7a) = 0x0;
            SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1110131);
            return;
          }
        }
      }
    }
  }
  return;
}


pub fn unk_win_ui_op_1020_67ce(in_struct_1: &mut Struct20,param_2: u16,param_3: i32)
{
  let HVar1: HGDIOBJ16;
  let HVar2: HCURSOR16;
  let iVar3: &mut Struct20;
  let u_var4: &mut Struct20;
  let unaff_SS: u16;
  
  struct_1020_790e(in_struct_1,s_TPPOPMENU_1050_43fa,param_2,param_3,
                   unaff_SS);
 // u_var4 = (in_struct_1 >> 0x10);
  iVar3 = in_struct_1;
  iVar3[0x1].field_0x10 = 0x0;
  &iVar3[0x1].field_0x14 = 0x0;
  in_struct_1.field_0x0 = 0x70e6;
  iVar3.field_0x2 = 0x1020;
  string_1000_3d3e
            ((in_struct_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x5b)),
             s_VrMode2_1050_4404);
  HVar1 = GetStockObject16(0x1000);
  iVar3.hgdiobj_field_0xc6 = HVar1;
  HVar2 = LoadCursor16(s_tile2_bmp_1050_1538,0x7f00);
  iVar3.hcursor_field_0xc4 = HVar2;
  iVar3.field_0xac = 0x44c00000;
  iVar3.field_0xc8 = 0x2020;
  iVar3.field_0xbc = (param_3 + 0x8);
  iVar3.field_0xca = param_2;
  win_ui_reg_class_1008_96d2(in_struct_1,0x1008,unaff_SS);
  window_op_1020_6c3a(in_struct_1);
  return;
}


pub fn realize_palette_1020_6896(param_1: u32,param_2: i16,param_3: HGDIOBJ16)
{
  let pu_var1: u32;
  let ppcVar2: u32;
  let uVar3: u32;
  let i_var4: i16;
  let u_var5: u16;
  
  if (param_2 != 0x0) {
    uVar3 = (param_1 + 0xf2);
   // u_var5 = (uVar3 >> 0x10);
    i_var4 = uVar3;
    pu_var1 = (i_var4 + 0x24);
    ppcVar2 = (*pu_var1 + 0x18);
    (**ppcVar2)(param_3,pu_var1,(i_var4 + 0x26));
    UnrealizeObject16(param_3);
    RealizePalette16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}


pub fn
win_ui_op_1020_6ae6(param_1: U32Ptr,param_2: u16,param_3: i16,param_4: i16,param_5: HWND16,
                   param_6: WPARAM16)

{
  let ppcVar1: u32;
  let puVar2: U32Ptr;
  let iVar3: i16;
  let u_var4: u16;
  let hwnd: HWND16;
  let LVar5: LRESULT;
  let in_stack_0000ff86: u16;
  let in_stack_0000ff88: u16;
  let HVar6: HWND16;
  let local_58: [u8;50];
  let uStack8: u32;
  let HStack4: HWND16;
  
  if (param_4 == 0x1797) {
   // u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    hwnd = ctx.s_tile2_bmp_1050_1538;
    HStack4 = GetDlgItem16(param_5,0x1797);
    if (HStack4 != 0x0) {
      if (param_3 == 0x2) {
        hwnd = ctx.s_tile2_bmp_1050_1538;
        uStack8 = SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x4090000);
        if ((uStack8 != -0x1) || (false)) {
          HVar6 = HStack4;
          LVar5 = SendMessage16(ctx.s_tile2_bmp_1050_1538,local_58,param_6,
                                CONCAT22(0x40a,uStack8));
          puVar2 = local_58;
          pass1_1018_30ca((iVar3 + 0xf2),CONCAT22(param_6,puVar2),
                          (LVar5 >> 0x10));
          hwnd = 0x1018;
          pass1_1018_2fe8((iVar3 + 0xf2),in_stack_0000ff86,in_stack_0000ff88);
          if (puVar2 != 0x0) {
            invalidate_rect_1020_735a((iVar3 + 0xf6),0x1018);
            ppcVar1 = (*param_1 + 0x40);
            (**ppcVar1)(0x1018,param_1,0xef,HVar6);
          }
        }
      }
      else {
        if (param_3 != 0x3) {
          return;
        }
      }
      DestroyWindow16(hwnd);
    }
  }
  return;
}


pub fn enable_menu_item_1020_6b9a(HMENparam_1: u16,param_2: i16)
{
  if (param_2 != 0x0) {
    return;
  }
  EnableMenuItem16(param_1,0x400,0x0);
  return;
}


pub fn window_op_1020_6c3a(astruct *param_1)
{
  let u_var1: u32;
  let ppcVar2: u32;
  let HVar3: HICON16;
  let paVar4: &mut Struct160;
  bool *pBVar5;
  let uVar6: u32;
  let in_DX: U32Ptr;
  let uVar7: u16;
  let extraout_dx: U32Ptr;
  let puVar8: U32Ptr;
  let puVar9: U32Ptr;
  let uVar10: u16;
  let extraout_DX_00: u16;
  let iVar11: i16;
  let unaff_DI: i16;
  let uVar12: u16;
  let unaff_SS: u16;
  let puVar13: U32Ptr;
  let puVar14: U32Ptr;
  let local_6: u32;
  
  create_window_ex_1008_9760(param_1,0x1008);
  puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x4,unaff_SS,in_DX,unaff_DI);
 // uVar7 = (puVar13 >> 0x10);
 // uVar12 = (param_1 >> 0x10);
  iVar11 = param_1;
  (iVar11 + 0xf2) = puVar13;
  (iVar11 + 0xf4) = uVar7;
  (iVar11 + 0xe0) = (iVar11 + 0xf2);
  (iVar11 + 0xe2) = uVar7;
  puVar14 = ctx.PTR_LOOP_1050_038c;
  HVar3 = LoadIcon16(0x1010,s_TILEICON_1050_440c);
  (iVar11 + 0xc2) = HVar3;
  uVar6 = (iVar11 + 0xf2);
  ppcVar2 = ((iVar11 + 0xf2) + 0x30);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,uVar6,(uVar6 >> 0x10),HVar3,
              puVar14);
  paVar4 = (&local_6 + 0x2);
  puVar9 = extraout_dx;
  pass1_1018_2d22((iVar11 + 0xf2),CONCAT22(unaff_SS,&local_6),

                  CONCAT13((unaff_SS >> 0x8),CONCAT12(unaff_SS,paVar4)),0xbb8)
  ;
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    uVar7 = (iVar11 + 0x8);
    pass1_1008_3bd6(paVar4,puVar9,0x0,local_6,0x0,0x7c007d,
                    CONCAT13((uVar7 >> 0x8),CONCAT12(uVar7,0xbb8)),
                    puVar8,unaff_SS);
  }
  paVar4 = (&local_6 + 0x2);
  pass1_1018_2d22((iVar11 + 0xf2),CONCAT22(unaff_SS,&local_6),
                  CONCAT22(unaff_SS,paVar4),0xbb9);
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar8,0x0,local_6,0x0,0x7e007f,
                    CONCAT22((iVar11 + 0x8),0xbb9),puVar9,unaff_SS)
    ;
  }
  paVar4 = (&local_6 + 0x2);
  pass1_1018_2d22((iVar11 + 0xf2),CONCAT22(unaff_SS,&local_6),
                  CONCAT22(unaff_SS,paVar4),0xbba);
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar9,0x0,local_6,0x1b2,0x1b001b1,
                    CONCAT22((iVar11 + 0x8),0xbba),puVar8,unaff_SS)
    ;
  }
  mem_op_1000_179c(0x22,puVar8,0x1000);
  uVar10 = puVar8 | paVar4;
  if (uVar10 == 0x0) {
    (iVar11 + 0xf6) = 0x0;
  }
  else {
    unk_win_ui_op_1020_717e(CONCAT22(puVar8,paVar4),param_1,unaff_SS);
    (iVar11 + 0xf6) = paVar4;
    (iVar11 + 0xf8) = uVar10;
  }
  uVar6 = (iVar11 + 0xf6);
  (iVar11 + 0xe8) = uVar6;
  u_var1 = (iVar11 + 0xf2);
  ppcVar2 = ((iVar11 + 0xf2) + 0x10);
  (**ppcVar2)(0x1000,u_var1,(u_var1 >> 0x10));
  pBVar5 = (bool *)uVar6;
  MoveWindow16(0x1000,0x1,pBVar5[0x3],pBVar5[0x2],pBVar5[0x1],*pBVar5);
  uVar6 = param_1;
  ppcVar2 = (uVar6 + 0x94);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,iVar11,(param_1 >> 0x10),0x0);
  ppcVar2 = (uVar6 + 0x10);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,param_1,0x1);
  UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
  return;
}


pub fn win_ui_fn_1020_6e98(param_1: u32,in_win_handle: HWND16,param_3: u16)
{
  let piVar1: U32Ptr;
  let paVar2: &mut Struct18;
  let window_handle: HWND16;
  let uVar3: u16;
  let in_DX: u16;
  WVar4: WPARAM16;
  let iVar5: i16;
  let uVar6: u16;
  let LVar7: LRESULT;
  let mut pcVar8: String; 
  let uVar9: u16;
  let uVar10: u16;
  let iStack36: i16;
  let window_name: u32;
  let rectangle: RECT16;
  let IStack6: i16;
  let iStack4: i16;
  
 // uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  GetClientRect16(in_win_handle,&rectangle);
  window_name = 0x0;
  window_handle = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1797);
  if (window_handle != 0x0) {
    DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
  }
  pass1_1018_30fc((iVar5 + 0xf2),CONCAT22(param_3,&window_name),in_DX
                 );
  if ((window_name._2_2_ | window_name) != 0x0) {
    window_handle =
         CreateWindow16(0x1018,window_name,
                        CONCAT22(ctx.PTR_LOOP_1050_038c,window_name._2_2_),0x1797,
                        (iVar5 + 0x8),iStack4 + -0x19,IStack6,0x0,0x0,0x103,
                        0x40a0);
    paVar2 = window_name;
    if (window_handle == 0x0) {
      if ((window_name._2_2_ | window_name) != 0x0) {
        pass1_1018_2afa(window_name);
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
        return;
      }
    }
    else {
      LVar7 = SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
     // WVar4 = (LVar7 >> 0x10);
      if ((window_name + 0x4) == 0x0) {
        uVar9 = 0x0;
        uVar10 = 0x401;
        pcVar8 = load_string_1010_847e
                           (ctx.PTR__LOOP_1050_14cc,
                            (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
        SendMessage16(0x1010,pcVar8,(pcVar8 >> 0x10),
                      CONCAT22(uVar10,uVar9));
      }
      else {
        iStack36 = 0x0;
        loop {
          piVar1 = (window_name + 0x4);
          if (*piVar1 == iStack36 || *piVar1 < iStack36) { break; }
          uVar9 = 0x0;
          uVar10 = 0x401;
          uVar3 = pass1_1020_bd80(
                                   (window_name + iStack36 * 0x2));
          LVar7 = SendMessage16(ctx.s_tile2_bmp_1050_1538,uVar3,WVar4,
                                CONCAT22(uVar10,uVar9));
         // WVar4 = (LVar7 >> 0x10);
          iStack36 += 0x1;
        }
      }
      LVar7 = SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
     // WVar4 = (LVar7 >> 0x10);
      uVar3 = LVar7;
      uVar9 = 0xffff;
      uVar10 = 0x40d;
      pass1_1018_2d84(uVar3,(iVar5 + 0xf2));
      LVar7 = SendMessage16(0x1018,uVar3,WVar4,CONCAT22(uVar10,uVar9));
      iVar5 = LVar7;
      if ((iVar5 != -0x1) || ((LVar7 >> 0x10) != -0x1)) {
        SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x407,iVar5));
        SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x418,iVar5));
      }
      paVar2 = window_name;
      window_handle = ctx.s_tile2_bmp_1050_1538;
      if (window_name != 0x0) {
        pass1_1018_2afa(window_name);
        window_handle = 0x1000;
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
      }
      ShowWindow16(window_handle,0x1);
      SetFocus16(ctx.s_tile2_bmp_1050_1538);
    }
  }
  return;
}


pub fn unk_win_ui_op_1020_717e(param_1: U32Ptr,param_2: i32,param_3: u16)
{
  let paVar1: &mut Struct13;
  let ppcVar2: u32;
  let uVar3: u32;
  let HVar4: HPALETTE16;
  let puVar5: u32;
  let in_DX: U32Ptr;
  let uVar6: u16;
  let extraout_dx: U32Ptr;
  let puVar7: U32Ptr;
  let iVar8: i16;
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  let unaff_CS: u16;
  let puVar13: U32Ptr;
  let local_4: HDC16;
  let iVar9: &mut Struct41;
  
  get_sys_metrics_1020_7c1a(param_1,param_2,unaff_CS);
 // uVar11 = (param_1 >> 0x10);
  iVar8 = param_1;
  (iVar8 + 0x14) = 0x0;
  (iVar8 + 0x18) = param_2;
  (iVar8 + 0x1c) = 0x0;
  (iVar8 + 0x20) = 0x0;
  *param_1 = 0x754c;
  (iVar8 + 0x2) = 0x1020;
  puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x4,param_3,in_DX,unaff_DI);
 // uVar6 = (puVar13 >> 0x10);
  (iVar8 + 0x1c) = puVar13;
  (iVar8 + 0x1e) = uVar6;
  ppcVar2 = ((iVar8 + 0x1c) + 0x4);
  (**ppcVar2)(0x1010,(iVar8 + 0x1c),uVar6,0x0,param_1);
  uVar6 = (iVar8 + 0x4);
  local_4 = GetDC16(0x1010);
  uVar3 = (iVar8 + 0x1c);
  *(uVar3 + 0x178) = local_4;
  uVar3 = (iVar8 + 0x1c);
 // uVar12 = (uVar3 >> 0x10);
  iVar10 = uVar3;
  puVar5 = (iVar10 + 0x24);
  ppcVar2 = (*puVar5 + 0x14);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,puVar5,(iVar10 + 0x26),uVar6)
  ;
  puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x29,param_3,extraout_dx,unaff_DI);
 // puVar7 = (puVar13 >> 0x10);
  paVar1 = (puVar13 + 0xe);
  pass1_1008_4d84((puVar5 & 0xffff | ZEXT24(extraout_dx) << 0x10),
                  paVar1,puVar7);
  HVar4 = palette_op_1008_4e08(paVar1,&local_4,puVar7,0x1008);
  (iVar8 + 0x20) = HVar4;
  return;
}


pub fn post_win_msg_1020_7308(param_1: u32,param_2: u16,param_3: HWND16)
{
  let cVar1: u8;
  
  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar1 = param_2;
      if (cVar1 == '\x01') {
        (param_1 + 0x1c) = 0x0;
        return;
      }
      if (('\x03' < (cVar1 + -0x1)) && ((cVar1 + -0x5) < '\x02'))
//       TODO: goto LAB_1020_7310;
    }
    return;
  }
//LAB_1020_7310:
  PostMessage16(param_3,0x0,0x0,0x11100eb);
  invalidate_rect_1020_735a(param_1,s_tile2_bmp_1050_1538);
  return;
}



pub fn win_ui_op_1020_737a(param_1: i32,param_2: HWND16,param_3: u16) -> bool

{
  let u_var1: u16;
  let ppcVar2: u32;
  let uVar3: u32;
  let Bvar4: bool;
  let puVar5: U32Ptr;
  let puVar6: u32;
  let in_DX: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let paStack78: &mut Struct18;
  let local_42: [u8;6];
  let local_brush_handle: u32;
  let iStack56: i16;
  let HStack54: HWND16;
  let HStack52: HWND16;
  let iStack50: i16;
  let local_30: RECT16;
  let iStack44: i16;
  let iStack42: i16;
  let local_rect: *mut RECT16;
  let BStack38: bool;
  let local_24: HDC16;
  let local_paint_struct: PAINTSTRUCT16;
  
 // uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar11 = (iVar8 + 0x4);
  local_24 = BeginPaint16(param_2,&local_paint_struct);
  uVar10 = (iVar8 + 0x4);
  BVar4 = IsIconic16(ctx.s_tile2_bmp_1050_1538);
  if (BVar4 == 0x0) {
    uVar3 = (iVar8 + 0x1c);
    puVar6 = (uVar3 + 0x24);
    local_brush_handle = puVar6;
    pass1_1018_2e5e(param_3,puVar6,in_DX,(iVar8 + 0x1c));
    local_30 = puVar6 & 0xffff | in_DX << 0x10;
    pass1_1008_3e54(
                    CONCAT13((param_3 >> 0x8),CONCAT12(param_3,local_42)),0x0,
                    0x35,0xc);
    if ((iVar8 + 0x14) != 0x0) {
      pass1_1008_5236((iVar8 + 0x14));
    }
    if (local_30 != 0x0) {
      u_var1 = (iVar8 + 0x14);
      uVar7 = (iVar8 + 0x16);
      paStack78 = CONCAT22(uVar7,u_var1);
      if ((uVar7 | u_var1) != 0x0) {
        pass1_1008_5118(CONCAT22(uVar7,u_var1));
        fn_ptr_1000_17ce(ctx, paStack78, 0x1000);
      }
      puVar5 = local_42;
      pass1_1008_8ce4(local_30,CONCAT22(param_3,puVar5),
                      local_brush_handle,param_3);
      (iVar8 + 0x14) = puVar5;
      (iVar8 + 0x16) = uVar7;
    }
    ppcVar2 = (*local_brush_handle + 0x4);
    (**ppcVar2)(0x1008,local_brush_handle,(local_brush_handle >> 0x10),
                0x0,0x0);
    ppcVar2 = ((iVar8 + 0x18) + 0x94);
    (**ppcVar2)(0x1008,(iVar8 + 0x18),0x1);
    HStack52 = GetDlgItem16(0x1008,0x1797);
    if (HStack52 != 0x0) {
      ShowWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    }
  }
  else {
    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
      ppcVar2 = ((iVar8 + 0x1c) + 0x2c);
      (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,(iVar8 + 0x1c),uVar10,uVar11);
      BStack38 = BVar4;
      if (BVar4 != 0x0) {
        local_rect = GetStockObject16(s_tile2_bmp_1050_1538);
        GetClientRect16(ctx.s_tile2_bmp_1050_1538,&local_30);
        local_brush_handle = 0x0;
        iStack56 = (iStack44 - local_30.x) + -0x1;
        HStack54 = (iStack42 - local_30.y) - 0x1;
        HStack52 = HStack54;
        iStack50 = iStack56;
        FillRect16(ctx.s_tile2_bmp_1050_1538,local_rect,&local_brush_handle);
        DrawIcon16(ctx.s_tile2_bmp_1050_1538,BStack38,0x2,0x2);
      }
    }
  }
  BVar4 = EndPaint16(ctx.s_tile2_bmp_1050_1538,&local_paint_struct);
  return BVar4;
}


pub fn win_1020_75f0(param_1: u32,param_2: u16)
{
  let pUVar1: U32Ptr;
  let ppcVar2: u32;
  let uVar3: u16;
  let u_var4: u32;
  let puVar5: U32Ptr;
  let puVar6: U32Ptr;
  let iVar7: &mut Struct283;
  let uVar7: u16;
  let puVar8: U32Ptr;
  let puStack10: u32;
  let local_6: [u8;4];
  
 // uVar7 = (param_1 >> 0x10);
  iVar7 = param_1;
  if (iVar7.field_0xee != 0x0) {
    ppcVar2 = (*iVar7.field_0xee + 0x8);
    (**ppcVar2)();
  }
  if (iVar7.field_0xea == 0x0) {
    iVar7.field_0xea = 0x1;
    puVar8 = pass1_1008_941a(CONCAT22(param_2,local_6),0x1,0x91);
   // puVar5 = (puVar8 >> 0x10);
    u_var4 = ZEXT24(local_6);
    win_1008_5c9e(ctx.PTR__LOOP_1050_02a0,CONCAT22(param_2,local_6),local_6,puVar5,
                  param_2);
    iVar7.field_0xec = u_var4;
    mem_op_1000_179c(0x112,puVar5,0x1000);
    puVar6 = (puVar5 | u_var4);
    if (puVar6 == 0x0) {
      uVar3 = 0x0;
      puStack10 = 0x0;
    }
    else {
      pUVar1 = &iVar7.field_0xcc;
      *pUVar1 = *pUVar1 + 0x1;
      struct_1020_3644((u_var4 & 0xffff | ZEXT24(puVar5) << 0x10),
                       iVar7.field_0xcc,param_1,param_2);
      uVar3 = u_var4;
      puStack10 = (u_var4 & 0xffff | ZEXT24(puVar6) << 0x10);
    }
    pass1_1008_6978(param_1,0x0,puStack10 & 0xffff0000 | uVar3,uVar3,puVar6)
    ;
    ppcVar2 = (*puStack10 + 0xc);
    (**ppcVar2)();
  }
  return;
}



pub fn window_op_1020_76aa(astruct *param_1)
{
  let in_AX: &mut Struct666;
  let in_DX: U32Ptr;
  let uVar3: u32;
  let i_var1: i16;
  let unaff_DI: i16;
  let u_var2: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
 // u_var2 = (param_1 >> 0x10);
  i_var1 = param_1;
  get_dc_1018_4db0((i_var1 + 0xf2),(i_var1 + 0x8),0x1018);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar3 = in_DX | in_AX;
  if (uVar3 != 0x0) {
    pass1_1020_7824(in_AX,in_DX,(i_var1 + 0x8),unaff_DI,unaff_SS);
    (i_var1 + 0xee) = in_AX;
    (i_var1 + 0xf0) = uVar3;
    return;
  }
  (i_var1 + 0xee) = 0x0;
  return;
}


pub fn
post_win_msg_1020_79fc
          (param_1: &mut Struct69,param_2: u16,param_3: u16,param_4: i16,param_5: HWND16)

{
  let pu_var1: u32;
  let ppcVar2: u32;
  let iVar3: i16;
  let i_var4: &mut Struct69;
  let u_var4: u16;
  let u_var5: u16;
  
 // u_var4 = (param_1 >> 0x10);
  i_var4 = param_1;
  ppcVar2 = (*i_var4.field_0xe0 + 0x24);
  iVar3 = (**ppcVar2)(param_5,i_var4.field_0xe0);
  if (iVar3 != param_4) {
    u_var5 = i_var4.field_0x8;
    PostMessage16(param_5,0x0,0x0,0x850000);
    pu_var1 = i_var4.field_0xe0;
    ppcVar2 = (*i_var4.field_0xe0 + 0x28);
    (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,pu_var1,(pu_var1 >> 0x10),
                param_4,u_var5);
  }
  return;
}



pub fn get_win_ui_info_op_1020_7a50(param_1: u32,param_2: HWND16)
{
  let ppcVar1: u32;
  let b_var2: bool;
  let IVar2: i16;
  let IVar3: i16;
  let var5: u16;
  let local_a: RECT16;
  let iStack6: i16;
  let iStack4: i16;
  
  local_a.x = 0x0;
  local_a.y = 0x0;
  iStack6 = 0x0;
  iStack4 = 0x0;
 // var5 = (param_1 >> 0x10);
  b_var2 = IsIconic16(param_2);
  if (b_var2 == 0x0) {
    GetWindowRect16(ctx.s_tile2_bmp_1050_1538,&local_a);
    iStack6 -= local_a.x;
    iStack4 -= local_a.y;
    IVar2 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    IVar3 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    local_a.x += IVar2 * 0x2;
    local_a.y += IVar3 * 0x2;
  }
  ppcVar1 = ((param_1 + 0xe0) + 0x14);
  (**ppcVar1)(ctx.s_tile2_bmp_1050_1538,(param_1 + 0xe0),&local_a);
  return;
}



pub fn win_ui_menu_op_1020_7ad2(param_1: u32,param_2: HWND16,param_3: &RECT16,param_4: HWND16)
{
  let HVar1: HMENU16;
  let iVar2: i16;
  let uVar3: u16;
  POlet local_6: i16;
  
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0xee) != 0x0) && ((iVar2 + 0xec) == 0x0)) {
    HVar1 = LoadMenu16(param_4,(iVar2 + 0xee));
    (iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    param_4 = ctx.s_tile2_bmp_1050_1538;
    HVar1 = GetSubMenu16(ctx.s_tile2_bmp_1050_1538,0x0);
    (iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  local_6.x = param_3;
  local_6.y = param_2;
  ClientToScreen16(param_4,&local_6);
  TrackPopupMenu16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,(iVar2 + 0x8),0x0,
                   local_6.y,local_6.x);
  return;
}


