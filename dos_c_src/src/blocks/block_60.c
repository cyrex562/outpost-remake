
astruct_18 * __stdcall16far pass1_1020_d7d8(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_d866(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0xd8ec;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_d888(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xd8ec;
  *(undefined2 *)(param_1 + 0x2) = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



undefined4 __stdcall16far pass1_1020_d8ca(int param_1,ushort param_2)

{
  pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
  if ((*(byte *)(param_1 + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6),0x1000);
  }
  return CONCAT22(*(undefined2 *)(param_1 + 0x8),*(undefined2 *)(param_1 + 0x6));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1020_d954(ushort *param_1)

{
  uchar *extraout_DX;
  astruct_182 *iVar1;
  int unaff_DI;
  undefined2 uVar1;
  ushort unaff_SS;
  ushort *puVar2;
  
  struct_1030_dc96(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_182 *)param_1;
  iVar1->field_0x24 = 0x0;
  iVar1->field_0x26 = 0x0;
  *(undefined4 *)&iVar1->field_0x28 = 0x0;
  *param_1 = 0xe792;
  iVar1->field_0x2 = 0x1020;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,extraout_DX,unaff_DI);
  iVar1->field_0x28 = (int)puVar2;
  iVar1->field_0x2a = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort * __stdcall16far
struct_1020_d99e(ushort *param_1,ushort param_2,int param_3,ulong param_4,ushort param_5,ushort param_6)

{
  int unaff_DI;
  ushort *puVar1;
  ushort uVar2;
  astruct_178 *iVar2;
  
  iVar2 = (astruct_178 *)param_1;
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  puVar1 = pass1_1030_dcc2((int)iVar2,uVar2,param_3,param_4,param_5);
  iVar2->field_0x24 = param_2;
  iVar2->field_0x26 = 0x0;
  *(undefined4 *)&iVar2->field_0x28 = 0x0;
  *param_1 = 0xe792;
  iVar2->field_0x2 = 0x1020;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,(uchar *)((ulong)puVar1 >> 0x10),unaff_DI);
  iVar2->field_0x28 = (int)puVar1;
  iVar2->field_0x2a = (int)((ulong)puVar1 >> 0x10);
  iVar2->field_0x10 = 0x49;
  return param_1;
}



void __stdcall16far pass1_1020_d9fa(ulong param_1,ushort param_2)

{
  undefined2 extraout_DX;
  
  if (*(int *)((int)param_1 + 0xc) != 0x79) {
    pass1_1030_df0c(param_1,param_2);
    pass1_1028_b58e(param_1);
    pass1_1038_57dc(*(ulong *)(param_2 + 0x2e),0x1,0x2);
  }
  return;
}



void __stdcall16far pass1_1020_da3c(ulong *param_1)

{
  pass1_1028_bdac(param_1,0x2,(ushort)&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_da4e(ulong *param_1,ushort *param_2,ulong param_3,ulong param_4,ushort param_5,int param_6,ushort param_7)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  BOOL16 BVar4;
  uchar *extraout_DX;
  uchar *puVar5;
  uchar *extraout_DX_00;
  uint uVar6;
  ulong uVar7;
  ulong uVar8;
  ushort uVar9;
  ushort uVar11;
  ushort *puVar10;
  ulong uVar12;
  byte bStack31;
  undefined4 local_e;
  ushort uStack10;
  uint uStack8;
  undefined4 uStack6;
  
  puVar2 = &local_e;
  pass1_1030_64ce(param_7,puVar2,param_5,_PTR_LOOP_1050_5740,param_2,param_4,(ulong *)CONCAT22(param_7,puVar2));
  uStack6 = *puVar2;
  uVar6 = *(uint *)((int)puVar2 + 0x2);
  bStack31 = (byte)((ulong)uStack6 >> 0x18);
  uVar3 = (uint)bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack6,uVar6);
    uVar7 = struct_op_1030_73a8(CONCAT22(uVar6,uVar3));
    uVar6 = (uint)(uVar7 >> 0x10);
    uVar3 = (uint)uVar7;
    if (*(int *)(uVar3 + 0xc) == 0x10) {
      PTR_LOOP_1050_50ca = (undefined *)0x6a9;
      return;
    }
  }
  uVar9 = (ushort)param_1;
  uVar11 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1028_c7b6(param_7,uVar6,uVar9,uVar11,param_2,param_4);
  uVar8 = (ulong)param_1 & 0xffff | (ulong)uVar11 << 0x10;
  ppcVar1 = (code **)((int)*param_1 + 0x60);
  puVar10 = param_2;
  uVar7 = param_3;
  uVar12 = param_4;
  uStack8 = uVar3;
  (**ppcVar1)();
  if (((uVar3 != 0x0) &&
      (puVar5 = extraout_DX, pass1_1028_c23e(uVar9,uVar11,param_2,param_3,param_4,uVar3,(uint)extraout_DX,param_7),
      uVar3 != 0x0)) &&
     (BVar4 = pass1_1028_c314(param_7,uVar3,(ushort)puVar5,uVar9,uVar11,param_2,(ushort)param_3,
                              (ushort)(param_3 >> 0x10),param_4), BVar4 != 0x0)) {
    uVar6 = (uint)((ulong)param_2 >> 0x10);
    if (((*(int *)((int)param_2 + 0x4) == 0x0) && (uStack8 != 0x4)) &&
       (ppcVar1 = (code **)((int)*param_1 + 0x5c),
       (**ppcVar1)((int)&USHORT_1050_1028,param_1,param_2,param_3,param_4,uVar8,puVar10,uVar7,uVar12),
       puVar5 = extraout_DX_00, BVar4 == 0x0)) {
      return;
    }
    uStack10 = *(ushort *)((int)param_2 + 0x4);
    if (uStack10 != 0x0) {
      pass1_1020_df10((ulong)param_1,(ushort *)((ulong)param_2 & 0xffff | (ulong)uVar6 << 0x10),param_4,uStack10,puVar5,
                      param_6,param_7);
      return;
    }
    pass1_1020_deac((ulong)param_1,(ushort *)((ulong)param_2 & 0xffff | (ulong)uVar6 << 0x10),param_4,0x0,puVar5,param_6
                    ,param_7);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_db86(ushort param_1,ushort param_2,ushort *param_3,ulong param_4,long param_5,ushort param_6)

{
  int iVar1;
  undefined *puVar2;
  uint uVar3;
  ulong uVar4;
  ushort *puVar5;
  undefined local_4 [0x2];
  
  uVar4 = pass1_1030_bcae((ushort)local_4,param_6);
  uVar3 = (uint)(uVar4 >> 0x10);
  iVar1 = (int)uVar4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
  uVar4 = *(ulong *)(iVar1 + 0x10);
  puVar5 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar4,(uint)(uVar4 >> 0x10));
  puVar2 = local_4;
  pass1_1030_bcde(param_6,(ushort)puVar2,param_6,uVar4 & 0xffff | (ulong)uVar3 << 0x10,puVar5,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (undefined *)0x6af;
  }
  else {
    if (((int)puVar2 < 0x1f) || (*(int *)((int)param_3 + 0x4) < 0x1)) {
      return;
    }
    PTR_LOOP_1050_50ca = (undefined *)0x6b6;
    PTR_LOOP_1050_50cc = puVar2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_dc1c(ulong param_1,ushort *param_2,ushort param_3)

{
  int iVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  undefined4 *puVar7;
  ulong *puVar8;
  byte bStack27;
  undefined local_a [0x4];
  undefined4 uStack6;
  
  puVar8 = (ulong *)CONCAT22(param_3,local_a);
  uVar6 = pass1_1028_bb24(param_1);
  uVar5 = (uint)(uVar6 >> 0x10);
  puVar3 = (undefined4 *)uVar6;
  pass1_1030_64ce(param_3,puVar3,uVar5,_PTR_LOOP_1050_5740,param_2,uVar6 & 0xffff | (ulong)uVar5 << 0x10,puVar8);
  uStack6 = *puVar3;
  uVar5 = *(uint *)((int)puVar3 + 0x2);
  bStack27 = (byte)((ulong)uStack6 >> 0x18);
  uVar4 = (uint)bStack27;
  if (bStack27 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack6,uVar5);
    puVar7 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar5,uVar4));
    iVar1 = *(int *)((int)puVar7 + 0xc);
    if (((iVar1 < 0x1) || (SBORROW2(iVar1,0x1))) ||
       ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 && ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73)))))) {
      ppcVar2 = (code **)((int)*puVar7 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1020_dca8(ulong param_1,ushort param_2,ushort param_3)

{
  undefined2 uVar1;
  ushort uVar2;
  undefined local_2e [0xe];
  undefined4 *puStack32;
  ushort uStack30;
  ushort uStack28;
  ushort uStack26;
  ushort uStack24;
  ushort uStack22;
  ushort uStack20;
  ushort uStack18;
  undefined4 local_10;
  ushort uStack12;
  undefined4 uStack10;
  undefined local_6 [0x2];
  int local_4;
  
  pass1_1028_c1f8(param_3,(int)local_6,param_2,param_1,(ushort *)CONCAT22(param_3,local_6),
                  (ushort *)CONCAT22(param_3,&local_4));
  uStack10 = pass1_1028_b58e(param_1);
  uVar1 = (undefined2)((ulong)uStack10 >> 0x10);
  local_10 = *(undefined4 *)((int)uStack10 + 0xc);
  uStack12 = *(ushort *)((int)uStack10 + 0x10);
  puStack32 = &local_10;
  uStack18 = (ushort)local_10;
  uStack20 = (ushort)((ulong)local_10 >> 0x10);
  uStack24 = (ushort)local_10 - 0x1;
  if ((int)uStack24 < 0x0) {
    uStack24 = 0x0;
  }
  uVar2 = local_4 - 0x1;
  uStack26 = (ushort)local_10 + 0x1;
  if ((int)uVar2 < (int)((ushort)local_10 + 0x1)) {
    uStack26 = uVar2;
  }
  uStack28 = uStack20 - 0x1;
  if ((int)uStack28 < 0x0) {
    uStack28 = 0x0;
  }
  uStack30 = uStack20 + 0x1;
  if ((int)uVar2 < (int)(uStack20 + 0x1)) {
    uStack30 = uVar2;
  }
  uStack22 = uStack12;
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack12,uStack28,uStack24);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack22,uStack28,uStack18);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack22,uStack28,uStack26);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack22,uStack20,uStack24);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack22,uStack20,uStack26);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack22,uStack30,uStack24);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack22,uStack30,uStack18);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,local_2e),uStack22,uStack30,uStack26);
  pass1_1020_dc1c(param_1,(ushort *)CONCAT22(param_3,local_2e),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_de32(ulong param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined2 uVar4;
  ushort *puVar5;
  
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x5,param_5,param_3,param_4);
  uVar2 = (ushort)((ulong)puVar5 >> 0x10);
  *(ushort *)((int)puVar5 + 0x12) = param_2;
  uVar3 = uVar2;
  BVar1 = bring_win_to_top_1038_b72e(_PTR_LOOP_1050_5b7c,0x4,(HWND16)&PTR_LOOP_1050_1038);
  if (BVar1 == 0x0) {
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)((ushort)_PTR_LOOP_1050_4230 + 0x16),0x4,uVar3,
                    (ushort)_PTR_LOOP_1050_4230,(ushort)&PTR_LOOP_1050_1038,param_5);
  }
  PTR_LOOP_1050_5b80 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5b80,0x1008,param_5);
  uVar4 = (undefined2)(param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x24) = *(undefined2 *)((int)puVar5 + 0xa);
  if (*(int *)((int)param_1 + 0x24) == 0x0) {
    PTR_LOOP_1050_50ca = (undefined *)0x6b2;
  }
  return;
}



BOOL16 __stdcall16far
pass1_1020_deac(ulong param_1,ushort *param_2,long param_3,int param_4,uchar *param_5,int param_6,ushort param_7)

{
  ushort uVar1;
  ushort uVar2;
  
  uVar1 = (ushort)param_1;
  uVar2 = (ushort)(param_1 >> 0x10);
  pass1_1028_c7b6(param_7,(ushort)param_5,uVar1,uVar2,param_2,param_3);
  if (param_4 < 0x1) {
    return 0x0;
  }
  if (SBORROW2(param_4,0x1)) {
    return 0x0;
  }
  if (param_4 != 0x3 && 0x0 < param_4 + -0x2) {
    if (param_4 == 0x4) {
      pass1_1020_de32(param_1,0x4,param_5,param_6,param_7);
      if (*(int *)(uVar1 + 0x24) == 0x6) {
        return 0x1;
      }
      return 0x0;
    }
    if (param_4 != 0x5) {
      return 0x0;
    }
  }
  *(undefined2 *)(uVar1 + 0x24) = 0x1;
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_df10(ulong param_1,ushort *param_2,long param_3,ushort param_4,uchar *param_5,int param_6,ushort param_7)

{
  undefined4 *puVar1;
  uint uVar2;
  BOOL16 BVar3;
  uint uVar4;
  ulong uVar5;
  ushort uVar6;
  ushort uVar7;
  byte bStack31;
  undefined4 local_e;
  undefined4 uStack10;
  ushort uStack6;
  undefined2 uStack4;
  
  uStack4 = 0x0;
  uVar6 = (ushort)param_1;
  uVar7 = (ushort)(param_1 >> 0x10);
  pass1_1028_c7b6(param_7,(ushort)param_5,uVar6,uVar7,param_2,param_3);
  uStack6 = param_4;
  if (param_4 == 0x0) {
    puVar1 = &local_e;
    pass1_1030_64ce(param_7,puVar1,param_5,_PTR_LOOP_1050_5740,param_2,param_3,(ulong *)CONCAT22(param_7,puVar1));
    uStack10 = *puVar1;
    uVar4 = *(uint *)((int)puVar1 + 0x2);
    bStack31 = (byte)((ulong)uStack10 >> 0x18);
    uVar2 = (uint)bStack31;
    if (bStack31 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack10,uVar4);
      uVar5 = struct_op_1030_73a8(CONCAT22(uVar4,uVar2));
      if (*(int *)((int)uVar5 + 0xc) == 0x6a) {
        BVar3 = pass1_1020_e044(param_1);
        if (BVar3 == 0x0) {
          *(undefined2 *)(uVar6 + 0x24) = 0x1;
        }
        else {
          PTR_LOOP_1050_50ca = (undefined *)0x6ac;
        }
      }
    }
  }
  else {
    if (((0x5 < (int)param_4) && (!SBORROW2(param_4,0x6))) && ((int)(param_4 - 0x6) < 0x4)) {
      pass1_1020_de32(param_1,param_4,param_5,param_6,param_7);
      switch(*(undefined2 *)(uVar6 + 0x24)) {
      case 0x1:
        BVar3 = pass1_1020_e044(param_1);
        if (BVar3 != 0x0) {
          PTR_LOOP_1050_50ca = (undefined *)0x6ac;
        }
        break;
      case 0x2:
      case 0x3:
      case 0x4:
      case 0x5:
        pass1_1020_e652(param_1,(ulong *)param_2,(ushort)((ulong)param_2 >> 0x10),param_3,param_7);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far pass1_1020_e044(ulong param_1)

{
  undefined4 uVar1;
  ushort uVar2;
  undefined2 uVar3;
  ulong uVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar4 = pass1_1018_04b8(*(ulong *)((int)param_1 + 0x28));
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar4,(uint)(uVar4 >> 0x10));
  uVar2 = pass1_1030_2fac(uVar4);
  uVar1 = *(undefined4 *)((int)param_1 + 0x28);
  if ((int)uVar2 <= *(int *)((int)uVar1 + 0x1e)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_e08e(ulong param_1,ushort param_2,ushort param_3,uchar param_4)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  ulong uVar5;
  undefined2 extraout_DX;
  uint uVar6;
  ushort uVar7;
  ushort uVar8;
  int *piVar9;
  ushort *puVar10;
  ushort uVar11;
  ushort uVar12;
  undefined2 local_158;
  undefined2 uStack342;
  ulong uStack50;
  ulong *puStack42;
  int local_22;
  undefined local_20 [0x2];
  undefined local_1e [0x2];
  undefined2 uStack28;
  int *piStack26;
  int local_18;
  ushort local_16;
  ulong uStack20;
  ulong local_10;
  int iStack12;
  ulong uStack10;
  undefined4 uStack6;
  
  uVar8 = (ushort)(param_1 >> 0x10);
  uVar7 = (ushort)param_1;
  iVar3 = *(int *)(uVar7 + 0xc);
  if (iVar3 == 0x74) {
    iVar1 = *(int *)(uVar7 + 0x24);
    iVar3 = iVar1 + -0x1;
    if (iVar3 == 0x0) goto LAB_1020_e0ae;
    iVar3 = iVar1 + -0x6;
    if (iVar3 != 0x0) goto LAB_1020_e0b9;
    uVar11 = 0x1;
  }
  else {
    if (iVar3 == 0x78) {
      iVar1 = *(int *)(uVar7 + 0x24);
      iVar4 = iVar1 + -0x1;
      if (iVar4 != 0x0) {
        iVar3 = iVar1 + -0x2;
        if ((0x0 < iVar4) && (!SBORROW2(iVar4,0x1))) {
          if (iVar1 + -0x5 == 0x0 || iVar3 < 0x3) {
            iVar3 = iVar1 + -0x5;
            pass1_1020_e49a(param_1,param_3,param_4);
          }
          else {
            iVar3 = iVar1 + -0x6;
            if (iVar3 == 0x0) {
              pass1_1020_e39c(param_1,0x6,0x0,param_3,param_4);
              pass1_1020_dca8(param_1,param_2,param_3);
            }
          }
        }
        goto LAB_1020_e0b9;
      }
      uVar11 = 0x6a;
      iVar3 = iVar4;
    }
    else {
      iVar3 = iVar3 + -0x79;
      if (iVar3 == 0x0) {
        pass1_1020_e49a(param_1,param_3,param_4);
        return;
      }
LAB_1020_e0ae:
      uVar11 = 0x47;
    }
  }
  pass1_1020_e39c(param_1,uVar11,iVar3,param_3,param_4);
LAB_1020_e0b9:
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,iVar3);
  uVar5 = *(ulong *)(iVar3 + 0x2e);
  uVar6 = *(uint *)(iVar3 + 0x30);
  uStack10 = uVar5;
  if (*(int *)(uVar7 + 0xc) != 0x79) {
    pass1_1038_5804(uVar5 & 0xffff | (ulong)uVar6 << 0x10,0x1,0x2);
  }
  if (*(int *)(uVar7 + 0x24) == 0x6) {
    pass1_1038_43cc((int)uStack10,(int)(uStack10 >> 0x10),0x1,0x2,(int)uVar5,uVar6);
  }
  local_10 = *(ulong *)((int)uStack6 + 0xc);
  iStack12 = *(int *)((int)uStack6 + 0x10);
  puStack42 = &local_10;
  if ((*(int *)(uVar7 + 0x24) == 0x6) && (iStack12 == 0x0)) {
    return;
  }
  uVar2 = *(undefined4 *)(uVar7 + 0x28);
  uVar5 = *(ulong *)((int)uVar2 + 0x20);
  puVar10 = &local_16;
  piStack26 = &local_18;
  piVar9 = piStack26;
  uVar11 = param_3;
  uVar12 = param_3;
  uStack20 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar5,(uint)(uVar5 >> 0x10));
  uStack28 = (undefined2)uVar5;
  pass1_1030_5b1c(uVar5 & 0xffff | ZEXT24(piStack26) << 0x10,(ushort *)CONCAT22(uVar11,piVar9),
                  (ushort *)CONCAT22(uVar12,puVar10));
  pass1_1028_c8ee(param_3,uVar7,uVar8,*(int *)(uVar7 + 0x24),(ushort *)CONCAT22(param_3,&local_10));
  pass1_1008_3eb4((ushort *)CONCAT22(param_3,&local_10),(ushort *)CONCAT22(param_3,&local_22),
                  (ushort *)CONCAT22(param_3,local_20),(ushort *)CONCAT22(param_3,local_1e));
  if (*(int *)(uVar7 + 0x24) == 0x1) {
    if (local_18 < local_22) {
      pass1_1030_5b3e(CONCAT22(piStack26,uStack28),local_22,local_16);
      pass1_1030_5b1c(CONCAT22(piStack26,uStack28),(ushort *)CONCAT22(param_3,&local_18),
                      (ushort *)CONCAT22(param_3,&local_16));
    }
    uStack50 = *(ulong *)((int)uStack10 + 0x4);
    struct_op_1028_87f0(param_3,param_4,(astruct_97 *)CONCAT22(param_3,&local_158),0x0,0x0,0x6a,&local_10,param_3,
                        uStack50,uStack20);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,&local_158));
    local_158 = 0x389a;
    uStack342 = 0x1008;
  }
  pass1_1028_ccd0(param_4,param_3,param_1,(ushort *)CONCAT22(param_3,&local_10));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_e294(ulong param_1,ushort param_2,uchar param_3)

{
  undefined4 uVar1;
  ulong *puVar2;
  ulong uVar3;
  undefined2 extraout_DX;
  undefined2 uVar4;
  ushort uVar5;
  ushort uVar6;
  char cStack347;
  undefined local_150 [0xc];
  ulong *puStack324;
  undefined local_140 [0x124];
  ulong uStack28;
  undefined4 uStack24;
  ulong uStack20;
  ulong local_10;
  undefined2 uStack12;
  int iStack10;
  undefined2 uStack8;
  ulong uStack6;
  
  uVar6 = (ushort)(param_1 >> 0x10);
  uVar5 = (ushort)param_1;
  if ((0x1 < *(int *)(uVar5 + 0x24)) && (*(int *)(uVar5 + 0x24) < 0x6)) {
    uVar1 = *(undefined4 *)(uVar5 + 0x28);
    uVar3 = *(ulong *)((int)uVar1 + 0x20);
    uStack6 = uVar3;
    pass1_1028_b58e(param_1);
    iStack10 = (int)uVar3;
    local_10 = *(ulong *)(iStack10 + 0xc);
    uStack12 = *(undefined2 *)(iStack10 + 0x10);
    puStack324 = &local_10;
    uVar4 = extraout_DX;
    uStack8 = extraout_DX;
    pass1_1028_c8ee(param_2,uVar5,uVar6,*(int *)(uVar5 + 0x24),(ushort *)CONCAT22(param_2,puStack324));
    puVar2 = &local_10;
    pass1_1028_c89c(param_1,(ushort *)CONCAT22(param_2,puVar2),(ulong *)CONCAT22(param_2,local_150),(int)puVar2,param_2)
    ;
    uStack20 = *puVar2;
    cStack347 = (char)(uStack20 >> 0x18);
    if ((cStack347 == '\0') && ((int)uStack20 == 0x9)) {
      *(undefined2 *)(uVar5 + 0x14) = 0x1;
    }
    uStack24 = *(undefined4 *)(iStack10 + 0x2e);
    uStack28 = *(ulong *)((int)uStack24 + 0x4);
    struct_op_1028_87f0(param_2,param_3,(astruct_97 *)CONCAT22(param_2,local_140),0x0,*(int *)(uVar5 + 0x14) + 0x1,0x79,
                        &local_10,param_2,uStack28,uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,local_140));
  }
  *(undefined2 *)(uVar5 + 0x26) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_e39c(ulong param_1,ushort param_2,int param_3,ushort param_4,uchar param_5)

{
  undefined4 uVar1;
  undefined2 uVar2;
  undefined2 extraout_DX;
  undefined local_13c [0x124];
  ulong uStack24;
  undefined4 uStack20;
  ulong uStack16;
  ulong local_c;
  int iStack8;
  undefined4 uStack6;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_3);
  local_c = *(ulong *)(param_3 + 0xc);
  iStack8 = *(int *)(param_3 + 0x10);
  if (iStack8 < 0x1) {
    uVar2 = 0x5;
  }
  else {
    uVar2 = 0x6;
  }
  *(undefined2 *)(param_3 + 0x14) = uVar2;
  uVar1 = *(undefined4 *)((int)param_1 + 0x28);
  uStack16 = *(ulong *)((int)uVar1 + 0x20);
  uStack20 = *(undefined4 *)(param_3 + 0x2e);
  uStack24 = *(ulong *)((int)uStack20 + 0x4);
  struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_13c),0x0,0x1,param_2,&local_c,param_4,
                      uStack24,uStack16);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_13c));
  return;
}



void __stdcall16far pass1_1020_e44c(ulong param_1,ushort param_2,ushort param_3,uchar param_4)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0x12) == 0x2) {
    piVar1 = (int *)(iVar2 + 0x14);
    *piVar1 = *piVar1 + -0x1;
    if ((*(int *)(iVar2 + 0x26) == 0x0) && (*(int *)(iVar2 + 0xc) == 0x78)) {
      pass1_1020_e294(param_1 & 0xffff | (ulong)uVar3 << 0x10,param_3,param_4);
    }
    if (*(int *)(iVar2 + 0x14) == 0x0) {
      pass1_1020_e08e(param_1 & 0xffff | (ulong)uVar3 << 0x10,param_2,param_3,param_4);
      return;
    }
    if (*(int *)(iVar2 + 0x24) == 0x6) {
      *(undefined2 *)(iVar2 + 0xe) = 0x49;
    }
  }
  return;
}



void __stdcall16far pass1_1020_e49a(ulong param_1,ushort param_2,uchar param_3)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  ushort uStack10;
  
  uVar3 = pass1_1028_b58e(param_1);
  iVar1 = *(int *)((int)uVar3 + 0x14);
  uStack10 = 0x0;
  iVar2 = iVar1 + -0x6;
  if (iVar2 == 0x0) {
    uStack10 = 0x9;
  }
  else {
    iVar2 = iVar1 + -0x7;
    if (iVar2 == 0x0) {
      uStack10 = 0x6;
    }
    else {
      iVar2 = iVar1 + -0x8;
      if (iVar2 == 0x0) {
        uStack10 = 0x7;
      }
      else {
        iVar2 = iVar1 + -0x9;
        if (iVar2 == 0x0) {
          uStack10 = 0x8;
        }
      }
    }
  }
  pass1_1020_e39c(param_1,uStack10,iVar2,param_2,param_3);
  return;
}



int __stdcall16far pass1_1020_e4fa(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  int iStack4;
  
  switch(param_2) {
  case 0x2:
  case 0x5:
  case 0x6:
  case 0x7:
    iStack4 = 0x4;
    break;
  case 0x3:
  case 0x8:
    iStack4 = 0x5;
    break;
  default:
    uVar1 = pass1_1028_b58e(param_1);
    iStack4 = *(int *)((int)uVar1 + 0x14) + 0x2;
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_e558(ulong param_1,int param_2,ushort param_3)

{
  undefined4 *puVar1;
  uint uVar2;
  int iVar3;
  undefined2 extraout_DX;
  undefined2 uVar4;
  uint uVar5;
  ushort uVar6;
  ushort uVar7;
  byte bStack45;
  undefined local_24 [0xc];
  undefined4 uStack24;
  undefined4 uStack20;
  undefined4 local_10;
  undefined2 uStack12;
  int iStack10;
  ushort uStack8;
  int iStack6;
  undefined2 uStack4;
  
  uVar7 = (ushort)(param_1 >> 0x10);
  uVar6 = (ushort)param_1;
  if (*(int *)(uVar6 + 0xc) == 0x79) {
    param_2 = *(int *)(uVar6 + 0x24);
    *(int *)(uVar6 + 0x14) = param_2;
    *(undefined2 *)(uVar6 + 0x24) = 0x0;
  }
  if (*(int *)(uVar6 + 0x24) != 0x6) {
    pass1_1028_b58e(param_1);
    uStack8 = *(ushort *)(param_2 + 0x14);
    iStack6 = param_2;
    uStack4 = extraout_DX;
    iStack10 = pass1_1020_e4fa(param_1,uStack8);
    local_10 = *(undefined4 *)(iStack6 + 0xc);
    uStack12 = *(undefined2 *)(iStack6 + 0x10);
    uStack24 = CONCAT22(uStack24._2_2_,&local_10);
    uVar4 = uStack4;
    pass1_1028_c8ee(param_3,uVar6,uVar7,*(int *)(uVar6 + 0x24),(ushort *)CONCAT22(param_3,&local_10));
    puVar1 = &local_10;
    pass1_1028_c89c(param_1,(ushort *)CONCAT22(param_3,puVar1),(ulong *)CONCAT22(param_3,local_24),(int)puVar1,param_3);
    uStack24 = *puVar1;
    uVar5 = *(uint *)((int)puVar1 + 0x2);
    bStack45 = (byte)((ulong)uStack24 >> 0x18);
    uVar2 = (uint)bStack45;
    uStack20._0_2_ = (ushort)uStack24;
    uStack20 = uStack24;
    if (bStack45 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack20,uVar5);
      uStack20._0_2_ = *(ushort *)(uVar2 + 0x14);
    }
    uStack8 = (ushort)uStack20;
    iVar3 = pass1_1020_e4fa(param_1,(ushort)uStack20);
    *(int *)(uVar6 + 0x14) = iStack10 + iVar3;
    return;
  }
  *(undefined2 *)(uVar6 + 0x14) = 0x1;
  return;
}



ulong * __stdcall16far pass1_1020_e652(ulong param_1,ulong *param_2,ushort param_3,long param_4,ushort param_5)

{
  ulong *puVar1;
  ushort uVar2;
  ushort uVar3;
  ulong local_8;
  undefined2 uStack4;
  
  local_8 = *param_2;
  uStack4 = *(undefined2 *)(param_2 + 0x1);
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar2 = (ushort)param_1;
  pass1_1028_c8ee(param_5,uVar2,uVar3,*(int *)(uVar2 + 0x24),(ushort *)CONCAT22(param_5,&local_8));
  puVar1 = &local_8;
  pass1_1028_c7b6(param_5,param_3,uVar2,uVar3,(ushort *)CONCAT22(param_5,puVar1),param_4);
  if (puVar1 != (ulong *)0x0) {
    puVar1 = (ulong *)((int)&PTR_LOOP_1050_0000 + 0x1);
  }
  return puVar1;
}



BOOL16 __stdcall16far write_to_file_1020_e6a4(ulong param_1,ulong param_2,ushort param_3)

{
  int in_AX;
  BOOL16 BVar1;
  undefined2 uVar2;
  ushort uVar3;
  undefined2 local_c [0x3];
  undefined2 local_6 [0x2];
  
  pass1_1030_de7c(param_1,param_2,param_3);
  if (in_AX != 0x0) {
    uVar2 = (undefined2)(param_1 >> 0x10);
    local_c[0] = *(undefined2 *)((int)param_1 + 0x24);
    uVar3 = (ushort)(param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)local_c,param_3,(char *)0x2,0x1008);
    if (BVar1 != 0x0) {
      local_6[0] = *(undefined2 *)((int)param_1 + 0x26);
      BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)local_6,param_3,(char *)0x2,0x1008);
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



void __stdcall16far pass1_1020_e70e(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  ushort uVar1;
  BOOL16 BVar2;
  ushort uVar3;
  
  pass1_1030_dec4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar3 = (ushort)(param_2 >> 0x10);
    BVar2 = read_file_1008_7dee((ushort)param_2,uVar3,(int)param_1 + 0x24,0x0,uVar1,0x2,0x1008);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee((ushort)param_2,uVar3,(int)param_1 + 0x26,0x0,uVar1,0x2,0x1008);
      if (BVar2 != 0x0) {
        return;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_e76c(astruct_18 *param_1,byte param_2,uint param_3)

{
  pass1_1030_dcf4(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_e7fa(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0xe88e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_e81c(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xe88e;
  *(undefined2 *)(param_1 + 0x2) = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1020_e846(ushort *param_1)

{
  *param_1 = 0xe88e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  pass1_1028_b418(param_1);
  return;
}

