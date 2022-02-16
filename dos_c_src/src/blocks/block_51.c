
BOOL16 __stdcall16far enable_menu_item_1020_2c2a(HMENU16 param_1,int param_2)

{
  BOOL16 BVar1;
  UINT16 w_item_id;
  
  if (param_2 != 0x0) {
    return param_2 - 0x1;
  }
  EnableMenuItem16(param_1,0x400,0x3);
  if ((int)PTR_LOOP_1050_3960 < 0x2) {
    w_item_id = 0x401;
  }
  else {
    w_item_id = 0x400;
  }
  BVar1 = EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,w_item_id,0x5);
  return BVar1;
}



void __stdcall16far pass1_1020_2c72(ulong param_1,ushort param_2,ushort param_3)

{
  draw_op_1020_30be(*(ulong *)((int)param_1 + 0xf6),param_2,param_3);
  return;
}



void __stdcall16far destroy_icon_1020_2c88(ulong param_1,HICON16 param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar6 = *(undefined2 *)(iVar4 + 0xc2);
  DestroyIcon16(param_2);
  *(undefined2 *)(iVar4 + 0xc2) = 0x0;
  *(undefined2 *)(iVar4 + 0x8) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xf6);
  uVar2 = *(uint *)(iVar4 + 0xf8);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)((int)s_tile2_bmp_1050_1538,puVar1,uVar2,0x1,uVar6);
  }
  *(undefined4 *)(iVar4 + 0xf6) = 0x0;
  pass1_1010_1dda(*(ulong *)(iVar4 + 0xf2));
  *(undefined4 *)(iVar4 + 0xf2) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1020_2cf0(astruct *param_1)

{
  undefined4 uVar1;
  code **ppcVar2;
  uint uVar3;
  BOOL16 *pBVar4;
  uchar *in_DX;
  undefined2 uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uint uVar7;
  undefined2 extraout_DX_00;
  int iVar8;
  int unaff_DI;
  uint uVar9;
  ushort unaff_SS;
  ushort *puVar10;
  ulong uVar11;
  undefined *puVar12;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar9 = (uint)((ulong)param_1 >> 0x10);
  iVar8 = (int)param_1;
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,*(ushort *)(iVar8 + 0xfc),unaff_SS,in_DX,unaff_DI);
  uVar5 = (undefined2)((ulong)puVar10 >> 0x10);
  *(undefined2 *)(iVar8 + 0xf2) = (int)puVar10;
  *(undefined2 *)(iVar8 + 0xf4) = uVar5;
  *(undefined2 *)(iVar8 + 0xe0) = *(undefined2 *)(iVar8 + 0xf2);
  *(undefined2 *)(iVar8 + 0xe2) = uVar5;
  puVar12 = PTR_LOOP_1050_038c;
  uVar3 = LoadIcon16(0x1010,(LPCSTR)s_SITEICON_1050_428d);
  *(HICON16 *)(iVar8 + 0xc2) = uVar3;
  uVar1 = *(undefined4 *)(iVar8 + 0xf2);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xf2) + 0x30);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(int)((ulong)uVar1 >> 0x10),uVar3,puVar12);
  puVar6 = extraout_DX;
  mem_op_1000_179c(0x22,extraout_DX,0x1000);
  uVar7 = (uint)puVar6 | uVar3;
  if (uVar7 == 0x0) {
    *(undefined4 *)(iVar8 + 0xf6) = 0x0;
  }
  else {
    load_draw_op_1020_2ede((ushort *)CONCAT22(puVar6,uVar3),(ulong)param_1,0x1000);
    *(uint *)(iVar8 + 0xf6) = uVar3;
    *(uint *)(iVar8 + 0xf8) = uVar7;
  }
  *(undefined4 *)(iVar8 + 0xe8) = *(undefined4 *)(iVar8 + 0xf6);
  pass1_1018_0ac0(*(ulong *)(iVar8 + 0xf2),(ulong)param_1 & 0xffff | (ulong)uVar9 << 0x10);
  uVar11 = pass1_1018_0b08(*(ulong *)(iVar8 + 0xf2));
  pBVar4 = (BOOL16 *)uVar11;
  ppcVar2 = (code **)((int)*(undefined4 *)param_1 + 0x14);
  (**ppcVar2)();
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xf2) + 0x10);
  (**ppcVar2)();
  MoveWindow16(0x1018,0x1,pBVar4[0x3],pBVar4[0x2],pBVar4[0x1],*pBVar4);
  pass1_1008_3e0e((ulong)param_1);
  UpdateWindow16(0x1008);
  return;
}



astruct_3 * __stdcall16far pass1_1020_2e24(astruct_3 *param_1,byte param_2)

{
  ushort unaff_CS;
  
  pass1_1020_28fc(param_1,unaff_CS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far load_draw_op_1020_2ede(ushort *param_1,ulong param_2,ushort param_3)

{
  undefined4 uVar1;
  code **ppcVar2;
  HDC16 HVar3;
  int iVar4;
  HPEN16 handle;
  HGDIOBJ16 HVar5;
  uchar *in_DX;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  ushort unaff_SS;
  ushort *puVar8;
  astruct_76 *paVar9;
  ulong uVar10;
  
  get_sys_metrics_1020_7c1a(param_1,param_2,param_3);
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  *(undefined4 *)(iVar6 + 0x14) = 0x0;
  *(undefined2 *)(iVar6 + 0x18) = 0x0;
  *(undefined2 *)(iVar6 + 0x1a) = 0x0;
  *(undefined2 *)(iVar6 + 0x1c) = 0x0;
  *(undefined2 *)(iVar6 + 0x1e) = 0x0;
  *(undefined2 *)(iVar6 + 0x20) = 0x0;
  *param_1 = 0x363c;
  *(undefined2 *)(iVar6 + 0x2) = 0x1020;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,*(ushort *)((int)param_2 + 0xfc),unaff_SS,in_DX,unaff_DI);
  *(undefined2 *)(iVar6 + 0x14) = (int)puVar8;
  *(undefined2 *)(iVar6 + 0x16) = (int)((ulong)puVar8 >> 0x10);
  uVar1 = *(undefined4 *)(iVar6 + 0x14);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x14) + 0x4);
  (**ppcVar2)(0x1010,(int)uVar1,(int)((ulong)uVar1 >> 0x10),0x0,param_1);
  paVar9 = (astruct_76 *)pass1_1018_0a50(*(ulong *)(iVar6 + 0x14));
  uVar10 = pass1_1008_4772(paVar9);
  HVar3 = CreateDC16((LPCSTR)0x1008,(LPCSTR)uVar10,(LPCSTR)(uVar10 >> 0x10),(DEVMODEA *)0x0);
  *(HDC16 *)(iVar6 + 0x18) = HVar3;
  iVar4 = iVar6 + 0x18;
  ppcVar2 = (code **)((int)*(undefined4 *)paVar9 + 0x8);
  (**ppcVar2)();
  *(int *)(iVar6 + 0x20) = iVar4;
  uVar1 = *(undefined4 *)(iVar6 + 0x14);
  uVar1 = *(undefined4 *)((int)uVar1 + 0x64);
  handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)uVar1,(COLORREF)((ulong)uVar1 >> 0x10));
  *(HPEN16 *)(iVar6 + 0x1a) = handle;
  HVar5 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  *(HGDIOBJ16 *)(iVar6 + 0x1c) = HVar5;
  HVar5 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar5 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar5);
  *(HGDIOBJ16 *)(iVar6 + 0x1e) = HVar5;
  return;
}



void __stdcall16far cleanup_win_ui_1020_2fea(astruct_12 *in_struct_1,HDC16 in_dc_handle_2)

{
  astruct_12 *iVar1;
  UINT16 var2;
  ushort unaff_SS;
  
  var2 = (UINT16)((ulong)in_struct_1 >> 0x10);
  iVar1 = (astruct_12 *)in_struct_1;
  in_struct_1->offset_field_0x0 = 0x363c;
  iVar1->offset_field_0x2 = 0x1020;
  if (iVar1->field_0x14 != 0x0) {
    in_dc_handle_2 = 0x1010;
    pass1_1010_1ea6(iVar1->field_0x14,(ulong)in_struct_1 & 0xffff | (ulong)var2 << 0x10,unaff_SS);
  }
  SelectObject16(in_dc_handle_2,iVar1->field_0x1c);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,iVar1->field_0x1e);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,iVar1->field_0x20);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  in_struct_1->offset_field_0x0 = 0x3ab0;
  iVar1->offset_field_0x2 = 0x1008;
  in_struct_1->offset_field_0x0 = 0x389a;
  iVar1->offset_field_0x2 = 0x1008;
  return;
}



void __stdcall16far invalidate_rect_1020_3080(ulong param_1,int param_2,HWND16 param_3)

{
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x14) = 0x0;
    return;
  }
  if ((param_2 != 0x4) && (param_2 != 0x6)) {
    return;
  }
  InvalidateRect16(param_3,(RECT16 *)0x0,0x0);
  return;
}



void __stdcall16far draw_op_1020_30be(ulong param_1,HWND16 param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  BOOL16 BVar3;
  int iVar4;
  undefined2 uVar5;
  HWND16 hwnd;
  undefined2 uVar6;
  undefined2 uVar7;
  undefined4 *local_3c;
  int iStack56;
  int iStack54;
  int iStack52;
  int iStack50;
  RECT16 local_30;
  int iStack44;
  int iStack42;
  RECT16 *pRStack40;
  int iStack38;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar7 = *(undefined2 *)(iVar4 + 0x4);
  local_24 = BeginPaint16(param_2,&local_22);
  uVar6 = *(undefined2 *)(iVar4 + 0x4);
  hwnd = (HWND16)s_tile2_bmp_1050_1538;
  BVar3 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
  if (BVar3 == 0x0) {
    hwnd = 0x1018;
    local_3c = (undefined4 *)pass1_1018_0a50(*(ulong *)(iVar4 + 0x14));
    ppcVar1 = (code **)((int)*local_3c + 0x8);
    (**ppcVar1)(0x1018,(int)local_3c,(int)((ulong)local_3c >> 0x10),&local_24,param_3,uVar6,uVar7);
    uVar2 = *(undefined4 *)(iVar4 + 0x14);
    if (*(int *)((int)uVar2 + 0x84) == 0x1) {
      unk_draw_op_1020_320e(param_1,local_24,param_3);
    }
    ppcVar1 = (code **)((int)*local_3c + 0x4);
    (**ppcVar1)(0x1018,(int)local_3c,(int)((ulong)local_3c >> 0x10),0x0,0x0,0xdc,param_3);
    uVar2 = *(undefined4 *)(iVar4 + 0x14);
    if (*(int *)((int)uVar2 + 0x84) != 0x1) {
      unk_draw_op_1020_320e(param_1,local_24,param_3);
    }
    draw_op_1020_3488(param_1);
    ppcVar1 = (code **)((int)*local_3c + 0xc);
    (**ppcVar1)(0x1018,(int)local_3c,(int)((ulong)local_3c >> 0x10),&local_24,param_3);
  }
  else {
    if (PTR_LOOP_1050_0010 == (undefined *)0x0) {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x14) + 0x2c);
      iStack38 = (**ppcVar1)((int)s_tile2_bmp_1050_1538);
      if (iStack38 != 0x0) {
        pRStack40 = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_30);
        local_3c = (undefined4 *)0x0;
        iStack56 = (iStack44 - local_30.x) + -0x1;
        iStack54 = (iStack42 - local_30.y) + -0x1;
        iStack52 = iStack54;
        iStack50 = iStack56;
        FillRect16((HDC16)s_tile2_bmp_1050_1538,pRStack40,(HBRUSH16)&local_3c);
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        DrawIcon16((HDC16)s_tile2_bmp_1050_1538,iStack38,0x2,0x2);
      }
    }
  }
  EndPaint16(hwnd,&local_22);
  return;
}



void __stdcall16far unk_draw_op_1020_320e(ulong param_1,HDC16 param_2,ushort param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  ulong uVar8;
  DEVMODEA *init_data;
  int local_c;
  ulong local_a;
  HDC16 *pHStack6;
  HDC16 local_4;
  
  local_4 = param_2;
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  if (*(int *)((int)uVar3 + 0x84) == 0x1) {
    uVar3 = *(undefined4 *)(iVar4 + 0x14);
    uVar7 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar5 = (int)uVar3;
    puVar1 = (undefined4 *)*(ulong *)(iVar5 + 0x24);
    init_data = (DEVMODEA *)0x0;
    uVar8 = pass1_1008_4772((astruct_76 *)((ulong)puVar1 & 0xffff | (ulong)*(uint *)(iVar5 + 0x26) << 0x10));
    local_4 = CreateDC16((LPCSTR)0x1008,(LPCSTR)uVar8,(LPCSTR)(uVar8 >> 0x10),init_data);
    pHStack6 = &local_4;
    ppcVar2 = (code **)((int)*puVar1 + 0x8);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)puVar1,(int)((ulong)puVar1 >> 0x10),pHStack6,param_3);
  }
  pass1_1018_0d9a(*(ulong *)(iVar4 + 0x14),(ushort *)CONCAT22(param_3,&local_c),(ulong *)CONCAT22(param_3,&local_a));
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  draw_op_1020_33c0(param_1,*(ulong *)((int)uVar3 + 0x6c),local_c,local_a,0x1,local_4,0x1018);
  pass1_1018_1054(*(ulong *)(iVar4 + 0x14),(ushort *)CONCAT22(param_3,&local_c),(ulong *)CONCAT22(param_3,&local_a),
                  param_3);
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  draw_op_1020_33c0(param_1,*(ulong *)((int)uVar3 + 0x74),local_c,local_a,0x2,local_4,0x1018);
  pass1_1018_1320(*(ulong *)(iVar4 + 0x14),(ushort *)CONCAT22(param_3,&local_c),(ulong *)CONCAT22(param_3,&local_a));
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  draw_op_1020_33c0(param_1,*(ulong *)((int)uVar3 + 0x68),local_c,local_a,0x1,local_4,0x1018);
  pass1_1018_15f6(*(ulong *)(iVar4 + 0x14),(ushort *)CONCAT22(param_3,&local_c),(ulong *)CONCAT22(param_3,&local_a));
  if (local_c != 0x0) {
    uVar3 = *(undefined4 *)(iVar4 + 0x14);
    draw_op_1020_33c0(param_1,*(ulong *)((int)uVar3 + 0x70),local_c,local_a,0x1,local_4,0x1018);
  }
  pass1_1018_108c(*(ulong *)(iVar4 + 0x14),(ushort *)CONCAT22(param_3,&local_c),(ulong *)CONCAT22(param_3,&local_a),
                  param_3);
  if (local_c != 0x0) {
    uVar3 = *(undefined4 *)(iVar4 + 0x14);
    draw_op_1020_33c0(param_1,*(ulong *)((int)uVar3 + 0x78),local_c,local_a,0x0,local_4,0x1018);
  }
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  if (*(int *)((int)uVar3 + 0x84) == 0x1) {
    SelectPalette16(0x1018,0x0,(BOOL16)pHStack6);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  }
  return;
}



void __stdcall16far
draw_op_1020_33c0(ulong param_1,ulong param_2,int param_3,ulong param_4,int param_5,ushort param_6,ushort param_7)

{
  HPEN16 pen_handle;
  HGDIOBJ16 object_handle;
  HBRUSH16 brush_handle;
  HGDIOBJ16 obj_handle_2;
  int iVar1;
  undefined2 uVar2;
  undefined2 in_DX;
  undefined2 uVar3;
  HDC16 hdc;
  undefined2 unaff_SS;
  ushort uVar4;
  int iStack20;
  ushort *puStack14;
  
  if (param_3 != 0x0) {
    pen_handle = CreatePen16(param_7,(INT16)param_2,(COLORREF)(param_2 >> 0x10));
    object_handle = SelectObject16((HDC16)s_tile2_bmp_1050_1538,pen_handle);
    brush_handle = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    hdc = (HDC16)s_tile2_bmp_1050_1538;
    obj_handle_2 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,brush_handle);
    puStack14 = (ushort *)param_4;
    for (iStack20 = 0x0; iStack20 < param_3; iStack20 = iStack20 + 0x1) {
      uVar4 = (ushort)(param_1 >> 0x10);
      iVar1 = param_3;
      pass1_1020_3540((ushort)param_1,uVar4,param_5,puStack14,in_DX,unaff_SS);
      if (param_5 < 0x1) {
        uVar2 = 0x3;
      }
      else {
        uVar2 = 0x4;
      }
      uVar3 = in_DX;
      draw_polygon_1020_3602((ushort)param_1,uVar4,CONCAT22(iVar1,uVar2),hdc);
      hdc = 0x1000;
      fn_ptr_1000_17ce((astruct_18 *)CONCAT22(in_DX,iVar1),0x1000);
      puStack14 = (ushort *)((ulong)puStack14 & 0xffff0000 | (ulong)((int)puStack14 + 0x6));
      in_DX = uVar3;
    }
    SelectObject16(hdc,obj_handle_2);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,object_handle);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far draw_op_1020_3488(ULONG param_1)

{
  uint uVar1;
  ulong uVar2;
  undefined4 uVar3;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  HGDIOBJ16 HVar4;
  undefined2 uVar5;
  undefined2 unaff_SS;
  int bottom;
  undefined4 local_a;
  ushort *puStack6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  uVar2 = *(ulong *)((int)param_1 + 0x14);
  puStack6 = (ushort *)(uVar2 & 0xffff0000 | (ulong)((int)uVar2 + 0x36));
  pass1_1008_3e94(puStack6,(ushort *)CONCAT22(unaff_SS,&local_a),(ushort *)CONCAT22(unaff_SS,(int)&local_a + 0x2));
  uVar2 = (ulong)(local_a._2_2_ - 0x3U) << 0x10;
  if ((int)(local_a._2_2_ - 0x3U) < 0x0) {
    uVar2 = 0x0;
  }
  uVar1 = (int)local_a - 0x3;
  local_a = (ulong)uVar1;
  if ((int)uVar1 < 0x0) {
    local_a = 0x0;
  }
  local_a = uVar2 | local_a;
  uVar3 = *(undefined4 *)((int)param_1 + 0x14);
  uVar3 = *(undefined4 *)((int)uVar3 + 0x64);
  handle = CreatePen16(0x1008,(INT16)uVar3,(COLORREF)((ulong)uVar3 >> 0x10));
  handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  HVar4 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar4 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar4);
  bottom = (int)(local_a >> 0x10);
  Rectangle16((HDC16)s_tile2_bmp_1050_1538,(int)local_a + 0x6,bottom + 0x6,(int)local_a,bottom);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar4);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return;
}



void __stdcall16far
pass1_1020_3540(ushort param_1,ushort param_2,int param_3,ushort *param_4,uchar *param_5,undefined2 param_6)

{
  ushort uVar1;
  astruct_279 *iVar2;
  int iStack18;
  int iStack12;
  int iStack10;
  int local_6;
  int local_4;
  
  pass1_1008_3e94(param_4,(ushort *)CONCAT22(param_6,&local_6),(ushort *)CONCAT22(param_6,&local_4));
  if (param_3 == 0x0) {
    iStack12 = 0x3;
    iStack10 = 0x42a6;
  }
  else {
    if (param_3 == 0x1) {
      iStack12 = 0x4;
      iStack10 = (int)s_SITEICON_1050_428d + 0x9;
    }
    else {
      if (param_3 != 0x2) {
        return;
      }
      iStack12 = 0x4;
      iStack10 = 0x42b2;
    }
  }
  uVar1 = iStack12 << 0x2;
  mem_op_1000_179c(uVar1,param_5,0x1000);
  for (iStack18 = 0x0; iStack18 < iStack12; iStack18 = iStack18 + 0x1) {
    iVar2 = (astruct_279 *)(iStack18 * 0x4);
    *(int *)(iVar2 + uVar1) = *(int *)(iVar2 + iStack10) + local_4;
    *(int *)(iVar2 + uVar1 + 0x2) = *(int *)(iVar2 + iStack10 + 0x2) + local_6;
  }
  return;
}



void __stdcall16far draw_polygon_1020_3602(ushort param_1,ushort param_2,ulong param_3,HDC16 param_4)

{
  Polygon16(param_4,(POINT16 *)param_3,(INT16)(param_3 >> 0x10));
  return;
}



astruct_18 * __stdcall16far pass1_1020_3616(astruct_18 *param_1,byte param_2,ushort param_3)

{
  cleanup_win_ui_1020_2fea((astruct_12 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1020_3644(ushort *param_1,UINT16 param_2,ulong param_3,UINT16 param_4)

{
  astruct_272 *iVar2;
  short in_buf_len_5;
  astruct_270 *iVar1;
  
  struct_1020_790e(param_1,0x0,param_2,param_3,param_4);
  in_buf_len_5 = (short)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_272 *)param_1;
  iVar2->field_0xf2 = 0x389a;
  iVar2->field_0xf4 = 0x1008;
  iVar2->field_0xf2 = 0x3aa8;
  iVar2->field_0xf4 = 0x1008;
  iVar2->field_0x100 = 0x0;
  iVar2->field_0x10a = 0x0;
  iVar2->field_0x10e = 0x0;
  *param_1 = 0x3d08;
  iVar2->field_0x2 = 0x1020;
  iVar2->field_0xf2 = 0x3d9c;
  iVar2->field_0xf4 = 0x1020;
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,&iVar2->field_0xa,
             in_buf_len_5);
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x5b),s_VrMode_1050_42ca);
  iVar2->field_0xac = 0x44c00000;
  window_op_1020_38aa(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1020_36f6(ulong param_1,int param_2,short param_3)

{
  int iVar1;
  code **ppcVar2;
  undefined4 uVar3;
  char *pcVar4;
  ushort uVar5;
  undefined2 uVar6;
  SEGPTR lp_string;
  int iVar7;
  undefined2 uVar8;
  HWND16 hwnd;
  char *pcVar9;
  INT16 id;
  undefined *puStack1034;
  char local_406 [0x400];
  ulong uStack6;
  
  iVar7 = (int)param_1;
  uVar8 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x1) {
    *(undefined4 *)(iVar7 + 0x8) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  uStack6 = pass1_1018_1e78(*(ulong *)(iVar7 + 0x8),-0x1);
  uVar6 = (undefined2)(uStack6 >> 0x10);
  GetWindowText16(0x1018,0x3ff,(INT16)local_406);
  pcVar4 = pass1_1000_472c(CONCAT22(param_3,local_406),':');
  puStack1034 = (undefined *)CONCAT22(uVar6,pcVar4 + 0x2);
  *puStack1034 = 0x0;
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_406,param_3);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0x18) + 0x18);
  (**ppcVar2)();
  uVar3 = *(undefined4 *)(iVar7 + 0x8);
  iVar1 = *(int *)((int)uVar3 + 0x4a);
  uVar3 = *(undefined4 *)((int)uStack6 + 0x2);
  SetDlgItemText16(0x1010,(INT16)uVar3,(SEGPTR)((ulong)uVar3 >> 0x10));
  uVar3 = *(undefined4 *)((int)uStack6 + 0xa);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)uVar3,(SEGPTR)((ulong)uVar3 >> 0x10));
  uVar3 = *(undefined4 *)((int)uStack6 + 0x12);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)uVar3,(SEGPTR)((ulong)uVar3 >> 0x10));
  uVar3 = *(undefined4 *)((int)uStack6 + 0xe);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)uVar3,(SEGPTR)((ulong)uVar3 >> 0x10));
  if (iVar1 != 0x0) {
    hwnd = 0x1018;
    uVar5 = pass1_1018_1f1a(*(ulong *)(iVar7 + 0x8),*(int *)((int)uStack6 + 0x1a));
    if (uVar5 != 0x0) {
      uVar3 = *(undefined4 *)((int)uStack6 + 0x16);
      id = (INT16)uVar3;
      lp_string = (SEGPTR)((ulong)uVar3 >> 0x10);
      goto LAB_1020_3846;
    }
  }
  hwnd = 0x1010;
  pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  lp_string = (SEGPTR)((ulong)pcVar9 >> 0x10);
  id = (INT16)pcVar9;
LAB_1020_3846:
  SetDlgItemText16(hwnd,id,lp_string);
  if (*(long *)(iVar7 + 0x1c) != 0x0) {
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,*(INT16 *)((int)uStack6 + 0x1a));
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    return;
  }
  InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,0x0);
  return;
}



void __stdcall16far pass1_1020_3898(astruct_30 *param_1,ushort param_2)

{
  destroy_window_1020_3b3e(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far window_op_1020_38aa(astruct *param_1)

{
  code **ppcVar1;
  uint uVar2;
  astruct_160 *paVar3;
  undefined4 uVar4;
  uchar *in_DX;
  undefined2 uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  uchar *puVar7;
  uint uVar8;
  undefined2 extraout_DX_01;
  int unaff_DI;
  HWND16 HVar9;
  ushort unaff_SS;
  ushort *puVar10;
  uint uVar11;
  uint uVar12;
  RECT16 local_12;
  int iStack14;
  int iStack12;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  uVar11 = (uint)param_1;
  uVar12 = (uint)((ulong)param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,unaff_SS,in_DX,unaff_DI);
  uVar5 = (undefined2)((ulong)puVar10 >> 0x10);
  *(undefined2 *)(uVar11 + 0xfa) = (int)puVar10;
  *(undefined2 *)(uVar11 + 0xfc) = uVar5;
  *(undefined2 *)(uVar11 + 0xe0) = *(undefined2 *)(uVar11 + 0xfa);
  *(undefined2 *)(uVar11 + 0xe2) = uVar5;
  if ((uVar12 | uVar11) == 0x0) {
    uVar2 = 0x0;
    uVar8 = 0x0;
  }
  else {
    uVar2 = uVar11 + 0xf2;
    uVar8 = uVar12;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar11 + 0xfa) + 0x4);
  (**ppcVar1)(0x1010,*(undefined4 *)(uVar11 + 0xfa),0x0,uVar2,uVar8);
  puVar7 = extraout_DX;
  pass1_1018_1a8e(*(ulong *)(uVar11 + 0xfa),extraout_DX,unaff_DI,unaff_SS);
  mem_op_1000_179c(0x20,puVar7,0x1000);
  puVar6 = (uchar *)((uint)puVar7 | uVar2);
  if (puVar6 == (uchar *)0x0) {
    *(undefined4 *)(uVar11 + 0xf6) = 0x0;
  }
  else {
    unk_draw_op_1020_3da4((astruct_24 *)CONCAT22(puVar7,uVar2),(ULONG)param_1);
    *(uint *)(uVar11 + 0xf6) = uVar2;
    *(uchar **)(uVar11 + 0xf8) = extraout_DX_00;
    puVar6 = extraout_DX_00;
  }
  uVar4 = *(undefined4 *)(uVar11 + 0xf6);
  *(undefined4 *)(uVar11 + 0xe8) = uVar4;
  mem_op_1000_179c(0x42,puVar6,0x1000);
  paVar3 = (astruct_160 *)uVar4;
  puVar7 = (uchar *)((uint)puVar6 | (uint)paVar3);
  if (puVar7 == (uchar *)0x0) {
    *(undefined4 *)(uVar11 + 0x102) = 0x0;
  }
  else {
    pass1_1008_3bd6(paVar3,(ushort)puVar6,0x0,0xf1,0x0,0x1ac01ad,CONCAT22(*(undefined2 *)(uVar11 + 0x8),0xf4),
                    (ushort)puVar7,unaff_SS);
    *(astruct_160 **)(uVar11 + 0x102) = paVar3;
    *(uchar **)(uVar11 + 0x104) = puVar7;
  }
  HVar9 = 0x1000;
  mem_op_1000_179c(0x42,puVar7,0x1000);
  uVar8 = (uint)puVar7 | (uint)paVar3;
  if (uVar8 == 0x0) {
    *(undefined4 *)(uVar11 + 0x106) = 0x0;
  }
  else {
    HVar9 = 0x1008;
    pass1_1008_3bd6(paVar3,(ushort)puVar7,0x0,0xf500f1,0x0,0x1ae01af,CONCAT22(*(undefined2 *)(uVar11 + 0x8),0xf5),uVar8,
                    unaff_SS);
    *(astruct_160 **)(uVar11 + 0x106) = paVar3;
    *(uint *)(uVar11 + 0x108) = uVar8;
  }
  uVar4 = *(undefined4 *)(uVar11 + 0xfa);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar11 + 0xfa) + 0x10);
  (**ppcVar1)(HVar9,(int)uVar4,(int)((ulong)uVar4 >> 0x10));
  puVar7 = (uchar *)paVar3->field_0x2;
  uVar8 = MoveWindow16(HVar9,0x1,*(INT16 *)&paVar3->field_0x6,*(INT16 *)&paVar3->field_0x4,(INT16)puVar7,
                       *(BOOL16 *)paVar3);
  HVar9 = 0x1000;
  mem_op_1000_179c(0x8e,puVar7,0x1000);
  uVar2 = (uint)puVar7 | uVar8;
  if (uVar2 == 0x0) {
    *(undefined4 *)(uVar11 + 0x10a) = 0x0;
  }
  else {
    HVar9 = (HWND16)&PTR_LOOP_1050_1040;
    get_sys_metrics_1040_7728((astruct_57 *)CONCAT22(puVar7,uVar8),0x1,0x0,0xfc0,*(ushort *)(uVar11 + 0x8));
    *(uint *)(uVar11 + 0x10a) = uVar8;
    *(uint *)(uVar11 + 0x10c) = uVar2;
  }
  uVar4 = *(undefined4 *)(uVar11 + 0x10a);
  *(undefined2 *)((int)uVar4 + 0x74) = 0x1;
  uVar4 = *(undefined4 *)(uVar11 + 0x10a);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar11 + 0x10a) + 0x8);
  (**ppcVar1)(HVar9,(int)uVar4,(char)((ulong)uVar4 >> 0x10));
  if ((*(uint *)(uVar11 + 0x10c) | *(uint *)(uVar11 + 0x10a)) != 0x0) {
    GetWindowRect16(HVar9,&local_a);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_12);
    iStack12 = iStack12 - local_12.y;
    iStack14 = iStack6 - local_a.x;
    local_12.x = local_a.x;
    local_12.y = iStack4 + 0x3;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x44,iStack12,iStack14,local_12.y,local_a.x,0x0);
  }
  return;
}



void __stdcall16far destroy_window_1020_3b3e(astruct_30 *param_1,HWND16 param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  uint uVar3;
  astruct_30 *paVar4;
  astruct_30 *uVar5;
  astruct_30 *uVar6;
  HWND16 HVar5;
  ushort unaff_SS;
  
  uVar6 = (astruct_30 *)((ulong)param_1 >> 0x10);
  uVar5 = (astruct_30 *)param_1;
  uVar5->field_0x10e = 0x0;
  HVar5 = param_2;
  if (uVar5->field_0x10a != 0x0) {
    HVar5 = (HWND16)s_tile2_bmp_1050_1538;
    DestroyWindow16(param_2);
  }
  puVar1 = uVar5->field_0xf6;
  uVar3 = uVar5->field_0xf8;
  if ((uVar3 | (uint)puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)(HVar5,puVar1);
  }
  *(undefined4 *)&uVar5->field_0xf6 = 0x0;
  if (uVar5->field_0xfa != 0x0) {
    uVar3 = (uint)uVar6 | (uint)uVar5;
    if (param_1 == (astruct_30 *)0x0) {
      paVar4 = (astruct_30 *)0x0;
    }
    else {
      uVar3 = &uVar5->field_0xf2;
      paVar4 = uVar6;
    }
    pass1_1010_1ea6(uVar5->field_0xfa,CONCAT22(paVar4,uVar3),unaff_SS);
  }
  uVar5->field_0xfa = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_3bd6(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int iVar1;
  ushort uVar2;
  undefined2 uVar3;
  undefined4 uVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar2 = (ushort)param_1;
  mixed_draw_op_1020_3fa0(*(ulong *)(uVar2 + 0xf6),param_3,param_4);
  if (*(int *)(uVar2 + 0x100) == 0x0) {
    *(undefined2 *)(uVar2 + 0x100) = 0x1;
    uVar4 = *(undefined4 *)(uVar2 + 0xfa);
    if (*(int *)((int)uVar4 + 0x56) == 0x0) {
      iVar1 = 0x5;
    }
    else {
      iVar1 = 0x8;
    }
    uVar4 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar2 + 0x8),iVar1,param_2,uVar2,(ushort)&PTR_LOOP_1050_1038
                            ,param_4);
    *(undefined2 *)(uVar2 + 0x10e) = (int)uVar4;
    *(undefined2 *)(uVar2 + 0x110) = (int)((ulong)uVar4 >> 0x10);
  }
  return;
}



void __stdcall16far pass1_1020_3c32(int param_1,ushort param_2,uint param_3,ushort param_4)

{
  char cVar1;
  int iVar2;
  
  if (param_3 == 0xf5) {
    iVar2 = 0x1;
LAB_1020_3c52:
    pass1_1018_1b02(param_4,*(ulong *)(param_1 + 0xfa),iVar2);
    return;
  }
  if ((param_3 < 0xf6) && (cVar1 = (char)param_3, cVar1 != '\0')) {
    if (cVar1 == '\x01' || cVar1 == '\x02') {
      return;
    }
    if (cVar1 == -0xc) {
      iVar2 = 0x0;
      goto LAB_1020_3c52;
    }
  }
  pass1_1020_3c32(param_1,param_2,param_3,param_4);
  return;
}



void __stdcall16far pass1_1020_3c74(ushort param_1,ulong param_2,ushort param_3,ushort param_4)

{
  pass1_1020_3c8c(CONCAT22((int)param_2,param_1),CONCAT22(param_3,(int)(param_2 >> 0x10)),param_4);
  return;
}



void __stdcall16far pass1_1020_3c8c(ulong param_1,ulong param_2,ushort param_3)

{
  pt_in_rect_1018_1bda(*(ulong *)((int)param_1 + 0xfa),(ushort)param_2,(ushort)(param_2 >> 0x10),param_3);
  return;
}



astruct_3 * __stdcall16far pass1_1020_3ca6(astruct_3 *param_1,byte param_2,undefined2 param_3)

{
  ulong uVar1;
  undefined2 *puStack10;
  
  uVar1 = (ulong)param_1 & 0xffff0000;
  param_1 = (astruct_3 *)(uVar1 | (int)param_1 - 0xf2);
  param_1._2_2_ = (undefined2)(uVar1 >> 0x10);
  if (param_1 == (astruct_3 *)0x0) {
    param_1._0_2_ = 0x0;
    param_1._2_2_ = 0x0;
  }
  puStack10 = (undefined2 *)CONCAT22(param_1._2_2_,(int)param_1);
  *puStack10 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10203dab) overlaps instruction at (ram,0x10203da8)
// 
// WARNING: Variable defined which should be unmapped: param_16
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_3d08(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7
               ,ushort param_8,ushort param_9,uchar param_10,uchar param_11,uchar param_12,uchar param_13,uchar param_14
               ,ulong param_15,ushort param_16,ushort param_17,ushort param_18,ushort param_19)

{
  char *pcVar1;
  byte *pbVar2;
  bool bVar3;
  bool bVar4;
  code **ppcVar5;
  undefined2 *puVar6;
  undefined4 uVar7;
  undefined4 *puVar8;
  ulong uVar9;
  byte bVar10;
  byte bVar12;
  int iVar13;
  byte bVar18;
  char cVar19;
  HDC16 HVar14;
  int16_t iVar15;
  HGDIOBJ16 HVar16;
  undefined *puVar17;
  uint uVar20;
  uint uVar21;
  byte bVar22;
  byte bVar23;
  uchar *puVar24;
  byte bVar25;
  char *pcVar26;
  char *pcVar27;
  undefined2 uVar28;
  undefined2 uVar29;
  bool bVar30;
  bool bVar31;
  ushort *puVar32;
  code *pcStack4;
  byte bVar11;
  
  uVar9 = CONCAT22(param_19,param_18);
  bVar22 = (char)param_2 + (char)(param_1 >> 0x8) + param_10;
  *(undefined *)param_6 = 0x3c;
  puVar24 = (uchar *)CONCAT11((char)(param_2 >> 0x8) + '<' + (*(byte *)(param_3 + param_5) < 0x20),bVar22);
  pcStack4 = switchD_1008:1091::caseD_a7;
  iVar13 = 0x203d;
  pbVar2 = (byte *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (byte *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 & bVar22;
  pbVar2 = (byte *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (byte *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (byte *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  bVar10 = (byte)(param_6 + 0x2);
  bVar12 = 0x9 < (bVar10 & 0xf) | param_11;
  bVar11 = bVar10 + bVar12 * '\x06' & 0xf;
  pbVar2 = (byte *)(param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  bVar10 = 0x9 < bVar11 | bVar12;
  uVar20 = CONCAT11((char)(param_6 + 0x2 >> 0x8) + bVar12 + bVar10,bVar11 + bVar10 * '\x06') & 0xff0f;
  pcVar27 = (char *)CONCAT11(0x79,(char)param_5 + -0x37);
  do {
    pcVar26 = pcVar27;
    pbVar2 = (byte *)(param_3 + iVar13);
    bVar23 = (byte)puVar24;
    *pbVar2 = *pbVar2 | bVar23;
    bVar12 = (byte)(uVar20 - 0x1);
    bVar3 = 0x9 < (bVar12 & 0xf);
    bVar22 = bVar3 | bVar10;
    pbVar2 = (byte *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar23;
    bVar4 = 0x9 < (bVar12 + bVar22 * '\x06' & 0xf);
    bVar18 = (char)(uVar20 - 0x1 >> 0x8) + bVar22 + (bVar4 | bVar22);
    pbVar2 = (byte *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar23;
    uVar20 = 0x0;
    bVar30 = &pcStack4 < (undefined *)*(uint *)(param_3 + iVar13);
    pbVar2 = (byte *)(param_3 + iVar13 + 0x896);
    bVar25 = (byte)param_3;
    bVar31 = CARRY1(*pbVar2,bVar25) || CARRY1(*pbVar2 + bVar25,bVar30);
    *pbVar2 = *pbVar2 + bVar25 + bVar30;
    pbVar2 = (byte *)(param_3 + iVar13 + 0x2038);
    bVar12 = *pbVar2;
    bVar11 = *pbVar2;
    *pbVar2 = bVar11 + bVar25 + bVar31;
    pcVar1 = (char *)(param_4 + iVar13);
    *pcVar1 = *pcVar1 + (char)((uint)puVar24 >> 0x8) + (CARRY1(bVar12,bVar25) || CARRY1(bVar11 + bVar25,bVar31));
    pcVar1 = (char *)(param_3 + iVar13 + -0x64);
    *pcVar1 = *pcVar1 + bVar18 + '\x01';
    pbVar2 = (byte *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar23;
    pcVar1 = pcVar26;
    pcVar27 = pcVar26 + 0x1;
    bVar30 = *pcVar1 != '\0';
    if (-*pcVar1 < '\0') {
      pcVar1 = (char *)(param_4 + 0x37);
      *pcVar1 = *pcVar1 + bVar25 + bVar30;
      pbVar2 = (byte *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar23;
      iVar13 = iVar13 + -0x1;
      pbVar2 = (byte *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar23;
      pbVar2 = (byte *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar23;
      puVar24 = puVar24 + -0x1;
      pbVar2 = (byte *)(param_3 + iVar13);
      bVar12 = (byte)puVar24;
      *pbVar2 = *pbVar2 | bVar12;
      if (*pbVar2 == 0x0) {
        pbVar2 = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar12;
code_r0x10203d96:
        pbVar2 = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 | (byte)puVar24;
        pbVar2 = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 & (byte)puVar24;
        pcVar27 = pcVar26 + 0x2;
        uVar21 = (uint)puVar24 & 0xff;
        puVar24 = (uchar *)(uVar21 | (uint)(byte)((char)((uint)puVar24 >> 0x8) * '\x02' + ((byte)uVar20 < 0x20)) << 0x8)
        ;
        pbVar2 = (byte *)(param_3 + iVar13 + 0x1);
        *pbVar2 = *pbVar2 & (byte)uVar21;
        param_4 = (ushort)&stack0xfff6;
        param_16 = (ushort)pcStack4;
        param_17 = (ushort)((ulong)pcStack4 >> 0x10);
        uVar9 = param_15;
code_r0x10203db1:
        get_sys_metrics_1020_7c1a((ushort *)CONCAT22(param_17,param_16),uVar9,param_8);
        puVar6 = (undefined2 *)*(undefined4 *)(param_4 + 0x6);
        uVar28 = (undefined2)((ulong)puVar6 >> 0x10);
        *(undefined4 *)((int)puVar6 + 0x14) = 0x0;
        *puVar6 = 0x408a;
        *(undefined2 *)((int)puVar6 + 0x2) = 0x1020;
        puVar32 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_9,puVar24,(int)pcVar27);
        uVar28 = (undefined2)((ulong)puVar32 >> 0x10);
        uVar7 = *(undefined4 *)(param_4 + 0x6);
        uVar29 = (undefined2)((ulong)uVar7 >> 0x10);
        iVar13 = (int)uVar7;
        *(undefined2 *)(iVar13 + 0x14) = (int)puVar32;
        *(undefined2 *)(iVar13 + 0x16) = uVar28;
        ppcVar5 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar13 + 0x14) + 0x4);
        (**ppcVar5)(0x1010,*(undefined2 *)(iVar13 + 0x14),uVar28,0x0,iVar13,uVar29);
        HVar14 = GetDC16(0x1010);
        *(HDC16 *)(param_4 + -0x2) = HVar14;
        iVar15 = SetMapMode16((HDC16)s_tile2_bmp_1050_1538,0x1);
        uVar7 = *(undefined4 *)(param_4 + 0x6);
        *(int16_t *)((int)uVar7 + 0x1e) = iVar15;
        uVar28 = *(undefined2 *)(param_4 + -0x2);
        HVar16 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        HVar16 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar16);
        uVar7 = *(undefined4 *)(param_4 + 0x6);
        *(HGDIOBJ16 *)((int)uVar7 + 0x18) = HVar16;
        uVar29 = *(undefined2 *)(param_4 + -0x2);
        HVar16 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        HVar16 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar16);
        uVar7 = *(undefined4 *)(param_4 + 0x6);
        *(HGDIOBJ16 *)((int)uVar7 + 0x1a) = HVar16;
        uVar7 = *(undefined4 *)(param_4 + 0x6);
        uVar7 = *(undefined4 *)((int)uVar7 + 0x14);
        *(undefined4 *)(param_4 + -0x6) = *(undefined4 *)((int)uVar7 + 0x24);
        puVar17 = (undefined *)(param_4 + -0x2);
        puVar8 = (undefined4 *)*(undefined4 *)(param_4 + -0x6);
        ppcVar5 = (code **)((int)*puVar8 + 0x8);
        (**ppcVar5)((int)s_tile2_bmp_1050_1538,(int)puVar8,(int)((ulong)puVar8 >> 0x10),puVar17,param_9,uVar29,uVar28);
        uVar7 = *(undefined4 *)(param_4 + 0x6);
        uVar29 = (undefined2)((ulong)uVar7 >> 0x10);
        iVar13 = (int)uVar7;
        *(int *)(iVar13 + 0x1c) = (int)puVar17;
        uVar28 = *(undefined2 *)(param_4 + -0x2);
        *(undefined2 *)(param_4 + -0x14) = uVar28;
        uVar7 = *(undefined4 *)(iVar13 + 0x14);
        *(undefined2 *)((int)uVar7 + 0x4c) = uVar28;
        return;
      }
      pbVar2 = (byte *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 & bVar12;
      pcVar1 = (char *)(param_4 + iVar13 + 0x20);
      bVar11 = (byte)param_1 & 0x1f;
      cVar19 = *pcVar1;
      *pcVar1 = *pcVar1 >> bVar11;
      pcVar1 = (char *)(param_4 + iVar13 + 0x6a);
      *pcVar1 = *pcVar1 + (byte)param_1 + ((param_1 & 0x1f) != 0x0 && (cVar19 >> bVar11 - 0x1 & 0x1U) != 0x0);
      pbVar2 = (byte *)(param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar12;
      param_8 = 0x1020;
      uVar20 = *(int *)(param_3 + iVar13) * 0x10;
      pbVar2 = (byte *)(pcVar27 + param_4 + 0x8);
      bVar12 = (byte)(uVar20 >> 0x8);
      uVar21 = uVar20 & 0xff | (uint)(byte)(bVar12 + *pbVar2) << 0x8;
      pcVar1 = (char *)(param_4 + iVar13 + 0x37);
      *pcVar1 = *pcVar1 + (char)(param_3 >> 0x8) + CARRY1(bVar12,*pbVar2);
    }
    else {
      pcVar1 = (char *)(param_4 + iVar13);
      *pcVar1 = *pcVar1 + bVar30;
      uVar21 = *(int *)(param_3 + iVar13) * 0x10;
      if ((POPCOUNT(*pcVar1) & 0x1U) == 0x0) goto code_r0x10203db1;
    }
    pbVar2 = (byte *)(param_3 + iVar13);
    bVar11 = (byte)puVar24;
    *pbVar2 = *pbVar2 | bVar11;
    pbVar2 = (byte *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_16 = (uint)(param_14 & 0x1) * 0x4000 | (uint)(param_13 & 0x1) * 0x200 | (uint)(param_12 & 0x1) * 0x100 |
               (uint)((char)*pbVar2 < '\0') * 0x80 | (uint)(*pbVar2 == 0x0) * 0x40 |
               (uint)(byte)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10 | (uint)((POPCOUNT(*pbVar2) & 0x1U) == 0x0) * 0x4;
    pbVar2 = (byte *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    bVar12 = in(0x79);
    pbVar2 = (byte *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 & bVar11;
    if (-0x1 < (char)*pbVar2) {
      uVar9 = CONCAT22(param_19,param_18);
      if ((bVar18 | *(byte *)(param_4 - 0x1)) == 0x0) {
        param_16 = param_7;
        uVar9 = CONCAT22(param_19,param_18);
      }
      goto code_r0x10203db1;
    }
    pbVar2 = (byte *)(param_4 + 0x89c);
    bVar30 = CARRY1(*pbVar2,bVar12);
    *pbVar2 = *pbVar2 + bVar12;
    bVar23 = bVar18 + bVar12;
    cVar19 = bVar23 + bVar30;
    uVar20 = CONCAT11(cVar19,bVar12);
    pcStack4._0_3_ =
         CONCAT21((uint)(param_14 & 0x1) * 0x4000 | (uint)(SCARRY1(bVar18,bVar12) != SCARRY1(bVar23,bVar30)) * 0x800 |
                  (uint)(param_13 & 0x1) * 0x200 | (uint)(param_12 & 0x1) * 0x100 | (uint)(cVar19 < '\0') * 0x80 |
                  (uint)(cVar19 == '\0') * 0x40 | (uint)(byte)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                  (uint)((POPCOUNT(cVar19) & 0x1U) == 0x0) * 0x4 |
                  (uint)(CARRY1(bVar18,bVar12) || CARRY1(bVar23,bVar30)),pcStack4._0_1_);
    pcStack4 = (code *)((ulong)pcStack4 & 0xff000000 | (ulong)(uint3)pcStack4);
    pbVar2 = (byte *)(param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_1 = uVar21 - 0x1;
    bVar10 = bVar4 | bVar22;
    if (param_1 == 0x0 || *pbVar2 == 0x0) goto code_r0x10203d96;
  } while( true );
}
