
void __cdecl16far pass1_1000_27d6(uint16_t *param_1)

{
  int *piVar1;
  char *pcVar2;
  uint16_t *puVar3;
  int *piVar4;
  char cVar5;
  SEGPTR SVar6;
  uint16_t *puVar7;
  uint16_t **ppuVar8;
  int iVar9;
  undefined2 uVar10;
  uint16_t *puVar11;
  int iVar12;
  int *piVar13;
  int *piVar14;
  char *pcVar15;
  int *piVar16;
  bool bVar17;
  uint16_t *puVar18;
  
  SVar6 = GetDOSEnvironment16();
  if (SVar6 != 0x0) {
    param_1 = (uint16_t *)0x0;
  }
  iVar12 = 0x0;
  pcVar15 = (char *)0x0;
  iVar9 = -0x1;
  if (param_1 != (uint16_t *)0x0) {
    cVar5 = *(char *)0x0;
    while (cVar5 != '\0') {
      do {
        if (iVar9 == 0x0) break;
        iVar9 = iVar9 + -0x1;
        pcVar2 = pcVar15;
        pcVar15 = pcVar15 + 0x1;
      } while (*pcVar2 != '\0');
      iVar12 = iVar12 + 0x1;
      pcVar2 = pcVar15;
      pcVar15 = pcVar15 + 0x1;
      cVar5 = *pcVar2;
    }
  }
  uVar10 = 0x9;
  puVar11 = param_1;
  puVar7 = pass1_1000_2950(0x9,param_1,param_1,(int)s_tile2_bmp_1050_1538);
  puVar18 = puVar11;
  ppuVar8 = (uint16_t **)pass1_1000_2950(uVar10,puVar11,param_1,(int)s_tile2_bmp_1050_1538);
  piVar13 = (int *)0x0;
  PTR_LOOP_1050_5fbe = (undefined *)ppuVar8;
  PTR_LOOP_1050_5fc0 = (undefined *)puVar11;
  do {
    if (iVar12 == 0x0) {
      *ppuVar8 = (uint16_t *)0x0;
      ppuVar8[0x1] = (uint16_t *)0x0;
      return;
    }
    bVar17 = *piVar13 == s__C_FILE_INFO__1050_5f5c._0_2_;
    if (bVar17) {
      piVar16 = (int *)s__C_FILE_INFO__1050_5f5c;
      iVar9 = 0x6;
      piVar14 = piVar13;
      do {
        if (iVar9 == 0x0) break;
        iVar9 = iVar9 + -0x1;
        piVar4 = piVar16;
        piVar16 = piVar16 + 0x1;
        piVar1 = piVar14;
        piVar14 = piVar14 + 0x1;
        bVar17 = *piVar1 == *piVar4;
      } while (bVar17);
      if (!bVar17) goto LAB_1000_2867;
    }
    else {
LAB_1000_2867:
      *ppuVar8 = puVar7;
      ppuVar8[0x1] = puVar18;
      ppuVar8 = ppuVar8 + 0x2;
    }
    do {
      piVar1 = piVar13;
      piVar13 = (int *)((int)piVar13 + 0x1);
      cVar5 = *(char *)piVar1;
      puVar3 = puVar7;
      puVar7 = (uint16_t *)((int)puVar7 + 0x1);
      *(char *)puVar3 = cVar5;
    } while (cVar5 != '\0');
    iVar12 = iVar12 + -0x1;
  } while( true );
}



uint16_t * __cdecl16near pass1_1000_2950(int param_1,uint param_2,uint16_t param_3,ushort param_4)

{
  uint16_t *puVar1;
  uint16_t uVar2;
  char *pcVar3;
  undefined *puVar4;
  LPCSTR str;
  int iVar5;
  uint16_t *puVar6;
  uint in_AX;
  uint16_t *puVar7;
  char *pcVar8;
  uint16_t uVar9;
  
  puVar4 = PTR_LOOP_1050_6066;
  PTR_LOOP_1050_6066 = (undefined *)&PTR_LOOP_1050_1000;
  uVar9 = param_3;
  puVar7 = (uint16_t *)mem_1000_167a(in_AX,param_4,param_2);
  PTR_LOOP_1050_6066 = puVar4;
  if ((param_2 | (uint)puVar7) != 0x0) {
    return puVar7;
  }
  iVar5 = param_1;
  pass1_1000_25a8(param_3,param_4);
  pass1_1000_2913(param_1,param_3,param_4);
  str = poss_str_op_1000_28dc(iVar5);
  if (str != (PCHAR)0x0) {
    iVar5 = 0x9;
    if (*str == 'M') {
      iVar5 = 0xf;
    }
    str = str + iVar5;
    iVar5 = 0x22;
    pcVar8 = str;
    do {
      if (iVar5 == 0x0) break;
      iVar5 = iVar5 + -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16(param_4,str);
  FatalExit();
  puVar7 = (uint16_t *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar7;
    puVar7 = puVar7 + 0x1;
    uVar2 = *puVar1;
    puVar6 = puVar7;
    if ((uVar2 == uVar9) || (puVar6 = (uint16_t *)(uVar2 + 0x1), puVar6 == (uint16_t *)0x0)) {
      return puVar6;
    }
    iVar5 = -0x1;
    do {
      if (iVar5 == 0x0) break;
      iVar5 = iVar5 + -0x1;
      puVar1 = puVar7;
      puVar7 = (uint16_t *)((int)puVar7 + 0x1);
    } while (*(char *)puVar1 != '\0');
  } while( true );
}



ushort __cdecl16far
pass1_1000_2a00(ushort *param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,uchar param_6)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  ushort uVar4;
  ushort uVar5;
  undefined *puStack20;
  char local_10;
  undefined uStack15;
  undefined local_e [0x8];
  ushort uStack6;
  undefined2 local_4;
  int iStack2;
  
  iStack2 = param_2 + 0x1;
  local_4 = SUB42(&USHORT_1050_1050,0x0);
  uVar5 = 0xffff;
  if ((*(byte *)(param_1 + 0x5) & 0x40) != 0x0) {
    *(undefined *)(param_1 + 0x5) = 0x0;
    return 0xffff;
  }
  if ((*(byte *)(param_1 + 0x5) & 0x83) == 0x0) goto LAB_1000_2af2;
  uVar5 = pass1_1000_2fa4((int *)param_1,param_3,param_4,param_5,param_6);
  uStack6 = param_1[0x7a];
  pass1_1000_2cb0(param_1,param_4);
  if (DAT_1050_5f8a < (int)(uint)*(byte *)((int)param_1 + 0xb)) {
    piVar2 = pass1_1000_55b1(0x2a63,param_3,param_4);
    if ((int)piVar2 < 0x0) goto LAB_1000_2a6a;
LAB_1000_2a82:
    bVar1 = false;
  }
  else {
    iVar3 = dos3_call_op_1000_35fe((uint)*(byte *)((int)param_1 + 0xb),&iStack2);
    if (-0x1 < iVar3) goto LAB_1000_2a82;
LAB_1000_2a6a:
    bVar1 = true;
  }
  if (!bVar1) {
    if (uStack6 == 0x0) goto LAB_1000_2af2;
    unk_str_op_1000_3d3e((char *)CONCAT22(param_5,&local_10),(char *)0x10505fea);
    puStack20 = local_e;
    if (local_10 == '\\') {
      puStack20 = &uStack15;
    }
    else {
      pass1_1000_3cea(CONCAT22(param_5,&local_10),0x10505fec);
    }
    pass1_1000_3e82(uStack6,(uchar *)CONCAT22(param_5,puStack20),0xa);
    uVar4 = dos3_call_1000_514e(&iStack2);
    if (uVar4 == 0x0) goto LAB_1000_2af2;
  }
  uVar5 = 0xffff;
LAB_1000_2af2:
  *(undefined *)(param_1 + 0x5) = 0x0;
  return uVar5;
}



ushort __cdecl16far
pass1_1000_2b5c(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,int param_6,ushort param_7,
               ushort param_8)

{
  ushort uVar1;
  ushort uVar2;
  undefined in_AF;
  int iStack2;
  
  iStack2 = param_6 + 0x1;
  uVar1 = pass1_1000_2e74((uint *)param_1,param_7);
  uVar2 = sys_1000_30b4(param_1,(ushort)&USHORT_1050_1050,(byte *)CONCAT22(param_4,param_3),(int)&iStack2,param_1,
                        param_5,param_7,param_8);
  pass1_1000_2f00(uVar1,(int *)param_1,param_5,param_7,param_8,in_AF);
  return uVar2;
}


uint __cdecl16far
mem_1000_2bb6(uint param_1,int *param_2,int param_3,ushort param_4,ushort param_5,ushort param_6,uchar param_7,
             uint param_8)

{
  int *piVar1;
  int iVar2;
  int *piVar3;
  byte bVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  int iStack2;
  
  piVar3 = param_2;
  iStack2 = param_3 + 0x1;
  bVar4 = *(byte *)(param_2 + 0x5);
  if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
    param_2[0x2] = 0x0;
    if ((bVar4 & 0x1) != 0x0) {
      if ((bVar4 & 0x10) == 0x0) goto LAB_1000_2c37;
      *param_2 = param_2[0x3];
      bVar4 = bVar4 & 0xfe;
    }
    *(byte *)(param_2 + 0x5) = bVar4 & 0xef | 0x2;
    puVar7 = (uchar *)(uint)*(byte *)((int)param_2 + 0xb);
    if (((bVar4 & 0x8) == 0x0) &&
       (((bVar4 & 0x4) != 0x0 ||
        (((*(byte *)(param_2 + 0x78) & 0x1) == 0x0 &&
         (((PTR_LOOP_1050_61ec != (undefined *)0x0 &&
           (((param_2 == (int *)0x621c || (param_2 == (int *)0x6228)) && ((puVar7[0x5f90] & 0x40) != 0x0)))) ||
          (mem_1000_2ce8(param_2,param_8,param_5), (*(byte *)(piVar3 + 0x5) & 0x8) == 0x0)))))))) {
      puVar5 = mixed_dos3_call_1000_39f2
                         (puVar7,(char *)CONCAT22(param_6,&param_1),(uchar *)((int)&PTR_LOOP_1050_0000 + 0x1),param_4,
                          param_5,param_6,param_7);
      puVar6 = (uchar *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
    else {
      iVar2 = piVar3[0x3];
      puVar6 = (uchar *)(*piVar3 - iVar2);
      *piVar3 = iVar2 + 0x1;
      piVar3[0x2] = piVar3[0x79] + -0x1;
      if (puVar6 == (uchar *)0x0) {
        puVar5 = (uchar *)0x0;
        if ((puVar7[0x5f90] & 0x20) != 0x0) {
          mixed_dos3_call_1000_3636((uint)puVar7,0x0,0x0,0x2,&iStack2);
          puVar5 = (uchar *)0x0;
          puVar6 = puVar5;
        }
      }
      else {
        puVar5 = mixed_dos3_call_1000_39f2
                           (puVar7,(char *)CONCAT22(piVar3[0x4],piVar3[0x3]),puVar6,param_4,param_5,param_6,param_7);
      }
      **(undefined **)(piVar3 + 0x3) = (char)param_1;
    }
    if (puVar5 == puVar6) {
      return param_1 & 0xff;
    }
  }
LAB_1000_2c37:
  piVar1 = piVar3 + 0x5;
  *(byte *)piVar1 = *(byte *)piVar1 | 0x20;
  return 0xffff;
}


void __cdecl16near mem_1000_2ce8(int *param_1,uint param_2,ushort param_3)

{
  int *piVar1;
  ushort uVar2;
  
  uVar2 = mem_1000_167a(0x200,param_3,param_2);
  if (param_2 == 0x0) {
    piVar1 = param_1 + 0x5;
    *(byte *)piVar1 = *(byte *)piVar1 | 0x4;
    param_1[0x79] = 0x1;
    param_2 = (uint)&USHORT_1050_1050;
    uVar2 = (int)param_1 + 0xf1;
  }
  else {
    piVar1 = param_1 + 0x5;
    *(byte *)piVar1 = *(byte *)piVar1 | 0x8;
    param_1[0x79] = 0x200;
  }
  param_1[0x1] = param_2;
  *param_1 = uVar2;
  param_1[0x4] = param_2;
  param_1[0x3] = uVar2;
  param_1[0x2] = 0x0;
  return;
}



ushort * __cdecl16far
pass1_1000_2d34(ushort param_1,ushort param_2,byte *param_3,byte param_4,ushort *param_5,int param_6)

{
  byte bVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  undefined uStack14;
  byte bStack8;
  undefined uStack6;
  int iStack2;
  
  iStack2 = param_6 + 0x1;
  bStack8 = (byte)PTR_LOOP_1050_6062;
  bVar3 = false;
  bVar1 = *param_3;
  if (bVar1 == 0x77) {
    uVar4 = 0x301;
  }
  else {
    if (0x77 < bVar1) {
      return (ushort *)0x0;
    }
    if (bVar1 != 0x61) {
      if (bVar1 != 0x72) {
        return (ushort *)0x0;
      }
      uVar4 = 0x0;
      uStack6 = 0x1;
      goto LAB_1000_2d6c;
    }
    uVar4 = 0x109;
  }
  uStack6 = 0x2;
LAB_1000_2d6c:
  bVar2 = true;
LAB_1000_2d71:
  param_3 = (byte *)((ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x1));
  if ((*param_3 == 0x0) || (!bVar2)) {
    uVar4 = mixed_dos3_call_1000_370a(param_1,param_2,uVar4,param_4,0x1a4,&iStack2);
    if ((int)uVar4 < 0x0) {
      return (ushort *)0x0;
    }
    PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 0x1;
    *(undefined *)(param_5 + 0x5) = uStack6;
    param_5[0x1] = 0x0;
    *param_5 = 0x0;
    param_5[0x4] = 0x0;
    param_5[0x3] = 0x0;
    uStack14 = (undefined)uVar4;
    *(undefined *)((int)param_5 + 0xb) = uStack14;
    *(byte *)(param_5 + 0x78) = bStack8;
    param_5[0x2] = 0x0;
    param_5[0x7a] = 0x0;
    return param_5;
  }
  bVar1 = *param_3;
  if (bVar1 == 0x74) {
    if ((uVar4 & 0xc000) == 0x0) {
      uVar4 = uVar4 | 0x4000;
      goto LAB_1000_2d71;
    }
  }
  else {
    if (0x74 < bVar1) goto LAB_1000_2da4;
    if (bVar1 == 0x2b) {
      if ((uVar4 & 0x2) != 0x0) goto LAB_1000_2da4;
      uVar4 = uVar4 & 0xfffe | 0x2;
      uStack6 = 0x80;
      goto LAB_1000_2d71;
    }
    if (bVar1 == 0x62) {
      if ((uVar4 & 0xc000) == 0x0) {
        uVar4 = uVar4 | 0x8000;
        goto LAB_1000_2d71;
      }
    }
    else {
      if (bVar1 != 0x63) {
        if ((bVar1 != 0x6e) || (bVar3)) goto LAB_1000_2da4;
        bVar3 = true;
        bStack8 = bStack8 & 0xbf;
        goto LAB_1000_2d71;
      }
      if (!bVar3) {
        bVar3 = true;
        bStack8 = bStack8 | 0x40;
        goto LAB_1000_2d71;
      }
    }
  }
LAB_1000_2da4:
  bVar2 = false;
  goto LAB_1000_2d71;
}
ulong __stdcall16far mem_op_1000_1b68(ushort param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ulong uVar1;
  
  if (*(int *)(param_3 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_2,0xa,0x0,0x0);
    return (ulong)param_1 << 0x10;
  }
  uVar1 = mem_op_1000_1b9a(0x0,param_3,param_4,param_2);
  return uVar1;
}



ulong __stdcall16far mem_op_1000_1b9a(UINT16 param_1,UINT32 param_2,undefined2 param_3,UINT16 param_4)

{
  uint uVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  long lVar6;
  uint *puStack8;
  uint uStack4;
  
  *(undefined2 *)(param_2 + 0x14) = 0x0;
  uStack4 = 0x0;
  do {
    iVar5 = *(int *)(uStack4 * 0x2);
    if (iVar5 != 0x0) {
      do {
        uVar2 = *(undefined4 *)(iVar5 + 0x8);
        *(undefined2 *)((int)uVar2 + 0xc) = 0x0;
        mem_op_1000_13ce(param_4);
        iVar5 = *(int *)(iVar5 + 0x4);
      } while (*(int *)(uStack4 * 0x2) != iVar5);
    }
    uStack4 = uStack4 + 0x1;
  } while (uStack4 < 0x5);
  uVar4 = *(uint *)(param_2 + 0x12);
  uVar3 = *(uint *)(param_2 + 0x10);
  while( true ) {
    puStack8 = (uint *)CONCAT22(uVar4,uVar3);
    if ((uVar4 | uVar3) == 0x0) break;
    uVar1 = *puStack8;
    uVar4 = *(uint *)(uVar3 + 0x2);
    mem_op_1000_13ce(param_4);
    uVar3 = uVar1;
  }
  pass1_1000_20a2(param_2,param_3);
  lVar6 = mem_op_1000_13ce(param_4);
  return CONCAT22((int)((ulong)lVar6 >> 0x10),0x1);
}



BOOL16 mem_op_1000_1dfa(int param_1,byte param_2,uint param_3,uint param_4)

{
  undefined3 uVar1;
  uint uVar2;
  
  if ((param_2 & 0x4) == 0x0) {
    uVar2 = (uint)(byte)(((byte)(-(uint)((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
  }
  else {
    uVar2 = 0x1800;
  }
  if ((param_4 == 0x0) ||
     ((param_4 & 0xff00 & (uint)(byte)(((byte)(-(uint)((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8) != uVar2)
     ) {
    return 0x1;
  }
  if (param_1 != 0x0) {
    uVar1 = SegmentLimit(param_4);
    if (CARRY2(param_3,param_1 - 0x1U)) {
      return 0x1;
    }
    if ((uint)uVar1 < param_3 + (param_1 - 0x1U)) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

UINT16 pass1_1000_1e61(UINT16 param_1,UINT16 param_2,UINT16 param_3,UINT16 param_4)

{
  int iVar1;
  BOOL16 BVar2;
  UINT16 UVar3;
  UINT16 UStack64;
  UINT16 UStack62;
  UINT16 UStack60;
  code *pcStack6;
  undefined *puStack4;
  UINT16 uVar3;
  
  uVar3 = (UINT16)&USHORT_1050_1050;
  UStack62 = param_3;
  UStack60 = param_4;
  UStack64 = param_2;
  puStack4 = (undefined *)&USHORT_1050_1050;
  pcStack6 = (code *)&PTR_PTR_1050_5f1a;
  if (((uint)PTR_LOOP_1050_5f1c | (uint)PTR_PTR_1050_5f1a) == 0x0) {
    pcStack6 = (code *)0x0;
    puStack4 = (undefined *)0x0;
  }
  else {
    iVar1 = mem_op_1000_21b6((UINT16)PTR_PTR_1050_5f1a,(UINT16)PTR_LOOP_1050_5f1c);
    pcStack6 = (code *)PTR_PTR_1050_5f1a;
    puStack4 = PTR_LOOP_1050_5f1c;
    if (iVar1 == 0x0) {
      PTR_PTR_1050_5f1a = (undefined *)&PTR_PTR_1050_1f7e;
      PTR_LOOP_1050_5f1c = (undefined *)&PTR_LOOP_1050_1000;
      pcStack6 = (code *)&PTR_PTR_1050_1f7e;
      puStack4 = (undefined *)&PTR_LOOP_1050_1000;
    }
  }
  if (((uint)puStack4 | (uint)pcStack6) == 0x0) {
    return 0x0;
  }
  BVar2 = msg_box_op_1000_1f24((int)&PTR_PTR_1050_5f1a,(UINT16)&USHORT_1050_1050,0x0,0x1000);
  if (BVar2 == 0x0) {
    UVar3 = (*pcStack6)(0x1000,&UStack64);
  }
  else {
    puStack4 = (undefined *)0x0;
    pcStack6 = (code *)0x0;
    UVar3 = 0x0;
  }
  if (((uint)puStack4 | (uint)pcStack6) != 0x0) {
    pass1_1000_1f68(uVar3);
  }
  return UVar3;
}



UINT16 __stdcall16far _SHI_INVOKEERRORHANDLER1(void)

{
  int iVar1;
  BOOL16 BVar2;
  UINT16 uVar2;
  UINT16 unaff_CS;
  code *pcStack6;
  UINT8 *puStack4;
  UINT16 uVar3;
  
  uVar3 = (UINT16)&USHORT_1050_1050;
  puStack4 = (UINT8 *)&USHORT_1050_1050;
  if (((uint)PTR_LOOP_1050_5f1c | (uint)PTR_PTR_1050_5f1a) == 0x0) {
    pcStack6 = (code *)0x0;
    puStack4 = (UINT8 *)0x0;
  }
  else {
    iVar1 = mem_op_1000_21b6((UINT16)PTR_PTR_1050_5f1a,(UINT16)PTR_LOOP_1050_5f1c);
    pcStack6 = (code *)PTR_PTR_1050_5f1a;
    puStack4 = PTR_LOOP_1050_5f1c;
    if (iVar1 == 0x0) {
      PTR_PTR_1050_5f1a = (undefined *)&PTR_PTR_1050_1f7e;
      PTR_LOOP_1050_5f1c = (undefined *)&PTR_LOOP_1050_1000;
      pcStack6 = (code *)&PTR_PTR_1050_1f7e;
      puStack4 = (UINT8 *)&PTR_LOOP_1050_1000;
    }
  }
  if (((uint)puStack4 | (uint)pcStack6) != 0x0) {
    BVar2 = msg_box_op_1000_1f24((int)&PTR_PTR_1050_5f1a,(UINT16)&USHORT_1050_1050,0x0,unaff_CS);
    if (BVar2 == 0x0) {
      uVar2 = (*pcStack6)();
    }
    else {
      puStack4 = (UINT8 *)0x0;
      pcStack6 = (code *)0x0;
      uVar2 = 0x0;
    }
    if (((uint)puStack4 | (uint)pcStack6) != 0x0) {
      pass1_1000_1f68(uVar3);
    }
    return uVar2;
  }
  return 0x0;
}


void __cdecl16near pass1_1000_201c(int param_1,int param_2,UINT16 param_3)

{
  undefined2 uVar1;
  undefined4 uVar2;
  uint uVar3;
  BOOL16 BVar4;
  int iVar5;
  undefined2 uVar6;
  
  if (param_1 == 0x0) {
    *(undefined2 *)(param_2 + 0x6) = 0x0;
    *(undefined2 *)(param_2 + 0x4) = 0x0;
  }
  uVar3 = *(uint *)(param_2 + 0x6) | *(uint *)(param_2 + 0x4);
  while (uVar3 != 0x0) {
    BVar4 = pass1_1000_206c(*(uint *)(param_2 + 0x4),*(uint *)(param_2 + 0x6));
    if (BVar4 == 0x0) {
      uVar2 = *(undefined4 *)(param_2 + 0x4);
      uVar6 = (undefined2)((ulong)uVar2 >> 0x10);
      iVar5 = (int)uVar2;
      uVar1 = *(undefined2 *)(iVar5 + 0x2c);
      *(undefined2 *)(param_2 + 0x4) = *(undefined2 *)(iVar5 + 0x2a);
      *(undefined2 *)(param_2 + 0x6) = uVar1;
    }
    else {
      mem_op_1000_1b9a(0x1,*(UINT32 *)(param_2 + 0x4),*(undefined2 *)(param_2 + 0x6),param_3);
    }
    uVar3 = *(uint *)(param_2 + 0x6) | *(uint *)(param_2 + 0x4);
  }
  return;
}


ushort __stdcall16far pass1_1000_21d2(byte param_1,long param_2,uint param_3,uint param_4,undefined param_5)

{
  undefined3 uVar1;
  BOOL16 BVar2;
  
  BVar2 = mem_op_1000_1dfa(0x0,param_1,param_3,param_4);
  if (BVar2 == 0x0) {
    if ((param_1 & 0x4) == 0x0) {
      uVar1 = SegmentLimit((ulong)param_4);
      if ((bool)((byte)((uint3)uVar1 >> 0x10) & 0x1)) {
        if (param_2 == 0x0) {
          return 0x1;
        }
        if ((!CARRY4((ulong)param_3,param_2 - 0x1U)) && ((ulong)param_3 + (param_2 - 0x1U) <= (ulong)(uint)uVar1)) {
          return 0x1;
        }
      }
    }
    else {
      BVar2 = pass1_1000_22c0(param_3,param_4,(ushort)param_2,param_2._2_2_,_param_1);
      if (BVar2 != 0x0) {
        return 0x1;
      }
    }
  }
  return 0x0;
}


int * entry(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,CONTEXT *in_task_context,
           undefined2 param_7,int param_8)

{
  ushort *puVar1;
  ushort uVar2;
  char *pcVar3;
  code *pcVar4;
  ushort uVar5;
  LPCSTR str;
  ushort *puVar6;
  ushort *puVar7;
  char *pcVar8;
  CHAR *unaff_SS;
  bool bVar9;
  DWORD DVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int iVar13;
  int iVar14;
  undefined *puVar15;
  undefined2 uVar16;
  
  uVar11 = CONCAT22(param_7,PTR_LOOP_1050_5f84);
  do {
    uVar16 = 0x0;
    InitTask16(in_task_context);
    PTR_LOOP_1050_5f84 = (undefined *)uVar11;
    if ((param_8 != 0x0) &&
       (bVar9 = param_1 < (undefined *)0xff00, param_1 = param_1 + 0x100, PTR_LOOP_1050_5f7e = (undefined *)param_5,
       bVar9)) {
      PTR_LOOP_1050_5f48 = (undefined *)param_1;
      PTR_LOOP_1050_5f4a = (undefined *)param_3;
      PTR_LOOP_1050_5f4c = (undefined *)param_4;
      PTR_LOOP_1050_5f4e = (undefined *)param_2;
      PTR_LOOP_1050_5f50 = (undefined *)param_5;
      LockSegment16((HGLOBAL16)s_tile2_bmp_1050_1538);
      PTR_LOOP_1050_5f52 = (undefined *)((ulong)uVar11 >> 0x10);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      DVar10 = GetVersion16();
      PTR_LOOP_1050_5f52 = (undefined *)((ulong)uVar11 >> 0x10);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      PTR_LOOP_1050_5f80 = (undefined *)CONCAT11((char)DVar10,(char)(DVar10 >> 0x8));
      pcVar4 = (code *)swi(0x21);
      uVar12 = uVar11;
      uVar11 = (*pcVar4)(uVar16);
      PTR_LOOP_1050_5f52 = (undefined *)((ulong)uVar12 >> 0x10);
      PTR_LOOP_1050_5f84 = (undefined *)uVar12;
      _DAT_1050_5f82 = CONCAT11((char)uVar11,(char)((ulong)uVar11 >> 0x8));
      DAT_1050_5f87 = 0x0;
      WaitEvent16(0x1000);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      puVar15 = PTR_LOOP_1050_5f4c;
      param_8 = InitApp16((HINSTANCE16)s_tile2_bmp_1050_1538);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      if (param_8 != 0x0) break;
    }
    in_task_context = (CONTEXT *)s_tile2_bmp_1050_1538;
    param_8 = CONCAT11((char)((uint)param_8 >> 0x8),0xff);
    pass1_1000_24db(param_8,0x0);
    PTR_LOOP_1050_5f84 = (undefined *)uVar11;
  } while( true );
  dos3_call_1000_23ea(param_2,param_5,0x0,(ushort)unaff_SS);
  PTR_LOOP_1050_5f84 = (undefined *)uVar11;
  pass1_1000_262c((undefined *)0x238d,(undefined *)s_tile2_bmp_1050_1538,unaff_SS,(int)s_tile2_bmp_1050_1538);
  PTR_LOOP_1050_5f84 = (undefined *)uVar11;
  pass1_1000_27d6((int)((ulong)uVar11 >> 0x10));
  uVar11 = ret_op_1000_55ac(puVar15);
  uVar5 = (ushort)uVar11;
  init_1000_23be(param_1,(ushort)((ulong)uVar11 >> 0x10),(ushort)unaff_SS);
  fn_ptr_op_1000_24cd(uVar5,0x0);
  iVar14 = 0x15;
  iVar13 = 0x15;
  pass1_1000_25a8(param_5,(int)s_tile2_bmp_1050_1538);
  pass1_1000_2913(iVar13,param_5,(uint16_t)s_tile2_bmp_1050_1538);
  str = poss_str_op_1000_28dc(iVar14);
  if (str != (PCHAR)0x0) {
    iVar13 = 0x9;
    if (*str == 'M') {
      iVar13 = 0xf;
    }
    str = str + iVar13;
    iVar13 = 0x22;
    pcVar8 = str;
    do {
      if (iVar13 == 0x0) break;
      iVar13 = iVar13 + -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16((UINT16)s_tile2_bmp_1050_1538,str);
  FatalExit();
  puVar7 = (ushort *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar7;
    puVar7 = puVar7 + 0x1;
    uVar2 = *puVar1;
    puVar6 = puVar7;
    if ((uVar2 == uVar5) || (puVar6 = (ushort *)(uVar2 + 0x1), puVar6 == (ushort *)0x0)) {
      return (int *)puVar6;
    }
    iVar13 = -0x1;
    do {
      if (iVar13 == 0x0) break;
      iVar13 = iVar13 + -0x1;
      puVar1 = puVar7;
      puVar7 = (ushort *)((int)puVar7 + 0x1);
    } while (*(char *)puVar1 != '\0');
  } while( true );
}


int * __cdecl16far dos3_call_1000_23ea(ushort param_1,ushort param_2,int param_3,ushort param_4)

{
  byte *pbVar1;
  byte *pbVar2;
  byte bVar3;
  int *piVar4;
  byte *pbVar5;
  char *pcVar6;
  uint16_t uVar7;
  code **ppcVar8;
  code *pcVar9;
  uint uVar10;
  byte bVar11;
  byte bVar12;
  uint16_t uVar13;
  LPCSTR str;
  int *piVar14;
  undefined2 extraout_DX;
  undefined2 uVar15;
  uint uVar16;
  byte *pbVar17;
  int *piVar18;
  byte *pbVar19;
  char *pcVar20;
  bool bVar21;
  undefined4 uVar22;
  int iVar23;
  int iVar24;
  
  pcVar9 = (code *)swi(0x21);
  (*pcVar9)(param_3 + 0x1);
  pcVar9 = (code *)swi(0x21);
  PTR_LOOP_1050_5f6a = (undefined *)param_1;
  PTR_LOOP_1050_5f6c = (undefined *)param_2;
  (*pcVar9)();
  uVar15 = extraout_DX;
  uVar13 = pass1_1000_29dc(param_4);
  uVar22 = CONCAT22(uVar15,uVar13);
  if (*(int *)&PTR_LOOP_1050_6202 != 0x0) {
    uVar7 = *(uint16_t *)&PTR_LOOP_1050_5f7e;
    bVar21 = false;
    ppcVar8 = (code **)&PTR_LOOP_1050_6200;
    (**ppcVar8)(0x1000);
    if (bVar21) {
      iVar24 = 0x2;
      iVar23 = 0x2;
      pass1_1000_25a8(uVar7,0x1000);
      pass1_1000_2913(iVar23,uVar7,0x1000);
      str = poss_str_op_1000_28dc(iVar24);
      if (str != (PCHAR)0x0) {
        iVar23 = 0x9;
        if (*str == 'M') {
          iVar23 = 0xf;
        }
        str = str + iVar23;
        iVar23 = 0x22;
        pcVar20 = str;
        do {
          if (iVar23 == 0x0) break;
          iVar23 = iVar23 + -0x1;
          pcVar6 = pcVar20;
          pcVar20 = pcVar20 + 0x1;
        } while (*pcVar6 != '\r');
        pcVar20[-0x1] = '\0';
      }
      FatalAppExit16(0x1000,str);
      FatalExit();
      piVar18 = (int *)&PTR_LOOP_1050_63fe;
      do {
        piVar4 = piVar18;
        piVar18 = piVar18 + 0x1;
        iVar23 = *piVar4;
        piVar14 = piVar18;
        if ((iVar23 == (int)&USHORT_1050_1050) || (piVar14 = (int *)(iVar23 + 0x1), piVar14 == (int *)0x0)) {
          return piVar14;
        }
        iVar23 = -0x1;
        do {
          if (iVar23 == 0x0) break;
          iVar23 = iVar23 + -0x1;
          piVar4 = piVar18;
          piVar18 = (int *)((int)piVar18 + 0x1);
        } while (*(char *)piVar4 != '\0');
      } while( true );
    }
    ppcVar8 = (code **)&PTR_LOOP_1050_6200;
    uVar22 = (**ppcVar8)(0x1000);
  }
  iVar23 = *(int *)((int)s_New_failed_in_Op__Op_1050_0020 + 0xc);
  piVar18 = (int *)uVar22;
  if (iVar23 != 0x0) {
    pbVar19 = (byte *)0x0;
    piVar14 = (int *)uVar22;
    do {
      bVar21 = *pbVar19 == 0x0;
      piVar18 = piVar14;
      if (bVar21) break;
      iVar24 = 0xd;
      pbVar17 = (byte *)s__C_FILE_INFO__1050_5f5c;
      do {
        if (iVar24 == 0x0) break;
        iVar24 = iVar24 + -0x1;
        pbVar5 = pbVar19;
        pbVar19 = pbVar19 + 0x1;
        pbVar1 = pbVar17;
        pbVar17 = pbVar17 + 0x1;
        bVar21 = *pbVar1 == *pbVar5;
      } while (bVar21);
      if (bVar21) {
        pbVar17 = (byte *)0x5f90;
        uVar16 = (uint)((ulong)uVar22 >> 0x10);
        goto LAB_1000_2495;
      }
      iVar24 = 0x7fff;
      piVar18 = (int *)0x0;
      bVar21 = true;
      do {
        if (iVar24 == 0x0) break;
        iVar24 = iVar24 + -0x1;
        pbVar1 = pbVar19;
        pbVar19 = pbVar19 + 0x1;
        bVar21 = *pbVar1 == 0x0;
      } while (!bVar21);
      piVar14 = piVar18;
    } while (bVar21);
  }
LAB_1000_24a9:
  fn_ptr_op_1000_2594(0x620c,0x620c);
  fn_ptr_op_1000_2594(0x620c,0x620c);
  fn_ptr_op_1000_2594(0x61fe,0x61ee);
  return piVar18;
LAB_1000_2495:
  pbVar2 = pbVar19 + 0x1;
  bVar3 = *pbVar19;
  uVar10 = (uint)piVar14 & 0xff00;
  bVar11 = bVar3 + 0xbf;
  piVar18 = (int *)(uVar10 | bVar11);
  if (bVar3 < 0x41) goto LAB_1000_24a9;
  pbVar19 = pbVar19 + 0x2;
  bVar3 = *pbVar2;
  piVar14 = (int *)(uVar16 & 0xff00);
  bVar12 = bVar3 + 0xbf;
  piVar18 = (int *)((uint)piVar14 | (uint)bVar12);
  if (bVar3 < 0x41) goto LAB_1000_24a9;
  pbVar1 = pbVar17;
  pbVar17 = pbVar17 + 0x1;
  *pbVar1 = bVar12 | bVar11 * '\x10';
  uVar16 = uVar10;
  goto LAB_1000_2495;
}

void __cdecl16near dos3_op_1000_256b(void)

{
  code *pcVar1;
  
  if (PTR_LOOP_1050_6202 != (undefined *)0x0) {
    (*(code *)PTR_LOOP_1050_6200)();
  }
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}





int * exit_1000_25cc(int param_1,ushort param_2,ushort param_3)

{
  int *piVar1;
  char *pcVar2;
  LPCSTR str;
  int *piVar3;
  int *piVar4;
  char *pcVar5;
  int iVar6;
  int iVar7;
  
  iVar7 = 0x2;
  iVar6 = 0x2;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar6,param_2,param_3);
  str = poss_str_op_1000_28dc(iVar7);
  if (str != (PCHAR)0x0) {
    iVar6 = 0x9;
    if (*str == 'M') {
      iVar6 = 0xf;
    }
    str = str + iVar6;
    iVar6 = 0x22;
    pcVar5 = str;
    do {
      if (iVar6 == 0x0) break;
      iVar6 = iVar6 + -0x1;
      pcVar2 = pcVar5;
      pcVar5 = pcVar5 + 0x1;
    } while (*pcVar2 != '\r');
    pcVar5[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar4 = (int *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar4;
    piVar4 = piVar4 + 0x1;
    iVar6 = *piVar1;
    piVar3 = piVar4;
    if ((iVar6 == param_1) || (piVar3 = (int *)(iVar6 + 0x1), piVar3 == (int *)0x0)) {
      return piVar3;
    }
    iVar6 = -0x1;
    do {
      if (iVar6 == 0x0) break;
      iVar6 = iVar6 + -0x1;
      piVar1 = piVar4;
      piVar4 = (int *)((int)piVar4 + 0x1);
    } while (*(char *)piVar1 != '\0');
  } while( true );
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
