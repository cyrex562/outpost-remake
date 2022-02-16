

/*
Unable to decompile 'FUN_1008_1df2'
Cause: 
Low-level Error: Symbol $$undef00000009 extends beyond the end of the address space
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far ui_op_1008_2c4e(int param_1,undefined2 param_2,undefined2 param_3,HINSTANCE16 in_h_instance_4)

{
  int *piVar1;
  code **ppcVar2;
  HCURSOR16 HVar3;
  HCURSOR16 HVar4;
  undefined2 in_DX;
  undefined2 uVar5;
  u16 uVar6;
  u16 unaff_SS;
  undefined4 uVar7;
  
  uVar5 = 0x0;
  HVar3 = LoadCursor16(in_h_instance_4,(LPCSTR)0x7f02);
  HVar4 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  uVar7 = CONCAT22(in_DX,HVar4);
  piVar1 = (int *)(param_1 + 0xf2);
  *piVar1 = *piVar1 + 0x1;
  uVar6 = param_1;
  if (*(long *)(param_1 + 0xee) != 0x0) {
    uVar7 = *(undefined4 *)(param_1 + 0xee);
    uVar6 = (u16)*(undefined4 *)*(undefined4 *)(param_1 + 0xee);
    ppcVar2 = (code **)(uVar6 + 0x90);
    uVar7 = (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar7,(int)((ulong)uVar7 >> 0x10),HVar3,uVar5);
  }
  big_switch_1008_15d4(uVar6,(u16)s_tile2_bmp_1050_1538,unaff_SS,CONCAT22(param_2,param_1),CONCAT22(HVar3,param_3));
  uVar5 = (undefined2)((ulong)uVar7 >> 0x10);
  *(undefined2 *)(param_1 + 0xee) = (int)uVar7;
  *(undefined2 *)(param_1 + 0xf0) = uVar5;
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xee) + 0x8);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,*(undefined2 *)(param_1 + 0xee),uVar5);
  if (*(long *)(param_1 + 0xe8) != 0x0) {
    uVar7 = *(undefined4 *)(param_1 + 0xe8);
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xe8) + 0xc);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar7,(char)((ulong)uVar7 >> 0x10),0x0);
  }
  show_win_1038_b634(_PTR_LOOP_1050_5b7c,(int)&PTR_LOOP_1050_1038);
  show_win_1010_7a76(*(ulong *)(param_1 + 0xf4),0x1010);
  uVar7 = *(undefined4 *)(param_1 + 0xee);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xee) + 0xc);
  (**ppcVar2)(0x1010,(int)uVar7,(char)((ulong)uVar7 >> 0x10),0x5);
  BringWindowToTop16(0x1010);
  SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far post_msg_1008_2d22(ulong param_1)

{
  int *piVar1;
  undefined4 uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  ushort unaff_SS;
  undefined4 uVar6;
  undefined2 uVar7;
  undefined4 uVar8;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((*(long *)(iVar4 + 0xee) != 0x0) &&
     (piVar1 = (int *)(iVar4 + 0xf2), *piVar1 = *piVar1 + -0x1, *(int *)(iVar4 + 0xf2) < 0x1)) {
    uVar8 = *(undefined4 *)(iVar4 + 0xee);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xee) + 0x90);
    (**ppcVar3)();
    *(undefined4 *)(iVar4 + 0xee) = 0x0;
    *(undefined2 *)(iVar4 + 0xf2) = 0x0;
    if (*(long *)(iVar4 + 0xe8) != 0x0) {
      uVar7 = 0x3;
      uVar6 = *(undefined4 *)(iVar4 + 0xe8);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xe8) + 0xc);
      (**ppcVar3)();
      show_win_1038_b68a(_PTR_LOOP_1050_5b7c,(int)&PTR_LOOP_1050_1038);
      show_window_1010_7ace(*(ulong *)(iVar4 + 0xf4),unaff_SS);
      uVar2 = *(undefined4 *)(iVar4 + 0xe8);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xe8) + 0x98);
      (**ppcVar3)(0x1010,(int)uVar2,(char)((ulong)uVar2 >> 0x10),0x1,uVar6,uVar7,uVar8);
      PostMessage16(0x1010,0x0,0x0,0x11100fc);
    }
  }
  return;
}



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



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far save_file_1008_3178(ulong param_1,int param_2,ushort param_3)

{
  char cVar1;
  undefined4 uVar2;
  int iVar3;
  undefined *puVar4;
  ushort uVar5;
  BOOL16 BVar6;
  uchar *in_DX;
  uint extraout_DX;
  ushort uVar7;
  int unaff_DI;
  undefined2 uVar8;
  char *pcVar9;
  INT16 in_buf_len_2;
  uint uVar10;
  char local_782 [0x104];
  undefined local_67e [0x8];
  astruct_18 *paStack1654;
  LPCSTR pCStack1650;
  UINT16 UStack1648;
  astruct_18 *paStack1646;
  undefined local_666 [0x100];
  char *pcStack1382;
  undefined4 local_562;
  undefined2 uStack1374;
  char *pcStack1370;
  ushort uStack1326;
  char acStack1305 [0x101];
  uint uStack1048;
  char local_416 [0x8];
  undefined2 uStack1038;
  undefined local_40c [0x102];
  ulong uStack778;
  ushort *puStack774;
  undefined local_302;
  undefined local_202 [0xff];
  char acStack259 [0x101];
  
  acStack259[1] = 0x0;
  local_302 = 0x0;
  local_202[0] = 0x0;
  puStack774 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,in_DX,unaff_DI);
  uVar8 = (undefined2)((ulong)puStack774 >> 0x10);
  iVar3 = (int)puStack774;
  uStack778 = *(undefined4 *)(iVar3 + 0x1a);
  uVar10 = *(uint *)(iVar3 + 0x1c);
  if ((uVar10 | (uint)uStack778) == 0x0) {
    paStack1646 = *(astruct_18 **)(iVar3 + 0x64);
    uVar10 = *(uint *)(iVar3 + 0x66);
    if ((uVar10 | (uint)paStack1646) != 0x0) {
      pass1_1008_5784((ulong *)CONCAT22(param_3,local_67e),(ulong)paStack1646 & 0xffff | (ulong)uVar10 << 0x10);
      puVar4 = local_67e;
      pass1_1008_5b12(puVar4,param_3);
      paStack1654 = (astruct_18 *)CONCAT22(extraout_DX,puVar4);
      if ((extraout_DX | (uint)puVar4) != 0x0) {
        uVar2 = *(undefined4 *)(puVar4 + 0x4);
        uStack778._0_2_ = (uint)uVar2;
        uVar10 = (uint)((ulong)uVar2 >> 0x10);
        goto LAB_1008_3206;
      }
    }
  }
  else {
LAB_1008_3206:
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,acStack259 + 0x1),(char *)CONCAT22(uVar10,(uint)uStack778));
  }
  pass1_1000_5008((ushort)local_40c,param_3,0x100,(int)&stack0xfffe);
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(param_3,local_40c));
  if (local_40c[uStack1038 - 0x1] == '\\') {
    local_40c[uStack1038 - 0x1] = 0x0;
  }
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(param_3,acStack259 + 0x1));
  if (acStack259[uStack1038] == '\\') {
    acStack259[uStack1038] = '\0';
  }
  pass1_1000_4f2e((ushort)&stack0xfffe);
  uVar8 = (undefined2)((ulong)puStack774 >> 0x10);
  uStack778 = *(ulong *)((int)puStack774 + 0x12);
  uVar10 = *(uint *)((int)puStack774 + 0x14);
  if ((uVar10 | (uint)uStack778) != 0x0) {
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_202),(char *)(uStack778 & 0xffff | (ulong)uVar10 << 0x10));
  }
  local_416[0] = '\0';
  pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_416),pcVar9);
  uStack1048 = str_op_1000_3da4((char *)CONCAT22(param_3,local_416));
  uStack1038 = uStack1048;
  for (; -0x1 < (int)uStack1048; uStack1048 = uStack1048 - 0x1) {
    if (local_416[uStack1048] == '.') {
      unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_67e),(char *)CONCAT22(param_3,local_416 + uStack1048 + 0x1));
      unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_416),(char *)CONCAT22(param_3,local_67e));
    }
  }
  acStack1305[1] = 0x0;
  pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uVar5 = (ushort)((ulong)pcVar9 >> 0x10);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_3,acStack1305 + 0x1),pcVar9);
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(param_3,acStack1305 + 0x1));
  cVar1 = acStack1305[uStack1038];
  uStack1048 = 0x0;
  while (acStack1305[uStack1048 + 0x1] != '\0') {
    if (acStack1305[uStack1048 + 0x1] == cVar1) {
      acStack1305[uStack1048 + 0x1] = '\0';
    }
    uStack1048 = uStack1048 + 0x1;
  }
  pass1_1000_4906((astruct_20 *)CONCAT22(param_3,&local_562),(WNDCLASS16 *)0x0,0x48);
  local_562 = 0x48;
  uStack1374 = *(undefined2 *)((int)param_1 + 0x8);
  pcStack1370 = acStack1305 + 0x1;
  pcStack1382 = (char *)0x0;
  local_666[0] = 0x0;
  in_buf_len_2 = (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
  if (param_2 == 0x1) {
    pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,in_buf_len_2,0x1010);
    uVar5 = (ushort)((ulong)pcVar9 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_666),pcVar9);
    BVar6 = GetOpenFileName16(0x1000);
  }
  else {
    if (param_2 != 0x2) {
      debug_print_1008_6048((ulong)s_Unsupported_FileStructType_in_Op_1050_01ca,0x1000,param_3);
      goto LAB_1008_3461;
    }
    pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,in_buf_len_2,0x1010);
    uVar5 = (ushort)((ulong)pcVar9 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_666),pcVar9);
    BVar6 = GetSaveFileName16(0x1000);
  }
  if (BVar6 != 0x0) {
    pcStack1382 = (char *)CONCAT22(param_3,local_202);
  }
LAB_1008_3461:
  if (pcStack1382 != (char *)0x0) {
    if ((int)uStack1326 < 0x0) {
      paStack1654 = (astruct_18 *)
                    load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      uVar7 = (ushort)((ulong)paStack1654 >> 0x10);
      uVar5 = str_op_1008_60e8((char *)paStack1654,uVar7);
      paStack1654 = (astruct_18 *)CONCAT22(uVar7,uVar5);
      pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      UStack1648 = (UINT16)((ulong)pcVar9 >> 0x10);
      pCStack1650 = (LPCSTR)pcVar9;
      MessageBox16(0x1010,(LPCSTR)&PTR_LOOP_1050_0010,pCStack1650,UStack1648);
      pcStack1382 = (char *)0x0;
      paStack1646 = paStack1654;
      fn_ptr_1000_17ce(paStack1654,0x1000);
    }
    else {
      str_op_1000_3dbe((char *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,local_782)),
                       (char *)CONCAT22(param_3,local_202),uStack1326);
      local_782[uStack1326] = '\0';
      if (local_782[0] != '\0') {
        pass1_1010_60cc((ulong)puStack774,(char *)CONCAT22(param_3,local_782),uVar5);
      }
    }
  }
  pass1_1000_4f2e((ushort)&stack0xfffe);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far set_sys_color_1008_357e(ulong param_1,int param_2,INT16 in_index_3,ushort param_4)

{
  ushort uVar1;
  COLORREF colorref_var2;
  int iVar2;
  astruct_53 *iVar3;
  int iVar4;
  astruct_53 *uVar6;
  undefined2 uVar5;
  INT16 count;
  undefined4 uVar7;
  int iStack132;
  undefined4 local_80;
  undefined2 uStack124;
  undefined2 uStack122;
  undefined2 uStack120;
  undefined2 uStack118;
  undefined2 uStack116;
  undefined2 uStack114;
  undefined4 uStack112;
  undefined4 uStack108;
  undefined2 uStack104;
  undefined2 uStack102;
  undefined2 uStack100;
  undefined2 uStack98;
  undefined2 uStack96;
  undefined2 uStack94;
  undefined2 uStack92;
  undefined2 uStack90;
  undefined4 uStack88;
  undefined4 uStack84;
  undefined2 uStack80;
  undefined2 uStack78;
  undefined4 uStack76;
  undefined4 uStack72;
  undefined4 uStack68;
  undefined4 uStack64;
  undefined4 uStack60;
  undefined4 uStack56;
  undefined4 uStack52;
  undefined4 uStack48;
  undefined4 local_2c;
  undefined4 uStack40;
  undefined4 uStack36;
  undefined4 uStack32;
  undefined4 uStack28;
  undefined4 uStack24;
  undefined4 uStack20;
  undefined4 uStack16;
  undefined4 uStack12;
  undefined4 uStack8;
  undefined2 uStack4;
  
  local_2c = 0x70004;
  uStack40 = 0xf0000;
  uStack36 = 0x100014;
  uStack32 = 0xd0012;
  uStack28 = 0x6000e;
  uStack24 = 0x80005;
  uStack20 = 0x10011;
  uStack16 = 0x30002;
  uStack12 = 0xa0009;
  uStack8 = 0xc000b;
  uStack4 = 0x13;
  local_80 = 0x0;
  uStack108 = 0x808080;
  iVar2 = 0x100;
  uStack116 = 0x0;
  uStack114 = 0x100;
  uStack100 = 0x0;
  uStack98 = 0x100;
  uStack96 = 0xffff;
  uStack94 = 0x0;
  uStack124 = 0x2;
  uStack122 = 0x100;
  uStack120 = 0x2;
  uStack118 = 0x100;
  uStack104 = 0x2;
  uStack102 = 0x100;
  uStack92 = 0x2;
  uStack90 = 0x100;
  uStack88 = 0x0;
  uStack80 = 0xc0c0;
  uStack78 = 0x0;
  uStack76 = 0x0;
  uStack72 = 0x0;
  uStack68 = 0x0;
  uStack52 = 0x0;
  uVar1 = 0x8000;
  uStack112 = 0x8000;
  uStack84 = 0x8000;
  uStack64 = 0x8000;
  uStack60 = 0x8000;
  uStack56 = 0x8000;
  uStack48 = 0x8000;
  uVar6 = (astruct_53 *)(param_1 >> 0x10);
  iVar3 = (astruct_53 *)param_1;
  if (*(long *)&iVar3->field_0xf8 == 0x0) {
    mem_op_1000_179c(0x54,(uchar *)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),0x1000);
    iVar3->field_0xf8 = uVar1;
    iVar3->field_0xfa = iVar2;
    in_index_3 = 0x1000;
    for (iStack132 = 0x0; iStack132 < 0x15; iStack132 = iStack132 + 0x1) {
      colorref_var2 = GetSysColor16(in_index_3);
      uVar7 = *(undefined4 *)&iVar3->field_0xf8;
      uVar5 = (undefined2)((ulong)uVar7 >> 0x10);
      iVar4 = (int)uVar7;
      *(COLORREF *)(iVar4 + iStack132 * 0x4) = colorref_var2;
      *(int *)(iVar4 + iStack132 * 0x4 + 0x2) = iVar2;
      in_index_3 = (INT16)s_tile2_bmp_1050_1538;
    }
  }
  count = in_index_3;
  if (param_2 != 0x0) {
    count = (INT16)s_tile2_bmp_1050_1538;
    colorref_var2 = GetSysColor16(in_index_3);
    if ((colorref_var2 == (COLORREF)local_80) && (iVar2 == local_80._2_2_)) {
      return;
    }
  }
  if (PTR_LOOP_1050_0010 == (undefined *)0x0) {
    uStack112 = 0xc0c0c0;
  }
  if (param_2 == 0x0) {
    uVar7 = *(undefined4 *)&iVar3->field_0xf8;
  }
  else {
    uVar7 = CONCAT22(param_4,&local_80);
  }
  SetSysColors16(count,(INT16 *)uVar7,(COLORREF *)((ulong)uVar7 >> 0x10));
  return;
}



void __stdcall16far pass1_1008_3714(ulong param_1)

{
  pass1_1008_3e0e(param_1);
  return;
}



ulong __stdcall16far pass1_1008_372c(int param_1,ushort param_2)

{
  return CONCAT22(param_2,param_1 + 0xa);
}



void __stdcall16far pass1_1008_373c(void)

{
  return;
}



void __stdcall16far pass1_1008_3740(void)

{
  return;
}



void __stdcall16far pass1_1008_3744(void)

{
  return;
}



void __stdcall16far pass1_1008_3748(void)

{
  return;
}



void __stdcall16far pass1_1008_374c(void)

{
  return;
}



void __stdcall16far pass1_1008_3750(void)

{
  return;
}



void __stdcall16far pass1_1008_3754(void)

{
  return;
}



ushort __stdcall16far pass1_1008_3758(void)

{
  return 0x1;
}



void __stdcall16far pass1_1008_375e(void)

{
  return;
}



void __stdcall16far pass1_1008_3762(void)

{
  return;
}



void __stdcall16far pass1_1008_3766(void)

{
  return;
}



void __stdcall16far pass1_1008_377a(void)

{
  return;
}



ushort * __stdcall16far pass1_1008_377e(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_37aa(ushort *param_1,byte param_2)

{
  astruct_450 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_450 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_37e4(ulong param_1,byte param_2)

{
  cleanup_ui_op_1008_0618(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_392e(ushort *param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa8;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *(ushort *)(iVar1 + 0x4) = param_2;
  *param_1 = 0x3ab0;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa0;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return param_1;
}



void __stdcall16far pass1_1008_397a(ushort *param_1)

{
  astruct_452 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_452 *)param_1;
  *param_1 = 0x3aa0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



// l

void __stdcall16far fill_rect_1008_39ac(HWND16 in_win_handle_1)

{
  RECT16 local_brush_handle [0x2];
  RECT16 *local_brush_handle_2;
  HDC16 HStack36;
  PAINTSTRUCT16 local_paint_struct;
  
  HStack36 = BeginPaint16(in_win_handle_1,&local_paint_struct);
  local_brush_handle_2 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_brush_handle);
  FillRect16((HDC16)s_tile2_bmp_1050_1538,local_brush_handle_2,(HBRUSH16)local_brush_handle);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_paint_struct);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return;
}



void __stdcall16far pass1_1008_3a10(void)

{
  return;
}



ushort * __stdcall16far pass1_1008_3a14(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_3a40(ushort *param_1,byte param_2)

{
  astruct_451 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_451 *)param_1;
  *param_1 = 0x3ab0;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_3a7a(ulong param_1,byte param_2)

{
  pass1_1008_397a((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_20 * __stdcall16far pass1_1008_3ab8(astruct_20 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  set_struct_1008_687a(param_1,0x0);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0xde) = 0x0;
  param_1->field_0x0 = 0x3b46;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x5b)),s_SOLDefaultWindowClass_1050_01fe);
  return param_1;
}



void __stdcall16far post_quit_msg_1008_3af4(short exit_code)

{
  PostQuitMessage16(exit_code);
  return;
}



void __stdcall16far pass1_1008_3afe(ushort *param_1,byte param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xd2)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}
