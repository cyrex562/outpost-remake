
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far string_1010_5286(ushort param_1,ushort param_2,ulong param_3,char *param_4,uint param_5)

{
  char *in_buffer_4;
  uchar *in_buf_len_5;
  char *pcVar1;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_3,(uint)(param_3 >> 0x10));
  in_buf_len_5 = (uchar *)(param_5 | (uint)param_4);
  if (in_buf_len_5 == (uchar *)0x0) {
    return 0x0;
  }
  in_buffer_4 = param_4;
  mem_op_1000_179c(0x80,in_buf_len_5,0x1000);
  load_string_1010_84e0
            (0x1000,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x80,in_buffer_4,
             (short)in_buf_len_5);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4),0x105013ac);
  pcVar1 = pass1_1038_4d28(CONCAT22(param_5,param_4));
  pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4),(ULONG)pcVar1);
  return CONCAT22(in_buf_len_5,in_buffer_4);
}



void __stdcall16far pass1_1010_52fc(ulong param_1,ulong param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  undefined2 uVar1;
  
  pass1_1010_533c(param_1,param_2,param_4,param_5);
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x12) = param_3;
  *(uchar **)((int)param_1 + 0x14) = param_4;
  return;
}



void __stdcall16far pass1_1010_531c(ulong param_1,ulong param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  undefined2 uVar1;
  
  pass1_1010_533c(param_1,param_2,param_4,param_5);
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x16) = param_3;
  *(uchar **)((int)param_1 + 0x18) = param_4;
  return;
}



void __stdcall16far pass1_1010_533c(ulong param_1,ulong param_2,uchar *param_3,ushort param_4)

{
  uint *puVar1;
  ulong uVar2;
  undefined4 uVar3;
  int iVar4;
  ushort uVar5;
  ushort uVar6;
  astruct_18 *paVar7;
  uint uStack6;
  undefined local_4 [0x2];
  
  pass1_1010_519a(param_1,(int *)CONCAT22(param_4,local_4),param_3,param_4);
  uStack6 = 0x0;
  while( true ) {
    uVar6 = (ushort)(param_1 >> 0x10);
    uVar5 = (ushort)param_1;
    puVar1 = (uint *)(uVar5 + 0x10);
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      return;
    }
    uVar3 = *(undefined4 *)(uVar5 + 0xc);
    uVar2 = *(ulong *)((int)uVar3 + uStack6 * 0x4);
    paVar7 = (astruct_18 *)string_1010_5286(uVar5,uVar6,uVar2,(char *)uVar2,(uint)param_3);
    param_3 = (uchar *)((ulong)paVar7 >> 0x10);
    iVar4 = pass1_1000_3d7a(param_2,(ulong)paVar7 & 0xffff | ZEXT24(param_3) << 0x10);
    if (iVar4 == 0x0) break;
    fn_ptr_1000_17ce(paVar7,0x1000);
    uStack6 = uStack6 + 0x1;
  }
  return;
}



ushort * __stdcall16far pass1_1010_53ce(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_50f2(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_sys_op_1010_5404(astruct_54 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int *piVar1;
  uint **ppuVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  code **ppcVar5;
  LPCSTR pCVar6;
  int iVar7;
  ushort uVar8;
  uint *puVar9;
  uint uVar10;
  uchar *puVar11;
  uchar *extraout_DX;
  uchar *puVar12;
  uchar *extraout_DX_00;
  uchar *extraout_DX_01;
  undefined *puVar13;
  undefined2 *puVar14;
  int unaff_DI;
  undefined2 uVar15;
  LPCSTR pCVar16;
  INT16 index;
  astruct_79 *paVar17;
  char *pcVar18;
  ushort *puVar19;
  undefined2 uVar20;
  undefined local_134 [0x102];
  undefined2 *puStack50;
  ushort uStack46;
  uchar *puStack44;
  int iStack42;
  int iStack26;
  uchar *puStack24;
  int iStack22;
  undefined2 *puStack20;
  undefined4 uStack16;
  int iStack12;
  int iStack10;
  undefined2 uStack8;
  uchar *puStack6;
  uint uStack4;
  
  paVar17 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar12 = (uchar *)((ulong)paVar17 >> 0x10);
  uVar15 = 0x0;
  *(undefined4 *)&param_1->field_0xa = 0x0;
  param_1->field_0xe = (char *)0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  *(undefined4 *)&param_1->field_0x1a = 0x0;
  param_1->field_0x62 = 0x0;
  param_1->field_0x64 = (undefined4 *)0x0;
  *(undefined4 *)&param_1->field_0x68 = 0x0;
  *(undefined4 *)&param_1->field_0x6c = 0x0;
  param_1->field_0x70 = 0x1;
  param_1->field_0x7a = 0x0;
  param_1->field_0x7c = 0x0;
  param_1->field_0x7e = 0x0;
  param_1->field_0x80 = 0x0;
  param_1->field_0x82 = 0x1;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x6312;
  param_1->field_0x2 = 0x1010;
  pass1_1010_6034(CONCAT22(param_2,param_1),(ushort)puVar12);
  mem_op_1000_179c(0x101,puVar12,0x1000);
  *(undefined2 *)&param_1->field_0xe = uVar15;
  *(uchar **)((int)&param_1->field_0xe + 0x2) = puVar12;
  pass1_1000_5008(*(ushort *)&param_1->field_0xe,(ushort)puVar12,0x100,(int)&stack0xfffe);
  uStack4 = str_op_1000_3da4(param_1->field_0xe);
  pcVar18 = param_1->field_0xe;
  uVar15 = (undefined2)((ulong)pcVar18 >> 0x10);
  puVar13 = (undefined *)((int)pcVar18 + uStack4);
  if (puVar13[-0x1] != '\\') {
    *puVar13 = 0x5c;
    pcVar18 = param_1->field_0xe;
    *(undefined *)((int)pcVar18 + uStack4 + 0x1) = 0x0;
  }
  pcVar18 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1000);
  puVar12 = (uchar *)((ulong)pcVar18 >> 0x10);
  uStack8 = SUB42(pcVar18,0x0);
  puStack6 = puVar12;
  pass1_1000_3cea((ULONG)param_1->field_0xe,(ULONG)pcVar18);
  pCVar6 = (LPCSTR)str_op_1008_60e8(param_1->field_0xe,(ushort)puVar12);
  param_1->field_0xa = pCVar6;
  param_1->field_0xc = puVar12;
  pcVar18 = param_1->field_0xe;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            ((LPCSTR)0x1008,param_1->field_0xa,(LPCSTR)puVar12,
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    iStack22 = pass1_1000_3e2c((ulong)param_1->field_0xe);
    puVar19 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar12,unaff_DI);
    puVar12 = (uchar *)((ulong)puVar19 >> 0x10);
    iStack26 = (int)puVar19;
    iStack10 = *(int *)(iStack26 + 0xa);
    iStack12 = *(int *)(iStack26 + 0xc);
    param_1->field_0x62 = (uint)(iStack22 != iStack10);
    puStack24 = puVar12;
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e((ulong)param_1->field_0xe,0x105013c4);
    if (iVar7 == 0x0) {
      param_1->field_0x80 = 0x1;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e((ulong)param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x74 = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e((ulong)param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x72 = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e((ulong)param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x1e = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e((ulong)param_1->field_0xe,0x105013c8);
    if (iVar7 == 0x0) {
      param_1->field_0x20 = 0x0;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  puVar11 = puVar12;
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
    uStack46 = pass1_1000_3e2c((ulong)param_1->field_0xe);
    puVar11 = (uchar *)((uint)puVar12 | uStack46);
    puStack44 = puVar12;
    if ((uchar *)((uint)puVar12 | uStack46) != (uchar *)0x0) {
      param_1->field_0x76 = uStack46;
      param_1->field_0x78 = puVar12;
      puVar11 = puVar12;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
    iVar7 = pass1_1000_475e((ulong)param_1->field_0xe,0x105013c4);
    if (iVar7 == 0x0) {
      param_1->field_0x7a = 0x1;
    }
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar6 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar16,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar6 = (LPCSTR)0x1008;
    uVar8 = str_op_1008_60e8(param_1->field_0xe,(ushort)puVar11);
    param_1->field_0x1a = uVar8;
    param_1->field_0x1c = puVar11;
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
  GetPrivateProfileString16
            (pCVar6,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pCVar16 = (LPCSTR)0x1008;
    uVar8 = str_op_1008_60e8(param_1->field_0xe,(ushort)puVar11);
    param_1->field_0x68 = uVar8;
    param_1->field_0x6a = puVar11;
  }
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  index = (INT16)s_tile2_bmp_1050_1538;
  puVar9 = (uint *)GetPrivateProfileString16
                             (pCVar16,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
                              (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
                              (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    index = 0x1008;
    puVar9 = (uint *)str_op_1008_60e8(param_1->field_0xe,(ushort)puVar11);
    param_1->field_0x6c = (undefined *)puVar9;
    param_1->field_0x6e = puVar11;
  }
  if (param_1->field_0x62 == 0x0) {
    uVar15 = SUB42(s_tile2_bmp_1050_1538,0x0);
    uStack46 = GetSystemMetrics16(index);
    iStack42 = 0x1;
    do {
      get_private_profile_string_1010_6132(CONCAT22(param_2,param_1),iStack42,uVar15);
      puVar14 = &param_1->field_0x0 + iStack42 * 0x4;
      if (((((int)puVar14[0x11] < 0x0) || ((int)puVar14[0x12] < 0x0)) ||
          (piVar1 = puVar14 + 0x11, *piVar1 != iStack10 - uStack46 && (int)(iStack10 - uStack46) <= *piVar1)) ||
         (puVar9 = (uint *)(iStack12 - uStack46), ppuVar2 = (uint **)(puVar14 + 0x12),
         *ppuVar2 != puVar9 && (int)puVar9 <= (int)*ppuVar2)) {
        uVar15 = 0x1000;
        puVar9 = pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x22 + iStack42 * 0x8),(WNDCLASS16 *)0x0
                                 ,0x8);
      }
      iStack42 = iStack42 + 0x1;
    } while (iStack42 < 0x8);
  }
  mem_op_1000_179c(0xc,puVar11,0x1000);
  puStack50 = (undefined2 *)CONCAT22(puVar11,puVar9);
  if (((uint)puVar11 | (uint)puVar9) == 0x0) {
    puVar9 = (uint *)0x0;
    puVar12 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar11,puVar9));
    puVar12 = extraout_DX;
  }
  *(uint **)&param_1->field_0x64 = puVar9;
  *(uchar **)((int)&param_1->field_0x64 + 0x2) = puVar12;
  pcVar18 = param_1->field_0xe;
  pass1_1000_5008((ushort)pcVar18,(ushort)((ulong)pcVar18 >> 0x10),0x100,(int)&stack0xfffe);
  uStack4 = str_op_1000_3da4(param_1->field_0xe);
  pcVar18 = param_1->field_0xe;
  uVar15 = (undefined2)((ulong)pcVar18 >> 0x10);
  puVar13 = (undefined *)((int)pcVar18 + uStack4);
  if (puVar13[-0x1] != '\\') {
    *puVar13 = 0x5c;
    pcVar18 = param_1->field_0xe;
    *(undefined *)((int)pcVar18 + uStack4 + 0x1) = 0x0;
  }
  uVar10 = str_op_1008_60e8(param_1->field_0xe,(ushort)puVar12);
  uStack16 = CONCAT22(puVar12,uVar10);
  mem_op_1000_179c(0x8,puVar12,0x1000);
  puStack50 = (undefined2 *)CONCAT22(puVar12,uVar10);
  if (((uint)puVar12 | uVar10) == 0x0) {
    puStack20 = (undefined2 *)0x0;
  }
  else {
    *puStack50 = 0x389a;
    *(undefined2 *)(uVar10 + 0x2) = 0x1008;
    *(undefined4 *)(uVar10 + 0x4) = uStack16;
    *puStack50 = 0x6322;
    *(undefined2 *)(uVar10 + 0x2) = 0x1010;
    puStack20 = puStack50;
  }
  puVar4 = param_1->field_0x64;
  ppcVar5 = (code **)((int)*param_1->field_0x64 + 0x4);
  (**ppcVar5)(0x1000,(int)puVar4,(int)((ulong)puVar4 >> 0x10),(int)puStack20,(int)((ulong)puStack20 >> 0x10));
  pcVar18 = param_1->field_0xe;
  uVar3 = *(undefined4 *)&param_1->field_0xa;
  puVar12 = extraout_DX_00;
  GetPrivateProfileString16
            ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)pcVar18,
             (LPCSTR)((ulong)pcVar18 >> 0x10));
  if (*param_1->field_0xe != '\0') {
    pcVar18 = param_1->field_0xe;
    uVar15 = SUB42(pcVar18,0x0);
    uVar20 = (undefined2)((ulong)pcVar18 >> 0x10);
    while (uStack46 = pass1_1000_47a4(CONCAT22(uVar20,uVar15),0x105013f8,param_4), ((uint)puVar12 | uStack46) != 0x0) {
      puStack44 = puVar12;
      unk_str_op_1000_3d3e((char *)CONCAT22(param_4,local_134),(char *)CONCAT22(puVar12,uStack46));
      uStack4 = str_op_1000_3da4((char *)CONCAT22(param_4,local_134));
      if (local_134[uStack4 - 0x1] != '\\') {
        local_134[uStack4] = 0x5c;
        local_134[uStack4 + 0x1] = 0x0;
      }
      uVar10 = str_op_1008_60e8((char *)CONCAT22(param_4,local_134),(ushort)puVar12);
      uStack16 = CONCAT22(puVar12,uVar10);
      mem_op_1000_179c(0x8,puVar12,0x1000);
      puStack50 = (undefined2 *)CONCAT22(puVar12,uVar10);
      if (((uint)puVar12 | uVar10) == 0x0) {
        puStack20 = (undefined2 *)0x0;
      }
      else {
        *puStack50 = 0x389a;
        *(undefined2 *)(uVar10 + 0x2) = 0x1008;
        *(undefined4 *)(uVar10 + 0x4) = uStack16;
        *puStack50 = 0x6322;
        *(undefined2 *)(uVar10 + 0x2) = 0x1010;
        puStack20 = puStack50;
      }
      puVar4 = param_1->field_0x64;
      ppcVar5 = (code **)((int)*param_1->field_0x64 + 0x8);
      (**ppcVar5)(0x1000,(int)puVar4,(int)((ulong)puVar4 >> 0x10),(int)puStack20,(int)((ulong)puStack20 >> 0x10));
      uVar15 = 0x0;
      uVar20 = 0x0;
      puVar12 = extraout_DX_01;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far write_private_profile_str_1010_5b10(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 uVar3;
  code **ppcVar4;
  LPCSTR pCVar5;
  uchar *in_DX;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  ushort unaff_SS;
  undefined in_AF;
  ushort *puVar8;
  int iStack12;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  *param_1 = 0x6312;
  *(undefined2 *)(iVar6 + 0x2) = 0x1010;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  uVar3 = *(undefined4 *)(iVar6 + 0xe);
  sys_1000_3f9c((uchar *)uVar3,(uchar *)((ulong)uVar3 >> 0x10),0x149c,(ushort)&USHORT_1050_1050,
                *(ushort *)((int)puVar8 + 0xa),&stack0xfffe,uVar7,0x1000,unaff_SS,in_AF);
  if (*(int *)(iVar6 + 0x80) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),pCVar5);
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPCSTR)*(undefined4 *)(iVar6 + 0xe));
  if (*(int *)(iVar6 + 0x1e) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),pCVar5);
  if (*(int *)(iVar6 + 0x74) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),pCVar5);
  if (*(int *)(iVar6 + 0x72) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),pCVar5);
  if (*(int *)(iVar6 + 0x20) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),pCVar5);
  uVar3 = *(undefined4 *)(iVar6 + 0xe);
  sys_1000_3f9c((uchar *)uVar3,(uchar *)((ulong)uVar3 >> 0x10),0x14a2,(ushort)&USHORT_1050_1050,
                (ushort)*(undefined4 *)(iVar6 + 0x76),&stack0xfffe,uVar7,0x1000,unaff_SS,in_AF);
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPCSTR)*(undefined4 *)(iVar6 + 0xe));
  if (*(int *)(iVar6 + 0x7a) == 0x0) {
    pCVar5 = (LPCSTR)0x13c8;
  }
  else {
    pCVar5 = (LPCSTR)0x13c4;
  }
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),pCVar5);
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPCSTR)*(undefined4 *)(iVar6 + 0x1a));
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPCSTR)*(undefined4 *)(iVar6 + 0x68));
  uVar3 = *(undefined4 *)(iVar6 + 0xa);
  WritePrivateProfileString16
            ((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)uVar3,(LPCSTR)((ulong)uVar3 >> 0x10),
             (LPCSTR)*(undefined4 *)(iVar6 + 0x6c));
  iStack12 = 0x1;
  do {
    switchD_1010:2ab5::caseD_13((ulong)param_1,iStack12);
    iStack12 = iStack12 + 0x1;
  } while (iStack12 < 0x8);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0xa),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0xe),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x12),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x16),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x1a),0x1000);
  puVar1 = (undefined4 *)*(uint *)(iVar6 + 0x64);
  uVar2 = *(uint *)(iVar6 + 0x66);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)(0x1000,puVar1,uVar2,0x1);
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x68),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x6c),0x1000);
  pass1_1010_1d80(param_1,unaff_SS);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_5d9c(ulong param_1,int param_2,uchar *param_3,int param_4,ushort param_5)

{
  ushort *puVar1;
  
  *(int *)((int)param_1 + 0x1e) = param_2;
  if (param_2 == 0x0) {
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2e,param_5,param_3,param_4);
    pass1_1018_209c((ulong)puVar1);
  }
  return;
}



undefined2 __stdcall16far pass1_1010_5dc6(ulong param_1,ulong param_2,uint16_t param_3)

{
  BOOL16 BVar1;
  int iVar2;
  undefined2 uVar3;
  ushort uVar4;
  undefined *local_c [0x3];
  undefined2 local_6 [0x2];
  
  BVar1 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar1 != 0x0) {
    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    BVar1 = pass1_1008_7c2a(param_2,*(char **)(iVar2 + 0x68),0x1008);
    if (BVar1 != 0x0) {
      BVar1 = pass1_1008_7c2a(param_2,*(char **)(iVar2 + 0x6c),0x1008);
      if (BVar1 != 0x0) {
        local_c[0] = PTR_LOOP_1050_13ae;
        uVar4 = (ushort)(param_2 >> 0x10);
        BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar4,(ushort)local_c,param_3,(char *)0x2,0x1008);
        if (BVar1 != 0x0) {
          local_6[0] = *(undefined2 *)(iVar2 + 0x82);
          BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar4,(ushort)local_6,param_3,(char *)0x2,0x1008);
          if (BVar1 != 0x0) {
            return 0x1;
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



void __stdcall16far pass1_1010_5e56(ulong param_1,ulong param_2,int param_3,ushort param_4,uint16_t param_5)

{
  undefined *puVar1;
  ushort uVar2;
  BOOL16 BVar3;
  int iVar4;
  ushort uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined *local_404;
  undefined local_402 [0x400];
  
  uVar6 = (ushort)param_2;
  uVar7 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar6,uVar7,0x4,0x1008,param_5);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
  }
  else {
    puVar1 = local_402;
    read_file_1008_7c6e(uVar6,uVar7,(char *)CONCAT22(param_5,puVar1),0x1008);
    if (puVar1 != (undefined *)0x0) {
      uVar2 = str_op_1008_60e8((char *)CONCAT22(param_5,local_402),param_4);
      uVar5 = (ushort)(param_1 >> 0x10);
      iVar4 = (int)param_1;
      *(ushort *)(iVar4 + 0x68) = uVar2;
      *(ushort *)(iVar4 + 0x6a) = param_4;
      puVar1 = local_402;
      read_file_1008_7c6e(uVar6,uVar7,(char *)CONCAT22(param_5,puVar1),0x1008);
      if (puVar1 != (undefined *)0x0) {
        uVar2 = str_op_1008_60e8((char *)CONCAT22(param_5,local_402),param_4);
        *(ushort *)(iVar4 + 0x6c) = uVar2;
        *(ushort *)(iVar4 + 0x6e) = param_4;
        BVar3 = read_file_1008_7dee(uVar6,uVar7,(ushort)&local_404,0x0,param_5,0x2,0x1008);
        if (BVar3 != 0x0) {
          PTR_LOOP_1050_13ae = local_404;
          if ((int)PTR_LOOP_1050_0312 < 0x2) {
            return;
          }
          BVar3 = read_file_1008_7dee(uVar6,uVar7,iVar4 + 0x82,0x0,uVar5,0x2,0x1008);
          if (BVar3 != 0x0) {
            return;
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



void __stdcall16far struct_1010_5f1e(astruct_73 *param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  astruct_73 *iVar3;
  astruct_73 *uVar3;
  
  uVar3 = (astruct_73 *)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_73 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x16,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x16 = uVar1;
  iVar3->field_0x18 = param_3;
  return;
}



void __stdcall16far pass1_1010_5f4c(ulong param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  astruct_484 *iVar3;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_484 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x12,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x12 = uVar1;
  iVar3->field_0x14 = param_3;
  return;
}



ulong __stdcall16far pass1_1010_5f7a(int param_1,ushort param_2,ushort param_3,int param_4)

{
  int iVar1;
  
  iVar1 = param_4 * 0x8 + param_1;
  if ((*(int *)(iVar1 + 0x26) == 0x0) && (*(int *)(iVar1 + 0x28) == 0x0)) {
    return 0x0;
  }
  return CONCAT22(param_2,param_4 * 0x8 + param_1 + 0x22);
}



void __stdcall16far pass1_1010_5fb0(ulong param_1,ushort param_2,ulong *param_3,ushort param_4,int param_5)

{
  undefined2 uVar1;
  astruct_656 *iVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  iVar1 = (astruct_656 *)((int)param_1 + param_5 * 0x8);
  iVar1->field_0x22 = *param_3;
  iVar1->field_0x26 = param_3[0x1];
  return;
}



void __stdcall16far pass1_1010_5fd8(ulong param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  astruct_485 *iVar3;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_485 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x68,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x68 = uVar1;
  iVar3->field_0x6a = param_3;
  return;
}



void __stdcall16far pass1_1010_6006(ulong param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  astruct_486 *iVar3;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_486 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x6c,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x6c = uVar1;
  iVar3->field_0x6e = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_6034(ulong param_1,ushort param_2)

{
  uint *puVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  *(undefined2 *)(iVar2 + 0x1e) = 0x1;
  *(undefined2 *)(iVar2 + 0x20) = 0x1;
  *(undefined2 *)(iVar2 + 0x72) = 0x1;
  *(undefined2 *)(iVar2 + 0x74) = 0x1;
  pass1_1010_60a0(param_1);
  puVar1 = pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)(iVar2 + 0x22)),(WNDCLASS16 *)0x0,0x40);
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1000);
  *(uint **)(iVar2 + 0x68) = puVar1;
  *(ushort *)(iVar2 + 0x6a) = param_2;
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1000);
  *(uint **)(iVar2 + 0x6c) = puVar1;
  *(ushort *)(iVar2 + 0x6e) = param_2;
  return;
}



void __stdcall16far pass1_1010_60a0(ulong param_1)

{
  *(undefined4 *)((int)param_1 + 0x76) = 0x5;
  return;
}



ushort __stdcall16far pass1_1010_60b4(void)

{
  return 0x1;
}



ushort __stdcall16far pass1_1010_60ba(void)

{
  return 0x1;
}



ushort __stdcall16far pass1_1010_60c0(void)

{
  return 0x1;
}



ushort __stdcall16far pass1_1010_60c6(void)

{
  return 0x1;
}



void __stdcall16far pass1_1010_60cc(ulong param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  astruct_487 *iVar3;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_487 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x1a,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x1a = uVar1;
  iVar3->field_0x1c = param_3;
  return;
}



void __stdcall16far pass1_1010_60fa(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x7e) = 0x1;
  *(undefined2 *)(iVar1 + 0x7c) = *(undefined2 *)(iVar1 + 0x20);
  *(undefined2 *)(iVar1 + 0x20) = 0x1;
  return;
}



void __stdcall16far pass1_1010_6118(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x7e) != 0x0) {
    *(undefined2 *)(iVar1 + 0x20) = *(undefined2 *)(iVar1 + 0x7c);
  }
  return;
}



void __stdcall16far get_private_profile_string_1010_6132(ulong param_1,int param_2,LPCSTR param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  ushort uVar3;
  int iVar4;
  uint in_DX;
  uint uVar5;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  ushort unaff_SS;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar7 + 0xe);
  uVar2 = *(undefined4 *)(iVar7 + 0xa);
  GetPrivateProfileString16
            (param_3,(LPCSTR)uVar2,(LPCSTR)((ulong)uVar2 >> 0x10),
             (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),(UINT16)uVar1,
             (LPCSTR)((ulong)uVar1 >> 0x10));
  if (**(char **)(iVar7 + 0xe) != '\0') {
    uVar3 = pass1_1000_47a4(*(ulong *)(iVar7 + 0xe),0x105014a6,unaff_SS);
    uVar5 = in_DX | uVar3;
    if (uVar5 != 0x0) {
      iVar4 = pass1_1000_3e2c(CONCAT22(in_DX,uVar3));
      iVar7 = param_2 * 0x8 + iVar7;
      *(int *)(iVar7 + 0x22) = iVar4;
      uVar3 = pass1_1000_47a4(0x0,0x105014a8,unaff_SS);
      uVar6 = uVar5 | uVar3;
      if (uVar6 != 0x0) {
        iVar4 = pass1_1000_3e2c(CONCAT22(uVar5,uVar3));
        *(int *)(iVar7 + 0x24) = iVar4;
        uVar3 = pass1_1000_47a4(0x0,0x105014aa,unaff_SS);
        uVar5 = uVar6 | uVar3;
        if (uVar5 != 0x0) {
          iVar4 = pass1_1000_3e2c(CONCAT22(uVar6,uVar3));
          *(int *)(iVar7 + 0x26) = iVar4;
          uVar3 = pass1_1000_47a4(0x0,0x105014ac,unaff_SS);
          if ((uVar5 | uVar3) != 0x0) {
            iVar4 = pass1_1000_3e2c(CONCAT22(uVar5,uVar3));
            *(int *)(iVar7 + 0x28) = iVar4;
          }
        }
      }
    }
  }
  return;
}



void __stdcall16far switchD_1010:2ab5::caseD_13(ulong param_1,int param_2)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 unaff_SS;
  undefined1 in_AF;
  
  iVar2 = param_2 * 0x8 + (int)param_1;
  if ((((*(int *)(iVar2 + 0x22) != 0x0) || (*(int *)(iVar2 + 0x24) != 0x0)) || (*(int *)(iVar2 + 0x26) != 0x0)) ||
     (*(int *)(iVar2 + 0x28) != 0x0)) {
    uVar1 = *(undefined4 *)((int)param_1 + 0xe);
    sys_1000_3f9c((uchar *)uVar1,(uchar *)((ulong)uVar1 >> 0x10),(ushort)s__d__d__d__d_1050_14ae,
                  (ushort)&USHORT_1050_1050,(ushort)*(undefined4 *)(param_2 * 0x8 + (int)param_1 + 0x22),&stack0xfffe,
                  param_1._2_2_,0x1000,unaff_SS,in_AF);
    uVar1 = *(undefined4 *)((int)param_1 + 0xa);
    WritePrivateProfileString16
              ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar1,(LPCSTR)((ulong)uVar1 >> 0x10),
               (LPCSTR)*(undefined4 *)((int)param_1 + 0xe));
  }
  return;
}



void __stdcall16far pass1_1010_62a4(ushort *param_1,byte param_2)

{
  astruct_488 *uVar2;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  uVar2 = (astruct_488 *)param_1;
  *param_1 = 0x6322;
  uVar2->field_0x2 = 0x1010;
  fn_ptr_1000_17ce((astruct_18 *)uVar2->field_0x4,0x1000);
  *param_1 = 0x389a;
  uVar2->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



ushort * __stdcall16far pass1_1010_62ec(ushort *param_1,byte param_2)

{
  write_private_profile_str_1010_5b10(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}

