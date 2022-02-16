
ushort * __stdcall16far pass1_1018_4a64(ushort *param_1,byte param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_4aaa(int param_1,ushort param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  struct_op_1018_4cda(param_1,param_2,param_3);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x4b06;
  *(undefined2 *)(param_1 + 0x2) = 0x1018;
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x9a,param_4,param_5);
  _PTR_LOOP_1050_4230 = CONCAT22(param_2,param_1);
  return;
}



astruct_11 * __stdcall16far pass1_1018_4ae0(astruct_11 *param_1,byte param_2,ushort param_3)

{
  clenaup_win_ui_1018_4d22(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far get_sys_metrics_1018_4b1e(astruct_55 *param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_op_1010_1d48((astruct_79 *)param_1,param_3);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ushort *)(iVar1 + 0x12) = param_2;
  *(undefined2 *)(iVar1 + 0x14) = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_4c9e;
  *(undefined2 *)(iVar1 + 0x2) = 0x1018;
  if (PTR_LOOP_1050_416c == (undefined *)0x0) {
    PTR_LOOP_1050_416c = (undefined *)GetSystemMetrics16(0x1010);
    PTR_LOOP_1050_416e = (undefined *)GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    PTR_LOOP_1050_4170 = (undefined *)GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  }
  return &param_1->field_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_4b78(ulong *param_1,ushort param_2)

{
  code **ppcVar1;
  uchar *puVar2;
  uint uVar3;
  ushort *puVar4;
  undefined4 *puVar5;
  
  puVar2 = param_1._2_2_;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | ZEXT24((undefined4 *)((int)param_1 + 0xa))),
                  (WNDCLASS16 *)0x0,0x8);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x18)),(WNDCLASS16 *)0x0,0x8);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar2,(int)param_1._2_2_);
  puVar5 = (undefined4 *)pass1_1010_5f7a((int)puVar4,(ushort)((ulong)puVar4 >> 0x10),0x0,*(int *)((int)param_1 + 0x12));
  uVar3 = (uint)((ulong)puVar5 >> 0x10);
  if ((uVar3 | (uint)puVar5) != 0x0) {
    *(undefined4 *)((int)param_1 + 0xa) = *puVar5;
    *(undefined4 *)((int)param_1 + 0xe) = *(undefined4 *)((uint)puVar5 + 0x4);
  }
  ppcVar1 = (code **)((int)*param_1 + 0x20);
  (**ppcVar1)(0x1010,param_1);
  if ((*(int *)((int)param_1 + 0xe) == 0x0) && (*(int *)((int)param_1 + 0x10) == 0x0)) {
    *(undefined2 *)((int)param_1 + 0xa) = *(undefined2 *)((int)param_1 + 0x18);
    *(undefined2 *)((int)param_1 + 0xc) = *(undefined2 *)((int)param_1 + 0x1a);
  }
  *(undefined2 *)((int)param_1 + 0xe) = *(undefined2 *)((int)param_1 + 0x1c);
  *(undefined2 *)((int)param_1 + 0x10) = *(undefined2 *)((int)param_1 + 0x1e);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_4c2c(ulong param_1,ulong *param_2,ushort param_3,ushort param_4)

{
  ushort *puVar1;
  
  *(ulong *)((int)param_1 + 0xa) = *param_2;
  *(ulong *)((int)param_1 + 0xe) = param_2[0x1];
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_1._2_2_,(int)param_1._2_2_);
  pass1_1010_5fb0((ulong)puVar1,0x0,(ulong *)((int)param_1 + 0xa),(ushort)param_1._2_2_,*(int *)((int)param_1 + 0x12));
  return;
}



ushort * __stdcall16far pass1_1018_4c78(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_op_1018_4cda(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)(param_1 + 0xa) = 0x0;
  *(undefined4 *)(param_1 + 0xe) = 0x0;
  *(undefined2 *)(param_1 + 0x12) = 0x0;
  *(undefined2 *)(param_1 + 0x14) = 0x0;
  *(undefined2 *)(param_1 + 0x16) = 0x0;
  *(undefined2 *)(param_1 + 0x18) = 0x1;
  *(undefined2 *)(param_1 + 0x1a) = 0x0;
  *(int *)CONCAT22(param_2,param_1) = (int)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
  *(undefined2 *)(param_1 + 0x2) = 0x1018;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far clenaup_win_ui_1018_4d22(astruct_11 *in_struct_1,HDC16 in_hdc_2)

{
  uint uVar1;
  code **ppcVar2;
  astruct_11 *local_struct_1;
  astruct_11 *uVar4;
  ushort unaff_SS;
  ULONG *puVar2;
  ULONG *puVar1;
  
  uVar4 = (astruct_11 *)((ulong)in_struct_1 >> 0x10);
  local_struct_1 = (astruct_11 *)in_struct_1;
  *(int *)in_struct_1 = (int)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
  local_struct_1->field_0x2 = 0x1018;
  if (local_struct_1->field_0x12 != 0x0) {
    SelectPalette16(in_hdc_2,0x0,local_struct_1->field_0x1a);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    in_hdc_2 = (HDC16)s_tile2_bmp_1050_1538;
    DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
  }
  puVar1 = local_struct_1->field_0xa;
  uVar1 = local_struct_1->field_0xc;
  if ((uVar1 | (uint)puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)(in_hdc_2,puVar1,uVar1,0x1);
  }
  puVar2 = local_struct_1->field_0xe;
  uVar1 = local_struct_1->field_0x10;
  if ((uVar1 | (uint)puVar2) != 0x0) {
    ppcVar2 = (code **)*puVar2;
    (**ppcVar2)(in_hdc_2,puVar2,uVar1,0x1);
  }
  _PTR_LOOP_1050_4230 = 0x0;
  pass1_1010_1d80((ushort *)in_struct_1,unaff_SS);
  return;
}



void __stdcall16far get_dc_1018_4db0(ULONG param_1,ushort param_2,HWND16 param_3)

{
  HDC16 HVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x16) = param_2;
  HVar1 = GetDC16(param_3);
  *(HDC16 *)((int)param_1 + 0x14) = HVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_4dce(ulong *param_1,ushort param_2,uchar *param_3,ushort param_4)

{
  code **ppcVar1;
  undefined2 uVar2;
  int unaff_DI;
  ushort *puVar3;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,param_3,unaff_DI);
  uVar2 = (undefined2)((ulong)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)*param_1 + 0x10);
  (**ppcVar1)(0x1010,param_1,param_2,*(undefined2 *)((int)puVar3 + 0xc),*(undefined2 *)((int)puVar3 + 0xa));
  return;
}



void __stdcall16far
create_dc_1018_4e04(astruct_8 **param_1,UINT16 param_2,int param_3,int param_4,LPCSTR in_string_5,undefined2 in_string_6
                   )

{
  code **ppcVar1;
  astruct_8 *paVar2;
  astruct_9 *iVar4;
  undefined2 uVar3;
  ulong uVar4;
  int iStack16;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_9 *)param_1;
  ppcVar1 = (code **)((int)*param_1 + 0x14);
  (**ppcVar1)();
  uVar4 = pass1_1008_4772((astruct_76 *)iVar4->field_0xa);
  pass1_1008_43cc((ulong)iVar4->field_0xa);
  paVar2 = (astruct_8 *)CreateDC16((LPCSTR)0x1008,(LPCSTR)uVar4,(LPCSTR)(uVar4 >> 0x10),(DEVMODEA *)0x0);
  iVar4->field_0x12 = paVar2;
  paVar2 = (astruct_8 *)&iVar4->field_0x12;
  ppcVar1 = (code **)((int)*iVar4->field_0xa + 0x8);
  (**ppcVar1)();
  iVar4->field_0x1a = paVar2;
  if ((DAT_1050_422e != 0x0) && (0x280 < param_4)) {
    for (iStack16 = 0x0; iStack16 < DAT_1050_4216 + 0x1; iStack16 = iStack16 + 0x1) {
      *(undefined2 *)((int)&PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) =
           (int)(((long)*(int *)((int)&PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) * ((long)param_4 + 0x1)) / 0x280);
    }
    for (iStack16 = 0x0; iStack16 < DAT_1050_422c + 0x1; iStack16 = iStack16 + 0x1) {
      *(undefined2 *)((int)&DAT_1050_419a + iStack16 * 0x2) =
           (int)(((long)*(int *)((int)&DAT_1050_419a + iStack16 * 0x2) * ((long)param_3 + 0x1)) / 0x1e0);
    }
  }
  DAT_1050_422e = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1018_4f18(astruct_39 *param_1,UINT16 param_2,ulong param_3)

{
  code **ppcVar1;
  undefined4 *puVar2;
  RECT16 *rect;
  int iVar3;
  ulong uVar4;
  uchar *extraout_DX;
  uchar *puVar5;
  uchar *extraout_DX_00;
  uchar *puVar6;
  uint uVar7;
  astruct_39 *iVar6;
  undefined2 uVar8;
  undefined2 uVar9;
  ushort unaff_SS;
  astruct_76 *paStack22;
  RECT16 local_12;
  int iStack14;
  int iStack12;
  ulong uStack10;
  astruct_43 *paStack6;
  
  paStack6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_2,unaff_SS);
  uVar4 = (ulong)paStack6 & 0xffff;
  ppcVar1 = (code **)((int)*(undefined4 *)paStack6 + 0x14);
  (**ppcVar1)(0x1010,(int)uVar4,(int)((ulong)paStack6 >> 0x10));
  puVar2 = (undefined4 *)uVar4;
  uStack10 = uVar4 & 0xffff | ZEXT24(extraout_DX) << 0x10;
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_39 *)param_1;
  puVar5 = extraout_DX;
  if (*(long *)&iVar6->field_0xe != 0x0) {
    uVar7 = iVar6->field_0x10;
    puVar2 = (undefined4 *)*(undefined4 *)&iVar6->field_0xe;
    puVar5 = (uchar *)(uVar7 | (uint)puVar2);
    if (puVar5 != (uchar *)0x0) {
      ppcVar1 = (code **)*puVar2;
      (**ppcVar1)();
      puVar5 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar5,0x1000);
  puVar6 = (uchar *)((uint)puVar5 | (uint)puVar2);
  if (puVar6 == (uchar *)0x0) {
    puVar2 = (undefined4 *)0x0;
    puVar6 = (uchar *)0x0;
  }
  else {
    struct_1008_4c58((ushort *)CONCAT22(puVar5,puVar2));
  }
  iVar6->field_0xe = puVar2;
  iVar6->field_0x10 = (uint)puVar6;
  pass1_1008_4d84(*(astruct_90 **)&iVar6->field_0xe,uStack10,puVar6);
  rect = &local_12;
  GetClientRect16(0x1008,rect);
  uVar9 = 0x1000;
  mem_op_1000_179c(0x1e,puVar6,0x1000);
  paStack22 = (astruct_76 *)CONCAT22(puVar6,rect);
  uVar7 = (uint)puVar6 | (uint)rect;
  if (uVar7 == 0x0) {
    *(undefined4 *)&iVar6->field_0xa = 0x0;
  }
  else {
    iVar3 = (iStack12 - local_12.y) + 0x1;
    uVar9 = 0x1008;
    pass1_1008_405c(paStack22,*(ulong *)&iVar6->field_0xe,iVar3,(iStack14 - local_12.x) + 0x1);
    iVar6->field_0xa = iVar3;
    iVar6->field_0xc = uVar7;
  }
  if (paStack6 != (astruct_43 *)0x0) {
    ppcVar1 = (code **)*(undefined4 *)paStack6;
    (**ppcVar1)(uVar9,(int)paStack6,(int)((ulong)paStack6 >> 0x10),0x1);
  }
  return;
}



astruct_11 * __stdcall16far pass1_1018_5032(astruct_11 *param_1,byte param_2,ushort param_3)

{
  clenaup_win_ui_1018_4d22(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1018_5070(astruct_641 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x56d2;
  param_1->field_0x2 = 0x1018;
  return;
}



void __stdcall16far pass1_1018_50ac(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x56d2;
  *(undefined2 *)(iVar4 + 0x2) = 0x1018;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe);
  uVar2 = *(uint *)(iVar4 + 0x10);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_50ea(ulong param_1,ushort param_2,ulong param_3)

{
  int iVar1;
  code **ppcVar2;
  undefined4 uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  uint uVar9;
  undefined2 uVar10;
  astruct_99 *paStack6;
  
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
  uVar6 = (uint)((ulong)paStack6 >> 0x10);
  uVar4 = (uint)paStack6;
  if ((uchar *)(uVar6 | uVar4) == (uchar *)0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    *(undefined2 *)(uVar4 + 0x2) = 0x1008;
    *(undefined2 *)(uVar4 + 0x4) = 0x0;
    *(undefined2 *)(uVar4 + 0x6) = 0x0;
    *(undefined2 *)(uVar4 + 0x8) = 0x0;
    *(undefined2 *)(uVar4 + 0xa) = 0x0;
    *(undefined2 *)(uVar4 + 0xc) = 0x0;
    paStack6->field_0x0 = 0x56ce;
    *(undefined2 *)(uVar4 + 0x2) = 0x1018;
  }
  uVar9 = (uint)((ulong)paStack6 >> 0x10);
  uVar7 = (uint)paStack6;
  *(ushort *)(uVar7 + 0xa) = param_2;
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar3 = *(undefined4 *)(iVar8 + 0xa);
  iVar1 = *(int *)((int)uVar3 + 0xc);
  if (iVar1 == 0x1) {
    uVar3 = *(undefined4 *)(iVar8 + 0xa);
    uVar5 = *(uint *)((int)uVar3 + 0xe);
    *(uint *)(uVar7 + 0x4) = uVar5;
  }
  else {
    if (iVar1 == 0x5) {
      uVar3 = *(undefined4 *)(iVar8 + 0xa);
      uVar5 = *(uint *)((int)uVar3 + 0xe);
      *(uint *)(uVar7 + 0x6) = uVar5;
    }
    else {
      if (iVar1 != 0x6) {
        if ((uVar9 | uVar7) == 0x0) {
          return;
        }
        ppcVar2 = (code **)*(undefined4 *)paStack6;
        (**ppcVar2)();
        return;
      }
      uVar3 = *(undefined4 *)(iVar8 + 0xa);
      uVar5 = *(uint *)((int)uVar3 + 0xe);
      *(uint *)(uVar7 + 0x8) = uVar5;
    }
  }
  pass1_1030_6c66(param_3,0x1,(ulong)paStack6,uVar5,(uchar *)(uVar6 | uVar4),0x1030);
  return;
}



void __stdcall16far pass1_1018_51d2(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe);
  uVar2 = *(uint *)(iVar4 + 0x10);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar4 + 0xe) = 0x0;
  return;
}



ulong __stdcall16far pass1_1018_5206(ulong param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  undefined local_a [0x8];
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(undefined4 *)(iVar3 + 0xa) = 0x0;
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar3 + 0xe));
  do {
    uVar5 = pass1_1008_5b12(local_a,param_3);
    uVar2 = (uint)((ulong)uVar5 >> 0x10);
    *(undefined2 *)(iVar3 + 0xa) = (int)uVar5;
    *(uint *)(iVar3 + 0xc) = uVar2;
    if ((uVar2 | *(uint *)(iVar3 + 0xa)) == 0x0) break;
    uVar5 = *(undefined4 *)(iVar3 + 0xa);
    iVar1 = pass1_1000_3d7a(*(ulong *)((int)uVar5 + 0x4),param_2);
  } while (iVar1 != 0x0);
  return CONCAT22(*(undefined2 *)(iVar3 + 0xc),*(undefined2 *)(iVar3 + 0xa));
}



ulong __stdcall16far pass1_1018_526a(ulong param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(long *)(iVar1 + 0xe) == 0x0) {
    pass1_1018_5292(param_1 & 0xffff | (ulong)uVar2 << 0x10,param_2,param_3);
  }
  return CONCAT22(*(undefined2 *)(iVar1 + 0x10),*(undefined2 *)(iVar1 + 0xe));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_5292(ulong param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  undefined4 uVar2;
  code **ppcVar3;
  ushort uVar4;
  BOOL16 BVar5;
  undefined *puVar6;
  int iVar7;
  char *pcVar8;
  ushort uVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  ulong uVar12;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uint uVar13;
  uchar *extraout_DX_01;
  uchar *puVar14;
  uint extraout_DX_02;
  uint extraout_DX_03;
  uchar *puVar15;
  uchar *extraout_DX_04;
  undefined2 uVar16;
  uint extraout_DX_05;
  undefined2 extraout_DX_06;
  uchar *extraout_DX_07;
  uchar *extraout_DX_08;
  int iVar17;
  undefined2 uVar18;
  undefined2 uVar19;
  ushort *puVar20;
  uint uStack50;
  undefined local_26 [0x8];
  ulong uStack30;
  ulong uStack26;
  ulong uStack22;
  undefined4 *puStack18;
  uchar *puStack16;
  undefined4 *puStack14;
  uchar *puStack12;
  undefined2 uStack10;
  ulong uStack8;
  ushort uStack4;
  
  uVar18 = (undefined2)(param_1 >> 0x10);
  iVar17 = (int)param_1;
  puStack18 = (undefined4 *)*(uint *)(iVar17 + 0xe);
  uVar12 = ZEXT24(puStack18);
  puVar14 = *(uchar **)(iVar17 + 0x10);
  puStack16 = puVar14;
  puStack14 = puStack18;
  puStack12 = puVar14;
  if (((uint)puVar14 | (uint)puStack18) != 0x0) {
    ppcVar3 = (code **)*puStack18;
    (**ppcVar3)();
    puVar14 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar14,0x1000);
  puStack18 = (undefined4 *)(uint)uVar12;
  puStack16 = puVar14;
  if (((uint)puVar14 | (uint)puStack18) == 0x0) {
    uVar12 = 0x0;
    puVar14 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10));
    puVar14 = extraout_DX_00;
  }
  *(undefined2 *)(iVar17 + 0xe) = (int)uVar12;
  *(uchar **)(iVar17 + 0x10) = puVar14;
  for (uStack4 = 0x21; -0x1 < (int)uStack4; uStack4 = uStack4 - 0x1) {
    uStack22 = pass1_1030_7c28(param_2,uStack4,(uint)uVar12,(uint)puVar14,param_3);
    uVar12 = uStack22 & 0xffff;
    uVar13 = (uint)uVar12;
    puVar14 = (uchar *)((uint)(uStack22 >> 0x10) | uVar13);
    if (puVar14 != (uchar *)0x0) {
      string_1020_c0ca(uStack4);
      uVar4 = str_op_1008_60e8((char *)CONCAT22(puVar14,uVar13),(ushort)puVar14);
      uVar12 = (ulong)uVar4;
      uStack26 = CONCAT22(puVar14,uVar4);
      mem_op_1000_179c(0x10,puVar14,0x1000);
      puStack14 = (undefined4 *)uVar12;
      puStack12 = puVar14;
      if (((uint)puVar14 | (uint)puStack14) == 0x0) {
        uVar12 = 0x0;
        uVar13 = 0x0;
      }
      else {
        struct_1018_4790(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10,uStack22,uStack26,uStack4);
        uVar13 = extraout_DX_02;
      }
      uStack30 = uVar12 & 0xffff | (ulong)uVar13 << 0x10;
      uVar2 = *(undefined4 *)(iVar17 + 0xe);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
      (**ppcVar3)(0x1000,(int)uVar2,(char)((ulong)uVar2 >> 0x10),(int)uVar12,uVar13);
      puVar14 = extraout_DX_01;
    }
  }
  uStack8 = struct_op_1030_73a8(param_2);
  uStack10 = *(undefined2 *)((int)uStack8 + 0xc);
  BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack10,0x4);
  if (BVar5 != 0x0) {
    uStack30 = uStack8;
    uStack26 = *(ulong *)((int)uStack8 + 0x20);
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_26),uStack26);
    while( true ) {
      puVar6 = local_26;
      pass1_1008_5b12(puVar6,param_3);
      uStack22 = CONCAT22(extraout_DX_03,puVar6);
      puVar14 = (uchar *)(extraout_DX_03 | (uint)puVar6);
      if (puVar14 == (uchar *)0x0) break;
      iVar1 = *(int *)(puVar6 + 0x6);
      iVar7 = iVar1 + -0x7;
      if (iVar7 == 0x0) {
LAB_1018_53f0:
        pcVar8 = string_op_1020_c222(*(ushort *)(puVar6 + 0x6));
        uVar9 = str_op_1008_60e8((char *)CONCAT22(puVar14,pcVar8),(ushort)puVar14);
        puVar15 = puVar14;
        uVar4 = uVar9;
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack18 = (undefined4 *)uVar4;
        puStack16 = puVar15;
        if (((uint)puVar15 | uVar4) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar19 = (undefined2)(uStack22 >> 0x10);
          puVar20 = struct_1018_48b0((ushort *)CONCAT22(puVar15,uVar4),(ulong)*(uint *)((int)uStack22 + 0xa),
                                     CONCAT22(puVar14,uVar9),*(ushort *)((int)uStack22 + 0x6));
          uVar16 = (undefined2)((ulong)puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uVar2 = *(undefined4 *)(iVar17 + 0xe);
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar19,uVar16);
        puVar14 = extraout_DX_04;
      }
      else {
        if (((0x5 < iVar7) && (!SBORROW2(iVar7,0x6))) && (iVar1 + -0xd < 0x2)) goto LAB_1018_53f0;
      }
      uVar19 = (undefined2)(uStack22 >> 0x10);
      if (*(int *)((int)uStack22 + 0x8) != 0x0) {
        pcVar8 = string_op_1020_c2f8(*(ushort *)((int)uStack22 + 0x8));
        puVar10 = (undefined4 *)str_op_1008_60e8((char *)CONCAT22(puVar14,pcVar8),(ushort)puVar14);
        puVar15 = puVar14;
        puVar11 = puVar10;
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack14 = puVar11;
        puStack12 = puVar15;
        if (((uint)puVar15 | (uint)puVar11) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar19 = (undefined2)(uStack22 >> 0x10);
          puVar20 = struct_1018_48e8((ushort *)CONCAT22(puVar15,puVar11),(ulong)*(uint *)((int)uStack22 + 0xa),
                                     CONCAT22(puVar14,puVar10),*(ushort *)((int)uStack22 + 0x8));
          uVar16 = (undefined2)((ulong)puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uVar2 = *(undefined4 *)(iVar17 + 0xe);
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar19,uVar16);
      }
    }
  }
  uVar19 = (undefined2)(param_2 >> 0x10);
  uVar12 = *(ulong *)((int)param_2 + 0x3e);
  uVar13 = *(uint *)((int)param_2 + 0x40);
  uStack50 = (uint)uVar12;
  if ((uVar13 | uStack50) != 0x0) {
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_26),uVar12 & 0xffff | (ulong)uVar13 << 0x10);
    while( true ) {
      puVar6 = local_26;
      pass1_1008_5b12(puVar6,param_3);
      puVar14 = (uchar *)(extraout_DX_05 | (uint)puVar6);
      if (puVar14 == (uchar *)0x0) break;
      if (*(int *)(puVar6 + 0x4) != 0x0) {
        pcVar8 = string_1020_c0d8(*(ushort *)(puVar6 + 0x4));
        uVar13 = str_op_1008_60e8((char *)CONCAT22(puVar14,pcVar8),(ushort)puVar14);
        uStack30 = CONCAT22(puVar14,uVar13);
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack18 = (undefined4 *)uVar13;
        puStack16 = puVar14;
        if (((uint)puVar14 | uVar13) == 0x0) {
          uVar13 = 0x0;
          uVar19 = 0x0;
        }
        else {
          struct_1018_4790(CONCAT22(puVar14,uVar13),(ulong)*(uint *)(puVar6 + 0xa),uStack30,*(ushort *)(puVar6 + 0x4));
          uVar19 = extraout_DX_06;
        }
        uStack26 = CONCAT22(uVar19,uVar13);
        uVar2 = *(undefined4 *)(iVar17 + 0xe);
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar13,uVar19);
        puVar14 = extraout_DX_07;
      }
      if (*(int *)(puVar6 + 0x6) != 0x0) {
        pcVar8 = string_op_1020_c222(*(ushort *)(puVar6 + 0x6));
        puVar11 = (undefined4 *)str_op_1008_60e8((char *)CONCAT22(puVar14,pcVar8),(ushort)puVar14);
        uStack30 = CONCAT22(puVar14,puVar11);
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack14 = puVar11;
        puStack12 = puVar14;
        if (((uint)puVar14 | (uint)puVar11) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          puVar20 = struct_1018_48b0((ushort *)CONCAT22(puVar14,puVar11),(ulong)*(uint *)(puVar6 + 0xa),uStack30,
                                     *(ushort *)(puVar6 + 0x6));
          uVar16 = (undefined2)((ulong)puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uStack26 = CONCAT22(uVar16,uVar19);
        uVar2 = *(undefined4 *)(iVar17 + 0xe);
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar19,uVar16);
        puVar14 = extraout_DX_08;
      }
      if (*(int *)(puVar6 + 0x8) != 0x0) {
        pcVar8 = string_op_1020_c2f8(*(ushort *)(puVar6 + 0x8));
        uVar13 = str_op_1008_60e8((char *)CONCAT22(puVar14,pcVar8),(ushort)puVar14);
        uStack30 = CONCAT22(puVar14,uVar13);
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack18 = (undefined4 *)uVar13;
        puStack16 = puVar14;
        if (((uint)puVar14 | uVar13) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          puVar20 = struct_1018_48e8((ushort *)CONCAT22(puVar14,uVar13),(ulong)*(uint *)(puVar6 + 0xa),uStack30,
                                     *(ushort *)(puVar6 + 0x8));
          uVar16 = (undefined2)((ulong)puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uStack26 = CONCAT22(uVar16,uVar19);
        uVar2 = *(undefined4 *)(iVar17 + 0xe);
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,(int)uVar2,(char)((ulong)uVar2 >> 0x10),uVar19,uVar16);
      }
    }
  }
  return;
}



ushort * __stdcall16far pass1_1018_567c(ushort *param_1,byte param_2)

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



ulong __stdcall16far pass1_1018_56a8(ulong param_1,byte param_2,ushort param_3)

{
  pass1_1018_50ac((ushort *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1018_56e6(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)(param_1 + 0xa) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0x5830;
  *(undefined2 *)(param_1 + 0x2) = 0x1018;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1018_5714(ushort *param_1,ushort param_2)

{
  *param_1 = 0x5830;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  pass1_1010_1d80(param_1,param_2);
  return;
}



ulong __stdcall16far
pass1_1018_5732(ushort param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5,ushort param_6)

{
  ulong uVar1;
  
  uVar1 = pass1_1030_6d4e(param_3,param_4,param_5,param_6);
  return uVar1;
}



void __stdcall16far pass1_1018_5742(ushort param_1,ushort param_2,ulong *param_3,ulong param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  ulong uVar3;
  bool bVar4;
  undefined4 *puVar5;
  ulong uVar6;
  uint extraout_DX;
  uint extraout_DX_00;
  ulong uStack16;
  
  bVar4 = false;
  puVar1 = (undefined4 *)*(undefined4 *)((int)param_3 + 0x4);
  ppcVar2 = (code **)((int)*puVar1 + 0x10);
  puVar5 = puVar1;
  (**ppcVar2)();
  uVar3 = (ulong)puVar5 & 0xffff | (ulong)extraout_DX << 0x10;
  uStack16 = 0x0;
  do {
    if (uVar3 <= uStack16) {
LAB_1018_579f:
      if (!bVar4) {
        if (param_3 != (ulong *)0x0) {
          ppcVar2 = (code **)*param_3;
          (**ppcVar2)();
        }
        param_3 = (ulong *)0x0;
      }
      pass1_1030_6d80(param_4,(ulong)param_3);
      return;
    }
    ppcVar2 = (code **)((int)*puVar1 + 0x4);
    uVar6 = uVar3;
    (**ppcVar2)();
    if ((extraout_DX_00 | (uint)uVar6) != 0x0) {
      bVar4 = true;
      goto LAB_1018_579f;
    }
    uStack16 = uStack16 + 0x1;
  } while( true );
}



void __stdcall16far pass1_1018_57d2(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0xa) = param_2;
  return;
}



void __stdcall16far pass1_1018_57e6(ulong param_1,long param_2,ushort param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  send_dlg_item_msg_1040_d20c(*(ulong *)((int)param_1 + 0xa),param_2,(ushort)&PTR_LOOP_1050_1040,param_3);
  *(undefined4 *)((int)param_1 + 0xa) = 0x0;
  return;
}



ushort * __stdcall16far pass1_1018_580a(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1018_5714(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1018_5840(astruct_20 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uchar *extraout_DX;
  undefined2 uVar1;
  astruct_130 *iVar2;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  unk_draw_op_1020_7f7a(param_1,0x6,CONCAT22(param_3,param_2));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_130 *)param_1;
  iVar2->field_0xee = 0x0;
  *(undefined4 *)&iVar2->field_0xf2 = 0x0;
  iVar2->field_0xf6 = 0x0;
  param_1->field_0x0 = (int)s_Alloc__s_1050_5a5b + 0x7;
  iVar2->field_0x2 = 0x1018;
  iVar2->field_0xe2 = 0x5afe;
  iVar2->field_0xe4 = 0x1018;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x27,param_4,extraout_DX,unaff_DI);
  uVar1 = (undefined2)((ulong)puVar3 >> 0x10);
  iVar2->field_0xf2 = (int)puVar3;
  iVar2->field_0xf4 = uVar1;
  iVar2->field_0xe6 = iVar2->field_0xf2;
  iVar2->field_0xe8 = uVar1;
  return;
}



void __stdcall16far pass1_1018_58b6(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = (int)s_Alloc__s_1050_5a5b + 0x7;
  *(undefined2 *)(iVar1 + 0x2) = 0x1018;
  *(undefined2 *)(iVar1 + 0xe2) = 0x5afe;
  *(undefined2 *)(iVar1 + 0xe4) = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



void __stdcall16far invalidate_rect_1018_58e2(astruct_58 *param_1,int param_2,HWND16 param_3)

{
  int *piVar1;
  astruct_58 *iVar2;
  ushort uVar2;
  
  if (param_2 == 0x105) {
    uVar2 = (ushort)((ulong)param_1 >> 0x10);
    iVar2 = (astruct_58 *)param_1;
    piVar1 = &iVar2->field_0xf6;
    *piVar1 = *piVar1 + 0x1;
    if ((int)PTR_DAT_1050_0004_1050_4240 <= iVar2->field_0xf6) {
      PostMessage16(param_3,0x0,0x0,0x11100ca);
      return;
    }
    iVar2->field_0xea = 0x0;
    InvalidateRect16(param_3,(RECT16 *)0x0,0x0);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint __stdcall16far pass1_1018_5932(ulong param_1,ushort param_2,ushort param_3)

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
    uVar5 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar3 + 0x8),*(int *)(*(int *)(uVar3 + 0xf6) * 0x2 + 0x4238)
                            ,param_2,uVar3,(ushort)&PTR_LOOP_1050_1038,param_3);
    uVar2 = (uint)uVar5;
  }
  return uVar2;
}
