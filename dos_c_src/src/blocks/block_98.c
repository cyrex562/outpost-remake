


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_88f2(astruct_57 *param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x184c));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x94) = _PTR_LOOP_1050_5a68;
  *(undefined2 *)(iVar1 + 0x98) = 0x0;
  *(undefined2 *)(iVar1 + 0x9a) = 0x0;
  *(undefined2 *)(iVar1 + 0x9c) = 0x0;
  *(undefined2 *)(iVar1 + 0x9e) = 0x0;
  *(undefined2 *)param_1 = 0x8c2e;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_893a(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x8c2e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}



ushort __stdcall16far pass1_1038_8966(ulong param_1,ushort param_2,ushort param_3,int param_4,HWND16 param_5)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  undefined2 uVar4;
  
  bVar2 = false;
  iVar3 = (int)param_1;
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (param_4 == 0x0) {
    if (*(int *)(iVar3 + 0x98) < 0x1) goto LAB_1038_89af;
    piVar1 = (int *)(iVar3 + 0x9a);
    *piVar1 = *piVar1 + 0x1;
    piVar1 = (int *)(iVar3 + 0x98);
    *piVar1 = *piVar1 + -0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1038_89af;
    if (*(int *)(iVar3 + 0x9a) < 0x1) goto LAB_1038_89af;
    piVar1 = (int *)(iVar3 + 0x9a);
    *piVar1 = *piVar1 + -0x1;
    piVar1 = (int *)(iVar3 + 0x98);
    *piVar1 = *piVar1 + 0x1;
  }
  bVar2 = true;
LAB_1038_89af:
  if (bVar2) {
    SetDlgItemInt16(param_5,0x0,*(UINT16 *)(iVar3 + 0x9a),(int)s_dibtext_bmp_1050_1844 + 0x9);
    SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,*(UINT16 *)(iVar3 + 0x98),(int)s_dibtext_bmp_1050_1844 + 0xb);
  }
  return 0x0;
}



void __stdcall16far pass1_1038_89e8(ulong param_1,ushort param_2)

{
  send_dlg_item_msg_1038_8b58(param_1,param_2);
  return;
}



void __stdcall16far
pass1_1038_89f8(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,ushort param_6)

{
  if (param_4._2_2_ == 0xeb) {
    send_dlg_item_msg_1038_8b58(CONCAT22(param_2,param_1),param_6);
  }
  else {
    if (param_4._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,(ushort)&PTR_LOOP_1050_1040,param_6);
      return;
    }
    msg_box_ui_op_1038_8a3a(CONCAT22(param_2,param_1),0x0,param_5,param_6);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_ui_op_1038_8a3a(ulong param_1,char *param_2,uchar *param_3,UINT16 param_4)

{
  char local_20a [0x102];
  char *pcStack264;
  uchar *puStack262;
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  pcStack264 = param_2;
  puStack262 = param_3;
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(puStack262,pcStack264),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(puStack262,pcStack264),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x101,local_20a,param_4);
  MessageBox16(0x1010,(LPCSTR)0x0,local_20a,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puStack262,pcStack264),0x1000);
  return;
}



void __stdcall16far unk_win_ui_op_1038_8afe(astruct_50 *param_1,HWND16 param_2,BOOL16 param_3)

{
  undefined4 uVar1;
  UINT16 dlg_item;
  uint in_DX;
  astruct_50 *iVar4;
  astruct_50 *uVar4;
  BOOL16 local_4;
  
  uVar4 = (astruct_50 *)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_50 *)param_1;
  dlg_item = GetDlgItemInt16(param_2,0x0,&local_4,param_3);
  pass1_1030_6c1a(iVar4->field_0x94,dlg_item);
  uVar1 = iVar4->field_0x94;
  pass1_1038_387e(*(ulong *)((int)uVar1 + 0x2e),dlg_item,iVar4->field_0x9c,iVar4->field_0x94,in_DX);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far send_dlg_item_msg_1038_8b58(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  ulong uVar2;
  uchar *in_DX;
  ushort uVar3;
  undefined2 uVar4;
  int iVar5;
  int unaff_DI;
  undefined2 uVar6;
  uchar in_AF;
  LRESULT LVar7;
  undefined local_106 [0x100];
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  uVar3 = (ushort)((ulong)puStack6 >> 0x10);
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  pass1_1010_c3c2((ushort)puStack6,uVar3,CONCAT22(param_2,local_106),*(ulong *)(iVar5 + 0x94),(uchar *)uVar3,in_AF,
                  param_2);
  LVar7 = SendDlgItemMessage16(0x1010,(INT16)local_106,param_2,0x0,0x1846000c);
  uVar4 = (undefined2)((ulong)LVar7 >> 0x10);
  uVar1 = *(undefined4 *)(iVar5 + 0x94);
  *(undefined2 *)(iVar5 + 0x9c) = *(undefined2 *)((int)uVar1 + 0x32);
  *(undefined2 *)(iVar5 + 0x9a) = *(undefined2 *)(iVar5 + 0x9c);
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,*(UINT16 *)(iVar5 + 0x9c),(int)s_dibtext_bmp_1050_1844 + 0x9);
  uVar1 = *(undefined4 *)(iVar5 + 0x94);
  uVar2 = *(ulong *)((int)uVar1 + 0x2e);
  pass1_1038_3aa6(uVar2,(ushort)uVar2,uVar4);
  *(UINT16 *)(iVar5 + 0x98) = (UINT16)uVar2;
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,(UINT16)uVar2,(int)s_dibtext_bmp_1050_1844 + 0xb);
  return;
}



astruct_18 * __stdcall16far pass1_1038_8c08(astruct_18 *param_1,byte param_2)

{
  pass1_1038_893a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1038_8caa(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_704 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x185a));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_704 *)param_1;
  *(undefined4 *)&iVar1->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x90c8;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3f,param_5,param_3,param_4);
  iVar1->field_0x94 = (int)puVar2;
  iVar1->field_0x96 = (int)((ulong)puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_8cf6(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x90c8;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}



void __stdcall16far send_dlg_item_msg_1038_8d22(ulong param_1,HWND16 param_2,UINT16 param_3)

{
  undefined2 unaff_DI;
  undefined in_AF;
  LRESULT LVar1;
  undefined local_106 [0x100];
  WPARAM16 WStack6;
  int iStack4;
  
  LVar1 = SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x185b0409);
  WStack6 = (WPARAM16)LVar1;
  iStack4 = (int)WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_106,param_3,WStack6,0x185b040a);
    pass1_1008_c79a(*(ulong *)((int)param_1 + 0x94),CONCAT22(param_3,local_106),unaff_DI,param_3,in_AF);
  }
  return;
}



LRESULT __stdcall16far pass1_1038_8d7e(ulong param_1,ushort param_2)

{
  LRESULT LVar1;
  
  pass1_1040_78de(param_1);
  LVar1 = send_dlg_item_msg_1038_8f74(param_1,(int)&PTR_LOOP_1050_1040,param_2);
  return LVar1;
}



void __stdcall16far
pass1_1038_8d98(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,undefined2 param_6,ushort param_7
               )

{
  if (param_4._2_2_ == 0xeb) {
    send_dlg_item_msg_1038_8f74(CONCAT22(param_2,param_1),param_6,param_7);
  }
  else {
    if (param_4._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,(ushort)&PTR_LOOP_1050_1040,param_7);
      return;
    }
    msg_box_op_1038_8dda(CONCAT22(param_2,param_1),0x0,param_5,param_7);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_op_1038_8dda(ulong param_1,char *param_2,uchar *param_3,ushort param_4)

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
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
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



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT __stdcall16far send_dlg_item_msg_1038_8f74(ulong param_1,HWND16 param_2,WORD *param_3)

{
  int iVar1;
  undefined2 uVar2;
  long lVar3;
  LRESULT LVar4;
  BOOL16 enable;
  CHAR local_50c [0x100];
  undefined local_40c [0x8];
  undefined4 local_404;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x185b000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x185b0405);
  iVar1 = pass1_1008_c83a(*(ulong *)((int)param_1 + 0x94));
  if (iVar1 == 0x0) {
    local_404 = pass1_1008_c85e(*(ulong *)((int)param_1 + 0x94),param_3);
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_40c),local_404);
    while( true ) {
      lVar3 = pass1_1008_5b12(local_40c,param_3);
      if (lVar3 == 0x0) break;
      wsprintf16((LPSTR)0x1008,local_50c,param_3);
      SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_50c,(UINT16)param_3,0x0,0x185b0401);
    }
    GetDlgItem16(0x1008,0x1);
    enable = 0x1;
  }
  else {
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,(char *)&local_404,
               (short)param_3);
    SendDlgItemMessage16(0x1010,(INT16)&local_404,(UINT16)param_3,0x0,0x185b0401);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
    enable = 0x0;
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,enable);
  LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x185b000b);
  return LVar4;
}



astruct_18 * __stdcall16far pass1_1038_90a2(astruct_18 *param_1,byte param_2)

{
  pass1_1038_8cf6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_9144(ushort *param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  uint uVar2;
  uchar *in_DX;
  uchar *puVar3;
  uchar *puVar4;
  int iVar5;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  undefined2 uVar8;
  ushort *puVar9;
  int *piStack8;
  astruct_432 *iVar8;
  
  struct_1040_b082((astruct_57 *)param_1,CONCAT22(param_2,0xfaa));
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *(undefined2 *)(iVar5 + 0x94) = 0x0;
  *(undefined2 *)(iVar5 + 0x96) = 0x0;
  *(undefined4 *)(iVar5 + 0x98) = 0x0;
  *param_1 = 0x99a2;
  *(undefined2 *)(iVar5 + 0x2) = (int)&PTR_LOOP_1050_1038;
  *(undefined2 *)(iVar5 + 0x8a) = 0x27;
  puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,param_3,in_DX,unaff_DI);
  puVar3 = (uchar *)((ulong)puVar9 >> 0x10);
  uVar2 = (uint)puVar9;
  *(uint *)(iVar5 + 0x98) = uVar2;
  *(uchar **)(iVar5 + 0x9a) = puVar3;
  mem_op_1000_179c(0x18,puVar3,0x1000);
  puVar4 = (uchar *)((uint)puVar3 | uVar2);
  if (puVar4 == (uchar *)0x0) {
    *(undefined4 *)(iVar5 + 0x90) = 0x0;
  }
  else {
    struct_1040_a598((ushort *)CONCAT22(puVar3,uVar2));
    *(uint *)(iVar5 + 0x90) = uVar2;
    *(uchar **)(iVar5 + 0x92) = puVar4;
  }
  *(undefined2 *)*(undefined4 *)(iVar5 + 0x90) = 0x11;
  iVar6 = **(int **)(iVar5 + 0x90);
  uVar2 = iVar6 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,puVar4,0x1000);
  piStack8 = (int *)CONCAT22(puVar4,uVar2);
  if (((uint)puVar4 | uVar2) == 0x0) {
    uVar1 = *(undefined4 *)(iVar5 + 0x90);
    *(undefined4 *)((int)uVar1 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar6;
    pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar6,0xa,uVar2 + 0x2,(ushort)puVar4);
    uVar1 = *(undefined4 *)(iVar5 + 0x90);
    uVar8 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar6 = (int)uVar1;
    *(int *)(iVar6 + 0x2) = uVar2 + 0x2;
    *(uchar **)(iVar6 + 0x4) = puVar4;
  }
  uVar1 = *(undefined4 *)(iVar5 + 0x90);
  *(undefined2 *)((int)uVar1 + 0xa) = 0x18;
  uVar1 = *(undefined4 *)(iVar5 + 0x90);
  *(undefined2 *)((int)uVar1 + 0x12) = *(undefined2 *)(iVar5 + 0xa);
  return;
}



void __stdcall16far pass1_1038_927c(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x74);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_dlg_op_1038_9294(astruct_1 *param_1,ushort param_2)

{
  UINT16 UVar1;
  ushort uVar2;
  ushort in_DX;
  uint uVar3;
  WNDCLASS16 *unaff_SS;
  BOOL16 local_6;
  BOOL16 local_4;
  
  unk_win_ui_op_1040_b230(param_1,(ushort)&PTR_LOOP_1050_1040,(ushort)unaff_SS);
  uVar3 = (uint)((ulong)param_1 >> 0x10);
  UVar1 = GetDlgItemInt16((HWND16)&PTR_LOOP_1050_1040,0x1,&local_4,(BOOL16)unaff_SS);
  *(UINT16 *)((int)param_1 + 0x94) = UVar1;
  uVar2 = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,&local_6,(BOOL16)unaff_SS);
  *(UINT16 *)((int)param_1 + 0x96) = uVar2;
  win_ui_dlg_op_1038_98b4
            ((astruct_51 *)((ulong)param_1 & 0xffff | (ulong)uVar3 << 0x10),(int)s_tile2_bmp_1050_1538,unaff_SS);
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x950001,unaff_SS,uVar2,in_DX);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
draw_op_1038_92f6(ushort param_1,ushort param_2,ushort param_3,ulong param_4,HWND16 param_5,ushort param_6)

{
  undefined4 uVar1;
  code **ppcVar2;
  uint uVar3;
  int iVar4;
  astruct_18 *paVar5;
  uchar *in_DX;
  uchar *puVar6;
  uchar *puVar7;
  int unaff_DI;
  undefined2 uVar8;
  BOOL16 local_1a [0x2];
  UINT16 UStack22;
  astruct_18 *paStack20;
  astruct_18 *paStack16;
  int iStack12;
  astruct_18 *paStack10;
  astruct_20 *paStack6;
  
  if (param_4._2_2_ == 0xeb) {
    paStack6 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
    puVar6 = (uchar *)((ulong)paStack6 >> 0x10);
    paVar5 = *(astruct_18 **)(param_1 + 0x90);
    if (paVar5 != (astruct_18 *)0x0) {
      paStack10 = paVar5;
      mem_op_1000_179c(0x18,puVar6,0x1000);
      uVar3 = (uint)paVar5;
      paStack16 = (astruct_18 *)((ulong)paVar5 & 0xffff | ZEXT24(puVar6) << 0x10);
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
      *(undefined2 *)*(undefined4 *)(param_1 + 0x90) = 0x11;
      iStack12 = **(int **)(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,puVar7,0x1000);
      paStack16 = (astruct_18 *)CONCAT22(puVar7,uVar3);
      if (((uint)puVar7 | uVar3) == 0x0) {
        uVar1 = *(undefined4 *)(param_1 + 0x90);
        *(undefined4 *)((int)uVar1 + 0x2) = 0x0;
      }
      else {
        *(int *)paStack16 = iStack12;
        pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,(ushort)puVar7);
        uVar1 = *(undefined4 *)(param_1 + 0x90);
        uVar8 = (undefined2)((ulong)uVar1 >> 0x10);
        iVar4 = (int)uVar1;
        *(int *)(iVar4 + 0x2) = uVar3 + 0x2;
        *(uchar **)(iVar4 + 0x4) = puVar7;
      }
      uVar8 = (undefined2)((ulong)paStack10 >> 0x10);
      uVar1 = *(undefined4 *)(param_1 + 0x90);
      *(undefined4 *)((int)uVar1 + 0x6) = *(undefined4 *)((int)paStack10 + 0x6);
      uVar1 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar1 + 0xa) = *(undefined2 *)((int)paStack10 + 0xa);
      uVar1 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar1 + 0x12) = *(undefined2 *)(param_1 + 0xa);
      uVar8 = 0x1010;
      pass1_1010_a50c(paStack6,0x10505b42,*(ulong *)(param_1 + 0x90));
      paStack20 = paStack10;
      paStack16 = paStack10;
      if (paStack10 != (astruct_18 *)0x0) {
        pass1_1040_a5d0((ulong)paStack10);
        uVar8 = 0x1000;
        fn_ptr_1000_17ce(paStack10,0x1000);
      }
      ppcVar2 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar2)(uVar8,param_1,param_2);
    }
  }
  else {
    if (param_4._2_2_ != 0xf9) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,(ushort)&PTR_LOOP_1050_1040,param_6);
      return;
    }
    iVar4 = pass1_1038_993a(param_1,param_2,param_3);
    if (-0x1 < iVar4) {
      uVar8 = *(undefined2 *)(param_1 + 0x6);
      UStack22 = GetDlgItemInt16(param_5,0x1,local_1a,param_6);
      if (local_1a[0] != 0x0) {
        uVar1 = *(undefined4 *)(param_1 + 0x98);
        draw_fn_1010_2a32(0x94be,(uint16_t)s_tile2_bmp_1050_1538,(uint16 *)uVar1,(int)((ulong)uVar1 >> 0x10),UStack22,
                          CONCAT22(uVar8,*(undefined2 *)(iVar4 * 0xe + 0x5a72)),(uint16_t)in_DX,param_1,
                          (uint16_t)&stack0xfffe,param_2);
      }
    }
  }
  return;
}



BOOL16 __stdcall16far
send_dlg_item_int_1038_94da
          (int param_1,ushort param_2,ushort param_3,ushort param_4,int param_5,HWND16 param_6,BOOL16 param_7)

{
  UINT16 *pUVar1;
  int iVar2;
  long lVar3;
  BOOL16 local_c;
  uint uStack10;
  int iStack8;
  UINT16 UStack6;
  int iStack4;
  
  iStack4 = 0x1;
  iStack8 = pass1_1038_993a(param_1,param_2,param_3);
  if ((-0x1 < iStack8) && (UStack6 = GetDlgItemInt16(param_6,0x1,&local_c,param_7), local_c != 0x0)) {
    if (param_5 == 0x0) {
      UStack6 = UStack6 + 0x1;
    }
    else {
      iStack4 = -0x1;
      UStack6 = UStack6 - 0x1;
    }
    uStack10 = (uint)((int)UStack6 <= *(int *)(iStack8 * 0xe + 0x5a7a));
    pUVar1 = (UINT16 *)(iStack8 * 0xe + 0x5a78);
    if (*pUVar1 != UStack6 && (int)UStack6 <= (int)*pUVar1) {
      uStack10 = 0x0;
    }
    iVar2 = iStack8 * 0xe;
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,*(INT16 *)(iVar2 + 0x5a72));
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    if ((uStack10 != 0x0) &&
       (lVar3 = unk_win_ui_op_1038_9820
                          ((astruct_51 *)CONCAT22(param_2,param_1),0x1,iStack4,(int)s_tile2_bmp_1050_1538,param_7),
       (int)lVar3 != 0x0)) {
      SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,UStack6,*(BOOL16 *)(iVar2 + 0x5a72));
      SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,*(UINT16 *)(param_1 + 0x94),0xfa9);
      SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,*(UINT16 *)(param_1 + 0x96),0xfa8);
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_msg_op_1038_95fc(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  uint uVar2;
  UINT16 UVar3;
  UINT16 UVar4;
  uchar *in_DX;
  uchar *extraout_DX;
  uchar *puVar5;
  uchar *extraout_DX_00;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  HWND16 hwnd;
  HWND16 HVar8;
  ushort uVar9;
  ushort uVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  undefined2 *puStack30;
  undefined2 *puStack24;
  int iStack20;
  BOOL16 local_10;
  undefined4 *puStack14;
  ushort *puStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_2,in_DX,unaff_DI);
  puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_2,(uchar *)((ulong)puStack6 >> 0x10),unaff_DI);
  puVar5 = (uchar *)((ulong)puStack10 >> 0x10);
  uVar2 = (uint)puStack10;
  hwnd = 0x1000;
  mem_op_1000_179c(0xc,puVar5,0x1000);
  if (((uint)puVar5 | uVar2) == 0x0) {
    uVar2 = 0x0;
    puVar5 = (uchar *)0x0;
  }
  else {
    hwnd = 0x1008;
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar5,uVar2));
    puVar5 = extraout_DX;
  }
  puStack14 = (undefined4 *)CONCAT22(puVar5,uVar2);
  for (iStack20 = 0x0; iStack20 < 0xf; iStack20 = iStack20 + 0x1) {
    uVar12 = *(undefined2 *)((int)param_1 + 0x6);
    HVar8 = (HWND16)s_tile2_bmp_1050_1538;
    UVar3 = GetDlgItemInt16(hwnd,0x1,&local_10,param_2);
    if (UVar3 != 0x0) {
      if (*(int *)(iStack20 * 0xe + 0x5a7c) < 0x83) {
        uVar11 = 0x8;
        HVar8 = 0x1000;
        UVar4 = UVar3;
        mem_op_1000_179c(0x8,puVar5,0x1000);
        puStack24 = (undefined2 *)CONCAT22(puVar5,UVar4);
        if (((uint)puVar5 | UVar4) == 0x0) {
          puStack30 = (undefined2 *)0x0;
        }
        else {
          *puStack24 = 0x389a;
          *(undefined2 *)(UVar4 + 0x2) = 0x1008;
          *puStack24 = 0xa1c4;
          *(undefined2 *)(UVar4 + 0x2) = 0x1010;
          puStack30 = puStack24;
        }
        uVar7 = (undefined2)((ulong)puStack30 >> 0x10);
        iVar6 = (int)puStack30;
        *(UINT16 *)(iVar6 + 0x6) = UVar3;
        *(undefined2 *)(iVar6 + 0x4) = *(undefined2 *)(iStack20 * 0xe + 0x5a7c);
        ppcVar1 = (code **)((int)*puStack14 + 0x4);
        (**ppcVar1)(0x1000,(int)puStack14,(int)((ulong)puStack14 >> 0x10),iVar6,uVar7,uVar11,uVar12);
        puVar5 = extraout_DX_00;
      }
      else {
        if (*(int *)(iStack20 * 0xe + 0x5a7c) == 0x89) {
          uVar10 = *(ushort *)(iStack20 * 0xe + 0x5a7c);
          uVar9 = UVar3;
        }
        else {
          uVar10 = *(ushort *)(iStack20 * 0xe + 0x5a7c);
          uVar9 = 0x0;
        }
        HVar8 = 0x1010;
        pass1_1010_6566((ulong)puStack10,uVar9,UVar3,uVar10,param_2);
      }
    }
    hwnd = HVar8;
  }
  *(undefined4 *)((int)puStack6 + 0xa) = puStack14;
  PostMessage16(hwnd,0x0,0x0,0x11100ed);
  return;
}



void __stdcall16far
win_ui_op_1038_977a(int param_1,ushort param_2,int param_3,uchar *param_4,HWND16 param_5,ushort param_6)

{
  code **ppcVar1;
  uint uVar2;
  int iVar3;
  uchar *puVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  undefined local_10 [0x4];
  undefined4 *puStack12;
  int iStack8;
  UINT16 UStack6;
  BOOL16 local_4;
  
  iStack8 = 0x0;
  uVar6 = *(undefined2 *)(param_1 + 0x6);
  uVar2 = GetDlgItemInt16(param_5,0x1,&local_4,param_6);
  UStack6 = uVar2;
  if (uVar2 != 0x0) {
    uVar5 = 0xb4;
    mem_op_1000_179c(0xb4,param_4,0x1000);
    puVar4 = (uchar *)((uint)param_4 | uVar2);
    if (puVar4 == (uchar *)0x0) {
      iVar3 = 0x0;
      puVar4 = (uchar *)0x0;
    }
    else {
      iVar3 = string_1040_8520((astruct_57 *)CONCAT22(param_4,uVar2),*(ushort *)(param_1 + 0x6),0x41,0x2,0x5db,0x5da,
                               puVar4,param_6);
    }
    puStack12 = (undefined4 *)CONCAT22(puVar4,iVar3);
    pass1_1008_941a((ushort *)CONCAT22(param_6,local_10),0x1,0xc3);
    ppcVar1 = (code **)((int)*puStack12 + 0x6c);
    iStack8 = (**ppcVar1)(0x1008,(int)puStack12,(int)((ulong)puStack12 >> 0x10),local_10,param_6,uVar5,uVar6,uVar2);
  }
  if ((iStack8 == 0x1) || (UStack6 == 0x0)) {
    destroy_window_1040_b726((ULONG *)CONCAT22(param_2,param_1),param_3,(int)&PTR_LOOP_1050_1040);
  }
  return;
}



long __stdcall16far unk_win_ui_op_1038_9820(astruct_51 *param_1,int param_2,int param_3,HWND16 param_4,BOOL16 param_5)

{
  int *piVar1;
  long lVar2;
  UINT16 UVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  astruct_51 *iVar7;
  astruct_51 *uVar7;
  BOOL16 local_6;
  BOOL16 local_4;
  
  uVar7 = (astruct_51 *)((ulong)param_1 >> 0x10);
  iVar7 = (astruct_51 *)param_1;
  UVar3 = GetDlgItemInt16(param_4,0x1,&local_4,param_5);
  iVar4 = UVar3 * param_2 * param_3;
  UVar3 = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,&local_6,param_5);
  lVar2 = (long)(int)(UVar3 * param_2) * (long)param_3;
  uVar6 = (uint)((ulong)lVar2 >> 0x10);
  iVar5 = (int)lVar2;
  if ((iVar4 - iVar7->field_0x94 < 0x1) && (-0x1 < iVar7->field_0x96 - iVar5)) {
    piVar1 = &iVar7->field_0x94;
    *piVar1 = *piVar1 - iVar4;
    piVar1 = &iVar7->field_0x96;
    *piVar1 = *piVar1 - iVar5;
    return CONCAT22(uVar6,0x1);
  }
  return (long)((ulong)uVar6 << 0x10);
}



void __stdcall16far win_ui_dlg_op_1038_98b4(astruct_51 *param_1,HWND16 param_2,BOOL16 param_3)

{
  UINT16 UVar1;
  undefined2 uVar2;
  int iVar3;
  int iStack8;
  BOOL16 local_4;
  
  local_4 = 0x0;
  for (iStack8 = 0x0; iVar3 = (int)param_1, uVar2 = (undefined2)((ulong)param_1 >> 0x10), iStack8 < 0xf;
      iStack8 = iStack8 + 0x1) {
    iVar3 = *(int *)(iVar3 + 0x6);
    UVar1 = GetDlgItemInt16(param_2,0x1,&local_4,param_3);
    unk_win_ui_op_1038_9820(param_1,UVar1,iVar3,(int)s_tile2_bmp_1050_1538,param_3);
    param_2 = (HWND16)s_tile2_bmp_1050_1538;
  }
  SetDlgItemInt16(param_2,0x1,*(UINT16 *)(iVar3 + 0x94),0xfa9);
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,*(UINT16 *)(iVar3 + 0x96),0xfa8);
  return;
}



int __stdcall16far pass1_1038_993a(ushort param_1,ushort param_2,int param_3)

{
  int iStack6;
  
  iStack6 = 0x0;
  while( true ) {
    if (0xe < iStack6) {
      return -0x1;
    }
    if (*(int *)(iStack6 * 0xe + 0x5a70) == param_3) break;
    iStack6 = iStack6 + 0x1;
  }
  return iStack6;
}



astruct_18 * __stdcall16far pass1_1038_997c(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1038_9a1e(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(ushort)(param_4 >> 0x10));
  *(ushort *)CONCAT22(param_2,param_1) = 0x9af6;
  *(undefined2 *)(param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1038_9a48(astruct_18 *param_1)

{
  param_1->field_0x0 = 0x9af6;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



void __stdcall16far
enable_win_1038_9a66(ushort param_1,ushort param_2,ushort in_b_enable_3,ulong param_4,HWND16 in_hwnd_5)

{
  uchar *in_DX;
  ushort unaff_SS;
  
  if (param_4._2_2_ == 0xf8) {
    GetDlgItem16(in_hwnd_5,0x17d9);
    in_b_enable_3 = 0x1;
  }
  else {
    if (param_4._2_2_ != 0x17d9) {
      pass1_1040_b54a(param_1,param_2,in_b_enable_3,param_4,in_DX,(ushort)&PTR_LOOP_1050_1040,unaff_SS);
      return;
    }
    SetWindowPos16(in_hwnd_5,0x6,0x1a0,0x12c,0x0,0x0,0x0);
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,in_b_enable_3);
  return;
}



ULONG __stdcall16far pass1_1038_9ad0(ULONG param_1,byte param_2)

{
  pass1_1038_9a48((astruct_18 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1038_9b72(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  int iStack4;
  
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(ushort)(param_4 >> 0x10));
  *(undefined2 *)(param_1 + 0x128) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x9efa;
  *(undefined2 *)(param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  iStack4 = 0x0;
  do {
    *(undefined2 *)(param_1 + iStack4 * 0x2 + 0x94) = 0x0;
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x4a);
  return CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1038_9bc8(astruct_1 *param_1)

{
  int *piVar1;
  code **ppcVar2;
  int iVar3;
  INT16 IVar4;
  HDC16 hdc;
  int iVar5;
  HWND16 HVar6;
  uchar *in_DX;
  uchar *puVar7;
  int iVar8;
  int unaff_DI;
  undefined2 uVar9;
  ushort unaff_SS;
  ushort *puVar10;
  undefined2 uVar12;
  undefined4 uVar11;
  ushort *puVar13;
  int iStack36;
  RECT16 local_16;
  int iStack16;
  int iStack14;
  int iStack12;
  undefined4 uStack10;
  int local_6;
  int local_4;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  if (PTR_LOOP_1050_5ef8 == (undefined *)((int)&DAT_1050_0004 + 0x1)) {
    PTR_LOOP_1050_5ef8 = (undefined *)0x0;
  }
  puVar13 = (ushort *)CONCAT22(unaff_SS,&local_4);
  puVar10 = (ushort *)CONCAT22(unaff_SS,&local_6);
  uStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)uStack10 & 0xffff0000 | (ulong)((int)uStack10 + 0xe)),puVar10,puVar13);
  IVar4 = GetSystemMetrics16(0x1008);
  puVar7 = (uchar *)((ulong)((long)IVar4 * (long)(int)PTR_LOOP_1050_5ef8) >> 0x10);
  iStack12 = (int)((long)IVar4 * (long)(int)PTR_LOOP_1050_5ef8) + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 0x1;
  iStack14 = iStack12 + local_6;
  iStack12 = iStack12 + local_4;
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  iVar8 = (int)param_1;
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_16);
  uVar12 = 0x0;
  hdc = GetDC16((HWND16)s_tile2_bmp_1050_1538);
  IVar4 = GetDeviceCaps16((HDC16)s_tile2_bmp_1050_1538,0xa);
  ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,hdc);
  if (IVar4 < iStack16) {
    iStack14 = (local_16.y - (iStack16 - IVar4)) + 0x1;
  }
  uVar11 = CONCAT22(uVar12,*(undefined2 *)(iVar8 + 0x6));
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,0x0,iStack14,iStack12,0x0);
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,puVar7,unaff_DI);
  iVar5 = (int)puVar10 + 0xa4;
  uVar12 = (undefined2)((ulong)puVar10 >> 0x10);
  iStack36 = 0x0;
  HVar6 = 0x1010;
  while (iVar3 = iStack36 * 0x2, *(int *)(iVar3 + iVar5) != 0x0) {
    HVar6 = GetDlgItem16(HVar6,*(INT16 *)(iVar3 + iVar5));
    *(HWND16 *)(iVar8 + iVar3 + 0x94) = HVar6;
    iStack36 = iStack36 + 0x1;
    piVar1 = (int *)(iVar8 + 0x128);
    *piVar1 = *piVar1 + 0x1;
    HVar6 = (HWND16)s_tile2_bmp_1050_1538;
  }
  ppcVar2 = (code **)((int)param_1->field_0x0 + 0x6c);
  (**ppcVar2)(HVar6,iVar8,uVar9,uVar11);
  return;
}
