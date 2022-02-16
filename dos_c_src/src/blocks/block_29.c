
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_451a(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ushort uVar1;
  undefined2 uVar2;
  ushort *puVar3;
  ulong uVar4;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_2,param_3);
  uVar1 = (ushort)((ulong)puVar3 >> 0x10);
  uVar4 = pass1_1010_ec40((ushort)puVar3,uVar1,*(ulong *)((int)param_1 + 0x6c),(ushort)puVar3,uVar1);
  uVar2 = (undefined2)(uVar4 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)uVar4 + 0x4),*(undefined2 *)((int)uVar4 + 0x2));
}



ulong __stdcall16far pass1_1010_454a(ulong param_1)

{
  int iVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  iVar2 = *(int *)(iVar1 + 0x24) * 0x4;
  return CONCAT22(*(undefined2 *)(iVar1 + iVar2 + 0x28),*(undefined2 *)(iVar1 + iVar2 + 0x26));
}



void __stdcall16far pass1_1010_4566(int param_1,ushort param_2,int param_3,ushort param_4)

{
  if (param_3 != 0x2) {
    return;
  }
  pass1_1010_4956(CONCAT22(param_2,param_1 + -0x20));
  pass1_1010_1f62(param_4,CONCAT22(param_2,param_1 + -0x20),0x2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_459e(long param_1)

{
  uint uVar1;
  uchar *puVar2;
  
  if (param_1 == 0x0) {
    uVar1 = 0x0;
    puVar2 = (uchar *)0x0;
  }
  else {
    uVar1 = (int)param_1 + 0x20;
    puVar2 = param_1._2_2_;
  }
  pass1_1008_9262((int)_PTR_LOOP_1050_0388,(ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10),0x1f4,CONCAT22(puVar2,uVar1),
                  uVar1,puVar2);
  *(undefined2 *)((int)param_1 + 0x7e) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_45d6(long param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  int iStack4;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(int *)(iVar6 + 0x7e) != 0x0) {
    if (_PTR_LOOP_1050_0388 != 0x0) {
      if (param_1 == 0x0) {
        iVar4 = 0x0;
        uVar5 = 0x0;
      }
      else {
        iVar4 = iVar6 + 0x20;
        uVar5 = uVar7;
      }
      param_2 = 0x1008;
      pass1_1008_92b2(_PTR_LOOP_1050_0388,0x1f4,CONCAT22(uVar5,iVar4));
    }
    for (iStack4 = 0x0; iStack4 < 0x10; iStack4 = iStack4 + 0x1) {
      if (*(int *)(iVar6 + 0x24) != iStack4) {
        puVar1 = (undefined4 *)*(uint *)(iVar6 + 0x26 + iStack4 * 0x4);
        uVar2 = *(uint *)(iVar6 + 0x26 + iStack4 * 0x4 + 0x2);
        if ((uVar2 | (uint)puVar1) != 0x0) {
          ppcVar3 = (code **)*puVar1;
          (**ppcVar3)(param_2,puVar1,uVar2,0x1);
        }
        *(undefined4 *)(iVar6 + iStack4 * 0x4 + 0x26) = 0x0;
      }
    }
    *(undefined2 *)(iVar6 + 0x7e) = 0x0;
  }
  return;
}



void __stdcall16far pass1_1010_4674(ulong param_1,int param_2,ushort param_3,UINT16 param_4)

{
  int *piVar1;
  UINT32 UVar2;
  UINT16 UVar3;
  
  UVar2 = (UINT32)param_1;
  UVar3 = (UINT16)(param_1 >> 0x10);
  if (param_2 == 0x1) {
    piVar1 = (int *)(UVar2 + 0x24);
    *piVar1 = *piVar1 + 0x1;
    if (0xf < *(int *)(UVar2 + 0x24)) {
      *(undefined2 *)(UVar2 + 0x24) = 0x0;
    }
LAB_1010_469a:
    draw_op_1010_47d0(UVar2,UVar3,*(UINT16 *)(UVar2 + 0x24),param_3,param_4);
  }
  else {
    if (param_2 != 0x2) {
      if (param_2 != 0x3) {
        if ((*(int *)(UVar2 + 0x6a) != 0x0) && (*(int *)(UVar2 + 0x6a) != 0x4)) {
          pass1_1010_459e(param_1);
        }
        goto LAB_1010_46e8;
      }
      piVar1 = (int *)(UVar2 + 0x24);
      *piVar1 = *piVar1 + -0x1;
      if (*piVar1 < 0x0) {
        *(undefined2 *)(UVar2 + 0x24) = 0xf;
      }
      goto LAB_1010_469a;
    }
  }
  pass1_1010_1f62(param_4,param_1,0x2);
  pass1_1010_45d6(param_1,param_3);
LAB_1010_46e8:
  *(int *)(UVar2 + 0x6a) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far get_sys_metrics_1010_46f6(ulong param_1)

{
  undefined2 uVar1;
  INT16 IVar2;
  INT16 IVar3;
  uchar *in_DX;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort unaff_SS;
  ushort *puVar6;
  ulong uVar7;
  ushort *puVar8;
  ushort *puVar9;
  int local_6;
  int local_4;
  
  puVar9 = (ushort *)CONCAT22(unaff_SS,&local_4);
  puVar8 = (ushort *)CONCAT22(unaff_SS,&local_6);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)),puVar8,puVar9);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar7 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x66));
  uVar1 = (undefined2)(uVar7 >> 0x10);
  *(int *)(iVar4 + 0x18) = local_4 + 0x8;
  *(int *)(iVar4 + 0x1a) = local_6 + 0x9;
  IVar2 = GetSystemMetrics16(0x1008);
  *(int *)(iVar4 + 0x1c) = IVar2 * 0x2 + *(int *)((int)uVar7 + 0x4);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  *(int *)(iVar4 + 0x1e) = IVar3 + IVar2 + *(int *)((int)uVar7 + 0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_4788(ulong param_1,char *param_2,ushort param_3,ushort param_4)

{
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                  *(ulong *)((int)param_1 + 0x6c));
  pass1_1030_301a(CONCAT22(param_4,param_3),param_2,param_4);
  return;
}



void __stdcall16far draw_1010_47ae(ulong param_1,ushort param_2,UINT16 param_3)

{
  UINT16 UStack4;
  
  UStack4 = 0x0;
  do {
    draw_op_1010_47d0((UINT32)param_1,(UINT16)(param_1 >> 0x10),UStack4,param_2,param_3);
    UStack4 = UStack4 + 0x1;
  } while ((int)UStack4 < 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far draw_op_1010_47d0(UINT32 param_1,UINT16 param_2,UINT16 param_3,INT16 in_style_3,UINT16 param_5)

{
  int *piVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  int iVar4;
  HPALETTE16 b_force_background;
  HGDIOBJ16 handle;
  HGDIOBJ16 handle_00;
  uint uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  LPCSTR output;
  astruct_5 *iVar6;
  int iVar7;
  astruct_4 *iVar9;
  undefined2 uVar8;
  HDC16 hdc;
  ulong uVar9;
  DEVMODEA *init_data;
  undefined4 uVar10;
  int iStack32;
  HDC16 local_14;
  LPCSTR pCStack18;
  LPCSTR pCStack16;
  undefined2 local_e;
  undefined2 uStack12;
  undefined2 uStack10;
  undefined2 uStack8;
  HGDIOBJ16 stock_obj_handle;
  HPEN16 pen_handle;
  
  uVar10 = 0x1;
  pen_handle = CreatePen16(in_style_3,-0x2805,0x77);
  uVar8 = 0x5;
  stock_obj_handle = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  local_e = 0x0;
  uStack12 = 0x0;
  uStack10 = 0x1;
  uStack8 = 0x1;
  puVar2 = (undefined4 *)*(uint *)(param_1 + 0x26 + param_3 * 0x4);
  puVar6 = *(uchar **)(param_1 + 0x26 + param_3 * 0x4 + 0x2);
  if (((uint)puVar6 | (uint)puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)((int)s_tile2_bmp_1050_1538,puVar2,puVar6,0x1,uVar8,uVar10);
    puVar6 = extraout_DX;
  }
  iVar4 = param_3 + 0x105;
  pass1_1010_8170(_PTR_LOOP_1050_14cc,iVar4,puVar6,(ushort)s_tile2_bmp_1050_1538);
  iVar7 = param_3 * 0x4;
  *(int *)(param_1 + iVar7 + 0x26) = iVar4;
  *(uchar **)(param_1 + iVar7 + 0x28) = puVar6;
  init_data = (DEVMODEA *)0x0;
  uVar9 = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x26 + iVar7));
  output = (LPCSTR)(uVar9 >> 0x10);
  pCStack18 = (LPCSTR)uVar9;
  pCStack16 = output;
  local_14 = CreateDC16((LPCSTR)0x1008,pCStack18,output,init_data);
  b_force_background =
       palette_op_1008_4e08(*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe),&local_14,(ushort)output,0x1008);
  handle = SelectObject16(0x1008,pen_handle);
  hdc = (HDC16)s_tile2_bmp_1050_1538;
  handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,stock_obj_handle);
  iStack32 = 0x0;
  while( true ) {
    piVar1 = (int *)(param_1 + 0x74);
    if (*piVar1 == iStack32 || *piVar1 < iStack32) break;
    iVar4 = (iStack32 * 0x10 + param_3) * 0x8;
    hdc = 0x1000;
    uVar5 = pass1_1000_484c(CONCAT22(param_5,&local_e),
                            CONCAT22(*(undefined2 *)(param_1 + 0x72),iVar4 + *(int *)(param_1 + 0x70)),0x8);
    if (uVar5 != 0x0) {
      uVar10 = *(undefined4 *)(param_1 + 0x70);
      uVar8 = (undefined2)((ulong)uVar10 >> 0x10);
      iVar7 = (int)uVar10;
      iVar9 = (astruct_4 *)(iVar4 + iVar7);
      hdc = (HDC16)s_tile2_bmp_1050_1538;
      Rectangle16(0x1000,iVar9->field_0x6,iVar9->field_0x4,iVar9->field_0x2,*(INT16 *)(iVar7 + iVar4));
    }
    iStack32 = iStack32 + 0x1;
  }
  SelectPalette16(hdc,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
  DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  return;
}



void __stdcall16far pass1_1010_4956(ulong param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = *(int *)(iVar3 + 0x6a);
  if (iVar2 == 0x0) {
    piVar1 = (int *)(iVar3 + 0x24);
    *piVar1 = *piVar1 + 0x1;
    if (0xf < *(int *)(iVar3 + 0x24)) {
      *(undefined2 *)(iVar3 + 0x24) = 0x0;
      return;
    }
  }
  else {
    if (iVar2 != 0x4) {
      return;
    }
    piVar1 = (int *)(iVar3 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      *(undefined2 *)(iVar3 + 0x24) = 0xf;
    }
  }
  return;
}



astruct_18 * pass1_1010_4994(ushort param_1,astruct_18 *param_2,byte param_3,ushort param_4)

{
  param_2 = (astruct_18 *)((ulong)param_2 & 0xffff0000 | (ulong)((int)param_2 - 0x20));
  pass1_1010_3f00((ushort *)param_2,param_4);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2,0x1000);
  }
  return param_2;
}



ulong __stdcall16far pass1_1010_49a0(int param_1,ushort param_2)

{
  return CONCAT22(param_2,param_1 + 0xa);
}



ulong __stdcall16far pass1_1010_49b0(int param_1,ushort param_2)

{
  return CONCAT22(param_2,param_1 + 0x18);
}



ushort __stdcall16far pass1_1010_49c0(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0x14);
}



void __stdcall16far pass1_1010_49ce(ulong param_1,ushort param_2)

{
  *(ushort *)((int)param_1 + 0x14) = param_2;
  return;
}



ushort __stdcall16far pass1_1010_49e0(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0x16);
}



void __stdcall16far pass1_1010_49ee(ulong param_1,ushort param_2)

{
  *(ushort *)((int)param_1 + 0x16) = param_2;
  return;
}



void __stdcall16far pass1_1010_4a00(ulong param_1,ushort param_2)

{
  *(ushort *)((int)param_1 + 0x12) = param_2;
  return;
}



ushort __stdcall16far pass1_1010_4a12(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0x12);
}



ushort * __stdcall16far pass1_1010_4a20(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_3f00(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_4a8a(astruct_637 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uchar *puVar1;
  int unaff_DI;
  astruct_43 *paVar2;
  ushort *puVar3;
  
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x16 = (astruct_76 *)0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  param_1->field_0x20 = 0x1;
  param_1->field_0x22 = 0x0;
  param_1->field_0x24 = 0x0;
  *(undefined4 *)&param_1->field_0x26 = 0x0;
  param_1->field_0x2a = 0x0;
  param_1->field_0x2c = 0x1;
  param_1->field_0x2e = 0x0;
  param_1->field_0x30 = 0x0;
  param_1->field_0x32 = 0x0;
  *(int *)CONCAT22(param_2,param_1) = (int)s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
  param_1->field_0x2 = 0x1010;
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1b3,param_4);
  puVar1 = (uchar *)((ulong)paVar2 >> 0x10);
  *(int *)&param_1->field_0x16 = (int)paVar2;
  *(uchar **)((int)&param_1->field_0x16 + 0x2) = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,puVar1,unaff_DI);
  param_1->field_0x26 = (int)puVar3;
  param_1->field_0x28 = (int)((ulong)puVar3 >> 0x10);
  pass1_1008_4772(param_1->field_0x16);
  param_1->field_0xe = 0x13c;
  param_1->field_0xa = 0x0;
  param_1->field_0x10 = 0x0;
  param_1->field_0xc = 0x0;
  return;
}



void __stdcall16far free_rsrc_1010_4b3e(ushort *param_1,HGLOBAL16 param_2)

{
  int *piVar1;
  undefined4 *puVar2;
  uint uVar3;
  code **ppcVar4;
  undefined4 *puVar5;
  undefined4 uVar6;
  BOOL16 BVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  HGLOBAL16 HVar12;
  ushort unaff_SS;
  int iStack4;
  
  uVar10 = (undefined2)((ulong)param_1 >> 0x10);
  iVar8 = (int)param_1;
  *param_1 = (int)s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
  *(undefined2 *)(iVar8 + 0x2) = 0x1010;
  HVar12 = param_2;
  if (*(int *)(iVar8 + 0x2a) != 0x0) {
    HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
    BVar7 = GlobalUnlock16(param_2);
    if (BVar7 == 0x0) {
      HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
      FreeResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
  }
  *(undefined2 *)(iVar8 + 0x2a) = 0x0;
  if (**(long **)(iVar8 + 0x12) != 0x0) {
    iStack4 = 0x0;
    while( true ) {
      puVar5 = (undefined4 *)*(undefined4 *)(iVar8 + 0x12);
      piVar1 = (int *)((int)puVar5 + 0x8);
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      uVar11 = (undefined2)((ulong)*puVar5 >> 0x10);
      iVar9 = (int)*puVar5;
      puVar2 = (undefined4 *)*(uint *)(iVar9 + iStack4 * 0x4);
      uVar3 = *(uint *)(iVar9 + iStack4 * 0x4 + 0x2);
      if ((uVar3 | (uint)puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)(HVar12,puVar2,uVar3,0x1);
      }
      iStack4 = iStack4 + 0x1;
    }
  }
  uVar6 = *(undefined4 *)(iVar8 + 0x12);
  fn_ptr_1000_17ce(*(astruct_18 **)((int)uVar6 + 0x4),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x12),0x1000);
  puVar2 = (undefined4 *)*(uint *)(iVar8 + 0x16);
  uVar3 = *(uint *)(iVar8 + 0x18);
  if ((uVar3 | (uint)puVar2) != 0x0) {
    ppcVar4 = (code **)*puVar2;
    (**ppcVar4)(0x1000,puVar2,uVar3,0x1);
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x1a),0x1000);
  pass1_1010_1d80(param_1,unaff_SS);
  return;
}



ulong __stdcall16far pass1_1010_4c2c(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x18),*(undefined2 *)((int)param_1 + 0x16));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_4c3e(ulong param_1,int param_2,int param_3,uchar *param_4,ushort param_5)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  astruct_43 *paVar7;
  ulong uVar8;
  int iStack14;
  undefined local_c [0x6];
  undefined2 uStack6;
  int iStack4;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  pass1_1010_bffa(*(ulong *)(iVar3 + 0x26),param_3,param_4,param_5);
  *(int *)(iVar3 + 0x12) = param_3;
  *(uchar **)(iVar3 + 0x14) = param_4;
  if (((uint)param_4 | *(uint *)(iVar3 + 0x12)) != 0x0) {
    if (param_2 == 0x0) {
      uVar2 = *(undefined4 *)(iVar3 + 0x12);
      *(undefined2 *)(iVar3 + 0x30) = *(undefined2 *)((int)uVar2 + 0x8);
    }
    else {
      *(undefined2 *)(iVar3 + 0x2e) = 0x1;
      uVar2 = *(undefined4 *)(iVar3 + 0x12);
      uVar2 = *(undefined4 *)((int)uVar2 + 0x4);
      iVar4 = *(int *)((int)uVar2 + 0x2);
      if ((iVar4 == 0x5) || (iVar4 == 0x6)) {
        *(undefined2 *)(iVar3 + 0x30) = 0x1;
        *(undefined2 *)(iVar3 + 0x20) = 0x0;
      }
      else {
        *(undefined2 *)(iVar3 + 0x30) = 0x2;
        uVar2 = *(undefined4 *)*(undefined4 *)(iVar3 + 0x12);
        *(undefined4 *)(iVar3 + 0x32) = *(undefined4 *)((int)uVar2 + 0x4);
        paVar7 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1bf,param_5);
        uVar2 = *(undefined4 *)*(undefined4 *)(iVar3 + 0x12);
        uVar6 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar4 = (int)uVar2;
        *(undefined2 *)(iVar4 + 0x4) = (int)paVar7;
        *(undefined2 *)(iVar4 + 0x6) = (int)((ulong)paVar7 >> 0x10);
      }
    }
    iStack4 = 0x14;
    pass1_1008_3e38((ushort *)CONCAT22(param_5,local_c));
    uStack6 = 0x0;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = (int *)(iVar3 + 0x30);
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar2 = *(undefined4 *)*(undefined4 *)(iVar3 + 0x12);
      uVar8 = pass1_1008_4772(*(astruct_76 **)((int)uVar2 + iStack14 * 0x4));
      iStack4 = iStack4 + (-(uint)(iStack14 == 0x0) & 0x5) + 0x14 + *(int *)((int)uVar8 + 0x4);
      iStack14 = iStack14 + 0x1;
    }
    if (*(int *)(iVar3 + 0xe) < iStack4) {
      *(int *)(iVar3 + 0xe) = iStack4;
    }
  }
  return;
}



// WARNING: This is an inlined function

void __stdcall16far
struct_1010_4d5c(ulong param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,int param_6,uchar *param_7)

{
  undefined4 uVar1;
  ushort uVar2;
  astruct_245 *iVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_245 *)param_1;
  if (*(long *)&iVar3->field_0x1a == 0x0) {
    uVar2 = iVar3->field_0x30 << 0x3;
    mem_op_1000_179c(uVar2,param_7,0x1000);
    *(ushort *)&iVar3->field_0x1a = uVar2;
    iVar3->field_0x1c = param_7;
  }
  uVar1 = *(undefined4 *)&iVar3->field_0x1a;
  iVar4 = param_6 * 0x8;
  *(ushort *)((int)uVar1 + iVar4) = param_5;
  uVar1 = *(undefined4 *)&iVar3->field_0x1a;
  *(ushort *)((int)uVar1 + iVar4 + 0x2) = param_4;
  uVar1 = *(undefined4 *)&iVar3->field_0x1a;
  *(ushort *)((int)uVar1 + iVar4 + 0x4) = param_3;
  uVar1 = *(undefined4 *)&iVar3->field_0x1a;
  *(ushort *)((int)uVar1 + iVar4 + 0x6) = param_2;
  return;
}



ulong __stdcall16far pass1_1010_4dc8(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x20) == 0x0) {
    return 0x0;
  }
  return CONCAT22(*(undefined2 *)(iVar1 + 0x1c),*(int *)(iVar1 + 0x20) * 0x8 + *(int *)(iVar1 + 0x1a));
}



void __stdcall16far pass1_1010_4df0(ulong param_1,uint param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x26);
  pass1_1010_c1ba((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(int *)((int)param_1 + 0x20),param_2,param_3);
  return;
}



void __stdcall16far pt_in_rect_1010_4e08(ulong param_1,ushort param_2,ushort param_3,RECT16 *param_4)

{
  int *piVar1;
  bool bVar2;
  BOOL16 BVar3;
  int iVar4;
  undefined2 uVar5;
  int iStack12;
  int iStack10;
  POINT16 PStack8;
  
  PStack8 = (POINT16)CONCAT22(param_2,param_3);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  *(undefined2 *)(iVar4 + 0x22) = *(undefined2 *)(iVar4 + 0x20);
  bVar2 = false;
  *(undefined2 *)(iVar4 + 0x24) = 0x0;
  iStack12 = 0x0;
  iStack10 = 0x0;
  do {
    piVar1 = (int *)(iVar4 + 0x30);
    if (*piVar1 == iStack12 || *piVar1 < iStack12) {
LAB_1010_4e67:
      if (iStack10 != 0x0) {
        *(int *)(iVar4 + 0x20) = iStack10;
      }
      if (bVar2) {
        return;
      }
      return;
    }
    BVar3 = PtInRect16(param_4,PStack8);
    if (BVar3 != 0x0) {
      iStack10 = iStack12;
      bVar2 = true;
      goto LAB_1010_4e67;
    }
    iStack12 = iStack12 + 0x1;
    param_4 = (RECT16 *)s_tile2_bmp_1050_1538;
  } while( true );
}



void __stdcall16far pass1_1010_4e8c(ulong param_1,ushort param_2)

{
  pass1_1010_1f62(param_2,param_1,0xd);
  return;
}



void __stdcall16far find_n_load_rsrc_1010_4e9e(ulong param_1,HGLOBAL16 param_2)

{
  BOOL16 BVar1;
  HRSRC16 h_rsrc;
  int iVar2;
  ushort uVar3;
  HGLOBAL16 HVar3;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0x20) != 0x0) {
    HVar3 = param_2;
    if (*(int *)(iVar2 + 0x2a) != 0x0) {
      HVar3 = (HGLOBAL16)s_tile2_bmp_1050_1538;
      BVar1 = GlobalUnlock16(param_2);
      if (BVar1 == 0x0) {
        HVar3 = (HGLOBAL16)s_tile2_bmp_1050_1538;
        FreeResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
      }
    }
    h_rsrc = FindResource16(HVar3,(LPCSTR)&PTR_LOOP_1050_000a,(LPCSTR)0x0);
    HVar3 = LoadResource16((HMODULE16)s_tile2_bmp_1050_1538,h_rsrc);
    *(HGLOBAL16 *)(iVar2 + 0x2a) = HVar3;
    if (HVar3 != 0x0) {
      WIN16_LockResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
      return;
    }
  }
  return;
}



ushort __stdcall16far pass1_1010_4f20(ushort param_1,ushort param_2,int param_3)

{
  return *(ushort *)(param_3 * 0x2 + 0x139a);
}



void __stdcall16far pass1_1010_4f30(ushort param_1,ushort param_2,ushort *param_3,ushort *param_4)

{
  *param_4 = 0xa;
  *param_3 = 0x73;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_4f48(ulong param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 *puVar4;
  undefined4 uVar5;
  astruct_482 *iVar6;
  astruct_483 *iVar7;
  undefined2 uVar6;
  undefined2 uVar7;
  astruct_43 *paVar8;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_482 *)param_1;
  puVar4 = iVar6->field_0x12;
  iVar6->field_0x30 = *(undefined2 *)((int)puVar4 + 0x8);
  if (iVar6->field_0x32 != 0x0) {
    uVar5 = *iVar6->field_0x12;
    uVar7 = (undefined2)((ulong)uVar5 >> 0x10);
    iVar7 = (astruct_483 *)uVar5;
    puVar4 = iVar7->field_0x4;
    iVar7->field_0x4 = (undefined4 *)iVar6->field_0x32;
    if (puVar4 != (undefined4 *)0x0) {
      ppcVar3 = (code **)*puVar4;
      (**ppcVar3)();
    }
    iVar6->field_0x32 = 0x0;
  }
  puVar1 = iVar6->field_0x16;
  uVar2 = iVar6->field_0x18;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1b3,param_2);
  iVar6->field_0x16 = (undefined4 *)paVar8;
  iVar6->field_0x18 = (uint)((ulong)paVar8 >> 0x10);
  fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x1a,0x1000);
  iVar6->field_0x1a = 0x0;
  iVar6->field_0x2e = 0x0;
  return;
}



ushort * __stdcall16far pass1_1010_5004(ushort *param_1,byte param_2,ushort param_3)

{
  free_rsrc_1010_4b3e(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_503e(int param_1,ushort param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  struct_op_1018_4cda(param_1,param_2,param_3);
  *(int *)CONCAT22(param_2,param_1) = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
  *(undefined2 *)(param_1 + 0x2) = 0x1010;
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x1b3,param_4,param_5);
  _PTR_LOOP_1050_4230 = CONCAT22(param_2,param_1);
  return;
}



astruct_11 * __stdcall16far pass1_1010_5074(astruct_11 *param_1,byte param_2)

{
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1010_50b2(astruct_646 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xc = 0x0;
  param_1->field_0x10 = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x53f4;
  param_1->field_0x2 = 0x1010;
  return;
}



void __stdcall16far pass1_1010_50f2(ushort *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x53f4;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0xc),0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_5120(ulong param_1,uint param_2,uint param_3,uint param_4,ushort param_5)

{
  undefined4 uVar1;
  ulong uVar2;
  uint uVar3;
  uint uVar4;
  ulong uVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar9 = (int)param_1;
  if (*(long *)(iVar9 + 0x16) != 0x0) {
    uVar1 = *(undefined4 *)(iVar9 + 0x16);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    uVar6 = param_4 | param_3;
    if (uVar6 != 0x0) {
      uVar2 = *(ulong *)(param_3 + 0x1f6);
      uVar5 = uVar2;
      pass1_1030_38f2(uVar2,0x3,param_5);
      uVar3 = (uint)uVar5;
      uVar7 = uVar6;
      uVar4 = uVar3;
      pass1_1030_38f2(uVar2,0x4,param_5);
      iVar8 = uVar7 + uVar6 + (uint)CARRY2(uVar4,uVar3);
      if ((0x0 < iVar8) || ((-0x1 < iVar8 && (param_2 <= uVar4 + uVar3)))) {
        *(uint *)(iVar9 + 0xa) = param_2;
        return;
      }
    }
  }
  return;
}



void __stdcall16far pass1_1010_519a(ulong param_1,int *param_2,uchar *param_3,ushort param_4)

{
  undefined4 uVar1;
  ushort uVar2;
  undefined *puVar3;
  uchar *puVar4;
  astruct_246 *iVar5;
  astruct_247 *iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  int *piStack44;
  undefined local_18 [0xc];
  int iStack12;
  undefined4 uStack6;
  
  uStack6 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_18),0x1,0x0,0x400);
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_246 *)param_1;
  iVar5->field_0x10 = iStack12;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0xc,0x1000);
  uVar2 = iVar5->field_0x10 << 0x2;
  mem_op_1000_179c(uVar2,param_3,0x1000);
  iVar5->field_0xc = uVar2;
  iVar5->field_0xe = param_3;
  iVar5->field_0x10 = 0x0;
  while( true ) {
    puVar4 = param_3;
    puVar3 = local_18;
    pass1_1028_e4ec(CONCAT22(param_4,puVar3));
    uStack6 = CONCAT22(puVar4,puVar3);
    if ((uchar *)((uint)puVar4 | (uint)puVar3) == (uchar *)0x0) break;
    param_3 = (uchar *)((uint)puVar4 | (uint)puVar3);
    if (*(long *)(puVar3 + 0x200) != 0x8000002) {
      param_3 = *(uchar **)(puVar3 + 0x6);
      uVar1 = *(undefined4 *)&iVar5->field_0xc;
      uVar9 = (undefined2)((ulong)uVar1 >> 0x10);
      iVar7 = (int)uVar1;
      iVar6 = (astruct_247 *)(iVar5->field_0x10 * 0x4);
      piStack44 = (int *)(param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x10));
      *(undefined2 *)(iVar6 + iVar7) = *(undefined2 *)(puVar3 + 0x4);
      *(uchar **)(iVar6 + iVar7 + 0x2) = param_3;
      *piStack44 = *piStack44 + 0x1;
    }
  }
  *param_2 = iVar5->field_0x10;
  return;
}

