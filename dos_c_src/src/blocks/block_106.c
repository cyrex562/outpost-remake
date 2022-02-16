


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



astruct_18 * __stdcall16far pass1_1040_38d4(astruct_18 *param_1,byte param_2)

{
  pass1_1040_3506(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_3966(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_722 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x185,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_722 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x9a = 0x0;
  iVar1->field_0x9c = 0x0;
  iVar1->field_0x9e = 0x0;
  iVar1->field_0xa0 = 0x0;
  iVar1->field_0xa2 = 0x0;
  iVar1->field_0xa4 = 0x5;
  *(undefined2 *)param_1 = 0x3ffc;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_3a0e(ushort param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  ushort *puVar1;
  int iVar2;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e((ulong)puVar1,iVar2,param_5);
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),0x1010);
  return;
}



ushort __stdcall16far enable_win_1040_3a36(ulong param_1,ushort param_2,ushort param_3,int param_4,HWND16 param_5)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 hwnd;
  HWND16 hwnd_00;
  
  bVar2 = false;
  iVar3 = (int)param_1;
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (param_4 == 0x0) {
    if (*(uint *)(iVar3 + 0x9e) <= *(uint *)(iVar3 + 0x9c)) goto LAB_1040_3a79;
    piVar1 = (int *)(iVar3 + 0x9c);
    *piVar1 = *piVar1 + 0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1040_3a79;
    if (*(int *)(iVar3 + 0x9c) == 0x0) goto LAB_1040_3a79;
    piVar1 = (int *)(iVar3 + 0x9c);
    *piVar1 = *piVar1 + -0x1;
  }
  bVar2 = true;
LAB_1040_3a79:
  hwnd = param_5;
  if (bVar2) {
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    SetDlgItemInt16(param_5,0x0,*(UINT16 *)(iVar3 + 0x9c),0x18e);
  }
  hwnd_00 = hwnd;
  if ((*(int *)(iVar3 + 0x9c) != 0x0) && (*(int *)(iVar3 + 0xa2) == 0x0)) {
    *(undefined2 *)(iVar3 + 0xa2) = 0x1;
    hwnd_00 = (HWND16)s_tile2_bmp_1050_1538;
    EnableWindow16(hwnd,0x1);
  }
  if ((*(int *)(iVar3 + 0x9c) == 0x0) && (*(int *)(iVar3 + 0xa2) != 0x0)) {
    *(undefined2 *)(iVar3 + 0xa2) = 0x0;
    EnableWindow16(hwnd_00,0x0);
  }
  return 0x0;
}



void __stdcall16far show_win_1040_3ae8(astruct_1 *param_1,ushort param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far get_dc_op_1040_3d5e(ulong param_1,HWND16 param_2,ushort param_3)

{
  code **ppcVar1;
  int iVar2;
  undefined2 uVar3;
  astruct_43 *paVar4;
  undefined2 uVar5;
  HDC16 local_4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar5 = *(undefined2 *)((int)param_1 + 0x6);
  local_4 = GetDC16(param_2);
  paVar4 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,*(ushort *)((int)param_1 + 0xa4),param_3);
  iVar2 = (int)*(undefined4 *)paVar4;
  ppcVar1 = (code **)(iVar2 + 0x8);
  (**ppcVar1)(0x1010,(int)paVar4,(int)((ulong)paVar4 >> 0x10),&local_4,param_3,uVar5);
  ppcVar1 = (code **)(iVar2 + 0x4);
  (**ppcVar1)(0x1010,paVar4,0x50078,&local_4,param_3);
  ppcVar1 = (code **)(iVar2 + 0xc);
  (**ppcVar1)(0x1010,paVar4,&local_4,param_3);
  ReleaseDC16(0x1010,local_4);
  return 0x0;
}



void __stdcall16far invalidate_rect_1040_3ddc(astruct_2 *in_struct_1,HWND16 in_win_handle_2)

{
  undefined4 local_b_erase;
  undefined4 uStack6;
  
  local_b_erase = 0x780005;
  uStack6 = 0xdc0069;
  InvalidateRect16(in_win_handle_2,(RECT16 *)0x0,(BOOL16)&local_b_erase);
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



void __stdcall16far
send_dlg_item_msg_1040_3f12(ushort param_1,ushort param_2,ulong param_3,HWND16 param_4,ushort param_5)

{
  undefined4 uVar1;
  undefined *puVar2;
  uint extraout_DX;
  int iVar3;
  HWND16 hwnd;
  LRESULT LVar4;
  undefined local_a [0x8];
  
  SendDlgItemMessage16(param_4,0x0,0x0,0x0,0x190000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1900405);
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_a),param_3);
  while( true ) {
    puVar2 = local_a;
    hwnd = 0x1008;
    pass1_1008_5b12(puVar2,param_5);
    if ((extraout_DX | (uint)puVar2) == 0x0) break;
    uVar1 = *(undefined4 *)(puVar2 + 0x4);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    LVar4 = SendDlgItemMessage16(0x1008,(INT16)uVar1,(UINT16)((ulong)uVar1 >> 0x10),0x0,0x1900401);
    iVar3 = (int)((ulong)LVar4 >> 0x10);
    if ((((int)LVar4 == -0x1) && (iVar3 == -0x1)) || (((int)LVar4 == -0x2 && (iVar3 == -0x1)))) break;
  }
  SendDlgItemMessage16(hwnd,0x0,0x0,0x0,0x1900407);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x190000b);
  return;
}



astruct_18 * __stdcall16far pass1_1040_3fd6(astruct_18 *param_1,byte param_2)

{
  pass1_1040_39e2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_4068(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  uchar *puVar1;
  astruct_723 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfb7,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_723 *)param_1;
  *(undefined4 *)&iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = 0x0;
  iVar2->field_0x9a = 0x0;
  *(undefined2 *)param_1 = 0x4466;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar2->field_0x76 = 0x1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,param_6,param_7);
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  iVar2->field_0x8e = (int)puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_8,puVar1,param_7);
  iVar2->field_0x96 = (int)puVar3;
  iVar2->field_0x98 = (int)((ulong)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



void __stdcall16far get_win_rect_1040_43ea(int param_1,HWND16 param_2,ushort param_3,ushort param_4)

{
  undefined4 uVar1;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  GetWindowRect16(param_2,&local_a);
  iStack6 = iStack6 - local_a.x;
  iStack4 = iStack4 - local_a.y;
  pass1_1010_5fb0(*(ulong *)(param_1 + 0x8e),0x0,(ulong *)&local_a,param_3,0x7);
  uVar1 = *(undefined4 *)(param_1 + 0x8e);
  *(uint *)((int)uVar1 + 0x7a) = (uint)(*(int *)(param_1 + 0x9a) == 0x0);
  return;
}



astruct_18 * __stdcall16far pass1_1040_4440(astruct_18 *param_1,byte param_2)

{
  pass1_1040_40e2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_44d2(astruct_57 *param_1,ulong param_2,ushort param_3,uint param_4,uchar *param_5)

{
  undefined4 uVar1;
  uint uVar2;
  uchar *puVar3;
  int iVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  int *piStack8;
  
  iVar6 = (int)param_1;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  struct_1040_b082(param_1,CONCAT22(param_3,0xfa2));
  *(undefined2 *)param_1 = 0x4824;
  *(undefined2 *)(iVar6 + 0x2) = (int)&PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_5,0x1000);
  puVar3 = (uchar *)((uint)param_5 | param_4);
  if (puVar3 == (uchar *)0x0) {
    *(undefined4 *)(iVar6 + 0x90) = 0x0;
  }
  else {
    struct_1040_a598((ushort *)CONCAT22(param_5,param_4));
    *(uint *)(iVar6 + 0x90) = param_4;
    *(uchar **)(iVar6 + 0x92) = puVar3;
  }
  *(undefined2 *)*(undefined4 *)(iVar6 + 0x90) = 0x14;
  iVar4 = **(int **)(iVar6 + 0x90);
  uVar2 = iVar4 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,puVar3,0x1000);
  piStack8 = (int *)CONCAT22(puVar3,uVar2);
  if (((uint)puVar3 | uVar2) == 0x0) {
    uVar1 = *(undefined4 *)(iVar6 + 0x90);
    *(undefined4 *)((int)uVar1 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar4;
    pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar4,0xa,uVar2 + 0x2,(ushort)puVar3);
    uVar1 = *(undefined4 *)(iVar6 + 0x90);
    uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar4 = (int)uVar1;
    *(int *)(iVar4 + 0x2) = uVar2 + 0x2;
    *(uchar **)(iVar4 + 0x4) = puVar3;
  }
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  *(ulong *)((int)uVar1 + 0x6) = param_2;
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  *(undefined2 *)((int)uVar1 + 0xa) = 0x1;
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  *(undefined2 *)((int)uVar1 + 0x12) = *(undefined2 *)(iVar6 + 0xa);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_45e8(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  astruct_18 *paVar1;
  code **ppcVar2;
  undefined4 uVar3;
  uint uVar4;
  astruct_18 *paVar5;
  uchar *puVar6;
  uchar *puVar7;
  int iVar8;
  int unaff_DI;
  undefined2 uVar9;
  astruct_20 *paVar10;
  int *piStack16;
  
  if (param_4._2_2_ != 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
    return;
  }
  paVar10 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_7,param_5,unaff_DI);
  puVar6 = (uchar *)((ulong)paVar10 >> 0x10);
  paVar1 = *(astruct_18 **)(param_1 + 0x90);
  if (paVar1 != (astruct_18 *)0x0) {
    paVar5 = paVar1;
    mem_op_1000_179c(0x18,puVar6,0x1000);
    uVar4 = (uint)paVar5;
    puVar7 = (uchar *)((uint)puVar6 | uVar4);
    if (puVar7 == (uchar *)0x0) {
      uVar4 = 0x0;
      puVar7 = (uchar *)0x0;
    }
    else {
      struct_1040_a598((ushort *)((ulong)paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
    }
    *(uint *)(param_1 + 0x90) = uVar4;
    *(uchar **)(param_1 + 0x92) = puVar7;
    *(undefined2 *)*(undefined4 *)(param_1 + 0x90) = 0x14;
    iVar8 = **(int **)(param_1 + 0x90);
    uVar4 = iVar8 * 0xa + 0x2;
    mem_op_1000_179c(uVar4,puVar7,0x1000);
    piStack16 = (int *)CONCAT22(puVar7,uVar4);
    if (((uint)puVar7 | uVar4) == 0x0) {
      uVar3 = *(undefined4 *)(param_1 + 0x90);
      *(undefined4 *)((int)uVar3 + 0x2) = 0x0;
    }
    else {
      *piStack16 = iVar8;
      pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar8,0xa,uVar4 + 0x2,(ushort)puVar7);
      uVar3 = *(undefined4 *)(param_1 + 0x90);
      uVar9 = (undefined2)((ulong)uVar3 >> 0x10);
      iVar8 = (int)uVar3;
      *(int *)(iVar8 + 0x2) = uVar4 + 0x2;
      *(uchar **)(iVar8 + 0x4) = puVar7;
    }
    uVar3 = *(undefined4 *)(param_1 + 0x90);
    *(undefined4 *)((int)uVar3 + 0x6) = *(undefined4 *)((int)paVar1 + 0x6);
    uVar3 = *(undefined4 *)(param_1 + 0x90);
    *(undefined2 *)((int)uVar3 + 0xa) = 0x1;
    uVar3 = *(undefined4 *)(param_1 + 0x90);
    *(undefined2 *)((int)uVar3 + 0x12) = *(undefined2 *)(param_1 + 0xa);
    pass1_1010_a50c(paVar10,0x10505d40,*(ulong *)(param_1 + 0x90));
    if (paVar1 != (astruct_18 *)0x0) {
      pass1_1040_a5d0((ulong)paVar1);
      fn_ptr_1000_17ce(paVar1,0x1000);
    }
    ppcVar2 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x70);
    (**ppcVar2)();
  }
  return;
}



void __stdcall16far pass1_1040_4766(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x74);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



astruct_18 * __stdcall16far pass1_1040_47fe(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_48a0(astruct_57 *param_1,ushort param_2,ulong param_3,ushort param_4,uchar *param_5,ushort param_6)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  uchar *puVar4;
  uchar *puVar5;
  astruct_444 *iVar5;
  astruct_445 *iVar6;
  int unaff_DI;
  undefined2 uVar6;
  undefined2 uVar7;
  ushort *puVar8;
  int *piStack8;
  
  struct_1040_b082(param_1,CONCAT22(param_4,0xfa1));
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_444 *)param_1;
  iVar5->field_0x94 = 0x0;
  *(int *)param_1 = (int)&PTR_LOOP_1050_4e18;
  iVar5->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_5,unaff_DI);
  puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
  uVar3 = (uint)puVar8;
  *(uint *)&iVar5->field_0x94 = uVar3;
  *(uchar **)((int)&iVar5->field_0x94 + 0x2) = puVar4;
  mem_op_1000_179c(0x18,puVar4,0x1000);
  puVar5 = (uchar *)((uint)puVar4 | uVar3);
  if (puVar5 == (uchar *)0x0) {
    iVar5->field_0x90 = (int *)0x0;
  }
  else {
    struct_1040_a598((ushort *)CONCAT22(puVar4,uVar3));
    *(uint *)&iVar5->field_0x90 = uVar3;
    *(uchar **)((int)&iVar5->field_0x90 + 0x2) = puVar5;
  }
  *iVar5->field_0x90 = 0x7;
  iVar1 = *iVar5->field_0x90;
  uVar3 = iVar1 * 0xa + 0x2;
  mem_op_1000_179c(uVar3,puVar5,0x1000);
  piStack8 = (int *)CONCAT22(puVar5,uVar3);
  if (((uint)puVar5 | uVar3) == 0x0) {
    piVar2 = iVar5->field_0x90;
    *(undefined4 *)((int)piVar2 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar1;
    pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar1,0xa,uVar3 + 0x2,(ushort)puVar5);
    piVar2 = iVar5->field_0x90;
    uVar7 = (undefined2)((ulong)piVar2 >> 0x10);
    iVar6 = (astruct_445 *)piVar2;
    iVar6->field_0x2 = uVar3 + 0x2;
    iVar6->field_0x4 = puVar5;
  }
  piVar2 = iVar5->field_0x90;
  *(ulong *)((int)piVar2 + 0x6) = param_3;
  piVar2 = iVar5->field_0x90;
  *(ushort *)((int)piVar2 + 0xa) = param_2;
  piVar2 = iVar5->field_0x90;
  *(undefined2 *)((int)piVar2 + 0x12) = iVar5->field_0xa;
  iVar1 = *(int *)&iVar5->field_0x90;
  uVar7 = *(undefined2 *)((int)&iVar5->field_0x90 + 0x2);
  pass1_1010_debe(iVar5->field_0x94,*(ushort *)(iVar1 + 0xa),(ushort *)CONCAT22(uVar7,iVar1 + 0x10),
                  (ulong *)CONCAT22(uVar7,iVar1 + 0xc),param_3,param_6);
  return;
}



LRESULT __stdcall16far send_win_msg_1040_4a0a(astruct_48 **param_1,HWND16 param_2)

{
  int *piVar1;
  code **ppcVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  ushort uVar5;
  astruct_48 *iVar5;
  undefined2 uVar6;
  LRESULT LVar7;
  char *pcVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  int iStack10;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_48 *)param_1;
  ppcVar2 = (code **)((int)*param_1 + 0x74);
  (**ppcVar2)(param_2,param_1,0x5d6a,(int)&USHORT_1050_1050);
  GetDlgItem16(param_2,0x1770);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x40b0000);
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
  uVar5 = (ushort)((ulong)LVar7 >> 0x10);
  for (iStack10 = 0x0; uVar3 = iVar5->field_0x90, piVar1 = (int *)((int)uVar3 + 0x10),
      *piVar1 != iStack10 && iStack10 <= *piVar1; iStack10 = iStack10 + 0x1) {
    uVar10 = 0x0;
    uVar9 = 0x403;
    uVar3 = iVar5->field_0x90;
    uVar3 = *(undefined4 *)((int)uVar3 + 0xc);
    pcVar8 = pass1_1040_4dcc((ulong)param_1,*(int *)((int)uVar3 + iStack10 * 0x2),uVar5);
    LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)pcVar8,(WPARAM16)((ulong)pcVar8 >> 0x10),
                          CONCAT22(uVar9,uVar10));
    uVar5 = (ushort)((ulong)LVar7 >> 0x10);
  }
  pass1_1040_4d7e((ulong)param_1);
  if (iStack10 == 0x0) {
    uVar10 = 0x40a;
    uVar4 = iVar5->field_0x90;
    uVar3 = iVar5->field_0x94;
    pcVar8 = string_op_1010_ada6(0x1010,uVar5,(ushort)uVar3,(ushort)((ulong)uVar3 >> 0x10),0x0,
                                 *(int *)((int)uVar4 + 0xa));
    SendMessage16(0x1010,(UINT16)pcVar8,(WPARAM16)((ulong)pcVar8 >> 0x10),CONCAT22(uVar10,iStack10));
  }
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
  return LVar7;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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
