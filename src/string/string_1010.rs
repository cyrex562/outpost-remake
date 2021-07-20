
pub fn string_1010_1722(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u32,
    extraout_dx: u16)
{
  // let extraout_dx: u16;
  let mut u_var1 = 0u16;
  let mut str_1: String;
  let local_52: [u8;50] = [0;50];
  
  pass1_1028_b58e(param_5);
  if (extraout_dx | param_2) == 0x0 {
    str_1 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc as i16,
                        ((ctx.PTR__LOOP_1050_14cc >> 0x10) as i16),
                        ctx.USHORT_1050_1028);
   // u_var1 = (pcVar2 >> 0x10);
    unk_str_op_1000_3d3e(&mut read_string_from_addr(CONCAT22(param_1, local_52[0] as u16)), &mut str_1);
    str_1 = read_string_from_addr(CONCAT22(u_var1, local_52[0] as u16));
  }
  else {
    str_1 = load_string_1038_4d28(((param_2 + 0x2e) as u32));
   // param_1 = (pcVar2 >> 0x10);
  }
  str_op_1008_60e8(ctx, (str_1 & 0xffff | param_1 << 0x10),
                   (str_1 >> 0x10));
  return;
}


use crate::win_struct::HINSTANCE16;
use crate::util::{CONCAT22, read_string_from_addr, ZEXT24};
use crate::pass::pass_1028::{pass1_1028_e1ec, pass1_1028_b58e};
use crate::mem_1000::mem_op_1000_179c;
use crate::pass::pass_1000::pass1_1000_3cea;
use crate::pass::pass_1038::load_string_1038_4d28;
use crate::winapi::LoadString16;
use crate::string::string_1008::str_op_1008_60e8;
use std::default::default;
use crate::struct_ops::struct_1030::struct_op_1030_73a8;
use crate::string::string_1000::{str_op_1000_3da4, unk_str_op_1000_3d3e};
use crate::struct_ops::struct_1010::struct_1010_dd5e;
use crate::pass::pass_1010::pass1_1010_e964;
use crate::mixed::mixed_1010_20ba;
use crate::global::AppContext;

pub fn unk_load_str_op_1010_2c34() -> u16

{
  let pUVar1: U32Ptr;
  let in_DX: U32Ptr;
  in_buf_len_5: i16;
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar2: U32Ptr;
  
  puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
  mem_op_1000_179c(0x80,(puVar2 >> 0x10),0x1000);
 // in_buf_len_5 = (puVar2 >> 0x10);
  load_string_1010_84e0
            (0x1000, _PTR_LOOP_1050_14cc, 0x80, puVar2, in_buf_len_5
            );
  pUVar1 = pass1_1000_3cea(puVar2,s__1050_11c8);
  pass1_1010_e964(in_buf_len_5,unaff_SS,unaff_DI);
  pass1_1000_3cea(puVar2,CONCAT22(in_buf_len_5,pUVar1));
  return puVar2;
}


pub unsafe fn string_1010_5286(param_1: u16,param_2: u16,param_3: u32,param_4: &mut String,param_5: u16) -> u32

{
  let mut in_buffer_4: String; 
  let in_buf_len_5: U32Ptr;
  let mut pc_var1: String;
  
  pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2,param_3,(param_3 >> 0x10));
  in_buf_len_5 = (param_5 | param_4);
  if in_buf_len_5.is_null() {
    return 0x0;
  }
  in_buffer_4 = param_4.clone();
  mem_op_1000_179c(ctx, 0x80,in_buf_len_5,0x1000);
  load_string_1010_84e0
            (0x1000, _PTR_LOOP_1050_14cc, 0x80, in_buffer_4,
             in_buf_len_5);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4),0x105013ac);
  pc_var1 = load_string_1038_4d28(CONCAT22(param_5, param_4));
  pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4), pc_var1);
  return CONCAT22(in_buf_len_5,in_buffer_4);
}


pub fn load_string_1010_847e(param_1: i16, buf_len: i16, h_inst: HINSTANCE16) -> String
{
  LoadString16(h_inst, 0x3ff, &mut read_string_from_addr((param_1 + 0x682) as u32), buf_len as u16);
  return read_string_from_addr(CONCAT22(buf_len as u16, ((param_1 + 0x682) as u16)));
}



pub fn load_string_1010_84ac(param_1: i16,param_2: i16,param_3: HINSTANCE16)
{
  let uVar1: u16;
  
  uVar1 = param_2;
  LoadString16(param_3,0x3ff,(param_1 + 0x682),param_2);
  str_op_1008_60e8(ctx, CONCAT22(param_2,(param_1 + 0x682)),uVar1);
  return;
}



pub fn
load_string_1010_84e0
          (in_hinstance_5: HINSTANCE16, param_2: U32Ptr, in_resc_id_3: u16,
           in_buffer_4: &mut String, in_buf_len_5: i16)

{
  LoadString16(in_hinstance_5,in_resc_id_3,in_buffer_4,in_buf_len_5);
  return;
}


u32 
unk_load_str_op_1010_8c96
          (param_1: u32,param_2: u32,param_3: u32,param_4: u16,param_5: u16)

{
  let uVar1: u32;
  let IVar2: i16;
  let puVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let in_DX: U32Ptr;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let in_AF: u8;
  let uVar9: u32;
  let mut pcVar10: String; 
  let mut spec: String;
  let valist: U32Ptr;
  let uStack46: u32;
  let local_10: u32;
  let iStack12: i16;
  let uStack10: i16;
  let puStack8: U32Ptr;
  let uStack6: u16;
  let uStack4: u16;
  
 // uVar7 = (param_3 >> 0x10);
  iVar6 = param_3;
  uVar5 = (iVar6 + 0x6);
  uVar9 = uVar5;
  spec = param_2;
 // valist = (param_2 >> 0x10);
  if (uVar5 != 0x0) {
   // uVar8 = (param_1 >> 0x10);
    if (uVar5 == 0x1) {
      uVar5 = (iVar6 + 0x4) - 0x1;
      uVar9 = uVar5;
      if (false) {
// goto switchD_1010_8e11_caseD_4;
}
      uVar9 = param_3 & 0xffff;
      iVar4 = uVar9;
      param_4 = 0x1010;
      switch(uVar5) {
      0x0 =>
      0x1 =>
        uVar1 = (iVar6 + 0x8);
        pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
        local_10 = (iVar4 + 0xc);
        iStack12 = (iVar4 + 0x10);
        iStack10 = iVar4;
        if (0x0 < iStack12) {
          pcVar10 = load_string_1010_847e
                              (ctx.PTR__LOOP_1050_14cc,
                               (ctx.PTR__LOOP_1050_14cc >> 0x10),
                               &USHORT_1050_1028);
         // uStack4 = (pcVar10 >> 0x10);
          uStack6 = SUB42(pcVar10,0x0);
          IVar2 = wsprintf16(&USHORT_1050_1028,spec,valist);
          return CONCAT22(IVar2,uStack4);
        }
        break;
      0x2 =>
        uVar1 = (iVar6 + 0x8);
        pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
        local_10 = (iVar4 + 0xc);
        iStack12 = (iVar4 + 0x10);
        if (0x0 < iStack12) {
          iStack12 = 0x0;
          uVar9 = struct_op_1030_73a8(CONCAT22(in_DX,iVar4));
          uVar9 = pass1_1028_bb24(uVar9);
         // in_DX = (uVar9 >> 0x10);
          iStack10 = uVar9;
          puVar3 = &local_10;
          puStack8 = in_DX;
          pass1_1030_627e(param_5,puVar3,in_DX,_PTR_LOOP_1050_5740,
                          CONCAT22(param_5,puVar3),uVar9);
          pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2,puVar3,in_DX);
          uVar1 = (param_1 + 0xa);
          pass1_1010_c3c2(uVar1,(uVar1 >> 0x10),0x0,
                          CONCAT22(in_DX,puVar3),in_DX,in_AF,param_5);
          uStack46 = CONCAT22(in_DX,puVar3);
          pcVar10 = load_string_1010_847e
                              (ctx.PTR__LOOP_1050_14cc,
                               (ctx.PTR__LOOP_1050_14cc >> 0x10),
                               &USHORT_1050_1028);
         // uStack4 = (pcVar10 >> 0x10);
          uStack6 = SUB42(pcVar10,0x0);
          wsprintf16(&USHORT_1050_1028,spec,valist);
//           TODO: goto LAB_1010_8def;
        }
        break;
      default:
//         TODO: goto switchD_1010_8e11_caseD_4;
      0x4 =>
      0x7 =>
      0x8 =>
      0xa =>
//         TODO: goto LAB_1010_8ea5;
      }
      uVar9 = ZEXT24(&local_10);
      param_4 = &USHORT_1050_1028;
    }
    else {
      uVar9 = (uVar5 - 0x2);
      if (uVar5 - 0x2 == 0x0) {
        iVar4 = (iVar6 + 0x4);
        uVar5 = iVar4 - 0x4;
        if (uVar5 != 0x0) {
          uVar5 = iVar4 - 0xc;
          uVar9 = uVar5;
          if (uVar5 != 0x0) {
// goto LAB_1010_8ea5;
}
        }
        uVar1 = (iVar6 + 0x8);
        pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
        uVar1 = (param_1 + 0xa);
        pass1_1010_c3c2(uVar1,(uVar1 >> 0x10),0x0,
                        CONCAT22(in_DX,uVar5),in_DX,in_AF,param_5);
        uStack46 = CONCAT22(in_DX,uVar5);
        pcVar10 = load_string_1010_847e
                            (ctx.PTR__LOOP_1050_14cc,
                             (ctx.PTR__LOOP_1050_14cc >> 0x10),
                             &USHORT_1050_1028);
       // uStack4 = (pcVar10 >> 0x10);
        uStack6 = SUB42(pcVar10,0x0);
        wsprintf16(&USHORT_1050_1028,spec,valist);
//LAB_1010_8def:
        fn_ptr_1000_17ce((uStack46 & 0xffff | ZEXT24(in_DX) << 0x10),0x1000)
        ;
        return CONCAT22(uStack46,in_DX);
      }
    }
  }
//LAB_1010_8ea5:
  load_string_1010_84e0
            (param_4,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,spec,valist);
switchD_1010_8e11_caseD_4:
  return CONCAT22(uVar9,in_DX);
}


pub fn load_string_1010_9432(param_1: HINSTANCE16) -> U32Ptr

{
  let mut pcVar1: String; 
  
  pcVar1 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,param_1);
  return pcVar1;
}


pub fn load_string_1010_ac92(param_1: HINSTANCE16,param_2: u16,param_3: u16,param_4: i16) -> U32Ptr

{
  let mut pcVar1: String; 
  
  if ((0x0 < param_4) && (param_4 < 0x43)) {
    pcVar1 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),param_1);
    return pcVar1;
  }
  return 0x0;
}

char * 
string_op_1010_ada6(param_1: HINSTANCE16,param_2: u16,param_3: u16,param_4: u16,
                   param_5: i16,param_6: i16)

{
  let mut pcVar1: String; 
  let mut pcStack6: String; 
  
  pcStack6 = 0x0;
  if (param_6 == 0x6) {
    if (param_5 == 0x0) {
// goto LAB_1010_adee;
}
    pcVar1 = string_op_1020_c222(param_5);
  }
  else {
    if (param_6 != 0x7) {
      return 0x0;
    }
    if (param_5 == 0x0) {
// goto LAB_1010_adee;
}
    pcVar1 = string_op_1020_c2f8(param_5);
  }
  param_1 = 0x1020;
  pcStack6 = CONCAT22(param_2,pcVar1);
//LAB_1010_adee:
  if (pcStack6 == 0x0) {
    pcStack6 = load_string_1010_847e
                         (ctx.PTR__LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10),param_1);
  }
  return pcStack6;
}


pub fn
string_op_1010_c446(param_1: u16,param_2: u8,uparam_3: &mut String,param_4: u32,
                   param_5: &mut String,param_6: u32)

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  let mut pcVar5: String; 
  let uVar6: u16;
  let uVar7: u16;
  let mut in_buffer_4: String; 
  let in_buf_len_5: U32Ptr;
  let uStack22: u16;
  let mut pcStack6: String; 
  
  pcStack6 = param_5;
  if (param_5 == 0x0) {
    mem_op_1000_179c(0x100,param_3,0x1000);
    pcStack6 = (param_5 & 0xffff | ZEXT24(param_3) << 0x10);
  }
  uVar4 = struct_op_1030_73a8(param_6);
 // uVar2 = (uVar4 >> 0x10);
  uVar3 = uVar2;
  struct_1010_dd5e(param_4,(param_4 >> 0x10),param_6);
  iVar1 = (uVar4 + 0x12);
  if (0x6 < iVar1 - 0x3) {
    return;
  }
  in_buffer_4 = pcStack6;
 // in_buf_len_5 = (pcStack6 >> 0x10);
  uVar7 = ctx._PTR_LOOP_1050_14cc;
  uVar6 = (ctx.PTR__LOOP_1050_14cc >> 0x10);
  switch(iVar1) {
  default:
    break;
  0x6 =>
    load_string_1010_84e0(0x1010, uVar7, 0x3ff, in_buffer_4, in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar5 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1000);
    uVar7 = pcVar5;
    uVar6 = s_____s__lu_1050_38d7;
//     TODO: goto LAB_1010_c4f9;
  0x7 =>
  0x9 =>
    break;
  0x8 =>
    load_string_1010_84e0(0x1010, uVar7, 0x3ff, in_buffer_4, in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar5 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1000);
    uVar7 = pcVar5;
    uVar6 = s_____s__lu_1050_38cd;
//LAB_1010_c4f9:
    sys_1000_3f9c((in_buffer_4 + uStack22),in_buf_len_5,uVar6,
                  ctx.data_seg,uVar7,&stack0xfffe,uVar3,0x1000,param_1,
                  param_2);
    return;
  }
  load_string_1010_84e0(0x1010, uVar7, 0x3ff, in_buffer_4, in_buf_len_5);
  return;
}


i16 
string_1010_dcac(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: u32,
                param_6: &mut Struct104)

{
  let uVar1: u32;
  let iVar2: &mut Struct105;
  let uVar2: u16;
  let uVar3: u16;
  let iVar5: &mut Struct104;
  let uVar6: u16;
  let uVar7: u16;
  let mut pcVar4: String; 
  
  pcVar4 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,param_1);
 // uVar6 = (param_6 >> 0x10);
  iVar5 = param_6;
  uVar2 = (&iVar5.field_0x2 + 0x2);
  iVar2 = (&iVar5.field_0x2 + param_4 * 0xa);
 // uVar7 = (param_5 >> 0x10);
  iVar2.field_0x4 = (param_4 * 0x2 + param_5);
  string_1040_a626(CONCAT22(uVar2,iVar2),pcVar4,uVar2);
  unk_str_op_1000_3d3e(pcVar4,0x10503941);
  uVar2 = param_4 + 0x1;
  uVar1 = iVar5.field_0x2;
  uVar3 = uVar1 + uVar2 * 0xa;
  (uVar3 + 0x4) = (uVar2 * 0x2 + param_5);
  string_1040_a626((uVar1 & 0xffff0000 | uVar3),pcVar4,uVar2);
  return uVar2;
}


pub fn load_str_1010_ddf6(param_1: u32,param_2: u32)
{
  in_buf_len_5: i16;
  let uVar1: u32;
  
 // in_buf_len_5 = (param_1 >> 0x10);
  *(param_1 + 0x13c) = 0x0;
  uVar1 = struct_op_1030_73a8(param_2);
  switch((uVar1 + 0x12)) {
  0x1 =>
  0x2 =>
  0x4 =>
  0x7 =>
  0x9 =>
    break;
  0x3 =>
  0x5 =>
    break;
  0x6 =>
    break;
  0x8 =>
    break;
  default:
//     TODO: goto switchD_1010_de53_caseD_9;
  }
  load_string_1010_84e0
            (0x1010, _PTR_LOOP_1050_14cc, 0x3ff,
             (param_1 + 0x13c), in_buf_len_5);
switchD_1010_de53_caseD_9:
  return;
}
