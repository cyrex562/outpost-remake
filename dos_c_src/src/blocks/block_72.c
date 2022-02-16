
void __stdcall16far pass1_1028_b418(ushort *param_1)

{
  int iVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *param_1 = 0xcf6a;
  *(undefined2 *)(iVar2 + 0x2) = (int)&USHORT_1050_1028;
  iVar1 = *(int *)(iVar2 + 0x12);
  if (((iVar1 == 0x4) || (iVar1 == 0x5)) ||
     ((iVar1 == 0x6 && ((iVar1 = *(int *)(iVar2 + 0x18), iVar1 == 0x4 || (iVar1 == 0x5)))))) {
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar2 + 0x14),0x1000);
  }
  pass1_1030_16b2(param_1);
  return;
}



void __stdcall16far pass1_1028_b46e(ulong param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  int iVar2;
  uint extraout_DX;
  uint uVar3;
  ulong uVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  
  uVar4 = pass1_1028_b4f2(param_1);
  iVar2 = (int)uVar4;
  uVar5 = 0x0;
  uVar6 = 0x0;
  pass1_1028_b58e(param_1);
  uVar3 = extraout_DX;
  pass1_1030_6d80(CONCAT22(extraout_DX,iVar2),CONCAT22(uVar6,uVar5));
  iVar1 = *(int *)(iVar2 + 0x32);
  if (iVar1 != 0x0) {
    pass1_1030_6c4c(CONCAT22(extraout_DX,iVar2),0x0);
    pass1_1038_387e(uVar4,0x0,iVar1,CONCAT22(extraout_DX,iVar2),uVar3);
  }
  fn_ptr_1030_7296(CONCAT22(extraout_DX,iVar2));
  *(undefined4 *)((int)param_1 + 0x1c) = *(undefined4 *)((int)param_2 + 0x200);
  return;
}



ulong __stdcall16far pass1_1028_b4f2(ulong param_1)

{
  undefined2 uVar1;
  undefined4 uVar2;
  
  uVar2 = pass1_1028_b58e(param_1);
  uVar1 = (undefined2)((ulong)uVar2 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)uVar2 + 0x30),*(undefined2 *)((int)uVar2 + 0x2e));
}



void __stdcall16far pass1_1028_b514(ulong param_1)

{
  int iVar1;
  ushort uVar2;
  astruct_604 *iVar3;
  uint uVar3;
  ushort unaff_SS;
  ulong uVar4;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar3 = (astruct_604 *)param_1;
  iVar1 = iVar3->field_0x12;
  if (((iVar1 == 0x4) || (iVar1 == 0x5)) ||
     ((iVar1 == 0x6 && ((iVar1 = iVar3->field_0x18, iVar1 == 0x4 || (iVar1 == 0x5)))))) {
    fn_ptr_1000_17ce(iVar3->field_0x14,0x1000);
  }
  iVar3->field_0x14 = (astruct_18 *)0x0;
  iVar3->field_0x12 = 0x7;
  uVar4 = pass1_1028_b58e(param_1 & 0xffff | (ulong)uVar3 << 0x10);
  uVar2 = (ushort)uVar4;
  fn_ptr_1030_7296(uVar4);
  pass1_1030_72d0(uVar4);
  pass1_1030_730a(uVar4,uVar2,0x1030,unaff_SS);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_b58e(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x8);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  return;
}



ushort __stdcall16far pass1_1028_b5a8(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) != 0x5) {
    return 0x0;
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x14);
  return *(ushort *)((int)uVar1 + 0x94);
}



ushort __stdcall16far pass1_1028_b5ca(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) != 0x5) {
    return 0x0;
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x14);
  return *(ushort *)((int)uVar1 + 0x9c);
}



BOOL16 __stdcall16far write_to_file_1028_b5ec(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  int iVar3;
  undefined2 uVar4;
  ushort uVar5;
  ushort uVar6;
  undefined2 local_e [0x3];
  undefined2 local_8 [0x2];
  int iStack4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  local_e[0] = *(undefined2 *)(iVar3 + 0xc);
  uVar5 = (ushort)param_2;
  uVar6 = (ushort)(param_2 >> 0x10);
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_e,param_3,(char *)0x2,0x1008);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  pass1_1030_16d6(param_1,param_2,param_3);
  if (BVar2 == 0x0) {
    return 0x0;
  }
  local_8[0] = *(undefined2 *)(iVar3 + 0xc);
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  local_8[0] = *(undefined2 *)(iVar3 + 0xe);
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  local_8[0] = *(undefined2 *)(iVar3 + 0x10);
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  local_8[0] = *(undefined2 *)(iVar3 + 0x12);
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  local_8[0] = *(undefined2 *)(iVar3 + 0x18);
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  local_8[0] = *(undefined2 *)(iVar3 + 0x1a);
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  iStack4 = *(int *)(iVar3 + 0x12);
  if (iStack4 == 0x6) {
    iStack4 = *(int *)(iVar3 + 0x18);
  }
  if (iStack4 < 0x1) {
    return 0x1;
  }
  if (SBORROW2(iStack4,0x1)) {
    return 0x1;
  }
  if (iStack4 == 0x3 || iStack4 + -0x2 < 0x1) {
    local_8[0] = *(undefined2 *)(iVar3 + 0x14);
  }
  else {
    if (iStack4 == 0x4) {
      if (*(long *)(iVar3 + 0x14) == 0x0) {
        local_8[0] = 0x0;
        BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
        goto joined_r0x1028b766;
      }
      uVar1 = *(undefined4 *)(iVar3 + 0x14);
      local_8[0] = *(undefined2 *)((int)uVar1 + 0x94);
    }
    else {
      if (iStack4 != 0x5) {
        return 0x1;
      }
      uVar1 = *(undefined4 *)(iVar3 + 0x14);
      local_8[0] = *(undefined2 *)((int)uVar1 + 0xa4);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
      }
      uVar1 = *(undefined4 *)(iVar3 + 0x14);
      local_8[0] = *(undefined2 *)((int)uVar1 + 0xa6);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
      }
      uVar1 = *(undefined4 *)(iVar3 + 0x14);
      local_8[0] = *(undefined2 *)((int)uVar1 + 0xa8);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
      }
      uVar1 = *(undefined4 *)(iVar3 + 0x14);
      local_8[0] = *(undefined2 *)((int)uVar1 + 0xaa);
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
      }
      uVar1 = *(undefined4 *)(iVar3 + 0x14);
      local_8[0] = *(undefined2 *)((int)uVar1 + 0xac);
    }
  }
  BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_8,param_3,(char *)0x2,0x1008);
joined_r0x1028b766:
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return 0x0;
  }
  return 0x1;
}



// WARNING: Unable to use type for symbol puVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1028_b81a(ulong param_1,ulong param_2,int param_3,ushort param_4,uchar *param_5)

{
  BOOL16 BVar1;
  int iVar2;
  ulong *puVar4;
  ushort uVar5;
  uint uVar6;
  ulong uVar7;
  ushort uVar8;
  ushort local_2a [0x2];
  undefined local_26 [0x16];
  ulong *puStack16;
  uchar *puStack14;
  int iStack10;
  int local_8;
  int local_6;
  int local_4;
  ulong *puVar3;
  
  puVar3 = (ulong *)param_1;
  uVar6 = (uint)(param_1 >> 0x10);
  file_1030_1730(param_1,param_2);
  if (param_3 == 0x0) {
    return;
  }
  uVar5 = (ushort)param_2;
  uVar8 = (ushort)(param_2 >> 0x10);
  BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)&local_4,0x0,param_4,0x2,0x1008);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)&local_6,0x0,param_4,0x2,0x1008);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)&local_8,0x0,param_4,0x2,0x1008);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  iVar2 = switch_1008_73ea(uVar5,uVar8,local_4);
  *(int *)(puVar3 + 0x3) = iVar2;
  iVar2 = switch_1008_73ea(uVar5,uVar8,local_6);
  *(int *)((int)puVar3 + 0xe) = iVar2;
  iVar2 = switch_1008_73ea(uVar5,uVar8,local_8);
  *(int *)(puVar3 + 0x4) = iVar2;
  BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)&local_4,0x0,param_4,0x2,0x1008);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)&local_6,0x0,param_4,0x2,0x1008);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(uVar5,uVar8,(int)puVar3 + 0x1a,0x0,uVar6,0x2,0x1008);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  *(int *)((int)puVar3 + 0x12) = local_4;
  *(int *)(puVar3 + 0x6) = local_6;
  iStack10 = *(int *)((int)puVar3 + 0x12);
  if (iStack10 == 0x6) {
    iStack10 = *(int *)(puVar3 + 0x6);
  }
  switch(iStack10) {
  case 0x1:
  case 0x2:
  case 0x3:
    puVar4 = puVar3 + 0x5;
LAB_1028_b968:
    BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)puVar4,0x0,uVar6,0x2,0x1008);
    break;
  case 0x4:
    uVar7 = pass1_1028_e0bc(_PTR_LOOP_1050_65e2,*(int *)(puVar3 + 0x3),puVar3,param_5,param_4);
    puStack14 = (uchar *)(uint)(uVar7 >> 0x10);
    *(int *)(puVar3 + 0x5) = (int)uVar7;
    *(uint *)((int)puVar3 + 0x16) = (uint)puStack14;
    if (((uint)puStack14 | *(uint *)(puVar3 + 0x5)) != 0x0) {
      puVar4 = (ulong *)(*(int *)(puVar3 + 0x5) + 0x94);
      uVar6 = (uint)puStack14;
      puStack16 = puVar4;
      goto LAB_1028_b968;
    }
    BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)local_26,0x0,param_4,0x2,0x1008);
    break;
  case 0x5:
    puVar4 = puVar3;
    pass1_1028_e100(_PTR_LOOP_1050_65e2,*(ushort *)(puVar3 + 0x3),param_5);
    *(ulong **)(puVar3 + 0x5) = puVar4;
    *(uchar **)((int)puVar3 + 0x16) = param_5;
    puStack16 = (ulong *)(*(int *)(puVar3 + 0x5) + 0xa4);
    puStack14 = param_5;
    BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)puStack16,0x0,(ushort)param_5,0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
    BVar1 = read_file_1008_7dee(uVar5,uVar8,(ushort)local_2a,0x0,param_4,0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
    uVar7 = puVar3[0x5];
    BVar1 = read_file_1008_7dee(uVar5,uVar8,(int)uVar7 + 0xa8,0x0,(ushort)(uVar7 >> 0x10),0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
    uVar7 = puVar3[0x5];
    BVar1 = read_file_1008_7dee(uVar5,uVar8,(int)uVar7 + 0xaa,0x0,(ushort)(uVar7 >> 0x10),0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
    uVar7 = puVar3[0x5];
    BVar1 = read_file_1008_7dee(uVar5,uVar8,(int)uVar7 + 0xac,0x0,(ushort)(uVar7 >> 0x10),0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
    uVar5 = switch_1008_72bc(uVar5,uVar8,local_2a[0]);
    uVar7 = puVar3[0x5];
    *(ushort *)((int)uVar7 + 0xa6) = uVar5;
    return;
  default:
    goto switchD_1028_ba97_caseD_6;
  case 0x9:
    puVar4 = puVar3;
    pass1_1028_e100(_PTR_LOOP_1050_65e2,*(ushort *)(puVar3 + 0x3),param_5);
    *(ulong **)(puVar3 + 0x5) = puVar4;
    *(uchar **)((int)puVar3 + 0x16) = param_5;
    goto switchD_1028_ba97_caseD_6;
  }
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
switchD_1028_ba97_caseD_6:
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_bab6(ulong param_1,int param_2,ushort param_3)

{
  ulong uVar1;
  
  uVar1 = pass1_1028_bad4(param_1,param_2,param_3);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)(uVar1 >> 0x10));
  return;
}



ulong __stdcall16far pass1_1028_bad4(ulong param_1,int param_2,ushort param_3)

{
  pass1_1028_baf6(param_1);
  return CONCAT22(*(undefined2 *)(param_2 + 0xa),*(undefined2 *)(param_2 + 0x8));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_baf6(ulong param_1)

{
  ulong uVar1;
  
  uVar1 = pass1_1028_bb24(param_1);
  if (uVar1 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)(uVar1 >> 0x10));
  return;
}



ulong __stdcall16far pass1_1028_bb24(ulong param_1)

{
  undefined2 uVar1;
  uint uVar2;
  undefined4 uVar3;
  
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x8) == 0x0) {
    return 0x0;
  }
  uVar3 = pass1_1028_b58e(param_1 & 0xffff | (ulong)uVar2 << 0x10);
  uVar1 = (undefined2)((ulong)uVar3 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)uVar3 + 0xa),*(undefined2 *)((int)uVar3 + 0x8));
}



void __stdcall16far pass1_1028_bb56(ulong param_1,ulong param_2)

{
  pass1_1030_177a(param_1,param_2);
  return;
}



ulong __stdcall16far pass1_1028_bb6a(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((*(int *)(iVar1 + 0x12) != 0x5) && (*(int *)(iVar1 + 0x12) != 0x6)) {
    return 0x0;
  }
  return CONCAT22(*(undefined2 *)(iVar1 + 0x16),*(int *)(iVar1 + 0x14) + 0xa4);
}



void __stdcall16far pass1_1028_bb96(ulong param_1,ulong *param_2,ushort param_3)

{
  ulong *puVar1;
  ulong *puVar2;
  undefined4 uVar3;
  int iVar6;
  astruct_296 *iVar5;
  astruct_297 *iVar4;
  ulong *puVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_296 *)param_1;
  if ((iVar5->field_0x12 == 0x5) || (iVar5->field_0x12 == 0x6)) {
    uVar3 = iVar5->field_0x14;
    uVar9 = (undefined2)((ulong)uVar3 >> 0x10);
    puVar7 = (ulong *)((int)uVar3 + 0xa4);
    for (iVar6 = 0x2; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = param_2;
      param_2 = param_2 + 0x1;
      *puVar2 = *puVar1;
    }
    *(undefined2 *)puVar7 = *(undefined2 *)param_2;
    pass1_1028_c724(param_1);
    uVar3 = iVar5->field_0x14;
    uVar8 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar4 = (astruct_297 *)uVar3;
    if (iVar4->field_0xaa == 0x0) {
      iVar4->field_0xaa = 0x1;
    }
  }
  return;
}



void __stdcall16far pass1_1028_bbf0(ushort param_1,ushort param_2,ulong *param_3)

{
  *param_3 = 0x0;
  return;
}



void __stdcall16far pass1_1028_bc02(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x40);
  (**ppcVar1)();
  return;
}



ushort __stdcall16far pass1_1028_bc1c(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x12) == 0x4) {
    return *(ushort *)(iVar1 + 0xe);
  }
  if (*(int *)(iVar1 + 0x12) == 0x7) {
    return *(ushort *)(iVar1 + 0x10);
  }
  return *(ushort *)(iVar1 + 0xc);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_bc4a(ulong param_1,ulong *param_2,uchar *param_3,ushort param_4)

{
  ushort uVar1;
  astruct_18 *paVar2;
  
  paVar2 = (astruct_18 *)pass1_1028_e0bc(_PTR_LOOP_1050_65e2,*(int *)((int)param_1 + 0xc),param_2,param_3,param_4);
  uVar1 = *(ushort *)((int)paVar2 + 0x96);
  fn_ptr_1000_17ce(paVar2,0x1000);
  return uVar1;
}



void __stdcall16far pass1_1028_bc7e(ulong *param_1,ushort param_2)

{
  pass1_1028_bdac(param_1,0x4,param_2);
  return;
}



ushort __stdcall16far
pass1_1028_bc90(ulong *param_1,ushort *param_2,ulong param_3,ulong param_4,int param_5,ushort param_6,ushort param_7)

{
  code **ppcVar1;
  ulong uVar2;
  int iVar3;
  BOOL16 BVar4;
  undefined4 uVar5;
  ushort uVar6;
  ushort uVar7;
  
  uVar6 = (ushort)param_1;
  uVar7 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_6,uVar6,uVar7,param_2,param_4);
  if ((param_5 == 0x5) || (param_5 == 0x6)) {
    uVar2 = *param_1;
    ppcVar1 = (code **)((int)uVar2 + 0x60);
    iVar3 = (**ppcVar1)();
    if (iVar3 != 0x0) {
      ppcVar1 = (code **)((int)uVar2 + 0x5c);
      uVar5 = (**ppcVar1)();
      if ((uint)uVar5 != 0x0) {
        pass1_1028_c23e(uVar6,uVar7,param_2,param_3,param_4,(uint)uVar5,(uint)((ulong)uVar5 >> 0x10),param_7);
        if ((int)uVar5 != 0x0) {
          BVar4 = pass1_1028_c314(param_7,(int)uVar5,(ushort)((ulong)uVar5 >> 0x10),uVar6,uVar7,param_2,(ushort)param_3,
                                  (ushort)(param_3 >> 0x10),param_4);
          if (BVar4 != 0x0) {
            return 0x1;
          }
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = (undefined *)0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_bd38(ulong param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  ulong uVar5;
  undefined2 extraout_DX;
  int iStack20;
  
  uVar5 = *(ulong *)((int)_PTR_LOOP_1050_65e2 + 0x52);
  pass1_1030_4bbe(param_3,param_2,uVar5,*(int *)((int)param_1 + 0xc));
  iVar3 = (int)uVar5;
  iVar4 = iVar3;
  pass1_1028_b58e(param_1);
  uVar5 = *(ulong *)(iVar4 + 0x2e);
  iStack20 = 0x11;
  do {
    uVar1 = *(uint *)(iStack20 * 0x4 + iVar3);
    uVar2 = *(uint *)(iStack20 * 0x4 + iVar3 + 0x2);
    if ((uVar2 | uVar1) != 0x0) {
      pass1_1038_5770(uVar5,CONCAT22(uVar2,uVar1),iStack20);
    }
    iStack20 = iStack20 + 0x1;
  } while (iStack20 < 0x25);
  return;
}



void __stdcall16far pass1_1028_bdac(ulong *param_1,int param_2,ushort param_3)

{
  int iVar1;
  code **ppcVar2;
  astruct_599 *iVar3;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_599 *)param_1;
  if (iVar3->field_0x12 != param_2) {
    if (iVar3->field_0x12 == 0x6) {
      if (iVar3->field_0x18 == param_2) {
        iVar3->field_0x12 = iVar3->field_0x18;
        iVar3->field_0x18 = 0x0;
        return;
      }
    }
    else {
      if (param_2 != 0x6) {
        iVar1 = iVar3->field_0x12;
        if ((iVar1 == 0x4) || (iVar1 == 0x5)) {
          param_3 = 0x1000;
          fn_ptr_1000_17ce(iVar3->field_0x14,0x1000);
        }
        iVar3->field_0x12 = param_2;
        ppcVar2 = (code **)((int)*param_1 + 0x3c);
        (**ppcVar2)(param_3,param_1);
        return;
      }
      iVar3->field_0x18 = iVar3->field_0x12;
      iVar3->field_0x12 = 0x6;
    }
  }
  return;
}



void __stdcall16far pass1_1028_be2a(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  undefined2 uVar2;
  ulong uVar3;
  int iVar4;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2((ulong)param_1);
  if (*(long *)((int)uVar3 + 0x200) != 0x8000002) {
    if (*(long *)((int)param_1 + 0x1c) == 0x8000002) {
      iVar4 = 0x6;
      goto code_r0x1028be96;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x64);
    iVar4 = (**ppcVar1)(param_4,param_1);
    if (iVar4 == 0x0) {
      return;
    }
    pass1_1028_cb04((ulong)param_1,param_2,param_3,param_5);
    if (iVar4 == 0x0) {
      iVar4 = 0x6;
      goto code_r0x1028be96;
    }
    pass1_1028_c952((ulong)param_1,param_2,param_3,param_5);
  }
  iVar4 = 0x5;
code_r0x1028be96:
  pass1_1028_bdac(param_1,iVar4,param_4);
  return;
}



void __stdcall16far pass1_1028_be9e(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(int *)(iVar4 + 0x12) == 0x4) {
    uVar6 = pass1_1028_b4f2((ulong)param_1);
    iVar3 = (int)uVar6;
    if (*(long *)(iVar3 + 0x200) == 0x8000002) {
      uVar2 = *(undefined4 *)(iVar4 + 0x14);
      piVar1 = (int *)((int)uVar2 + 0x94);
      *piVar1 = *piVar1 + -0x1;
    }
    else {
      pass1_1028_cb04((ulong)param_1,param_2,param_3,param_5);
      if (iVar3 == 0x0) {
        return;
      }
      uVar2 = *(undefined4 *)(iVar4 + 0x14);
      piVar1 = (int *)((int)uVar2 + 0x94);
      *piVar1 = *piVar1 + -0x1;
      pass1_1028_c952((ulong)param_1,param_2,param_3,param_5);
    }
    uVar2 = *(undefined4 *)(iVar4 + 0x14);
    if (*(int *)((int)uVar2 + 0x94) < 0x1) {
      pass1_1028_bdac(param_1,0x5,param_4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_bf22(ulong param_1,uchar *param_2,ushort param_3)

{
  int iVar1;
  int iVar2;
  undefined2 uVar3;
  ulong uVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  iVar1 = *(int *)(iVar2 + 0x12);
  if (iVar1 == 0x4) {
    uVar4 = pass1_1028_e0bc(_PTR_LOOP_1050_65e2,*(int *)(iVar2 + 0xc),(ulong *)0x0,param_2,param_3);
  }
  else {
    iVar1 = iVar1 + -0x5;
    if (iVar1 != 0x0) {
      if (iVar1 != 0x1) {
        *(undefined4 *)(iVar2 + 0x14) = 0x0;
      }
      return;
    }
    pass1_1028_e100(_PTR_LOOP_1050_65e2,*(ushort *)(iVar2 + 0xc),param_2);
    uVar4 = CONCAT22(param_2,iVar1);
  }
  *(undefined2 *)(iVar2 + 0x14) = (int)uVar4;
  *(undefined2 *)(iVar2 + 0x16) = (int)(uVar4 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_bf76(ulong param_1,uint param_2)

{
  BOOL16 BVar1;
  astruct_174 *iVar2;
  undefined2 uVar2;
  
  pass1_1008_612e(0x1,0x3,param_2);
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_174 *)param_1;
  BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar2->field_0xc,0x28);
  if (BVar1 == 0x0) {
    if (param_2 == 0x1) {
      iVar2->field_0x10 = 0x48;
      return;
    }
    if (param_2 != 0x2) {
      iVar2->field_0x10 = 0x4a;
      return;
    }
    iVar2->field_0x10 = 0x49;
    return;
  }
  if (param_2 == 0x1) {
    iVar2->field_0x10 = 0x70;
    return;
  }
  if (param_2 != 0x2) {
    iVar2->field_0x10 = 0x72;
    return;
  }
  iVar2->field_0x10 = 0x71;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_c00a(ulong param_1,long param_2,int param_3,ushort param_4)

{
  code **ppcVar1;
  uint uVar2;
  ushort uVar3;
  undefined2 extraout_DX;
  uchar *puVar4;
  undefined2 extraout_DX_00;
  uint extraout_DX_01;
  uint uVar5;
  undefined2 uVar6;
  ulong *puVar7;
  ulong uVar8;
  ulong uVar9;
  ulong uStack26;
  ulong uStack22;
  undefined4 *puStack18;
  
  pass1_1028_b58e(param_1);
  uVar8 = *(ulong *)(param_3 + 0x2e);
  puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
  puVar4 = (uchar *)((ulong)puVar7 >> 0x10);
  uVar2 = (uint)puVar7;
  uVar6 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4d6e(uVar8,puVar7,uVar2,puVar4);
  puStack18 = (undefined4 *)CONCAT22(puVar4,uVar2);
  ppcVar1 = (code **)((int)*puStack18 + 0x10);
  uVar5 = uVar2;
  (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar2,puVar4);
  uStack22 = CONCAT22(extraout_DX_00,uVar5);
  uStack26 = 0x0;
  do {
    if (uStack22 <= uStack26) {
LAB_1028_c0d6:
      if (puStack18 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack18;
        (**ppcVar1)(uVar6,uVar2,(char)puVar4,0x1);
      }
      return;
    }
    ppcVar1 = (code **)((int)*puStack18 + 0x4);
    uVar8 = uStack22;
    (**ppcVar1)((char)uVar6,uVar2,puVar4,(char)uStack26,(int)(uStack26 >> 0x10));
    uVar3 = (ushort)uVar8;
    uVar5 = extraout_DX_01;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_01);
    uVar6 = 0x1030;
    uVar8 = struct_op_1030_73a8(CONCAT22(uVar5,uVar3));
    uVar9 = pass1_1028_6302(uVar8,param_4);
    uVar5 = (uint)(uVar9 >> 0x10);
    if ((param_2._2_2_ <= uVar5) && ((param_2._2_2_ < uVar5 || ((uint)param_2 <= (uint)uVar9)))) {
      pass1_1028_6356(uVar8,0x0,(uint)param_2,param_2._2_2_,param_4);
      goto LAB_1028_c0d6;
    }
    pass1_1028_6356(uVar8,0x0,(uint)uVar9,uVar5,param_4);
    param_2 = param_2 - uVar9;
    uStack26 = uStack26 + 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_c0f0(ulong param_1,long param_2,int param_3,ushort param_4,ushort param_5,ushort param_6)

{
  code **ppcVar1;
  uint uVar2;
  ushort uVar3;
  undefined2 extraout_DX;
  uchar *puVar4;
  uchar *extraout_DX_00;
  uint extraout_DX_01;
  uint uVar5;
  uchar *puVar6;
  uchar *extraout_DX_02;
  undefined2 uVar7;
  ulong *puVar8;
  ulong uVar9;
  ulong uStack28;
  ulong uStack24;
  undefined4 *puStack20;
  ulong uStack6;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_3);
  uVar9 = *(ulong *)(param_3 + 0x2e);
  pass1_1028_cb04(param_1,param_4,param_5,param_6);
  uVar7 = (undefined2)(uVar9 >> 0x10);
  if ((*(int *)((int)uVar9 + 0x204) == 0x0) && (*(int *)((int)uVar9 + 0x206) == 0x0)) {
    puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
    puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
    uVar2 = (uint)puVar8;
    uVar7 = SUB42(&PTR_LOOP_1050_1038,0x0);
    pass1_1038_4d6e(uVar9,puVar8,uVar2,puVar4);
    puStack20 = (undefined4 *)CONCAT22(puVar4,uVar2);
    ppcVar1 = (code **)((int)*puStack20 + 0x10);
    uVar5 = uVar2;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar2,puVar4);
    uStack24 = CONCAT22(extraout_DX_00,uVar5);
    puVar6 = extraout_DX_00;
    for (uStack28 = 0x0; uStack28 < uStack24; uStack28 = uStack28 + 0x1) {
      ppcVar1 = (code **)((int)*puStack20 + 0x4);
      uVar9 = uStack24;
      (**ppcVar1)((char)uVar7,uVar2,puVar4,(char)uStack28,(int)(uStack28 >> 0x10));
      uVar3 = (ushort)uVar9;
      uVar5 = extraout_DX_01;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_01);
      uVar7 = 0x1030;
      uVar9 = struct_op_1030_73a8(CONCAT22(uVar5,uVar3));
      uVar9 = pass1_1028_6302(uVar9,param_6);
      puVar6 = (uchar *)(uVar9 >> 0x10);
      uVar5 = (uint)uVar9;
      if ((param_2._2_2_ <= puVar6) && ((param_2._2_2_ < puVar6 || ((uint)param_2 <= uVar5)))) {
        param_2 = 0x0;
        break;
      }
      param_2 = CONCAT22(param_2._2_2_ + (-(uint)((uint)param_2 < uVar5) - (int)puVar6),(uint)param_2 - uVar5);
    }
    if (puStack20 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack20;
      (**ppcVar1)(uVar7,uVar2,(char)puVar4,0x1);
      puVar6 = extraout_DX_02;
    }
    if (param_2 != 0x0) {
      pass1_1030_7d7c(uStack6,(uint)param_2,CONCAT22(0x1d,(int)((ulong)param_2 >> 0x10)),(uint)puStack20,puVar6,param_4,
                      param_5,param_6);
    }
  }
  return;
}



void __stdcall16far
pass1_1028_c1f8(ushort param_1,int param_2,ushort param_3,ulong param_4,ushort *param_5,ushort *param_6)

{
  undefined4 *puVar1;
  undefined4 local_c;
  undefined2 uStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_baf6(param_4);
  iStack6 = param_2;
  uStack4 = param_3;
  puVar1 = (undefined4 *)pass1_1030_5b5c(param_2,param_3);
  local_c = *puVar1;
  uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
  pass1_1008_3e94((ushort *)CONCAT22(param_1,&local_c),param_5,param_6);
  return;
}
