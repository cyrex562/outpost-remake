
void __stdcall16far pass1_1028_b58e(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    return;
}


ushort __stdcall16far pass1_1028_b5a8(ulong param_1)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) != 0x5)
    {
        return 0x0;
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x14);
    return *(ushort *)((int)uVar1 + 0x94);
}


ushort __stdcall16far pass1_1028_b5ca(ulong param_1)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) != 0x5)
    {
        return 0x0;
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x14);
    return *(ushort *)((int)uVar1 + 0x9c);
}


void __stdcall16far pass1_1028_bab6(ulong param_1, int param_2, ushort param_3)

{
    ulong uVar1;

    uVar1 = pass1_1028_bad4(param_1, param_2, param_3);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)(uVar1 >> 0x10));
    return;
}


ulong __stdcall16far pass1_1028_bad4(ulong param_1, int param_2, ushort param_3)

{
    pass1_1028_baf6(param_1);
    return CONCAT22(*(undefined2 *)(param_2 + 0xa), *(undefined2 *)(param_2 + 0x8));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_baf6(ulong param_1)

{
    ulong uVar1;

    uVar1 = pass1_1028_bb24(param_1);
    if(uVar1 == 0x0)
    {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)(uVar1 >> 0x10));
    return;
}


ulong __stdcall16far pass1_1028_bb24(ulong param_1)

{
    undefined2 uVar1;
    uint       uVar2;
    undefined4 uVar3;

    uVar2 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x8) == 0x0)
    {
        return 0x0;
    }
    uVar3 = pass1_1028_b58e(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    uVar1 = (undefined2)((ulong)uVar3 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)uVar3 + 0xa), *(undefined2 *)((int)uVar3 + 0x8));
}


void __stdcall16far pass1_1028_bb56(ulong param_1, ulong param_2)

{
    pass1_1030_177a(param_1, param_2);
    return;
}


ulong __stdcall16far pass1_1028_bb6a(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if((*(int *)(iVar1 + 0x12) != 0x5) && (*(int *)(iVar1 + 0x12) != 0x6))
    {
        return 0x0;
    }
    return CONCAT22(*(undefined2 *)(iVar1 + 0x16), *(int *)(iVar1 + 0x14) + 0xa4);
}


void __stdcall16far pass1_1028_bb96(ulong param_1, ulong *param_2, ushort param_3)

{
    ulong       *puVar1;
    ulong       *puVar2;
    undefined4   uVar3;
    int          iVar6;
    astruct_296 *iVar5;
    astruct_297 *iVar4;
    ulong       *puVar7;
    undefined2   uVar8;
    undefined2   uVar9;

    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_296 *)param_1;
    if((iVar5->field_0x12 == 0x5) || (iVar5->field_0x12 == 0x6))
    {
        uVar3  = iVar5->field_0x14;
        uVar9  = (undefined2)((ulong)uVar3 >> 0x10);
        puVar7 = (ulong *)((int)uVar3 + 0xa4);
        for(iVar6 = 0x2; iVar6 != 0x0; iVar6 = iVar6 + -0x1)
        {
            puVar2  = puVar7;
            puVar7  = puVar7 + 0x1;
            puVar1  = param_2;
            param_2 = param_2 + 0x1;
            *puVar2 = *puVar1;
        }
        *(undefined2 *)puVar7 = *(undefined2 *)param_2;
        pass1_1028_c724(param_1);
        uVar3 = iVar5->field_0x14;
        uVar8 = (undefined2)((ulong)uVar3 >> 0x10);
        iVar4 = (astruct_297 *)uVar3;
        if(iVar4->field_0xaa == 0x0)
        {
            iVar4->field_0xaa = 0x1;
        }
    }
    return;
}


void __stdcall16far pass1_1028_bbf0(ushort param_1, ushort param_2, ulong *param_3)

{
    *param_3 = 0x0;
    return;
}


ushort __stdcall16far pass1_1028_bc1c(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0x12) == 0x4)
    {
        return *(ushort *)(iVar1 + 0xe);
    }
    if(*(int *)(iVar1 + 0x12) == 0x7)
    {
        return *(ushort *)(iVar1 + 0x10);
    }
    return *(ushort *)(iVar1 + 0xc);
}


void __stdcall16far pass1_1028_bc7e(ulong *param_1, ushort param_2)

{
    pass1_1028_bdac(param_1, 0x4, param_2);
    return;
}


ushort __stdcall16far pass1_1028_bc90(
  ulong *param_1, ushort *param_2, ulong param_3, ulong param_4, int param_5, ushort param_6, ushort param_7)

{
    code     **ppcVar1;
    ulong      uVar2;
    int        iVar3;
    BOOL16     BVar4;
    undefined4 uVar5;
    ushort     uVar6;
    ushort     uVar7;

    uVar6 = (ushort)param_1;
    uVar7 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1028_c7b6(param_7, param_6, uVar6, uVar7, param_2, param_4);
    if((param_5 == 0x5) || (param_5 == 0x6))
    {
        uVar2   = *param_1;
        ppcVar1 = (code **)((int)uVar2 + 0x60);
        iVar3   = (**ppcVar1)();
        if(iVar3 != 0x0)
        {
            ppcVar1 = (code **)((int)uVar2 + 0x5c);
            uVar5   = (**ppcVar1)();
            if((uint)uVar5 != 0x0)
            {
                pass1_1028_c23e(
                  uVar6, uVar7, param_2, param_3, param_4, (uint)uVar5, (uint)((ulong)uVar5 >> 0x10), param_7);
                if((int)uVar5 != 0x0)
                {
                    BVar4 = pass1_1028_c314(param_7,
                                            (int)uVar5,
                                            (ushort)((ulong)uVar5 >> 0x10),
                                            uVar6,
                                            uVar7,
                                            param_2,
                                            (ushort)param_3,
                                            (ushort)(param_3 >> 0x10),
                                            param_4);
                    if(BVar4 != 0x0)
                    {
                        return 0x1;
                    }
                }
            }
        }
    }
    else
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6a8;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_bd38(ulong param_1, ushort param_2, ushort param_3)

{
    uint       uVar1;
    uint       uVar2;
    int        iVar3;
    int        iVar4;
    ulong      uVar5;
    undefined2 extraout_DX;
    int        iStack20;

    uVar5 = *(ulong *)((int)_PTR_LOOP_1050_65e2 + 0x52);
    pass1_1030_4bbe(param_3, param_2, uVar5, *(int *)((int)param_1 + 0xc));
    iVar3 = (int)uVar5;
    iVar4 = iVar3;
    pass1_1028_b58e(param_1);
    uVar5    = *(ulong *)(iVar4 + 0x2e);
    iStack20 = 0x11;
    do
    {
        uVar1 = *(uint *)(iStack20 * 0x4 + iVar3);
        uVar2 = *(uint *)(iStack20 * 0x4 + iVar3 + 0x2);
        if((uVar2 | uVar1) != 0x0)
        {
            pass1_1038_5770(uVar5, CONCAT22(uVar2, uVar1), iStack20);
        }
        iStack20 = iStack20 + 0x1;
    } while(iStack20 < 0x25);
    return;
}


void __stdcall16far pass1_1028_be2a(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined2 uVar2;
    ulong      uVar3;
    int        iVar4;

    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) != 0x6)
    {
        return;
    }
    uVar3 = pass1_1028_b4f2((ulong)param_1);
    if(*(long *)((int)uVar3 + 0x200) != 0x8000002)
    {
        if(*(long *)((int)param_1 + 0x1c) == 0x8000002)
        {
            iVar4 = 0x6;
            goto code_r0x1028be96;
        }
        ppcVar1 = (code **)((int)*param_1 + 0x64);
        iVar4   = (**ppcVar1)(param_4, param_1);
        if(iVar4 == 0x0)
        {
            return;
        }
        pass1_1028_cb04((ulong)param_1, param_2, param_3, param_5);
        if(iVar4 == 0x0)
        {
            iVar4 = 0x6;
            goto code_r0x1028be96;
        }
        pass1_1028_c952((ulong)param_1, param_2, param_3, param_5);
    }
    iVar4 = 0x5;
code_r0x1028be96:
    pass1_1028_bdac(param_1, iVar4, param_4);
    return;
}


void __stdcall16far pass1_1028_be9e(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    int        iVar4;
    undefined2 uVar5;
    ulong      uVar6;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(int *)(iVar4 + 0x12) == 0x4)
    {
        uVar6 = pass1_1028_b4f2((ulong)param_1);
        iVar3 = (int)uVar6;
        if(*(long *)(iVar3 + 0x200) == 0x8000002)
        {
            uVar2   = *(undefined4 *)(iVar4 + 0x14);
            piVar1  = (int *)((int)uVar2 + 0x94);
            *piVar1 = *piVar1 + -0x1;
        }
        else
        {
            pass1_1028_cb04((ulong)param_1, param_2, param_3, param_5);
            if(iVar3 == 0x0)
            {
                return;
            }
            uVar2   = *(undefined4 *)(iVar4 + 0x14);
            piVar1  = (int *)((int)uVar2 + 0x94);
            *piVar1 = *piVar1 + -0x1;
            pass1_1028_c952((ulong)param_1, param_2, param_3, param_5);
        }
        uVar2 = *(undefined4 *)(iVar4 + 0x14);
        if(*(int *)((int)uVar2 + 0x94) < 0x1)
        {
            pass1_1028_bdac(param_1, 0x5, param_4);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_bf22(ulong param_1, uchar *param_2, ushort param_3)

{
    int        iVar1;
    int        iVar2;
    undefined2 uVar3;
    ulong      uVar4;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    iVar1 = *(int *)(iVar2 + 0x12);
    if(iVar1 == 0x4)
    {
        uVar4 = pass1_1028_e0bc(_PTR_LOOP_1050_65e2, *(int *)(iVar2 + 0xc), (ulong *)0x0, param_2, param_3);
    }
    else
    {
        iVar1 = iVar1 + -0x5;
        if(iVar1 != 0x0)
        {
            if(iVar1 != 0x1)
            {
                *(undefined4 *)(iVar2 + 0x14) = 0x0;
            }
            return;
        }
        pass1_1028_e100(_PTR_LOOP_1050_65e2, *(ushort *)(iVar2 + 0xc), param_2);
        uVar4 = CONCAT22(param_2, iVar1);
    }
    *(undefined2 *)(iVar2 + 0x14) = (int)uVar4;
    *(undefined2 *)(iVar2 + 0x16) = (int)(uVar4 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_bf76(ulong param_1, uint param_2)

{
    BOOL16       BVar1;
    astruct_174 *iVar2;
    undefined2   uVar2;

    pass1_1008_612e(0x1, 0x3, param_2);
    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar2 = (astruct_174 *)param_1;
    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar2->field_0xc, 0x28);
    if(BVar1 == 0x0)
    {
        if(param_2 == 0x1)
        {
            iVar2->field_0x10 = 0x48;
            return;
        }
        if(param_2 != 0x2)
        {
            iVar2->field_0x10 = 0x4a;
            return;
        }
        iVar2->field_0x10 = 0x49;
        return;
    }
    if(param_2 == 0x1)
    {
        iVar2->field_0x10 = 0x70;
        return;
    }
    if(param_2 != 0x2)
    {
        iVar2->field_0x10 = 0x72;
        return;
    }
    iVar2->field_0x10 = 0x71;
    return;
}


void __stdcall16far
pass1_1028_c1f8(ushort param_1, int param_2, ushort param_3, ulong param_4, ushort *param_5, ushort *param_6)

{
    undefined4 *puVar1;
    undefined4  local_c;
    undefined2  uStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_baf6(param_4);
    iStack6 = param_2;
    uStack4 = param_3;
    puVar1  = (undefined4 *)pass1_1030_5b5c(param_2, param_3);
    local_c = *puVar1;
    uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
    pass1_1008_3e94((ushort *)CONCAT22(param_1, &local_c), param_5, param_6);
    return;
}


void __stdcall16far pass1_1028_a4ee(ulong param_1, ulong param_2, int param_3, ushort param_4)

{
    ulong       uVar1;
    code      **ppcVar2;
    uint        uVar3;
    BOOL16      BVar4;
    uint        uVar5;
    ulong       uVar6;
    uchar      *puVar7;
    uchar      *puVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    ulong      *puVar11;
    ushort      uVar12;
    int         iStack50;
    undefined4 *puStack18;

    uVar9   = (undefined2)(param_2 >> 0x10);
    uVar1   = *(ulong *)((int)param_2 + 0x1f6);
    uVar6   = *_PTR_LOOP_1050_65e2;
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x26);
    puVar7  = (uchar *)((ulong)puVar11 >> 0x10);
    uVar5   = (uint)puVar11;
    uVar10  = SUB42(&PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(param_2, puVar11, uVar5, puVar7);
    puStack18 = (undefined4 *)CONCAT22(puVar7, uVar5);
    ppcVar2   = (code **)((int)*puStack18 + 0x10);
    uVar3     = uVar5;
    puVar8    = puVar7;
    (**ppcVar2)((int)&PTR_LOOP_1050_1038, uVar5, puVar7);
    if(((uint)puVar8 | uVar3) != 0x0)
    {
        uVar10 = 0x1030;
        pass1_1030_3548(uVar1, CONCAT22(puVar8, uVar3));
    }
    if(puStack18 != (undefined4 *)0x0)
    {
        ppcVar2 = (code **)*puStack18;
        (**ppcVar2)(uVar10, uVar5, (char)puVar7, 0x1);
    }
    uVar3  = (uint)(uVar6 % 0xc);
    uVar12 = (ushort)(param_1 >> 0x10);
    uVar5  = uVar3;
    if(uVar6 % 0xc == 0x0)
    {
        pass1_1030_387c(uVar1);
        pass1_1028_a61e((ushort)param_1, uVar12, uVar1, param_2, uVar5, uVar3, param_3, param_4);
    }
    pass1_1038_3fb0(param_2);
    if((*(int *)((int)param_2 + 0x204) != 0x0)
       && (BVar4 = pass1_1030_25b2(CONCAT13((char)(uVar3 >> 0x8), CONCAT12((char)uVar3, uVar5)), 0x80), BVar4 != 0x0))
    {
        return;
    }
    uVar9    = (undefined2)(uVar1 >> 0x10);
    uVar5    = (int)uVar1 + 0x180;
    uVar6    = (ulong)uVar5;
    iStack50 = 0x1;
    do
    {
        if(*(int *)(iStack50 * 0x2 + uVar5) != 0x0)
        {
            pass1_1008_612e(0x1, 0x64, (uint)uVar6);
            if((int)uVar6 <= *(int *)(iStack50 * 0x2 + uVar5))
            {
                pass1_1028_a188(
                  (ushort)param_1, uVar12, *(int *)(iStack50 * 0x2 + (int)uVar1 + 0x174), iStack50, param_2, param_4);
            }
        }
        iStack50 = iStack50 + 0x1;
    } while(iStack50 < 0x6);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_a61e(
  ushort param_1, ushort param_2, ulong param_3, ulong param_4, uint param_5, int param_6, int param_7, ushort param_8)

{
    uint       uVar1;
    ulong      uVar2;
    int        iVar3;
    uchar     *puVar4;
    undefined2 uVar5;
    ushort    *puVar6;
    uint       uStack16;
    undefined4 uStack14;

    pass1_1030_38b8();
    if((param_6 < 0x3fff) || ((param_6 < 0x4000 && (param_5 != 0xffff))))
    {
        pass1_1030_38f2(param_3, 0x3, param_8);
        uVar1 = param_5;
        iVar3 = param_6;
        pass1_1030_38f2(param_3, 0x4, param_8);
        uStack14 = CONCAT22(param_6 + iVar3 + (uint)CARRY2(param_5, uVar1), param_5 + uVar1);
        uStack16 = *(uint *)((int)param_3 + 0x1a8);
        if(uStack16 == 0x0)
        {
            uStack16 = 0x5;
        }
        uVar2          = uStack14 / (long)(ulong)uStack16;
        uStack14._2_2_ = (uint)(uVar2 >> 0x10);
        puVar4         = (uchar *)(uStack14._2_2_ | (uint)uVar2);
        if((puVar4 != (uchar *)0x0)
           && (uVar5 = (undefined2)(param_4 >> 0x10), *(long *)((int)param_4 + 0x200) != 0x8000002))
        {
            puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_8, puVar4, param_7);
            pass1_1010_043a((ulong)puVar6, *(long *)((int)param_4 + 0x4), 0xc, param_8);
            pass1_1030_3534(param_3, uVar2);
        }
    }
    return;
}


ushort __stdcall16far pass1_1028_a73c(uint param_1, ushort param_2)

{
    undefined *puVar1;
    undefined *puVar2;
    uint       uVar3;
    undefined  local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    while(true)
    {
        puVar1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar1));
        uVar3 = param_1 | (uint)puVar1;
        if(uVar3 == 0x0)
            break;
        puVar2 = puVar1;
        pass1_1038_5464(CONCAT22(param_1, puVar1), (ushort)puVar1, (ushort)&PTR_LOOP_1050_1038, param_2);
        pass1_1038_56d6(CONCAT22(param_1, puVar1), 0x0);
        pass1_1038_518c(CONCAT22(param_1, puVar1), (ushort)puVar2, (ushort)&PTR_LOOP_1050_1038);
        param_1 = uVar3;
    }
    return 0x1;
}


ushort __stdcall16far pass1_1028_a89c(uint param_1, ushort param_2)

{
    undefined *puVar1;
    uint       uVar2;
    undefined  local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    while(true)
    {
        uVar2  = param_1;
        puVar1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar1));
        param_1 = uVar2 | (uint)puVar1;
        if(param_1 == 0x0)
            break;
        if(*(long *)(puVar1 + 0x200) != 0x8000002)
        {
            pass1_1038_3fca(CONCAT22(uVar2, puVar1), (uint)puVar1, param_2);
        }
    }
    return 0x1;
}


ushort __stdcall16far pass1_1028_a9f4(uint param_1, ushort param_2)

{
    code      **ppcVar1;
    undefined  *puVar2;
    BOOL16      BVar3;
    uint        uVar4;
    uint        extraout_DX;
    undefined4 *puStack24;
    undefined   local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, local_14), 0x1, 0x0, 0x700);
    while(true)
    {
        puVar2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar2));
        puStack24 = (undefined4 *)CONCAT22(param_1, puVar2);
        uVar4     = param_1 | (uint)puVar2;
        if(uVar4 == 0x0)
            break;
        BVar3   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(puVar2 + 0xc), 0xc);
        param_1 = uVar4;
        if(BVar3 != 0x0)
        {
            ppcVar1 = (code **)((int)*puStack24 + 0x34);
            (**ppcVar1)(0x1008, puVar2);
            param_1 = extraout_DX;
        }
    }
    return 0x1;
}


ushort __stdcall16far pass1_1028_ab68(uint param_1, ushort param_2)

{
    undefined2  uVar1;
    code      **ppcVar2;
    undefined  *puVar3;
    BOOL16      BVar4;
    uint        uVar5;
    uint        extraout_DX;
    undefined4 *puStack24;
    undefined   local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_2 >> 0x8), CONCAT12((char)param_2, local_14)), 0x1, 0x0, 0x700);
LAB_1028_ab7e:
    uVar5  = param_1;
    puVar3 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2, puVar3));
    puStack24 = (undefined4 *)CONCAT22(uVar5, puVar3);
    param_1   = uVar5 | (uint)puVar3;
    if(param_1 == 0x0)
    {
        return 0x1;
    }
    uVar1 = *(undefined2 *)(puVar3 + 0xc);
    BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x11);
    if(BVar4 == 0x0)
        goto code_r0x1028abad;
    goto LAB_1028_abc0;
code_r0x1028abad:
    BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x12);
    if(BVar4 != 0x0)
    {
    LAB_1028_abc0:
        if(*(int *)(puVar3 + 0x12) == 0x5)
        {
            ppcVar2 = (code **)((int)*puStack24 + 0x30);
            (**ppcVar2)(0x1008);
            param_1 = extraout_DX;
        }
    }
    goto LAB_1028_ab7e;
}


ushort __stdcall16far pass1_1028_acec(uint param_1, ushort param_2)

{
    undefined2 *puVar1;
    undefined2 *puVar2;
    uint        uVar3;
    undefined2  local_14;
    undefined2  uStack18;

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, &local_14), 0x1, 0x0, 0x400);
    while(true)
    {
        uVar3  = param_1;
        puVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar1));
        param_1 = uVar3 | (uint)puVar1;
        if(param_1 == 0x0)
            break;
        puVar2 = puVar1;
        vsprintf_op_1030_840a((ulong)s_SCSetup__calcMe_clearing_colony_0_1050_512c, 0x1030, param_2, param_1);
        if(*(long *)(puVar1 + 0x100) != 0x8000002)
        {
            pass1_1038_5464(CONCAT22(uVar3, puVar1), (ushort)puVar2, (ushort)&PTR_LOOP_1050_1038, param_2);
            pass1_1038_56d6(CONCAT22(uVar3, puVar1), 0x1);
        }
    }
    local_14 = 0x389a;
    uStack18 = 0x1008;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, &local_14), 0x1, 0x0, 0x800);
    while(true)
    {
        puVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar1));
        uVar3 = param_1 | (uint)puVar1;
        if(uVar3 == 0x0)
            break;
        pass1_1030_2690(CONCAT22(param_1, puVar1));
        param_1 = uVar3;
    }
    return 0x1;
}


void __stdcall16far pass1_1028_aec0(ulong param_1, int param_2, ushort param_3, ushort param_4)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    pass1_1030_375a(*(ulong *)(param_2 + 0x1f6), 0x0, (long)*(int *)((int)param_1 + 0x114), param_4);
    return;
}


astruct_100 *__stdcall16far
pass1_1028_b0de(astruct_100 *param_1, ulong param_2, ulong param_3, ushort param_4, uchar param_5)

{
    pass1_1028_6af2(param_1, param_2, param_3, param_4, param_5);
    param_1->field_0x0                  = 0xb1f4;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_b22c(ushort *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    undefined2 uVar1;

    pass1_1030_165e(param_1, 0x6000000, param_3, param_4);
    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0xc)     = param_2;
    *param_1                            = 0xb33c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


void __stdcall16far pass1_1028_b260(ushort *param_1)

{
    *param_1                            = 0xb33c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    pass1_1030_16b2(param_1);
    return;
}


BOOL16 __stdcall16far write_to_file_1028_b286(int param_1, ushort param_2)

{
    undefined4 uVar1;
    BOOL16     in_AX;
    BOOL16     BVar2;

    pass1_1030_16d6(*(ulong *)(param_1 + 0x6), *(ulong *)(param_1 + 0xa), param_2);
    if(in_AX != 0x0)
    {
        uVar1                           = *(undefined4 *)(param_1 + 0x6);
        *(undefined2 *)(param_1 + -0xa) = *(undefined2 *)((int)uVar1 + 0xc);
        uVar1                           = *(undefined4 *)(param_1 + 0xa);
        BVar2                           = write_to_file_1008_7e1c(
          (ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_1 - 0xa, param_2, (char *)0x2, 0x1008);
        if(BVar2 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return BVar2;
        }
        in_AX = 0x1;
    }
    return in_AX;
}


void __stdcall16far pass1_1028_9600(ulong param_1, uchar *param_2, int param_3, ushort param_4, uchar param_5)

{
    ushort   *puVar1;
    undefined local_6[0x4];

    puVar1 = pass1_1020_a43e(param_4, param_2, (ushort *)CONCAT22(param_4, local_6));
    pass1_1020_a80e((ushort)local_6,
                    param_4,
                    *(int *)((int)param_1 + 0x11a),
                    (uint)local_6,
                    (uint)((ulong)puVar1 >> 0x10),
                    param_4,
                    param_5,
                    param_3);
    return;
}


void __stdcall16far
pass1_1028_9624(ulong param_1, ushort param_2, uchar *param_3, ushort param_4, int param_5, uchar param_6)

{
    code       **ppcVar1;
    ulong       *puVar2;
    undefined2   uVar3;
    uint         uVar4;
    BOOL16       BVar5;
    ulong        uVar7;
    uchar       *extraout_DX;
    uint         extraout_DX_00;
    astruct_688 *iVar9;
    undefined2   uVar8;
    undefined2   uVar9;
    uchar       *puVar10;
    undefined2   uStack332;
    undefined2   uStack330;
    undefined2   uStack64;
    undefined4   uStack62;
    int          iStack58;
    undefined4   uStack56;
    undefined4  *puStack46;
    ulong        uStack42;
    undefined    local_26[0x4];
    undefined2   uStack34;
    uchar       *puStack32;
    ulong        uStack30;
    undefined4   uStack26;
    undefined4  *puStack22;
    undefined    local_12[0x2];
    undefined    local_10[0x2];
    undefined    local_e[0x2];
    uint         uStack12;
    undefined4   uStack10;
    ushort      *puStack6;
    undefined4  *puVar6;

    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar9 = (astruct_688 *)param_1;
    uVar7 = iVar9->field_0x10c;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar7, (uint)(uVar7 >> 0x10));
    *(ushort *)&iVar9->field_0x110              = param_2;
    *(uchar **)((int)&iVar9->field_0x110 + 0x2) = param_3;
    puStack6                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, param_3, param_5);
    uStack10._2_2_                              = (uchar *)((ulong)puStack6 >> 0x10);
    puVar2                                      = &iVar9->field_0x114;
    pass1_1030_64ce(param_4,
                    puVar2,
                    uStack10._2_2_,
                    _PTR_LOOP_1050_5740,
                    (ushort *)(param_1 & 0xffff0000 | ZEXT24(puVar2)),
                    iVar9->field_0x108,
                    (ulong *)CONCAT22(param_4, local_26));
    uStack56       = (undefined4 *)*puVar2;
    uStack56._3_1_ = (char)((ulong)uStack56 >> 0x18);
    uStack12       = (uint)(uStack56._3_1_ != '\0');
    uVar9          = 0x1008;
    puStack46      = uStack56;
    uStack10       = uStack56;
    pass1_1008_3eb4((ushort *)(param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x114)),
                    (ushort *)CONCAT22(param_4, local_12),
                    (ushort *)CONCAT22(param_4, local_10),
                    (ushort *)CONCAT22(param_4, local_e));
    if(uStack12 == 0x0)
    {
        puVar2 = &iVar9->field_0x114;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x500);
        puStack22 = (undefined4 *)CONCAT22(uStack10._2_2_, puVar2);
        uVar9     = 0x1030;
        pass1_1030_61fe(_PTR_LOOP_1050_5740,
                        CONCAT22(uStack10._2_2_, puVar2),
                        param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x114),
                        iVar9->field_0x108,
                        (ushort)puVar2,
                        (ushort)uStack10._2_2_,
                        param_4);
        if((iVar9->field_0x11a == 0xa) || (iVar9->field_0x11a == 0x37))
        {
            if(iVar9->field_0x11a == 0x37)
            {
                uStack56                         = iVar9->field_0x11e;
                uStack10._2_2_                   = *(uchar **)((int)&iVar9->field_0x11e + 0x2);
                uStack42                         = iVar9->field_0x10c;
                *(ulong *)((int)uStack56 + 0x20) = uStack42;
            }
            puVar2 = &iVar9->field_0x114;
            pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x400);
            *(ulong **)&iVar9->field_0x10c              = puVar2;
            *(uchar **)((int)&iVar9->field_0x10c + 0x2) = uStack10._2_2_;
            uVar9                                       = 0x1018;
            pass1_1018_0196((ulong)puStack6,
                            CONCAT22(uStack10._2_2_, *(undefined2 *)&iVar9->field_0x10c),
                            iVar9->field_0x108,
                            (ushort)puVar2,
                            uStack10._2_2_,
                            param_4);
            if(iVar9->field_0x11a == 0xa)
            {
                uVar9 = 0x1010;
                pass1_1010_ed22((ulong)puStack6, iVar9->field_0x10c, param_4);
            }
        }
        uVar7 = iVar9->field_0x10c;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar7, (uint)(uVar7 >> 0x10));
        *(ulong **)&iVar9->field_0x110              = puVar2;
        *(uchar **)((int)&iVar9->field_0x110 + 0x2) = uStack10._2_2_;
        uVar4                                       = (uint)uStack10._2_2_ | *(uint *)&iVar9->field_0x110;
        puVar6                                      = (undefined4 *)(ulong)uVar4;
        if(uVar4 == 0x0)
            goto LAB_1028_9807;
        uVar3   = SUB42(puStack22, 0x0);
        puVar10 = (uchar *)((ulong)puStack22 >> 0x10);
    }
    else
    {
        puStack22 = uStack10;
        puVar6    = uStack10;
        if(iVar9->field_0x11a != 0x75)
            goto LAB_1028_9807;
        uVar3          = SUB42(uStack10, 0x0);
        puVar10        = uStack10._2_2_;
        uStack10._2_2_ = *(uchar **)((int)&iVar9->field_0x110 + 0x2);
    }
    ppcVar1 = (code **)((int)*iVar9->field_0x110 + 0x8);
    (**ppcVar1)(uVar9, *(undefined2 *)&iVar9->field_0x110, uStack10._2_2_, 0x0, uVar3, puVar10, 0x0);
    uStack10._2_2_ = extraout_DX;
LAB_1028_9807:
    uVar9 = SUB42(puVar6, 0x0);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puStack22, (uint)((ulong)puStack22 >> 0x10));
    uStack26 = CONCAT22(uStack10._2_2_, uVar9);
    pass1_1030_73ee(CONCAT22(uStack10._2_2_, uVar9), iVar9->field_0x10c, (ushort)uStack10._2_2_);
    BVar5     = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar9->field_0x11a, 0x31);
    puStack32 = uStack10._2_2_;
    if((BVar5 == 0x0) && (iVar9->field_0x122 == 0x0))
    {
        uStack62 = *(undefined4 *)((int)uStack26 + 0xc);
        iStack58 = *(int *)((int)uStack26 + 0x10);
        uStack56 = (undefined4 *)((ulong)uStack56 & 0xffff0000 | ZEXT24(&uStack62));
        if(iStack58 < 0x1)
        {
            uStack64 = 0x5;
        }
        else
        {
            uStack64 = 0x6;
        }
        *(undefined2 *)((int)uStack26 + 0x14) = uStack64;
        puStack32                             = uStack26._2_2_;
    }
    uVar7    = *(ulong *)((int)uStack26 + 0x16);
    uStack30 = uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar7, (uint)(uVar7 >> 0x10));
    uStack34 = (undefined2)uVar7;
    if(uStack30 != 0x0)
    {
        struct_1030_e4fa((astruct_100 *)CONCAT22(param_4, &uStack332), uStack30, param_4, param_6);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_4, &uStack332));
        uStack332 = 0x389a;
        uStack330 = 0x1008;
    }
    ppcVar1 = (code **)((int)*iVar9->field_0x11e + 0x4);
    (**ppcVar1)();
    puVar6 = iVar9->field_0x11e;
    pass1_1030_7e5a(uStack26, *(ulong *)((int)puVar6 + 0x4), extraout_DX_00);
    return;
}


void __stdcall16far pass1_1028_99c4(ulong param_1, int param_2, ushort param_3)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    pass1_1030_355c(*(ulong *)(param_2 + 0x1f6), *(ulong *)((int)param_1 + 0x114));
    return;
}


ushort __stdcall16far pass1_1028_9c90(ulong param_1)

{
    uint   uVar1;
    ushort uVar2;

    uVar1 = *(int *)((int)param_1 + 0x108) - 0x3e8;
    if((uVar1 < 0x3a99) && (uVar1 % 0x3e8 == 0x0))
    {
        // WARNING: Could not recover jumptable at 0x10289dc0. Too many branches
        // WARNING: Treating indirect jump as call
        uVar2 = (**(code **)((uVar1 / 0x3e8) * 0x2 + -0x623a))();
        return uVar2;
    }
    return 0x1;
}


void __stdcall16far pass1_1028_a28a(ushort param_1, ushort param_2, ulong param_3)

{
    code       **ppcVar1;
    uint         uVar2;
    undefined2   uVar3;
    ulong        uVar4;
    uchar       *puVar5;
    uchar       *puVar6;
    uchar       *puVar7;
    uint         uVar8;
    astruct_691 *iVar9;
    undefined2   uVar9;
    ulong       *puVar10;
    undefined4  *puStack10;

    puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0xe);
    puVar5  = (uchar *)((ulong)puVar10 >> 0x10);
    uVar2   = (uint)puVar10;
    pass1_1038_4d6e(param_3, puVar10, uVar2, puVar5);
    puStack10 = (undefined4 *)CONCAT22(puVar5, uVar2);
    uVar9     = (undefined2)(param_3 >> 0x10);
    iVar9     = (astruct_691 *)param_3;
    uVar4     = iVar9->field_0x1f6;
    ppcVar1   = (code **)((int)*puStack10 + 0x10);
    puVar6    = puVar5;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar2, puVar5);
    uVar3  = (undefined2)uVar4;
    puVar7 = puVar6;
    pass1_1030_38b8();
    if((uVar4 & 0xffff | ZEXT24(puVar6) << 0x10) == 0x0)
    {
        uVar4 = 0x64;
        uVar8 = 0x0;
    }
    else
    {
        uVar4 = CONCAT22(puVar7, uVar3) / (long)(uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
        uVar8 = (uint)(uVar4 >> 0x10);
    }
    uVar4 = uVar4 & 0xffff | (ulong)uVar8 << 0x10;
    if(puStack10 != (undefined4 *)0x0)
    {
        ppcVar1 = (code **)*puStack10;
        (**ppcVar1)(0x1030, uVar2, (char)puVar5, 0x1);
    }
    if((long)uVar4 < 0x64)
    {
        if((long)uVar4 < 0x55)
        {
            if((long)uVar4 < 0x4b)
            {
                if((long)uVar4 < 0x32)
                {
                    if((long)uVar4 < 0x19)
                    {
                        iVar9->field_0x20a = 0x1;
                        iVar9->field_0x20c = 0xffff;
                        return;
                    }
                    iVar9->field_0x20a = 0x0;
                    iVar9->field_0x20c = 0x0;
                    return;
                }
                iVar9->field_0x20a = 0xfffb;
            }
            else
            {
                iVar9->field_0x20a = 0xfff6;
            }
        }
        else
        {
            iVar9->field_0x20a = 0xfff1;
        }
    }
    else
    {
        iVar9->field_0x20a = 0xffec;
    }
    iVar9->field_0x20c = 0x1;
    return;
}


ushort __stdcall16far pass1_1028_83b4(uint param_1, ushort param_2)

{
    undefined *puVar1;
    undefined  local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    while(true)
    {
        puVar1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar1));
        if((param_1 | (uint)puVar1) == 0x0)
            break;
        *(undefined2 *)(puVar1 + 0x206) = 0x1;
        param_1                         = param_1 | (uint)puVar1;
    }
    return 0x1;
}


ushort __stdcall16far pass1_1028_853e(ulong param_1, ushort param_2, ushort param_3)

{
    undefined4 uVar1;
    ushort     uVar2;
    int        iVar3;
    undefined2 uVar4;

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(int *)(iVar3 + 0x108) == 0x0)
    {
        return 0x0;
    }
    uVar1 = *(undefined4 *)(iVar3 + 0x10e);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    if(*(int *)(iVar3 + 0x108) == 0x1)
    {
        uVar2 = 0x3e8;
    }
    else
    {
        uVar2 = 0x0;
    }
    pass1_1038_4d0e(CONCAT22(param_3, param_2), uVar2);
    return 0x1;
}


astruct_100 *__stdcall16far
pass1_1028_8698(astruct_100 *param_1, ulong param_2, ulong param_3, uchar param_4, ushort param_5)

{
    pass1_1028_6af2(param_1, param_2, param_3, param_5, param_4);
    param_1->field_0x0                  = 0x87e0;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


void __stdcall16far pass1_1028_8e1e(ulong param_1, int param_2, ushort param_3)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    pass1_1030_355c(*(ulong *)(param_2 + 0x1f6), *(ulong *)((int)param_1 + 0x114));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_8e5c(ulong param_1, int param_2, uchar *param_3, ushort param_4)

{
    undefined4 uVar1;
    ulong      uVar2;
    int        iVar3;
    undefined2 uVar4;

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    uVar1 = *(undefined4 *)(iVar3 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    uVar2 = *(ulong *)(param_2 + 0x1f6);
    pass1_1030_35a4(uVar2, *(long *)(iVar3 + 0x110), param_3, 0x1030, param_4);
    *(undefined2 *)(iVar3 + 0x114) = (int)uVar2;
    *(uchar **)(iVar3 + 0x116)     = param_3;
    return;
}


astruct_100 *__stdcall16far
pass1_1028_8fc0(astruct_100 *param_1, ulong param_2, ulong param_3, ushort param_4, uchar param_5)

{
    pass1_1028_6af2(param_1, param_2, param_3, param_4, param_5);
    param_1->field_0x0                  = 0x90d6;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}

ushort __stdcall16far pass1_1028_74e4(ulong param_1, long param_2, int param_3, ushort param_4, uchar param_5)

{
    int iVar1;

    pass1_1028_7fb6(param_1, param_3, param_4, param_5);
    pass1_1028_7c4e(param_1, (uchar *)param_2, param_3, param_4);
    pass1_1028_7dfc(param_1, (uchar *)param_2, param_3, param_4, param_5);
    iVar1 = post_msg_1028_76da(param_1);
    pass1_1028_767e(iVar1, (ushort)param_2, param_3, param_4);
    pass1_1028_75bc(param_4);
    pass1_1028_78b8(param_1, param_2, param_3, param_4, param_5);
    return 0x1;
}

void __stdcall16far pass1_1028_75bc(ushort param_1)

{
    uint       uVar1;
    undefined *puVar2;
    uint       uVar3;
    ulong      uVar4;
    ulong      uStack28;
    undefined  local_18[0x8];
    undefined2 uStack16;
    uint       uStack14;
    undefined2 uStack12;
    uint       uStack10;
    int        iStack8;
    ulong      uStack6;

    uStack6 = *_PTR_LOOP_1050_65e2;
    uVar1   = (uint)((qword)uStack6 % 0x7b);
    uVar4   = (ulong)uVar1;
    if((uVar1 == 0x0) && (0x95 < uStack6))
    {
        pass1_1028_dc52((astruct_92 *)CONCAT22(param_1, local_18), 0x1, 0x0, 0x400);
        while(true)
        {
            uVar1  = (uint)uVar4;
            puVar2 = local_18;
            pass1_1028_e4ec(CONCAT22(param_1, puVar2));
            uStack28 = CONCAT22(uVar1, puVar2);
            uVar4    = (ulong)(uVar1 | (uint)puVar2);
            if((uVar1 | (uint)puVar2) == 0x0)
                break;
            pass1_1008_612e(0x1, 0x64, (uint)puVar2);
            if((int)puVar2 < 0x6)
            {
                pass1_1038_362e(uStack28);
            }
        }
        if(iStack8 != 0x0)
        {
            uStack12 = 0x1;
            uStack10 = 0x0;
        }
        uVar4    = (ulong)uStack10;
        uStack16 = uStack12;
        uStack14 = uStack10;
        while(true)
        {
            uVar1  = (uint)uVar4;
            puVar2 = local_18;
            pass1_1028_e4ec(CONCAT22(param_1, puVar2));
            uVar3 = uVar1 | (uint)puVar2;
            uVar4 = (ulong)uVar3;
            if(uVar3 == 0x0)
                break;
            pass1_1038_3698(CONCAT22(uVar1, puVar2), (ushort)puVar2, uVar3, param_1);
        }
    }
    return;
}

void __stdcall16far pass1_1028_7742(ushort param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    code      **ppcVar1;
    uint        uVar2;
    undefined  *puVar3;
    undefined2  uVar4;
    uchar      *puVar5;
    undefined2  extraout_DX;
    undefined2  uVar6;
    undefined2  extraout_DX_00;
    ulong      *puVar7;
    ulong       uVar8;
    undefined   uVar9;
    undefined   uVar10;
    uint        uVar11;
    ulong       uStack26;
    undefined   local_16[0x2];
    ulong       uStack20;
    undefined2  uStack16;
    undefined4 *puStack14;
    uint        uStack10;
    uchar      *puStack8;
    undefined2  uStack6;
    undefined2  uStack4;

    puVar7   = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x18);
    uVar4    = (undefined2)((ulong)puVar7 >> 0x10);
    uVar6    = SUB42(puVar7, 0x0);
    uStack6  = uVar6;
    uStack4  = uVar4;
    uVar8    = pass1_1028_b4f2(param_4);
    puVar5   = (uchar *)(uVar8 >> 0x10);
    uVar2    = (uint)uVar8;
    uStack10 = uVar2;
    puStack8 = puVar5;
    pass1_1038_4d6e(uVar8, (ulong *)CONCAT22(uVar4, uVar6), uVar2, puVar5);
    puStack14 = (undefined4 *)CONCAT22(puVar5, uVar2);
    uStack16  = 0x0;
    ppcVar1   = (code **)((int)*puStack14 + 0x10);
    uVar11    = uVar2;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar2, puVar5);
    uStack20 = CONCAT22(extraout_DX, uVar2);
    uVar8    = pass1_1030_bcae((ushort)local_16, param_5);
    uVar6    = (undefined2)(uVar8 >> 0x10);
    uStack26 = 0x0;
    do
    {
        if(uStack20 <= uStack26)
        {
        LAB_1028_77e7:
            if(puStack14 != (undefined4 *)0x0)
            {
                ppcVar1 = (code **)*puStack14;
                (**ppcVar1)(
                  0x1030, (int)puStack14, (char)((ulong)puStack14 >> 0x10), 0x1, uVar11, puVar5, puStack14, puStack14);
            }
            return;
        }
        uVar8 = uStack20;
        pass1_1030_1d58((ulong)puStack14);
        uVar4  = (undefined2)uVar8;
        uVar9  = (undefined)uVar8;
        uVar10 = (undefined)(uVar8 >> 0x8);
        pass1_1028_b58e(param_4);
        puVar3 = local_16;
        uVar8  = CONCAT22(uVar6, CONCAT11(uVar10, uVar9));
        uVar6  = extraout_DX_00;
        pass1_1030_bd74((ushort)puVar3, param_5, CONCAT22(extraout_DX_00, uVar4), uVar8, param_5);
        if((int)puVar3 <= param_3)
        {
            uStack16 = 0x1;
            goto LAB_1028_77e7;
        }
        uStack26 = uStack26 + 0x1;
    } while(true);
}


ushort __stdcall16far pass1_1028_81e0(uint param_1, ushort param_2, ushort param_3)

{
    int         iVar1;
    code      **ppcVar2;
    undefined  *puVar3;
    uint        uVar4;
    uint        extraout_DX;
    undefined4 *puStack24;
    undefined   local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
switchD_1028_8225_caseD_0:
    do
    {
        while(true)
        {
            uVar4  = param_1;
            puVar3 = local_14;
            pass1_1028_e4ec(CONCAT22(param_3, puVar3));
            puStack24 = (undefined4 *)CONCAT22(uVar4, puVar3);
            param_1   = uVar4 | (uint)puVar3;
            if(param_1 == 0x0)
            {
                return 0x1;
            }
            iVar1 = *(int *)(puVar3 + 0xc);
            if(iVar1 < 0x35)
                goto code_r0x10288222;
            if(0x61 < iVar1)
                break;
            if((iVar1 < 0x5d) && ((iVar1 != 0x37 && (iVar1 != 0x47))))
                goto switchD_1028_8225_caseD_1;
        }
    } while((iVar1 == 0x6a)
            || ((0x8 < iVar1 + -0x6a
                 && ((iVar1 == 0x75 || iVar1 + -0x74 < 0x1 || ((0x0 < iVar1 + -0x76 && (iVar1 + -0x78 < 0x2))))))));
    goto switchD_1028_8225_caseD_1;
code_r0x10288222:
    param_2 = (ushort)&USHORT_1050_1028;
    switch(iVar1)
    {
    case 0x1:
    case 0x2:
    case 0x3:
    case 0x4:
    case 0x6:
    case 0x7:
    case 0x8:
    case 0xa:
    case 0xb:
    case 0xc:
    case 0xd:
    case 0xe:
    case 0xf:
    case 0x11:
    switchD_1028_8225_caseD_1:
        if(*(int *)(puVar3 + 0x12) == 0x5)
        {
            ppcVar2 = (code **)((int)*puStack24 + 0x30);
            (**ppcVar2)(param_2);
            param_1 = extraout_DX;
        }
    }
    goto switchD_1028_8225_caseD_0;
}


void __stdcall16far pass1_1028_6356(ulong param_1, int param_2, uint param_3, int param_4, ushort param_5)

{
    int       *piVar1;
    uint       uVar2;
    uint       uVar3;
    code     **ppcVar4;
    int        iVar5;
    undefined2 uVar6;
    int        iVar7;
    undefined2 uVar8;
    bool       bVar9;
    long       lVar10;
    undefined  local_a[0x4];
    undefined4 uStack6;

    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar7 = (int)param_1;
    pass1_1008_5784((ulong *)CONCAT22(param_5, local_a), *(ulong *)(iVar7 + 0x20));
    while(true)
    {
        do
        {
            lVar10 = pass1_1008_5b12(local_a, param_5);
            uVar6  = (undefined2)((ulong)lVar10 >> 0x10);
            iVar5  = (int)lVar10;
            if(lVar10 == 0x0)
            {
                return;
            }
        } while(((*(int *)(iVar5 + 0x8) == 0x0) || ((param_2 != 0x0 && (*(int *)(iVar5 + 0x8) != param_2))))
                || ((*(int *)(iVar5 + 0x8) == 0xf && (param_2 != 0xf))));
        uVar2 = *(uint *)(iVar5 + 0xa);
        if((param_4 == 0x0) && (param_3 < uVar2))
            break;
        bVar9   = param_3 < uVar2;
        param_3 = param_3 - uVar2;
        param_4 = param_4 - (uint)bVar9;
        ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0x20) + 0xc);
        (**ppcVar4)(0x1008, *(undefined4 *)(iVar7 + 0x20));
        uStack6 = 0x0;
    }
    uVar3   = *(uint *)(iVar5 + 0xc);
    piVar1  = (int *)(iVar5 + 0xa);
    *piVar1 = *piVar1 - param_3;
    piVar1  = (int *)(iVar5 + 0xc);
    *piVar1 = *piVar1 - param_3 * (uVar3 / uVar2);
    return;
}


void __stdcall16far pass1_1028_6408(ulong param_1, ulong *param_2, ushort param_3)

{
    code     **ppcVar1;
    bool       bVar2;
    undefined *puVar3;
    uint       extraout_DX;
    int        iVar4;
    int        iVar5;
    undefined2 uVar6;
    undefined2 uVar7;
    undefined  local_a[0x8];

    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)(iVar4 + 0x20));
    bVar2 = false;
    while(true)
    {
        puVar3 = local_a;
        pass1_1008_5b12(puVar3, param_3);
        iVar5 = (int)param_2;
        uVar7 = (undefined2)((ulong)param_2 >> 0x10);
        if((extraout_DX | (uint)puVar3) == 0x0)
            break;
        if((*(int *)(puVar3 + 0x4) == *(int *)(iVar5 + 0x4)) && (*(int *)(puVar3 + 0x6) == *(int *)(iVar5 + 0x6)))
        {
            if(*(int *)(puVar3 + 0x8) == *(int *)(iVar5 + 0x8))
            {
                bVar2                  = true;
                *(int *)(puVar3 + 0xa) = *(int *)(puVar3 + 0xa) + *(int *)(iVar5 + 0xa);
                *(int *)(puVar3 + 0xc) = *(int *)(puVar3 + 0xc) + *(int *)(iVar5 + 0xc);
            }
        }
    }
    if(bVar2)
    {
        if(param_2 != (ulong *)0x0)
        {
            ppcVar1 = (code **)*param_2;
            (**ppcVar1)(0x1008, param_2, 0x1, param_2, param_2);
            return;
        }
    }
    else
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x20) + 0x4);
        (**ppcVar1)(0x1008, *(undefined4 *)(iVar4 + 0x20), param_2);
    }
    return;
}


ushort __stdcall16far pass1_1028_6744(ushort param_1, ulong param_2, int param_3)

{
    undefined2 uVar1;
    long       lVar2;
    undefined  local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_1, local_a), *(ulong *)((int)param_2 + 0x20));
    do
    {
        lVar2 = pass1_1008_5b12(local_a, param_1);
        uVar1 = (undefined2)((ulong)lVar2 >> 0x10);
        if(lVar2 == 0x0)
        {
            return 0x0;
        }
    } while(*(int *)((int)lVar2 + 0x6) != param_3);
    return *(ushort *)((int)lVar2 + 0xa);
}


ushort __stdcall16far pass1_1028_678c(ulong param_1, int param_2, ushort param_3)

{
    undefined2 uVar1;
    long       lVar2;
    undefined  local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)((int)param_1 + 0x20));
    do
    {
        lVar2 = pass1_1008_5b12(local_a, param_3);
        uVar1 = (undefined2)((ulong)lVar2 >> 0x10);
        if(lVar2 == 0x0)
        {
            return 0x0;
        }
    } while(*(int *)((int)lVar2 + 0x8) != param_2);
    return *(ushort *)((int)lVar2 + 0xa);
}


// WARNING: Could not reconcile some variable overlaps

ulong __stdcall16far pass1_1028_67d4(ulong param_1, ushort param_2)

{
    uint      uVar1;
    long      lVar2;
    ulong     uStack18;
    undefined local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_2, local_a), *(ulong *)((int)param_1 + 0x20));
    uStack18 = 0x0;
    while(true)
    {
        lVar2 = pass1_1008_5b12(local_a, param_2);
        if(lVar2 == 0x0)
            break;
        uVar1    = *(uint *)((int)lVar2 + 0xc);
        uStack18 = CONCAT22((int)(uStack18 >> 0x10) + (uint)CARRY2((uint)uStack18, uVar1), (uint)uStack18 + uVar1);
    }
    return uStack18;
}


ushort __stdcall16far pass1_1028_6822(ulong param_1, uint *param_2, ushort param_3)

{
    int   iVar1;
    ulong uVar2;

    uVar2                        = pass1_1028_67d4(param_1, param_3);
    iVar1                        = (int)(uVar2 >> 0x10);
    *param_2                     = (uint)uVar2;
    *(int *)((int)param_2 + 0x2) = iVar1;
    if((iVar1 == 0x0) && (*param_2 < 0x64))
    {
        return 0x0;
    }
    return 0x1;
}


void __stdcall16far pass1_1028_6926(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uint        uVar1;
    undefined4  uVar2;
    code      **ppcVar3;
    uint        uVar4;
    uint        uVar5;
    uchar      *puVar6;
    uint        extraout_DX;
    uchar      *extraout_DX_00;
    uchar      *puVar7;
    ushort      uVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    ulong      *puVar11;
    undefined4 *puStack14;

    uVar9 = (undefined2)(param_1 >> 0x10);
    uVar2 = *(undefined4 *)((int)param_1 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)((ulong)uVar2 >> 0x10));
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0xa);
    puVar6  = (uchar *)((ulong)puVar11 >> 0x10);
    uVar4   = (uint)puVar11;
    uVar10  = SUB42(&PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(CONCAT22(param_3, param_2), puVar11, uVar4, puVar6);
    puStack14 = (undefined4 *)CONCAT22(puVar6, uVar4);
    uVar2     = *puStack14;
    uVar8     = (ushort)uVar2;
    ppcVar3   = (code **)(uVar8 + 0x10);
    uVar5     = uVar4;
    (**ppcVar3)((int)&PTR_LOOP_1050_1038, uVar4, puVar6);
    if((extraout_DX | uVar5) != 0x0)
    {
        ppcVar3 = (code **)(uVar8 + 0x4);
        (**ppcVar3)(0x38, uVar4, puVar6, 0x0, 0x0);
        puVar7 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, (uint)extraout_DX_00);
        uVar1  = *(uint *)((int)param_1 + 0x10c);
        uVar10 = 0x1030;
        pass1_1030_7ddc(CONCAT22(puVar7, uVar5),
                        CONCAT13((undefined)((int)uVar1 >> 0xf), (int3)(int)uVar1),
                        0x1f,
                        uVar1,
                        puVar7,
                        uVar8,
                        (ushort)((ulong)uVar2 >> 0x10),
                        param_4);
    }
    if(puStack14 != (undefined4 *)0x0)
    {
        ppcVar3 = (code **)*puStack14;
        (**ppcVar3)(uVar10, uVar4, (char)puVar6, 0x1);
    }
    return;
}


ushort __stdcall16far pass1_1028_6b2c(ulong param_1, ushort param_2)

{
    pass1_1028_6b40(param_1, param_2);
    return 0x1;
}


void __stdcall16far pass1_1028_6b40(ulong param_1, ushort param_2)

{
    undefined4  uVar1;
    code      **ppcVar2;
    undefined  *puVar3;
    ushort      in_DX;
    ushort      uVar4;
    ushort      uVar5;
    ushort      uVar6;
    ushort     *puVar7;
    undefined   local_36[0xe];
    undefined4 *puStack40;
    ushort      uStack38;
    ushort      uStack36;
    ushort      uStack34;
    ushort      uStack32;
    ushort      uStack30;
    ushort      uStack28;
    ushort      uStack26;
    undefined4  local_18;
    ushort      uStack20;
    ulong       uStack18;
    undefined4  uStack14;
    undefined4 *puStack10;
    undefined   local_6[0x2];
    int         local_4;

    puVar3 = local_6;
    pass1_1028_6daa(param_1,
                    (ushort *)CONCAT22(param_2, puVar3),
                    (ushort *)CONCAT22(param_2, &local_4),
                    (ushort)puVar3,
                    in_DX,
                    param_2);
    uVar6 = (ushort)(param_1 >> 0x10);
    uVar5 = (ushort)param_1;
    uVar1 = *(undefined4 *)(uVar5 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    puStack10 = (undefined4 *)CONCAT22(in_DX, puVar3);
    ppcVar2   = (code **)((int)*puStack10 + 0x24);
    (**ppcVar2)();
    uStack14  = pass1_1028_b58e((ulong)puStack10);
    uStack18  = pass1_1028_bb24((ulong)puStack10);
    local_18  = *(undefined4 *)((int)uStack14 + 0xc);
    uStack20  = *(ushort *)((int)uStack14 + 0x10);
    puStack40 = &local_18;
    uStack26  = (ushort)local_18;
    uStack28  = (ushort)((ulong)local_18 >> 0x10);
    uStack32  = (ushort)local_18 - 0x1;
    if((int)uStack32 < 0x0)
    {
        uStack32 = 0x0;
    }
    uVar4    = local_4 - 0x1;
    uStack34 = (ushort)local_18 + 0x1;
    if((int)uVar4 < (int)((ushort)local_18 + 0x1))
    {
        uStack34 = uVar4;
    }
    uStack36 = uStack28 - 0x1;
    if((int)uStack36 < 0x0)
    {
        uStack36 = 0x0;
    }
    uStack38 = uStack28 + 0x1;
    if((int)uVar4 < (int)(uStack28 + 0x1))
    {
        uStack38 = uVar4;
    }
    uStack30 = uStack20;
    puVar7   = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack20, uStack36, uStack32);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack30, uStack36, uStack26);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack30, uStack36, uStack34);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack30, uStack28, uStack32);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack30, uStack28, uStack34);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack30, uStack38, uStack32);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack30, uStack38, uStack26);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_2, local_36), uStack30, uStack38, uStack34);
    pass1_1028_6d24(
      uVar5, uVar6, (ushort *)CONCAT22(param_2, local_36), uStack18, (ushort)((ulong)puVar7 >> 0x10), param_2);
    return;
}


ushort __stdcall16far pass1_1028_6e96(uint param_1, ushort param_2)

{
    int         iVar1;
    code      **ppcVar2;
    undefined  *puVar3;
    uint        uVar4;
    uint        extraout_DX;
    undefined4 *puStack24;
    undefined   local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, local_14), 0x1, 0x0, 0x700);
    while(true)
    {
        puVar3 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar3));
        puStack24 = (undefined4 *)CONCAT22(param_1, puVar3);
        uVar4     = param_1 | (uint)puVar3;
        if(uVar4 == 0x0)
            break;
        iVar1   = *(int *)(puVar3 + 0x12);
        param_1 = uVar4;
        if(((0x0 < iVar1) && (!SBORROW2(iVar1, 0x1))) && (iVar1 + -0x1 < 0x4))
        {
            ppcVar2 = (code **)((int)*puStack24 + 0x38);
            (**ppcVar2)();
            param_1 = extraout_DX;
        }
    }
    return 0x1;
}


void __stdcall16far pass1_1028_740c(ushort param_1, ushort param_2, int param_3, ulong param_4)

{
    code      **ppcVar1;
    undefined4  uVar2;
    uint        uVar3;
    uint        uVar4;
    uchar      *puVar5;
    undefined2  extraout_DX;
    ulong      *puVar6;
    long        lStack14;
    undefined4 *puStack10;

    puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, param_3);
    puVar5 = (uchar *)((ulong)puVar6 >> 0x10);
    uVar3  = (uint)puVar6;
    pass1_1038_4d6e(param_4, puVar6, uVar3, puVar5);
    puStack10 = (undefined4 *)CONCAT22(puVar5, uVar3);
    uVar2     = *puStack10;
    ppcVar1   = (code **)uVar2 + 0x8;
    uVar4     = uVar3;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar3, puVar5);
    lStack14 = CONCAT22(extraout_DX, uVar4);
    if(puStack10 != (undefined4 *)0x0)
    {
        ppcVar1 = (code **)uVar2;
        (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar3, (char)puVar5, 0x1);
    }
    if(lStack14 != 0x0)
    {
        return;
    }
    return;
}


ushort *__stdcall16far pass1_1028_50fa(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x5280;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_530a(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x535e;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_53e8(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x54bc;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_5412(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined2 uVar2;
    ulong      uVar3;
    int        iVar4;

    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) != 0x6)
    {
        return;
    }
    uVar3 = pass1_1028_b4f2((ulong)param_1);
    if(*(long *)((int)uVar3 + 0x200) != 0x8000002)
    {
        if(*(long *)((int)param_1 + 0x1c) == 0x8000002)
        {
            iVar4 = 0x6;
            goto code_r0x1028548e;
        }
        ppcVar1 = (code **)((int)*param_1 + 0x64);
        iVar4   = (**ppcVar1)(param_4, param_1);
        if(iVar4 == 0x0)
        {
            return;
        }
        pass1_1028_c0f0((ulong)param_1, 0x1, iVar4, param_2, param_3, param_5);
        if(iVar4 == 0x0)
        {
            iVar4 = 0x6;
            goto code_r0x1028548e;
        }
        pass1_1028_c952((ulong)param_1, param_2, param_3, param_5);
        pass1_1028_c00a((ulong)param_1, 0x1, iVar4, param_5);
    }
    iVar4 = 0x5;
code_r0x1028548e:
    pass1_1028_bdac(param_1, iVar4, param_4);
    return;
}


ushort *__stdcall16far pass1_1028_5546(int param_1, ushort param_2, ushort param_3, ulong param_4, uchar *param_5)

{
    pass1_1028_00cc(param_1, param_2, param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x55c8;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_5570(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    uint       uVar1;
    ulong      uVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    int        iVar5;

    pass1_1028_0550(param_1, param_2, param_3, param_4, param_5);
    uVar1 = (uint)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        uVar4 = 0x0;
        iVar5 = 0x4;
        uVar3 = 0x1;
        uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        pass1_1030_7c50(uVar2, CONCAT22(uVar4, uVar3), iVar5, (uint)uVar2, (uchar *)(uVar2 >> 0x10));
    }
    return;
}


ushort *__stdcall16far pass1_1028_5652(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x56ac;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_57c8(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x581c;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_58a6(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x58fe;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}
