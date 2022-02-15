
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



// WARNING: Could not reconcile some variable overlaps

void __cdecl16far
pass1_1000_4aea(uint param_1,uint param_2,int param_3,uint param_4,uchar *param_5,int param_6,int param_7,ushort param_8
               ,ushort param_9,ushort param_10)

{
  uint *puVar1;
  code **ppcVar2;
  long lVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  astruct_171 *puVar11;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  bool bVar12;
  uint uStack26;
  uint uStack24;
  uint uStack22;
  uint uVar13;
  uint uVar14;
  undefined4 uStack18;
  uint uStack14;
  
  if ((param_4 != 0x0) && (param_3 != 0x0)) {
    uStack14 = param_1;
    uVar11 = param_2;
    for (iVar6 = param_3 + -0x1; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
      uVar9 = uStack14 + param_4;
      uVar11 = uVar11 + (-(uint)CARRY2(uStack14,param_4) & 0x6c);
      uStack18 = CONCAT22(uVar11,uVar9);
      iVar5 = (*(code *)param_5)(param_9);
      if (iVar5 < 0x0) {
        uVar11 = param_3 - 0x1;
        iVar6 = 0x0;
        do {
          uVar11 = uVar11 >> 0x1;
          iVar6 = iVar6 + -0x1;
        } while (iVar6 != 0x0 && uVar11 != 0x0);
        if (((int)((ulong)(uint)-iVar6 * 0x8 >> 0x10) != 0x0) ||
           (uVar11 = pass1_1000_3bac(), uVar11 < (uint)((ulong)(uint)-iVar6 * 0x8))) {
          exit_1000_25f2(0x4b7b,param_9,param_7,-0x4,param_8,param_9,param_10);
          return;
        }
        puVar11 = (astruct_171 *)&stack0xfff6;
        lVar3 = (ulong)(param_3 - 0x1) * (ulong)param_4;
        uVar11 = (uint)lVar3;
        uStack14 = uVar11 + param_1;
        uVar11 = ((int)((ulong)lVar3 >> 0x10) + (uint)CARRY2(uVar11,param_1)) * 0x100 + param_2;
        uStack18._0_2_ = param_1;
        uStack18._2_2_ = param_2;
LAB_1000_4b7d:
        if (puVar11 <= (astruct_171 *)&uStack18) {
          return;
        }
LAB_1000_4b81:
        if ((uStack18._2_2_ < uVar11) || ((uStack18._2_2_ <= uVar11 && ((uint)uStack18 < uStack14)))) {
          uStack22 = uStack14;
          puVar1 = &puVar11->field_0x14;
          uVar8 = uStack14 + *puVar1;
          uVar7 = uVar11 + (-(uint)CARRY2(uStack14,*puVar1) & 0x6c);
          uVar9 = uStack18._2_2_;
          uVar10 = (uint)uStack18;
          uStack26 = (uint)uStack18;
          uStack24 = uStack18._2_2_;
          uVar13 = uVar11;
LAB_1000_4bbc:
          do {
            puVar1 = &puVar11->field_0x14;
            bVar12 = CARRY2(uVar10,*puVar1);
            uVar10 = uVar10 + *puVar1;
            uVar9 = uVar9 + (-(uint)bVar12 & 0x6c);
            uVar4 = uStack22;
            if ((uVar10 != uStack14) || (uVar9 != uVar11)) {
              ppcVar2 = (code **)&puVar11->field_0x16;
              iVar6 = (**ppcVar2)(param_9,uVar10,uVar9,(uint)uStack18,uStack18._2_2_);
              if (iVar6 < 0x1) {
                if (iVar6 != 0x0) {
                  uStack26 = uVar10;
                  uStack24 = uVar9;
                }
                goto LAB_1000_4bbc;
              }
            }
            do {
              uVar14 = uVar13;
              uStack22 = uVar4;
              puVar1 = &puVar11->field_0x14;
              bVar12 = uVar8 < *puVar1;
              uVar8 = uVar8 - *puVar1;
              uVar7 = uVar7 - (-(uint)bVar12 & 0x6c);
              ppcVar2 = (code **)&puVar11->field_0x16;
              iVar6 = (**ppcVar2)(param_9,(uint)uStack18,uStack18._2_2_,uVar8,uVar7);
              if (0x0 < iVar6) break;
              uVar4 = uVar8;
              uVar13 = uVar7;
            } while (((iVar6 != 0x0) || (uVar4 = uStack22, uVar13 = uVar14, uVar8 != (uint)uStack18)) ||
                    (uVar7 != uStack18._2_2_));
            if ((uVar7 < uVar9) || ((uVar7 <= uVar9 && (uVar8 <= uVar10)))) goto LAB_1000_4c58;
            pass1_1000_4ceb(puVar11->field_0x14,uVar10,uVar8,uVar7);
            uStack26 = uVar10;
            uStack24 = uVar9;
            uVar13 = uVar7;
            uStack22 = uVar8;
          } while( true );
        }
        goto LAB_1000_4b7d;
      }
      uStack14 = uVar9;
    }
  }
  return;
LAB_1000_4c58:
  pass1_1000_4ceb(puVar11->field_0x14,(uint)uStack18,uVar8,uVar7);
  uVar10 = ((uVar11 - (-(uint)(uStack14 < uStack22) & 0x6c)) - uVar14) +
           (-(uint)CARRY2(uStack14 - uStack22,(uint)uStack18) & 0x6c) + uStack18._2_2_;
  uVar9 = -(uint)((uStack14 - uStack22) + (uint)uStack18 < uStack26) & 0x6c;
  if ((uVar10 < uVar9) || (uVar10 - uVar9 < uStack24)) {
    uStack14 = uStack26;
    uVar11 = uStack24;
  }
  else {
    uStack18._0_2_ = uStack22;
    uStack18._2_2_ = uVar14;
  }
  goto LAB_1000_4b81;
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



void __cdecl16far str_1000_4d58(char *in_string_1,char *in_string_2,ulong param_3,ulong param_4,WNDCLASS16 *param_5)

{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  undefined2 uVar4;
  uint uVar5;
  char *pcStack18;
  uint uStack12;
  uint uStack10;
  uint uStack8;
  uint uStack6;
  
  uStack10 = 0x0;
  uStack12 = 0x0;
  uVar4 = (undefined2)((ulong)in_string_1 >> 0x10);
  iVar2 = (int)in_string_1;
  if ((*in_string_1 == '\0') || (*(char *)(iVar2 + 0x1) != ':')) {
    if (in_string_2 != (char *)0x0) {
      *in_string_2 = '\0';
    }
  }
  else {
    if (in_string_2 != (char *)0x0) {
      *in_string_2 = *in_string_1;
      *(undefined *)((int)in_string_2 + 0x1) = *(undefined *)(iVar2 + 0x1);
      *(undefined *)((int)in_string_2 + 0x2) = 0x0;
    }
    in_string_1 = (char *)((ulong)in_string_1 & 0xffff0000 | (ulong)(iVar2 + 0x2));
  }
  uStack6 = 0x0;
  uStack8 = 0x0;
  pcStack18 = in_string_1;
  while( true ) {
    uVar5 = (uint)((ulong)pcStack18 >> 0x10);
    uVar3 = (uint)pcStack18;
    if (*pcStack18 == '\0') break;
    if ((*pcStack18 == '/') || (*pcStack18 == '\\')) {
      uStack8 = uVar3 + 0x1;
      uStack6 = uVar5;
    }
    else {
      if (*pcStack18 == '.') {
        uStack12 = uVar3;
        uStack10 = uVar5;
      }
    }
    pcStack18 = (char *)((ulong)pcStack18 & 0xffff0000 | (ulong)(uVar3 + 0x1));
  }
  if ((uStack6 | uStack8) == 0x0) {
    if (param_3 != 0x0) {
      *(undefined *)param_3 = 0x0;
    }
  }
  else {
    if (param_3 != 0x0) {
      uVar1 = uStack8 - (uint)in_string_1;
      if (0xff < (int)uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((char *)(param_3 & 0xffff | (ulong)param_3._2_2_ << 0x10),in_string_1,uVar1);
      *(undefined *)((int)param_3 + uVar1) = 0x0;
    }
    in_string_1 = (char *)CONCAT22(uStack6,uStack8);
  }
  if (((uStack10 | uStack12) != 0x0) && ((uint)in_string_1 <= uStack12)) {
    if (param_4 != 0x0) {
      uVar1 = uStack12 - (uint)in_string_1;
      if (0xff < (int)uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((char *)(param_4 & 0xffff | (ulong)param_4._2_2_ << 0x10),
                       (char *)((ulong)in_string_1 & 0xffff | (ulong)in_string_1._2_2_ << 0x10),uVar1);
      *(undefined *)((int)param_4 + uVar1) = 0x0;
    }
    if (param_5 == (WNDCLASS16 *)0x0) {
      return;
    }
    uVar1 = uVar3 - uStack12;
    if (0xff < (int)uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((char *)((ulong)param_5 & 0xffff | (ulong)param_5._2_2_ << 0x10),
                     (char *)CONCAT22(uStack10,uStack12),uVar1);
    *(undefined *)((int)param_5 + uVar1) = 0x0;
    return;
  }
  if (param_4 != 0x0) {
    uVar1 = uVar3 - (uint)in_string_1;
    if (0xff < (int)uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((char *)(param_4 & 0xffff | (ulong)param_4._2_2_ << 0x10),
                     (char *)((ulong)in_string_1 & 0xffff | (ulong)in_string_1._2_2_ << 0x10),uVar1);
    *(undefined *)((int)param_4 + uVar1) = 0x0;
  }
  if (param_5 != (WNDCLASS16 *)0x0) {
    *(undefined *)&param_5->style = 0x0;
  }
  return;
}



int * pass1_1000_4f1a(int param_1,uint16_t param_2,uint16_t param_3)

{
  int *piVar1;
  char *pcVar2;
  LPCSTR str;
  int *piVar3;
  int *piVar4;
  char *pcVar5;
  int iVar6;
  int iVar7;
  
  iVar7 = 0x19;
  iVar6 = 0x19;
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



// WARNING: Removing unreachable block (ram,0x10004f47)

ushort __cdecl16far dos3_call_1000_4f20(ushort param_1)

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



// WARNING: Removing unreachable block (ram,0x10004f47)

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



// WARNING: Removing unreachable block (ram,0x10004f6d)

ushort __cdecl16far dos3call_1000_4f54(ulong param_1,int param_2)

{
  char cVar1;
  code *pcVar2;
  uint uVar3;
  char *pcVar4;
  bool bVar5;
  undefined4 uVar6;
  
  bVar5 = false;
  pcVar2 = (code *)swi(0x21);
  uVar6 = (*pcVar2)((int)&USHORT_1050_1050,param_2 + 0x1);
  pcVar4 = (char *)((ulong)uVar6 >> 0x10);
  uVar3 = (uint)uVar6;
  if ((bVar5) && (bVar5 = uVar3 < 0x10, uVar3 == 0x10)) {
    do {
      cVar1 = *pcVar4;
      pcVar4 = pcVar4 + 0x1;
      if (cVar1 == '\0') goto LAB_1000_4f90;
    } while ((cVar1 != '?') && (cVar1 != '*'));
    uVar3 = 0x3;
LAB_1000_4f90:
    bVar5 = true;
  }
  if (!bVar5) {
    return 0x0;
  }
  pass1_1000_29b5(uVar3);
  return 0xffff;
}



// WARNING: Removing unreachable block (ram,0x10004fa9)

int __cdecl16far dos3_call_1000_4f94(ushort param_1)

{
  code *pcVar1;
  byte bVar2;
  
  pcVar1 = (code *)swi(0x21);
  bVar2 = (*pcVar1)(param_1 + 0x1);
  return bVar2 + 0x1;
}



// WARNING: Removing unreachable block (ram,0x10004fd7)
// WARNING: Removing unreachable block (ram,0x10004feb)

ushort __cdecl16far dos3_call_1000_4fbe(char param_1,int param_2)

{
  code *pcVar1;
  char cVar2;
  ushort uVar3;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)(param_2 + 0x1);
  pcVar1 = (code *)swi(0x21);
  cVar2 = (*pcVar1)();
  uVar3 = 0xffff;
  if ((char)(cVar2 + '\x01') == param_1) {
    uVar3 = 0x0;
  }
  return uVar3;
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



void __cdecl16far
pass1_1000_5026(int param_1,uint param_2,uint param_3,uint param_4,int param_5,ushort param_6,ushort param_7)

{
  uint uVar1;
  uint uVar2;
  ushort uStack304;
  ushort local_12c [0x3];
  ushort uStack294;
  undefined *local_124 [0x6];
  int iStack280;
  undefined local_116;
  undefined uStack277;
  char cStack272;
  undefined *puStack270;
  undefined local_108;
  undefined uStack263;
  undefined uStack262;
  undefined auStack261 [0x101];
  undefined2 local_4;
  int iStack2;
  
  iStack2 = param_5 + 0x1;
  local_4 = SUB42(&USHORT_1050_1050,0x0);
  _uStack304 = (char *)CONCAT22(param_7,&local_108);
  if (param_1 == 0x0) {
    param_1 = dos3_call_1000_4f94((ushort)&iStack2);
  }
  *_uStack304 = (char)param_1 + '@';
  uStack263 = 0x3a;
  puStack270 = auStack261;
  uStack262 = 0x5c;
  uStack277 = 0x47;
  cStack272 = (char)param_1;
  local_12c[0] = param_7;
  uStack294 = param_7;
  dos3_call_set_struct_1000_42de
            ((ushort *)CONCAT22(param_7,&local_116),(ushort *)CONCAT22(param_7,local_124),
             (ushort *)CONCAT22(param_7,local_12c));
  if (iStack280 == 0x0) {
    uVar1 = str_op_1000_3da4((char *)CONCAT22(param_7,&local_108));
    uVar1 = uVar1 + 0x1;
    uStack304 = param_2;
    uVar2 = param_3 | param_2;
    if (uVar2 == 0x0) {
      if ((int)param_4 < (int)uVar1) {
        param_4 = uVar1;
      }
      uStack304 = mem_1000_167a(param_4,param_6,0x0);
      param_3 = uVar2;
      if ((uVar2 | uStack304) == 0x0) {
        PTR_LOOP_1050_5f78 = (undefined *)&PTR_LOOP_1050_000c;
        return;
      }
    }
    if ((int)param_4 < (int)uVar1) {
      PTR_LOOP_1050_5f78 = (undefined *)((int)s_New_failed_in_Op__Op_1050_0020 + 0x2);
    }
    else {
      unk_str_op_1000_3d3e((char *)CONCAT22(param_3,uStack304),(char *)CONCAT22(param_7,&local_108));
    }
  }
  else {
    PTR_LOOP_1050_5f78 = (undefined *)((int)&PTR_LOOP_1050_000c + 0x1);
    PTR_LOOP_1050_5f88 = local_124[0];
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10005167)

ushort __cdecl16far dos3_call_1000_514e(int param_1)

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



// WARNING: Removing unreachable block (ram,0x1000518c)

uint __cdecl16far dos3_call_1000_5174(ushort param_1)

{
  code *pcVar1;
  uint uVar2;
  bool bVar3;
  
  bVar3 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)(param_1 + 0x1);
  if (!bVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return uVar2 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

uint __cdecl16far dos3_calls_1000_5198(int param_1)

{
  code *pcVar1;
  uint uVar2;
  byte bVar3;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&USHORT_1050_1050,param_1 + 0x1);
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  bVar3 = 0x0;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  if ((bVar3 & 0x1) == 0x0) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return uVar2 & 0xff;
}
