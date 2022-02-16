void __stdcall16far pass1_1030_1eee(ulong param_1,ulong param_2)

{
  ulong uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(ulong *)(iVar2 + 0xc);
  param_2._2_2_ = *(undefined2 *)(iVar2 + 0xe);
  if (uVar1 < param_2) {
    uVar1 = param_2 & 0xffff;
  }
  *(undefined2 *)(iVar2 + 0xc) = (int)uVar1;
  *(undefined2 *)(iVar2 + 0xe) = param_2._2_2_;
  return;
}



void __stdcall16far pass1_1030_1f16(ulong *param_1,ulong param_2)

{
  long *plVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((*(long *)(iVar4 + 0x4) == 0x0) || (*(ulong *)(iVar4 + 0x10) <= *(ulong *)(iVar4 + 0x8))) {
    ppcVar2 = (code **)((int)*param_1 + 0x20);
    (**ppcVar2)();
  }
  uVar3 = *(undefined4 *)(iVar4 + 0x4);
  *(ulong *)(*(int *)(iVar4 + 0x8) * 0x4 + (int)uVar3) = param_2;
  plVar1 = (long *)(iVar4 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_1f77(int param_1,int param_2,ushort param_3,ushort param_4)

{
  uint *puVar1;
  uint uVar2;
  undefined4 uVar3;
  int iVar4;
  ushort uVar5;
  undefined2 uVar6;
  long lVar7;
  
  if (*(long *)(param_1 + 0x10) == 0x0) {
    iVar4 = *(int *)(param_1 + 0xc);
    PTR_LOOP_1050_5f2e = (undefined *)*(undefined2 *)(param_1 + 0xe);
  }
  else {
    uVar2 = *(uint *)(param_1 + 0x10);
    puVar1 = (uint *)(param_1 + 0x14);
    iVar4 = uVar2 + *puVar1;
    PTR_LOOP_1050_5f2e =
         (undefined *)(*(int *)(param_1 + 0x12) + *(int *)(param_1 + 0x16) + (uint)CARRY2(uVar2,*puVar1));
  }
  *(int *)(param_2 + -0x4) = iVar4;
  *(undefined2 *)(param_2 + -0x2) = PTR_LOOP_1050_5f2e;
  *(undefined4 *)(param_2 + -0x8) = 0x0;
  if (*(long *)(param_1 + 0x4) == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar5 = fn_ptr_op_1000_1708(*(int *)(param_2 + -0x4) << 0x2,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,
                                (uint)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    uVar3 = *(undefined4 *)(param_1 + 0x4);
    uVar2 = *(uint *)(param_2 + -0x4);
    lVar7 = pass1_1000_0ed4(0x1000,param_4,0x1,uVar2 * 0x4,
                            ((int)PTR_LOOP_1050_5f2e * 0x2 + (uint)CARRY2(uVar2,uVar2)) * 0x2 +
                            (uint)CARRY2(uVar2 * 0x2,uVar2 * 0x2),(ushort *)uVar3,(ushort)((ulong)uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar7 >> 0x10);
    uVar5 = (ushort)lVar7;
  }
  *(ushort *)(param_2 + -0x8) = uVar5;
  *(undefined2 *)(param_2 + -0x6) = PTR_LOOP_1050_5f2e;
  if (((uint)PTR_LOOP_1050_5f2e | *(uint *)(param_2 + -0x8)) != 0x0) {
    uVar3 = *(undefined4 *)(param_2 + 0x6);
    uVar6 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar4 = (int)uVar3;
    *(undefined4 *)(iVar4 + 0x10) = *(undefined4 *)(param_2 + -0x4);
    *(undefined4 *)(iVar4 + 0x4) = *(undefined4 *)(param_2 + -0x8);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_201e(astruct_18 *param_1,byte param_2)

{
  pass1_1030_1d28(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1030_2068(ushort *param_1)

{
  int iVar1;
  int iVar2;
  undefined2 uVar3;
  int iStack4;
  
  struct_1030_17ce(param_1,0x0,0x0);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x293c;
  *(undefined2 *)(iVar1 + 0x2) = 0x1030;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x10)),(WNDCLASS16 *)0x0,0x106);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x116)),(WNDCLASS16 *)0x0,0x86);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x19c)),(WNDCLASS16 *)0x0,0xa);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x2ac)),(WNDCLASS16 *)0x0,0x106);
  iStack4 = 0x0;
  do {
    iVar2 = iStack4 * 0x2 + iVar1;
    *(undefined2 *)(iVar2 + 0x10) = 0xffff;
    *(undefined2 *)(iVar2 + 0x1a6) = 0x19;
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x83);
  return;
}



void __stdcall16far struct_1030_2112(ushort *param_1,ulong param_2,uint param_3,uchar *param_4)

{
  astruct_366 *iVar1;
  astruct_367 *iVar2;
  undefined2 uVar1;
  int iStack4;
  
  pass1_1030_183c(param_1,0x1,0x1,0x8000000,param_2,param_3,param_4);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_366 *)param_1;
  *param_1 = 0x293c;
  iVar1->field_0x2 = 0x1030;
  iStack4 = 0x0;
  do {
    iVar2 = (astruct_367 *)(&iVar1->field_0x0 + iStack4 * 0x2);
    iVar2->field_0x10 = 0xffff;
    iVar2->field_0x1a6 = 0x19;
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x83);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x116),(WNDCLASS16 *)0x0,0x86)
  ;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x19c),(WNDCLASS16 *)0x0,0xa);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1 + 0x1)),(WNDCLASS16 *)0x0,0x106);
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x44 = 0x0;
  iVar1->field_0x50 = 0x0;
  iVar1->field_0x6a = 0x0;
  iVar1->field_0x84 = 0x0;
  iVar1->field_0xc8 = 0x0;
  iVar1->field_0xe4 = 0x0;
  iVar1->field_0xf0 = 0x0;
  iVar1->field_0xf4 = 0x0;
  iVar1->field_0xf6 = 0x0;
  iVar1->field_0x102 = 0x0;
  iVar1->field_0xfe = 0x0;
  iVar1->field_0x1a6 = 0x0;
  iVar1->field_0x1aa = 0x0;
  iVar1->field_0x1ac = 0x0;
  iVar1->field_0x1b6 = 0x0;
  iVar1->field_0x1da = 0x0;
  iVar1->field_0x1e6 = 0x0;
  iVar1->field_0x200 = 0x0;
  iVar1->field_0x21a = 0x0;
  iVar1->field_0x25e = 0x0;
  iVar1->field_0x27a = 0x0;
  iVar1->field_0x286 = 0x0;
  iVar1->field_0x28a = 0x0;
  iVar1->field_0x28c = 0x0;
  iVar1->field_0x298 = 0x0;
  iVar1->field_0x294 = 0x0;
  return;
}



int __stdcall16far pass1_1030_2242(ulong param_1,int param_2)

{
  int iVar1;
  astruct_168 *iVar2;
  astruct_168 *paVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_168 *)param_1;
  paVar2 = (astruct_168 *)&iVar2->field_0x10;
  if (-0x1 < *(int *)(&paVar2->field_0x0 + param_2 * 0x2)) {
    iVar1 = *(int *)(&iVar2->field_0x10 + param_2 * 0x2);
    paVar2 = iVar2 + 0x1;
    if (*(int *)(&paVar2->field_0x0 + param_2 * 0x2) <= iVar1) {
      return iVar1;
    }
  }
  return *(int *)(&paVar2->field_0x0 + param_2 * 0x2);
}



void __stdcall16far pass1_1030_227a(ulong param_1,ulong param_2,uint16_t param_3,ushort param_4)

{
  ushort uVar1;
  int iVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uVar5;
  
  uVar1 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar1 != 0x0) {
    iVar2 = (int)param_1;
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar4 = (ushort)param_2;
    uVar5 = (ushort)(param_2 >> 0x10);
    BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x10,uVar1,(char *)0x106,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x116,uVar1,(char *)0x86,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x19c,uVar1,(char *)0xa,0x1008);
        if (BVar3 != 0x0) {
          BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x1a6,uVar1,(char *)0x106,0x1008);
          if (BVar3 != 0x0) {
            BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x2ac,uVar1,(char *)0x106,0x1008);
            if (BVar3 != 0x0) {
              return;
            }
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far pass1_1030_232e(ulong param_1,ulong param_2,int param_3,undefined2 param_4,undefined2 param_5)

{
  ushort uVar1;
  int iVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uVar5;
  
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    iVar2 = (int)param_1;
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar4 = (ushort)param_2;
    uVar5 = (ushort)(param_2 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x10,0x0,uVar1,0x106,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x116,0x0,uVar1,0x86,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x19c,0x0,uVar1,0xa,0x1008);
        if (BVar3 != 0x0) {
          BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x1a6,0x0,uVar1,0x106,0x1008);
          if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x2ac,0x0,uVar1,0x106,0x1008);
            if (BVar3 != 0x0) {
              return;
            }
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_23e2(ulong param_1,int param_2,uint param_3,int param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  code **ppcVar1;
  undefined4 uVar2;
  bool bVar3;
  bool bVar4;
  undefined3 extraout_var;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uint uVar5;
  ushort uVar6;
  ushort uVar7;
  int iVar8;
  undefined2 uVar9;
  undefined4 *puVar10;
  ushort *puVar11;
  uint uVar12;
  int iStack8;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  uVar6 = (ushort)param_1;
  if (*(int *)(uVar6 + 0x10 + param_3 * 0x2) < 0x0) {
    uVar12 = param_3;
    if (param_2 == 0x0) {
      puVar10 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x31,param_7,param_5,param_6);
      ppcVar1 = (code **)((int)*puVar10 + 0x14);
      (**ppcVar1)(0x1010,(int)puVar10,(int)((ulong)puVar10 >> 0x10),param_3,(int)param_3 >> 0xf);
      param_5 = extraout_DX_00;
    }
    else {
      puVar10 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x41,param_7,param_5,param_6);
      ppcVar1 = (code **)((int)*puVar10 + 0x14);
      (**ppcVar1)(0x1010,(int)puVar10,(int)((ulong)puVar10 >> 0x10),param_3,(int)param_3 >> 0xf);
      param_5 = extraout_DX;
    }
    uVar2 = *(undefined4 *)(uVar12 + 0x16);
    param_4 = *(int *)((int)uVar2 + 0x4);
    *(int *)(uVar6 + param_3 * 0x2 + 0x10) = param_4;
  }
  if (*(int *)(uVar6 + 0x10 + param_3 * 0x2) != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
    pass1_1030_2852();
    bVar3 = false;
    iStack8 = param_4;
    if (*(int *)(uVar6 + 0x152) != 0x0) {
      bVar4 = pass1_1030_266c(uVar6,CONCAT22(param_3,uVar9));
      if ((int)CONCAT31(extraout_var,bVar4) != 0x0) {
        iStack8 = param_4 + 0x1;
        bVar3 = true;
      }
    }
    iVar8 = param_3 * 0x2;
    iStack8 = *(int *)(uVar6 + iVar8 + 0x10) - iStack8;
    *(int *)(uVar6 + iVar8 + 0x10) = iStack8;
    if (iStack8 < 0x0) {
      *(undefined2 *)(uVar6 + iVar8 + 0x10) = 0x0;
    }
    uVar7 = param_3 * 0x2;
    if (*(int *)(uVar6 + 0x2ac + uVar7) == 0x0) {
      iVar8 = uVar7 + uVar6;
      *(undefined2 *)(iVar8 + 0x2ac) = 0x1;
      param_5 = (uchar *)(*(int *)(uVar6 + uVar7 + 0x1a6) + -0x1);
      *(uchar **)(iVar8 + 0x1a6) = param_5;
      param_6 = uVar7;
      if (*(int *)(uVar6 + uVar7 + 0x1a6) < 0x0) {
        *(undefined2 *)(iVar8 + 0x1a6) = 0x0;
      }
    }
    if ((*(int *)(uVar6 + 0x10 + param_3 * 0x2) != 0x0) ||
       (uVar7 = uVar6 + 0x1a6, *(int *)(uVar7 + param_3 * 0x2) != 0x0)) {
      if (*(int *)(uVar6 + 0x10 + param_3 * 0x2) == 0x0) {
        *(undefined2 *)(uVar6 + param_3 * 0x2 + 0x10) = 0x1;
      }
      return;
    }
    uVar12 = param_3;
    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,param_6);
    uVar5 = (uint)((ulong)puVar11 >> 0x10);
    pass1_1010_6cf8(0x1010,(ulong)puVar11,uVar12,param_7,uVar5,uVar7,param_6);
    pass1_1030_26ac(param_1,param_3,uVar5,param_7);
    if (bVar3) {
      iVar8 = pass1_1030_28dc(param_1,param_3);
      *(undefined2 *)(uVar6 + iVar8 * 0x2 + 0x19c) = 0x0;
    }
  }
  return;
}



BOOL16 __stdcall16far pass1_1030_25b2(ulong param_1,int param_2)

{
  if (*(int *)((int)param_1 + 0x10 + param_2 * 0x2) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



void __stdcall16far pass1_1030_25d8(ulong param_1,ushort param_2,int param_3)

{
  *(ushort *)((int)param_1 + param_3 * 0x2 + 0x10) = param_2;
  return;
}



void __stdcall16far pass1_1030_25f0(ulong param_1,int param_2,int param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    param_2 = *(int *)((int)param_1 + 0x116 + param_3 * 0x2) + 0x1;
  }
  *(int *)((int)param_1 + param_3 * 0x2 + 0x116) = param_2;
  return;
}



bool __stdcall16far pass1_1030_2622(ulong param_1,int param_2)

{
  int iVar1;
  
  if ((param_2 != 0x70) && (param_2 != 0x1)) {
    iVar1 = pass1_1030_28dc(param_1,0x0);
    if (-0x1 < iVar1) {
      *(int *)((int)param_1 + iVar1 * 0x2 + 0x19c) = param_2;
    }
    return -0x1 < iVar1;
  }
  return false;
}



bool __stdcall16far pass1_1030_266c(ushort param_1,ulong param_2)

{
  int iVar1;
  
  iVar1 = pass1_1030_28dc(CONCAT22((int)param_2,param_1),(int)(param_2 >> 0x10));
  return iVar1 != -0x1;
}



void __stdcall16far pass1_1030_2690(ulong param_1)

{
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x2ac)),(WNDCLASS16 *)0x0,0x106);
  return;
}



void __stdcall16far pass1_1030_26ac(ulong param_1,uint param_2,uint param_3,ushort param_4)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  char cVar5;
  undefined *puVar6;
  ushort uVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  int iVar11;
  undefined2 uVar12;
  int iStack38;
  undefined local_14 [0x12];
  
  iVar11 = (int)param_1;
  uVar12 = (undefined2)(param_1 >> 0x10);
  if (param_2 != 0x13) {
    if (0x13 < (int)param_2) {
      if (param_2 != 0x5f) {
        if ((int)(param_2 - 0x5f) < 0x6) {
          return;
        }
        if (param_2 != 0x66 && 0x0 < (int)(param_2 - 0x65)) {
          if ((int)(param_2 - 0x66) < 0x5) {
            return;
          }
          if (0x4 < (int)(param_2 - 0x6b)) {
            return;
          }
        }
      }
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
      while( true ) {
        uVar10 = param_3;
        puVar6 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4,puVar6));
        param_3 = uVar10 | (uint)puVar6;
        if (param_3 == 0x0) break;
        if (*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200)) {
          uVar7 = *(int *)(puVar6 + 0x18) + 0x19;
          if (0x3e8 < (int)uVar7) {
            uVar7 = 0x3e8;
          }
          pass1_1038_4d0e(CONCAT22(uVar10,puVar6),uVar7);
        }
      }
      return;
    }
    if (param_2 == 0x12) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
      while( true ) {
        uVar10 = param_3;
        puVar6 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4,puVar6));
        param_3 = uVar10 | (uint)puVar6;
        if (param_3 == 0x0) break;
        if (*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200)) {
          uVar2 = *(undefined4 *)(puVar6 + 0x1f6);
          iVar9 = (int)uVar2;
          uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
          piVar1 = (int *)(iVar9 + 0x182);
          *piVar1 = *piVar1 + -0x19;
          iVar8 = *(int *)(iVar9 + 0x182);
          if (iVar8 < 0x1) {
            iVar8 = 0x1;
          }
          *(int *)(iVar9 + 0x182) = iVar8;
        }
      }
      return;
    }
    if (0x12 < param_2) {
      return;
    }
    cVar5 = (char)param_2;
    if (cVar5 != '\n') {
      if ((char)(cVar5 + -0xa) < '\x06') {
        return;
      }
      if ('\x01' < (char)(cVar5 + -0x10)) {
        return;
      }
    }
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
  while( true ) {
    uVar10 = param_3;
    puVar6 = local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar6));
    param_3 = uVar10 | (uint)puVar6;
    if (param_3 == 0x0) break;
    if (*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200)) {
      uVar2 = *(undefined4 *)(puVar6 + 0x1f6);
      iVar8 = (int)uVar2 + 0x180;
      uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
      iStack38 = 0x1;
      do {
        iVar3 = iStack38 * 0x2;
        piVar1 = (int *)(iVar3 + iVar8);
        *piVar1 = *piVar1 + -0x1;
        iVar9 = *(int *)(iVar3 + iVar8);
        if (iVar9 < 0x1) {
          iVar9 = 0x1;
        }
        *(int *)(iVar3 + iVar8) = iVar9;
        iStack38 = iStack38 + 0x1;
      } while (iStack38 < 0x6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_2852(void)

{
  return;
}



int __stdcall16far pass1_1030_28dc(ulong param_1,int param_2)

{
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x4 < iStack4) {
      return -0x1;
    }
    if (*(int *)((int)param_1 + 0x19c + iStack4 * 0x2) == param_2) break;
    iStack4 = iStack4 + 0x1;
  }
  return iStack4;
}



astruct_18 * __stdcall16far pass1_1030_2916(astruct_18 *param_1,byte param_2)

{
  pass1_1030_18b2(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1030_2958(ushort *param_1)

{
  astruct_347 *iVar1;
  undefined2 uVar1;
  
  struct_1030_17ce(param_1,0x5,0xf);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_347 *)param_1;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x18 = 0x2710;
  iVar1->field_0x1a = 0x0;
  *param_1 = 0x3130;
  iVar1->field_0x2 = 0x1030;
  return;
}



void __stdcall16far struct_1030_299a(ushort *param_1,ulong param_2,uint param_3,uchar *param_4)

{
  astruct_352 *iVar1;
  undefined2 uVar1;
  
  pass1_1030_183c(param_1,0x5,0xf,0x2000000,param_2,param_3,param_4);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_352 *)param_1;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x18 = 0x2710;
  iVar1->field_0x1a = 0x0;
  *param_1 = 0x3130;
  iVar1->field_0x2 = 0x1030;
  return;
}



void __stdcall16far pass1_1030_29e6(ushort *param_1)

{
  uint uVar1;
  astruct_18 *paVar2;
  astruct_607 *iVar4;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_607 *)param_1;
  *param_1 = 0x3130;
  iVar4->field_0x2 = 0x1030;
  paVar2 = *(astruct_18 **)&iVar4->field_0x10;
  uVar1 = iVar4->field_0x12;
  if ((uVar1 | (uint)paVar2) != 0x0) {
    pass1_1030_8496((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1030_18b2(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_2a2c(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  int *piVar1;
  astruct_678 *iVar2;
  undefined2 uVar2;
  astruct_67 *paVar3;
  undefined2 uVar4;
  int iVar5;
  ushort uVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  int iVar9;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_678 *)param_1;
  if (0x0 < iVar2->field_0x18) {
    piVar1 = &iVar2->field_0x18;
    *piVar1 = *piVar1 + -0x1;
  }
  if (iVar2->field_0x16 == 0x0) {
    iVar2->field_0x16 = 0x1;
  }
  if (iVar2->field_0x1a == 0x0) {
    iVar2->field_0x1a = 0x2;
  }
  if (iVar2->field_0x18 < 0x1) {
    iVar2->field_0x16 = 0x2;
    iVar2->field_0x1a = 0x1;
    uVar8 = 0x0;
    iVar9 = 0x21;
    uVar6 = 0x1;
    uVar7 = 0x0;
    uVar4 = 0x0;
    iVar5 = 0x0;
    uVar2 = 0x0;
    paVar3 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,param_2,param_3);
    post_win_msg_1008_a0e4(paVar3,CONCAT22(uVar4,uVar2),iVar5,uVar6,CONCAT22(uVar8,uVar7),iVar9,0x1008,param_4);
  }
  return;
}



ushort __stdcall16far pass1_1030_2a98(ulong param_1)

{
  int *piVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  piVar1 = (int *)((int)param_1 + 0x14);
  *piVar1 = *piVar1 + 0x1;
  return *(ushort *)((int)param_1 + 0x14);
}



ushort __stdcall16far pass1_1030_2aaa(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x10) == 0x0) {
    return 0x0;
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x10);
  return *(ushort *)((int)uVar1 + 0xc);
}



void __stdcall16far pass1_1030_2aca(ulong param_1,ulong param_2,uint16_t param_3,ushort param_4)

{
  undefined4 uVar1;
  undefined2 *puVar2;
  ushort uVar3;
  BOOL16 BVar4;
  int iVar5;
  astruct_730 *iVar6;
  undefined2 uVar6;
  undefined2 uVar7;
  ushort uVar8;
  undefined4 local_18 [0x3];
  undefined2 local_c [0x3];
  undefined2 local_6 [0x2];
  
  uVar3 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar3 == 0x0) {
    return;
  }
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_730 *)param_1;
  local_c[0] = *iVar6->field_0x10;
  uVar3 = (ushort)param_2;
  uVar8 = (ushort)(param_2 >> 0x10);
  BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_c,param_4,(char *)0x2,0x1008);
  if (((BVar4 != 0x0) &&
      (puVar2 = iVar6->field_0x10, BVar4 = pass1_1008_7c2a(param_2,*(char **)((int)puVar2 + 0x2),0x1008), BVar4 != 0x0))
     && (puVar2 = iVar6->field_0x10,
        iVar5 = write_to_file_1008_7b4c(param_2,(ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0x6),0x1008,param_4),
        iVar5 != 0x0)) {
    puVar2 = iVar6->field_0x10;
    local_6[0] = *(undefined2 *)((int)puVar2 + 0xc);
    BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_6,param_4,(char *)0x2,0x1008);
    if (BVar4 != 0x0) {
      puVar2 = iVar6->field_0x10;
      local_18[0] = *(undefined4 *)((int)puVar2 + 0xe);
      BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_18,param_4,(char *)0x4,0x1008);
      if ((BVar4 != 0x0) &&
         (puVar2 = iVar6->field_0x10,
         BVar4 = write_to_file_1008_7e1c
                           (uVar3,uVar8,(int)puVar2 + 0x12,(ushort)((ulong)puVar2 >> 0x10),(char *)0x10,0x1008),
         BVar4 != 0x0)) {
        puVar2 = iVar6->field_0x10;
        local_c[0] = *(undefined2 *)((int)puVar2 + 0x22);
        BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_c,param_4,(char *)0x2,0x1008);
        if ((BVar4 != 0x0) &&
           ((puVar2 = iVar6->field_0x10, *(int *)((int)puVar2 + 0x22) == 0x0 ||
            (puVar2 = iVar6->field_0x10, uVar7 = (undefined2)((ulong)puVar2 >> 0x10), iVar5 = (int)puVar2,
            uVar1 = *(undefined4 *)(iVar5 + 0x24),
            BVar4 = write_to_file_1008_7e1c
                              (uVar3,uVar8,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),
                               (char *)(ulong)(uint)(*(int *)(iVar5 + 0x22) * 0x2),0x1008), BVar4 != 0x0)))) {
          local_c[0] = iVar6->field_0x14;
          BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_c,param_4,(char *)0x2,0x1008);
          if (BVar4 != 0x0) {
            local_c[0] = iVar6->field_0x16;
            BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_c,param_4,(char *)0x2,0x1008);
            if (BVar4 != 0x0) {
              local_c[0] = iVar6->field_0x18;
              BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_c,param_4,(char *)0x2,0x1008);
              if (BVar4 != 0x0) {
                local_c[0] = iVar6->field_0x1a;
                BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,(ushort)local_c,param_4,(char *)0x2,0x1008);
                if (BVar4 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_2c8a(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  undefined4 uVar1;
  ushort uVar2;
  BOOL16 BVar3;
  undefined *puVar4;
  ushort uVar5;
  uchar *puVar6;
  astruct_374 *iVar7;
  astruct_371 *iVar8;
  astruct_372 *iVar9;
  int unaff_DI;
  undefined2 uVar7;
  ushort *puVar8;
  ushort uVar9;
  ushort uVar10;
  ushort *puStack1038;
  undefined2 local_406;
  undefined2 local_404;
  undefined local_402 [0x400];
  astruct_373 *iVar14;
  
  iVar14 = (astruct_373 *)param_1;
  uVar10 = (ushort)(param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 == 0x0) {
    return;
  }
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4,0x1000);
    PTR_LOOP_1050_5f2e = param_4;
  }
  else {
  }
  uVar2 = fn_ptr_op_1000_1708(0x28,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  puStack1038 = (ushort *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
  puVar6 = (uchar *)((uint)PTR_LOOP_1050_5f2e | uVar2);
  if (puVar6 == (uchar *)0x0) {
    iVar14->field_0x10 = (ushort *)0x0;
  }
  else {
    puVar8 = pass1_1008_3e38((ushort *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2 + 0x6));
    puVar6 = (uchar *)((ulong)puVar8 >> 0x10);
    iVar14->field_0x10 = puStack1038;
  }
  puVar8 = iVar14->field_0x10;
  uVar2 = (ushort)param_2;
  uVar9 = (ushort)(param_2 >> 0x10);
  BVar3 = read_file_1008_7dee(uVar2,uVar9,(ushort)puVar8,0x0,(ushort)((ulong)puVar8 >> 0x10),0x2,0x1008);
  if (BVar3 != 0x0) {
    puVar4 = local_402;
    read_file_1008_7c6e(uVar2,uVar9,(char *)CONCAT22(param_5,puVar4),0x1008);
    if (puVar4 != (undefined *)0x0) {
      uVar5 = str_op_1008_60e8((char *)CONCAT22(param_5,local_402),(ushort)puVar6);
      puVar8 = iVar14->field_0x10;
      uVar7 = (undefined2)((ulong)puVar8 >> 0x10);
      iVar7 = (astruct_374 *)puVar8;
      iVar7->field_0x2 = uVar5;
      iVar7->field_0x4 = puVar6;
      puVar8 = iVar14->field_0x10;
      BVar3 = read_file_1008_7bc8(param_2,(ushort *)((ulong)puVar8 & 0xffff0000 | (ulong)((int)puVar8 + 0x6)),0x1008,
                                  param_5);
      if ((((BVar3 != 0x0) &&
           (puVar8 = iVar14->field_0x10,
           BVar3 = read_file_1008_7dee(uVar2,uVar9,(int)puVar8 + 0xc,0x0,(ushort)((ulong)puVar8 >> 0x10),0x2,0x1008),
           BVar3 != 0x0)) &&
          (puVar8 = iVar14->field_0x10,
          BVar3 = read_file_1008_7dee(uVar2,uVar9,(int)puVar8 + 0xe,0x0,(ushort)((ulong)puVar8 >> 0x10),0x4,0x1008),
          BVar3 != 0x0)) &&
         ((puVar8 = iVar14->field_0x10,
          BVar3 = read_file_1008_7dee(uVar2,uVar9,(int)puVar8 + 0x12,0x0,(ushort)((ulong)puVar8 >> 0x10),0x10,0x1008),
          BVar3 != 0x0 &&
          (puVar8 = iVar14->field_0x10,
          BVar3 = read_file_1008_7dee(uVar2,uVar9,(int)puVar8 + 0x22,0x0,(ushort)((ulong)puVar8 >> 0x10),0x2,0x1008),
          BVar3 != 0x0)))) {
        puVar8 = iVar14->field_0x10;
        if (*(int *)((int)puVar8 + 0x22) != 0x0) {
          puVar8 = iVar14->field_0x10;
          unaff_DI = (int)((ulong)puVar8 >> 0x10);
          iVar8 = (astruct_371 *)puVar8;
          uVar5 = iVar8->field_0x22 * 0x2;
          mem_op_1000_179c(uVar5,puVar6,0x1000);
          iVar8->field_0x24 = uVar5;
          iVar8->field_0x26 = puVar6;
          puVar8 = iVar14->field_0x10;
          uVar7 = (undefined2)((ulong)puVar8 >> 0x10);
          iVar9 = (astruct_372 *)puVar8;
          uVar1 = iVar9->field_0x24;
          BVar3 = read_file_1008_7dee(uVar2,uVar9,(ushort)uVar1,0x0,(ushort)((ulong)uVar1 >> 0x10),
                                      iVar9->field_0x22 * 0x2,0x1008);
          if (BVar3 == 0x0) {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
          }
        }
        BVar3 = read_file_1008_7dee(uVar2,uVar9,&iVar14->field_0x14,0x0,uVar10,0x2,0x1008);
        if (((BVar3 != 0x0) &&
            (BVar3 = read_file_1008_7dee(uVar2,uVar9,(ushort)&local_404,0x0,param_5,0x2,0x1008), BVar3 != 0x0)) &&
           ((BVar3 = read_file_1008_7dee(uVar2,uVar9,&iVar14->field_0x18,0x0,uVar10,0x2,0x1008), BVar3 != 0x0 &&
            (BVar3 = read_file_1008_7dee(uVar2,uVar9,(ushort)&local_406,0x0,param_5,0x2,0x1008), BVar3 != 0x0)))) {
          iVar14->field_0x16 = local_404;
          iVar14->field_0x1a = local_406;
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_5,puVar6,unaff_DI);
          pass1_1018_04a4((ulong)puVar8,iVar14->field_0x4);
          pass1_1010_82f8(_PTR_LOOP_1050_14cc,*iVar14->field_0x10);
          return;
        }
      }
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  return;
}



int __stdcall16far pass1_1030_2f1a(ulong param_1,ushort *param_2,ushort *param_3)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  
  uVar2 = *(undefined4 *)((int)param_1 + 0x10);
  iVar3 = (int)uVar2;
  iVar1 = *(int *)(iVar3 + 0xc);
  if (iVar1 - 0x1U < 0x9) {
    switch(iVar1) {
    default:
      *param_3 = 0x19;
      *param_2 = 0x2d;
      return iVar3;
    case 0x3:
    case 0x4:
    case 0x5:
      *param_3 = 0xa;
      *param_2 = 0xf;
      return iVar3;
    case 0x6:
      *param_3 = 0xa;
      *param_2 = 0x19;
      return iVar3;
    case 0x7:
      *param_3 = 0x19;
      *param_2 = 0x37;
      return iVar3;
    }
  }
  *param_3 = 0x0;
  *param_2 = 0x0;
  return 0x0;
}



ushort __stdcall16far pass1_1030_2fac(ulong param_1)

{
  long lVar1;
  astruct_598 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_598 *)param_1;
  if (iVar2->field_0x10 == 0x0) {
    return 0x0;
  }
  lVar1 = iVar2->field_0x10;
  if (*(uint *)((int)lVar1 + 0xc) < 0x2) {
    return 0x4;
  }
  lVar1 = iVar2->field_0x10;
  if (*(uint *)((int)lVar1 + 0xc) < 0x5) {
    return 0x3;
  }
  lVar1 = iVar2->field_0x10;
  if (*(uint *)((int)lVar1 + 0xc) < 0x8) {
    return 0x2;
  }
  return 0x1;
}
