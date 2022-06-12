)CONCAT22(puVar6,uVar3);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    uVar4 = uVar3;
    (**ppcVar1)(0x1008,(char)uVar3,puVar6);
    uStack18 = CONCAT22(extraout_DX_00,uVar4);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      ppcVar1 = (code **)((int)*puStack14 + 0x4);
      uVar10 = uStack18;
      (**ppcVar1)();
      uVar4 = uVar10;
      uVar7 = extraout_DX_01 | uVar4;
      if (uVar7 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | (u32)extraout_DX_01 << 0x10);
        uVar8 = 0x1030;
        uVar10 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar7,uVar4),uVar4,uVar7);
        if (((uVar10 >> 0x10) | uVar10) != 0x0) {
          pass1_1038_0e00(uVar10,param_1,puStack10,uVar10);
        }
      }
    }
    if (puStack10 != NULL) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(uVar8,uVar2,(char)puVar5,0x1);
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar8,uVar3,(char)puVar6,0x1);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_0f8c(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,u32 *param_5,mut param_6: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  code **ppcVar5;
  let mut uVar6: u32;
  qword qVar7;
  u8 *puVar8;
  let mut uVar9: u32;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut iVar13: i16;
  let mut uVar14: u16;
  let mut unaff_CS: u16;
  u32 *puVar15;
  astruct_99 *paStack80;
  let mut uStack76: u16;
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
  ppcVar5 = (code **)((int)*param_5 + 0x10);
  puVar15 = param_5;
  (**ppcVar5)();
  uStack12 = CONCAT22((int)param_2,param_1);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar5 = (code **)((int)*param_5 + 0x4);
    uVar9 = uStack12;
    uVar11 = param_2;
    (**ppcVar5)(unaff_CS,param_5,(char)uStack16,(int)(uStack16 >> 0x10),puVar15);
    uStack18 = uVar11;
    uVar12 = uVar9;
    uVar11 = uStack18 | uVar12;
    param_2 = (u32)uVar11;
    uStack20 = uVar12;
    if (uVar11 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9 & 0xffff | (u32)uStack18 << 0x10);
      uStack22 = uVar11;
      unaff_CS = 0x1030;
      uStack24 = uVar12;
      puStack28 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uStack22,uVar12),uVar12,uStack22);
      param_2 = (u32)puStack28 >> 0x10;
      puVar8 = local_20;
      ppcVar5 = (code **)((int)*puStack28 + 0x40);
      (**ppcVar5)(0x1030,(int)puStack28,(int)((u32)puStack28 >> 0x10),(char)puVar8,(int)&DAT_1050_1050);
      if (puVar8 == NULL) {
        uStack36 = pass1_1028_62c8((u32)puStack28);
        uVar9 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (u32 *)(u32)((int)param_6 + 0x22);
        pass1_1008_5784((char *)CONCAT22(0x1050,local_30),(u32)puStack40);
        while( true ) {
          uVar11 = uVar9;
          puVar8 = local_30;
          unaff_CS = 0x1008;
          pass1_1008_5b12((char *)CONCAT22(0x1050,puVar8));
          param_2 = (u32)(uVar11 | puVar8);
          if ((uVar11 | puVar8) == 0x0) break;
          uVar2 = (puVar8 + 0x4);
          uVar3 = (puVar8 + 0x6);
          uVar4 = (puVar8 + 0x8);
          uVar12 = (puVar8 + 0xa);
          uVar6 = (u32)(puVar8 + 0xc) / (u32)uVar12;
          uVar9 = uStack36;
          if (uStack6 < uStack36) {
            uVar9 = uStack6 & 0xffff;
            uStack36 = uStack6;
          }
          uVar10 = uStack36 | uVar9;
          param_2 = (u32)uVar10;
          if (uVar10 == 0x0) break;
          qVar7 = (qword)(uVar9 & 0xffff | (u32)uStack36 << 0x10) / (qword)uVar6;
          param_2 = (u32)qVar7 >> 0x10;
          uStack76 = qVar7;
          if (uStack76 == 0x0) break;
          if (uStack76 < uVar12) {
            piVar1 = (puVar8 + 0xc);
            *piVar1 = *piVar1 - uVar9;
            piVar1 = (puVar8 + 0xa);
            *piVar1 = *piVar1 - uStack76;
          }
          else {
            ppcVar5 = (code **)((int)*puStack40 + 0xc);
            (**ppcVar5)(0x1008,(int)puStack40,(int)((u32)puStack40 >> 0x10),(char)puVar8,uVar11);
            uStack44 = 0x0;
            uStack76 = uVar12;
          }
          paStack80 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
          uVar12 = ((u32)paStack80 >> 0x10);
          uVar11 = paStack80;
          if ((uVar12 | uVar11) == 0x0) {
            paStack80 = NULL;
          }
          else {
            paStack80.field0_0x0 = 0x389a;
            (uVar11 + 0x2) = 0x1008;
            (uVar11 + 0x4) = 0x0;
            (uVar11 + 0x6) = 0x0;
            (uVar11 + 0x8) = 0x0;
            (uVar11 + 0xa) = 0x0;
            (uVar11 + 0xc) = 0x0;
            paStack80.field0_0x0 = 0x56ce;
            (uVar11 + 0x2) = 0x1018;
          }
          uVar14 = ((u32)paStack80 >> 0x10);
          iVar13 = (int)paStack80;
          (iVar13 + 0xa) = uStack76;
          uVar6 = uStack76 * uVar6;
          uVar9 = uVar6 >> 0x10;
          (iVar13 + 0xc) = (int)uVar6;
          (iVar13 + 0x4) = uVar2;
          (iVar13 + 0x6) = uVar3;
          (iVar13 + 0x8) = uVar4;
          pass1_1028_6408((u32)puStack28,(u32 *)paStack80);
        }
      }
      else {
        ppcVar5 = (code **)((int)*param_5 + 0x8);
        (**ppcVar5)(0x1030,param_5,0x0,0x0,(char)uStack16,(int)(uStack16 >> 0x10));
      }
    }
    uStack16 += 0x1;
  } while( true );
}
