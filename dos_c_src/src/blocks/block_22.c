

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_c626(ulong *param_1)

{
  _PTR_LOOP_1050_06e0 = 0x0;
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1008_c646(ushort param_1,ulong param_2,ushort param_3)

{
  int *piVar1;
  int iVar2;
  uchar *puVar3;
  int unaff_DI;
  ulong *puVar4;
  ushort *puVar5;
  int iStack18;
  int iStack16;
  
  puVar4 = pass1_1008_c6fa((int *)CONCAT22((int)param_2,param_1),(int)(param_2 >> 0x10));
  puVar3 = (uchar *)((ulong)puVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_3,puVar3,unaff_DI);
  iStack18 = 0x0;
  iStack16 = 0x0;
  while ((piVar1 = (int *)((int)puVar4 + 0x4), iVar2 = iStack16, *piVar1 != iStack18 && iStack18 <= *piVar1 &&
         (iVar2 = *(int *)((int)*puVar4 + iStack18 * 0x2), *(int *)(iVar2 * 0x2 + (int)puVar5 + 0xa) == 0x0))) {
    iStack18 = iStack18 + 0x1;
  }
  iStack16 = iVar2;
  return iStack16;
}



BOOL16 __stdcall16far pass1_1008_c6ae(ulong param_1,int param_2,int param_3)

{
  int *piVar1;
  ulong *puVar2;
  int iStack8;
  
  puVar2 = pass1_1008_c6fa((int *)param_1,param_3);
  iStack8 = 0x0;
  while( true ) {
    piVar1 = (int *)((int)puVar2 + 0x4);
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
      return 0x0;
    }
    if (*(int *)((int)*puVar2 + iStack8 * 0x2) == param_2) break;
    iStack8 = iStack8 + 0x1;
  }
  return 0x1;
}



ulong * __stdcall16far pass1_1008_c6fa(int *param_1,int param_2)

{
  if ((0x0 < param_2) && (param_2 < 0x47)) {
    return (ulong *)CONCAT22(*(undefined2 *)((int)param_1 + 0x2),param_2 * 0x6 + *param_1);
  }
  return (ulong *)0x0;
}



void __stdcall16far pass1_1008_c72a(astruct_642 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xca4a;
  param_1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_c75c(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_469 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_469 *)param_1;
  *param_1 = 0xca4a;
  iVar4->field_0x2 = 0x1008;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_c79a(ulong param_1,ulong param_2,int param_3,ushort param_4,uchar param_5)

{
  undefined *puVar1;
  int iVar2;
  ulong uVar3;
  uint extraout_DX;
  uchar *puVar4;
  ushort uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  undefined local_12 [0x4];
  undefined4 uStack14;
  undefined local_a [0x8];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0xa));
  while( true ) {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    uStack14 = CONCAT22(extraout_DX,puVar1);
    puVar4 = (uchar *)(extraout_DX | (uint)puVar1);
    if (puVar4 == (uchar *)0x0) break;
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
    if (iVar2 == 0x0) {
      puVar7 = pass1_1020_a43e(param_4,puVar4,(ushort *)CONCAT22(param_4,local_12));
      uVar5 = (ushort)((ulong)puVar7 >> 0x10);
      pass1_1020_a6ee(CONCAT22(param_4,local_12),*(ushort *)((int)uStack14 + 0x12),(uint)local_12,uVar5,param_3,param_4,
                      param_5);
      uVar3 = *(ulong *)((int)_PTR_LOOP_1050_65e2 + 0x52);
      pass1_1030_4bbe(param_4,uVar5,uVar3,*(int *)((int)uStack14 + 0x12));
      *(long *)((int)param_1 + 0xe) = (long)*(int *)((int)uVar3 + 0x94) + *_PTR_LOOP_1050_65e2;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_c83a(ulong param_1)

{
  if (*_PTR_LOOP_1050_65e2 <= *(ulong *)((int)param_1 + 0xe)) {
    return;
  }
  return;
}



ulong __stdcall16far pass1_1008_c85e(ulong param_1,ushort param_2)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(long *)(iVar1 + 0xa) == 0x0) {
    pass1_1008_c882(param_1 & 0xffff | (ulong)uVar2 << 0x10,param_2);
  }
  return CONCAT22(*(undefined2 *)(iVar1 + 0xc),*(undefined2 *)(iVar1 + 0xa));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_c882(ulong param_1,ushort param_2)

{
  int *piVar1;
  undefined4 *puVar2;
  ushort uVar3;
  undefined4 *puVar4;
  code **ppcVar5;
  ushort uVar6;
  ushort uVar7;
  uchar *puVar8;
  uchar *extraout_DX;
  uchar *puVar9;
  uchar *puVar10;
  uint uVar11;
  astruct_201 *iVar9;
  int unaff_DI;
  undefined2 uVar12;
  undefined2 uVar13;
  astruct_21 *paVar14;
  undefined4 uVar15;
  ushort *puVar16;
  ulong *puVar17;
  int iStack16;
  
  uVar12 = (undefined2)(param_1 >> 0x10);
  iVar9 = (astruct_201 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar2 = iVar9->field_0xa;
  uVar11 = *(uint *)((int)&iVar9->field_0xa + 0x2);
  paVar14 = (astruct_21 *)CONCAT22(uVar11,puVar2);
  if ((uVar11 | (uint)puVar2) != 0x0) {
    ppcVar5 = (code **)*puVar2;
    paVar14 = (astruct_21 *)(**ppcVar5)();
  }
  mem_op_1000_179c(0xc,(uchar *)((ulong)paVar14 >> 0x10),0x1000);
  if (paVar14 == (astruct_21 *)0x0) {
    uVar15 = 0x0;
  }
  else {
    uVar15 = set_struct_1008_574a(paVar14);
  }
  puVar9 = (uchar *)((ulong)uVar15 >> 0x10);
  *(int *)&iVar9->field_0xa = (int)uVar15;
  *(uchar **)((int)&iVar9->field_0xa + 0x2) = puVar9;
  puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_2,puVar9,unaff_DI);
  puVar17 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x44);
  puVar8 = (uchar *)((ulong)puVar17 >> 0x10);
  iStack16 = 0x0;
  puVar9 = puVar8;
  while( true ) {
    piVar1 = (int *)((int)puVar17 + 0x4);
    if (*piVar1 == iStack16 || *piVar1 < iStack16) break;
    uVar3 = *(ushort *)((int)*puVar17 + iStack16 * 0x2);
    if (*(int *)(uVar3 * 0x2 + (int)puVar16 + 0xa) != 0x0) {
      uVar6 = pass1_1020_bd80(uVar3);
      uVar7 = str_op_1008_60e8((char *)CONCAT22(puVar9,uVar6),(ushort)puVar9);
      uVar13 = 0x1000;
      uVar6 = uVar7;
      puVar10 = puVar9;
      mem_op_1000_179c(0x14,puVar9,0x1000);
      uVar11 = (uint)puVar10 | uVar6;
      if (uVar11 == 0x0) {
        uVar6 = 0x0;
        uVar11 = 0x0;
      }
      else {
        uVar13 = 0x1018;
        struct_1018_47c8((ushort *)CONCAT22(puVar10,uVar6),0x1,CONCAT22(puVar9,uVar7),uVar3,0x0);
      }
      puVar4 = iVar9->field_0xa;
      ppcVar5 = (code **)((int)*iVar9->field_0xa + 0x4);
      (**ppcVar5)(uVar13,(int)puVar4,(char)((ulong)puVar4 >> 0x10),uVar6,uVar11);
      puVar9 = extraout_DX;
    }
    iStack16 = iStack16 + 0x1;
  }
  return;
}



void __stdcall16far pass1_1008_c98e(ulong param_1,ulong param_2,HFILE16 param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  undefined4 local_10 [0x3];
  
  BVar1 = write_to_file_1008_7cac(param_2,param_4);
  if (BVar1 != 0x0) {
    local_10[0] = *(undefined4 *)((int)param_1 + 0xe);
    BVar1 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)local_10,param_4,(char *)0x4,param_3);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1008_c9d4(ulong param_1,ulong param_2,int param_3,uint16_t param_4,longlong param_5)

{
  BOOL16 BVar1;
  uint16_t unaff_SS;
  ushort uVar2;
  
  if (0x1 < (int)PTR_LOOP_1050_0312) {
    uVar2 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe((ushort)param_2,uVar2,0x15,param_4,unaff_SS);
    if (param_3 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d4;
      return;
    }
    BVar1 = read_file_1008_7dee((ushort)param_2,uVar2,(int)param_1 + 0xe,0x0,(ushort)(param_1 >> 0x10),0x4,param_4);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
  }
  return;
}



ulong __stdcall16far pass1_1008_ca24(ulong param_1,byte param_2,ushort param_3)

{
  pass1_1008_c75c((ushort *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_ca5a(astruct_639 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xd71a;
  param_1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_caa0(ushort *param_1,ushort param_2)

{
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  *param_1 = 0xd71a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  pass1_1008_cac6((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1008_cac6(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_470 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_470 *)param_1;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0xa = 0x0;
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0xe = 0x0;
  puVar1 = iVar4->field_0x12;
  uVar2 = iVar4->field_0x14;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x12 = 0x0;
  puVar1 = iVar4->field_0x16;
  uVar2 = iVar4->field_0x18;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x16 = 0x0;
  puVar1 = iVar4->field_0x1a;
  uVar2 = iVar4->field_0x1c;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x1a = 0x0;
  puVar1 = iVar4->field_0x1e;
  uVar2 = iVar4->field_0x20;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x1e = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_cbc4(ulong param_1,ulong param_2,ushort param_3)

{
  long *plVar1;
  code **ppcVar2;
  bool bVar3;
  undefined4 *puVar4;
  ushort uVar5;
  undefined *puVar6;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar8;
  uchar *puVar9;
  uchar *extraout_DX_01;
  astruct_202 *iVar10;
  undefined2 uVar10;
  char *pcVar11;
  ulong uStack64;
  ulong uStack52;
  int iStack30;
  undefined local_18 [0x8];
  undefined2 uStack16;
  uchar *puStack14;
  undefined2 uStack12;
  uchar *puStack10;
  int iStack8;
  long lStack6;
  ulong uVar7;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar10 = (astruct_202 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar4 = iVar10->field_0x1e;
  puVar8 = *(uchar **)((int)&iVar10->field_0x1e + 0x2);
  if (((uint)puVar8 | (uint)puVar4) != 0x0) {
    ppcVar2 = (code **)*puVar4;
    (**ppcVar2)();
    puVar8 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar8,0x1000);
  if (((uint)puVar8 | (uint)puVar4) == 0x0) {
    puVar4 = (undefined4 *)0x0;
    puVar8 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar8,puVar4));
    puVar8 = extraout_DX_00;
  }
  *(undefined4 **)&iVar10->field_0x1e = puVar4;
  *(uchar **)((int)&iVar10->field_0x1e + 0x2) = puVar8;
  lStack6 = *(long *)((int)param_2 + 0x200);
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_18),0x1,0x0,0x400);
  iStack30 = 0x0;
  while( true ) {
    puVar6 = local_18;
    pass1_1028_e4ec(CONCAT22(param_3,puVar6));
    puVar9 = (uchar *)((uint)puVar8 | (uint)puVar6);
    if (puVar9 == (uchar *)0x0) break;
    plVar1 = (long *)(puVar6 + 0x200);
    puVar8 = puVar9;
    if (*plVar1 == lStack6) {
      iStack30 = iStack30 + 0x1;
    }
  }
  bVar3 = false;
  if (0x1 < iStack30) {
    uStack16 = uStack12;
    puStack14 = puStack10;
    if (iStack8 != 0x0) {
      uStack16 = 0x1;
      puStack14 = (uchar *)0x0;
      puStack10 = puStack14;
    }
    while( true ) {
      puVar8 = puStack10;
      puVar6 = local_18;
      pass1_1028_e4ec(CONCAT22(param_3,puVar6));
      puVar9 = (uchar *)((uint)puVar8 | (uint)puVar6);
      if (puVar9 == (uchar *)0x0) break;
      puStack10 = puVar9;
      if ((*(long *)(puVar6 + 0x200) == lStack6) && (*(long *)(puVar6 + 0x4) != 0x4000001)) {
        pcVar11 = pass1_1038_4d28(CONCAT22(puVar8,puVar6));
        puVar9 = (uchar *)((ulong)pcVar11 >> 0x10);
        uVar5 = str_op_1008_60e8(pcVar11,(ushort)puVar9);
        uVar7 = (ulong)uVar5;
        uStack52 = CONCAT22(puVar9,uVar5);
        mem_op_1000_179c(0x12,puVar9,0x1000);
        if (((uint)puVar9 | (uint)uVar7) != 0x0) {
          struct_1018_4920((ushort *)(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10),0x1,uStack52,*(ulong *)(puVar6 + 0x4));
        }
        ppcVar2 = (code **)((int)*iVar10->field_0x1e + 0x4);
        (**ppcVar2)();
        bVar3 = true;
        puStack10 = extraout_DX_01;
      }
    }
  }
  if (!bVar3) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uStack64 = CONCAT22(puVar9,puVar6);
    mem_op_1000_179c(0x12,puVar9,0x1000);
    if (((uint)puVar9 | (uint)puVar6) != 0x0) {
      struct_1018_4920((ushort *)CONCAT22(puVar9,puVar6),0x0,uStack64,0x0);
    }
    ppcVar2 = (code **)((int)*iVar10->field_0x1e + 0x4);
    (**ppcVar2)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_cda2(ulong param_1,ulong param_2,ushort param_3)

{
  long *plVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  undefined4 *puVar4;
  char *pcVar5;
  ushort uVar6;
  ushort uVar7;
  astruct_206 *puVar9;
  uint uVar8;
  uint uVar9;
  ulong uVar10;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  uchar *puVar11;
  undefined2 uVar12;
  uint extraout_DX_01;
  uchar *puVar13;
  astruct_205 *iVar15;
  undefined2 uVar14;
  undefined2 uVar15;
  undefined uVar16;
  ushort *puVar17;
  long lStack50;
  undefined local_2e [0xa];
  ushort uStack36;
  ulong uStack34;
  ulong uStack30;
  undefined4 uStack26;
  undefined4 *puStack18;
  uchar *puStack16;
  ushort *puStack14;
  undefined2 uStack10;
  undefined4 uStack8;
  int iStack4;
  
  uVar14 = (undefined2)(param_1 >> 0x10);
  iVar15 = (astruct_205 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar4 = iVar15->field_0x1a;
  puVar13 = *(uchar **)((int)&iVar15->field_0x1a + 0x2);
  puStack14 = (ushort *)CONCAT22(puVar13,puVar4);
  puStack18 = puVar4;
  puStack16 = puVar13;
  if (((uint)puVar13 | (uint)puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
    puVar13 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar13,0x1000);
  puStack18 = puVar4;
  puStack16 = puVar13;
  if (((uint)puVar13 | (uint)puVar4) == 0x0) {
    puVar4 = (undefined4 *)0x0;
    uVar15 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar13,puVar4));
    uVar15 = extraout_DX_00;
  }
  *(undefined4 **)&iVar15->field_0x1a = puVar4;
  *(undefined2 *)((int)&iVar15->field_0x1a + 0x2) = uVar15;
  iStack4 = 0x0;
  uVar15 = (undefined2)(param_2 >> 0x10);
  uStack8 = *(undefined4 *)((int)param_2 + 0x210);
  uStack26._2_2_ = *(uchar **)((int)param_2 + 0x212);
  uVar10 = (ulong)((uint)uStack26._2_2_ | (uint)uStack8);
  if (((uint)uStack26._2_2_ | (uint)uStack8) != 0x0) {
    uStack26 = *(ulong *)((uint)uStack8 + 0xa);
    uStack30 = 0x0;
    while( true ) {
      uVar10 = uStack26;
      if (uStack26 <= uStack30) break;
      bad_1030_1312();
      uStack34 = uVar10 & 0xffff | ZEXT24(uStack26._2_2_) << 0x10;
      if (((uint)uStack26._2_2_ | (uint)uVar10) != 0x0) {
        for (uStack36 = 0x1; (int)uStack36 < 0x15; uStack36 = uStack36 + 0x1) {
          local_2e._8_2_ = pass1_1030_ce2e((int)uStack34,(ushort)(uStack34 >> 0x10),uStack36);
          if (local_2e._8_2_ != 0x0) {
            pass1_1008_5784((ulong *)CONCAT22(param_3,local_2e),(ulong)iVar15->field_0x1a);
            do {
              puVar9 = (astruct_206 *)local_2e;
              pass1_1008_5b12(puVar9,param_3);
              lStack50 = CONCAT22(extraout_DX_01,puVar9);
              puVar13 = (uchar *)(extraout_DX_01 | (uint)puVar9);
              if (puVar13 == (uchar *)0x0) break;
            } while (puVar9->field_0xe != uStack36);
            if (lStack50 == 0x0) {
              pcVar5 = string_op_1020_c222(uStack36);
              uVar6 = str_op_1008_60e8((char *)CONCAT22(puVar13,pcVar5),(ushort)puVar13);
              uVar16 = 0x0;
              puVar11 = puVar13;
              uVar7 = uVar6;
              mem_op_1000_179c(0x10,puVar13,0x1000);
              puStack14 = (ushort *)CONCAT22(puVar11,uVar7);
              if (((uint)puVar11 | uVar7) == 0x0) {
                uVar15 = 0x0;
                uVar12 = 0x0;
              }
              else {
                uVar16 = 0x18;
                puVar17 = struct_1018_48b0(puStack14,
                                           CONCAT22((int)local_2e._8_2_ >> 0xf,
                                                    local_2e._8_2_ & 0xff |
                                                    (uint)(byte)((ulong)(long)(int)local_2e._8_2_ >> 0x8) << 0x8),
                                           CONCAT22(puVar13,uVar6),uStack36);
                uVar12 = (undefined2)((ulong)puVar17 >> 0x10);
                uVar15 = SUB42(puVar17,0x0);
              }
              puVar2 = iVar15->field_0x1a;
              ppcVar3 = (code **)((int)*iVar15->field_0x1a + 0x4);
              (**ppcVar3)(uVar16,(int)puVar2,(char)((ulong)puVar2 >> 0x10),uVar15,uVar12);
            }
            else {
              plVar1 = &puVar9->field_0x8;
              *plVar1 = *plVar1 + (long)(int)local_2e._8_2_;
            }
            iStack4 = 0x1;
          }
        }
      }
      uStack30 = uStack30 + 0x1;
    }
  }
  uVar8 = (uint)uVar10;
  uStack10 = 0x0;
  if (iStack4 == 0x0) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar16 = 0x0;
    puVar13 = uStack26._2_2_;
    uVar9 = uVar8;
    mem_op_1000_179c(0x10,uStack26._2_2_,0x1000);
    puStack18 = (undefined4 *)uVar9;
    puStack16 = puVar13;
    if (((uint)puVar13 | uVar9) == 0x0) {
      uVar15 = 0x0;
      uVar12 = 0x0;
    }
    else {
      uVar16 = 0x18;
      puVar17 = struct_1018_48b0((ushort *)CONCAT22(puVar13,uVar9),0x0,CONCAT22(uStack26._2_2_,uVar8),0x0);
      uVar12 = (undefined2)((ulong)puVar17 >> 0x10);
      uVar15 = SUB42(puVar17,0x0);
    }
    puVar2 = iVar15->field_0x1a;
    ppcVar3 = (code **)((int)*iVar15->field_0x1a + 0x4);
    (**ppcVar3)(uVar16,(int)puVar2,(char)((ulong)puVar2 >> 0x10),uVar15,uVar12);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_cfa0(ulong param_1,ulong param_2)

{
  ulong uVar1;
  undefined4 uVar2;
  code **ppcVar3;
  bool bVar4;
  undefined4 *puVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  undefined2 uVar9;
  ulong uVar10;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar11;
  uchar *puVar12;
  uchar *puVar13;
  uchar *extraout_DX_01;
  uchar *extraout_DX_02;
  uchar *extraout_DX_03;
  undefined2 uVar14;
  int iVar15;
  undefined2 uVar16;
  undefined2 uVar17;
  ushort unaff_SS;
  ushort *puVar18;
  
  uVar16 = (undefined2)(param_1 >> 0x10);
  iVar15 = (int)param_1;
  puVar5 = (undefined4 *)*(uint *)(iVar15 + 0x16);
  puVar11 = *(uchar **)(iVar15 + 0x18);
  if (((uint)puVar11 | (uint)puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
    puVar11 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar11,0x1000);
  if (((uint)puVar11 | (uint)puVar5) == 0x0) {
    puVar5 = (undefined4 *)0x0;
    puVar11 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar11,puVar5));
    puVar11 = extraout_DX_00;
  }
  *(undefined2 *)(iVar15 + 0x16) = puVar5;
  *(uchar **)(iVar15 + 0x18) = puVar11;
  bVar4 = false;
  uVar1 = *(ulong *)((int)param_2 + 0x1f6);
  uVar10 = uVar1;
  pass1_1030_38f2(uVar1,0x2,unaff_SS);
  uVar6 = (uint)uVar10;
  if ((-0x1 < (int)puVar11) && ((0x0 < (int)puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar6;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if (((uint)puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar13,uVar7),uVar10 & 0xffff | ZEXT24(puVar11) << 0x10,
                                 CONCAT22(puVar12,uVar6),0x2);
      uVar9 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar6 = (uint)puVar18;
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_01;
  }
  pass1_1030_38f2(uVar1,0x3,unaff_SS);
  if ((-0x1 < (int)puVar11) && ((0x0 < (int)puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    uVar8 = uVar6;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar8;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if (((uint)puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar13,uVar7),CONCAT22(puVar11,uVar6),CONCAT22(puVar12,uVar8),0x3);
      uVar9 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar6 = (uint)puVar18;
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_02;
  }
  pass1_1030_38f2(uVar1,0x4,unaff_SS);
  if ((-0x1 < (int)puVar11) && ((0x0 < (int)puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    uVar8 = uVar6;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar8;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if (((uint)puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar13,uVar7),CONCAT22(puVar11,uVar6),CONCAT22(puVar12,uVar8),0x4);
      uVar9 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar6 = (uint)puVar18;
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_03;
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar12 = puVar11;
    uVar7 = uVar6;
    mem_op_1000_179c(0x14,puVar11,0x1000);
    if (((uint)puVar12 | uVar7) == 0x0) {
      uVar9 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar12,uVar7),0x0,CONCAT22(puVar11,uVar6),0x0);
      uVar14 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar9 = SUB42(puVar18,0x0);
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar9,uVar14);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_str_op_1008_d1c6(ulong param_1,ulong param_2)

{
  int iVar1;
  undefined4 uVar2;
  code **ppcVar3;
  bool bVar4;
  undefined4 *puVar5;
  uint uVar6;
  ushort uVar7;
  ushort uVar8;
  uint uVar9;
  undefined uVar10;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  uchar *puVar11;
  uchar *extraout_DX_01;
  uint uVar12;
  uchar *puVar13;
  uchar *extraout_DX_02;
  uchar *puVar14;
  uint uVar15;
  int iVar16;
  WORD *valist;
  undefined2 uVar17;
  ulong *puVar18;
  ulong uVar19;
  uint uStack52;
  ulong uStack20;
  ulong uStack14;
  undefined4 *puStack10;
  
  valist = (WORD *)(param_1 >> 0x10);
  iVar16 = (int)param_1;
  puVar5 = (undefined4 *)*(uint *)(iVar16 + 0x12);
  puVar13 = *(uchar **)(iVar16 + 0x14);
  if (((uint)puVar13 | (uint)puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
    puVar13 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar13,0x1000);
  if (((uint)puVar13 | (uint)puVar5) == 0x0) {
    puVar5 = (undefined4 *)0x0;
    uVar17 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar13,puVar5));
    uVar17 = extraout_DX_00;
  }
  *(undefined2 *)(iVar16 + 0x12) = puVar5;
  *(undefined2 *)(iVar16 + 0x14) = uVar17;
  puVar18 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
  puVar11 = (uchar *)((ulong)puVar18 >> 0x10);
  uVar6 = (uint)puVar18;
  uVar17 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4e78(uVar6,puVar11,param_2,puVar18);
  puStack10 = (undefined4 *)CONCAT22(puVar11,uVar6);
  ppcVar3 = (code **)((int)*puStack10 + 0x10);
  uVar9 = uVar6;
  (**ppcVar3)((int)&PTR_LOOP_1050_1038,uVar6,puVar11);
  uStack14 = CONCAT22(extraout_DX_01,uVar9);
  bVar4 = false;
  puVar13 = extraout_DX_01;
  for (uStack20 = 0x0; uStack20 < uStack14; uStack20 = uStack20 + 0x1) {
    uVar17 = 0x1030;
    uVar19 = pass1_1030_1d7c(uVar9,puVar13,(ulong)puStack10);
    uVar12 = (uint)(uVar19 >> 0x10);
    uVar15 = (uint)uVar19;
    puVar13 = (uchar *)(uVar12 | uVar15);
    if (((puVar13 != (uchar *)0x0) && (*(long *)(uVar15 + 0x1c) != 0x8000002)) &&
       ((iVar1 = *(int *)(uVar15 + 0x12), iVar1 == 0x5 || (iVar1 == 0x6)))) {
      puVar13 = (uchar *)(*(uint *)(uVar15 + 0x6) & 0xff);
      pass1_1020_bd80(*(ushort *)(uVar15 + 0xc));
      wsprintf16((LPSTR)0x1020,(LPCSTR)(iVar16 + 0x22),valist);
      uVar7 = str_op_1008_60e8((char *)(param_1 & 0xffff0000 | (ulong)(iVar16 + 0x22)),(ushort)puVar13);
      uVar17 = 0x1000;
      puVar14 = puVar13;
      uVar8 = uVar7;
      mem_op_1000_179c(0x12,puVar13,0x1000);
      uStack52 = (uint)puVar14 | uVar8;
      if (uStack52 == 0x0) {
        uVar8 = 0x0;
        uStack52 = 0x0;
      }
      else {
        uVar17 = 0x1018;
        pass1_1018_4808((ushort *)CONCAT22(puVar14,uVar8),0x1,
                        CONCAT13((char)((uint)puVar13 >> 0x8),CONCAT12((char)puVar13,uVar7)),*(ulong *)(uVar15 + 0x4));
      }
      uVar2 = *(undefined4 *)(iVar16 + 0x12);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar16 + 0x12) + 0x4);
      (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar8,uStack52);
      bVar4 = true;
      puVar13 = extraout_DX_02;
    }
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar9 = (uint)uStack14;
    uVar17 = 0x1000;
    puVar14 = puVar13;
    mem_op_1000_179c(0x12,puVar13,0x1000);
    uVar15 = (uint)puVar14 | uVar9;
    if (uVar15 == 0x0) {
      uVar9 = 0x0;
      uVar10 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      pass1_1018_4808((ushort *)CONCAT22(puVar14,uVar9),0x0,uStack14 & 0xffff | ZEXT24(puVar13) << 0x10,0x0);
      uVar10 = (undefined)uVar15;
    }
    uVar2 = *(undefined4 *)(iVar16 + 0x12);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar16 + 0x12) + 0x4);
    (**ppcVar3)(uVar17,(char)uVar2,(int)((ulong)uVar2 >> 0x10),uVar9,uVar10);
  }
  if (((uint)puVar11 | uVar6) != 0x0) {
    ppcVar3 = (code **)*puStack10;
    (**ppcVar3)(uVar17,uVar6,(char)puVar11,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_d3ae(ulong param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  bool bVar4;
  uint uVar5;
  ushort uVar6;
  ushort uVar7;
  uchar *puVar8;
  uchar *puVar9;
  astruct_208 *iVar13;
  undefined2 uVar10;
  undefined2 uVar11;
  astruct_21 *paVar12;
  ulong uVar13;
  undefined4 uVar14;
  ulong uVar15;
  ushort uStack6;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar13 = (astruct_208 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar1 = iVar13->field_0xa;
  uVar5 = *(uint *)((int)&iVar13->field_0xa + 0x2);
  paVar12 = (astruct_21 *)CONCAT22(uVar5,puVar1);
  if ((uVar5 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    paVar12 = (astruct_21 *)(**ppcVar3)();
  }
  mem_op_1000_179c(0xc,(uchar *)((ulong)paVar12 >> 0x10),0x1000);
  if (paVar12 == (astruct_21 *)0x0) {
    uVar13 = 0x0;
  }
  else {
    uVar13 = set_struct_1008_574a(paVar12);
  }
  *(int *)&iVar13->field_0xa = (int)uVar13;
  *(undefined2 *)((int)&iVar13->field_0xa + 0x2) = (int)(uVar13 >> 0x10);
  bVar4 = false;
  for (uStack6 = 0x21; 0x10 < (int)uStack6; uStack6 = uStack6 - 0x1) {
    uVar15 = uVar13;
    empty_1038_540a();
    puVar8 = (uchar *)(uVar15 >> 0x10);
    uVar5 = (uint)puVar8 | (uint)uVar15;
    uVar13 = uVar15 & 0xffff0000 | (ulong)uVar5;
    if (uVar15 != 0x0) {
      bVar4 = true;
      string_1020_c0ca(uStack6);
      uVar6 = str_op_1008_60e8((char *)CONCAT22(puVar8,uVar5),(ushort)puVar8);
      uVar11 = 0x1000;
      uVar7 = uVar6;
      puVar9 = puVar8;
      mem_op_1000_179c(0x10,puVar8,0x1000);
      if (((uint)puVar9 | uVar7) == 0x0) {
        uVar14 = 0x0;
      }
      else {
        uVar11 = 0x1018;
        uVar14 = struct_1018_4790(CONCAT22(puVar9,uVar7),uVar15,CONCAT22(puVar8,uVar6),uStack6);
      }
      puVar2 = iVar13->field_0xa;
      ppcVar3 = (code **)((int)*iVar13->field_0xa + 0x4);
      uVar13 = (**ppcVar3)(uVar11,(char)puVar2,(char)((ulong)puVar2 >> 0x10),uVar14);
    }
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar11 = 0x1000;
    uVar15 = uVar13;
    mem_op_1000_179c(0x10,(uchar *)(uVar13 >> 0x10),0x1000);
    if (uVar15 == 0x0) {
      uVar14 = 0x0;
    }
    else {
      uVar11 = 0x1018;
      uVar14 = struct_1018_4790(uVar15,0x0,uVar13,0x0);
    }
    puVar2 = iVar13->field_0xa;
    ppcVar3 = (code **)((int)*iVar13->field_0xa + 0x4);
    (**ppcVar3)(uVar11,(char)puVar2,(char)((ulong)puVar2 >> 0x10),uVar14);
  }
  return;
}
