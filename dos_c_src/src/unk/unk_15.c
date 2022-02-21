
char * __cdecl16far pass1_1000_472c(ulong param_1,char param_2)

{
  char *pcVar1;
  uint uVar2;
  char *pcVar3;
  char *pcVar4;
  undefined2 uVar5;
  bool bVar6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  pcVar3 = (char *)param_1;
  bVar6 = true;
  uVar2 = 0xffff;
  pcVar4 = pcVar3;
  do {
    if (uVar2 == 0x0) break;
    uVar2 = uVar2 - 0x1;
    pcVar1 = pcVar4;
    pcVar4 = pcVar4 + 0x1;
    bVar6 = *pcVar1 == '\0';
  } while (!bVar6);
  uVar2 = ~uVar2;
  do {
    if (uVar2 == 0x0) break;
    uVar2 = uVar2 - 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar6 = param_2 == *pcVar1;
  } while (!bVar6);
  if (!bVar6) {
    if (param_2 != '\0') {
      return (char *)0x0;
    }
    pcVar3 = pcVar3 + 0x1;
  }
  return pcVar3 + -0x1;
}



int __cdecl16far pass1_1000_475e(ulong param_1,ulong param_2)

{
  char *pcVar1;
  char cVar2;
  char cVar3;
  byte bVar4;
  astruct_235 *bVar3;
  astruct_236 *bVar5;
  char *pcVar5;
  char *pcVar6;
  
  pcVar6 = (char *)param_2;
  pcVar5 = (char *)param_1;
  bVar5 = (astruct_236 *)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
  do {
    do {
      cVar3 = (char)bVar5;
      if (cVar3 == '\0') goto LAB_1000_479d;
      pcVar1 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
      cVar3 = *pcVar1;
      cVar2 = *pcVar5;
      bVar5 = (astruct_236 *)CONCAT11(cVar2,cVar3);
      pcVar5 = pcVar5 + 0x1;
    } while (cVar2 == cVar3);
    bVar4 = cVar3 + 0xbfU + (-((byte)(cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
    bVar3._0_1_ = cVar2 + 0xbf;
    bVar5._0_1_ = (byte)bVar3 + (-((byte)bVar3 < 0x1a) & 0x20U) + 0x41;
    bVar5 = (astruct_236 *)CONCAT11(bVar4,(byte)bVar5);
  } while ((byte)bVar5 == bVar4);
  cVar3 = ((byte)bVar5 < bVar4) * -0x2 + '\x01';
LAB_1000_479d:
  return (int)cVar3;
}



ushort __cdecl16far pass1_1000_47a4(ulong param_1,ulong param_2,ushort param_3)

{
  byte *pbVar1;
  byte bVar2;
  undefined2 *puVar3;
  byte *pbVar4;
  int iVar5;
  byte *pbVar6;
  undefined2 *puVar7;
  undefined2 uVar8;
  undefined2 local_22 [0x10];
  
  puVar7 = local_22;
  for (iVar5 = 0x10; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
    puVar3 = puVar7;
    puVar7 = puVar7 + 0x1;
    *puVar3 = 0x0;
  }
  pbVar6 = (byte *)param_2;
  while( true ) {
    pbVar1 = pbVar6;
    pbVar6 = pbVar6 + 0x1;
    bVar2 = *pbVar1;
    if (bVar2 == 0x0) break;
    pbVar1 = (byte *)((int)local_22 + (uint)(bVar2 >> 0x3));
    *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
  }
  pbVar1 = (byte *)param_1;
  if (param_1 == 0x0) {
    pbVar1 = pbRam105061e4;
  }
  do {
    pbRam105061e4 = pbVar1;
    uVar8 = (undefined2)((ulong)pbRam105061e4 >> 0x10);
    pbVar6 = (byte *)((int)pbRam105061e4 + 0x1);
    bVar2 = *pbRam105061e4;
    if (bVar2 == 0x0) {
      return 0x0;
    }
    pbVar1 = (byte *)((ulong)pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
  } while (('\x01' << (bVar2 & 0x7) & *(byte *)((int)local_22 + (uint)(bVar2 >> 0x3))) != 0x0);
  do {
    pbVar4 = pbVar6;
    bVar2 = *pbVar4;
    if (bVar2 == 0x0) goto LAB_1000_483c;
    pbVar6 = pbVar4 + 0x1;
  } while (('\x01' << (bVar2 & 0x7) & *(byte *)((int)local_22 + (uint)(bVar2 >> 0x3))) == 0x0);
  *pbVar4 = 0x0;
  pbVar4 = pbVar4 + 0x1;
LAB_1000_483c:
  pbRam105061e4 = (byte *)((ulong)pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
  return (ushort)pbRam105061e4;
}



uint __cdecl16far pass1_1000_484c(ulong param_1,ulong param_2,uint param_3)

{
  byte *pbVar1;
  byte *pbVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  byte *pbVar6;
  byte *pbVar7;
  int iVar8;
  bool bVar9;
  bool bVar10;
  
  if (param_3 == 0x0) {
    return 0x0;
  }
  do {
    iVar8 = (int)(param_2 >> 0x10);
    pbVar7 = (byte *)param_2;
    iVar3 = (int)(param_1 >> 0x10);
    pbVar6 = (byte *)param_1;
    uVar4 = ~(uint)pbVar7;
    uVar4 = ((param_3 - 0x1) - uVar4 & -(uint)(param_3 - 0x1 < uVar4)) + uVar4;
    uVar5 = ~(uint)pbVar6;
    uVar4 = (uVar4 - uVar5 & -(uint)(uVar4 < uVar5)) + uVar5 + 0x1;
    bVar9 = param_3 < uVar4;
    param_3 = param_3 - uVar4;
    bVar10 = param_3 == 0x0;
    do {
      if (uVar4 == 0x0) break;
      uVar4 = uVar4 - 0x1;
      pbVar2 = pbVar7;
      pbVar7 = pbVar7 + 0x1;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar9 = *pbVar1 < *pbVar2;
      bVar10 = *pbVar1 == *pbVar2;
    } while (bVar10);
    param_2 = param_2 & 0xffff0000 | ZEXT24(pbVar7);
    if (!bVar10) {
      return (0x1 - (uint)bVar9) - (uint)(bVar9 != 0x0);
    }
    if (param_3 == 0x0) {
      return uVar4;
    }
    if (pbVar6 == (byte *)0x0) {
      iVar3 = iVar3 + 0x6c;
    }
    param_1 = CONCAT22(iVar3,pbVar6);
    if (pbVar7 == (byte *)0x0) {
      param_2 = (ulong)(iVar8 + 0x6c) << 0x10;
      param_1 = CONCAT22(iVar3,pbVar6);
    }
  } while( true );
}



ushort __cdecl16far pass1_1000_48a8(ulong param_1,ulong param_2,int param_3)

{
  undefined2 *puVar1;
  undefined2 *puVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  undefined2 *puVar6;
  undefined2 *puVar7;
  int iVar8;
  
  if (param_3 != 0x0) {
    while( true ) {
      iVar3 = (int)(param_2 >> 0x10);
      puVar6 = (undefined2 *)param_2;
      iVar8 = (int)(param_1 >> 0x10);
      puVar7 = (undefined2 *)param_1;
      uVar4 = ~(uint)puVar7;
      uVar4 = ((param_3 - 0x1U) - uVar4 & -(uint)(param_3 - 0x1U < uVar4)) + uVar4;
      uVar5 = ~(uint)puVar6;
      uVar4 = (uVar4 - uVar5 & -(uint)(uVar4 < uVar5)) + uVar5 + 0x1;
      param_3 = param_3 - uVar4;
      for (uVar5 = uVar4 >> 0x1; uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
        puVar2 = puVar7;
        puVar7 = puVar7 + 0x1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 0x1;
        *puVar2 = *puVar1;
      }
      for (uVar4 = (uint)((uVar4 & 0x1) != 0x0); uVar4 != 0x0; uVar4 = uVar4 - 0x1) {
        puVar2 = puVar7;
        puVar7 = (undefined2 *)((int)puVar7 + 0x1);
        puVar1 = puVar6;
        puVar6 = (undefined2 *)((int)puVar6 + 0x1);
        *(undefined *)puVar2 = *(undefined *)puVar1;
      }
      if (param_3 == 0x0) break;
      if (puVar6 == (undefined2 *)0x0) {
        iVar3 = iVar3 + 0x6c;
      }
      param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
      param_2 = CONCAT22(iVar3,puVar6);
      if (puVar7 == (undefined2 *)0x0) {
        param_1 = (ulong)(iVar8 + 0x6c) << 0x10;
        param_2 = CONCAT22(iVar3,puVar6);
      }
    }
  }
  return (ushort)param_1;
}



uint * __cdecl16far pass1_1000_4906(astruct_20 *param_1,WNDCLASS16 *in_wnd_class,uint param_3)

{
  uint *puVar1;
  undefined uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint *puVar7;
  int iVar8;
  
  if (param_3 != 0x0) {
    iVar8 = (int)((ulong)param_1 >> 0x10);
    uVar5 = -(int)(uint *)param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(uint)(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = (uint)in_wnd_class & 0xff | (int)in_wnd_class << 0x8;
    puVar7 = (uint *)param_1;
    for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 = uVar4 - 0x1) {
      puVar1 = puVar7;
      puVar7 = puVar7 + 0x1;
      *puVar1 = uVar3;
    }
    for (uVar6 = (uint)((uVar6 & 0x1) != 0x0); uVar2 = (undefined)((uint)in_wnd_class & 0xff), uVar6 != 0x0;
        uVar6 = uVar6 - 0x1) {
      puVar1 = puVar7;
      puVar7 = (uint *)((int)puVar7 + 0x1);
      *(undefined *)puVar1 = uVar2;
    }
    if (uVar5 != 0x0) {
      for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar1 = uVar3;
      }
      for (uVar6 = (uint)((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
        puVar1 = puVar7;
        puVar7 = (uint *)((int)puVar7 + 0x1);
        *(undefined *)puVar1 = uVar2;
      }
    }
  }
  return (uint *)param_1;
}



int __cdecl16far pass1_1000_49b2(uint param_1)

{
  return (param_1 ^ (int)param_1 >> 0xf) - ((int)param_1 >> 0xf);
}



uint __cdecl16far
pass1_1000_49c6(ushort param_1,ushort param_2,uint param_3,uint param_4,uint param_5,uint param_6,uchar *param_7,
               int param_8)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  long lVar7;
  uint uStack20;
  uint uStack18;
  uint uStack8;
  uint uStack6;
  
  uStack20 = param_3;
  uStack18 = param_4;
  lVar7 = pass1_1000_52be(param_5 - 0x1,-(uint)(param_5 == 0x0),param_6,0x0);
  uStack8 = (uint)(lVar7 + 0x8);
  uStack6 = (int)((ulong)(lVar7 + 0x8) >> 0x10) * 0x100 + param_4;
  while( true ) {
    if (uStack6 < uStack18) {
      return 0x0;
    }
    if ((uStack6 <= uStack18) && (uStack8 < uStack20)) {
      return 0x0;
    }
    uVar1 = param_5 >> 0x1;
    if (uVar1 == 0x0) {
      if ((param_5 != 0x0) && (iVar5 = (*(code *)param_7)(), iVar5 == 0x0)) {
        return uStack20;
      }
      return 0x0;
    }
    uVar2 = uVar1;
    if ((param_5 & 0x1) == 0x0) {
      uVar2 = uVar1 - 0x1;
    }
    uVar3 = (uint)((ulong)uVar2 * (ulong)param_6);
    uVar4 = uVar3 + uStack20;
    iVar6 = ((int)((ulong)uVar2 * (ulong)param_6 >> 0x10) + (uint)CARRY2(uVar3,uStack20)) * 0x100 + uStack18;
    iVar5 = (*(code *)param_7)();
    if (iVar5 == 0x0) break;
    if (iVar5 < 0x0) {
      uStack8 = -param_6 + uVar4;
      uStack6 = ((uint)CARRY2(-param_6,uVar4) - (uint)(param_6 != 0x0)) * 0x100 + iVar6;
      uVar2 = param_5 & 0x1;
      param_5 = uVar1;
      if (uVar2 == 0x0) {
        param_5 = uVar1 - 0x1;
      }
    }
    else {
      uStack20 = param_6 + uVar4;
      uStack18 = (uint)CARRY2(param_6,uVar4) * 0x100 + iVar6;
      param_5 = uVar1;
    }
  }
  return uVar4;
}



void __cdecl16near pass1_1000_4ceb(uint param_1,int param_2,int param_3,ushort param_4)

{
  undefined *puVar1;
  undefined2 *puVar2;
  undefined uVar3;
  undefined2 uVar4;
  
  if ((param_1 & 0x1) != 0x0) {
    param_1 = param_1 - 0x1;
    puVar1 = (undefined *)(param_1 + param_3);
    uVar3 = *puVar1;
    *puVar1 = *(undefined *)(param_1 + param_2);
    *(undefined *)(param_1 + param_2) = uVar3;
    if (param_1 == 0x0) {
      return;
    }
  }
  do {
    param_1 = param_1 - 0x2;
    puVar2 = (undefined2 *)(param_1 + param_3);
    uVar4 = *puVar2;
    *puVar2 = *(undefined2 *)(param_1 + param_2);
    *(undefined2 *)(param_1 + param_2) = uVar4;
  } while (param_1 != 0x0);
  return;
}



void __cdecl16far pass1_1000_4d0c(UINT16 param_1)

{
  DAT_1050_61e8 = param_1;
  PTR_LOOP_1050_61ea = (undefined *)0x0;
  return;
}



uint __cdecl16far pass1_1000_4d24(void)

{
  long lVar1;
  
  lVar1 = pass1_1000_52be(DAT_1050_61e8,PTR_LOOP_1050_61ea,(int)s_TPPOPMENU_1050_43fa + 0x3,0x3);
  PTR_LOOP_1050_61ea = (undefined *)((ulong)(lVar1 + 0x269ec3) >> 0x10);
  DAT_1050_61e8 = (int)(lVar1 + 0x269ec3);
  return (uint)PTR_LOOP_1050_61ea & 0x7fff;
}


ushort __cdecl16far pass1_1000_4f2e(ushort param_1)

{
  code *pcVar1;
  undefined2 uVar2;
  bool bVar3;
  
  bVar3 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)((int)&USHORT_1050_1050,param_1 + 0x1);
  if (bVar3) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}


void __cdecl16far pass1_1000_5008(ushort param_1,ushort param_2,ushort param_3,int param_4)

{
  ushort unaff_CS;
  ushort unaff_SS;
  int iStack2;
  
  iStack2 = param_4 + 0x1;
  pass1_1000_5026(0x0,param_1,param_2,param_3,(int)&iStack2,unaff_CS,unaff_SS);
  return;
}


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


void __cdecl16far pass1_1000_43f0(uint16_t param_1,uint16_t param_2)

{
  if (PTR_LOOP_1050_68b4 == (undefined *)0x0) {
    pass1_1000_440c(param_2);
    PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 0x1;
  }
  return;
}


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

void __cdecl16near pass1_1000_3552(int param_1,int param_2,ushort param_3)

{
  int *piVar1;
  uint uVar2;
  undefined2 in_DX;
  uint uVar3;
  undefined2 unaff_CS;
  undefined in_AF;
  
  if (param_1 != 0x0) {
    piVar1 = (int *)(param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar3 = 0x0;
    do {
      uVar2 = pass1_1000_3503((char)in_DX,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar3 = uVar3 | uVar2;
      param_1 = param_1 + -0x1;
    } while (param_1 != 0x0);
    if (uVar3 != 0x0) {
      *(undefined2 *)(param_2 + -0xa) = 0xffff;
    }
  }
  return;
}



void __cdecl16near
pass1_1000_356e(uint param_1,uint param_2,uint param_3,int param_4,int param_5,byte *param_6,undefined2 param_7,
               undefined2 param_8)

{
  byte *pbVar1;
  ulong uVar2;
  byte bVar3;
  
  while (((0x0 < param_5 || (param_1 != 0x0)) || (param_3 != 0x0))) {
    uVar2 = (ulong)param_3;
    param_3 = param_3 / param_2;
    uVar2 = uVar2 % (ulong)param_2 << 0x10 | (ulong)param_1;
    param_1 = (uint)(uVar2 / param_2);
    bVar3 = (char)(uVar2 % (ulong)param_2) + 0x30;
    if (0x39 < bVar3) {
      bVar3 = bVar3 + *(char *)(param_4 + -0x3);
    }
    pbVar1 = param_6;
    param_6 = param_6 + -0x1;
    *pbVar1 = bVar3;
    param_5 = param_5 + -0x1;
  }
  return;
}



ushort * __cdecl16far pass1_1000_35aa(void)

{
  ushort *puVar1;
  
  puVar1 = (ushort *)&PTR_LOOP_1050_6210;
  while( true ) {
    if (PTR_LOOP_1050_5ff0 < puVar1) {
      return (ushort *)0x0;
    }
    if ((*(byte *)(puVar1 + 0x5) & 0x83) == 0x0) break;
    puVar1 = puVar1 + 0x6;
  }
  *(undefined *)(puVar1 + 0x5) = 0x0;
  puVar1[0x2] = 0x0;
  puVar1[0x4] = 0x0;
  puVar1[0x3] = 0x0;
  puVar1[0x1] = 0x0;
  *puVar1 = 0x0;
  *(undefined *)((int)puVar1 + 0xb) = 0xff;
  return puVar1;
}


void __cdecl16near pass1_1000_39e1(void)

{
  return;
}

int __cdecl16far pass1_1000_3bac(void)

{
  int iVar1;
  
  if (PTR_LOOP_1050_5f48 < &stack0x0004) {
    iVar1 = -((int)PTR_LOOP_1050_5f48 - (int)&stack0x0004);
  }
  else {
    iVar1 = 0x0;
  }
  return iVar1;
}

void __cdecl16near pass1_1000_3cb7(int param_1)

{
  uint uVar1;
  uint *puVar2;
  
  puVar2 = *(uint **)(param_1 + 0xa);
  if (puVar2 == *(uint **)(param_1 + 0xc)) {
    puVar2 = *(uint **)(param_1 + 0x8);
  }
  while( true ) {
    uVar1 = *puVar2;
    if (uVar1 == 0xfffe) break;
    puVar2 = (uint *)((int)puVar2 + (uVar1 & 0xfffe) + 0x2);
  }
  return;
}


UINT16 * __cdecl16far pass1_1000_3cea(ULONG param_1,ULONG param_2)

{
  UINT16 *pUVar1;
  char *pcVar2;
  UINT16 *pUVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  UINT16 *pUVar7;
  char *pcVar8;
  UINT16 *pUVar9;
  UINT16 *pUVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  bool bVar13;
  
  uVar11 = (undefined2)(param_1 >> 0x10);
  bVar13 = true;
  iVar4 = -0x1;
  pUVar7 = (UINT16 *)param_1;
  do {
    if (iVar4 == 0x0) break;
    iVar4 = iVar4 + -0x1;
    pUVar1 = pUVar7;
    pUVar7 = (UINT16 *)((int)pUVar7 + 0x1);
    bVar13 = *(char *)pUVar1 == '\0';
  } while (!bVar13);
  pUVar10 = (UINT16 *)((int)pUVar7 + -0x1);
  uVar12 = (undefined2)(param_2 >> 0x10);
  pcVar8 = (char *)param_2;
  uVar5 = 0xffff;
  do {
    if (uVar5 == 0x0) break;
    uVar5 = uVar5 - 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
    bVar13 = *pcVar2 == '\0';
  } while (!bVar13);
  uVar5 = ~uVar5;
  if (!bVar13) {
    pcVar8 = pcVar8 + -uVar5;
    uVar5 = uVar5 + 0x1;
  }
  pUVar9 = (UINT16 *)(pcVar8 + -uVar5);
  if (uVar5 == 0x0) {
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar10 = *pUVar1;
    uVar5 = 0xfffe;
    pUVar10 = (UINT16 *)((int)pUVar7 + 0x1);
  }
  else {
    if (((uint)pUVar9 & 0x1) != 0x0) {
      pUVar1 = pUVar9;
      pUVar9 = (UINT16 *)((int)pUVar9 + 0x1);
      *(undefined *)pUVar10 = *(undefined *)pUVar1;
      uVar5 = uVar5 - 0x1;
      pUVar10 = pUVar7;
    }
  }
  for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = pUVar10 + 0x1;
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar3 = *pUVar1;
  }
  for (uVar5 = (uint)((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = (UINT16 *)((int)pUVar10 + 0x1);
    pUVar1 = pUVar9;
    pUVar9 = (UINT16 *)((int)pUVar9 + 0x1);
    *(undefined *)pUVar3 = *(undefined *)pUVar1;
  }
  return (UINT16 *)param_1;
}



void __cdecl16far unk_str_op_1000_3d3e(char *param_1,char *in_string_2)

{
  undefined2 *puVar4;
  undefined2 *puVar5;
  uint uVar6;
  uint uVar7;
  char *l_string_2;
  char *puVar6;
  char *puVar7;
  ushort uVar8;
  char *l_string_1;
  bool l_b_var8;
  char *puVar3;
  char *puVar2;
  char *puVar1;
  
  l_string_1 = (char *)((ulong)in_string_2 >> 0x10);
  l_string_2 = (char *)in_string_2;
  l_b_var8 = true;
  uVar6 = 0xffff;
  puVar6 = l_string_2;
  do {
    if (uVar6 == 0x0) break;
    uVar6 = uVar6 - 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    l_b_var8 = *puVar1 == '\0';
  } while (!l_b_var8);
  uVar6 = ~uVar6;
  uVar8 = (ushort)((ulong)param_1 >> 0x10);
  puVar7 = (char *)param_1;
  if (l_b_var8) {
    if (((ulong)param_1 & 0x1) != 0x0) {
      puVar7 = puVar7 + 0x1;
      l_string_2 = l_string_2 + 0x1;
      *param_1 = *in_string_2;
      uVar6 = uVar6 - 0x1;
    }
  }
  else {
    puVar7 = puVar7 + 0x2;
    l_string_2 = l_string_2 + 0x2;
    *(undefined2 *)param_1 = *(undefined2 *)in_string_2;
    uVar6 = uVar6 - 0x1;
  }
  for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 = uVar7 - 0x1) {
    puVar5 = (undefined2 *)puVar7;
    puVar7 = (char *)((int)puVar7 + 0x2);
    puVar4 = (undefined2 *)l_string_2;
    l_string_2 = (char *)((int)l_string_2 + 0x2);
    *puVar5 = *puVar4;
  }
  for (uVar6 = (uint)((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
    puVar5 = (undefined2 *)puVar7;
    puVar7 = (char *)((int)puVar7 + 0x1);
    puVar4 = (undefined2 *)l_string_2;
    l_string_2 = (char *)((int)l_string_2 + 0x1);
    *(undefined *)puVar5 = *(undefined *)puVar4;
  }
  return;
}
void __cdecl16near pass1_1000_2f00(int param_1,int *param_2,ushort param_3,ushort param_4,ushort param_5,uchar param_6)

{
  if (((*(byte *)(param_2 + 0x78) & 0x10) != 0x0) && ((*(byte *)(*(byte *)((int)param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)
     ) {
    pass1_1000_2fa4(param_2,param_3,param_4,param_5,param_6);
    if (param_1 != 0x0) {
      *(byte *)(param_2 + 0x78) = 0x0;
      param_2[0x79] = 0x0;
      *param_2 = 0x0;
      param_2[0x1] = 0x0;
      param_2[0x3] = 0x0;
      param_2[0x4] = 0x0;
    }
  }
  return;
}



ushort __cdecl16far pass1_1000_2f48(long param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,uchar param_6)

{
  ushort uVar1;
  uchar *puVar2;
  int iStack2;
  
  iStack2 = param_2 + 0x1;
  if (param_1 == 0x0) {
    uVar1 = pass1_1000_3038(0x0,param_3,param_4,param_5,param_6);
  }
  else {
    uVar1 = pass1_1000_2fa4((int *)param_1,param_3,param_4,param_5,param_6);
    if (uVar1 == 0x0) {
      if ((*(byte *)((int *)param_1 + 0x78) & 0x40) != 0x0) {
        puVar2 = pass1_1000_400a((uint)*(byte *)((int)(int *)param_1 + 0xb),(ushort)&iStack2);
        uVar1 = -(uint)(puVar2 != (uchar *)0x0);
      }
    }
    else {
      uVar1 = 0xffff;
    }
  }
  return uVar1;
}



ushort __cdecl16near pass1_1000_2fa4(int *param_1,ushort param_2,ushort param_3,ushort param_4,uchar param_5)

{
  int *piVar1;
  byte bVar2;
  int iVar3;
  uchar *puVar4;
  uchar *puVar5;
  ushort uVar6;
  
  uVar6 = 0x0;
  bVar2 = *(byte *)(param_1 + 0x5);
  if (((bVar2 & 0x3) == 0x2) && (((bVar2 & 0x8) != 0x0 || ((*(byte *)(param_1 + 0x78) & 0x1) != 0x0)))) {
    puVar4 = (uchar *)(*param_1 - param_1[0x3]);
    if (0x0 < (int)puVar4) {
      puVar5 = mixed_dos3_call_1000_39f2
                         ((uchar *)(uint)*(byte *)((int)param_1 + 0xb),(char *)CONCAT22(param_1[0x4],param_1[0x3]),
                          puVar4,param_2,param_3,param_4,param_5);
      if (puVar5 == puVar4) {
        if ((*(byte *)(param_1 + 0x5) & 0x80) != 0x0) {
          piVar1 = param_1 + 0x5;
          *(byte *)piVar1 = *(byte *)piVar1 & 0xfd;
        }
      }
      else {
        piVar1 = param_1 + 0x5;
        *(byte *)piVar1 = *(byte *)piVar1 | 0x20;
        uVar6 = 0xffff;
      }
    }
  }
  iVar3 = param_1[0x4];
  *param_1 = param_1[0x3];
  param_1[0x1] = iVar3;
  param_1[0x2] = 0x0;
  return uVar6;
}



void __cdecl16far pass1_1000_3024(ushort param_1,ushort param_2,ushort param_3,uchar param_4)

{
  pass1_1000_3038(0x1,param_1,param_2,param_3,param_4);
  return;
}



int pass1_1000_3038(int param_1,ushort param_2,ushort param_3,ushort param_4,uchar param_5)

{
  ushort uVar1;
  undefined *puVar2;
  int iVar3;
  int iStack4;
  
  iVar3 = 0x0;
  iStack4 = 0x0;
  for (puVar2 = (undefined *)&PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc) {
    if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
      uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),(int)&stack0xfffe,param_2,param_3,param_4,param_5);
      if (uVar1 != 0xffff) {
        iVar3 = iVar3 + 0x1;
      }
    }
    else {
      if ((param_1 == 0x0) &&
         (((puVar2[0xa] & 0x2) != 0x0 &&
          (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),(int)&stack0xfffe,param_2,param_3,param_4,param_5),
          uVar1 == 0xffff)))) {
        iStack4 = -0x1;
      }
    }
  }
  if (param_1 == 0x1) {
    iStack4 = iVar3;
  }
  return iStack4;
}


ushort __cdecl16far pass1_1000_3113(ushort param_1,ushort param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  pass1_1000_3552(0x1,param_1,param_2);
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 - 0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 - 0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 - 0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 - 0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 - 0xa);
}


ushort __cdecl16far pass1_1000_311e(int param_1,undefined2 param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  *(undefined2 *)(param_1 + -0x12) = 0x0;
  *(undefined2 *)(param_1 + -0xc) = 0x0;
  *(undefined2 *)(param_1 + -0x14) = 0x0;
  *(undefined2 *)(param_1 + -0x6) = 0x20;
  *(undefined2 *)(param_1 + -0xe) = 0xffff;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 + -0xa);
}


ushort __cdecl16far pass1_1000_3134(int param_1,ushort param_2)

{
  byte *pbVar1;
  char cVar2;
  char *pcVar3;
  byte bVar4;
  ushort uVar5;
  
  cVar2 = *(char *)(param_1 + -0x4);
  if (cVar2 == '-') {
    pbVar1 = (byte *)(param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x4;
  }
  else {
    if (cVar2 == '+') {
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x1;
    }
    else {
      if (cVar2 == ' ') {
        pbVar1 = (byte *)(param_1 + -0x6);
        *pbVar1 = *pbVar1 | 0x2;
      }
      else {
        if (cVar2 == '#') {
          pbVar1 = (byte *)(param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x80;
        }
        else {
          pbVar1 = (byte *)(param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  *(int *)(param_1 + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)(ulong)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)(ulong)((byte)(bVar4 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return *(ushort *)(param_1 + -0xa);
}

ushort __cdecl16far pass1_1000_3168(int param_1,ushort param_2)

{
  byte *pbVar1;
  char cVar2;
  char *pcVar3;
  byte bVar4;
  ushort uVar5;
  
  cVar2 = *(char *)(param_1 + -0x4);
  if (cVar2 == '*') {
    uVar5 = pass1_1000_34cf(param_1,param_2);
    if ((int)uVar5 < 0x0) {
      uVar5 = -uVar5;
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x4;
    }
  }
  else {
    uVar5 = *(int *)(param_1 + -0xc) * 0xa + (uint)(byte)(cVar2 - 0x30);
  }
  *(ushort *)(param_1 + -0xc) = uVar5;
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  *(int *)(param_1 + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)(ulong)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)(ulong)((byte)(bVar4 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return *(ushort *)(param_1 + -0xa);
}


ushort __cdecl16far pass1_1000_3194(int param_1,undefined2 param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  *(undefined2 *)(param_1 + -0xe) = 0x0;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

ushort __cdecl16far pass1_1000_319c(int param_1,ushort param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  cVar1 = *(char *)(param_1 + -0x4);
  if (cVar1 == '*') {
    uVar4 = pass1_1000_34cf(param_1,param_2);
    if ((int)uVar4 < 0x0) {
      uVar4 = 0xffff;
    }
  }
  else {
    uVar4 = *(int *)(param_1 + -0xe) * 0xa + (uint)(byte)(cVar1 - 0x30);
  }
  *(ushort *)(param_1 + -0xe) = uVar4;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 + -0xa);
}


ushort __cdecl16far pass1_1000_31c5(int param_1,ushort param_2)

{
  byte *pbVar1;
  char cVar2;
  char *pcVar3;
  byte bVar4;
  ushort uVar5;
  
  cVar2 = *(char *)(param_1 + -0x4);
  if (cVar2 == 'l') {
    pbVar1 = (byte *)(param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x10;
  }
  else {
    if (cVar2 == 'F') {
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x20;
    }
    else {
      if (cVar2 == 'N') {
        pbVar1 = (byte *)(param_1 + -0x5);
        *pbVar1 = *pbVar1 | 0x10;
      }
      else {
        if (cVar2 == 'L') {
          pbVar1 = (byte *)(param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x4;
        }
        else {
          pbVar1 = (byte *)(param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  *(int *)(param_1 + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)(ulong)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)(ulong)((byte)(bVar4 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return *(ushort *)(param_1 + -0xa);
}


ushort __cdecl16far pass1_1000_31f7(ushort param_1,int param_2,ushort *param_3,int param_4,ushort param_5)

{
  int *piVar1;
  byte *pbVar2;
  ushort *puVar3;
  char cVar4;
  char *pcVar5;
  byte bVar6;
  ushort uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  char *pcVar11;
  uint uVar12;
  ushort *puVar13;
  char *pcVar14;
  bool bVar15;
  ulong uVar16;
  
  cVar4 = *(char *)(param_2 + -0x4);
  if ((cVar4 == 'd') || (cVar4 == 'i')) {
    pbVar2 = (byte *)(param_2 + -0x6);
    *pbVar2 = *pbVar2 | 0x40;
LAB_1000_3399:
    *(undefined *)(param_2 + -0x8) = 0xa;
LAB_1000_33d4:
    if ((*(byte *)(param_2 + -0x6) & 0x10) == 0x0) {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      if ((*(byte *)(param_2 + -0x6) & 0x40) == 0x0) {
        uVar16 = (ulong)uVar7;
      }
      else {
        uVar16 = SEXT24((int)uVar7);
      }
    }
    else {
      uVar16 = pass1_1000_34d8(param_2,param_5);
    }
    if (((*(byte *)(param_2 + -0x6) & 0x40) != 0x0) && ((long)uVar16 < 0x0)) {
      pbVar2 = (byte *)(param_2 + -0x5);
      *pbVar2 = *pbVar2 | 0x1;
      uVar16 = CONCAT22(-((int)(uVar16 >> 0x10) + (uint)((int)uVar16 != 0x0)),-(int)uVar16);
    }
    if (*(int *)(param_2 + -0xe) < 0x0) {
      *(undefined2 *)(param_2 + -0xe) = 0x1;
    }
    else {
      pbVar2 = (byte *)(param_2 + -0x6);
      *pbVar2 = *pbVar2 & 0xf7;
    }
    if (uVar16 == 0x0) {
      *(undefined2 *)(param_2 + -0x12) = 0x0;
    }
    pcVar11 = (char *)(uint)*(byte *)(param_2 + -0x8);
    pass1_1000_356e((uint)uVar16,(uint)pcVar11,(uint)(uVar16 >> 0x10),param_2,*(int *)(param_2 + -0xe),
                    (byte *)(param_2 + -0x17),param_5,param_5);
    if (((*(byte *)(param_2 + -0x5) & 0x2) != 0x0) && ((pcVar11 == (char *)0x0 || (*(byte *)(param_2 + -0x17) != 0x30)))
       ) {
      *(undefined *)(param_2 + -0x18) = 0x30;
      pcVar11 = pcVar11 + 0x1;
    }
  }
  else {
    if (cVar4 == 'u') goto LAB_1000_3399;
    if (cVar4 == 'X') {
      *(undefined *)(param_2 + -0x3) = 0x7;
LAB_1000_33a9:
      if ((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) {
        *(undefined2 *)(param_2 + -0x12) = 0x2;
        *(undefined *)(param_2 + -0x10) = 0x30;
        *(char *)(param_2 + -0xf) = *(char *)(param_2 + -0x3) + 'Q';
      }
      *(undefined *)(param_2 + -0x8) = 0x10;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'x') {
      *(undefined *)(param_2 + -0x3) = 0x27;
      goto LAB_1000_33a9;
    }
    if (cVar4 == 'o') {
      if ((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) {
        pbVar2 = (byte *)(param_2 + -0x5);
        *pbVar2 = *pbVar2 | 0x2;
      }
      *(undefined *)(param_2 + -0x8) = 0x8;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'c') {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      *(undefined *)(param_2 + -0x216) = (char)uVar7;
      pcVar11 = (char *)0x1;
    }
    else {
      if (cVar4 == 's') {
        pass1_1000_34e6(param_1,param_2,param_5);
        if ((param_3 != (ushort *)0x0) || (pcVar11 = DAT_1050_605d, param_4 != 0x0)) {
          iVar10 = *(int *)(param_2 + -0xe);
          puVar13 = param_3;
          if (iVar10 != 0x0) {
            bVar15 = true;
            do {
              if (iVar10 == 0x0) break;
              iVar10 = iVar10 + -0x1;
              puVar3 = puVar13;
              puVar13 = (ushort *)((int)puVar13 + 0x1);
              bVar15 = *(char *)puVar3 == '\0';
            } while (!bVar15);
            if (bVar15) {
              puVar13 = (ushort *)((int)puVar13 + -0x1);
            }
          }
          pcVar11 = (char *)((int)puVar13 - (int)param_3);
        }
      }
      else {
        if (cVar4 == 'n') {
          pass1_1000_34e6(param_1,param_2,param_5);
          *param_3 = *(ushort *)(param_2 + -0xa);
          if ((*(byte *)(param_2 + -0x6) & 0x10) != 0x0) {
            param_3[0x1] = 0x0;
          }
          goto LAB_1000_30cf;
        }
        if (cVar4 == 'p') {
          if ((*(byte *)(param_2 + -0x6) & 0x30) == 0x0) {
            uVar7 = pass1_1000_34cf(param_2,param_5);
            uVar16 = (ulong)uVar7;
          }
          else {
            uVar16 = pass1_1000_34d8(param_2,param_5);
            uVar12 = (uint)(uVar16 >> 0x10);
            if ((*(byte *)(param_2 + -0x5) & 0x18) == 0x0) {
              *(undefined *)(param_2 + -0x3) = 0x7;
              pass1_1000_356e((uint)uVar16,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x20e),param_5,param_5);
              pass1_1000_356e(uVar12,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x213),param_5,param_5);
              *(undefined *)(param_2 + -0x212) = 0x3a;
              pcVar11 = (char *)0x9;
              goto LAB_1000_3444;
            }
          }
          *(undefined *)(param_2 + -0x3) = 0x7;
          pass1_1000_356e((uint)uVar16,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x213),param_5,param_5);
          pcVar11 = (char *)0x4;
        }
        else {
          if ((cVar4 == 'E') || (cVar4 == 'G')) {
            piVar1 = (int *)(param_2 + -0x14);
            *piVar1 = *piVar1 + 0x1;
          }
          pbVar2 = (byte *)(param_2 + -0x6);
          *pbVar2 = *pbVar2 | 0x40;
          bVar6 = *(byte *)(param_2 + -0x4) | 0x20;
          iVar10 = *(int *)(param_2 + -0xe);
          if (iVar10 < 0x1) {
            if (iVar10 == 0x0) {
              if (bVar6 == 0x67) {
                *(undefined2 *)(param_2 + -0xe) = 0x1;
              }
            }
            else {
              *(undefined2 *)(param_2 + -0xe) = 0x6;
            }
          }
          pcVar11 = (char *)(param_2 + -0x216);
          if ((*(byte *)(param_2 + -0x5) & 0x4) == 0x0) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6068)();
            piVar1 = (int *)(param_2 + 0xe);
            *piVar1 = *piVar1 + 0x8;
          }
          else {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_607c)();
            piVar1 = (int *)(param_2 + 0xe);
            *piVar1 = *piVar1 + 0xa;
          }
          if (((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) && (*(int *)(param_2 + -0xe) == 0x0)) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6074)();
          }
          if ((bVar6 == 0x67) && ((*(uint *)(param_2 + -0x6) & 0x80) == 0x0)) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6070)();
          }
          if (*pcVar11 == '-') {
            pcVar11 = (char *)(param_2 + -0x215);
            pbVar2 = (byte *)(param_2 + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
          }
          iVar10 = -0x1;
          pcVar14 = pcVar11;
          do {
            if (iVar10 == 0x0) break;
            iVar10 = iVar10 + -0x1;
            pcVar5 = pcVar14;
            pcVar14 = pcVar14 + 0x1;
          } while (*pcVar5 != '\0');
          pcVar11 = pcVar14 + (-0x1 - (int)pcVar11);
        }
      }
    }
  }
LAB_1000_3444:
  if ((*(byte *)(param_2 + -0x6) & 0x40) != 0x0) {
    if ((*(byte *)(param_2 + -0x5) & 0x1) == 0x0) {
      if ((*(byte *)(param_2 + -0x6) & 0x1) == 0x0) {
        if ((*(byte *)(param_2 + -0x6) & 0x2) != 0x0) {
          *(undefined *)(param_2 + -0x10) = 0x20;
          *(undefined2 *)(param_2 + -0x12) = 0x1;
        }
      }
      else {
        *(undefined *)(param_2 + -0x10) = 0x2b;
        *(undefined2 *)(param_2 + -0x12) = 0x1;
      }
    }
    else {
      *(undefined *)(param_2 + -0x10) = 0x2d;
      *(undefined2 *)(param_2 + -0x12) = 0x1;
    }
  }
  iVar8 = *(int *)(param_2 + -0xc) - (int)pcVar11;
  iVar10 = *(int *)(param_2 + -0x12);
  iVar9 = iVar8 - iVar10;
  if (iVar8 < iVar10) {
    iVar9 = 0x0;
  }
  if ((*(byte *)(param_2 + -0x6) & 0xc) == 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534(*(undefined2 *)(param_2 + -0x12),param_2,param_5);
  if (((*(byte *)(param_2 + -0x6) & 0x8) != 0x0) && ((*(byte *)(param_2 + -0x6) & 0x4) == 0x0)) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534(pcVar11,param_2,param_5);
  if ((*(byte *)(param_2 + -0x6) & 0x4) != 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
LAB_1000_30cf:
  pcVar5 = *(char **)(param_2 + 0xa);
  cVar4 = *pcVar5;
  *(int *)(param_2 + 0xa) = (int)pcVar5 + 0x1;
  *(char *)(param_2 + -0x4) = cVar4;
  if ((cVar4 != '\0') && (-0x1 < *(int *)(param_2 + -0xa))) {
    if ((byte)(cVar4 - 0x20U) < 0x59) {
      bVar6 = *(byte *)(ulong)((byte)(cVar4 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar6 = 0x0;
    }
    bVar6 = *(byte *)(ulong)((byte)(bVar6 * '\b' + *(char *)(param_2 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_2 + -0x7) = bVar6;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar7 = (**(code **)((char)bVar6 * 0x2 + 0x30a4))();
    return uVar7;
  }
  return *(ushort *)(param_2 + -0xa);
}



ushort __cdecl16near pass1_1000_34cf(int param_1,ushort param_2)

{
  ushort uVar1;
  ushort *puVar2;
  
  puVar2 = *(ushort **)(param_1 + 0xe);
  uVar1 = *puVar2;
  *(int *)(param_1 + 0xe) = (int)puVar2 + 0x2;
  return uVar1;
}



ulong __cdecl16near pass1_1000_34d8(int param_1,ushort param_2)

{
  undefined2 uVar1;
  undefined2 uVar2;
  undefined2 *puVar3;
  
  puVar3 = (undefined2 *)*(undefined4 *)(param_1 + 0xe);
  uVar1 = *puVar3;
  uVar2 = *(undefined2 *)((int)puVar3 + 0x2);
  *(int *)(param_1 + 0xe) = (int)puVar3 + 0x4;
  return CONCAT22(uVar2,uVar1);
}



ulong __cdecl16near pass1_1000_34e6(uint param_1,int param_2,ushort param_3)

{
  ushort uVar1;
  ulong uVar2;
  
  if ((*(byte *)(param_2 + -0x6) & 0x20) != 0x0) {
    uVar2 = pass1_1000_34d8(param_2,param_3);
    return uVar2;
  }
  uVar1 = pass1_1000_34cf(param_2,param_3);
  if (uVar1 == 0x0) {
    return (ulong)param_1 << 0x10;
  }
  return CONCAT22(param_1,uVar1);
}


void __cdecl16near pass1_1000_3534(int param_1,int param_2,ushort param_3)

{
  int *piVar1;
  undefined *puVar2;
  uint uVar3;
  undefined2 in_DX;
  undefined *unaff_DI;
  uint uVar4;
  undefined2 unaff_ES;
  undefined2 unaff_CS;
  undefined in_AF;
  
  if (param_1 != 0x0) {
    piVar1 = (int *)(param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar4 = 0x0;
    do {
      puVar2 = unaff_DI;
      unaff_DI = unaff_DI + 0x1;
      uVar3 = pass1_1000_3503(*puVar2,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar4 = uVar4 | uVar3;
      param_1 = param_1 + -0x1;
    } while (param_1 != 0x0);
    if (uVar4 != 0x0) {
      *(undefined2 *)(param_2 + -0xa) = 0xffff;
    }
  }
  return;
}
