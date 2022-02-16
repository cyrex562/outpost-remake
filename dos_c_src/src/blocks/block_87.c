
astruct_18 * __stdcall16far pass1_1030_bbe6(astruct_18 *param_1,byte param_2)

{
  pass1_1030_b96c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1030_bc24(ushort param_1,int param_2,ushort param_3,ushort param_4,ulong param_5)

{
  pass1_1028_b22c((ushort *)CONCAT22(param_3,param_2),param_4,param_5,param_1);
  *(ushort *)CONCAT22(param_3,param_2) = 0xbc96;
  *(undefined2 *)(param_2 + 0x2) = 0x1030;
  return (ushort *)CONCAT22(param_3,param_2);
}



void __stdcall16far pass1_1030_bc4e(ushort *param_1)

{
  *param_1 = 0xbc96;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  pass1_1028_b260(param_1);
  return;
}



astruct_18 * __stdcall16far pass1_1030_bc70(astruct_18 *param_1,byte param_2)

{
  pass1_1030_bc4e(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1030_bcae(ushort param_1,ushort param_2)

{
  return CONCAT22(param_2,param_1);
}



void __stdcall16far
pass1_1030_bcbc(ushort param_1,ushort param_2,ulong param_3,ulong param_4,ushort param_5,ulong param_6)

{
  pass1_1030_bcde(param_1,param_2,(ushort)param_3,CONCAT22((undefined2)param_4,param_3._2_2_),
                  (ushort *)CONCAT22(param_5,param_4._2_2_),*(long *)((int)param_6 + 0x4));
  return;
}



void __stdcall16far
pass1_1030_bcde(ushort param_1,ushort param_2,ushort param_3,ulong param_4,ushort *param_5,long param_6)

{
  int iVar1;
  undefined2 uVar2;
  int local_14;
  int local_12;
  int local_10;
  int local_e;
  undefined4 local_c;
  undefined2 uStack8;
  long lStack6;
  
  uVar2 = (undefined2)(param_4 >> 0x10);
  iVar1 = (int)param_4;
  lStack6 = *(long *)(iVar1 + 0x8);
  if (lStack6 != param_6) {
    return;
  }
  local_c = *(undefined4 *)(iVar1 + 0xc);
  uStack8 = *(undefined2 *)(iVar1 + 0x10);
  pass1_1008_3e94(param_5,(ushort *)CONCAT22(param_1,&local_10),(ushort *)CONCAT22(param_1,&local_e));
  pass1_1008_3e94((ushort *)CONCAT22(param_1,&local_c),(ushort *)CONCAT22(param_1,&local_14),
                  (ushort *)CONCAT22(param_1,&local_12));
  pass1_1000_49b2(local_e - local_12);
  pass1_1000_49b2(local_10 - local_14);
  return;
}



void __stdcall16far pass1_1030_bd74(ushort param_1,ushort param_2,ulong param_3,ulong param_4,ushort param_5)

{
  astruct_670 *iVar1;
  int iVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  int local_1e;
  int local_1c;
  int local_1a;
  int local_18;
  undefined4 local_16;
  undefined2 uStack18;
  undefined4 local_10;
  undefined2 uStack12;
  long lStack10;
  long lStack6;
  
  uVar3 = (undefined2)(param_4 >> 0x10);
  iVar1 = (astruct_670 *)param_4;
  lStack6 = iVar1->field_0x8;
  uVar4 = (undefined2)(param_3 >> 0x10);
  iVar2 = (int)param_3;
  lStack10 = *(long *)(iVar2 + 0x8);
  if (lStack10 != lStack6) {
    return;
  }
  local_10 = iVar1->field_0xc;
  uStack12 = iVar1->field_0x10;
  local_16 = *(undefined4 *)(iVar2 + 0xc);
  uStack18 = *(undefined2 *)(iVar2 + 0x10);
  pass1_1008_3e94((ushort *)CONCAT22(param_5,&local_10),(ushort *)CONCAT22(param_5,&local_1a),
                  (ushort *)CONCAT22(param_5,&local_18));
  pass1_1008_3e94((ushort *)CONCAT22(param_5,&local_16),(ushort *)CONCAT22(param_5,&local_1e),
                  (ushort *)CONCAT22(param_5,&local_1c));
  pass1_1000_49b2(local_18 - local_1c);
  pass1_1000_49b2(local_1a - local_1e);
  return;
}



ushort * __stdcall16far struct_1030_be34(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0xc006;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



ushort * __stdcall16far pass1_1030_be56(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xc006;
  *(undefined2 *)(param_1 + 0x2) = 0x1030;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1030_be80(ulong param_1,uchar *param_2,ushort param_3)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  BOOL16 BVar4;
  ulong uVar5;
  uint extraout_DX;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  int iVar9;
  
  pass1_1028_bf22(param_1,param_2,param_3);
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(int *)(iVar7 + 0x12) == 0x5) {
    uVar2 = *(undefined4 *)(iVar7 + 0x14);
    *(undefined2 *)((int)uVar2 + 0xa4) = 0x1e;
    uVar2 = *(undefined4 *)(iVar7 + 0x14);
    *(undefined2 *)((int)uVar2 + 0xac) = 0x1;
    iVar9 = *(int *)(iVar7 + 0xc);
    iVar3 = iVar9 + -0x1b;
    if (iVar3 == 0x0) {
      uVar2 = *(undefined4 *)(iVar7 + 0x14);
      *(undefined2 *)((int)uVar2 + 0xaa) = 0xa;
    }
    else {
      iVar3 = iVar9 + -0x1c;
      if (iVar3 == 0x0) {
        uVar2 = *(undefined4 *)(iVar7 + 0x14);
        *(undefined2 *)((int)uVar2 + 0xaa) = 0xb;
      }
      else {
        iVar3 = iVar9 + -0x1d;
        if (iVar3 == 0x0) {
          uVar2 = *(undefined4 *)(iVar7 + 0x14);
          *(undefined2 *)((int)uVar2 + 0xaa) = 0xc;
        }
      }
    }
    pass1_1028_b58e(param_1);
    uVar5 = *(ulong *)(iVar3 + 0x2e);
    iVar9 = 0xc;
    uVar6 = extraout_DX;
    pass1_1038_3fb0(uVar5);
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (ulong)uVar6 << 0x10,iVar9);
    if (BVar4 != 0x0) {
      uVar2 = *(undefined4 *)(iVar7 + 0x14);
      piVar1 = (int *)((int)uVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (ulong)uVar6 << 0x10,0xe);
    if (BVar4 != 0x0) {
      uVar2 = *(undefined4 *)(iVar7 + 0x14);
      piVar1 = (int *)((int)uVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (ulong)uVar6 << 0x10,0x76);
    if (BVar4 != 0x0) {
      uVar2 = *(undefined4 *)(iVar7 + 0x14);
      piVar1 = (int *)((int)uVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
  }
  return;
}



void __stdcall16far pass1_1030_bf6e(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uint *puVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  ulong uVar7;
  undefined4 uVar8;
  ushort uVar9;
  
  uVar9 = 0x1e;
  uVar7 = pass1_1028_b58e(param_1);
  uVar8 = pass1_1030_7c28(uVar7,uVar9,(uint)uVar7,(uint)(uVar7 >> 0x10),param_4);
  uVar4 = 0x3e8 - (int)uVar8;
  uVar2 = *(undefined4 *)((int)param_1 + 0x14);
  uVar6 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar5 = (int)uVar2;
  puVar1 = (uint *)(iVar5 + 0xaa);
  uVar3 = -(uint)(uVar4 < *puVar1);
  pass1_1030_7ddc(uVar7,(ulong)((uVar4 - *puVar1 & uVar3) + *(int *)(iVar5 + 0xaa)),0x1e,uVar3,
                  (uchar *)((ulong)uVar8 >> 0x10),param_2,param_3,param_4);
  return;
}



ushort __stdcall16far pass1_1030_bfb8(ulong param_1,ushort param_2)

{
  ulong uVar1;
  undefined4 uVar2;
  ushort uVar3;
  
  uVar3 = 0x1e;
  uVar1 = pass1_1028_b58e(param_1);
  uVar2 = pass1_1030_7c28(uVar1,uVar3,(uint)uVar1,(uint)(uVar1 >> 0x10),param_2);
  return 0x3e8 - (int)uVar2;
}



astruct_18 * __stdcall16far pass1_1030_bfe0(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1030_c06e(ushort *param_1)

{
  astruct_188 *iVar1;
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_188 *)param_1;
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x24 = 0x0;
  *param_1 = 0xc68e;
  iVar1->field_0x2 = 0x1030;
  return;
}



void __stdcall16far pass1_1030_c09c(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined4 *)(param_1 + 0x20) = 0x0;
  *(undefined2 *)(param_1 + 0x24) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xc68e;
  *(undefined2 *)(param_1 + 0x2) = 0x1030;
  return;
}



ushort __stdcall16far pass1_1030_c0d2(ulong param_1)

{
  if (0x0 < *(int *)((int)param_1 + 0x24)) {
    return 0x1;
  }
  return 0x0;
}



ushort __stdcall16far pass1_1030_c0ec(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  if ((*(int *)((int)param_1 + 0xc) != 0xb) && (*(int *)((int)param_1 + 0x24) < 0x1)) {
    return 0x0;
  }
  return 0x1;
}



void __stdcall16far pass1_1030_c10e(ulong param_1)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (0x0 < *(int *)(iVar2 + 0x24)) {
    piVar1 = (int *)(iVar2 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  *(undefined2 *)(iVar2 + 0xc) = 0x37;
  return;
}



void __stdcall16far pass1_1030_c12e(ulong param_1,int param_2,int param_3,ushort param_4,ushort param_5,ushort param_6)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 extraout_DX;
  int iVar4;
  undefined2 uVar5;
  uint uVar6;
  ulong uStack6;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_3);
  uVar2 = *(undefined4 *)(param_3 + 0x2e);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  iVar3 = (int)uVar2;
  if (*(int *)(iVar4 + 0x24) < 0x1) {
    pass1_1030_7d1c(uStack6,0x0,0x230000,iVar3,extraout_DX,param_4,param_5,param_6);
  }
  else {
    if (param_2 == 0x0) {
      uVar6 = 0x0;
    }
    else {
      uVar6 = 0x32;
    }
    pass1_1030_7d1c(uStack6,uVar6,0x230000,iVar3,extraout_DX,param_4,param_5,param_6);
    piVar1 = (int *)(iVar4 + 0x24);
    *piVar1 = *piVar1 + -0x1;
  }
  if ((0x0 < *(int *)(iVar4 + 0x24)) && (*(int *)(iVar4 + 0x24) < 0x19)) {
    *(undefined2 *)(iVar3 + 0x1fe) = 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_c1b2(ulong *param_1,uchar *param_2,ushort param_3,ushort param_4,ushort param_5)

{
  int iVar1;
  astruct_695 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  pass1_1028_be9e(param_1,param_3,param_4,(ushort)&USHORT_1050_1028,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_695 *)param_1;
  if (iVar2->field_0x12 == 0x5) {
    if (iVar2->field_0xc == 0xb) {
      pass1_1030_c652(param_2,param_4,param_5);
      iVar1 = 0x82;
      puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,param_2,param_4);
      pass1_1010_9f8c((ulong)puVar3,iVar1,param_5);
      iVar2->field_0x24 = (int)puVar3 * 0x3;
      mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,(uchar *)((ulong)puVar3 >> 0x10),param_4);
      if ((int)PTR_LOOP_1050_13ae < 0x3) {
        iVar1 = iVar2->field_0x24;
        if (iVar1 < 0x32) {
          iVar1 = 0x32;
        }
        iVar2->field_0x24 = iVar1;
        return;
      }
    }
    else {
      iVar2->field_0x24 = 0x64;
    }
  }
  return;
}



void __stdcall16far pass1_1030_c230(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  undefined2 uVar2;
  ushort uVar3;
  undefined4 local_10 [0x2];
  undefined2 local_8 [0x3];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    uVar2 = (undefined2)(param_1 >> 0x10);
    local_10[0] = *(undefined4 *)((int)param_1 + 0x20);
    uVar3 = (ushort)(param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)local_10,param_3,(char *)0x4,0x1008);
    if (BVar1 != 0x0) {
      local_8[0] = *(undefined2 *)((int)param_1 + 0x24);
      BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)local_8,param_3,(char *)0x2,0x1008);
      if (BVar1 != 0x0) {
        return;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far pass1_1030_c29c(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  ushort uVar1;
  BOOL16 BVar2;
  ushort uVar3;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar3 = (ushort)(param_2 >> 0x10);
    BVar2 = read_file_1008_7dee((ushort)param_2,uVar3,(int)param_1 + 0x20,0x0,uVar1,0x4,0x1008);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee((ushort)param_2,uVar3,(int)param_1 + 0x24,0x0,uVar1,0x2,0x1008);
      if (BVar2 != 0x0) {
        return;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_c2fa(ulong param_1,int param_2,uchar *param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  ulong uVar2;
  ulong uVar3;
  ulong uVar4;
  int iVar6;
  uint uVar7;
  ulong uVar8;
  uchar *puVar9;
  ulong uVar10;
  ushort uVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  ushort *puVar14;
  ushort uVar15;
  ushort uStack84;
  long lStack80;
  int iStack56;
  ulong uStack10;
  ulong uStack6;
  astruct_698 *iVar5;
  
  uVar12 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0xc) != 0xb) {
    pass1_1028_bd38(param_1,(ushort)param_3,param_5);
    uVar1 = *(undefined4 *)((int)param_1 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    uStack6 = CONCAT22(param_3,param_2);
    iVar6 = param_2;
    puVar9 = param_3;
    pass1_1028_b58e(param_1);
    uStack10 = CONCAT22(puVar9,iVar6);
    uVar2 = *(ulong *)(iVar6 + 0x2e);
    puVar14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_5,puVar9,param_4);
    uVar10 = (ulong)puVar14 >> 0x10;
    uVar13 = (undefined2)(uVar2 >> 0x10);
    pass1_1010_ed22((ulong)puVar14,*(ulong *)((int)uVar2 + 0x4),param_5);
    uVar3 = *(ulong *)(param_2 + 0x1f6);
    uVar8 = uVar3;
    pass1_1030_3694(uVar3,0x3,0x2,(uchar *)uVar10,0x1010,param_5);
    uVar12 = (undefined2)(uVar3 >> 0x10);
    uVar4 = *(ulong *)((int)uVar2 + 0x1f6);
    pass1_1030_355c(uVar4,uVar8 & 0xffff | uVar10 << 0x10);
    uVar13 = (undefined2)(uVar4 >> 0x10);
    iStack56 = 0x0;
    do {
      iVar5 = (astruct_698 *)(iStack56 * 0x2);
      *(undefined2 *)(iVar5 + (int)uVar4 + 0x174) = *(undefined2 *)(iVar5 + (int)uVar3 + 0x174);
      uVar7 = *(uint *)(iVar5 + (int)uVar3 + 0x180);
      uVar8 = (ulong)uVar7;
      *(uint *)(iVar5 + (int)uVar4 + 0x180) = uVar7;
      iStack56 = iStack56 + 0x1;
    } while (iStack56 < 0x6);
    uStack84 = 0x11;
    while( true ) {
      puVar9 = (uchar *)uVar10;
      uVar7 = (uint)uVar8;
      if (0x24 < (int)uStack84) break;
      if (0x0 < *(int *)(uStack84 * 0x2 + (int)_PTR_LOOP_1050_580e)) {
        empty_1038_540a();
        lStack80 = CONCAT22(puVar9,uVar7);
        uVar12 = (undefined2)((ulong)_PTR_LOOP_1050_580e >> 0x10);
        uVar11 = (ushort)_PTR_LOOP_1050_580e;
        iVar6 = *(int *)(uStack84 * 0x2 + uVar11);
        uVar10 = (ulong)(long)iVar6 >> 0x10;
        uVar15 = uStack84;
        if (lStack80 < iVar6) {
          iVar6 = *(int *)(uStack84 * 0x2 + uVar11);
          uVar10 = (ulong)(uint)(iVar6 >> 0xf);
          uVar15 = 0x21;
        }
        pass1_1038_52b8(uStack6,CONCAT22((int)uVar10,iVar6),uVar15,uVar11,param_4,(ushort)&PTR_LOOP_1050_1038,param_5);
        uVar15 = uStack84 * 0x2;
        uVar7 = *(uint *)(uVar15 + (int)_PTR_LOOP_1050_580e);
        pass1_1030_7ddc(uStack10,(long)(int)uVar7,uStack84,uVar7,(uchar *)uVar10,uVar15,param_4,param_5);
        iVar6 = *(int *)((int)_PTR_LOOP_1050_580e + uVar15);
        uVar8 = SEXT24(iVar6);
        pass1_1038_5694(uVar2,(long)iVar6,uStack84);
      }
      uStack84 = uStack84 + 0x1;
    }
    pass1_1030_7c50(uStack10,0x2,0x1,uVar7,puVar9);
    pass1_1030_7c50(uStack10,0x2,0x2,uVar7,puVar9);
    pass1_1030_7c50(uStack10,0x2,0x3,uVar7,puVar9);
    pass1_1030_7c50(uStack10,0x2,0x4,uVar7,puVar9);
    pass1_1038_44d8(param_2,(ushort)param_3,0x2,0x1,uVar7,(int)puVar9);
    pass1_1038_44d8(param_2,(ushort)param_3,0x2,0x2,uVar7,(int)puVar9);
    pass1_1038_44d8(param_2,(ushort)param_3,0x2,0x3,uVar7,(int)puVar9);
    pass1_1038_44d8(param_2,(ushort)param_3,0x2,0x4,uVar7,(int)puVar9);
    puVar14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,puVar9,param_4);
    pass1_1010_043a((ulong)puVar14,*(long *)(param_2 + 0x4),0x7,param_5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_c52e(ulong param_1,ushort *param_2,ulong param_3,ulong param_4,int param_5,ushort param_6,ushort param_7)

{
  BOOL16 BVar1;
  ulong *puVar2;
  undefined *puVar3;
  undefined4 *puVar4;
  uint uVar5;
  uint uVar6;
  ulong uVar7;
  ushort uVar8;
  ushort *puVar9;
  ulong uVar10;
  undefined local_32 [0x12];
  undefined4 local_20;
  undefined4 uStack28;
  undefined4 *puStack24;
  ulong uStack22;
  undefined2 uStack18;
  uint uStack16;
  ulong local_c;
  uint uStack8;
  ulong uStack6;
  
  uVar8 = (ushort)(param_1 >> 0x10);
  BVar1 = pass1_1028_c314(param_7,param_5,param_6,(ushort)param_1,uVar8,param_2,(ushort)param_3,
                          (ushort)(param_3 >> 0x10),param_4);
  if (BVar1 != 0x0) {
    puVar2 = &local_c;
    pass1_1030_64ce(param_7,puVar2,param_6,_PTR_LOOP_1050_5740,param_2,param_4,(ulong *)CONCAT22(param_7,puVar2));
    local_20 = *puVar2;
    local_20._3_1_ = (byte)(local_20 >> 0x18);
    uStack8 = (uint)local_20._3_1_;
    if (local_20._3_1_ == 0x0) {
      uStack22 = local_20;
      uStack6 = local_20;
      pass1_1028_c7b6(param_7,param_6,(ushort)param_1,uVar8,param_2,param_4);
      if ((uStack8 != 0x4) && (uStack8 != 0x0)) {
        uVar7 = pass1_1030_bcae((ushort)&local_20,param_7);
        uVar5 = (uint)(uVar7 >> 0x10);
        pass1_1028_dc52((astruct_92 *)CONCAT22(param_7,local_32),0x1,0x0,0x400);
        while( true ) {
          puVar3 = local_32;
          pass1_1028_e4ec(CONCAT22(param_7,puVar3));
          uStack28 = CONCAT22(uVar5,puVar3);
          uVar6 = uVar5 | (uint)puVar3;
          if (uVar6 == 0x0) {
            return;
          }
          uVar7 = *(ulong *)(puVar3 + 0x10);
          uVar10 = param_4;
          uStack22 = uVar7;
          puVar9 = param_2;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar7,(uint)(uVar7 >> 0x10));
          uStack18 = (undefined2)uVar7;
          puVar4 = &local_20;
          uStack16 = uVar6;
          pass1_1030_bcde(param_7,(ushort)puVar4,param_7,uVar7 & 0xffff | (ulong)uVar6 << 0x10,puVar9,uVar10);
          if ((int)puVar4 < 0x0) break;
          uVar5 = uVar6;
          puStack24 = puVar4;
          if ((int)puVar4 < 0x1f) {
            PTR_LOOP_1050_50ca = (undefined *)0x6ae;
            return;
          }
        }
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
        return;
      }
      PTR_LOOP_1050_50ca = (undefined *)0x6a8;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_c652(uchar *param_1,int param_2,ushort param_3)

{
  ushort *puVar1;
  
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_3,param_1,param_2);
  pass1_1010_9794((ulong)puVar1,param_3);
  return;
}



astruct_18 * __stdcall16far pass1_1030_c668(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1030_c6f6(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x20) = 0x0;
  *param_1 = 0xc940;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



ushort * __stdcall16far pass1_1030_c71e(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined2 *)(param_1 + 0x20) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xc940;
  *(undefined2 *)(param_1 + 0x2) = 0x1030;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1030_c74e(ulong param_1,ulong param_2,ushort param_3)

{
  pass1_1028_b46e(param_1,param_2,param_3);
  *(undefined2 *)((int)param_1 + 0x20) = 0x70;
  return;
}



void __stdcall16far pass1_1030_c76c(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((*(int *)(iVar1 + 0x12) != 0x6) && (*(int *)(iVar1 + 0x12) != 0x5)) {
    return;
  }
  iVar1 = *(int *)(iVar1 + 0x20);
  if (iVar1 != 0x0) {
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) || (0x1 < iVar1 + -0x70)) {
      pass1_1028_be2a(param_1,param_2,param_3,(ushort)&USHORT_1050_1028,param_4);
      return;
    }
  }
  pass1_1028_bdac(param_1,0x6,(ushort)&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_c7b0(ulong param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  BOOL16 BVar5;
  ulong uVar6;
  uchar *extraout_DX;
  uchar *puVar7;
  int iVar8;
  uint uVar9;
  
  uVar9 = (uint)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  iVar1 = *(int *)(iVar8 + 0x20);
  if (iVar1 != 0x0) {
    iVar3 = iVar1 + -0x70;
    iVar4 = iVar3;
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) || (iVar4 = iVar1 + -0x71, iVar4 != 0x0 && 0x0 < iVar3)) {
      pass1_1028_b58e(param_1 & 0xffff | (ulong)uVar9 << 0x10);
      uVar2 = *(undefined4 *)(iVar4 + 0x2e);
      uVar6 = *(ulong *)((int)uVar2 + 0x200);
      puVar7 = extraout_DX;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar6,(uint)(uVar6 >> 0x10));
      uVar6 = uVar6 & 0xffff | ZEXT24(puVar7) << 0x10;
      BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(iVar8 + 0xc),0x11);
      pass1_1030_23e2(uVar6,BVar5,*(uint *)(iVar8 + 0x20),BVar5,puVar7,param_2,param_3);
      if (BVar5 != 0x0) {
        if (*(int *)(iVar8 + 0x20) == 0x1) {
          pass1_1030_25d8(uVar6,0x64,*(int *)(iVar8 + 0x20));
          return;
        }
        *(undefined2 *)(iVar8 + 0x20) = 0x70;
      }
    }
  }
  return;
}



BOOL16 __stdcall16far pass1_1030_c84e(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  undefined2 local_c [0x5];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    local_c[0] = *(undefined2 *)((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)local_c,param_3,(char *)0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 __stdcall16far pass1_1030_c894(ulong param_1,ulong param_2,BOOL16 param_3,uchar *param_4,ushort param_5)

{
  BOOL16 BVar1;
  undefined2 local_4;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    BVar1 = read_file_1008_7dee((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)&local_4,0x0,param_5,0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return BVar1;
    }
    *(undefined2 *)((int)param_1 + 0x20) = local_4;
    param_3 = 0x1;
  }
  return param_3;
}



ulong __stdcall16far pass1_1030_c8da(ulong param_1,ulong param_2,ulong param_3)

{
  ulong uVar1;
  
  uVar1 = 0x0;
  if (param_3._2_2_ == 0x1) {
    *(undefined2 *)((int)param_1 + 0x20) = param_2._2_2_;
  }
  else {
    uVar1 = func_0x1030178e();
  }
  return uVar1;
}



astruct_18 * __stdcall16far pass1_1030_c91a(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1030_c9a8(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1028_b354(param_1);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x98) = 0x1;
  *param_1 = 0xd88e;
  *(undefined2 *)(iVar1 + 0x2) = 0x1030;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x20)),(WNDCLASS16 *)0x0,0x78);
  return param_1;
}



ulong __stdcall16far pass1_1030_c9e4(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined2 *)(param_1 + 0x98) = 0x1;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xd88e;
  *(undefined2 *)(param_1 + 0x2) = 0x1030;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x20),(WNDCLASS16 *)0x0,0x78);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1030_ca26(ulong param_1,ulong param_2,ushort param_3)

{
  ushort uVar1;
  undefined2 extraout_DX;
  int iVar2;
  undefined2 uVar3;
  ushort uStack4;
  
  for (uStack4 = 0x0; iVar2 = (int)param_1, uVar3 = (undefined2)(param_1 >> 0x10), (int)uStack4 < 0xa;
      uStack4 = uStack4 + 0x1) {
    if ((*(int *)(iVar2 + uStack4 * 0xc + 0x26) == 0x2) || (*(int *)(iVar2 + uStack4 * 0xc + 0x26) == 0x1)) {
      *(undefined2 *)(iVar2 + uStack4 * 0xc + 0x26) = 0x4;
      param_3 = uStack4;
    }
    else {
      uVar1 = uStack4;
      pass1_1028_b58e(param_1);
      iVar2 = uStack4 * 0xc + iVar2;
      pass1_1030_6e9c(CONCAT22(extraout_DX,uVar1),0x1,*(int *)(iVar2 + 0x24));
      param_3 = 0x0;
      *(undefined4 *)(iVar2 + 0x20) = 0x0;
      *(undefined2 *)(iVar2 + 0x24) = 0x0;
      *(undefined2 *)(iVar2 + 0x26) = 0x0;
    }
  }
  pass1_1028_b46e(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_cac2(ulong *param_1,int param_2,ushort param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  ulong uVar4;
  uint uVar5;
  ulong uVar6;
  undefined4 *puVar7;
  ulong uVar8;
  uint extraout_DX;
  uint extraout_DX_00;
  uint extraout_DX_01;
  uint uVar9;
  uint uVar10;
  ulong uStack34;
  int iStack30;
  int iStack28;
  
  pass1_1028_be9e(param_1,param_3,param_4,(ushort)&USHORT_1050_1028,param_5);
  uVar10 = (uint)((ulong)param_1 >> 0x10);
  if ((*(int *)((int)param_1 + 0x12) == 0x5) && (PTR_LOOP_1050_5812 == (undefined *)0x0)) {
    PTR_LOOP_1050_5812 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
    pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar10 << 0x10);
    uVar1 = *(undefined4 *)(param_2 + 0x2e);
    uVar6 = *(ulong *)((int)uVar1 + 0x10);
    uVar10 = extraout_DX;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar6,(uint)(uVar6 >> 0x10));
    puVar2 = (undefined4 *)*(undefined4 *)((int)uVar6 + 0x1e);
    ppcVar3 = (code **)((int)*puVar2 + 0x10);
    puVar7 = puVar2;
    (**ppcVar3)((int)&USHORT_1050_1028,(int)puVar2,*(undefined2 *)((int)uVar6 + 0x20));
    uVar4 = (ulong)puVar7 & 0xffff | (ulong)extraout_DX_00 << 0x10;
    iStack28 = 0x0;
    iStack30 = pass1_1030_d144((ulong)param_1);
    uStack34 = 0x0;
    while ((uStack34 < uVar4 && (iStack30 != 0x0))) {
      ppcVar3 = (code **)((int)*puVar2 + 0x4);
      uVar8 = uVar4;
      (**ppcVar3)((int)&USHORT_1050_1028,(int)puVar2,(int)((ulong)puVar2 >> 0x10),(char)uStack34,(int)(uStack34 >> 0x10)
                 );
      uVar5 = (uint)uVar8;
      uVar9 = extraout_DX_01 | uVar5;
      if (uVar9 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_01);
        uVar5 = *(uint *)(uVar5 + 0xc);
        if ((0x0 < (int)uVar5) && (!SBORROW2(uVar5,0x1))) {
          if (uVar5 != 0x3 && 0x0 < (int)(uVar5 - 0x2)) {
            if (uVar5 != 0x4) goto LAB_1030_cbbc;
            iStack28 = iStack28 + 0x1;
          }
          pass1_1030_6e9c(uVar6 & 0xffff | (ulong)uVar10 << 0x10,0x1,uVar5);
          pass1_1030_d180((ulong)param_1,uVar5);
          iStack30 = iStack30 + -0x1;
        }
      }
LAB_1030_cbbc:
      uStack34 = uStack34 + 0x1;
    }
    while (iStack28 < 0x4) {
      pass1_1030_d180((ulong)param_1,0x4);
      iStack28 = iStack28 + 0x1;
    }
  }
  return;
}



ushort __stdcall16far pass1_1030_cbf0(int param_1,ushort param_2,int param_3)

{
  astruct_595 *iVar1;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    iVar1 = (astruct_595 *)(iStack4 * 0xc + param_1);
    if ((iVar1->field_0x24 == param_3) && (iVar1->field_0x26 == 0x3)) break;
    iStack4 = iStack4 + 0x1;
  }
  iVar1->field_0x26 = 0x0;
  iVar1->field_0x28 = 0x0;
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_cc44(int param_1,ushort param_2,int param_3,ulong param_4,int param_5)

{
  code **ppcVar1;
  int iVar2;
  undefined *puVar3;
  ushort uVar4;
  uint uVar5;
  uint uVar6;
  uint extraout_DX;
  undefined2 extraout_DX_00;
  uchar *puVar7;
  uint extraout_DX_01;
  astruct_304 *iVar7;
  astruct_303 *iVar8;
  undefined uVar8;
  ushort unaff_SS;
  undefined4 *puVar9;
  ulong *puVar10;
  uchar *puVar11;
  undefined local_32 [0x8];
  undefined4 *puStack42;
  ulong uStack38;
  ulong uStack34;
  undefined4 *puStack30;
  uint uStack26;
  uchar *puStack24;
  uint uStack22;
  uchar *puStack20;
  ulong *puStack18;
  int iStack14;
  undefined2 uStack12;
  int iStack10;
  undefined4 uStack8;
  int iStack4;
  
  iStack4 = 0x0;
  uStack8 = *(undefined4 *)((int)param_4 + 0x4);
  iStack10 = 0x0;
  do {
    if (0x9 < iStack10) {
      return;
    }
    iVar8 = (astruct_303 *)(iStack10 * 0xc + param_1);
    if (((iVar8->field_0x28 == (int)uStack8) && (iVar8->field_0x2a == uStack8._2_2_)) && (iVar8->field_0x24 == param_5))
    {
      if (iVar8->field_0x26 == 0x4) {
        iVar2 = param_5;
        pass1_1028_b58e(CONCAT22(param_2,param_1));
        iStack14 = iVar2;
        uStack12 = extraout_DX_00;
        pass1_1030_6e9c(CONCAT13((char)((uint)extraout_DX_00 >> 0x8),CONCAT12((char)extraout_DX_00,iStack14)),0x1,
                        iVar8->field_0x24);
        iVar8->field_0x20 = 0x0;
        iVar8->field_0x24 = 0x0;
        iVar8->field_0x26 = 0x0;
        puStack42 = (undefined4 *)0x0;
        puStack18 = (ulong *)0x0;
        _DAT_0000_0006 = param_5;
        uRam0000000a = 0x1;
        uVar4 = switch_1020_c3b4(param_5);
        *(ushort *)((int)puStack18 + 0xc) = uVar4;
        puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
        puVar7 = (uchar *)((ulong)puVar10 >> 0x10);
        uVar6 = (uint)puVar10;
        uVar5 = uVar6;
        puVar11 = puVar7;
        uStack22 = uVar6;
        puStack20 = puVar7;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
        uVar8 = 0x38;
        uStack26 = uVar6;
        puStack24 = puVar7;
        pass1_1038_4d6e(CONCAT22(puVar7,uVar6),(ulong *)CONCAT22(puVar11,uVar5),uVar6,puVar7);
        puStack30 = (undefined4 *)CONCAT22(puVar7,uVar6);
        ppcVar1 = (code **)((int)*puStack30 + 0x10);
        (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar6,puVar7);
        uStack34 = CONCAT22(extraout_DX_01,uVar6);
        uVar6 = extraout_DX_01;
        for (uStack38 = 0x0; uStack38 < uStack34; uStack38 = uStack38 + 0x1) {
          puVar9 = (undefined4 *)pass1_1030_1d7c((int)uStack34,uVar6,(ulong)puStack30);
          uVar5 = (uint)((ulong)puVar9 >> 0x10);
          uVar6 = uVar5 | (uint)puVar9;
          if (uVar6 != 0x0) {
            puVar3 = local_32;
            ppcVar1 = (code **)((int)*puVar9 + 0x40);
            (**ppcVar1)(0x38,(char)puVar9,uVar5,puVar3);
            uVar6 = extraout_DX;
            if (puVar3 == (undefined *)0x0) {
              uVar8 = 0x28;
              pass1_1028_6408((ulong)puVar9,puStack18,unaff_SS);
              break;
            }
          }
        }
        puStack42 = puStack30;
        if (puStack30 != (undefined4 *)0x0) {
          ppcVar1 = (code **)*puStack30;
          (**ppcVar1)(uVar8,(int)puStack30,(int)((ulong)puStack30 >> 0x10),0x1);
        }
      }
      else {
        iVar7 = (astruct_304 *)(iStack10 * 0xc + param_1);
        iVar7->field_0x26 = 0x0;
        iVar7->field_0x28 = 0x0;
      }
      iStack4 = iStack4 + 0x1;
      param_3 = param_3 + -0x1;
      if (param_3 == 0x0) {
        return;
      }
    }
    iStack10 = iStack10 + 0x1;
  } while( true );
}

