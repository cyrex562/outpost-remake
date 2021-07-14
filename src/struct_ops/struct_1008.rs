use crate::bad::bad_1030_1312;
use crate::defines::{Struct18, Struct19, Struct20, Struct76, Struct79};
use crate::file::file_1008::{read_file_1008_7cfe, read_file_1008_7dee, write_to_file_1008_7cac};
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_0a48, mem_op_1000_160a, mem_op_1000_179c};
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_3d7a, pass1_1000_4906};
use crate::pass::pass_1008::{pass1_1008_4016, pass1_1008_47cc, pass1_1008_4834, pass1_1008_5784, pass1_1008_5b12, pass1_1008_909c};
use crate::pass::pass_1010::pass1_1010_1d80;
use crate::pass::pass_1020::{pass1_1020_a43e, pass1_1020_a6ee, pass1_1020_bd80, string_op_1020_c222};
use crate::pass::pass_1028::{pass1_1028_dc52, pass1_1028_e4ec};
use crate::pass::pass_1030::{pass1_1030_38f2, pass1_1030_4bbe};
use crate::pass::pass_1038::load_string_1038_4d28;
use crate::string::string_1000::unk_str_op_1000_3d3e;
use crate::string::string_1008::str_op_1008_60e8;
use crate::string::string_1010::load_string_1010_84ac;
use crate::struct_ops::struct_1010::{struct_1010_2cd2, struct_1010_383a};
use crate::struct_ops::struct_1018::{struct_1018_47c8, struct_1018_4842, struct_1018_48b0, struct_1018_4920};
use crate::ui::ui_1008::set_sys_color_1008_357e;
use crate::util::{CONCAT22, SUB42, ZEXT24};
use crate::win_struct::{HCURSOR16, HFILE16, HGDIOBJ16, HICON16, HINSTANCE16};
use crate::winapi::{GetStockObject16, LoadCursor16, LoadIcon16};

pub fn struct_op_1008_0000(param_1: *mut u16)
{
  let i_var1: i16;
  let u_var2: u16;
  i_var1 = param_1;
  *param_1 = 0x52a;
  (i_var1 + 0x2) = 0x1008;
  (i_var1 + 0x4) = 0x0;
  (i_var1 + 0x8) = 0x0;
  *param_1 = 0x51e;
  (i_var1 + 0x2) = 0x1008;
  return;
}


pub fn set_struct_op_1008_0536(param_1: *mut u16,param_2: HINSTANCE16,param_3: u16)
{
  let HVar1: HICON16;
  let HVar2: HCURSOR16;
  let HVar3: HGDIOBJ16;
  let puVar4: *mut u8;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  let paVar7: &mut Struct20;
  let puVar8: *mut u16;
  
  paVar7 = pass1_1008_3ab8(param_1);
 // puVar4 = (paVar7 >> 0x10);
 // uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  (iVar5 + 0xe0) = 0x0;
  (iVar5 + 0xe4) = 0x0;
  (iVar5 + 0xe8) = 0x0;
  (iVar5 + 0xec) = 0x0;
  (iVar5 + 0xee) = 0x0;
  (iVar5 + 0xf2) = 0x0;
  (iVar5 + 0xf4) = 0x0;
  (iVar5 + 0xf8) = 0x0;
  *param_1 = 0x389e;
  (iVar5 + 0x2) = 0x1008;
  (iVar5 + 0xc8) = 0x2008;
  (iVar5 + 0xac) = 0x0;
  (iVar5 + 0xae) = 0x8700;
  HVar1 = LoadIcon16(param_2,0xd4);
  (iVar5 + 0xc2) = HVar1;
  HVar2 = LoadCursor16(s_tile2_bmp_1050_1538,0x7f00);
  (iVar5 + 0xc4) = HVar2;
  HVar3 = GetStockObject16(s_tile2_bmp_1050_1538);
  (iVar5 + 0xc6) = HVar3;
  puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x48,param_3,puVar4,unaff_DI);
 // puVar4 = (puVar8 >> 0x10);
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (iVar5 + 0xa)),
             s_Outpost_1050_00d7);
  puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x32,param_3,puVar4,unaff_DI);
  (iVar5 + 0xf4) = puVar8;
  (iVar5 + 0xf6) = (puVar8 >> 0x10);
  set_sys_color_1008_357e(param_1,0x1,0x1010,param_3);
  return;
}


pub fn struct_op_1008_3f92(param_1: &mut Struct76,param_2: &mut Struct83)
{
  let ppcVar1: u32;
  let iVar2: &mut Struct76;
  let uVar2: u16;
  
  struct_op_1008_56b4(param_1);
 // uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  &iVar2.field_0x6 = 0x0;
  (&iVar2.field_0x8 + 0x2) = 0x0;
  &iVar2.field_0xe = 0x0;
  (&iVar2.field_0xe + 0x2) = 0x0;
  iVar2.field_0x14 = 0x0;
  &iVar2.field_0x18 = 0x0;
  iVar2.field_0x1c = 0x0;
  param_1.field_0x0 = &ctx.PTR_LOOP_1050_48de;
  iVar2.field_0x2 = 0x1008;
  if (param_2 == 0x0) {
    return;
  }
  ppcVar1 = (param_2 + 0x8);
  (**ppcVar1)();
  struct_op_1008_4214(param_1,param_2);
  pass1_1008_47cc(param_1);
  pass1_1008_4834(param_1);
  return;
}


pub fn struct_op_1008_4214(param_1: &mut Struct76,param_2: &mut Struct83)
{
  let puVar1: u32;
  let uVar2: u16;
  let ppcVar3: u32;
  let iVar4: &mut Struct83;
  let uVar4: &mut Struct83;
  
 // uVar4 = (param_2 >> 0x10);
  iVar4 = param_2;
  (param_1 + 0x6) = iVar4.field_0x1a;
  iVar4.field_0x1a = 0x0;
  puVar1 = iVar4.field_0x4;
  uVar2 = iVar4.field_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field_0x4 = 0x0;
  iVar4.field_0xe = 0x0;
  iVar4.field_0x12 = 0x0;
  iVar4.field_0x16 = 0x0;
  iVar4.field_0x1e = 0x0;
  return;
}


pub fn struct_op_1008_48fe(param_1: &mut Struct81,param_2: u16,param_3: &mut String,param_4: u16)
{
  let uVar1: u16;
  let iVar2: &mut Struct81;
  let uVar3: u16;
  
 // uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1 = 0x389a;
  iVar2.field_0x2 = 0x1008;
  iVar2.field_0x4 = 0x0;
  &iVar2.field_0x8 = 0x0;
  iVar2.field_0xc = 0xffff;
  iVar2.field_0xe = 0x0;
  iVar2.field_0x12 = 0x0;
  iVar2.field_0x16 = 0x0;
  iVar2.field_0x1a = 0x0;
  iVar2.field_0x1e = 0x0;
  iVar2.field_0x22 = param_2;
  param_1 = &ctx.PTR_LOOP_1050_4c4c;
  iVar2.field_0x2 = 0x1008;
  uVar1 = str_op_1008_60e8(param_3,param_4);
  iVar2.field_0x8 = uVar1;
  iVar2.field_0xa = param_4;
  return;
}



pub fn struct_1008_4c58(param_1: *mut u16)
{
  let iVar1: &mut Struct394;
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = 0x0;
  iVar1.field_0xc = 0x0;
  iVar1.field_0xe = 0x0;
  iVar1.field_0x12 = 0x1;
  *param_1 = 0x4f1c;
  iVar1.field_0x2 = 0x1008;
  return;
}



pub fn struct_op_1008_4c98(param_1: &mut Struct76,param_2: u32,param_3: u16)
{
  let iVar1: &mut Struct76;
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field_0x0 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  &iVar1.field_0x4 = param_2;
  iVar1.field_0xc = param_3;
  iVar1.field_0xe = 0x0;
  iVar1.field_0x12 = 0x0;
  param_1.field_0x0 = 0x4f1c;
  iVar1.field_0x2 = 0x1008;
  return;
}


pub fn struct_op_1008_56b4(param_1: &mut Struct76) -> u16

{
  let iVar1: &mut Struct82;
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field_0x0 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = 0x0;
  param_1.field_0x0 = s__s__d_1050_573a;
  iVar1.field_0x2 = 0x1008;
  return param_1;
}



pub fn set_struct_1008_574a(param_1: &mut Struct21)
{
  let iVar1: &mut Struct21;
  let uVar1: &mut Struct21;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field_0x0 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = 0x0;
  iVar1.field_0x8 = 0x0;
  iVar1.field_0xa = 0x1;
  param_1.field_0x0 = 0x5bc4;
  iVar1.field_0x2 = 0x1008;
  return;
}


pub fn struct_op_1008_6604(param_1: &mut Struct85,param_2: i16,param_3: i16)
{
  let puVar1: u32;
  let iVar3: i16;
  let iVar4: &mut Struct85;
  let iVar2: &mut Struct84;
  let uVar4: u16;
  let uVar5: u16;
  let lVar6: i32;
  
  pass1_1008_4016(ctx, param_1);
 // uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  param_1 = 0x685a;
  iVar4.field_0x2 = 0x1008;
  lVar6 = mem_op_1000_0a48(0x1,0x28,0x0,_PTR_LOOP_1050_5f2c,0x1000);
  &iVar4.field_0x10 = lVar6;
  (&iVar4.field_0x10 + 0x2) = (lVar6 >> 0x10);
  iVar3 = param_3 * 0x8 + 0x1f;
  iVar3 = ((iVar3 + (iVar3 >> 0xf & 0x1f)) >> 0x5) << 0x2;
  &iVar4.field_0x18 = iVar3;
  (&iVar4.field_0x18 + 0x2) = iVar3 >> 0xf;
  lVar6 = mem_op_1000_0a48(0x1,(iVar3 * param_2),
                           ((iVar3 * param_2) >> 0x10),
                           ctx._PTR_LOOP_1050_5f2c,0x1000);
 // uVar5 = (lVar6 >> 0x10);
  iVar4.field_0x6 = lVar6;
  iVar4.field_0x8 = uVar5;
  iVar4.field_0x14 = iVar4.field_0x6;
  iVar4.field_0x16 = uVar5;
  *iVar4.field_0x10 = 0x28;
  puVar1 = iVar4.field_0x10;
  (puVar1 + 0x4) = param_3;
  puVar1 = iVar4.field_0x10;
 // uVar5 = (puVar1 >> 0x10);
  iVar2 = puVar1;
  iVar2.field_0x8 = param_2;
  iVar2.field_0xa = param_2 >> 0xf;
  puVar1 = iVar4.field_0x10;
  (puVar1 + 0xc) = 0x1;
  puVar1 = iVar4.field_0x10;
  (puVar1 + 0xe) = 0x8;
  puVar1 = iVar4.field_0x10;
  (puVar1 + 0x10) = 0x0;
  puVar1 = iVar4.field_0x10;
  (puVar1 + 0x14) = iVar4.field_0x18 * param_2;
  puVar1 = iVar4.field_0x10;
  (puVar1 + 0x20) = 0x100;
  puVar1 = iVar4.field_0x10;
  (puVar1 + 0x24) = 0x100;
  return;
}


pub fn set_struct_1008_687a(param_1: &mut Struct20,param_2: i32)
{
  let iVar1: &mut Struct20;
  let uVar1: &mut Struct20;
  
  set_struct_op_1008_9584(param_1,param_2);
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field_0xcc = 0x0;
  iVar1.field_0xce = 0x0;
  set_struct_1008_574a
            ((param_1 & 0xffff0000 | ZEXT24(&iVar1.field_0xd2)));
  param_1.field_0x0 = 0x6bfc;
  iVar1.field_0x2 = 0x1008;
  (iVar1.field_0xd2).field_0xa = 0x0;
  return;
}


pub fn struct_op_1008_8e9e(param_1: &mut Struct78,param_2: u32,param_3: u32)
{
  let iVar1: &mut Struct78;
  let uVar1: &mut Struct78;
  let unaff_SS: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field_0x0 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = 0x0;
  iVar1.field_0x6 = 0x0;
  iVar1.field_0xa = 0x0;
  iVar1.field_0xe = param_3;
  iVar1.field_0x12 = 0x0;
  iVar1.field_0x16 = param_2;
  iVar1.field_0x1a = 0x1;
  param_1.field_0x0 = 0x9170;
  iVar1.field_0x2 = 0x1008;
  if (iVar1.field_0xe < 0x7) {
    iVar1.field_0xe = 0x6;
  }
  pass1_1008_909c(param_1,unaff_SS);
  *iVar1.field_0x6 = 0x0;
  return;
}


pub fn struct_op_1008_9174(param_1: &mut Struct88,param_2: u32,param_3: u32)
{
  let iVar1: &mut Struct88;
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = param_3;
  iVar1.field_0x8 = param_2;
  iVar1.field_0xc = param_2;
  iVar1.field_0x10 = 0x0;
  param_1 = 0x9412;
  iVar1.field_0x2 = 0x1008;
  return;
}


pub fn set_struct_op_1008_9584(param_1: &mut Struct20,param_2: i32)
{
  let iVar1: &mut Struct20;
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field_0x0 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = param_2;
  param_1.field_0x0 = 0x9d2e;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x8 = 0x0;
  iVar1.field_0xac = 0x2000000;
  iVar1.field_0xb0 = 0x0;
  iVar1.field_0xb4 = 0x8000;
  iVar1.field_0xb6 = 0x8000;
  iVar1.field_0xb8 = 0x8000;
  iVar1.field_0xba = 0x8000;
  iVar1.field_0xbc = 0x0;
  iVar1.field_0xbe = 0x0;
  iVar1.field_0xc2 = 0x0;
  iVar1.hcursor_field_0xc4 = 0x0;
  iVar1.hgdiobj_field_0xc6 = 0x0;
  iVar1.field_0xc8 = 0x2008;
  iVar1.field_0xca = 0x0;
  param_1.field_0x0 = 0x380a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x5b = '\0';
  *&iVar1.field_0xa = 0x0;
  return;
}


pub fn struct_1008_9fd2(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16)
{
  let uVar1: u16;
  let puVar2: *mut u8;
  let puVar3: *mut u8;
  let extraout_DX: *mut u8;
  let extraout_DX_00: u16;
  let uVar4: u16;
  let paVar5: &mut Struct79;
  
  paVar5 = struct_op_1010_1d48(CONCAT22(param_2,param_1),param_3);
 // puVar2 = (paVar5 >> 0x10);
  uVar1 = 0x0;
  (param_1 + 0x1) = 0x0;
  (param_1 + 0x68) = 0x0;
  &param_1[0x68].field_0x4 = 0x0;
  (&param_1[0x68].field_0x4 + 0x2) = 0x0;
  param_1[0x68].field_0x8 = 0x0;
  ((param_1 + 0x69)).field_0x0 = 0x0;
  param_1[0x69].field_0x2 = 0x0;
  &param_1[0x69].field_0x4 = 0x0;
  CONCAT22(param_2,param_1) = 0xad92;
  param_1.field_0x2 = 0x1008;
  mem_op_1000_179c(0xc,puVar2,0x1000);
  puVar3 = (puVar2 | uVar1);
  if (puVar3 == 0x0) {
    (param_1 + 0x1) = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(puVar2,uVar1));
    ((param_1 + 0x1)).field_0x0 = uVar1;
    param_1[0x1].field_0x2 = extraout_DX;
    puVar3 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar3,0x1000);
  if ((puVar3 | uVar1) == 0x0) {
    uVar1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(puVar3,uVar1));
    uVar4 = extraout_DX_00;
  }
  ((param_1 + 0x68)).field_0x0 = uVar1;
  param_1[0x68].field_0x2 = uVar4;
  return;
}


pub fn struct_1008_bde0(param_1: *mut u32,param_2: *mut u8)
{
  let uVar1: u16;
  let iVar2: &mut Struct139;
  let iVar3: &mut Struct140;
  let iVar4: &mut Struct141;
  let iVar5: &mut Struct142;
  let iVar6: &mut Struct143;
  let iVar7: &mut Struct144;
  let iVar8: &mut Struct145;
  let iVar9: &mut Struct146;
  let iVar10: &mut Struct147;
  let iVar11: &mut Struct148;
  let iVar12: &mut Struct149;
  let iVar2_00: &mut Struct150;
  let iVar2_01: &mut Struct151;
  let iVar2_02: &mut Struct152;
  let iVar2_03: &mut Struct153;
  let iVar2_04: &mut Struct154;
  let iVar2_05: &mut Struct155;
  let iVar2_06: i16;
  let uVar3: u16;
  let uVar13: u16;
  
  ctx._PTR_LOOP_1050_06e0 = param_1;
  if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_2,0x1000);
    ctx.PTR_LOOP_1050_5f2e = param_2;
  }
  else {
  }
  uVar1 = fn_ptr_op_1000_1708(0x1aa,0x0,0x1,PTR_LOOP_1050_5f2c,
                              ctx.PTR_LOOP_1050_5f2e,0x1000);
  param_1 = uVar1;
  (param_1 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
  uVar3 = (*param_1 >> 0x10);
  iVar2 = *param_1;
  iVar2.field_0x6 = 0x6e4;
  iVar2.field_0x8 = ctx.data_seg;
  (*param_1 + 0xa) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar3 = *param_1;
  iVar3.field_0xc = 0x6ea;
  iVar3.field_0xe = ctx.data_seg;
  (*param_1 + 0x10) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar4 = *param_1;
  iVar4.field_0x12 = 0x6ee;
  iVar4.field_0x14 = ctx.data_seg;
  (*param_1 + 0x16) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar5 = *param_1;
  iVar5.field_0x18 = 0x6f2;
  iVar5.field_0x1a = ctx.data_seg;
  (*param_1 + 0x1c) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar6 = *param_1;
  iVar6.field_0x1e = 0x6f6;
  iVar6.field_0x20 = ctx.data_seg;
  (*param_1 + 0x22) = 0x4;
  uVar13 = (*param_1 >> 0x10);
  iVar7 = *param_1;
  iVar7.field_0x24 = 0x6fe;
  iVar7.field_0x26 = ctx.data_seg;
  (*param_1 + 0x28) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar8 = *param_1;
  iVar8.field_0x2a = 0x702;
  iVar8.field_0x2c = ctx.data_seg;
  (*param_1 + 0x2e) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar9 = *param_1;
  iVar9.field_0x30 = 0x708;
  iVar9.field_0x32 = ctx.data_seg;
  (*param_1 + 0x34) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar10 = *param_1;
  iVar10.field_0x36 = 0x70e;
  iVar10.field_0x38 = ctx.data_seg;
  (*param_1 + 0x3a) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar11 = *param_1;
  iVar11.field_0x3c = 0x714;
  iVar11.field_0x3e = ctx.data_seg;
  (*param_1 + 0x40) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar12 = *param_1;
  iVar12.field_0x42 = 0x71a;
  iVar12.field_0x44 = ctx.data_seg;
  (*param_1 + 0x46) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_00 = *param_1;
  iVar2_00.field_0x48 = 0x71e;
  iVar2_00.field_0x4a = ctx.data_seg;
  (*param_1 + 0x4c) = 0x7;
  uVar13 = (*param_1 >> 0x10);
  iVar2_01 = *param_1;
  iVar2_01.field_0x4e = 0x72c;
  iVar2_01.field_0x50 = ctx.data_seg;
  (*param_1 + 0x52) = 0x6;
  uVar13 = (*param_1 >> 0x10);
  iVar2_02 = *param_1;
  iVar2_02.field_0x54 = 0x738;
  iVar2_02.field_0x56 = ctx.data_seg;
  (*param_1 + 0x58) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_03 = *param_1;
  iVar2_03.field_0x5a = 0x73e;
  iVar2_03.field_0x5c = ctx.data_seg;
  (*param_1 + 0x5e) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_04 = *param_1;
  iVar2_04.field_0x60 = 0x744;
  iVar2_04.field_0x62 = ctx.data_seg;
  (*param_1 + 0x64) = 0x4;
  uVar13 = (*param_1 >> 0x10);
  iVar2_05 = *param_1;
  iVar2_05.field_0x66 = 0x74c;
  iVar2_05.field_0x68 = ctx.data_seg;
  (*param_1 + 0x6a) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x6c) = 0x750;
  (iVar2_06 + 0x6e) = ctx.data_seg;
  (*param_1 + 0x70) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x72) = 0x756;
  (iVar2_06 + 0x74) = ctx.data_seg;
  (*param_1 + 0x76) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x78) = 0x75a;
  (iVar2_06 + 0x7a) = ctx.data_seg;
  (*param_1 + 0x7c) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x7e) = 0x75e;
  (iVar2_06 + 0x80) = ctx.data_seg;
  (*param_1 + 0x82) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x84) = 0x764;
  (iVar2_06 + 0x86) = ctx.data_seg;
  (*param_1 + 0x88) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x8a) = 0x76a;
  (iVar2_06 + 0x8c) = ctx.data_seg;
  (*param_1 + 0x8e) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x90) = 0x770;
  (iVar2_06 + 0x92) = ctx.data_seg;
  (*param_1 + 0x94) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x96) = 0x774;
  (iVar2_06 + 0x98) = ctx.data_seg;
  (*param_1 + 0x9a) = 0x4;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x9c) = 0x77c;
  (iVar2_06 + 0x9e) = ctx.data_seg;
  (*param_1 + 0xa0) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xa2) = 0x780;
  (iVar2_06 + 0xa4) = ctx.data_seg;
  (*param_1 + 0xa6) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xa8) = 0x782;
  (iVar2_06 + 0xaa) = ctx.data_seg;
  (*param_1 + 0xac) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xae) = 0x786;
  (iVar2_06 + 0xb0) = ctx.data_seg;
  (*param_1 + 0xb2) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xb4) = 0x78a;
  (iVar2_06 + 0xb6) = ctx.data_seg;
  (*param_1 + 0xb8) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xba) = 0x78e;
  (iVar2_06 + 0xbc) = ctx.data_seg;
  (*param_1 + 0xbe) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xc0) = 0x792;
  (iVar2_06 + 0xc2) = ctx.data_seg;
  (*param_1 + 0xc4) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xc6) = 0x796;
  (iVar2_06 + 0xc8) = ctx.data_seg;
  (*param_1 + 0xca) = 0x4;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xcc) = 0x79e;
  (iVar2_06 + 0xce) = ctx.data_seg;
  (*param_1 + 0xd0) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xd2) = 0x7a0;
  (iVar2_06 + 0xd4) = ctx.data_seg;
  (*param_1 + 0xd6) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xd8) = 0x7a4;
  (iVar2_06 + 0xda) = ctx.data_seg;
  (*param_1 + 0xdc) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xde) = 0x7a6;
  (iVar2_06 + 0xe0) = ctx.data_seg;
  (*param_1 + 0xe2) = 0x6;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xe4) = 0x7b2;
  (iVar2_06 + 0xe6) = ctx.data_seg;
  (*param_1 + 0xe8) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xea) = 0x7b4;
  (iVar2_06 + 0xec) = ctx.data_seg;
  (*param_1 + 0xee) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xf0) = 0x7ba;
  (iVar2_06 + 0xf2) = ctx.data_seg;
  (*param_1 + 0xf4) = 0x2d;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xf6) = 0x814;
  (iVar2_06 + 0xf8) = ctx.data_seg;
  (*param_1 + 0xfa) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0xfc) = 0x81a;
  (iVar2_06 + 0xfe) = ctx.data_seg;
  (*param_1 + 0x100) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x102) = 0x81c;
  (iVar2_06 + 0x104) = ctx.data_seg;
  (*param_1 + 0x106) = 0x4b;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x108) = 0x8b2;
  (iVar2_06 + 0x10a) = ctx.data_seg;
  (*param_1 + 0x10c) = 0x6;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x10e) = 0x8be;
  (iVar2_06 + 0x110) = ctx.data_seg;
  (*param_1 + 0x112) = 0x4;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x11a) = 0x8c6;
  (iVar2_06 + 0x11c) = ctx.data_seg;
  (*param_1 + 0x11e) = 0x35;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x120) = 0x930;
  (iVar2_06 + 0x122) = ctx.data_seg;
  (*param_1 + 0x124) = 0x2e;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x114) = 0x98c;
  (iVar2_06 + 0x116) = ctx.data_seg;
  (*param_1 + 0x118) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x126) = 0x98e;
  (iVar2_06 + 0x128) = ctx.data_seg;
  (*param_1 + 0x12a) = 0x9;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x12c) = 0x9a0;
  (iVar2_06 + 0x12e) = ctx.data_seg;
  (*param_1 + 0x130) = 0x1a;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x132) = 0x9d4;
  (iVar2_06 + 0x134) = ctx.data_seg;
  (*param_1 + 0x136) = 0x8;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x138) = 0x9e4;
  (iVar2_06 + 0x13a) = ctx.data_seg;
  (*param_1 + 0x13c) = 0x4a;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x144) = 0xa78;
  (iVar2_06 + 0x146) = ctx.data_seg;
  (*param_1 + 0x148) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x14a) = 0xa7c;
  (iVar2_06 + 0x14c) = ctx.data_seg;
  (*param_1 + 0x14e) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x150) = 0xa7e;
  (iVar2_06 + 0x152) = ctx.data_seg;
  (*param_1 + 0x154) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x156) = 0xa80;
  (iVar2_06 + 0x158) = ctx.data_seg;
  (*param_1 + 0x15a) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x15c) = 0xa86;
  (iVar2_06 + 0x15e) = ctx.data_seg;
  (*param_1 + 0x160) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x168) = 0xa8a;
  (iVar2_06 + 0x16a) = ctx.data_seg;
  (*param_1 + 0x16c) = 0x1b;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x16e) = 0xac0;
  (iVar2_06 + 0x170) = ctx.data_seg;
  (*param_1 + 0x172) = 0x16;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x174) = 0xaec;
  (iVar2_06 + 0x176) = ctx.data_seg;
  (*param_1 + 0x178) = 0x3e;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x17a) = 0xb68;
  (iVar2_06 + 0x17c) = ctx.data_seg;
  (*param_1 + 0x17e) = 0x46;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x180) = 0xbf4;
  (iVar2_06 + 0x182) = ctx.data_seg;
  (*param_1 + 0x184) = 0x1;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x186) = 0xbf6;
  (iVar2_06 + 0x188) = ctx.data_seg;
  (*param_1 + 0x18a) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x18c) = 0xbfc;
  (iVar2_06 + 0x18e) = ctx.data_seg;
  (*param_1 + 0x190) = 0x3;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x192) = 0xc02;
  (iVar2_06 + 0x194) = ctx.data_seg;
  (*param_1 + 0x196) = 0xa;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x198) = 0xc16;
  (iVar2_06 + 0x19a) = ctx.data_seg;
  (*param_1 + 0x19c) = 0x24;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x19e) = 0xc5e;
  (iVar2_06 + 0x1a0) = ctx.data_seg;
  (*param_1 + 0x1a2) = 0x2;
  uVar13 = (*param_1 >> 0x10);
  iVar2_06 = *param_1;
  (iVar2_06 + 0x1a4) = 0xc62;
  (iVar2_06 + 0x1a6) = ctx.data_seg;
  (*param_1 + 0x1a8) = 0x44;
  return;
}


pub fn pass1_1008_c626(param_1: *mut u32)
{
  ctx._PTR_LOOP_1050_06e0 = 0x0;
  fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16  pass1_1008_c646(param_1: u16,param_2: u32,param_3: u16)

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let puVar3: *mut u8;
  let unaff_DI: i16;
  let puVar4: u32;
  let puVar5: *mut u16;
  let iStack18: i16;
  let iStack16: i16;
  
  puVar4 = pass1_1008_c6fa(CONCAT22(param_2,param_1),(param_2 >> 0x10));
 // puVar3 = (puVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x35,param_3,puVar3,unaff_DI);
  iStack18 = 0x0;
  iStack16 = 0x0;
  while ((piVar1 = (puVar4 + 0x4), iVar2 = iStack16,
         *piVar1 != iStack18 && iStack18 <= *piVar1 &&
         (iVar2 = (*puVar4 + iStack18 * 0x2),
         (iVar2 * 0x2 + puVar5 + 0xa) == 0x0))) {
    iStack18 += 0x1;
  }
  iStack16 = iVar2;
  return iStack16;
}



pub fn pass1_1008_c6ae(param_1: u32,param_2: i16,param_3: i16) -> bool

{
  let piVar1: *mut i16;
  let puVar2: u32;
  let iStack8: i16;
  
  puVar2 = pass1_1008_c6fa(param_1,param_3);
  iStack8 = 0x0;
  loop {
    piVar1 = (puVar2 + 0x4);
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
      return 0x0;
    }
    if ((*puVar2 + iStack8 * 0x2) == param_2) { break; }
    iStack8 += 0x1;
  }
  return 0x1;
}



pub fn pass1_1008_c6fa(param_1: &mut i16,param_2: i16) -> u32

{
  if ((0x0 < param_2) && (param_2 < 0x47)) {
    return CONCAT22((param_1 + 0x2),param_2 * 0x6 + *param_1)
    ;
  }
  return 0x0;
}



pub fn pass1_1008_c72a(param_1: &mut Struct642,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48(CONCAT22(param_2,param_1),param_3);
  param_1.field_0xa = 0x0;
  param_1.field_0xe = 0x0;
  CONCAT22(param_2,param_1) = 0xca4a;
  param_1.field_0x2 = 0x1008;
  return;
}



pub fn pass1_1008_c75c(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let ppcVar3: u32;
  let iVar4: &mut Struct469;
  let uVar4: u16;
  
 // uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0xca4a;
  iVar4.field_0x2 = 0x1008;
  puVar1 = iVar4.field_0xa;
  uVar2 = iVar4.field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_c79a(param_1: u32,param_2: u32,param_3: i16,param_4: u16,param_5: u8)
{
  let puVar1: *mut u8;
  let iVar2: i16;
  let uVar3: u32;
  let extraout_DX: u16;
  let puVar4: *mut u8;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let local_12: [u8;4];
  let uStack14: u32;
  let local_a: [u8;8];
  
 // uVar6 = (param_1 >> 0x10);
  pass1_1008_5784(CONCAT22(param_4,local_a),(param_1 + 0xa));
  loop {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    uStack14 = CONCAT22(extraout_DX,puVar1);
    puVar4 = (extraout_DX | puVar1);
    if (puVar4 == 0x0) { break; }
    iVar2 = pass1_1000_3d7a((puVar1 + 0x4),param_2);
    if (iVar2 == 0x0) {
      puVar7 = pass1_1020_a43e(param_4,puVar4,CONCAT22(param_4,local_12));
     // uVar5 = (puVar7 >> 0x10);
      pass1_1020_a6ee(CONCAT22(param_4,local_12),(uStack14 + 0x12),
                      local_12,uVar5,param_3,param_4,param_5);
      uVar3 = (ctx.PTR__LOOP_1050_65e2 + 0x52);
      pass1_1030_4bbe(param_4,uVar5,uVar3,(uStack14 + 0x12));
      (param_1 + 0xe) =
           (uVar3 + 0x94) + *_PTR_LOOP_1050_65e2;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_c83a(param_1: u32)
{
  if (*_PTR_LOOP_1050_65e2 <= (param_1 + 0xe)) {
    return;
  }
  return;
}



pub fn pass1_1008_c85e(param_1: u32,param_2: u16) -> u32

{
  let iVar1: i16;
  let uVar2: u16;
  
 // uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0xa) == 0x0) {
    pass1_1008_c882(param_1 & 0xffff | uVar2 << 0x10,param_2);
  }
  return CONCAT22((iVar1 + 0xc),(iVar1 + 0xa));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_c882(param_1: u32,param_2: u16)
{
  let piVar1: *mut i16;
  let puVar2: u32;
  let uVar3: u16;
  let puVar4: u32;
  let ppcVar5: u32;
  let uVar6: u16;
  let uVar7: u16;
  let puVar8: *mut u8;
  let extraout_DX: *mut u8;
  let puVar9: *mut u8;
  let puVar10: *mut u8;
  let uVar11: u16;
  let iVar9: &mut Struct201;
  let unaff_DI: i16;
  let uVar12: u16;
  let uVar13: u16;
  let paVar14: &mut Struct21;
  let uVar15: u32;
  let puVar16: *mut u16;
  let puVar17: u32;
  let iStack16: i16;
  
 // uVar12 = (param_1 >> 0x10);
  iVar9 = param_1;
                    // WARNING: Load size is inaccurate
  puVar2 = iVar9.field_0xa;
  uVar11 = (&iVar9.field_0xa + 0x2);
  paVar14 = CONCAT22(uVar11,puVar2);
  if ((uVar11 | puVar2) != 0x0) {
    ppcVar5 = *puVar2;
    paVar14 = (**ppcVar5)();
  }
  mem_op_1000_179c(0xc,(paVar14 >> 0x10),0x1000);
  if (paVar14 == 0x0) {
    uVar15 = 0x0;
  }
  else {
    uVar15 = set_struct_1008_574a(paVar14);
  }
 // puVar9 = (uVar15 >> 0x10);
  &iVar9.field_0xa = uVar15;
  (&iVar9.field_0xa + 0x2) = puVar9;
  puVar16 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x35,param_2,puVar9,unaff_DI);
  puVar17 = pass1_1008_c6fa(ctx.PTR__LOOP_1050_06e0,0x44);
 // puVar8 = (puVar17 >> 0x10);
  iStack16 = 0x0;
  puVar9 = puVar8;
  loop {
    piVar1 = (puVar17 + 0x4);
    if (*piVar1 == iStack16 || *piVar1 < iStack16) { break; }
    uVar3 = (*puVar17 + iStack16 * 0x2);
    if ((uVar3 * 0x2 + puVar16 + 0xa) != 0x0) {
      uVar6 = pass1_1020_bd80(uVar3);
      uVar7 = str_op_1008_60e8(CONCAT22(puVar9,uVar6),puVar9);
      uVar13 = 0x1000;
      uVar6 = uVar7;
      puVar10 = puVar9;
      mem_op_1000_179c(0x14,puVar9,0x1000);
      uVar11 = puVar10 | uVar6;
      if (uVar11 == 0x0) {
        uVar6 = 0x0;
        uVar11 = 0x0;
      }
      else {
        uVar13 = 0x1018;
        struct_1018_47c8(CONCAT22(puVar10,uVar6),0x1,CONCAT22(puVar9,uVar7),
                         uVar3,0x0);
      }
      puVar4 = iVar9.field_0xa;
      ppcVar5 = (*iVar9.field_0xa + 0x4);
      (**ppcVar5)(uVar13,puVar4,(puVar4 >> 0x10),uVar6,uVar11);
      puVar9 = extraout_DX;
    }
    iStack16 += 0x1;
  }
  return;
}



pub fn pass1_1008_c98e(param_1: u32,param_2: u32,param_3: HFILE16,param_4: u16)
{
  let BVar1: bool;
  let local_10: [u32;0x3];
  
  BVar1 = write_to_file_1008_7cac(param_2,param_4);
  if (BVar1 != 0x0) {
    local_10[0] = (param_1 + 0xe);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,(param_2 >> 0x10),local_10,param_4,
                       0x4,param_3);
    if (BVar1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d0;
      return;
    }
  }
  return;
}



pub fn pass1_1008_c9d4(param_1: u32,param_2: u32,param_3: i16,param_4: u16,longparam_5: i32)
{
  let BVar1: bool;
  let unaff_SS: u16;
  let uVar2: u16;
  
  if (0x1 < ctx.PTR_LOOP_1050_0312) {
   // uVar2 = (param_2 >> 0x10);
    read_file_1008_7cfe(param_2,uVar2,0x15,param_4,unaff_SS);
    if (param_3 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d4;
      return;
    }
    BVar1 = read_file_1008_7dee(param_2,uVar2,param_1 + 0xe,0x0,
                                (param_1 >> 0x10),0x4,param_4);
    if (BVar1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      return;
    }
  }
  return;
}



pub fn pass1_1008_ca24(param_1: u32,param_2: u8,param_3: u16) -> u32

{
  pass1_1008_c75c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(ctx, param_1, 0x1000);
  }
  return param_1;
}



pub fn pass1_1008_ca5a(param_1: &mut Struct639,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48(CONCAT22(param_2,param_1),param_3);
  param_1.field_0xa = 0x0;
  param_1.field_0xe = 0x0;
  param_1.field_0x12 = 0x0;
  param_1.field_0x16 = 0x0;
  param_1.field_0x1a = 0x0;
  param_1.field_0x1e = 0x0;
  CONCAT22(param_2,param_1) = 0xd71a;
  param_1.field_0x2 = 0x1008;
  return;
}



pub fn pass1_1008_caa0(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  *param_1 = 0xd71a;
  (param_1 + 0x2) = 0x1008;
  pass1_1008_cac6(param_1 & 0xffff | uVar1 << 0x10);
  pass1_1010_1d80(param_1,param_2);
  return;
}



pub fn pass1_1008_cac6(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  let ppcVar3: u32;
  let iVar4: &mut Struct470;
  let uVar4: u16;
  
 // uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = iVar4.field_0xa;
  uVar2 = iVar4.field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field_0xa = 0x0;
  puVar1 = iVar4.field_0xe;
  uVar2 = iVar4.field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field_0xe = 0x0;
  puVar1 = iVar4.field_0x12;
  uVar2 = iVar4.field_0x14;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field_0x12 = 0x0;
  puVar1 = iVar4.field_0x16;
  uVar2 = iVar4.field_0x18;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field_0x16 = 0x0;
  puVar1 = iVar4.field_0x1a;
  uVar2 = iVar4.field_0x1c;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field_0x1a = 0x0;
  puVar1 = iVar4.field_0x1e;
  uVar2 = iVar4.field_0x20;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field_0x1e = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_cbc4(param_1: u32,param_2: u32,param_3: u16)
{
  plVar1: &i32;
  let ppcVar2: u32;
  let bVar3: bool;
  let puVar4: u32;
  let uVar5: u16;
  let puVar6: *mut u8;
  let extraout_DX: *mut u8;
  let extraout_DX_00: *mut u8;
  let puVar8: *mut u8;
  let puVar9: *mut u8;
  let extraout_DX_01: *mut u8;
  let iVar10: &mut Struct202;
  let uVar10: u16;
  let mut pcVar11: String; 
  let uStack64: u32;
  let uStack52: u32;
  let iStack30: i16;
  let local_18: [u8;8];
  let uStack16: u16;
  let puStack14: *mut u8;
  let uStack12: u16;
  let puStack10: *mut u8;
  let iStack8: i16;
  let lStack6: i32;
  let uVar7: u32;
  
 // uVar10 = (param_1 >> 0x10);
  iVar10 = param_1;
                    // WARNING: Load size is inaccurate
  puVar4 = iVar10.field_0x1e;
  puVar8 = (&iVar10.field_0x1e + 0x2);
  if ((puVar8 | puVar4) != 0x0) {
    ppcVar2 = *puVar4;
    (**ppcVar2)();
    puVar8 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar8,0x1000);
  if ((puVar8 | puVar4) == 0x0) {
    puVar4 = 0x0;
    puVar8 = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(puVar8,puVar4));
    puVar8 = extraout_DX_00;
  }
  &iVar10.field_0x1e = puVar4;
  (&iVar10.field_0x1e + 0x2) = puVar8;
  lStack6 = (param_2 + 0x200);
  pass1_1028_dc52(CONCAT22(param_3,local_18),0x1,0x0,0x400);
  iStack30 = 0x0;
  loop {
    puVar6 = local_18;
    pass1_1028_e4ec(CONCAT22(param_3,puVar6));
    puVar9 = (puVar8 | puVar6);
    if (puVar9 == 0x0) { break; }
    plVar1 = (puVar6 + 0x200);
    puVar8 = puVar9;
    if (*plVar1 == lStack6) {
      iStack30 += 0x1;
    }
  }
  bVar3 = false;
  if (0x1 < iStack30) {
    uStack16 = uStack12;
    puStack14 = puStack10;
    if (iStack8 != 0x0) {
      uStack16 = 0x1;
      puStack14 = 0x0;
      puStack10 = puStack14;
    }
    loop {
      puVar8 = puStack10;
      puVar6 = local_18;
      pass1_1028_e4ec(CONCAT22(param_3,puVar6));
      puVar9 = (puVar8 | puVar6);
      if (puVar9 == 0x0) { break; }
      puStack10 = puVar9;
      if (((puVar6 + 0x200) == lStack6) && ((puVar6 + 0x4) != 0x4000001)
         ) {
        pcVar11 = load_string_1038_4d28(CONCAT22(puVar8, puVar6));
       // puVar9 = (pcVar11 >> 0x10);
        uVar5 = str_op_1008_60e8(pcVar11,puVar9);
        uVar7 = uVar5;
        uStack52 = CONCAT22(puVar9,uVar5);
        mem_op_1000_179c(0x12,puVar9,0x1000);
        if ((puVar9 | uVar7) != 0x0) {
          struct_1018_4920((uVar7 & 0xffff | ZEXT24(puVar9) << 0x10),0x1,
                           uStack52,(puVar6 + 0x4));
        }
        ppcVar2 = (*iVar10.field_0x1e + 0x4);
        (**ppcVar2)();
        bVar3 = true;
        puStack10 = extraout_DX_01;
      }
    }
  }
  if (!bVar3) {
    load_string_1010_84ac
              (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010
              );
    uStack64 = CONCAT22(puVar9,puVar6);
    mem_op_1000_179c(0x12,puVar9,0x1000);
    if ((puVar9 | puVar6) != 0x0) {
      struct_1018_4920(CONCAT22(puVar9,puVar6),0x0,uStack64,0x0);
    }
    ppcVar2 = (*iVar10.field_0x1e + 0x4);
    (**ppcVar2)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_cda2(param_1: u32,param_2: u32,param_3: u16)
{
  plVar1: &i32;
  let puVar2: u32;
  let ppcVar3: u32;
  let puVar4: u32;
  let mut pcVar5: String; 
  let uVar6: u16;
  let uVar7: u16;
  let puVar9: &mut Struct206;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u32;
  let extraout_DX: *mut u8;
  let extraout_DX_00: u16;
  let puVar11: *mut u8;
  let uVar12: u16;
  let extraout_DX_01: u16;
  let puVar13: *mut u8;
  let iVar15: &mut Struct205;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u8;
  let puVar17: *mut u16;
  let lStack50: i32;
  u8 local_2e [0xa];
  let uStack36: u16;
  let uStack34: u32;
  let uStack30: u32;
  let uStack26: u32;
  let puStack18: u32;
  let puStack16: *mut u8;
  let puStack14: *mut u16;
  let uStack10: u16;
  let uStack8: u32;
  let iStack4: i16;
  
 // uVar14 = (param_1 >> 0x10);
  iVar15 = param_1;
                    // WARNING: Load size is inaccurate
  puVar4 = iVar15.field_0x1a;
  puVar13 = (&iVar15.field_0x1a + 0x2);
  puStack14 = CONCAT22(puVar13,puVar4);
  puStack18 = puVar4;
  puStack16 = puVar13;
  if ((puVar13 | puVar4) != 0x0) {
    ppcVar3 = *puVar4;
    (**ppcVar3)();
    puVar13 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar13,0x1000);
  puStack18 = puVar4;
  puStack16 = puVar13;
  if ((puVar13 | puVar4) == 0x0) {
    puVar4 = 0x0;
    uVar15 = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(puVar13,puVar4));
    uVar15 = extraout_DX_00;
  }
  &iVar15.field_0x1a = puVar4;
  (&iVar15.field_0x1a + 0x2) = uVar15;
  iStack4 = 0x0;
 // uVar15 = (param_2 >> 0x10);
  uStack8 = (param_2 + 0x210);
  uStack26._2_2_ = (param_2 + 0x212);
  uVar10 = (uStack26._2_2_ | uStack8);
  if ((uStack26._2_2_ | uStack8) != 0x0) {
    uStack26 = (uStack8 + 0xa);
    uStack30 = 0x0;
    loop {
      uVar10 = uStack26;
      if (uStack26 <= uStack30) { break; }
      bad_1030_1312();
      uStack34 = uVar10 & 0xffff | ZEXT24(uStack26._2_2_) << 0x10;
      if ((uStack26._2_2_ | uVar10) != 0x0) {
        // for (uStack36 = 0x1; uStack36 < 0x15; uStack36 += 0x1) {
        //   local_2e._8_2_ =
        //        pass1_1030_ce2e(uStack34,(uStack34 >> 0x10),uStack36);
        //   if (local_2e._8_2_ != 0x0) {
        //     pass1_1008_5784(CONCAT22(param_3,local_2e),iVar15.field_0x1a)
        //     ;
        //     loop {
        //       puVar9 = local_2e;
        //       pass1_1008_5b12(puVar9,param_3);
        //       lStack50 = CONCAT22(extraout_DX_01,puVar9);
        //       puVar13 = (extraout_DX_01 | puVar9);
        //       if (puVar13 == 0x0) break;
        //     } while (puVar9.field_0xe != uStack36);
        //     if (lStack50 == 0x0) {
        //       pcVar5 = string_op_1020_c222(uStack36);
        //       uVar6 = str_op_1008_60e8(CONCAT22(puVar13,pcVar5),puVar13);
        //       uVar16 = 0x0;
        //       puVar11 = puVar13;
        //       uVar7 = uVar6;
        //       mem_op_1000_179c(0x10,puVar13,0x1000);
        //       puStack14 = CONCAT22(puVar11,uVar7);
        //       if ((puVar11 | uVar7) == 0x0) {
        //         uVar15 = 0x0;
        //         uVar12 = 0x0;
        //       }
        //       else {
        //         uVar16 = 0x18;
        //         puVar17 = struct_1018_48b0(puStack14,
        //                                    CONCAT22(local_2e._8_2_ >> 0xf,
        //                                             local_2e._8_2_ & 0xff |
        //                                             (
        //                                      local_2e._8_2_ >> 0x8) << 0x8),
        //                                    CONCAT22(puVar13,uVar6),uStack36);
        //         uVar12 = (puVar17 >> 0x10);
        //         uVar15 = SUB42(puVar17,0x0);
        //       }
        //       puVar2 = iVar15.field_0x1a;
        //       ppcVar3 = (*iVar15.field_0x1a + 0x4);
        //       (**ppcVar3)(uVar16,puVar2,(puVar2 >> 0x10),uVar15,uVar12);
        //     }
        //     else {
        //       plVar1 = &puVar9.field_0x8;
        //       *plVar1 = *plVar1 + local_2e._8_2_;
        //     }
        //     iStack4 = 0x1;
        //   }
        // }
      }
      uStack30 += 0x1;
    }
  }
  uVar8 = uVar10;
  uStack10 = 0x0;
  if (iStack4 == 0x0) {
    load_string_1010_84ac
              (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar16 = 0x0;
    puVar13 = uStack26._2_2_;
    uVar9 = uVar8;
    mem_op_1000_179c(0x10,uStack26._2_2_,0x1000);
    puStack18 = uVar9;
    puStack16 = puVar13;
    if ((puVar13 | uVar9) == 0x0) {
      uVar15 = 0x0;
      uVar12 = 0x0;
    }
    else {
      uVar16 = 0x18;
      puVar17 = struct_1018_48b0(CONCAT22(puVar13,uVar9),0x0,
                                 CONCAT22(uStack26._2_2_,uVar8),0x0);
     // uVar12 = (puVar17 >> 0x10);
      uVar15 = SUB42(puVar17,0x0);
    }
    puVar2 = iVar15.field_0x1a;
    ppcVar3 = (*iVar15.field_0x1a + 0x4);
    (**ppcVar3)(uVar16,puVar2,(puVar2 >> 0x10),uVar15,uVar12);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_cfa0(param_1: u32,param_2: u32)
{
  let uVar1: u32;
  let uVar2: u32;
  let ppcVar3: u32;
  let bVar4: bool;
  let puVar5: u32;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u32;
  let extraout_DX: *mut u8;
  let extraout_DX_00: *mut u8;
  let puVar11: *mut u8;
  let puVar12: *mut u8;
  let puVar13: *mut u8;
  let extraout_DX_01: *mut u8;
  let extraout_DX_02: *mut u8;
  let extraout_DX_03: *mut u8;
  let uVar14: u16;
  let iVar15: i16;
  let uVar16: u16;
  let uVar17: u16;
  let unaff_SS: u16;
  let puVar18: *mut u16;
  
 // uVar16 = (param_1 >> 0x10);
  iVar15 = param_1;
  puVar5 = (iVar15 + 0x16);
  puVar11 = (iVar15 + 0x18);
  if ((puVar11 | puVar5) != 0x0) {
    ppcVar3 = *puVar5;
    (**ppcVar3)();
    puVar11 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar11,0x1000);
  if ((puVar11 | puVar5) == 0x0) {
    puVar5 = 0x0;
    puVar11 = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(puVar11,puVar5));
    puVar11 = extraout_DX_00;
  }
  (iVar15 + 0x16) = puVar5;
  (iVar15 + 0x18) = puVar11;
  bVar4 = false;
  uVar1 = (param_2 + 0x1f6);
  uVar10 = uVar1;
  pass1_1030_38f2(uVar1,0x2,unaff_SS);
  uVar6 = uVar10;
  if ((-0x1 < puVar11) && ((0x0 < puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    load_string_1010_84ac
              (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar6;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if ((puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842(CONCAT22(puVar13,uVar7),
                                 uVar10 & 0xffff | ZEXT24(puVar11) << 0x10,
                                 CONCAT22(puVar12,uVar6),0x2);
     // uVar9 = (puVar18 >> 0x10);
      uVar6 = puVar18;
    }
    uVar2 = (iVar15 + 0x16);
    ppcVar3 = ((iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,uVar2,(uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_01;
  }
  pass1_1030_38f2(uVar1,0x3,unaff_SS);
  if ((-0x1 < puVar11) && ((0x0 < puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    uVar8 = uVar6;
    load_string_1010_84ac
              (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar8;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if ((puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842(CONCAT22(puVar13,uVar7),CONCAT22(puVar11,uVar6)
                                 ,CONCAT22(puVar12,uVar8),0x3);
     // uVar9 = (puVar18 >> 0x10);
      uVar6 = puVar18;
    }
    uVar2 = (iVar15 + 0x16);
    ppcVar3 = ((iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,uVar2,(uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_02;
  }
  pass1_1030_38f2(uVar1,0x4,unaff_SS);
  if ((-0x1 < puVar11) && ((0x0 < puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    uVar8 = uVar6;
    load_string_1010_84ac
              (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar8;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if ((puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842(CONCAT22(puVar13,uVar7),CONCAT22(puVar11,uVar6)
                                 ,CONCAT22(puVar12,uVar8),0x4);
     // uVar9 = (puVar18 >> 0x10);
      uVar6 = puVar18;
    }
    uVar2 = (iVar15 + 0x16);
    ppcVar3 = ((iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,uVar2,(uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_03;
  }
  if (!bVar4) {
    load_string_1010_84ac
              (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010
              );
    uVar17 = 0x1000;
    puVar12 = puVar11;
    uVar7 = uVar6;
    mem_op_1000_179c(0x14,puVar11,0x1000);
    if ((puVar12 | uVar7) == 0x0) {
      uVar9 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842(CONCAT22(puVar12,uVar7),0x0,
                                 CONCAT22(puVar11,uVar6),0x0);
     // uVar14 = (puVar18 >> 0x10);
      uVar9 = SUB42(puVar18,0x0);
    }
    uVar2 = (iVar15 + 0x16);
    ppcVar3 = ((iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,uVar2,(uVar2 >> 0x10),uVar9,uVar14);
  }
  return;
}


pub fn struct_1008_dc90(param_1: *mut u16,param_2: u32,param_3: u32)
{
  let iVar1: &mut Struct212;
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = param_3;
  iVar1.field_0x8 = param_2;
  iVar1.field_0xc = 0x0;
  iVar1.field_0xe = 0x0;
  iVar1.field_0x12 = 0x0;
  *param_1 = 0xdd4a;
  iVar1.field_0x2 = 0x1008;
  return;
}



pub fn struct_1008_dcdc(param_1: *mut u16)
{
  let iVar1: &mut Struct220;
  let uVar1: u16;
  
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = 0x0;
  iVar1.field_0x8 = 0x0;
  iVar1.field_0xc = 0x0;
  iVar1.field_0xe = 0x0;
  iVar1.field_0x12 = 0x0;
  *param_1 = 0xdd4a;
  iVar1.field_0x2 = 0x1008;
  return;
}


pub fn struct_1008_dd4e(param_1: &mut Struct209,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  let puVar2: *mut u8;
  let extraout_DX: u16;
  let paVar3: &mut Struct79;
  
  paVar3 = struct_op_1010_1d48(CONCAT22(param_2,param_1),param_3);
 // puVar2 = (paVar3 >> 0x10);
  uVar1 = 0x0;
  &param_1.field_0xa = 0x0;
  param_1.field_0xe = 0x0;
  param_1.field_0x12 = 0x0;
  param_1.field_0x16 = 0x0;
  param_1.field_0x1a = 0x0;
  param_1.field_0x1e = 0x0;
  CONCAT22(param_2,param_1) = 0xeaac;
  param_1.field_0x2 = 0x1008;
  mem_op_1000_179c(0xc,puVar2,0x1000);
  if ((puVar2 | uVar1) == 0x0) {
    &param_1.field_0xa = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(puVar2,uVar1));
    param_1.field_0xa = uVar1;
    param_1.field_0xc = extraout_DX;
  }
  return;
}


pub fn struct_1008_ec72(param_1: *mut u16) -> u16

{
  struct_1010_383a(param_1);
  *param_1 = 0xefc4;
  (param_1 + 0x2) = 0x1008;
  return param_1;
}



pub fn struct_1008_ecb2(param_1: &mut Struct221,param_2: u16,param_3: u16) -> u32

{
  let in_AX: u16;
  let in_DX: *mut u8;
  let unaff_SS: u16;
  
  struct_1010_2cd2(param_1,param_2,param_3,unaff_SS);
  CONCAT22(param_2,param_1) = 0xef9c;
  param_1.field_0x2 = 0x1008;
  mem_op_1000_179c(0x20c,in_DX,0x1000);
  param_1.field_0x5c = in_AX;
  param_1.field_0x5e = in_DX;
  pass1_1000_4906(CONCAT22(in_DX,param_1.field_0x5c),0x0,
                  0x20c);
  return CONCAT22(param_2,param_1);
}


pub fn pass1_1008_3ab8(
    ctx: &mut AppContext,
    param_1: &mut Struct20) -> &mut Struct20
{
    set_struct_1008_687a(param_1, 0x0);
    param_1.field_0xde = 0x0;
    param_1.field_0x0 = 0x3b46;
    param_1.field_0x2 = 0x1008;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1.field_0x5b)), ctx.s_SOLDefaultWindowClass_1050_01fe);
    return param_1;
}


pub fn clear_struct_1008_3e38(param_1: &mut Struct18) -> &mut Struct18

{
    param_1.field_0x0 = 0x0;
    param_1.field_0x2 = 0x0;
    param_1.field_0x2 = 0x0;
    return param_1;
}




