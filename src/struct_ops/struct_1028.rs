
pub fn struct_1028_0068(param_1: U32Ptr,param_2: U32Ptr)
{
  let uVar1: u16;
  let extraout_DX: u16;
  let iVar2: &mut Struct183;
  let uVar2: u16;
  
  struct_1028_b354(param_1);
 // uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = 0x0;
  iVar2.field_0x20 = 0x0;
  &iVar2.field_0x22 = 0x0;
  *param_1 = 0x8ec;
  iVar2.field_0x2 = &USHORT_1050_1028;
  mem_op_1000_179c(0xc,param_2,0x1000);
  if ((param_2 | uVar1) == 0x0) {
    &iVar2.field_0x22 = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(param_2,uVar1));
    iVar2.field_0x22 = uVar1;
    iVar2.field_0x24 = extraout_DX;
  }
  return;
}


pub fn struct_1028_0954(param_1: U32Ptr) -> u16

{
  let iVar1: &mut Struct185;
  let uVar1: u16;
  
  struct_1028_b354(param_1);
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field_0x20 = 0x0;
  *param_1 = 0xada;
  iVar1.field_0x2 = &USHORT_1050_1028;
  iVar1.field_0xe = 0x4b;
  return param_1;
}


pub fn struct_1028_0b42(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xbbc;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_178c(param_1: U32Ptr) -> u16

{
  struct_1030_dc96(param_1);
  *param_1 = 0x1b54;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_1bbc(param_1: U32Ptr) -> u16

{
  let iVar1: &mut Struct190;
  let uVar1: u16;
  
  struct_1028_b354(param_1);
 // uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field_0x20 = 0x0;
  iVar1.field_0x22 = 0x0;
  *param_1 = 0x1eee;
  iVar1.field_0x2 = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_1f56(param_1: U32Ptr,param_2: U32Ptr)
{
  let uVar1: u32;
  let uVar2: u16;
  let extraout_DX: u16;
  let iVar3: &mut Struct186;
  let uVar3: u16;
  
  struct_1028_b354(param_1);
 // uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar2 = 0x0;
  &iVar3.field_0x20 = 0x0;
  iVar3.field_0x24 = 0x0;
  *param_1 = 0x2572;
  iVar3.field_0x2 = &USHORT_1050_1028;
  mem_op_1000_179c(0xc,param_2,0x1000);
  if ((param_2 | uVar2) == 0x0) {
    &iVar3.field_0x20 = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(param_2,uVar2));
    iVar3.field_0x20 = uVar2;
    iVar3.field_0x22 = extraout_DX;
  }
  uVar1 = &iVar3.field_0x20;
  (uVar1 + 0xa) = 0x0;
  return;
}


pub fn struct_1028_25da(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = (s_fem16_wav_1050_2644 + 0x8);
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}



pub fn struct_1028_26b4(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x2788;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_27f0(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x2a92;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_2bdc(param_1: U32Ptr) -> u16

{
  struct_1028_0954(param_1);
  *param_1 = 0x341c;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_355e(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x3608;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}



pub fn struct_1028_37a6(param_1: U32Ptr,param_2: U32Ptr,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let iVar3: &mut Struct189;
  let uVar3: u16;
  
  struct_1028_b354(param_1);
 // uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = 0x0;
  iVar3.field_0x20 = 0x0;
  iVar3.field_0x24 = 0x0;
  &iVar3.field_0x28 = 0x0;
  *param_1 = 0x3e2c;
  iVar3.field_0x2 = &USHORT_1050_1028;
  mem_op_1000_179c(0xa,param_2,0x1000);
  uVar2 = param_2 | uVar1;
  if (uVar2 == 0x0) {
    &iVar3.field_0x28 = 0x0;
  }
  else {
    pass1_1020_ba3e(CONCAT22(param_2,uVar1),0x5,0x5,param_4,param_3);
    iVar3.field_0x28 = uVar1;
    iVar3.field_0x2a = uVar2;
  }
  return;
}


pub fn struct_1028_3e94(param_1: U32Ptr) -> u16

{
  let uVar1: u16;
  
  struct_1028_b354(param_1);
 // uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0x0;
  *param_1 = 0x4004;
  (param_1 + 0x2) = &USHORT_1050_1028;
  pass1_1028_3fa2(param_1 & 0xffff | uVar1 << 0x10);
  return param_1;
}


pub fn struct_1028_406c(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x42ec;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_4354(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x446a;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_44d2(param_1: U32Ptr) -> u16

{
  let uVar1: u16;
  
  struct_1028_b354(param_1);
 // uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0x0;
  *param_1 = 0x4836;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_489e(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = &ctx.PTR_LOOP_1050_4942;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_49aa(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x4b1c;
  (param_1 + 0x2) = &USHORT_1050_1028;
  pass1_1000_4906(
                  (param_1 & 0xffff0000 | (param_1 + 0x20)),
                  0x0,0xa);
  return param_1;
}


pub fn struct_1028_4b84(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = (s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1);
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_50d8(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x5280;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_52e8(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x535e;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_53c6(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x54bc;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_5630(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x56ac;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_57a6(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0x581c;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_5966(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = (s_mineToSmelter__no_mines_1050_59df + 0x1);
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_5a48(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = s_thisLo_1050_5bec;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_5c54(param_1: U32Ptr) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = (s_static_1050_5d8b + 0x3);
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_5ed8(param_1: U32Ptr) -> u16

{
  let uVar1: u16;
  
  struct_1028_b354(param_1);
 // uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0x0;
  *param_1 = 0x6054;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return param_1;
}


pub fn struct_1028_60bc(param_1: U32Ptr,param_2: u16,param_3: U32Ptr) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  let iVar2: &mut Struct187;
  
  iVar2 = param_1;
 // uVar2 = (param_1 >> 0x10);
  struct_1028_b354(param_1);
  &iVar2.field_0x20 = 0x0;
  *param_1 = 0x6876;
  iVar2.field_0x2 = &USHORT_1050_1028;
  mem_op_1000_179c(0xc,param_3,0x1000);
  if ((param_3 | param_2) == 0x0) {
    &iVar2.field_0x20 = 0x0;
  }
  else {
    uVar1 = set_struct_1008_574a(CONCAT22(param_3,param_2));
    iVar2.field_0x20 = uVar1;
    iVar2.field_0x22 = (uVar1 >> 0x10);
  }
  return param_1;
}


pub fn
struct_op_1028_87f0(param_1: u16,param_2: u8,param_3: &mut Struct97,param_4: u16,
                   param_5: u16,param_6: u16,param_7: U32Ptr,param_8: u16,
                   param_9: u32,param_1: u320)

{
  let iVar1: &mut Struct97;
  let puVar1: &mut Struct97;
  
  struct_op_1028_d1dc(param_1,param_2,param_3,0x3e8);
 // puVar1 = (param_3 >> 0x10);
  iVar1 = param_3;
  iVar1.field_0x108 = param_10;
  iVar1.field_0x10c = param_9;
  iVar1.field_0x110 = 0x0;
  iVar1.field_0x114 = *param_7;
  iVar1.field_0x118 = (param_7 + 0x1);
  iVar1.field_0x11a = param_6;
  iVar1.field_0x11c = param_5;
  iVar1.field_0x11e = param_4;
  iVar1.field_0x122 = 0x0;
  iVar1.field_0x120 = 0x0;
  param_3 = 0x8d8e;
  iVar1.field_0x2 = &USHORT_1050_1028;
  sys_1000_3f9c(&iVar1.field_0x8,puVar1,
                s_SCInternalPutBldg_site_0x_08lx__b_1050_5046,
                ctx.data_seg,param_10,&stack0xfffe,puVar1,0x1000,
                param_1,param_2);
  return;
}



pub fn
struct_op_1028_8888(param_1: u16,param_2: u8,param_3: &mut Struct100,param_4: u16,
                   param_5: u16,param_6: U32Ptr,param_7: u16,param_8: u32,
                   param_9: u32,param_1: u320)

{
  let iVar1: &mut Struct100;
  let puVar1: U32Ptr;
  
  struct_op_1028_d1dc(param_1,param_2,param_3,0x3e8);
 // puVar1 = (param_3 >> 0x10);
  iVar1 = param_3;
  iVar1.field_0x108 = param_10;
  iVar1.field_0x10c = param_9;
  iVar1.field_0x110 = param_8;
  iVar1.field_0x114 = *param_6;
  iVar1.field_0x118 = (param_6 + 0x1);
  iVar1.field_0x11a = param_5;
  iVar1.field_0x11c = 0x0;
  iVar1.field_0x11e = param_4;
  iVar1.field_0x122 = 0x0;
  iVar1.field_0x120 = 0x0;
  param_3.field_0x0 = 0x8d8e;
  iVar1.field_0x2 = &USHORT_1050_1028;
  sys_1000_3f9c(&iVar1.field_0x8,puVar1,
                s_SCInternalPutBldg2_site_0x_08lx__1050_506f,
                ctx.data_seg,param_10,&stack0xfffe,puVar1,0x1000,
                param_1,param_2);
  return;
}


pub fn
struct_op_1028_933c(param_1: &mut Struct100,param_2: u16,param_3: u16,param_4: u16,
                   param_5: U32Ptr,param_6: u16,param_7: u32,param_8: u32,
                   param_9: u16,param_10: u8)

{
  let iVar1: i16;
  let puVar2: U32Ptr;
  
  struct_op_1028_d1dc(param_9,param_10,param_1,0x3e8);
 // puVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x108) = param_8;
  (iVar1 + 0x10c) = param_7;
  (iVar1 + 0x110) = 0x0;
  (iVar1 + 0x114) = *param_5;
  (iVar1 + 0x118) = (param_5 + 0x1);
  (iVar1 + 0x11a) = param_4;
  (iVar1 + 0x11c) = param_2;
  (iVar1 + 0x120) = 0x0;
  (iVar1 + 0x11e) = 0x0;
  (iVar1 + 0x122) = param_3;
  param_1.field_0x0 = 0x9934;
  (iVar1 + 0x2) = &USHORT_1050_1028;
  sys_1000_3f9c((iVar1 + 0x8),puVar2,
                s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,
                ctx.data_seg,param_8,&stack0xfffe,puVar2,0x1000,
                param_9,param_10);
  return;
}


pub fn struct_1028_9c62(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: u8) -> u16

{
  struct_op_1028_d1dc(param_4,param_5,CONCAT22(param_2,param_1),param_3);
  (param_1 + 0x108) = param_3;
  CONCAT22(param_2,param_1) = 0x9eb6;
  (param_1 + 0x2) = &USHORT_1050_1028;
  return CONCAT22(param_2,param_1);
}


astruct_100 * 
struct_op_1028_d1dc(param_1: u16,param_2: u8,param_3: &mut Struct100,param_4: u16)

{
  let iVar1: &mut Struct101;
  let puVar1: U32Ptr;
  let in_stack_0000fffa: u16;
  
 // puVar1 = (param_3 >> 0x10);
  iVar1 = param_3;
  param_3.field_0x0 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = param_4;
  iVar1.field_0x6 = 0x0;
  param_3.field_0x0 = 0x6ad2;
  iVar1.field_0x2 = &USHORT_1050_1028;
  sys_1000_3f9c(&iVar1.field_0x8,puVar1,0x5160,ctx.data_seg,
                in_stack_0000fffa,&stack0xfffe,puVar1,0x1000,param_1,param_2);
  return param_3;
}



pub fn struct_1028_d22e(param_1: U32Ptr,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let puVar2: U32Ptr;
  let uVar3: u16;
  
 // uVar3 = (param_1 >> 0x10);
  *param_1 = 0x0;
  (param_1 + 0x4) = param_2;
  mem_op_1000_179c(0xc,param_3,0x1000);
  uVar1 = param_2;
  puVar2 = (param_3 | uVar1);
  if (puVar2 == 0x0) {
    *param_1 = 0x0;
  }
  else {
    struct_1028_d59c((param_2 & 0xffff | param_3 << 0x10),puVar2);
    param_1 = uVar1;
    (param_1 + 0x2) = puVar2;
  }
  return;
}


pub fn struct_1028_d2b0(param_1: U32Ptr,param_2: u16,param_3: u8)
{
  let local_10c: u16;
  let uStack266: u16;
  
  struct_1028_9c62(&local_10c,param_2,0x3e80,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x3a98,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x36b0,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x32c8,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x2ee0,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x2af8,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x2710,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,(s_noth_bmp_1050_2321 + 0x7),param_2,
                   param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x1f40,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x1b58,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x1770,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x1388,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0xfa0,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0xbb8,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(&local_10c,param_2,0x3e8,param_2,param_3);
  fn_ptr_1028_d566(param_1,CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  pass1_1028_d6b2(*param_1);
  return;
}


pub fn struct_1028_d59c(param_1: U32Ptr,param_2: U32Ptr)
{
  let puVar1: U32Ptr;
  let uVar2: u16;
  let puVar3: U32Ptr;
  let puVar4: U32Ptr;
  let extraout_DX: U32Ptr;
  let iVar5: &mut Struct158;
  let uVar5: u16;
  let puStack14: U32Ptr;
  
 // uVar5 = (param_1 >> 0x10);
  iVar5 = param_1;
  *param_1 = 0x0;
  iVar5.field_0x4 = 0x0;
  iVar5.field_0x8 = 0x0;
  puVar3 = *_PTR_LOOP_1050_5748;
  *param_1 = puVar3;
  mem_op_1000_179c(0xc,param_2,0x1000);
  puVar1 = (puVar3 & 0xffff | ZEXT24(param_2) << 0x10);
  puVar4 = (param_2 | puVar3);
  if (puVar4 == 0x0) {
    iVar5.field_0x4 = 0x0;
  }
  else {
    set_struct_1008_574a((puVar3 & 0xffff | ZEXT24(param_2) << 0x10))
    ;
    *puVar1 = 0xd804;
    (puVar3 + 0x2) = &USHORT_1050_1028;
    iVar5.field_0x4 = puVar1;
    puVar3 = puVar1;
    puVar4 = extraout_DX;
  }
  uVar2 = puVar3;
  mem_op_1000_179c(0xc,puVar4,0x1000);
  puStack14 = CONCAT22(puVar4,uVar2);
  if ((puVar4 | uVar2) == 0x0) {
    iVar5.field_0x8 = 0x0;
  }
  else {
    set_struct_1008_574a(CONCAT22(puVar4,uVar2));
    *puStack14 = 0xd804;
    (uVar2 + 0x2) = &USHORT_1050_1028;
    iVar5.field_0x8 = puStack14;
  }
  return;
}

