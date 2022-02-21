
int __stdcall16far pass1_1030_d56a(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    switch(*(int *)(iVar1 + 0x98) + -0x1)
    {
    case 0x0:
        *(undefined2 *)(iVar1 + 0x98) = 0x2;
        break;
    case 0x1:
        *(undefined2 *)(iVar1 + 0x98) = 0x3;
        break;
    case 0x2:
        *(undefined2 *)(iVar1 + 0x98) = 0x4;
        break;
    case 0x3:
        *(undefined2 *)(iVar1 + 0x98) = 0xc;
        break;
    default:
        *(undefined2 *)(iVar1 + 0x98) = 0x1;
        return iVar1;
    case 0x7:
        *(undefined2 *)(iVar1 + 0x98) = 0x9;
        return iVar1;
    case 0x8:
        *(undefined2 *)(iVar1 + 0x98) = 0xb;
        return iVar1;
    case 0xa:
        *(undefined2 *)(iVar1 + 0x98) = 0x5;
        return iVar1;
    case 0xb:
        *(undefined2 *)(iVar1 + 0x98) = 0x8;
        return iVar1;
    }
    return iVar1;
}


ulong __stdcall16far pass1_1030_d942(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xdc2e;
    *(undefined2 *)(param_1 + 0x2)            = 0x1030;
    if(*(int *)(param_1 + 0xc) == 0x4c)
    {
        *(undefined2 *)(param_1 + 0xe) = 0x43;
    }
    else
    {
        if(*(int *)(param_1 + 0xc) == 0x4d)
        {
            *(undefined2 *)(param_1 + 0xe) = 0x44;
        }
        else
        {
            *(undefined2 *)(param_1 + 0xe) = 0x45;
        }
    }
    return CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1030_d994(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    int        iVar4;
    undefined2 uVar5;
    ulong      uVar6;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(int *)(iVar4 + 0x12) != 0x4)
    {
        return;
    }
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
        pass1_1028_cb04((ulong)param_1, param_2, param_3, param_4);
        if(iVar3 == 0x0)
        {
            return;
        }
        pass1_1030_dace((ulong)param_1, param_4);
        if(iVar3 == 0x0)
        {
            return;
        }
        uVar2   = *(undefined4 *)(iVar4 + 0x14);
        piVar1  = (int *)((int)uVar2 + 0x94);
        *piVar1 = *piVar1 + -0x1;
        pass1_1028_c952((ulong)param_1, param_2, param_3, param_4);
        pass1_1030_da22((ulong)param_1, param_4);
    }
    uVar2 = *(undefined4 *)(iVar4 + 0x14);
    if(*(int *)((int)uVar2 + 0x94) < 0x1)
    {
        pass1_1028_bdac(param_1, 0x5, (ushort)&USHORT_1050_1028);
    }
    return;
}


void __stdcall16far pass1_1030_da22(ulong param_1, ushort param_2)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    undefined2  uVar3;
    BOOL16      BVar4;
    ushort      uVar5;
    undefined4 *puVar6;
    uint        extraout_DX;
    uint        uVar7;
    uint        uVar8;
    ulong       uVar9;
    ulong       uStack18;

    uVar9   = pass1_1028_b4f2(param_1);
    uVar3   = (undefined2)(uVar9 >> 0x10);
    puVar1  = (undefined4 *)*(ulong *)((int)uVar9 + 0xc);
    ppcVar2 = (code **)((int)*puVar1 + 0x10);
    puVar6  = puVar1;
    (**ppcVar2)((int)&USHORT_1050_1028, (int)puVar1, *(undefined2 *)((int)uVar9 + 0xe));
    uStack18 = 0x0;
    while(true)
    {
        if(((ulong)puVar6 & 0xffff | (ulong)extraout_DX << 0x10) <= uStack18)
        {
            return;
        }
        uVar9 = pass1_1030_1d7c((int)((ulong)puVar6 & 0xffff), extraout_DX, (ulong)puVar1);
        uVar7 = (uint)(uVar9 >> 0x10);
        uVar8 = uVar7 | (uint)uVar9;
        if(((uVar8 != 0x0)
            && (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((uint)uVar9 + 0xc), 0x4), BVar4 != 0x0))
           && (uVar5 = pass1_1028_6744(param_2, uVar9, 0xd), (uVar8 | uVar5) != 0x0))
            break;
        uStack18 = uStack18 + 0x1;
    }
    pass1_1028_6228(uVar9, 0x1, 0x0, 0xd, param_2);
    return;
}


void __stdcall16far pass1_1030_dace(ulong param_1, ushort param_2)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    undefined2  uVar3;
    BOOL16      BVar4;
    ushort      uVar5;
    undefined4 *puVar6;
    uint        extraout_DX;
    uint        uVar7;
    uint        uVar8;
    ulong       uVar9;
    ulong       uStack20;

    uVar9   = pass1_1028_b4f2(param_1);
    uVar3   = (undefined2)(uVar9 >> 0x10);
    puVar1  = (undefined4 *)*(ulong *)((int)uVar9 + 0xc);
    ppcVar2 = (code **)((int)*puVar1 + 0x10);
    puVar6  = puVar1;
    (**ppcVar2)((int)&USHORT_1050_1028, (int)puVar1, *(undefined2 *)((int)uVar9 + 0xe));
    uStack20 = 0x0;
    uVar8    = extraout_DX;
    do
    {
        if(((ulong)puVar6 & 0xffff | (ulong)extraout_DX << 0x10) <= uStack20)
        {
            return;
        }
        uVar9 = pass1_1030_1d7c((int)((ulong)puVar6 & 0xffff), uVar8, (ulong)puVar1);
        uVar7 = (uint)(uVar9 >> 0x10);
        uVar8 = uVar7 | (uint)uVar9;
        if((uVar8 != 0x0)
           && (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((uint)uVar9 + 0xc), 0x4), BVar4 != 0x0))
        {
            uVar5 = pass1_1028_6744(param_2, uVar9, 0xd);
            uVar8 = uVar8 | uVar5;
            if(uVar8 != 0x0)
            {
                return;
            }
        }
        uStack20 = uStack20 + 0x1;
    } while(true);
}


ushort __stdcall16far pass1_1030_db72(void)

{
    return 0x1;
}


void __stdcall16far pass1_1030_db78(ulong param_1)

{
    uint uVar1;

    uVar1 = (uint)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) == 0x6)
    {
        pass1_1028_bdac((ulong *)(param_1 & 0xffff | (ulong)uVar1 << 0x10), 0x5, (ushort)&USHORT_1050_1028);
    }
    return;
}


void __stdcall16far
pass1_1030_db92(ushort param_1, ushort param_2, ushort *param_3, ulong param_4, long param_5, ushort param_6)

{
    int        iVar1;
    undefined *puVar2;
    uint       uVar3;
    ulong      uVar4;
    undefined  local_4[0x2];

    uVar4 = pass1_1030_bcae((ushort)local_4, param_6);
    uVar3 = (uint)(uVar4 >> 0x10);
    iVar1 = (int)uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
    uVar4 = *(ulong *)(iVar1 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar4, (uint)(uVar4 >> 0x10));
    puVar2 = local_4;
    pass1_1030_bcde(param_6, (ushort)puVar2, param_6, uVar4 & 0xffff | (ulong)uVar3 << 0x10, param_3, param_5);
    if((int)puVar2 < 0x0)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
        return;
    }
    return;
}


ushort __stdcall16far pass1_1030_dc02(void)

{
    return 0x1;
}


ushort *__stdcall16far pass1_1030_dcc2(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined4 *)(param_1 + 0x20)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0xe036;
    *(undefined2 *)(param_1 + 0x2)        = 0x1030;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1030_dcf4(ushort *param_1, uint param_2)

{
    long         lVar1;
    code       **ppcVar2;
    uint         uVar3;
    uint         uVar4;
    ushort       uVar5;
    uint         extraout_DX;
    uint         uVar6;
    uchar       *puVar7;
    uint         extraout_DX_00;
    uint         uVar8;
    astruct_596 *iVar9;
    undefined2   uVar9;
    ulong       *puVar10;
    ulong        uVar11;
    ulong        uStack28;
    ulong        uStack24;
    undefined4  *puStack20;
    int          iStack12;

    uVar9            = (undefined2)((ulong)param_1 >> 0x10);
    iVar9            = (astruct_596 *)param_1;
    *param_1         = 0xe036;
    iVar9->field_0x2 = 0x1030;
    if(_PTR_LOOP_1050_65e2 != 0x0)
    {
        pass1_1028_b58e((ulong)param_1);
        if(iVar9->field_0x20 == 0x0)
        {
            uVar3 = extraout_DX | param_2;
            if(uVar3 == 0x0)
            {
                uVar6 = extraout_DX;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x1, 0x400);
            }
            else
            {
                uVar3 = *(uint *)(param_2 + 0x2e);
                uVar6 = *(uint *)(param_2 + 0x30);
            }
            puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
            puVar7  = (uchar *)((ulong)puVar10 >> 0x10);
            uVar4   = (uint)puVar10;
            pass1_1038_4d6e(CONCAT22(uVar6, uVar3), puVar10, uVar4, puVar7);
            puStack20 = (undefined4 *)CONCAT22(puVar7, uVar4);
            ppcVar2   = (code **)((int)*puStack20 + 0x10);
            uVar6     = uVar4;
            (**ppcVar2)((int)&PTR_LOOP_1050_1038, uVar4, puVar7);
            uStack24 = CONCAT22(extraout_DX_00, uVar6);
            uVar3    = extraout_DX_00;
            for(uStack28 = 0x0; uStack28 < uStack24; uStack28 = uStack28 + 0x1)
            {
                uVar11 = pass1_1030_1d7c(uVar6, uVar3, (ulong)puStack20);
                uVar8  = (uint)(uVar11 >> 0x10);
                uVar3  = uVar8 | (uint)uVar11;
                if(uVar3 != 0x0)
                {
                    uVar5 = pass1_1030_dfcc((ulong)param_1);
                    uVar5 = pass1_1030_cbf0((uint)uVar11, uVar8, uVar5);
                    if(uVar5 != 0x0)
                        break;
                }
            }
            if(puStack20 != (undefined4 *)0x0)
            {
                ppcVar2 = (code **)*puStack20;
                (**ppcVar2)(0x38, uVar4, puVar7, 0x1);
            }
        }
        else
        {
            lVar1 = iVar9->field_0x20;
            uVar3 = extraout_DX;
            uVar6 = param_2;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)lVar1, (uint)((ulong)lVar1 >> 0x10));
            if((uVar3 | uVar6) != 0x0)
            {
                iStack12 = 0x0;
                switch(iVar9->field_0xc)
                {
                case 0x73:
                case 0x77:
                    iStack12 = 0x1;
                    break;
                case 0x74:
                case 0x78:
                    iStack12 = 0x2;
                    break;
                case 0x75:
                    iStack12 = 0x3;
                    break;
                case 0x76:
                    iStack12 = 0x5;
                }
                if(iStack12 != 0x0)
                {
                    pass1_1030_cc44(uVar6, uVar3, 0x1, CONCAT22(extraout_DX, param_2), iStack12);
                }
            }
        }
    }
    pass1_1028_b418(param_1);
    return;
}


ushort *__stdcall16far pass1_1030_bc24(ushort param_1, int param_2, ushort param_3, ushort param_4, ulong param_5)

{
    pass1_1028_b22c((ushort *)CONCAT22(param_3, param_2), param_4, param_5, param_1);
    *(ushort *)CONCAT22(param_3, param_2) = 0xbc96;
    *(undefined2 *)(param_2 + 0x2)        = 0x1030;
    return (ushort *)CONCAT22(param_3, param_2);
}


void __stdcall16far pass1_1030_bc4e(ushort *param_1)

{
    *param_1                            = 0xbc96;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
    pass1_1028_b260(param_1);
    return;
}


ulong __stdcall16far pass1_1030_bcae(ushort param_1, ushort param_2)

{
    return CONCAT22(param_2, param_1);
}


void __stdcall16far
pass1_1030_bcbc(ushort param_1, ushort param_2, ulong param_3, ulong param_4, ushort param_5, ulong param_6)

{
    pass1_1030_bcde(param_1,
                    param_2,
                    (ushort)param_3,
                    CONCAT22((undefined2)param_4, param_3._2_2_),
                    (ushort *)CONCAT22(param_5, param_4._2_2_),
                    *(long *)((int)param_6 + 0x4));
    return;
}


void __stdcall16far
pass1_1030_bcde(ushort param_1, ushort param_2, ushort param_3, ulong param_4, ushort *param_5, long param_6)

{
    int        iVar1;
    undefined2 uVar2;
    int        local_14;
    int        local_12;
    int        local_10;
    int        local_e;
    undefined4 local_c;
    undefined2 uStack8;
    long       lStack6;

    uVar2   = (undefined2)(param_4 >> 0x10);
    iVar1   = (int)param_4;
    lStack6 = *(long *)(iVar1 + 0x8);
    if(lStack6 != param_6)
    {
        return;
    }
    local_c = *(undefined4 *)(iVar1 + 0xc);
    uStack8 = *(undefined2 *)(iVar1 + 0x10);
    pass1_1008_3e94(param_5, (ushort *)CONCAT22(param_1, &local_10), (ushort *)CONCAT22(param_1, &local_e));
    pass1_1008_3e94((ushort *)CONCAT22(param_1, &local_c),
                    (ushort *)CONCAT22(param_1, &local_14),
                    (ushort *)CONCAT22(param_1, &local_12));
    pass1_1000_49b2(local_e - local_12);
    pass1_1000_49b2(local_10 - local_14);
    return;
}


void __stdcall16far pass1_1030_bd74(ushort param_1, ushort param_2, ulong param_3, ulong param_4, ushort param_5)

{
    astruct_670 *iVar1;
    int          iVar2;
    undefined2   uVar3;
    undefined2   uVar4;
    int          local_1e;
    int          local_1c;
    int          local_1a;
    int          local_18;
    undefined4   local_16;
    undefined2   uStack18;
    undefined4   local_10;
    undefined2   uStack12;
    long         lStack10;
    long         lStack6;

    uVar3    = (undefined2)(param_4 >> 0x10);
    iVar1    = (astruct_670 *)param_4;
    lStack6  = iVar1->field_0x8;
    uVar4    = (undefined2)(param_3 >> 0x10);
    iVar2    = (int)param_3;
    lStack10 = *(long *)(iVar2 + 0x8);
    if(lStack10 != lStack6)
    {
        return;
    }
    local_10 = iVar1->field_0xc;
    uStack12 = iVar1->field_0x10;
    local_16 = *(undefined4 *)(iVar2 + 0xc);
    uStack18 = *(undefined2 *)(iVar2 + 0x10);
    pass1_1008_3e94((ushort *)CONCAT22(param_5, &local_10),
                    (ushort *)CONCAT22(param_5, &local_1a),
                    (ushort *)CONCAT22(param_5, &local_18));
    pass1_1008_3e94((ushort *)CONCAT22(param_5, &local_16),
                    (ushort *)CONCAT22(param_5, &local_1e),
                    (ushort *)CONCAT22(param_5, &local_1c));
    pass1_1000_49b2(local_18 - local_1c);
    pass1_1000_49b2(local_1a - local_1e);
    return;
}


ushort *__stdcall16far pass1_1030_be56(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xc006;
    *(undefined2 *)(param_1 + 0x2)        = 0x1030;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1030_be80(ulong param_1, uchar *param_2, ushort param_3)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    BOOL16     BVar4;
    ulong      uVar5;
    uint       extraout_DX;
    uint       uVar6;
    int        iVar7;
    undefined2 uVar8;
    int        iVar9;

    pass1_1028_bf22(param_1, param_2, param_3);
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar7 = (int)param_1;
    if(*(int *)(iVar7 + 0x12) == 0x5)
    {
        uVar2                              = *(undefined4 *)(iVar7 + 0x14);
        *(undefined2 *)((int)uVar2 + 0xa4) = 0x1e;
        uVar2                              = *(undefined4 *)(iVar7 + 0x14);
        *(undefined2 *)((int)uVar2 + 0xac) = 0x1;
        iVar9                              = *(int *)(iVar7 + 0xc);
        iVar3                              = iVar9 + -0x1b;
        if(iVar3 == 0x0)
        {
            uVar2                              = *(undefined4 *)(iVar7 + 0x14);
            *(undefined2 *)((int)uVar2 + 0xaa) = 0xa;
        }
        else
        {
            iVar3 = iVar9 + -0x1c;
            if(iVar3 == 0x0)
            {
                uVar2                              = *(undefined4 *)(iVar7 + 0x14);
                *(undefined2 *)((int)uVar2 + 0xaa) = 0xb;
            }
            else
            {
                iVar3 = iVar9 + -0x1d;
                if(iVar3 == 0x0)
                {
                    uVar2                              = *(undefined4 *)(iVar7 + 0x14);
                    *(undefined2 *)((int)uVar2 + 0xaa) = 0xc;
                }
            }
        }
        pass1_1028_b58e(param_1);
        uVar5 = *(ulong *)(iVar3 + 0x2e);
        iVar9 = 0xc;
        uVar6 = extraout_DX;
        pass1_1038_3fb0(uVar5);
        BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (ulong)uVar6 << 0x10, iVar9);
        if(BVar4 != 0x0)
        {
            uVar2   = *(undefined4 *)(iVar7 + 0x14);
            piVar1  = (int *)((int)uVar2 + 0xaa);
            *piVar1 = *piVar1 + 0x1;
        }
        BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (ulong)uVar6 << 0x10, 0xe);
        if(BVar4 != 0x0)
        {
            uVar2   = *(undefined4 *)(iVar7 + 0x14);
            piVar1  = (int *)((int)uVar2 + 0xaa);
            *piVar1 = *piVar1 + 0x1;
        }
        BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (ulong)uVar6 << 0x10, 0x76);
        if(BVar4 != 0x0)
        {
            uVar2   = *(undefined4 *)(iVar7 + 0x14);
            piVar1  = (int *)((int)uVar2 + 0xaa);
            *piVar1 = *piVar1 + 0x1;
        }
    }
    return;
}


void __stdcall16far pass1_1030_bf6e(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uint      *puVar1;
    undefined4 uVar2;
    uint       uVar3;
    uint       uVar4;
    int        iVar5;
    undefined2 uVar6;
    ulong      uVar7;
    undefined4 uVar8;
    ushort     uVar9;

    uVar9  = 0x1e;
    uVar7  = pass1_1028_b58e(param_1);
    uVar8  = pass1_1030_7c28(uVar7, uVar9, (uint)uVar7, (uint)(uVar7 >> 0x10), param_4);
    uVar4  = 0x3e8 - (int)uVar8;
    uVar2  = *(undefined4 *)((int)param_1 + 0x14);
    uVar6  = (undefined2)((ulong)uVar2 >> 0x10);
    iVar5  = (int)uVar2;
    puVar1 = (uint *)(iVar5 + 0xaa);
    uVar3  = -(uint)(uVar4 < *puVar1);
    pass1_1030_7ddc(uVar7,
                    (ulong)((uVar4 - *puVar1 & uVar3) + *(int *)(iVar5 + 0xaa)),
                    0x1e,
                    uVar3,
                    (uchar *)((ulong)uVar8 >> 0x10),
                    param_2,
                    param_3,
                    param_4);
    return;
}


ushort __stdcall16far pass1_1030_bfb8(ulong param_1, ushort param_2)

{
    ulong      uVar1;
    undefined4 uVar2;
    ushort     uVar3;

    uVar3 = 0x1e;
    uVar1 = pass1_1028_b58e(param_1);
    uVar2 = pass1_1030_7c28(uVar1, uVar3, (uint)uVar1, (uint)(uVar1 >> 0x10), param_2);
    return 0x3e8 - (int)uVar2;
}


void __stdcall16far pass1_1030_c09c(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined4 *)(param_1 + 0x20)           = 0x0;
    *(undefined2 *)(param_1 + 0x24)           = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xc68e;
    *(undefined2 *)(param_1 + 0x2)            = 0x1030;
    return;
}


ushort __stdcall16far pass1_1030_c0d2(ulong param_1)

{
    if(0x0 < *(int *)((int)param_1 + 0x24))
    {
        return 0x1;
    }
    return 0x0;
}


ushort __stdcall16far pass1_1030_c0ec(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    if((*(int *)((int)param_1 + 0xc) != 0xb) && (*(int *)((int)param_1 + 0x24) < 0x1))
    {
        return 0x0;
    }
    return 0x1;
}


void __stdcall16far pass1_1030_c10e(ulong param_1)

{
    int       *piVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(0x0 < *(int *)(iVar2 + 0x24))
    {
        piVar1  = (int *)(iVar2 + 0x24);
        *piVar1 = *piVar1 + -0x1;
        return;
    }
    *(undefined2 *)(iVar2 + 0xc) = 0x37;
    return;
}


void __stdcall16far
pass1_1030_c12e(ulong param_1, int param_2, int param_3, ushort param_4, ushort param_5, ushort param_6)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 extraout_DX;
    int        iVar4;
    undefined2 uVar5;
    uint       uVar6;
    ulong      uStack6;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_DX, param_3);
    uVar2   = *(undefined4 *)(param_3 + 0x2e);
    uVar5   = (undefined2)(param_1 >> 0x10);
    iVar4   = (int)param_1;
    iVar3   = (int)uVar2;
    if(*(int *)(iVar4 + 0x24) < 0x1)
    {
        pass1_1030_7d1c(uStack6, 0x0, 0x230000, iVar3, extraout_DX, param_4, param_5, param_6);
    }
    else
    {
        if(param_2 == 0x0)
        {
            uVar6 = 0x0;
        }
        else
        {
            uVar6 = 0x32;
        }
        pass1_1030_7d1c(uStack6, uVar6, 0x230000, iVar3, extraout_DX, param_4, param_5, param_6);
        piVar1  = (int *)(iVar4 + 0x24);
        *piVar1 = *piVar1 + -0x1;
    }
    if((0x0 < *(int *)(iVar4 + 0x24)) && (*(int *)(iVar4 + 0x24) < 0x19))
    {
        *(undefined2 *)(iVar3 + 0x1fe) = 0x1;
    }
    return;
}


void __stdcall16far pass1_1030_c52e(
  ulong param_1, ushort *param_2, ulong param_3, ulong param_4, int param_5, ushort param_6, ushort param_7)

{
    BOOL16      BVar1;
    ulong      *puVar2;
    undefined  *puVar3;
    undefined4 *puVar4;
    uint        uVar5;
    uint        uVar6;
    ulong       uVar7;
    ushort      uVar8;
    ushort     *puVar9;
    ulong       uVar10;
    undefined   local_32[0x12];
    undefined4  local_20;
    undefined4  uStack28;
    undefined4 *puStack24;
    ulong       uStack22;
    undefined2  uStack18;
    uint        uStack16;
    ulong       local_c;
    uint        uStack8;
    ulong       uStack6;

    uVar8 = (ushort)(param_1 >> 0x10);
    BVar1 = pass1_1028_c314(
      param_7, param_5, param_6, (ushort)param_1, uVar8, param_2, (ushort)param_3, (ushort)(param_3 >> 0x10), param_4);
    if(BVar1 != 0x0)
    {
        puVar2 = &local_c;
        pass1_1030_64ce(
          param_7, puVar2, param_6, _PTR_LOOP_1050_5740, param_2, param_4, (ulong *)CONCAT22(param_7, puVar2));
        local_20       = *puVar2;
        local_20._3_1_ = (byte)(local_20 >> 0x18);
        uStack8        = (uint)local_20._3_1_;
        if(local_20._3_1_ == 0x0)
        {
            uStack22 = local_20;
            uStack6  = local_20;
            pass1_1028_c7b6(param_7, param_6, (ushort)param_1, uVar8, param_2, param_4);
            if((uStack8 != 0x4) && (uStack8 != 0x0))
            {
                uVar7 = pass1_1030_bcae((ushort)&local_20, param_7);
                uVar5 = (uint)(uVar7 >> 0x10);
                pass1_1028_dc52((astruct_92 *)CONCAT22(param_7, local_32), 0x1, 0x0, 0x400);
                while(true)
                {
                    puVar3 = local_32;
                    pass1_1028_e4ec(CONCAT22(param_7, puVar3));
                    uStack28 = CONCAT22(uVar5, puVar3);
                    uVar6    = uVar5 | (uint)puVar3;
                    if(uVar6 == 0x0)
                    {
                        return;
                    }
                    uVar7    = *(ulong *)(puVar3 + 0x10);
                    uVar10   = param_4;
                    uStack22 = uVar7;
                    puVar9   = param_2;
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar7, (uint)(uVar7 >> 0x10));
                    uStack18 = (undefined2)uVar7;
                    puVar4   = &local_20;
                    uStack16 = uVar6;
                    pass1_1030_bcde(
                      param_7, (ushort)puVar4, param_7, uVar7 & 0xffff | (ulong)uVar6 << 0x10, puVar9, uVar10);
                    if((int)puVar4 < 0x0)
                        break;
                    uVar5     = uVar6;
                    puStack24 = puVar4;
                    if((int)puVar4 < 0x1f)
                    {
                        PTR_LOOP_1050_50ca = (undefined *)0x6ae;
                        return;
                    }
                }
                PTR_LOOP_1050_50ca = (undefined *)0x6af;
                return;
            }
            PTR_LOOP_1050_50ca = (undefined *)0x6a8;
        }
    }
    return;
}


ushort *__stdcall16far pass1_1030_c71e(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)(param_1 + 0x20)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0xc940;
    *(undefined2 *)(param_1 + 0x2)        = 0x1030;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1030_c74e(ulong param_1, ulong param_2, ushort param_3)

{
    pass1_1028_b46e(param_1, param_2, param_3);
    *(undefined2 *)((int)param_1 + 0x20) = 0x70;
    return;
}


void __stdcall16far pass1_1030_c76c(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    iVar1 = (int)param_1;
    if((*(int *)(iVar1 + 0x12) != 0x6) && (*(int *)(iVar1 + 0x12) != 0x5))
    {
        return;
    }
    iVar1 = *(int *)(iVar1 + 0x20);
    if(iVar1 != 0x0)
    {
        if(((iVar1 < 0x70) || (SBORROW2(iVar1, 0x70))) || (0x1 < iVar1 + -0x70))
        {
            pass1_1028_be2a(param_1, param_2, param_3, (ushort)&USHORT_1050_1028, param_4);
            return;
        }
    }
    pass1_1028_bdac(param_1, 0x6, (ushort)&USHORT_1050_1028);
    return;
}


ulong __stdcall16far pass1_1030_c8da(ulong param_1, ulong param_2, ulong param_3)

{
    ulong uVar1;

    uVar1 = 0x0;
    if(param_3._2_2_ == 0x1)
    {
        *(undefined2 *)((int)param_1 + 0x20) = param_2._2_2_;
    }
    else
    {
        uVar1 = func_0x1030178e();
    }
    return uVar1;
}


ulong __stdcall16far pass1_1030_c9e4(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)(param_1 + 0x98)           = 0x1;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xd88e;
    *(undefined2 *)(param_1 + 0x2)            = 0x1030;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, param_1 + 0x20), (WNDCLASS16 *)0x0, 0x78);
    return CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1030_ca26(ulong param_1, ulong param_2, ushort param_3)

{
    ushort     uVar1;
    undefined2 extraout_DX;
    int        iVar2;
    undefined2 uVar3;
    ushort     uStack4;

    for(uStack4 = 0x0; iVar2 = (int)param_1, uVar3 = (undefined2)(param_1 >> 0x10), (int)uStack4 < 0xa;
        uStack4 = uStack4 + 0x1)
    {
        if((*(int *)(iVar2 + uStack4 * 0xc + 0x26) == 0x2) || (*(int *)(iVar2 + uStack4 * 0xc + 0x26) == 0x1))
        {
            *(undefined2 *)(iVar2 + uStack4 * 0xc + 0x26) = 0x4;
            param_3                                       = uStack4;
        }
        else
        {
            uVar1 = uStack4;
            pass1_1028_b58e(param_1);
            iVar2 = uStack4 * 0xc + iVar2;
            pass1_1030_6e9c(CONCAT22(extraout_DX, uVar1), 0x1, *(int *)(iVar2 + 0x24));
            param_3                       = 0x0;
            *(undefined4 *)(iVar2 + 0x20) = 0x0;
            *(undefined2 *)(iVar2 + 0x24) = 0x0;
            *(undefined2 *)(iVar2 + 0x26) = 0x0;
        }
    }
    pass1_1028_b46e(param_1, param_2, param_3);
    return;
}


void __stdcall16far pass1_1030_cac2(ulong *param_1, int param_2, ushort param_3, ushort param_4, ushort param_5)

{
    undefined4  uVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    ulong       uVar4;
    uint        uVar5;
    ulong       uVar6;
    undefined4 *puVar7;
    ulong       uVar8;
    uint        extraout_DX;
    uint        extraout_DX_00;
    uint        extraout_DX_01;
    uint        uVar9;
    uint        uVar10;
    ulong       uStack34;
    int         iStack30;
    int         iStack28;

    pass1_1028_be9e(param_1, param_3, param_4, (ushort)&USHORT_1050_1028, param_5);
    uVar10 = (uint)((ulong)param_1 >> 0x10);
    if((*(int *)((int)param_1 + 0x12) == 0x5) && (PTR_LOOP_1050_5812 == (undefined *)0x0))
    {
        PTR_LOOP_1050_5812 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
        pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar10 << 0x10);
        uVar1  = *(undefined4 *)(param_2 + 0x2e);
        uVar6  = *(ulong *)((int)uVar1 + 0x10);
        uVar10 = extraout_DX;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar6, (uint)(uVar6 >> 0x10));
        puVar2  = (undefined4 *)*(undefined4 *)((int)uVar6 + 0x1e);
        ppcVar3 = (code **)((int)*puVar2 + 0x10);
        puVar7  = puVar2;
        (**ppcVar3)((int)&USHORT_1050_1028, (int)puVar2, *(undefined2 *)((int)uVar6 + 0x20));
        uVar4    = (ulong)puVar7 & 0xffff | (ulong)extraout_DX_00 << 0x10;
        iStack28 = 0x0;
        iStack30 = pass1_1030_d144((ulong)param_1);
        uStack34 = 0x0;
        while((uStack34 < uVar4 && (iStack30 != 0x0)))
        {
            ppcVar3 = (code **)((int)*puVar2 + 0x4);
            uVar8   = uVar4;
            (**ppcVar3)((int)&USHORT_1050_1028,
                        (int)puVar2,
                        (int)((ulong)puVar2 >> 0x10),
                        (char)uStack34,
                        (int)(uStack34 >> 0x10));
            uVar5 = (uint)uVar8;
            uVar9 = extraout_DX_01 | uVar5;
            if(uVar9 != 0x0)
            {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_01);
                uVar5 = *(uint *)(uVar5 + 0xc);
                if((0x0 < (int)uVar5) && (!SBORROW2(uVar5, 0x1)))
                {
                    if(uVar5 != 0x3 && 0x0 < (int)(uVar5 - 0x2))
                    {
                        if(uVar5 != 0x4)
                            goto LAB_1030_cbbc;
                        iStack28 = iStack28 + 0x1;
                    }
                    pass1_1030_6e9c(uVar6 & 0xffff | (ulong)uVar10 << 0x10, 0x1, uVar5);
                    pass1_1030_d180((ulong)param_1, uVar5);
                    iStack30 = iStack30 + -0x1;
                }
            }
        LAB_1030_cbbc:
            uStack34 = uStack34 + 0x1;
        }
        while(iStack28 < 0x4)
        {
            pass1_1030_d180((ulong)param_1, 0x4);
            iStack28 = iStack28 + 0x1;
        }
    }
    return;
}


ushort __stdcall16far pass1_1030_cbf0(int param_1, ushort param_2, int param_3)

{
    astruct_595 *iVar1;
    int          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return 0x0;
        }
        iVar1 = (astruct_595 *)(iStack4 * 0xc + param_1);
        if((iVar1->field_0x24 == param_3) && (iVar1->field_0x26 == 0x3))
            break;
        iStack4 = iStack4 + 0x1;
    }
    iVar1->field_0x26 = 0x0;
    iVar1->field_0x28 = 0x0;
    return 0x1;
}


void __stdcall16far pass1_1030_cc44(int param_1, ushort param_2, int param_3, ulong param_4, int param_5)

{
    code       **ppcVar1;
    int          iVar2;
    undefined   *puVar3;
    ushort       uVar4;
    uint         uVar5;
    uint         uVar6;
    uint         extraout_DX;
    undefined2   extraout_DX_00;
    uchar       *puVar7;
    uint         extraout_DX_01;
    astruct_304 *iVar7;
    astruct_303 *iVar8;
    undefined    uVar8;
    ushort       unaff_SS;
    undefined4  *puVar9;
    ulong       *puVar10;
    uchar       *puVar11;
    undefined    local_32[0x8];
    undefined4  *puStack42;
    ulong        uStack38;
    ulong        uStack34;
    undefined4  *puStack30;
    uint         uStack26;
    uchar       *puStack24;
    uint         uStack22;
    uchar       *puStack20;
    ulong       *puStack18;
    int          iStack14;
    undefined2   uStack12;
    int          iStack10;
    undefined4   uStack8;
    int          iStack4;

    iStack4  = 0x0;
    uStack8  = *(undefined4 *)((int)param_4 + 0x4);
    iStack10 = 0x0;
    do
    {
        if(0x9 < iStack10)
        {
            return;
        }
        iVar8 = (astruct_303 *)(iStack10 * 0xc + param_1);
        if(((iVar8->field_0x28 == (int)uStack8) && (iVar8->field_0x2a == uStack8._2_2_))
           && (iVar8->field_0x24 == param_5))
        {
            if(iVar8->field_0x26 == 0x4)
            {
                iVar2 = param_5;
                pass1_1028_b58e(CONCAT22(param_2, param_1));
                iStack14 = iVar2;
                uStack12 = extraout_DX_00;
                pass1_1030_6e9c(CONCAT13((char)((uint)extraout_DX_00 >> 0x8), CONCAT12((char)extraout_DX_00, iStack14)),
                                0x1,
                                iVar8->field_0x24);
                iVar8->field_0x20                 = 0x0;
                iVar8->field_0x24                 = 0x0;
                iVar8->field_0x26                 = 0x0;
                puStack42                         = (undefined4 *)0x0;
                puStack18                         = (ulong *)0x0;
                _DAT_0000_0006                    = param_5;
                uRam0000000a                      = 0x1;
                uVar4                             = switch_1020_c3b4(param_5);
                *(ushort *)((int)puStack18 + 0xc) = uVar4;
                puVar10                           = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
                puVar7                            = (uchar *)((ulong)puVar10 >> 0x10);
                uVar6                             = (uint)puVar10;
                uVar5                             = uVar6;
                puVar11                           = puVar7;
                uStack22                          = uVar6;
                puStack20                         = puVar7;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x1, 0x400);
                uVar8     = 0x38;
                uStack26  = uVar6;
                puStack24 = puVar7;
                pass1_1038_4d6e(CONCAT22(puVar7, uVar6), (ulong *)CONCAT22(puVar11, uVar5), uVar6, puVar7);
                puStack30 = (undefined4 *)CONCAT22(puVar7, uVar6);
                ppcVar1   = (code **)((int)*puStack30 + 0x10);
                (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar6, puVar7);
                uStack34 = CONCAT22(extraout_DX_01, uVar6);
                uVar6    = extraout_DX_01;
                for(uStack38 = 0x0; uStack38 < uStack34; uStack38 = uStack38 + 0x1)
                {
                    puVar9 = (undefined4 *)pass1_1030_1d7c((int)uStack34, uVar6, (ulong)puStack30);
                    uVar5  = (uint)((ulong)puVar9 >> 0x10);
                    uVar6  = uVar5 | (uint)puVar9;
                    if(uVar6 != 0x0)
                    {
                        puVar3  = local_32;
                        ppcVar1 = (code **)((int)*puVar9 + 0x40);
                        (**ppcVar1)(0x38, (char)puVar9, uVar5, puVar3);
                        uVar6 = extraout_DX;
                        if(puVar3 == (undefined *)0x0)
                        {
                            uVar8 = 0x28;
                            pass1_1028_6408((ulong)puVar9, puStack18, unaff_SS);
                            break;
                        }
                    }
                }
                puStack42 = puStack30;
                if(puStack30 != (undefined4 *)0x0)
                {
                    ppcVar1 = (code **)*puStack30;
                    (**ppcVar1)(uVar8, (int)puStack30, (int)((ulong)puStack30 >> 0x10), 0x1);
                }
            }
            else
            {
                iVar7             = (astruct_304 *)(iStack10 * 0xc + param_1);
                iVar7->field_0x26 = 0x0;
                iVar7->field_0x28 = 0x0;
            }
            iStack4 = iStack4 + 0x1;
            param_3 = param_3 + -0x1;
            if(param_3 == 0x0)
            {
                return;
            }
        }
        iStack10 = iStack10 + 0x1;
    } while(true);
}


BOOL16 __stdcall16far pass1_1030_ad22(
  ushort param_1, ushort param_2, ushort *param_3, long param_4, uint param_5, uint param_6, ushort param_7)

{
    BOOL16 BVar1;
    uint   uVar2;
    ulong  uVar3;

    pass1_1030_627e(param_7, param_5, param_6, _PTR_LOOP_1050_5740, param_3, param_4);
    uVar2 = param_6 | param_5;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, param_6);
        if((uVar2 | param_5) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_5));
            if(uVar3 != 0x0)
            {
                BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((int)uVar3 + 0xc), 0x46);
                return BVar1;
            }
        }
    }
    return 0x0;
}


void __stdcall16far
pass1_1030_ad86(ushort param_1, ushort param_2, ushort *param_3, long param_4, ushort param_5, ushort param_6)

{
    undefined4  uVar1;
    undefined4 *puVar2;
    char        cStack17;
    undefined4  local_a;
    int         iStack6;

    puVar2 = &local_a;
    pass1_1030_64ce(
      param_5, puVar2, param_6, _PTR_LOOP_1050_5740, param_3, param_4, (ulong *)CONCAT22(param_5, puVar2));
    uVar1    = *puVar2;
    cStack17 = (char)((ulong)uVar1 >> 0x18);
    if(cStack17 == '\0')
    {
        iStack6 = (int)uVar1;
        if(((0x0 < iStack6) && (!SBORROW2(iStack6, 0x1)))
           && ((iStack6 == 0x3 || iStack6 + -0x2 < 0x1 || ((0x3 < iStack6 + -0x3 && (iStack6 + -0x7 < 0x2))))))
        {
            return;
        }
    }
    return;
}


ushort __stdcall16far pass1_1030_addc(ushort  param_1,
                                      ushort  param_2,
                                      ushort *param_3,
                                      ushort  param_4,
                                      ushort  param_5,
                                      ulong   param_6,
                                      int     param_7,
                                      ushort  param_8,
                                      ushort  param_9)

{
    undefined4 *puVar1;
    int         local_14;
    int         local_12;
    int         local_10;
    int         local_e;
    undefined4  local_c;
    undefined2  uStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_6, (uint)(param_6 >> 0x10));
    iStack6 = param_7;
    uStack4 = param_8;
    puVar1  = (undefined4 *)pass1_1030_5b5c(param_7, param_8);
    local_c = *puVar1;
    uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
    pass1_1008_3e94(param_3, (ushort *)CONCAT22(param_9, &local_10), (ushort *)CONCAT22(param_9, &local_e));
    pass1_1008_3e94((ushort *)CONCAT22(param_9, &local_c),
                    (ushort *)CONCAT22(param_9, &local_14),
                    (ushort *)CONCAT22(param_9, &local_12));
    if((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) && (local_10 < local_14 + -0x1))
    {
        return 0x1;
    }
    return 0x0;
}


void __stdcall16far pass1_1030_b13c(void)

{
    return;
}


void __stdcall16far pass1_1030_b142(ulong param_1, ulong param_2)

{
    int        iVar1;
    int        iVar2;
    undefined2 uVar3;
    bool       bVar4;
    ulong      uVar5;
    undefined4 uStack12;

    uVar5    = struct_op_1030_73a8(param_2);
    uVar3    = (undefined2)(uVar5 >> 0x10);
    iVar1    = (int)uVar5;
    iVar2    = *(int *)(iVar1 + 0xc);
    uStack12 = 0x0;
    if(iVar2 == 0x18)
    {
        uStack12._2_2_ = pass1_1028_1c1c();
        uVar3          = *(undefined2 *)(iVar1 + 0x22);
    }
    else
    {
        if(iVar2 != 0x3f)
            goto LAB_1030_b1a6;
        uStack12._2_2_ = pass1_1028_20b0();
        uVar3          = *(undefined2 *)(iVar1 + 0x24);
    }
    uStack12 = CONCAT22(uStack12._2_2_, uVar3);
LAB_1030_b1a6:
    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(int *)(iVar2 + 0xe) == 0x1)
    {
        bVar4 = (uStack12 & 0x10000) == 0x0;
    }
    else
    {
        if(*(int *)(iVar2 + 0xe) == 0x2)
        {
            bVar4 = (uStack12 & 0x20000) == 0x0;
        }
        else
        {
            if(*(int *)(iVar2 + 0xe) == 0x3)
            {
                bVar4 = (uStack12 & 0x40000) == 0x0;
            }
            else
            {
                bVar4 = (uStack12 & 0x80000) == 0x0;
            }
        }
    }
    if((bVar4) || ((int)uStack12 != 0x0))
    {
        bVar4 = false;
        while(true)
        {
            if(((uStack12 & 0x10000) != 0x0) && ((int)uStack12 == 0x0))
                goto LAB_1030_b239;
            if(((uStack12 & 0x20000) != 0x0) && ((int)uStack12 == 0x0))
                goto LAB_1030_b247;
            if(((uStack12 & 0x40000) != 0x0) && ((int)uStack12 == 0x0))
                goto LAB_1030_b255;
            if(((uStack12 & 0x80000) != 0x0) && ((int)uStack12 == 0x0))
                goto LAB_1030_b263;
            if(bVar4)
                break;
            uStack12._1_3_ = (uint3)(uStack12 >> 0x8) & 0xffff00;
            iVar1          = *(int *)(iVar2 + 0xe);
            if(iVar1 == 0x1)
            {
                uStack12 = CONCAT31(uStack12._1_3_, 0x4);
            }
            else
            {
                if(iVar1 == 0x2)
                {
                    uStack12 = CONCAT31(uStack12._1_3_, 0x8);
                }
                else
                {
                    if(iVar1 == 0x3)
                    {
                        uStack12 = CONCAT31(uStack12._1_3_, 0x1);
                    }
                    else
                    {
                        uStack12 = CONCAT31(uStack12._1_3_, 0x2);
                    }
                }
            }
            bVar4 = true;
        }
        if(*(int *)(iVar2 + 0xe) == 0x1)
        {
        LAB_1030_b255:
            *(undefined2 *)(iVar2 + 0xe) = 0x3;
            return;
        }
        if(*(int *)(iVar2 + 0xe) == 0x2)
        {
        LAB_1030_b263:
            *(undefined2 *)(iVar2 + 0xe) = 0x4;
            return;
        }
        if(*(int *)(iVar2 + 0xe) == 0x3)
        {
        LAB_1030_b239:
            *(undefined2 *)(iVar2 + 0xe) = 0x1;
            return;
        }
        if(*(int *)(iVar2 + 0xe) == 0x4)
        {
        LAB_1030_b247:
            *(undefined2 *)(iVar2 + 0xe) = 0x2;
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1030_b9b2(ulong param_1)

{
    undefined2 uVar1;

    uVar1                                = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0xe)  = 0x0;
    *(undefined4 *)((int)param_1 + 0x12) = 0x0;
    return;
}


void __stdcall16far pass1_1030_b9da(
  ulong param_1, ulong param_2, ulong param_3, ulong param_4, ushort param_5, ushort param_6, ushort param_7)

{
    ulong       *puVar1;
    ulong        uVar2;
    uchar       *puVar3;
    uint         uVar4;
    uint         uVar5;
    ulong        uVar6;
    astruct_402 *iVar7;
    int          iVar8;
    undefined2   uVar9;
    ulong        uVar10;
    uint         uStack12;
    uint         uStack4;

    puVar3 = (uchar *)param_3;
    uVar9  = (undefined2)(param_1 >> 0x10);
    iVar7  = (astruct_402 *)param_1;
    if(iVar7->field_0xe == (long *)0x0)
    {
        mem_op_1000_179c(0xa, puVar3, 0x1000);
        uVar4   = (uint)puVar3 | (uint)param_4;
        param_3 = (ulong)uVar4;
        if(uVar4 == 0x0)
        {
            iVar7->field_0xe = (long *)0x0;
        }
        else
        {
            pass1_1020_ba3e((long *)(param_4 & 0xffff | ZEXT24(puVar3) << 0x10), 0x5, 0x5, param_6, param_5);
            *(int *)&iVar7->field_0xe                     = (int)param_4;
            *(undefined2 *)((int)&iVar7->field_0xe + 0x2) = (int)param_3;
        }
        iVar7->field_0x12 = 0x0;
    }
    for(uStack4 = 0x4; (int)uStack4 < 0xe; uStack4 = uStack4 + 0x1)
    {
        uVar10  = pass1_1030_7c28(param_2, uStack4, (uint)param_4, (uint)param_3, param_7);
        uVar4   = (uint)(uVar10 >> 0x10);
        param_4 = uVar10 & 0xffff;
        uVar5   = uVar4 | (uint)param_4;
        param_3 = (ulong)uVar5;
        if(uVar5 != 0x0)
        {
            uVar2    = 0x64 - iVar7->field_0x12;
            uVar6    = uVar2 >> 0x10;
            uStack12 = (uint)uVar10;
            if(uVar10 < uVar2)
            {
                uVar2 = uVar10 & 0xffff;
                uVar6 = (ulong)uVar4;
            }
            uVar5   = (uint)uVar2;
            param_4 = uVar2 & 0xffff | uVar6 << 0x10;
            iVar8   = (uVar4 - (int)uVar6) - (uint)(uStack12 < uVar5);
            param_3 = uVar6;
            pass1_1030_7d1c(
              param_2, uStack12 - uVar5, CONCAT22(uStack4, iVar8), uVar5, (int)uVar6, iVar8, param_6, param_7);
            pass1_1020_bb8a(iVar7->field_0xe, uVar5, uVar6 | (ulong)uStack4 << 0x10, param_6, param_7);
            puVar1  = &iVar7->field_0x12;
            *puVar1 = *puVar1 + param_4;
            string_1020_c0ca(uStack4);
            vsprintf_op_1030_840a(
              (ulong)s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c, 0x1020, param_7, (ushort)param_3);
            if(0x63 < iVar7->field_0x12)
                break;
        }
    }
    if(iVar7->field_0x12 != 0x0)
    {
        return;
    }
    return;
}


ushort *__stdcall16far pass1_1030_9e9c(ushort *param_1, byte param_2)

{
    ushort uVar1;

    uVar1                 = (ushort)((ulong)param_1 >> 0x10);
    *param_1              = 0x389a;
    ((int *)param_1)[0x1] = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        pass1_1000_093a((int *)param_1, uVar1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1030_9ecc(ulong *param_1, ulong param_2)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0x0;
    *(ulong *)((int)param_1 + 0x4)      = param_2;
    *(undefined2 *)((int)param_1 + 0x8) = 0x0;
    return;
}


void __stdcall16far pass1_1030_9f40(ulong param_1, ushort param_2, ushort param_3, uchar param_4)

{
    ushort uVar1;

    uVar1 = pass1_1008_c646(
      (ushort)_PTR_LOOP_1050_06e0, CONCAT22(param_2, (int)((ulong)_PTR_LOOP_1050_06e0 >> 0x10)), param_3);
    *(ushort *)((int)param_1 + 0x8) = uVar1;
    pass1_1030_9f7a((ushort *)param_1, uVar1, param_3, param_4);
    return;
}


void __stdcall16far pass1_1030_9f64(ulong *param_1)

{
    *(undefined2 *)((int)param_1 + 0x8) = 0x0;
    *param_1                            = 0x0;
    return;
}


void __stdcall16far pass1_1030_a39a(ulong param_1, ushort *param_2, ushort param_3)

{
    pass1_1030_aa18(param_1, param_2, param_3);
    return;
}


void __stdcall16far pass1_1030_a3ae(ulong param_1, ushort *param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4  uVar2;
    uint        uVar3;
    ushort      uVar4;
    BOOL16      BVar5;
    ulong       uVar6;
    uchar      *puVar7;
    undefined2  extraout_DX;
    ushort      uVar8;
    ushort      uVar9;
    ushort      uVar10;
    int         iVar11;
    ushort      uVar12;
    undefined2  uVar13;
    ulong      *puVar14;
    ushort     *puVar15;
    uint        uVar16;
    ulong       uStack44;
    int         local_28;
    int         local_26;
    int         local_24;
    undefined   local_22[0x6];
    int         local_1c;
    int         iStack26;
    long        lStack22;
    ulong       uStack18;
    undefined4 *puStack14;
    uint        uStack10;
    uchar      *puStack8;
    int         iStack6;
    undefined2  uStack4;

    uStack4  = 0x0;
    iStack6  = *(int *)((int)param_2 + 0x4);
    puVar14  = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x45);
    puVar7   = (uchar *)((ulong)puVar14 >> 0x10);
    uVar3    = (uint)puVar14;
    uVar12   = (ushort)(param_1 >> 0x10);
    uVar10   = (ushort)param_1;
    uStack10 = uVar3;
    puStack8 = puVar7;
    pass1_1038_4e78(uVar3, puVar7, *(ulong *)(uVar10 + 0x4), puVar14);
    puStack14 = (undefined4 *)CONCAT22(puVar7, uVar3);
    ppcVar1   = (code **)((int)*puStack14 + 0x10);
    uVar16    = uVar3;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar3, puVar7);
    uStack18 = CONCAT22(extraout_DX, uVar3);
    uVar2    = *(undefined4 *)(uVar10 + 0x4);
    lStack22 = *(long *)((int)uVar2 + 0x8);
    pass1_1008_3e38((ushort *)CONCAT22(param_3, &local_1c));
    puVar15  = pass1_1008_3e38((ushort *)CONCAT22(param_3, local_22));
    uStack44 = 0x0;
    uVar8    = (ushort)((ulong)puVar15 >> 0x10);
    do
    {
        if(uStack18 <= uStack44)
        {
        LAB_1030_a4e7:
            if(puStack14 != (undefined4 *)0x0)
            {
                ppcVar1 = (code **)*puStack14;
                (**ppcVar1)(
                  0x1008, (int)puStack14, (char)((ulong)puStack14 >> 0x10), 0x1, uVar16, puVar7, puStack14, puStack14);
            }
            return;
        }
        uVar6 = uStack18;
        pass1_1030_1d58((ulong)puStack14);
        uVar9 = uVar8 | (uint)uVar6;
        if(uVar9 != 0x0)
        {
            pass1_1008_3f62((ushort *)CONCAT22(param_3, &local_1c), (ushort *)CONCAT22(uVar8, (uint)uVar6 + 0xc));
            pass1_1008_3eb4((ushort *)CONCAT22(param_3, &local_1c),
                            (ushort *)CONCAT22(param_3, &local_28),
                            (ushort *)CONCAT22(param_3, &local_26),
                            (ushort *)CONCAT22(param_3, &local_24));
            uVar9 = uVar8;
            if((local_28 == iStack6)
               && (uVar2  = *(undefined4 *)(uVar10 + 0x4),
                   uVar13 = (undefined2)((ulong)uVar2 >> 0x10),
                   iVar11 = (int)uVar2,
                   uVar2  = *(undefined4 *)(iVar11 + 0x4),
                   uVar4  = pass1_1030_addc(uVar10,
                                           uVar12,
                                           (ushort *)CONCAT22(param_3, &local_1c),
                                           (ushort)uVar2,
                                           (ushort)((ulong)uVar2 >> 0x10),
                                           *(ulong *)(iVar11 + 0x8),
                                           (int)&local_1c,
                                           uVar8,
                                           param_3),
                   uVar9  = uVar8,
                   uVar4 != 0x0))
            {
                pass1_1008_3f62((ushort *)CONCAT22(param_3, local_22), (ushort *)CONCAT22(param_3, &local_1c));
                iStack26 = local_26 + -0x1;
                BVar5    = pass1_1030_ad22(
                  uVar10, uVar12, (ushort *)CONCAT22(param_3, &local_1c), lStack22, &local_1c, uVar8, param_3);
                if(BVar5 == 0x0)
                {
                    iStack26 = local_26 + 0x1;
                    BVar5    = pass1_1030_ad22(
                      uVar10, uVar12, (ushort *)CONCAT22(param_3, &local_1c), lStack22, &local_1c, uVar8, param_3);
                    if(BVar5 == 0x0)
                    {
                        iStack26 = local_26;
                        local_1c = local_24 + -0x1;
                        BVar5    = pass1_1030_ad22(
                          uVar10, uVar12, (ushort *)CONCAT22(param_3, &local_1c), lStack22, &local_1c, uVar8, param_3);
                        if(BVar5 == 0x0)
                        {
                            local_1c = local_24 + 0x1;
                            BVar5    = pass1_1030_ad22(uVar10,
                                                    uVar12,
                                                    (ushort *)CONCAT22(param_3, &local_1c),
                                                    lStack22,
                                                    &local_1c,
                                                    uVar8,
                                                    param_3);
                            uVar9    = uVar8;
                            if(BVar5 == 0x0)
                                goto LAB_1030_a45b;
                        }
                    }
                }
                pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_3, local_22));
                uStack4 = 0x1;
                goto LAB_1030_a4e7;
            }
        }
    LAB_1030_a45b:
        uStack44 = uStack44 + 0x1;
        uVar8    = uVar9;
    } while(true);
}


void __stdcall16far pass1_1030_a57e(ulong param_1, ushort *param_2, int param_3, int param_4, ushort param_5)

{
    ulong       uVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    ushort      uVar4;
    int        *piVar5;
    ulong       uVar6;
    uchar      *puVar7;
    uint        extraout_DX;
    uint        uVar8;
    ushort      uVar9;
    ushort      uVar10;
    undefined2  uVar11;
    int         iVar12;
    undefined4 *puVar13;
    ushort      uVar14;
    undefined2  uVar15;
    undefined2  uVar16;
    undefined2  uVar17;
    ulong      *puVar18;
    ulong       uVar19;
    undefined   uVar20;
    ulong       uStack40;
    undefined   local_1c[0x2];
    int         local_1a;
    int         local_18;
    int         local_16;
    int         iStack20;
    undefined4  uStack16;
    uint        uStack12;
    uchar      *puStack10;
    int         iStack8;
    int         iStack6;
    undefined2  uStack4;

    uStack4 = 0x0;
    uVar14  = (ushort)(param_1 >> 0x10);
    uVar10  = (ushort)param_1;
    pass1_1038_53ba(*(ulong *)(uVar10 + 0x4), 0x1);
    if((param_4 != 0x0) || (param_3 != 0x0))
    {
        iStack6   = *(int *)((int)param_2 + 0x4);
        iStack8   = 0x8 - (uint)(iStack6 == 0x0);
        puVar18   = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, iStack8);
        puVar7    = (uchar *)((ulong)puVar18 >> 0x10);
        uVar8     = (uint)puVar18;
        uStack12  = uVar8;
        puStack10 = puVar7;
        pass1_1038_4e78(uVar8, puVar7, *(ulong *)(uVar10 + 0x4), puVar18);
        uStack16 = (undefined4 *)CONCAT22(puVar7, uVar8);
        uVar17   = 0x1008;
        pass1_1008_3e38((ushort *)CONCAT22(param_5, &local_16));
        uVar3   = *(undefined4 *)(uVar10 + 0x4);
        uVar1   = *(ulong *)((int)uVar3 + 0x8);
        uVar15  = (undefined2)((ulong)uStack16 >> 0x10);
        uVar11  = SUB42(uStack16, 0x0);
        ppcVar2 = (code **)((int)*uStack16 + 0x10);
        uVar6   = uVar1;
        (**ppcVar2)(0x1008, uVar11, uVar15);
        uVar6 = uVar6 & 0xffff | (ulong)extraout_DX << 0x10;
        uVar8 = extraout_DX;
        for(uStack40 = 0x0; uStack40 < uVar6; uStack40 = uStack40 + 0x1)
        {
            uVar19 = uVar6;
            pass1_1030_1d58((ulong)uStack16);
            uVar9 = uVar8 | (uint)uVar19;
            if(uVar9 != 0x0)
            {
                uVar9 = uVar8;
                pass1_1008_3f62((ushort *)CONCAT22(param_5, &local_16), (ushort *)CONCAT22(uVar8, (uint)uVar19 + 0xc));
                uVar17 = 0x1008;
                pass1_1008_3eb4((ushort *)CONCAT22(param_5, &local_16),
                                (ushort *)CONCAT22(param_5, local_1c),
                                (ushort *)CONCAT22(param_5, &local_1a),
                                (ushort *)CONCAT22(param_5, &local_18));
                uVar3  = *(undefined4 *)(uVar10 + 0x4);
                uVar16 = (undefined2)((ulong)uVar3 >> 0x10);
                iVar12 = (int)uVar3;
                uVar3  = *(undefined4 *)(iVar12 + 0x4);
                uVar4  = pass1_1030_addc(uVar10,
                                        uVar14,
                                        (ushort *)CONCAT22(param_5, &local_16),
                                        (ushort)uVar3,
                                        (ushort)((ulong)uVar3 >> 0x10),
                                        *(ulong *)(iVar12 + 0x8),
                                        (int)&local_16,
                                        uVar9,
                                        param_5);
                if(uVar4 == 0x0)
                    goto LAB_1030_a660;
                uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | (ulong)uVar8 << 0x10);
                uVar9  = (ushort)(uVar19 >> 0x10);
                iVar12 = *(int *)((int)uVar19 + 0xc);
                if(0x5 < iVar12 - 0x7aU)
                    goto LAB_1030_a660;
                uVar17 = 0x1030;
                switch(iVar12)
                {
                default:
                    iStack20 = local_1a + -0x1;
                    piVar5   = &local_16;
                    pass1_1030_ad86(uVar10, uVar14, (ushort *)CONCAT22(param_5, piVar5), uVar1, param_5, uVar9);
                    if(piVar5 != (int *)0x0)
                        goto LAB_1030_a7df;
                    iStack20 = local_1a + 0x1;
                    piVar5   = &local_16;
                    pass1_1030_ad86(uVar10, uVar14, (ushort *)CONCAT22(param_5, piVar5), uVar1, param_5, uVar9);
                    if(piVar5 == (int *)0x0)
                    {
                        iStack20 = local_1a;
                        local_16 = local_18 + -0x1;
                        piVar5   = &local_16;
                        pass1_1030_ad86(uVar10, uVar14, (ushort *)CONCAT22(param_5, piVar5), uVar1, param_5, uVar9);
                        goto joined_r0x1030a722;
                    }
                LAB_1030_a748:
                    pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_5, &local_16));
                    break;
                case 0x7b:
                case 0x7e:
                    iStack20 = local_1a + -0x1;
                    piVar5   = &local_16;
                    pass1_1030_ad86(uVar10, uVar14, (ushort *)CONCAT22(param_5, piVar5), uVar1, param_5, uVar9);
                    if(piVar5 == (int *)0x0)
                    {
                        iStack20 = local_1a + 0x1;
                        goto LAB_1030_a730;
                    }
                    pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_5, &local_16));
                    if(uStack16 == (undefined4 *)0x0)
                    {
                        return;
                    }
                    uVar17  = (undefined2)((ulong)uStack16 >> 0x10);
                    puVar13 = (undefined4 *)uStack16;
                    uVar20  = (undefined)((ulong)uStack16 >> 0x10);
                    goto LAB_1030_a6ea;
                case 0x7c:
                case 0x7d:
                    local_16 = local_18 + -0x1;
                    piVar5   = &local_16;
                    pass1_1030_ad86(uVar10, uVar14, (ushort *)CONCAT22(param_5, piVar5), uVar1, param_5, uVar9);
                joined_r0x1030a722:
                    if(piVar5 == (int *)0x0)
                    {
                        local_16 = local_18 + 0x1;
                    LAB_1030_a730:
                        piVar5 = &local_16;
                        pass1_1030_ad86(uVar10, uVar14, (ushort *)CONCAT22(param_5, piVar5), uVar1, param_5, uVar9);
                        if(piVar5 != (int *)0x0)
                            goto LAB_1030_a748;
                        goto LAB_1030_a660;
                    }
                LAB_1030_a7df:
                    pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_5, &local_16));
                }
                puVar13 = (undefined4 *)uStack16;
                if((uStack16._2_2_ | (uint)puVar13) != 0x0)
                {
                    uVar17 = (undefined2)((ulong)uStack16 >> 0x10);
                    uVar20 = (undefined)((ulong)uStack16 >> 0x10);
                LAB_1030_a6ea:
                    ppcVar2 = (code **)*puVar13;
                    (**ppcVar2)(0x1008, puVar13, uVar20, 0x1, uVar11, uVar15, uStack16, uStack16);
                }
                return;
            }
        LAB_1030_a660:
            uVar8 = uVar9;
        }
        if(uStack16 != (undefined4 *)0x0)
        {
            ppcVar2 = (code **)*uStack16;
            (**ppcVar2)(
              uVar17, (int)uStack16, (char)((ulong)uStack16 >> 0x10), 0x1, uVar11, uVar15, uStack16, uStack16);
        }
    }
    return;
}


void __stdcall16far pass1_1030_8aa0(ulong param_1, ulong param_2, ushort *param_3, uint param_4, ushort param_5)

{
    uint       uVar1;
    int        unaff_DI;
    ulong      local_12;
    undefined *puStack14;
    ulong      uStack12;
    undefined  local_8[0x2];
    undefined  local_6[0x2];
    undefined  local_4[0x2];

    puStack14 = local_8;
    pass1_1008_3eb4(param_3,
                    (ushort *)CONCAT13((char)(param_5 >> 0x8), CONCAT12((char)param_5, puStack14)),
                    (ushort *)CONCAT22(param_5, local_6),
                    (ushort *)CONCAT22(param_5, local_4));
    bad_1030_8cd2();
    uStack12 = CONCAT22(param_4, puStack14);
    uVar1    = param_4 | (uint)puStack14;
    if(uVar1 != 0x0)
    {
        pass1_1030_8d9e(param_1, param_5);
        local_12 = param_2;
        pass1_1030_8660(uStack12,
                        (ulong *)CONCAT22(param_5, &local_12),
                        (ushort)puStack14,
                        (uint)&local_12,
                        uVar1,
                        param_5,
                        unaff_DI);
    }
    return;
}


void __stdcall16far pass1_1030_8b00(ulong param_1, ushort *param_2, ushort *param_3, ushort param_4)

{
    undefined4 *puVar1;
    int        *piVar2;
    ushort      uVar3;
    undefined4  local_2a;
    undefined4  uStack38;
    undefined4  uStack28;
    undefined4 *puStack18;
    undefined4 *puStack16;
    int        *piStack14;
    int         local_c;
    undefined   local_a[0x4];
    undefined4  uStack6;

    uStack6 = 0x0;
    puVar1  = (undefined4 *)(local_a + 0x2);
    piVar2  = &local_c;
    pass1_1008_3eb4(param_2,
                    (ushort *)CONCAT13((char)(param_4 >> 0x8), CONCAT12((char)param_4, piVar2)),
                    (ushort *)CONCAT22(param_4, local_a),
                    (ushort *)CONCAT22(param_4, puVar1));
    bad_1030_8cd2();
    puStack16 = puVar1;
    piStack14 = piVar2;
    pass1_1030_8d9e(param_1, param_4);
    puStack18 = puVar1;
    pass1_1030_861a((ushort)puStack16, (ushort)piStack14, (ushort)puVar1, (uint)puVar1, (uint)piVar2, param_4);
    uStack38       = *puVar1;
    uVar3          = *(ushort *)((int)puVar1 + 0x2);
    uStack38._3_1_ = (char)((ulong)uStack38 >> 0x18);
    uStack6        = uStack38;
    if(uStack38._3_1_ == '\0')
    {
        puVar1   = &local_2a;
        uStack28 = uStack38;
        pass1_1030_8c66(param_1,
                        local_c,
                        (byte *)local_a,
                        (ushort)((ulong)local_a >> 0x10),
                        (ulong *)CONCAT22(param_4, puVar1),
                        uVar3);
        uStack6 = *puVar1;
        uVar3   = *(ushort *)((int)puVar1 + 0x2);
    }
    *param_3                        = (ushort)uStack6;
    *(ushort *)((int)param_3 + 0x2) = uVar3;
    return;
}


void __stdcall16far pass1_1030_8bac(ulong param_1, ushort param_2)

{
    int iStack4;

    iStack4 = 0x0;
    do
    {
        pass1_1030_86ec(*(undefined4 *)((int)param_1 + 0x38 + iStack4 * 0x4), param_2);
        iStack4 = iStack4 + 0x1;
    } while(iStack4 < 0x5);
    return;
}


void __stdcall16far pass1_1030_8bdc(ulong param_1, ulong param_2, ushort *param_3, int param_4, ushort param_5)

{
    undefined *puVar1;
    ulong      local_12;
    undefined *puStack14;
    long      *plStack12;
    undefined  local_8[0x2];
    undefined  local_6[0x2];
    undefined  local_4[0x2];

    puStack14 = local_4;
    puVar1    = local_8;
    pass1_1008_3eb4(param_3,
                    (ushort *)CONCAT13((char)(param_5 >> 0x8), CONCAT12((char)param_5, puVar1)),
                    (ushort *)CONCAT22(param_5, local_6),
                    (ushort *)CONCAT22(param_5, puStack14));
    bad_1030_8cd2();
    plStack12 = (long *)CONCAT22(puVar1, puStack14);
    pass1_1030_8d9e(param_1, param_5);
    local_12 = param_2;
    pass1_1030_871e(plStack12, (ulong *)CONCAT22(param_5, &local_12), (ushort)puStack14, param_4, param_5);
    return;
}


void __stdcall16far pass1_1030_8c38(ulong param_1, int param_2, ushort param_3)

{
    int iStack4;

    iStack4 = 0x0;
    do
    {
        pass1_1030_877c(*(uint **)((int)param_1 + 0x38 + iStack4 * 0x4), param_2, param_3);
        iStack4 = iStack4 + 0x1;
    } while(iStack4 < 0x5);
    return;
}


void __stdcall16far
pass1_1030_8c66(ulong param_1, int param_2, byte *param_3, ushort param_4, ulong *param_5, ushort param_6)

{
    byte  bVar1;
    uint  uVar2;
    ulong uStack6;

    pass1_1008_4544(*(ulong *)((int)param_1 + 0x12));
    bVar1   = *param_3;
    uVar2   = (uint)bVar1;
    uStack6 = (ulong)(uVar2 + 0x1);
    if(0x0 < param_2)
    {
        if(uVar2 == 0x0)
        {
            uStack6 = 0x7;
        }
        else
        {
            if(((bVar1 == 0x0) || (SBORROW2(uVar2, 0x1))) || (0x1 < (int)(uVar2 - 0x1)))
            {
                uStack6 = 0x9;
            }
            else
            {
                uStack6 = 0x8;
            }
        }
    }
    *param_5 = uStack6;
    return;
}
