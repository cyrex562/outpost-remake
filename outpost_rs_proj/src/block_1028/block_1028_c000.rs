
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c00a(mut param_1: i16,param_2: *mut astruct_15,i32 param_3)

{
  astruct_691 *paVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u8 *puVar5;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u32 *puVar8;
  let mut uVar9: u32;
  let mut uVar10: u32;
  let mut uStack26: u32;
  let mut uStack22: u32;
  u32 *puStack18;

  pass1_1028_b58e(param_2);
  paVar1 = *(astruct_691 **)(param_1 + 0x2e);
  puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar5 = ((u32)puVar8 >> 0x10);
  uVar3 = puVar8;
  uVar7 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4d6e(uVar3,puVar5,paVar1,puVar8);
  puStack18 = (u32 *)CONCAT22(puVar5,uVar3);
  ppcVar2 = (code **)((int)*puStack18 + 0x10);
  uVar6 = uVar3;
  (**ppcVar2)((int)&u16_1050_1038,uVar3,puVar5);
  uStack22 = CONCAT22(extraout_DX_00,uVar6);
  uStack26 = 0x0;
  do {
    if (uStack22 <= uStack26) {//
LAB_1028_c0d6:
      if (puStack18 != NULL) {
        ppcVar2 = (code **)*puStack18;
        (**ppcVar2)(uVar7,uVar3,(char)puVar5,0x1);
      }
      return;
    }
    ppcVar2 = (code **)((int)*puStack18 + 0x4);
    uVar9 = uStack22;
    (**ppcVar2)((char)uVar7,uVar3,puVar5,(char)uStack26,(int)(uStack26 >> 0x10));
    uVar4 = uVar9;
    uVar6 = extraout_DX_01;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9 & 0xffff | (u32)extraout_DX_01 << 0x10);
    uVar7 = 0x1030;
    uVar9 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar6,uVar4),uVar4,uVar6);
    uVar10 = pass1_1028_6302(uVar9);
    uVar6 = (uVar10 >> 0x10);
    if ((param_3 <= uVar6) && ((param_3 < uVar6 || (param_3 <= uVar10)))) {
      pass1_1028_6356(uVar9,0x0,param_3,param_3);
      goto LAB_1028_c0d6;
    }
    pass1_1028_6356(uVar9,0x0,uVar10,uVar6);
    param_3 -= uVar10;
    uStack26 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c0f0(mut param_1: i16,param_2: *mut astruct_15,i32 param_3)

{
  astruct_691 *paVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u8 *puVar5;
  u8 *extraout_DX_00;
  let mut extraout_DX_01: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  u8 *extraout_DX_02;
  let mut uVar8: u16;
  u32 *puVar9;
  let mut uVar10: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  u32 *puStack20;
  astruct_398 *paStack6;

  pass1_1028_b58e(param_2);
  paStack6 = (astruct_398 *)CONCAT22(extraout_DX,param_1);
  paVar1 = *(astruct_691 **)(param_1 + 0x2e);
  pass1_1028_cb04(param_2);
  uVar8 = ((u32)paVar1 >> 0x10);
  if ((((int)paVar1 + 0x204) == 0x0) && (((int)paVar1 + 0x206) == 0x0)) {
    puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
    puVar5 = ((u32)puVar9 >> 0x10);
    uVar3 = puVar9;
    uVar8 = SUB42(&u16_1050_1038,0x0);
    pass1_1038_4d6e(uVar3,puVar5,paVar1,puVar9);
    puStack20 = (u32 *)CONCAT22(puVar5,uVar3);
    ppcVar2 = (code **)((int)*puStack20 + 0x10);
    uVar6 = uVar3;
    (**ppcVar2)((int)&u16_1050_1038,uVar3,puVar5);
    uStack24 = CONCAT22(extraout_DX_00,uVar6);
    puVar7 = extraout_DX_00;
    for (uStack28 = 0x0; uStack28 < uStack24; uStack28 += 0x1) {
      ppcVar2 = (code **)((int)*puStack20 + 0x4);
      uVar10 = uStack24;
      (**ppcVar2)((char)uVar8,uVar3,puVar5,(char)uStack28,(int)(uStack28 >> 0x10));
      uVar4 = uVar10;
      uVar6 = extraout_DX_01;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | (u32)extraout_DX_01 << 0x10);
      uVar8 = 0x1030;
      uVar10 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar6,uVar4),uVar4,uVar6);
      uVar10 = pass1_1028_6302(uVar10);
      puVar7 = (uVar10 >> 0x10);
      uVar6 = uVar10;
      if ((param_3 <= puVar7) && ((param_3 < puVar7 || (param_3 <= uVar6)))) {
        param_3 = 0x0;
        break;
      }
      param_3 = CONCAT22(param_3 + (-(param_3 < uVar6) - (int)puVar7),param_3 - uVar6);
    }
    if (puStack20 != NULL) {
      ppcVar2 = (code **)*puStack20;
      (**ppcVar2)(uVar8,uVar3,(char)puVar5,0x1);
      puVar7 = extraout_DX_02;
    }
    if (param_3 != 0x0) {
      pass1_1030_7d7c(puStack20,puVar7,paStack6,param_3,CONCAT22(0x1d,(int)((u32)param_3 >> 0x10)));
    }
  }
  return;
}
pub fn pass1_1028_c1f8(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15,param_4: *mut u16,param_5: *mut u16)

{
  u32 *puVar1;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_baf6(param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = (u32 *)pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = ((int)puVar1 + 0x4);
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),param_4,(char *)param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c23e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,mut param_6: u32,i32 param_7)

{
  let mut uVar1: u32;
  code **ppcVar2;
  u32 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut extraout_DX: u16;
  u32 *puStack22;
  astruct_294 *paStack10;
  let mut uStack6: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_7);
  uStack6 = CONCAT22(param_2,param_1);
  uVar7 = param_2 | param_1;
  if (uVar7 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    paStack10 = (astruct_294 *)CONCAT22(uVar7,param_1);
    uVar1 = (u32)(param_1 + 0x2a);
    if (uVar1 != param_6) {
      uVar6 = param_6;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
      uVar4 = uVar6;
      puVar3 = (u32 *)(uVar6 & 0xffff | (u32)uVar7 << 0x10);
      uVar8 = uVar7;
      uVar5 = uVar4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
      puStack22 = (u32 *)CONCAT22(uVar8,uVar5);
      if (((puVar3 == NULL) || ((uVar8 | uVar5) == 0x0)) || (*(i32 *)(uVar5 + 0x200) != *(i32 *)(uVar4 + 0x200))) {
        return;
      }
      ppcVar2 = (code **)((int)*puVar3 + 0x18);
      (**ppcVar2)(0x1030,uVar4,uVar7,uStack6);
      ppcVar2 = (code **)((int)*puStack22 + 0x8);
      (**ppcVar2)();
      pass1_1030_73ee(extraout_DX,paStack10,param_6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1028_c314(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,mut param_6: u16 ,
                      mut param_7: u16 ,mut param_8: u32)

{
  u32 *puVar1;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut local_e: i16;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_8);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = (u32 *)pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = ((int)puVar1 + 0x4);
  pass1_1008_3e94(param_5,(u16 *)CONCAT22(0x1050,&local_10),(char *)CONCAT22(0x1050,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_14),(char *)CONCAT22(0x1050,&local_12))
  ;
  if ((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) && (local_10 < local_14 + -0x1)) {
    return 0x1;
  }
  PTR_LOOP_1050_50ca = 0x6b8;
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c3aa(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,mut param_5: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  let mut uVar8: u32;
  let mut uVar9: u32;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar12;
  let mut uVar13: u32;
  let mut uVar14: u16;
  let mut uVar15: u32;
  u32 *puVar16;
  u32 *puVar17;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut puVar18: *mut u16;
  u8 uVar19;
  u8 uVar20;
  let mut uVar21: u16;
  let mut uVar22: u16;
  let mut uStack40: u32;
  let mut uStack36: u32;
  u32 *puStack32;
  u8 *puStack24;
  u8 local_4 [0x2];

  uVar10 = ((u32)in_EDX >> 0x10);
  uVar15 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  paVar12 = (astruct_57 *)CONCAT22(uVar10,(int)(uVar15 >> 0x10));
  iVar2 = (int)uVar15;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar10 = SUB42(paVar12,0x0);
  uVar8 = (u32)(iVar2 + 0x10);
  uVar19 = SUB41(param_3,0x0);
  uVar20 = (u8)((u32)param_3 >> 0x8);
  uVar14 = ((u32)param_3 >> 0x10);
  uVar15 = param_5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8);
  puStack24 = local_4;
  pass1_1030_bcde(puStack24,&DAT_1050_1050,uVar8 & 0xffff | (long)paVar12 << 0x10,
                  (u16 *)CONCAT22(uVar14,CONCAT11(uVar20,uVar19)),uVar15);
  if ((int)puStack24 < 0x0) {
    PTR_LOOP_1050_50ca = 0x6af;
    return;
  }
  if (0x1e < (int)puStack24) {
    uVar3 = 0x87;
    puVar16 = mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)0x870009,in_stack_0000fe72,in_stack_0000ff96,
                              in_stack_0000ff9c,in_stack_0000ffa0);
    uVar3 = pass1_1010_65d0((u32)puVar16,uVar3);
    if (uVar3 == 0x0) {
      puVar17 = pass1_1008_c6fa(_u16_1050_06e0,0x15);
      uVar15 = (u32)puVar17 >> 0x10;
      uVar4 = puVar17;
      uVar14 = SUB42(&u16_1050_1038,0x0);
      pass1_1038_4d6e(uVar4,((u32)puVar17 >> 0x10),(astruct_691 *)CONCAT22(uVar10,iVar2),puVar17);
      uVar10 = uVar15;
      puStack32 = (u32 *)CONCAT22(uVar10,uVar4);
      ppcVar1 = (code **)((int)*puStack32 + 0x10);
      uVar13 = uVar15;
      uVar5 = uVar4;
      uVar22 = uVar4;
      (**ppcVar1)((int)&u16_1050_1038,uVar4,uVar10);
      uStack36 = CONCAT22((int)uVar13,uVar5);
      uStack40 = 0x0;
      while( true ) {
        if (uStack36 <= uStack40) {
          if (puStack32 != NULL) {
            ppcVar1 = (code **)*puStack32;
            (**ppcVar1)(uVar14,uVar4,(char)uVar15,0x1,uVar22,uVar10,puStack32,puStack32);
          }
          PTR_LOOP_1050_50ca = 0x6b6;
          PTR_LOOP_1050_50cc = puStack24 + -0x1e;
          return;
        }
        uVar19 = (u8)param_5;
        uVar20 = (u8)(param_5 >> 0x8);
        uVar9 = uStack36;
        puVar18 = param_3;
        uVar21 = (int)(param_5 >> 0x10);
        pass1_1030_1d58((u32)puStack32);
        uVar6 = uVar9;
        uVar11 = uVar13;
        puVar7 = local_4;
        uVar14 = 0x1030;
        pass1_1030_bcde(puVar7,&DAT_1050_1050,uVar9 & 0xffff | uVar13 << 0x10,puVar18,
                        CONCAT22(uVar21,CONCAT11(uVar20,uVar19)));
        if ((0x0 < (int)puVar7) && ((int)puVar7 < 0x1f)) break;
        if ((int)puVar7 < (int)puStack24) {
          puStack24 = puVar7;
        }
        uStack40 += 0x1;
      }
      if (puStack32 == NULL) {
        return;
      }
      ppcVar1 = (code **)*puStack32;
      (**ppcVar1)(0x1030,uVar4,(char)uVar15,0x1,uVar22,uVar10,puStack32,puStack32,uVar6,uVar11);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c522(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,i32 param_5)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  u8 local_4 [0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (u32)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | (u32)uVar4 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = 0x6af;
  }
  else {
    if ((int)puVar2 < 0x1f) {
      return;
    }
    PTR_LOOP_1050_50ca = 0x6b6;
    PTR_LOOP_1050_50cc = puVar2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1028_c5a6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16,i32 param_7)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iStack14: i16;
  astruct_419 *paStack10;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    paStack10 = (astruct_419 *)CONCAT22(uVar2,param_1);
    iVar1 = ((int)param_6 + 0x4);
    iStack14 = 0x7a;
    if (0x0 < iVar1) {
      iVar1 = param_5 + -0x7b;
      if (iVar1 == 0x0) {
        param_5 = 0x7e;
      }
      else {
        iVar1 = param_5 + -0x7c;
        if (iVar1 == 0x0) {
          param_5 = 0x7d;
        }
      }
      iStack14 = 0x7f;
    }
    if (paStack10 != NULL) {
      uVar3 = struct_op_1030_73a8(paStack10,iVar1,uVar2);
      if ((uVar3 != 0x0) && ((iVar1 = ((int)uVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_5)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

BOOL16 pass1_1028_c64a(mut param_1: u32,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,i32 param_6)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 local_e [0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (u32)(local_c - 0x1) << 0x10;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
  if (BVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (u32)(local_c + 0x1) << 0x10;
    BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
    if (BVar1 == 0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8 = local_c;
      BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7c,(u16 *)CONCAT22(0x1050,&local_8),param_6);
      if (BVar1 == 0x0) {
        local_8 = CONCAT22(local_8,local_a + 0x1);
        BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7c,(u16 *)CONCAT22(0x1050,&local_8),param_6);
        if (BVar1 == 0x0) {
          return BVar1;
        }
      }
    }
  }
  return 0x1;
}
pub fn pass1_1028_c724(param_1: *mut astruct_295)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  astruct_295 *iVar3;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_295 *)param_1;
  uVar2 = iVar3->field20_0x14;
  if (((int)uVar2 + 0xac) != 0x0) {
    return;
  }
  uVar2 = iVar3->field20_0x14;
  uVar1 = ((int)uVar2 + 0xa6);
  if (uVar1 == 0xd) {
    uVar2 = iVar3->field20_0x14;
    ((int)uVar2 + 0xac) = 0x1;
    goto LAB_1028_c770;
  }
  if (uVar1 < 0xe) {
    if ((char)uVar1 == '\0') goto LAB_1028_c770;
    if ((char)uVar1 == '\a') {
      uVar2 = iVar3->field20_0x14;
      ((int)uVar2 + 0xac) = 0xa;
      goto LAB_1028_c770;
    }
  }
  uVar2 = iVar3->field20_0x14;
  ((int)uVar2 + 0xac) = 0x5;//
LAB_1028_c770:
  uVar2 = iVar3->field20_0x14;
  if (((int)uVar2 + 0xac) == 0x0) {
    uVar2 = iVar3->field20_0x14;
    if (((int)uVar2 + 0xa8) != 0x0) {
      uVar2 = iVar3->field20_0x14;
      ((int)uVar2 + 0xac) = 0x1;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c7b6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,i32 param_5)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  u8 bStack27;
  let mut local_a: u32;
  let mut uStack6: u32;

  puVar1 = &local_a;
  pass1_1030_64ce(puVar1,param_1,_PTR_LOOP_1050_5740,param_4,param_5,(u32 *)CONCAT22(0x1050,puVar1));
  uStack6 = *puVar1;
  uVar3 = ((int)puVar1 + 0x2);
  bStack27 = (u8)(uStack6 >> 0x18);
  uVar2 = bStack27;
  if (bStack27 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uVar3 << 0x10);
  uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar2),uVar2,uVar3);
  uVar3 = (uVar4 >> 0x10);
  if ((uVar3 | uVar4) != 0x0) {
    switch((uVar4 + 0xc)) {
    case 0x1:
      break;
    case 0x2:
      break;
    case 0x3:
      break;
    case 0x4:
      break;
    case 0x5:
      break;
    case 0x6:
      break;
    case 0x7:
      return;
    case 0x8:
      return;
    case 0x9:
      return;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c89c(mut param_1: i16,param_2: *mut astruct_15,param_3: *mut u16,u32 *param_4)

{
  u32 *puVar1;
  let mut extraout_DX: u16;
  let mut uVar2: u16;
  u32 local_16 [0x3];
  i32 lStack10;
  let mut uStack6: u32;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(extraout_DX,param_1);
  lStack10 = *(i32 *)(param_1 + 0x8);
  puVar1 = local_16;
  uVar2 = extraout_DX;
  pass1_1030_64ce(puVar1,extraout_DX,_PTR_LOOP_1050_5740,param_3,lStack10,(u32 *)CONCAT22(0x1050,puVar1));
  *param_4 = *puVar1;
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1028_c8ee(param_1: *mut astruct_15,mut param_2: i16,param_3: *mut u16)

{
  let mut local_8: u16;
  let mut local_6: u32;

  pass1_1008_3eb4((astruct_615 *)param_3,(u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  if (param_2 == 0x1) {
    local_8 += 0x1;
  }
  else if (param_2 == 0x2) {
    local_6 = local_6 & 0xffff0000 | (u32)((int)local_6 - 0x1);
  }
  else if (param_2 == 0x3) {
    local_6 = local_6 & 0xffff0000 | (u32)((int)local_6 + 0x1);
  }
  else if (param_2 == 0x4) {
    local_6 = local_6 & 0xffff | (u32)(local_6 + 0x1) << 0x10;
  }
  else if (param_2 == 0x5) {
    local_6 = local_6 & 0xffff | (u32)(local_6 - 0x1) << 0x10;
  }
  pass1_1008_3e76(param_3,local_8,local_6,(local_6 >> 0x10));
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c952(param_1: *mut astruct_15)

{
  let mut uVar2: u32;
  astruct_600 *uVar3;
  let mut BVar3: bool;
  let mut uVar4: u16;
  StructD *pSVar5;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  astruct_15 *pstruct15_9;
  astruct_15 *uVar10;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack30: u32;
  let mut uStack16: u16;
  let mut uStack14: u16;
  StructD *uVar1;

  uVar10 = (astruct_15 *)((u32)param_1 >> 0x10);
  pstruct15_9 = (astruct_15 *)param_1;
  uVar1 = pstruct15_9->field16_0x14;
  uVar3 = (astruct_600 *)uVar1;
  uVar7 = ((int)&pstruct15_9->field16_0x14 + 0x2) | uVar3;
  if (uVar7 != 0x0) {
    pSVar5 = uVar1;
    pass1_1028_b58e(param_1);
    uVar2 = (u32)((int)pSVar5 + 0x2e);
    uStack14 = uVar2;
    if (((((int)pSVar5 + 0x30) | uStack14) != 0x0) &&
       (uVar11 = (uVar2 >> 0x10), (uStack14 + 0x206) == 0x0)) {
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,pstruct15_9->field10_0xc,0x32);
      if (BVar3 == 0x0) {
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0,pstruct15_9->field10_0xc,0x33);
        if ((BVar3 != 0x0) && ((int)((qword)*_PTR_LOOP_1050_65e2 % 0x5) == 0x0)) {
          return;
        }
      }
      else if ((int)((qword)*_PTR_LOOP_1050_65e2 % 0xa) == 0x0) {
        return;
      }
      uVar12 = ((u32)uVar1 >> 0x10);
      if ((uStack14 + 0x204) == 0x0) {
        for (uStack16 = 0x0; (int)uStack16 < 0x25; uStack16 += 0x1) {
          uStack30 = (u32)(&uVar3->field_0x0 + uStack16 * 0x4);
          uVar7 = uStack30;
          uVar9 = (&uVar3->field_0x2 + uStack16 * 0x4) | uVar7;
          if (uVar9 != 0x0) {
            uVar6 = uStack30;
            empty_1038_540a();
            uStack30 = (uStack30 >> 0x10);
            if ((uVar6 & 0xffff | (u32)uVar9 << 0x10) < uStack30) {
              uVar4 = uVar7 - uVar6;
              iVar8 = (uStack30 - uVar9) - (uVar7 < uVar6);
              pass1_1038_52b8(uVar2,CONCAT22(iVar8,uVar4),0x21);
              uStack30 = CONCAT22((uStack30 - iVar8) - (uVar7 < uVar4),uVar7 - uVar4);
            }
            if ((uStack30 | uStack30) != 0x0) {
              pass1_1038_52b8(uVar2,uStack30,uStack16);
            }
          }
        }
      }
      else {
        uVar7 = uVar3->field140_0x8c;
        uVar9 = uVar3->field141_0x8e;
        if ((uVar9 | uVar7) != 0x0) {
          pass1_1038_52b8(uVar2,CONCAT22(uVar9,uVar7),0x23);
        }
        uVar7 = uVar3->field142_0x90;
        uVar9 = uVar3->field143_0x92;
        if ((uVar9 | uVar7) != 0x0) {
          pass1_1038_52b8(uVar2,CONCAT22(uVar9,uVar7),0x24);
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_cb04(param_1: *mut astruct_15)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  astruct_398 *paVar3;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u32;
  i32 lVar7;
  u8 *puVar8;
  let mut in_EDX: u32;
  astruct_57 *paVar9;
  let mut iVar10: i16;
  let mut unaff_SI: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut bVar13: bool;
  astruct_27 *paVar14;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  u8 *puStack52;
  let mut uStack38: u16;
  u8 *puStack36;
  let mut iStack22: i16;
  let mut uStack18: u16;
  u8 *puStack16;
  let mut uStack14: u16;

  uVar1 = (u32)((int)param_1 + 0x14);
  if (uVar1 != 0x0) {
    uVar5 = uVar1;
    pass1_1028_b58e(param_1);
    paVar3 = (astruct_398 *)(uVar5 & 0xffff | in_EDX << 0x10);
    uVar2 = (u32)((int)uVar5 + 0x2e);
    uStack18 = ((int)uVar5 + 0x30);
    paVar9 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uStack18);
    uStack14 = uVar2;
    uStack18 |= uStack14;
    if (uStack18 != 0x0) {
      uVar11 = ((u32)uVar2 >> 0x10);
      if ((uStack14 + 0x206) != 0x0) {
        return;
      }
      iVar10 = (int)uVar1;
      uVar12 = (uVar1 >> 0x10);
      if ((uStack14 + 0x204) != 0x0) {
        uVar2 = (u32)(iVar10 + 0x8c);
        uVar6 = uVar2;
        empty_1038_540a();
        puStack36 = ((u32)uVar2 >> 0x10);
        puVar8 = paVar9;
        if ((puVar8 <= puStack36) &&
           ((uVar4 = uVar6, uStack38 = uVar2, puVar8 < puStack36 || (uVar4 < uStack38)))) {
          pass1_1030_7d7c(uVar4,puVar8,paVar3,uStack38 - uVar4,
                          CONCAT22(0x23,puStack36 + (-(uStack38 < uVar4) - (int)puVar8)));
          paVar14 = (astruct_27 *)
                    mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe70,
                                    in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
          paVar9 = (astruct_57 *)((u32)paVar14 >> 0x10);
          pass1_1010_043a(paVar14,*(i32 *)(uStack14 + 0x4),0x12);
        }
        puVar8 = paVar9;
        uVar2 = (u32)(iVar10 + 0x90);
        uVar6 = uVar2;
        empty_1038_540a();
        puStack36 = ((u32)uVar2 >> 0x10);
        if ((puVar8 <= puStack36) &&
           ((uVar4 = uVar6, uStack38 = uVar2, puVar8 < puStack36 || (uVar4 < uStack38)))) {
          pass1_1030_7d7c(uVar4,puVar8,paVar3,uStack38 - uVar4,
                          CONCAT22(0x24,puStack36 + (-(uStack38 < uVar4) - (int)puVar8)));
        }
        return;
      }
      empty_1038_540a();
      puStack16 = paVar9;
      for (iStack22 = 0x11; iStack22 < 0x25; iStack22 += 0x1) {
        uVar1 = (u32)(iStack22 * 0x4 + iVar10);
        uVar5 = uVar1;
        empty_1038_540a();
        uVar5 = uVar5 & 0xffff | (long)paVar9 << 0x10;
        puStack52 = (uVar1 >> 0x10);
        paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | ZEXT24(puStack52));
        if (uVar5 < uVar1) {
          if ((((iStack22 == 0x23) || (iStack22 == 0x24)) || (puStack16 < puStack52)) ||
             ((uVar4 = uVar1, puStack16 <= puStack52 && (uStack18 < uVar4)))) {
            lVar7 = uVar1 - uVar5;
            uVar4 = lVar7;
            pass1_1030_7d7c(uVar4,puStack52,paVar3,uVar4,CONCAT22(iStack22,(int)((u32)lVar7 >> 0x10)));
            if (iStack22 == 0x23) {
              paVar14 = (astruct_27 *)
                        mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe70,
                                        in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
              paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)paVar14 >> 0x10);
              pass1_1010_043a(paVar14,*(i32 *)(uStack14 + 0x4),0x12);
            }
          }
          else {
            bVar13 = uStack18 < uVar4;
            uStack18 -= uVar4;
            puStack16 = puStack16 + (-bVar13 - (int)puStack52);
          }
        }
      }
      return;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_ccd0(param_1: *mut astruct_15,param_2: *mut u16)

{
  code **ppcVar1;
  let mut puVar2: *mut u16;
  u8 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  astruct_57 *in_EDX;
  let mut in_stack_0000fd28: u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000fe52: u16;
  let mut in_stack_0000fe56: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000fe80: u16;
  let mut local_178: u16;
  let mut uStack374: u16;
  let mut iStack84: i16;
  let mut uStack72: u16;
  let mut uStack64: u16;
  let mut iStack62: i16;
  let mut uStack60: u32;
  u32 *puStack56;
  let mut uStack52: u32;
  u32 *puStack48;
  u8 local_2c [0xc];
  let mut local_20: i16;
  let mut local_1e: i16;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut local_8: u16;
  let mut local_6: i16;
  let mut local_4: i16;

  puVar2 = &local_8;
  pass1_1008_3eb4((astruct_615 *)param_2,(u16 *)CONCAT22(0x1050,puVar2),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,&local_4));
  pass1_1028_b58e(param_1);
  uVar7 = in_EDX;
  uStack20 = CONCAT22(uVar7,puVar2);
  uStack24 = (u32)(puVar2 + 0x17);
  uStack28 = (u32)((int)uStack24 + 0x4);
  pass1_1028_c1f8((int)&local_20,uVar7,param_1,(u16 *)CONCAT22(0x1050,&local_20),(u16 *)CONCAT22(0x1050,&local_1e)
                 );
  uStack10 = local_4 - 0x1;
  iStack14 = local_4 + 0x1;
  uStack12 = local_6 - 0x1;
  iStack16 = local_6 + 0x1;
  if ((int)uStack10 < 0x0) {
    uStack10 = 0x0;
  }
  if (local_1e <= iStack14) {
    iStack14 = local_1e + -0x1;
  }
  if ((int)uStack12 < 0x0) {
    uStack12 = 0x0;
  }
  if (local_20 <= iStack16) {
    iStack16 = local_20 + -0x1;
  }
  pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_2c));
  pass1_1008_6cec((u16 *)CONCAT22(0x1050,local_2c),local_8,CONCAT22(iStack14,iStack16),local_8,
                  CONCAT22(uStack10,uStack12));
  puStack48 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe80,0x2f),in_stack_0000fd28,
                              in_stack_0000fe4c,in_stack_0000fe52,in_stack_0000fe56);
  uVar5 = ((u32)puStack48 >> 0x10);
  uStack52 = (u32)((int)puStack48 + 0x20);
  puVar3 = local_2c;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),uStack52);
  puStack56 = (u32 *)CONCAT22(uVar5,puVar3);
  if ((uVar5 | puVar3) != 0x0) {
    uStack60 = 0x0;
    iStack62 = 0x0;
    for (uStack64 = uStack12; (int)uStack64 <= iStack16; uStack64 += 0x1) {
      for (uStack72 = uStack10; iVar4 = iStack62, (int)uStack72 <= iStack14; uStack72 += 0x1) {
        iVar6 = iStack62 >> 0xf;
        ppcVar1 = (code **)((int)*puStack56 + 0x4);
        iStack62 = iStack62 + 0x1;
        (**ppcVar1)(0x1030,(int)puStack56,(int)((u32)puStack56 >> 0x10),iVar4,iVar6);
        uStack60 = CONCAT22(iVar6,iVar4);
        uStack60._3_1_ = (char)(iVar6 >> 0x8);
        if (uStack60._3_1_ == '\0') {
          iStack84 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_2,local_8,uStack64,uStack72);
            uVar10 = uStack52;
            uVar11 = (uStack52 >> 0x10);
            uVar8 = uStack28;
            uVar9 = ((u32)uStack28 >> 0x10);
            uVar7 = 0x6;
          }
          else if (iVar4 == 0x8) {
            pass1_1008_3e76(param_2,local_8,uStack64,uStack72);
            uVar10 = uStack52;
            uVar11 = (uStack52 >> 0x10);
            uVar8 = uStack28;
            uVar9 = ((u32)uStack28 >> 0x10);
            uVar7 = 0x7;
          }
          else {
            if (iVar4 != 0x9) goto LAB_1028_ce2c;
            pass1_1008_3e76(param_2,local_8,uStack64,uStack72);
            uVar10 = uStack52;
            uVar11 = (uStack52 >> 0x10);
            uVar8 = uStack28;
            uVar9 = ((u32)uStack28 >> 0x10);
            uVar7 = 0x8;
          }
          struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_178),0x0,0x0,uVar7,(u32 *)param_2,
                              ((u32)param_2 >> 0x10),CONCAT22(uVar9,uVar8),CONCAT22(uVar11,uVar10));
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_178));
          local_178 = 0x389a;
          uStack374 = 0x1008;
        }//
LAB_1028_ce2c:
      }
    }
  }
  return;
}



u16 pass1_1028_ced2(param_1: *mut astruct_15)

{
  astruct_15 *uVar1;
  let mut bVar1: bool;
  let mut bVar2: bool;
  astruct_398 *paVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;

  uVar1 = (astruct_15 *)((u32)param_1 >> 0x10);
  bVar1 = (*((int)param_1 + 0x1a) & 0x2) == 0x0;
  if (bVar1) {
    uVar5 = 0x0;
    uVar6 = 0x23;
    uVar4 = 0x1;
    paVar3 = (astruct_398 *)pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
    pass1_1030_7d7c(paVar3,((u32)paVar3 >> 0x10),paVar3,uVar4,CONCAT22(uVar6,uVar5));
  }
  bVar2 = (*((int)param_1 + 0x1a) & 0x1) == 0x0;
  if (bVar2) {
    uVar5 = 0x0;
    uVar6 = 0xe;
    uVar4 = 0x1;
    paVar3 = (astruct_398 *)pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
    pass1_1030_7d7c(paVar3,((u32)paVar3 >> 0x10),paVar3,uVar4,CONCAT22(uVar6,uVar5));
  }
  if (bVar2 || bVar1) {
    pass1_1028_bdac(param_1,0x6);
    return 0x0;
  }
  return 0x1;
}



StructD * pass1_1028_cf44(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_cfd2(u32 *param_1,mut param_2: u32)

{
  *param_1 = param_2;
  (u32)((int)param_1 + 0x4) = 0x0;
  return;
}
pub fn pass1_1028_cff2(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  puVar1 = (u32 *)((int)param_1 + 0x4);
  uVar2 = ((int)param_1 + 0x6);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  return;
}
pub fn pass1_1028_d01a(u32 *param_1)

{
  u32 *puVar1;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u32 *puStack14;

  puVar1 = (u32 *)*(u32*)*param_1;
  puStack14 = puVar1;
  while( true ) {
    uVar4 = puStack14;
    fn_ptr_1028_d728((u32)puVar1);
    puStack14 = (u32 *)CONCAT22(extraout_DX,uVar4);
    if ((extraout_DX | uVar4) == 0x0) break;
    uVar3 = *puStack14;
    ppcVar2 = (code **)uVar3 + 0x2;
    (**ppcVar2)();
    if (puStack14 != NULL) {
      ppcVar2 = (code **)uVar3;
      (**ppcVar2)();
    }
  }
  return;
}
