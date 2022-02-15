
int __cdecl16far pass1_1000_3d7a(ulong param_1,ulong param_2)

{
  byte *pbVar1;
  char *pcVar2;
  byte *pbVar3;
  int iVar4;
  uint uVar5;
  char *pcVar6;
  byte *pbVar7;
  char *pcVar8;
  byte *pbVar9;
  undefined2 uVar10;
  bool bVar11;
  bool bVar12;
  
  pbVar7 = (byte *)param_1;
  uVar10 = (undefined2)(param_2 >> 0x10);
  pcVar8 = (char *)param_2;
  iVar4 = 0x0;
  uVar5 = 0xffff;
  do {
    if (uVar5 == 0x0) break;
    uVar5 = uVar5 - 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
  } while (*pcVar2 != '\0');
  pcVar6 = (char *)~uVar5;
  bVar11 = pcVar8 < pcVar6;
  pbVar9 = (byte *)(pcVar8 + -(int)pcVar6);
  bVar12 = pbVar9 == (byte *)0x0;
  do {
    if (pcVar6 == (char *)0x0) break;
    pcVar6 = pcVar6 + -0x1;
    pbVar3 = pbVar9;
    pbVar9 = pbVar9 + 0x1;
    pbVar1 = pbVar7;
    pbVar7 = pbVar7 + 0x1;
    bVar11 = *pbVar1 < *pbVar3;
    bVar12 = *pbVar1 == *pbVar3;
  } while (bVar12);
  if (!bVar12) {
    iVar4 = (0x1 - (uint)bVar11) - (uint)(bVar11 != 0x0);
  }
  return iVar4;
}



uint __cdecl16far str_op_1000_3da4(char *param_1)

{
  char *pcVar1;
  uint uVar2;
  char *pcVar3;
  bool bVar4;
  
  pcVar3 = (char *)param_1;
  bVar4 = true;
  uVar2 = 0xffff;
  do {
    if (uVar2 == 0x0) break;
    uVar2 = uVar2 - 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar4 = *pcVar1 == '\0';
  } while (!bVar4);
  uVar2 = ~uVar2;
  if (bVar4) {
    uVar2 = uVar2 - 0x1;
  }
  return uVar2;
}



uchar __cdecl16far str_op_1000_3dbe(char *param_1,char *param_2,ushort param_3)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  char *pcVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  pcVar4 = (char *)param_1;
  pcVar3 = (char *)param_2;
  if (param_3 != 0x0) {
    do {
      pcVar1 = pcVar3;
      pcVar3 = pcVar3 + 0x1;
      cVar2 = *pcVar1;
      if (cVar2 == '\0') break;
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = cVar2;
      param_3 = param_3 - 0x1;
    } while (param_3 != 0x0);
    for (; param_3 != 0x0; param_3 = param_3 - 0x1) {
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = '\0';
    }
  }
  return (uchar)param_1;
}



uint __cdecl16far pass1_1000_3de8(char *param_1,char *param_2,uint param_3,uint16_t param_4,uint16_t param_5)

{
  byte *pbVar1;
  char *pcVar2;
  char *pcVar3;
  byte bVar4;
  uint uVar5;
  int iVar6;
  char *pcVar7;
  char *pcVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  bool bVar11;
  
  if (param_3 != 0x0) {
    uVar9 = (undefined2)((ulong)param_1 >> 0x10);
    pcVar8 = (char *)param_1;
    uVar5 = param_3;
    pcVar7 = pcVar8;
    do {
      if (uVar5 == 0x0) break;
      uVar5 = uVar5 - 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
    } while (*pcVar2 != '\0');
    iVar6 = param_3 - uVar5;
    uVar10 = (undefined2)((ulong)param_2 >> 0x10);
    pcVar7 = (char *)param_2;
    do {
      if (iVar6 == 0x0) break;
      iVar6 = iVar6 + -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
    } while (*pcVar2 == *pcVar3);
    bVar4 = pcVar7[-0x1];
    uVar5 = 0x0;
    pbVar1 = (byte *)(pcVar8 + -0x1);
    bVar11 = bVar4 == *pbVar1;
    if (bVar4 < *pbVar1 || bVar11) {
      if (bVar11) {
        return 0x0;
      }
      uVar5 = 0xfffe;
    }
    param_3 = ~uVar5;
  }
  return param_3;
}



int __cdecl16far pass1_1000_3e2c(ulong param_1)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  int iVar4;
  byte *pbVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  pbVar5 = (byte *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (uint)(byte)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



int __cdecl16far pass1_1000_3e2c(ulong param_1)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  int iVar4;
  byte *pbVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  pbVar5 = (byte *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (uint)(byte)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



int __cdecl16far pass1_1000_3e2c(ulong param_1)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  int iVar4;
  byte *pbVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  pbVar5 = (byte *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (uint)(byte)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



byte * __cdecl16far pass1_1000_3e82(uint param_1,uchar *param_2,uint param_3)

{
  byte *pbVar1;
  ulong uVar2;
  byte bVar3;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  byte *pbVar8;
  byte *pbVar9;
  byte *pbVar10;
  byte *pbVar11;
  undefined2 uVar12;
  bool bVar13;
  char cVar4;
  
  uVar6 = 0x0;
  if (param_3 == 0xa) {
    uVar6 = (int)param_1 >> 0xf;
  }
  uVar12 = (undefined2)((ulong)param_2 >> 0x10);
  pbVar9 = (byte *)param_2;
  pbVar10 = pbVar9;
  pbVar8 = pbVar9;
  if ((param_3 == 0xa) && ((int)uVar6 < 0x0)) {
    pbVar10 = pbVar9 + 0x1;
    *param_2 = '-';
    bVar13 = param_1 != 0x0;
    param_1 = -param_1;
    uVar6 = -(uVar6 + bVar13);
    pbVar8 = pbVar10;
  }
  do {
    uVar7 = 0x0;
    uVar5 = uVar6;
    if (uVar6 != 0x0) {
      uVar5 = uVar6 / param_3;
      uVar7 = uVar6 % param_3;
    }
    uVar2 = CONCAT22(uVar7,param_1);
    param_1 = (uint)(uVar2 / param_3);
    cVar4 = (char)(uVar2 % (ulong)param_3);
    bVar3 = cVar4 + 0x30;
    if (0x39 < bVar3) {
      bVar3 = cVar4 + 0x57;
    }
    pbVar11 = pbVar10 + 0x1;
    *pbVar10 = bVar3;
    uVar6 = uVar5;
    pbVar10 = pbVar11;
  } while ((uVar5 | param_1) != 0x0);
  *pbVar11 = 0x0;
  do {
    pbVar11 = pbVar11 + -0x1;
    pbVar1 = pbVar11;
    bVar3 = *pbVar1;
    *pbVar1 = *pbVar8;
    *pbVar8 = bVar3;
    pbVar10 = pbVar8 + 0x2;
    pbVar8 = pbVar8 + 0x1;
  } while (pbVar10 < pbVar11);
  return pbVar9;
}



void __cdecl16far fatal_app_exit_1000_3e9e(UINT16 app_exit_action)

{
  FatalAppExit16(app_exit_action,(LPCSTR)s_ABNORMAL_PROGRAM_TERMINATION_1050_6544);
  return;
}



int __cdecl16far pass1_1000_3ec0(uint param_1,uint param_2)

{
  uint uVar1;
  uint uVar2;
  uint16_t uVar3;
  uint16_t unaff_SI;
  uint16_t uVar4;
  undefined4 *puVar4;
  
  puVar4 = (undefined4 *)CONCAT22(PTR_LOOP_1050_5fc0,PTR_LOOP_1050_5fbe);
  if ((((uint)PTR_LOOP_1050_5fc0 | (uint)PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0)) {
    uVar1 = str_op_1000_3da4((char *)CONCAT22(param_2,param_1));
    while( true ) {
      uVar4 = (uint16_t)((ulong)puVar4 >> 0x10);
      uVar3 = (uint16_t)puVar4;
      if ((*(uint *)(uVar3 + 0x2) | *(uint *)puVar4) == 0x0) break;
      uVar2 = str_op_1000_3da4((char *)CONCAT22(*(undefined2 *)(uVar3 + 0x2),*(uint *)puVar4));
      if (((uVar1 < uVar2) && (*(char *)((int)*puVar4 + uVar1) == '=')) &&
         (uVar2 = pass1_1000_3de8((char *)CONCAT22(*(undefined2 *)(uVar3 + 0x2),*(uint *)puVar4),
                                  (char *)CONCAT22(param_2,param_1),uVar1,unaff_SI,uVar3), uVar2 == 0x0)) {
        return *(uint *)puVar4 + uVar1 + 0x1;
      }
      puVar4 = (undefined4 *)((ulong)puVar4 & 0xffff0000 | (ulong)(uVar3 + 0x4));
    }
  }
  return 0x0;
}



int __cdecl16far pass1_1000_3f5c(int param_1,ushort param_2,ushort param_3,ushort param_4,uchar param_5)

{
  ushort uVar1;
  ushort *puVar2;
  int iVar3;
  int iStack2;
  
  iStack2 = param_1 + 0x1;
  iVar3 = 0x0;
  if (PTR_LOOP_1050_61ec == (undefined *)0x0) {
    puVar2 = (ushort *)&PTR_LOOP_1050_6210;
  }
  else {
    puVar2 = (ushort *)0x6234;
  }
  for (; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6) {
    uVar1 = pass1_1000_2a00(puVar2,(int)&iStack2,param_2,param_3,param_4,param_5);
    if (uVar1 != 0xffff) {
      iVar3 = iVar3 + 0x1;
    }
  }
  return iVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __cdecl16far
sys_1000_3f9c(uchar *param_1,uchar *param_2,ushort param_3,ushort param_4,ushort param_5,int param_6,ushort param_7,
             ushort param_8,ushort param_9,uchar param_10)

{
  undefined *puVar1;
  ushort uVar2;
  ushort local_4;
  int iStack2;
  
  iStack2 = param_6 + 0x1;
  PTR_LOOP_1050_68b2._0_1_ = 0x42;
  PTR_LOOP_1050_68ae = param_1;
  PTR_LOOP_1050_68b0 = param_2;
  _USHORT_1050_68a8 = (undefined *)CONCAT22(param_2,param_1);
  PTR_LOOP_1050_68ac = (undefined *)0x7fff;
  uVar2 = sys_1000_30b4((ushort)&USHORT_1050_68a8,(ushort)&USHORT_1050_1050,(byte *)CONCAT22(param_4,param_3),
                        (int)&iStack2,(uint)&USHORT_1050_68a8,param_7,param_8,param_9);
  puVar1 = _USHORT_1050_68a8;
  PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -0x1;
  if ((int)PTR_LOOP_1050_68ac < 0x0) {
    mem_1000_2bb6(0x0,(int *)&USHORT_1050_68a8,(int)&iStack2,param_7,param_8,param_9,param_10,(uint)param_2);
  }
  else {
    _USHORT_1050_68a8 = (undefined *)((ulong)_USHORT_1050_68a8 & 0xffff0000 | (ulong)(USHORT_1050_68a8 + 0x1));
    *puVar1 = 0x0;
  }
  return uVar2;
}



uchar * __cdecl16far pass1_1000_400a(int param_1,ushort param_2)

{
  uchar *puVar1;
  int iStack2;
  
  iStack2 = param_2 + 0x1;
  if ((param_1 < 0x0) || ((int)PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
    PTR_LOOP_1050_5f78 = (undefined *)&DAT_1050_0009;
    puVar1 = (uchar *)0xffff;
  }
  else {
    if (((PTR_LOOP_1050_61ec == (undefined *)0x0) || ((param_1 < DAT_1050_5f8a && (0x2 < param_1)))) &&
       (0x31d < CONCAT11(DAT_1050_5f83,DAT_1050_5f82))) {
      puVar1 = PTR_LOOP_1050_5f88;
      if (((*(byte *)(param_1 + 0x5f90) & 0x1) == 0x0) ||
         (puVar1 = (uchar *)dos3_call_1000_5174((ushort)&iStack2), puVar1 != (uchar *)0x0)) {
        PTR_LOOP_1050_5f88 = puVar1;
        PTR_LOOP_1050_5f78 = (undefined *)&DAT_1050_0009;
        puVar1 = (uchar *)0xffff;
      }
    }
    else {
      puVar1 = (uchar *)0x0;
    }
  }
  return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10004090)
// WARNING: Removing unreachable block (ram,0x1000409a)

void __cdecl16far free_mem_1000_407a(ushort param_1,ushort param_2,undefined2 param_3)

{
  GlobalFree16(0x1000);
  return;
}



int * __cdecl16far mixed_sys_op_1000_40af(uint param_1,int param_2,uint param_3,ushort param_4,uint16_t param_5)

{
  ushort *puVar1;
  ushort uVar2;
  char *pcVar3;
  undefined2 *puVar4;
  LPCSTR str;
  ushort *puVar5;
  uint uVar6;
  uint uVar7;
  HGLOBAL16 HVar8;
  SEGPTR SVar9;
  int iVar10;
  uint uVar11;
  ushort *puVar12;
  char *pcVar13;
  undefined2 *puVar14;
  undefined2 unaff_SS;
  bool bVar15;
  int iVar16;
  uint uVar17;
  
  do {
    uVar6 = (uint)((ulong)param_1 * (ulong)param_3);
    uVar7 = param_2 * param_3 + (int)((ulong)param_1 * (ulong)param_3 >> 0x10);
    if ((uVar7 | uVar6) != 0x0) {
      puVar12 = (ushort *)0x0;
      if ((uVar7 < 0x3) && ((uVar7 < 0x2 || (uVar6 == 0x0)))) {
        if (uVar7 == 0x0) {
          uVar6 = uVar6 + 0xfff & 0xf000;
          if (uVar6 == 0x0) {
            uVar7 = 0x1;
          }
        }
        else {
          if ((param_3 - 0x1 & param_3) != 0x0) {
            puVar12 = (ushort *)(((ulong)uVar7 << 0x10) % (ulong)param_3);
            bVar15 = CARRY2(uVar6,(uint)puVar12);
            uVar6 = uVar6 + (int)puVar12;
            if (bVar15) goto LAB_1000_41aa;
            uVar7 = 0x1;
          }
        }
      }
      else {
        if ((param_3 - 0x1 & param_3) != 0x0) goto LAB_1000_41aa;
      }
      uVar17 = 0x0;
      uVar11 = uVar7;
      HVar8 = GLobalAlloc16(0x1000,CONCAT22(uVar7,uVar6));
      if ((HVar8 != 0x0) && ((uVar17 & 0x1) != 0x0)) {
        SVar9 = WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
        if ((SVar9 != 0x0) || (uVar7 == 0x0)) {
          iVar16 = 0x12;
          iVar10 = 0x12;
          pass1_1000_25a8(param_5,(int)s_tile2_bmp_1050_1538);
          pass1_1000_2913(iVar10,param_5,(uint16_t)s_tile2_bmp_1050_1538);
          str = poss_str_op_1000_28dc(iVar16);
          if (str == (PCHAR)0x0) goto LAB_1000_28cb;
          iVar10 = 0x9;
          if (*str == 'M') {
            iVar10 = 0xf;
          }
          str = str + iVar10;
          iVar10 = 0x22;
          pcVar13 = str;
          break;
        }
        HVar8 = pass1_1000_422a(uVar7,HVar8,(int)s_tile2_bmp_1050_1538,unaff_SS);
        if (HVar8 == 0x0) {
          GlobalUnlock16((HGLOBAL16)s_tile2_bmp_1050_1538);
          GlobalFree16((HGLOBAL16)s_tile2_bmp_1050_1538);
          HVar8 = 0x0;
        }
      }
      param_4 = (ushort)s_tile2_bmp_1050_1538;
      if (HVar8 != 0x0) {
        puVar14 = (undefined2 *)0x0;
        for (; uVar11 != 0x0; uVar11 = uVar11 - 0x1) {
          for (iVar10 = -0x8000; iVar10 != 0x0; iVar10 = iVar10 + -0x1) {
            puVar4 = puVar14;
            puVar14 = puVar14 + 0x1;
            *puVar4 = 0x0;
          }
          HVar8 = HVar8 + 0x100;
        }
        if (uVar6 != 0x0) {
          for (; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
            puVar4 = puVar14;
            puVar14 = (undefined2 *)((int)puVar14 + 0x1);
            *(undefined *)puVar4 = 0x0;
          }
        }
        return (int *)puVar12;
      }
    }
LAB_1000_41aa:
    if (((uint)PTR_LOOP_1050_618e | (uint)PTR_LOOP_1050_618c) == 0x0) {
      return (int *)(ushort *)0x0;
    }
    iVar10 = (*(code *)PTR_LOOP_1050_618c)(param_4,param_3,param_1,param_2);
    if (iVar10 == 0x0) {
      return (int *)(ushort *)0x0;
    }
  } while( true );
  while( true ) {
    iVar10 = iVar10 + -0x1;
    pcVar3 = pcVar13;
    pcVar13 = pcVar13 + 0x1;
    if (*pcVar3 == '\r') break;
    if (iVar10 == 0x0) break;
  }
  pcVar13[-0x1] = '\0';
LAB_1000_28cb:
  FatalAppExit16((UINT16)s_tile2_bmp_1050_1538,str);
  FatalExit();
  puVar12 = (ushort *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar12;
    puVar12 = puVar12 + 0x1;
    uVar2 = *puVar1;
    puVar5 = puVar12;
    if ((uVar2 == HVar8) || (puVar5 = (ushort *)(uVar2 + 0x1), puVar5 == (ushort *)0x0)) {
      return (int *)puVar5;
    }
    iVar10 = -0x1;
    do {
      if (iVar10 == 0x0) break;
      iVar10 = iVar10 + -0x1;
      puVar1 = puVar12;
      puVar12 = (ushort *)((int)puVar12 + 0x1);
    } while (*(char *)puVar1 != '\0');
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps

ushort __cdecl16near pass1_1000_41e0(int param_1)

{
  int *piStack6;
  
  piStack6 = (int *)CONCAT22(PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  while( true ) {
    if (PTR_LOOP_1050_6190 + ((uint)PTR_LOOP_1050_6194 & 0xfffc) <= (undefined *)piStack6) {
      return 0x0;
    }
    if (*piStack6 == param_1) break;
    piStack6 = (int *)((ulong)piStack6 & 0xffff0000 | ZEXT24((undefined *)piStack6 + 0x4));
  }
  *piStack6 = 0x0;
  return *(ushort *)((undefined *)piStack6 + 0x2);
}



// WARNING: Could not reconcile some variable overlaps

int __cdecl16near pass1_1000_422a(int param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined *puVar1;
  undefined *puVar2;
  undefined *puVar3;
  undefined *puVar4;
  int *piStack6;
  
  piStack6 = (int *)CONCAT22(PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  while( true ) {
    if (PTR_LOOP_1050_6190 + ((uint)PTR_LOOP_1050_6194 & 0xfffc) <= (undefined *)piStack6) {
      puVar2 = PTR_LOOP_1050_6194 + 0x28;
      puVar4 = PTR_LOOP_1050_6192;
      puVar3 = (undefined *)
               pass1_1000_16aa((ushort *)PTR_LOOP_1050_6190,(uint)PTR_LOOP_1050_6192,(uint)puVar2,
                               (uint)PTR_LOOP_1050_6192,param_3,param_4);
      if (((uint)puVar4 | (uint)puVar3) == 0x0) {
        param_1 = 0x0;
      }
      else {
        puVar1 = puVar3 + ((uint)PTR_LOOP_1050_6194 & 0xfffc);
        piStack6 = (int *)CONCAT22(puVar4,puVar1);
        PTR_LOOP_1050_6190 = puVar3;
        PTR_LOOP_1050_6192 = puVar4;
        *piStack6 = param_1;
        *(ushort *)(puVar1 + 0x2) = param_2;
        PTR_LOOP_1050_6194 = puVar2;
        pass1_1000_4906((astruct_20 *)CONCAT22(puVar4,puVar1 + 0x4),(WNDCLASS16 *)0x0,0x24);
      }
      return param_1;
    }
    if (*piStack6 == 0x0) break;
    piStack6 = (int *)((ulong)piStack6 & 0xffff0000 | ZEXT24((undefined *)piStack6 + 0x4));
  }
  *(ushort *)((undefined *)piStack6 + 0x2) = param_2;
  *piStack6 = param_1;
  return param_1;
}



// WARNING: Removing unreachable block (ram,0x10004311)

void __cdecl16far dos3_call_set_struct_1000_42de(ushort *param_1,ushort *param_2,ushort *param_3)

{
  undefined2 uVar1;
  ushort uVar2;
  code *pcVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  bool bVar10;
  undefined4 uVar11;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar5 = *(undefined2 *)(iVar6 + 0x2);
  uVar4 = *(undefined2 *)(iVar6 + 0x4);
  uVar1 = *(undefined2 *)(iVar6 + 0x8);
  uVar7 = *(undefined2 *)(iVar6 + 0xa);
  uVar8 = (undefined2)((ulong)param_3 >> 0x10);
  uVar2 = *param_3;
  uVar9 = *(undefined2 *)((int)param_3 + 0x6);
  bVar10 = false;
  pcVar3 = (code *)swi(0x21);
  uVar11 = (*pcVar3)();
  *param_3 = uVar2;
  *(undefined2 *)((int)param_3 + 0x6) = uVar9;
  uVar9 = (undefined2)((ulong)param_2 >> 0x10);
  iVar6 = (int)param_2;
  *param_2 = (uint)uVar11;
  *(undefined2 *)(iVar6 + 0x2) = uVar5;
  *(undefined2 *)(iVar6 + 0x4) = uVar4;
  *(undefined2 *)(iVar6 + 0x6) = (int)((ulong)uVar11 >> 0x10);
  *(undefined2 *)(iVar6 + 0x8) = uVar1;
  *(undefined2 *)(iVar6 + 0xa) = uVar7;
  if (bVar10) {
    pass1_1000_29af((uint)uVar11);
  }
  *(uint *)(iVar6 + 0xc) = (uint)bVar10;
  return;
}



// WARNING: Removing unreachable block (ram,0x1000438a)
// WARNING: Removing unreachable block (ram,0x10004372)
// WARNING: Removing unreachable block (ram,0x100043aa)

void __cdecl16far dos3_call_op_1000_435c(UINT16 *param_1,uint param_2,undefined2 param_3,int param_4,UINT16 param_5)

{
  code *pcVar1;
  UINT16 UVar2;
  uint uVar3;
  uint extraout_DX;
  uint extraout_DX_00;
  uint extraout_DX_01;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  int iStack2;
  
  iStack2 = param_4 + 0x1;
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&USHORT_1050_1050);
  pcVar1 = (code *)swi(0x21);
  uVar3 = param_2;
  uVar5 = extraout_DX;
  (*pcVar1)();
  uVar9 = extraout_DX_00 >> 0x8;
  uVar8 = uVar3 & 0xff;
  uVar6 = uVar3 >> 0x8;
  pcVar1 = (code *)swi(0x21);
  uVar7 = uVar6;
  (*pcVar1)();
  uVar4 = extraout_DX_01;
  if ((uVar5 != extraout_DX_01) && (uVar4 = extraout_DX_01, (char)uVar6 == '\x17')) {
    uVar3 = param_2;
    uVar4 = uVar5;
  }
  UVar2 = pass1_1000_462e(uVar3 - 0x7bc,uVar4 >> 0x8,uVar4 & 0xff,uVar7,uVar8,uVar9,&iStack2,param_5,uVar4);
  if (param_1._2_2_ != 0x0) {
    *(uint *)((int)param_1 + 0x2) = uVar4;
    *param_1 = UVar2;
  }
  return;
}



void __cdecl16far pass1_1000_43f0(uint16_t param_1,uint16_t param_2)

{
  if (PTR_LOOP_1050_68b4 == (undefined *)0x0) {
    pass1_1000_440c(param_2);
    PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 0x1;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far pass1_1000_440c(uint param_1)

{
  char cVar1;
  char *pcVar2;
  uint uVar3;
  int iVar4;
  undefined2 uVar5;
  long lVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  char *pcStack8;
  
  uVar3 = pass1_1000_3ec0(0x61ca,(int)&USHORT_1050_1050);
  pcStack8 = (char *)CONCAT22(param_1,uVar3);
  if (((param_1 | uVar3) != 0x0) && (_DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0,DAT_1050_61ce), *pcStack8 != '\0')) {
    str_op_1000_3dbe((char *)CONCAT13((char)((uint)PTR_USHORT_1050_1050_1050_61de >> 0x8),
                                      CONCAT12((char)PTR_USHORT_1050_1050_1050_61de,
                                               PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc)),
                     (char *)CONCAT22(param_1,uVar3),0x3);
    pcStack8 = (char *)CONCAT22(param_1,uVar3 + 0x3);
    cVar1 = *pcStack8;
    if (cVar1 == '-') {
      pcStack8 = (char *)CONCAT22(param_1,uVar3 + 0x4);
    }
    uVar5 = 0x0;
    uVar8 = 0x0;
    uVar7 = 0xe10;
    iVar4 = pass1_1000_3e2c((ulong)pcStack8 & 0xffff | (ulong)param_1 << 0x10);
    _DAT_1050_61ce = pass1_1000_52be((char)iVar4,uVar5,uVar7,uVar8);
    for (; (pcVar2 = pcStack8, *pcStack8 == '+' || (('/' < *pcStack8 && (*pcStack8 < ':'))));
        pcStack8 = (char *)((ulong)pcStack8 & 0xffff0000 | (ulong)((int)pcStack8 + 0x1))) {
    }
    if (*pcStack8 == ':') {
      uVar5 = 0x0;
      uVar8 = 0x0;
      uVar7 = 0x3c;
      pcStack8 = (char *)((ulong)pcStack8 & 0xffff0000 | (ulong)((int)pcStack8 + 0x1));
      iVar4 = pass1_1000_3e2c((ulong)pcVar2 & 0xffff0000 | (ulong)((int)pcStack8 + 0x1));
      lVar6 = pass1_1000_52be((char)iVar4,uVar5,uVar7,uVar8);
      uVar5 = (undefined2)((ulong)lVar6 >> 0x10);
      _DAT_1050_61ce = _DAT_1050_61ce + lVar6;
      for (; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':'));
          pcStack8 = (char *)((ulong)pcStack8 & 0xffff0000 | (ulong)((int)pcStack8 + 0x1))) {
      }
      if (*pcStack8 == ':') {
        pcStack8 = (char *)((ulong)pcStack8 & 0xffff0000 | (ulong)((int)pcStack8 + 0x1));
        iVar4 = pass1_1000_3e2c((ulong)pcVar2 & 0xffff0000 | (ulong)((int)pcStack8 + 0x1));
        _DAT_1050_61ce = _DAT_1050_61ce + CONCAT22(uVar5,iVar4);
        for (; ('/' < *pcStack8 && (*pcStack8 < ':'));
            pcStack8 = (char *)((ulong)pcStack8 & 0xffff0000 | (ulong)((int)pcStack8 + 0x1))) {
        }
      }
    }
    PTR_LOOP_1050_61d0 = (undefined *)((ulong)_DAT_1050_61ce >> 0x10);
    if (cVar1 == '-') {
      _DAT_1050_61ce = CONCAT22(-(int)(PTR_LOOP_1050_61d0 + (DAT_1050_61ce != 0x0)),-DAT_1050_61ce);
    }
    DAT_1050_61d2 = (int)*pcStack8;
    if (DAT_1050_61d2 == 0x0) {
      *_PTR_PTR_1050_61e0 = '\0';
    }
    else {
      str_op_1000_3dbe(_PTR_PTR_1050_61e0,pcStack8,0x3);
    }
  }
  PTR_LOOP_1050_61d0 = (undefined *)((ulong)_DAT_1050_61ce >> 0x10);
  return;
}



UINT16 __cdecl16far pass1_1000_455a(UINT32 param_1,UINT16 param_2)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  UINT16 UVar5;
  ulong uVar6;
  int iStack6;
  
  if (((*(int *)(param_1 + 0xa) < 0x43) || (*(int *)(param_1 + 0x8) < 0x3)) || (0x9 < *(int *)(param_1 + 0x8)))
  goto LAB_1000_4623;
  if ((*(int *)(param_1 + 0x8) < 0x4) || (0x8 < *(int *)(param_1 + 0x8))) {
    uVar3 = *(uint *)(param_1 + 0xa);
    if (((int)uVar3 < 0x57) || (*(int *)(param_1 + 0x8) != 0x3)) {
      iStack6 = *(int *)(*(int *)(param_1 + 0x8) * 0x2 + 0x61b2);
    }
    else {
      iStack6 = *(int *)(*(int *)(param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
    }
    if ((uVar3 & 0x3) == 0x0) {
      iStack6 = iStack6 + 0x1;
    }
    uVar3 = (uVar3 - 0x46) * 0x16d + ((int)(uVar3 - 0x1) >> 0x2) + iStack6;
    uVar6 = pass1_1000_52f0(uVar3 - 0xd,((int)uVar3 >> 0xf) - (uint)(uVar3 < 0xd),0x7,0x0);
    iStack6 = (int)uVar6 - iStack6;
    iVar4 = -iStack6;
    if (*(int *)(param_1 + 0x8) == 0x3) {
      iVar2 = *(int *)(param_1 + 0xe);
      if ((iVar4 < iVar2) || ((-iVar2 == iStack6 && (0x1 < *(int *)(param_1 + 0x4))))) goto LAB_1000_460e;
    }
    else {
      piVar1 = (int *)(param_1 + 0xe);
      iVar2 = *piVar1;
      if ((SBORROW2(*piVar1,iVar4) != iVar2 + iStack6 < 0x0) || ((iVar2 == iVar4 && (*(int *)(param_1 + 0x4) < 0x1))))
      goto LAB_1000_460e;
    }
LAB_1000_4623:
    UVar5 = 0x0;
  }
  else {
LAB_1000_460e:
    UVar5 = 0x1;
  }
  return UVar5;
}



int __cdecl16far
pass1_1000_462e(uint param_1,int param_2,uint param_3,uint param_4,uint param_5,int param_6,int param_7,UINT16 param_8,
               uint16_t param_9)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  UINT16 UVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  undefined4 uVar9;
  int iStack26;
  undefined local_16 [0x4];
  uint uStack18;
  int iStack14;
  int iStack12;
  int iStack8;
  UINT16 local_4;
  int iStack2;
  UINT16 uVar10;
  UINT16 uVar11;
  UINT16 uVar12;
  UINT16 uVar13;
  
  iStack2 = param_7 + 0x1;
  local_4 = (UINT16)&USHORT_1050_1050;
  uVar8 = *(uint *)(param_2 * 0x2 + 0x61ae);
  if (((param_1 & 0x3) == 0x0) && (0x2 < param_2)) {
    uVar8 = uVar8 + 0x1;
  }
  pass1_1000_43f0((uint16_t)&iStack2,param_9);
  uVar13 = 0x0;
  uVar12 = 0x3c;
  uVar11 = 0x0;
  uVar10 = 0x3c;
  uVar1 = (uint)((long)(int)param_1 * 0x16d);
  uVar2 = (int)(param_1 + 0x3) >> 0x2;
  uVar3 = uVar2 + param_3;
  uVar6 = uVar1 + uVar3;
  uVar7 = uVar6 + uVar8;
  uVar9 = pass1_1000_52be(uVar7 + 0xe44,
                          (int)((ulong)((long)(int)param_1 * 0x16d) >> 0x10) +
                          ((int)(param_1 + 0x3) >> 0xf) + ((int)param_3 >> 0xf) + (uint)CARRY2(uVar2,param_3) +
                          (uint)CARRY2(uVar1,uVar3) + ((int)uVar8 >> 0xf) + (uint)CARRY2(uVar6,uVar8) +
                          (uint)(0xf1bb < uVar7),0x18,0x0);
  uVar9 = pass1_1000_52be((uint)uVar9 + param_4,
                          (int)((ulong)uVar9 >> 0x10) + ((int)param_4 >> 0xf) + (uint)CARRY2((uint)uVar9,param_4),uVar10
                          ,uVar11);
  iVar4 = pass1_1000_52be((uint)uVar9 + param_5,
                          (int)((ulong)uVar9 >> 0x10) + ((int)param_5 >> 0xf) + (uint)CARRY2((uint)uVar9,param_5),uVar12
                          ,uVar13);
  iStack26 = iVar4 + param_6 + DAT_1050_61ce;
  iStack8 = param_3 + uVar8;
  iStack12 = param_1 + 0x50;
  iStack14 = param_2 + -0x1;
  uStack18 = param_4;
  if (DAT_1050_61d2 != 0x0) {
    UVar5 = pass1_1000_455a((UINT32)local_16,param_8);
    if (UVar5 != 0x0) {
      iStack26 = iStack26 + -0xe10;
    }
  }
  return iStack26;
}
