
StructD * pass1_1028_b0a2(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_b0de(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32)

{
  pass1_1028_6af2(param_1,param_2,param_3);
  param_1->offset_0x0 = 0xb1f4;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}
pub fn pass1_1028_b108(param_1: *mut astruct_341,mut param_2: u16 ,param_3: *mut astruct_342)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_342 *iVar5;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x110,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = (astruct_342 *)param_3;
    param_1->field3_0x4 = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = &param_1->field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    param_1->field257_0x108 = iVar5->field258_0x108;
    param_1->field258_0x10c = iVar5->field259_0x10c;
    *puStack10 = 0x6e50;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0xb1f4;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_b1c8(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1028_b204(param_1: *mut u16)

{
  let mut uVar1: u16;

  struct_1030_1628((astruct_180 *)param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0xc) = 0x0;
  *param_1 = 0xb33c;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_b22c(mut param_1: u16 ,param_2: *mut u16,mut param_3: u16 ,mut param_4: u32)

{
  let mut in_register_0000000a: u16;
  let mut uVar1: u16;

  pass1_1030_165e((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_175 *)param_2,0x6000000,param_4);
  uVar1 = (param_2 >> 0x10);
  (param_2 + 0xc) = param_3;
  *param_2 = 0xb33c;
  (param_2 + 0x2) = 0x1028;
  return param_2;
}
pub fn pass1_1028_b260(param_1: *mut u16)

{
  *param_1 = 0xb33c;
  (param_1 + 0x2) = 0x1028;
  pass1_1030_16b2(param_1);
  return;
}
pub fn FUN_1028_b27e(void)

{
  return;
}



BOOL16 FUN_1028_b282(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  let mut in_AX: bool;
  let mut BVar1: bool;
  HFILE16 in_stack_0000ffde;
  u16 auStack12 [0x5];

  pass1_1030_16d6((astruct_731 *)param_2,param_3);
  if (in_AX != 0x0) {
    auStack12[0] = (param_2 + 0xc);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,auStack12),0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    in_AX = 0x1;
  }
  return in_AX;
}



BOOL16 pass1_1028_b2c8(BOOL16 param_1,param_2: *mut astruct_373,HFILE16 *param_3)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut local_4: u16;

  file_1030_1730(param_2,param_3);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,&local_4),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    uVar2 = switch_1008_72bc(param_3,local_4);
    (param_2 + 0xc) = uVar2;
    param_1 = 0x1;
  }
  return param_1;
}



StructD * pass1_1028_b316(StructD *param_1,param_2: u8)

{
  pass1_1028_b260(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1028_b354(param_1: *mut astruct_180)

{
  astruct_180 *iVar1;
  let mut uVar1: u16;

  struct_1030_1628(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  iVar1->field10_0xc = 0x0;
  iVar1->field11_0xe = 0x0;
  iVar1->field12_0x10 = 0x0;
  iVar1->field13_0x12 = 0x0;
  iVar1->field16_0x18 = 0x0;
  iVar1->field17_0x1a = 0x0;
  iVar1->field18_0x1c = 0x0;
  param_1->field0_0x0 = 0xcf6a;
  iVar1->field1_0x2 = 0x1028;
  iVar1->field15_0x16 = 0x0;
  iVar1->field14_0x14 = 0x0;
  return;
}
pub fn pass1_1028_b39e(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut in_register_0000000a: u16;
  astruct_12 *iVar1;
  let mut uVar1: u16;

  pass1_1030_165e((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_175 *)param_2,0x7000000,param_4);
  uVar1 = (param_2 >> 0x10);
  iVar1 = (astruct_12 *)param_2;
  iVar1->field10_0xc = param_3;
  iVar1->field11_0xe = 0x42;
  iVar1->field12_0x10 = 0x0;
  iVar1->field13_0x12 = 0x0;
  iVar1->field18_0x18 = 0x0;
  iVar1->field19_0x1a = 0x0;
  iVar1->field20_0x1c = 0x0;
  param_2->field0_0x0 = 0xcf6a;
  iVar1->field1_0x2 = 0x1028;
  pass1_1028_bf76(0x0,(astruct_12 *)(param_2 & 0xffff | uVar1 << 0x10));
  &iVar1->field_0x14 = 0x0;
  if ((0x4e < iVar1->field10_0xc) && (iVar1->field10_0xc < 0x70)) {
    iVar1->field11_0xe = 0x6b;
  }
  return;
}
pub fn pass1_1028_b418(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  *param_1 = 0xcf6a;
  (iVar2 + 0x2) = 0x1028;
  iVar1 = (iVar2 + 0x12);
  if (((iVar1 == 0x4) || (iVar1 == 0x5)) ||
     ((iVar1 == 0x6 && ((iVar1 = (iVar2 + 0x18), iVar1 == 0x4 || (iVar1 == 0x5)))))) {
    fn_ptr_1000_17ce(*(char **)(iVar2 + 0x14));
  }
  pass1_1030_16b2(param_1);
  return;
}
pub fn pass1_1028_b46e(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  let mut paVar4: *mut Struct57;
  astruct_302 *paVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;

  uVar3 = (in_EDX >> 0x10);
  paVar5 = (astruct_302 *)pass1_1028_b4f2(param_2);
  paVar4 = (astruct_57 *)CONCAT22(uVar3,(paVar5 >> 0x10));
  iVar2 = paVar5;
  uVar6 = 0x0;
  uVar7 = 0x0;
  pass1_1028_b58e(param_2);
  uVar3 = SUB42(paVar4,0x0);
  pass1_1030_6d80((astruct_299 *)CONCAT22(uVar3,iVar2),CONCAT22(uVar7,uVar6));
  iVar1 = (iVar2 + 0x32);
  if (iVar1 != 0x0) {
    pass1_1030_6c4c(CONCAT22(uVar3,iVar2),0x0);
    pass1_1038_387e(paVar4,paVar5,0x0,iVar1,CONCAT22(uVar3,iVar2));
  }
  fn_ptr_1030_7296((astruct_292 *)CONCAT22(uVar3,iVar2));
  (param_2 + 0x1c) = (param_3 + 0x200);
  return;
}



u16 FUN_1028_b4e6(void)

{
  return 0x0;
}



u16 FUN_1028_b4ec(void)

{
  return 0x0;
}



pub fn pass1_1028_b4f2(param_1: *mut astruct_15) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  uVar2 = pass1_1028_b58e(param_1);
  uVar1 = (uVar2 >> 0x10);
  return CONCAT22((uVar2 + 0x30),(uVar2 + 0x2e));
}
pub fn pass1_1028_b514(param_1: *mut astruct_15)

{
  let mut iVar1: i16;
  astruct_15 *pstruct15_1;
  let mut pstruct15_1_hi: u16;
  astruct_290 *paVar3;

  pstruct15_1_hi = (param_1 >> 0x10);
  pstruct15_1 = (astruct_15 *)param_1;
  iVar1 = pstruct15_1->field15_0x12;
  if (((iVar1 == 0x4) || (iVar1 == 0x5)) ||
     ((iVar1 == 0x6 && ((iVar1 = pstruct15_1->field17_0x18, iVar1 == 0x4 || (iVar1 == 0x5)))))) {
    fn_ptr_1000_17ce(pstruct15_1->field16_0x14);
  }
  pstruct15_1->field16_0x14 = NULL;
  pstruct15_1->field15_0x12 = 0x7;
  paVar3 = (astruct_290 *)pass1_1028_b58e((astruct_15 *)(param_1 & 0xffff | pstruct15_1_hi << 0x10));
  paVar3 = paVar3;
  fn_ptr_1030_7296((astruct_292 *)paVar3);
  pass1_1030_72d0((astruct_292 *)paVar3);
  pass1_1030_730a(paVar3,paVar3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_b58e(param_1: *mut astruct_15)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_1 + 0x8));
  return;
}



u16 pass1_1028_b5a8(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x12) != 0x5) {
    return 0x0;
  }
  uVar1 = (param_1 + 0x14);
  return (uVar1 + 0x94);
}



u16 pass1_1028_b5ca(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x12) != 0x5) {
    return 0x0;
  }
  uVar1 = (param_1 + 0x14);
  return (uVar1 + 0x9c);
}



BOOL16 write_to_file_1028_b5ec(param_1: *mut astruct_731,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffbc;
  u16 local_e [0x3];
  u16 local_8 [0x2];
  let mut iStack4: i16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  local_e[0] = (iVar3 + 0xc);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_e),0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  pass1_1030_16d6(param_1,param_2);
  if (BVar2 == 0x0) {
    return 0x0;
  }
  local_8[0] = (iVar3 + 0xc);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0xe);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x10);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x12);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x18);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x1a);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  iStack4 = (iVar3 + 0x12);
  if (iStack4 == 0x6) {
    iStack4 = (iVar3 + 0x18);
  }
  if (iStack4 < 0x1) {
    return 0x1;
  }
  if (SBORROW2(iStack4,0x1)) {
    return 0x1;
  }
  if (iStack4 == 0x3 || iStack4 + -0x2 < 0x1) {
    local_8[0] = (iVar3 + 0x14);
  }
  else if (iStack4 == 0x4) {
    if (*(i32 *)(iVar3 + 0x14) == 0x0) {
      local_8[0] = 0x0;
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
      goto joined_r0x1028b766;
    }
    uVar1 = (iVar3 + 0x14);
    local_8[0] = (uVar1 + 0x94);
  }
  else {
    if (iStack4 != 0x5) {
      return 0x1;
    }
    uVar1 = (iVar3 + 0x14);
    local_8[0] = (uVar1 + 0xa4);
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (iVar3 + 0x14);
    local_8[0] = (uVar1 + 0xa6);
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (iVar3 + 0x14);
    local_8[0] = (uVar1 + 0xa8);
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (iVar3 + 0x14);
    local_8[0] = (uVar1 + 0xaa);
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (iVar3 + 0x14);
    local_8[0] = (uVar1 + 0xac);
  }
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffbc);
joined_r0x1028b766:
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  return 0x1;
}



// WARNING: Unable to use type for symbol temp_7fffad5008a
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn file_1028_b81a(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u32;
  let mut uVar8: u16;
  u16 local_2a [0x2];
  u8 local_26 [0x16];
  let mut puStack16: *mut u32;
  let mut uStack14: u16;
  let mut iStack10: i16;
  let mut local_8: i16;
  let mut local_6: i16;
  let mut local_4: i16;
  let mut temp_7fffad5008a: *mut u32;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  temp_7fffad5008a = param_3;
  uVar5 = (param_3 >> 0x10);
  file_1030_1730(param_3,param_4);
  if (param_1 == 0x0) {
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_6),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_8),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  uVar4 = param_4;
  uVar8 = (param_4 >> 0x10);
  iVar2 = switch_1008_73ea(uVar4,uVar8,local_4);
  (temp_7fffad5008a + 0x3) = iVar2;
  iVar2 = switch_1008_73ea(uVar4,uVar8,local_6);
  (temp_7fffad5008a + 0xe) = iVar2;
  iVar2 = switch_1008_73ea(uVar4,uVar8,local_8);
  (temp_7fffad5008a + 0x4) = iVar2;
  BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_6),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (temp_7fffad5008a + 0x1a)),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  (temp_7fffad5008a + 0x12) = local_4;
  (temp_7fffad5008a + 0x6) = local_6;
  iStack10 = (temp_7fffad5008a + 0x12);
  if (iStack10 == 0x6) {
    iStack10 = (temp_7fffad5008a + 0x6);
  }
  switch(iStack10) {
  case 0x1:
  case 0x2:
  case 0x3:
    puVar3 = temp_7fffad5008a + 0x5;//
LAB_1028_b968:
    BVar1 = read_file_1008_7dee(param_4,CONCAT22(uVar5,puVar3),0x2);
    break;
  case 0x4:
    uVar7 = pass1_1028_e0bc(temp_7fffad5008a,paVar6,_PTR_LOOP_1050_65e2,(temp_7fffad5008a + 0x3));
    uStack14 = (uVar7 >> 0x10);
    (temp_7fffad5008a + 0x5) = uVar7;
    (temp_7fffad5008a + 0x16) = uStack14;
    if ((uStack14 | (temp_7fffad5008a + 0x5)) != 0x0) {
      puVar3 = ((temp_7fffad5008a + 0x5) + 0x94);
      uVar5 = uStack14;
      puStack16 = puVar3;
      goto LAB_1028_b968;
    }
    BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_26),0x2);
    break;
  case 0x5:
    puVar3 = temp_7fffad5008a;
    pass1_1028_e100(paVar6,_PTR_LOOP_1050_65e2,(temp_7fffad5008a + 0x3));
    (u32*)(temp_7fffad5008a + 0x5) = puVar3;
    uStack14 = paVar6;
    (temp_7fffad5008a + 0x16) = uStack14;
    puStack16 = ((temp_7fffad5008a + 0x5) + 0xa4);
    BVar1 = read_file_1008_7dee(param_4,CONCAT22(uStack14,puStack16),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_2a),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar7 = temp_7fffad5008a[0x5];
    BVar1 = read_file_1008_7dee(param_4,(uVar7 & 0xffff0000 | (uVar7 + 0xa8)),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar7 = temp_7fffad5008a[0x5];
    BVar1 = read_file_1008_7dee(param_4,(uVar7 & 0xffff0000 | (uVar7 + 0xaa)),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar7 = temp_7fffad5008a[0x5];
    BVar1 = read_file_1008_7dee(param_4,(uVar7 & 0xffff0000 | (uVar7 + 0xac)),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar4 = switch_1008_72bc(param_4,local_2a[0]);
    uVar7 = temp_7fffad5008a[0x5];
    (uVar7 + 0xa6) = uVar4;
    return;
  default:
    goto switchD_1028_ba97_caseD_6;
  case 0x9:
    puVar3 = temp_7fffad5008a;
    pass1_1028_e100(paVar6,_PTR_LOOP_1050_65e2,(temp_7fffad5008a + 0x3));
    (u32*)(temp_7fffad5008a + 0x5) = puVar3;
    (temp_7fffad5008a + 0x16) = paVar6;
    goto switchD_1028_ba97_caseD_6;
  }
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
switchD_1028_ba97_caseD_6:
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bab6(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_bad4(param_1,param_2,param_3);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  return;
}



pub fn pass1_1028_bad4(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15) -> u32

{
  pass1_1028_baf6(param_3);
  return CONCAT22((param_1 + 0xa),(param_1 + 0x8));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_baf6(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_bb24(param_1);
  if (uVar1 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  return;
}



pub fn pass1_1028_bb24(param_1: *mut astruct_15) -> u32

{
  let mut uVar1: u16;
  astruct_15 *uVar2;
  let mut uVar3: u32;

  uVar2 = (astruct_15 *)(param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x8) == 0x0) {
    return 0x0;
  }
  uVar3 = pass1_1028_b58e((astruct_15 *)(param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
  uVar1 = (uVar3 >> 0x10);
  return CONCAT22((uVar3 + 0xa),(uVar3 + 0x8));
}
pub fn pass1_1028_bb56(mut param_1: u32,mut param_2: u32)

{
  pass1_1030_177a(param_1,param_2);
  return;
}



pub fn pass1_1028_bb6a(mut param_1: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((iVar1 + 0x12) != 0x5) && ((iVar1 + 0x12) != 0x6)) {
    return 0x0;
  }
  return CONCAT22((iVar1 + 0x16),(iVar1 + 0x14) + 0xa4);
}
pub fn pass1_1028_bb96(param_1: *mut astruct_295,u32 *param_2,mut param_3: u16 )

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut uVar3: u32;
  let mut iVar6: i16;
  astruct_295 *iVar5;
  astruct_297 *iVar4;
  let mut puVar7: *mut u32;
  astruct_295 *uVar8;
  let mut uVar9: u16;

  uVar8 = (astruct_295 *)(param_1 >> 0x10);
  iVar5 = (astruct_295 *)param_1;
  if ((&iVar5->field_0x12 == 0x5) || (&iVar5->field_0x12 == 0x6)) {
    uVar3 = iVar5->field20_0x14;
    uVar9 = (uVar3 >> 0x10);
    puVar7 = (uVar3 + 0xa4);
    for (iVar6 = 0x2; iVar6 != 0x0; iVar6 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = param_2;
      param_2 = param_2 + 0x1;
      *puVar2 = *puVar1;
    }
    puVar7 = param_2;
    pass1_1028_c724(param_1);
    uVar3 = iVar5->field20_0x14;
    uVar9 = (uVar3 >> 0x10);
    iVar4 = (astruct_297 *)uVar3;
    if (iVar4->field170_0xaa == 0x0) {
      iVar4->field170_0xaa = 0x1;
    }
  }
  return;
}
pub fn pass1_1028_bbf0(mut param_1: u16 ,mut param_2: u16 ,u32 *param_3)

{
  *param_3 = 0x0;
  return;
}
pub fn pass1_1028_bc02(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)(*param_1 + 0x40);
  (**ppcVar1)();
  return;
}



u16 pass1_1028_bc1c(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x12) == 0x4) {
    return (iVar1 + 0xe);
  }
  if ((iVar1 + 0x12) == 0x7) {
    return (iVar1 + 0x10);
  }
  return (iVar1 + 0xc);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_bc4a(u32 *param_1,u8 *param_2,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut pcVar2: *mut c_char;

  pcVar2 = pass1_1028_e0bc(param_1,(astruct_57 *)CONCAT22(in_register_0000000a,param_2),_PTR_LOOP_1050_65e2,
                                   (param_3 + 0xc));
  uVar1 = (pcVar2 + 0x96);
  fn_ptr_1000_17ce(pcVar2);
  return uVar1;
}
pub fn pass1_1028_bc7e(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x4);
  return;
}



u16 pass1_1028_bc90(mut param_1: i16,mut param_2: u16 ,u32 *param_3,param_4: *mut u16,mut param_5: u32,mut param_6: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut BVar4: bool;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;

  uVar6 = param_3;
  uVar7 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar6,uVar7,param_4,param_6);
  if ((param_1 == 0x5) || (param_1 == 0x6)) {
    uVar2 = *param_3;
    ppcVar1 = (code **)(uVar2 + 0x60);
    iVar3 = (**ppcVar1)();
    if (iVar3 != 0x0) {
      ppcVar1 = (code **)(uVar2 + 0x5c);
      uVar5 = (**ppcVar1)();
      if (uVar5 != 0x0) {
        pass1_1028_c23e(uVar5,(uVar5 >> 0x10),uVar6,uVar7,param_4,param_5,param_6);
        if (uVar5 != 0x0) {
          BVar4 = pass1_1028_c314(uVar5,(uVar5 >> 0x10),uVar6,uVar7,param_4,param_5,
                                  (param_5 >> 0x10),param_6);
          if (BVar4 != 0x0) {
            return 0x1;
          }
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = 0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bd38(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  astruct_117 *pstruct117_4;
  astruct_117 *pstruct117_6;
  astruct_117 *pstruct117_5;
  let mut extraout_DX: u16;
  let mut iStack20: i16;

  pstruct117_5 = *(astruct_117 **)(_PTR_LOOP_1050_65e2 + 0x52);
  pass1_1030_4bbe(param_1,pstruct117_5,(param_2 + 0xc));
  pstruct117_4 = (astruct_117 *)pstruct117_5;
  pstruct117_6 = pstruct117_4;
  pass1_1028_b58e(param_2);
  uVar3 = (&pstruct117_6[0x1].field15_0x12 + 0x2);
  iStack20 = 0x11;
  do {
    uVar1 = (&pstruct117_4->field_0x0 + iStack20 * 0x4);
    uVar2 = (&pstruct117_4->field_0x2 + iStack20 * 0x4);
    if ((uVar2 | uVar1) != 0x0) {
      pass1_1038_5770(uVar3,CONCAT22(uVar2,uVar1),iStack20);
    }
    iStack20 += 0x1;
  } while (iStack20 < 0x25);
  return;
}
pub fn pass1_1028_bdac(param_1: *mut astruct_15,mut param_2: i16)

{
  let mut iVar1: i16;
  code **ppcVar2;
  astruct_15 *pstruct15_3;
  param_1: *mut astruct_15_hi;
  let mut unaff_CS: u16;

  param_1_hi = (astruct_15 *)(param_1 >> 0x10);
  pstruct15_3 = (astruct_15 *)param_1;
  if (pstruct15_3->field15_0x12 != param_2) {
    if (pstruct15_3->field15_0x12 == 0x6) {
      if (pstruct15_3->field17_0x18 == param_2) {
        pstruct15_3->field15_0x12 = pstruct15_3->field17_0x18;
        pstruct15_3->field17_0x18 = 0x0;
        return;
      }
    }
    else {
      if (param_2 != 0x6) {
        iVar1 = pstruct15_3->field15_0x12;
        if ((iVar1 == 0x4) || (iVar1 == 0x5)) {
          unaff_CS = 0x1000;
          fn_ptr_1000_17ce(pstruct15_3->field16_0x14);
        }
        pstruct15_3->field15_0x12 = param_2;
        ppcVar2 = (code **)(param_1 + 0x3c);
        (**ppcVar2)(unaff_CS,param_1);
        return;
      }
      pstruct15_3->field17_0x18 = pstruct15_3->field15_0x12;
      pstruct15_3->field15_0x12 = 0x6;
    }
  }
  return;
}
pub fn pass1_1028_be2a(param_1: *mut astruct_15)

{
  code **ppcVar1;
  astruct_15 *uVar2;
  let mut uVar3: u32;
  let mut iVar4: i16;

  uVar2 = (astruct_15 *)(param_1 >> 0x10);
  if ((param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if (*(i32 *)(uVar3 + 0x200) != 0x8000002) {
    if (*(i32 *)(param_1 + 0x1c) == 0x8000002) {
      iVar4 = 0x6;
      goto code_r0x1028be96;
    }
    ppcVar1 = (code **)(param_1 + 0x64);
    iVar4 = (**ppcVar1)();
    if (iVar4 == 0x0) {
      return;
    }
    pass1_1028_cb04(param_1);
    if (iVar4 == 0x0) {
      iVar4 = 0x6;
      goto code_r0x1028be96;
    }
    pass1_1028_c952(param_1);
  }
  iVar4 = 0x5;
code_r0x1028be96:
  pass1_1028_bdac(param_1,iVar4);
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn pass1_1028_be9e(param_1: *mut astruct_15)

{
  let mut piVar1: *mut i16;
  StructD *pSVar2;
  let mut iVar3: i16;
  astruct_15 *pstruct15_4;
  astruct_15 *uVar5;
  let mut uVar4: u32;
  StructD *uVar2;

  uVar5 = (astruct_15 *)(param_1 >> 0x10);
  pstruct15_4 = (astruct_15 *)param_1;
  if (pstruct15_4->field15_0x12 == 0x4) {
    uVar4 = pass1_1028_b4f2(param_1);
    iVar3 = uVar4;
    if (*(i32 *)(iVar3 + 0x200) == 0x8000002) {
      pSVar2 = pstruct15_4->field16_0x14;
      piVar1 = (pSVar2 + 0x94);
      *piVar1 = *piVar1 + -0x1;
    }
    else {
      pass1_1028_cb04(param_1);
      if (iVar3 == 0x0) {
        return;
      }
      pSVar2 = pstruct15_4->field16_0x14;
      piVar1 = (pSVar2 + 0x94);
      *piVar1 = *piVar1 + -0x1;
      pass1_1028_c952(param_1);
    }
    uVar2 = pstruct15_4->field16_0x14;
    if ((uVar2 + 0x94) < 0x1) {
      pass1_1028_bdac(param_1,0x5);
    }
  }
  return;
}
pub fn FUN_1028_bf16(void)

{
  return;
}
pub fn FUN_1028_bf1a(void)

{
  return;
}
pub fn FUN_1028_bf1e(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bf22(u8 *param_1,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  iVar1 = (iVar4 + 0x12);
  if (iVar1 == 0x4) {
    uVar6 = pass1_1028_e0bc(NULL,paVar3,_PTR_LOOP_1050_65e2,(iVar4 + 0xc));
    uVar2 = (uVar6 >> 0x10);
    iVar1 = uVar6;
  }
  else {
    iVar1 += -0x5;
    if (iVar1 != 0x0) {
      if (iVar1 != 0x1) {
        (iVar4 + 0x14) = 0x0;
      }
      return;
    }
    pass1_1028_e100(param_1,_PTR_LOOP_1050_65e2,(iVar4 + 0xc));
    uVar2 = SUB42(paVar3,0x0);
  }
  (iVar4 + 0x14) = iVar1;
  (iVar4 + 0x16) = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bf76(mut param_1: u16 ,param_2: *mut astruct_12)

{
  let mut BVar1: bool;
  astruct_12 *iVar2;
  let mut uVar2: u16;

  pass1_1008_612e(param_1,0x1,0x3);
  uVar2 = (param_2 >> 0x10);
  iVar2 = (astruct_12 *)param_2;
  BVar1 = pass1_1008_c6ae(_u16_1050_06e0,iVar2->field10_0xc,0x28);
  if (BVar1 == 0x0) {
    if (param_1 == 0x1) {
      iVar2->field12_0x10 = 0x48;
      return;
    }
    if (param_1 != 0x2) {
      iVar2->field12_0x10 = 0x4a;
      return;
    }
    iVar2->field12_0x10 = 0x49;
    return;
  }
  if (param_1 == 0x1) {
    iVar2->field12_0x10 = 0x70;
    return;
  }
  if (param_1 != 0x2) {
    iVar2->field12_0x10 = 0x72;
    return;
  }
  iVar2->field12_0x10 = 0x71;
  return;
}


