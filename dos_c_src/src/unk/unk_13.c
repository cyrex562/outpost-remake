

void __stdcall16far pass1_1010_0f76(ushort *param_1, ushort param_2)

{
    uint uVar1;

    uVar1                               = (uint)((ulong)param_1 >> 0x10);
    *param_1                            = (int)s_648_bmp_1050_1919 + 0x1;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    pass1_1010_17c0((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1010_2db2(param_1, param_2);
    return;
}

void __stdcall16far pass1_1010_1146(ulong param_1, ushort param_2, int param_3, ushort param_4)

{
    undefined4 uVar1;
    ushort     uVar2;

    DAT_1050_0ecc = param_2;
    uVar2         = (ushort)(param_1 >> 0x10);
    uVar1         = *(undefined4 *)((int)param_1 + 0x66);
    pass1_1000_4aea(*(uint *)((int)param_1 + 0x64), (uint)uVar1, (int)((ulong)uVar1 >> 0x10), 0x4, (uchar *)((int)s_dibtext_bmp_1050_1844 + 0x6), (int)&stack0xfffe, param_3, uVar2, 0x1000, param_4);
    return;
}

void __stdcall16far pass1_1010_116c(ulong *param_1, int param_2, ushort param_3)

{
    code     **ppcVar1;
    int        iVar2;
    undefined2 uVar3;
    int        iVar4;
    undefined2 uVar5;
    undefined4 uVar6;
    ushort     uStack4;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0x56) != 0x0)
    {
        ppcVar1 = (code **)((int)*param_1 + 0x34);
        (**ppcVar1)();
    }
    ppcVar1 = (code **)((int)*param_1 + 0x28);
    uVar6   = (**ppcVar1)();
    uVar3   = (undefined2)((ulong)uVar6 >> 0x10);
    if((int)uVar6 != 0x0)
    {
        uStack4 = DAT_1050_0ecc;
        iVar2   = DAT_1050_0ecc + 0x1;
        if(iVar2 == 0x0)
        {
            uStack4 = 0x0;
        }
        pass1_1010_1146((ulong)param_1, uStack4, param_2, param_3);
        pass1_1010_11c6((ulong)param_1, iVar2, uVar3);
        *(int *)(iVar4 + 0x56)        = iVar2;
        *(undefined2 *)(iVar4 + 0x58) = uVar3;
    }
    return;
}

void __stdcall16far pass1_1008_e852(ushort param_1, ushort param_2, ulong param_3, ushort param_4, uint param_5)

{
    undefined *puVar1;
    int        iVar2;
    char      *pcVar3;
    undefined  local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
    do
    {
        puVar1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar1));
        if((param_5 | (uint)puVar1) == 0x0)
        {
            return;
        }
        pcVar3  = pass1_1038_4d28(CONCAT22(param_5, puVar1));
        param_5 = (uint)((ulong)pcVar3 >> 0x10);
        iVar2   = pass1_1000_3d7a(param_3, (ulong)pcVar3 & 0xffff | (ulong)param_5 << 0x10);
    } while(iVar2 != 0x0);
    return;
}

long __stdcall16far pass1_1008_e8cc(ushort param_1, ulong param_2, ulong param_3, ulong param_4)

{
    undefined4 uVar1;
    uint       uVar2;
    uint       uVar3;
    int        iVar4;
    uint       uVar5;
    uint       uVar6;
    long       lVar7;
    char      *pcVar8;
    char      *pcVar9;
    ulong      uStack22;
    ulong      uStack18;
    undefined  local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_1, local_a), *(ulong *)((int)param_2 + 0xa));
    while(true)
    {
        lVar7 = pass1_1008_5b12(local_a, param_1);
        uVar5 = (uint)((ulong)lVar7 >> 0x10);
        uVar2 = (uint)lVar7;
        uVar6 = uVar5 | uVar2;
        if(lVar7 == 0x0)
        {
            return 0x0;
        }
        uVar1 = *(undefined4 *)(uVar2 + 0x4);
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        uStack18 = CONCAT22(uVar6, uVar3);
        uVar1    = *(undefined4 *)(uVar2 + 0x8);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        uStack22 = CONCAT22(uVar6, uVar3);
        pcVar8   = pass1_1038_4d28(uStack18);
        pcVar9   = pass1_1038_4d28(uStack22);
        iVar4    = pass1_1000_3d7a(param_4, (ulong)pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3, (ulong)pcVar9), iVar4 == 0x0))
            break;
        iVar4 = pass1_1000_3d7a(param_3, (ulong)pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4, (ulong)pcVar9), iVar4 == 0x0))
        {
            return lVar7;
        }
    }
    return lVar7;
}


ulong __stdcall16far pass1_1008_eb5c(ushort param_1, ushort param_2, int param_3)

{
    return CONCAT22(0x1050, param_3 * 0x10 + 0xd0e);
}


ushort __stdcall16far pass1_1008_eb6e(void)

{
    return 0x5;
}

void __stdcall16far pass1_1008_ec94(ushort *param_1)

{
    *param_1                            = 0xefc4;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    pass1_1010_3880(param_1);
    return;
}

void __stdcall16far pass1_1008_ed00(ushort *param_1, ushort param_2)

{
    *param_1                            = 0xef9c;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    pass1_1010_2db2(param_1, param_2);
    return;
}


void __stdcall16far pass1_1008_ed62(ulong param_1, int param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                         = (undefined2)(param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(int *)(iVar1 + 0x16)        = param_2 * 0x8 + 0xd5e;
    *(undefined2 *)(iVar1 + 0x18) = (int)&USHORT_1050_1050;
    *(undefined2 *)(iVar1 + 0x12) = *(undefined2 *)(param_2 * 0x8 + 0xd64);
    return;
}

void __stdcall16far pass1_1008_ee72(ushort param_1, ushort param_2, int param_3)

{
    code **ppcVar1;
    ulong  uVar2;

    if(*(long *)(param_1 + 0x56) == 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)CONCAT22(param_2, param_1) + 0x10);
        (**ppcVar1)();
    }
    uVar2 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, uVar2);
    return;
}


ushort __stdcall16far pass1_1008_eea6(void)

{
    return 0x0;
}

bool __stdcall16far pass1_1008_eeac(ushort param_1, ushort param_2, ulong param_3, uchar *param_4, int param_5, ushort param_6)

{
    ushort  uVar1;
    char    cVar2;
    uint    uVar3;
    ushort  uVar4;
    ushort  uVar5;
    ushort *puVar6;
    uint    uVar7;

    uVar7  = *(uint *)((int)param_3 + 0x12);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_4, param_5);
    uVar4  = (ushort)((ulong)puVar6 >> 0x10);
    uVar1  = (ushort)puVar6;
    uVar5  = uVar4;
    if(uVar7 == 0x7d)
    {
        pass1_1010_a5ca(uVar1, uVar4, 0x7c, 0x7d, uVar4);
        if(uVar7 != 0x0)
        {
            return false;
        }
        pass1_1010_a5ca(uVar1, uVar4, 0x7d, 0x0, uVar5);
        if(uVar7 != 0x0)
        {
            return false;
        }
        uVar3 = uVar7;
        uVar7 = 0x78;
    }
    else
    {
        uVar3 = uVar7;
        if(uVar7 < 0x7e)
        {
            cVar2 = (char)uVar7;
            uVar3 = uVar7 & 0xff00;
            if((byte)(cVar2 + 0x8dU) == 0x0)
            {
                uVar7 = 0x9;
                uVar3 = uVar3 | (byte)(cVar2 + 0x8dU);
            }
            else
            {
                if((byte)(cVar2 + 0x89U) == 0x0)
                {
                    uVar7 = 0x2e;
                    uVar3 = uVar3 | (byte)(cVar2 + 0x89U);
                }
                else
                {
                    uVar3 = uVar3 | (byte)(cVar2 + 0x87U);
                    if((byte)(cVar2 + 0x87U) == 0x0)
                    {
                        uVar7 = 0x5b;
                    }
                }
            }
        }
    }
    pass1_1010_a5ca(uVar1, uVar4, uVar7, uVar3, uVar5);
    return uVar3 == 0x0;
}


ushort __stdcall16far pass1_1008_ef38(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x16);
    return *(ushort *)((int)uVar1 + 0x2);
}


ushort __stdcall16far pass1_1008_ef4a(void)

{
    return 0x41;
}

ulong __stdcall16far pass1_1010_0000(astruct_645 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int         unaff_DI;
    astruct_79 *paVar1;
    ushort     *puVar2;
    undefined2 *puVar3;
    ushort      uVar4;
    undefined2 *puVar5;
    ushort      uVar6;

    // Segment:    3
    // Offset:     00015420
    // Length:     ee9f
    // Min Alloc:  ee9f
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    paVar1                                    = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x0;
    param_1->field_0xc                        = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x2c8;
    param_1->field_0x2                        = 0x1010;
    puVar5                                    = &param_1->field_0xa;
    puVar3                                    = &param_1->field_0xc;
    uVar4                                     = param_2;
    uVar6                                     = param_2;
    puVar2                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, (uchar *)((ulong)paVar1 >> 0x10), unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)), (ushort *)CONCAT22(uVar4, puVar3), (ushort *)CONCAT22(uVar6, puVar5));
    return CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1010_0052(ushort *param_1, ushort param_2)

{
    *param_1                            = 0x2c8;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    pass1_1010_1d80(param_1, param_2);
    return;
}

void __stdcall16far pass1_1010_01f8(ulong param_1, ulong param_2, int param_3)

{
    int        iVar1;
    int        iVar2;
    undefined2 uVar3;
    undefined2 uVar4;

    iVar2                        = *(int *)(param_3 * 0x4 + 0xe02) * 0x4;
    iVar1                        = *(int *)(iVar2 + 0xdfc);
    uVar3                        = (undefined2)(param_1 >> 0x10);
    uVar4                        = (undefined2)(param_2 >> 0x10);
    *(int *)((int)param_2 + 0x6) = *(int *)(param_3 * 0x4 + 0xe04) * 0x28 + *(int *)(iVar2 + 0xdfa) + *(int *)((int)param_1 + 0xa);
    *(int *)((int)param_2 + 0x8) = *(int *)((int)param_1 + 0xc) + iVar1;
    return;
}

void __stdcall16far pass1_1010_0350(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_474 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_474 *)param_1;
    *param_1         = 0xe98;
    iVar4->field_0x2 = 0x1010;
    puVar1           = iVar4->field_0xa;
    uVar2            = iVar4->field_0xc;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
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



BOOL16 __stdcall16far pass1_1008_c6ae(ulong param_1,int param_2,int param_3)

{
  int *piVar1;
  ulong *puVar2;
  int iStack8;
  
  puVar2 = pass1_1008_c6fa((int *)param_1,param_3);
  iStack8 = 0x0;
  while( true ) {
    piVar1 = (int *)((int)puVar2 + 0x4);
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
      return 0x0;
    }
    if (*(int *)((int)*puVar2 + iStack8 * 0x2) == param_2) break;
    iStack8 = iStack8 + 0x1;
  }
  return 0x1;
}



ulong * __stdcall16far pass1_1008_c6fa(int *param_1,int param_2)

{
  if ((0x0 < param_2) && (param_2 < 0x47)) {
    return (ulong *)CONCAT22(*(undefined2 *)((int)param_1 + 0x2),param_2 * 0x6 + *param_1);
  }
  return (ulong *)0x0;
}


void __stdcall16far pass1_1008_c75c(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_469 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_469 *)param_1;
  *param_1 = 0xca4a;
  iVar4->field_0x2 = 0x1008;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_c79a(ulong param_1,ulong param_2,int param_3,ushort param_4,uchar param_5)

{
  undefined *puVar1;
  int iVar2;
  ulong uVar3;
  uint extraout_DX;
  uchar *puVar4;
  ushort uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  undefined local_12 [0x4];
  undefined4 uStack14;
  undefined local_a [0x8];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0xa));
  while( true ) {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    uStack14 = CONCAT22(extraout_DX,puVar1);
    puVar4 = (uchar *)(extraout_DX | (uint)puVar1);
    if (puVar4 == (uchar *)0x0) break;
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
    if (iVar2 == 0x0) {
      puVar7 = pass1_1020_a43e(param_4,puVar4,(ushort *)CONCAT22(param_4,local_12));
      uVar5 = (ushort)((ulong)puVar7 >> 0x10);
      pass1_1020_a6ee(CONCAT22(param_4,local_12),*(ushort *)((int)uStack14 + 0x12),(uint)local_12,uVar5,param_3,param_4,
                      param_5);
      uVar3 = *(ulong *)((int)_PTR_LOOP_1050_65e2 + 0x52);
      pass1_1030_4bbe(param_4,uVar5,uVar3,*(int *)((int)uStack14 + 0x12));
      *(long *)((int)param_1 + 0xe) = (long)*(int *)((int)uVar3 + 0x94) + *_PTR_LOOP_1050_65e2;
    }
  }
  return;
}

void __stdcall16far pass1_1008_c83a(ulong param_1)

{
  if (*_PTR_LOOP_1050_65e2 <= *(ulong *)((int)param_1 + 0xe)) {
    return;
  }
  return;
}



ulong __stdcall16far pass1_1008_c85e(ulong param_1,ushort param_2)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(long *)(iVar1 + 0xa) == 0x0) {
    pass1_1008_c882(param_1 & 0xffff | (ulong)uVar2 << 0x10,param_2);
  }
  return CONCAT22(*(undefined2 *)(iVar1 + 0xc),*(undefined2 *)(iVar1 + 0xa));
}



void __stdcall16far pass1_1008_caa0(ushort *param_1,ushort param_2)

{
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  *param_1 = 0xd71a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  pass1_1008_cac6((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1008_cac6(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_470 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_470 *)param_1;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0xa = 0x0;
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0xe = 0x0;
  puVar1 = iVar4->field_0x12;
  uVar2 = iVar4->field_0x14;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x12 = 0x0;
  puVar1 = iVar4->field_0x16;
  uVar2 = iVar4->field_0x18;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x16 = 0x0;
  puVar1 = iVar4->field_0x1a;
  uVar2 = iVar4->field_0x1c;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x1a = 0x0;
  puVar1 = iVar4->field_0x1e;
  uVar2 = iVar4->field_0x20;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x1e = 0x0;
  return;
}


ulong __stdcall16far pass1_1008_b340(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x16) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x16);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0x6),*(undefined2 *)(iVar2 + 0x4));
  }
  return 0x0;
}



ulong __stdcall16far pass1_1008_b366(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0x6),*(undefined2 *)(iVar2 + 0x4));
  }
  return 0x0;
}


ulong __stdcall16far pass1_1008_b47a(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1e) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1e);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0x6),*(undefined2 *)(iVar2 + 0x4));
  }
  return 0x0;
}



void __stdcall16far pass1_1008_b4a0(ulong param_1,long param_2,uint param_3,uchar *param_4,ushort param_5)

{
  undefined4 uVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  long lVar7;
  
  iVar4 = (int)param_1;
  uVar5 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    *(undefined4 *)(iVar4 + 0x16) = 0x0;
  }
  else {
    pass1_1008_b9ce(param_1,param_2,param_5);
    *(uint *)(iVar4 + 0x16) = param_3;
    *(uchar **)(iVar4 + 0x18) = param_4;
  }
  uVar1 = *(undefined4 *)(iVar4 + 0x16);
  if (*(int *)((int)uVar1 + 0x8) != 0x0) {
    pass1_1008_b200(param_1,param_5);
    uVar6 = pass1_1008_b38c(param_1,param_3,param_4);
    uVar3 = (undefined2)(uVar6 >> 0x10);
    uVar2 = (undefined2)uVar6;
    uVar1 = *(undefined4 *)(iVar4 + 0x16);
    pass1_1008_b85c(param_1,*(long *)((int)uVar1 + 0xa));
    *(undefined2 *)(iVar4 + 0x1a) = uVar2;
    *(undefined2 *)(iVar4 + 0x1c) = uVar3;
    uVar1 = *(undefined4 *)(iVar4 + 0x16);
    lVar7 = pass1_1008_b8ac(param_1,*(int *)((int)uVar1 + 0xe),param_5);
    *(undefined2 *)(iVar4 + 0x1e) = (int)lVar7;
    *(undefined2 *)(iVar4 + 0x20) = (int)((ulong)lVar7 >> 0x10);
    return;
  }
  *(undefined4 *)(iVar4 + 0x1a) = 0x0;
  *(undefined4 *)(iVar4 + 0x1e) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_b544(ulong param_1,int param_2,ushort param_3,ushort param_4)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined2 uVar6;
  int iVar7;
  undefined2 uVar8;
  
  iVar7 = (int)param_1;
  uVar8 = (undefined2)(param_1 >> 0x10);
  if (param_2 != 0x0) {
    if (*(long *)(iVar7 + 0x1a) != 0x0) {
      uVar4 = *(undefined4 *)(iVar7 + 0x16);
      *(undefined2 *)((int)uVar4 + 0x8) = 0x1;
      uVar4 = *(undefined4 *)(iVar7 + 0x1a);
      uVar5 = *(undefined4 *)(iVar7 + 0x16);
      *(undefined4 *)((int)uVar5 + 0xa) = *(undefined4 *)((int)uVar4 + 0x8);
      uVar4 = *(undefined4 *)(iVar7 + 0x1e);
      uVar6 = *(undefined2 *)((int)uVar4 + 0x8);
      uVar4 = *(undefined4 *)(iVar7 + 0x16);
      *(undefined2 *)((int)uVar4 + 0xe) = uVar6;
      uVar4 = *(undefined4 *)(iVar7 + 0x16);
      pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                      *(ulong *)((int)uVar4 + 0xa));
      param_4 = (ushort)&PTR_LOOP_1050_1038;
      pass1_1038_3608(CONCAT22(param_3,uVar6));
    }
  }
  *(undefined4 *)(iVar7 + 0x1e) = 0x0;
  *(undefined4 *)(iVar7 + 0x1a) = 0x0;
  *(undefined4 *)(iVar7 + 0x16) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar7 + 0xe);
  uVar2 = *(uint *)(iVar7 + 0x10);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_4,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar7 + 0xe) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar7 + 0x12);
  uVar2 = *(uint *)(iVar7 + 0x14);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_4,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar7 + 0x12) = 0x0;
  return;
}



void __stdcall16far pass1_1008_b61a(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  undefined2 uVar1;
  
  pass1_1008_b8fa(param_1,param_2,param_4,param_5);
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x1a) = param_3;
  *(ushort *)((int)param_1 + 0x1c) = param_4;
  return;
}



void __stdcall16far pass1_1008_b63a(ulong param_1,ulong param_2)

{
  undefined2 in_AX;
  undefined2 in_DX;
  undefined2 uVar1;
  ushort unaff_SS;
  
  pass1_1008_b964(param_1,param_2,unaff_SS);
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x1e) = in_AX;
  *(undefined2 *)((int)param_1 + 0x20) = in_DX;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far load_string_1008_b65a(ULONG param_1,LPSTR in_string_2,ULONG param_3,UINT16 param_4)

{
  ushort unaff_SS;
  
  pass1_1008_b9ce(param_1,CONCAT22(param_4,param_3._2_2_),unaff_SS);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,in_string_2,
             (short)param_3);
  return;
}


ulong __stdcall16far pass1_1008_b820(ulong param_1,int param_2,ushort param_3)

{
  undefined2 uVar1;
  
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
  if (*(int *)(param_2 + 0x152) == 0x0) {
    return 0x0;
  }
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0xc),*(undefined2 *)((int)param_1 + 0xa));
}



void __stdcall16far pass1_1008_b85c(ulong param_1,long param_2)

{
  undefined *puVar1;
  uint extraout_DX;
  undefined2 unaff_SS;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(unaff_SS,local_a),*(ulong *)((int)param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,unaff_SS);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
  } while (*(long *)(puVar1 + 0x8) != param_2);
  return;
}



long __stdcall16far pass1_1008_b8ac(ulong param_1,int param_2,ushort param_3)

{
  long lVar1;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x12));
  do {
    lVar1 = pass1_1008_5b12(local_a,param_3);
    if (lVar1 == 0x0) {
      return 0x0;
    }
  } while (*(int *)((int)lVar1 + 0x8) != param_2);
  return lVar1;
}



void __stdcall16far pass1_1008_b8fa(ulong param_1,ulong param_2,ushort param_3,ushort param_4)

{
  undefined *puVar1;
  int iVar2;
  uint extraout_DX;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



void __stdcall16far pass1_1008_b964(ulong param_1,ulong param_2,ushort param_3)

{
  undefined *puVar1;
  int iVar2;
  uint extraout_DX;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x12));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



void __stdcall16far pass1_1008_b9ce(ulong param_1,ulong param_2,ushort param_3)

{
  undefined *puVar1;
  int iVar2;
  uint extraout_DX;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



ulong __stdcall16far
pass1_1008_a8f4(ulong param_1,ushort *param_2,ushort *param_3,ushort *param_4,ushort param_5,ushort param_6,
               ushort param_7,uchar param_8)

{
  int iVar1;
  undefined4 local_6;
  
  iVar1 = (int)&local_6 + 0x2;
  pass1_1008_a1f0(param_6,param_7,param_8,param_1,param_2,(ushort *)CONCAT22(param_7,&local_6),
                  (ushort *)CONCAT22(param_7,iVar1),param_4);
  pass1_1008_944e(param_3,(ushort)local_6,(ushort)((ulong)local_6 >> 0x10));
  return CONCAT22(param_5,iVar1);
}



ushort __stdcall16far pass1_1008_a9ec(ulong param_1)

{
  undefined4 uVar1;
  uint in_AX;
  int iVar2;
  uint uVar3;
  WNDCLASS16 *unaff_SS;
  uint uStack4;
  
  uStack4 = 0x0;
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(int *)(iVar2 + 0x414) == 0x0) && (uVar1 = *(undefined4 *)(iVar2 + 0x410), *(int *)((int)uVar1 + 0x8) != 0x0)) {
    *(undefined2 *)(iVar2 + 0x414) = 0x1;
    pass1_1008_aa28(param_1 & 0xffff | (ulong)uVar3 << 0x10,in_AX,unaff_SS);
    uStack4 = in_AX;
  }
  return uStack4;
}



ushort __stdcall16far pass1_1008_aaa8(ushort param_1,ushort param_2,ushort param_3)

{
  ushort uStack4;
  
  uStack4 = 0x0;
  switch(param_3) {
  case 0x1:
    uStack4 = 0x24;
    break;
  case 0x2:
    uStack4 = 0x16;
    break;
  case 0x3:
    uStack4 = 0x17;
    break;
  case 0x4:
    uStack4 = 0x18;
    break;
  case 0x5:
    uStack4 = 0x1b;
    break;
  case 0x6:
    uStack4 = 0x1c;
    break;
  case 0x7:
    uStack4 = 0x1f;
  }
  return uStack4;
}



ushort __stdcall16far pass1_1008_ab12(ushort param_1,ushort param_2,uint param_3)

{
  if (param_3 == 0x37) {
    return 0x22;
  }
  if (param_3 < 0x38) {
    if ((char)param_3 == '\r') {
      return 0xf;
    }
    if ((char)param_3 == '*') {
      return 0x2b;
    }
  }
  return 0x0;
}



ushort __stdcall16far pass1_1008_ab54(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  ushort uStack4;
  
  uStack4 = 0x0;
  uVar2 = (undefined2)(param_1 >> 0x10);
  if ((*(long *)((int)param_1 + 0xa) != 0x0) &&
     (uVar1 = *(undefined4 *)((int)param_1 + 0xa), *(int *)((int)uVar1 + 0x8) != 0x0)) {
    uStack4 = 0x1;
  }
  return uStack4;
}



ushort __stdcall16far pass1_1008_ab80(ushort param_1,ushort param_2,ushort param_3)

{
  ushort uStack4;
  
  uStack4 = 0x0;
  switch(param_3) {
  case 0x8:
    uStack4 = 0x82;
    break;
  case 0x9:
    uStack4 = 0x7f;
    break;
  case 0xa:
    uStack4 = 0x80;
    break;
  case 0xb:
    uStack4 = 0x84;
    break;
  case 0xc:
    uStack4 = 0x89;
    break;
  case 0xd:
    uStack4 = 0x8a;
    break;
  case 0xe:
    uStack4 = 0x8c;
    break;
  case 0xf:
    uStack4 = 0x8e;
    break;
  case 0x10:
    uStack4 = 0x8f;
    break;
  case 0x11:
    uStack4 = 0x90;
    break;
  case 0x12:
    uStack4 = 0x91;
    break;
  case 0x13:
    uStack4 = 0x95;
    break;
  case 0x14:
    uStack4 = 0x96;
    break;
  case 0x16:
    uStack4 = 0x9b;
    break;
  case 0x17:
    uStack4 = 0x9f;
    break;
  case 0x18:
    uStack4 = 0xa2;
    break;
  case 0x19:
    uStack4 = 0xa4;
    break;
  case 0x1b:
  case 0x1c:
    uStack4 = 0xa7;
    break;
  case 0x1d:
    uStack4 = 0xaa;
    break;
  case 0x1e:
    uStack4 = 0xac;
    break;
  case 0x1f:
    uStack4 = 0xad;
    break;
  case 0x20:
    uStack4 = 0xae;
    break;
  case 0x21:
    uStack4 = 0xb1;
    break;
  case 0x22:
    uStack4 = 0xb3;
    break;
  case 0x23:
    uStack4 = 0xb4;
    break;
  case 0x24:
    uStack4 = 0xb5;
    break;
  case 0x25:
    uStack4 = 0xb6;
    break;
  case 0x26:
    uStack4 = 0xb7;
    break;
  case 0x27:
    uStack4 = 0xab;
    break;
  case 0x28:
    uStack4 = 0xb9;
    break;
  case 0x29:
    uStack4 = 0xba;
    break;
  case 0x2a:
    uStack4 = 0xbc;
    break;
  case 0x2b:
    uStack4 = 0xbe;
    break;
  case 0x2c:
    uStack4 = 0xdf;
    break;
  case 0x2d:
    uStack4 = 0xe0;
  }
  return uStack4;
}



ushort * __stdcall16far pass1_1008_ad0c(ushort *param_1,byte param_2)

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

ushort * __stdcall16far pass1_1008_ada2(ushort *param_1,int param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x0;
  *(int *)((int)param_1 + 0x4) = param_2;
  *param_1 = *(ushort *)(param_2 * 0x6 + 0x3a4);
  return param_1;
}



void __stdcall16far pass1_1008_add2(ushort *param_1)

{
  *param_1 = *(ushort *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a4);
  return;
}



ushort __stdcall16far pass1_1008_adf2(ulong param_1)

{
  return *(ushort *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a4);
}



ushort __stdcall16far pass1_1008_ae0c(ulong param_1)

{
  return *(ushort *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a6);
}



void __stdcall16far pass1_1008_ae26(int *param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = *(int *)(*(int *)(iVar3 + 0x4) * 0x6 + 0x3a8);
  if (iVar2 == 0x2) {
    if (*(int *)(iVar3 + 0x2) == 0x1) {
      *param_1 = *param_1 + -0x1;
      iVar2 = *(int *)(iVar3 + 0x4) * 0x6;
      piVar1 = (int *)(iVar2 + 0x3a4);
      if (*piVar1 != *param_1 && *param_1 <= *piVar1) {
        *param_1 = *(int *)(iVar2 + 0x3a4) + 0x1;
        *(undefined2 *)(iVar3 + 0x2) = 0x0;
        return;
      }
    }
    else {
      *param_1 = *param_1 + 0x1;
      iVar2 = *(int *)(iVar3 + 0x4) * 0x6;
      if (*(int *)(iVar2 + 0x3a6) < *param_1) {
        *param_1 = *(int *)(iVar2 + 0x3a6) + -0x1;
        *(undefined2 *)(iVar3 + 0x2) = 0x1;
        return;
      }
    }
  }
  else {
    if ((iVar2 != 0x3) && (iVar2 != 0x4)) {
      *param_1 = *param_1 + 0x1;
      iVar2 = *(int *)(iVar3 + 0x4) * 0x6;
      if (*(int *)(iVar2 + 0x3a6) < *param_1) {
        *param_1 = *(int *)(iVar2 + 0x3a4);
      }
    }
  }
  return;
}



BOOL16 __stdcall16far pass1_1008_aed8(ulong param_1)

{
  if (*(int *)(*(int *)((int)param_1 + 0x4) * 0x6 + 0x3a4) != 0x0) {
    return 0x1;
  }
  return 0x0;
}

void __stdcall16far pass1_1008_afde(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_468 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_468 *)param_1;
  *param_1 = 0xbdcc;
  iVar4->field_0x2 = 0x1008;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x12;
  uVar2 = iVar4->field_0x14;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



ushort * __stdcall16far pass1_1008_b05a(ushort *param_1)

{
  astruct_193 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_193 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  *param_1 = 0xbdc8;
  iVar1->field_0x2 = 0x1008;
  return param_1;
}


void __stdcall16far set_stuct_1008_b0bc(astruct_26 *param_1)

{
  astruct_26 *iVar1;
  undefined2 uVar1;
  
  pass1_1008_b05a((ushort *)param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_26 *)param_1;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x10 = 0x0;
  *(undefined2 *)param_1 = 0xbdc4;
  iVar1->field_0x2 = 0x1008;
  return;
}



ushort * __stdcall16far pass1_1008_b0f2(ushort *param_1)

{
  undefined2 uVar1;
  
  pass1_1008_b05a(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x8) = 0x0;
  *param_1 = 0xbdc0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return param_1;
}



ushort * __stdcall16far pass1_1008_b11e(ushort *param_1)

{
  undefined2 uVar1;
  
  pass1_1008_b05a(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x8) = 0x0;
  *param_1 = 0xbddc;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return param_1;
}

void __stdcall16far pass1_1008_b146(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x16) != 0x0) {
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                    *(ulong *)((int)uVar1 + 0xa));
    pass1_1038_3608(CONCAT22(param_3,param_2));
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined2 *)((int)uVar1 + 0x8) = 0x0;
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined4 *)((int)uVar1 + 0xa) = 0x0;
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined2 *)((int)uVar1 + 0xe) = 0x0;
    uVar1 = *(undefined4 *)(iVar2 + 0x16);
    *(undefined2 *)((int)uVar1 + 0x10) = 0x0;
  }
  return;
}


void __stdcall16far pass1_1008_9628(undefined4 param_1,undefined2 param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x8) == 0x0) {
    *(undefined2 *)((int)param_1 + 0x8) = param_2;
  }
  return;
}



LRESULT __stdcall16far pass1_1008_9c16(UINT16 param_1,ulong param_2,ulong param_3,HWND16 param_4)

{
  LRESULT LVar1;
  
  LVar1 = make_def_wnd_proc_1008_9ce6
                    (param_1,(UINT16)param_2,(UINT16)(param_2 >> 0x10),(WPARAM16)param_3,
                     CONCAT22(0x85,(int)(param_3 >> 0x10)),param_4);
  return LVar1;
}



LRESULT __stdcall16far pass1_1008_9c30(UINT16 param_1,ulong param_2,ulong param_3,HWND16 param_4)

{
  LRESULT LVar1;
  
  LVar1 = make_def_wnd_proc_1008_9ce6
                    (param_1,(UINT16)param_2,(UINT16)(param_2 >> 0x10),(WPARAM16)param_3,
                     CONCAT22(0x86,(int)(param_3 >> 0x10)),param_4);
  return LVar1;
}



void __stdcall16far pass1_1008_9c4a(void)

{
  return;
}



void __stdcall16far pass1_1008_9c4e(void)

{
  return;
}



void __stdcall16far pass1_1008_9c52(void)

{
  return;
}




void __stdcall16far pass1_1008_9c60(ushort param_1,ushort param_2,ulong *param_3,int param_4)

{
  code **ppcVar1;
  
  if ((param_4 == 0xc7) && (param_3 != (ulong *)0x0)) {
    ppcVar1 = (code **)*param_3;
    (**ppcVar1)();
  }
  return;
}



BOOL16 __stdcall16far pass1_1008_9cc4(ulong param_1,int param_2)

{
  if (*(int *)((int)param_1 + 0x8) != param_2) {
    return 0x1;
  }
  return 0x0;
}



ushort __stdcall16far pass1_1008_9ce0(void)

{
  return 0x0;
}


void __stdcall16far pass1_1008_9f18(int param_1,ushort param_2,int param_3,ushort param_4)

{
  if (param_3 == 0x2) {
    pass1_1008_9f64(CONCAT22(param_2,param_1 + -0x1c));
    pass1_1010_1f62(param_4,CONCAT22(param_2,param_1 + -0x1c),0x2);
  }
  return;
}



ulong __stdcall16far pass1_1008_9f48(ulong param_1)

{
  astruct_134 *iVar1;
  int iVar2;
  ushort uVar3;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  iVar1 = (astruct_134 *)param_1;
  iVar2 = iVar1->field_0x20 * 0x4;
  return CONCAT22(*(undefined2 *)(&iVar1[0x1].field_0x2 + iVar2),*(undefined2 *)(&iVar1[0x1].field_0x0 + iVar2));
}



void __stdcall16far pass1_1008_9f64(ulong param_1)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  piVar1 = (int *)(iVar2 + 0x20);
  *piVar1 = *piVar1 + 0x1;
  if (0xb < *(int *)(iVar2 + 0x20)) {
    *(undefined2 *)(iVar2 + 0x20) = 0x0;
  }
  return;
}

void __stdcall16far pass1_1008_a086(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_465 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_465 *)param_1;
  *param_1 = 0xad92;
  iVar4->field_0x2 = 0x1008;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x410;
  uVar2 = iVar4->field_0x412;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}

void __stdcall16far
pass1_1008_87cc(astruct_86 *param_1,int param_2,int param_3,ushort param_4,ulong param_5,ulong param_6,ushort param_7)

{
  long lVar1;
  uint uVar2;
  BOOL16 BVar3;
  int *piVar4;
  uchar *puVar5;
  astruct_86 *iVar5;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  undefined2 uVar8;
  ushort *puVar9;
  int *piStack48;
  undefined4 local_24;
  ushort uStack32;
  ulong uStack30;
  astruct_18 *paStack26;
  undefined4 uStack18;
  int iStack14;
  int iStack12;
  int iStack10;
  int iStack8;
  ulong uStack6;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_86 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  iVar5->field_0x4 = (astruct_76 *)param_5;
  *(undefined4 *)&iVar5->field_0x8 = 0x0;
  iVar5->field_0xc = param_3;
  iVar5->field_0xe = param_2;
  iVar5->field_0x10 = 0x0;
  iVar5->field_0x12 = 0x0;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c)));
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x22));
  puVar9 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x28));
  iVar5->field_0x2e = param_4;
  iVar5->field_0x30 = 0xffff;
  iVar5->field_0x3a = 0x0;
  iVar5->field_0x3e = 0x1;
  iVar5->field_0x40 = 0x1;
  iVar5->field_0x42 = param_6;
  param_1->field_0x0 = 0x8e9a;
  iVar5->field_0x2 = 0x1008;
  if (_PTR_LOOP_1050_0382 == (ushort *)0x0) {
    _PTR_LOOP_1050_0382 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2e,param_7,(uchar *)((ulong)puVar9 >> 0x10),unaff_DI);
  }
  uStack6 = pass1_1008_4772(iVar5->field_0x4);
  iVar5->field_0x12 = 0x2f - *(int *)((int)uStack6 + 0x8);
  uVar8 = (undefined2)((ulong)_PTR_LOOP_1050_0382 >> 0x10);
  iVar6 = (int)_PTR_LOOP_1050_0382;
  iStack8 = *(int *)(iVar6 + 0xa);
  iStack10 = *(int *)(iVar6 + 0xc);
  iStack12 = *(int *)(iVar6 + 0xe);
  iStack14 = *(int *)(iVar6 + 0x10);
  iVar6 = iVar5->field_0xc;
  lVar1 = (long)(iVar6 + iVar5->field_0xe) * (long)iStack14;
  puVar5 = (uchar *)((ulong)lVar1 >> 0x10);
  pass1_1008_3e76((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c)),0x0,
                  (int)lVar1 + iVar5->field_0x12 + iStack10,
                  (iVar6 - iVar5->field_0xe) * iStack12 + iVar5->field_0x10 + iStack8);
  iVar5->field_0x14 = iVar5->field_0x1c + 0x20;
  iVar5->field_0x16 = *(int *)((int)uStack6 + 0x8) + iVar5->field_0x1e + -0x25;
  iVar5->field_0x18 = iVar5->field_0x14 + 0x32;
  uVar2 = iVar5->field_0x16 + 0x19;
  iVar5->field_0x1a = uVar2;
  mem_op_1000_179c(0x6,puVar5,0x1000);
  paStack26 = (astruct_18 *)CONCAT22(puVar5,uVar2);
  uStack18._2_2_ = (uint)puVar5 | uVar2;
  if (uStack18._2_2_ == 0x0) {
    *(undefined4 *)&iVar5->field_0x8 = 0x0;
  }
  else {
    puVar9 = pass1_1008_ada2((ushort *)CONCAT22(puVar5,uVar2),iVar5->field_0x2e);
    uStack18._2_2_ = (uint)((ulong)puVar9 >> 0x10);
    iVar5->field_0x8 = (uint)puVar9;
    iVar5->field_0xa = uStack18._2_2_;
  }
  BVar3 = pass1_1008_aed8(*(ulong *)&iVar5->field_0x8);
  if (BVar3 == 0x0) {
    paStack26 = *(astruct_18 **)&iVar5->field_0x8;
    uStack18 = paStack26;
    fn_ptr_1000_17ce(paStack26,0x1000);
    *(undefined4 *)&iVar5->field_0x8 = 0x0;
  }
  else {
    piVar4 = *(int **)&iVar5->field_0x8;
    pass1_1018_20ee((ulong)_PTR_LOOP_1050_0382,piVar4);
    uStack18._0_2_ = SUB42(piVar4,0x0);
    pass1_1008_add2(*(ushort **)&iVar5->field_0x8);
    uStack30 = pass1_1008_4772((astruct_76 *)CONCAT22(uStack18._2_2_,(undefined2)uStack18));
    pass1_1018_214e((ushort)_PTR_LOOP_1050_0382,(ushort)((ulong)_PTR_LOOP_1050_0382 >> 0x10),
                    (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x28),iVar5->field_0x2e);
    local_24 = *(undefined4 *)&iVar5->field_0x1c;
    uStack32 = iVar5->field_0x20;
    pass1_1008_3f32((int *)CONCAT22(param_7,&local_24),
                    (int *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x28));
    piStack48 = (int *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x32);
    pass1_1008_3e94((ushort *)CONCAT22(param_7,&local_24),
                    (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x34)),
                    (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x32));
    uVar8 = (undefined2)(uStack30 >> 0x10);
    iVar5->field_0x36 = *(int *)((int)uStack30 + 0x4) + *piStack48;
    uVar2 = *(int *)((int)uStack30 + 0x8) + iVar5->field_0x34;
    iVar5->field_0x38 = uVar2;
    pass1_1008_612e(0x2,0x5,uVar2);
    iVar5->field_0x3e = uVar2;
  }
  return;
}
void __stdcall16far pass1_1008_8b20(ulong param_1,ushort param_2)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  undefined local_a [0x2];
  undefined local_8 [0x2];
  astruct_76 *paStack6;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x8) != 0x0) {
    iVar1 = *(int *)(iVar4 + 0x40);
    piVar2 = (int *)(iVar4 + 0x40);
    *piVar2 = *piVar2 + 0x1;
    uVar3 = iVar1 % *(int *)(iVar4 + 0x3e);
    if (uVar3 == 0x0) {
      *(undefined2 *)(iVar4 + 0x40) = 0x1;
      piVar2 = *(int **)(iVar4 + 0x8);
      pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar2);
      paStack6 = (astruct_76 *)((ulong)piVar2 & 0xffff | (ulong)uVar3 << 0x10);
      pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar4 + 0x28U)),(ushort *)CONCAT22(param_2,local_a),
                      (ushort *)CONCAT22(param_2,local_8));
      pass1_1008_8d8a(param_1 & 0xffff | (ulong)uVar5 << 0x10,paStack6,*(ulong *)(iVar4 + 0x4));
      pass1_1008_4480(*(ulong *)(iVar4 + 0x4),(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar4 + 0x28U)),paStack6,param_2
                     );
      return;
    }
  }
  return;
}


void __stdcall16far pass1_1008_8bc6(ushort param_1,uint param_2,ulong param_3)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  undefined local_a [0x2];
  undefined local_8 [0x2];
  astruct_76 *paStack6;
  
  uVar3 = (uint)(param_3 >> 0x10);
  iVar2 = (int)param_3;
  if (*(long *)(iVar2 + 0x8) == 0x0) {
    return;
  }
  piVar1 = *(int **)(iVar2 + 0x8);
  pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar1);
  paStack6 = (astruct_76 *)((ulong)piVar1 & 0xffff | (ulong)param_2 << 0x10);
  pass1_1008_3e94((ushort *)(param_3 & 0xffff0000 | (ulong)(iVar2 + 0x28U)),(ushort *)CONCAT22(param_1,local_a),
                  (ushort *)CONCAT22(param_1,local_8));
  pass1_1008_8d8a(param_3 & 0xffff | (ulong)uVar3 << 0x10,paStack6,*(ulong *)(iVar2 + 0x4));
  pass1_1008_4480(*(ulong *)(iVar2 + 0x4),(ushort *)(param_3 & 0xffff0000 | (ulong)(iVar2 + 0x28U)),paStack6,param_1);
  return;
}


void __stdcall16far pass1_1008_8c4e(ulong param_1,ulong param_2,ushort param_3)

{
  undefined2 uVar1;
  uint *puVar2;
  uchar *puVar3;
  uchar *puVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  ulong uVar8;
  astruct_110 *paStack14;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar8 = pass1_1008_4772(*(astruct_76 **)(iVar6 + 0x4));
  puVar3 = (uchar *)(uVar8 >> 0x10);
  uVar5 = 0x0;
  if ((*(int *)(iVar6 + 0xc) == 0x0) || (*(int *)(iVar6 + 0xe) == 0x0)) {
    puVar4 = puVar3;
    mem_op_1000_179c(0x14,puVar3,0x1000);
    paStack14 = (astruct_110 *)CONCAT22(puVar4,uVar5);
    uVar5 = (uint)puVar4 | uVar5;
    if (uVar5 == 0x0) {
      uVar1 = 0x0;
      uVar5 = 0x0;
    }
    else {
      puVar2 = (uint *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x1c));
      pass1_1008_50c2(paStack14,*(ulong *)((int)uVar8 + 0x8),*(ulong *)((int)uVar8 + 0x4),puVar2,param_2);
      uVar1 = SUB42(puVar2,0x0);
    }
    pass1_1008_5134(CONCAT22(uVar5,uVar1));
  }
  pass1_1008_4480(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x1c)),*(astruct_76 **)(iVar6 + 0x4),param_3
                 );
  return;
}

void __stdcall16far pass1_1008_8ce4(ulong param_1,ushort *param_2,ulong param_3,ushort param_4)

{
  undefined *puVar1;
  uchar *puVar2;
  uint uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  undefined local_10 [0x6];
  undefined4 uStack10;
  ulong uStack6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uStack6 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x4));
  uStack10 = 0x0;
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_4,local_10),0x0,*(ushort *)(iVar4 + 0x12),*(ushort *)(iVar4 + 0x10))
  ;
  puVar2 = (uchar *)((ulong)puVar7 >> 0x10);
  puVar1 = local_10;
  pass1_1008_3f32((int *)param_2,(int *)CONCAT22(param_4,puVar1));
  mem_op_1000_179c(0x14,puVar2,0x1000);
  uVar3 = (uint)puVar2 | (uint)puVar1;
  if (uVar3 == 0x0) {
    puVar1 = (undefined *)0x0;
    uVar3 = 0x0;
  }
  else {
    uVar6 = (undefined2)(uStack6 >> 0x10);
    pass1_1008_50c2((astruct_110 *)CONCAT22(puVar2,puVar1),*(ulong *)((int)uStack6 + 0x8),*(ulong *)((int)uStack6 + 0x4)
                    ,param_2,param_3);
  }
  uStack10 = CONCAT22(uVar3,puVar1);
  pass1_1008_5134(CONCAT22(uVar3,puVar1));
  pass1_1008_4480(param_3,param_2,*(astruct_76 **)(iVar4 + 0x4),param_4);
  return;
}


void __stdcall16far pass1_1008_8faa(ulong param_1,ulong param_2)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  pass1_1008_9004(param_1 & 0xffff | (ulong)uVar1 << 0x10,(ushort)param_2,(ushort)(param_2 >> 0x10),
                  *(ulong *)((int)param_1 + 0xa));
  return;
}



void __stdcall16far empty_1008_8fc4(void)

{
  return;
}



void __stdcall16far pass1_1008_9004(ulong param_1,ushort param_2,ushort param_3,ulong param_4)

{
  ulong *puVar1;
  uint *puVar2;
  long lVar3;
  astruct_107 *iVar4;
  astruct_108 *iVar5;
  uint uVar4;
  undefined2 uVar5;
  ushort unaff_SS;
  bool bVar6;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar4 = (astruct_107 *)param_1;
  puVar1 = (ulong *)&iVar4->field_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
    puVar2 = (uint *)((int)&iVar4->field_0x12 + 0x2);
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar1 = &iVar4->field_0x12, *(uint *)puVar1 < (uint)param_4 || *(uint *)puVar1 == (uint)param_4))))
    {
      pass1_1008_909c(param_1 & 0xffff | (ulong)uVar4 << 0x10,unaff_SS);
    }
    puVar1 = &iVar4->field_0x12;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
      return;
    }
    puVar2 = &iVar4->field_0xc;
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar2 = &iVar4->field_0xa, *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4)))) {
      iVar4->field_0xa = (uint)(param_4 + 0x1);
      iVar4->field_0xc = (uint)(param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = iVar4->field_0x6;
  uVar5 = (undefined2)((ulong)lVar3 >> 0x10);
  iVar5 = (astruct_108 *)lVar3;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4) = param_2;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4 + 0x2) = param_3;
  return;
}


void __stdcall16far pass1_1008_92b2(ulong param_1,long param_2,long param_3)

{
  code **ppcVar1;
  undefined *puVar2;
  uint extraout_DX;
  undefined2 unaff_SS;
  undefined local_c [0x4];
  undefined4 uStack8;
  undefined2 uStack4;
  
  uStack4 = 0x0;
  pass1_1008_57a4((ulong *)CONCAT22(unaff_SS,local_c),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x6));
  while( true ) {
    puVar2 = local_c;
    pass1_1008_5b12(puVar2,unaff_SS);
    if ((extraout_DX | (uint)puVar2) == 0x0) break;
    if ((*(long *)(puVar2 + 0x4) == param_3) && (*(long *)(puVar2 + 0x8) == param_2)) {
      uStack4 = 0x1;
      ppcVar1 = (code **)((int)*(undefined4 *)((int)param_1 + 0x6) + 0xc);
      (**ppcVar1)();
      uStack8 = 0x0;
    }
  }
  return;
}



void __stdcall16far pass1_1008_932a(ulong param_1,ushort param_2)

{
  uint uVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined local_a [0x8];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(int *)(iVar5 + 0x4) == 0x0) {
    *(undefined2 *)(iVar5 + 0x4) = 0x1;
    pass1_1008_57a4((ulong *)CONCAT22(param_2,local_a),param_1 & 0xffff0000 | (ulong)(iVar5 + 0x6));
    while( true ) {
      puVar3 = local_a;
      pass1_1008_5b12(puVar3,param_2);
      if ((extraout_DX | (uint)puVar3) == 0x0) break;
      uVar1 = *(uint *)(puVar3 + 0xc);
      iVar4 = *(int *)(puVar3 + 0xe) - (uint)(uVar1 < 0x37);
      *(uint *)(puVar3 + 0xc) = uVar1 - 0x37;
      *(int *)(puVar3 + 0xe) = iVar4;
      if ((iVar4 < 0x1) && (((iVar4 < 0x0 || (*(int *)(puVar3 + 0xc) == 0x0)) && (*(int *)(puVar3 + 0x10) == 0x0)))) {
        ppcVar2 = (code **)((int)**(undefined4 **)(puVar3 + 0x4) + 0x4);
        (**ppcVar2)();
        *(undefined4 *)(puVar3 + 0xc) = *(undefined4 *)(puVar3 + 0x8);
      }
    }
    *(undefined2 *)(iVar5 + 0x4) = 0x0;
  }
  return;
}
