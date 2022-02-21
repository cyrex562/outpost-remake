
ushort *__stdcall16far pass1_1010_3702(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)(param_1 + 0xa)        = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x37c4;
    *(undefined2 *)(param_1 + 0x2)        = 0x1010;
    return (ushort *)CONCAT22(param_2, param_1);
}

ushort *__stdcall16far pass1_1010_37d4(ushort *param_1)

{
    undefined2 uVar1;

    struct_1010_383a(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x16) = 0x0;
    *param_1                             = 0x3b3e;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1010;
    return param_1;
}

void __stdcall16far struct_1010_383a(ushort *param_1)

{
    astruct_223 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_223 *)param_1;
    *param_1          = 0x389a;
    iVar1->field_0x2  = 0x1008;
    iVar1->field_0x4  = 0x0;
    iVar1->field_0x8  = 0x0;
    iVar1->field_0xc  = 0x0;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x14 = 0x0;
    *param_1          = 0x3b5e;
    iVar1->field_0x2  = 0x1010;
    return;
}

void __stdcall16far struct_1010_3b7a(astruct_648 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x389a;
    param_1->field_0xc                        = 0x1008;
    param_1->field_0xa                        = 0x3aa8;
    param_1->field_0xc                        = 0x1008;
    param_1->field_0xe                        = 0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x14                       = 0x0;
    param_1->field_0x16                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x3d6a;
    param_1->field_0x2                        = 0x1010;
    param_1->field_0xa                        = 0x3d7a;
    param_1->field_0xc                        = 0x1010;
    return;
}

ushort *__stdcall16far pass1_1010_2bfc(astruct_644 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                    = 0x0;
    param_1->field_0xc                    = 0x0;
    param_1->field_0xe                    = 0x0;
    param_1->field_0x10                   = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x2cc2;
    param_1->field_0x2                    = 0x1010;
    return (ushort *)CONCAT22(param_2, param_1);
}

astruct_79 *__stdcall16far struct_op_1010_1d48(astruct_79 *param_1, ushort param_2)

{
    astruct_79 *iVar1;
    astruct_79 *uVar1;

    uVar1              = (astruct_79 *)((ulong)param_1 >> 0x10);
    iVar1              = (astruct_79 *)param_1;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    iVar1->field_0x4   = 0x0;
    iVar1->field_0x8   = param_2;
    param_1->field_0x0 = 0x2014;
    iVar1->field_0x2   = 0x1010;
    return param_1;
}

ulong __stdcall16far pass1_1010_0eac(uchar *param_1, uchar *param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    struct_op_1018_4cda((int)param_1, (ushort)param_2, param_3);
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xf0c;
    *(undefined2 *)(param_1 + 0x2)            = 0x1010;
    PTR_LOOP_1050_4230                        = param_1;
    PTR_LOOP_1050_4232                        = param_2;
    pass1_1018_4dce((ulong *)CONCAT22(param_2, param_1), 0xff, param_4, param_5);
    return CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1010_0f24(astruct_79 *param_1, astruct_79 *param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    int     unaff_DI;
    ushort *puVar1;

    struct_1010_2cd2(param_1, param_2, param_3, param_5);
    *(undefined4 *)((int)&param_1[0x9].field_0x4 + 0x2) = 0x0;
    *(undefined4 *)(param_1 + 0xa)                      = 0x0;
    *(undefined2 *)&param_1[0xa].field_0x4              = 0x0;
    *(undefined4 *)((int)&param_1[0xa].field_0x4 + 0x2) = 0x0;
    *(int *)CONCAT22(param_2, param_1)                  = (int)s_648_bmp_1050_1919 + 0x1;
    param_1->field_0x2                                  = 0x1010;
    puVar1                                              = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_5, param_4, unaff_DI);
    *(undefined2 *)((int)&param_1[0xa].field_0x4 + 0x2) = (int)puVar1;
    param_1[0xa].field_0x8                              = (ushort)((ulong)puVar1 >> 0x10);
    return;
}


void __stdcall16far struct_1010_0f9c(ulong *param_1, ushort param_2, ushort param_3)

{
    code       **ppcVar1;
    ushort       uVar2;
    undefined   *puVar3;
    undefined   *puVar4;
    undefined4   uVar5;
    uchar       *extraout_DX;
    uchar       *puVar6;
    uchar       *puVar7;
    uchar       *extraout_DX_00;
    undefined2   extraout_DX_01;
    undefined2   extraout_DX_02;
    undefined2   extraout_DX_03;
    uchar       *extraout_DX_04;
    astruct_232 *iVar8;
    astruct_231 *iVar9;
    astruct_230 *iVar10;
    astruct_233 *iVar11;
    undefined2   uVar8;
    undefined2   uVar9;
    ulong       *puVar10;
    undefined4   uVar11;
    ulong       *puVar12;
    undefined    uVar13;
    undefined4   uStack36;
    int          iStack32;
    ushort       uStack30;
    uint        *puStack28;
    undefined4   uStack24;
    undefined    local_14[0x12];

    uVar8   = (undefined2)((ulong)param_1 >> 0x10);
    iVar8   = (astruct_232 *)param_1;
    ppcVar1 = (code **)((int)*param_1 + 0x38);
    (**ppcVar1)();
    iVar8->field_0x68 = param_2;
    if((*(long *)&iVar8->field_0x60 != 0x0) && (iVar8->field_0x68 == 0x1))
    {
        return;
    }
    if(iVar8->field_0x68 == 0x0)
    {
        return;
    }
    puVar7 = extraout_DX;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    uVar2 = iVar8->field_0x68 * 0x18;
    mem_op_1000_179c(uVar2, puVar7, 0x1000);
    iVar8->field_0x60 = uVar2;
    iVar8->field_0x62 = puVar7;
    puStack28         = (uint *)CONCAT22(puVar7, iVar8->field_0x60);
    uStack30          = iVar8->field_0x68;
    do
    {
        do
        {
            puVar6 = puVar7;
            puVar3 = local_14;
            pass1_1028_e4ec(CONCAT13((char)(param_3 >> 0x8), CONCAT12((char)param_3, puVar3)));
            uStack24 = CONCAT22(puVar6, puVar3);
            puVar7   = (uchar *)((uint)puVar6 | (uint)puVar3);
            if(puVar7 == (uchar *)0x0)
                goto LAB_1010_10ca;
            iVar9   = (astruct_231 *)*param_1;
            ppcVar1 = (code **)&iVar9->field_0x40;
            puVar4  = puVar3;
            (**ppcVar1)();
            puVar7 = extraout_DX_00;
        } while(puVar4 == (undefined *)0x0);
        uVar13 = SUB21(puVar6, 0x0);
        pass1_1028_b58e(CONCAT13((char)((uint)puVar6 >> 0x8), CONCAT12(uVar13, puVar3)));
        uStack36 = CONCAT22(extraout_DX_01, puVar4);
        ppcVar1  = (code **)&iVar9->field_0x2c;
        puVar12  = param_1;
        (**ppcVar1)();
        uVar9             = (undefined2)((ulong)puStack28 >> 0x10);
        iVar10            = (astruct_230 *)puStack28;
        *puStack28        = (uint)puVar4;
        iVar10->field_0x2 = extraout_DX_02;
        ppcVar1           = (code **)&iVar9->field_0x30;
        puVar10           = param_1;
        uVar11            = uStack24;
        (**ppcVar1)();
        iVar10->field_0x8 = puVar4;
        iVar10->field_0xa = extraout_DX_03;
        iVar10->field_0xc = uStack36;
        ppcVar1           = (code **)&iVar9->field_0x3c;
        uVar5             = uStack36;
        (**ppcVar1)((int)&USHORT_1050_1028, param_1, uStack24, puVar10, uVar11, puVar12, puVar3, uVar13);
        iVar10->field_0x10 = (int)uVar5;
        iVar10->field_0x12 = extraout_DX_04;
        iVar10->field_0x14 = uStack36;
        puStack28          = (uint *)((ulong)puStack28 & 0xffff0000 | ZEXT24(iVar10 + 0x1));
        uStack30           = uStack30 - 0x1;
        puVar7             = extraout_DX_04;
    } while(uStack30 != 0x0);
LAB_1010_10ca:
    uVar2 = iVar8->field_0x68 << 0x2;
    mem_op_1000_179c(uVar2, puVar7, 0x1000);
    iVar8->field_0x64 = uVar2;
    iVar8->field_0x66 = puVar7;
    iStack32          = 0x0;
    uStack30          = 0x0;
    while(true)
    {
        if((int)(iVar8->field_0x68 * 0x3) <= (int)uStack30)
            break;
        puVar7                                     = iVar8->field_0x62;
        uVar5                                      = *(undefined4 *)&iVar8->field_0x64;
        uVar9                                      = (undefined2)((ulong)uVar5 >> 0x10);
        iVar11                                     = (astruct_233 *)uVar5;
        *(ushort *)(iVar11 + iStack32 * 0x4)       = iVar8->field_0x60 + uStack30 * 0x8;
        *(uchar **)(iVar11 + iStack32 * 0x4 + 0x2) = puVar7;
        uStack30                                   = uStack30 + 0x3;
        iStack32                                   = iStack32 + 0x1;
    }
    return;
}

undefined2 *__stdcall16far pass1_1008_eabc(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined2 *)(param_1 + 0xa) = 0x0;
    pass1_1008_3e38((ushort *)CONCAT22(param_2, param_1 + 0xc));
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xeb1a;
    *(undefined2 *)(param_1 + 0x2)            = 0x1008;
    return (undefined2 *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1008_eb2a(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined2 *)(param_1 + 0xa)            = 0x0;
    *(undefined4 *)(param_1 + 0xc)            = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xec00;
    *(undefined2 *)(param_1 + 0x2)            = 0x1008;
    return;
}

ushort *__stdcall16far pass1_1008_ec10(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined2 *)(param_1 + 0xa)        = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0xec62;
    *(undefined2 *)(param_1 + 0x2)        = 0x1008;
    return (ushort *)CONCAT22(param_2, param_1);
}

ushort *__stdcall16far struct_1008_ec72(ushort *param_1)

{
    struct_1010_383a(param_1);
    *param_1                            = 0xefc4;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    return param_1;
}

void __stdcall16far pass1_1008_ee14(ulong param_1, ushort param_2)

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
        puVar5 = struct_1008_ec72((ushort *)CONCAT22(param_2, local_1c));
        uVar2  = (undefined2)((ulong)puVar5 >> 0x10);
        puVar1 = local_1c;
        pass1_1010_398e((ulong *)CONCAT22(param_2, puVar1), 0x0, 0x0, 0x0, (ushort)puVar1);
        *(ushort *)(iVar3 + 0x56)     = (ushort)puVar1;
        *(undefined2 *)(iVar3 + 0x58) = uVar2;
        pass1_1008_ec94((ushort *)CONCAT22(param_2, local_1c));
    }
    return;
}
