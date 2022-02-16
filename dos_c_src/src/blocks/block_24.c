
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far string_1008_e586(ushort param_1,ushort param_2,ulong param_3,uint param_4,uint param_5)

{
  uint uVar1;
  uchar *puVar2;
  char *in_string_2;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_3,(uint)(param_3 >> 0x10));
  puVar2 = (uchar *)(param_5 | param_4);
  if (puVar2 == (uchar *)0x0) {
    return 0x0;
  }
  uVar1 = param_4;
  mem_op_1000_179c(0x80,puVar2,0x1000);
  in_string_2 = pass1_1038_4d28(CONCAT22(param_5,param_4));
  unk_str_op_1000_3d3e((char *)CONCAT22(puVar2,uVar1),in_string_2);
  return CONCAT22(puVar2,uVar1);
}



void __stdcall16far pass1_1008_e5da(ulong param_1,ulong param_2,HFILE16 param_3,uint16_t param_4)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  undefined2 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined4 local_30 [0x2];
  undefined4 local_28;
  undefined4 local_24 [0x2];
  undefined2 local_1c [0x3];
  undefined2 local_16 [0x3];
  undefined4 uStack16;
  undefined local_c [0x8];
  undefined2 uStack4;
  
  BVar2 = write_to_file_1008_7cac(param_2,param_4);
  if (BVar2 != 0x0) {
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if (*(long *)(iVar4 + 0xa) == 0x0) {
      uStack4 = 0x0;
    }
    else {
      uVar1 = *(undefined4 *)(iVar4 + 0xa);
      uStack4 = *(undefined2 *)((int)uVar1 + 0x8);
    }
    local_1c[0] = uStack4;
    uVar6 = (ushort)param_2;
    uVar7 = (ushort)(param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_1c,param_4,(char *)0x2,param_3);
    if (BVar2 != 0x0) {
      pass1_1008_5784((ulong *)CONCAT22(param_4,local_c),*(ulong *)(iVar4 + 0xa));
      do {
        puVar3 = local_c;
        pass1_1008_5b12(puVar3,param_4);
        uStack16 = CONCAT22(extraout_DX,puVar3);
        if ((extraout_DX | (uint)puVar3) == 0x0) {
          return;
        }
        local_24[0] = *(undefined4 *)(puVar3 + 0x4);
        BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_24,param_4,(char *)0x4,param_3);
        if (BVar2 == 0x0) break;
        local_28 = *(undefined4 *)((int)uStack16 + 0x8);
        BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)&local_28,param_4,(char *)0x4,param_3);
        if (BVar2 == 0x0) break;
        local_16[0] = *(undefined2 *)((int)uStack16 + 0xc);
        BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_16,param_4,(char *)0x2,param_3);
        if (BVar2 == 0x0) break;
        local_30[0] = *(undefined4 *)((int)uStack16 + 0xe);
        BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_30,param_4,(char *)0x4,param_3);
        if (BVar2 == 0x0) break;
        local_16[0] = *(undefined2 *)((int)uStack16 + 0x12);
        BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_16,param_4,(char *)0x2,param_3);
      } while (BVar2 != 0x0);
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far
file_1008_e70e(ulong param_1,ulong param_2,int param_3,uchar *param_4,uint16_t param_5,uint16_t param_6)

{
  undefined4 uVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  uint uVar4;
  uchar *extraout_DX;
  ushort uVar5;
  undefined2 uVar6;
  ushort uVar7;
  ushort uVar8;
  undefined2 uVar9;
  undefined2 local_12 [0x2];
  undefined4 *puStack14;
  uint uStack10;
  uint local_4;
  
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  uVar7 = (ushort)param_2;
  uVar8 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar7,uVar8,0x14,param_5,param_6);
  if (param_3 != 0x0) {
    BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)&local_4,0x0,param_6,0x2,param_5);
    if (BVar3 != 0x0) {
      if (local_4 == 0x0) {
        return;
      }
      uStack10 = 0x0;
      while( true ) {
        if (local_4 <= uStack10) {
          return;
        }
        uVar9 = 0x14;
        uVar4 = local_4;
        mem_op_1000_179c(0x14,param_4,0x1000);
        uVar5 = (uint)param_4 | uVar4;
        if (uVar5 == 0x0) {
          uVar4 = 0x0;
          uVar5 = 0x0;
        }
        else {
          struct_1008_dcdc((ushort *)CONCAT22(param_4,uVar4));
        }
        puStack14 = (undefined4 *)CONCAT22(uVar5,uVar4);
        BVar3 = read_file_1008_7dee(uVar7,uVar8,uVar4 + 0x4,0x0,uVar5,0x4,0x1000);
        if ((((BVar3 == 0x0) ||
             (BVar3 = read_file_1008_7dee(uVar7,uVar8,(int)puStack14 + 0x8,0x0,(ushort)((ulong)puStack14 >> 0x10),0x4,
                                          0x1000), BVar3 == 0x0)) ||
            (BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)local_12,0x0,param_6,0x2,0x1000), BVar3 == 0x0)) ||
           ((BVar3 = read_file_1008_7dee(uVar7,uVar8,(int)puStack14 + 0xe,0x0,(ushort)((ulong)puStack14 >> 0x10),0x4,
                                         0x1000), BVar3 == 0x0 ||
            (BVar3 = read_file_1008_7dee(uVar7,uVar8,(int)puStack14 + 0x12,0x0,(ushort)((ulong)puStack14 >> 0x10),0x2,
                                         0x1000), BVar3 == 0x0)))) break;
        uVar9 = (undefined2)((ulong)puStack14 >> 0x10);
        *(undefined2 *)((int)puStack14 + 0xc) = local_12[0];
        uVar6 = (undefined2)(param_1 >> 0x10);
        uVar1 = *(undefined4 *)((int)param_1 + 0xa);
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xa) + 0x4);
        (**ppcVar2)(0x0,(int)uVar1,(int)((ulong)uVar1 >> 0x10),(int)puStack14,uVar9);
        uStack10 = uStack10 + 0x1;
        param_4 = extraout_DX;
      }
      if (puStack14 != (undefined4 *)0x0) {
        ppcVar2 = (code **)*puStack14;
        (**ppcVar2)(0x1000,(int)puStack14,(int)((ulong)puStack14 >> 0x10),0x1,uVar9,puStack14);
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



void __stdcall16far pass1_1008_e852(ushort param_1,ushort param_2,ulong param_3,ushort param_4,uint param_5)

{
  undefined *puVar1;
  int iVar2;
  char *pcVar3;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
  do {
    puVar1 = local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar1));
    if ((param_5 | (uint)puVar1) == 0x0) {
      return;
    }
    pcVar3 = pass1_1038_4d28(CONCAT22(param_5,puVar1));
    param_5 = (uint)((ulong)pcVar3 >> 0x10);
    iVar2 = pass1_1000_3d7a(param_3,(ulong)pcVar3 & 0xffff | (ulong)param_5 << 0x10);
  } while (iVar2 != 0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

long __stdcall16far pass1_1008_e8cc(ushort param_1,ulong param_2,ulong param_3,ulong param_4)

{
  undefined4 uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  long lVar7;
  char *pcVar8;
  char *pcVar9;
  ulong uStack22;
  ulong uStack18;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_1,local_a),*(ulong *)((int)param_2 + 0xa));
  while( true ) {
    lVar7 = pass1_1008_5b12(local_a,param_1);
    uVar5 = (uint)((ulong)lVar7 >> 0x10);
    uVar2 = (uint)lVar7;
    uVar6 = uVar5 | uVar2;
    if (lVar7 == 0x0) {
      return 0x0;
    }
    uVar1 = *(undefined4 *)(uVar2 + 0x4);
    uVar3 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    uStack18 = CONCAT22(uVar6,uVar3);
    uVar1 = *(undefined4 *)(uVar2 + 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    uStack22 = CONCAT22(uVar6,uVar3);
    pcVar8 = pass1_1038_4d28(uStack18);
    pcVar9 = pass1_1038_4d28(uStack22);
    iVar4 = pass1_1000_3d7a(param_4,(ulong)pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3,(ulong)pcVar9), iVar4 == 0x0)) break;
    iVar4 = pass1_1000_3d7a(param_3,(ulong)pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4,(ulong)pcVar9), iVar4 == 0x0)) {
      return lVar7;
    }
  }
  return lVar7;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_e9a4(ushort param_1,ushort param_2,ulong param_3,int param_4,ushort param_5)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  uchar *puVar4;
  int iVar5;
  undefined2 uVar6;
  ulong uVar7;
  int iStack20;
  ulong uStack16;
  ulong uStack6;
  
  uVar7 = pass1_1030_8326();
  uVar6 = (undefined2)(param_3 >> 0x10);
  iVar5 = (int)param_3;
  puVar1 = (uint *)(iVar5 + 0xe);
  uVar2 = (uint)uVar7 - *puVar1;
  puVar4 = (uchar *)(((int)(uVar7 >> 0x10) - *(int *)(iVar5 + 0x10)) - (uint)((uint)uVar7 < *puVar1));
  uStack6 = CONCAT22(puVar4,uVar2);
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,puVar4,param_4);
  uStack16 = 0x0;
  if (((int)PTR_LOOP_1050_13ae < 0x1) || (SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) goto LAB_1008_ea2b;
  if (PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1) {
    if (*(int *)(iVar5 + 0x12) == 0x0) {
LAB_1008_ea20:
      uVar3 = 0x1e;
    }
    else {
      uVar3 = 0xa;
    }
  }
  else {
    if (PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
      if (*(int *)(iVar5 + 0x12) == 0x0) {
        uVar3 = 0x28;
      }
      else {
        uVar3 = 0x14;
      }
    }
    else {
      if (PTR_LOOP_1050_13ae != (undefined *)&DAT_1050_0004) goto LAB_1008_ea2b;
      if (*(int *)(iVar5 + 0x12) != 0x0) goto LAB_1008_ea20;
      uVar3 = 0x32;
    }
  }
  uStack16 = (ulong)uVar3;
LAB_1008_ea2b:
  if (uStack16 < uStack6) {
    pass1_1008_612e(0x1,0x64,uVar2);
    iStack20 = 0x0;
    iVar5 = *(int *)(iVar5 + 0xc);
    if (iVar5 == 0x2) {
      iStack20 = 0x32;
    }
    else {
      if (iVar5 == 0x3) {
        iStack20 = 0x19;
      }
    }
    if ((int)uStack6 < iStack20) {
      return;
    }
  }
  return;
}



ushort * __stdcall16far pass1_1008_ea86(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1008_ddca(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



undefined2 * __stdcall16far pass1_1008_eabc(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined2 *)(param_1 + 0xa) = 0x0;
  pass1_1008_3e38((ushort *)CONCAT22(param_2,param_1 + 0xc));
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xeb1a;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  return (undefined2 *)CONCAT22(param_2,param_1);
}



ushort * __stdcall16far pass1_1008_eaf4(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_eb2a(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined2 *)(param_1 + 0xa) = 0x0;
  *(undefined4 *)(param_1 + 0xc) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xec00;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  return;
}



ulong __stdcall16far pass1_1008_eb5c(ushort param_1,ushort param_2,int param_3)

{
  return CONCAT22(0x1050,param_3 * 0x10 + 0xd0e);
}



ushort __stdcall16far pass1_1008_eb6e(void)

{
  return 0x5;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_eb74(undefined4 param_1,int param_2,uchar *param_3,int param_4,ushort param_5)

{
  *(int *)((int)param_1 + 0xa) = param_2;
  if (param_2 != 0x0) {
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_5,param_3,param_4);
    pass1_1010_c312();
  }
  return;
}



ushort * __stdcall16far pass1_1008_ebda(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_ec10(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined2 *)(param_1 + 0xa) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xec62;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort * __stdcall16far pass1_1008_ec3c(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1008_ec72(ushort *param_1)

{
  struct_1010_383a(param_1);
  *param_1 = 0xefc4;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return param_1;
}



void __stdcall16far pass1_1008_ec94(ushort *param_1)

{
  *param_1 = 0xefc4;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  pass1_1010_3880(param_1);
  return;
}



undefined4 __stdcall16far struct_1008_ecb2(astruct_221 *param_1,ushort param_2,ushort param_3)

{
  uint in_AX;
  uchar *in_DX;
  undefined2 unaff_SS;
  
  struct_1010_2cd2((astruct_79 *)param_1,(astruct_79 *)param_2,param_3,unaff_SS);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xef9c;
  param_1->field_0x2 = 0x1008;
  mem_op_1000_179c(0x20c,in_DX,0x1000);
  param_1->field_0x5c = in_AX;
  param_1->field_0x5e = (int)in_DX;
  pass1_1000_4906((astruct_20 *)CONCAT22(in_DX,param_1->field_0x5c),(WNDCLASS16 *)0x0,0x20c);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1008_ed00(ushort *param_1,ushort param_2)

{
  *param_1 = 0xef9c;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  pass1_1010_2db2(param_1,param_2);
  return;
}



void __stdcall16far mem_1008_ed1e(ushort param_1,ushort param_2,int param_3,uint param_4,uchar *param_5)

{
  if (param_3 != 0x0) {
    mem_op_1000_179c(param_3 << 0x2,param_5,0x1000);
    return;
  }
  mem_op_1000_179c(0x1a,param_5,0x1000);
  if (((uint)param_5 | param_4) != 0x0) {
    struct_1008_ec72((ushort *)CONCAT22(param_5,param_4));
    return;
  }
  return;
}



void __stdcall16far pass1_1008_ed62(ulong param_1,int param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(int *)(iVar1 + 0x16) = param_2 * 0x8 + 0xd5e;
  *(undefined2 *)(iVar1 + 0x18) = (int)&USHORT_1050_1050;
  *(undefined2 *)(iVar1 + 0x12) = *(undefined2 *)(param_2 * 0x8 + 0xd64);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1008_ed8a(ulong *param_1,uint param_2,int param_3,int param_4,int param_5,int param_6,ushort param_7)

{
  code **ppcVar1;
  char cVar2;
  uint uVar3;
  uint uVar4;
  bool bVar5;
  undefined4 uVar6;
  ulong uVar7;
  
  if (0x0 < param_4) {
    if (_PTR_LOOP_1050_0df6 == (ushort *)0x0) {
      ppcVar1 = (code **)((int)*param_1 + 0x18);
      uVar6 = (**ppcVar1)();
      _PTR_LOOP_1050_0df6 =
           mixed_1010_20ba(_PTR_LOOP_1050_0ed0,(ushort)uVar6,param_7,(uchar *)((ulong)uVar6 >> 0x10),param_6);
    }
    uVar6 = *(undefined4 *)((int)param_1 + 0xc);
    uVar7 = pass1_1010_2e02((ulong)_PTR_LOOP_1050_0df6,*(int *)((int)uVar6 + 0x12));
    uVar3 = param_2 + 0x1;
    uVar4 = param_3 + (uint)(0xfffe < param_2);
    for (cVar2 = ((char)param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 = cVar2 + -0x1) {
      bVar5 = CARRY2(uVar3,uVar3);
      uVar3 = uVar3 * 0x2;
      uVar4 = uVar4 * 0x2 + (uint)bVar5;
    }
    pass1_1010_2e30((ulong)_PTR_LOOP_1050_0df6,uVar3 | (uint)uVar7,uVar4 | (uint)(uVar7 >> 0x10),
                    *(int *)(param_5 * 0x8 + 0xd64));
  }
  return;
}



void __stdcall16far pass1_1008_ee14(ulong param_1,ushort param_2)

{
  undefined *puVar1;
  undefined2 uVar2;
  int iVar3;
  undefined2 uVar4;
  ushort *puVar5;
  undefined local_1c [0x1a];
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x56) == 0x0) {
    puVar5 = struct_1008_ec72((ushort *)CONCAT22(param_2,local_1c));
    uVar2 = (undefined2)((ulong)puVar5 >> 0x10);
    puVar1 = local_1c;
    pass1_1010_398e((ulong *)CONCAT22(param_2,puVar1),0x0,0x0,0x0,(ushort)puVar1);
    *(ushort *)(iVar3 + 0x56) = (ushort)puVar1;
    *(undefined2 *)(iVar3 + 0x58) = uVar2;
    pass1_1008_ec94((ushort *)CONCAT22(param_2,local_1c));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __stdcall16far load_string_1008_ee56(void)

{
  char *pcVar1;
  
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  return pcVar1;
}



void __stdcall16far pass1_1008_ee72(ushort param_1,ushort param_2,int param_3)

{
  code **ppcVar1;
  ulong uVar2;
  
  if (*(long *)(param_1 + 0x56) == 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x10);
    (**ppcVar1)();
  }
  uVar2 = pass1_1010_2e02(CONCAT22(param_2,param_1),param_3);
  pass1_1010_2e5c(param_1,param_2,uVar2);
  return;
}



ushort __stdcall16far pass1_1008_eea6(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool __stdcall16far
pass1_1008_eeac(ushort param_1,ushort param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  ushort uVar1;
  char cVar2;
  uint uVar3;
  ushort uVar4;
  ushort uVar5;
  ushort *puVar6;
  uint uVar7;
  
  uVar7 = *(uint *)((int)param_3 + 0x12);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_4,param_5);
  uVar4 = (ushort)((ulong)puVar6 >> 0x10);
  uVar1 = (ushort)puVar6;
  uVar5 = uVar4;
  if (uVar7 == 0x7d) {
    pass1_1010_a5ca(uVar1,uVar4,0x7c,0x7d,uVar4);
    if (uVar7 != 0x0) {
      return false;
    }
    pass1_1010_a5ca(uVar1,uVar4,0x7d,0x0,uVar5);
    if (uVar7 != 0x0) {
      return false;
    }
    uVar3 = uVar7;
    uVar7 = 0x78;
  }
  else {
    uVar3 = uVar7;
    if (uVar7 < 0x7e) {
      cVar2 = (char)uVar7;
      uVar3 = uVar7 & 0xff00;
      if ((byte)(cVar2 + 0x8dU) == 0x0) {
        uVar7 = 0x9;
        uVar3 = uVar3 | (byte)(cVar2 + 0x8dU);
      }
      else {
        if ((byte)(cVar2 + 0x89U) == 0x0) {
          uVar7 = 0x2e;
          uVar3 = uVar3 | (byte)(cVar2 + 0x89U);
        }
        else {
          uVar3 = uVar3 | (byte)(cVar2 + 0x87U);
          if ((byte)(cVar2 + 0x87U) == 0x0) {
            uVar7 = 0x5b;
          }
        }
      }
    }
  }
  pass1_1010_a5ca(uVar1,uVar4,uVar7,uVar3,uVar5);
  return uVar3 == 0x0;
}



ushort __stdcall16far pass1_1008_ef38(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x16);
  return *(ushort *)((int)uVar1 + 0x2);
}



ushort __stdcall16far pass1_1008_ef4a(void)

{
  return 0x41;
}



ushort * __stdcall16far pass1_1008_ef50(ushort *param_1,byte param_2)

{
  pass1_1008_ec94(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1008_ef76(astruct_18 *param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_ed00(&param_1->field_0x0,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_0000(astruct_645 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int unaff_DI;
  astruct_79 *paVar1;
  ushort *puVar2;
  undefined2 *puVar3;
  ushort uVar4;
  undefined2 *puVar5;
  ushort uVar6;
  
                    // Segment:    3
                    // Offset:     00015420
                    // Length:     ee9f
                    // Min Alloc:  ee9f
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  paVar1 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xc = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x2c8;
  param_1->field_0x2 = 0x1010;
  puVar5 = &param_1->field_0xa;
  puVar3 = &param_1->field_0xc;
  uVar4 = param_2;
  uVar6 = param_2;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,(uchar *)((ulong)paVar1 >> 0x10),unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)),(ushort *)CONCAT22(uVar4,puVar3),
                  (ushort *)CONCAT22(uVar6,puVar5));
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1010_0052(ushort *param_1,ushort param_2)

{
  *param_1 = 0x2c8;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far
set_window_placement_1010_0070(ulong param_1,int param_2,ushort param_3,HWND16 param_4,ushort param_5)

{
  code **ppcVar1;
  undefined2 uVar2;
  undefined4 *puVar3;
  long lVar4;
  undefined local_18 [0x6];
  INT16 IStack18;
  int iStack16;
  INT16 IStack14;
  INT16 IStack12;
  INT16 IStack10;
  INT16 IStack8;
  undefined2 uStack6;
  undefined2 uStack4;
  
  local_18._0_2_ = 0x16;
  local_18._2_4_ = 0x0;
  IStack18 = 0x0;
  iStack16 = 0x0;
  IStack14 = 0x0;
  IStack12 = 0x0;
  IStack10 = 0x0;
  IStack8 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  GetWindowPlacement16(param_4,(WINDOWPLACEMENT16 *)local_18);
  if ((iStack16 == -0x1) || (param_2 != 0x0)) {
    local_18._2_4_ = 0x50001;
    lVar4 = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538,0x0);
    uVar2 = (undefined2)((ulong)lVar4 >> 0x10);
    puVar3 = (undefined4 *)*(undefined4 *)((int)lVar4 + 0xe0);
    ppcVar1 = (code **)((int)*puVar3 + 0x38);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puVar3,*(undefined2 *)((int)lVar4 + 0xe2),param_3);
    pass1_1010_01f8(param_1,CONCAT22(param_5,local_18),(int)puVar3);
    SetWindowPlacement16((HWND16)s_tile2_bmp_1050_1538,(WINDOWPLACEMENT16 *)local_18);
  }
  return;
}



void __stdcall16far set_win_placement_1010_010e(ushort param_1,ushort param_2,ushort param_3,HWND16 param_4)

{
  code **ppcVar1;
  int iVar2;
  int *piVar3;
  undefined2 uVar4;
  undefined4 *puVar5;
  uint extraout_DX;
  long lVar6;
  WINDOWPLACEMENT16 local_18;
  int iStack6;
  int iStack4;
  
  local_18.length = 0x16;
  local_18.flags = 0x0;
  local_18.show_cmd = 0x0;
  local_18.pt_min_position.x = 0x0;
  local_18.pt_min_position.y = 0x0;
  local_18.pt_max_position.x = 0x0;
  local_18.pt_max_position.y = 0x0;
  local_18.rc_normal_position.x = 0x0;
  local_18.rc_normal_position.y = 0x0;
  iStack6 = 0x0;
  iStack4 = 0x0;
  GetWindowPlacement16(param_4,&local_18);
  if (local_18.rc_normal_position.x == -0x1) {
    lVar6 = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538,0x0);
    uVar4 = (undefined2)((ulong)lVar6 >> 0x10);
    puVar5 = (undefined4 *)*(undefined4 *)((int)lVar6 + 0xe0);
    ppcVar1 = (code **)((int)*puVar5 + 0x1c);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puVar5,*(undefined2 *)((int)lVar6 + 0xe2),param_3);
    iVar2 = (int)puVar5;
    piVar3 = (int *)((ulong)puVar5 & 0xffff | (ulong)extraout_DX << 0x10);
    local_18.show_cmd = 0x9;
    local_18.rc_normal_position.x = *piVar3;
    local_18.rc_normal_position.y = *(INT16 *)(iVar2 + 0x2);
    iStack6 = *(int *)(iVar2 + 0x4) + *piVar3;
    iStack4 = *(int *)(iVar2 + 0x2) + *(int *)(iVar2 + 0x6);
    SetWindowPlacement16((HWND16)s_tile2_bmp_1050_1538,&local_18);
  }
  return;
}



void __stdcall16far enum_child_windows_1010_01be(LPVOID param_1)

{
  LPVOID pvVar1;
  
  if (PTR_LOOP_1050_0010 == (undefined *)0x0) {
    pvVar1 = MakeProcInstance16(param_1,(HANDLE16)PTR_LOOP_1050_038c);
    EnumChildWindows1((HWND16)s_tile2_bmp_1050_1538,(LPVOID)0x0,ZEXT24(pvVar1) << 0x10);
    FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
  }
  return;
}



void __stdcall16far pass1_1010_01f8(ulong param_1,ulong param_2,int param_3)

{
  int iVar1;
  int iVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  
  iVar2 = *(int *)(param_3 * 0x4 + 0xe02) * 0x4;
  iVar1 = *(int *)(iVar2 + 0xdfc);
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar4 = (undefined2)(param_2 >> 0x10);
  *(int *)((int)param_2 + 0x6) =
       *(int *)(param_3 * 0x4 + 0xe04) * 0x28 + *(int *)(iVar2 + 0xdfa) + *(int *)((int)param_1 + 0xa);
  *(int *)((int)param_2 + 0x8) = *(int *)((int)param_1 + 0xc) + iVar1;
  return;
}



BOOL16 __stdcall16far win_ui_op_1010_0240(ushort param_1,ushort param_2,ushort param_3,HWND16 param_4,ushort param_5)

{
  code **ppcVar1;
  BOOL16 BVar2;
  WORD WVar3;
  uchar *in_DX;
  int unaff_DI;
  undefined4 *puVar4;
  ushort uVar5;
  ushort uVar6;
  undefined2 uVar7;
  
  uVar7 = SUB42(&USHORT_1050_1050,0x0);
  uVar6 = param_3;
  BVar2 = IsWindow16(param_4);
  if (BVar2 != 0x0) {
    WVar3 = GetWindowWord16((HWND16)s_tile2_bmp_1050_1538,-0x6);
    if (WVar3 == *(WORD *)&PTR_LOOP_1050_038c) {
      uVar5 = param_3;
      BVar2 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
      if (BVar2 != 0x0) {
        puVar4 = (undefined4 *)mixed_1010_20ba(*(ulong *)&PTR_LOOP_1050_0ed0,0x45,param_5,in_DX,unaff_DI);
        ppcVar1 = (code **)((int)*puVar4 + 0x10);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538,puVar4,0x1,param_3,uVar5,uVar6,uVar7);
      }
    }
  }
  return 0x1;
}



ushort * __stdcall16far pass1_1010_02a2(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_0052(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1010_02e0(astruct_79 *param_1,astruct_79 *param_2,ushort param_3)

{
  uint uVar1;
  uchar *puVar2;
  ushort extraout_DX;
  astruct_79 *paVar3;
  
  paVar3 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar2 = (uchar *)((ulong)paVar3 >> 0x10);
  uVar1 = 0x0;
  *(undefined4 *)(param_1 + 0x1) = 0x0;
  *(undefined2 *)&param_1[0x1].field_0x4 = 0x0;
  *(undefined2 *)((int)&param_1[0x1].field_0x4 + 0x2) = 0x0;
  *(undefined2 *)&param_1[0x2].field_0x4 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xe98;
  param_1->field_0x2 = 0x1010;
  mem_op_1000_179c(0xc,puVar2,0x1000);
  if (((uint)puVar2 | uVar1) == 0x0) {
    *(undefined4 *)(param_1 + 0x1) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2,uVar1));
    ((astruct_79 *)(param_1 + 0x1))->field_0x0 = uVar1;
    param_1[0x1].field_0x2 = extraout_DX;
  }
  return;
}



void __stdcall16far pass1_1010_0350(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_474 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_474 *)param_1;
  *param_1 = 0xe98;
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
