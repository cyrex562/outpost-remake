
void __stdcall16far pass1_1028_6356(ulong param_1,int param_2,uint param_3,int param_4,ushort param_5)

{
  int *piVar1;
  uint uVar2;
  uint uVar3;
  code **ppcVar4;
  int iVar5;
  undefined2 uVar6;
  int iVar7;
  undefined2 uVar8;
  bool bVar9;
  long lVar10;
  undefined local_a [0x4];
  undefined4 uStack6;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_a),*(ulong *)(iVar7 + 0x20));
  while( true ) {
    do {
      lVar10 = pass1_1008_5b12(local_a,param_5);
      uVar6 = (undefined2)((ulong)lVar10 >> 0x10);
      iVar5 = (int)lVar10;
      if (lVar10 == 0x0) {
        return;
      }
    } while (((*(int *)(iVar5 + 0x8) == 0x0) || ((param_2 != 0x0 && (*(int *)(iVar5 + 0x8) != param_2)))) ||
            ((*(int *)(iVar5 + 0x8) == 0xf && (param_2 != 0xf))));
    uVar2 = *(uint *)(iVar5 + 0xa);
    if ((param_4 == 0x0) && (param_3 < uVar2)) break;
    bVar9 = param_3 < uVar2;
    param_3 = param_3 - uVar2;
    param_4 = param_4 - (uint)bVar9;
    ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0x20) + 0xc);
    (**ppcVar4)(0x1008,*(undefined4 *)(iVar7 + 0x20));
    uStack6 = 0x0;
  }
  uVar3 = *(uint *)(iVar5 + 0xc);
  piVar1 = (int *)(iVar5 + 0xa);
  *piVar1 = *piVar1 - param_3;
  piVar1 = (int *)(iVar5 + 0xc);
  *piVar1 = *piVar1 - param_3 * (uVar3 / uVar2);
  return;
}



void __stdcall16far pass1_1028_6408(ulong param_1,ulong *param_2,ushort param_3)

{
  code **ppcVar1;
  bool bVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  undefined local_a [0x8];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar4 + 0x20));
  bVar2 = false;
  while( true ) {
    puVar3 = local_a;
    pass1_1008_5b12(puVar3,param_3);
    iVar5 = (int)param_2;
    uVar7 = (undefined2)((ulong)param_2 >> 0x10);
    if ((extraout_DX | (uint)puVar3) == 0x0) break;
    if ((*(int *)(puVar3 + 0x4) == *(int *)(iVar5 + 0x4)) && (*(int *)(puVar3 + 0x6) == *(int *)(iVar5 + 0x6))) {
      if (*(int *)(puVar3 + 0x8) == *(int *)(iVar5 + 0x8)) {
        bVar2 = true;
        *(int *)(puVar3 + 0xa) = *(int *)(puVar3 + 0xa) + *(int *)(iVar5 + 0xa);
        *(int *)(puVar3 + 0xc) = *(int *)(puVar3 + 0xc) + *(int *)(iVar5 + 0xc);
      }
    }
  }
  if (bVar2) {
    if (param_2 != (ulong *)0x0) {
      ppcVar1 = (code **)*param_2;
      (**ppcVar1)(0x1008,param_2,0x1,param_2,param_2);
      return;
    }
  }
  else {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x20) + 0x4);
    (**ppcVar1)(0x1008,*(undefined4 *)(iVar4 + 0x20),param_2);
  }
  return;
}



undefined2 __stdcall16far pass1_1028_64d6(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  undefined2 *puVar3;
  undefined2 uVar4;
  ushort uVar5;
  ushort uVar6;
  undefined2 local_26;
  undefined2 local_24;
  undefined2 local_22;
  undefined2 local_20;
  undefined2 local_1e;
  undefined2 local_1c [0x6];
  undefined2 uStack16;
  long lStack14;
  undefined local_a [0x8];
  
  BVar2 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar2 != 0x0) {
    uVar4 = (undefined2)(param_1 >> 0x10);
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x20));
    uVar1 = *(undefined4 *)((int)param_1 + 0x20);
    local_1c[0] = *(undefined2 *)((int)uVar1 + 0x8);
    puVar3 = local_1c;
    uStack16 = local_1c[0];
    while( true ) {
      uVar5 = (ushort)param_2;
      uVar6 = (ushort)(param_2 >> 0x10);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)puVar3,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) break;
      lStack14 = pass1_1008_5b12(local_a,param_3);
      if (lStack14 == 0x0) {
        return 0x1;
      }
      local_1e = *(undefined2 *)((int)lStack14 + 0x4);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_1e,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) break;
      local_20 = *(undefined2 *)((int)lStack14 + 0x6);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_20,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) break;
      local_22 = *(undefined2 *)((int)lStack14 + 0x8);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_22,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) break;
      local_24 = *(undefined2 *)((int)lStack14 + 0xa);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_24,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) break;
      local_26 = *(undefined2 *)((int)lStack14 + 0xc);
      puVar3 = &local_26;
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_65e2(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  code **ppcVar1;
  uint uVar2;
  BOOL16 BVar3;
  ushort uVar4;
  uint uVar5;
  undefined2 uVar6;
  ushort uVar7;
  ushort uVar8;
  undefined2 local_16;
  astruct_99 *paStack20;
  undefined2 local_10 [0x2];
  ushort local_c [0x3];
  uint uStack6;
  uint local_4;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    uVar7 = (ushort)param_2;
    uVar8 = (ushort)(param_2 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)&local_4,0x0,param_5,0x2,0x1008);
    if (BVar3 != 0x0) {
      uStack6 = 0x0;
      while( true ) {
        if (local_4 <= uStack6) {
          return;
        }
        paStack20 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar5 = (uint)((ulong)paStack20 >> 0x10);
        uVar2 = (uint)paStack20;
        if ((uVar5 | uVar2) == 0x0) {
          paStack20 = (astruct_99 *)0x0;
        }
        else {
          paStack20->field_0x0 = 0x389a;
          *(undefined2 *)(uVar2 + 0x2) = 0x1008;
          *(undefined2 *)(uVar2 + 0x4) = 0x0;
          *(undefined2 *)(uVar2 + 0x6) = 0x0;
          *(undefined2 *)(uVar2 + 0x8) = 0x0;
          *(undefined2 *)(uVar2 + 0xa) = 0x0;
          *(undefined2 *)(uVar2 + 0xc) = 0x0;
          paStack20->field_0x0 = 0x56ce;
          *(undefined2 *)(uVar2 + 0x2) = 0x1018;
        }
        BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)local_10,0x0,param_5,0x2,0x1008);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)local_c,0x0,param_5,0x2,0x1008);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)&local_16,0x0,param_5,0x2,0x1008);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(uVar7,uVar8,(int)paStack20 + 0xa,0x0,(ushort)((ulong)paStack20 >> 0x10),0x2,0x1008);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(uVar7,uVar8,(int)paStack20 + 0xc,0x0,(ushort)((ulong)paStack20 >> 0x10),0x2,0x1008);
        if (BVar3 == 0x0) break;
        *(undefined2 *)((int)paStack20 + 0x4) = local_10[0];
        uVar4 = switch_1008_72bc(uVar7,uVar8,local_c[0]);
        uVar6 = (undefined2)((ulong)paStack20 >> 0x10);
        *(ushort *)((int)paStack20 + 0x6) = uVar4;
        *(undefined2 *)((int)paStack20 + 0x8) = local_16;
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x20) + 0x8);
        (**ppcVar1)();
        uStack6 = uStack6 + 0x1;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



ushort __stdcall16far pass1_1028_6744(ushort param_1,ulong param_2,int param_3)

{
  undefined2 uVar1;
  long lVar2;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_1,local_a),*(ulong *)((int)param_2 + 0x20));
  do {
    lVar2 = pass1_1008_5b12(local_a,param_1);
    uVar1 = (undefined2)((ulong)lVar2 >> 0x10);
    if (lVar2 == 0x0) {
      return 0x0;
    }
  } while (*(int *)((int)lVar2 + 0x6) != param_3);
  return *(ushort *)((int)lVar2 + 0xa);
}



ushort __stdcall16far pass1_1028_678c(ulong param_1,int param_2,ushort param_3)

{
  undefined2 uVar1;
  long lVar2;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x20));
  do {
    lVar2 = pass1_1008_5b12(local_a,param_3);
    uVar1 = (undefined2)((ulong)lVar2 >> 0x10);
    if (lVar2 == 0x0) {
      return 0x0;
    }
  } while (*(int *)((int)lVar2 + 0x8) != param_2);
  return *(ushort *)((int)lVar2 + 0xa);
}



// WARNING: Could not reconcile some variable overlaps

ulong __stdcall16far pass1_1028_67d4(ulong param_1,ushort param_2)

{
  uint uVar1;
  long lVar2;
  ulong uStack18;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_2,local_a),*(ulong *)((int)param_1 + 0x20));
  uStack18 = 0x0;
  while( true ) {
    lVar2 = pass1_1008_5b12(local_a,param_2);
    if (lVar2 == 0x0) break;
    uVar1 = *(uint *)((int)lVar2 + 0xc);
    uStack18 = CONCAT22((int)(uStack18 >> 0x10) + (uint)CARRY2((uint)uStack18,uVar1),(uint)uStack18 + uVar1);
  }
  return uStack18;
}



ushort __stdcall16far pass1_1028_6822(ulong param_1,uint *param_2,ushort param_3)

{
  int iVar1;
  ulong uVar2;
  
  uVar2 = pass1_1028_67d4(param_1,param_3);
  iVar1 = (int)(uVar2 >> 0x10);
  *param_2 = (uint)uVar2;
  *(int *)((int)param_2 + 0x2) = iVar1;
  if ((iVar1 == 0x0) && (*param_2 < 0x64)) {
    return 0x0;
  }
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1028_6850(astruct_18 *param_1,byte param_2)

{
  pass1_1028_6186(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1028_68de(astruct_100 *param_1,ushort param_2,ulong param_3,uchar param_4,ushort param_5)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_op_1028_d1dc(param_5,param_4,param_1,0x3e8);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x108) = param_3;
  *(ushort *)(iVar1 + 0x10c) = param_2;
  param_1->field_0x0 = 0x6ae2;
  *(undefined2 *)(iVar1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x8)),s_SCAddSpew_1050_4fd2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_6926(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uint uVar1;
  undefined4 uVar2;
  code **ppcVar3;
  uint uVar4;
  uint uVar5;
  uchar *puVar6;
  uint extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar7;
  ushort uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ulong *puVar11;
  undefined4 *puStack14;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  uVar2 = *(undefined4 *)((int)param_1 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
  puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0xa);
  puVar6 = (uchar *)((ulong)puVar11 >> 0x10);
  uVar4 = (uint)puVar11;
  uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4d6e(CONCAT22(param_3,param_2),puVar11,uVar4,puVar6);
  puStack14 = (undefined4 *)CONCAT22(puVar6,uVar4);
  uVar2 = *puStack14;
  uVar8 = (ushort)uVar2;
  ppcVar3 = (code **)(uVar8 + 0x10);
  uVar5 = uVar4;
  (**ppcVar3)((int)&PTR_LOOP_1050_1038,uVar4,puVar6);
  if ((extraout_DX | uVar5) != 0x0) {
    ppcVar3 = (code **)(uVar8 + 0x4);
    (**ppcVar3)(0x38,uVar4,puVar6,0x0,0x0);
    puVar7 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,(uint)extraout_DX_00);
    uVar1 = *(uint *)((int)param_1 + 0x10c);
    uVar10 = 0x1030;
    pass1_1030_7ddc(CONCAT22(puVar7,uVar5),CONCAT13((undefined)((int)uVar1 >> 0xf),(int3)(int)uVar1),0x1f,uVar1,puVar7,
                    uVar8,(ushort)((ulong)uVar2 >> 0x10),param_4);
  }
  if (puStack14 != (undefined4 *)0x0) {
    ppcVar3 = (code **)*puStack14;
    (**ppcVar3)(uVar10,uVar4,(char)puVar6,0x1);
  }
  return;
}



void __stdcall16far pass1_1028_69cc(ulong param_1,astruct_317 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_316 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10e,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_316 *)param_1;
    param_2->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_2->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    param_2->field_0x108 = iVar5->field_0x108;
    param_2->field_0x10c = iVar5->field_0x10c;
    *puStack10 = 0x6ae2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_6a7a(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1028_6aa6(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1028_6af2(astruct_100 *param_1,ulong param_2,ulong param_3,ushort param_4,uchar param_5)

{
  astruct_683 *iVar1;
  undefined2 uVar1;
  
  struct_op_1028_d1dc(param_4,param_5,param_1,0x1387);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_683 *)param_1;
  iVar1->field_0x108 = param_3;
  iVar1->field_0x10c = param_2;
  param_1->field_0x0 = 0x6e50;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  return;
}



ushort __stdcall16far pass1_1028_6b2c(ulong param_1,ushort param_2)

{
  pass1_1028_6b40(param_1,param_2);
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_6b40(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  code **ppcVar2;
  undefined *puVar3;
  ushort in_DX;
  ushort uVar4;
  ushort uVar5;
  ushort uVar6;
  ushort *puVar7;
  undefined local_36 [0xe];
  undefined4 *puStack40;
  ushort uStack38;
  ushort uStack36;
  ushort uStack34;
  ushort uStack32;
  ushort uStack30;
  ushort uStack28;
  ushort uStack26;
  undefined4 local_18;
  ushort uStack20;
  ulong uStack18;
  undefined4 uStack14;
  undefined4 *puStack10;
  undefined local_6 [0x2];
  int local_4;
  
  puVar3 = local_6;
  pass1_1028_6daa(param_1,(ushort *)CONCAT22(param_2,puVar3),(ushort *)CONCAT22(param_2,&local_4),(ushort)puVar3,in_DX,
                  param_2);
  uVar6 = (ushort)(param_1 >> 0x10);
  uVar5 = (ushort)param_1;
  uVar1 = *(undefined4 *)(uVar5 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  puStack10 = (undefined4 *)CONCAT22(in_DX,puVar3);
  ppcVar2 = (code **)((int)*puStack10 + 0x24);
  (**ppcVar2)();
  uStack14 = pass1_1028_b58e((ulong)puStack10);
  uStack18 = pass1_1028_bb24((ulong)puStack10);
  local_18 = *(undefined4 *)((int)uStack14 + 0xc);
  uStack20 = *(ushort *)((int)uStack14 + 0x10);
  puStack40 = &local_18;
  uStack26 = (ushort)local_18;
  uStack28 = (ushort)((ulong)local_18 >> 0x10);
  uStack32 = (ushort)local_18 - 0x1;
  if ((int)uStack32 < 0x0) {
    uStack32 = 0x0;
  }
  uVar4 = local_4 - 0x1;
  uStack34 = (ushort)local_18 + 0x1;
  if ((int)uVar4 < (int)((ushort)local_18 + 0x1)) {
    uStack34 = uVar4;
  }
  uStack36 = uStack28 - 0x1;
  if ((int)uStack36 < 0x0) {
    uStack36 = 0x0;
  }
  uStack38 = uStack28 + 0x1;
  if ((int)uVar4 < (int)(uStack28 + 0x1)) {
    uStack38 = uVar4;
  }
  uStack30 = uStack20;
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack20,uStack36,uStack32);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack30,uStack36,uStack26);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack30,uStack36,uStack34);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack30,uStack28,uStack32);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack30,uStack28,uStack34);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack30,uStack38,uStack32);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack30,uStack38,uStack26);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2,local_36),uStack30,uStack38,uStack34);
  pass1_1028_6d24(uVar5,uVar6,(ushort *)CONCAT22(param_2,local_36),uStack18,(ushort)((ulong)puVar7 >> 0x10),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_6d24(ushort param_1,ushort param_2,ushort *param_3,long param_4,ushort param_5,ushort param_6)

{
  int iVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  uint uVar4;
  uint uVar5;
  undefined4 *puVar6;
  byte bStack27;
  undefined4 local_a;
  undefined4 uStack6;
  
  puVar3 = &local_a;
  pass1_1030_64ce(param_6,puVar3,param_5,_PTR_LOOP_1050_5740,param_3,param_4,(ulong *)CONCAT22(param_6,puVar3));
  uStack6 = *puVar3;
  uVar5 = *(uint *)((int)puVar3 + 0x2);
  bStack27 = (byte)((ulong)uStack6 >> 0x18);
  uVar4 = (uint)bStack27;
  if (bStack27 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack6,uVar5);
    puVar6 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar5,uVar4));
    iVar1 = *(int *)((int)puVar6 + 0xc);
    if (((iVar1 < 0x1) || (SBORROW2(iVar1,0x1))) ||
       ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 && ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73)))))) {
      ppcVar2 = (code **)((int)*puVar6 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_6daa(ulong param_1,ushort *param_2,ushort *param_3,ushort param_4,ushort param_5,ushort param_6)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  undefined4 local_18;
  undefined2 uStack20;
  int iStack18;
  ushort uStack16;
  undefined4 uStack14;
  ulong uStack10;
  ushort uStack6;
  ushort uStack4;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  uStack6 = param_4;
  uStack4 = param_5;
  uStack10 = pass1_1028_b4f2(CONCAT22(param_5,param_4));
  uStack16 = (ushort)(uStack10 >> 0x10);
  uVar1 = *(undefined4 *)((int)uStack10 + 0x8);
  uStack14 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  iStack18 = (int)uVar1;
  puVar2 = (undefined4 *)pass1_1030_5b5c(iStack18,uStack16);
  local_18 = *puVar2;
  uStack20 = *(undefined2 *)((int)puVar2 + 0x4);
  pass1_1008_3e94((ushort *)CONCAT22(param_6,&local_18),param_2,param_3);
  return;
}



astruct_18 * __stdcall16far pass1_1028_6e24(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_6e60(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x32c7);
  param_1->field_0x0 = 0x6fb0;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCConstruct_1050_4fdc);
  return param_1;
}



ushort __stdcall16far pass1_1028_6e96(uint param_1,ushort param_2)

{
  int iVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint uVar4;
  uint extraout_DX;
  undefined4 *puStack24;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x700);
  while( true ) {
    puVar3 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar3));
    puStack24 = (undefined4 *)CONCAT22(param_1,puVar3);
    uVar4 = param_1 | (uint)puVar3;
    if (uVar4 == 0x0) break;
    iVar1 = *(int *)(puVar3 + 0x12);
    param_1 = uVar4;
    if (((0x0 < iVar1) && (!SBORROW2(iVar1,0x1))) && (iVar1 + -0x1 < 0x4)) {
      ppcVar2 = (code **)((int)*puStack24 + 0x38);
      (**ppcVar2)();
      param_1 = extraout_DX;
    }
  }
  return 0x1;
}



void __stdcall16far pass1_1028_6ef6(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *puStack10 = 0x6fb0;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_6f84(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_6fc0(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x3e7);
  param_1->field_0x0 = 0x749e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCEndSim_1050_4fea);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_6ff6(ulong param_1,uint param_2,int param_3,ushort param_4)

{
  long *plVar1;
  undefined *puVar2;
  astruct_67 *paVar3;
  uchar *puVar4;
  uchar *puVar5;
  uint uVar6;
  uint uVar7;
  ushort *puVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  int iVar11;
  undefined uVar12;
  undefined uVar13;
  undefined uVar14;
  undefined2 uVar15;
  ushort uVar16;
  undefined2 uVar17;
  int iVar18;
  undefined local_46 [0x12];
  undefined4 uStack52;
  int iStack48;
  uchar *puStack46;
  astruct_67 *paStack38;
  undefined *puStack34;
  uchar *puStack32;
  int iStack30;
  int iStack28;
  int iStack26;
  ulong uStack24;
  undefined local_14 [0x8];
  undefined2 uStack12;
  uchar *puStack10;
  undefined2 uStack8;
  uchar *puStack6;
  int iStack4;
  
  uVar13 = (undefined)(param_4 >> 0x8);
  pass1_1028_dc52((astruct_92 *)CONCAT13(uVar13,CONCAT12((char)param_4,local_14)),0x1,0x0,0x400);
  iStack26 = 0x1;
  iStack28 = 0x0;
  do {
    do {
      uVar7 = param_2;
      puVar2 = local_14;
      pass1_1028_e4ec(CONCAT22(param_4,puVar2));
      uStack24 = CONCAT22(uVar7,puVar2);
      param_2 = uVar7 | (uint)puVar2;
      if (param_2 == 0x0) goto LAB_1028_7066;
    } while ((*(int *)(puVar2 + 0x1fe) == 0x0) || (*(long *)(puVar2 + 0x200) == 0x8000002));
    iStack28 = 0x1;
    paVar3 = *(astruct_67 **)(puVar2 + 0x1f6);
    paStack38 = paVar3;
    pass1_1030_38b8();
  } while (((int)param_2 < 0x0) || (((int)param_2 < 0x1 && ((int)paVar3 == 0x0))));
  iStack26 = 0x0;
LAB_1028_7066:
  puStack10 = puStack6;
  uStack12 = uStack8;
  if (iStack4 != 0x0) {
    puStack10 = (uchar *)0x0;
    uStack12 = 0x1;
  }
  iStack30 = 0x0;
  puVar4 = puStack10;
  while( true ) {
    puVar2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar2));
    uStack24 = CONCAT22(puVar4,puVar2);
    puStack32 = (uchar *)((uint)puVar4 | (uint)puVar2);
    if (puStack32 == (uchar *)0x0) break;
    plVar1 = (long *)(puVar2 + 0x200);
    puVar4 = puStack32;
    if (*plVar1 == 0x8000001) {
      iStack30 = 0x1;
    }
  }
  if (iStack30 == 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
    uStack24 = CONCAT22(puStack32,puVar2);
    puStack32 = (uchar *)((uint)puStack32 | (uint)puVar2);
    if (puStack32 != (uchar *)0x0) {
      PTR_LOOP_1050_4fe8 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
      uVar16 = 0x0;
      iVar11 = 0x1;
      uStack52 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,puStack32,param_3);
      puStack32 = (uchar *)((ulong)uStack52 >> 0x10);
      puVar2 = (undefined *)uStack52;
      pass1_1010_089e(param_4,(ulong)uStack52,uVar16,iVar11);
      pass1_1010_089e(param_4,(ulong)uStack52,0x0,0x2);
      pass1_1010_089e(param_4,(ulong)uStack52,0x0,0x3);
      pass1_1010_089e(param_4,(ulong)uStack52,0x0,0x4);
      pass1_1010_089e(param_4,(ulong)uStack52,0x0,0x5);
      pass1_1010_089e(param_4,(ulong)uStack52,0x0,0x7);
      pass1_1010_089e(param_4,(ulong)uStack52,0x0,0x8);
      pass1_1010_089e(param_4,(ulong)uStack52,0x0,0xa);
    }
  }
  if ((iStack28 != 0x0) && (iStack26 != 0x0)) {
    uVar17 = 0x0;
    iVar18 = 0x6;
    uVar12 = 0x1;
    uVar14 = 0x0;
    uVar15 = 0x0;
    uVar10 = 0x0;
    iVar11 = 0x0;
    uVar9 = 0x0;
    uStack52 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,puStack32,param_3);
    puStack32 = (uchar *)((ulong)uStack52 >> 0x10);
    puVar2 = (undefined *)uStack52;
    post_win_msg_1008_a0e4
              (uStack52,CONCAT22(uVar10,uVar9),iVar11,CONCAT11(uVar14,uVar12),CONCAT22(uVar17,uVar15),iVar18,0x1008,
               param_4);
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x800);
  puVar4 = (uchar *)((uint)puStack32 | (uint)puVar2);
  puStack34 = puVar2;
  if (((((puVar4 != (uchar *)0x0) &&
        (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32,puVar2),0x4), puVar2 == (undefined *)0x0)) &&
       (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32,puStack34),0x2a), puVar2 == (undefined *)0x0)) &&
      ((puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32,puStack34),0x4b), puVar2 == (undefined *)0x0 &&
       (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32,puStack34),0x54), puVar2 == (undefined *)0x0)))) &&
     ((puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32,puStack34),0x2c), puVar2 == (undefined *)0x0 &&
      ((puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32,puStack34),0x3c), puVar2 == (undefined *)0x0 &&
       (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32,puStack34),0x3d), puVar2 == (undefined *)0x0)))))) {
    if (iStack4 != 0x0) {
      uStack8 = 0x1;
      puStack6 = (uchar *)0x0;
    }
    uStack52 = (astruct_67 *)((ulong)uStack52 & 0xffff0000);
    iStack48 = 0x0;
    uStack12 = uStack8;
    puStack10 = puStack6;
    do {
      do {
        puVar5 = puStack6;
        puVar2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4,puVar2));
        uStack24 = CONCAT22(puVar5,puVar2);
        puVar4 = (uchar *)((uint)puVar5 | (uint)puVar2);
        if (puVar4 == (uchar *)0x0) goto LAB_1028_72d3;
        puStack6 = puVar4;
      } while (*(long *)(puVar2 + 0x200) == 0x8000002);
      uVar16 = (ushort)(param_1 >> 0x10);
      if (((int)uStack52 == 0x0) &&
         (pass1_1028_740c((ushort)param_1,uVar16,0x22,CONCAT22(puVar5,puVar2)), puVar2 != (undefined *)0x0)) {
        uStack52 = (astruct_67 *)CONCAT22(uStack52._2_2_,0x1);
      }
      if ((iStack48 == 0x0) && (pass1_1028_740c((ushort)param_1,uVar16,0x24,uStack24), puVar2 != (undefined *)0x0)) {
        iStack48 = 0x1;
      }
      puStack6 = puVar4;
    } while (((int)uStack52 == 0x0) || (iStack48 == 0x0));
    uVar17 = 0x0;
    iVar18 = 0x14;
    uVar12 = 0x1;
    uVar14 = 0x0;
    uVar15 = 0x0;
    uVar10 = 0x0;
    iVar11 = 0x0;
    uVar9 = 0x0;
    paStack38 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,puVar4,param_3);
    puVar4 = (uchar *)((ulong)paStack38 >> 0x10);
    puVar2 = (undefined *)paStack38;
    post_win_msg_1008_a0e4
              (paStack38,CONCAT22(uVar10,uVar9),iVar11,CONCAT11(uVar14,uVar12),CONCAT22(uVar17,uVar15),iVar18,0x1008,
               param_4);
  }
LAB_1028_72d3:
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
  uStack24 = CONCAT22(puVar4,puVar2);
  if ((uchar *)((uint)puVar4 | (uint)puVar2) != (uchar *)0x0) {
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_4,(uchar *)((uint)puVar4 | (uint)puVar2),param_3);
    puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
    iStack48 = (int)puVar8;
    puStack46 = puVar4;
    pass1_1008_df4a((ulong)puVar8,param_3,param_4);
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_4,puVar4,param_3);
    uVar7 = (uint)((ulong)puVar8 >> 0x10);
    iStack48 = (int)puVar8;
    puStack46 = (uchar *)uVar7;
    pass1_1018_34a6((ulong)puVar8);
    pass1_1028_dc52((astruct_92 *)CONCAT13(uVar13,CONCAT12((char)param_4,local_46)),0x1,0x0,0x400);
    while( true ) {
      uVar6 = uVar7;
      puVar2 = local_46;
      pass1_1028_e4ec(CONCAT22(param_4,puVar2));
      uStack52 = (astruct_67 *)CONCAT22(uVar6,puVar2);
      uVar7 = uVar6 | (uint)puVar2;
      if (uVar7 == 0x0) break;
      if (*(long *)(puVar2 + 0x200) != 0x8000002) {
        pass1_1038_3ba0(CONCAT22(uVar6,puVar2));
      }
    }
  }
  return;
}



void __stdcall16far pass1_1028_737e(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x108,(uchar *)param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *puStack10 = 0x749e;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_740c(ushort param_1,ushort param_2,int param_3,ulong param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  uchar *puVar5;
  undefined2 extraout_DX;
  ulong *puVar6;
  long lStack14;
  undefined4 *puStack10;
  
  puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,param_3);
  puVar5 = (uchar *)((ulong)puVar6 >> 0x10);
  uVar3 = (uint)puVar6;
  pass1_1038_4d6e(param_4,puVar6,uVar3,puVar5);
  puStack10 = (undefined4 *)CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar1 = (code **)uVar2 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar3,puVar5);
  lStack14 = CONCAT22(extraout_DX,uVar4);
  if (puStack10 != (undefined4 *)0x0) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar3,(char)puVar5,0x1);
  }
  if (lStack14 != 0x0) {
    return;
  }
  return;
}

