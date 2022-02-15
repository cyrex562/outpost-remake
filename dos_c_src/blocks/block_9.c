
// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

uint __cdecl16far dos3_call_1000_51aa(ushort param_1)

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



ulong __stdcall16far pass1_1000_5224(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  bool bVar10;
  char cVar11;
  uint uVar9;
  
  cVar11 = (int)param_2 < 0x0;
  if ((bool)cVar11) {
    bVar10 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -(uint)bVar10 - param_2;
  }
  if ((int)param_4 < 0x0) {
    cVar11 = cVar11 + '\x01';
    bVar10 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -(uint)bVar10 - param_4;
  }
  uVar3 = param_1;
  uVar5 = param_3;
  uVar6 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) / (ulong)param_3);
  }
  else {
    do {
      uVar8 = uVar9 >> 0x1;
      uVar5 = uVar5 >> 0x1 | (uint)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar6 & 0x1) != 0x0) << 0xf;
      uVar6 = uVar7;
      uVar9 = uVar8;
    } while (uVar8 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (ulong)uVar5;
    iVar4 = (int)uVar1;
    lVar2 = (ulong)param_3 * (uVar1 & 0xffff);
    uVar3 = (uint)((ulong)lVar2 >> 0x10);
    uVar5 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 < (uint)lVar2)))) {
      iVar4 = iVar4 + -0x1;
    }
    uVar3 = 0x0;
  }
  if (cVar11 == '\x01') {
    bVar10 = iVar4 != 0x0;
    iVar4 = -iVar4;
    uVar3 = -(uint)bVar10 - uVar3;
  }
  return CONCAT22(uVar3,iVar4);
}



ulong __stdcall16far pass1_1000_52be(uint param_1,uint param_2,uint param_3,uint param_4)

{
  if ((param_4 | param_2) == 0x0) {
    return (ulong)param_1 * (ulong)param_3;
  }
  return (ulong)param_1 * (ulong)param_3 & 0xffff |
         (ulong)((int)((ulong)param_1 * (ulong)param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}



ulong __stdcall16far pass1_1000_52f0(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  bool bVar12;
  bool bVar13;
  
  bVar13 = (int)param_2 < 0x0;
  if (bVar13) {
    bVar12 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -(uint)bVar12 - param_2;
  }
  uVar11 = (uint)bVar13;
  if ((int)param_4 < 0x0) {
    bVar13 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -(uint)bVar13 - param_4;
  }
  uVar3 = param_1;
  uVar4 = param_3;
  uVar8 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    iVar5 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) % (ulong)param_3);
    iVar6 = 0x0;
    if ((int)(uVar11 - 0x1) < 0x0) goto LAB_1000_538a;
  }
  else {
    do {
      uVar10 = uVar9 >> 0x1;
      uVar4 = uVar4 >> 0x1 | (uint)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar8 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar8 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar7;
      uVar9 = uVar10;
    } while (uVar10 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (ulong)uVar4;
    uVar3 = (int)uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * (ulong)param_3;
    uVar8 = (uint)((ulong)lVar2 >> 0x10);
    uVar4 = (uint)lVar2;
    uVar9 = uVar8 + uVar3;
    if (((CARRY2(uVar8,uVar3)) || (param_2 < uVar9)) || ((param_2 <= uVar9 && (param_1 < uVar4)))) {
      bVar13 = uVar4 < param_3;
      uVar4 = uVar4 - param_3;
      uVar9 = (uVar9 - param_4) - (uint)bVar13;
    }
    iVar5 = uVar4 - param_1;
    iVar6 = (uVar9 - param_2) - (uint)(uVar4 < param_1);
    if (-0x1 < (int)(uVar11 - 0x1)) goto LAB_1000_538a;
  }
  bVar13 = iVar5 != 0x0;
  iVar5 = -iVar5;
  iVar6 = -(uint)bVar13 - iVar6;
LAB_1000_538a:
  return CONCAT22(iVar6,iVar5);
}



ulong __stdcall16far pass1_1000_5390(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  
  uVar3 = param_1;
  uVar8 = param_4;
  uVar6 = param_2;
  uVar9 = param_3;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) / (ulong)param_3);
  }
  else {
    do {
      uVar5 = uVar8 >> 0x1;
      uVar9 = uVar9 >> 0x1 | (uint)((uVar8 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar6 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar5;
      uVar6 = uVar7;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (ulong)uVar9;
    iVar4 = (int)uVar1;
    lVar2 = (ulong)param_3 * (uVar1 & 0xffff);
    uVar3 = (uint)((ulong)lVar2 >> 0x10);
    uVar8 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar8)) || ((param_2 <= uVar8 && (param_1 < (uint)lVar2)))) {
      iVar4 = iVar4 + -0x1;
    }
    uVar3 = 0x0;
  }
  return CONCAT22(uVar3,iVar4);
}



ulong __stdcall16far pass1_1000_53f0(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  bool bVar11;
  
  uVar3 = param_1;
  uVar4 = param_4;
  uVar9 = param_2;
  uVar10 = param_3;
  if (param_4 == 0x0) {
    iVar6 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) % (ulong)param_3);
    iVar7 = 0x0;
  }
  else {
    do {
      uVar5 = uVar4 >> 0x1;
      uVar10 = uVar10 >> 0x1 | (uint)((uVar4 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar9 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar4 = uVar5;
      uVar9 = uVar8;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar8,uVar3) / (ulong)uVar10;
    uVar3 = (int)uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * (ulong)param_3;
    uVar9 = (uint)((ulong)lVar2 >> 0x10);
    uVar4 = (uint)lVar2;
    uVar10 = uVar9 + uVar3;
    if (((CARRY2(uVar9,uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
      bVar11 = uVar4 < param_3;
      uVar4 = uVar4 - param_3;
      uVar10 = (uVar10 - param_4) - (uint)bVar11;
    }
    iVar6 = -(uVar4 - param_1);
    iVar7 = -(uint)(uVar4 - param_1 != 0x0) - ((uVar10 - param_2) - (uint)(uVar4 < param_1));
  }
  return CONCAT22(iVar7,iVar6);
}



int __cdecl16far pass1_1000_545a(ulong param_1,ulong param_2)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  byte *pbVar5;
  byte *pbVar6;
  
  pbVar6 = (byte *)param_2;
  pbVar5 = (byte *)param_1;
  bVar4 = 0xff;
  do {
    do {
      if (bVar4 == 0x0) goto LAB_1000_5499;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar4 = *pbVar1;
      bVar3 = *pbVar5;
      pbVar5 = pbVar5 + 0x1;
    } while (bVar3 == bVar4);
    bVar2 = bVar4 + 0xbf + (-((byte)(bVar4 + 0xbf) < 0x1a) & 0x20U) + 0x41;
    bVar3 = bVar3 + 0xbf;
    bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20U) + 0x41;
  } while (bVar4 == bVar2);
  bVar4 = (bVar4 < bVar2) * -0x2 + 0x1;
LAB_1000_5499:
  return (int)(char)bVar4;
}



uint * __cdecl16far pass1_1000_54a0(ulong param_1,uint param_2,uint param_3)

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
    iVar8 = (int)(param_1 >> 0x10);
    uVar5 = -(int)(uint *)param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(uint)(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = param_2 & 0xff | param_2 << 0x8;
    puVar7 = (uint *)param_1;
    for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 = uVar4 - 0x1) {
      puVar1 = puVar7;
      puVar7 = puVar7 + 0x1;
      *puVar1 = uVar3;
    }
    for (uVar6 = (uint)((uVar6 & 0x1) != 0x0); uVar2 = (undefined)(param_2 & 0xff), uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
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



void __stdcall16far pass1_1000_54e8(uchar *param_1,ushort param_2,int param_3,int param_4,int param_5,ushort param_6)

{
  int iVar1;
  
  iVar1 = param_3;
  while (iVar1 = iVar1 + -0x1, -0x1 < iVar1) {
    (*(code *)param_1)();
  }
  return;
}



void __stdcall16far pass1_1000_5512(uchar *param_1,ushort param_2,int param_3,int param_4,ushort param_5)

{
  bool bVar1;
  ushort uStack4;
  
  pass1_1000_52be(param_3,param_4,param_5,0x0);
  while( true ) {
    bVar1 = param_3 == 0x0;
    param_3 = param_3 + -0x1;
    param_4 = param_4 - (uint)bVar1;
    if (param_4 < 0x0) break;
    uStack4 = param_2;
    (*(code *)param_1)();
  }
  return;
}



void __stdcall16far pass1_1000_5586(uchar *param_1,ushort param_2,int param_3,int param_4,int param_5,ushort param_6)

{
  int iVar1;
  
  iVar1 = param_3;
  while (iVar1 = iVar1 + -0x1, -0x1 < iVar1) {
    (*(code *)param_1)();
  }
  return;
}



void __cdecl16near ret_op_1000_55ac(void)

{
  return;
}



int * pass1_1000_55b1(int param_1,uint16_t param_2,uint16_t param_3)

{
  int *piVar2;
  char *pcVar3;
  LPCSTR str;
  int *piVar4;
  int *piVar5;
  char *pcVar6;
  int iVar7;
  int iVar8;
  char *piVar1;
  
  iVar8 = 0x14;
  iVar7 = 0x14;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar7,param_2,param_3);
  str = poss_str_op_1000_28dc(iVar8);
  if (str != (PCHAR)0x0) {
    iVar7 = 0x9;
    if (*str == 'M') {
      iVar7 = 0xf;
    }
    str = str + iVar7;
    iVar7 = 0x22;
    pcVar6 = str;
    do {
      if (iVar7 == 0x0) break;
      iVar7 = iVar7 + -0x1;
      pcVar3 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar6[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar5 = (int *)&PTR_LOOP_1050_63fe;
  do {
    piVar2 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar7 = *piVar2;
    piVar4 = piVar5;
    if ((iVar7 == param_1) || (piVar4 = (int *)(iVar7 + 0x1), piVar4 == (int *)0x0)) {
      return piVar4;
    }
    iVar7 = -0x1;
    do {
      if (iVar7 == 0x0) break;
      iVar7 = iVar7 + -0x1;
      piVar1 = (char *)piVar5;
      piVar5 = (int *)((int)piVar5 + 0x1);
    } while (*piVar1 != '\0');
  } while( true );
}



void __stdcall16far struct_op_1008_0000(ushort *param_1)

{
  int iVar1;
  ushort uVar2;
  
                    // Segment:    2
                    // Offset:     000060e0
                    // Length:     efe0
                    // Min Alloc:  efe0
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x52a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *(undefined4 *)(iVar1 + 0x4) = 0x0;
  *(undefined4 *)(iVar1 + 0x8) = 0x0;
  *param_1 = 0x51e;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_0036(ushort *param_1,ushort param_2)

{
  uint uVar1;
  undefined4 *puVar2;
  astruct_18 *paVar3;
  code **ppcVar4;
  uint uVar5;
  astruct_449 *iVar6;
  undefined2 uVar6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_449 *)param_1;
  *param_1 = 0x51e;
  iVar6->field_0x2 = 0x1008;
  paVar3 = *(astruct_18 **)&iVar6->field_0x8;
  uVar1 = iVar6->field_0xa;
  uVar5 = (uint)paVar3;
  if ((uVar1 | uVar5) != 0x0) {
    pass1_1008_53aa(uVar5,uVar1);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5748;
  _PTR_LOOP_1050_0298 = 0x0;
  if (_PTR_LOOP_1050_5748 != (astruct_18 *)0x0) {
    pass1_1030_8210(&_PTR_LOOP_1050_5748->field_0x0);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_0ed0;
  if (_PTR_LOOP_1050_0ed0 != (astruct_18 *)0x0) {
    pass1_1010_2050((ulong)_PTR_LOOP_1050_0ed0);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_14cc;
  if (_PTR_LOOP_1050_14cc != (astruct_18 *)0x0) {
    pass1_1010_7efc((ulong *)_PTR_LOOP_1050_14cc,0x1010);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5b7c;
  if (_PTR_LOOP_1050_5b7c != (astruct_18 *)0x0) {
    pass1_1038_af34();
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  if (_PTR_LOOP_1050_5bc8 != (undefined4 *)0x0) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_5bc8;
    (**ppcVar4)(param_2,(int)_PTR_LOOP_1050_5bc8,(int)((ulong)_PTR_LOOP_1050_5bc8 >> 0x10),0x1);
  }
  if (_PTR_LOOP_1050_02a0 != (undefined4 *)0x0) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_02a0;
    (**ppcVar4)(param_2,(int)_PTR_LOOP_1050_02a0,(int)((ulong)_PTR_LOOP_1050_02a0 >> 0x10),0x1);
  }
  puVar2 = iVar6->field_0x4;
  uVar1 = iVar6->field_0x6;
  if ((uVar1 | (uint)puVar2) != 0x0) {
    ppcVar4 = (code **)*puVar2;
    (**ppcVar4)(param_2,puVar2,uVar1,0x1);
  }
  pass1_1008_9466(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mixed_win_sys_op_1008_016e(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  undefined2 *puVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  uchar *puVar6;
  uchar *extraout_DX;
  uchar *puVar7;
  uint uVar8;
  int unaff_DI;
  undefined2 uVar9;
  HINSTANCE16 instance;
  undefined2 uVar10;
  DWORD DVar11;
  ulong *puVar12;
  ulong uVar13;
  astruct_20 *paVar14;
  CHAR local_1be [0x80];
  CHAR local_13e [0xac];
  CHAR local_92 [0x80];
  uint uStack18;
  uchar *puStack16;
  undefined4 *puStack14;
  uint uStack10;
  uint uStack8;
  undefined2 uStack6;
  uchar *puStack4;
  
  instance = (HINSTANCE16)s_tile2_bmp_1050_1538;
  DVar11 = GetVersion16();
  puVar7 = (uchar *)(DVar11 >> 0x10);
  uStack6 = (undefined2)(DVar11 & 0xffff);
  uVar4 = (uint)DVar11 & 0xff;
  uStack10 = (uint)(byte)((DVar11 & 0xffff) >> 0x8);
  uStack8 = uVar4;
  puStack4 = puVar7;
  if ((uVar4 < 0x3) || ((uVar4 == 0x3 && (uStack10 < 0xa)))) {
    uVar10 = 0x1000;
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    puVar6 = (uchar *)((uint)puVar7 | uVar4);
    uStack18 = uVar4;
    puStack16 = puVar7;
    if (puVar6 == (uchar *)0x0) {
      iVar3 = 0x0;
      puVar6 = (uchar *)0x0;
    }
    else {
      uVar10 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520((astruct_57 *)CONCAT22(puVar7,uVar4),0x0,0x10,0x2,0x5de,0x5dd,puVar6,param_2);
    }
    puStack14 = (undefined4 *)CONCAT22(puVar6,iVar3);
    ppcVar1 = (code **)((int)*puStack14 + 0x74);
    (**ppcVar1)(uVar10,iVar3,(char)puVar6);
    instance = 0x1000;
    puVar7 = extraout_DX;
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  debug_print_1008_6048((ulong)s_version__d__d_1050_0012,instance,param_2);
  if ((uStack8 == 0x3) && (0xb < (int)uStack10)) {
    PTR_LOOP_1050_0010 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  }
  LoadString16(instance,0x80,local_92,param_2);
  uVar4 = dos3_call_1000_51aa((ushort)&stack0xfffe);
  if (uVar4 != 0x0) {
    LoadString16(0x1000,0x80,local_13e,param_2);
    LoadString16((HINSTANCE16)s_tile2_bmp_1050_1538,0x80,local_1be,param_2);
    uVar4 = MessageBox16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,local_13e,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0x4,puVar7,0x1000);
  if (((uint)puVar7 | uVar4) == 0x0) {
    uVar10 = 0x0;
    puVar6 = (uchar *)0x0;
    uStack18 = uVar4;
    puStack16 = puVar7;
  }
  else {
    uStack18 = uVar4;
    puStack16 = puVar7;
    puVar12 = pass1_1008_5394((ulong *)CONCAT22(puVar7,uVar4));
    puVar6 = (uchar *)((ulong)puVar12 >> 0x10);
    uVar10 = SUB42(puVar12,0x0);
  }
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(undefined2 *)(iVar3 + 0x8) = uVar10;
  *(uchar **)(iVar3 + 0xa) = puVar6;
  uVar5 = *(undefined4 *)(iVar3 + 0x8);
  puVar2 = (undefined2 *)*(undefined4 *)(iVar3 + 0x8);
  _PTR_LOOP_1050_0298 = uVar5;
  *puVar2 = 0x70;
  *(undefined2 *)((int)puVar2 + 0x2) = (int)s_tile2_bmp_1050_1538;
  uVar10 = 0x1000;
  mem_op_1000_179c(0x126,puVar6,0x1000);
  uVar4 = (uint)uVar5;
  puVar7 = (uchar *)((uint)puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    uVar10 = 0x1010;
    uVar13 = pass1_1010_2024(CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12((char)puVar6,uVar4)));
    puVar7 = (uchar *)(uVar13 >> 0x10);
    uVar4 = (uint)uVar13;
  }
  if (_PTR_LOOP_1050_0ed0 == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op_1050_0020,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xe8c,puVar7,0x1000);
  puVar6 = (uchar *)((uint)puVar7 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (puVar6 != (uchar *)0x0) {
    uVar10 = 0x1010;
    pass1_1010_7e40((ulong *)CONCAT22(puVar7,uVar4),puVar6,unaff_DI,param_2);
  }
  if (_PTR_LOOP_1050_14cc == 0x0) {
    debug_print_1008_6048(0x10500035,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xb0,puVar6,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
    paVar14 = pass1_1038_aeca((astruct_20 *)CONCAT22(puVar6,uVar4),param_2);
    puVar7 = (uchar *)((ulong)paVar14 >> 0x10);
    uVar4 = (uint)paVar14;
  }
  if (_PTR_LOOP_1050_5b7c == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__DialogCtr_1050_0053,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xa,puVar7,0x1000);
  puVar6 = (uchar *)((uint)puVar7 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (puVar6 != (uchar *)0x0) {
    uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
    make_proc_inst_1038_cf6c((ushort *)CONCAT22(puVar7,uVar4),puVar6,(int)&PTR_LOOP_1050_1038);
  }
  if (_PTR_LOOP_1050_5bc8 == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__DialogHand_1050_0073,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    pass1_1008_5bdc((astruct_79 *)CONCAT22(puVar6,uVar4),unaff_DI,param_2);
  }
  if (_PTR_LOOP_1050_02a0 == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__Simulator_1050_0097,0x1000,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0xfc,puVar7,0x1000);
  uVar8 = (uint)puVar7 | uVar4;
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (uVar8 == 0x0) {
    uVar4 = 0x0;
    uVar8 = 0x0;
  }
  else {
    set_struct_op_1008_0536((ushort *)CONCAT22(puVar7,uVar4),0x1000,param_2);
  }
  *(uint *)(iVar3 + 0x4) = uVar4;
  *(uint *)(iVar3 + 0x6) = uVar8;
  if (*(long *)(iVar3 + 0x4) == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op_1050_00b7,0x1000,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  win_ui_reg_class_1008_96d2(*(astruct_20 **)(iVar3 + 0x4),0x1000,param_2);
  uVar5 = *(undefined4 *)(iVar3 + 0x4);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0x8);
  (**ppcVar1)(0x1000,(int)uVar5,(int)((ulong)uVar5 >> 0x10));
  uVar5 = *(undefined4 *)(iVar3 + 0x4);
  PTR_LOOP_1050_0396 = (undefined *)*(undefined2 *)((int)uVar5 + 0x8);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1000,*(undefined4 *)(iVar3 + 0x4),0x3);
  UpdateWindow16(0x1000);
  return;
}



void __stdcall16far pass1_1008_049c(ushort param_1,ushort param_2,char *param_3)

{
  uint uVar1;
  undefined *puVar2;
  
  if (param_3 != (char *)0x0) {
    uVar1 = str_op_1000_3da4(param_3);
    if (uVar1 != 0x0) {
      puVar2 = (undefined *)pass1_1000_545a((ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x1),0x105000cc);
      if (puVar2 == (undefined *)0x0) {
        PTR_LOOP_1050_02ec = puVar2;
      }
    }
  }
  return;
}



ushort * __stdcall16far pass1_1008_04d2(ushort *param_1,byte param_2)

{
  pass1_1008_9466(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_04f8(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1008_0036(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far set_struct_op_1008_0536(ushort *param_1,HINSTANCE16 param_2,ushort param_3)

{
  HICON16 HVar1;
  HCURSOR16 HVar2;
  HGDIOBJ16 HVar3;
  uchar *puVar4;
  int iVar5;
  int unaff_DI;
  undefined2 uVar6;
  astruct_20 *paVar7;
  ushort *puVar8;
  
  paVar7 = pass1_1008_3ab8((astruct_20 *)param_1);
  puVar4 = (uchar *)((ulong)paVar7 >> 0x10);
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *(undefined4 *)(iVar5 + 0xe0) = 0x0;
  *(undefined4 *)(iVar5 + 0xe4) = 0x0;
  *(undefined4 *)(iVar5 + 0xe8) = 0x0;
  *(undefined2 *)(iVar5 + 0xec) = 0x0;
  *(undefined4 *)(iVar5 + 0xee) = 0x0;
  *(undefined2 *)(iVar5 + 0xf2) = 0x0;
  *(undefined4 *)(iVar5 + 0xf4) = 0x0;
  *(undefined4 *)(iVar5 + 0xf8) = 0x0;
  *param_1 = 0x389e;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  *(undefined2 *)(iVar5 + 0xc8) = 0x2008;
  *(undefined2 *)(iVar5 + 0xac) = 0x0;
  *(undefined2 *)(iVar5 + 0xae) = 0x8700;
  HVar1 = LoadIcon16(param_2,(LPCSTR)0xd4);
  *(HICON16 *)(iVar5 + 0xc2) = HVar1;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  *(HCURSOR16 *)(iVar5 + 0xc4) = HVar2;
  HVar3 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  *(HGDIOBJ16 *)(iVar5 + 0xc6) = HVar3;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_3,puVar4,unaff_DI);
  puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar5 + 0xa)),s_Outpost_1050_00d7);
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_3,puVar4,unaff_DI);
  *(undefined2 *)(iVar5 + 0xf4) = (int)puVar8;
  *(undefined2 *)(iVar5 + 0xf6) = (int)((ulong)puVar8 >> 0x10);
  set_sys_color_1008_357e((ulong)param_1,0x1,0x1010,param_3);
  return;
}



void __stdcall16far cleanup_ui_op_1008_0618(undefined2 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  astruct_18 *paVar3;
  code **ppcVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 unaff_CS;
  HICON16 h_icon;
  ushort unaff_SS;
  undefined2 uVar7;
  undefined2 uVar8;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = 0x389e;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  set_sys_color_1008_357e((ulong)param_1,0x0,unaff_CS,unaff_SS);
  paVar3 = *(astruct_18 **)(iVar5 + 0xf8);
  uVar8 = (undefined2)((ulong)paVar3 >> 0x10);
  h_icon = 0x1000;
  fn_ptr_1000_17ce(paVar3,0x1000);
  if (*(int *)(iVar5 + 0xec) != 0x0) {
    uVar8 = *(undefined2 *)(iVar5 + 0xec);
    h_icon = (HICON16)s_tile2_bmp_1050_1538;
    DestroyMenu16(0x1000);
  }
  uVar7 = *(undefined2 *)(iVar5 + 0xc2);
  DestroyIcon16(h_icon);
  *(undefined2 *)(iVar5 + 0xc2) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar5 + 0xe0);
  uVar2 = *(uint *)(iVar5 + 0xe2);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)((int)s_tile2_bmp_1050_1538,puVar1,uVar2,0x1,uVar7,uVar8);
  }
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar5 + 0xd2)));
  *param_1 = 0x380a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_cursor_op_1008_06c0(ulong *param_1,ulong param_2,ushort param_3,int param_4)

{
  code **ppcVar1;
  uint in_AX;
  uint in_DX;
  uchar *puVar2;
  uchar *extraout_DX;
  int unaff_DI;
  undefined2 uVar3;
  uchar *unaff_SS;
  undefined in_AF;
  char *pcVar4;
  ushort *puVar5;
  uchar local_5a [0x50];
  undefined4 uStack10;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;
  
  if (param_4 == 0x400) {
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x4000001);
    puVar2 = (uchar *)(in_DX | in_AX);
    if (puVar2 != (uchar *)0x0) {
      if (PTR_LOOP_1050_4fe8 != (undefined *)0x0) {
        pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        MessageBox16(0x1010,(LPCSTR)&PTR_LOOP_1050_0010,(LPCSTR)pcVar4,(UINT16)((ulong)pcVar4 >> 0x10));
        return;
      }
      HStack4 = LoadCursor16(0x1030,(LPCSTR)0x7f02);
      HStack6 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
      pass1_1030_83ba(_PTR_LOOP_1050_5748,param_2,unaff_SS,in_AF);
      uVar3 = (undefined2)((ulong)_PTR_LOOP_1050_5748 >> 0x10);
      *(undefined2 *)((int)_PTR_LOOP_1050_5748 + 0x8) = 0x1;
      uStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,(ushort)unaff_SS,puVar2,unaff_DI);
      pass1_1018_262e((ulong)uStack10);
      pass1_1030_8326();
      pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      sys_1000_3f9c(local_5a,unaff_SS,0x109,(ushort)&USHORT_1050_1050,(ushort)pcVar4,&stack0xfffe,uVar3,0x1000,unaff_SS,
                    in_AF);
      ppcVar1 = (code **)((int)*param_1 + 0x14);
      (**ppcVar1)(0x1000,(int)param_1,(char)((ulong)param_1 >> 0x10),0x0,local_5a);
      puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,(ushort)unaff_SS,extraout_DX,unaff_DI);
      pass1_1008_a9ec((ulong)puVar5);
      SetCursor16(0x1010);
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100fc);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far pass1_1008_07d8(ushort param_1,BOOL16 param_2,uchar *param_3,ushort param_4)

{
  ushort uVar2;
  ushort uVar1;
  uchar in_AF;
  ulong uVar3;
  
  if (_PTR_LOOP_1050_5748 == (ulong *)0x0) {
    uVar1 = 0x1000;
    mem_op_1000_179c(0xa,param_3,0x1000);
    uVar2 = (uint)param_3 | param_2;
    if (uVar2 != 0x0) {
      uVar1 = 0x1030;
      struct_1030_8128((ulong *)CONCAT22(param_3,param_2),uVar2,param_4);
    }
    if (_PTR_LOOP_1050_5748 == (ulong *)0x0) {
      debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__Simulator_1050_0110,uVar1,param_4);
      fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
    }
    uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,uVar2,0x8);
    uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)(uVar3 >> 0x10),0x8);
    pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)(uVar3 >> 0x10),0xff);
    pass1_1030_838e(_PTR_LOOP_1050_5748,param_4,in_AF);
    param_2 = pass1_1030_8334(_PTR_LOOP_1050_5748);
  }
  return param_2;
}

