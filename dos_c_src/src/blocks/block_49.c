
void __stdcall16far send_win_msg_1020_08fe(astruct_63 *param_1,HWND16 param_2)

{
  BOOL16 BVar1;
  astruct_63 *iVar2;
  astruct_63 *uVar2;
  
  uVar2 = (astruct_63 *)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_63 *)param_1;
  *(undefined2 *)param_1 = 0xb0e;
  iVar2->field_0x2 = 0x1020;
  if (iVar2->field_0xe8 != 0x0) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 != 0x0) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110001);
    }
    iVar2->field_0xe8 = 0x0;
  }
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0xd2));
  *(undefined2 *)param_1 = 0x380a;
  iVar2->field_0x2 = 0x1008;
  *(undefined2 *)param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  return;
}



void __stdcall16far send_msg_1020_097e(ulong param_1,HWND16 param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((*(uint *)(iVar1 + 0xea) | *(uint *)(iVar1 + 0xe8)) != 0x0) {
    SendMessage16(param_2,0x0,0x0,0x1110001);
    *(undefined4 *)(iVar1 + 0xe8) = 0x0;
  }
  return;
}



void __stdcall16far win_1020_09ba(astruct *param_1,uint param_2,uchar *param_3,ushort param_4)

{
  uchar *puVar1;
  astruct_275 *iVar1;
  undefined2 uVar2;
  
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0xe,param_3,0x1000);
  puVar1 = (uchar *)((uint)param_3 | param_2);
  iVar1 = (astruct_275 *)param_1;
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  if (puVar1 != (uchar *)0x0) {
    struct_1020_0baa((ushort *)CONCAT22(param_3,param_2),iVar1->field_0x8,puVar1,param_4);
    iVar1->field_0xe2 = param_2;
    iVar1->field_0xe4 = (int)puVar1;
    return;
  }
  *(undefined4 *)&iVar1->field_0xe2 = 0x0;
  return;
}



void __stdcall16far pass1_1020_0a0c(ULONG param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  destroy_win_1008_628e(param_1,0x1008);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe2);
  uVar2 = *(uint *)(iVar4 + 0xe4);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar4 + 0xe2) = 0x0;
  *(undefined2 *)(iVar4 + 0xe6) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_0a52(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ushort uVar1;
  undefined2 uVar2;
  undefined4 uVar3;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = (ushort)param_1;
  unk_draw_op_1020_0c3e(*(undefined4 *)(uVar1 + 0xe2),param_3);
  if (*(int *)(uVar1 + 0xe6) == 0x0) {
    *(undefined2 *)(uVar1 + 0xe6) = 0x1;
    *(undefined2 *)((int)_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
    uVar3 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar1 + 0x8),0x6,param_2,uVar1,(ushort)&PTR_LOOP_1050_1038,
                            param_4);
    *(undefined2 *)(uVar1 + 0xe8) = (int)uVar3;
    *(undefined2 *)(uVar1 + 0xea) = (int)((ulong)uVar3 >> 0x10);
  }
  return;
}



void __stdcall16far pass1_1020_0aa6(ulong param_1,ushort param_2)

{
  win_ui_palette_op_1020_0cd2(*(undefined4 *)((int)param_1 + 0xe2),param_2);
  return;
}



void __stdcall16far pass1_1020_0abc(ulong param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0xe6) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xe8) + 0x10);
    (**ppcVar1)();
  }
  return;
}



astruct_63 * __stdcall16far pass1_1020_0ae8(astruct_63 *param_1,byte param_2,ushort param_3)

{
  send_win_msg_1020_08fe(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1020_0baa(ushort *param_1,ushort param_2,uchar *param_3,ushort param_4)

{
  uchar *puVar1;
  astruct_276 *iVar2;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  undefined2 *puVar4;
  undefined2 *puVar5;
  undefined2 uVar6;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_276 *)param_1;
  *param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar2->field_0x2 = 0x1008;
  *(undefined4 *)&iVar2->field_0x6 = 0x0;
  iVar2->field_0xa = 0x0;
  iVar2->field_0xc = 0x0;
  *param_1 = 0xdbc;
  iVar2->field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x7,param_4,param_3,unaff_DI);
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  iVar2->field_0x6 = (int)puVar3;
  iVar2->field_0x8 = puVar1;
  puVar5 = &iVar2->field_0xa;
  puVar4 = &iVar2->field_0xc;
  uVar6 = uVar2;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar1,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar3 & 0xffff0000 | (ulong)((int)puVar3 + 0xe)),(ushort *)CONCAT22(uVar2,puVar4),
                  (ushort *)CONCAT22(uVar6,puVar5));
  return;
}



void __stdcall16far unk_draw_op_1020_0c3e(undefined4 param_1,HWND16 param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  HDC16 *b_force_background;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  uint uStack40;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  local_24 = BeginPaint16(param_2,&local_22);
  uVar3 = *(undefined4 *)(iVar4 + 0x6);
  uVar7 = (undefined2)((ulong)uVar3 >> 0x10);
  iVar5 = (int)uVar3;
  puVar1 = (undefined4 *)*(undefined4 *)(iVar5 + 0xa);
  uStack40 = (uint)puVar1;
  if ((*(uint *)(iVar5 + 0xc) | uStack40) != 0x0) {
    b_force_background = &local_24;
    uVar3 = *puVar1;
    ppcVar2 = (code **)((int)uVar3 + 0x8);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,uStack40,(int)((ulong)puVar1 >> 0x10),b_force_background);
    ppcVar2 = (code **)((int)uVar3 + 0x4);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538,puVar1,*(undefined2 *)(iVar4 + 0xc),*(undefined2 *)(iVar4 + 0xa),&local_24);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,(BOOL16)b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  }
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



void __stdcall16far win_ui_palette_op_1020_0cd2(undefined4 param_1,HWND16 param_2)

{
  uint uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  undefined4 uVar4;
  uint uVar5;
  HDC16 hdc;
  HDC16 b_force_background;
  HPALETTE16 b_force_background_00;
  UINT16 UVar6;
  uint extraout_DX;
  int iVar7;
  undefined2 uVar8;
  astruct_13 *paStack10;
  uint uStack6;
  
  uVar4 = *(undefined4 *)((int)param_1 + 0x6);
  uVar8 = (undefined2)((ulong)uVar4 >> 0x10);
  iVar7 = (int)uVar4;
  puVar2 = (undefined4 *)*(undefined4 *)(iVar7 + 0xa);
  uVar1 = *(uint *)(iVar7 + 0xc);
  uStack6 = (uint)puVar2;
  uVar5 = uVar1 | uStack6;
  if (uVar5 != 0x0) {
    ppcVar3 = (code **)((int)*puVar2 + 0x14);
    (**ppcVar3)(param_2,uStack6,uVar1);
    paStack10 = (astruct_13 *)CONCAT22(extraout_DX,uVar5);
    uVar5 = extraout_DX | uVar5;
    if (uVar5 != 0x0) {
      hdc = GetDC16(param_2);
      b_force_background = hdc;
      create_palette_1008_4e38(paStack10,0x1008,uVar5);
      b_force_background_00 = SelectPalette16(0x1008,0x0,b_force_background);
      UVar6 = RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
      SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x1,b_force_background_00);
      DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
      if (0x0 < (int)UVar6) {
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
      }
      ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,hdc);
      return;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_0d82(astruct_18 *param_1,byte param_2)

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



void __stdcall16far pass1_1020_0dc4(ushort *param_1,UINT16 param_2,ulong param_3,UINT16 param_4)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1020_790e(param_1,(ulong)s_PCPOPMENU_1050_4256,param_2,param_3,param_4);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0xf2) = 0x0;
  *(undefined4 *)(iVar1 + 0xf6) = 0x0;
  *(undefined2 *)(iVar1 + 0xfa) = 0x0;
  *param_1 = 0x1384;
  *(undefined2 *)(iVar1 + 0x2) = 0x1020;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x5b)),s_VrMode_1050_4260);
  *(undefined4 *)(iVar1 + 0xac) = 0x44c00000;
  window_op_1020_10a0(param_1);
  return;
}



void __stdcall16far pass1_1020_0e2c(ulong param_1,ushort param_2)

{
  get_win_ui_info_op_1020_7a50(param_1,param_2);
  cleanup_ui_op_1020_1038(param_1);
  return;
}



void __stdcall16far realize_palette_1020_0e46(ulong param_1,int param_2,HGDIOBJ16 param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  
  if (param_2 != 0x0) {
    uVar3 = *(undefined4 *)((int)param_1 + 0xf2);
    uVar5 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar4 = (int)uVar3;
    puVar1 = (undefined4 *)*(undefined4 *)(iVar4 + 0x66);
    ppcVar2 = (code **)((int)*puVar1 + 0x18);
    (**ppcVar2)(param_3,(int)puVar1,*(undefined2 *)(iVar4 + 0x68));
    UnrealizeObject16(param_3);
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  }
  return;
}



void __stdcall16far
pass1_1020_0e8e(int param_1,ushort param_2,int param_3,int param_4,int param_5,ushort param_6,ushort param_7)

{
  code **ppcVar1;
  
  win_ui_cursor_op_1020_1294(CONCAT22(param_2,param_1),param_3,param_4,param_6,param_7);
  if (param_5 == 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x4) + 0x5c);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_help_op_1020_0ec4(ulong *param_1,uint param_2,ushort param_3)

{
  code **ppcVar1;
  char cVar2;
  ushort uVar3;
  uchar *in_DX;
  ushort uVar4;
  int unaff_DI;
  ushort *puVar5;
  ulong uVar6;
  astruct_43 *paVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  int iVar10;
  
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  uVar3 = (ushort)param_1;
  if (param_2 == 0xfb) {
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_3,in_DX,unaff_DI);
    pass1_1010_375e((ulong)puVar5);
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
    uVar6 = pass1_1010_375e((ulong)puVar5);
    uVar4 = (ushort)(uVar6 >> 0x10);
    pass1_1010_4788(*(ulong *)(uVar3 + 0xf2),(char *)(uVar6 & 0xffff | (ulong)uVar4 << 0x10),(ushort)uVar6,uVar4);
    return;
  }
  if (0xfb < param_2) {
    switch(param_2) {
    default:
      return;
    case 0x12a:
      uVar8 = 0xf012;
      break;
    case 0x12c:
      uVar8 = 0xf020;
    }
    PostMessage16(0x1020,0x0,0x0,CONCAT22(0x112,uVar8));
    return;
  }
  if (param_2 == 0xf3) {
    iVar10 = 0x3;
  }
  else {
    if (0xf3 < param_2) {
      return;
    }
    cVar2 = (char)param_2;
    if (cVar2 == 'o') {
      paVar7 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_3);
      WinHelp16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x8),0x0,CONCAT22((int)paVar7,0x1));
      return;
    }
    if (cVar2 == 'r') {
      iVar10 = uVar3 + 0xa;
      uVar9 = uVar8;
      puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_3,in_DX,unaff_DI);
      uVar4 = (ushort)((ulong)puVar5 >> 0x10);
      pass1_1010_3770((ulong)puVar5,(char *)CONCAT22(uVar9,iVar10),uVar4);
      pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar3 + 0x8),0x3,uVar4,uVar3,(ushort)&PTR_LOOP_1050_1038,param_3);
      return;
    }
    if (cVar2 == -0xf) {
      iVar10 = 0x1;
    }
    else {
      if (cVar2 != -0xe) {
        return;
      }
      iVar10 = 0x2;
    }
  }
  pass1_1010_4674(*(ulong *)(uVar3 + 0xf2),iVar10,0x1010,param_3);
  return;
}



void __stdcall16far enable_menu_1020_1000(HMENU16 param_1,int param_2)

{
  if (param_2 != 0x0) {
    return;
  }
  EnableMenuItem16(param_1,0x400,0x3);
  return;
}



void __stdcall16far pass1_1020_1022(ulong param_1,ushort param_2)

{
  draw_op_1020_15de(*(undefined4 *)((int)param_1 + 0xf6),param_2);
  return;
}



void __stdcall16far cleanup_ui_op_1020_1038(undefined4 param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  HICON16 unaff_CS;
  undefined2 uVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar6 = *(undefined2 *)(iVar4 + 0xc2);
  DestroyIcon16(unaff_CS);
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

void __stdcall16far window_op_1020_10a0(astruct *param_1)

{
  undefined4 uVar1;
  code **ppcVar2;
  astruct_160 *in_AX;
  uint uVar3;
  BOOL16 *pBVar4;
  uchar *in_DX;
  uchar *puVar5;
  uchar *puVar6;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  int unaff_DI;
  ushort unaff_SS;
  uchar in_AF;
  ushort *puVar7;
  ulong uVar8;
  undefined2 uVar9;
  undefined *puVar10;
  int iVar11;
  undefined2 uVar12;
  
  iVar11 = (int)param_1;
  uVar12 = (undefined2)((ulong)param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0x42,in_DX,0x1000);
  puVar5 = (uchar *)((uint)in_DX | (uint)in_AX);
  if (puVar5 != (uchar *)0x0) {
    pass1_1008_3bd6(in_AX,(ushort)in_DX,0x0,0x1f009b,0x0,0x740075,CONCAT22(*(undefined2 *)(iVar11 + 0x8),0xf1),
                    (ushort)puVar5,unaff_SS);
  }
  mem_op_1000_179c(0x42,puVar5,0x1000);
  puVar6 = (uchar *)((uint)puVar5 | (uint)in_AX);
  if (puVar6 != (uchar *)0x0) {
    pass1_1008_3bd6(in_AX,(ushort)puVar5,0x0,0x31009b,0x0,0x760077,CONCAT22(*(undefined2 *)(iVar11 + 0x8),0xf2),
                    (ushort)puVar6,unaff_SS);
  }
  mem_op_1000_179c(0x42,puVar6,0x1000);
  puVar5 = (uchar *)((uint)puVar6 | (uint)in_AX);
  if (puVar5 != (uchar *)0x0) {
    pass1_1008_3bd6(in_AX,(ushort)puVar6,0x0,0x77009b,0x0,0x780079,CONCAT22(*(undefined2 *)(iVar11 + 0x8),0xf3),
                    (ushort)puVar5,unaff_SS);
  }
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2d,unaff_SS,puVar5,unaff_DI);
  uVar9 = (undefined2)((ulong)puVar7 >> 0x10);
  *(undefined2 *)(iVar11 + 0xf2) = (int)puVar7;
  *(undefined2 *)(iVar11 + 0xf4) = uVar9;
  *(undefined2 *)(iVar11 + 0xe0) = *(undefined2 *)(iVar11 + 0xf2);
  *(undefined2 *)(iVar11 + 0xe2) = uVar9;
  puVar10 = PTR_LOOP_1050_038c;
  uVar3 = LoadIcon16(0x1010,(LPCSTR)s_PLNTICON_1050_4267);
  *(HICON16 *)(iVar11 + 0xc2) = uVar3;
  uVar1 = *(undefined4 *)(iVar11 + 0xf2);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xf2) + 0x30);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar1,(int)((ulong)uVar1 >> 0x10),uVar3,puVar10);
  puVar5 = extraout_DX;
  mem_op_1000_179c(0x24,extraout_DX,0x1000);
  puVar6 = (uchar *)((uint)puVar5 | uVar3);
  if (puVar6 == (uchar *)0x0) {
    *(undefined4 *)(iVar11 + 0xf6) = 0x0;
  }
  else {
    unk_win_ui_op_1020_1418((astruct_40 *)CONCAT22(puVar5,uVar3),(ULONG)param_1,unaff_SS);
    *(uint *)(iVar11 + 0xf6) = uVar3;
    *(uchar **)(iVar11 + 0xf8) = puVar6;
  }
  *(undefined4 *)(iVar11 + 0xe8) = *(undefined4 *)(iVar11 + 0xf6);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,puVar6,unaff_DI);
  uVar8 = pass1_1018_04b8((ulong)puVar7);
  puVar5 = (uchar *)(uVar8 >> 0x10);
  pass1_1010_41d6(*(ulong *)(iVar11 + 0xf2),uVar8,puVar5,unaff_SS,in_AF);
  uVar8 = pass1_1010_451a(*(ulong *)(iVar11 + 0xf2),puVar5,unaff_DI,unaff_SS);
  pBVar4 = (BOOL16 *)uVar8;
  uVar1 = *(undefined4 *)param_1;
  ppcVar2 = (code **)((int)uVar1 + 0x14);
  (**ppcVar2)(0x1010,iVar11,uVar12,0x0,pBVar4,(char)(uVar8 >> 0x10));
  uVar9 = 0x1;
  ppcVar2 = (code **)((int)uVar1 + 0x10);
  (**ppcVar2)();
  pass1_1010_459e(*(long *)(iVar11 + 0xf2));
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xf2) + 0x10);
  (**ppcVar2)(0x1010,*(undefined4 *)(iVar11 + 0xf2),param_1,uVar9);
  MoveWindow16(0x1010,0x1,pBVar4[0x3],pBVar4[0x2],pBVar4[0x1],*pBVar4);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_cursor_op_1020_1294(ulong param_1,int param_2,int param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  uint in_AX;
  HCURSOR16 HVar2;
  HCURSOR16 HVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  int local_12;
  int local_10;
  ushort *puStack14;
  undefined4 *puStack10;
  int local_6;
  int iStack4;
  
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x4000001);
  if ((param_4 | in_AX) == 0x0) {
    local_6 = param_3;
    iStack4 = param_2;
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    puStack10 = (undefined4 *)pass1_1010_40cc(*(ulong *)(iVar4 + 0xf2),param_2,0x0);
    uVar6 = *(ulong *)(iVar4 + 0xf2);
    puStack14 = (ushort *)(uVar6 & 0xffff0000 | (ulong)((int)uVar6 + 0x76));
    pass1_1008_3e94(puStack14,(ushort *)CONCAT22(param_5,&local_12),(ushort *)CONCAT22(param_5,&local_10));
    local_6 = local_6 - local_10;
    iStack4 = iStack4 - local_12;
    iVar4 = pt_in_rect_1010_40f8(*(ulong *)(iVar4 + 0xf2),(POINT16 *)CONCAT22(param_5,&local_6),0x1010);
    if (iVar4 != -0x1) {
      uVar6 = 0x0;
      HVar2 = LoadCursor16(0x1010,(LPCSTR)0x7f02);
      uVar6 = uVar6 & 0xffff0000 | (ulong)HVar2;
      HVar3 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
      ppcVar1 = (code **)((int)*puStack10 + 0x4);
      (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack10,(int)((ulong)puStack10 >> 0x10),iVar4,iVar4 >> 0xf,iVar4,0x2
                  ,uVar6,HVar3,HVar2);
      pass1_1008_3e0e(param_1);
      SetCursor16(0x1008);
    }
  }
  return;
}



astruct_3 * __stdcall16far pass1_1020_135e(astruct_3 *param_1,byte param_2,ushort param_3)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1020_1418(astruct_40 *param_1,ULONG param_2,UINT16 param_3)

{
  undefined4 uVar1;
  astruct_13 *paVar2;
  code **ppcVar3;
  HDC16 *pHVar4;
  undefined4 *puVar5;
  uchar *puVar6;
  uchar *extraout_DX;
  astruct_40 *iVar5;
  int unaff_DI;
  undefined2 uVar7;
  undefined2 unaff_CS;
  ushort *puVar8;
  HDC16 local_8;
  ushort *puStack6;
  
  get_sys_metrics_1020_7c1a((ushort *)param_1,param_2,unaff_CS);
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_40 *)param_1;
  *(undefined4 *)&iVar5->field_0x14 = 0x0;
  iVar5->field_0x18 = (undefined4 *)0x0;
  puVar8 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x1e));
  *(undefined2 *)param_1 = 0x1730;
  iVar5->field_0x2 = 0x1020;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2d,param_3,(uchar *)((ulong)puVar8 >> 0x10),unaff_DI);
  puVar6 = (uchar *)((ulong)puVar8 >> 0x10);
  iVar5->field_0x14 = (int)puVar8;
  *(uchar **)&iVar5->field_0x16 = puVar6;
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_3,puVar6,unaff_DI);
  uVar1 = *(undefined4 *)&iVar5->field_0x14;
  ppcVar3 = (code **)((int)**(undefined4 **)&iVar5->field_0x14 + 0x4);
  (**ppcVar3)(0x1010,(int)uVar1,(int)((ulong)uVar1 >> 0x10),0x0,param_1);
  local_8 = GetDC16(0x1010);
  uVar1 = *(undefined4 *)&iVar5->field_0x14;
  *(HDC16 *)((int)uVar1 + 0x7c) = local_8;
  uVar1 = *(undefined4 *)&iVar5->field_0x14;
  puVar5 = (undefined4 *)*(ulong *)((int)uVar1 + 0x66);
  iVar5->field_0x18 = puVar5;
  ppcVar3 = (code **)((int)*iVar5->field_0x18 + 0x14);
  (**ppcVar3)();
  paVar2 = *(astruct_13 **)((int)puStack6 + 0xe);
  puVar6 = extraout_DX;
  pass1_1008_4d84((astruct_90 *)((ulong)puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10),(ulong)paVar2,extraout_DX);
  pHVar4 = (HDC16 *)palette_op_1008_4e08(paVar2,&local_8,(ushort)puVar6,0x1008);
  iVar5->field_0x1c = pHVar4;
  return;
}



void __stdcall16far win_ui_op_1020_150e(undefined2 *param_1,HDC16 param_2)

{
  HPALETTE16 HVar1;
  int iVar2;
  uint uVar3;
  ushort unaff_SS;
  
  uVar3 = (uint)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *param_1 = 0x1730;
  *(undefined2 *)(iVar2 + 0x2) = 0x1020;
  if (*(long *)(iVar2 + 0x14) != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6(*(ulong *)(iVar2 + 0x14),(ulong)param_1 & 0xffff | (ulong)uVar3 << 0x10,unaff_SS);
  }
  HVar1 = SelectPalette16(param_2,0x0,*(BOOL16 *)(iVar2 + 0x1c));
  *(HPALETTE16 *)(iVar2 + 0x1c) = HVar1;
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  *param_1 = 0x3ab0;
  *(undefined2 *)(iVar2 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar2 + 0x2) = 0x1008;
  return;
}



void __stdcall16far invalidate_rect_1020_157c(ulong param_1,int param_2,HWND16 param_3)

{
  BOOL16 BVar1;
  RECT16 local_a;
  ushort uStack4;
  
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    BVar1 = IsIconic16(param_3);
    if (BVar1 == 0x0) {
      GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
      uStack4 = 0x9a;
      InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,(BOOL16)&local_a);
      return;
    }
  }
  return;
}



void __stdcall16far draw_op_1020_15de(ULONG param_1,HWND16 in_win_handle_2)

{
  ulong uVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  HWND16 hwnd;
  ushort unaff_SS;
  ulong uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  uVar9 = *(undefined2 *)(iVar5 + 0x4);
  local_24 = BeginPaint16(in_win_handle_2,&local_22);
  uVar8 = *(undefined2 *)(iVar5 + 0x4);
  hwnd = (HWND16)s_tile2_bmp_1050_1538;
  BVar3 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
  if (BVar3 == 0x0) {
    hwnd = 0x1010;
    uVar7 = pass1_1010_454a(*(ulong *)(iVar5 + 0x14));
    uVar4 = (uint)(uVar7 >> 0x10);
    if ((uVar4 | (uint)uVar7) != 0x0) {
      uVar1 = *(ulong *)(iVar5 + 0x14);
      hwnd = 0x1008;
      pass1_1008_4480(*(ulong *)(iVar5 + 0x18),(ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x76)),
                      (astruct_76 *)(uVar7 & 0xffff | (ulong)uVar4 << 0x10),unaff_SS);
    }
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x18) + 0x4);
    (**ppcVar2)(hwnd,*(undefined4 *)(iVar5 + 0x18),0x0,&local_24,unaff_SS,uVar8,uVar9);
  }
  else {
    draw_op_1020_1674(param_1,(int)s_tile2_bmp_1050_1538);
  }
  EndPaint16(hwnd,&local_22);
  return;
}



void __stdcall16far draw_op_1020_1674(ULONG param_1,INT16 param_2)

{
  code **ppcVar1;
  undefined2 uVar2;
  undefined2 local_1a;
  undefined2 uStack24;
  int iStack22;
  int iStack20;
  int iStack18;
  int iStack16;
  RECT16 local_e;
  int iStack10;
  int iStack8;
  RECT16 *pRStack6;
  int iStack4;
  
  if (PTR_LOOP_1050_0010 == (undefined *)0x0) {
    uVar2 = (undefined2)(param_1 >> 0x10);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x14) + 0x2c);
    iStack4 = (**ppcVar1)(param_2,*(undefined4 *)((int)param_1 + 0x14));
    if (iStack4 != 0x0) {
      pRStack6 = (RECT16 *)GetStockObject16(param_2);
      GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_e);
      local_1a = 0x0;
      uStack24 = 0x0;
      iStack22 = (iStack10 - local_e.x) + -0x1;
      iStack20 = (iStack8 - local_e.y) + -0x1;
      iStack18 = iStack20;
      iStack16 = iStack22;
      FillRect16((HDC16)s_tile2_bmp_1050_1538,pRStack6,(HBRUSH16)&local_1a);
      DrawIcon16((HDC16)s_tile2_bmp_1050_1538,iStack4,0x2,0x2);
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_170a(astruct_18 *param_1,byte param_2,undefined2 param_3)

{
  win_ui_op_1020_150e(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1020_1738(astruct_57 *param_1,ushort param_2,ulong param_3)

{
  astruct_278 *iVar1;
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcd,*(ushort *)((int)param_3 + 0x8));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_278 *)param_1;
  iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x96 = 0x0;
  *(undefined2 *)param_1 = 0x1e7a;
  iVar1->field_0x2 = 0x1020;
  return;
}



void __stdcall16far pass1_1020_1780(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x6c);
  (**ppcVar1)();
  destroy_win_1040_8212((ULONG)param_1,(HWND16)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mixed_ui_op_1020_179c(astruct_1 *param_1)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 uVar3;
  ushort uVar4;
  INT16 IVar5;
  undefined *puVar6;
  uchar *in_DX;
  uchar *extraout_DX;
  uchar *puVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int unaff_DI;
  undefined2 uVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  WNDCLASS16 *unaff_SS;
  ushort *puVar14;
  WNDCLASS16 *in_resc_id_3;
  WNDCLASS16 *in_buffer_4;
  WNDCLASS16 local_178 [0xc];
  undefined4 uStack118;
  undefined4 uStack114;
  RECT16 local_6e;
  undefined4 uStack106;
  HWND16 HStack102;
  int iStack98;
  int iStack94;
  uint uStack78;
  uchar *puStack76;
  undefined4 uStack74;
  HWND16 HStack70;
  undefined4 uStack68;
  undefined4 uStack64;
  LPVOID pvStack60;
  ushort uStack58;
  undefined2 uStack56;
  ULONG *pUStack54;
  undefined4 uStack50;
  undefined local_2e [0x12];
  RECT16 local_1c;
  undefined4 uStack24;
  int iStack20;
  int iStack18;
  ushort *puStack16;
  INT16 *pIStack12;
  uint uStack8;
  ushort *puStack6;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  uVar4 = 0x89;
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,(ushort)unaff_SS,in_DX,unaff_DI);
  puVar7 = (uchar *)((ulong)puStack6 >> 0x10);
  uVar4 = pass1_1010_65d0((ushort)unaff_SS,(ulong)puStack6,uVar4);
  uStack8 = (uint)(uVar4 == 0x0);
  uVar4 = pass1_1010_65d0((ushort)unaff_SS,(ulong)puStack6,0x86);
  if (uVar4 != 0x0) {
    uStack8 = 0x0;
  }
  puVar14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,(ushort)unaff_SS,puVar7,unaff_DI);
  uVar12 = (undefined2)((ulong)puVar14 >> 0x10);
  uVar8 = (uint)puVar14;
  uVar11 = (undefined2)((ulong)param_1 >> 0x10);
  iVar9 = (int)param_1;
  *(uint *)(iVar9 + 0x8e) = uVar8;
  *(undefined2 *)(iVar9 + 0x90) = uVar12;
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar9 + 0x8e) + 0x10);
  (**ppcVar2)(0x1010,*(undefined2 *)(iVar9 + 0x8e),uVar12,uStack8);
  puStack76 = extraout_DX;
  mem_op_1000_179c(0x12,extraout_DX,0x1000);
  puVar7 = (uchar *)((uint)puStack76 | uVar8);
  uStack78 = uVar8;
  if (puVar7 == (uchar *)0x0) {
    *(undefined4 *)(iVar9 + 0x92) = 0x0;
  }
  else {
    pass1_1020_1eea((ushort *)CONCAT22(puStack76,uVar8),(ulong)param_1,*(ushort *)(iVar9 + 0x6),puVar7,unaff_DI,
                    (ushort)unaff_SS);
    *(uint *)(iVar9 + 0x92) = uVar8;
    *(uchar **)(iVar9 + 0x94) = puVar7;
  }
  uVar1 = *(ulong *)(iVar9 + 0x8e);
  pIStack12 = (INT16 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0xa));
  puStack16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,(ushort)unaff_SS,puVar7,unaff_DI);
  GetClientRect16(0x1010,&local_1c);
  IVar5 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  uVar12 = (undefined2)((ulong)pIStack12 >> 0x10);
  iVar10 = (int)pIStack12;
  *(int *)(iVar10 + 0x6) = IVar5 + uStack24._2_2_;
  uVar13 = (undefined2)((ulong)puStack16 >> 0x10);
  iStack18 = *(int *)((int)puStack16 + 0xa);
  iStack20 = *(int *)((int)puStack16 + 0xc);
  *(int *)(iVar10 + 0x2) = (iStack20 - *(int *)(iVar10 + 0x6)) / 0x2;
  iVar10 = iStack18 - *(int *)(iVar10 + 0x4);
  uVar8 = iVar10 >> 0xf;
  *pIStack12 = iVar10 / 0x2;
  pass1_1028_dc52((astruct_92 *)CONCAT22(unaff_SS,local_2e),0x1,0x0,0x100);
  uStack56 = 0x0;
  while( true ) {
    puVar6 = local_2e;
    pass1_1028_e4ec(CONCAT22(unaff_SS,puVar6));
    uStack50 = CONCAT22(uVar8,puVar6);
    uStack58 = uVar8 | (uint)puVar6;
    if (uStack58 == 0x0) break;
    pUStack54 = *(ULONG **)(puVar6 + 0x10);
    uVar8 = uStack58;
    if (pUStack54 != (ULONG *)0x0) {
      pass1_1000_3cea((ulong)param_1 & 0xffff0000 | (ulong)(iVar9 + 0x10),*pUStack54);
      uVar8 = uStack58;
    }
  }
  uVar4 = pass1_1020_1da8((ulong)param_1,puVar6,0x0,unaff_SS);
  *(ushort *)(iVar9 + 0x96) = uVar4;
  uVar4 = pass1_1010_65d0((ushort)unaff_SS,(ulong)puStack6,0x86);
  if ((uVar4 == 0x0) || (*(int *)(iVar9 + 0x96) != 0x0)) {
    uVar3 = *(undefined4 *)(iVar9 + 0x8e);
    *(undefined2 *)((int)uVar3 + 0x2c) = 0x0;
    HStack102 = GetDlgItem16(0x1010,0x175);
    if (uStack8 != 0x0) {
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,(char *)local_178
                 ,(short)unaff_SS);
      SetWindowText16(0x1010,(SEGPTR)local_178);
    }
    pvStack60 = MakeProcInstance16((LPVOID)s_tile2_bmp_1050_1538,(HANDLE16)PTR_LOOP_1050_038c);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_6e);
    uStack114 = uStack106;
    iStack98 = (int)uStack106 - local_6e.x;
    iStack94 = uStack106._2_2_ - local_6e.y;
    uStack118 = local_6e & 0xffff0000 | (ulong)(uint)(-(iStack98 - *(int *)((int)pIStack12 + 0x4)) / 0x2);
    IVar5 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    uVar1 = uStack118 & 0xffff;
    uStack118 = uVar1 | (ulong)(uint)(uStack118._2_2_ - IVar5) << 0x10;
    uStack118._0_2_ = (BOOL16)uVar1;
    MoveWindow16((HWND16)s_tile2_bmp_1050_1538,0x0,iStack94,iStack98,uStack118._2_2_ - IVar5,(BOOL16)uStack118);
  }
  else {
    win_1008_5c7c(_PTR_LOOP_1050_02a0,0x9d0001,unaff_SS,uVar4,uStack58);
    *(ushort *)(iVar9 + 0x8c) = uVar4;
    pvStack60 = MakeProcInstance16((LPVOID)0x1008,(HANDLE16)PTR_LOOP_1050_038c);
  }
  EnumChildWindows1((HWND16)s_tile2_bmp_1050_1538,(LPVOID)0x0,ZEXT24(pvStack60) << 0x10);
  FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
  HStack70 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_1c);
  uStack64 = uStack24;
  local_1c.x = (int)uStack24 - local_1c.x;
  uStack74 = CONCAT22(local_1c.x,uStack24._2_2_ - local_1c.y);
  uStack68 = local_1c & 0xffff0000 | (ulong)(uint)(-(local_1c.x - *(int *)((int)pIStack12 + 0x4)) / 0x2);
  IVar5 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  uStack68 = uStack68 & 0xffff | (ulong)(uint)(uStack68._2_2_ - IVar5) << 0x10;
  if (*(int *)(iVar9 + 0x96) == 0x0) {
    if (uStack8 == 0x0) goto LAB_1020_1b24;
    in_buffer_4 = local_178;
    in_resc_id_3 = (WNDCLASS16 *)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21);
  }
  else {
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,(char *)local_178,
               (short)unaff_SS);
    GetDlgItem16(0x1010,0x175);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_178);
    in_resc_id_3 = local_178;
    in_buffer_4 = unaff_SS;
    unaff_SS = (WNDCLASS16 *)0x3fe;
  }
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),(ushort)in_resc_id_3,
             (char *)in_buffer_4,(short)unaff_SS);
  SetWindowText16(0x1010,(SEGPTR)local_178);
LAB_1020_1b24:
  MoveWindow16((HWND16)s_tile2_bmp_1050_1538,0x0,(INT16)uStack74,(INT16)((ulong)uStack74 >> 0x10),uStack68._2_2_,
               (BOOL16)uStack68);
  uVar12 = (undefined2)((ulong)pIStack12 >> 0x10);
  iVar9 = (int)pIStack12;
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x44,*(INT16 *)(iVar9 + 0x6),*(INT16 *)(iVar9 + 0x4),
                 *(INT16 *)(iVar9 + 0x2),*pIStack12,0x0);
  return;
}
