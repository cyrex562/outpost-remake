// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_a3ae(ushort param_1,ushort param_2,ulong param_3,long param_4,int param_5,ushort param_6,uchar param_7,
               int param_8)

{
  uint uVar1;
  ushort uVar2;
  BOOL16 BVar3;
  uint uVar4;
  ulong uVar5;
  uchar *puVar6;
  uint uVar7;
  ushort *puVar8;
  int iVar9;
  undefined2 uVar10;
  undefined2 local_146;
  undefined2 uStack324;
  undefined2 uStack32;
  undefined2 uStack30;
  ulong uStack26;
  ulong uStack22;
  uint uStack18;
  uint uStack16;
  undefined4 uStack14;
  ulong uStack10;
  int iStack6;
  undefined2 uStack4;
  
  iVar9 = (int)param_3;
  uVar10 = (undefined2)(param_3 >> 0x10);
  pass1_1038_3fb0(param_3);
  uStack4 = (undefined2)param_4;
  iStack6 = param_8;
  if ((*(int *)(iVar9 + 0x204) != 0x0) && (BVar3 = pass1_1030_25b2(CONCAT22(uStack4,param_8),0x82), BVar3 != 0x0)) {
    return;
  }
  uVar5 = *(ulong *)(iVar9 + 0x1f6);
  uStack10 = uVar5;
  pass1_1030_38b8();
  uVar4 = (uint)uVar5;
  uStack16 = (uint)param_4;
  uStack14 = uVar5 & 0xffff | param_4 << 0x10;
  empty_1038_540a();
  puVar6 = (uchar *)(uStack16 | uVar4);
  uStack18 = uVar4;
  if ((((puVar6 == (uchar *)0x0) && (*(long *)(iVar9 + 0x200) != 0x8000002)) && (pass1_1030_38b8(), -0x1 < (int)puVar6))
     && ((0x0 < (int)puVar6 || (uVar4 != 0x0)))) {
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,puVar6,param_5);
    uStack30 = (undefined2)((ulong)puVar8 >> 0x10);
    uStack32 = SUB42(puVar8,0x0);
    pass1_1010_043a((ulong)puVar8,*(long *)(iVar9 + 0x4),0x11,param_6);
  }
  uVar2 = uStack16;
  uVar1 = uStack18;
  uStack26 = uStack14;
  uVar4 = uStack18 * 0xa;
  uVar7 = (uStack16 * 0x5 + (uint)CARRY2(uStack18,uStack18) * 0x2 + (uint)CARRY2(uStack18 * 0x2,uStack18 * 0x2) +
          (uint)CARRY2(uStack18 * 0x4,uStack18)) * 0x2 + (uint)CARRY2(uStack18 * 0x5,uStack18 * 0x5);
  uStack22 = CONCAT22(uVar7,uVar4);
  if ((uVar7 <= uStack14._2_2_) && ((uVar7 < uStack14._2_2_ || (uVar4 < (uint)uStack14)))) {
    pass1_1028_ae66((astruct_100 *)CONCAT22(param_6,&local_146),uStack14,CONCAT22(uVar7,uVar4),*(ulong *)(iVar9 + 0x4),
                    param_6,param_7);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,&local_146));
    uStack26 = uStack22;
    local_146 = 0x389a;
    uStack324 = 0x1008;
  }
  uStack26 = uStack26 + 0x9;
  pass1_1038_52b8(param_3,uStack26 / 0xa,0x1e,uVar1,uVar2,(ushort)&PTR_LOOP_1050_1038,param_6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_a4ee(ulong param_1,ulong param_2,int param_3,ushort param_4)

{
  ulong uVar1;
  code **ppcVar2;
  uint uVar3;
  BOOL16 BVar4;
  uint uVar5;
  ulong uVar6;
  uchar *puVar7;
  uchar *puVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ulong *puVar11;
  ushort uVar12;
  int iStack50;
  undefined4 *puStack18;
  
  uVar9 = (undefined2)(param_2 >> 0x10);
  uVar1 = *(ulong *)((int)param_2 + 0x1f6);
  uVar6 = *_PTR_LOOP_1050_65e2;
  puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x26);
  puVar7 = (uchar *)((ulong)puVar11 >> 0x10);
  uVar5 = (uint)puVar11;
  uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4d6e(param_2,puVar11,uVar5,puVar7);
  puStack18 = (undefined4 *)CONCAT22(puVar7,uVar5);
  ppcVar2 = (code **)((int)*puStack18 + 0x10);
  uVar3 = uVar5;
  puVar8 = puVar7;
  (**ppcVar2)((int)&PTR_LOOP_1050_1038,uVar5,puVar7);
  if (((uint)puVar8 | uVar3) != 0x0) {
    uVar10 = 0x1030;
    pass1_1030_3548(uVar1,CONCAT22(puVar8,uVar3));
  }
  if (puStack18 != (undefined4 *)0x0) {
    ppcVar2 = (code **)*puStack18;
    (**ppcVar2)(uVar10,uVar5,(char)puVar7,0x1);
  }
  uVar3 = (uint)(uVar6 % 0xc);
  uVar12 = (ushort)(param_1 >> 0x10);
  uVar5 = uVar3;
  if (uVar6 % 0xc == 0x0) {
    pass1_1030_387c(uVar1);
    pass1_1028_a61e((ushort)param_1,uVar12,uVar1,param_2,uVar5,uVar3,param_3,param_4);
  }
  pass1_1038_3fb0(param_2);
  if ((*(int *)((int)param_2 + 0x204) != 0x0) &&
     (BVar4 = pass1_1030_25b2(CONCAT13((char)(uVar3 >> 0x8),CONCAT12((char)uVar3,uVar5)),0x80), BVar4 != 0x0)) {
    return;
  }
  uVar9 = (undefined2)(uVar1 >> 0x10);
  uVar5 = (int)uVar1 + 0x180;
  uVar6 = (ulong)uVar5;
  iStack50 = 0x1;
  do {
    if (*(int *)(iStack50 * 0x2 + uVar5) != 0x0) {
      pass1_1008_612e(0x1,0x64,(uint)uVar6);
      if ((int)uVar6 <= *(int *)(iStack50 * 0x2 + uVar5)) {
        pass1_1028_a188((ushort)param_1,uVar12,*(int *)(iStack50 * 0x2 + (int)uVar1 + 0x174),iStack50,param_2,param_4);
      }
    }
    iStack50 = iStack50 + 0x1;
  } while (iStack50 < 0x6);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_a61e(ushort param_1,ushort param_2,ulong param_3,ulong param_4,uint param_5,int param_6,int param_7,
               ushort param_8)

{
  uint uVar1;
  ulong uVar2;
  int iVar3;
  uchar *puVar4;
  undefined2 uVar5;
  ushort *puVar6;
  uint uStack16;
  undefined4 uStack14;
  
  pass1_1030_38b8();
  if ((param_6 < 0x3fff) || ((param_6 < 0x4000 && (param_5 != 0xffff)))) {
    pass1_1030_38f2(param_3,0x3,param_8);
    uVar1 = param_5;
    iVar3 = param_6;
    pass1_1030_38f2(param_3,0x4,param_8);
    uStack14 = CONCAT22(param_6 + iVar3 + (uint)CARRY2(param_5,uVar1),param_5 + uVar1);
    uStack16 = *(uint *)((int)param_3 + 0x1a8);
    if (uStack16 == 0x0) {
      uStack16 = 0x5;
    }
    uVar2 = uStack14 / (long)(ulong)uStack16;
    uStack14._2_2_ = (uint)(uVar2 >> 0x10);
    puVar4 = (uchar *)(uStack14._2_2_ | (uint)uVar2);
    if ((puVar4 != (uchar *)0x0) &&
       (uVar5 = (undefined2)(param_4 >> 0x10), *(long *)((int)param_4 + 0x200) != 0x8000002)) {
      puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_8,puVar4,param_7);
      pass1_1010_043a((ulong)puVar6,*(long *)((int)param_4 + 0x4),0xc,param_8);
      pass1_1030_3534(param_3,uVar2);
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_a6ca(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_a706(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0xbb7);
  param_1->field_0x0 = 0xa856;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCPrelimAlloc_1050_50f6);
  return param_1;
}



ushort __stdcall16far pass1_1028_a73c(uint param_1,ushort param_2)

{
  undefined *puVar1;
  undefined *puVar2;
  uint uVar3;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  while( true ) {
    puVar1 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar1));
    uVar3 = param_1 | (uint)puVar1;
    if (uVar3 == 0x0) break;
    puVar2 = puVar1;
    pass1_1038_5464(CONCAT22(param_1,puVar1),(ushort)puVar1,(ushort)&PTR_LOOP_1050_1038,param_2);
    pass1_1038_56d6(CONCAT22(param_1,puVar1),0x0);
    pass1_1038_518c(CONCAT22(param_1,puVar1),(ushort)puVar2,(ushort)&PTR_LOOP_1050_1038);
    param_1 = uVar3;
  }
  return 0x1;
}



void __stdcall16far pass1_1028_a79c(ulong param_1,uint param_2,uchar *param_3)

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
    *puStack10 = 0xa856;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_a82a(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_a866(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x36af);
  param_1->field_0x0 = 0xa9ae;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCProdSched_1050_5104);
  return param_1;
}



ushort __stdcall16far pass1_1028_a89c(uint param_1,ushort param_2)

{
  undefined *puVar1;
  uint uVar2;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  while( true ) {
    uVar2 = param_1;
    puVar1 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar1));
    param_1 = uVar2 | (uint)puVar1;
    if (param_1 == 0x0) break;
    if (*(long *)(puVar1 + 0x200) != 0x8000002) {
      pass1_1038_3fca(CONCAT22(uVar2,puVar1),(uint)puVar1,param_2);
    }
  }
  return 0x1;
}



void __stdcall16far pass1_1028_a8f4(ulong param_1,astruct_335 *param_2,uchar *param_3)

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
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    param_2->field_0x4 = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = &param_2->field_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0xa9ae;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_a982(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_a9be(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x176f);
  param_1->field_0x0 = 0xab22;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCPower_1050_5110);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_a9f4(uint param_1,ushort param_2)

{
  code **ppcVar1;
  undefined *puVar2;
  BOOL16 BVar3;
  uint uVar4;
  uint extraout_DX;
  undefined4 *puStack24;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x700);
  while( true ) {
    puVar2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar2));
    puStack24 = (undefined4 *)CONCAT22(param_1,puVar2);
    uVar4 = param_1 | (uint)puVar2;
    if (uVar4 == 0x0) break;
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(puVar2 + 0xc),0xc);
    param_1 = uVar4;
    if (BVar3 != 0x0) {
      ppcVar1 = (code **)((int)*puStack24 + 0x34);
      (**ppcVar1)(0x1008,puVar2);
      param_1 = extraout_DX;
    }
  }
  return 0x1;
}



void __stdcall16far pass1_1028_aa68(ulong param_1,astruct_336 *param_2,uchar *param_3)

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
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    param_2->field_0x4 = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = &param_2->field_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0xab22;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_aaf6(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_ab32(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x2edf);
  param_1->field_0x0 = 0xaca6;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCRchSched_1050_5118);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_ab68(uint param_1,ushort param_2)

{
  undefined2 uVar1;
  code **ppcVar2;
  undefined *puVar3;
  BOOL16 BVar4;
  uint uVar5;
  uint extraout_DX;
  undefined4 *puStack24;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,local_14)),0x1,0x0,0x700);
LAB_1028_ab7e:
  uVar5 = param_1;
  puVar3 = local_14;
  pass1_1028_e4ec(CONCAT22(param_2,puVar3));
  puStack24 = (undefined4 *)CONCAT22(uVar5,puVar3);
  param_1 = uVar5 | (uint)puVar3;
  if (param_1 == 0x0) {
    return 0x1;
  }
  uVar1 = *(undefined2 *)(puVar3 + 0xc);
  BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x11);
  if (BVar4 == 0x0) goto code_r0x1028abad;
  goto LAB_1028_abc0;
code_r0x1028abad:
  BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x12);
  if (BVar4 != 0x0) {
LAB_1028_abc0:
    if (*(int *)(puVar3 + 0x12) == 0x5) {
      ppcVar2 = (code **)((int)*puStack24 + 0x30);
      (**ppcVar2)(0x1008);
      param_1 = extraout_DX;
    }
  }
  goto LAB_1028_ab7e;
}



void __stdcall16far pass1_1028_abec(ulong param_1,astruct_337 *param_2,uchar *param_3)

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
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    param_2->field_0x4 = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = &param_2->field_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0xaca6;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_ac7a(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_acb6(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x3e7f);
  param_1->field_0x0 = 0xae56;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCSetup_1050_5124);
  return param_1;
}



ushort __stdcall16far pass1_1028_acec(uint param_1,ushort param_2)

{
  undefined2 *puVar1;
  undefined2 *puVar2;
  uint uVar3;
  undefined2 local_14;
  undefined2 uStack18;
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,&local_14),0x1,0x0,0x400);
  while( true ) {
    uVar3 = param_1;
    puVar1 = &local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar1));
    param_1 = uVar3 | (uint)puVar1;
    if (param_1 == 0x0) break;
    puVar2 = puVar1;
    vsprintf_op_1030_840a((ulong)s_SCSetup__calcMe_clearing_colony_0_1050_512c,0x1030,param_2,param_1);
    if (*(long *)(puVar1 + 0x100) != 0x8000002) {
      pass1_1038_5464(CONCAT22(uVar3,puVar1),(ushort)puVar2,(ushort)&PTR_LOOP_1050_1038,param_2);
      pass1_1038_56d6(CONCAT22(uVar3,puVar1),0x1);
    }
  }
  local_14 = 0x389a;
  uStack18 = 0x1008;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,&local_14),0x1,0x0,0x800);
  while( true ) {
    puVar1 = &local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar1));
    uVar3 = param_1 | (uint)puVar1;
    if (uVar3 == 0x0) break;
    pass1_1030_2690(CONCAT22(param_1,puVar1));
    param_1 = uVar3;
  }
  return 0x1;
}



void __stdcall16far pass1_1028_ad9c(ulong param_1,astruct_338 *param_2,uchar *param_3)

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
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    param_2->field_0x4 = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = &param_2->field_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0xae56;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_ae2a(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
pass1_1028_ae66(astruct_100 *param_1,ulong param_2,ulong param_3,ulong param_4,ushort param_5,uchar param_6)

{
  astruct_689 *iVar1;
  undefined2 uVar1;
  
  struct_op_1028_d1dc(param_5,param_6,param_1,0x1387);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_689 *)param_1;
  iVar1->field_0x108 = param_4;
  iVar1->field_0x10c = param_3;
  iVar1->field_0x110 = param_2;
  iVar1->field_0x114 = 0x0;
  param_1->field_0x0 = 0xb0ce;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x8),s_SCStarve_1050_5156);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_aec0(ulong param_1,int param_2,ushort param_3,ushort param_4)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  pass1_1030_375a(*(ulong *)(param_2 + 0x1f6),0x0,(long)*(int *)((int)param_1 + 0x114),param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far pass1_1028_af08(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ulong uVar1;
  undefined *puVar2;
  undefined *puVar3;
  uchar *puVar4;
  uchar *puVar5;
  astruct_693 *iVar6;
  undefined2 uVar6;
  ushort *puVar7;
  astruct_67 *paVar8;
  astruct_67 *paVar9;
  int iStack12;
  int iStack10;
  
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  puVar4 = (uchar *)((ulong)puVar7 >> 0x10);
  puVar2 = PTR_LOOP_1050_13ae + -0x1;
  if (((int)PTR_LOOP_1050_13ae < 0x1) || (SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) {
LAB_1028_af27:
    iStack10 = 0x1;
  }
  else {
    puVar3 = PTR_LOOP_1050_13ae + -0x2;
    if (puVar3 == (undefined *)0x0 || (int)puVar2 < 0x1) {
      iStack12 = 0x1;
      iStack10 = 0x1;
      goto LAB_1028_af42;
    }
    puVar2 = PTR_LOOP_1050_13ae + -0x4;
    if (puVar2 != (undefined *)0x0) goto LAB_1028_af27;
    iStack10 = 0x2;
  }
  iStack12 = 0x3;
  puVar3 = puVar2;
LAB_1028_af42:
  pass1_1008_612e(iStack10,iStack12,(uint)puVar3);
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_693 *)param_1;
  iVar6->field_0x114 = puVar3;
  paVar8 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,puVar4,param_3);
  uVar1 = iVar6->field_0x108;
  paVar9 = paVar8;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)(uVar1 >> 0x10));
  puVar5 = (uchar *)((ulong)paVar9 >> 0x10);
  puVar4 = puVar5;
  post_win_msg_1008_a0e4
            (paVar8,0x0,(int)iVar6->field_0x114,*(ushort *)((int)paVar9 + 0x208),iVar6->field_0x108,0x2,0x1008,param_4);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,puVar4,param_3);
  pass1_1010_043a((ulong)puVar7,*(long *)((int)paVar9 + 0x4),0xd,param_4);
  return 0x1;
}



void __stdcall16far pass1_1028_afce(ulong param_1,astruct_339 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_340 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x116,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_340 *)param_1;
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
    param_2->field_0x110 = iVar5->field_0x110;
    param_2->field_0x114 = iVar5->field_0x114;
    *puStack10 = 0xb0ce;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_b0a2(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far
pass1_1028_b0de(astruct_100 *param_1,ulong param_2,ulong param_3,ushort param_4,uchar param_5)

{
  pass1_1028_6af2(param_1,param_2,param_3,param_4,param_5);
  param_1->field_0x0 = 0xb1f4;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



void __stdcall16far pass1_1028_b108(ulong param_1,astruct_341 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_342 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x110,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_342 *)param_1;
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
    *puStack10 = 0x6e50;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0xb1f4;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_b1c8(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1028_b204(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1030_1628(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0xc) = 0x0;
  *param_1 = 0xb33c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_b22c(ushort *param_1,ushort param_2,ulong param_3,ushort param_4)

{
  undefined2 uVar1;
  
  pass1_1030_165e(param_1,0x6000000,param_3,param_4);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0xc) = param_2;
  *param_1 = 0xb33c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



void __stdcall16far pass1_1028_b260(ushort *param_1)

{
  *param_1 = 0xb33c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  pass1_1030_16b2(param_1);
  return;
}



BOOL16 __stdcall16far write_to_file_1028_b286(int param_1,ushort param_2)

{
  undefined4 uVar1;
  BOOL16 in_AX;
  BOOL16 BVar2;
  
  pass1_1030_16d6(*(ulong *)(param_1 + 0x6),*(ulong *)(param_1 + 0xa),param_2);
  if (in_AX != 0x0) {
    uVar1 = *(undefined4 *)(param_1 + 0x6);
    *(undefined2 *)(param_1 + -0xa) = *(undefined2 *)((int)uVar1 + 0xc);
    uVar1 = *(undefined4 *)(param_1 + 0xa);
    BVar2 = write_to_file_1008_7e1c
                      ((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),param_1 - 0xa,param_2,(char *)0x2,0x1008);
    if (BVar2 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return BVar2;
    }
    in_AX = 0x1;
  }
  return in_AX;
}



BOOL16 __stdcall16far pass1_1028_b2c8(ulong param_1,ulong param_2,BOOL16 param_3,ushort param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort local_4;
  
  file_1030_1730(param_1,param_2);
  if (param_3 != 0x0) {
    uVar2 = (ushort)(param_2 >> 0x10);
    BVar1 = read_file_1008_7dee((ushort)param_2,uVar2,(ushort)&local_4,0x0,param_4,0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return BVar1;
    }
    uVar2 = switch_1008_72bc((ushort)param_2,uVar2,local_4);
    *(ushort *)((int)param_1 + 0xc) = uVar2;
    param_3 = 0x1;
  }
  return param_3;
}



astruct_18 * __stdcall16far pass1_1028_b316(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b260(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1028_b354(ushort *param_1)

{
  astruct_180 *iVar1;
  undefined2 uVar1;
  
  struct_1030_1628(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x18 = 0x0;
  iVar1->field_0x1a = 0x0;
  iVar1->field_0x1c = 0x0;
  *param_1 = 0xcf6a;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x14 = 0x0;
  return;
}



void __stdcall16far pass1_1028_b39e(ushort *param_1,int param_2,ulong param_3,ushort param_4)

{
  astruct_173 *iVar1;
  uint uVar1;
  
  pass1_1030_165e(param_1,0x7000000,param_3,param_4);
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_173 *)param_1;
  iVar1->field_0xc = param_2;
  iVar1->field_0xe = 0x42;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x18 = 0x0;
  iVar1->field_0x1a = 0x0;
  iVar1->field_0x1c = 0x0;
  *param_1 = 0xcf6a;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  pass1_1028_bf76((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,0x0);
  iVar1->field_0x14 = 0x0;
  if ((0x4e < iVar1->field_0xc) && (iVar1->field_0xc < 0x70)) {
    iVar1->field_0xe = 0x6b;
  }
  return;
}
