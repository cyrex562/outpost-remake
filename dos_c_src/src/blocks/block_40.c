
astruct_11 * __stdcall16far pass1_1018_2aa8(astruct_11 *param_1,byte param_2,ushort param_3)

{
  param_1 = (astruct_11 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x1c));
  pass1_1018_2440(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1018_2afa(ulong *param_1)

{
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1018_2b10(astruct_55 *param_1,ushort param_2,ushort param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined2 *puVar3;
  undefined2 uVar4;
  int unaff_DI;
  ushort *puVar5;
  astruct_43 *paVar6;
  ulong uVar7;
  undefined2 uVar8;
  astruct_626 *uVar9;
  
  uVar9 = (astruct_626 *)param_1;
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  puVar5 = get_sys_metrics_1018_4b1e(param_1,0x1,param_2);
  uVar9->field_0x20 = 0x389a;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x20 = 0x3aa8;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x24 = (astruct_76 *)0x0;
  uVar9->field_0x174 = 0x0;
  uVar9->field_0x176 = 0x0;
  uVar9->field_0x178 = 0x0;
  uVar9->field_0x17a = 0x0;
  uVar9->field_0x17e = 0x0;
  uVar9->field_0x182 = (undefined4 *)0x0;
  uVar9->field_0x186 = 0x0;
  param_1->field_0x0 = 0x32d8;
  uVar9->field_0x2 = 0x1018;
  uVar9->field_0x20 = 0x3314;
  uVar9->field_0x22 = 0x1018;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,(uchar *)((ulong)puVar5 >> 0x10),unaff_DI);
  *(int *)&uVar9->field_0x182 = (int)puVar5;
  *(undefined2 *)((int)&uVar9->field_0x182 + 0x2) = (int)((ulong)puVar5 >> 0x10);
  if (param_1 == (astruct_55 *)0x0) {
    puVar3 = (undefined2 *)0x0;
    uVar4 = 0x0;
  }
  else {
    puVar3 = &uVar9->field_0x20;
    uVar4 = uVar8;
  }
  puVar1 = uVar9->field_0x182;
  ppcVar2 = (code **)((int)*uVar9->field_0x182 + 0x4);
  (**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),0x0,puVar3,uVar4);
  puVar1 = uVar9->field_0x182;
  uVar9->field_0x17a = *(undefined4 *)((int)puVar1 + 0x24);
  paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x6e,param_3);
  *(int *)&uVar9->field_0x24 = (int)paVar6;
  *(undefined2 *)((int)&uVar9->field_0x24 + 0x2) = (int)((ulong)paVar6 >> 0x10);
  uVar9->field_0x28 = 0x0;
  uVar7 = pass1_1008_4772(uVar9->field_0x24);
  uVar4 = (undefined2)(uVar7 >> 0x10);
  pass1_1018_4b78((ulong *)param_1,param_3);
  uVar9->field_0x16c = 0x1;
  uVar9->field_0x16e = 0x1;
  uVar9->field_0x170 = *(int *)((int)uVar7 + 0x4) + uVar9->field_0x16c;
  uVar9->field_0x172 = *(int *)((int)uVar7 + 0x8) + -0x19;
  return;
}



void __stdcall16far pass1_1018_2c60(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 *puVar4;
  undefined2 uVar5;
  astruct_503 *uVar6;
  undefined2 uVar7;
  undefined2 *puStack6;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  uVar6 = (astruct_503 *)param_1;
  *param_1 = 0x32d8;
  uVar6->field_0x2 = 0x1018;
  uVar6->field_0x20 = 0x3314;
  uVar6->field_0x22 = 0x1018;
  if (uVar6->field_0x182 != 0x0) {
    if (param_1 == (ushort *)0x0) {
      puVar4 = (undefined2 *)0x0;
      uVar5 = 0x0;
    }
    else {
      puVar4 = &uVar6->field_0x20;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6(uVar6->field_0x182,CONCAT22(uVar5,puVar4),param_2);
  }
  fn_ptr_1000_17ce((astruct_18 *)uVar6->field_0x186,0x1000);
  puVar1 = uVar6->field_0x24;
  uVar2 = uVar6->field_0x26;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,0x1);
  }
  if (param_1 == (ushort *)0x0) {
    puVar4 = (undefined2 *)0x0;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar6->field_0x20;
  }
  puStack6 = (undefined2 *)CONCAT22(uVar7,puVar4);
  *puStack6 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1018_2d22(ulong param_1,int *param_2,ushort *param_3,int param_4)

{
  ulong uVar1;
  
  *param_3 = 0x0;
  *param_2 = 0x0;
  uVar1 = pass1_1008_4772(*(astruct_76 **)((int)param_1 + 0x24));
  *param_2 = *(int *)((int)uVar1 + 0x8) + -0x14;
  if (param_4 == 0xbb8) {
    *param_3 = 0x5;
  }
  if (param_4 == 0xbba) {
    *param_3 = 0x23;
  }
  if (param_4 == 0xbb9) {
    *param_3 = 0x75;
  }
  return;
}



void __stdcall16far pass1_1018_2d84(ushort param_1,ulong param_2)

{
  pass1_1018_2e28(param_2);
  pass1_1020_bd80(param_1);
  return;
}



void __stdcall16far pass1_1018_2d9a(ulong param_1)

{
  int *piVar1;
  undefined4 uVar2;
  uint uVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar3 = *(uint *)(iVar4 + 0x180) | *(uint *)(iVar4 + 0x17e);
  if (uVar3 != 0x0) {
    piVar1 = (int *)(iVar4 + 0x174);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      uVar2 = *(undefined4 *)(iVar4 + 0x17e);
      uVar3 = *(int *)((int)uVar2 + 0xa) - 0x1;
      *(uint *)(iVar4 + 0x174) = uVar3;
    }
    pass1_1018_2e28(param_1);
    *(uint *)(iVar4 + 0x176) = uVar3;
  }
  return;
}



void __stdcall16far pass1_1018_2dde(ulong param_1)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((*(uint *)(iVar4 + 0x180) | *(uint *)(iVar4 + 0x17e)) != 0x0) {
    piVar1 = (int *)(iVar4 + 0x174);
    *piVar1 = *piVar1 + 0x1;
    iVar3 = *(int *)(iVar4 + 0x174);
    uVar2 = *(undefined4 *)(iVar4 + 0x17e);
    piVar1 = (int *)((int)uVar2 + 0xa);
    if (*piVar1 == iVar3 || *piVar1 < iVar3) {
      *(undefined2 *)(iVar4 + 0x174) = 0x0;
    }
    pass1_1018_2e28(param_1);
    *(int *)(iVar4 + 0x176) = iVar3;
  }
  return;
}



void __stdcall16far pass1_1018_2e28(ulong param_1)

{
  long lVar1;
  uint extraout_DX;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  lVar1 = (long)*(int *)((int)param_1 + 0x174);
  empty_1008_8fc4(*(undefined4 *)((int)param_1 + 0x17e),lVar1);
  if ((extraout_DX | (uint)lVar1) != 0x0) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_2e5e(ushort param_1,ushort param_2,ushort param_3,ulong param_4)

{
  ushort uVar1;
  long lVar1;
  astruct_126 *iVar4;
  ushort uVar2;
  
  uVar2 = (ushort)(param_4 >> 0x10);
  iVar4 = (astruct_126 *)param_4;
  if (iVar4->field_0x17e == 0x0) {
    pass1_1030_82f0(param_1,_PTR_LOOP_1050_5748,iVar4->field_0x17a);
    *(ushort *)&iVar4->field_0x17e = param_2;
    *(ushort *)((int)&iVar4->field_0x17e + 0x2) = param_3;
  }
  if ((iVar4->field_0x17e != 0x0) && (lVar1 = iVar4->field_0x17e, *(int *)((int)lVar1 + 0xa) != 0x0)) {
    lVar1 = (long)iVar4->field_0x174;
    empty_1008_8fc4(iVar4->field_0x17e,lVar1);
    uVar1 = (ushort)lVar1;
    pass1_1018_2e28(param_4);
    iVar4->field_0x176 = uVar1;
    return;
  }
  return;
}



void __stdcall16far pass1_1018_2ee4(ulong param_1,uint param_2,ushort param_3)

{
  undefined4 uVar1;
  char cVar2;
  ushort uVar3;
  
  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar2 = (char)param_2;
      if (cVar2 == '\x01') {
        *(undefined4 *)((int)param_1 + 0x162) = 0x0;
        return;
      }
      if (('\x02' < (char)(cVar2 + -0x1)) && ((char)(cVar2 + -0x4) < '\x03')) goto LAB_1018_2f06;
    }
    return;
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x162);
  *(undefined4 *)((int)param_1 + 0x15a) = *(undefined4 *)((int)uVar1 + 0x24);
LAB_1018_2f06:
  uVar3 = (int)param_1 - 0x20;
  pass1_1018_31fa(param_1 & 0xffff0000 | (ulong)uVar3,uVar3,param_1._2_2_,param_3);
  pass1_1010_1f62(param_3,param_1 & 0xffff0000 | (ulong)uVar3,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far get_sys_metrics_1018_2f56(ulong param_1)

{
  undefined2 uVar1;
  INT16 IVar2;
  INT16 IVar3;
  uchar *in_DX;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort unaff_SS;
  ushort *puVar6;
  ulong uVar7;
  ushort *puVar8;
  ushort *puVar9;
  int local_6;
  int local_4;
  
  puVar9 = (ushort *)CONCAT22(unaff_SS,&local_4);
  puVar8 = (ushort *)CONCAT22(unaff_SS,&local_6);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)),puVar8,puVar9);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar7 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x24));
  uVar1 = (undefined2)(uVar7 >> 0x10);
  *(int *)(iVar4 + 0x18) = local_4 + 0xb5;
  *(int *)(iVar4 + 0x1a) = local_6 + 0x9;
  IVar2 = GetSystemMetrics16(0x1008);
  *(int *)(iVar4 + 0x1c) = IVar2 * 0x2 + *(int *)((int)uVar7 + 0x4);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  *(int *)(iVar4 + 0x1e) = IVar3 + IVar2 + *(int *)((int)uVar7 + 0x8);
  return;
}



void __stdcall16far pass1_1018_2fe8(ulong param_1,uint16_t param_2,uint16_t param_3)

{
  int *piVar1;
  undefined4 uVar2;
  uint uVar3;
  ushort uVar4;
  uint uVar5;
  undefined2 uVar6;
  int iVar7;
  undefined2 extraout_DX;
  undefined2 uVar8;
  int iVar9;
  undefined2 uVar10;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar9 = (int)param_1;
  uVar6 = *(undefined2 *)(iVar9 + 0x174);
  uVar2 = *(undefined4 *)(iVar9 + 0x17e);
  iVar7 = *(int *)((int)uVar2 + 0xa);
  if (iVar7 != 0x0) {
    if (*(long *)(iVar9 + 0x186) != 0x0) {
      uVar3 = str_op_1000_3da4(*(char **)(iVar9 + 0x186));
      *(undefined2 *)(iVar9 + 0x174) = 0x0;
      while( true ) {
        if (iVar7 <= *(int *)(iVar9 + 0x174)) break;
        uVar4 = *(ushort *)(iVar9 + 0x174);
        uVar2 = *(undefined4 *)(iVar9 + 0x17e);
        empty_1008_8fc4((int)uVar2,(int)((ulong)uVar2 >> 0x10),uVar4,(int)uVar4 >> 0xf);
        uVar8 = extraout_DX;
        pass1_1018_2e28(param_1);
        uVar4 = pass1_1020_bd80(uVar4);
        uVar5 = pass1_1000_3de8((char *)CONCAT22(uVar8,uVar4),*(char **)(iVar9 + 0x186),uVar3,param_2,param_3);
        if (uVar5 == 0x0) break;
        piVar1 = (int *)(iVar9 + 0x174);
        *piVar1 = *piVar1 + 0x1;
      }
      if (*(int *)(iVar9 + 0x174) < iVar7) {
        pass1_1018_2e28(param_1);
        *(int *)(iVar9 + 0x176) = iVar7;
        return;
      }
      *(undefined2 *)(iVar9 + 0x174) = uVar6;
      pass1_1018_2e28(param_1);
      *(undefined2 *)(iVar9 + 0x176) = uVar6;
    }
  }
  return;
}



void __stdcall16far pass1_1018_30ca(ulong param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  astruct_504 *iVar3;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_504 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x186,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x186 = uVar1;
  iVar3->field_0x188 = param_3;
  return;
}



void __stdcall16far pass1_1018_30fc(ulong param_1,ushort **param_2,uchar *param_3)

{
  uint uVar1;
  undefined4 uVar2;
  ushort *puVar3;
  uint uVar4;
  ushort uVar5;
  long lVar6;
  uchar *puVar7;
  undefined2 extraout_DX;
  undefined2 uVar8;
  undefined4 *puStack18;
  int iStack6;
  
  *param_2 = (ushort *)0x0;
  uVar8 = (undefined2)(param_1 >> 0x10);
  uVar2 = *(undefined4 *)((int)param_1 + 0x17e);
  uVar1 = *(uint *)((int)uVar2 + 0xa);
  if (uVar1 != 0x0) {
    uVar4 = uVar1;
    mem_op_1000_179c(0x6,param_3,0x1000);
    puStack18 = (undefined4 *)CONCAT22(param_3,uVar4);
    puVar7 = (uchar *)((uint)param_3 | uVar4);
    if (puVar7 == (uchar *)0x0) {
      *param_2 = (ushort *)0x0;
    }
    else {
      *puStack18 = 0x0;
      *(undefined2 *)(uVar4 + 0x4) = 0x0;
      *param_2 = (ushort *)puStack18;
    }
    uVar5 = uVar1 * 0x2;
    mem_op_1000_179c(uVar5,puVar7,0x1000);
    puVar3 = *param_2;
    *puVar3 = uVar5;
    *(uchar **)((int)puVar3 + 0x2) = puVar7;
    *(uint *)((int)*param_2 + 0x4) = uVar1;
    for (iStack6 = 0x0; iStack6 < (int)uVar1; iStack6 = iStack6 + 0x1) {
      lVar6 = (long)iStack6;
      empty_1008_8fc4(*(undefined4 *)((int)param_1 + 0x17e),lVar6);
      *(undefined2 *)((int)*(undefined4 *)*param_2 + iStack6 * 0x2) = *(undefined2 *)((int)lVar6 + 0x2e);
    }
  }
  return;
}



ushort __stdcall16far pass1_1018_31d0(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if ((*(long *)((int)param_1 + 0x17e) != 0x0) &&
     (uVar1 = *(undefined4 *)((int)param_1 + 0x17e), *(long *)((int)uVar1 + 0xa) != 0x0)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_31fa(ulong param_1,ushort param_2,uint param_3,ushort param_4)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  long lVar5;
  int iVar6;
  undefined2 uVar7;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  pass1_1030_82f0(param_4,_PTR_LOOP_1050_5748,*(ulong *)(iVar6 + 0x17a));
  *(ushort *)(iVar6 + 0x17e) = param_2;
  *(uint *)(iVar6 + 0x180) = param_3;
  if (((param_3 | *(uint *)(iVar6 + 0x17e)) != 0x0) &&
     (uVar2 = *(undefined4 *)(iVar6 + 0x17e), iVar4 = *(int *)((int)uVar2 + 0xa), iVar4 != 0x0)) {
    *(undefined2 *)(iVar6 + 0x174) = 0x0;
    while( true ) {
      if (iVar4 <= *(int *)(iVar6 + 0x174)) break;
      lVar5 = (long)*(int *)(iVar6 + 0x174);
      empty_1008_8fc4(*(undefined4 *)(iVar6 + 0x17e),lVar5);
      iVar3 = (int)lVar5;
      pass1_1018_2e28(param_1);
      if (*(int *)(iVar6 + 0x176) == iVar3) break;
      piVar1 = (int *)(iVar6 + 0x174);
      *piVar1 = *piVar1 + 0x1;
    }
    if (iVar4 <= *(int *)(iVar6 + 0x174)) {
      *(undefined2 *)(iVar6 + 0x174) = 0x0;
    }
    pass1_1018_2e28(param_1);
    *(int *)(iVar6 + 0x176) = iVar4;
  }
  return;
}



ushort * __stdcall16far pass1_1018_32a6(ushort *param_1,byte param_2,ushort param_3)

{
  param_1 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x20));
  pass1_1018_2c60(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_331c(astruct_638 *param_1,ushort param_2,ushort param_3,ushort param_4,uchar *param_5)

{
  uint uVar1;
  int unaff_DI;
  ushort *puVar2;
  
  pass1_1008_ca5a((astruct_639 *)param_1,param_2,param_3);
  *(undefined4 *)&param_1->field_0x122 = 0x0;
  param_1->field_0x126 = 0x0;
  param_1->field_0x12a = 0x0;
  param_1->field_0x12e = 0x0;
  param_1->field_0x130 = 0x0;
  param_1->field_0x132 = 0x0;
  param_1->field_0x136 = 0x0;
  param_1->field_0x13a = 0x0;
  param_1->field_0x13c = 0x0;
  param_1->field_0x13e = 0x0;
  param_1->field_0x142 = 0x0;
  *(int *)CONCAT22(param_2,param_1) = (int)&PTR_LOOP_1050_470c;
  param_1->field_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_4,param_5,unaff_DI);
  uVar1 = (uint)puVar2;
  param_1->field_0x122 = uVar1;
  param_1->field_0x124 = (int)((ulong)puVar2 >> 0x10);
  *(undefined *)&param_1->field_0x22 = 0x0;
  pass1_1008_612e(0x8,0xc,uVar1);
  param_1->field_0x13c = uVar1;
  return;
}



void __stdcall16far pass1_1018_33b4(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_505 *iVar5;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_505 *)param_1;
  *param_1 = (ushort)&PTR_LOOP_1050_470c;
  iVar5->field_0x2 = 0x1018;
  puVar1 = iVar5->field_0x136;
  uVar2 = iVar5->field_0x138;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar5->field_0x136 = 0x0;
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x126,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x12a,0x1000);
  pass1_1008_caa0(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_3424(ulong param_1,int param_2,uint param_3,ushort param_4)

{
  undefined4 uVar1;
  uint uVar2;
  int iVar3;
  undefined2 uVar4;
  ulong uStack10;
  ulong uStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar3 + 0x122);
  pass1_1008_e852((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(ulong *)(iVar3 + 0x126),param_4,param_3);
  uStack6 = CONCAT22(param_3,param_2);
  uVar1 = *(undefined4 *)(iVar3 + 0x122);
  pass1_1008_e852((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(ulong *)(iVar3 + 0x12a),param_4,param_3);
  uStack10 = CONCAT22(param_3,param_2);
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uStack6);
  uVar2 = param_3;
  iVar3 = param_2;
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uStack10);
  if (*(long *)(iVar3 + 0x200) == *(long *)(param_2 + 0x200)) {
    return;
  }
  return;
}



void __stdcall16far pass1_1018_34a6(ulong param_1)

{
  pass1_1018_3d6c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far sprintf_op_1018_34b6(ulong param_1,uchar param_2)

{
  int iVar1;
  undefined3 in_register_00000001;
  undefined2 in_DX;
  int iVar2;
  WORD *valist;
  LPSTR buffer;
  ushort unaff_SS;
  ulong uVar3;
  long lVar4;
  
  valist = (WORD *)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar3 = switch_1018_3b9e(param_1,*(ushort *)(iVar2 + 0x12e),(ushort)CONCAT31(in_register_00000001,param_2),in_DX,
                           unaff_SS);
  iVar1 = *(int *)(iVar2 + 0x12e);
  if (iVar1 == 0x188) {
    lVar4 = pass1_1008_57f0(uVar3,*(int *)(iVar2 + 0x130),unaff_SS);
    buffer = (LPSTR)0x1020;
    string_1020_c0d8(*(ushort *)((int)lVar4 + 0xe));
  }
  else {
    if (iVar1 == 0x18b) {
      buffer = (LPSTR)0x1008;
      pass1_1008_57f0(uVar3,*(int *)(iVar2 + 0x130),unaff_SS);
    }
    else {
      if (iVar1 != 0x18c) {
        load_string_1010_84e0
                  (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,
                   (char *)(iVar2 + 0x22),(short)valist);
        return;
      }
      buffer = (LPSTR)0x1008;
      pass1_1008_57f0(uVar3,*(int *)(iVar2 + 0x130),unaff_SS);
    }
  }
  wsprintf16(buffer,(LPCSTR)(iVar2 + 0x22),valist);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_str_op_1018_35b0(ulong param_1,ushort param_2,undefined2 param_3)

{
  uint *puVar1;
  int *piVar2;
  ushort uVar3;
  uint uVar4;
  code **ppcVar5;
  undefined2 uVar6;
  undefined4 *puVar7;
  undefined2 uVar8;
  uint uVar9;
  uint extraout_DX;
  ushort uVar10;
  WORD *valist;
  bool bVar11;
  ulong uVar12;
  undefined4 uVar13;
  int local_12;
  int local_10;
  long lStack14;
  undefined2 uStack10;
  undefined2 uStack8;
  uint uStack6;
  uint uStack4;
  
  uVar12 = pass1_1030_8326();
  uStack4 = (uint)(uVar12 >> 0x10);
  uStack6 = (uint)uVar12;
  valist = (WORD *)(param_1 >> 0x10);
  uVar10 = (ushort)param_1;
  puVar1 = (uint *)(uVar10 + 0x140);
  bVar11 = *puVar1 < uStack4;
  if ((bVar11) || ((bVar11 || *puVar1 == uStack4 && (*(uint *)(uVar10 + 0x13e) < uStack6)))) {
    uVar3 = *(ushort *)(uVar10 + 0x13c);
    if (*(int *)(uVar10 + 0x13a) < (int)uVar3) {
      uVar13 = switch_1018_3b9e(param_1,*(ushort *)(uVar10 + 0x12e),uVar3,uStack4,param_2);
      uVar8 = (undefined2)((ulong)uVar13 >> 0x10);
      uVar6 = (undefined2)uVar13;
      uStack10 = uVar6;
      uStack8 = uVar8;
      pass1_1018_427c(param_1);
      lStack14 = CONCAT22(uVar8,uVar6);
      pass1_1018_3e8c(uVar10,(ushort)valist,(ushort *)CONCAT22(param_2,&local_12),(ushort *)CONCAT22(param_2,&local_10))
      ;
      if (lStack14 < local_12) {
        local_12 = (int)lStack14;
      }
      uVar4 = *(uint *)(uVar10 + 0x138);
      puVar7 = (undefined4 *)*(undefined4 *)(uVar10 + 0x136);
      uVar9 = uVar4 | (uint)puVar7;
      if (uVar9 != 0x0) {
        ppcVar5 = (code **)*puVar7;
        (**ppcVar5)(0x30,puVar7,uVar4,0x1);
        uVar9 = extraout_DX;
      }
      pass1_1018_435e(param_1,lStack14,local_12,local_10,uVar9,param_2);
      *(undefined2 *)(uVar10 + 0x136) = puVar7;
      *(uint *)(uVar10 + 0x138) = uVar9;
      piVar2 = (int *)(uVar10 + 0x13a);
      *piVar2 = *piVar2 + 0x1;
      wsprintf16((LPSTR)0x1030,(LPCSTR)(uVar10 + 0x22),valist);
      return;
    }
    *(uint *)(uVar10 + 0x13e) = uStack6;
    *(uint *)(uVar10 + 0x140) = uStack4;
    *(undefined2 *)(uVar10 + 0x13a) = 0x0;
    pass1_1008_612e(0x8,0xc,uStack6);
    *(uint *)(uVar10 + 0x13c) = uStack6;
  }
  return;
}



void __stdcall16far pass1_1018_36e6(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ushort *)(iVar1 + 0x12e) = param_4;
  *(ushort *)(iVar1 + 0x130) = param_3;
  *(ushort *)(iVar1 + 0x132) = param_2;
  *(undefined2 *)(iVar1 + 0x134) = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_3710(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined4 uVar1;
  code **ppcVar2;
  int iVar3;
  uint uVar4;
  uchar *puVar5;
  int iVar6;
  undefined2 uVar7;
  undefined in_AF;
  long lVar8;
  ushort *puVar9;
  undefined local_12a [0x118];
  undefined4 uStack18;
  ushort *puStack14;
  ulong uStack10;
  ushort *puStack6;
  
  puStack6 = (ushort *)0x0;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  uStack10 = switch_1018_3b9e((ulong)param_1,*(ushort *)(iVar6 + 0x12e),param_3,param_4,param_2);
  uVar4 = *(int *)(iVar6 + 0x12e) - 0x188;
  uStack18 = (ushort *)(uStack10 & 0xffff0000 | (ulong)uVar4);
  switch(uVar4) {
  case 0x0:
    lVar8 = pass1_1008_57f0(uStack10,*(int *)(iVar6 + 0x130),param_2);
    puVar5 = (uchar *)((ulong)lVar8 >> 0x10);
    iVar3 = (int)lVar8;
    mem_op_1000_179c(0x10,puVar5,0x1000);
    if (lVar8 != 0x0) {
      uStack18 = (ushort *)struct_1018_4790(lVar8,*(undefined4 *)(iVar6 + 0x132),0x0,*(ushort *)(iVar3 + 0xe));
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x1:
    puVar9 = (ushort *)pass1_1008_57f0(uStack10,*(int *)(iVar6 + 0x130),param_2);
    puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
    iVar3 = (int)puVar9;
    mem_op_1000_179c(0x14,puVar5,0x1000);
    uVar4 = (uint)((ulong)puVar9 >> 0x10) | (uint)puVar9;
    if (puVar9 != (ushort *)0x0) {
      struct_1018_47c8(puVar9,*(ulong *)(iVar6 + 0x132),0x0,*(ushort *)(iVar3 + 0x12),*(ulong *)(iVar3 + 0xe));
      uStack18 = (ushort *)((ulong)puVar9 & 0xffff | (ulong)uVar4 << 0x10);
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x2:
    puVar9 = (ushort *)pass1_1008_57f0(uStack10,*(int *)(iVar6 + 0x130),param_2);
    puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
    iVar3 = (int)puVar9;
    mem_op_1000_179c(0x12,puVar5,0x1000);
    uVar4 = (uint)((ulong)puVar9 >> 0x10) | (uint)puVar9;
    if (puVar9 != (ushort *)0x0) {
      pass1_1018_4808(puVar9,*(ulong *)(iVar6 + 0x132),0x0,*(ulong *)(iVar3 + 0xe));
      uStack18 = (ushort *)((ulong)puVar9 & 0xffff | (ulong)uVar4 << 0x10);
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x3:
    puVar9 = (ushort *)pass1_1008_57f0(uStack10,*(int *)(iVar6 + 0x130),param_2);
    puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
    iVar3 = (int)puVar9;
    mem_op_1000_179c(0x14,puVar5,0x1000);
    if (puVar9 != (ushort *)0x0) {
      uStack18 = struct_1018_4842(puVar9,*(ulong *)(iVar6 + 0x132),0x0,*(ushort *)(iVar3 + 0xe));
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x4:
    puVar9 = (ushort *)pass1_1008_57f0(uStack10,*(int *)(iVar6 + 0x130),param_2);
    puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
    iVar3 = (int)puVar9;
    mem_op_1000_179c(0x10,puVar5,0x1000);
    if (puVar9 != (ushort *)0x0) {
      uStack18 = struct_1018_48b0(puVar9,*(ulong *)(iVar6 + 0x132),0x0,*(ushort *)(iVar3 + 0xe));
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x5:
    puVar9 = (ushort *)pass1_1008_57f0(uStack10,*(int *)(iVar6 + 0x130),param_2);
    puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
    iVar3 = (int)puVar9;
    mem_op_1000_179c(0x12,puVar5,0x1000);
    if (puVar9 != (ushort *)0x0) {
      uStack18 = (ushort *)struct_1018_4920(puVar9,*(ulong *)(iVar6 + 0x132),0x0,*(ulong *)(iVar3 + 0xe));
      puStack6 = uStack18;
    }
    break;
  default:
    goto switchD_1018_393f_caseD_6;
  }
  uStack18 = (ushort *)0x0;
  puStack6 = uStack18;
switchD_1018_393f_caseD_6:
  uVar1 = *(undefined4 *)(iVar6 + 0x122);
  pass1_1008_e852((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(ulong *)(iVar6 + 0x126),param_2,
                  (uint)((ulong)uStack18 >> 0x10));
  uVar1 = *(undefined4 *)(iVar6 + 0x122);
  puStack14 = uStack18;
  pass1_1008_e852((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(ulong *)(iVar6 + 0x12a),param_2,
                  (uint)((ulong)uStack18 >> 0x10));
  pass1_1038_2a0e((astruct_100 *)CONCAT22(param_2,local_12a),*(ulong *)(iVar6 + 0x136),(ulong)puStack6,(ulong)uStack18,
                  (ulong)puStack14,param_2,in_AF);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,local_12a));
  *(undefined4 *)(iVar6 + 0x136) = 0x0;
  ppcVar2 = (code **)((int)*param_1 + 0x10);
  (**ppcVar2)(0x1030,param_1);
  pass1_1038_2a5c((ushort *)CONCAT22(param_2,local_12a));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far string_1018_39d8(ushort param_1,ulong param_2,ulong param_3,ulong param_4)

{
  int iVar1;
  char *pcVar2;
  long lVar3;
  ulong uVar4;
  
  uVar4 = param_3;
  pcVar2 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  iVar1 = pass1_1000_3d7a((ulong)pcVar2,uVar4);
  if (iVar1 != 0x0) {
    iVar1 = pass1_1000_3d7a(param_4,param_3);
    if (iVar1 != 0x0) {
      lVar3 = pass1_1018_4608(param_1,param_2,param_3,param_4);
      if ((lVar3 != 0x0) && (*(int *)((int)lVar3 + 0xc) == 0x1)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



void __stdcall16far pass1_1018_3a42(ulong param_1,ulong param_2,uint param_3,ushort param_4)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x122);
  pass1_1008_e852((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),param_2,param_4,param_3);
  return;
}



void __stdcall16far pass1_1018_3a5c(ulong param_1,ulong param_2,ulong param_3,ushort param_4)

{
  pass1_1008_e320(*(astruct_102 **)((int)param_1 + 0x122),param_2,param_3,param_4);
  return;
}



ulong __stdcall16far pass1_1018_3a7a(ulong param_1,ulong param_2,uint param_3,uint param_4)

{
  undefined4 uVar1;
  ulong uVar2;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x122);
  uVar2 = string_1008_e586((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),param_2,param_3,param_4);
  return uVar2;
}



void __stdcall16far pass1_1018_3a94(ulong param_1,ulong *param_2,ulong *param_3,ushort param_4)

{
  pass1_1008_e3ec(*(ulong *)((int)param_1 + 0x122),param_2,param_3,param_4);
  return;
}



ushort __stdcall16far pass1_1018_3ab2(ulong param_1,int param_2,int param_3,ushort param_4)

{
  undefined2 uVar1;
  undefined2 uVar2;
  int iVar3;
  long lVar4;
  ushort uStack22;
  undefined local_10 [0x8];
  int iStack8;
  ulong uStack6;
  
  if (0x5 < param_3 - 0x188U) {
    return 0x0;
  }
  iVar3 = (int)param_1;
  uVar2 = (undefined2)(param_1 >> 0x10);
  switch(param_3) {
  case 0x188:
    uVar1 = *(undefined2 *)(iVar3 + 0xa);
    uVar2 = *(undefined2 *)(iVar3 + 0xc);
    break;
  case 0x189:
    uVar1 = *(undefined2 *)(iVar3 + 0xe);
    uVar2 = *(undefined2 *)(iVar3 + 0x10);
    break;
  case 0x18a:
    uVar1 = *(undefined2 *)(iVar3 + 0x12);
    uVar2 = *(undefined2 *)(iVar3 + 0x14);
    break;
  case 0x18b:
    uVar1 = *(undefined2 *)(iVar3 + 0x16);
    uVar2 = *(undefined2 *)(iVar3 + 0x18);
    break;
  case 0x18c:
    uVar1 = *(undefined2 *)(iVar3 + 0x1a);
    uVar2 = *(undefined2 *)(iVar3 + 0x1c);
    break;
  case 0x18d:
    uVar1 = *(undefined2 *)(iVar3 + 0x1e);
    uVar2 = *(undefined2 *)(iVar3 + 0x20);
  }
  uStack6 = CONCAT22(uVar2,uVar1);
  iStack8 = 0x0;
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_10),uStack6);
  while( true ) {
    lVar4 = pass1_1008_5b12(local_10,param_4);
    uVar2 = (undefined2)((ulong)lVar4 >> 0x10);
    if ((lVar4 == 0x0) || (iStack8 == param_2)) break;
    iStack8 = iStack8 + 0x1;
  }
  uStack22 = 0x0;
  if (lVar4 != 0x0) {
    if (*(int *)((int)lVar4 + 0xa) == 0x0) {
      uStack22 = *(ushort *)((int)lVar4 + 0x8);
    }
    else {
      uStack22 = 0xffff;
    }
  }
  return uStack22;
}
