
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_c23e(ushort param_1,ushort param_2,ushort *param_3,ulong param_4,long param_5,uint param_6,uint param_7,
               ushort param_8)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  uint uVar7;
  uint uVar8;
  ushort extraout_DX;
  undefined4 *puStack22;
  ulong uStack10;
  undefined4 uStack6;
  
  pass1_1030_627e(param_8,param_6,param_7,_PTR_LOOP_1050_5740,param_3,param_5);
  uStack6 = CONCAT22(param_7,param_6);
  uVar7 = param_7 | param_6;
  if (uVar7 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6,param_7);
    uStack10 = CONCAT22(uVar7,param_6);
    uVar1 = *(ulong *)(param_6 + 0x2a);
    if (uVar1 != param_4) {
      uVar6 = param_4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)(uVar1 >> 0x10));
      uVar4 = (uint)uVar6;
      puVar3 = (undefined4 *)(uVar6 & 0xffff | (ulong)uVar7 << 0x10);
      uVar8 = uVar7;
      uVar5 = uVar4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
      puStack22 = (undefined4 *)CONCAT22(uVar8,uVar5);
      if (((puVar3 == (undefined4 *)0x0) || ((uVar8 | uVar5) == 0x0)) ||
         (*(long *)(uVar5 + 0x200) != *(long *)(uVar4 + 0x200))) {
        return;
      }
      ppcVar2 = (code **)((int)*puVar3 + 0x18);
      (**ppcVar2)(0x1030,uVar4,uVar7,uStack6);
      ppcVar2 = (code **)((int)*puStack22 + 0x8);
      (**ppcVar2)();
      pass1_1030_73ee(uStack10,param_4,extraout_DX);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far
pass1_1028_c314(ushort param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,ushort *param_6,ushort param_7,
               ushort param_8,ulong param_9)

{
  undefined4 *puVar1;
  int local_14;
  int local_12;
  int local_10;
  int local_e;
  undefined4 local_c;
  undefined2 uStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_9,(uint)(param_9 >> 0x10));
  iStack6 = param_2;
  uStack4 = param_3;
  puVar1 = (undefined4 *)pass1_1030_5b5c(param_2,param_3);
  local_c = *puVar1;
  uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
  pass1_1008_3e94(param_6,(ushort *)CONCAT22(param_1,&local_10),(ushort *)CONCAT22(param_1,&local_e));
  pass1_1008_3e94((ushort *)CONCAT22(param_1,&local_c),(ushort *)CONCAT22(param_1,&local_14),
                  (ushort *)CONCAT22(param_1,&local_12));
  if ((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) && (local_10 < local_14 + -0x1)) {
    return 0x1;
  }
  PTR_LOOP_1050_50ca = (undefined *)0x6b8;
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_c3aa(ushort param_1,ushort param_2,ushort *param_3,ulong param_4,ulong param_5,ushort param_6)

{
  code **ppcVar1;
  int iVar2;
  ushort uVar3;
  uint uVar4;
  undefined2 uVar5;
  undefined *puVar6;
  uchar *puVar7;
  uchar *puVar8;
  uchar *puVar9;
  uint extraout_DX;
  uint uVar10;
  int unaff_DI;
  undefined2 uVar11;
  ulong uVar12;
  ushort *puVar13;
  ulong *puVar14;
  undefined uVar15;
  undefined uVar16;
  undefined2 uVar17;
  uint uVar18;
  ulong uVar19;
  uint uVar20;
  ulong uStack40;
  ulong uStack36;
  undefined4 *puStack32;
  undefined *puStack24;
  undefined local_4 [0x2];
  
  uVar12 = pass1_1030_bcae((ushort)local_4,param_6);
  puVar7 = (uchar *)(uVar12 >> 0x10);
  iVar2 = (int)uVar12;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
  uVar12 = *(ulong *)(iVar2 + 0x10);
  uVar15 = SUB41(param_3,0x0);
  uVar16 = (undefined)((ulong)param_3 >> 0x8);
  uVar11 = (undefined2)((ulong)param_3 >> 0x10);
  puVar8 = puVar7;
  uVar19 = param_5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar12,(uint)(uVar12 >> 0x10));
  puStack24 = local_4;
  pass1_1030_bcde(param_6,(ushort)puStack24,param_6,uVar12 & 0xffff | ZEXT24(puVar8) << 0x10,
                  (ushort *)CONCAT22(uVar11,CONCAT11(uVar16,uVar15)),uVar19);
  if ((int)puStack24 < 0x0) {
    PTR_LOOP_1050_50ca = (undefined *)0x6af;
    return;
  }
  if (0x1e < (int)puStack24) {
    uVar3 = 0x87;
    puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_6,puVar8,unaff_DI);
    uVar3 = pass1_1010_65d0(param_6,(ulong)puVar13,uVar3);
    if (uVar3 == 0x0) {
      puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x15);
      puVar9 = (uchar *)((ulong)puVar14 >> 0x10);
      uVar4 = (uint)puVar14;
      uVar11 = SUB42(&PTR_LOOP_1050_1038,0x0);
      pass1_1038_4d6e(CONCAT22(puVar7,iVar2),puVar14,uVar4,puVar9);
      puStack32 = (undefined4 *)CONCAT22(puVar9,uVar4);
      ppcVar1 = (code **)((int)*puStack32 + 0x10);
      uVar10 = uVar4;
      uVar18 = uVar4;
      puVar8 = puVar9;
      (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar4,puVar9);
      uStack36 = CONCAT22(extraout_DX,uVar10);
      uStack40 = 0x0;
      uVar10 = extraout_DX;
      while( true ) {
        if (uStack36 <= uStack40) {
          if (puStack32 != (undefined4 *)0x0) {
            ppcVar1 = (code **)*puStack32;
            (**ppcVar1)(uVar11,uVar4,(char)puVar9,0x1,uVar18,puVar8,puStack32,puStack32);
          }
          PTR_LOOP_1050_50ca = (undefined *)0x6b6;
          PTR_LOOP_1050_50cc = puStack24 + -0x1e;
          return;
        }
        uVar15 = (undefined)param_5;
        uVar16 = (undefined)(param_5 >> 0x8);
        uVar12 = uStack36;
        puVar13 = param_3;
        uVar17 = (int)(param_5 >> 0x10);
        pass1_1030_1d58((ulong)puStack32);
        uVar5 = (undefined2)uVar12;
        puVar6 = local_4;
        uVar11 = 0x1030;
        uVar20 = uVar10;
        pass1_1030_bcde(param_6,(ushort)puVar6,param_6,uVar12 & 0xffff | (ulong)uVar10 << 0x10,puVar13,
                        CONCAT22(uVar17,CONCAT11(uVar16,uVar15)));
        if ((0x0 < (int)puVar6) && ((int)puVar6 < 0x1f)) break;
        if ((int)puVar6 < (int)puStack24) {
          puStack24 = puVar6;
        }
        uStack40 = uStack40 + 0x1;
      }
      if (puStack32 == (undefined4 *)0x0) {
        return;
      }
      ppcVar1 = (code **)*puStack32;
      (**ppcVar1)(0x1030,uVar4,(char)puVar9,0x1,uVar18,puVar8,puStack32,puStack32,uVar5,uVar20);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_c522(ushort param_1,ushort param_2,ushort *param_3,ulong param_4,long param_5,ushort param_6)

{
  int iVar1;
  undefined *puVar2;
  uint uVar3;
  ulong uVar4;
  undefined local_4 [0x2];
  
  uVar4 = pass1_1030_bcae((ushort)local_4,param_6);
  uVar3 = (uint)(uVar4 >> 0x10);
  iVar1 = (int)uVar4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
  uVar4 = *(ulong *)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar4,(uint)(uVar4 >> 0x10));
  puVar2 = local_4;
  pass1_1030_bcde(param_6,(ushort)puVar2,param_6,uVar4 & 0xffff | (ulong)uVar3 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (undefined *)0x6af;
  }
  else {
    if ((int)puVar2 < 0x1f) {
      return;
    }
    PTR_LOOP_1050_50ca = (undefined *)0x6b6;
    PTR_LOOP_1050_50cc = puVar2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far
pass1_1028_c5a6(ushort param_1,ushort param_2,int param_3,ushort *param_4,long param_5,uint param_6,uint param_7,
               ushort param_8)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  int iStack14;
  ulong uStack10;
  
  pass1_1030_627e(param_8,param_6,param_7,_PTR_LOOP_1050_5740,param_4,param_5);
  uVar2 = param_7 | param_6;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6,param_7);
    uStack10 = CONCAT22(uVar2,param_6);
    iStack14 = 0x7a;
    if (0x0 < *(int *)((int)param_4 + 0x4)) {
      if (param_3 == 0x7b) {
        param_3 = 0x7e;
      }
      else {
        if (param_3 == 0x7c) {
          param_3 = 0x7d;
        }
      }
      iStack14 = 0x7f;
    }
    if (uStack10 != 0x0) {
      uVar3 = struct_op_1030_73a8(uStack10);
      if ((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_3)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

BOOL16 __stdcall16far
pass1_1028_c64a(ulong param_1,ulong *param_2,uint param_3,ushort param_4,ushort param_5,long param_6,ushort param_7)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined local_e [0x2];
  int local_c;
  int local_a;
  undefined4 local_8;
  undefined2 uStack4;
  
  local_8 = *param_2;
  uStack4 = *(undefined2 *)(param_2 + 0x1);
  pass1_1008_3eb4((ushort *)CONCAT22(param_7,&local_8),(ushort *)CONCAT22(param_7,local_e),
                  (ushort *)CONCAT22(param_7,&local_c),(ushort *)CONCAT22(param_7,&local_a));
  local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  BVar1 = pass1_1028_c5a6(uVar2,uVar3,0x7b,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,param_7);
  if (BVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
    BVar1 = pass1_1028_c5a6(uVar2,uVar3,0x7b,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,param_7
                           );
    if (BVar1 == 0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8._2_2_ = local_c;
      BVar1 = pass1_1028_c5a6(uVar2,uVar3,0x7c,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,
                              param_7);
      if (BVar1 == 0x0) {
        local_8 = CONCAT22(local_8._2_2_,local_a + 0x1);
        BVar1 = pass1_1028_c5a6(uVar2,uVar3,0x7c,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,
                                param_7);
        if (BVar1 == 0x0) {
          return BVar1;
        }
      }
    }
  }
  return 0x1;
}



void __stdcall16far pass1_1028_c724(ulong param_1)

{
  uint uVar1;
  undefined4 uVar2;
  astruct_295 *iVar3;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_295 *)param_1;
  uVar2 = iVar3->field_0x14;
  if (*(int *)((int)uVar2 + 0xac) != 0x0) {
    return;
  }
  uVar2 = iVar3->field_0x14;
  uVar1 = *(uint *)((int)uVar2 + 0xa6);
  if (uVar1 == 0xd) {
    uVar2 = iVar3->field_0x14;
    *(undefined2 *)((int)uVar2 + 0xac) = 0x1;
    goto LAB_1028_c770;
  }
  if (uVar1 < 0xe) {
    if ((char)uVar1 == '\0') goto LAB_1028_c770;
    if ((char)uVar1 == '\a') {
      uVar2 = iVar3->field_0x14;
      *(undefined2 *)((int)uVar2 + 0xac) = 0xa;
      goto LAB_1028_c770;
    }
  }
  uVar2 = iVar3->field_0x14;
  *(undefined2 *)((int)uVar2 + 0xac) = 0x5;
LAB_1028_c770:
  uVar2 = iVar3->field_0x14;
  if (*(int *)((int)uVar2 + 0xac) == 0x0) {
    uVar2 = iVar3->field_0x14;
    if (*(int *)((int)uVar2 + 0xa8) != 0x0) {
      uVar2 = iVar3->field_0x14;
      *(undefined2 *)((int)uVar2 + 0xac) = 0x1;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_c7b6(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort *param_5,long param_6)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  ulong uVar4;
  byte bStack27;
  undefined4 local_a;
  undefined4 uStack6;
  
  puVar1 = &local_a;
  pass1_1030_64ce(param_1,puVar1,param_2,_PTR_LOOP_1050_5740,param_5,param_6,(ulong *)CONCAT22(param_1,puVar1));
  uStack6 = *puVar1;
  uVar3 = *(uint *)((int)puVar1 + 0x2);
  bStack27 = (byte)((ulong)uStack6 >> 0x18);
  uVar2 = (uint)bStack27;
  if (bStack27 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack6,uVar3);
  uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2));
  uVar3 = (uint)(uVar4 >> 0x10);
  if ((uVar3 | (uint)uVar4) != 0x0) {
    switch(*(undefined2 *)((uint)uVar4 + 0xc)) {
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

void __stdcall16far pass1_1028_c89c(ulong param_1,ushort *param_2,ulong *param_3,int param_4,ushort param_5)

{
  ulong *puVar1;
  undefined2 extraout_DX;
  undefined2 uVar2;
  ulong local_16 [0x3];
  long lStack10;
  undefined4 uStack6;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_4);
  lStack10 = *(long *)(param_4 + 0x8);
  puVar1 = local_16;
  uVar2 = extraout_DX;
  pass1_1030_64ce(param_5,puVar1,extraout_DX,_PTR_LOOP_1050_5740,param_2,lStack10,(ulong *)CONCAT22(param_5,puVar1));
  *param_3 = *puVar1;
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1028_c8ee(ushort param_1,ushort param_2,ushort param_3,int param_4,ushort *param_5)

{
  ushort local_8;
  undefined4 local_6;
  
  pass1_1008_3eb4(param_5,(ushort *)CONCAT22(param_1,&local_8),(ushort *)CONCAT22(param_1,&local_6),
                  (ushort *)CONCAT22(param_1,(int)&local_6 + 0x2));
  if (param_4 == 0x1) {
    local_8 = local_8 + 0x1;
  }
  else {
    if (param_4 == 0x2) {
      local_6 = local_6 & 0xffff0000 | (ulong)((int)local_6 - 0x1);
    }
    else {
      if (param_4 == 0x3) {
        local_6 = local_6 & 0xffff0000 | (ulong)((int)local_6 + 0x1);
      }
      else {
        if (param_4 == 0x4) {
          local_6 = local_6 & 0xffff | (ulong)(local_6._2_2_ + 0x1) << 0x10;
        }
        else {
          if (param_4 == 0x5) {
            local_6 = local_6 & 0xffff | (ulong)(local_6._2_2_ - 0x1) << 0x10;
          }
        }
      }
    }
  }
  pass1_1008_3e76(param_5,local_8,(ushort)local_6,(ushort)(local_6 >> 0x10));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_c952(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uint uVar1;
  undefined4 uVar2;
  ulong uVar4;
  astruct_600 *uVar3;
  BOOL16 BVar5;
  astruct_600 *paVar6;
  astruct_600 *paVar7;
  undefined4 uVar8;
  ulong uVar9;
  uint uVar10;
  int iVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  undefined4 uStack30;
  ushort uStack16;
  uint uStack14;
  
  uVar12 = (undefined2)(param_1 >> 0x10);
  iVar11 = (int)param_1;
  uVar2 = *(undefined4 *)(iVar11 + 0x14);
  uVar3 = (astruct_600 *)uVar2;
  uVar10 = *(uint *)(iVar11 + 0x16) | (uint)uVar3;
  if (uVar10 != 0x0) {
    uVar8 = uVar2;
    pass1_1028_b58e(param_1);
    uVar4 = *(ulong *)((int)uVar8 + 0x2e);
    uStack14 = (uint)uVar4;
    if (((*(uint *)((int)uVar8 + 0x30) | uStack14) != 0x0) &&
       (uVar13 = (undefined2)(uVar4 >> 0x10), *(int *)(uStack14 + 0x206) == 0x0)) {
      BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(iVar11 + 0xc),0x32);
      if (BVar5 == 0x0) {
        BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(iVar11 + 0xc),0x33);
        if ((BVar5 != 0x0) && ((int)((qword)*_PTR_LOOP_1050_65e2 % 0x5) == 0x0)) {
          return;
        }
      }
      else {
        if ((int)((qword)*_PTR_LOOP_1050_65e2 % 0xa) == 0x0) {
          return;
        }
      }
      uVar12 = (undefined2)((ulong)uVar2 >> 0x10);
      if (*(int *)(uStack14 + 0x204) == 0x0) {
        for (uStack16 = 0x0; (int)uStack16 < 0x25; uStack16 = uStack16 + 0x1) {
          uStack30 = *(ulong *)(&uVar3->field_0x0 + uStack16 * 0x4);
          paVar7 = (astruct_600 *)uStack30;
          uVar10 = *(uint *)(&uVar3->field_0x2 + uStack16 * 0x4) | (uint)paVar7;
          if (uVar10 != 0x0) {
            uVar9 = uStack30;
            empty_1038_540a();
            uStack30._2_2_ = (uint)(uStack30 >> 0x10);
            paVar6 = uVar3;
            if ((uVar9 & 0xffff | (ulong)uVar10 << 0x10) < uStack30) {
              paVar6 = (astruct_600 *)((int)paVar7 - (int)(astruct_600 *)uVar9);
              param_3 = (uStack30._2_2_ - uVar10) - (uint)(paVar7 < (astruct_600 *)uVar9);
              pass1_1038_52b8(uVar4,CONCAT22(param_3,paVar6),0x21,(ushort)paVar6,param_3,(ushort)&PTR_LOOP_1050_1038,
                              param_4);
              uStack30 = CONCAT22((uStack30._2_2_ - param_3) - (uint)(paVar7 < paVar6),(int)paVar7 - (int)paVar6);
            }
            if ((uStack30._2_2_ | (uint)uStack30) != 0x0) {
              pass1_1038_52b8(uVar4,uStack30,uStack16,(ushort)paVar6,param_3,(ushort)&PTR_LOOP_1050_1038,param_4);
            }
          }
        }
      }
      else {
        uVar10 = uVar3->field_0x8c;
        uVar1 = uVar3->field_0x8e;
        if ((uVar1 | uVar10) != 0x0) {
          pass1_1038_52b8(uVar4,CONCAT22(uVar1,uVar10),0x23,param_2,param_3,(ushort)&PTR_LOOP_1050_1038,param_4);
        }
        uVar10 = uVar3->field_0x90;
        uVar1 = uVar3->field_0x92;
        if ((uVar1 | uVar10) != 0x0) {
          pass1_1038_52b8(uVar4,CONCAT22(uVar1,uVar10),0x24,param_2,param_3,(ushort)&PTR_LOOP_1050_1038,param_4);
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_cb04(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ulong uVar1;
  undefined4 uVar2;
  ulong uVar3;
  uint uVar4;
  ulong uVar5;
  undefined4 uVar6;
  long lVar7;
  uint extraout_DX;
  ushort uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  bool bVar11;
  ushort *puVar12;
  uchar *puStack52;
  uint uStack38;
  uchar *puStack36;
  int iStack22;
  uint uStack18;
  uchar *puStack16;
  uint uStack14;
  
  uVar1 = *(ulong *)((int)param_1 + 0x14);
  if (uVar1 != 0x0) {
    uVar5 = uVar1;
    pass1_1028_b58e(param_1);
    uVar3 = uVar5 & 0xffff | (ulong)extraout_DX << 0x10;
    uVar2 = *(undefined4 *)((int)uVar5 + 0x2e);
    puStack52 = *(uchar **)((int)uVar5 + 0x30);
    uStack14 = (uint)uVar2;
    uStack18 = (uint)puStack52 | uStack14;
    if (uStack18 != 0x0) {
      uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
      if (*(int *)(uStack14 + 0x206) != 0x0) {
        return;
      }
      uVar8 = (ushort)uVar1;
      uVar10 = (undefined2)(uVar1 >> 0x10);
      if (*(int *)(uStack14 + 0x204) != 0x0) {
        uVar2 = *(undefined4 *)(uVar8 + 0x8c);
        uVar6 = uVar2;
        empty_1038_540a();
        puStack36 = (uchar *)((ulong)uVar2 >> 0x10);
        if ((puStack52 <= puStack36) &&
           ((uVar4 = (uint)uVar6, uStack38 = (uint)uVar2, puStack52 < puStack36 || (uVar4 < uStack38)))) {
          pass1_1030_7d7c(uVar3,uStack38 - uVar4,CONCAT22(0x23,puStack36 + (-(uint)(uStack38 < uVar4) - (int)puStack52))
                          ,uVar4,puStack52,param_2,param_3,param_4);
          puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,puStack52,param_3);
          puStack52 = (uchar *)((ulong)puVar12 >> 0x10);
          pass1_1010_043a((ulong)puVar12 & 0xffff | ZEXT24(puStack52) << 0x10,*(long *)(uStack14 + 0x4),0x12,param_4);
        }
        uVar2 = *(undefined4 *)(uVar8 + 0x90);
        uVar6 = uVar2;
        empty_1038_540a();
        puStack36 = (uchar *)((ulong)uVar2 >> 0x10);
        if ((puStack52 <= puStack36) &&
           ((uVar4 = (uint)uVar6, uStack38 = (uint)uVar2, puStack52 < puStack36 || (uVar4 < uStack38)))) {
          pass1_1030_7d7c(uVar3,uStack38 - uVar4,CONCAT22(0x24,puStack36 + (-(uint)(uStack38 < uVar4) - (int)puStack52))
                          ,uVar4,puStack52,param_2,param_3,param_4);
        }
        return;
      }
      empty_1038_540a();
      puStack16 = puStack52;
      for (iStack22 = 0x11; iStack22 < 0x25; iStack22 = iStack22 + 0x1) {
        uVar1 = *(ulong *)(iStack22 * 0x4 + uVar8);
        uVar5 = uVar1;
        empty_1038_540a();
        uVar5 = uVar5 & 0xffff | ZEXT24(puStack52) << 0x10;
        puStack52 = (uchar *)(uVar1 >> 0x10);
        if (uVar5 < uVar1) {
          if ((((iStack22 == 0x23) || (iStack22 == 0x24)) || (puStack16 < puStack52)) ||
             ((uVar4 = (uint)uVar1, puStack16 <= puStack52 && (uStack18 < uVar4)))) {
            lVar7 = uVar1 - uVar5;
            uVar4 = (uint)lVar7;
            pass1_1030_7d7c(uVar3,uVar4,CONCAT22(iStack22,(int)((ulong)lVar7 >> 0x10)),uVar4,puStack52,uVar8,param_3,
                            param_4);
            if (iStack22 == 0x23) {
              puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,puStack52,param_3);
              puStack52 = (uchar *)((ulong)puVar12 >> 0x10);
              pass1_1010_043a((ulong)puVar12 & 0xffff | ZEXT24(puStack52) << 0x10,*(long *)(uStack14 + 0x4),0x12,param_4
                             );
            }
          }
          else {
            bVar11 = uStack18 < uVar4;
            uStack18 = uStack18 - uVar4;
            puStack16 = puStack16 + (-(uint)bVar11 - (int)puStack52);
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

void __stdcall16far pass1_1028_ccd0(uchar param_1,ushort param_2,ulong param_3,ushort *param_4)

{
  code **ppcVar1;
  ushort *puVar2;
  undefined *puVar3;
  int iVar4;
  uchar *extraout_DX;
  uchar *puVar5;
  uint uVar6;
  int iVar7;
  undefined2 extraout_DX_00;
  int unaff_DI;
  ushort uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  undefined2 local_178;
  undefined2 uStack374;
  int iStack84;
  ushort uStack72;
  ushort uStack64;
  int iStack62;
  undefined4 uStack60;
  undefined4 *puStack56;
  ulong uStack52;
  ushort *puStack48;
  undefined local_2c [0xc];
  int local_20;
  int local_1e;
  undefined4 uStack28;
  undefined4 uStack24;
  undefined4 uStack20;
  int iStack16;
  int iStack14;
  ushort uStack12;
  ushort uStack10;
  ushort local_8;
  int local_6;
  int local_4;
  
  puVar2 = &local_8;
  pass1_1008_3eb4(param_4,(ushort *)CONCAT22(param_2,puVar2),(ushort *)CONCAT22(param_2,&local_6),
                  (ushort *)CONCAT22(param_2,&local_4));
  pass1_1028_b58e(param_3);
  uStack20 = CONCAT22(extraout_DX,puVar2);
  uStack24 = *(undefined4 *)(puVar2 + 0x17);
  uStack28 = *(undefined4 *)((int)uStack24 + 0x4);
  puVar5 = extraout_DX;
  pass1_1028_c1f8(param_2,(int)&local_20,(ushort)extraout_DX,param_3,(ushort *)CONCAT22(param_2,&local_20),
                  (ushort *)CONCAT22(param_2,&local_1e));
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
  pass1_1008_6c90((ushort *)CONCAT22(param_2,local_2c));
  pass1_1008_6cec((ushort *)CONCAT22(param_2,local_2c),local_8,CONCAT22(iStack14,iStack16),local_8,
                  CONCAT22(uStack10,uStack12));
  puStack48 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_2,puVar5,unaff_DI);
  uVar6 = (uint)((ulong)puStack48 >> 0x10);
  uStack52 = *(ulong *)((int)puStack48 + 0x20);
  puVar3 = local_2c;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(param_2,puVar3),uStack52,param_2);
  puStack56 = (undefined4 *)CONCAT22(uVar6,puVar3);
  if ((uVar6 | (uint)puVar3) != 0x0) {
    uStack60 = 0x0;
    iStack62 = 0x0;
    for (uStack64 = uStack12; (int)uStack64 <= iStack16; uStack64 = uStack64 + 0x1) {
      for (uStack72 = uStack10; iVar4 = iStack62, (int)uStack72 <= iStack14; uStack72 = uStack72 + 0x1) {
        iVar7 = iStack62 >> 0xf;
        ppcVar1 = (code **)((int)*puStack56 + 0x4);
        iStack62 = iStack62 + 0x1;
        (**ppcVar1)(0x1030,(int)puStack56,(int)((ulong)puStack56 >> 0x10),iVar4,iVar7);
        uStack60 = CONCAT22(extraout_DX_00,iVar4);
        uStack60._3_1_ = (char)((uint)extraout_DX_00 >> 0x8);
        if (uStack60._3_1_ == '\0') {
          iStack84 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_4,local_8,uStack64,uStack72);
            uVar11 = (undefined2)uStack52;
            uVar12 = (undefined2)(uStack52 >> 0x10);
            uVar9 = (undefined2)uStack28;
            uVar10 = (undefined2)((ulong)uStack28 >> 0x10);
            uVar8 = 0x6;
          }
          else {
            if (iVar4 == 0x8) {
              pass1_1008_3e76(param_4,local_8,uStack64,uStack72);
              uVar11 = (undefined2)uStack52;
              uVar12 = (undefined2)(uStack52 >> 0x10);
              uVar9 = (undefined2)uStack28;
              uVar10 = (undefined2)((ulong)uStack28 >> 0x10);
              uVar8 = 0x7;
            }
            else {
              if (iVar4 != 0x9) goto LAB_1028_ce2c;
              pass1_1008_3e76(param_4,local_8,uStack64,uStack72);
              uVar11 = (undefined2)uStack52;
              uVar12 = (undefined2)(uStack52 >> 0x10);
              uVar9 = (undefined2)uStack28;
              uVar10 = (undefined2)((ulong)uStack28 >> 0x10);
              uVar8 = 0x8;
            }
          }
          struct_op_1028_87f0(param_2,param_1,(astruct_97 *)CONCAT22(param_2,&local_178),0x0,0x0,uVar8,(ulong *)param_4,
                              (ushort)((ulong)param_4 >> 0x10),CONCAT22(uVar10,uVar9),CONCAT22(uVar12,uVar11));
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,&local_178));
          local_178 = 0x389a;
          uStack374 = 0x1008;
        }
LAB_1028_ce2c:
      }
    }
  }
  return;
}



ushort __stdcall16far pass1_1028_ced2(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uint uVar1;
  bool bVar2;
  bool bVar3;
  ulong uVar4;
  uint uVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  bVar2 = (*(byte *)((int)param_1 + 0x1a) & 0x2) == 0x0;
  if (bVar2) {
    uVar6 = 0x0;
    uVar7 = 0x23;
    uVar5 = 0x1;
    uVar4 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    param_4 = 0x1030;
    pass1_1030_7d7c(uVar4,uVar5,CONCAT22(uVar7,uVar6),(uint)uVar4,(uchar *)(uVar4 >> 0x10),param_2,param_3,param_5);
  }
  bVar3 = (*(byte *)((int)param_1 + 0x1a) & 0x1) == 0x0;
  if (bVar3) {
    uVar6 = 0x0;
    uVar7 = 0xe;
    uVar5 = 0x1;
    uVar4 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    param_4 = 0x1030;
    pass1_1030_7d7c(uVar4,uVar5,CONCAT22(uVar7,uVar6),(uint)uVar4,(uchar *)(uVar4 >> 0x10),param_2,param_3,param_5);
  }
  if (bVar3 || bVar2) {
    pass1_1028_bdac(param_1,0x6,param_4);
    return 0x0;
  }
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1028_cf44(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1028_cfd2(ulong *param_1,ulong param_2)

{
  *param_1 = param_2;
  *(undefined4 *)((int)param_1 + 0x4) = 0x0;
  return;
}



void __stdcall16far pass1_1028_cff2(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  puVar1 = (undefined4 *)*(uint *)((int)param_1 + 0x4);
  uVar2 = *(uint *)((int)param_1 + 0x6);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  return;
}



void __stdcall16far pass1_1028_d01a(ulong *param_1)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  uint uVar4;
  uint extraout_DX;
  undefined4 *puStack14;
  
  puVar1 = (undefined4 *)**(ulong **)*param_1;
  puStack14 = puVar1;
  while( true ) {
    uVar4 = (uint)puStack14;
    fn_ptr_1028_d728((ulong)puVar1);
    puStack14 = (undefined4 *)CONCAT22(extraout_DX,uVar4);
    if ((extraout_DX | uVar4) == 0x0) break;
    uVar3 = *puStack14;
    ppcVar2 = (code **)uVar3 + 0x2;
    (**ppcVar2)();
    if (puStack14 != (undefined4 *)0x0) {
      ppcVar2 = (code **)uVar3;
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_d078(ushort param_1,ulong param_2,ulong param_3)

{
  code **ppcVar1;
  uchar *extraout_DX;
  uchar *puVar2;
  int iVar3;
  undefined2 uVar4;
  ushort *puVar5;
  ulong uVar6;
  undefined local_16 [0x4];
  undefined4 *puStack18;
  uchar *puStack16;
  undefined4 uStack14;
  uint uStack10;
  uint uStack8;
  undefined4 *puStack6;
  uint uStack4;
  
  uVar4 = (undefined2)(param_2 >> 0x10);
  iVar3 = (int)param_2;
  puStack6 = (undefined4 *)*(uint *)(iVar3 + 0x4);
  puVar2 = *(uchar **)(iVar3 + 0x6);
  uStack14 = CONCAT22(puVar2,puStack6);
  puStack18 = puStack6;
  puStack16 = puVar2;
  if (((uint)puVar2 | (uint)puStack6) != 0x0) {
    ppcVar1 = (code **)*puStack6;
    (**ppcVar1)();
    puVar2 = extraout_DX;
  }
  mem_op_1000_179c(0x1c,puVar2,0x1000);
  uStack4 = (uint)puVar2 | (uint)puStack6;
  puStack18 = puStack6;
  puStack16 = puVar2;
  if (uStack4 == 0x0) {
    puStack6 = (undefined4 *)0x0;
    uStack4 = 0x0;
  }
  else {
    struct_op_1008_8e9e((astruct_78 *)CONCAT22(puVar2,puStack6),0x6,0x24);
  }
  *(undefined2 *)(iVar3 + 0x4) = puStack6;
  *(uint *)(iVar3 + 0x6) = uStack4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_3,(uint)(param_3 >> 0x10));
  if ((uchar *)(uStack4 | (uint)puStack6) == (uchar *)0x0) {
    puVar5 = pass1_1018_dcf6((ushort *)CONCAT22(param_1,local_16));
    uVar6 = pass1_1018_dd1e(param_1,(uint)local_16,(uchar *)((ulong)puVar5 >> 0x10),(ushort)local_16,param_1,0x0,0xa0000
                           );
    pass1_1008_8faa(*(undefined4 *)(iVar3 + 0x4),uVar6);
    return;
  }
  uVar6 = pass1_1038_565e(param_1,(uchar *)(uStack4 | (uint)puStack6),CONCAT22(uStack4,puStack6));
  uStack8 = (uint)(uVar6 >> 0x10);
  uStack10 = (uint)uVar6;
  if ((uStack8 | uStack10) != 0x0) {
    pass1_1028_d172(param_1,param_2,uVar6 & 0xffff | (ulong)uStack8 << 0x10);
  }
  return;
}



void __stdcall16far pass1_1028_d172(ushort param_1,ulong param_2,ulong param_3)

{
  uint uVar1;
  long lVar2;
  ulong uVar3;
  undefined local_e [0x8];
  undefined local_6 [0x4];
  
  pass1_1018_dcf6((ushort *)CONCAT22(param_1,local_6));
  pass1_1008_5784((ulong *)CONCAT22(param_1,local_e),param_3);
  while( true ) {
    lVar2 = pass1_1008_5b12(local_e,param_1);
    uVar1 = (uint)((ulong)lVar2 >> 0x10);
    if (lVar2 == 0x0) break;
    uVar3 = pass1_1018_dd1e(param_1,(uint)local_6,(uchar *)(uVar1 | (uint)lVar2),(ushort)local_6,param_1,0x0,
                            (ulong)*(uint *)((uint)lVar2 + 0x4) << 0x10);
    pass1_1008_8faa(*(undefined4 *)((int)param_2 + 0x4),uVar3);
  }
  return;
}



astruct_100 * __stdcall16far struct_op_1028_d1dc(ushort param_1,uchar param_2,astruct_100 *param_3,ushort param_4)

{
  astruct_101 *iVar1;
  uchar *puVar1;
  ushort in_stack_0000fffa;
  
  puVar1 = (uchar *)((ulong)param_3 >> 0x10);
  iVar1 = (astruct_101 *)param_3;
  param_3->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_4;
  iVar1->field_0x6 = 0x0;
  param_3->field_0x0 = 0x6ad2;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  sys_1000_3f9c(&iVar1->field_0x8,puVar1,0x5160,(ushort)&USHORT_1050_1050,in_stack_0000fffa,&stack0xfffe,puVar1,0x1000,
                param_1,param_2);
  return param_3;
}

