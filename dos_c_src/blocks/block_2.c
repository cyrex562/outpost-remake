
BOOL16 __stdcall16far call_fn_ptr_1000_0dc6(u16 param_1,u16 param_2,u16 param_3)

{
  if ((*(uint *)&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(param_3,0xe,0x0,0x0);
    return 0x0;
  }
  (**(code **)0x8)((int)&USHORT_1050_1050);
  return 0x1;
}



uint __cdecl16near pass1_1000_0e08(int param_1,ushort param_2)

{
  uint *puVar1;
  byte *pbVar2;
  uint uVar3;
  uint *puVar4;
  uint *puVar5;
  bool bVar6;
  ulong uVar7;
  
  puVar5 = (uint *)(param_1 + -0x2);
  bVar6 = (*(byte *)puVar5 & 0x2) != 0x0;
  if (bVar6) {
    puVar1 = puVar5;
    *(byte *)puVar1 = *(byte *)puVar1 & 0xfe;
  }
  else {
    puVar4 = (uint *)((int)puVar5 - *(int *)(param_1 + -0x4));
    puVar1 = puVar4;
    *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
    puVar5 = puVar4;
  }
  puVar4 = (uint *)((*puVar5 & 0xfffc) + (int)puVar5);
  if ((*(byte *)puVar4 & 0x1) == 0x0) {
    puVar1 = puVar5;
    *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
    if (puVar4 == (uint *)PTR_LOOP_1050_000e) {
      PTR_LOOP_1050_000e = (undefined *)puVar5;
    }
    *(uint *)(puVar4[0x2] + 0x2) = puVar4[0x1];
    *(uint *)(puVar4[0x1] + 0x4) = puVar4[0x2];
    puVar4 = (uint *)((*puVar5 & 0xfffc) + (int)puVar5);
  }
  puVar4[-0x1] = *puVar5 & 0xfffc;
  uVar3 = *DAT_1050_0004;
  puVar1 = puVar4 + -0x1;
  if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
    uVar3 = *puVar5 & 0xfffc;
    *DAT_1050_0004 = uVar3;
  }
  puVar1 = puVar4;
  *(byte *)puVar1 = *(byte *)puVar1 & 0xfd;
  if (bVar6) {
    if (*(undefined **)(PTR_LOOP_1050_0010 + 0x2) != PTR_LOOP_1050_0010) {
      pbVar2 = (byte *)((int)DAT_1050_0004 + 0x3);
      *pbVar2 = *pbVar2 & 0x7f;
    }
    puVar5[0x2] = (uint)PTR_LOOP_1050_0010;
    uVar3 = *(uint *)(PTR_LOOP_1050_0010 + 0x2);
    puVar5[0x1] = uVar3;
    *(uint **)(*(int *)(PTR_LOOP_1050_0010 + 0x2) + 0x4) = puVar5;
    *(uint **)(PTR_LOOP_1050_0010 + 0x2) = puVar5;
  }
  PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
  if (PTR_LOOP_1050_000a == (undefined *)0x0) {
    uVar7 = mem_op_1000_0510(0x1,0x0,param_2);
    uVar3 = (uint)uVar7;
  }
  return uVar3;
}



long __stdcall16far
pass1_1000_0ed4(ushort param_1,ushort param_2,uint param_3,uint param_4,uint param_5,ushort *param_6,ushort param_7)

{
  ushort *puVar1;
  ushort *puVar2;
  undefined2 uVar3;
  ushort **ppuVar4;
  uint uVar5;
  uint uVar6;
  ushort *puVar7;
  ushort *puVar8;
  long lVar9;
  UINT16 UVar10;
  UINT16 UVar11;
  UINT16 UVar12;
  
  if ((*(uint *)&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
    UVar11 = *(UINT16 *)0x0;
    UVar12 = *(UINT16 *)&PTR_LOOP_1050_0002;
    if ((param_3 & 0x8) == 0x0) {
      ppuVar4 = &param_6;
    }
    else {
      ppuVar4 = (ushort **)0x0;
      param_2 = 0x0;
    }
    uVar5 = pass1_1000_0fb8(param_1,param_4,(int)param_6,param_5,param_3,(ushort *)ppuVar4,param_2);
    if (uVar5 == 0x0) {
      return CONCAT22(param_7,param_6);
    }
    if ((param_3 & 0x8) == 0x0) {
      lVar9 = mem_op_1000_0a48((byte)param_3,param_4,param_5,CONCAT22(UVar12,UVar11),param_1);
      uVar3 = (undefined2)((ulong)lVar9 >> 0x10);
      puVar8 = (ushort *)lVar9;
      if (lVar9 != 0x0) {
        puVar7 = param_6;
        for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
          puVar2 = puVar8;
          puVar8 = puVar8 + 0x1;
          puVar1 = puVar7;
          puVar7 = puVar7 + 0x1;
          *puVar2 = *puVar1;
        }
        for (uVar5 = (uint)((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
          puVar2 = puVar8;
          puVar8 = (ushort *)((int)puVar8 + 0x1);
          puVar1 = puVar7;
          puVar7 = (ushort *)((int)puVar7 + 0x1);
          *(undefined *)puVar2 = *(undefined *)puVar1;
        }
        call_fn_ptr_1000_0dc6((u16)param_6,param_7,param_1);
      }
      return lVar9;
    }
    if ((param_5 | param_4) == 0x0) {
      return 0x0;
    }
    UVar10 = 0x5;
  }
  else {
    UVar11 = 0x0;
    UVar12 = 0x0;
    UVar10 = 0xe;
  }
  pass1_1000_1e61(param_1,UVar10,UVar11,UVar12);
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10001126)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint pass1_1000_0fb8(ushort param_1,uint param_2,int param_3,uint param_4,uint param_5,ushort *param_6,uint param_7)

{
  undefined2 *puVar1;
  byte bVar2;
  uint uVar3;
  BOOL16 BVar4;
  int iVar5;
  uint uVar6;
  undefined2 *puVar7;
  uint *puVar8;
  ulong uVar9;
  uint uStack4;
  
  if ((param_4 | param_2) == 0x0) {
    pass1_1000_1e61(param_1,0x4,(UINT16)PTR_LOOP_1050_0000,(UINT16)PTR_LOOP_1050_0002);
    if ((param_7 | (uint)param_6) != 0x0) {
      param_6[0x1] = 0x0;
      *param_6 = 0x0;
      return 0x0;
    }
    return 0x1;
  }
  bVar2 = (byte)PTR_LOOP_1050_000c & 0x7;
  if (((byte)PTR_LOOP_1050_000c & 0x7) != 0x0) {
    if (bVar2 == 0x1) {
      uVar3 = *(uint *)(PTR_LOOP_1050_0000 + 0x18);
      if (param_4 != 0x0) {
        return uVar3;
      }
      if (param_2 <= uVar3) {
        return 0x0;
      }
      return uVar3;
    }
    if (bVar2 != 0x2) {
      if (bVar2 != 0x3) {
        if ((param_7 | (uint)param_6) != 0x0) {
          param_6[0x1] = 0x0;
          *param_6 = 0x0;
          return 0x0;
        }
        return 0x1;
      }
      if ((((param_7 | (uint)param_6) != 0x0) && (param_4 == 0x0)) && (param_2 <= *(uint *)(PTR_LOOP_1050_0000 + 0x1c)))
      {
        uVar9 = pass1_1000_1284(CONCAT22(0x1050,param_3),param_1);
        if (CONCAT22(param_4,param_2) < uVar9) {
          return param_2;
        }
        return (uint)uVar9;
      }
      iVar5 = mem_1000_0670(param_5,(int *)CONCAT22(param_7,param_6),param_2,(ulong *)0x0,param_4,param_1);
      if (iVar5 != 0x0) {
        return 0x0;
      }
      if ((param_7 | (uint)param_6) != 0x0) {
        return 0x0;
      }
      return 0x1;
    }
  }
  puVar8 = (uint *)(param_3 + -0x2);
  uVar3 = *puVar8 & 0x7ffc;
  uStack4 = uVar3 - 0x2;
  if ((*(byte *)(param_3 + -0x1) & 0x80) != 0x0) {
    uStack4 = uVar3 - 0x6;
  }
  if ((((param_4 == 0x0) && (param_2 <= uStack4)) ||
      ((param_4 == 0x0 && (param_2 <= *(uint *)(PTR_LOOP_1050_0000 + 0x1c))))) &&
     (BVar4 = pass1_1000_115c(param_2,puVar8), BVar4 != 0x0)) {
    if ((param_5 & 0x1) != 0x0) {
      uVar3 = (*puVar8 & 0x7ffc) - 0x2;
      if (uStack4 < param_2) {
        puVar7 = (undefined2 *)(uStack4 + param_3);
        iVar5 = -uStack4;
      }
      else {
        if (uVar3 <= param_2) {
          return 0x0;
        }
        puVar7 = (undefined2 *)(param_2 + param_3);
        iVar5 = -param_2;
      }
      uVar3 = uVar3 + iVar5;
      for (uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar1 = 0x0;
      }
      if ((uVar3 & 0x1) != 0x0) {
        *(undefined *)puVar7 = 0x0;
      }
    }
    return 0x0;
  }
  return uStack4;
}



BOOL16 __cdecl16near pass1_1000_115c(int param_1,uint *param_2)

{
  byte *pbVar1;
  uint *puVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  int iVar6;
  uint uStack4;
  
  uVar3 = *param_2 & 0x7ffc;
  uVar4 = param_1 + 0x5U & 0xfffc;
  uVar4 = (uVar4 - 0x8 & ~-(uint)(uVar4 < 0x8)) + 0x8;
  if (uVar3 < uVar4) {
    puVar5 = (uint *)(uVar3 + (int)param_2);
    if (((*(byte *)puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
      return 0x0;
    }
    if (puVar5 == (uint *)PTR_LOOP_1050_000e) {
      PTR_LOOP_1050_000e = (undefined *)puVar5[0x1];
    }
    *(uint *)(puVar5[0x2] + 0x2) = puVar5[0x1];
    *(uint *)(puVar5[0x1] + 0x4) = puVar5[0x2];
    uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      puVar2 = param_2;
      *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
      pbVar1 = (byte *)((int)puVar5 + (*puVar5 & 0xfffc));
      *pbVar1 = *pbVar1 | 0x2;
      return 0x1;
    }
  }
  else {
    uStack4 = uVar3 - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      return 0x1;
    }
    puVar5 = (uint *)(uVar3 + (int)param_2);
    if ((*(byte *)puVar5 & 0x1) == 0x0) {
      uStack4 = uStack4 + (*puVar5 & 0xfffc);
      if (puVar5 == (uint *)PTR_LOOP_1050_000e) {
        PTR_LOOP_1050_000e = (undefined *)puVar5[0x1];
      }
      *(uint *)(puVar5[0x2] + 0x2) = puVar5[0x1];
      *(uint *)(puVar5[0x1] + 0x4) = puVar5[0x2];
    }
    if (*DAT_1050_0004 < uStack4) {
      *DAT_1050_0004 = uStack4;
    }
  }
  *param_2 = *param_2 & 0x8003 | uVar4;
  *(uint *)(uVar4 + (int)param_2) = uStack4 | 0x2;
  iVar6 = uVar4 + (int)param_2;
  *(undefined2 *)(iVar6 + 0x4) = PTR_LOOP_1050_0010;
  *(undefined2 *)(iVar6 + 0x2) = *(undefined2 *)(PTR_LOOP_1050_0010 + 0x2);
  *(int *)(*(int *)(PTR_LOOP_1050_0010 + 0x2) + 0x4) = iVar6;
  *(int *)(PTR_LOOP_1050_0010 + 0x2) = iVar6;
  *(uint *)((byte *)(iVar6 + uStack4) + -0x2) = uStack4;
  pbVar1 = (byte *)(iVar6 + uStack4);
  *pbVar1 = *pbVar1 & 0xfd;
  return 0x1;
}



ulong __stdcall16far pass1_1000_1284(ulong param_1,UINT16 param_2)

{
  byte bVar1;
  uint uVar2;
  undefined4 uVar3;
  byte bVar4;
  uint uVar5;
  bool bVar6;
  DWORD DVar7;
  uint uStack6;
  int iStack4;
  
  if ((*(uint *)&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(param_2,0xe,0x0,0x0);
    return 0xffffffff;
  }
  bVar1 = *(byte *)&PTR_LOOP_1050_000c;
  bVar4 = bVar1 & 0x7;
  if ((bVar1 & 0x7) != 0x0) {
    if (bVar4 == 0x1) {
      uVar3 = *(undefined4 *)0x0;
      return (ulong)*(uint *)((int)uVar3 + 0x18);
    }
    if (bVar4 != 0x2) {
      if (bVar4 != 0x3) {
        return 0xffffffff;
      }
      DVar7 = mem_op_1000_1532(param_2);
      return CONCAT22((int)(DVar7 >> 0x10) - (uint)((uint)DVar7 < 0x14),(uint)DVar7 - 0x14);
    }
  }
  uVar2 = *(uint *)((int)param_1 + -0x2);
  uVar5 = uVar2 & 0x7ffc;
  uStack6 = uVar5 - 0x2;
  iStack4 = 0x0;
  if ((uVar2 & 0x8000) != 0x0) {
    bVar6 = uStack6 < 0x4;
    uStack6 = uVar5 - 0x6;
    iStack4 = -(uint)bVar6;
  }
  return CONCAT22(iStack4,uStack6);
}



void __stdcall16far mem_op_1000_131c(uint param_1,uint param_2,int param_3,UINT16 param_4)

{
  HGLOBAL16 HVar1;
  bool bVar2;
  long lVar3;
  UINT16 uStack10;
  UINT16 uStack8;
  int iStack6;
  
  lVar3 = CONCAT22(uStack8,uStack10);
  iStack6 = 0x1;
  if (((param_1 & 0x1000) != 0x0) && ((param_3 != 0x0 || (0xfff0 < param_2)))) {
    param_2 = 0xfff0;
    param_3 = 0x0;
  }
  if ((param_1 & 0x4) != 0x0) {
    lVar3 = mem_op_1000_1558(param_2,param_3,param_4);
  }
  do {
    HVar1 = GLobalAlloc16(param_4,CONCAT22(param_3,param_2));
    uStack10 = (UINT16)lVar3;
    if (HVar1 != 0x0) break;
    bVar2 = iStack6 != 0x0;
    iStack6 = iStack6 + -0x1;
    param_4 = (UINT16)s_tile2_bmp_1050_1538;
  } while (bVar2);
  if ((param_1 & 0x4) != 0x0) {
    if (HVar1 != 0x0) {
      GlobalPageLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
    pass1_1000_15ce((uint *)uStack10,(uint)((ulong)lVar3 >> 0x10),(int)s_tile2_bmp_1050_1538);
  }
  if (HVar1 != 0x0) {
    WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    return;
  }
  return;
}



long __stdcall16far mem_op_1000_13ce(WORD param_1)

{
  HGLOBAL16 HVar1;
  uint uVar2;
  DWORD DVar3;
  
  DVar3 = GlobalHandle16(param_1);
  uVar2 = (uint)(DVar3 >> 0x10);
  if ((int)DVar3 != 0x0) {
    HVar1 = GlobalFree16((HGLOBAL16)s_tile2_bmp_1050_1538);
    return CONCAT22(uVar2,(uint)(HVar1 == 0x0));
  }
  return (long)((ulong)uVar2 << 0x10);
}



void __stdcall16far mem_op_1000_1408(uint param_1,uint param_2,UINT16 param_3,WORD param_4)

{
  HGLOBAL16 HVar1;
  bool bVar2;
  DWORD DVar3;
  int iStack12;
  uint uStack8;
  
  DVar3 = GlobalHandle16(param_4);
  uStack8 = 0x32;
  iStack12 = 0x1;
  if (((param_1 & 0x1000) != 0x0) && ((param_3 != 0x0 || (0xfff0 < param_2)))) {
    param_2 = 0xfff0;
    param_3 = 0x0;
  }
  if ((param_1 & 0x100) != 0x0) {
    uStack8 = 0x72;
  }
  if ((param_1 & 0x804) != 0x0) {
    uStack8 = uStack8 & 0xfffd;
  }
  if ((int)DVar3 != 0x0) {
    if ((param_1 & 0x4) != 0x0) {
      GlobalPageUnlock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
    do {
      HVar1 = GlobalReAlloc16((HGLOBAL16)s_tile2_bmp_1050_1538,CONCAT22(param_2,uStack8),param_3);
      if (HVar1 != 0x0) break;
      uStack8 = uStack8 & 0xffcf;
      bVar2 = iStack12 != 0x0;
      iStack12 = iStack12 + -0x1;
    } while (bVar2);
    if ((HVar1 != 0x0) && ((param_1 & 0x4) != 0x0)) {
      GlobalPageLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
    if (HVar1 != 0x0) {
      WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
      return;
    }
  }
  return;
}



BOOL16 __stdcall16far mem_op_1000_14f2(uint param_1,uint param_2,int param_3,UINT16 param_4,UINT16 param_5)

{
  uint in_AX;
  uint in_DX;
  WORD unaff_CS;
  
  if (((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1)))) {
    mem_op_1000_1408(param_1 & 0xfdff | 0x800,param_2,param_3,unaff_CS);
    if ((in_DX | in_AX) != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



DWORD __stdcall16far mem_op_1000_1532(WORD param_1)

{
  DWORD DVar1;
  
  DVar1 = GlobalHandle16(param_1);
  if ((int)DVar1 != 0x0) {
    DVar1 = GlobalSize16((HGLOBAL16)s_tile2_bmp_1050_1538);
    return DVar1;
  }
  return 0x0;
}



long __cdecl16near mem_op_1000_1558(uint param_1,uint param_2,UINT16 param_3)

{
  uint uVar1;
  DWORD DVar2;
  uint uStack12;
  uint uStack10;
  uint uStack8;
  
  uStack12 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x8;
  if ((param_2 < 0x9) && ((param_2 < 0x8 || (param_1 == 0x0)))) {
    do {
      while( true ) {
        DVar2 = CONCAT22(uStack10,param_3);
        param_3 = (UINT16)s_tile2_bmp_1050_1538;
        DVar2 = GlobalDOSAlloc16(DVar2);
        uVar1 = (uint)DVar2;
        if (uVar1 == 0x0) break;
        *(undefined2 *)0x0 = 0x0;
        *(uint *)&PTR_LOOP_1050_0002 = uStack12;
        uStack12 = uVar1;
      }
      uVar1 = uStack8 & 0x1;
      uStack8 = uStack8 >> 0x1;
      uStack10 = uStack10 >> 0x1 | (uint)(uVar1 != 0x0) << 0xf;
    } while ((param_2 < uStack8) || ((param_2 <= uStack8 && (param_1 <= uStack10))));
  }
  return (long)((ulong)uStack12 << 0x10);
}



void pass1_1000_15ce(uint *param_1,uint param_2,WORD param_3)

{
  uint *puVar1;
  uint uVar2;
  
  uVar2 = param_2 | (uint)param_1;
  while (uVar2 != 0x0) {
    puVar1 = (uint *)*param_1;
    param_2 = param_1[0x1];
    GlobalDOSFree16(param_3);
    param_1 = puVar1;
    param_3 = (WORD)s_tile2_bmp_1050_1538;
    uVar2 = param_2 | (uint)puVar1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar * __cdecl16far mem_op_1000_160a(ushort param_1,ushort param_2)

{
  uchar *puVar1;
  
  puVar1 = (uchar *)ret_true_1000_2146();
  if (puVar1 == (uchar *)0x0) {
    return puVar1;
  }
  if (((uint)PTR_LOOP_1050_5f2e | (uint)PTR_LOOP_1050_5f2c) == 0x0) {
    DAT_1050_5f30 = 0x1;
    DAT_1050_5f32 = 0x1;
    _PTR_LOOP_1050_5f2c = mem_op_1000_18ec(DAT_1050_5f46,param_1,param_2);
    if (_PTR_LOOP_1050_5f2c != 0x0) {
      if (PTR_LOOP_1050_5f42 != (undefined *)0x0) {
        pass1_1000_1a54((ushort)PTR_LOOP_1050_5f42,(int)_PTR_LOOP_1050_5f2c,(ushort)(_PTR_LOOP_1050_5f2c >> 0x10),
                        param_2);
      }
      PTR_LOOP_1050_5f2e = (undefined *)(_PTR_LOOP_1050_5f2c >> 0x10);
      if (DAT_1050_5f44 != 0xffff) {
        pass1_1000_1afe(DAT_1050_5f44,(UINT32)PTR_LOOP_1050_5f2c,(UINT16)PTR_LOOP_1050_5f2e);
      }
    }
  }
  empty_fn_1000_214a();
  return PTR_LOOP_1050_5f2c;
}



ushort __cdecl16far mem_1000_167a(uint param_1,ushort param_2,uint param_3)

{
  uchar *puVar1;
  long lVar2;
  
  if (((uint)PTR_LOOP_1050_5f2e | (uint)PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(param_3,param_2);
    if ((param_3 | (uint)puVar1) == 0x0) {
      return 0x0;
    }
  }
  lVar2 = mem_op_1000_0a48(0x0,param_1,0x0,CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c),param_2);
  return (ushort)lVar2;
}



ushort __cdecl16far
pass1_1000_16aa(ushort *param_1,uint param_2,uint param_3,uint param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  long lVar2;
  
  if ((param_2 | (uint)param_1) == 0x0) {
    uVar1 = mem_1000_167a(param_3,param_5,param_4);
    return uVar1;
  }
  if (param_3 == 0x0) {
    pass1_1000_16ee((uint)param_1,param_2,param_5);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(param_5,param_6,0x0,param_3,0x0,param_1,param_2);
  return (ushort)lVar2;
}



void __cdecl16far pass1_1000_16ee(uint param_1,uint param_2,u16 param_3)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}



ushort __cdecl16far fn_ptr_op_1000_1708(uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,ushort param_6)

{
  int iVar1;
  bool bVar2;
  long lVar3;
  
  if ((param_2 | param_1) == 0x0) {
    bVar2 = 0xfffe < param_1;
    param_1 = param_1 + 0x1;
    param_2 = param_2 + bVar2;
  }
LAB_1000_1724:
  do {
    if ((param_5 | param_4) != 0x0) {
      lVar3 = mem_op_1000_0a48((byte)param_3,param_1,param_2,CONCAT22(param_5,param_4),param_6);
      if (lVar3 != 0x0) {
        return (ushort)lVar3;
      }
    }
    if (((param_3 & 0x8000) == 0x0) || (((uint)PTR_LOOP_1050_5f3a | (uint)PTR_LOOP_1050_5f38) == 0x0)) {
      if (((uint)PTR_LOOP_1050_5f36 | (uint)PTR_LOOP_1050_5f34) == 0x0) {
        if (((uint)PTR_LOOP_1050_5f3e | (uint)PTR_LOOP_1050_5f3c) == 0x0) {
          return 0x0;
        }
        (*(code *)PTR_LOOP_1050_5f3c)();
        goto LAB_1000_1724;
      }
      iVar1 = (*(code *)PTR_LOOP_1050_5f34)();
    }
    else {
      iVar1 = (*(code *)PTR_LOOP_1050_5f38)(param_6,param_1);
    }
    if (iVar1 == 0x0) {
      return 0x0;
    }
  } while( true );
}



void mem_op_1000_179c(ushort param_1,uchar *param_2,ushort param_3)

{
  uchar *puVar1;
  uchar *puVar2;
  
  puVar1 = PTR_LOOP_1050_5f2c;
  puVar2 = PTR_LOOP_1050_5f2e;
  if (((uint)PTR_LOOP_1050_5f2e | (uint)PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a((ushort)param_2,param_3);
    puVar2 = param_2;
  }
  fn_ptr_op_1000_1708(param_1,0x0,0x0,(uint)puVar1,(uint)puVar2,param_3);
  return;
}



void __cdecl16far fn_ptr_1000_17ce(astruct_18 *param_1,ushort param_2)

{
  if (param_1 != (astruct_18 *)0x0) {
    call_fn_ptr_1000_0dc6((u16)param_1,param_1._2_2_,param_2);
  }
  return;
}



uchar * __cdecl16far pass1_1000_17e8(uchar *param_1,uchar *param_2)

{
  uchar *puVar1;
  
  puVar1 = PTR_LOOP_1050_5f34;
  PTR_LOOP_1050_5f34 = param_1;
  PTR_LOOP_1050_5f36 = param_2;
  return puVar1;
}



ushort __cdecl16far pass1_1000_180c(uint param_1,uint param_2,ushort param_3)

{
  uchar *puVar1;
  long lVar2;
  
  if (((uint)PTR_LOOP_1050_5f2e | (uint)PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(param_2,param_3);
    if ((param_2 | (uint)puVar1) == 0x0) {
      return 0x0;
    }
  }
  lVar2 = mem_op_1000_0a48(0x0,param_1,0x0,CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c),param_3);
  return (ushort)lVar2;
}



ushort __cdecl16far pass1_1000_183c(uint param_1,uint param_2,ushort param_3)

{
  undefined *puVar1;
  long lVar2;
  
  puVar1 = (undefined *)0x0;
  if ((int)((ulong)param_2 * (ulong)param_1 >> 0x10) != 0x0) {
    return 0x0;
  }
  if ((((uint)PTR_LOOP_1050_5f2e | (uint)PTR_LOOP_1050_5f2c) == 0x0) &&
     (PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0,param_3), PTR_LOOP_1050_5f2e = puVar1,
     ((uint)puVar1 | (uint)PTR_LOOP_1050_5f2c) == 0x0)) {
    return 0x0;
  }
  lVar2 = mem_op_1000_0a48(0x1,(uint)((ulong)param_2 * (ulong)param_1),0x0,
                           CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c),param_3);
  return (ushort)lVar2;
}



ushort __cdecl16far
pass1_1000_188e(ushort *param_1,uint param_2,uint param_3,uint param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  long lVar2;
  
  if ((param_2 | (uint)param_1) == 0x0) {
    uVar1 = pass1_1000_180c(param_3,param_4,param_5);
    return uVar1;
  }
  if (param_3 == 0x0) {
    pass1_1000_18d2((uint)param_1,param_2,param_5);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(param_5,param_6,0x0,param_3,0x0,param_1,param_2);
  return (ushort)lVar2;
}



void __cdecl16far pass1_1000_18d2(uint param_1,uint param_2,ushort param_3)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}



ulong __stdcall16far mem_op_1000_18ec(ushort param_1,ushort param_2,UINT16 param_3)

{
  ulong uVar1;
  
  uVar1 = mem_op_1000_1902(param_1,0x0,0x0,0xc,param_3,param_2);
  return uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far
mem_op_1000_1902(uint param_1,uint param_2,uint param_3,UINT16 param_4,UINT16 param_5,UINT16 param_6)

{
  UINT16 *pUVar1;
  UINT16 UVar2;
  BOOL16 BVar3;
  int iVar4;
  UINT16 uVar3;
  UINT16 UVar5;
  UINT16 *pUVar6;
  DWORD DVar7;
  ulong uVar8;
  UINT16 *puVar1;
  
  UVar5 = param_6;
  if (((param_1 & 0x8000) != 0x0) && (UVar5 = param_6, _SHI_INVOKEERRORHANDLER1 != -0x6f70)) {
    param_1 = param_1 | 0x1;
    UVar5 = param_6;
  }
  do {
    uVar3 = UVar5;
    pUVar1 = (UINT16 *)(param_1 & 0xfffb | 0x1000);
    mem_op_1000_131c((uint)pUVar1,0x100,0x0,param_5);
    UVar5 = uVar3 | (uint)pUVar1;
    if (UVar5 != 0x0) break;
    UVar2 = pass1_1000_1e61(param_5,0x2,0x0,0x0);
  } while (UVar2 != 0x0);
  if ((uVar3 | (uint)pUVar1) != 0x0) {
    pUVar1[0x17] = (UINT16)&PTR_PTR_1050_5f1a;
    pUVar1[0x18] = (UINT16)&USHORT_1050_1050;
    pUVar1[0x15] = (UINT16)PTR_LOOP_1050_5f1e;
    pUVar1[0x16] = (UINT16)PTR_LOOP_1050_5f20;
    pUVar6 = pUVar1;
    PTR_LOOP_1050_5f1e = (undefined *)pUVar1;
    PTR_LOOP_1050_5f20 = (undefined *)uVar3;
    for (iVar4 = 0x5; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar1 = pUVar6;
      pUVar6 = pUVar6 + 0x1;
      *puVar1 = 0x0;
    }
    pUVar1[0x5] = 0x0;
    pUVar1[0x7] = 0x0;
    pUVar1[0x6] = 0x0;
    pUVar1[0x9] = 0x0;
    pUVar1[0x8] = 0x0;
    pUVar1[0xa] = 0xbead;
    pUVar1[0xb] = param_1 & 0xfffd;
    pUVar1[0xc] = 0x0;
    pUVar1[0xd] = 0x2000;
    pUVar1[0xe] = 0x800;
    DVar7 = mem_op_1000_1532(param_5);
    pUVar1[0xf] = (UINT16)DVar7;
    pUVar1[0x10] = (UINT16)(DVar7 >> 0x10);
    pUVar1[0x12] = 0x0;
    pUVar1[0x11] = 0x0;
    pUVar1[0x13] = 0xfffe;
    pUVar1[0x14] = 0xffff;
    pUVar1[0x19] = 0x0;
    pUVar1[0x1a] = 0x0;
    pUVar1[0x20] = 0x0;
    pUVar1[0x1f] = 0x0;
    BVar3 = pass1_1000_1afe(param_4,(UINT32)pUVar1,uVar3);
    if (BVar3 != 0x0) {
      if ((param_3 | param_2) != 0x0) {
        pUVar6 = pUVar1;
        UVar5 = uVar3;
        uVar8 = pass1_1000_52be(param_2,param_3,param_4,0x0);
        pass1_1000_010c(0x1,(uint)uVar8,(uint)(uVar8 >> 0x10),(UINT32)pUVar6,UVar5,param_5);
      }
      return CONCAT22(uVar3,pUVar1);
    }
    mem_op_1000_1b9a(0x0,(UINT32)pUVar1,uVar3,param_5);
  }
  return 0x0;
}



ushort __stdcall16far pass1_1000_1a54(ushort param_1,int param_2,ushort param_3,ushort param_4)

{
  uint uVar1;
  ushort uVar2;
  
  if (*(int *)(param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_4,0xa,0x0,0x0);
    return 0x0;
  }
  uVar1 = pass1_1000_1ab0(param_1);
  if (uVar1 < *(int *)(param_2 + 0x18) + 0x14U) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = *(ushort *)(param_2 + 0x1a);
    *(uint *)(param_2 + 0x1a) = uVar1;
    *(uint *)(param_2 + 0x1c) = uVar1 >> 0x2;
  }
  return uVar2;
}



uint __cdecl16near pass1_1000_1ab0(uint param_1)

{
  uint uVar1;
  uint uVar2;
  
  if (param_1 == 0x2000) {
    return 0x2000;
  }
  if (param_1 < 0xfff0) {
    if (param_1 < 0x1001) {
      return 0x1000;
    }
    uVar1 = 0x2000;
    if (param_1 < 0x2001) {
      do {
        uVar2 = uVar1;
        uVar1 = uVar2 >> 0x1;
      } while (param_1 <= uVar1);
      return uVar2 & 0xfffe;
    }
    while (uVar1 = uVar1 * 0x2, uVar1 != 0x0) {
      if (param_1 <= uVar1) {
        return (uVar1 + 0x10 & -(uint)(uVar1 < 0xfff0)) - 0x10;
      }
    }
  }
  return 0xfff0;
}



BOOL16 __stdcall16far pass1_1000_1afe(UINT16 param_1,UINT32 param_2,UINT16 param_3)

{
  uint uVar1;
  UINT16 unaff_CS;
  
  if (param_1 == 0x0) {
    uVar1 = 0x0;
  }
  else {
    uVar1 = param_1 + 0x1 & 0xfffe;
  }
  if (*(int *)(param_2 + 0x14) == -0x4153) {
    if ((uVar1 < param_1) || (*(int *)(param_2 + 0x1a) - 0x14U < uVar1)) {
      pass1_1000_1e61(unaff_CS,0x3,param_2,param_3);
    }
    else {
      if (*(int *)(param_2 + 0x2) == 0x0) {
        *(uint *)(param_2 + 0x18) = uVar1;
        return 0x1;
      }
    }
    return 0x0;
  }
  pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
  return 0x0;
}