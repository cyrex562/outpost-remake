


void __stdcall16far pass1_1030_6a2c(ulong param_1,long param_2,uint param_3,uchar *param_4,ushort param_5)

{
  code **ppcVar1;
  astruct_384 *iVar2;
  undefined2 uVar2;
  astruct_382 *iVar4;
  astruct_383 *iVar5;
  undefined2 uVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  long lVar6;
  undefined local_a [0x8];
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_382 *)param_1;
  if (iVar4->field_0x3e == (undefined4 *)0x0) {
    mem_op_1000_179c(0xc,param_4,0x1000);
    if (((uint)param_4 | param_3) == 0x0) {
      iVar4->field_0x3e = (undefined4 *)0x0;
    }
    else {
      uVar5 = set_struct_1008_574a((astruct_21 *)CONCAT22(param_4,param_3));
      *(int *)&iVar4->field_0x3e = (int)uVar5;
      *(undefined2 *)((int)&iVar4->field_0x3e + 0x2) = (int)((ulong)uVar5 >> 0x10);
    }
  }
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_a),(ulong)iVar4->field_0x3e);
  do {
    do {
      lVar6 = pass1_1008_5b12(local_a,param_5);
      uVar2 = (undefined2)((ulong)lVar6 >> 0x10);
      iVar2 = (astruct_384 *)lVar6;
      if (lVar6 == 0x0) goto LAB_1030_6af4;
      uVar4 = (undefined2)((ulong)param_2 >> 0x10);
      iVar5 = (astruct_383 *)param_2;
    } while ((iVar2->field_0x6 != iVar5->field_0x6) || (iVar2->field_0x4 != iVar5->field_0x4));
  } while (iVar2->field_0x8 != iVar5->field_0x8);
  iVar2->field_0xa = iVar2->field_0xa + iVar5->field_0xa;
  iVar2->field_0xc = iVar2->field_0xc + iVar5->field_0xc;
  param_2 = 0x0;
LAB_1030_6af4:
  if (param_2 != 0x0) {
    ppcVar1 = (code **)((int)*iVar4->field_0x3e + 0x8);
    (**ppcVar1)(0x1008,iVar4->field_0x3e,param_2);
  }
  return;
}



ulong __stdcall16far pass1_1030_6b16(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  astruct_412 *iVar5;
  undefined2 uVar5;
  ulong uVar6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_412 *)param_1;
  if (*(long *)&iVar5->field_0x3a == 0x0) {
    return 0x0;
  }
  ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)&iVar5->field_0x3a + 0x10);
  uVar6 = (**ppcVar3)();
  uVar4 = *(undefined4 *)&iVar5->field_0x3a;
  if (*(int *)((int)uVar4 + 0x8) == 0x0) {
    puVar1 = (undefined4 *)*(uint *)&iVar5->field_0x3a;
    uVar2 = iVar5->field_0x3c;
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    *(undefined4 *)&iVar5->field_0x3a = 0x0;
  }
  return uVar6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_6b86(ulong param_1,undefined2 param_2,undefined2 param_3)

{
  code **ppcVar1;
  ulong uVar2;
  undefined2 extraout_DX;
  undefined2 uVar3;
  uint extraout_DX_00;
  int iVar4;
  undefined2 uVar5;
  ulong uStack12;
  ulong uStack8;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x1e) == 0x0) {
    param_2 = 0x0;
    uVar3 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x1e) + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack8 = CONCAT22(uVar3,param_2);
  for (uStack12 = 0x0; uStack12 < uStack8; uStack12 = uStack12 + 0x1) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x1e) + 0x4);
    uVar2 = uStack8;
    (**ppcVar1)(param_3,*(undefined4 *)(iVar4 + 0x1e));
    if ((extraout_DX_00 | (uint)uVar2) != 0x0) {
      param_3 = SUB42(&USHORT_1050_1028,0x0);
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uint)uVar2,extraout_DX_00);
    }
  }
  return;
}



void __stdcall16far pass1_1030_6c1a(ulong param_1,int param_2)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = *(int *)(iVar3 + 0x32);
  *(int *)(iVar3 + 0x32) = param_2;
  piVar1 = (int *)(iVar3 + 0x34);
  *piVar1 = *piVar1 + (param_2 - iVar2);
  iVar2 = *(int *)(iVar3 + 0x32);
  if (iVar2 < 0x0) {
    iVar2 = 0x0;
  }
  *(int *)(iVar3 + 0x32) = iVar2;
  return;
}



void __stdcall16far pass1_1030_6c4c(ulong param_1,int param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = *(int *)((int)param_1 + 0x32);
  if (param_2 < iVar1) {
    iVar1 = param_2;
  }
  *(int *)((int)param_1 + 0x34) = iVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_6c66(ulong param_1,int param_2,ulong param_3,uint param_4,uchar *param_5,ushort param_6)

{
  uint uVar1;
  code **ppcVar2;
  uint uVar3;
  ushort uVar4;
  BOOL16 BVar5;
  undefined2 extraout_DX;
  uchar *puVar6;
  astruct_386 *iVar7;
  astruct_385 *iVar6;
  ushort unaff_SI;
  ushort unaff_DI;
  undefined2 uVar7;
  undefined2 uVar8;
  ushort unaff_SS;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar7 = (astruct_386 *)param_1;
  if (iVar7->field_0x3a == (undefined4 *)0x0) {
    param_6 = 0x1000;
    mem_op_1000_179c(0xc,param_5,0x1000);
    if (((uint)param_5 | param_4) == 0x0) {
      iVar7->field_0x3a = (undefined4 *)0x0;
    }
    else {
      param_6 = 0x1008;
      set_struct_1008_574a((astruct_21 *)CONCAT22(param_5,param_4));
      *(uint *)&iVar7->field_0x3a = param_4;
      *(undefined2 *)((int)&iVar7->field_0x3a + 0x2) = extraout_DX;
    }
  }
  ppcVar2 = (code **)((int)*iVar7->field_0x3a + 0x8);
  (**ppcVar2)(param_6,iVar7->field_0x3a,param_3);
  if (param_2 != 0x0) {
    uVar8 = (undefined2)(param_3 >> 0x10);
    iVar6 = (astruct_385 *)param_3;
    if (iVar6->field_0x6 != 0x0) {
      pass1_1030_6e9c(param_1,(ulong)iVar6->field_0xa,iVar6->field_0x6);
      return;
    }
    if (iVar6->field_0x4 != 0x0) {
      uVar1 = iVar6->field_0xa;
      uVar3 = -uVar1;
      puVar6 = (uchar *)-(uint)(uVar1 != 0x0);
      pass1_1030_7ddc(param_1,CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12((char)puVar6,uVar3)),iVar6->field_0x4,uVar3,
                      puVar6,unaff_SI,unaff_DI,unaff_SS);
      return;
    }
    if (iVar6->field_0x8 != 0x0) {
      uVar4 = pass1_1030_6fa0(param_1);
      BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar4,0x4);
      if (BVar5 != 0x0) {
        pass1_1028_6356(iVar7->field_0x1a,0x0,iVar6->field_0xa,0x0,unaff_SS);
      }
    }
  }
  return;
}



ulong __stdcall16far pass1_1030_6d4e(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined2 uVar1;
  ushort uStack6;
  ushort uStack4;
  
  uStack6 = 0x0;
  uStack4 = 0x0;
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x36) != 0x0) {
    pass1_1010_9092(*(ulong *)((int)param_1 + 0x36),param_2,param_4);
    uStack6 = param_2;
    uStack4 = param_3;
  }
  return CONCAT22(uStack4,uStack6);
}



void __stdcall16far pass1_1030_6d80(ulong param_1,ulong param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_299 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_299 *)param_1;
  puVar1 = *(undefined4 **)&iVar4->field_0x36;
  uVar2 = *(uint *)((int)&iVar4->field_0x36 + 0x2);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4->field_0x36 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1030_6db4(uchar *param_1,int param_2,ushort param_3)

{
  ushort *puVar1;
  
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,param_1,param_2);
  pass1_1010_ed3e((ulong)puVar1);
  return *(ushort *)((int)puVar1 + 0x18);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_6ddc(ulong param_1)

{
  ushort uVar1;
  BOOL16 BVar2;
  
  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1e);
  if (BVar2 != 0x0) {
    pass1_1030_d0c6(*(ulong *)((int)param_1 + 0x1a));
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_6e14(ulong param_1)

{
  undefined4 uVar1;
  ushort uVar2;
  BOOL16 BVar3;
  
  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1e);
  if (BVar3 != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    pass1_1030_d102((int)uVar1,(ushort)((ulong)uVar1 >> 0x10));
    return;
  }
  return;
}



void __stdcall16far pass1_1030_6e4c(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar3 << 0x10);
  }
  if ((*(long *)(iVar2 + 0x1a) != 0x0) && (uVar1 = *(undefined4 *)(iVar2 + 0x1a), *(int *)((int)uVar1 + 0x12) == 0x4)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_6e9c(ulong param_1,long param_2,int param_3)

{
  code **ppcVar1;
  uint uVar2;
  uint uVar3;
  ulong uVar4;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar5;
  astruct_301 *iVar6;
  undefined2 uVar6;
  ushort unaff_SS;
  ulong uStack10;
  ulong uStack6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_301 *)param_1;
  uVar2 = *(uint *)((int)&iVar6->field_0x1e + 0x2) | *(uint *)&iVar6->field_0x1e;
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((int)*iVar6->field_0x1e + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,uVar2);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
      ppcVar1 = (code **)((int)*iVar6->field_0x1e + 0x4);
      uVar4 = uStack6;
      (**ppcVar1)();
      uVar2 = (uint)uVar4;
      uVar5 = extraout_DX_00 | uVar2;
      if (uVar5 != 0x0) {
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
        if (*(int *)(uVar3 + 0xc) == param_3) {
          param_2 = param_2 + -0x1;
          pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00,unaff_SS);
          ppcVar1 = (code **)((int)*iVar6->field_0x1e + 0x8);
          (**ppcVar1)((int)&USHORT_1050_1028,iVar6->field_0x1e,0x0,uStack10);
        }
        if ((param_2._2_2_ | (uint)param_2) == 0x0) {
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_6f5a(ulong param_1,ushort param_2)

{
  ushort uVar1;
  BOOL16 BVar2;
  
  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x4);
  if (BVar2 != 0x0) {
    pass1_1028_6302(*(ulong *)((int)param_1 + 0x1a),param_2);
  }
  return;
}



ushort __stdcall16far pass1_1030_6fa0(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar3 << 0x10);
  }
  if (*(long *)(iVar2 + 0x1a) != 0x0) {
    uVar1 = *(undefined4 *)(iVar2 + 0x1a);
    return *(ushort *)((int)uVar1 + 0xc);
  }
  return 0x0;
}



void __stdcall16far pass1_1030_6fd4(ulong param_1)

{
  undefined4 uVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
  if (*(int *)((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



void __stdcall16far pass1_1030_701c(ulong param_1)

{
  undefined4 uVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
  if (*(int *)((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



void __stdcall16far pass1_1030_7064(ulong param_1)

{
  undefined4 uVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
  if (*(int *)((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



void __stdcall16far pass1_1030_70ac(ulong param_1)

{
  undefined4 uVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
  if (*(int *)((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_70f4(ulong param_1)

{
  undefined2 uVar1;
  undefined4 uVar2;
  BOOL16 BVar3;
  int iVar4;
  uint uVar5;
  long *plVar6;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar5 << 0x10);
  }
  uVar2 = *(undefined4 *)(iVar4 + 0x1a);
  uVar1 = *(undefined2 *)((int)uVar2 + 0xc);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x2);
    if ((BVar3 == 0x0) || (*(long *)(iVar4 + 0x22) == 0x0)) {
      return;
    }
    plVar6 = *(long **)(iVar4 + 0x22);
  }
  else {
    uVar2 = *(undefined4 *)(iVar4 + 0x1a);
    plVar6 = *(long **)((int)uVar2 + 0x28);
  }
  pass1_1020_ba94(plVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_7176(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  long local_1a;
  int local_16 [0x2];
  uint uStack18;
  uint uStack14;
  BOOL16 BStack10;
  undefined2 uStack8;
  long lStack6;
  
  lStack6 = 0x0;
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x22) == 0x0) {
    return;
  }
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1);
  }
  uVar1 = *(undefined4 *)(iVar2 + 0x1a);
  uStack8 = *(undefined2 *)((int)uVar1 + 0xc);
  BStack10 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x3);
  if ((BStack10 != 0x0) && (uVar1 = *(undefined4 *)(iVar2 + 0x1a), *(int *)((int)uVar1 + 0x12) == 0x5)) {
    uVar1 = *(undefined4 *)(iVar2 + 0x22);
    uStack14 = *(uint *)((int)uVar1 + 0x4);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1) {
      pass1_1020_bb16(*(ulong **)(iVar2 + 0x22),(ulong *)CONCAT22(param_2,&local_1a),
                      (ushort *)CONCAT22(param_2,local_16),uStack18);
      if (0x0 < local_16[0]) {
        lStack6 = lStack6 + local_1a;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_7226(ulong param_1)

{
  ulong uVar1;
  undefined4 uVar2;
  BOOL16 BVar3;
  int iVar4;
  uint uVar5;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar5 << 0x10);
  }
  uVar2 = *(undefined4 *)(iVar4 + 0x1a);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar2 + 0xc),0x10);
  if (((BVar3 != 0x0) && (uVar2 = *(undefined4 *)(iVar4 + 0x1a), *(int *)((int)uVar2 + 0x12) == 0x5)) &&
     (uVar1 = *(ulong *)(iVar4 + 0x1a), uVar2 = *(undefined4 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x14)),
     *(int *)((int)uVar2 + 0xa4) == 0x1e)) {
    return;
  }
  return;
}



void __stdcall16far fn_ptr_1030_7296(ulong param_1)

{
  uint uVar1;
  uint uVar2;
  astruct_292 *iVar3;
  undefined2 uVar3;
  astruct_18 *paStack6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_292 *)param_1;
  uVar1 = iVar3->field_0x22;
  uVar2 = iVar3->field_0x24;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  *(undefined4 *)&iVar3->field_0x22 = 0x0;
  return;
}



void __stdcall16far pass1_1030_72d0(ulong param_1)

{
  uint uVar1;
  uint uVar2;
  astruct_605 *iVar3;
  undefined2 uVar3;
  astruct_18 *paStack6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_605 *)param_1;
  uVar1 = iVar3->field_0x26;
  uVar2 = iVar3->field_0x28;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  *(undefined4 *)&iVar3->field_0x26 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_730a(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  ulong uVar4;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  astruct_290 *iVar5;
  undefined2 uVar5;
  undefined4 *puVar6;
  ulong uStack10;
  ulong uStack6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_290 *)param_1;
  if (iVar5->field_0x1e != (undefined4 *)0x0) {
    puVar6 = iVar5->field_0x1e;
    ppcVar3 = (code **)((int)*iVar5->field_0x1e + 0x10);
    (**ppcVar3)();
    uStack6 = CONCAT22(extraout_DX,param_2);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
      ppcVar3 = (code **)((int)*iVar5->field_0x1e + 0x4);
      uVar4 = uStack6;
      (**ppcVar3)(param_3);
      if ((extraout_DX_00 | (uint)uVar4) != 0x0) {
        param_3 = (ushort)&USHORT_1050_1028;
        pass1_1028_e332(_PTR_LOOP_1050_65e2,(uint)uVar4,extraout_DX_00,param_4);
      }
    }
                    // WARNING: Load size is inaccurate
    puVar1 = iVar5->field_0x1e;
    uVar2 = *(uint *)((int)&iVar5->field_0x1e + 0x2);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)(param_3,puVar1,uVar2,0x1,puVar6);
    }
    iVar5->field_0x1e = (undefined4 *)0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far struct_op_1030_73a8(ulong param_1)

{
  undefined4 uVar1;
  undefined2 in_AX;
  undefined2 in_DX;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    *(undefined2 *)(iVar2 + 0x1a) = in_AX;
    *(undefined2 *)(iVar2 + 0x1c) = in_DX;
  }
  return CONCAT22(*(undefined2 *)(iVar2 + 0x1c),*(undefined2 *)(iVar2 + 0x1a));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_73ee(ulong param_1,ulong param_2,ushort param_3)

{
  astruct_294 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  iVar1 = (astruct_294 *)param_1;
  iVar1->field_0x2a = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_2,(uint)(param_2 >> 0x10));
  iVar1->field_0x2e = (int)param_2;
  iVar1->field_0x30 = param_3;
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1030_7418(ulong param_1,ulong param_2,int param_3,ushort param_4)

{
  ulong uVar1;
  astruct_731 *iVar2;
  int iVar3;
  BOOL16 BVar4;
  undefined *puVar5;
  uint extraout_DX;
  uint extraout_DX_00;
  undefined2 uVar6;
  ushort uVar7;
  ushort uVar8;
  undefined2 uStack62;
  undefined2 local_2a [0x2];
  undefined local_26 [0xe];
  ulong local_18;
  ulong local_14 [0x2];
  undefined2 local_c;
  undefined4 local_a;
  undefined2 local_6 [0x2];
  
  pass1_1030_16d6(param_1,param_2,param_4);
  if (param_3 == 0x0) {
    return;
  }
  iVar2 = (astruct_731 *)param_1;
  iVar2 = (astruct_731 *)&iVar2->field_0xc;
  iVar3 = write_to_file_1008_7b4c(param_2,param_1 & 0xffff0000 | ZEXT24(iVar2),0x1008,param_4);
  if (iVar3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  uVar6 = (undefined2)(param_1 >> 0x10);
  local_c = iVar2->field_0x12;
  uVar7 = (ushort)param_2;
  uVar8 = (ushort)(param_2 >> 0x10);
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_c,param_4,(char *)0x2,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  local_6[0] = iVar2->field_0x14;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  local_18 = iVar2->field_0x16;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_18,param_4,(char *)0x4,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  write_to_file_1008_7954(param_2,iVar2->field_0x1e,BVar4,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  write_to_file_1008_7a22(param_2,iVar2->field_0x22,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  write_to_file_1008_7a22(param_2,iVar2->field_0x26,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  local_a = iVar2->field_0x2a;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_a,param_4,(char *)0x4,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  local_c = iVar2->field_0x32;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_c,param_4,(char *)0x2,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  local_c = iVar2->field_0x34;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_c,param_4,(char *)0x2,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  pass1_1008_79f0(param_2,iVar2->field_0x36,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  if (iVar2->field_0x3a == 0x0) {
    local_18 = local_18 & 0xffff0000;
  }
  else {
    uVar1 = iVar2->field_0x3a;
    local_18 = local_18 & 0xffff0000 | (ulong)*(uint *)((int)uVar1 + 0x8);
  }
  local_6[0] = (undefined2)local_18;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_26),iVar2->field_0x3a);
  while( true ) {
    puVar5 = local_26;
    pass1_1008_5b12(puVar5,param_4);
    local_14[0] = CONCAT22(extraout_DX,puVar5);
    if ((extraout_DX | (uint)puVar5) == 0x0) {
      if (iVar2->field_0x3e == 0x0) {
        uStack62 = 0x0;
      }
      else {
        uVar1 = iVar2->field_0x3e;
        uStack62 = *(undefined2 *)((int)uVar1 + 0x8);
      }
      local_2a[0] = uStack62;
      BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_2a,param_4,(char *)0x2,0x1008);
      if (BVar4 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
      }
      pass1_1008_5784((ulong *)CONCAT22(param_4,local_26),iVar2->field_0x3e);
      while( true ) {
        puVar5 = local_26;
        pass1_1008_5b12(puVar5,param_4);
        if ((extraout_DX_00 | (uint)puVar5) == 0x0) {
          return;
        }
        local_18 = local_18 & 0xffff0000 | (ulong)*(uint *)(puVar5 + 0x4);
        BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_18,param_4,(char *)0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = (undefined *)0x6d0;
          return;
        }
        local_14[0] = local_14[0] & 0xffff0000 | (ulong)*(uint *)(puVar5 + 0x6);
        BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_14,param_4,(char *)0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = (undefined *)0x6d0;
          return;
        }
        local_c = *(undefined2 *)(puVar5 + 0x8);
        BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_c,param_4,(char *)0x2,0x1008);
        if (BVar4 == 0x0) break;
        local_c = *(undefined2 *)(puVar5 + 0xa);
        BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_c,param_4,(char *)0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = (undefined *)0x6d0;
          return;
        }
        local_6[0] = *(undefined2 *)(puVar5 + 0xc);
        BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = (undefined *)0x6d0;
          return;
        }
      }
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
    local_6[0] = *(undefined2 *)(puVar5 + 0x4);
    BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
    local_6[0] = *(undefined2 *)((int)local_14[0] + 0x6);
    BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
    if (BVar4 == 0x0) break;
    local_6[0] = *(undefined2 *)((int)local_14[0] + 0x8);
    BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
    local_6[0] = *(undefined2 *)((int)local_14[0] + 0xa);
    BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
    local_6[0] = *(undefined2 *)((int)local_14[0] + 0xc);
    BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1030_778c(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  long lVar1;
  code **ppcVar2;
  astruct_387 *iVar3;
  BOOL16 BVar3;
  int iVar6;
  long *plVar7;
  ulong *puVar8;
  undefined2 extraout_DX;
  uint uVar9;
  uchar *puVar10;
  undefined2 extraout_DX_00;
  astruct_389 *iVar4;
  astruct_391 *iVar5;
  ushort uVar11;
  undefined2 uVar12;
  ushort uVar13;
  ushort uVar14;
  uint local_56 [0x2];
  uint uStack82;
  astruct_99 *paStack74;
  undefined2 local_46 [0x2];
  undefined2 local_42 [0x2];
  ulong local_3e [0x3];
  astruct_99 *paStack50;
  uint local_2e;
  astruct_99 *paStack44;
  undefined2 local_28 [0x2];
  undefined2 local_24 [0x2];
  uint local_20 [0x9];
  uint uStack14;
  undefined2 local_4;
  astruct_388 *uVar5;
  astruct_390 *uVar8;
  
  file_1030_1730(param_1,param_2);
  if (param_3 != 0x0) {
    iVar3 = (astruct_387 *)param_1;
    iVar3 = (astruct_387 *)&iVar3->field_0xc;
    BVar3 = read_file_1008_7bc8(param_2,(ushort *)(param_1 & 0xffff0000 | ZEXT24(iVar3)),0x1008,param_5);
    if (BVar3 != 0x0) {
      uVar13 = (ushort)param_2;
      uVar14 = (ushort)(param_2 >> 0x10);
      BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)&local_4,0x0,param_5,0x2,0x1008);
      if (BVar3 != 0x0) {
        uVar11 = (ushort)(param_1 >> 0x10);
        iVar3->field_0x12 = local_4;
        BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)&local_4,0x0,param_5,0x2,0x1008);
        if (BVar3 != 0x0) {
          iVar3->field_0x14 = local_4;
          BVar3 = read_file_1008_7dee(uVar13,uVar14,&iVar3->field_0x16,0x0,uVar11,0x4,0x1008);
          if (BVar3 != 0x0) {
            plVar7 = (long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1e);
            file_1008_76e4(param_2,plVar7,0x1008,param_5,(ushort)param_4);
            if (((((int)plVar7 != 0x0) &&
                 (iVar6 = file_1008_77cc(param_2,(long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x22),
                                         param_4,0x1008,param_5), iVar6 != 0x0)) &&
                (iVar6 = file_1008_77cc(param_2,(long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x26),param_4
                                        ,0x1008,param_5), iVar6 != 0x0)) &&
               (BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)&iVar3->field_0x2a,0x0,uVar11,0x4,0x1008),
               BVar3 != 0x0)) {
              if (iVar3->field_0x2a != 0x0) {
                lVar1 = iVar3->field_0x2a;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)lVar1,(uint)((ulong)lVar1 >> 0x10));
                iVar3->field_0x2e = BVar3;
                iVar3->field_0x30 = param_4;
              }
              if ((int)PTR_LOOP_1050_0312 < 0x2) {
                return;
              }
              BVar3 = read_file_1008_7dee(uVar13,uVar14,&iVar3->field_0x32,0x0,uVar11,0x2,0x1008);
              if ((BVar3 != 0x0) &&
                 (BVar3 = read_file_1008_7dee(uVar13,uVar14,&iVar3->field_0x34,0x0,uVar11,0x2,0x1008), BVar3 != 0x0)) {
                puVar8 = (ulong *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x36);
                pass1_1008_766e(param_2,puVar8,param_5,0x1008,param_4);
                if (((int)puVar8 != 0x0) &&
                   (BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)local_20,0x0,param_5,0x2,0x1008), BVar3 != 0x0)) {
                  for (uStack14 = 0x0; uStack14 < local_20[0]; uStack14 = uStack14 + 0x1) {
                    local_3e[0] = _PTR_LOOP_1050_68a2;
                    paStack50 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
                    uVar9 = (uint)((ulong)paStack50 >> 0x10);
                    uVar5 = (astruct_388 *)paStack50;
                    puVar10 = (uchar *)(uVar9 | (uint)uVar5);
                    if (puVar10 == (uchar *)0x0) {
                      paStack44 = (astruct_99 *)0x0;
                    }
                    else {
                      paStack50->field_0x0 = 0x389a;
                      uVar5->field_0x2 = 0x1008;
                      uVar5->field_0x4 = 0x0;
                      uVar5->field_0x6 = 0x0;
                      uVar5->field_0x8 = 0x0;
                      uVar5->field_0xa = 0x0;
                      uVar5->field_0xc = 0x0;
                      paStack50->field_0x0 = 0x56ce;
                      uVar5->field_0x2 = 0x1018;
                      paStack44 = paStack50;
                    }
                    BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)local_28,0x0,param_5,0x2,0x1008);
                    if (((BVar3 == 0x0) ||
                        (BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)local_24,0x0,param_5,0x2,0x1008),
                        BVar3 == 0x0)) ||
                       ((BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)&local_2e,0x0,param_5,0x2,0x1008),
                        BVar3 == 0x0 ||
                        ((BVar3 = read_file_1008_7dee(uVar13,uVar14,(int)paStack44 + 0xa,0x0,
                                                      (ushort)((ulong)paStack44 >> 0x10),0x2,0x1008), BVar3 == 0x0 ||
                         (BVar3 = read_file_1008_7dee(uVar13,uVar14,(int)paStack44 + 0xc,0x0,
                                                      (ushort)((ulong)paStack44 >> 0x10),0x2,0x1008), BVar3 == 0x0))))))
                    goto LAB_1030_77be;
                    uVar12 = (undefined2)((ulong)paStack44 >> 0x10);
                    iVar4 = (astruct_389 *)paStack44;
                    iVar4->field_0x4 = local_28[0];
                    iVar4->field_0x6 = local_24[0];
                    iVar4->field_0x8 = local_2e;
                    if (iVar3->field_0x3a == (undefined4 *)0x0) {
                      uVar9 = local_2e;
                      mem_op_1000_179c(0xc,puVar10,0x1000);
                      paStack50 = (astruct_99 *)CONCAT22(puVar10,uVar9);
                      if (((uint)puVar10 | uVar9) == 0x0) {
                        iVar3->field_0x3a = (undefined4 *)0x0;
                      }
                      else {
                        set_struct_1008_574a((astruct_21 *)CONCAT22(puVar10,uVar9));
                        *(uint *)&iVar3->field_0x3a = uVar9;
                        *(undefined2 *)((int)&iVar3->field_0x3a + 0x2) = extraout_DX;
                      }
                    }
                    ppcVar2 = (code **)((int)*iVar3->field_0x3a + 0x8);
                    (**ppcVar2)();
                  }
                  BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)local_56,0x0,param_5,0x2,0x1008);
                  if (BVar3 != 0x0) {
                    uStack82 = 0x0;
                    while( true ) {
                      if (local_56[0] <= uStack82) {
                        return;
                      }
                      paStack44 = (astruct_99 *)_PTR_LOOP_1050_68a2;
                      paStack50 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
                      uVar9 = (uint)((ulong)paStack50 >> 0x10);
                      uVar8 = (astruct_390 *)paStack50;
                      puVar10 = (uchar *)(uVar9 | (uint)uVar8);
                      if (puVar10 == (uchar *)0x0) {
                        paStack74 = (astruct_99 *)0x0;
                      }
                      else {
                        paStack50->field_0x0 = 0x389a;
                        uVar8->field_0x2 = 0x1008;
                        uVar8->field_0x4 = 0x0;
                        uVar8->field_0x6 = 0x0;
                        uVar8->field_0x8 = 0x0;
                        uVar8->field_0xa = 0x0;
                        uVar8->field_0xc = 0x0;
                        paStack50->field_0x0 = 0x56ce;
                        uVar8->field_0x2 = 0x1018;
                        paStack74 = paStack50;
                      }
                      BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)local_46,0x0,param_5,0x2,0x1008);
                      if ((((BVar3 == 0x0) ||
                           (BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)local_42,0x0,param_5,0x2,0x1008),
                           BVar3 == 0x0)) ||
                          (BVar3 = read_file_1008_7dee(uVar13,uVar14,(ushort)local_3e,0x0,param_5,0x2,0x1008),
                          BVar3 == 0x0)) ||
                         ((BVar3 = read_file_1008_7dee(uVar13,uVar14,(int)paStack74 + 0xa,0x0,
                                                       (ushort)((ulong)paStack74 >> 0x10),0x2,0x1008), BVar3 == 0x0 ||
                          (BVar3 = read_file_1008_7dee(uVar13,uVar14,(int)paStack74 + 0xc,0x0,
                                                       (ushort)((ulong)paStack74 >> 0x10),0x2,0x1008), BVar3 == 0x0))))
                      break;
                      uVar12 = (undefined2)((ulong)paStack74 >> 0x10);
                      iVar5 = (astruct_391 *)paStack74;
                      iVar5->field_0x4 = local_46[0];
                      iVar5->field_0x6 = local_42[0];
                      iVar5->field_0x8 = (uint)local_3e[0];
                      if (iVar3->field_0x3e == (undefined4 *)0x0) {
                        mem_op_1000_179c(0xc,puVar10,0x1000);
                        paStack50 = (astruct_99 *)CONCAT22(puVar10,(uint)local_3e[0]);
                        if (((uint)puVar10 | (uint)local_3e[0]) == 0x0) {
                          iVar3->field_0x3e = (undefined4 *)0x0;
                        }
                        else {
                          set_struct_1008_574a((astruct_21 *)CONCAT22(puVar10,(uint)local_3e[0]));
                          *(uint *)&iVar3->field_0x3e = (uint)local_3e[0];
                          *(undefined2 *)((int)&iVar3->field_0x3e + 0x2) = extraout_DX_00;
                        }
                      }
                      ppcVar2 = (code **)((int)*iVar3->field_0x3e + 0x8);
                      (**ppcVar2)();
                      uStack82 = uStack82 + 0x1;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
LAB_1030_77be:
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}
