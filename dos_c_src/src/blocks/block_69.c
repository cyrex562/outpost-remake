
astruct_100 * __stdcall16far pass1_1028_837e(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0xf9f);
  param_1->field_0x0 = 0x84ba;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCFillResources_1050_500c);
  return param_1;
}



ushort __stdcall16far pass1_1028_83b4(uint param_1,ushort param_2)

{
  undefined *puVar1;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  while( true ) {
    puVar1 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar1));
    if ((param_1 | (uint)puVar1) == 0x0) break;
    *(undefined2 *)(puVar1 + 0x206) = 0x1;
    param_1 = param_1 | (uint)puVar1;
  }
  return 0x1;
}



void __stdcall16far pass1_1028_8400(ulong param_1,uint param_2,uchar *param_3)

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
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *puStack10 = 0x84ba;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_848e(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
pass1_1028_84ca(astruct_100 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,
               uchar param_7)

{
  ushort uVar1;
  int iVar2;
  uchar *puVar3;
  
  struct_op_1028_d1dc(param_6,param_7,param_1,0x3e7);
  puVar3 = (uchar *)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *(ushort *)(iVar2 + 0x108) = param_5;
  *(ushort *)(iVar2 + 0x10a) = param_4;
  *(ushort *)(iVar2 + 0x10c) = param_3;
  *(ulong *)(iVar2 + 0x10e) = param_2;
  param_1->field_0x0 = 0x8688;
  *(undefined2 *)(iVar2 + 0x2) = (int)&USHORT_1050_1028;
  if (*(int *)(iVar2 + 0x108) == 0x1) {
    uVar1 = (ushort)s_max_1050_501c;
  }
  else {
    uVar1 = (ushort)s_min_1050_5020;
  }
  sys_1000_3f9c((uchar *)(iVar2 + 0x8),puVar3,(ushort)s_SCForceMorale__s_for_colony__08l_1050_5024,
                (ushort)&USHORT_1050_1050,uVar1,&stack0xfffe,puVar3,0x1000,param_6,param_7);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1028_853e(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  ushort uVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(int *)(iVar3 + 0x108) == 0x0) {
    return 0x0;
  }
  uVar1 = *(undefined4 *)(iVar3 + 0x10e);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  if (*(int *)(iVar3 + 0x108) == 0x1) {
    uVar2 = 0x3e8;
  }
  else {
    uVar2 = 0x0;
  }
  pass1_1038_4d0e(CONCAT22(param_3,param_2),uVar2);
  return 0x1;
}



void __stdcall16far pass1_1028_858c(ulong param_1,astruct_318 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_319 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x112,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_319 *)param_1;
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
    param_2->field_0x10a = iVar5->field_0x10a;
    param_2->field_0x10c = iVar5->field_0x10c;
    param_2->field_0x10e = iVar5->field_0x10e;
    *puStack10 = 0x8688;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_865c(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far
pass1_1028_8698(astruct_100 *param_1,ulong param_2,ulong param_3,uchar param_4,ushort param_5)

{
  pass1_1028_6af2(param_1,param_2,param_3,param_5,param_4);
  param_1->field_0x0 = 0x87e0;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_86c2(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  astruct_67 *paVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  int iVar4;
  ushort uVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  int iVar8;
  
  uVar7 = 0x0;
  iVar8 = 0x1d;
  uVar5 = 0x1;
  uVar6 = 0x0;
  uVar3 = 0x0;
  iVar4 = 0x0;
  uVar2 = 0x0;
  paVar1 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,param_2,param_3);
  post_win_msg_1008_a0e4(paVar1,CONCAT22(uVar3,uVar2),iVar4,uVar5,CONCAT22(uVar7,uVar6),iVar8,0x1008,param_4);
  pass1_1028_6b2c(param_1,param_4);
  return;
}



void __stdcall16far pass1_1028_86f4(ulong param_1,astruct_320 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_321 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x110,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_321 *)param_1;
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
    *puStack10 = 0x6e50;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0x87e0;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_87b4(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
struct_op_1028_87f0(ushort param_1,uchar param_2,astruct_97 *param_3,ushort param_4,ushort param_5,ushort param_6,
                   ulong *param_7,ushort param_8,ulong param_9,ulong param_10)

{
  astruct_97 *iVar1;
  astruct_97 *puVar1;
  
  struct_op_1028_d1dc(param_1,param_2,(astruct_100 *)param_3,0x3e8);
  puVar1 = (astruct_97 *)((ulong)param_3 >> 0x10);
  iVar1 = (astruct_97 *)param_3;
  iVar1->field_0x108 = param_10;
  iVar1->field_0x10c = param_9;
  iVar1->field_0x110 = 0x0;
  iVar1->field_0x114 = *param_7;
  iVar1->field_0x118 = *(undefined2 *)(param_7 + 0x1);
  iVar1->field_0x11a = param_6;
  iVar1->field_0x11c = param_5;
  iVar1->field_0x11e = param_4;
  iVar1->field_0x122 = 0x0;
  iVar1->field_0x120 = 0x0;
  *(undefined2 *)param_3 = 0x8d8e;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  sys_1000_3f9c(&iVar1->field_0x8,(uchar *)puVar1,(ushort)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046,
                (ushort)&USHORT_1050_1050,(ushort)param_10,&stack0xfffe,puVar1,0x1000,param_1,param_2);
  return;
}



void __stdcall16far
struct_op_1028_8888(ushort param_1,uchar param_2,astruct_100 *param_3,ushort param_4,ushort param_5,ulong *param_6,
                   ushort param_7,ulong param_8,ulong param_9,ulong param_10)

{
  astruct_100 *iVar1;
  uchar *puVar1;
  
  struct_op_1028_d1dc(param_1,param_2,param_3,0x3e8);
  puVar1 = (uchar *)((ulong)param_3 >> 0x10);
  iVar1 = (astruct_100 *)param_3;
  iVar1->field_0x108 = param_10;
  iVar1->field_0x10c = param_9;
  iVar1->field_0x110 = param_8;
  iVar1->field_0x114 = *param_6;
  iVar1->field_0x118 = *(ushort *)(param_6 + 0x1);
  iVar1->field_0x11a = param_5;
  iVar1->field_0x11c = 0x0;
  iVar1->field_0x11e = param_4;
  iVar1->field_0x122 = 0x0;
  iVar1->field_0x120 = 0x0;
  param_3->field_0x0 = 0x8d8e;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  sys_1000_3f9c(&iVar1->field_0x8,puVar1,(ushort)s_SCInternalPutBldg2_site_0x_08lx__1050_506f,(ushort)&USHORT_1050_1050,
                (ushort)param_10,&stack0xfffe,puVar1,0x1000,param_1,param_2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_8920(ulong param_1,ushort param_2,ushort param_3,uchar param_4)

{
  ushort **ppuVar1;
  undefined4 uVar2;
  code **ppcVar3;
  ushort **ppuVar4;
  int iVar5;
  BOOL16 BVar6;
  ulong uVar7;
  uchar *puVar8;
  uchar *extraout_DX;
  uint uVar9;
  uint uVar10;
  int iVar11;
  astruct_684 *iVar12;
  int iVar13;
  undefined2 uVar14;
  undefined uVar15;
  undefined2 uVar16;
  ulong *local_156 [0x43];
  undefined4 local_4a;
  int iStack70;
  ulong uStack68;
  undefined4 uStack56;
  undefined4 *puStack52;
  undefined2 uStack48;
  ushort *puStack46;
  undefined4 uStack42;
  undefined local_26 [0x4];
  undefined4 uStack34;
  ulong uStack30;
  ulong uStack26;
  undefined4 uStack22;
  ushort *puStack18;
  uint uStack14;
  undefined local_c [0x2];
  undefined local_a [0x2];
  undefined local_8 [0x2];
  undefined4 uStack6;
  
  iVar13 = (int)(param_1 >> 0x10);
  iVar11 = (int)param_1;
  ppuVar1 = (ushort **)(iVar11 + 0x114);
  ppuVar4 = ppuVar1;
  pass1_1030_64ce(param_3,ppuVar1,param_2,_PTR_LOOP_1050_5740,(ushort *)(param_1 & 0xffff0000 | ZEXT24(ppuVar1)),
                  *(long *)(iVar11 + 0x108),(ulong *)CONCAT22(param_3,local_26));
  uStack6 = *ppuVar4;
  uVar15 = (undefined)(param_3 >> 0x8);
  pass1_1008_3eb4((ushort *)(param_1 & 0xffff0000 | ZEXT24(ppuVar1)),(ushort *)CONCAT22(param_3,local_c),
                  (ushort *)CONCAT13(uVar15,CONCAT12((char)param_3,local_a)),(ushort *)CONCAT22(param_3,local_8));
  puStack46 = uStack6;
  uStack56 = uStack6;
  uStack56._3_1_ = (char)((ulong)uStack6 >> 0x18);
  uStack14 = (uint)(uStack56._3_1_ != '\0');
  if (uStack14 == 0x0) {
    uVar7 = (ulong)(iVar11 + 0x114U);
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    puStack18 = (ushort *)(uVar7 & 0xffff | ZEXT24(uStack6._2_2_) << 0x10);
    uVar14 = 0x1030;
    pass1_1030_61fe(_PTR_LOOP_1050_5740,uVar7 & 0xffff | ZEXT24(uStack6._2_2_) << 0x10,
                    param_1 & 0xff000000 | (ulong)CONCAT12((char)(param_1 >> 0x10),iVar11 + 0x114U),
                    *(long *)(iVar11 + 0x108),(ushort)uVar7,(ushort)uStack6._2_2_,param_3);
    uStack56 = (ushort *)0x0;
    if ((*(int *)(iVar11 + 0x11a) == 0xa) || (*(int *)(iVar11 + 0x11a) == 0x37)) {
      if (*(int *)(iVar11 + 0x11a) == 0x37) {
        uStack56 = *(ushort **)(iVar11 + 0x10c);
      }
      iVar5 = iVar11 + 0x114;
      pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x400);
      *(int *)(iVar11 + 0x10c) = iVar5;
      *(uchar **)(iVar11 + 0x10e) = uStack6._2_2_;
      puStack46 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,uStack6._2_2_,iVar13);
      uStack6._2_2_ = (uchar *)((ulong)puStack46 >> 0x10);
      uVar7 = (ulong)puStack46 & 0xffff;
      uVar14 = 0x1018;
      pass1_1018_0196(uVar7 | ZEXT24(uStack6._2_2_) << 0x10,*(ulong *)(iVar11 + 0x10c),*(ulong *)(iVar11 + 0x108),
                      (ushort)uVar7,uStack6._2_2_,param_3);
      iVar5 = (int)uVar7;
      if (*(long *)(iVar11 + 0x110) != 0x0) {
        uVar2 = *(undefined4 *)(iVar11 + 0x10c);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
        uStack42 = CONCAT22(uStack6._2_2_,iVar5);
        uVar7 = *(ulong *)(iVar11 + 0x110);
        *(ulong *)(iVar5 + 0x200) = uVar7;
        uStack68 = uVar7;
      }
    }
    uStack6._0_2_ = (uint)uVar7;
    uVar2 = *(undefined4 *)(iVar11 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
    puStack52 = (undefined4 *)CONCAT22(uStack6._2_2_,(uint)uStack6);
    puVar8 = (uchar *)((uint)uStack6._2_2_ | (uint)uStack6);
    if (puVar8 != (uchar *)0x0) {
      ppcVar3 = (code **)((int)*puStack52 + 0x8);
      (**ppcVar3)(uVar14,(uint)uStack6,uStack6._2_2_,0x0,(char)puStack18,(int)((ulong)puStack18 >> 0x10),0x0);
      puVar8 = extraout_DX;
    }
  }
  else {
    puStack18 = uStack6;
    puVar8 = uStack6._2_2_;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puStack18,(uint)((ulong)puStack18 >> 0x10));
  uStack22 = CONCAT22(puVar8,(uint)uStack6);
  pass1_1030_73ee(CONCAT13((char)((uint)puVar8 >> 0x8),CONCAT12((char)puVar8,(uint)uStack6)),*(ulong *)(iVar11 + 0x10c),
                  (ushort)puVar8);
  BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(iVar11 + 0x11a),0x31);
  if ((BVar6 == 0x0) && (*(int *)(iVar11 + 0x11c) == 0x0)) {
    local_4a = *(undefined4 *)((int)uStack22 + 0xc);
    iStack70 = *(int *)((int)uStack22 + 0x10);
    uStack68 = uStack68 & 0xffff0000 | ZEXT24(&local_4a);
    if (iStack70 < 0x1) {
      uStack48 = 0x5;
    }
    else {
      uStack48 = 0x6;
    }
    *(undefined2 *)((int)uStack22 + 0x14) = uStack48;
  }
  uStack26 = *(ulong *)((int)uStack22 + 0x16);
  uVar9 = *(uint *)((int)uStack22 + 0x18);
  if ((uVar9 | (uint)uStack26) != 0x0) {
    struct_1030_e4fa((astruct_100 *)CONCAT13(uVar15,CONCAT12((char)param_3,local_156)),
                     uStack26 & 0xffff | (ulong)uVar9 << 0x10,param_3,param_4);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,local_156));
    local_156[0] = &ULONG_1008_389a;
  }
  uStack30 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,uVar9,0x7);
  uVar9 = (uint)uStack30;
  uVar10 = (uint)(uStack30 >> 0x10) | uVar9;
  if (uVar10 == 0x0) {
    return;
  }
  pass1_1030_7e5a(uStack22,uStack30,uVar10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack30,(uint)(uStack30 >> 0x10));
  uStack34 = (undefined4 *)CONCAT22(uVar10,uVar9);
  uVar14 = SUB42(puStack18,0x0);
  uVar16 = (undefined2)((ulong)puStack18 >> 0x10);
  uVar15 = (undefined)uVar10;
  iVar12 = (astruct_684 *)*uStack34;
  ppcVar3 = (code **)&iVar12->field_0x4;
  (**ppcVar3)();
  ppcVar3 = (code **)&iVar12->field_0x20;
  (**ppcVar3)(0x1030,uStack34,uVar9,uVar15,uVar14,uVar16);
  ppcVar3 = (code **)&iVar12->field_0x18;
  (**ppcVar3)(0x1030,(int)uStack34,(char)((ulong)uStack34 >> 0x10),0x1);
  if (*(int *)(iVar11 + 0x11a) == 0x37) {
    *(undefined4 *)((int)uStack34 + 0x20) = *(undefined4 *)(iVar11 + 0x10c);
  }
  *(undefined4 *)(iVar11 + 0x120) = uStack34;
  return;
}



void __stdcall16far pass1_1028_8c46(ulong param_1,astruct_322 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_323 *iVar5;
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
    iVar5 = (astruct_323 *)param_1;
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
    param_2->field_0x120 = iVar5->field_0x120;
    *puStack10 = 0x8d8e;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_8d62(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
pass1_1028_8d9e(astruct_100 *param_1,ulong param_2,ulong param_3,ulong param_4,ushort param_5,uchar param_6)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_op_1028_d1dc(param_5,param_6,param_1,0x3e8);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x108) = param_4;
  *(ulong *)(iVar1 + 0x10c) = param_3;
  *(ulong *)(iVar1 + 0x110) = param_2;
  *(undefined4 *)(iVar1 + 0x114) = 0x0;
  param_1->field_0x0 = 0x8fb0;
  *(undefined2 *)(iVar1 + 0x2) = (int)&USHORT_1050_1028;
  return;
}



void __stdcall16far pass1_1028_8dec(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x8fb0;
  *(undefined2 *)(iVar1 + 0x2) = (int)&USHORT_1050_1028;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x114),0x1000);
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_8e1e(ulong param_1,int param_2,ushort param_3)

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

void __stdcall16far pass1_1028_8e5c(ulong param_1,int param_2,uchar *param_3,ushort param_4)

{
  undefined4 uVar1;
  ulong uVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar3 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  uVar2 = *(ulong *)(param_2 + 0x1f6);
  pass1_1030_35a4(uVar2,*(long *)(iVar3 + 0x110),param_3,0x1030,param_4);
  *(undefined2 *)(iVar3 + 0x114) = (int)uVar2;
  *(uchar **)(iVar3 + 0x116) = param_3;
  return;
}



void __stdcall16far pass1_1028_8ea6(ulong param_1,astruct_324 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_325 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x118,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  iVar5 = (astruct_325 *)param_1;
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
    *puStack10 = 0x8fb0;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  iVar5->field_0x114 = 0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1028_8f8a(astruct_18 *param_1,byte param_2)

{
  pass1_1028_8dec(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far
pass1_1028_8fc0(astruct_100 *param_1,ulong param_2,ulong param_3,ushort param_4,uchar param_5)

{
  pass1_1028_6af2(param_1,param_2,param_3,param_4,param_5);
  param_1->field_0x0 = 0x90d6;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



void __stdcall16far pass1_1028_8fea(ulong param_1,astruct_326 *param_2,uchar *param_3)

{
  ulong *puVar2;
  int iVar3;
  astruct_327 *iVar5;
  ulong *puVar4;
  ulong *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  ulong *puVar1;
  
  mem_op_1000_179c(0x110,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_327 *)param_1;
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
    *puStack10 = 0x6e50;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    *puStack10 = 0x90d6;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_90aa(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_90e6(astruct_100 *param_1,ushort param_2,ushort param_3,uchar param_4)

{
  undefined2 uVar1;
  
  struct_op_1028_d1dc(param_3,param_4,param_1,0x1387);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x108) = param_2;
  param_1->field_0x0 = 0x932c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_9114(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  undefined2 uVar1;
  uchar *puVar2;
  uint uVar3;
  astruct_67 *paVar4;
  ushort *puVar5;
  ushort uVar6;
  uchar *puVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  uint uStack10;
  
  paVar4 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,param_2,param_3);
  uVar3 = (uint)param_1;
  iVar10 = *(int *)(uVar3 + 0x108);
  if (iVar10 - 0x1U < 0x8) {
    uStack10 = (uint)*_PTR_LOOP_1050_65e2;
    iVar8 = (int)((ulong)*_PTR_LOOP_1050_65e2 >> 0x10);
    switch(iVar10) {
    case 0x1:
      iVar10 = 0x16;
      break;
    case 0x2:
      iVar10 = 0x17;
      break;
    case 0x3:
      iVar10 = 0x18;
      break;
    case 0x4:
      iVar10 = 0x1b;
      break;
    case 0x5:
      iVar10 = 0x1f;
      break;
    case 0x6:
      iVar10 = 0x24;
      break;
    case 0x7:
      pass1_1008_612e(0x0,0x14,uVar3);
      puVar2 = (uchar *)(((int)uVar3 >> 0xf) + (uint)(0xff91 < uVar3));
      uVar6 = uStack10 + uVar3 + 0x6e;
      puVar7 = puVar2 + (uint)CARRY2(uStack10,uVar3 + 0x6e) + iVar8;
      iVar10 = 0x7;
      puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,puVar2,param_3);
      uVar1 = (undefined2)((ulong)puVar5 >> 0x10);
      uVar3 = (uint)puVar5;
      pass1_1010_ebf8((ulong)puVar5,uVar6,(ushort)puVar7,iVar10);
      pass1_1008_612e(0x1,0x64,uVar3);
      if (0x32 < (int)uVar3) {
        return;
      }
      pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,0x1,0x400);
      pass1_1038_4900(CONCAT22(uVar1,uVar3));
      iVar10 = 0x2c;
      break;
    case 0x8:
      pass1_1008_612e(0x0,0x14,uVar3);
      puVar2 = (uchar *)(((int)uVar3 >> 0xf) + (uint)(0xff9b < uVar3));
      uVar6 = uStack10 + uVar3 + 0x64;
      puVar7 = puVar2 + (uint)CARRY2(uStack10,uVar3 + 0x64) + iVar8;
      iVar8 = 0x8;
      puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,puVar2,param_3);
      uVar1 = (undefined2)((ulong)puVar5 >> 0x10);
      iVar10 = (int)puVar5;
      pass1_1010_ebf8((ulong)puVar5,uVar6,(ushort)puVar7,iVar8);
      if (0x19 < (int)uVar3) {
        return;
      }
      uVar3 = 0x1;
      uVar9 = 0x2;
      pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,0x1,0x400);
      pass1_1038_43cc(iVar10,uVar1,uVar3,uVar9,iVar10,uVar1);
      iVar10 = 0x2d;
    }
    post_win_msg_1008_a0e4(paVar4,0x0,0x0,0x1,0x0,iVar10,0x1008,param_4);
  }
  return;
}



void __stdcall16far pass1_1028_9264(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined2 uVar7;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10a,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)(iVar5 + 0x4);
    puVar3 = (undefined4 *)(iVar5 + 0x8);
    puVar6 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *(undefined2 *)(param_2 + 0x108) = *(undefined2 *)(iVar5 + 0x108);
    *puStack10 = 0x932c;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_9300(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
struct_op_1028_933c(astruct_100 *param_1,ushort param_2,ushort param_3,ushort param_4,ulong *param_5,ushort param_6,
                   ulong param_7,ulong param_8,ushort param_9,uchar param_10)

{
  int iVar1;
  uchar *puVar2;
  
  struct_op_1028_d1dc(param_9,param_10,param_1,0x3e8);
  puVar2 = (uchar *)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x108) = param_8;
  *(ulong *)(iVar1 + 0x10c) = param_7;
  *(undefined4 *)(iVar1 + 0x110) = 0x0;
  *(ulong *)(iVar1 + 0x114) = *param_5;
  *(undefined2 *)(iVar1 + 0x118) = *(undefined2 *)(param_5 + 0x1);
  *(ushort *)(iVar1 + 0x11a) = param_4;
  *(ushort *)(iVar1 + 0x11c) = param_2;
  *(undefined2 *)(iVar1 + 0x120) = 0x0;
  *(undefined2 *)(iVar1 + 0x11e) = 0x0;
  *(ushort *)(iVar1 + 0x122) = param_3;
  param_1->field_0x0 = 0x9934;
  *(undefined2 *)(iVar1 + 0x2) = (int)&USHORT_1050_1028;
  sys_1000_3f9c((uchar *)(iVar1 + 0x8),puVar2,(ushort)s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,
                (ushort)&USHORT_1050_1050,(ushort)param_8,&stack0xfffe,puVar2,0x1000,param_9,param_10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_93d4(ulong param_1,ushort param_2,int param_3,ushort param_4,uchar param_5)

{
  code **ppcVar1;
  ushort uVar2;
  uint uVar3;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  int iVar4;
  undefined2 uVar5;
  undefined local_112 [0x10c];
  ulong uStack6;
  
  PTR_LOOP_1050_50ca = (undefined *)0x0;
  PTR_LOOP_1050_50cc = (undefined *)0x0;
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uStack6 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,param_2,0x7);
  uVar3 = (uint)(uStack6 >> 0x10);
  uVar2 = (ushort)uStack6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,uVar3);
  *(ushort *)(iVar4 + 0x11e) = uVar2;
  *(uint *)(iVar4 + 0x120) = uVar3;
  uVar2 = iVar4 + 0x114;
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x11e) + 0x1c);
  (**ppcVar1)();
  if (uVar2 != 0x0) {
    pass1_1028_9624(param_1,uVar2,extraout_DX,param_4,param_3,param_5);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x11e) + 0x20);
    (**ppcVar1)();
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x11e) + 0x18);
    (**ppcVar1)();
    pass1_1028_9600(param_1,extraout_DX_00,param_3,param_4,param_5);
    return;
  }
  *(undefined4 *)(iVar4 + 0x11e) = 0x0;
  struct_1030_e4fa((astruct_100 *)CONCAT22(param_4,local_112),uStack6,param_4,param_5);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_112));
  if (PTR_LOOP_1050_50ca == (undefined *)0x0) {
    PTR_LOOP_1050_50ca = (undefined *)0x6ad;
  }
  return;
}
