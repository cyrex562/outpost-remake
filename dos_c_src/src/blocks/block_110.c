


void __stdcall16far pass1_1040_869a(astruct_18 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  param_1->field_0x0 = 0x8ddc;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x90),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x94),0x1000);
  ui_cleanup_op_1040_782c(param_1,0x1000);
  return;
}



void __stdcall16far enable_win_1040_86dc(HWND16 param_1)

{
  HWND16 HVar1;
  
  HVar1 = GetDlgItem16(param_1,0x1);
  if (HVar1 != 0x0) {
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x2);
    if (HVar1 != 0x0) {
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar * __stdcall16far win_ui_op_1040_8718(astruct_37 *param_1,ushort param_2)

{
  int *piVar1;
  int iVar2;
  ushort uVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort *puVar6;
  undefined2 uVar7;
  undefined2 uVar9;
  UINT32 UVar10;
  int local_104 [0x80];
  uint uStack4;
  ushort uVar8;
  
  uVar5 = 0x1008;
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_2);
  UVar10 = (UINT32)param_1;
  uVar8 = (ushort)((ulong)param_1 >> 0x10);
  dialog_ui_fn_1040_78e2((astruct_1 *)param_1,0x1008);
  PTR_LOOP_1050_5df6 = (undefined *)*(undefined2 *)(UVar10 + 0x6);
  if (*(long *)(UVar10 + 0x94) != 0x0) {
    uVar5 = 0x1000;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(UVar10 + 0x10)),*(char **)(UVar10 + 0x94));
  }
  get_sys_metrics_1040_8c66(param_1,uVar5);
  uStack4 = *(byte *)(UVar10 + 0x98) & 0xf;
  if (uStack4 == 0x1) {
    *(int *)(UVar10 + 0xae) = (*(int *)(UVar10 + 0xaa) + -0xc4) / 0x2;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    create_window_1040_8bea(UVar10,uVar8,0x1,0x1,(int)*(undefined4 *)(UVar10 + 0xae));
    piVar1 = (int *)(UVar10 + 0xae);
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    uVar9 = (undefined2)*(undefined4 *)(UVar10 + 0xae);
    uVar7 = 0x2;
  }
  else {
    if (uStack4 != 0x4) {
      *(int *)(UVar10 + 0xae) = (*(int *)(UVar10 + 0xaa) + -0x58) / 0x2;
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
                 param_2);
      uVar9 = (undefined2)*(undefined4 *)(UVar10 + 0xae);
      uVar5 = 0x1;
      uVar7 = 0x1;
      goto LAB_1040_88a5;
    }
    *(int *)(UVar10 + 0xae) = (*(int *)(UVar10 + 0xaa) + -0xc4) / 0x2;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    create_window_1040_8bea(UVar10,uVar8,0x1,0x6,(int)*(undefined4 *)(UVar10 + 0xae));
    piVar1 = (int *)(UVar10 + 0xae);
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    uVar9 = (undefined2)*(undefined4 *)(UVar10 + 0xae);
    uVar7 = 0x7;
  }
  uVar5 = 0x0;
LAB_1040_88a5:
  create_window_1040_8bea(UVar10,uVar8,uVar5,uVar7,uVar9);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,extraout_DX,unaff_DI);
  uVar5 = (undefined2)((ulong)puVar6 >> 0x10);
  local_104[0] = *(int *)((int)puVar6 + 0xa);
  uStack4 = *(int *)((int)puVar6 + 0xc);
  iVar2 = uStack4 - *(int *)(UVar10 + 0xac);
  puVar4 = (uchar *)(iVar2 >> 0xf);
  SetWindowPos16(0x1010,0x40,*(INT16 *)(UVar10 + 0xac),*(INT16 *)(UVar10 + 0xaa),iVar2 / 0x2,
                 (local_104[0] - *(int *)(UVar10 + 0xaa)) / 0x2,0x0);
  PTR_LOOP_1050_5df4 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_2);
  destroy_win_1040_8b7e(0x1008);
  PTR_LOOP_1050_5df6 = (undefined *)0x0;
  if (*(int *)(UVar10 + 0xb2) != 0x0) {
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_2,puVar4,unaff_DI);
    uVar3 = pass1_1008_ab54((ulong)puVar6);
    if (uVar3 != 0x0) {
      PostMessage16(0x1008,0x0,0x0,0x11100fc);
    }
  }
  return PTR_LOOP_1050_5df8;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_8978(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,WNDCLASS16 *param_5)

{
  code **ppcVar1;
  
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_5);
  win_1008_5c5c(param_5,param_3,param_4,_PTR_LOOP_1050_02a0,param_2);
  ppcVar1 = (code **)((int)*param_1 + 0x74);
  (**ppcVar1)(0x1008,param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_89a4(ulong *param_1,ushort *param_2,uchar *param_3,int param_4,WNDCLASS16 *param_5)

{
  undefined2 uVar1;
  ushort uVar2;
  code **ppcVar3;
  ushort uVar4;
  ushort uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_5);
  uVar1 = *(undefined2 *)((int)param_2 + 0x2);
  uVar2 = *param_2;
  uVar6 = 0x1010;
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_5,param_3,param_4);
  uVar5 = (ushort)((ulong)puVar7 >> 0x10);
  uVar4 = (ushort)puVar7;
  if (*(int *)(uVar4 + 0x72) != 0x0) {
    uVar6 = 0x1008;
    win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uVar1,uVar2),param_5,uVar4,uVar5);
    *(ushort *)((int)param_1 + 0x8c) = uVar4;
  }
  ppcVar3 = (code **)((int)*param_1 + 0x74);
  (**ppcVar3)(uVar6,param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mixed_draw_op_1040_8a06(ulong param_1,HWND16 param_2,ushort param_3)

{
  undefined uVar1;
  undefined uVar2;
  astruct_13 *paVar3;
  undefined2 uVar4;
  HPALETTE16 b_force_background;
  COLORREF color;
  COLORREF color_00;
  HANDLE16 handle;
  ushort in_DX;
  RECT16 *rect;
  ulong uVar5;
  HGDIOBJ16 HStack62;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  rect = (RECT16 *)(param_1 >> 0x10);
  local_24 = BeginPaint16(param_2,&local_22);
  paVar3 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
  b_force_background = palette_op_1008_4e08(paVar3,&local_24,in_DX,0x1008);
  uVar5 = pass1_1008_4d72((ulong)paVar3);
  uVar4 = (undefined2)(uVar5 >> 0x10);
  uVar1 = *(undefined *)((int)uVar5 + 0x95);
  uVar2 = *(undefined *)((int)uVar5 + 0x96);
  DrawIcon16(0x1008,*(INT16 *)((int)param_1 + 0x8e),0xa,0x14);
  color = SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538,CONCAT11(uVar1,uVar2));
  HStack62 = 0x0;
  handle = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5dfa);
  if (handle != 0x0) {
    HStack62 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  }
  DrawText16((HDC16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,(int)param_1 + 0x9e,rect,0xffff);
  if (handle != 0x0) {
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack62);
  }
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,color);
  SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color_00);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



void __stdcall16far pass1_1040_8b3c(ushort param_1,ulong param_2,ulong param_3,ushort param_4)

{
  if ((param_3._2_2_ != (undefined *)0x0) &&
     ((param_3._2_2_ == (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1) ||
       param_3._2_2_ == (undefined *)&PTR_LOOP_1050_0002 ||
      (((undefined *)((int)&PTR_LOOP_1050_0002 + 0x1U) < param_3._2_2_ + -0x2 &&
       (param_3._2_2_ + -0x6 < (undefined *)&PTR_LOOP_1050_0002)))))) {
    PTR_LOOP_1050_5df4 = (undefined *)0x0;
    PTR_LOOP_1050_5df8 = param_3._2_2_;
    return;
  }
  post_win_msg_1040_7b3c
            ((ulong *)CONCAT22((int)param_2,param_1),(ushort)(param_2 >> 0x10),(ushort)param_3,(int)param_3._2_2_,
             param_4);
  return;
}



void __stdcall16far destroy_win_1040_8b7e(HWND16 param_1)

{
  DestroyWindow16(param_1);
  return;
}



void __stdcall16far load_icon_1040_8b92(ulong param_1,HINSTANCE16 param_2)

{
  byte bVar1;
  HICON16 HVar2;
  undefined2 uVar3;
  LPCSTR name;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  bVar1 = *(byte *)((int)param_1 + 0x98) & 0xf0;
  if (bVar1 == 0x30) {
    name = (LPCSTR)0x7f03;
  }
  else {
    if ((bVar1 == 0x10) || (bVar1 == 0x10)) {
      name = (LPCSTR)0x7f01;
    }
    else {
      if ((bVar1 == 0x40) || (bVar1 == 0x40)) {
        name = (LPCSTR)0x7f04;
      }
      else {
        if (bVar1 != 0x20) {
          return;
        }
        name = (LPCSTR)0x7f02;
      }
    }
  }
  HVar2 = LoadIcon16(param_2,name);
  *(HICON16 *)((int)param_1 + 0x8e) = HVar2;
  return;
}



HANDLE16 __stdcall16far
create_window_1040_8bea(UINT32 param_1,undefined2 param_2,int param_3,INT16 param_4,HMENU16 param_5)

{
  HANDLE16 HVar1;
  LPCSTR unaff_CS;
  LRESULT LVar2;
  HWND16 in_stack_0000000e;
  ulong uStack6;
  
  uStack6 = 0x50010000;
  if (param_3 != 0x0) {
    uStack6 = 0x50010001;
  }
  if (*(int *)(param_1 + 0x74) != 0x0) {
    uStack6 = uStack6 | 0x8000000;
  }
  CreateWindow16(unaff_CS,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_4,*(INT16 *)(param_1 + 0x6),0x17,0x58,
                 in_stack_0000000e,param_5,(HINSTANCE16)uStack6,(LPVOID)(uStack6 >> 0x10));
  HVar1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e09);
  if (HVar1 != 0x0) {
    LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,CONCAT22(0x30,HVar1));
    HVar1 = (HANDLE16)LVar2;
  }
  return HVar1;
}



void __stdcall16far get_sys_metrics_1040_8c66(astruct_37 *param_1,HWND16 param_2)

{
  int *piVar1;
  byte bVar2;
  HDC16 hdc;
  INT16 IVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  hdc = GetDC16(param_2);
  draw_text_1040_8d14(param_1,(int)s_tile2_bmp_1050_1538);
  *(undefined4 *)(iVar4 + 0xa6) = *(undefined4 *)(iVar4 + 0x9e);
  *(undefined4 *)(iVar4 + 0xaa) = *(undefined4 *)(iVar4 + 0xa2);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  piVar1 = (int *)(iVar4 + 0xac);
  *piVar1 = *piVar1 + IVar3;
  bVar2 = *(byte *)(iVar4 + 0x98) & 0xf0;
  if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
    IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    if (*(int *)(iVar4 + 0xac) < IVar3) {
      IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
      *(INT16 *)(iVar4 + 0xac) = IVar3;
    }
  }
  piVar1 = (int *)(iVar4 + 0xaa);
  *piVar1 = *piVar1 + 0x14;
  piVar1 = (int *)(iVar4 + 0xac);
  *piVar1 = *piVar1 + 0xa;
  *(undefined2 *)(iVar4 + 0xb0) = *(undefined2 *)(iVar4 + 0xac);
  piVar1 = (int *)(iVar4 + 0xac);
  *piVar1 = *piVar1 + 0x30;
  ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,hdc);
  return;
}



void __stdcall16far draw_text_1040_8d14(astruct_37 *param_1,HWND16 param_2)

{
  byte bVar1;
  INT16 IVar2;
  HANDLE16 handle;
  astruct_37 *iVar3;
  RECT16 *rect;
  HGDIOBJ16 HStack8;
  
  rect = (RECT16 *)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_37 *)param_1;
  bVar1 = iVar3->field_0x98 & 0xf0;
  if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20)) {
    iVar3->field_0xa0 = 0xa;
    IVar2 = GetSystemMetrics16(param_2);
    iVar3->field_0x9e = IVar2 + 0x28;
    param_2 = (HWND16)s_tile2_bmp_1050_1538;
  }
  else {
    iVar3->field_0xa0 = 0xa;
    iVar3->field_0x9e = 0x14;
  }
  HStack8 = 0x0;
  handle = GetProp16(param_2,(LPCSTR)0x5e0f);
  if (handle != 0x0) {
    HStack8 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  }
  DrawText16((HDC16)s_tile2_bmp_1050_1538,(LPCSTR)0x410,(INT16)&iVar3->field_0x9e,rect,0xffff);
  if (HStack8 != 0x0) {
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack8);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1040_8db6(astruct_18 *param_1,byte param_2)

{
  pass1_1040_869a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1040_8e58(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(ushort)(param_4 >> 0x10));
  *(ushort *)CONCAT22(param_2,param_1) = 0x8f3c;
  *(undefined2 *)(param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1040_8e82(astruct_18 *param_1)

{
  param_1->field_0x0 = 0x8f3c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



void __stdcall16far enable_window_1040_8ea0(ushort param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  HWND16 enable;
  uchar *in_DX;
  ushort unaff_SS;
  
  if (param_4._2_2_ == 0xf8) {
    GetDlgItem16(param_5,0x17d8);
    enable = 0x1;
  }
  else {
    if (param_4._2_2_ != 0x17d8) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
      return;
    }
    SetWindowPos16(param_5,0x6,0xf6,0x269,0x0,0x0,0x0);
    enable = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x17d8);
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,enable);
  return;
}



astruct_18 * __stdcall16far pass1_1040_8f16(astruct_18 *param_1,byte param_2)

{
  pass1_1040_8e82(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
mixed_struct_op_1040_8fb8
          (ushort *param_1,ushort param_2,char *param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7,
          ushort param_8,ushort param_9,LPVOID param_10,ushort param_11)

{
  undefined2 uVar1;
  ushort uVar2;
  LPVOID pvVar3;
  int iVar4;
  undefined2 uVar5;
  astruct_43 *paVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  *(undefined4 *)(iVar4 + 0x4) = 0x0;
  *(undefined4 *)(iVar4 + 0x8) = 0x0;
  *(undefined4 *)(iVar4 + 0xc) = 0x0;
  *(undefined4 *)(iVar4 + 0x10) = 0x0;
  *(undefined4 *)(iVar4 + 0x14) = 0x0;
  *(undefined2 *)(iVar4 + 0x18) = 0x0;
  *(ushort *)(iVar4 + 0x1a) = param_8;
  *(ushort *)(iVar4 + 0x1c) = param_7;
  *(undefined2 *)(iVar4 + 0x36) = 0x5;
  *(undefined2 *)(iVar4 + 0x38) = 0x0;
  *(undefined2 *)(iVar4 + 0x3a) = 0x0;
  *(undefined2 *)(iVar4 + 0x3c) = 0x2;
  *(undefined2 *)(iVar4 + 0x3e) = 0x0;
  *(ushort *)(iVar4 + 0x40) = param_2;
  *param_1 = 0x9800;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1040;
  uVar1 = *(undefined2 *)(iVar4 + 0x36);
  *(undefined2 *)(iVar4 + 0x28) = uVar1;
  *(undefined2 *)(iVar4 + 0x26) = uVar1;
  *(undefined2 *)(iVar4 + 0x2c) = 0x0;
  *(undefined2 *)(iVar4 + 0x2a) = 0x0;
  if ((param_6 != 0x0) && (param_5 != 0x0)) {
    *(undefined2 *)(iVar4 + 0x38) = 0x1;
    paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_6,param_11);
    *(undefined2 *)(iVar4 + 0x8) = (int)paVar6;
    *(undefined2 *)(iVar4 + 0xa) = (int)((ulong)paVar6 >> 0x10);
    param_10 = (LPVOID)0x1010;
    paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_5,param_11);
    param_9 = (ushort)((ulong)paVar6 >> 0x10);
    *(undefined2 *)(iVar4 + 0xc) = (int)paVar6;
    *(ushort *)(iVar4 + 0xe) = param_9;
    if (param_4 == 0x0) {
      *(undefined4 *)(iVar4 + 0x10) = 0x0;
    }
    else {
      param_10 = (LPVOID)0x1010;
      paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_4,param_11);
      param_9 = (ushort)((ulong)paVar6 >> 0x10);
      *(undefined2 *)(iVar4 + 0x10) = (int)paVar6;
      *(ushort *)(iVar4 + 0x12) = param_9;
    }
  }
  uVar1 = *(undefined2 *)(iVar4 + 0x36);
  *(undefined2 *)(iVar4 + 0x30) = uVar1;
  *(undefined2 *)(iVar4 + 0x2e) = uVar1;
  *(undefined4 *)(iVar4 + 0x32) = 0x0;
  if (param_3 != (char *)0x0) {
    param_10 = (LPVOID)0x1008;
    uVar2 = str_op_1008_60e8(param_3,param_9);
    *(ushort *)(iVar4 + 0x4) = uVar2;
    *(ushort *)(iVar4 + 0x6) = param_9;
  }
  *(undefined4 *)(iVar4 + 0x22) = 0x0;
  *(undefined2 *)(iVar4 + 0x1e) = 0x0;
  *(undefined2 *)(iVar4 + 0x20) = 0x0;
  if (_PTR_LOOP_1050_5e18 == 0x0) {
    pvVar3 = MakeProcInstance16(param_10,(HANDLE16)PTR_LOOP_1050_038c);
    _PTR_LOOP_1050_5e18 = CONCAT22(param_9,pvVar3);
  }
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mix_win_ui_op_1040_911e(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 uVar3;
  code **ppcVar4;
  int iVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = 0x9800;
  *(undefined2 *)(iVar5 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (*(int *)(iVar5 + 0x38) != 0x0) {
    puVar1 = (undefined4 *)*(uint *)(iVar5 + 0x8);
    uVar2 = *(uint *)(iVar5 + 0xa);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
    puVar1 = (undefined4 *)*(uint *)(iVar5 + 0xc);
    uVar2 = *(uint *)(iVar5 + 0xe);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
    puVar1 = (undefined4 *)*(uint *)(iVar5 + 0x10);
    uVar2 = *(uint *)(iVar5 + 0x12);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar5 + 0x4),0x1000);
  uVar3 = *(undefined4 *)(iVar5 + 0x14);
  SetWindowLong16(0x1000,(INT16)uVar3,CONCAT22(0xfffc,(int)((ulong)uVar3 >> 0x10)));
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e1c);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5e23);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5e2a);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procHi_1050_5e31);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e38);
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -0x1;
  if (PTR_LOOP_1050_5e16 == (undefined *)0x0) {
    FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
    _PTR_LOOP_1050_5e18 = 0x0;
  }
  *param_1 = 0x389a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  return;
}



void __stdcall16far enable_win_1040_9234(ulong param_1,BOOL16 param_2,HWND16 param_3)

{
  if (*(int *)((int)param_1 + 0x18) != 0x0) {
    EnableWindow16(param_3,param_2);
  }
  return;
}



void __stdcall16far pass1_1040_9252(ulong param_1,ushort param_2)

{
  int *piVar1;
  int iVar2;
  astruct_161 *iVar3;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar3 = (astruct_161 *)param_1;
  if (iVar3->field_0x4 != 0x0) {
    draw_text_1040_9650(param_1 & 0xffff | (ulong)uVar3 << 0x10,param_2);
  }
  if (iVar3->field_0x8 != 0x0) {
    pass1_1040_9618(param_1 & 0xffff | (ulong)uVar3 << 0x10);
  }
  iVar2 = iVar3->field_0x32;
  if (iVar3->field_0x22 < iVar2) {
    iVar3->field_0x22 = iVar2;
  }
  iVar2 = iVar3->field_0x34;
  if (iVar3->field_0x24 < iVar2) {
    iVar3->field_0x24 = iVar2;
  }
  iVar2 = iVar3->field_0x26 + iVar3->field_0x2a;
  if (iVar3->field_0x22 < iVar2) {
    iVar3->field_0x22 = iVar2;
  }
  iVar2 = iVar3->field_0x28 + iVar3->field_0x2c;
  if (iVar3->field_0x24 < iVar2) {
    iVar3->field_0x24 = iVar2;
  }
  piVar1 = &iVar3->field_0x22;
  *piVar1 = *piVar1 + iVar3->field_0x36;
  piVar1 = &iVar3->field_0x24;
  *piVar1 = *piVar1 + iVar3->field_0x36;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far create_window_1040_92dc(undefined4 param_1,UINT16 param_2)

{
  HWND16 HVar1;
  LPCSTR str;
  LPCSTR str_00;
  LPCSTR str_01;
  long lVar2;
  
  str_01 = (LPCSTR)((ulong)param_1 >> 0x10);
  str_00 = (LPCSTR)param_1;
  if (*(int *)(str_00 + 0x18) == 0x0) {
    HVar1 = CreateWindow16((LPCSTR)param_2,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,*(INT16 *)(str_00 + 0x1c),
                           *(INT16 *)(str_00 + 0x1a),0x0,0x0,*(HWND16 *)(str_00 + 0x20),*(HMENU16 *)(str_00 + 0x1e),0xb,
                           (LPVOID)0x4000);
    *(HWND16 *)(str_00 + 0x18) = HVar1;
    lVar2 = SetWindowLong16((HWND16)s_tile2_bmp_1050_1538,(INT16)_PTR_LOOP_1050_5e18,
                            CONCAT22(0xfffc,(int)((ulong)_PTR_LOOP_1050_5e18 >> 0x10)));
    str = (LPCSTR)((ulong)lVar2 >> 0x10);
    *(int *)(str_00 + 0x14) = (int)lVar2;
    *(LPCSTR *)(str_00 + 0x16) = str;
    SetProp16((HWND16)s_tile2_bmp_1050_1538,str,(HANDLE16)s_procHi_1050_5e46);
    SetProp16((HWND16)s_tile2_bmp_1050_1538,*(LPCSTR *)(str_00 + 0x14),(HANDLE16)s_procLo_1050_5e4d);
    SetProp16((HWND16)s_tile2_bmp_1050_1538,str_01,(HANDLE16)s_thisHi_1050_5e54);
    SetProp16((HWND16)s_tile2_bmp_1050_1538,str_00,(HANDLE16)s_thisLo_1050_5e5b);
    if (*(int *)(str_00 + 0x40) != 0x0) {
      SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)((int)&PTR_LOOP_1050_0000 + 0x1),0x5e62);
    }
    ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  }
  return;
}



void __stdcall16far mov_update_win_1040_93aa(astruct_65 *param_1,INT16 param_2,ushort param_3,HWND16 param_4)

{
  astruct_65 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_65 *)param_1;
  iVar1->field_0x1e = param_3;
  iVar1->field_0x20 = param_2;
  MoveWindow16(param_4,0x1,iVar1->field_0x24,iVar1->field_0x22,param_2,iVar1->field_0x1e);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



LRESULT __stdcall16far pass1_1040_93e6(ulong param_1,HWND16 param_2)

{
  LRESULT LVar1;
  
  LVar1 = SendMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)param_1 + 0x1c)));
  return LVar1;
}



LRESULT __stdcall16far send_msg_1040_9404(ulong param_1,HWND16 param_2)

{
  LRESULT LVar1;
  
  LVar1 = SendMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)param_1 + 0x1c)));
  return LVar1;
}



void __stdcall16far pass1_1040_9422(ulong *param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x8) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x10);
    (**ppcVar1)();
  }
  if (*(long *)((int)param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
  }
  return;
}



void __stdcall16far unk_draw_op_1040_9458(astruct_17 *param_1,byte param_2,undefined2 param_3,HDC16 param_4)

{
  code **ppcVar1;
  ULONG UVar2;
  undefined2 *b_force_background;
  UINT16 uVar3;
  astruct_17 *iVar4;
  UINT16 uVar4;
  UINT16 *puStack8;
  ULONG *puStack6;
  
  uVar4 = (UINT16)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_17 *)param_1;
  if (iVar4->field_0x8 != (undefined4 *)0x0) {
    puStack6 = iVar4->field_0x8;
    uVar3 = *(UINT16 *)((int)&iVar4->field_0x8 + 0x2);
    if (((*(uint *)((int)&iVar4->field_0xc + 0x2) | *(uint *)&iVar4->field_0xc) != 0x0) && ((param_2 & 0x1) != 0x0)) {
      puStack6 = iVar4->field_0xc;
      uVar3 = *(UINT16 *)((int)&iVar4->field_0xc + 0x2);
    }
    if ((iVar4->field_0x10 != (undefined4 *)0x0) && ((param_2 & 0x4) != 0x0)) {
      puStack6 = iVar4->field_0x10;
      uVar3 = *(UINT16 *)((int)&iVar4->field_0x10 + 0x2);
    }
    b_force_background = &param_3;
    UVar2 = *puStack6;
    ppcVar1 = (code **)((int)UVar2 + 0x8);
    (**ppcVar1)(param_4,(int)puStack6,uVar3,b_force_background);
    ppcVar1 = (code **)((int)UVar2 + 0x4);
    (**ppcVar1)(param_4,puStack6,iVar4->field_0x28,iVar4->field_0x26,&param_3);
    SelectPalette16(param_4,0x0,(BOOL16)b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  }
  return;
}



void __stdcall16far draw_text_1040_94fc(astruct_37 *param_1,HDC16 param_2)

{
  COLORREF color;
  COLORREF color_00;
  astruct_38 *iVar1;
  RECT16 *rect;
  
  rect = (RECT16 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_38 *)param_1;
  color = SetBkColor16(param_2,iVar1->field_0x3a);
  color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538,iVar1->field_0x3c);
  DrawText16((HDC16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,&iVar1->field_0x2e,rect,0xffff);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,color);
  SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color_00);
  return;
}



void __cdecl16far win_ui_get_prop_op_1040_9566(int *param_1,HWND16 param_2)

{
  undefined2 uVar1;
  int iVar2;
  code **ppcVar3;
  HANDLE16 HVar4;
  HANDLE16 HVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined4 *puStack12;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*param_1 == 0x4) {
    uVar1 = *(undefined2 *)(iVar6 + 0xc);
    uVar9 = *(undefined2 *)(iVar6 + 0xa);
    HVar4 = GetProp16(param_2,(LPCSTR)s_thisHi_1050_5e6f);
    uVar8 = *(undefined2 *)(iVar6 + 0xa);
    HVar5 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e68);
    puStack12 = (undefined4 *)CONCAT22(HVar4,HVar5);
    if ((HVar4 | HVar5) != 0x0) {
      iVar2 = *(int *)(iVar6 + 0x6);
      if (iVar2 == 0x1) {
        ppcVar3 = (code **)((int)*puStack12 + 0xc);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,*(undefined2 *)(iVar6 + 0x8),uVar1,uVar8,uVar9);
        return;
      }
      if (iVar2 == 0x2) {
        ppcVar3 = (code **)((int)*puStack12 + 0x10);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,*(undefined2 *)(iVar6 + 0x8),uVar1);
        return;
      }
      if (iVar2 == 0x4) {
        ppcVar3 = (code **)((int)*puStack12 + 0x18);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,*(byte *)(iVar6 + 0x8) & 0x10,uVar1);
        return;
      }
    }
  }
  return;
}



void __stdcall16far pass1_1040_9618(ulong param_1)

{
  undefined2 uVar1;
  astruct_162 *iVar2;
  undefined2 uVar2;
  ulong uVar3;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_162 *)param_1;
  uVar3 = pass1_1008_4772(iVar2->field_0x8);
  uVar1 = (undefined2)(uVar3 >> 0x10);
  iVar2->field_0x2a = *(undefined2 *)((int)uVar3 + 0x4);
  iVar2->field_0x2c = *(undefined2 *)((int)uVar3 + 0x8);
  return;
}



void __stdcall16far draw_text_1040_9650(ULONG param_1,HWND16 param_2)

{
  HDC16 hdc;
  
  hdc = GetDC16(param_2);
  DrawText16((HDC16)s_tile2_bmp_1050_1538,(LPCSTR)0x410,(int)param_1 + 0x2e,(RECT16 *)(param_1 >> 0x10),0xffff);
  ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,hdc);
  return;
}



// WARNING: Unable to use type for symbol var11
// WARNING: Unable to use type for symbol var7
// WARNING: Unable to use type for symbol var8

void __stdcall16far
call_win_proc_1040_9684
          (HWND16 win_handle_1,u16 param_2,WPARAM16 w_param_1,LPARAM l_param_1,HWND16 win_handle_2,u16 param_6)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  BOOL16 bool_1;
  RECT16 local_1a [0x2];
  u32 *var18;
  u32 *var14;
  u32 *var10;
  i32 var6;
  u32 var2;
  u16 var4;
  undefined2 var11;
  undefined2 var7;
  undefined2 var8;
  u16 var9;
  code **fn_ptr_1;
  RECT16 *rect_1;
  undefined4 var3;
  undefined2 var5;
  
  var9 = (u16)&USHORT_1050_1050;
  var8 = l_param_1._2_2_;
  handle_1 = GetProp16(win_handle_2,(LPCSTR)s_procHi_1050_5e7d);
  var7 = l_param_1._2_2_;
  handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5e76);
  var6 = CONCAT22(handle_1,handle_2);
  var11 = l_param_1._2_2_;
  handle_1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5e8b);
  var5 = l_param_1._2_2_;
  handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e84);
  var10 = (u32 *)CONCAT22(handle_1,handle_2);
  if ((handle_1 | handle_2) != 0x0) {
    if ((int)l_param_1 == 0x2) {
      var18 = var10;
      var14 = var10;
      if (var10 != (u32 *)0x0) {
        fn_ptr_1 = (code **)*var10;
        (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,handle_2,handle_1,0x1,var5,var11,var7,var8,var9);
      }
    }
    else {
      if ((int)l_param_1 == 0x201) {
        handle_1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e92);
        if (handle_1 == 0x0) {
          var5 = *(undefined2 *)((int)var10 + 0x18);
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_1a);
          rect_1 = local_1a;
          var2 = CONCAT22(var5,param_6);
          bool_1 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,
                              (POINT16)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,win_handle_1)));
          if (bool_1 == 0x0) {
            return;
          }
          debug_print_1008_6048(CONCAT22(param_6,0x5e98),0x1008,param_6);
          fn_ptr_1 = (code **)((int)*var10 + 0x1c);
          (**fn_ptr_1)(0x1008,(int)var10,(int)((ulong)var10 >> 0x10),param_2,win_handle_1,(char)w_param_1,rect_1,var2,
                       l_param_1._2_2_);
          return;
        }
      }
      else {
        if ((int)l_param_1 == 0x204) {
          var4 = *(u16 *)(handle_2 + 0x18);
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_1a);
          var3 = CONCAT22(param_6,local_1a);
          bool_1 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,(POINT16)CONCAT22(param_2,win_handle_1));
          if (bool_1 == 0x0) {
            return;
          }
          debug_print_1008_6048(CONCAT22(param_6,0x5eab),0x1008,param_6);
          fn_ptr_1 = (code **)((int)*var10 + 0x20);
          (**fn_ptr_1)(0x1008,(int)var10,(int)((ulong)var10 >> 0x10),param_2,(char)win_handle_1,w_param_1,var3,var4);
          return;
        }
      }
    }
  }
  if (var6 != 0x0) {
    CallWindowProc16((LPVOID)s_tile2_bmp_1050_1538,win_handle_1,param_2,w_param_1,l_param_1);
  }
  return;
}



ushort * __stdcall16far pass1_1040_97da(ushort *param_1,byte param_2)

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_9824(ulong *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x0;
  *(undefined2 *)(iVar1 + 0x4) = 0x0;
  *(undefined4 *)(iVar1 + 0x56) = 0x0;
  *(undefined2 *)(iVar1 + 0x5a) = 0x0;
  *(undefined2 *)(iVar1 + 0x5c) = 0x0;
  *(undefined *)(iVar1 + 0x6) = 0x0;
  return;
}
