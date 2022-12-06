
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_42cc(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut bVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u8;
  let mut extraout_DX: u16;
  let mut uVar8: u16;
  let mut extraout_DX_00: u16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut puVar12: *mut u32;
  let mut puVar13: *mut u32;
  let mut uStack24: u32;
  let mut uStack18: u32;
  let mut puStack10: *mut u32;

  uVar10 = (param_1 >> 0x10);
  iVar9 = param_1;
  if (*(i32 *)(iVar9 + 0x1f6) == 0x0) {
    return;
  }
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_u16_1050_06e0,0x2d);
  puVar7 = (puVar12 >> 0x10);
  uVar4 = puVar12;
  pass1_1038_4d6e(uVar4,puVar7,(astruct_691 *)param_1,puVar12);
  puStack10 = CONCAT22(puVar7,uVar4);
  ppcVar1 = (code **)(*puStack10 + 0x10);
  uVar5 = uVar4;
  (**ppcVar1)(0x1008,uVar4,puVar7);
  uStack18 = CONCAT22(extraout_DX,uVar5);
  bVar3 = false;
  uVar8 = extraout_DX;
  for (uStack24 = 0x0; uStack24 < uStack18; uStack24 += 0x1) {
    uVar11 = 0x1030;
    puVar13 = pass1_1030_1d7c(uVar5,uVar8,puStack10);
    uVar6 = puVar13;
    uVar8 = (puVar13 >> 0x10) | uVar6;
    if (uVar8 != 0x0) {
      ppcVar1 = (code **)(*puVar13 + 0x50);
      (**ppcVar1)();
      uVar8 = extraout_DX_00;
      if (uVar6 != 0x0) {
        bVar3 = true;
      }
    }
  }
  if (bVar3) {
    uVar2 = (iVar9 + 0x1f6);
    (uVar2 + 0x1aa) = 0x0;
  }
  else {
    uVar11 = 0x1030;
    pass1_1030_38b8();
    uVar8 |= uStack18;
    if (uVar8 != 0x0) {
      uVar11 = 0x1030;
      pass1_1030_326a(uStack18,uVar8,*(astruct_692 **)(iVar9 + 0x1f6));
    }
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(uVar11,uVar4,puVar7,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_43cc(mut param_1: i16,mut param_2: i16,mut param_3: i16,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut paVar7: *mut Struct57;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  astruct_15 *paVar12;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut puStack14: *mut u32;

  if (param_6 == 0x5) {
    pass1_1038_4900(CONCAT22(param_4,param_3));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_4,param_3),param_6);
  if ((param_2 != 0x0) || (param_1 != 0x0)) {
    iVar8 = param_6 * 0x4;
    uVar2 = (param_3 + iVar8 + 0x14e);
    iVar9 = ((param_3 + iVar8 + 0x150) - (param_5 >> 0xf)) - (uVar2 < param_5);
    (param_3 + iVar8 + 0x14e) = uVar2 - param_5;
    (param_3 + iVar8 + 0x150) = iVar9;
    if (iVar9 < 0x0) {
      (param_3 + iVar8 + 0x14e) = 0x0;
    }
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
    paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,(puVar11 >> 0x10));
    uVar2 = puVar11;
    pass1_1038_4e78(uVar2,paVar6,CONCAT22(param_4,param_3),puVar11);
    puStack14 = CONCAT22(paVar6,uVar2);
    ppcVar1 = (code **)(*puStack14 + 0x10);
    paVar7 = paVar6;
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,paVar6);
    uStack18 = CONCAT22(paVar7,uVar3);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      paVar12 = (astruct_15 *)pass1_1030_1d7c(uVar3,paVar7,puStack14);
      paVar7 = (astruct_57 *)(paVar12 >> 0x10);
      uVar5 = paVar12 & 0xffff;
      for (; uVar4 = uVar5, param_5 != 0x0; param_5 -= 0x1) {
        pass1_1030_cf78(paVar12,param_6);
        uVar5 = uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar10 = 0x1030;
      if (param_5 == 0x0) break;
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar10,uVar2,paVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_44d8(mut param_1: i16,mut param_2: i16,mut param_3: i16,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut paVar7: *mut Struct57;
  astruct_697 *iVar9;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut puVar10: *mut u32;
  astruct_15 *paVar11;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut puStack14: *mut u32;

  if (param_6 == 0x5) {
    pass1_1038_4900(CONCAT22(param_4,param_3));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_4,param_3),param_6);
  if ((param_2 != 0x0) || (param_1 != 0x0)) {
    iVar9 = (astruct_697 *)(param_6 * 0x4);
    uVar2 = (iVar9 + param_3 + 0x14e);
    iVar8 = ((iVar9 + param_3 + 0x150) - (param_5 >> 0xf)) - (uVar2 < param_5);
    (iVar9 + param_3 + 0x14e) = uVar2 - param_5;
    (iVar9 + param_3 + 0x150) = iVar8;
    if (iVar8 < 0x0) {
      (iVar9 + param_3 + 0x14e) = 0x0;
    }
    uVar9 = 0x1008;
    puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
    paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,(puVar10 >> 0x10));
    uVar2 = puVar10;
    pass1_1038_4e78(uVar2,paVar6,CONCAT22(param_4,param_3),puVar10);
    puStack14 = CONCAT22(paVar6,uVar2);
    ppcVar1 = (code **)(*puStack14 + 0x10);
    paVar7 = paVar6;
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,paVar6);
    uStack18 = CONCAT22(paVar7,uVar3);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      paVar11 = (astruct_15 *)pass1_1030_1d7c(uVar3,paVar7,puStack14);
      paVar7 = (astruct_57 *)(paVar11 >> 0x10);
      uVar5 = paVar11 & 0xffff;
      for (; uVar4 = uVar5, param_5 != 0x0; param_5 -= 0x1) {
        pass1_1030_d00c(paVar11,param_6);
        uVar5 = uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar9 = 0x1030;
      if (param_5 == 0x0) break;
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar9,uVar2,paVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_45e4(mut param_1: u16 ,mut param_2: i16,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut iVar10: i16;
  let mut puVar11: *mut u8;
  let mut iVar12: i16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut bVar15: bool;
  let mut puVar16: *mut u32;
  let mut uStack28: u16;
  let mut puStack22: *mut u32;

  uVar14 = (param_3 >> 0x10);
  iVar12 = param_3;
  pass1_1030_38f2((iVar12 + 0x1f6),0x2);
  iVar8 = param_2;
  uVar4 = param_1;
  pass1_1030_38f2((iVar12 + 0x1f6),0x1);
  bVar15 = param_1 < uVar4;
  uVar13 = param_1 - uVar4;
  iVar10 = param_2 - iVar8;
  pass1_1030_38f2((iVar12 + 0x1f6),0x4);
  iVar9 = iVar8;
  uVar5 = uVar4;
  pass1_1030_38f2((iVar12 + 0x1f6),0x3);
  uVar7 = (iVar12 + 0x24);
  uVar6 = uVar7 + (uVar4 - uVar5);
  iVar10 = (uVar7 >> 0xf) + ((iVar8 - iVar9) - (uVar4 < uVar5)) + CARRY2(uVar7,uVar4 - uVar5) +
           (iVar10 - bVar15) + CARRY2(uVar6,uVar13);
  if ((iVar10 < 0x0) || ((iVar10 < 0x1 && (uVar6 + uVar13 == 0x0)))) {
    iVar10 = -0x1;
  }
  else {
    iVar10 = 0x1;
  }
  piVar1 = (iVar12 + 0x24);
  *piVar1 = *piVar1 + iVar10;
  puVar16 = pass1_1008_c6fa(_u16_1050_06e0,0x16);
  puVar11 = (puVar16 >> 0x10);
  uVar7 = puVar16;
  pass1_1038_4d6e(uVar7,puVar11,(astruct_691 *)param_3,puVar16);
  puStack22 = CONCAT22(puVar11,uVar7);
  uVar3 = *puStack22;
  ppcVar2 = (code **)uVar3 + 0x8;
  uVar5 = uVar7;
  (**ppcVar2)(0x1008,uVar7,puVar11);
  if (puStack22 != NULL) {
    ppcVar2 = (code **)uVar3;
    (**ppcVar2)(0x1008,uVar7,puVar11,0x1);
  }
  piVar1 = (iVar12 + 0x24);
  *piVar1 = *piVar1 + uVar5 * 0x2;
  iVar10 = (iVar12 + 0x24);
  if (0x64 < iVar10) {
    iVar10 = 0x64;
  }
  (iVar12 + 0x24) = iVar10;
  if (iVar10 < 0x0) {
    iVar10 = 0x0;
  }
  (iVar12 + 0x24) = iVar10;
  iVar10 /= 0xa;
  uStack28 = 0x10;
  if (iVar10 < 0xb) {
    uStack28 = 0x14;
  }
  else if (iVar10 < 0x15) {
    uStack28 = 0x13;
  }
  else if (iVar10 < 0x1f) {
    uStack28 = 0x12;
  }
  else if (iVar10 < 0x29) {
    uStack28 = 0x11;
  }
  else if (iVar10 < 0x33) {
    uStack28 = 0x10;
  }
  else if (iVar10 < 0x3d) {
    uStack28 = 0xf;
  }
  else if (iVar10 < 0x47) {
    uStack28 = 0xe;
  }
  else if (iVar10 < 0x51) {
    uStack28 = 0xd;
  }
  else if (iVar10 < 0x5b) {
    uStack28 = 0xc;
  }
  pass1_1030_3258((iVar12 + 0x1f6),uStack28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4760(mut param_1: u32)

{
  let mut puVar1: *mut u16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u8;
  let mut puVar8: *mut u8;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut extraout_DX_02: u16;
  let mut extraout_DX_03: u16;
  let mut uVar9: u16;
  astruct_700 *iVar10;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut puVar12: *mut u32;
  let mut uVar13: u32;
  let mut puVar14: *mut u32;
  u8 uVar15;
  let mut puVar16: *mut u8;
  let mut uStack26: u32;
  let mut uStack22: u32;
  let mut puStack14: *mut u32;
  let mut puStack10: *mut u32;

  uVar10 = (param_1 >> 0x10);
  iVar10 = (astruct_700 *)param_1;
  puVar1 = &iVar10->field33_0x22;
  *puVar1 = *puVar1 + iVar10->field521_0x20c;
  puVar12 = pass1_1008_c6fa(_u16_1050_06e0,0x26);
  puVar7 = (puVar12 >> 0x10);
  uVar6 = puVar12;
  pass1_1038_4d6e(uVar6,puVar7,(astruct_691 *)param_1,puVar12);
  puStack10 = CONCAT22(puVar7,uVar6);
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_u16_1050_06e0,0x1a);
  puVar8 = (puVar12 >> 0x10);
  uVar3 = puVar12;
  pass1_1038_4d6e(uVar3,puVar8,(astruct_691 *)param_1,puVar12);
  puStack14 = CONCAT22(puVar8,uVar3);
  ppcVar2 = (code **)(*puStack14 + 0x10);
  uVar4 = uVar3;
  (**ppcVar2)(0x1008,uVar3,puVar8);
  uVar15 = uVar6;
  puVar16 = puVar7;
  if ((extraout_DX | uVar4) == 0x0) {
    ppcVar2 = (code **)(*puStack10 + 0x10);
    (**ppcVar2)();
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + uVar4;
    uVar9 = extraout_DX_00;
  }
  else {
    ppcVar2 = (code **)(*puStack10 + 0x10);
    (**ppcVar2)();
    uStack22 = CONCAT22(extraout_DX_03,uVar4);
    uVar9 = extraout_DX_03;
    for (uStack26 = 0x0; uStack26 < uStack22; uStack26 += 0x1) {
      puVar14 = puStack14;
      uVar13 = pass1_1030_1d7c(uVar4,uVar9,puStack10);
      uVar9 = (uVar13 >> 0x10);
      uVar5 = uVar13;
      uVar11 = 0x1028;
      FUN_1028_5a94(uVar5,0x1030,(astruct_15 *)(uVar13 & 0xffff | uVar9 << 0x10),puVar14);
      if (uVar5 == 0x2) {
        if ((*_PTR_LOOP_1050_65e2 & 0x1) == 0x0) goto LAB_1038_485e;
      }
      else if (uVar5 != 0x3) {//
LAB_1038_485e:
        puVar1 = &iVar10->field33_0x22;
        *puVar1 = *puVar1 + 0x1;
      }
    }
  }
  if (puStack10 != NULL) {
    ppcVar2 = (code **)*puStack10;
    (**ppcVar2)(uVar11,uVar6,puVar7,0x1,uVar15,puVar16);
    uVar9 = extraout_DX_01;
  }
  if (puStack14 != NULL) {
    ppcVar2 = (code **)*puStack14;
    (**ppcVar2)(uVar11,uVar3,puVar8,0x1);
    uVar9 = extraout_DX_02;
  }
  pass1_1038_45e4(puStack14,uVar9,param_1);
  if (0x32 < iVar10->field34_0x24) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 - 0x1;
  }
  if (iVar10->field34_0x24 < 0x32) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + 0x1;
  }
  if (iVar10->field24_0x18 < 0xfa) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + 0x2;
  }
  else if (iVar10->field24_0x18 < 0x1c2) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + 0x1;
  }
  else if (0x225 < iVar10->field24_0x18) {
    if (iVar10->field24_0x18 < 0x2ee) {
      puVar1 = &iVar10->field33_0x22;
      *puVar1 = *puVar1 - 0x1;
    }
    else {
      puVar1 = &iVar10->field33_0x22;
      *puVar1 = *puVar1 - 0x2;
    }
  }
  uVar6 = iVar10->field33_0x22;
  if (0x64 < uVar6) {
    uVar6 = 0x64;
  }
  iVar10->field33_0x22 = uVar6;
  if (uVar6 < 0x0) {
    uVar6 = 0x0;
  }
  iVar10->field33_0x22 = uVar6;
  return;
}
pub fn pass1_1038_48e0(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (param_1 + 0x20e) + param_2;
  if (0xa < iVar1) {
    iVar1 = 0xa;
  }
  (param_1 + 0x20e) = iVar1;
  return;
}
pub fn pass1_1038_4900(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  piVar1 = (param_1 + 0x20e);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    (param_1 + 0x20e) = 0x0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4918(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u32;
  u8 bStack347;
  u8 local_14a [0x4];
  let mut puStack326: *mut u32;
  u8 local_144 [0x124];
  let mut local_20: u32;
  let mut uStack28: u16;
  let mut uStack26: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar8 = (param_3 >> 0x10);
  iVar6 = param_3;
  if (*(i32 *)(iVar6 + 0x4) != 0x4000001) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar6 + 0x8));
  uStack6 = CONCAT22(param_2,param_1);
  uStack10 = (param_1 + 0x10);
  uVar9 = (uStack10 >> 0x10);
  iVar7 = uStack10;
  if ((iVar7 + 0x1c) == 0x0) {
    return;
  }
  uStack14 = 0x0;
  switch((iVar6 + 0x20e)) {
  case 0x1:
    uStack14 = 0x1e;
    break;
  case 0x2:
    uStack14 = 0x1c;
    break;
  case 0x3:
    uStack14 = 0x1a;
    break;
  case 0x4:
    uStack14 = 0x18;
    break;
  case 0x5:
    uStack14 = 0x16;
    break;
  case 0x6:
    uStack14 = 0x14;
    break;
  case 0x7:
    uStack14 = 0x12;
    break;
  case 0x8:
    uStack14 = 0x10;
    break;
  case 0x9:
    uStack14 = 0xe;
    break;
  case 0xa:
    uStack14 = 0xc;
    break;
  default:
// TODO: goto switchD_1038_49cf_caseD_a;
  }
  uStack14 = uStack14;
switchD_1038_49cf_caseD_a:
  uStack18 = *_PTR_LOOP_1050_65e2;
  if ((uStack14 != 0x0) &&
     (((uStack18 & 0xffff | (_PTR_LOOP_1050_65e2 + 0x2) << 0x10) % uStack14) == 0x0)) {
    piVar1 = (iVar7 + 0x1c);
    *piVar1 = *piVar1 + -0x1;
    piVar1 = (iVar7 + 0x1a);
    *piVar1 = *piVar1 + 0x1;
    iVar2 = (iVar7 + 0x1a) * 0x6 + (iVar7 + 0x16);
    uVar9 = (iVar7 + 0x18);
    local_20 = (iVar2 + -0x6);
    uStack28 = (iVar2 + -0x2);
    puStack326 = &local_20;
    puVar3 = &local_20;
    pass1_1030_64ce(puVar3,uVar9,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),*(i32 *)(iVar6 + 0x8),
                    CONCAT22(0x1050,local_14a));
    uStack26 = *puVar3;
    uVar5 = (puVar3 + 0x2);
    bStack347 = (uStack26 >> 0x18);
    uVar4 = bStack347;
    if (bStack347 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack26 & 0xffff | uVar5 << 0x10);
      uVar10 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar4),uVar4,uVar5);
      uVar5 = (uVar10 >> 0x10);
      if ((uVar5 | uVar10) != 0x0) {
        iVar7 = (uVar10 + 0xc);
        if (iVar7 < 0x1) {
          return;
        }
        if (SBORROW2(iVar7,0x1)) {
          return;
        }
        if (0x8 < iVar7 + -0x1) {
          return;
        }
      }
    }
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_144),0x0,0x0,0x10,&local_20,&DAT_1050_1050,
                        (iVar6 + 0x4),(iVar6 + 0x8));
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_144));
  }
  return;
}
pub fn pass1_1038_4b20(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  astruct_360 *paVar1;

  paVar1 = *(astruct_360 **)(param_2 + 0xc);
  pass1_1020_c4f4((astruct_361 *)paVar1,param_1,paVar1,param_3,(param_3 >> 0x10),param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4b40(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_2 >> 0x10);
  iVar6 = param_2;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_1);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x4);
    uVar3 = uStack10;
    (**ppcVar1)(unaff_CS,(iVar6 + 0xc));
    uVar2 = uVar3;
    uVar5 = extraout_DX_00 | uVar2;
    if (uVar5 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3 & 0xffff | extraout_DX_00 << 0x10);
      unaff_CS = 0x1030;
      struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar2),uVar2,uVar5);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4c1a(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut uVar8: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_3 >> 0x10);
  iVar6 = param_3;
  uVar8 = (iVar6 + 0xc);
  ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22(param_2,param_1);
  for (uStack14 = 0x0; uVar5 = param_2, uStack14 < uStack10; uStack14 += 0x1) {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar1)(unaff_CS,(iVar6 + 0xc),uStack14,uVar8);
    uVar2 = uVar4;
    param_2 = (uVar5 | uVar2);
    if ((uVar5 | uVar2) != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | uVar5 << 0x10);
      uVar3 = pass1_1030_6fa0(CONCAT22(param_2,uVar2));
      unaff_CS = 0x1008;
      pass1_1008_c6ae(_u16_1050_06e0,uVar3,0xe);
    }
  }
  return;
}
pub fn pass1_1038_4cba()

{
  pass1_1030_38b8();
  return;
}
pub fn pass1_1038_4cd0(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1c) = param_3;
  (param_1 + 0x1e) = param_2;
  return;
}
pub fn pass1_1038_4cea(mut param_1: u32,u32 *param_2,param_3: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x1c);
  *param_2 = (param_1 + 0x1e);
  return;
}
pub fn pass1_1038_4d0e(param_1: *mut astruct_685,mut param_2: u16 )

{
  astruct_686 *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_686 *)param_1;
  iVar1->field25_0x1a = iVar1->field24_0x18;
  iVar1->field24_0x18 = param_2;
  return;
}



char * pass1_1038_4d28(char *param_1)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x1fc),(param_1 + 0x1fa));
}
pub fn pass1_1038_4d3c(mut param_1: u32,char *param_2,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  fn_ptr_1000_17ce(*(iVar2 + 0x1fa));
  uVar1 = str_op_1008_60e8(param_3,param_2);
  (iVar2 + 0x1fa) = uVar1;
  (iVar2 + 0x1fc) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4d6e(mut param_1: u16 ,u8 *param_2,param_3: *mut astruct_691,u32 *param_4)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u32;
  let mut iStack30: i16;
  astruct_419 *paStack26;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut puStack6: *mut u32;
  let mut uVar7: u32;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x18,paVar6);
  uVar5 = paVar6 | param_1;
  uVar7 = uVar5;
  if (uVar5 == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(paVar6,param_1),0x5,0x5);
  }
  puStack6 = CONCAT22(uVar7,param_1);
  uVar9 = (param_3 >> 0x10);
  iVar8 = param_3;
  if (*(i32 *)(iVar8 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    ppcVar2 = (code **)((iVar8 + 0xc) + 0x10);
    (**ppcVar2)();
  }
  uStack10 = CONCAT22(uVar7,param_1);
  uStack14 = 0x0;
  loop {
    uVar5 = uVar7;
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar2 = (code **)((iVar8 + 0xc) + 0x4);
    uVar10 = uStack10;
    (**ppcVar2)();
    uVar3 = uVar10;
    uVar7 = (uVar5 | uVar3);
    if ((uVar5 | uVar3) != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | uVar5 << 0x10);
      paStack26 = (astruct_419 *)CONCAT22(uVar7,uVar3);
      uVar4 = pass1_1030_6fa0(CONCAT22(uVar7,uVar3));
      iStack30 = 0x0;
      while( true ) {
        piVar1 = (param_4 + 0x4);
        if (*piVar1 == iStack30 || *piVar1 < iStack30) break;
        if ((*param_4 + iStack30 * 0x2) == uVar4) {
          uVar10 = struct_op_1030_73a8(paStack26,uVar4,uVar7);
          uVar7 = uVar10 >> 0x10;
          if ((uVar10 + 0x12) == 0x5) {
            ppcVar2 = (code **)(*puStack6 + 0xc);
            (**ppcVar2)();
          }
          break;
        }
        iStack30 += 0x1;
      }
    }
    uStack14 += 0x1;
  } while( true );
}
pub fn pass1_1038_4e78(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,u32 *param_4)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut iStack26: i16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut puStack6: *mut u32;
  let mut uVar7: u32;

  mem_op_1000_179c(0x18,param_2);
  uVar5 = param_2 | param_1;
  uVar7 = uVar5;
  if (uVar5 == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2,param_1),0x5,0x5);
  }
  puStack6 = CONCAT22(uVar7,param_1);
  uVar9 = (param_3 >> 0x10);
  iVar8 = param_3;
  if (*(i32 *)(iVar8 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    ppcVar2 = (code **)((iVar8 + 0xc) + 0x10);
    (**ppcVar2)();
  }
  uStack10 = CONCAT22(uVar7,param_1);
  uStack14 = 0x0;
  loop {
    uVar5 = uVar7;
    if (uStack10 <= uStack14) {
      return;
    }
    uVar4 = uStack10;
    pass1_1030_1d58((iVar8 + 0xc));
    uVar6 = uVar5 | uVar4;
    uVar7 = uVar6;
    if (uVar6 != 0x0) {
      uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | uVar5 << 0x10);
      iStack26 = 0x0;
      while( true ) {
        piVar1 = (param_4 + 0x4);
        if (*piVar1 == iStack26 || *piVar1 < iStack26) break;
        if ((*param_4 + iStack26 * 0x2) == uVar3) {
          ppcVar2 = (code **)(*puStack6 + 0xc);
          (**ppcVar2)();
          break;
        }
        iStack26 += 0x1;
      }
    }
    uStack14 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4f54(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar8 = (param_2 >> 0x10);
  iVar7 = param_2;
  if (*(i32 *)(iVar7 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar7 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar5 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar5,param_1);
  uStack10 = 0x0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    uVar4 = uStack6;
    pass1_1030_1d58((iVar7 + 0xc));
    uVar6 = uVar5 | uVar4;
    if (uVar6 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar4 & 0xffff | uVar5 << 0x10);
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,param_3);
      if (BVar3 != 0x0) {
        return;
      }
    }
    uStack10 += 0x1;
    uVar5 = uVar6;
  } while( true );
}
pub fn pass1_1038_4fd8(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar7 = (param_2 >> 0x10);
  iVar6 = param_2;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_1);
  uStack10 = 0x0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    uVar3 = uStack6;
    pass1_1030_1d58((iVar6 + 0xc));
    uVar5 = uVar4 | uVar3;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | uVar4 << 0x10);
      if (uVar2 == param_3) {
        return;
      }
    }
    uStack10 += 0x1;
    uVar4 = uVar5;
  } while( true );
}
