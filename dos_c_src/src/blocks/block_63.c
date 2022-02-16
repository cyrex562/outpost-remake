

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_199a(ushort param_1,int param_2,uchar param_3,ulong param_4)

{
  int *piVar1;
  ulong uVar2;
  uchar *extraout_DX;
  uchar *puVar3;
  undefined2 uVar4;
  int *piVar5;
  ushort uVar6;
  ushort *puVar7;
  ushort uVar8;
  undefined2 local_15e;
  undefined2 uStack348;
  ulong *puStack50;
  ulong uStack42;
  undefined2 uStack38;
  int *piStack36;
  int local_22;
  ushort local_20;
  ulong uStack30;
  ushort *puStack26;
  int local_16;
  undefined4 local_14;
  ulong local_10;
  undefined2 uStack12;
  ulong uStack10;
  int iStack6;
  uchar *puStack4;
  
  piVar1 = (int *)((int)param_4 + 0x14);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    pass1_1028_b58e(param_4);
    uStack10 = *(ulong *)(param_2 + 0x2e);
    iStack6 = param_2;
    puStack4 = extraout_DX;
    pass1_1038_5804(uStack10,0x1,0x3);
    local_10 = *(ulong *)(iStack6 + 0xc);
    uStack12 = *(undefined2 *)(iStack6 + 0x10);
    puStack50 = &local_10;
    puVar3 = puStack4;
    pass1_1008_3eb4((ushort *)CONCAT22(param_1,&local_10),(ushort *)CONCAT22(param_1,&local_16),
                    (ushort *)CONCAT22(param_1,&local_14),(ushort *)CONCAT22(param_1,(int)&local_14 + 0x2));
    puStack26 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,puVar3,(int)&uStack10);
    uVar2 = *(ulong *)((int)puStack26 + 0x20);
    puVar7 = &local_20;
    piStack36 = &local_22;
    piVar5 = piStack36;
    uVar6 = param_1;
    uVar8 = param_1;
    uStack30 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)(uVar2 >> 0x10));
    uStack38 = (undefined2)uVar2;
    pass1_1030_5b1c(uVar2 & 0xffff | ZEXT24(piStack36) << 0x10,(ushort *)CONCAT22(uVar6,piVar5),
                    (ushort *)CONCAT22(uVar8,puVar7));
    if (local_22 < local_16 + 0x1) {
      pass1_1030_5b3e(CONCAT22(piStack36,uStack38),local_16 + 0x1,local_20);
      pass1_1030_5b1c(CONCAT22(piStack36,uStack38),(ushort *)CONCAT22(param_1,&local_22),
                      (ushort *)CONCAT22(param_1,&local_20));
    }
    uVar4 = (undefined2)(uStack10 >> 0x10);
    uStack42 = *(ulong *)((int)uStack10 + 0x4);
    struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,&local_15e),0x0,0x0,
                        (-(uint)(local_16 == 0x0) & 0xffd3) + 0x60,&local_10,param_1,
                        uStack42 & 0xffff | (ulong)*(uint *)((int)uStack10 + 0x6) << 0x10,uStack30);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,&local_15e));
    local_15e = 0x389a;
    uStack348 = 0x1008;
    pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_10),local_16 + 0x1,(ushort)local_14,
                    (ushort)((ulong)local_14 >> 0x10));
    struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,&local_15e),0x0,0x0,0x60,&local_10,param_1,
                        uStack42,uStack30);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,&local_15e));
  }
  return;
}



void __stdcall16far pass1_1028_1b1e(ulong param_1)

{
  *(undefined2 *)((int)param_1 + 0x14) = 0x7;
  return;
}



astruct_18 * __stdcall16far pass1_1028_1b2e(astruct_18 *param_1,byte param_2,uint param_3)

{
  pass1_1030_dcf4(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_1bbc(ushort *param_1)

{
  astruct_190 *iVar1;
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_190 *)param_1;
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x22 = 0x0;
  *param_1 = 0x1eee;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_1be8(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined2 *)(param_1 + 0x20) = 0x0;
  *(undefined2 *)(param_1 + 0x22) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0x1eee;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far pass1_1028_1c1c(void)

{
  return 0x0;
}



ushort __stdcall16far pass1_1028_1c22(ulong param_1)

{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(int *)(iVar2 + 0x20) != 0x0) && ((*(int *)(iVar2 + 0x12) == 0x5 || (*(int *)(iVar2 + 0x12) == 0x6)))) {
    if (*(int *)(iVar2 + 0xc) == 0x16) {
      return 0x19;
    }
    if (*(int *)(iVar2 + 0xc) == 0x17) {
      return 0x1a;
    }
  }
  uVar1 = pass1_1028_bc1c(param_1 & 0xffff | (ulong)uVar3 << 0x10);
  return uVar1;
}



void __stdcall16far pass1_1028_1c66(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  int iVar2;
  ulong uVar3;
  
  if (*(int *)((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2((ulong)param_1);
  if (*(long *)((int)uVar3 + 0x200) != 0x8000002) {
    ppcVar1 = (code **)((int)*param_1 + 0x64);
    iVar2 = (**ppcVar1)(param_4,param_1);
    if (iVar2 == 0x0) {
      return;
    }
    pass1_1028_cb04((ulong)param_1,param_2,param_3,param_5);
    if (iVar2 == 0x0) {
      iVar2 = 0x6;
      goto LAB_1028_1cbd;
    }
    pass1_1028_c952((ulong)param_1,param_2,param_3,param_5);
  }
  iVar2 = 0x5;
LAB_1028_1cbd:
  pass1_1028_bdac(param_1,iVar2,param_4);
  return;
}



// WARNING: Could not reconcile some variable overlaps

ushort __stdcall16far
pass1_1028_1cca(ulong param_1,ulong *param_2,uint param_3,ushort param_4,ushort param_5,long param_6,ushort param_7)

{
  ushort uVar1;
  ushort uVar2;
  ushort uVar3;
  undefined local_e [0x2];
  int local_c;
  int local_a;
  undefined4 local_8;
  undefined2 uStack4;
  
  local_8 = *param_2;
  uStack4 = *(undefined2 *)(param_2 + 0x1);
  pass1_1008_3eb4((ushort *)CONCAT22(param_7,&local_8),(ushort *)CONCAT22(param_7,local_e),
                  (ushort *)CONCAT22(param_7,&local_c),(ushort *)CONCAT22(param_7,&local_a));
  local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar1 = pass1_1028_1e14(uVar2,uVar3,0x16,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,param_7);
  if (uVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
    uVar1 = pass1_1028_1e14(uVar2,uVar3,0x16,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,param_7
                           );
    if (uVar1 == 0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8._2_2_ = local_c;
      uVar1 = pass1_1028_1e14(uVar2,uVar3,0x17,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,
                              param_7);
      if (uVar1 == 0x0) {
        local_8 = CONCAT22(local_8._2_2_,local_a + 0x1);
        uVar1 = pass1_1028_1e14(uVar2,uVar3,0x17,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,
                                param_7);
        if (uVar1 == 0x0) {
          return uVar1;
        }
      }
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_1da4(ushort param_1,ushort param_2,ushort *param_3,ulong param_4,long param_5,ushort param_6)

{
  int iVar1;
  undefined *puVar2;
  uint uVar3;
  ulong uVar4;
  undefined local_4 [0x2];
  
  uVar4 = pass1_1030_bcae((ushort)local_4,param_6);
  uVar3 = (uint)(uVar4 >> 0x10);
  iVar1 = (int)uVar4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
  uVar4 = *(ulong *)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar4,(uint)(uVar4 >> 0x10));
  puVar2 = local_4;
  pass1_1030_bcde(param_6,(ushort)puVar2,param_6,uVar4 & 0xffff | (ulong)uVar3 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (undefined *)0x6af;
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1028_1e14(ushort param_1,ushort param_2,int param_3,ushort *param_4,long param_5,uint param_6,uint param_7,
               ushort param_8)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  
  pass1_1030_627e(param_8,param_6,param_7,_PTR_LOOP_1050_5740,param_4,param_5);
  uVar2 = param_7 | param_6;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6,param_7);
    if ((uVar2 | param_6) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_6));
      if (uVar3 != 0x0) {
        iVar1 = *(int *)((int)uVar3 + 0xc);
        if (((iVar1 == 0x18) || (iVar1 == 0x3f)) || (iVar1 == param_3)) {
          return 0x1;
        }
      }
    }
  }
  return 0x0;
}



ushort __stdcall16far pass1_1028_1e8a(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uint uVar1;
  ulong uVar2;
  uint uVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  if ((*(byte *)((int)param_1 + 0x1a) & 0x2) == 0x0) {
    uVar4 = 0x0;
    uVar5 = 0x23;
    uVar3 = 0x1;
    uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1030_7d7c(uVar2,uVar3,CONCAT22(uVar5,uVar4),(uint)uVar2,(uchar *)(uVar2 >> 0x10),param_2,param_3,param_4);
    pass1_1028_bdac(param_1,0x6,0x1030);
    return 0x0;
  }
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1028_1ec8(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1028_1f56(ushort *param_1,uchar *param_2)

{
  undefined4 uVar1;
  uint uVar2;
  undefined2 extraout_DX;
  astruct_186 *iVar3;
  undefined2 uVar3;
  
  struct_1028_b354(param_1);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_186 *)param_1;
  uVar2 = 0x0;
  *(undefined4 *)&iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  *param_1 = 0x2572;
  iVar3->field_0x2 = (int)&USHORT_1050_1028;
  mem_op_1000_179c(0xc,param_2,0x1000);
  if (((uint)param_2 | uVar2) == 0x0) {
    *(undefined4 *)&iVar3->field_0x20 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_2,uVar2));
    iVar3->field_0x20 = uVar2;
    iVar3->field_0x22 = extraout_DX;
  }
  uVar1 = *(undefined4 *)&iVar3->field_0x20;
  *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
  return;
}



void __stdcall16far pass1_1028_1fc8(int param_1,ushort param_2,int param_3,ulong param_4,uchar *param_5)

{
  undefined4 uVar1;
  uint uVar2;
  undefined2 extraout_DX;
  
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,(ushort)param_5);
  uVar2 = 0x0;
  *(undefined4 *)(param_1 + 0x20) = 0x0;
  *(undefined2 *)(param_1 + 0x24) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x2572;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  mem_op_1000_179c(0xc,param_5,0x1000);
  if (((uint)param_5 | uVar2) == 0x0) {
    *(undefined4 *)(param_1 + 0x20) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_5,uVar2));
    *(uint *)(param_1 + 0x20) = uVar2;
    *(undefined2 *)(param_1 + 0x22) = extraout_DX;
  }
  uVar1 = *(undefined4 *)(param_1 + 0x20);
  *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_2042(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  astruct_602 *iVar5;
  undefined2 uVar5;
  ulong uVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_602 *)param_1;
  *param_1 = 0x2572;
  iVar5->field_0x2 = (int)&USHORT_1050_1028;
  uVar4 = *(undefined4 *)&iVar5->field_0x20;
  *(undefined2 *)((int)uVar4 + 0xa) = 0x1;
  puVar1 = iVar5->field_0x20;
  uVar2 = iVar5->field_0x22;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if ((_PTR_LOOP_1050_65e2 != 0x0) && (_PTR_LOOP_1050_5a64 != 0x0)) {
    uVar6 = pass1_1028_b58e((ulong)param_1);
    pass1_1038_79f2(_PTR_LOOP_1050_5a64,uVar6,param_2);
  }
  pass1_1028_b418(param_1);
  return;
}



ushort __stdcall16far pass1_1028_20b0(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_20b6(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uint uVar1;
  uchar *puVar2;
  ushort uVar3;
  uint uVar4;
  undefined auStack22 [0x2];
  int iStack20;
  int iStack18;
  ulong uStack16;
  undefined2 uStack12;
  ulong uStack10;
  ulong uStack6;
  
  pass1_1028_be9e(param_1,param_2,param_3,param_4,param_5);
  uVar4 = (uint)((ulong)param_1 >> 0x10);
  uVar3 = (ushort)param_1;
  if (*(int *)(uVar3 + 0x12) == 0x5) {
    uStack6 = pass1_1028_bb24((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10);
    uStack10 = pass1_1028_b58e((ulong)param_1);
    puVar2 = (uchar *)(uStack10 >> 0x10);
    uStack16 = *(ulong *)((int)uStack10 + 0xc);
    uStack12 = *(undefined2 *)((int)uStack10 + 0x10);
    pass1_1008_3eb4((ushort *)CONCAT22(param_5,&uStack16),(ushort *)CONCAT22(param_5,auStack22),
                    (ushort *)CONCAT22(param_5,&iStack20),(ushort *)CONCAT22(param_5,&iStack18));
    uStack16 = uStack16 & 0xffff | (ulong)(iStack20 - 0x1) << 0x10;
    uVar1 = pass1_1028_21ba(uVar3,uVar4,(ushort *)CONCAT22(param_5,&uStack16),uStack6,(uint)&uStack16,(uint)puVar2,
                            param_5);
    if (uVar1 == 0x0) {
      uStack16 = uStack16 & 0xffff | (ulong)(iStack20 + 0x1) << 0x10;
      uVar1 = pass1_1028_21ba(uVar3,uVar4,(ushort *)CONCAT22(param_5,&uStack16),uStack6,(uint)&uStack16,(uint)puVar2,
                              param_5);
      if (uVar1 == 0x0) {
        uStack16 = CONCAT22(iStack20,iStack18 + -0x1);
        uVar1 = pass1_1028_21ba(uVar3,uVar4,(ushort *)CONCAT22(param_5,&uStack16),uStack6,(uint)&uStack16,(uint)puVar2,
                                param_5);
        if (uVar1 == 0x0) {
          uStack16 = uStack16 & 0xffff0000 | (ulong)(iStack18 + 0x1);
          uVar1 = pass1_1028_21ba(uVar3,uVar4,(ushort *)CONCAT22(param_5,&uStack16),uStack6,(uint)&uStack16,(uint)puVar2
                                  ,param_5);
          if (uVar1 == 0x0) {
            return;
          }
        }
      }
    }
    pass1_1038_79b2(_PTR_LOOP_1050_5a64,uStack10,uVar1,puVar2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1028_21ba(ushort param_1,ushort param_2,ushort *param_3,long param_4,uint param_5,uint param_6,ushort param_7)

{
  uint uVar1;
  ulong uVar2;
  
  pass1_1030_627e(param_7,param_5,param_6,_PTR_LOOP_1050_5740,param_3,param_4);
  uVar1 = param_6 | param_5;
  if (uVar1 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,param_6);
    if ((uVar1 | param_5) != 0x0) {
      uVar2 = struct_op_1030_73a8(CONCAT22(uVar1,param_5));
      if ((uVar2 != 0x0) && (*(int *)((int)uVar2 + 0xc) == 0x40)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far
pass1_1028_2220(undefined2 param_1,undefined2 param_2,int param_3,ushort *param_4,long param_5,uint param_6,uint param_7
               ,ushort param_8)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  
  pass1_1030_627e(param_8,param_6,param_7,_PTR_LOOP_1050_5740,param_4,param_5);
  uVar2 = param_7 | param_6;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6,param_7);
    if ((uVar2 | param_6) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_6));
      if ((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == 0x40 || (iVar1 == param_3)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

int __stdcall16far
pass1_1028_2290(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,ushort param_5,long param_6,ushort param_7)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  undefined local_e [0x2];
  int local_c;
  int local_a;
  undefined4 local_8;
  undefined2 uStack4;
  
  local_8 = *param_2;
  uStack4 = *(undefined2 *)(param_2 + 0x1);
  pass1_1008_3eb4((ushort *)CONCAT22(param_7,&local_8),(ushort *)CONCAT22(param_7,local_e),
                  (ushort *)CONCAT22(param_7,&local_c),(ushort *)CONCAT22(param_7,&local_a));
  local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
  uVar2 = (undefined2)param_1;
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar1 = pass1_1028_2220(uVar2,uVar3,0x16,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
  if (iVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
    iVar1 = pass1_1028_2220(uVar2,uVar3,0x16,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
    if (iVar1 == 0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8._2_2_ = local_c;
      iVar1 = pass1_1028_2220(uVar2,uVar3,0x17,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
      if (iVar1 == 0x0) {
        local_8 = CONCAT22(local_8._2_2_,local_a + 0x1);
        iVar1 = pass1_1028_2220(uVar2,uVar3,0x17,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
        if (iVar1 == 0x0) {
          return iVar1;
        }
      }
    }
  }
  return 0x1;
}



ushort __stdcall16far pass1_1028_236a(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uint uVar1;
  ulong uVar2;
  uint uVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  if ((*(byte *)((int)param_1 + 0x1a) & 0x2) == 0x0) {
    uVar4 = 0x0;
    uVar5 = 0x23;
    uVar3 = 0x1;
    uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1030_7d7c(uVar2,uVar3,CONCAT22(uVar5,uVar4),(uint)uVar2,(uchar *)(uVar2 >> 0x10),param_2,param_3,param_4);
    pass1_1028_bdac(param_1,0x6,0x1030);
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_23a8(ushort param_1,ushort param_2,ushort *param_3,ulong param_4,long param_5,ushort param_6)

{
  int iVar1;
  undefined *puVar2;
  uint uVar3;
  ulong uVar4;
  undefined local_4 [0x2];
  
  uVar4 = pass1_1030_bcae((ushort)local_4,param_6);
  uVar3 = (uint)(uVar4 >> 0x10);
  iVar1 = (int)uVar4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
  uVar4 = *(ulong *)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar4,(uint)(uVar4 >> 0x10));
  puVar2 = local_4;
  pass1_1030_bcde(param_6,(ushort)puVar2,param_6,uVar4 & 0xffff | (ulong)uVar3 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (undefined *)0x6af;
    return;
  }
  return;
}



BOOL16 __stdcall16far pass1_1028_2418(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  undefined2 uVar3;
  ulong uVar4;
  undefined2 local_1c [0x6];
  undefined2 uStack16;
  int iStack14;
  undefined2 uStack12;
  undefined local_a [0x8];
  
  BVar2 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar2 != 0x0) {
    uVar3 = (undefined2)(param_1 >> 0x10);
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x20));
    uVar1 = *(undefined4 *)((int)param_1 + 0x20);
    local_1c[0] = *(undefined2 *)((int)uVar1 + 0x8);
    uStack16 = local_1c[0];
    BVar2 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)local_1c,param_3,(char *)0x2,0x1008);
    if (BVar2 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return BVar2;
    }
    while( true ) {
      uVar4 = pass1_1008_5b12(local_a,param_3);
      iStack14 = (int)uVar4;
      if (uVar4 == 0x0) break;
      pass1_1038_75ca(uVar4,param_2,iStack14,param_3);
      uStack12 = (undefined2)(uVar4 >> 0x10);
      if ((BOOL16)uVar4 == 0x0) {
        return (BOOL16)uVar4;
      }
    }
    BVar2 = 0x1;
  }
  return BVar2;
}



BOOL16 __stdcall16far file_1028_24a2(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  undefined4 uVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uchar *extraout_DX;
  uchar *puVar7;
  undefined2 uVar8;
  undefined2 uVar10;
  ulong uVar9;
  uint uStack6;
  uint local_4;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 == 0x0) {
    return 0x0;
  }
  if (0x1 < (int)PTR_LOOP_1050_0312) {
    BVar3 = read_file_1008_7dee((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)&local_4,0x0,param_5,0x2,0x1008);
    if (BVar3 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return 0x0;
    }
    for (uStack6 = 0x0; uStack6 < local_4; uStack6 = uStack6 + 0x1) {
      uVar8 = 0x2a;
      uVar6 = local_4;
      uVar9 = param_2;
      mem_op_1000_179c(0x2a,param_4,0x1000);
      puVar7 = (uchar *)((uint)param_4 | uVar6);
      if (puVar7 == (uchar *)0x0) {
        uVar4 = 0x0;
        puVar7 = (uchar *)0x0;
      }
      else {
        uVar5 = uVar6;
        struct_1038_6520((ushort *)CONCAT22(param_4,uVar6));
        uVar4 = uVar6;
        uVar6 = uVar5;
      }
      uVar10 = (undefined2)(uVar9 >> 0x10);
      uVar5 = uVar4;
      file_1038_774e(CONCAT22(puVar7,uVar4),CONCAT22((int)uVar9,uVar8),puVar7,param_5);
      if (uVar5 == 0x0) {
        return 0x0;
      }
      uVar8 = (undefined2)(param_1 >> 0x10);
      uVar1 = *(undefined4 *)((int)param_1 + 0x20);
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x20) + 0x8);
      (**ppcVar2)((int)&PTR_LOOP_1050_1038,(int)uVar1,(int)((ulong)uVar1 >> 0x10),uVar4,puVar7,uVar10,uVar6);
      param_4 = extraout_DX;
    }
  }
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1028_254c(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1028_2042(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_25da(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = (int)s_fem16_wav_1050_2644 + 0x8;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_25fc(int param_1,undefined2 param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = (int)s_fem16_wav_1050_2644 + 0x8;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



astruct_18 * __stdcall16far pass1_1028_2626(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_26b4(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0x2788;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_26d6(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x2788;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1028_2700(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uint uVar1;
  ulong uVar2;
  
  pass1_1028_be2a(param_1,param_2,param_3,param_4,param_5);
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) == 0x5) {
    uVar2 = pass1_1028_b4f2((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    *(undefined2 *)((int)uVar2 + 0x204) = 0x1;
  }
  return;
}



void __stdcall16far pass1_1028_272e(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  ulong uVar1;
  
  pass1_1028_be9e(param_1,param_2,param_3,param_4,param_5);
  uVar1 = pass1_1028_b4f2((ulong)param_1);
  if (*(int *)((int)param_1 + 0x12) == 0x5) {
    *(undefined2 *)((int)uVar1 + 0x204) = 0x1;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_2762(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_27f0(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0x2a92;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_2812(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x2a92;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  *(undefined2 *)(param_1 + 0xe) = *(undefined2 *)(param_1 + 0xc);
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps

ushort __stdcall16far
pass1_1028_2844(ulong param_1,ulong *param_2,uint param_3,ushort param_4,ushort param_5,long param_6,ushort param_7)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  ushort uVar4;
  undefined local_e [0x2];
  int local_c;
  int local_a;
  undefined4 local_8;
  undefined2 uStack4;
  
  local_8 = *param_2;
  uStack4 = *(undefined2 *)(param_2 + 0x1);
  pass1_1008_3eb4((ushort *)CONCAT22(param_7,&local_8),(ushort *)CONCAT22(param_7,local_e),
                  (ushort *)CONCAT22(param_7,&local_c),(ushort *)CONCAT22(param_7,&local_a));
  local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
  uVar3 = (ushort)param_1;
  uVar4 = (ushort)(param_1 >> 0x10);
  BVar1 = pass1_1028_c5a6(uVar3,uVar4,0x7b,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,param_7);
  if (BVar1 == 0x0) {
    uVar2 = pass1_1028_297c(param_1,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
    if (uVar2 == 0x0) {
      local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
      BVar1 = pass1_1028_c5a6(uVar3,uVar4,0x7b,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,
                              param_7);
      if (BVar1 == 0x0) {
        uVar2 = pass1_1028_297c(param_1,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
        if (uVar2 == 0x0) {
          local_8._0_2_ = local_a + -0x1;
          local_8._2_2_ = local_c;
          BVar1 = pass1_1028_c5a6(uVar3,uVar4,0x7c,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,param_3,
                                  param_7);
          if (BVar1 == 0x0) {
            uVar2 = pass1_1028_297c(param_1,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
            if (uVar2 == 0x0) {
              local_8 = CONCAT22(local_8._2_2_,local_a + 0x1);
              BVar1 = pass1_1028_c5a6(uVar3,uVar4,0x7c,(ushort *)CONCAT22(param_7,&local_8),param_6,(uint)&local_8,
                                      param_3,param_7);
              if (BVar1 == 0x0) {
                uVar3 = pass1_1028_297c(param_1,(ushort *)CONCAT22(param_7,&local_8),param_6,&local_8,param_3,param_7);
                if (uVar3 == 0x0) {
                  return uVar3;
                }
              }
            }
          }
        }
      }
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1028_297c(ulong param_1,ushort *param_2,long param_3,uint param_4,uint param_5,ushort param_6)

{
  char cVar1;
  uint uVar2;
  ulong uVar3;
  
  pass1_1028_c7b6(param_6,param_5,(ushort)param_1,(ushort)(param_1 >> 0x10),param_2,param_3);
  if (param_4 == 0x0) {
    pass1_1030_627e(param_6,0x0,param_5,_PTR_LOOP_1050_5740,param_2,param_3);
    uVar2 = param_5 | param_4;
    if (uVar2 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,param_5);
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_4));
      uVar2 = *(uint *)((int)uVar3 + 0xc);
      if (0x4a < (int)uVar2) {
        switch(uVar2) {
        case 0x4c:
        case 0x4d:
        case 0x4e:
        case 0x60:
        case 0x61:
        case 0x62:
        case 0x63:
        case 0x6e:
        case 0x73:
        case 0x74:
        case 0x75:
        case 0x76:
        case 0x77:
        case 0x78:
        case 0x79:
          goto switchD_1028_2a0b_caseD_4c;
        default:
          goto switchD_1028_2a0b_caseD_4f;
        }
      }
      if (((int)uVar2 < 0x48) && (uVar2 != 0x41)) {
        if (uVar2 < 0x42) {
          cVar1 = (char)uVar2;
          if (cVar1 < '5') {
            if ('2' < cVar1) {
              return 0x0;
            }
            cVar1 = cVar1 + -0x10;
          }
          else {
            cVar1 = cVar1 + -0x3e;
          }
          if (cVar1 == '\0') {
            return 0x0;
          }
        }
switchD_1028_2a0b_caseD_4f:
        return 0x1;
      }
    }
  }
switchD_1028_2a0b_caseD_4c:
  return 0x0;
}



astruct_18 * __stdcall16far pass1_1028_2a6c(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_2afa(ushort *param_1)

{
  struct_1030_dc96(param_1);
  *param_1 = 0x2b74;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_2b1c(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1030_dcc2(param_1,param_2,param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x2b74;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



astruct_18 * __stdcall16far pass1_1028_2b4e(astruct_18 *param_1,byte param_2,uint param_3)

{
  pass1_1030_dcf4(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_2bdc(ushort *param_1)

{
  struct_1028_0954(param_1);
  *param_1 = 0x341c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}

