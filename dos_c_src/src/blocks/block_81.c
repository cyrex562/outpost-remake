

ulong __stdcall16far pass1_1030_5b5c(int param_1,ushort param_2)

{
  return CONCAT22(param_2,param_1 + 0x14);
}



void __stdcall16far pass1_1030_5b6c(ulong param_1,char *param_2,ushort param_3)

{
  long lVar1;
  ushort uVar2;
  astruct_610 *iVar4;
  astruct_609 *iVar3;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_610 *)param_1;
  if (iVar4->field_0x10 != 0x0) {
    lVar1 = iVar4->field_0x10;
    fn_ptr_1000_17ce(*(astruct_18 **)((int)lVar1 + 0x4),0x1000);
    uVar2 = str_op_1008_60e8(param_2,param_3);
    lVar1 = iVar4->field_0x10;
    uVar3 = (undefined2)((ulong)lVar1 >> 0x10);
    iVar3 = (astruct_609 *)lVar1;
    iVar3->field_0x4 = uVar2;
    iVar3->field_0x6 = param_3;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_5baa(astruct_18 *param_1,byte param_2)

{
  pass1_1030_56b0(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_5bec(ulong param_1)

{
  _PTR_LOOP_1050_5736 = param_1;
  pass1_1000_54a0(param_1,0x0,0x24);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_5c0e(void)

{
  _PTR_LOOP_1050_5736 = 0x0;
  return;
}



BOOL16 __stdcall16far pass1_1030_5c1a(ulong param_1,ulong param_2,uint16_t param_3)

{
  BOOL16 BVar1;
  
  BVar1 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar1 != 0x0) {
    BVar1 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)param_1,(ushort)(param_1 >> 0x10),(char *)0x24,
                       0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 __stdcall16far read_file_1030_5c52(undefined4 param_1,undefined4 param_2,uint16_t param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  
  uVar2 = (ushort)((ulong)param_2 >> 0x10);
  read_file_1008_7cfe((ushort)param_2,uVar2,0x9,0x1008,param_4);
  if (param_3 != 0x0) {
    BVar1 = read_file_1008_7dee((ushort)param_2,uVar2,(ushort)param_1,0x0,(ushort)((ulong)param_1 >> 0x10),0x24,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return BVar1;
    }
    param_3 = 0x1;
  }
  return param_3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_5c8a(ulong param_1,ulong param_2)

{
  long *plVar1;
  uint uVar2;
  ulong uVar3;
  uint uVar4;
  astruct_177 *iVar5;
  undefined2 uVar5;
  ulong uStack6;
  
  uStack6 = 0x0;
  uVar2 = (uint)param_2._3_1_;
  if (uVar2 == 0xff) {
    return;
  }
  uVar5 = (undefined2)((ulong)_PTR_LOOP_1050_65e2 >> 0x10);
  iVar5 = (astruct_177 *)((int)_PTR_LOOP_1050_65e2 + 0xa);
  uVar3 = *(ulong *)(iVar5 + uVar2 * 0x4);
  uVar4 = *(uint *)(iVar5 + uVar2 * 0x4 + 0x2);
  if (*(int *)((int)uVar3 + 0x4) != 0x0) {
    pass1_1030_12ca(uVar3 & 0xffff | (ulong)uVar4 << 0x10);
    uStack6 = uVar3 & 0xffff | (ulong)uVar4 << 0x10;
  }
  if (uStack6 == 0x0) {
    plVar1 = (long *)(uVar2 * 0x4 + (int)param_1);
    *plVar1 = *plVar1 + 0x1;
  }
  return;
}



ushort * __stdcall16far pass1_1030_5d0a(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1030_17ce(param_1,0x1,0x4);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x10) = 0x0;
  *param_1 = 0x613e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



ushort * __stdcall16far pass1_1030_5d3c(ushort *param_1,ulong param_2,uint param_3,uchar *param_4)

{
  ushort uVar1;
  
  pass1_1030_183c(param_1,0x1,0x4,0x1000000,param_2,param_3,param_4);
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x10) = 0x0;
  *param_1 = 0x613e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



void __stdcall16far pass1_1030_5d78(ushort *param_1)

{
  uint uVar1;
  astruct_18 *paVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = 0x613e;
  *(undefined2 *)(iVar3 + 0x2) = 0x1030;
  paVar2 = *(astruct_18 **)(iVar3 + 0x10);
  uVar1 = *(uint *)(iVar3 + 0x12);
  if ((uVar1 | (uint)paVar2) != 0x0) {
    pass1_1030_8480((astruct_18 **)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10));
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1030_18b2(param_1);
  return;
}



void __stdcall16far pass1_1030_5dbe(ulong param_1,ulong param_2,uint16_t param_3,ushort param_4)

{
  ulong uVar1;
  undefined4 uVar2;
  ushort uVar3;
  BOOL16 BVar4;
  int iVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 local_c [0x5];
  
  uVar3 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar3 != 0x0) {
    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    BVar4 = pass1_1008_7c2a(param_2,*(char **)*(char **)(iVar6 + 0x10),0x1008);
    if ((BVar4 != 0x0) &&
       (uVar1 = *(ulong *)(iVar6 + 0x10),
       iVar5 = write_to_file_1008_7b4c(param_2,uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x4),0x1008,param_4),
       iVar5 != 0x0)) {
      uVar2 = *(undefined4 *)(iVar6 + 0x10);
      local_c[0] = *(undefined2 *)((int)uVar2 + 0xa);
      uVar3 = (ushort)(param_2 >> 0x10);
      BVar4 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)local_c,param_4,(char *)0x2,0x1008);
      if (BVar4 != 0x0) {
        uVar2 = *(undefined4 *)(iVar6 + 0x10);
        if (*(int *)((int)uVar2 + 0xa) == 0x0) {
          return;
        }
        uVar2 = *(undefined4 *)(iVar6 + 0x10);
        uVar7 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar6 = (int)uVar2;
        uVar2 = *(undefined4 *)(iVar6 + 0xc);
        BVar4 = write_to_file_1008_7e1c
                          ((ushort)param_2,uVar3,(ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),
                           (char *)(ulong)(uint)(*(int *)(iVar6 + 0xa) * 0x2),0x1008);
        if (BVar4 != 0x0) {
          return;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1030_5e70(ulong param_1,ulong param_2,int param_3,undefined *param_4,ushort param_5)

{
  ulong uVar1;
  undefined4 uVar2;
  ushort uVar3;
  undefined *puVar4;
  ushort uVar5;
  BOOL16 BVar6;
  ushort uVar7;
  uchar *puVar8;
  int iVar9;
  int unaff_DI;
  undefined2 uVar10;
  ushort *puVar11;
  int iVar12;
  undefined2 uVar13;
  ushort uVar14;
  undefined4 uStack1034;
  undefined local_402 [0x400];
  
  iVar12 = (int)param_1;
  uVar13 = (undefined2)(param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4,0x1000);
      PTR_LOOP_1050_5f2e = param_4;
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(0x10,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    uStack1034 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    puVar8 = (uchar *)((uint)PTR_LOOP_1050_5f2e | uVar3);
    if (puVar8 == (uchar *)0x0) {
      *(undefined4 *)(iVar12 + 0x10) = 0x0;
    }
    else {
      puVar11 = pass1_1008_3e38((ushort *)CONCAT22(PTR_LOOP_1050_5f2e,uVar3 + 0x4));
      puVar8 = (uchar *)((ulong)puVar11 >> 0x10);
      *(undefined4 *)(iVar12 + 0x10) = uStack1034;
    }
    puVar4 = local_402;
    uVar3 = (ushort)param_2;
    uVar14 = (ushort)(param_2 >> 0x10);
    read_file_1008_7c6e(uVar3,uVar14,(char *)CONCAT22(param_5,puVar4),0x1008);
    if (puVar4 != (undefined *)0x0) {
      uVar5 = str_op_1008_60e8((char *)CONCAT22(param_5,local_402),(ushort)puVar8);
      puVar11 = *(ushort **)(iVar12 + 0x10);
      *puVar11 = uVar5;
      *(uchar **)((int)puVar11 + 0x2) = puVar8;
      uVar1 = *(ulong *)(iVar12 + 0x10);
      BVar6 = read_file_1008_7bc8(param_2,(ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x4)),0x1008,param_5);
      if (BVar6 != 0x0) {
        uVar2 = *(undefined4 *)(iVar12 + 0x10);
        BVar6 = read_file_1008_7dee(uVar3,uVar14,(int)uVar2 + 0xa,0x0,(ushort)((ulong)uVar2 >> 0x10),0x2,0x1008);
        if (BVar6 != 0x0) {
          uVar2 = *(undefined4 *)(iVar12 + 0x10);
          uVar10 = (undefined2)((ulong)uVar2 >> 0x10);
          iVar9 = (int)uVar2;
          if (*(int *)(iVar9 + 0xa) == 0x0) {
LAB_1030_5fb7:
            puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_5,puVar8,unaff_DI);
            pass1_1018_04ca((ulong)puVar11,*(ulong *)(iVar12 + 0x4));
            return;
          }
          uVar5 = *(int *)(iVar9 + 0xa) * 0x2;
          uVar7 = uVar5;
          mem_op_1000_179c(uVar5,puVar8,0x1000);
          uVar2 = *(undefined4 *)(iVar12 + 0x10);
          uVar10 = (undefined2)((ulong)uVar2 >> 0x10);
          iVar9 = (int)uVar2;
          *(ushort *)(iVar9 + 0xc) = uVar7;
          *(uchar **)(iVar9 + 0xe) = puVar8;
          uVar2 = *(undefined4 *)(iVar12 + 0x10);
          uVar2 = *(undefined4 *)((int)uVar2 + 0xc);
          BVar6 = read_file_1008_7dee(uVar3,uVar14,(ushort)uVar2,0x0,(ushort)((ulong)uVar2 >> 0x10),uVar5,0x1008);
          if (BVar6 != 0x0) goto LAB_1030_5fb7;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



void __stdcall16far pass1_1030_5fe2(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x10) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_5ff6(ulong param_1,uint param_2,uchar *param_3,uchar *param_4,uchar param_5)

{
  uint *puVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  int iVar5;
  ushort uVar6;
  uchar *puVar7;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  uchar local_6c [0x58];
  undefined4 uStack20;
  undefined4 uStack16;
  ulong uStack12;
  uint uStack8;
  uchar *puStack6;
  uint uStack4;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  puVar7 = param_3;
  if (*(long *)(iVar8 + 0xc) == 0x0) {
    mem_op_1000_179c(0x18,param_3,0x1000);
    puVar7 = (uchar *)((uint)param_3 | param_2);
    uStack8 = param_2;
    puStack6 = param_3;
    if (puVar7 == (uchar *)0x0) {
      *(undefined4 *)(iVar8 + 0xc) = 0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_3,param_2),0x5,0x5);
      *(uint *)(iVar8 + 0xc) = param_2;
      *(uchar **)(iVar8 + 0xe) = extraout_DX;
      puVar7 = extraout_DX;
    }
  }
  for (uStack4 = 0x0; uVar4 = *(undefined4 *)(iVar8 + 0x10), puVar1 = (uint *)((int)uVar4 + 0xa),
      uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 = uStack4 + 0x1) {
    uStack12 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)puVar7,0x2);
    iVar5 = (int)uStack12;
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xc) + 0x8);
    (**ppcVar2)((int)&USHORT_1050_1028,*(undefined4 *)(iVar8 + 0xc),iVar5,(char)(uStack12 >> 0x10),uStack4,0x0);
    puVar7 = extraout_DX_00;
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uStack12);
    uStack16 = CONCAT22(puVar7,iVar5);
    uStack20 = *(undefined4 *)(iVar5 + 0x10);
    if (*(long *)((int)uStack20 + 0x2) == 0x0) {
      puVar3 = (undefined4 *)*(undefined4 *)(iVar8 + 0x10);
      sys_1000_3f9c(local_6c,param_4,(ushort)s__s__d_1050_573a,(ushort)&USHORT_1050_1050,(ushort)*puVar3,&stack0xfffe,
                    (int)((ulong)puVar3 >> 0x10),0x1000,param_4,param_5);
      uVar6 = str_op_1008_60e8((char *)CONCAT22(param_4,local_6c),(ushort)puVar7);
      uVar10 = (undefined2)((ulong)uStack20 >> 0x10);
      *(ushort *)((int)uStack20 + 0x2) = uVar6;
      *(uchar **)((int)uStack20 + 0x4) = puVar7;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_6118(astruct_18 *param_1,byte param_2)

{
  pass1_1030_5d78(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_615a(astruct_137 *param_1,ushort param_2)

{
  uint uVar1;
  ushort extraout_DX;
  astruct_137 *iVar2;
  ushort uVar2;
  
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_137 *)param_1;
  uVar1 = 0x0;
  *(undefined4 *)param_1 = 0x0;
  *(undefined4 *)&iVar2->field_0x4 = 0x0;
  mem_op_1000_179c(0xc,(uchar *)param_2,0x1000);
  if ((param_2 | uVar1) == 0x0) {
    *(undefined4 *)&iVar2->field_0x4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_2,uVar1));
    iVar2->field_0x4 = uVar1;
    iVar2->field_0x6 = extraout_DX;
  }
  _PTR_LOOP_1050_5740 = param_1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_61b0(uint *param_1)

{
  uint uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar1 = *(uint *)(iVar4 + 0x2);
  if ((uVar1 | (uint)(undefined4 *)*param_1) != 0x0) {
    ppcVar3 = (code **)*(undefined4 *)*param_1;
    (**ppcVar3)();
  }
  puVar2 = (undefined4 *)*(uint *)(iVar4 + 0x4);
  uVar1 = *(uint *)(iVar4 + 0x6);
  if ((uVar1 | (uint)puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)();
  }
  _PTR_LOOP_1050_5740 = 0x0;
  return;
}



void __stdcall16far
pass1_1030_61fe(ulong param_1,ulong param_2,ulong param_3,long param_4,ushort param_5,ushort param_6,ushort param_7)

{
  pass1_1030_677a(param_1,param_4,param_7);
  pass1_1030_8aa0(CONCAT22(param_6,param_5),param_2,(ushort *)param_3,param_6,param_7);
  return;
}



ushort __stdcall16far
pass1_1030_6222(ulong param_1,int param_2,ulong param_3,ulong param_4,uint param_5,uchar *param_6,ushort param_7)

{
  code **ppcVar1;
  uint uVar2;
  ushort extraout_DX;
  ulong uStack6;
  
  mem_op_1000_179c(0x4c,param_6,0x1000);
  uVar2 = (uint)param_6 | param_5;
  if (uVar2 == 0x0) {
    param_5 = 0x0;
    uVar2 = 0x0;
  }
  else {
    pass1_1030_88ce((ushort *)CONCAT22(param_6,param_5),param_3,param_4,param_7);
  }
  uStack6 = CONCAT22(uVar2,param_5);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x4);
  (**ppcVar1)();
  if (param_2 != 0x0) {
    pass1_1030_8d08(uStack6,extraout_DX);
  }
  return 0x0;
}



void __stdcall16far pass1_1030_627e(ushort param_1,uint param_2,uint param_3,ulong param_4,ushort *param_5,long param_6)

{
  undefined4 local_12 [0x2];
  ulong uStack10;
  undefined4 uStack6;
  
  uStack6 = 0x0;
  pass1_1030_677a(param_4,param_6,param_1);
  uStack10 = CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    pass1_1030_8b00(uStack10,param_5,(ushort *)CONCAT22(param_1,local_12),param_1);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1030_62e4(ulong *param_1,ushort *param_2,long param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  uint extraout_DX_00;
  uint extraout_DX_01;
  undefined2 uVar5;
  int local_64 [0x3];
  undefined4 uStack94;
  ushort uStack88;
  undefined2 uStack78;
  undefined2 uStack76;
  undefined4 local_40;
  undefined4 uStack60;
  ushort uStack56;
  undefined4 *puStack54;
  uchar *puStack52;
  undefined4 *puStack50;
  uchar *puStack48;
  ushort uStack46;
  int iStack44;
  undefined local_2a [0x2];
  int local_28;
  int local_26;
  ushort local_24;
  undefined local_22 [0x2];
  undefined local_20 [0x2];
  ushort local_1e;
  ushort local_1c;
  ushort local_1a;
  undefined local_18 [0x6];
  undefined local_12 [0x6];
  undefined local_c [0x6];
  ulong uStack6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  puVar2 = *(undefined4 **)param_1;
  puVar4 = *(uchar **)((int)param_1 + 0x2);
  puStack54 = puVar2;
  puStack52 = puVar4;
  puStack50 = puVar2;
  puStack48 = puVar4;
  if (((uint)puVar4 | (uint)puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
    puVar4 = extraout_DX;
  }
  mem_op_1000_179c(0x18,puVar4,0x1000);
  puStack54 = puVar2;
  puStack52 = puVar4;
  if (((uint)puVar4 | (uint)puVar2) == 0x0) {
    puVar2 = (undefined4 *)0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(puVar4,puVar2),0x5,0x5);
    uVar3 = extraout_DX_00;
  }
  *(undefined4 **)param_1 = puVar2;
  *(uint *)((int)param_1 + 0x2) = uVar3;
  pass1_1030_677a((ulong)param_1,param_3,param_4);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | (uint)puVar2) != 0x0) {
    pass1_1008_3e38((ushort *)CONCAT22(param_4,local_c));
    pass1_1008_3e38((ushort *)CONCAT22(param_4,local_12));
    pass1_1008_3e38((ushort *)CONCAT22(param_4,local_18));
    pass1_1008_6d3e(param_2,(ushort *)CONCAT22(param_4,local_12),(ushort *)CONCAT22(param_4,local_c));
    pass1_1008_3eb4((ushort *)CONCAT22(param_4,local_c),(ushort *)CONCAT22(param_4,&local_1e),
                    (ushort *)CONCAT22(param_4,&local_1c),(ushort *)CONCAT22(param_4,&local_1a));
    pass1_1008_3eb4((ushort *)CONCAT22(param_4,local_12),(ushort *)CONCAT22(param_4,&local_24),
                    (ushort *)CONCAT22(param_4,local_22),(ushort *)CONCAT22(param_4,local_20));
    pass1_1008_6d64(param_2,(ushort *)CONCAT22(param_4,local_18));
    pass1_1008_3eb4((ushort *)CONCAT22(param_4,local_18),(ushort *)CONCAT22(param_4,local_2a),
                    (ushort *)CONCAT22(param_4,&local_28),(ushort *)CONCAT22(param_4,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0x0;
      for (uStack46 = local_1c; uVar3 = local_28 + local_1c, (int)uStack46 < (int)uVar3; uStack46 = uStack46 + 0x1) {
        for (uStack56 = local_1a; (int)uStack56 < (int)(local_26 + local_1a); uStack56 = uStack56 + 0x1) {
          uStack88 = local_1e;
          pass1_1008_3e54((ushort *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,local_64)),local_1e,uStack46,
                          uStack56);
          pass1_1030_8b00(uStack6,(ushort *)CONCAT22(param_4,local_64),(ushort *)CONCAT22(param_4,&local_40),param_4);
          uStack60 = local_40;
          local_64[0] = iStack44;
          uStack60._0_2_ = (undefined2)local_40;
          uStack78 = (undefined2)uStack60;
          uStack76 = local_40._2_2_;
          uStack76._1_1_ = (char)((ulong)local_40 >> 0x18);
          if (uStack76._1_1_ == '\0') {
            uStack60._0_2_ = 0x0;
            local_40._2_2_ = 0x0;
          }
          uStack94 = CONCAT22(local_40._2_2_,(undefined2)uStack60);
          ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x8);
          iStack44 = iStack44 + 0x1;
          (**ppcVar1)();
        }
      }
      ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((extraout_DX_01 | uVar3) != 0x0) {
        return;
      }
    }
  }
  return;
}



void __stdcall16far
pass1_1030_64ce(ushort param_1,uint param_2,uint param_3,ulong param_4,ushort *param_5,long param_6,ulong *param_7)

{
  ulong *puVar1;
  uint uVar2;
  ulong local_e;
  ulong uStack10;
  ulong uStack6;
  
  uStack6 = 0x0;
  pass1_1030_677a(param_4,param_6,param_1);
  uStack10 = CONCAT22(param_3,param_2);
  uVar2 = param_3 | param_2;
  if (uVar2 != 0x0) {
    puVar1 = &local_e;
    pass1_1030_8b00(uStack10,param_5,(ushort *)CONCAT22(param_1,puVar1),param_1);
    uStack6 = *puVar1;
  }
  *param_7 = uStack6;
  return;
}



void __stdcall16far pass1_1030_6522(ulong *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  uint extraout_DX_00;
  uint extraout_DX_01;
  undefined2 uVar5;
  undefined local_64 [0xc];
  ushort uStack88;
  undefined4 local_40;
  undefined4 uStack60;
  ushort uStack56;
  undefined4 *puStack54;
  uchar *puStack52;
  undefined4 *puStack50;
  uchar *puStack48;
  ushort uStack46;
  int iStack44;
  undefined local_2a [0x2];
  int local_28;
  int local_26;
  ushort local_24;
  undefined local_22 [0x2];
  undefined local_20 [0x2];
  ushort local_1e;
  ushort local_1c;
  ushort local_1a;
  undefined local_18 [0x6];
  undefined local_12 [0x6];
  undefined local_c [0x6];
  ulong uStack6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  puVar2 = *(undefined4 **)param_1;
  puVar4 = *(uchar **)((int)param_1 + 0x2);
  puStack54 = puVar2;
  puStack52 = puVar4;
  puStack50 = puVar2;
  puStack48 = puVar4;
  if (((uint)puVar4 | (uint)puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
    puVar4 = extraout_DX;
  }
  mem_op_1000_179c(0x18,puVar4,0x1000);
  puStack54 = puVar2;
  puStack52 = puVar4;
  if (((uint)puVar4 | (uint)puVar2) == 0x0) {
    puVar2 = (undefined4 *)0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(puVar4,puVar2),0x5,0x5);
    uVar3 = extraout_DX_00;
  }
  *(undefined4 **)param_1 = puVar2;
  *(uint *)((int)param_1 + 0x2) = uVar3;
  pass1_1030_677a((ulong)param_1,param_3,param_4);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | (uint)puVar2) != 0x0) {
    pass1_1008_3e38((ushort *)CONCAT22(param_4,local_c));
    pass1_1008_3e38((ushort *)CONCAT22(param_4,local_12));
    pass1_1008_3e38((ushort *)CONCAT22(param_4,local_18));
    pass1_1008_6d3e((ushort *)param_2,(ushort *)CONCAT22(param_4,local_12),(ushort *)CONCAT22(param_4,local_c));
    pass1_1008_3eb4((ushort *)CONCAT22(param_4,local_c),(ushort *)CONCAT22(param_4,&local_1e),
                    (ushort *)CONCAT22(param_4,&local_1c),(ushort *)CONCAT22(param_4,&local_1a));
    pass1_1008_3eb4((ushort *)CONCAT22(param_4,local_12),(ushort *)CONCAT22(param_4,&local_24),
                    (ushort *)CONCAT22(param_4,local_22),(ushort *)CONCAT22(param_4,local_20));
    pass1_1008_6d64((ushort *)param_2,(ushort *)CONCAT22(param_4,local_18));
    pass1_1008_3eb4((ushort *)CONCAT22(param_4,local_18),(ushort *)CONCAT22(param_4,local_2a),
                    (ushort *)CONCAT22(param_4,&local_28),(ushort *)CONCAT22(param_4,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0x0;
      for (uStack46 = local_1c; uVar3 = local_28 + local_1c, (int)uStack46 < (int)uVar3; uStack46 = uStack46 + 0x1) {
        for (uStack56 = local_1a; (int)uStack56 < (int)(local_26 + local_1a); uStack56 = uStack56 + 0x1) {
          uStack88 = local_1e;
          pass1_1008_3e54((ushort *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,local_64)),local_1e,uStack46,
                          uStack56);
          pass1_1030_8b00(uStack6,(ushort *)CONCAT22(param_4,local_64),(ushort *)CONCAT22(param_4,&local_40),param_4);
          uStack60 = local_40;
          iStack44 = iStack44 + 0x1;
          ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x8);
          (**ppcVar1)();
        }
      }
      ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((extraout_DX_01 | uVar3) != 0x0) {
        return;
      }
    }
  }
  return;
}



void __stdcall16far pass1_1030_66de(ulong param_1,ulong param_2,ushort param_3)

{
  ulong uVar1;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x4));
  while( true ) {
    uVar1 = pass1_1008_5b12(local_a,param_3);
    if (uVar1 == 0x0) break;
    pass1_1030_8bac(uVar1,(ushort)param_2);
  }
  return;
}



void __stdcall16far
pass1_1030_671c(ulong param_1,ulong param_2,ushort *param_3,long param_4,ushort param_5,ushort param_6,int param_7,
               ushort param_8)

{
  pass1_1030_677a(param_1,param_4,param_8);
  pass1_1030_8bdc(CONCAT22(param_6,param_5),param_2,param_3,param_7,param_8);
  return;
}



void __stdcall16far pass1_1030_6740(ulong param_1,ushort param_2,int param_3)

{
  ulong uVar1;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_2,local_a),*(ulong *)((int)param_1 + 0x4));
  while( true ) {
    uVar1 = pass1_1008_5b12(local_a,param_2);
    if (uVar1 == 0x0) break;
    pass1_1030_8c38(uVar1,param_3,param_2);
  }
  return;
}



void __stdcall16far pass1_1030_677a(ulong param_1,long param_2,ushort param_3)

{
  undefined *puVar1;
  uint extraout_DX;
  undefined2 uVar2;
  undefined local_a [0x8];
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x4) == 0x0) {
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x4));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
  } while (*(long *)(puVar1 + 0x24) != param_2);
  return;
}



void __stdcall16far pass1_1030_67cc(ushort *param_1)

{
  astruct_687 *iVar1;
  undefined2 uVar1;
  
  struct_1030_1628(param_1);
  iVar1 = (astruct_687 *)param_1;
  iVar1 = (astruct_687 *)&iVar1->field_0xc;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x1a = 0x0;
  iVar1->field_0x1e = 0x0;
  iVar1->field_0x22 = 0x0;
  iVar1->field_0x26 = 0x0;
  iVar1->field_0x2a = 0x0;
  iVar1->field_0x2e = 0x0;
  iVar1->field_0x32 = 0x0;
  iVar1->field_0x34 = 0x0;
  iVar1->field_0x38 = 0x0;
  iVar1->field_0x36 = 0x0;
  iVar1->field_0x3c = 0x0;
  iVar1->field_0x3a = 0x0;
  iVar1->field_0x40 = 0x0;
  iVar1->field_0x3e = 0x0;
  *param_1 = 0x8114;
  iVar1->field_0x2 = 0x1030;
  return;
}



void __stdcall16far
pass1_1030_684c(ushort *param_1,ulong *param_2,ushort param_3,ushort param_4,ushort param_5,ulong param_6,ushort param_7
               )

{
  int iVar1;
  undefined2 uVar2;
  
  pass1_1030_165e(param_1,0x5000000,param_6,param_7);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0xc) = *param_2;
  *(undefined2 *)(iVar1 + 0x10) = *(undefined2 *)(param_2 + 0x1);
  *(ushort *)(iVar1 + 0x12) = param_4;
  *(ushort *)(iVar1 + 0x14) = param_4;
  *(undefined4 *)(iVar1 + 0x16) = 0x0;
  *(undefined4 *)(iVar1 + 0x1a) = 0x0;
  *(undefined4 *)(iVar1 + 0x1e) = 0x0;
  *(undefined4 *)(iVar1 + 0x22) = 0x0;
  *(undefined4 *)(iVar1 + 0x26) = 0x0;
  *(undefined4 *)(iVar1 + 0x2a) = 0x0;
  *(undefined4 *)(iVar1 + 0x2e) = 0x0;
  *(undefined2 *)(iVar1 + 0x32) = 0x0;
  *(undefined2 *)(iVar1 + 0x34) = 0x0;
  *(undefined4 *)(iVar1 + 0x36) = 0x0;
  *(undefined4 *)(iVar1 + 0x3a) = 0x0;
  *(undefined4 *)(iVar1 + 0x3e) = 0x0;
  *param_1 = 0x8114;
  *(undefined2 *)(iVar1 + 0x2) = 0x1030;
  return;
}



void __stdcall16far pass1_1030_68dc(ushort *param_1,ushort param_2)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  astruct_18 *paVar4;
  code **ppcVar5;
  astruct_611 *iVar6;
  undefined2 uVar6;
  astruct_18 *paStack10;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_611 *)param_1;
  *param_1 = 0x8114;
  iVar6->field_0x2 = 0x1030;
  paVar4 = *(astruct_18 **)&iVar6->field_0x22;
  uVar1 = iVar6->field_0x24;
  if ((uVar1 | (uint)paVar4) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)((ulong)paVar4 & 0xffff | (ulong)uVar1 << 0x10));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar4,0x1000);
  }
  uVar1 = iVar6->field_0x26;
  uVar2 = iVar6->field_0x28;
  paStack10 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)CONCAT22(uVar2,uVar1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  puVar3 = iVar6->field_0x1e;
  uVar1 = iVar6->field_0x20;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field_0x36;
  uVar1 = iVar6->field_0x38;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field_0x3a;
  uVar1 = iVar6->field_0x3c;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field_0x3e;
  uVar1 = iVar6->field_0x40;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  pass1_1030_16b2(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_69cc(ulong param_1,uint param_2,uint param_3,ushort param_4)

{
  ushort uVar1;
  BOOL16 BVar2;
  int iVar3;
  undefined2 uVar4;
  ulong uVar5;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x3e) != 0x0) {
    return;
  }
  if ((*(long *)(iVar3 + 0x22) != 0x0) && (pass1_1020_ba94(*(long **)(iVar3 + 0x22)), (param_3 | param_2) != 0x0)) {
    return;
  }
  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x4);
  if ((BVar2 != 0x0) &&
     (uVar5 = pass1_1028_67d4(*(ulong *)(iVar3 + 0x1a),param_4), ((uint)(uVar5 >> 0x10) | (uint)uVar5) != 0x0)) {
    return;
  }
  return;
}
