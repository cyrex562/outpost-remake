
void __stdcall16far cursor_op_1008_2dcc(int param_1,uint16_t param_2,uint16_t param_3,HINSTANCE16 in_hinstance)

{
  undefined4 uVar1;
  code **ppcVar2;
  HCURSOR16 cursor_handle;
  HCURSOR16 HVar3;
  undefined2 in_DX;
  undefined2 extraout_DX;
  u16 uVar4;
  u16 unaff_SS;
  undefined2 uVar5;
  
  uVar5 = 0x0;
  cursor_handle = LoadCursor16(in_hinstance,(LPCSTR)0x7f02);
  HVar3 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  uVar4 = param_1;
  if (*(long *)(param_1 + 0xe8) != 0x0) {
    uVar1 = *(undefined4 *)(param_1 + 0xe8);
    uVar4 = (u16)*(undefined4 *)*(undefined4 *)(param_1 + 0xe8);
    ppcVar2 = (code **)(uVar4 + 0x90);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(int)((ulong)uVar1 >> 0x10),cursor_handle,uVar5);
    in_DX = extraout_DX;
  }
  big_switch_1008_15d4
            (uVar4,(u16)s_tile2_bmp_1050_1538,unaff_SS,CONCAT22(param_2,param_1),CONCAT22(cursor_handle,param_3));
  *(HCURSOR16 *)(param_1 + 0xe8) = HVar3;
  *(undefined2 *)(param_1 + 0xea) = in_DX;
  uVar1 = *(undefined4 *)(param_1 + 0xe8);
  if (*(int *)((int)uVar1 + 0xe0) == 0x0) {
    uVar1 = *(undefined4 *)(param_1 + 0xe8);
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xe8) + 0x8);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(int)((ulong)uVar1 >> 0x10));
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xe8) + 0xc);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,*(undefined4 *)(param_1 + 0xe8),0x3);
    *(undefined4 *)(param_1 + 0xce) = *(undefined4 *)(param_1 + 0xe8);
  }
  else {
    *(undefined4 *)(param_1 + 0xe8) = 0x0;
    ui_op_1008_2c4e(param_1,param_2,param_3,(HINSTANCE16)s_tile2_bmp_1050_1538);
    *(undefined4 *)(param_1 + 0xce) = 0x0;
  }
  SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  return;
}


void __stdcall16far win_ui_cursor_op_1008_2e9a(astruct_72 **param_1,ushort param_2)

{
  undefined2 uVar1;
  int iVar2;
  uchar *in_DX;
  uint uVar3;
  uint uVar4;
  int unaff_DI;
  undefined in_AF;
  char local_22e [0xa];
  undefined local_224 [0x108];
  uint uStack284;
  char *pcStack282;
  HCURSOR16 HStack274;
  HCURSOR16 HStack272;
  ulong uStack270;
  ULONG UStack266;
  undefined4 uStack262;
  char local_102 [0x100];
  
  local_102[0] = '\0';
  uStack262 = (astruct_73 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uVar1 = (undefined2)((ulong)uStack262 >> 0x10);
  iVar2 = (int)uStack262;
  UStack266 = *(ULONG *)(iVar2 + 0x16);
  uVar3 = *(uint *)(iVar2 + 0x18);
  UStack266._0_2_ = uVar3 | (uint)UStack266;
  if ((uint)UStack266 == 0x0) {
    save_file_1008_3178((ulong)param_1,0x1,param_2);
    UStack266 = CONCAT22(uVar3,(uint)UStack266);
    uVar4 = uVar3 | (uint)UStack266;
    if (uVar4 == 0x0) {
      PostMessage16(0x1010,0x0,0x0,0x111013d);
      return;
    }
    unk_str_op_1000_3d3e((char *)CONCAT22(param_2,local_102),(char *)CONCAT22(uVar3,(uint)UStack266));
    str_1000_4d58((char *)CONCAT22(param_2,local_102),(char *)0x0,0x0,CONCAT22(param_2,local_224),
                  (WNDCLASS16 *)CONCAT22(param_2,local_22e));
    uVar3 = uVar4;
    if (local_22e[0] != '\0') {
      pass1_1000_3cea(CONCAT22(param_2,local_224),CONCAT22(param_2,local_22e));
      uVar3 = uVar4;
    }
    struct_1010_5f1e(uStack262,CONCAT22(param_2,local_224),uVar3);
  }
  else {
    pcStack282 = *(char **)(iVar2 + 0x1a);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_2,local_102),pcStack282);
    uStack284 = str_op_1000_3da4((char *)CONCAT22(param_2,local_102));
    if (local_102[uStack284 - 0x1] != '\\') {
      local_102[uStack284] = '\\';
      local_102[uStack284 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(param_2,local_102),UStack266);
  }
  if (local_102[0] != '\0') {
    uStack270 = *(ulong *)((int)param_1 + 0xe8);
    send_msg_1020_097e(uStack270,0x1020);
    UpdateWindow16(0x1020);
    HStack272 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f02);
    HStack274 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    win_ui_op_1008_1414(param_1,CONCAT22(param_2,local_102),(LPCSTR)s_tile2_bmp_1050_1538,param_2,in_AF,uVar3);
    SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  }
  return;
}

void __stdcall16far pass1_1008_3018(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ULONG UVar1;
  undefined2 uVar2;
  int iVar3;
  uint uVar4;
  uint uStack266;
  undefined4 uStack262;
  char local_102 [0x100];
  
  local_102[0] = '\0';
  uStack262 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  uVar2 = (undefined2)((ulong)uStack262 >> 0x10);
  iVar3 = (int)uStack262;
  UVar1 = *(ULONG *)(iVar3 + 0x12);
  uVar4 = *(uint *)(iVar3 + 0x14);
  uStack266 = (uint)UVar1;
  if ((uVar4 | uStack266) == 0x0) {
    pass1_1008_30cc(param_1,0x0,uVar4,param_3,param_4);
  }
  else {
    unk_str_op_1000_3d3e((char *)CONCAT22(param_4,local_102),*(char **)(iVar3 + 0x1a));
    uVar4 = str_op_1000_3da4((char *)CONCAT22(param_4,local_102));
    if (local_102[uVar4 - 0x1] != '\\') {
      local_102[uVar4] = '\\';
      local_102[uVar4 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(param_4,local_102),UVar1);
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc(param_1,CONCAT22(param_4,local_102),0x1000,param_4);
      return;
    }
  }
  return;
}


void __stdcall16far menu_ui_op_1008_09ba(ulong param_1,HWND16 param_2,RECT16 *param_3,HWND16 param_4)

{
  HMENU16 HVar1;
  int iVar2;
  undefined2 uVar3;
  POINT16 local_6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0xec) == 0x0) {
    HVar1 = LoadMenu16(param_4,(LPCSTR)s_OPPOPMENU_1050_0150);
    *(HMENU16 *)(iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    param_4 = (HWND16)s_tile2_bmp_1050_1538;
    HVar1 = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0);
    *(HMENU16 *)(iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  local_6.x = (INT16)param_3;
  local_6.y = param_2;
  ClientToScreen16(param_4,&local_6);
  TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x0,(INT16)PTR_LOOP_1050_0396,0x0,local_6.y,(RECT16 *)local_6.x);
  return;
}


void __stdcall16far switchD_1008:1091::caseD_a7(void)

{
  undefined4 uVar1;
  int unaff_BP;
  HINSTANCE16 unaff_CS;
  undefined2 unaff_SS;
  
  uVar1 = *(undefined4 *)(unaff_BP + 0x6);
  ui_op_1008_2c4e((int)uVar1,(int)((ulong)uVar1 >> 0x10),0x57,unaff_CS);
  return;
}



void __stdcall16far switchD_1008:1091::caseD_aa(void)

{
  undefined4 uVar1;
  int unaff_BP;
  HINSTANCE16 unaff_CS;
  undefined2 unaff_SS;
  
  uVar1 = *(undefined4 *)(unaff_BP + 0x6);
  ui_op_1008_2c4e((int)uVar1,(int)((ulong)uVar1 >> 0x10),0x58,unaff_CS);
  return;
}



void __stdcall16far switchD_1008:1091::caseD_ac(void)

{
  undefined4 uVar1;
  int unaff_BP;
  HINSTANCE16 unaff_CS;
  undefined2 unaff_SS;
  
  uVar1 = *(undefined4 *)(unaff_BP + 0x6);
  ui_op_1008_2c4e((int)uVar1,(int)((ulong)uVar1 >> 0x10),0x59,unaff_CS);
  return;
}



void __stdcall16far switchD_1008:1091::caseD_ad(void)

{
  undefined4 uVar1;
  int unaff_BP;
  HINSTANCE16 unaff_CS;
  undefined2 unaff_SS;
  
  uVar1 = *(undefined4 *)(unaff_BP + 0x6);
  ui_op_1008_2c4e((int)uVar1,(int)((ulong)uVar1 >> 0x10),0x5a,unaff_CS);
  return;
}



void __stdcall16far switchD_1008:1091::caseD_ae(void)

{
  undefined4 uVar1;
  int unaff_BP;
  HINSTANCE16 unaff_CS;
  undefined2 unaff_SS;
  
  uVar1 = *(undefined4 *)(unaff_BP + 0x6);
  ui_op_1008_2c4e((int)uVar1,(int)((ulong)uVar1 >> 0x10),0x5b,unaff_CS);
  return;
}



void __stdcall16far switchD_1008:1091::caseD_b1(void)

{
  undefined4 uVar1;
  int unaff_BP;
  HINSTANCE16 unaff_CS;
  undefined2 unaff_SS;
  
  uVar1 = *(undefined4 *)(unaff_BP + 0x6);
  ui_op_1008_2c4e((int)uVar1,(int)((ulong)uVar1 >> 0x10),0x5c,unaff_CS);
  return;
}



void __stdcall16far switchD_1008:1091::caseD_b3(void)

{
  undefined4 uVar1;
  int unaff_BP;
  HINSTANCE16 unaff_CS;
  undefined2 unaff_SS;
  
  uVar1 = *(undefined4 *)(unaff_BP + 0x6);
  ui_op_1008_2c4e((int)uVar1,(int)((ulong)uVar1 >> 0x10),0x5d,unaff_CS);
  return;
}



void __stdcall16far draw_op_1008_1230(HWND16 param_1)

{
  fill_rect_1008_39ac(param_1);
  return;
}


void __stdcall16far message_box_op_1008_12dc(ulong param_1,ulong param_2,HINSTANCE16 param_3,ushort param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort in_DX;
  uint uVar3;
  uchar in_AF;
  char *pcVar4;
  ulong uStack36;
  ulong uStack16;
  undefined local_c [0x6];
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;
  
  HStack4 = LoadCursor16(param_3,(LPCSTR)0x7f02);
  HStack6 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  str_1008_6d8a((ulong *)CONCAT22(param_4,local_c),(char *)param_2,in_DX,param_4,in_AF);
  BVar1 = file_fn_1008_6e02((uint32_t *)CONCAT22(param_4,local_c),(int)s_tile2_bmp_1050_1538,param_4);
  if (BVar1 == 0x0) {
    SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar3 = (uint)((ulong)pcVar4 >> 0x10);
    uVar2 = str_op_1008_60e8(pcVar4,uVar3);
    uStack16 = CONCAT22(uVar3,uVar2);
    pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    MessageBeep16(0x1010);
    MessageBox16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,(LPCSTR)pcVar4,
                 (UINT16)((ulong)pcVar4 >> 0x10));
  }
  else {
    *(undefined2 *)((int)_PTR_LOOP_1050_5748 + 0x8) = 0x0;
    SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar3 = (uint)((ulong)pcVar4 >> 0x10);
    uVar2 = str_op_1008_60e8(pcVar4,uVar3);
    uStack36 = CONCAT22(uVar3,uVar2);
    pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    MessageBeep16(0x1010);
    MessageBox16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x40,(LPCSTR)pcVar4,(UINT16)((ulong)pcVar4 >> 0x10));
    uStack16 = uStack36;
  }
  fn_ptr_1000_17ce((astruct_18 *)(uStack16 & 0xffff | (ulong)uVar3 << 0x10),0x1000);
  close_file_1008_6dd0((undefined4 *)CONCAT22(param_4,local_c),0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_ui_op_1008_1414(astruct_72 **param_1,ulong param_2,LPCSTR param_3,ushort param_4,uchar param_5,ushort param_6)

{
  code **ppcVar1;
  BOOL16 BVar2;
  ushort uVar3;
  int iVar4;
  ulong *puVar5;
  ulong uVar5;
  uchar *puVar6;
  ushort uVar7;
  uchar *type;
  uint uVar8;
  ushort extraout_DX;
  int unaff_DI;
  uint16_t uVar9;
  ulong *puVar10;
  char *pcVar11;
  ushort *puVar12;
  undefined uVar13;
  undefined uVar14;
  int iVar15;
  undefined4 local_2a;
  ushort uStack38;
  int iStack36;
  uchar *puStack34;
  ulong uStack32;
  ulong uStack28;
  ulong uStack24;
  ulong uStack20;
  ulong uStack16;
  ushort *puStack12;
  undefined local_8 [0x6];
  ushort uVar10;
  
  puVar10 = str_1008_6d8a((ulong *)CONCAT22(param_4,local_8),(char *)param_2,param_6,param_4,param_5);
  puVar6 = (uchar *)((ulong)puVar10 >> 0x10);
  BVar2 = read_file_1008_6e78((uint32_t)local_8,param_4,param_3,param_4);
  iVar15 = (int)param_1;
  uVar9 = (uint16_t)((ulong)param_1 >> 0x10);
  if (BVar2 == 0x0) {
    if (PTR_LOOP_1050_0310 == (undefined *)0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d4;
    }
    pcVar11 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar7 = (ushort)((ulong)pcVar11 >> 0x10);
    uVar3 = str_op_1008_60e8(pcVar11,uVar7);
    pcVar11 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    type = (uchar *)((ulong)pcVar11 >> 0x10);
    puVar6 = type;
    MessageBeep16(0x1010);
    MessageBox16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,(LPCSTR)pcVar11,(UINT16)type);
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar7,uVar3),0x1000);
    param_3 = (LPCSTR)&PTR_LOOP_1050_1000;
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  cursor_op_1008_2dcc(iVar15,uVar9,0x8,param_3);
  puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,puVar6,unaff_DI);
  uVar8 = (uint)((ulong)puStack12 >> 0x10);
  uVar5 = *(ulong *)((int)puStack12 + 0x20);
  uStack16 = uVar5;
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uVar5);
  uStack20 = uVar5 & 0xffff | (ulong)uVar8 << 0x10;
  uStack24 = *(ulong *)((int)uVar5 + 0x10);
  iVar4 = *(int *)((int)uStack24 + 0x2) + -0x1;
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0xe8) + 0x4);
  (**ppcVar1)(0x1030,*(undefined4 *)(iVar15 + 0xe8),(int)uStack16,(int)(uStack16 >> 0x10),iVar4,0x2);
  puVar6 = (uchar *)extraout_DX;
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x4000001);
  uStack28 = CONCAT22(puVar6,iVar4);
  uVar5 = *(ulong *)(iVar4 + 0x10);
  uStack32 = uVar5;
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uVar5);
  iStack36 = (int)uVar5;
  local_2a = *(undefined4 *)(iStack36 + 0xc);
  uStack38 = *(ushort *)(iStack36 + 0x10);
  puStack34 = puVar6;
  puVar5 = (ulong *)pass1_1030_5b00(uStack20);
  uVar13 = SUB21(&local_2a,0x0);
  uVar14 = (undefined)((uint)&local_2a >> 0x8);
  uVar3 = param_4;
  puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,(ushort)puVar5,param_4,puVar6,(int)&iStack36);
  puVar6 = (uchar *)((ulong)puVar12 >> 0x10);
  pass1_1018_179e((ulong)puVar12,CONCAT22(uVar3,CONCAT11(uVar14,uVar13)),0x1018,param_4);
  uVar13 = 0x0;
  uVar14 = 0x4;
  iVar15 = 0x1b;
  uVar10 = 0x1;
  puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,puVar6,(int)&iStack36);
  pass1_1010_043a((ulong)puVar12,CONCAT13(uVar14,CONCAT12(uVar13,uVar10)),iVar15,param_4);
  close_file_1008_6dd0((undefined4 *)CONCAT22(param_4,local_8),0x1010);
  return;
}

void __stdcall16far cleanup_ui_op_1008_0618(undefined2 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  astruct_18 *paVar3;
  code **ppcVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 unaff_CS;
  HICON16 h_icon;
  ushort unaff_SS;
  undefined2 uVar7;
  undefined2 uVar8;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = 0x389e;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  set_sys_color_1008_357e((ulong)param_1,0x0,unaff_CS,unaff_SS);
  paVar3 = *(astruct_18 **)(iVar5 + 0xf8);
  uVar8 = (undefined2)((ulong)paVar3 >> 0x10);
  h_icon = 0x1000;
  fn_ptr_1000_17ce(paVar3,0x1000);
  if (*(int *)(iVar5 + 0xec) != 0x0) {
    uVar8 = *(undefined2 *)(iVar5 + 0xec);
    h_icon = (HICON16)s_tile2_bmp_1050_1538;
    DestroyMenu16(0x1000);
  }
  uVar7 = *(undefined2 *)(iVar5 + 0xc2);
  DestroyIcon16(h_icon);
  *(undefined2 *)(iVar5 + 0xc2) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar5 + 0xe0);
  uVar2 = *(uint *)(iVar5 + 0xe2);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)((int)s_tile2_bmp_1050_1538,puVar1,uVar2,0x1,uVar7,uVar8);
  }
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar5 + 0xd2)));
  *param_1 = 0x380a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  return;
}


void __stdcall16far win_ui_cursor_op_1008_06c0(ulong *param_1,ulong param_2,ushort param_3,int param_4)

{
  code **ppcVar1;
  uint in_AX;
  uint in_DX;
  uchar *puVar2;
  uchar *extraout_DX;
  int unaff_DI;
  undefined2 uVar3;
  uchar *unaff_SS;
  undefined in_AF;
  char *pcVar4;
  ushort *puVar5;
  uchar local_5a [0x50];
  undefined4 uStack10;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;
  
  if (param_4 == 0x400) {
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x4000001);
    puVar2 = (uchar *)(in_DX | in_AX);
    if (puVar2 != (uchar *)0x0) {
      if (PTR_LOOP_1050_4fe8 != (undefined *)0x0) {
        pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        MessageBox16(0x1010,(LPCSTR)&PTR_LOOP_1050_0010,(LPCSTR)pcVar4,(UINT16)((ulong)pcVar4 >> 0x10));
        return;
      }
      HStack4 = LoadCursor16(0x1030,(LPCSTR)0x7f02);
      HStack6 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
      pass1_1030_83ba(_PTR_LOOP_1050_5748,param_2,unaff_SS,in_AF);
      uVar3 = (undefined2)((ulong)_PTR_LOOP_1050_5748 >> 0x10);
      *(undefined2 *)((int)_PTR_LOOP_1050_5748 + 0x8) = 0x1;
      uStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,(ushort)unaff_SS,puVar2,unaff_DI);
      pass1_1018_262e((ulong)uStack10);
      pass1_1030_8326();
      pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      sys_1000_3f9c(local_5a,unaff_SS,0x109,(ushort)&USHORT_1050_1050,(ushort)pcVar4,&stack0xfffe,uVar3,0x1000,unaff_SS,
                    in_AF);
      ppcVar1 = (code **)((int)*param_1 + 0x14);
      (**ppcVar1)(0x1000,(int)param_1,(char)((ulong)param_1 >> 0x10),0x0,local_5a);
      puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,(ushort)unaff_SS,extraout_DX,unaff_DI);
      pass1_1008_a9ec((ulong)puVar5);
      SetCursor16(0x1010);
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100fc);
    }
  }
  return;
}



BOOL16 msg_box_op_1000_1f24(int param_1,UINT16 param_2,uint param_3,UINT16 param_4)

{
  int *piVar1;
  UINT16 unaff_CS;
  
  if (param_3 < *(uint *)(param_1 + 0xc)) {
    msg_box_op_1000_214c(0x0,0x0,0xd940,(UINT16)&PTR_LOOP_1050_1040,param_4);
    return 0x1;
  }
  piVar1 = (int *)(param_1 + 0xc);
  *piVar1 = *piVar1 + 0x1;
  return 0x0;
}


BOOL16 __stdcall16far pass1_1000_1f7e(uint *param_1,ushort param_2)

{
  char cVar1;
  BOOL16 BVar2;
  uint uVar3;
  int iVar4;
  char *pcVar5;
  
  uVar3 = *param_1;
  if (uVar3 == 0xf) {
LAB_1000_1fb6:
    iVar4 = 0x1;
  }
  else {
    if (uVar3 < 0x10) {
      cVar1 = (char)uVar3;
      if (cVar1 == '\x02') goto LAB_1000_1fb6;
      if (('\0' < (char)(cVar1 + -0x2)) && ((char)(cVar1 + -0x3) < '\f')) {
        iVar4 = 0x0;
        goto LAB_1000_1fbe;
      }
    }
    iVar4 = 0x0;
    uVar3 = 0x1;
  }
LAB_1000_1fbe:
  pcVar5 = pass1_1000_1fd2(uVar3);
  BVar2 = msg_box_op_1000_214c(0x0,iVar4,(UINT16)pcVar5,(UINT16)((ulong)pcVar5 >> 0x10),param_2);
  return BVar2;
}

BOOL16 __stdcall16far msg_box_op_1000_214c(UINT16 param_1,int param_2,UINT16 param_3,UINT16 param_4,UINT16 param_5)

{
  INT16 IVar1;
  int iVar2;
  LPCSTR text;
  
  text = (LPCSTR)(0x2 - (param_2 == 0x0) | 0x2110);
  MessageBeep16(param_5);
  do {
    IVar1 = MessageBox16((HWND16)s_tile2_bmp_1050_1538,text,(LPCSTR)0x1de8,0x1000);
    iVar2 = IVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x0;
    }
    if ((0x0 < iVar2) && (!SBORROW2(iVar2,0x1))) {
      if (IVar1 == 0x3 || IVar1 + -0x2 < 0x1) {
        fatal_app_exit_1000_3e9e((int)s_tile2_bmp_1050_1538);
        return 0x0;
      }
      if (IVar1 == 0x4) {
        return 0x1;
      }
      if (IVar1 == 0x5) {
        return 0x0;
      }
    }
    if (((uint)text & 0x2000) == 0x0) {
      return 0x0;
    }
    text = (LPCSTR)((uint)text & 0xdfef | 0x1010);
  } while( true );
}



bool __stdcall16far mem_op_1000_21b6(UINT16 param_1,UINT16 param_2)

{
  BOOL16 BVar1;
  
  BVar1 = mem_op_1000_1dfa(0x0,0x4,param_1,param_2);
  return BVar1 == 0x0;
}


