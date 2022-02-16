
void __stdcall16far win_1018_598c(astruct *param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  astruct_131 *iVar1;
  undefined2 uVar2;
  ushort unaff_SS;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_131 *)param_1;
  get_dc_1018_4db0(iVar1->field_0xf2,iVar1->field_0x8,0x1008);
  mem_op_1000_179c(0x2a,(uchar *)param_3,0x1000);
  uVar1 = param_3 | param_2;
  if (uVar1 != 0x0) {
    pass1_1018_5b06((astruct_132 *)param_2,param_3,iVar1->field_0x8,unaff_SS);
    iVar1->field_0xee = param_2;
    iVar1->field_0xf0 = uVar1;
    return;
  }
  *(undefined4 *)&iVar1->field_0xee = 0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1018_5a2e(astruct_18 *param_1,byte param_2)

{
  param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  pass1_1018_58b6((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_5b06(astruct_132 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  ulong *puVar3;
  uint uVar5;
  undefined4 uVar6;
  uchar *puVar7;
  undefined2 uVar8;
  uchar *puVar9;
  uint uVar10;
  int unaff_DI;
  ushort *puVar11;
  astruct_76 *paVar12;
  ulong uVar13;
  undefined2 uVar14;
  undefined2 uVar15;
  astruct_132 *paVar16;
  ushort uVar17;
  undefined local_c [0x6];
  ushort *puStack6;
  uint uVar4;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x14 = (ulong *)0x0;
  param_1->field_0x18 = 0x0;
  puVar11 = pass1_1008_3e38((ushort *)CONCAT22(param_2,&param_1->field_0x1c));
  *(int *)CONCAT22(param_2,param_1) = (int)&PTR_LOOP_1050_5e1a;
  param_1->field_0x2 = 0x1018;
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,(uchar *)((ulong)puVar11 >> 0x10),unaff_DI);
  puVar11 = pass1_1008_3e38((ushort *)CONCAT22(param_4,local_c));
  puVar7 = (uchar *)((ulong)puVar11 >> 0x10);
  pass1_1008_3f62((ushort *)CONCAT22(param_4,local_c),
                  (ushort *)((ulong)puStack6 & 0xffff0000 | (ulong)((int)puStack6 + 0xe)));
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x27,param_4,puVar7,unaff_DI);
  uVar8 = (undefined2)((ulong)puVar11 >> 0x10);
  *(int *)&param_1->field_0x14 = (int)puVar11;
  *(undefined2 *)((int)&param_1->field_0x14 + 0x2) = uVar8;
  uVar15 = 0x0;
  uVar14 = *(undefined2 *)&param_1->field_0x14;
  ppcVar2 = (code **)((int)*param_1->field_0x14 + 0x4);
  paVar16 = param_1;
  uVar17 = param_2;
  (**ppcVar2)();
  param_1->field_0x6 = param_1->field_0x14;
  puVar3 = param_1->field_0x14;
  puVar1 = (undefined4 *)*(ulong *)((int)puVar3 + 0xa);
  uVar6 = CONCAT22(param_2,&param_1->field_0xa);
  ppcVar2 = (code **)((int)*puVar1 + 0x8);
  (**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),uVar6,uVar14,uVar8,uVar15,paVar16,uVar17);
  param_1->field_0x12 = (int)uVar6;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_4);
  puVar3 = param_1->field_0x14;
  pass1_1008_3f62((ushort *)CONCAT22(param_2,&param_1->field_0x1c),
                  (ushort *)((ulong)puVar3 & 0xffff0000 | (ulong)((int)puVar3 + 0x52)));
  pass1_1008_3f32((int *)CONCAT22(param_2,&param_1->field_0x1c),(int *)CONCAT22(param_4,local_c));
  paVar12 = (astruct_76 *)pass1_1008_9f48((ulong)param_1->field_0x14);
  uVar13 = pass1_1008_4772(paVar12);
  puVar9 = (uchar *)(uVar13 >> 0x10);
  uVar4 = (uint)uVar13;
  puVar7 = puVar9;
  uVar5 = uVar4;
  mem_op_1000_179c(0x14,puVar9,0x1000);
  uVar10 = (uint)puVar7 | uVar5;
  if (uVar10 == 0x0) {
    param_1->field_0x18 = 0x0;
  }
  else {
    pass1_1008_50c2((astruct_110 *)CONCAT22(puVar7,uVar5),*(ulong *)(uVar4 + 0x8),*(ulong *)(uVar4 + 0x4),
                    (uint *)CONCAT22(param_2,&param_1->field_0x1c),(ulong)puVar1);
    *(uint *)&param_1->field_0x18 = uVar5;
    *(uint *)((int)&param_1->field_0x18 + 0x2) = uVar10;
  }
  pass1_1008_5134(param_1->field_0x18);
  param_1->field_0x22 = param_1->field_0x1c;
  param_1->field_0x24 = param_1->field_0x1e;
  param_1->field_0x26 = *(int *)(uVar4 + 0x4) + param_1->field_0x22 + 0x1;
  param_1->field_0x28 = *(int *)(uVar4 + 0x8) + param_1->field_0x24 + 0x1;
  return;
}



void __stdcall16far pass1_1018_5cc8(ushort *param_1,ushort param_2)

{
  uint uVar1;
  astruct_18 *paVar2;
  astruct_508 *iVar3;
  uint uVar3;
  
  uVar3 = (uint)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_508 *)param_1;
  *param_1 = (ushort)&PTR_LOOP_1050_5e1a;
  iVar3->field_0x2 = 0x1018;
  paVar2 = *(astruct_18 **)&iVar3->field_0x18;
  uVar1 = iVar3->field_0x1a;
  if ((uVar1 | (uint)paVar2) != 0x0) {
    pass1_1008_5118((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  if (iVar3->field_0x14 != 0x0) {
    pass1_1010_1ea6(iVar3->field_0x14,(ulong)param_1 & 0xffff | (ulong)uVar3 << 0x10,param_2);
    pass1_1010_1dda(iVar3->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}



void __stdcall16far invalidate_rect_1018_5d32(ulong param_1,int param_2,HWND16 param_3)

{
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  InvalidateRect16(param_3,(RECT16 *)0x0,(int)param_1 + 0x22);
  return;
}



void __stdcall16far misc_draw_op_1018_5d6c(ulong param_1,HWND16 param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  ushort unaff_SS;
  astruct_76 *paVar6;
  undefined2 uVar7;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar7 = *(undefined2 *)(iVar4 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar3 = *(undefined4 *)(iVar4 + 0x14);
  puVar1 = (undefined4 *)*(ulong *)((int)uVar3 + 0xa);
  paVar6 = (astruct_76 *)pass1_1008_9f48(*(ulong *)(iVar4 + 0x14));
  pass1_1008_5236(*(ulong *)(iVar4 + 0x18));
  pass1_1008_4480((ulong)puVar1,(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar4 + 0x1c)),paVar6,unaff_SS);
  ppcVar2 = (code **)((int)*puVar1 + 0x4);
  (**ppcVar2)(0x1008,(int)puVar1,(int)((ulong)puVar1 >> 0x10),0x0,param_1 & 0xffff0000 | (ulong)(iVar4 + 0xa),uVar7);
  EndPaint16(0x1008,&local_22);
  return;
}



ushort * __stdcall16far pass1_1018_5df4(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1018_5cc8(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far pass1_1018_5e26(astruct_57 *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd0,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)param_1 = 0x6128;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  *(undefined2 *)((int)param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_5e5a(ushort *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x6128;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c((astruct_18 *)param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far pass1_1018_5e86(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x6c);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1018_5e9a(astruct_1 *param_1,ushort param_2)

{
  ulong uVar1;
  ULONG *pUVar2;
  INT16 IVar3;
  undefined *puVar4;
  uchar *in_DX;
  uchar *puVar5;
  uchar *puVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  int unaff_DI;
  undefined2 uVar10;
  ushort *puVar11;
  undefined local_28 [0x12];
  int iStack22;
  undefined2 uStack20;
  int iStack18;
  int iStack16;
  RECT16 local_e;
  int iStack8;
  INT16 *pIStack6;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_2,in_DX,unaff_DI);
  puVar5 = (uchar *)((ulong)puVar11 >> 0x10);
  uVar7 = (uint)puVar11;
  uVar10 = (undefined2)((ulong)param_1 >> 0x10);
  iVar9 = (int)param_1;
  *(uint *)(iVar9 + 0x8e) = uVar7;
  *(uchar **)(iVar9 + 0x90) = puVar5;
  mem_op_1000_179c(0x12,puVar5,0x1000);
  puVar6 = (uchar *)((uint)puVar5 | uVar7);
  if (puVar6 == (uchar *)0x0) {
    *(undefined4 *)(iVar9 + 0x92) = 0x0;
  }
  else {
    pass1_1018_6198((ushort *)CONCAT22(puVar5,uVar7),(ulong)param_1,*(ushort *)(iVar9 + 0x6),puVar6,unaff_DI,param_2);
    *(uint *)(iVar9 + 0x92) = uVar7;
    *(uchar **)(iVar9 + 0x94) = puVar6;
  }
  uVar1 = *(ulong *)(iVar9 + 0x8e);
  pIStack6 = (INT16 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0xa));
  GetClientRect16(0x1000,&local_e);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  *(int *)((int)pIStack6 + 0x6) = IVar3 + iStack8;
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,puVar6,unaff_DI);
  uStack20 = (undefined2)((ulong)puVar11 >> 0x10);
  iStack22 = (int)puVar11;
  iStack16 = *(int *)(iStack22 + 0xa);
  iStack18 = *(int *)(iStack22 + 0xc);
  uVar10 = (undefined2)((ulong)pIStack6 >> 0x10);
  *(int *)((int)pIStack6 + 0x2) = (iStack18 - *(int *)((int)pIStack6 + 0x6)) / 0x2;
  uVar7 = iStack16 >> 0xf;
  *pIStack6 = iStack16 / 0x2 + 0x3;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_28),0x1,0x0,0x100);
  while( true ) {
    puVar4 = local_28;
    pass1_1028_e4ec(CONCAT22(param_2,puVar4));
    uVar8 = uVar7 | (uint)puVar4;
    if (uVar8 == 0x0) break;
    pUVar2 = *(ULONG **)(puVar4 + 0x10);
    uVar7 = uVar8;
    if (pUVar2 != (ULONG *)0x0) {
      pass1_1000_3cea((ulong)param_1 & 0xffff0000 | (ulong)(iVar9 + 0x10),*pUVar2);
      uVar7 = uVar8;
    }
  }
  uVar10 = (undefined2)((ulong)pIStack6 >> 0x10);
  iVar9 = (int)pIStack6;
  SetWindowPos16((HWND16)&USHORT_1050_1028,0x44,*(INT16 *)(iVar9 + 0x6),*(INT16 *)(iVar9 + 0x4),*(INT16 *)(iVar9 + 0x2),
                 *pIStack6,0x0);
  return;
}



void __stdcall16far pass1_1018_5ffa(ulong param_1)

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
  pass1_1010_1dda(*(ulong *)(iVar4 + 0x8e));
  *(undefined4 *)(iVar4 + 0x8e) = 0x0;
  return;
}



ushort __stdcall16far pass1_1018_6048(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void __stdcall16far
set_window_text_1018_6066
          (UINT16 param_1,UINT16 param_2,SEGPTR in_win_text_3,UINT16 param_4,INT16 dialog_id_5,HWND16 in_hwnd_6)

{
  GetDlgItem16(in_hwnd_6,dialog_id_5);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,in_win_text_3);
  return;
}



void __stdcall16far set_window_text_1018_6086(ULONG param_1,LPSTR param_2,WORD *param_3)

{
  wsprintf16(param_2,&stack0xfff4,param_3);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1be);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)&stack0xfff4);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,&stack0xfff4,param_3);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1bf);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)&stack0xfff4);
  return;
}



ushort * __stdcall16far pass1_1018_6102(ushort *param_1,byte param_2)

{
  pass1_1018_5e5a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1018_6198(ushort *param_1,ulong param_2,ushort param_3,uchar *param_4,int param_5,ushort param_6)

{
  astruct_657 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_657 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *(undefined4 *)&iVar1->field_0x6 = 0x0;
  iVar1->field_0xa = param_2;
  *param_1 = 0x66c0;
  iVar1->field_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_6,param_4,param_5);
  iVar1->field_0x6 = (int)puVar2;
  iVar1->field_0x8 = (int)((ulong)puVar2 >> 0x10);
  return;
}



void __stdcall16far pass1_1018_620c(ushort *param_1)

{
  astruct_509 *iVar1;
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_509 *)param_1;
  *param_1 = 0x66c0;
  iVar1->field_0x2 = 0x1018;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



// WARNING: Inlined function: struct_1010_4d5c

void __stdcall16far unk_draw_op_1018_623e(ulong param_1,HWND16 param_2,ushort param_3)

{
  int *piVar1;
  code **ppcVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  ushort uVar5;
  HDC16 *pHVar6;
  int iVar7;
  HPEN16 handle;
  HGDIOBJ16 HVar8;
  HBRUSH16 handle_00;
  uchar *puVar9;
  ushort uVar10;
  int iVar11;
  int iVar12;
  undefined *puVar13;
  undefined2 uVar14;
  undefined2 uVar15;
  ulong uVar16;
  int iVar17;
  undefined uVar18;
  undefined uVar19;
  undefined local_38 [0x6];
  undefined2 local_32;
  undefined2 uStack48;
  undefined4 uStack46;
  undefined2 uStack42;
  undefined4 *puStack40;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  puVar13 = &stack0xfffe;
  uVar14 = (undefined2)(param_1 >> 0x10);
  iVar11 = (int)param_1;
  uVar15 = *(undefined2 *)(iVar11 + 0x4);
  local_24 = BeginPaint16(param_2,&local_22);
  puStack40 = (undefined4 *)pass1_1010_4c2c(*(ulong *)(iVar11 + 0x6));
  pHVar6 = &local_24;
  ppcVar2 = (code **)((int)*puStack40 + 0x8);
  (**ppcVar2)(0x1010,(int)puStack40,(int)((ulong)puStack40 >> 0x10),pHVar6,param_3,uVar15);
  *(HDC16 **)(iVar11 + 0x10) = pHVar6;
  uVar3 = *(undefined4 *)(iVar11 + 0x6);
  uStack42 = *(undefined2 *)((int)uVar3 + 0x30);
  uVar3 = *(undefined4 *)(iVar11 + 0x6);
  uStack46 = *(undefined4 *)*(undefined4 *)((int)uVar3 + 0x12);
  uStack48 = 0x14;
  local_32 = 0x0;
  pass1_1008_3e38((ushort *)CONCAT22(param_3,local_38));
  while (*(int *)(puVar13 + -0x38) < *(int *)(puVar13 + -0x28)) {
    iVar11 = *(int *)(puVar13 + -0x38) * 0x4;
    uVar3 = *(undefined4 *)(puVar13 + -0x2c);
    uVar16 = pass1_1008_4772(*(astruct_76 **)(iVar11 + (int)uVar3));
    puVar9 = (uchar *)(uVar16 >> 0x10);
    *(int *)(puVar13 + -0x44) = (int)uVar16;
    *(uchar **)(puVar13 + -0x42) = puVar9;
    uVar3 = *(undefined4 *)(puVar13 + 0x6);
    pass1_1018_642e((ushort)uVar3,(ushort)((ulong)uVar3 >> 0x10),
                    (int *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,puVar13 + -0x30)),
                    *(int *)((int)uVar16 + 0x8));
    uVar3 = *(undefined4 *)(puVar13 + -0x30);
    pass1_1008_3e76((ushort *)CONCAT22(param_3,puVar13 + -0x36),0x0,(ushort)uVar3,(ushort)((ulong)uVar3 >> 0x10));
    uVar3 = *(undefined4 *)(puVar13 + -0x2c);
    pass1_1008_4480(*(ulong *)(puVar13 + -0x26),(ushort *)CONCAT22(param_3,puVar13 + -0x36),
                    *(astruct_76 **)((int)uVar3 + iVar11),param_3);
    iVar11 = *(int *)(puVar13 + -0x38);
    uVar3 = *(undefined4 *)(puVar13 + -0x30);
    uVar14 = (undefined2)uVar3;
    uVar18 = (undefined)((ulong)uVar3 >> 0x10);
    uVar19 = (undefined)((ulong)uVar3 >> 0x18);
    uVar3 = *(undefined4 *)(puVar13 + -0x44);
    uVar15 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar12 = (int)uVar3;
    iVar7 = *(int *)(iVar12 + 0x4) + *(int *)(puVar13 + -0x2e);
    iVar12 = *(int *)(iVar12 + 0x8) + *(int *)(puVar13 + -0x30);
    uVar3 = *(undefined4 *)(puVar13 + 0x6);
    uVar3 = *(undefined4 *)((int)uVar3 + 0x6);
    iVar17 = (int)uVar3;
    uVar15 = (undefined2)((ulong)uVar3 >> 0x10);
    if (*(long *)(iVar17 + 0x1a) == 0x0) {
      uVar5 = *(int *)(iVar17 + 0x30) << 0x3;
      mem_op_1000_179c(uVar5,puVar9,0x1000);
      *(ushort *)(iVar17 + 0x1a) = uVar5;
      *(uchar **)(iVar17 + 0x1c) = puVar9;
    }
    uVar3 = *(undefined4 *)(iVar17 + 0x1a);
    iVar11 = iVar11 * 0x8;
    *(undefined2 *)((int)uVar3 + iVar11) = CONCAT11(uVar19,uVar18);
    uVar3 = *(undefined4 *)(iVar17 + 0x1a);
    *(undefined2 *)((int)uVar3 + iVar11 + 0x2) = uVar14;
    uVar3 = *(undefined4 *)(iVar17 + 0x1a);
    *(int *)((int)uVar3 + iVar11 + 0x4) = iVar7;
    uVar3 = *(undefined4 *)(iVar17 + 0x1a);
    *(int *)((int)uVar3 + iVar11 + 0x6) = iVar12;
    uVar3 = *(undefined4 *)(puVar13 + -0x44);
    piVar1 = (int *)(puVar13 + -0x2e);
    *piVar1 = *piVar1 + (-(uint)(*(int *)(puVar13 + -0x38) == 0x0) & 0x5) + 0x14 + *(int *)((int)uVar3 + 0x4);
    piVar1 = (int *)(puVar13 + -0x38);
    *piVar1 = *piVar1 + 0x1;
  }
  puVar4 = *(undefined4 **)(puVar13 + -0x26);
  ppcVar2 = (code **)((int)*puVar4 + 0x4);
  (**ppcVar2)(0x1008,(int)puVar4,(int)((ulong)puVar4 >> 0x10),0x0,0x0,(char)puVar13 + -0x22,param_3);
  handle = CreatePen16(0x1008,0x25,0x100);
  *(HPEN16 *)(puVar13 + -0x3a) = handle;
  HVar8 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  *(HGDIOBJ16 *)(puVar13 + -0x3c) = HVar8;
  handle_00 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  *(HBRUSH16 *)(puVar13 + -0x3e) = handle_00;
  HVar8 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
  *(HGDIOBJ16 *)(puVar13 + -0x40) = HVar8;
  draw_line_1018_6444(*(ulong *)(puVar13 + 0x6),(int)s_tile2_bmp_1050_1538);
  uVar3 = *(undefined4 *)(puVar13 + 0x6);
  uVar16 = pass1_1010_4dc8(*(ulong *)((int)uVar3 + 0x6));
  uVar10 = (ushort)(uVar16 >> 0x10);
  uVar5 = (ushort)uVar16;
  draw_op_1018_6544(*(ulong *)(puVar13 + 0x6),(int *)(uVar16 & 0xffff | (ulong)uVar10 << 0x10),param_3);
  pass1_1018_6630(*(ulong *)(puVar13 + 0x6),uVar5,uVar10);
  uVar3 = *(undefined4 *)(puVar13 + 0x6);
  SelectPalette16(0x1010,0x0,*(BOOL16 *)((int)uVar3 + 0x10));
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar13 + -0x3c));
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar13 + -0x40));
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,(PAINTSTRUCT16 *)(puVar13 + -0x20));
  return;
}



void __stdcall16far pass1_1018_642e(ushort param_1,ushort param_2,int *param_3,int param_4)

{
  *param_3 = 0x64 - param_4 >> 0x1;
  return;
}



void __stdcall16far draw_line_1018_6444(ulong param_1,HDC16 param_2)

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



void __stdcall16far draw_op_1018_6544(ulong param_1,int *param_2,ushort param_3)

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
    uVar2 = pass1_1018_659a((ushort)param_1,uVar3,(ushort *)CONCAT22(param_3,local_a),(uchar *)((ulong)puVar1 >> 0x10),
                            param_3);
    draw_polygon_1018_661c((ushort)param_1,uVar3,CONCAT22((int)uVar2,0x3),0x1008);
  }
  return;
}



ulong __stdcall16far pass1_1018_659a(ushort param_1,ushort param_2,ushort *param_3,uchar *param_4,ushort param_5)

{
  int *piVar1;
  int iStack18;
  int local_6;
  int local_4;
  
  piVar1 = &local_6;
  pass1_1008_3e94(param_3,(ushort *)CONCAT22(param_5,piVar1),(ushort *)CONCAT22(param_5,&local_4));
  mem_op_1000_179c(0xc,param_4,0x1000);
  for (iStack18 = 0x0; iStack18 < 0x3; iStack18 = iStack18 + 0x1) {
    piVar1[iStack18 * 0x2] = *(int *)(iStack18 * 0x4 + 0x4248) + local_4;
    piVar1[iStack18 * 0x2 + 0x1] = *(int *)(iStack18 * 0x4 + 0x424a) + local_6;
  }
  return CONCAT22(param_4,piVar1);
}



void __stdcall16far draw_polygon_1018_661c(ushort param_1,ushort param_2,ulong param_3,HDC16 param_4)

{
  Polygon16(param_4,(POINT16 *)param_3,(INT16)(param_3 >> 0x10));
  return;
}



void __stdcall16far pass1_1018_6630(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  ushort dialog_id_5;
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
      dialog_id_5 = pass1_1010_4f20((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),iStack12);
      uVar1 = *(undefined4 *)(iVar3 + 0xa);
      set_window_text_1018_6066((UINT16)uVar1,(UINT16)((ulong)uVar1 >> 0x10),SStack10,param_3,dialog_id_5,0x1010);
      uVar2 = str_op_1000_3da4((char *)CONCAT22(param_3,SStack10));
      SStack10 = SStack10 + uVar2 + 0x1;
    }
  }
  return;
}



ulong __stdcall16far pass1_1018_669a(ulong param_1,byte param_2)

{
  pass1_1018_620c((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1018_66cc(astruct_20 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uchar *extraout_DX;
  undefined2 uVar1;
  astruct_20 *iVar2;
  int unaff_DI;
  astruct_20 *uVar2;
  ushort *puVar2;
  
  unk_draw_op_1020_7f7a(param_1,0xa,CONCAT22(param_3,param_2));
  uVar2 = (astruct_20 *)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_20 *)param_1;
  *(undefined4 *)&iVar2[0x1].field_0xc = 0x0;
  iVar2[0x1].field_0x10 = 0x0;
  param_1->field_0x0 = 0x6880;
  iVar2->field_0x2 = 0x1018;
  ((astruct_20 *)(iVar2 + 0x1))->field_0x0 = 0x691c;
  iVar2[0x1].field_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xb,param_4,extraout_DX,unaff_DI);
  uVar1 = (undefined2)((ulong)puVar2 >> 0x10);
  *(int *)&iVar2[0x1].field_0x10 = (int)puVar2;
  *(undefined2 *)((int)&iVar2[0x1].field_0x10 + 0x2) = uVar1;
  *(undefined2 *)&iVar2[0x1].field_0x4 = *(undefined2 *)&iVar2[0x1].field_0x10;
  *(undefined2 *)((int)&iVar2[0x1].field_0x4 + 0x2) = uVar1;
  return;
}



void __stdcall16far pass1_1018_673c(ushort *param_1)

{
  astruct_510 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_510 *)param_1;
  *param_1 = 0x6880;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xe2 = 0x691c;
  iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint __stdcall16far pass1_1018_6768(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  uint uVar2;
  ushort uVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  uVar3 = (ushort)param_1;
  uVar2 = *(uint *)(uVar3 + 0xf0) | *(uint *)(uVar3 + 0xee);
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_2 = (ushort)((ulong)uVar5 >> 0x10);
    uVar2 = (uint)uVar5;
  }
  if (*(int *)(uVar3 + 0xea) == 0x0) {
    *(undefined2 *)(uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar3 + 0x8),0x16,param_2,uVar3,(ushort)&PTR_LOOP_1050_1038,
                            param_3);
    uVar2 = (uint)uVar5;
  }
  return uVar2;
}



void __stdcall16far window_op_1018_67b6(astruct *param_1)

{
  astruct_658 *in_AX;
  uchar *in_DX;
  uint uVar1;
  int iVar2;
  int unaff_DI;
  undefined2 uVar3;
  ushort unaff_SS;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),*(ushort *)(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = (uint)in_DX | (uint)in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_6924(in_AX,(ushort)in_DX,*(ushort *)(iVar2 + 0x8),unaff_DI,unaff_SS);
    *(astruct_658 **)(iVar2 + 0xee) = in_AX;
    *(uint *)(iVar2 + 0xf0) = uVar1;
    return;
  }
  *(undefined4 *)(iVar2 + 0xee) = 0x0;
  return;
}



void __stdcall16far pass1_1018_681a(ULONG param_1)

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



ushort * __stdcall16far pass1_1018_684c(ushort *param_1,byte param_2)

{
  param_1 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  pass1_1018_673c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_6924(astruct_658 *param_1,ushort param_2,ushort param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  uchar *extraout_DX;
  undefined2 uVar4;
  ushort *puVar5;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0x14 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x6a02;
  param_1->field_0x2 = 0x1018;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xb,param_5,extraout_DX,param_4);
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
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_5);
  return;
}



void __stdcall16far pass1_1018_69ac(ushort *param_1)

{
  astruct_511 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_511 *)param_1;
  *param_1 = 0x6a02;
  iVar1->field_0x2 = 0x1018;
  if (iVar1->field_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}



ushort * __stdcall16far pass1_1018_69dc(ushort *param_1,byte param_2)

{
  pass1_1018_69ac(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_20 * __stdcall16far
struct_op_1018_6a0e(astruct_20 *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,
                   ulong param_7,ushort param_8)

{
  int iVar1;
  ushort uVar2;
  
  unk_draw_op_1008_61b2(param_1,param_3,param_6,param_7,param_8);
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ushort *)(iVar1 + 0xea) = param_5;
  *(ushort *)(iVar1 + 0xec) = param_4;
  *(ushort *)(iVar1 + 0xee) = param_2;
  *(undefined2 *)(iVar1 + 0xf0) = 0x0;
  param_1->field_0x0 = 0x6c66;
  *(undefined2 *)(iVar1 + 0x2) = 0x1018;
  *(undefined2 *)(iVar1 + 0xe0) = 0x1;
  *(undefined2 *)(iVar1 + 0xe2) = 0x0;
  *(undefined2 *)(iVar1 + 0xe4) = 0x0;
  *(undefined4 *)(iVar1 + 0xe6) = 0x1df027f;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mixed_draw_op_1018_6a7a(astruct_28 *param_1,ushort param_2)

{
  uchar *in_DX;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar1;
  PAINTSTRUCT16 local_22;
  
  BeginPaint16(param_2,&local_22);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
  if (*(int *)((int)puVar1 + 0x20) == 0x0) {
    unk_destroy_window_op_1018_6bb6(param_1,0x1010);
    return;
  }
  mix_ui_op_1018_6adc(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mix_ui_op_1018_6adc(astruct_28 *param_1)

{
  int iVar1;
  int iVar2;
  ushort uVar3;
  uchar *in_DX;
  ushort uVar4;
  int iVar5;
  int unaff_DI;
  undefined2 uVar6;
  WNDCLASS16 *unaff_SS;
  ushort *puVar7;
  astruct_43 *paVar8;
  
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,(ushort)unaff_SS,in_DX,unaff_DI);
  uVar4 = (ushort)((ulong)puVar7 >> 0x10);
  iVar1 = *(int *)((int)puVar7 + 0xa);
  iVar2 = *(int *)((int)puVar7 + 0xc);
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (0x1 < iVar1 - *(int *)(iVar5 + 0xe6)) {
    uVar4 = iVar1 >> 0xf;
    *(int *)(iVar5 + 0xe2) = iVar1 / 0x2 - (*(int *)(iVar5 + 0xe6) + 0x1) / 0x2;
  }
  if (0x1 < iVar2 - *(int *)(iVar5 + 0xe8)) {
    uVar4 = iVar2 >> 0xf;
    *(int *)(iVar5 + 0xe4) = iVar2 / 0x2 - (*(int *)(iVar5 + 0xe8) + 0x1) / 0x2;
  }
  uVar3 = ShowCursor16(0x1010);
  if (*(int *)(iVar5 + 0xee) != 0x0) {
    win_1008_5c5c(unaff_SS,uVar3,uVar4,_PTR_LOOP_1050_02a0,*(ushort *)(iVar5 + 0xee));
    *(ushort *)(iVar5 + 0xf0) = uVar3;
  }
  paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,*(ushort *)(iVar5 + 0xec),(ushort)unaff_SS);
  mci_send_command_1008_53ae((ulong)paVar8,*(ushort *)(iVar5 + 0x8),0x1008,(ushort)unaff_SS);
  ShowCursor16(0x1008);
  unk_destroy_window_op_1018_6bb6(param_1,(int)s_tile2_bmp_1050_1538);
  return;
}
