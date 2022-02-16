


astruct_18 * __stdcall16far pass1_1038_64de(astruct_18 *param_1,byte param_2)

{
  pass1_1038_33f8(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1038_6520(ushort *param_1)

{
  astruct_308 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_308 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x1a));
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x24 = 0x0;
  iVar1->field_0x26 = 0x0;
  iVar1->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_6590(ushort *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ulong param_6)

{
  ushort uVar1;
  int iVar2;
  astruct_410 *iVar3;
  undefined2 uVar3;
  ushort unaff_SS;
  ushort *puVar4;
  ulong uVar5;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_410 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  *(undefined4 *)&iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_6;
  iVar3->field_0xc = param_4;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x12 = 0x0;
  iVar3->field_0x14 = 0x0;
  iVar3->field_0x16 = param_2;
  iVar3->field_0x18 = param_3;
  puVar4 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
  uVar1 = (ushort)((ulong)puVar4 >> 0x10);
  *(undefined4 *)&iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_5;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = (int)&PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_6,(uint)(param_6 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_5),param_5,uVar1,unaff_SS);
  iVar2 = (int)(uVar5 >> 0x10);
  iVar3->field_0x4 = (int)uVar5;
  iVar3->field_0x6 = iVar2;
  puVar4 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,(ushort *)CONCAT22(uVar1,param_5 + 0xc));
  uVar1 = (ushort)puVar4;
  pass1_1010_8fba(*(ulong *)&iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = iVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_666e(ushort *param_1,long *param_2,ushort param_3,ulong param_4)

{
  ushort uVar1;
  undefined2 uVar2;
  astruct_420 *iVar3;
  undefined2 uVar3;
  ushort unaff_SS;
  ushort *puVar4;
  ulong uVar5;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_420 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_4;
  iVar3->field_0xc = 0x0;
  iVar3->field_0xe = param_2;
  iVar3->field_0x12 = 0x0;
  iVar3->field_0x14 = 0x0;
  iVar3->field_0x18 = 0x0;
  iVar3->field_0x16 = 0x0;
  puVar4 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
  uVar1 = (ushort)((ulong)puVar4 >> 0x10);
  *(undefined4 *)&iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_3;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = (int)&PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_3),param_3,uVar1,unaff_SS);
  uVar2 = (undefined2)(uVar5 >> 0x10);
  *(int *)&iVar3->field_0x4 = (int)uVar5;
  *(undefined2 *)((int)&iVar3->field_0x4 + 0x2) = uVar2;
  puVar4 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,(ushort *)CONCAT22(uVar1,param_3 + 0xc));
  uVar1 = (ushort)puVar4;
  pass1_1010_8fba(iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = uVar2;
  pass1_1020_ba94(param_2);
  iVar3->field_0x16 = uVar1;
  iVar3->field_0x18 = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_675c(ushort *param_1,ulong param_2,ushort param_3,ushort param_4,ulong param_5)

{
  ushort uVar1;
  undefined2 uVar2;
  astruct_414 *iVar3;
  undefined2 uVar3;
  ushort unaff_SS;
  ushort *puVar4;
  ulong uVar5;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_414 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_5;
  iVar3->field_0xc = 0x0;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x12 = param_3;
  iVar3->field_0x14 = 0x0;
  iVar3->field_0x16 = param_2;
  puVar4 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
  uVar1 = (ushort)((ulong)puVar4 >> 0x10);
  *(undefined4 *)&iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_4;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = (int)&PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_5,(uint)(param_5 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_4),param_4,uVar1,unaff_SS);
  uVar2 = (undefined2)(uVar5 >> 0x10);
  *(int *)&iVar3->field_0x4 = (int)uVar5;
  *(undefined2 *)((int)&iVar3->field_0x4 + 0x2) = uVar2;
  puVar4 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,(ushort *)CONCAT22(uVar1,param_4 + 0xc));
  uVar1 = (ushort)puVar4;
  pass1_1010_8fba(iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_6838(ushort *param_1,ulong param_2,ushort param_3,ushort param_4,ulong param_5)

{
  ushort uVar1;
  undefined2 uVar2;
  astruct_415 *iVar3;
  undefined2 uVar3;
  ushort unaff_SS;
  ushort *puVar4;
  ulong uVar5;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_415 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_5;
  iVar3->field_0xc = 0x0;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x12 = 0x0;
  iVar3->field_0x14 = param_3;
  iVar3->field_0x16 = param_2;
  puVar4 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
  uVar1 = (ushort)((ulong)puVar4 >> 0x10);
  *(undefined4 *)&iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_4;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = (int)&PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_5,(uint)(param_5 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_4),param_4,uVar1,unaff_SS);
  uVar2 = (undefined2)(uVar5 >> 0x10);
  *(int *)&iVar3->field_0x4 = (int)uVar5;
  *(undefined2 *)((int)&iVar3->field_0x4 + 0x2) = uVar2;
  puVar4 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,(ushort *)CONCAT22(uVar1,param_4 + 0xc));
  uVar1 = (ushort)puVar4;
  pass1_1010_8fba(iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = uVar2;
  return;
}



void __stdcall16far pass1_1038_6912(ushort *param_1)

{
  uint uVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined2 uVar6;
  astruct_18 *paStack10;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = 0x78de;
  *(undefined2 *)(iVar5 + 0x2) = (int)&PTR_LOOP_1050_1038;
  uVar1 = *(uint *)(iVar5 + 0x6);
  puVar4 = (undefined4 *)*(undefined4 *)(iVar5 + 0x4);
  if ((uVar1 | (uint)puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
  }
  uVar1 = *(uint *)(iVar5 + 0xe);
  uVar2 = *(uint *)(iVar5 + 0x10);
  paStack10 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  *param_1 = 0x389a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  return;
}



void __stdcall16far pass1_1038_6984(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0xc) != 0x0) {
    pass1_1020_c3ae();
    return;
  }
  if (*(long *)(iVar1 + 0xe) != 0x0) {
    pass1_1020_ba94(*(long **)(iVar1 + 0xe));
    return;
  }
  if (*(int *)(iVar1 + 0x12) == 0x0) {
    if (*(int *)(iVar1 + 0x14) == 0x0) {
      return;
    }
    pass1_1020_c42e(*(int *)(iVar1 + 0x14));
  }
  else {
    switch_1020_c3b4(*(ushort *)(iVar1 + 0x12));
  }
  return;
}



void __stdcall16far pass1_1038_69fe(ulong param_1)

{
  *(undefined2 *)((int)param_1 + 0x28) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_6a0e(ulong param_1,undefined2 param_2,undefined2 param_3,ushort param_4,ushort param_5,ushort param_6)

{
  int *piVar1;
  undefined4 uVar2;
  uint uVar3;
  BOOL16 BVar4;
  ushort uVar5;
  uint uVar6;
  uint uVar7;
  ushort uVar8;
  ushort uVar9;
  ushort *puVar10;
  ulong uVar11;
  ulong uStack22;
  undefined local_10 [0x4];
  undefined local_c [0x6];
  ulong uStack6;
  
  uVar9 = (ushort)(param_1 >> 0x10);
  uVar8 = (ushort)param_1;
  if (*(int *)(uVar8 + 0x28) == 0x0) {
    uVar2 = *(undefined4 *)(uVar8 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
    uStack6 = CONCAT22(param_3,param_2);
    piVar1 = (int *)(uVar8 + 0x24);
    *piVar1 = *piVar1 + 0x3c;
    puVar10 = pass1_1008_3e38((ushort *)CONCAT22(param_6,local_c));
    uVar6 = (uint)((ulong)puVar10 >> 0x10);
    while( true ) {
      uVar3 = pass1_1038_6d24(param_1,(ulong *)CONCAT22(param_6,local_10),(ushort *)CONCAT22(param_6,local_c),
                              (int)uStack6,(ushort)(uStack6 >> 0x10),param_6);
      if (uVar3 == 0x0) {
        pass1_1010_8fba(*(ulong *)(uVar8 + 0x4),0x0);
        uStack22 = CONCAT22(uVar6,uVar3);
        uVar7 = uVar6 | uVar3;
        if (uVar7 == 0x0) {
          uVar2 = *(undefined4 *)(uVar8 + 0x20);
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
          pass1_1038_7356(param_1,CONCAT22(uVar7,uVar3),param_6,param_4,param_5);
          return;
        }
        uVar11 = struct_op_1030_73a8(uStack6);
        BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar11 + 0xc),0x40);
        if (BVar4 != 0x0) {
          *(undefined2 *)(uVar8 + 0x28) = 0x1;
          *(ulong *)(uVar8 + 0x20) = uStack22;
          return;
        }
        *(ulong *)(uVar8 + 0x20) = uStack22;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(ushort *)(uVar8 + 0x20),uVar6);
        uStack6 = uStack22 & 0xffff | (ulong)uVar6 << 0x10;
      }
      uVar5 = pass1_1038_6e1a(uVar8,uVar9,(long *)CONCAT22(param_6,local_10));
      if (*(int *)(uVar8 + 0x24) < (int)uVar5) break;
      piVar1 = (int *)(uVar8 + 0x24);
      *piVar1 = *piVar1 - uVar5;
      pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)(uVar8 + 0x1a)),(ushort *)CONCAT22(param_6,local_c));
    }
  }
  return;
}



ushort __stdcall16far pass1_1038_6b3c(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((((*(int *)(iVar1 + 0xc) == 0x0) && (*(int *)(iVar1 + 0x12) == 0x0)) && (*(int *)(iVar1 + 0x14) == 0x0)) &&
     ((*(long *)(iVar1 + 0xe) == 0x0 && (*(long *)(iVar1 + 0x16) != 0x0)))) {
    *(undefined4 *)(iVar1 + 0x16) = 0x0;
  }
  if (*(long *)(iVar1 + 0x16) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_6b88(ushort param_1,ushort param_2,ushort *param_3,ulong *param_4,uchar *param_5,int param_6,ushort param_7)

{
  ulong *puVar1;
  undefined2 uVar2;
  ulong local_12 [0x2];
  long lStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar2 = (undefined2)((ulong)puStack6 >> 0x10);
  lStack10 = *(long *)((int)puStack6 + 0x20);
  puVar1 = local_12;
  pass1_1030_64ce(param_7,puVar1,uVar2,_PTR_LOOP_1050_5740,param_3,lStack10,(ulong *)CONCAT22(param_7,puVar1));
  *param_4 = *puVar1;
  return;
}



void __stdcall16far
pass1_1038_6bd4(ulong param_1,ushort *param_2,ulong *param_3,int param_4,uchar *param_5,int param_6,ushort param_7)

{
  ushort uStack4;
  
  pass1_1008_3f62(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x1a)));
  if (param_4 < 0x0) {
    uStack4 = *param_2 - 0x1;
  }
  else {
    uStack4 = *param_2 + 0x1;
  }
  *param_2 = uStack4;
  pass1_1038_6b88((ushort)param_1,(ushort)(param_1 >> 0x10),param_2,param_3,param_5,param_6,param_7);
  return;
}



void __stdcall16far
pass1_1038_6c1c(ulong param_1,ushort *param_2,ulong *param_3,int param_4,uchar *param_5,int param_6,ushort param_7)

{
  undefined2 uVar1;
  int iStack4;
  
  pass1_1008_3f62(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x1a)));
  uVar1 = (undefined2)((ulong)param_2 >> 0x10);
  iStack4 = *(int *)((int)param_2 + 0x2);
  if (param_4 < 0x0) {
    iStack4 = iStack4 + -0x1;
  }
  else {
    iStack4 = iStack4 + 0x1;
  }
  *(int *)((int)param_2 + 0x2) = iStack4;
  pass1_1038_6b88((ushort)param_1,(ushort)(param_1 >> 0x10),param_2,param_3,param_5,param_6,param_7);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_6c68(ulong param_1,ushort *param_2,ulong *param_3,int param_4,uchar *param_5,int param_6,ushort param_7)

{
  int iVar1;
  ushort uVar2;
  uint uVar3;
  ushort *puVar4;
  uint uVar5;
  uint uVar6;
  uchar *puVar7;
  ushort uVar8;
  ushort *puVar9;
  ulong uVar10;
  int iStack30;
  
  uVar2 = (ushort)param_1;
  pass1_1008_3f62(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(uVar2 + 0x1a)));
  puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar5 = (uint)((ulong)puVar9 >> 0x10);
  puVar4 = (ushort *)(param_1 & 0xffff0000 | (ulong)(uVar2 + 0x1a));
  pass1_1030_627e(param_7,uVar2 + 0x1a,uVar5,_PTR_LOOP_1050_5740,puVar4,*(long *)((int)puVar9 + 0x20));
  uVar3 = (uint)puVar4;
  uVar6 = uVar5 | uVar3;
  if (uVar6 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,uVar5);
    uVar10 = struct_op_1030_73a8(CONCAT22(uVar6,uVar3));
    puVar7 = (uchar *)(uVar10 >> 0x10);
    iVar1 = *(int *)((int)uVar10 + 0xc);
    if ((iVar1 == 0x47) || (iVar1 == 0x6a)) {
      uVar8 = (ushort)(param_1 >> 0x10);
      iStack30 = *(int *)(uVar2 + 0x1e);
      if (param_4 < 0x0) {
        iStack30 = iStack30 + -0x1;
      }
      else {
        iStack30 = iStack30 + 0x1;
      }
      *(int *)((int)param_2 + 0x4) = iStack30;
      pass1_1038_6b88(uVar2,uVar8,param_2,param_3,puVar7,param_6,param_7);
    }
  }
  return;
}



int __stdcall16far
pass1_1038_6d24(ulong param_1,ulong *param_2,ushort *param_3,int param_4,ushort param_5,ushort param_6)

{
  int local_14;
  int local_12;
  int local_10;
  int local_e;
  int local_c;
  int local_a;
  undefined4 local_8;
  undefined2 uStack4;
  
  *param_2 = 0x0;
  local_8 = *(undefined4 *)(param_4 + 0xc);
  uStack4 = *(undefined2 *)(param_4 + 0x10);
  pass1_1008_3eb4((ushort *)CONCAT22(param_6,&local_8),(ushort *)CONCAT22(param_6,&local_e),
                  (ushort *)CONCAT22(param_6,&local_c),(ushort *)CONCAT22(param_6,&local_a));
  pass1_1008_3eb4((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x1a)),(ushort *)CONCAT22(param_6,&local_14),
                  (ushort *)CONCAT22(param_6,&local_12),(ushort *)CONCAT22(param_6,&local_10));
  local_c = local_c - local_12;
  local_e = local_e - local_14;
  local_a = local_a - local_10;
  if (((local_a == 0x0) && (local_c == 0x0)) && (local_e == 0x0)) {
    return 0x0;
  }
  if ((local_c != 0x0) || (local_a == 0x0)) {
    if ((local_a == 0x0) && (local_c != 0x0)) {
      pass1_1038_6c1c(param_1,param_3,param_2,local_c,(uchar *)0x0,(int)&stack0xfffe,param_6);
      return 0x1;
    }
    if (((local_a == 0x0) && (local_c == 0x0)) && (local_e != 0x0)) {
      pass1_1038_6c68(param_1,param_3,param_2,local_e,(uchar *)0x0,(int)&stack0xfffe,param_6);
      if (local_c != 0x0) {
        return 0x1;
      }
      return local_c;
    }
  }
  pass1_1038_6bd4(param_1,param_3,param_2,local_a,(uchar *)local_a,(int)&stack0xfffe,param_6);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1038_6e1a(ushort param_1,ushort param_2,long *param_3)

{
  uint uVar1;
  BOOL16 BVar2;
  ushort uVar3;
  undefined2 uVar4;
  byte bStack21;
  undefined2 uStack4;
  
  uStack4 = 0x0;
  if ((*param_3 == 0x0) && (*(int *)param_3 == 0x0)) {
    return 0x1;
  }
  uVar4 = *(undefined2 *)((int)param_3 + 0x2);
  bStack21 = (byte)((uint)uVar4 >> 0x8);
  uVar1 = (uint)bStack21;
  if (bStack21 == 0x0) {
    uStack4 = *(undefined2 *)param_3;
    goto switchD_1038_6eab_caseD_9;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)*param_3,(uint)((ulong)*param_3 >> 0x10));
  uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar1));
  if ((int)uVar3 < 0xa) {
    switch(uVar3) {
    case 0x1:
      uStack4 = 0x1;
      break;
    case 0x2:
    case 0x6:
      uStack4 = 0x2;
      break;
    case 0x3:
    case 0x7:
      uStack4 = 0x3;
      break;
    case 0x4:
    case 0x8:
      uStack4 = 0x4;
      break;
    case 0x5:
    case 0x9:
      goto switchD_1038_6eab_caseD_5;
    }
  }
  else {
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x41);
    if (BVar2 != 0x0) {
      uStack4 = 0xa;
      goto switchD_1038_6eab_caseD_9;
    }
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x42);
    if ((BVar2 != 0x0) || (uVar3 == 0x3f)) {
      uStack4 = 0xb;
      goto switchD_1038_6eab_caseD_9;
    }
switchD_1038_6eab_caseD_5:
    uStack4 = 0x5;
  }
switchD_1038_6eab_caseD_9:
  switch(uStack4) {
  case 0x1:
    return 0x14;
  case 0x2:
  case 0x7:
    return 0x3c;
  case 0x3:
  case 0x8:
    return 0x78;
  case 0x4:
  case 0x9:
    return 0xf0;
  case 0x5:
  case 0x6:
    return 0xf;
  case 0xa:
    uVar3 = 0xc;
    break;
  case 0xb:
    uVar3 = 0xa;
    break;
  default:
    uVar3 = 0xffff;
  }
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_6f5a(ulong param_1,ulong param_2,uint param_3,uchar *param_4,ushort param_5,ushort param_6,ushort param_7)

{
  undefined4 uVar1;
  long lVar2;
  ushort *puVar3;
  ushort uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  astruct_99 *paStack16;
  uint uStack12;
  undefined2 local_a;
  undefined2 uStack8;
  ushort local_6;
  uint uStack4;
  astruct_623 *uVar3;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(long *)(iVar6 + 0xe) == 0x0) {
    if (*(int *)(iVar6 + 0xc) != 0x0) {
      pass1_1030_7ddc(param_2,*(long *)(iVar6 + 0x16),*(ushort *)(iVar6 + 0xc),param_3,param_4,param_5,param_6,param_7);
      return;
    }
    if (*(int *)(iVar6 + 0x12) != 0x0) {
      pass1_1030_7c50(param_2,*(long *)(iVar6 + 0x16),*(int *)(iVar6 + 0x12),param_3,param_4);
      return;
    }
    paStack16 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
    uVar5 = (uint)((ulong)paStack16 >> 0x10);
    uVar3 = (astruct_623 *)paStack16;
    if ((uVar5 | (uint)uVar3) == 0x0) {
      paStack16 = (astruct_99 *)0x0;
    }
    else {
      paStack16->field_0x0 = 0x389a;
      uVar3->field_0x2 = 0x1008;
      uVar3->field_0x4 = 0x0;
      uVar3->field_0x6 = 0x0;
      uVar3->field_0x8 = 0x0;
      uVar3->field_0xa = 0x0;
      uVar3->field_0xc = 0x0;
      paStack16->field_0x0 = 0x56ce;
      uVar3->field_0x2 = 0x1018;
    }
    uVar9 = (undefined2)((ulong)paStack16 >> 0x10);
    iVar7 = (int)paStack16;
    *(undefined2 *)(iVar7 + 0x8) = *(undefined2 *)(iVar6 + 0x14);
    *(undefined2 *)(iVar7 + 0xa) = *(undefined2 *)(iVar6 + 0x16);
    uVar4 = pass1_1020_c42e(*(int *)(iVar6 + 0x14));
    lVar2 = (ulong)uVar4 * (ulong)*(uint *)(iVar7 + 0xa);
    uVar5 = (uint)lVar2;
    *(uint *)(iVar7 + 0xc) = uVar5;
    pass1_1030_6a2c(param_2,(long)paStack16,uVar5,(uchar *)((ulong)lVar2 >> 0x10),param_7);
  }
  else {
    uVar1 = *(undefined4 *)(iVar6 + 0xe);
    uStack4 = *(uint *)((int)uVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 = uStack12 + 0x1) {
      puVar3 = &local_6;
      pass1_1020_bb16(*(ulong **)(iVar6 + 0xe),(ulong *)CONCAT22(param_7,&local_a),(ushort *)CONCAT22(param_7,puVar3),
                      uStack12);
      if (CONCAT22(uStack8,local_a) != 0x0) {
        pass1_1030_7ddc(param_2,CONCAT22(uStack8,local_a),local_6,(uint)puVar3,param_4,param_5,param_6,param_7);
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_709c(ulong param_1,ulong param_2,uchar *param_3,ushort param_4)

{
  ulong *puVar1;
  long lVar2;
  ushort uVar7;
  uint uVar8;
  uchar *puVar9;
  astruct_618 *iVar8;
  int iVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  astruct_99 *paStack40;
  astruct_99 *paStack16;
  uint uStack12;
  long local_a;
  undefined2 local_6;
  uint uStack4;
  astruct_617 *uVar3;
  astruct_619 *uVar4;
  astruct_620 *uVar5;
  astruct_621 *uVar6;
  
  uVar11 = (undefined2)(param_1 >> 0x10);
  iVar8 = (astruct_618 *)param_1;
  if ((*(uint *)((int)&iVar8->field_0xe + 0x2) | *(uint *)&iVar8->field_0xe) == 0x0) {
    if (iVar8->field_0xc == 0x0) {
      if (iVar8->field_0x12 == 0x0) {
        if (iVar8->field_0x14 == 0x0) {
          return;
        }
        paStack40 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar8 = (uint)((ulong)paStack40 >> 0x10);
        uVar3 = (astruct_617 *)paStack40;
        if ((uVar8 | (uint)uVar3) == 0x0) {
          paStack40 = (astruct_99 *)0x0;
        }
        else {
          paStack40->field_0x0 = 0x389a;
          uVar3->field_0x2 = 0x1008;
          uVar3->field_0x4 = 0x0;
          uVar3->field_0x6 = 0x0;
          uVar3->field_0x8 = 0x0;
          uVar3->field_0xa = 0x0;
          uVar3->field_0xc = 0x0;
          paStack40->field_0x0 = 0x56ce;
          uVar3->field_0x2 = 0x1018;
        }
        uVar12 = (undefined2)((ulong)paStack40 >> 0x10);
        *(int *)((int)paStack40 + 0x8) = iVar8->field_0x14;
        *(undefined2 *)((int)paStack40 + 0xa) = *(undefined2 *)&iVar8->field_0x16;
        uVar8 = pass1_1020_c42e(iVar8->field_0x14);
      }
      else {
        pass1_1030_7c50(param_2,iVar8->field_0x16,iVar8->field_0x12,0x0,param_3);
        paStack40 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar8 = (uint)((ulong)paStack40 >> 0x10);
        uVar4 = (astruct_619 *)paStack40;
        if ((uVar8 | (uint)uVar4) == 0x0) {
          paStack40 = (astruct_99 *)0x0;
        }
        else {
          paStack40->field_0x0 = 0x389a;
          uVar4->field_0x2 = 0x1008;
          uVar4->field_0x4 = 0x0;
          uVar4->field_0x6 = 0x0;
          uVar4->field_0x8 = 0x0;
          uVar4->field_0xa = 0x0;
          uVar4->field_0xc = 0x0;
          paStack40->field_0x0 = 0x56ce;
          uVar4->field_0x2 = 0x1018;
        }
        uVar12 = (undefined2)((ulong)paStack40 >> 0x10);
        *(ushort *)((int)paStack40 + 0x6) = iVar8->field_0x12;
        *(undefined2 *)((int)paStack40 + 0xa) = *(undefined2 *)&iVar8->field_0x16;
        uVar8 = switch_1020_c3b4(iVar8->field_0x12);
      }
      uVar12 = (undefined2)((ulong)paStack40 >> 0x10);
      iVar10 = (int)paStack40;
      lVar2 = (ulong)uVar8 * (ulong)*(uint *)(iVar10 + 0xa);
      puVar9 = (uchar *)((ulong)lVar2 >> 0x10);
      uVar8 = (uint)lVar2;
    }
    else {
      paStack40 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
      uVar8 = (uint)((ulong)paStack40 >> 0x10);
      uVar5 = (astruct_620 *)paStack40;
      puVar9 = (uchar *)(uVar8 | (uint)uVar5);
      if (puVar9 == (uchar *)0x0) {
        paStack40 = (astruct_99 *)0x0;
      }
      else {
        paStack40->field_0x0 = 0x389a;
        uVar5->field_0x2 = 0x1008;
        uVar5->field_0x4 = 0x0;
        uVar5->field_0x6 = 0x0;
        uVar5->field_0x8 = 0x0;
        uVar5->field_0xa = 0x0;
        uVar5->field_0xc = 0x0;
        paStack40->field_0x0 = 0x56ce;
        uVar5->field_0x2 = 0x1018;
      }
      uVar12 = (undefined2)((ulong)paStack40 >> 0x10);
      iVar10 = (int)paStack40;
      *(int *)(iVar10 + 0x4) = iVar8->field_0xc;
      uVar8 = *(uint *)&iVar8->field_0x16;
      *(uint *)(iVar10 + 0xa) = uVar8;
    }
    *(uint *)(iVar10 + 0xc) = uVar8;
    pass1_1030_6a2c(param_2,CONCAT22(uVar12,iVar10),uVar8,puVar9,param_4);
  }
  else {
    puVar1 = iVar8->field_0xe;
    uStack4 = *(uint *)((int)puVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 = uStack12 + 0x1) {
      pass1_1020_bb16(iVar8->field_0xe,(ulong *)CONCAT22(param_4,&local_a),(ushort *)CONCAT22(param_4,&local_6),uStack12
                     );
      if (local_a != 0x0) {
        paStack16 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar8 = (uint)((ulong)paStack16 >> 0x10);
        uVar6 = (astruct_621 *)paStack16;
        if ((uVar8 | (uint)uVar6) == 0x0) {
          paStack16 = (astruct_99 *)0x0;
        }
        else {
          paStack16->field_0x0 = 0x389a;
          uVar6->field_0x2 = 0x1008;
          uVar6->field_0x4 = 0x0;
          uVar6->field_0x6 = 0x0;
          uVar6->field_0x8 = 0x0;
          uVar6->field_0xa = 0x0;
          uVar6->field_0xc = 0x0;
          paStack16->field_0x0 = 0x56ce;
          uVar6->field_0x2 = 0x1018;
        }
        uVar12 = (undefined2)((ulong)paStack16 >> 0x10);
        iVar10 = (int)paStack16;
        *(undefined2 *)(iVar10 + 0x4) = local_6;
        *(undefined2 *)(iVar10 + 0xa) = (undefined2)local_a;
        uVar7 = pass1_1020_c3ae();
        lVar2 = (ulong)uVar7 * (ulong)*(uint *)(iVar10 + 0xa);
        uVar8 = (uint)lVar2;
        *(uint *)(iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(param_2,(long)paStack16,uVar8,(uchar *)((ulong)lVar2 >> 0x10),param_4);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_7356(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uchar **ppuVar1;
  uint *puVar2;
  undefined4 uVar3;
  astruct_18 *paVar4;
  long lVar5;
  BOOL16 BVar6;
  uint uVar7;
  uint uVar9;
  uchar *puVar10;
  uchar *puVar11;
  astruct_615 *iVar9;
  int iVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  bool bVar15;
  ulong uVar16;
  ulong uVar17;
  astruct_99 *paStack50;
  astruct_99 *paStack26;
  astruct_616 *uVar8;
  astruct_622 *uVar10;
  
  uVar16 = struct_op_1030_73a8(param_2);
  puVar10 = (uchar *)(uVar16 >> 0x10);
  uVar7 = (uint)uVar16;
  puVar11 = puVar10;
  BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(uVar7 + 0xc),0x4);
  iVar9 = (astruct_615 *)param_1;
  uVar13 = (undefined2)(param_1 >> 0x10);
  if (BVar6 == 0x0) {
    uVar9 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(uVar7 + 0xc),0x3);
    if (uVar9 == 0x0) {
code_r0x10387545:
      pass1_1038_6f5a(param_1,param_2,uVar9,puVar11,param_4,param_5,param_3);
      goto LAB_1038_7549;
    }
    if ((iVar9->field_0xc != 0x0) || (*(long *)&iVar9->field_0xe != 0x0)) {
      uVar16 = pass1_1028_45e2(uVar16,uVar7,(int)puVar11,param_3);
      puVar11 = (uchar *)(uVar16 >> 0x10);
      uVar9 = (uint)uVar16;
      ppuVar1 = (uchar **)&iVar9->field_0x18;
      bVar15 = *ppuVar1 < puVar11;
      if ((bVar15 || *ppuVar1 == puVar11) &&
         ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9)))) goto code_r0x10387545;
    }
  }
  else {
    uVar17 = pass1_1028_62c8(uVar16,param_3);
    puVar11 = (uchar *)(uVar17 >> 0x10);
    uVar9 = (uint)uVar17;
    ppuVar1 = (uchar **)&iVar9->field_0x18;
    bVar15 = *ppuVar1 < puVar11;
    if ((bVar15 || *ppuVar1 == puVar11) &&
       ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9)))) {
      if (iVar9->field_0x12 == 0x0) {
        if (iVar9->field_0x14 == 0x0) goto LAB_1038_74e0;
        paStack50 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar7 = (uint)((ulong)paStack50 >> 0x10);
        uVar10 = (astruct_622 *)paStack50;
        if ((uVar7 | (uint)uVar10) == 0x0) {
          paStack50 = (astruct_99 *)0x0;
        }
        else {
          paStack50->field_0x0 = 0x389a;
          uVar10->field_0x2 = 0x1008;
          uVar10->field_0x4 = 0x0;
          uVar10->field_0x6 = 0x0;
          uVar10->field_0x8 = 0x0;
          uVar10->field_0xa = 0x0;
          uVar10->field_0xc = 0x0;
          paStack50->field_0x0 = 0x56ce;
          uVar10->field_0x2 = 0x1018;
        }
        uVar14 = (undefined2)((ulong)paStack50 >> 0x10);
        iVar12 = (int)paStack50;
        *(int *)(iVar12 + 0x8) = iVar9->field_0x14;
        *(uint *)(iVar12 + 0xa) = iVar9->field_0x16;
        uVar7 = pass1_1020_c42e(iVar9->field_0x14);
      }
      else {
        paStack26 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar7 = (uint)((ulong)paStack26 >> 0x10);
        uVar8 = (astruct_616 *)paStack26;
        if ((uVar7 | (uint)uVar8) == 0x0) {
          paStack26 = (astruct_99 *)0x0;
        }
        else {
          paStack26->field_0x0 = 0x389a;
          uVar8->field_0x2 = 0x1008;
          uVar8->field_0x4 = 0x0;
          uVar8->field_0x6 = 0x0;
          uVar8->field_0x8 = 0x0;
          uVar8->field_0xa = 0x0;
          uVar8->field_0xc = 0x0;
          paStack26->field_0x0 = 0x56ce;
          uVar8->field_0x2 = 0x1018;
        }
        uVar14 = (undefined2)((ulong)paStack26 >> 0x10);
        iVar12 = (int)paStack26;
        *(ushort *)(iVar12 + 0x6) = iVar9->field_0x12;
        *(uint *)(iVar12 + 0xa) = iVar9->field_0x16;
        uVar7 = switch_1020_c3b4(iVar9->field_0x12);
      }
      lVar5 = (ulong)uVar7 * (ulong)*(uint *)(iVar12 + 0xa);
      puVar11 = (uchar *)((ulong)lVar5 >> 0x10);
      uVar9 = (uint)lVar5;
      *(uint *)(iVar12 + 0xc) = uVar9;
      pass1_1028_6408(uVar16,(ulong *)CONCAT22(uVar14,iVar12),param_3);
      goto LAB_1038_7549;
    }
  }
LAB_1038_74e0:
  pass1_1038_709c(param_1,param_2,puVar11,param_3);
LAB_1038_7549:
  uVar3 = iVar9->field_0x8;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar3,(uint)((ulong)uVar3 >> 0x10));
  pass1_1030_6c4c(CONCAT22(puVar11,uVar9),*(int *)(uVar9 + 0x34) + iVar9->field_0x26);
  iVar9->field_0xc = 0x0;
  iVar9->field_0x12 = 0x0;
  iVar9->field_0x14 = 0x0;
  *(undefined4 *)&iVar9->field_0x16 = 0x0;
  paVar4 = *(astruct_18 **)&iVar9->field_0xe;
  uVar7 = iVar9->field_0x10;
  if ((uVar7 | (uint)paVar4) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)((ulong)paVar4 & 0xffff | (ulong)uVar7 << 0x10));
    fn_ptr_1000_17ce(paVar4,0x1000);
  }
  *(undefined4 *)&iVar9->field_0xe = 0x0;
  return;
}
