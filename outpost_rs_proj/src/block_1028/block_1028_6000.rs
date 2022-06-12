
pub fn pass1_1028_6008(u32 *param_1)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  pass1_1028_be2a((astruct_15 *)param_1);
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0x12) == 0x5) && (0x0 < (iVar2 + 0x20))) {
    piVar1 = (iVar2 + 0x20);
    *piVar1 = *piVar1 + -0x1;
  }
  return;
}



StructD * pass1_1028_602e(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_60bc(param_1: *mut astruct_180,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_180 *iVar1;
  let mut uVar3: u16;

  struct_1028_b354(param_1);
  uVar3 = (param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  (iVar1 + 0x1) = 0x0;
  param_1.field0_0x0 = 0x6876;
  iVar1.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xc,param_2);
  uVar2 = param_2 | param_3;
  if (uVar2 == 0x0) {
    (iVar1 + 0x1) = 0x0;
  }
  else {
    uVar1 = set_struct_1008_574a((astruct_57 *)CONCAT22(param_2,param_3));
    (iVar1 + 0x1)->field0_0x0 = uVar1;
    iVar1[0x1].field1_0x2 = uVar2;
  }
  return &param_1.field0_0x0;
}



pub fn pass1_1028_611e(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;

  uVar1 = (param_1 >> 0x10);
  paVar4 = (astruct_57 *)(param_1 & 0xffff0000 | param_1 & 0xffff);
  pass1_1028_b39e((StructD *)(param_1 & 0xffff),param_2,param_3,param_4);
  (param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0x6876;
  (param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xc,paVar4);
  uVar3 = paVar4 | uVar1;
  if (uVar3 == 0x0) {
    (param_2 + 0x20) = 0x0;
  }
  else {
    uVar2 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar4,uVar1));
    (param_2 + 0x20) = uVar2;
    (param_2 + 0x22) = uVar3;
  }
  return param_2;
}
pub fn pass1_1028_6186(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar4;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1.address_offset_field_0x0 = 0x6876;
  iVar4.address_offset_field_0x2 = 0x1028;
  puVar1 = iVar4.field19_0x20;
  uVar2 = iVar4.field20_0x22;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  return;
}
pub fn pass1_1028_61c4(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  astruct_21 *paVar1;
  astruct_21 *uVar4;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  astruct_15 *iVar5;
  let mut uVar6: u16;
  astruct_21 *puVar1;
  let mut paVar3: *mut Struct57;
  code **fn_ptr_1;

  uVar5 = (in_EDX >> 0x10);
  pass1_1028_b46e(param_1,param_2,param_3);
  uVar6 = (param_2 >> 0x10);
  iVar5 = (astruct_15 *)param_2;
  paVar1 = iVar5.field24_0x20;
  uVar2 = iVar5.field25_0x22;
  uVar4 = (astruct_21 *)(uVar2 | paVar1);
  paVar3 = (astruct_57 *)CONCAT22(uVar5,uVar4);
  if (uVar4 != NULL) {
    fn_ptr_1 = (code **)paVar1;
    paVar1 = (astruct_21 *)(**fn_ptr_1)();
  }
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | paVar1;
  if (uVar2 == 0x0) {
    paVar1 = NULL;
    uVar2 = 0x0;
  }
  else {
    paVar1 = (astruct_21 *)set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,paVar1));
  }
  iVar5.field24_0x20 = paVar1;
  iVar5.field25_0x22 = uVar2;
  return;
}
pub fn pass1_1028_6228(mut param_1: u32,mut param_2: u16 ,mut param_3: i16,mut param_4: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut bVar8: bool;
  i32 lVar9;
  u8 local_a [0x4];
  let mut uStack6: u32;

  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_a),(iVar6 + 0x20));
  while( true ) {
    do {
      lVar9 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      uVar5 = (lVar9 >> 0x10);
      iVar4 = lVar9;
      if (lVar9 == 0x0) {
        return;
      }
    } while ((iVar4 + 0x6) != param_4);
    uVar1 = (iVar4 + 0xa);
    if ((param_3 == 0x0) && (param_2 < uVar1)) break;
    bVar8 = param_2 < uVar1;
    param_2 -= uVar1;
    param_3 -= bVar8;
    ppcVar3 = (code **)((iVar6 + 0x20) + 0xc);
    (**ppcVar3)(0x1008,(iVar6 + 0x20));
    uStack6 = 0x0;
  }
  uVar2 = (iVar4 + 0xc);
  (iVar4 + 0xa) = uVar1 - param_2;
  (iVar4 + 0xc) = -(param_2 * (uVar2 / uVar1) - (iVar4 + 0xc));
  return;
}



pub fn pass1_1028_62c8(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x12) == 0x5) {
    uVar2 = pass1_1028_67d4(param_1 & 0xffff | uVar1 << 0x10);
    uVar1 = uVar2;
    if (((uVar2 >> 0x10) == 0x0) && (uVar1 < 0x64)) {
      return CONCAT22(-(0x64 < uVar1),0x64 - uVar1);
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_6302(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  i32 lVar3;
  let mut uStack18: u32;
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x20));
  uStack18 = 0x0;
  while( true ) {
    lVar3 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    uVar2 = (lVar3 >> 0x10);
    if (lVar3 == 0x0) break;
    if ((lVar3 + 0x8) != 0x0) {
      uVar1 = (lVar3 + 0xa);
      uStack18 = CONCAT22((uStack18 >> 0x10) + CARRY2(uStack18,uVar1),uStack18 + uVar1);
    }
  }
  return uStack18;
}
pub fn pass1_1028_6356(mut param_1: u32,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  code **ppcVar4;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut bVar9: bool;
  i32 lVar10;
  u8 local_a [0x4];
  let mut uStack6: u32;

  uVar8 = (param_1 >> 0x10);
  iVar7 = param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_a),(iVar7 + 0x20));
  while( true ) {
    do {
      lVar10 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      uVar6 = (lVar10 >> 0x10);
      iVar5 = lVar10;
      if (lVar10 == 0x0) {
        return;
      }
    } while ((((iVar5 + 0x8) == 0x0) || ((param_2 != 0x0 && ((iVar5 + 0x8) != param_2)))) ||
            (((iVar5 + 0x8) == 0xf && (param_2 != 0xf))));
    uVar2 = (iVar5 + 0xa);
    if ((param_4 == 0x0) && (param_3 < uVar2)) break;
    bVar9 = param_3 < uVar2;
    param_3 -= uVar2;
    param_4 -= bVar9;
    ppcVar4 = (code **)((iVar7 + 0x20) + 0xc);
    (**ppcVar4)(0x1008,(iVar7 + 0x20));
    uStack6 = 0x0;
  }
  uVar3 = (iVar5 + 0xc);
  piVar1 = (iVar5 + 0xa);
  *piVar1 = *piVar1 - param_3;
  piVar1 = (iVar5 + 0xc);
  *piVar1 = *piVar1 - param_3 * (uVar3 / uVar2);
  return;
}
pub fn pass1_1028_6408(mut param_1: u32,u32 *param_2)

{
  code **ppcVar1;
  let mut bVar2: bool;
  let mut puVar3: *mut u8,
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u8 local_a [0x8];

  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_a),(iVar4 + 0x20));
  bVar2 = false;
  while( true ) {
    puVar3 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar3));
    iVar5 = param_2;
    uVar7 = (param_2 >> 0x10);
    if ((extraout_DX | puVar3) == 0x0) break;
    if (((puVar3 + 0x4) == (iVar5 + 0x4)) && ((puVar3 + 0x6) == (iVar5 + 0x6))) {
      if ((puVar3 + 0x8) == (iVar5 + 0x8)) {
        bVar2 = true;
        (puVar3 + 0xa) = (puVar3 + 0xa) + (iVar5 + 0xa);
        (puVar3 + 0xc) = (puVar3 + 0xc) + (iVar5 + 0xc);
      }
    }
  }
  if (bVar2) {
    if (param_2 != NULL) {
      ppcVar1 = (code **)*param_2;
      (**ppcVar1)(0x1008,param_2,0x1,param_2,param_2);
      return;
    }
  }
  else {
    ppcVar1 = (code **)((iVar4 + 0x20) + 0x4);
    (**ppcVar1)(0x1008,(iVar4 + 0x20),param_2);
  }
  return;
}



u16 pass1_1028_64d6(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffc4;
  let mut local_26: u16;
  let mut local_24: u16;
  let mut local_22: u16;
  let mut local_20: u16;
  let mut local_1e: u16;
  u16 local_1c [0x6];
  let mut uStack16: u16;
  i32 lStack14;
  u8 local_a [0x8];

  BVar2 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar2 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x20));
    uVar1 = (param_1 + 0x20);
    local_1c[0] = (uVar1 + 0x8);
    puVar3 = local_1c;
    uStack16 = local_1c[0];
    while( true ) {
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,puVar3),0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      lStack14 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      if (lStack14 == 0x0) {
        return 0x1;
      }
      local_1e = (lStack14 + 0x4);
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_1e),0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_20 = (lStack14 + 0x6);
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_20),0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_22 = (lStack14 + 0x8);
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_22),0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_24 = (lStack14 + 0xa);
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_24),0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_26 = (lStack14 + 0xc);
      puVar3 = &local_26;
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_65e2(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut local_16: u16;
  astruct_99 *paStack20;
  u16 local_10 [0x2];
  u16 local_c [0x3];
  let mut uStack6: u16;
  let mut local_4: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    BVar3 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
    if (BVar3 != 0x0) {
      uStack6 = 0x0;
      while( true ) {
        if (local_4 <= uStack6) {
          return;
        }
        paStack20 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar5 = (paStack20 >> 0x10);
        uVar2 = paStack20;
        if ((uVar5 | uVar2) == 0x0) {
          paStack20 = NULL;
        }
        else {
          paStack20.field0_0x0 = 0x389a;
          (uVar2 + 0x2) = 0x1008;
          (uVar2 + 0x4) = 0x0;
          (uVar2 + 0x6) = 0x0;
          (uVar2 + 0x8) = 0x0;
          (uVar2 + 0xa) = 0x0;
          (uVar2 + 0xc) = 0x0;
          paStack20->field0_0x0 = 0x56ce;
          (uVar2 + 0x2) = 0x1018;
        }
        BVar3 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_10),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_c),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_16),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,(paStack20 & 0xffff0000 | (paStack20 + 0xa)),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,(paStack20 & 0xffff0000 | (paStack20 + 0xc)),0x2);
        if (BVar3 == 0x0) break;
        (paStack20 + 0x4) = local_10[0];
        uVar4 = switch_1008_72bc(param_4,local_c[0]);
        uVar6 = (paStack20 >> 0x10);
        (paStack20 + 0x6) = uVar4;
        (paStack20 + 0x8) = local_16;
        ppcVar1 = (code **)((param_3 + 0x20) + 0x8);
        (**ppcVar1)();
        uStack6 += 0x1;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



u16 pass1_1028_6744(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;
  i32 lVar2;
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x20));
  do {
    lVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    uVar1 = (lVar2 >> 0x10);
    if (lVar2 == 0x0) {
      return 0x0;
    }
  } while ((lVar2 + 0x6) != param_2);
  return (lVar2 + 0xa);
}



u16 pass1_1028_678c(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;
  i32 lVar2;
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x20));
  do {
    lVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    uVar1 = (lVar2 >> 0x10);
    if (lVar2 == 0x0) {
      return 0x0;
    }
  } while ((lVar2 + 0x8) != param_2);
  return (lVar2 + 0xa);
}



// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_67d4(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  i32 lVar2;
  let mut uStack18: u32;
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x20));
  uStack18 = 0x0;
  while( true ) {
    lVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar2 == 0x0) break;
    uVar1 = (lVar2 + 0xc);
    uStack18 = CONCAT22((uStack18 >> 0x10) + CARRY2(uStack18,uVar1),uStack18 + uVar1);
  }
  return uStack18;
}



u16 pass1_1028_6822(mut param_1: u32,param_2: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u32;

  uVar2 = pass1_1028_67d4(param_1);
  iVar1 = (uVar2 >> 0x10);
  *param_2 = uVar2;
  (param_2 + 0x2) = iVar1;
  if ((iVar1 == 0x0) && (*param_2 < 0x64)) {
    return 0x0;
  }
  return 0x1;
}



StructD * pass1_1028_6850(StructD *param_1,param_2: u8)

{
  pass1_1028_6186(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1028_68de(param_1: *mut astruct_97,mut param_2: u16 ,mut param_3: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e8);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  &iVar1->field259_0x108 = param_3;
  iVar1->field262_0x10c = param_2;
  param_1->offset_0x0 = 0x6ae2;
  iVar1->segment_0x2 = 0x1028;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCAddSpew_1050_4fd2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6926(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u8,
  let mut puVar7: *mut u8,
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  let mut puStack14: *mut u32;

  uVar9 = (param_3 >> 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_3 + 0x108));
  puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0xa);
  puVar6 = (puVar11 >> 0x10);
  uVar4 = puVar11;
  uVar10 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4d6e(uVar4,puVar6,(astruct_691 *)CONCAT22(param_2,param_1),puVar11);
  puStack14 = CONCAT22(puVar6,uVar4);
  uVar3 = *puStack14;
  ppcVar2 = (code **)(uVar3 + 0x10);
  puVar7 = puVar6;
  uVar5 = uVar4;
  (**ppcVar2)(&u16_1050_1038,uVar4,puVar6);
  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,puVar7 | uVar5);
  if ((puVar7 | uVar5) != 0x0) {
    ppcVar2 = (code **)(uVar3 + 0x4);
    (**ppcVar2)(0x38,uVar4,puVar6,0x0,0x0);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(paVar8,uVar5));
    uVar1 = (param_3 + 0x10c);
    uVar10 = 0x1030;
    pass1_1030_7ddc(uVar1,paVar8,CONCAT22(paVar8,uVar5),CONCAT13((u8)(uVar1 >> 0xf),(int3)uVar1),
                    0x1f);
  }
  if (puStack14 != NULL) {
    ppcVar2 = (code **)*puStack14;
    (**ppcVar2)(uVar10,uVar4,(char)puVar6,0x1);
  }
  return;
}
pub fn pass1_1028_69cc(param_1: *mut astruct_317,param_2: *mut astruct_57,param_3: *mut astruct_316)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_316 *iVar5;
  let mut puVar5: *mut u32;
  let mut puVar6: *mut u32;
  let mut uVar7: u16;
  let mut puStack10: *mut u16;

  mem_op_1000_179c(0x10e,param_2);
  uVar4 = param_2;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar7 = (param_3 >> 0x10);
    iVar5 = (astruct_316 *)param_3;
    param_1->field3_0x4 = iVar5->field4_0x4;
    puVar5 = &iVar5->field5_0x8;
    puVar6 = &param_1->field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 0x1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    param_1->field257_0x108 = iVar5->field258_0x108;
    param_1->field258_0x10c = iVar5->field259_0x10c;
    *puStack10 = 0x6ae2;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_6a7a(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * pass1_1028_6aa6(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1028_6af2(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x1387);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  &iVar1->field259_0x108 = param_3;
  &iVar1->field262_0x10c = param_2;
  param_1->offset_0x0 = 0x6e50;
  iVar1->segment_0x2 = 0x1028;
  return;
}



u16 pass1_1028_6b2c(mut param_1: u32)

{
  let mut in_DX: u16;

  pass1_1028_6b40(in_DX,param_1);
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6b40(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut puVar2: *mut u8,
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  u8 local_36 [0xe];
  let mut puStack40: *mut u32;
  let mut uStack38: u16;
  let mut uStack36: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut uStack26: u16;
  let mut local_18: u32;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  astruct_15 *pstruct15_10;
  u8 local_6 [0x2];
  let mut local_4: i16;

  puVar2 = local_6;
  pass1_1028_6daa((astruct_15 *)CONCAT22(puVar2,param_1),param_2,CONCAT22(0x1050,puVar2),
                  CONCAT22(0x1050,&local_4),&DAT_1050_1050);
  uVar5 = (param_2 >> 0x10);
  uVar4 = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar4 + 0x10c));
  pstruct15_10 = (astruct_15 *)CONCAT22(param_1,puVar2);
  ppcVar1 = (code **)(pstruct15_10 + 0x24);
  (**ppcVar1)();
  uStack14 = pass1_1028_b58e(pstruct15_10);
  uStack18 = pass1_1028_bb24(pstruct15_10);
  local_18 = (uStack14 + 0xc);
  uStack20 = (uStack14 + 0x10);
  puStack40 = &local_18;
  uStack26 = local_18;
  uStack28 = (local_18 >> 0x10);
  uStack32 = local_18 - 0x1;
  if (uStack32 < 0x0) {
    uStack32 = 0x0;
  }
  uVar3 = local_4 - 0x1;
  uStack34 = local_18 + 0x1;
  if (uVar3 < (local_18 + 0x1)) {
    uStack34 = uVar3;
  }
  uStack36 = uStack28 - 0x1;
  if (uStack36 < 0x0) {
    uStack36 = 0x0;
  }
  uStack38 = uStack28 + 0x1;
  if (uVar3 < (uStack28 + 0x1)) {
    uStack38 = uVar3;
  }
  uStack30 = uStack20;
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack20,uStack36,uStack32);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack30,uStack36,uStack26);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack30,uStack36,uStack34);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack30,uStack28,uStack32);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack30,uStack28,uStack34);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack30,uStack38,uStack32);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack30,uStack38,uStack26);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54(CONCAT22(0x1050,local_36),uStack30,uStack38,uStack34);
  pass1_1028_6d24((puVar6 >> 0x10),uVar4,uVar5,CONCAT22(0x1050,local_36),uStack18);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6d24(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,i32 param_5)

{
  let mut iVar1: i16;
  code **ppcVar2;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  u8 bStack27;
  let mut local_a: u32;
  let mut uStack6: u32;

  puVar3 = &local_a;
  pass1_1030_64ce(puVar3,param_1,_PTR_LOOP_1050_5740,param_4,param_5,CONCAT22(0x1050,puVar3));
  uStack6 = *puVar3;
  uVar5 = (puVar3 + 0x2);
  bStack27 = (u8)(uStack6 >> 0x18);
  uVar4 = bStack27;
  if (bStack27 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | uVar5 << 0x10);
    puVar6 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar4),uVar4,uVar5);
    iVar1 = (puVar6 + 0xc);
    if (((iVar1 < 0x1) || (SBORROW2(iVar1,0x1))) ||
       ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 && ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73)))))) {
      ppcVar2 = (code **)(*puVar6 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6daa(param_1: *mut astruct_15,mut param_2: u32,param_3: *mut u16,param_4: *mut u16,mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut puVar2: *mut u32;
  let mut local_18: u32;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u16;
  let mut uStack4: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_2 + 0x10c));
  uStack10 = pass1_1028_b4f2((astruct_15 *)CONCAT22(param_1,(param_1 >> 0x10)));
  uStack6 = (param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  uStack16 = (uStack10 >> 0x10);
  uVar1 = (uStack10 + 0x8);
  uStack14 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  uStack6 = (param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  iStack18 = uVar1;
  puVar2 = pass1_1030_5b5c(iStack18,uStack16);
  uStack6 = (param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  local_18 = *puVar2;
  uStack20 = (puVar2 + 0x4);
  pass1_1008_3e94(CONCAT22(0x1050,&local_18),param_3,param_4);
  uStack6 = (param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  return;
}



StructD * pass1_1028_6e24(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_6e60(uchar param_1,param_2: *mut astruct_97)

{
  struct_op_1028_d1dc(param_2,0x32c7);
  param_2->offset_0x0 = 0x6fb0;
  (param_2 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((param_2 & 0xffff0000 | (param_2 + 0x8)),s_SCConstruct_1050_4fdc);
  return param_2;
}



u16 pass1_1028_6e96(mut param_1: u16 )

{
  astruct_92 *paVar1;
  code **ppcVar2;
  astruct_92 **ppaVar3;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut puStack24: *mut u32;
  astruct_92 *local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    ppaVar3 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,ppaVar3));
    puStack24 = CONCAT22(param_1,ppaVar3);
    uVar4 = param_1 | ppaVar3;
    if (uVar4 == 0x0) break;
    paVar1 = ppaVar3[0x9];
    param_1 = uVar4;
    if (((0x0 < paVar1) && (!SBORROW2(paVar1,0x1))) && ((&paVar1[-0x1].field6_0x10 + 0x1) < 0x4)) {
      ppcVar2 = (code **)(*puStack24 + 0x38);
      (**ppcVar2)();
      param_1 = extraout_DX;
    }
  }
  return 0x1;
}
pub fn pass1_1028_6ef6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (param_1 + 0x4) = (param_3 + 0x4);
    puVar3 = (param_3 + 0x8);
    puVar7 = (param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0x6fb0;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



StructD * pass1_1028_6f84(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_6fc0(uchar param_1,param_2: *mut astruct_97)

{
  struct_op_1028_d1dc(param_2,0x3e7);
  param_2->offset_0x0 = 0x749e;
  (param_2 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((param_2 & 0xffff0000 | (param_2 + 0x8)),s_SCEndSim_1050_4fea);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6ff6(StructD *param_1,mut param_2: u32,mut param_3: u16 )

{
  let mut bVar1: bool;
  let mut bVar2: bool;
  let mut bVar3: bool;
  astruct_92 *paVar4;
  astruct_92 *paVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  let mut unaff_DI: i16;
  astruct_27 *paVar10;
  astruct_67 *paVar11;
  astruct_102 *paVar12;
  astruct_679 *paVar13;
  let mut in_stack_0000fe52: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7c: u16;
  let mut in_stack_0000ff80: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff86: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut uVar14: u16;
  let mut uVar15: u16;
  let mut iVar16: i16;
  u8 uVar17;
  u8 uVar18;
  let mut uVar19: u16;
  let mut uVar20: u16;
  let mut uVar21: u16;
  let mut iVar22: i16;
  astruct_92 local_46;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_14)),0x1,0x0,0x400);
  bVar3 = true;
  bVar2 = false;
  do {
    do {
      paVar4 = &local_14;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
      uVar7 = param_1;
      param_1 = (StructD *)(param_1 & 0xffff0000 | (uVar7 | paVar4));
      if ((uVar7 | paVar4) == 0x0) goto LAB_1028_7066;
    } while (((&paVar4[0x1c].field3_0x4 + 0x2) == 0x0) || (paVar4[0x1c].field4_0x8 == 0x8000002));
    bVar2 = true;
    iVar16 = &paVar4[0x1b].field6_0x10;
    pass1_1030_38b8();
  } while ((param_1 < 0x0) || ((param_1 < 0x1 && (iVar16 == 0x0))));
  bVar3 = false;//
LAB_1028_7066:
  if (local_14.field6_0x10 == 0x0) {
    paVar8 = (astruct_57 *)(param_1 & 0xffff0000 | local_14.field5_0xc);
    local_14.field4_0x8 = local_14.field5_0xc;
  }
  else {
    paVar8 = (astruct_57 *)(param_1 & 0xffff0000);
    local_14.field4_0x8 = 0x1;
  }
  local_14.field4_0x8 = SUB42(paVar8,0x0);
  bVar1 = false;
  while( true ) {
    paVar4 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
    uVar7 = paVar8;
    uVar6 = uVar7 | paVar4;
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | uVar6);
    if (uVar6 == 0x0) break;
    if (paVar4[0x1c].field4_0x8 == 0x8000001) {
      bVar1 = true;
    }
  }
  if (!bVar1) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
    uVar7 = paVar8 | paVar4;
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | uVar7);
    if (uVar7 != 0x0) {
      PTR_LOOP_1050_4fe8 = (&PTR_LOOP_1050_0000 + 0x1);
      uVar20 = 0x0;
      iVar16 = 0x1;
      paVar10 = (astruct_27 *)
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x2b,in_stack_0000fe5c,in_stack_0000ff80,
                                in_stack_0000ff86,in_stack_0000ff8a);
      paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | paVar10 >> 0x10);
      paVar4 = (astruct_92 *)paVar10;
      pass1_1010_089e(paVar10,uVar20,iVar16);
      pass1_1010_089e(paVar10,0x0,0x2);
      pass1_1010_089e(paVar10,0x0,0x3);
      pass1_1010_089e(paVar10,0x0,0x4);
      pass1_1010_089e(paVar10,0x0,0x5);
      pass1_1010_089e(paVar10,0x0,0x7);
      pass1_1010_089e(paVar10,0x0,0x8);
      pass1_1010_089e(paVar10,0x0,0xa);
    }
  }
  if ((bVar2) && (bVar3)) {
    uVar21 = 0x0;
    iVar22 = 0x6;
    uVar17 = 0x1;
    uVar18 = 0x0;
    uVar19 = 0x0;
    uVar15 = 0x0;
    iVar16 = 0x0;
    uVar14 = 0x0;
    paVar11 = (astruct_67 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe52,in_stack_0000ff76,
                              in_stack_0000ff7c,in_stack_0000ff80);
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | paVar11 >> 0x10);
    paVar4 = (astruct_92 *)paVar11;
    post_win_msg_1008_a0e4
              (paVar11,CONCAT22(uVar15,uVar14),iVar16,CONCAT11(uVar18,uVar17),CONCAT22(uVar21,uVar19),iVar22);
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x8000001);
  uVar7 = paVar8;
  paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | (uVar7 | paVar4));
  paVar5 = paVar4;
  if ((((((uVar7 | paVar4) != 0x0) &&
        (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x4), paVar5 == NULL)) &&
       (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x2a), paVar5 == NULL)) &&
      ((paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x4b), paVar5 == NULL &&
       (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x54), paVar5 == NULL)))) &&
     ((paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x2c), paVar5 == NULL &&
      ((paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x3c), paVar5 == NULL &&
       (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x3d), paVar5 == NULL)))))) {
    if (local_14.field6_0x10 == 0x0) {
      paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | local_14.field5_0xc);
    }
    else {
      local_14.field5_0xc = 0x1;
      paVar8 = (astruct_57 *)(paVar8 & 0xffff0000);
    }
    local_14.field4_0x8 = SUB42(paVar8,0x0);
    bVar2 = false;
    bVar3 = false;
    local_14.field4_0x8 = local_14.field5_0xc;
    do {
      do {
        paVar5 = &local_14;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar5));
        uVar7 = paVar8;
        paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | (uVar7 | paVar5));
        if ((uVar7 | paVar5) == 0x0) goto LAB_1028_72d3;
      } while (paVar5[0x1c].field4_0x8 == 0x8000002);
      uVar20 = (param_2 >> 0x10);
      paVar4 = paVar5;
      if ((!bVar2) && (pass1_1028_740c(param_2,uVar20,0x22,CONCAT22(uVar7,paVar5)), paVar4 != NULL)) {
        bVar2 = true;
      }
      if ((!bVar3) && (pass1_1028_740c(param_2,uVar20,0x24,CONCAT22(uVar7,paVar5)), paVar4 != NULL)) {
        bVar3 = true;
      }
    } while ((!bVar2) || (!bVar3));
    uVar21 = 0x0;
    iVar22 = 0x14;
    uVar17 = 0x1;
    uVar18 = 0x0;
    uVar19 = 0x0;
    uVar15 = 0x0;
    iVar16 = 0x0;
    uVar14 = 0x0;
    paVar11 = (astruct_67 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe52,in_stack_0000ff76,
                              in_stack_0000ff7c,in_stack_0000ff80);
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | paVar11 >> 0x10);
    paVar5 = (astruct_92 *)paVar11;
    post_win_msg_1008_a0e4
              (paVar11,CONCAT22(uVar15,uVar14),iVar16,CONCAT11(uVar18,uVar17),CONCAT22(uVar21,uVar19),iVar22);
  }//
LAB_1028_72d3:
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
  uVar7 = paVar8 | paVar5;
  paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | uVar7);
  if (uVar7 != 0x0) {
    paVar12 = (astruct_102 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3b),in_stack_0000fe60,
                              in_stack_0000ff84,in_stack_0000ff8a,in_stack_0000ff8e);
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | paVar12 >> 0x10);
    pass1_1008_df4a(paVar12,unaff_DI,&DAT_1050_1050);
    paVar13 = (astruct_679 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3c),in_stack_0000fe60,
                              in_stack_0000ff84,in_stack_0000ff8a,in_stack_0000ff8e);
    uVar9 = paVar13 >> 0x10;
    pass1_1018_34a6(paVar13);
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_46)),0x1,0x0,0x400);
    while( true ) {
      uVar7 = uVar9;
      paVar4 = &local_46;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
      uVar9 = (uVar7 | paVar4);
      if ((uVar7 | paVar4) == 0x0) break;
      if (paVar4[0x1c].field4_0x8 != 0x8000002) {
        pass1_1038_3ba0((astruct_428 *)CONCAT22(uVar7,paVar4));
      }
    }
  }
  return;
}
