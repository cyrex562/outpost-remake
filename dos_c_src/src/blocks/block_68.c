
astruct_18 * __stdcall16far pass1_1028_7472(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_74ae(astruct_100 *param_1)

{
  ushort unaff_SS;
  uchar in_AF;
  
  struct_op_1028_d1dc(unaff_SS,in_AF,param_1,0x1387);
  param_1->field_0x0 = 0x819a;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCEvent_1050_4ff4);
  return param_1;
}



ushort __stdcall16far pass1_1028_74e4(ulong param_1,long param_2,int param_3,ushort param_4,uchar param_5)

{
  int iVar1;
  
  pass1_1028_7fb6(param_1,param_3,param_4,param_5);
  pass1_1028_7c4e(param_1,(uchar *)param_2,param_3,param_4);
  pass1_1028_7dfc(param_1,(uchar *)param_2,param_3,param_4,param_5);
  iVar1 = post_msg_1028_76da(param_1);
  pass1_1028_767e(iVar1,(ushort)param_2,param_3,param_4);
  pass1_1028_75bc(param_4);
  pass1_1028_78b8(param_1,param_2,param_3,param_4,param_5);
  return 0x1;
}



void __stdcall16far pass1_1028_752e(ulong param_1,uint param_2,uchar *param_3)

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
    *puStack10 = 0x819a;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_75bc(ushort param_1)

{
  uint uVar1;
  undefined *puVar2;
  uint uVar3;
  ulong uVar4;
  ulong uStack28;
  undefined local_18 [0x8];
  undefined2 uStack16;
  uint uStack14;
  undefined2 uStack12;
  uint uStack10;
  int iStack8;
  ulong uStack6;
  
  uStack6 = *_PTR_LOOP_1050_65e2;
  uVar1 = (uint)((qword)uStack6 % 0x7b);
  uVar4 = (ulong)uVar1;
  if ((uVar1 == 0x0) && (0x95 < uStack6)) {
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_1,local_18),0x1,0x0,0x400);
    while( true ) {
      uVar1 = (uint)uVar4;
      puVar2 = local_18;
      pass1_1028_e4ec(CONCAT22(param_1,puVar2));
      uStack28 = CONCAT22(uVar1,puVar2);
      uVar4 = (ulong)(uVar1 | (uint)puVar2);
      if ((uVar1 | (uint)puVar2) == 0x0) break;
      pass1_1008_612e(0x1,0x64,(uint)puVar2);
      if ((int)puVar2 < 0x6) {
        pass1_1038_362e(uStack28);
      }
    }
    if (iStack8 != 0x0) {
      uStack12 = 0x1;
      uStack10 = 0x0;
    }
    uVar4 = (ulong)uStack10;
    uStack16 = uStack12;
    uStack14 = uStack10;
    while( true ) {
      uVar1 = (uint)uVar4;
      puVar2 = local_18;
      pass1_1028_e4ec(CONCAT22(param_1,puVar2));
      uVar3 = uVar1 | (uint)puVar2;
      uVar4 = (ulong)uVar3;
      if (uVar3 == 0x0) break;
      pass1_1038_3698(CONCAT22(uVar1,puVar2),(ushort)puVar2,uVar3,param_1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_767e(int param_1,ushort param_2,int param_3,ushort param_4)

{
  ushort *puVar1;
  
  pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,0x1,0x800);
  if ((*(int *)(param_1 + 0x152) != 0x0) && ((int)((qword)*_PTR_LOOP_1050_65e2 % 0x64) == 0x0)) {
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x40,param_4,(uchar *)0x0,param_3);
    load_str_and_sprintf_1008_b78a((ULONG)puVar1,param_4,(int)((ulong)puVar1 >> 0x10),(int)puVar1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far post_msg_1028_76da(void)

{
  long lVar1;
  undefined2 uVar2;
  uchar *in_DX;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar3;
  uint uStack10;
  uint uStack8;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2c,unaff_SS,in_DX,unaff_DI);
  uVar2 = (undefined2)((ulong)puVar3 >> 0x10);
  lVar1 = *(long *)((int)puVar3 + 0xc);
  uStack8 = (uint)((ulong)lVar1 >> 0x10);
  uStack10 = (uint)lVar1;
  if (((uStack8 | uStack10) != 0x0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
    PostMessage16(0x1010,0x0,0x0,0x1110106);
    *(undefined4 *)((int)puVar3 + 0xc) = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_7742(ushort param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  code **ppcVar1;
  uint uVar2;
  undefined *puVar3;
  undefined2 uVar4;
  uchar *puVar5;
  undefined2 extraout_DX;
  undefined2 uVar6;
  undefined2 extraout_DX_00;
  ulong *puVar7;
  ulong uVar8;
  undefined uVar9;
  undefined uVar10;
  uint uVar11;
  ulong uStack26;
  undefined local_16 [0x2];
  ulong uStack20;
  undefined2 uStack16;
  undefined4 *puStack14;
  uint uStack10;
  uchar *puStack8;
  undefined2 uStack6;
  undefined2 uStack4;
  
  puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x18);
  uVar4 = (undefined2)((ulong)puVar7 >> 0x10);
  uVar6 = SUB42(puVar7,0x0);
  uStack6 = uVar6;
  uStack4 = uVar4;
  uVar8 = pass1_1028_b4f2(param_4);
  puVar5 = (uchar *)(uVar8 >> 0x10);
  uVar2 = (uint)uVar8;
  uStack10 = uVar2;
  puStack8 = puVar5;
  pass1_1038_4d6e(uVar8,(ulong *)CONCAT22(uVar4,uVar6),uVar2,puVar5);
  puStack14 = (undefined4 *)CONCAT22(puVar5,uVar2);
  uStack16 = 0x0;
  ppcVar1 = (code **)((int)*puStack14 + 0x10);
  uVar11 = uVar2;
  (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar2,puVar5);
  uStack20 = CONCAT22(extraout_DX,uVar2);
  uVar8 = pass1_1030_bcae((ushort)local_16,param_5);
  uVar6 = (undefined2)(uVar8 >> 0x10);
  uStack26 = 0x0;
  do {
    if (uStack20 <= uStack26) {
LAB_1028_77e7:
      if (puStack14 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1030,(int)puStack14,(char)((ulong)puStack14 >> 0x10),0x1,uVar11,puVar5,puStack14,puStack14);
      }
      return;
    }
    uVar8 = uStack20;
    pass1_1030_1d58((ulong)puStack14);
    uVar4 = (undefined2)uVar8;
    uVar9 = (undefined)uVar8;
    uVar10 = (undefined)(uVar8 >> 0x8);
    pass1_1028_b58e(param_4);
    puVar3 = local_16;
    uVar8 = CONCAT22(uVar6,CONCAT11(uVar10,uVar9));
    uVar6 = extraout_DX_00;
    pass1_1030_bd74((ushort)puVar3,param_5,CONCAT22(extraout_DX_00,uVar4),uVar8,param_5);
    if ((int)puVar3 <= param_3) {
      uStack16 = 0x1;
      goto LAB_1028_77e7;
    }
    uStack26 = uStack26 + 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_780c(ushort param_1,ushort param_2,ulong param_3)

{
  code **ppcVar1;
  uint uVar2;
  ushort uVar3;
  ulong uVar4;
  uchar *puVar5;
  uint extraout_DX;
  uint extraout_DX_00;
  uint uVar6;
  undefined2 uVar7;
  ulong *puVar8;
  undefined4 *puVar9;
  ulong uStack18;
  ulong uStack14;
  undefined4 *puStack10;
  
  puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x25);
  puVar5 = (uchar *)((ulong)puVar8 >> 0x10);
  uVar2 = (uint)puVar8;
  uVar7 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4e78(uVar2,puVar5,param_3,puVar8);
  puStack10 = (undefined4 *)CONCAT22(puVar5,uVar2);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar6 = uVar2;
  (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar2,puVar5);
  uStack14 = CONCAT22(extraout_DX,uVar6);
  if ((extraout_DX | uVar6) == 0x0) {
    return;
  }
  for (uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1) {
    ppcVar1 = (code **)((int)*puStack10 + 0x4);
    uVar4 = uStack14;
    (**ppcVar1)();
    uVar3 = (ushort)uVar4;
    uVar6 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_00);
    uVar7 = 0x1030;
    puVar9 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar6,uVar3));
    ppcVar1 = (code **)((int)*puVar9 + 0x24);
    (**ppcVar1)();
  }
  if (puStack10 != (undefined4 *)0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(uVar7,uVar2,(char)puVar5,0x1);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_78b8(ulong param_1,long param_2,int param_3,ushort param_4,uchar param_5)

{
  ulong *puVar1;
  ulong **ppuVar2;
  undefined2 *puVar3;
  undefined2 *puVar4;
  ulong *puVar5;
  uchar *puVar6;
  uint uVar7;
  uchar *puVar8;
  int iVar9;
  ulong uVar10;
  bool bVar11;
  bool bVar12;
  ushort *puVar13;
  ushort uVar14;
  undefined2 uVar15;
  undefined2 uVar16;
  int iVar17;
  undefined2 uStack340;
  undefined2 uStack338;
  ushort *puStack74;
  uchar *puStack70;
  uchar *puStack68;
  ulong *local_42 [0x4];
  uchar *local_30;
  uchar *puStack46;
  uchar *local_1e [0x3];
  ulong local_18;
  uchar *puStack20;
  uchar *puStack18;
  ulong uStack16;
  undefined *puStack12;
  undefined2 uStack10;
  uchar *puStack8;
  ulong *puStack6;
  
  puVar6 = (uchar *)param_2;
  puVar5 = *_PTR_LOOP_1050_65e2;
  puStack6 = puVar5;
  if (puVar5 == (ulong *)0x98) {
    pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,0x2,0x400);
    puVar6 = (uchar *)param_2;
    uStack16 = (ulong)puVar5 & 0xffff | param_2 << 0x10;
    if (*(long *)((int)puVar5 + 0x200) == 0x8000002) {
      pass1_1020_a43e(param_4,puVar6,(ushort *)CONCAT22(param_4,&local_18));
      puVar13 = pass1_1008_3e38((ushort *)CONCAT22(param_4,local_1e));
      puVar6 = (uchar *)((ulong)puVar13 >> 0x10);
      puVar1 = &local_18;
      pass1_1020_a49a(param_4,param_5,puVar6,CONCAT22(param_4,puVar1),(int *)CONCAT22(param_4,local_1e),0x7a);
      pass1_1038_4f54(uStack16,0x1,(ushort)puVar1);
      if (puVar1 == (ulong *)0x0) {
        pass1_1020_a49a(param_4,param_5,puVar6,CONCAT22(param_4,&local_18),(int *)0x0,0x35);
      }
    }
  }
  if (((ulong *)0xe < puStack6) && (puStack6 < (ulong *)0x16)) {
    puVar13 = pass1_1020_a43e(param_4,puVar6,(ushort *)CONCAT22(param_4,local_1e));
    local_18 = (long)puStack6 - 0xf;
    pass1_1020_a54c(param_4,param_5,(uchar *)((ulong)puVar13 >> 0x10),(ushort)local_1e,param_4,(int)local_18);
  }
  uVar10 = (ulong)(ZEXT48(puStack6) % 0x7d);
  puVar8 = (uchar *)(ZEXT48(puStack6) % 0x7d);
  puVar6 = puVar8;
  if (uVar10 == 0x0) {
    local_1e[0] = puVar8;
    pass1_1008_612e(0x1,0x64,(uint)puVar8);
    puVar8 = (uchar *)uVar10;
    puVar6 = local_1e[0];
    if ((int)local_1e[0] < 0x1a) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,&local_30),0x1,0x0,0x400);
      do {
        uVar7 = (uint)uVar10;
        uVar10 = ZEXT24(&local_30);
        pass1_1028_e4ec(CONCAT22(param_4,&local_30));
        puVar6 = (uchar *)uVar10;
        local_18 = uVar10 & 0xffff | (ulong)uVar7 << 0x10;
        puVar8 = (uchar *)(uVar7 | (uint)puVar6);
        uVar10 = ZEXT24(puVar8);
        if (puVar8 == (uchar *)0x0) goto LAB_1028_79d6;
      } while (*(long *)(puVar6 + 0x200) == 0x8000002);
      pass1_1038_43cc((int)puVar6,uVar7,0x1,0x4,(int)puVar6,(int)puVar8);
LAB_1028_79d6:
      local_30 = (uchar *)0x389a;
      puStack46 = (uchar *)0x1008;
    }
  }
  if (puStack6 == (ulong *)0x5) {
    uVar16 = SUB42(&USHORT_1050_1050,0x0);
    uVar15 = SUB42(s_Rebel_1050_4ffc,0x0);
    pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,0x2,0x400);
    local_30 = puVar6;
    puStack46 = puVar8;
    pass1_1038_4d3c(CONCAT22(puVar8,puVar6),(char *)CONCAT22(uVar16,uVar15),(ushort)puVar8);
  }
  if (puStack6 == (ulong *)0x12c) {
    uVar16 = 0x400;
    iVar9 = 0xf;
    uVar15 = 0x1;
    puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,puVar8,param_3);
    puVar8 = (uchar *)((ulong)puVar13 >> 0x10);
    puVar6 = (uchar *)puVar13;
    local_30 = puVar6;
    puStack46 = puVar8;
    pass1_1010_043a((ulong)puVar13,CONCAT22(uVar16,uVar15),iVar9,param_4);
  }
  if (puStack6 == (ulong *)0x3d) {
    puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,puVar8,param_3);
    uVar10 = (ulong)puVar13 >> 0x10;
    local_30 = (uchar *)puVar13;
    puVar8 = (uchar *)((ulong)puVar13 >> 0x10);
    local_1e[0] = PTR_LOOP_1050_13ae;
    puVar6 = PTR_LOOP_1050_13ae;
    puStack46 = puVar8;
    if (PTR_LOOP_1050_13ae != (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_42),0x1,0x0,0x400);
      while( true ) {
        uVar7 = (uint)uVar10;
        ppuVar2 = local_42;
        pass1_1028_e4ec(CONCAT22(param_4,ppuVar2));
        local_18 = CONCAT22(uVar7,ppuVar2);
        uVar10 = (ulong)(uVar7 | (uint)ppuVar2);
        if ((uVar7 | (uint)ppuVar2) == 0x0) break;
        uStack16 = *(ulong *)((int)ppuVar2 + 0x1f6);
        pass1_1030_34da(uStack16);
      }
      uVar16 = 0x400;
      iVar9 = 0x10;
      uVar15 = 0x1;
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,(uchar *)0x0,param_3);
      puVar8 = (uchar *)((ulong)puVar13 >> 0x10);
      puVar6 = (uchar *)puVar13;
      puStack20 = puVar6;
      puStack18 = puVar8;
      pass1_1010_043a((ulong)puVar13,CONCAT22(uVar16,uVar15),iVar9,param_4);
      local_42[0] = &ULONG_1008_389a;
    }
  }
  if (puStack6 == (ulong *)0x96) {
    pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,0x1,0x400);
    puStack74 = (ushort *)CONCAT22(puVar8,puVar6);
    uVar14 = (ushort)(param_1 >> 0x10);
    pass1_1028_780c((ushort)param_1,uVar14,CONCAT22(puVar8,puVar6));
    if (puVar6 != (uchar *)0x0) {
      uVar16 = 0x400;
      iVar9 = 0x1d;
      uVar15 = 0x1;
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,puVar8,param_3);
      puVar8 = (uchar *)((ulong)puVar13 >> 0x10);
      puVar6 = (uchar *)puVar13;
      puStack70 = puVar6;
      puStack68 = puVar8;
      pass1_1010_043a((ulong)puVar13,CONCAT22(uVar16,uVar15),iVar9,param_4);
    }
    pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,0x2,0x400);
    puStack74 = (ushort *)CONCAT22(puVar8,puVar6);
    pass1_1028_780c((ushort)param_1,uVar14,CONCAT22(puVar8,puVar6));
  }
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,puVar8,param_3);
  puStack8 = (uchar *)((ulong)puVar13 >> 0x10);
  uStack10 = SUB42(puVar13,0x0);
  puStack12 = PTR_LOOP_1050_13ae;
  if (0x2 < (int)PTR_LOOP_1050_13ae) {
    puStack74 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,puStack8,param_3);
    for (puStack70 = (uchar *)0x1; (int)puStack70 < 0x9; puStack70 = (uchar *)((int)puStack70 + 0x1)) {
      local_42[0] = *(ulong **)((int)puStack74 + 0x34 + (int)puStack70 * 0x4);
      if (local_42[0] == puStack6) {
        puVar6 = (uchar *)((int)&PTR_LOOP_1050_0000 + 0x1);
        local_30 = (uchar *)0x1;
        pass1_1008_612e(0x1,0x64,0x1);
        puVar4 = (undefined2 *)((int)puStack70 - 0x7);
        if (puVar4 == (undefined2 *)0x0) {
          bVar12 = SBORROW2((int)puVar6,0x32);
          puVar8 = puVar6 + -0x32;
          bVar11 = puVar6 == (uchar *)((int)s_New_failed_in_Op__Op_1050_0020 + 0x12);
LAB_1028_7b74:
          if (!bVar11 && bVar12 == (int)puVar8 < 0x0) {
            local_30 = (uchar *)0x0;
          }
        }
        else {
          puVar4 = (undefined2 *)((int)puStack70 - 0x8);
          if (puVar4 == (undefined2 *)0x0) {
            bVar12 = SBORROW2((int)puVar6,0x19);
            puVar8 = puVar6 + -0x19;
            bVar11 = puVar8 == (uchar *)0x0;
            goto LAB_1028_7b74;
          }
        }
        local_1e[0] = puVar6;
        if (local_30 != (uchar *)0x0) {
          pass1_1028_90e6((astruct_100 *)CONCAT22(param_4,&uStack340),(ushort)puStack70,param_4,param_5);
          puVar4 = &uStack340;
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,puVar4));
          uStack340 = 0x389a;
          uStack338 = 0x1008;
        }
        pass1_1008_612e(0x0,0xa,(uint)puVar4);
        local_18 = local_18 & 0xffff0000 | ZEXT24(puVar4);
        if (puStack70 == (uchar *)0x7) {
          iVar17 = 0x7;
          puVar3 = puVar4 + 0x37;
          iVar9 = (int)puVar3 >> 0xf;
        }
        else {
          if (puStack70 != (uchar *)0x8) goto LAB_1028_7ba0;
          iVar17 = 0x8;
          puVar3 = puVar4 + 0x32;
          iVar9 = ((int)puVar4 >> 0xf) + (uint)((undefined2 *)0xff9b < puVar4);
        }
        uVar14 = (int)((ulong)local_42[0] >> 0x10) + iVar9 + (uint)CARRY2((uint)local_42[0],(uint)puVar3);
        local_42[0] = (ulong *)CONCAT22(uVar14,(uint)local_42[0] + (int)puVar3);
        pass1_1010_ebf8((ulong)puStack74,(uint)local_42[0] + (int)puVar3,uVar14,iVar17);
      }
LAB_1028_7ba0:
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_7c4e(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  code **ppcVar1;
  undefined *puVar2;
  int iVar3;
  uint uVar4;
  ulong uVar5;
  undefined uVar6;
  undefined in_AF;
  ushort *puVar7;
  ulong *puVar8;
  astruct_100 *paVar9;
  undefined uVar10;
  undefined2 local_156;
  undefined2 uStack340;
  uint uStack70;
  uint uStack68;
  int iStack66;
  ulong uStack64;
  ulong uStack56;
  uint uStack52;
  ulong uStack50;
  undefined4 *puStack46;
  uint uStack42;
  uchar *puStack40;
  ulong uStack38;
  undefined local_22 [0x12];
  int iStack16;
  int iStack14;
  undefined4 uStack12;
  undefined *puStack8;
  undefined2 uStack6;
  undefined2 uStack4;
  
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  uStack4 = (undefined2)((ulong)puVar7 >> 0x10);
  uStack6 = SUB42(puVar7,0x0);
  puStack8 = PTR_LOOP_1050_13ae;
  if (0x2 < (int)PTR_LOOP_1050_13ae) {
    uStack12 = *_PTR_LOOP_1050_65e2;
    uStack12._2_2_ = (int)(uStack12 >> 0x10);
    if (0x2 < uStack12) {
      iStack16 = (uint)uStack12 - 0x2;
      iStack14 = uStack12._2_2_ - (uint)((uint)uStack12 < 0x2);
      uVar5 = CONCAT22(iStack14,iStack16) % 0x14;
      if (uVar5 == 0x0) {
        uVar10 = (undefined)(param_4 >> 0x8);
        pass1_1028_dc52((astruct_92 *)CONCAT13(uVar10,CONCAT12((char)param_4,local_22)),0x1,0x0,0x400);
        while( true ) {
          uVar4 = (uint)uVar5;
          puVar2 = local_22;
          pass1_1028_e4ec(CONCAT22(param_4,puVar2));
          uStack38 = CONCAT22(uVar4,puVar2);
          uVar5 = (ulong)(uVar4 | (uint)puVar2);
          if ((uVar4 | (uint)puVar2) == 0x0) break;
          if (*(long *)(puVar2 + 0x200) != 0x8000002) {
            puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2a);
            uVar5 = (ulong)puVar8 >> 0x10;
            uVar4 = (uint)puVar8;
            puStack40 = (uchar *)((ulong)puVar8 >> 0x10);
            uVar6 = 0x38;
            uStack42 = uVar4;
            pass1_1038_4d6e(uStack38,puVar8,uVar4,puStack40);
            puStack46 = (undefined4 *)CONCAT22((int)uVar5,uVar4);
            ppcVar1 = (code **)((int)*puStack46 + 0x10);
            (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar4,(int)uVar5);
            uStack50 = CONCAT22((int)uVar5,uVar4);
            if (puStack8 == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
              uStack52 = 0x6;
            }
            else {
              uStack52 = 0xc;
            }
            for (uStack56 = 0x0; uStack56 < uStack50; uStack56 = uStack56 + 0x1) {
              uStack64 = pass1_1030_1d7c((int)uStack50,(int)uVar5,(ulong)puStack46);
              uVar5 = uStack64 >> 0x10;
              iVar3 = (int)uStack64;
              pass1_1028_7742((ushort)param_1,(ushort)(param_1 >> 0x10),0x4,uStack64,param_4);
              uVar4 = uStack52;
              if (iVar3 == 0x0) {
                uVar4 = 0x19;
              }
              uVar6 = 0x8;
              uStack68 = uVar4;
              iStack66 = iVar3;
              pass1_1008_612e(0x1,0x64,uVar4);
              uStack70 = uVar4;
              if ((int)uVar4 <= (int)uStack68) {
                paVar9 = pass1_1028_8fc0((astruct_100 *)CONCAT13(uVar10,CONCAT12((char)param_4,&local_156)),
                                         *(ulong *)((int)uStack64 + 0x4),*(ulong *)((int)uStack38 + 0x4),param_4,in_AF);
                uVar5 = (ulong)paVar9 >> 0x10;
                uVar6 = 0x30;
                fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,&local_156));
                local_156 = 0x389a;
                uStack340 = 0x1008;
              }
            }
            if (puStack46 != (undefined4 *)0x0) {
              ppcVar1 = (code **)*puStack46;
              (**ppcVar1)(uVar6,(int)puStack46,(int)((ulong)puStack46 >> 0x10),0x1,puStack46,puStack46);
            }
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_7dfc(ulong param_1,uchar *param_2,int param_3,ushort param_4,uchar param_5)

{
  code **ppcVar1;
  undefined *puVar2;
  uint uVar3;
  uchar *puVar4;
  ulong uVar5;
  undefined uVar6;
  ushort *puVar7;
  ulong *puVar8;
  astruct_100 *paVar9;
  undefined uVar10;
  undefined2 local_158;
  undefined2 uStack342;
  uint uStack72;
  uint uStack70;
  ulong uStack68;
  ulong uStack60;
  undefined2 uStack56;
  undefined2 uStack54;
  int iStack52;
  ulong uStack50;
  undefined4 *puStack46;
  uint uStack42;
  uchar *puStack40;
  ulong uStack38;
  undefined local_22 [0x12];
  int iStack16;
  int iStack14;
  undefined4 uStack12;
  undefined *puStack8;
  undefined2 uStack6;
  undefined2 uStack4;
  
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  uStack4 = (undefined2)((ulong)puVar7 >> 0x10);
  uStack6 = SUB42(puVar7,0x0);
  puStack8 = PTR_LOOP_1050_13ae;
  if (0x2 < (int)PTR_LOOP_1050_13ae) {
    uStack12 = *_PTR_LOOP_1050_65e2;
    uStack12._2_2_ = (int)(uStack12 >> 0x10);
    if (0x3 < uStack12) {
      iStack16 = (uint)uStack12 - 0x3;
      iStack14 = uStack12._2_2_ - (uint)((uint)uStack12 < 0x3);
      uVar5 = uStack12 % 0x14;
      if (uVar5 == 0x0) {
        uVar10 = (undefined)(param_4 >> 0x8);
        pass1_1028_dc52((astruct_92 *)CONCAT13(uVar10,CONCAT12((char)param_4,local_22)),0x1,0x0,0x400);
        while( true ) {
          uVar3 = (uint)uVar5;
          puVar2 = local_22;
          pass1_1028_e4ec(CONCAT22(param_4,puVar2));
          uStack38 = CONCAT22(uVar3,puVar2);
          uVar5 = (ulong)(uVar3 | (uint)puVar2);
          if ((uVar3 | (uint)puVar2) == 0x0) break;
          if (*(long *)(puVar2 + 0x200) != 0x8000002) {
            puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x29);
            puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
            uVar3 = (uint)puVar8;
            uStack42 = uVar3;
            puStack40 = puVar4;
            pass1_1038_4d6e(uStack38,puVar8,uVar3,puVar4);
            puStack46 = (undefined4 *)CONCAT22(puVar4,uVar3);
            ppcVar1 = (code **)((int)*puStack46 + 0x10);
            (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar3,puVar4);
            uStack50 = CONCAT22(puVar4,uVar3);
            uVar6 = 0x10;
            puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,puVar4,param_3);
            uVar5 = (ulong)puVar7 >> 0x10;
            uStack56 = SUB42(puVar7,0x0);
            uStack54 = (undefined2)((ulong)puVar7 >> 0x10);
            if (puStack8 == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
              iStack52 = 0x5;
            }
            else {
              iStack52 = 0x1e;
            }
            for (uStack60 = 0x0; uStack60 < uStack50; uStack60 = uStack60 + 0x1) {
              uStack68 = pass1_1030_1d7c((int)uStack50,(int)uVar5,(ulong)puStack46);
              uVar5 = uStack68 >> 0x10;
              uVar3 = (uint)uStack68;
              uVar6 = 0x8;
              pass1_1008_612e(0x1,0x64,uVar3);
              uStack70 = uVar3;
              if (((int)uVar3 <= iStack52) &&
                 (pass1_1028_7742((ushort)param_1,(ushort)(param_1 >> 0x10),0x4,uStack68,param_4), uStack72 = uVar3,
                 uVar3 == 0x0)) {
                paVar9 = pass1_1028_b0de((astruct_100 *)CONCAT13(uVar10,CONCAT12((char)param_4,&local_158)),
                                         *(ulong *)((int)uStack68 + 0x4),*(ulong *)((int)uStack38 + 0x4),param_4,param_5
                                        );
                uVar5 = (ulong)paVar9 >> 0x10;
                uVar6 = 0x30;
                fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,&local_158));
                local_158 = 0x389a;
                uStack342 = 0x1008;
              }
            }
            if (puStack46 != (undefined4 *)0x0) {
              ppcVar1 = (code **)*puStack46;
              (**ppcVar1)(uVar6,(int)puStack46,(int)((ulong)puStack46 >> 0x10),0x1,puStack46,puStack46);
            }
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_7fb6(ulong param_1,int param_2,ushort param_3,uchar param_4)

{
  code **ppcVar1;
  undefined *puVar2;
  uint uVar3;
  uchar *puVar4;
  ulong uVar5;
  undefined uVar6;
  ulong *puVar7;
  ushort *puVar8;
  astruct_100 *paVar9;
  undefined uVar10;
  undefined2 local_158;
  undefined2 uStack342;
  uint uStack72;
  uint uStack68;
  uint uStack66;
  ulong uStack64;
  ulong uStack56;
  int iStack52;
  undefined *puStack50;
  undefined2 uStack48;
  undefined2 uStack46;
  ulong uStack44;
  undefined4 *puStack40;
  uint uStack36;
  uchar *puStack34;
  ulong uStack32;
  undefined local_1c [0x12];
  int iStack10;
  int iStack8;
  undefined4 uStack6;
  
  uStack6 = *_PTR_LOOP_1050_65e2;
  uStack6._2_2_ = (int)(uStack6 >> 0x10);
  if (0xb < uStack6) {
    iStack10 = (uint)uStack6 - 0xb;
    iStack8 = uStack6._2_2_ - (uint)((uint)uStack6 < 0xb);
    uVar5 = uStack6 % 0x32;
    if (uVar5 == 0x0) {
      uVar10 = (undefined)(param_3 >> 0x8);
      pass1_1028_dc52((astruct_92 *)CONCAT13(uVar10,CONCAT12((char)param_3,local_1c)),0x1,0x0,0x400);
      while( true ) {
        uVar3 = (uint)uVar5;
        puVar2 = local_1c;
        pass1_1028_e4ec(CONCAT22(param_3,puVar2));
        uStack32 = CONCAT22(uVar3,puVar2);
        uVar5 = (ulong)(uVar3 | (uint)puVar2);
        if ((uVar3 | (uint)puVar2) == 0x0) break;
        if (*(long *)(puVar2 + 0x200) != 0x8000002) {
          puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x11);
          puVar4 = (uchar *)((ulong)puVar7 >> 0x10);
          uVar3 = (uint)puVar7;
          uStack36 = uVar3;
          puStack34 = puVar4;
          pass1_1038_4d6e(uStack32,puVar7,uVar3,puVar4);
          puStack40 = (undefined4 *)CONCAT22(puVar4,uVar3);
          ppcVar1 = (code **)((int)*puStack40 + 0x10);
          (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar3,puVar4);
          uStack44 = CONCAT22(puVar4,uVar3);
          uVar6 = 0x10;
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,puVar4,param_2);
          uVar5 = (ulong)puVar8 >> 0x10;
          uStack48 = SUB42(puVar8,0x0);
          uStack46 = (undefined2)((ulong)puVar8 >> 0x10);
          puStack50 = PTR_LOOP_1050_13ae;
          if ((int)PTR_LOOP_1050_13ae < 0x3) {
            iStack52 = 0x5;
          }
          else {
            iStack52 = 0x14;
          }
          for (uStack56 = 0x0; uStack56 < uStack44; uStack56 = uStack56 + 0x1) {
            uVar6 = 0x30;
            uStack64 = pass1_1030_1d7c((int)uStack44,(int)uVar5,(ulong)puStack40);
            uVar5 = uStack64 >> 0x10;
            uVar3 = *(uint *)((int)uStack64 + 0x20);
            uStack66 = uVar3;
            if (((uVar3 != 0x0) && (uVar3 != 0x70)) && (uVar3 != 0x71)) {
              uVar6 = 0x8;
              pass1_1008_612e(0x1,0x64,uVar3);
              uStack68 = uVar3;
              if (((int)uVar3 <= iStack52) &&
                 (pass1_1028_7742((ushort)param_1,(ushort)(param_1 >> 0x10),0x4,uStack64,param_3), uStack72 = uVar3,
                 uVar3 == 0x0)) {
                paVar9 = pass1_1028_8698((astruct_100 *)CONCAT13(uVar10,CONCAT12((char)param_3,&local_158)),
                                         *(ulong *)((int)uStack64 + 0x4),*(ulong *)((int)uStack32 + 0x4),param_4,param_3
                                        );
                uVar5 = (ulong)paVar9 >> 0x10;
                uVar6 = 0x30;
                fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,&local_158));
                local_158 = 0x389a;
                uStack342 = 0x1008;
              }
            }
          }
          if (puStack40 != (undefined4 *)0x0) {
            ppcVar1 = (code **)*puStack40;
            (**ppcVar1)(uVar6,(int)puStack40,(int)((ulong)puStack40 >> 0x10),0x1,puStack40,puStack40);
          }
        }
      }
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_816e(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1028_81aa(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x1b57);
  param_1->field_0x0 = 0x836e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCFactory_1050_5002);
  return param_1;
}



ushort __stdcall16far pass1_1028_81e0(uint param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint uVar4;
  uint extraout_DX;
  undefined4 *puStack24;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_14),0x1,0x0,0x700);
switchD_1028_8225_caseD_0:
  do {
    while( true ) {
      uVar4 = param_1;
      puVar3 = local_14;
      pass1_1028_e4ec(CONCAT22(param_3,puVar3));
      puStack24 = (undefined4 *)CONCAT22(uVar4,puVar3);
      param_1 = uVar4 | (uint)puVar3;
      if (param_1 == 0x0) {
        return 0x1;
      }
      iVar1 = *(int *)(puVar3 + 0xc);
      if (iVar1 < 0x35) goto code_r0x10288222;
      if (0x61 < iVar1) break;
      if ((iVar1 < 0x5d) && ((iVar1 != 0x37 && (iVar1 != 0x47)))) goto switchD_1028_8225_caseD_1;
    }
  } while ((iVar1 == 0x6a) ||
          ((0x8 < iVar1 + -0x6a &&
           ((iVar1 == 0x75 || iVar1 + -0x74 < 0x1 || ((0x0 < iVar1 + -0x76 && (iVar1 + -0x78 < 0x2))))))));
  goto switchD_1028_8225_caseD_1;
code_r0x10288222:
  param_2 = (ushort)&USHORT_1050_1028;
  switch(iVar1) {
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x4:
  case 0x6:
  case 0x7:
  case 0x8:
  case 0xa:
  case 0xb:
  case 0xc:
  case 0xd:
  case 0xe:
  case 0xf:
  case 0x11:
switchD_1028_8225_caseD_1:
    if (*(int *)(puVar3 + 0x12) == 0x5) {
      ppcVar2 = (code **)((int)*puStack24 + 0x30);
      (**ppcVar2)(param_2);
      param_1 = extraout_DX;
    }
  }
  goto switchD_1028_8225_caseD_0;
}



void __stdcall16far pass1_1028_82b4(ulong param_1,uint param_2,uchar *param_3)

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
    *puStack10 = 0x836e;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_8342(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}
