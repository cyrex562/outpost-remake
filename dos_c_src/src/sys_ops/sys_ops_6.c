
void __stdcall16far pass1_1020_04f6(ushort *param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5)

{
    code       **ppcVar1;
    int          iVar2;
    undefined2   uVar3;
    uchar       *extraout_DX;
    astruct_662 *iVar4;
    undefined2   uVar4;
    ushort      *puVar5;

    uVar4                                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                                         = (astruct_662 *)param_1;
    *param_1                                      = 0x389a;
    iVar4->field_0x2                              = 0x1008;
    *param_1                                      = 0x3aa8;
    iVar4->field_0x2                              = 0x1008;
    iVar4->field_0x4                              = param_2;
    *param_1                                      = 0x3ab0;
    iVar4->field_0x2                              = 0x1008;
    iVar4->field_0x6                              = (undefined4 *)0x0;
    iVar4->field_0xa                              = 0x0;
    iVar4->field_0xc                              = 0x0;
    iVar4->field_0xe                              = 0x0;
    iVar4->field_0x10                             = 0x0;
    *param_1                                      = 0x75a;
    iVar4->field_0x2                              = 0x1020;
    puVar5                                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x1, param_5, param_3, param_4);
    uVar3                                         = (undefined2)((ulong)puVar5 >> 0x10);
    *(int *)&iVar4->field_0x6                     = (int)puVar5;
    *(undefined2 *)((int)&iVar4->field_0x6 + 0x2) = uVar3;
    ppcVar1                                       = (code **)((int)*iVar4->field_0x6 + 0x4);
    (**ppcVar1)(0x1010, *(undefined2 *)&iVar4->field_0x6, uVar3, 0x0, param_1);
    puVar5           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_5, extraout_DX, param_4);
    iVar2            = (int)puVar5;
    iVar4->field_0xa = *(undefined2 *)(iVar2 + 0xa);
    iVar4->field_0xc = *(undefined2 *)(iVar2 + 0xc);
    pass1_1008_3e94((ushort *)((ulong)puVar5 & 0xffff0000 | (ulong)(iVar2 + 0xe)), (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x10)), (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0xe)));
    return;
}

ulong __stdcall16far pass1_1018_dd1e(ushort param_1, uint param_2, uchar *param_3, ushort param_4, ushort param_5, int param_6, ulong param_7)

{
    uint  uVar1;
    ulong uStack6;

    pass1_1010_81f6(0x1010, param_1, _PTR_LOOP_1050_14cc, 0x0, param_7._2_2_);
    uStack6 = CONCAT22(param_3, param_2);
    mem_op_1000_179c(0x46, param_3, 0x1000);
    uVar1 = (uint)param_3 | param_2;
    if(uVar1 == 0x0)
    {
        param_2 = 0x0;
        uVar1   = 0x0;
    }
    else
    {
        pass1_1008_87cc((astruct_86 *)CONCAT22(param_3, param_2), param_6, (int)param_7, param_7._2_2_, uStack6, 0x0, param_1);
    }
    pass1_1008_8bc6(param_1, uVar1, CONCAT22(uVar1, param_2));
    return CONCAT22(uVar1, param_2);
}

ushort *__stdcall16far struct_1018_e100(ushort *param_1, ushort param_2, uchar *param_3, ushort param_4)

{
    astruct_268 *iVar1;
    int          unaff_DI;
    undefined2   uVar1;
    ushort      *puVar2;

    uVar1                            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                            = (astruct_268 *)param_1;
    *param_1                         = 0x389a;
    iVar1->field_0x2                 = 0x1008;
    *param_1                         = 0x3aa8;
    iVar1->field_0x2                 = 0x1008;
    iVar1->field_0x4                 = param_2;
    *param_1                         = 0x3ab0;
    iVar1->field_0x2                 = 0x1008;
    *(undefined4 *)&iVar1->field_0x6 = 0x0;
    *param_1                         = 0xe228;
    iVar1->field_0x2                 = 0x1018;
    puVar2                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x36, param_4, param_3, unaff_DI);
    iVar1->field_0x6                 = (int)puVar2;
    iVar1->field_0x8                 = (int)((ulong)puVar2 >> 0x10);
    return param_1;
}

void __stdcall16far pass1_1018_c402(astruct_20 *param_1, uint param_2, uint param_3, ushort param_4, ulong param_5, ulong param_6, ulong param_7, ulong param_8, ushort param_9, ushort param_10)

{
    int         iVar1;
    uint       *puVar2;
    astruct_20 *iVar4;
    int         unaff_DI;
    astruct_20 *uVar4;
    ushort     *puVar3;

    struct_1020_0762(param_1, CONCAT22((int)param_5, param_4), (ulong *)CONCAT22((int)param_6, (int)(param_5 >> 0x10)), (ushort)(param_6 >> 0x10), param_7, param_8, param_9);
    uVar4                 = (astruct_20 *)((ulong)param_1 >> 0x10);
    iVar4                 = (astruct_20 *)param_1;
    iVar4[0x1].field_0x14 = 0x0;
    iVar4[0x1].field_0x16 = 0x0;
    iVar4[0x1].field_0x18 = 0x0;
    iVar4[0x1].field_0x1a = 0x0;
    iVar4[0x1].field_0x1c = 0x2;
    iVar4[0x1].field_0x26 = 0x0;
    iVar4[0x1].field_0x2a = 0x0;
    iVar4[0x1].field_0x2c = 0x1e0190;
    iVar4[0x1].field_0x30 = 0x0;
    param_1->field_0x0    = 0xc8bc;
    iVar4->field_0x2      = 0x1018;
    puVar2                = pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1e)), (WNDCLASS16 *)0x0, 0x8);
    if((param_3 == 0x0) || (param_2 != 0x0))
    {
        if((param_2 & param_3) == 0x0)
            goto LAB_1018_c4bb;
        puVar2 = (uint *)pass1_1008_5fd8(param_9, (uchar *)param_10);
    }
    else
    {
        load_string_1010_84ac((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    }
    *(uint **)&iVar4[0x1].field_0x26               = puVar2;
    *(uchar **)((int)&iVar4[0x1].field_0x26 + 0x2) = (uchar *)param_10;
LAB_1018_c4bb:
    puVar3                = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_9, (uchar *)param_10, unaff_DI);
    iVar1                 = (int)puVar3;
    iVar4[0x1].field_0x14 = *(ushort *)(iVar1 + 0xa);
    iVar4[0x1].field_0x16 = *(ushort *)(iVar1 + 0xc);
    pass1_1008_3e94((ushort *)((ulong)puVar3 & 0xffff0000 | (ulong)(iVar1 + 0xe)), (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1a)), (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x18)));
    return;
}

astruct_57 *__stdcall16far pass1_1018_5e26(astruct_57 *param_1, ushort param_2)

{
    undefined2 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfd0, param_2);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined2 *)param_1               = 0x6128;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1018;
    *(undefined2 *)((int)param_1 + 0x74) = 0x1;
    return param_1;
}

void __stdcall16far pass1_1018_6198(ushort *param_1, ulong param_2, ushort param_3, uchar *param_4, int param_5, ushort param_6)

{
    astruct_657 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    uVar1                            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                            = (astruct_657 *)param_1;
    *param_1                         = 0x389a;
    iVar1->field_0x2                 = 0x1008;
    *param_1                         = 0x3aa8;
    iVar1->field_0x2                 = 0x1008;
    iVar1->field_0x4                 = param_3;
    *param_1                         = 0x3ab0;
    iVar1->field_0x2                 = 0x1008;
    *(undefined4 *)&iVar1->field_0x6 = 0x0;
    iVar1->field_0xa                 = param_2;
    *param_1                         = 0x66c0;
    iVar1->field_0x2                 = 0x1018;
    puVar2                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, param_6, param_4, param_5);
    iVar1->field_0x6                 = (int)puVar2;
    iVar1->field_0x8                 = (int)((ulong)puVar2 >> 0x10);
    return;
}

ulong __stdcall16far pass1_1018_659a(ushort param_1, ushort param_2, ushort *param_3, uchar *param_4, ushort param_5)

{
    int *piVar1;
    int  iStack18;
    int  local_6;
    int  local_4;

    piVar1 = &local_6;
    pass1_1008_3e94(param_3, (ushort *)CONCAT22(param_5, piVar1), (ushort *)CONCAT22(param_5, &local_4));
    mem_op_1000_179c(0xc, param_4, 0x1000);
    for(iStack18 = 0x0; iStack18 < 0x3; iStack18 = iStack18 + 0x1)
    {
        piVar1[iStack18 * 0x2]       = *(int *)(iStack18 * 0x4 + 0x4248) + local_4;
        piVar1[iStack18 * 0x2 + 0x1] = *(int *)(iStack18 * 0x4 + 0x424a) + local_6;
    }
    return CONCAT22(param_4, piVar1);
}

ushort *__stdcall16far get_sys_metrics_1018_4b1e(astruct_55 *param_1, ushort param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;

    struct_op_1010_1d48((astruct_79 *)param_1, param_3);
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(ushort *)(iVar1 + 0x12)     = param_2;
    *(undefined2 *)(iVar1 + 0x14) = 0x0;
    param_1->field_0x0            = (ushort)&PTR_LOOP_1050_4c9e;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1018;
    if(PTR_LOOP_1050_416c == (undefined *)0x0)
    {
        PTR_LOOP_1050_416c = (undefined *)GetSystemMetrics16(0x1010);
        PTR_LOOP_1050_416e = (undefined *)GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
        PTR_LOOP_1050_4170 = (undefined *)GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    }
    return &param_1->field_0x0;
}

void __stdcall16far pass1_1018_4b78(ulong *param_1, ushort param_2)

{
    code      **ppcVar1;
    uchar      *puVar2;
    uint        uVar3;
    ushort     *puVar4;
    undefined4 *puVar5;

    puVar2 = param_1._2_2_;
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | ZEXT24((undefined4 *)((int)param_1 + 0xa))), (WNDCLASS16 *)0x0, 0x8);
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x18)), (WNDCLASS16 *)0x0, 0x8);
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, puVar2, (int)param_1._2_2_);
    puVar5 = (undefined4 *)pass1_1010_5f7a((int)puVar4, (ushort)((ulong)puVar4 >> 0x10), 0x0, *(int *)((int)param_1 + 0x12));
    uVar3  = (uint)((ulong)puVar5 >> 0x10);
    if((uVar3 | (uint)puVar5) != 0x0)
    {
        *(undefined4 *)((int)param_1 + 0xa) = *puVar5;
        *(undefined4 *)((int)param_1 + 0xe) = *(undefined4 *)((uint)puVar5 + 0x4);
    }
    ppcVar1 = (code **)((int)*param_1 + 0x20);
    (**ppcVar1)(0x1010, param_1);
    if((*(int *)((int)param_1 + 0xe) == 0x0) && (*(int *)((int)param_1 + 0x10) == 0x0))
    {
        *(undefined2 *)((int)param_1 + 0xa) = *(undefined2 *)((int)param_1 + 0x18);
        *(undefined2 *)((int)param_1 + 0xc) = *(undefined2 *)((int)param_1 + 0x1a);
    }
    *(undefined2 *)((int)param_1 + 0xe)  = *(undefined2 *)((int)param_1 + 0x1c);
    *(undefined2 *)((int)param_1 + 0x10) = *(undefined2 *)((int)param_1 + 0x1e);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_4c2c(ulong param_1, ulong *param_2, ushort param_3, ushort param_4)

{
    ushort *puVar1;

    *(ulong *)((int)param_1 + 0xa) = *param_2;
    *(ulong *)((int)param_1 + 0xe) = param_2[0x1];
    puVar1                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, param_1._2_2_, (int)param_1._2_2_);
    pass1_1010_5fb0((ulong)puVar1, 0x0, (ulong *)((int)param_1 + 0xa), (ushort)param_1._2_2_, *(int *)((int)param_1 + 0x12));
    return;
}
void __stdcall16far pass1_1018_4dce(ulong *param_1, ushort param_2, uchar *param_3, ushort param_4)

{
    code     **ppcVar1;
    undefined2 uVar2;
    int        unaff_DI;
    ushort    *puVar3;

    puVar3  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, param_3, unaff_DI);
    uVar2   = (undefined2)((ulong)puVar3 >> 0x10);
    ppcVar1 = (code **)((int)*param_1 + 0x10);
    (**ppcVar1)(0x1010, param_1, param_2, *(undefined2 *)((int)puVar3 + 0xc), *(undefined2 *)((int)puVar3 + 0xa));
    return;
}

void __stdcall16far pass1_1018_5292(ulong param_1, ulong param_2, ushort param_3)

{
    int         iVar1;
    undefined4  uVar2;
    code      **ppcVar3;
    ushort      uVar4;
    BOOL16      BVar5;
    undefined  *puVar6;
    int         iVar7;
    char       *pcVar8;
    ushort      uVar9;
    undefined4 *puVar10;
    undefined4 *puVar11;
    ulong       uVar12;
    uchar      *extraout_DX;
    uchar      *extraout_DX_00;
    uint        uVar13;
    uchar      *extraout_DX_01;
    uchar      *puVar14;
    uint        extraout_DX_02;
    uint        extraout_DX_03;
    uchar      *puVar15;
    uchar      *extraout_DX_04;
    undefined2  uVar16;
    uint        extraout_DX_05;
    undefined2  extraout_DX_06;
    uchar      *extraout_DX_07;
    uchar      *extraout_DX_08;
    int         iVar17;
    undefined2  uVar18;
    undefined2  uVar19;
    ushort     *puVar20;
    uint        uStack50;
    undefined   local_26[0x8];
    ulong       uStack30;
    ulong       uStack26;
    ulong       uStack22;
    undefined4 *puStack18;
    uchar      *puStack16;
    undefined4 *puStack14;
    uchar      *puStack12;
    undefined2  uStack10;
    ulong       uStack8;
    ushort      uStack4;

    uVar18    = (undefined2)(param_1 >> 0x10);
    iVar17    = (int)param_1;
    puStack18 = (undefined4 *)*(uint *)(iVar17 + 0xe);
    uVar12    = ZEXT24(puStack18);
    puVar14   = *(uchar **)(iVar17 + 0x10);
    puStack16 = puVar14;
    puStack14 = puStack18;
    puStack12 = puVar14;
    if(((uint)puVar14 | (uint)puStack18) != 0x0)
    {
        ppcVar3 = (code **)*puStack18;
        (**ppcVar3)();
        puVar14 = extraout_DX;
    }
    mem_op_1000_179c(0xc, puVar14, 0x1000);
    puStack18 = (undefined4 *)(uint)uVar12;
    puStack16 = puVar14;
    if(((uint)puVar14 | (uint)puStack18) == 0x0)
    {
        uVar12  = 0x0;
        puVar14 = (uchar *)0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10));
        puVar14 = extraout_DX_00;
    }
    *(undefined2 *)(iVar17 + 0xe) = (int)uVar12;
    *(uchar **)(iVar17 + 0x10)    = puVar14;
    for(uStack4 = 0x21; - 0x1 < (int)uStack4; uStack4 = uStack4 - 0x1)
    {
        uStack22 = pass1_1030_7c28(param_2, uStack4, (uint)uVar12, (uint)puVar14, param_3);
        uVar12   = uStack22 & 0xffff;
        uVar13   = (uint)uVar12;
        puVar14  = (uchar *)((uint)(uStack22 >> 0x10) | uVar13);
        if(puVar14 != (uchar *)0x0)
        {
            string_1020_c0ca(uStack4);
            uVar4    = str_op_1008_60e8((char *)CONCAT22(puVar14, uVar13), (ushort)puVar14);
            uVar12   = (ulong)uVar4;
            uStack26 = CONCAT22(puVar14, uVar4);
            mem_op_1000_179c(0x10, puVar14, 0x1000);
            puStack14 = (undefined4 *)uVar12;
            puStack12 = puVar14;
            if(((uint)puVar14 | (uint)puStack14) == 0x0)
            {
                uVar12 = 0x0;
                uVar13 = 0x0;
            }
            else
            {
                struct_1018_4790(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10, uStack22, uStack26, uStack4);
                uVar13 = extraout_DX_02;
            }
            uStack30 = uVar12 & 0xffff | (ulong)uVar13 << 0x10;
            uVar2    = *(undefined4 *)(iVar17 + 0xe);
            ppcVar3  = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
            (**ppcVar3)(0x1000, (int)uVar2, (char)((ulong)uVar2 >> 0x10), (int)uVar12, uVar13);
            puVar14 = extraout_DX_01;
        }
    }
    uStack8  = struct_op_1030_73a8(param_2);
    uStack10 = *(undefined2 *)((int)uStack8 + 0xc);
    BVar5    = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack10, 0x4);
    if(BVar5 != 0x0)
    {
        uStack30 = uStack8;
        uStack26 = *(ulong *)((int)uStack8 + 0x20);
        pass1_1008_5784((ulong *)CONCAT22(param_3, local_26), uStack26);
        while(true)
        {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            uStack22 = CONCAT22(extraout_DX_03, puVar6);
            puVar14  = (uchar *)(extraout_DX_03 | (uint)puVar6);
            if(puVar14 == (uchar *)0x0)
                break;
            iVar1 = *(int *)(puVar6 + 0x6);
            iVar7 = iVar1 + -0x7;
            if(iVar7 == 0x0)
            {
            LAB_1018_53f0:
                pcVar8  = string_op_1020_c222(*(ushort *)(puVar6 + 0x6));
                uVar9   = str_op_1008_60e8((char *)CONCAT22(puVar14, pcVar8), (ushort)puVar14);
                puVar15 = puVar14;
                uVar4   = uVar9;
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = (undefined4 *)uVar4;
                puStack16 = puVar15;
                if(((uint)puVar15 | uVar4) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar19  = (undefined2)(uStack22 >> 0x10);
                    puVar20 = struct_1018_48b0((ushort *)CONCAT22(puVar15, uVar4), (ulong) * (uint *)((int)uStack22 + 0xa), CONCAT22(puVar14, uVar9), *(ushort *)((int)uStack22 + 0x6));
                    uVar16  = (undefined2)((ulong)puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uVar2   = *(undefined4 *)(iVar17 + 0xe);
                ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, (int)uVar2, (char)((ulong)uVar2 >> 0x10), uVar19, uVar16);
                puVar14 = extraout_DX_04;
            }
            else
            {
                if(((0x5 < iVar7) && (!SBORROW2(iVar7, 0x6))) && (iVar1 + -0xd < 0x2))
                    goto LAB_1018_53f0;
            }
            uVar19 = (undefined2)(uStack22 >> 0x10);
            if(*(int *)((int)uStack22 + 0x8) != 0x0)
            {
                pcVar8  = string_op_1020_c2f8(*(ushort *)((int)uStack22 + 0x8));
                puVar10 = (undefined4 *)str_op_1008_60e8((char *)CONCAT22(puVar14, pcVar8), (ushort)puVar14);
                puVar15 = puVar14;
                puVar11 = puVar10;
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack14 = puVar11;
                puStack12 = puVar15;
                if(((uint)puVar15 | (uint)puVar11) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar19  = (undefined2)(uStack22 >> 0x10);
                    puVar20 = struct_1018_48e8((ushort *)CONCAT22(puVar15, puVar11), (ulong) * (uint *)((int)uStack22 + 0xa), CONCAT22(puVar14, puVar10), *(ushort *)((int)uStack22 + 0x8));
                    uVar16  = (undefined2)((ulong)puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uVar2   = *(undefined4 *)(iVar17 + 0xe);
                ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, (int)uVar2, (char)((ulong)uVar2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    uVar19   = (undefined2)(param_2 >> 0x10);
    uVar12   = *(ulong *)((int)param_2 + 0x3e);
    uVar13   = *(uint *)((int)param_2 + 0x40);
    uStack50 = (uint)uVar12;
    if((uVar13 | uStack50) != 0x0)
    {
        pass1_1008_5784((ulong *)CONCAT22(param_3, local_26), uVar12 & 0xffff | (ulong)uVar13 << 0x10);
        while(true)
        {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            puVar14 = (uchar *)(extraout_DX_05 | (uint)puVar6);
            if(puVar14 == (uchar *)0x0)
                break;
            if(*(int *)(puVar6 + 0x4) != 0x0)
            {
                pcVar8   = string_1020_c0d8(*(ushort *)(puVar6 + 0x4));
                uVar13   = str_op_1008_60e8((char *)CONCAT22(puVar14, pcVar8), (ushort)puVar14);
                uStack30 = CONCAT22(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = (undefined4 *)uVar13;
                puStack16 = puVar14;
                if(((uint)puVar14 | uVar13) == 0x0)
                {
                    uVar13 = 0x0;
                    uVar19 = 0x0;
                }
                else
                {
                    struct_1018_4790(CONCAT22(puVar14, uVar13), (ulong) * (uint *)(puVar6 + 0xa), uStack30, *(ushort *)(puVar6 + 0x4));
                    uVar19 = extraout_DX_06;
                }
                uStack26 = CONCAT22(uVar19, uVar13);
                uVar2    = *(undefined4 *)(iVar17 + 0xe);
                ppcVar3  = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, (int)uVar2, (char)((ulong)uVar2 >> 0x10), uVar13, uVar19);
                puVar14 = extraout_DX_07;
            }
            if(*(int *)(puVar6 + 0x6) != 0x0)
            {
                pcVar8   = string_op_1020_c222(*(ushort *)(puVar6 + 0x6));
                puVar11  = (undefined4 *)str_op_1008_60e8((char *)CONCAT22(puVar14, pcVar8), (ushort)puVar14);
                uStack30 = CONCAT22(puVar14, puVar11);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack14 = puVar11;
                puStack12 = puVar14;
                if(((uint)puVar14 | (uint)puVar11) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    puVar20 = struct_1018_48b0((ushort *)CONCAT22(puVar14, puVar11), (ulong) * (uint *)(puVar6 + 0xa), uStack30, *(ushort *)(puVar6 + 0x6));
                    uVar16  = (undefined2)((ulong)puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uStack26 = CONCAT22(uVar16, uVar19);
                uVar2    = *(undefined4 *)(iVar17 + 0xe);
                ppcVar3  = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, (int)uVar2, (char)((ulong)uVar2 >> 0x10), uVar19, uVar16);
                puVar14 = extraout_DX_08;
            }
            if(*(int *)(puVar6 + 0x8) != 0x0)
            {
                pcVar8   = string_op_1020_c2f8(*(ushort *)(puVar6 + 0x8));
                uVar13   = str_op_1008_60e8((char *)CONCAT22(puVar14, pcVar8), (ushort)puVar14);
                uStack30 = CONCAT22(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = (undefined4 *)uVar13;
                puStack16 = puVar14;
                if(((uint)puVar14 | uVar13) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    puVar20 = struct_1018_48e8((ushort *)CONCAT22(puVar14, uVar13), (ulong) * (uint *)(puVar6 + 0xa), uStack30, *(ushort *)(puVar6 + 0x8));
                    uVar16  = (undefined2)((ulong)puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uStack26 = CONCAT22(uVar16, uVar19);
                uVar2    = *(undefined4 *)(iVar17 + 0xe);
                ppcVar3  = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, (int)uVar2, (char)((ulong)uVar2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    return;
}

void __stdcall16far switch_1018_3ee6(ulong param_1, long param_2, int param_3, ushort param_4, uchar *param_5)

{
    int        iVar1;
    char      *pcVar2;
    ushort     uVar3;
    ushort     uVar4;
    uint       uVar5;
    uint       uVar6;
    ulong      uVar7;
    uchar     *puVar8;
    ushort     unaff_SS;
    ushort    *puVar9;
    long       lVar10;
    int        iVar11;
    INT16      IVar12;
    ushort     uVar13;
    ulong      uStack26;
    ushort    *puStack22;
    long       lStack18;
    long       lStack14;
    int        iStack10;
    undefined2 uStack8;
    int       *piStack6;

    switch(param_4)
    {
    case 0x1:
        iVar1 = param_3 * 0x4 + 0x40b6;
        break;
    default:
        iVar1 = param_3 * 0x4 + 0x40ce;
        break;
    case 0x3:
        iVar1 = param_3 * 0x4 + 0x40e2;
        break;
    case 0x4:
        iVar1 = param_3 * 0x4 + 0x40ee;
        break;
    case 0x8:
        iVar1 = param_3 * 0x4 + 0x40f2;
        break;
    case 0x9:
        iVar1 = param_3 * 0x4 + 0x4106;
        break;
    case 0xa:
        iVar1 = param_3 * 0x4 + 0x410a;
        break;
    case 0x14:
        iVar1 = param_3 * 0x4 + 0x410e;
        break;
    case 0x16:
        iVar1 = param_3 * 0x4 + 0x4112;
        break;
    case 0x17:
        iVar1 = param_3 * 0x4 + 0x4116;
        break;
    case 0x19:
        iVar1 = param_3 * 0x4 + 0x411a;
    }
    piStack6 = (int *)CONCAT22(0x1050, iVar1);
    if(piStack6 == (int *)0x0)
    {
        return;
    }
    iStack10 = 0x0;
    uStack8  = 0x0;
    iVar11   = *piStack6;
    uVar13   = (ushort)param_1;
    uVar3    = (ushort)(param_1 >> 0x10);
    if(iVar11 == 0x1)
    {
        uVar13   = pass1_1018_456a(uVar13, uVar3, *(ushort *)(iVar1 + 0x2));
        lStack14 = CONCAT22(param_5, uVar13);
        pcVar2   = string_1020_c0d8(*(ushort *)(iVar1 + 0x2));
        uVar3    = str_op_1008_60e8((char *)CONCAT22(param_5, pcVar2), (ushort)param_5);
        puVar8   = param_5;
        uVar13   = uVar3;
        mem_op_1000_179c(0x10, param_5, 0x1000);
        puStack22 = (ushort *)CONCAT22(puVar8, uVar13);
        if(((uint)puVar8 | uVar13) != 0x0)
        {
            lVar10  = param_2 / lStack14;
            uStack8 = (undefined2)(param_2 % lStack14);
            struct_1018_4790(puStack22, lVar10, CONCAT22(param_5, uVar3), *(ushort *)(iVar1 + 0x2));
            iStack10 = (int)lVar10;
            goto LAB_1018_425e;
        }
    }
    else
    {
        if(iVar11 == 0x2)
        {
            uVar13   = pass1_1018_451e(uVar13, uVar3, *(int *)(iVar1 + 0x2));
            lStack18 = CONCAT22(param_5, uVar13);
            pcVar2   = string_op_1020_c222(*(ushort *)(iVar1 + 0x2));
            uVar3    = str_op_1008_60e8((char *)CONCAT22(param_5, pcVar2), (ushort)param_5);
            puVar8   = param_5;
            uVar13   = uVar3;
            mem_op_1000_179c(0x10, param_5, 0x1000);
            puStack22 = (ushort *)CONCAT22(puVar8, uVar13);
            if(((uint)puVar8 | uVar13) != 0x0)
            {
                puVar9   = struct_1018_48b0(puStack22, param_2 / lStack18, CONCAT22(param_5, uVar3), *(ushort *)(iVar1 + 0x2));
                uStack8  = (undefined2)((ulong)puVar9 >> 0x10);
                iStack10 = (int)puVar9;
                goto LAB_1018_425e;
            }
        }
        else
        {
            if(iVar11 == 0x3)
            {
                uVar4 = pass1_1008_c646((ushort)_PTR_LOOP_1050_06e0, CONCAT22(*(undefined2 *)(iVar1 + 0x2), (int)((ulong)_PTR_LOOP_1050_06e0 >> 0x10)), unaff_SS);
                if(uVar4 == 0x0)
                {
                    uVar4 = 0x4f;
                }
                uVar13   = switch_1018_43ec(uVar13, uVar3, uVar4);
                lStack14 = CONCAT22(param_5, uVar13);
                uVar13   = pass1_1020_bd80(uVar4);
                uVar5    = str_op_1008_60e8((char *)CONCAT22(param_5, uVar13), (ushort)param_5);
                uStack26 = CONCAT22(param_5, uVar5);
                mem_op_1000_179c(0x14, param_5, 0x1000);
                puStack22 = (ushort *)CONCAT22(param_5, uVar5);
                if(((uint)param_5 | uVar5) != 0x0)
                {
                    uVar7   = param_2 / lStack14;
                    uStack8 = (undefined2)(param_2 % lStack14);
                    struct_1018_47c8(puStack22, uVar7, uStack26, uVar4, 0x0);
                    iStack10 = (int)uVar7;
                    goto LAB_1018_425e;
                }
            }
            else
            {
                if(iVar11 != 0x4)
                    goto LAB_1018_425e;
                iVar1  = *(int *)(iVar1 + 0x2);
                uVar5  = iVar1 - 0x1;
                iVar11 = (int)_PTR_LOOP_1050_14cc;
                IVar12 = (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
                if(uVar5 == 0x0)
                {
                    load_string_1010_84ac(iVar11, IVar12, 0x1010);
                    uVar6  = uVar5;
                    puVar8 = param_5;
                    mem_op_1000_179c(0x14, param_5, 0x1000);
                    puStack22 = (ushort *)CONCAT22(puVar8, uVar6);
                    if(((uint)puVar8 | uVar6) != 0x0)
                    {
                        uVar13 = 0x2;
                        lVar10 = 0x14;
                    LAB_1018_4230:
                        puVar9   = struct_1018_4842(puStack22, param_2 / lVar10, CONCAT22(param_5, uVar5), uVar13);
                        uStack8  = (undefined2)((ulong)puVar9 >> 0x10);
                        iStack10 = (int)puVar9;
                        goto LAB_1018_425e;
                    }
                }
                else
                {
                    uVar5 = iVar1 - 0x2;
                    if(uVar5 == 0x0)
                    {
                        load_string_1010_84ac(iVar11, IVar12, 0x1010);
                        uVar6  = uVar5;
                        puVar8 = param_5;
                        mem_op_1000_179c(0x14, param_5, 0x1000);
                        puStack22 = (ushort *)CONCAT22(puVar8, uVar6);
                        if(((uint)puVar8 | uVar6) != 0x0)
                        {
                            uVar13 = 0x3;
                            lVar10 = 0x16;
                            goto LAB_1018_4230;
                        }
                    }
                    else
                    {
                        uVar5 = iVar1 - 0x3;
                        if(uVar5 == 0x0)
                        {
                            load_string_1010_84ac(iVar11, IVar12, 0x1010);
                            uVar6  = uVar5;
                            puVar8 = param_5;
                            mem_op_1000_179c(0x14, param_5, 0x1000);
                            puStack22 = (ushort *)CONCAT22(puVar8, uVar6);
                            if(((uint)puVar8 | uVar6) != 0x0)
                            {
                                uVar13 = 0x4;
                                lVar10 = 0x17;
                                goto LAB_1018_4230;
                            }
                        }
                        else
                        {
                            uVar5 = iVar1 - 0x4;
                            if(uVar5 != 0x0)
                                goto LAB_1018_425e;
                            load_string_1010_84ac(iVar11, IVar12, 0x1010);
                            uVar6  = uVar5;
                            puVar8 = param_5;
                            mem_op_1000_179c(0x14, param_5, 0x1000);
                            puStack22 = (ushort *)CONCAT22(puVar8, uVar6);
                            if(((uint)puVar8 | uVar6) != 0x0)
                            {
                                uVar13 = 0x4;
                                lVar10 = 0xa;
                                goto LAB_1018_4230;
                            }
                        }
                    }
                }
            }
        }
    }
    iStack10 = 0x0;
    uStack8  = 0x0;
LAB_1018_425e:
    if(*(long *)(iStack10 + 0x8) == 0x0)
    {
        *(undefined4 *)(iStack10 + 0x8) = 0x1;
    }
    return;
}

void __stdcall16far get_sys_metrics_1018_2f56(ulong param_1)

{
    undefined2 uVar1;
    INT16      IVar2;
    INT16      IVar3;
    uchar     *in_DX;
    int        iVar4;
    int        unaff_DI;
    undefined2 uVar5;
    ushort     unaff_SS;
    ushort    *puVar6;
    ulong      uVar7;
    ushort    *puVar8;
    ushort    *puVar9;
    int        local_6;
    int        local_4;

    puVar9 = (ushort *)CONCAT22(unaff_SS, &local_4);
    puVar8 = (ushort *)CONCAT22(unaff_SS, &local_6);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)), puVar8, puVar9);
    uVar5                  = (undefined2)(param_1 >> 0x10);
    iVar4                  = (int)param_1;
    uVar7                  = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x24));
    uVar1                  = (undefined2)(uVar7 >> 0x10);
    *(int *)(iVar4 + 0x18) = local_4 + 0xb5;
    *(int *)(iVar4 + 0x1a) = local_6 + 0x9;
    IVar2                  = GetSystemMetrics16(0x1008);
    *(int *)(iVar4 + 0x1c) = IVar2 * 0x2 + *(int *)((int)uVar7 + 0x4);
    IVar2                  = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    IVar3                  = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    *(int *)(iVar4 + 0x1e) = IVar3 + IVar2 + *(int *)((int)uVar7 + 0x8);
    return;
}

void __stdcall16far pass1_1018_30fc(ulong param_1, ushort **param_2, uchar *param_3)

{
    uint        uVar1;
    undefined4  uVar2;
    ushort     *puVar3;
    uint        uVar4;
    ushort      uVar5;
    long        lVar6;
    uchar      *puVar7;
    undefined2  extraout_DX;
    undefined2  uVar8;
    undefined4 *puStack18;
    int         iStack6;

    *param_2 = (ushort *)0x0;
    uVar8    = (undefined2)(param_1 >> 0x10);
    uVar2    = *(undefined4 *)((int)param_1 + 0x17e);
    uVar1    = *(uint *)((int)uVar2 + 0xa);
    if(uVar1 != 0x0)
    {
        uVar4 = uVar1;
        mem_op_1000_179c(0x6, param_3, 0x1000);
        puStack18 = (undefined4 *)CONCAT22(param_3, uVar4);
        puVar7    = (uchar *)((uint)param_3 | uVar4);
        if(puVar7 == (uchar *)0x0)
        {
            *param_2 = (ushort *)0x0;
        }
        else
        {
            *puStack18                   = 0x0;
            *(undefined2 *)(uVar4 + 0x4) = 0x0;
            *param_2                     = (ushort *)puStack18;
        }
        uVar5 = uVar1 * 0x2;
        mem_op_1000_179c(uVar5, puVar7, 0x1000);
        puVar3                         = *param_2;
        *puVar3                        = uVar5;
        *(uchar **)((int)puVar3 + 0x2) = puVar7;
        *(uint *)((int)*param_2 + 0x4) = uVar1;
        for(iStack6 = 0x0; iStack6 < (int)uVar1; iStack6 = iStack6 + 0x1)
        {
            lVar6 = (long)iStack6;
            empty_1008_8fc4(*(undefined4 *)((int)param_1 + 0x17e), lVar6);
            *(undefined2 *)((int)*(undefined4 *)*param_2 + iStack6 * 0x2) = *(undefined2 *)((int)lVar6 + 0x2e);
        }
    }
    return;
}

void __stdcall16far pass1_1018_3710(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    undefined4 uVar1;
    code     **ppcVar2;
    int        iVar3;
    uint       uVar4;
    uchar     *puVar5;
    int        iVar6;
    undefined2 uVar7;
    undefined  in_AF;
    long       lVar8;
    ushort    *puVar9;
    undefined  local_12a[0x118];
    undefined4 uStack18;
    ushort    *puStack14;
    ulong      uStack10;
    ushort    *puStack6;

    puStack6 = (ushort *)0x0;
    uVar7    = (undefined2)((ulong)param_1 >> 0x10);
    iVar6    = (int)param_1;
    uStack10 = switch_1018_3b9e((ulong)param_1, *(ushort *)(iVar6 + 0x12e), param_3, param_4, param_2);
    uVar4    = *(int *)(iVar6 + 0x12e) - 0x188;
    uStack18 = (ushort *)(uStack10 & 0xffff0000 | (ulong)uVar4);
    switch(uVar4)
    {
    case 0x0:
        lVar8  = pass1_1008_57f0(uStack10, *(int *)(iVar6 + 0x130), param_2);
        puVar5 = (uchar *)((ulong)lVar8 >> 0x10);
        iVar3  = (int)lVar8;
        mem_op_1000_179c(0x10, puVar5, 0x1000);
        if(lVar8 != 0x0)
        {
            uStack18 = (ushort *)struct_1018_4790(lVar8, *(undefined4 *)(iVar6 + 0x132), 0x0, *(ushort *)(iVar3 + 0xe));
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x1:
        puVar9 = (ushort *)pass1_1008_57f0(uStack10, *(int *)(iVar6 + 0x130), param_2);
        puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
        iVar3  = (int)puVar9;
        mem_op_1000_179c(0x14, puVar5, 0x1000);
        uVar4 = (uint)((ulong)puVar9 >> 0x10) | (uint)puVar9;
        if(puVar9 != (ushort *)0x0)
        {
            struct_1018_47c8(puVar9, *(ulong *)(iVar6 + 0x132), 0x0, *(ushort *)(iVar3 + 0x12), *(ulong *)(iVar3 + 0xe));
            uStack18 = (ushort *)((ulong)puVar9 & 0xffff | (ulong)uVar4 << 0x10);
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x2:
        puVar9 = (ushort *)pass1_1008_57f0(uStack10, *(int *)(iVar6 + 0x130), param_2);
        puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
        iVar3  = (int)puVar9;
        mem_op_1000_179c(0x12, puVar5, 0x1000);
        uVar4 = (uint)((ulong)puVar9 >> 0x10) | (uint)puVar9;
        if(puVar9 != (ushort *)0x0)
        {
            pass1_1018_4808(puVar9, *(ulong *)(iVar6 + 0x132), 0x0, *(ulong *)(iVar3 + 0xe));
            uStack18 = (ushort *)((ulong)puVar9 & 0xffff | (ulong)uVar4 << 0x10);
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x3:
        puVar9 = (ushort *)pass1_1008_57f0(uStack10, *(int *)(iVar6 + 0x130), param_2);
        puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
        iVar3  = (int)puVar9;
        mem_op_1000_179c(0x14, puVar5, 0x1000);
        if(puVar9 != (ushort *)0x0)
        {
            uStack18 = struct_1018_4842(puVar9, *(ulong *)(iVar6 + 0x132), 0x0, *(ushort *)(iVar3 + 0xe));
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x4:
        puVar9 = (ushort *)pass1_1008_57f0(uStack10, *(int *)(iVar6 + 0x130), param_2);
        puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
        iVar3  = (int)puVar9;
        mem_op_1000_179c(0x10, puVar5, 0x1000);
        if(puVar9 != (ushort *)0x0)
        {
            uStack18 = struct_1018_48b0(puVar9, *(ulong *)(iVar6 + 0x132), 0x0, *(ushort *)(iVar3 + 0xe));
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x5:
        puVar9 = (ushort *)pass1_1008_57f0(uStack10, *(int *)(iVar6 + 0x130), param_2);
        puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
        iVar3  = (int)puVar9;
        mem_op_1000_179c(0x12, puVar5, 0x1000);
        if(puVar9 != (ushort *)0x0)
        {
            uStack18 = (ushort *)struct_1018_4920(puVar9, *(ulong *)(iVar6 + 0x132), 0x0, *(ulong *)(iVar3 + 0xe));
            puStack6 = uStack18;
        }
        break;
    default:
        goto switchD_1018_393f_caseD_6;
    }
    uStack18 = (ushort *)0x0;
    puStack6 = uStack18;
switchD_1018_393f_caseD_6:
    uVar1 = *(undefined4 *)(iVar6 + 0x122);
    pass1_1008_e852((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), *(ulong *)(iVar6 + 0x126), param_2, (uint)((ulong)uStack18 >> 0x10));
    uVar1     = *(undefined4 *)(iVar6 + 0x122);
    puStack14 = uStack18;
    pass1_1008_e852((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), *(ulong *)(iVar6 + 0x12a), param_2, (uint)((ulong)uStack18 >> 0x10));
    pass1_1038_2a0e((astruct_100 *)CONCAT22(param_2, local_12a), *(ulong *)(iVar6 + 0x136), (ulong)puStack6, (ulong)uStack18, (ulong)puStack14, param_2, in_AF);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_2, local_12a));
    *(undefined4 *)(iVar6 + 0x136) = 0x0;
    ppcVar2                        = (code **)((int)*param_1 + 0x10);
    (**ppcVar2)(0x1030, param_1);
    pass1_1038_2a5c((ushort *)CONCAT22(param_2, local_12a));
    return;
}

void __stdcall16far get_sys_metrics_1018_1ea0(astruct_55 *param_1, ushort param_2)

{
    INT16       IVar1;
    INT16       IVar2;
    astruct_55 *iVar3;
    undefined2  uVar3;

    IVar1             = GetSystemMetrics16(param_2);
    uVar3             = (undefined2)((ulong)param_1 >> 0x10);
    iVar3             = (astruct_55 *)param_1;
    iVar3->field_0x2e = IVar1 * 0x2 + iVar3->field_0x36;
    IVar1             = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    IVar2             = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    iVar3->field_0x30 = IVar1 + iVar3->field_0x38 + IVar2;
    return;
}

ulong __stdcall16far pass1_1018_1ff4(astruct_634 *param_1, ushort param_2, ushort param_3)

{
    int        *piVar1;
    int         unaff_DI;
    ushort      unaff_SS;
    astruct_79 *paVar2;
    int        *piVar3;
    int        *piVar4;
    ushort      uVar5;
    int         local_a;
    int         local_8;
    undefined4  uStack6;

    paVar2                                    = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0xa        = 0xb9010b;
    param_1->field_0xe                        = 0x170035;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x21e8;
    param_1->field_0x2                        = 0x1018;
    piVar4                                    = &local_8;
    piVar3                                    = &local_a;
    uVar5                                     = unaff_SS;
    uStack6                                   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, (uchar *)((ulong)paVar2 >> 0x10), unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0xe)), (ushort *)CONCAT22(unaff_SS, piVar3), (ushort *)CONCAT22(uVar5, piVar4));
    piVar1  = &param_1->field_0xa;
    *piVar1 = *piVar1 + local_8;
    piVar1  = &param_1->field_0xc;
    *piVar1 = *piVar1 + local_a;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, param_1 + 0x1), (WNDCLASS16 *)0x0, 0x7f4);
    return CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1018_270e(ulong param_1, int param_2, uint param_3, uchar *param_4, int param_5, ushort param_6)

{
    code       **ppcVar1;
    ulong        uVar2;
    int          iVar3;
    undefined2   uVar4;
    uchar       *extraout_DX;
    astruct_655 *iVar5;
    undefined2   uVar5;
    ushort      *puVar6;

    iVar5 = (astruct_655 *)param_1;
    uVar5 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        if((iVar5->field_0x20 == 0x0) || (uVar2 = iVar5->field_0x20, *(uint *)((int)uVar2 + 0x8) != param_3))
        {
            puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, param_3, param_6, param_4, param_5);
            if(iVar5->field_0x20 != 0x0)
            {
                if(param_1 == 0x0)
                {
                    iVar3 = 0x0;
                    uVar4 = 0x0;
                }
                else
                {
                    iVar3 = &iVar5->field_0x1c;
                    uVar4 = uVar5;
                }
                pass1_1010_1ea6(iVar5->field_0x20, CONCAT22(uVar4, iVar3), param_6);
            }
            iVar5->field_0x20 = (ulong)puVar6;
            if(param_1 == 0x0)
            {
                param_3 = 0x0;
                uVar4   = 0x0;
            }
            else
            {
                param_3 = &iVar5->field_0x1c;
                uVar4   = uVar5;
            }
            uVar2   = iVar5->field_0x20;
            ppcVar1 = (code **)((int)*(undefined4 *)iVar5->field_0x20 + 0x4);
            (**ppcVar1)(0x1010, (int)uVar2, (int)(uVar2 >> 0x10), 0x0, param_3, uVar4);
            param_4 = extraout_DX;
        }
        pass1_1018_2862(param_1);
        if(((uint)param_4 | param_3) != 0x0)
        {
            iVar5->field_0x24 = 0x1;
        }
        pass1_1010_1f62(param_6, param_1, 0x7);
    }
    else
    {
        if((*(uint *)((int)&iVar5->field_0x20 + 0x2) | *(uint *)&iVar5->field_0x20) != 0x0)
        {
            if(param_1 == 0x0)
            {
                iVar3 = 0x0;
                uVar4 = 0x0;
            }
            else
            {
                iVar3 = &iVar5->field_0x1c;
                uVar4 = uVar5;
            }
            pass1_1010_1ea6(iVar5->field_0x20, CONCAT22(uVar4, iVar3), param_6);
            iVar5->field_0x20 = 0x0;
            return;
        }
    }
    return;
}

void __stdcall16far mixed_sys_op_1018_2978(ulong param_1, ushort param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined  *puVar2;
    undefined  *puVar3;
    RECT16     *rect;
    int         iVar4;
    uchar      *in_DX;
    uint        uVar5;
    uchar      *extraout_DX;
    uchar      *puVar6;
    uchar      *puVar7;
    int         iVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    undefined2  uVar11;
    undefined   uVar12;
    astruct_76 *paStack62;
    RECT16      local_3a;
    int         iStack54;
    int         iStack52;
    ulong       uStack50;
    undefined4 *puStack46;
    undefined   local_2a[0x24];
    ushort      uStack6;

    pass1_1010_8096(_PTR_LOOP_1050_14cc, 0x1);
    puVar2  = local_2a;
    uStack6 = param_2;
    struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3, puVar2), 0x1, (char *)CONCAT22(in_DX, param_2), (ushort)in_DX);
    uVar9 = 0x1000;
    mem_op_1000_179c(0x1e, in_DX, 0x1000);
    uVar5 = (uint)in_DX | (uint)puVar2;
    if(uVar5 == 0x0)
    {
        puVar3 = (undefined *)0x0;
        uVar5  = 0x0;
    }
    else
    {
        puVar3 = local_2a;
        uVar9  = 0x1008;
        struct_op_1008_3f92((astruct_76 *)CONCAT22(in_DX, puVar2), (astruct_83 *)CONCAT22(param_3, puVar3));
    }
    puStack46 = (undefined4 *)CONCAT22(uVar5, puVar3);
    ppcVar1   = (code **)((int)*puStack46 + 0x14);
    (**ppcVar1)(uVar9, puVar3, uVar5);
    uStack50 = CONCAT22(extraout_DX, puVar3);
    puVar6   = extraout_DX;
    mem_op_1000_179c(0x14, extraout_DX, 0x1000);
    puVar7 = (uchar *)((uint)puVar6 | (uint)puVar3);
    if(puVar7 == (uchar *)0x0)
    {
        puVar3 = (undefined *)0x0;
        puVar7 = (uchar *)0x0;
    }
    else
    {
        struct_1008_4c58((ushort *)CONCAT13((char)((uint)puVar6 >> 0x8), CONCAT12((char)puVar6, puVar3)));
    }
    uVar9                     = (undefined2)(param_1 >> 0x10);
    iVar8                     = (int)param_1;
    *(uint *)(iVar8 + 0xe)    = (uint)puVar3;
    *(uchar **)(iVar8 + 0x10) = puVar7;
    pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0xe), uStack50, puVar7);
    uVar12 = SUB21(PTR_LOOP_1050_0396, 0x0);
    rect   = &local_3a;
    GetClientRect16(0x1008, rect);
    uVar11 = 0x1e;
    uVar10 = 0x1000;
    mem_op_1000_179c(0x1e, puVar7, 0x1000);
    paStack62 = (astruct_76 *)CONCAT22(puVar7, rect);
    uVar5     = (uint)puVar7 | (uint)rect;
    if(uVar5 == 0x0)
    {
        *(undefined4 *)(iVar8 + 0xa) = 0x0;
    }
    else
    {
        iVar4  = (iStack52 - local_3a.y) + 0x1;
        uVar10 = 0x1008;
        pass1_1008_405c(paStack62, *(ulong *)(iVar8 + 0xe), iVar4, (iStack54 - local_3a.x) + 0x1);
        *(int *)(iVar8 + 0xa)  = iVar4;
        *(uint *)(iVar8 + 0xc) = uVar5;
    }
    if(puStack46 != (undefined4 *)0x0)
    {
        ppcVar1 = (code **)*puStack46;
        (**ppcVar1)(uVar10, (int)puStack46, (int)((ulong)puStack46 >> 0x10), 0x1, uVar11, uVar12);
    }
    close_file_1008_496c(local_2a, param_3);
    return;
}

void __stdcall16far pass1_1018_10c4(ushort param_1, uint param_2, ulong param_3)

{
    ulong       uVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    int         iVar4;
    undefined  *puVar5;
    uint        uVar6;
    ushort      uVar7;
    ulong       uVar8;
    uint        uVar9;
    uchar      *puVar10;
    uint        extraout_DX;
    uint        extraout_DX_00;
    uint        extraout_DX_01;
    uint        extraout_DX_02;
    int         iVar11;
    undefined2  uVar12;
    undefined   uVar13;
    bool        bVar14;
    ulong      *puVar15;
    ulong       uStack60;
    ulong       uStack56;
    undefined4  uStack52;
    undefined4 *puStack48;
    undefined4 *puStack40;
    uint        uStack30;
    uint        uStack28;
    undefined   local_16[0x8];
    undefined2  uStack14;
    uint        uStack12;
    undefined2  uStack10;
    uint        uStack8;
    int         iStack6;
    int         iStack4;

    uVar12  = (undefined2)(param_3 >> 0x10);
    iVar11  = (int)param_3;
    iStack4 = *(int *)(iVar11 + 0x86);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar11 + 0x88), 0x1000);
    *(undefined2 *)(iVar11 + 0x86) = 0x0;
    *(undefined4 *)(iVar11 + 0x88) = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_1 >> 0x8), CONCAT12((char)param_1, local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0x0;
    uStack28 = 0x0;
    while(true)
    {
        uVar9  = param_2;
        puVar5 = local_16;
        pass1_1028_e4ec(CONCAT22(param_1, puVar5));
        param_2 = uVar9 | (uint)puVar5;
        if(param_2 == 0x0)
            break;
        if(*(long *)(iVar11 + 0x3c) == *(long *)(puVar5 + 0x8))
        {
            puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
            puVar10 = (uchar *)((ulong)puVar15 >> 0x10);
            uVar6   = (uint)puVar15;
            pass1_1038_4e78(uVar6, puVar10, CONCAT22(uVar9, puVar5), puVar15);
            puStack48 = (undefined4 *)CONCAT22(puVar10, uVar6);
            uVar3     = *puStack48;
            ppcVar2   = (code **)uVar3 + 0x8;
            uVar9     = uVar6;
            (**ppcVar2)((int)&PTR_LOOP_1050_1038, uVar6, puVar10);
            bVar14   = CARRY2(uStack30, uVar9);
            uStack30 = uStack30 + uVar9;
            uStack28 = uStack28 + extraout_DX + (uint)bVar14;
            param_2  = extraout_DX;
            if(puStack48 != (undefined4 *)0x0)
            {
                ppcVar2 = (code **)uVar3;
                (**ppcVar2)(0x38, uVar6, puVar10, 0x1);
                param_2 = extraout_DX_00;
            }
        }
    }
    if((uStack28 | uStack30) != 0x0)
    {
        *(uint *)(iVar11 + 0x86) = uStack30;
        uVar7                    = uStack30 * 0x6;
        mem_op_1000_179c(uVar7, (uchar *)0x0, 0x1000);
        uStack52 = CONCAT22(param_2, uVar7);
        if((param_2 | uVar7) == 0x0)
        {
            *(undefined4 *)(iVar11 + 0x88) = 0x0;
        }
        else
        {
            pass1_1000_5586((uchar *)0x3e38, 0x1008, uStack30, 0x6, uVar7, param_2);
            *(undefined4 *)(iVar11 + 0x88) = uStack52;
        }
        if(iStack6 != 0x0)
        {
            uStack10 = 0x1;
            uStack8  = 0x0;
        }
        iVar4    = 0x0;
        uStack14 = uStack10;
        uStack12 = uStack8;
        while(true)
        {
            uVar9  = uStack8;
            puVar5 = local_16;
            pass1_1028_e4ec(CONCAT22(param_1, puVar5));
            if((uVar9 | (uint)puVar5) == 0x0)
                break;
            uStack8 = uVar9 | (uint)puVar5;
            if(*(long *)(iVar11 + 0x3c) == *(long *)(puVar5 + 0x8))
            {
                puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
                puVar10 = (uchar *)((ulong)puVar15 >> 0x10);
                uVar6   = (uint)puVar15;
                uVar13  = 0x38;
                pass1_1038_4e78(uVar6, puVar10, CONCAT13((char)(uVar9 >> 0x8), CONCAT12((char)uVar9, puVar5)), puVar15);
                puStack40 = (undefined4 *)CONCAT22(puVar10, uVar6);
                ppcVar2   = (code **)((int)*puStack40 + 0x10);
                uVar9     = uVar6;
                (**ppcVar2)(0x38, uVar6, puVar10);
                uStack56 = CONCAT22(extraout_DX_01, uVar9);
                uStack8  = extraout_DX_01;
                for(uStack60 = 0x0; uStack60 < uStack56; uStack60 = uStack60 + 0x1)
                {
                    uVar8 = uStack56;
                    pass1_1030_1d58((ulong)puStack40);
                    uVar1  = *(ulong *)(iVar11 + 0x88);
                    uVar13 = 0x8;
                    pass1_1008_3f62((ushort *)(uVar1 & 0xff000000 | (ulong)CONCAT12((char)(uVar1 >> 0x10), (int)uVar1 + iVar4 * 0x6)), (ushort *)CONCAT22(uStack8, (int)uVar8 + 0xc));
                    iVar4 = iVar4 + 0x1;
                }
                if(puStack40 != (undefined4 *)0x0)
                {
                    ppcVar2 = (code **)*puStack40;
                    (**ppcVar2)(uVar13, (char)uVar6, puVar10, 0x1);
                    uStack8 = extraout_DX_02;
                }
            }
        }
        if(*(int *)(iVar11 + 0x86) != iStack4)
        {
            pass1_1010_1f62(param_1, param_3, 0x6);
        }
    }
    return;
}

void __stdcall16far pass1_1018_1320(ulong param_1, ushort *param_2, ulong *param_3)

{
    undefined2 uVar1;

    uVar1    = (undefined2)(param_1 >> 0x10);
    *param_3 = *(ulong *)((int)param_1 + 0x88);
    *param_2 = *(ushort *)((int)param_1 + 0x86);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_1346(ushort param_1, uint param_2, astruct_93 *param_3)

{
    code      **ppcVar1;
    int         iVar2;
    undefined  *puVar3;
    uint        uVar4;
    ushort      uVar5;
    uchar      *puVar6;
    uint        extraout_DX;
    uint        extraout_DX_00;
    uint        extraout_DX_01;
    uint        uVar7;
    uint        extraout_DX_02;
    astruct_93 *iVar9;
    undefined2  uVar8;
    undefined   uVar9;
    ulong      *puVar10;
    ulong       uVar11;
    ulong       uVar12;
    ulong       uStack70;
    undefined4 *puStack56;
    ulong       uStack52;
    undefined4 *puStack48;
    undefined4  uStack30;
    undefined   local_16[0x8];
    undefined2  uStack14;
    uint        uStack12;
    undefined2  uStack10;
    uint        uStack8;
    int         iStack6;
    uint        uStack4;

    uVar8   = (undefined2)((ulong)param_3 >> 0x10);
    iVar9   = (astruct_93 *)param_3;
    uStack4 = iVar9->field_0x8c;
    fn_ptr_1000_17ce((astruct_18 *)iVar9->field_0x8e, 0x1000);
    iVar9->field_0x8c = 0x0;
    iVar9->field_0x8e = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_1 >> 0x8), CONCAT12((char)param_1, local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0x0;
    while(true)
    {
        uVar7  = param_2;
        puVar3 = local_16;
        pass1_1028_e4ec(CONCAT22(param_1, puVar3));
        param_2 = uVar7 | (uint)puVar3;
        if(param_2 == 0x0)
            break;
        if(iVar9->field_0x3c == *(long *)(puVar3 + 0x8))
        {
            puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
            puVar6  = (uchar *)((ulong)puVar10 >> 0x10);
            uVar4   = (uint)puVar10;
            uVar9   = 0x38;
            pass1_1038_4e78(uVar4, puVar6, CONCAT22(uVar7, puVar3), puVar10);
            puStack48 = (undefined4 *)CONCAT22(puVar6, uVar4);
            ppcVar1   = (code **)((int)*puStack48 + 0x10);
            uVar7     = uVar4;
            (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar4, puVar6);
            uStack52 = CONCAT22(extraout_DX, uVar7);
            param_2  = extraout_DX;
            for(puStack56 = (undefined4 *)0x0; puStack56 < uStack52; puStack56 = (undefined4 *)((long)puStack56 + 0x1))
            {
                uVar9   = 0x30;
                uVar11  = pass1_1030_1d7c(uVar7, param_2, (ulong)puStack48);
                param_2 = (uint)(uVar11 >> 0x10);
                if(*(int *)((int)uVar11 + 0x12) == 0x9)
                {
                    uStack30 = uStack30 + 0x1;
                }
            }
            if(puStack48 != (undefined4 *)0x0)
            {
                ppcVar1 = (code **)*puStack48;
                (**ppcVar1)(uVar9, uVar4, puVar6, 0x1);
                param_2 = extraout_DX_00;
            }
        }
    }
    if((uStack30._2_2_ | (uint)uStack30) != 0x0)
    {
        iVar9->field_0x8c = (uint)uStack30;
        uVar5             = (uint)uStack30 * 0x6;
        mem_op_1000_179c(uVar5, (uchar *)0x0, 0x1000);
        uStack70 = CONCAT22(param_2, uVar5);
        if((param_2 | uVar5) == 0x0)
        {
            iVar9->field_0x8e = 0x0;
        }
        else
        {
            pass1_1000_5586((uchar *)0x3e38, 0x1008, (uint)uStack30, 0x6, uVar5, param_2);
            iVar9->field_0x8e = uStack70;
        }
        if(iStack6 != 0x0)
        {
            uStack10 = 0x1;
            uStack8  = 0x0;
        }
        iVar2    = 0x0;
        uStack14 = uStack10;
        uStack12 = uStack8;
        while(true)
        {
            uVar7  = uStack8;
            puVar3 = local_16;
            pass1_1028_e4ec(CONCAT22(param_1, puVar3));
            if((uVar7 | (uint)puVar3) == 0x0)
                break;
            uStack8 = uVar7 | (uint)puVar3;
            if(iVar9->field_0x3c == *(long *)(puVar3 + 0x8))
            {
                puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
                puVar6  = (uchar *)((ulong)puVar10 >> 0x10);
                uVar4   = (uint)puVar10;
                uVar9   = 0x38;
                pass1_1038_4e78(uVar4, puVar6, CONCAT13((char)(uVar7 >> 0x8), CONCAT12((char)uVar7, puVar3)), puVar10);
                puStack56 = (undefined4 *)CONCAT22(puVar6, uVar4);
                ppcVar1   = (code **)((int)*puStack56 + 0x10);
                uVar7     = uVar4;
                (**ppcVar1)(0x38, uVar4, puVar6);
                uStack52 = CONCAT22(extraout_DX_01, uVar7);
                uStack8  = extraout_DX_01;
                for(puStack48 = (undefined4 *)0x0; puStack48 < uStack52; puStack48 = (undefined4 *)((long)puStack48 + 0x1))
                {
                    uVar11 = uStack52;
                    pass1_1030_1d58((ulong)puStack56);
                    uVar9  = 0x30;
                    uVar12 = struct_op_1030_73a8(uVar11 & 0xffff | (ulong)uStack8 << 0x10);
                    uVar7  = (uint)(uVar12 >> 0x10);
                    if(*(int *)((int)uVar12 + 0x12) == 0x9)
                    {
                        uVar12 = iVar9->field_0x8e;
                        uVar9  = 0x8;
                        pass1_1008_3f62((ushort *)(uVar12 & 0xff000000 | (ulong)CONCAT12((char)(uVar12 >> 0x10), (int)uVar12 + iVar2 * 0x6)), (ushort *)CONCAT22(uStack8, (int)uVar11 + 0xc));
                        iVar2 = iVar2 + 0x1;
                    }
                    uStack8 = uVar7;
                }
                if(puStack56 != (undefined4 *)0x0)
                {
                    ppcVar1 = (code **)*puStack56;
                    (**ppcVar1)(uVar9, (char)uVar4, puVar6, 0x1);
                    uStack8 = extraout_DX_02;
                }
            }
        }
        if(iVar9->field_0x8c != uStack4)
        {
            pass1_1010_1f62(param_1, (ulong)param_3, 0x6);
        }
    }
    return;
}

void __stdcall16far pass1_1018_18b8(ushort param_1, astruct_55 *param_2, ushort param_3)

{
    uchar      *puVar1;
    astruct_55 *iVar3;
    int         unaff_DI;
    astruct_55 *uVar3;
    ushort     *puVar2;
    astruct_43 *paVar3;
    ulong       uVar4;
    int        *piVar5;
    ushort      uVar6;
    int        *piVar7;
    ushort      uVar8;
    int         local_6;
    int         local_4;
    ushort      uVar1;

    get_sys_metrics_1018_4b1e(param_2, 0x0, param_3);
    uVar3                                = (astruct_55 *)((ulong)param_2 >> 0x10);
    iVar3                                = (astruct_55 *)param_2;
    iVar3->field_0x20                    = 0x389a;
    iVar3->field_0x22                    = 0x1008;
    iVar3->field_0x20                    = 0x3aa8;
    iVar3->field_0x22                    = 0x1008;
    *(undefined4 *)&iVar3->field_0x24    = 0x0;
    iVar3->field_0x28                    = 0x4;
    puVar2                               = pass1_1008_3e38((ushort *)((ulong)param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
    puVar1                               = (uchar *)((ulong)puVar2 >> 0x10);
    *(undefined4 *)&iVar3[0x1].field_0x6 = 0x0;
    iVar3[0x1].field_0xa                 = 0x0;
    *(undefined4 *)&iVar3[0x1].field_0xc = 0x0;
    iVar3[0x1].field_0x10                = 0x0;
    iVar3[0x1].field_0x1c                = 0x0;
    param_2->field_0x0                   = 0x1fb0;
    iVar3->field_0x2                     = 0x1018;
    iVar3->field_0x20                    = 0x1fec;
    iVar3->field_0x22                    = 0x1018;
    pass1_1000_4906((astruct_20 *)((ulong)param_2 & 0xffff0000 | (ulong)(uint)&iVar3[0x1].field_0x14), (WNDCLASS16 *)0x0, 0x8);
    piVar7 = &local_4;
    piVar5 = &local_6;
    uVar6  = param_1;
    uVar8  = param_1;
    puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_1, puVar1, unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)), (ushort *)CONCAT22(uVar6, piVar5), (ushort *)CONCAT22(uVar8, piVar7));
    paVar3                            = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x9a, param_1);
    iVar3->field_0x24                 = (ushort)paVar3;
    *(undefined2 *)&iVar3->field_0x26 = (int)((ulong)paVar3 >> 0x10);
    uVar4                             = pass1_1008_4772((astruct_76 *)((ulong)paVar3 & 0xffff0000 | (ulong)iVar3->field_0x24));
    uVar1                             = (ushort)(uVar4 >> 0x10);
    pass1_1000_4906((astruct_20 *)((ulong)param_2 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x32), (WNDCLASS16 *)0x0, 0x8);
    iVar3->field_0x36 = *(int *)((int)uVar4 + 0x4);
    iVar3->field_0x38 = *(int *)((int)uVar4 + 0x8);
    iVar3->field_0x2a = local_4 + 0x14;
    iVar3->field_0x2c = local_6 + 0x14;
    get_sys_metrics_1018_1ea0(param_2, 0x1000);
    pass1_1008_3e76((ushort *)((ulong)param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)), 0x0, 0x88, 0x99);
    return;
}

void __stdcall16far pass1_1018_1a8e(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    long         lVar1;
    astruct_653 *iVar2;
    uint         uVar2;
    ushort      *puVar3;
    int         *piVar4;
    int          local_8;
    undefined4   uStack6;

    uVar2 = (uint)(param_1 >> 0x10);
    iVar2 = (astruct_653 *)param_1;
    if(iVar2->field_0x44 != 0x0)
    {
        if(iVar2->field_0x46 != 0x0)
        {
            lVar1                             = iVar2->field_0x46;
            *(undefined2 *)((int)lVar1 + 0xe) = 0x0;
            iVar2->field_0x46                 = 0x0;
        }
        piVar4  = &iVar2->field_0x4a;
        *piVar4 = *piVar4 + 0x1;
        return;
    }
    piVar4 = (int *)CONCAT22(param_4, &local_8);
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, param_2, param_3);
    pass1_1010_bf1e((ulong)puVar3, piVar4, (int)puVar3, (uchar *)((ulong)puVar3 >> 0x10), param_4);
    iVar2->field_0x44 = local_8;
    iVar2->field_0x40 = uStack6;
    pass1_1018_1ce8(param_4, param_1 & 0xffff | (ulong)uVar2 << 0x10);
    return;
}

void __stdcall16far pass1_1010_e964(uchar *param_1, ushort param_2, int param_3)

{
    ulong   uVar1;
    uint    uVar2;
    ushort *puVar3;

    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_2, param_1, param_3);
    uVar2  = (uint)((ulong)puVar3 >> 0x10);
    uVar1  = *(ulong *)((int)puVar3 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)(uVar1 >> 0x10));
    pass1_1038_4d28(uVar1 & 0xffff | (ulong)uVar2 << 0x10);
    return;
}

void __stdcall16far struct_1010_e9e4(astruct_261 *param_1, ushort param_2, ushort param_3)

{
    uint       *puVar1;
    uint        uVar2;
    int         iVar3;
    uint        uVar4;
    uint        uVar5;
    ulong       uVar6;
    uchar      *puVar7;
    int         iVar8;
    astruct_79 *paVar9;
    ushort     *puVar10;
    int         iStack4;

    paVar9                                    = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    puVar7                                    = (uchar *)((ulong)paVar9 >> 0x10);
    param_1->field_0xa                        = 0x389a;
    param_1->field_0xc                        = 0x1008;
    param_1->field_0xa                        = 0x3aa8;
    param_1->field_0xc                        = 0x1008;
    uVar5                                     = 0x0;
    *(undefined4 *)&param_1->field_0xe        = 0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x16                       = 0x0;
    param_1->field_0x1a                       = 0x0;
    param_1->field_0x1e                       = 0x0;
    param_1->field_0x20                       = 0x0;
    param_1->field_0x24                       = 0x0;
    param_1->field_0x28                       = 0x0;
    param_1->field_0x2c                       = 0x0;
    param_1->field_0x30                       = 0x0;
    param_1->field_0x32                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x558;
    param_1->field_0x2                        = 0x1018;
    param_1->field_0xa                        = 0x568;
    param_1->field_0xc                        = 0x1018;
    mem_op_1000_179c(0x4, puVar7, 0x1000);
    if(((uint)puVar7 | uVar5) == 0x0)
    {
        *(undefined4 *)&param_1->field_0xe = 0x0;
    }
    else
    {
        puVar10             = pass1_1018_dcf6((ushort *)CONCAT22(puVar7, uVar5));
        param_1->field_0xe  = (int)puVar10;
        param_1->field_0x10 = (int)((ulong)puVar10 >> 0x10);
    }
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x34), (WNDCLASS16 *)0x0, 0x24);
    param_1->field_0x38 = 0xfa;
    param_1->field_0x3c = 0x15e;
    uVar6               = 0x1c2;
    param_1->field_0x40 = 0x1c2;
    param_1->field_0x44 = 0x1c2;
    param_1->field_0x46 = 0x2260000;
    param_1->field_0x4a = 0x28a0000;
    param_1->field_0x4e = 0x730000;
    param_1->field_0x52 = 0x960000;
    param_1->field_0x56 = 0x0;
    for(iStack4 = 0x1; iStack4 < 0x9; iStack4 = iStack4 + 0x1)
    {
        pass1_1008_612e(0x0, 0x1d, (uint)uVar6);
        uVar5 = (uint)uVar6;
        pass1_1008_612e(0x1, 0x2, uVar5);
        if((uVar6 & 0x1) != 0x0)
        {
            uVar5 = -uVar5;
        }
        iVar8                                   = iStack4 * 0x4;
        puVar1                                  = (uint *)(&param_1->field_0x34 + iVar8);
        uVar2                                   = *puVar1;
        uVar4                                   = uVar5 + *puVar1;
        uVar6                                   = (ulong)uVar4;
        iVar3                                   = *(int *)(&param_1->field_0x34 + iVar8 + 0x2);
        *(uint *)(&param_1->field_0x34 + iVar8) = uVar4;
        *(int *)(&param_1->field_0x36 + iVar8)  = ((int)uVar5 >> 0xf) + iVar3 + (uint)CARRY2(uVar5, uVar2);
    }
    return;
}

void __stdcall16far pass1_1018_0196(ulong param_1, ulong param_2, ulong param_3, ushort param_4, uchar *param_5, ushort param_6)

{
    int       *piVar1;
    int        iVar2;
    undefined4 uVar3;
    ulong      uVar4;
    ushort     uVar5;
    ulong      uVar6;
    int        iVar7;
    undefined2 uVar8;
    long       lVar9;

    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), param_3);
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar7 = (int)param_1;
    if(*(long *)(iVar7 + 0x2c) == 0x0)
    {
        *(undefined2 *)(iVar7 + 0x32) = 0x5;
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_5, 0x1000);
            PTR_LOOP_1050_5f2e = param_5;
        }
        else
        {
        }
        uVar5 = fn_ptr_op_1000_1708(0x1e, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    }
    else
    {
        uVar5              = *(int *)(iVar7 + 0x30) + 0x1;
        PTR_LOOP_1050_5f2e = param_5;
        if((int)uVar5 < *(int *)(iVar7 + 0x32))
            goto LAB_1018_022a;
        piVar1             = (int *)(iVar7 + 0x32);
        *piVar1            = *piVar1 + 0x5;
        uVar3              = *(undefined4 *)(iVar7 + 0x2c);
        lVar9              = pass1_1000_0ed4(0x1000, param_6, 0x1, *(int *)(iVar7 + 0x32) * 0x6, 0x0, (ushort *)uVar3, (ushort)((ulong)uVar3 >> 0x10));
        PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar9 >> 0x10);
        uVar5              = (ushort)lVar9;
    }
    *(ushort *)(iVar7 + 0x2c) = uVar5;
    *(uchar **)(iVar7 + 0x2e) = PTR_LOOP_1050_5f2e;
LAB_1018_022a:
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), param_2);
    uVar6 = *(ulong *)(uVar5 + 0x10);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), uVar6);
    iVar2   = *(int *)(iVar7 + 0x30);
    piVar1  = (int *)(iVar7 + 0x30);
    *piVar1 = *piVar1 + 0x1;
    uVar4   = *(ulong *)(iVar7 + 0x2c);
    pass1_1008_3f62((ushort *)(uVar4 & 0xffff0000 | (ulong)(uint)((int)uVar4 + iVar2 * 0x6)), (ushort *)CONCAT22(PTR_LOOP_1050_5f2e, (int)uVar6 + 0xc));
    return;
}

void __stdcall16far pass1_1018_028c(ulong param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5)

{
    undefined4  uVar1;
    code      **ppcVar2;
    undefined  *puVar3;
    uint        uVar4;
    int         iVar5;
    ulong       uVar6;
    uchar      *puVar7;
    uchar      *puVar8;
    uint        uVar9;
    uint        extraout_DX;
    ushort      uVar10;
    int         iVar11;
    int         unaff_DI;
    undefined2  uVar12;
    undefined2  uVar13;
    undefined2  uVar14;
    uint        uVar15;
    int         iStack36;
    undefined4 *puStack28;
    undefined   local_18[0x4];
    uint        uStack20;
    ushort     *puStack12;
    ushort      uStack8;
    ushort      uStack6;
    uchar      *puStack4;

    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), param_2);
    uStack6   = param_3;
    puStack4  = (uchar *)param_4;
    uStack8   = pass1_1030_5b00(CONCAT22(param_4, param_3));
    puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, uStack8, param_5, (uchar *)param_4, unaff_DI);
    pass1_1008_6c90((ushort *)CONCAT22(param_5, local_18));
    pass1_1018_0b1e(puStack12, (ushort *)CONCAT22(param_5, local_18), param_5);
    puVar7 = (uchar *)((int)uStack20 >> 0xf);
    if(((uint)puVar7 | uStack20) == 0x0)
    {
        puVar3 = local_18;
        pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(param_5, puVar3), param_2, param_5);
    }
    else
    {
        puVar3 = local_18;
        pass1_1030_62e4(_PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_5, puVar3), param_2, param_5);
    }
    puStack28 = (undefined4 *)CONCAT22(puVar7, puVar3);
    uVar4     = (uint)puVar7 | (uint)puVar3;
    if(uVar4 == 0x0)
    {
        return;
    }
    puVar8 = puVar7;
    pass1_1018_04f2(param_1);
    uVar14 = 0x1c;
    uVar13 = 0x1000;
    mem_op_1000_179c(0x1c, puVar8, 0x1000);
    uVar9  = (uint)puVar8 | uVar4;
    iVar11 = (int)param_1;
    uVar12 = (undefined2)(param_1 >> 0x10);
    uVar15 = uVar4;
    if(uVar9 == 0x0)
    {
        *(undefined4 *)(iVar11 + 0x12) = 0x0;
    }
    else
    {
        uVar13 = 0x1008;
        struct_op_1008_8e9e((astruct_78 *)CONCAT22(puVar8, uVar4), 0x6, 0x24);
        *(uint *)(iVar11 + 0x12) = uVar4;
        *(uint *)(iVar11 + 0x14) = uVar9;
    }
    ppcVar2 = (code **)((int)*puStack28 + 0x10);
    (**ppcVar2)(uVar13, puVar3, puVar7, uVar14, uVar15);
    for(iStack36 = 0x0; iStack36 < (int)uVar4; iStack36 = iStack36 + 0x1)
    {
        uVar6   = SEXT24(iStack36);
        ppcVar2 = (code **)((int)*puStack28 + 0x4);
        (**ppcVar2)();
        if((extraout_DX | (uint)uVar6) != 0x0)
        {
            iVar5  = iStack36 / 0x6;
            uVar10 = iStack36 % 0x6;
            uVar1  = *(undefined4 *)(iVar11 + 0xe);
            pass1_1018_dd7c((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), CONCAT22(iStack36 % 0x6, iVar5), uVar6 & 0xffff | (ulong)extraout_DX << 0x10, uVar10, param_5);
            pass1_1008_8faa(*(undefined4 *)(iVar11 + 0x12), CONCAT22(uVar10, iVar5));
        }
    }
    return;
}

void __stdcall16far pass1_1018_0412(ulong param_1, ushort param_2, ulong param_3, ushort param_4, ulong param_5, ushort param_6, uchar param_7)

{
    undefined *puVar1;
    undefined  local_128[0x124];
    ushort     uStack4;

    uStack4 = 0x0;
    if(((0x72 < (int)param_4) && (!SBORROW2(param_4, 0x73))) && ((param_4 == 0x75 || (int)(param_4 - 0x74) < 0x1 || ((0x0 < (int)(param_4 - 0x76) && ((int)(param_4 - 0x77) < 0x2))))))
    {
        uStack4 = 0x1;
    }
    struct_op_1028_933c((astruct_100 *)CONCAT22(param_6, local_128), param_2, uStack4, param_4, (ulong *)param_3, (ushort)(param_3 >> 0x10), *(ulong *)((int)param_1 + 0x24), param_5, param_6, param_7);
    puVar1 = local_128;
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, puVar1));
    if(puVar1 != (undefined *)0x0)
    {
        pass1_1010_1f62(param_6, param_1, 0x6);
    }
    return;
}

void __stdcall16far pass1_1018_04a4(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x16) = param_2;
    return;
}


ulong __stdcall16far pass1_1018_04b8(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x18), *(undefined2 *)((int)param_1 + 0x16));
}


void __stdcall16far pass1_1018_04ca(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x1a) = param_2;
    return;
}


void __stdcall16far pass1_1018_04de(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x20) = param_2;
    return;
}

void __stdcall16far struct_1018_0570(astruct_55 *param_1, ushort param_2, ushort param_3)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    undefined2  *puVar3;
    ushort       uVar4;
    uchar       *puVar5;
    undefined2   uVar6;
    uchar       *extraout_DX;
    int          unaff_DI;
    ushort      *puVar7;
    undefined2   uVar8;
    astruct_262 *uVar9;

    uVar9 = (astruct_262 *)param_1;
    uVar8 = (undefined2)((ulong)param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, 0x0, param_2);
    uVar9->field_0x20 = 0x389a;
    uVar9->field_0x22 = 0x1008;
    uVar9->field_0x20 = 0x3aa8;
    uVar9->field_0x22 = 0x1008;
    uVar9->field_0x24 = 0x0;
    uVar9->field_0x2c = (undefined4 *)0x0;
    pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&uVar9->field_0x30));
    puVar7            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&uVar9->field_0x36));
    puVar5            = (uchar *)((ulong)puVar7 >> 0x10);
    uVar9->field_0x3c = 0x0;
    pass1_1008_6c90((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&uVar9->field_0x40));
    uVar6                             = 0x0;
    uVar9->field_0x4c                 = 0x0;
    uVar9->field_0x5a                 = 0x0;
    uVar9->field_0x5e                 = 0x0;
    uVar9->field_0x60                 = 0x0;
    uVar9->field_0x64                 = 0xff00;
    uVar9->field_0x66                 = 0x0;
    uVar9->field_0x68                 = 0x10000fb;
    uVar9->field_0x6c                 = 0x10000f9;
    uVar9->field_0x70                 = 0x10000ff;
    uVar9->field_0x74                 = 0x10000fe;
    uVar9->field_0x78                 = 0x10000fc;
    uVar9->field_0x7c                 = 0x0;
    uVar9->field_0x80                 = 0x0;
    uVar9->field_0x84                 = 0x1;
    uVar9->field_0x86                 = 0x0;
    uVar9->field_0x88                 = 0x0;
    uVar9->field_0x8c                 = 0x0;
    uVar9->field_0x8e                 = 0x0;
    uVar9->field_0x92                 = 0x0;
    uVar9->field_0x94                 = 0x0;
    uVar9->field_0x98                 = 0x0;
    uVar9->field_0x9a                 = 0x0;
    *(undefined4 *)&uVar9->field_0xa2 = 0x0;
    uVar9->field_0xa6                 = 0xffff;
    uVar9->field_0xa8                 = 0x0;
    param_1->field_0x0                = 0x1874;
    uVar9->field_0x2                  = 0x1018;
    uVar9->field_0x20                 = 0x18b0;
    uVar9->field_0x22                 = 0x1018;
    if((PTR_LOOP_1050_3960 == (undefined *)0x0) && (_PTR_LOOP_1050_3962 == 0x0))
    {
        mem_op_1000_179c(0x8, puVar5, 0x1000);
        _PTR_LOOP_1050_3962 = CONCAT22(puVar5, uVar6);
        pass1_1000_4906((astruct_20 *)CONCAT22(puVar5, uVar6), (WNDCLASS16 *)0x0, 0x8);
    }
    PTR_LOOP_1050_3960                             = PTR_LOOP_1050_3960 + 0x1;
    puVar7                                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_3, puVar5, unaff_DI);
    *(int *)&uVar9->field_0x2c                     = (int)puVar7;
    *(undefined2 *)((int)&uVar9->field_0x2c + 0x2) = (int)((ulong)puVar7 >> 0x10);
    if(param_1 == (astruct_55 *)0x0)
    {
        puVar3 = (undefined2 *)0x0;
        uVar6  = 0x0;
    }
    else
    {
        puVar3 = &uVar9->field_0x20;
        uVar6  = uVar8;
    }
    puVar1  = uVar9->field_0x2c;
    ppcVar2 = (code **)((int)*uVar9->field_0x2c + 0x4);
    (**ppcVar2)(0x1010, (int)puVar1, (int)((ulong)puVar1 >> 0x10), 0x0, puVar3, uVar6);
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_3, extraout_DX, unaff_DI);
    puVar5 = (uchar *)((ulong)puVar7 >> 0x10);
    if(*(int *)((int)puVar7 + 0x80) != 0x0)
    {
        uVar9->field_0x84 = 0x2;
    }
    puVar7            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, param_3, puVar5, unaff_DI);
    puVar5            = (uchar *)((ulong)puVar7 >> 0x10);
    uVar9->field_0x9e = (uint)puVar7;
    uVar9->field_0xa0 = puVar5;
    uVar4             = pass1_1010_65d0(param_3, (ulong)puVar7 & 0xffff0000 | (ulong)uVar9->field_0x9e, 0x88);
    if(uVar4 != 0x0)
    {
        uVar9->field_0xa8 = 0x1;
    }
    puVar7            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_3, puVar5, unaff_DI);
    uVar9->field_0xa2 = (int)puVar7;
    uVar9->field_0xa4 = (int)((ulong)puVar7 >> 0x10);
    return;
}

void __stdcall16far get_sys_metrics_1018_09a8(ulong param_1, INT16 param_2)

{
    undefined4 uVar1;
    INT16      IVar2;
    INT16      IVar3;
    uchar     *in_DX;
    int        iVar4;
    int        unaff_DI;
    undefined2 uVar5;
    ushort     unaff_SS;
    ushort    *puVar6;
    ushort    *puVar7;
    ushort    *puVar8;
    int        local_a;
    int        local_8;
    int        iStack6;
    INT16      IStack4;

    IStack4 = GetSystemMetrics16(param_2);
    uVar5   = (undefined2)(param_1 >> 0x10);
    iVar4   = (int)param_1;
    iStack6 = *(int *)(iVar4 + 0x12) + -0x2;
    puVar8  = (ushort *)CONCAT22(unaff_SS, &local_8);
    puVar7  = (ushort *)CONCAT22(unaff_SS, &local_a);
    puVar6  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)), puVar7, puVar8);
    *(int *)(iVar4 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
    *(int *)(iVar4 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
    IVar2                  = GetSystemMetrics16(0x1008);
    uVar1                  = *(undefined4 *)(iVar4 + 0x5a);
    *(int *)(iVar4 + 0x1c) = IVar2 * 0x2 + *(int *)((int)uVar1 + 0x4);
    IVar2                  = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    IVar3                  = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    uVar1                  = *(undefined4 *)(iVar4 + 0x5a);
    *(int *)(iVar4 + 0x1e) = IVar3 + IVar2 + *(int *)((int)uVar1 + 0x8);
    return;
}

void __stdcall16far pass1_1010_d448(uchar *param_1, ulong param_2, uint *param_3, uchar *param_4, uchar param_5, int param_6)

{
    uint       uVar1;
    ushort    *puVar2;
    undefined4 uVar3;
    ulong      uVar4;
    ushort    *puVar5;
    char      *pcVar6;
    int        iVar7;
    uint       uVar8;
    ushort     uVar9;
    int        iVar10;
    undefined2 uVar11;
    ulong      uVar12;
    ushort     uVar13;
    ushort     local_40c;
    ulong      uStack1034;
    ulong      uStack1030;
    uchar      local_402[0x400];

    uVar11     = (undefined2)((ulong)param_3 >> 0x10);
    iVar10     = (int)param_3;
    uStack1030 = struct_op_1030_73a8(*(ulong *)(iVar10 + 0x6));
    uVar8      = (uint)(uStack1030 >> 0x10);
    uVar1      = (uint)uStack1030;
    if((uVar8 | uVar1) != 0x0)
    {
        uStack1034 = *(ulong *)(uVar1 + 0x20);
        uVar1      = *(uint *)(uVar1 + 0x22);
        if((uVar1 | (uint)uStack1034) != 0x0)
        {
            local_40c = 0x0;
            puVar5    = &local_40c;
            uVar13    = (ushort)((ulong)param_1 >> 0x10);
            pass1_1010_d984((ushort)param_1, uVar13, (int *)CONCAT22(param_4, puVar5), 0x3, uStack1034 & 0xffff | (ulong)uVar1 << 0x10, (ulong)&PTR_DAT_1050_1805_1050_368e, (ulong)param_3, param_4, param_5);
            puVar2                             = *(ushort **)(iVar10 + 0x2);
            uVar9                              = *(ushort *)(iVar10 + 0x4);
            *(undefined2 *)((int)puVar2 + 0x4) = PTR_DAT_1050_1805_1050_368e;
            uVar3                              = *(undefined4 *)(iVar10 + 0x6);
            pcVar6                             = pass1_1010_b038(param_1, (ushort)uVar3, (ushort)((ulong)uVar3 >> 0x10), *(uchar **)((int)puVar2 + 0x4), param_6);
            unk_str_op_1000_3d3e((char *)CONCAT22(param_4, local_402), (char *)CONCAT22(uVar9, pcVar6));
            string_1040_a626(puVar2, (char *)CONCAT22(param_4, local_402), uVar9);
            uVar4                        = *(ulong *)(iVar10 + 0x2);
            uVar9                        = *(ushort *)(iVar10 + 0x4);
            iVar7                        = (int)uVar4;
            *(undefined2 *)(iVar7 + 0xe) = PTR_DAT_1050_1822_1050_3690;
            sys_1000_3f9c(local_402, param_4, 0x3920, (ushort)&USHORT_1050_1050, local_40c, &stack0xfffe, uVar9, 0x1000, param_4, param_5);
            string_1040_a626((ushort *)(uVar4 & 0xffff0000 | (ulong)(iVar7 + 0xa)), (char *)CONCAT22(param_4, local_402), uVar9);
            uVar4                          = *(ulong *)(iVar10 + 0x2);
            uVar11                         = *(undefined2 *)(iVar10 + 0x4);
            iVar10                         = (int)uVar4;
            *(undefined2 *)(iVar10 + 0x18) = PTR_DAT_1050_1823_1050_3692;
            uVar12                         = pass1_1028_62c8(uStack1030, (ushort)param_4);
            uVar9                          = (ushort)(uVar12 >> 0x10);
            sys_1000_3f9c(local_402, param_4, 0x3923, (ushort)&USHORT_1050_1050, (ushort)uVar12, &stack0xfffe, uVar11, 0x1000, param_4, param_5);
            string_1040_a626((ushort *)(uVar4 & 0xffff0000 | (ulong)(iVar10 + 0x14)), (char *)CONCAT22(param_4, local_402), uVar9);
            pass1_1010_dc36((ushort)param_1, uVar13, (uint)puVar5, param_2, param_3, (ushort)param_4);
        }
    }
    return;
}
