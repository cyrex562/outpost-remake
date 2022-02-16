
int __cdecl16far pass1_1020_c7fa(ulong param_1,ulong param_2)

{
  return *(int *)((int)param_1 + 0x4) - *(int *)((int)param_2 + 0x4);
}



astruct_18 * __stdcall16far pass1_1020_c80e(astruct_18 *param_1,byte param_2)

{
  pass1_1020_c47a(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1020_c860(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x6),*(undefined2 *)((int)param_1 + 0x4));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_c872(ulong param_1,ulong param_2,ulong param_3)

{
  uint *puVar1;
  ulong *puVar2;
  int *piVar3;
  astruct_98 *uVar4;
  uint uVar6;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  bool bVar12;
  ulong uStack14;
  ulong uStack10;
  astruct_99 *puStack6;
  astruct_99 *uVar5;
  
  puStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_4fb8);
  uVar6 = (uint)((ulong)puStack6 >> 0x10);
  uVar5 = (astruct_99 *)puStack6;
  if ((uVar6 | (uint)uVar5) == 0x0) {
    puStack6 = (astruct_99 *)0x0;
  }
  else {
    puStack6->field_0x0 = 0x389a;
    uVar5->field_0x2 = 0x1008;
    uVar5->field_0x4 = 0x0;
    uVar5->field_0x8 = 0x0;
    puStack6->field_0x0 = 0x5bc0;
    uVar5->field_0x2 = 0x1008;
    uVar5->field_0xe = 0x0;
    uVar5->field_0xc = 0x0;
    puStack6->field_0x0 = 0xc9e6;
    uVar5->field_0x2 = 0x1020;
  }
  if (puStack6 == (astruct_99 *)0x0) {
    return;
  }
  uVar9 = (undefined2)((ulong)puStack6 >> 0x10);
  iVar7 = (int)puStack6;
  *(ulong *)(iVar7 + 0x8) = param_3;
  *(ulong *)(iVar7 + 0xc) = param_2;
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  uStack14 = *(ulong *)(iVar8 + 0x4);
  uVar11 = *(undefined2 *)(iVar8 + 0x6);
  if (*(int *)(iVar8 + 0x8) == 0x0) {
LAB_1020_c92d:
    *(undefined4 *)(iVar7 + 0x4) = *(undefined4 *)(iVar8 + 0x4);
  }
  else {
    puVar1 = (uint *)((int)uStack14 + 0xe);
    bVar12 = *puVar1 < param_2._2_2_;
    if ((bVar12 || *puVar1 == param_2._2_2_) &&
       ((bVar12 || (puVar1 = (uint *)((int)uStack14 + 0xc), *puVar1 < (uint)param_2 || *puVar1 == (uint)param_2))))
    goto LAB_1020_c92d;
    bVar12 = false;
    while( true ) {
      if (uStack14 == 0x0) break;
      uVar11 = (undefined2)(uStack14 >> 0x10);
      puVar2 = (ulong *)((int)uStack14 + 0xc);
      if (*puVar2 < param_2 || *puVar2 == param_2) {
        bVar12 = true;
        *(ulong *)(iVar7 + 0x4) = uStack14;
        *(astruct_99 **)((int)uStack10 + 0x4) = puStack6;
        break;
      }
      uStack10 = uStack14;
      uStack14 = *(ulong *)((int)uStack14 + 0x4);
    }
    param_1 = uStack10;
    if (bVar12) goto LAB_1020_c9ab;
  }
  uVar11 = (undefined2)(param_1 >> 0x10);
  *(int *)((int)param_1 + 0x4) = iVar7;
  *(undefined2 *)((int)param_1 + 0x6) = uVar9;
LAB_1020_c9ab:
  piVar3 = (int *)(iVar8 + 0x8);
  *piVar3 = *piVar3 + 0x1;
  return;
}



ushort * __stdcall16far pass1_1020_c9ba(ushort *param_1,byte param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x389a;
  ((int *)param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((int *)param_1,uVar1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_c9ea(ushort *param_1)

{
  struct_1028_0954(param_1);
  *param_1 = 0xcc7c;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_ca0c(astruct_179 *param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xcc7c;
  param_1->field_0x2 = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ca36(int param_1,ushort param_2,ushort param_3,int param_4,ushort param_5)

{
  uchar *puVar1;
  ulong uVar2;
  ushort *puVar3;
  
  pass1_1028_09b8(CONCAT22(param_2,param_1));
  uVar2 = pass1_1028_b4f2(CONCAT22(param_2,param_1));
  puVar1 = (uchar *)(uVar2 >> 0x10);
  if (*(long *)((int)uVar2 + 0x200) != 0x8000002) {
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,puVar1,param_4);
    pass1_1010_988c((ulong)puVar3,*(int *)(param_1 + 0xc));
  }
  return;
}



void __stdcall16far pass1_1020_ca82(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uchar *puVar1;
  uint uVar2;
  ulong uVar3;
  
  pass1_1028_be9e(param_1,param_2,param_3,(ushort)&USHORT_1050_1028,param_4);
  uVar3 = pass1_1028_b4f2((ulong)param_1);
  puVar1 = (uchar *)(uVar3 >> 0x10);
  if (*(long *)((int)uVar3 + 0x200) != 0x8000002) {
    uVar2 = (uint)((ulong)param_1 >> 0x10);
    if (*(int *)((int)param_1 + 0x12) == 0x5) {
      pass1_1020_cac2((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10,puVar1,param_2,param_3,param_4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_cac2(ulong param_1,uchar *param_2,ushort param_3,ushort param_4,ushort param_5)

{
  int *piVar1;
  code **ppcVar2;
  undefined *puVar3;
  undefined *puVar4;
  uint uVar5;
  int iVar6;
  uint extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 uVar7;
  ushort *puVar8;
  int iStack52;
  undefined *puStack36;
  undefined local_1c [0x4];
  undefined4 uStack24;
  undefined4 *puStack20;
  undefined4 *puStack16;
  ushort *puStack12;
  undefined *puStack8;
  undefined2 uStack6;
  uchar *puStack4;
  
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,param_2,param_4);
  puStack4 = (uchar *)((ulong)puVar8 >> 0x10);
  uStack6 = SUB42(puVar8,0x0);
  puStack8 = PTR_LOOP_1050_13ae;
  if (PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
    puStack8 = (undefined *)&PTR_LOOP_1050_0002;
  }
  puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,puStack4,param_4);
  uVar7 = (undefined2)((ulong)puStack12 >> 0x10);
  puStack16 = (undefined4 *)*(ulong *)((int)puStack12 + 0xa);
  puStack20 = (undefined4 *)*(undefined4 *)((int)puStack12 + 0xe);
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_1c),(ulong)puStack16);
  do {
    do {
      while( true ) {
        do {
          puVar3 = local_1c;
          pass1_1008_5b12(puVar3,param_5);
          if ((extraout_DX | (uint)puVar3) == 0x0) {
            return;
          }
          iVar6 = *(int *)(puVar3 + 0x4);
        } while ((iVar6 < 0x12) || (SBORROW2(iVar6,0x12)));
        if (iVar6 != 0x13 && 0x0 < iVar6 + -0x12) break;
        iStack52 = 0x0;
        if (puStack8 == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
          iStack52 = *(int *)(puVar3 + 0x6) / 0x2;
        }
        else {
          if (puStack8 == (undefined *)&DAT_1050_0004) {
            iVar6 = *(int *)(puVar3 + 0x6) * 0x3;
            iStack52 = (int)(iVar6 + (iVar6 >> 0xf & 0x3U)) >> 0x2;
          }
        }
        piVar1 = (int *)(puVar3 + 0x6);
        *piVar1 = *piVar1 - iStack52;
        uVar7 = (undefined2)((ulong)puStack16 >> 0x10);
        *(undefined2 *)((int)puStack16 + 0xa) = 0x0;
        ppcVar2 = (code **)((int)*puStack16 + 0xc);
        (**ppcVar2)(0x1008,(char)puStack16,uVar7,puVar3,extraout_DX);
        *(undefined2 *)((int)puStack16 + 0xa) = 0x1;
        uStack24 = 0x0;
        ppcVar2 = (code **)((int)*puStack20 + 0x4);
        (**ppcVar2)(0x1008,(int)puStack20,(int)((ulong)puStack20 >> 0x10),(char)puVar3,extraout_DX);
      }
    } while (iVar6 != 0x81);
    puStack36 = (undefined *)0x0;
    if (puStack8 == (undefined *)&PTR_LOOP_1050_0002) {
      iVar6 = *(int *)(puVar3 + 0x6);
LAB_1020_cba7:
      puVar4 = (undefined *)((int)(iVar6 + (iVar6 >> 0xf & 0x3U)) >> 0x2);
      puStack36 = puVar4;
    }
    else {
      if (puStack8 == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
        puVar4 = (undefined *)(*(int *)(puVar3 + 0x6) / 0x2);
        puStack36 = puVar4;
      }
      else {
        puVar4 = puStack8 + -0x4;
        if (puVar4 == (undefined *)0x0) {
          iVar6 = *(int *)(puVar3 + 0x6) * 0x3;
          goto LAB_1020_cba7;
        }
      }
    }
    pass1_1028_b58e(param_1);
    uVar5 = *(int *)(puVar3 + 0x6) - (int)puStack36;
    pass1_1030_7ddc(CONCAT13((char)((uint)extraout_DX_00 >> 0x8),CONCAT12((char)extraout_DX_00,puVar4)),(long)(int)uVar5
                    ,0x1e,uVar5,(uchar *)((int)uVar5 >> 0xf),param_3,param_4,param_5);
    ppcVar2 = (code **)((int)*puStack16 + 0xc);
    (**ppcVar2)(0x1030,(char)puStack16,(int)((ulong)puStack16 >> 0x10),puVar3,extraout_DX);
    uStack24 = 0x0;
  } while( true );
}



astruct_18 * __stdcall16far pass1_1020_cc56(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_cce4(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0xcd7e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_cd06(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xcd7e;
  *(undefined2 *)(param_1 + 0x2) = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far pass1_1020_cd30(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (((*(int *)(iVar1 + 0x12) == 0x6) || (*(int *)(iVar1 + 0x12) == 0x5)) && ((*(byte *)(iVar1 + 0x1a) & 0x2) != 0x0))
  {
    return 0x1;
  }
  return 0x0;
}



astruct_18 * __stdcall16far pass1_1020_cd58(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_cde6(ushort *param_1)

{
  struct_1028_0954(param_1);
  *param_1 = 0xd004;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_ce08(astruct_179 *param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xd004;
  param_1->field_0x2 = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ce32(int param_1,ushort param_2,int param_3,ushort param_4)

{
  uchar *puVar1;
  ulong uVar2;
  ushort *puVar3;
  astruct_67 *paVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  int iVar7;
  ushort uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  int iVar11;
  
  pass1_1028_09b8(CONCAT22(param_2,param_1));
  uVar2 = pass1_1028_b4f2(CONCAT22(param_2,param_1));
  puVar1 = (uchar *)(uVar2 >> 0x10);
  if (*(long *)((int)uVar2 + 0x200) != 0x8000002) {
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_4,puVar1,param_3);
    puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
    pass1_1010_988c((ulong)puVar3,*(int *)(param_1 + 0xc));
    uVar10 = 0x0;
    iVar11 = 0x9;
    uVar8 = 0x1;
    uVar9 = 0x0;
    uVar6 = 0x0;
    iVar7 = 0x0;
    uVar5 = 0x0;
    paVar4 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,puVar1,param_3);
    post_win_msg_1008_a0e4(paVar4,CONCAT22(uVar6,uVar5),iVar7,uVar8,CONCAT22(uVar10,uVar9),iVar11,0x1008,param_4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ce9e(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uchar *puVar1;
  ulong uVar2;
  ushort *puVar3;
  
  pass1_1028_be9e((ulong *)param_1,param_4,param_2,(ushort)&USHORT_1050_1028,param_3);
  if (*(int *)((int)param_1 + 0x12) == 0x5) {
    pass1_1020_cefc(param_1,param_2,param_3);
    uVar2 = pass1_1028_b4f2(param_1);
    puVar1 = (uchar *)(uVar2 >> 0x10);
    if (*(long *)((int)uVar2 + 0x200) != 0x8000002) {
      puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_3,puVar1,param_2);
      pass1_1010_043a((ulong)puVar3,*(long *)((int)uVar2 + 0x4),0x5,param_3);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_cefc(ulong param_1,int param_2,ushort param_3)

{
  uchar *puVar1;
  ulong uVar2;
  ushort *puVar3;
  ushort uStack12;
  
  uVar2 = pass1_1028_b4f2(param_1);
  puVar1 = (uchar *)(uVar2 >> 0x10);
  if (*(long *)((int)uVar2 + 0x200) == 0x8000002) {
    uStack12 = 0x32;
  }
  else {
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_3,puVar1,param_2);
    uStack12 = pass1_1010_96c2((ulong)puVar3);
    if (0x32 < (int)uStack12) {
      uStack12 = 0x32;
    }
    pass1_1010_96a8((ulong)puVar3,uStack12);
  }
  pass1_1020_cf6c((ushort)param_1,(ushort)(param_1 >> 0x10),uStack12,uVar2);
  return;
}



void __stdcall16far pass1_1020_cf6c(ushort param_1,ushort param_2,int param_3,ulong param_4)

{
  uint *puVar1;
  int *piVar2;
  uint uVar3;
  undefined4 uVar4;
  undefined2 uVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  undefined2 uVar10;
  
  uVar10 = (undefined2)(param_4 >> 0x10);
  uVar4 = *(undefined4 *)((int)param_4 + 0x1f6);
  iVar6 = (int)uVar4;
  uVar5 = (undefined2)((ulong)uVar4 >> 0x10);
  uVar7 = param_3 / 0x5;
  uVar8 = uVar7 * -0x4 + param_3;
  puVar1 = (uint *)(iVar6 + 0x50);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar8;
  piVar2 = (int *)(iVar6 + 0x52);
  *piVar2 = *piVar2 + ((int)uVar8 >> 0xf) + (uint)CARRY2(uVar3,uVar8);
  iVar9 = (int)uVar7 >> 0xf;
  puVar1 = (uint *)(iVar6 + 0x78);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (int *)(iVar6 + 0x7a);
  *piVar2 = *piVar2 + iVar9 + (uint)CARRY2(uVar3,uVar7);
  puVar1 = (uint *)(iVar6 + 0xa0);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (int *)(iVar6 + 0xa2);
  *piVar2 = *piVar2 + iVar9 + (uint)CARRY2(uVar3,uVar7);
  puVar1 = (uint *)(iVar6 + 0xc8);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (int *)(iVar6 + 0xca);
  *piVar2 = *piVar2 + iVar9 + (uint)CARRY2(uVar3,uVar7);
  puVar1 = (uint *)(iVar6 + 0xf0);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (int *)(iVar6 + 0xf2);
  *piVar2 = *piVar2 + iVar9 + (uint)CARRY2(uVar3,uVar7);
  *(undefined2 *)((int)param_4 + 0x1fe) = 0x1;
  return;
}



astruct_18 * __stdcall16far pass1_1020_cfde(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_d06c(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0xd314;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_d08e(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xd314;
  *(undefined2 *)(param_1 + 0x2) = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1020_d0b8(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ulong uVar1;
  int iVar2;
  
  if (*(int *)((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar1 = pass1_1028_b4f2((ulong)param_1);
  iVar2 = (int)uVar1;
  if (*(long *)(iVar2 + 0x200) != 0x8000002) {
    pass1_1028_cb04((ulong)param_1,param_2,param_3,param_4);
    if ((iVar2 == 0x0) || (pass1_1020_d194((ulong)param_1,param_3,param_4), iVar2 == 0x0)) {
      iVar2 = 0x6;
      goto LAB_1020_d10b;
    }
    pass1_1028_c952((ulong)param_1,param_2,param_3,param_4);
  }
  iVar2 = 0x5;
LAB_1020_d10b:
  pass1_1028_bdac(param_1,iVar2,(ushort)&USHORT_1050_1028);
  return;
}



undefined2 __stdcall16far
pass1_1020_d118(ulong param_1,ushort *param_2,ulong param_3,ulong param_4,int param_5,ushort param_6,ushort param_7)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_6,uVar2,uVar3,param_2,param_4);
  if (param_5 == 0x5) {
    pass1_1028_c23e(uVar2,uVar3,param_2,param_3,param_4,0x5,param_6,param_7);
    if (param_5 != 0x0) {
      pass1_1028_c3aa(uVar2,uVar3,param_2,param_3,param_4,param_7);
      if (param_5 != 0x0) {
        BVar1 = pass1_1028_c314(param_7,param_5,param_6,uVar2,uVar3,param_2,(ushort)param_3,(ushort)(param_3 >> 0x10),
                                param_4);
        if (BVar1 != 0x0) {
          return 0x1;
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = (undefined *)0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_d194(ulong param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  undefined2 uVar2;
  undefined *puVar3;
  ushort uVar4;
  undefined2 uVar5;
  ulong uVar6;
  uint uVar7;
  uchar *extraout_DX;
  uchar *puVar8;
  uchar *puVar9;
  uint extraout_DX_00;
  uint uVar10;
  undefined2 uVar11;
  ulong uVar12;
  ushort *puVar13;
  ulong *puVar14;
  undefined uVar15;
  undefined uVar16;
  uchar *puVar17;
  uint uVar18;
  uint uVar19;
  ulong uStack42;
  ulong uStack38;
  undefined4 *puStack34;
  undefined local_4 [0x2];
  
  pass1_1030_bcae((ushort)local_4,param_3);
  uVar12 = pass1_1028_b4f2(param_1);
  uVar7 = (uint)(uVar12 >> 0x10);
  uVar6 = *(ulong *)((int)uVar12 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar6,(uint)(uVar6 >> 0x10));
  uVar2 = (undefined2)uVar6;
  pass1_1028_b58e(param_1);
  puVar3 = local_4;
  puVar8 = extraout_DX;
  pass1_1030_bd74((ushort)puVar3,param_3,uVar6 & 0xffff | (ulong)uVar7 << 0x10,CONCAT22(extraout_DX,uVar2),param_3);
  if ((int)puVar3 < 0x0) {
    return;
  }
  if (0x1e < (int)puVar3) {
    uVar4 = 0x87;
    puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_3,puVar8,param_2);
    uVar4 = pass1_1010_65d0(param_3,(ulong)puVar13,uVar4);
    if (uVar4 == 0x0) {
      puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x15);
      puVar9 = (uchar *)((ulong)puVar14 >> 0x10);
      uVar7 = (uint)puVar14;
      uVar11 = SUB42(&PTR_LOOP_1050_1038,0x0);
      pass1_1038_4e78(uVar7,puVar9,uVar12,puVar14);
      puStack34 = (undefined4 *)CONCAT22(puVar9,uVar7);
      ppcVar1 = (code **)((int)*puStack34 + 0x10);
      uVar10 = uVar7;
      uVar18 = uVar7;
      puVar8 = puVar9;
      (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar7,puVar9);
      uStack38 = CONCAT22(extraout_DX_00,uVar10);
      uStack42 = 0x0;
      uVar10 = extraout_DX_00;
      while( true ) {
        if (uStack38 <= uStack42) {
          if (puStack34 == (undefined4 *)0x0) {
            return;
          }
          ppcVar1 = (code **)*puStack34;
          (**ppcVar1)(uVar11,uVar7,(char)puVar9,0x1,uVar18,puVar8,puStack34,puStack34);
          return;
        }
        uVar15 = (undefined)uVar2;
        uVar16 = (undefined)((uint)uVar2 >> 0x8);
        uVar6 = uStack38;
        puVar17 = extraout_DX;
        pass1_1030_1d58((ulong)puStack34);
        uVar5 = (undefined2)uVar6;
        puVar3 = local_4;
        uVar11 = 0x1030;
        uVar19 = uVar10;
        pass1_1030_bd74((ushort)puVar3,param_3,uVar6 & 0xffff | (ulong)uVar10 << 0x10,
                        CONCAT22(puVar17,CONCAT11(uVar16,uVar15)),param_3);
        if ((0x0 < (int)puVar3) && ((int)puVar3 < 0x1f)) break;
        uStack42 = uStack42 + 0x1;
      }
      if (puStack34 == (undefined4 *)0x0) {
        return;
      }
      ppcVar1 = (code **)*puStack34;
      (**ppcVar1)(0x1030,uVar7,(char)puVar9,0x1,uVar18,puVar8,puStack34,puStack34,uVar5,uVar19);
      return;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_d2ee(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_d37c(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x20) = 0x0;
  *param_1 = 0xd53e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_d3a4(ushort *param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  undefined2 uVar1;
  
  pass1_1028_b39e(param_1,param_3,param_4,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x20) = param_2;
  *param_1 = 0xd53e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



BOOL16 __stdcall16far write_to_file_1020_d3d4(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  undefined2 local_c [0x5];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    local_c[0] = *(undefined2 *)((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)local_c,param_3,(char *)0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 __stdcall16far pass1_1020_d41a(ulong param_1,ulong param_2,BOOL16 param_3,uchar *param_4,ushort param_5)

{
  BOOL16 BVar1;
  undefined2 local_4;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    BVar1 = read_file_1008_7dee((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)&local_4,0x0,param_5,0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return BVar1;
    }
    *(undefined2 *)((int)param_1 + 0x20) = local_4;
    param_3 = 0x1;
  }
  return param_3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1020_d460(ulong *param_1,ushort *param_2,ulong param_3,ulong param_4,int param_5,ushort param_6,int param_7,
               undefined8 param_8)

{
  ushort uVar1;
  uchar *puVar2;
  ushort unaff_SS;
  undefined4 uVar3;
  ushort *puVar4;
  
  uVar1 = pass1_1028_bc90(param_1,param_2,param_3,param_4,param_5,param_6,unaff_SS);
  if (uVar1 != 0x0) {
    uVar3 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)((ushort)_PTR_LOOP_1050_4230 + 0x16),0x11,param_6,
                            (ushort)_PTR_LOOP_1050_4230,(ushort)&PTR_LOOP_1050_1038,unaff_SS);
    puVar2 = (uchar *)((ulong)uVar3 >> 0x10);
    PTR_LOOP_1050_5b80 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
    unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5b80,0x1008,unaff_SS);
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3a,unaff_SS,puVar2,param_7);
    *(undefined2 *)((int)param_1 + 0x20) = *(undefined2 *)((int)puVar4 + 0xa);
    uVar1 = 0x1;
  }
  return uVar1;
}



void __stdcall16far pass1_1020_d4ca(ulong param_1,int param_2)

{
  BOOL16 BVar1;
  ulong uVar2;
  uint extraout_DX;
  uint uVar3;
  int iVar4;
  
  if (*(int *)((int)param_1 + 0x20) == 0x2) {
    return;
  }
  pass1_1028_b58e(param_1);
  uVar2 = *(ulong *)(param_2 + 0x2e);
  iVar4 = 0x63;
  uVar3 = extraout_DX;
  pass1_1038_3fb0(uVar2);
  BVar1 = pass1_1030_25b2(uVar2 & 0xffff | (ulong)uVar3 << 0x10,iVar4);
  if (BVar1 != 0x0) {
    return;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_d518(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_d5a6(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0xd7fe;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_d5c8(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xd7fe;
  *(undefined2 *)(param_1 + 0x2) = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_d5f2(ulong param_1,ulong param_2,int param_3,ushort param_4)

{
  code **ppcVar1;
  ushort uVar2;
  undefined4 *puVar3;
  ushort extraout_DX;
  uint uVar4;
  ulong uVar5;
  undefined4 *puVar6;
  ulong *puVar7;
  byte bStack55;
  undefined local_32 [0xa];
  undefined4 uStack40;
  undefined4 uStack36;
  undefined4 *puStack28;
  undefined4 local_1a;
  int iStack22;
  ushort uStack20;
  int iStack18;
  ushort uStack16;
  int iStack14;
  undefined4 local_c;
  int iStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_b58e(param_1);
  local_c = *(undefined4 *)(param_3 + 0xc);
  iStack18 = *(int *)(param_3 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_3;
  uStack4 = extraout_DX;
  pass1_1028_bab6(param_1,iStack18,extraout_DX);
  uVar2 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36._2_2_,&local_1a);
  iStack14 = iStack14 + 0x1;
  uStack20 = uVar2;
  if (iStack14 < (int)uVar2) {
    puVar7 = (ulong *)CONCAT22(param_4,local_32);
    iStack22 = iStack14;
    uVar5 = pass1_1028_bb24(param_1);
    uVar4 = (uint)(uVar5 >> 0x10);
    puVar3 = &local_1a;
    pass1_1030_64ce(param_4,puVar3,uVar4,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_4,puVar3),
                    uVar5 & 0xffff | (ulong)uVar4 << 0x10,puVar7);
    uStack40 = *puVar3;
    uVar4 = *(uint *)((int)puVar3 + 0x2);
    bStack55 = (byte)((ulong)uStack40 >> 0x18);
    uVar2 = (ushort)bStack55;
    uStack36 = uStack40;
    if (bStack55 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack40,uVar4);
      puVar6 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar4,uVar2));
      uVar2 = (ushort)puVar6;
      ppcVar1 = (code **)((int)*puVar6 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(param_1,param_2,uVar2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_d6e6(ulong param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  ushort extraout_DX;
  uint uVar4;
  ulong uVar5;
  undefined4 *puVar6;
  ulong *puVar7;
  byte bStack55;
  undefined local_32 [0xa];
  undefined4 uStack40;
  undefined4 uStack36;
  undefined4 *puStack28;
  undefined4 local_1a;
  int iStack22;
  ushort uStack20;
  int iStack18;
  ushort uStack16;
  int iStack14;
  undefined4 local_c;
  int iStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_b514(param_1);
  pass1_1028_b58e(param_1);
  local_c = *(undefined4 *)(param_2 + 0xc);
  iStack18 = *(int *)(param_2 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_2;
  uStack4 = extraout_DX;
  pass1_1028_bab6(param_1,iStack18,extraout_DX);
  uStack20 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  uStack36 = CONCAT22(uStack36._2_2_,&local_1a);
  iStack22 = iStack14 + 0x1;
  if (iStack22 < (int)uStack20) {
    puVar7 = (ulong *)CONCAT22(param_3,local_32);
    iStack14 = iStack22;
    uVar5 = pass1_1028_bb24(param_1);
    uVar4 = (uint)(uVar5 >> 0x10);
    puVar2 = &local_1a;
    pass1_1030_64ce(param_3,puVar2,uVar4,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_3,puVar2),
                    uVar5 & 0xffff | (ulong)uVar4 << 0x10,puVar7);
    uStack40 = *puVar2;
    uVar4 = *(uint *)((int)puVar2 + 0x2);
    bStack55 = (byte)((ulong)uStack40 >> 0x18);
    uVar3 = (uint)bStack55;
    if (bStack55 != 0x0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack40,uVar4);
      puVar6 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar4,uVar3));
      if (*(int *)((int)puVar6 + 0xc) == 0x6a) {
        ppcVar1 = (code **)((int)*puVar6 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}

