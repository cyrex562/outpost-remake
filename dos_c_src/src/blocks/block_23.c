
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_str_op_1008_d4f6(ulong param_1,ulong param_2)

{
  long lVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  code **ppcVar4;
  bool bVar5;
  undefined4 *puVar6;
  BOOL16 BVar7;
  ushort uVar8;
  ushort uVar9;
  uint uVar10;
  undefined4 *puVar11;
  ulong uVar12;
  undefined uVar13;
  uchar *extraout_DX;
  uchar *puVar14;
  undefined2 extraout_DX_00;
  undefined2 uVar15;
  uchar *extraout_DX_01;
  WORD *pWVar16;
  WORD *pWVar17;
  uchar *puVar18;
  uint uVar19;
  int iVar20;
  int iVar21;
  undefined2 uVar22;
  WORD *valist;
  ulong uVar23;
  uint uStack58;
  ulong uStack20;
  
  uVar22 = (undefined2)(param_2 >> 0x10);
  iVar20 = (int)param_2;
  lVar1 = *(long *)(iVar20 + 0x200);
  valist = (WORD *)(param_1 >> 0x10);
  iVar21 = (int)param_1;
  puVar6 = (undefined4 *)*(uint *)(iVar21 + 0xe);
  puVar14 = *(uchar **)(iVar21 + 0x10);
  if (((uint)puVar14 | (uint)puVar6) != 0x0) {
    ppcVar4 = (code **)*puVar6;
    (**ppcVar4)();
    puVar14 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar14,0x1000);
  if (((uint)puVar14 | (uint)puVar6) == 0x0) {
    puVar6 = (undefined4 *)0x0;
    uVar15 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar14,puVar6));
    uVar15 = extraout_DX_00;
  }
  *(undefined2 *)(iVar21 + 0xe) = puVar6;
  *(undefined2 *)(iVar21 + 0x10) = uVar15;
  puVar2 = (undefined4 *)*(ulong *)(iVar20 + 0xc);
  ppcVar4 = (code **)((int)*puVar2 + 0x10);
  puVar11 = puVar2;
  (**ppcVar4)(0x1000,(int)puVar2,*(undefined2 *)(iVar20 + 0xe));
  uVar12 = (ulong)puVar11 & 0xffff | ZEXT24(extraout_DX_01) << 0x10;
  bVar5 = false;
  for (uStack20 = 0x0; uStack20 < uVar12; uStack20 = uStack20 + 0x1) {
    uVar23 = pass1_1030_1d7c((int)((ulong)puVar11 & 0xffff),extraout_DX_01,(ulong)puVar2);
    uVar19 = (uint)(uVar23 >> 0x10);
    uVar10 = (uint)uVar23;
    if ((((uVar19 | uVar10) != 0x0) && (*(long *)(uVar10 + 0x1c) != 0x8000002)) &&
       ((iVar20 = *(int *)(uVar10 + 0x12), iVar20 == 0x5 || (iVar20 == 0x6)))) {
      uVar9 = *(ushort *)(uVar10 + 0xc);
      BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar9,0x34);
      if ((BVar7 == 0x0) && (*(long *)(uVar10 + 0x1c) != lVar1)) {
        pass1_1020_bd80(uVar9);
        pWVar16 = valist;
        wsprintf16((LPSTR)0x1020,(LPCSTR)(iVar21 + 0x22),valist);
        uVar8 = str_op_1008_60e8((char *)(param_1 & 0xffff0000 | ZEXT24((LPCSTR)(iVar21 + 0x22))),(ushort)pWVar16);
        uVar22 = 0x1000;
        pWVar17 = pWVar16;
        uVar9 = uVar8;
        mem_op_1000_179c(0x14,(uchar *)pWVar16,0x1000);
        uStack58 = (uint)pWVar17 | uVar9;
        if (uStack58 == 0x0) {
          uVar9 = 0x0;
          uStack58 = 0x0;
        }
        else {
          uVar22 = 0x1018;
          struct_1018_47c8((ushort *)CONCAT22(pWVar17,uVar9),0x1,CONCAT22(pWVar16,uVar8),*(ushort *)(uVar10 + 0xc),
                           *(ulong *)(uVar10 + 0x4));
        }
        uVar3 = *(undefined4 *)(iVar21 + 0xe);
        ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar21 + 0xe) + 0x4);
        (**ppcVar4)(uVar22,(int)uVar3,(char)((ulong)uVar3 >> 0x10),uVar9,uStack58);
        bVar5 = true;
      }
    }
  }
  if (!bVar5) {
    puVar14 = extraout_DX_01;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar22 = 0x1000;
    puVar18 = puVar14;
    uVar10 = (uint)uVar12;
    mem_op_1000_179c(0x14,puVar14,0x1000);
    uVar19 = (uint)puVar18 | uVar10;
    if (uVar19 == 0x0) {
      uVar10 = 0x0;
      uVar13 = 0x0;
    }
    else {
      uVar22 = 0x1018;
      struct_1018_47c8((ushort *)CONCAT22(puVar18,uVar10),0x0,
                       CONCAT13((char)((uint)puVar14 >> 0x8),CONCAT12((char)puVar14,(uint)uVar12)),0x0,0x0);
      uVar13 = (undefined)uVar19;
    }
    uVar3 = *(undefined4 *)(iVar21 + 0xe);
    ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar21 + 0xe) + 0x4);
    (**ppcVar4)(uVar22,(char)uVar3,(int)((ulong)uVar3 >> 0x10),uVar10,uVar13);
  }
  return;
}



ushort * __stdcall16far pass1_1008_d6f4(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1008_caa0(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_d72e(int param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined2 *)(param_1 + 0xa) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xd780;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort * __stdcall16far pass1_1008_d75a(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_d790(astruct_647 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  astruct_43 *paVar1;
  
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xd98e;
  param_1->field_0x2 = 0x1008;
  paVar1 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x9b,param_4);
  param_1->field_0xa = (int)paVar1;
  param_1->field_0xc = (int)((ulong)paVar1 >> 0x10);
  return;
}



void __stdcall16far pass1_1008_d7da(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0xd98e;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xa);
  uVar2 = *(uint *)(iVar4 + 0xc);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1008_d818(ulong param_1,int param_2)

{
  astruct_732 *iVar1;
  undefined2 uVar1;
  
  if (param_2 - 0x1a0U < 0x15) {
    iVar1 = (astruct_732 *)param_1;
    uVar1 = (undefined2)(param_1 >> 0x10);
    switch(param_2) {
    case 0x1a0:
      iVar1->field_0xe = 0x14;
      break;
    case 0x1a1:
      iVar1->field_0xe = 0x3;
      break;
    case 0x1a2:
      iVar1->field_0xe = 0x2;
      break;
    case 0x1a3:
      iVar1->field_0xe = 0xe;
      break;
    case 0x1a4:
      iVar1->field_0xe = 0xc;
      break;
    case 0x1a5:
      iVar1->field_0xe = 0x4;
      break;
    case 0x1a6:
      iVar1->field_0xe = 0xb;
      break;
    case 0x1a7:
      iVar1->field_0xe = 0x6;
      break;
    case 0x1a8:
      iVar1->field_0xe = 0xa;
      break;
    case 0x1a9:
      iVar1->field_0xe = 0xd;
      break;
    case 0x1aa:
      iVar1->field_0xe = 0x13;
      break;
    case 0x1ab:
      iVar1->field_0xe = 0x5;
      break;
    case 0x1ac:
      iVar1->field_0xe = 0x9;
      break;
    case 0x1ad:
      iVar1->field_0xe = 0x8;
      break;
    case 0x1ae:
      iVar1->field_0xe = 0x12;
      break;
    case 0x1af:
      iVar1->field_0xe = 0x11;
      break;
    case 0x1b0:
      iVar1->field_0xe = 0x7;
      return;
    case 0x1b1:
      iVar1->field_0xe = 0x10;
      return;
    case 0x1b2:
      iVar1->field_0xe = 0x1;
      return;
    case 0x1b3:
      iVar1->field_0xe = 0xf;
      return;
    case 0x1b4:
      iVar1->field_0xe = 0x15;
      return;
    }
  }
  return;
}



ushort * __stdcall16far pass1_1008_d968(ushort *param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_d7da(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_d99e(int param_1,ushort param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  struct_op_1018_4cda(param_1,param_2,param_3);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xd9fa;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x9a,param_4,param_5);
  _PTR_LOOP_1050_4230 = CONCAT22(param_2,param_1);
  return;
}



astruct_11 * __stdcall16far pass1_1008_d9d4(astruct_11 *param_1,byte param_2)

{
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_draw_op_1008_da12(astruct_19 *param_1,ushort param_2,ushort param_3)

{
  int *piVar1;
  byte bVar2;
  ulong uVar3;
  undefined2 *puVar4;
  HDC16 hdc;
  INT16 IVar6;
  int iVar7;
  uint uVar8;
  astruct_80 *IVar5;
  ushort start;
  ushort uVar9;
  PALETTEENTRY *entries;
  undefined *count;
  int iVar10;
  HWND16 hwnd;
  undefined2 *puStack32;
  int iStack16;
  long lStack8;
  
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xc = 0x0;
  pass1_1008_3e38((ushort *)CONCAT22(param_2,&param_1->field_0xe));
  param_1->field_0x14 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x18 = (undefined2 *)0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xdc80;
  param_1->field_0x2 = 0x1008;
  hdc = GetDC16(0x1010);
  IVar6 = GetDeviceCaps16((HDC16)s_tile2_bmp_1050_1538,0x8);
  param_1->field_0xa = IVar6;
  IVar6 = GetDeviceCaps16((HDC16)s_tile2_bmp_1050_1538,0xa);
  param_1->field_0xc = IVar6;
  iVar7 = param_1->field_0xc + -0x1e0;
  count = (undefined *)(iVar7 >> 0xf);
  pass1_1008_3e76((ushort *)CONCAT22(param_2,&param_1->field_0xe),0x0,iVar7 / 0x2,(param_1->field_0xa + -0x280) / 0x2);
  hwnd = (HWND16)s_tile2_bmp_1050_1538;
  uVar8 = GetDeviceCaps16((HDC16)s_tile2_bmp_1050_1538,0x26);
  if ((uVar8 & 0x100) != 0x0) {
    IVar6 = GetDeviceCaps16((HDC16)s_tile2_bmp_1050_1538,0x68);
    param_1->field_0x14 = IVar6;
    IVar5 = (astruct_80 *)GetDeviceCaps16((HDC16)s_tile2_bmp_1050_1538,0x6a);
    param_1->field_0x16 = (INT16)IVar5;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)count,0x1000);
    }
    else {
      count = PTR_LOOP_1050_5f2e;
    }
    start = fn_ptr_op_1000_1708((int)(IVar5 + 0x1) * 0x4,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)count,0x1000);
    lStack8 = CONCAT22(count,start);
    iVar7 = param_1->field_0x16;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2e = count;
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)count,0x1000);
    }
    else {
    }
    uVar9 = fn_ptr_op_1000_1708((iVar7 + 0x1) * 0x4,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    *(ushort *)&param_1->field_0x18 = uVar9;
    *(undefined2 *)((int)&param_1->field_0x18 + 0x2) = PTR_LOOP_1050_5f2e;
    if (lStack8 != 0x0) {
      if (param_1->field_0x18 != (undefined2 *)0x0) {
        entries = (PALETTEENTRY *)(param_1->field_0x16 / 0x2);
        GetSystemPaletteEntries(0x1000,start,(UINT16)count,entries);
        GetSystemPaletteEntries((HDC16)s_tile2_bmp_1050_1538,(int)entries * 0x4 + start,(UINT16)count,entries);
        puStack32 = param_1->field_0x18;
        for (iStack16 = 0x0; puVar4 = puStack32, piVar1 = &param_1->field_0x16,
            *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 = iStack16 + 0x1) {
          bVar2 = *(byte *)(iStack16 * 0x4 + start);
          iVar7 = iStack16 * 0x4 + start;
          uVar3 = (ulong)puStack32 >> 0x10;
          iVar10 = (int)puStack32;
          puStack32 = (undefined2 *)((ulong)puStack32 & 0xffff0000 | (ulong)(iVar10 + 0x4));
          *puVar4 = CONCAT11(*(undefined *)(iVar7 + 0x1),*(undefined *)(iVar7 + 0x2));
          *(uint *)(iVar10 + 0x2) = (uint)bVar2;
        }
      }
    }
    hwnd = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(count,start),0x1000);
  }
  ReleaseDC16(hwnd,hdc);
  return;
}



void __stdcall16far pass1_1008_dc2c(ushort *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0xdc80;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x18),0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



ushort * __stdcall16far pass1_1008_dc5a(ushort *param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_dc2c(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Variable defined which should be unmapped: param_10

void __stdcall16far
pass1_1008_dc80(ushort param_1,ushort *param_2,ulong param_3,ulong param_4,uint param_5,uchar param_6,int param_7,
               int param_8,byte param_9,ushort param_10)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  code *pcVar4;
  uint uVar5;
  char cVar6;
  char extraout_DL;
  byte bVar7;
  int iVar8;
  undefined2 uVar9;
  byte bVar10;
  
  bVar7 = (byte)(param_10 >> 0x8);
  bVar10 = (byte)param_10 + bVar7;
  cVar6 = bVar10 + param_9;
  uVar2 = (uint)(CARRY1((byte)param_10,bVar7) || CARRY1(bVar10,param_9));
  uVar3 = param_5 + 0xeff0;
  bVar10 = param_5 < 0x1010 || uVar3 < uVar2;
  uVar5 = uVar3 - uVar2;
  pcVar4 = (code *)swi(0x4);
  if (SBORROW2(param_5,0x1010) != SBORROW2(uVar3,uVar2)) {
    (*pcVar4)();
    cVar6 = extraout_DL;
  }
  pcVar1 = (char *)(param_7 + param_8);
  *pcVar1 = *pcVar1 + cVar6 + (uVar5 < 0x1010 || uVar5 + 0xeff0 < (uint)bVar10);
  uVar9 = (undefined2)((ulong)param_2 >> 0x10);
  iVar8 = (int)param_2;
  *param_2 = 0x389a;
  *(undefined2 *)(iVar8 + 0x2) = 0x1008;
  *(ulong *)(iVar8 + 0x4) = param_4;
  *(ulong *)(iVar8 + 0x8) = param_3;
  *(undefined2 *)(iVar8 + 0xc) = 0x0;
  *(undefined4 *)(iVar8 + 0xe) = 0x0;
  *(undefined2 *)(iVar8 + 0x12) = 0x0;
  *param_2 = 0xdd4a;
  *(undefined2 *)(iVar8 + 0x2) = 0x1008;
  return;
}



void __stdcall16far struct_1008_dc90(ushort *param_1,ulong param_2,ulong param_3)

{
  astruct_212 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_212 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x0;
  *param_1 = 0xdd4a;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far struct_1008_dcdc(ushort *param_1)

{
  astruct_220 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_220 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x0;
  *param_1 = 0xdd4a;
  iVar1->field_0x2 = 0x1008;
  return;
}



ushort * __stdcall16far pass1_1008_dd1e(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1008_dd4e(astruct_209 *param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  uchar *puVar2;
  undefined2 extraout_DX;
  astruct_79 *paVar3;
  
  paVar3 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar2 = (uchar *)((ulong)paVar3 >> 0x10);
  uVar1 = 0x0;
  *(undefined4 *)&param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xeaac;
  param_1->field_0x2 = 0x1008;
  mem_op_1000_179c(0xc,puVar2,0x1000);
  if (((uint)puVar2 | uVar1) == 0x0) {
    *(undefined4 *)&param_1->field_0xa = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2,uVar1));
    param_1->field_0xa = uVar1;
    param_1->field_0xc = extraout_DX;
  }
  return;
}



void __stdcall16far pass1_1008_ddca(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_471 *iVar5;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_471 *)param_1;
  *param_1 = 0xeaac;
  iVar5->field_0x2 = 0x1008;
  puVar1 = iVar5->field_0xe;
  uVar2 = iVar5->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar5->field_0x12;
  uVar2 = iVar5->field_0x14;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar5->field_0xa;
  uVar2 = iVar5->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x1e,0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_de58(ulong param_1,long param_2,long param_3,ushort param_4)

{
  code **ppcVar1;
  bool bVar2;
  astruct_210 *puVar4;
  uint extraout_DX;
  uchar *puVar3;
  uint uVar4;
  astruct_211 *iVar6;
  astruct_210 *paVar5;
  undefined2 uVar6;
  ulong uVar7;
  undefined local_a [0x8];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_211 *)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),(ulong)iVar6->field_0xa);
  bVar2 = false;
  do {
    puVar4 = (astruct_210 *)local_a;
    pass1_1008_5b12(puVar4,param_4);
    puVar3 = (uchar *)(extraout_DX | (uint)puVar4);
    paVar5 = puVar4;
    if (puVar3 == (uchar *)0x0) goto LAB_1008_dedb;
  } while (((puVar4->field_0x4 != param_3) || (puVar4->field_0x8 != param_2)) &&
          ((puVar4->field_0x8 != param_3 || (puVar4->field_0x4 != param_2))));
  puVar4->field_0xc = 0x1;
  uVar7 = pass1_1030_8326();
  puVar3 = (uchar *)(uVar7 >> 0x10);
  paVar5 = (astruct_210 *)uVar7;
  puVar4->field_0xe = (undefined *)paVar5;
  puVar4->field_0x10 = puVar3;
  bVar2 = true;
LAB_1008_dedb:
  if (!bVar2) {
    mem_op_1000_179c(0x14,puVar3,0x1000);
    uVar4 = (uint)puVar3 | (uint)paVar5;
    if (uVar4 == 0x0) {
      paVar5 = (astruct_210 *)0x0;
      uVar4 = 0x0;
    }
    else {
      struct_1008_dc90((ushort *)CONCAT22(puVar3,paVar5),param_2,param_3);
    }
    paVar5->field_0xc = 0x1;
    uVar7 = pass1_1030_8326();
    paVar5->field_0xe = (undefined *)uVar7;
    paVar5->field_0x10 = (uchar *)(uVar7 >> 0x10);
    ppcVar1 = (code **)((int)*iVar6->field_0xa + 0x4);
    (**ppcVar1)();
  }
  return;
}



void __stdcall16far pass1_1008_df4a(ulong param_1,int param_2,ushort param_3)

{
  undefined2 uVar1;
  ushort uVar2;
  ulong uVar3;
  undefined local_a [0x8];
  
  uVar2 = (ushort)(param_1 >> 0x10);
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((ushort)param_1 + 0xa));
  while( true ) {
    uVar3 = pass1_1008_5b12(local_a,param_3);
    uVar1 = (undefined2)(uVar3 >> 0x10);
    if (uVar3 == 0x0) break;
    if ((*(int *)((int)uVar3 + 0xc) == 0x2) || (*(int *)((int)uVar3 + 0xc) == 0x3)) {
      pass1_1008_e9a4((ushort)param_1,uVar2,uVar3,param_2,param_3);
    }
  }
  return;
}



void __stdcall16far pass1_1008_dfa6(ulong param_1,long param_2,long param_3,ushort param_4)

{
  undefined *puVar1;
  uint extraout_DX;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
  } while (((*(long *)(puVar1 + 0x4) != param_3) || (*(long *)(puVar1 + 0x8) != param_2)) &&
          ((*(long *)(puVar1 + 0x8) != param_3 || (*(long *)(puVar1 + 0x4) != param_2))));
  if (*(int *)(puVar1 + 0xc) != 0x1) {
    return;
  }
  return;
}



void __stdcall16far pass1_1008_e01c(ulong param_1,ulong param_2,ulong param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ulong *)((int)param_1 + 0x16) = param_3;
  *(ulong *)((int)param_1 + 0x1a) = param_2;
  return;
}



void __stdcall16far pass1_1008_e038(ulong param_1,ulong *param_2,ulong *param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *param_3 = *(ulong *)((int)param_1 + 0x16);
  *param_2 = *(ulong *)((int)param_1 + 0x1a);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1008_e05e(ulong param_1,ushort param_2,ulong param_3,ulong param_4,ushort param_5,uchar param_6)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  undefined local_122 [0x112];
  int iStack16;
  undefined local_e [0x8];
  long lStack6;
  
  lStack6 = pass1_1008_e8cc(param_5,param_1,param_3,param_4);
  if (lStack6 != 0x0) {
    uVar3 = pass1_1030_8326();
    uVar2 = (undefined2)((ulong)lStack6 >> 0x10);
    iVar1 = (int)lStack6;
    *(undefined2 *)(iVar1 + 0xe) = (int)uVar3;
    *(undefined2 *)(iVar1 + 0x10) = (int)(uVar3 >> 0x10);
    *(ushort *)(iVar1 + 0xc) = param_2;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_e),*(ulong *)((int)param_1 + 0xa));
  iStack16 = 0x0;
  do {
    lStack6 = pass1_1008_5b12(local_e,param_5);
    if (lStack6 == 0x0) goto LAB_1008_e0d3;
  } while (*(int *)((int)lStack6 + 0xc) != 0x1);
  iStack16 = 0x1;
LAB_1008_e0d3:
  if (iStack16 == 0x0) {
    struct_1030_e2be((astruct_100 *)CONCAT22(param_5,local_122),0x0,0x0,0x0,param_5,param_6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_5,local_122));
  }
  return;
}



int __stdcall16far pass1_1008_e10c(ulong param_1,ulong param_2,ulong param_3,int param_4,ushort param_5)

{
  int iVar1;
  int iVar2;
  ulong uVar3;
  
  uVar3 = pass1_1008_e8cc(param_5,param_1,param_2,param_3);
  if (uVar3 == 0x0) {
    return 0x1;
  }
  iVar1 = *(int *)((int)uVar3 + 0xc);
  if (-0x1 < iVar1) {
    if (iVar1 < 0x2) {
      return 0x1;
    }
    if ((0x0 < iVar1 + -0x1) && (iVar2 = iVar1 + -0x3, iVar2 == 0x0 || iVar1 + -0x2 < 0x1)) {
      pass1_1008_e9a4((ushort)param_1,(ushort)(param_1 >> 0x10),uVar3,param_4,param_5);
      return iVar2;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_e164(ulong param_1,ushort param_2,uchar param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  astruct_215 *uVar5;
  astruct_215 *paVar3;
  astruct_216 *paVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  uint uVar8;
  astruct_214 *uVar11;
  astruct_215 *paVar9;
  astruct_213 *iVar12;
  ushort uVar10;
  undefined2 uVar12;
  ulong uVar13;
  undefined local_118 [0x112];
  long lStack6;
  astruct_216 *iVar1;
  
  uVar10 = (ushort)(param_1 >> 0x10);
  uVar11 = (astruct_214 *)param_1;
  lStack6 = pass1_1008_e8cc(param_2,param_1,uVar11->field_0x1a,uVar11->field_0x16);
  uVar8 = (uint)((ulong)lStack6 >> 0x10);
  uVar5 = (astruct_215 *)lStack6;
  puVar5 = (uchar *)(uVar8 | (uint)uVar5);
  if (lStack6 == 0x0) {
    pass1_1008_e852((ushort)uVar11,uVar10,uVar11->field_0x16,param_2,(uint)puVar5);
    paVar3 = uVar5;
    puVar6 = puVar5;
    pass1_1008_e852((ushort)uVar11,uVar10,uVar11->field_0x1a,param_2,(uint)puVar5);
    paVar9 = paVar3;
    puVar7 = puVar6;
    mem_op_1000_179c(0x14,puVar6,0x1000);
    uVar8 = (uint)puVar7 | (uint)paVar9;
    if (uVar8 == 0x0) {
      paVar9 = (astruct_215 *)0x0;
      uVar8 = 0x0;
    }
    else {
      struct_1008_dc90((ushort *)CONCAT22(puVar7,paVar9),
                       CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12((char)puVar6,paVar3)),CONCAT22(puVar5,uVar5));
    }
    lStack6 = CONCAT22(uVar8,paVar9);
    paVar9->field_0xc = 0x1;
    uVar13 = pass1_1030_8326();
    uVar12 = (undefined2)((ulong)lStack6 >> 0x10);
    iVar12 = (astruct_213 *)lStack6;
    iVar12->field_0xe = (int)uVar13;
    iVar12->field_0x10 = (int)(uVar13 >> 0x10);
    puVar1 = uVar11->field_0xa;
    ppcVar2 = (code **)((int)*uVar11->field_0xa + 0x4);
    (**ppcVar2)(0x1030,(char)puVar1,(char)((ulong)puVar1 >> 0x10),iVar12,uVar12);
  }
  else {
    iVar1 = (astruct_216 *)uVar5->field_0xc;
    paVar4 = iVar1 + -0x1;
    if (paVar4 == (astruct_216 *)0x0) {
      return;
    }
    if (((0x0 < (int)paVar4) && (!SBORROW2((int)paVar4,0x1))) && ((int)(iVar1 + -0x2) < 0x2)) {
      uVar5->field_0x12 = 0x1;
    }
    uVar5->field_0xc = 0x1;
  }
  uVar12 = (undefined2)((ulong)lStack6 >> 0x10);
  struct_1030_e2be((astruct_100 *)CONCAT22(param_2,local_118),0x1,*(ulong *)((int)lStack6 + 0x8),
                   *(ulong *)((int)lStack6 + 0x4),param_2,param_3);
  uVar13 = pass1_1030_8326();
  pass1_1030_8372(_PTR_LOOP_1050_5748,uVar13 + 0x1,(ulong *)CONCAT22(param_2,local_118));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1008_e2a4(ulong param_1,ulong param_2,ulong param_3)

{
  int iVar1;
  int iVar2;
  ushort unaff_SS;
  char *pcVar3;
  long lVar4;
  ulong uVar5;
  
  uVar5 = param_2;
  pcVar3 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  iVar1 = pass1_1000_3d7a((ulong)pcVar3,uVar5);
  if ((iVar1 == 0x0) || (iVar1 = pass1_1000_3d7a(param_3,param_2), iVar1 == 0x0)) {
    return 0x0;
  }
  lVar4 = pass1_1008_e8cc(unaff_SS,param_1,param_2,param_3);
  if (lVar4 != 0x0) {
    iVar1 = *(int *)((int)lVar4 + 0xc);
    iVar2 = iVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x2;
    }
    if (iVar2 < 0x1) {
      return 0x0;
    }
    if (SBORROW2(iVar2,0x1)) {
      return 0x0;
    }
    if (0x1 < iVar1 + -0x2) {
      return 0x0;
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_e320(astruct_102 *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  astruct_103 *paVar1;
  astruct_103 *uVar2;
  uint uVar3;
  uint uVar4;
  astruct_102 *iVar5;
  astruct_102 *uVar6;
  char *pcVar5;
  long lVar6;
  ulong uVar7;
  
  uVar6 = (astruct_102 *)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_102 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0x1e,0x1000);
  *(undefined4 *)&iVar5->field_0x1e = 0x0;
  uVar7 = param_2;
  pcVar5 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uVar4 = (uint)((ulong)pcVar5 >> 0x10);
  uVar2 = (astruct_103 *)pass1_1000_3d7a((ulong)pcVar5,uVar7);
  if ((uVar2 != (astruct_103 *)0x0) &&
     (uVar2 = (astruct_103 *)pass1_1000_3d7a(param_3,param_2), uVar2 != (astruct_103 *)0x0)) {
    lVar6 = pass1_1008_e8cc(param_4,(ulong)param_1,param_2,param_3);
    uVar3 = (uint)((ulong)lVar6 >> 0x10);
    uVar2 = (astruct_103 *)lVar6;
    uVar4 = uVar3 | (uint)uVar2;
    if ((uVar4 != 0x0) &&
       (((paVar1 = (astruct_103 *)uVar2->field_0xc, uVar2 = paVar1, paVar1 != (astruct_103 *)0x0 &&
         (uVar2 = (astruct_103 *)((int)&paVar1[-0x1].field_0xc + 0x1), uVar2 != (astruct_103 *)0x0)) &&
        (uVar2 = (astruct_103 *)&paVar1[-0x1].field_0xc, uVar2 != (astruct_103 *)0x0)))) {
      uVar2 = (astruct_103 *)&paVar1[-0x1].field_0xb;
    }
  }
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  iVar5->field_0x1e = (uint)uVar2;
  iVar5->field_0x20 = uVar4;
  return;
}



void __stdcall16far pass1_1008_e3ec(ulong param_1,ulong *param_2,ulong *param_3,ushort param_4)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  astruct_219 *paVar4;
  undefined4 *puVar5;
  astruct_219 *puVar4;
  uchar *extraout_DX;
  uint extraout_DX_00;
  uint uVar6;
  uint uVar7;
  uint extraout_DX_01;
  uchar *puVar8;
  uchar *extraout_DX_02;
  undefined2 extraout_DX_03;
  undefined2 uVar9;
  uint extraout_DX_04;
  astruct_218 *iVar10;
  undefined2 uVar10;
  astruct_219 local_14;
  undefined2 uStack12;
  uint uStack10;
  undefined2 uStack8;
  uint uStack6;
  int iStack4;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar10 = (astruct_218 *)param_1;
                    // WARNING: Load size is inaccurate
  puVar5 = iVar10->field_0xe;
  puVar8 = *(uchar **)((int)&iVar10->field_0xe + 0x2);
  if (((uint)puVar8 | (uint)puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
    puVar8 = extraout_DX;
  }
  mem_op_1000_179c(0x18,puVar8,0x1000);
  if (((uint)puVar8 | (uint)puVar5) == 0x0) {
    puVar5 = (undefined4 *)0x0;
    uVar6 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT13((char)((uint)puVar8 >> 0x8),CONCAT12((char)puVar8,puVar5)),0x5,0x5);
    uVar6 = extraout_DX_00;
  }
  *(undefined4 **)&iVar10->field_0xe = puVar5;
  *(uint *)((int)&iVar10->field_0xe + 0x2) = uVar6;
  pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,&local_14)),0x1,0x0,0x400);
  while( true ) {
    uVar7 = uVar6;
    paVar4 = &local_14;
    pass1_1028_e4ec(CONCAT22(param_4,paVar4));
    if ((uVar7 | (uint)paVar4) == 0x0) break;
    uVar6 = uVar7 | (uint)paVar4;
    if (*(long *)(paVar4 + 0x40) != 0x8000002) {
      uVar1 = paVar4->field_0x4;
      puVar2 = iVar10->field_0xe;
      ppcVar3 = (code **)((int)*iVar10->field_0xe + 0xc);
      (**ppcVar3)(0x28,(char)puVar2,(int)((ulong)puVar2 >> 0x10),(int)uVar1,(int)((ulong)uVar1 >> 0x10));
      uVar6 = extraout_DX_01;
    }
  }
  *param_3 = (ulong)iVar10->field_0xe;
  uVar6 = *(uint *)((int)&iVar10->field_0x12 + 0x2);
  puVar5 = (undefined4 *)iVar10->field_0x12;
  puVar8 = (uchar *)(uVar6 | (uint)puVar5);
  if (puVar8 != (uchar *)0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)(0x28,puVar5,uVar6,0x1);
    puVar8 = extraout_DX_02;
  }
  mem_op_1000_179c(0x18,puVar8,0x1000);
  if (((uint)puVar8 | (uint)puVar5) == 0x0) {
    puVar5 = (undefined4 *)0x0;
    uVar9 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT13((char)((uint)puVar8 >> 0x8),CONCAT12((char)puVar8,puVar5)),0x5,0x5);
    uVar9 = extraout_DX_03;
  }
  *(undefined4 **)&iVar10->field_0x12 = puVar5;
  *(undefined2 *)((int)&iVar10->field_0x12 + 0x2) = uVar9;
  uStack12 = uStack8;
  uStack10 = uStack6;
  if (iStack4 != 0x0) {
    uStack12 = 0x1;
    uStack6 = 0x0;
    uStack10 = uStack6;
  }
  while( true ) {
    puVar4 = &local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar4));
    if ((uStack6 | (uint)puVar4) == 0x0) break;
    uVar1 = puVar4->field_0x4;
    puVar2 = iVar10->field_0x12;
    ppcVar3 = (code **)((int)*iVar10->field_0x12 + 0xc);
    (**ppcVar3)(0x28,(char)puVar2,(char)((ulong)puVar2 >> 0x10),(int)uVar1,(int)((ulong)uVar1 >> 0x10));
    uStack6 = extraout_DX_04;
  }
  *param_2 = (ulong)iVar10->field_0x12;
  return;
}
