





pub fn pass1_1010_6034(mut param_1: u16 ,param_2: *mut Struct19)

{
  let mut puVar1: *mut u16;
  let mut struct_1: *mut Struct19;
  let mut struct_1_hi: *mut Struct19;

  struct_1_hi = (param_2 >> 0x10);
  struct_1 = param_2;
  struct_1.field16_0x1e = 0x1;
  struct_1.field17_0x20 = 0x1;
  struct_1[0x1].field9_0x12 = 0x1;
  struct_1[0x1].field10_0x14 = 0x1;
  pass1_1010_60a0(param_2);
  puVar1 = pass1_1000_4906((param_2 & 0xffff0000 | ZEXT24(&struct_1.field18_0x22)),NULL,0x40);
  load_string_1010_84ac(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x630);
  struct_1[0x1].field3_0x8 = puVar1;
  struct_1[0x1].horiz_res_0xa = param_1;
  load_string_1010_84ac(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x62f);
  struct_1[0x1].ver_res_0xc = puVar1;
  struct_1[0x1].field_0xe = param_1;
  return;
}
pub fn pass1_1010_60a0(param_1: *mut Struct19)

{
  (param_1 + 0x76) = 0x5;
  return;
}


pub fn struct_1010_6326(param_1: *mut Struct19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0;
  (param_1 + 0xe) = 0;
  (param_1 + 0x12) = 0;
  (param_1 + 0x16) = 0;
  (param_1 + 0x1a) = 0;
  (param_1 + 0x1e) = 0;
  (param_1 + 0x22) = 0;
  param_1.offset_0x0 = 0x66f0;
  (param_1 + 0x2) = 0x1010;
  return;
}


pub fn pass1_1010_6700(param_1: *mut Struct19,mut param_2: u16 ) -> *mut Struct19

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0x148) = 0x33;
    //        1010:6aac  86  6a  10  10      addr         pass1_1010_6a86
    //
  param_1.offset_0x0 = 0x6aac;
  (param_1 + 0x2) = 0x1010;
  pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0xa)),NULL,0x114);
  (param_1 + 0x32) = 0x1;
  (param_1 + 0x40) = 0x1;
  (param_1 + 0x46) = 0x1;
  (param_1 + 0x4e) = 0x1;
  (param_1 + 0x54) = 0x1;
  (param_1 + 0x5e) = 0x1;
  (param_1 + 0x68) = 0x1;
  (param_1 + 0x6c) = 0x1;
  (param_1 + 0x74) = 0x1;
  (param_1 + 0x78) = 0x1;
  (param_1 + 0x7a) = 0x1;
  (param_1 + 0x7e) = 0x1;
  (param_1 + 0x82) = 0x1;
  (param_1 + 0xa2) = 0x1;
  (param_1 + 0xa4) = 0x1;
  (param_1 + 0xa6) = 0x1;
  (param_1 + 0xa8) = 0x1;
  (param_1 + 0xae) = 0x1;
  (param_1 + 0xb2) = 0x1;
  (param_1 + 0xb8) = 0x1;
  (param_1 + 0xbe) = 0x1;
  (param_1 + 0xc0) = 0x1;
  (param_1 + 0xc4) = 0x1;
  (param_1 + 0xd4) = 0x1;
  (param_1 + 0xda) = 0x1;
  (param_1 + 0xe2) = 0x1;
  (param_1 + 0xfe) = 0x1;
  (param_1 + 0x100) = 0x1;
  (param_1 + 0x102) = 0x1;
  (param_1 + 0x104) = 0x1;
  (param_1 + 0x106) = 0x1;
  (param_1 + 0x108) = 0x1;
  pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x11e)),NULL,0x2a);
  (param_1 + 0x120) = 0x1;
  (param_1 + 0x122) = 0x1;
  (param_1 + 0x124) = 0x1;
  (param_1 + 0x126) = 0x1;
  (param_1 + 0x128) = 0x1;
  (param_1 + 0x12c) = 0x1;
  (param_1 + 0x138) = 0x1;
  return param_1;
}
pub fn pass1_1010_6814(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  (param_1 + param_3 * 0x2 + 0x11e) = param_2;
  return;
}
pub fn pass1_1010_682e(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  (param_1 + param_3 * 0x2 + 0xa) = param_2;
  return;
}








pub fn pass1_1010_6abc(param_1: *mut Struct19,mut param_2: u16 ,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut paVar2: *mut Struct57;
  let mut paVar4: *mut Struct19;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;

  uVar3 = (in_EDX >> 0x10);
  paVar4 = struct_op_1010_1d48(param_1,param_2);
  paVar2 = CONCAT22(uVar3,(paVar4 >> 0x10));
    //        1008:389a  7e  37  08  10      addr         pass1_1008_377e
    //
  (param_1 + 0xa) = 0x389a;
  (param_1 + 0xc) = 0x1008;
    //        1008:3aa8  14  3a  08  10      addr *       pass1_1008_3a14
  (param_1 + 0xa) = 0x3aa8;
  (param_1 + 0xc) = 0x1008;
  (param_1 + 0xe) = 0;
  (param_1 + 0x10) = 0;
  (param_1 + 0x14) = 0;
  (param_1 + 0x1c) = 0;
  (param_1 + 0x20) = 0;
    //        1010:7e28  fe  7d  10  10      addr         FUN_1010_7dfe
    //
  (param_1 + 0x22) = 0;
  param_1.offset_0x0 = 0x7e28;
  (param_1 + 0x2) = 0x1010;
    //        1010:7e38  c6  7d  10  10      addr         pass1_1010_7dc6
    //
  (param_1 + 0xa) = 0x7e38;
  (param_1 + 0xc) = 0x1010;
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(param_3,0x3),in_stack_0000fe9e,in_stack_0000ffc2
                           ,in_stack_0000ffc8,in_stack_0000ffcc);
  paVar2 = (paVar2 & 0xffff0000);
  (param_1 + 0x14) = puVar5;
  (param_1 + 0x16) = (puVar5 >> 0x10);
  if (param_1.is_null() == false) {
    paVar2 = (paVar2 | param_1);
  }
  uVar3 = (param_1 + 0x14);
  ppcVar1 = ((param_1 + 0x14) + 0x4);
  (**ppcVar1)();
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(uVar3,0x2f),in_stack_0000fe94,in_stack_0000ffb8,
                           in_stack_0000ffbe,in_stack_0000ffc2);
  (param_1 + 0x22) = puVar5;
  (param_1 + 0x24) = (puVar5 >> 0x10);
  ppcVar1 = ((param_1 + 0x22) + 0x4);
  (**ppcVar1)();
  return;
}






pub fn pass1_1010_6ca2(mut param_1: u16 ,mut param_2: u32,mut param_3: i16) -> u16

{
  let mut uVar1: u32;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut iStack10: i16;
  let mut puStack8: *mut u16;

  puStack8 = CONCAT22(0x1050,&stack0x000a);
  iStack10 = param_3;
  loop {
    puVar2 = puStack8;
    if (iStack10 == 0) {
      return 0x1;
    }
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0x2));
    uVar3 = *puVar2;
    uVar1 = (param_2 + 0x14);
    pass1_1010_a5ca(uVar3,param_1,uVar1,(uVar1 >> 0x10),uVar3);
    iStack10 = iStack10 -0x1;
    if uVar3 != 0 {break;}
  }
  return 0x0;
}
