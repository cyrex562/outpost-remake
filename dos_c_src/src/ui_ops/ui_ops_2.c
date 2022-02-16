
void __stdcall16far win_ui_op_1040_5800(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  ushort uVar4;
  astruct_18 *paVar5;
  uchar *in_DX;
  uchar *puVar6;
  uchar *puVar7;
  uchar *extraout_DX;
  int iVar8;
  uchar *unaff_DI;
  undefined2 uVar9;
  HWND16 hwnd;
  ushort unaff_SS;
  int *piStack24;
  RECT16 local_14 [0x2];
  int iStack12;
  astruct_18 *paStack10;
  astruct_20 *paStack6;
  
  if (param_4._2_2_ == 0xeb) {
    paStack6 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,(int)unaff_DI);
    puVar6 = (uchar *)((ulong)paStack6 >> 0x10);
    paVar5 = *(astruct_18 **)(param_1 + 0x90);
    if (paVar5 != (astruct_18 *)0x0) {
      paStack10 = paVar5;
      mem_op_1000_179c(0x18,puVar6,0x1000);
      uVar3 = (uint)paVar5;
      puVar7 = (uchar *)((uint)puVar6 | uVar3);
      if (puVar7 == (uchar *)0x0) {
        uVar3 = 0x0;
        puVar7 = (uchar *)0x0;
      }
      else {
        struct_1040_a598((ushort *)((ulong)paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
      }
      *(uint *)(param_1 + 0x90) = uVar3;
      *(uchar **)(param_1 + 0x92) = puVar7;
      *(undefined2 *)*(undefined4 *)(param_1 + 0x90) = 0x6;
      iStack12 = **(int **)(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,puVar7,0x1000);
      piStack24 = (int *)CONCAT22(puVar7,uVar3);
      if (((uint)puVar7 | uVar3) == 0x0) {
        uVar2 = *(undefined4 *)(param_1 + 0x90);
        *(undefined4 *)((int)uVar2 + 0x2) = 0x0;
      }
      else {
        *piStack24 = iStack12;
        pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,(ushort)puVar7);
        uVar2 = *(undefined4 *)(param_1 + 0x90);
        uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar8 = (int)uVar2;
        *(int *)(iVar8 + 0x2) = uVar3 + 0x2;
        *(uchar **)(iVar8 + 0x4) = puVar7;
        unaff_DI = puVar7;
      }
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined4 *)((int)uVar2 + 0x6) = *(undefined4 *)((int)paStack10 + 0x6);
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar2 + 0xa) = 0x4;
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar2 + 0x12) = *(undefined2 *)(param_1 + 0xa);
      hwnd = 0x1010;
      pass1_1010_a50c(paStack6,0x10505d78,*(ulong *)(param_1 + 0x90));
      if (paStack10 != (astruct_18 *)0x0) {
        pass1_1040_a5d0((ulong)paStack10);
        hwnd = 0x1000;
        fn_ptr_1000_17ce(paStack10,0x1000);
      }
      ppcVar1 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar1)();
      puVar6 = extraout_DX;
      uVar4 = pass1_1040_5cd6(CONCAT22(param_2,param_1));
      if (uVar4 != 0x0) {
        pass1_1040_5eaa(CONCAT22(param_2,param_1));
        *(undefined2 *)(param_1 + 0x94) = 0x0;
      }
      pass1_1040_5dc4(CONCAT22(param_2,param_1),puVar6,(int)unaff_DI,unaff_SS);
      GetWindowRect16(hwnd,local_14);
      InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,*(RECT16 **)(param_1 + 0x9c),0x0);
      if (*(int *)(param_1 + 0x9c) != 0x0) {
        *(undefined2 *)(param_1 + 0x9c) = 0x0;
      }
    }
  }
  else {
    if (param_4._2_2_ != 0x13b) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
      return;
    }
    GetDlgItem16(param_5,0x1790);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  }
  return;
}



void __stdcall16far
message_box_op_1040_37f0(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  uchar *in_DX;
  ushort uVar2;
  int unaff_DI;
  LRESULT LVar3;
  int iVar4;
  char local_40c [0x402];
  undefined4 uStack10;
  ushort *puStack6;
  
  if (param_4._2_2_ == 0x193) {
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_6,in_DX,unaff_DI);
    uVar2 = (ushort)((ulong)puStack6 >> 0x10);
    uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_40c,param_6);
    uVar1 = MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10),(LPCSTR)uStack10,
                         (UINT16)((ulong)uStack10 >> 0x10));
    pass1_1018_3710(*(ulong **)(param_1 + 0x8e),param_6,uVar1,uVar2);
    PostMessage16(0x1018,0x0,0x0,0x1110002);
  }
  else {
    if (param_4._2_2_ != 0x194) {
      post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,param_5);
      return;
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x8),0x21,(ushort)in_DX,param_1,
                    (ushort)&PTR_LOOP_1050_1038,param_6);
    LVar3 = SendMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
    iVar4 = 0x1;
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,(uchar *)((ulong)LVar3 >> 0x10),unaff_DI);
    pass1_1010_038e((ulong)puStack6,iVar4,param_6);
  }
  return;
}



void __stdcall16far pass1_1040_39e2(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x3ffc;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far show_win_1040_3ae8(astruct_1 *param_1,ushort param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



void __stdcall16far win_ui_op_1040_3b1e(astruct_2 *param_1,WORD *param_2)

{
  BOOL16 BVar1;
  HWND16 HVar2;
  uchar *in_DX;
  ushort uVar3;
  ushort uVar4;
  int unaff_DI;
  ushort uVar5;
  ulong uVar6;
  CHAR local_10e [0x82];
  CHAR local_8c [0x82];
  undefined4 uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_2,in_DX,unaff_DI);
  uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
  uVar5 = (ushort)((ulong)param_1 >> 0x10);
  uVar4 = (ushort)param_1;
  GetWindowText16(0x1010,0x80,(INT16)local_8c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_10e,param_2);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_10e);
  uVar3 = uVar5;
  pass1_1018_3d44(*(ulong *)(uVar4 + 0x8e),(ulong *)((ulong)param_1 & 0xffff0000 | (ulong)(uVar4 + 0x92)),
                  (ulong *)((ulong)param_1 & 0xffff0000 | (ulong)(uVar4 + 0x96)));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x80,local_10e,
             (short)param_2);
  wsprintf16((LPSTR)0x1010,local_8c,param_2);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_8c,(SEGPTR)param_2);
  BVar1 = CheckRadioButton16((HWND16)s_tile2_bmp_1050_1538,0x188,0x18d,0x188);
  *(undefined2 *)(uVar4 + 0xa0) = 0x188;
  uVar6 = switch_1018_3b9e(*(ulong *)(uVar4 + 0x8e),*(ushort *)(uVar4 + 0xa0),BVar1,uVar3,param_2);
  send_dlg_item_msg_1040_3f12(uVar4,uVar5,uVar6,0x1018,(ushort)param_2);
  dialog_item_ui_op_1040_3e08(param_1,0x1018);
  HVar2 = GetDlgItem16(0x1018,0x186);
  *(HWND16 *)(uVar4 + 0x9a) = HVar2;
  return;
}



void __stdcall16far unk_win_ui_op_1040_3c64(int param_1,ushort param_2,ushort param_3,ulong param_4,UINT16 param_5)

{
  UINT16 UVar1;
  undefined2 in_DX;
  ushort uVar2;
  int unaff_DI;
  ushort unaff_SS;
  ulong uVar3;
  LRESULT LVar4;
  ushort *puVar5;
  int iVar6;
  
  if (param_4._2_2_ == 0x186) {
    LVar4 = SendDlgItemMessage16(param_5,0x0,0x0,0x0,0x1900409);
    uVar2 = (ushort)((ulong)LVar4 >> 0x10);
    UVar1 = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,(BOOL16 *)0x0,0x0);
    pass1_1018_36e6(*(ulong *)(param_1 + 0x8e),UVar1,(ushort)LVar4,*(ushort *)(param_1 + 0xa0));
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x8),0x22,uVar2,param_1,(ushort)&PTR_LOOP_1050_1038,
                    unaff_SS);
    LVar4 = SendMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
    iVar6 = 0x1;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,(uchar *)((ulong)LVar4 >> 0x10),unaff_DI);
    pass1_1010_038e((ulong)puVar5,iVar6,unaff_SS);
  }
  else {
    if (param_4._2_2_ - 0x186 < 0x2) {
LAB_1040_3c7f:
      post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,param_5);
      return;
    }
    if (param_4._2_2_ - 0x188 < 0x5 || param_4._2_2_ == 0x18d) {
      *(ushort *)(param_1 + 0xa0) = param_4._2_2_;
      param_5 = 0x1018;
      uVar3 = switch_1018_3b9e(*(ulong *)(param_1 + 0x8e),param_4._2_2_,param_4._2_2_,in_DX,unaff_SS);
      send_dlg_item_msg_1040_3f12(param_1,param_2,uVar3,0x1018,unaff_SS);
    }
    else {
      if (param_4._2_2_ - 0x188 != 0x8) goto LAB_1040_3c7f;
      if ((ushort)param_4 != 0x1) {
        return;
      }
    }
    dialog_item_ui_op_1040_3e08((astruct_2 *)CONCAT22(param_2,param_1),param_5);
  }
  return;
}



void __stdcall16far dialog_item_ui_op_1040_3e08(astruct_2 *in_struct_1,UINT16 param_2)

{
  UINT16 UVar1;
  ushort uVar2;
  astruct_2 *local_struct_1;
  undefined2 var3;
  HWND16 HVar3;
  ushort unaff_SS;
  LRESULT LVar4;
  
  var3 = (undefined2)((ulong)in_struct_1 >> 0x10);
  local_struct_1 = (astruct_2 *)in_struct_1;
  CheckRadioButton16(param_2,local_struct_1->field_0xa0,0x18d,0x188);
  local_struct_1->field_0x9c = 0x0;
  local_struct_1->field_0x9e = 0x0;
  HVar3 = (HWND16)s_tile2_bmp_1050_1538;
  LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1900409);
  if (LVar4 != -0x1) {
    HVar3 = 0x1018;
    uVar2 = pass1_1018_3ab2(local_struct_1->field_0x8e,(int)LVar4,local_struct_1->field_0xa0,unaff_SS);
    local_struct_1->field_0x9e = uVar2;
  }
  SetDlgItemInt16(HVar3,0x0,local_struct_1->field_0x9c,0x18e);
  HVar3 = (HWND16)s_tile2_bmp_1050_1538;
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,local_struct_1->field_0x9e,0x191);
  UVar1 = local_struct_1->field_0xa0;
  if (UVar1 - 0x188 < 0x6) {
    HVar3 = (HWND16)&PTR_LOOP_1050_1040;
    switch(UVar1) {
    case 0x188:
      local_struct_1->field_0xa4 = 0x5;
      break;
    case 0x189:
      local_struct_1->field_0xa4 = 0x6;
      break;
    case 0x18a:
      local_struct_1->field_0xa4 = 0x7;
      break;
    case 0x18b:
      local_struct_1->field_0xa4 = 0x8;
      break;
    case 0x18c:
      local_struct_1->field_0xa4 = 0x9;
      break;
    case 0x18d:
      local_struct_1->field_0xa4 = 0xa;
    }
  }
  invalidate_rect_1040_3ddc(in_struct_1,HVar3);
  return;
}



void __stdcall16far pass1_1040_40e2(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x4466;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far win_ui_op_1040_410e(astruct_1 *param_1,ushort param_2,uchar *param_3)

{
  undefined4 uVar1;
  uchar *puVar2;
  int iVar3;
  RECT16 *unaff_DI;
  undefined2 uVar4;
  undefined2 uVar5;
  HWND16 hwnd;
  undefined in_AF;
  ushort *puVar6;
  int *piVar7;
  int *piVar8;
  uchar *puVar9;
  int local_36;
  int local_34;
  int local_32;
  undefined local_30 [0x6];
  int local_2a [0x4];
  undefined4 uStack34;
  undefined4 local_1e;
  undefined4 uStack26;
  RECT16 local_16;
  int iStack18;
  int iStack16;
  HWND16 HStack14;
  uchar local_c [0xa];
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_3,local_c),(WNDCLASS16 *)0x0,0xa);
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar3 + 0x8e);
  uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
  sys_1000_3f9c(local_c,param_3,0x5d38,(ushort)&USHORT_1050_1050,(ushort)*(undefined4 *)((int)uVar1 + 0x76),&stack0xfffe
                ,uVar5,0x1000,param_3,in_AF);
  HStack14 = GetDlgItem16(0x1000,0xfb5);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_c,(WPARAM16)param_3,0xc0000);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_16);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_3,&local_1e),(WNDCLASS16 *)0x0,0x8);
  uVar1 = *(undefined4 *)(iVar3 + 0x8e);
  hwnd = 0x1010;
  uStack34 = (undefined4 *)pass1_1010_5f7a((int)uVar1,(ushort)((ulong)uVar1 >> 0x10),0x0,0x7);
  if (uStack34 != (undefined4 *)0x0) {
    local_1e = *uStack34;
    unaff_DI = &local_16;
    uStack26 = *(undefined4 *)((int)uStack34 + 0x4);
  }
  if ((local_1e._2_2_ == 0x0) && ((BOOL16)local_1e == 0x0)) {
    puVar6 = pass1_1008_3e38((ushort *)CONCAT22(param_3,local_30));
    puVar2 = (uchar *)((ulong)puVar6 >> 0x10);
    uVar1 = *(undefined4 *)(iVar3 + 0x96);
    pass1_1018_2678((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),(ushort *)CONCAT22(param_3,local_30));
    pass1_1008_3e94((ushort *)CONCAT22(param_3,local_30),(ushort *)CONCAT22(param_3,&local_32),
                    (ushort *)CONCAT22(param_3,local_2a));
    piVar8 = &local_34;
    piVar7 = &local_36;
    puVar9 = param_3;
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,(ushort)param_3,puVar2,(int)unaff_DI);
    hwnd = 0x1008;
    pass1_1008_3e94((ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)),
                    (ushort *)CONCAT22(param_3,piVar7),(ushort *)CONCAT22(puVar9,piVar8));
    uStack26 = CONCAT22(iStack16 - local_16.y,iStack18 - local_16.x);
    local_1e = CONCAT22(((*(int *)((int)puVar6 + 0xc) * -0x14) / 0x258 - (iStack16 - local_16.y)) + local_36 + local_32,
                        local_34 + local_2a[0]);
  }
  move_win_1040_826c(param_1,local_1e._2_2_,(BOOL16)local_1e);
  ShowWindow16(hwnd,0x5);
  return;
}



void __stdcall16far win_ui_op_1040_42b2(ulong param_1,int param_2,HWND16 param_3,WORD *param_4)

{
  undefined4 uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  undefined2 uVar5;
  LRESULT LVar6;
  CHAR local_54 [0x52];
  
  iVar4 = (int)param_1;
  uVar5 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    *(undefined2 *)(iVar4 + 0x9a) = 0x1;
    DestroyWindow16(param_3);
    return;
  }
  pass1_1000_4906((astruct_20 *)CONCAT22(param_4,local_54),(WNDCLASS16 *)0x0,0x51);
  GetDlgItem16(0x1000,0xfb5);
  LVar6 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_54,(WPARAM16)param_4,0xd0051);
  uVar3 = (uint)((ulong)LVar6 >> 0x10);
  uVar2 = pass1_1000_3e2c(CONCAT22(param_4,local_54));
  if ((uVar3 | uVar2) != 0x0) {
    *(uint *)(iVar4 + 0x92) = uVar2;
    *(uint *)(iVar4 + 0x94) = uVar3;
  }
  if ((int)uVar3 < 0x0) {
    wsprintf16((LPSTR)&PTR_LOOP_1050_1000,local_54,param_4);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_54,(WPARAM16)param_4,0xc0000);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
    return;
  }
  GetDlgItem16(0x1000,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  uVar1 = *(undefined4 *)(iVar4 + 0x8e);
  *(undefined4 *)((int)uVar1 + 0x76) = *(undefined4 *)(iVar4 + 0x92);
  uVar1 = *(undefined4 *)(iVar4 + 0x92);
  PostMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)uVar1,(WPARAM16)((ulong)uVar1 >> 0x10),0x4000000);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  return;
}



void __stdcall16far pass1_1040_477e(astruct_1 *param_1,uchar *param_2,ushort param_3,ushort param_4)

{
  uchar *puVar1;
  UINT16 *pUVar2;
  uchar *puVar3;
  uchar *puVar4;
  int unaff_DI;
  ushort *puVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  
  unk_win_ui_op_1040_b230(param_1,param_3,param_4);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,param_2,unaff_DI);
  puVar3 = (uchar *)((ulong)puVar5 >> 0x10);
  uVar7 = SUB42(&USHORT_1050_1050,0x0);
  uVar6 = 0x5d68;
  puVar1 = pass1_1008_5fd8(param_4,puVar3);
  puVar4 = puVar3;
  pUVar2 = pass1_1000_3cea(CONCAT22(puVar3,puVar1),CONCAT22(uVar7,uVar6));
  pass1_1010_e964(puVar4,param_4,unaff_DI);
  pass1_1000_3cea(CONCAT22(puVar3,puVar1),CONCAT22(puVar4,pUVar2));
  unk_str_op_1000_3d3e
            ((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)),(char *)CONCAT22(puVar3,puVar1));
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puVar3,puVar1),0x1000);
  return;
}



void __stdcall16far set_win_pos_1040_4ae4(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  astruct_18 *paVar4;
  uchar *in_DX;
  uchar *puVar5;
  uchar *puVar6;
  int iVar7;
  int unaff_DI;
  undefined2 uVar8;
  ushort unaff_SS;
  RECT16 local_24;
  int iStack32;
  astruct_18 *paStack20;
  astruct_18 *paStack16;
  int iStack12;
  astruct_18 *paStack10;
  astruct_20 *paStack6;
  
  if (param_4._2_2_ == 0xeb) {
    paStack6 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
    puVar5 = (uchar *)((ulong)paStack6 >> 0x10);
    paVar4 = *(astruct_18 **)(param_1 + 0x90);
    if (paVar4 != (astruct_18 *)0x0) {
      paStack10 = paVar4;
      mem_op_1000_179c(0x18,puVar5,0x1000);
      uVar3 = (uint)paVar4;
      paStack16 = (astruct_18 *)((ulong)paVar4 & 0xffff | ZEXT24(puVar5) << 0x10);
      puVar6 = (uchar *)((uint)puVar5 | uVar3);
      if (puVar6 == (uchar *)0x0) {
        uVar3 = 0x0;
        puVar6 = (uchar *)0x0;
      }
      else {
        struct_1040_a598((ushort *)((ulong)paVar4 & 0xffff | ZEXT24(puVar5) << 0x10));
      }
      *(uint *)(param_1 + 0x90) = uVar3;
      *(uchar **)(param_1 + 0x92) = puVar6;
      *(undefined2 *)*(undefined4 *)(param_1 + 0x90) = 0x7;
      iStack12 = **(int **)(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,puVar6,0x1000);
      paStack16 = (astruct_18 *)CONCAT22(puVar6,uVar3);
      if (((uint)puVar6 | uVar3) == 0x0) {
        uVar2 = *(undefined4 *)(param_1 + 0x90);
        *(undefined4 *)((int)uVar2 + 0x2) = 0x0;
      }
      else {
        *(int *)paStack16 = iStack12;
        pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,(ushort)puVar6);
        uVar2 = *(undefined4 *)(param_1 + 0x90);
        uVar8 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar7 = (int)uVar2;
        *(int *)(iVar7 + 0x2) = uVar3 + 0x2;
        *(uchar **)(iVar7 + 0x4) = puVar6;
      }
      uVar8 = (undefined2)((ulong)paStack10 >> 0x10);
      iVar7 = (int)paStack10;
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined4 *)((int)uVar2 + 0x6) = *(undefined4 *)(iVar7 + 0x6);
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar2 + 0xa) = *(undefined2 *)(iVar7 + 0xa);
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar2 + 0x12) = *(undefined2 *)(iVar7 + 0x12);
      pass1_1010_a50c(paStack6,0x10505d6a,*(ulong *)(param_1 + 0x90));
      paStack20 = paStack10;
      paStack16 = paStack10;
      if (paStack10 != (astruct_18 *)0x0) {
        pass1_1040_a5d0((ulong)paStack10);
        fn_ptr_1000_17ce(paStack10,0x1000);
      }
      ppcVar1 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar1)();
    }
  }
  else {
    if (param_4._2_2_ != 0x1770) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
      return;
    }
    if ((int)param_4 == 0x7) {
      GetWindowRect16(param_5,&local_24);
      iStack32 = iStack32 - local_24.x;
      SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack32,0x0,0x0,0x0);
    }
  }
  return;
}


void __stdcall16far pass1_1040_2464(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x2956;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far show_win_1040_2490(astruct_1 *param_1,HWND16 param_2)

{
  code **ppcVar1;
  undefined2 uVar2;
  astruct_1 *iVar4;
  undefined2 uVar3;
  int *piVar4;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_1 *)param_1;
  GetDlgItem16(param_2,0xfb1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
                    // WARNING: Load size is inaccurate
  ppcVar1 = (code **)((int)*iVar4->field_0x8e + 0x10);
  piVar4 = (int *)(**ppcVar1)((int)s_tile2_bmp_1050_1538,*(undefined4 *)&iVar4->field_0x8e);
  uVar2 = (undefined2)((ulong)piVar4 >> 0x10);
  move_win_1040_826c(param_1,*(int *)((int)piVar4 + 0x2) + -0x2,*(int *)((int)piVar4 + 0x4) + *piVar4 + 0x3);
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  pass1_1018_1c9a(*(ulong *)&iVar4->field_0x8e,0x1a0);
  return;
}



ulong __stdcall16far
win_ui_op_1040_2512(ulong *param_1,ulong param_2,UINT16 param_3,HWND16 param_4,WNDCLASS16 *param_5,uchar *param_6)

{
  int *piVar1;
  code **ppcVar2;
  uint uVar3;
  BOOL16 BVar4;
  int iVar5;
  int iVar6;
  UINT16 UVar7;
  uchar *puVar8;
  int unaff_DI;
  undefined2 uVar9;
  undefined2 uVar10;
  HWND16 hwnd;
  undefined in_AF;
  undefined4 uVar11;
  undefined local_1e [0x4];
  uint uStack26;
  uchar *puStack24;
  ushort *local_16 [0x2];
  ushort uStack12;
  undefined4 *puStack10;
  BOOL16 BStack6;
  uchar *puStack4;
  
  BStack6 = 0x0;
  puStack4 = (uchar *)0x0;
  if (param_3 == 0x2) {
LAB_1040_266d:
    BStack6 = 0x1;
    puStack4 = (uchar *)0x0;
  }
  else {
    uVar9 = (undefined2)((ulong)param_1 >> 0x10);
    if (0x19d < param_3 - 0x2) {
      iVar5 = (int)param_1;
      if (param_3 - 0x1a0 < 0x14 || param_3 == 0x1b4) {
        UVar7 = IsDlgButtonChecked(param_4,param_3);
        if (UVar7 == 0x0) {
          piVar1 = (int *)(iVar5 + 0x92);
          *piVar1 = *piVar1 + 0x1;
          if (0x0 < *(int *)(iVar5 + 0x92)) {
            *(undefined2 *)(iVar5 + 0x94) = 0x0;
          }
          uVar11 = *(undefined4 *)(iVar5 + 0x8e);
          if (*(int *)((int)uVar11 + 0x28) == *(int *)(iVar5 + 0x92)) {
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
          }
        }
        else {
          piVar1 = (int *)(iVar5 + 0x92);
          *piVar1 = *piVar1 + -0x1;
          GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
          BVar4 = IsWindowEnabled16((HWND16)s_tile2_bmp_1050_1538);
          if (BVar4 == 0x0) {
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
          }
          if (*(int *)(iVar5 + 0x92) < 0x1) {
            *(undefined2 *)(iVar5 + 0x94) = 0x1;
          }
          pass1_1018_1c9a(*(ulong *)(iVar5 + 0x8e),param_3);
          puStack10 = (undefined4 *)pass1_1018_1e78(*(ulong *)(iVar5 + 0x8e),-0x1);
          uVar3 = (uint)((ulong)puStack10 >> 0x10);
          if (puStack10 == (undefined4 *)0x0) {
            uStack12 = 0x0;
          }
          else {
            uStack12 = *(ushort *)((uint)puStack10 + 0x1c);
          }
          win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uStack12,0x1),param_5,uStack12,uVar3 | (uint)puStack10);
        }
        if ((-0x1 < *(int *)(iVar5 + 0x92)) &&
           (uVar11 = *(undefined4 *)(iVar5 + 0x8e), *(int *)(iVar5 + 0x92) <= *(int *)((int)uVar11 + 0x28))) {
          sys_1000_3f9c((uchar *)local_16,(uchar *)param_5,0x5cf4,(ushort)&USHORT_1050_1050,*(ushort *)(iVar5 + 0x92),
                        &stack0xfffe,uVar9,0x1000,param_5,in_AF);
          SetDlgItemText16(0x1000,(INT16)local_16,(SEGPTR)param_5);
        }
        goto LAB_1040_266d;
      }
      uVar3 = param_3 - 0xfb1;
      if (uVar3 == 0x0) {
        if (*(int *)(iVar5 + 0x92) < 0x0) {
          mem_op_1000_179c(0xb4,param_6,0x1000);
          puVar8 = (uchar *)((uint)param_6 | uVar3);
          uStack26 = uVar3;
          puStack24 = param_6;
          if (puVar8 == (uchar *)0x0) {
            iVar5 = 0x0;
            puVar8 = (uchar *)0x0;
          }
          else {
            iVar5 = string_1040_8520((astruct_57 *)CONCAT22(param_6,uVar3),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x57b,
                                     0x57c,puVar8,(ushort)param_5);
          }
          puStack10 = (undefined4 *)CONCAT22(puVar8,iVar5);
          ppcVar2 = (code **)((int)*puStack10 + 0x74);
          (**ppcVar2)(0x1000,iVar5,puVar8);
          goto LAB_1040_27c0;
        }
        if (0x0 < *(int *)(iVar5 + 0x92)) {
          mem_op_1000_179c(0xb4,param_6,0x1000);
          puVar8 = (uchar *)((uint)param_6 | uVar3);
          uStack26 = uVar3;
          puStack24 = param_6;
          if (puVar8 == (uchar *)0x0) {
            iVar6 = 0x0;
            puVar8 = (uchar *)0x0;
          }
          else {
            iVar6 = string_1040_8520((astruct_57 *)CONCAT22(param_6,uVar3),(ushort)PTR_LOOP_1050_0396,0x21,0x2,0x57b,
                                     0x57d,puVar8,(ushort)param_5);
          }
          puStack10 = (undefined4 *)CONCAT22(puVar8,iVar6);
          pass1_1008_941a((ushort *)CONCAT22(param_5,local_1e),0x1,0xc2);
          ppcVar2 = (code **)((int)*puStack10 + 0x6c);
          uVar11 = (**ppcVar2)(0x1008,(char)puStack10,(int)((ulong)puStack10 >> 0x10),local_1e,param_5);
          param_6 = (uchar *)((ulong)uVar11 >> 0x10);
          if ((int)uVar11 == 0x2) goto LAB_1040_27c0;
        }
        local_16[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,(ushort)param_5,param_6,unaff_DI);
        param_6 = (uchar *)((ulong)local_16[0] >> 0x10);
        uStack12 = 0x1a0;
        hwnd = 0x1010;
        do {
          UVar7 = IsDlgButtonChecked(hwnd,uStack12);
          if (UVar7 == 0x1) {
            uVar10 = (undefined2)((ulong)local_16[0] >> 0x10);
            iVar6 = (int)local_16[0];
            *(UINT16 *)(iVar6 + *(int *)(iVar6 + 0x56) * 0x2 + 0x4e) = uStack12;
            piVar1 = (int *)(iVar6 + 0x56);
            *piVar1 = *piVar1 + 0x1;
          }
          uStack12 = uStack12 + 0x1;
          hwnd = (HWND16)s_tile2_bmp_1050_1538;
        } while ((int)uStack12 < 0x1b5);
        uVar3 = *(uint *)(iVar5 + 0x92);
        puStack10 = (undefined4 *)((ulong)puStack10 & 0xffff0000 | (ulong)uVar3);
        uVar11 = *(undefined4 *)(iVar5 + 0x8e);
        *(uint *)((int)uVar11 + 0x28) = uVar3;
        param_4 = (HWND16)s_tile2_bmp_1050_1538;
        PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100c8);
        param_3 = 0x1;
      }
    }
    BStack6 = post_win_msg_1040_7b3c(param_1,(ushort)param_2,(ushort)(param_2 >> 0x10),param_3,param_4);
    puStack4 = param_6;
  }
LAB_1040_27c0:
  return CONCAT22(puStack4,BStack6);
}



void __stdcall16far dlg_ui_op_1040_2a64(astruct_1 *param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  astruct_160 *paVar2;
  ushort uVar3;
  uchar *puVar4;
  uchar *puVar5;
  int iVar6;
  undefined2 uVar7;
  HWND16 hwnd;
  HWND16 hwnd_00;
  int iVar8;
  RECT16 local_16;
  undefined2 uStack18;
  undefined2 uStack16;
  int iStack14;
  ulong uStack12;
  undefined4 uStack8;
  int iStack4;
  
  unk_win_ui_op_1040_b230(param_1,param_2,param_3);
  iStack4 = 0x5;
  iVar8 = 0x0;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  uStack12 = struct_op_1030_73a8(*(ulong *)((int)uVar1 + 0x6));
  puVar4 = (uchar *)(uStack12 >> 0x10);
  hwnd = (HWND16)&USHORT_1050_1028;
  PTR_LOOP_1050_5d04 = (undefined *)pass1_1028_4a9a(uStack12,iVar8);
  for (iStack14 = 0x0; iStack14 < iStack4; iStack14 = iStack14 + 0x1) {
    if (iStack14 != 0x0) {
      *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0x0;
    }
    iVar8 = iStack14 * 0xc;
    local_16.x = *(INT16 *)(iVar8 + 0x5cfc);
    local_16.y = *(INT16 *)(iVar8 + 0x5cfe);
    paVar2 = (astruct_160 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    uStack18 = 0x1;
    uStack16 = 0x1;
    MapDialogRect16(hwnd,&local_16);
    hwnd_00 = 0x1000;
    mem_op_1000_179c(0x42,puVar4,0x1000);
    puVar5 = (uchar *)((uint)puVar4 | (uint)paVar2);
    if (puVar5 == (uchar *)0x0) {
      paVar2 = (astruct_160 *)0x0;
      puVar4 = (uchar *)0x0;
    }
    else {
      hwnd_00 = 0x1008;
      pass1_1008_3bd6(paVar2,(ushort)puVar4,0x1,CONCAT22(local_16.x,local_16.y),0x101,0xff0100,
                      CONCAT22(*(undefined2 *)(iVar6 + 0x6),*(undefined2 *)(iVar8 + 0x5d00)),(ushort)puVar5,param_3);
      puVar4 = puVar5;
    }
    uStack8 = CONCAT22(puVar4,paVar2);
    if (PTR_LOOP_1050_5d04 == (undefined *)0x0) {
      hwnd = hwnd_00;
      if ((iStack14 != 0x0) && (((uint)puVar4 | (uint)paVar2) != 0x0)) {
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        EnableWindow16(hwnd_00,0x0);
      }
    }
    else {
      iVar8 = iStack14 * 0xc;
      uVar3 = pass1_1028_4a9a(uStack12,*(int *)(iVar8 + 0x5d02));
      hwnd = (HWND16)&USHORT_1050_1028;
      if (uVar3 != 0x0) {
        *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iVar8) = 0x1;
        uVar1 = *(undefined4 *)(iVar6 + 0x98);
        SetDlgItemText16((HWND16)&USHORT_1050_1028,(INT16)uVar1,(SEGPTR)((ulong)uVar1 >> 0x10));
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
      }
    }
  }
  return;
}



void __stdcall16far win_ui_op_1040_2bb2(int param_1,ushort param_2,ushort param_3,ulong param_4,HWND16 param_5)

{
  uint uVar1;
  uchar *in_DX;
  ushort unaff_SS;
  undefined4 uVar2;
  int iStack8;
  int iStack4;
  
  if (param_4._2_2_ == 0x158) {
    PTR_LOOP_1050_5d04 = (undefined *)(uint)(PTR_LOOP_1050_5d04 == (undefined *)0x0);
    if (PTR_LOOP_1050_5d04 == (undefined *)0x0) {
      for (iStack8 = 0x1; iStack8 < 0x5; iStack8 = iStack8 + 0x1) {
        GetDlgItem16(param_5,*(INT16 *)(iStack8 * 0xc + 0x5d00));
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
        *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
        uVar2 = *(undefined4 *)(param_1 + 0x94);
        param_5 = (HWND16)s_tile2_bmp_1050_1538;
        SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)uVar2,(SEGPTR)((ulong)uVar2 >> 0x10));
      }
      uVar2 = *(undefined4 *)(param_1 + 0x94);
      goto LAB_1040_2ccc;
    }
    for (iStack8 = 0x1; iStack8 < 0x5; iStack8 = iStack8 + 0x1) {
      GetDlgItem16(param_5,*(INT16 *)(iStack8 * 0xc + 0x5d00));
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
      *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
      uVar2 = *(undefined4 *)(param_1 + 0x94);
      param_5 = (HWND16)s_tile2_bmp_1050_1538;
      SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)uVar2,(SEGPTR)((ulong)uVar2 >> 0x10));
    }
  }
  else {
    if (param_4._2_2_ == 0x159) {
      iStack4 = 0x1;
    }
    else {
      if (param_4._2_2_ == 0x15a) {
        iStack4 = 0x2;
      }
      else {
        if (param_4._2_2_ == 0x15b) {
          iStack4 = 0x3;
        }
        else {
          if (param_4._2_2_ != 0x15c) {
            pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
            return;
          }
          iStack4 = 0x4;
        }
      }
    }
    if (iStack4 == 0x0) {
      return;
    }
    uVar1 = (uint)(*(int *)((int)&PTR_LOOP_1050_5d04 + iStack4 * 0xc) == 0x0);
    *(uint *)((int)&PTR_LOOP_1050_5d04 + iStack4 * 0xc) = uVar1;
    if (uVar1 == 0x0) {
      uVar2 = *(undefined4 *)(param_1 + 0x94);
      goto LAB_1040_2ccc;
    }
  }
  uVar2 = *(undefined4 *)(param_1 + 0x98);
LAB_1040_2ccc:
  SetDlgItemText16(param_5,(INT16)uVar2,(SEGPTR)((ulong)uVar2 >> 0x10));
  return;
}



void __stdcall16far win_dlg_item_1040_2d48(ulong param_1,HWND16 param_2,BOOL16 param_3)

{
  UINT16 UVar1;
  UINT16 value;
  BOOL16 local_4;
  
  pass1_1040_b45e(param_1,param_2);
  UVar1 = GetDlgItemInt16(param_2,0x1,&local_4,param_3);
  value = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,&local_4,param_3);
  if (UVar1 != 0x0) {
    value = value / UVar1;
  }
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,value,0x165);
  return;
}



void __stdcall16far pass1_1040_2f06(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x3436;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}


void __stdcall16far show_win_1040_2f5a(astruct_1 *param_1,HWND16 param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}


void __stdcall16far win_dlg_op_1040_2f90(ulong param_1,WORD *param_2)

{
  HWND16 HVar1;
  uchar *in_DX;
  uchar *puVar2;
  uint uVar3;
  UINT16 msg;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort *puVar6;
  ulong uVar7;
  char *pcVar8;
  ulong *local_116;
  ulong *local_112;
  CHAR local_10e [0x82];
  undefined local_8c [0x82];
  undefined4 uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_2,in_DX,unaff_DI);
  puVar2 = (uchar *)((ulong)puStack6 >> 0x10);
  uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  GetWindowText16(0x1010,0x80,(INT16)local_8c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_10e,param_2);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_10e);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x182);
  *(HWND16 *)(iVar4 + 0x92) = HVar1;
  pass1_1018_3a94(*(ulong *)(iVar4 + 0x96),(ulong *)CONCAT22(param_2,&local_116),(ulong *)CONCAT22(param_2,&local_112),
                  param_2);
  send_msg_1040_3374(param_1,local_112,*(uint *)(iVar4 + 0x92),0x1018);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,(ushort)param_2,puVar2,unaff_DI);
  uVar3 = (uint)((ulong)puVar6 >> 0x10);
  uVar7 = *(ulong *)((int)puVar6 + 0x24);
  uVar7 = pass1_1018_3a7a(*(ulong *)(iVar4 + 0x96),uVar7,(uint)uVar7,uVar3);
  SendMessage16(0x1018,(UINT16)uVar7,(WPARAM16)(uVar7 >> 0x10),0x40dffff);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x183);
  *(HWND16 *)(iVar4 + 0x94) = HVar1;
  send_msg_1040_3374(param_1,local_116,HVar1,(int)s_tile2_bmp_1050_1538);
  pcVar8 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  msg = (UINT16)((ulong)pcVar8 >> 0x10);
  SendDlgItemMessage16(0x1010,(INT16)pcVar8,msg,0x0,0x1830403);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,(INT16)pcVar8,msg,0xffff,0x183040d);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x181);
  *(HWND16 *)(iVar4 + 0x8e) = HVar1;
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x184);
  *(HWND16 *)(iVar4 + 0x90) = HVar1;
  return;
}



void __stdcall16far win_ui_op_1040_311a(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  int iVar1;
  ulong uVar2;
  undefined *puVar3;
  undefined2 unaff_CS;
  ushort unaff_SS;
  LRESULT LVar4;
  ushort *puVar5;
  int iVar6;
  
  send_msg_1040_323c(CONCAT22(param_2,param_1),unaff_CS);
  load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  if (param_4._2_2_ == 0x181) {
    iVar1 = param_1 + 0x9a;
    puVar3 = (undefined *)param_2;
    iVar6 = iVar1;
    pass1_1018_3cda(*(ulong **)(param_1 + 0x96),(char *)CONCAT22(param_2,param_1 + 0x19a),
                    (char *)CONCAT22(param_2,iVar1));
    pass1_1018_3424(*(ulong *)(param_1 + 0x96),iVar6,(uint)puVar3,unaff_SS);
    if (iVar6 == 0x0) {
      iVar6 = 0x21;
    }
    else {
      pass1_1018_3a42(*(ulong *)(param_1 + 0x96),CONCAT22(param_2,iVar1),puVar3,unaff_SS);
      pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),CONCAT22(puVar3,iVar6));
      uVar2 = *(ulong *)(iVar6 + 0x10);
      pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uVar2);
      PTR_LOOP_1050_5f0c = (undefined *)uVar2;
      PTR_LOOP_1050_5f10 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
      iVar6 = 0x25;
      PTR_LOOP_1050_5f0e = puVar3;
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x8),iVar6,(ushort)puVar3,param_1,
                    (ushort)&PTR_LOOP_1050_1038,unaff_SS);
    LVar4 = SendMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
    iVar6 = 0x1;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,(uchar *)((ulong)LVar4 >> 0x10),param_2);
    pass1_1010_038e((ulong)puVar5,iVar6,unaff_SS);
  }
  else {
    if ((param_4._2_2_ == 0x181) || (0x1 < param_4._2_2_ - 0x182U)) {
      post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,0x1010);
      return;
    }
    set_win_pos_1040_331a(CONCAT22(param_2,param_1),param_3,(ushort)param_4,0x1010);
  }
  return;
}



void __stdcall16far enable_win_1040_32a8(ulong param_1)

{
  SEGPTR lp_string;
  BOOL16 BVar1;
  ushort unaff_SS;
  ulong uStack12;
  
  lp_string = (int)param_1 + 0x19a;
  uStack12 = param_1 & 0xffff0000 | (ulong)lp_string;
  pass1_1018_3a5c(*(ulong *)((int)param_1 + 0x96),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x9aU),
                  param_1 & 0xffff0000 | (ulong)lp_string,unaff_SS);
  SetWindowText16(0x1018,lp_string);
  BVar1 = string_1018_39d8(unaff_SS,*(ulong *)((int)param_1 + 0x96),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x9aU)
                           ,uStack12);
  EnableWindow16(0x1018,BVar1 & 0x1);
  return;
}


BOOL16 __stdcall16far set_win_pos_1040_331a(ulong param_1,ushort param_2,int param_3,HWND16 param_4)

{
  RECT16 local_e;
  int iStack10;
  ushort uStack6;
  int iStack4;
  
  iStack4 = param_3;
  uStack6 = param_2;
  if (param_3 == 0x1) {
    enable_win_1040_32a8(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16(param_4,&local_e);
    iStack10 = iStack10 - local_e.x;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack10,0x0,0x0,0x0);
  }
  return 0x1;
}



void __stdcall16far pass1_1040_3506(astruct_18 *param_1)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = (int)s_Null_Ptr_1050_38f3 + 0x7;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far show_win_1040_355a(astruct_1 *param_1,HWND16 param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}


void __stdcall16far set_win_text_1040_3590(ulong param_1,WORD *param_2)

{
  HWND16 HVar1;
  SEGPTR lp_string;
  uint lp_string_00;
  uchar *in_DX;
  uint uVar2;
  int iVar3;
  int unaff_DI;
  undefined2 uVar4;
  undefined local_59a [0x4];
  undefined local_596 [0x4];
  BOOL16 BStack1426;
  uint uStack1424;
  CHAR local_58e [0x82];
  CHAR local_50c [0x100];
  undefined4 uStack1036;
  ushort *puStack1032;
  char local_404 [0x402];
  
  puStack1032 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_2,in_DX,unaff_DI);
  uVar2 = (uint)((ulong)puStack1032 >> 0x10);
  uStack1036 = *(undefined4 *)((int)puStack1032 + 0x68);
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  GetWindowText16(0x1010,0x80,(INT16)local_50c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_58e,param_2);
  BStack1426 = SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_58e);
  sprintf_op_1018_34b6(*(ulong *)(iVar3 + 0x8e),(uchar)BStack1426);
  uStack1424 = uVar2;
  pass1_1018_3d44(*(ulong *)(iVar3 + 0x8e),(ulong *)CONCAT22(param_2,local_59a),(ulong *)CONCAT22(param_2,local_596));
  HVar1 = GetDlgItem16(0x1018,0x193);
  *(HWND16 *)(iVar3 + 0x98) = HVar1;
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
             (short)param_2);
  wsprintf16((LPSTR)0x1010,local_50c,param_2);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x195);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_50c);
  lp_string = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x196);
  sprintf_op_1018_34b6(*(ulong *)(iVar3 + 0x8e),(uchar)lp_string);
  SetWindowText16(0x1018,lp_string);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x197);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
             (short)param_2);
  SetWindowText16(0x1010,(SEGPTR)local_404);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
             (short)param_2);
  wsprintf16((LPSTR)0x1010,local_50c,param_2);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x198);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_50c);
  lp_string_00 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x199);
  unk_str_op_1018_35b0(*(ulong *)(iVar3 + 0x8e),(ushort)param_2,lp_string_00);
  if ((uVar2 | lp_string_00) == 0x0) {
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
               (short)param_2);
    SetWindowText16(0x1010,(SEGPTR)local_404);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x19a);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
               (short)param_2);
    SetWindowText16(0x1010,(SEGPTR)local_404);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    return;
  }
  SetWindowText16(0x1018,lp_string_00);
  return;
}



void __stdcall16far pass1_1040_0e86(astruct_18 *param_1,ushort param_2)

{
  uint uVar1;
  astruct_18 *paVar2;
  uchar *puVar3;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  param_1->field_0x0 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1040;
  paVar2 = *(astruct_18 **)(iVar4 + 0x92);
  uVar1 = *(uint *)(iVar4 + 0x94);
  puVar3 = (uchar *)(uVar1 | (uint)paVar2);
  if (puVar3 != (uchar *)0x0) {
    pass1_1040_a5d0((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  PTR_LOOP_1050_5b82 = (undefined *)*(undefined2 *)(iVar4 + 0x96);
  if (*(long *)(iVar4 + 0x92) == 0x0) {
    uVar6 = SUB42(&PTR_LOOP_1050_1038,0x0);
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)(iVar4 + 0x6));
  }
  else {
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_2,puVar3,unaff_DI);
    uVar6 = 0x1010;
    pass1_1010_7b8c((ulong)puVar7,*(int *)(iVar4 + 0x6),param_2);
  }
  ui_cleanup_op_1040_782c(param_1,uVar6);
  return;
}



void __stdcall16far set_win_pos_1040_0f10(HWND16 param_1,ushort param_2,int param_3)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int unaff_DI;
  undefined2 uVar4;
  ulong uVar5;
  ushort *puVar6;
  UINT16 check;
  
  dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_3 + 0x6),param_1);
  uVar2 = *(undefined4 *)(param_3 + 0x6);
  uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar3 = (int)uVar2;
  if (*(int *)(iVar3 + 0x98) == 0x0) {
    GetWindowRect16(param_1,(RECT16 *)(param_3 + -0x24));
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1830);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)(param_3 + -0x2c));
    piVar1 = (int *)(param_3 + -0x20);
    *piVar1 = *piVar1 - *(int *)(param_3 + -0x24);
    iVar3 = (*(int *)(param_3 + -0x2a) - *(int *)(param_3 + -0x22)) + -0x2;
    *(int *)(param_3 + -0x1e) = iVar3;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x6,iVar3,*(INT16 *)(param_3 + -0x20),0x0,0x0,0x0);
    CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,0x1,0x1c1);
    uVar2 = *(undefined4 *)(param_3 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x8e);
    *(undefined2 *)((int)uVar2 + 0xa) = 0x2;
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1830);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  }
  else {
    uVar2 = *(undefined4 *)(iVar3 + 0x92);
    uVar5 = struct_op_1030_73a8(*(ulong *)((int)uVar2 + 0x6));
    *(undefined2 *)(param_3 + -0x32) = (int)uVar5;
    *(undefined2 *)(param_3 + -0x30) = (int)(uVar5 >> 0x10);
    uVar2 = *(undefined4 *)(param_3 + -0x32);
    if (*(int *)((int)uVar2 + 0x20) == 0x2) {
      check = 0x1c1;
    }
    else {
      check = 0x1c2;
    }
    CheckDlgButton16(0x1030,0x1,check);
  }
  GetCursorPos16((POINT16 *)s_tile2_bmp_1050_1538);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)(param_3 + -0xc));
  iVar3 = *(int *)(param_3 + -0x8) - *(int *)(param_3 + -0xc);
  *(int *)(param_3 + -0x12) = iVar3;
  *(int *)(param_3 + -0xe) = -(iVar3 / 0x2 - *(int *)(param_3 + -0x4));
  iVar3 = *(int *)(param_3 + -0x6) - *(int *)(param_3 + -0xa);
  *(int *)(param_3 + -0x14) = iVar3;
  *(int *)(param_3 + -0x10) = -(iVar3 / 0x2 - *(int *)(param_3 + -0x2));
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,(uchar *)(iVar3 >> 0xf),unaff_DI);
  uVar4 = (undefined2)((ulong)puVar6 >> 0x10);
  iVar3 = (int)puVar6;
  *(int *)(param_3 + -0x1c) = iVar3;
  *(undefined2 *)(param_3 + -0x1a) = uVar4;
  *(undefined2 *)(param_3 + -0x16) = *(undefined2 *)(iVar3 + 0xa);
  *(undefined2 *)(param_3 + -0x18) = *(undefined2 *)(iVar3 + 0xc);
  if (*(int *)(param_3 + -0x16) < *(int *)(param_3 + -0x12) + *(int *)(param_3 + -0xe)) {
    *(int *)(param_3 + -0xe) = *(int *)(param_3 + -0x16) - *(int *)(param_3 + -0x12);
  }
  if (*(int *)(param_3 + -0x18) < *(int *)(param_3 + -0x14) + *(int *)(param_3 + -0x10)) {
    *(int *)(param_3 + -0x10) = *(int *)(param_3 + -0x18) - *(int *)(param_3 + -0x14);
  }
  uVar2 = *(undefined4 *)(param_3 + -0x10);
  SetWindowPos16(0x1010,0x45,0x0,0x0,(INT16)uVar2,(INT16)((ulong)uVar2 >> 0x10),0x0);
  return;
}



void __stdcall16far pass1_1040_1290(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x17b0;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}




void __stdcall16far win_ui_op_1040_12bc(astruct_1 *param_1,ushort param_2,uchar *param_3)

{
  undefined4 uVar1;
  WPARAM16 wparam;
  HWND16 HVar2;
  undefined2 uVar3;
  undefined in_AF;
  char *pcVar4;
  uchar local_54 [0x52];
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
  uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
  sys_1000_3f9c(local_54,param_3,0x5cd4,(ushort)&USHORT_1050_1050,*(ushort *)((int)uVar1 + 0xa),&stack0xfffe,uVar3,
                0x1000,param_3,in_AF);
  GetDlgItem16(0x1000,0xfd2);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_54,(WPARAM16)param_3,0xc0000);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  move_win_1040_826c(param_1,-0x1,0xffff);
  pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  wparam = (WPARAM16)((ulong)pcVar4 >> 0x10);
  HVar2 = GetDlgItem16(0x1010,(int)s_vrpal_bmp_1050_183a + 0x5);
  send_msg_1040_1696((ulong)param_1,HVar2,(ushort)param_3,(int)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)pcVar4,wparam,0x40dffff);
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x4);
  send_msg_1040_1696((ulong)param_1,HVar2,(ushort)param_3,(int)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)pcVar4,wparam,0x40dffff);
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  return;
}




void __stdcall16far win_msg_op_1040_13b2(ulong param_1,int param_2,HWND16 param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  int iVar4;
  undefined *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined in_AF;
  LRESULT LVar11;
  undefined4 *puStack562;
  undefined local_22e [0x118];
  ulong uStack278;
  ulong uStack274;
  undefined *puStack270;
  uchar *puStack268;
  undefined4 uStack266;
  uint uStack262;
  char *pcStack260;
  undefined local_100 [0x52];
  int iStack174;
  HWND16 HStack172;
  uchar local_aa [0x52];
  uint uStack88;
  HWND16 HStack86;
  undefined local_54 [0x52];
  
  iVar4 = (int)param_1;
  uVar9 = (undefined2)(param_1 >> 0x10);
  if (param_2 != 0x0) {
    HStack86 = GetDlgItem16(param_3,0xfd2);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_54,param_4,0xd0051);
    uStack88 = pass1_1000_3e2c(CONCAT22(param_4,local_54));
    HStack172 = GetDlgItem16(0x1000,(int)s_vrpal_bmp_1050_183a + 0x4);
    LVar11 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
    iStack174 = (int)LVar11;
    if (iStack174 != -0x1) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_aa,param_4,CONCAT22(0x408,iStack174));
    }
    HStack172 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x5);
    LVar11 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
    iStack174 = (int)LVar11;
    if (iStack174 != -0x1) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_100,param_4,CONCAT22(0x408,iStack174));
    }
    pcStack260 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    puVar6 = local_aa;
    uVar3 = pass1_1000_3d7a(CONCAT22(param_4,puVar6),CONCAT22(param_4,local_100));
    if (uVar3 != 0x0) {
      uVar3 = pass1_1000_3d7a(CONCAT22(param_4,local_aa),(ulong)pcStack260);
      if (uVar3 != 0x0) {
        uVar3 = pass1_1000_3d7a(CONCAT22(param_4,local_100),(ulong)pcStack260);
        if (uVar3 != 0x0) {
          pass1_1010_531c(*(ulong *)(iVar4 + 0x8e),CONCAT22(param_4,local_aa),(ushort)local_aa,puVar6,param_4);
          puVar5 = local_100;
          pass1_1010_52fc(*(ulong *)(iVar4 + 0x8e),CONCAT22(param_4,puVar5),(ushort)puVar5,puVar6,param_4);
          pass1_1010_5120(*(ulong *)(iVar4 + 0x8e),uStack88,(uint)puVar5,(uint)puVar6,param_4);
          uStack266 = (undefined4 *)CONCAT22(uStack266._2_2_,puVar5);
          if (puVar5 == (undefined *)0x0) {
            mem_op_1000_179c(0xb4,puVar6,0x1000);
            puVar7 = (uchar *)((uint)puVar6 | (uint)puVar5);
            puStack270 = puVar5;
            puStack268 = puVar6;
            if (puVar7 == (uchar *)0x0) {
              iVar4 = 0x0;
              puVar7 = (uchar *)0x0;
            }
            else {
              iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puVar6,puVar5),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x57b,
                                       0x7d2,puVar7,param_4);
            }
            puStack562 = (undefined4 *)CONCAT22(puVar7,iVar4);
            ppcVar1 = (code **)((int)*puStack562 + 0x74);
            (**ppcVar1)(0x1000,iVar4,puVar7);
            return;
          }
          uVar2 = *(undefined4 *)(iVar4 + 0x8e);
          uStack274 = *(ulong *)((int)uVar2 + 0x12);
          uVar2 = *(undefined4 *)(iVar4 + 0x8e);
          uVar10 = (undefined2)((ulong)uVar2 >> 0x10);
          iVar8 = (int)uVar2;
          uStack278 = *(ulong *)(iVar8 + 0x16);
          uVar2 = *(undefined4 *)(iVar4 + 0x8e);
          uStack262 = *(uint *)((int)uVar2 + 0xa);
          pass1_1028_8d9e((astruct_100 *)CONCAT22(param_4,local_22e),(ulong)uStack262,uStack274,
                          uStack278 & 0xffff | (ulong)*(uint *)(iVar8 + 0x18) << 0x10,param_4,in_AF);
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_22e));
          param_3 = (HWND16)&USHORT_1050_1028;
          pass1_1028_8dec((ushort *)CONCAT22(param_4,local_22e));
          goto LAB_1040_1619;
        }
      }
    }
    param_3 = 0x1000;
    mem_op_1000_179c(0xb4,puVar6,0x1000);
    puVar7 = (uchar *)((uint)puVar6 | uVar3);
    puStack270 = (undefined *)uVar3;
    puStack268 = puVar6;
    if (puVar7 == (uchar *)0x0) {
      iVar4 = 0x0;
      puVar7 = (uchar *)0x0;
    }
    else {
      iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puVar6,uVar3),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x755,
                               puVar7,param_4);
    }
    uStack266 = (undefined4 *)CONCAT22(puVar7,iVar4);
    ppcVar1 = (code **)((int)*uStack266 + 0x74);
    (**ppcVar1)(0x1000,iVar4,puVar7);
  }
LAB_1040_1619:
  DestroyWindow16(param_3);
  return;
}



ulong __stdcall16far set_win_pos_1040_162a(ushort param_1,ulong param_2,ulong param_3,ushort param_4,HWND16 param_5)

{
  uint uVar1;
  BOOL16 BVar2;
  RECT16 local_a;
  int iStack6;
  
  if ((param_3._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x5) && (param_3._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x4)) {
    BVar2 = post_win_msg_1040_7b3c
                      ((ulong *)CONCAT22((int)param_2,param_1),param_2._2_2_,(ushort)param_3,param_3._2_2_,param_5);
    return CONCAT22(param_4,BVar2);
  }
  if ((ushort)param_3 == 0x7) {
    GetWindowRect16(param_5,&local_a);
    iStack6 = iStack6 - local_a.x;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack6,0x0,0x0,0x0);
  }
  else {
    if (((ushort)param_3 != 0x9) && ((ushort)param_3 != 0xa)) {
      uVar1 = 0x0;
      goto LAB_1040_164d;
    }
  }
  uVar1 = 0x1;
LAB_1040_164d:
  return (ulong)uVar1;
}



void __stdcall16far pass1_1040_1876(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x1c48;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far show_win_1040_18a2(astruct_1 *param_1,HWND16 param_2,WORD *param_3)

{
  undefined4 uVar1;
  CHAR local_304 [0x100];
  char local_204 [0x100];
  char local_104 [0x100];
  undefined2 uStack4;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  check_dialog_btn_1040_1afe(param_1);
  if (PTR_LOOP_1050_13ae != (undefined *)0x0) {
    if (PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002) {
      uStack4 = 0x621;
    }
    else {
      if (PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
        uStack4 = 0x622;
      }
      else {
        if (PTR_LOOP_1050_13ae == (undefined *)&DAT_1050_0004) {
          uStack4 = 0x623;
        }
        else {
          uStack4 = 0x620;
        }
      }
    }
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,
               (short)param_3);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_204,
               (short)param_3);
    wsprintf16((LPSTR)0x1010,local_304,param_3);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_304,(SEGPTR)param_3);
    uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
    if (*(int *)((int)uVar1 + 0x82) == 0x0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,
               (short)param_3);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_204,
               (short)param_3);
    wsprintf16((LPSTR)0x1010,local_304,param_3);
    param_2 = (HWND16)s_tile2_bmp_1050_1538;
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_304,(SEGPTR)param_3);
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  return;
}



void __stdcall16far unk_win_ui_op_1040_19ea(astruct_32 *param_1,int param_2,HWND16 param_3)

{
  ulong uVar1;
  UINT16 UVar2;
  uchar *in_DX;
  astruct_32 *iVar4;
  int unaff_DI;
  astruct_32 *uVar3;
  ushort unaff_SS;
  
  iVar4 = (astruct_32 *)param_1;
  uVar3 = (astruct_32 *)((ulong)param_1 >> 0x10);
  if (param_2 != 0x0) {
    UVar2 = IsDlgButtonChecked(param_3,0xfdb);
    pass1_1010_5d9c(iVar4->field_0x8e,UVar2,in_DX,unaff_DI,unaff_SS);
    UVar2 = IsDlgButtonChecked(0x1010,0xfdc);
    uVar1 = iVar4->field_0x8e;
    *(UINT16 *)((int)uVar1 + 0x20) = UVar2;
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfdd);
    uVar1 = iVar4->field_0x8e;
    *(UINT16 *)((int)uVar1 + 0x74) = UVar2;
    param_3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfde);
    uVar1 = iVar4->field_0x8e;
    *(UINT16 *)((int)uVar1 + 0x72) = UVar2;
    if (iVar4->field_0x92 != 0x0) {
      uVar1 = iVar4->field_0x8e;
      param_3 = 0x1000;
      pass1_1000_4906((astruct_20 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x22)),(WNDCLASS16 *)0x0,0x40);
    }
    if (iVar4->field_0x94 != 0x0) {
      param_3 = 0x1010;
      pass1_1010_60a0(iVar4->field_0x8e);
    }
  }
  DestroyWindow16(param_3);
  return;
}



ulong __stdcall16far
pass1_1040_1ab0(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6)

{
  BOOL16 BStack6;
  ushort uStack4;
  
  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_4._2_2_ == 0x1831) {
    *(undefined2 *)(param_1 + 0x92) = 0x1;
    *(undefined2 *)(param_1 + 0x94) = 0x1;
    check_dialog_btn_1040_1b8a(param_1,param_2);
  }
  else {
    BStack6 = post_win_msg_1040_7b3c
                        ((ulong *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)),param_3,
                         (ushort)param_4,param_4._2_2_,param_6);
    uStack4 = param_5;
  }
  return CONCAT22(uStack4,BStack6);
}




void __stdcall16far check_dialog_btn_1040_1afe(undefined4 param_1)

{
  INT16 id;
  INT16 id_00;
  INT16 id_01;
  undefined4 uVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 unaff_CS;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar3 + 0x8e);
  uVar2 = *(undefined4 *)(iVar3 + 0x8e);
  id = *(INT16 *)((int)uVar2 + 0x20);
  uVar2 = *(undefined4 *)(iVar3 + 0x8e);
  id_00 = *(INT16 *)((int)uVar2 + 0x74);
  uVar2 = *(undefined4 *)(iVar3 + 0x8e);
  id_01 = *(INT16 *)((int)uVar2 + 0x72);
  CheckDlgButton16(unaff_CS,*(INT16 *)((int)uVar1 + 0x1e),0xfdb);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_00,0xfdd);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_01,0xfde);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id,0xfdc);
  return;
}



void __stdcall16far check_dialog_btn_1040_1b8a(void)

{
  ushort id;
  ushort id_00;
  ushort id_01;
  ushort id_02;
  
  id = pass1_1010_60b4();
  id_00 = pass1_1010_60c6();
  id_01 = pass1_1010_60c0();
  id_02 = pass1_1010_60ba();
  CheckDlgButton16(0x1010,id,0xfdb);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_01,0xfdd);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_02,0xfde);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_00,0xfdc);
  return;
}




void __stdcall16far pass1_1040_1d24(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x1eee;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}





void __stdcall16far show_win_1040_1d50(astruct_1 *param_1,HWND16 param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  return;
}




void __stdcall16far unk_win_ui_op_1040_1d7a(astruct_33 *param_1,int param_2,HWND16 param_3)

{
  undefined4 uVar1;
  UINT16 UVar2;
  astruct_33 *iVar3;
  astruct_33 *uVar3;
  HWND16 HVar3;
  HWND16 HVar4;
  ushort unaff_SS;
  
  iVar3 = (astruct_33 *)param_1;
  uVar3 = (astruct_33 *)((ulong)param_1 >> 0x10);
  if ((param_2 != 0x0) && (uVar1 = iVar3->field_0x8e, *(int *)((int)uVar1 + 0x72) != 0x0)) {
    HVar3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(param_3,0xe1);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d5,unaff_SS);
    }
    HVar4 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar3,0xe2);
    if (UVar2 != 0x0) {
      HVar4 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d6,unaff_SS);
    }
    HVar3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar4,0xe3);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d7,unaff_SS);
    }
    HVar4 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar3,0xe5);
    if (UVar2 != 0x0) {
      HVar4 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d8,unaff_SS);
    }
    HVar3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar4,0xe6);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1e2,unaff_SS);
    }
    UVar2 = IsDlgButtonChecked(HVar3,0xe7);
    if (UVar2 != 0x0) {
      pass1_1008_a930(iVar3->field_0x92,0x1dc,unaff_SS);
    }
    return;
  }
  DestroyWindow16(param_3);
  return;
}



void __stdcall16far pass1_1040_205e(astruct_18 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_624 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_624 *)param_1;
  param_1->field_0x0 = 0x237e;
  iVar4->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar1 = iVar4->field_0x8e;
  uVar2 = iVar4->field_0x90;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(iVar4->field_0xa2,0x1000);
  fn_ptr_1000_17ce(iVar4->field_0xa6,0x1000);
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar4->field_0x6);
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far create_win_1040_20d4(ushort param_1,ushort param_2,ushort param_3,astruct_1 *param_4)

{
  int y;
  int unaff_DI;
  undefined2 uVar1;
  ushort *puVar2;
  RECT16 local_1e;
  int iStack26;
  int iStack24;
  undefined4 uStack22;
  ulong uStack18;
  int iStack14;
  undefined2 uStack12;
  int iStack10;
  int iStack8;
  undefined2 uStack6;
  int iStack4;
  
  dialog_ui_fn_1040_78e2(param_4,param_2);
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_1,(uchar *)param_3,unaff_DI);
  uStack12 = (undefined2)((ulong)puVar2 >> 0x10);
  iStack14 = (int)puVar2;
  iStack8 = *(int *)(iStack14 + 0xa);
  iStack10 = *(int *)(iStack14 + 0xc);
  uVar1 = (undefined2)((ulong)param_4 >> 0x10);
  uStack18 = pass1_1008_4772(*(astruct_76 **)((int)param_4 + 0x8e));
  y = *(int *)((int)uStack18 + 0x4);
  iStack4 = (iStack8 - y) / 0x2;
  uStack6 = 0x5;
  SetWindowPos16(0x1008,0x6,0x1d6,y,0x5,iStack4,0x0);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_1e);
  load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uStack22 = 0x50010001;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x1,*(INT16 *)((int)param_4 + 0x6),0x19,
                 0x58,iStack24 - 0x28,(iStack26 + -0x58) / 0x2,0x1,(LPVOID)((int)s_Rebel_1050_4ffc + 0x5));
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x45,iStack10 + -0xa,*(INT16 *)((int)uStack18 + 0x4),0x5,iStack4,0x0);
  return;
}



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




