


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_11b0(ulong param_1,ulong *param_2,ulong *param_3,ushort param_4,ulong param_5,ushort param_6)

{
  code **ppcVar1;
  ushort uVar2;
  ulong uVar3;
  ulong uStack10;
  ulong uStack6;
  
  ppcVar1 = (code **)((int)*param_3 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22((int)param_5,param_4);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_3 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)();
    uVar2 = (ushort)uVar3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uint)param_5);
    uVar3 = struct_op_1030_73a8(CONCAT22((int)param_5,uVar2));
    param_5 = param_5 & 0xffff0000 | uVar3 >> 0x10;
    uVar2 = (ushort)uVar3;
    pass1_1038_0f8c((ushort)param_1,(ushort)(param_1 >> 0x10),param_2,uVar3,uVar2,param_5,0x1030,param_6);
    if (uVar2 == 0x0) break;
    uStack10 = uStack10 + 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_1220(ulong param_1,ulong param_2,ulong param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uchar *puVar6;
  uchar *puVar7;
  uchar *puVar8;
  undefined2 uVar10;
  ulong uVar9;
  ulong *puVar11;
  undefined uVar12;
  undefined4 *puStack14;
  undefined4 *puStack10;
  
  uVar10 = (undefined2)(param_3 >> 0x10);
  puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
  puVar6 = (uchar *)((ulong)puVar11 >> 0x10);
  uVar3 = (uint)puVar11;
  pass1_1038_4d6e(param_2,puVar11,uVar3,puVar6);
  puStack10 = (undefined4 *)CONCAT22(puVar6,uVar3);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  puVar7 = puVar6;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar6);
  if ((puVar7 != (uchar *)0x0) || (uVar4 != 0x0)) {
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x5);
    puVar8 = (uchar *)((ulong)puVar11 >> 0x10);
    uVar4 = (uint)puVar11;
    pass1_1038_4d6e(param_2,puVar11,uVar4,puVar8);
    puStack14 = (undefined4 *)CONCAT22(puVar8,uVar4);
    uVar12 = (undefined)uVar4;
    uVar2 = *puStack14;
    ppcVar1 = (code **)uVar2 + 0x8;
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x1008,uVar12,puVar8);
    uVar9 = CONCAT22(uVar10,puVar7);
    if (((puVar7 != (uchar *)0x0) || (uVar5 != 0x0)) &&
       (pass1_1038_11b0(param_1,(ulong *)CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12((char)puVar6,uVar3)),
                        (ulong *)CONCAT22(puVar8,uVar4),uVar5,uVar9,param_4), uVar5 == 0x0)) {
      if (puStack14 == (undefined4 *)0x0) {
        return;
      }
      ppcVar1 = (code **)uVar2;
      (**ppcVar1)(0x8,uVar12,(char)puVar8,0x1);
      return;
    }
    uVar10 = (undefined2)(uVar9 >> 0x10);
    if (puStack14 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar12,(char)puVar8,0x1);
    }
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x6);
    puVar8 = (uchar *)((ulong)puVar11 >> 0x10);
    uVar4 = (uint)puVar11;
    pass1_1038_4d6e(param_2,puVar11,uVar4,puVar8);
    puStack14 = (undefined4 *)CONCAT22(puVar8,uVar4);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x8,(char)uVar4,puVar8);
    if ((puVar7 != (uchar *)0x0) || (uVar5 != 0x0)) {
      pass1_1038_11b0(param_1,(ulong *)CONCAT22(puVar6,uVar3),(ulong *)CONCAT22(puVar8,uVar4),uVar5,
                      CONCAT22(uVar10,puVar7),param_4);
    }
    if (puStack14 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar4,(char)puVar8,0x1);
    }
  }
  if (puStack10 != (undefined4 *)0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x8,uVar3,(char)puVar6,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_134a(ushort param_1,ushort param_2,ulong *param_3,ulong *param_4,ulong *param_5,undefined2 param_6,
               ushort param_7)

{
  code **ppcVar1;
  ushort uVar2;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar3;
  ushort unaff_SS;
  ulong uVar4;
  ulong *puVar5;
  ulong uStack6;
  
  ppcVar1 = (code **)((int)*param_5 + 0x10);
  puVar5 = param_5;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_6);
  *param_3 = 0x0;
  do {
    if (uStack6 <= *param_4) {
      return;
    }
    uVar4 = *param_4;
    *param_4 = *param_4 + 0x1;
    ppcVar1 = (code **)((int)*param_5 + 0x4);
    (**ppcVar1)(param_7,param_5,uVar4,puVar5);
    uVar2 = (ushort)uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2));
    uVar3 = (uint)(uVar4 >> 0x10);
    param_7 = (ushort)&USHORT_1050_1028;
    uVar4 = pass1_1028_45e2(uVar4 & 0xffff | (ulong)uVar3 << 0x10,(uint)uVar4,uVar3,unaff_SS);
    uVar3 = (uint)(uVar4 >> 0x10);
    *(int *)param_3 = (int)uVar4;
    *(uint *)((int)param_3 + 0x2) = uVar3;
  } while ((uVar3 | *(uint *)param_3) == 0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_13da(ushort param_1,ushort param_2,ulong *param_3,ulong *param_4,ulong *param_5,ushort param_6,ushort param_7
               )

{
  code **ppcVar1;
  uint uVar2;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar3;
  ulong uVar4;
  ulong *puVar5;
  ulong uStack6;
  
  ppcVar1 = (code **)((int)*param_5 + 0x10);
  puVar5 = param_5;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_6);
  *param_3 = 0x0;
  do {
    if (uStack6 <= *param_4) {
      return;
    }
    uVar4 = *param_4;
    *param_4 = *param_4 + 0x1;
    ppcVar1 = (code **)((int)*param_5 + 0x4);
    (**ppcVar1)(param_7,param_5,uVar4,puVar5);
    uVar2 = (uint)uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
    if ((uVar3 | uVar2) == 0x0) {
      return;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2));
    uVar3 = (uint)(uVar4 >> 0x10);
    if ((uVar3 | (uint)uVar4) == 0x0) {
      return;
    }
    param_7 = (ushort)&USHORT_1050_1028;
    uVar4 = pass1_1028_3c32((ulong *)(uVar4 & 0xffff | (ulong)uVar3 << 0x10));
    uVar3 = (uint)(uVar4 >> 0x10);
    *(int *)param_3 = (int)uVar4;
    *(uint *)((int)param_3 + 0x2) = uVar3;
  } while ((uVar3 | *(uint *)param_3) == 0x0);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_1482(ulong param_1,ulong *param_2,ulong *param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7,
               ushort param_8)

{
  code **ppcVar1;
  sqword sVar2;
  ushort uVar3;
  undefined4 *puVar4;
  uint uVar5;
  ushort uVar6;
  ulong uVar7;
  uchar *puVar8;
  uchar *puVar9;
  uint uVar10;
  ushort uVar11;
  undefined uVar12;
  undefined uVar13;
  undefined2 uVar14;
  long lStack74;
  undefined4 local_46;
  int local_42 [0x4];
  uint uStack58;
  uint uStack56;
  ulong *puStack54;
  ulong *puStack50;
  ulong uStack46;
  ushort uStack42;
  uint uStack40;
  ushort uStack38;
  uint uStack36;
  undefined4 uStack34;
  uint uStack30;
  uint uStack28;
  uint uStack26;
  uint uStack24;
  ulong uStack22;
  undefined4 uStack18;
  undefined4 uStack14;
  undefined4 local_a;
  undefined4 local_6;
  
  local_6 = 0x0;
  local_a = 0x0;
  puVar4 = &local_a;
  uVar11 = (ushort)(param_1 >> 0x10);
  uVar3 = (ushort)param_1;
  pass1_1038_134a(uVar3,uVar11,(ulong *)CONCAT22(param_6,puVar4),(ulong *)CONCAT22(param_6,&local_6),param_3,puVar4,
                  param_4);
  uStack14 = CONCAT22(param_5,puVar4);
  ppcVar1 = (code **)((int)*param_2 + 0x10);
  (**ppcVar1)(param_4,param_2);
  uStack18 = CONCAT22(param_5,puVar4);
  uStack22 = 0x0;
  do {
    if (uStack18 <= uStack22) {
      return;
    }
    uStack14._2_2_ = uStack14._2_2_ | (uint)uStack14;
    if (uStack14._2_2_ == 0x0) {
      return;
    }
    pass1_1028_b58e(uStack14);
    uStack26 = uStack14._2_2_;
    uStack24 = uStack18._2_2_;
    pass1_1038_1a30(uVar3,uVar11,CONCAT22(uStack18._2_2_,uStack14._2_2_),(ushort)&USHORT_1050_1028);
    uStack30 = uStack14._2_2_;
    uStack28 = uStack18._2_2_;
    if ((uStack18._2_2_ | uStack14._2_2_) != 0x0) {
      sVar2 = (qword)CONCAT22(uStack18._2_2_,uStack14._2_2_) * 0x64;
      uVar5 = (uint)((qword)sVar2 >> 0x20);
      uVar7 = (ulong)sVar2 >> 0x1;
      ppcVar1 = (code **)((int)*param_2 + 0x4);
      uStack34 = uVar7;
      (**ppcVar1)((int)&USHORT_1050_1028,param_2,(char)uStack22,(int)(uStack22 >> 0x10));
      uVar6 = (ushort)uVar7;
      uStack38 = uVar6;
      uStack36 = uVar5;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6,uVar5);
      uStack42 = uVar6;
      uStack40 = uVar5;
      uStack46 = struct_op_1030_73a8(CONCAT22(uVar5,uVar6));
      puStack50 = *(ulong **)((int)uStack46 + 0x28);
      puStack54 = (ulong *)0x0;
      uStack56 = *(uint *)((int)puStack50 + 0x4);
      for (uStack58 = 0x0; uVar5 = uStack56, uStack58 < uStack56; uStack58 = uStack58 + 0x1) {
        pass1_1020_bb16(puStack50,(ulong *)CONCAT22(param_6,&local_46),
                        (ushort *)CONCAT13((char)(param_6 >> 0x8),CONCAT12((char)param_6,local_42)),uStack58);
        if (((local_46 != 0x0) && (0xd < local_42[0])) && (local_42[0] < 0x1d)) {
          uVar7 = local_46;
          if (uStack34 < local_46) {
            uVar7 = uStack34 & 0xffff;
            local_46._2_2_ = uStack34._2_2_;
          }
          uVar5 = (uint)uVar7;
          if ((local_a._2_2_ <= local_46._2_2_) && ((local_a._2_2_ < local_46._2_2_ || ((uint)local_a < uVar5)))) {
            uVar5 = (uint)local_a;
            local_46._2_2_ = local_a._2_2_;
          }
          lStack74 = CONCAT22(local_46._2_2_,uVar5);
          uStack34 = CONCAT22(uStack34._2_2_ + (-(uint)((uint)uStack34 < uVar5) - (int)local_46._2_2_),
                              (uint)uStack34 - uVar5);
          local_a = CONCAT22(local_a._2_2_ + (-(uint)((uint)local_a < uVar5) - (int)local_46._2_2_),
                             (uint)local_a - uVar5);
          puVar9 = local_46._2_2_;
          if (puStack54 == (ulong *)0x0) {
            puVar8 = local_46._2_2_;
            uVar10 = uVar5;
            mem_op_1000_179c(0xa,local_46._2_2_,0x1000);
            puVar9 = (uchar *)((uint)puVar8 | uVar10);
            if (puVar9 == (uchar *)0x0) {
              uVar10 = 0x0;
              puVar9 = (uchar *)0x0;
            }
            else {
              pass1_1020_ba3e((long *)CONCAT22(puVar8,uVar10),0x5,0x5,param_8,param_7);
            }
            puStack54 = (ulong *)CONCAT22(puVar9,uVar10);
          }
          pass1_1020_bb8a((long *)puStack54,uVar5,CONCAT22(local_42[0],local_46._2_2_),param_8,param_6);
          uVar7 = local_46 - lStack74;
          pass1_1020_bb8a((long *)puStack50,(uint)uVar7,CONCAT22(local_42[0],(int)(uVar7 >> 0x10)),param_8,param_6);
          if (local_a == 0x0) {
            pass1_1038_1b3a(uVar3,uVar11,uStack14,puStack54,param_6,(ushort)uVar7,param_7,param_8);
            puStack54 = (ulong *)0x0;
            uVar7 = ZEXT24(&local_a);
            pass1_1038_134a(uVar3,uVar11,(ulong *)CONCAT22(param_6,&local_a),(ulong *)CONCAT22(param_6,&local_6),param_3
                            ,&local_a,0x1020);
            uVar5 = (uint)uVar7;
            uStack14 = uVar7 & 0xffff | ZEXT24(puVar9) << 0x10;
            uVar10 = (uint)puVar9 | uVar5;
            if (uVar10 != 0x0) {
              uVar12 = 0x64;
              uVar13 = 0x0;
              uVar14 = 0x0;
              pass1_1028_b58e(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10);
              uStack26 = uVar5;
              uStack24 = uVar10;
              pass1_1038_1a30(uVar3,uVar11,CONCAT22(uVar10,uVar5),(ushort)&USHORT_1050_1028);
              uVar7 = (ulong)(CONCAT22(uVar10,uVar5) * CONCAT22(uVar14,CONCAT11(uVar13,uVar12))) >> 0x1;
              uStack34 = uVar7;
              uStack30 = uVar5;
              uStack28 = uVar10;
            }
          }
          uVar5 = (uint)uVar7;
          if ((uStack34 == 0x0) || (local_a == 0x0)) break;
        }
      }
      if (puStack54 != (ulong *)0x0) {
        pass1_1038_1b3a(uVar3,uVar11,uStack14,puStack54,param_6,uVar5,param_7,param_8);
        puStack54 = (ulong *)0x0;
      }
    }
    uStack22 = uStack22 + 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_16f2(ulong param_1,ulong *param_2,ulong *param_3,uint param_4,ushort param_5,ushort param_6,ushort param_7,
               ushort param_8,uchar param_9)

{
  long *plVar1;
  code **ppcVar2;
  ushort uVar3;
  undefined4 *puVar4;
  ushort uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  uchar *puVar12;
  ulong uVar13;
  ulong uVar14;
  ulong uVar15;
  long lVar16;
  ushort uVar17;
  long lStack68;
  undefined4 *puStack56;
  undefined4 *puStack52;
  long *plStack50;
  uint uStack46;
  undefined4 uStack42;
  ulong uStack22;
  ulong uStack18;
  ulong uStack14;
  undefined4 local_a;
  undefined4 local_6;
  
  local_6 = 0x0;
  local_a = 0x0;
  puVar6 = &local_a;
  uVar17 = (ushort)(param_1 >> 0x10);
  uVar3 = (ushort)param_1;
  pass1_1038_13da(uVar3,uVar17,(ulong *)CONCAT22(param_8,puVar6),(ulong *)CONCAT22(param_8,&local_6),param_3,
                  (ushort)puVar6,param_7);
  uStack14 = CONCAT22(param_4,puVar6);
  uVar8 = param_4 | (uint)puVar6;
  if (uVar8 != 0x0) {
    ppcVar2 = (code **)((int)*param_2 + 0x10);
    (**ppcVar2)(param_7,param_2);
    uStack18 = CONCAT22(uVar8,puVar6);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1) {
      ppcVar2 = (code **)((int)*param_2 + 0x4);
      uVar15 = uStack18;
      uVar10 = uVar8;
      (**ppcVar2)(param_7,param_2,(char)uStack22,(int)(uStack22 >> 0x10));
      uVar5 = (ushort)uVar15;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,uVar10);
      param_7 = 0x1030;
      uVar15 = struct_op_1030_73a8(CONCAT22(uVar10,uVar5));
      uVar11 = (uint)(uVar15 >> 0x10);
      uVar9 = (uint)uVar15;
      pass1_1038_1a30(uVar3,uVar17,CONCAT22(uVar10,uVar5),0x1030);
      if ((uVar11 | uVar9) != 0x0) {
        uStack42 = (ulong)(CONCAT22(uVar11,uVar9) * 0x64) >> 0x1;
        plVar1 = *(long **)(uVar5 + 0x22);
        uVar9 = *(uint *)(uVar5 + 0x24);
        uVar13 = (ulong)uVar9;
        uStack46 = (uint)plVar1;
        if ((uVar9 | uStack46) != 0x0) {
          plStack50 = (long *)0x0;
          puVar6 = (undefined4 *)pass1_1028_0d80(uVar15);
          puStack56 = (undefined4 *)0x0;
          puStack52 = puVar6;
          while( true ) {
            lVar16 = pass1_1020_bae6(uStack46,CONCAT22(puStack52,(int)((ulong)plVar1 >> 0x10)),(uint)puStack52,
                                     (uint)uVar13,param_8);
            uVar9 = (uint)((ulong)lVar16 >> 0x10);
            puVar7 = (undefined4 *)lVar16;
            uVar13 = (ulong)(uVar9 | (uint)puVar7);
            if ((uVar9 | (uint)puVar7) != 0x0) {
              uVar14 = (ulong)uVar9;
              if ((uStack42._2_2_ <= uVar9) && ((uStack42._2_2_ < uVar9 || ((undefined4 *)uStack42 < puVar7)))) {
                uVar14 = (ulong)uStack42._2_2_;
                puVar7 = (undefined4 *)uStack42;
              }
              if ((local_a._2_2_ <= (uint)uVar14) &&
                 ((local_a._2_2_ < (uint)uVar14 || ((undefined4 *)local_a < puVar7)))) {
                uVar14 = (ulong)local_a._2_2_;
                puVar7 = (undefined4 *)local_a;
              }
              puVar12 = (uchar *)uVar14;
              lStack68 = CONCAT22(puVar12,puVar7);
              uStack42 = CONCAT22((uStack42._2_2_ - (int)puVar12) - (uint)((undefined4 *)uStack42 < puVar7),
                                  (int)(undefined4 *)uStack42 - (int)puVar7);
              local_a = CONCAT22((local_a._2_2_ - (int)puVar12) - (uint)((undefined4 *)local_a < puVar7),
                                 (int)(undefined4 *)local_a - (int)puVar7);
              uVar13 = uVar14;
              if (plStack50 == (long *)0x0) {
                puVar4 = puVar7;
                mem_op_1000_179c(0xa,puVar12,0x1000);
                uVar13 = (ulong)((uint)puVar12 | (uint)puVar4);
                if (((uint)puVar12 | (uint)puVar4) == 0x0) {
                  puVar4 = (undefined4 *)0x0;
                  uVar13 = 0x0;
                }
                else {
                  pass1_1020_ba3e((long *)CONCAT22(puVar12,puVar4),0x5,0x5,param_6,param_5);
                }
                plStack50 = (long *)CONCAT22((int)uVar13,puVar4);
              }
              pass1_1020_bb8a(plStack50,(uint)puVar7,uVar14 | ZEXT24(puStack52) << 0x10,param_6,param_8);
              pass1_1020_bb8a(plVar1,(uint)(lVar16 - lStack68),
                              CONCAT22(puStack52,(int)((ulong)(lVar16 - lStack68) >> 0x10)),param_6,param_8);
              uVar9 = (uint)uVar13;
              puStack56 = puStack52;
              puVar7 = puStack52;
              if (local_a == 0x0) {
                pass1_1038_1ac6(uVar3,uVar17,uStack14,(ulong)plStack50,(int)puStack52,param_8,param_9);
                plStack50 = (long *)0x0;
                puVar7 = &local_a;
                pass1_1038_13da(uVar3,uVar17,(ulong *)CONCAT22(param_8,puVar7),(ulong *)CONCAT22(param_8,&local_6),
                                param_3,(ushort)puVar7,0x1020);
                uStack14 = CONCAT22(uVar9,puVar7);
                uVar13 = (ulong)(uVar9 | (uint)puVar7);
                if ((uVar9 | (uint)puVar7) == 0x0) {
                  return;
                }
              }
            }
            param_7 = 0x1020;
            if ((uStack42 == 0x0) || (local_a == 0x0)) break;
            param_7 = (ushort)&USHORT_1050_1028;
            puVar7 = (undefined4 *)pass1_1028_0d80(uVar15);
            if ((puVar7 == puStack56) || ((puStack52 = puVar7, puStack56 == (undefined4 *)0x0 && (puVar7 == puVar6))))
            break;
          }
          if (plStack50 != (long *)0x0) {
            pass1_1038_1ac6(uVar3,uVar17,uStack14,(ulong)plStack50,(int)puVar7,param_8,param_9);
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_1940(ulong param_1,ulong *param_2,ulong param_3,ushort param_4,ushort param_5,ushort param_6)

{
  code **ppcVar1;
  uint uVar2;
  uint uVar3;
  uchar *puVar4;
  uint extraout_DX;
  ulong *puVar5;
  ulong *puStack10;
  
  puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x3);
  puVar4 = (uchar *)((ulong)puVar5 >> 0x10);
  uVar2 = (uint)puVar5;
  pass1_1038_4d6e(param_3,puVar5,uVar2,puVar4);
  puStack10 = (ulong *)CONCAT22(puVar4,uVar2);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar4);
  if ((extraout_DX | uVar3) != 0x0) {
    pass1_1038_1482(param_1,param_2,puStack10,0x1008,extraout_DX | uVar3,param_6,param_4,param_5);
  }
  if (puStack10 != (ulong *)0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x1008,uVar2,(char)puVar4,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_19a0(ulong param_1,ulong *param_2,ulong param_3,ushort param_4,uchar param_5)

{
  code **ppcVar1;
  ulong uVar2;
  uint uVar3;
  uint uVar4;
  uchar *puVar5;
  uint extraout_DX;
  code **ppcVar6;
  ulong *puVar7;
  ulong *puStack10;
  
  puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
  puVar5 = (uchar *)((ulong)puVar7 >> 0x10);
  uVar3 = (uint)puVar7;
  pass1_1038_4d6e(param_3,puVar7,uVar3,puVar5);
  puStack10 = (ulong *)CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar6 = (code **)uVar2;
  ppcVar1 = ppcVar6 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar5);
  if ((extraout_DX | uVar4) == 0x0) {
    vsprintf_op_1030_840a((ulong)s_mineToSmelter__no_mines_1050_59df,0x1030,param_4,0x0);
    if (puStack10 != (ulong *)0x0) {
      ppcVar1 = ppcVar6;
      (**ppcVar1)(0x1030,uVar3,(char)puVar5,0x1);
      return;
    }
  }
  else {
    pass1_1038_16f2(param_1,puStack10,param_2,extraout_DX | uVar4,(ushort)ppcVar6,(ushort)(uVar2 >> 0x10),0x1008,param_4
                    ,param_5);
    if (puStack10 != (ulong *)0x0) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(0x1008,uVar3,(char)puVar5,0x1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_1a30(ushort param_1,ushort param_2,ulong param_3,ushort param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  uint uVar3;
  ulong uVar4;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  undefined2 uVar5;
  uint uVar6;
  uint uVar7;
  ulong uStack18;
  ulong uStack10;
  uint uStack6;
  
  uVar5 = (undefined2)(param_3 >> 0x10);
  puVar1 = (undefined4 *)*(undefined4 *)((int)param_3 + 0x1e);
  uVar7 = *(uint *)((int)param_3 + 0x20);
  uStack6 = (uint)puVar1;
  uVar3 = uVar7 | uStack6;
  if (uVar3 != 0x0) {
    ppcVar2 = (code **)((int)*puVar1 + 0x10);
    uVar6 = uStack6;
    (**ppcVar2)();
    uStack10 = CONCAT22(extraout_DX,uVar3);
    for (uStack18 = 0x0; uStack18 < uStack10; uStack18 = uStack18 + 0x1) {
      ppcVar2 = (code **)((int)*puVar1 + 0x4);
      uVar4 = uStack10;
      (**ppcVar2)(param_4,uStack6,(int)((ulong)puVar1 >> 0x10),uStack18,uVar6,uVar7);
      if ((extraout_DX_00 | (uint)uVar4) != 0x0) {
        param_4 = (ushort)&USHORT_1050_1028;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uint)uVar4,extraout_DX_00);
      }
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_1ac6(ushort param_1,ushort param_2,ulong param_3,ulong param_4,int param_5,ushort param_6,uchar param_7)

{
  undefined2 extraout_DX;
  undefined local_118 [0x112];
  undefined4 uStack6;
  
  pass1_1028_b58e(param_3);
  uStack6 = CONCAT22(extraout_DX,param_5);
  pass1_1030_e8a0((astruct_100 *)CONCAT22(param_6,local_118),param_4,*(ushort *)((int)param_3 + 0xc),
                  *(ulong *)(param_5 + 0x4),param_6,param_7);
  pass1_1028_d52c(*_PTR_LOOP_1050_5748,*_PTR_LOOP_1050_65e2 + 0x1,(ulong *)CONCAT22(param_6,local_118));
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far
pass1_1038_1b3a(ushort param_1,ushort param_2,ulong param_3,ulong *param_4,ushort param_5,ushort param_6,ushort param_7,
               ushort param_8)

{
  int extraout_DX;
  ulong local_1a;
  ushort local_16 [0x2];
  uint uStack18;
  uint uStack16;
  undefined4 uStack14;
  ulong uStack10;
  ulong uStack6;
  
  pass1_1028_b58e(param_3);
  uStack6 = CONCAT22(extraout_DX,param_6);
  uStack10 = param_3;
  uStack14 = pass1_1028_45e2(param_3,(uint)param_3,extraout_DX,param_5);
  uStack16 = *(uint *)((int)param_4 + 0x4);
  for (uStack18 = 0x0; uStack18 < uStack16; uStack18 = uStack18 + 0x1) {
    pass1_1020_bb16(param_4,(ulong *)CONCAT22(param_5,&local_1a),(ushort *)CONCAT22(param_5,local_16),uStack18);
    if (uStack14 < local_1a) {
      pass1_1030_7ddc(uStack6,uStack14,local_16[0],(uint)uStack14,uStack14._2_2_,param_7,param_8,param_5);
      uStack14 = 0x0;
    }
    else {
      uStack14 = uStack14 - local_1a;
      pass1_1030_7ddc(uStack6,local_1a,local_16[0],(uint)local_1a,uStack14._2_2_,param_7,param_8,param_5);
    }
    if (uStack14 == 0x0) break;
  }
  if (param_4 != (ulong *)0x0) {
    fn_ptr_1020_ba7e(param_4);
    fn_ptr_1000_17ce((astruct_18 *)param_4,0x1000);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_1c02(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_1c3e(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6)

{
  undefined2 uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  ulong uVar4;
  uint uVar5;
  int iVar6;
  BOOL16 BVar7;
  undefined4 *puVar8;
  uint extraout_DX;
  uint extraout_DX_00;
  uint uVar9;
  undefined2 uVar10;
  ulong uVar11;
  ushort uVar12;
  ushort uVar13;
  undefined2 uVar14;
  ulong uStack26;
  ulong uStack14;
  
  uVar10 = (undefined2)(param_2 >> 0x10);
  puVar2 = (undefined4 *)*(undefined4 *)((int)param_2 + 0xc);
  uVar10 = *(undefined2 *)((int)param_2 + 0xe);
  ppcVar3 = (code **)((int)*puVar2 + 0x10);
  puVar8 = puVar2;
  uVar14 = (int)puVar2;
  (**ppcVar3)();
  uVar4 = (ulong)puVar8 & 0xffff | (ulong)extraout_DX << 0x10;
  uStack14 = 0x0;
  do {
    if (uVar4 <= uStack14) {
      return;
    }
    ppcVar3 = (code **)((int)*puVar2 + 0x4);
    uVar11 = uVar4;
    (**ppcVar3)(param_5,(int)puVar2,(int)((ulong)puVar2 >> 0x10),uStack14,uVar14,uVar10);
    uVar5 = (uint)uVar11;
    uVar9 = extraout_DX_00 | uVar5;
    if (uVar9 != 0x0) {
      param_5 = (ushort)&USHORT_1050_1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_00);
      uStack26 = CONCAT22(uVar9,uVar5);
      iVar6 = *(int *)(uVar5 + 0x34);
      if ((iVar6 != 0x0) && (*(long *)(uVar5 + 0x36) != 0x0)) {
        uVar12 = (ushort)param_1;
        uVar13 = (ushort)(param_1 >> 0x10);
        pass1_1038_201a(uVar12,uVar13,CONCAT22(uVar9,uVar5),iVar6,uVar9);
        if (iVar6 == 0x0) {
          uVar11 = struct_op_1030_73a8(uStack26);
          uVar1 = *(undefined2 *)((int)uVar11 + 0xc);
          param_5 = 0x1008;
          BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1);
          if (BVar7 == 0x0) {
            param_5 = 0x1008;
            BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x2);
            if (BVar7 == 0x0) {
              BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x5);
              if (BVar7 == 0x0) {
                param_5 = 0x1008;
                BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x6);
                if (BVar7 == 0x0) goto LAB_1038_1c76;
              }
              param_5 = 0x1008;
              pass1_1038_2306(uVar12,uVar13,uStack26);
            }
            else {
              pass1_1038_26ee(uVar12,uVar13,uStack26,param_3,param_4,param_6);
            }
          }
          else {
            pass1_1038_24e8(uVar12,uVar13,uStack26,param_3,param_4,param_6);
          }
        }
      }
    }
LAB_1038_1c76:
    uStack14 = uStack14 + 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_1d68(ushort param_1,ushort param_2,ulong *param_3,ulong param_4,ushort param_5,ushort param_6,ushort param_7,
               ulong param_8)

{
  int *piVar1;
  undefined2 uVar2;
  int iVar3;
  undefined2 uVar4;
  uint uVar5;
  code **ppcVar6;
  ulong uVar7;
  uint uVar8;
  bool bVar9;
  undefined *puVar10;
  ulong uVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  ulong *puVar15;
  astruct_99 *paStack82;
  uint uStack78;
  undefined4 uStack52;
  undefined local_30 [0x4];
  undefined4 uStack44;
  undefined4 *puStack40;
  undefined4 uStack36;
  undefined local_20 [0x4];
  undefined4 *puStack28;
  uint uStack24;
  uint uStack22;
  uint uStack20;
  uint uStack18;
  ulong uStack16;
  ulong uStack12;
  undefined2 uStack8;
  undefined4 uStack6;
  
  uStack6 = 0x64;
  uStack8 = 0x0;
  ppcVar6 = (code **)((int)*param_3 + 0x10);
  puVar15 = param_3;
  (**ppcVar6)();
  uStack12 = CONCAT22((int)param_8,param_5);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar6 = (code **)((int)*param_3 + 0x4);
    uVar11 = uStack12;
    uVar13 = (uint)param_8;
    (**ppcVar6)(param_6,param_3,uStack16,puVar15);
    uStack18 = uVar13;
    uVar12 = (uint)uVar11;
    uVar13 = uStack18 | uVar12;
    param_8 = (ulong)uVar13;
    uStack20 = uVar12;
    if (uVar13 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar12,uStack18);
      uStack22 = uVar13;
      param_6 = 0x1030;
      uStack24 = uVar12;
      puStack28 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uStack22,uVar12));
      param_8 = (ulong)puStack28 >> 0x10;
      puVar10 = local_20;
      ppcVar6 = (code **)((int)*puStack28 + 0x40);
      (**ppcVar6)(0x1030,(int)puStack28,(int)((ulong)puStack28 >> 0x10),puVar10,param_7);
      if (puVar10 == (undefined *)0x0) {
        uStack36 = pass1_1028_62c8((ulong)puStack28,param_7);
        uVar11 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (undefined4 *)*(ulong *)((int)param_4 + 0x22);
        pass1_1008_5784((ulong *)CONCAT22(param_7,local_30),(ulong)puStack40);
        while( true ) {
          uVar13 = (uint)uVar11;
          puVar10 = local_30;
          param_6 = 0x1008;
          pass1_1008_5b12(puVar10,param_7);
          uStack52 = CONCAT22(uVar13,puVar10);
          param_8 = (ulong)(uVar13 | (uint)puVar10);
          if ((uVar13 | (uint)puVar10) == 0x0) break;
          uVar2 = *(undefined2 *)(puVar10 + 0x4);
          iVar3 = *(int *)(puVar10 + 0x6);
          uVar4 = *(undefined2 *)(puVar10 + 0x8);
          uVar12 = *(uint *)(puVar10 + 0xc);
          uVar5 = *(uint *)(puVar10 + 0xa);
          uVar8 = uVar12 / uVar5;
          uVar11 = (ulong)uVar12 % (ulong)uVar5;
          bVar9 = false;
          if (((0x0 < iVar3) && (!SBORROW2(iVar3,0x1))) && ((iVar3 == 0x5 || iVar3 + -0x1 < 0x4 || (iVar3 == 0x8)))) {
            bVar9 = true;
          }
          if (bVar9) {
            uVar11 = uStack36;
            if (uStack6 < uStack36) {
              uVar11 = uStack6 & 0xffff;
              uStack36._2_2_ = uStack6._2_2_;
            }
            uVar12 = uStack36._2_2_ | (uint)uVar11;
            param_8 = (ulong)uVar12;
            if (uVar12 == 0x0) break;
            uStack78 = (uint)((uVar11 & 0xffff | (ulong)uStack36._2_2_ << 0x10) / (ulong)uVar8);
            if (uStack78 < uVar5) {
              piVar1 = (int *)(puVar10 + 0xc);
              *piVar1 = *piVar1 - (uint)uVar11;
              piVar1 = (int *)(puVar10 + 0xa);
              *piVar1 = *piVar1 - uStack78;
            }
            else {
              ppcVar6 = (code **)((int)*puStack40 + 0xc);
              (**ppcVar6)(0x1008,(int)puStack40,(int)((ulong)puStack40 >> 0x10),uStack52);
              uStack44 = 0x0;
              uStack78 = uVar5;
            }
            paStack82 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
            uVar12 = (uint)((ulong)paStack82 >> 0x10);
            uVar13 = (uint)paStack82;
            if ((uVar12 | uVar13) == 0x0) {
              paStack82 = (astruct_99 *)0x0;
            }
            else {
              paStack82->field_0x0 = 0x389a;
              *(undefined2 *)(uVar13 + 0x2) = 0x1008;
              *(undefined2 *)(uVar13 + 0x4) = 0x0;
              *(undefined2 *)(uVar13 + 0x6) = 0x0;
              *(undefined2 *)(uVar13 + 0x8) = 0x0;
              *(undefined2 *)(uVar13 + 0xa) = 0x0;
              *(undefined2 *)(uVar13 + 0xc) = 0x0;
              paStack82->field_0x0 = 0x56ce;
              *(undefined2 *)(uVar13 + 0x2) = 0x1018;
            }
            uVar13 = (uint)((ulong)paStack82 >> 0x10);
            iVar14 = (int)paStack82;
            *(uint *)(iVar14 + 0xa) = uStack78;
            uVar7 = (ulong)uStack78 * (ulong)uVar8;
            uVar11 = uVar7 >> 0x10;
            *(undefined2 *)(iVar14 + 0xc) = (int)uVar7;
            *(undefined2 *)(iVar14 + 0x4) = uVar2;
            *(int *)(iVar14 + 0x6) = iVar3;
            *(undefined2 *)(iVar14 + 0x8) = uVar4;
            pass1_1028_6408((ulong)puStack28,(ulong *)((ulong)paStack82 & 0xffff | (ulong)uVar13 << 0x10),param_7);
          }
        }
      }
      else {
        ppcVar6 = (code **)((int)*param_3 + 0x8);
        (**ppcVar6)(0x1030,param_3,0x0,uStack16);
      }
    }
    uStack16 = uStack16 + 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_1faa(ulong param_1,ulong *param_2,ulong *param_3,ushort param_4,ulong param_5,ushort param_6)

{
  code **ppcVar1;
  ushort uVar2;
  ulong uVar3;
  ulong uStack10;
  ulong uStack6;
  
  ppcVar1 = (code **)((int)*param_3 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22((int)param_5,param_4);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_3 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)();
    uVar2 = (ushort)uVar3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uint)param_5);
    uVar3 = struct_op_1030_73a8(CONCAT22((int)param_5,uVar2));
    param_5 = param_5 & 0xffff0000 | uVar3 >> 0x10;
    uVar2 = (ushort)uVar3;
    pass1_1038_1d68((ushort)param_1,(ushort)(param_1 >> 0x10),param_2,uVar3,uVar2,0x1030,param_6,param_5);
    if (uVar2 == 0x0) break;
    uStack10 = uStack10 + 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_201a(ushort param_1,ushort param_2,ulong param_3,undefined2 param_4,undefined2 param_5)

{
  uint *puVar1;
  int iVar2;
  code **ppcVar3;
  long lVar4;
  uint uVar6;
  uint uVar7;
  ushort uVar8;
  ulong uVar9;
  uchar *puVar10;
  ulong uVar11;
  ulong uVar12;
  astruct_416 *iVar12;
  uint uVar13;
  uchar *puVar14;
  undefined2 uVar15;
  undefined4 *puVar16;
  undefined2 uVar17;
  long lStack24;
  long lStack20;
  uint uStack10;
  astruct_413 *uVar5;
  
  uVar17 = (undefined2)(param_3 >> 0x10);
  uVar15 = 0x1030;
  puVar16 = (undefined4 *)pass1_1030_6b16(param_3);
  uVar6 = (uint)((ulong)puVar16 >> 0x10);
  uVar5 = (astruct_413 *)puVar16;
  if ((uVar6 | (uint)uVar5) == 0x0) {
    return;
  }
  iVar12 = (astruct_416 *)param_3;
  iVar2 = iVar12->field_0x34;
  lVar4 = (long)iVar2;
  uVar12 = lVar4 * 0x64;
  puVar10 = (uchar *)(uVar12 >> 0x10);
  uVar7 = (uint)uVar12;
  uStack10 = 0x0;
  lStack20 = 0x0;
  if (uVar5->field_0x4 == 0x0) {
    if (uVar5->field_0x6 == 0x0) {
      if (uVar5->field_0x8 == 0x0) goto LAB_1038_2102;
      uVar8 = pass1_1020_c42e(uVar5->field_0x8);
      uVar11 = (ulong)uVar5->field_0xa * (ulong)uVar8;
      puVar14 = (uchar *)(uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11) {
        uVar11 = uVar12 & 0xffff;
        puVar14 = puVar10;
      }
      uVar12 = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
      uVar9 = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)(ulong)uVar8;
      puVar1 = &uVar5->field_0xa;
      *puVar1 = *puVar1 - (int)uVar9;
      uStack10 = (uint)((long)uVar12 / 0x64);
      uVar12 = (long)uVar12 % 0x64;
      uVar11 = uVar12;
      if (uVar12 != 0x0) {
        uStack10 = uStack10 + 0x1;
        uVar11 = (ulong)uStack10;
      }
      uVar7 = (uint)uVar11;
      mem_op_1000_179c(0x2a,(uchar *)uVar12,0x1000);
      puVar10 = (uchar *)((uint)uVar12 | uVar7);
      if (puVar10 == (uchar *)0x0) goto LAB_1038_20fa;
      pass1_1038_6838((ushort *)CONCAT22((uint)uVar12,uVar7),uVar9,uVar5->field_0x8,uStack10,iVar12->field_0x4);
    }
    else {
      uVar8 = switch_1020_c3b4(uVar5->field_0x6);
      uVar11 = (ulong)uVar5->field_0xa * (ulong)uVar8;
      puVar14 = (uchar *)(uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11) {
        uVar11 = uVar12 & 0xffff;
        puVar14 = puVar10;
      }
      uVar12 = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
      uVar9 = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)(ulong)uVar8;
      puVar1 = &uVar5->field_0xa;
      *puVar1 = *puVar1 - (int)uVar9;
      uStack10 = (uint)((long)uVar12 / 0x64);
      uVar12 = (long)uVar12 % 0x64;
      uVar11 = uVar12;
      if (uVar12 != 0x0) {
        uStack10 = uStack10 + 0x1;
        uVar11 = (ulong)uStack10;
      }
      uVar7 = (uint)uVar11;
      mem_op_1000_179c(0x2a,(uchar *)uVar12,0x1000);
      puVar10 = (uchar *)((uint)uVar12 | uVar7);
      if (puVar10 == (uchar *)0x0) goto LAB_1038_20fa;
      pass1_1038_675c((ushort *)CONCAT22((uint)uVar12,uVar7),uVar9,uVar5->field_0x6,uStack10,iVar12->field_0x4);
    }
  }
  else {
    uVar13 = uVar5->field_0xa;
    puVar14 = (uchar *)0x0;
    if (((int)puVar10 < 0x1) && (((uchar *)0x7fff < puVar10 || (uVar7 < uVar13)))) {
      uVar13 = uVar7;
      puVar14 = puVar10;
    }
    lStack24 = CONCAT22(puVar14,uVar13);
    puVar1 = &uVar5->field_0xa;
    *puVar1 = *puVar1 - uVar13;
    uStack10 = (uint)(lStack24 / 0x64);
    uVar11 = lStack24 % 0x64;
    uVar12 = uVar11;
    if (uVar11 != 0x0) {
      uStack10 = uStack10 + 0x1;
      uVar12 = (ulong)uStack10;
    }
    uVar7 = (uint)uVar12;
    mem_op_1000_179c(0x2a,(uchar *)uVar11,0x1000);
    puVar10 = (uchar *)((uint)uVar11 | uVar7);
    if (puVar10 == (uchar *)0x0) {
LAB_1038_20fa:
      uVar15 = 0x1000;
      lStack20 = 0x0;
      goto LAB_1038_2102;
    }
    pass1_1038_6590((ushort *)CONCAT22((uint)uVar11,uVar7),uVar13,(ushort)puVar14,uVar5->field_0x4,uStack10,
                    iVar12->field_0x4);
  }
  uVar15 = 0x1000;
  lStack20 = CONCAT22(puVar10,uVar7);
LAB_1038_2102:
  if (lStack20 != 0x0) {
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    uVar15 = 0x1030;
    uVar7 = uStack10;
    pass1_1030_6c4c(param_3,iVar2 - uStack10);
  }
  if (uVar5->field_0xa == 0x0) {
    if ((uVar6 | (uint)uVar5) != 0x0) {
      ppcVar3 = (code **)*puVar16;
      (**ppcVar3)(uVar15,uVar5,uVar6,0x1);
    }
  }
  else {
    pass1_1030_6c66(param_3,0x0,(ulong)puVar16,uVar7,puVar10,0x1030);
  }
  return;
}
