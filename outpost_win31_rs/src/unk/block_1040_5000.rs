








pub fn pass1_1040_5626(param_1: *mut Struct57,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 )

{
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar9: *mut Struct57;
  let mut iVar6: i16;
  let mut uVar11: *mut Struct57;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut piStack12: *mut i16;
  let mut uVar1: u32;
  let mut paVar5: *mut Struct57;

  struct_1040_b082(param_2,CONCAT22(param_4,0xfa3));
  uVar11 = (param_2 >> 0x10);
  iVar9 = param_2;
  uVar3 = 0;
  iVar9[0x1].field3_0x6 = 0;
  iVar9[0x1].field4_0x8 = 0;
  iVar9[0x1].field5_0xa = 0;
  iVar9[0x1].field7_0xe = 0;
  param_2.field0_0x0 = 0x6386;
  iVar9.field1_0x2 = &PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_1);
  uVar4 = param_1 | uVar3;
  paVar5 = (param_1 & 0xffff0000 | uVar4);
  if (uVar4 == 0) {
    iVar9[0x1].field1_0x2 = 0;
  }
  else {
    struct_1040_a598(CONCAT22(param_1,uVar3));
    iVar9[0x1].field1_0x2 = uVar3;
    iVar9[0x1].field2_0x4 = paVar5;
  }
  *&iVar9[0x1].field1_0x2 = 0x6;
  iVar6 = *&iVar9[0x1].field1_0x2;
  uVar3 = iVar6 * 0xa + 2;
  mem_op_1000_179c(uVar3,paVar5);
  uVar4 = paVar5;
  piStack12 = CONCAT22(uVar4,uVar3);
  if ((uVar4 | uVar3) == 0) {
    uVar2 = &iVar9[0x1].field1_0x2;
    (uVar2 + 0x2) = 0;
  }
  else {
    *piStack12 = iVar6;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar6,0xa,uVar3 + 0x2,uVar4);
    uVar2 = &iVar9[0x1].field1_0x2;
    uVar7 = (uVar2 >> 0x10);
    iVar6 = uVar2;
    (iVar6 + 0x2) = uVar3 + 2;
    (iVar6 + 0x4) = uVar4;
  }
  uVar1 = &iVar9[0x1].field1_0x2;
  (uVar1 + 0x6) = param_3;
  uVar2 = &iVar9[0x1].field1_0x2;
  (uVar2 + 0xa) = 0x4;
  uVar2 = &iVar9[0x1].field1_0x2;
  (uVar2 + 0x12) = iVar9.field5_0xa;
  uVar8 = pass1_1040_5d12(param_2);
  uVar3 = (uVar8 >> 0x10);
  if ((uVar3 | uVar8) == 0) {
    iVar9[0x1].field6_0xc = 0;
  }
  else {
    iVar9[0x1].field6_0xc = (uVar8 + 0x20);
  }
  return;
}










pub fn pass1_1040_5cd6(mut param_1: u32) -> u16

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar3 = pass1_1040_5d12(param_1);
  if (uVar3 != 0) {
    iVar1 = (uVar3 + 0x20);
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x9a) != iVar1) {
      (param_1 + 0x9a) = iVar1;
      return 0x1;
    }
  }
  return 0x0;
}





pub fn pass1_1040_5d12(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar4: u16;
  let mut iVar4: *mut astruct_440;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar3: u32;

  uVar3 = (param_1 + 0x90);
  uVar5 = (uVar3 >> 0x10);
  iVar4 = uVar3;
  uVar1 = iVar4.field6_0x6;
  uVar2 = iVar4.field7_0x8;
  uVar4 = uVar2 | uVar1;
  if (uVar4 != 0) {
    uVar6 = struct_op_1030_73a8(CONCAT22(uVar2,uVar1),uVar1,uVar4);
    return uVar6;
  }
  return 0x0;
}
pub fn pass1_1040_5d42(param_1: *mut StructB)

{
  let mut uVar1: u16;
  let mut cVar2: u8;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;

  uVar5 = pass1_1040_5d12(param_1);
  if (uVar5 != 0) {
    uVar1 = (uVar5 + 0xc);
    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if (uVar1 == 0x5f) {
      (iVar3 + 0x96) = 0x53;
      return;
    }
    if (uVar1 < 0x60) {
      cVar2 = uVar1;
      if (cVar2 == '(') {
        (iVar3 + 0x96) = 0x54;
        return;
      }
      if (cVar2 == ')') {
        (iVar3 + 0x96) = 0x55;
        return;
      }
      if (cVar2 == ']') {
        (iVar3 + 0x96) = 0x51;
        return;
      }
      if (cVar2 == '^') {
        (iVar3 + 0x96) = 0x52;
        return;
      }
    }
  }
  return;
}




pub fn pass1_1040_5dc4(param_1: *mut u8,param_2: *mut StructB)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
   let mut iVar7: *mut StructB;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut puVar9: *mut u32;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut uVar10: u16;
  let mut iStack18: i16;

  paVar7 = CONCAT22(in_register_0000000a,param_1);
  puVar9 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(unaff_SI,0x3),in_stack_0000fe8c,
                           in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  paVar7 = (paVar7 & 0xffff0000 | puVar9 >> 0x10);
  uVar3 = puVar9;
  uVar8 = (param_2 >> 0x10);
  iVar7 = param_2;
  uVar5 = (puVar9 >> 0x10);
  pass1_1010_a5ca(uVar3,uVar5,uVar3,uVar5,iVar7[0x7].field7_0xe);
  if (uVar3 == 0) {
    iVar7[0x7].max_count_field_0x10 = 0;
    iVar7[0x7].field8_0x10 = 0x1;
  }
  if (-0x1 < uVar3) {
    if (iVar7[0x7].field7_0xe < 0x72) {
      uVar10 = 0x31;
    }
    else {
      uVar10 = 0x41;
    }
    puVar9 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(unaff_SI,uVar10),in_stack_0000fe8c,
                             in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
    uVar6 = (puVar9 >> 0x10);
    uVar4 = iVar7[0x7].field7_0xe;
    ppcVar1 = (*puVar9 + 0x14);
    (**ppcVar1)(0x1010,puVar9,uVar6,uVar4,uVar4 >> 0xf);
    if ((uVar6 | uVar4) == 0) {
      iStack18 = 0;
    }
    else {
      uVar2 = (uVar4 + 0x16);
      iStack18 = (uVar2 + 0x4);
    }
    if ((iStack18 != 0) && (uVar3 != 0)) {
      uVar4 = ((iStack18 - uVar3) * 0x64) / iStack18;
      uVar6 = uVar4 / 0xa;
      iVar7[0x7].max_count_field_0x10 = uVar6;
      if (0x4 < uVar4 % 0xa) {
        iVar7[0x7].max_count_field_0x10 = uVar6 + 1;
      }
    }
  }
  return;
}



pub fn pass1_1040_5eaa(param_1: *mut StructB) -> i16

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  match (iVar1 + 0x9a) {
  0x0 |
  0x70 |
  0x71 =>{
    (iVar1 + 0x98) = 0;
    return iVar1;}
  0x1 |
  0x2 =>{
    (iVar1 + 0x98) = 0xd;
    return iVar1;}
  0x3 =>{
    (iVar1 + 0x98) = 0xe;
    return iVar1;}
  0x4 |
  0x4b =>{
    (iVar1 + 0x98) = 0xf;}

  0x5 =>{
    (iVar1 + 0x98) = 0x10;
    return iVar1;}
  0x6 =>{
    (iVar1 + 0x98) = 0x11;
    return iVar1;}
  0x7 =>{
    (iVar1 + 0x98) = 0x12;}

  0x8 =>{
    (iVar1 + 0x98) = 0x13;}

  0x9 |
  0xa |
  0xb =>{
    (iVar1 + 0x98) = 0x14;}

  0xc =>{
    (iVar1 + 0x98) = 0x18;}

  0xd =>{
    (iVar1 + 0x98) = 0x19;}

  0xe |
  0x76 =>{
    (iVar1 + 0x98) = 0x17;}

  0xf |
  0x10 |
  0x11 =>{
    (iVar1 + 0x98) = 0x1a;}

  0x12 =>{
    (iVar1 + 0x98) = 0x1b;}

  0x13 =>{
    (iVar1 + 0x98) = 0x1c;}

  0x14 =>{
    (iVar1 + 0x98) = 0x1d;}

  0x15 |
  0x16 |
  0x17 |
  0x18 |
  0x19 =>{
    (iVar1 + 0x98) = 0x1e;}

  0x1a =>{
    (iVar1 + 0x98) = 0x1f;}

  0x1b =>{
    (iVar1 + 0x98) = 0x20;}

  0x1c |
  0x1d |
  0x1e =>{
    (iVar1 + 0x98) = 0x21;}

  0x1f =>{
    (iVar1 + 0x98) = 0x22;}

  0x20 =>{
    (iVar1 + 0x98) = 0x23;}

  0x21 =>{
    (iVar1 + 0x98) = 0x24;}

  0x22 =>{
    (iVar1 + 0x98) = 0x25;}

  0x23 |
  0x24 |
  0x25 |
  0x26 |
  0x27 |
  0x28 |
  0x29 |
  0x2a |
  0x2b =>{
    (iVar1 + 0x98) = 0x26;}

  0x2c =>{
    (iVar1 + 0x98) = 0x27;}

  0x2d =>{
    (iVar1 + 0x98) = 0x28;}

  0x2e |
  0x2f |
  0x30 |
  0x31 =>{
    (iVar1 + 0x98) = 0x29;}

  0x32 |
  0x33 |
  0x34 |
  0x35 |
  0x4d =>{
    (iVar1 + 0x98) = 0x2a;}

  0x36 =>{
    (iVar1 + 0x98) = 0x2b;}

  0x37 |
  0x38 |
  0x39 =>{
    (iVar1 + 0x98) = 0x2c;}

  0x3a =>{
    (iVar1 + 0x98) = 0x2d;}

  0x3b |
  0x3c =>{
    (iVar1 + 0x98) = 0x2e;}

  0x3d =>{
    (iVar1 + 0x98) = 0x2f;}

  0x3e =>{
    (iVar1 + 0x98) = 0x30;}

  0x3f =>{
    (iVar1 + 0x98) = 0x31;}

  0x40 =>{
    (iVar1 + 0x98) = 0x32;}

  0x41 =>{
    (iVar1 + 0x98) = 0x33;}

  0x42 =>{
    (iVar1 + 0x98) = 0x34;}

  0x43 =>{
    (iVar1 + 0x98) = 0x35;}

  0x44 =>{
    (iVar1 + 0x98) = 0x36;}

  0x45 =>{
    (iVar1 + 0x98) = 0x37;}

  0x46 =>{
    (iVar1 + 0x98) = 0x38;}

  0x47 =>{
    (iVar1 + 0x98) = 0x39;}

  0x48 |
  0x49 |
  0x4a =>{
    (iVar1 + 0x98) = 0x3a;}

  0x4c =>{
    (iVar1 + 0x98) = 0x3b;}

  0x4e =>{
    (iVar1 + 0x98) = 0x3c;}

  0x4f |
  0x50 =>{
    (iVar1 + 0x98) = 0x3d;}

  0x51 |
  0x52 |
  0x53 |
  0x54 |
  0x55 =>{
    (iVar1 + 0x98) = 0x3e;}

  0x56 |
  0x57 |
  0x58 |
  0x59 |
  0x5a =>{
    (iVar1 + 0x98) = 0x3f;}

  0x5b =>{
    (iVar1 + 0x98) = 0x40;}

  0x5c |
  0x5d |
  0x5e =>{
    (iVar1 + 0x98) = 0x41;}

  0x5f |
  0x60 |
  0x61 =>{
    (iVar1 + 0x98) = 0x42;}

  0x62 |
  0x63 |
  0x64 |
  0x65 |
  0x66 =>{
    (iVar1 + 0x98) = 0x43;}

  0x67 |
  0x68 =>{
    (iVar1 + 0x98) = 0x44;}

  0x69 =>{
    (iVar1 + 0x98) = 0x45;}

  0x6a =>{
    (iVar1 + 0x98) = 0x46;}

  0x6b =>{
    (iVar1 + 0x98) = 0x47;}

  0x6c =>{
    (iVar1 + 0x98) = 0x48;}

  0x6d =>{
    (iVar1 + 0x98) = 0x49;}

  0x6e =>{
    (iVar1 + 0x98) = 0x4a;}

  0x6f =>{
    (iVar1 + 0x98) = 0x4b;}

  0x74 =>{
    (iVar1 + 0x98) = 0x15;}

  0x75 =>{
    (iVar1 + 0x98) = 0x16;}

  0x78 |
  0x7a |
  0x7b |
  0x7c |
  0x7d |
  0x7e |
  0x7f |
  0x80 |
  0x81 |
  0x82 =>{
    (iVar1 + 0x98) = 0x4c;}
  }
  return iVar1;
}
