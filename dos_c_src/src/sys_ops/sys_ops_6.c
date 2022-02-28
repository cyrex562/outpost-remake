
void pass1_1020_04f6(u16 *param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    code       **ppcVar1;
    i16          iVar2;
    u16          uVar3;
    u8          *extraout_DX;
    astruct_662 *iVar4;
    u16          uVar4;
    u16         *puVar5;

    uVar4                     = (param_1 >> 0x10);
    iVar4                     = (astruct_662 *)param_1;
    *param_1                  = 0x389a;
    iVar4->field_0x2          = 0x1008;
    *param_1                  = 0x3aa8;
    iVar4->field_0x2          = 0x1008;
    iVar4->field_0x4          = param_2;
    *param_1                  = 0x3ab0;
    iVar4->field_0x2          = 0x1008;
    iVar4->field_0x6          = 0x0;
    iVar4->field_0xa          = 0x0;
    iVar4->field_0xc          = 0x0;
    iVar4->field_0xe          = 0x0;
    iVar4->field_0x10         = 0x0;
    *param_1                  = 0x75a;
    iVar4->field_0x2          = 0x1020;
    puVar5                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x1, param_5, param_3, param_4);
    uVar3                     = (puVar5 >> 0x10);
    &iVar4->field_0x6         = puVar5;
    (&iVar4->field_0x6 + 0x2) = uVar3;
    ppcVar1                   = (*iVar4->field_0x6 + 0x4);
    (**ppcVar1)(0x1010, &iVar4->field_0x6, uVar3, 0x0, param_1);
    puVar5           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_5, extraout_DX, param_4);
    iVar2            = puVar5;
    iVar4->field_0xa = (iVar2 + 0xa);
    iVar4->field_0xc = (iVar2 + 0xc);
    pass1_1008_3e94((puVar5 & 0xffff0000 | (iVar2 + 0xe)), (param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x10)), (param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0xe)));
    return;
}

u32 pass1_1018_dd1e(u16 param_1, u16 param_2, u8 *param_3, u16 param_4, u16 param_5, i16 param_6, u32 param_7)

{
    u16 uVar1;
    u32 uStack6;

    pass1_1010_81f6(0x1010, param_1, globals->_PTR_LOOP_1050_14cc, 0x0, param_7._2_2_);
    uStack6 = CONCAT22(param_3, param_2);
    mem_op_1000_179c(0x46, param_3, 0x1000);
    uVar1 = param_3 | param_2;
    if(uVar1 == 0x0)
    {
        param_2 = 0x0;
        uVar1   = 0x0;
    }
    else
    {
        pass1_1008_87cc((astruct_86 *)CONCAT22(param_3, param_2), param_6, param_7, param_7._2_2_, uStack6, 0x0, param_1);
    }
    pass1_1008_8bc6(param_1, uVar1, CONCAT22(uVar1, param_2));
    return CONCAT22(uVar1, param_2);
}

u16 *struct_1018_e100(u16 *param_1, u16 param_2, u8 *param_3, u16 param_4)

{
    astruct_268 *iVar1;
    i16          unaff_DI;
    u16          uVar1;
    u16         *puVar2;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_268 *)param_1;
    *param_1          = 0x389a;
    iVar1->field_0x2  = 0x1008;
    *param_1          = 0x3aa8;
    iVar1->field_0x2  = 0x1008;
    iVar1->field_0x4  = param_2;
    *param_1          = 0x3ab0;
    iVar1->field_0x2  = 0x1008;
    &iVar1->field_0x6 = 0x0;
    *param_1          = 0xe228;
    iVar1->field_0x2  = 0x1018;
    puVar2            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x36, param_4, param_3, unaff_DI);
    iVar1->field_0x6  = puVar2;
    iVar1->field_0x8  = (puVar2 >> 0x10);
    return param_1;
}

void pass1_1018_c402(astruct_20 *param_1, u16 param_2, u16 param_3, u16 param_4, u32 param_5, u32 param_6, u32 param_7, u32 param_8, u16 param_9, u16 param_10)

{
    i16         iVar1;
    u16        *puVar2;
    astruct_20 *iVar4;
    i16         unaff_DI;
    astruct_20 *uVar4;
    u16        *puVar3;

    struct_1020_0762(param_1, CONCAT22(param_5, param_4), CONCAT22(param_6, (param_5 >> 0x10)), (param_6 >> 0x10), param_7, param_8, param_9);
    uVar4                 = (astruct_20 *)(param_1 >> 0x10);
    iVar4                 = (astruct_20 *)param_1;
    iVar4[0x1].field_0x14 = 0x0;
    iVar4[0x1].field_0x16 = 0x0;
    iVar4[0x1].field_0x18 = 0x0;
    iVar4[0x1].field_0x1a_addr_offset = 0x0;
    iVar4[0x1].field_0x1c_addr_base = 0x2;
    iVar4[0x1].field_0x26 = 0x0;
    iVar4[0x1].field_0x2a = 0x0;
    iVar4[0x1].field_0x2c = 0x1e0190;
    iVar4[0x1].field_0x30 = 0x0;
    param_1->field_0x0    = 0xc8bc;
    iVar4->field_0x2      = 0x1018;
    puVar2                = pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1e)), 0x0, 0x8);
    if((param_3 == 0x0) || (param_2 != 0x0))
    {
        if((param_2 & param_3) == 0x0)
            goto LAB_1018_c4bb;
        puVar2 = pass1_1008_5fd8(param_9, param_10);
    }
    else
    {
        load_string_1010_84ac(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    }
    *(u16 **)&iVar4[0x1].field_0x26 = puVar2;
    (&iVar4[0x1].field_0x26 + 0x2)  = param_10;
LAB_1018_c4bb:
    puVar3                = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_9, param_10, unaff_DI);
    iVar1                 = puVar3;
    iVar4[0x1].field_0x14 = (iVar1 + 0xa);
    iVar4[0x1].field_0x16 = (iVar1 + 0xc);
    pass1_1008_3e94((puVar3 & 0xffff0000 | (iVar1 + 0xe)), (param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1a_addr_offset)), (param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x18)));
    return;
}

Struct57 *pass1_1018_5e26(Struct57 *param_1, u16 param_2)

{
    u16 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfd0, param_2);
    uVar1            = (param_1 >> 0x10);
    param_1          = 0x6128;
    (param_1 + 0x2)  = 0x1018;
    (param_1 + 0x74) = 0x1;
    return param_1;
}

void pass1_1018_6198(u16 *param_1, u32 param_2, u16 param_3, u8 *param_4, i16 param_5, u16 param_6)

{
    astruct_657 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_657 *)param_1;
    *param_1          = 0x389a;
    iVar1->field_0x2  = 0x1008;
    *param_1          = 0x3aa8;
    iVar1->field_0x2  = 0x1008;
    iVar1->field_0x4  = param_3;
    *param_1          = 0x3ab0;
    iVar1->field_0x2  = 0x1008;
    &iVar1->field_0x6 = 0x0;
    iVar1->field_0xa  = param_2;
    *param_1          = 0x66c0;
    iVar1->field_0x2  = 0x1018;
    puVar2            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, param_6, param_4, param_5);
    iVar1->field_0x6  = puVar2;
    iVar1->field_0x8  = (puVar2 >> 0x10);
    return;
}

u32 pass1_1018_659a(u16 param_1, u16 param_2, u16 *param_3, u8 *param_4, u16 param_5)

{
    i16 *piVar1;
    i16  iStack18;
    i16  local_6;
    i16  local_4;

    piVar1 = &local_6;
    pass1_1008_3e94(param_3, CONCAT22(param_5, piVar1), CONCAT22(param_5, &local_4));
    mem_op_1000_179c(0xc, param_4, 0x1000);
    for(iStack18 = 0x0; iStack18 < 0x3; iStack18 = iStack18 + 0x1)
    {
        piVar1[iStack18 * 0x2]       = (iStack18 * 0x4 + 0x4248) + local_4;
        piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x424a) + local_6;
    }
    return CONCAT22(param_4, piVar1);
}

u16 *get_sys_metrics_1018_4b1e(astruct_55 *param_1, u16 param_2, u16 param_3)

{
    i16 iVar1;
    u16 uVar2;

    struct_op_1010_1d48((astruct_79 *)param_1, param_3);
    uVar2              = (param_1 >> 0x10);
    iVar1              = param_1;
    (iVar1 + 0x12)     = param_2;
    (iVar1 + 0x14)     = 0x0;
    param_1->field_0x0 = &PTR_LOOP_1050_4c9e;
    (iVar1 + 0x2)      = 0x1018;
    if(PTR_LOOP_1050_416c == 0x0)
    {
        globals->PTR_LOOP_1050_416c = GetSystemMetrics16(0x1010);
        globals->PTR_LOOP_1050_416e = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
        globals->PTR_LOOP_1050_4170 = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    }
    return &param_1->field_0x0;
}

void pass1_1018_4b78(u32 *param_1, u16 param_2)

{
    code      **ppcVar1;
    u8         *puVar2;
    u16         uVar3;
    u16        *puVar4;
    u32 *puVar5;

    puVar2 = param_1._2_2_;
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | ZEXT24((param_1 + 0xa))), 0x0, 0x8);
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (param_1 + 0x18)), 0x0, 0x8);
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, puVar2, param_1._2_2_);
    puVar5 = pass1_1010_5f7a(puVar4, (puVar4 >> 0x10), 0x0, (param_1 + 0x12));
    uVar3  = (puVar5 >> 0x10);
    if((uVar3 | puVar5) != 0x0)
    {
        (param_1 + 0xa) = *puVar5;
        (param_1 + 0xe) = (puVar5 + 0x4);
    }
    ppcVar1 = (*param_1 + 0x20);
    (**ppcVar1)(0x1010, param_1);
    if(((param_1 + 0xe) == 0x0) && ((param_1 + 0x10) == 0x0))
    {
        (param_1 + 0xa) = (param_1 + 0x18);
        (param_1 + 0xc) = (param_1 + 0x1a);
    }
    (param_1 + 0xe)  = (param_1 + 0x1c);
    (param_1 + 0x10) = (param_1 + 0x1e);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_4c2c(u32 param_1, u32 *param_2, u16 param_3, u16 param_4)

{
    u16 *puVar1;

    *(param_1 + 0xa) = *param_2;
    *(param_1 + 0xe) = param_2[0x1];
    puVar1           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, param_1._2_2_, param_1._2_2_);
    pass1_1010_5fb0(puVar1, 0x0, (param_1 + 0xa), param_1._2_2_, (param_1 + 0x12));
    return;
}
void pass1_1018_4dce(u32 *param_1, u16 param_2, u8 *param_3, u16 param_4)

{
    code **ppcVar1;
    u16    uVar2;
    i16    unaff_DI;
    u16   *puVar3;

    puVar3  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, param_3, unaff_DI);
    uVar2   = (puVar3 >> 0x10);
    ppcVar1 = (*param_1 + 0x10);
    (**ppcVar1)(0x1010, param_1, param_2, (puVar3 + 0xc), (puVar3 + 0xa));
    return;
}

void pass1_1018_5292(u32 param_1, u32 param_2, u16 param_3)

{
    i16         iVar1;
    u32  uVar2;
    code      **ppcVar3;
    u16         uVar4;
    BOOL16      BVar5;
    u8         *puVar6;
    i16         iVar7;
    char       *pcVar8;
    u16         uVar9;
    u32 *puVar10;
    u32 *puVar11;
    u32         uVar12;
    u8         *extraout_DX;
    u8         *extraout_DX_00;
    u16         uVar13;
    u8         *extraout_DX_01;
    u8         *puVar14;
    u16         extraout_DX_02;
    u16         extraout_DX_03;
    u8         *puVar15;
    u8         *extraout_DX_04;
    u16         uVar16;
    u16         extraout_DX_05;
    u16         extraout_DX_06;
    u8         *extraout_DX_07;
    u8         *extraout_DX_08;
    i16         iVar17;
    u16         uVar18;
    u16         uVar19;
    u16        *puVar20;
    u16         uStack50;
    u8          local_26[0x8];
    u32         uStack30;
    u32         uStack26;
    u32         uStack22;
    u32 *puStack18;
    u8         *puStack16;
    u32 *puStack14;
    u8         *puStack12;
    u16         uStack10;
    u32         uStack8;
    u16         uStack4;

    uVar18    = (param_1 >> 0x10);
    iVar17    = param_1;
    puStack18 = (iVar17 + 0xe);
    uVar12    = ZEXT24(puStack18);
    puVar14   = (iVar17 + 0x10);
    puStack16 = puVar14;
    puStack14 = puStack18;
    puStack12 = puVar14;
    if((puVar14 | puStack18) != 0x0)
    {
        ppcVar3 = *puStack18;
        (**ppcVar3)();
        puVar14 = extraout_DX;
    }
    mem_op_1000_179c(0xc, puVar14, 0x1000);
    puStack18 = uVar12;
    puStack16 = puVar14;
    if((puVar14 | puStack18) == 0x0)
    {
        uVar12  = 0x0;
        puVar14 = 0x0;
    }
    else
    {
        set_struct_1008_574a((uVar12 & 0xffff | ZEXT24(puVar14) << 0x10));
        puVar14 = extraout_DX_00;
    }
    (iVar17 + 0xe)  = uVar12;
    (iVar17 + 0x10) = puVar14;
    for(uStack4 = 0x21; - 0x1 < uStack4; uStack4 = uStack4 - 0x1)
    {
        uStack22 = pass1_1030_7c28(param_2, uStack4, uVar12, puVar14, param_3);
        uVar12   = uStack22 & 0xffff;
        uVar13   = uVar12;
        puVar14  = ((uStack22 >> 0x10) | uVar13);
        if(puVar14 != 0x0)
        {
            string_1020_c0ca(uStack4);
            uVar4    = str_op_1008_60e8(CONCAT22(puVar14, uVar13), puVar14);
            uVar12   = uVar4;
            uStack26 = CONCAT22(puVar14, uVar4);
            mem_op_1000_179c(0x10, puVar14, 0x1000);
            puStack14 = uVar12;
            puStack12 = puVar14;
            if((puVar14 | puStack14) == 0x0)
            {
                uVar12 = 0x0;
                uVar13 = 0x0;
            }
            else
            {
                struct_1018_4790(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10, uStack22, uStack26, uStack4);
                uVar13 = extraout_DX_02;
            }
            uStack30 = uVar12 & 0xffff | uVar13 << 0x10;
            uVar2    = (iVar17 + 0xe);
            ppcVar3  = ((iVar17 + 0xe) + 0x4);
            (**ppcVar3)(0x1000, uVar2, (uVar2 >> 0x10), uVar12, uVar13);
            puVar14 = extraout_DX_01;
        }
    }
    uStack8  = struct_op_1030_73a8(param_2);
    uStack10 = (uStack8 + 0xc);
    BVar5    = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack10, 0x4);
    if(BVar5 != 0x0)
    {
        uStack30 = uStack8;
        uStack26 = *(uStack8 + 0x20);
        pass1_1008_5784(CONCAT22(param_3, local_26), uStack26);
        while(true)
        {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            uStack22 = CONCAT22(extraout_DX_03, puVar6);
            puVar14  = (extraout_DX_03 | puVar6);
            if(puVar14 == 0x0)
                break;
            iVar1 = (puVar6 + 0x6);
            iVar7 = iVar1 + -0x7;
            if(iVar7 == 0x0)
            {
            LAB_1018_53f0:
                pcVar8  = string_op_1020_c222((puVar6 + 0x6));
                uVar9   = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                puVar15 = puVar14;
                uVar4   = uVar9;
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = uVar4;
                puStack16 = puVar15;
                if((puVar15 | uVar4) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar19  = (uStack22 >> 0x10);
                    puVar20 = struct_1018_48b0(CONCAT22(puVar15, uVar4), (uStack22 + 0xa), CONCAT22(puVar14, uVar9), (uStack22 + 0x6));
                    uVar16  = (puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uVar2   = (iVar17 + 0xe);
                ppcVar3 = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, uVar2, (uVar2 >> 0x10), uVar19, uVar16);
                puVar14 = extraout_DX_04;
            }
            else
            {
                if(((0x5 < iVar7) && (!SBORROW2(iVar7, 0x6))) && (iVar1 + -0xd < 0x2))
                    goto LAB_1018_53f0;
            }
            uVar19 = (uStack22 >> 0x10);
            if((uStack22 + 0x8) != 0x0)
            {
                pcVar8  = string_op_1020_c2f8((uStack22 + 0x8));
                puVar10 = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                puVar15 = puVar14;
                puVar11 = puVar10;
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack14 = puVar11;
                puStack12 = puVar15;
                if((puVar15 | puVar11) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar19  = (uStack22 >> 0x10);
                    puVar20 = struct_1018_48e8(CONCAT22(puVar15, puVar11), (uStack22 + 0xa), CONCAT22(puVar14, puVar10), (uStack22 + 0x8));
                    uVar16  = (puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uVar2   = (iVar17 + 0xe);
                ppcVar3 = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, uVar2, (uVar2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    uVar19   = (param_2 >> 0x10);
    uVar12   = *(param_2 + 0x3e);
    uVar13   = (param_2 + 0x40);
    uStack50 = uVar12;
    if((uVar13 | uStack50) != 0x0)
    {
        pass1_1008_5784(CONCAT22(param_3, local_26), uVar12 & 0xffff | uVar13 << 0x10);
        while(true)
        {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            puVar14 = (extraout_DX_05 | puVar6);
            if(puVar14 == 0x0)
                break;
            if((puVar6 + 0x4) != 0x0)
            {
                pcVar8   = string_1020_c0d8((puVar6 + 0x4));
                uVar13   = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                uStack30 = CONCAT22(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = uVar13;
                puStack16 = puVar14;
                if((puVar14 | uVar13) == 0x0)
                {
                    uVar13 = 0x0;
                    uVar19 = 0x0;
                }
                else
                {
                    struct_1018_4790(CONCAT22(puVar14, uVar13), (puVar6 + 0xa), uStack30, (puVar6 + 0x4));
                    uVar19 = extraout_DX_06;
                }
                uStack26 = CONCAT22(uVar19, uVar13);
                uVar2    = (iVar17 + 0xe);
                ppcVar3  = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, uVar2, (uVar2 >> 0x10), uVar13, uVar19);
                puVar14 = extraout_DX_07;
            }
            if((puVar6 + 0x6) != 0x0)
            {
                pcVar8   = string_op_1020_c222((puVar6 + 0x6));
                puVar11  = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                uStack30 = CONCAT22(puVar14, puVar11);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack14 = puVar11;
                puStack12 = puVar14;
                if((puVar14 | puVar11) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    puVar20 = struct_1018_48b0(CONCAT22(puVar14, puVar11), (puVar6 + 0xa), uStack30, (puVar6 + 0x6));
                    uVar16  = (puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uStack26 = CONCAT22(uVar16, uVar19);
                uVar2    = (iVar17 + 0xe);
                ppcVar3  = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, uVar2, (uVar2 >> 0x10), uVar19, uVar16);
                puVar14 = extraout_DX_08;
            }
            if((puVar6 + 0x8) != 0x0)
            {
                pcVar8   = string_op_1020_c2f8((puVar6 + 0x8));
                uVar13   = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                uStack30 = CONCAT22(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = uVar13;
                puStack16 = puVar14;
                if((puVar14 | uVar13) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    puVar20 = struct_1018_48e8(CONCAT22(puVar14, uVar13), (puVar6 + 0xa), uStack30, (puVar6 + 0x8));
                    uVar16  = (puVar20 >> 0x10);
                    uVar19  = SUB42(puVar20, 0x0);
                }
                uStack26 = CONCAT22(uVar16, uVar19);
                uVar2    = (iVar17 + 0xe);
                ppcVar3  = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(0x1000, uVar2, (uVar2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    return;
}

void switch_1018_3ee6(u32 param_1, long param_2, i16 param_3, u16 param_4, u8 *param_5)

{
    i16   iVar1;
    char *pcVar2;
    u16   uVar3;
    u16   uVar4;
    u16   uVar5;
    u16   uVar6;
    u32   uVar7;
    u8   *puVar8;
    u16   unaff_SS;
    u16  *puVar9;
    long  lVar10;
    i16   iVar11;
    u16 IVar12;
    u16   uVar13;
    u32   uStack26;
    u16  *puStack22;
    long  lStack18;
    long  lStack14;
    i16   iStack10;
    u16   uStack8;
    i16  *piStack6;

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
    piStack6 = CONCAT22(0x1050, iVar1);
    if(piStack6 == 0x0)
    {
        return;
    }
    iStack10 = 0x0;
    uStack8  = 0x0;
    iVar11   = *piStack6;
    uVar13   = param_1;
    uVar3    = (param_1 >> 0x10);
    if(iVar11 == 0x1)
    {
        uVar13   = pass1_1018_456a(uVar13, uVar3, (iVar1 + 0x2));
        lStack14 = CONCAT22(param_5, uVar13);
        pcVar2   = string_1020_c0d8((iVar1 + 0x2));
        uVar3    = str_op_1008_60e8(CONCAT22(param_5, pcVar2), param_5);
        puVar8   = param_5;
        uVar13   = uVar3;
        mem_op_1000_179c(0x10, param_5, 0x1000);
        puStack22 = CONCAT22(puVar8, uVar13);
        if((puVar8 | uVar13) != 0x0)
        {
            lVar10  = param_2 / lStack14;
            uStack8 = (param_2 % lStack14);
            struct_1018_4790(puStack22, lVar10, CONCAT22(param_5, uVar3), (iVar1 + 0x2));
            iStack10 = lVar10;
            goto LAB_1018_425e;
        }
    }
    else
    {
        if(iVar11 == 0x2)
        {
            uVar13   = pass1_1018_451e(uVar13, uVar3, (iVar1 + 0x2));
            lStack18 = CONCAT22(param_5, uVar13);
            pcVar2   = string_op_1020_c222((iVar1 + 0x2));
            uVar3    = str_op_1008_60e8(CONCAT22(param_5, pcVar2), param_5);
            puVar8   = param_5;
            uVar13   = uVar3;
            mem_op_1000_179c(0x10, param_5, 0x1000);
            puStack22 = CONCAT22(puVar8, uVar13);
            if((puVar8 | uVar13) != 0x0)
            {
                puVar9   = struct_1018_48b0(puStack22, param_2 / lStack18, CONCAT22(param_5, uVar3), (iVar1 + 0x2));
                uStack8  = (puVar9 >> 0x10);
                iStack10 = puVar9;
                goto LAB_1018_425e;
            }
        }
        else
        {
            if(iVar11 == 0x3)
            {
                uVar4 = pass1_1008_c646(_PTR_LOOP_1050_06e0, CONCAT22((iVar1 + 0x2), (_PTR_LOOP_1050_06e0 >> 0x10)), unaff_SS);
                if(uVar4 == 0x0)
                {
                    uVar4 = 0x4f;
                }
                uVar13   = switch_1018_43ec(uVar13, uVar3, uVar4);
                lStack14 = CONCAT22(param_5, uVar13);
                uVar13   = pass1_1020_bd80(uVar4);
                uVar5    = str_op_1008_60e8(CONCAT22(param_5, uVar13), param_5);
                uStack26 = CONCAT22(param_5, uVar5);
                mem_op_1000_179c(0x14, param_5, 0x1000);
                puStack22 = CONCAT22(param_5, uVar5);
                if((param_5 | uVar5) != 0x0)
                {
                    uVar7   = param_2 / lStack14;
                    uStack8 = (param_2 % lStack14);
                    struct_1018_47c8(puStack22, uVar7, uStack26, uVar4, 0x0);
                    iStack10 = uVar7;
                    goto LAB_1018_425e;
                }
            }
            else
            {
                if(iVar11 != 0x4)
                    goto LAB_1018_425e;
                iVar1  = (iVar1 + 0x2);
                uVar5  = iVar1 - 0x1;
                iVar11 = globals->_PTR_LOOP_1050_14cc;
                IVar12 = (u16)(_PTR_LOOP_1050_14cc >> 0x10);
                if(uVar5 == 0x0)
                {
                    load_string_1010_84ac(iVar11, IVar12, 0x1010);
                    uVar6  = uVar5;
                    puVar8 = param_5;
                    mem_op_1000_179c(0x14, param_5, 0x1000);
                    puStack22 = CONCAT22(puVar8, uVar6);
                    if((puVar8 | uVar6) != 0x0)
                    {
                        uVar13 = 0x2;
                        lVar10 = 0x14;
                    LAB_1018_4230:
                        puVar9   = struct_1018_4842(puStack22, param_2 / lVar10, CONCAT22(param_5, uVar5), uVar13);
                        uStack8  = (puVar9 >> 0x10);
                        iStack10 = puVar9;
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
                        puStack22 = CONCAT22(puVar8, uVar6);
                        if((puVar8 | uVar6) != 0x0)
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
                            puStack22 = CONCAT22(puVar8, uVar6);
                            if((puVar8 | uVar6) != 0x0)
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
                            puStack22 = CONCAT22(puVar8, uVar6);
                            if((puVar8 | uVar6) != 0x0)
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
    if((iStack10 + 0x8) == 0x0)
    {
        (iStack10 + 0x8) = 0x1;
    }
    return;
}

void get_sys_metrics_1018_2f56(u32 param_1)

{
    u16   uVar1;
    u16 IVar2;
    u16 IVar3;
    u8   *in_DX;
    i16   iVar4;
    i16   unaff_DI;
    u16   uVar5;
    u16   unaff_SS;
    u16  *puVar6;
    u32   uVar7;
    u16  *puVar8;
    u16  *puVar9;
    i16   local_6;
    i16   local_4;

    puVar9 = CONCAT22(unaff_SS, &local_4);
    puVar8 = CONCAT22(unaff_SS, &local_6);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)), puVar8, puVar9);
    uVar5          = (param_1 >> 0x10);
    iVar4          = param_1;
    uVar7          = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x24));
    uVar1          = (uVar7 >> 0x10);
    (iVar4 + 0x18) = local_4 + 0xb5;
    (iVar4 + 0x1a) = local_6 + 0x9;
    IVar2          = GetSystemMetrics16(0x1008);
    (iVar4 + 0x1c) = IVar2 * 0x2 + (uVar7 + 0x4);
    IVar2          = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    IVar3          = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    (iVar4 + 0x1e) = IVar3 + IVar2 + (uVar7 + 0x8);
    return;
}

void pass1_1018_30fc(u32 param_1, u16 **param_2, u8 *param_3)

{
    u16         uVar1;
    u32  uVar2;
    u16        *puVar3;
    u16         uVar4;
    u16         uVar5;
    long        lVar6;
    u8         *puVar7;
    u16         extraout_DX;
    u16         uVar8;
    u32 *puStack18;
    i16         iStack6;

    *param_2 = 0x0;
    uVar8    = (param_1 >> 0x10);
    uVar2    = (param_1 + 0x17e);
    uVar1    = (uVar2 + 0xa);
    if(uVar1 != 0x0)
    {
        uVar4 = uVar1;
        mem_op_1000_179c(0x6, param_3, 0x1000);
        puStack18 = CONCAT22(param_3, uVar4);
        puVar7    = (param_3 | uVar4);
        if(puVar7 == 0x0)
        {
            *param_2 = 0x0;
        }
        else
        {
            *puStack18    = 0x0;
            (uVar4 + 0x4) = 0x0;
            *param_2      = puStack18;
        }
        uVar5 = uVar1 * 0x2;
        mem_op_1000_179c(uVar5, puVar7, 0x1000);
        puVar3           = *param_2;
        *puVar3          = uVar5;
        (puVar3 + 0x2)   = puVar7;
        (*param_2 + 0x4) = uVar1;
        for(iStack6 = 0x0; iStack6 < uVar1; iStack6 = iStack6 + 0x1)
        {
            lVar6 = (long)iStack6;
            empty_1008_8fc4((param_1 + 0x17e), lVar6);
            (*param_2 + iStack6 * 0x2) = (lVar6 + 0x2e);
        }
    }
    return;
}

void pass1_1018_3710(u32 *param_1, u16 param_2, u16 param_3, u16 param_4)

{
    u32 uVar1;
    code     **ppcVar2;
    i16        iVar3;
    u16        uVar4;
    u8        *puVar5;
    i16        iVar6;
    u16        uVar7;
    u8         in_AF;
    long       lVar8;
    u16       *puVar9;
    u8         local_12a[0x118];
    u32 uStack18;
    u16       *puStack14;
    u32        uStack10;
    u16       *puStack6;

    puStack6 = 0x0;
    uVar7    = (param_1 >> 0x10);
    iVar6    = param_1;
    uStack10 = switch_1018_3b9e(param_1, (iVar6 + 0x12e), param_3, param_4, param_2);
    uVar4    = (iVar6 + 0x12e) - 0x188;
    uStack18 = (uStack10 & 0xffff0000 | uVar4);
    switch(uVar4)
    {
    case 0x0:
        lVar8  = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (lVar8 >> 0x10);
        iVar3  = lVar8;
        mem_op_1000_179c(0x10, puVar5, 0x1000);
        if(lVar8 != 0x0)
        {
            uStack18 = struct_1018_4790(lVar8, (iVar6 + 0x132), 0x0, (iVar3 + 0xe));
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x1:
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
        iVar3  = puVar9;
        mem_op_1000_179c(0x14, puVar5, 0x1000);
        uVar4 = (puVar9 >> 0x10) | puVar9;
        if(puVar9 != 0x0)
        {
            struct_1018_47c8(puVar9, *(iVar6 + 0x132), 0x0, (iVar3 + 0x12), *(iVar3 + 0xe));
            uStack18 = (puVar9 & 0xffff | uVar4 << 0x10);
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x2:
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
        iVar3  = puVar9;
        mem_op_1000_179c(0x12, puVar5, 0x1000);
        uVar4 = (puVar9 >> 0x10) | puVar9;
        if(puVar9 != 0x0)
        {
            pass1_1018_4808(puVar9, *(iVar6 + 0x132), 0x0, *(iVar3 + 0xe));
            uStack18 = (puVar9 & 0xffff | uVar4 << 0x10);
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x3:
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
        iVar3  = puVar9;
        mem_op_1000_179c(0x14, puVar5, 0x1000);
        if(puVar9 != 0x0)
        {
            uStack18 = struct_1018_4842(puVar9, *(iVar6 + 0x132), 0x0, (iVar3 + 0xe));
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x4:
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
        iVar3  = puVar9;
        mem_op_1000_179c(0x10, puVar5, 0x1000);
        if(puVar9 != 0x0)
        {
            uStack18 = struct_1018_48b0(puVar9, *(iVar6 + 0x132), 0x0, (iVar3 + 0xe));
            puStack6 = uStack18;
            goto switchD_1018_393f_caseD_6;
        }
        break;
    case 0x5:
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
        iVar3  = puVar9;
        mem_op_1000_179c(0x12, puVar5, 0x1000);
        if(puVar9 != 0x0)
        {
            uStack18 = struct_1018_4920(puVar9, *(iVar6 + 0x132), 0x0, *(iVar3 + 0xe));
            puStack6 = uStack18;
        }
        break;
    default:
        goto switchD_1018_393f_caseD_6;
    }
    uStack18 = 0x0;
    puStack6 = uStack18;
switchD_1018_393f_caseD_6:
    uVar1 = (iVar6 + 0x122);
    pass1_1008_e852(uVar1, (uVar1 >> 0x10), *(iVar6 + 0x126), param_2, (uStack18 >> 0x10));
    uVar1     = (iVar6 + 0x122);
    puStack14 = uStack18;
    pass1_1008_e852(uVar1, (uVar1 >> 0x10), *(iVar6 + 0x12a), param_2, (uStack18 >> 0x10));
    pass1_1038_2a0e((astruct_100 *)CONCAT22(param_2, local_12a), *(iVar6 + 0x136), puStack6, uStack18, puStack14, param_2, in_AF);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_2, local_12a));
    (iVar6 + 0x136) = 0x0;
    ppcVar2         = (*param_1 + 0x10);
    (**ppcVar2)(0x1030, param_1);
    pass1_1038_2a5c(CONCAT22(param_2, local_12a));
    return;
}

void get_sys_metrics_1018_1ea0(astruct_55 *param_1, u16 param_2)

{
    u16       IVar1;
    u16       IVar2;
    astruct_55 *iVar3;
    u16         uVar3;

    IVar1             = GetSystemMetrics16(param_2);
    uVar3             = (param_1 >> 0x10);
    iVar3             = (astruct_55 *)param_1;
    iVar3->field_0x2e = IVar1 * 0x2 + iVar3->field_0x36;
    IVar1             = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    IVar2             = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    iVar3->field_0x30 = IVar1 + iVar3->field_0x38 + IVar2;
    return;
}

u32 pass1_1018_1ff4(astruct_634 *param_1, u16 param_2, u16 param_3)

{
    i16        *piVar1;
    i16         unaff_DI;
    u16         unaff_SS;
    astruct_79 *paVar2;
    i16        *piVar3;
    i16        *piVar4;
    u16         uVar5;
    i16         local_a;
    i16         local_8;
    u32  uStack6;

    paVar2                     = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    &param_1->field_0xa        = 0xb9010b;
    param_1->field_0xe         = 0x170035;
    CONCAT22(param_2, param_1) = 0x21e8;
    param_1->field_0x2         = 0x1018;
    piVar4                     = &local_8;
    piVar3                     = &local_a;
    uVar5                      = unaff_SS;
    uStack6                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, (paVar2 >> 0x10), unaff_DI);
    pass1_1008_3e94((uStack6 & 0xffff0000 | (uStack6 + 0xe)), CONCAT22(unaff_SS, piVar3), CONCAT22(uVar5, piVar4));
    piVar1  = &param_1->field_0xa;
    *piVar1 = *piVar1 + local_8;
    piVar1  = &param_1->field_0xc;
    *piVar1 = *piVar1 + local_a;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, param_1 + 0x1), 0x0, 0x7f4);
    return CONCAT22(param_2, param_1);
}

void pass1_1018_270e(u32 param_1, i16 param_2, u16 param_3, u8 *param_4, i16 param_5, u16 param_6)

{
    code       **ppcVar1;
    u32          uVar2;
    i16          iVar3;
    u16          uVar4;
    u8          *extraout_DX;
    astruct_655 *iVar5;
    u16          uVar5;
    u16         *puVar6;

    iVar5 = (astruct_655 *)param_1;
    uVar5 = (param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        if((iVar5->field_0x20 == 0x0) || (uVar2 = iVar5->field_0x20, (uVar2 + 0x8) != param_3))
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
                    iVar3 = &iVar5->field_0x1c_addr_base;
                    uVar4 = uVar5;
                }
                pass1_1010_1ea6(iVar5->field_0x20, CONCAT22(uVar4, iVar3), param_6);
            }
            iVar5->field_0x20 = puVar6;
            if(param_1 == 0x0)
            {
                param_3 = 0x0;
                uVar4   = 0x0;
            }
            else
            {
                param_3 = &iVar5->field_0x1c_addr_base;
                uVar4   = uVar5;
            }
            uVar2   = iVar5->field_0x20;
            ppcVar1 = (iVar5->field_0x20 + 0x4);
            (**ppcVar1)(0x1010, uVar2, (uVar2 >> 0x10), 0x0, param_3, uVar4);
            param_4 = extraout_DX;
        }
        pass1_1018_2862(param_1);
        if((param_4 | param_3) != 0x0)
        {
            iVar5->field_0x24 = 0x1;
        }
        pass1_1010_1f62(param_6, param_1, 0x7);
    }
    else
    {
        if(((&iVar5->field_0x20 + 0x2) | &iVar5->field_0x20) != 0x0)
        {
            if(param_1 == 0x0)
            {
                iVar3 = 0x0;
                uVar4 = 0x0;
            }
            else
            {
                iVar3 = &iVar5->field_0x1c_addr_base;
                uVar4 = uVar5;
            }
            pass1_1010_1ea6(iVar5->field_0x20, CONCAT22(uVar4, iVar3), param_6);
            iVar5->field_0x20 = 0x0;
            return;
        }
    }
    return;
}

void mixed_sys_op_1018_2978(u32 param_1, u16 param_2, u16 param_3)

{
    code      **ppcVar1;
    u8         *puVar2;
    u8         *puVar3;
    RECT16     *rect;
    i16         iVar4;
    u8         *in_DX;
    u16         uVar5;
    u8         *extraout_DX;
    u8         *puVar6;
    u8         *puVar7;
    i16         iVar8;
    u16         uVar9;
    u16         uVar10;
    u16         uVar11;
    u8          uVar12;
    astruct_76 *paStack62;
    RECT16      local_3a;
    i16         iStack54;
    i16         iStack52;
    u32         uStack50;
    u32 *puStack46;
    u8          local_2a[0x24];
    u16         uStack6;

    pass1_1010_8096(_PTR_LOOP_1050_14cc, 0x1);
    puVar2  = local_2a;
    uStack6 = param_2;
    struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3, puVar2), 0x1, CONCAT22(in_DX, param_2), in_DX);
    uVar9 = 0x1000;
    mem_op_1000_179c(0x1e, in_DX, 0x1000);
    uVar5 = in_DX | puVar2;
    if(uVar5 == 0x0)
    {
        puVar3 = 0x0;
        uVar5  = 0x0;
    }
    else
    {
        puVar3 = local_2a;
        uVar9  = 0x1008;
        struct_op_1008_3f92((astruct_76 *)CONCAT22(in_DX, puVar2), (astruct_83 *)CONCAT22(param_3, puVar3));
    }
    puStack46 = CONCAT22(uVar5, puVar3);
    ppcVar1   = (*puStack46 + 0x14);
    (**ppcVar1)(uVar9, puVar3, uVar5);
    uStack50 = CONCAT22(extraout_DX, puVar3);
    puVar6   = extraout_DX;
    mem_op_1000_179c(0x14, extraout_DX, 0x1000);
    puVar7 = (puVar6 | puVar3);
    if(puVar7 == 0x0)
    {
        puVar3 = 0x0;
        puVar7 = 0x0;
    }
    else
    {
        struct_1008_4c58(CONCAT13((puVar6 >> 0x8), CONCAT12(puVar6, puVar3)));
    }
    uVar9          = (param_1 >> 0x10);
    iVar8          = param_1;
    (iVar8 + 0xe)  = puVar3;
    (iVar8 + 0x10) = puVar7;
    pass1_1008_4d84((iVar8 + 0xe), uStack50, puVar7);
    uVar12 = SUB21(PTR_LOOP_1050_0396, 0x0);
    rect   = &local_3a;
    GetClientRect16(0x1008, rect);
    uVar11 = 0x1e;
    uVar10 = 0x1000;
    mem_op_1000_179c(0x1e, puVar7, 0x1000);
    paStack62 = (astruct_76 *)CONCAT22(puVar7, rect);
    uVar5     = puVar7 | rect;
    if(uVar5 == 0x0)
    {
        (iVar8 + 0xa) = 0x0;
    }
    else
    {
        iVar4  = (iStack52 - local_3a.y) + 0x1;
        uVar10 = 0x1008;
        pass1_1008_405c(paStack62, *(iVar8 + 0xe), iVar4, (iStack54 - local_3a.x) + 0x1);
        (iVar8 + 0xa) = iVar4;
        (iVar8 + 0xc) = uVar5;
    }
    if(puStack46 != 0x0)
    {
        ppcVar1 = *puStack46;
        (**ppcVar1)(uVar10, puStack46, (puStack46 >> 0x10), 0x1, uVar11, uVar12);
    }
    close_file_1008_496c(local_2a, param_3);
    return;
}

void pass1_1018_10c4(u16 param_1, u16 param_2, u32 param_3)

{
    u32         uVar1;
    code      **ppcVar2;
    u32  uVar3;
    i16         iVar4;
    u8         *puVar5;
    u16         uVar6;
    u16         uVar7;
    u32         uVar8;
    u16         uVar9;
    u8         *puVar10;
    u16         extraout_DX;
    u16         extraout_DX_00;
    u16         extraout_DX_01;
    u16         extraout_DX_02;
    i16         iVar11;
    u16         uVar12;
    u8          uVar13;
    bool        bVar14;
    u32        *puVar15;
    u32         uStack60;
    u32         uStack56;
    u32  uStack52;
    u32 *puStack48;
    u32 *puStack40;
    u16         uStack30;
    u16         uStack28;
    u8          local_16[0x8];
    u16         uStack14;
    u16         uStack12;
    u16         uStack10;
    u16         uStack8;
    i16         iStack6;
    i16         iStack4;

    uVar12  = (param_3 >> 0x10);
    iVar11  = param_3;
    iStack4 = (iVar11 + 0x86);
    fn_ptr_1000_17ce((iVar11 + 0x88), 0x1000);
    (iVar11 + 0x86) = 0x0;
    (iVar11 + 0x88) = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT13((param_1 >> 0x8), CONCAT12(param_1, local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0x0;
    uStack28 = 0x0;
    while(true)
    {
        uVar9  = param_2;
        puVar5 = local_16;
        pass1_1028_e4ec(CONCAT22(param_1, puVar5));
        param_2 = uVar9 | puVar5;
        if(param_2 == 0x0)
            break;
        if((iVar11 + 0x3c) == (puVar5 + 0x8))
        {
            puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
            puVar10 = (puVar15 >> 0x10);
            uVar6   = puVar15;
            pass1_1038_4e78(uVar6, puVar10, CONCAT22(uVar9, puVar5), puVar15);
            puStack48 = CONCAT22(puVar10, uVar6);
            uVar3     = *puStack48;
            ppcVar2   = uVar3 + 0x8;
            uVar9     = uVar6;
            (**ppcVar2)(&PTR_LOOP_1050_1038, uVar6, puVar10);
            bVar14   = CARRY2(uStack30, uVar9);
            uStack30 = uStack30 + uVar9;
            uStack28 = uStack28 + extraout_DX + bVar14;
            param_2  = extraout_DX;
            if(puStack48 != 0x0)
            {
                ppcVar2 = uVar3;
                (**ppcVar2)(0x38, uVar6, puVar10, 0x1);
                param_2 = extraout_DX_00;
            }
        }
    }
    if((uStack28 | uStack30) != 0x0)
    {
        (iVar11 + 0x86) = uStack30;
        uVar7           = uStack30 * 0x6;
        mem_op_1000_179c(uVar7, 0x0, 0x1000);
        uStack52 = CONCAT22(param_2, uVar7);
        if((param_2 | uVar7) == 0x0)
        {
            (iVar11 + 0x88) = 0x0;
        }
        else
        {
            pass1_1000_5586(0x3e38, 0x1008, uStack30, 0x6, uVar7, param_2);
            (iVar11 + 0x88) = uStack52;
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
            if((uVar9 | puVar5) == 0x0)
                break;
            uStack8 = uVar9 | puVar5;
            if((iVar11 + 0x3c) == (puVar5 + 0x8))
            {
                puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
                puVar10 = (puVar15 >> 0x10);
                uVar6   = puVar15;
                uVar13  = 0x38;
                pass1_1038_4e78(uVar6, puVar10, CONCAT13((uVar9 >> 0x8), CONCAT12(uVar9, puVar5)), puVar15);
                puStack40 = CONCAT22(puVar10, uVar6);
                ppcVar2   = (*puStack40 + 0x10);
                uVar9     = uVar6;
                (**ppcVar2)(0x38, uVar6, puVar10);
                uStack56 = CONCAT22(extraout_DX_01, uVar9);
                uStack8  = extraout_DX_01;
                for(uStack60 = 0x0; uStack60 < uStack56; uStack60 = uStack60 + 0x1)
                {
                    uVar8 = uStack56;
                    pass1_1030_1d58(puStack40);
                    uVar1  = *(iVar11 + 0x88);
                    uVar13 = 0x8;
                    pass1_1008_3f62((uVar1 & 0xff000000 | CONCAT12((uVar1 >> 0x10), uVar1 + iVar4 * 0x6)), CONCAT22(uStack8, uVar8 + 0xc));
                    iVar4 = iVar4 + 0x1;
                }
                if(puStack40 != 0x0)
                {
                    ppcVar2 = *puStack40;
                    (**ppcVar2)(uVar13, uVar6, puVar10, 0x1);
                    uStack8 = extraout_DX_02;
                }
            }
        }
        if((iVar11 + 0x86) != iStack4)
        {
            pass1_1010_1f62(param_1, param_3, 0x6);
        }
    }
    return;
}

void pass1_1018_1320(u32 param_1, u16 *param_2, u32 *param_3)

{
    u16 uVar1;

    uVar1    = (param_1 >> 0x10);
    *param_3 = *(param_1 + 0x88);
    *param_2 = (param_1 + 0x86);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_1346(u16 param_1, u16 param_2, astruct_93 *param_3)

{
    code      **ppcVar1;
    i16         iVar2;
    u8         *puVar3;
    u16         uVar4;
    u16         uVar5;
    u8         *puVar6;
    u16         extraout_DX;
    u16         extraout_DX_00;
    u16         extraout_DX_01;
    u16         uVar7;
    u16         extraout_DX_02;
    astruct_93 *iVar9;
    u16         uVar8;
    u8          uVar9;
    u32        *puVar10;
    u32         uVar11;
    u32         uVar12;
    u32         uStack70;
    u32 *puStack56;
    u32         uStack52;
    u32 *puStack48;
    u32  uStack30;
    u8          local_16[0x8];
    u16         uStack14;
    u16         uStack12;
    u16         uStack10;
    u16         uStack8;
    i16         iStack6;
    u16         uStack4;

    uVar8   = (param_3 >> 0x10);
    iVar9   = (astruct_93 *)param_3;
    uStack4 = iVar9->field_0x8c;
    fn_ptr_1000_17ce((Struct18 *)iVar9->field_0x8e, 0x1000);
    iVar9->field_0x8c = 0x0;
    iVar9->field_0x8e = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT13((param_1 >> 0x8), CONCAT12(param_1, local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0x0;
    while(true)
    {
        uVar7  = param_2;
        puVar3 = local_16;
        pass1_1028_e4ec(CONCAT22(param_1, puVar3));
        param_2 = uVar7 | puVar3;
        if(param_2 == 0x0)
            break;
        if(iVar9->field_0x3c == (puVar3 + 0x8))
        {
            puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
            puVar6  = (puVar10 >> 0x10);
            uVar4   = puVar10;
            uVar9   = 0x38;
            pass1_1038_4e78(uVar4, puVar6, CONCAT22(uVar7, puVar3), puVar10);
            puStack48 = CONCAT22(puVar6, uVar4);
            ppcVar1   = (*puStack48 + 0x10);
            uVar7     = uVar4;
            (**ppcVar1)(&PTR_LOOP_1050_1038, uVar4, puVar6);
            uStack52 = CONCAT22(extraout_DX, uVar7);
            param_2  = extraout_DX;
            for(puStack56 = 0x0; puStack56 < uStack52; puStack56 = ((long)puStack56 + 0x1))
            {
                uVar9   = 0x30;
                uVar11  = pass1_1030_1d7c(uVar7, param_2, puStack48);
                param_2 = (uVar11 >> 0x10);
                if((uVar11 + 0x12) == 0x9)
                {
                    uStack30 = uStack30 + 0x1;
                }
            }
            if(puStack48 != 0x0)
            {
                ppcVar1 = *puStack48;
                (**ppcVar1)(uVar9, uVar4, puVar6, 0x1);
                param_2 = extraout_DX_00;
            }
        }
    }
    if((uStack30._2_2_ | uStack30) != 0x0)
    {
        iVar9->field_0x8c = uStack30;
        uVar5             = uStack30 * 0x6;
        mem_op_1000_179c(uVar5, 0x0, 0x1000);
        uStack70 = CONCAT22(param_2, uVar5);
        if((param_2 | uVar5) == 0x0)
        {
            iVar9->field_0x8e = 0x0;
        }
        else
        {
            pass1_1000_5586(0x3e38, 0x1008, uStack30, 0x6, uVar5, param_2);
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
            if((uVar7 | puVar3) == 0x0)
                break;
            uStack8 = uVar7 | puVar3;
            if(iVar9->field_0x3c == (puVar3 + 0x8))
            {
                puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
                puVar6  = (puVar10 >> 0x10);
                uVar4   = puVar10;
                uVar9   = 0x38;
                pass1_1038_4e78(uVar4, puVar6, CONCAT13((uVar7 >> 0x8), CONCAT12(uVar7, puVar3)), puVar10);
                puStack56 = CONCAT22(puVar6, uVar4);
                ppcVar1   = (*puStack56 + 0x10);
                uVar7     = uVar4;
                (**ppcVar1)(0x38, uVar4, puVar6);
                uStack52 = CONCAT22(extraout_DX_01, uVar7);
                uStack8  = extraout_DX_01;
                for(puStack48 = 0x0; puStack48 < uStack52; puStack48 = ((long)puStack48 + 0x1))
                {
                    uVar11 = uStack52;
                    pass1_1030_1d58(puStack56);
                    uVar9  = 0x30;
                    uVar12 = struct_op_1030_73a8(uVar11 & 0xffff | uStack8 << 0x10);
                    uVar7  = (uVar12 >> 0x10);
                    if((uVar12 + 0x12) == 0x9)
                    {
                        uVar12 = iVar9->field_0x8e;
                        uVar9  = 0x8;
                        pass1_1008_3f62((uVar12 & 0xff000000 | CONCAT12((uVar12 >> 0x10), uVar12 + iVar2 * 0x6)), CONCAT22(uStack8, uVar11 + 0xc));
                        iVar2 = iVar2 + 0x1;
                    }
                    uStack8 = uVar7;
                }
                if(puStack56 != 0x0)
                {
                    ppcVar1 = *puStack56;
                    (**ppcVar1)(uVar9, uVar4, puVar6, 0x1);
                    uStack8 = extraout_DX_02;
                }
            }
        }
        if(iVar9->field_0x8c != uStack4)
        {
            pass1_1010_1f62(param_1, param_3, 0x6);
        }
    }
    return;
}

void pass1_1018_18b8(u16 param_1, astruct_55 *param_2, u16 param_3)

{
    u8         *puVar1;
    astruct_55 *iVar3;
    i16         unaff_DI;
    astruct_55 *uVar3;
    u16        *puVar2;
    astruct_43 *paVar3;
    u32         uVar4;
    i16        *piVar5;
    u16         uVar6;
    i16        *piVar7;
    u16         uVar8;
    i16         local_6;
    i16         local_4;
    u16         uVar1;

    get_sys_metrics_1018_4b1e(param_2, 0x0, param_3);
    uVar3                 = (astruct_55 *)(param_2 >> 0x10);
    iVar3                 = (astruct_55 *)param_2;
    iVar3->field_0x20     = 0x389a;
    iVar3->field_0x22     = 0x1008;
    iVar3->field_0x20     = 0x3aa8;
    iVar3->field_0x22     = 0x1008;
    &iVar3->field_0x24    = 0x0;
    iVar3->field_0x28     = 0x4;
    puVar2                = pass1_1008_3e38((param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
    puVar1                = (puVar2 >> 0x10);
    &iVar3[0x1].field_0x6 = 0x0;
    iVar3[0x1].field_0xa  = 0x0;
    &iVar3[0x1].field_0xc = 0x0;
    iVar3[0x1].field_0x10 = 0x0;
    iVar3[0x1].field_0x1c_addr_base = 0x0;
    param_2->field_0x0    = 0x1fb0;
    iVar3->field_0x2      = 0x1018;
    iVar3->field_0x20     = 0x1fec;
    iVar3->field_0x22     = 0x1018;
    pass1_1000_4906((astruct_20 *)(param_2 & 0xffff0000 | &iVar3[0x1].field_0x14), 0x0, 0x8);
    piVar7 = &local_4;
    piVar5 = &local_6;
    uVar6  = param_1;
    uVar8  = param_1;
    puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_1, puVar1, unaff_DI);
    pass1_1008_3e94((puVar2 & 0xffff0000 | (puVar2 + 0xe)), CONCAT22(uVar6, piVar5), CONCAT22(uVar8, piVar7));
    paVar3             = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x9a, param_1);
    iVar3->field_0x24  = paVar3;
    &iVar3->field_0x26 = (paVar3 >> 0x10);
    uVar4              = pass1_1008_4772((astruct_76 *)(paVar3 & 0xffff0000 | iVar3->field_0x24));
    uVar1              = (uVar4 >> 0x10);
    pass1_1000_4906((astruct_20 *)(param_2 & 0xffff0000 | &iVar3->field_0x32), 0x0, 0x8);
    iVar3->field_0x36 = (uVar4 + 0x4);
    iVar3->field_0x38 = (uVar4 + 0x8);
    iVar3->field_0x2a = local_4 + 0x14;
    iVar3->field_0x2c = local_6 + 0x14;
    get_sys_metrics_1018_1ea0(param_2, 0x1000);
    pass1_1008_3e76((param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)), 0x0, 0x88, 0x99);
    return;
}

void pass1_1018_1a8e(u32 param_1, u8 *param_2, i16 param_3, u16 param_4)

{
    long         lVar1;
    astruct_653 *iVar2;
    u16          uVar2;
    u16         *puVar3;
    i16         *piVar4;
    i16          local_8;
    u32   uStack6;

    uVar2 = (param_1 >> 0x10);
    iVar2 = (astruct_653 *)param_1;
    if(iVar2->field_0x44 != 0x0)
    {
        if(iVar2->field_0x46 != 0x0)
        {
            lVar1             = iVar2->field_0x46;
            (lVar1 + 0xe)     = 0x0;
            iVar2->field_0x46 = 0x0;
        }
        piVar4  = &iVar2->field_0x4a;
        *piVar4 = *piVar4 + 0x1;
        return;
    }
    piVar4 = CONCAT22(param_4, &local_8);
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, param_2, param_3);
    pass1_1010_bf1e(puVar3, piVar4, puVar3, (puVar3 >> 0x10), param_4);
    iVar2->field_0x44 = local_8;
    iVar2->field_0x40 = uStack6;
    pass1_1018_1ce8(param_4, param_1 & 0xffff | uVar2 << 0x10);
    return;
}

void pass1_1010_e964(u8 *param_1, u16 param_2, i16 param_3)

{
    u32  uVar1;
    u16  uVar2;
    u16 *puVar3;

    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_2, param_1, param_3);
    uVar2  = (puVar3 >> 0x10);
    uVar1  = *(puVar3 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    pass1_1038_4d28(uVar1 & 0xffff | uVar2 << 0x10);
    return;
}

void struct_1010_e9e4(astruct_261 *param_1, u16 param_2, u16 param_3)

{
    u16        *puVar1;
    u16         uVar2;
    i16         iVar3;
    u16         uVar4;
    u16         uVar5;
    u32         uVar6;
    u8         *puVar7;
    i16         iVar8;
    astruct_79 *paVar9;
    u16        *puVar10;
    i16         iStack4;

    paVar9                     = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    puVar7                     = (paVar9 >> 0x10);
    param_1->field_0xa         = 0x389a;
    param_1->field_0xc         = 0x1008;
    param_1->field_0xa         = 0x3aa8;
    param_1->field_0xc         = 0x1008;
    uVar5                      = 0x0;
    &param_1->field_0xe        = 0x0;
    param_1->field_0x12        = 0x0;
    param_1->field_0x16        = 0x0;
    param_1->field_0x1a_addr_offset = 0x0;
    param_1->field_0x1e        = 0x0;
    param_1->field_0x20        = 0x0;
    param_1->field_0x24        = 0x0;
    param_1->field_0x28        = 0x0;
    param_1->field_0x2c        = 0x0;
    param_1->field_0x30        = 0x0;
    param_1->field_0x32        = 0x0;
    CONCAT22(param_2, param_1) = 0x558;
    param_1->field_0x2         = 0x1018;
    param_1->field_0xa         = 0x568;
    param_1->field_0xc         = 0x1018;
    mem_op_1000_179c(0x4, puVar7, 0x1000);
    if((puVar7 | uVar5) == 0x0)
    {
        &param_1->field_0xe = 0x0;
    }
    else
    {
        puVar10             = pass1_1018_dcf6(CONCAT22(puVar7, uVar5));
        param_1->field_0xe  = puVar10;
        param_1->field_0x10 = (puVar10 >> 0x10);
    }
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x34), 0x0, 0x24);
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
        pass1_1008_612e(0x0, 0x1d, uVar6);
        uVar5 = uVar6;
        pass1_1008_612e(0x1, 0x2, uVar5);
        if((uVar6 & 0x1) != 0x0)
        {
            uVar5 = -uVar5;
        }
        iVar8                          = iStack4 * 0x4;
        puVar1                         = (&param_1->field_0x34 + iVar8);
        uVar2                          = *puVar1;
        uVar4                          = uVar5 + *puVar1;
        uVar6                          = uVar4;
        iVar3                          = (&param_1->field_0x34 + iVar8 + 0x2);
        (&param_1->field_0x34 + iVar8) = uVar4;
        (&param_1->field_0x36 + iVar8) = (uVar5 >> 0xf) + iVar3 + CARRY2(uVar5, uVar2);
    }
    return;
}

void pass1_1018_0196(u32 param_1, u32 param_2, u32 param_3, u16 param_4, u8 *param_5, u16 param_6)

{
    i16       *piVar1;
    i16        iVar2;
    u32 uVar3;
    u32        uVar4;
    u16        uVar5;
    u32        uVar6;
    i16        iVar7;
    u16        uVar8;
    long       lVar9;

    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), param_3);
    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    if((iVar7 + 0x2c) == 0x0)
    {
        (iVar7 + 0x32) = 0x5;
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            globals->PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_5, 0x1000);
            globals->PTR_LOOP_1050_5f2e = param_5;
        }
        else
        {
        }
        uVar5 = fn_ptr_op_1000_1708(0x1e, 0x0, 0x1, globals->PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e, 0x1000);
    }
    else
    {
        uVar5                       = (iVar7 + 0x30) + 0x1;
        globals->PTR_LOOP_1050_5f2e = param_5;
        if(uVar5 < (iVar7 + 0x32))
            goto LAB_1018_022a;
        piVar1                      = (iVar7 + 0x32);
        *piVar1                     = *piVar1 + 0x5;
        uVar3                       = (iVar7 + 0x2c);
        lVar9                       = pass1_1000_0ed4(0x1000, param_6, 0x1, (iVar7 + 0x32) * 0x6, 0x0, uVar3, (uVar3 >> 0x10));
        globals->PTR_LOOP_1050_5f2e = (lVar9 >> 0x10);
        uVar5                       = lVar9;
    }
    (iVar7 + 0x2c) = uVar5;
    (iVar7 + 0x2e) = globals->PTR_LOOP_1050_5f2e;
LAB_1018_022a:
    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), param_2);
    uVar6 = *(uVar5 + 0x10);
    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), uVar6);
    iVar2   = (iVar7 + 0x30);
    piVar1  = (iVar7 + 0x30);
    *piVar1 = *piVar1 + 0x1;
    uVar4   = *(iVar7 + 0x2c);
    pass1_1008_3f62((uVar4 & 0xffff0000 | (uVar4 + iVar2 * 0x6)), CONCAT22(PTR_LOOP_1050_5f2e, uVar6 + 0xc));
    return;
}

void pass1_1018_028c(u32 param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    u32  uVar1;
    code      **ppcVar2;
    u8         *puVar3;
    u16         uVar4;
    i16         iVar5;
    u32         uVar6;
    u8         *puVar7;
    u8         *puVar8;
    u16         uVar9;
    u16         extraout_DX;
    u16         uVar10;
    i16         iVar11;
    i16         unaff_DI;
    u16         uVar12;
    u16         uVar13;
    u16         uVar14;
    u16         uVar15;
    i16         iStack36;
    u32 *puStack28;
    u8          local_18[0x4];
    u16         uStack20;
    u16        *puStack12;
    u16         uStack8;
    u16         uStack6;
    u8         *puStack4;

    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), param_2);
    uStack6   = param_3;
    puStack4  = param_4;
    uStack8   = pass1_1030_5b00(CONCAT22(param_4, param_3));
    puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, uStack8, param_5, param_4, unaff_DI);
    pass1_1008_6c90(CONCAT22(param_5, local_18));
    pass1_1018_0b1e(puStack12, CONCAT22(param_5, local_18), param_5);
    puVar7 = (uStack20 >> 0xf);
    if((puVar7 | uStack20) == 0x0)
    {
        puVar3 = local_18;
        pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(param_5, puVar3), param_2, param_5);
    }
    else
    {
        puVar3 = local_18;
        pass1_1030_62e4(_PTR_LOOP_1050_5740, CONCAT22(param_5, puVar3), param_2, param_5);
    }
    puStack28 = CONCAT22(puVar7, puVar3);
    uVar4     = puVar7 | puVar3;
    if(uVar4 == 0x0)
    {
        return;
    }
    puVar8 = puVar7;
    pass1_1018_04f2(param_1);
    uVar14 = 0x1c;
    uVar13 = 0x1000;
    mem_op_1000_179c(0x1c, puVar8, 0x1000);
    uVar9  = puVar8 | uVar4;
    iVar11 = param_1;
    uVar12 = (param_1 >> 0x10);
    uVar15 = uVar4;
    if(uVar9 == 0x0)
    {
        (iVar11 + 0x12) = 0x0;
    }
    else
    {
        uVar13 = 0x1008;
        struct_op_1008_8e9e((astruct_78 *)CONCAT22(puVar8, uVar4), 0x6, 0x24);
        (iVar11 + 0x12) = uVar4;
        (iVar11 + 0x14) = uVar9;
    }
    ppcVar2 = (*puStack28 + 0x10);
    (**ppcVar2)(uVar13, puVar3, puVar7, uVar14, uVar15);
    for(iStack36 = 0x0; iStack36 < uVar4; iStack36 = iStack36 + 0x1)
    {
        uVar6   = SEXT24(iStack36);
        ppcVar2 = (*puStack28 + 0x4);
        (**ppcVar2)();
        if((extraout_DX | uVar6) != 0x0)
        {
            iVar5  = iStack36 / 0x6;
            uVar10 = iStack36 % 0x6;
            uVar1  = (iVar11 + 0xe);
            pass1_1018_dd7c(uVar1, (uVar1 >> 0x10), CONCAT22(iStack36 % 0x6, iVar5), uVar6 & 0xffff | extraout_DX << 0x10, uVar10, param_5);
            pass1_1008_8faa((iVar11 + 0x12), CONCAT22(uVar10, iVar5));
        }
    }
    return;
}

void pass1_1018_0412(u32 param_1, u16 param_2, u32 param_3, u16 param_4, u32 param_5, u16 param_6, u8 param_7)

{
    u8 *puVar1;
    u8  local_128[0x124];
    u16 uStack4;

    uStack4 = 0x0;
    if(((0x72 < param_4) && (!SBORROW2(param_4, 0x73))) && ((param_4 == 0x75 || (param_4 - 0x74) < 0x1 || ((0x0 < (param_4 - 0x76) && ((param_4 - 0x77) < 0x2))))))
    {
        uStack4 = 0x1;
    }
    struct_op_1028_933c((astruct_100 *)CONCAT22(param_6, local_128), param_2, uStack4, param_4, param_3, (param_3 >> 0x10), *(param_1 + 0x24), param_5, param_6, param_7);
    puVar1 = local_128;
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_6, puVar1));
    if(puVar1 != 0x0)
    {
        pass1_1010_1f62(param_6, param_1, 0x6);
    }
    return;
}

void pass1_1018_04a4(u32 param_1, u32 param_2)

{
    *(param_1 + 0x16) = param_2;
    return;
}


u32 pass1_1018_04b8(u32 param_1)

{
    u16 uVar1;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16));
}


void pass1_1018_04ca(u32 param_1, u32 param_2)

{
    *(param_1 + 0x1a) = param_2;
    return;
}


void pass1_1018_04de(u32 param_1, u32 param_2)

{
    *(param_1 + 0x20) = param_2;
    return;
}

void struct_1018_0570(astruct_55 *param_1, u16 param_2, u16 param_3)

{
    u32  *puVar1;
    code       **ppcVar2;
    u16         *puVar3;
    u16          uVar4;
    u8          *puVar5;
    u16          uVar6;
    u8          *extraout_DX;
    i16          unaff_DI;
    u16         *puVar7;
    u16          uVar8;
    astruct_262 *uVar9;

    uVar9 = (astruct_262 *)param_1;
    uVar8 = (param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, 0x0, param_2);
    uVar9->field_0x20 = 0x389a;
    uVar9->field_0x22 = 0x1008;
    uVar9->field_0x20 = 0x3aa8;
    uVar9->field_0x22 = 0x1008;
    uVar9->field_0x24 = 0x0;
    uVar9->field_0x2c = 0x0;
    pass1_1008_3e38((param_1 & 0xffff0000 | &uVar9->field_0x30));
    puVar7            = pass1_1008_3e38((param_1 & 0xffff0000 | &uVar9->field_0x36));
    puVar5            = (puVar7 >> 0x10);
    uVar9->field_0x3c = 0x0;
    pass1_1008_6c90((param_1 & 0xffff0000 | &uVar9->field_0x40));
    uVar6              = 0x0;
    uVar9->field_0x4c  = 0x0;
    uVar9->field_0x5a  = 0x0;
    uVar9->field_0x5e  = 0x0;
    uVar9->field_0x60  = 0x0;
    uVar9->field_0x64  = 0xff00;
    uVar9->field_0x66  = 0x0;
    uVar9->field_0x68  = 0x10000fb;
    uVar9->field_0x6c  = 0x10000f9;
    uVar9->field_0x70  = 0x10000ff;
    uVar9->field_0x74  = 0x10000fe;
    uVar9->field_0x78  = 0x10000fc;
    uVar9->field_0x7c  = 0x0;
    uVar9->field_0x80  = 0x0;
    uVar9->field_0x84  = 0x1;
    uVar9->field_0x86  = 0x0;
    uVar9->field_0x88  = 0x0;
    uVar9->field_0x8c  = 0x0;
    uVar9->field_0x8e  = 0x0;
    uVar9->field_0x92  = 0x0;
    uVar9->field_0x94  = 0x0;
    uVar9->field_0x98  = 0x0;
    uVar9->field_0x9a  = 0x0;
    &uVar9->field_0xa2 = 0x0;
    uVar9->field_0xa6  = 0xffff;
    uVar9->field_0xa8  = 0x0;
    param_1->field_0x0 = 0x1874;
    uVar9->field_0x2   = 0x1018;
    uVar9->field_0x20  = 0x18b0;
    uVar9->field_0x22  = 0x1018;
    if((PTR_LOOP_1050_3960 == 0x0) && (_PTR_LOOP_1050_3962 == 0x0))
    {
        mem_op_1000_179c(0x8, puVar5, 0x1000);
        globals->_PTR_LOOP_1050_3962 = CONCAT22(puVar5, uVar6);
        pass1_1000_4906((astruct_20 *)CONCAT22(puVar5, uVar6), 0x0, 0x8);
    }
    globals->PTR_LOOP_1050_3960 = globals->PTR_LOOP_1050_3960 + 0x1;
    puVar7                      = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_3, puVar5, unaff_DI);
    &uVar9->field_0x2c          = puVar7;
    (&uVar9->field_0x2c + 0x2)  = (puVar7 >> 0x10);
    if(param_1 == (astruct_55 *)0x0)
    {
        puVar3 = 0x0;
        uVar6  = 0x0;
    }
    else
    {
        puVar3 = &uVar9->field_0x20;
        uVar6  = uVar8;
    }
    puVar1  = uVar9->field_0x2c;
    ppcVar2 = (*uVar9->field_0x2c + 0x4);
    (**ppcVar2)(0x1010, puVar1, (puVar1 >> 0x10), 0x0, puVar3, uVar6);
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_3, extraout_DX, unaff_DI);
    puVar5 = (puVar7 >> 0x10);
    if((puVar7 + 0x80) != 0x0)
    {
        uVar9->field_0x84 = 0x2;
    }
    puVar7            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, param_3, puVar5, unaff_DI);
    puVar5            = (puVar7 >> 0x10);
    uVar9->field_0x9e = puVar7;
    uVar9->field_0xa0 = puVar5;
    uVar4             = pass1_1010_65d0(param_3, puVar7 & 0xffff0000 | uVar9->field_0x9e, 0x88);
    if(uVar4 != 0x0)
    {
        uVar9->field_0xa8 = 0x1;
    }
    puVar7            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_3, puVar5, unaff_DI);
    uVar9->field_0xa2 = puVar7;
    uVar9->field_0xa4 = (puVar7 >> 0x10);
    return;
}

void get_sys_metrics_1018_09a8(u32 param_1, u16 param_2)

{
    u32 uVar1;
    u16      IVar2;
    u16      IVar3;
    u8        *in_DX;
    i16        iVar4;
    i16        unaff_DI;
    u16        uVar5;
    u16        unaff_SS;
    u16       *puVar6;
    u16       *puVar7;
    u16       *puVar8;
    i16        local_a;
    i16        local_8;
    i16        iStack6;
    u16      IStack4;

    IStack4 = GetSystemMetrics16(param_2);
    uVar5   = (param_1 >> 0x10);
    iVar4   = param_1;
    iStack6 = (iVar4 + 0x12) + -0x2;
    puVar8  = CONCAT22(unaff_SS, &local_8);
    puVar7  = CONCAT22(unaff_SS, &local_a);
    puVar6  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)), puVar7, puVar8);
    (iVar4 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
    (iVar4 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
    IVar2          = GetSystemMetrics16(0x1008);
    uVar1          = (iVar4 + 0x5a);
    (iVar4 + 0x1c) = IVar2 * 0x2 + (uVar1 + 0x4);
    IVar2          = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    IVar3          = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    uVar1          = (iVar4 + 0x5a);
    (iVar4 + 0x1e) = IVar3 + IVar2 + (uVar1 + 0x8);
    return;
}

void pass1_1010_d448(u8 *param_1, u32 param_2, u16 *param_3, u8 *param_4, u8 param_5, i16 param_6)

{
    u16        uVar1;
    u16       *puVar2;
    u32 uVar3;
    u32        uVar4;
    u16       *puVar5;
    char      *pcVar6;
    i16        iVar7;
    u16        uVar8;
    u16        uVar9;
    i16        iVar10;
    u16        uVar11;
    u32        uVar12;
    u16        uVar13;
    u16        local_40c;
    u32        uStack1034;
    u32        uStack1030;
    u8         local_402[0x400];

    uVar11     = (param_3 >> 0x10);
    iVar10     = param_3;
    uStack1030 = struct_op_1030_73a8(*(iVar10 + 0x6));
    uVar8      = (uStack1030 >> 0x10);
    uVar1      = uStack1030;
    if((uVar8 | uVar1) != 0x0)
    {
        uStack1034 = *(uVar1 + 0x20);
        uVar1      = (uVar1 + 0x22);
        if((uVar1 | uStack1034) != 0x0)
        {
            local_40c = 0x0;
            puVar5    = &local_40c;
            uVar13    = (param_1 >> 0x10);
            pass1_1010_d984(param_1, uVar13, CONCAT22(param_4, puVar5), 0x3, uStack1034 & 0xffff | uVar1 << 0x10, &PTR_DAT_1050_1805_1050_368e, param_3, param_4, param_5);
            puVar2         = *(u16 **)(iVar10 + 0x2);
            uVar9          = (iVar10 + 0x4);
            (puVar2 + 0x4) = globals->PTR_DAT_1050_1805_1050_368e;
            uVar3          = (iVar10 + 0x6);
            pcVar6         = pass1_1010_b038(param_1, uVar3, (uVar3 >> 0x10), (puVar2 + 0x4), param_6);
            unk_str_op_1000_3d3e(CONCAT22(param_4, local_402), CONCAT22(uVar9, pcVar6));
            string_1040_a626(puVar2, CONCAT22(param_4, local_402), uVar9);
            uVar4         = *(iVar10 + 0x2);
            uVar9         = (iVar10 + 0x4);
            iVar7         = uVar4;
            (iVar7 + 0xe) = globals->PTR_DAT_1050_1822_1050_3690;
            sys_1000_3f9c(local_402, param_4, 0x3920, &USHORT_1050_1050, local_40c, &stack0xfffe, uVar9, 0x1000, param_4, param_5);
            string_1040_a626((uVar4 & 0xffff0000 | (iVar7 + 0xa)), CONCAT22(param_4, local_402), uVar9);
            uVar4           = *(iVar10 + 0x2);
            uVar11          = (iVar10 + 0x4);
            iVar10          = uVar4;
            (iVar10 + 0x18) = globals->PTR_DAT_1050_1823_1050_3692;
            uVar12          = pass1_1028_62c8(uStack1030, param_4);
            uVar9           = (uVar12 >> 0x10);
            sys_1000_3f9c(local_402, param_4, 0x3923, &USHORT_1050_1050, uVar12, &stack0xfffe, uVar11, 0x1000, param_4, param_5);
            string_1040_a626((uVar4 & 0xffff0000 | (iVar10 + 0x14)), CONCAT22(param_4, local_402), uVar9);
            pass1_1010_dc36(param_1, uVar13, puVar5, param_2, param_3, param_4);
        }
    }
    return;
}
