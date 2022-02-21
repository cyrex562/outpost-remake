
void __stdcall16far pass1_1010_927a(ushort *param_1)

{
    *param_1                            = 0x958e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    pass1_1010_3880(param_1);
    return;
}

void __stdcall16far pass1_1010_92e6(ushort *param_1, ushort param_2)

{
    *param_1                            = 0x9566;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    pass1_1010_2db2(param_1, param_2);
    return;
}

void __stdcall16far pass1_1010_9348(ulong param_1, int param_2)

{
    int        iVar1;
    undefined2 uVar2;

    *(int *)(param_2 * 0x8 + 0x319e) = param_2;
    uVar2                            = (undefined2)(param_1 >> 0x10);
    iVar1                            = (int)param_1;
    *(int *)(iVar1 + 0x16)           = param_2 * 0x8 + 0x3198;
    *(undefined2 *)(iVar1 + 0x18)    = (int)&USHORT_1050_1050;
    *(int *)(iVar1 + 0x12)           = param_2;
    return;
}

void __stdcall16far pass1_1010_93f0(ulong param_1, ushort param_2)

{
    undefined *puVar1;
    undefined2 uVar2;
    int        iVar3;
    undefined2 uVar4;
    ushort    *puVar5;
    undefined  local_1c[0x1a];

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0x56) == 0x0)
    {
        puVar5 = pass1_1010_9258((ushort *)CONCAT22(param_2, local_1c));
        uVar2  = (undefined2)((ulong)puVar5 >> 0x10);
        puVar1 = local_1c;
        pass1_1010_398e((ulong *)CONCAT22(param_2, puVar1), 0x0, 0x0, 0x0, (ushort)puVar1);
        *(ushort *)(iVar3 + 0x56)     = (ushort)puVar1;
        *(undefined2 *)(iVar3 + 0x58) = uVar2;
        pass1_1010_927a((ushort *)CONCAT22(param_2, local_1c));
    }
    return;
}

void __stdcall16far pass1_1010_944e(ushort param_1, ushort param_2, int param_3)

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

bool __stdcall16far pass1_1010_9488(ushort param_1, ushort param_2, ulong param_3, uchar *param_4, int param_5, ushort param_6)

{
    ushort  uVar1;
    ushort  uVar2;
    ushort  uVar3;
    ushort *puVar4;
    ushort  uVar5;
    ushort  uStack10;

    uVar5    = *(ushort *)((int)param_3 + 0x12);
    puVar4   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_4, param_5);
    uVar2    = (ushort)((ulong)puVar4 >> 0x10);
    uVar1    = uVar5 - 0x32;
    uStack10 = (ushort)puVar4;
    uVar3    = uVar2;
    if(uVar1 == 0x0)
    {
        pass1_1010_a5ca(uStack10, uVar2, 0x32, 0x0, uVar2);
        if(uVar1 != 0x0)
        {
            return false;
        }
        uVar5 = 0x4d;
    }
    else
    {
        uVar1 = uVar5 - 0x3f;
        if(uVar1 == 0x0)
        {
            pass1_1010_a5ca(uStack10, uVar2, 0x3f, 0x0, uVar2);
            if(uVar1 != 0x0)
            {
                return false;
            }
            uVar5 = 0x4e;
        }
    }
    pass1_1010_a5ca(uStack10, uVar2, uVar5, uVar1, uVar3);
    return uVar1 == 0x0;
}

ushort __stdcall16far pass1_1010_9502(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x16);
    return *(ushort *)((int)uVar1 + 0x2);
}


ushort __stdcall16far pass1_1010_9514(void)

{
    return 0x31;
}

void __stdcall16far pass1_1010_95f8(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_491 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_491 *)param_1;
    *param_1         = 0xa1c8;
    iVar4->field_0x2 = 0x1010;
    puVar1           = iVar4->field_0xa;
    uVar2            = iVar4->field_0xc;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar4->field_0xe;
    uVar2  = iVar4->field_0x10;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar4->field_0x12;
    uVar2  = iVar4->field_0x14;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


void __stdcall16far pass1_1010_9674(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x12);
    uVar2  = *(uint *)(iVar4 + 0x14);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *(undefined4 *)(iVar4 + 0x12) = 0x0;
    return;
}


void __stdcall16far pass1_1010_96a8(ulong param_1, int param_2)

{
    int       *piVar1;
    undefined2 uVar2;

    uVar2   = (undefined2)(param_1 >> 0x10);
    piVar1  = (int *)((int)param_1 + 0x1e);
    *piVar1 = *piVar1 - param_2;
    if(*piVar1 < 0x0)
    {
        *(undefined2 *)((int)param_1 + 0x1e) = 0x0;
    }
    return;
}


ushort __stdcall16far pass1_1010_96c2(ulong param_1)

{
    return *(ushort *)((int)param_1 + 0x1e);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1010_96d0(ulong param_1)

{
    int         *piVar1;
    int          iVar2;
    astruct_690 *iVar3;
    undefined2   uVar3;
    ulong        uVar4;
    int          iStack8;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_690 *)param_1;
    if(iVar3->field_0x1a != 0x0)
    {
        if(0x0 < iVar3->field_0x1c)
        {
            piVar1  = &iVar3->field_0x1c;
            *piVar1 = *piVar1 + -0x1;
        }
        if((iVar3->field_0x1c == 0x0) && (iVar3->field_0x1e != 0x0))
        {
            iStack8 = 0x1;
            uVar4   = pass1_1030_8326();
            iVar2   = (int)(uVar4 >> 0x10);
            if((iVar2 != 0x0) || (0x32 < (uint)uVar4))
            {
                iStack8 = 0x5;
            }
            if((iVar2 != 0x0) || (0x3c < (uint)uVar4))
            {
                iStack8 = 0xa;
            }
            if(iVar3->field_0x1e < iStack8)
            {
                iStack8 = iVar3->field_0x1e;
            }
            piVar1  = &iVar3->field_0x1e;
            *piVar1 = *piVar1 - iStack8;
            if(*piVar1 < 0x0)
            {
                iVar3->field_0x1e = 0x0;
            }
            if(0x0 < iVar3->field_0x1e)
            {
                return iStack8;
            }
            return -0x1;
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1010_9766(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int        in_AX;
    undefined2 uVar1;

    uVar1                                = (undefined2)(param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x1a) = 0x1;
    pass1_1010_a0a0(param_1, param_2, param_3, param_4);
    pass1_1010_9f8c(param_1, 0x80, param_4);
    *(int *)((int)param_1 + 0x1e) = in_AX * 0x32;
    return;
}

ushort __stdcall16far pass1_1010_7818(ulong param_1, ulong param_2)

{
    undefined4 uVar1;
    ushort     uVar2;
    BOOL16     BVar3;
    ushort     uVar4;
    ushort     uStack6;

    uVar4 = (ushort)(param_1 >> 0x10);
    uVar1 = *(undefined4 *)((ushort)param_1 + 0x14);
    uVar2 = pass1_1010_b028((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x1e);
    if(BVar3 == 0x0)
    {
        BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0xb);
        if(((BVar3 == 0x0) && (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x20), BVar3 == 0x0)) && (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x1c), BVar3 == 0x0))
        {
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x2);
            if((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x13), BVar3 != 0x0))
            {
                return 0x5;
            }
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x11);
            if((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x12), BVar3 != 0x0))
            {
                return 0x4;
            }
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x5);
            if(BVar3 != 0x0)
            {
                return 0x6;
            }
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x6);
            if(BVar3 != 0x0)
            {
                return 0x7;
            }
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x4);
            if(BVar3 != 0x0)
            {
                return 0x10;
            }
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x3);
            if(BVar3 != 0x0)
            {
                return 0x11;
            }
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x19);
            if(BVar3 != 0x0)
            {
                return 0x15;
            }
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x1d);
            if(BVar3 != 0x0)
            {
                return 0x16;
            }
            uVar2 = pass1_1010_7d7e((ushort)param_1, uVar4, 0x1, uVar2);
            if(uVar2 == 0x0)
            {
                return 0x0;
            }
            return 0xc;
        }
        uStack6 = 0x1;
    }
    else
    {
        uStack6 = 0x18;
    }
    return uStack6;
}

void __stdcall16far pass1_1010_7b8c(ulong param_1, int param_2, ushort param_3)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    undefined4  uVar4;
    undefined  *puVar5;
    uint        extraout_DX;
    int         iVar6;
    undefined2  uVar7;
    undefined4  uStack14;
    undefined   local_a[0x8];

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if((*(uint *)(iVar6 + 0x1e) | *(uint *)(iVar6 + 0x1c)) != 0x0)
    {
        pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)(iVar6 + 0x1c));
        do
        {
            puVar5 = local_a;
            pass1_1008_5b12(puVar5, param_3);
            uStack14 = CONCAT22(extraout_DX, puVar5);
            if((extraout_DX | (uint)puVar5) == 0x0)
                break;
            uVar4 = *(undefined4 *)(puVar5 + 0x8);
        } while(*(int *)((int)uVar4 + 0x6) != param_2);
        if((extraout_DX | (uint)puVar5) != 0x0)
        {
            ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x1c) + 0xc);
            (**ppcVar3)(0x1008, *(undefined4 *)(iVar6 + 0x1c), uStack14);
        }
        uVar4 = *(undefined4 *)(iVar6 + 0x1c);
        if(*(int *)((int)uVar4 + 0x8) == 0x0)
        {
            puVar1 = (undefined4 *)*(uint *)(iVar6 + 0x1c);
            uVar2  = *(uint *)(iVar6 + 0x1e);
            if((uVar2 | (uint)puVar1) != 0x0)
            {
                ppcVar3 = (code **)*puVar1;
                (**ppcVar3)(0x1008, puVar1, uVar2, 0x1, puVar1, uVar2, puVar1, uVar2);
            }
            *(undefined4 *)(iVar6 + 0x1c) = 0x0;
        }
    }
    return;
}

undefined2 __stdcall16far pass1_1010_7d38(ushort param_1, ushort param_2, int param_3, ushort param_4, ushort param_5)

{
    undefined4 local_e;
    undefined2 uStack10;
    undefined2 local_8;
    undefined  local_6[0x2];
    undefined  local_4[0x2];

    local_e  = *(undefined4 *)(param_3 + 0xc);
    uStack10 = *(undefined2 *)(param_3 + 0x10);
    pass1_1008_3eb4((ushort *)CONCAT22(param_5, &local_e), (ushort *)CONCAT22(param_5, &local_8), (ushort *)CONCAT22(param_5, local_6), (ushort *)CONCAT22(param_5, local_4));
    return local_8;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1010_7d7e(ushort param_1, ushort param_2, int param_3, int param_4)

{
    BOOL16 BVar1;

    if(param_3 != 0x3)
    {
        if((param_4 < 0xa) || (0x7f < param_4))
        {
            return 0x0;
        }
        BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, param_4, 0x3c);
        if(BVar1 != 0x0)
        {
            return 0x0;
        }
        if(((param_4 == 0x6a) && (param_3 != 0x4)) && (param_3 != 0x5))
        {
            return 0x0;
        }
    }
    return 0x1;
}

void __stdcall16far pass1_1010_7e40(ulong *param_1, uchar *param_2, int param_3, ushort param_4)

{
    undefined4   uVar1;
    astruct_652 *puVar2;
    undefined2   uVar2;
    ushort      *puVar3;

    uVar2                               = (undefined2)((ulong)param_1 >> 0x10);
    puVar2                              = (astruct_652 *)param_1;
    *param_1                            = 0x0;
    puVar2->field_0x67c                 = 0x0;
    puVar2->field_0x680                 = 0x0;
    puVar2->field_0xe82                 = 0x0;
    puVar2->field_0xe84                 = 0x0;
    *(undefined4 *)&puVar2->field_0xe88 = 0x0;
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&puVar2->field_0x4), (WNDCLASS16 *)0x0, 0x228);
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&puVar2->field_0x22c), (WNDCLASS16 *)0x0, 0x228);
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&puVar2->field_0x454), (WNDCLASS16 *)0x0, 0x228);
    *(undefined *)&puVar2->field_0x682 = 0x0;
    *(undefined *)&puVar2->field_0xa82 = 0x0;
    _PTR_LOOP_1050_14cc                = param_1;
    puVar3                             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    puVar2->field_0xe88                = (int)puVar3;
    puVar2->field_0xe8a                = (int)((ulong)puVar3 >> 0x10);
    uVar1                              = *(undefined4 *)&puVar2->field_0xe88;
    puVar2->field_0xe84                = *(undefined4 *)((int)uVar1 + 0x64);
    return;
}

void __stdcall16far pass1_1010_8018(ulong param_1, uint param_2, uchar *param_3, ushort param_4)

{
    int     iVar1;
    ushort *uVar2;

    if(*(int *)(param_2 * 0xa + 0x1fa0) != 0x0)
    {
        pass1_1010_878c((astruct_87 **)param_1, *(int *)(param_2 * 0xa + 0x1fa0), param_4);
        uVar2 = (ushort *)(param_1 >> 0x10);
        if(*(long *)((int)param_1 + 0x67c) != 0x0)
        {
            iVar1 = param_2 * 0xa;
            pass1_1008_64c8(*(ulong **)((int)param_1 + 0x67c), CONCAT22(*(undefined2 *)(iVar1 + 0x1fa6), *(undefined2 *)(iVar1 + 0x1fa8)), *(int *)(iVar1 + 0x1fa4), param_2, param_3);
            return;
        }
    }
    return;
}

void __stdcall16far pass1_1010_8170(astruct_87 **param_1, int param_2, uchar *param_3, ushort param_4)

{
    uint       uVar1;
    int        iVar2;
    int        iVar3;
    undefined2 uVar4;

    uVar4 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    uVar1 = *(uint *)(iVar2 + 0x680);
    iVar3 = param_2 * 0x10;
    if(*(uint *)(iVar3 + 0x16) != uVar1)
    {
        pass1_1010_8096((ulong *)param_1, *(int *)(iVar3 + 0x16));
        pass1_1010_878c(param_1, *(int *)(iVar3 + 0x16), param_4);
        if(*(long *)(iVar2 + 0x67c) == 0x0)
        {
            return;
        }
    }
    iVar3 = param_2 * 0x10;
    pass1_1008_6562(*(ulong **)(iVar2 + 0x67c), CONCAT22(*(undefined2 *)(iVar3 + 0x1c), *(undefined2 *)(iVar3 + 0x1e)), *(int *)(iVar3 + 0x1a), uVar1, param_3);
    return;
}


void __stdcall16far pass1_1010_81f6(HINSTANCE16 param_1, ushort param_2, astruct_87 **param_3, long param_4, int param_5)

{
    ushort     uVar1;
    undefined2 uVar2;
    ushort     uStack12;
    ulong      uStack10;

    if(param_4 == 0x8000001)
    {
        uStack10 = (ulong)param_3 & 0xffff0000 | (ulong)((ushort)param_3 + 0x22c);
        uStack12 = 0xfa;
    }
    else
    {
        if(param_4 == 0x8000002)
        {
            uStack10 = (ulong)param_3 & 0xffff0000 | (ulong)((ushort)param_3 + 0x454);
            uStack12 = 0xfc;
        }
        else
        {
            uStack10 = (ulong)param_3 & 0xffff0000 | (ulong)((ushort)param_3 + 0x4);
            uStack12 = 0x2;
        }
    }
    uVar2 = (undefined2)(uStack10 >> 0x10);
    uVar1 = param_3._2_2_;
    if(*(long *)(param_5 * 0x4 + (int)uStack10) == 0x0)
    {
        if((0x0 < param_5) && (param_5 < 0xa))
        {
            pass1_1010_89f0((ushort)param_3, param_3._2_2_, uStack12, uStack10, param_1, param_2);
            return;
        }
        if(param_5 < 0xa)
        {
            return;
        }
        if(0x7f < param_5)
        {
            return;
        }
        if(*(long *)((int)uStack10 + 0x14) == 0x0)
        {
            pass1_1010_89f0((ushort)param_3, param_3._2_2_, uStack12, uStack10, param_1, param_2);
        }
        pass1_1010_887a(param_3, uStack10, param_5, param_1, param_2);
    }
    pass1_1010_866c(uVar1, (ushort)param_3, param_3._2_2_, uStack10, param_5);
    return;
}


void __stdcall16far pass1_1010_82f8(ulong param_1, ushort param_2)

{
    *(ushort *)((int)param_1 + 0xe82) = param_2;
    return;
}

void __stdcall16far pass1_1010_86de(ushort param_1, ushort param_2, uchar param_3, ulong param_4)

{
    long      *plVar1;
    int        iVar2;
    bool       bVar3;
    undefined2 uVar4;
    long       lVar5;
    ulong      uVar6;
    long       lStack20;
    undefined4 uStack10;

    uVar6    = pass1_1008_4772((astruct_76 *)param_4);
    uVar4    = (undefined2)(uVar6 >> 0x10);
    uStack10 = 0x0;
    do
    {
        plVar1 = (long *)((int)uVar6 + 0x8);
        if(*plVar1 == uStack10 || *plVar1 < uStack10)
        {
            return;
        }
        lVar5 = uStack10;
        pass1_1008_4544(param_4);
        iVar2 = (int)lVar5;
        bVar3 = false;
        for(lStack20 = 0x0; plVar1 = (long *)((int)uVar6 + 0x4), *plVar1 != lStack20 && lStack20 <= *plVar1; lStack20 = lStack20 + 0x1)
        {
            if(bVar3)
            {
            LAB_1010_86fc:
                if(bVar3)
                {
                    if(*(char *)((int)lStack20 + iVar2) == -0x1)
                    {
                        *(uchar *)((int)lStack20 + iVar2) = param_3;
                        break;
                    }
                }
            }
            else
            {
                if(*(char *)((int)lStack20 + iVar2) == -0x1)
                    goto LAB_1010_86fc;
                *(uchar *)((int)lStack20 + iVar2 + -0x1) = param_3;
                bVar3                                    = true;
            }
        }
        uStack10 = uStack10 + 0x1;
    } while(true);
}

void __stdcall16far pass1_1010_887a(astruct_87 **param_1, undefined4 param_2, int param_3, undefined2 param_4, ushort param_5)

{
    uint        uVar1;
    ulong       uVar2;
    ulong       uVar3;
    uchar      *in_DX;
    uchar      *puVar4;
    undefined2  extraout_DX;
    int         iVar5;
    undefined2  uVar6;
    undefined2  uVar7;
    byte        bVar8;
    undefined   local_26[0x6];
    ushort      uStack32;
    ushort      uStack30;
    ulong       uStack28;
    undefined4  uStack24;
    ulong       uStack20;
    ulong       uStack16;
    astruct_76 *paStack12;
    astruct_76 *paStack8;
    uint        uStack4;

    uStack4 = param_3 - 0xa;
    pass1_1010_878c(param_1, *(int *)(uStack4 * 0xa + 0x3382), param_4);
    uVar6 = (undefined2)((ulong)param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x67c) != 0x0)
    {
        iVar5 = uStack4 * 0xa;
        uVar1 = uStack4;
        pass1_1008_6562(*(ulong **)((int)param_1 + 0x67c), CONCAT22(*(undefined2 *)(iVar5 + 0x3388), *(undefined2 *)(iVar5 + 0x338a)), *(int *)(iVar5 + 0x3386), uStack4, in_DX);
        paStack8  = (astruct_76 *)CONCAT22(in_DX, uVar1);
        uVar6     = (undefined2)((ulong)param_2 >> 0x10);
        paStack12 = *(astruct_76 **)((int)param_2 + 0x14);
        uStack16  = pass1_1008_4772(paStack12);
        uStack20  = pass1_1008_4772(paStack8);
        puVar4    = (uchar *)(uStack20 >> 0x10);
        uVar2     = *(ulong *)((int)uStack20 + 0x4);
        uVar7     = (undefined2)(uStack16 >> 0x10);
        iVar5     = (int)uStack16;
        if((long)uVar2 < *(long *)(iVar5 + 0x4))
        {
            uVar2 = (ulong) * (uint *)(iVar5 + 0x4);
        }
        uVar3 = *(ulong *)((int)uStack20 + 0x8);
        if((long)uVar3 < *(long *)(iVar5 + 0x8))
        {
            uVar3 = (ulong) * (uint *)(iVar5 + 0x8);
        }
        uVar1    = (uint)uVar3;
        uStack24 = uVar3 & 0xffff | uVar2 << 0x10;
        bVar8    = 0x1e;
        mem_op_1000_179c(0x1e, puVar4, 0x1000);
        if(((uint)puVar4 | uVar1) == 0x0)
        {
            uVar1 = 0x0;
            uVar7 = 0x0;
        }
        else
        {
            struct_op_1008_6604((astruct_85 *)CONCAT22(puVar4, uVar1), (int)uStack24, (int)(uStack24 >> 0x10));
            uVar7 = extraout_DX;
        }
        uStack28 = CONCAT22(uVar7, uVar1);
        pass1_1008_431c(CONCAT22(uVar7, uVar1), bVar8);
        uVar7    = (undefined2)(uStack16 >> 0x10);
        uStack30 = (uStack24._2_2_ - *(int *)((int)uStack16 + 0x4)) / 0x2;
        uStack32 = (int)uStack24 - *(int *)((int)uStack16 + 0x8);
        pass1_1008_3e54((ushort *)CONCAT22(param_5, local_26), 0x0, uStack32, uStack30);
        pass1_1008_4480(uStack28, (ushort *)CONCAT22(param_5, local_26), paStack12, param_5);
        pass1_1008_3e76((ushort *)CONCAT22(param_5, local_26), 0x0, 0x0, 0x7);
        pass1_1008_4480(uStack28, (ushort *)CONCAT22(param_5, local_26), paStack8, param_5);
        *(ulong *)(param_3 * 0x4 + (int)param_2) = uStack28;
    }
    return;
}

void __stdcall16far pass1_1010_6566(ulong param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    ushort uVar1;
    ushort uVar2;
    int    local_4;

    uVar1 = (ushort)param_1;
    uVar2 = (ushort)(param_1 >> 0x10);
    switch_1010_6646(uVar1, uVar2, (ushort *)CONCAT22(param_5, &local_4), param_4);
    if(local_4 != 0x0)
    {
        *(ushort *)(uVar1 + local_4)       = param_3;
        *(ushort *)(uVar1 + local_4 + 0x2) = param_2;
    }
    return;
}


int __stdcall16far pass1_1010_659a(ulong param_1, ushort param_2, ushort param_3)

{
    ushort uVar1;
    ushort uVar2;
    int    local_4;

    uVar1 = (ushort)param_1;
    uVar2 = (ushort)(param_1 >> 0x10);
    switch_1010_6646(uVar1, uVar2, (ushort *)CONCAT22(param_3, &local_4), param_2);
    if(local_4 == 0x0)
    {
        return 0x0;
    }
    return *(int *)(uVar1 + local_4) - *(int *)(uVar1 + local_4 + 0x2);
}


ushort __stdcall16far pass1_1010_65d0(ushort param_1, ulong param_2, ushort param_3)

{
    ushort uVar1;
    int    local_4;

    uVar1 = (ushort)(param_2 >> 0x10);
    switch_1010_6646((ushort)param_2, uVar1, (ushort *)CONCAT22(param_1, &local_4), param_3);
    if(local_4 == 0x0)
    {
        return 0x0;
    }
    return *(ushort *)((ushort)param_2 + local_4 + 0x2);
}


void __stdcall16far pass1_1010_6604(ulong param_1, ushort param_2, ushort param_3)

{
    int    iVar1;
    ushort uVar2;
    ushort uVar3;
    int    local_4;

    uVar2 = (ushort)param_1;
    uVar3 = (ushort)(param_1 >> 0x10);
    switch_1010_6646(uVar2, uVar3, (ushort *)CONCAT22(param_3, &local_4), param_2);
    if(local_4 != 0x0)
    {
        iVar1                            = *(int *)(uVar2 + local_4 + 0x2);
        *(undefined2 *)(uVar2 + local_4) = *(undefined2 *)(uVar2 + local_4);
        *(int *)(uVar2 + local_4 + 0x2)  = iVar1 + 0x1;
        pass1_1010_1f62(param_3, param_1 & 0xffff | (ulong)uVar3 << 0x10, 0x15);
    }
    return;
}


void __stdcall16far switch_1010_6646(ushort param_1, ushort param_2, ushort *param_3, ushort param_4)

{
    switch(param_4)
    {
    case 0x83:
        *param_3 = 0xa;
        break;
    case 0x84:
        *param_3 = 0xe;
        break;
    case 0x85:
        *param_3 = 0x12;
        break;
    case 0x86:
        *param_3 = 0x16;
        return;
    case 0x87:
        *param_3 = 0x1a;
        return;
    case 0x88:
        *param_3 = 0x1e;
        return;
    case 0x89:
        *param_3 = 0x22;
        return;
    default:
        *param_3 = 0x0;
        return;
    }
    return;
}

void __stdcall16far pass1_1010_6814(ulong param_1, ushort param_2, int param_3)

{
    *(ushort *)((int)param_1 + param_3 * 0x2 + 0x11e) = param_2;
    return;
}


void __stdcall16far pass1_1010_682e(ulong param_1, ushort param_2, int param_3)

{
    *(ushort *)((int)param_1 + param_3 * 0x2 + 0xa) = param_2;
    return;
}

void __stdcall16far pass1_1010_6bb2(ushort *param_1, ushort param_2)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    uint        uVar3;
    int         iVar4;
    uint        uVar5;
    uint        uVar6;
    uint        uVar7;
    undefined2 *puStack14;

    uVar7                        = (uint)((ulong)param_1 >> 0x10);
    uVar6                        = (uint)param_1;
    *param_1                     = 0x7e28;
    *(undefined2 *)(uVar6 + 0x2) = 0x1010;
    *(undefined2 *)(uVar6 + 0xa) = 0x7e38;
    *(undefined2 *)(uVar6 + 0xc) = 0x1010;
    puVar1                       = (undefined4 *)*(uint *)(uVar6 + 0x1c);
    uVar3                        = *(uint *)(uVar6 + 0x1e);
    if((uVar3 | (uint)puVar1) != 0x0)
    {
        ppcVar2 = (code **)*puVar1;
        (**ppcVar2)();
    }
    *(undefined4 *)(uVar6 + 0x1c) = 0x0;
    if(*(long *)(uVar6 + 0x14) != 0x0)
    {
        uVar3 = uVar7 | uVar6;
        if(param_1 == (ushort *)0x0)
        {
            uVar5 = 0x0;
        }
        else
        {
            uVar3 = uVar6 + 0xa;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6(*(ulong *)(uVar6 + 0x14), CONCAT22(uVar5, uVar3), param_2);
    }
    if(*(long *)(uVar6 + 0x22) != 0x0)
    {
        uVar3 = uVar7 | uVar6;
        if(param_1 == (ushort *)0x0)
        {
            uVar5 = 0x0;
        }
        else
        {
            uVar3 = uVar6 + 0xa;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6(*(ulong *)(uVar6 + 0x22), CONCAT22(uVar5, uVar3), param_2);
    }
    *(undefined4 *)(uVar6 + 0x14) = 0x0;
    *(undefined4 *)(uVar6 + 0x22) = 0x0;
    if(param_1 == (ushort *)0x0)
    {
        iVar4 = 0x0;
        uVar7 = 0x0;
    }
    else
    {
        iVar4 = uVar6 + 0xa;
    }
    puStack14                    = (undefined2 *)CONCAT22(uVar7, iVar4);
    *puStack14                   = 0x389a;
    *(undefined2 *)(iVar4 + 0x2) = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}

ushort __cdecl16far pass1_1010_6ca2(ulong param_1, int param_2, ushort param_3, ushort param_4)

{
    undefined4 uVar1;
    ushort    *puVar2;
    ushort     uVar3;
    int        iStack10;
    int        iStack8;

    _iStack8 = (ushort *)CONCAT22(param_4, &stack0x000a);
    iStack10 = param_2;
    do
    {
        puVar2 = _iStack8;
        if(iStack10 == 0x0)
        {
            return 0x1;
        }
        _iStack8 = (ushort *)((ulong)_iStack8 & 0xffff0000 | (ulong)(iStack8 + 0x2));
        uVar3    = *puVar2;
        uVar1    = *(undefined4 *)((int)param_1 + 0x14);
        pass1_1010_a5ca((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), uVar3, uVar3, param_3);
        iStack10 = iStack10 + -0x1;
    } while(uVar3 == 0x0);
    return 0x0;
}

void __stdcall16far pass1_1010_715c(ulong param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6)

{
    pass1_1010_a69c(*(ulong *)((int)param_1 + 0x14), param_2, param_3, param_4, param_5, param_6);
    return;
}

void __stdcall16far pass1_1010_52fc(ulong param_1, ulong param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    undefined2 uVar1;

    pass1_1010_533c(param_1, param_2, param_4, param_5);
    uVar1                            = (undefined2)(param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0x12) = param_3;
    *(uchar **)((int)param_1 + 0x14) = param_4;
    return;
}


void __stdcall16far pass1_1010_531c(ulong param_1, ulong param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    undefined2 uVar1;

    pass1_1010_533c(param_1, param_2, param_4, param_5);
    uVar1                            = (undefined2)(param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0x16) = param_3;
    *(uchar **)((int)param_1 + 0x18) = param_4;
    return;
}

ulong __stdcall16far pass1_1010_5f7a(int param_1, ushort param_2, ushort param_3, int param_4)

{
    int iVar1;

    iVar1 = param_4 * 0x8 + param_1;
    if((*(int *)(iVar1 + 0x26) == 0x0) && (*(int *)(iVar1 + 0x28) == 0x0))
    {
        return 0x0;
    }
    return CONCAT22(param_2, param_4 * 0x8 + param_1 + 0x22);
}


void __stdcall16far pass1_1010_5fb0(ulong param_1, ushort param_2, ulong *param_3, ushort param_4, int param_5)

{
    undefined2   uVar1;
    astruct_656 *iVar1;

    uVar1             = (undefined2)(param_1 >> 0x10);
    iVar1             = (astruct_656 *)((int)param_1 + param_5 * 0x8);
    iVar1->field_0x22 = *param_3;
    iVar1->field_0x26 = param_3[0x1];
    return;
}

void __stdcall16far pass1_1010_60a0(ulong param_1)

{
    *(undefined4 *)((int)param_1 + 0x76) = 0x5;
    return;
}


ushort __stdcall16far pass1_1010_60b4(void)

{
    return 0x1;
}


ushort __stdcall16far pass1_1010_60ba(void)

{
    return 0x1;
}


ushort __stdcall16far pass1_1010_60c0(void)

{
    return 0x1;
}


ushort __stdcall16far pass1_1010_60c6(void)

{
    return 0x1;
}

void __stdcall16far pass1_1010_60fa(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                         = (undefined2)(param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined2 *)(iVar1 + 0x7e) = 0x1;
    *(undefined2 *)(iVar1 + 0x7c) = *(undefined2 *)(iVar1 + 0x20);
    *(undefined2 *)(iVar1 + 0x20) = 0x1;
    return;
}


void __stdcall16far pass1_1010_6118(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0x7e) != 0x0)
    {
        *(undefined2 *)(iVar1 + 0x20) = *(undefined2 *)(iVar1 + 0x7c);
    }
    return;
}

ulong __stdcall16far pass1_1010_454a(ulong param_1)

{
    int        iVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    iVar2 = *(int *)(iVar1 + 0x24) * 0x4;
    return CONCAT22(*(undefined2 *)(iVar1 + iVar2 + 0x28), *(undefined2 *)(iVar1 + iVar2 + 0x26));
}


void __stdcall16far pass1_1010_4566(int param_1, ushort param_2, int param_3, ushort param_4)

{
    if(param_3 != 0x2)
    {
        return;
    }
    pass1_1010_4956(CONCAT22(param_2, param_1 + -0x20));
    pass1_1010_1f62(param_4, CONCAT22(param_2, param_1 + -0x20), 0x2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_459e(long param_1)

{
    uint   uVar1;
    uchar *puVar2;

    if(param_1 == 0x0)
    {
        uVar1  = 0x0;
        puVar2 = (uchar *)0x0;
    }
    else
    {
        uVar1  = (int)param_1 + 0x20;
        puVar2 = param_1._2_2_;
    }
    pass1_1008_9262((int)_PTR_LOOP_1050_0388, (ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10), 0x1f4, CONCAT22(puVar2, uVar1), uVar1, puVar2);
    *(undefined2 *)((int)param_1 + 0x7e) = 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_45d6(long param_1, ushort param_2)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;
    int         iVar6;
    undefined2  uVar7;
    int         iStack4;

    uVar7 = (undefined2)((ulong)param_1 >> 0x10);
    iVar6 = (int)param_1;
    if(*(int *)(iVar6 + 0x7e) != 0x0)
    {
        if(_PTR_LOOP_1050_0388 != 0x0)
        {
            if(param_1 == 0x0)
            {
                iVar4 = 0x0;
                uVar5 = 0x0;
            }
            else
            {
                iVar4 = iVar6 + 0x20;
                uVar5 = uVar7;
            }
            param_2 = 0x1008;
            pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x1f4, CONCAT22(uVar5, iVar4));
        }
        for(iStack4 = 0x0; iStack4 < 0x10; iStack4 = iStack4 + 0x1)
        {
            if(*(int *)(iVar6 + 0x24) != iStack4)
            {
                puVar1 = (undefined4 *)*(uint *)(iVar6 + 0x26 + iStack4 * 0x4);
                uVar2  = *(uint *)(iVar6 + 0x26 + iStack4 * 0x4 + 0x2);
                if((uVar2 | (uint)puVar1) != 0x0)
                {
                    ppcVar3 = (code **)*puVar1;
                    (**ppcVar3)(param_2, puVar1, uVar2, 0x1);
                }
                *(undefined4 *)(iVar6 + iStack4 * 0x4 + 0x26) = 0x0;
            }
        }
        *(undefined2 *)(iVar6 + 0x7e) = 0x0;
    }
    return;
}

void __stdcall16far pass1_1010_4788(ulong param_1, char *param_2, ushort param_3, ushort param_4)

{
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), *(ulong *)((int)param_1 + 0x6c));
    pass1_1030_301a(CONCAT22(param_4, param_3), param_2, param_4);
    return;
}

void __stdcall16far pass1_1010_4956(ulong param_1)

{
    int       *piVar1;
    int        iVar2;
    int        iVar3;
    undefined2 uVar4;

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    iVar2 = *(int *)(iVar3 + 0x6a);
    if(iVar2 == 0x0)
    {
        piVar1  = (int *)(iVar3 + 0x24);
        *piVar1 = *piVar1 + 0x1;
        if(0xf < *(int *)(iVar3 + 0x24))
        {
            *(undefined2 *)(iVar3 + 0x24) = 0x0;
            return;
        }
    }
    else
    {
        if(iVar2 != 0x4)
        {
            return;
        }
        piVar1  = (int *)(iVar3 + 0x24);
        *piVar1 = *piVar1 + -0x1;
        if(*piVar1 < 0x0)
        {
            *(undefined2 *)(iVar3 + 0x24) = 0xf;
        }
    }
    return;
}

ulong __stdcall16far pass1_1010_49a0(int param_1, ushort param_2)

{
    return CONCAT22(param_2, param_1 + 0xa);
}


ulong __stdcall16far pass1_1010_49b0(int param_1, ushort param_2)

{
    return CONCAT22(param_2, param_1 + 0x18);
}


ushort __stdcall16far pass1_1010_49c0(ulong param_1)

{
    return *(ushort *)((int)param_1 + 0x14);
}


void __stdcall16far pass1_1010_49ce(ulong param_1, ushort param_2)

{
    *(ushort *)((int)param_1 + 0x14) = param_2;
    return;
}


ushort __stdcall16far pass1_1010_49e0(ulong param_1)

{
    return *(ushort *)((int)param_1 + 0x16);
}


void __stdcall16far pass1_1010_49ee(ulong param_1, ushort param_2)

{
    *(ushort *)((int)param_1 + 0x16) = param_2;
    return;
}


void __stdcall16far pass1_1010_4a00(ulong param_1, ushort param_2)

{
    *(ushort *)((int)param_1 + 0x12) = param_2;
    return;
}


ushort __stdcall16far pass1_1010_4a12(ulong param_1)

{
    return *(ushort *)((int)param_1 + 0x12);
}

ulong __stdcall16far pass1_1010_4c2c(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x18), *(undefined2 *)((int)param_1 + 0x16));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_4c3e(ulong param_1, int param_2, int param_3, uchar *param_4, ushort param_5)

{
    int        *piVar1;
    undefined4  uVar2;
    int         iVar3;
    int         iVar4;
    undefined2  uVar5;
    undefined2  uVar6;
    astruct_43 *paVar7;
    ulong       uVar8;
    int         iStack14;
    undefined   local_c[0x6];
    undefined2  uStack6;
    int         iStack4;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    pass1_1010_bffa(*(ulong *)(iVar3 + 0x26), param_3, param_4, param_5);
    *(int *)(iVar3 + 0x12)    = param_3;
    *(uchar **)(iVar3 + 0x14) = param_4;
    if(((uint)param_4 | *(uint *)(iVar3 + 0x12)) != 0x0)
    {
        if(param_2 == 0x0)
        {
            uVar2                         = *(undefined4 *)(iVar3 + 0x12);
            *(undefined2 *)(iVar3 + 0x30) = *(undefined2 *)((int)uVar2 + 0x8);
        }
        else
        {
            *(undefined2 *)(iVar3 + 0x2e) = 0x1;
            uVar2                         = *(undefined4 *)(iVar3 + 0x12);
            uVar2                         = *(undefined4 *)((int)uVar2 + 0x4);
            iVar4                         = *(int *)((int)uVar2 + 0x2);
            if((iVar4 == 0x5) || (iVar4 == 0x6))
            {
                *(undefined2 *)(iVar3 + 0x30) = 0x1;
                *(undefined2 *)(iVar3 + 0x20) = 0x0;
            }
            else
            {
                *(undefined2 *)(iVar3 + 0x30) = 0x2;
                uVar2                         = *(undefined4 *)*(undefined4 *)(iVar3 + 0x12);
                *(undefined4 *)(iVar3 + 0x32) = *(undefined4 *)((int)uVar2 + 0x4);
                paVar7                        = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1bf, param_5);
                uVar2                         = *(undefined4 *)*(undefined4 *)(iVar3 + 0x12);
                uVar6                         = (undefined2)((ulong)uVar2 >> 0x10);
                iVar4                         = (int)uVar2;
                *(undefined2 *)(iVar4 + 0x4)  = (int)paVar7;
                *(undefined2 *)(iVar4 + 0x6)  = (int)((ulong)paVar7 >> 0x10);
            }
        }
        iStack4 = 0x14;
        pass1_1008_3e38((ushort *)CONCAT22(param_5, local_c));
        uStack6  = 0x0;
        iStack14 = 0x0;
        while(true)
        {
            piVar1 = (int *)(iVar3 + 0x30);
            if(*piVar1 == iStack14 || *piVar1 < iStack14)
                break;
            uVar2    = *(undefined4 *)*(undefined4 *)(iVar3 + 0x12);
            uVar8    = pass1_1008_4772(*(astruct_76 **)((int)uVar2 + iStack14 * 0x4));
            iStack4  = iStack4 + (-(uint)(iStack14 == 0x0) & 0x5) + 0x14 + *(int *)((int)uVar8 + 0x4);
            iStack14 = iStack14 + 0x1;
        }
        if(*(int *)(iVar3 + 0xe) < iStack4)
        {
            *(int *)(iVar3 + 0xe) = iStack4;
        }
    }
    return;
}

ulong __stdcall16far pass1_1010_4dc8(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0x20) == 0x0)
    {
        return 0x0;
    }
    return CONCAT22(*(undefined2 *)(iVar1 + 0x1c), *(int *)(iVar1 + 0x20) * 0x8 + *(int *)(iVar1 + 0x1a));
}


void __stdcall16far pass1_1010_4df0(ulong param_1, uint param_2, ushort param_3)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x26);
    pass1_1010_c1ba((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), *(int *)((int)param_1 + 0x20), param_2, param_3);
    return;
}

void __stdcall16far pass1_1010_4e8c(ulong param_1, ushort param_2)

{
    pass1_1010_1f62(param_2, param_1, 0xd);
    return;
}

ushort __stdcall16far pass1_1010_4f20(ushort param_1, ushort param_2, int param_3)

{
    return *(ushort *)(param_3 * 0x2 + 0x139a);
}


void __stdcall16far pass1_1010_4f30(ushort param_1, ushort param_2, ushort *param_3, ushort *param_4)

{
    *param_4 = 0xa;
    *param_3 = 0x73;
    return;
}

void __stdcall16far pass1_1010_5120(ulong param_1, uint param_2, uint param_3, uint param_4, ushort param_5)

{
    undefined4 uVar1;
    ulong      uVar2;
    uint       uVar3;
    uint       uVar4;
    ulong      uVar5;
    uint       uVar6;
    uint       uVar7;
    int        iVar8;
    int        iVar9;
    undefined2 uVar10;

    uVar10 = (undefined2)(param_1 >> 0x10);
    iVar9  = (int)param_1;
    if(*(long *)(iVar9 + 0x16) != 0x0)
    {
        uVar1 = *(undefined4 *)(iVar9 + 0x16);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        uVar6 = param_4 | param_3;
        if(uVar6 != 0x0)
        {
            uVar2 = *(ulong *)(param_3 + 0x1f6);
            uVar5 = uVar2;
            pass1_1030_38f2(uVar2, 0x3, param_5);
            uVar3 = (uint)uVar5;
            uVar7 = uVar6;
            uVar4 = uVar3;
            pass1_1030_38f2(uVar2, 0x4, param_5);
            iVar8 = uVar7 + uVar6 + (uint)CARRY2(uVar4, uVar3);
            if((0x0 < iVar8) || ((-0x1 < iVar8 && (param_2 <= uVar4 + uVar3))))
            {
                *(uint *)(iVar9 + 0xa) = param_2;
                return;
            }
        }
    }
    return;
}

ulong __stdcall16far pass1_1010_375e(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0xc), *(undefined2 *)((int)param_1 + 0xa));
}

void __stdcall16far pass1_1010_398e(ulong *param_1, ushort param_2, ushort param_3, ulong param_4, ushort param_5)

{
    int        *piVar1;
    code      **ppcVar2;
    ulong       uVar3;
    undefined4  uVar4;
    int         iVar5;
    ushort      uVar6;
    uint        extraout_DX;
    undefined2  extraout_DX_00;
    int         iVar7;
    int         iVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    ushort      uStack12;
    undefined4 *puStack6;

    uVar9   = (undefined2)((ulong)param_1 >> 0x10);
    uVar3   = *param_1;
    ppcVar2 = (code **)((int)uVar3 + 0x8);
    (**ppcVar2)();
    puStack6 = (undefined4 *)CONCAT22(extraout_DX, param_5);
    if((extraout_DX | param_5) == 0x0)
    {
        return;
    }
    *(ulong *)(param_5 + 0xc) = param_4;
    iVar7                     = (int)*puStack6;
    ppcVar2                   = (code **)(iVar7 + 0xc);
    (**ppcVar2)();
    iVar5   = *(int *)((int)param_1 + 0x14);
    piVar1  = (int *)((int)param_1 + 0x14);
    *piVar1 = *piVar1 + 0x1;
    ppcVar2 = (code **)(iVar7 + 0x10);
    (**ppcVar2)();
    ppcVar2 = (code **)(iVar7 + 0x4);
    (**ppcVar2)();
    if(iVar5 != 0x0)
    {
        ppcVar2 = (code **)((int)uVar3 + 0x8);
        iVar7   = iVar5;
        (**ppcVar2)();
        *(int *)(param_5 + 0x8)        = iVar7;
        *(undefined2 *)(param_5 + 0xa) = extraout_DX_00;
        PTR_LOOP_1050_11de             = PTR_LOOP_1050_11de + 0x1;
        uVar9                          = extraout_DX_00;
        for(uStack12 = 0x0; (int)uStack12 < iVar5; uStack12 = uStack12 + 0x1)
        {
            uVar6 = uStack12;
            pass1_1010_398e(param_1, uStack12, (int)uStack12 >> 0xf, (ulong)puStack6, uStack12);
            uVar4                                = *(undefined4 *)(param_5 + 0x8);
            uVar10                               = (undefined2)((ulong)uVar4 >> 0x10);
            iVar7                                = (int)uVar4;
            iVar8                                = uStack12 * 0x4;
            *(ushort *)(iVar7 + iVar8)           = uVar6;
            *(undefined2 *)(iVar7 + iVar8 + 0x2) = uVar9;
            uVar4                                = *(undefined4 *)(param_5 + 0x8);
            if(*(long *)((int)uVar4 + iVar8) == 0x0)
                break;
        }
        PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + -0x1;
    }
    return;
}

ushort __stdcall16far pass1_1010_3a86(ulong param_1)

{
    return *(ushort *)((int)param_1 + 0x10);
}


void __stdcall16far pass1_1010_3a94(ulong param_1, ushort param_2)

{
    *(ushort *)((int)param_1 + 0x12) = param_2;
    return;
}


ulong __stdcall16far pass1_1010_3aaa(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x6), *(undefined2 *)((int)param_1 + 0x4));
}


void __stdcall16far pass1_1010_3ac2(ulong param_1, ushort param_2, ulong param_3)

{
    undefined2 uVar1;

    uVar1                            = (undefined2)(param_1 >> 0x10);
    *(ulong *)((int)param_1 + 0x16)  = param_3;
    *(ushort *)((int)param_1 + 0x12) = param_2;
    return;
}


ulong __stdcall16far pass1_1010_3adc(ulong param_1)

{
    undefined2 *puVar1;

    puVar1 = (undefined2 *)*(undefined4 *)((int)param_1 + 0x16);
    return CONCAT22(*(undefined2 *)((int)puVar1 + 0x2), *puVar1);
}

void __stdcall16far pass1_1010_3bde(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    undefined2  *puVar4;
    astruct_479 *iVar4;
    undefined2   uVar5;
    undefined2  *puStack14;

    uVar5            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_479 *)param_1;
    *param_1         = 0x3d6a;
    iVar4->field_0x2 = 0x1010;
    iVar4->field_0xa = 0x3d7a;
    iVar4->field_0xc = 0x1010;
    puVar1           = iVar4->field_0xe;
    uVar2            = iVar4->field_0x10;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    if(param_1 == (ushort *)0x0)
    {
        puVar4 = (undefined2 *)0x0;
        uVar5  = 0x0;
    }
    else
    {
        puVar4 = &iVar4->field_0xa;
    }
    puStack14   = (undefined2 *)CONCAT22(uVar5, puVar4);
    *puStack14  = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1010_3c52(ulong param_1, ushort param_2, ushort param_3)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_274 *iVar4;
    undefined2   uVar4;
    astruct_43  *paVar5;

    uVar4             = (undefined2)(param_1 >> 0x10);
    iVar4             = (astruct_274 *)param_1;
    iVar4->field_0x14 = param_2;
    puVar1            = iVar4->field_0xe;
    uVar2             = iVar4->field_0x10;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    paVar5            = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, iVar4->field_0x14, param_3);
    iVar4->field_0xe  = (undefined4 *)paVar5;
    iVar4->field_0x10 = (uint)((ulong)paVar5 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_3c9e(long param_1)

{
    uint   uVar1;
    uchar *puVar2;

    if(param_1 == 0x0)
    {
        uVar1  = 0x0;
        puVar2 = (uchar *)0x0;
    }
    else
    {
        uVar1  = (int)param_1 + 0xa;
        puVar2 = param_1._2_2_;
    }
    pass1_1008_9262((int)_PTR_LOOP_1050_0388, (ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10), (ulong) * (uint *)((int)param_1 + 0x12), CONCAT22(puVar2, uVar1), uVar1, puVar2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_3cd0(long param_1)

{
    int        iVar1;
    undefined2 uVar2;

    if(_PTR_LOOP_1050_0388 != 0x0)
    {
        if(param_1 == 0x0)
        {
            iVar1 = 0x0;
            uVar2 = 0x0;
        }
        else
        {
            iVar1 = (int)param_1 + 0xa;
            uVar2 = param_1._2_2_;
        }
        pass1_1008_92b2(_PTR_LOOP_1050_0388, (ulong) * (uint *)((int)param_1 + 0x12), CONCAT22(uVar2, iVar1));
    }
    return;
}


void __stdcall16far pass1_1010_3d0a(int param_1, ushort param_2, int param_3, ushort param_4)

{
    if(param_3 == 0x2)
    {
        pass1_1010_3cd0(CONCAT22(param_2, param_1 + -0xa));
        pass1_1010_1f62(param_4, CONCAT22(param_2, param_1 + -0xa), 0x2);
    }
    return;
}


void __stdcall16far pass1_1010_3dc8(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_480 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_480 *)param_1;
    *param_1         = 0x3e2c;
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

ulong __stdcall16far pass1_1010_40cc(ulong param_1, int param_2, ushort param_3)

{
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), *(ulong *)((int)param_1 + 0x6c));
    return CONCAT22(*(undefined2 *)(param_2 + 0xe), *(undefined2 *)(param_2 + 0xc));
}

void __stdcall16far pass1_1010_2816(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    int         iVar5;
    undefined2  uVar6;

    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(int *)(iVar4 + 0x124) != 0x0)
    {
        iVar5  = *(int *)(iVar4 + 0x124) * 0x4;
        puVar1 = (undefined4 *)*(uint *)(iVar5 + iVar4);
        uVar2  = *(uint *)(iVar5 + iVar4 + 0x2);
        if((uVar2 | (uint)puVar1) != 0x0)
        {
            ppcVar3 = (code **)*puVar1;
            (**ppcVar3)();
        }
        *(undefined4 *)(*(int *)(iVar4 + 0x124) * 0x4 + iVar4) = 0x0;
        *(undefined2 *)(iVar4 + 0x124)                         = 0x0;
    }
    return;
}


ushort __cdecl16far pass1_1010_286c(void)

{
    ushort *puVar1;

    pass1_1008_3e54((ushort *)&PTR_LOOP_1048_0000, 0x0, 0x5, 0x12c);
    pass1_1008_3e54((ushort *)0x105065a6, 0x0, 0x9b, 0x20);
    pass1_1008_3e54((ushort *)0x105065ac, 0x0, 0xf5, 0x3f);
    pass1_1008_3e54((ushort *)0x105065b2, 0x0, 0x114, 0x4a);
    pass1_1008_3e54((ushort *)0x105065b8, 0x0, 0x135, 0x45);
    pass1_1008_3e54((ushort *)0x105065be, 0x0, 0xf5, 0x7b);
    puVar1 = pass1_1008_3e54((ushort *)0x105065c4, 0x0, 0x117, 0x91);
    return (ushort)puVar1;
}

void __stdcall16far pass1_1010_2b50(ushort param_1, ushort param_2, ushort *param_3)

{
    pass1_1008_3f62(param_3, (ushort *)&PTR_LOOP_1048_0000);
    return;
}


ulong __stdcall16far pass1_1010_2b66(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x1e), *(undefined2 *)((int)param_1 + 0x1c));
}


void __stdcall16far pass1_1010_2b78(ushort param_1, ushort param_2, int param_3, ulong param_4)

{
    undefined4 *puVar1;
    undefined4 *puVar2;
    undefined4 *puVar3;
    int         iVar4;
    undefined4 *puVar5;

    puVar3 = (undefined4 *)(param_3 * 0x7c + 0xed4);
    puVar5 = (undefined4 *)param_4;
    for(iVar4 = 0x1f; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
    {
        puVar2  = puVar5;
        puVar5  = puVar5 + 0x1;
        puVar1  = puVar3;
        puVar3  = puVar3 + 0x1;
        *puVar2 = *puVar1;
    }
    return;
}


ulong __stdcall16far pass1_1010_2b98(ulong param_1, int param_2)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar1 = *(undefined4 *)((int)param_1 + 0x28);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(param_2 * 0x4 + iVar2 + -0x156), *(undefined2 *)(param_2 * 0x4 + iVar2 + -0x158));
}


void __cdecl16far pass1_1010_2bb9(void)

{
    pass1_1010_286c();
    return;
}

void __stdcall16far struct_1010_2cd2(astruct_79 *param_1, astruct_79 *param_2, ushort param_3, ushort param_4)

{
    int         unaff_DI;
    astruct_79 *paVar1;
    ushort     *puVar2;
    int        *piVar3;
    int        *piVar4;
    ushort      uVar5;
    int         local_6;
    int         local_4;

    paVar1                                              = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1[0x1].field_0x8              = 0x0;
    param_1[0x2].field_0x2                              = 0x0;
    *(undefined2 *)&param_1[0x2].field_0x4              = 0x0;
    *(undefined2 *)&param_1[0x3].field_0x4              = 0x0;
    *(undefined2 *)((int)&param_1[0x3].field_0x4 + 0x2) = 0x0;
    param_1[0x3].field_0x8                              = 0x0;
    ((astruct_79 *)(param_1 + 0x4))->field_0x0          = 0x0;
    *(undefined4 *)&param_1[0x8].field_0x2              = 0x0;
    *(undefined4 *)((int)&param_1[0x8].field_0x4 + 0x2) = 0x0;
    ((astruct_79 *)(param_1 + 0x9))->field_0x0          = 0x0;
    *(undefined2 *)&param_1[0x9].field_0x4              = 0x0;
    param_1[0x9].field_0x2                              = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1)           = 0x36da;
    param_1->field_0x2                                  = 0x1010;
    piVar4                                              = &local_4;
    piVar3                                              = &local_6;
    uVar5                                               = param_4;
    puVar2                                              = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, (uchar *)((ulong)paVar1 >> 0x10), unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)), (ushort *)CONCAT22(param_4, piVar3), (ushort *)CONCAT22(uVar5, piVar4));
    param_1[0x1].field_0x4                              = 0x19001db;
    ((astruct_79 *)(param_1 + 0x1))->field_0x0          = 0x140 - (*(int *)&param_1[0x1].field_0x4 / 0x2 - local_4);
    param_1[0x1].field_0x2                              = 0xf0 - (*(int *)((int)&param_1[0x1].field_0x4 + 0x2) / 0x2 - local_6);
    *(undefined4 *)((int)&param_1[0x2].field_0x4 + 0x2) = 0xa006e;
    *(undefined4 *)(param_1 + 0x3)                      = 0xa012c;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1[0x4].field_0x2), (WNDCLASS16 *)0x0, 0x28);
    return;
}


ulong __stdcall16far pass1_1010_2e02(ulong param_1, int param_2)

{
    undefined4   uVar1;
    astruct_163 *iVar2;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x5c) != 0x0)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x5c);
        uVar2 = (undefined2)((ulong)uVar1 >> 0x10);
        iVar2 = (astruct_163 *)uVar1;
        return CONCAT22(*(undefined2 *)(iVar2 + param_2 * 0x4 + 0x2), *(undefined2 *)(iVar2 + param_2 * 0x4));
    }
    return 0x0;
}


void __stdcall16far pass1_1010_2e30(ulong param_1, ushort param_2, ushort param_3, int param_4)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x5c) != 0x0)
    {
        uVar1                                    = *(undefined4 *)((int)param_1 + 0x5c);
        uVar3                                    = (undefined2)((ulong)uVar1 >> 0x10);
        iVar2                                    = (int)uVar1;
        *(ushort *)(iVar2 + param_4 * 0x4)       = param_2;
        *(ushort *)(iVar2 + param_4 * 0x4 + 0x2) = param_3;
    }
    return;
}


void __stdcall16far pass1_1010_2e5c(ushort param_1, ushort param_2, ulong param_3)

{
    ulong uStack12;

    uStack12 = param_3;
    if(param_3 == 0x0)
    {
        return;
    }
    for(; (uStack12 & 0xf) != 0x0; uStack12 = uStack12 >> 0x4)
    {
    }
    return;
}


void __stdcall16far pass1_1010_2ee2(ulong *param_1, ushort param_2, ushort param_3)

{
    undefined4 uVar1;
    code     **ppcVar2;
    int        iVar3;
    undefined2 extraout_DX;
    int        iVar4;
    undefined2 uVar5;
    ulong      uVar6;
    ulong     *puStack6;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0x52) != 0x0)
    {
        return;
    }
    iVar3                         = 0x0;
    *(undefined2 *)(iVar4 + 0x28) = 0x0;
    uVar6                         = *param_1;
    ppcVar2                       = (code **)((int)uVar6 + 0x20);
    (**ppcVar2)(param_3, param_1, *(undefined4 *)(iVar4 + 0x12));
    if(iVar3 == 0x0)
    {
        puStack6 = *(ulong **)(iVar4 + 0x56);
    }
    else
    {
        uVar1   = *(undefined4 *)(iVar4 + 0x12);
        ppcVar2 = (code **)((int)uVar6 + 0x14);
        (**ppcVar2)(param_3, param_1, (char)uVar1, (int)((ulong)uVar1 >> 0x10));
        puStack6 = (ulong *)CONCAT22(extraout_DX, iVar3);
        uVar6    = pass1_1010_2e02((ulong)param_1, *(int *)(iVar3 + 0x12));
        pass1_1010_35a4(param_1, uVar6, (int)(uVar6 >> 0x10));
    }
    pass1_1010_32f4(param_1, puStack6, param_2, param_3);
    pass1_1010_1f62(param_2, (ulong)param_1, 0x8);
    if(*(long *)(iVar4 + 0x52) != 0x0)
    {
        return;
    }
    return;
}

void __stdcall16far pass1_1010_32c0(ulong param_1, ulong param_2)

{
    undefined2 uVar1;

    uVar1                                = (undefined2)(param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x28) = 0x0;
    *(ulong *)((int)param_1 + 0x12)      = param_2;
    return;
}


void __stdcall16far pass1_1010_32da(ulong *param_1, ulong param_2, ushort param_3, ushort param_4)

{
    pass1_1010_32f4(param_1, *(ulong **)((int)param_2 + 0x42), param_4, param_3);
    return;
}

void __stdcall16far string_1010_1722(ushort param_1, ushort param_2, ushort param_3, ushort param_4, ulong param_5)

{
    uint       extraout_DX;
    undefined2 uVar1;
    char      *pcVar2;
    undefined  local_52[0x50];

    pass1_1028_b58e(param_5);
    if((extraout_DX | param_2) == 0x0)
    {
        pcVar2 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), (HINSTANCE16)&USHORT_1050_1028);
        uVar1  = (undefined2)((ulong)pcVar2 >> 0x10);
        unk_str_op_1000_3d3e((char *)CONCAT22(param_1, local_52), pcVar2);
        pcVar2 = (char *)CONCAT22(uVar1, local_52);
    }
    else
    {
        pcVar2  = pass1_1038_4d28(*(ulong *)(param_2 + 0x2e));
        param_1 = (ushort)((ulong)pcVar2 >> 0x10);
    }
    str_op_1008_60e8((char *)((ulong)pcVar2 & 0xffff | (ulong)param_1 << 0x10), (ushort)((ulong)pcVar2 >> 0x10));
    return;
}

void __cdecl16far pass1_1010_184a(ulong *param_1, ulong *param_2)

{
    int iVar1;
    int iVar2;

    iVar2 = DAT_1050_0ecc;
    iVar1 = *(int *)(DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    iVar1 = pass1_1000_475e(*(ulong *)(iVar1 + (int)*param_1), *(ulong *)(iVar1 + (int)*param_2));
    if(iVar1 == 0x0)
    {
        iVar1 = *(int *)(iVar2 * 0x6 + 0xebc) * 0x8;
        iVar1 = pass1_1000_475e(*(ulong *)(iVar1 + (int)*param_1), *(ulong *)(iVar1 + (int)*param_2));
        if(iVar1 == 0x0)
        {
            iVar2 = *(int *)(iVar2 * 0x6 + 0xebe) * 0x8;
            pass1_1000_475e(*(ulong *)(iVar2 + (int)*param_1), *(ulong *)(iVar2 + (int)*param_2));
        }
    }
    return;
}

void __stdcall16far pass1_1010_19a4(ulong *param_1, uint param_2, ushort param_3)

{
    code     **ppcVar1;
    undefined *puVar2;
    uint       extraout_DX;
    undefined  local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    while(true)
    {
        puVar2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_3, puVar2));
        if((param_2 | (uint)puVar2) == 0x0)
            break;
        ppcVar1 = (code **)((int)*param_1 + 0x40);
        (**ppcVar1)((int)&USHORT_1050_1028, param_1);
        param_2 = extraout_DX;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_1a06(ulong param_1, ulong param_2, int param_3, ushort param_4)

{
    char      *pcVar1;
    int        iVar2;
    ushort     uVar3;
    uchar     *puVar4;
    ushort     uVar5;
    undefined2 uVar6;
    undefined4 uVar7;
    ushort    *puVar8;
    char      *pcVar9;
    int        in_stack_0000ffee;

    uVar7  = pass1_1028_b58e(param_2);
    puVar4 = (uchar *)((ulong)uVar7 >> 0x10);
    uVar6  = (undefined2)(param_1 >> 0x10);
    pcVar1 = pass1_1010_b038(*(uchar **)((int)param_1 + 0x6e), (ushort)uVar7, (ushort)puVar4, (uchar *)0x1770, in_stack_0000ffee);
    iVar2  = pass1_1000_3e2c(CONCAT22(puVar4, pcVar1));
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_4, puVar4, param_3);
    uVar5  = (ushort)((ulong)puVar8 >> 0x10);
    uVar3  = pass1_1010_7818((ulong)puVar8, param_2);
    uVar7  = *(undefined4 *)((int)param_1 + 0x6e);
    pcVar9 = string_op_1010_ada6(0x1000, uVar5, (ushort)uVar7, (ushort)((ulong)uVar7 >> 0x10), iVar2, uVar3);
    str_op_1008_60e8(pcVar9, (ushort)((ulong)pcVar9 >> 0x10));
    return;
}

uchar __stdcall16far pass1_1010_1a66(ulong param_1, ulong param_2)

{
    undefined4 uVar1;
    uchar      uVar2;
    ushort     uVar3;
    BOOL16     BVar4;
    uint       uVar5;
    undefined2 uVar6;
    ulong      uVar7;

    uVar5 = (uint)(param_2 >> 0x10);
    if((*(int *)((int)param_2 + 0x1c) != 0x2) || ((*(uint *)((int)param_2 + 0x1e) & 0xff) != 0x0))
    {
        uVar7 = pass1_1028_b58e(param_2 & 0xffff | (ulong)uVar5 << 0x10);
        uVar6 = (undefined2)(param_1 >> 0x10);
        uVar1 = *(undefined4 *)((int)param_1 + 0x6e);
        pass1_1010_c2d8((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), uVar7);
        if(((int)uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0))
        {
            uVar1 = *(undefined4 *)((int)param_1 + 0x6e);
            uVar3 = pass1_1010_b028((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2);
            BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar3, 0x5);
            if((BVar4 == 0x0) && (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar3, 0x6), BVar4 == 0x0))
            {
                uVar2 = '\0';
            }
            else
            {
                uVar2 = '\x01';
            }
            return uVar2;
        }
    }
    return '\0';
}

void __stdcall16far pass1_1010_1bb4(undefined4 *param_1, uint param_2, undefined2 param_3)

{
    code     **ppcVar1;
    undefined *puVar2;
    uint       extraout_DX;
    undefined  local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    while(true)
    {
        puVar2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_3, puVar2));
        if((param_2 | (uint)puVar2) == 0x0)
            break;
        ppcVar1 = (code **)((int)*param_1 + 0x40);
        (**ppcVar1)((int)&USHORT_1050_1028, param_1);
        param_2 = extraout_DX;
    }
    return;
}


void __stdcall16far pass1_1010_1c16(ulong param_1, ulong param_2, int param_3)

{
    char      *pcVar1;
    ushort     uVar2;
    undefined4 uVar3;

    uVar3  = pass1_1028_b58e(param_2);
    uVar2  = (ushort)((ulong)uVar3 >> 0x10);
    pcVar1 = pass1_1010_b038(*(uchar **)((int)param_1 + 0x6e), (ushort)uVar3, uVar2, (uchar *)0x178a, param_3);
    str_op_1008_60e8((char *)CONCAT22(uVar2, pcVar1), uVar2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar __stdcall16far pass1_1010_1c40(ulong param_1, ulong param_2)

{
    undefined4 uVar1;
    uchar      uVar2;
    ushort     uVar3;
    BOOL16     BVar4;
    uint       uVar5;
    undefined2 uVar6;
    ulong      uVar7;

    uVar5 = (uint)(param_2 >> 0x10);
    if((*(int *)((int)param_2 + 0x1c) != 0x2) || ((*(uint *)((int)param_2 + 0x1e) & 0xff) != 0x0))
    {
        uVar7 = pass1_1028_b58e(param_2 & 0xffff | (ulong)uVar5 << 0x10);
        uVar6 = (undefined2)(param_1 >> 0x10);
        uVar1 = *(undefined4 *)((int)param_1 + 0x6e);
        pass1_1010_c2d8((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), uVar7);
        if(((int)uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0))
        {
            uVar1 = *(undefined4 *)((int)param_1 + 0x6e);
            uVar3 = pass1_1010_b028((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2);
            BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar3, 0x11);
            if((BVar4 == 0x0) && (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar3, 0x12), BVar4 == 0x0))
            {
                uVar2 = '\0';
            }
            else
            {
                uVar2 = '\x01';
            }
            return uVar2;
        }
    }
    return '\0';
}

void __stdcall16far pass1_1010_1d80(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_455 *iVar4;
    uint         uVar4;

    uVar4            = (uint)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_455 *)param_1;
    *param_1         = 0x2014;
    iVar4->field_0x2 = 0x1010;
    pass1_1010_1f62(param_2, (ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10, 0x1);
    puVar1 = iVar4->field_0x4;
    uVar2  = iVar4->field_0x6;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *param_1         = 0x389a;
    iVar4->field_0x2 = 0x1008;
    return;
}


ushort __stdcall16far pass1_1010_1dce(void)

{
    return 0x0;
}


ushort __stdcall16far pass1_1010_1dd4(void)

{
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_1dda(ulong param_1)

{
    pass1_1010_209e(_PTR_LOOP_1050_0ed0, *(ushort *)((int)param_1 + 0x8));
    return;
}

void __stdcall16far pass1_1010_1ea6(ulong param_1, long param_2, ushort param_3)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    undefined4  *puVar4;
    undefined   *puVar5;
    uint         extraout_DX;
    astruct_498 *iVar6;
    undefined2   uVar6;
    undefined    local_c[0x4];
    undefined4   uStack8;
    undefined2   uStack4;

    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar6 = (astruct_498 *)param_1;
    if(iVar6->field_0x4 == (undefined4 *)0x0)
    {
        return;
    }
    uStack4 = 0x0;
    pass1_1008_5784((ulong *)CONCAT22(param_3, local_c), (ulong)iVar6->field_0x4);
    while(true)
    {
        puVar5 = local_c;
        pass1_1008_5b12(puVar5, param_3);
        if((extraout_DX | (uint)puVar5) == 0x0)
            break;
        if(*(long *)(puVar5 + 0x4) == param_2)
        {
            uStack4 = 0x1;
            ppcVar3 = (code **)((int)*iVar6->field_0x4 + 0xc);
            (**ppcVar3)(0x1008);
            uStack8 = 0x0;
        }
    }
    puVar4 = iVar6->field_0x4;
    if(*(int *)((int)puVar4 + 0x8) == 0x0)
    {
        // WARNING: Load size is inaccurate
        puVar1 = iVar6->field_0x4;
        uVar2  = *(uint *)((int)&iVar6->field_0x4 + 0x2);
        if((uVar2 | (uint)puVar1) != 0x0)
        {
            ppcVar3 = (code **)*puVar1;
            (**ppcVar3)(0x1008, puVar1, uVar2, 0x1, puVar1, uVar2, puVar1, uVar2);
        }
        iVar6->field_0x4 = (undefined4 *)0x0;
    }
    return;
}


void __stdcall16far pass1_1010_1f62(ushort param_1, ulong param_2, int param_3)

{
    undefined4 uVar1;
    code     **ppcVar2;
    int        iVar3;
    undefined2 uVar4;
    long       lVar5;
    undefined  local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_1, local_a), *(ulong *)((int)param_2 + 0x4));
    while(true)
    {
        lVar5 = pass1_1008_5b12(local_a, param_1);
        uVar4 = (undefined2)((ulong)lVar5 >> 0x10);
        iVar3 = (int)lVar5;
        if(lVar5 == 0x0)
            break;
        if(((*(int *)(iVar3 + 0x8) == 0x0) || (param_3 == 0x0)) || (*(int *)(iVar3 + 0x8) == param_3))
        {
            uVar1   = *(undefined4 *)(iVar3 + 0x4);
            ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0x4);
            (**ppcVar2)(0x8, (int)uVar1, (int)((ulong)uVar1 >> 0x10), param_3);
        }
    }
    return;
}

ulong __stdcall16far pass1_1010_2024(ulong param_1)

{
    uint uVar1;

    uVar1                                 = (uint)(param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x124) = 0x0;
    _PTR_LOOP_1050_0ed0                   = param_1;
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff | (ulong)uVar1 << 0x10), (WNDCLASS16 *)0x0, 0x124);
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_2050(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    undefined2  uVar4;
    int         iStack4;

    pass1_1010_2816(param_1);
    iStack4 = 0x0;
    do
    {
        uVar4  = (undefined2)(param_1 >> 0x10);
        puVar1 = (undefined4 *)*(uint *)(iStack4 * 0x4 + (int)param_1);
        uVar2  = *(uint *)(iStack4 * 0x4 + (int)param_1 + 0x2);
        if((uVar2 | (uint)puVar1) != 0x0)
        {
            ppcVar3 = (code **)*puVar1;
            (**ppcVar3)();
        }
        iStack4 = iStack4 + 0x1;
    } while(iStack4 < 0x49);
    _PTR_LOOP_1050_0ed0 = 0x0;
    return;
}


void __stdcall16far pass1_1010_209e(ulong param_1, ushort param_2)

{
    pass1_1010_2816(param_1);
    *(ushort *)((int)param_1 + 0x124) = param_2;
    return;
}

void __stdcall16far pass1_1010_038e(ulong param_1, int param_2, ushort param_3)

{
    bool         bVar1;
    astruct_707 *iVar2;
    undefined2   uVar2;

    bVar1 = false;
    iVar2 = (astruct_707 *)param_1;
    uVar2 = (undefined2)(param_1 >> 0x10);
    if(param_2 != 0x0)
    {
        if(iVar2->field_0x18 == 0x0)
        {
            iVar2->field_0x12  = DAT_1050_0e28;
            iVar2->field_0x14  = PTR_LOOP_1050_0e30;
            iVar2->field_0x16  = PTR_LOOP_1050_0ea8;
            DAT_1050_0e28      = 0x0;
            PTR_LOOP_1050_0e30 = (undefined *)0x0;
            PTR_LOOP_1050_0ea8 = (undefined *)0x0;
            iVar2->field_0x18  = 0x1;
            bVar1              = true;
            goto LAB_1010_0404;
        }
    }
    if(param_2 == 0x0)
    {
        if(iVar2->field_0x18 != 0x0)
        {
            DAT_1050_0e28      = iVar2->field_0x12;
            PTR_LOOP_1050_0e30 = iVar2->field_0x14;
            PTR_LOOP_1050_0ea8 = iVar2->field_0x16;
            iVar2->field_0x18  = 0x0;
            bVar1              = true;
        }
    }
LAB_1010_0404:
    if(bVar1)
    {
        pass1_1010_1f62(param_3, param_1, 0x3);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far pass1_1010_041a(void)

{
    ulong uVar1;

    uVar1 = pass1_1030_8326();
    if(((int)(uVar1 >> 0x10) == 0x0) && ((uint)uVar1 < 0x64))
    {
        return 0x0;
    }
    return 0x1;
}

ushort __stdcall16far pass1_1010_0886(void)

{
    return 0xa;
}


ushort __stdcall16far pass1_1010_088c(void)

{
    return 0x3;
}


ushort __stdcall16far pass1_1010_0892(void)

{
    return 0x3;
}


ushort __stdcall16far pass1_1010_0898(void)

{
    return 0x3;
}


void __stdcall16far pass1_1010_089e(ushort param_1, ulong param_2, ushort param_3, int param_4)

{
    *(ushort *)((param_4 + -0x1) * 0x8 + 0xe28) = param_3;
    pass1_1010_1f62(param_1, param_2, 0x3);
    return;
}


void __stdcall16far pass1_1010_08c0(ulong param_1, ushort param_2, int param_3, ushort param_4)

{
    *(ushort *)((param_3 + -0x1) * 0x8 + 0xea8) = param_2;
    pass1_1010_1f62(param_4, param_1, 0x3);
    return;
}


ulong __stdcall16far pass1_1010_08e2(ushort param_1, ushort param_2, int param_3)

{
    if(PTR_LOOP_1050_4fe8 != (undefined *)0x0)
    {
        DAT_1050_0e28      = 0x0;
        PTR_LOOP_1050_0e30 = (undefined *)0x0;
        PTR_LOOP_1050_0e38 = (undefined *)0x0;
        PTR_LOOP_1050_0e40 = (undefined *)0x0;
        PTR_LOOP_1050_0e48 = (undefined *)0x0;
        DAT_1050_0e58      = 0x0;
        DAT_1050_0e60      = 0x0;
        PTR_LOOP_1050_0e70 = (undefined *)0x0;
    }
    return CONCAT22(0x1050, (param_3 + -0x1) * 0x8 + 0xe22);
}


ulong __stdcall16far pass1_1010_091e(ushort param_1, ushort param_2, int param_3)

{
    return CONCAT22(0x1050, (param_3 + -0x1) * 0x8 + 0xe72);
}


ulong __stdcall16far pass1_1010_0932(ushort param_1, ushort param_2, int param_3)

{
    return CONCAT22(0x1050, (param_3 + -0x1) * 0x8 + 0xe8a);
}
