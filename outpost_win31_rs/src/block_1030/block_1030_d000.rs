
pub fn pass1_1030_d00c(param_1: *mut astruct_15,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut extraout_DX: u16;
  local_BX_40: *mut astruct_696;
  let mut iVar2: i16;
  let mut iStack4: i16;

  iStack4 = 0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    iVar2 = iStack4 * 0xc + param_1;
    if (((iVar2 + 0x26) == 0) && (uVar1 = param_2, (iVar2 + 0x24) == param_2)) break;
    iStack4 += 0x1;
  }
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_4900((uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c((uVar1 & 0xffff | extraout_DX << 0x10),0x1,param_2);
  }
  local_BX_40 = (iStack4 * 0xc + param_1);
  local_BX_40.field32_0x20 = 0;
  local_BX_40.field33_0x24 = 0;
  local_BX_40.field34_0x26 = 0;
  return;
}



pub unsafe fn pass1_1030_d0a8(mut param_1: u32) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x98);
  pass1_1030_d56a(param_1 & 0xffff | uVar2 << 0x10);
  return uVar1;
}



// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1030_d0c6(mut param_1: u32) -> i16

{
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (uStack6 + 1)) {
    if ((param_1 + uStack6 * 0xc + 0x20) != 0) {
      uStack6 = uStack6 & 0xffff | (uStack6 + 1) << 0x10;
    }
  }
  return uStack6;
}



// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1030_d102(mut param_1: i16,mut param_2: u16 ) -> i16

{
  let mut iVar1: i16;
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (uStack6 + 1)) {
    iVar1 = uStack6 * 0xc + param_1;
    if (((iVar1 + 0x20) != 0) && ((iVar1 + 0x26) != 0)) {
      uStack6 = uStack6 & 0xffff | (uStack6 + 1) << 0x10;
    }
  }
  return uStack6;
}



// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1030_d144(mut param_1: u32) -> i16

{
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (uStack6 + 1)) {
    if ((param_1 + uStack6 * 0xc + 0x20) == 0) {
      uStack6 = uStack6 & 0xffff | (uStack6 + 1) << 0x10;
    }
  }
  return uStack6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_d180(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iStack4: i16;
  let mut paVar2: *mut Struct57;

  iStack4 = 0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    uVar5 = (param_1 >> 0x10);
    uVar3 = param_1;
    if (((uVar3 + iStack4 * 0xc + 0x22) | (uVar3 + iStack4 * 0xc + 0x20)) == 0) break;
    iStack4 += 0x1;
  }
  uVar1 = (_PTR_LOOP_1050_65e2 + 0x2) + (0xff37 < *_PTR_LOOP_1050_65e2);
  paVar2 = (in_EDX & 0xffff0000 | uVar1);
  iVar4 = iStack4 * 0xc + uVar3;
  (iVar4 + 0x20) = *_PTR_LOOP_1050_65e2 + 0xc8;
  (iVar4 + 0x22) = uVar1;
  (iVar4 + 0x24) = param_2;
  uVar1 = param_2;
  pass1_1030_d340(uVar3,uVar5,param_1 & 0xffff0000 | (iVar4 + 0x20));
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_48e0((uVar1 + 0x2e),1);
    return;
  }
  pass1_1030_7c50(uVar1,paVar2,CONCAT22(paVar2,uVar1),0x1,param_2);
  return;
}



pub unsafe fn pass1_1030_d230(mut param_1: u32) -> u16

{
  let mut iStack4: i16;

  iStack4 = 0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x1;
    }
    if ((param_1 + iStack4 * 0xc + 0x20) == 0) break;
    iStack4 += 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_d26c(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut iVar5: i16;
  let mut iStack8: i16;

  uVar2 = *_PTR_LOOP_1050_65e2;
  for (iStack8 = 0; iStack8 < 0xa; iStack8 += 1) {
    iVar5 = iStack8 * 0xc + param_1;
    if ((((iVar5 + 0x22) | (iVar5 + 0x20)) != 0) &&
       (puVar1 = (iVar5 + 0x20), *puVar1 < uVar2 || *puVar1 == uVar2)) {
      uVar4 = uVar2;
      pass1_1030_d3b2(uVar2,param_1,param_1,iStack8);
      iVar3 = uVar4;
      if (iVar3 == 0) {
        pass1_1028_b58e(param_1);
        if ((iVar5 + 0x24) == 0x5) {
          pass1_1038_4900((iVar3 + 0x2e));
        }
        else {
          pass1_1030_6e9c(CONCAT22(extraout_DX,iVar3),0x1,(param_1 + iStack8 * 0xc + 0x24));
        }
        iVar5 = iStack8 * 0xc + param_1;
        (iVar5 + 0x20) = 0;
        (iVar5 + 0x24) = 0;
        (iVar5 + 0x26) = 0;
      }
    }
  }
  return;
}
pub fn pass1_1030_d340(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_3 >> 0x10);
  iVar2 = param_3;
  iVar1 = (iVar2 + 0x4);
  if (((0x0 < iVar1) && (!SBORROW2(iVar1,1))) && ((iVar1 == 0x4 || iVar1 + -0x1 < 0x3 || (iVar1 == 0xc)))) {
    (iVar2 + 0x6) = 0;
    return;
  }
  (iVar2 + 0x6) = 0x1;
  return;
}



pub unsafe fn pass1_1030_d36e(mut param_1: u32,mut param_2: i16) -> u16

{
  let mut iStack4: i16;

  iStack4 = 0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if ((iStack4 != param_2) && ((param_1 + iStack4 * 0xc + 0x24) == 0x8)) break;
    iStack4 += 0x1;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_d3b2(mut param_1: i16,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  let mut iVar1: i16;
  paVar2: *mut astruct_691;
  let mut ppcVar3: *mut *mut code;
  let mut bVar4: bool;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut extraout_DX: u16;
  let mut puVar7: *mut u8;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut puVar10: *mut u32;
  let mut uVar11: u32;
  let mut puStack26: *mut u32;
  let mut uStack18: u32;
  let mut uStack14: u32;

  pass1_1028_b58e(CONCAT22(param_3,param_2));
  paVar2 = *(astruct_691 **)(param_1 + 0x2e);
  uVar5 = pass1_1030_d36e(CONCAT22(param_3,param_2),param_4);
  if (uVar5 == 0) {
    puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
    puVar7 = (puVar10 >> 0x10);
    uVar6 = puVar10;
    pass1_1038_4d6e(uVar6,puVar7,paVar2,puVar10);
    puStack26 = CONCAT22(puVar7,uVar6);
    ppcVar3 = (*puStack26 + 0x10);
    uVar8 = uVar6;
    (**ppcVar3)(&u16_1050_1038,uVar6,puVar7);
    uStack18 = CONCAT22(extraout_DX_00,uVar8);
    bVar4 = false;
    for (uStack14 = 0; uStack14 < uStack18; uStack14 += 1) {
      uVar11 = pass1_1030_1d7c(uStack14,uStack14,puStack26);
      uVar8 = (uVar11 >> 0x10);
      if ((((uVar8 | uVar11) != 0) && ((uVar11 + 0x4) != (param_2 + 0x4))) &&
         (uVar5 = pass1_1030_cf3a(uVar11,0x8), uVar5 != 0)) {
        bVar4 = true;
        break;
      }
    }
    if (puStack26.is_null() == false) {
      ppcVar3 = *puStack26;
      (**ppcVar3)(0x38,uVar6,puVar7,1);
    }
    if (!bVar4) {
      return;
    }
  }
  puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar7 = (puVar10 >> 0x10);
  uVar6 = puVar10;
  uVar9 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4d6e(uVar6,puVar7,paVar2,puVar10);
  puStack26 = CONCAT22(puVar7,uVar6);
  ppcVar3 = (*puStack26 + 0x10);
  uVar8 = uVar6;
  (**ppcVar3)(&u16_1050_1038,uVar6,puVar7);
  uStack18 = CONCAT22(extraout_DX_01,uVar8);
  bVar4 = false;
  uStack14 = 0;
  loop {
    if (uStack18 <= uStack14) {//
// LAB_1030_d51b:
      if (puStack26.is_null() == false) {
        ppcVar3 = *puStack26;
        (**ppcVar3)(uVar9,uVar6,puVar7,1);
      }
      if (!bVar4) {
        return;
      }
      uVar6 = *_PTR_LOOP_1050_65e2;
      iVar1 = (_PTR_LOOP_1050_65e2 + 2);
      (param_2 + param_4 * 0xc + 0x20) = uVar6 + 0xc8;
      (param_2 + param_4 * 0xc + 0x22) = iVar1 + (0xff37 < uVar6);
      return;
    }
    uVar11 = pass1_1030_1d7c(uStack14,uStack14,puStack26);
    uVar8 = (uVar11 >> 0x10) | uVar11;
    if (uVar8 != 0) {
      uVar9 = 0x1028;
      uVar5 = pass1_1028_6744(uVar11,0x7);
      if ((uVar8 | uVar5) != 0) {
        uVar9 = 0x1028;
        pass1_1028_6228(uVar11,0x1,0x0,0x7);
        bVar4 = true;
    // TODO: goto LAB_1030_d51b;
      }
    }
    uStack14 += 0x1;
  } while( true );
}



pub unsafe fn pass1_1030_d56a(mut param_1: u32) -> i16

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  switch((iVar1 + 0x98) + -1) {
  case 0x0:
    (iVar1 + 0x98) = 0x2;
    break;
  case 0x1:
    (iVar1 + 0x98) = 0x3;
    break;
  case 0x2:
    (iVar1 + 0x98) = 0x4;
    break;
  case 0x3:
    (iVar1 + 0x98) = 0xc;
    break;
  default:
    (iVar1 + 0x98) = 0x1;
    return iVar1;
  case 0x7:
    (iVar1 + 0x98) = 0x9;
    return iVar1;
  case 0x8:
    (iVar1 + 0x98) = 0xb;
    return iVar1;
  case 0xa:
    (iVar1 + 0x98) = 0x5;
    return iVar1;
  case 0xb:
    (iVar1 + 0x98) = 0x8;
    return iVar1;
  }
  return iVar1;
}
pub fn pass1_1030_d61c(mut param_1: u32,mut param_2: u32)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;
  in_stack_0000ffcc: mut HFILE16;
  let mut local_1a: u32;
  let mut local_16: *mut u8;
  let mut local_14: u16;
  u32 local_12 [0x3];
  let mut iStack4: i16;

  BVar1 = write_to_file_1028_b5ec(param_1,param_2);
  if (BVar1 != 0) {
    for (iStack4 = 0; iStack4 < 0xa; iStack4 += 1) {
      uVar3 = (param_1 >> 0x10);
      iVar2 = param_1;
      local_12[0] = (iVar2 + iStack4 * 0xc + 0x20);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_12),0x4,in_stack_0000ffcc);
      if (BVar1 == 0) goto LAB_1030_d701;
      local_14 = (iVar2 + iStack4 * 0xc + 0x24);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_14),0x2,in_stack_0000ffcc);
      if (BVar1 == 0) goto LAB_1030_d701;
      local_16 = (iVar2 + iStack4 * 0xc + 0x26);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_16),0x2,in_stack_0000ffcc);
      if (BVar1 == 0) goto LAB_1030_d701;
      local_1a = (iVar2 + iStack4 * 0xc + 0x28);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_1a),0x4,in_stack_0000ffcc);
      if (BVar1 == 0) goto LAB_1030_d701;
    }
    local_16 = PTR_LOOP_1050_5812;
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_16),0x2,in_stack_0000ffcc);
    if (BVar1 != 0) {
      return;
    }//
// LAB_1030_d701:
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1030_d72e(mut param_1: i16,param_2: *mut u8,param_3: *mut astruct_373,HFILE16 *param_4)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut iVar3: i16;
  let mut iStack10: i16;
  let mut local_8: u32;
  let mut local_4: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 == 0) {
    return;
  }
  iStack10 = 0;
  while( true ) {
    if (0x9 < iStack10) {
    // just 0x5812
      if ((0x3 < u16_1050_0312) &&
         (BVar2 = read_file_1008_7dee(param_4,&PTR_LOOP_1050_5812,0x2), BVar2 == 0)) {
        u16_1050_0310 = 0x6d2;
        return;
      }
      return;
    }
    BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_8),0x4);
    if (BVar2 == 0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
    if (BVar2 == 0) break;
    iVar3 = iStack10 * 0xc + param_3;
    (iVar3 + 0x20) = local_8;
    (iVar3 + 0x22) = local_8;
    uVar1 = switch_1008_72bc(param_4,local_4);
    (iVar3 + 0x24) = uVar1;
    if (u16_1050_0312 < 0x2) {
      iVar3 = iStack10 * 0xc + param_3;
      (iVar3 + 0x26) = 0x3;
      (iVar3 + 0x28) = 0;
    }
    else {
      BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 == 0) {
        u16_1050_0310 = 0x6d2;
        return;
      }
      BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_8),0x4);
      if (BVar2 == 0) {
        u16_1050_0310 = 0x6d2;
        return;
      }
      iVar3 = iStack10 * 0xc + param_3;
      (iVar3 + 0x26) = local_4;
      (iVar3 + 0x28) = local_8;
    }
    iStack10 += 0x1;
  }
  u16_1050_0310 = 0x6d2;
  return;
}



pub fn pass1_1030_d868(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_180 * struct_1030_d8f6(param_1: *mut astruct_180)

{
  iVar1: *mut astruct_180;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0xdc2e;
  iVar1.field1_0x2 = 0x1030;
  if (iVar1.field10_0xc == 0x4c) {
    iVar1.field11_0xe = 0x43;
  }
  else if (iVar1.field10_0xc == 0x4d) {
    iVar1.field11_0xe = 0x44;
  }
  else {
    iVar1.field11_0xe = 0x45;
  }
  return param_1;
}



pub fn pass1_1030_d942(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0xdc2e;
  (param_2 + 0x2) = 0x1030;
  if ((param_2 + 0xc) == 0x4c) {
    (param_2 + 0xe) = 0x43;
  }
  else if ((param_2 + 0xc) == 0x4d) {
    (param_2 + 0xe) = 0x44;
  }
  else {
    (param_2 + 0xe) = 0x45;
  }
  return param_2;
}
pub fn pass1_1030_d994(param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x12) != 0x4) {
    return;
  }
  uVar6 = pass1_1028_b4f2(param_1);
  iVar3 = uVar6;
  if ((iVar3 + 0x200) == 0x8000002) {
    uVar2 = (iVar4 + 0x14);
    piVar1 = (uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
  }
  else {
    pass1_1028_cb04(param_1);
    if (iVar3 == 0) {
      return;
    }
    pass1_1030_dace(param_1);
    if (iVar3 == 0) {
      return;
    }
    uVar2 = (iVar4 + 0x14);
    piVar1 = (uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
    pass1_1028_c952(param_1);
    pass1_1030_da22(param_1);
  }
  uVar2 = (iVar4 + 0x14);
  if ((uVar2 + 0x94) < 1) {
    pass1_1028_bdac(param_1,0x5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_da22(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut BVar4: bool;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut uStack18: u32;

  uVar9 = pass1_1028_b4f2(param_1);
  uVar3 = (uVar9 >> 0x10);
  puVar1 = (uVar9 + 0xc);
  ppcVar2 = (*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)(0x1028,puVar1,(uVar9 + 0xe));
  uStack18 = 0;
  while( true ) {
    if ((puVar6 & 0xffff | extraout_DX << 0x10) <= uStack18) {
      return;
    }
    uVar9 = pass1_1030_1d7c((puVar6 & 0xffff),extraout_DX,puVar1);
    uVar7 = (uVar9 >> 0x10);
    uVar8 = uVar7 | uVar9;
    if (((uVar8 != 0) &&
        (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,(uVar9 + 0xc),0x4), BVar4 != 0)) &&
       (uVar5 = pass1_1028_6744(uVar9,0xd), (uVar8 | uVar5) != 0)) break;
    uStack18 += 0x1;
  }
  pass1_1028_6228(uVar9,0x1,0x0,0xd);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_dace(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut BVar4: bool;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut uStack20: u32;

  uVar9 = pass1_1028_b4f2(param_1);
  uVar3 = (uVar9 >> 0x10);
  puVar1 = (uVar9 + 0xc);
  ppcVar2 = (*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)(0x1028,puVar1,(uVar9 + 0xe));
  uStack20 = 0;
  uVar8 = extraout_DX;
  loop {
    if ((puVar6 & 0xffff | extraout_DX << 0x10) <= uStack20) {
      return;
    }
    uVar9 = pass1_1030_1d7c((puVar6 & 0xffff),uVar8,puVar1);
    uVar7 = (uVar9 >> 0x10);
    uVar8 = uVar7 | uVar9;
    if ((uVar8 != 0) && (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,(uVar9 + 0xc),0x4), BVar4 != 0)
       ) {
      uVar5 = pass1_1028_6744(uVar9,0xd);
      uVar8 |= uVar5;
      if (uVar8 != 0) {
        return;
      }
    }
    uStack20 += 0x1;
  } while( true );
}



pub unsafe fn pass1_1030_db72() -> u16

{
  return 0x1;
}
pub fn pass1_1030_db78(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x12) == 0x6) {
    pass1_1028_bdac((param_1 & 0xffff | uVar1 << 0x10),0x5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_db92(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,param_5: i32)

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



pub unsafe fn pass1_1030_dc02() -> u16

{
  return 0x1;
}



pub fn pass1_1030_dc08(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_180 * struct_1030_dc96(param_1: *mut astruct_180)

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0;
  param_1.field0_0x0 = 0xe036;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



u16 * pass1_1030_dcc2(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0;
  param_2.field0_0x0 = 0xe036;
  (param_2 + 0x2) = 0x1030;
  return &param_2.field0_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_dcf4(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u8;
  let mut extraout_DX_00: u16;
  let mut uVar7: u16;
  iVar9: *mut astruct_15;
  let mut uVar8: u16;
  let mut puVar9: *mut u32;
  let mut uVar10: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut puStack20: *mut u32;
  let mut iStack12: i16;

  uVar8 = (param_2 >> 0x10);
  iVar9 = param_2;
  param_2.field0_0x0 = 0xe036;
  iVar9.field1_0x2 = 0x1030;
  if (_PTR_LOOP_1050_65e2 != 0) {
    pass1_1028_b58e(param_2);
    if (&iVar9.field24_0x20 == 0) {
      uVar2 = extraout_DX | param_1;
      if (uVar2 == 0) {
        uVar5 = extraout_DX;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
      }
      else {
        uVar2 = (param_1 + 0x2e);
        uVar5 = (param_1 + 0x30);
      }
      puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
      puVar6 = (puVar9 >> 0x10);
      uVar3 = puVar9;
      pass1_1038_4d6e(uVar3,puVar6,CONCAT22(uVar5,uVar2),puVar9);
      puStack20 = CONCAT22(puVar6,uVar3);
      ppcVar1 = (*puStack20 + 0x10);
      uVar5 = uVar3;
      (**ppcVar1)(&u16_1050_1038,uVar3,puVar6);
      uStack24 = CONCAT22(extraout_DX_00,uVar5);
      uVar2 = extraout_DX_00;
      for (uStack28 = 0; uStack28 < uStack24; uStack28 += 1) {
        uVar10 = pass1_1030_1d7c(uVar5,uVar2,puStack20);
        uVar7 = (uVar10 >> 0x10);
        uVar2 = uVar7 | uVar10;
        if (uVar2 != 0) {
          uVar4 = pass1_1030_dfcc(param_2);
          uVar4 = pass1_1030_cbf0(uVar10,uVar7,uVar4);
          if (uVar4 != 0) break;
        }
      }
      if (puStack20.is_null() == false) {
        ppcVar1 = *puStack20;
        (**ppcVar1)(0x38,uVar3,puVar6,1);
      }
    }
    else {
      uVar2 = extraout_DX;
      uVar5 = param_1;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,&iVar9.field24_0x20);
      if ((uVar2 | uVar5) != 0) {
        iStack12 = 0;
        switch(iVar9.field10_0xc) {
        case 0x73:
        case 0x77:
          iStack12 = 0x1;
          break;
        case 0x74:
        case 0x78:
          iStack12 = 0x2;
          break;
        case 0x75:
          iStack12 = 0x3;
          break;
        case 0x76:
          iStack12 = 0x5;
        }
        if (iStack12 != 0) {
          pass1_1030_cc44(uVar5,uVar2,0x1,CONCAT22(extraout_DX,param_1),iStack12);
        }
      }
    }
  }
  pass1_1028_b418(&param_2.field0_0x0);
  return;
}
pub fn pass1_1030_de7c(mut param_1: u32,mut param_2: u32)

{
  let mut BVar1: bool;
  in_stack_0000ffda: mut HFILE16;
  u32 local_10 [0x3];

  BVar1 = write_to_file_1028_b5ec(param_1,param_2);
  if (BVar1 != 0) {
    local_10[0] = (param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffda);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
  }
  return;
}
pub fn pass1_1030_dec4(mut param_1: i16,param_2: *mut u8,param_3: *mut astruct_373,HFILE16 *param_4)

{
  let mut BVar1: bool;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (((param_1 != 0) && (0x1 < u16_1050_0312)) &&
     (BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (param_3 + 0x20)),0x4),
     BVar1 == 0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}
pub fn pass1_1030_df0c(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut lVar3: i32;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u32;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack14: u16;
  let mut uStack10: u16;

  pass1_1028_b58e(param_2);
  uVar1 = (param_1 + 0x2e);
  uStack10 = uVar1;
  if (((param_1 + 0x30) | uStack10) != 0) {
    uVar9 = (uVar1 >> 0x10);
    uVar1 = (uStack10 + 0x210);
    uVar7 = (uStack10 + 0x212);
    uStack14 = uVar1;
    if ((uVar7 | uStack14) != 0) {
      uVar2 = (uStack14 + 0xa);
      uVar4 = pass1_1030_dfcc(param_2);
      if (uVar4 != 0) {
        uStack24 = 0x1;
        uStack22 = 0;
        while (CONCAT22(uStack22,uStack24) < uVar2) {
          uVar6 = uVar2;
          uVar10 = uVar4;
          bad_1030_1312();
          uVar8 = uVar7;
          iVar5 = pass1_1030_cde8(uVar6,uVar7,uVar10);
          if (-0x1 < iVar5) {
            pass1_1030_cef8(uVar6 & 0xffff | uVar7 << 0x10,CONCAT22(extraout_DX,param_1),0x1,iVar5);
            (param_2 + 0x20) = (uVar6 + 0x4);
            return;
          }
          lVar3 = CONCAT22(uStack22,uStack24) + 1;
          uStack24 = lVar3;
          uVar7 = uVar8;
          uStack22 = (lVar3 >> 0x10);
        }
      }
    }
  }
  return;
}



pub unsafe fn pass1_1030_dfcc(mut param_1: u32) -> u16

{
  let mut iVar1: i16;
  let mut uStack4: u16;

  iVar1 = (param_1 + 0xc);
  if (iVar1 == 0x73) {//
// LAB_1030_dfde:
    uStack4 = 0x1;
  }
  else {
    if (iVar1 != 0x74) {
      if (iVar1 == 0x75) {
        return 0x3;
      }
      if (iVar1 == 0x77) goto LAB_1030_dfde;
      if (iVar1 != 0x78) {
        return 0x0;
      }
    }
    uStack4 = 0x2;
  }
  return uStack4;
}
