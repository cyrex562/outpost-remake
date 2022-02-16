
// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_35a4(ulong *param_1,ulong param_2,uchar *param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  ulong *puVar4;
  uint uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  undefined2 uVar7;
  ushort unaff_SS;
  ulong uStack12;
  ulong *puStack8;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  uVar2 = *(undefined4 *)((int)param_1 + 0x56);
  uVar2 = *(undefined4 *)((int)uVar2 + 0x8);
  puStack8 = *(ulong **)((int)uVar2 + *(int *)((int)param_1 + 0x5a) * 0x4);
  uStack12 = param_2;
  if (param_2 != 0x0) {
    uVar7 = 0x1000;
    mem_op_1000_179c(0x4a,param_3,0x1000);
    uVar3 = (uint)param_2;
    uVar5 = (uint)param_3 | uVar3;
    if (uVar5 == 0x0) {
      uVar3 = 0x0;
      uVar5 = 0x0;
    }
    else {
      uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
      pass1_1040_c54a((ushort *)(param_2 & 0xffff | ZEXT24(param_3) << 0x10),0x1,puStack8,(ushort)&PTR_LOOP_1050_1040,
                      unaff_SS);
    }
    ppcVar1 = (code **)((int)*param_1 + 0x18);
    (**ppcVar1)(uVar7,param_1,0x1,uVar3,uVar5);
    puVar6 = extraout_DX;
    for (; (uStack12 & 0xf) != 0x0; uStack12 = uStack12 >> 0x4) {
      uVar2 = *(undefined4 *)((int)puStack8 + 0x8);
      puStack8 = *(ulong **)((((byte)uStack12 & 0xf) - 0x1) * 0x4 + (int)uVar2);
      uVar7 = 0x1000;
      puVar4 = puStack8;
      mem_op_1000_179c(0x4a,puVar6,0x1000);
      uVar3 = (uint)puVar4;
      uVar5 = (uint)puVar6 | uVar3;
      if (uVar5 == 0x0) {
        uVar3 = 0x0;
        uVar5 = 0x0;
      }
      else {
        uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
        pass1_1040_c54a((ushort *)((ulong)puVar4 & 0xffff | ZEXT24(puVar6) << 0x10),0x1,puStack8,
                        (ushort)&PTR_LOOP_1050_1040,unaff_SS);
      }
      ppcVar1 = (code **)((int)*param_1 + 0x18);
      (**ppcVar1)(uVar7,param_1,0x1,uVar3,uVar5);
      puVar6 = extraout_DX_00;
    }
  }
  return;
}



void __stdcall16far
pass1_1010_3680(ushort param_1,ushort param_2,ushort param_3,ushort param_4,uint param_5,uchar *param_6,ushort param_7)

{
  mem_op_1000_179c(0x4a,param_6,0x1000);
  if (((uint)param_6 | param_5) != 0x0) {
    pass1_1040_c54a((ushort *)CONCAT22(param_6,param_5),0x1,(ulong *)CONCAT22(param_4,param_3),
                    (ushort)&PTR_LOOP_1050_1040,param_7);
    return;
  }
  return;
}



ushort * __stdcall16far pass1_1010_36b4(ushort *param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1010_2db2(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_3702(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)(param_1 + 0xa) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0x37c4;
  *(undefined2 *)(param_1 + 0x2) = 0x1010;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1010_3730(ushort *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x37c4;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0xa),0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



ulong __stdcall16far pass1_1010_375e(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0xc),*(undefined2 *)((int)param_1 + 0xa));
}



void __stdcall16far pass1_1010_3770(ulong param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  astruct_477 *iVar3;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_477 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0xa,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0xa = uVar1;
  iVar3->field_0xc = param_3;
  return;
}



ushort * __stdcall16far pass1_1010_379e(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_3730(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_37d4(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1010_383a(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x16) = 0x0;
  *param_1 = 0x3b3e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  return param_1;
}



void __stdcall16far pass1_1010_3800(ushort *param_1)

{
  astruct_478 *iVar2;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_478 *)param_1;
  *param_1 = 0x3b3e;
  iVar2->field_0x2 = 0x1010;
  if (iVar2->field_0x16 != (undefined4 *)0x0) {
    fn_ptr_1000_17ce(*(astruct_18 **)iVar2->field_0x16,0x1000);
  }
  pass1_1010_3880(param_1);
  return;
}



void __stdcall16far struct_1010_383a(ushort *param_1)

{
  astruct_223 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_223 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  *param_1 = 0x3b5e;
  iVar1->field_0x2 = 0x1010;
  return;
}



void __stdcall16far pass1_1010_3880(ushort *param_1)

{
  int *piVar1;
  undefined4 *puVar2;
  uint uVar3;
  code **ppcVar4;
  long lVar5;
  astruct_472 *iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  int iStack4;
  
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_472 *)param_1;
  *param_1 = 0x3b5e;
  iVar6->field_0x2 = 0x1010;
  if (iVar6->field_0x8 != 0x0) {
    iStack4 = 0x0;
    while( true ) {
      piVar1 = &iVar6->field_0x10;
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      lVar5 = iVar6->field_0x8;
      uVar9 = (undefined2)((ulong)lVar5 >> 0x10);
      iVar7 = (int)lVar5;
      puVar2 = (undefined4 *)*(uint *)(iVar7 + iStack4 * 0x4);
      uVar3 = *(uint *)(iVar7 + iStack4 * 0x4 + 0x2);
      if ((uVar3 | (uint)puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      iStack4 = iStack4 + 0x1;
    }
    fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x8,0x1000);
  }
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



ushort * __stdcall16far struct_1010_38f8(ulong param_1,int param_2,uint param_3,uchar *param_4)

{
  ushort uVar1;
  astruct_240 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  if (param_2 != 0x0) {
    uVar1 = param_2 << 0x2;
    mem_op_1000_179c(uVar1,param_4,0x1000);
    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar2 = (astruct_240 *)param_1;
    iVar2->field_0x8 = uVar1;
    iVar2->field_0xa = param_4;
    return (ushort *)CONCAT22(param_4,iVar2->field_0x8);
  }
  mem_op_1000_179c(0x1a,param_4,0x1000);
  if (((uint)param_4 | param_3) != 0x0) {
    puVar3 = pass1_1010_37d4((ushort *)CONCAT22(param_4,param_3));
    return puVar3;
  }
  return (ushort *)0x0;
}



void __stdcall16far pass1_1010_394a(ushort param_1,ushort param_2,int param_3,uint param_4,uchar *param_5)

{
  if (param_3 != 0x0) {
    mem_op_1000_179c(param_3 << 0x2,param_5,0x1000);
    return;
  }
  mem_op_1000_179c(0x16,param_5,0x1000);
  if (((uint)param_5 | param_4) != 0x0) {
    struct_1010_383a((ushort *)CONCAT22(param_5,param_4));
    return;
  }
  return;
}



void __stdcall16far pass1_1010_398e(ulong *param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  int *piVar1;
  code **ppcVar2;
  ulong uVar3;
  undefined4 uVar4;
  int iVar5;
  ushort uVar6;
  uint extraout_DX;
  undefined2 extraout_DX_00;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ushort uStack12;
  undefined4 *puStack6;
  
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  uVar3 = *param_1;
  ppcVar2 = (code **)((int)uVar3 + 0x8);
  (**ppcVar2)();
  puStack6 = (undefined4 *)CONCAT22(extraout_DX,param_5);
  if ((extraout_DX | param_5) == 0x0) {
    return;
  }
  *(ulong *)(param_5 + 0xc) = param_4;
  iVar7 = (int)*puStack6;
  ppcVar2 = (code **)(iVar7 + 0xc);
  (**ppcVar2)();
  iVar5 = *(int *)((int)param_1 + 0x14);
  piVar1 = (int *)((int)param_1 + 0x14);
  *piVar1 = *piVar1 + 0x1;
  ppcVar2 = (code **)(iVar7 + 0x10);
  (**ppcVar2)();
  ppcVar2 = (code **)(iVar7 + 0x4);
  (**ppcVar2)();
  if (iVar5 != 0x0) {
    ppcVar2 = (code **)((int)uVar3 + 0x8);
    iVar7 = iVar5;
    (**ppcVar2)();
    *(int *)(param_5 + 0x8) = iVar7;
    *(undefined2 *)(param_5 + 0xa) = extraout_DX_00;
    PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + 0x1;
    uVar9 = extraout_DX_00;
    for (uStack12 = 0x0; (int)uStack12 < iVar5; uStack12 = uStack12 + 0x1) {
      uVar6 = uStack12;
      pass1_1010_398e(param_1,uStack12,(int)uStack12 >> 0xf,(ulong)puStack6,uStack12);
      uVar4 = *(undefined4 *)(param_5 + 0x8);
      uVar10 = (undefined2)((ulong)uVar4 >> 0x10);
      iVar7 = (int)uVar4;
      iVar8 = uStack12 * 0x4;
      *(ushort *)(iVar7 + iVar8) = uVar6;
      *(undefined2 *)(iVar7 + iVar8 + 0x2) = uVar9;
      uVar4 = *(undefined4 *)(param_5 + 0x8);
      if (*(long *)((int)uVar4 + iVar8) == 0x0) break;
    }
    PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + -0x1;
  }
  return;
}



ushort __stdcall16far pass1_1010_3a86(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0x10);
}



void __stdcall16far pass1_1010_3a94(ulong param_1,ushort param_2)

{
  *(ushort *)((int)param_1 + 0x12) = param_2;
  return;
}



ulong __stdcall16far pass1_1010_3aaa(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x6),*(undefined2 *)((int)param_1 + 0x4));
}



void __stdcall16far pass1_1010_3ac2(ulong param_1,ushort param_2,ulong param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ulong *)((int)param_1 + 0x16) = param_3;
  *(ushort *)((int)param_1 + 0x12) = param_2;
  return;
}



ulong __stdcall16far pass1_1010_3adc(ulong param_1)

{
  undefined2 *puVar1;
  
  puVar1 = (undefined2 *)*(undefined4 *)((int)param_1 + 0x16);
  return CONCAT22(*(undefined2 *)((int)puVar1 + 0x2),*puVar1);
}



ushort * __stdcall16far pass1_1010_3af2(ushort *param_1,byte param_2)

{
  pass1_1010_3800(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_3b18(ushort *param_1,byte param_2)

{
  pass1_1010_3880(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1010_3b7a(astruct_648 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x14 = 0x0;
  param_1->field_0x16 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x3d6a;
  param_1->field_0x2 = 0x1010;
  param_1->field_0xa = 0x3d7a;
  param_1->field_0xc = 0x1010;
  return;
}



void __stdcall16far pass1_1010_3bde(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 *puVar4;
  astruct_479 *iVar4;
  undefined2 uVar5;
  undefined2 *puStack14;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_479 *)param_1;
  *param_1 = 0x3d6a;
  iVar4->field_0x2 = 0x1010;
  iVar4->field_0xa = 0x3d7a;
  iVar4->field_0xc = 0x1010;
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (param_1 == (ushort *)0x0) {
    puVar4 = (undefined2 *)0x0;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar4->field_0xa;
  }
  puStack14 = (undefined2 *)CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1010_3c52(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_274 *iVar4;
  undefined2 uVar4;
  astruct_43 *paVar5;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_274 *)param_1;
  iVar4->field_0x14 = param_2;
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iVar4->field_0x14,param_3);
  iVar4->field_0xe = (undefined4 *)paVar5;
  iVar4->field_0x10 = (uint)((ulong)paVar5 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_3c9e(long param_1)

{
  uint uVar1;
  uchar *puVar2;
  
  if (param_1 == 0x0) {
    uVar1 = 0x0;
    puVar2 = (uchar *)0x0;
  }
  else {
    uVar1 = (int)param_1 + 0xa;
    puVar2 = param_1._2_2_;
  }
  pass1_1008_9262((int)_PTR_LOOP_1050_0388,(ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10),
                  (ulong)*(uint *)((int)param_1 + 0x12),CONCAT22(puVar2,uVar1),uVar1,puVar2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_3cd0(long param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == 0x0) {
      iVar1 = 0x0;
      uVar2 = 0x0;
    }
    else {
      iVar1 = (int)param_1 + 0xa;
      uVar2 = param_1._2_2_;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,(ulong)*(uint *)((int)param_1 + 0x12),CONCAT22(uVar2,iVar1));
  }
  return;
}



void __stdcall16far pass1_1010_3d0a(int param_1,ushort param_2,int param_3,ushort param_4)

{
  if (param_3 == 0x2) {
    pass1_1010_3cd0(CONCAT22(param_2,param_1 + -0xa));
    pass1_1010_1f62(param_4,CONCAT22(param_2,param_1 + -0xa),0x2);
  }
  return;
}



ushort * __stdcall16far pass1_1010_3d38(ushort *param_1,byte param_2)

{
  ushort unaff_SS;
  
  param_1 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa));
  pass1_1010_3bde(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_3d82(astruct_628 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  astruct_43 *paVar1;
  
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0xa = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x3e2c;
  param_1->field_0x2 = 0x1010;
  paVar1 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x99,param_4);
  param_1->field_0xa = (int)paVar1;
  param_1->field_0xc = (int)((ulong)paVar1 >> 0x10);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1010_3dc8(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_480 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_480 *)param_1;
  *param_1 = 0x3e2c;
  iVar4->field_0x2 = 0x1010;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



ushort * __stdcall16far pass1_1010_3e06(ushort *param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1010_3dc8(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_3e3c(astruct_55 *param_1,ushort param_2,ushort param_3)

{
  astruct_633 *iVar1;
  undefined2 uVar1;
  astruct_43 *paVar2;
  
  get_sys_metrics_1018_4b1e(param_1,0x6,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_633 *)param_1;
  iVar1->field_0x20 = 0x389a;
  iVar1->field_0x22 = 0x1008;
  iVar1->field_0x20 = 0x3aa8;
  iVar1->field_0x22 = 0x1008;
  iVar1->field_0x24 = 0x0;
  *(undefined4 *)&iVar1->field_0x66 = 0x0;
  iVar1->field_0x6a = 0x4;
  iVar1->field_0x6c = 0x0;
  iVar1->field_0x70 = 0x0;
  iVar1->field_0x74 = 0x0;
  pass1_1008_3e54((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x76),0x0,0x3,0x5);
  iVar1->field_0x7c = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_4a46;
  iVar1->field_0x2 = 0x1010;
  iVar1->field_0x20 = (int)&PTR_LOOP_1050_4a82;
  iVar1->field_0x22 = 0x1010;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x26),(WNDCLASS16 *)0x0,0x40);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1a1,param_3);
  iVar1->field_0x66 = (int)paVar2;
  iVar1->field_0x68 = (int)((ulong)paVar2 >> 0x10);
  pass1_1018_4b78((ulong *)param_1,param_3);
  return;
}



void __stdcall16far pass1_1010_3f00(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int *piVar4;
  astruct_481 *iVar5;
  undefined2 uVar5;
  undefined2 *puStack16;
  int iStack4;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_481 *)param_1;
  *param_1 = (ushort)&PTR_LOOP_1050_4a46;
  iVar5->field_0x2 = 0x1010;
  iVar5->field_0x20 = (int)&PTR_LOOP_1050_4a82;
  iVar5->field_0x22 = 0x1010;
  iStack4 = 0x0;
  do {
    puVar1 = (undefined4 *)*(uint *)(&iVar5->field_0x26 + iStack4 * 0x4);
    uVar2 = *(uint *)(&iVar5->field_0x26 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x10);
  puVar1 = iVar5->field_0x66;
  uVar2 = iVar5->field_0x68;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x70,0x1000);
  if (param_1 == (ushort *)0x0) {
    piVar4 = (int *)0x0;
    uVar5 = 0x0;
  }
  else {
    piVar4 = &iVar5->field_0x20;
  }
  puStack16 = (undefined2 *)CONCAT22(uVar5,piVar4);
  *puStack16 = 0x389a;
  piVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1010_404a(ulong param_1,ulong param_2,int param_3,uint16_t param_4)

{
  ushort uVar1;
  int iVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uVar5;
  undefined2 local_4;
  
  uVar4 = (ushort)param_2;
  uVar5 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar4,uVar5,0x5,0x1008,param_4);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
  }
  else {
    iVar2 = (int)param_1;
    uVar1 = (ushort)(param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x24,0x0,uVar1,0x2,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = read_file_1008_7dee(uVar4,uVar5,(ushort)&local_4,0x0,param_4,0x2,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x7e,0x0,uVar1,0x2,0x1008);
        if (BVar3 != 0x0) {
          *(undefined2 *)(iVar2 + 0x6a) = local_4;
          return;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_40cc(ulong param_1,int param_2,ushort param_3)

{
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                  *(ulong *)((int)param_1 + 0x6c));
  return CONCAT22(*(undefined2 *)(param_2 + 0xe),*(undefined2 *)(param_2 + 0xc));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pt_in_rect_1010_40f8(ulong param_1,POINT16 *param_2,RECT16 *param_3)

{
  int *piVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uVar5;
  int iVar6;
  uchar *in_DX;
  uchar *puVar7;
  uchar *puVar8;
  int unaff_DI;
  undefined2 uVar9;
  ushort unaff_SS;
  ushort *puVar10;
  undefined4 *puStack16;
  int iStack6;
  ushort uStack4;
  
  iStack6 = 0x0;
  uStack4 = 0x0;
  do {
    uVar9 = (undefined2)(param_1 >> 0x10);
    piVar1 = (int *)((int)param_1 + 0x74);
    if (*piVar1 == iStack6 || *piVar1 < iStack6) {
LAB_1010_413e:
      if ((uStack4 != 0x0) && (0x3 < (int)PTR_LOOP_1050_3960)) {
        puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,iStack6 + 0xcU,unaff_SS,in_DX,unaff_DI);
        puVar7 = (uchar *)((ulong)puVar10 >> 0x10);
        uVar4 = pass1_1018_0afa((ulong)puVar10);
        if (uVar4 == 0x0) {
          uVar9 = 0x1000;
          uVar5 = uVar4;
          mem_op_1000_179c(0xb4,puVar7,0x1000);
          puVar8 = (uchar *)((uint)puVar7 | uVar5);
          if (puVar8 == (uchar *)0x0) {
            iVar6 = 0x0;
            puVar8 = (uchar *)0x0;
          }
          else {
            uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
            iVar6 = string_1040_8520((astruct_57 *)CONCAT22(puVar7,uVar5),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x643,
                                     0x645,puVar8,unaff_SS);
          }
          puStack16 = (undefined4 *)CONCAT22(puVar8,iVar6);
          ppcVar2 = (code **)((int)*puStack16 + 0x74);
          (**ppcVar2)(uVar9,iVar6,puVar8);
          pass1_1010_209e(_PTR_LOOP_1050_0ed0,iStack6 + 0xcU);
          uStack4 = uVar4;
        }
      }
      if (uStack4 != 0x0) {
        return iStack6;
      }
      return -0x1;
    }
    in_DX = *(uchar **)((int)param_1 + 0x72);
    BVar3 = PtInRect16(param_3,*param_2);
    if (BVar3 != 0x0) {
      uStack4 = 0x1;
      goto LAB_1010_413e;
    }
    iStack6 = iStack6 + 0x1;
    param_3 = (RECT16 *)s_tile2_bmp_1050_1538;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_41d6(ulong param_1,ulong param_2,uchar *param_3,ushort param_4,uchar param_5)

{
  uint *puVar1;
  int *piVar2;
  undefined4 uVar3;
  ushort uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  uchar *puVar8;
  uchar *puVar9;
  astruct_243 *iVar9;
  astruct_244 *iVar10;
  int unaff_DI;
  undefined2 uVar10;
  undefined2 uVar11;
  ushort *puVar12;
  int iStack50;
  int local_30;
  astruct_18 *local_2e;
  int iStack42;
  astruct_18 *paStack40;
  astruct_18 *paStack34;
  astruct_18 *paStack30;
  int iStack26;
  uint uStack24;
  int iStack22;
  undefined4 uStack20;
  uint uStack16;
  undefined4 uStack14;
  ulong uStack10;
  ushort uStack6;
  ushort uStack4;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar9 = (astruct_243 *)param_1;
  iVar9->field_0x6c = param_2;
  puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_3,unaff_DI);
  uStack4 = (ushort)((ulong)puVar12 >> 0x10);
  uStack6 = (ushort)puVar12;
  uStack10 = pass1_1010_ec40(uStack6,uStack4,iVar9->field_0x6c,uStack6,uStack4);
  puVar8 = (uchar *)(uStack10 >> 0x10);
  iVar9->field_0x74 = *(int *)((int)uStack10 + 0x22);
  if (*(long *)&iVar9->field_0x70 != 0x0) {
    paStack34 = *(astruct_18 **)&iVar9->field_0x70;
    paStack30 = paStack34;
    fn_ptr_1000_17ce(paStack34,0x1000);
    *(undefined4 *)&iVar9->field_0x70 = 0x0;
  }
  uVar4 = iVar9->field_0x74 << 0x7;
  mem_op_1000_179c(uVar4,puVar8,0x1000);
  *(ushort *)&iVar9->field_0x70 = uVar4;
  iVar9->field_0x72 = puVar8;
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),iVar9->field_0x6c);
  uStack14 = CONCAT22(puVar8,uVar4);
  uStack16 = (uint)(**(int **)(uVar4 + 0x10) == 0x9);
  iStack22 = *(int *)((int)uStack10 + 0x22);
  uVar4 = iStack22 * 0x6;
  mem_op_1000_179c(uVar4,puVar8,0x1000);
  paStack30 = (astruct_18 *)CONCAT22(puVar8,uVar4);
  puVar9 = (uchar *)((uint)puVar8 | uVar4);
  if (puVar9 == (uchar *)0x0) {
    uStack20 = (astruct_18 *)0x0;
  }
  else {
    pass1_1000_5586((uchar *)0x3e38,0x1008,iStack22,0x6,uVar4,(ushort)puVar8);
    uStack20 = paStack30;
  }
  uStack24 = 0x0;
  while( true ) {
    uVar11 = (undefined2)(uStack10 >> 0x10);
    puVar1 = (uint *)((int)uStack10 + 0x22);
    if (*puVar1 < uStack24 || *puVar1 == uStack24) break;
    uVar3 = *(undefined4 *)((int)uStack10 + 0x24);
    uVar5 = uStack24;
    pass1_1028_e0a0(_PTR_LOOP_1050_65e2,(ulong)*(uint *)((int)uVar3 + uStack24 * 0x2) << 0x10,puVar9,param_4,param_5);
    paStack34 = (astruct_18 *)CONCAT22(puVar9,uVar5);
    pass1_1008_3f62((ushort *)((ulong)uStack20 & 0xffff0000 | (ulong)(uStack24 * 0x6 + (int)uStack20)),
                    (ushort *)CONCAT22(puVar9,uVar5 + 0x8));
    paStack40 = paStack34;
    paStack30 = paStack34;
    if (paStack34 != (astruct_18 *)0x0) {
      fn_ptr_1030_84d0((ulong)paStack34);
      fn_ptr_1000_17ce(paStack34,0x1000);
    }
    uStack24 = uStack24 + 0x1;
    puVar9 = uStack20._2_2_;
  }
  for (iStack26 = 0x0; piVar2 = &iVar9->field_0x74, *piVar2 != iStack26 && iStack26 <= *piVar2;
      iStack26 = iStack26 + 0x1) {
    pass1_1008_3e94((ushort *)((ulong)uStack20 & 0xffff0000 | (ulong)(uint)(iStack26 * 0x6 + (int)uStack20)),
                    (ushort *)CONCAT22(param_4,&local_2e),(ushort *)CONCAT22(param_4,&local_30));
    iStack42 = pass1_1000_49b2((uint)local_2e);
    iStack42 = iStack42 / 0x5;
    if (0xc < iStack42) {
      iStack42 = 0xc;
      iVar6 = pass1_1000_49b2((uint)local_2e);
      local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)(uint)(((int)(uint)local_2e / iVar6) * 0x3c));
    }
    iVar7 = pass1_1000_49b2((uint)local_2e);
    iVar6 = (int)((long)iVar7 % 0x5);
    paStack34 = (astruct_18 *)((ulong)paStack34 & 0xffff0000 | (long)iVar7 % 0x5 & 0xffffU);
    if ((int)(uint)local_2e < 0x0) {
      if (0x2 < iVar6) {
        iVar6 = iVar6 + -0x5;
      }
      local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)((uint)local_2e + iVar6));
    }
    else {
      if (iVar6 < 0x3) {
        local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)((uint)local_2e - iVar6));
      }
      else {
        local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)((uint)local_2e + (0x5 - iVar6)));
      }
    }
    iStack50 = local_30 / 0x16;
    for (iVar6 = 0x0; iVar6 < 0x10; iVar6 = iVar6 + 0x1) {
      if (0xf < iStack50) {
        iStack50 = 0x0;
      }
      if (((int)(uint)(uStack16 != 0x0) < iStack50) && (iStack50 < 0x8)) {
        iVar7 = *(int *)((iStack42 * 0x10 + iStack50) * 0x2 + 0x11e0);
        iVar10 = (astruct_244 *)((iStack26 * 0x10 + iVar6) * 0x8);
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(int *)(iVar10 + (int)uVar3) = iVar7 + 0x49;
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(uint *)(iVar10 + (int)uVar3 + 0x2) = (uint)local_2e + 0x49;
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(int *)(iVar10 + (int)uVar3 + 0x4) = iVar7 + 0x4e;
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(uint *)(iVar10 + (int)uVar3 + 0x6) = (uint)local_2e + 0x4e;
      }
      else {
        iVar7 = (iStack26 * 0x10 + iVar6) * 0x8;
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(undefined2 *)(iVar7 + (int)uVar3) = 0x0;
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(undefined2 *)((int)uVar3 + iVar7 + 0x2) = 0x0;
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(undefined2 *)((int)uVar3 + iVar7 + 0x4) = 0x1;
        uVar3 = *(undefined4 *)&iVar9->field_0x70;
        *(undefined2 *)((int)uVar3 + iVar7 + 0x6) = 0x1;
      }
      iStack50 = iStack50 + 0x1;
    }
  }
  paStack40 = uStack20;
  local_2e = uStack20;
  fn_ptr_1000_17ce(uStack20,0x1000);
  draw_1010_47ae(param_1,0x1000,param_4);
  return;
}

