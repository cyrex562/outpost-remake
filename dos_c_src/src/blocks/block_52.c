
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_draw_op_1020_3da4(astruct_24 *param_1,ULONG param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int16_t iVar4;
  HGDIOBJ16 HVar5;
  HDC16 *pHVar6;
  uchar *in_DX;
  undefined2 uVar7;
  int iVar8;
  int unaff_DI;
  undefined2 uVar9;
  undefined2 unaff_CS;
  ushort unaff_SS;
  ushort *puVar10;
  HDC16 local_4;
  astruct_24 *iVar9;
  astruct_24 *uVar8;
  
  get_sys_metrics_1020_7c1a(&param_1->field_0x0,param_2,unaff_CS);
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  iVar8 = (int)param_1;
  *(undefined4 *)(iVar8 + 0x14) = 0x0;
  param_1->field_0x0 = 0x408a;
  *(undefined2 *)(iVar8 + 0x2) = 0x1020;
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,unaff_SS,in_DX,unaff_DI);
  uVar7 = (undefined2)((ulong)puVar10 >> 0x10);
  *(undefined2 *)(iVar8 + 0x14) = (int)puVar10;
  *(undefined2 *)(iVar8 + 0x16) = uVar7;
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x14) + 0x4);
  (**ppcVar2)(0x1010,*(undefined2 *)(iVar8 + 0x14),uVar7,0x0,param_1);
  local_4 = GetDC16(0x1010);
  iVar4 = SetMapMode16((HDC16)s_tile2_bmp_1050_1538,0x1);
  *(int16_t *)(iVar8 + 0x1e) = iVar4;
  HVar5 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar5 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar5);
  *(HGDIOBJ16 *)(iVar8 + 0x18) = HVar5;
  HVar5 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar5 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar5);
  *(HGDIOBJ16 *)(iVar8 + 0x1a) = HVar5;
  uVar3 = *(undefined4 *)(iVar8 + 0x14);
  puVar1 = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
  pHVar6 = &local_4;
  ppcVar2 = (code **)((int)*puVar1 + 0x8);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)puVar1,(int)((ulong)puVar1 >> 0x10),pHVar6);
  *(HDC16 **)(iVar8 + 0x1c) = pHVar6;
  uVar3 = *(undefined4 *)(iVar8 + 0x14);
  *(HDC16 *)((int)uVar3 + 0x4c) = local_4;
  return;
}



void __stdcall16far win_ui_palette_op_1020_3e84(astruct_16 *param_1)

{
  astruct_16 *iVar1;
  UINT16 uVar1;
  ushort unaff_SS;
  
  uVar1 = (UINT16)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_16 *)param_1;
  *(undefined2 *)param_1 = 0x408a;
  iVar1->field_0x2 = 0x1020;
  pass1_1010_1ea6(iVar1->field_0x14,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,unaff_SS);
  SelectObject16(0x1010,iVar1->field_0x18);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,iVar1->field_0x1a);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,iVar1->field_0x1c);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SetMapMode16((HDC16)s_tile2_bmp_1050_1538,iVar1->field_0x1e);
  *(undefined2 *)param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *(undefined2 *)param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far validate_rect_1020_3f12(ulong param_1,int param_2,HWND16 param_3)

{
  RECT16 local_a;
  undefined4 uStack6;
  
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  local_a = (RECT16)0x8000e;
  uStack6 = 0x1100116;
  InvalidateRect16(param_3,(RECT16 *)0x0,(BOOL16)&local_a);
  local_a = (RECT16)0xf10000;
  uStack6 = 0x1220030;
  ValidateRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
  local_a = (RECT16)0xf100f5;
  uStack6 = 0x1220127;
  ValidateRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
  return;
}



void __stdcall16far mixed_draw_op_1020_3fa0(ulong param_1,HWND16 param_2,ushort param_3)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  int iStack56;
  ulong uStack54;
  undefined4 local_32;
  int iStack46;
  ulong uStack44;
  undefined4 *puStack40;
  undefined2 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar6 = *(undefined2 *)(iVar4 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  local_24 = *(undefined2 *)((int)uVar3 + 0x4c);
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  puStack40 = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
  ppcVar2 = (code **)((int)*puStack40 + 0x4);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)puStack40,(int)((ulong)puStack40 >> 0x10),0x0,&local_24,param_3,uVar6);
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  iStack46 = *(int *)((int)uVar3 + 0x44);
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  uStack44 = *(ulong *)((int)uVar3 + 0x40);
  uVar1 = *(ulong *)(iVar4 + 0x14);
  pass1_1008_3e94((ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x3a)),(ushort *)CONCAT22(param_3,&local_32),
                  (ushort *)CONCAT22(param_3,(int)&local_32 + 0x2));
  uStack54 = uStack44;
  for (iStack56 = 0x0; iStack56 < iStack46; iStack56 = iStack56 + 0x1) {
    draw_rect_1020_40ce(uStack54,(int)local_32,(int)((ulong)local_32 >> 0x10),param_3);
    uStack54 = uStack54 & 0xffff0000 | (ulong)((int)uStack54 + 0x18);
  }
  EndPaint16(0x1008,&local_22);
  return;
}



astruct_18 * __stdcall16far pass1_1020_4064(astruct_18 *param_1,byte param_2)

{
  win_ui_palette_op_1020_3e84((astruct_16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1020_4092(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  pass1_1008_3e38(param_1);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x6) = 0x0;
  *(undefined2 *)(iVar1 + 0x8) = 0x0;
  *(undefined2 *)(iVar1 + 0xa) = 0x1;
  *(undefined2 *)(iVar1 + 0xc) = 0x0;
  *(undefined2 *)(iVar1 + 0xe) = 0x0;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x10)));
  return param_1;
}



void __stdcall16far draw_rect_1020_40ce(ulong param_1,int param_2,int param_3,ushort param_4)

{
  int iVar1;
  HGDIOBJ16 HVar2;
  HPEN16 handle;
  int local_6;
  int local_4;
  
  pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)),(ushort *)CONCAT22(param_4,&local_6),
                  (ushort *)CONCAT22(param_4,&local_4));
  pass1_1008_3e94((ushort *)param_1,(ushort *)CONCAT22(param_4,&local_6),(ushort *)CONCAT22(param_4,&local_4));
  iVar1 = *(int *)((int)param_1 + 0xa);
  Ellipse16(0x1008,iVar1 + local_6 + param_2,iVar1 + local_4 + param_3,
            (local_6 - *(int *)((int)param_1 + 0xa)) + param_2,(local_4 - *(int *)((int)param_1 + 0xa)) + param_3);
  if ((*(byte *)((int)param_1 + 0xe) & 0x1) != 0x0) {
    HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar2);
    handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,0xf9,0x100);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
    Rectangle16((HDC16)s_tile2_bmp_1050_1538,local_6 + param_2 + 0x5,local_4 + param_3 + 0x5,local_6 + param_2 + -0x5,
                local_4 + param_3 + -0x5);
    HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar2);
    HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar2);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_draw_op_1020_41c8(astruct_20 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  code **ppcVar1;
  HCURSOR16 HVar2;
  undefined2 *puVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  undefined2 uVar6;
  astruct_64 *uVar5;
  int unaff_DI;
  undefined2 uVar7;
  ushort unaff_SS;
  ushort *puVar8;
  undefined *puVar9;
  undefined *puVar10;
  undefined *puVar11;
  
  unk_draw_op_1020_7f7a(param_1,0x8,CONCAT22(param_3,param_2));
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  uVar5 = (astruct_64 *)param_1;
  uVar5->field_0xee = 0x0;
  uVar5->field_0xf0 = 0x0;
  uVar5->field_0xf2 = 0x0;
  uVar5->field_0xf4 = 0x1;
  uVar5->field_0xf6 = 0x0;
  uVar5->field_0xfa = (undefined4 *)0x0;
  uVar5->field_0xfe = 0x0;
  uVar5->field_0x102 = 0x0;
  uVar5->field_0x106 = 0x0;
  uVar5->field_0x10a = 0x0;
  uVar5->field_0x108 = 0x0;
  uVar5->field_0x10c = 0x0;
  uVar5->field_0x110 = 0x0;
  uVar5->field_0x10e = 0x0;
  uVar5->field_0x112 = 0x0;
  uVar5->field_0x114 = 0x0;
  uVar5->field_0x116 = 0x0;
  param_1->field_0x0 = 0x623c;
  uVar5->field_0x2 = 0x1020;
  uVar5->field_0xe2 = 0x62d8;
  uVar5->field_0xe4 = 0x1020;
  puVar4 = extraout_DX;
  puVar11 = PTR_LOOP_1050_038c;
  HVar2 = LoadCursor16(param_4,(LPCSTR)((int)s__s__ld_1050_019c + 0x2));
  uVar5->field_0xf0 = HVar2;
  puVar10 = PTR_LOOP_1050_038c;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)((int)s__s__ld_1050_019c + 0x3));
  uVar5->field_0xf2 = HVar2;
  puVar9 = PTR_LOOP_1050_038c;
  PTR_LOOP_1050_0398 = (undefined *)LoadAccelerators16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)s_OpAccel_1050_43e8);
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,unaff_SS,puVar4,unaff_DI);
  *(int *)&uVar5->field_0xfa = (int)puVar8;
  *(undefined2 *)((int)&uVar5->field_0xfa + 0x2) = (int)((ulong)puVar8 >> 0x10);
  if (param_1 == (astruct_20 *)0x0) {
    puVar3 = (undefined2 *)0x0;
    uVar6 = 0x0;
  }
  else {
    puVar3 = &uVar5->field_0xe2;
    uVar6 = uVar7;
  }
  ppcVar1 = (code **)((int)*uVar5->field_0xfa + 0x4);
  (**ppcVar1)(0x1010,uVar5->field_0xfa,0x0,puVar3,uVar6,puVar9,puVar10,puVar11);
  uVar5->field_0xe6 = uVar5->field_0xfa;
  return;
}



void __stdcall16far destroy_cursor_1020_42f4(ushort *param_1,HMENU16 param_2)

{
  int iVar1;
  undefined2 uVar2;
  HMENU16 h_cursor;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x623c;
  *(undefined2 *)(iVar1 + 0x2) = 0x1020;
  *(undefined2 *)(iVar1 + 0xe2) = 0x62d8;
  *(undefined2 *)(iVar1 + 0xe4) = 0x1020;
  h_cursor = param_2;
  if (*(int *)(iVar1 + 0x106) != 0x0) {
    h_cursor = (HMENU16)s_tile2_bmp_1050_1538;
    DestroyMenu16(param_2);
  }
  DestroyCursor16(h_cursor);
  DestroyCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  pass1_1020_808e(param_1);
  return;
}



void __stdcall16far
pass1_1020_434c(int param_1,ushort param_2,ulong *param_3,ushort param_4,ushort param_5,int param_6,ushort param_7,
               ushort param_8)

{
  if (param_6 == 0x1) {
    pass1_1020_6184(CONCAT22(param_2,param_1),param_5,param_8);
    return;
  }
  if (param_6 == 0x2) {
    ui_op_1020_536e(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),param_5,0x2,param_7);
    return;
  }
  pass1_1008_68ea(param_1,param_2,param_3,param_4,param_5,param_6);
  return;
}



void __stdcall16far post_msg_1020_4394(ulong param_1,uint param_2,HWND16 param_3)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  iVar2 = (int)param_1;
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x10) {
    if (*(int *)(iVar2 + 0x34) != 0x0) {
      PostMessage16(param_3,0x0,0x0,0x11100f6);
      return;
    }
  }
  else {
    if (param_2 < 0x11) {
      if ((char)param_2 == '\x01') {
        *(undefined4 *)(iVar2 + 0x18) = 0x0;
        return;
      }
      if ((char)param_2 == '\v') {
        uVar1 = *(undefined4 *)(iVar2 + 0x2c);
        *(undefined2 *)((int)uVar1 + 0xe) = *(undefined2 *)(iVar2 + -0xda);
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_1020_43f6(astruct *param_1,uchar *param_2,ushort param_3)

{
  code **ppcVar1;
  int iVar2;
  uint uVar3;
  undefined2 uVar4;
  uint uVar5;
  uint uVar6;
  int unaff_DI;
  ushort *puVar7;
  long lVar8;
  undefined2 uVar9;
  astruct_282 *iVar9;
  
  iVar9 = (astruct_282 *)param_1;
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  get_dc_1018_4db0(iVar9->field_0xfa,iVar9->field_0x8,0x1018);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_3,param_2,unaff_DI);
  *(int *)&iVar9->field_0x10e = (int)puVar7;
  *(undefined2 *)((int)&iVar9->field_0x10e + 0x2) = (int)((ulong)puVar7 >> 0x10);
  if (param_1 == (astruct *)0x0) {
    iVar2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    iVar2 = &iVar9->field_0xe2;
    uVar4 = uVar9;
  }
  ppcVar1 = (code **)((int)*iVar9->field_0x10e + 0x4);
  lVar8 = (**ppcVar1)(0x1010,iVar9->field_0x10e,0xb,iVar2,uVar4);
  mem_op_1000_179c(0x30,(uchar *)((ulong)lVar8 >> 0x10),0x1000);
  uVar5 = (uint)((ulong)lVar8 >> 0x10);
  uVar3 = (uint)lVar8;
  uVar6 = uVar5 | uVar3;
  if (lVar8 == 0x0) {
    *(undefined4 *)&iVar9->field_0xf6 = 0x0;
  }
  else {
    pass1_1020_62e0(uVar3,uVar5,iVar9->field_0x8,param_3);
    iVar9->field_0xf6 = uVar3;
    iVar9->field_0xf8 = uVar6;
  }
  ui_op_1020_536e(param_1,0x0,-0x1,0x3,uVar6);
  return;
}



void __stdcall16far pass1_1020_44b0(ulong *param_1)

{
  code **ppcVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0xf6) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x98);
    (**ppcVar1)();
    *(undefined2 *)(iVar2 + 0x112) = 0x0;
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xf6) + 0x8);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
mixed_menu_op_1020_44ec(ulong param_1,ushort param_2,int param_3,HMENU16 param_4,HMENU16 param_5,ushort param_6)

{
  undefined4 uVar1;
  ushort uVar2;
  UINT16 UVar3;
  BOOL16 BVar4;
  HMENU16 HVar5;
  uint uVar6;
  int iVar7;
  ulong uVar8;
  uchar *in_DX;
  uchar *puVar9;
  int iVar10;
  int unaff_DI;
  undefined2 uVar11;
  HMENU16 HVar12;
  uchar in_AF;
  UINT16 local_138 [0x2];
  ushort local_134 [0x2];
  ushort *puStack304;
  ulong uStack300;
  undefined4 uStack296;
  ulong uStack292;
  char *pcStack286;
  undefined4 uStack278;
  BOOL16 BStack270;
  undefined4 uStack268;
  ulong local_108 [0x40];
  ushort uStack8;
  ushort *puStack6;
  
  uVar11 = (undefined2)(param_1 >> 0x10);
  iVar10 = (int)param_1;
  HVar12 = param_5;
  if (*(int *)(iVar10 + 0x106) != 0x0) {
    if (*(HMENU16 *)(iVar10 + 0x106) == param_4) {
      puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar1 = *(undefined4 *)(iVar10 + 0x108);
      uStack8 = *(ushort *)((int)uVar1 + 0x2e);
      uVar1 = *(undefined4 *)(iVar10 + 0x108);
      uVar11 = (undefined2)((ulong)uVar1 >> 0x10);
      iVar10 = (int)uVar1;
      uStack296 = *(char **)(iVar10 + 0x42);
      puVar9 = *(uchar **)(iVar10 + 0x44);
      uStack296._3_1_ = (byte)((ulong)uStack296 >> 0x18);
      uVar6 = (uint)uStack296._3_1_;
      pcStack286 = uStack296;
      uStack268 = uStack296;
      if (uStack296._3_1_ == 0x0) {
        uVar2 = pass1_1020_bd80(uStack8);
        HVar12 = 0x1000;
        unk_str_op_1000_3d3e((char *)CONCAT22(param_6,local_108),(char *)CONCAT22(puVar9,uVar2));
      }
      else {
        pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                        (ulong)uStack296 & 0xffff | ZEXT24(puVar9) << 0x10);
        uStack296 = (char *)CONCAT22(puVar9,uVar6);
        HVar12 = 0x1010;
        pass1_1010_c3c2((ushort)puStack6,(ushort)((ulong)puStack6 >> 0x10),CONCAT22(param_6,local_108),
                        CONCAT22(puVar9,uVar6),puVar9,in_AF,param_6);
      }
      BStack270 = ModifyMenu16(HVar12,(UINT16)local_108,param_6,0x76,0x0);
      UVar3 = GetMenuState16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x13c);
      if (UVar3 != 0xffff) {
        DeleteMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x13c);
      }
      HVar12 = 0x1008;
      BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x20);
      if (BVar4 != 0x0) {
        uStack296 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
        InsertMenu16(0x1010,(UINT16)uStack296,(UINT16)((ulong)uStack296 >> 0x10),0x13c,0x400);
      }
      if (*(int *)((int)s_VrMode_1050_42ca + 0x8 + uStack8 * 0x2) == 0x0) {
        UVar3 = 0x1;
        param_4 = 0x77;
        goto LAB_1020_464c;
      }
      param_4 = 0x77;
    }
    else {
      HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
      HVar5 = GetSubMenu16(param_5,0x1);
      uStack296 = (char *)((ulong)uStack296 & 0xffff0000 | (ulong)HVar5);
      if (HVar5 != param_4) goto LAB_1020_479e;
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x1,0x200);
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x1,0x201);
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x1,0x202);
      uVar1 = *(undefined4 *)(iVar10 + 0x108);
      uVar8 = *(ulong *)((int)uVar1 + 0x42);
      uStack292 = uVar8;
      pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uVar8);
      uVar6 = (uint)uVar8;
      pcStack286 = (char *)(uVar8 & 0xffff | ZEXT24(in_DX) << 0x10);
      if (((uint)in_DX | uVar6) == 0x0) {
        return;
      }
      uStack278 = *(undefined4 *)(uVar6 + 0x2e);
      if ((*(uint *)(uVar6 + 0x30) | (uint)uStack278) == 0x0) {
        return;
      }
      uStack268 = *(char **)((uint)uStack278 + 0x200);
      local_108[0] = struct_op_1030_73a8(CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,uVar6)));
      uVar11 = (undefined2)(local_108[0] >> 0x10);
      puStack6 = *(ushort **)((int)local_108[0] + 0x1c);
      uVar6 = *(uint *)((int)local_108[0] + 0x1e);
      if ((uVar6 | (uint)puStack6) != 0x0) {
        uStack268 = (char *)((ulong)puStack6 & 0xffff | (ulong)uVar6 << 0x10);
      }
      uStack268._2_2_ = uStack268._2_2_ & 0xff;
      if ((int)uStack268 != 0x1) {
        return;
      }
      if (((ulong)uStack268 & 0xff0000) != 0x0) {
        return;
      }
      local_134[0] = pass1_1030_6fa0((ulong)pcStack286);
      HVar12 = 0x1008;
      BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,local_134[0],0x3f);
      if (BVar4 != 0x0) {
        HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
        BVar4 = EnableMenuItem16(0x1008,0x0,0x201);
      }
      if (*(long *)((int)pcStack286 + 0x36) != 0x0) {
        BVar4 = EnableMenuItem16(HVar12,0x0,0x202);
      }
      HVar12 = 0x1030;
      pass1_1030_69cc((ulong)pcStack286,BVar4,uStack268._2_2_,param_6);
      if (BVar4 == 0x0) {
        return;
      }
      param_4 = 0x200;
    }
    UVar3 = 0x0;
    goto LAB_1020_464c;
  }
LAB_1020_479e:
  iVar7 = param_3 + -0x1;
  if (iVar7 == 0x0) {
    pass1_1018_2504(0x0,in_DX);
    if (iVar7 == 0x0) {
      EnableMenuItem16(0x1018,0x401,0x0);
LAB_1020_47e3:
      HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
      UVar3 = 0x401;
      goto LAB_1020_464c;
    }
    HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
    EnableMenuItem16(0x1018,0x400,0x0);
  }
  else {
    if (param_3 == 0x2) {
      uVar2 = pass1_1020_64d4(*(ulong *)(iVar10 + 0xf6),0x2);
      if (uVar2 == 0x0) {
        EnableMenuItem16(HVar12,0x401,0x0);
      }
      else {
        EnableMenuItem16(HVar12,0x400,0x0);
      }
      HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,CONCAT11(0x4,uVar2 == 0x0),0x1);
      if ((PTR_LOOP_1050_0010 != (undefined *)0x0) || (*(long *)(iVar10 + 0x102) == 0x0)) goto LAB_1020_47e3;
    }
    else {
      if (param_3 == 0x3) {
        local_138[0] = 0x0;
        local_134[0] = 0x0;
        HVar12 = 0x1010;
        puStack304 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,in_DX,unaff_DI);
        uVar11 = (undefined2)((ulong)puStack304 >> 0x10);
        uStack300 = *(ulong *)((int)puStack304 + 0x20);
        uVar6 = *(uint *)((int)puStack304 + 0x22);
        if ((uVar6 | (uint)uStack300) != 0x0) {
          HVar12 = 0x1030;
          pass1_1030_8308((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                          (ushort *)CONCAT22(param_6,local_134),(ushort *)CONCAT22(param_6,local_138),
                          uStack300 & 0xffff | (ulong)uVar6 << 0x10,(ushort)local_134,uVar6);
          local_138[0] = *(UINT16 *)((int)puStack304 + 0x1e);
        }
        uStack296 = (char *)((ulong)uStack296 & 0xffff0000);
        do {
          CheckMenuItem16(HVar12,0x400,(UINT16)uStack296);
          HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
          EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x401,(UINT16)uStack296);
          uStack296 = (char *)((ulong)uStack296 & 0xffff0000 | (ulong)((UINT16)uStack296 + 0x1));
        } while ((int)((UINT16)uStack296 + 0x1) < 0x5);
        CheckMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x408,local_138[0]);
        for (uStack296 = (char *)((ulong)uStack296 & 0xffff0000); (int)(UINT16)uStack296 <= (int)local_134[0];
            uStack296 = (char *)((ulong)uStack296 & 0xffff0000 | (ulong)((UINT16)uStack296 + 0x1))) {
          EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x400,(UINT16)uStack296);
        }
        return;
      }
      if (param_3 != 0x4) {
        return;
      }
      param_4 = 0x2;
    }
  }
  UVar3 = 0x400;
LAB_1020_464c:
  EnableMenuItem16(HVar12,UVar3,param_4);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_sys_op_1020_493c
          (ulong *param_1,uint param_2,uchar *param_3,undefined2 param_4,HCURSOR16 param_5,WNDCLASS16 *param_6)

{
  code **ppcVar1;
  HCURSOR16 HVar2;
  undefined *puVar3;
  int iVar4;
  undefined4 *puVar5;
  ushort uVar6;
  uchar *puVar7;
  uchar *puVar8;
  ushort uVar9;
  uint uVar10;
  int unaff_DI;
  undefined2 uVar11;
  uchar in_AF;
  undefined4 uVar12;
  ushort *puVar13;
  astruct_100 *paVar14;
  char *pcVar15;
  undefined uVar16;
  ushort uVar17;
  undefined2 uVar18;
  uint uVar19;
  undefined4 local_356 [0x42];
  uint local_24e;
  uchar *puStack588;
  undefined4 local_144;
  undefined4 uStack320;
  undefined4 local_13c;
  ushort uStack42;
  undefined4 uStack38;
  uint uStack34;
  uchar *puStack32;
  undefined4 uStack30;
  undefined4 uStack26;
  ulong uStack22;
  astruct_43 *paStack18;
  undefined *puStack14;
  uchar *puStack12;
  ushort uStack10;
  undefined4 uStack6;
  
  if (param_2 == 0xe9) {
    return;
  }
  uVar9 = (ushort)param_1;
  puVar8 = (uchar *)((ulong)param_1 >> 0x10);
  if (param_2 < 0xea) {
    switch(param_2) {
    case 0x69:
      iVar4 = 0x0;
      break;
    case 0x6a:
      iVar4 = 0x1;
      break;
    case 0x6b:
      iVar4 = 0x2;
      break;
    case 0x6c:
      iVar4 = 0x3;
      break;
    case 0x6d:
      iVar4 = 0x4;
      break;
    default:
      return;
    case 0x77:
      if ((*(uint *)(uVar9 + 0x10a) | *(uint *)(uVar9 + 0x108)) == 0x0) {
        return;
      }
      uVar12 = *(undefined4 *)(uVar9 + 0x108);
      uVar19 = *(uint *)((int)s_VrMode_1050_42ca + 0x8 + *(int *)((int)uVar12 + 0x2e) * 0x2);
      uStack26 = (ushort *)((ulong)uStack26 & 0xffff0000 | (ulong)uVar19);
      if (uVar19 == 0x0) {
        return;
      }
      paStack18 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,(ushort)param_6);
      WinHelp16(0x1010,(LPCSTR)uStack26,
                CONCAT11((undefined)((int)(LPCSTR)uStack26 >> 0xf),(undefined)((int)(LPCSTR)uStack26 >> 0xf)),
                CONCAT22((int)paStack18,0x1));
      return;
    case 0x78:
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x45,(ushort)param_6,param_3,unaff_DI);
      puStack588 = (uchar *)(int)((ulong)puVar13 >> 0x10);
      local_24e = (uint)puVar13;
      enum_child_windows_1010_01be(0x1010);
      return;
    }
    set_cursor_1020_5764((ulong)param_1,iVar4,(ushort)param_6);
    return;
  }
  if (param_2 == 0x132) {
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0xffff;
    goto LAB_1020_4ef8;
  }
  if (param_2 < 0x133) {
    if (param_2 == 0x102) {
      uVar16 = 0x0;
      mem_op_1000_179c(0xb4,param_3,0x1000);
      puVar8 = (uchar *)((uint)param_3 | param_2);
      uStack34 = param_2;
      puStack32 = param_3;
      if (puVar8 == (uchar *)0x0) {
        iVar4 = 0x0;
        puVar8 = (uchar *)0x0;
      }
      else {
        uVar16 = 0x40;
        iVar4 = string_1040_8520((astruct_57 *)CONCAT22(param_3,param_2),(ushort)PTR_LOOP_1050_0396,0x31,0x2,0x57b,0x62b
                                 ,puVar8,(ushort)param_6);
      }
      local_144 = (undefined4 *)CONCAT22(puVar8,iVar4);
      ppcVar1 = (code **)((int)*local_144 + 0x74);
      (**ppcVar1)(uVar16,iVar4,puVar8);
      uStack320 = (undefined4 *)CONCAT22(uStack320._2_2_,iVar4);
      if (iVar4 != 0x1) {
        return;
      }
      pass1_1028_837e((astruct_100 *)CONCAT22(param_6,&local_13c),param_6,in_AF);
LAB_1020_4b6c:
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,&local_13c));
      return;
    }
    if (param_2 < 0x103) {
      switch(param_2) {
      case 0xf0:
        ui_op_1020_536e(param_1,0x0,-0x1,0x1,param_3);
        return;
      default:
        return;
      case 0xf6:
        if (*(int *)(uVar9 + 0x116) != 0x0) {
          if (param_1 == (ulong *)0x0) {
            iVar4 = 0x0;
            param_3 = (uchar *)0x0;
          }
          else {
            iVar4 = uVar9 + 0xe2;
            param_3 = puVar8;
          }
          local_356[0] = CONCAT22(param_3,iVar4);
          pass1_1010_1ea6(_PTR_LOOP_1050_02a0,CONCAT22(param_3,iVar4),(ushort)param_6);
          *(undefined2 *)(uVar9 + 0x116) = 0x0;
        }
        iVar4 = 0x12;
        break;
      case 0xf7:
        unk_win_op_1010_7300(*(ulong *)(uVar9 + 0x10e),0x0,0x9,0x0);
        return;
      case 0xfb:
        local_144 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,(ushort)param_6,param_3,unaff_DI);
        uStack320 = (undefined4 *)
                    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,(ushort)param_6,(uchar *)((ulong)local_144 >> 0x10),
                                    unaff_DI);
        pcVar15 = (char *)pass1_1010_375e((ulong)uStack320);
        pass1_1010_c25e((ushort)local_144,(ushort)((ulong)local_144 >> 0x10),pcVar15,(uint)pcVar15,
                        (uchar *)((ulong)pcVar15 >> 0x10),unaff_DI,(ushort)param_6);
        return;
      case 0xfc:
        post_msg_1020_55b0((ulong)param_1,param_6);
        return;
      case 0x101:
        uStack26 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,(ushort)param_6,param_3,unaff_DI);
        uVar11 = (undefined2)((ulong)uStack26 >> 0x10);
        uStack22 = *(ulong *)((int)uStack26 + 0x24);
        puVar7 = *(uchar **)((int)uStack26 + 0x26);
        uStack22._0_2_ = (uint)puVar7 | (uint)uStack22;
        if ((uint)uStack22 == 0x0) {
          uVar16 = 0x0;
          mem_op_1000_179c(0xb4,puVar7,0x1000);
          puVar8 = (uchar *)((uint)puVar7 | (uint)uStack22);
          uStack34 = (uint)uStack22;
          puStack32 = puVar7;
          if (puVar8 == (uchar *)0x0) {
            puVar5 = (undefined4 *)0x0;
            puVar8 = (uchar *)0x0;
          }
          else {
            uVar16 = 0x40;
            puVar5 = (undefined4 *)
                     string_1040_8520((astruct_57 *)CONCAT22(puVar7,(uint)uStack22),(ushort)PTR_LOOP_1050_0396,0x30,0x2,
                                      0x57b,0x730,puVar8,(ushort)param_6);
          }
          uStack30 = CONCAT22(puVar8,puVar5);
LAB_1020_4c5f:
          ppcVar1 = (code **)((int)*puVar5 + 0x74);
          (**ppcVar1)(uVar16,puVar5,puVar8);
          return;
        }
        uVar12 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar9 + 0x8),0xe,(ushort)puVar7,uVar9,
                                 (ushort)&PTR_LOOP_1050_1038,(ushort)param_6);
        paStack18 = (astruct_43 *)
                    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x43,(ushort)param_6,(uchar *)((ulong)uVar12 >> 0x10),unaff_DI);
        uVar11 = (undefined2)((ulong)paStack18 >> 0x10);
        iVar4 = (int)paStack18;
        puStack14 = (undefined *)*(ushort *)(iVar4 + 0xa);
        uStack10 = *(ushort *)(iVar4 + 0xc);
        uVar9 = *(ushort *)(iVar4 + 0xe);
        uStack6 = CONCAT22(uStack6._2_2_,uVar9);
        if (*(int *)(iVar4 + 0x10) != 0x0) {
          return;
        }
        pass1_1028_84ca((astruct_100 *)CONCAT22(param_6,&local_13c),uStack22,uVar9,uStack10,(ushort)puStack14,
                        (ushort)param_6,in_AF);
        goto LAB_1020_4b6c;
      }
    }
    else {
      if (param_2 != 0x106) {
        if (param_2 < 0x107) {
          if (param_2 == 0x103) {
            local_144 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,(ushort)param_6,param_3,unaff_DI);
            uVar11 = (undefined2)((ulong)local_144 >> 0x10);
            uStack320 = (undefined4 *)*(char **)((int)local_144 + 0x24);
            puStack32 = *(uchar **)((int)local_144 + 0x26);
            uStack34 = (uint)puStack32 | (uint)uStack320;
            if (uStack34 != 0x0) {
              uVar12 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar9 + 0x8),0xf,(ushort)puStack32,uVar9,
                                       (ushort)&PTR_LOOP_1050_1038,(ushort)param_6);
              local_13c = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x42,(ushort)param_6,(uchar *)((ulong)uVar12 >> 0x10),
                                          unaff_DI);
              uStack42 = *(ushort *)((int)local_13c + 0xa);
              if (uStack42 == 0x0) {
                return;
              }
              pass1_1030_e63e((astruct_100 *)CONCAT22(param_6,&local_24e),uStack42,param_6,in_AF);
              fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,&local_24e));
              return;
            }
            uVar16 = 0x0;
            mem_op_1000_179c(0xb4,puStack32,0x1000);
            puVar8 = (uchar *)((uint)puStack32 | uStack34);
            if (puVar8 == (uchar *)0x0) {
              puVar5 = (undefined4 *)0x0;
              puVar8 = (uchar *)0x0;
            }
            else {
              uVar16 = 0x40;
              puVar5 = (undefined4 *)
                       string_1040_8520((astruct_57 *)CONCAT22(puStack32,uStack34),(ushort)PTR_LOOP_1050_0396,0x30,0x2,
                                        0x57b,0x730,puVar8,(ushort)param_6);
            }
            uStack38 = CONCAT22(puVar8,puVar5);
          }
          else {
            if (param_2 != 0x104) {
              return;
            }
            puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,(ushort)param_6,param_3,unaff_DI);
            puStack32 = (uchar *)((ulong)puVar13 >> 0x10);
            uStack34 = (uint)puVar13;
            local_24e = uStack34;
            puStack588 = puStack32;
            pass1_1010_af66((ulong)puVar13,(uint)puStack32);
            local_144 = (undefined4 *)CONCAT22(local_144._2_2_,uStack34);
            if (uStack34 != 0x0) {
              uVar16 = 0x0;
              mem_op_1000_179c(0xb4,puStack32,0x1000);
              puVar8 = (uchar *)((uint)puStack32 | uStack34);
              if (puVar8 == (uchar *)0x0) {
                iVar4 = 0x0;
                puVar8 = (uchar *)0x0;
              }
              else {
                uVar16 = 0x40;
                iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puStack32,uStack34),(ushort)PTR_LOOP_1050_0396,0x31,0x2,
                                         0x57b,0x62c,puVar8,(ushort)param_6);
              }
              uStack320 = (undefined4 *)CONCAT22(puVar8,iVar4);
              ppcVar1 = (code **)((int)*uStack320 + 0x74);
              (**ppcVar1)(uVar16,iVar4,puVar8);
              local_13c = (ushort *)CONCAT22(local_13c._2_2_,iVar4);
              if (iVar4 != 0x1) {
                return;
              }
              uVar16 = (undefined)((uint)param_6 >> 0x8);
              paVar14 = pass1_1030_e79a((astruct_100 *)CONCAT13(uVar16,CONCAT12((char)param_6,local_356)),param_6,in_AF)
              ;
              uVar9 = (ushort)((ulong)paVar14 >> 0x10);
              puVar5 = local_356;
              fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT13(uVar16,CONCAT12((char)param_6,puVar5)));
              win_1008_5c5c(param_6,(ushort)puVar5,uVar9,_PTR_LOOP_1050_02a0,0x1e6);
              return;
            }
            uVar16 = 0x0;
            mem_op_1000_179c(0xb4,puStack32,0x1000);
            puVar8 = (uchar *)((uint)puStack32 | uStack34);
            if (puVar8 == (uchar *)0x0) {
              puVar5 = (undefined4 *)0x0;
              puVar8 = (uchar *)0x0;
            }
            else {
              uVar16 = 0x40;
              puVar5 = (undefined4 *)
                       string_1040_8520((astruct_57 *)CONCAT22(puStack32,uStack34),(ushort)PTR_LOOP_1050_0396,0x30,0x2,
                                        0x57b,0x731,puVar8,(ushort)param_6);
            }
            local_356[0] = CONCAT22(puVar8,puVar5);
          }
          goto LAB_1020_4c5f;
        }
        if (param_2 == 0x12f) {
          pass1_1020_61c4(uVar9,(ushort)puVar8,CONCAT22(param_6,&local_144),(ushort *)CONCAT22(param_6,&local_24e),
                          param_3,unaff_DI,(ushort)param_6);
          iVar4 = local_24e + 0x6a;
        }
        else {
          if (param_2 != 0x130) {
            if (param_2 != 0x131) {
              return;
            }
            uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x2);
            if (uVar6 == 0x0) {
              return;
            }
            iVar4 = 0x7;
            goto LAB_1020_49b7;
          }
          pass1_1020_61c4(uVar9,(ushort)puVar8,CONCAT22(param_6,&local_144),(ushort *)CONCAT22(param_6,&local_24e),
                          param_3,unaff_DI,(ushort)param_6);
          iVar4 = local_24e + 0x68;
        }
        uStack320 = (undefined4 *)CONCAT22(uStack320._2_2_,iVar4);
        if (0x6d < iVar4) {
          return;
        }
        if (iVar4 < 0x69) {
          return;
        }
        ppcVar1 = (code **)((int)*param_1 + 0x40);
        (**ppcVar1)();
        return;
      }
      iVar4 = 0x13;
    }
LAB_1020_49b7:
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar9 + 0x8),iVar4,(ushort)param_3,uVar9,(ushort)&PTR_LOOP_1050_1038
                    ,(ushort)param_6);
    return;
  }
  if (param_2 == 0x1c8) {
    SendMessage16(param_5,0x0,0x0,0x1110072);
    return;
  }
  if (0x1c8 < param_2) {
    if (param_2 == 0x1ca) {
      local_144 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,(ushort)param_6,param_3,unaff_DI);
      uStack320 = (undefined4 *)
                  pass1_1010_c234((uint)local_144,(uchar *)((ulong)local_144 >> 0x10),unaff_DI,(ushort)param_6);
      uVar10 = (uint)((ulong)uStack320 >> 0x10);
      uVar19 = (uint)uStack320;
      if ((uchar *)(uVar10 | uVar19) == (uchar *)0x0) {
        return;
      }
      local_13c = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,(ushort)param_6,(uchar *)(uVar10 | uVar19),unaff_DI);
      param_3 = (uchar *)((ulong)local_13c >> 0x10);
      pass1_1010_3770((ulong)local_13c,(char *)CONCAT22(uVar10,uVar19),(ushort)param_3);
      iVar4 = 0x3;
    }
    else {
      uVar17 = (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10);
      uVar6 = (ushort)_PTR_LOOP_1050_5748;
      if (param_2 == 0x200) {
        uVar12 = *(undefined4 *)(uVar9 + 0x108);
        uVar11 = (undefined2)((ulong)uVar12 >> 0x10);
        iVar4 = (int)uVar12;
        uStack26 = *(ushort **)(iVar4 + 0x42);
        param_3 = *(uchar **)(iVar4 + 0x44);
        uStack26._3_1_ = (byte)((ulong)uStack26 >> 0x18);
        puStack14 = (undefined *)(uint)uStack26._3_1_;
        if (uStack26._3_1_ != 0x5) {
          return;
        }
        pass1_1030_8344(uVar6,uVar17,(ulong)uStack26 & 0xffff | ZEXT24(param_3) << 0x10);
        iVar4 = 0x25;
        PTR_LOOP_1050_5f0c = puStack14;
        PTR_LOOP_1050_5f0e = param_3;
        puStack12 = param_3;
      }
      else {
        if (param_2 == 0x201) {
          uVar12 = *(undefined4 *)(uVar9 + 0x108);
          uVar11 = (undefined2)((ulong)uVar12 >> 0x10);
          iVar4 = (int)uVar12;
          uStack26 = *(ushort **)(iVar4 + 0x42);
          param_3 = *(uchar **)(iVar4 + 0x44);
          uStack26._3_1_ = (byte)((ulong)uStack26 >> 0x18);
          puStack14 = (undefined *)(uint)uStack26._3_1_;
          if (uStack26._3_1_ != 0x5) {
            return;
          }
          pass1_1030_8344(uVar6,uVar17,(ulong)uStack26 & 0xffff | ZEXT24(param_3) << 0x10);
          iVar4 = 0x26;
          PTR_LOOP_1050_5f16 = puStack14;
          PTR_LOOP_1050_5f18 = param_3;
          puStack12 = param_3;
        }
        else {
          if (param_2 != 0x202) {
            if (param_2 != 0x203) {
              return;
            }
            if (*(int *)(uVar9 + 0xf4) != 0x1) {
              return;
            }
            HVar2 = SetCursor16(param_5);
            *(HCURSOR16 *)(uVar9 + 0xee) = HVar2;
            *(undefined2 *)(uVar9 + 0xf4) = 0x3;
            SetCapture16((HWND16)s_tile2_bmp_1050_1538);
            return;
          }
          uVar12 = *(undefined4 *)(uVar9 + 0x108);
          uVar11 = (undefined2)((ulong)uVar12 >> 0x10);
          iVar4 = (int)uVar12;
          uStack6 = *(ulong *)(iVar4 + 0x42);
          param_3 = *(uchar **)(iVar4 + 0x44);
          uStack6._3_1_ = (byte)(uStack6 >> 0x18);
          puVar3 = (undefined *)(uint)uStack6._3_1_;
          if (uStack6._3_1_ != 0x5) {
            return;
          }
          pass1_1030_8344(uVar6,uVar17,uStack6 & 0xffff | ZEXT24(param_3) << 0x10);
          uStack22 = CONCAT22(param_3,puVar3);
          iVar4 = 0x27;
          PTR_LOOP_1050_5a68 = puVar3;
          PTR_LOOP_1050_5a6a = param_3;
        }
      }
    }
    goto LAB_1020_49b7;
  }
  switch(param_2) {
  case 0x133:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0xffff;
    uVar11 = 0x0;
    break;
  case 0x134:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0x1;
    goto LAB_1020_4ef8;
  case 0x135:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0x1;
    uVar11 = 0x0;
    break;
  case 0x136:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0xfffb;
    goto LAB_1020_4ef8;
  case 0x137:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0xfffb;
    uVar11 = 0x0;
    break;
  case 0x138:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0x5;
LAB_1020_4ef8:
    uVar18 = 0x0;
    break;
  case 0x139:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0x5;
    uVar11 = 0x0;
    break;
  default:
    goto switchD_1020_518a_caseD_13a;
  case 0x13c:
    uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6),0x2);
    if (uVar6 != 0x0) {
      iVar4 = 0x1a;
      goto LAB_1020_49b7;
    }
    goto switchD_1020_518a_caseD_13a;
  }
  pass1_1020_2a94(*(ulong *)(uVar9 + 0xce),CONCAT22(uVar11,uVar18),(ushort)param_6);
switchD_1020_518a_caseD_13a:
  return;
}
