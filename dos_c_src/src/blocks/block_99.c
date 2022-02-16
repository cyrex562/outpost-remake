


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
enable_window_1038_9cec(int param_1,ushort param_2,ushort param_3,ushort param_4,int param_5,HWND16 param_6)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  int iVar4;
  HWND16 HVar5;
  uchar *in_DX;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar6;
  int iStack12;
  
  if (param_5 == 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(0xeb,param_4),in_DX,(ushort)&PTR_LOOP_1050_1040,unaff_SS);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
    iVar4 = (int)puVar6 + 0xa4;
    uVar3 = (undefined2)((ulong)puVar6 >> 0x10);
    iStack12 = 0x0;
    HVar5 = 0x1010;
    while (iVar2 = iStack12 * 0x2, *(int *)(iVar2 + iVar4) != 0x0) {
      HVar5 = GetDlgItem16(HVar5,*(INT16 *)(iVar2 + iVar4));
      *(HWND16 *)(param_1 + iVar2 + 0x94) = HVar5;
      iStack12 = iStack12 + 0x1;
      piVar1 = (int *)(param_1 + 0x128);
      *piVar1 = *piVar1 + 0x1;
      HVar5 = (HWND16)s_tile2_bmp_1050_1538;
    }
  }
  else {
    if (param_5 == 0xf8) {
      GetDlgItem16(param_6,0x17d8);
      HVar5 = 0x1;
    }
    else {
      if (param_5 != 0x17d8) {
        pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(param_5,param_4),in_DX,(ushort)&PTR_LOOP_1050_1040,unaff_SS);
        return;
      }
      SetWindowPos16(param_6,0x6,0xed,0x237,0x0,0x0,0x0);
      HVar5 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x17d8);
    }
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,HVar5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
draw_op_1038_9dcc(astruct_10 *in_struct_1,int param_2,uint param_3,COLORREF in_colorref_4,UINT16 param_5)

{
  uint *puVar1;
  bool bVar2;
  undefined2 uVar3;
  int iVar4;
  HBRUSH16 local_brush_handle;
  ULONG uVar5;
  UINT16 extraout_DX;
  astruct_10 *local_struct_5;
  astruct_10 *var5;
  COLORREF hdc;
  ulong uVar6;
  uint uStack14;
  
  var5 = (astruct_10 *)((ulong)in_struct_1 >> 0x10);
  local_struct_5 = (astruct_10 *)in_struct_1;
  hdc = in_colorref_4;
  if (local_struct_5->brush_handle_field_0x8e == 0x0) {
    hdc = (COLORREF)s_tile2_bmp_1050_1538;
    local_brush_handle = CreateSolidBrush16(in_colorref_4);
    local_struct_5->brush_handle_field_0x8e = local_brush_handle;
  }
  if (_PTR_LOOP_1050_5b64 == 0x0) {
    hdc = 0x1008;
    uVar6 = pass1_1008_4d72(*(ulong *)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar3 = (undefined2)(uVar6 >> 0x10);
    iVar4 = (int)uVar6;
    _PTR_LOOP_1050_5b64 =
         (ulong)CONCAT12(*(undefined *)(iVar4 + 0x94),
                         CONCAT11(*(undefined *)(iVar4 + 0x95),*(undefined *)(iVar4 + 0x96)));
    PTR_LOOP_1050_5b68 = (undefined *)CONCAT11(*(undefined *)(iVar4 + 0x3e5),*(undefined *)(iVar4 + 0x3e6));
    PTR_LOOP_1050_5b6a = (undefined *)(uint)*(byte *)(iVar4 + 0x3e4);
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar2 = false;
    for (uStack14 = 0x0; puVar1 = &local_struct_5->field_0x128, uStack14 <= *puVar1 && *puVar1 != uStack14;
        uStack14 = uStack14 + 0x1) {
      if (*(int *)(&local_struct_5->field_0x94 + uStack14 * 0x2) == param_2) {
        bVar2 = true;
        break;
      }
    }
    if (bVar2) {
      PTR_LOOP_1050_5b64 = PTR_LOOP_1050_5b68;
    }
  }
  SetTextColor16(hdc,(COLORREF)PTR_LOOP_1050_5b64);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return;
}



astruct_18 * __stdcall16far pass1_1038_9ed4(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far
pass1_1038_9f76(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfba,param_5);
  *(undefined2 *)param_1 = 0xa0b6;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_9fa4(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xa0b6;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far show_win_1038_9fd0(astruct_1 *param_1)

{
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far call_fn_ptr_1038_9ffa(HWND16 win_handle,ushort param_2,astruct_733 *struct_1,ushort param_4)

{
  code **ppcVar1;
  astruct_43 *var_2;
  astruct_43 *var_3;
  HDC16 dev_ctx;
  u16 var_5;
  
  var_5 = struct_1->field_0x6;
  dev_ctx = GetDC16(win_handle);
  var_3 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x3,param_2);
  var_2 = (astruct_43 *)*(undefined4 *)var_3;
  ppcVar1 = (code **)&var_2->fn_ptr_field_0x8;
  (**ppcVar1)(0x1010,(int)var_3,(int)((ulong)var_3 >> 0x10),&dev_ctx,param_2,var_5);
  ppcVar1 = (code **)&var_2->fn_ptr_field_0x4;
  (**ppcVar1)(0x1010,var_3,0x50005,&dev_ctx,param_2);
  ppcVar1 = (code **)&var_2->fn_ptr_field_0xc;
  (**ppcVar1)(0x1010,var_3,&dev_ctx,param_2);
  ReleaseDC16(0x1010,dev_ctx);
  return 0x0;
}



void __stdcall16far destroy_window_1038_a072(ushort param_1,ushort param_2,int param_3,HWND16 param_4)

{
  if (param_3 != 0x0) {
    DestroyWindow16(param_4);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_a090(astruct_18 *param_1,byte param_2)

{
  pass1_1038_9fa4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1038_a122(int param_1,ushort param_2,ushort param_3,ulong param_4,ulong param_5)

{
  get_sys_metrics_1040_7728
            ((astruct_57 *)CONCAT22(param_2,param_1),param_3,param_4,(ushort)param_5,(ushort)(param_5 >> 0x10));
  *(undefined2 *)(param_1 + 0x8e) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xa2d0;
  *(undefined2 *)(param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1038_a156(astruct_18 *param_1)

{
  param_1->field_0x0 = 0xa2d0;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far pass1_1038_a174(ulong param_1,int param_2)

{
  if (param_2 == 0x1) {
    *(undefined2 *)((int)param_1 + 0x8e) = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1038_a18c(astruct_1 *param_1,ushort param_2)

{
  code **ppcVar1;
  INT16 IVar2;
  uchar *in_DX;
  int unaff_DI;
  undefined2 uVar3;
  int *piVar4;
  undefined *puVar5;
  undefined2 uVar6;
  ushort uVar7;
  RECT16 local_2c;
  int iStack40;
  ushort *puStack36;
  int iStack32;
  undefined2 uStack30;
  int local_1c;
  undefined local_1a [0x2];
  ulong uStack24;
  astruct_76 *paStack20;
  int local_10;
  BOOL16 local_e;
  undefined local_c [0x6];
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x27,param_2,in_DX,unaff_DI);
  pass1_1008_3e38((ushort *)CONCAT22(param_2,local_c));
  pass1_1008_3f62((ushort *)CONCAT22(param_2,local_c),
                  (ushort *)((ulong)puStack6 & 0xffff0000 | (ulong)((int)puStack6 + 0x52)));
  pass1_1008_3e94((ushort *)CONCAT22(param_2,local_c),(ushort *)CONCAT22(param_2,&local_10),
                  (ushort *)CONCAT22(param_2,&local_e));
  paStack20 = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1c0,param_2);
  uStack24 = pass1_1008_4772(paStack20);
  puVar5 = local_1a;
  piVar4 = &local_1c;
  uVar7 = param_2;
  puStack36 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,(uchar *)(uStack24 >> 0x10),unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puStack36 & 0xffff0000 | (ulong)((int)puStack36 + 0xe)),
                  (ushort *)CONCAT22(param_2,piVar4),(ushort *)CONCAT22(uVar7,puVar5));
  uVar3 = (undefined2)((ulong)puStack36 >> 0x10);
  uStack30 = *(undefined2 *)((int)puStack36 + 0xa);
  iStack32 = *(int *)((int)puStack36 + 0xc);
  local_10 = local_10 + (iStack32 * 0xa) / 0x258 + *(int *)((int)uStack24 + 0x8) + local_1c;
  uVar3 = *(undefined2 *)((int)param_1 + 0x6);
  GetWindowRect16(0x1008,&local_2c);
  uVar6 = 0x0;
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  local_e = (IVar2 - (iStack40 - local_2c.x)) / 0x2;
  move_win_1040_826c(param_1,local_10,local_e);
  if (paStack20 != (astruct_76 *)0x0) {
    ppcVar1 = (code **)*(undefined4 *)paStack20;
    (**ppcVar1)((int)&PTR_LOOP_1050_1040,(int)paStack20,(int)((ulong)paStack20 >> 0x10),0x1,uVar6,uVar3,paStack20,
                paStack20);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_a2aa(astruct_18 *param_1,byte param_2)

{
  pass1_1038_a156(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1038_a33c(ushort *param_1,ushort param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1038_a122((int)param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfc7));
  *param_1 = 0xa428;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_a36a(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xa428;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far show_win_1038_a396(astruct_1 *param_1,ushort param_2,ushort param_3)

{
  ushort in_AX;
  ushort in_DX;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  unk_win_ui_op_1038_a18c(param_1,param_3);
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x10001,(WNDCLASS16 *)param_3,in_AX,in_DX);
  *(ushort *)((int)param_1 + 0x8c) = in_AX;
  ShowWindow16(0x1008,0x5);
  return;
}



void __stdcall16far destroy_win_1038_a3d2(ULONG param_1,HWND16 param_2)

{
  GetWindowWord16(param_2,-0x8);
  PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
  destroy_win_1040_7b98(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



astruct_18 * __stdcall16far pass1_1038_a402(astruct_18 *param_1,byte param_2)

{
  pass1_1038_a36a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1038_a494(ushort *param_1,ushort param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1038_a122((int)param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfc8));
  *param_1 = 0xa62e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_a4c2(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xa62e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_a4ee(astruct_1 *param_1,ushort param_2)

{
  undefined4 uVar1;
  uchar *in_DX;
  int unaff_DI;
  WNDCLASS16 *unaff_SS;
  ushort *puVar2;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x20001,unaff_SS,param_2,(ushort)in_DX);
  *(ushort *)((int)param_1 + 0x8c) = param_2;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)unaff_SS,in_DX,unaff_DI);
  uVar1 = *(undefined4 *)((int)puVar2 + 0x6c);
  GetDlgItem16(0x1010,0x114);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)uVar1);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  unk_win_ui_op_1038_a18c(param_1,unaff_SS);
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_a584(ushort param_1,int param_2,HWND16 param_3,ushort param_4)

{
  uint uVar1;
  uchar *in_DX;
  int unaff_DI;
  ushort *puVar2;
  undefined2 in_stack_00000006;
  undefined *puVar3;
  undefined local_52 [0x50];
  
  if (param_2 != 0x0) {
    GetDlgItem16(param_3,0x114);
    GetWindowText16((HWND16)s_tile2_bmp_1050_1538,0x50,(INT16)local_52);
    uVar1 = str_op_1000_3da4((char *)CONCAT22(param_4,local_52));
    if (uVar1 != 0x0) {
      puVar3 = local_52;
      puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,in_DX,unaff_DI);
      pass1_1010_6006((ulong)puVar2,(char *)CONCAT22(param_4,puVar3),(ushort)((ulong)puVar2 >> 0x10));
      GetWindowWord16(0x1010,-0x8);
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
      destroy_win_1040_7b98(CONCAT22(in_stack_00000006,param_1),(int)&PTR_LOOP_1050_1040);
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_a608(astruct_18 *param_1,byte param_2)

{
  pass1_1038_a4c2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1038_a69a(ushort *param_1,ushort param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1038_a122((int)param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfc9));
  *param_1 = 0xa832;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_a6c8(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xa832;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_a6f4(astruct_1 *param_1)

{
  undefined4 uVar1;
  ushort uVar2;
  uchar *in_DX;
  ushort uVar3;
  int unaff_DI;
  WNDCLASS16 *unaff_SS;
  ushort *puVar4;
  LRESULT LVar5;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)unaff_SS,in_DX,unaff_DI);
  uVar1 = *(undefined4 *)((int)puVar4 + 0x68);
  GetDlgItem16(0x1010,0x115);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)uVar1);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  LVar5 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  uVar3 = (ushort)((ulong)LVar5 >> 0x10);
  uVar2 = (ushort)LVar5;
  unk_win_ui_op_1038_a18c(param_1,unaff_SS);
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x30001,unaff_SS,uVar2,uVar3);
  *(ushort *)((int)param_1 + 0x8c) = uVar2;
  ShowWindow16(0x1008,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_a788(ulong param_1,int param_2,HWND16 param_3,ushort param_4)

{
  uint uVar1;
  uchar *in_DX;
  int unaff_DI;
  ushort *pUVar2;
  undefined *puVar2;
  undefined local_52 [0x50];
  uchar *puVar3;
  
  if (param_2 != 0x0) {
    GetDlgItem16(param_3,0x115);
    GetWindowText16((HWND16)s_tile2_bmp_1050_1538,0x50,(INT16)local_52);
    uVar1 = str_op_1000_3da4((char *)CONCAT22(param_4,local_52));
    if (uVar1 != 0x0) {
      puVar2 = local_52;
      pUVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,in_DX,unaff_DI);
      pass1_1010_5fd8((ulong)pUVar2,(char *)CONCAT22(param_4,puVar2),(ushort)((ulong)pUVar2 >> 0x10));
      GetWindowWord16(0x1010,-0x8);
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
      destroy_win_1040_7b98(param_1,(int)&PTR_LOOP_1050_1040);
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_a80c(astruct_18 *param_1,byte param_2)

{
  pass1_1038_a6c8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1038_a89e(ushort *param_1,ushort param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1038_a122((int)param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfca));
  *param_1 = 0xab16;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_a8cc(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xab16;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



void __stdcall16far enable_win_1038_a8f8(ushort param_1,ushort param_2,ushort param_3,TwoWords param_4,HWND16 in_hwnd_5)

{
  BOOL16 enable;
  
  if (param_4.b_0x2 == 0x116) {
    SendDlgItemMessage16(in_hwnd_5,0x0,0x0,0x1,0x11a0401);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x11a);
    enable = 0x0;
  }
  else {
    if ((param_4.b_0x2 == 0x116) || (0x2 < param_4.b_0x2 - 0x117)) {
      post_win_msg_1040_7b3c
                ((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4.b_0x2,(int)&PTR_LOOP_1050_1040);
      return;
    }
    GetDlgItem16(in_hwnd_5,0x11a);
    enable = 0x1;
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,enable);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_a972(astruct_1 *param_1)

{
  BOOL16 BVar1;
  ushort uVar2;
  WNDCLASS16 *unaff_SS;
  LRESULT LVar3;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  SendDlgItemMessage16((HWND16)&PTR_LOOP_1050_1040,0x0,0x0,0x1,0x1160401);
  LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x11a0401);
  uVar2 = (ushort)((ulong)LVar3 >> 0x10);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x11a);
  BVar1 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x40001,unaff_SS,BVar1,uVar2);
  *(BOOL16 *)((int)param_1 + 0x8c) = BVar1;
  unk_win_ui_op_1038_a18c(param_1,unaff_SS);
  ShowWindow16(0x1008,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_sys_op_1038_a9fa(ULONG param_1,int param_2)

{
  uchar *in_DX;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar1;
  LRESULT LVar2;
  
  if (param_2 != 0x0) {
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
    LVar2 = SendDlgItemMessage16(0x1010,0x0,0x0,0x0,0x1160400);
    if (LVar2 == 0x0) {
      LVar2 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1170400);
      if (LVar2 == 0x0) {
        LVar2 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1180400);
        if (LVar2 == 0x0) {
          LVar2 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1190400);
          if (LVar2 != 0x0) {
            PTR_LOOP_1050_13ae = (undefined *)&DAT_1050_0004;
          }
        }
        else {
          PTR_LOOP_1050_13ae = (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1);
        }
      }
      else {
        PTR_LOOP_1050_13ae = (undefined *)&PTR_LOOP_1050_0002;
      }
    }
    else {
      PTR_LOOP_1050_13ae = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
    LVar2 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x11a0400);
    *(undefined2 *)((int)puVar1 + 0x82) = (int)LVar2;
    GetWindowWord16((HWND16)s_tile2_bmp_1050_1538,-0x8);
    PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
    destroy_win_1040_7b98(param_1,(int)&PTR_LOOP_1050_1040);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_aaf0(astruct_18 *param_1,byte param_2)

{
  pass1_1038_a8cc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far pass1_1038_ab82(astruct_57 *param_1,ushort param_2)

{
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd3,param_2);
  *(undefined2 *)param_1 = 0xad72;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_abb0(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xad72;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far set_win_pos_1038_abdc(HWND16 param_1)

{
  RECT16 local_12 [0x2];
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  GetWindowRect16(param_1,&local_a);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfd7);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,local_12);
  iStack6 = iStack6 - local_a.x;
  iStack4 = (local_12[0].y - local_a.y) + -0x2;
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x6,iStack4,iStack6,0x0,0x0,0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1038_ac38(INT16 param_1,uint param_2)

{
  undefined2 uVar1;
  int iVar2;
  INT16 IVar3;
  ULONG uVar3;
  UINT16 extraout_DX;
  HWND16 hwnd;
  HWND16 hdc;
  ulong uVar5;
  COLORREF color;
  UCHAR uVar4;
  astruct_46 *iVar1;
  
  hwnd = (HWND16)s_tile2_bmp_1050_1538;
  GetStockObject16(param_1);
  if (_PTR_LOOP_1050_5b78 == 0x0) {
    hwnd = 0x1008;
    uVar5 = pass1_1008_4d72(*(ulong *)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (undefined2)(uVar5 >> 0x10);
    iVar2 = (int)uVar5;
    _PTR_LOOP_1050_5b6c =
         (ulong)CONCAT12(*(undefined *)(iVar2 + 0x3ec),
                         CONCAT11(*(undefined *)(iVar2 + 0x3ed),*(undefined *)(iVar2 + 0x3ee)));
    _PTR_LOOP_1050_5b70 =
         (ulong)CONCAT12(*(undefined *)(iVar2 + 0x3e4),
                         CONCAT11(*(undefined *)(iVar2 + 0x3e5),*(undefined *)(iVar2 + 0x3e6)));
    _PTR_LOOP_1050_5b74 =
         (ulong)CONCAT12(*(undefined *)(iVar2 + 0x3f8),
                         CONCAT11(*(undefined *)(iVar2 + 0x3f9),*(undefined *)(iVar2 + 0x3fa)));
    _PTR_LOOP_1050_5b78 =
         (ulong)CONCAT12(*(undefined *)(iVar2 + 0x94),
                         CONCAT11(*(undefined *)(iVar2 + 0x95),*(undefined *)(iVar2 + 0x96)));
  }
  if (param_2 < 0x4) {
LAB_1038_acf0:
    hdc = (HWND16)s_tile2_bmp_1050_1538;
    IVar3 = GetDlgCtrlID16(hwnd);
    if (IVar3 == 0xfd4) {
      color = (COLORREF)_PTR_LOOP_1050_5b70;
      goto LAB_1038_ad0e;
    }
    if (IVar3 != 0xfd5) {
      if (IVar3 == 0xfd6) {
        color = (COLORREF)_PTR_LOOP_1050_5b6c;
        goto LAB_1038_ad0e;
      }
      if (IVar3 == 0xfd7) {
        color = (COLORREF)_PTR_LOOP_1050_5b74;
        goto LAB_1038_ad0e;
      }
    }
  }
  else {
    hdc = hwnd;
    if (param_2 != 0x4) {
      if ((param_2 == 0x4) || (0x1 < param_2 - 0x5)) {
        return;
      }
      goto LAB_1038_acf0;
    }
  }
  color = (COLORREF)_PTR_LOOP_1050_5b78;
LAB_1038_ad0e:
  SetTextColor16(hdc,color);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return;
}



astruct_18 * __stdcall16far pass1_1038_ad4c(astruct_18 *param_1,byte param_2)

{
  pass1_1038_abb0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1038_adde(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  pass1_1038_9b72(param_1,param_2,param_3,param_4);
  *(ushort *)CONCAT22(param_2,param_1) = 0xae4e;
  *(undefined2 *)(param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1038_ae08(astruct_18 *param_1)

{
  param_1->field_0x0 = 0xae4e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



astruct_18 * __stdcall16far pass1_1038_ae28(astruct_18 *param_1,byte param_2)

{
  pass1_1038_ae08(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_20 * __stdcall16far pass1_1038_aeca(astruct_20 *param_1,UINT16 param_2)

{
  undefined2 uVar1;
  undefined2 local_b6;
  undefined2 uStack180;
  undefined local_5c [0x5a];
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0xac) = 0x0;
  *(undefined2 *)((int)param_1 + 0xae) = 0x0;
  if (_PTR_LOOP_1050_5b7c == (astruct_20 *)0x0) {
    _PTR_LOOP_1050_5b7c = param_1;
  }
  pass1_1000_4906(param_1,(WNDCLASS16 *)0x0,0xac);
  unk_draw_op_1008_80ee((astruct_23 *)CONCAT22(param_2,local_5c),param_2);
  unk_win_ui_op_1040_9854((ushort *)CONCAT22(param_2,&local_b6),param_2);
  local_b6 = 0x389a;
  uStack180 = 0x1008;
  pass1_1008_8168((ushort *)CONCAT22(param_2,local_5c));
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_af34(void)

{
  _PTR_LOOP_1050_5b7c = 0x0;
  return;
}



undefined4 __stdcall16far
pass1_1038_af40(ulong param_1,ushort param_2,int param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7)

{
  code **ppcVar1;
  undefined4 uVar2;
  undefined *puVar3;
  uchar *puVar4;
  uint uVar5;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  undefined2 uVar8;
  astruct_57 *paVar9;
  
  puVar3 = (undefined *)bring_win_to_top_1038_b72e(param_1,param_3,param_6);
  iVar6 = (int)param_1;
  uVar7 = (undefined2)(param_1 >> 0x10);
  if (puVar3 != (undefined *)0x0) goto LAB_1038_b61f;
  uVar8 = SUB42(&PTR_LOOP_1050_1038,0x0);
  PTR_LOOP_1050_5b82 = puVar3;
  switch(param_3) {
  case 0x1:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x8e,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) {
LAB_1038_afa0:
      uVar8 = 0x1000;
      paVar9 = (astruct_57 *)0x0;
    }
    else {
      paVar9 = pass1_1038_9f76((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2);
    }
    break;
  case 0x2:
    mem_op_1000_179c(0x96,(uchar *)param_4,0x1000);
    uVar5 = param_4 | param_5;
    if (uVar5 == 0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_181c((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,uVar5,param_7);
    paVar9 = (astruct_57 *)CONCAT22(uVar5,param_5);
    break;
  case 0x3:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_e99a((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,(uchar *)(param_4 | param_5),
                             param_7);
    break;
  case 0x4:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_c7b8((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,(uchar *)(param_4 | param_5),
                             param_7);
    break;
  case 0x5:
    mem_op_1000_179c(0x96,(uchar *)param_4,0x1000);
    uVar5 = param_4 | param_5;
    if (uVar5 == 0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_23ea((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,param_7,uVar5);
    paVar9 = (astruct_57 *)CONCAT22(uVar5,param_5);
    break;
  case 0x6:
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar9 = pass1_1040_06e8((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,(uchar *)(param_4 | param_5),
                             param_7);
    break;
  case 0x7:
    mem_op_1000_179c(0x9c,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_4068((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x8:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x9a,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    pass1_1038_b772((astruct_57 *)CONCAT22(param_4,param_5),puVar4,unaff_DI,param_7,param_2);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x9:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x8e,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_e140((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2);
    break;
  case 0xa:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x90,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = (astruct_57 *)pass1_1038_a33c((ushort *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0xb:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x90,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = (astruct_57 *)pass1_1038_a494((ushort *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0xc:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x90,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = (astruct_57 *)pass1_1038_a69a((ushort *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0xd:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x90,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = (astruct_57 *)pass1_1038_a89e((ushort *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0xe:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x94,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    pass1_1038_e69a((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0xf:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x94,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    pass1_1038_cd06((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x10:
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar9 = pass1_1040_0bfc((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,(uchar *)(param_4 | param_5),
                             unaff_DI,param_7);
    break;
  case 0x11:
    mem_op_1000_179c(0x9a,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_0e1c((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x12:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x9a,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_d756((astruct_57 *)CONCAT22(param_4,param_5),param_2,(uchar *)(param_4 | param_5),unaff_DI,
                             param_7);
    break;
  case 0x13:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_cad8((astruct_57 *)CONCAT22(param_4,param_5),param_2,(uchar *)(param_4 | param_5),unaff_DI,
                             param_7);
    break;
  case 0x14:
    mem_op_1000_179c(0xaa,(uchar *)param_4,0x1000);
    uVar5 = param_4 | param_5;
    if (uVar5 == 0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1f5a((astruct_57 *)CONCAT22(param_4,param_5),param_2,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(uVar5,param_5);
    break;
  case 0x15:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x8e,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_d242((astruct_57 *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0x16:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x9a,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    pass1_1038_eeda((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x17:
    mem_op_1000_179c(0x96,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    uVar8 = 0x1018;
    paVar9 = pass1_1018_5e26((astruct_57 *)CONCAT22(param_4,param_5),param_2);
    break;
  default:
    goto switchD_1038_b581_caseD_18;
  case 0x19:
    mem_op_1000_179c(0x96,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1cb4((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x1a:
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar9 = pass1_1040_123e((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,(uchar *)(param_4 | param_5),
                             unaff_DI,param_7);
    break;
  case 0x1b:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x8e,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_ab82((astruct_57 *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0x1c:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_e2d0((astruct_57 *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0x1d:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x92,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_eb9e((astruct_57 *)CONCAT22(param_4,param_5),param_2);
    break;
  case 0x1e:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x29e,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    pass1_1038_bddc((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x1f:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x9a,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    pass1_1038_c4a2((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x20:
    mem_op_1000_179c(0x29a,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_2ea2((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x21:
    mem_op_1000_179c(0xa6,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_3966((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x22:
    mem_op_1000_179c(0x9a,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_34a2((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x23:
    mem_op_1000_179c(0x9c,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ac84((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x25:
    mem_op_1000_179c(0xa0,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ca16((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x26:
    mem_op_1000_179c(0xa2,(uchar *)param_4,0x1000);
    uVar5 = param_4 | param_5;
    if (uVar5 == 0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_d0f8((astruct_57 *)CONCAT22(param_4,param_5),param_2);
    paVar9 = (astruct_57 *)CONCAT22(uVar5,param_5);
    break;
  case 0x27:
    uVar8 = 0x1000;
    mem_op_1000_179c(0xa0,(uchar *)param_4,0x1000);
    uVar5 = param_4 | param_5;
    if (uVar5 == 0x0) goto LAB_1038_afa0;
    pass1_1038_88f2((astruct_57 *)CONCAT22(param_4,param_5),param_2);
    paVar9 = (astruct_57 *)CONCAT22(uVar5,param_5);
    break;
  case 0x28:
    mem_op_1000_179c(0x96,(uchar *)param_4,0x1000);
    puVar4 = (uchar *)(param_4 | param_5);
    if (puVar4 == (uchar *)0x0) goto LAB_1038_afa0;
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_6402((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,param_7);
    paVar9 = (astruct_57 *)CONCAT22(puVar4,param_5);
    break;
  case 0x29:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x98,(uchar *)param_4,0x1000);
    if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_7d10((astruct_57 *)CONCAT22(param_4,param_5),param_2,param_4 | param_5,unaff_DI,param_7);
    break;
  case 0x2a:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x98,(uchar *)param_4,0x1000);
    if ((uchar *)(param_4 | param_5) == (uchar *)0x0) goto LAB_1038_afa0;
    paVar9 = pass1_1038_8caa((astruct_57 *)CONCAT22(param_4,param_5),param_2,(uchar *)(param_4 | param_5),unaff_DI,
                             param_7);
  }
  *(undefined2 *)(param_3 * 0x4 + iVar6) = (int)paVar9;
  *(undefined2 *)(param_3 * 0x4 + iVar6 + 0x2) = (int)((ulong)paVar9 >> 0x10);
switchD_1038_b581_caseD_18:
  if (*(long *)(param_3 * 0x4 + iVar6) != 0x0) {
    if (*(int *)(iVar6 + 0xae) != 0x0) {
      uVar2 = *(undefined4 *)(param_3 * 0x4 + iVar6);
      *(undefined2 *)((int)uVar2 + 0x6e) = *(undefined2 *)(iVar6 + 0xae);
    }
    *(undefined2 *)(iVar6 + 0xae) = 0x0;
    uVar2 = *(undefined4 *)(param_3 * 0x4 + iVar6);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_3 * 0x4 + iVar6) + 0x8);
    (**ppcVar1)(uVar8,(int)uVar2,(int)((ulong)uVar2 >> 0x10));
  }
LAB_1038_b61f:
  return CONCAT22(*(undefined2 *)(param_3 * 0x4 + iVar6 + 0x2),*(undefined2 *)(param_3 * 0x4 + iVar6));
}
