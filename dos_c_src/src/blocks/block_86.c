


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far
pass1_1030_acbe(ushort param_1,ushort param_2,ushort *param_3,long param_4,uint param_5,uint param_6,ushort param_7)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  
  pass1_1030_627e(param_7,param_5,param_6,_PTR_LOOP_1050_5740,param_3,param_4);
  uVar2 = param_6 | param_5;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,param_6);
    if ((uVar2 | param_5) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_5));
      if ((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far
pass1_1030_ad22(ushort param_1,ushort param_2,ushort *param_3,long param_4,uint param_5,uint param_6,ushort param_7)

{
  BOOL16 BVar1;
  uint uVar2;
  ulong uVar3;
  
  pass1_1030_627e(param_7,param_5,param_6,_PTR_LOOP_1050_5740,param_3,param_4);
  uVar2 = param_6 | param_5;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,param_6);
    if ((uVar2 | param_5) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_5));
      if (uVar3 != 0x0) {
        BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar3 + 0xc),0x46);
        return BVar1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_ad86(ushort param_1,ushort param_2,ushort *param_3,long param_4,ushort param_5,ushort param_6)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  char cStack17;
  undefined4 local_a;
  int iStack6;
  
  puVar2 = &local_a;
  pass1_1030_64ce(param_5,puVar2,param_6,_PTR_LOOP_1050_5740,param_3,param_4,(ulong *)CONCAT22(param_5,puVar2));
  uVar1 = *puVar2;
  cStack17 = (char)((ulong)uVar1 >> 0x18);
  if (cStack17 == '\0') {
    iStack6 = (int)uVar1;
    if (((0x0 < iStack6) && (!SBORROW2(iStack6,0x1))) &&
       ((iStack6 == 0x3 || iStack6 + -0x2 < 0x1 || ((0x3 < iStack6 + -0x3 && (iStack6 + -0x7 < 0x2)))))) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1030_addc(ushort param_1,ushort param_2,ushort *param_3,ushort param_4,ushort param_5,ulong param_6,int param_7,
               ushort param_8,ushort param_9)

{
  undefined4 *puVar1;
  int local_14;
  int local_12;
  int local_10;
  int local_e;
  undefined4 local_c;
  undefined2 uStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_6,(uint)(param_6 >> 0x10));
  iStack6 = param_7;
  uStack4 = param_8;
  puVar1 = (undefined4 *)pass1_1030_5b5c(param_7,param_8);
  local_c = *puVar1;
  uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
  pass1_1008_3e94(param_3,(ushort *)CONCAT22(param_9,&local_10),(ushort *)CONCAT22(param_9,&local_e));
  pass1_1008_3e94((ushort *)CONCAT22(param_9,&local_c),(ushort *)CONCAT22(param_9,&local_14),
                  (ushort *)CONCAT22(param_9,&local_12));
  if ((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) && (local_10 < local_14 + -0x1)) {
    return 0x1;
  }
  return 0x0;
}



void __stdcall16far pass1_1030_ae6c(ushort *param_1)

{
  undefined4 uVar1;
  uint uVar2;
  uchar *puVar3;
  undefined2 extraout_DX;
  astruct_399 *iVar4;
  undefined2 uVar4;
  ushort *puVar5;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_399 *)param_1;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  iVar4->field_0x4 = 0x0;
  puVar5 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x8));
  puVar3 = (uchar *)((ulong)puVar5 >> 0x10);
  uVar2 = 0x0;
  iVar4->field_0xe = 0x0;
  *(undefined4 *)&iVar4->field_0x10 = 0x0;
  *param_1 = 0xb932;
  iVar4->field_0x2 = 0x1030;
  mem_op_1000_179c(0xc,puVar3,0x1000);
  if (((uint)puVar3 | uVar2) == 0x0) {
    *(undefined4 *)&iVar4->field_0x10 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar3,uVar2));
    iVar4->field_0x10 = uVar2;
    iVar4->field_0x12 = extraout_DX;
  }
  uVar1 = *(undefined4 *)&iVar4->field_0x10;
  *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
  return;
}



void __stdcall16far pass1_1030_aefa(ushort *param_1,ulong param_2)

{
  undefined4 uVar1;
  uint uVar2;
  uchar *puVar3;
  undefined2 extraout_DX;
  undefined2 uVar4;
  astruct_400 *iVar5;
  undefined2 uVar5;
  ushort *puVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_400 *)param_1;
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  iVar5->field_0x4 = 0x0;
  puVar6 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x8));
  puVar3 = (uchar *)((ulong)puVar6 >> 0x10);
  iVar5->field_0xe = 0x0;
  *(undefined4 *)&iVar5->field_0x10 = 0x0;
  *param_1 = 0xb932;
  iVar5->field_0x2 = 0x1030;
  iVar5->field_0x4 = *(undefined4 *)((int)param_2 + 0x4);
  puVar6 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x8);
  pass1_1008_3f62(puVar6,(ushort *)(param_2 & 0xffff0000 | (ulong)((int)param_2 + 0xc)));
  uVar2 = (uint)puVar6;
  mem_op_1000_179c(0xc,puVar3,0x1000);
  if (((uint)puVar3 | uVar2) == 0x0) {
    uVar2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar3,uVar2));
    uVar4 = extraout_DX;
  }
  iVar5->field_0x10 = uVar2;
  iVar5->field_0x12 = uVar4;
  uVar1 = *(undefined4 *)&iVar5->field_0x10;
  *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
  return;
}



void __stdcall16far pass1_1030_afa6(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  astruct_614 *iVar5;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_614 *)param_1;
  *param_1 = 0xb932;
  iVar5->field_0x2 = 0x1030;
  if (*(long *)&iVar5->field_0x10 != 0x0) {
    uVar4 = *(undefined4 *)&iVar5->field_0x10;
    *(undefined2 *)((int)uVar4 + 0xa) = 0x1;
  }
  puVar1 = (undefined4 *)*(uint *)&iVar5->field_0x10;
  uVar2 = iVar5->field_0x12;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_affc(ulong param_1,int param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  uint uVar3;
  BOOL16 BVar4;
  uint uVar5;
  uint uVar6;
  ulong uVar7;
  ulong uVar8;
  int iStack12;
  undefined4 uStack10;
  undefined4 local_6;
  
  uVar8 = ZEXT24(&local_6);
  pass1_1030_b718((ushort)param_1,param_1._2_2_,(ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                  (ulong *)CONCAT22(param_3,&local_6),(uchar *)param_1._2_2_,param_2,param_3);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)local_6,(uint)((ulong)local_6 >> 0x10));
  uStack10 = uVar8 & 0xffff | (ulong)param_1._2_2_ << 0x10;
  uVar5 = param_1._2_2_ | (uint)uVar8;
  if (uVar5 != 0x0) {
    uVar7 = struct_op_1030_73a8(uVar8 & 0xffff | (ulong)param_1._2_2_ << 0x10);
    uVar5 = (uint)(uVar7 >> 0x10);
    iVar1 = *(int *)((int)uVar7 + 0xc);
    uVar8 = (ulong)(iVar1 - 0x16U);
    if ((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) {
      uVar8 = (ulong)(iVar1 - 0x17U);
      if (iVar1 - 0x17U != 0x0 && 0x0 < (int)(iVar1 - 0x16U)) {
        uVar8 = (ulong)(iVar1 - 0x19U);
        if ((iVar1 + -0x18 < 0x1) ||
           (uVar8 = (ulong)(iVar1 - 0x1aU), iVar1 - 0x1aU != 0x0 && 0x0 < (int)(iVar1 - 0x19U))) goto LAB_1030_b064;
      }
      *(undefined2 *)((int)uVar7 + 0x20) = 0x0;
    }
  }
LAB_1030_b064:
  iStack12 = 0x6;
  do {
    uVar3 = (uint)uVar8;
    if (iStack12 == 0x0) {
LAB_1030_b0fc:
      if ((uStack10._2_2_ | (uint)uStack10) != 0x0) {
        uVar8 = struct_op_1030_73a8(uStack10);
        uVar2 = (undefined2)(uVar8 >> 0x10);
        iVar1 = *(int *)((int)uVar8 + 0xc);
        if (((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) &&
           ((iVar1 == 0x17 || iVar1 + -0x16 < 0x1 || ((0x0 < iVar1 + -0x18 && (iVar1 + -0x19 < 0x2)))))) {
          *(undefined2 *)((int)uVar8 + 0x20) = 0x1;
        }
      }
      return;
    }
    pass1_1030_b578(param_1,param_2,param_3);
    if ((uVar5 | uVar3) == 0x0) goto LAB_1030_b0fc;
    uStack10 = CONCAT22(uVar5,uVar3);
    uVar8 = struct_op_1030_73a8(CONCAT22(uVar5,uVar3));
    uVar6 = (uint)(uVar8 >> 0x10);
    iVar1 = *(int *)((int)uVar8 + 0xc);
    pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                    (ushort *)CONCAT22(uVar5,uVar3 + 0xc));
    if ((iVar1 == 0x18) || (uVar5 = uVar6, iVar1 == 0x3f)) {
      pass1_1030_b142(param_1,uStack10);
      uVar5 = uVar6;
    }
    BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar1,0x40);
    uVar8 = (ulong)BVar4;
    if (BVar4 != 0x0) {
      pass1_1030_b454(param_1,uStack10,param_3);
      goto LAB_1030_b0fc;
    }
    iStack12 = iStack12 + -0x1;
  } while( true );
}



void __stdcall16far pass1_1030_b13c(void)

{
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1030_b142(ulong param_1,ulong param_2)

{
  int iVar1;
  int iVar2;
  undefined2 uVar3;
  bool bVar4;
  ulong uVar5;
  undefined4 uStack12;
  
  uVar5 = struct_op_1030_73a8(param_2);
  uVar3 = (undefined2)(uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  iVar2 = *(int *)(iVar1 + 0xc);
  uStack12 = 0x0;
  if (iVar2 == 0x18) {
    uStack12._2_2_ = pass1_1028_1c1c();
    uVar3 = *(undefined2 *)(iVar1 + 0x22);
  }
  else {
    if (iVar2 != 0x3f) goto LAB_1030_b1a6;
    uStack12._2_2_ = pass1_1028_20b0();
    uVar3 = *(undefined2 *)(iVar1 + 0x24);
  }
  uStack12 = CONCAT22(uStack12._2_2_,uVar3);
LAB_1030_b1a6:
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0xe) == 0x1) {
    bVar4 = (uStack12 & 0x10000) == 0x0;
  }
  else {
    if (*(int *)(iVar2 + 0xe) == 0x2) {
      bVar4 = (uStack12 & 0x20000) == 0x0;
    }
    else {
      if (*(int *)(iVar2 + 0xe) == 0x3) {
        bVar4 = (uStack12 & 0x40000) == 0x0;
      }
      else {
        bVar4 = (uStack12 & 0x80000) == 0x0;
      }
    }
  }
  if ((bVar4) || ((int)uStack12 != 0x0)) {
    bVar4 = false;
    while( true ) {
      if (((uStack12 & 0x10000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b239;
      if (((uStack12 & 0x20000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b247;
      if (((uStack12 & 0x40000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b255;
      if (((uStack12 & 0x80000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b263;
      if (bVar4) break;
      uStack12._1_3_ = (uint3)(uStack12 >> 0x8) & 0xffff00;
      iVar1 = *(int *)(iVar2 + 0xe);
      if (iVar1 == 0x1) {
        uStack12 = CONCAT31(uStack12._1_3_,0x4);
      }
      else {
        if (iVar1 == 0x2) {
          uStack12 = CONCAT31(uStack12._1_3_,0x8);
        }
        else {
          if (iVar1 == 0x3) {
            uStack12 = CONCAT31(uStack12._1_3_,0x1);
          }
          else {
            uStack12 = CONCAT31(uStack12._1_3_,0x2);
          }
        }
      }
      bVar4 = true;
    }
    if (*(int *)(iVar2 + 0xe) == 0x1) {
LAB_1030_b255:
      *(undefined2 *)(iVar2 + 0xe) = 0x3;
      return;
    }
    if (*(int *)(iVar2 + 0xe) == 0x2) {
LAB_1030_b263:
      *(undefined2 *)(iVar2 + 0xe) = 0x4;
      return;
    }
    if (*(int *)(iVar2 + 0xe) == 0x3) {
LAB_1030_b239:
      *(undefined2 *)(iVar2 + 0xe) = 0x1;
      return;
    }
    if (*(int *)(iVar2 + 0xe) == 0x4) {
LAB_1030_b247:
      *(undefined2 *)(iVar2 + 0xe) = 0x2;
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_b2aa(ulong param_1,ushort *param_2,uchar *param_3,int param_4,ushort param_5)

{
  uint uVar1;
  BOOL16 BVar2;
  ulong uVar3;
  byte bStack23;
  undefined4 local_6;
  
  pass1_1030_b718((ushort)param_1,(ushort)(param_1 >> 0x10),param_2,(ulong *)CONCAT22(param_5,&local_6),param_3,param_4,
                  param_5);
  bStack23 = (byte)((ulong)local_6 >> 0x18);
  uVar1 = (uint)bStack23;
  if (bStack23 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)local_6,local_6._2_2_);
    if ((local_6._2_2_ | uVar1) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(local_6._2_2_,uVar1));
      BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar3 + 0xc),0x42);
      if (BVar2 != 0x0) {
        pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                        (ushort *)CONCAT22(local_6._2_2_,uVar1 + 0xc));
        return;
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

ulong __stdcall16far pass1_1030_b344(ulong param_1,ushort param_2)

{
  uchar *puVar1;
  undefined4 *puStack18;
  uchar *puStack16;
  undefined local_e [0x2];
  int local_c;
  int local_a;
  undefined4 local_8;
  undefined2 uStack4;
  
  local_8 = *(ulong *)((int)param_1 + 0x8);
  uStack4 = *(undefined2 *)((int)param_1 + 0xc);
  puVar1 = param_1._2_2_;
  pass1_1008_3eb4((ushort *)CONCAT22(param_2,&local_8),(ushort *)CONCAT22(param_2,local_e),
                  (ushort *)CONCAT22(param_2,&local_c),(ushort *)CONCAT22(param_2,&local_a));
  local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
  puStack18 = &local_8;
  pass1_1030_b2aa(param_1,(ushort *)CONCAT22(param_2,puStack18),puVar1,(int)&stack0xfffe,param_2);
  puStack16 = (uchar *)((uint)puVar1 | (uint)puStack18);
  if (puStack16 == (uchar *)0x0) {
    local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
    puStack18 = &local_8;
    pass1_1030_b2aa(param_1,(ushort *)CONCAT22(param_2,puStack18),(uchar *)0x0,(int)&stack0xfffe,param_2);
    puVar1 = (uchar *)((uint)puStack16 | (uint)puStack18);
    if (puVar1 == (uchar *)0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8._2_2_ = local_c;
      puStack18 = &local_8;
      pass1_1030_b2aa(param_1,(ushort *)CONCAT22(param_2,puStack18),(uchar *)0x0,(int)&stack0xfffe,param_2);
      puStack16 = (uchar *)((uint)puVar1 | (uint)puStack18);
      if (puStack16 == (uchar *)0x0) {
        local_8 = CONCAT22(local_8._2_2_,local_a + 0x1);
        puStack18 = &local_8;
        pass1_1030_b2aa(param_1,(ushort *)CONCAT22(param_2,puStack18),(uchar *)0x0,(int)&stack0xfffe,param_2);
        if (((uint)puStack16 | (uint)puStack18) == 0x0) {
          return 0x0;
        }
        *(undefined2 *)((int)param_1 + 0xe) = 0x2;
      }
      else {
        *(undefined2 *)((int)param_1 + 0xe) = 0x4;
        puStack16 = puVar1;
      }
    }
    else {
      *(undefined2 *)((int)param_1 + 0xe) = 0x3;
    }
  }
  else {
    *(undefined2 *)((int)param_1 + 0xe) = 0x1;
    puStack16 = puVar1;
  }
  return CONCAT22(puStack16,puStack18);
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1030_b454(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  uint extraout_DX_00;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  ulong uVar8;
  undefined4 uVar9;
  long lStack38;
  undefined4 uStack30;
  undefined local_12 [0x4];
  undefined4 uStack14;
  ulong uStack10;
  long lStack6;
  
  lStack6 = *(long *)((int)param_2 + 0x4);
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_12),*(ulong *)(iVar6 + 0x10));
  while( true ) {
    puVar3 = local_12;
    pass1_1008_5b12(puVar3,param_3);
    uStack10 = CONCAT22(extraout_DX,puVar3);
    if ((extraout_DX | (uint)puVar3) == 0x0) break;
    if (*(long *)(puVar3 + 0x20) == lStack6) {
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x10) + 0xc);
      (**ppcVar2)();
      uStack14 = 0x0;
      pass1_1038_69fe(uStack10);
    }
  }
  uVar8 = struct_op_1030_73a8(param_2);
  iVar4 = (int)(uVar8 >> 0x10);
  puVar1 = (undefined4 *)*(ulong *)((int)uVar8 + 0x20);
  puVar3 = local_12;
  pass1_1008_5784((ulong *)CONCAT22(param_3,puVar3),(ulong)puVar1);
  pass1_1030_b13c();
  uStack30 = CONCAT22(-(uint)((undefined *)((int)s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2aU) < puVar3) - iVar4
                      ,0x1f4 - (int)puVar3);
  do {
    puVar3 = local_12;
    pass1_1008_5b12(puVar3,param_3);
    uStack10 = CONCAT22(extraout_DX_00,puVar3);
    uVar5 = extraout_DX_00 | (uint)puVar3;
    if (uVar5 == 0x0) {
      return;
    }
    pass1_1038_6984(CONCAT22(extraout_DX_00,puVar3));
    lStack38 = CONCAT22(uVar5,puVar3);
    if (((int)uVar5 <= uStack30._2_2_) && (((int)uVar5 < uStack30._2_2_ || (puVar3 <= (undefined *)uStack30)))) {
      uVar9 = *(undefined4 *)(iVar6 + 0x10);
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x10) + 0x8);
      (**ppcVar2)();
      uStack30 = uStack30 - lStack38;
      ppcVar2 = (code **)((int)*puVar1 + 0xc);
      (**ppcVar2)((int)&PTR_LOOP_1050_1038,(int)puVar1,(int)((ulong)puVar1 >> 0x10),uStack10,uVar9);
      uStack14 = 0x0;
    }
  } while (0x0 < uStack30);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_b578(ulong param_1,int param_2,ushort param_3)

{
  int iVar1;
  ulong *puVar2;
  uint uVar3;
  uchar *puVar4;
  bool bVar5;
  ulong uVar6;
  undefined4 uStack48;
  undefined local_1c [0x2];
  int local_1a;
  int local_18;
  ulong local_16;
  undefined2 uStack18;
  undefined2 uStack16;
  ulong uStack14;
  uint uStack10;
  uint uStack8;
  undefined4 local_6;
  
  pass1_1030_b718((ushort)param_1,(ushort)param_1._2_2_,
                  (ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),(ulong *)CONCAT22(param_3,&local_6),
                  param_1._2_2_,param_2,param_3);
  uStack48._3_1_ = (byte)((ulong)local_6 >> 0x18);
  uStack10 = (uint)uStack48._3_1_;
  if (uStack48._3_1_ == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)local_6,local_6._2_2_);
  uStack8 = local_6._2_2_;
  uStack14 = struct_op_1030_73a8(CONCAT22(local_6._2_2_,uStack10));
  uStack16 = *(undefined2 *)((int)uStack14 + 0xc);
  local_16 = *(ulong *)((ushort)param_1 + 0x8);
  uStack18 = *(undefined2 *)((ushort)param_1 + 0xc);
  puVar4 = param_1._2_2_;
  pass1_1008_3eb4((ushort *)CONCAT22(param_3,&local_16),(ushort *)CONCAT22(param_3,local_1c),
                  (ushort *)CONCAT22(param_3,&local_1a),(ushort *)CONCAT22(param_3,&local_18));
  iVar1 = *(int *)((ushort)param_1 + 0xe);
  if (iVar1 == 0x0) {
    pass1_1030_b344(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,param_3);
    return;
  }
  if (iVar1 == 0x1) {
    uVar3 = local_1a - 0x1;
LAB_1030_b63e:
    local_16 = local_16 & 0xffff | (ulong)uVar3 << 0x10;
    puVar2 = &local_16;
    pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,(ushort *)CONCAT22(param_3,puVar2),puVar4,
                    (int)&uStack16,param_3);
    uStack48 = CONCAT22(puVar4,puVar2);
    if (((uint)puVar4 | (uint)puVar2) == 0x0) {
      return;
    }
    uVar6 = struct_op_1030_73a8(CONCAT22(puVar4,puVar2));
    uVar3 = *(uint *)((int)uVar6 + 0xc);
    if (uVar3 == 0x3f) goto LAB_1030_b6e0;
    if (0x3f < uVar3) {
      return;
    }
    if ((char)uVar3 == '\x16') goto LAB_1030_b6e0;
    bVar5 = (char)uVar3 == '\x18';
  }
  else {
    if (iVar1 == 0x2) {
      uVar3 = local_18 + 0x1;
    }
    else {
      if (iVar1 == 0x3) {
        uVar3 = local_1a + 0x1;
        goto LAB_1030_b63e;
      }
      if (iVar1 != 0x4) {
        return;
      }
      uVar3 = local_18 - 0x1;
    }
    local_16 = local_16 & 0xffff0000 | (ulong)uVar3;
    puVar2 = &local_16;
    pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,(ushort *)CONCAT22(param_3,puVar2),puVar4,
                    (int)&uStack16,param_3);
    uStack48 = CONCAT22(puVar4,puVar2);
    if (((uint)puVar4 | (uint)puVar2) == 0x0) {
      return;
    }
    uVar6 = struct_op_1030_73a8(CONCAT22(puVar4,puVar2));
    iVar1 = *(int *)((int)uVar6 + 0xc);
    if (iVar1 < 0x17) {
      return;
    }
    if (SBORROW2(iVar1,0x17)) {
      return;
    }
    if (iVar1 == 0x18 || iVar1 + -0x17 < 0x1) goto LAB_1030_b6e0;
    bVar5 = iVar1 == 0x3f;
  }
  if (!bVar5) {
    return;
  }
LAB_1030_b6e0:
  pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                  (ushort *)(uStack48 & 0xffff0000 | (ulong)((int)uStack48 + 0xc)));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_b718(ushort param_1,ushort param_2,ushort *param_3,ulong *param_4,uchar *param_5,int param_6,ushort param_7)

{
  ulong *puVar1;
  undefined2 uVar2;
  ulong local_12 [0x2];
  long lStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar2 = (undefined2)((ulong)puStack6 >> 0x10);
  lStack10 = *(long *)((int)puStack6 + 0x20);
  puVar1 = local_12;
  pass1_1030_64ce(param_7,puVar1,uVar2,_PTR_LOOP_1050_5740,param_3,lStack10,(ulong *)CONCAT22(param_7,puVar1));
  *param_4 = *puVar1;
  return;
}



void __stdcall16far pass1_1030_b768(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  int iVar3;
  undefined *puVar4;
  uint extraout_DX;
  int iVar5;
  undefined2 uVar6;
  ushort uVar7;
  ushort uVar8;
  uint local_22 [0x4];
  undefined local_1a [0xa];
  ulong local_10;
  undefined *puStack12;
  uint uStack10;
  undefined2 local_8 [0x3];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  local_10 = *(ulong *)(iVar5 + 0x4);
  uVar7 = (ushort)param_2;
  uVar8 = (ushort)(param_2 >> 0x10);
  BVar2 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)&local_10,param_3,(char *)0x4,0x1008);
  if ((BVar2 != 0x0) &&
     (iVar3 = write_to_file_1008_7b4c(param_2,param_1 & 0xffff0000 | (ulong)(iVar5 + 0x8),0x1008,param_3), iVar3 != 0x0)
     ) {
    local_8[0] = *(undefined2 *)(iVar5 + 0xe);
    BVar2 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_8,param_3,(char *)0x2,0x1008);
    if (BVar2 != 0x0) {
      uVar1 = *(undefined4 *)(iVar5 + 0x10);
      local_22[0] = *(uint *)((int)uVar1 + 0x8);
      local_10 = local_10 & 0xffff0000 | (ulong)local_22[0];
      BVar2 = write_to_file_1008_7e1c(uVar7,uVar8,(ushort)local_22,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) {
        return;
      }
      pass1_1008_5784((ulong *)CONCAT22(param_3,local_1a),*(ulong *)(iVar5 + 0x10));
      do {
        puVar4 = local_1a;
        pass1_1008_5b12(puVar4,param_3);
        if ((extraout_DX | (uint)puVar4) == 0x0) {
          return;
        }
        puStack12 = puVar4;
        uStack10 = extraout_DX;
        pass1_1038_75ca(CONCAT22(extraout_DX,puVar4),param_2,(int)puVar4,param_3);
      } while (puVar4 != (undefined *)0x0);
      return;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



void __stdcall16far file_1030_b836(ulong param_1,ulong param_2,uchar *param_3,ushort param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  ushort uVar3;
  astruct_401 *iVar4;
  BOOL16 BVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uchar *puVar8;
  uchar *extraout_DX;
  ushort uVar9;
  ushort uVar10;
  undefined2 uVar11;
  ulong uVar12;
  undefined2 uVar13;
  uint local_12 [0x7];
  undefined2 local_4;
  
  iVar4 = (astruct_401 *)param_1;
  iVar4 = (astruct_401 *)&iVar4->field_0x4;
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar9 = (ushort)param_2;
  uVar10 = (ushort)(param_2 >> 0x10);
  BVar4 = read_file_1008_7dee(uVar9,uVar10,(ushort)iVar4,0x0,uVar3,0x4,0x1008);
  if (((BVar4 == 0x0) ||
      (BVar4 = read_file_1008_7bc8(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x8),0x1008,
                                   param_4), BVar4 == 0x0)) ||
     (BVar4 = read_file_1008_7dee(uVar9,uVar10,(ushort)&local_4,0x0,param_4,0x2,0x1008), BVar4 == 0x0)) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  else {
    iVar4->field_0xe = local_4;
    BVar4 = read_file_1008_7dee(uVar9,uVar10,(ushort)local_12,0x0,param_4,0x2,0x1008);
    if (BVar4 != 0x0) {
      while( true ) {
        if (local_12[0] == 0x0) {
          return;
        }
        uVar11 = 0x2a;
        uVar5 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        uVar12 = param_2;
        mem_op_1000_179c(0x2a,param_3,0x1000);
        puVar8 = (uchar *)((uint)param_3 | uVar5);
        if (puVar8 == (uchar *)0x0) {
          uVar6 = 0x0;
          puVar8 = (uchar *)0x0;
        }
        else {
          uVar6 = uVar5;
          struct_1038_6520((ushort *)CONCAT22(param_3,uVar5));
        }
        uVar13 = (undefined2)(uVar12 >> 0x10);
        uVar7 = uVar6;
        file_1038_774e(CONCAT22(puVar8,uVar6),CONCAT22((int)uVar12,uVar11),puVar8,param_4);
        if (uVar7 == 0x0) break;
        puVar1 = iVar4->field_0x10;
        ppcVar2 = (code **)((int)*iVar4->field_0x10 + 0x4);
        (**ppcVar2)((int)&PTR_LOOP_1050_1038,(int)puVar1,(int)((ulong)puVar1 >> 0x10),uVar6,puVar8,uVar13,uVar5);
        param_3 = extraout_DX;
      }
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_b90c(astruct_18 *param_1,byte param_2)

{
  pass1_1030_afa6(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1030_b936(astruct_365 *param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b22c((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xbc0c;
  param_1->field_0x2 = 0x1030;
  return;
}



void __stdcall16far pass1_1030_b96c(ushort *param_1)

{
  uint uVar1;
  astruct_18 *paVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = 0xbc0c;
  *(undefined2 *)(iVar3 + 0x2) = 0x1030;
  paVar2 = *(astruct_18 **)(iVar3 + 0xe);
  uVar1 = *(uint *)(iVar3 + 0x10);
  if ((uVar1 | (uint)paVar2) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10));
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1028_b260(param_1);
  return;
}



void __stdcall16far pass1_1030_b9b2(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0xe) = 0x0;
  *(undefined4 *)((int)param_1 + 0x12) = 0x0;
  return;
}



void __stdcall16far
pass1_1030_b9da(ulong param_1,ulong param_2,ulong param_3,ulong param_4,ushort param_5,ushort param_6,ushort param_7)

{
  ulong *puVar1;
  ulong uVar2;
  uchar *puVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  astruct_402 *iVar7;
  int iVar8;
  undefined2 uVar9;
  ulong uVar10;
  uint uStack12;
  uint uStack4;
  
  puVar3 = (uchar *)param_3;
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar7 = (astruct_402 *)param_1;
  if (iVar7->field_0xe == (long *)0x0) {
    mem_op_1000_179c(0xa,puVar3,0x1000);
    uVar4 = (uint)puVar3 | (uint)param_4;
    param_3 = (ulong)uVar4;
    if (uVar4 == 0x0) {
      iVar7->field_0xe = (long *)0x0;
    }
    else {
      pass1_1020_ba3e((long *)(param_4 & 0xffff | ZEXT24(puVar3) << 0x10),0x5,0x5,param_6,param_5);
      *(int *)&iVar7->field_0xe = (int)param_4;
      *(undefined2 *)((int)&iVar7->field_0xe + 0x2) = (int)param_3;
    }
    iVar7->field_0x12 = 0x0;
  }
  for (uStack4 = 0x4; (int)uStack4 < 0xe; uStack4 = uStack4 + 0x1) {
    uVar10 = pass1_1030_7c28(param_2,uStack4,(uint)param_4,(uint)param_3,param_7);
    uVar4 = (uint)(uVar10 >> 0x10);
    param_4 = uVar10 & 0xffff;
    uVar5 = uVar4 | (uint)param_4;
    param_3 = (ulong)uVar5;
    if (uVar5 != 0x0) {
      uVar2 = 0x64 - iVar7->field_0x12;
      uVar6 = uVar2 >> 0x10;
      uStack12 = (uint)uVar10;
      if (uVar10 < uVar2) {
        uVar2 = uVar10 & 0xffff;
        uVar6 = (ulong)uVar4;
      }
      uVar5 = (uint)uVar2;
      param_4 = uVar2 & 0xffff | uVar6 << 0x10;
      iVar8 = (uVar4 - (int)uVar6) - (uint)(uStack12 < uVar5);
      param_3 = uVar6;
      pass1_1030_7d1c(param_2,uStack12 - uVar5,CONCAT22(uStack4,iVar8),uVar5,(int)uVar6,iVar8,param_6,param_7);
      pass1_1020_bb8a(iVar7->field_0xe,uVar5,uVar6 | (ulong)uStack4 << 0x10,param_6,param_7);
      puVar1 = &iVar7->field_0x12;
      *puVar1 = *puVar1 + param_4;
      string_1020_c0ca(uStack4);
      vsprintf_op_1030_840a((ulong)s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c,0x1020,param_7,(ushort)param_3);
      if (0x63 < iVar7->field_0x12) break;
    }
  }
  if (iVar7->field_0x12 != 0x0) {
    return;
  }
  return;
}



void __stdcall16far
pass1_1030_bb0e(ulong param_1,ulong param_2,uint param_3,ushort param_4,ushort param_5,ushort param_6)

{
  astruct_18 *paVar1;
  ulong uVar2;
  ushort uVar3;
  ulong uVar4;
  uint in_DX;
  uchar *puVar5;
  ulong uVar6;
  ushort uStack8;
  
  uVar3 = pass1_1030_7bee(param_2);
  uVar4 = (ulong)uVar3;
  if (uVar3 != 0x0) {
    return;
  }
  pass1_1030_b9b2(param_1);
  uVar2 = uVar4 & 0xffff;
  paVar1 = (astruct_18 *)(uVar2 | (ulong)in_DX << 0x10);
  puVar5 = (uchar *)(in_DX | (uint)uVar4);
  if (puVar5 != (uchar *)0x0) {
    for (uStack8 = 0x4; (int)uStack8 < 0x25; uStack8 = uStack8 + 0x1) {
      uVar6 = pass1_1020_bae6((ushort)uVar2,CONCAT22(uStack8,in_DX),(uint)uVar4,(uint)puVar5,param_6);
      uVar4 = uVar6 & 0xffff;
      puVar5 = (uchar *)((uint)(uVar6 >> 0x10) | (uint)uVar4);
      if (puVar5 != (uchar *)0x0) {
        pass1_1030_7ddc(param_2,uVar6,uStack8,(uint)uVar4,puVar5,param_4,param_5,param_6);
        uVar3 = pass1_1030_7bee(param_2);
        uVar4 = (ulong)uVar3;
        if (uVar3 != 0x0) {
          return;
        }
        string_1020_c0ca(uStack8);
        vsprintf_op_1030_840a((ulong)s_truck_0x_08lx_unloaded__ld_of__s_1050_5798,0x1020,param_6,(ushort)puVar5);
        pass1_1020_bb8a((long *)paVar1,0x0,(ulong)uStack8 << 0x10,param_5,param_6);
      }
    }
    if (paVar1 != (astruct_18 *)0x0) {
      fn_ptr_1020_ba7e((ulong *)paVar1);
      fn_ptr_1000_17ce(paVar1,0x1000);
    }
  }
  return;
}


