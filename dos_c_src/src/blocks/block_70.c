
void __stdcall16far pass1_1028_94e4(ulong param_1,astruct_328 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_329 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x124,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_329 *)param_1;
    param_2->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_2->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    param_2->field_0x108 = iVar5->field_0x108;
    param_2->field_0x10c = iVar5->field_0x10c;
    param_2->field_0x110 = iVar5->field_0x110;
    param_2->field_0x114 = iVar5->field_0x114;
    param_2->field_0x118 = iVar5->field_0x118;
    param_2->field_0x11a = iVar5->field_0x11a;
    param_2->field_0x11c = iVar5->field_0x11c;
    param_2->field_0x11e = iVar5->field_0x11e;
    param_2->field_0x122 = iVar5->field_0x122;
    *puStack10 = 0x9934;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



void __stdcall16far pass1_1028_9600(ulong param_1,uchar *param_2,int param_3,ushort param_4,uchar param_5)

{
  ushort *puVar1;
  undefined local_6 [0x4];
  
  puVar1 = pass1_1020_a43e(param_4,param_2,(ushort *)CONCAT22(param_4,local_6));
  pass1_1020_a80e((ushort)local_6,param_4,*(int *)((int)param_1 + 0x11a),(uint)local_6,(uint)((ulong)puVar1 >> 0x10),
                  param_4,param_5,param_3);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_9624(ulong param_1,ushort param_2,uchar *param_3,ushort param_4,int param_5,uchar param_6)

{
  code **ppcVar1;
  ulong *puVar2;
  undefined2 uVar3;
  uint uVar4;
  BOOL16 BVar5;
  ulong uVar7;
  uchar *extraout_DX;
  uint extraout_DX_00;
  astruct_688 *iVar9;
  undefined2 uVar8;
  undefined2 uVar9;
  uchar *puVar10;
  undefined2 uStack332;
  undefined2 uStack330;
  undefined2 uStack64;
  undefined4 uStack62;
  int iStack58;
  undefined4 uStack56;
  undefined4 *puStack46;
  ulong uStack42;
  undefined local_26 [0x4];
  undefined2 uStack34;
  uchar *puStack32;
  ulong uStack30;
  undefined4 uStack26;
  undefined4 *puStack22;
  undefined local_12 [0x2];
  undefined local_10 [0x2];
  undefined local_e [0x2];
  uint uStack12;
  undefined4 uStack10;
  ushort *puStack6;
  undefined4 *puVar6;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar9 = (astruct_688 *)param_1;
  uVar7 = iVar9->field_0x10c;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar7,(uint)(uVar7 >> 0x10));
  *(ushort *)&iVar9->field_0x110 = param_2;
  *(uchar **)((int)&iVar9->field_0x110 + 0x2) = param_3;
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_3,param_5);
  uStack10._2_2_ = (uchar *)((ulong)puStack6 >> 0x10);
  puVar2 = &iVar9->field_0x114;
  pass1_1030_64ce(param_4,puVar2,uStack10._2_2_,_PTR_LOOP_1050_5740,(ushort *)(param_1 & 0xffff0000 | ZEXT24(puVar2)),
                  iVar9->field_0x108,(ulong *)CONCAT22(param_4,local_26));
  uStack56 = (undefined4 *)*puVar2;
  uStack56._3_1_ = (char)((ulong)uStack56 >> 0x18);
  uStack12 = (uint)(uStack56._3_1_ != '\0');
  uVar9 = 0x1008;
  puStack46 = uStack56;
  uStack10 = uStack56;
  pass1_1008_3eb4((ushort *)(param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x114)),(ushort *)CONCAT22(param_4,local_12),
                  (ushort *)CONCAT22(param_4,local_10),(ushort *)CONCAT22(param_4,local_e));
  if (uStack12 == 0x0) {
    puVar2 = &iVar9->field_0x114;
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    puStack22 = (undefined4 *)CONCAT22(uStack10._2_2_,puVar2);
    uVar9 = 0x1030;
    pass1_1030_61fe(_PTR_LOOP_1050_5740,CONCAT22(uStack10._2_2_,puVar2),
                    param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x114),iVar9->field_0x108,(ushort)puVar2,
                    (ushort)uStack10._2_2_,param_4);
    if ((iVar9->field_0x11a == 0xa) || (iVar9->field_0x11a == 0x37)) {
      if (iVar9->field_0x11a == 0x37) {
        uStack56 = iVar9->field_0x11e;
        uStack10._2_2_ = *(uchar **)((int)&iVar9->field_0x11e + 0x2);
        uStack42 = iVar9->field_0x10c;
        *(ulong *)((int)uStack56 + 0x20) = uStack42;
      }
      puVar2 = &iVar9->field_0x114;
      pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x400);
      *(ulong **)&iVar9->field_0x10c = puVar2;
      *(uchar **)((int)&iVar9->field_0x10c + 0x2) = uStack10._2_2_;
      uVar9 = 0x1018;
      pass1_1018_0196((ulong)puStack6,CONCAT22(uStack10._2_2_,*(undefined2 *)&iVar9->field_0x10c),iVar9->field_0x108,
                      (ushort)puVar2,uStack10._2_2_,param_4);
      if (iVar9->field_0x11a == 0xa) {
        uVar9 = 0x1010;
        pass1_1010_ed22((ulong)puStack6,iVar9->field_0x10c,param_4);
      }
    }
    uVar7 = iVar9->field_0x10c;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar7,(uint)(uVar7 >> 0x10));
    *(ulong **)&iVar9->field_0x110 = puVar2;
    *(uchar **)((int)&iVar9->field_0x110 + 0x2) = uStack10._2_2_;
    uVar4 = (uint)uStack10._2_2_ | *(uint *)&iVar9->field_0x110;
    puVar6 = (undefined4 *)(ulong)uVar4;
    if (uVar4 == 0x0) goto LAB_1028_9807;
    uVar3 = SUB42(puStack22,0x0);
    puVar10 = (uchar *)((ulong)puStack22 >> 0x10);
  }
  else {
    puStack22 = uStack10;
    puVar6 = uStack10;
    if (iVar9->field_0x11a != 0x75) goto LAB_1028_9807;
    uVar3 = SUB42(uStack10,0x0);
    puVar10 = uStack10._2_2_;
    uStack10._2_2_ = *(uchar **)((int)&iVar9->field_0x110 + 0x2);
  }
  ppcVar1 = (code **)((int)*iVar9->field_0x110 + 0x8);
  (**ppcVar1)(uVar9,*(undefined2 *)&iVar9->field_0x110,uStack10._2_2_,0x0,uVar3,puVar10,0x0);
  uStack10._2_2_ = extraout_DX;
LAB_1028_9807:
  uVar9 = SUB42(puVar6,0x0);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puStack22,(uint)((ulong)puStack22 >> 0x10));
  uStack26 = CONCAT22(uStack10._2_2_,uVar9);
  pass1_1030_73ee(CONCAT22(uStack10._2_2_,uVar9),iVar9->field_0x10c,(ushort)uStack10._2_2_);
  BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar9->field_0x11a,0x31);
  puStack32 = uStack10._2_2_;
  if ((BVar5 == 0x0) && (iVar9->field_0x122 == 0x0)) {
    uStack62 = *(undefined4 *)((int)uStack26 + 0xc);
    iStack58 = *(int *)((int)uStack26 + 0x10);
    uStack56 = (undefined4 *)((ulong)uStack56 & 0xffff0000 | ZEXT24(&uStack62));
    if (iStack58 < 0x1) {
      uStack64 = 0x5;
    }
    else {
      uStack64 = 0x6;
    }
    *(undefined2 *)((int)uStack26 + 0x14) = uStack64;
    puStack32 = uStack26._2_2_;
  }
  uVar7 = *(ulong *)((int)uStack26 + 0x16);
  uStack30 = uVar7;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar7,(uint)(uVar7 >> 0x10));
  uStack34 = (undefined2)uVar7;
  if (uStack30 != 0x0) {
    struct_1030_e4fa((astruct_100 *)CONCAT22(param_4,&uStack332),uStack30,param_4,param_6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,&uStack332));
    uStack332 = 0x389a;
    uStack330 = 0x1008;
  }
  ppcVar1 = (code **)((int)*iVar9->field_0x11e + 0x4);
  (**ppcVar1)();
  puVar6 = iVar9->field_0x11e;
  pass1_1030_7e5a(uStack26,*(ulong *)((int)puVar6 + 0x4),extraout_DX_00);
  return;
}



astruct_18 * __stdcall16far pass1_1028_9908(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
pass1_1028_9944(astruct_100 *param_1,ulong param_2,ulong param_3,ulong param_4,ushort param_5,uchar param_6)

{
  astruct_699 *iVar1;
  undefined2 uVar1;
  
  struct_op_1028_d1dc(param_5,param_6,param_1,0x1387);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_699 *)param_1;
  iVar1->field_0x108 = param_4;
  iVar1->field_0x10c = param_3;
  iVar1->field_0x110 = param_2;
  iVar1->field_0x114 = 0x0;
  param_1->field_0x0 = 0x9c52;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  return;
}



void __stdcall16far pass1_1028_9992(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x9c52;
  *(undefined2 *)(iVar1 + 0x2) = (int)&USHORT_1050_1028;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x114),0x1000);
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_99c4(ulong param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  pass1_1030_355c(*(ulong *)(param_2 + 0x1f6),*(ulong *)((int)param_1 + 0x114));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_9a02(ulong param_1,int param_2,ushort param_3,ushort param_4,int param_5)

{
  undefined4 uVar1;
  undefined *puVar2;
  ulong uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  ushort *puVar8;
  astruct_67 *paVar9;
  undefined2 uVar10;
  undefined uVar11;
  undefined uVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  int iVar15;
  undefined local_30 [0x12];
  int iStack30;
  undefined2 uStack26;
  undefined2 uStack22;
  undefined2 uStack20;
  long lStack18;
  ulong uStack10;
  ulong uStack6;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar6 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  uStack6 = CONCAT22(param_3,param_2);
  uVar3 = *(ulong *)(param_2 + 0x1f6);
  uStack10 = uVar3;
  pass1_1030_3694(uVar3,0x0,*(long *)(iVar6 + 0x110),(uchar *)param_3,0x1030,param_4);
  uVar4 = (uint)uVar3;
  *(uint *)(iVar6 + 0x114) = uVar4;
  *(uchar **)(iVar6 + 0x116) = (uchar *)param_3;
  pass1_1030_38b8();
  if ((param_3 | uVar4) == 0x0) {
    lStack18 = *(long *)((int)uStack6 + 0x200);
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,(uchar *)0x0,param_5);
    uStack20 = (undefined2)((ulong)puVar8 >> 0x10);
    uStack22 = SUB42(puVar8,0x0);
    if (lStack18 == 0x8000002) {
      iVar6 = 0x1f;
    }
    else {
      iVar6 = 0xb;
    }
    pass1_1010_043a((ulong)puVar8,*(long *)((int)uStack6 + 0x4),iVar6,param_4);
    if (lStack18 == 0x8000001) {
      uVar7 = 0x2;
    }
    else {
      uVar7 = 0x1;
    }
    uVar4 = 0x800;
    lStack18 = CONCAT22(0x800,uVar7);
    pass1_1038_349e(uStack6,CONCAT22(0x800,uVar7));
    iStack30 = 0x0;
    uStack26 = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,local_30)),0x1,0x0,0x400);
    while( true ) {
      puVar2 = local_30;
      pass1_1028_e4ec(CONCAT22(param_4,puVar2));
      uStack6 = CONCAT22(uVar4,puVar2);
      uVar5 = uVar4 | (uint)puVar2;
      if (uVar5 == 0x0) break;
      if (*(long *)(puVar2 + 0x200) == 0x8000002) {
        uStack26 = 0x1;
        uVar4 = uVar5;
      }
      else {
        iStack30 = 0x1;
        uVar4 = uVar5;
      }
    }
    if (iStack30 == 0x0) {
      uVar14 = 0x0;
      iVar15 = 0x3c;
      uVar11 = 0x1;
      uVar12 = 0x0;
      uVar13 = 0x0;
      uVar10 = 0x0;
      iVar6 = 0x0;
      uVar7 = 0x0;
      paVar9 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,(uchar *)0x0,param_5);
      post_win_msg_1008_a0e4
                (paVar9,CONCAT22(uVar10,uVar7),iVar6,CONCAT11(uVar12,uVar11),CONCAT22(uVar14,uVar13),iVar15,0x1008,
                 param_4);
    }
  }
  return;
}



void __stdcall16far pass1_1028_9b48(ulong param_1,astruct_330 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_331 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x118,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  iVar5 = (astruct_331 *)param_1;
  uVar6 = (undefined2)(param_1 >> 0x10);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    param_2->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_2->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    param_2->field_0x108 = iVar5->field_0x108;
    param_2->field_0x10c = iVar5->field_0x10c;
    param_2->field_0x110 = iVar5->field_0x110;
    param_2->field_0x114 = iVar5->field_0x114;
    *puStack10 = 0x9c52;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  iVar5->field_0x114 = 0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1028_9c2c(astruct_18 *param_1,byte param_2)

{
  pass1_1028_9992(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_9c62(int param_1,ushort param_2,ushort param_3,ushort param_4,uchar param_5)

{
  struct_op_1028_d1dc(param_4,param_5,(astruct_100 *)CONCAT22(param_2,param_1),param_3);
  *(ushort *)(param_1 + 0x108) = param_3;
  *(ushort *)CONCAT22(param_2,param_1) = 0x9eb6;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far pass1_1028_9c90(ulong param_1)

{
  uint uVar1;
  ushort uVar2;
  
  uVar1 = *(int *)((int)param_1 + 0x108) - 0x3e8;
  if ((uVar1 < 0x3a99) && (uVar1 % 0x3e8 == 0x0)) {
                    // WARNING: Could not recover jumptable at 0x10289dc0. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar2 = (**(code **)((uVar1 / 0x3e8) * 0x2 + -0x623a))();
    return uVar2;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9ca0(int param_1,ushort param_2,uchar param_3)

{
  pass1_1028_acb6((astruct_100 *)CONCAT22(param_2,param_1 + -0x108),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x108));
  *(undefined2 *)(param_1 + -0x108) = 0x389a;
  *(undefined2 *)(param_1 + -0x106) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9cca(int param_1,ushort param_2)

{
  uchar in_AF;
  
  pass1_1038_28d8((astruct_100 *)CONCAT22(param_2,param_1 + -0x108),param_2,in_AF);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x108));
  *(undefined2 *)(param_1 + -0x108) = 0x389a;
  *(undefined2 *)(param_1 + -0x106) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9cd8(ushort param_1,ushort param_2,uchar param_3)

{
  pass1_1028_a866((astruct_100 *)CONCAT22(param_2,param_1 - 0x108),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 - 0x108));
  *(undefined2 *)(param_1 - 0x108) = 0x389a;
  *(undefined2 *)(param_1 - 0x106) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9ce6(int param_1,ushort param_2,uchar param_3)

{
  pass1_1028_6e60((astruct_100 *)CONCAT22(param_2,param_1 + -0x108),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x108));
  *(undefined2 *)(param_1 + -0x108) = 0x389a;
  *(undefined2 *)(param_1 + -0x106) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9cf4(int param_1,ushort param_2,uchar param_3)

{
  pass1_1028_ab32((astruct_100 *)CONCAT22(param_2,param_1 + -0x108),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x108));
  *(undefined2 *)(param_1 + -0x108) = 0x389a;
  *(undefined2 *)(param_1 + -0x106) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d02(int param_1,ushort param_2,uchar param_3)

{
  pass1_1030_e09e((astruct_100 *)CONCAT22(param_2,param_1 + -0x108),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x108));
  *(undefined2 *)(param_1 + -0x108) = 0x389a;
  *(undefined2 *)(param_1 + -0x106) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d10(int param_1,int param_2,ushort param_3,uchar param_4)

{
  pass1_1038_0ba6((astruct_100 *)CONCAT22(param_3,param_1 + -0x220),param_2,param_3,param_4);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,param_1 + -0x220));
  *(undefined2 *)(param_1 + -0x220) = 0x389a;
  *(undefined2 *)(param_1 + -0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d3a(ushort param_1,ushort param_2)

{
  uchar in_AF;
  
  pass1_1028_9ec6((astruct_100 *)CONCAT22(param_2,param_1 - 0x220),param_2,in_AF);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 - 0x220));
  *(undefined2 *)(param_1 - 0x220) = 0x389a;
  *(undefined2 *)(param_1 - 0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d48(int param_1,ushort param_2,uchar param_3)

{
  pass1_1030_eb50((astruct_100 *)CONCAT22(param_2,param_1 + -0x220),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x220));
  *(undefined2 *)(param_1 + -0x220) = 0x389a;
  *(undefined2 *)(param_1 + -0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d56(int param_1,ushort param_2)

{
  uchar in_AF;
  
  pass1_1028_81aa((astruct_100 *)CONCAT22(param_2,param_1 + -0x220),param_2,in_AF);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x220));
  *(undefined2 *)(param_1 + -0x220) = 0x389a;
  *(undefined2 *)(param_1 + -0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d64(ushort param_1,ushort param_2,uchar param_3)

{
  pass1_1028_a9be((astruct_100 *)CONCAT22(param_2,param_1 - 0x220),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 - 0x220));
  *(undefined2 *)(param_1 - 0x220) = 0x389a;
  *(undefined2 *)(param_1 - 0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d72(ushort param_1,ushort param_2)

{
  pass1_1028_74ae(param_1 - 0x220,param_2);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 - 0x220));
  *(undefined2 *)(param_1 - 0x220) = 0x389a;
  *(undefined2 *)(param_1 - 0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far pass1_1028_9d80(int param_1,ushort param_2,uchar param_3)

{
  pass1_1030_ecc2((astruct_100 *)CONCAT22(param_2,param_1 + -0x220),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x220));
  *(undefined2 *)(param_1 + -0x220) = 0x389a;
  *(undefined2 *)(param_1 + -0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d8e(int param_1,ushort param_2,uchar param_3)

{
  pass1_1028_a706((astruct_100 *)CONCAT22(param_2,param_1 + -0x220),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x220));
  *(undefined2 *)(param_1 + -0x220) = 0x389a;
  *(undefined2 *)(param_1 + -0x21e) = 0x1008;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_9d9c(int param_1,ushort param_2,uchar param_3)

{
  pass1_1028_6fc0((astruct_100 *)CONCAT22(param_2,param_1 + -0x220),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,param_1 + -0x220));
  *(undefined2 *)(param_1 + -0x220) = 0x389a;
  *(undefined2 *)(param_1 + -0x21e) = 0x1008;
  return 0x1;
}



void __stdcall16far pass1_1028_9dee(ulong param_1,astruct_332 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_333 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10a,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_333 *)param_1;
    param_2->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_2->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    param_2->field_0x108 = iVar5->field_0x108;
    *puStack10 = 0x9eb6;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_9e8a(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_9ec6(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,(int)s_noth_bmp_1050_2321 + 0x6);
  param_1->field_0x0 = 0xa6f6;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),(char *)0x105050f0);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_9efc(ulong param_1,ushort *param_2,uint param_3,int param_4,ushort param_5,uchar param_6)

{
  long lVar1;
  ushort *puVar2;
  uint uVar3;
  int iVar4;
  long lVar5;
  uchar *puVar6;
  undefined2 in_register_0000000a;
  ulong uVar7;
  astruct_67 *paVar8;
  ushort *puVar9;
  ushort uVar10;
  ushort local_18;
  undefined2 uStack22;
  ushort *puStack6;
  uchar *puStack4;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
  uVar7 = CONCAT22(in_register_0000000a,param_3 | (uint)param_2);
  if ((param_3 | (uint)param_2) != 0x0) {
    puStack6 = param_2;
    puStack4 = (uchar *)param_3;
    pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_5 >> 0x8),CONCAT12((char)param_5,&local_18)),0x1,0x0,0x400);
    while( true ) {
      puVar2 = &local_18;
      pass1_1028_e4ec(CONCAT22(param_5,puVar2));
      puStack4 = (uchar *)(uint)uVar7;
      puStack6 = puVar2;
      if (((uint)puStack4 | (uint)puVar2) == 0x0) break;
      lVar1 = *(long *)(puVar2 + 0x100);
      uVar3 = puVar2[0x101];
      uVar7 = uVar7 & 0xffff0000 | (ulong)uVar3;
      if (puVar2[0xff] != 0x0) {
        uVar10 = (ushort)(param_1 >> 0x10);
        lVar5 = lVar1;
        if (((int)lVar1 != 0x2) || (uVar3 != 0x800)) {
          pass1_1028_a3ae((ushort)param_1,uVar10,CONCAT22(puStack4,puVar2),uVar7,param_4,param_5,param_6,(int)lVar1);
        }
        uVar3 = (uint)lVar5;
        pass1_1028_a28a((ushort)param_1,uVar10,CONCAT22(puStack4,puStack6));
        if (((int)uVar7 < 0x1) && (((int)uVar7 < 0x0 || (uVar3 < 0x64)))) {
          pass1_1028_a4ee(param_1,CONCAT22(puStack4,puStack6),param_4,param_5);
        }
        if (lVar1 != 0x8000002) {
          pass1_1038_42cc(CONCAT22(puStack4,puStack6),param_5);
          puVar6 = (uchar *)((uint)uVar7 | uVar3);
          uVar7 = uVar7 & 0xffff0000 | ZEXT24(puVar6);
          if (puVar6 != (uchar *)0x0) {
            paVar8 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_5,puVar6,param_4);
            uVar7 = uVar7 & 0xffff0000 | (ulong)paVar8 >> 0x10;
            post_win_msg_1008_a0e4(paVar8,0x0,uVar3,puStack6[0x104],*(ulong *)(puStack6 + 0x2),0x2,0x1008,param_5);
          }
        }
      }
    }
    local_18 = 0x389a;
    uStack22 = 0x1008;
    puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,(uchar *)0x0,param_4);
    puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
    iVar4 = (int)puVar9;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
    puStack6 = (ushort *)iVar4;
    puStack4 = puVar6;
    pass1_1010_9f72((ulong)puVar9,0x3e,param_5);
    if (iVar4 != 0x0) {
      iVar4 = pass1_1010_96d0((ulong)puVar9);
      if (iVar4 < 0x1) {
        if (iVar4 < 0x0) {
          iVar4 = (int)*(undefined4 *)((int)puStack6 + 0x1f6);
          pass1_1030_38b8();
          if (((int)puVar6 < 0x1) && (((int)puVar6 < 0x0 || (iVar4 == 0x0)))) {
            paVar8 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_5,puVar6,param_4);
            post_win_msg_1008_a0e4(paVar8,0x0,0x0,0x1,*(ulong *)((int)puStack6 + 0x4),0x6,0x1008,param_5);
          }
        }
      }
      else {
        puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_5,puVar6,param_4);
        puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
        post_win_msg_1008_a0e4
                  ((astruct_67 *)((ulong)puVar9 & 0xffff | ZEXT24(puVar6) << 0x10),0x0,iVar4,
                   *(ushort *)((int)puStack6 + 0x208),0x4000001,0x2,0x1008,param_5);
        puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,puVar6,param_4);
        pass1_1010_043a((ulong)puVar9,*(long *)((int)puStack6 + 0x4),0x14,param_5);
      }
    }
  }
  return;
}



void __stdcall16far pass1_1028_a0fa(ulong param_1,astruct_334 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    param_2->field_0x4 = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = &param_2->field_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0xa6f6;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_a188(ushort param_1,ushort param_2,int param_3,int param_4,ulong param_5,ushort param_6)

{
  ulong uVar1;
  long lVar2;
  ulong uVar3;
  undefined2 uVar4;
  uint uVar5;
  uint uVar6;
  ulong uVar7;
  long lVar8;
  long lVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  uchar *puVar13;
  undefined2 uVar14;
  ushort *puVar15;
  uint uStack18;
  uint uStack16;
  uint uStack14;
  int iStack12;
  
  uVar14 = (undefined2)(param_5 >> 0x10);
  iVar11 = (int)param_5;
  uVar1 = *(ulong *)(iVar11 + 0x1f6);
  uVar6 = *(uint *)(iVar11 + 0x1f8);
  uVar5 = (int)uVar1 + 0x18c;
  uVar4 = (undefined2)(uVar1 >> 0x10);
  uVar7 = (ulong)uVar5;
  pass1_1030_38f2(uVar1 & 0xffff | (ulong)uVar6 << 0x10,param_4,param_6);
  uVar3 = 0x64 / (long)param_3;
  uVar10 = (int)uVar3 >> 0xf;
  iVar12 = param_4 * 0x4;
  lVar2 = (uVar7 & 0xffff | (ulong)uVar6 << 0x10) + *(long *)(iVar12 + uVar5);
  lVar8 = lVar2 / (long)(uVar3 & 0xffff | (ulong)uVar10 << 0x10);
  lVar9 = lVar8 * (uVar3 & 0xffff | (ulong)uVar10 << 0x10);
  uStack14 = (uint)lVar2;
  iStack12 = (int)((ulong)lVar2 >> 0x10);
  uVar6 = (uint)lVar9;
  puVar13 = (uchar *)((iStack12 - (int)((ulong)lVar9 >> 0x10)) - (uint)(uStack14 < uVar6));
  *(int *)(uVar5 + iVar12) = uStack14 - uVar6;
  *(uchar **)(uVar5 + iVar12 + 0x2) = puVar13;
  uStack16 = (uint)((ulong)lVar8 >> 0x10);
  uStack18 = (uint)lVar8;
  if ((uStack16 | uStack18) != 0x0) {
    pass1_1030_375a(uVar1,param_4,lVar8,param_6);
    if (*(long *)(iVar11 + 0x200) != 0x8000002) {
      puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_6,puVar13,iVar12);
      puVar13 = (uchar *)((ulong)puVar15 >> 0x10);
      post_win_msg_1008_a0e4
                ((astruct_67 *)((ulong)puVar15 & 0xffff | ZEXT24(puVar13) << 0x10),0x0,uStack18,
                 *(ushort *)(iVar11 + 0x208),*(ulong *)(iVar11 + 0x4),0x2,0x1008,param_6);
      puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,puVar13,iVar12);
      pass1_1010_043a((ulong)puVar15,*(long *)(iVar11 + 0x4),0xd,param_6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_a28a(ushort param_1,ushort param_2,ulong param_3)

{
  code **ppcVar1;
  uint uVar2;
  undefined2 uVar3;
  ulong uVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  uint uVar8;
  astruct_691 *iVar9;
  undefined2 uVar9;
  ulong *puVar10;
  undefined4 *puStack10;
  
  puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0xe);
  puVar5 = (uchar *)((ulong)puVar10 >> 0x10);
  uVar2 = (uint)puVar10;
  pass1_1038_4d6e(param_3,puVar10,uVar2,puVar5);
  puStack10 = (undefined4 *)CONCAT22(puVar5,uVar2);
  uVar9 = (undefined2)(param_3 >> 0x10);
  iVar9 = (astruct_691 *)param_3;
  uVar4 = iVar9->field_0x1f6;
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  puVar6 = puVar5;
  (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar2,puVar5);
  uVar3 = (undefined2)uVar4;
  puVar7 = puVar6;
  pass1_1030_38b8();
  if ((uVar4 & 0xffff | ZEXT24(puVar6) << 0x10) == 0x0) {
    uVar4 = 0x64;
    uVar8 = 0x0;
  }
  else {
    uVar4 = CONCAT22(puVar7,uVar3) / (long)(uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
    uVar8 = (uint)(uVar4 >> 0x10);
  }
  uVar4 = uVar4 & 0xffff | (ulong)uVar8 << 0x10;
  if (puStack10 != (undefined4 *)0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x1030,uVar2,(char)puVar5,0x1);
  }
  if ((long)uVar4 < 0x64) {
    if ((long)uVar4 < 0x55) {
      if ((long)uVar4 < 0x4b) {
        if ((long)uVar4 < 0x32) {
          if ((long)uVar4 < 0x19) {
            iVar9->field_0x20a = 0x1;
            iVar9->field_0x20c = 0xffff;
            return;
          }
          iVar9->field_0x20a = 0x0;
          iVar9->field_0x20c = 0x0;
          return;
        }
        iVar9->field_0x20a = 0xfffb;
      }
      else {
        iVar9->field_0x20a = 0xfff6;
      }
    }
    else {
      iVar9->field_0x20a = 0xfff1;
    }
  }
  else {
    iVar9->field_0x20a = 0xffec;
  }
  iVar9->field_0x20c = 0x1;
  return;
}
