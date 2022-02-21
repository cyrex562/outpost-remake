
void __stdcall16far bad_1030_8cd2(void)

{
    return;
}


void __stdcall16far pass1_1030_8d08(ulong param_1, ushort param_2)

{
    int       *piVar1;
    undefined4 uVar2;
    ushort     uVar3;
    undefined2 uVar4;
    ulong      uVar5;
    ulong      uStack16;
    int        iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar4  = (undefined2)(param_1 >> 0x10);
        piVar1 = (int *)((int)param_1 + 0x1e);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        uVar3                                     = iStack4 * 0x6;
        uVar2                                     = *(undefined4 *)((int)param_1 + 0x1a);
        *(undefined2 *)((int)uVar2 + uVar3 + 0x4) = 0x0;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x500);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, param_2);
        uStack16 = CONCAT22(param_2, uVar3);
        uVar5    = pass1_1028_e2e0(_PTR_LOOP_1050_65e2, param_2, 0x7);
        param_2  = (ushort)(uVar5 >> 0x10);
        pass1_1030_7e5a(uStack16, uVar5 & 0xffff | (ulong)param_2 << 0x10, param_2);
        iStack4 = iStack4 + 0x1;
    }
    return;
}


void __stdcall16far pass1_1030_8d9e(ulong param_1, ushort param_2)

{
    undefined local_c[0x2];
    undefined local_a[0x2];
    undefined local_8[0x6];

    pass1_1008_3e38((ushort *)CONCAT22(param_2, local_8));
    pass1_1008_6d64((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x28)),
                    (ushort *)CONCAT22(param_2, local_8));
    pass1_1008_3e94(
      (ushort *)CONCAT22(param_2, local_8), (ushort *)CONCAT22(param_2, local_c), (ushort *)CONCAT22(param_2, local_a));
    return;
}


astruct_18 *__stdcall16far pass1_1030_8e12(astruct_18 *param_1, byte param_2)

{
    pass1_1030_8a2c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1030_8f04(ushort param_1, ushort param_2, ulong param_3, ulong param_4, uint param_5)

{
    uint       uVar1;
    uint       uVar2;
    uint       uVar3;
    uint       uVar4;
    ulong      uVar5;
    undefined2 uVar6;
    int        iStack8;
    ulong      uStack6;

    pass1_1038_53ba(param_3, 0x1);
    if((((param_5 != 0x0) || (0x1 < (uint)param_4))
        && ((pass1_1038_53ba(param_3, 0x2), param_5 != 0x0 || (0x1 < (uint)param_4))))
       && ((pass1_1038_53ba(param_3, 0x3), param_5 != 0x0 || (0x1 < (uint)param_4))))
    {
        pass1_1038_53ba(param_3, 0x4);
        uVar5 = (ulong)param_5;
        if((param_5 != 0x0) || (0x1 < (uint)param_4))
        {
            empty_1038_540a();
            uStack6 = param_4 & 0xffff | uVar5 << 0x10;
            iStack8 = 0x0;
            do
            {
                uVar3 = (uint)uVar5;
                uVar2 = (uint)param_4;
                if(0x0 < *(int *)(iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e))
                {
                    empty_1038_540a();
                    uVar6   = (undefined2)((ulong)_PTR_LOOP_1050_580e >> 0x10);
                    uVar1   = *(uint *)(iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e);
                    param_4 = (ulong)uVar1;
                    uVar4   = (int)uVar1 >> 0xf;
                    uVar5   = (ulong)uVar4;
                    if((uVar3 <= uVar4) && ((uVar3 < uVar4 || (uVar2 < uVar1))))
                    {
                        if(0x1c < iStack8)
                        {
                            return;
                        }
                        uVar2   = *(uint *)(iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e);
                        param_4 = SEXT24((int)uVar2);
                        uVar5   = param_4 >> 0x10;
                        if((long)uStack6 < (long)param_4)
                        {
                            return;
                        }
                        uStack6
                          = CONCAT22(((int)(uStack6 >> 0x10) - ((int)uVar2 >> 0xf)) - (uint)((uint)uStack6 < uVar2),
                                     (uint)uStack6 - uVar2);
                    }
                }
                iStack8 = iStack8 + 0x1;
                if(0x24 < iStack8)
                {
                    return;
                }
            } while(true);
        }
    }
    return;
}


undefined4 __stdcall16far pass1_1030_7c28(ulong param_1, ushort param_2, uint param_3, uint param_4, ushort param_5)

{
    undefined2 uVar1;
    undefined4 uVar2;

    uVar1 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x22) == 0x0)
    {
        return 0x0;
    }
    uVar2 = *(undefined4 *)((int)param_1 + 0x22);
    uVar2 = pass1_1020_bae6((ushort)uVar2, CONCAT22(param_2, (int)((ulong)uVar2 >> 0x10)), param_3, param_4, param_5);
    return uVar2;
}


void __stdcall16far pass1_1030_7c50(ulong param_1, long param_2, int param_3, uint param_4, uchar *param_5)

{
    int         *piVar1;
    code       **ppcVar2;
    ushort       uVar3;
    ulong        uVar4;
    ushort       uVar5;
    uchar       *puVar6;
    uchar       *extraout_DX;
    undefined2   extraout_DX_00;
    undefined2   uVar7;
    uchar       *extraout_DX_01;
    astruct_305 *iVar8;
    undefined2   uVar8;
    ulong        uVar9;
    undefined4  *puVar10;
    undefined4  *puStack18;

    uVar8  = (undefined2)(param_1 >> 0x10);
    iVar8  = (astruct_305 *)param_1;
    puVar6 = param_5;
    if(iVar8->field_0x1e == (undefined4 *)0x0)
    {
        mem_op_1000_179c(0x18, param_5, 0x1000);
        puVar6 = (uchar *)((uint)param_5 | param_4);
        if(puVar6 == (uchar *)0x0)
        {
            iVar8->field_0x1e = (undefined4 *)0x0;
        }
        else
        {
            struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_5, param_4), 0x5, 0x5);
            *(uint *)&iVar8->field_0x1e                = param_4;
            *(uchar **)((int)&iVar8->field_0x1e + 0x2) = extraout_DX;
            puVar6                                     = extraout_DX;
        }
    }
    if(param_3 == 0x4)
    {
        piVar1  = &iVar8->field_0x34;
        *piVar1 = *piVar1 + (int)param_2;
    }
    while(param_2 != 0x0)
    {
        uVar9   = pass1_1028_e2e0(_PTR_LOOP_1050_65e2, (ushort)puVar6, 0x6);
        uVar3   = (ushort)uVar9;
        uVar4   = uVar9 >> 0x10;
        puVar10 = iVar8->field_0x1e;
        ppcVar2 = (code **)((int)*iVar8->field_0x1e + 0xc);
        uVar5   = uVar3;
        (**ppcVar2)();
        uVar7 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, (uint)uVar4);
        puStack18 = (undefined4 *)CONCAT22(uVar7, uVar5);
        ppcVar2   = (code **)((int)*puStack18 + 0x14);
        (**ppcVar2)((int)&USHORT_1050_1028, uVar5, uVar7, param_1, puVar10, uVar9);
        puVar6  = extraout_DX_01;
        param_2 = param_2 + -0x1;
    }
    return;
}


BOOL16 __stdcall16far pass1_1030_7ea0(ulong param_1)

{
    undefined4 uVar1;
    ushort     uVar2;
    BOOL16     BVar3;

    uVar2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0xb);
    if(BVar3 != 0x0)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
        if(*(int *)((int)uVar1 + 0x12) == 0x5)
        {
            return 0x1;
        }
        BVar3 = 0x0;
    }
    return BVar3;
}


ulong __stdcall16far pass1_1030_8086(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x18), *(undefined2 *)((int)param_1 + 0x16)) & 0xffffff;
}


ushort *__stdcall16far pass1_1030_80ee(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1030_68dc(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        pass1_1000_093a((int *)param_1, (ushort)((ulong)param_1 >> 0x10), 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1030_82f0(ushort param_1, ulong param_2, ulong param_3)

{
    pass1_1028_d078(param_1, *(ulong *)((int)param_2 + 0x4), param_3);
    return;
}


void __stdcall16far pass1_1030_8308(
  ushort param_1, ushort param_2, ushort *param_3, ushort *param_4, ulong param_5, ushort param_6, ushort param_7)

{
    pass1_1028_e198(_PTR_LOOP_1050_65e2, param_3, param_4, param_5, param_6, param_7);
    return;
}


ulong __stdcall16far pass1_1030_8326(void)

{
    return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_65e2 + 0x2), *_PTR_LOOP_1050_65e2);
}


void __stdcall16far pass1_1030_8334(void)

{
    *_PTR_LOOP_1050_65e2 = 0x0;
    return;
}


void __stdcall16far pass1_1030_8344(ushort param_1, ushort param_2, ulong param_3)

{
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_3, (uint)(param_3 >> 0x10));
    return;
}


void __stdcall16far pass1_1030_8372(ulong **param_1, ulong param_2, ulong *param_3)

{
    pass1_1028_d52c(*param_1, param_2, param_3);
    return;
}


void __cdecl16far vsprintf_op_1030_840a(ulong param_1, LPSTR param_2, WORD *param_3, ushort param_4)

{
    LPCSTR pCVar1;
    ushort unaff_ES;
    uchar  in_AF;
    WORD  *args;

    if(PTR_LOOP_1050_574c != (undefined *)0x0)
    {
        args = param_3;
        if(PTR_LOOP_1050_5750 == (undefined *)0x0)
        {
            param_2 = (LPSTR)&PTR_LOOP_1050_1000;
            pCVar1  = &stack0x0008;
            pass1_1000_2b3c((ushort)s_simres_out_1050_5758,
                            (ushort)&USHORT_1050_1050,
                            0x5756,
                            (ushort)&USHORT_1050_1050,
                            param_4,
                            (int)&stack0xfffe);
            _PTR_LOOP_1050_5752 = CONCAT22(param_4, pCVar1);
            PTR_LOOP_1050_5750  = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
        }
        wvsprintf16(param_2, &stack0x0008, args);
        pass1_1000_2b5c((ushort)_PTR_LOOP_1050_5752,
                        (ushort)((ulong)_PTR_LOOP_1050_5752 >> 0x10),
                        0x5763,
                        (ushort)&USHORT_1050_1050,
                        unaff_ES,
                        (int)&stack0xfffe,
                        0x1000,
                        (ushort)param_3);
        pass1_1000_2f48(_PTR_LOOP_1050_5752, (int)&stack0xfffe, unaff_ES, 0x1000, (ushort)param_3, in_AF);
    }
    return;
}


void __stdcall16far
pass1_1030_861a(ushort param_1, ushort param_2, ushort param_3, uint param_4, uint param_5, ushort param_6)

{
    undefined4 *puStack6;

    pass1_1030_8854(param_1, param_2, param_3, param_6);
    puStack6 = (undefined4 *)CONCAT22(param_5, param_4);
    if((param_5 | param_4) == 0x0)
    {
        *(undefined4 *)(param_1 + 0xa) = 0x0;
    }
    else
    {
        *(undefined4 *)(param_1 + 0xa) = *puStack6;
    }
    return;
}


void __stdcall16far
pass1_1030_8660(ulong param_1, ulong *param_2, ushort param_3, uint param_4, uint param_5, ushort param_6, int param_7)

{
    uint   uVar1;
    ushort uVar2;
    ushort uVar3;
    ulong *puStack6;

    uVar2 = (ushort)param_1;
    uVar3 = (ushort)(param_1 >> 0x10);
    pass1_1030_8854(uVar2, uVar3, param_3, param_6);
    puStack6 = (ulong *)CONCAT22(param_5, param_4);
    uVar1    = param_5 | param_4;
    if(uVar1 == 0x0)
    {
        pass1_1030_8854(uVar2, uVar3, 0x0, param_6);
        puStack6 = (ulong *)CONCAT22(uVar1, param_4);
        uVar1    = uVar1 | param_4;
        if(uVar1 == 0x0)
        {
            pass1_1030_878c((long *)param_1, param_7, param_6);
            pass1_1030_8854(uVar2, uVar3, 0x0, param_6);
            puStack6 = (ulong *)CONCAT22(uVar1, param_4);
            if((uVar1 | param_4) == 0x0)
            {
                return;
            }
        }
        *(ushort *)((int)puStack6 + 0x4) = param_3;
        *puStack6                        = *param_2;
        pass1_1030_8834((uint *)param_1, param_7, param_6);
    }
    else
    {
        *puStack6 = *param_2;
    }
    return;
}


void __stdcall16far pass1_1030_871e(long *param_1, ulong *param_2, ushort param_3, int param_4, ushort param_5)

{
    int         *piVar1;
    astruct_681 *iVar2;
    uint         uVar2;

    uVar2 = (uint)((ulong)param_1 >> 0x10);
    iVar2 = (astruct_681 *)param_1;
    if(*param_1 == 0x0)
    {
        pass1_1030_878c((long *)((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10), param_4, param_5);
    }
    piVar1                                                    = &iVar2->field_0xe;
    *piVar1                                                   = *piVar1 + 0x1;
    *(ushort *)((int)*param_1 + iVar2->field_0xe * 0x6 + 0x4) = param_3;
    *(ulong *)(iVar2->field_0xe * 0x6 + (int)*param_1)        = *param_2;
    return;
}


void __stdcall16far pass1_1030_877c(uint *param_1, int param_2, ushort param_3)

{
    pass1_1030_8834(param_1, param_2, param_3);
    return;
}


void __stdcall16far pass1_1030_8834(uint *param_1, int param_2, ushort param_3)

{
    undefined4 uVar1;
    ushort     uVar2;

    uVar2 = (ushort)((ulong)param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x2);
    pass1_1000_4aea(*param_1,
                    (uint)uVar1,
                    (int)((ulong)uVar1 >> 0x10),
                    0x6,
                    (uchar *)0x888e,
                    (int)&stack0xfffe,
                    param_2,
                    uVar2,
                    0x1000,
                    param_3);
    return;
}


void __stdcall16far pass1_1030_8854(ushort param_1, ushort param_2, ushort param_3, ushort param_4)

{
    undefined4 uVar1;
    undefined4 local_c;
    ushort     uStack8;

    uStack8 = param_3;
    local_c = 0x0;
    uVar1   = *(undefined4 *)(param_1 + 0x2);
    pass1_1000_49c6((ushort)&local_c,
                    param_4,
                    *_param_1,
                    (uint)uVar1,
                    (uint)((ulong)uVar1 >> 0x10),
                    0x6,
                    (uchar *)0x888e,
                    (int)&stack0xfffe);
    return;
}


ushort __cdecl16far pass1_1030_888e(ulong param_1, ulong param_2)

{
    int       *piVar1;
    int        iVar2;
    undefined2 uVar3;
    undefined2 uVar4;

    uVar3  = (undefined2)(param_1 >> 0x10);
    iVar2  = *(int *)((int)param_1 + 0x4);
    uVar4  = (undefined2)(param_2 >> 0x10);
    piVar1 = (int *)((int)param_2 + 0x4);
    if(*piVar1 != iVar2 && iVar2 <= *piVar1)
    {
        return 0xffff;
    }
    if(*(int *)((int)param_2 + 0x4) < *(int *)((int)param_1 + 0x4))
    {
        return 0x1;
    }
    return 0x0;
}


void __stdcall16far pass1_1030_88ce(ushort *param_1, ulong param_2, ulong param_3, ushort param_4)

{
    uchar       *puVar1;
    uchar       *puVar2;
    astruct_354 *iVar4;
    undefined2   uVar3;
    ulong        uVar4;
    ushort      *puStack38;
    int          iStack34;
    undefined    local_20[0x2];
    int          local_1e;
    int          local_1c;
    undefined    local_1a[0x6];
    undefined    local_14[0x6];
    undefined4   uStack14;
    undefined4   uStack10;
    int          iStack6;
    undefined2   uStack4;

    uVar3            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_354 *)param_1;
    *param_1         = 0x389a;
    iVar4->field_0x2 = 0x1008;
    pass1_1030_84ae((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x4);
    iVar4->field_0x24 = param_3;
    puStack38         = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x28);
    pass1_1008_6c90((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x28));
    *(undefined4 *)&iVar4->field_0x34 = 0x0;
    *param_1                          = 0x8e38;
    iVar4->field_0x2                  = 0x1030;
    struct_1030_8544((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x4), (ushort *)param_2);
    uVar4    = pass1_1008_4772(iVar4->field_0x12);
    uStack4  = (undefined2)(uVar4 >> 0x10);
    iStack6  = (int)uVar4;
    uStack10 = *(undefined4 *)(iStack6 + 0x4);
    uStack14 = *(undefined4 *)(iStack6 + 0x8);
    pass1_1008_3e54((ushort *)CONCAT22(param_4, local_14), 0x0, (int)uStack14 - 0x1, (int)uStack10 - 0x1);
    pass1_1008_3e54((ushort *)CONCAT22(param_4, local_1a), 0x0, 0x0, 0x0);
    pass1_1008_6d18(puStack38, (ushort *)CONCAT22(param_4, local_14), (ushort *)CONCAT22(param_4, local_1a));
    pass1_1008_6d64(puStack38, (ushort *)CONCAT22(param_4, local_1a));
    pass1_1008_3eb4((ushort *)CONCAT22(param_4, local_1a),
                    (ushort *)CONCAT22(param_4, local_20),
                    (ushort *)CONCAT22(param_4, &local_1e),
                    (ushort *)CONCAT22(param_4, &local_1c));
    puVar1            = (uchar *)((ulong)((long)local_1e * (long)local_1c) >> 0x10);
    uVar4             = (long)local_1e * (long)local_1c & 0xffff;
    iVar4->field_0x34 = (int)uVar4;
    iVar4->field_0x36 = puVar1;
    for(iStack34 = 0x0; iStack34 < 0x5; iStack34 = iStack34 + 0x1)
    {
        mem_op_1000_179c(0x10, puVar1, 0x1000);
        puVar2 = (uchar *)((uint)puVar1 | (uint)uVar4);
        if(puVar2 == (uchar *)0x0)
        {
            *(undefined4 *)(&iVar4[0x1].field_0x0 + iStack34 * 0x4) = 0x0;
        }
        else
        {
            pass1_1030_85be((long *)(uVar4 & 0xffff | ZEXT24(puVar1) << 0x10), 0x19, 0x64, uVar3, param_4);
            *(undefined2 *)(&iVar4[0x1].field_0x0 + iStack34 * 0x4) = (int)uVar4;
            (&iVar4[0x1].field_0x2)[iStack34 * 0x2]                 = puVar2;
        }
        puVar1 = puVar2;
    }
    return;
}


void __stdcall16far pass1_1030_6b86(ulong param_1, undefined2 param_2, undefined2 param_3)

{
    code     **ppcVar1;
    ulong      uVar2;
    undefined2 extraout_DX;
    undefined2 uVar3;
    uint       extraout_DX_00;
    int        iVar4;
    undefined2 uVar5;
    ulong      uStack12;
    ulong      uStack8;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0x1e) == 0x0)
    {
        param_2 = 0x0;
        uVar3   = 0x0;
    }
    else
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x1e) + 0x10);
        (**ppcVar1)();
        uVar3 = extraout_DX;
    }
    uStack8 = CONCAT22(uVar3, param_2);
    for(uStack12 = 0x0; uStack12 < uStack8; uStack12 = uStack12 + 0x1)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x1e) + 0x4);
        uVar2   = uStack8;
        (**ppcVar1)(param_3, *(undefined4 *)(iVar4 + 0x1e));
        if((extraout_DX_00 | (uint)uVar2) != 0x0)
        {
            param_3 = SUB42(&USHORT_1050_1028, 0x0);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (uint)uVar2, extraout_DX_00);
        }
    }
    return;
}


void __stdcall16far pass1_1030_6c1a(ulong param_1, int param_2)

{
    int       *piVar1;
    int        iVar2;
    int        iVar3;
    undefined2 uVar4;

    uVar4                  = (undefined2)(param_1 >> 0x10);
    iVar3                  = (int)param_1;
    iVar2                  = *(int *)(iVar3 + 0x32);
    *(int *)(iVar3 + 0x32) = param_2;
    piVar1                 = (int *)(iVar3 + 0x34);
    *piVar1                = *piVar1 + (param_2 - iVar2);
    iVar2                  = *(int *)(iVar3 + 0x32);
    if(iVar2 < 0x0)
    {
        iVar2 = 0x0;
    }
    *(int *)(iVar3 + 0x32) = iVar2;
    return;
}


void __stdcall16far pass1_1030_6c4c(ulong param_1, int param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = *(int *)((int)param_1 + 0x32);
    if(param_2 < iVar1)
    {
        iVar1 = param_2;
    }
    *(int *)((int)param_1 + 0x34) = iVar1;
    return;
}


ulong __stdcall16far pass1_1030_6d4e(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    undefined2 uVar1;
    ushort     uStack6;
    ushort     uStack4;

    uStack6 = 0x0;
    uStack4 = 0x0;
    uVar1   = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x36) != 0x0)
    {
        pass1_1010_9092(*(ulong *)((int)param_1 + 0x36), param_2, param_4);
        uStack6 = param_2;
        uStack4 = param_3;
    }
    return CONCAT22(uStack4, uStack6);
}


void __stdcall16far pass1_1030_6d80(ulong param_1, ulong param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_299 *iVar4;
    undefined2   uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    iVar4  = (astruct_299 *)param_1;
    puVar1 = *(undefined4 **)&iVar4->field_0x36;
    uVar2  = *(uint *)((int)&iVar4->field_0x36 + 0x2);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    iVar4->field_0x36 = param_2;
    return;
}


void __stdcall16far pass1_1030_6ddc(ulong param_1)

{
    ushort uVar1;
    BOOL16 BVar2;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1e);
    if(BVar2 != 0x0)
    {
        pass1_1030_d0c6(*(ulong *)((int)param_1 + 0x1a));
        return;
    }
    return;
}


void __stdcall16far pass1_1030_6e14(ulong param_1)

{
    undefined4 uVar1;
    ushort     uVar2;
    BOOL16     BVar3;

    uVar2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x1e);
    if(BVar3 != 0x0)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
        pass1_1030_d102((int)uVar1, (ushort)((ulong)uVar1 >> 0x10));
        return;
    }
    return;
}


void __stdcall16far pass1_1030_6e9c(ulong param_1, long param_2, int param_3)

{
    code       **ppcVar1;
    uint         uVar2;
    uint         uVar3;
    ulong        uVar4;
    undefined2   extraout_DX;
    uint         extraout_DX_00;
    uint         uVar5;
    astruct_301 *iVar6;
    undefined2   uVar6;
    ushort       unaff_SS;
    ulong        uStack10;
    ulong        uStack6;

    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar6 = (astruct_301 *)param_1;
    uVar2 = *(uint *)((int)&iVar6->field_0x1e + 0x2) | *(uint *)&iVar6->field_0x1e;
    if(uVar2 != 0x0)
    {
        ppcVar1 = (code **)((int)*iVar6->field_0x1e + 0x10);
        (**ppcVar1)();
        uStack6 = CONCAT22(extraout_DX, uVar2);
        for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
        {
            ppcVar1 = (code **)((int)*iVar6->field_0x1e + 0x4);
            uVar4   = uStack6;
            (**ppcVar1)();
            uVar2 = (uint)uVar4;
            uVar5 = extraout_DX_00 | uVar2;
            if(uVar5 != 0x0)
            {
                uVar3 = uVar2;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
                if(*(int *)(uVar3 + 0xc) == param_3)
                {
                    param_2 = param_2 + -0x1;
                    pass1_1028_e332(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00, unaff_SS);
                    ppcVar1 = (code **)((int)*iVar6->field_0x1e + 0x8);
                    (**ppcVar1)((int)&USHORT_1050_1028, iVar6->field_0x1e, 0x0, uStack10);
                }
                if((param_2._2_2_ | (uint)param_2) == 0x0)
                {
                    return;
                }
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_6f5a(ulong param_1, ushort param_2)

{
    ushort uVar1;
    BOOL16 BVar2;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x4);
    if(BVar2 != 0x0)
    {
        pass1_1028_6302(*(ulong *)((int)param_1 + 0x1a), param_2);
    }
    return;
}


ulong __stdcall16far struct_op_1030_73a8(ulong param_1)

{
    undefined4 uVar1;
    undefined2 in_AX;
    undefined2 in_DX;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(long *)(iVar2 + 0x16) == 0x0)
    {
        return 0x0;
    }
    if(*(long *)(iVar2 + 0x1a) == 0x0)
    {
        uVar1 = *(undefined4 *)(iVar2 + 0x16);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        *(undefined2 *)(iVar2 + 0x1a) = in_AX;
        *(undefined2 *)(iVar2 + 0x1c) = in_DX;
    }
    return CONCAT22(*(undefined2 *)(iVar2 + 0x1c), *(undefined2 *)(iVar2 + 0x1a));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_73ee(ulong param_1, ulong param_2, ushort param_3)

{
    astruct_294 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)(param_1 >> 0x10);
    iVar1             = (astruct_294 *)param_1;
    iVar1->field_0x2a = param_2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_2, (uint)(param_2 >> 0x10));
    iVar1->field_0x2e = (int)param_2;
    iVar1->field_0x30 = param_3;
    return;
}


ulong __stdcall16far pass1_1030_5b5c(int param_1, ushort param_2)

{
    return CONCAT22(param_2, param_1 + 0x14);
}


void __stdcall16far pass1_1030_5bec(ulong param_1)

{
    _PTR_LOOP_1050_5736 = param_1;
    pass1_1000_54a0(param_1, 0x0, 0x24);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_5c0e(void)

{
    _PTR_LOOP_1050_5736 = 0x0;
    return;
}

ushort *__stdcall16far pass1_1030_5d0a(ushort *param_1)

{
    undefined2 uVar1;

    struct_1030_17ce(param_1, 0x1, 0x4);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x10) = 0x0;
    *param_1                             = 0x613e;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1030;
    return param_1;
}


ushort *__stdcall16far pass1_1030_5d3c(ushort *param_1, ulong param_2, uint param_3, uchar *param_4)

{
    ushort uVar1;

    pass1_1030_183c(param_1, 0x1, 0x4, 0x1000000, param_2, param_3, param_4);
    uVar1                                = (ushort)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x10) = 0x0;
    *param_1                             = 0x613e;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1030;
    return param_1;
}


void __stdcall16far pass1_1030_5d78(ushort *param_1)

{
    uint        uVar1;
    astruct_18 *paVar2;
    int         iVar3;
    undefined2  uVar4;

    uVar4                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                        = (int)param_1;
    *param_1                     = 0x613e;
    *(undefined2 *)(iVar3 + 0x2) = 0x1030;
    paVar2                       = *(astruct_18 **)(iVar3 + 0x10);
    uVar1                        = *(uint *)(iVar3 + 0x12);
    if((uVar1 | (uint)paVar2) != 0x0)
    {
        pass1_1030_8480((astruct_18 **)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10));
        fn_ptr_1000_17ce(paVar2, 0x1000);
    }
    pass1_1030_18b2(param_1);
    return;
}


void __stdcall16far pass1_1030_5fe2(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x10) = param_2;
    return;
}


void __stdcall16far pass1_1030_61b0(uint *param_1)

{
    uint        uVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar1 = *(uint *)(iVar4 + 0x2);
    if((uVar1 | (uint)(undefined4 *)*param_1) != 0x0)
    {
        ppcVar3 = (code **)*(undefined4 *)*param_1;
        (**ppcVar3)();
    }
    puVar2 = (undefined4 *)*(uint *)(iVar4 + 0x4);
    uVar1  = *(uint *)(iVar4 + 0x6);
    if((uVar1 | (uint)puVar2) != 0x0)
    {
        ppcVar3 = (code **)*puVar2;
        (**ppcVar3)();
    }
    _PTR_LOOP_1050_5740 = 0x0;
    return;
}


void __stdcall16far pass1_1030_61fe(
  ulong param_1, ulong param_2, ulong param_3, long param_4, ushort param_5, ushort param_6, ushort param_7)

{
    pass1_1030_677a(param_1, param_4, param_7);
    pass1_1030_8aa0(CONCAT22(param_6, param_5), param_2, (ushort *)param_3, param_6, param_7);
    return;
}


void __stdcall16far
pass1_1030_627e(ushort param_1, uint param_2, uint param_3, ulong param_4, ushort *param_5, long param_6)

{
    undefined4 local_12[0x2];
    ulong      uStack10;
    undefined4 uStack6;

    uStack6 = 0x0;
    pass1_1030_677a(param_4, param_6, param_1);
    uStack10 = CONCAT22(param_3, param_2);
    if((param_3 | param_2) != 0x0)
    {
        pass1_1030_8b00(uStack10, param_5, (ushort *)CONCAT22(param_1, local_12), param_1);
    }
    return;
}


void __stdcall16far pass1_1030_64ce(
  ushort param_1, uint param_2, uint param_3, ulong param_4, ushort *param_5, long param_6, ulong *param_7)

{
    ulong *puVar1;
    uint   uVar2;
    ulong  local_e;
    ulong  uStack10;
    ulong  uStack6;

    uStack6 = 0x0;
    pass1_1030_677a(param_4, param_6, param_1);
    uStack10 = CONCAT22(param_3, param_2);
    uVar2    = param_3 | param_2;
    if(uVar2 != 0x0)
    {
        puVar1 = &local_e;
        pass1_1030_8b00(uStack10, param_5, (ushort *)CONCAT22(param_1, puVar1), param_1);
        uStack6 = *puVar1;
    }
    *param_7 = uStack6;
    return;
}


void __stdcall16far pass1_1030_6522(ulong *param_1, ulong param_2, ulong param_3, ushort param_4)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    uint        uVar3;
    uchar      *extraout_DX;
    uchar      *puVar4;
    uint        extraout_DX_00;
    uint        extraout_DX_01;
    undefined2  uVar5;
    undefined   local_64[0xc];
    ushort      uStack88;
    undefined4  local_40;
    undefined4  uStack60;
    ushort      uStack56;
    undefined4 *puStack54;
    uchar      *puStack52;
    undefined4 *puStack50;
    uchar      *puStack48;
    ushort      uStack46;
    int         iStack44;
    undefined   local_2a[0x2];
    int         local_28;
    int         local_26;
    ushort      local_24;
    undefined   local_22[0x2];
    undefined   local_20[0x2];
    ushort      local_1e;
    ushort      local_1c;
    ushort      local_1a;
    undefined   local_18[0x6];
    undefined   local_12[0x6];
    undefined   local_c[0x6];
    ulong       uStack6;

    uVar5     = (undefined2)((ulong)param_1 >> 0x10);
    puVar2    = *(undefined4 **)param_1;
    puVar4    = *(uchar **)((int)param_1 + 0x2);
    puStack54 = puVar2;
    puStack52 = puVar4;
    puStack50 = puVar2;
    puStack48 = puVar4;
    if(((uint)puVar4 | (uint)puVar2) != 0x0)
    {
        ppcVar1 = (code **)*puVar2;
        (**ppcVar1)();
        puVar4 = extraout_DX;
    }
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    puStack54 = puVar2;
    puStack52 = puVar4;
    if(((uint)puVar4 | (uint)puVar2) == 0x0)
    {
        puVar2 = (undefined4 *)0x0;
        uVar3  = 0x0;
    }
    else
    {
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(puVar4, puVar2), 0x5, 0x5);
        uVar3 = extraout_DX_00;
    }
    *(undefined4 **)param_1       = puVar2;
    *(uint *)((int)param_1 + 0x2) = uVar3;
    pass1_1030_677a((ulong)param_1, param_3, param_4);
    uStack6 = CONCAT22(uVar3, puVar2);
    if((uVar3 | (uint)puVar2) != 0x0)
    {
        pass1_1008_3e38((ushort *)CONCAT22(param_4, local_c));
        pass1_1008_3e38((ushort *)CONCAT22(param_4, local_12));
        pass1_1008_3e38((ushort *)CONCAT22(param_4, local_18));
        pass1_1008_6d3e((ushort *)param_2, (ushort *)CONCAT22(param_4, local_12), (ushort *)CONCAT22(param_4, local_c));
        pass1_1008_3eb4((ushort *)CONCAT22(param_4, local_c),
                        (ushort *)CONCAT22(param_4, &local_1e),
                        (ushort *)CONCAT22(param_4, &local_1c),
                        (ushort *)CONCAT22(param_4, &local_1a));
        pass1_1008_3eb4((ushort *)CONCAT22(param_4, local_12),
                        (ushort *)CONCAT22(param_4, &local_24),
                        (ushort *)CONCAT22(param_4, local_22),
                        (ushort *)CONCAT22(param_4, local_20));
        pass1_1008_6d64((ushort *)param_2, (ushort *)CONCAT22(param_4, local_18));
        pass1_1008_3eb4((ushort *)CONCAT22(param_4, local_18),
                        (ushort *)CONCAT22(param_4, local_2a),
                        (ushort *)CONCAT22(param_4, &local_28),
                        (ushort *)CONCAT22(param_4, &local_26));
        if(local_24 == local_1e)
        {
            iStack44 = 0x0;
            for(uStack46 = local_1c; uVar3 = local_28 + local_1c, (int)uStack46 < (int)uVar3; uStack46 = uStack46 + 0x1)
            {
                for(uStack56 = local_1a; (int)uStack56 < (int)(local_26 + local_1a); uStack56 = uStack56 + 0x1)
                {
                    uStack88 = local_1e;
                    pass1_1008_3e54((ushort *)CONCAT13((char)(param_4 >> 0x8), CONCAT12((char)param_4, local_64)),
                                    local_1e,
                                    uStack46,
                                    uStack56);
                    pass1_1030_8b00(
                      uStack6, (ushort *)CONCAT22(param_4, local_64), (ushort *)CONCAT22(param_4, &local_40), param_4);
                    uStack60 = local_40;
                    iStack44 = iStack44 + 0x1;
                    ppcVar1  = (code **)((int)*(undefined4 *)*param_1 + 0x8);
                    (**ppcVar1)();
                }
            }
            ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x10);
            (**ppcVar1)(0x1008, *param_1);
            if((extraout_DX_01 | uVar3) != 0x0)
            {
                return;
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_66de(ulong param_1, ulong param_2, ushort param_3)

{
    ulong     uVar1;
    undefined local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)((int)param_1 + 0x4));
    while(true)
    {
        uVar1 = pass1_1008_5b12(local_a, param_3);
        if(uVar1 == 0x0)
            break;
        pass1_1030_8bac(uVar1, (ushort)param_2);
    }
    return;
}


void __stdcall16far pass1_1030_671c(ulong   param_1,
                                    ulong   param_2,
                                    ushort *param_3,
                                    long    param_4,
                                    ushort  param_5,
                                    ushort  param_6,
                                    int     param_7,
                                    ushort  param_8)

{
    pass1_1030_677a(param_1, param_4, param_8);
    pass1_1030_8bdc(CONCAT22(param_6, param_5), param_2, param_3, param_7, param_8);
    return;
}


void __stdcall16far pass1_1030_6740(ulong param_1, ushort param_2, int param_3)

{
    ulong     uVar1;
    undefined local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_2, local_a), *(ulong *)((int)param_1 + 0x4));
    while(true)
    {
        uVar1 = pass1_1008_5b12(local_a, param_2);
        if(uVar1 == 0x0)
            break;
        pass1_1030_8c38(uVar1, param_3, param_2);
    }
    return;
}


void __stdcall16far pass1_1030_677a(ulong param_1, long param_2, ushort param_3)

{
    undefined *puVar1;
    uint       extraout_DX;
    undefined2 uVar2;
    undefined  local_a[0x8];

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x4) == 0x0)
    {
        return;
    }
    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)((int)param_1 + 0x4));
    do
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_3);
        if((extraout_DX | (uint)puVar1) == 0x0)
        {
            return;
        }
    } while(*(long *)(puVar1 + 0x24) != param_2);
    return;
}


void __stdcall16far pass1_1030_69cc(ulong param_1, uint param_2, uint param_3, ushort param_4)

{
    ushort     uVar1;
    BOOL16     BVar2;
    int        iVar3;
    undefined2 uVar4;
    ulong      uVar5;

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0x3e) != 0x0)
    {
        return;
    }
    if((*(long *)(iVar3 + 0x22) != 0x0) && (pass1_1020_ba94(*(long **)(iVar3 + 0x22)), (param_3 | param_2) != 0x0))
    {
        return;
    }
    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x4);
    if((BVar2 != 0x0)
       && (uVar5 = pass1_1028_67d4(*(ulong *)(iVar3 + 0x1a), param_4), ((uint)(uVar5 >> 0x10) | (uint)uVar5) != 0x0))
    {
        return;
    }
    return;
}


void __stdcall16far pass1_1030_4bbe(ushort param_1, ushort param_2, ulong param_3, int param_4)

{
    undefined4  *puVar1;
    undefined4  *puVar2;
    undefined2   uVar3;
    int          iVar4;
    astruct_117 *iVar5;
    undefined4  *puVar5;
    undefined4  *puVar6;
    uint         uVar7;

    uVar7 = (uint)(param_3 >> 0x10);
    iVar5 = (astruct_117 *)param_3;
    if(iVar5->field_0x12 == 0x0)
    {
        pass1_1030_4f5a(param_1, param_2, param_3 & 0xffff | (ulong)uVar7 << 0x10);
    }
    puVar6 = &iVar5->field_0x16;
    uVar3  = *(undefined2 *)((int)&iVar5->field_0x12 + 0x2);
    puVar5 = (undefined4 *)(*(int *)&iVar5->field_0x12 + param_4 * 0x98);
    for(iVar4 = 0x26; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
    {
        puVar2  = puVar6;
        puVar6  = puVar6 + 0x1;
        puVar1  = puVar5;
        puVar5  = puVar5 + 0x1;
        *puVar2 = *puVar1;
    }
    return;
}


void __stdcall16far pass1_1030_4c06(ulong param_1, int param_2, uint16_t param_3, ushort param_4)

{
    undefined4 *puVar1;
    undefined4 *puVar2;
    undefined2  uVar3;
    undefined4 *puVar4;
    int         iVar5;
    undefined4 *puVar6;
    uint        uVar7;

    uVar7 = (uint)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    if(*(long *)(iVar5 + 0x15c) == 0x0)
    {
        pass1_1030_5044(param_1 & 0xffff | (ulong)uVar7 << 0x10, param_4, param_3);
    }
    puVar4 = (undefined4 *)(iVar5 + 0xae);
    uVar3  = *(undefined2 *)(iVar5 + 0x15e);
    puVar6 = (undefined4 *)(*(int *)(iVar5 + 0x15c) + param_2 * 0xae);
    for(iVar5 = 0x2b; iVar5 != 0x0; iVar5 = iVar5 + -0x1)
    {
        puVar2  = puVar4;
        puVar4  = puVar4 + 0x1;
        puVar1  = puVar6;
        puVar6  = puVar6 + 0x1;
        *puVar2 = *puVar1;
    }
    *(undefined2 *)puVar4 = *(undefined2 *)puVar6;
    return;
}


void __stdcall16far
pass1_1030_4c52(ushort param_1, ushort param_2, ulong param_3, ulong param_4, uint param_5, ushort param_6)

{
    ushort     uVar1;
    int        iVar2;
    int        iVar3;
    undefined2 uVar4;
    char      *pcStack8;
    int        iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar1    = pass1_1000_47a4(param_4, 0x1050518a, param_6);
        pcStack8 = (char *)CONCAT22(param_5, uVar1);
        if((param_5 | uVar1) == 0x0)
            break;
        if(*pcStack8 != '\"')
        {
            iVar2 = pass1_1000_3e2c(CONCAT22(param_5, uVar1));
            iVar3 = (int)param_3;
            uVar4 = (undefined2)(param_3 >> 0x10);
            if(iStack4 < 0x25)
            {
                *(int *)(iStack4 * 0x4 + iVar3)        = iVar2;
                *(uint *)(iStack4 * 0x4 + iVar3 + 0x2) = param_5;
            }
            else
            {
                if(iStack4 == 0x25)
                {
                    *(int *)(iVar3 + 0x94) = iVar2;
                }
                else
                {
                    if(iStack4 == 0x26)
                    {
                        *(int *)(iVar3 + 0x96) = iVar2;
                    }
                    else
                    {
                        if(iStack4 == 0x27)
                        {
                            *(int *)(iVar3 + 0x98) = iVar2;
                        }
                        else
                        {
                            if(iStack4 == 0x28)
                            {
                                *(int *)(iVar3 + 0x9a) = iVar2;
                            }
                            else
                            {
                                if(iStack4 == 0x29)
                                {
                                    *(int *)(iVar3 + 0x9c) = iVar2;
                                }
                                else
                                {
                                    if(iStack4 == 0x2a)
                                    {
                                        *(int *)(iVar3 + 0x9e) = iVar2;
                                    }
                                    else
                                    {
                                        if(iStack4 == 0x2b)
                                        {
                                            *(int *)(iVar3 + 0xa0) = iVar2;
                                        }
                                        else
                                        {
                                            if(iStack4 == 0x2c)
                                            {
                                                *(int *)(iVar3 + 0xa2) = iVar2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            iStack4 = iStack4 + 0x1;
        }
        param_4 = 0x0;
    }
    return;
}


void __stdcall16far pass1_1030_4d3a(uint param_1, ushort param_2, ushort param_3, ulong param_4, ulong param_5)

{
    ushort       uVar1;
    int          iVar2;
    astruct_118 *iVar3;
    undefined2   uVar3;
    ushort       unaff_SS;
    char        *pcStack8;
    int          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar1    = pass1_1000_47a4(param_5, 0x1050518a, unaff_SS);
        pcStack8 = (char *)CONCAT22(param_1, uVar1);
        if((param_1 | uVar1) == 0x0)
            break;
        if(*pcStack8 != '\"')
        {
            iVar2 = pass1_1000_3e2c(CONCAT22(param_1, uVar1));
            iVar3 = (astruct_118 *)param_4;
            uVar3 = (undefined2)(param_4 >> 0x10);
            if(iStack4 < 0x25)
            {
                *(int *)(&iVar3->field_0x0 + iStack4 * 0x4)  = iVar2;
                *(uint *)(&iVar3->field_0x2 + iStack4 * 0x4) = param_1;
            }
            else
            {
                if(iStack4 == 0x25)
                {
                    iVar3->field_0x94 = iVar2;
                }
                else
                {
                    if(iStack4 == 0x26)
                    {
                        iVar3->field_0x96 = iVar2;
                    }
                }
            }
            iStack4 = iStack4 + 0x1;
        }
        param_5 = 0x0;
    }
    return;
}


void __stdcall16far pass1_1030_4e34(ushort param_1, ushort param_2, long param_3, char *param_4)

{
    while(param_3 != 0x0)
    {
        if((*param_4 == '\r') || (*param_4 == '\n'))
        {
            *param_4 = '\0';
        }
        param_4 = (char *)((ulong)param_4 & 0xffff0000 | (ulong)((int)param_4 + 0x1));
        param_3 = param_3 + -0x1;
    }
    return;
}


ulong __stdcall16far pass1_1030_5164(ulong param_1, ULONG param_2, ushort param_3)

{
    uint      uVar1;
    uint      uVar2;
    long      lVar3;
    undefined local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)((int)param_1 + 0x568));
    do
    {
        lVar3 = pass1_1008_5b12(local_a, param_3);
        if(lVar3 == 0x0)
        {
            return (ulong)param_2;
        }
        uVar1 = (int)param_1 + 0x168;
        unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)uVar1), *(char **)((int)lVar3 + 0x4));
        pass1_1000_3cea(param_1 & 0xffff0000 | (ulong)uVar1, param_2);
        uVar2 = dos3_call_1000_51aa((ushort)&stack0xfffe);
    } while(uVar2 != 0x0);
    return param_1 & 0xffff0000 | (ulong)uVar1;
}


void __cdecl16far pass1_1030_51eb(void)

{
    undefined2 unaff_SS;

    pass1_1030_3b28(unaff_SS);
    return;
}


ushort __stdcall16far pass1_1030_5260(ulong param_1, ushort param_2, ushort param_3)

{
    undefined4  uVar1;
    code      **ppcVar2;
    undefined4 *puStack6;

    uVar1 = *(undefined4 *)((int)param_1 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    puStack6 = (undefined4 *)CONCAT22(param_3, param_2);
    ppcVar2  = (code **)((int)*puStack6 + 0x14);
    (**ppcVar2)();
    return 0x1;
}


void __stdcall16far pass1_1030_53f4(ulong param_1, ushort param_2, ushort param_3, uchar param_4)

{
    undefined4 uVar1;
    uint       uVar2;
    int        iVar3;
    undefined2 uVar4;
    ulong      uVar5;
    byte       bStack291;
    undefined  local_11e[0x10e];
    undefined4 uStack16;
    undefined4 uStack12;

    uVar4          = (undefined2)(param_1 >> 0x10);
    iVar3          = (int)param_1;
    uStack12       = *(undefined4 *)(iVar3 + 0x108);
    uStack12._3_1_ = (char)((ulong)uStack12 >> 0x18);
    if(uStack12._3_1_ == -0x1)
    {
        uVar5 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2, param_2, (byte)((ulong) * (undefined4 *)(iVar3 + 0x108) >> 0x18));
        param_2 = (ushort)(uVar5 >> 0x10);
    }
    else
    {
        uStack16       = (ulong *)*(undefined4 *)(iVar3 + 0x108);
        uStack16._3_1_ = (char)((ulong)uStack16 >> 0x18);
        if(uStack16._3_1_ == '\x03')
        {
            pass1_1028_e44a(_PTR_LOOP_1050_65e2, *(long *)(iVar3 + 0x108), param_3);
        }
        else
        {
            uVar1 = *(undefined4 *)(iVar3 + 0x108);
            pass1_1028_e372(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10), param_3);
        }
    }
    uStack12       = *(undefined4 *)(iVar3 + 0x108);
    uStack12._3_1_ = (char)((ulong)uStack12 >> 0x18);
    if(uStack12._3_1_ != '\x03')
    {
        pass1_1030_521c((astruct_100 *)CONCAT13((char)(param_3 >> 0x8), CONCAT12((char)param_3, local_11e)),
                        *(ulong *)(iVar3 + 0x108),
                        param_3,
                        param_4);
        uStack16 = *_PTR_LOOP_1050_5748;
        fn_ptr_1028_d566(uStack16, (ulong *)CONCAT22(param_3, local_11e));
        bStack291 = (byte)((ulong) * (undefined4 *)(iVar3 + 0x108) >> 0x18);
        uVar2     = (uint)bStack291;
        if(bStack291 == 0x2)
        {
            uVar1 = *(undefined4 *)(iVar3 + 0x108);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
            pass1_1010_82f8(_PTR_LOOP_1050_14cc, **(ushort **)(uVar2 + 0x10));
        }
    }
    return;
}


void __stdcall16far pass1_1030_5a52(ulong param_1, ulong *param_2, ulong *param_3)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2    = (undefined2)(param_1 >> 0x10);
    uVar1    = *(undefined4 *)((int)param_1 + 0x10);
    *param_3 = *(ulong *)((int)uVar1 + 0xe);
    uVar1    = *(undefined4 *)((int)param_1 + 0x10);
    *param_2 = *(ulong *)((int)uVar1 + 0x12);
    return;
}


void __stdcall16far pass1_1030_5a80(ulong param_1, ulong param_2, ushort param_3)

{
    ulong     *puVar1;
    ushort     uVar2;
    ulong      uVar3;
    undefined  local_20[0xc];
    ulong      local_14;
    undefined4 uStack14;
    undefined4 uStack10;
    int        iStack6;
    undefined2 uStack4;

    uVar2                           = (ushort)(param_1 >> 0x10);
    *(ulong *)((int)param_1 + 0x10) = param_2;
    uVar3                           = pass1_1008_4772(*(astruct_76 **)((int)param_2 + 0xe));
    uStack4                         = (undefined2)(uVar3 >> 0x10);
    iStack6                         = (int)uVar3;
    uStack10                        = *(undefined4 *)(iStack6 + 0x4);
    uStack14                        = *(undefined4 *)(iStack6 + 0x8);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, &local_14), 0x0, (int)uStack14 - 0x1, (int)uStack10 - 0x1);
    puVar1 = (ulong *)((int)param_1 + 0x14);
    pass1_1008_6cb4((ulong *)CONCAT22(param_3, local_20), &local_14, param_3, puVar1, uVar2);
    pass1_1008_6d64((ushort *)CONCAT22(param_3, local_20), (ushort *)(param_1 & 0xffff0000 | ZEXT24(puVar1)));
    return;
}


int __stdcall16far pass1_1030_5b00(ulong param_1)

{
    return *(int *)((int)param_1 + 0x4) + 0xb;
}


void __stdcall16far pass1_1030_5b1c(ulong param_1, ushort *param_2, ushort *param_3)

{
    ushort uVar1;

    uVar1    = (ushort)(param_1 >> 0x10);
    *param_3 = *(ushort *)((int)param_1 + 0x1a);
    *param_2 = *(ushort *)((int)param_1 + 0x1c);
    return;
}


void __stdcall16far pass1_1030_5b3e(ulong param_1, int param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                     = (undefined2)(param_1 >> 0x10);
    iVar1                     = (int)param_1;
    *(ushort *)(iVar1 + 0x1a) = param_3;
    if(*(int *)(iVar1 + 0x1c) < param_2)
    {
        *(int *)(iVar1 + 0x1c) = param_2;
    }
    return;
}


void __stdcall16far pass1_1030_3006(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x10) = param_2;
    return;
}


void __stdcall16far pass1_1030_3258(ulong param_1, ushort param_2)

{
    *(ushort *)((int)param_1 + 0x1ae) = param_2;
    return;
}


void __stdcall16far pass1_1030_326a(ulong param_1, ulong param_2, uint param_3, ushort param_4)

{
    uint         uVar1;
    ulong        uVar2;
    uint         uVar3;
    astruct_692 *iVar4;
    uint         uVar4;
    long         lStack6;

    uVar4 = (uint)(param_1 >> 0x10);
    iVar4 = (astruct_692 *)param_1;
    if(iVar4->field_0x1aa == 0x0)
    {
        iVar4->field_0x1aa = 0x1;
    }
    else
    {
        param_2            = iVar4->field_0x1aa * 0x2;
        iVar4->field_0x1aa = param_2;
    }
    uVar1 = (uint)param_2;
    pass1_1030_38b8();
    lStack6 = CONCAT22(param_3, uVar1);
    uVar2   = iVar4->field_0x1aa;
    uVar3   = *(uint *)((int)&iVar4->field_0x1aa + 0x2);
    if(lStack6 < (long)uVar2)
    {
        uVar2 = (ulong)uVar1;
        uVar3 = param_3;
    }
    *(int *)&iVar4->field_0x1aa               = (int)uVar2;
    *(uint *)((int)&iVar4->field_0x1aa + 0x2) = uVar3;
    pass1_1030_375a(param_1 & 0xffff | (ulong)uVar4 << 0x10, 0x0, uVar2 & 0xffff | (ulong)uVar3 << 0x10, param_4);
    return;
}


void __stdcall16far pass1_1030_3534(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x4) = param_2;
    return;
}


void __stdcall16far pass1_1030_3548(ulong param_1, long param_2)

{
    long *plVar1;

    plVar1  = (long *)((int)param_1 + 0x4);
    *plVar1 = *plVar1 + param_2;
    return;
}


void __stdcall16far pass1_1030_355c(ulong param_1, ulong param_2)

{
    int        iVar1;
    undefined2 uVar2;
    int        iStack4;

    iStack4 = 0x0;
    do
    {
        iVar1                                 = iStack4 * 0x4;
        uVar2                                 = (undefined2)(param_1 >> 0x10);
        *(long *)((int)param_1 + iVar1 + 0x4) = *(long *)(iVar1 + (int)param_2) + *(long *)((int)param_1 + 0x4 + iVar1);
        iStack4                               = iStack4 + 0x1;
    } while(iStack4 < 0x5b);
    return;
}


void __stdcall16far pass1_1030_35a4(ulong param_1, long param_2, uchar *param_3, ushort param_4, ushort param_5)

{
    uint      *puVar1;
    uchar    **ppuVar2;
    uint       uVar3;
    uchar     *puVar4;
    ushort     uVar5;
    uint       uVar6;
    ulong      uVar7;
    uchar     *puVar8;
    undefined2 uVar9;
    undefined  uVar10;
    undefined  uVar11;
    undefined  local_c[0x2];
    undefined4 local_a;
    undefined4 uStack6;

    vsprintf_op_1030_840a((ulong)s_Pop_Leaving__ld_1050_516a, param_4, param_5, (ushort)param_3);
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_3, 0x1000);
        PTR_LOOP_1050_5f2e = param_3;
    }
    else
    {
    }
    uVar5   = fn_ptr_op_1000_1708(0x16c, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    uStack6 = CONCAT22(PTR_LOOP_1050_5f2e, uVar5);
    uVar10  = (undefined)param_5;
    uVar11  = (undefined)(param_5 >> 0x8);
    pass1_1030_3948(
      param_1, (ushort *)CONCAT22(param_5, local_c), (int *)CONCAT13(uVar11, CONCAT12(uVar10, &local_a)), 0x3);
    uVar7 = (ulong)((int)&local_a + 0x2U);
    pass1_1030_3948(param_1,
                    (ushort *)CONCAT22(param_5, (int)&local_a + 0x2U),
                    (int *)CONCAT13(uVar11, CONCAT12(uVar10, local_c)),
                    0x4);
    do
    {
        uVar6 = (uint)uVar7;
        if(param_2 < 0x1)
            break;
        pass1_1008_612e((int)local_a, (int)((ulong)local_a >> 0x10), uVar6);
        uVar7 = ZEXT24(&param_2);
        pass1_1030_3a3a(param_1, (long *)CONCAT13(uVar11, CONCAT12(uVar10, &param_2)), uVar6);
        uVar9    = (undefined2)((ulong)uStack6 >> 0x10);
        puVar1   = (uint *)(uVar6 * 0x4 + (int)uStack6);
        uVar3    = *puVar1;
        *puVar1  = *puVar1 + (uint)uVar7;
        ppuVar2  = (uchar **)(uVar6 * 0x4 + (int)uStack6 + 0x2);
        *ppuVar2 = PTR_LOOP_1050_5f2e + (int)(*ppuVar2 + CARRY2(uVar3, (uint)uVar7));
        pass1_1030_38f2(param_1, 0x3, param_5);
        uVar6  = (uint)uVar7;
        puVar8 = PTR_LOOP_1050_5f2e;
        pass1_1030_38f2(param_1, 0x4, param_5);
        puVar4             = PTR_LOOP_1050_5f2e + (int)puVar8;
        PTR_LOOP_1050_5f2e = puVar8;
    } while(((uint)(puVar4 + CARRY2(uVar6, (uint)uVar7)) | uVar6 + (uint)uVar7) != 0x0);
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x18c)), (WNDCLASS16 *)0x0, 0x18);
    return;
}


void __stdcall16far
pass1_1030_3694(ulong param_1, int param_2, long param_3, uchar *param_4, ushort param_5, ushort param_6)

{
    uint   *puVar1;
    uchar **ppuVar2;
    uint    uVar3;
    ushort  uVar4;
    uint    uVar5;
    ulong   uVar6;
    uchar  *puVar7;

    vsprintf_op_1030_840a((ulong)s_Pop_Leaving__ld_1050_517a, param_5, param_6, (ushort)param_4);
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4, 0x1000);
        PTR_LOOP_1050_5f2e = param_4;
    }
    else
    {
    }
    uVar4  = fn_ptr_op_1000_1708(0x16c, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    uVar6  = (ulong)(param_2 - 0x1U);
    puVar7 = PTR_LOOP_1050_5f2e;
    if(((param_2 < 0x1) || (SBORROW2(param_2, 0x1)))
       || (uVar6 = (ulong)(param_2 - 0x5U), param_2 - 0x5U != 0x0 && 0x3 < (int)(param_2 - 0x1U)))
    {
        while(uVar5 = (uint)uVar6, 0x0 < param_3)
        {
            pass1_1008_612e(0x0, 0x5a, uVar5);
            uVar6 = ZEXT24(&param_3);
            pass1_1030_3a3a(
              param_1, (long *)CONCAT13((char)(param_6 >> 0x8), CONCAT12((char)param_6, &param_3)), uVar5);
            puVar1   = (uint *)(uVar5 * 0x4 + uVar4);
            uVar3    = *puVar1;
            *puVar1  = *puVar1 + (uint)uVar6;
            ppuVar2  = (uchar **)(uVar5 * 0x4 + uVar4 + 0x2);
            *ppuVar2 = puVar7 + (int)(*ppuVar2 + CARRY2(uVar3, (uint)uVar6));
        }
    }
    else
    {
        pass1_1030_39dc(param_1,
                        (long *)CONCAT22(param_6, &param_3),
                        CONCAT13((char)((uint)PTR_LOOP_1050_5f2e >> 0x8), CONCAT12((char)PTR_LOOP_1050_5f2e, uVar4)),
                        param_2);
    }
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x18c)), (WNDCLASS16 *)0x0, 0x18);
    return;
}


void __stdcall16far pass1_1030_375a(ulong param_1, int param_2, long param_3, ushort param_4)

{
    int        iVar1;
    int        iVar2;
    uint       uVar3;
    long       lVar4;
    long       lVar5;
    int        iVar6;
    int        iVar7;
    undefined2 uVar8;
    int        iStack20;
    undefined4 uStack18;
    int        local_6;
    int        local_4;

    iVar6 = (int)param_1;
    if(param_2 == 0x0)
    {
        local_4 = 0x5a;
        while(
          (-0x1 < local_4 && (pass1_1030_3a3a(param_1, (long *)CONCAT22(param_4, &param_3), local_4), param_3 != 0x0)))
        {
            local_4 = local_4 + -0x1;
        }
    }
    else
    {
        pass1_1030_3948(param_1, (ushort *)CONCAT22(param_4, &local_4), (int *)CONCAT22(param_4, &local_6), param_2);
        iVar2 = (local_4 - local_6) + 0x1;
        lVar4 = param_3 / (long)iVar2;
        lVar5 = lVar4 * iVar2;
        uVar3 = (uint)lVar5;
        uStack18
          = CONCAT22(((int)((ulong)param_3 >> 0x10) - (int)((ulong)lVar5 >> 0x10)) - (uint)((uint)param_3 < uVar3),
                     (uint)param_3 - uVar3);
        for(iStack20 = local_6; iStack20 <= local_4; iStack20 = iStack20 + 0x1)
        {
            iVar7                          = iStack20 * 0x4;
            uVar8                          = (undefined2)(param_1 >> 0x10);
            *(long *)(iVar6 + iVar7 + 0x4) = *(long *)(iVar6 + iVar7 + 0x4) - lVar4;
            iVar2                          = *(int *)(iVar6 + iVar7 + 0x6);
            if((uStack18._2_2_ | (uint)uStack18) != 0x0)
            {
                iVar1                         = *(int *)(iVar6 + iVar7 + 0x4);
                *(int *)(iVar6 + iVar7 + 0x4) = iVar1 + -0x1;
                *(int *)(iVar6 + iVar7 + 0x6) = iVar2 - (uint)(iVar1 == 0x0);
                uStack18                      = uStack18 + -0x1;
            }
            if(*(int *)(iVar6 + iStack20 * 0x4 + 0x6) < 0x0)
            {
                *(undefined4 *)(iVar6 + iStack20 * 0x4 + 0x4) = 0x0;
            }
        }
    }
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x18c)), (WNDCLASS16 *)0x0, 0x18);
    return;
}


void __stdcall16far pass1_1030_387c(ulong param_1)

{
    int iStack4;

    iStack4 = 0x5a;
    do
    {
        *(undefined4 *)(iStack4 * 0x4 + (int)param_1 + 0x4) = *(undefined4 *)(iStack4 * 0x4 + (int)param_1);
        iStack4                                             = iStack4 + -0x1;
    } while(0x0 < iStack4);
    *(undefined4 *)((int)param_1 + 0x4) = 0x0;
    return;
}


void __stdcall16far pass1_1030_38b8(void)

{
    int iStack8;

    iStack8 = 0x0;
    do
    {
        iStack8 = iStack8 + 0x1;
    } while(iStack8 < 0x5b);
    return;
}


void __stdcall16far pass1_1030_38f2(ulong param_1, int param_2, ushort param_3)

{
    int        iStack12;
    int        local_a;
    int        local_8;
    undefined4 uStack6;

    uStack6 = 0x0;
    pass1_1030_3948(param_1, (ushort *)CONCAT22(param_3, &local_a), (int *)CONCAT22(param_3, &local_8), param_2);
    for(iStack12 = local_8; iStack12 <= local_a; iStack12 = iStack12 + 0x1)
    {
    }
    return;
}


void __stdcall16far pass1_1030_3948(ulong param_1, ushort *param_2, int *param_3, int param_4)

{
    undefined2 uVar1;

    if(param_4 == 0x1)
    {
        *param_3 = 0x0;
        *param_2 = 0x3;
        return;
    }
    uVar1 = (undefined2)(param_1 >> 0x10);
    if(param_4 == 0x2)
    {
        *param_3 = 0x4;
        *param_2 = *(ushort *)((int)param_1 + 0x1ae);
        return;
    }
    if(param_4 == 0x3)
    {
        *param_3 = *(int *)((int)param_1 + 0x1ae) + 0x1;
        *param_2 = 0x27;
        return;
    }
    if(param_4 != 0x4)
    {
        if(param_4 == 0x5)
        {
            *param_3 = 0x4c;
        }
        else
        {
            *param_3 = 0x0;
        }
        *param_2 = 0x5a;
        return;
    }
    *param_3 = 0x28;
    *param_2 = 0x4b;
    return;
}


void __stdcall16far pass1_1030_39dc(ulong param_1, long *param_2, ulong param_3, int param_4)

{
    int        iVar1;
    undefined2 in_DX;
    undefined2 uVar2;
    undefined2 unaff_SS;
    int        iStack8;
    int        local_6;
    int        local_4;

    pass1_1030_3948(param_1, (ushort *)CONCAT22(unaff_SS, &local_6), (int *)CONCAT22(unaff_SS, &local_4), param_4);
    iStack8 = local_6;
    while(true)
    {
        if(iStack8 < local_4)
        {
            return;
        }
        iVar1 = local_4;
        pass1_1030_3a3a(param_1, param_2, iStack8);
        uVar2                                               = (undefined2)(param_3 >> 0x10);
        *(int *)(iStack8 * 0x4 + (int)param_3)              = iVar1;
        *(undefined2 *)(iStack8 * 0x4 + (int)param_3 + 0x2) = in_DX;
        if(*param_2 == 0x0)
            break;
        iStack8 = iStack8 + -0x1;
    }
    return;
}


void __stdcall16far pass1_1030_3a3a(ulong param_1, long *param_2, int param_3)

{
    int       *piVar1;
    int        iVar2;
    int        iVar3;
    uint       uVar4;
    uint       uVar5;
    int        iVar6;
    int        iVar7;
    int        iVar8;
    undefined2 uVar9;

    iVar2  = *(int *)((int)param_2 + 0x2);
    uVar9  = (undefined2)(param_1 >> 0x10);
    iVar6  = (int)param_1;
    iVar7  = iVar6 + 0x4;
    iVar8  = param_3 * 0x4;
    piVar1 = (int *)(iVar7 + iVar8 + 0x2);
    iVar3  = *piVar1;
    if((iVar3 < iVar2)
       || ((uVar5 = (uint)*param_2, *piVar1 == iVar2 || iVar3 < iVar2 && (*(uint *)(iVar7 + iVar8) < uVar5))))
    {
        *param_2                                     = *param_2 - *(long *)(iVar6 + 0x4 + param_3 * 0x4);
        *(undefined4 *)(iVar6 + param_3 * 0x4 + 0x4) = 0x0;
    }
    else
    {
        uVar4                         = *(uint *)(iVar7 + iVar8);
        iVar3                         = *(int *)(iVar7 + iVar8 + 0x2);
        *(int *)(iVar6 + iVar8 + 0x4) = uVar4 - uVar5;
        *(int *)(iVar6 + iVar8 + 0x6) = (iVar3 - iVar2) - (uint)(uVar4 < uVar5);
        *param_2                      = 0x0;
    }
    return;
}
