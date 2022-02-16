
long __stdcall16far pass1_1008_57f0(ulong param_1,int param_2,ushort param_3)

{
  bool bVar1;
  long lVar2;
  int iStack12;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),param_1);
  iStack12 = 0x0;
  do {
    lVar2 = pass1_1008_5b12(local_a,param_3);
    if (lVar2 == 0x0) {
      return 0x0;
    }
    bVar1 = iStack12 != param_2;
    iStack12 = iStack12 + 0x1;
  } while (bVar1);
  return lVar2;
}



void __stdcall16far pass1_1008_5830(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  
  while( true ) {
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if (*(long *)(iVar6 + 0x4) == 0x0) break;
    if (*(int *)(iVar6 + 0xa) != 0x0) {
      uVar4 = *(undefined4 *)(iVar6 + 0x4);
      uVar9 = (undefined2)((ulong)uVar4 >> 0x10);
      iVar7 = (int)uVar4;
      puVar1 = (undefined4 *)*(uint *)(iVar7 + 0x8);
      uVar2 = *(uint *)(iVar7 + 0xa);
      if ((uVar2 | (uint)puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
      }
    }
    puVar5 = (undefined4 *)*(long *)(iVar6 + 0x4);
    *(undefined4 *)(iVar6 + 0x4) = *(undefined4 *)((int)puVar5 + 0x4);
    if (puVar5 != (undefined4 *)0x0) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
    }
  }
  *(undefined2 *)(iVar6 + 0x8) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_58a6(ulong param_1,ulong param_2)

{
  int *piVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  astruct_99 *paStack6;
  
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_029c);
  uVar3 = (uint)((ulong)paStack6 >> 0x10);
  uVar2 = (uint)paStack6;
  if ((uVar3 | uVar2) == 0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    *(undefined2 *)(uVar2 + 0x2) = 0x1008;
    *(undefined4 *)(uVar2 + 0x4) = 0x0;
    *(undefined4 *)(uVar2 + 0x8) = 0x0;
    paStack6->field_0x0 = 0x5bc0;
    *(undefined2 *)(uVar2 + 0x2) = 0x1008;
  }
  if (paStack6 == (astruct_99 *)0x0) {
    return;
  }
  uVar5 = (undefined2)((ulong)paStack6 >> 0x10);
  *(ulong *)((int)paStack6 + 0x8) = param_2;
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  *(undefined4 *)((int)paStack6 + 0x4) = *(undefined4 *)(iVar4 + 0x4);
  *(astruct_99 **)(iVar4 + 0x4) = paStack6;
  piVar1 = (int *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_593c(ulong *param_1,ulong param_2)

{
  int *piVar1;
  code **ppcVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  astruct_99 *paStack6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(int *)(iVar5 + 0x8) == 0x0) {
    ppcVar2 = (code **)((int)*param_1 + 0x4);
    (**ppcVar2)();
    return;
  }
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_029c);
  uVar4 = (uint)((ulong)paStack6 >> 0x10);
  uVar3 = (uint)paStack6;
  if ((uVar4 | uVar3) == 0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    *(undefined2 *)(uVar3 + 0x2) = 0x1008;
    *(undefined4 *)(uVar3 + 0x4) = 0x0;
    *(undefined4 *)(uVar3 + 0x8) = 0x0;
    paStack6->field_0x0 = 0x5bc0;
    *(undefined2 *)(uVar3 + 0x2) = 0x1008;
  }
  if (paStack6 == (astruct_99 *)0x0) {
    return;
  }
  *(ulong *)((int)paStack6 + 0x8) = param_2;
  do {
    param_1 = *(ulong **)((int)param_1 + 0x4);
    uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  } while (*(long *)((int)param_1 + 0x4) != 0x0);
  *(astruct_99 **)((int)param_1 + 0x4) = paStack6;
  piVar1 = (int *)(iVar5 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



void __stdcall16far pass1_1008_59f4(ulong param_1,long param_2)

{
  int *piVar1;
  undefined4 *puVar2;
  uint uVar3;
  undefined4 *puVar4;
  code **ppcVar5;
  undefined4 *puVar6;
  undefined2 uVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  uint uStack10;
  undefined4 *puStack6;
  
  puStack6 = (undefined4 *)0x0;
  uVar9 = (undefined2)(param_1 >> 0x10);
  puVar6 = puStack6;
  puVar4 = (undefined4 *)param_1;
  do {
    puStack6 = puVar6;
    uVar10 = (undefined2)((ulong)puVar4 >> 0x10);
    iVar8 = (int)puVar4;
    puVar4 = (undefined4 *)*(long *)(iVar8 + 0x4);
    uStack10 = (uint)puVar4;
    uVar11 = (undefined2)((ulong)puVar4 >> 0x10);
    if ((*(uint *)(iVar8 + 0x6) | uStack10) == 0x0) break;
    puVar6 = puVar4;
  } while (*(long *)(uStack10 + 0x8) != param_2);
  if (puVar4 != (undefined4 *)0x0) {
    if (puStack6 == (undefined4 *)0x0) {
      uVar10 = *(undefined2 *)(uStack10 + 0x4);
      uVar7 = *(undefined2 *)(uStack10 + 0x6);
      puStack6 = (undefined4 *)param_1;
    }
    else {
      uVar10 = *(undefined2 *)(uStack10 + 0x4);
      uVar7 = *(undefined2 *)(uStack10 + 0x6);
    }
    uVar12 = (undefined2)((ulong)puStack6 >> 0x10);
    *(undefined2 *)((int)puStack6 + 0x4) = uVar10;
    *(undefined2 *)((int)puStack6 + 0x6) = uVar7;
    if (*(int *)((int)param_1 + 0xa) != 0x0) {
      puVar2 = (undefined4 *)*(uint *)(uStack10 + 0x8);
      uVar3 = *(uint *)(uStack10 + 0xa);
      if ((uVar3 | (uint)puVar2) != 0x0) {
        ppcVar5 = (code **)*puVar2;
        (**ppcVar5)();
      }
    }
    if (puVar4 != (undefined4 *)0x0) {
      ppcVar5 = (code **)*puVar4;
      (**ppcVar5)();
    }
    piVar1 = (int *)((int)param_1 + 0x8);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  return;
}



void __stdcall16far pass1_1008_5ab8(ulong param_1)

{
  int *piVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined2 uVar5;
  uint uVar6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x4) == 0x0) {
    return;
  }
  puVar3 = (undefined4 *)*(undefined4 *)(iVar4 + 0x4);
  uVar6 = (uint)((ulong)puVar3 >> 0x10);
  *(undefined4 *)(iVar4 + 0x4) = *(undefined4 *)((uint)puVar3 + 0x4);
  if ((uVar6 | (uint)puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
  }
  piVar1 = (int *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + -0x1;
  return;
}



void __stdcall16far pass1_1008_5b12(long *param_1)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  
  if ((*param_1 != 0x0) && (*(int *)((int)*param_1 + 0x8) != 0x0)) {
    uVar4 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    if (*(long *)(iVar2 + 0x4) == 0x0) {
      uVar5 = (undefined2)((ulong)*param_1 >> 0x10);
      iVar3 = (int)*param_1;
    }
    else {
      uVar1 = *(undefined4 *)(iVar2 + 0x4);
      uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
      iVar3 = (int)uVar1;
    }
    *(undefined4 *)(iVar2 + 0x4) = *(undefined4 *)(iVar3 + 0x4);
    if (*(long *)(iVar2 + 0x4) != 0x0) {
      return;
    }
  }
  return;
}



ushort * __stdcall16far pass1_1008_5b6e(ushort *param_1,byte param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x389a;
  ((int *)param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((int *)param_1,uVar1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_5b9a(ushort *param_1,byte param_2)

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_5bdc(astruct_79 *param_1,int param_2,ushort param_3)

{
  astruct_651 *puVar1;
  undefined2 uVar1;
  astruct_79 *paVar2;
  ushort *puVar3;
  
  paVar2 = struct_op_1010_1d48(param_1,0x44);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  puVar1 = (astruct_651 *)param_1;
  puVar1->field_0xa = 0x0;
  *(undefined4 *)&puVar1->field_0xc = 0x0;
  puVar1->field_0x10 = 0x0;
  puVar1->field_0x12 = 0x0;
  param_1->field_0x0 = 0x5fc8;
  puVar1->field_0x2 = 0x1008;
  _PTR_LOOP_1050_02a0 = param_1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,(uchar *)((ulong)paVar2 >> 0x10),param_2);
  puVar1->field_0xc = (int)puVar3;
  puVar1->field_0xe = (int)((ulong)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_5c34(ushort *param_1)

{
  ushort unaff_SS;
  
  *param_1 = 0x5fc8;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  _PTR_LOOP_1050_02a0 = 0x0;
  pass1_1010_1d80(param_1,unaff_SS);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_1008_5c5c(WNDCLASS16 *param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  pass1_1010_84f8(_PTR_LOOP_1050_14cc,param_5,(ushort)param_1);
  win_ui_op_1008_5cfe(param_4,CONCAT22(param_3,param_2),param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_1008_5c7c(ulong param_1,ulong param_2,WNDCLASS16 *param_3,ushort param_4,ushort param_5)

{
  pass1_1010_85be(_PTR_LOOP_1050_14cc,(int)param_2,(int)(param_2 >> 0x10),(ushort)param_3);
  win_ui_op_1008_5cfe(param_1,CONCAT22(param_5,param_4),param_3);
  return;
}



void __stdcall16far win_1008_5c9e(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,WNDCLASS16 *param_5)

{
  win_1008_5c7c(param_1,*param_2,param_5,param_3,param_4);
  return;
}



void __stdcall16far mci_send_command_1008_5cb6(ulong param_1,int param_2,ushort param_3)

{
  int iVar1;
  uint uVar2;
  ushort unaff_SS;
  
  mciSendCommand16(param_3,0x0,0x0,0x8040000);
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((*(int *)(iVar1 + 0xa) == 0x0) || (*(int *)(iVar1 + 0xa) != param_2)) {
    *(undefined2 *)(iVar1 + 0x12) = 0x0;
    iVar1 = 0x11;
  }
  else {
    *(undefined2 *)(iVar1 + 0x10) = 0x0;
    iVar1 = 0x10;
  }
  pass1_1010_1f62(unaff_SS,param_1 & 0xffff | (ulong)uVar2 << 0x10,iVar1);
  return;
}



void __stdcall16far win_ui_op_1008_5cfe(ulong param_1,char *param_2,WNDCLASS16 *in_wnd_class)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  DWORD DVar5;
  HWND16 message_1;
  undefined2 uStack298;
  HWND16 window_handle_1;
  undefined local_11e [0x100];
  char *string_1;
  int iStack26;
  int iStack24;
  undefined local_16 [0x4];
  int offset_val;
  char *pcStack14;
  char *pcStack10;
  
  pass1_1000_4906((astruct_20 *)CONCAT22(in_wnd_class,local_16),(WNDCLASS16 *)0x0,0x14);
  pcStack10 = param_2;
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar3 + 0xc);
  iStack24 = *(int *)((int)uVar1 + 0x72);
  iStack26 = 0x1;
  string_1 = s_waveaudio_1050_02a4;
  str_1000_4d58(param_2,(char *)0x0,0x0,0x0,(WNDCLASS16 *)CONCAT22(in_wnd_class,local_11e));
  iVar2 = pass1_1000_475e(CONCAT22(in_wnd_class,local_11e),0x105002ae);
  if (iVar2 == 0x0) {
    uVar1 = *(undefined4 *)(iVar3 + 0xc);
    iStack24 = *(int *)((int)uVar1 + 0x74);
    string_1 = s_sequencer_1050_02b3;
    iStack26 = 0x0;
  }
  if (iStack24 != 0x0) {
    if ((iStack26 != 0x0) && (*(int *)(iVar3 + 0x10) != 0x0)) {
      return;
    }
    if ((iStack26 == 0x0) && (*(int *)(iVar3 + 0x12) != 0x0)) {
      return;
    }
    pcStack14 = string_1;
    DVar5 = mciSendCommand16(0x1000,(UINT16)local_16,CONCAT13(0x22,ZEXT23(in_wnd_class)),0x8030000);
    if (((uint)(DVar5 >> 0x10) | (uint)DVar5) == 0x0) {
      if (iStack26 == 0x0) {
        *(undefined2 *)(iVar3 + 0x12) = 0x1;
      }
      else {
        *(int *)(iVar3 + 0xa) = offset_val;
        *(undefined2 *)(iVar3 + 0x10) = 0x1;
      }
      window_handle_1 = create_window_1008_5e7e((UINT16)s_tile2_bmp_1050_1538,in_wnd_class);
      if (window_handle_1 == 0x0) {
        mci_send_command_1008_5cb6(param_1,offset_val,(ushort)s_tile2_bmp_1050_1538);
        return;
      }
      pass1_1000_4906((astruct_20 *)CONCAT22(in_wnd_class,&message_1),(WNDCLASS16 *)0x0,0xc);
      message_1 = window_handle_1;
      uStack298 = 0x0;
      mciSendCommand16(0x1000,(UINT16)&message_1,(ulong)CONCAT12(0x1,in_wnd_class),0x8060000);
      SetWindowWord16((HWND16)s_tile2_bmp_1050_1538,offset_val,0x0);
      return;
    }
  }
  if (iStack26 == 0x0) {
    iVar2 = 0x11;
  }
  else {
    iVar2 = 0x10;
  }
  pass1_1010_1f62((ushort)in_wnd_class,param_1,iVar2);
  return;
}



HWND16 __stdcall16far create_window_1008_5e7e(UINT16 in_stock_obj_id,WNDCLASS16 *in_wnd_class)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  BOOL16 BVar3;
  ATOM AVar4;
  HWND16 window_handle_1;
  int iVar5;
  LPCSTR string_1;
  undefined4 *puVar6;
  undefined2 name;
  undefined2 uStack42;
  undefined2 uStack40;
  undefined2 uStack38;
  undefined2 uStack36;
  undefined *puStack34;
  undefined2 uStack32;
  undefined2 uStack30;
  HGDIOBJ16 HStack28;
  undefined4 uStack26;
  undefined4 *puStack22;
  undefined4 local_12 [0x4];
  
  puVar6 = local_12;
  string_1 = (LPCSTR)s_MciSoundWindow_1050_02bd;
  for (iVar5 = 0x3; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = (undefined4 *)string_1;
    string_1 = (LPCSTR)((int)string_1 + 0x4);
    *puVar2 = *puVar1;
  }
  *(undefined2 *)puVar6 = *(undefined2 *)string_1;
  *(undefined *)(undefined2 *)((int)puVar6 + 0x2) = *(undefined *)(undefined2 *)((int)string_1 + 0x2);
  name = 0x2000;
  uStack42 = SUB42(&DAT_1050_5f44,0x0);
  uStack40 = 0x1008;
  uStack36 = 0x2;
  puStack34 = PTR_LOOP_1050_038c;
  uStack32 = 0x0;
  uStack30 = 0x0;
  uStack38 = 0x0;
  HStack28 = GetStockObject16(in_stock_obj_id);
  uStack26 = 0x0;
  puStack22 = local_12;
  BVar3 = GetClassInfo16((HINSTANCE16)s_tile2_bmp_1050_1538,(SEGPTR)&name,in_wnd_class);
  if (BVar3 == 0x0) {
    AVar4 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar4 == 0x0) {
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      return 0x0;
    }
  }
  window_handle_1 =
       CreateWindow16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x0,
                      (INT16)PTR_LOOP_1050_0396,0x1,0x1,0x8000,0x8000,0x0,(LPVOID)0xcf);
  return window_handle_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT __stdcall16far make_def_win_proc_1008_5f44(UINT16 param_1,WPARAM16 in_wparam_2,LPARAM param_3,HWND16 in_hwnd_4)

{
  WORD WVar1;
  uchar *in_DX;
  int unaff_DI;
  WNDCLASS16 *unaff_SS;
  LRESULT LVar2;
  ushort *puVar3;
  
  if (param_3._2_2_ == 0x2) {
    WVar1 = GetWindowWord16(in_hwnd_4,0x0);
    mci_send_command_1008_5cb6(_PTR_LOOP_1050_02a0,WVar1,(ushort)s_tile2_bmp_1050_1538);
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,(ushort)unaff_SS,in_DX,unaff_DI);
    pass1_1008_aa28((ulong)puVar3,(uint)puVar3,unaff_SS);
  }
  else {
    if (param_3._2_2_ != 0x3b9) {
      LVar2 = DefWindowProc16(in_hwnd_4,param_1,in_wparam_2,param_3);
      return LVar2;
    }
    DestroyWindow16(in_hwnd_4);
  }
  return 0x0;
}



ulong __stdcall16far pass1_1008_5fa2(ulong param_1,byte param_2)

{
  pass1_1008_5c34((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar * __cdecl16far pass1_1008_5fd8(ushort param_1,uchar *param_2)

{
  int *piVar1;
  uchar *puVar2;
  char *pcVar3;
  int iStack6;
  
  puVar2 = &stack0x0006;
  _iStack6 = (int *)CONCAT22(param_1,puVar2);
  mem_op_1000_179c(0x1000,param_2,0x1000);
  pcVar3 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_2,puVar2),pcVar3);
  while( true ) {
    piVar1 = _iStack6;
    _iStack6 = (int *)((ulong)_iStack6 & 0xffff0000 | (ulong)(iStack6 + 0x2));
    if (*piVar1 == 0x0) break;
    pcVar3 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    pass1_1000_3cea(CONCAT22(param_2,puVar2),(ULONG)pcVar3);
  }
  return puVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far debug_print_1008_6048(ulong param_1,LPSTR param_2,WORD *param_3)

{
  uint uVar1;
  uint in_DX;
  ushort unaff_ES;
  uchar in_AF;
  WORD *args;
  
  if (PTR_LOOP_1050_02ec != (undefined *)0x0) {
    args = param_3;
    if (DAT_1050_02ee == 0xffff) {
      param_2 = (LPSTR)&PTR_LOOP_1050_1000;
      uVar1 = pass1_1000_3ec0(0x2f4,(int)&USHORT_1050_1050);
      DAT_1050_02ee = (uint)((in_DX | uVar1) != 0x0);
    }
    if (DAT_1050_02ee != 0x0) {
      wvsprintf16(param_2,&stack0x0008,args);
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      if (_PTR_LOOP_1050_02f0 != 0x0) {
        pass1_1000_2b5c((ushort)_PTR_LOOP_1050_02f0,(ushort)((ulong)_PTR_LOOP_1050_02f0 >> 0x10),0x2fd,
                        (ushort)&USHORT_1050_1050,unaff_ES,(int)&stack0xfffe,0x1000,(ushort)param_3);
        pass1_1000_2f48(_PTR_LOOP_1050_02f0,(int)&stack0xfffe,unaff_ES,0x1000,(ushort)param_3,in_AF);
      }
    }
  }
  return;
}



ushort __cdecl16far str_op_1008_60e8(char *param_1,ushort param_2)

{
  uint uVar1;
  
  if (param_1 != (char *)0x0) {
    uVar1 = str_op_1000_3da4(param_1);
    uVar1 = uVar1 + 0x1;
    mem_op_1000_179c(uVar1,(uchar *)param_2,0x1000);
    if ((param_2 | uVar1) != 0x0) {
      unk_str_op_1000_3d3e((char *)CONCAT22(param_2,uVar1),param_1);
      return uVar1;
    }
  }
  return 0x0;
}



void __cdecl16far pass1_1008_612e(int param_1,int param_2,uint param_3)

{
  uint uVar1;
  uint uVar2;
  long lVar3;
  int iVar4;
  int iStack18;
  int iStack16;
  
  uVar1 = pass1_1000_4d24();
  uVar2 = (param_2 - param_1) + 0x1;
  if (((int)uVar2 >> 0xf | uVar2) == 0x0) {
    return;
  }
  iStack16 = 0x1;
  iStack18 = param_1;
  do {
    if (param_2 < iStack18) {
      return;
    }
    lVar3 = (long)iStack16 * (long)(0x7fff / (sqword)(long)(int)uVar2);
    iVar4 = (int)((ulong)lVar3 >> 0x10);
    if ((int)uVar1 >> 0xf <= iVar4) {
      if ((int)uVar1 >> 0xf < iVar4) {
        return;
      }
      if (uVar1 <= (uint)lVar3) {
        return;
      }
    }
    iStack18 = iStack18 + 0x1;
    iStack16 = iStack16 + 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_20 * __stdcall16far
unk_draw_op_1008_61b2(astruct_20 *param_1,UINT16 param_2,UINT16 param_3,ULONG param_4,UINT16 param_5)

{
  HGDIOBJ16 l_hgdiobj_1;
  HCURSOR16 l_hcursor_1;
  uchar *extraout_DX;
  uchar *puVar1;
  int unaff_DI;
  UINT16 *l_struct_2;
  astruct_20 *uVar5;
  astruct_20 *iVar1;
  astruct_20 *iVar4;
  UINT16 *uVar1;
  
  iVar1 = (astruct_20 *)param_1;
  uVar5 = (astruct_20 *)((ulong)param_1 >> 0x10);
  set_struct_1008_687a(param_1,param_4);
  iVar1->field_0xde = param_2;
  iVar1->field_0xe0 = 0x0;
  param_1->field_0x0 = 0x6378;
  iVar1->field_0x2 = 0x1008;
  puVar1 = extraout_DX;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0x5b)),s_DanBrotherton_1050_0302);
  l_hgdiobj_1 = GetStockObject16(0x1000);
  iVar1->hgdiobj_field_0xc6 = l_hgdiobj_1;
  l_hcursor_1 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar1->hcursor_field_0xc4 = l_hcursor_1;
  iVar1->field_0xc8 = 0x200b;
  iVar1->field_0xac = 0x45000000;
  iVar1->field_0xbc = *(UINT16 *)((int)param_4 + 0x8);
  l_struct_2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_5,puVar1,unaff_DI);
  uVar1 = (UINT16 *)((ulong)l_struct_2 >> 0x10);
  iVar1->field_0xb4 = 0x0;
  iVar1->field_0xb6 = 0x0;
  iVar1->field_0xb8 = *(UINT16 *)((int)l_struct_2 + 0xa);
  iVar1->field_0xba = *(UINT16 *)((int)l_struct_2 + 0xc);
  iVar1->field_0xca = param_3;
  win_ui_reg_class_1008_96d2(param_1,0x1010,param_5);
  return param_1;
}



void __stdcall16far destroy_win_1008_628e(ULONG param_1,HWND16 param_2)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)((int)param_1 + 0xd2) + 0x14);
  (**ppcVar1)(param_2,(undefined4 *)((int)param_1 + 0xd2),param_1._2_2_);
  DestroyWindow16(param_2);
  *(undefined2 *)((int)param_1 + 0x8) = 0x0;
  return;
}



void __stdcall16far fill_rect_1008_62c0(HWND16 param_1)

{
  RECT16 local_2e [0x2];
  RECT16 *pRStack38;
  HDC16 HStack36;
  PAINTSTRUCT16 local_22;
  
  HStack36 = BeginPaint16(param_1,&local_22);
  pRStack38 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_2e);
  FillRect16((HDC16)s_tile2_bmp_1050_1538,pRStack38,(HBRUSH16)local_2e);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return;
}



void __stdcall16far pass1_1008_6330(ushort *param_1,byte param_2)

{
  astruct_456 *uVar1;
  undefined2 uVar2;
  
  uVar1 = (astruct_456 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



void __stdcall16far file_1008_6414(ulong *param_1,ulong param_2,ushort param_3,uchar *param_4)

{
  code **ppcVar1;
  undefined *puVar2;
  uint uVar3;
  undefined2 extraout_DX;
  int iVar4;
  undefined2 uVar5;
  astruct_76 *paStack42;
  undefined local_26 [0x24];
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x0;
  *(undefined4 *)(iVar4 + 0x4) = 0x0;
  puVar2 = local_26;
  struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3,puVar2),0x1,(char *)param_2,(ushort)param_4);
  mem_op_1000_179c(0x1e,param_4,0x1000);
  paStack42 = (astruct_76 *)CONCAT22(param_4,puVar2);
  uVar3 = (uint)param_4 | (uint)puVar2;
  if (uVar3 == 0x0) {
    *param_1 = 0x0;
  }
  else {
    puVar2 = local_26;
    struct_op_1008_3f92(paStack42,(astruct_83 *)CONCAT22(param_3,puVar2));
    *(undefined **)param_1 = puVar2;
    *(uint *)(iVar4 + 0x2) = uVar3;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x14);
  (**ppcVar1)();
  *(uint *)(iVar4 + 0x4) = (uint)puVar2;
  *(undefined2 *)(iVar4 + 0x6) = extraout_DX;
  close_file_1008_496c(local_26,param_3);
  return;
}



void __stdcall16far pass1_1008_64a2(uint *param_1)

{
  uint uVar1;
  code **ppcVar2;
  
  uVar1 = *(uint *)((int)param_1 + 0x2);
  if ((uVar1 | (uint)(undefined4 *)*param_1) != 0x0) {
    ppcVar2 = (code **)*(undefined4 *)*param_1;
    (**ppcVar2)();
  }
  return;
}



void __stdcall16far pass1_1008_64c8(ulong *param_1,ulong param_2,int param_3,uint param_4,uchar *param_5)

{
  int iVar1;
  int iVar2;
  undefined2 extraout_DX;
  undefined2 uVar3;
  int iVar4;
  int iVar5;
  int iStack8;
  ulong uStack6;
  
  if (*param_1 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_5,0x1000);
  if (((uint)param_5 | param_4) == 0x0) {
    param_4 = 0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1008_6604((astruct_85 *)CONCAT22(param_5,param_4),(int)param_2,(int)(param_2 >> 0x10));
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_4);
  iStack8 = 0x0;
  while (param_2 = param_2 & 0xffff0000 | (ulong)((int)param_2 - 0x1), (int)param_2 != 0x0) {
    iVar1 = param_3 + 0x1;
    iVar4 = param_3 >> 0xf;
    pass1_1008_4544(*param_1);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(uStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_3),param_2._2_2_);
    param_3 = iVar1;
    iStack8 = iVar2;
  }
  return;
}



void __stdcall16far pass1_1008_6562(ulong *param_1,ulong param_2,int param_3,uint param_4,uchar *param_5)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iStack8;
  ulong uStack6;
  
  if (*param_1 == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_5,0x1000);
  uVar3 = (uint)param_5 | param_4;
  if (uVar3 == 0x0) {
    param_4 = 0x0;
    uVar3 = 0x0;
  }
  else {
    pass1_1008_405c((astruct_76 *)CONCAT22(param_5,param_4),*(ulong *)((int)param_1 + 0x4),(int)param_2,
                    (int)(param_2 >> 0x10));
  }
  uStack6 = CONCAT22(uVar3,param_4);
  iStack8 = 0x0;
  while (param_2 = param_2 & 0xffff0000 | (ulong)((int)param_2 - 0x1), (int)param_2 != 0x0) {
    iVar1 = param_3 + 0x1;
    iVar4 = param_3 >> 0xf;
    pass1_1008_4544(*param_1);
    iVar2 = iStack8 + 0x1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(uStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_3),param_2._2_2_);
    param_3 = iVar1;
    iStack8 = iVar2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_op_1008_6604(astruct_85 *param_1,int param_2,int param_3)

{
  undefined4 *puVar1;
  int iVar3;
  astruct_85 *iVar4;
  astruct_84 *iVar2;
  undefined2 uVar4;
  undefined2 uVar5;
  long lVar6;
  
  pass1_1008_4016((astruct_76 *)param_1);
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_85 *)param_1;
  *(undefined2 *)param_1 = 0x685a;
  iVar4->field_0x2 = 0x1008;
  lVar6 = mem_op_1000_0a48(0x1,0x28,0x0,_PTR_LOOP_1050_5f2c,0x1000);
  *(int *)&iVar4->field_0x10 = (int)lVar6;
  *(undefined2 *)((int)&iVar4->field_0x10 + 0x2) = (int)((ulong)lVar6 >> 0x10);
  iVar3 = param_3 * 0x8 + 0x1f;
  iVar3 = ((int)(iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  *(int *)&iVar4->field_0x18 = iVar3;
  *(int *)((int)&iVar4->field_0x18 + 0x2) = iVar3 >> 0xf;
  lVar6 = mem_op_1000_0a48(0x1,(uint)((long)iVar3 * (long)param_2),(int)((ulong)((long)iVar3 * (long)param_2) >> 0x10),
                           _PTR_LOOP_1050_5f2c,0x1000);
  uVar5 = (undefined2)((ulong)lVar6 >> 0x10);
  iVar4->field_0x6 = (int)lVar6;
  iVar4->field_0x8 = uVar5;
  iVar4->field_0x14 = iVar4->field_0x6;
  iVar4->field_0x16 = uVar5;
  *iVar4->field_0x10 = 0x28;
  puVar1 = iVar4->field_0x10;
  *(long *)((int)puVar1 + 0x4) = (long)param_3;
  puVar1 = iVar4->field_0x10;
  uVar5 = (undefined2)((ulong)puVar1 >> 0x10);
  iVar2 = (astruct_84 *)puVar1;
  iVar2->field_0x8 = param_2;
  iVar2->field_0xa = param_2 >> 0xf;
  puVar1 = iVar4->field_0x10;
  *(undefined2 *)((int)puVar1 + 0xc) = 0x1;
  puVar1 = iVar4->field_0x10;
  *(undefined2 *)((int)puVar1 + 0xe) = 0x8;
  puVar1 = iVar4->field_0x10;
  *(undefined4 *)((int)puVar1 + 0x10) = 0x0;
  puVar1 = iVar4->field_0x10;
  *(long *)((int)puVar1 + 0x14) = iVar4->field_0x18 * (long)param_2;
  puVar1 = iVar4->field_0x10;
  *(undefined4 *)((int)puVar1 + 0x20) = 0x100;
  puVar1 = iVar4->field_0x10;
  *(undefined4 *)((int)puVar1 + 0x24) = 0x100;
  return;
}
