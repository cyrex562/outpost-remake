
void __stdcall16far pass1_1020_1b68(ulong param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x92);
  uVar2 = *(uint *)(iVar4 + 0x94);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar4 + 0x92) = 0x0;
  pass1_1010_4f48(*(ulong *)(iVar4 + 0x8e),param_2);
  *(undefined4 *)(iVar4 + 0x8e) = 0x0;
  return;
}



ushort __stdcall16far pass1_1020_1bb6(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void __stdcall16far enable_window_1020_1bd4(int param_1,ushort param_2,ushort param_3,ulong param_4,HWND16 param_5)

{
  code **ppcVar1;
  bool bVar2;
  uint in_AX;
  int iVar3;
  uchar *in_DX;
  uchar *puVar4;
  undefined2 uVar5;
  ushort unaff_SS;
  undefined4 *puStack12;
  
  bVar2 = false;
  pass1_1020_1d8e(CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3));
  if (in_AX != 0x0) {
    if ((int)in_AX < 0x2) {
      bVar2 = true;
    }
    else {
      GetDlgItem16(param_5,0x1);
      pass1_1010_4e8c(*(ulong *)(param_1 + 0x8e),unaff_SS);
      in_AX = EnableWindow16(0x1010,0x1);
      pass1_1010_4df0(*(ulong *)(param_1 + 0x8e),(uint)in_DX,unaff_SS);
      if ((in_AX == 0x0) && (bVar2 = true, *(int *)(param_1 + 0x96) == 0x0)) {
        in_AX = EnableWindow16(0x1010,0x0);
      }
    }
  }
  if (bVar2) {
    uVar5 = 0x1000;
    mem_op_1000_179c(0xb4,in_DX,0x1000);
    puVar4 = (uchar *)((uint)in_DX | in_AX);
    if (puVar4 == (uchar *)0x0) {
      iVar3 = 0x0;
      puVar4 = (uchar *)0x0;
    }
    else {
      uVar5 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520((astruct_57 *)CONCAT22(in_DX,in_AX),*(ushort *)(param_1 + 0x6),0x30,0x2,0x57b,0x62a,
                               puVar4,unaff_SS);
    }
    puStack12 = (undefined4 *)CONCAT22(puVar4,iVar3);
    ppcVar1 = (code **)((int)*puStack12 + 0x74);
    (**ppcVar1)(uVar5,iVar3,puVar4);
  }
  return;
}



BOOL16 __stdcall16far post_win_msg_1020_1ca4(ulong param_1)

{
  code **ppcVar1;
  uint in_AX;
  int iVar2;
  uchar *in_DX;
  uchar *puVar3;
  undefined2 uVar4;
  ushort unaff_SS;
  undefined4 *puStack10;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x96) == 0x0) {
    pass1_1010_4df0(*(ulong *)((int)param_1 + 0x8e),(uint)in_DX,unaff_SS);
    if (in_AX == 0x0) {
      mem_op_1000_179c(0xb4,in_DX,0x1000);
      puVar3 = (uchar *)((uint)in_DX | in_AX);
      if (puVar3 == (uchar *)0x0) {
        iVar2 = 0x0;
        puVar3 = (uchar *)0x0;
      }
      else {
        iVar2 = string_1040_8520((astruct_57 *)CONCAT22(in_DX,in_AX),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x62a,
                                 puVar3,unaff_SS);
      }
      puStack10 = (undefined4 *)CONCAT22(puVar3,iVar2);
      ppcVar1 = (code **)((int)*puStack10 + 0x74);
      (**ppcVar1)();
      return 0x0;
    }
    PostMessage16(0x1010,0x0,0x0,0x11100de);
  }
  return 0x1;
}



void __stdcall16far
set_win_tet_1020_1d2a
          (ushort param_1,ushort param_2,SEGPTR in_win_text_3,ushort param_4,INT16 in_dlg_id_5,HWND16 in_hwnd_6)

{
  GetDlgItem16(in_hwnd_6,in_dlg_id_5);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,in_win_text_3);
  return;
}



void __stdcall16far destroy_window_1020_1d4a(ULONG param_1,int param_2,HWND16 param_3)

{
  BOOL16 BVar1;
  HWND16 hwnd;
  
  if (param_2 != 0x0) {
    BVar1 = post_win_msg_1020_1ca4(param_1);
    if (BVar1 != 0x0) {
      hwnd = param_3;
      if (*(int *)((int)param_1 + 0x96) != 0x0) {
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        PostMessage16(param_3,0x0,0x0,0x11100ee);
      }
      DestroyWindow16(hwnd);
    }
  }
  return;
}



void __stdcall16far pass1_1020_1d8e(ulong param_1,ulong param_2)

{
  pt_in_rect_1010_4e08(*(ulong *)((int)param_1 + 0x8e),(ushort)param_2,(ushort)(param_2 >> 0x10),0x1010);
  return;
}



ushort __stdcall16far pass1_1020_1da8(ulong param_1,int param_2,uint param_3,ushort param_4)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar2 + 0x8e);
  if (*(int *)((int)uVar1 + 0x30) == 0x1) {
    return 0x1;
  }
  uVar1 = *(undefined4 *)(iVar2 + 0x8e);
  if ((*(int *)((int)uVar1 + 0x30) < 0x3) && (pass1_1010_4df0(*(ulong *)(iVar2 + 0x8e),param_3,param_4), param_2 == 0x0)
     ) {
    return 0x1;
  }
  return 0x0;
}



BOOL16 __stdcall16far destroy_win_1020_1dea(HWND16 param_1)

{
  BOOL16 BVar1;
  WORD WVar2;
  
  BVar1 = IsWindow16(param_1);
  if (BVar1 != 0x0) {
    WVar2 = GetWindowWord16((HWND16)s_tile2_bmp_1050_1538,-0xc);
    if (WVar2 == 0x175) {
      DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
      return 0x0;
    }
  }
  return 0x1;
}



ushort __stdcall16far destroy_win_1020_1e1e(HWND16 param_1)

{
  BOOL16 BVar1;
  WORD WVar2;
  
  BVar1 = IsWindow16(param_1);
  if (BVar1 != 0x0) {
    WVar2 = GetWindowWord16((HWND16)s_tile2_bmp_1050_1538,-0xc);
    if ((WVar2 != 0x1) && (WVar2 != 0x175)) {
      DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
    }
  }
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1020_1e54(astruct_18 *param_1,byte param_2)

{
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_1eea(ushort *param_1,ulong param_2,ushort param_3,uchar *param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  undefined2 uVar2;
  astruct_663 *iVar3;
  undefined2 uVar3;
  ushort *puVar4;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_663 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = param_3;
  *param_1 = 0x3ab0;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x6 = (undefined4 *)0x0;
  iVar3->field_0xa = param_2;
  *param_1 = 0x2518;
  iVar3->field_0x2 = 0x1020;
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_6,param_4,param_5);
  uVar2 = (undefined2)((ulong)puVar4 >> 0x10);
  *(int *)&iVar3->field_0x6 = (int)puVar4;
  *(undefined2 *)((int)&iVar3->field_0x6 + 0x2) = uVar2;
  ppcVar1 = (code **)((int)*iVar3->field_0x6 + 0x4);
  (**ppcVar1)(0x1010,*(undefined2 *)&iVar3->field_0x6,uVar2,0x0,param_1);
  return;
}



void __stdcall16far pass1_1020_1f74(ushort *param_1,ushort param_2)

{
  astruct_582 *iVar1;
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_582 *)param_1;
  *param_1 = 0x2518;
  iVar1->field_0x2 = 0x1020;
  pass1_1010_1ea6(iVar1->field_0x6,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,param_2);
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far invalidate_rect_1020_1fb2(ulong param_1,int param_2,HWND16 param_3)

{
  ushort local_16;
  ushort uStack20;
  int iStack18;
  ushort uStack16;
  RECT16 local_e;
  int iStack10;
  ushort uStack6;
  ushort uStack4;
  
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  GetWindowRect16(param_3,&local_e);
  local_16 = 0x0;
  uStack6 = 0x46;
  uStack20 = 0x46;
  iStack18 = iStack10 - local_e.x;
  uStack4 = 0x5f;
  uStack16 = 0x5f;
  InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,(BOOL16)&local_16);
  return;
}



// WARNING: Inlined function: struct_1010_4d5c

void __stdcall16far unk_draw_op_1020_2020(ulong param_1,HWND16 param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  ushort uVar4;
  HDC16 *pHVar5;
  int iVar6;
  HPEN16 HVar7;
  HGDIOBJ16 HVar8;
  HBRUSH16 HVar9;
  uchar *puVar10;
  uint extraout_DX;
  uint uVar11;
  int iVar12;
  int iVar13;
  undefined *puVar14;
  undefined2 uVar15;
  undefined2 uVar16;
  ulong uVar17;
  int *piVar18;
  int iVar19;
  undefined uVar20;
  undefined uVar21;
  undefined local_38 [0x6];
  undefined2 local_32;
  undefined2 uStack48;
  undefined4 uStack46;
  undefined2 uStack42;
  undefined4 *puStack40;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  puVar14 = &stack0xfffe;
  uVar15 = (undefined2)(param_1 >> 0x10);
  iVar12 = (int)param_1;
  uVar16 = *(undefined2 *)(iVar12 + 0x4);
  local_24 = BeginPaint16(param_2,&local_22);
  puStack40 = (undefined4 *)pass1_1010_4c2c(*(ulong *)(iVar12 + 0x6));
  pHVar5 = &local_24;
  ppcVar1 = (code **)((int)*puStack40 + 0x8);
  (**ppcVar1)(0x1010,(int)puStack40,(int)((ulong)puStack40 >> 0x10),pHVar5,param_3,uVar16);
  *(HDC16 **)(iVar12 + 0x10) = pHVar5;
  uVar2 = *(undefined4 *)(iVar12 + 0x6);
  uStack42 = *(undefined2 *)((int)uVar2 + 0x30);
  uVar2 = *(undefined4 *)(iVar12 + 0x6);
  uStack46 = *(undefined4 *)*(undefined4 *)((int)uVar2 + 0x12);
  uStack48 = 0x14;
  local_32 = 0x0;
  pass1_1008_3e38((ushort *)CONCAT22(param_3,local_38));
  while (*(int *)(puVar14 + -0x38) < *(int *)(puVar14 + -0x28)) {
    iVar12 = *(int *)(puVar14 + -0x38) * 0x4;
    uVar2 = *(undefined4 *)(puVar14 + -0x2c);
    uVar17 = pass1_1008_4772(*(astruct_76 **)(iVar12 + (int)uVar2));
    puVar10 = (uchar *)(uVar17 >> 0x10);
    *(int *)(puVar14 + -0x44) = (int)uVar17;
    *(uchar **)(puVar14 + -0x42) = puVar10;
    uVar2 = *(undefined4 *)(puVar14 + 0x6);
    pass1_1020_2286((ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),
                    (int *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,puVar14 + -0x30)),
                    *(int *)((int)uVar17 + 0x8));
    uVar2 = *(undefined4 *)(puVar14 + -0x30);
    pass1_1008_3e76((ushort *)CONCAT22(param_3,puVar14 + -0x36),0x0,(ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10));
    uVar2 = *(undefined4 *)(puVar14 + -0x2c);
    pass1_1008_4480(*(ulong *)(puVar14 + -0x26),(ushort *)CONCAT22(param_3,puVar14 + -0x36),
                    *(astruct_76 **)((int)uVar2 + iVar12),param_3);
    iVar12 = *(int *)(puVar14 + -0x38);
    uVar2 = *(undefined4 *)(puVar14 + -0x30);
    uVar15 = (undefined2)uVar2;
    uVar20 = (undefined)((ulong)uVar2 >> 0x10);
    uVar21 = (undefined)((ulong)uVar2 >> 0x18);
    uVar2 = *(undefined4 *)(puVar14 + -0x44);
    uVar16 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar13 = (int)uVar2;
    iVar6 = *(int *)(iVar13 + 0x4) + *(int *)(puVar14 + -0x2e);
    iVar13 = *(int *)(iVar13 + 0x8) + *(int *)(puVar14 + -0x30);
    uVar2 = *(undefined4 *)(puVar14 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x6);
    iVar19 = (int)uVar2;
    uVar16 = (undefined2)((ulong)uVar2 >> 0x10);
    if (*(long *)(iVar19 + 0x1a) == 0x0) {
      uVar4 = *(int *)(iVar19 + 0x30) << 0x3;
      mem_op_1000_179c(uVar4,puVar10,0x1000);
      *(ushort *)(iVar19 + 0x1a) = uVar4;
      *(uchar **)(iVar19 + 0x1c) = puVar10;
    }
    uVar2 = *(undefined4 *)(iVar19 + 0x1a);
    iVar12 = iVar12 * 0x8;
    *(undefined2 *)((int)uVar2 + iVar12) = CONCAT11(uVar21,uVar20);
    uVar2 = *(undefined4 *)(iVar19 + 0x1a);
    *(undefined2 *)((int)uVar2 + iVar12 + 0x2) = uVar15;
    uVar2 = *(undefined4 *)(iVar19 + 0x1a);
    *(int *)((int)uVar2 + iVar12 + 0x4) = iVar6;
    uVar2 = *(undefined4 *)(iVar19 + 0x1a);
    *(int *)((int)uVar2 + iVar12 + 0x6) = iVar13;
    uVar2 = *(undefined4 *)(puVar14 + -0x44);
    piVar18 = (int *)(puVar14 + -0x2e);
    *piVar18 = *piVar18 + (-(uint)(*(int *)(puVar14 + -0x38) == 0x0) & 0x5) + 0x14 + *(int *)((int)uVar2 + 0x4);
    piVar18 = (int *)(puVar14 + -0x38);
    *piVar18 = *piVar18 + 0x1;
  }
  puVar3 = *(undefined4 **)(puVar14 + -0x26);
  ppcVar1 = (code **)((int)*puVar3 + 0x4);
  (**ppcVar1)(0x1008,(int)puVar3,(int)((ulong)puVar3 >> 0x10),0x0,0x0,(char)puVar14 + -0x22,param_3);
  uVar11 = extraout_DX;
  HVar7 = CreatePen16(0x1008,0x25,0x100);
  *(HPEN16 *)(puVar14 + -0x3a) = HVar7;
  HVar8 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar7);
  *(HGDIOBJ16 *)(puVar14 + -0x3c) = HVar8;
  HVar9 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  *(HBRUSH16 *)(puVar14 + -0x3e) = HVar9;
  HVar8 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar9);
  *(HGDIOBJ16 *)(puVar14 + -0x40) = HVar8;
  draw_line_1020_229c(*(ulong *)(puVar14 + 0x6),(int)s_tile2_bmp_1050_1538);
  uVar2 = *(undefined4 *)(puVar14 + 0x6);
  pass1_1010_4df0(*(ulong *)((int)uVar2 + 0x6),uVar11,param_3);
  if (HVar8 == 0x0) {
    SelectObject16(0x1010,*(HGDIOBJ16 *)(puVar14 + -0x3c));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x40));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    HVar9 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    *(HBRUSH16 *)(puVar14 + -0x3e) = HVar9;
    HVar7 = CreatePen16((INT16)s_tile2_bmp_1050_1538,0xff,0x0);
    *(HPEN16 *)(puVar14 + -0x3a) = HVar7;
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x3e));
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x3a));
  }
  uVar2 = *(undefined4 *)(puVar14 + 0x6);
  piVar18 = (int *)pass1_1010_4dc8(*(ulong *)((int)uVar2 + 0x6));
  uVar15 = (undefined2)((ulong)piVar18 >> 0x10);
  uVar16 = SUB42(piVar18,0x0);
  pass1_1020_239c(*(ulong *)(puVar14 + 0x6),piVar18,param_3);
  uVar2 = *(undefined4 *)(puVar14 + 0x6);
  uVar2 = *(undefined4 *)((int)uVar2 + 0x6);
  if (*(int *)((int)uVar2 + 0x2c) != 0x0) {
    pass1_1020_2488(*(ulong *)(puVar14 + 0x6),uVar16,uVar15);
  }
  uVar2 = *(undefined4 *)(puVar14 + 0x6);
  SelectPalette16(0x1010,0x0,*(BOOL16 *)((int)uVar2 + 0x10));
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x3c));
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x40));
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,(PAINTSTRUCT16 *)(puVar14 + -0x20));
  return;
}



void __stdcall16far pass1_1020_2286(ushort param_1,ushort param_2,int *param_3,int param_4)

{
  *param_3 = 0x64 - param_4 >> 0x1;
  return;
}



void __stdcall16far draw_line_1020_229c(ulong param_1,HDC16 param_2)

{
  int iVar1;
  INT16 *pIVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  int *piVar6;
  undefined2 uVar7;
  int iStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  uVar3 = *(undefined4 *)((int)param_1 + 0x6);
  iVar1 = *(int *)((int)uVar3 + 0x30);
  uVar3 = *(undefined4 *)((int)param_1 + 0x6);
  pIVar2 = *(INT16 **)((int)uVar3 + 0x1a);
  MoveTo16(param_2,0x5,*pIVar2);
  uVar7 = (undefined2)((ulong)pIVar2 >> 0x10);
  iVar5 = (int)pIVar2;
  LineTo16((HDC16)s_tile2_bmp_1050_1538,0x5,*(INT16 *)(iVar5 + iVar1 * 0x8 + -0x4));
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1) {
    piVar6 = (int *)(iStack10 * 0x8 + iVar5);
    iVar4 = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,0x5,iVar4);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,0xa,iVar4);
  }
  MoveTo16((HDC16)s_tile2_bmp_1050_1538,0x5f,*pIVar2);
  LineTo16((HDC16)s_tile2_bmp_1050_1538,0x5f,*(INT16 *)(iVar5 + iVar1 * 0x8 + -0x4));
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1) {
    piVar6 = (int *)(iStack10 * 0x8 + iVar5);
    iVar4 = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,0x5f,iVar4);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,0x5a,iVar4);
  }
  return;
}



void __stdcall16far pass1_1020_239c(ulong param_1,int *param_2,ushort param_3)

{
  ushort *puVar1;
  ulong uVar2;
  ushort uVar3;
  undefined local_a [0x6];
  ushort uStack4;
  
  if (param_2 != (int *)0x0) {
    uStack4 = (*(int *)((int)param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
    puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_3,local_a),0x0,0x57,uStack4);
    uVar3 = (ushort)(param_1 >> 0x10);
    uVar2 = pass1_1020_23f2((ushort)param_1,uVar3,(ushort *)CONCAT22(param_3,local_a),(uchar *)((ulong)puVar1 >> 0x10),
                            param_3);
    draw_polygon_1020_2474((ushort)param_1,uVar3,CONCAT22((int)uVar2,0x3),0x1008);
  }
  return;
}



ulong __stdcall16far pass1_1020_23f2(ushort param_1,ushort param_2,ushort *param_3,uchar *param_4,undefined2 param_5)

{
  int *piVar1;
  int iStack18;
  int local_6;
  int local_4;
  
  piVar1 = &local_6;
  pass1_1008_3e94(param_3,(ushort *)CONCAT22(param_5,piVar1),(ushort *)CONCAT22(param_5,&local_4));
  mem_op_1000_179c(0xc,param_4,0x1000);
  for (iStack18 = 0x0; iStack18 < 0x3; iStack18 = iStack18 + 0x1) {
    piVar1[iStack18 * 0x2] = *(int *)(iStack18 * 0x4 + 0x4270) + local_4;
    piVar1[iStack18 * 0x2 + 0x1] = *(int *)(iStack18 * 0x4 + 0x4272) + local_6;
  }
  return CONCAT22(param_4,piVar1);
}



void __stdcall16far draw_polygon_1020_2474(ushort param_1,ushort param_2,ulong param_3,HDC16 param_4)

{
  Polygon16(param_4,(POINT16 *)param_3,(INT16)(param_3 >> 0x10));
  return;
}



void __stdcall16far pass1_1020_2488(ulong param_1,uint param_2,uint param_3)

{
  undefined4 uVar1;
  ushort in_dlg_id_5;
  uint uVar2;
  int iVar3;
  undefined2 uVar4;
  int iStack12;
  SEGPTR SStack10;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  find_n_load_rsrc_1010_4e9e(*(ulong *)(iVar3 + 0x6),0x1010);
  if ((param_3 | param_2) != 0x0) {
    SStack10 = param_2;
    for (iStack12 = 0x0; iStack12 < 0x9; iStack12 = iStack12 + 0x1) {
      uVar1 = *(undefined4 *)(iVar3 + 0x6);
      in_dlg_id_5 = pass1_1010_4f20((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),iStack12);
      uVar1 = *(undefined4 *)(iVar3 + 0xa);
      set_win_tet_1020_1d2a((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),SStack10,param_3,in_dlg_id_5,0x1010);
      uVar2 = str_op_1000_3da4((char *)CONCAT22(param_3,SStack10));
      SStack10 = SStack10 + uVar2 + 0x1;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_24f2(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1020_1f74(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1020_2524(astruct_20 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uchar *extraout_DX;
  undefined2 uVar1;
  astruct_20 *iVar2;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  unk_draw_op_1020_7f7a(param_1,0x7,CONCAT22(param_3,param_2));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_20 *)param_1;
  *(undefined4 *)&iVar2[0x1].field_0xc = 0x0;
  iVar2[0x1].field_0x10 = 0x0;
  param_1->field_0x0 = 0x270c;
  iVar2->field_0x2 = 0x1020;
  ((astruct_20 *)(iVar2 + 0x1))->field_0x0 = 0x27a8;
  iVar2[0x1].field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2a,param_4,extraout_DX,unaff_DI);
  uVar1 = (undefined2)((ulong)puVar3 >> 0x10);
  *(int *)&iVar2[0x1].field_0x10 = (int)puVar3;
  *(undefined2 *)((int)&iVar2[0x1].field_0x10 + 0x2) = uVar1;
  *(undefined2 *)&iVar2[0x1].field_0x4 = *(undefined2 *)&iVar2[0x1].field_0x10;
  *(undefined2 *)((int)&iVar2[0x1].field_0x4 + 0x2) = uVar1;
  return;
}



void __stdcall16far pass1_1020_2594(ushort *param_1)

{
  astruct_583 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_583 *)param_1;
  *param_1 = 0x270c;
  iVar1->field_0x2 = 0x1020;
  iVar1->field_0xe2 = 0x27a8;
  iVar1->field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}



void __stdcall16far pass1_1020_25c0(ulong param_1,ushort param_2,ushort param_3)

{
  int *piVar1;
  code **ppcVar2;
  uint uVar3;
  uint uVar4;
  astruct_277 *iVar3;
  undefined2 uVar5;
  astruct_57 *paVar6;
  undefined4 *puStack6;
  
  paVar6 = (astruct_57 *)CONCAT22(param_3,param_2);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_277 *)param_1;
  if (iVar3->field_0xee != (undefined4 *)0x0) {
    ppcVar2 = (code **)((int)*iVar3->field_0xee + 0x8);
    paVar6 = (astruct_57 *)(**ppcVar2)();
  }
  if (iVar3->field_0xea == 0x0) {
    iVar3->field_0xea = 0x1;
    mem_op_1000_179c(0x98,(uchar *)((ulong)paVar6 >> 0x10),0x1000);
    uVar3 = (uint)paVar6;
    uVar4 = (uint)((ulong)paVar6 >> 0x10) | uVar3;
    if (paVar6 == (astruct_57 *)0x0) {
      puStack6 = (undefined4 *)0x0;
    }
    else {
      piVar1 = &iVar3->field_0xcc;
      *piVar1 = *piVar1 + 0x1;
      struct_1020_1738(paVar6,iVar3->field_0xcc,param_1);
      puStack6 = (undefined4 *)CONCAT22(uVar4,uVar3);
    }
    ppcVar2 = (code **)((int)*puStack6 + 0x8);
    (**ppcVar2)(0x1000,(int)puStack6,(int)((ulong)puStack6 >> 0x10));
  }
  return;
}



void __stdcall16far window_op_1020_2642(astruct *param_1)

{
  astruct_664 *in_AX;
  uchar *in_DX;
  uint uVar1;
  int iVar2;
  int unaff_DI;
  undefined2 uVar3;
  ushort unaff_SS;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),*(ushort *)(iVar2 + 0x8),0x1018);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = (uint)in_DX | (uint)in_AX;
  if (uVar1 != 0x0) {
    pass1_1020_27b0(in_AX,(ushort)in_DX,*(ushort *)(iVar2 + 0x8),unaff_DI,unaff_SS);
    *(astruct_664 **)(iVar2 + 0xee) = in_AX;
    *(uint *)(iVar2 + 0xf0) = uVar1;
    return;
  }
  *(undefined4 *)(iVar2 + 0xee) = 0x0;
  return;
}



void __stdcall16far pass1_1020_26a6(ULONG param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  puVar1 = (undefined4 *)*(uint *)((int)param_1 + 0xee);
  uVar2 = *(uint *)((int)param_1 + 0xf0);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



astruct_18 * __stdcall16far pass1_1020_26d8(astruct_18 *param_1,byte param_2)

{
  param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  pass1_1020_2594((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_27b0(astruct_664 *param_1,ushort param_2,ushort param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  uchar *extraout_DX;
  undefined2 uVar4;
  ushort *puVar5;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0x14 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x288e;
  param_1->field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2a,param_5,extraout_DX,param_4);
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



void __stdcall16far pass1_1020_2838(ushort *param_1,ushort param_2)

{
  astruct_584 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_584 *)param_1;
  *param_1 = 0x288e;
  iVar1->field_0x2 = 0x1020;
  if (iVar1->field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



astruct_18 * __stdcall16far pass1_1020_2868(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1020_2838(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1020_289a(ushort *param_1,UINT16 param_2,ulong param_3,UINT16 param_4)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1020_790e(param_1,(ulong)s_SCPOPMENU_1050_427c,param_2,param_3,param_4);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0xf2) = 0x0;
  *(undefined4 *)(iVar1 + 0xf6) = 0x0;
  *(undefined2 *)(iVar1 + 0xfa) = 0x0;
  *(undefined2 *)(iVar1 + 0xfc) = 0x0;
  *param_1 = 0x2e4a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1020;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x5b)),s_VrMode_1050_4286);
  *(undefined4 *)(iVar1 + 0xac) = 0x44c00000;
  return;
}



void __stdcall16far pass1_1020_28fc(astruct_3 *param_1,ushort param_2)

{
  param_1->address_offset_field_0x0 = 0x2e4a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  cleanup_menu_ui_op_1020_795c(param_1,param_2);
  return;
}



void __stdcall16far post_win_msg_1020_291a(HWND16 param_1)

{
  PostMessage16(param_1,0x0,0x0,0x100000);
  return;
}



void __stdcall16far pass1_1020_2936(void)

{
  pass1_1020_79ae();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_294a(ulong param_1,ulong param_2,ushort param_3,uchar *param_4,int param_5,ushort param_6)

{
  undefined2 uVar1;
  astruct_665 *iVar3;
  undefined2 uVar2;
  ushort *puVar3;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_665 *)param_1;
  iVar3->field_0xfc = param_3;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,param_3,param_6,param_4,param_5);
  uVar1 = (undefined2)((ulong)puVar3 >> 0x10);
  iVar3->field_0xf2 = (int)puVar3;
  iVar3->field_0xf4 = uVar1;
  iVar3->field_0xe0 = iVar3->field_0xf2;
  iVar3->field_0xe2 = uVar1;
  pass1_1018_0902(*(ulong **)&iVar3->field_0xf2,param_2);
  return;
}



void __stdcall16far realize_palette_1020_2992(ULONG param_1,int param_2)

{
  code **ppcVar1;
  undefined4 *puVar2;
  
  if (param_2 != 0x0) {
    puVar2 = (undefined4 *)pass1_1018_0a50(*(ulong *)((int)param_1 + 0xf2));
    ppcVar1 = (code **)((int)*puVar2 + 0x18);
    (**ppcVar1)(0x1018,(int)puVar2,(int)((ulong)puVar2 >> 0x10));
    UnrealizeObject16(0x1018);
    GetDC16((HWND16)s_tile2_bmp_1050_1538);
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far
send_msg_1020_29d8(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6,ushort param_7)

{
  uchar *puVar1;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar2;
  int iVar3;
  
  iVar3 = (int)(param_4 >> 0x10);
  post_win_msg_1020_79fc((astruct_69 *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,iVar3,param_7);
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,unaff_SS,(uchar *)param_6,unaff_DI);
  puVar1 = (uchar *)((ulong)puVar2 >> 0x10);
  if (iVar3 == 0x0) {
    pass1_1018_270e((ulong)puVar2,0x1,*(uint *)(param_1 + 0xfc),puVar1,unaff_DI,unaff_SS);
  }
  else {
    pass1_1018_270e((ulong)puVar2,0x0,*(uint *)(param_1 + 0xfc),puVar1,unaff_DI,unaff_SS);
    SendMessage16(0x1018,0x0,0x0,0x1110069);
  }
  return CONCAT22(param_6,param_5);
}



void __stdcall16far pass1_1020_2a46(ushort param_1,ushort param_2,int param_3)

{
  pass1_1018_0ae8(*(ulong *)(param_1 + 0xf2),0x1);
  pass1_1008_68c6(param_1,param_2,param_3,0x1008);
  return;
}



void __stdcall16far pass1_1020_2a6a(ulong param_1,ushort param_2)

{
  get_win_ui_info_op_1020_7a50(param_1,param_2);
  pass1_1018_0ae8(*(ulong *)((int)param_1 + 0xf2),0x0);
  destroy_icon_1020_2c88(param_1,0x1018);
  return;
}



void __stdcall16far pass1_1020_2a94(ulong param_1,ulong param_2,ushort param_3)

{
  pass1_1018_1662(*(ulong *)((int)param_1 + 0xf2),(int)param_2,(int)(param_2 >> 0x10),param_3);
  return;
}



void __stdcall16far bring_window_to_top_1020_2aae(ulong param_1,ulong param_2)

{
  u16 unaff_SS;
  
  pass1_1008_3e0e(param_1);
  BringWindowToTop16(0x1008);
  pass1_1018_169e(*(ulong *)((int)param_1 + 0xf2),param_2,unaff_SS);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far invalidate_rect_1020_2ae4(ulong *param_1,uint param_2,HWND16 param_3,ushort param_4)

{
  code **ppcVar1;
  char cVar2;
  int iVar3;
  uchar *in_DX;
  ushort uVar4;
  ushort uVar5;
  int unaff_DI;
  ushort *puVar6;
  ulong uVar7;
  astruct_43 *paVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  
  if (param_2 != 0x129) {
    uVar5 = (ushort)param_1;
    uVar9 = (undefined2)((ulong)param_1 >> 0x10);
    if (0x129 < param_2) {
      if (param_2 == 0x12a) {
        uVar9 = 0xf012;
      }
      else {
        if (param_2 == 0x12b) {
          return;
        }
        if (param_2 == 0x12c) {
          uVar9 = 0xf020;
        }
        else {
          if (param_2 == 0x12d) {
            return;
          }
          if (param_2 != 0x12e) {
            return;
          }
          uVar9 = 0xf060;
        }
      }
      PostMessage16(param_3,0x0,0x0,CONCAT22(0x112,uVar9));
      return;
    }
    if (param_2 == 0xfb) {
      puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_4,in_DX,unaff_DI);
      pass1_1010_375e((ulong)puVar6);
      ppcVar1 = (code **)((int)*param_1 + 0x14);
      (**ppcVar1)();
      uVar7 = pass1_1010_375e((ulong)puVar6);
      uVar4 = (ushort)(uVar7 >> 0x10);
      pass1_1018_181c(*(ulong *)(uVar5 + 0xf2),(char *)(uVar7 & 0xffff | (ulong)uVar4 << 0x10),(uchar)(uVar7 & 0xffff),
                      uVar4);
      return;
    }
    if (param_2 < 0xfc) {
      cVar2 = (char)param_2;
      if (cVar2 == 'o') {
        paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_4);
        WinHelp16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0xa),0x0,CONCAT22((int)paVar8,0x1));
        return;
      }
      if (cVar2 == 'r') {
        iVar3 = uVar5 + 0xa;
        uVar10 = uVar9;
        puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_4,in_DX,unaff_DI);
        uVar4 = (ushort)((ulong)puVar6 >> 0x10);
        pass1_1010_3770((ulong)puVar6,(char *)CONCAT22(uVar10,iVar3),uVar4);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar5 + 0x8),0x3,uVar4,uVar5,(ushort)&PTR_LOOP_1050_1038,param_4
                       );
        return;
      }
      if (cVar2 == 'u') {
        pass1_1018_0a76(*(ulong *)(uVar5 + 0xf2),param_4);
        InvalidateRect16(0x1018,(RECT16 *)0x0,0x0);
        return;
      }
    }
  }
  return;
}
