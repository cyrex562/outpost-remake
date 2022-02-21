
void __stdcall16far pass1_1038_3698(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int       *piVar1;
    uint      *puVar2;
    undefined4 uVar3;
    code     **ppcVar4;
    undefined2 uVar5;
    BOOL16     BVar6;
    ushort     uVar7;
    uint       uVar8;
    long       lVar9;
    ulong      uVar10;
    uint       uVar11;
    undefined2 uVar12;
    uint       uVar13;
    ulong      uVar14;
    int        iVar15;
    undefined2 uVar16;
    ulong      uVar17;
    ulong      uStack32;
    ulong      uStack18;
    ulong      uStack14;
    undefined4 uStack10;
    undefined4 uStack6;

    uVar16 = (undefined2)(param_1 >> 0x10);
    iVar15 = (int)param_1;
    if(*(int *)(iVar15 + 0x214) == 0x0)
    {
        return;
    }
    pass1_1030_38b8();
    uStack6 = CONCAT22(param_3, param_2);
    uStack6 = uStack6 - *(long *)(iVar15 + 0x216);
    if(0x0 < uStack6)
    {
        uStack6  = uStack6 + 0x3;
        uStack10 = uStack6 / 0x5;
        uVar14   = uStack6 % 0x5;
        if(*(long *)(iVar15 + 0xc) == 0x0)
        {
            uVar5  = 0x0;
            uVar14 = 0x0;
        }
        else
        {
            uVar3   = *(undefined4 *)(iVar15 + 0xc);
            ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0xc) + 0x10);
            lVar9   = uStack10;
            (**ppcVar4)(0x1030, (int)uVar3, (int)((ulong)uVar3 >> 0x10));
            uVar5 = (undefined2)lVar9;
        }
        uStack14 = CONCAT22((int)uVar14, uVar5);
        for(uStack18 = 0x0; uVar12 = (undefined2)uVar14, uVar10 = uStack14, uStack18 < uStack14;
            uStack18 = uStack18 + 0x1)
        {
            uVar17 = pass1_1030_1d7c(uVar5, uVar12, *(ulong *)(iVar15 + 0xc));
            uVar8  = (uint)(uVar17 >> 0x10);
            uVar13 = uVar8 | (uint)uVar17;
            uVar14 = (ulong)uVar13;
            if(uVar13 != 0x0)
            {
                BVar6  = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((uint)uVar17 + 0xc), 0x4);
                uVar8  = (uint)uVar14;
                uVar10 = (ulong)BVar6;
                if(BVar6 != 0x0)
                {
                    uVar7    = pass1_1028_678c(uVar17, 0xf, param_4);
                    uStack32 = CONCAT22(uVar8, uVar7);
                    uVar14   = (ulong)(uVar8 | uVar7);
                    uVar10   = (ulong)uVar7;
                    if((uVar8 | uVar7) != 0x0)
                    {
                        if(uStack10 < (long)uStack32)
                        {
                            uVar8 = (uint)uStack10;
                            pass1_1028_6356(uVar17, 0xf, uVar8, uStack10._2_2_, param_4);
                            uVar13 = uVar8 * 0x5;
                            uVar11 = uStack10._2_2_ * 0x5 + (uint)CARRY2(uVar8, uVar8) * 0x2
                                   + (uint)CARRY2(uVar8 * 0x2, uVar8 * 0x2) + (uint)CARRY2(uVar8 * 0x4, uVar8);
                            uVar14   = (ulong)uVar11;
                            puVar2   = (uint *)(iVar15 + 0x216);
                            uVar8    = *puVar2;
                            *puVar2  = *puVar2 + uVar13;
                            piVar1   = (int *)(iVar15 + 0x218);
                            *piVar1  = *piVar1 + uVar11 + (uint)CARRY2(uVar8, uVar13);
                            uStack10 = 0x0;
                            uVar10   = (ulong)uVar13;
                        }
                        else
                        {
                            pass1_1028_6356(uVar17, 0xf, uVar7, uVar8, param_4);
                            uVar13 = uVar8 * 0x5 + (uint)CARRY2(uVar7, uVar7) * 0x2
                                   + (uint)CARRY2(uVar7 * 0x2, uVar7 * 0x2) + (uint)CARRY2(uVar7 * 0x4, uVar7);
                            uVar14   = (ulong)uVar13;
                            puVar2   = (uint *)(iVar15 + 0x216);
                            uVar8    = *puVar2;
                            *puVar2  = *puVar2 + uVar7 * 0x5;
                            piVar1   = (int *)(iVar15 + 0x218);
                            *piVar1  = *piVar1 + uVar13 + (uint)CARRY2(uVar8, uVar7 * 0x5);
                            uStack10 = uStack10 - uStack32;
                            uVar10   = uStack32;
                        }
                    }
                }
                uVar12 = (undefined2)uVar14;
                if(uStack10 == 0x0)
                    break;
            }
        }
        uVar5 = (undefined2)uVar10;
        pass1_1030_38b8();
        uStack6       = CONCAT22(uVar12, uVar5);
        uStack6       = uStack6 - *(long *)(iVar15 + 0x216);
        uStack6._2_2_ = (uint)((ulong)uStack6 >> 0x10);
        if((uStack6._2_2_ | (uint)uStack6) != 0x0)
        {
            uStack32 = uStack6 / (long)*(int *)(iVar15 + 0x214);
            if((long)uStack32 < 0x1)
            {
                uStack32 = 0x1;
            }
            pass1_1030_375a(*(ulong *)(iVar15 + 0x1f6), 0x0, uStack32, param_4);
        }
    }
    piVar1  = (int *)(iVar15 + 0x214);
    *piVar1 = *piVar1 + -0x1;
    return;
}


void __stdcall16far pass1_1038_387e(ulong param_1, int param_2, int param_3, ulong param_4, uint param_5)

{
    code       **ppcVar1;
    long         lVar2;
    ushort       uVar3;
    int          iVar4;
    ulong        uVar5;
    ulong        uVar6;
    ulong        uVar7;
    uchar       *extraout_DX;
    uchar       *puVar8;
    uchar       *puVar9;
    uint         uVar10;
    uint         extraout_DX_00;
    uint         uVar11;
    astruct_302 *iVar10;
    undefined2   uVar12;
    int          iStack22;
    uint         uStack12;
    ulong        uStack10;
    ulong        uStack6;

    if(param_2 != param_3)
    {
        iVar10 = (astruct_302 *)param_1;
        uVar12 = (undefined2)(param_1 >> 0x10);
        if(param_2 < param_3)
        {
            uStack12 = param_3 - param_2;
            if((iVar10->field_0x210 == 0x0) || (lVar2 = iVar10->field_0x210, *(long *)((int)lVar2 + 0xa) == 0x0))
            {
                if(iVar10->field_0xc == (undefined4 *)0x0)
                {
                    uVar11 = 0x0;
                    puVar8 = (uchar *)0x0;
                }
                else
                {
                    ppcVar1 = (code **)((int)*iVar10->field_0xc + 0x10);
                    uVar11  = uStack12;
                    (**ppcVar1)();
                    puVar8 = extraout_DX;
                }
                uStack6 = CONCAT22(puVar8, uVar11);
                for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
                {
                    uVar6 = uStack6;
                    pass1_1030_1d58((ulong)iVar10->field_0xc);
                    puVar9 = (uchar *)((uint)puVar8 | (uint)uVar6);
                    if((puVar9 != (uchar *)0x0)
                       && (uVar3 = pass1_1030_6fa0(uVar6 & 0xffff | ZEXT24(puVar8) << 0x10), uVar3 == 0xb))
                    {
                        pass1_1030_7c50(CONCAT13((char)((uint)puVar8 >> 0x8), CONCAT12((char)puVar8, (uint)uVar6)),
                                        (long)(int)uStack12,
                                        0x4,
                                        uStack12,
                                        puVar9);
                        return;
                    }
                    puVar8 = puVar9;
                }
            }
            else
            {
                lVar2 = iVar10->field_0x210;
                uVar6 = *(ulong *)((int)lVar2 + 0xa);
                for(uStack10 = 0x0; uStack10 < uVar6; uStack10 = uStack10 + 0x1)
                {
                    uVar5 = uVar6;
                    bad_1030_1312();
                    uVar11 = (uint)uVar5;
                    uVar10 = param_5 | uVar11;
                    if(((uVar10 != 0x0) && (pass1_1030_cc44(uVar11, param_5, uStack12, param_4, 0x4), uVar11 != 0x0))
                       && (uStack12 = uStack12 - uVar11, uStack12 == 0x0))
                    {
                        return;
                    }
                    param_5 = uVar10;
                }
            }
        }
        else
        {
            iStack22 = param_2 - param_3;
            if((iVar10->field_0x210 == 0x0) || (lVar2 = iVar10->field_0x210, *(long *)((int)lVar2 + 0xa) == 0x0))
            {
                if(iVar10->field_0xc == (undefined4 *)0x0)
                {
                    iVar4  = 0x0;
                    uVar11 = 0x0;
                }
                else
                {
                    ppcVar1 = (code **)((int)*iVar10->field_0xc + 0x10);
                    iVar4   = iStack22;
                    (**ppcVar1)();
                    uVar11 = extraout_DX_00;
                }
                uStack6 = CONCAT22(uVar11, iVar4);
                for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
                {
                    uVar6 = uStack6;
                    pass1_1030_1d58((ulong)iVar10->field_0xc);
                    uVar10 = uVar11 | (uint)uVar6;
                    if((uVar10 != 0x0)
                       && (uVar3 = pass1_1030_6fa0(uVar6 & 0xffff | (ulong)uVar11 << 0x10), uVar3 == 0xb))
                    {
                        pass1_1030_6e9c(
                          CONCAT13((char)(uVar11 >> 0x8), CONCAT12((char)uVar11, (uint)uVar6)), (long)iStack22, 0x4);
                        return;
                    }
                    uVar11 = uVar10;
                }
            }
            else
            {
                lVar2 = iVar10->field_0x210;
                uVar6 = *(ulong *)((int)lVar2 + 0xa);
                for(uStack10 = 0x0; uStack10 < uVar6; uStack10 = uStack10 + 0x1)
                {
                    uVar7 = uVar6;
                    bad_1030_1312();
                    uVar5   = (ulong)param_5;
                    uVar11  = (uint)uVar7;
                    param_5 = param_5 | uVar11;
                    if(param_5 != 0x0)
                    {
                        pass1_1030_ce72(uVar5 << 0x10 | uVar7 & 0xffff, iStack22, param_4, 0x4);
                        iStack22 = iStack22 - uVar11;
                        if(iStack22 == 0x0)
                        {
                            return;
                        }
                    }
                }
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_3aa6(ulong param_1, ushort param_2, uint param_3)

{
    code     **ppcVar1;
    undefined4 uVar2;
    ushort     uVar3;
    ulong      uVar4;
    ulong      uVar5;
    uint       extraout_DX;
    uint       uVar6;
    uint       uVar7;
    int        iVar8;
    undefined2 uVar9;
    ulong      uStack12;
    ulong      uStack8;

    uVar9 = (undefined2)(param_1 >> 0x10);
    iVar8 = (int)param_1;
    if((*(long *)(iVar8 + 0x210) == 0x0)
       || (uVar2 = *(undefined4 *)(iVar8 + 0x210), *(long *)((int)uVar2 + 0xa) == 0x0))
    {
        if(*(long *)(iVar8 + 0xc) == 0x0)
        {
            param_2 = 0x0;
            uVar6   = 0x0;
        }
        else
        {
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xc) + 0x10);
            (**ppcVar1)();
            uVar6 = extraout_DX;
        }
        uStack8 = CONCAT22(uVar6, param_2);
        for(uStack12 = 0x0; uStack12 < uStack8; uStack12 = uStack12 + 0x1)
        {
            uVar4 = uStack8;
            pass1_1030_1d58(*(ulong *)(iVar8 + 0xc));
            uVar7 = uVar6 | (uint)uVar4;
            if((uVar7 != 0x0) && (uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | (ulong)uVar6 << 0x10), uVar3 == 0xb))
            {
                pass1_1030_6b86(uVar4 & 0xffff | (ulong)uVar6 << 0x10, 0xb, 0x1030);
                return;
            }
            uVar6 = uVar7;
        }
    }
    else
    {
        uVar2 = *(undefined4 *)(iVar8 + 0x210);
        uVar4 = *(ulong *)((int)uVar2 + 0xa);
        for(uStack12 = 0x0; uStack12 < uVar4; uStack12 = uStack12 + 0x1)
        {
            uVar5 = uVar4;
            bad_1030_1312();
            uVar6 = param_3 | (uint)uVar5;
            if(uVar6 != 0x0)
            {
                pass1_1030_ce2e((uint)uVar5, param_3, 0x4);
            }
            param_3 = uVar6;
        }
    }
    return;
}


void __stdcall16far pass1_1038_3ba0(ulong param_1)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    char         cVar3;
    undefined4  *puVar4;
    ulong        uVar5;
    uint         uVar6;
    uint         uVar7;
    ulong        uVar8;
    uchar       *puVar9;
    uchar       *extraout_DX;
    uchar       *puVar10;
    uint         uVar11;
    astruct_428 *iVar13;
    undefined2   uVar12;
    undefined2   uVar13;
    ushort       unaff_SS;
    ulong       *puVar14;
    ulong        uVar15;
    ulong        uStack20;

    uVar12 = (undefined2)(param_1 >> 0x10);
    iVar13 = (astruct_428 *)param_1;
    puVar1 = *(undefined4 **)&iVar13->field_0x210;
    uVar6  = *(uint *)((int)&iVar13->field_0x210 + 0x2);
    if((uVar6 | (uint)puVar1) != 0x0)
    {
        ppcVar2 = (code **)*puVar1;
        (**ppcVar2)();
    }
    puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
    puVar9  = (uchar *)((ulong)puVar14 >> 0x10);
    uVar8   = (ulong)puVar14 & 0xffff;
    pass1_1038_4d6e(param_1, puVar14, (uint)uVar8, puVar9);
    uVar5   = uVar8 & 0xffff;
    puVar4  = (undefined4 *)(uVar5 | ZEXT24(puVar9) << 0x10);
    ppcVar2 = (code **)((int)*puVar4 + 0x10);
    (**ppcVar2)(0x1008, (int)uVar8, puVar9);
    uVar6 = (uint)uVar8;
    if((extraout_DX == (uchar *)0x0) && (uVar6 < 0x5))
    {
        uVar6 = 0x5;
    }
    uVar6   = uVar6 + 0x1;
    uVar13  = 0x1000;
    puVar10 = extraout_DX;
    uVar7   = uVar6;
    mem_op_1000_179c(0x1c, extraout_DX, 0x1000);
    uVar11 = (uint)puVar10 | uVar7;
    if(uVar11 == 0x0)
    {
        iVar13->field_0x210 = 0x0;
    }
    else
    {
        uVar11 = (int)uVar6 >> 0xf;
        cVar3  = (char)((ulong)uVar6 >> 0x8);
        uVar13 = 0x1030;
        struct_1030_11aa(
          (ushort *)CONCAT22(puVar10, uVar7), 0x5, CONCAT13(cVar3 >> 0xf, CONCAT12(cVar3 >> 0x7, uVar6)), unaff_SS);
        *(uint *)&iVar13->field_0x210              = uVar6;
        *(uint *)((int)&iVar13->field_0x210 + 0x2) = uVar11;
    }
    uVar15                              = iVar13->field_0x210;
    *(undefined2 *)((int)uVar15 + 0x1a) = 0x0;
    for(uStack20 = 0x0; uStack20 < (uVar8 & 0xffff | ZEXT24(extraout_DX) << 0x10); uStack20 = uStack20 + 0x1)
    {
        uVar15 = pass1_1030_1d7c((int)(uVar8 & 0xffff), uVar11, (ulong)puVar4);
        uVar6  = (uint)(uVar15 >> 0x10);
        uVar11 = uVar6 | (uint)uVar15;
        if(uVar11 != 0x0)
        {
            pass1_1030_1358(iVar13->field_0x210, (uint)uVar15, uVar6, uStack20 + 0x1, unaff_SS);
        }
        uVar13 = 0x1030;
    }
    if(puVar4 != (undefined4 *)0x0)
    {
        ppcVar2 = (code **)*puVar4;
        (**ppcVar2)(uVar13, (int)uVar5, (char)puVar9, 0x1);
    }
    return;
}


void __stdcall16far pass1_1038_3cc0(
  ulong param_1, uint param_2, uchar *param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7)

{
    long        lVar1;
    code      **ppcVar2;
    uint        uVar3;
    undefined4 *puVar4;
    ushort      uVar5;
    uchar      *extraout_DX;
    uchar      *extraout_DX_00;
    undefined2  extraout_DX_01;
    uint        extraout_DX_02;
    uint        uVar6;
    uchar      *extraout_DX_03;
    uchar      *puVar7;
    uchar      *extraout_DX_04;
    undefined4 *puVar8;
    uchar      *puVar9;
    undefined2  uVar10;
    ulong      *puVar11;
    ulong       uVar12;
    ulong       uVar13;
    undefined   uVar14;
    undefined   uVar15;
    undefined   uVar16;
    undefined   uVar17;
    undefined4 *puStack26;
    ulong       uStack22;
    ulong       uStack18;
    ulong       uStack14;
    undefined4 *puStack10;

    if(param_4 == 0x1e)
    {
        uVar10  = 0x1008;
        puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x27);
        puVar9  = (uchar *)((ulong)puVar11 >> 0x10);
        puVar8  = (undefined4 *)puVar11;
        pass1_1038_4e78((uint)puVar8, puVar9, param_1, puVar11);
        puStack10 = (undefined4 *)CONCAT22(puVar9, puVar8);
        ppcVar2   = (code **)((int)*puStack10 + 0x10);
        puVar4    = puVar8;
        (**ppcVar2)(0x1008, puVar8, puVar9);
        uStack14 = CONCAT22(extraout_DX_00, puVar4);
        puVar7   = extraout_DX_00;
        for(uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1)
        {
            uVar12 = pass1_1030_1d7c(puVar4, puVar7, (ulong)puStack10);
            puVar7 = (uchar *)((uint)(uVar12 >> 0x10) | (uint)uVar12);
            if(puVar7 != (uchar *)0x0)
            {
                uVar5     = pass1_1030_bfb8(uVar12, param_7);
                puStack26 = (undefined4 *)CONCAT22(puVar7, uVar5);
                puVar7    = (uchar *)((uint)puVar7 | uVar5);
                if(puVar7 != (uchar *)0x0)
                {
                    pass1_1028_b58e(uVar12);
                    if(CONCAT22(param_3, param_2) <= puStack26)
                    {
                        uVar10 = 0x1030;
                        pass1_1030_7ddc(CONCAT22(extraout_DX_01, uVar5),
                                        CONCAT13((char)((uint)param_3 >> 0x8), CONCAT12((char)param_3, param_2)),
                                        0x1e,
                                        param_2,
                                        param_3,
                                        param_5,
                                        param_6,
                                        param_7);
                        break;
                    }
                    puVar7 = param_3;
                    pass1_1030_7ddc(CONCAT22(extraout_DX_01, uVar5),
                                    (long)puStack26,
                                    0x1e,
                                    param_2,
                                    param_3,
                                    param_5,
                                    param_6,
                                    param_7);
                    lVar1   = CONCAT22(param_3, param_2) - (long)puStack26;
                    param_2 = (uint)lVar1;
                    param_3 = (uchar *)((ulong)lVar1 >> 0x10);
                }
            }
            uVar10 = 0x1030;
        }
        puStack26 = puStack10;
        if(puStack10 == (undefined4 *)0x0)
        {
            return;
        }
    }
    else
    {
        if(param_4 != 0x21)
        {
            uVar10  = 0x1008;
            puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x3);
            puVar7  = (uchar *)((ulong)puVar11 >> 0x10);
            uVar3   = (uint)puVar11;
            pass1_1038_4e78(uVar3, puVar7, param_1, puVar11);
            puStack26 = (undefined4 *)CONCAT22(puVar7, uVar3);
            ppcVar2   = (code **)((int)*puStack26 + 0x10);
            (**ppcVar2)(0x1008, uVar3, puVar7);
            uStack22 = CONCAT22(extraout_DX, uVar3);
            uStack18 = 0x0;
            puVar7   = extraout_DX;
        LAB_1038_3e9c:
            if(uStack18 < uStack22)
            {
                uVar10 = 0x1030;
                uVar12 = pass1_1030_1d7c(uVar3, puVar7, (ulong)puStack26);
                puVar7 = (uchar *)((uint)(uVar12 >> 0x10) | (uint)uVar12);
                if(puVar7 == (uchar *)0x0)
                    goto LAB_1038_3e98;
                uVar10 = SUB42(&USHORT_1050_1028, 0x0);
                uVar13 = pass1_1028_45e2(uVar12, (uint)uVar12, (int)puVar7, param_7);
                uVar6  = (uint)uVar13;
                puVar7 = (uchar *)((uint)(uVar13 >> 0x10) | uVar6);
                if(puVar7 == (uchar *)0x0)
                    goto LAB_1038_3e98;
                pass1_1028_b58e(uVar12);
                uVar12 = CONCAT22(param_3, param_2);
                if(uVar13 < uVar12)
                {
                    uVar10 = 0x1030;
                    puVar7 = param_3;
                    pass1_1030_7ddc(
                      CONCAT22(extraout_DX_04, uVar6), uVar13, param_4, param_2, param_3, param_5, param_6, param_7);
                    lVar1   = CONCAT22(param_3, param_2) - uVar13;
                    param_2 = (uint)lVar1;
                    param_3 = (uchar *)((ulong)lVar1 >> 0x10);
                    goto LAB_1038_3e98;
                }
                uVar16 = SUB21(param_3, 0x0);
                uVar17 = (undefined)((uint)param_3 >> 0x8);
                uVar14 = (undefined)uVar6;
                uVar15 = (undefined)(uVar6 >> 0x8);
                puVar7 = extraout_DX_04;
            LAB_1038_3e67:
                uVar10 = 0x1030;
                pass1_1030_7ddc(CONCAT22(puVar7, CONCAT11(uVar15, uVar14)),
                                CONCAT13(uVar17, CONCAT12(uVar16, param_2)),
                                param_4,
                                (uint)uVar12,
                                param_3,
                                param_5,
                                param_6,
                                param_7);
            }
            goto LAB_1038_3e6c;
        }
        uVar10  = 0x1008;
        puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0xa);
        puVar7  = (uchar *)((ulong)puVar11 >> 0x10);
        uVar3   = (uint)puVar11;
        pass1_1038_4e78(uVar3, puVar7, param_1, puVar11);
        puStack26 = (undefined4 *)CONCAT22(puVar7, uVar3);
        ppcVar2   = (code **)((int)*puStack26 + 0x10);
        (**ppcVar2)(0x1008, uVar3, puVar7);
        uStack22 = CONCAT22(extraout_DX_02, uVar3);
        uVar6    = extraout_DX_02;
        for(uStack18 = 0x0; uStack18 < uStack22; uStack18 = uStack18 + 0x1)
        {
            uVar10 = 0x1030;
            uVar13 = pass1_1030_1d7c(uVar3, uVar6, (ulong)puStack26);
            uVar12 = uVar13 & 0xffff;
            uVar6  = (uint)(uVar13 >> 0x10) | (uint)uVar12;
            if(uVar6 != 0x0)
            {
                uVar16 = SUB21(param_3, 0x0);
                uVar17 = (undefined)((uint)param_3 >> 0x8);
                pass1_1028_b58e(uVar13);
                uVar14  = (undefined)uVar12;
                uVar15  = (undefined)(uVar12 >> 0x8);
                param_3 = extraout_DX_03;
                puVar7  = extraout_DX_03;
                goto LAB_1038_3e67;
            }
        }
    LAB_1038_3e6c:
        if(puStack26 == (undefined4 *)0x0)
        {
            return;
        }
        puVar9 = (uchar *)((ulong)puStack26 >> 0x10);
        puVar8 = (undefined4 *)puStack26;
    }
    ppcVar2 = (code **)*puVar8;
    (**ppcVar2)(uVar10, (int)puStack26, (char)puVar9, 0x1);
    return;
LAB_1038_3e98:
    uStack18 = uStack18 + 0x1;
    goto LAB_1038_3e9c;
}


void __stdcall16far pass1_1038_3fb0(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x200);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    return;
}


ushort __stdcall16far pass1_1038_290e(uint param_1, uint param_2)

{
    undefined2 unaff_SI;
    undefined2 unaff_DI;
    ushort     unaff_SS;
    uchar      in_AF;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x1, 0x400);
    if((param_2 | param_1) != 0x0)
    {
        pass1_1038_4918(CONCAT22(param_2, param_1), param_1, param_2 | param_1, unaff_SS, in_AF);
    }
    pass1_1038_7a76(_PTR_LOOP_1050_5a64, unaff_SI, unaff_DI, unaff_SS);
    return 0x1;
}


ushort __stdcall16far pass1_1038_2ac2(
  ulong param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6, uchar param_7)

{
    undefined4 uVar1;
    ushort     uVar2;
    ushort     uVar3;
    ulong      uStack10;
    ulong      uStack6;

    uVar3 = (ushort)(param_1 >> 0x10);
    uVar2 = (ushort)param_1;
    uVar1 = *(undefined4 *)(uVar2 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    uStack6 = CONCAT22(param_3, param_2);
    uVar1   = *(undefined4 *)(uVar2 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    uStack10 = CONCAT22(param_3, param_2);
    pass1_1038_2c82(uVar2,
                    uVar3,
                    *(ulong *)(uVar2 + 0x110),
                    CONCAT22(param_3, param_2),
                    uStack6,
                    param_4,
                    param_5,
                    (int)&USHORT_1050_1028,
                    param_6,
                    param_7);
    pass1_1038_2c82(uVar2,
                    uVar3,
                    *(ulong *)(uVar2 + 0x114),
                    uStack6,
                    uStack10,
                    param_4,
                    param_5,
                    (int)&USHORT_1050_1028,
                    param_6,
                    param_7);
    return 0x1;
}


ushort __stdcall16far pass1_1038_2b2e(ulong param_1, ushort param_2, ushort param_3)

{
    undefined4 uVar1;
    ushort     uVar2;
    undefined2 unaff_SI;
    undefined2 unaff_DI;
    ushort     uVar3;
    undefined2 unaff_SS;
    ulong      uStack6;

    uVar3 = (ushort)(param_1 >> 0x10);
    uVar2 = (ushort)param_1;
    uVar1 = *(undefined4 *)(uVar2 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    uStack6 = CONCAT22(param_3, param_2);
    uVar1   = *(undefined4 *)(uVar2 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    pass1_1038_2f92(uVar2, uVar3, *(ulong *)(uVar2 + 0x110), CONCAT22(param_3, param_2), unaff_SI, unaff_DI, unaff_SS);
    pass1_1038_2f92(uVar2, uVar3, *(ulong *)(uVar2 + 0x114), uStack6, unaff_SI, unaff_DI, unaff_SS);
    return 0x1;
}


void __stdcall16far pass1_1038_2f92(
  ushort param_1, ushort param_2, ulong param_3, ulong param_4, ushort param_5, ushort param_6, ushort param_7)

{
    uint      *puVar1;
    int       *piVar2;
    int        iVar3;
    uint       uVar4;
    uint       uVar5;
    ulong      uVar6;
    undefined4 uVar7;
    int        iVar8;
    int        iVar9;
    undefined2 uVar10;
    undefined2 uVar11;
    int        iStack10;

    uVar10 = (undefined2)(param_4 >> 0x10);
    iVar8  = (int)param_4;
    uVar6  = *(ulong *)(iVar8 + 0x200);
    uVar11 = (undefined2)(param_3 >> 0x10);
    iVar9  = (int)param_3;
    iVar3  = *(int *)(iVar9 + 0xc);
    if(iVar3 == 0x1)
    {
        uVar7 = *(undefined4 *)(iVar9 + 0x8);
        pass1_1038_3cc0(
          param_4, (uint)uVar7, (uchar *)((ulong)uVar7 >> 0x10), *(ushort *)(iVar9 + 0xe), param_5, param_6, param_7);
        return;
    }
    if(iVar3 == 0x4)
    {
        pass1_1030_355c(*(ulong *)(iVar8 + 0x1f6), *(ulong *)(iVar9 + 0x10));
        return;
    }
    if(iVar3 == 0x5)
    {
        if(*(int *)(iVar9 + 0xe) != 0xc)
        {
            pass1_1038_5798(param_4, *(long *)(iVar9 + 0x8), *(int *)(iVar9 + 0xe));
            return;
        }
        iStack10 = (int)uVar6;
        if((iStack10 == 0x1) && ((uVar6 & 0xff0000) == 0x0))
        {
            uVar7   = *(undefined4 *)(iVar8 + 0x1f6);
            uVar4   = *(uint *)(iVar9 + 0x8);
            iVar3   = *(int *)(iVar9 + 0xa);
            uVar10  = (undefined2)((ulong)uVar7 >> 0x10);
            iVar8   = (int)uVar7;
            puVar1  = (uint *)(iVar8 + 0x170);
            uVar5   = *puVar1;
            *puVar1 = *puVar1 + uVar4;
            piVar2  = (int *)(iVar8 + 0x172);
            *piVar2 = *piVar2 + iVar3 + (uint)CARRY2(uVar5, uVar4);
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1038_1a30(ushort param_1, ushort param_2, ulong param_3, ushort param_4)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    uint        uVar3;
    ulong       uVar4;
    undefined2  extraout_DX;
    uint        extraout_DX_00;
    undefined2  uVar5;
    uint        uVar6;
    uint        uVar7;
    ulong       uStack18;
    ulong       uStack10;
    uint        uStack6;

    uVar5   = (undefined2)(param_3 >> 0x10);
    puVar1  = (undefined4 *)*(undefined4 *)((int)param_3 + 0x1e);
    uVar7   = *(uint *)((int)param_3 + 0x20);
    uStack6 = (uint)puVar1;
    uVar3   = uVar7 | uStack6;
    if(uVar3 != 0x0)
    {
        ppcVar2 = (code **)((int)*puVar1 + 0x10);
        uVar6   = uStack6;
        (**ppcVar2)();
        uStack10 = CONCAT22(extraout_DX, uVar3);
        for(uStack18 = 0x0; uStack18 < uStack10; uStack18 = uStack18 + 0x1)
        {
            ppcVar2 = (code **)((int)*puVar1 + 0x4);
            uVar4   = uStack10;
            (**ppcVar2)(param_4, uStack6, (int)((ulong)puVar1 >> 0x10), uStack18, uVar6, uVar7);
            if((extraout_DX_00 | (uint)uVar4) != 0x0)
            {
                param_4 = (ushort)&USHORT_1050_1028;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (uint)uVar4, extraout_DX_00);
            }
        }
        return;
    }
    return;
}


void __stdcall16far pass1_1038_1ac6(
  ushort param_1, ushort param_2, ulong param_3, ulong param_4, int param_5, ushort param_6, uchar param_7)

{
    undefined2 extraout_DX;
    undefined  local_118[0x112];
    undefined4 uStack6;

    pass1_1028_b58e(param_3);
    uStack6 = CONCAT22(extraout_DX, param_5);
    pass1_1030_e8a0((astruct_100 *)CONCAT22(param_6, local_118),
                    param_4,
                    *(ushort *)((int)param_3 + 0xc),
                    *(ulong *)(param_5 + 0x4),
                    param_6,
                    param_7);
    pass1_1028_d52c(*_PTR_LOOP_1050_5748, *_PTR_LOOP_1050_65e2 + 0x1, (ulong *)CONCAT22(param_6, local_118));
    return;
}


void __stdcall16far pass1_1038_0000(ulong param_1, uint param_2, uchar *param_3)

{
    undefined4 *puVar1;
    undefined4 *puVar2;
    undefined4 *puVar3;
    int         iVar4;
    undefined4 *puVar5;
    undefined2  uVar6;
    undefined2 *puStack10;

    // Segment:    8
    // Offset:     000606c0
    // Length:     ef91
    // Min Alloc:  ef91
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = (undefined2 *)CONCAT22(param_3, param_2);
    if(((uint)param_3 | param_2) != 0x0)
    {
        *puStack10                     = 0x389a;
        *(undefined2 *)(param_2 + 0x2) = 0x1008;
        uVar6                          = (undefined2)(param_1 >> 0x10);
        *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
        puVar3                         = (undefined4 *)((int)param_1 + 0x8);
        puVar5                         = (undefined4 *)(param_2 + 0x8);
        for(iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
        {
            puVar2  = puVar5;
            puVar5  = puVar5 + 0x1;
            puVar1  = puVar3;
            puVar3  = puVar3 + 0x1;
            *puVar2 = *puVar1;
        }
        *puStack10                     = 0x6ad2;
        *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
        *puStack10                     = 0xb96;
        *(undefined2 *)(param_2 + 0x2) = (int)&PTR_LOOP_1050_1038;
    }
    return;
}


void __stdcall16far
pass1_1038_08d4(ushort param_1, long param_2, ulong param_3, ulong param_4, ushort param_5, uchar param_6)

{
    undefined2 *puVar1;
    uint        uVar2;
    uint        uVar3;
    undefined2  local_16;
    undefined2  uStack20;
    int         iStack4;

    iStack4 = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_5, &local_16), 0x1, 0x0, 0x400);
    do
    {
        puVar1 = &local_16;
        pass1_1028_e4ec(CONCAT22(param_5, puVar1));
        uVar2   = (uint)param_4;
        uVar3   = uVar2 | (uint)puVar1;
        param_4 = param_4 & 0xffff0000 | (ulong)uVar3;
        if(uVar3 == 0x0)
            goto LAB_1038_0917;
    } while(*(long *)(puVar1 + 0x100) != 0x8000002);
    iStack4 = 0x1;
LAB_1038_0917:
    local_16 = 0x389a;
    uStack20 = 0x1008;
    if(iStack4 != 0x0)
    {
        if(param_2 < 0xc90000)
        {
            pass1_1038_0340(param_1, (ushort)param_2, param_2._2_2_, param_3, uVar3, param_5, param_6);
            return;
        }
        if(0x31fffff < param_2)
        {
            pass1_1038_05d8(param_1, (ushort)param_2, param_2._2_2_, param_3, param_4, param_5, param_6);
        }
    }
    return;
}


void __stdcall16far
pass1_1038_0c00(ulong param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, uchar param_6)

{
    code      **ppcVar1;
    undefined4  uVar2;
    undefined  *puVar3;
    uint        uVar4;
    uint        uVar5;
    uint        uVar6;
    uchar      *puVar7;
    uchar      *puVar8;
    ulong       uVar9;
    undefined2  uVar10;
    ulong      *puVar11;
    undefined4 *puStack32;
    ulong       uStack24;
    undefined   local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_5 >> 0x8), CONCAT12((char)param_5, local_14)), 0x1, 0x0, 0x400);
    while(true)
    {
        puVar3 = local_14;
        pass1_1028_e4ec(CONCAT22(param_5, puVar3));
        uVar6    = (uint)param_2;
        uStack24 = CONCAT22(uVar6, puVar3);
        uVar9    = param_2 & 0xffff0000 | (ulong)(uVar6 | (uint)puVar3);
        if((uVar6 | (uint)puVar3) == 0x0)
            break;
        pass1_1038_0e78(param_1, CONCAT22(uVar6, puVar3), param_5);
        pass1_1038_1220(param_1, CONCAT22(uVar6, puVar3), uVar9, param_5);
        uVar10  = (undefined2)(uVar9 >> 0x10);
        puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1);
        puVar7  = (uchar *)((ulong)puVar11 >> 0x10);
        uVar4   = (uint)puVar11;
        pass1_1038_4d6e(CONCAT22(uVar6, puVar3), puVar11, uVar4, puVar7);
        puStack32 = (undefined4 *)CONCAT22(puVar7, uVar4);
        ppcVar1   = (code **)((int)*puStack32 + 0x10);
        uVar5     = uVar4;
        puVar8    = puVar7;
        (**ppcVar1)(0x1008, uVar4, puVar7);
        param_2 = CONCAT22(uVar10, (uint)puVar8 | uVar5);
        if(((uint)puVar8 | uVar5) != 0x0)
        {
            uVar2 = *(undefined4 *)((int)param_1 + 0x108);
            if(*(int *)((int)uVar2 + 0x82) != 0x0)
            {
                pass1_1038_19a0(param_1, (ulong *)CONCAT22(puVar7, uVar4), CONCAT22(uVar6, puVar3), param_5, param_6);
            }
            pass1_1038_1940(param_1, (ulong *)CONCAT22(puVar7, uVar4), uStack24, param_3, param_4, param_5);
        }
        if(puStack32 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack32;
            (**ppcVar1)(0x8, uVar4, puVar7, 0x1);
        }
        pass1_1038_1c3e(param_1, uStack24, param_3, param_4, 0x1008, param_5);
    }
    return;
}


void __stdcall16far pass1_1038_0d8e(ushort param_1, ushort param_2, ulong param_3, ulong param_4, ushort param_5)

{
    ushort uVar1;
    uint   uVar2;
    ushort uVar3;
    long   lStack10;
    ushort uStack4;

    uVar1    = pass1_1030_d0a8(param_4);
    uVar2    = pass1_1030_d144(param_4);
    lStack10 = (long)(int)uVar2;
    uVar2    = (int)uVar2 >> 0xf | uVar2;
    uStack4  = uVar1;
    if(uVar2 != 0x0)
    {
        do
        {
            uVar3 = pass1_1028_6744(param_5, param_3, uStack4);
            uVar2 = uVar2 | uVar3;
            if(uVar2 != 0x0)
            {
                pass1_1028_6228(param_3, 0x1, 0x0, uStack4, param_5);
                lStack10 = lStack10 + -0x1;
                pass1_1030_d180(param_4, uStack4);
            }
            if(lStack10 == 0x0)
            {
                return;
            }
            uStack4 = pass1_1030_d0a8(param_4);
        } while(uStack4 != uVar1);
    }
    return;
}


void __stdcall16far pass1_1030_df0c(ulong param_1, ushort param_2)

{
    undefined4 uVar1;
    ulong      uVar2;
    long       lVar3;
    ushort     uVar4;
    int        iVar5;
    ulong      uVar6;
    undefined2 extraout_DX;
    uint       uVar7;
    uint       uVar8;
    undefined2 uVar9;
    ushort     uVar10;
    undefined2 uStack24;
    undefined2 uStack22;
    uint       uStack14;
    uint       uStack10;

    pass1_1028_b58e(param_1);
    uVar1    = *(undefined4 *)(param_2 + 0x2e);
    uStack10 = (uint)uVar1;
    if((*(uint *)(param_2 + 0x30) | uStack10) != 0x0)
    {
        uVar9    = (undefined2)((ulong)uVar1 >> 0x10);
        uVar1    = *(undefined4 *)(uStack10 + 0x210);
        uVar7    = *(uint *)(uStack10 + 0x212);
        uStack14 = (uint)uVar1;
        if((uVar7 | uStack14) != 0x0)
        {
            uVar2 = *(ulong *)(uStack14 + 0xa);
            uVar4 = pass1_1030_dfcc(param_1);
            if(uVar4 != 0x0)
            {
                uStack24 = 0x1;
                uStack22 = 0x0;
                while(CONCAT22(uStack22, uStack24) < uVar2)
                {
                    uVar6  = uVar2;
                    uVar10 = uVar4;
                    bad_1030_1312();
                    uVar8 = uVar7;
                    iVar5 = pass1_1030_cde8((int)uVar6, uVar7, uVar10);
                    if(-0x1 < iVar5)
                    {
                        pass1_1030_cef8(
                          uVar6 & 0xffff | (ulong)uVar7 << 0x10, CONCAT22(extraout_DX, param_2), 0x1, iVar5);
                        *(undefined4 *)((int)param_1 + 0x20) = *(undefined4 *)((int)uVar6 + 0x4);
                        return;
                    }
                    lVar3    = CONCAT22(uStack22, uStack24) + 0x1;
                    uStack24 = (undefined2)lVar3;
                    uVar7    = uVar8;
                    uStack22 = (undefined2)((ulong)lVar3 >> 0x10);
                }
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_e0d4(uchar *param_1, ushort param_2, int param_3)

{
    int       *piVar1;
    undefined4 uVar2;
    uint       uVar3;
    undefined *puVar4;
    undefined *puVar5;
    uint       uVar6;
    uint       extraout_DX;
    uchar     *puVar7;
    uchar     *puVar8;
    int        iVar9;
    undefined2 uVar10;
    ushort    *puVar11;
    undefined4 uStack42;
    undefined  local_1c[0x8];
    undefined4 uStack20;
    uint       uStack16;
    undefined4 uStack14;
    ulong      uStack10;
    int        iStack6;
    ushort     uStack4;

    puVar11  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x40, param_2, param_1, param_3);
    uStack4  = (ushort)((ulong)puVar11 >> 0x10);
    iStack6  = (int)puVar11;
    uStack10 = pass1_1008_b820((ulong)puVar11, iStack6, uStack4);
    uVar3    = (uint)uStack10;
    uVar6    = (uint)(uStack10 >> 0x10) | uVar3;
    if(uVar6 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x1, 0x800);
        uStack14 = CONCAT22(uVar6, uVar3);
        uStack16 = (uint)(*(int *)(uVar3 + 0x154) != 0x0);
        pass1_1008_5784((ulong *)CONCAT22(param_2, local_1c), uStack10);
        while(true)
        {
            puVar4 = local_1c;
            pass1_1008_5b12(puVar4, param_2);
            uStack20 = CONCAT22(extraout_DX, puVar4);
            puVar7   = (uchar *)(extraout_DX | (uint)puVar4);
            if(puVar7 == (uchar *)0x0)
                break;
            if(*(int *)(puVar4 + 0x8) != 0x0)
            {
                uVar2 = *(undefined4 *)(puVar4 + 0xa);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)((ulong)uVar2 >> 0x10));
                puVar8 = puVar7;
                puVar5 = puVar4;
                pass1_1038_354a(CONCAT22(puVar7, puVar4), (uint)puVar4, puVar7);
                if(puVar5 != (undefined *)0x0)
                {
                    uVar10 = (undefined2)((ulong)uStack20 >> 0x10);
                    if(uStack16 == 0x0)
                    {
                        iVar9    = *(int *)((int)uStack20 + 0xe) * 0xc;
                        uStack42 = *(undefined4 *)(iVar9 + 0x58c4);
                        uVar3    = *(uint *)(iVar9 + 0x58c8);
                    }
                    else
                    {
                        iVar9    = *(int *)((int)uStack20 + 0xe) * 0xc;
                        uStack42 = *(undefined4 *)(iVar9 + 0x58be);
                        uVar3    = *(uint *)(iVar9 + 0x58c2);
                    }
                    uVar6 = uVar3;
                    pass1_1038_35a8(CONCAT22(puVar7, puVar4),
                                    *(ushort *)(*(int *)((int)uStack20 + 0x10) * 0x2 + (int)uStack42),
                                    uVar3,
                                    puVar8);
                    if(uVar6 != 0x0)
                    {
                        uVar10  = (undefined2)((ulong)uStack20 >> 0x10);
                        iVar9   = (int)uStack20;
                        piVar1  = (int *)(iVar9 + 0x10);
                        *piVar1 = *piVar1 + 0x1;
                        if((int)uVar3 <= *(int *)(iVar9 + 0x10))
                        {
                            *(undefined2 *)(iVar9 + 0x10) = 0x0;
                        }
                    }
                }
            }
        }
    }
    return;
}


ushort __stdcall16far pass1_1030_e328(ulong param_1, ushort param_2, ushort param_3, ushort param_4, uchar param_5)

{
    uint uVar1;

    uVar1 = (uint)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x110) == 0x0)
    {
        pass1_1030_e4ba(param_1);
    }
    else
    {
        pass1_1030_e410(param_4, param_2, param_5, param_3, param_1 & 0xffff | (ulong)uVar1 << 0x10);
    }
    return 0x1;
}


void __stdcall16far pass1_1030_e410(ushort param_1, ushort param_2, uchar param_3, ushort param_4, ulong param_5)

{
    undefined4 uVar1;
    uchar     *puVar2;
    undefined2 uVar3;
    ushort    *puVar4;
    undefined  local_10[0x6];
    undefined  local_a[0x4];
    uint       uStack6;
    uint       uStack4;

    uVar1 = *(undefined4 *)((int)param_5 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    puVar2 = (uchar *)(param_4 | param_2);
    if(puVar2 != (uchar *)0x0)
    {
        uStack6 = param_2;
        uStack4 = param_4;
        pass1_1038_4fd8(param_2, CONCAT22(param_4, param_2), 0x21);
        if(param_2 == 0x0)
        {
            pass1_1020_a43e(param_1, puVar2, (ushort *)CONCAT22(param_1, local_a));
            puVar4 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_10), 0x0, 0x2, 0xfffd);
            uVar3  = (undefined2)((ulong)puVar4 >> 0x10);
            pass1_1020_a49a(
              param_1, param_3, uVar3, CONCAT22(param_1, local_a), (int *)CONCAT22(param_1, local_10), 0x7a);
            pass1_1008_3e76((ushort *)CONCAT22(param_1, local_10), 0x0, 0x3, 0xfffe);
            pass1_1020_a49a(
              param_1, param_3, uVar3, CONCAT22(param_1, local_a), (int *)CONCAT22(param_1, local_10), 0x7a);
            pass1_1008_3e76((ushort *)CONCAT22(param_1, local_10), 0x0, 0x3, 0xfffd);
            pass1_1020_a49a(
              param_1, param_3, uVar3, CONCAT22(param_1, local_a), (int *)CONCAT22(param_1, local_10), 0x21);
        }
    }
    return;
}


void __stdcall16far pass1_1030_e4ba(void)

{
    return;
}


ushort __stdcall16far pass1_1030_e540(void)

{
    return 0x1;
}


ushort __stdcall16far pass1_1030_e546(ulong param_1, ushort param_2)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x108);
    pass1_1028_e332(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10), param_2);
    return 0x1;
}


ushort __stdcall16far pass1_1030_e7d0(void)

{
    return 0x1;
}


ushort __stdcall16far pass1_1030_eb86(uint param_1, ushort param_2)

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
        uVar4  = param_1;
        puVar3 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, puVar3));
        puStack24 = (undefined4 *)CONCAT22(uVar4, puVar3);
        param_1   = uVar4 | (uint)puVar3;
        if(param_1 == 0x0)
            break;
        if(*(int *)(puVar3 + 0x12) == 0x5)
        {
            iVar1 = *(int *)(puVar3 + 0xc);
            if(((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
               && ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2))))))
            {
                ppcVar2 = (code **)((int)*puStack24 + 0x2c);
                (**ppcVar2)((int)&USHORT_1050_1028);
                param_1 = extraout_DX;
            }
        }
    }
    return 0x1;
}


void __stdcall16far pass1_1030_ecf8(ulong param_1, ulong param_2, int param_3, ushort param_4, uchar param_5)

{
    int         iVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    ushort      uVar4;
    ulong       uVar5;
    undefined  *puVar6;
    int         iVar7;
    ulong       uVar8;
    uint        uVar9;
    uint        uVar10;
    uint        uVar11;
    uint        uVar12;
    ulong       uVar13;
    undefined2  uVar14;
    bool        bVar15;
    ushort     *puVar16;
    undefined4 *puVar17;
    ushort      uVar18;
    ulong       uStack64;
    int         iStack56;
    ushort      uStack54;
    ulong       uStack38;
    undefined   local_22[0x12];
    undefined2  uStack16;
    undefined2  uStack14;
    ushort      uStack12;
    uint        uStack10;
    uint        uStack8;
    uint        uStack6;
    undefined2  uStack4;

    uStack12 = 0x0;
    puVar16  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, (uchar *)param_2, param_3);
    uVar13   = param_2 & 0xffff0000 | (ulong)puVar16 >> 0x10;
    uStack10 = (uint)puVar16;
    uStack4  = (undefined2)((ulong)puVar16 >> 0x10);
    uStack6  = uStack10;
    pass1_1010_ed3e((ulong)puVar16);
    uStack8 = (uint)uVar13;
    uVar13  = uVar13 & 0xffff0000 | (ulong)(uStack8 | uStack10);
    if((uStack8 | uStack10) != 0x0)
    {
        uStack12 = pass1_1030_2aaa(CONCAT22(uStack8, uStack10));
    }
    if((int)uStack12 < 0x2)
    {
        uStack12 = 0x0;
    }
    puVar16  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, (uchar *)uVar13, param_3);
    uVar13   = uVar13 & 0xffff0000 | (ulong)puVar16 >> 0x10;
    uStack16 = SUB42(puVar16, 0x0);
    uStack14 = (undefined2)((ulong)puVar16 >> 0x10);
    if((0x0 < (int)PTR_LOOP_1050_13ae) && (!SBORROW2((int)PTR_LOOP_1050_13ae, 0x1)))
    {
        if(PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1)
        {
            if(0x6 < (int)uStack12)
            {
                uStack12 = uStack12 - 0x2;
                goto LAB_1030_ed5b;
            }
            bVar15 = SBORROW2(uStack12, 0x4);
            iVar1  = uStack12 - 0x4;
        }
        else
        {
            if(PTR_LOOP_1050_13ae != (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1))
                goto LAB_1030_ed5b;
            bVar15 = SBORROW2(uStack12, 0x7);
            iVar1  = uStack12 - 0x7;
        }
        if(bVar15 == iVar1 < 0x0)
        {
            uStack12 = uStack12 - 0x1;
        }
    }
LAB_1030_ed5b:
    pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_4 >> 0x8), CONCAT12((char)param_4, local_22)), 0x1, 0x0, 0x400);
    while(true)
    {
        puVar6 = local_22;
        pass1_1028_e4ec(CONCAT22(param_4, puVar6));
        uVar9    = (uint)uVar13;
        uStack38 = CONCAT22(uVar9, puVar6);
        if((uVar9 | (uint)puVar6) == 0x0)
            break;
        uVar10 = (uint) * (undefined4 *)(puVar6 + 0x1f6);
        uVar13 = uVar13 & 0xffff0000 | (ulong) * (uint *)(puVar6 + 0x1f8);
        if((*(int *)(puVar6 + 0x1fe) != 0x0) && (*(long *)(puVar6 + 0x200) != 0x8000002))
        {
            pass1_1030_38b8();
            uVar10 = (uint)uVar13 | uVar10;
            uVar8  = uVar13 & 0xffff0000;
            uVar13 = uVar8 | uVar10;
            if(uVar10 != 0x0)
            {
                puVar2  = *(undefined4 **)(puVar6 + 0xc);
                uVar10  = *(uint *)(puVar6 + 0xe);
                uVar8   = uVar8 | uVar10;
                ppcVar3 = (code **)((int)*puVar2 + 0x10);
                puVar17 = puVar2;
                (**ppcVar3)((int)&USHORT_1050_1028, (int)puVar2, uVar10);
                uVar5    = (ulong)puVar17 & 0xffff | uVar8 << 0x10;
                uStack54 = *(ushort *)(puVar6 + 0x18);
                uVar14   = SUB42(&PTR_LOOP_1050_1038, 0x0);
                pass1_1038_4760(CONCAT22(uVar9, puVar6));
                iVar1    = *(int *)(puVar6 + 0x22);
                iStack56 = iVar1 / 0xa;
                uVar13   = uVar8 & 0xffff0000 | (long)iVar1 % 0xa & 0xffffU;
                iVar1    = *(int *)(puVar6 + 0x24);
                if(iVar1 < 0x33)
                {
                    if(iVar1 < 0x32)
                    {
                        iStack56 = iStack56 + -0x1;
                    }
                }
                else
                {
                    uStack54 = uStack54 + 0x1;
                }
                for(uStack64 = 0x0; uStack64 < uVar5; uStack64 = uStack64 + 0x1)
                {
                    ppcVar3 = (code **)((int)*puVar2 + 0x4);
                    uVar8   = uVar5;
                    (**ppcVar3)(
                      uVar14, (char)puVar2, (int)((ulong)puVar2 >> 0x10), (int)uStack64, (int)(uStack64 >> 0x10));
                    uVar10 = (uint)uVar8;
                    uVar11 = (uint)uVar13;
                    uVar12 = uVar11 | uVar10;
                    uVar13 = uVar13 & 0xffff0000 | (ulong)uVar12;
                    if(uVar12 != 0x0)
                    {
                        uVar14 = SUB42(&USHORT_1050_1028, 0x0);
                        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar10, uVar11);
                        puVar17 = (undefined4 *)struct_op_1030_73a8(CONCAT22((int)uVar13, uVar10));
                        uVar10  = (uint)puVar17;
                        uVar11  = (uint)((ulong)puVar17 >> 0x10);
                        uVar13  = uVar13 & 0xffff0000 | (ulong)(uVar11 | uVar10);
                        if(((uVar11 | uVar10) != 0x0) && (*(int *)(uVar10 + 0x12) == 0x5))
                        {
                            ppcVar3 = (code **)((int)*puVar17 + 0x48);
                            (**ppcVar3)((int)&USHORT_1050_1028, uVar10, uVar11);
                            if((int)uVar10 < 0x0)
                            {
                                iStack56 = iStack56 + uVar10;
                            }
                            else
                            {
                                uStack54 = uStack54 + uVar10;
                            }
                        }
                    }
                }
                iStack56 = iStack56 - uStack12;
                iVar1    = *(int *)(puVar6 + 0x20a);
                uVar18   = (ushort)(param_1 >> 0x10);
                uVar4    = (ushort)param_1;
                iVar7    = iVar1;
                pass1_1038_01c0(uVar4, uVar18, uStack38, param_4);
                iVar7    = iVar7 - iVar1;
                iStack56 = iStack56 - iVar7;
                pass1_1038_008e(uVar4, uVar18, uStack38, (uchar *)uVar13, param_3, param_4);
                if(iVar7 < 0x0)
                {
                    iStack56 = iStack56 + iVar7;
                }
                else
                {
                    uStack54 = uStack54 + iVar7;
                }
                if(0x3e8 < (int)uStack54)
                {
                    uStack54 = 0x3e8;
                }
                if((int)uStack54 < 0x0)
                {
                    uStack54 = 0x0;
                }
                uStack54 = uStack54 + iStack56;
                if(0x3e8 < (int)uStack54)
                {
                    uStack54 = 0x3e8;
                }
                if((int)uStack54 < 0x0)
                {
                    uStack54 = 0x0;
                }
                pass1_1038_4d0e(uStack38, uStack54);
                if(*(long *)(puVar6 + 0x4) == 0x4000001)
                {
                    pass1_1038_08d4(uVar4, CONCAT22(uStack54, uVar18), uStack38, uVar13, param_4, param_5);
                }
                pass1_1038_095e(uVar4, uVar18, uStack54, uStack38, (uchar *)uVar13, param_3, param_4);
            }
        }
    }
    return;
}


int __stdcall16far pass1_1030_cde8(int param_1, ushort param_2, int param_3)

{
    int iVar1;
    int iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return -0x1;
        }
        iVar1 = iStack4 * 0xc + param_1;
        if((*(int *)(iVar1 + 0x24) == param_3) && (*(int *)(iVar1 + 0x26) == 0x0))
            break;
        iStack4 = iStack4 + 0x1;
    }
    return iStack4;
}


int __stdcall16far pass1_1030_ce2e(int param_1, ushort param_2, int param_3)

{
    int        iVar1;
    undefined4 uStack6;

    for(uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1))
    {
        iVar1 = (int)uStack6 * 0xc + param_1;
        if((*(int *)(iVar1 + 0x24) == param_3) && (*(int *)(iVar1 + 0x26) == 0x0))
        {
            uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
        }
    }
    return uStack6._2_2_;
}


void __stdcall16far pass1_1030_ce72(ulong param_1, int param_2, ulong param_3, int param_4)

{
    long         lVar1;
    astruct_300 *iVar2;
    int          iStack10;

    lVar1    = *(long *)((int)param_3 + 0x4);
    iStack10 = 0x0;
    do
    {
        if(0x9 < iStack10)
        {
            return;
        }
        iVar2 = (astruct_300 *)(iStack10 * 0xc + (int)param_1);
        if((iVar2->field_0x24 == param_4) && (iVar2->field_0x28 == 0x0))
        {
            iVar2->field_0x28 = lVar1;
            if(param_4 == 0x4)
            {
                iVar2->field_0x26 = 0x2;
            }
            else
            {
                *(undefined2 *)((int)param_1 + iStack10 * 0xc + 0x26) = 0x1;
            }
            param_2 = param_2 + -0x1;
            if(param_2 == 0x0)
            {
                return;
            }
        }
        iStack10 = iStack10 + 0x1;
    } while(true);
}


void __stdcall16far pass1_1030_cef8(ulong param_1, ulong param_2, ushort param_3, int param_4)

{
    undefined2 uVar1;
    int        iVar2;
    undefined2 uVar3;
    undefined2 uVar4;

    uVar3                                         = (undefined2)(param_1 >> 0x10);
    iVar2                                         = (int)param_1;
    *(ushort *)(iVar2 + param_4 * 0xc + 0x26)     = param_3;
    uVar4                                         = (undefined2)(param_2 >> 0x10);
    uVar1                                         = *(undefined2 *)((int)param_2 + 0x6);
    *(undefined2 *)(iVar2 + param_4 * 0xc + 0x28) = *(undefined2 *)((int)param_2 + 0x4);
    *(undefined2 *)(iVar2 + param_4 * 0xc + 0x2a) = uVar1;
    return;
}


ushort __stdcall16far pass1_1030_cf3a(ulong param_1, int param_2)

{
    int iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return 0x0;
        }
        if(*(int *)((int)param_1 + iStack4 * 0xc + 0x24) == param_2)
            break;
        iStack4 = iStack4 + 0x1;
    }
    return 0x1;
}


void __stdcall16far pass1_1030_cf78(ulong param_1, uint param_2)

{
    ulong        uVar1;
    uint         extraout_DX;
    astruct_680 *iVar3;
    undefined2   uVar2;
    int          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return;
        }
        uVar1 = (ulong)param_2;
        uVar2 = (undefined2)(param_1 >> 0x10);
        if(*(uint *)((int)param_1 + iStack4 * 0xc + 0x24) == param_2)
            break;
        iStack4 = iStack4 + 0x1;
    }
    pass1_1028_b58e(param_1);
    if(param_2 == 0x5)
    {
        pass1_1038_4900(*(ulong *)((int)uVar1 + 0x2e));
    }
    else
    {
        pass1_1030_6e9c(uVar1 & 0xffff | (ulong)extraout_DX << 0x10, 0x1, param_2);
    }
    iVar3             = (astruct_680 *)(iStack4 * 0xc + (int)param_1);
    iVar3->field_0x20 = 0x0;
    iVar3->field_0x24 = 0x0;
    iVar3->field_0x26 = 0x0;
    return;
}


void __stdcall16far pass1_1030_d00c(int param_1, ushort param_2, uint param_3)

{
    ulong        uVar1;
    uint         extraout_DX;
    astruct_696 *local_BX_40;
    int          iVar2;
    int          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return;
        }
        iVar2 = iStack4 * 0xc + param_1;
        if((*(int *)(iVar2 + 0x26) == 0x0) && (uVar1 = (ulong)param_3, *(uint *)(iVar2 + 0x24) == param_3))
            break;
        iStack4 = iStack4 + 0x1;
    }
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    if(param_3 == 0x5)
    {
        pass1_1038_4900(*(ulong *)((int)uVar1 + 0x2e));
    }
    else
    {
        pass1_1030_6e9c(uVar1 & 0xffff | (ulong)extraout_DX << 0x10, 0x1, param_3);
    }
    local_BX_40             = (astruct_696 *)(iStack4 * 0xc + param_1);
    local_BX_40->field_0x20 = 0x0;
    local_BX_40->field_0x24 = 0x0;
    local_BX_40->field_0x26 = 0x0;
    return;
}


ushort __stdcall16far pass1_1030_d0a8(ulong param_1)

{
    ushort uVar1;
    uint   uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    uVar1 = *(ushort *)((int)param_1 + 0x98);
    pass1_1030_d56a(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    return uVar1;
}


int __stdcall16far pass1_1030_d0c6(ulong param_1)

{
    undefined4 uStack6;

    for(uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1))
    {
        if(*(long *)((int)param_1 + (int)uStack6 * 0xc + 0x20) != 0x0)
        {
            uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
        }
    }
    return uStack6._2_2_;
}


int __stdcall16far pass1_1030_d102(int param_1, ushort param_2)

{
    int        iVar1;
    undefined4 uStack6;

    for(uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1))
    {
        iVar1 = (int)uStack6 * 0xc + param_1;
        if((*(long *)(iVar1 + 0x20) != 0x0) && (*(int *)(iVar1 + 0x26) != 0x0))
        {
            uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
        }
    }
    return uStack6._2_2_;
}


int __stdcall16far pass1_1030_d144(ulong param_1)

{
    undefined4 uStack6;

    for(uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0x1))
    {
        if(*(long *)((int)param_1 + (int)uStack6 * 0xc + 0x20) == 0x0)
        {
            uStack6 = uStack6 & 0xffff | (ulong)(uStack6._2_2_ + 0x1) << 0x10;
        }
    }
    return uStack6._2_2_;
}


void __stdcall16far pass1_1030_d180(ulong param_1, uint param_2)

{
    int    iVar1;
    uint   uVar2;
    uchar *extraout_DX;
    ushort uVar3;
    int    iVar4;
    ushort uVar5;
    int    iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return;
        }
        uVar5 = (ushort)(param_1 >> 0x10);
        uVar3 = (ushort)param_1;
        if((*(uint *)(uVar3 + iStack4 * 0xc + 0x22) | *(uint *)(uVar3 + iStack4 * 0xc + 0x20)) == 0x0)
            break;
        iStack4 = iStack4 + 0x1;
    }
    uVar2                   = *_PTR_LOOP_1050_65e2;
    iVar1                   = *(int *)((int)_PTR_LOOP_1050_65e2 + 0x2);
    iVar4                   = iStack4 * 0xc + uVar3;
    *(int *)(iVar4 + 0x20)  = uVar2 + 0xc8;
    *(int *)(iVar4 + 0x22)  = iVar1 + (uint)(0xff37 < uVar2);
    *(uint *)(iVar4 + 0x24) = param_2;
    uVar2                   = param_2;
    pass1_1030_d340(uVar3, uVar5, param_1 & 0xffff0000 | (ulong)(iVar4 + 0x20));
    pass1_1028_b58e(param_1);
    if(param_2 == 0x5)
    {
        pass1_1038_48e0(*(ulong *)(uVar2 + 0x2e), 0x1);
        return;
    }
    pass1_1030_7c50(CONCAT22(extraout_DX, uVar2), 0x1, param_2, uVar2, extraout_DX);
    return;
}


ushort __stdcall16far pass1_1030_d230(ulong param_1)

{
    int iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return 0x1;
        }
        if(*(long *)((int)param_1 + iStack4 * 0xc + 0x20) == 0x0)
            break;
        iStack4 = iStack4 + 0x1;
    }
    return 0x0;
}


void __stdcall16far pass1_1030_d26c(ulong param_1, ushort param_2)

{
    ulong     *puVar1;
    ulong      uVar2;
    int        iVar3;
    ulong      uVar4;
    undefined2 extraout_DX;
    int        iVar5;
    int        iStack8;

    uVar2 = *_PTR_LOOP_1050_65e2;
    for(iStack8 = 0x0; iStack8 < 0xa; iStack8 = iStack8 + 0x1)
    {
        iVar5 = iStack8 * 0xc + (int)param_1;
        if(((*(uint *)(iVar5 + 0x22) | *(uint *)(iVar5 + 0x20)) != 0x0)
           && (puVar1 = (ulong *)(iVar5 + 0x20), *puVar1 < uVar2 || *puVar1 == uVar2))
        {
            uVar4 = uVar2;
            pass1_1030_d3b2((int)param_1, param_1._2_2_, iStack8, (int)uVar2, param_2);
            iVar3 = (int)uVar4;
            if(iVar3 == 0x0)
            {
                pass1_1028_b58e(param_1);
                if(*(int *)(iVar5 + 0x24) == 0x5)
                {
                    pass1_1038_4900(*(ulong *)(iVar3 + 0x2e));
                }
                else
                {
                    pass1_1030_6e9c(CONCAT22(extraout_DX, iVar3), 0x1, *(int *)((int)param_1 + iStack8 * 0xc + 0x24));
                }
                iVar5                         = iStack8 * 0xc + (int)param_1;
                *(undefined4 *)(iVar5 + 0x20) = 0x0;
                *(undefined2 *)(iVar5 + 0x24) = 0x0;
                *(undefined2 *)(iVar5 + 0x26) = 0x0;
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_d340(ushort param_1, ushort param_2, ulong param_3)

{
    int        iVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_3 >> 0x10);
    iVar2 = (int)param_3;
    iVar1 = *(int *)(iVar2 + 0x4);
    if(((0x0 < iVar1) && (!SBORROW2(iVar1, 0x1))) && ((iVar1 == 0x4 || iVar1 + -0x1 < 0x3 || (iVar1 == 0xc))))
    {
        *(undefined2 *)(iVar2 + 0x6) = 0x0;
        return;
    }
    *(undefined2 *)(iVar2 + 0x6) = 0x1;
    return;
}


ushort __stdcall16far pass1_1030_d36e(ulong param_1, int param_2)

{
    int iStack4;

    iStack4 = 0x0;
    while(true)
    {
        if(0x9 < iStack4)
        {
            return 0x0;
        }
        if((iStack4 != param_2) && (*(int *)((int)param_1 + iStack4 * 0xc + 0x24) == 0x8))
            break;
        iStack4 = iStack4 + 0x1;
    }
    return 0x1;
}


void __stdcall16far pass1_1030_d3b2(int param_1, ushort param_2, int param_3, int param_4, ushort param_5)

{
    int         iVar1;
    code      **ppcVar2;
    bool        bVar3;
    ushort      uVar4;
    uint        uVar5;
    undefined2  extraout_DX;
    uchar      *puVar6;
    undefined2  extraout_DX_00;
    undefined2  extraout_DX_01;
    uint        uVar7;
    undefined2  uVar8;
    ulong      *puVar9;
    ulong       uVar10;
    ulong       uVar11;
    undefined4 *puStack26;
    ulong       uStack18;
    undefined4  uStack14;

    pass1_1028_b58e(CONCAT22(param_2, param_1));
    uVar11 = *(ulong *)(param_4 + 0x2e);
    uVar4  = pass1_1030_d36e(CONCAT22(param_2, param_1), param_3);
    if(uVar4 == 0x0)
    {
        puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
        uVar5  = (uint)puVar9;
        pass1_1038_4d6e(uVar11, puVar9, uVar5, puVar6);
        puStack26 = (undefined4 *)CONCAT22(puVar6, uVar5);
        ppcVar2   = (code **)((int)*puStack26 + 0x10);
        uVar7     = uVar5;
        (**ppcVar2)((int)&PTR_LOOP_1050_1038, uVar5, puVar6);
        uStack18 = CONCAT22(extraout_DX_00, uVar7);
        bVar3    = false;
        for(uStack14 = 0x0; uStack14 < uStack18; uStack14 = uStack14 + 0x1)
        {
            uVar10 = pass1_1030_1d7c((int)uStack14, uStack14._2_2_, (ulong)puStack26);
            uVar7  = (uint)(uVar10 >> 0x10);
            if((((uVar7 | (uint)uVar10) != 0x0) && (*(long *)((uint)uVar10 + 0x4) != *(long *)(param_1 + 0x4)))
               && (uVar4 = pass1_1030_cf3a(uVar10, 0x8), uVar4 != 0x0))
            {
                bVar3 = true;
                break;
            }
        }
        if(puStack26 != (undefined4 *)0x0)
        {
            ppcVar2 = (code **)*puStack26;
            (**ppcVar2)(0x38, uVar5, puVar6, 0x1);
        }
        if(!bVar3)
        {
            return;
        }
    }
    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
    puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
    uVar5  = (uint)puVar9;
    uVar8  = SUB42(&PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(uVar11, puVar9, uVar5, puVar6);
    puStack26 = (undefined4 *)CONCAT22(puVar6, uVar5);
    ppcVar2   = (code **)((int)*puStack26 + 0x10);
    uVar7     = uVar5;
    (**ppcVar2)((int)&PTR_LOOP_1050_1038, uVar5, puVar6);
    uStack18 = CONCAT22(extraout_DX_01, uVar7);
    bVar3    = false;
    uStack14 = 0x0;
    do
    {
        if(uStack18 <= uStack14)
        {
        LAB_1030_d51b:
            if(puStack26 != (undefined4 *)0x0)
            {
                ppcVar2 = (code **)*puStack26;
                (**ppcVar2)(uVar8, (char)uVar5, (char)puVar6, 0x1);
            }
            if(!bVar3)
            {
                return;
            }
            uVar5                                    = *_PTR_LOOP_1050_65e2;
            iVar1                                    = *(int *)((int)_PTR_LOOP_1050_65e2 + 0x2);
            *(int *)(param_1 + param_3 * 0xc + 0x20) = uVar5 + 0xc8;
            *(int *)(param_1 + param_3 * 0xc + 0x22) = iVar1 + (uint)(0xff37 < uVar5);
            return;
        }
        uVar11 = pass1_1030_1d7c((int)uStack14, uStack14._2_2_, (ulong)puStack26);
        uVar7  = (uint)(uVar11 >> 0x10) | (uint)uVar11;
        if(uVar7 != 0x0)
        {
            uVar8 = SUB42(&USHORT_1050_1028, 0x0);
            uVar4 = pass1_1028_6744(param_5, uVar11, 0x7);
            if((uVar7 | uVar4) != 0x0)
            {
                uVar8 = SUB42(&USHORT_1050_1028, 0x0);
                pass1_1028_6228(uVar11, 0x1, 0x0, 0x7, param_5);
                bVar3 = true;
                goto LAB_1030_d51b;
            }
        }
        uStack14 = uStack14 + 0x1;
    } while(true);
}
