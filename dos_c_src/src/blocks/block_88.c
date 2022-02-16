

int __stdcall16far pass1_1030_cde8(int param_1,ushort param_2,int param_3)

{
  int iVar1;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return -0x1;
    }
    iVar1 = iStack4 * 0xc + param_1;
    if ((*(int *)(iVar1 + 0x24) == param_3) && (*(int *)(iVar1 + 0x26) == 0x0)) break;
    iStack4 = iStack4 + 0x1;
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps

int __stdcall16far pass1_1030_ce2e(int param_1,ushort param_2,int param_3)

{
  int iVar1;
  undefined4 uStack6;
  
  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1)) {
    iVar1 = (int)uStack6 * 0xc + param_1;
    if ((*(int *)(iVar1 + 0x24) == param_3) && (*(int *)(iVar1 + 0x26) == 0x0)) {
      uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



void __stdcall16far pass1_1030_ce72(ulong param_1,int param_2,ulong param_3,int param_4)

{
  long lVar1;
  astruct_300 *iVar2;
  int iStack10;
  
  lVar1 = *(long *)((int)param_3 + 0x4);
  iStack10 = 0x0;
  do {
    if (0x9 < iStack10) {
      return;
    }
    iVar2 = (astruct_300 *)(iStack10 * 0xc + (int)param_1);
    if ((iVar2->field_0x24 == param_4) && (iVar2->field_0x28 == 0x0)) {
      iVar2->field_0x28 = lVar1;
      if (param_4 == 0x4) {
        iVar2->field_0x26 = 0x2;
      }
      else {
        *(undefined2 *)((int)param_1 + iStack10 * 0xc + 0x26) = 0x1;
      }
      param_2 = param_2 + -0x1;
      if (param_2 == 0x0) {
        return;
      }
    }
    iStack10 = iStack10 + 0x1;
  } while( true );
}



void __stdcall16far pass1_1030_cef8(ulong param_1,ulong param_2,ushort param_3,int param_4)

{
  undefined2 uVar1;
  int iVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  *(ushort *)(iVar2 + param_4 * 0xc + 0x26) = param_3;
  uVar4 = (undefined2)(param_2 >> 0x10);
  uVar1 = *(undefined2 *)((int)param_2 + 0x6);
  *(undefined2 *)(iVar2 + param_4 * 0xc + 0x28) = *(undefined2 *)((int)param_2 + 0x4);
  *(undefined2 *)(iVar2 + param_4 * 0xc + 0x2a) = uVar1;
  return;
}



ushort __stdcall16far pass1_1030_cf3a(ulong param_1,int param_2)

{
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if (*(int *)((int)param_1 + iStack4 * 0xc + 0x24) == param_2) break;
    iStack4 = iStack4 + 0x1;
  }
  return 0x1;
}



void __stdcall16far pass1_1030_cf78(ulong param_1,uint param_2)

{
  ulong uVar1;
  uint extraout_DX;
  astruct_680 *iVar3;
  undefined2 uVar2;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    uVar1 = (ulong)param_2;
    uVar2 = (undefined2)(param_1 >> 0x10);
    if (*(uint *)((int)param_1 + iStack4 * 0xc + 0x24) == param_2) break;
    iStack4 = iStack4 + 0x1;
  }
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_4900(*(ulong *)((int)uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c(uVar1 & 0xffff | (ulong)extraout_DX << 0x10,0x1,param_2);
  }
  iVar3 = (astruct_680 *)(iStack4 * 0xc + (int)param_1);
  iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = 0x0;
  return;
}



void __stdcall16far pass1_1030_d00c(int param_1,ushort param_2,uint param_3)

{
  ulong uVar1;
  uint extraout_DX;
  astruct_696 *local_BX_40;
  int iVar2;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    iVar2 = iStack4 * 0xc + param_1;
    if ((*(int *)(iVar2 + 0x26) == 0x0) && (uVar1 = (ulong)param_3, *(uint *)(iVar2 + 0x24) == param_3)) break;
    iStack4 = iStack4 + 0x1;
  }
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  if (param_3 == 0x5) {
    pass1_1038_4900(*(ulong *)((int)uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c(uVar1 & 0xffff | (ulong)extraout_DX << 0x10,0x1,param_3);
  }
  local_BX_40 = (astruct_696 *)(iStack4 * 0xc + param_1);
  local_BX_40->field_0x20 = 0x0;
  local_BX_40->field_0x24 = 0x0;
  local_BX_40->field_0x26 = 0x0;
  return;
}



ushort __stdcall16far pass1_1030_d0a8(ulong param_1)

{
  ushort uVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  uVar1 = *(ushort *)((int)param_1 + 0x98);
  pass1_1030_d56a(param_1 & 0xffff | (ulong)uVar2 << 0x10);
  return uVar1;
}



// WARNING: Could not reconcile some variable overlaps

int __stdcall16far pass1_1030_d0c6(ulong param_1)

{
  undefined4 uStack6;
  
  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1)) {
    if (*(long *)((int)param_1 + (int)uStack6 * 0xc + 0x20) != 0x0) {
      uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



// WARNING: Could not reconcile some variable overlaps

int __stdcall16far pass1_1030_d102(int param_1,ushort param_2)

{
  int iVar1;
  undefined4 uStack6;
  
  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1)) {
    iVar1 = (int)uStack6 * 0xc + param_1;
    if ((*(long *)(iVar1 + 0x20) != 0x0) && (*(int *)(iVar1 + 0x26) != 0x0)) {
      uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



// WARNING: Could not reconcile some variable overlaps

int __stdcall16far pass1_1030_d144(ulong param_1)

{
  undefined4 uStack6;
  
  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1)) {
    if (*(long *)((int)param_1 + (int)uStack6 * 0xc + 0x20) == 0x0) {
      uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_d180(ulong param_1,uint param_2)

{
  int iVar1;
  uint uVar2;
  uchar *extraout_DX;
  ushort uVar3;
  int iVar4;
  ushort uVar5;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    uVar5 = (ushort)(param_1 >> 0x10);
    uVar3 = (ushort)param_1;
    if ((*(uint *)(uVar3 + iStack4 * 0xc + 0x22) | *(uint *)(uVar3 + iStack4 * 0xc + 0x20)) == 0x0) break;
    iStack4 = iStack4 + 0x1;
  }
  uVar2 = *_PTR_LOOP_1050_65e2;
  iVar1 = *(int *)((int)_PTR_LOOP_1050_65e2 + 0x2);
  iVar4 = iStack4 * 0xc + uVar3;
  *(int *)(iVar4 + 0x20) = uVar2 + 0xc8;
  *(int *)(iVar4 + 0x22) = iVar1 + (uint)(0xff37 < uVar2);
  *(uint *)(iVar4 + 0x24) = param_2;
  uVar2 = param_2;
  pass1_1030_d340(uVar3,uVar5,param_1 & 0xffff0000 | (ulong)(iVar4 + 0x20));
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_48e0(*(ulong *)(uVar2 + 0x2e),0x1);
    return;
  }
  pass1_1030_7c50(CONCAT22(extraout_DX,uVar2),0x1,param_2,uVar2,extraout_DX);
  return;
}



ushort __stdcall16far pass1_1030_d230(ulong param_1)

{
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x1;
    }
    if (*(long *)((int)param_1 + iStack4 * 0xc + 0x20) == 0x0) break;
    iStack4 = iStack4 + 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_d26c(ulong param_1,ushort param_2)

{
  ulong *puVar1;
  ulong uVar2;
  int iVar3;
  ulong uVar4;
  undefined2 extraout_DX;
  int iVar5;
  int iStack8;
  
  uVar2 = *_PTR_LOOP_1050_65e2;
  for (iStack8 = 0x0; iStack8 < 0xa; iStack8 = iStack8 + 0x1) {
    iVar5 = iStack8 * 0xc + (int)param_1;
    if (((*(uint *)(iVar5 + 0x22) | *(uint *)(iVar5 + 0x20)) != 0x0) &&
       (puVar1 = (ulong *)(iVar5 + 0x20), *puVar1 < uVar2 || *puVar1 == uVar2)) {
      uVar4 = uVar2;
      pass1_1030_d3b2((int)param_1,param_1._2_2_,iStack8,(int)uVar2,param_2);
      iVar3 = (int)uVar4;
      if (iVar3 == 0x0) {
        pass1_1028_b58e(param_1);
        if (*(int *)(iVar5 + 0x24) == 0x5) {
          pass1_1038_4900(*(ulong *)(iVar3 + 0x2e));
        }
        else {
          pass1_1030_6e9c(CONCAT22(extraout_DX,iVar3),0x1,*(int *)((int)param_1 + iStack8 * 0xc + 0x24));
        }
        iVar5 = iStack8 * 0xc + (int)param_1;
        *(undefined4 *)(iVar5 + 0x20) = 0x0;
        *(undefined2 *)(iVar5 + 0x24) = 0x0;
        *(undefined2 *)(iVar5 + 0x26) = 0x0;
      }
    }
  }
  return;
}



void __stdcall16far pass1_1030_d340(ushort param_1,ushort param_2,ulong param_3)

{
  int iVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_3 >> 0x10);
  iVar2 = (int)param_3;
  iVar1 = *(int *)(iVar2 + 0x4);
  if (((0x0 < iVar1) && (!SBORROW2(iVar1,0x1))) && ((iVar1 == 0x4 || iVar1 + -0x1 < 0x3 || (iVar1 == 0xc)))) {
    *(undefined2 *)(iVar2 + 0x6) = 0x0;
    return;
  }
  *(undefined2 *)(iVar2 + 0x6) = 0x1;
  return;
}



ushort __stdcall16far pass1_1030_d36e(ulong param_1,int param_2)

{
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if ((iStack4 != param_2) && (*(int *)((int)param_1 + iStack4 * 0xc + 0x24) == 0x8)) break;
    iStack4 = iStack4 + 0x1;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_d3b2(int param_1,ushort param_2,int param_3,int param_4,ushort param_5)

{
  int iVar1;
  code **ppcVar2;
  bool bVar3;
  ushort uVar4;
  uint uVar5;
  undefined2 extraout_DX;
  uchar *puVar6;
  undefined2 extraout_DX_00;
  undefined2 extraout_DX_01;
  uint uVar7;
  undefined2 uVar8;
  ulong *puVar9;
  ulong uVar10;
  ulong uVar11;
  undefined4 *puStack26;
  ulong uStack18;
  undefined4 uStack14;
  
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  uVar11 = *(ulong *)(param_4 + 0x2e);
  uVar4 = pass1_1030_d36e(CONCAT22(param_2,param_1),param_3);
  if (uVar4 == 0x0) {
    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
    uVar5 = (uint)puVar9;
    pass1_1038_4d6e(uVar11,puVar9,uVar5,puVar6);
    puStack26 = (undefined4 *)CONCAT22(puVar6,uVar5);
    ppcVar2 = (code **)((int)*puStack26 + 0x10);
    uVar7 = uVar5;
    (**ppcVar2)((int)&PTR_LOOP_1050_1038,uVar5,puVar6);
    uStack18 = CONCAT22(extraout_DX_00,uVar7);
    bVar3 = false;
    for (uStack14 = 0x0; uStack14 < uStack18; uStack14 = uStack14 + 0x1) {
      uVar10 = pass1_1030_1d7c((int)uStack14,uStack14._2_2_,(ulong)puStack26);
      uVar7 = (uint)(uVar10 >> 0x10);
      if ((((uVar7 | (uint)uVar10) != 0x0) && (*(long *)((uint)uVar10 + 0x4) != *(long *)(param_1 + 0x4))) &&
         (uVar4 = pass1_1030_cf3a(uVar10,0x8), uVar4 != 0x0)) {
        bVar3 = true;
        break;
      }
    }
    if (puStack26 != (undefined4 *)0x0) {
      ppcVar2 = (code **)*puStack26;
      (**ppcVar2)(0x38,uVar5,puVar6,0x1);
    }
    if (!bVar3) {
      return;
    }
  }
  puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
  puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
  uVar5 = (uint)puVar9;
  uVar8 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4d6e(uVar11,puVar9,uVar5,puVar6);
  puStack26 = (undefined4 *)CONCAT22(puVar6,uVar5);
  ppcVar2 = (code **)((int)*puStack26 + 0x10);
  uVar7 = uVar5;
  (**ppcVar2)((int)&PTR_LOOP_1050_1038,uVar5,puVar6);
  uStack18 = CONCAT22(extraout_DX_01,uVar7);
  bVar3 = false;
  uStack14 = 0x0;
  do {
    if (uStack18 <= uStack14) {
LAB_1030_d51b:
      if (puStack26 != (undefined4 *)0x0) {
        ppcVar2 = (code **)*puStack26;
        (**ppcVar2)(uVar8,(char)uVar5,(char)puVar6,0x1);
      }
      if (!bVar3) {
        return;
      }
      uVar5 = *_PTR_LOOP_1050_65e2;
      iVar1 = *(int *)((int)_PTR_LOOP_1050_65e2 + 0x2);
      *(int *)(param_1 + param_3 * 0xc + 0x20) = uVar5 + 0xc8;
      *(int *)(param_1 + param_3 * 0xc + 0x22) = iVar1 + (uint)(0xff37 < uVar5);
      return;
    }
    uVar11 = pass1_1030_1d7c((int)uStack14,uStack14._2_2_,(ulong)puStack26);
    uVar7 = (uint)(uVar11 >> 0x10) | (uint)uVar11;
    if (uVar7 != 0x0) {
      uVar8 = SUB42(&USHORT_1050_1028,0x0);
      uVar4 = pass1_1028_6744(param_5,uVar11,0x7);
      if ((uVar7 | uVar4) != 0x0) {
        uVar8 = SUB42(&USHORT_1050_1028,0x0);
        pass1_1028_6228(uVar11,0x1,0x0,0x7,param_5);
        bVar3 = true;
        goto LAB_1030_d51b;
      }
    }
    uStack14 = uStack14 + 0x1;
  } while( true );
}



int __stdcall16far pass1_1030_d56a(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  switch(*(int *)(iVar1 + 0x98) + -0x1) {
  case 0x0:
    *(undefined2 *)(iVar1 + 0x98) = 0x2;
    break;
  case 0x1:
    *(undefined2 *)(iVar1 + 0x98) = 0x3;
    break;
  case 0x2:
    *(undefined2 *)(iVar1 + 0x98) = 0x4;
    break;
  case 0x3:
    *(undefined2 *)(iVar1 + 0x98) = 0xc;
    break;
  default:
    *(undefined2 *)(iVar1 + 0x98) = 0x1;
    return iVar1;
  case 0x7:
    *(undefined2 *)(iVar1 + 0x98) = 0x9;
    return iVar1;
  case 0x8:
    *(undefined2 *)(iVar1 + 0x98) = 0xb;
    return iVar1;
  case 0xa:
    *(undefined2 *)(iVar1 + 0x98) = 0x5;
    return iVar1;
  case 0xb:
    *(undefined2 *)(iVar1 + 0x98) = 0x8;
    return iVar1;
  }
  return iVar1;
}



void __stdcall16far pass1_1030_d61c(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  int iVar2;
  undefined2 uVar3;
  ushort uVar4;
  ushort uVar5;
  undefined4 local_1a;
  undefined *local_16;
  undefined2 local_14;
  undefined4 local_12 [0x3];
  int iStack4;
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    for (iStack4 = 0x0; uVar4 = (ushort)param_2, uVar5 = (ushort)(param_2 >> 0x10), iStack4 < 0xa;
        iStack4 = iStack4 + 0x1) {
      uVar3 = (undefined2)(param_1 >> 0x10);
      iVar2 = (int)param_1;
      local_12[0] = *(undefined4 *)(iVar2 + iStack4 * 0xc + 0x20);
      BVar1 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)local_12,param_3,(char *)0x4,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_14 = *(undefined2 *)(iVar2 + iStack4 * 0xc + 0x24);
      BVar1 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_14,param_3,(char *)0x2,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_16 = (undefined *)*(undefined2 *)(iVar2 + iStack4 * 0xc + 0x26);
      BVar1 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_16,param_3,(char *)0x2,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_1a = *(undefined4 *)(iVar2 + iStack4 * 0xc + 0x28);
      BVar1 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_1a,param_3,(char *)0x4,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
    }
    local_16 = PTR_LOOP_1050_5812;
    BVar1 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_16,param_3,(char *)0x2,0x1008);
    if (BVar1 != 0x0) {
      return;
    }
LAB_1030_d701:
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1030_d72e(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  ushort uVar1;
  BOOL16 BVar2;
  int iVar3;
  ushort uVar4;
  ushort uVar5;
  int iStack10;
  undefined4 local_8;
  ushort local_4;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 == 0x0) {
    return;
  }
  iStack10 = 0x0;
  while( true ) {
    uVar4 = (ushort)param_2;
    uVar5 = (ushort)(param_2 >> 0x10);
    if (0x9 < iStack10) {
      if ((0x3 < (int)PTR_LOOP_1050_0312) &&
         (BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)&PTR_LOOP_1050_5812,0x0,(ushort)&USHORT_1050_1050,0x2,0x1008),
         BVar2 == 0x0)) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
      }
      return;
    }
    BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)&local_8,0x0,param_5,0x4,0x1008);
    if (BVar2 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
    BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)&local_4,0x0,param_5,0x2,0x1008);
    if (BVar2 == 0x0) break;
    iVar3 = iStack10 * 0xc + (int)param_1;
    *(undefined2 *)(iVar3 + 0x20) = (undefined2)local_8;
    *(undefined2 *)(iVar3 + 0x22) = local_8._2_2_;
    uVar1 = switch_1008_72bc(uVar4,uVar5,local_4);
    *(ushort *)(iVar3 + 0x24) = uVar1;
    if ((int)PTR_LOOP_1050_0312 < 0x2) {
      iVar3 = iStack10 * 0xc + (int)param_1;
      *(undefined2 *)(iVar3 + 0x26) = 0x3;
      *(undefined4 *)(iVar3 + 0x28) = 0x0;
    }
    else {
      BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)&local_4,0x0,param_5,0x2,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
      }
      BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)&local_8,0x0,param_5,0x4,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
      }
      iVar3 = iStack10 * 0xc + (int)param_1;
      *(ushort *)(iVar3 + 0x26) = local_4;
      *(undefined4 *)(iVar3 + 0x28) = local_8;
    }
    iStack10 = iStack10 + 0x1;
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  return;
}



astruct_18 * __stdcall16far pass1_1030_d868(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1030_d8f6(ushort *param_1)

{
  astruct_184 *iVar1;
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_184 *)param_1;
  *param_1 = 0xdc2e;
  iVar1->field_0x2 = 0x1030;
  if (iVar1->field_0xc == 0x4c) {
    iVar1->field_0xe = 0x43;
  }
  else {
    if (iVar1->field_0xc == 0x4d) {
      iVar1->field_0xe = 0x44;
    }
    else {
      iVar1->field_0xe = 0x45;
    }
  }
  return param_1;
}



ulong __stdcall16far pass1_1030_d942(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xdc2e;
  *(undefined2 *)(param_1 + 0x2) = 0x1030;
  if (*(int *)(param_1 + 0xc) == 0x4c) {
    *(undefined2 *)(param_1 + 0xe) = 0x43;
  }
  else {
    if (*(int *)(param_1 + 0xc) == 0x4d) {
      *(undefined2 *)(param_1 + 0xe) = 0x44;
    }
    else {
      *(undefined2 *)(param_1 + 0xe) = 0x45;
    }
  }
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1030_d994(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(int *)(iVar4 + 0x12) != 0x4) {
    return;
  }
  uVar6 = pass1_1028_b4f2((ulong)param_1);
  iVar3 = (int)uVar6;
  if (*(long *)(iVar3 + 0x200) == 0x8000002) {
    uVar2 = *(undefined4 *)(iVar4 + 0x14);
    piVar1 = (int *)((int)uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
  }
  else {
    pass1_1028_cb04((ulong)param_1,param_2,param_3,param_4);
    if (iVar3 == 0x0) {
      return;
    }
    pass1_1030_dace((ulong)param_1,param_4);
    if (iVar3 == 0x0) {
      return;
    }
    uVar2 = *(undefined4 *)(iVar4 + 0x14);
    piVar1 = (int *)((int)uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
    pass1_1028_c952((ulong)param_1,param_2,param_3,param_4);
    pass1_1030_da22((ulong)param_1,param_4);
  }
  uVar2 = *(undefined4 *)(iVar4 + 0x14);
  if (*(int *)((int)uVar2 + 0x94) < 0x1) {
    pass1_1028_bdac(param_1,0x5,(ushort)&USHORT_1050_1028);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_da22(ulong param_1,ushort param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined2 uVar3;
  BOOL16 BVar4;
  ushort uVar5;
  undefined4 *puVar6;
  uint extraout_DX;
  uint uVar7;
  uint uVar8;
  ulong uVar9;
  ulong uStack18;
  
  uVar9 = pass1_1028_b4f2(param_1);
  uVar3 = (undefined2)(uVar9 >> 0x10);
  puVar1 = (undefined4 *)*(ulong *)((int)uVar9 + 0xc);
  ppcVar2 = (code **)((int)*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)((int)&USHORT_1050_1028,(int)puVar1,*(undefined2 *)((int)uVar9 + 0xe));
  uStack18 = 0x0;
  while( true ) {
    if (((ulong)puVar6 & 0xffff | (ulong)extraout_DX << 0x10) <= uStack18) {
      return;
    }
    uVar9 = pass1_1030_1d7c((int)((ulong)puVar6 & 0xffff),extraout_DX,(ulong)puVar1);
    uVar7 = (uint)(uVar9 >> 0x10);
    uVar8 = uVar7 | (uint)uVar9;
    if (((uVar8 != 0x0) &&
        (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((uint)uVar9 + 0xc),0x4), BVar4 != 0x0)) &&
       (uVar5 = pass1_1028_6744(param_2,uVar9,0xd), (uVar8 | uVar5) != 0x0)) break;
    uStack18 = uStack18 + 0x1;
  }
  pass1_1028_6228(uVar9,0x1,0x0,0xd,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_dace(ulong param_1,ushort param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined2 uVar3;
  BOOL16 BVar4;
  ushort uVar5;
  undefined4 *puVar6;
  uint extraout_DX;
  uint uVar7;
  uint uVar8;
  ulong uVar9;
  ulong uStack20;
  
  uVar9 = pass1_1028_b4f2(param_1);
  uVar3 = (undefined2)(uVar9 >> 0x10);
  puVar1 = (undefined4 *)*(ulong *)((int)uVar9 + 0xc);
  ppcVar2 = (code **)((int)*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)((int)&USHORT_1050_1028,(int)puVar1,*(undefined2 *)((int)uVar9 + 0xe));
  uStack20 = 0x0;
  uVar8 = extraout_DX;
  do {
    if (((ulong)puVar6 & 0xffff | (ulong)extraout_DX << 0x10) <= uStack20) {
      return;
    }
    uVar9 = pass1_1030_1d7c((int)((ulong)puVar6 & 0xffff),uVar8,(ulong)puVar1);
    uVar7 = (uint)(uVar9 >> 0x10);
    uVar8 = uVar7 | (uint)uVar9;
    if ((uVar8 != 0x0) &&
       (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((uint)uVar9 + 0xc),0x4), BVar4 != 0x0)) {
      uVar5 = pass1_1028_6744(param_2,uVar9,0xd);
      uVar8 = uVar8 | uVar5;
      if (uVar8 != 0x0) {
        return;
      }
    }
    uStack20 = uStack20 + 0x1;
  } while( true );
}



ushort __stdcall16far pass1_1030_db72(void)

{
  return 0x1;
}



void __stdcall16far pass1_1030_db78(ulong param_1)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) == 0x6) {
    pass1_1028_bdac((ulong *)(param_1 & 0xffff | (ulong)uVar1 << 0x10),0x5,(ushort)&USHORT_1050_1028);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_db92(ushort param_1,ushort param_2,ushort *param_3,ulong param_4,long param_5,ushort param_6)

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
    return;
  }
  return;
}



ushort __stdcall16far pass1_1030_dc02(void)

{
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1030_dc08(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1030_dc96(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x20) = 0x0;
  *param_1 = 0xe036;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



ushort * __stdcall16far pass1_1030_dcc2(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined4 *)(param_1 + 0x20) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xe036;
  *(undefined2 *)(param_1 + 0x2) = 0x1030;
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_dcf4(ushort *param_1,uint param_2)

{
  long lVar1;
  code **ppcVar2;
  uint uVar3;
  uint uVar4;
  ushort uVar5;
  uint extraout_DX;
  uint uVar6;
  uchar *puVar7;
  uint extraout_DX_00;
  uint uVar8;
  astruct_596 *iVar9;
  undefined2 uVar9;
  ulong *puVar10;
  ulong uVar11;
  ulong uStack28;
  ulong uStack24;
  undefined4 *puStack20;
  int iStack12;
  
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  iVar9 = (astruct_596 *)param_1;
  *param_1 = 0xe036;
  iVar9->field_0x2 = 0x1030;
  if (_PTR_LOOP_1050_65e2 != 0x0) {
    pass1_1028_b58e((ulong)param_1);
    if (iVar9->field_0x20 == 0x0) {
      uVar3 = extraout_DX | param_2;
      if (uVar3 == 0x0) {
        uVar6 = extraout_DX;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
      }
      else {
        uVar3 = *(uint *)(param_2 + 0x2e);
        uVar6 = *(uint *)(param_2 + 0x30);
      }
      puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
      puVar7 = (uchar *)((ulong)puVar10 >> 0x10);
      uVar4 = (uint)puVar10;
      pass1_1038_4d6e(CONCAT22(uVar6,uVar3),puVar10,uVar4,puVar7);
      puStack20 = (undefined4 *)CONCAT22(puVar7,uVar4);
      ppcVar2 = (code **)((int)*puStack20 + 0x10);
      uVar6 = uVar4;
      (**ppcVar2)((int)&PTR_LOOP_1050_1038,uVar4,puVar7);
      uStack24 = CONCAT22(extraout_DX_00,uVar6);
      uVar3 = extraout_DX_00;
      for (uStack28 = 0x0; uStack28 < uStack24; uStack28 = uStack28 + 0x1) {
        uVar11 = pass1_1030_1d7c(uVar6,uVar3,(ulong)puStack20);
        uVar8 = (uint)(uVar11 >> 0x10);
        uVar3 = uVar8 | (uint)uVar11;
        if (uVar3 != 0x0) {
          uVar5 = pass1_1030_dfcc((ulong)param_1);
          uVar5 = pass1_1030_cbf0((uint)uVar11,uVar8,uVar5);
          if (uVar5 != 0x0) break;
        }
      }
      if (puStack20 != (undefined4 *)0x0) {
        ppcVar2 = (code **)*puStack20;
        (**ppcVar2)(0x38,uVar4,puVar7,0x1);
      }
    }
    else {
      lVar1 = iVar9->field_0x20;
      uVar3 = extraout_DX;
      uVar6 = param_2;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)lVar1,(uint)((ulong)lVar1 >> 0x10));
      if ((uVar3 | uVar6) != 0x0) {
        iStack12 = 0x0;
        switch(iVar9->field_0xc) {
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
        if (iStack12 != 0x0) {
          pass1_1030_cc44(uVar6,uVar3,0x1,CONCAT22(extraout_DX,param_2),iStack12);
        }
      }
    }
  }
  pass1_1028_b418(param_1);
  return;
}
