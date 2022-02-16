


astruct_18 * __stdcall16far pass1_1040_be94(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1040_bf3e(ushort *param_1,ushort param_2)

{
  astruct_442 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_442 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x6 = 0x0;
  *param_1 = 0xc53e;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  return param_1;
}



void __stdcall16far pass1_1040_bf92(ushort *param_1,ushort param_2)

{
  astruct_514 *iVar1;
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_514 *)param_1;
  *param_1 = 0xc53e;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  pass1_1010_1ea6(iVar1->field_0x6,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,param_2);
  unk_destroy_win_op_1010_2fa0(iVar1->field_0x6,0x1010);
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1040_bfde(ulong param_1,ulong *param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(ulong **)(iVar3 + 0x6) = param_2;
  ppcVar1 = (code **)((int)*param_2 + 0x4);
  (**ppcVar1)();
  uVar2 = *(undefined4 *)(iVar3 + 0x6);
  *(undefined2 *)((int)uVar2 + 0x22) = *(undefined2 *)(iVar3 + 0x4);
  pass1_1010_2ee2(*(ulong **)(iVar3 + 0x6),param_3,0x1010);
  return;
}



void __stdcall16far invalidate_rect_1040_c028(ulong param_1,int param_2,HWND16 param_3,RECT16 *param_4)

{
  undefined4 uVar1;
  ulong uVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  undefined2 uVar9;
  RECT16 *rect;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  int *piVar8;
  
  iVar7 = (int)param_1;
  uVar9 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x8) {
    GetClientRect16(param_3,&local_a);
    uVar1 = *(undefined4 *)(iVar7 + 0x6);
    uVar3 = *(undefined4 *)(iVar7 + 0x6);
    iVar5 = *(int *)((int)uVar3 + 0x16);
    uVar3 = *(undefined4 *)(iVar7 + 0x6);
    local_a.x = *(int *)((int)uVar3 + 0x1a);
    uVar3 = *(undefined4 *)(iVar7 + 0x6);
    local_a.y = *(int *)((int)uVar3 + 0x1c);
    if (iVar5 != 0x0) {
      if (iVar5 < 0x2) {
        iVar4 = 0x1;
      }
      else {
        iVar4 = 0x2;
      }
      uVar2 = *(ulong *)((iVar5 - iVar4) * 0x4 + (int)uVar1 + 0x2a);
      iVar5 = (int)uVar2;
      uVar2 = uVar2 & 0xffff0000;
      local_a.x = *(int *)(iVar5 + 0x22) + *(int *)(uVar2 | iVar5 + 0x1e);
    }
    uVar1 = *(undefined4 *)(iVar7 + 0x6);
    iStack6 = *(int *)((int)uVar1 + 0x1e);
    iStack4 = iStack4 + -0x5;
  }
  else {
    if (param_2 != 0x9) {
      if (param_2 != 0xa) {
        return;
      }
      uVar1 = *(undefined4 *)(iVar7 + 0x6);
      uVar6 = (int)uVar1 + 0x2a;
      if ((*(uint *)(iVar7 + 0x8) | uVar6) == 0x0) {
        return;
      }
      uVar3 = *(undefined4 *)(iVar7 + 0x6);
      uVar2 = *(ulong *)((*(int *)((int)uVar3 + 0x16) + -0x1) * 0x4 + uVar6);
      iVar7 = (int)uVar2;
      uVar2 = uVar2 & 0xffff0000;
      piVar8 = (int *)(uVar2 | iVar7 + 0x1e);
      uVar9 = (undefined2)(uVar2 >> 0x10);
      local_a.y = *(int *)(iVar7 + 0x20) + -0x8;
      local_a.x = *piVar8;
      iStack6 = *(int *)(iVar7 + 0x22) + *piVar8;
      iStack4 = *(int *)(iVar7 + 0x20);
      param_4 = &local_a;
      rect = (RECT16 *)0x0;
      goto LAB_1040_c19d;
    }
    local_a.x = 0x0;
    local_a.y = 0x0;
    iStack6 = 0x0;
    iStack4 = 0x0;
    GetClientRect16(param_3,&local_a);
    uVar1 = *(undefined4 *)(iVar7 + 0x6);
    local_a.x = *(int *)((int)uVar1 + 0x1a);
    uVar1 = *(undefined4 *)(iVar7 + 0x6);
    iStack6 = *(int *)((int)uVar1 + 0x1e);
    iStack4 = iStack4 + -0x5;
    uVar1 = *(undefined4 *)(iVar7 + 0x6);
    uVar3 = *(undefined4 *)(iVar7 + 0x6);
    iVar7 = *(int *)((int)uVar3 + 0x16);
    if (0x0 < iVar7) {
      uVar1 = *(undefined4 *)((int)uVar1 + iVar7 * 0x4 + 0x26);
      iVar7 = (int)uVar1;
      uVar9 = (undefined2)((ulong)uVar1 >> 0x10);
      local_a.y = *(int *)(iVar7 + 0x20) + *(int *)(iVar7 + 0x24);
    }
  }
  param_3 = (HWND16)s_tile2_bmp_1050_1538;
  rect = &local_a;
LAB_1040_c19d:
  InvalidateRect16(param_3,rect,(BOOL16)param_4);
  return;
}



void __stdcall16far unk_draw_op_1040_c226(ulong param_1,HWND16 param_2,ushort param_3)

{
  undefined4 uVar1;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  undefined2 uVar2;
  RECT16 local_32;
  int iStack46;
  int iStack44;
  undefined2 uStack42;
  int iStack40;
  RECT16 *pRStack38;
  HDC16 HStack36;
  PAINTSTRUCT16 local_22;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  HStack36 = BeginPaint16(param_2,&local_22);
  pRStack38 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_32);
  uVar1 = *(undefined4 *)((int)param_1 + 0x6);
  iStack40 = *(int *)((int)uVar1 + 0x1a);
  uVar1 = *(undefined4 *)((int)param_1 + 0x6);
  uStack42 = *(undefined2 *)((int)uVar1 + 0x1c);
  local_32.y = local_32.y + 0x2;
  local_32.x = iStack40 + -0xa;
  iStack46 = iStack46 + -0x2;
  iStack44 = iStack44 + -0x2;
  FrameRect16((HDC16)s_tile2_bmp_1050_1538,pRStack38,(HBRUSH16)&local_32);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,-0x7f80,0x0);
  handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  draw_line_1040_c302(param_1,(int)s_tile2_bmp_1050_1538);
  draw_op_1040_c38e(param_1);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



void __stdcall16far draw_line_1040_c302(ulong param_1,HDC16 param_2)

{
  ulong uVar1;
  ulong uVar2;
  undefined4 uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  undefined2 uVar7;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  uVar3 = *(undefined4 *)((int)param_1 + 0x6);
  iVar6 = *(int *)((int)uVar3 + 0x16);
  if (0x1 < iVar6) {
    uVar1 = *(ulong *)((int)param_1 + 0x6);
    uVar4 = (int)uVar1 + 0x2a;
    uVar1 = uVar1 & 0xffff0000;
    uVar2 = *(ulong *)(uVar1 | uVar4);
    iVar5 = (int)uVar2;
    uVar2 = uVar2 & 0xffff0000;
    uVar7 = (undefined2)(uVar2 >> 0x10);
    MoveTo16(param_2,*(int *)(iVar5 + 0x20) + *(int *)(iVar5 + 0x24),
             *(int *)(iVar5 + 0x22) / 0x2 + *(int *)(uVar2 | iVar5 + 0x1e));
    uVar1 = *(ulong *)(uVar4 + iVar6 * 0x4 + -0x4);
    iVar6 = (int)uVar1;
    uVar1 = uVar1 & 0xffff0000;
    uVar7 = (undefined2)(uVar1 >> 0x10);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,*(INT16 *)(iVar6 + 0x20),
             *(int *)(iVar6 + 0x22) / 0x2 + *(int *)(uVar1 | iVar6 + 0x1e));
  }
  return;
}



void __stdcall16far draw_op_1040_c38e(ulong param_1)

{
  ulong uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  INT16 *pIVar6;
  undefined2 in_DX;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  HDC16 hdc;
  ushort unaff_SS;
  DWORD DVar11;
  int iStack26;
  INT16 IStack20;
  int iStack18;
  INT16 IStack16;
  int iStack14;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar2 = *(undefined4 *)(iVar8 + 0x6);
  iVar7 = *(int *)((int)uVar2 + 0x18);
  if ((iVar7 != 0x0) && (uVar2 = *(undefined4 *)(iVar8 + 0x6), *(int *)((int)uVar2 + 0x16) != 0x0)) {
    hdc = 0x1010;
    iVar4 = iVar7;
    pass1_1010_2ee2(*(ulong **)(iVar8 + 0x6),unaff_SS,0x1010);
    for (iStack26 = 0x0; iStack26 < iVar7; iStack26 = iStack26 + 0x1) {
      uVar1 = *(ulong *)(iStack26 * 0x4 + iVar4);
      iVar5 = (int)uVar1;
      uVar1 = uVar1 & 0xffff0000;
      pIVar6 = (INT16 *)(uVar1 | iVar5 + 0x1e);
      uVar10 = (undefined2)(uVar1 >> 0x10);
      iVar5 = *(int *)(iVar5 + 0x24) / 0x2 + *(int *)(iVar5 + 0x20);
      MoveTo16(hdc,iVar5,*pIVar6);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iVar5,*pIVar6 + -0xf);
      hdc = (HDC16)s_tile2_bmp_1050_1538;
      DVar11 = GetCurrentPosition16((HDC16)s_tile2_bmp_1050_1538);
      iStack18 = (int)(DVar11 >> 0x10);
      IStack20 = (INT16)DVar11;
      if (iStack26 == 0x0) {
        IStack16 = IStack20;
        iStack14 = iStack18;
      }
    }
    uVar2 = *(undefined4 *)(iVar8 + 0x6);
    if (*(int *)((int)uVar2 + 0x24) != 0x0) {
      iStack14 = iStack14 + -0xd;
    }
    uVar2 = *(undefined4 *)(iVar8 + 0x6);
    if (*(int *)((int)uVar2 + 0x26) != 0x0) {
      iStack18 = iStack18 + 0xd;
    }
    uVar2 = *(undefined4 *)(iVar8 + 0x6);
    uVar3 = *(undefined4 *)(iVar8 + 0x6);
    uVar1 = *(ulong *)((int)uVar2 + *(int *)((int)uVar3 + 0x16) * 0x4 + 0x26);
    iVar7 = (int)uVar1;
    uVar1 = uVar1 & 0xffff0000;
    uVar9 = (undefined2)(uVar1 >> 0x10);
    MoveTo16(hdc,*(int *)(iVar7 + 0x24) / 0x2 + *(int *)(iVar7 + 0x20),
             *(int *)(iVar7 + 0x22) + *(int *)(uVar1 | iVar7 + 0x1e));
    LineTo16((HDC16)s_tile2_bmp_1050_1538,*(int *)(iVar7 + 0x24) / 0x2 + *(int *)(iVar7 + 0x20),IStack16);
    DVar11 = GetCurrentPosition16((HDC16)s_tile2_bmp_1050_1538);
    iVar7 = (int)(DVar11 >> 0x10);
    if (iVar7 < iStack14) {
      iStack14 = iVar7;
    }
    if (iStack18 < iVar7) {
      iStack18 = iVar7;
    }
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,iStack14,IStack16);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack18,IStack20);
  }
  return;
}



ulong __stdcall16far pass1_1040_c518(ulong param_1,byte param_2,ushort param_3)

{
  pass1_1040_bf92((ushort *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_c54a(ushort *param_1,ushort param_2,ulong *param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  int iVar3;
  ushort extraout_DX;
  astruct_164 *iVar2;
  uint uVar4;
  ulong *puVar5;
  ushort uVar6;
  undefined4 uVar7;
  
  iVar3 = *(int *)((int)param_3 + 0x12) + 0xc8;
  uVar7 = 0x0;
  uVar6 = 0x0;
  ppcVar1 = (code **)((int)*param_3 + 0x14);
  puVar5 = param_3;
  (**ppcVar1)();
  mixed_struct_op_1040_8fb8
            (param_1,0x0,(char *)CONCAT22(extraout_DX,iVar3),(ushort)puVar5,(ushort)((ulong)puVar5 >> 0x10),uVar6,
             (ushort)uVar7,(ushort)((ulong)uVar7 >> 0x10),extraout_DX,param_4,param_5);
  uVar4 = (uint)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_164 *)param_1;
  iVar2->field_0x42 = param_3;
  iVar2->field_0x46 = 0x0;
  iVar2->field_0x48 = param_2;
  *param_1 = 0xc9f2;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  pass1_1040_c630((ulong *)((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10),param_4,param_5);
  return;
}



void __stdcall16far pass1_1040_c5ac(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0xc9f2;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + -0x1;
  if (PTR_LOOP_1050_5f02 == (undefined *)0x0) {
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x8);
    uVar2 = *(uint *)(iVar4 + 0xa);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xc);
    uVar2 = *(uint *)(iVar4 + 0xe);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
  }
  mix_win_ui_op_1040_911e(param_1);
  return;
}



ushort __stdcall16far pass1_1040_c60e(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x42) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x42);
    return *(ushort *)((int)uVar1 + 0x12);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_c630(ulong *param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  code **ppcVar2;
  undefined4 uVar3;
  ulong uVar4;
  astruct_165 *iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_165 *)param_1;
  uVar3 = iVar4->field_0x42;
  if (*(int *)((int)uVar3 + 0x12) != 0x71) {
    iVar4->field_0x36 = 0x5;
    iVar4->field_0x26 = 0x5;
    iVar4->field_0x28 = 0x5;
    iVar1 = iVar4->field_0x36;
    iVar4->field_0x30 = iVar1;
    iVar4->field_0x2e = iVar1;
    if (PTR_LOOP_1050_5f02 == (undefined *)0x0) {
      _PTR_LOOP_1050_5f04 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0xff,param_3);
      param_2 = 0x1010;
      _PTR_LOOP_1050_5f08 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x100,param_3);
    }
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + 0x1;
    iVar4->field_0x8 = _PTR_LOOP_1050_5f04;
    iVar4->field_0xc = _PTR_LOOP_1050_5f08;
    pass1_1040_9618((ulong)param_1);
    iVar4->field_0x20 = 0x0;
    iVar4->field_0x1e = 0xc8;
    iVar4->field_0x22 = 0xa0;
    iVar4->field_0x24 = iVar4->field_0x2c + iVar4->field_0x36;
    iVar4->field_0x2e = iVar4->field_0x36 * 0x3 + iVar4->field_0x2a;
    iVar4->field_0x30 = iVar4->field_0x36;
    iVar4->field_0x32 = iVar4->field_0x22 - iVar4->field_0x36;
    iVar4->field_0x3c = 0x25;
    uVar4 = *param_1;
    ppcVar2 = (code **)((int)uVar4 + 0x4);
    (**ppcVar2)(param_2,param_1);
    ppcVar2 = (code **)((int)uVar4 + 0x8);
    (**ppcVar2)(param_2,(char)param_1,uVar5);
  }
  return;
}



void __stdcall16far pass1_1040_c71e(ulong param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  pass1_1040_9252(param_1,param_2);
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(int *)(iVar1 + 0x28) = *(int *)(iVar1 + 0x24) / 0x2 - *(int *)(iVar1 + 0x2c) / 0x2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far draw_op_1040_c74c(ulong *param_1,ulong param_2,ushort param_3)

{
  ushort uVar1;
  code **ppcVar2;
  ulong uVar3;
  HPALETTE16 b_force_background;
  HGDIOBJ16 HVar4;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  int iVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)((ulong)_PTR_LOOP_1050_4230 >> 0x10);
  uVar1 = *(ushort *)((int)_PTR_LOOP_1050_4230 + 0x10);
  b_force_background =
       palette_op_1008_4e08
                 ((astruct_13 *)CONCAT22(uVar1,*(undefined2 *)((int)_PTR_LOOP_1050_4230 + 0xe)),(int)&param_2 + 0x2,
                  uVar1,0x1008);
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *(undefined2 *)(iVar5 + 0x46) = 0x1;
  HVar4 = GetStockObject16(0x1008);
  handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,0x2,0x100);
  HVar4 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar4);
  handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  Rectangle16((HDC16)s_tile2_bmp_1050_1538,*(INT16 *)(iVar5 + 0x24),*(INT16 *)(iVar5 + 0x22),0x0,0x0);
  MoveTo16((HDC16)s_tile2_bmp_1050_1538,0x0,*(int *)(iVar5 + 0x36) * 0x2 + *(int *)(iVar5 + 0x2a));
  LineTo16((HDC16)s_tile2_bmp_1050_1538,*(INT16 *)(iVar5 + 0x24),*(int *)(iVar5 + 0x36) * 0x2 + *(int *)(iVar5 + 0x2a));
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar4);
  HVar4 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  uVar3 = *param_1;
  ppcVar2 = (code **)((int)uVar3 + 0x10);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,param_1,param_2,HVar4,param_2._2_2_);
  ppcVar2 = (code **)((int)uVar3 + 0x14);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,param_1,param_2._2_2_);
  *(undefined2 *)(iVar5 + 0x46) = 0x0;
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far send_msg_1040_c85a(ulong param_1,HWND16 param_2)

{
  _PTR_LOOP_1050_5efe = param_1;
  SendMessage16(param_2,0x0,0x0,0x11100fa);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far palette_op_1040_c886(ulong param_1,byte param_2,ushort param_3,HDC16 param_4)

{
  ushort uVar1;
  code **ppcVar2;
  int iVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  undefined4 *puStack8;
  HPALETTE16 HStack4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if ((*(uint *)(iVar3 + 0xa) | *(uint *)(iVar3 + 0x8)) != 0x0) {
    HStack4 = 0x0;
    if (*(int *)(iVar3 + 0x46) == 0x0) {
      uVar5 = (undefined2)((ulong)_PTR_LOOP_1050_4230 >> 0x10);
      uVar1 = *(ushort *)((int)_PTR_LOOP_1050_4230 + 0x10);
      param_4 = 0x1008;
      HStack4 = palette_op_1008_4e08
                          ((astruct_13 *)CONCAT22(uVar1,*(undefined2 *)((int)_PTR_LOOP_1050_4230 + 0xe)),&param_3,uVar1,
                           0x1008);
    }
    puStack8 = (undefined4 *)*(undefined4 *)(iVar3 + 0x8);
    uVar5 = *(undefined2 *)(iVar3 + 0xa);
    if (((*(uint *)(iVar3 + 0xe) | *(uint *)(iVar3 + 0xc)) != 0x0) && ((param_2 & 0x1) != 0x0)) {
      puStack8 = (undefined4 *)*(undefined4 *)(iVar3 + 0xc);
      uVar5 = *(undefined2 *)(iVar3 + 0xe);
    }
    if ((*(long *)(iVar3 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0)) {
      puStack8 = (undefined4 *)*(undefined4 *)(iVar3 + 0x10);
      uVar5 = *(undefined2 *)(iVar3 + 0x12);
    }
    ppcVar2 = (code **)((int)*puStack8 + 0x4);
    (**ppcVar2)(param_4,(int)puStack8,uVar5,*(undefined2 *)(iVar3 + 0x28),*(undefined2 *)(iVar3 + 0x26),&param_3);
    if (*(int *)(iVar3 + 0x46) == 0x0) {
      SelectPalette16(param_4,0x0,HStack4);
      DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_c94a(int param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  undefined4 uVar2;
  ushort uVar3;
  ushort uVar4;
  ushort *puVar5;
  
  if (*(int *)(param_1 + 0x48) != 0x0) {
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,param_4);
    uVar4 = (ushort)((ulong)puVar5 >> 0x10);
    uVar2 = *(undefined4 *)(param_1 + 0x42);
    uVar1 = *(ushort *)((int)uVar2 + 0x12);
    param_5 = 0x1010;
    uVar3 = uVar1;
    pass1_1010_a5ca((ushort)puVar5,uVar4,uVar1,uVar1,uVar4);
    if (uVar3 == 0xffff) {
      *(undefined2 *)(param_1 + 0x3c) = 0xf9;
    }
    else {
      if (uVar3 == 0x0) {
        *(undefined2 *)(param_1 + 0x3c) = 0x25;
        if ((uVar1 == 0x5b) || (uVar1 == 0x9)) {
          *(undefined2 *)(param_1 + 0x3c) = 0xfe;
        }
      }
      else {
        *(undefined2 *)(param_1 + 0x3c) = 0xfb;
      }
    }
  }
  draw_text_1040_94fc((astruct_37 *)CONCAT22(param_2,param_1),param_5);
  return;
}



ushort * __stdcall16far pass1_1040_c9cc(ushort *param_1,byte param_2)

{
  pass1_1040_c5ac(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_ca16(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_727 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1840));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_727 *)param_1;
  iVar1->field_0x94 = _PTR_LOOP_1050_5f0c;
  *(undefined4 *)&iVar1->field_0x98 = 0x0;
  iVar1->field_0x9c = 0x0;
  iVar1->field_0x9e = 0x0;
  *(undefined2 *)param_1 = 0xd07c;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3e,param_5,param_3,param_4);
  iVar1->field_0x98 = (int)puVar2;
  iVar1->field_0x9a = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_ca74(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xd07c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  PTR_LOOP_1050_5f10 = (undefined *)0x0;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_caa6(ushort param_1,ulong param_2,uchar *param_3,int param_4,ushort param_5)

{
  ushort *puVar1;
  int iVar2;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e((ulong)puVar1,iVar2,param_5);
  destroy_window_1040_b726((ULONG *)CONCAT22((int)param_2,param_1),(int)(param_2 >> 0x10),0x1010);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1040_cace(ulong param_1,HWND16 param_2,ushort param_3)

{
  undefined4 uVar1;
  bool bVar2;
  int iVar3;
  INT16 IVar4;
  ushort in_DX;
  ushort uVar5;
  undefined2 uVar6;
  bool bVar7;
  ushort uVar8;
  char local_208 [0x100];
  char local_108 [0x100];
  UINT16 UStack8;
  ushort uStack6;
  BOOL16 local_4;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  uVar5 = (ushort)param_1;
  uStack6 = GetDlgItemInt16(param_2,0x0,&local_4,param_3);
  UStack8 = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,&local_4,param_3);
  if (uStack6 == 0x0) {
    return;
  }
  pass1_1018_50ea(*(ulong *)(uVar5 + 0x98),uStack6,*(ulong *)(uVar5 + 0x94));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_208,param_3);
  uVar1 = *(undefined4 *)(uVar5 + 0x94);
  uVar8 = (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
  if (*(long *)((int)uVar1 + 0x36) == 0x0) {
    load_string_1010_84e0(0x1010,(ushort)_PTR_LOOP_1050_14cc,uVar8,0x3ff,local_108,param_3);
    iVar3 = MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x14),local_208,param_3);
  }
  else {
    load_string_1010_84e0(0x1010,(ushort)_PTR_LOOP_1050_14cc,uVar8,0x3ff,local_108,param_3);
    iVar3 = MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x14),local_208,param_3);
  }
  bVar2 = iVar3 == 0x6;
  bVar7 = false;
  if ((!bVar2) && (uVar1 = *(undefined4 *)(uVar5 + 0x94), *(int *)((int)uVar1 + 0x34) < 0x1)) {
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_108,param_3);
    IVar4 = MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x14),local_208,param_3);
    bVar7 = IVar4 == 0x6;
    bVar2 = false;
  }
  if (bVar2) {
    _PTR_LOOP_1050_5f16 = *(undefined4 *)(uVar5 + 0x94);
    iVar3 = 0x26;
  }
  else {
    if (!bVar7) {
      return;
    }
    _PTR_LOOP_1050_5a68 = *(undefined4 *)(uVar5 + 0x94);
    iVar3 = 0x27;
  }
  pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar5 + 0x8),iVar3,in_DX,uVar5,(ushort)&PTR_LOOP_1050_1038,param_3);
  return;
}



LRESULT __stdcall16far pass1_1040_cc66(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  LRESULT LVar2;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x98) + 0x10);
  (**ppcVar1)();
  LVar2 = send_dlg_msg_1040_cf1c(param_1,param_2);
  return LVar2;
}



void __stdcall16far
pass1_1040_cc8c(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  if (param_4._2_2_ == 0xeb) {
    send_dlg_msg_1040_cf1c(CONCAT22(param_2,param_1),param_7);
  }
  else {
    if (param_4._2_2_ == (int)s_vrpal_bmp_1050_183a + 0x7) {
      msg_box_op_1040_cce4(CONCAT22(param_2,param_1),0x0,param_5,param_7);
    }
    else {
      if (param_4._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x8) {
        pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
        return;
      }
      if ((int)param_4 == 0x1) {
        send_dlg_item_1040_ce76(CONCAT22(param_2,param_1),param_6,param_7);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_op_1040_cce4(ulong param_1,char *param_2,uchar *param_3,ushort param_4)

{
  ulong uStack522;
  char local_206 [0x102];
  char local_104 [0x102];
  ushort uVar2;
  ushort uVar3;
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



ushort __stdcall16far pass1_1040_cdac(ulong param_1,ushort param_2,ushort param_3,int param_4,HWND16 param_5)

{
  int *piVar1;
  int iVar2;
  bool bVar3;
  int iVar4;
  undefined2 uVar5;
  
  bVar3 = false;
  iVar4 = (int)param_1;
  uVar5 = (undefined2)(param_1 >> 0x10);
  if (param_4 == 0x0) {
    iVar2 = *(int *)(iVar4 + 0x9e);
    piVar1 = (int *)(iVar4 + 0x9c);
    if (*piVar1 == iVar2 || *piVar1 < iVar2) goto LAB_1040_cdef;
    piVar1 = (int *)(iVar4 + 0x9e);
    *piVar1 = *piVar1 + 0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1040_cdef;
    if (*(int *)(iVar4 + 0x9e) < 0x1) goto LAB_1040_cdef;
    piVar1 = (int *)(iVar4 + 0x9e);
    *piVar1 = *piVar1 + -0x1;
  }
  bVar3 = true;
LAB_1040_cdef:
  if (bVar3) {
    SetDlgItemInt16(param_5,0x0,*(UINT16 *)(iVar4 + 0x9e),0x18e);
  }
  return 0x0;
}



void __stdcall16far
send_dlg_item_msg_1040_ce12(ushort param_1,ushort param_2,ulong param_3,ushort param_4,WORD *param_5)

{
  long lVar1;
  CHAR local_10a [0x100];
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_a),param_3);
  while( true ) {
    lVar1 = pass1_1008_5b12(local_a,param_5);
    if (lVar1 == 0x0) break;
    wsprintf16((LPSTR)0x1008,local_10a,param_5);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_10a,(UINT16)param_5,0x0,CONCAT22(param_4,0x401));
  }
  return;
}



void __stdcall16far send_dlg_item_1040_ce76(ulong param_1,HWND16 param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  LRESULT LVar3;
  ulong uVar4;
  undefined local_106 [0x100];
  WPARAM16 WStack6;
  int iStack4;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  LVar3 = SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x18420409);
  WStack6 = (WPARAM16)LVar3;
  iStack4 = (int)WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_106,param_3,WStack6,0x1842040a);
    uVar4 = pass1_1018_5206(*(ulong *)(iVar1 + 0x98),CONCAT22(param_3,local_106),param_3);
    if (uVar4 != 0x0) {
      *(undefined2 *)(iVar1 + 0x9c) = *(undefined2 *)((int)uVar4 + 0x8);
      *(undefined2 *)(iVar1 + 0x9e) = 0x0;
      SetDlgItemInt16(0x1018,0x0,0x0,0x18e);
      SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,*(UINT16 *)(iVar1 + 0x9c),0x191);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT __stdcall16far send_dlg_msg_1040_cf1c(ulong param_1,ushort param_2)

{
  uchar *in_DX;
  ushort uVar1;
  ushort uVar2;
  int unaff_DI;
  ushort uVar3;
  uchar in_AF;
  LRESULT LVar4;
  BOOL16 enable;
  char local_50c [0x402];
  undefined4 uStack266;
  undefined local_106 [0x100];
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  uVar1 = (ushort)((ulong)puStack6 >> 0x10);
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar2 = (ushort)param_1;
  pass1_1010_c3c2((ushort)puStack6,uVar1,CONCAT22(param_2,local_106),*(ulong *)(uVar2 + 0x94),(uchar *)uVar1,in_AF,
                  param_2);
  SendDlgItemMessage16(0x1010,(INT16)local_106,param_2,0x0,0x1846000c);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1842000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18420405);
  uVar1 = (int)s_vrpal_bmp_1050_183a + 0x8;
  uStack266 = pass1_1018_526a(*(ulong *)(uVar2 + 0x98),*(ulong *)(uVar2 + 0x94),param_2);
  send_dlg_item_msg_1040_ce12(uVar2,uVar3,uStack266,uVar1,param_2);
  LVar4 = SendDlgItemMessage16(0x1018,0x0,0x0,0x0,0x1842040c);
  if (((int)((ulong)LVar4 >> 0x10) != 0x0 && -0x1 < LVar4) || ((-0x1 < LVar4 && ((int)LVar4 != 0x0)))) {
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
    enable = 0x1;
  }
  else {
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_50c,param_2);
    SendDlgItemMessage16(0x1010,(INT16)local_50c,param_2,0x0,0x18420401);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
    enable = 0x0;
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,enable);
  LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1842000b);
  return LVar4;
}







// WARNING: Globals starting with '_' overlap smaller symbols at the same address

