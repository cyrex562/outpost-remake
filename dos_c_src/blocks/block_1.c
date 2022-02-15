void __cdecl16near pass1_1000_0000(ushort *param_1,ushort *param_2,uint param_3)

{
  ushort *puVar1;
  ushort *puVar2;
  uint uVar3;
  
                    // Segment:    1
                    // Offset:     00000a20
                    // Length:     55b7
                    // Min Alloc:  55b7
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  for (uVar3 = param_3 >> 0x1; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
    puVar2 = param_2;
    param_2 = param_2 + 0x1;
    puVar1 = param_1;
    param_1 = param_1 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



ulong __stdcall16far mem_1000_0016(ulong param_1,UINT16 param_2)

{
  ulong uVar1;
  
  if (*(int *)((int)param_1 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_2,0xa,0x0,0x0);
    return 0xffffffff;
  }
  uVar1 = mem_op_1000_0052(0x0,param_2);
  return uVar1;
}



ulong __cdecl16near mem_op_1000_0052(ushort param_1,ushort param_2)

{
  uint *puVar1;
  uint uVar2;
  int iVar3;
  undefined4 uVar4;
  ulong uVar5;
  int iStack14;
  int iStack12;
  int iStack10;
  uint uStack8;
  
  uVar2 = *(uint *)(param_1 + 0x1e);
  iVar3 = *(int *)(param_1 + 0x20);
  uStack8 = 0x0;
  do {
    iStack10 = *(int *)(uStack8 * 0x2 + param_1);
    if ((iStack10 != 0x0) && (uStack8 != 0x3)) {
      iStack14 = 0x0;
      do {
        iStack12 = *(int *)(iStack10 + 0x4);
        uVar4 = *(undefined4 *)(iStack10 + 0x8);
        if (*(int *)((int)uVar4 + 0xa) == 0x0) {
          uVar5 = mem_op_1000_0510(0x1,0x0,param_2);
          if ((int)uVar5 == 0x0) goto LAB_1000_00f9;
          if (iStack12 == iStack10) {
            iStack12 = 0x0;
          }
        }
        else {
          if (iStack14 == 0x0) {
            iStack14 = iStack10;
          }
        }
        iStack10 = iStack12;
      } while (iStack12 != iStack14);
    }
    uStack8 = uStack8 + 0x1;
  } while (uStack8 < 0x5);
  if (*(int *)(param_1 + 0x32) != 0x0) {
    (**(code **)(param_1 + 0x32))();
  }
LAB_1000_00f9:
  puVar1 = (uint *)(param_1 + 0x1e);
  return CONCAT22((iVar3 - *(int *)(param_1 + 0x20)) - (uint)(uVar2 < *puVar1),uVar2 - *puVar1);
}



UINT32 __stdcall16far
pass1_1000_010c(int param_1,uint param_2,uint param_3,UINT32 param_4,UINT16 param_5,UINT16 param_6)

{
  uint uVar1;
  UINT32 UVar2;
  UINT16 uVar2;
  bool bVar3;
  UINT16 UVar4;
  uint uStack8;
  uint uStack6;
  uint uStack4;
  
  uStack6 = 0x0;
  uStack8 = 0x0;
  if (*(int *)(param_4 + 0x14) != -0x4153) {
    param_5 = 0x0;
    param_4 = 0x0;
    UVar4 = 0xa;
code_r0x10000128:
    pass1_1000_1e61(param_6,UVar4,param_4,param_5);
    return 0xffff;
  }
  DAT_1050_5f30 = 0x1;
  if (param_1 == 0x1) {
    uStack4 = 0x1;
    if (*(int *)(param_4 + 0x18) == 0x0) {
      UVar4 = 0x4;
      goto code_r0x10000128;
    }
  }
  else {
    if (param_1 == 0x2) {
      uStack4 = 0x2;
    }
    else {
      if (param_1 != 0x4) {
        DAT_1050_5f30 = 0x1;
        return 0xffff;
      }
      uStack4 = 0x0;
    }
  }
  while ((uStack6 <= param_3 &&
         (((uStack6 < param_3 || (uStack8 < param_2)) &&
          (uVar1 = uStack4,
          UVar2 = mem_op_1000_03c6(*(uint *)((int)s_version__d__d_1050_0012 + 0x8),0x0,uStack4,0x0,param_6,0x0,'\0'),
          (UVar2 | uVar1) != 0x0))))) {
    uVar1 = *(uint *)((int)s_version__d__d_1050_0012 + 0x8);
    bVar3 = CARRY2(uStack8,uVar1);
    uStack8 = uStack8 + uVar1;
    uStack6 = uStack6 + bVar3;
  }
  return uStack6;
}



bool __cdecl16near mem_op_1000_01b0(UINT16 param_1,UINT16 param_2)

{
  uint *puVar1;
  int *piVar2;
  BOOL16 BVar3;
  UINT16 UVar4;
  uint uVar5;
  DWORD DVar6;
  DWORD DVar7;
  ulong uVar8;
  uint uVar9;
  uint uVar10;
  uint uStack14;
  uint uStack12;
  int iStack10;
  uint uStack6;
  int iStack4;
  
  uStack14 = 0x0;
  if ((*(uint *)(param_1 + 0x40) | *(uint *)(param_1 + 0x3e)) == 0x0) {
    uVar5 = param_1 + 0x36;
    DVar6 = mem_op_1000_1532(param_2);
    DVar7 = DVar6;
  }
  else {
    DVar6 = mem_op_1000_1532(param_2);
    uVar5 = (uint)DVar6;
    if (((int)(DVar6 >> 0x10) != 0x0) || (0xffef < uVar5)) {
      pass1_1000_1e61(param_2,0x8,param_1,(UINT16)&USHORT_1050_1050);
      return false;
    }
    if (0x1fff < uVar5) {
      uVar5 = 0x2000;
    }
    while( true ) {
      uVar9 = uVar5;
      DVar7 = DVar6 + 0x18;
      if (((int)(DVar7 >> 0x10) != 0x0) || (0xfff0 < (uint)DVar7)) {
        DVar7 = 0xfff0;
      }
      BVar3 = mem_op_1000_14f2(*(uint *)(param_1 + 0x16) | 0x1000,(uint)DVar7,(int)(DVar7 >> 0x10),param_1,
                               (UINT16)&USHORT_1050_1050);
      iStack4 = (int)(DVar6 >> 0x10);
      uStack6 = (uint)DVar6;
      if (BVar3 != 0x0) break;
      uVar5 = uVar9 >> 0x1;
      if (uVar5 < 0xc) {
        UVar4 = pass1_1000_1e61(param_2,0x2,param_1,(UINT16)&USHORT_1050_1050);
        if (UVar4 == 0x0) {
          return (bool)('\x01' - (*(int *)(param_1 + 0xa) == 0x0));
        }
        DVar6 = mem_op_1000_1532(param_2);
        uVar5 = uVar9 & 0xfffe;
      }
    }
    uVar8 = pass1_1000_5390(uStack6 - 0x42,iStack4 - (uint)(uStack6 < 0x42),0xc,0x0);
    uVar5 = (int)uVar8 * 0xc + param_1 + 0x42;
  }
  puVar1 = (uint *)(param_1 + 0x1e);
  uVar9 = *puVar1;
  *puVar1 = *puVar1 - (uint)DVar6;
  piVar2 = (int *)(param_1 + 0x20);
  *piVar2 = (*piVar2 - (int)(DVar6 >> 0x10)) - (uint)(uVar9 < (uint)DVar6);
  if (uVar5 != 0x0) {
    uVar10 = 0x0;
    uVar9 = 0xc;
    DVar7 = mem_op_1000_1532(param_2);
    uVar8 = pass1_1000_5390((uint)DVar7 - 0x42,(int)(DVar7 >> 0x10) - (uint)((uint)DVar7 < 0x42),uVar9,uVar10);
    uStack14 = (int)uVar8 * 0xc + param_1 + 0x36;
  }
  iStack10 = (int)(DVar7 >> 0x10);
  uStack12 = (uint)DVar7;
  puVar1 = (uint *)(param_1 + 0x1e);
  uVar9 = *puVar1;
  *puVar1 = *puVar1 + uStack12;
  piVar2 = (int *)(param_1 + 0x20);
  *piVar2 = *piVar2 + iStack10 + (uint)CARRY2(uVar9,uStack12);
  uVar9 = *(uint *)(param_1 + 0xa);
  do {
    uVar10 = uVar5;
    *(uint *)(uVar10 + 0x4) = uVar9;
    uVar9 = uVar10;
    uVar5 = uVar10 + 0xc;
  } while (uVar10 < uStack14);
  *(uint *)(param_1 + 0xa) = uVar10;
  return true;
}



int __cdecl16near mem_op_1000_0308(int param_1,int param_2)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  undefined extraout_AH;
  int *piVar4;
  undefined2 unaff_CS;
  
  if (*(int *)(param_2 + 0xa) == 0x0) {
    bVar3 = mem_op_1000_01b0(param_2,unaff_CS);
    if (CONCAT11(extraout_AH,bVar3) == 0x0) {
      return 0x0;
    }
  }
  iVar1 = *(int *)(param_2 + 0xa);
  *(undefined2 *)(param_2 + 0xa) = *(undefined2 *)(iVar1 + 0x4);
  piVar4 = (int *)(param_1 * 0x2 + param_2);
  if (*piVar4 == 0x0) {
    *(int *)(iVar1 + 0x6) = iVar1;
    *(int *)(iVar1 + 0x4) = iVar1;
  }
  else {
    iVar2 = *piVar4;
    *(int *)(iVar1 + 0x6) = iVar2;
    *(undefined2 *)(iVar1 + 0x4) = *(undefined2 *)(iVar2 + 0x4);
    *(int *)(*(int *)(iVar2 + 0x4) + 0x6) = iVar1;
    *(int *)(iVar2 + 0x4) = iVar1;
  }
  *piVar4 = iVar1;
  return iVar1;
}



void __cdecl16near pass1_1000_0368(ushort param_1,ushort param_2,ushort param_3)

{
  ushort *puVar1;
  
  if (*(ushort *)(param_1 + 0x4) == param_1) {
    *(undefined2 *)(param_3 + param_2 * 0x2) = 0x0;
  }
  else {
    *(undefined2 *)(*(int *)(param_1 + 0x6) + 0x4) = *(undefined2 *)(param_1 + 0x4);
    *(undefined2 *)(*(int *)(param_1 + 0x4) + 0x6) = *(undefined2 *)(param_1 + 0x6);
    puVar1 = (ushort *)(param_2 * 0x2 + param_3);
    if (*puVar1 == param_1) {
      *puVar1 = *(ushort *)(param_1 + 0x4);
    }
  }
  *(undefined2 *)(param_1 + 0x4) = *(undefined2 *)(param_3 + 0xa);
  *(ushort *)(param_3 + 0xa) = param_1;
  return;
}



UINT32 mem_op_1000_03c6(uint param_1,int param_2,uint param_3,UINT32 param_4,UINT16 param_5,byte param_6,UINT8 param_7)

{
  uint *puVar1;
  int *piVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  UINT16 UVar6;
  uint uVar7;
  bool bVar8;
  DWORD DVar9;
  uint uStack20;
  UINT16 uVar9;
  
  uVar7 = CONCAT11(param_7,param_6);
  uVar3 = param_1 + 0xfff & 0xf000;
  puVar1 = (uint *)(param_4 + 0x1e);
  uVar4 = uVar3 + *puVar1;
  uVar3 = param_2 + (uint)(0xf000 < param_1) + *(int *)(param_4 + 0x20) + (uint)CARRY2(uVar3,*puVar1);
  puVar1 = (uint *)(param_4 + 0x28);
  bVar8 = uVar3 < *puVar1;
  if ((bVar8) ||
     ((bVar8 || uVar3 == *puVar1 && (puVar1 = (uint *)(param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
    if (param_3 == 0x3) {
      uStack20 = (uint)((byte)(-(uint)((param_6 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
    }
    else {
      uStack20 = 0x1000;
    }
    uStack20 = *(uint *)(param_4 + 0x16) | uStack20;
    mem_op_1000_131c(uStack20,param_1,param_2,param_5);
    if ((uVar3 | uStack20) != 0x0) {
      puVar5 = (uint *)mem_op_1000_0308(param_3,param_4);
      if (puVar5 != (uint *)0x0) {
        puVar5[0x4] = uStack20;
        puVar5[0x5] = uVar3;
        uVar9 = (UINT16)&USHORT_1050_1050;
        *(uint *)&PTR_LOOP_1050_000c = param_3 | 0xcad0;
        *(UINT32 *)0x0 = param_4;
        *(undefined2 *)&PTR_LOOP_1050_0002 = (int)&USHORT_1050_1050;
        *(uint **)&DAT_1050_0004 = puVar5;
        *(undefined2 *)((int)&DAT_1050_0004 + 0x2) = (int)&USHORT_1050_1050;
        *(undefined2 *)&PTR_LOOP_1050_000a = 0x0;
        DVar9 = mem_op_1000_1532(param_5);
        UVar6 = (UINT16)DVar9;
        if (param_3 == 0x1) {
          uVar7 = pass1_1000_0782(param_4,UVar6,0x0);
        }
        else {
          if (param_3 == 0x3) {
            pass1_1000_05b4(param_6,0x0);
          }
          else {
            uVar7 = pass1_1000_09ca(UVar6,(UINT32 *)0x0);
          }
        }
        param_2 = (int)(DVar9 >> 0x10);
        *puVar5 = uVar7;
        puVar5[0x1] = 0x8000;
        puVar1 = (uint *)(param_4 + 0x1e);
        uVar4 = *puVar1;
        *puVar1 = *puVar1 + UVar6;
        piVar2 = (int *)(param_4 + 0x20);
        *piVar2 = *piVar2 + param_2 + (uint)CARRY2(uVar4,UVar6);
        return uVar3;
      }
      mem_op_1000_13ce(param_5);
    }
  }
  else {
    pass1_1000_1e61(param_5,0x7,param_4,(UINT16)&USHORT_1050_1050);
  }
  return 0x0;
}



ulong __cdecl16near mem_op_1000_0510(ushort param_1,ushort param_2,ushort param_3)

{
  uint *puVar1;
  int *piVar2;
  byte bVar3;
  int iVar4;
  ushort uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  bool bVar11;
  DWORD DVar12;
  long lVar13;
  ushort uVar5;
  
  iVar4 = *(int *)param_2;
  uVar5 = *(ushort *)(param_2 + 0x2);
  uVar6 = *(ushort *)(param_2 + 0x4);
  bVar3 = *(byte *)(param_2 + 0xc);
  DVar12 = mem_op_1000_1532(param_3);
  uVar9 = (uint)(DVar12 >> 0x10);
  uVar8 = (uint)DVar12;
  if (param_1 != 0x0) {
    uVar7 = *(uint *)(iVar4 + 0x1e);
    uVar10 = (*(int *)(iVar4 + 0x20) - uVar9) - (uint)(uVar7 < uVar8);
    puVar1 = (uint *)(iVar4 + 0x24);
    bVar11 = uVar10 < *puVar1;
    if ((bVar11 || uVar10 == *puVar1) && ((bVar11 || (uVar7 - uVar8 < *(uint *)(iVar4 + 0x22))))) {
      bVar11 = false;
      uVar9 = uVar10;
      goto LAB_1000_0595;
    }
  }
  pass1_1000_0368(uVar6,bVar3 & 0x7,0x0);
  puVar1 = (uint *)((int)s_version__d__d_1050_0012 + 0xc);
  uVar7 = *puVar1;
  *puVar1 = *puVar1 - uVar8;
  piVar2 = (int *)s_New_failed_in_Op__Op_1050_0020;
  *piVar2 = (*piVar2 - uVar9) - (uint)(uVar7 < uVar8);
  bVar11 = true;
LAB_1000_0595:
  if (bVar11) {
    *(undefined2 *)(param_2 + 0xc) = 0x0;
    lVar13 = mem_op_1000_13ce(param_3);
    return CONCAT22((int)((ulong)lVar13 >> 0x10),0x1);
  }
  return (ulong)uVar9 << 0x10;
}



void __cdecl16near pass1_1000_05b4(byte param_1,int param_2)

{
  *(undefined2 *)(param_2 + 0xa) = 0x1;
  *(undefined2 *)(param_2 + 0x8) = 0x668;
  *(byte *)(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
  *(undefined2 *)(param_2 + 0x10) = 0x0;
  *(undefined2 *)(param_2 + 0xe) = 0x0;
  return;
}



ulong mem_op_1000_05e2(uint param_1,int param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uint *puVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  ushort UVar5;
  ulong UVar6;
  bool bVar5;
  ulong uVar6;
  
  iVar2 = param_2 + (uint)(0xffeb < param_1);
  do {
    uVar3 = 0x3;
    UVar6._0_1_ = (undefined)param_3;
    UVar6._1_1_ = (UINT8)(param_3 >> 0x8);
    UVar6._0_2_ = mem_op_1000_03c6(param_1 + 0x14,iVar2,0x3,param_4,param_5,(undefined)UVar6,UVar6._1_1_);
    if (((UINT32)UVar6 | uVar3) != 0x0) {
      return CONCAT22((UINT32)UVar6,uVar3 + 0x14);
    }
    uVar6 = mem_op_1000_0052(param_4,param_5);
    uVar3 = param_1 + 0x1013 & 0xf000;
    puVar1 = (uint *)(param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = iVar2 + (uint)(0xf000 < param_1 + 0x14) + *(int *)(param_4 + 0x20) + (uint)CARRY2(uVar3,*puVar1);
    puVar1 = (uint *)(param_4 + 0x28);
    bVar5 = uVar3 < *puVar1;
  } while (((bVar5 || uVar3 == *puVar1) &&
           ((bVar5 || (puVar1 = (uint *)(param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) &&
          ((uVar6 != 0x0 || (UVar5 = pass1_1000_1e61(param_5,0x2,param_4,(UINT16)&USHORT_1050_1050), UVar5 != 0x0))));
  return 0x0;
}



ulong __cdecl16near mem_1000_0668(ushort param_1)

{
  ulong uVar1;
  
  uVar1 = mem_op_1000_0510(0x0,0x0,param_1);
  return uVar1;
}



undefined2 mem_1000_0670(uint param_1,int *param_2,uint param_3,ulong *param_4,int param_5,WORD param_6)

{
  uint *puVar1;
  int *piVar2;
  UINT16 UVar3;
  UINT16 UVar4;
  int iVar5;
  UINT16 UVar6;
  ulong uVar7;
  uint uVar8;
  uint uVar9;
  UINT16 UVar10;
  BOOL16 BVar11;
  uint uVar12;
  uint uVar13;
  uint uVar14;
  DWORD DVar15;
  DWORD DVar16;
  
  UVar3 = *(UINT16 *)param_4;
  UVar4 = *(UINT16 *)((int)param_4 + 0x2);
  DVar15 = mem_op_1000_1532(param_6);
  UVar6 = param_5 + (uint)(0xffeb < param_3);
  uVar7 = *param_4;
  uVar8 = -(uint)((param_1 & 0x1) != 0x0) & 0x100 | -(uint)((param_1 & 0x4) != 0x0) & 0x400 |
          *(uint *)((int)uVar7 + 0x16);
  if (param_2 == (int *)0x0) {
    BVar11 = mem_op_1000_14f2(uVar8 | 0x2000,param_3 + 0x14,UVar6,(UINT16)param_4,(UINT16)&USHORT_1050_1050);
    if (BVar11 == 0x0) {
      return 0x0;
    }
  }
  else {
    iVar5 = *(int *)(param_4 + 0x1);
    uVar12 = *(uint *)((int)param_4 + 0x6);
    uVar14 = uVar12;
    do {
      uVar13 = uVar14;
      uVar9 = uVar8 | 0x2000;
      mem_op_1000_1408(uVar9,param_3 + 0x14,UVar6,param_6);
      uVar14 = uVar13 | uVar9;
      if (uVar14 != 0x0) break;
      UVar10 = pass1_1000_1e61(param_6,0x2,UVar3,UVar4);
    } while (UVar10 != 0x0);
    if ((uVar13 | uVar9) == 0x0) {
      *(undefined2 *)((int)param_2 + 0x2) = 0x0;
      *param_2 = 0x0;
      return 0x0;
    }
    *(uint *)(iVar5 + 0x8) = uVar9;
    *(uint *)(iVar5 + 0xa) = uVar13;
    *param_2 = uVar9 + 0x14;
    *(uint *)((int)param_2 + 0x2) = uVar13;
  }
  DVar16 = mem_op_1000_1532(param_6);
  uVar12 = (uint)(DVar16 - DVar15);
  puVar1 = (uint *)(UVar3 + 0x1e);
  uVar8 = *puVar1;
  *puVar1 = *puVar1 + uVar12;
  piVar2 = (int *)(UVar3 + 0x20);
  *piVar2 = *piVar2 + (int)(DVar16 - DVar15 >> 0x10) + (uint)CARRY2(uVar8,uVar12);
  return 0x1;
}



UINT16 pass1_1000_0782(UINT32 param_1,UINT16 param_2,int param_3)

{
  undefined2 in_stack_00000004;
  
  *(undefined2 *)(param_3 + 0xe) = 0x0;
  *(int *)(param_3 + 0x10) = param_3 + 0x14;
  *(undefined2 *)(param_3 + 0x8) = 0x9a0;
  pass1_1000_07ac(*(undefined2 *)(param_1 + 0x18),param_2,param_3);
  return 0x1;
}



void __cdecl16near pass1_1000_07ac(uint param_1,int param_2,int param_3)

{
  undefined2 *puVar1;
  int iVar2;
  uint uVar3;
  
  puVar1 = *(undefined2 **)(param_3 + 0x10);
  *(undefined2 **)(param_3 + 0xe) = puVar1;
  uVar3 = param_2 + (param_3 - (int)puVar1);
  iVar2 = (int)puVar1 + (uVar3 - uVar3 % param_1);
  *(int *)(param_3 + 0x10) = iVar2;
  while (puVar1 < (undefined2 *)(iVar2 - param_1)) {
    *puVar1 = (undefined2 *)((int)puVar1 + param_1);
    puVar1 = (undefined2 *)((int)puVar1 + param_1);
  }
  *puVar1 = 0x0;
  return;
}



astruct_99 * __stdcall16far pass1_1000_07fc(ushort param_1,ulong param_2)

{
  astruct_99 *paVar1;
  
  if (*(int *)((int)param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_1,0xa,0x0,0x0);
    return (astruct_99 *)0x0;
  }
  paVar1 = (astruct_99 *)mem_op_1000_0838(0x0,param_1);
  return paVar1;
}



ulong mem_op_1000_0838(ushort param_1,ushort param_2)

{
  uint *puVar1;
  int *piVar2;
  int iVar3;
  undefined2 *puVar4;
  uint uVar5;
  uint uVar6;
  UINT16 UVar7;
  UINT32 UVar8;
  int *piVar9;
  bool bVar10;
  uint uStack6;
  int *piStack4;
  
  piVar9 = *(int **)(param_1 + 0x2);
  piStack4 = piVar9;
  if (*(int *)(param_1 + 0x2) == 0x0) goto LAB_1000_085b;
  do {
    do {
      if (*piVar9 != 0x0) {
        iVar3 = piVar9[0x5];
        puVar4 = (undefined2 *)*(int *)&PTR_LOOP_1050_000e;
        if (puVar4 != (undefined2 *)0x0) {
          *(undefined2 *)&PTR_LOOP_1050_000e = *puVar4;
          piVar2 = (int *)&PTR_LOOP_1050_000a;
          *piVar2 = *piVar2 + 0x1;
          *(int **)(param_1 + 0x2) = piVar9;
          return CONCAT22(iVar3,puVar4);
        }
        *piVar9 = 0x0;
      }
      piVar9 = (int *)piVar9[0x2];
    } while (piVar9 != piStack4);
LAB_1000_085b:
    if (*(int *)(param_1 + 0x18) == 0x0) {
      pass1_1000_1e61(param_2,0x4,param_1,(UINT16)&USHORT_1050_1050);
      return 0x0;
    }
    uVar5 = *(uint *)(param_1 + 0x1a);
    while( true ) {
      uStack6 = uVar5;
      uVar5 = 0x1;
      UVar8 = mem_op_1000_03c6(uStack6,0x0,0x1,param_1,param_2,0x0,'\0');
      if ((UVar8 | uVar5) != 0x0) break;
      uVar5 = *(uint *)(param_1 + 0x1e);
      uVar6 = uVar5 + uStack6;
      uVar5 = *(int *)(param_1 + 0x20) + (uint)CARRY2(uVar5,uStack6);
      puVar1 = (uint *)(param_1 + 0x28);
      bVar10 = *puVar1 <= uVar5;
      if (bVar10) {
        if (bVar10 && uVar5 != *puVar1) {
          return 0x0;
        }
        puVar1 = (uint *)(param_1 + 0x26);
        if (*puVar1 <= uVar6 && uVar6 != *puVar1) {
          return 0x0;
        }
      }
      uVar5 = uStack6 >> 0x1;
      if (uStack6 >> 0x1 < *(int *)(param_1 + 0x18) + 0x14U) {
        UVar7 = pass1_1000_1e61(param_2,0x2,param_1,(UINT16)&USHORT_1050_1050);
        uVar5 = uStack6 & 0xfffe;
        if (UVar7 == 0x0) {
          return 0x0;
        }
      }
    }
    piVar9 = *(int **)(param_1 + 0x2);
    piStack4 = (int *)piVar9[0x2];
  } while( true );
}



ushort __stdcall16far pass1_1000_093a(int *param_1,ushort param_2,ushort param_3)

{
  int *piVar1;
  
  if (*(int *)&PTR_LOOP_1050_000c != -0x352f) {
    pass1_1000_1e61(param_3,0xe,0x0,0x0);
    return 0x0;
  }
  *param_1 = *(int *)&PTR_LOOP_1050_000e;
  if (*param_1 == 0x0) {
    *(undefined2 *)*(undefined4 *)&DAT_1050_0004 = 0x1;
  }
  *(int **)&PTR_LOOP_1050_000e = param_1;
  piVar1 = (int *)&PTR_LOOP_1050_000a;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    mem_op_1000_0510(0x1,0x0,param_3);
  }
  return 0x1;
}



uchar * __cdecl16near pass1_1000_09a0(ushort *param_1,ushort param_2)

{
  uchar *puVar1;
  ulong uVar2;
  
  *param_1 = (ushort)PTR_LOOP_1050_000e;
  if (PTR_LOOP_1050_000e == (undefined *)0x0) {
    *DAT_1050_0004 = 0x1;
  }
  PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
  puVar1 = PTR_LOOP_1050_000e;
  PTR_LOOP_1050_000e = (undefined *)param_1;
  if (PTR_LOOP_1050_000a == (undefined *)0x0) {
    uVar2 = mem_op_1000_0510(0x1,0x0,param_2);
    puVar1 = (uchar *)uVar2;
  }
  return puVar1;
}



uint __cdecl16near pass1_1000_09ca(int param_1,UINT32 *param_2)

{
  uint *puVar1;
  int iVar2;
  undefined4 uVar3;
  UINT16 *puVar4;
  
  puVar1 = param_2 + 0xa;
  puVar4 = (UINT16 *)(((int)param_2 + (param_1 - (int)puVar1) + -0x6 & 0xfffcU) + (int)puVar1);
  *puVar4 = 0x1;
  param_2[0x7] = (UINT32)puVar1;
  puVar4[0x2] = (UINT16)puVar4;
  puVar4[0x1] = (UINT16)puVar4;
  param_2[0x8] = (UINT32)puVar4;
  if ((*(byte *)(param_2 + 0x6) & 0x7) == 0x2) {
    param_2[0x9] = 0x8;
  }
  else {
    uVar3 = *(undefined4 *)param_2;
    iVar2 = *(int *)((int)uVar3 + 0x18);
    param_2[0x9] = (iVar2 - 0x5U & ~-(uint)(iVar2 + 0x3U < 0x8)) + 0x8;
  }
  puVar4[-0x1] = (int)puVar4 - (int)puVar1;
  *puVar1 = (int)puVar4 - (int)puVar1 | 0x2;
  param_2[0xc] = (UINT32)puVar4;
  param_2[0xb] = puVar4[0x1];
  *(uint **)(puVar4[0x1] + 0x4) = puVar1;
  puVar4[0x1] = (UINT16)puVar1;
  param_2[0x4] = 0xe08;
  return *puVar1 & 0xfffc;
}



long __stdcall16far mem_op_1000_0a48(byte param_1,uint param_2,int param_3,ulong param_4,ushort param_5)

{
  uint uVar1;
  ushort *puVar2;
  uint uVar4;
  ushort uVar3;
  ushort UVar4;
  ulong uVar5;
  undefined in_stack_00000005;
  ushort *puVar1;
  
  UVar4 = (ushort)(param_4 >> 0x10);
  if (*(int *)((UINT16)param_4 + 0x14) == -0x4153) {
    if ((param_3 == 0x0) && (param_2 <= *(uint *)((int)s_version__d__d_1050_0012 + 0x6))) {
      if (param_2 == 0x0) {
        pass1_1000_1e61(param_5,0x4,(UINT16)param_4,UVar4);
        uVar5 = 0x0;
      }
      else {
        uVar5 = mem_op_1000_0838(0x0,param_5);
        uVar3 = (ushort)(uVar5 >> 0x10);
        puVar2 = (ushort *)uVar5;
        if ((uVar5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
          uVar1 = *(uint *)((int)s_version__d__d_1050_0012 + 0x6);
          for (uVar4 = uVar1 >> 0x1; uVar4 != 0x0; uVar4 = uVar4 - 0x1) {
            puVar1 = puVar2;
            puVar2 = puVar2 + 0x1;
            *puVar1 = 0x0;
          }
          if ((uVar1 & 0x1) != 0x0) {
            *(undefined *)puVar2 = 0x0;
          }
        }
      }
    }
    else {
      if ((param_3 == 0x0) && (param_2 <= *(uint *)((int)s_version__d__d_1050_0012 + 0xa))) {
        uVar5 = mem_op_1000_0b20(_param_1 & 0xfffd,0x0,param_2,param_5);
      }
      else {
        uVar5 = mem_op_1000_05e2(param_2,param_3,_param_1 & 0xfffd,0x0,param_5);
      }
    }
    return (long)uVar5;
  }
  pass1_1000_1e61(param_5,0xa,0x0,0x0);
  return 0x0;
}



ulong mem_op_1000_0b20(ushort param_1,ushort param_2,ushort param_3,ushort param_4)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  UINT32 UVar6;
  uint *puVar7;
  undefined2 uVar8;
  bool bVar9;
  ulong uVar10;
  ushort uStack20;
  uint *puStack6;
  
  uVar8 = SUB42(&USHORT_1050_1050,0x0);
  uVar2 = param_1 & 0x2;
  uVar4 = param_3 + 0x5 & 0xfffc;
  uVar4 = uVar4 - 0x8 & ~-(uint)(uVar4 < 0x8);
  uVar5 = uVar4 + 0x8;
  puVar7 = *(uint **)(uVar2 * 0x2 + param_2);
  uStack20 = param_1;
  puStack6 = puVar7;
  if (puVar7 == (uint *)0x0) goto LAB_1000_0b64;
  do {
    do {
      if ((uVar5 <= *puVar7) && (uVar10 = pass1_1000_0c32(uVar5,uStack20,0x0), uVar10 != 0x0)) {
        *(uint **)(uVar2 * 0x2 + param_2) = puVar7;
        return uVar10;
      }
      puVar7 = (uint *)puVar7[0x2];
    } while (puVar7 != puStack6);
LAB_1000_0b64:
    if ((((uStack20 & 0x2) == 0x0) || ((uStack20 & 0x40) != 0x0)) || (*(int *)(param_2 + 0x32) == 0x0)) {
LAB_1000_0b9e:
      if (((uStack20 & 0x10) != 0x0) ||
         (uVar3 = uVar2, UVar6 = mem_op_1000_03c6(*(uint *)(param_2 + 0x1a),0x0,uVar2,param_2,param_4,0x0,'\0'),
         (UVar6 | uVar3) == 0x0)) {
        if ((uStack20 & 0x20) == 0x0) {
          uVar2 = uVar4 + 0x1007 & 0xf000;
          puVar1 = (uint *)(param_2 + 0x1e);
          uVar4 = uVar2 + *puVar1;
          uVar2 = *(int *)(param_2 + 0x20) + (uint)CARRY2(uVar2,*puVar1);
          puVar1 = (uint *)(param_2 + 0x28);
          bVar9 = uVar2 < *puVar1;
          if ((bVar9 || uVar2 == *puVar1) &&
             ((bVar9 || (puVar1 = (uint *)(param_2 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
            uVar10 = mem_op_1000_05e2(uVar5,0x0,uStack20,param_2,param_4);
            return uVar10;
          }
        }
        return 0x0;
      }
    }
    else {
      param_4 = 0x1000;
      uVar3 = (**(code **)(param_2 + 0x32))();
      if (uVar3 < uVar5) goto LAB_1000_0b9e;
      uStack20 = uStack20 | 0x40;
    }
    puVar7 = *(uint **)(uVar2 * 0x2 + param_2);
    puStack6 = (uint *)puVar7[0x2];
  } while( true );
}



ulong __cdecl16near pass1_1000_0c32(ushort param_1,ushort param_2,ushort param_3)

{
  uint *puVar1;
  byte *pbVar2;
  int *piVar3;
  undefined4 uVar4;
  uint uVar5;
  uint *puVar6;
  int iVar7;
  uint *puVar8;
  undefined2 uVar9;
  ushort uStack14;
  uint *puStack8;
  uint uStack6;
  
  puVar8 = *(uint **)(param_3 + 0xe);
  uStack6 = 0x0;
  puVar6 = puVar8;
  while( true ) {
    do {
      uVar5 = *puVar6;
      if (param_1 <= uVar5) {
        uVar5 = (uVar5 & 0xfffc) - param_1;
        puVar1 = (uint *)(param_3 + 0x12);
        puStack8 = puVar6;
        if (*puVar1 < uVar5 || *puVar1 == uVar5) {
          uStack14 = param_1;
          if ((param_2 & 0x6) == 0x0) {
            puStack8 = (uint *)(uVar5 + (int)puVar6);
            puStack8[-0x1] = uVar5;
            *puVar6 = uVar5 | 0x2;
            puVar8 = (uint *)puVar6[0x1];
            pbVar2 = (byte *)((int)puStack8 + param_1);
            *pbVar2 = *pbVar2 | 0x2;
            *puStack8 = param_1 | 0x1;
          }
          else {
            *puVar6 = param_1 & 0xff00 | *(byte *)puVar6 & 0x2 | param_1 & 0xff | 0x1;
            *(uint *)(puVar6[0x2] + 0x2) = puVar6[0x1];
            *(uint *)(puVar6[0x1] + 0x4) = puVar6[0x2];
            puVar8 = (uint *)((int)puVar6 + param_1);
            *(uint *)((int)puVar8 + (uVar5 - 0x2)) = uVar5;
            *puVar8 = uVar5 | 0x2;
            uVar5 = *(uint *)(param_3 + 0x10);
            puVar8[0x2] = uVar5;
            puVar8[0x1] = *(uint *)(uVar5 + 0x2);
            *(uint **)(*(int *)(uVar5 + 0x2) + 0x4) = puVar8;
            *(uint **)(uVar5 + 0x2) = puVar8;
          }
        }
        else {
          puVar8 = (uint *)puVar6[0x1];
          *(uint **)(puVar6[0x2] + 0x2) = puVar8;
          *(uint *)(puVar6[0x1] + 0x4) = puVar6[0x2];
          puVar1 = puVar6;
          *(byte *)puVar1 = *(byte *)puVar1 | 0x1;
          uStack14 = *puVar6 & 0xfffc;
          *(uint *)((int)puVar6 + uStack14) = *(uint *)((int)puVar6 + uStack14) | 0x2;
        }
        *(uint **)(param_3 + 0xe) = puVar8;
        if ((param_2 & 0x1) != 0x0) {
          puVar6 = puStack8;
          for (uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
            *puVar6 = 0x0;
          }
          if ((uStack14 - 0x2 & 0x1) != 0x0) {
            *(undefined *)puVar6 = 0x0;
          }
        }
        if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
          **(uint **)(param_3 + 0x4) = **(uint **)(*(int *)(param_3 + 0x10) + 0x2) & 0xfffc;
          uVar4 = *(undefined4 *)(param_3 + 0x4);
          pbVar2 = (byte *)((int)uVar4 + 0x3);
          *pbVar2 = *pbVar2 | 0x80;
        }
        piVar3 = (int *)(param_3 + 0xa);
        *piVar3 = *piVar3 + 0x1;
        return CONCAT22(0x1050,puStack8 + 0x1);
      }
      if (uStack6 < uVar5) {
        uStack6 = uVar5;
      }
      puVar6 = (uint *)puVar6[0x1];
    } while (puVar6 != puVar8);
    if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) break;
    uVar4 = *(undefined4 *)param_3;
    uVar9 = (undefined2)((ulong)uVar4 >> 0x10);
    iVar7 = (int)uVar4;
    if (*(int *)(iVar7 + 0x34) == 0x0) break;
    uStack6 = (**(code **)(iVar7 + 0x34))();
    if ((uStack6 < param_1) || (puVar6 = *(uint **)(param_3 + 0xe), puVar6 == (uint *)0x0)) break;
  }
  **(uint **)(param_3 + 0x4) = uStack6 & 0xfffc;
  return 0x0;
}