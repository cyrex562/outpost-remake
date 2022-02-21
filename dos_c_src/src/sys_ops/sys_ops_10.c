
void __stdcall16far pass1_1008_cbc4(ulong param_1,ulong param_2,ushort param_3)

{
  long *plVar1;
  code **ppcVar2;
  bool bVar3;
  undefined4 *puVar4;
  ushort uVar5;
  undefined *puVar6;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar8;
  uchar *puVar9;
  uchar *extraout_DX_01;
  astruct_202 *iVar10;
  undefined2 uVar10;
  char *pcVar11;
  ulong uStack64;
  ulong uStack52;
  int iStack30;
  undefined local_18 [0x8];
  undefined2 uStack16;
  uchar *puStack14;
  undefined2 uStack12;
  uchar *puStack10;
  int iStack8;
  long lStack6;
  ulong uVar7;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar10 = (astruct_202 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar4 = iVar10->field_0x1e;
  puVar8 = *(uchar **)((int)&iVar10->field_0x1e + 0x2);
  if (((uint)puVar8 | (uint)puVar4) != 0x0) {
    ppcVar2 = (code **)*puVar4;
    (**ppcVar2)();
    puVar8 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar8,0x1000);
  if (((uint)puVar8 | (uint)puVar4) == 0x0) {
    puVar4 = (undefined4 *)0x0;
    puVar8 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar8,puVar4));
    puVar8 = extraout_DX_00;
  }
  *(undefined4 **)&iVar10->field_0x1e = puVar4;
  *(uchar **)((int)&iVar10->field_0x1e + 0x2) = puVar8;
  lStack6 = *(long *)((int)param_2 + 0x200);
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_18),0x1,0x0,0x400);
  iStack30 = 0x0;
  while( true ) {
    puVar6 = local_18;
    pass1_1028_e4ec(CONCAT22(param_3,puVar6));
    puVar9 = (uchar *)((uint)puVar8 | (uint)puVar6);
    if (puVar9 == (uchar *)0x0) break;
    plVar1 = (long *)(puVar6 + 0x200);
    puVar8 = puVar9;
    if (*plVar1 == lStack6) {
      iStack30 = iStack30 + 0x1;
    }
  }
  bVar3 = false;
  if (0x1 < iStack30) {
    uStack16 = uStack12;
    puStack14 = puStack10;
    if (iStack8 != 0x0) {
      uStack16 = 0x1;
      puStack14 = (uchar *)0x0;
      puStack10 = puStack14;
    }
    while( true ) {
      puVar8 = puStack10;
      puVar6 = local_18;
      pass1_1028_e4ec(CONCAT22(param_3,puVar6));
      puVar9 = (uchar *)((uint)puVar8 | (uint)puVar6);
      if (puVar9 == (uchar *)0x0) break;
      puStack10 = puVar9;
      if ((*(long *)(puVar6 + 0x200) == lStack6) && (*(long *)(puVar6 + 0x4) != 0x4000001)) {
        pcVar11 = pass1_1038_4d28(CONCAT22(puVar8,puVar6));
        puVar9 = (uchar *)((ulong)pcVar11 >> 0x10);
        uVar5 = str_op_1008_60e8(pcVar11,(ushort)puVar9);
        uVar7 = (ulong)uVar5;
        uStack52 = CONCAT22(puVar9,uVar5);
        mem_op_1000_179c(0x12,puVar9,0x1000);
        if (((uint)puVar9 | (uint)uVar7) != 0x0) {
          struct_1018_4920((ushort *)(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10),0x1,uStack52,*(ulong *)(puVar6 + 0x4));
        }
        ppcVar2 = (code **)((int)*iVar10->field_0x1e + 0x4);
        (**ppcVar2)();
        bVar3 = true;
        puStack10 = extraout_DX_01;
      }
    }
  }
  if (!bVar3) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uStack64 = CONCAT22(puVar9,puVar6);
    mem_op_1000_179c(0x12,puVar9,0x1000);
    if (((uint)puVar9 | (uint)puVar6) != 0x0) {
      struct_1018_4920((ushort *)CONCAT22(puVar9,puVar6),0x0,uStack64,0x0);
    }
    ppcVar2 = (code **)((int)*iVar10->field_0x1e + 0x4);
    (**ppcVar2)();
  }
  return;
}
void __stdcall16far pass1_1008_cda2(ulong param_1,ulong param_2,ushort param_3)

{
  long *plVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  undefined4 *puVar4;
  char *pcVar5;
  ushort uVar6;
  ushort uVar7;
  astruct_206 *puVar9;
  uint uVar8;
  uint uVar9;
  ulong uVar10;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  uchar *puVar11;
  undefined2 uVar12;
  uint extraout_DX_01;
  uchar *puVar13;
  astruct_205 *iVar15;
  undefined2 uVar14;
  undefined2 uVar15;
  undefined uVar16;
  ushort *puVar17;
  long lStack50;
  undefined local_2e [0xa];
  ushort uStack36;
  ulong uStack34;
  ulong uStack30;
  undefined4 uStack26;
  undefined4 *puStack18;
  uchar *puStack16;
  ushort *puStack14;
  undefined2 uStack10;
  undefined4 uStack8;
  int iStack4;
  
  uVar14 = (undefined2)(param_1 >> 0x10);
  iVar15 = (astruct_205 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar4 = iVar15->field_0x1a;
  puVar13 = *(uchar **)((int)&iVar15->field_0x1a + 0x2);
  puStack14 = (ushort *)CONCAT22(puVar13,puVar4);
  puStack18 = puVar4;
  puStack16 = puVar13;
  if (((uint)puVar13 | (uint)puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
    puVar13 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar13,0x1000);
  puStack18 = puVar4;
  puStack16 = puVar13;
  if (((uint)puVar13 | (uint)puVar4) == 0x0) {
    puVar4 = (undefined4 *)0x0;
    uVar15 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar13,puVar4));
    uVar15 = extraout_DX_00;
  }
  *(undefined4 **)&iVar15->field_0x1a = puVar4;
  *(undefined2 *)((int)&iVar15->field_0x1a + 0x2) = uVar15;
  iStack4 = 0x0;
  uVar15 = (undefined2)(param_2 >> 0x10);
  uStack8 = *(undefined4 *)((int)param_2 + 0x210);
  uStack26._2_2_ = *(uchar **)((int)param_2 + 0x212);
  uVar10 = (ulong)((uint)uStack26._2_2_ | (uint)uStack8);
  if (((uint)uStack26._2_2_ | (uint)uStack8) != 0x0) {
    uStack26 = *(ulong *)((uint)uStack8 + 0xa);
    uStack30 = 0x0;
    while( true ) {
      uVar10 = uStack26;
      if (uStack26 <= uStack30) break;
      bad_1030_1312();
      uStack34 = uVar10 & 0xffff | ZEXT24(uStack26._2_2_) << 0x10;
      if (((uint)uStack26._2_2_ | (uint)uVar10) != 0x0) {
        for (uStack36 = 0x1; (int)uStack36 < 0x15; uStack36 = uStack36 + 0x1) {
          local_2e._8_2_ = pass1_1030_ce2e((int)uStack34,(ushort)(uStack34 >> 0x10),uStack36);
          if (local_2e._8_2_ != 0x0) {
            pass1_1008_5784((ulong *)CONCAT22(param_3,local_2e),(ulong)iVar15->field_0x1a);
            do {
              puVar9 = (astruct_206 *)local_2e;
              pass1_1008_5b12(puVar9,param_3);
              lStack50 = CONCAT22(extraout_DX_01,puVar9);
              puVar13 = (uchar *)(extraout_DX_01 | (uint)puVar9);
              if (puVar13 == (uchar *)0x0) break;
            } while (puVar9->field_0xe != uStack36);
            if (lStack50 == 0x0) {
              pcVar5 = string_op_1020_c222(uStack36);
              uVar6 = str_op_1008_60e8((char *)CONCAT22(puVar13,pcVar5),(ushort)puVar13);
              uVar16 = 0x0;
              puVar11 = puVar13;
              uVar7 = uVar6;
              mem_op_1000_179c(0x10,puVar13,0x1000);
              puStack14 = (ushort *)CONCAT22(puVar11,uVar7);
              if (((uint)puVar11 | uVar7) == 0x0) {
                uVar15 = 0x0;
                uVar12 = 0x0;
              }
              else {
                uVar16 = 0x18;
                puVar17 = struct_1018_48b0(puStack14,
                                           CONCAT22((int)local_2e._8_2_ >> 0xf,
                                                    local_2e._8_2_ & 0xff |
                                                    (uint)(byte)((ulong)(long)(int)local_2e._8_2_ >> 0x8) << 0x8),
                                           CONCAT22(puVar13,uVar6),uStack36);
                uVar12 = (undefined2)((ulong)puVar17 >> 0x10);
                uVar15 = SUB42(puVar17,0x0);
              }
              puVar2 = iVar15->field_0x1a;
              ppcVar3 = (code **)((int)*iVar15->field_0x1a + 0x4);
              (**ppcVar3)(uVar16,(int)puVar2,(char)((ulong)puVar2 >> 0x10),uVar15,uVar12);
            }
            else {
              plVar1 = &puVar9->field_0x8;
              *plVar1 = *plVar1 + (long)(int)local_2e._8_2_;
            }
            iStack4 = 0x1;
          }
        }
      }
      uStack30 = uStack30 + 0x1;
    }
  }
  uVar8 = (uint)uVar10;
  uStack10 = 0x0;
  if (iStack4 == 0x0) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar16 = 0x0;
    puVar13 = uStack26._2_2_;
    uVar9 = uVar8;
    mem_op_1000_179c(0x10,uStack26._2_2_,0x1000);
    puStack18 = (undefined4 *)uVar9;
    puStack16 = puVar13;
    if (((uint)puVar13 | uVar9) == 0x0) {
      uVar15 = 0x0;
      uVar12 = 0x0;
    }
    else {
      uVar16 = 0x18;
      puVar17 = struct_1018_48b0((ushort *)CONCAT22(puVar13,uVar9),0x0,CONCAT22(uStack26._2_2_,uVar8),0x0);
      uVar12 = (undefined2)((ulong)puVar17 >> 0x10);
      uVar15 = SUB42(puVar17,0x0);
    }
    puVar2 = iVar15->field_0x1a;
    ppcVar3 = (code **)((int)*iVar15->field_0x1a + 0x4);
    (**ppcVar3)(uVar16,(int)puVar2,(char)((ulong)puVar2 >> 0x10),uVar15,uVar12);
  }
  return;
}

void __stdcall16far pass1_1008_cfa0(ulong param_1,ulong param_2)

{
  ulong uVar1;
  undefined4 uVar2;
  code **ppcVar3;
  bool bVar4;
  undefined4 *puVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  undefined2 uVar9;
  ulong uVar10;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar11;
  uchar *puVar12;
  uchar *puVar13;
  uchar *extraout_DX_01;
  uchar *extraout_DX_02;
  uchar *extraout_DX_03;
  undefined2 uVar14;
  int iVar15;
  undefined2 uVar16;
  undefined2 uVar17;
  ushort unaff_SS;
  ushort *puVar18;
  
  uVar16 = (undefined2)(param_1 >> 0x10);
  iVar15 = (int)param_1;
  puVar5 = (undefined4 *)*(uint *)(iVar15 + 0x16);
  puVar11 = *(uchar **)(iVar15 + 0x18);
  if (((uint)puVar11 | (uint)puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
    puVar11 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar11,0x1000);
  if (((uint)puVar11 | (uint)puVar5) == 0x0) {
    puVar5 = (undefined4 *)0x0;
    puVar11 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar11,puVar5));
    puVar11 = extraout_DX_00;
  }
  *(undefined2 *)(iVar15 + 0x16) = puVar5;
  *(uchar **)(iVar15 + 0x18) = puVar11;
  bVar4 = false;
  uVar1 = *(ulong *)((int)param_2 + 0x1f6);
  uVar10 = uVar1;
  pass1_1030_38f2(uVar1,0x2,unaff_SS);
  uVar6 = (uint)uVar10;
  if ((-0x1 < (int)puVar11) && ((0x0 < (int)puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar6;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if (((uint)puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar13,uVar7),uVar10 & 0xffff | ZEXT24(puVar11) << 0x10,
                                 CONCAT22(puVar12,uVar6),0x2);
      uVar9 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar6 = (uint)puVar18;
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_01;
  }
  pass1_1030_38f2(uVar1,0x3,unaff_SS);
  if ((-0x1 < (int)puVar11) && ((0x0 < (int)puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    uVar8 = uVar6;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar8;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if (((uint)puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar13,uVar7),CONCAT22(puVar11,uVar6),CONCAT22(puVar12,uVar8),0x3);
      uVar9 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar6 = (uint)puVar18;
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_02;
  }
  pass1_1030_38f2(uVar1,0x4,unaff_SS);
  if ((-0x1 < (int)puVar11) && ((0x0 < (int)puVar11 || (uVar6 != 0x0)))) {
    puVar12 = puVar11;
    uVar8 = uVar6;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar13 = puVar12;
    uVar7 = uVar8;
    mem_op_1000_179c(0x14,puVar12,0x1000);
    if (((uint)puVar13 | uVar7) == 0x0) {
      uVar6 = 0x0;
      uVar9 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar13,uVar7),CONCAT22(puVar11,uVar6),CONCAT22(puVar12,uVar8),0x4);
      uVar9 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar6 = (uint)puVar18;
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar6,uVar9);
    bVar4 = true;
    puVar11 = extraout_DX_03;
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar17 = 0x1000;
    puVar12 = puVar11;
    uVar7 = uVar6;
    mem_op_1000_179c(0x14,puVar11,0x1000);
    if (((uint)puVar12 | uVar7) == 0x0) {
      uVar9 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      puVar18 = struct_1018_4842((ushort *)CONCAT22(puVar12,uVar7),0x0,CONCAT22(puVar11,uVar6),0x0);
      uVar14 = (undefined2)((ulong)puVar18 >> 0x10);
      uVar9 = SUB42(puVar18,0x0);
    }
    uVar2 = *(undefined4 *)(iVar15 + 0x16);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0x16) + 0x4);
    (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar9,uVar14);
  }
  return;
}


void __stdcall16far unk_str_op_1008_d1c6(ulong param_1,ulong param_2)

{
  int iVar1;
  undefined4 uVar2;
  code **ppcVar3;
  bool bVar4;
  undefined4 *puVar5;
  uint uVar6;
  ushort uVar7;
  ushort uVar8;
  uint uVar9;
  undefined uVar10;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  uchar *puVar11;
  uchar *extraout_DX_01;
  uint uVar12;
  uchar *puVar13;
  uchar *extraout_DX_02;
  uchar *puVar14;
  uint uVar15;
  int iVar16;
  WORD *valist;
  undefined2 uVar17;
  ulong *puVar18;
  ulong uVar19;
  uint uStack52;
  ulong uStack20;
  ulong uStack14;
  undefined4 *puStack10;
  
  valist = (WORD *)(param_1 >> 0x10);
  iVar16 = (int)param_1;
  puVar5 = (undefined4 *)*(uint *)(iVar16 + 0x12);
  puVar13 = *(uchar **)(iVar16 + 0x14);
  if (((uint)puVar13 | (uint)puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
    puVar13 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar13,0x1000);
  if (((uint)puVar13 | (uint)puVar5) == 0x0) {
    puVar5 = (undefined4 *)0x0;
    uVar17 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar13,puVar5));
    uVar17 = extraout_DX_00;
  }
  *(undefined2 *)(iVar16 + 0x12) = puVar5;
  *(undefined2 *)(iVar16 + 0x14) = uVar17;
  puVar18 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
  puVar11 = (uchar *)((ulong)puVar18 >> 0x10);
  uVar6 = (uint)puVar18;
  uVar17 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4e78(uVar6,puVar11,param_2,puVar18);
  puStack10 = (undefined4 *)CONCAT22(puVar11,uVar6);
  ppcVar3 = (code **)((int)*puStack10 + 0x10);
  uVar9 = uVar6;
  (**ppcVar3)((int)&PTR_LOOP_1050_1038,uVar6,puVar11);
  uStack14 = CONCAT22(extraout_DX_01,uVar9);
  bVar4 = false;
  puVar13 = extraout_DX_01;
  for (uStack20 = 0x0; uStack20 < uStack14; uStack20 = uStack20 + 0x1) {
    uVar17 = 0x1030;
    uVar19 = pass1_1030_1d7c(uVar9,puVar13,(ulong)puStack10);
    uVar12 = (uint)(uVar19 >> 0x10);
    uVar15 = (uint)uVar19;
    puVar13 = (uchar *)(uVar12 | uVar15);
    if (((puVar13 != (uchar *)0x0) && (*(long *)(uVar15 + 0x1c) != 0x8000002)) &&
       ((iVar1 = *(int *)(uVar15 + 0x12), iVar1 == 0x5 || (iVar1 == 0x6)))) {
      puVar13 = (uchar *)(*(uint *)(uVar15 + 0x6) & 0xff);
      pass1_1020_bd80(*(ushort *)(uVar15 + 0xc));
      wsprintf16((LPSTR)0x1020,(LPCSTR)(iVar16 + 0x22),valist);
      uVar7 = str_op_1008_60e8((char *)(param_1 & 0xffff0000 | (ulong)(iVar16 + 0x22)),(ushort)puVar13);
      uVar17 = 0x1000;
      puVar14 = puVar13;
      uVar8 = uVar7;
      mem_op_1000_179c(0x12,puVar13,0x1000);
      uStack52 = (uint)puVar14 | uVar8;
      if (uStack52 == 0x0) {
        uVar8 = 0x0;
        uStack52 = 0x0;
      }
      else {
        uVar17 = 0x1018;
        pass1_1018_4808((ushort *)CONCAT22(puVar14,uVar8),0x1,
                        CONCAT13((char)((uint)puVar13 >> 0x8),CONCAT12((char)puVar13,uVar7)),*(ulong *)(uVar15 + 0x4));
      }
      uVar2 = *(undefined4 *)(iVar16 + 0x12);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar16 + 0x12) + 0x4);
      (**ppcVar3)(uVar17,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar8,uStack52);
      bVar4 = true;
      puVar13 = extraout_DX_02;
    }
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar9 = (uint)uStack14;
    uVar17 = 0x1000;
    puVar14 = puVar13;
    mem_op_1000_179c(0x12,puVar13,0x1000);
    uVar15 = (uint)puVar14 | uVar9;
    if (uVar15 == 0x0) {
      uVar9 = 0x0;
      uVar10 = 0x0;
    }
    else {
      uVar17 = 0x1018;
      pass1_1018_4808((ushort *)CONCAT22(puVar14,uVar9),0x0,uStack14 & 0xffff | ZEXT24(puVar13) << 0x10,0x0);
      uVar10 = (undefined)uVar15;
    }
    uVar2 = *(undefined4 *)(iVar16 + 0x12);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar16 + 0x12) + 0x4);
    (**ppcVar3)(uVar17,(char)uVar2,(int)((ulong)uVar2 >> 0x10),uVar9,uVar10);
  }
  if (((uint)puVar11 | uVar6) != 0x0) {
    ppcVar3 = (code **)*puStack10;
    (**ppcVar3)(uVar17,uVar6,(char)puVar11,0x1);
  }
  return;
}


void __stdcall16far pass1_1008_d3ae(ulong param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  bool bVar4;
  uint uVar5;
  ushort uVar6;
  ushort uVar7;
  uchar *puVar8;
  uchar *puVar9;
  astruct_208 *iVar13;
  undefined2 uVar10;
  undefined2 uVar11;
  astruct_21 *paVar12;
  ulong uVar13;
  undefined4 uVar14;
  ulong uVar15;
  ushort uStack6;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar13 = (astruct_208 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar1 = iVar13->field_0xa;
  uVar5 = *(uint *)((int)&iVar13->field_0xa + 0x2);
  paVar12 = (astruct_21 *)CONCAT22(uVar5,puVar1);
  if ((uVar5 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    paVar12 = (astruct_21 *)(**ppcVar3)();
  }
  mem_op_1000_179c(0xc,(uchar *)((ulong)paVar12 >> 0x10),0x1000);
  if (paVar12 == (astruct_21 *)0x0) {
    uVar13 = 0x0;
  }
  else {
    uVar13 = set_struct_1008_574a(paVar12);
  }
  *(int *)&iVar13->field_0xa = (int)uVar13;
  *(undefined2 *)((int)&iVar13->field_0xa + 0x2) = (int)(uVar13 >> 0x10);
  bVar4 = false;
  for (uStack6 = 0x21; 0x10 < (int)uStack6; uStack6 = uStack6 - 0x1) {
    uVar15 = uVar13;
    empty_1038_540a();
    puVar8 = (uchar *)(uVar15 >> 0x10);
    uVar5 = (uint)puVar8 | (uint)uVar15;
    uVar13 = uVar15 & 0xffff0000 | (ulong)uVar5;
    if (uVar15 != 0x0) {
      bVar4 = true;
      string_1020_c0ca(uStack6);
      uVar6 = str_op_1008_60e8((char *)CONCAT22(puVar8,uVar5),(ushort)puVar8);
      uVar11 = 0x1000;
      uVar7 = uVar6;
      puVar9 = puVar8;
      mem_op_1000_179c(0x10,puVar8,0x1000);
      if (((uint)puVar9 | uVar7) == 0x0) {
        uVar14 = 0x0;
      }
      else {
        uVar11 = 0x1018;
        uVar14 = struct_1018_4790(CONCAT22(puVar9,uVar7),uVar15,CONCAT22(puVar8,uVar6),uStack6);
      }
      puVar2 = iVar13->field_0xa;
      ppcVar3 = (code **)((int)*iVar13->field_0xa + 0x4);
      uVar13 = (**ppcVar3)(uVar11,(char)puVar2,(char)((ulong)puVar2 >> 0x10),uVar14);
    }
  }
  if (!bVar4) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar11 = 0x1000;
    uVar15 = uVar13;
    mem_op_1000_179c(0x10,(uchar *)(uVar13 >> 0x10),0x1000);
    if (uVar15 == 0x0) {
      uVar14 = 0x0;
    }
    else {
      uVar11 = 0x1018;
      uVar14 = struct_1018_4790(uVar15,0x0,uVar13,0x0);
    }
    puVar2 = iVar13->field_0xa;
    ppcVar3 = (code **)((int)*iVar13->field_0xa + 0x4);
    (**ppcVar3)(uVar11,(char)puVar2,(char)((ulong)puVar2 >> 0x10),uVar14);
  }
  return;
}

void __stdcall16far pass1_1008_b200(ulong param_1,ushort param_2)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  undefined *puVar4;
  astruct_195 *uVar5;
  ushort uVar6;
  ulong uVar7;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar8;
  uchar *puVar9;
  undefined2 extraout_DX_01;
  undefined2 uVar10;
  ushort uVar11;
  uchar *extraout_DX_02;
  astruct_194 *iVar12;
  undefined2 uVar12;
  char *pcVar13;
  undefined local_14 [0x12];
  
  uVar12 = (undefined2)(param_1 >> 0x10);
  iVar12 = (astruct_194 *)param_1;
  if (iVar12->field_0xe != (undefined4 *)0x0) {
    return;
  }
                    // WARNING: Load size is inaccurate
  puVar3 = iVar12->field_0xe;
  puVar9 = *(uchar **)((int)&iVar12->field_0xe + 0x2);
  if (((uint)puVar9 | (uint)puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
    puVar9 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar9,0x1000);
  if (((uint)puVar9 | (uint)puVar3) == 0x0) {
    puVar3 = (undefined4 *)0x0;
    puVar9 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar9,puVar3));
    puVar9 = extraout_DX_00;
  }
  *(undefined4 **)&iVar12->field_0xe = puVar3;
  *(uchar **)((int)&iVar12->field_0xe + 0x2) = puVar9;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  while( true ) {
    puVar8 = puVar9;
    puVar4 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar4));
    puVar9 = (uchar *)((uint)puVar8 | (uint)puVar4);
    if (puVar9 == (uchar *)0x0) break;
    uVar1 = *(ulong *)(puVar4 + 0x4);
    if (*(long *)(puVar4 + 0x200) == 0x8000001) {
      uVar7 = uVar1;
      mem_op_1000_179c(0xc,puVar9,0x1000);
      uVar5 = (astruct_195 *)uVar7;
      if (((uint)puVar9 | (uint)uVar5) == 0x0) {
        uVar5 = (astruct_195 *)0x0;
        uVar10 = 0x0;
      }
      else {
        pass1_1008_b0f2((ushort *)(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10));
        uVar10 = extraout_DX_01;
      }
      pcVar13 = pass1_1038_4d28(CONCAT22(puVar8,puVar4));
      uVar11 = (ushort)((ulong)pcVar13 >> 0x10);
      uVar6 = str_op_1008_60e8(pcVar13,uVar11);
      uVar5->field_0x4 = uVar6;
      uVar5->field_0x6 = uVar11;
      uVar5->field_0x8 = uVar1;
      ppcVar2 = (code **)((int)*iVar12->field_0xe + 0x8);
      (**ppcVar2)((int)&PTR_LOOP_1050_1038,iVar12->field_0xe,uVar5,uVar10);
      puVar9 = extraout_DX_02;
    }
  }
  return;
}


ulong __stdcall16far pass1_1008_b38c(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined2 uVar3;
  astruct_197 *iVar3;
  astruct_196 *iVar4;
  undefined2 uVar4;
  ushort *puVar5;
  int iStack4;
  astruct_198 *iVar5;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_196 *)param_1;
  if (iVar4->field_0x12 == (undefined4 *)0x0) {
    mem_op_1000_179c(0xc,param_3,0x1000);
    puVar5 = (ushort *)CONCAT22((uint)param_3 | param_2,param_2);
    if (((uint)param_3 | param_2) == 0x0) {
      iVar4->field_0x12 = (undefined4 *)0x0;
    }
    else {
      puVar5 = (ushort *)set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,param_2));
      *(int *)&iVar4->field_0x12 = (int)puVar5;
      *(undefined2 *)((int)&iVar4->field_0x12 + 0x2) = (int)((ulong)puVar5 >> 0x10);
    }
    for (iStack4 = 0x6d9; iStack4 < 0x6e7; iStack4 = iStack4 + 0x1) {
      if (iStack4 == 0x6e3) {
        pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
        if (*(int *)((int)puVar5 + 0x136) != 0x0) goto LAB_1008_b44a;
      }
      else {
LAB_1008_b44a:
        mem_op_1000_179c(0xa,(uchar *)((ulong)puVar5 >> 0x10),0x1000);
        if (puVar5 == (ushort *)0x0) {
          puVar5 = (ushort *)0x0;
        }
        else {
          puVar5 = pass1_1008_b11e(puVar5);
        }
        uVar3 = (undefined2)((ulong)puVar5 >> 0x10);
        iVar5 = (astruct_198 *)puVar5;
        load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        iVar5->field_0x4 = (int)puVar5;
        iVar5->field_0x6 = (int)((ulong)puVar5 >> 0x10);
        iVar5->field_0x8 = iStack4 + -0x6d8;
        puVar1 = iVar4->field_0x12;
        ppcVar2 = (code **)((int)*iVar4->field_0x12 + 0x8);
        puVar5 = (ushort *)(**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),iVar5,uVar3);
      }
    }
  }
  return CONCAT22(*(undefined2 *)((int)&iVar4->field_0x12 + 0x2),*(undefined2 *)&iVar4->field_0x12);
}


void __stdcall16far
pass1_1008_a1f0(ushort param_1,ushort param_2,uchar param_3,ulong param_4,ushort *param_5,ushort *param_6,
               ushort *param_7,ushort *param_8)

{
  undefined4 uVar1;
  code **ppcVar2;
  ulong uVar3;
  uint uVar4;
  ushort uVar5;
  uint extraout_DX;
  uint uVar6;
  uchar *puVar7;
  ushort uVar8;
  int iVar9;
  uchar *in_buf_len_5;
  undefined2 uVar10;
  undefined2 uVar11;
  ushort *puVar12;
  char *pcVar13;
  ushort uVar14;
  undefined uVar15;
  undefined uVar16;
  char local_106 [0x100];
  undefined4 *puStack6;
  
  uVar4 = 0x0;
  *param_8 = 0x0;
  *param_7 = 0x0;
  *param_6 = 0x0;
  *param_5 = 0x0;
  in_buf_len_5 = (uchar *)(param_4 >> 0x10);
  uVar8 = (ushort)param_4;
  *(undefined *)(uVar8 + 0xe) = 0x0;
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar8 + 0xa) + 0x10);
  (**ppcVar2)(param_1,*(undefined4 *)(uVar8 + 0xa));
  puStack6 = (undefined4 *)CONCAT22(extraout_DX,uVar4);
  uVar6 = extraout_DX | uVar4;
  if (uVar6 == 0x0) {
    return;
  }
  *param_8 = *(ushort *)(uVar4 + 0x4);
  *param_6 = *(ushort *)(uVar4 + 0xa);
  uVar5 = pass1_1008_ab80(uVar8,(ushort)in_buf_len_5,*param_8);
  *param_5 = uVar5;
  uVar10 = (undefined2)((ulong)puStack6 >> 0x10);
  iVar9 = (int)puStack6;
  uVar11 = 0x1008;
  uVar14 = (ushort)_PTR_LOOP_1050_14cc;
  uVar5 = (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
  switch(*(undefined2 *)(iVar9 + 0x4)) {
  case 0x1:
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd1;
    goto LAB_1008_a2b1;
  case 0x2:
    uVar1 = *(undefined4 *)(iVar9 + 0x6);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_106,param_2);
    uVar3 = (ulong)puStack6 >> 0x10;
    pcVar13 = pass1_1038_4d28(CONCAT13((char)(uVar6 >> 0x8),CONCAT12((char)uVar6,iVar9)));
    uVar11 = 0x1000;
    sys_1000_3f9c((uchar *)(uVar8 + 0xe),in_buf_len_5,(ushort)local_106,param_2,(ushort)pcVar13,&stack0xfffe,(int)uVar3,
                  0x1000,param_2,param_3);
    break;
  case 0x5:
    goto LAB_1008_a277;
  case 0x6:
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd4;
LAB_1008_a2b1:
    uVar11 = 0x1010;
    *param_6 = 0x1;
    break;
  case 0x7:
LAB_1008_a277:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    break;
  case 0x9:
    if (*(int *)(uVar8 + 0x416) == 0x0) {
      *(undefined2 *)(uVar8 + 0x416) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0xb:
    if (*(int *)(uVar8 + 0x41a) == 0x0) {
      *(undefined2 *)(uVar8 + 0x41a) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0xe:
    if (*(int *)(uVar8 + 0x41c) == 0x0) {
      *(undefined2 *)(uVar8 + 0x41c) = 0x1;
      break;
    }
    goto LAB_1008_a35a;
  case 0x14:
    if (*(int *)(uVar8 + 0x418) == 0x0) {
      *(undefined2 *)(uVar8 + 0x418) = 0x1;
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
                 (char *)(uVar8 + 0xe),(short)in_buf_len_5);
      pcVar13 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      puVar7 = (uchar *)((ulong)pcVar13 >> 0x10);
      pass1_1000_3cea(param_4 & 0xffff0000 | ZEXT24((char *)(uVar8 + 0xe)),(ULONG)pcVar13);
      *param_7 = 0x4c;
      uVar15 = 0x1;
      uVar16 = 0x0;
      iVar9 = 0xa;
      puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_2,puVar7,(int)in_buf_len_5);
      uVar11 = 0x1010;
      pass1_1010_089e(param_2,(ulong)puVar12,CONCAT11(uVar16,uVar15),iVar9);
      break;
    }
    goto LAB_1008_a35a;
  case 0x16:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x28;
    break;
  case 0x17:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2c;
    break;
  case 0x18:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2e;
    break;
  case 0x1b:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x30;
    break;
  case 0x1c:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x32;
    break;
  case 0x1f:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x34;
    break;
  case 0x21:
    if (*(int *)(uVar8 + 0x41e) == 0x0) {
      *(undefined2 *)(uVar8 + 0x41e) = 0x1;
      break;
    }
LAB_1008_a35a:
    *param_5 = 0x0;
    break;
  case 0x24:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2a;
    break;
  case 0x31:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x27;
    break;
  case 0x32:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x29;
    break;
  case 0x33:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2b;
    break;
  case 0x34:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2d;
    break;
  case 0x35:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2f;
    break;
  case 0x36:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x31;
    break;
  case 0x37:
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    pcVar13 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar11 = 0x1000;
    pass1_1000_3cea(param_4 & 0xffff0000 | ZEXT24((char *)(uVar8 + 0xe)),(ULONG)pcVar13);
    *param_7 = 0x33;
    break;
  case 0x38:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x35;
    break;
  case 0x39:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x36;
    break;
  case 0x3a:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x37;
    break;
  case 0x3b:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x38;
    break;
  case 0x3c:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x39;
    break;
  case 0x3d:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xce;
    break;
  case 0x3e:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xcf;
    break;
  case 0x3f:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd0;
    break;
  case 0x40:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd1;
    break;
  case 0x41:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd2;
    break;
  case 0x42:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd3;
    break;
  case 0x43:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd5;
    break;
  case 0x44:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd6;
    break;
  case 0x45:
    uVar11 = 0x1010;
    load_string_1010_84e0(0x1010,uVar14,uVar5,0x3ff,(char *)(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd7;
  }
  if (puStack6 != (undefined4 *)0x0) {
    ppcVar2 = (code **)*puStack6;
    (**ppcVar2)(uVar11,(int)puStack6,(char)((ulong)puStack6 >> 0x10),0x1);
  }
  return;
}

void __stdcall16far pass1_1008_a930(ulong param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint extraout_DX;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 *puStack24;
  undefined2 *puStack18;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar5 + 0x410));
  do {
    puVar3 = local_a;
    pass1_1008_5b12(puVar3,param_3);
    uVar4 = extraout_DX | (uint)puVar3;
    if (uVar4 == 0x0) {
      mem_op_1000_179c(0x6,(uchar *)0x0,0x1000);
      puStack24 = (undefined2 *)CONCAT22(uVar4,puVar3);
      if ((uVar4 | (uint)puVar3) == 0x0) {
        puStack18 = (undefined2 *)0x0;
      }
      else {
        *puStack24 = 0x389a;
        *(undefined2 *)(puVar3 + 0x2) = 0x1008;
        *(int *)(puVar3 + 0x4) = param_2;
        *puStack24 = 0xad8a;
        *(undefined2 *)(puVar3 + 0x2) = 0x1008;
        puStack18 = puStack24;
      }
      uVar1 = *(undefined4 *)(iVar5 + 0x410);
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x410) + 0x8);
      (**ppcVar2)(0x1000,(int)uVar1,(char)((ulong)uVar1 >> 0x10),(int)puStack18,(int)((ulong)puStack18 >> 0x10));
      return;
    }
  } while (*(int *)(puVar3 + 0x4) != param_2);
  return;
}

void __stdcall16far pass1_1008_9d36(uchar *param_1,uchar *param_2,ushort param_3,ushort param_4)

{
  uchar *puVar1;
  undefined2 uVar2;
  ushort *puVar3;
  astruct_43 *paVar4;
  ulong uVar5;
  int iStack4;
  
  struct_op_1018_4cda((int)param_1,(ushort)param_2,param_3);
  *(undefined2 *)(param_1 + 0x1c) = 0x389a;
  *(undefined2 *)(param_1 + 0x1e) = 0x1008;
  *(undefined2 *)(param_1 + 0x1c) = 0x3aa8;
  *(undefined2 *)(param_1 + 0x1e) = 0x1008;
  *(undefined2 *)(param_1 + 0x20) = 0x0;
  puVar3 = pass1_1008_3e38((ushort *)CONCAT22(param_2,param_1 + 0x52));
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x9fb2;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  *(undefined2 *)(param_1 + 0x1c) = 0x9fca;
  *(undefined2 *)(param_1 + 0x1e) = 0x1008;
  PTR_LOOP_1050_4230 = param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x22),(WNDCLASS16 *)0x0,0x30);
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x1c0,puVar1,param_4);
  iStack4 = 0x0;
  do {
    paVar4 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iStack4 + 0x1c0,param_4);
    *(int *)(param_1 + iStack4 * 0x4 + 0x22) = (int)paVar4;
    *(int *)(param_1 + iStack4 * 0x4 + 0x24) = (int)((ulong)paVar4 >> 0x10);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0xc);
  uVar5 = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x22));
  uVar2 = (undefined2)(uVar5 >> 0x10);
  pass1_1008_3e76((ushort *)CONCAT22(param_2,param_1 + 0x52),0x0,(0x1e0 - *(int *)((int)uVar5 + 0x8)) / 0x2 - 0x32,
                  (0x280 - *(int *)((int)uVar5 + 0x4)) / 0x2);
  if (CONCAT22(param_2,param_1) == 0x0) {
    puVar1 = (uchar *)0x0;
    param_2 = (uchar *)0x0;
  }
  else {
    puVar1 = param_1 + 0x1c;
  }
  pass1_1008_9262((int)_PTR_LOOP_1050_0388,(ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10),0x50,CONCAT22(param_2,puVar1),
                  (uint)puVar1,param_2);
  return;
}



void pass1_1008_9fb2(ushort param_1,int param_2,ushort param_3,ushort param_4,uint param_5,byte param_6,byte param_7,
                    int param_8,int param_9,byte param_10)

{
  char *pcVar1;
  byte *pbVar2;
  byte bVar3;
  uint uVar4;
  code *pcVar5;
  byte bVar6;
  uint uVar7;
  uint uVar8;
  byte extraout_DL;
  uchar *puVar9;
  uchar *puVar10;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 uVar11;
  byte bVar12;
  bool bVar13;
  bool bVar14;
  astruct_79 *paVar15;
  
  *(undefined2 *)(param_8 + 0x1008) = (int)&USHORT_1050_1050;
  uVar8 = (uint)param_10;
  uVar4 = param_5 + 0xeff0;
  bVar12 = param_5 < 0x1010 || uVar4 < uVar8;
  uVar7 = uVar4 - uVar8;
  pcVar5 = (code *)swi(0x4);
  if (SBORROW2(param_5,0x1010) != SBORROW2(uVar4,uVar8)) {
    (*pcVar5)();
    param_7 = extraout_DL;
  }
  bVar6 = (byte)((char)(uVar7 + 0xeff0) - bVar12) % 0x1d;
  pcVar1 = (char *)(param_8 + param_9);
  *pcVar1 = *pcVar1 + param_7 + (uVar7 < 0x1010 || uVar7 + 0xeff0 < (uint)bVar12);
  pbVar2 = (byte *)(param_8 + param_9);
  bVar13 = *pbVar2 < param_7 || (byte)(*pbVar2 - param_7) < (0xb1 < bVar6);
  *pbVar2 = (*pbVar2 - param_7) - (0xb1 < bVar6);
  pbVar2 = (byte *)(param_8 + 0x18);
  bVar14 = *pbVar2 < param_6 || (byte)(*pbVar2 - param_6) < bVar13;
  *pbVar2 = (*pbVar2 - param_6) - bVar13;
  pbVar2 = (byte *)(param_8 + param_9 + 0x89f);
  bVar12 = *pbVar2;
  bVar3 = *pbVar2 + bVar6 + 0x4e;
  *pbVar2 = bVar3 + bVar14;
  pcVar1 = (char *)(param_8 + param_9);
  *pcVar1 = *pcVar1 + (char)param_8 + (CARRY1(bVar12,bVar6 + 0x4e) || CARRY1(bVar3,bVar14));
  pbVar2 = (byte *)(param_8 + param_9);
  *pbVar2 = *pbVar2 | param_7;
  paVar15 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_3,param_2),param_4);
  puVar9 = (uchar *)((ulong)paVar15 >> 0x10);
  uVar8 = 0x0;
  *(undefined4 *)(param_2 + 0xa) = 0x0;
  *(undefined4 *)(param_2 + 0x410) = 0x0;
  *(undefined2 *)(param_2 + 0x414) = 0x0;
  *(undefined2 *)(param_2 + 0x416) = 0x0;
  *(undefined2 *)(param_2 + 0x418) = 0x0;
  *(undefined2 *)(param_2 + 0x41a) = 0x0;
  *(undefined2 *)(param_2 + 0x41c) = 0x0;
  *(undefined2 *)(param_2 + 0x41e) = 0x0;
  *(undefined2 *)CONCAT22(param_3,param_2) = 0xad92;
  *(undefined2 *)(param_2 + 0x2) = 0x1008;
  mem_op_1000_179c(0xc,puVar9,0x1000);
  puVar10 = (uchar *)((uint)puVar9 | uVar8);
  if (puVar10 == (uchar *)0x0) {
    *(undefined4 *)(param_2 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar9,uVar8));
    *(uint *)(param_2 + 0xa) = uVar8;
    *(uchar **)(param_2 + 0xc) = extraout_DX;
    puVar10 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar10,0x1000);
  if (((uint)puVar10 | uVar8) == 0x0) {
    uVar8 = 0x0;
    uVar11 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar10,uVar8));
    uVar11 = extraout_DX_00;
  }
  *(uint *)(param_2 + 0x410) = uVar8;
  *(undefined2 *)(param_2 + 0x412) = uVar11;
  return;
}


void __stdcall16far struct_1008_9fd2(astruct_79 *param_1,astruct_79 *param_2,ushort param_3)

{
  uint uVar1;
  uchar *puVar2;
  uchar *puVar3;
  uchar *extraout_DX;
  ushort extraout_DX_00;
  ushort uVar4;
  astruct_79 *paVar5;
  
  paVar5 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar2 = (uchar *)((ulong)paVar5 >> 0x10);
  uVar1 = 0x0;
  *(undefined4 *)(param_1 + 0x1) = 0x0;
  *(undefined4 *)(param_1 + 0x68) = 0x0;
  *(undefined2 *)&param_1[0x68].field_0x4 = 0x0;
  *(undefined2 *)((int)&param_1[0x68].field_0x4 + 0x2) = 0x0;
  param_1[0x68].field_0x8 = 0x0;
  ((astruct_79 *)(param_1 + 0x69))->field_0x0 = 0x0;
  param_1[0x69].field_0x2 = 0x0;
  *(undefined2 *)&param_1[0x69].field_0x4 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xad92;
  param_1->field_0x2 = 0x1008;
  mem_op_1000_179c(0xc,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | uVar1);
  if (puVar3 == (uchar *)0x0) {
    *(undefined4 *)(param_1 + 0x1) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2,uVar1));
    ((astruct_79 *)(param_1 + 0x1))->field_0x0 = uVar1;
    param_1[0x1].field_0x2 = (ushort)extraout_DX;
    puVar3 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar3,0x1000);
  if (((uint)puVar3 | uVar1) == 0x0) {
    uVar1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar3,uVar1));
    uVar4 = extraout_DX_00;
  }
  ((astruct_79 *)(param_1 + 0x68))->field_0x0 = uVar1;
  param_1[0x68].field_0x2 = uVar4;
  return;
}

void __stdcall16far pass1_1008_8d8a(ulong param_1,astruct_76 *param_2,ulong param_3)

{
  uint uVar1;
  char cVar2;
  uint *puVar3;
  uchar *puVar4;
  uchar *puVar5;
  uint uVar6;
  astruct_112 *iVar7;
  undefined2 uVar7;
  ulong uVar8;
  astruct_110 *paStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar7 = (astruct_112 *)param_1;
  uVar1 = iVar7->field_0x2e;
  if ((int)uVar1 < 0x28) {
    if (((int)uVar1 < 0x25) && (uVar1 != 0x23)) {
      if (0x23 < uVar1) {
        return;
      }
      cVar2 = (char)uVar1;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else {
    if ((int)uVar1 < 0x46) {
      if ((int)uVar1 < 0x43) {
        if ((int)uVar1 < 0x33) {
          return;
        }
        if ((uVar1 != 0x34 && 0x0 < (int)(uVar1 - 0x33)) && (uVar1 != 0x37)) {
          return;
        }
      }
    }
    else {
      if (uVar1 != 0x49) {
        if ((int)(uVar1 - 0x49) < 0x2a) {
          return;
        }
        if (0x5 < (int)(uVar1 - 0x73)) {
          return;
        }
      }
    }
  }
  if (iVar7->field_0x3a == 0x0) {
    uVar8 = pass1_1008_4772(param_2);
    puVar4 = (uchar *)(uVar8 >> 0x10);
    uVar1 = (uint)uVar8;
    puVar5 = puVar4;
    uVar6 = uVar1;
    mem_op_1000_179c(0x14,puVar4,0x1000);
    paStack10 = (astruct_110 *)CONCAT22(puVar5,uVar6);
    uVar6 = (uint)puVar5 | uVar6;
    if (uVar6 == 0x0) {
      iVar7->field_0x3a = 0x0;
    }
    else {
      puVar3 = (uint *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar7->field_0x28);
      pass1_1008_50c2(paStack10,*(ulong *)(uVar1 + 0x8),*(ulong *)(uVar1 + 0x4),puVar3,param_3);
      *(int *)&iVar7->field_0x3a = (int)puVar3;
      *(uint *)((int)&iVar7->field_0x3a + 0x2) = uVar6;
    }
    pass1_1008_5134(iVar7->field_0x3a);
    return;
  }
  pass1_1008_5236(iVar7->field_0x3a);
  return;
}

void __stdcall16far pass1_1008_909c(ulong param_1,ushort param_2)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  astruct_106 *iVar5;
  undefined2 uVar4;
  long lVar5;
  long lStack10;
  undefined4 uStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_106 *)param_1;
  if (*(long *)&iVar5->field_0x12 == 0x0) {
    uVar3 = iVar5->field_0xe;
    PTR_LOOP_1050_5f2e = iVar5->field_0x10;
  }
  else {
    uVar2 = *(uint *)&iVar5->field_0x12;
    puVar1 = &iVar5->field_0x16;
    uVar3 = uVar2 + *puVar1;
    PTR_LOOP_1050_5f2e = (undefined *)(iVar5->field_0x14 + iVar5->field_0x18 + (uint)CARRY2(uVar2,*puVar1));
  }
  uStack6 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if (iVar5->field_0x6 == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(uVar3 << 0x2,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    lVar5 = iVar5->field_0x6;
    lVar5 = pass1_1000_0ed4(0x1000,param_2,0x1,uVar3 * 0x4,
                            ((int)PTR_LOOP_1050_5f2e * 0x2 + (uint)CARRY2(uVar3,uVar3)) * 0x2 +
                            (uint)CARRY2(uVar3 * 0x2,uVar3 * 0x2),(ushort *)lVar5,(ushort)((ulong)lVar5 >> 0x10));
    PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar5 >> 0x10);
    uVar3 = (uint)lVar5;
  }
  lStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if (((uint)PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
    *(undefined4 *)&iVar5->field_0x12 = uStack6;
    iVar5->field_0x6 = lStack10;
  }
  return;
}

void __stdcall16far pass1_1008_9262(int param_1,ushort param_2,ulong param_3,ulong param_4,uint param_5,uchar *param_6)

{
  code **ppcVar1;
  uint uVar2;
  
  mem_op_1000_179c(0x12,param_6,0x1000);
  uVar2 = (uint)param_6 | param_5;
  if (uVar2 == 0x0) {
    param_5 = 0x0;
    uVar2 = 0x0;
  }
  else {
    struct_op_1008_9174((astruct_88 *)CONCAT22(param_6,param_5),param_3,param_4);
  }
  if ((uVar2 | param_5) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)(param_1 + 0x6) + 0x4);
    (**ppcVar1)();
  }
  return;
}


ushort __stdcall16far
pass1_1008_7e4a(ushort param_1,uchar *param_2,uchar param_3,char *param_4,ushort param_5,uint16_t param_6)

{
  uint uVar1;
  
  sys_1000_3f9c((uchar *)&param_5,param_2,0x347,(ushort)&USHORT_1050_1050,(ushort)_PTR_s_dcbSC_1050_0336_1050_033c,
                &stack0xfffe,param_1,0x1000,param_2,param_3);
  uVar1 = str_op_1000_3da4((char *)CONCAT22(param_2,&param_5));
  uVar1 = pass1_1000_3de8(param_4,(char *)CONCAT22(param_2,&param_5),uVar1,param_5,param_6);
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}



ushort * __stdcall16far pass1_1008_7e98(ushort *param_1,byte param_2)

{
  astruct_460 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_460 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


astruct_20 * __stdcall16far unk_draw_op_1008_7f62(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 HVar2;
  astruct_20 *uVar4;
  astruct_20 *iVar3;
  
  iVar3 = (astruct_20 *)param_1;
  uVar4 = (astruct_20 *)((ulong)param_1 >> 0x10);
  set_struct_1008_687a(param_1,param_3);
  iVar3->field_0xde = param_2;
  param_1->field_0x0 = 0x8042;
  iVar3->field_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x5b)),s_SOLChildPar_1050_0358);
  HVar1 = GetStockObject16(0x1000);
  iVar3->hgdiobj_field_0xc6 = HVar1;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar3->hcursor_field_0xc4 = HVar2;
  iVar3->field_0xc8 = 0x2008;
  iVar3->field_0xac = 0x44000000;
  iVar3->field_0xbc = *(UINT16 *)((int)param_3 + 0x8);
  iVar3->field_0xca = iVar3->field_0xde;
  win_ui_reg_class_1008_96d2(param_1,(int)s_tile2_bmp_1050_1538,param_4);
  return param_1;
}


void __stdcall16far memcpy_op_1008_676e(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 uVar1;
  long lVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x6) == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_3,0x1000);
  uVar3 = (uint)param_3 | param_2;
  if (uVar3 == 0x0) {
    param_2 = 0x0;
    uVar3 = 0x0;
  }
  else {
    uVar1 = *(undefined4 *)(iVar4 + 0x10);
    uVar7 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar5 = (int)uVar1;
    struct_op_1008_6604((astruct_85 *)CONCAT22(param_3,param_2),*(undefined2 *)(iVar5 + 0x8),
                        *(undefined2 *)(iVar5 + 0x4));
  }
  pass1_1000_48a8(*(ulong *)(param_2 + 0x10),*(ulong *)(iVar4 + 0x10),0x28);
  uVar1 = *(undefined4 *)(param_2 + 0x10);
  lVar2 = *(long *)((int)uVar1 + 0x8) * *(long *)(iVar4 + 0x18);
  hmemcpy16((LPVOID)&PTR_LOOP_1050_1000,(LPCVOID)lVar2,
            CONCAT22((int)*(undefined4 *)(iVar4 + 0x6),(int)((ulong)lVar2 >> 0x10)));
  *(undefined2 *)(param_2 + 0x1c) = 0x1;
  return;
}


void __stdcall16far pass1_1008_6978(ulong param_1,int param_2,ulong param_3,uint param_4,uchar *param_5)

{
  code **ppcVar1;
  undefined2 *puStack10;
  undefined2 *puStack6;
  
  mem_op_1000_179c(0xa,param_5,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_5,param_4);
  if (((uint)param_5 | param_4) == 0x0) {
    puStack6 = (undefined2 *)0x0;
  }
  else {
    if (param_2 == 0x0) {
      param_2 = *(int *)((int)param_1 + 0xcc);
    }
    *puStack10 = 0x389a;
    *(undefined2 *)(param_4 + 0x2) = 0x1008;
    *(ulong *)(param_4 + 0x4) = param_3;
    *(int *)(param_4 + 0x8) = param_2;
    *puStack10 = 0x6c8c;
    *(undefined2 *)(param_4 + 0x2) = 0x1008;
    puStack6 = puStack10;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)((int)param_1 + 0xd2) + 0x4);
  (**ppcVar1)(0x1000,(undefined4 *)((int)param_1 + 0xd2),param_1._2_2_,puStack6);
  return;
}

int __stdcall16far
pass1_1008_7006(ushort param_1,ushort param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if ((int)PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar2 = (undefined4 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,*(ushort *)(iStack4 * 0x2 + 0x320),param_6,param_4,param_5);
    ppcVar1 = (code **)((int)*puVar2 + 0x8);
    uVar3 = (**ppcVar1)(0x1010,puVar2,param_3);
    param_4 = (uchar *)((ulong)uVar3 >> 0x10);
    if ((int)uVar3 == 0x0) break;
    iStack4 = iStack4 + 0x1;
  }
  return (int)uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far
pass1_1008_7056(ushort param_1,ushort param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if ((int)PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar2 = (undefined4 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,*(ushort *)(iStack4 * 0x2 + 0x320),param_6,param_4,param_5);
    ppcVar1 = (code **)((int)*puVar2 + 0xc);
    uVar3 = (**ppcVar1)(0x1010,puVar2,param_3);
    param_4 = (uchar *)((ulong)uVar3 >> 0x10);
    if ((int)uVar3 == 0x0) break;
    iStack4 = iStack4 + 0x1;
  }
  return (int)uVar3;
}


void __stdcall16far pass1_1008_5bdc(astruct_79 *param_1,int param_2,ushort param_3)

{
  astruct_651 *puVar1;
  undefined2 uVar1;
  astruct_79 *paVar2;
  ushort *puVar3;
  
  paVar2 = struct_op_1010_1d48(param_1,0x44);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  puVar1 = (astruct_651 *)param_1;
  puVar1->field_0xa = 0x0;
  *(undefined4 *)&puVar1->field_0xc = 0x0;
  puVar1->field_0x10 = 0x0;
  puVar1->field_0x12 = 0x0;
  param_1->field_0x0 = 0x5fc8;
  puVar1->field_0x2 = 0x1008;
  _PTR_LOOP_1050_02a0 = param_1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,(uchar *)((ulong)paVar2 >> 0x10),param_2);
  puVar1->field_0xc = (int)puVar3;
  puVar1->field_0xe = (int)((ulong)puVar3 >> 0x10);
  return;
}

uchar * __cdecl16far pass1_1008_5fd8(ushort param_1,uchar *param_2)

{
  int *piVar1;
  uchar *puVar2;
  char *pcVar3;
  int iStack6;
  
  puVar2 = &stack0x0006;
  _iStack6 = (int *)CONCAT22(param_1,puVar2);
  mem_op_1000_179c(0x1000,param_2,0x1000);
  pcVar3 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_2,puVar2),pcVar3);
  while( true ) {
    piVar1 = _iStack6;
    _iStack6 = (int *)((ulong)_iStack6 & 0xffff0000 | (ulong)(iStack6 + 0x2));
    if (*piVar1 == 0x0) break;
    pcVar3 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    pass1_1000_3cea(CONCAT22(param_2,puVar2),(ULONG)pcVar3);
  }
  return puVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far debug_print_1008_6048(ulong param_1,LPSTR param_2,WORD *param_3)

{
  uint uVar1;
  uint in_DX;
  ushort unaff_ES;
  uchar in_AF;
  WORD *args;
  
  if (PTR_LOOP_1050_02ec != (undefined *)0x0) {
    args = param_3;
    if (DAT_1050_02ee == 0xffff) {
      param_2 = (LPSTR)&PTR_LOOP_1050_1000;
      uVar1 = pass1_1000_3ec0(0x2f4,(int)&USHORT_1050_1050);
      DAT_1050_02ee = (uint)((in_DX | uVar1) != 0x0);
    }
    if (DAT_1050_02ee != 0x0) {
      wvsprintf16(param_2,&stack0x0008,args);
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      if (_PTR_LOOP_1050_02f0 != 0x0) {
        pass1_1000_2b5c((ushort)_PTR_LOOP_1050_02f0,(ushort)((ulong)_PTR_LOOP_1050_02f0 >> 0x10),0x2fd,
                        (ushort)&USHORT_1050_1050,unaff_ES,(int)&stack0xfffe,0x1000,(ushort)param_3);
        pass1_1000_2f48(_PTR_LOOP_1050_02f0,(int)&stack0xfffe,unaff_ES,0x1000,(ushort)param_3,in_AF);
      }
    }
  }
  return;
}



void __stdcall16far pass1_1008_64c8(ulong *param_1,ulong param_2,int param_3,uint param_4,uchar *param_5)

{
  int iVar1;
  int iVar2;
  undefined2 extraout_DX;
  undefined2 uVar3;
  int iVar4;
  int iVar5;
  int iStack8;
  ulong uStack6;
  
  if (*param_1 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_5,0x1000);
  if (((uint)param_5 | param_4) == 0x0) {
    param_4 = 0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1008_6604((astruct_85 *)CONCAT22(param_5,param_4),(int)param_2,(int)(param_2 >> 0x10));
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_4);
  iStack8 = 0x0;
  while (param_2 = param_2 & 0xffff0000 | (ulong)((int)param_2 - 0x1), (int)param_2 != 0x0) {
    iVar1 = param_3 + 0x1;
    iVar4 = param_3 >> 0xf;
    pass1_1008_4544(*param_1);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(uStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_3),param_2._2_2_);
    param_3 = iVar1;
    iStack8 = iVar2;
  }
  return;
}



void __stdcall16far pass1_1008_6562(ulong *param_1,ulong param_2,int param_3,uint param_4,uchar *param_5)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iStack8;
  ulong uStack6;
  
  if (*param_1 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_5,0x1000);
  uVar3 = (uint)param_5 | param_4;
  if (uVar3 == 0x0) {
    param_4 = 0x0;
    uVar3 = 0x0;
  }
  else {
    pass1_1008_405c((astruct_76 *)CONCAT22(param_5,param_4),*(ulong *)((int)param_1 + 0x4),(int)param_2,
                    (int)(param_2 >> 0x10));
  }
  uStack6 = CONCAT22(uVar3,param_4);
  iStack8 = 0x0;
  while (param_2 = param_2 & 0xffff0000 | (ulong)((int)param_2 - 0x1), (int)param_2 != 0x0) {
    iVar1 = param_3 + 0x1;
    iVar4 = param_3 >> 0xf;
    pass1_1008_4544(*param_1);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(uStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_3),param_2._2_2_);
    param_3 = iVar1;
    iStack8 = iVar2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_op_1008_6604(astruct_85 *param_1,int param_2,int param_3)

{
  undefined4 *puVar1;
  int iVar3;
  astruct_85 *iVar4;
  astruct_84 *iVar2;
  undefined2 uVar4;
  undefined2 uVar5;
  long lVar6;
  
  pass1_1008_4016((astruct_76 *)param_1);
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_85 *)param_1;
  *(undefined2 *)param_1 = 0x685a;
  iVar4->field_0x2 = 0x1008;
  lVar6 = mem_op_1000_0a48(0x1,0x28,0x0,_PTR_LOOP_1050_5f2c,0x1000);
  *(int *)&iVar4->field_0x10 = (int)lVar6;
  *(undefined2 *)((int)&iVar4->field_0x10 + 0x2) = (int)((ulong)lVar6 >> 0x10);
  iVar3 = param_3 * 0x8 + 0x1f;
  iVar3 = ((int)(iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  *(int *)&iVar4->field_0x18 = iVar3;
  *(int *)((int)&iVar4->field_0x18 + 0x2) = iVar3 >> 0xf;
  lVar6 = mem_op_1000_0a48(0x1,(uint)((long)iVar3 * (long)param_2),(int)((ulong)((long)iVar3 * (long)param_2) >> 0x10),
                           _PTR_LOOP_1050_5f2c,0x1000);
  uVar5 = (undefined2)((ulong)lVar6 >> 0x10);
  iVar4->field_0x6 = (int)lVar6;
  iVar4->field_0x8 = uVar5;
  iVar4->field_0x14 = iVar4->field_0x6;
  iVar4->field_0x16 = uVar5;
  *iVar4->field_0x10 = 0x28;
  puVar1 = iVar4->field_0x10;
  *(long *)((int)puVar1 + 0x4) = (long)param_3;
  puVar1 = iVar4->field_0x10;
  uVar5 = (undefined2)((ulong)puVar1 >> 0x10);
  iVar2 = (astruct_84 *)puVar1;
  iVar2->field_0x8 = param_2;
  iVar2->field_0xa = param_2 >> 0xf;
  puVar1 = iVar4->field_0x10;
  *(undefined2 *)((int)puVar1 + 0xc) = 0x1;
  puVar1 = iVar4->field_0x10;
  *(undefined2 *)((int)puVar1 + 0xe) = 0x8;
  puVar1 = iVar4->field_0x10;
  *(undefined4 *)((int)puVar1 + 0x10) = 0x0;
  puVar1 = iVar4->field_0x10;
  *(long *)((int)puVar1 + 0x14) = iVar4->field_0x18 * (long)param_2;
  puVar1 = iVar4->field_0x10;
  *(undefined4 *)((int)puVar1 + 0x20) = 0x100;
  puVar1 = iVar4->field_0x10;
  *(undefined4 *)((int)puVar1 + 0x24) = 0x100;
  return;
}


void __stdcall16far pass1_1008_4b8e(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ulong uVar1;
  undefined2 uVar2;
  ushort *puVar3;
  int iStack18;
  int iStack16;
  int iStack10;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,param_2,param_3);
  uVar2 = (undefined2)((ulong)puVar3 >> 0x10);
  uVar1 = *(ulong *)((int)puVar3 + 0x18);
  iStack18 = *(int *)((int)puVar3 + 0x16) / 0x2;
  for (iStack16 = 0x0; iStack10 = (int)uVar1, uVar2 = (undefined2)(param_1 >> 0x10), iStack16 < iStack18;
      iStack16 = iStack16 + 0x1) {
    pass1_1008_4d26(*(ulong *)((int)param_1 + 0x4),
                    (ushort *)(uVar1 & 0xffff0000 | (ulong)(uint)(iStack16 * 0x4 + iStack10)),iStack16);
  }
  for (iStack18 = 0x100 - iStack18; iStack18 < 0x100; iStack18 = iStack18 + 0x1) {
    pass1_1008_4d26(*(ulong *)((int)param_1 + 0x4),
                    (ushort *)(uVar1 & 0xffff0000 | (ulong)(uint)(iStack16 * 0x4 + iStack10)),iStack18);
    iStack16 = iStack16 + 0x1;
  }
  return;
}


void __stdcall16far pass1_1008_4d84(astruct_90 *param_1,ulong param_2,uchar *param_3)

{
  ushort uVar1;
  astruct_90 *iVar3;
  undefined2 uVar2;
  undefined2 uVar3;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_90 *)param_1;
  uVar3 = (undefined2)(param_2 >> 0x10);
  if (iVar3->field_0x12 != 0x0) {
    iVar3->field_0xc = *(int *)((int)param_2 + 0xc);
    fn_ptr_1000_17ce((astruct_18 *)iVar3->field_0x4,0x1000);
    iVar3->field_0x4 = 0x0;
    uVar1 = iVar3->field_0xc << 0x2;
    mem_op_1000_179c(uVar1,param_3,0x1000);
    *(ushort *)&iVar3->field_0x4 = uVar1;
    *(uchar **)((int)&iVar3->field_0x4 + 0x2) = param_3;
  }
  if (iVar3->field_0xc != 0x100) {
    return;
  }
  pass1_1000_48a8(iVar3->field_0x4,*(ulong *)((int)param_2 + 0x4),0x400);
  return;
}


void __stdcall16far pass1_1008_405c(astruct_76 *param_1,ulong param_2,int param_3,int param_4)

{
  undefined4 uVar1;
  sqword sVar2;
  int iVar3;
  long lVar4;
  uchar *puVar5;
  astruct_76 *iVar4;
  uint uVar6;
  ulong uStack10;
  
  struct_op_1008_56b4(param_1);
  uVar6 = (uint)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_76 *)param_1;
  *(undefined4 *)&iVar4->field_0x6 = 0x0;
  *(undefined4 *)((int)&iVar4->field_0x8 + 0x2) = 0x0;
  *(undefined2 *)&iVar4->field_0xe = 0x0;
  *(undefined4 *)((int)&iVar4->field_0xe + 0x2) = 0x0;
  iVar4->field_0x14 = 0x0;
  *(undefined4 *)&iVar4->field_0x18 = 0x0;
  iVar4->field_0x1c = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_48de;
  iVar4->field_0x2 = 0x1008;
  iVar3 = param_4 * 0x8 + 0x1f;
  iVar3 = ((int)(iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  uStack10 = SEXT24(param_3);
  lVar4 = (long)iVar3 * (long)param_3 + 0x436;
  lVar4 = mem_op_1000_0a48(0x1,(uint)lVar4,(int)((ulong)lVar4 >> 0x10),_PTR_LOOP_1050_5f2c,0x1000);
  iVar4->field_0x6 = (int)lVar4;
  *(int *)&iVar4->field_0x8 = (int)((ulong)lVar4 >> 0x10);
  pass1_1008_47cc((astruct_76 *)((ulong)param_1 & 0xffff | (ulong)uVar6 << 0x10));
  *(int *)&iVar4->field_0x18 = iVar3;
  iVar4->field_0x1a = iVar3 >> 0xf;
  *(undefined4 *)*(undefined4 *)((int)&iVar4->field_0xe + 0x2) = 0x28;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(long *)((int)uVar1 + 0x4) = (long)param_4;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(ulong *)((int)uVar1 + 0x8) = uStack10;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined2 *)((int)uVar1 + 0xc) = 0x1;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined2 *)((int)uVar1 + 0xe) = 0x8;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x10) = 0x0;
  sVar2 = (qword)*(ulong *)&iVar4->field_0x18 * (qword)uStack10;
  puVar5 = (uchar *)((qword)sVar2 >> 0x20);
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x14) = (long)sVar2;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x20) = 0x100;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x24) = 0x100;
  pass1_1008_4834(param_1);
  pass1_1008_4d84(*(astruct_90 **)((int)&iVar4->field_0x8 + 0x2),param_2,puVar5);
  return;
}


void __stdcall16far memcpy_op_1008_4274(ulong param_1,undefined2 param_2)

{
  int iVar1;
  uchar *puVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  undefined2 uVar6;
  ulong uVar7;
  long lVar8;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x6) != 0x0) {
    uVar7 = pass1_1000_1284(*(ulong *)(iVar4 + 0x6),0x1000);
    iVar1 = (int)(uVar7 >> 0x10);
    lVar8 = mem_op_1000_0a48(0x1,(uint)(LPCVOID)uVar7,iVar1,_PTR_LOOP_1050_5f2c,0x1000);
    uVar5 = (uint)lVar8;
    puVar2 = (uchar *)((uint)((ulong)lVar8 >> 0x10) | uVar5);
    if (puVar2 != (uchar *)0x0) {
      hmemcpy16((LPVOID)&PTR_LOOP_1050_1000,(LPCVOID)uVar7,CONCAT22((int)*(undefined4 *)(iVar4 + 0x6),iVar1));
      mem_op_1000_179c(0x1e,puVar2,0x1000);
      uVar3 = (uint)puVar2 | uVar5;
      if (uVar3 == 0x0) {
        uVar5 = 0x0;
        uVar3 = 0x0;
      }
      else {
        pass1_1008_4016((astruct_76 *)CONCAT22(puVar2,uVar5));
      }
      *(long *)(uVar5 + 0x6) = lVar8;
      pass1_1008_47cc((astruct_76 *)CONCAT22(uVar3,uVar5));
      pass1_1008_4834((astruct_76 *)CONCAT22(uVar3,uVar5));
      *(undefined2 *)(uVar5 + 0x1c) = 0x1;
      return;
    }
  }
  return;
}


void __stdcall16far pass1_1008_4834(astruct_76 *param_1)

{
  code **ppcVar1;
  undefined4 *puVar2;
  ulong uVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  ushort extraout_DX_00;
  astruct_76 *struct_var5_1;
  astruct_76 *struct_var5;
  astruct_76 *paStack10;
  
  struct_var5 = (astruct_76 *)((ulong)param_1 >> 0x10);
  struct_var5_1 = (astruct_76 *)param_1;
  puVar2 = (undefined4 *)*(uint *)((int)&struct_var5_1->field_0x8 + 0x2);
  puVar4 = (uchar *)struct_var5_1->field_0xc;
  if (((uint)puVar4 | (uint)puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
    puVar4 = extraout_DX;
  }
  mem_op_1000_179c(0x14,puVar4,0x1000);
  paStack10 = (astruct_76 *)CONCAT22(puVar4,puVar2);
  if (((uint)puVar4 | (uint)puVar2) != 0x0) {
    uVar3 = *(ulong *)((int)&struct_var5_1->field_0xe + 0x2);
    uVar3 = uVar3 & 0xffff0000 | (ulong)((int)uVar3 + 0x28);
    struct_op_1008_4c98(paStack10,uVar3,0x100);
    *(undefined2 *)((int)&struct_var5_1->field_0x8 + 0x2) = (int)uVar3;
    struct_var5_1->field_0xc = extraout_DX_00;
    return;
  }
  *(undefined4 *)((int)&struct_var5_1->field_0x8 + 0x2) = 0x0;
  return;
}


int __stdcall16far win_ui_op_1008_2b54(uint param_1,uchar *param_2,ushort param_3)

{
  uint uVar1;
  code **ppcVar2;
  int iVar3;
  uchar *puVar4;
  HWND16 hwnd;
  char *pcVar5;
  undefined2 uVar6;
  undefined4 *local_a6 [0x14];
  undefined local_56 [0x50];
  int iStack6;
  int iStack4;
  
  iStack4 = 0x0;
  if (_PTR_LOOP_1050_4230 == 0x0) {
    pcVar5 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_56),pcVar5);
    pcVar5 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_a6),pcVar5);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    iStack4 = MessageBox16(0x1000,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x1),(LPCSTR)local_a6,param_3);
  }
  else {
    uVar6 = 0xb4;
    hwnd = 0x1000;
    mem_op_1000_179c(0xb4,param_2,0x1000);
    puVar4 = (uchar *)((uint)param_2 | param_1);
    if (puVar4 == (uchar *)0x0) {
      iVar3 = 0x0;
      puVar4 = (uchar *)0x0;
    }
    else {
      hwnd = (HWND16)&PTR_LOOP_1050_1040;
      iVar3 = string_1040_8520((astruct_57 *)CONCAT22(param_2,param_1),(ushort)PTR_LOOP_1050_0396,0x21,0x2,0x57b,0x5f2,
                               puVar4,param_3);
    }
    local_a6[0] = (undefined4 *)CONCAT22(puVar4,iVar3);
    ppcVar2 = (code **)((int)*local_a6[0] + 0x74);
    iStack4 = (**ppcVar2)(hwnd,iVar3,puVar4,uVar6,param_1);
  }
  iStack6 = iStack4;
  if (iStack4 != 0x1) {
    iStack6 = 0x0;
  }
  if (((iStack6 != 0x0) && (_PTR_LOOP_1050_5748 != 0x0)) &&
     (uVar1 = *(uint *)((int)_PTR_LOOP_1050_5748 + 0x8),
     local_a6[0] = (undefined4 *)((ulong)local_a6[0] & 0xffff0000 | (ulong)uVar1), uVar1 != 0x0)) {
    PostMessage16(hwnd,0x0,0x0,0x11100b4);
    iStack6 = 0x0;
  }
  return iStack6;
}


void __stdcall16far pass1_1008_30cc(ulong param_1,uint param_2,uint param_3,int param_4,ushort param_5)

{
  uchar *puVar1;
  ushort *puVar2;
  undefined *puVar3;
  ushort uVar4;
  char local_210 [0xa];
  undefined local_206 [0x100];
  uint uStack262;
  uint uStack260;
  char local_102 [0x100];
  
  local_102[0] = '\0';
  save_file_1008_3178(param_1,0x2,param_5);
  puVar1 = (uchar *)(param_3 | param_2);
  if (puVar1 != (uchar *)0x0) {
    uStack262 = param_2;
    uStack260 = param_3;
    unk_str_op_1000_3d3e((char *)CONCAT22(param_5,local_102),(char *)CONCAT22(param_3,param_2));
    str_1000_4d58((char *)CONCAT22(param_5,local_102),(char *)0x0,0x0,CONCAT22(param_5,local_206),
                  (WNDCLASS16 *)CONCAT22(param_5,local_210));
    if (local_210[0] != '\0') {
      pass1_1000_3cea(CONCAT22(param_5,local_206),CONCAT22(param_5,local_210));
    }
    puVar3 = local_206;
    uVar4 = param_5;
    puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,puVar1,param_4);
    pass1_1010_5f4c((ulong)puVar2,(char *)CONCAT22(uVar4,puVar3),(ushort)((ulong)puVar2 >> 0x10));
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc(param_1,CONCAT22(param_5,local_102),0x1010,param_5);
    }
  }
  return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_087e(uint param_1,uchar *param_2,ushort param_3,uchar param_4)

{
  ushort uVar1;
  undefined2 uVar2;
  ulong uVar3;
  undefined2 local_112;
  undefined2 uStack272;
  uint uStack6;
  uchar *puStack4;
  
  uVar2 = 0x1000;
  mem_op_1000_179c(0xa,param_2,0x1000);
  uVar1 = (uint)param_2 | param_1;
  uStack6 = param_1;
  puStack4 = param_2;
  if (uVar1 != 0x0) {
    uVar2 = 0x1030;
    struct_1030_8128((ulong *)CONCAT22(param_2,param_1),uVar1,param_3);
  }
  if (_PTR_LOOP_1050_5748 == (ulong **)0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__Simulator_1050_0130,uVar2,param_3);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,uVar1,0x8);
  pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)(uVar3 >> 0x10),0x8);
  pass1_1030_532e((astruct_100 *)CONCAT22(param_3,&local_112),0xff000000,param_3,param_4);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,&local_112));
  pass1_1030_838e((ulong *)_PTR_LOOP_1050_5748,param_3,param_4);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334(_PTR_LOOP_1050_5748);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1008_0932(void)

{
  ulong uVar1;
  
  if (_PTR_LOOP_1050_14cc != 0x0) {
    pass1_1010_7fd6(_PTR_LOOP_1050_14cc);
  }
  mem_1000_0016(_PTR_LOOP_1050_03a0,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_029c,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_4fb8,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_68a2,0x1000);
  mem_1000_0016(_PTR_LOOP_1050_5744,0x1000);
  uVar1 = mem_1000_0016(_PTR_LOOP_1050_5f2c,0x1000);
  return uVar1;
}


uint __cdecl16far dos3_call_1000_51aa(ushort param_1)

{
  code *pcVar1;
  uint uVar2;
  byte bVar3;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&USHORT_1050_1050,param_1 + 0x1);
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  bVar3 = 0x0;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  if ((bVar3 & 0x1) == 0x0) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return uVar2 & 0xff;
}



int * pass1_1000_55b1(int param_1,uint16_t param_2,uint16_t param_3)

{
  int *piVar2;
  char *pcVar3;
  LPCSTR str;
  int *piVar4;
  int *piVar5;
  char *pcVar6;
  int iVar7;
  int iVar8;
  char *piVar1;
  
  iVar8 = 0x14;
  iVar7 = 0x14;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar7,param_2,param_3);
  str = poss_str_op_1000_28dc(iVar8);
  if (str != (PCHAR)0x0) {
    iVar7 = 0x9;
    if (*str == 'M') {
      iVar7 = 0xf;
    }
    str = str + iVar7;
    iVar7 = 0x22;
    pcVar6 = str;
    do {
      if (iVar7 == 0x0) break;
      iVar7 = iVar7 + -0x1;
      pcVar3 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar6[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar5 = (int *)&PTR_LOOP_1050_63fe;
  do {
    piVar2 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar7 = *piVar2;
    piVar4 = piVar5;
    if ((iVar7 == param_1) || (piVar4 = (int *)(iVar7 + 0x1), piVar4 == (int *)0x0)) {
      return piVar4;
    }
    iVar7 = -0x1;
    do {
      if (iVar7 == 0x0) break;
      iVar7 = iVar7 + -0x1;
      piVar1 = (char *)piVar5;
      piVar5 = (int *)((int)piVar5 + 0x1);
    } while (*piVar1 != '\0');
  } while( true );
}


