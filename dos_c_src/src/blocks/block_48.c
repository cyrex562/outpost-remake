
astruct_18 * __stdcall16far pass1_1018_e5aa(astruct_18 *param_1,byte param_2)

{
  pass1_1018_e57a(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_e5dc(ushort param_1,astruct_20 *param_2,ushort param_3,ushort param_4)

{
  uchar *extraout_DX;
  undefined2 uVar1;
  astruct_20 *iVar2;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  unk_draw_op_1020_7f7a(param_2,0x9,CONCAT22(param_4,param_3));
  uVar2 = (undefined2)((ulong)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  *(undefined4 *)&iVar2[0x1].field_0xc = 0x0;
  iVar2[0x1].field_0x10 = 0x0;
  param_2->field_0x0 = 0xe790;
  iVar2->field_0x2 = 0x1018;
  ((astruct_20 *)(iVar2 + 0x1))->field_0x0 = 0xe82c;
  iVar2[0x1].field_0x2 = 0x1018;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xa,param_1,extraout_DX,unaff_DI);
  uVar1 = (undefined2)((ulong)puVar3 >> 0x10);
  *(int *)&iVar2[0x1].field_0x10 = (int)puVar3;
  *(undefined2 *)((int)&iVar2[0x1].field_0x10 + 0x2) = uVar1;
  *(undefined2 *)&iVar2[0x1].field_0x4 = *(undefined2 *)&iVar2[0x1].field_0x10;
  *(undefined2 *)((int)&iVar2[0x1].field_0x4 + 0x2) = uVar1;
  return;
}



void __stdcall16far pass1_1018_e64c(ushort *param_1)

{
  astruct_576 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_576 *)param_1;
  *param_1 = 0xe790;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xe2 = 0xe82c;
  iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint __stdcall16far pass1_1018_e678(ulong param_1,ushort param_2,ushort param_3)

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
    uVar5 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar3 + 0x8),0x15,param_2,uVar3,(ushort)&PTR_LOOP_1050_1038,
                            param_3);
    uVar2 = (uint)uVar5;
  }
  return uVar2;
}



void __stdcall16far window_op_1018_e6c6(astruct *param_1)

{
  astruct_660 *in_AX;
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
    pass1_1018_e834(in_AX,(ushort)in_DX,*(ushort *)(iVar2 + 0x8),unaff_DI,unaff_SS);
    *(astruct_660 **)(iVar2 + 0xee) = in_AX;
    *(uint *)(iVar2 + 0xf0) = uVar1;
    return;
  }
  *(undefined4 *)(iVar2 + 0xee) = 0x0;
  return;
}



void __stdcall16far pass1_1018_e72a(ULONG param_1)

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



astruct_18 * __stdcall16far pass1_1018_e75c(astruct_18 *param_1,byte param_2)

{
  param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  pass1_1018_e64c((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_e834(astruct_660 *param_1,ushort param_2,ushort param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  uchar *extraout_DX;
  undefined2 uVar4;
  ushort *puVar5;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0x14 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xe912;
  param_1->field_0x2 = 0x1018;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xa,param_5,extraout_DX,param_4);
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



void __stdcall16far pass1_1018_e8bc(ushort *param_1)

{
  astruct_577 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_577 *)param_1;
  *param_1 = 0xe912;
  iVar1->field_0x2 = 0x1018;
  if (iVar1->field_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}



astruct_18 * __stdcall16far pass1_1018_e8ec(astruct_18 *param_1,byte param_2)

{
  pass1_1018_e8bc(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_e91e(astruct_20 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ulong *puVar1;
  code **ppcVar2;
  undefined2 *puVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  undefined2 uVar5;
  int unaff_DI;
  ushort *puVar6;
  undefined2 uVar7;
  astruct_127 *iVar7;
  
  iVar7 = (astruct_127 *)param_1;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  unk_draw_op_1020_7f7a(param_1,0x3,CONCAT22(param_3,param_2));
  iVar7->field_0xee = 0x0;
  *(undefined4 *)&iVar7->field_0xf2 = 0x0;
  iVar7->field_0xf6 = (ulong *)0x0;
  param_1->field_0x0 = 0xebd0;
  iVar7->field_0x2 = 0x1018;
  iVar7->field_0xe2 = 0xec6c;
  iVar7->field_0xe4 = 0x1018;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,param_4,extraout_DX,unaff_DI);
  puVar4 = (uchar *)((ulong)puVar6 >> 0x10);
  iVar7->field_0xf2 = (int)puVar6;
  iVar7->field_0xf4 = puVar4;
  iVar7->field_0xe6 = iVar7->field_0xf2;
  iVar7->field_0xe8 = puVar4;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_4,puVar4,unaff_DI);
  *(int *)&iVar7->field_0xf6 = (int)puVar6;
  *(undefined2 *)((int)&iVar7->field_0xf6 + 0x2) = (int)((ulong)puVar6 >> 0x10);
  if (param_1 == (astruct_20 *)0x0) {
    puVar3 = (undefined2 *)0x0;
    uVar5 = 0x0;
  }
  else {
    puVar3 = &iVar7->field_0xe2;
    uVar5 = uVar7;
  }
  puVar1 = iVar7->field_0xf6;
  ppcVar2 = (code **)((int)*iVar7->field_0xf6 + 0x4);
  (**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),0xb,puVar3,uVar5);
  return;
}



void __stdcall16far pass1_1018_e9de(ushort *param_1)

{
  astruct_578 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_578 *)param_1;
  *param_1 = 0xebd0;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xe2 = 0xec6c;
  iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



void __stdcall16far post_win_msg_1018_ea0a(ushort param_1,ushort param_2,int param_3,HWND16 param_4)

{
  if (param_3 == 0xed) {
    PostMessage16(param_4,0x0,0x0,0x11100c6);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_ea66(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  undefined *puVar2;
  int iVar3;
  undefined2 uVar4;
  ushort *puVar5;
  undefined local_6 [0x4];
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0xee) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xee) + 0x8);
    (**ppcVar1)();
  }
  if (*(int *)(iVar3 + 0xea) == 0x0) {
    *(undefined2 *)(iVar3 + 0xea) = 0x1;
    puVar5 = pass1_1008_941a((ushort *)CONCAT22(param_2,local_6),0x1,0x95);
    puVar2 = local_6;
    win_1008_5c9e(_PTR_LOOP_1050_02a0,(ulong *)CONCAT22(param_2,puVar2),puVar2,(int)((ulong)puVar5 >> 0x10),param_2);
    *(int *)(iVar3 + 0xec) = (int)puVar2;
    unk_win_op_1010_7300(*(ulong *)(iVar3 + 0xf6),0x0,0x8,0x0);
  }
  return;
}



void __stdcall16far window_op_1018_eada(astruct *param_1)

{
  astruct_661 *in_AX;
  uchar *in_DX;
  uint uVar1;
  int iVar2;
  undefined2 uVar3;
  ushort unaff_SS;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),*(ushort *)(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x28,in_DX,0x1000);
  uVar1 = (uint)in_DX | (uint)in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_ec74(in_AX,(int)in_DX,*(ushort *)(iVar2 + 0x8),unaff_SS);
    *(astruct_661 **)(iVar2 + 0xee) = in_AX;
    *(uint *)(iVar2 + 0xf0) = uVar1;
    return;
  }
  *(undefined4 *)(iVar2 + 0xee) = 0x0;
  return;
}



void __stdcall16far pass1_1018_eb3e(ULONG param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar6 + 0xee);
  uVar2 = *(uint *)(iVar6 + 0xf0);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (*(long *)(iVar6 + 0xf6) != 0x0) {
    if (param_1 == 0x0) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = iVar6 + 0xe2;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6(*(ulong *)(iVar6 + 0xf6),CONCAT22(uVar5,iVar4),param_2);
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



astruct_18 * __stdcall16far pass1_1018_eb9c(astruct_18 *param_1,byte param_2)

{
  param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  pass1_1018_e9de((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_ec74(astruct_661 *param_1,int param_2,ushort param_3,ushort param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  undefined2 uVar4;
  uchar *extraout_DX;
  uchar *puVar5;
  ushort *puVar6;
  ulong uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  astruct_661 *paVar10;
  int iVar11;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x14 = (undefined4 *)0x0;
  pass1_1008_3e38((ushort *)CONCAT22(param_2,&param_1->field_0x18));
  puVar6 = pass1_1008_3e38((ushort *)CONCAT22(param_2,&param_1->field_0x1e));
  *(undefined4 *)&param_1->field_0x24 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x1cc;
  param_1->field_0x2 = 0x1020;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,param_4,(uchar *)((ulong)puVar6 >> 0x10),param_2);
  uVar4 = (undefined2)((ulong)puVar6 >> 0x10);
  *(int *)&param_1->field_0x14 = (int)puVar6;
  *(undefined2 *)((int)&param_1->field_0x14 + 0x2) = uVar4;
  uVar9 = 0x0;
  uVar8 = *(undefined2 *)&param_1->field_0x14;
  ppcVar2 = (code **)((int)*param_1->field_0x14 + 0x4);
  paVar10 = param_1;
  iVar11 = param_2;
  (**ppcVar2)();
  param_1->field_0x6 = param_1->field_0x14;
  puVar1 = param_1->field_0x14;
  pass1_1010_2b50((ushort)puVar1,(ushort)((ulong)puVar1 >> 0x10),(ushort *)CONCAT22(param_2,&param_1->field_0x18));
  uVar7 = pass1_1010_2b66((ulong)param_1->field_0x14);
  param_1->field_0x24 = (int)uVar7;
  param_1->field_0x26 = (int)(uVar7 >> 0x10);
  puVar1 = param_1->field_0x14;
  puVar1 = (undefined4 *)*(undefined4 *)((int)puVar1 + 0xa);
  uVar3 = CONCAT22(param_2,&param_1->field_0xa);
  ppcVar2 = (code **)((int)*puVar1 + 0x8);
  (**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),uVar3,uVar8,uVar4,uVar9,paVar10,iVar11);
  param_1->field_0x12 = (int)uVar3;
  puVar5 = extraout_DX;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_4);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar5,param_2);
  pass1_1008_3f62((ushort *)CONCAT22(param_2,&param_1->field_0x1e),
                  (ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)));
  pass1_1008_3f32((int *)CONCAT22(param_2,&param_1->field_0x18),(int *)CONCAT22(param_2,&param_1->field_0x1e));
  return;
}



void __stdcall16far pass1_1018_ed98(ushort *param_1,ushort param_2)

{
  astruct_579 *iVar1;
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_579 *)param_1;
  *param_1 = 0x1cc;
  iVar1->field_0x2 = 0x1020;
  if (iVar1->field_0x14 != 0x0) {
    pass1_1010_1ea6(iVar1->field_0x14,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,param_2);
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}



void __stdcall16far invalidate_rect_1018_edd8(ulong param_1,int param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  int local_16;
  int iStack20;
  int iStack18;
  int iStack16;
  ulong uStack14;
  undefined2 uStack10;
  uint uStack8;
  int local_6;
  int local_4;
  
  iVar1 = (int)param_1;
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x1) {
    *(undefined4 *)(iVar1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xc) {
    return;
  }
  pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0x18)),(ushort *)CONCAT22(param_3,&local_6),
                  (ushort *)CONCAT22(param_3,&local_4));
  uVar3 = pass1_1010_2b66(*(ulong *)(iVar1 + 0x14));
  uStack8 = (uint)(uVar3 >> 0x10);
  uStack10 = (undefined2)uVar3;
  uStack14 = pass1_1008_4772((astruct_76 *)(uVar3 & 0xffff | (ulong)uStack8 << 0x10));
  uVar2 = (undefined2)(uStack14 >> 0x10);
  local_16 = local_4;
  iStack20 = local_6;
  iStack18 = local_4 + *(int *)((int)uStack14 + 0x4);
  iStack16 = local_6 + *(int *)((int)uStack14 + 0x8);
  InvalidateRect16(0x1008,(RECT16 *)0x0,(BOOL16)&local_16);
  return;
}



void __stdcall16far unk_draw_op_1020_0000(ulong param_1,HWND16 param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  HWND16 hwnd;
  undefined2 uVar7;
  undefined local_c4 [0x6];
  undefined local_be [0x2];
  int local_b4;
  int iStack178;
  int aiStack176 [0x3c];
  int iStack56;
  int iStack48;
  astruct_76 *paStack46;
  int local_2a;
  int local_28;
  undefined4 *puStack38;
  PAINTSTRUCT16 local_22;
  
                    // Segment:    5
                    // Offset:     00033420
                    // Length:     efba
                    // Min Alloc:  efba
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  uVar7 = *(undefined2 *)(iVar5 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar2 = *(undefined4 *)(iVar5 + 0x14);
  puStack38 = (undefined4 *)*(undefined4 *)((int)uVar2 + 0xa);
  pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar5 + 0x18U)),(ushort *)CONCAT22(param_3,&local_2a),
                  (ushort *)CONCAT22(param_3,&local_28));
  hwnd = 0x1008;
  pass1_1008_4480((ulong)puStack38,(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar5 + 0x18U)),
                  *(astruct_76 **)(iVar5 + 0x24),param_3);
  paStack46 = (astruct_76 *)0x0;
  for (iStack48 = 0x0; iStack48 < 0x6; iStack48 = iStack48 + 0x1) {
    uVar2 = *(undefined4 *)(iVar5 + 0x14);
    hwnd = 0x1010;
    pass1_1010_2b78((ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),iStack48,CONCAT22(param_3,&local_b4));
    if (local_b4 == 0x0) {
      for (iStack56 = 0x0; iVar4 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 0x1) {
        iVar3 = iStack56 * 0x3;
        if (aiStack176[iStack56 * 0x3 + 0x2] != 0x0) {
          paStack46 = (astruct_76 *)pass1_1010_2b98(*(ulong *)(iVar5 + 0x14),aiStack176[iStack56 * 0x3 + 0x2]);
          pass1_1008_3e54((ushort *)CONCAT22(param_3,local_be),0x0,aiStack176[iVar4 * 0x3 + 0x1] + local_2a,
                          aiStack176[iVar3] + local_28);
          hwnd = 0x1008;
          pass1_1008_4480((ulong)puStack38,(ushort *)CONCAT22(param_3,local_be),paStack46,param_3);
        }
      }
    }
    else {
      _local_be = (int *)CONCAT22(param_3,aiStack176 + iStack178 * 0x3);
      if (aiStack176[iStack178 * 0x3 + 0x2] != 0x0) {
        paStack46 = (astruct_76 *)pass1_1010_2b98(*(ulong *)(iVar5 + 0x14),aiStack176[iStack178 * 0x3 + 0x2]);
        pass1_1008_3e54((ushort *)CONCAT22(param_3,local_c4),0x0,*(int *)((int)_local_be + 0x2) + local_2a,
                        *_local_be + local_28);
        hwnd = 0x1008;
        pass1_1008_4480((ulong)puStack38,(ushort *)CONCAT22(param_3,local_c4),paStack46,param_3);
      }
    }
  }
  ppcVar1 = (code **)((int)*puStack38 + 0x4);
  (**ppcVar1)(hwnd,(int)puStack38,(int)((ulong)puStack38 >> 0x10),0x0,0x0,iVar5 + 0xa,uVar6,uVar7);
  EndPaint16(hwnd,&local_22);
  return;
}



astruct_18 * __stdcall16far pass1_1020_01a6(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1018_ed98(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
pass1_1020_01d8(astruct_20 *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,
               ushort param_7,ulong param_8,ushort param_9)

{
  unk_draw_op_1008_61b2((astruct_20 *)CONCAT22(param_2,param_1),param_3,param_7,param_8,param_9);
  *(undefined4 *)(param_1 + 0x1) = 0x0;
  param_1[0x1].field_0x4 = 0x0;
  param_1[0x1].field_0x8 = param_6;
  param_1[0x1].field_0xa = param_5;
  param_1[0x1].field_0xc = param_4;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x45a;
  param_1->field_0x2 = 0x1020;
  return;
}



void __stdcall16far pass1_1020_022c(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_580 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_580 *)param_1;
  *param_1 = 0x45a;
  iVar4->field_0x2 = 0x1020;
  puVar1 = iVar4->field_0xe6;
  uVar2 = iVar4->field_0xe8;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0xd2));
  *param_1 = 0x380a;
  iVar4->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1020_028c(ushort param_1,ushort param_2,int param_3)

{
  pass1_1010_3c9e(*(long *)(param_1 + 0xe2));
  pass1_1008_68c6(param_1,param_2,param_3,0x1008);
  return;
}



void __stdcall16far pass1_1020_02ae(ULONG param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_3cd0(*(long *)(iVar4 + 0xe2));
  destroy_win_1008_628e(param_1,0x1008);
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe6);
  uVar2 = *(uint *)(iVar4 + 0xe8);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar4 + 0xe6) = 0x0;
  pass1_1010_1dda(*(ulong *)(iVar4 + 0xe2));
  *(undefined4 *)(iVar4 + 0xe2) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_1020_0316(astruct *param_1,uchar *param_2,ushort param_3)

{
  undefined4 uVar1;
  uint uVar2;
  uchar *puVar3;
  uchar *puVar4;
  astruct_273 *iVar1;
  int unaff_DI;
  undefined2 uVar5;
  ushort *puVar6;
  
  create_window_ex_1008_9760(param_1,0x1008);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x1,param_3,param_2,unaff_DI);
  puVar3 = (uchar *)((ulong)puVar6 >> 0x10);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_273 *)param_1;
  iVar1->field_0xe2 = (int)puVar6;
  iVar1->field_0xe4 = (int)puVar3;
  uVar1 = *(undefined4 *)&iVar1->field_0xe2;
  *(undefined2 *)((int)uVar1 + 0x16) = iVar1->field_0xea;
  uVar2 = iVar1->field_0xee;
  uVar1 = *(undefined4 *)&iVar1->field_0xe2;
  *(uint *)((int)uVar1 + 0x12) = uVar2;
  struct_1010_3c52(*(ulong *)&iVar1->field_0xe2,iVar1->field_0xec,param_3);
  mem_op_1000_179c(0x12,puVar3,0x1000);
  puVar4 = (uchar *)((uint)puVar3 | uVar2);
  if (puVar4 != (uchar *)0x0) {
    pass1_1020_04f6((ushort *)CONCAT22(puVar3,uVar2),iVar1->field_0x8,puVar4,unaff_DI,param_3);
    iVar1->field_0xe6 = uVar2;
    iVar1->field_0xe8 = (int)puVar4;
    return;
  }
  *(undefined4 *)&iVar1->field_0xe6 = 0x0;
  return;
}



void __stdcall16far post_msg_1020_03b2(ulong param_1,HWND16 param_2)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)uVar1 + 0x16)));
  return;
}



void __stdcall16far post_msg_1020_03d6(ulong param_1,HWND16 param_2)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)uVar1 + 0x16)));
  return;
}



void __stdcall16far post_msg_1020_03fa(ulong param_1,HWND16 param_2)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)uVar1 + 0x16)));
  return;
}



void __stdcall16far draw_op_1020_041e(ulong param_1,ushort param_2)

{
  fill_rect_1020_065e(*(undefined4 *)((int)param_1 + 0xe6),param_2);
  return;
}



astruct_18 * __stdcall16far pass1_1020_0434(astruct_18 *param_1,byte param_2)

{
  pass1_1020_022c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_04f6(ushort *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  int iVar2;
  undefined2 uVar3;
  uchar *extraout_DX;
  astruct_662 *iVar4;
  undefined2 uVar4;
  ushort *puVar5;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_662 *)param_1;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar4->field_0x2 = 0x1008;
  iVar4->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar4->field_0x2 = 0x1008;
  iVar4->field_0x6 = (undefined4 *)0x0;
  iVar4->field_0xa = 0x0;
  iVar4->field_0xc = 0x0;
  iVar4->field_0xe = 0x0;
  iVar4->field_0x10 = 0x0;
  *param_1 = 0x75a;
  iVar4->field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x1,param_5,param_3,param_4);
  uVar3 = (undefined2)((ulong)puVar5 >> 0x10);
  *(int *)&iVar4->field_0x6 = (int)puVar5;
  *(undefined2 *)((int)&iVar4->field_0x6 + 0x2) = uVar3;
  ppcVar1 = (code **)((int)*iVar4->field_0x6 + 0x4);
  (**ppcVar1)(0x1010,*(undefined2 *)&iVar4->field_0x6,uVar3,0x0,param_1);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_5,extraout_DX,param_4);
  iVar2 = (int)puVar5;
  iVar4->field_0xa = *(undefined2 *)(iVar2 + 0xa);
  iVar4->field_0xc = *(undefined2 *)(iVar2 + 0xc);
  pass1_1008_3e94((ushort *)((ulong)puVar5 & 0xffff0000 | (ulong)(iVar2 + 0xe)),
                  (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x10)),
                  (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0xe)));
  return;
}



void __stdcall16far pass1_1020_05d6(ushort *param_1,ushort param_2)

{
  astruct_581 *iVar1;
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_581 *)param_1;
  *param_1 = 0x75a;
  iVar1->field_0x2 = 0x1020;
  if (iVar1->field_0x6 != 0x0) {
    pass1_1010_1ea6(iVar1->field_0x6,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,param_2);
  }
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far post_win_msg_1020_061c(ulong param_1,int param_2,HWND16 param_3)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x6);
  PostMessage16(param_3,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)uVar1 + 0x16)));
  return;
}



void __stdcall16far fill_rect_1020_065e(undefined4 param_1,HWND16 in_win_handle_2)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  undefined2 local_brush_handle;
  undefined2 uStack50;
  int iStack48;
  int iStack46;
  RECT16 *local_rect_1;
  HDC16 *pHStack42;
  undefined4 *puStack40;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  local_24 = BeginPaint16(in_win_handle_2,&local_22);
  if (0x280 < *(int *)(iVar3 + 0xa)) {
    local_rect_1 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    local_brush_handle = 0x0;
    uStack50 = 0x0;
    iStack48 = *(int *)(iVar3 + 0xa) + -0x1;
    iStack46 = *(int *)(iVar3 + 0xc) + -0x1;
    FillRect16((HDC16)s_tile2_bmp_1050_1538,local_rect_1,(HBRUSH16)&local_brush_handle);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  }
  uVar2 = *(undefined4 *)(iVar3 + 0x6);
  puStack40 = (undefined4 *)*(undefined4 *)((int)uVar2 + 0xe);
  pHStack42 = &local_24;
  uVar2 = *puStack40;
  ppcVar1 = (code **)((int)uVar2 + 0x8);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack40,(int)((ulong)puStack40 >> 0x10),pHStack42);
  ppcVar1 = (code **)((int)uVar2 + 0x4);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,puStack40,*(undefined2 *)(iVar3 + 0x10),*(undefined2 *)(iVar3 + 0xe),&local_24)
  ;
  pHStack42 = (HDC16 *)SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,(BOOL16)pHStack42);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



astruct_18 * __stdcall16far pass1_1020_0734(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1020_05d6(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
struct_1020_0762(astruct_20 *param_1,ulong param_2,ulong *param_3,ushort param_4,ulong param_5,ulong param_6,
                ushort param_7)

{
  astruct_20 *iVar1;
  astruct_20 *uVar1;
  astruct_20 *paVar1;
  ushort uVar2;
  
  paVar1 = (astruct_20 *)param_1;
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1020_01d8(paVar1,uVar2,(ushort)param_2,(ushort)(param_2 >> 0x10),param_4,(ushort)param_5,
                  (ushort)(param_5 >> 0x10),param_6,param_7);
  paVar1[0x1].field_0xe = 0x0;
  paVar1[0x1].field_0x10 = *param_3;
  param_1->field_0x0 = 0x81a;
  paVar1->field_0x2 = 0x1020;
  return;
}



void __stdcall16far pass1_1020_07aa(ulong param_1,ushort param_2,int param_3,ushort param_4,ushort param_5)

{
  int iVar1;
  undefined2 uVar2;
  undefined local_16 [0x14];
  
  draw_op_1020_041e(param_1,param_4);
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0xf0) == 0x0) {
    *(undefined2 *)(iVar1 + 0xf0) = 0x1;
    pass1_1008_5bdc((astruct_79 *)CONCAT22(param_5,local_16),param_3,param_5);
    win_1008_5c9e(CONCAT22(param_5,local_16),(ulong *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0xf2)),local_16,param_2,
                  param_5);
    pass1_1008_5c34((ushort *)CONCAT22(param_5,local_16));
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_07f4(astruct_18 *param_1,byte param_2)

{
  pass1_1020_022c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_08b6(WNDCLASS16 *param_1,astruct_20 *param_2,UINT16 param_3,ULONG param_4)

{
  astruct_20 *iVar1;
  undefined2 uVar1;
  astruct_20 *paVar2;
  
  paVar2 = unk_draw_op_1008_61b2(param_2,0x1,param_3,param_4,(UINT16)param_1);
  uVar1 = (undefined2)((ulong)param_2 >> 0x10);
  iVar1 = (astruct_20 *)param_2;
  *(undefined2 *)&iVar1[0x1].field_0x4 = 0x0;
  *(undefined4 *)((int)&iVar1[0x1].field_0x4 + 0x2) = 0x0;
  param_2->field_0x0 = 0xb0e;
  iVar1->field_0x2 = 0x1020;
  win_1008_5c5c(param_1,0x0,(ushort)((ulong)paVar2 >> 0x10),_PTR_LOOP_1050_02a0,0x1d4);
  return;
}

