
astruct_18 * __stdcall16far pass1_1020_7526(astruct_18 *param_1,byte param_2,ushort param_3)

{
  palette_op_1020_7270(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1020_7554(ushort param_1,astruct_20 *param_2,ushort param_3,ushort param_4)

{
  uchar *extraout_DX;
  undefined2 uVar1;
  astruct_129 *iVar2;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  unk_draw_op_1020_7f7a(param_2,0x5,CONCAT22(param_4,param_3));
  uVar2 = (undefined2)((ulong)param_2 >> 0x10);
  iVar2 = (astruct_129 *)param_2;
  iVar2->field_0xee = 0x0;
  *(undefined4 *)&iVar2->field_0xf2 = 0x0;
  param_2->field_0x0 = 0x7780;
  iVar2->field_0x2 = 0x1020;
  iVar2->field_0xe2 = 0x781c;
  iVar2->field_0xe4 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x25,param_1,extraout_DX,unaff_DI);
  uVar1 = (undefined2)((ulong)puVar3 >> 0x10);
  iVar2->field_0xf2 = (int)puVar3;
  iVar2->field_0xf4 = uVar1;
  iVar2->field_0xe6 = iVar2->field_0xf2;
  iVar2->field_0xe8 = uVar1;
  return;
}



void __stdcall16far pass1_1020_75c4(ushort *param_1)

{
  astruct_586 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_586 *)param_1;
  *param_1 = 0x7780;
  iVar1->field_0x2 = 0x1020;
  iVar1->field_0xe2 = 0x781c;
  iVar1->field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_1020_75f0(ulong param_1,UINT16 param_2)

{
  UINT16 *pUVar1;
  code **ppcVar2;
  uint uVar3;
  ulong uVar4;
  uchar *puVar5;
  uchar *puVar6;
  astruct_283 *iVar7;
  undefined2 uVar7;
  ushort *puVar8;
  undefined4 *puStack10;
  undefined local_6 [0x4];
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar7 = (astruct_283 *)param_1;
  if (iVar7->field_0xee != (undefined4 *)0x0) {
    ppcVar2 = (code **)((int)*iVar7->field_0xee + 0x8);
    (**ppcVar2)();
  }
  if (iVar7->field_0xea == 0x0) {
    iVar7->field_0xea = 0x1;
    puVar8 = pass1_1008_941a((ushort *)CONCAT22(param_2,local_6),0x1,0x91);
    puVar5 = (uchar *)((ulong)puVar8 >> 0x10);
    uVar4 = ZEXT24(local_6);
    win_1008_5c9e(_PTR_LOOP_1050_02a0,(ulong *)CONCAT22(param_2,local_6),local_6,puVar5,param_2);
    iVar7->field_0xec = (int)uVar4;
    mem_op_1000_179c(0x112,puVar5,0x1000);
    puVar6 = (uchar *)((uint)puVar5 | (uint)uVar4);
    if (puVar6 == (uchar *)0x0) {
      uVar3 = 0x0;
      puStack10 = (undefined4 *)0x0;
    }
    else {
      pUVar1 = &iVar7->field_0xcc;
      *pUVar1 = *pUVar1 + 0x1;
      struct_1020_3644((ushort *)(uVar4 & 0xffff | ZEXT24(puVar5) << 0x10),iVar7->field_0xcc,param_1,param_2);
      uVar3 = (uint)uVar4;
      puStack10 = (undefined4 *)(uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
    }
    pass1_1008_6978(param_1,0x0,(ulong)puStack10 & 0xffff0000 | (ulong)uVar3,uVar3,puVar6);
    ppcVar2 = (code **)((int)*puStack10 + 0xc);
    (**ppcVar2)();
  }
  return;
}



void __stdcall16far window_op_1020_76aa(astruct *param_1)

{
  astruct_666 *in_AX;
  uchar *in_DX;
  UINT32 uVar3;
  int iVar1;
  int unaff_DI;
  UINT16 uVar2;
  ushort unaff_SS;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar2 = (UINT16)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar1 + 0xf2),*(ushort *)(iVar1 + 0x8),0x1018);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar3 = (uint)in_DX | (uint)in_AX;
  if (uVar3 != 0x0) {
    pass1_1020_7824(in_AX,(ushort)in_DX,*(ushort *)(iVar1 + 0x8),unaff_DI,unaff_SS);
    *(astruct_666 **)(iVar1 + 0xee) = in_AX;
    *(UINT32 *)(iVar1 + 0xf0) = uVar3;
    return;
  }
  *(undefined4 *)(iVar1 + 0xee) = 0x0;
  return;
}



void __stdcall16far pass1_1020_770e(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  uint uVar5;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xee);
  uVar2 = *(uint *)(iVar4 + 0xf0);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar4 + 0xee) = 0x0;
  destroy_win_1008_628e(param_1 & 0xffff | (ulong)uVar5 << 0x10,0x1008);
  return;
}



astruct_18 * __stdcall16far pass1_1020_774c(astruct_18 *param_1,byte param_2)

{
  param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  pass1_1020_75c4((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_7824(astruct_666 *param_1,ushort param_2,ushort param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  uchar *extraout_DX;
  undefined2 uVar4;
  ushort *puVar5;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0x14 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x7902;
  param_1->field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x25,param_5,extraout_DX,param_4);
  uVar4 = (undefined2)((ulong)puVar5 >> 0x10);
  param_1->field_0x14 = (int)puVar5;
  param_1->field_0x16 = uVar4;
  param_1->field_0x6 = param_1->field_0x14;
  param_1->field_0x8 = uVar4;
  uVar2 = *(undefined4 *)&param_1->field_0x14;
  iVar3 = &param_1->field_0xa;
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1->field_0x12 = iVar3;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1010,param_5);
  return;
}



void __stdcall16far pass1_1020_78ac(ushort *param_1,ushort param_2)

{
  astruct_587 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_587 *)param_1;
  *param_1 = 0x7902;
  iVar1->field_0x2 = 0x1020;
  if (iVar1->field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



astruct_18 * __stdcall16far pass1_1020_78dc(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1020_78ac(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1020_790e(ushort *param_1,ulong param_2,UINT16 param_3,ulong param_4,UINT16 param_5)

{
  astruct_271 *iVar1;
  ushort uVar1;
  
  unk_draw_op_1008_7f62(param_1,param_3,param_4,param_5);
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_271 *)param_1;
  iVar1->field_0xe0 = 0x0;
  iVar1->field_0xe4 = 0x0;
  iVar1->field_0xe8 = 0x0;
  iVar1->field_0xec = 0x0;
  iVar1->field_0xee = param_2;
  *param_1 = 0x7b86;
  iVar1->field_0x2 = 0x1020;
  return;
}



void __stdcall16far cleanup_menu_ui_op_1020_795c(astruct_3 *in_struct_1,HMENU16 in_menu_handle_2)

{
  astruct_3 *local_struct_1;
  astruct_3 *uVar1;
  
  uVar1 = (astruct_3 *)((ulong)in_struct_1 >> 0x10);
  local_struct_1 = (astruct_3 *)in_struct_1;
  in_struct_1->address_offset_field_0x0 = 0x7b86;
  local_struct_1->address_offset_field_0x2 = 0x1020;
  if (local_struct_1->field_0xec != 0x0) {
    DestroyMenu16(in_menu_handle_2);
  }
  pass1_1008_57c4((ushort *)((ulong)in_struct_1 & 0xffff0000 | (ulong)(uint)&local_struct_1->field_0xd2));
  in_struct_1->address_offset_field_0x0 = 0x380a;
  local_struct_1->address_offset_field_0x2 = 0x1008;
  in_struct_1->address_offset_field_0x0 = 0x389a;
  local_struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



ushort __stdcall16far pass1_1020_79ae(void)

{
  return 0x1;
}



void __stdcall16far string_1020_79b4(ushort param_1,ulong param_2,int param_3,char *param_4)

{
  unk_str_op_1000_3d3e((char *)(param_2 & 0xffff0000 | (ulong)((int)param_2 + 0xa)),param_4);
  if (param_3 != 0x0) {
    draw_op_1020_7cc8(*(ULONG *)((int)param_2 + 0xe8),0x1000,param_1);
  }
  return;
}



void __stdcall16far pass1_1020_79e4(ulong param_1,ushort param_2,UINT16 param_3)

{
  draw_op_1020_7cc8(*(ULONG *)((int)param_1 + 0xe8),param_2,param_3);
  return;
}



void __stdcall16far post_win_msg_1020_79fc(astruct_69 *param_1,ushort param_2,ushort param_3,int param_4,HWND16 param_5)

{
  undefined4 *puVar1;
  code **ppcVar2;
  int iVar3;
  astruct_69 *iVar4;
  undefined2 uVar4;
  undefined2 uVar5;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_69 *)param_1;
  ppcVar2 = (code **)((int)*iVar4->field_0xe0 + 0x24);
  iVar3 = (**ppcVar2)(param_5,iVar4->field_0xe0);
  if (iVar3 != param_4) {
    uVar5 = iVar4->field_0x8;
    PostMessage16(param_5,0x0,0x0,0x850000);
    puVar1 = iVar4->field_0xe0;
    ppcVar2 = (code **)((int)*iVar4->field_0xe0 + 0x28);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)puVar1,(int)((ulong)puVar1 >> 0x10),(char)param_4,uVar5);
  }
  return;
}



void __stdcall16far get_win_ui_info_op_1020_7a50(ulong param_1,HWND16 param_2)

{
  code **ppcVar1;
  BOOL16 b_var2;
  INT16 IVar2;
  INT16 IVar3;
  ushort var5;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  local_a.x = 0x0;
  local_a.y = 0x0;
  iStack6 = 0x0;
  iStack4 = 0x0;
  var5 = (ushort)(param_1 >> 0x10);
  b_var2 = IsIconic16(param_2);
  if (b_var2 == 0x0) {
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
    iStack6 = iStack6 - local_a.x;
    iStack4 = iStack4 - local_a.y;
    IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    local_a.x = local_a.x + IVar2 * 0x2;
    local_a.y = local_a.y + IVar3 * 0x2;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xe0) + 0x14);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,*(undefined4 *)((int)param_1 + 0xe0),&local_a);
  return;
}



void __stdcall16far win_ui_menu_op_1020_7ad2(ulong param_1,HWND16 param_2,RECT16 *param_3,HWND16 param_4)

{
  HMENU16 HVar1;
  int iVar2;
  undefined2 uVar3;
  POINT16 local_6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(long *)(iVar2 + 0xee) != 0x0) && (*(int *)(iVar2 + 0xec) == 0x0)) {
    HVar1 = LoadMenu16(param_4,(LPCSTR)*(undefined4 *)(iVar2 + 0xee));
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
  TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x0,*(INT16 *)(iVar2 + 0x8),0x0,local_6.y,(RECT16 *)local_6.x);
  return;
}



astruct_3 * __stdcall16far pass1_1020_7b60(astruct_3 *param_1,byte param_2,ushort param_3)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far get_sys_metrics_1020_7c1a(ushort *param_1,ulong param_2,INT16 param_3)

{
  INT16 IVar1;
  astruct_56 *iVar3;
  ushort uVar3;
  ushort uVar4;
  ushort uVar1;
  
  uVar3 = (ushort)(param_2 >> 0x10);
  uVar1 = *(ushort *)((int)param_2 + 0x8);
  uVar4 = (ushort)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_56 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = uVar1;
  *param_1 = 0x3ab0;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x6 = param_2;
  iVar3->field_0xa = 0x0;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x10 = 0x0;
  iVar3->field_0x12 = 0x0;
  *param_1 = 0x7f72;
  iVar3->field_0x2 = 0x1020;
  iVar3->field_0xa = *(undefined4 *)((int)param_2 + 0xe4);
  IVar1 = GetSystemMetrics16(param_3);
  iVar3->field_0xe = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x10 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x12 = IVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far draw_op_1020_7cc8(ULONG param_1,HWND16 in_win_handle_2,UINT16 param_3)

{
  code **ppcVar1;
  RECT16 *rect;
  COLORREF color;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  char *count;
  LPCSTR str;
  undefined4 *puVar2;
  ushort in_DX;
  char *str_00;
  astruct_6 *iVar4;
  int iVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  DWORD DVar6;
  ulong uVar7;
  ulong uVar8;
  HBRUSH16 hbrush;
  undefined4 uVar9;
  HDC16 HVar10;
  undefined2 uVar11;
  int iStack66;
  undefined2 local_20;
  int iStack30;
  int iStack28;
  int iStack26;
  int iStack24;
  int iStack22;
  RECT16 local_rect_1;
  int iStack16;
  int iStack14;
  HPALETTE16 HStack12;
  astruct_13 *paStack10;
  HDC16 local_hdc_1;
  BOOL16 is_iconic;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_6 *)param_1;
  is_iconic = IsIconic16(in_win_handle_2);
  if ((is_iconic == 0x0) || (PTR_LOOP_1050_0010 != (undefined *)0x0)) {
    local_hdc_1 = GetWindowDC16((HWND16)s_tile2_bmp_1050_1538);
    paStack10 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
    HStack12 = palette_op_1008_4e08(paStack10,&local_hdc_1,in_DX,0x1008);
    uVar11 = iVar4->field_0x4;
    GetWindowRect16(0x1008,&local_rect_1);
    iStack28 = (iStack16 - local_rect_1.x) + -0x1;
    iStack24 = (iStack14 - local_rect_1.y) + -0x1;
    local_20 = iVar4->field_0x10;
    iStack30 = iVar4->field_0x12;
    iStack26 = iStack24;
    if (is_iconic == 0x0) {
      iStack26 = iVar4->field_0xe - iVar4->field_0x12;
    }
    uVar9 = CONCAT22(param_3,&local_20);
    hbrush = 0x4;
    HVar10 = local_hdc_1;
    iStack22 = iStack28;
    rect = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    FillRect16((HDC16)s_tile2_bmp_1050_1538,rect,hbrush);
    puVar2 = iVar4->field_0x6;
    uVar5 = (undefined2)((ulong)puVar2 >> 0x10);
    iVar3 = (int)puVar2;
    puVar2 = (undefined4 *)*(undefined4 *)(iVar3 + 0xe0);
    ppcVar1 = (code **)((int)*puVar2 + 0x24);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puVar2,*(undefined2 *)(iVar3 + 0xe2),0x0,uVar9,HVar10,uVar11);
    color = (-(uint)((int)puVar2 == 0x0) & 0x1e) + 0x25;
    handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,color,0x100);
    handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,0x0,0x0);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,0x0,iStack22);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack24,iStack22);
    uVar7 = (ulong)local_hdc_1 << 0x10;
    LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack24,0x0);
    uVar8 = uVar7 & 0xffff0000 | (ulong)local_hdc_1;
    uVar7 = 0x0;
    count = (char *)LineTo16((HDC16)s_tile2_bmp_1050_1538,0x0,0x0);
    if (is_iconic == 0x0) {
      iVar3 = iVar4->field_0xe - iVar4->field_0x12;
      uVar7 = (ulong)local_hdc_1 << 0x10;
      MoveTo16((HDC16)s_tile2_bmp_1050_1538,iVar3,0x0);
      uVar7 = uVar7 & 0xffff0000 | (ulong)local_hdc_1;
      count = (char *)LineTo16((HDC16)s_tile2_bmp_1050_1538,iVar3,iStack22);
    }
    ppcVar1 = (code **)((int)*iVar4->field_0x6 + 0x18);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,iVar4->field_0x6,uVar7,uVar8);
    if (*count != '\0') {
      SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
      SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color);
      str = (LPCSTR)lstrlen16((LPCSTR)s_tile2_bmp_1050_1538);
      DVar6 = GetTextExtent16((HDC16)s_tile2_bmp_1050_1538,str,(INT16)count);
      iVar3 = (int)(DVar6 >> 0x10);
      if (is_iconic == 0x0) {
        iStack66 = (iStack26 - iStack30) / 0x2 - iVar3 / 0x2;
      }
      else {
        iStack66 = iStack24 / 0x2 - iVar3 / 0x2;
      }
      TextOut16((HDC16)s_tile2_bmp_1050_1538,(INT16)str,(INT16)count,str_00,iStack66);
    }
    HStack12 = SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,HStack12);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,local_hdc_1);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_7f38(astruct_18 *param_1,byte param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x3ab0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_draw_op_1020_7f7a(astruct_20 *param_1,UINT16 param_2,ULONG param_3)

{
  undefined2 uVar1;
  HGDIOBJ16 HVar2;
  HCURSOR16 HVar3;
  uchar *puVar4;
  astruct_20 *iVar4;
  int unaff_DI;
  undefined2 uVar5;
  UINT16 unaff_SS;
  astruct_20 *paVar6;
  ushort *puVar7;
  undefined2 in_stack_0000000e;
  
  paVar6 = unk_draw_op_1008_61b2(param_1,param_2,(UINT16)param_3,CONCAT22(in_stack_0000000e,param_3._2_2_),unaff_SS);
  puVar4 = (uchar *)((ulong)paVar6 >> 0x10);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  ((astruct_20 *)(iVar4 + 0x1))->field_0x0 = 0x389a;
  iVar4[0x1].field_0x2 = 0x1008;
  ((astruct_20 *)(iVar4 + 0x1))->field_0x0 = 0x3aa8;
  iVar4[0x1].field_0x2 = 0x1008;
  iVar4[0x1].field_0x4 = 0x0;
  iVar4[0x1].field_0x8 = 0x0;
  iVar4[0x1].field_0xa = 0x0;
  param_1->field_0x0 = 0x82bc;
  iVar4->field_0x2 = 0x1020;
  ((astruct_20 *)(iVar4 + 0x1))->field_0x0 = 0x8358;
  iVar4[0x1].field_0x2 = 0x1020;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x5b)),s_VrMode_1050_4422);
  HVar2 = GetStockObject16(0x1000);
  iVar4->hgdiobj_field_0xc6 = HVar2;
  HVar3 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar4->hcursor_field_0xc4 = HVar3;
  iVar4->field_0xc8 = 0x2028;
  iVar4->field_0xac = 0x47000000;
  iVar4->field_0xbc = *(UINT16 *)(param_3._2_2_ + 0x8);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,puVar4,unaff_DI);
  uVar1 = (undefined2)((ulong)puVar7 >> 0x10);
  iVar4->field_0xb4 = 0x0;
  iVar4->field_0xb6 = 0x0;
  iVar4->field_0xb8 = *(UINT16 *)((int)puVar7 + 0xa);
  iVar4->field_0xba = *(UINT16 *)((int)puVar7 + 0xc);
  iVar4->field_0xca = (UINT16)param_3;
  win_ui_reg_class_1008_96d2(param_1,0x1008,unaff_SS);
  return;
}



void __stdcall16far pass1_1020_808e(ushort *param_1)

{
  undefined2 *puVar1;
  undefined2 uVar2;
  astruct_574 *iVar3;
  undefined2 uVar3;
  undefined2 *puStack6;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_574 *)param_1;
  *param_1 = 0x82bc;
  iVar3->field_0x2 = 0x1020;
  iVar3->field_0xe2 = 0x8358;
  iVar3->field_0xe4 = 0x1020;
  if (param_1 == (ushort *)0x0) {
    puVar1 = (undefined2 *)0x0;
    uVar2 = 0x0;
  }
  else {
    puVar1 = &iVar3->field_0xe2;
    uVar2 = uVar3;
  }
  puStack6 = (undefined2 *)CONCAT22(uVar2,puVar1);
  *puStack6 = 0x389a;
  puVar1[0x1] = 0x1008;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0xd2));
  *param_1 = 0x380a;
  iVar3->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1020_8106(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x60);
  (**ppcVar1)();
  return;
}



void __stdcall16far realize_palette_1020_8128(ulong param_1,int param_2,HGDIOBJ16 param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  undefined *puVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  uint extraout_DX;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined local_12 [0x8];
  undefined2 uStack10;
  undefined2 uStack8;
  undefined4 *puStack6;
  
  if (param_2 != 0x0) {
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    uVar2 = *(undefined4 *)(iVar6 + 0xe6);
    uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar7 = (int)uVar2;
    puVar5 = (undefined4 *)*(undefined4 *)(iVar7 + 0xa);
    ppcVar1 = (code **)((int)*puVar5 + 0x18);
    puStack6 = puVar5;
    (**ppcVar1)(param_3,(int)puVar5,*(undefined2 *)(iVar7 + 0xc));
    uStack8 = SUB42(puVar5,0x0);
    UnrealizeObject16(param_3);
    uVar2 = *(undefined4 *)(iVar6 + 0xe6);
    uVar8 = *(undefined2 *)((int)uVar2 + 0x14);
    uStack10 = uVar8;
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    pass1_1008_57a4((ulong *)CONCAT22(param_4,local_12),param_1 & 0xffff0000 | (ulong)(iVar6 + 0xd2));
    while( true ) {
      puVar3 = local_12;
      pass1_1008_5b12(puVar3,param_4);
      if ((extraout_DX | (uint)puVar3) == 0x0) break;
      uVar9 = *(undefined2 *)(puVar3 + 0x6);
      puVar4 = (undefined4 *)*(undefined4 *)(puVar3 + 0x4);
      ppcVar1 = (code **)((int)*puVar4 + 0x90);
      (**ppcVar1)(0x1008,puVar4,uVar9,0x1,uVar8);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_palette_op_1020_81c0(HWND16 param_1)

{
  astruct_13 *in_struct_1;
  BOOL16 b_force_background;
  HPALETTE16 b_force_background_00;
  UINT16 UVar1;
  uint uVar2;
  undefined2 uVar3;
  uint uStack6;
  
  uVar3 = (undefined2)((ulong)_PTR_LOOP_1050_4230 >> 0x10);
  in_struct_1 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
  uVar2 = *(uint *)((int)_PTR_LOOP_1050_4230 + 0x10);
  uStack6 = (uint)in_struct_1;
  if ((uVar2 | uStack6) == 0x0) {
    return;
  }
  b_force_background = GetDC16(param_1);
  create_palette_1008_4e38(in_struct_1,0x1008,uVar2);
  b_force_background_00 = SelectPalette16(0x1008,0x0,b_force_background);
  UVar1 = RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x1,b_force_background_00);
  RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  if (0x0 < (int)UVar1) {
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
  }
  return;
}



void __stdcall16far destroy_window_1020_8250(ulong param_1,HWND16 param_2)

{
  BOOL16 BVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0xec) != 0x0) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 != 0x0) {
      DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
      *(undefined2 *)((int)param_1 + 0xec) = 0x0;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_8288(astruct_18 *param_1,byte param_2)

{
  param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  pass1_1020_808e((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_8360(ushort *param_1,ushort param_2)

{
  undefined4 uVar1;
  ushort uVar2;
  ushort *puVar3;
  uint uVar4;
  astruct_667 *iVar4;
  
  iVar4 = (astruct_667 *)param_1;
  uVar4 = (uint)((ulong)param_1 >> 0x10);
  struct_1020_847a(param_1,0x1,param_2);
  puVar3 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x16));
  *(undefined4 *)&iVar4->field_0x1c = 0x0;
  *param_1 = 0x8462;
  iVar4->field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_2,(uchar *)((ulong)puVar3 >> 0x10),uVar4);
  uVar2 = (ushort)((ulong)puVar3 >> 0x10);
  iVar4->field_0x1c = (int)puVar3;
  iVar4->field_0x1e = uVar2;
  pass1_1018_26f8(iVar4->field_0x1c,uVar2,(ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x16));
  uVar1 = *(undefined4 *)&iVar4->field_0x1c;
  pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10,iVar4->field_0x8,*(astruct_76 **)((int)uVar1 + 0x2a),
                  (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x16));
  return;
}



void __stdcall16far pass1_1020_83f8(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(int *)(iVar3 + 0x4) != 0x0) {
    uVar1 = *(undefined4 *)(iVar3 + 0x1c);
    uVar2 = *(undefined4 *)(iVar3 + 0x1c);
    pass1_1008_4480(*(ulong *)((int)uVar1 + 0xa),(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar3 + 0x16)),
                    *(astruct_76 **)((int)uVar2 + 0x2a),param_2);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_843c(astruct_18 *param_1,byte param_2)

{
  pass1_1020_8556(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1020_847a(undefined2 *param_1,int param_2,ushort param_3)

{
  ushort uVar1;
  uchar *puVar2;
  astruct_280 *iVar3;
  int iVar4;
  ushort *puVar5;
  
  iVar4 = (int)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_280 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = 0x0;
  iVar3->field_0x6 = param_2;
  iVar3->field_0x8 = (astruct_20 *)0x0;
  iVar3->field_0xc = (astruct_20 *)0x0;
  puVar5 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
  *param_1 = 0x87aa;
  iVar3->field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_3,(uchar *)((ulong)puVar5 >> 0x10),iVar4);
  puVar2 = (uchar *)((ulong)puVar5 >> 0x10);
  pass1_1008_3f62((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar3 + 0x1)),
                  (ushort *)((ulong)puVar5 & 0xffff0000 | (ulong)((int)puVar5 + 0xe)));
  uVar1 = iVar3->field_0x6 << 0x3;
  mem_op_1000_179c(uVar1,puVar2,0x1000);
  *(ushort *)&iVar3->field_0x8 = uVar1;
  *(uchar **)((int)&iVar3->field_0x8 + 0x2) = puVar2;
  uVar1 = iVar3->field_0x6 << 0x2;
  mem_op_1000_179c(uVar1,puVar2,0x1000);
  *(ushort *)&iVar3->field_0xc = uVar1;
  *(uchar **)((int)&iVar3->field_0xc + 0x2) = puVar2;
  pass1_1000_4906(iVar3->field_0x8,(WNDCLASS16 *)0x0,iVar3->field_0x6 << 0x3);
  pass1_1000_4906(iVar3->field_0xc,(WNDCLASS16 *)0x0,iVar3->field_0x6 << 0x2);
  return;
}



void __stdcall16far pass1_1020_8556(ushort *param_1)

{
  int *piVar1;
  uint uVar2;
  astruct_18 *paVar3;
  astruct_588 *iVar5;
  astruct_589 *iVar4;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  int iStack12;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_588 *)param_1;
  *param_1 = 0x87aa;
  iVar5->field_0x2 = 0x1020;
  fn_ptr_1000_17ce(iVar5->field_0x8,0x1000);
  if ((*(uint *)((int)&iVar5->field_0xc + 0x2) | *(uint *)&iVar5->field_0xc) != 0x0) {
    iStack12 = 0x0;
    while( true ) {
      piVar1 = &iVar5->field_0x6;
      if (*piVar1 == iStack12 || *piVar1 < iStack12) break;
      iVar6 = iStack12 * 0x4;
      paVar3 = iVar5->field_0xc;
      uVar8 = (undefined2)((ulong)paVar3 >> 0x10);
      iVar4 = (astruct_589 *)paVar3;
      if (*(long *)(iVar4 + iVar6) != 0x0) {
        paVar3 = *(astruct_18 **)(iVar4 + iVar6);
        uVar2 = *(uint *)(iVar4 + iVar6 + 0x2);
        if ((uVar2 | (uint)paVar3) != 0x0) {
          pass1_1008_5118((ulong)paVar3 & 0xffff | (ulong)uVar2 << 0x10);
          fn_ptr_1000_17ce(paVar3,0x1000);
        }
      }
      iStack12 = iStack12 + 0x1;
    }
    fn_ptr_1000_17ce(iVar5->field_0xc,0x1000);
  }
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1020_85f6(ulong param_1)

{
  int *piVar1;
  uint uVar2;
  astruct_18 *paVar3;
  undefined4 uVar4;
  int iVar5;
  astruct_590 *iVar6;
  undefined2 uVar6;
  undefined2 uVar7;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (astruct_590 *)param_1;
    piVar1 = &iVar6->field_0x6;
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar4 = iVar6->field_0xc;
    uVar6 = (undefined2)((ulong)uVar4 >> 0x10);
    iVar5 = (int)uVar4;
    paVar3 = *(astruct_18 **)(iVar5 + iStack4 * 0x4);
    uVar2 = *(uint *)(iVar5 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | (uint)paVar3) != 0x0) {
      pass1_1008_5118((ulong)paVar3 & 0xffff | (ulong)uVar2 << 0x10);
      fn_ptr_1000_17ce(paVar3,0x1000);
    }
    uVar4 = iVar6->field_0xc;
    *(undefined4 *)((int)uVar4 + iStack4 * 0x4) = 0x0;
    iStack4 = iStack4 + 0x1;
  }
  return;
}



void __stdcall16far pass1_1020_865a(ulong param_1)

{
  int *piVar1;
  uint uVar2;
  astruct_18 *paVar3;
  undefined4 uVar4;
  int iVar5;
  astruct_592 *iVar7;
  astruct_591 *iVar6;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar9 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    piVar1 = (int *)(iVar5 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    iVar8 = iStack4 * 0x4;
    uVar4 = *(undefined4 *)(iVar5 + 0xc);
    uVar10 = (undefined2)((ulong)uVar4 >> 0x10);
    iVar7 = (astruct_592 *)uVar4;
    if (*(long *)(iVar7 + iVar8) != 0x0) {
      pass1_1008_5236(*(ulong *)(iVar7 + iVar8));
      uVar4 = *(undefined4 *)(iVar5 + 0xc);
      uVar10 = (undefined2)((ulong)uVar4 >> 0x10);
      iVar6 = (astruct_591 *)uVar4;
      paVar3 = *(astruct_18 **)(iVar6 + iVar8);
      uVar2 = *(uint *)(iVar6 + iVar8 + 0x2);
      if ((uVar2 | (uint)paVar3) != 0x0) {
        pass1_1008_5118((ulong)paVar3 & 0xffff | (ulong)uVar2 << 0x10);
        fn_ptr_1000_17ce(paVar3,0x1000);
      }
      uVar4 = *(undefined4 *)(iVar5 + 0xc);
      *(undefined4 *)((int)uVar4 + iStack4 * 0x4) = 0x0;
    }
    iStack4 = iStack4 + 0x1;
  }
  return;
}
