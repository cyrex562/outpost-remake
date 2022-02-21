
void __stdcall16far begin_end_paint_1008_97c8(HWND16 param_1)

{
  PAINTSTRUCT16 local_22;
  
  BeginPaint16(param_1,&local_22);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}

void __stdcall16far get_stock_obj_1008_9c56(INT16 param_1)

{
  GetStockObject16(param_1);
  return;
}


astruct_23 * __stdcall16far unk_draw_op_1008_80ee(astruct_23 *param_1,UINT16 param_2)

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  astruct_23 *iVar3;
  astruct_23 *uVar3;
  
  uVar3 = (astruct_23 *)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_23 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x54 = 0x0;
  iVar3->field_0x56 = 0x0;
  iVar3->field_0x58 = 0x0;
  param_1->field_0x0 = 0x87c8;
  iVar3->field_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x4)),s_MicroSpinControl_1050_0370);
  iVar3->field_0x54 = 0x3;
  HVar1 = LoadCursor16(0x1000,(LPCSTR)0x7f00);
  iVar3->field_0x58 = HVar1;
  HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x56 = HVar2;
  pass1_1008_818c((astruct_23 *)((ulong)param_1 & 0xffff | ZEXT24(uVar3) << 0x10),(int)s_tile2_bmp_1050_1538,param_2);
  return param_1;
}


void __stdcall16far pass1_1008_818c(astruct_23 *param_1,HINSTANCE16 param_2,WNDCLASS16 *param_3)

{
  BOOL16 BVar1;
  ATOM AVar2;
  undefined2 local_1c;
  undefined2 uStack26;
  undefined2 uStack24;
  undefined4 uStack22;
  undefined *puStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  undefined2 uStack12;
  undefined4 uStack10;
  int iStack6;
  undefined2 uStack4;
  
  iStack6 = (int)param_1 + 0x4;
  BVar1 = GetClassInfo16(param_2,(SEGPTR)&local_1c,param_3);
  if (BVar1 == 0x0) {
    local_1c = *(undefined2 *)((int)param_1 + 0x54);
    uStack26 = 0x84f2;
    uStack24 = 0x1008;
    uStack22 = 0x40000;
    puStack18 = PTR_LOOP_1050_038c;
    uStack16 = 0x0;
    uStack14 = *(undefined2 *)((int)param_1 + 0x58);
    uStack12 = *(undefined2 *)((int)param_1 + 0x56);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far draw_op_1008_8288(ushort param_1,ulong param_2,HWND16 param_3)

{
  HGDIOBJ16 HVar1;
  HGDIOBJ16 HVar2;
  int x;
  undefined2 uVar3;
  RECT16 local_58;
  uint uStack84;
  uint uStack82;
  HBRUSH16 HStack80;
  HPEN16 HStack78;
  HPEN16 HStack76;
  HDC16 HStack74;
  uint uStack72;
  uint uStack70;
  uint uStack68;
  uint uStack66;
  uint uStack64;
  uint uStack62;
  PAINTSTRUCT16 local_3c;
  int local_1c;
  int iStack26;
  int iStack24;
  int iStack22;
  int iStack20;
  int iStack18;
  int local_10;
  int iStack14;
  int iStack12;
  int iStack10;
  int iStack8;
  int iStack6;
  uint uStack4;
  
  HStack74 = BeginPaint16(param_3,&local_3c);
  uStack4 = 0x0;
  HStack76 = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)_PTR_LOOP_1050_0368,
                         (COLORREF)((ulong)_PTR_LOOP_1050_0368 >> 0x10));
  HStack78 = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)DAT_1050_0364,(COLORREF)((ulong)DAT_1050_0364 >> 0x10));
  HStack80 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_58);
  uStack62 = uStack84;
  uStack64 = uStack82;
  uStack66 = uStack84 >> 0x1;
  uStack68 = uStack82 >> 0x1;
  uStack70 = uStack84 >> 0x2;
  uStack72 = uStack82 >> 0x2;
  HVar1 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar1 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar1);
  HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar2 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar2);
  Rectangle16((HDC16)s_tile2_bmp_1050_1538,uStack82,uStack84,local_58.y,local_58.x);
  MoveTo16((HDC16)s_tile2_bmp_1050_1538,uStack68,0x0);
  LineTo16((HDC16)s_tile2_bmp_1050_1538,uStack68,uStack62);
  uVar3 = (undefined2)(param_2 >> 0x10);
  if ((*(byte *)((int)param_2 + 0x4) & 0x4) != 0x0) {
    uStack4 = 0x1;
  }
  local_10 = uStack66 + uStack4;
  iStack14 = uStack72 + uStack4 + -0x2;
  iStack12 = local_10 + -0x3;
  iStack10 = uStack72 + uStack4 + 0x1;
  iStack8 = local_10 + 0x3;
  iStack6 = iStack10;
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack76);
  if (uStack4 == 0x0) {
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,uStack68 - 0x2,0x1);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,0x1,0x1);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,0x1,uStack62 - 0x1);
  }
  uStack4 = (uint)((*(byte *)((int)param_2 + 0x4) & 0x8) != 0x0);
  local_1c = uStack66 + uStack4;
  iStack22 = (uStack64 - uStack72) + uStack4;
  iStack26 = iStack22 + 0x1;
  iStack24 = local_1c + -0x3;
  iStack22 = iStack22 + -0x2;
  iStack20 = local_1c + 0x3;
  iStack18 = iStack22;
  if (uStack4 == 0x0) {
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,uStack82 - 0x2,0x1);
    x = uStack68 + 0x1;
    LineTo16((HDC16)s_tile2_bmp_1050_1538,x,0x1);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,x,uStack62 - 0x1);
  }
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack78);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack80);
  Polygon16((HDC16)s_tile2_bmp_1050_1538,(POINT16 *)((int)&PTR_LOOP_1050_0002 + 0x1),(INT16)&local_10);
  Polygon16((HDC16)s_tile2_bmp_1050_1538,(POINT16 *)((int)&PTR_LOOP_1050_0002 + 0x1),(INT16)&local_1c);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar2);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar1);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_3c);
  return;
}


astruct_20 * __stdcall16far
unk_draw_op_1008_61b2(astruct_20 *param_1,UINT16 param_2,UINT16 param_3,ULONG param_4,UINT16 param_5)

{
  HGDIOBJ16 l_hgdiobj_1;
  HCURSOR16 l_hcursor_1;
  uchar *extraout_DX;
  uchar *puVar1;
  int unaff_DI;
  UINT16 *l_struct_2;
  astruct_20 *uVar5;
  astruct_20 *iVar1;
  astruct_20 *iVar4;
  UINT16 *uVar1;
  
  iVar1 = (astruct_20 *)param_1;
  uVar5 = (astruct_20 *)((ulong)param_1 >> 0x10);
  set_struct_1008_687a(param_1,param_4);
  iVar1->field_0xde = param_2;
  iVar1->field_0xe0 = 0x0;
  param_1->field_0x0 = 0x6378;
  iVar1->field_0x2 = 0x1008;
  puVar1 = extraout_DX;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0x5b)),s_DanBrotherton_1050_0302);
  l_hgdiobj_1 = GetStockObject16(0x1000);
  iVar1->hgdiobj_field_0xc6 = l_hgdiobj_1;
  l_hcursor_1 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar1->hcursor_field_0xc4 = l_hcursor_1;
  iVar1->field_0xc8 = 0x200b;
  iVar1->field_0xac = 0x45000000;
  iVar1->field_0xbc = *(UINT16 *)((int)param_4 + 0x8);
  l_struct_2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_5,puVar1,unaff_DI);
  uVar1 = (UINT16 *)((ulong)l_struct_2 >> 0x10);
  iVar1->field_0xb4 = 0x0;
  iVar1->field_0xb6 = 0x0;
  iVar1->field_0xb8 = *(UINT16 *)((int)l_struct_2 + 0xa);
  iVar1->field_0xba = *(UINT16 *)((int)l_struct_2 + 0xc);
  iVar1->field_0xca = param_3;
  win_ui_reg_class_1008_96d2(param_1,0x1010,param_5);
  return param_1;
}


void __stdcall16far fill_rect_1008_62c0(HWND16 param_1)

{
  RECT16 local_2e [0x2];
  RECT16 *pRStack38;
  HDC16 HStack36;
  PAINTSTRUCT16 local_22;
  
  HStack36 = BeginPaint16(param_1,&local_22);
  pRStack38 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_2e);
  FillRect16((HDC16)s_tile2_bmp_1050_1538,pRStack38,(HBRUSH16)local_2e);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return;
}


HPALETTE16 __stdcall16far palette_op_1008_4e08(astruct_13 *param_1,BOOL16 param_2,ushort param_3,HDC16 param_4)

{
  HPALETTE16 HVar1;
  
  create_palette_1008_4e38(param_1,param_4,param_3);
  HVar1 = SelectPalette16(param_4,0x0,param_2);
  RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  return HVar1;
}

// WARNING: Unable to use type for symbol uVar3

void __stdcall16far create_palette_1008_4e38(astruct_13 *in_struct_1,LOGPALETTE *in_log_palette_2,uchar *param_3)

{
  int *piVar1;
  undefined2 *puVar2;
  ushort uVar4;
  astruct_13 *local_struct_1;
  int iVar5;
  int iVar6;
  UINT16 uVar8;
  UINT16 uVar9;
  UINT16 uVar10;
  int iStack14;
  UCHAR *puStack12;
  UCHAR *puStack8;
  undefined2 *uVar3;
  
  uVar8 = (UINT16)((ulong)in_struct_1 >> 0x10);
  local_struct_1 = (astruct_13 *)in_struct_1;
  uVar4 = (local_struct_1->field_0xc + 0x2) * 0x4;
  if (local_struct_1->field_0xe == (undefined2 *)0x0) {
    in_log_palette_2 = (LOGPALETTE *)&PTR_LOOP_1050_1000;
    mem_op_1000_179c(uVar4,param_3,0x1000);
    *(ushort *)&local_struct_1->field_0xe = uVar4;
    *(uchar **)((int)&local_struct_1->field_0xe + 0x2) = param_3;
    *local_struct_1->field_0xe = 0x300;
    uVar3 = local_struct_1->field_0xe;
    *(int *)((int)uVar3 + 0x2) = local_struct_1->field_0xc;
    puVar2 = local_struct_1->field_0xe;
    puStack8 = (UCHAR *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0x4));
    puStack12 = local_struct_1->field_0x4;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = &local_struct_1->field_0xc;
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar9 = (UINT16)((ulong)puStack12 >> 0x10);
      iVar5 = (int)puStack12;
      *puStack8 = *(UCHAR *)(iVar5 + 0x2);
      uVar10 = (UINT16)((ulong)puStack8 >> 0x10);
      iVar6 = (int)puStack8;
      *(undefined *)(iVar6 + 0x1) = *(undefined *)(iVar5 + 0x1);
      *(UCHAR *)(iVar6 + 0x2) = *puStack12;
      *(undefined *)(iVar6 + 0x3) = 0x0;
      iStack14 = iStack14 + 0x1;
      puStack8 = (UCHAR *)((ulong)puStack8 & 0xffff0000 | (ulong)(iVar6 + 0x4));
      puStack12 = (UCHAR *)((ulong)puStack12 & 0xffff0000 | (ulong)(iVar5 + 0x4));
    }
  }
  CreatePalette16(in_log_palette_2);
  return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
file_and_draw_op_1008_4f20(ushort *param_1,ulong param_2,ushort param_3,ulong param_4,ushort param_5)

{
  undefined4 uVar1;
  ushort b_force_background;
  COLORREF color;
  COLORREF color_00;
  uint x;
  uint16_t uVar2;
  LPCSTR output;
  int iVar3;
  undefined2 uVar4;
  astruct_43 *paVar5;
  ulong uVar6;
  DEVMODEA *init_data;
  HDC16 local_2c;
  LPCSTR pCStack42;
  LPCSTR pCStack40;
  undefined local_26 [0x24];
  
  pass1_1008_4016((astruct_76 *)param_1);
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(ulong *)(iVar3 + 0x1e) = param_4;
  *(ushort *)(iVar3 + 0x22) = param_3;
  *(ulong *)(iVar3 + 0x24) = param_2;
  *param_1 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x9;
  *(undefined2 *)(iVar3 + 0x2) = 0x1008;
  paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x2,param_5);
  uVar2 = (uint16_t)((ulong)paVar5 >> 0x10);
  struct_op_1008_48fe((astruct_81 *)CONCAT22(param_5,local_26),0x1,(char *)paVar5,uVar2);
  read_file_1008_49e8(CONCAT22(param_5,local_26),0x1010,uVar2);
  pass1_1008_5068((astruct_76 *)param_1,(astruct_83 *)CONCAT22(param_5,local_26));
  pass1_1008_47cc((astruct_76 *)param_1);
  pass1_1008_4834((astruct_76 *)param_1);
  init_data = (DEVMODEA *)0x0;
  uVar6 = pass1_1008_4772((astruct_76 *)param_1);
  output = (LPCSTR)(uVar6 >> 0x10);
  pCStack42 = (LPCSTR)uVar6;
  pCStack40 = output;
  local_2c = CreateDC16((LPCSTR)0x1010,pCStack42,output,init_data);
  b_force_background = palette_op_1008_46e4((ulong)param_1,(ushort)&local_2c,(ushort)output,(int)s_tile2_bmp_1050_1538);
  color = SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0xffff);
  color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538,*(COLORREF *)(iVar3 + 0x22));
  x = str_op_1000_3da4(*(char **)(iVar3 + 0x1e));
  uVar1 = *(undefined4 *)(iVar3 + 0x1e);
  TextOut16(0x1000,x,(INT16)uVar1,(char *)((ulong)uVar1 >> 0x10),0x0);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,color);
  SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color_00);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  close_file_1008_496c(local_26,param_5);
  return;
}



BOOL16 __stdcall16far cleanup_palette_1008_56e2(ULONG param_1,HDC16 param_2)

{
  HPALETTE16 HVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  HVar1 = SelectPalette16(param_2,0x0,*(BOOL16 *)((int)param_1 + 0x4));
  *(HPALETTE16 *)((int)param_1 + 0x4) = HVar1;
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return 0x1;
}


void __stdcall16far set_di_bits_to_device_1008_45d6(ulong param_1,HDC16 param_2)

{
  INT16 info;
  undefined4 uVar1;
  bool bVar2;
  int iVar3;
  int y_dest;
  undefined2 uVar4;
  INT16 cx;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)param_1);
  }
  if ((*(uint *)(iVar3 + 0x8) | *(uint *)(iVar3 + 0x6)) == 0x0) {
    bVar2 = false;
  }
  else {
    if ((*(uint *)(iVar3 + 0xc) | *(uint *)(iVar3 + 0xa)) == 0x0) {
      pass1_1008_4834((astruct_76 *)param_1);
    }
    bVar2 = true;
  }
  if (!bVar2) {
    return;
  }
  uVar1 = *(undefined4 *)(iVar3 + 0x10);
  cx = (INT16)((ulong)uVar1 >> 0x10);
  y_dest = (int)uVar1;
  info = *(INT16 *)(y_dest + 0x8);
  uVar1 = *(undefined4 *)(iVar3 + 0x14);
  SetDIBitsToDevice(param_2,0x0,y_dest,cx,(INT16)uVar1,(INT16)((ulong)uVar1 >> 0x10),info,0x0,0x0,(LPCVOID)0x0,
                    (BITMAPINFO *)info,*(UINT16 *)(y_dest + 0x4));
  return;
}



void __stdcall16far stretch_di_bits_1008_465a(ulong param_1,HDC16 param_2)

{
  PVOID bits;
  INT16 height_src;
  undefined4 uVar1;
  bool bVar2;
  int iVar3;
  int height_dst;
  undefined2 uVar4;
  INT16 x_src;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)param_1);
  }
  if ((*(uint *)(iVar3 + 0x8) | *(uint *)(iVar3 + 0x6)) == 0x0) {
    bVar2 = false;
  }
  else {
    if ((*(uint *)(iVar3 + 0xc) | *(uint *)(iVar3 + 0xa)) == 0x0) {
      pass1_1008_4834((astruct_76 *)param_1);
    }
    bVar2 = true;
  }
  if (!bVar2) {
    return;
  }
  uVar1 = *(undefined4 *)(iVar3 + 0x10);
  x_src = (INT16)((ulong)uVar1 >> 0x10);
  height_dst = (int)uVar1;
  bits = *(PVOID *)(height_dst + 0x4);
  height_src = *(INT16 *)(height_dst + 0x8);
  uVar1 = *(undefined4 *)(iVar3 + 0x14);
  StretchDIBits16(param_2,0x20,0xcc,0x0,height_dst,x_src,(INT16)uVar1,(INT16)((ulong)uVar1 >> 0x10),height_src,bits,
                  (BITMAPINFO *)0x0,0x0,CONCAT22(bits,height_src));
  return;
}



ushort __stdcall16far palette_op_1008_46e4(ulong param_1,ushort param_2,ushort param_3,HDC16 param_4)

{
  bool bVar1;
  ushort uVar2;
  HPALETTE16 HVar2;
  int iVar3;
  ushort uVar4;
  ulong uVar5;
  undefined4 uVar6;
  
  uVar4 = (ushort)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    uVar5._0_2_ = param_2;
    uVar5._2_2_ = param_3;
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar4 << 0x10));
    param_2 = (ushort)uVar5;
    param_3 = uVar5._2_2_;
  }
  uVar6 = CONCAT22(param_3,param_2);
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar3 + 0xa) == 0x0) {
      uVar6 = pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar4 << 0x10));
    }
    bVar1 = true;
  }
  uVar2 = (ushort)uVar6;
  if (!bVar1) {
    return 0x0;
  }
  create_palette_1008_4e38(*(astruct_13 **)(iVar3 + 0xa),param_4,(int)((ulong)uVar6 >> 0x10));
  *(ushort *)(iVar3 + 0xe) = uVar2;
  HVar2 = SelectPalette16(param_4,0x0,*(BOOL16 *)(iVar3 + 0xe));
  *(HPALETTE16 *)(iVar3 + 0x4) = HVar2;
  RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  return *(ushort *)(iVar3 + 0x4);
}


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


void __stdcall16far pass1_1008_0984(int param_1,ushort param_2,int param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  code **ppcVar2;
  
  set_sys_color_1008_357e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  if (*(long *)(param_1 + 0xe8) != 0x0) {
    uVar1 = *(undefined4 *)(param_1 + 0xe8);
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xe8) + 0x98);
    (**ppcVar2)(param_4,(int)uVar1,(int)((ulong)uVar1 >> 0x10),param_3);
  }
  return;
}


void __stdcall16far set_struct_op_1008_0536(ushort *param_1,HINSTANCE16 param_2,ushort param_3)

{
  HICON16 HVar1;
  HCURSOR16 HVar2;
  HGDIOBJ16 HVar3;
  uchar *puVar4;
  int iVar5;
  int unaff_DI;
  undefined2 uVar6;
  astruct_20 *paVar7;
  ushort *puVar8;
  
  paVar7 = pass1_1008_3ab8((astruct_20 *)param_1);
  puVar4 = (uchar *)((ulong)paVar7 >> 0x10);
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *(undefined4 *)(iVar5 + 0xe0) = 0x0;
  *(undefined4 *)(iVar5 + 0xe4) = 0x0;
  *(undefined4 *)(iVar5 + 0xe8) = 0x0;
  *(undefined2 *)(iVar5 + 0xec) = 0x0;
  *(undefined4 *)(iVar5 + 0xee) = 0x0;
  *(undefined2 *)(iVar5 + 0xf2) = 0x0;
  *(undefined4 *)(iVar5 + 0xf4) = 0x0;
  *(undefined4 *)(iVar5 + 0xf8) = 0x0;
  *param_1 = 0x389e;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  *(undefined2 *)(iVar5 + 0xc8) = 0x2008;
  *(undefined2 *)(iVar5 + 0xac) = 0x0;
  *(undefined2 *)(iVar5 + 0xae) = 0x8700;
  HVar1 = LoadIcon16(param_2,(LPCSTR)0xd4);
  *(HICON16 *)(iVar5 + 0xc2) = HVar1;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  *(HCURSOR16 *)(iVar5 + 0xc4) = HVar2;
  HVar3 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  *(HGDIOBJ16 *)(iVar5 + 0xc6) = HVar3;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_3,puVar4,unaff_DI);
  puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar5 + 0xa)),s_Outpost_1050_00d7);
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_3,puVar4,unaff_DI);
  *(undefined2 *)(iVar5 + 0xf4) = (int)puVar8;
  *(undefined2 *)(iVar5 + 0xf6) = (int)((ulong)puVar8 >> 0x10);
  set_sys_color_1008_357e((ulong)param_1,0x1,0x1010,param_3);
  return;
}

