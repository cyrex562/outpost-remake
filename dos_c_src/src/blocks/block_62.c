
ushort * __stdcall16far pass1_1028_0982(astruct_179 *param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  param_1->field_0x20 = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xada;
  param_1->field_0x2 = (int)&USHORT_1050_1028;
  param_1->field_0xe = 0x4b;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1028_09b8(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = pass1_1028_b58e(param_1);
  *(undefined2 *)((int)uVar1 + 0x14) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_09d4(ushort param_1,int param_2,ushort param_3,ulong param_4,ushort *param_5,ulong param_6,long param_7)

{
  int iVar1;
  undefined *puVar2;
  uint uVar3;
  ulong uVar4;
  ushort uVar5;
  ushort uVar6;
  uint uVar7;
  undefined local_6 [0x2];
  BOOL16 BStack4;
  
  uVar5 = (ushort)param_4;
  uVar6 = (ushort)(param_4 >> 0x10);
  uVar7 = (uint)(param_6 >> 0x10);
  BStack4 = pass1_1028_c314(param_1,param_2,param_3,uVar5,uVar6,param_5,(ushort)param_6,uVar7,param_7);
  if (BStack4 == 0x0) {
    return;
  }
  pass1_1028_c7b6(param_1,param_3,uVar5,uVar6,param_5,param_7);
  if ((BStack4 != 0x0) && (BStack4 != 0x4)) {
    if (((int)(BStack4 - 0x5) < 0x1) || ((SBORROW2(BStack4 - 0x5,0x1) || (0x3 < (int)(BStack4 - 0x6))))) {
      if ((*(int *)(uVar5 + 0xc) != 0x3e) && (*(int *)(uVar5 + 0xc) != 0x41)) {
        return;
      }
      uVar4 = pass1_1030_bcae((ushort)local_6,param_1);
      uVar3 = (uint)(uVar4 >> 0x10);
      iVar1 = (int)uVar4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_6,uVar7);
      uVar4 = *(ulong *)(iVar1 + 0x10);
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar4,(uint)(uVar4 >> 0x10));
      puVar2 = local_6;
      pass1_1030_bcde(param_1,(ushort)puVar2,param_1,uVar4 & 0xffff | (ulong)uVar3 << 0x10,param_5,param_7);
      if ((int)puVar2 < 0x0) {
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
        return;
      }
      if (0x5 < (int)puVar2) {
        PTR_LOOP_1050_50ca = (undefined *)0x6b5;
        return;
      }
      return;
    }
  }
  PTR_LOOP_1050_50ca = (undefined *)0x6a8;
  return;
}



astruct_18 * __stdcall16far pass1_1028_0ab4(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_0b42(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0xbbc;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_0b64(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0xbbc;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



astruct_18 * __stdcall16far pass1_1028_0b96(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_0c24(ushort *param_1)

{
  astruct_191 *iVar1;
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_191 *)param_1;
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x22 = 0x0;
  *param_1 = (int)s_480_bmp_1050_1721 + 0x3;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_0c50(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined2 *)(param_1 + 0x20) = 0x0;
  *(undefined2 *)(param_1 + 0x22) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = (int)s_480_bmp_1050_1721 + 0x3;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_0c84(ulong param_1,ulong param_2,int param_3,ushort param_4)

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
  ulong uStack6;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_3);
  local_c = *(undefined4 *)(param_3 + 0xc);
  iStack18 = *(int *)(param_3 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  pass1_1028_bab6(param_1,iStack18,extraout_DX);
  uVar2 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36._2_2_,&local_1a);
  iStack14 = iStack14 + 0x1;
  uStack20 = uVar2;
  if (iStack14 <= (int)uVar2) {
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
  fn_ptr_1030_7296(uStack6);
  return;
}



ushort __stdcall16far pass1_1028_0d80(ulong param_1)

{
  ushort uVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  uVar1 = *(ushort *)((int)param_1 + 0x20);
  pass1_1028_1646(param_1 & 0xffff | (ulong)uVar2 << 0x10);
  return uVar1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_0d9c(ulong param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 *puVar2;
  uint uVar3;
  ushort uVar4;
  BOOL16 BVar5;
  ushort extraout_DX;
  uint uVar6;
  ulong uVar7;
  ulong *puVar8;
  undefined4 uStack58;
  undefined local_32 [0x6];
  undefined4 *puStack44;
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
  if (iStack22 <= (int)uStack20) {
    puVar8 = (ulong *)CONCAT22(param_3,local_32);
    iStack14 = iStack22;
    uVar7 = pass1_1028_bb24(param_1);
    uVar6 = (uint)(uVar7 >> 0x10);
    puVar2 = &local_1a;
    pass1_1030_64ce(param_3,puVar2,uVar6,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_3,puVar2),
                    uVar7 & 0xffff | (ulong)uVar6 << 0x10,puVar8);
    uStack40 = *puVar2;
    uVar6 = *(uint *)((int)puVar2 + 0x2);
    uStack58._3_1_ = (byte)((ulong)uStack40 >> 0x18);
    uVar3 = (uint)uStack58._3_1_;
    if (uStack58._3_1_ != 0x0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack40,uVar6);
      uStack58 = CONCAT22(uVar6,uVar3);
      uVar4 = pass1_1030_6fa0(CONCAT22(uVar6,uVar3));
      BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar4,0x13);
      if (BVar5 != 0x0) {
        puStack44 = (undefined4 *)struct_op_1030_73a8(uStack58);
        ppcVar1 = (code **)((int)*puStack44 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_0ea6(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  int *piVar1;
  BOOL16 BVar2;
  ushort uVar3;
  astruct_597 *iVar3;
  uint uVar4;
  
  uVar4 = (uint)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_597 *)param_1;
  if (iVar3->field_0xc != 0x10) {
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar3->field_0xc,0x13);
    if (BVar2 == 0x0) {
      BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar3->field_0xc,0x2);
      if (((BVar2 != 0x0) && (iVar3->field_0x12 != 0x7)) && (iVar3->field_0x12 != 0x4)) {
        uVar3 = pass1_1028_1556((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10,BVar2,param_2,param_5);
        if (uVar3 == 0x0) goto LAB_1028_0f0a;
        if (iVar3->field_0x12 == 0x9) {
          iVar3->field_0x12 = 0x5;
        }
      }
    }
    else {
      if (iVar3->field_0x22 < 0x1) {
        if ((iVar3->field_0x12 != 0x5) && (iVar3->field_0x12 != 0x6)) {
          return;
        }
        fn_ptr_1000_17ce(iVar3->field_0x14,0x1000);
        iVar3->field_0x14 = (astruct_18 *)0x0;
LAB_1028_0f0a:
        iVar3->field_0x12 = 0x9;
        return;
      }
    }
    pass1_1028_be2a(param_1,param_3,param_4,0x1008,param_5);
    if (iVar3->field_0x12 == 0x5) {
      piVar1 = &iVar3->field_0x22;
      *piVar1 = *piVar1 + -0x1;
      pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_0fa4(ulong *param_1,uchar *param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6)

{
  BOOL16 BVar1;
  int iVar2;
  undefined2 uVar3;
  ushort *puVar4;
  ulong uVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  int iVar8;
  
  pass1_1028_be9e(param_1,param_3,param_4,param_5,param_6);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_6,param_2,param_4);
  iVar8 = *(int *)((int)puVar4 + 0x82);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0x12) == 0x5) {
    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(iVar2 + 0xc),0x2);
    if ((BVar1 != 0x0) && ((PTR_LOOP_1050_4fbc == (undefined *)0x0 || (iVar8 != 0x0)))) {
      PTR_LOOP_1050_4fbc = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
      uVar7 = 0x0;
      iVar8 = 0x4;
      uVar6 = 0x1;
      uVar5 = pass1_1028_b58e((ulong)param_1);
      pass1_1030_7c50(uVar5,CONCAT22(uVar7,uVar6),iVar8,(uint)uVar5,(uchar *)(uVar5 >> 0x10));
    }
    *(undefined2 *)(iVar2 + 0x22) = 0x64;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1028_1024(ulong param_1,int param_2,ushort param_3,ushort param_4)

{
  BOOL16 BVar1;
  undefined4 *puVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  int iStack26;
  int iStack24;
  undefined4 local_16;
  int iStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  ulong uStack12;
  ushort uStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_2;
  uStack4 = param_3;
  uStack8 = pass1_1030_2fac(CONCAT22(param_3,param_2));
  uStack12 = pass1_1028_bb24(param_1);
  uVar6 = pass1_1028_b58e(param_1);
  uStack14 = (undefined2)(uVar6 >> 0x10);
  local_16 = *(undefined4 *)((int)uVar6 + 0xc);
  iStack26 = 0x0;
  iStack24 = 0x0;
  while( true ) {
    if ((int)uStack8 < iStack26) {
      return iStack24;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(param_4,(uint)puVar2,(uint)(uVar6 >> 0x10),_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_4,puVar2),
                    uStack12);
    uStack16 = (undefined2)uVar6;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar2,(uint)(uVar6 >> 0x10));
    uStack16 = (undefined2)uVar6;
    if (((uint)(uVar6 >> 0x10) | (uint)puVar2) == 0x0) break;
    uVar6 = struct_op_1030_73a8(uVar6 & 0xffff0000 | ZEXT24(puVar2));
    uVar4 = (uint)(uVar6 >> 0x10);
    uVar3 = (uint)uVar6;
    uVar5 = uVar4 | uVar3;
    if (uVar6 == 0x0) {
      return iStack24;
    }
    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(uVar3 + 0xc),0x13);
    uVar6 = CONCAT22(uVar5,uStack16);
    if ((BVar1 != 0x0) && (*(int *)(uVar3 + 0x12) == 0x5)) {
      iStack24 = iStack24 + 0x1;
    }
    iStack26 = iStack26 + 0x1;
  }
  return iStack24;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1028_1106(ulong param_1,int param_2,ushort param_3,ushort param_4)

{
  BOOL16 BVar1;
  undefined4 *puVar2;
  uint uVar3;
  uint uVar4;
  ulong uVar5;
  int iStack26;
  int iStack24;
  undefined4 local_16;
  int iStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  ulong uStack12;
  ushort uStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_2;
  uStack4 = param_3;
  uStack8 = pass1_1030_2fac(CONCAT22(param_3,param_2));
  uStack12 = pass1_1028_bb24(param_1);
  uVar5 = pass1_1028_b58e(param_1);
  uStack14 = (undefined2)(uVar5 >> 0x10);
  local_16 = *(undefined4 *)((int)uVar5 + 0xc);
  iStack26 = 0x0;
  iStack24 = 0x0;
  while( true ) {
    if ((int)uStack8 < iStack26) {
      return iStack24;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(param_4,(uint)puVar2,(uint)(uVar5 >> 0x10),_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_4,puVar2),
                    uStack12);
    uStack16 = (undefined2)uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar2,(uint)(uVar5 >> 0x10));
    uStack16 = (undefined2)uVar5;
    if (((uint)(uVar5 >> 0x10) | (uint)puVar2) == 0x0) break;
    uVar5 = struct_op_1030_73a8(uVar5 & 0xffff0000 | ZEXT24(puVar2));
    uVar3 = (uint)(uVar5 >> 0x10);
    uVar4 = uVar3 | (uint)uVar5;
    if (uVar5 == 0x0) {
      return iStack24;
    }
    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((uint)uVar5 + 0xc),0x13);
    uVar5 = CONCAT22(uVar4,uStack16);
    if (BVar1 != 0x0) {
      iStack24 = iStack24 + 0x1;
    }
    iStack26 = iStack26 + 0x1;
  }
  return iStack24;
}



bool __stdcall16far pass1_1028_11de(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = pass1_1028_b58e(param_1);
  return *(int *)((int)uVar1 + 0x10) == 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1028_121e(ulong param_1,ushort param_2)

{
  bool bVar1;
  undefined extraout_AH;
  undefined4 *puVar2;
  uint uVar3;
  ulong uVar4;
  undefined4 local_c;
  undefined2 uStack8;
  undefined4 uStack6;
  
  bVar1 = pass1_1028_11de(param_1);
  if (CONCAT11(extraout_AH,bVar1) != 0x0) {
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



ushort __stdcall16far pass1_1028_12be(ulong param_1,ulong *param_2,ushort param_3)

{
  int *piVar1;
  uint uVar2;
  code **ppcVar3;
  bool bVar4;
  undefined extraout_AH;
  ushort uVar5;
  undefined4 *puVar6;
  ulong uVar7;
  ulong uVar8;
  ushort uStack8;
  
  bVar4 = pass1_1028_11de(param_1);
  if (CONCAT11(extraout_AH,bVar4) == 0x0) {
    puVar6 = (undefined4 *)pass1_1028_121e(param_1,param_3);
    ppcVar3 = (code **)((int)*puVar6 + 0x40);
    uVar5 = (**ppcVar3)();
    return uVar5;
  }
  *param_2 = 0x0;
  uVar7 = pass1_1028_b58e(param_1);
  uStack8 = 0x4;
  uVar8 = uVar7;
  do {
    uVar8 = pass1_1030_7c28(uVar7,uStack8,(uint)uVar8,(uint)(uVar8 >> 0x10),param_3);
    uVar2 = *(uint *)param_2;
    *(uint *)param_2 = *(int *)param_2 + (uint)uVar8;
    piVar1 = (int *)((int)param_2 + 0x2);
    *piVar1 = *piVar1 + (int)(uVar8 >> 0x10) + (uint)CARRY2(uVar2,(uint)uVar8);
    uStack8 = uStack8 + 0x1;
  } while ((int)uStack8 < 0xe);
  if (0x1f4 < *param_2) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_134a(ulong *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int *piVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  long *plVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  ulong uVar7;
  long lStack26;
  int iStack22;
  ulong uStack18;
  undefined4 uStack10;
  long local_6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)param_1 + 0xc),0x13);
  if (BVar3 != 0x0) {
    plVar4 = &local_6;
    ppcVar2 = (code **)((int)*param_1 + 0x40);
    (**ppcVar2)(0x1008,param_1,plVar4,param_4);
    if (plVar4 != (long *)0x0) {
      piVar1 = (int *)((int)param_1 + 0x22);
      *piVar1 = *piVar1 + 0x1;
      return;
    }
    uStack10 = 0x1f4 - local_6;
    uVar7 = pass1_1028_121e((ulong)param_1,param_4);
    uVar5 = (undefined2)(uVar7 >> 0x10);
    uVar6 = (undefined2)uVar7;
    pass1_1028_b58e(uVar7);
    uStack18 = CONCAT22(uVar5,uVar6);
    for (iStack22 = 0x0; iStack22 < 0xa; iStack22 = iStack22 + 0x1) {
      uStack10._0_2_ = *(uint *)(iStack22 * 0x2 + 0x4fbe);
      uStack10._2_2_ = (uchar *)((int)(uint)uStack10 >> 0xf);
      if (uStack10 < (int)(uint)uStack10) {
      }
      lStack26 = CONCAT22(uStack10._2_2_,(uint)uStack10);
      pass1_1030_7ddc(uStack18,CONCAT13((char)((uint)uStack10._2_2_ >> 0x8),
                                        CONCAT12((char)uStack10._2_2_,(uint)uStack10)),iStack22 + 0x4,(uint)uStack10,
                      uStack10._2_2_,param_2,param_3,param_4);
      uStack10 = uStack10 - lStack26;
      if (uStack10 < 0x1) {
        return;
      }
    }
  }
  return;
}



int __stdcall16far pass1_1028_1416(ulong param_1,ushort param_2,ushort param_3)

{
  bool bVar1;
  undefined extraout_AH;
  int iVar2;
  uint uVar3;
  ulong uVar4;
  
  bVar1 = pass1_1028_11de(param_1);
  if (CONCAT11(extraout_AH,bVar1) == 0x0) {
    uVar4 = pass1_1028_121e(param_1,param_3);
    uVar3 = (uint)(uVar4 >> 0x10);
    iVar2 = pass1_1028_1416(uVar4 & 0xffff | (ulong)uVar3 << 0x10,uVar3,param_3);
    return iVar2;
  }
  iVar2 = pass1_1028_1024(param_1,CONCAT11(extraout_AH,bVar1),param_2,param_3);
  return iVar2 * 0xf;
}



undefined2 __stdcall16far write_to_file_1028_1452(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  undefined2 uVar2;
  ushort uVar3;
  ushort uVar4;
  undefined2 local_c [0x3];
  undefined *local_6 [0x2];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    uVar2 = (undefined2)(param_1 >> 0x10);
    local_c[0] = *(undefined2 *)((int)param_1 + 0x22);
    uVar3 = (ushort)param_2;
    uVar4 = (ushort)(param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)local_c,param_3,(char *)0x2,0x1008);
    if (BVar1 != 0x0) {
      local_6[0] = (undefined *)*(undefined2 *)((int)param_1 + 0x20);
      BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)local_6,param_3,(char *)0x2,0x1008);
      if (BVar1 != 0x0) {
        local_6[0] = PTR_LOOP_1050_4fbc;
        BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)local_6,param_3,(char *)0x2,0x1008);
        if (BVar1 != 0x0) {
          return 0x1;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



void __stdcall16far pass1_1028_14d8(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  ushort uVar1;
  BOOL16 BVar2;
  ushort uVar3;
  ushort uVar4;
  undefined2 local_4;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar3 = (ushort)param_2;
    uVar4 = (ushort)(param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(uVar3,uVar4,(int)param_1 + 0x22,0x0,uVar1,0x2,0x1008);
    if ((BVar2 != 0x0) &&
       (BVar2 = read_file_1008_7dee(uVar3,uVar4,(ushort)&local_4,0x0,param_5,0x2,0x1008), BVar2 != 0x0)) {
      *(undefined2 *)((int)param_1 + 0x20) = local_4;
      if ((int)PTR_LOOP_1050_0312 < 0x2) {
        return;
      }
      BVar2 = read_file_1008_7dee(uVar3,uVar4,(ushort)&PTR_LOOP_1050_4fbc,0x0,(ushort)&USHORT_1050_1050,0x2,0x1008);
      if (BVar2 != 0x0) {
        return;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_1556(ulong param_1,int param_2,ushort param_3,ushort param_4)

{
  int iVar1;
  undefined4 *puVar2;
  uint uVar3;
  BOOL16 BVar4;
  uint uVar5;
  uint uVar6;
  ulong uVar7;
  int iStack26;
  undefined4 local_16;
  int iStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  ulong uStack12;
  ushort uStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_2;
  uStack4 = param_3;
  uStack8 = pass1_1030_2fac(CONCAT22(param_3,param_2));
  uStack12 = pass1_1028_bb24(param_1);
  uVar7 = pass1_1028_b58e(param_1);
  uStack14 = (undefined2)(uVar7 >> 0x10);
  local_16 = *(undefined4 *)((int)uVar7 + 0xc);
  iStack26 = 0x1;
  while( true ) {
    if ((int)uStack8 < iStack26) {
      return 0x0;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(param_4,(uint)puVar2,(uint)(uVar7 >> 0x10),_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_4,puVar2),
                    uStack12);
    uStack16 = (undefined2)uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar2,(uint)(uVar7 >> 0x10));
    uStack16 = (undefined2)uVar7;
    if (((uint)(uVar7 >> 0x10) | (uint)puVar2) == 0x0) {
      return 0x0;
    }
    uVar7 = struct_op_1030_73a8(uVar7 & 0xffff0000 | ZEXT24(puVar2));
    uVar5 = (uint)(uVar7 >> 0x10);
    uVar3 = (uint)uVar7;
    uVar6 = uVar5 | uVar3;
    if (uVar7 == 0x0) {
      return 0x0;
    }
    iVar1 = *(int *)(uVar3 + 0xc);
    BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar1,0x13);
    uVar7 = CONCAT22(uVar6,uStack16);
    if ((BVar4 == 0x0) && (iVar1 != 0x75)) break;
    if (*(int *)(uVar3 + 0x12) != 0x9) {
      return 0x1;
    }
    iStack26 = iStack26 + 0x1;
  }
  return 0x0;
}



astruct_409 * __stdcall16far pass1_1028_1646(ulong param_1)

{
  astruct_409 *paVar1;
  astruct_409 *uVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar2 = (astruct_409 *)param_1;
  paVar1 = (astruct_409 *)(uVar2->field_0x20 + -0x4);
  if (paVar1 < (astruct_409 *)&DAT_1050_0009) {
    switch(paVar1) {
    case (astruct_409 *)0x0:
      uVar2->field_0x20 = 0x5;
      break;
    case (astruct_409 *)0x1:
      uVar2->field_0x20 = 0x6;
      break;
    case (astruct_409 *)0x2:
      uVar2->field_0x20 = 0x7;
      break;
    case (astruct_409 *)0x3:
      uVar2->field_0x20 = 0x8;
      break;
    case (astruct_409 *)0x4:
      uVar2->field_0x20 = 0x9;
      break;
    case (astruct_409 *)0x5:
      uVar2->field_0x20 = 0xa;
      return uVar2;
    case (astruct_409 *)0x6:
      uVar2->field_0x20 = 0xb;
      return uVar2;
    case (astruct_409 *)0x7:
      uVar2->field_0x20 = 0xc;
      return uVar2;
    case (astruct_409 *)0x8:
      uVar2->field_0x20 = 0xd;
      return uVar2;
    }
    return uVar2;
  }
  uVar2->field_0x20 = 0x4;
  return paVar1;
}



astruct_18 * __stdcall16far pass1_1028_16fe(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_178c(ushort *param_1)

{
  struct_1030_dc96(param_1);
  *param_1 = 0x1b54;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_17ae(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1030_dcc2(param_1,param_2,param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x1b54;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1028_17d8(ushort param_1,ushort param_2,ushort param_3)

{
  undefined2 extraout_DX;
  
  pass1_1030_df0c(CONCAT22(param_2,param_1),param_3);
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  pass1_1038_57dc(*(ulong *)(param_3 + 0x2e),0x1,0x3);
  return;
}



void __stdcall16far pass1_1028_1812(ulong *param_1,ushort param_2)

{
  pass1_1028_bdac(param_1,0x2,param_2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_1824(ulong param_1,ulong *param_2,ulong param_3,ulong param_4,int param_5,ushort param_6,int param_7,
               ushort param_8)

{
  BOOL16 BVar1;
  ulong *puVar2;
  uint uVar3;
  ushort uVar4;
  uchar *puVar5;
  ushort uVar6;
  ulong uVar7;
  ushort uVar8;
  ushort uVar9;
  ulong local_2a;
  int iStack38;
  int iStack36;
  uint uStack34;
  uint uStack32;
  uchar *puStack30;
  ushort uStack28;
  int iStack24;
  ushort *puStack22;
  ushort uStack16;
  uint uStack14;
  ulong local_c;
  int iStack8;
  ulong uStack6;
  
  uVar8 = (ushort)param_1;
  uVar9 = (ushort)(param_1 >> 0x10);
  pass1_1028_c3aa(uVar8,uVar9,(ushort *)param_2,param_3,param_4,param_8);
  if (param_5 == 0x0) {
    return;
  }
  BVar1 = pass1_1028_c314(param_8,param_5,param_6,uVar8,uVar9,(ushort *)param_2,(ushort)param_3,
                          (ushort)(param_3 >> 0x10),param_4);
  if (BVar1 == 0x0) {
    return;
  }
  puVar2 = &local_c;
  pass1_1030_64ce(param_8,puVar2,param_6,_PTR_LOOP_1050_5740,(ushort *)param_2,param_4,(ulong *)CONCAT22(param_8,puVar2)
                 );
  uStack6 = *puVar2;
  puVar5 = *(uchar **)((int)puVar2 + 0x2);
  uVar6 = (ushort)((ulong)param_2 >> 0x10);
  iStack8 = *(int *)((int)param_2 + 0x4);
  puStack22 = (ushort *)(uStack6 & 0xffff | ZEXT24(puVar5) << 0x10);
  uStack32 = (ushort)uStack6;
  uVar3 = (uint)puVar5 >> 0x8;
  if ((char)((uint)puVar5 >> 0x8) != '\0') {
    puStack30 = puVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack6,(uint)puVar5);
    uStack32 = uVar3;
    puStack30 = puVar5;
    uStack28 = pass1_1030_6fa0(CONCAT22(puVar5,uVar3));
    if (uStack28 == 0x10) {
      if (iStack8 != 0x0) {
        PTR_LOOP_1050_50ca = (undefined *)0x6ab;
        return;
      }
      return;
    }
    if ((uStack28 == 0x60) || (uStack28 == 0x61)) {
      puStack22 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_8,puVar5,param_7);
      uVar7 = pass1_1018_04b8((ulong)puStack22);
      uStack34 = (uint)(uVar7 >> 0x10);
      uStack16 = (ushort)uVar7;
      iStack36 = *(int *)((int)puStack22 + 0x1e);
      iStack24 = iStack36;
      uStack14 = uStack34;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack16,uStack34);
      uVar4 = pass1_1030_2fac(CONCAT22(uStack34,iStack36));
      if ((int)uVar4 <= iStack24) {
        PTR_LOOP_1050_50ca = (undefined *)0x6ac;
        return;
      }
      local_2a = *param_2;
      iStack38 = iStack8 + 0x1;
      puVar2 = &local_2a;
      pass1_1028_c7b6(param_8,uVar6,uVar8,uVar9,(ushort *)CONCAT22(param_8,puVar2),param_4);
      if (puVar2 == (ulong *)0x0) {
        return;
      }
      if (puVar2 == (ulong *)((int)&DAT_1050_0004 + 0x2)) {
        return;
      }
      return;
    }
  }
  PTR_LOOP_1050_50ca = (undefined *)0x6aa;
  return;
}
