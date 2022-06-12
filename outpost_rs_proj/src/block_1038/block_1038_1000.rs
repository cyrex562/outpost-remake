pub fn pass1_1038_11b0(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,u32 *param_4,u32 *param_5)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_5 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22((int)param_2,param_1);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_5 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar2 = uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | param_2 << 0x10);
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22((int)param_2,uVar2),uVar2,(int)param_2);
    param_2 = param_2 & 0xffff0000 | uVar4 >> 0x10;
    uVar3 = uVar4;
    pass1_1038_0f8c(uVar3,param_2,param_3,(param_3 >> 0x10),param_4,uVar4);
    if (uVar3 == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1220(mut param_1: u32,mut param_2: u32,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u8 *puVar6;
  u8 *puVar7;
  u8 *puVar8;
  let mut uVar10: u16;
  let mut uVar9: u32;
  u32 *puVar11;
  u8 uVar12;
  u32 *puStack14;
  u32 *puStack10;

  uVar10 = (param_1 >> 0x10);
  puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar6 = ((u32)puVar11 >> 0x10);
  uVar3 = puVar11;
  pass1_1038_4d6e(uVar3,puVar6,(astruct_691 *)param_3,puVar11);
  puStack10 = (u32 *)CONCAT22(puVar6,uVar3);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  puVar7 = puVar6;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar6);
  if ((puVar7 != NULL) || (uVar4 != 0x0)) {
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x5);
    puVar8 = ((u32)puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(uVar4,puVar8,(astruct_691 *)param_3,puVar11);
    puStack14 = (u32 *)CONCAT22(puVar8,uVar4);
    uVar12 = (u8)uVar4;
    uVar2 = *puStack14;
    ppcVar1 = (code **)uVar2 + 0x8;
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x1008,uVar12,puVar8);
    uVar9 = CONCAT22(uVar10,puVar7);
    if (((puVar7 != NULL) || (uVar5 != 0x0)) &&
       (pass1_1038_11b0(uVar5,uVar9,param_2,(u32 *)CONCAT13((char)(puVar6 >> 0x8),CONCAT12((char)puVar6,uVar3)),
                        (u32 *)CONCAT22(puVar8,uVar4)), uVar5 == 0x0)) {
      if (puStack14 == NULL) {
        return;
      }
      ppcVar1 = (code **)uVar2;
      (**ppcVar1)(0x8,uVar12,(char)puVar8,0x1);
      return;
    }
    uVar10 = (uVar9 >> 0x10);
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar12,(char)puVar8,0x1);
    }
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x6);
    puVar8 = ((u32)puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(uVar4,puVar8,(astruct_691 *)param_3,puVar11);
    puStack14 = (u32 *)CONCAT22(puVar8,uVar4);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x8,(char)uVar4,puVar8);
    if ((puVar7 != NULL) || (uVar5 != 0x0)) {
      pass1_1038_11b0(uVar5,CONCAT22(uVar10,puVar7),param_2,(u32 *)CONCAT22(puVar6,uVar3),
                      (u32 *)CONCAT22(puVar8,uVar4));
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar4,(char)puVar8,0x1);
    }
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x8,uVar3,(char)puVar6,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_134a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,u32 *param_4,u32 *param_5,u32 *param_6)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  u32 *puVar5;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_6 + 0x10);
  puVar5 = param_6;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  *param_4 = 0x0;
  do {
    if (uStack6 <= *param_5) {
      return;
    }
    uVar4 = *param_5;
    *param_5 = *param_5 + 0x1;
    ppcVar1 = (code **)((int)*param_6 + 0x4);
    (**ppcVar1)(unaff_CS,param_6,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar2),uVar2,uVar3);
    uVar3 = (uVar4 >> 0x10);
    unaff_CS = 0x1028;
    uVar4 = pass1_1028_45e2(uVar4,uVar3,uVar4 & 0xffff | (u32)uVar3 << 0x10);
    uVar3 = (uVar4 >> 0x10);
    param_4 = (int)uVar4;
    ((int)param_4 + 0x2) = uVar3;
  } while ((uVar3 | param_4) == 0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_13da(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,u32 *param_4,u32 *param_5,u32 *param_6)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  u32 *puVar5;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_6 + 0x10);
  puVar5 = param_6;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  *param_4 = 0x0;
  do {
    if (uStack6 <= *param_5) {
      return;
    }
    uVar4 = *param_5;
    *param_5 = *param_5 + 0x1;
    ppcVar1 = (code **)((int)*param_6 + 0x4);
    (**ppcVar1)(unaff_CS,param_6,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
    if ((uVar3 | uVar2) == 0x0) {
      return;
    }
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar2),uVar2,uVar3 | uVar2);
    uVar3 = (uVar4 >> 0x10);
    if ((uVar3 | uVar4) == 0x0) {
      return;
    }
    unaff_CS = 0x1028;
    uVar4 = pass1_1028_3c32((u32 *)(uVar4 & 0xffff | (u32)uVar3 << 0x10));
    uVar3 = (uVar4 >> 0x10);
    param_4 = (int)uVar4;
    ((int)param_4 + 0x2) = uVar3;
  } while ((uVar3 | param_4) == 0x0);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1482(mut param_1: u16 ,mut param_2: u32,u32 *param_3,u32 *param_4)

{
  code **ppcVar1;
  sqword sVar2;
  let mut uVar3: u16;
  u32 *puVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  astruct_57 *paVar11;
  astruct_57 *paVar13;
  astruct_57 *paVar14;
  let mut uVar15: u32;
  let mut unaff_DI: u16;
  let mut uVar16: u16;
  u8 uVar17;
  u8 uVar18;
  i32 lStack74;
  let mut local_46: u32;
  u16 local_42 [0x4];
  let mut uStack58: u16;
  let mut uStack56: u16;
  u32 *puStack54;
  u32 *puStack50;
  let mut uStack46: u32;
  let mut uStack42: u16;
  let mut uStack40: u16;
  let mut uStack38: u16;
  let mut uStack36: u16;
  let mut uStack34: u32;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut local_a: u32;
  let mut local_6: u32;
  astruct_57 *paVar12;

  local_6 = 0x0;
  local_a = 0x0;
  puVar4 = &local_a;
  uVar16 = (param_2 >> 0x10);
  uVar3 = param_2;
  pass1_1038_134a(puVar4,uVar3,uVar16,(u32 *)CONCAT22(0x1050,puVar4),(u32 *)CONCAT22(0x1050,&local_6),param_4);
  uStack14 = (astruct_15 *)CONCAT22(param_1,puVar4);
  ppcVar1 = (code **)((int)*param_3 + 0x10);
  (**ppcVar1)();
  uStack18 = CONCAT22(param_1,puVar4);
  uStack22 = 0x0;
  do {
    if (uStack18 <= uStack22) {
      return;
    }
    uStack14 |= uStack14;
    if (uStack14 == 0x0) {
      return;
    }
    pass1_1028_b58e(uStack14);
    uStack26 = uStack14;
    uStack24 = uStack18;
    pass1_1038_1a30(uVar3,uVar16,CONCAT22(uStack18,uStack14));
    uStack30 = uStack14;
    uStack28 = uStack18;
    if ((uStack18 | uStack14) != 0x0) {
      sVar2 = (qword)CONCAT22(uStack18,uStack14) * 0x64;
      uVar15 = (u32)((qword)sVar2 >> 0x20);
      uVar7 = (u32)sVar2 >> 0x1;
      ppcVar1 = (code **)((int)*param_3 + 0x4);
      uStack34 = uVar7;
      (**ppcVar1)(0x1028,param_3,(char)uStack22,(int)(uStack22 >> 0x10));
      uVar6 = uVar7;
      uStack36 = uVar15;
      uStack38 = uVar6;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff | uVar15 << 0x10);
      uStack40 = uVar15;
      uStack42 = uVar6;
      uStack46 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uStack40,uVar6),uVar6,uStack40);
      paVar14 = (astruct_57 *)(uVar15 & 0xffff0000);
      puStack50 = (u32*)((int)uStack46 + 0x28);
      puStack54 = NULL;
      uStack56 = ((int)puStack50 + 0x4);
      for (uStack58 = 0x0; uVar5 = uStack56, uStack58 < uStack56; uStack58 += 0x1) {
        pass1_1020_bb16(puStack50,(u32 *)CONCAT22(0x1050,&local_46),(u16 *)CONCAT13(0x10,CONCAT12(0x50,local_42)),
                        uStack58);
        if (((local_46 != 0x0) && (0xd < (int)local_42[0])) && ((int)local_42[0] < 0x1d)) {
          uVar15 = local_46;
          uVar7 = local_46;
          if (uStack34 < local_46) {
            uVar15 = uStack34 & 0xffff;
            uVar7 = uStack34;
          }
          paVar11 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | uVar7 >> 0x10);
          uVar5 = uVar15;
          uVar10 = (uVar7 >> 0x10);
          if ((local_a <= uVar10) && ((local_a < uVar10 || (local_a < uVar5)))) {
            paVar11 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)local_a);
            uVar5 = local_a;
          }
          iVar8 = (int)paVar11;
          lStack74 = CONCAT22(iVar8,uVar5);
          uStack34 = CONCAT22(((int)(uStack34 >> 0x10) - iVar8) - (uStack34 < uVar5),uStack34 - uVar5)
          ;
          local_a = CONCAT22((local_a - iVar8) - (local_a < uVar5),local_a - uVar5);
          paVar13 = paVar11;
          if (puStack54 == NULL) {
            paVar14 = paVar11;
            uVar10 = uVar5;
            mem_op_1000_179c(0xa,paVar11);
            uVar9 = paVar14 | uVar10;
            paVar13 = (astruct_57 *)((u32)paVar14 & 0xffff0000);
            paVar12 = (astruct_57 *)((u32)paVar13 | (u32)uVar9);
            if (uVar9 == 0x0) {
              uVar10 = 0x0;
            }
            else {
              pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar14,uVar10),0x5,0x5);
              paVar13 = paVar12;
            }
            puStack54 = (u32 *)CONCAT22((int)paVar13,uVar10);
          }
          pass1_1020_bb8a((i32 *)puStack54,uVar5,(u32)paVar11 & 0xffff | (u32)local_42[0] << 0x10);
          uVar7 = local_46 - lStack74;
          pass1_1020_bb8a((i32 *)puStack50,uVar7,CONCAT22(local_42[0],(int)(uVar7 >> 0x10)));
          paVar14 = paVar13;
          if (local_a == 0x0) {
            pass1_1038_1b3a(uVar7,uVar3,uVar16,(u32)uStack14,puStack54,unaff_DI);
            puStack54 = NULL;
            uVar7 = ZEXT24(&local_a);
            pass1_1038_134a(&local_a,uVar3,uVar16,(u32 *)CONCAT22(0x1050,&local_a),(u32 *)CONCAT22(0x1050,&local_6),
                            param_4);
            uVar5 = uVar7;
            uStack14 = (astruct_15 *)(uVar7 & 0xffff | (long)paVar13 << 0x10);
            uVar10 = paVar13 | uVar5;
            paVar14 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)uVar10);
            if (uVar10 != 0x0) {
              uVar17 = 0x64;
              uVar18 = 0x0;
              uVar6 = 0x0;
              pass1_1028_b58e((astruct_15 *)(uVar7 & 0xffff | (long)paVar13 << 0x10));
              uVar10 = paVar14;
              uStack26 = uVar5;
              uStack24 = uVar10;
              pass1_1038_1a30(uVar3,uVar16,CONCAT22(uVar10,uVar5));
              sVar2 = (qword)CONCAT22(uVar10,uVar5) * (qword)CONCAT22(uVar6,CONCAT11(uVar18,uVar17));
              paVar14 = (astruct_57 *)((qword)sVar2 >> 0x20);
              uVar7 = (u32)sVar2 >> 0x1;
              uStack34 = uVar7;
              uStack30 = uVar5;
              uStack28 = uVar10;
            }
          }
          uVar5 = uVar7;
          if ((uStack34 == 0x0) || (local_a == 0x0)) break;
        }
      }
      if (puStack54 != NULL) {
        pass1_1038_1b3a(uVar5,uVar3,uVar16,(u32)uStack14,puStack54,unaff_DI);
        puStack54 = NULL;
      }
    }
    uStack22 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_16f2(mut param_1: u16 ,mut param_2: u32,u32 *param_3,u32 *param_4)

{
  i32 *plVar1;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  u32 *puVar5;
  let mut iVar6: i16;
  u32 *puVar7;
  u32 *puVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  astruct_57 *paVar13;
  astruct_57 *paVar14;
  astruct_57 *paVar15;
  let mut unaff_CS: u16;
  let mut uVar16: u32;
  i32 lVar17;
  let mut uVar18: u16;
  u32 *puVar19;
  i32 lStack68;
  u32 *puStack56;
  u32 *puStack52;
  i32 *plStack50;
  let mut uStack46: u16;
  let mut uStack42: u32;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut local_a: u32;
  let mut local_6: u32;

  local_6 = 0x0;
  local_a = 0x0;
  puVar7 = &local_a;
  uVar18 = (param_2 >> 0x10);
  uVar4 = param_2;
  pass1_1038_13da(puVar7,uVar4,uVar18,(u32 *)CONCAT22(0x1050,puVar7),(u32 *)CONCAT22(0x1050,&local_6),
                  param_4);
  uStack14 = CONCAT22(param_1,puVar7);
  uVar9 = param_1 | puVar7;
  if (uVar9 != 0x0) {
    ppcVar2 = (code **)((int)*param_3 + 0x10);
    puVar19 = param_3;
    (**ppcVar2)();
    uStack18 = CONCAT22(uVar9,puVar7);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      ppcVar2 = (code **)((int)*param_3 + 0x4);
      uVar16 = uStack18;
      uVar10 = uVar9;
      (**ppcVar2)(unaff_CS,param_3,(char)uStack22,(int)(uStack22 >> 0x10),puVar19);
      iVar6 = (int)uVar16;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar16 & 0xffff | (u32)uVar10 << 0x10);
      unaff_CS = 0x1030;
      uVar16 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar10,iVar6),iVar6,uVar10);
      uVar11 = (uVar16 >> 0x10);
      uVar12 = uVar16;
      pass1_1038_1a30(uVar4,uVar18,CONCAT22(uVar10,iVar6));
      if ((uVar11 | uVar12) != 0x0) {
        uStack42 = (u32)(CONCAT22(uVar11,uVar12) * 0x64) >> 0x1;
        plVar1 = *(i32 **)(iVar6 + 0x22);
        uVar12 = (iVar6 + 0x24);
        paVar13 = (astruct_57 *)(u32)uVar12;
        uStack46 = plVar1;
        if ((uVar12 | uStack46) != 0x0) {
          plStack50 = NULL;
          puVar7 = (u32 *)pass1_1028_0d80(uVar16);
          puStack56 = NULL;
          puStack52 = puVar7;
          while( true ) {
            lVar17 = pass1_1020_bae6(puStack52,paVar13,uStack46,
                                     CONCAT22(puStack52,(int)((u32)plVar1 >> 0x10)));
            uVar3 = (u32)paVar13 & 0xffff0000;
            puVar8 = (u32 *)lVar17;
            uVar12 = ((u32)lVar17 >> 0x10);
            paVar13 = (astruct_57 *)(uVar3 | (uVar12 | puVar8));
            if ((uVar12 | puVar8) != 0x0) {
              paVar14 = (astruct_57 *)(uVar3 | uVar12);
              if ((uStack42 <= uVar12) && ((uStack42 < uVar12 || ((u32 *)uStack42 < puVar8)))) {
                paVar14 = (astruct_57 *)(uVar3 | uStack42);
                puVar8 = (u32 *)uStack42;
              }
              if ((local_a <= paVar14) &&
                 ((local_a < paVar14 || ((u32 *)local_a < puVar8)))) {
                paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)local_a);
                puVar8 = (u32 *)local_a;
              }
              iVar6 = (int)paVar14;
              lStack68 = CONCAT22(iVar6,puVar8);
              uStack42 = CONCAT22((uStack42 - iVar6) - ((u32 *)uStack42 < puVar8),
                                  (int)(u32 *)uStack42 - (int)puVar8);
              local_a = CONCAT22((local_a - iVar6) - ((u32 *)local_a < puVar8),
                                 (int)(u32 *)local_a - (int)puVar8);
              paVar13 = paVar14;
              if (plStack50 == NULL) {
                paVar15 = paVar14;
                puVar5 = puVar8;
                mem_op_1000_179c(0xa,paVar14);
                uVar12 = paVar15 | puVar5;
                paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)uVar12);
                if (uVar12 == 0x0) {
                  puVar5 = NULL;
                  paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000);
                }
                else {
                  pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar15,puVar5),0x5,0x5);
                }
                plStack50 = (i32 *)CONCAT22((int)paVar13,puVar5);
              }
              pass1_1020_bb8a(plStack50,puVar8,(u32)paVar14 & 0xffff | ZEXT24(puStack52) << 0x10);
              pass1_1020_bb8a(plVar1,(lVar17 - lStack68),
                              CONCAT22(puStack52,(int)((u32)(lVar17 - lStack68) >> 0x10)));
              puStack56 = puStack52;
              puVar8 = puStack52;
              if (local_a == 0x0) {
                pass1_1038_1ac6((int)puStack52,uVar4,uVar18,uStack14,(u32)plStack50);
                plStack50 = NULL;
                puVar8 = &local_a;
                pass1_1038_13da(puVar8,uVar4,uVar18,(u32 *)CONCAT22(0x1050,puVar8),
                                (u32 *)CONCAT22(0x1050,&local_6),param_4);
                uStack14 = CONCAT22(paVar13,puVar8);
                uVar12 = paVar13 | puVar8;
                paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)uVar12);
                if (uVar12 == 0x0) {
                  return;
                }
              }
            }
            unaff_CS = 0x1020;
            if ((uStack42 == 0x0) || (local_a == 0x0)) break;
            unaff_CS = 0x1028;
            puVar8 = (u32 *)pass1_1028_0d80(uVar16);
            if ((puVar8 == puStack56) || ((puStack52 = puVar8, puStack56 == NULL && (puVar8 == puVar7)))) break;
          }
          if (plStack50 != NULL) {
            pass1_1038_1ac6((int)puVar8,uVar4,uVar18,uStack14,(u32)plStack50);
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1940(mut param_1: u32,u32 *param_2,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut extraout_DX: u16;
  u32 *puVar5;
  u32 *puStack10;

  puVar5 = pass1_1008_c6fa(_u16_1050_06e0,0x3);
  puVar4 = ((u32)puVar5 >> 0x10);
  uVar2 = puVar5;
  pass1_1038_4d6e(uVar2,puVar4,(astruct_691 *)param_3,puVar5);
  puStack10 = (u32 *)CONCAT22(puVar4,uVar2);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar4);
  if ((extraout_DX | uVar3) != 0x0) {
    pass1_1038_1482(extraout_DX | uVar3,param_1,param_2,puStack10);
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x1008,uVar2,(char)puVar4,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_19a0(mut param_1: u32,u32 *param_2,mut param_3: u32,mut param_4: u16 ,undefined1 param_5)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 *puVar5;
  let mut extraout_DX: u16;
  u32 *puVar6;
  u32 *puStack10;

  puVar6 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
  puVar5 = ((u32)puVar6 >> 0x10);
  uVar3 = puVar6;
  pass1_1038_4d6e(uVar3,puVar5,(astruct_691 *)param_3,puVar6);
  puStack10 = (u32 *)CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar1 = (code **)uVar2 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar5);
  if ((extraout_DX | uVar4) == 0x0) {
    vsprintf_op_1030_840a(0x0,(u32)s_mineToSmelter__no_mines_1050_59df);
    if (puStack10 != NULL) {
      ppcVar1 = (code **)uVar2;
      (**ppcVar1)(0x1030,uVar3,(char)puVar5,0x1);
      return;
    }
  }
  else {
    pass1_1038_16f2(extraout_DX | uVar4,param_1,puStack10,param_2);
    if (puStack10 != NULL) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(0x1008,uVar3,(char)puVar5,0x1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1a30(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  u32 *puVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut unaff_CS: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uStack18: u32;
  let mut uStack10: u32;
  let mut uStack6: u16;

  uVar5 = (param_3 >> 0x10);
  puVar1 = (u32 *)(u32)((int)param_3 + 0x1e);
  uVar7 = ((int)param_3 + 0x20);
  uStack6 = puVar1;
  uVar3 = uVar7 | uStack6;
  if (uVar3 != 0x0) {
    ppcVar2 = (code **)((int)*puVar1 + 0x10);
    uVar6 = uStack6;
    (**ppcVar2)();
    uStack10 = CONCAT22(extraout_DX,uVar3);
    for (uStack18 = 0x0; uStack18 < uStack10; uStack18 += 0x1) {
      ppcVar2 = (code **)((int)*puVar1 + 0x4);
      uVar4 = uStack10;
      (**ppcVar2)(unaff_CS,uStack6,(int)((u32)puVar1 >> 0x10),uStack18,uVar6,uVar7);
      if ((extraout_DX_00 | uVar4) != 0x0) {
        unaff_CS = 0x1028;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
      }
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1ac6(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  let mut extraout_DX: u16;
  u8 local_118 [0x112];
  let mut uStack6: u32;

  pass1_1028_b58e((astruct_15 *)param_4);
  uStack6 = CONCAT22(extraout_DX,param_1);
  pass1_1030_e8a0((astruct_97 *)CONCAT22(0x1050,local_118),param_5,((int)param_4 + 0xc),
                  (u32)(param_1 + 0x4));
  pass1_1028_d52c(*_u16_1050_5748,*_PTR_LOOP_1050_65e2 + 0x1,(u32 *)CONCAT22(0x1050,local_118));
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_1b3a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,u32 *param_5,mut param_6: u16 )

{
  let mut in_EDX: u32;
  let mut uVar2: u16;
  astruct_57 *paVar1;
  let mut local_1a: u32;
  u16 local_16 [0x2];
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pass1_1028_b58e((astruct_15 *)param_4);
  uStack6 = CONCAT22((int)in_EDX,param_1);
  uStack10 = param_4;
  uStack14 = pass1_1028_45e2(param_4,(int)in_EDX,param_4);
  paVar1 = (astruct_57 *)(in_EDX & 0xffff0000);
  uStack16 = ((int)param_5 + 0x4);
  for (uStack18 = 0x0; uVar2 = ((u32)paVar1 >> 0x10), uStack18 < uStack16; uStack18 += 0x1) {
    pass1_1020_bb16(param_5,(u32 *)CONCAT22(0x1050,&local_1a),(u16 *)CONCAT22(0x1050,local_16),uStack18);
    paVar1 = (astruct_57 *)CONCAT22(uVar2,uStack14);
    if (uStack14 < local_1a) {
      pass1_1030_7ddc(uStack14,paVar1,uStack6,uStack14,local_16[0]);
      uStack14 = 0x0;
    }
    else {
      uStack14 -= local_1a;
      pass1_1030_7ddc(local_1a,paVar1,uStack6,local_1a,local_16[0]);
    }
    if (uStack14 == 0x0) break;
  }
  if (param_5 != NULL) {
    fn_ptr_1020_ba7e(param_5);
    fn_ptr_1000_17ce((char *)param_5);
  }
  return;
}



StructD * pass1_1038_1c02(StructD *param_1,param_2: u8)

{
  param_1.address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1c3e(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;
  u32 *puVar2;
  code **ppcVar3;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut BVar7: bool;
  u32 *puVar8;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut unaff_CS: u16;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  astruct_419 *paStack26;
  let mut uStack14: u32;

  uVar10 = (param_2 >> 0x10);
  puVar2 = (u32 *)(u32)((int)param_2 + 0xc);
  uVar10 = ((int)param_2 + 0xe);
  ppcVar3 = (code **)((int)*puVar2 + 0x10);
  puVar8 = puVar2;
  uVar14 = (int)puVar2;
  (**ppcVar3)();
  uVar4 = (u32)puVar8 & 0xffff | (u32)extraout_DX << 0x10;
  uStack14 = 0x0;
  do {
    if (uVar4 <= uStack14) {
      return;
    }
    ppcVar3 = (code **)((int)*puVar2 + 0x4);
    uVar11 = uVar4;
    (**ppcVar3)(unaff_CS,(int)puVar2,(int)((u32)puVar2 >> 0x10),uStack14,uVar14,uVar10);
    uVar5 = uVar11;
    uVar9 = extraout_DX_00 | uVar5;
    if (uVar9 != 0x0) {
      unaff_CS = 0x1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar11 & 0xffff | (u32)extraout_DX_00 << 0x10);
      paStack26 = (astruct_419 *)CONCAT22(uVar9,uVar5);
      iVar6 = (uVar5 + 0x34);
      if ((iVar6 != 0x0) && (*(i32 *)(uVar5 + 0x36) != 0x0)) {
        uVar12 = param_1;
        uVar13 = (param_1 >> 0x10);
        pass1_1038_201a(iVar6,uVar9,uVar12,uVar13,(astruct_412 *)CONCAT22(uVar9,uVar5));
        if (iVar6 == 0x0) {
          uVar11 = struct_op_1030_73a8(paStack26,0x0,uVar9);
          uVar1 = ((int)uVar11 + 0xc);
          unaff_CS = 0x1008;
          BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1);
          if (BVar7 == 0x0) {
            unaff_CS = 0x1008;
            BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x2);
            if (BVar7 == 0x0) {
              BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x5);
              if (BVar7 == 0x0) {
                unaff_CS = 0x1008;
                BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x6);
                if (BVar7 == 0x0) goto LAB_1038_1c76;
              }
              unaff_CS = 0x1008;
              pass1_1038_2306(uVar12,uVar13,paStack26);
            }
            else {
              pass1_1038_26ee(uVar12,uVar13,(u32)paStack26);
            }
          }
          else {
            pass1_1038_24e8(uVar12,uVar13,(u32)paStack26);
          }
        }
      }
    }//
LAB_1038_1c76:
    uStack14 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1d68(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,u32 *param_5,mut param_6: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  code **ppcVar6;
  let mut uVar7: u16;
  let mut bVar8: bool;
  u8 *puVar9;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u32;
  let mut iVar14: i16;
  let mut unaff_CS: u16;
  u32 *puVar15;
  astruct_99 *paStack82;
  let mut uStack78: u16;
  let mut uStack52: u32;
  u8 local_30 [0x4];
  let mut uStack44: u32;
  u32 *puStack40;
  let mut uStack36: u32;
  u8 local_20 [0x4];
  u32 *puStack28;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uStack6 = 0x64;
  uStack8 = 0x0;
  ppcVar6 = (code **)((int)*param_5 + 0x10);
  puVar15 = param_5;
  (**ppcVar6)();
  uStack12 = CONCAT22((int)param_2,param_1);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar6 = (code **)((int)*param_5 + 0x4);
    uVar10 = uStack12;
    (**ppcVar6)(unaff_CS,param_5,uStack16,puVar15);
    uVar11 = uVar10;
    uStack18 = param_2;
    uVar12 = uStack18 | uVar11;
    uVar13 = (u32)uVar12;
    uStack20 = uVar11;
    if (uVar12 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | param_2 << 0x10);
      uStack22 = uVar12;
      unaff_CS = 0x1030;
      uStack24 = uVar11;
      puStack28 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uStack22,uVar11),uVar11,uStack22);
      uVar13 = (u32)puStack28 >> 0x10;
      puVar9 = local_20;
      ppcVar6 = (code **)((int)*puStack28 + 0x40);
      (**ppcVar6)(0x1030,(int)puStack28,(int)((u32)puStack28 >> 0x10),puVar9,(int)&DAT_1050_1050);
      if (puVar9 == NULL) {
        uStack36 = pass1_1028_62c8((u32)puStack28);
        uVar13 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (u32 *)(u32)((int)param_6 + 0x22);
        pass1_1008_5784((char *)CONCAT22(0x1050,local_30),(u32)puStack40);
        while( true ) {
          uVar12 = uVar13;
          puVar9 = local_30;
          unaff_CS = 0x1008;
          pass1_1008_5b12((char *)CONCAT22(0x1050,puVar9));
          uStack52 = CONCAT22(uVar12,puVar9);
          uVar13 = (u32)(uVar12 | puVar9);
          if ((uVar12 | puVar9) == 0x0) break;
          uVar2 = (puVar9 + 0x4);
          iVar3 = (puVar9 + 0x6);
          uVar4 = (puVar9 + 0x8);
          uVar11 = (puVar9 + 0xc);
          uVar5 = (puVar9 + 0xa);
          uVar7 = uVar11 / uVar5;
          uVar13 = (u32)uVar11 % (u32)uVar5;
          bVar8 = false;
          if (((0x0 < iVar3) && (!SBORROW2(iVar3,0x1))) && ((iVar3 == 0x5 || iVar3 + -0x1 < 0x4 || (iVar3 == 0x8)))) {
            bVar8 = true;
          }
          if (bVar8) {
            uVar10 = uStack36;
            if (uStack6 < uStack36) {
              uVar10 = uStack6 & 0xffff;
              uStack36 = uStack6;
            }
            uVar11 = uStack36 | uVar10;
            uVar13 = (u32)uVar11;
            if (uVar11 == 0x0) break;
            uStack78 = ((uVar10 & 0xffff | (u32)uStack36 << 0x10) / (u32)uVar7);
            if (uStack78 < uVar5) {
              piVar1 = (puVar9 + 0xc);
              *piVar1 = *piVar1 - uVar10;
              piVar1 = (puVar9 + 0xa);
              *piVar1 = *piVar1 - uStack78;
            }
            else {
              ppcVar6 = (code **)((int)*puStack40 + 0xc);
              (**ppcVar6)(0x1008,(int)puStack40,(int)((u32)puStack40 >> 0x10),uStack52);
              uStack44 = 0x0;
              uStack78 = uVar5;
            }
            paStack82 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
            uVar11 = ((u32)paStack82 >> 0x10);
            uVar12 = paStack82;
            if ((uVar11 | uVar12) == 0x0) {
              paStack82 = NULL;
            }
            else {
              paStack82.field0_0x0 = 0x389a;
              (uVar12 + 0x2) = 0x1008;
              (uVar12 + 0x4) = 0x0;
              (uVar12 + 0x6) = 0x0;
              (uVar12 + 0x8) = 0x0;
              (uVar12 + 0xa) = 0x0;
              (uVar12 + 0xc) = 0x0;
              paStack82.field0_0x0 = 0x56ce;
              (uVar12 + 0x2) = 0x1018;
            }
            uVar12 = ((u32)paStack82 >> 0x10);
            iVar14 = (int)paStack82;
            (iVar14 + 0xa) = uStack78;
            uVar10 = (u32)uStack78 * (u32)uVar7;
            uVar13 = uVar10 >> 0x10;
            (iVar14 + 0xc) = (int)uVar10;
            (iVar14 + 0x4) = uVar2;
            (iVar14 + 0x6) = iVar3;
            (iVar14 + 0x8) = uVar4;
            pass1_1028_6408((u32)puStack28,(u32 *)((u32)paStack82 & 0xffff | (u32)uVar12 << 0x10));
          }
        }
      }
      else {
        ppcVar6 = (code **)((int)*param_5 + 0x8);
        (**ppcVar6)(0x1030,param_5,0x0,uStack16);
      }
    }
    uStack16 += 0x1;
    param_2 = uVar13;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1faa(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,u32 *param_4,u32 *param_5)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_5 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22((int)param_2,param_1);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_5 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar2 = uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | param_2 << 0x10);
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22((int)param_2,uVar2),uVar2,(int)param_2);
    param_2 = param_2 & 0xffff0000 | uVar4 >> 0x10;
    uVar3 = uVar4;
    pass1_1038_1d68(uVar3,param_2,param_3,(param_3 >> 0x10),param_4,uVar4);
    if (uVar3 == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}
