


astruct_18 * __stdcall16far pass1_1040_abe2(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_ac84(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_726 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1f3));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_726 *)param_1;
  iVar1->field_0x94 = 0x0;
  *(undefined4 *)&iVar1->field_0x98 = 0x0;
  *(undefined2 *)param_1 = 0xafc4;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar1->field_0x94 = _PTR_LOOP_1050_5ef0;
  _PTR_LOOP_1050_5ef0 = 0x0;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3d,param_5,param_3,param_4);
  iVar1->field_0x98 = (int)puVar2;
  iVar1->field_0x9a = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_ace8(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xafc4;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}



void __stdcall16far pass1_1040_ad14(ulong param_1,ushort param_2)

{
  win_ui_op_1040_ae04(param_1,param_2);
  return;
}



void __stdcall16far
pass1_1040_ad24(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  if (param_4._2_2_ == 0xeb) {
    win_ui_op_1040_ae04(CONCAT22(param_2,param_1),param_7);
  }
  else {
    if (param_4._2_2_ != 0x1f0) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
      return;
    }
    msg_box_ui_op_1040_ad66(CONCAT22(param_2,param_1),0x0,param_5,param_7);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_ui_op_1040_ad66(ulong param_1,char *param_2,uchar *param_3,UINT16 param_4)

{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1040_ae04(ulong param_1,WORD *param_2)

{
  bool bVar1;
  int iVar2;
  char *id;
  uchar *in_DX;
  ushort uVar3;
  SEGPTR lp_string;
  int iVar4;
  uint uVar5;
  int unaff_DI;
  undefined2 uVar6;
  undefined2 uVar7;
  uchar in_AF;
  ushort *puVar8;
  ulong uVar9;
  ulong uVar10;
  char *pcVar11;
  int iStack280;
  CHAR local_102 [0x100];
  
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,(ushort)param_2,in_DX,unaff_DI);
  uVar3 = (ushort)((ulong)puVar8 >> 0x10);
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_c3c2((ushort)puVar8,uVar3,CONCAT22(param_2,local_102),*(ulong *)(iVar4 + 0x94),(uchar *)uVar3,in_AF,
                  (ushort)param_2);
  SetDlgItemText16(0x1010,(INT16)local_102,(SEGPTR)param_2);
  uVar9 = struct_op_1030_73a8(*(ulong *)(iVar4 + 0x94));
  iVar2 = (int)uVar9 + 0x20;
  uVar10 = pass1_1030_8326();
  lp_string = (SEGPTR)(uVar10 >> 0x10);
  bVar1 = false;
  for (iStack280 = 0x0; iStack280 < 0xa; iStack280 = iStack280 + 0x1) {
    uVar7 = (undefined2)((uVar9 & 0xffff0000) >> 0x10);
    if ((*(uint *)(iStack280 * 0xc + iVar2 + 0x2) | *(uint *)(iStack280 * 0xc + iVar2)) != 0x0) {
      uVar5 = iStack280 * 0xc + iVar2;
      id = string_op_1020_c222(*(ushort *)(uVar5 + 0x4));
      SetDlgItemText16(0x1020,(INT16)id,lp_string);
      wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_102,param_2);
      SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_102,(SEGPTR)param_2);
      uVar10 = unk_load_str_op_1010_8c96
                         (*(ulong *)(iVar4 + 0x98),CONCAT22(param_2,local_102),uVar9 & 0xffff0000 | (ulong)uVar5,0x1010,
                          (ushort)param_2);
      lp_string = (SEGPTR)uVar10;
      SetDlgItemText16(0x1010,(INT16)local_102,(SEGPTR)param_2);
      bVar1 = true;
    }
  }
  if (!bVar1) {
    pcVar11 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    SetDlgItemText16(0x1010,(INT16)pcVar11,(SEGPTR)((ulong)pcVar11 >> 0x10));
  }
  return;
}



astruct_18 * __stdcall16far pass1_1040_af9e(astruct_18 *param_1,byte param_2)

{
  pass1_1040_ace8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_b040(astruct_57 *param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,*(ushort *)((int)param_2 + 0x12),param_3);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x8e) = 0x0;
  *(ulong *)(iVar1 + 0x90) = param_2;
  *(undefined2 *)param_1 = 0xb772;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}



void __stdcall16far struct_1040_b082(astruct_57 *param_1,ulong param_2)

{
  astruct_437 *iVar1;
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,(ushort)param_2,(ushort)(param_2 >> 0x10));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_437 *)param_1;
  iVar1->field_0x8e = 0x0;
  iVar1->field_0x90 = 0x0;
  *(undefined2 *)param_1 = 0xb772;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  return;
}



void __stdcall16far pass1_1040_b0bc(astruct_57 *param_1,ulong param_2,ulong param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,(ushort)param_3,(ushort)(param_3 >> 0x10));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x8e) = 0x0;
  *(ulong *)(iVar1 + 0x90) = param_2;
  *(undefined2 *)param_1 = 0xb772;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_draw_op_1040_b0f8(astruct_18 *param_1)

{
  uint uVar1;
  uint uVar2;
  uchar *in_DX;
  astruct_18 *iVar3;
  int unaff_DI;
  undefined2 uVar3;
  undefined2 uVar4;
  ushort unaff_SS;
  ushort *puVar5;
  astruct_18 *paStack10;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_18 *)param_1;
  param_1->field_0x0 = 0xb772;
  iVar3->field_0x2 = (ushort)&PTR_LOOP_1050_1040;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,unaff_SS,in_DX,unaff_DI);
  uVar4 = 0x1010;
  pass1_1010_7b8c((ulong)puVar5,iVar3->field_0x6,unaff_SS);
  if (iVar3->field_0x8e != 0x0) {
    uVar4 = SUB42(s_tile2_bmp_1050_1538,0x0);
    DeleteObject16(0x1010);
    iVar3->field_0x8e = 0x0;
  }
  uVar1 = iVar3->field_0x90;
  uVar2 = iVar3->field_0x92;
  paStack10 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1040_a5d0(CONCAT22(uVar2,uVar1));
    uVar4 = 0x1000;
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  ui_cleanup_op_1040_782c(param_1,uVar4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_b17c(ulong param_1,undefined4 param_2,uchar *param_3,int param_4,int param_5,ushort param_6)

{
  int *piVar1;
  undefined4 uVar2;
  char *pcVar3;
  ushort uVar4;
  int iVar5;
  undefined2 uVar6;
  uchar *puVar7;
  ushort *puVar8;
  ushort *puStack12;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    piVar1 = *(int **)(iVar5 + 0x90);
    puVar7 = (uchar *)((ulong)piVar1 >> 0x10);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    param_5 = *(int *)(iStack4 * 0x2 + (int)param_2);
    uVar2 = *(undefined4 *)((int)piVar1 + 0x2);
    *(int *)(iStack4 * 0xa + (int)uVar2 + 0x4) = param_5;
    iStack4 = iStack4 + 0x1;
    param_3 = puVar7;
  }
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,param_5);
  uVar4 = (ushort)((ulong)puVar8 >> 0x10);
  uVar2 = *(undefined4 *)(iVar5 + 0x90);
  puStack12 = *(ushort **)((int)uVar2 + 0x2);
  for (iStack4 = 0x0; piVar1 = *(int **)(iVar5 + 0x90), *piVar1 != iStack4 && iStack4 <= *piVar1;
      iStack4 = iStack4 + 0x1) {
    uVar2 = *(undefined4 *)(iVar5 + 0x90);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x6);
    pcVar3 = pass1_1010_b038((uchar *)puVar8,(ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),
                             *(uchar **)((int)puStack12 + 0x4),param_4);
    string_1040_a626(puStack12,(char *)CONCAT22(uVar4,pcVar3),uVar4);
    puStack12 = (ushort *)((ulong)puStack12 & 0xffff0000 | (ulong)((int)puStack12 + 0xa));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1040_b230(astruct_1 *param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  INT16 IVar2;
  uchar *in_DX;
  int unaff_DI;
  undefined2 uVar3;
  ushort *puVar4;
  ushort *puVar5;
  undefined2 uVar7;
  undefined4 uVar6;
  RECT16 local_1a;
  int iStack22;
  int iStack20;
  int iStack18;
  int iStack16;
  int iStack14;
  int iStack12;
  ushort *puStack10;
  int local_6;
  int local_4;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  if (PTR_LOOP_1050_5ef8 == (undefined *)((int)&DAT_1050_0004 + 0x1)) {
    PTR_LOOP_1050_5ef8 = (undefined *)0x0;
  }
  puVar5 = (ushort *)CONCAT22(param_3,&local_4);
  puVar4 = (ushort *)CONCAT22(param_3,&local_6);
  puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_3,in_DX,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puStack10 & 0xffff0000 | (ulong)((int)puStack10 + 0xe)),puVar4,puVar5);
  uVar3 = (undefined2)((ulong)puStack10 >> 0x10);
  iStack12 = *(int *)((int)puStack10 + 0xa);
  iStack14 = *(int *)((int)puStack10 + 0xc);
  uVar7 = 0x4;
  IVar2 = GetSystemMetrics16(0x1008);
  iStack16 = IVar2 * (int)PTR_LOOP_1050_5ef8 + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 0x1;
  iStack18 = iStack16 + local_6;
  iStack16 = iStack16 + local_4;
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  uVar6 = CONCAT22(uVar7,*(undefined2 *)((int)param_1 + 0x6));
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_1a);
  if (iStack14 < (iStack20 - local_1a.y) + iStack18) {
    iStack18 = -0x2 - ((iStack20 - local_1a.y) - iStack14);
  }
  if (iStack12 < (iStack22 - local_1a.x) + iStack16) {
    iStack16 = -0x2 - ((iStack22 - local_1a.x) - iStack12);
  }
  uVar3 = *(undefined2 *)((int)param_1 + 0x6);
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,0x0,iStack18,iStack16,0x0);
  ppcVar1 = (code **)((int)param_1->field_0x0 + 0x6c);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_1,uVar3,uVar6);
  return;
}



ushort __stdcall16far pass1_1040_b316(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,int param_5)

{
  code **ppcVar1;
  ushort uStack4;
  
  if (param_5 == 0xf) {
    ppcVar1 = (code **)((int)*param_1 + 0x60);
    uStack4 = (**ppcVar1)();
  }
  else {
    if (param_5 == 0x111) {
      ppcVar1 = (code **)((int)*param_1 + 0x10);
      (**ppcVar1)();
      uStack4 = 0x1;
    }
    else {
      uStack4 = pass1_1040_79c0(param_1,(int *)param_2,param_3,param_4,param_5);
    }
  }
  return uStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1040_b372(ULONG param_1,UINT16 param_2,uint param_3,COLORREF in_colorref_4)

{
  undefined2 uVar1;
  int iVar2;
  HBRUSH16 local_brush_handle;
  INT16 IVar3;
  ULONG uVar4;
  UINT16 extraout_DX;
  UINT16 uVar5;
  HWND16 local_win_handle;
  ulong uVar6;
  COLORREF local_colorref;
  
  uVar5 = (UINT16)(param_1 >> 0x10);
  local_colorref = in_colorref_4;
  if (*(int *)((int)param_1 + 0x8e) == 0x0) {
    local_colorref = (COLORREF)s_tile2_bmp_1050_1538;
    local_brush_handle = CreateSolidBrush16(in_colorref_4);
    *(HBRUSH16 *)((int)param_1 + 0x8e) = local_brush_handle;
  }
  if (_PTR_LOOP_1050_5efa == 0x0) {
    local_colorref = 0x1008;
    uVar6 = pass1_1008_4d72(*(ulong *)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (undefined2)(uVar6 >> 0x10);
    iVar2 = (int)uVar6;
    _PTR_LOOP_1050_5efa =
         (ulong)CONCAT12(*(undefined *)(iVar2 + 0x94),
                         CONCAT11(*(undefined *)(iVar2 + 0x95),*(undefined *)(iVar2 + 0x96)));
  }
  if (param_3 < 0x4) {
LAB_1040_b3ea:
    local_win_handle = (HWND16)s_tile2_bmp_1050_1538;
    IVar3 = GetDlgCtrlID16(local_colorref);
    if (IVar3 == 0x14c) {
      local_colorref = 0x0;
      goto LAB_1040_b41a;
    }
    if (IVar3 == 0x175) {
      local_colorref = 0x0;
      goto LAB_1040_b41a;
    }
  }
  else {
    local_win_handle = local_colorref;
    if (param_3 != 0x4) {
      if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
        return;
      }
      goto LAB_1040_b3ea;
    }
  }
  local_colorref = (COLORREF)_PTR_LOOP_1050_5efa;
LAB_1040_b41a:
  SetTextColor16(local_win_handle,local_colorref);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return;
}



void __stdcall16far show_win_1040_b43c(ulong *param_1,HWND16 param_2)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x70);
  (**ppcVar1)(param_2,param_1);
  ShowWindow16(param_2,0x5);
  return;
}



void __stdcall16far pass1_1040_b45e(ulong param_1,HWND16 param_2)

{
  undefined4 uVar1;
  int *piVar2;
  int iVar3;
  undefined2 uVar4;
  int iStack8;
  INT16 *pIStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x90) != 0x0) {
    uVar1 = *(undefined4 *)(iVar3 + 0x90);
    *(undefined2 *)((int)uVar1 + 0x14) = *(undefined2 *)(iVar3 + 0x6);
    uVar1 = *(undefined4 *)(iVar3 + 0x90);
    pIStack6 = *(INT16 **)((int)uVar1 + 0x2);
    for (iStack8 = 0x0; piVar2 = *(int **)(iVar3 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2;
        iStack8 = iStack8 + 0x1) {
      SetDlgItemText16(param_2,*pIStack6,(SEGPTR)*(undefined4 *)((int)pIStack6 + 0x2));
      pIStack6 = (INT16 *)((ulong)pIStack6 & 0xffff0000 | (ulong)((int)pIStack6 + 0xa));
      param_2 = (HWND16)s_tile2_bmp_1050_1538;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_b4c8(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  ushort *puVar5;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x90) != 0x0) {
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_4,param_2,param_3);
    uVar2 = *(undefined4 *)((int)param_1 + 0x90);
    iVar1 = *(int *)((int)uVar2 + 0xa);
    iVar3 = iVar1 + -0x4;
    if (iVar3 == 0x0) {
      ui_op_1010_79aa(puVar5,0xfd9,0x0,param_4);
      if (iVar3 == 0x0) {
        uVar4 = 0xe;
LAB_1040_b50f:
        unk_win_op_1010_7300((ulong)puVar5,CONCAT22(iVar3,iVar3),uVar4,CONCAT22(iVar3,iVar3));
        return;
      }
    }
    else {
      if (((0x0 < iVar1 + -0x5) && (!SBORROW2(iVar1 + -0x5,0x1))) &&
         (iVar3 = iVar1 + -0x7, iVar3 == 0x0 || iVar1 + -0x6 < 0x1)) {
        ui_op_1010_79aa(puVar5,0xfda,0x0,param_4);
        if (iVar3 == 0x0) {
          uVar4 = 0xd;
          goto LAB_1040_b50f;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_b54a(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  astruct_18 *paVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  astruct_18 *paVar6;
  uint uVar7;
  astruct_515 *iVar6;
  int unaff_DI;
  undefined2 uVar8;
  ushort *puVar9;
  undefined uVar10;
  undefined uVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  
  if (param_4._2_2_ == 0xea) {
    ppcVar2 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x5c);
    (**ppcVar2)(param_6,param_1,param_2);
  }
  else {
    if (param_4._2_2_ == 0xeb) {
      puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_7,param_5,unaff_DI);
      uVar7 = (uint)((ulong)puVar9 >> 0x10);
      paVar1 = *(astruct_18 **)(param_1 + 0x90);
      if (paVar1 != (astruct_18 *)0x0) {
        uVar8 = (undefined2)((ulong)paVar1 >> 0x10);
        uVar12 = 0x1010;
        paVar6 = paVar1;
        pass1_1010_ad64((ushort)puVar9,CONCAT22(*(undefined2 *)((int)paVar1 + 0xa),uVar7),*(ulong *)((int)paVar1 + 0x6),
                        (ulong)paVar1,uVar7);
        *(undefined2 *)(param_1 + 0x90) = (int)paVar6;
        *(uint *)(param_1 + 0x92) = uVar7;
        if ((uVar7 | *(uint *)(param_1 + 0x90)) == 0x0) {
          *(astruct_18 **)(param_1 + 0x90) = paVar1;
        }
        else {
          if (paVar1 != (astruct_18 *)0x0) {
            pass1_1040_a5d0((ulong)paVar1);
            uVar12 = 0x1000;
            fn_ptr_1000_17ce(paVar1,0x1000);
          }
          ppcVar2 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x70);
          (**ppcVar2)(uVar12,param_1,param_2);
        }
      }
    }
    else {
      if (param_4._2_2_ == 0x1790) {
        puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,unaff_DI);
        uVar3 = *(undefined4 *)(param_1 + 0x90);
        uVar3 = *(undefined4 *)((int)uVar3 + 0x6);
        iVar4 = pass1_1010_7d38((ushort)puVar9,(ushort)((ulong)puVar9 >> 0x10),(int)uVar3,(ushort)((ulong)uVar3 >> 0x10)
                                ,param_7);
        iVar5 = iVar4;
        ui_op_1010_79aa(puVar9,0xfab,0x0,param_7);
        if (iVar5 != 0x0) {
          return;
        }
        if (iVar4 == 0x0) {
          uVar3 = *(undefined4 *)(param_1 + 0x90);
          uVar8 = (undefined2)((ulong)uVar3 >> 0x10);
          iVar6 = (astruct_515 *)uVar3;
          uVar3 = iVar6->field_0x6;
          uVar13 = (undefined2)uVar3;
          uVar14 = (undefined2)((ulong)uVar3 >> 0x10);
          uVar12 = 0x14;
        }
        else {
          uVar3 = *(undefined4 *)(param_1 + 0x90);
          uVar8 = (undefined2)((ulong)uVar3 >> 0x10);
          iVar6 = (astruct_515 *)uVar3;
          uVar3 = iVar6->field_0x6;
          uVar13 = (undefined2)uVar3;
          uVar14 = (undefined2)((ulong)uVar3 >> 0x10);
          uVar12 = 0x9;
        }
        uVar10 = (undefined)uVar8;
        uVar11 = (undefined)((uint)uVar8 >> 0x8);
      }
      else {
        if (param_4._2_2_ == 0x1824) {
          puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,unaff_DI);
          iVar6 = (astruct_515 *)puVar9;
          uVar3 = *(undefined4 *)(param_1 + 0x90);
          ui_op_1010_79aa(puVar9,0xfc5,*(long *)((int)uVar3 + 0x6),param_7);
          if (iVar6 != (astruct_515 *)0x0) {
            return;
          }
          uVar3 = *(undefined4 *)(param_1 + 0x90);
          uVar3 = *(undefined4 *)((int)uVar3 + 0x6);
          uVar13 = (undefined2)uVar3;
          uVar14 = (undefined2)((ulong)uVar3 >> 0x10);
          uVar12 = 0x12;
          uVar10 = 0x0;
          uVar11 = 0x0;
        }
        else {
          if (param_4._2_2_ != 0x1830) {
            post_win_msg_1040_7b3c
                      ((ulong *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)),param_3,(ushort)param_4
                       ,param_4._2_2_,param_6);
            return;
          }
          puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,unaff_DI);
          iVar6 = (astruct_515 *)puVar9;
          uVar3 = *(undefined4 *)(param_1 + 0x90);
          ui_op_1010_79aa(puVar9,0xfb6,*(long *)((int)uVar3 + 0x6),param_7);
          if (iVar6 != (astruct_515 *)0x0) {
            return;
          }
          uVar3 = *(undefined4 *)(param_1 + 0x90);
          uVar3 = *(undefined4 *)((int)uVar3 + 0x6);
          uVar13 = (undefined2)uVar3;
          uVar14 = (undefined2)((ulong)uVar3 >> 0x10);
          uVar12 = 0xc;
          uVar10 = 0x0;
          uVar11 = 0x0;
        }
      }
      unk_win_op_1010_7300((ulong)puVar9,CONCAT13(uVar11,CONCAT12(uVar10,iVar6)),uVar12,CONCAT22(uVar14,uVar13));
    }
  }
  return;
}



void __stdcall16far destroy_window_1040_b726(ULONG *param_1,int param_2,HWND16 in_win_handle_3)

{
  code **ppcVar1;
  
  if (param_2 != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x78);
    (**ppcVar1)(in_win_handle_3,param_1);
  }
  DestroyWindow16(in_win_handle_3);
  return;
}



ULONG __stdcall16far pass1_1040_b74c(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return (ULONG)param_1;
}



void __stdcall16far pass1_1040_b7ee(astruct_57 *param_1,long param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x94) = 0x0;
  *(undefined4 *)(iVar1 + 0x98) = 0x0;
  *(undefined4 *)(iVar1 + 0xb0) = 0x0;
  *(undefined2 *)(iVar1 + 0xb4) = 0x0;
  *(undefined2 *)(iVar1 + 0xb6) = 0x0;
  *(undefined2 *)param_1 = 0xbeba;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (param_2 != 0x0) {
    uVar3 = (undefined2)((ulong)param_2 >> 0x10);
    *(undefined4 *)(iVar1 + 0xb0) = *(undefined4 *)((int)param_2 + 0x6);
    *(undefined2 *)(iVar1 + 0xb4) = *(undefined2 *)((int)param_2 + 0x14);
  }
  return;
}



ushort __stdcall16far
pass1_1040_b864(ulong *param_1,int *param_2,ushort param_3,ushort param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  ushort uVar2;
  
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((int *)CONCAT22(param_3,param_2),param_6);
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,(ushort)param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x7c);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



void __stdcall16far pass1_1040_b8be(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x80);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_1040_b8d2(astruct_1 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ulong *puVar1;
  ushort uVar2;
  ulong uVar3;
  astruct_160 *paVar4;
  ushort uVar5;
  ushort uVar6;
  int iVar7;
  uchar *puVar8;
  uchar *puVar9;
  ushort uVar10;
  ushort uVar11;
  astruct_167 *iVar10;
  int unaff_DI;
  undefined2 uVar12;
  ushort *puVar13;
  
  dialog_ui_fn_1040_78e2(param_1,param_3);
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x31,param_4,(uchar *)param_2,unaff_DI);
  puVar9 = (uchar *)((ulong)puVar13 >> 0x10);
  paVar4 = (astruct_160 *)puVar13;
  uVar12 = (undefined2)((ulong)param_1 >> 0x10);
  iVar10 = (astruct_167 *)param_1;
  *(astruct_160 **)&iVar10->field_0x98 = paVar4;
  *(uchar **)((int)&iVar10->field_0x98 + 0x2) = puVar9;
  mem_op_1000_179c(0xa,puVar9,0x1000);
  puVar8 = (uchar *)((uint)puVar9 | (uint)paVar4);
  if (puVar8 == (uchar *)0x0) {
    iVar10->field_0x94 = 0x0;
  }
  else {
    puVar13 = struct_1040_bf3e((ushort *)CONCAT22(puVar9,paVar4),iVar10->field_0x6);
    puVar8 = (uchar *)((ulong)puVar13 >> 0x10);
    paVar4 = (astruct_160 *)puVar13;
    *(astruct_160 **)&iVar10->field_0x94 = paVar4;
    *(uchar **)((int)&iVar10->field_0x94 + 0x2) = puVar8;
  }
  pass1_1040_bfde(iVar10->field_0x94,iVar10->field_0x98,param_4);
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (uchar *)((uint)puVar8 | (uint)paVar4);
  if (puVar9 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar8,0x1,0xa000a,0x0,0x800081,CONCAT22(iVar10->field_0x6,0x10a),(ushort)puVar9,
                    param_4);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (uchar *)((uint)puVar9 | (uint)paVar4);
  if (puVar8 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar9,0x1,0xa0028,0x0,0x840085,CONCAT22(iVar10->field_0x6,0x10b),(ushort)puVar8,
                    param_4);
  }
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (uchar *)((uint)puVar8 | (uint)paVar4);
  if (puVar9 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar8,0x1,0xa0046,0x0,0x860087,CONCAT22(iVar10->field_0x6,0x10d),(ushort)puVar9,
                    param_4);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (uchar *)((uint)puVar9 | (uint)paVar4);
  if (puVar8 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar9,0x1,0xa0064,0x0,0x880089,CONCAT22(iVar10->field_0x6,0x10e),(ushort)puVar8,
                    param_4);
  }
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (uchar *)((uint)puVar8 | (uint)paVar4);
  if (puVar9 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar8,0x1,0xa0082,0x0,0x820083,CONCAT22(iVar10->field_0x6,0x10c),(ushort)puVar9,
                    param_4);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (uchar *)((uint)puVar9 | (uint)paVar4);
  if (puVar8 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar9,0x1,0xa00d2,0x0,0x8a008b,CONCAT22(iVar10->field_0x6,0xbbb),(ushort)puVar8,
                    param_4);
  }
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (uchar *)((uint)puVar8 | (uint)paVar4);
  if (puVar9 == (uchar *)0x0) {
    paVar4 = (astruct_160 *)0x0;
    puVar9 = (uchar *)0x0;
  }
  else {
    pass1_1008_3bd6(paVar4,(ushort)puVar8,0x1,0xa00a0,0x8e,0x8c008d,CONCAT22(iVar10->field_0x6,0xbbc),(ushort)puVar9,
                    param_4);
  }
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,puVar9,unaff_DI);
  uVar10 = (ushort)((ulong)puVar13 >> 0x10);
  uVar2 = (ushort)puVar13;
  uVar11 = uVar10;
  uVar5 = pass1_1010_a5ac(uVar2,uVar10,iVar10->field_0xb0);
  uVar6 = pass1_1010_ac62(uVar2,uVar10,0x1e,uVar5,uVar11);
  if (uVar6 != 0x0) {
    pass1_1010_a5ca(uVar2,uVar10,uVar5,uVar6,uVar11);
    if (0x0 < (int)uVar6) {
      pass1_1010_a58a(uVar2,uVar10,uVar5,uVar6,uVar11);
      if (uVar6 == 0x0) goto LAB_1040_bb26;
    }
  }
  enable_win_1040_9234(CONCAT22(puVar9,paVar4),0x0,0x1010);
LAB_1040_bb26:
  puVar1 = iVar10->field_0x98;
  iVar7 = (int)puVar1;
  uVar3 = (ulong)puVar1 & 0xffff0000;
  uVar12 = (undefined2)(uVar3 >> 0x10);
  SetWindowPos16(0x1010,0x40,*(INT16 *)(iVar7 + 0x10),*(INT16 *)(iVar7 + 0xe),*(INT16 *)(iVar7 + 0xc),
                 *(INT16 *)(uVar3 | iVar7 + 0xa),0x0);
  return;
}



ushort __stdcall16far pass1_1040_bb5a(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void __stdcall16far destroy_win_1040_bb78(astruct_35 *param_1,HWND16 param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  BOOL16 BVar4;
  astruct_35 *iVar5;
  undefined2 uVar5;
  HWND16 HVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_35 *)param_1;
  HVar6 = param_2;
  if (iVar5->field_0xb6 != 0x0) {
    HVar6 = (HWND16)s_tile2_bmp_1050_1538;
    BVar4 = IsWindow16(param_2);
    if (BVar4 != 0x0) {
      HVar6 = (HWND16)s_tile2_bmp_1050_1538;
      DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
    }
  }
  iVar5->field_0xb6 = 0x0;
  puVar1 = iVar5->field_0x94;
  uVar2 = iVar5->field_0x96;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(HVar6,puVar1,uVar2,0x1);
  }
  *(undefined4 *)&iVar5->field_0x94 = 0x0;
  iVar5->field_0x98 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_ui_op_1040_bbe2(int param_1,ushort param_2,ushort param_3,ulong param_4,HWND16 param_5,ushort param_6)

{
  code **ppcVar1;
  BOOL16 BVar2;
  int iVar3;
  ushort uVar4;
  uchar *in_DX;
  ushort uVar5;
  ushort uVar6;
  ushort uVar7;
  int unaff_DI;
  ushort *puVar8;
  ushort *puVar9;
  undefined4 uVar10;
  ushort uVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  ushort uStack30;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  if (param_4._2_2_ != 0x10c) {
    if (param_4._2_2_ < 0x10d) {
      if (param_4._2_2_ == 0xfa) {
        uVar10 = *(undefined4 *)(param_1 + 0x98);
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x98) + 0x18);
        (**ppcVar1)(param_5,(int)uVar10,(int)((ulong)uVar10 >> 0x10),0x0,(int)_PTR_LOOP_1050_5efe,
                    (int)((ulong)_PTR_LOOP_1050_5efe >> 0x10));
        return;
      }
      if (param_4._2_2_ == 0x10a) {
        GetClientRect16(param_5,&local_a);
        uVar10 = *(undefined4 *)(param_1 + 0x98);
        local_a.y = local_a.y + 0x3;
        local_a.x = *(int *)((int)uVar10 + 0x1a) + -0x9;
        iStack6 = iStack6 + -0x3;
        iStack4 = iStack4 + -0x3;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),(BOOL16)&local_a);
        unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x98),0x1010);
        pass1_1010_32c0(*(ulong *)(param_1 + 0x98),0x0);
        pass1_1010_2ee2(*(ulong **)(param_1 + 0x98),param_6,0x1010);
        return;
      }
      if (param_4._2_2_ != 0x10b) goto LAB_1040_be78;
      uVar10 = *(undefined4 *)(param_1 + 0x98);
      uVar11 = *(ushort *)((int)uVar10 + 0x12);
      uVar6 = uVar11;
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar5 = (ushort)((ulong)puVar8 >> 0x10);
      puVar9 = puVar8;
      pass1_1010_a5ca((ushort)puVar8,uVar5,uVar6,(ushort)puVar8,uVar5);
      uVar6 = (ushort)((ulong)puVar9 >> 0x10);
      if ((uVar11 != 0x70) && ((int)puVar9 == 0x0)) {
        return;
      }
      uVar10 = *(undefined4 *)(param_1 + 0xb0);
      uVar12 = (undefined2)uVar10;
      uVar13 = (undefined2)((ulong)uVar10 >> 0x10);
      uVar10 = *(undefined4 *)(param_1 + 0x98);
      uVar11 = *(ushort *)((int)uVar10 + 0x12);
    }
    else {
      if (param_4._2_2_ != 0x10d) {
        if (param_4._2_2_ == 0x10e) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,in_DX,unaff_DI);
          iVar3 = (int)puVar8;
          ui_op_1010_79aa(puVar8,0xfc6,*(long *)(param_1 + 0xb0),param_6);
          if (iVar3 != 0x0) {
            return;
          }
          unk_win_op_1010_7300((ulong)puVar8,0x0,0x13,*(ulong *)(param_1 + 0xb0));
          return;
        }
        if (param_4._2_2_ == 0xbbb) {
          if (*(int *)(param_1 + 0xb6) != 0x0) {
            BVar2 = IsWindow16(param_5);
            param_5 = (HWND16)s_tile2_bmp_1050_1538;
            if (BVar2 != 0x0) goto LAB_1040_bd39;
          }
          uVar10 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x6),0x1b,(ushort)in_DX,param_1,
                                   (ushort)&PTR_LOOP_1050_1038,param_6);
          *(undefined2 *)(param_1 + 0xb6) = *(undefined2 *)((int)uVar10 + 0x6);
          ShowWindow16((HWND16)&PTR_LOOP_1050_1038,0x1);
          return;
        }
        if (param_4._2_2_ == 0xbbc) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
          uVar7 = (ushort)((ulong)puVar8 >> 0x10);
          uVar6 = (ushort)puVar8;
          uVar5 = uVar7;
          uVar4 = pass1_1010_a5ac(uVar6,uVar7,*(ulong *)(param_1 + 0xb0));
          uVar11 = uVar4;
          pass1_1010_a58a(uVar6,uVar7,uVar4,uVar4,uVar5);
          if (uVar11 == 0x0) {
            pass1_1010_a568(uVar6,uVar7,uVar4,0x0,uVar5);
          }
          GetDlgItem16(0x1010,0xbbc);
          EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
          return;
        }
LAB_1040_be78:
        pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,param_6);
        return;
      }
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar6 = (ushort)((ulong)puVar8 >> 0x10);
      uVar10 = *(undefined4 *)(param_1 + 0xb0);
      uVar12 = (undefined2)uVar10;
      uVar13 = (undefined2)((ulong)uVar10 >> 0x10);
      uVar11 = 0x71;
      uVar5 = uVar6;
    }
    uStack30 = (ushort)puVar8;
    param_5 = 0x1010;
    pass1_1010_a5ec(uStack30,uVar5,uVar11,CONCAT22(uVar13,uVar12),uVar6);
    if (*(int *)(param_1 + 0xb4) != 0x0) {
      param_5 = (HWND16)s_tile2_bmp_1050_1538;
      BVar2 = IsWindow16(0x1010);
      if (BVar2 != 0x0) {
        param_5 = (HWND16)s_tile2_bmp_1050_1538;
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100eb);
      }
    }
  }
LAB_1040_bd39:
  DestroyWindow16(param_5);
  return;
}
