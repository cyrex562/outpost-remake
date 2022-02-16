


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_ebd6(astruct_18 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  param_1->field_0x0 = 0xee6e;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x8e),0x1000);
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_ec1a(ushort param_1,int param_2)

{
  int *piVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  ushort uVar4;
  astruct_160 *rect;
  uchar *in_DX;
  uchar *puVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int unaff_DI;
  undefined2 uVar9;
  ushort *puVar10;
  undefined2 *puVar11;
  
  dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_2 + 0x6),(int)&PTR_LOOP_1050_1040);
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_1,in_DX,unaff_DI);
  PTR_LOOP_1050_5f2e = (undefined *)((ulong)puVar10 >> 0x10);
  *(undefined2 *)(param_2 + -0x4) = (int)puVar10;
  *(undefined2 *)(param_2 + -0x2) = PTR_LOOP_1050_5f2e;
  uVar4 = pass1_1010_0892();
  *(ushort *)(param_2 + -0x6) = uVar4;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
  }
  *(uchar **)(param_2 + -0x18) = PTR_LOOP_1050_5f2c;
  *(undefined2 *)(param_2 + -0x16) = PTR_LOOP_1050_5f2e;
  uVar4 = fn_ptr_op_1000_1708((*(int *)(param_2 + -0x6) + 0x2) * 0x4,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,
                              (uint)PTR_LOOP_1050_5f2e,0x1000);
  uVar2 = *(undefined4 *)(param_2 + 0x6);
  uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar7 = (int)uVar2;
  *(ushort *)(iVar7 + 0x8e) = uVar4;
  *(undefined2 *)(iVar7 + 0x90) = PTR_LOOP_1050_5f2e;
  *(undefined2 *)(param_2 + -0x8) = 0x1;
  while (iVar7 = *(int *)(param_2 + -0x6), piVar1 = (int *)(param_2 + -0x8), *piVar1 == iVar7 || *piVar1 < iVar7) {
    uVar2 = *(undefined4 *)(param_2 + -0x4);
    puVar11 = (undefined2 *)pass1_1010_0932((ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),*(int *)(param_2 + -0x8));
    puVar5 = (uchar *)((ulong)puVar11 >> 0x10);
    *(int *)(param_2 + -0x18) = (int)puVar11;
    *(uchar **)(param_2 + -0x16) = puVar5;
    *(undefined2 *)(param_2 + -0x20) = *puVar11;
    *(undefined2 *)(param_2 + -0x1e) = *(undefined2 *)((int)puVar11 + 0x2);
    *(undefined2 *)(param_2 + -0x1c) = 0x1;
    *(undefined2 *)(param_2 + -0x1a) = 0x1;
    rect = (astruct_160 *)(param_2 + -0x20);
    MapDialogRect16(0x1010,(RECT16 *)rect);
    mem_op_1000_179c(0x42,puVar5,0x1000);
    *(astruct_160 **)(param_2 + -0x24) = rect;
    *(uchar **)(param_2 + -0x22) = puVar5;
    uVar6 = (uint)puVar5 | (uint)rect;
    if (uVar6 == 0x0) {
      uVar2 = *(undefined4 *)(param_2 + 0x6);
      uVar2 = *(undefined4 *)((int)uVar2 + 0x8e);
      *(undefined4 *)((int)uVar2 + *(int *)(param_2 + -0x8) * 0x4) = 0x0;
    }
    else {
      uVar2 = *(undefined4 *)(param_2 + 0x6);
      uVar3 = *(undefined4 *)(param_2 + -0x18);
      pass1_1008_3bd6(rect,*(ushort *)(param_2 + -0x22),0x0,
                      CONCAT22(*(undefined2 *)(param_2 + -0x20),*(undefined2 *)(param_2 + -0x1e)),0x101,0xff0100,
                      CONCAT22(*(undefined2 *)((int)uVar2 + 0x6),*(undefined2 *)((int)uVar3 + 0x4)),uVar6,param_1);
      uVar2 = *(undefined4 *)(param_2 + 0x6);
      uVar2 = *(undefined4 *)((int)uVar2 + 0x8e);
      uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
      iVar7 = (int)uVar2;
      iVar8 = *(int *)(param_2 + -0x8) * 0x4;
      *(astruct_160 **)(iVar7 + iVar8) = rect;
      *(uint *)(iVar7 + iVar8 + 0x2) = uVar6;
    }
    uVar2 = *(undefined4 *)(param_2 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x8e);
    uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar7 = (int)uVar2;
    iVar8 = *(int *)(param_2 + -0x8) * 0x4;
    if (*(long *)(iVar7 + iVar8) != 0x0) {
      uVar2 = *(undefined4 *)(param_2 + -0x18);
      enable_win_1040_9234(*(ulong *)(iVar7 + iVar8),*(BOOL16 *)((int)uVar2 + 0x6),(int)&PTR_LOOP_1050_1040);
    }
    piVar1 = (int *)(param_2 + -0x8);
    *piVar1 = *piVar1 + 0x1;
  }
  move_win_1040_826c(*(astruct_1 **)(param_2 + 0x6),-0x1,0xffff);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far send_msg_1038_ed8a(ushort param_1,ulong param_2,ulong param_3,HWND16 param_4)

{
  uint uVar1;
  undefined2 uVar2;
  uint uVar3;
  ushort uVar4;
  uchar *in_DX;
  uint uVar5;
  uchar *puVar6;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar7;
  ulong uVar8;
  
  if (param_3._2_2_ != 0x1c8) {
    if (param_3._2_2_ == 0x1c9) {
      puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,in_DX,unaff_DI);
      uVar2 = (undefined2)((ulong)puVar7 >> 0x10);
      uVar1 = *(uint *)((int)puVar7 + 0x20);
      uVar5 = *(uint *)((int)puVar7 + 0x22);
      uVar3 = uVar5 | uVar1;
      if (uVar3 == 0x0) {
        return;
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,uVar5);
      puVar6 = (uchar *)(uVar5 | uVar3);
      if (puVar6 == (uchar *)0x0) {
        return;
      }
      uVar4 = pass1_1030_5b00(CONCAT22(uVar5,uVar3));
      puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar4,unaff_SS,puVar6,unaff_DI);
      if (puVar7 == (ushort *)0x0) {
        return;
      }
      param_4 = 0x1018;
      uVar8 = pass1_1018_0ad4((ulong)puVar7);
      if (uVar8 == 0x0) {
        return;
      }
      param_3._2_2_ = 0x72;
    }
    else {
      if (param_3._2_2_ != 0x1ca) {
        post_win_msg_1040_7b3c
                  ((ulong *)CONCAT22((int)param_2,param_1),(ushort)(param_2 >> 0x10),(ushort)param_3,param_3._2_2_,
                   (int)&PTR_LOOP_1050_1040);
        return;
      }
    }
  }
  SendMessage16(param_4,0x0,0x0,CONCAT22(0x111,param_3._2_2_));
  return;
}



astruct_18 * __stdcall16far pass1_1038_ee48(astruct_18 *param_1,byte param_2)

{
  pass1_1038_ebd6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_eeda(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_714 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x166,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_714 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x67c;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_5,param_3,param_4);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  iVar1->field_0x74 = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far destroy_win_1038_ef3a(astruct_31 *param_1,HWND16 param_2)

{
  astruct_31 *iVar1;
  astruct_31 *uVar1;
  
  uVar1 = (astruct_31 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_31 *)param_1;
  *(undefined2 *)param_1 = 0x67c;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  if (iVar1->field_0x96 != 0x0) {
    DestroyWindow16(param_2);
    iVar1->field_0x96 = 0x0;
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar1->field_0x6);
  ui_cleanup_op_1040_782c((astruct_18 *)param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1040_0000(astruct_1 *param_1,uchar *param_2,HWND16 param_3)

{
  astruct_160 *rect;
  uint uVar1;
  undefined2 uVar2;
  ushort uVar3;
  ushort uVar4;
  ushort uVar5;
  int unaff_DI;
  undefined2 uVar6;
  WNDCLASS16 *unaff_SS;
  LRESULT LVar7;
  undefined4 uVar8;
  undefined2 local_24;
  undefined2 uStack34;
  undefined2 uStack32;
  undefined2 uStack30;
  int iStack28;
  RECT16 local_1a;
  int iStack22;
  undefined4 uStack18;
  undefined2 uStack14;
  int iStack12;
  int iStack10;
  astruct_160 *paStack8;
  uint uStack6;
  int iStack4;
  
                    // Segment:    9
                    // Offset:     0006f820
                    // Length:     d974
                    // Min Alloc:  d974
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  dialog_ui_fn_1040_78e2(param_1,param_3);
  iStack4 = 0x8;
  for (iStack10 = 0x0; uVar5 = (ushort)param_1, uVar6 = (undefined2)((ulong)param_1 >> 0x10), iStack10 < iStack4;
      iStack10 = iStack10 + 0x1) {
    unaff_DI = iStack10 * 0xe;
    local_24 = *(undefined2 *)(unaff_DI + 0x5c60);
    uStack34 = *(undefined2 *)(unaff_DI + 0x5c62);
    uStack32 = 0x1;
    uStack30 = 0x1;
    rect = (astruct_160 *)&local_24;
    MapDialogRect16(param_3,(RECT16 *)rect);
    param_3 = 0x1000;
    mem_op_1000_179c(0x42,param_2,0x1000);
    uVar1 = (uint)param_2 | (uint)rect;
    if (uVar1 == 0x0) {
      rect = (astruct_160 *)0x0;
      uVar1 = 0x0;
    }
    else {
      param_3 = 0x1008;
      pass1_1008_3bd6(rect,(ushort)param_2,0x1,CONCAT22(local_24,uStack34),0x104,0x1020103,
                      CONCAT22(*(undefined2 *)(uVar5 + 0x6),*(undefined2 *)(unaff_DI + 0x5c64)),uVar1,(ushort)unaff_SS);
    }
    paStack8 = rect;
    uStack6 = uVar1;
    LVar7 = win_ui_op_1040_0558((ulong)param_1,iStack10,param_3);
    param_2 = (uchar *)((ulong)LVar7 >> 0x10);
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  uStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,(ushort)unaff_SS,param_2,unaff_DI);
  uVar2 = (undefined2)((ulong)uStack18 >> 0x10);
  iStack12 = *(int *)((int)uStack18 + 0xa);
  uStack14 = *(undefined2 *)((int)uStack18 + 0xc);
  GetWindowRect16(0x1010,&local_1a);
  uVar3 = iStack12 >> 0xf;
  iStack28 = iStack22 - local_1a.x;
  local_1a.x = (iStack12 / 0x2 - iStack28) + -0x3;
  if (local_1a.x < 0x0) {
    local_1a.x = 0x0;
  }
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x41,0x0,0x0,local_1a.y,local_1a.x,0x0);
  uVar8 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar5 + 0x6),0x17,uVar3,uVar5,(ushort)&PTR_LOOP_1050_1038,
                          (ushort)unaff_SS);
  uVar4 = (ushort)((ulong)uVar8 >> 0x10);
  uVar3 = (ushort)uVar8;
  *(ushort *)(uVar5 + 0x96) = uVar3;
  *(ushort *)(uVar5 + 0x98) = uVar4;
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x9e0001,unaff_SS,uVar3,uVar4);
  *(ushort *)(uVar5 + 0x8c) = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_ui_op_1040_0170(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,WNDCLASS16 *param_6)

{
  uint uVar1;
  BOOL16 BVar2;
  int iVar3;
  uchar *in_DX;
  int iVar4;
  uchar *extraout_DX;
  int unaff_DI;
  undefined2 uVar5;
  uchar in_AF;
  char *pcVar6;
  LRESULT LVar7;
  WPARAM16 w_param;
  undefined2 uVar8;
  HCURSOR16 *pHVar9;
  WNDCLASS16 *pWVar10;
  HCURSOR16 *pHVar11;
  WNDCLASS16 *pWVar12;
  ulong *local_12a [0x43];
  ushort *puStack30;
  ushort uStack26;
  HCURSOR16 local_18;
  HCURSOR16 local_16;
  ulong uStack20;
  int iStack16;
  int iStack14;
  ushort *puStack12;
  int iStack8;
  int iStack6;
  int iStack4;
  
  iStack4 = 0x8;
  iStack6 = 0x0;
  switch(param_4._2_2_) {
  case 0x167:
    enable_win_1040_060e(CONCAT22(param_2,param_1),0x3,(int)&PTR_LOOP_1050_1040,(ushort)param_6);
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1040,0x16b);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x0;
    break;
  case 0x168:
    enable_win_1040_060e(CONCAT22(param_2,param_1),0x3,(int)&PTR_LOOP_1050_1040,(ushort)param_6);
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1040,0x16b);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x1;
    break;
  case 0x169:
    enable_win_1040_060e(CONCAT22(param_2,param_1),0x3,(int)&PTR_LOOP_1050_1040,(ushort)param_6);
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1040,0x16b);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x2;
    break;
  case 0x16a:
    enable_win_1040_060e(CONCAT22(param_2,param_1),0x3,(int)&PTR_LOOP_1050_1040,(ushort)param_6);
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1040,0x16b);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x3;
    break;
  case 0x16b:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1040,0x16b);
    uVar5 = SUB42(s_tile2_bmp_1050_1538,0x0);
    BVar2 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    if (*(int *)(param_1 + 0x92) != 0x3) {
      uVar5 = 0x1008;
      win_1008_5c5c(param_6,BVar2,(ushort)in_DX,_PTR_LOOP_1050_02a0,0x1de);
    }
    if (*(int *)(param_1 + 0x92) != 0x8) {
      iVar3 = *(int *)(param_1 + 0x92) * 0xe;
      iStack6 = *(int *)(iVar3 + 0x5c6c);
      uVar5 = 0x1010;
      pass1_1010_6604(*(ulong *)(param_1 + 0x8e),*(ushort *)(iVar3 + 0x5c66),(ushort)param_6);
      *(undefined2 *)(param_1 + 0x92) = 0x8;
    }
    for (iStack8 = 0x0; iStack8 < 0x4; iStack8 = iStack8 + 0x1) {
      LVar7 = win_ui_op_1040_0558(CONCAT22(param_2,param_1),iStack8,uVar5);
      in_DX = (uchar *)((ulong)LVar7 >> 0x10);
    }
    goto LAB_1040_04da;
  case 0x16c:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1040,0x16d);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x5;
    *(undefined2 *)(param_1 + 0x94) = 0x5;
    goto LAB_1040_04da;
  case 0x16d:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1040,0x16d);
    BVar2 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    uVar5 = 0x1008;
    win_1008_5c5c(param_6,BVar2,(ushort)in_DX,_PTR_LOOP_1050_02a0,0x1de);
    if (*(int *)(param_1 + 0x94) != 0x8) {
      iVar3 = *(int *)(param_1 + 0x94) * 0xe;
      iStack6 = *(int *)(iVar3 + 0x5c6c);
      uVar5 = 0x1010;
      pass1_1010_6604(*(ulong *)(param_1 + 0x8e),*(ushort *)(iVar3 + 0x5c66),(ushort)param_6);
      *(undefined2 *)(param_1 + 0x94) = 0x8;
    }
    LVar7 = win_ui_op_1040_0558(CONCAT22(param_2,param_1),0x5,uVar5);
    puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,(ushort)param_6,(uchar *)((ulong)LVar7 >> 0x10),unaff_DI);
    iVar3 = *(int *)((int)puStack12 + 0x20);
    pHVar11 = &local_16;
    pHVar9 = &local_18;
    iVar4 = (iVar3 >> 0xf) + 0x200;
    pWVar10 = param_6;
    pWVar12 = param_6;
    iStack16 = iVar3;
    iStack14 = iVar4;
    iStack8 = iVar3;
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),CONCAT22(iVar4,iVar3));
    uStack20 = CONCAT22(iVar4,iVar3);
    pass1_1030_2f1a(CONCAT22(iVar4,iVar3),(ushort *)CONCAT22(pWVar10,pHVar9),(ushort *)CONCAT22(pWVar12,pHVar11));
    in_DX = (uchar *)((int)(local_18 - local_16) >> 0xf);
    local_16 = local_16 + (int)(local_18 - local_16) / 0x2;
    uStack26 = pass1_1030_2fac(uStack20);
    set_window_text_1018_6086(*(ULONG *)(param_1 + 0x96),0x1018,param_6);
    goto LAB_1040_04da;
  case 0x16e:
    puStack30 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,(ushort)param_6,in_DX,unaff_DI);
    uStack26 = *(ushort *)((int)puStack30 + 0x20);
    local_18 = LoadCursor16(0x1010,(LPCSTR)0x7f02);
    local_16 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    pass1_1030_532e((astruct_100 *)CONCAT22(param_6,local_12a),(long)(int)uStack26 + 0x2000000,(ushort)param_6,in_AF);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,local_12a));
    pass1_1030_838e((ulong *)_PTR_LOOP_1050_5748,(ushort)param_6,in_AF);
    pass1_1030_8334((int)_PTR_LOOP_1050_5748,(int)((ulong)_PTR_LOOP_1050_5748 >> 0x10));
    in_DX = extraout_DX;
    SetCursor16(0x1030);
    PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x111007e);
    DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
    local_12a[0] = &ULONG_1008_389a;
    goto LAB_1040_04da;
  default:
    post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,param_5);
    return;
  }
  *(int *)(param_1 + 0x92) = iStack4;
LAB_1040_04da:
  if (iStack4 != 0x8) {
    uVar5 = *(undefined2 *)(iStack4 * 0xe + 0x5c68);
    w_param = 0x0;
    uVar8 = 0xc;
    pcVar6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    LVar7 = SendDlgItemMessage16(0x1010,(INT16)pcVar6,(UINT16)((ulong)pcVar6 >> 0x10),w_param,CONCAT22(uVar5,uVar8));
    in_DX = (uchar *)((ulong)LVar7 >> 0x10);
  }
  if (iStack6 != 0x0) {
    local_12a[0] = (ulong *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_6,in_DX,unaff_DI);
    uVar1 = *(uint *)((int)local_12a[0] + 0x20);
    puStack30 = (ushort *)((ulong)puStack30 & 0xffff0000 | (ulong)uVar1);
    if (uVar1 != 0x0) {
      PostMessage16(0x1010,0x0,0x0,CONCAT22(0x111,iStack6));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT __stdcall16far win_ui_op_1040_0558(ulong param_1,int param_2,HWND16 param_3)

{
  int iVar1;
  int iVar2;
  ushort unaff_SS;
  char *pcVar3;
  LRESULT LVar4;
  WPARAM16 w_param;
  undefined2 uVar5;
  undefined2 uVar6;
  
  iVar1 = param_2 * 0xe;
  GetDlgItem16(param_3,*(INT16 *)(iVar1 + 0x5c64));
  iVar2 = pass1_1010_659a(*(ulong *)((int)param_1 + 0x8e),*(ushort *)(iVar1 + 0x5c66),unaff_SS);
  if ((iVar2 == 0x0) && (*(int *)(iVar1 + 0x5c66) != 0xa)) {
    EnableWindow16(0x1010,0x0);
    uVar6 = *(undefined2 *)(param_2 * 0xe + 0x5c68);
  }
  else {
    EnableWindow16(0x1010,0x1);
    uVar6 = *(undefined2 *)(param_2 * 0xe + 0x5c68);
  }
  uVar5 = 0xc;
  w_param = 0x0;
  pcVar3 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  LVar4 = SendDlgItemMessage16(0x1010,(INT16)pcVar3,(UINT16)((ulong)pcVar3 >> 0x10),w_param,CONCAT22(uVar6,uVar5));
  return LVar4;
}



void __cdecl16far enable_win_1040_060e(ulong param_1,int param_2,HWND16 param_3,ushort param_4)

{
  INT16 *pIVar1;
  int iStack10;
  int iStack8;
  
  _iStack8 = (INT16 *)CONCAT22(param_4,&stack0x000a);
  iStack10 = param_2;
  while( true ) {
    pIVar1 = _iStack8;
    if (iStack10 == 0x0) break;
    _iStack8 = (INT16 *)((ulong)_iStack8 & 0xffff0000 | (ulong)(iStack8 + 0x2));
    GetDlgItem16(param_3,*pIVar1);
    param_3 = (HWND16)s_tile2_bmp_1050_1538;
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    iStack10 = iStack10 + -0x1;
  }
  return;
}



astruct_31 * __stdcall16far pass1_1040_0656(astruct_31 *param_1,byte param_2)

{
  destroy_win_1038_ef3a(param_1,(int)&PTR_LOOP_1050_1038);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1040_06e8(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               ushort param_7)

{
  int iVar1;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfbc,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x8e) = 0x0;
  *(undefined2 *)param_1 = 0xb90;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x7,param_7,param_6,unaff_DI);
  *(undefined2 *)(iVar1 + 0x8e) = (int)puVar3;
  *(undefined2 *)(iVar1 + 0x90) = (int)((ulong)puVar3 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_073a(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xb90;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far show_win_1040_0766(astruct_1 *param_1,undefined2 param_2)

{
  uchar *in_DX;
  uchar *puVar1;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar2;
  int *piVar3;
  int *piVar4;
  ushort uVar5;
  int local_a;
  int local_8;
  undefined4 uStack6;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
  puVar1 = (uchar *)((ulong)uStack6 >> 0x10);
  pass1_1010_6118((ulong)uStack6);
  piVar4 = &local_8;
  piVar3 = &local_a;
  uVar5 = unaff_SS;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,puVar1,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)),
                  (ushort *)CONCAT22(unaff_SS,piVar3),(ushort *)CONCAT22(uVar5,piVar4));
  move_win_1040_826c(param_1,local_a + 0x8c,local_8 + 0xb9);
  ShowWindow16(0x1008,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_ui_op_1040_07dc(ushort param_1,ushort param_2,ushort param_3,ushort param_4,uint param_5,ushort param_6,
                   HWND16 param_7,ushort param_8)

{
  code **ppcVar1;
  INT16 IVar2;
  BOOL16 BVar3;
  uchar *puVar4;
  uchar *puVar5;
  int unaff_DI;
  undefined2 uVar6;
  ushort *puVar7;
  undefined4 *puVar8;
  undefined uVar9;
  undefined uVar10;
  undefined4 uStack2060;
  char local_806 [0x400];
  undefined4 local_406 [0x100];
  undefined4 uStack6;
  
  uStack6 = 0x0;
  if (param_5 == 0x73) {
    enable_window_1040_0acc(param_1,param_2,0x0,param_7);
    puVar4 = pass1_1008_5fd8(param_8,(uchar *)param_6);
    uStack2060 = CONCAT22(param_6,puVar4);
    puVar5 = (uchar *)param_6;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_806,param_8);
    IVar2 = MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x14),local_806,param_8);
    local_406[0] = uStack2060;
    uVar6 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_6,puVar4),0x1000);
    if (IVar2 == 0x6) {
      uVar6 = SUB42(s_tile2_bmp_1050_1538,0x0);
      PostMessage16(0x1000,0x0,0x0,0x11100cb);
      BVar3 = post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,param_4,0x1,(int)s_tile2_bmp_1050_1538);
      uStack6 = CONCAT22(puVar5,BVar3);
    }
  }
  else {
    uVar9 = (undefined)(param_2 >> 0x8);
    if (param_5 < 0x74) {
      if (param_5 == 0x6e) {
        *(undefined2 *)((int)_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        puVar8 = (undefined4 *)
                 pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x6),0x2,param_6,param_1,
                                 (ushort)&PTR_LOOP_1050_1038,param_8);
        ppcVar1 = (code **)((int)*puVar8 + 0x3c);
        (**ppcVar1)((int)&PTR_LOOP_1050_1038,(int)puVar8,(int)((ulong)puVar8 >> 0x10));
        SetFocus16((HWND16)&PTR_LOOP_1050_1038);
        return;
      }
      if (0x6e < param_5) {
LAB_1040_09f9:
        post_win_msg_1040_7b3c((ulong *)CONCAT13(uVar9,CONCAT12((char)param_2,param_1)),param_3,param_4,param_5,param_7)
        ;
        return;
      }
      if ((char)param_5 == '\x02') {
LAB_1040_09b4:
        post_win_msg_1040_7b3c((ulong *)CONCAT13(uVar9,CONCAT12((char)param_2,param_1)),0x0,0x0,0x2,param_7);
        PostMessage16(param_7,0x0,0x0,0x11100ee);
        return;
      }
      if ((char)param_5 != 'd') goto LAB_1040_09f9;
      uVar9 = 0x0;
      uVar10 = 0x0;
      uVar6 = SUB42(s_tile2_bmp_1050_1538,0x0);
      PostMessage16(param_7,0x0,0x0,0x1110064);
      goto LAB_1040_0821;
    }
    if (param_5 != 0x74) {
      if (param_5 == 0xee) goto LAB_1040_09b4;
      if (param_5 == 0x13d) {
        enable_window_1040_0acc(param_1,param_2,0x1,param_7);
        return;
      }
      goto LAB_1040_09f9;
    }
    enable_window_1040_0acc(param_1,param_2,0x0,param_7);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,(char *)local_406,
               param_8);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_806,param_8);
    uVar6 = SUB42(s_tile2_bmp_1050_1538,0x0);
    IVar2 = MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x14),(LPCSTR)local_406,param_8);
    if (IVar2 == 0x6) {
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x111007a);
      BVar3 = post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,param_4,0x1,(int)s_tile2_bmp_1050_1538);
      uStack6 = CONCAT22(param_6,BVar3);
      puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,(uchar *)param_6,unaff_DI);
      uVar6 = 0x1010;
      pass1_1010_60fa((ulong)puVar7);
    }
  }
  uVar9 = 0x1;
  uVar10 = 0x0;
LAB_1040_0821:
  enable_window_1040_0acc(param_1,param_2,CONCAT11(uVar10,uVar9),uVar6);
  return;
}



void __stdcall16far pass1_1040_0a1a(ulong param_1)

{
  uint uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  uchar *puVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  ulong uStack10;
  uint uStack6;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar4 = *(undefined4 *)(iVar8 + 0x8e);
  uVar11 = (undefined2)((ulong)uVar4 >> 0x10);
  iVar9 = (int)uVar4;
  puVar2 = (undefined4 *)*(undefined4 *)(iVar9 + 0xa);
  uStack6 = (uint)puVar2;
  puVar5 = (undefined4 *)(*(uint *)(iVar9 + 0xc) | uStack6);
  if (puVar5 == (undefined4 *)0x0) {
    return;
  }
  ppcVar3 = (code **)((int)*puVar2 + 0x14);
  (**ppcVar3)();
  uStack10 = CONCAT22(extraout_DX,puVar5);
  puVar6 = extraout_DX;
  if (*(long *)(iVar8 + 0x70) != 0x0) {
    puVar5 = (undefined4 *)*(uint *)(iVar8 + 0x70);
    uVar1 = *(uint *)(iVar8 + 0x72);
    puVar6 = (uchar *)(uVar1 | (uint)puVar5);
    if (puVar6 != (uchar *)0x0) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
      puVar6 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | (uint)puVar5);
  if (puVar7 == (uchar *)0x0) {
    puVar5 = (undefined4 *)0x0;
    puVar7 = (uchar *)0x0;
  }
  else {
    struct_1008_4c58((ushort *)CONCAT22(puVar6,puVar5));
  }
  *(undefined2 *)(iVar8 + 0x70) = puVar5;
  *(uchar **)(iVar8 + 0x72) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70),uStack10,puVar7);
  return;
}



void __stdcall16far enable_window_1040_0acc(ushort param_1,ushort param_2,BOOL16 param_3,HWND16 param_4)

{
  BOOL16 BVar1;
  
  BVar1 = IsWindow16(param_4);
  if (BVar1 != 0x0) {
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x64);
    BVar1 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
    if (BVar1 != 0x0) {
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x74);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x73);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x6e);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xee);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1040_0b6a(astruct_18 *param_1,byte param_2)

{
  pass1_1040_073a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1040_0bfc(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_715 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfcd,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_715 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  *(undefined2 *)param_1 = 0xdb0;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  iVar1->field_0x74 = 0x1;
  return param_1;
}



void __stdcall16far pass1_1040_0c54(astruct_18 *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xdb0;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  *(undefined4 *)((int)param_1 + 0x8e) = 0x0;
  ui_cleanup_op_1040_782c(param_1,param_2);
  return;
}



void __stdcall16far show_win_1040_0c7c(astruct_1 *param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined4 local_6;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
  pass1_1010_4f30((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),(ushort *)CONCAT22(param_3,&local_6),
                  (ushort *)CONCAT22(param_3,(int)&local_6 + 0x2));
  move_win_1040_826c(param_1,(INT16)local_6,(BOOL16)((ulong)local_6 >> 0x10));
  ShowWindow16(0x1010,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined4 __stdcall16far set_text_bk_color_1040_0cc0(ulong *param_1,ushort param_2,uint param_3,INT16 param_4)

{
  code **ppcVar1;
  int iVar2;
  HDC16 obj;
  HDC16 hdc;
  ulong uVar3;
  undefined2 uVar4;
  HGDIOBJ16 HStack4;
  
  uVar4 = 0x4;
  obj = (HDC16)s_tile2_bmp_1050_1538;
  HStack4 = GetStockObject16(param_4);
  if (_PTR_LOOP_1050_5cd0 == 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x68);
    uVar3 = (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_1,*(undefined2 *)((int)param_1 + 0x6e),uVar4);
    obj = 0x1008;
    uVar3 = pass1_1008_4d72(uVar3);
    uVar4 = (undefined2)(uVar3 >> 0x10);
    iVar2 = (int)uVar3;
    _PTR_LOOP_1050_5cd0 =
         CONCAT22(CONCAT11(0x2,*(undefined *)(iVar2 + 0x94)),
                  CONCAT11(*(undefined *)(iVar2 + 0x95),*(undefined *)(iVar2 + 0x96)));
  }
  hdc = obj;
  if (0x3 < param_3) {
    if (param_3 == 0x4) {
      hdc = (HDC16)s_tile2_bmp_1050_1538;
      HStack4 = GetStockObject16(obj);
    }
    else {
      if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
        return 0x0;
      }
    }
  }
  SetTextColor16(hdc,(COLORREF)_PTR_LOOP_1050_5cd0);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return CONCAT22(0x1050,HStack4);
}



void __stdcall16far post_win_msg_1040_0d5e(ushort param_1,ushort param_2,int param_3,HWND16 param_4)

{
  if (param_3 != 0x0) {
    PostMessage16(param_4,0x0,0x0,0x1110001);
  }
  return;
}



ushort __stdcall16far pass1_1040_0d80(void)

{
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1040_0d8a(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1040_0c54(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_0e1c(astruct_57 *param_1,ushort param_2,ulong param_3,ushort param_4,uchar *param_5,int param_6,
               ushort param_7)

{
  astruct_716 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c0,param_4);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_716 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = param_3;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = param_2;
  *(int *)param_1 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3a,param_7,param_5,param_6);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}
