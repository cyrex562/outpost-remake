


ushort __stdcall16far pass1_1030_7bee(ulong param_1)

{
  code **ppcVar1;
  ushort uVar2;
  int iVar3;
  uint uVar4;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(long *)(iVar3 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar4 << 0x10);
  }
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x1a) + 0x44);
  uVar2 = (**ppcVar1)();
  return uVar2;
}



undefined4 __stdcall16far pass1_1030_7c28(ulong param_1,ushort param_2,uint param_3,uint param_4,ushort param_5)

{
  undefined2 uVar1;
  undefined4 uVar2;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x22) == 0x0) {
    return 0x0;
  }
  uVar2 = *(undefined4 *)((int)param_1 + 0x22);
  uVar2 = pass1_1020_bae6((ushort)uVar2,CONCAT22(param_2,(int)((ulong)uVar2 >> 0x10)),param_3,param_4,param_5);
  return uVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_7c50(ulong param_1,long param_2,int param_3,uint param_4,uchar *param_5)

{
  int *piVar1;
  code **ppcVar2;
  ushort uVar3;
  ulong uVar4;
  ushort uVar5;
  uchar *puVar6;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 uVar7;
  uchar *extraout_DX_01;
  astruct_305 *iVar8;
  undefined2 uVar8;
  ulong uVar9;
  undefined4 *puVar10;
  undefined4 *puStack18;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar8 = (astruct_305 *)param_1;
  puVar6 = param_5;
  if (iVar8->field_0x1e == (undefined4 *)0x0) {
    mem_op_1000_179c(0x18,param_5,0x1000);
    puVar6 = (uchar *)((uint)param_5 | param_4);
    if (puVar6 == (uchar *)0x0) {
      iVar8->field_0x1e = (undefined4 *)0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_5,param_4),0x5,0x5);
      *(uint *)&iVar8->field_0x1e = param_4;
      *(uchar **)((int)&iVar8->field_0x1e + 0x2) = extraout_DX;
      puVar6 = extraout_DX;
    }
  }
  if (param_3 == 0x4) {
    piVar1 = &iVar8->field_0x34;
    *piVar1 = *piVar1 + (int)param_2;
  }
  while (param_2 != 0x0) {
    uVar9 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)puVar6,0x6);
    uVar3 = (ushort)uVar9;
    uVar4 = uVar9 >> 0x10;
    puVar10 = iVar8->field_0x1e;
    ppcVar2 = (code **)((int)*iVar8->field_0x1e + 0xc);
    uVar5 = uVar3;
    (**ppcVar2)();
    uVar7 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,(uint)uVar4);
    puStack18 = (undefined4 *)CONCAT22(uVar7,uVar5);
    ppcVar2 = (code **)((int)*puStack18 + 0x14);
    (**ppcVar2)((int)&USHORT_1050_1028,uVar5,uVar7,param_1,puVar10,uVar9);
    puVar6 = extraout_DX_01;
    param_2 = param_2 + -0x1;
  }
  return;
}



void __stdcall16far
pass1_1030_7d1c(ulong param_1,uint param_2,ulong param_3,uint param_4,uchar *param_5,ushort param_6,ushort param_7,
               ushort param_8)

{
  uint uVar1;
  astruct_397 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_397 *)param_1;
  if (iVar2->field_0x22 == (long *)0x0) {
    mem_op_1000_179c(0xa,param_5,0x1000);
    uVar1 = (uint)param_5 | param_4;
    if (uVar1 == 0x0) {
      iVar2->field_0x22 = (long *)0x0;
    }
    else {
      pass1_1020_ba3e((long *)CONCAT22(param_5,param_4),0xa,0x2,param_7,param_6);
      *(uint *)&iVar2->field_0x22 = param_4;
      *(uint *)((int)&iVar2->field_0x22 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field_0x22,param_2,param_3,param_7,param_8);
  return;
}



void __stdcall16far
pass1_1030_7d7c(ulong param_1,uint param_2,ulong param_3,uint param_4,uchar *param_5,ushort param_6,ushort param_7,
               ushort param_8)

{
  uint uVar1;
  astruct_398 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_398 *)param_1;
  if (iVar2->field_0x26 == (long *)0x0) {
    mem_op_1000_179c(0xa,param_5,0x1000);
    uVar1 = (uint)param_5 | param_4;
    if (uVar1 == 0x0) {
      iVar2->field_0x26 = (long *)0x0;
    }
    else {
      pass1_1020_ba3e((long *)CONCAT22(param_5,param_4),0xa,0x2,param_7,param_6);
      *(uint *)&iVar2->field_0x26 = param_4;
      *(uint *)((int)&iVar2->field_0x26 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field_0x26,param_2,param_3,param_7,param_8);
  return;
}



void __stdcall16far
pass1_1030_7ddc(ulong param_1,long param_2,ushort param_3,uint param_4,uchar *param_5,ushort param_6,ushort param_7,
               ushort param_8)

{
  undefined4 uVar1;
  uchar *puVar2;
  int iVar3;
  undefined2 uVar4;
  long lVar5;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  puVar2 = param_5;
  if (*(long *)(iVar3 + 0x22) == 0x0) {
    mem_op_1000_179c(0xa,param_5,0x1000);
    puVar2 = (uchar *)((uint)param_5 | param_4);
    if (puVar2 == (uchar *)0x0) {
      *(undefined4 *)(iVar3 + 0x22) = 0x0;
    }
    else {
      pass1_1020_ba3e((long *)CONCAT22(param_5,param_4),0xa,0x2,param_7,param_6);
      *(uint *)(iVar3 + 0x22) = param_4;
      *(uchar **)(iVar3 + 0x24) = puVar2;
    }
  }
  uVar1 = *(undefined4 *)(iVar3 + 0x22);
  lVar5 = pass1_1020_bae6((ushort)uVar1,CONCAT22(param_3,(int)((ulong)uVar1 >> 0x10)),param_4,(uint)puVar2,param_8);
  pass1_1020_bb8a(*(long **)(iVar3 + 0x22),(uint)(lVar5 + param_2),
                  CONCAT22(param_3,(int)((ulong)(lVar5 + param_2) >> 0x10)),param_7,param_8);
  return;
}



void __stdcall16far pass1_1030_7e5a(ulong param_1,ulong param_2,uint param_3)

{
  astruct_358 *iVar1;
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  iVar1 = (astruct_358 *)param_1;
  iVar1->field_0x16 = param_2;
  iVar1->field_0x1a = 0x0;
  pass1_1030_6fa0(param_1 & 0xffff | (ulong)uVar1 << 0x10);
  if (iVar1->field_0x2e != 0x0) {
    pass1_1038_4b20(iVar1->field_0x2e,iVar1->field_0x16,iVar1->field_0x4,param_3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far pass1_1030_7ea0(ulong param_1)

{
  undefined4 uVar1;
  ushort uVar2;
  BOOL16 BVar3;
  
  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0xb);
  if (BVar3 != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    if (*(int *)((int)uVar1 + 0x12) == 0x5) {
      return 0x1;
    }
    BVar3 = 0x0;
  }
  return BVar3;
}



void __stdcall16far pass1_1030_7eda(ulong param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  undefined2 local_c;
  undefined2 uStack10;
  ushort uStack8;
  undefined2 uStack6;
  undefined2 uStack4;
  
  local_c = 0x0;
  uStack10 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack8 = param_2;
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
  }
  pass1_1028_bb96(*(ulong *)((int)param_1 + 0x1a),(ulong *)&local_c,param_3);
  return;
}



void __stdcall16far pass1_1030_7f1a(ulong param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  undefined2 local_c;
  ushort uStack10;
  undefined2 uStack8;
  undefined2 uStack6;
  undefined2 uStack4;
  
  local_c = 0x0;
  uStack8 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack10 = param_2;
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
  }
  pass1_1028_bb96(*(ulong *)((int)param_1 + 0x1a),(ulong *)&local_c,param_3);
  return;
}



ushort __stdcall16far pass1_1030_7f5a(ulong param_1)

{
  uint uVar1;
  ulong uVar2;
  
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
  }
  uVar2 = pass1_1028_bb6a(*(ulong *)((int)param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return *(ushort *)((int)uVar2 + 0x4);
  }
  return 0x0;
}



ushort __stdcall16far pass1_1030_7f98(ulong param_1)

{
  uint uVar1;
  ulong uVar2;
  
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
  }
  uVar2 = pass1_1028_bb6a(*(ulong *)((int)param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return *(ushort *)((int)uVar2 + 0x2);
  }
  return 0x0;
}



void __stdcall16far pass1_1030_7fd6(ulong param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  uint uVar4;
  ulong uVar5;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x1a) == 0x0) {
    uVar5 = struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar4 << 0x10);
    param_2 = (ushort)(uVar5 >> 0x10);
  }
  uVar2 = *(undefined4 *)(iVar3 + 0x1a);
  iVar1 = *(int *)((int)uVar2 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    pass1_1028_1416(*(ulong *)(iVar3 + 0x1a),param_2,param_3);
  }
  return;
}



void __stdcall16far pass1_1030_8030(ulong param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  uint uVar4;
  ulong uVar5;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x1a) == 0x0) {
    uVar5 = struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar4 << 0x10);
    param_2 = (ushort)(uVar5 >> 0x10);
  }
  uVar2 = *(undefined4 *)(iVar3 + 0x1a);
  iVar1 = *(int *)((int)uVar2 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    uVar5 = *(ulong *)(iVar3 + 0x1a);
    pass1_1028_1106(uVar5,(int)uVar5,param_2,param_3);
  }
  return;
}



ulong __stdcall16far pass1_1030_8086(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x18),*(undefined2 *)((int)param_1 + 0x16)) & 0xffffff;
}



void __stdcall16far pass1_1030_809c(ulong param_1)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
  }
  return;
}



ushort * __stdcall16far pass1_1030_80ee(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1030_68dc(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((int *)param_1,(ushort)((ulong)param_1 >> 0x10),0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1030_8128(ulong *param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  uchar *puVar2;
  uchar *puVar3;
  uchar *extraout_DX;
  astruct_135 *iVar4;
  ushort uVar5;
  
  uVar5 = (ushort)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_135 *)param_1;
  uVar1 = 0x0;
  *param_1 = 0x0;
  *(undefined4 *)&iVar4->field_0x4 = 0x0;
  iVar4->field_0x8 = 0x0;
  _PTR_LOOP_1050_5748 = param_1;
  mem_op_1000_179c(0x56,(uchar *)param_2,0x1000);
  puVar2 = (uchar *)(param_2 | uVar1);
  if (puVar2 != (uchar *)0x0) {
    pass1_1028_d81c((ulong *)CONCAT22(param_2,uVar1),(ulong)param_1,puVar2,param_3);
  }
  mem_op_1000_179c(0x8,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | uVar1);
  if (puVar3 == (uchar *)0x0) {
    *param_1 = 0x0;
  }
  else {
    struct_1028_d22e((ulong *)CONCAT22(puVar2,uVar1),(ulong)param_1,(ushort)puVar3);
    *(uint *)param_1 = uVar1;
    iVar4->field_0x2 = puVar3;
  }
  mem_op_1000_179c(0x8,puVar3,0x1000);
  puVar2 = (uchar *)((uint)puVar3 | uVar1);
  if (puVar2 == (uchar *)0x0) {
    *(undefined4 *)&iVar4->field_0x4 = 0x0;
  }
  else {
    pass1_1028_cfd2((ulong *)CONCAT22(puVar3,uVar1),param_1);
    iVar4->field_0x4 = uVar1;
    iVar4->field_0x6 = extraout_DX;
    puVar2 = extraout_DX;
  }
  mem_op_1000_179c(0x24,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | uVar1);
  if (puVar3 != (uchar *)0x0) {
    pass1_1030_5bec(CONCAT22(puVar2,uVar1));
  }
  mem_op_1000_179c(0x8,puVar3,0x1000);
  if ((uchar *)((uint)puVar3 | uVar1) != (uchar *)0x0) {
    pass1_1038_78e2((ulong *)CONCAT22(puVar3,uVar1),(uchar *)((uint)puVar3 | uVar1));
  }
  PTR_LOOP_1050_574a = (undefined *)((ulong)_PTR_LOOP_1050_5748 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_8210(uint *param_1)

{
  uint uVar1;
  uint uVar2;
  astruct_18 *paVar3;
  int iVar4;
  undefined2 uVar5;
  astruct_18 *paStack10;
  astruct_18 *paStack6;
  
  paVar3 = _PTR_LOOP_1050_65e2;
  if (_PTR_LOOP_1050_65e2 != (astruct_18 *)0x0) {
    pass1_1028_daba((ulong)_PTR_LOOP_1050_65e2,(ushort)&USHORT_1050_1028);
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar1 = *param_1;
  uVar2 = *(uint *)(iVar4 + 0x2);
  paStack10 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_d282((uint *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  uVar1 = *(uint *)(iVar4 + 0x4);
  uVar2 = *(uint *)(iVar4 + 0x6);
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_cff2(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5736;
  if (_PTR_LOOP_1050_5736 != (astruct_18 *)0x0) {
    pass1_1030_5c0e();
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5a64;
  if (((uint)PTR_LOOP_1050_5a66 | (uint)_PTR_LOOP_1050_5a64) != 0x0) {
    pass1_1038_7964((uint *)((ulong)_PTR_LOOP_1050_5a64 & 0xffff | ZEXT24(PTR_LOOP_1050_5a66) << 0x10));
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  _PTR_LOOP_1050_5748 = 0x0;
  return;
}



void __stdcall16far pass1_1030_82f0(ushort param_1,ulong param_2,ulong param_3)

{
  pass1_1028_d078(param_1,*(ulong *)((int)param_2 + 0x4),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_8308(ushort param_1,ushort param_2,ushort *param_3,ushort *param_4,ulong param_5,ushort param_6,
               ushort param_7)

{
  pass1_1028_e198(_PTR_LOOP_1050_65e2,param_3,param_4,param_5,param_6,param_7);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1030_8326(void)

{
  return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_8334(void)

{
  *_PTR_LOOP_1050_65e2 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_8344(ushort param_1,ushort param_2,ulong param_3)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_3,(uint)(param_3 >> 0x10));
  return;
}



void __stdcall16far fn_ptr_1030_835a(ulong **param_1,ulong *param_2)

{
  fn_ptr_1028_d566(*param_1,param_2);
  return;
}



void __stdcall16far pass1_1030_8372(ulong **param_1,ulong param_2,ulong *param_3)

{
  pass1_1028_d52c(*param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_838e(ulong *param_1,ushort param_2,uchar param_3)

{
  struct_1028_d2b0((ulong *)*param_1,param_2,param_3);
  pass1_1028_d01a(*(ulong **)((int)param_1 + 0x4));
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x1,(int)&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_83ba(ulong **param_1,long param_2,ushort param_3,uchar param_4)

{
  long lVar1;
  
  while (lVar1 = param_2 + -0x1, param_2 != 0x0) {
    struct_1028_d2b0(*param_1,param_3,param_4);
    pass1_1028_d01a(*(ulong **)((int)param_1 + 0x4));
    param_2 = lVar1;
    if (lVar1 != 0x0) {
      send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x0,(int)&USHORT_1050_1028);
    }
  }
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x1,(int)&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far vsprintf_op_1030_840a(ulong param_1,LPSTR param_2,WORD *param_3,ushort param_4)

{
  LPCSTR pCVar1;
  ushort unaff_ES;
  uchar in_AF;
  WORD *args;
  
  if (PTR_LOOP_1050_574c != (undefined *)0x0) {
    args = param_3;
    if (PTR_LOOP_1050_5750 == (undefined *)0x0) {
      param_2 = (LPSTR)&PTR_LOOP_1050_1000;
      pCVar1 = &stack0x0008;
      pass1_1000_2b3c((ushort)s_simres_out_1050_5758,(ushort)&USHORT_1050_1050,0x5756,(ushort)&USHORT_1050_1050,param_4,
                      (int)&stack0xfffe);
      _PTR_LOOP_1050_5752 = CONCAT22(param_4,pCVar1);
      PTR_LOOP_1050_5750 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
    wvsprintf16(param_2,&stack0x0008,args);
    pass1_1000_2b5c((ushort)_PTR_LOOP_1050_5752,(ushort)((ulong)_PTR_LOOP_1050_5752 >> 0x10),0x5763,
                    (ushort)&USHORT_1050_1050,unaff_ES,(int)&stack0xfffe,0x1000,(ushort)param_3);
    pass1_1000_2f48(_PTR_LOOP_1050_5752,(int)&stack0xfffe,unaff_ES,0x1000,(ushort)param_3,in_AF);
  }
  return;
}



void __stdcall16far pass1_1030_8480(astruct_18 **param_1)

{
  fn_ptr_1000_17ce(*param_1,0x1000);
  return;
}



void __stdcall16far pass1_1030_8496(ulong param_1)

{
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x2),0x1000);
  return;
}



void __stdcall16far pass1_1030_84ae(ulong param_1)

{
  pass1_1008_3e38((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)));
  *(undefined2 *)((int)param_1 + 0x1e) = 0x1;
  return;
}



void __stdcall16far fn_ptr_1030_84d0(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(int *)(iVar4 + 0x1e) != 0x0) {
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe);
    uVar2 = *(uint *)(iVar4 + 0x10);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x12);
    uVar2 = *(uint *)(iVar4 + 0x14);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x4),0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x16),0x1000);
  }
  return;
}



void __stdcall16far struct_1030_8544(ushort *param_1,ushort *param_2)

{
  astruct_356 *iVar1;
  astruct_355 *iVar2;
  undefined2 uVar1;
  undefined2 uVar2;
  
  *param_1 = *param_2;
  uVar1 = (undefined2)((ulong)param_2 >> 0x10);
  iVar1 = (astruct_356 *)param_2;
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_355 *)param_1;
  iVar2->field_0x4 = iVar1->field_0x4;
  pass1_1008_3f62((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x8),
                  (ushort *)((ulong)param_2 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x8));
  iVar2->field_0xe = iVar1->field_0xe;
  iVar2->field_0x12 = iVar1->field_0x12;
  iVar2->field_0x16 = iVar1->field_0x16;
  iVar2->field_0x1a = iVar1->field_0x1a;
  iVar2->field_0x1e = 0x0;
  return;
}



void __stdcall16far pass1_1030_85be(long *param_1,ushort param_2,int param_3,int param_4,ushort param_5)

{
  astruct_357 *iVar1;
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_357 *)param_1;
  *param_1 = 0x0;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xe = 0x0;
  if (iVar1->field_0x6 == 0x0) {
    iVar1->field_0x6 = 0x5;
  }
  pass1_1030_878c(param_1,param_4,param_5);
  return;
}



void __stdcall16far pass1_1030_8604(astruct_18 **param_1)

{
  fn_ptr_1000_17ce(*param_1,0x1000);
  return;
}



void __stdcall16far
pass1_1030_861a(ushort param_1,ushort param_2,ushort param_3,uint param_4,uint param_5,ushort param_6)

{
  undefined4 *puStack6;
  
  pass1_1030_8854(param_1,param_2,param_3,param_6);
  puStack6 = (undefined4 *)CONCAT22(param_5,param_4);
  if ((param_5 | param_4) == 0x0) {
    *(undefined4 *)(param_1 + 0xa) = 0x0;
  }
  else {
    *(undefined4 *)(param_1 + 0xa) = *puStack6;
  }
  return;
}



void __stdcall16far
pass1_1030_8660(ulong param_1,ulong *param_2,ushort param_3,uint param_4,uint param_5,ushort param_6,int param_7)

{
  uint uVar1;
  ushort uVar2;
  ushort uVar3;
  ulong *puStack6;
  
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  pass1_1030_8854(uVar2,uVar3,param_3,param_6);
  puStack6 = (ulong *)CONCAT22(param_5,param_4);
  uVar1 = param_5 | param_4;
  if (uVar1 == 0x0) {
    pass1_1030_8854(uVar2,uVar3,0x0,param_6);
    puStack6 = (ulong *)CONCAT22(uVar1,param_4);
    uVar1 = uVar1 | param_4;
    if (uVar1 == 0x0) {
      pass1_1030_878c((long *)param_1,param_7,param_6);
      pass1_1030_8854(uVar2,uVar3,0x0,param_6);
      puStack6 = (ulong *)CONCAT22(uVar1,param_4);
      if ((uVar1 | param_4) == 0x0) {
        return;
      }
    }
    *(ushort *)((int)puStack6 + 0x4) = param_3;
    *puStack6 = *param_2;
    pass1_1030_8834((uint *)param_1,param_7,param_6);
  }
  else {
    *puStack6 = *param_2;
  }
  return;
}



void __stdcall16far pass1_1030_86ec(astruct_18 **param_1,ushort param_2)

{
  astruct_612 *iVar1;
  undefined2 uVar1;
  
  fn_ptr_1000_17ce(*param_1,0x1000);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_612 *)param_1;
  *param_1 = (astruct_18 *)0x0;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = param_2;
  iVar1->field_0xe = 0x0;
  return;
}



void __stdcall16far pass1_1030_871e(long *param_1,ulong *param_2,ushort param_3,int param_4,ushort param_5)

{
  int *piVar1;
  astruct_681 *iVar2;
  uint uVar2;
  
  uVar2 = (uint)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_681 *)param_1;
  if (*param_1 == 0x0) {
    pass1_1030_878c((long *)((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10),param_4,param_5);
  }
  piVar1 = &iVar2->field_0xe;
  *piVar1 = *piVar1 + 0x1;
  *(ushort *)((int)*param_1 + iVar2->field_0xe * 0x6 + 0x4) = param_3;
  *(ulong *)(iVar2->field_0xe * 0x6 + (int)*param_1) = *param_2;
  return;
}



void __stdcall16far pass1_1030_877c(uint *param_1,int param_2,ushort param_3)

{
  pass1_1030_8834(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_878c(long *param_1,int param_2,ushort param_3)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  astruct_350 *iVar4;
  uint uVar4;
  long lVar5;
  long lStack12;
  
  uVar4 = (uint)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_350 *)param_1;
  if (iVar4->field_0x4 == 0x0) {
    PTR_LOOP_1050_5f2e = (undefined *)0x0;
    uVar2 = iVar4->field_0x6;
  }
  else {
    uVar3 = iVar4->field_0x4;
    puVar1 = &iVar4->field_0x8;
    uVar2 = uVar3 + *puVar1;
    PTR_LOOP_1050_5f2e = (undefined *)(uint)CARRY2(uVar3,*puVar1);
  }
  if (PTR_LOOP_1050_5f2e == (undefined *)0x0) {
    if (*param_1 == 0x0) {
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0,0x1000);
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(uVar2 * 0x6,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
      lVar5 = pass1_1000_0ed4(0x1000,param_3,0x1,uVar2 * 0x6,0x0,(ushort *)*param_1,(ushort)((ulong)*param_1 >> 0x10));
      PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar5 >> 0x10);
      uVar3 = (uint)lVar5;
    }
    lStack12 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if (((uint)PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
      iVar4->field_0x4 = uVar2;
      *param_1 = lStack12;
      pass1_1030_8834((uint *)((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10),param_2,param_3);
    }
  }
  return;
}



void __stdcall16far pass1_1030_8834(uint *param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  ushort uVar2;
  
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x2);
  pass1_1000_4aea(*param_1,(uint)uVar1,(int)((ulong)uVar1 >> 0x10),0x6,(uchar *)0x888e,(int)&stack0xfffe,param_2,uVar2,
                  0x1000,param_3);
  return;
}



void __stdcall16far pass1_1030_8854(ushort param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined4 uVar1;
  undefined4 local_c;
  ushort uStack8;
  
  uStack8 = param_3;
  local_c = 0x0;
  uVar1 = *(undefined4 *)(param_1 + 0x2);
  pass1_1000_49c6((ushort)&local_c,param_4,*_param_1,(uint)uVar1,(uint)((ulong)uVar1 >> 0x10),0x6,(uchar *)0x888e,
                  (int)&stack0xfffe);
  return;
}



ushort __cdecl16far pass1_1030_888e(ulong param_1,ulong param_2)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = *(int *)((int)param_1 + 0x4);
  uVar4 = (undefined2)(param_2 >> 0x10);
  piVar1 = (int *)((int)param_2 + 0x4);
  if (*piVar1 != iVar2 && iVar2 <= *piVar1) {
    return 0xffff;
  }
  if (*(int *)((int)param_2 + 0x4) < *(int *)((int)param_1 + 0x4)) {
    return 0x1;
  }
  return 0x0;
}



void __stdcall16far pass1_1030_88ce(ushort *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  uchar *puVar1;
  uchar *puVar2;
  astruct_354 *iVar4;
  undefined2 uVar3;
  ulong uVar4;
  ushort *puStack38;
  int iStack34;
  undefined local_20 [0x2];
  int local_1e;
  int local_1c;
  undefined local_1a [0x6];
  undefined local_14 [0x6];
  undefined4 uStack14;
  undefined4 uStack10;
  int iStack6;
  undefined2 uStack4;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_354 *)param_1;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  pass1_1030_84ae((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x4);
  iVar4->field_0x24 = param_3;
  puStack38 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x28);
  pass1_1008_6c90((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x28));
  *(undefined4 *)&iVar4->field_0x34 = 0x0;
  *param_1 = 0x8e38;
  iVar4->field_0x2 = 0x1030;
  struct_1030_8544((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x4),(ushort *)param_2);
  uVar4 = pass1_1008_4772(iVar4->field_0x12);
  uStack4 = (undefined2)(uVar4 >> 0x10);
  iStack6 = (int)uVar4;
  uStack10 = *(undefined4 *)(iStack6 + 0x4);
  uStack14 = *(undefined4 *)(iStack6 + 0x8);
  pass1_1008_3e54((ushort *)CONCAT22(param_4,local_14),0x0,(int)uStack14 - 0x1,(int)uStack10 - 0x1);
  pass1_1008_3e54((ushort *)CONCAT22(param_4,local_1a),0x0,0x0,0x0);
  pass1_1008_6d18(puStack38,(ushort *)CONCAT22(param_4,local_14),(ushort *)CONCAT22(param_4,local_1a));
  pass1_1008_6d64(puStack38,(ushort *)CONCAT22(param_4,local_1a));
  pass1_1008_3eb4((ushort *)CONCAT22(param_4,local_1a),(ushort *)CONCAT22(param_4,local_20),
                  (ushort *)CONCAT22(param_4,&local_1e),(ushort *)CONCAT22(param_4,&local_1c));
  puVar1 = (uchar *)((ulong)((long)local_1e * (long)local_1c) >> 0x10);
  uVar4 = (long)local_1e * (long)local_1c & 0xffff;
  iVar4->field_0x34 = (int)uVar4;
  iVar4->field_0x36 = puVar1;
  for (iStack34 = 0x0; iStack34 < 0x5; iStack34 = iStack34 + 0x1) {
    mem_op_1000_179c(0x10,puVar1,0x1000);
    puVar2 = (uchar *)((uint)puVar1 | (uint)uVar4);
    if (puVar2 == (uchar *)0x0) {
      *(undefined4 *)(&iVar4[0x1].field_0x0 + iStack34 * 0x4) = 0x0;
    }
    else {
      pass1_1030_85be((long *)(uVar4 & 0xffff | ZEXT24(puVar1) << 0x10),0x19,0x64,uVar3,param_4);
      *(undefined2 *)(&iVar4[0x1].field_0x0 + iStack34 * 0x4) = (int)uVar4;
      (&iVar4[0x1].field_0x2)[iStack34 * 0x2] = puVar2;
    }
    puVar1 = puVar2;
  }
  return;
}
