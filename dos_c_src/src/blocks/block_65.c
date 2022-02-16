
astruct_18 * __stdcall16far pass1_1028_3fde(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_406c(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0x42ec;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_408e(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x42ec;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_40b8(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  ushort uVar4;
  undefined2 extraout_DX;
  uint uVar5;
  ulong uVar6;
  ulong *puVar7;
  undefined4 uStack54;
  undefined local_2c [0x6];
  undefined4 *puStack38;
  undefined4 uStack34;
  undefined4 *puStack26;
  undefined4 uStack24;
  undefined4 local_14;
  int iStack16;
  int iStack14;
  undefined4 local_c;
  int iStack8;
  ushort uStack6;
  undefined2 uStack4;
  
  pass1_1028_b58e(param_1);
  local_14 = *(undefined4 *)(param_2 + 0xc);
  iStack8 = *(int *)(param_2 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34._2_2_,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = (ulong *)CONCAT22(param_3,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  uStack6 = param_2;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_1);
  uVar5 = (uint)(uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(param_3,puVar2,uVar5,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_3,puVar2),
                  uVar6 & 0xffff | (ulong)uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = *(uint *)((int)puVar2 + 0x2);
  uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
  uVar3 = (uint)uStack54._3_1_;
  uStack24 = uStack34;
  if (uStack54._3_1_ != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack34,uVar5);
    uStack54 = CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if (uVar4 == 0x64) {
      puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
      ppcVar1 = (code **)((int)*puStack38 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b514(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_4194(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uchar *puVar1;
  ulong uVar2;
  ushort *puVar3;
  
  pass1_1028_be9e(param_1,param_2,param_3,param_4,param_5);
  uVar2 = pass1_1028_b4f2((ulong)param_1);
  puVar1 = (uchar *)(uVar2 >> 0x10);
  if ((*(long *)((int)uVar2 + 0x200) != 0x8000002) && (*(int *)((int)param_1 + 0x12) == 0x5)) {
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,puVar1,param_3);
    pass1_1010_043a((ulong)puVar3,*(long *)((int)uVar2 + 0x4),0xe,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_41ea(ulong param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  ushort uVar4;
  undefined2 extraout_DX;
  uint uVar5;
  ulong uVar6;
  ulong *puVar7;
  undefined4 uStack54;
  undefined local_2c [0x6];
  undefined4 *puStack38;
  undefined4 uStack34;
  undefined4 *puStack26;
  undefined4 uStack24;
  undefined4 local_14;
  int iStack16;
  int iStack14;
  undefined4 local_c;
  int iStack8;
  int iStack6;
  undefined2 uStack4;
  
  pass1_1028_b514(param_1);
  pass1_1028_b58e(param_1);
  local_14 = *(undefined4 *)(param_2 + 0xc);
  iStack8 = *(int *)(param_2 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34._2_2_,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = (ulong *)CONCAT22(param_3,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_2;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_1);
  uVar5 = (uint)(uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(param_3,puVar2,uVar5,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_3,puVar2),
                  uVar6 & 0xffff | (ulong)uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = *(uint *)((int)puVar2 + 0x2);
  uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
  uVar3 = (uint)uStack54._3_1_;
  if (uStack54._3_1_ != 0x0) {
    uStack24 = uStack34;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack34,uVar5);
    uStack54 = CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if (uVar4 == 0x64) {
      puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
      ppcVar1 = (code **)((int)*puStack38 + 0x24);
      (**ppcVar1)();
    }
  }
  return;
}



ulong __stdcall16far pass1_1028_42ca(int param_1,ushort param_2)

{
  pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
  if ((*(byte *)(param_1 + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6),0x1000);
  }
  return CONCAT22(*(undefined2 *)(param_1 + 0x8),*(undefined2 *)(param_1 + 0x6));
}



ushort * __stdcall16far struct_1028_4354(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0x446a;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_4376(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x446a;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far pass1_1028_43a0(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  if ((*(int *)((int)param_1 + 0x12) != 0x6) && (*(int *)((int)param_1 + 0x12) != 0x5)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_43c2(int param_1,ushort param_2,int param_3,uchar *param_4,int param_5,ushort param_6)

{
  ushort *puVar1;
  
  pass1_1028_bd38(CONCAT22(param_2,param_1),(ushort)param_4,param_6);
  if (param_3 == 0x0) {
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_6,param_4,param_5);
    pass1_1010_988c((ulong)puVar1,*(int *)(param_1 + 0xc));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_43f6(ulong param_1,int param_2,uchar *param_3,ushort param_4,int param_5,ushort param_6)

{
  ushort uVar1;
  ushort *puVar2;
  ulong uVar3;
  uint uVar4;
  
  uVar1 = 0x83;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_6,param_3,param_5);
  uVar1 = pass1_1010_65d0(param_6,(ulong)puVar2,uVar1);
  if (0x0 < (int)uVar1) {
    uVar3 = pass1_1028_b58e(param_1);
    if (param_2 == 0x0) {
      uVar4 = 0x0;
    }
    else {
      uVar4 = 0x3e8;
    }
    pass1_1030_7d1c(uVar3,uVar4,0x230000,(int)uVar3,(int)(uVar3 >> 0x10),param_4,param_5,param_6);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_4444(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_44d2(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x20) = 0x0;
  *param_1 = 0x4836;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_44fe(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined4 *)(param_1 + 0x20) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0x4836;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1028_4530(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x4836;
  *(undefined2 *)(iVar4 + 0x2) = (int)&USHORT_1050_1028;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x20);
  uVar2 = *(uint *)(iVar4 + 0x22);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(param_1);
  return;
}



void __stdcall16far pass1_1028_456e(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  pass1_1028_b46e(param_1,param_2,param_3);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x20);
  uVar2 = *(uint *)(iVar4 + 0x22);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar4 + 0x20) = 0x0;
  return;
}



void __stdcall16far pass1_1028_45b0(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uint uVar1;
  ulong uVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  int iVar5;
  
  pass1_1028_be9e(param_1,param_2,param_3,param_4,param_5);
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) == 0x5) {
    uVar4 = 0x0;
    iVar5 = 0x4;
    uVar3 = 0x2;
    uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1030_7c50(uVar2,CONCAT22(uVar4,uVar3),iVar5,(uint)uVar2,(uchar *)(uVar2 >> 0x10));
  }
  return;
}



ulong __stdcall16far pass1_1028_45e2(ulong param_1,uint param_2,int param_3,ushort param_4)

{
  pass1_1028_478a(param_1,param_2,param_4);
  return CONCAT22(-(uint)(0x3e8 < param_2) - param_3,0x3e8 - param_2);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_45fe(ulong param_1,int param_2,ushort param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  undefined2 extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar4;
  undefined2 extraout_DX_01;
  uint uVar5;
  astruct_312 *iVar6;
  astruct_314 *iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  astruct_99 *paStack44;
  long local_28;
  undefined4 *puStack34;
  uchar *puStack32;
  astruct_99 *paStack30;
  int local_1a [0x4];
  ulong uStack18;
  ulong uStack14;
  ulong *puStack10;
  undefined4 uStack6;
  astruct_313 *uVar2;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_2);
  puStack10 = *(ulong **)(param_2 + 0x22);
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_312 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar3 = iVar6->field_0x20;
  puVar4 = *(uchar **)((int)&iVar6->field_0x20 + 0x2);
  paStack30 = (astruct_99 *)CONCAT22(puVar4,puVar3);
  puStack34 = puVar3;
  puStack32 = puVar4;
  if (((uint)puVar4 | (uint)puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
    puVar4 = extraout_DX_00;
  }
  mem_op_1000_179c(0xc,puVar4,0x1000);
  puStack34 = puVar3;
  puStack32 = puVar4;
  if (((uint)puVar4 | (uint)puVar3) == 0x0) {
    puVar3 = (undefined4 *)0x0;
    uVar7 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar4,puVar3));
    uVar7 = extraout_DX_01;
  }
  *(undefined4 **)&iVar6->field_0x20 = puVar3;
  *(undefined2 *)((int)&iVar6->field_0x20 + 0x2) = uVar7;
  if (puStack10 != (ulong *)0x0) {
    uStack14 = (ulong)*(uint *)((int)puStack10 + 0x4);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1) {
      pass1_1020_bb16(puStack10,(ulong *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,&local_28)),
                      (ushort *)CONCAT22(param_3,local_1a),(uint)uStack18);
      if ((local_28 != 0x0) && (local_1a[0] != 0x0)) {
        paStack30 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar5 = (uint)((ulong)paStack30 >> 0x10);
        uVar2 = (astruct_313 *)paStack30;
        if ((uVar5 | (uint)uVar2) == 0x0) {
          paStack44 = (astruct_99 *)0x0;
        }
        else {
          paStack30->field_0x0 = 0x389a;
          uVar2->field_0x2 = 0x1008;
          uVar2->field_0x4 = 0x0;
          uVar2->field_0x6 = 0x0;
          uVar2->field_0x8 = 0x0;
          uVar2->field_0xa = 0x0;
          uVar2->field_0xc = 0x0;
          paStack30->field_0x0 = 0x56ce;
          uVar2->field_0x2 = 0x1018;
          paStack44 = paStack30;
        }
        uVar7 = (undefined2)((ulong)paStack44 >> 0x10);
        iVar5 = (astruct_314 *)paStack44;
        iVar5->field_0x4 = local_1a[0];
        iVar5->field_0xa = (undefined2)local_28;
        iVar5->field_0xc = (undefined2)local_28;
        puVar1 = iVar6->field_0x20;
        ppcVar2 = (code **)((int)*iVar6->field_0x20 + 0x8);
        (**ppcVar2)(0x1000,(char)puVar1,(int)((ulong)puVar1 >> 0x10),iVar5,uVar7);
      }
    }
  }
  return;
}



ushort __stdcall16far pass1_1028_4768(ulong param_1,uint param_2,int param_3,ushort param_4)

{
  pass1_1028_478a(param_1,param_2,param_4);
  if ((param_3 == 0x0) && (param_2 < 0x3e8)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1028_478a(ulong param_1,int param_2,ushort param_3)

{
  undefined2 extraout_DX;
  long local_1e;
  int local_1a [0x4];
  uint uStack18;
  uint uStack16;
  long lStack14;
  ulong *puStack10;
  undefined4 uStack6;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_2);
  puStack10 = *(ulong **)(param_2 + 0x22);
  lStack14 = 0x0;
  if ((*(uint *)(param_2 + 0x24) | (uint)puStack10) == 0x0) {
    return;
  }
  uStack16 = *(uint *)((uint)puStack10 + 0x4);
  for (uStack18 = 0x0; uStack18 < uStack16; uStack18 = uStack18 + 0x1) {
    pass1_1020_bb16(puStack10,(ulong *)CONCAT22(param_3,&local_1e),(ushort *)CONCAT22(param_3,local_1a),uStack18);
    if (0x0 < local_1a[0]) {
      lStack14 = lStack14 + local_1e;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_4810(astruct_18 *param_1,byte param_2)

{
  pass1_1028_4530(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_489e(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = (ushort)&PTR_LOOP_1050_4942;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_48c0(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = (ushort)&PTR_LOOP_1050_4942;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  *(undefined2 *)(param_1 + 0xe) = *(undefined2 *)(param_1 + 0xc);
  *(undefined2 *)(param_1 + 0x10) = *(undefined2 *)(param_1 + 0xc);
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1028_48fa(ulong *param_1,ushort param_2)

{
  pass1_1028_bdac(param_1,0x0,param_2);
  return;
}



ulong __stdcall16far pass1_1028_4920(int param_1,ushort param_2)

{
  pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
  if ((*(byte *)(param_1 + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6),0x1000);
  }
  return CONCAT22(*(undefined2 *)(param_1 + 0x8),*(undefined2 *)(param_1 + 0x6));
}



ushort * __stdcall16far struct_1028_49aa(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0x4b1c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x20)),(WNDCLASS16 *)0x0,0xa);
  return param_1;
}



ulong __stdcall16far pass1_1028_49de(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x4b1c;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x20),(WNDCLASS16 *)0x0,0xa);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1028_4a1a(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if ((BVar1 != 0x0) &&
     (BVar1 = write_to_file_1008_7e1c
                        ((ushort)param_2,(ushort)(param_2 >> 0x10),(int)param_1 + 0x20,(ushort)(param_1 >> 0x10),
                         (char *)0xa,0x1008), BVar1 == 0x0)) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
  }
  return;
}



void __stdcall16far pass1_1028_4a5a(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  BOOL16 BVar1;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if ((param_3 != 0x0) &&
     (BVar1 = read_file_1008_7dee((ushort)param_2,(ushort)(param_2 >> 0x10),(int)param_1 + 0x20,0x0,
                                  (ushort)(param_1 >> 0x10),0xa,0x1008), BVar1 == 0x0)) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  return;
}



ushort __stdcall16far pass1_1028_4a9a(ulong param_1,int param_2)

{
  return *(ushort *)((int)param_1 + 0x20 + param_2 * 0x2);
}



void __stdcall16far pass1_1028_4ab2(ulong param_1,ushort param_2,int param_3)

{
  *(ushort *)((int)param_1 + param_3 * 0x2 + 0x20) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_4aca(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ushort *puVar1;
  
  if (*(int *)((int)param_1 + 0x20) != 0x0) {
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_2,param_3);
    pass1_1010_ed3e((ulong)puVar1);
    pass1_1030_2a2c((ulong)puVar1,(uchar *)((ulong)puVar1 >> 0x10),param_3,param_4);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_4af6(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_4b84(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_4ba6(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far pass1_1028_4bd0(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  if ((*(int *)((int)param_1 + 0x12) != 0x6) && (*(int *)((int)param_1 + 0x12) != 0x5)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_4bf2(ulong param_1,ulong param_2,int param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 *puVar2;
  ushort uVar3;
  undefined2 extraout_DX;
  uint uVar4;
  ulong uVar5;
  ulong *puVar6;
  undefined4 uStack54;
  undefined local_2c [0x6];
  undefined4 *puStack38;
  undefined4 uStack34;
  undefined4 *puStack26;
  undefined4 uStack24;
  undefined4 local_14;
  int iStack16;
  int iStack14;
  undefined4 local_c;
  int iStack8;
  int iStack6;
  undefined2 uStack4;
  
  pass1_1028_b58e(param_1);
  local_14 = *(undefined4 *)(param_3 + 0xc);
  iStack8 = *(int *)(param_3 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34._2_2_,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar6 = (ulong *)CONCAT22(param_4,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_3;
  uStack4 = extraout_DX;
  uVar5 = pass1_1028_bb24(param_1);
  uVar4 = (uint)(uVar5 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(param_4,puVar2,uVar4,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_4,puVar2),
                  uVar5 & 0xffff | (ulong)uVar4 << 0x10,puVar6);
  uStack34 = *puVar2;
  uVar4 = *(uint *)((int)puVar2 + 0x2);
  uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
  uVar3 = (ushort)uStack54._3_1_;
  uStack24 = uStack34;
  if (uStack54._3_1_ != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack34,uVar4);
    uStack54 = CONCAT22(uVar4,uVar3);
    uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar3));
    if ((uVar3 == 0x62) || (uVar3 == 0x63)) {
      puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
      uVar3 = (ushort)puStack38;
      ppcVar1 = (code **)((int)*puStack38 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(param_1,param_2,uVar3);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_4cd6(ulong param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  ushort uVar4;
  undefined2 extraout_DX;
  uint uVar5;
  ulong uVar6;
  ulong *puVar7;
  undefined4 uStack54;
  undefined local_2c [0x6];
  undefined4 *puStack38;
  undefined4 uStack34;
  undefined4 *puStack26;
  undefined4 uStack24;
  undefined4 local_14;
  int iStack16;
  int iStack14;
  undefined4 local_c;
  int iStack8;
  int iStack6;
  undefined2 uStack4;
  
  pass1_1028_b514(param_1);
  pass1_1028_b58e(param_1);
  local_14 = *(undefined4 *)(param_2 + 0xc);
  iStack8 = *(int *)(param_2 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34._2_2_,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = (ulong *)CONCAT22(param_3,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_2;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_1);
  uVar5 = (uint)(uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(param_3,puVar2,uVar5,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_3,puVar2),
                  uVar6 & 0xffff | (ulong)uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = *(uint *)((int)puVar2 + 0x2);
  uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
  uVar3 = (uint)uStack54._3_1_;
  if (uStack54._3_1_ != 0x0) {
    uStack24 = uStack34;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack34,uVar5);
    uStack54 = CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if ((uVar4 == 0x62) || (uVar4 == 0x63)) {
      puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
      ppcVar1 = (code **)((int)*puStack38 + 0x24);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_4db2(ushort param_1,ushort param_2,int param_3,uchar *param_4,int param_5,ushort param_6,uchar param_7)

{
  BOOL16 BVar1;
  int *piVar2;
  undefined2 extraout_DX;
  ushort *puVar3;
  int *piVar4;
  ushort uVar5;
  ushort *puVar6;
  ushort uVar7;
  undefined local_14e [0x124];
  ulong uStack42;
  undefined4 uStack38;
  int local_22;
  undefined local_20 [0x2];
  undefined local_1e [0x2];
  ulong local_1c;
  int iStack24;
  undefined4 uStack22;
  int *piStack18;
  undefined2 uStack16;
  int local_e;
  ushort local_c;
  ulong uStack10;
  ushort *puStack6;
  
  BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(param_1 + 0xc),0x29);
  if (BVar1 != 0x0) {
    pass1_1028_bd38(CONCAT22(param_2,param_1),(ushort)param_4,param_6);
    if ((param_3 == 0x0) && (*(int *)(param_1 + 0xc) == 0x13)) {
      puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_6,param_4,param_5);
      param_4 = (uchar *)((ulong)puVar3 >> 0x10);
      pass1_1010_988c((ulong)puVar3,*(int *)(param_1 + 0xc));
    }
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,param_4,param_5);
    uStack16 = (undefined2)((ulong)puStack6 >> 0x10);
    uStack10 = *(ulong *)((int)puStack6 + 0x20);
    puVar6 = &local_c;
    piVar2 = &local_e;
    piVar4 = piVar2;
    uVar5 = param_6;
    uVar7 = param_6;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack10,(uint)(uStack10 >> 0x10));
    piStack18 = piVar2;
    pass1_1030_5b1c(CONCAT22(uStack16,piVar2),(ushort *)CONCAT22(uVar5,piVar4),(ushort *)CONCAT22(uVar7,puVar6));
    pass1_1028_b58e(CONCAT22(param_2,param_1));
    uStack22 = CONCAT22(extraout_DX,piVar2);
    local_1c = *(ulong *)(piVar2 + 0x6);
    iStack24 = piVar2[0x8];
    pass1_1028_c8ee(param_6,param_1,param_2,0x1,(ushort *)CONCAT22(param_6,&local_1c));
    pass1_1008_3eb4((ushort *)CONCAT22(param_6,&local_1c),(ushort *)CONCAT22(param_6,&local_22),
                    (ushort *)CONCAT22(param_6,local_20),(ushort *)CONCAT22(param_6,local_1e));
    if (local_e < local_22) {
      pass1_1030_5b3e(CONCAT22(uStack16,piStack18),local_22,local_c);
      pass1_1030_5b1c(CONCAT22(uStack16,piStack18),(ushort *)CONCAT22(param_6,&local_e),
                      (ushort *)CONCAT22(param_6,&local_c));
    }
    uStack38 = *(undefined4 *)((int)uStack22 + 0x2e);
    uStack42 = *(ulong *)((int)uStack38 + 0x4);
    struct_op_1028_87f0(param_6,param_7,(astruct_97 *)CONCAT22(param_6,local_14e),0x0,0x0,0x62,&local_1c,param_6,
                        uStack42,uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,local_14e));
    pass1_1028_ccd0(param_7,param_6,CONCAT22(param_2,param_1),(ushort *)CONCAT22(param_6,&local_1c));
  }
  return;
}



void __stdcall16far pass1_1028_4f30(ulong param_1,int param_2,ushort param_3,ushort param_4,ushort param_5)

{
  ulong uVar1;
  uint uVar2;
  
  uVar1 = pass1_1028_b58e(param_1);
  if (param_2 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = 0x3e8;
  }
  pass1_1030_7d1c(uVar1,uVar2,0x230000,(int)uVar1,(int)(uVar1 >> 0x10),param_3,param_4,param_5);
  return;
}



ushort __stdcall16far pass1_1028_4f62(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = pass1_1028_b58e(param_1);
  if (*(int *)((int)uVar1 + 0x10) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1028_4faa(ulong param_1,ushort param_2)

{
  ushort uVar1;
  undefined4 *puVar2;
  uint uVar3;
  ulong uVar4;
  undefined4 local_c;
  undefined2 uStack8;
  undefined4 uStack6;
  
  uVar1 = pass1_1028_4f62(param_1);
  if (uVar1 != 0x0) {
    return param_1;
  }
  uStack6 = pass1_1028_b58e(param_1);
  local_c = *(undefined4 *)((int)uStack6 + 0xc);
  uStack8 = 0x0;
  uVar4 = pass1_1028_bb24(param_1);
  uVar3 = (uint)(uVar4 >> 0x10);
  puVar2 = &local_c;
  pass1_1030_627e(param_2,(uint)puVar2,uVar3,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_2,puVar2),
                  uVar4 & 0xffff | (ulong)uVar3 << 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar2,uVar3);
  if ((uVar3 | (uint)puVar2) == 0x0) {
    return 0x0;
  }
  uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,puVar2));
  return uVar4;
}

