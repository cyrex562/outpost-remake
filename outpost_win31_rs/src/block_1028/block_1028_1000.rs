
pub unsafe fn pass1_1028_1024(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15) -> i16

{
  let mut BVar1: bool;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut local_16: u32;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  uStack8 = pass1_1030_2fac(CONCAT22(param_2,param_1));
  uStack12 = pass1_1028_bb24(param_3);
  uVar6 = pass1_1028_b58e(param_3);
  uStack14 = (uVar6 >> 0x10);
  local_16 = (uVar6 + 0xc);
  iStack26 = 0;
  iStack24 = 0;
  while( true ) {
    if (uStack8 < iStack26) {
      return iStack24;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(puVar2,(uVar6 >> 0x10),_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),uStack12);
    uStack16 = uVar6;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6 & 0xffff0000 | ZEXT24(puVar2));
    uStack16 = uVar6;
    uVar3 = (uVar6 >> 0x10) | puVar2;
    if (uVar3 == 0) break;
    uVar6 = struct_op_1030_73a8((uVar6 & 0xffff0000 | ZEXT24(puVar2)),puVar2,uVar3);
    uVar4 = (uVar6 >> 0x10);
    uVar3 = uVar6;
    uVar5 = uVar4 | uVar3;
    if (uVar6 == 0) {
      return iStack24;
    }
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0,(uVar3 + 0xc),0x13);
    uVar6 = CONCAT22(uVar5,uStack16);
    if ((BVar1 != 0) && ((uVar3 + 0x12) == 0x5)) {
      iStack24 += 0x1;
    }
    iStack26 += 0x1;
  }
  return iStack24;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_1106(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15) -> i16

{
  let mut BVar1: bool;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut local_16: u32;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  uStack8 = pass1_1030_2fac(CONCAT22(param_2,param_1));
  uStack12 = pass1_1028_bb24(param_3);
  uVar5 = pass1_1028_b58e(param_3);
  uStack14 = (uVar5 >> 0x10);
  local_16 = (uVar5 + 0xc);
  iStack26 = 0;
  iStack24 = 0;
  while( true ) {
    if (uStack8 < iStack26) {
      return iStack24;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(puVar2,(uVar5 >> 0x10),_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),uStack12);
    uStack16 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff0000 | ZEXT24(puVar2));
    uStack16 = uVar5;
    uVar3 = (uVar5 >> 0x10) | puVar2;
    if (uVar3 == 0) break;
    uVar5 = struct_op_1030_73a8((uVar5 & 0xffff0000 | ZEXT24(puVar2)),puVar2,uVar3);
    uVar4 = (uVar5 >> 0x10);
    uVar3 = uVar4 | uVar5;
    if (uVar5 == 0) {
      return iStack24;
    }
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0,(uVar5 + 0xc),0x13);
    uVar5 = CONCAT22(uVar3,uStack16);
    if (BVar1 != 0) {
      iStack24 += 0x1;
    }
    iStack26 += 0x1;
  }
  return iStack24;
}



bool pass1_1028_11de(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_b58e(param_1);
  return (uVar1 + 0x10) == 0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_15 * pass1_1028_121e(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut bVar1: bool;
  let mut extraout_AH: u8;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u32;
  paVar5: *mut astruct_15;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  bVar1 = pass1_1028_11de(param_2);
  if (CONCAT11(extraout_AH,bVar1) != 0) {
    return param_2;
  }
  uStack6 = pass1_1028_b58e(param_2);
  local_c = (uStack6 + 0xc);
  uStack8 = 0;
  uVar4 = pass1_1028_bb24(param_2);
  uVar3 = (uVar4 >> 0x10);
  puVar2 = &local_c;
  pass1_1030_627e(puVar2,uVar3,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                  uVar4 & 0xffff | uVar3 << 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar3,puVar2));
  if ((uVar3 | puVar2) == 0) {
    return NULL;
  }
  paVar5 = struct_op_1030_73a8(CONCAT22(uVar3,puVar2),puVar2,uVar3 | puVar2);
  return paVar5;
}



pub unsafe fn pass1_1028_12be(param_1: *mut astruct_15,param_2: *mut u32) -> u16

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut bVar4: bool;
  let mut extraout_AH: u8;
  let mut uVar5: u16;
  paVar6: *mut astruct_15;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut uStack8: u16;

  bVar4 = pass1_1028_11de(param_1);
  if (CONCAT11(extraout_AH,bVar4) == 0) {
    paVar6 = pass1_1028_121e(&DAT_1050_1050,param_1);
    ppcVar3 = (paVar6 + 0x40);
    uVar5 = (**ppcVar3)();
    return uVar5;
  }
  *param_2 = 0;
  uVar7 = pass1_1028_b58e(param_1);
  uStack8 = 0x4;
  uVar8 = uVar7;
  loop {
    uVar8 = pass1_1030_7c28(uVar8,(uVar8 >> 0x10),uVar7,uStack8);
    uVar2 = param_2;
    param_2 = param_2 + uVar8;
    piVar1 = (param_2 + 2);
    *piVar1 = *piVar1 + (uVar8 >> 0x10) + CARRY2(uVar2,uVar8);
    uStack8 += 0x1;
  } while (uStack8 < 0xe);
  if (0x1f4 < *param_2) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_134a(param_1: *mut astruct_15)

{
  let mut piVar1: *mut i16;
  let mut ppcVar2: *mut *mut code;
  let mut BVar3: bool;
  i32 *plVar4;
  let mut uVar5: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u16;
  paVar8: *mut astruct_15;
  let mut lStack26: i32;
  let mut iStack22: i16;
  let mut uStack18: u32;
  let mut uStack10: u32;
  let mut local_6: i32;

  uVar7 = (param_1 >> 0x10);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,(param_1 + 0xc),0x13);
  if (BVar3 != 0) {
    plVar4 = &local_6;
    ppcVar2 = (param_1 + 0x40);
    (**ppcVar2)(0x1008,param_1,plVar4,&DAT_1050_1050);
    if (plVar4.is_null() == false) {
      piVar1 = (param_1 + 0x22);
      *piVar1 = *piVar1 + 1;
      return;
    }
    uStack10 = 0x1f4 - local_6;
    paVar8 = pass1_1028_121e(&DAT_1050_1050,param_1);
    uVar5 = (paVar8 >> 0x10);
    uVar7 = SUB42(paVar8,0x0);
    pass1_1028_b58e(paVar8);
    uStack18 = CONCAT22(uVar5,uVar7);
    for (iStack22 = 0; iStack22 < 0xa; iStack22 += 1) {
      uStack10 = (iStack22 * 0x2 + 0x4fbe);
      paVar6 = (uStack10 >> 0x10);
      if (uStack10 < uStack10) {
        paVar6 = (uStack10 >> 0x10);
      }
      lStack26 = CONCAT22(paVar6,uStack10);
      pass1_1030_7ddc(uStack10,paVar6,uStack18,
                      CONCAT13((paVar6 >> 0x8),CONCAT12(paVar6,uStack10)),iStack22 + 0x4);
      uStack10 -= lStack26;
      if (uStack10 < 1) {
        return;
      }
    }
  }
  return;
}



pub unsafe fn pass1_1028_1416(mut param_1: u16 ,mut param_2: u32) -> i16

{
  let mut bVar1: bool;
  let mut extraout_AH: u8;
  let mut iVar2: i16;
  let mut uVar3: u16;
  paVar4: *mut astruct_15;

  bVar1 = pass1_1028_11de(param_2);
  if (CONCAT11(extraout_AH,bVar1) == 0) {
    paVar4 = pass1_1028_121e(&DAT_1050_1050,param_2);
    uVar3 = (paVar4 >> 0x10);
    iVar2 = pass1_1028_1416(uVar3,paVar4 & 0xffff | uVar3 << 0x10);
    return iVar2;
  }
  iVar2 = pass1_1028_1024(CONCAT11(extraout_AH,bVar1),param_1,param_2);
  return iVar2 * 0xf;
}



pub unsafe fn write_to_file_1028_1452(param_1: *mut astruct_731,param_2: *mut u8) -> u16

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  in_stack_0000ffda: mut HFILE16;
  u16 local_c [0x3];
  local_6: *mut u8 [0x2];

  BVar1 = write_to_file_1028_b5ec(param_1,param_2);
  if (BVar1 != 0) {
    uVar2 = (param_1 >> 0x10);
    local_c[0] = (param_1 + 0x22);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffda);
    if (BVar1 != 0) {
      local_6[0] = (param_1 + 0x20);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffda);
      if (BVar1 != 0) {
        local_6[0] = PTR_LOOP_1050_4fbc;
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffda);
        if (BVar1 != 0) {
          return 0x1;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}
pub fn pass1_1028_14d8(mut param_1: i16,param_2: *mut u8,param_3: *mut astruct_373,HFILE16 *param_4)

{
  let mut BVar1: bool;
  let mut local_4: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0) {
    BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (param_3 + 0x22)),0x2);
    if ((BVar1 != 0) && (BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2), BVar1 != 0)) {
      (param_3 + 0x20) = local_4;
      if (u16_1050_0312 < 0x2) {
        return;
      }
      BVar1 = read_file_1008_7dee(param_4,&PTR_LOOP_1050_4fbc,0x2);
      if (BVar1 != 0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_1556(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15) -> u16

{
  let mut iVar1: i16;
  let mut puVar2: *mut u32;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iStack26: i16;
  let mut local_16: u32;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  uStack8 = pass1_1030_2fac(CONCAT22(param_2,param_1));
  uStack12 = pass1_1028_bb24(param_3);
  uVar7 = pass1_1028_b58e(param_3);
  uStack14 = (uVar7 >> 0x10);
  local_16 = (uVar7 + 0xc);
  iStack26 = 0x1;
  while( true ) {
    if (uStack8 < iStack26) {
      return 0x0;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(puVar2,(uVar7 >> 0x10),_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),uStack12);
    uStack16 = uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff0000 | ZEXT24(puVar2));
    uStack16 = uVar7;
    uVar4 = (uVar7 >> 0x10) | puVar2;
    if (uVar4 == 0) {
      return 0x0;
    }
    uVar7 = struct_op_1030_73a8((uVar7 & 0xffff0000 | ZEXT24(puVar2)),puVar2,uVar4);
    uVar5 = (uVar7 >> 0x10);
    uVar4 = uVar7;
    uVar6 = uVar5 | uVar4;
    if (uVar7 == 0) {
      return 0x0;
    }
    iVar1 = (uVar4 + 0xc);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0,iVar1,0x13);
    uVar7 = CONCAT22(uVar6,uStack16);
    if ((BVar3 == 0) && (iVar1 != 0x75)) break;
    if ((uVar4 + 0x12) != 0x9) {
      return 0x1;
    }
    iStack26 += 0x1;
  }
  return 0x0;
}



astruct_409 * pass1_1028_1646(param_1: *mut astruct_409)

{
  paVar1: *mut astruct_409;
  uVar2: *mut astruct_409;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  uVar2 = param_1;
  paVar1 = (uVar2.field32_0x20 + -0x4);
  if (paVar1 < (&u16_1050_0008 + 1)) {
    switch(paVar1) {
    case NULL:
      uVar2.field32_0x20 = 0x5;
      break;
    case 0x1:
      uVar2.field32_0x20 = 0x6;
      break;
    case 0x2:
      uVar2.field32_0x20 = 0x7;
      break;
    case 0x3:
      uVar2.field32_0x20 = 0x8;
      break;
    case 0x4:
      uVar2.field32_0x20 = 0x9;
      break;
    case 0x5:
      uVar2.field32_0x20 = 0xa;
      return uVar2;
    case 0x6:
      uVar2.field32_0x20 = 0xb;
      return uVar2;
    case 0x7:
      uVar2.field32_0x20 = 0xc;
      return uVar2;
    case 0x8:
      uVar2.field32_0x20 = 0xd;
      return uVar2;
    }
    return uVar2;
  }
  uVar2.field32_0x20 = 0x4;
  return paVar1;
}



pub fn pass1_1028_16fe(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_178c(param_1: *mut u16)

{
  struct_1030_dc96(param_1);
  *param_1 = 0x1b54;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_17ae(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  pass1_1030_dcc2(param_1,CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0x1b54;
  (param_2 + 0x2) = 0x1028;
  return CONCAT22(param_3,param_2);
}
pub fn pass1_1028_17d8(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut extraout_DX: u16;

  pass1_1030_df0c(param_1,param_2);
  pass1_1028_b58e(param_2);
  pass1_1038_57dc((param_1 + 0x2e),0x1,0x3);
  return;
}
pub fn pass1_1028_1812(param_1: u32)

{
  pass1_1028_bdac(param_1,0x2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_1824(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,param_4: *mut u32,mut param_5: u32,mut param_6: u32)

{
  let mut BVar1: bool;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut unaff_SI: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9a: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut local_2a: u32;
  let mut iStack38: i16;
  let mut iStack36: i16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut iStack24: i16;
  let mut puStack22: *mut u32;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut uStack6: u32;

  uVar8 = param_3;
  uVar9 = (param_3 >> 0x10);
  pass1_1028_c3aa(uVar8,uVar9,param_4,param_5,param_6);
  if (param_1 == 0) {
    return;
  }
  BVar1 = pass1_1028_c314(param_1,param_2,uVar8,uVar9,param_4,param_5,(param_5 >> 0x10),
                          param_6);
  if (BVar1 == 0) {
    return;
  }
  puVar2 = &local_c;
  pass1_1030_64ce(puVar2,param_2,_PTR_LOOP_1050_5740,param_4,param_6,CONCAT22(0x1050,puVar2));
  uStack6 = *puVar2;
  uStack30 = (puVar2 + 2);
  paVar5 = CONCAT22(in_register_0000000a,uStack30);
  uVar6 = (param_4 >> 0x10);
  iStack8 = (param_4 + 0x4);
  puStack22 = (uStack6 & 0xffff | uStack30 << 0x10);
  uStack32 = uStack6;
  uVar3 = uStack30 >> 0x8;
  if ((uStack30 >> 0x8) != '\0') {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | uStack30 << 0x10);
    uStack30 = paVar5;
    uStack32 = uVar3;
    uStack28 = pass1_1030_6fa0(CONCAT22(uStack30,uVar3));
    if (uStack28 == 0x10) {
      if (iStack8 != 0) {
        PTR_LOOP_1050_50ca = 0x6ab;
        return;
      }
      return;
    }
    if ((uStack28 == 0x60) || (uStack28 == 0x61)) {
      puStack22 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2f),in_stack_0000fe6c,
                                  in_stack_0000ff90,in_stack_0000ff96,in_stack_0000ff9a);
      uVar7 = pass1_1018_04b8(puStack22);
      uStack34 = (uVar7 >> 0x10);
      uStack16 = uVar7;
      iStack36 = (puStack22 + 0x1e);
      iStack24 = iStack36;
      uStack14 = uStack34;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
      uVar4 = pass1_1030_2fac(CONCAT22(uStack34,iStack36));
      if (uVar4 <= iStack24) {
        PTR_LOOP_1050_50ca = 0x6ac;
        return;
      }
      local_2a = *param_4;
      iStack38 = iStack8 + 1;
      puVar2 = &local_2a;
      pass1_1028_c7b6(uVar6,uVar8,uVar9,CONCAT22(0x1050,puVar2),param_6);
      if (puVar2.is_null()) {
        return;
      }
      if (puVar2 == (&u32_1050_0004 + 0x2)) {
        return;
      }
      return;
    }
  }
  PTR_LOOP_1050_50ca = 0x6aa;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_199a(mut param_1: i16,param_2: *mut astruct_15)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  let mut paVar3: *mut Struct57;
  let mut unaff_SI: u16;
  let mut in_stack_0000fd42: u16;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe70: u16;
  let mut piVar4: *mut i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u16;
  let mut local_15e: u16;
  let mut uStack348: u16;
  let mut puStack50: *mut u32;
  let mut uStack42: u32;
  let mut uStack38: u16;
  let mut piStack36: *mut i16;
  let mut local_22: i16;
  let mut local_20: u16;
  let mut uStack30: u32;
  let mut puStack26: *mut u32;
  let mut local_16: i16;
  let mut local_14: u32;
  let mut local_10: u32;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  piVar1 = (param_2 + 0x14);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0) {
    pass1_1028_b58e(param_2);
    uStack4 = in_EDX;
    uStack10 = (param_1 + 0x2e);
    iStack6 = param_1;
    pass1_1038_5804(uStack10,0x1,0x3);
    paVar3 = (in_EDX & 0xffff0000 | uStack4);
    local_10 = (iStack6 + 0xc);
    uStack12 = (iStack6 + 0x10);
    puStack50 = &local_10;
    pass1_1008_3eb4(CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_16),
                    CONCAT22(0x1050,&local_14),CONCAT22(0x1050,&local_14 + 0x2));
    puStack26 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2f),in_stack_0000fd42,
                                in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
    uVar2 = (puStack26 + 0x20);
    puVar6 = &local_20;
    uVar7 = SUB42(&DAT_1050_1050,0x0);
    piStack36 = &local_22;
    uVar5 = SUB42(&DAT_1050_1050,0x0);
    piVar4 = piStack36;
    uStack30 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
    uStack38 = uVar2;
    pass1_1030_5b1c(uVar2 & 0xffff | ZEXT24(piStack36) << 0x10,CONCAT22(uVar5,piVar4),
                    CONCAT22(uVar7,puVar6));
    if (local_22 < local_16 + 1) {
      pass1_1030_5b3e(CONCAT22(piStack36,uStack38),local_16 + 0x1,local_20);
      pass1_1030_5b1c(CONCAT22(piStack36,uStack38),CONCAT22(0x1050,&local_22),
                      CONCAT22(0x1050,&local_20));
    }
    uVar5 = (uStack10 >> 0x10);
    uStack42 = (uStack10 + 0x4);
    struct_op_1028_87f0(CONCAT22(0x1050,&local_15e),0x0,0x0,(-(local_16 == 0) & 0xffd3) + 0x60,
                        &local_10,&DAT_1050_1050,
                        uStack42 & 0xffff | (uStack10 + 0x6) << 0x10,uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_15e));
    local_15e = 0x389a;
    uStack348 = 0x1008;
    pass1_1008_3e76(CONCAT22(0x1050,&local_10),local_16 + 0x1,local_14,(local_14 >> 0x10));
    struct_op_1028_87f0(CONCAT22(0x1050,&local_15e),0x0,0x0,0x60,&local_10,&DAT_1050_1050,uStack42
                        ,uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_15e));
  }
  return;
}
pub fn pass1_1028_1b1e(mut param_1: u32)

{
  (param_1 + 0x14) = 0x7;
  return;
}



pub fn pass1_1028_1b2e(mut param_1: u16 ,param_2: *mut StructD,param_3: u8) -> *mut StructD

{
  pass1_1030_dcf4(param_1,param_2);
  if ((param_3 & 1) != 0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



u16 * struct_1028_1bbc(param_1: *mut astruct_180)

{
  iVar1: *mut astruct_180;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = 0;
  param_1.field0_0x0 = 0x1eee;
  iVar1.field1_0x2 = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_1be8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0;
  (param_2 + 0x22) = 0;
  param_2.field0_0x0 = 0x1eee;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



pub unsafe fn pass1_1028_1c1c() -> u16

{
  return 0x0;
}



pub unsafe fn pass1_1028_1c22(mut param_1: u32) -> u16

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0x20) != 0) && (((iVar2 + 0x12) == 0x5 || ((iVar2 + 0x12) == 0x6)))) {
    if ((iVar2 + 0xc) == 0x16) {
      return 0x19;
    }
    if ((iVar2 + 0xc) == 0x17) {
      return 0x1a;
    }
  }
  uVar1 = pass1_1028_bc1c(param_1 & 0xffff | uVar3 << 0x10);
  return uVar1;
}
pub fn pass1_1028_1c66(param_1: *mut astruct_15)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u32;

  if ((param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if ((uVar3 + 0x200) != 0x8000002) {
    ppcVar1 = (param_1 + 0x64);
    iVar2 = (**ppcVar1)();
    if (iVar2 == 0) {
      return;
    }
    pass1_1028_cb04(param_1);
    if (iVar2 == 0) {
      iVar2 = 0x6;
  // TODO: goto LAB_1028_1cbd;
    }
    pass1_1028_c952(param_1);
  }
  iVar2 = 0x5;//
// LAB_1028_1cbd:
  pass1_1028_bdac(param_1,iVar2);
  return;
}



// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1028_1cca(mut param_1: u32,param_2: *mut u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,param_6: i32) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut local_e: [u8;0x2] = [0;0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 1);
  pass1_1008_3eb4(CONCAT22(0x1050,&local_8),CONCAT22(0x1050,local_e),
                  CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (local_c - 1) << 0x10;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x16,CONCAT22(0x1050,&local_8),param_6);
  if (uVar1 == 0) {
    local_8 = local_8 & 0xffff | (local_c + 1) << 0x10;
    uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x16,CONCAT22(0x1050,&local_8),param_6);
    if (uVar1 == 0) {
      local_8 = local_a + -0x1;
      local_8 = local_c;
      uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x17,CONCAT22(0x1050,&local_8),param_6);
      if (uVar1 == 0) {
        local_8 = CONCAT22(local_8,local_a + 1);
        uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x17,CONCAT22(0x1050,&local_8),param_6);
        if (uVar1 == 0) {
          return uVar1;
        }
      }
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_1da4(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,param_5: i32)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u8;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut local_4: [u8;0x2] = [0;0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | uVar4 << 0x10,param_3,param_5);
  if (puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = 0x6af;
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_1e14(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16,param_7: i32) -> u16

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_1),uVar2 | param_1,uVar2);
      if (uVar3 != 0) {
        iVar1 = (uVar3 + 0xc);
        if (((iVar1 == 0x18) || (iVar1 == 0x3f)) || (iVar1 == param_5)) {
          return 0x1;
        }
      }
    }
  }
  return 0x0;
}



pub unsafe fn pass1_1028_1e8a(param_1: *mut astruct_15) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar3: u16;

  uVar1 = (param_1 >> 0x10);
  if ((*(param_1 + 0x1a) & 0x2) == 0) {
    uVar4 = 0;
    uVar5 = 0x23;
    uVar3 = 0x1;
    uVar2 = pass1_1028_b58e((param_1 & 0xffff | uVar1 << 0x10));
    pass1_1030_7d7c(uVar2,(uVar2 >> 0x10),uVar2,uVar3,CONCAT22(uVar5,uVar4));
    pass1_1028_bdac(param_1,0x6);
    return 0x0;
  }
  return 0x1;
}



pub fn pass1_1028_1ec8(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar1
pub fn struct_1028_1f56(param_1: *mut astruct_57,param_2: *mut astruct_180)

{
  let mut extraout_DX: u16;
  let mut uVar2: u16;
  iVar3: *mut astruct_180;
  let mut uVar3: u16;
  let mut uVar1: u32;

  struct_1028_b354(param_2);
  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  uVar2 = 0;
  (iVar3 + 1) = 0;
  &iVar3[0x1].field_0x4 = 0;
  param_2.field0_0x0 = 0x2572;
  iVar3.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xc,param_1);
  extraout_DX = param_1 | uVar2;
  if (extraout_DX == 0) {
    (iVar3 + 1) = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(param_1,uVar2));
    (iVar3 + 1).field0_0x0 = uVar2;
    iVar3[0x1].field1_0x2 = extraout_DX;
  }
  uVar1 = (iVar3 + 1);
  (uVar1 + 0xa) = 0;
  return;
}
pub fn pass1_1028_1fc8(param_1: *mut u8,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  uVar2 = 0;
  (param_2 + 0x20) = 0;
  (param_2 + 0x24) = 0;
  param_2.field0_0x0 = 0x2572;
  (param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xc,paVar4);
  uVar3 = paVar4 | uVar2;
  if (uVar3 == 0) {
    (param_2 + 0x20) = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(paVar4,uVar2));
    (param_2 + 0x20) = uVar2;
    (param_2 + 0x22) = uVar3;
  }
  uVar1 = (param_2 + 0x20);
  (uVar1 + 0xa) = 0;
  return;
}
