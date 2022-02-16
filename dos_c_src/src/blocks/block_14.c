
void __stdcall16far struct_op_1008_48fe(astruct_81 *param_1,ushort param_2,char *param_3,ushort param_4)

{
  ushort uVar1;
  astruct_81 *iVar2;
  ushort uVar3;
  
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_81 *)param_1;
  *(undefined2 *)param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = 0x0;
  *(undefined4 *)&iVar2->field_0x8 = 0x0;
  iVar2->field_0xc = 0xffff;
  iVar2->field_0xe = 0x0;
  iVar2->field_0x12 = 0x0;
  iVar2->field_0x16 = 0x0;
  iVar2->field_0x1a = 0x0;
  iVar2->field_0x1e = 0x0;
  iVar2->field_0x22 = param_2;
  *(int *)param_1 = (int)&PTR_LOOP_1050_4c4c;
  iVar2->field_0x2 = 0x1008;
  uVar1 = str_op_1008_60e8(param_3,param_4);
  iVar2->field_0x8 = uVar1;
  iVar2->field_0xa = param_4;
  return;
}



void __stdcall16far close_file_1008_496c(undefined2 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 uVar3;
  code **ppcVar4;
  int iVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = (int)&PTR_LOOP_1050_4c4c;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  puVar1 = (undefined4 *)*(uint *)(iVar5 + 0x4);
  uVar2 = *(uint *)(iVar5 + 0x6);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar5 + 0x8),0x1000);
  if (*(long *)(iVar5 + 0x1a) != 0x0) {
    uVar3 = *(undefined4 *)(iVar5 + 0x1a);
    call_fn_ptr_1000_0dc6((u16)uVar3,(u16)((ulong)uVar3 >> 0x10),0x1000);
  }
  if (*(int *)(iVar5 + 0xc) != -0x1) {
    _lclose16(0x1000);
  }
  *param_1 = 0x389a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint16_t __stdcall16far read_file_1008_49e8(ulong param_1,uint16_t param_2,uint16_t param_3)

{
  HFILE16 HVar1;
  int iVar2;
  ulong uVar3;
  ulong uVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *extraout_DX;
  int iVar7;
  int unaff_DI;
  undefined2 uVar8;
  uint16_t h_file;
  ushort unaff_SS;
  long lVar9;
  int local_18;
  undefined4 uStack22;
  undefined2 uStack10;
  uchar *puStack8;
  undefined4 uStack6;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(long *)(iVar7 + 0x8) != 0x0) {
    if (*(int *)(iVar7 + 0x1e) != 0x0) {
      return (uint16_t)(uchar *)param_3;
    }
    h_file = param_2;
    if (*(int *)(iVar7 + 0xc) == -0x1) {
      h_file = (uint16_t)s_tile2_bmp_1050_1538;
      HVar1 = _lopen16((LPCSTR)param_2,0x0);
      *(HFILE16 *)(iVar7 + 0xc) = HVar1;
      if (HVar1 == 0xffff) {
        return (uint16_t)(uchar *)param_3;
      }
    }
    uStack6 = 0x0;
    lVar9 = WIN16_hread(h_file,0xe,ZEXT24(&local_18) << 0x10);
    param_3 = (uint16_t)(uchar *)((ulong)lVar9 >> 0x10);
    if (((int)lVar9 == 0xe) && ((uchar *)param_3 == (uchar *)0x0)) {
      uStack6 = uStack22;
      if (local_18 == (int)&PTR_LOOP_1050_4d42) {
        _llseek16((HFILE16)s_tile2_bmp_1050_1538,0x0,0x0);
        lVar9 = mem_op_1000_0a48(0x1,(uint)uStack6,(int)((ulong)uStack6 >> 0x10),_PTR_LOOP_1050_5f2c,0x1000);
        puVar6 = (uchar *)((ulong)lVar9 >> 0x10);
        *(undefined2 *)(iVar7 + 0x1a) = (int)lVar9;
        *(uchar **)(iVar7 + 0x1c) = puVar6;
        if (((uint)puVar6 | *(uint *)(iVar7 + 0x1a)) == 0x0) {
          return (uint16_t)puVar6;
        }
        lVar9 = WIN16_hread(0x1000,(SEGPTR)uStack6,
                            CONCAT22((int)*(undefined4 *)(iVar7 + 0x1a),(int)((ulong)uStack6 >> 0x10)));
        puVar5 = (uchar *)((ulong)lVar9 >> 0x10);
        uStack10 = (undefined2)lVar9;
        puStack8 = puVar5;
        _lclose16((HFILE16)s_tile2_bmp_1050_1538);
        *(undefined2 *)(iVar7 + 0xc) = 0xffff;
        *(undefined2 *)(iVar7 + 0x1e) = 0x1;
        *(undefined4 *)(iVar7 + 0xe) = *(undefined4 *)(iVar7 + 0x1a);
        uVar3 = *(ulong *)(iVar7 + 0x1a);
        iVar2 = (int)uVar3;
        uVar3 = uVar3 & 0xffff0000;
        *(ulong *)(iVar7 + 0x12) = uVar3 | iVar2 + 0xe;
        uVar3 = uVar3 | iVar2 + 0x436;
        *(ulong *)(iVar7 + 0x16) = uVar3;
        mem_op_1000_179c(0x14,puVar5,0x1000);
        puVar6 = (uchar *)((uint)puVar5 | (uint)uVar3);
        if (puVar6 == (uchar *)0x0) {
          *(undefined4 *)(iVar7 + 0x4) = 0x0;
        }
        else {
          uVar4 = *(ulong *)(iVar7 + 0x12);
          uVar4 = uVar4 & 0xffff0000 | (ulong)((int)uVar4 + 0x28);
          struct_op_1008_4c98((astruct_76 *)(uVar3 & 0xffff | ZEXT24(puVar5) << 0x10),uVar4,0x100);
          *(undefined2 *)(iVar7 + 0x4) = (int)uVar4;
          *(uchar **)(iVar7 + 0x6) = extraout_DX;
          puVar6 = extraout_DX;
        }
        if (*(int *)(iVar7 + 0x22) != 0x0) {
          pass1_1008_4b8e(param_1,puVar6,unaff_DI,unaff_SS);
          return (uint16_t)puVar6;
        }
        return (uint16_t)puVar6;
      }
    }
    _lclose16((HFILE16)s_tile2_bmp_1050_1538);
    *(undefined2 *)(iVar7 + 0xc) = 0xffff;
  }
  return param_3;
}



ulong __stdcall16far pass1_1008_4b5e(ulong *param_1)

{
  code **ppcVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(int *)(iVar3 + 0x1e) == 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x8);
    iVar2 = (**ppcVar1)();
    if (iVar2 == 0x0) {
      return 0x0;
    }
  }
  return CONCAT22(*(undefined2 *)(iVar3 + 0x6),*(undefined2 *)(iVar3 + 0x4));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_4b8e(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ulong uVar1;
  undefined2 uVar2;
  ushort *puVar3;
  int iStack18;
  int iStack16;
  int iStack10;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,param_2,param_3);
  uVar2 = (undefined2)((ulong)puVar3 >> 0x10);
  uVar1 = *(ulong *)((int)puVar3 + 0x18);
  iStack18 = *(int *)((int)puVar3 + 0x16) / 0x2;
  for (iStack16 = 0x0; iStack10 = (int)uVar1, uVar2 = (undefined2)(param_1 >> 0x10), iStack16 < iStack18;
      iStack16 = iStack16 + 0x1) {
    pass1_1008_4d26(*(ulong *)((int)param_1 + 0x4),
                    (ushort *)(uVar1 & 0xffff0000 | (ulong)(uint)(iStack16 * 0x4 + iStack10)),iStack16);
  }
  for (iStack18 = 0x100 - iStack18; iStack18 < 0x100; iStack18 = iStack18 + 0x1) {
    pass1_1008_4d26(*(ulong *)((int)param_1 + 0x4),
                    (ushort *)(uVar1 & 0xffff0000 | (ulong)(uint)(iStack16 * 0x4 + iStack10)),iStack18);
    iStack16 = iStack16 + 0x1;
  }
  return;
}



ulong __stdcall16far file_1008_4c26(ulong param_1,byte param_2)

{
  close_file_1008_496c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1008_4c58(ushort *param_1)

{
  astruct_394 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_394 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x1;
  *param_1 = 0x4f1c;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far struct_op_1008_4c98(astruct_76 *param_1,ulong param_2,ushort param_3)

{
  astruct_76 *iVar1;
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *(ulong *)&iVar1->field_0x4 = param_2;
  iVar1->field_0xc = param_3;
  iVar1->field_0xe = (undefined4 *)0x0;
  iVar1->field_0x12 = 0x0;
  param_1->field_0x0 = 0x4f1c;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_4cdc(ushort *param_1)

{
  astruct_454 *iVar2;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_454 *)param_1;
  *param_1 = 0x4f1c;
  iVar2->field_0x2 = 0x1008;
  fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0xe,0x1000);
  if (iVar2->field_0x12 != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0x4,0x1000);
  }
  *param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  return;
}



ushort __stdcall16far pass1_1008_4d26(ulong param_1,ushort *param_2,int param_3)

{
  int *piVar1;
  undefined2 uVar2;
  long lVar3;
  astruct_650 *iVar5;
  astruct_649 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_650 *)param_1;
  if (((iVar5->field_0x4 != 0x0) && (-0x1 < param_3)) &&
     (piVar1 = &iVar5->field_0xc, *piVar1 != param_3 && param_3 <= *piVar1)) {
    uVar2 = *(undefined2 *)((int)param_2 + 0x2);
    lVar3 = iVar5->field_0x4;
    uVar4 = (undefined2)((ulong)lVar3 >> 0x10);
    iVar4 = (astruct_649 *)lVar3;
    *(ushort *)(iVar4 + param_3 * 0x4) = *param_2;
    *(undefined2 *)(iVar4 + param_3 * 0x4 + 0x2) = uVar2;
    return 0x1;
  }
  return 0x0;
}



ulong __stdcall16far pass1_1008_4d72(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x6),*(undefined2 *)((int)param_1 + 0x4));
}



void __stdcall16far pass1_1008_4d84(astruct_90 *param_1,ulong param_2,uchar *param_3)

{
  ushort uVar1;
  astruct_90 *iVar3;
  undefined2 uVar2;
  undefined2 uVar3;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_90 *)param_1;
  uVar3 = (undefined2)(param_2 >> 0x10);
  if (iVar3->field_0x12 != 0x0) {
    iVar3->field_0xc = *(int *)((int)param_2 + 0xc);
    fn_ptr_1000_17ce((astruct_18 *)iVar3->field_0x4,0x1000);
    iVar3->field_0x4 = 0x0;
    uVar1 = iVar3->field_0xc << 0x2;
    mem_op_1000_179c(uVar1,param_3,0x1000);
    *(ushort *)&iVar3->field_0x4 = uVar1;
    *(uchar **)((int)&iVar3->field_0x4 + 0x2) = param_3;
  }
  if (iVar3->field_0xc != 0x100) {
    return;
  }
  pass1_1000_48a8(iVar3->field_0x4,*(ulong *)((int)param_2 + 0x4),0x400);
  return;
}



HPALETTE16 __stdcall16far palette_op_1008_4e08(astruct_13 *param_1,BOOL16 param_2,ushort param_3,HDC16 param_4)

{
  HPALETTE16 HVar1;
  
  create_palette_1008_4e38(param_1,param_4,param_3);
  HVar1 = SelectPalette16(param_4,0x0,param_2);
  RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  return HVar1;
}



// WARNING: Unable to use type for symbol uVar3

void __stdcall16far create_palette_1008_4e38(astruct_13 *in_struct_1,LOGPALETTE *in_log_palette_2,uchar *param_3)

{
  int *piVar1;
  undefined2 *puVar2;
  ushort uVar4;
  astruct_13 *local_struct_1;
  int iVar5;
  int iVar6;
  UINT16 uVar8;
  UINT16 uVar9;
  UINT16 uVar10;
  int iStack14;
  UCHAR *puStack12;
  UCHAR *puStack8;
  undefined2 *uVar3;
  
  uVar8 = (UINT16)((ulong)in_struct_1 >> 0x10);
  local_struct_1 = (astruct_13 *)in_struct_1;
  uVar4 = (local_struct_1->field_0xc + 0x2) * 0x4;
  if (local_struct_1->field_0xe == (undefined2 *)0x0) {
    in_log_palette_2 = (LOGPALETTE *)&PTR_LOOP_1050_1000;
    mem_op_1000_179c(uVar4,param_3,0x1000);
    *(ushort *)&local_struct_1->field_0xe = uVar4;
    *(uchar **)((int)&local_struct_1->field_0xe + 0x2) = param_3;
    *local_struct_1->field_0xe = 0x300;
    uVar3 = local_struct_1->field_0xe;
    *(int *)((int)uVar3 + 0x2) = local_struct_1->field_0xc;
    puVar2 = local_struct_1->field_0xe;
    puStack8 = (UCHAR *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0x4));
    puStack12 = local_struct_1->field_0x4;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = &local_struct_1->field_0xc;
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar9 = (UINT16)((ulong)puStack12 >> 0x10);
      iVar5 = (int)puStack12;
      *puStack8 = *(UCHAR *)(iVar5 + 0x2);
      uVar10 = (UINT16)((ulong)puStack8 >> 0x10);
      iVar6 = (int)puStack8;
      *(undefined *)(iVar6 + 0x1) = *(undefined *)(iVar5 + 0x1);
      *(UCHAR *)(iVar6 + 0x2) = *puStack12;
      *(undefined *)(iVar6 + 0x3) = 0x0;
      iStack14 = iStack14 + 0x1;
      puStack8 = (UCHAR *)((ulong)puStack8 & 0xffff0000 | (ulong)(iVar6 + 0x4));
      puStack12 = (UCHAR *)((ulong)puStack12 & 0xffff0000 | (ulong)(iVar5 + 0x4));
    }
  }
  CreatePalette16(in_log_palette_2);
  return;
}



ushort * __stdcall16far pass1_1008_4ef6(ushort *param_1,byte param_2)

{
  pass1_1008_4cdc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
file_and_draw_op_1008_4f20(ushort *param_1,ulong param_2,ushort param_3,ulong param_4,ushort param_5)

{
  undefined4 uVar1;
  ushort b_force_background;
  COLORREF color;
  COLORREF color_00;
  uint x;
  uint16_t uVar2;
  LPCSTR output;
  int iVar3;
  undefined2 uVar4;
  astruct_43 *paVar5;
  ulong uVar6;
  DEVMODEA *init_data;
  HDC16 local_2c;
  LPCSTR pCStack42;
  LPCSTR pCStack40;
  undefined local_26 [0x24];
  
  pass1_1008_4016((astruct_76 *)param_1);
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(ulong *)(iVar3 + 0x1e) = param_4;
  *(ushort *)(iVar3 + 0x22) = param_3;
  *(ulong *)(iVar3 + 0x24) = param_2;
  *param_1 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x9;
  *(undefined2 *)(iVar3 + 0x2) = 0x1008;
  paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x2,param_5);
  uVar2 = (uint16_t)((ulong)paVar5 >> 0x10);
  struct_op_1008_48fe((astruct_81 *)CONCAT22(param_5,local_26),0x1,(char *)paVar5,uVar2);
  read_file_1008_49e8(CONCAT22(param_5,local_26),0x1010,uVar2);
  pass1_1008_5068((astruct_76 *)param_1,(astruct_83 *)CONCAT22(param_5,local_26));
  pass1_1008_47cc((astruct_76 *)param_1);
  pass1_1008_4834((astruct_76 *)param_1);
  init_data = (DEVMODEA *)0x0;
  uVar6 = pass1_1008_4772((astruct_76 *)param_1);
  output = (LPCSTR)(uVar6 >> 0x10);
  pCStack42 = (LPCSTR)uVar6;
  pCStack40 = output;
  local_2c = CreateDC16((LPCSTR)0x1010,pCStack42,output,init_data);
  b_force_background = palette_op_1008_46e4((ulong)param_1,(ushort)&local_2c,(ushort)output,(int)s_tile2_bmp_1050_1538);
  color = SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0xffff);
  color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538,*(COLORREF *)(iVar3 + 0x22));
  x = str_op_1000_3da4(*(char **)(iVar3 + 0x1e));
  uVar1 = *(undefined4 *)(iVar3 + 0x1e);
  TextOut16(0x1000,x,(INT16)uVar1,(char *)((ulong)uVar1 >> 0x10),0x0);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,color);
  SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color_00);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  close_file_1008_496c(local_26,param_5);
  return;
}



void __stdcall16far pass1_1008_5068(astruct_76 *param_1,astruct_83 *param_2)

{
  struct_op_1008_4214(param_1,param_2);
  return;
}



ushort * __stdcall16far pass1_1008_507c(ushort *param_1,byte param_2)

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_50c2(astruct_110 *param_1,ulong param_2,ulong param_3,uint *param_4,ulong param_5)

{
  astruct_110 *iVar1;
  uint uVar1;
  
  param_1->field_0x0 = *param_4;
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_110 *)param_1;
  iVar1->field_0x2 = *(undefined2 *)((int)param_4 + 0x2);
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = param_5;
  iVar1->field_0x10 = 0x0;
  pass1_1008_52fc((uint *)((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10));
  return;
}



void __stdcall16far pass1_1008_5118(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x10) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x10);
    call_fn_ptr_1000_0dc6((u16)uVar1,(u16)((ulong)uVar1 >> 0x10),0x1000);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_5134(ulong param_1)

{
  uint *puVar1;
  int iVar2;
  uint uVar3;
  long lVar4;
  int iVar5;
  int iVar6;
  undefined2 uVar7;
  int iStack16;
  long lStack14;
  ulong uStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  lVar4 = *(long *)(iVar6 + 0x4) * *(long *)(iVar6 + 0x8);
  lVar4 = mem_op_1000_0a48(0x1,(uint)lVar4,(int)((ulong)lVar4 >> 0x10),_PTR_LOOP_1050_5f2c,0x1000);
  uVar3 = (uint)((ulong)lVar4 >> 0x10);
  *(undefined2 *)(iVar6 + 0x10) = (int)lVar4;
  *(uint *)(iVar6 + 0x12) = uVar3;
  if ((uVar3 | *(uint *)(iVar6 + 0x10)) == 0x0) {
    return;
  }
  iVar5 = *(int *)(iVar6 + 0x8);
  iVar2 = *(int *)(iVar6 + 0xa);
  lVar4 = CONCAT22(iVar2 - (uint)(iVar5 == 0x0),iVar5 + -0x1) * *(long *)(iVar6 + 0x4);
  puVar1 = (uint *)(iVar6 + 0x10);
  uVar3 = (uint)lVar4;
  uStack10 = CONCAT22(((int)((ulong)lVar4 >> 0x10) + (uint)CARRY2(uVar3,*puVar1)) * 0x100 + *(int *)(iVar6 + 0x12),
                      uVar3 + *puVar1);
  lStack14 = CONCAT22(iVar2,iVar5);
  iStack16 = *(int *)(iVar6 + 0x2);
  while (lStack14 != 0x0) {
    iVar2 = iStack16 + 0x1;
    iVar5 = iStack16 >> 0xf;
    pass1_1008_4544(*(ulong *)(iVar6 + 0xc));
    pass1_1000_48a8(uStack10,CONCAT22(iVar5,iStack16),*(int *)(iVar6 + 0x4));
    iVar5 = *(int *)(iVar6 + 0x4);
    uVar3 = -iVar5;
    uStack10 = CONCAT22((int)(uStack10 >> 0x10) +
                        ((uint)CARRY2((uint)uStack10,uVar3) - (*(int *)(iVar6 + 0x6) + (uint)(iVar5 != 0x0))) * 0x100,
                        (uint)uStack10 + uVar3);
    iStack16 = iVar2;
    lStack14 = lStack14 + -0x1;
  }
  return;
}



void __stdcall16far pass1_1008_5236(ulong param_1)

{
  uint *puVar1;
  int iVar2;
  uint uVar3;
  long lVar4;
  int iVar5;
  astruct_109 *iVar6;
  undefined2 uVar6;
  bool bVar7;
  int iStack12;
  long lStack10;
  uint uStack6;
  int iStack4;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_109 *)param_1;
  iVar5 = iVar6->field_0x8;
  iVar2 = iVar6->field_0xa;
  lVar4 = CONCAT22(iVar2 - (uint)(iVar5 == 0x0),iVar5 + -0x1) * *(long *)&iVar6->field_0x4;
  puVar1 = &iVar6->field_0x10;
  uVar3 = (uint)lVar4;
  uStack6 = uVar3 + *puVar1;
  iStack4 = ((int)((ulong)lVar4 >> 0x10) + (uint)CARRY2(uVar3,*puVar1)) * 0x100 + iVar6->field_0x12;
  lStack10 = CONCAT22(iVar2,iVar5);
  iStack12 = iVar6->field_0x2;
  while (lStack10 != 0x0) {
    iVar2 = iStack12 + 0x1;
    iVar5 = iStack12 >> 0xf;
    pass1_1008_4544(iVar6->field_0xc);
    pass1_1000_48a8(CONCAT22(iVar5,iStack12),CONCAT22(iStack4,uStack6),*(int *)&iVar6->field_0x4);
    iVar5 = *(int *)&iVar6->field_0x4;
    uVar3 = -iVar5;
    bVar7 = CARRY2(uStack6,uVar3);
    uStack6 = uStack6 + uVar3;
    iStack4 = iStack4 + ((uint)bVar7 - (iVar6->field_0x6 + (uint)(iVar5 != 0x0))) * 0x100;
    iStack12 = iVar2;
    lStack10 = lStack10 + -0x1;
  }
  return;
}



void __stdcall16far pass1_1008_52fc(uint *param_1)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  long lVar4;
  undefined2 uVar5;
  int iVar6;
  int iVar7;
  astruct_111 *iVar8;
  undefined2 uVar8;
  ulong uVar9;
  uint uStack14;
  int iStack12;
  
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  iVar8 = (astruct_111 *)param_1;
  uVar9 = pass1_1008_4772(iVar8->field_0xc);
  uVar5 = (undefined2)(uVar9 >> 0x10);
  iVar7 = (int)uVar9;
  iVar6 = *(int *)(iVar7 + 0x4);
  uVar3 = iVar6 - 0x1;
  iVar6 = *(int *)(iVar7 + 0x6) - (uint)(iVar6 == 0x0);
  lVar4 = *(long *)(iVar7 + 0x8) + -0x1;
  uVar2 = *param_1;
  puVar1 = &iVar8->field_0x4;
  iVar7 = ((int)uVar2 >> 0xf) + iVar8->field_0x6 + (uint)CARRY2(uVar2,*puVar1);
  if ((iVar6 <= iVar7) && ((iVar6 < iVar7 || (uVar3 < uVar2 + *puVar1)))) {
    iVar8->field_0x4 = uVar3 - uVar2;
    iVar8->field_0x6 = (iVar6 - ((int)uVar2 >> 0xf)) - (uint)(uVar3 < uVar2);
  }
  uVar2 = iVar8->field_0x2;
  puVar1 = &iVar8->field_0x8;
  iVar6 = ((int)uVar2 >> 0xf) + iVar8->field_0xa + (uint)CARRY2(uVar2,*puVar1);
  iStack12 = (int)((ulong)lVar4 >> 0x10);
  if ((iStack12 <= iVar6) && ((uStack14 = (uint)lVar4, iStack12 < iVar6 || (uStack14 < uVar2 + *puVar1)))) {
    iVar8->field_0x8 = uStack14 - uVar2;
    iVar8->field_0xa = (iStack12 - ((int)uVar2 >> 0xf)) - (uint)(uStack14 < uVar2);
  }
  return;
}



ulong * __stdcall16far pass1_1008_5394(ulong *param_1)

{
  *param_1 = 0x0;
  return param_1;
}



void __stdcall16far pass1_1008_53aa(void)

{
  return;
}



void __cdecl16far mci_send_command_1008_53ae(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  DWORD DVar1;
  CHAR local_432 [0x400];
  ushort local_32;
  undefined2 uStack48;
  ushort local_2e;
  undefined2 uStack44;
  uint uStack34;
  uint uStack32;
  undefined4 local_1e;
  int iStack26;
  undefined2 uStack22;
  undefined2 uStack20;
  ulong uStack18;
  undefined4 uStack14;
  undefined2 uStack10;
  undefined2 uStack8;
  ushort uStack6;
  
  local_1e = 0x0;
  uStack22 = 0x28c;
  uStack20 = SUB42(&USHORT_1050_1050,0x0);
  uStack18 = param_1;
  uStack14 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x4000;
  uStack6 = param_2;
  DVar1 = mciSendCommand16(param_3,(UINT16)&local_1e,CONCAT22(0x200,param_4),0x8030003);
  uStack32 = (uint)(DVar1 >> 0x10);
  uStack34 = (uint)DVar1;
  if (iStack26 != 0x0) {
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,&local_2e),(WNDCLASS16 *)0x0,0xc);
    local_2e = param_2;
    uStack44 = 0x0;
    DVar1 = mciSendCommand16(0x1000,(UINT16)&local_2e,CONCAT22(0x2,param_4),0x8060000);
    uStack32 = (uint)(DVar1 >> 0x10);
    uStack34 = (uint)DVar1;
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
    local_32 = param_2;
    uStack48 = 0x0;
    DVar1 = mciSendCommand16((UINT16)s_tile2_bmp_1050_1538,(UINT16)&local_32,CONCAT22(0x1,param_4),0x8040000);
    uStack32 = (uint)(DVar1 >> 0x10);
    uStack34 = (uint)DVar1;
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
init_op_1008_54aa(uchar *param_1,char *param_2,uchar *param_3,uchar *param_4,ushort param_5,ushort param_6,
                 ushort param_7,ushort param_8)

{
  code **ppcVar1;
  uint uVar3;
  ushort in_CX;
  ushort in_DX;
  uchar *puVar4;
  ushort extraout_DX;
  ushort uVar5;
  ushort extraout_DX_00;
  ushort uVar6;
  ushort extraout_DX_01;
  ulong uVar7;
  ulong *puStack12;
  ulong uVar2;
  
  if (param_3 != (uchar *)0x0) {
    return;
  }
  dos3_call_op_1000_435c((UINT16 *)0x0,in_CX,in_DX,&stack0xfffe,param_8);
  pass1_1000_4d0c(param_5);
  pass1_1000_1fea();
  _PTR_LOOP_1050_03a0 = mem_op_1000_1902(0x0,0x32,0x0,0x12,0x1000,in_DX);
  _PTR_LOOP_1050_029c = mem_op_1000_1902(0x0,0x64,0x0,0xc,0x1000,(int)(_PTR_LOOP_1050_03a0 >> 0x10));
  _PTR_LOOP_1050_4fb8 = mem_op_1000_1902(0x0,0x64,0x0,0x10,0x1000,(int)(_PTR_LOOP_1050_029c >> 0x10));
  _PTR_LOOP_1050_68a2 = mem_op_1000_1902(0x0,0x64,0x0,0xe,0x1000,(int)(_PTR_LOOP_1050_4fb8 >> 0x10));
  _PTR_LOOP_1050_5744 = mem_op_1000_1902(0x0,0x1f4,0x0,0x42,0x1000,(int)(_PTR_LOOP_1050_68a2 >> 0x10));
  uVar7 = mem_op_1000_1902(0x0,0x32,0x0,0x6,0x1000,(int)(_PTR_LOOP_1050_5744 >> 0x10));
  puVar4 = (uchar *)(uVar7 >> 0x10);
  PTR_LOOP_1050_5768 = (undefined *)uVar7;
  PTR_LOOP_1050_038c = param_4;
  PTR_LOOP_1050_038e = param_3;
  PTR_LOOP_1050_0390 = param_1;
  PTR_LOOP_1050_576a = puVar4;
  uVar3 = str_op_1008_60e8(param_2,(ushort)puVar4);
  _PTR_LOOP_1050_0392 = CONCAT22(puVar4,uVar3);
  mem_op_1000_179c(0xc,puVar4,0x1000);
  if (((uint)puVar4 | uVar3) == 0x0) {
    uVar3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    struct_op_1008_0000((ushort *)CONCAT13((char)((uint)puVar4 >> 0x8),CONCAT12((char)puVar4,uVar3)));
    uVar5 = extraout_DX;
  }
  puStack12 = (ulong *)CONCAT22(uVar5,uVar3);
  if (_PTR_LOOP_1050_0392 != 0x0) {
    ppcVar1 = (code **)((int)*puStack12 + 0x4);
    (**ppcVar1)(0x1000,(char)uVar3,uVar5,(int)_PTR_LOOP_1050_0392,(char)((ulong)_PTR_LOOP_1050_0392 >> 0x10));
  }
  uVar2 = *puStack12;
  ppcVar1 = (code **)uVar2 + 0x4;
  (**ppcVar1)(0x1000,uVar3,(char)uVar5);
  uVar6 = extraout_DX_00;
  win_msg_op_1008_9498((MSG *)&PTR_LOOP_1050_1000,(MSG16 *)param_8);
  if (puStack12 != (ulong *)0x0) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)(0x1000,uVar3,(char)uVar5,0x1);
    uVar6 = extraout_DX_01;
  }
  uVar7 = mem_op_1000_1b68(uVar6,0x1000,(ushort)_PTR_LOOP_1050_03a0,(ushort)(_PTR_LOOP_1050_03a0 >> 0x10));
  uVar7 = mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_029c,
                           (ushort)(_PTR_LOOP_1050_029c >> 0x10));
  uVar7 = mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_4fb8,
                           (ushort)(_PTR_LOOP_1050_4fb8 >> 0x10));
  uVar7 = mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_68a2,
                           (ushort)(_PTR_LOOP_1050_68a2 >> 0x10));
  mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_5744,(ushort)(_PTR_LOOP_1050_5744 >> 0x10));
  return;
}



void __stdcall16far def_win_proc_1008_5632(ulong *param_1,WPARAM16 param_2,ushort param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  HWND16 unaff_CS;
  undefined2 unaff_SS;
  undefined2 uVar2;
  undefined4 *puStack6;
  
  uVar2 = SUB42(&USHORT_1050_1050,0x0);
  puStack6 = (undefined4 *)GetWindowLong16(unaff_CS,0x0);
  if (((uint)((ulong)puStack6 >> 0x10) | (uint)puStack6) == 0x0) {
    if (param_4 != 0x1) {
      DefWindowProc16((HWND16)s_tile2_bmp_1050_1538,(UINT16)param_1,param_2,CONCAT22(param_4,param_3));
      return;
    }
    puStack6 = (undefined4 *)*param_1;
    SetWindowLong16((HWND16)s_tile2_bmp_1050_1538,(INT16)puStack6,(ulong)puStack6 >> 0x10);
    pass1_1008_9628(puStack6,param_5);
  }
  ppcVar1 = (code **)((int)*puStack6 + 0x1c);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack6,(int)((ulong)puStack6 >> 0x10),param_1,param_2,param_3,param_4,
              uVar2);
  return;
}



ushort * __stdcall16far struct_op_1008_56b4(astruct_76 *param_1)

{
  astruct_82 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_82 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  param_1->field_0x0 = (ushort)s__s__d_1050_573a;
  iVar1->field_0x2 = 0x1008;
  return &param_1->field_0x0;
}



BOOL16 __stdcall16far cleanup_palette_1008_56e2(ULONG param_1,HDC16 param_2)

{
  HPALETTE16 HVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  HVar1 = SelectPalette16(param_2,0x0,*(BOOL16 *)((int)param_1 + 0x4));
  *(HPALETTE16 *)((int)param_1 + 0x4) = HVar1;
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return 0x1;
}



ushort * __stdcall16far pass1_1008_570e(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far set_struct_1008_574a(astruct_21 *param_1)

{
  astruct_21 *iVar1;
  astruct_21 *uVar1;
  
  uVar1 = (astruct_21 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_21 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xa = 0x1;
  param_1->field_0x0 = 0x5bc4;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_5784(ulong *param_1,ulong param_2)

{
  *param_1 = param_2;
  *(undefined4 *)((int)param_1 + 0x4) = 0x0;
  return;
}



void __stdcall16far pass1_1008_57a4(ulong *param_1,ulong param_2)

{
  *param_1 = param_2;
  *(undefined4 *)((int)param_1 + 0x4) = 0x0;
  return;
}



void __stdcall16far pass1_1008_57c4(ushort *param_1)

{
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  *param_1 = 0x5bc4;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  pass1_1008_5830((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return;
}

