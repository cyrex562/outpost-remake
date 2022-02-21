
void __stdcall16far struct_1040_b082(astruct_57 *param_1, ulong param_2)

{
    astruct_437 *iVar1;
    undefined2   uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, (ushort)param_2, (ushort)(param_2 >> 0x10));
    uVar1                  = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                  = (astruct_437 *)param_1;
    iVar1->field_0x8e      = 0x0;
    iVar1->field_0x90      = 0x0;
    *(undefined2 *)param_1 = 0xb772;
    iVar1->field_0x2       = (int)&PTR_LOOP_1050_1040;
    return;
}

void __stdcall16far pass1_1040_b040(astruct_57 *param_1, ulong param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, *(ushort *)((int)param_2 + 0x12), param_3);
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined2 *)(iVar1 + 0x8e) = 0x0;
    *(ulong *)(iVar1 + 0x90)      = param_2;
    *(undefined2 *)param_1        = 0xb772;
    *(undefined2 *)(iVar1 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    return;
}

void __stdcall16far pass1_1040_b0bc(astruct_57 *param_1, ulong param_2, ulong param_3)

{
    int        iVar1;
    undefined2 uVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, (ushort)param_3, (ushort)(param_3 >> 0x10));
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined2 *)(iVar1 + 0x8e) = 0x0;
    *(ulong *)(iVar1 + 0x90)      = param_2;
    *(undefined2 *)param_1        = 0xb772;
    *(undefined2 *)(iVar1 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    return;
}

void __stdcall16far pass1_1040_4068(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    uchar       *puVar1;
    astruct_723 *iVar2;
    undefined2   uVar2;
    ushort      *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb7, param_5);
    uVar2                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                             = (astruct_723 *)param_1;
    *(undefined4 *)&iVar2->field_0x8e = 0x0;
    iVar2->field_0x92                 = 0x0;
    iVar2->field_0x9a                 = 0x0;
    *(undefined2 *)param_1            = 0x4466;
    iVar2->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    iVar2->field_0x76                 = 0x1;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_8, param_6, param_7);
    puVar1                            = (uchar *)((ulong)puVar3 >> 0x10);
    iVar2->field_0x8e                 = (int)puVar3;
    iVar2->field_0x90                 = puVar1;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_8, puVar1, param_7);
    iVar2->field_0x96                 = (int)puVar3;
    iVar2->field_0x98                 = (int)((ulong)puVar3 >> 0x10);
    return;
}

void __stdcall16far
get_sys_metrics_1040_7728(astruct_57 *param_1, ushort param_2, ulong param_3, ushort param_4, ushort param_5)

{
    INT16       IVar1;
    astruct_57 *iVar2;
    undefined2  uVar2;

    uVar2                  = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                  = (astruct_57 *)param_1;
    *(undefined2 *)param_1 = 0x389a;
    iVar2->field_0x2       = 0x1008;
    *(undefined2 *)param_1 = 0x3aa8;
    iVar2->field_0x2       = 0x1008;
    iVar2->field_0x4       = 0x0;
    iVar2->field_0x6       = 0x0;
    iVar2->field_0x8       = param_5;
    iVar2->field_0xa       = param_4;
    iVar2->field_0xc       = 0x0;
    iVar2->field_0x60      = 0x0;
    iVar2->field_0x62      = 0x0;
    iVar2->field_0x64      = 0x0;
    iVar2->field_0x66      = 0x0;
    iVar2->field_0x68      = 0x0;
    iVar2->field_0x6a      = param_3;
    iVar2->field_0x6e      = param_2;
    iVar2->field_0x70      = 0x0;
    iVar2->field_0x74      = 0x0;
    iVar2->field_0x76      = 0x0;
    iVar2->field_0x78      = 0x0;
    iVar2->field_0x8a      = 0x0;
    iVar2->field_0x8c      = 0x0;
    *(undefined2 *)param_1 = 0x840c;
    iVar2->field_0x2       = (int)&PTR_LOOP_1050_1040;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x10), (char *)0x10505db0);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x7a), (WNDCLASS16 *)0x0, 0x8);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x82), (WNDCLASS16 *)0x0, 0x8);
    IVar1             = GetSystemMetrics16(0x1000);
    iVar2->field_0x62 = IVar1;
    IVar1             = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    iVar2->field_0x64 = IVar1;
    IVar1             = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    iVar2->field_0x66 = IVar1;
    return;
}

void __stdcall16far pass1_1040_44d2(astruct_57 *param_1, ulong param_2, ushort param_3, uint param_4, uchar *param_5)

{
    undefined4 uVar1;
    uint       uVar2;
    uchar     *puVar3;
    int        iVar4;
    undefined2 uVar5;
    int        iVar6;
    undefined2 uVar7;
    int       *piStack8;

    iVar6 = (int)param_1;
    uVar7 = (undefined2)((ulong)param_1 >> 0x10);
    struct_1040_b082(param_1, CONCAT22(param_3, 0xfa2));
    *(undefined2 *)param_1       = 0x4824;
    *(undefined2 *)(iVar6 + 0x2) = (int)&PTR_LOOP_1050_1040;
    mem_op_1000_179c(0x18, param_5, 0x1000);
    puVar3 = (uchar *)((uint)param_5 | param_4);
    if(puVar3 == (uchar *)0x0)
    {
        *(undefined4 *)(iVar6 + 0x90) = 0x0;
    }
    else
    {
        struct_1040_a598((ushort *)CONCAT22(param_5, param_4));
        *(uint *)(iVar6 + 0x90)   = param_4;
        *(uchar **)(iVar6 + 0x92) = puVar3;
    }
    *(undefined2 *)*(undefined4 *)(iVar6 + 0x90) = 0x14;
    iVar4                                        = **(int **)(iVar6 + 0x90);
    uVar2                                        = iVar4 * 0xa + 0x2;
    mem_op_1000_179c(uVar2, puVar3, 0x1000);
    piStack8 = (int *)CONCAT22(puVar3, uVar2);
    if(((uint)puVar3 | uVar2) == 0x0)
    {
        uVar1                             = *(undefined4 *)(iVar6 + 0x90);
        *(undefined4 *)((int)uVar1 + 0x2) = 0x0;
    }
    else
    {
        *piStack8 = iVar4;
        pass1_1000_5586((uchar *)0xa564, (ushort)&PTR_LOOP_1050_1040, iVar4, 0xa, uVar2 + 0x2, (ushort)puVar3);
        uVar1                    = *(undefined4 *)(iVar6 + 0x90);
        uVar5                    = (undefined2)((ulong)uVar1 >> 0x10);
        iVar4                    = (int)uVar1;
        *(int *)(iVar4 + 0x2)    = uVar2 + 0x2;
        *(uchar **)(iVar4 + 0x4) = puVar3;
    }
    uVar1                              = *(undefined4 *)(iVar6 + 0x90);
    *(ulong *)((int)uVar1 + 0x6)       = param_2;
    uVar1                              = *(undefined4 *)(iVar6 + 0x90);
    *(undefined2 *)((int)uVar1 + 0xa)  = 0x1;
    uVar1                              = *(undefined4 *)(iVar6 + 0x90);
    *(undefined2 *)((int)uVar1 + 0x12) = *(undefined2 *)(iVar6 + 0xa);
    return;
}

void __stdcall16far pass1_1040_45e8(
  int param_1, ushort param_2, ushort param_3, ulong param_4, uchar *param_5, ushort param_6, ushort param_7)

{
    astruct_18 *paVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    uint        uVar4;
    astruct_18 *paVar5;
    uchar      *puVar6;
    uchar      *puVar7;
    int         iVar8;
    int         unaff_DI;
    undefined2  uVar9;
    astruct_20 *paVar10;
    int        *piStack16;

    if(param_4._2_2_ != 0xeb)
    {
        pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
        return;
    }
    paVar10 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_7, param_5, unaff_DI);
    puVar6  = (uchar *)((ulong)paVar10 >> 0x10);
    paVar1  = *(astruct_18 **)(param_1 + 0x90);
    if(paVar1 != (astruct_18 *)0x0)
    {
        paVar5 = paVar1;
        mem_op_1000_179c(0x18, puVar6, 0x1000);
        uVar4  = (uint)paVar5;
        puVar7 = (uchar *)((uint)puVar6 | uVar4);
        if(puVar7 == (uchar *)0x0)
        {
            uVar4  = 0x0;
            puVar7 = (uchar *)0x0;
        }
        else
        {
            struct_1040_a598((ushort *)((ulong)paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
        }
        *(uint *)(param_1 + 0x90)                      = uVar4;
        *(uchar **)(param_1 + 0x92)                    = puVar7;
        *(undefined2 *)*(undefined4 *)(param_1 + 0x90) = 0x14;
        iVar8                                          = **(int **)(param_1 + 0x90);
        uVar4                                          = iVar8 * 0xa + 0x2;
        mem_op_1000_179c(uVar4, puVar7, 0x1000);
        piStack16 = (int *)CONCAT22(puVar7, uVar4);
        if(((uint)puVar7 | uVar4) == 0x0)
        {
            uVar3                             = *(undefined4 *)(param_1 + 0x90);
            *(undefined4 *)((int)uVar3 + 0x2) = 0x0;
        }
        else
        {
            *piStack16 = iVar8;
            pass1_1000_5586((uchar *)0xa564, (ushort)&PTR_LOOP_1050_1040, iVar8, 0xa, uVar4 + 0x2, (ushort)puVar7);
            uVar3                    = *(undefined4 *)(param_1 + 0x90);
            uVar9                    = (undefined2)((ulong)uVar3 >> 0x10);
            iVar8                    = (int)uVar3;
            *(int *)(iVar8 + 0x2)    = uVar4 + 0x2;
            *(uchar **)(iVar8 + 0x4) = puVar7;
        }
        uVar3                              = *(undefined4 *)(param_1 + 0x90);
        *(undefined4 *)((int)uVar3 + 0x6)  = *(undefined4 *)((int)paVar1 + 0x6);
        uVar3                              = *(undefined4 *)(param_1 + 0x90);
        *(undefined2 *)((int)uVar3 + 0xa)  = 0x1;
        uVar3                              = *(undefined4 *)(param_1 + 0x90);
        *(undefined2 *)((int)uVar3 + 0x12) = *(undefined2 *)(param_1 + 0xa);
        pass1_1010_a50c(paVar10, 0x10505d40, *(ulong *)(param_1 + 0x90));
        if(paVar1 != (astruct_18 *)0x0)
        {
            pass1_1040_a5d0((ulong)paVar1);
            fn_ptr_1000_17ce(paVar1, 0x1000);
        }
        ppcVar2 = (code **)((int)*(undefined4 *)CONCAT22(param_2, param_1) + 0x70);
        (**ppcVar2)();
    }
    return;
}

void __stdcall16far
pass1_1040_48a0(astruct_57 *param_1, ushort param_2, ulong param_3, ushort param_4, uchar *param_5, ushort param_6)

{
    int          iVar1;
    int         *piVar2;
    uint         uVar3;
    uchar       *puVar4;
    uchar       *puVar5;
    astruct_444 *iVar5;
    astruct_445 *iVar6;
    int          unaff_DI;
    undefined2   uVar6;
    undefined2   uVar7;
    ushort      *puVar8;
    int         *piStack8;

    struct_1040_b082(param_1, CONCAT22(param_4, 0xfa1));
    uVar6                                      = (undefined2)((ulong)param_1 >> 0x10);
    iVar5                                      = (astruct_444 *)param_1;
    iVar5->field_0x94                          = 0x0;
    *(int *)param_1                            = (int)&PTR_LOOP_1050_4e18;
    iVar5->field_0x2                           = (int)&PTR_LOOP_1050_1040;
    puVar8                                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_5, unaff_DI);
    puVar4                                     = (uchar *)((ulong)puVar8 >> 0x10);
    uVar3                                      = (uint)puVar8;
    *(uint *)&iVar5->field_0x94                = uVar3;
    *(uchar **)((int)&iVar5->field_0x94 + 0x2) = puVar4;
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    puVar5 = (uchar *)((uint)puVar4 | uVar3);
    if(puVar5 == (uchar *)0x0)
    {
        iVar5->field_0x90 = (int *)0x0;
    }
    else
    {
        struct_1040_a598((ushort *)CONCAT22(puVar4, uVar3));
        *(uint *)&iVar5->field_0x90                = uVar3;
        *(uchar **)((int)&iVar5->field_0x90 + 0x2) = puVar5;
    }
    *iVar5->field_0x90 = 0x7;
    iVar1              = *iVar5->field_0x90;
    uVar3              = iVar1 * 0xa + 0x2;
    mem_op_1000_179c(uVar3, puVar5, 0x1000);
    piStack8 = (int *)CONCAT22(puVar5, uVar3);
    if(((uint)puVar5 | uVar3) == 0x0)
    {
        piVar2                             = iVar5->field_0x90;
        *(undefined4 *)((int)piVar2 + 0x2) = 0x0;
    }
    else
    {
        *piStack8 = iVar1;
        pass1_1000_5586((uchar *)0xa564, (ushort)&PTR_LOOP_1050_1040, iVar1, 0xa, uVar3 + 0x2, (ushort)puVar5);
        piVar2           = iVar5->field_0x90;
        uVar7            = (undefined2)((ulong)piVar2 >> 0x10);
        iVar6            = (astruct_445 *)piVar2;
        iVar6->field_0x2 = uVar3 + 0x2;
        iVar6->field_0x4 = puVar5;
    }
    piVar2                              = iVar5->field_0x90;
    *(ulong *)((int)piVar2 + 0x6)       = param_3;
    piVar2                              = iVar5->field_0x90;
    *(ushort *)((int)piVar2 + 0xa)      = param_2;
    piVar2                              = iVar5->field_0x90;
    *(undefined2 *)((int)piVar2 + 0x12) = iVar5->field_0xa;
    iVar1                               = *(int *)&iVar5->field_0x90;
    uVar7                               = *(undefined2 *)((int)&iVar5->field_0x90 + 0x2);
    pass1_1010_debe(iVar5->field_0x94,
                    *(ushort *)(iVar1 + 0xa),
                    (ushort *)CONCAT22(uVar7, iVar1 + 0x10),
                    (ulong *)CONCAT22(uVar7, iVar1 + 0xc),
                    param_3,
                    param_6);
    return;
}

void __stdcall16far pass1_1040_23ea(
  astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7)

{
    undefined4   uVar1;
    astruct_436 *iVar2;
    int          unaff_DI;
    undefined2   uVar2;
    ushort      *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x9a, param_2, 0xfbd, param_5);
    uVar2                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                             = (astruct_436 *)param_1;
    *(undefined4 *)&iVar2->field_0x8e = 0x0;
    iVar2->field_0x92                 = 0x0;
    iVar2->field_0x94                 = 0x0;
    *(undefined2 *)param_1            = 0x2956;
    iVar2->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    iVar2->field_0x8a                 = 0x26;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, param_6, (uchar *)param_7, unaff_DI);
    iVar2->field_0x8e                 = (int)puVar3;
    iVar2->field_0x90                 = (int)((ulong)puVar3 >> 0x10);
    uVar1                             = *(undefined4 *)&iVar2->field_0x8e;
    iVar2->field_0x92                 = *(undefined2 *)((int)uVar1 + 0x28);
    return;
}

void __stdcall16far pass1_1040_2ea2(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_720 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x180, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_720 *)param_1;
    iVar1->field_0x8e                 = 0x0;
    iVar1->field_0x90                 = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x94                 = 0x0;
    *(undefined4 *)&iVar1->field_0x96 = 0x0;
    *(undefined2 *)param_1            = 0x3436;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1->field_0x96                 = (int)puVar2;
    iVar1->field_0x98                 = (int)((ulong)puVar2 >> 0x10);
    return;
}

void __stdcall16far pass1_1040_34a2(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_721 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x192, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_721 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x94                 = 0x0;
    iVar1->field_0x96                 = 0x0;
    iVar1->field_0x98                 = 0x0;
    *(int *)param_1                   = (int)s_Null_Ptr_1050_38f3 + 0x7;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}

astruct_57 *__stdcall16far pass1_1040_123e(astruct_57 *param_1,
                                           ulong       param_2,
                                           ushort      param_3,
                                           ushort      param_4,
                                           ushort      param_5,
                                           uchar      *param_6,
                                           int         param_7,
                                           ushort      param_8)

{
    astruct_717 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfd1, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_717 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    *(undefined2 *)param_1            = 0x17b0;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x46, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return param_1;
}

void __stdcall16far pass1_1040_181c(
  astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, uchar *param_6, ushort param_7)

{
    astruct_433 *iVar1;
    int          unaff_DI;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfbb, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_433 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x94                 = 0x0;
    *(undefined2 *)param_1            = 0x1c48;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_7, param_6, unaff_DI);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}

void __stdcall16far pass1_1040_1cb4(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    uchar       *puVar1;
    astruct_718 *iVar2;
    undefined2   uVar2;
    ushort      *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xe8, param_5);
    uVar2                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                             = (astruct_718 *)param_1;
    *(undefined4 *)&iVar2->field_0x8e = 0x0;
    *(undefined4 *)&iVar2->field_0x92 = 0x0;
    *(undefined2 *)param_1            = 0x1eee;
    iVar2->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_8, param_6, param_7);
    puVar1                            = (uchar *)((ulong)puVar3 >> 0x10);
    iVar2->field_0x8e                 = (int)puVar3;
    iVar2->field_0x90                 = puVar1;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_8, puVar1, param_7);
    iVar2->field_0x92                 = (int)puVar3;
    iVar2->field_0x94                 = (int)((ulong)puVar3 >> 0x10);
    return;
}

void __stdcall16far pass1_1040_1f5a(astruct_57 *param_1, ushort param_2, int param_3, ushort param_4)

{
    int         *piVar1;
    uchar       *puVar2;
    astruct_719 *iVar3;
    astruct_43  *paVar3;
    ulong        uVar4;
    ushort      *puVar5;
    int          iVar6;
    undefined2   uVar7;
    int          iVar8;
    int          iVar9;
    undefined2   uVar10;
    undefined4   local_16;
    undefined4   uStack18;

    iVar6 = (int)param_1;
    uVar7 = (undefined2)((ulong)param_1 >> 0x10);
    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfcf, param_2);
    *(undefined4 *)(iVar6 + 0x8e) = 0x0;
    *(undefined4 *)(iVar6 + 0xa2) = 0x0;
    *(undefined4 *)(iVar6 + 0xa6) = 0x0;
    *(undefined2 *)param_1        = 0x237e;
    *(undefined2 *)(iVar6 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    paVar3                        = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1cc, param_4);
    *(undefined2 *)(iVar6 + 0x8e) = (int)paVar3;
    *(undefined2 *)(iVar6 + 0x90) = (int)((ulong)paVar3 >> 0x10);
    uVar4    = pass1_1008_4772((astruct_76 *)((ulong)paVar3 & 0xffff0000 | (ulong) * (uint *)(iVar6 + 0x8e)));
    puVar2   = (uchar *)(uVar4 >> 0x10);
    puVar5   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, puVar2, param_3);
    local_16 = CONCAT22(*(int *)((int)uVar4 + 0x8) + 0xa, 0xa);
    uStack18 = CONCAT22(0x1d6, *(int *)((int)uVar4 + 0x4) + -0xa);
    *(undefined4 *)(iVar6 + 0x92) = local_16;
    *(undefined4 *)(iVar6 + 0x96) = uStack18;
    *(undefined4 *)(iVar6 + 0x9a) = local_16;
    *(undefined4 *)(iVar6 + 0x9e) = uStack18;
    piVar1                        = (int *)(iVar6 + 0x9c);
    *piVar1                       = *piVar1 + 0x14;
    iVar9                         = iVar6 + 0xa2;
    iVar8                         = iVar6 + 0xa6;
    uVar10                        = uVar7;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_4, (uchar *)((ulong)puVar5 >> 0x10), iVar6 + 0xa2);
    pass1_1010_0538((ulong)puVar5, (char **)CONCAT22(uVar7, iVar8), (char **)CONCAT22(uVar10, iVar9), 0x1010, param_4);
    return;
}

void __stdcall16far pass1_1038_eeda(astruct_57 *param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5)

{
    astruct_714 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x166, param_2);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_714 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x94                 = 0x0;
    *(undefined2 *)param_1            = 0x67c;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, param_5, param_3, param_4);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    iVar1->field_0x74                 = 0x1;
    return;
}

astruct_57 *__stdcall16far pass1_1040_06e8(
  astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, uchar *param_6, ushort param_7)

{
    int        iVar1;
    int        unaff_DI;
    undefined2 uVar2;
    ushort    *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfbc, param_5);
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined4 *)(iVar1 + 0x8e) = 0x0;
    *(undefined2 *)param_1        = 0xb90;
    *(undefined2 *)(iVar1 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    puVar3                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x7, param_7, param_6, unaff_DI);
    *(undefined2 *)(iVar1 + 0x8e) = (int)puVar3;
    *(undefined2 *)(iVar1 + 0x90) = (int)((ulong)puVar3 >> 0x10);
    return param_1;
}

void __stdcall16far pass1_1040_0a1a(ulong param_1)

{
    uint        uVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    undefined4  uVar4;
    undefined4 *puVar5;
    uchar      *extraout_DX;
    uchar      *puVar6;
    uchar      *extraout_DX_00;
    uchar      *puVar7;
    int         iVar8;
    int         iVar9;
    undefined2  uVar10;
    undefined2  uVar11;
    ulong       uStack10;
    uint        uStack6;

    uVar10  = (undefined2)(param_1 >> 0x10);
    iVar8   = (int)param_1;
    uVar4   = *(undefined4 *)(iVar8 + 0x8e);
    uVar11  = (undefined2)((ulong)uVar4 >> 0x10);
    iVar9   = (int)uVar4;
    puVar2  = (undefined4 *)*(undefined4 *)(iVar9 + 0xa);
    uStack6 = (uint)puVar2;
    puVar5  = (undefined4 *)(*(uint *)(iVar9 + 0xc) | uStack6);
    if(puVar5 == (undefined4 *)0x0)
    {
        return;
    }
    ppcVar3 = (code **)((int)*puVar2 + 0x14);
    (**ppcVar3)();
    uStack10 = CONCAT22(extraout_DX, puVar5);
    puVar6   = extraout_DX;
    if(*(long *)(iVar8 + 0x70) != 0x0)
    {
        puVar5 = (undefined4 *)*(uint *)(iVar8 + 0x70);
        uVar1  = *(uint *)(iVar8 + 0x72);
        puVar6 = (uchar *)(uVar1 | (uint)puVar5);
        if(puVar6 != (uchar *)0x0)
        {
            ppcVar3 = (code **)*puVar5;
            (**ppcVar3)();
            puVar6 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar6, 0x1000);
    puVar7 = (uchar *)((uint)puVar6 | (uint)puVar5);
    if(puVar7 == (uchar *)0x0)
    {
        puVar5 = (undefined4 *)0x0;
        puVar7 = (uchar *)0x0;
    }
    else
    {
        struct_1008_4c58((ushort *)CONCAT22(puVar6, puVar5));
    }
    *(undefined2 *)(iVar8 + 0x70) = puVar5;
    *(uchar **)(iVar8 + 0x72)     = puVar7;
    pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70), uStack10, puVar7);
    return;
}

astruct_57 *__stdcall16far pass1_1040_0bfc(astruct_57 *param_1,
                                           ulong       param_2,
                                           ushort      param_3,
                                           ushort      param_4,
                                           ushort      param_5,
                                           uchar      *param_6,
                                           int         param_7,
                                           ushort      param_8)

{
    astruct_715 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcd, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_715 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    *(undefined2 *)param_1            = 0xdb0;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    iVar1->field_0x74                 = 0x1;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same
// address

void __stdcall16far pass1_1040_0e1c(
  astruct_57 *param_1, ushort param_2, ulong param_3, ushort param_4, uchar *param_5, int param_6, ushort param_7)

{
    astruct_716 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c0, param_4);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_716 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = param_3;
    iVar1->field_0x96                 = 0x0;
    iVar1->field_0x98                 = param_2;
    *(int *)param_1                   = (int)s_overflow_on_node__d_1050_11ca + 0x8;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3a, param_7, param_5, param_6);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}

void __stdcall16far pass1_1038_de20(
  ulong param_1, ushort param_2, ushort param_3, int param_4, uchar *param_5, uint param_6, ushort param_7)

{
    code      **ppcVar1;
    int         iVar2;
    uchar      *puVar3;
    undefined2  uVar4;
    undefined   local_12[0x4];
    uint        uStack14;
    uchar      *puStack12;
    undefined4 *puStack10;
    ushort      uStack6;
    int         iStack4;

    iStack4 = 0x644;
    uStack6 = 0x0;
    switch(param_4 + -0x11c)
    {
    case 0x0:
        iStack4 = 0x635;
        uStack6 = 0x3a;
        break;
    case 0x1:
        iStack4 = 0x636;
        uStack6 = 0x3b;
        break;
    case 0x2:
        iStack4 = 0x637;
        uStack6 = 0x3c;
        break;
    case 0x4:
        iStack4 = 0x639;
        uStack6 = 0x3e;
        break;
    case 0x5:
        iStack4 = 0x63a;
        uStack6 = 0x3f;
        break;
    case 0x6:
        iStack4 = 0x63b;
        uStack6 = 0x40;
        break;
    case 0x7:
        iStack4 = 0x640;
        uStack6 = 0x45;
        break;
    case 0x9:
        iStack4 = 0x642;
        uStack6 = 0x47;
        break;
    case 0xa:
        iStack4 = 0x641;
        uStack6 = 0x46;
        break;
    case 0xb:
        iStack4 = 0x63f;
        uStack6 = 0x44;
    }
    if(iStack4 != 0x0)
    {
        uVar4 = 0x1000;
        mem_op_1000_179c(0xb4, param_5, 0x1000);
        puVar3    = (uchar *)((uint)param_5 | param_6);
        uStack14  = param_6;
        puStack12 = param_5;
        if(puVar3 == (uchar *)0x0)
        {
            iVar2  = 0x0;
            puVar3 = (uchar *)0x0;
        }
        else
        {
            uVar4 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar2 = string_1040_8520((astruct_57 *)CONCAT22(param_5, param_6),
                                     *(ushort *)((int)param_1 + 0x6),
                                     0x0,
                                     0x2,
                                     0x634,
                                     iStack4,
                                     puVar3,
                                     param_7);
        }
        puStack10 = (undefined4 *)CONCAT22(puVar3, iVar2);
        if(uStack6 == 0x0)
        {
            ppcVar1 = (code **)((int)*puStack10 + 0x74);
            (**ppcVar1)(uVar4, iVar2, puVar3);
        }
        else
        {
            pass1_1008_941a((ushort *)CONCAT22(param_7, local_12), 0x1, uStack6);
            ppcVar1 = (code **)((int)*puStack10 + 0x6c);
            (**ppcVar1)(0x1008, (char)puStack10, (int)((ulong)puStack10 >> 0x10), local_12, param_7);
        }
    }
    return;
}

void __stdcall16far pass1_1038_df86(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    char       *pcVar1;
    code      **ppcVar2;
    BOOL16      BVar3;
    uint        uVar4;
    ushort      uVar5;
    uchar      *puVar6;
    undefined2  uVar7;
    ushort      uVar8;
    undefined2  uVar9;
    undefined   uVar10;
    ushort     *puVar11;
    char       *pcVar12;
    astruct_57 *paVar13;
    undefined4 *puStack22;

    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    uVar5   = (ushort)((ulong)puVar11 >> 0x10);
    pcVar1  = *(char **)((int)puVar11 + 0x68);
    uVar9   = (undefined2)(param_1 >> 0x10);
    uVar8   = (ushort)param_1;
    BVar3   = pass1_1010_041a();
    if(BVar3 != 0x0)
    {
        pass1_1010_038e(*(ulong *)(uVar8 + 0x92), 0x1, param_4);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar8 + 0x8), 0x1e, uVar5, uVar8, 0x1010, param_4);
        return;
    }
    pcVar12 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    puVar6  = (uchar *)((ulong)pcVar12 >> 0x10);
    uVar4   = (uint)pcVar12;
    uVar10  = 0x0;
    mem_op_1000_179c(0xb4, puVar6, 0x1000);
    if(((uint)puVar6 | uVar4) == 0x0)
    {
        uVar9 = 0x0;
        uVar7 = 0x0;
    }
    else
    {
        uVar10  = 0x40;
        paVar13 = pass1_1040_8478(
          (astruct_57 *)CONCAT22(puVar6, uVar4), 0x20, pcVar1, pcVar12, *(ushort *)(uVar8 + 0x6), (uint)puVar6 | uVar4);
        uVar7 = (undefined2)((ulong)paVar13 >> 0x10);
        uVar9 = SUB42(paVar13, 0x0);
    }
    puStack22 = (undefined4 *)CONCAT22(uVar7, uVar9);
    ppcVar2   = (code **)((int)*puStack22 + 0x74);
    (**ppcVar2)(uVar10, uVar9, uVar7);
    return;
}

astruct_57 *__stdcall16far
pass1_1038_e140(astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5)

{
    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfc2, param_5);
    *(undefined2 *)param_1              = 0xe264;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return param_1;
}

astruct_57 *__stdcall16far pass1_1038_e2d0(astruct_57 *param_1, ushort param_2)

{
    undefined2 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c3, param_2);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x8e) = 0x0;
    *(undefined2 *)param_1               = 0xe62e;
    *(undefined2 *)((int)param_1 + 0x2)  = (int)&PTR_LOOP_1050_1038;
    return param_1;
}

void __stdcall16far pass1_1038_e69a(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_713 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcb, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_713 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    *(undefined2 *)param_1            = 0xe92e;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x43, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}

astruct_57 *__stdcall16far pass1_1038_e99a(astruct_57 *param_1,
                                           ulong       param_2,
                                           undefined2  param_3,
                                           undefined2  param_4,
                                           ushort      param_5,
                                           uchar      *param_6,
                                           ushort      param_7)

{
    astruct_434 *iVar1;
    int          unaff_DI;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb9, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_434 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    *(undefined2 *)param_1            = 0xeb32;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_7, param_6, unaff_DI);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return param_1;
}

astruct_57 *__stdcall16far pass1_1038_eb9e(astruct_57 *param_1, ushort param_2)

{
    undefined2 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c7, param_2);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x8e) = 0x0;
    *(undefined2 *)param_1               = 0xee6e;
    *(undefined2 *)((int)param_1 + 0x2)  = (int)&PTR_LOOP_1050_1038;
    return param_1;
}

astruct_57 *__stdcall16far
pass1_1038_cad8(astruct_57 *param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5)

{
    astruct_709 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1cb, param_2);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_709 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    *(undefined2 *)param_1            = 0xcc9a;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2c, param_5, param_3, param_4);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    iVar1->field_0x74                 = 0x0;
    return param_1;
}


void __stdcall16far pass1_1038_cd06(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_710 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcc, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_710 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    *(undefined2 *)param_1            = 0xcf00;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x42, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}


void __stdcall16far make_proc_inst_1038_cf6c(ushort *param_1, uchar *param_2, LPVOID param_3)

{
    LPVOID     pvVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                        = (int)param_1;
    *param_1                     = 0x389a;
    *(undefined2 *)(iVar2 + 0x2) = 0x1008;
    *(undefined4 *)(iVar2 + 0x4) = 0x0;
    *(undefined2 *)(iVar2 + 0x8) = 0x0;
    *param_1                     = 0xd23e;
    *(undefined2 *)(iVar2 + 0x2) = (int)&PTR_LOOP_1050_1038;
    _PTR_LOOP_1050_5bc8          = param_1;
    pvVar1                       = MakeProcInstance16(param_3, (HANDLE16)PTR_LOOP_1050_038c);
    *(LPVOID *)(iVar2 + 0x4)     = pvVar1;
    *(uchar **)(iVar2 + 0x6)     = param_2;
    PTR_LOOP_1050_5bcc = (undefined *)MakeProcInstance16((LPVOID)s_tile2_bmp_1050_1538, (HANDLE16)PTR_LOOP_1050_038c);
    PTR_LOOP_1050_5bce = param_2;
    return;
}


void __stdcall16far free_proc_inst_1038_cfda(ushort *param_1, LPVOID param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *param_1                     = 0xd23e;
    *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    FreeProcInstance16(param_2);
    FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
    *(undefined4 *)(iVar1 + 0x4) = 0x0;
    *param_1                     = 0x389a;
    *(undefined2 *)(iVar1 + 0x2) = 0x1008;
    return;
}


astruct_57 *__stdcall16far pass1_1038_d242(astruct_57 *param_1, ushort param_2)

{
    undefined2 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x13e, param_2);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined2 *)param_1               = 0xd6ea;
    *(undefined2 *)((int)param_1 + 0x2)  = (int)&PTR_LOOP_1050_1038;
    *(undefined2 *)((int)param_1 + 0x74) = 0x1;
    return param_1;
}


astruct_57 *__stdcall16far
pass1_1038_d756(astruct_57 *param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5)

{
    code       **ppcVar1;
    astruct_711 *iVar2;
    undefined2   uVar2;
    ushort      *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x11b, param_2);
    uVar2                      = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                      = (astruct_711 *)param_1;
    iVar2->field_0x8e          = 0x0;
    iVar2->field_0x90          = 0x0;
    iVar2->field_0x92          = (undefined4 *)0x0;
    iVar2->field_0x96          = 0x0;
    *(undefined2 *)param_1     = 0xe0d4;
    iVar2->field_0x2           = (int)&PTR_LOOP_1050_1038;
    puVar3                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    *(int *)&iVar2->field_0x92 = (int)puVar3;
    *(undefined2 *)((int)&iVar2->field_0x92 + 0x2) = (int)((ulong)puVar3 >> 0x10);
    ppcVar1                                        = (code **)((int)*iVar2->field_0x92 + 0x4);
    (**ppcVar1)();
    return param_1;
}


void __stdcall16far pass1_1038_b772(astruct_57 *param_1, uchar *param_2, int param_3, ushort param_4, ushort param_5)

{
    uchar       *puVar1;
    astruct_705 *iVar2;
    undefined2   uVar2;
    ushort      *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x9a, 0x0, 0xfbf, param_5);
    uVar2                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                             = (astruct_705 *)param_1;
    *(undefined4 *)&iVar2->field_0x8e = 0x0;
    *(undefined4 *)&iVar2->field_0x92 = 0x0;
    iVar2->field_0x96                 = 0x1;
    iVar2->field_0x98                 = 0x0;
    *(undefined2 *)param_1            = 0xbd70;
    iVar2->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x36, param_4, param_2, param_3);
    puVar1                            = (uchar *)((ulong)puVar3 >> 0x10);
    iVar2->field_0x8e                 = (int)puVar3;
    iVar2->field_0x90                 = puVar1;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, param_4, puVar1, param_3);
    iVar2->field_0x92                 = (int)puVar3;
    iVar2->field_0x94                 = (int)((ulong)puVar3 >> 0x10);
    return;
}


void __stdcall16far pass1_1038_bca8(ulong param_1)

{
    uint        uVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    undefined4 *puVar4;
    undefined4 *puVar5;
    uchar      *extraout_DX;
    uchar      *puVar6;
    uchar      *extraout_DX_00;
    uchar      *puVar7;
    int         iVar8;
    undefined2  uVar9;

    uVar9   = (undefined2)(param_1 >> 0x10);
    iVar8   = (int)param_1;
    uVar3   = *(undefined4 *)(iVar8 + 0x8e);
    puVar5  = (undefined4 *)*(undefined4 *)((int)uVar3 + 0xa);
    ppcVar2 = (code **)((int)*puVar5 + 0x14);
    (**ppcVar2)();
    puVar4 = (undefined4 *)puVar5;
    puVar6 = extraout_DX;
    if(*(long *)(iVar8 + 0x70) != 0x0)
    {
        puVar4 = (undefined4 *)*(uint *)(iVar8 + 0x70);
        uVar1  = *(uint *)(iVar8 + 0x72);
        puVar6 = (uchar *)(uVar1 | (uint)puVar4);
        if(puVar6 != (uchar *)0x0)
        {
            ppcVar2 = (code **)*puVar4;
            (**ppcVar2)();
            puVar6 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar6, 0x1000);
    puVar7 = (uchar *)((uint)puVar6 | (uint)puVar4);
    if(puVar7 == (uchar *)0x0)
    {
        puVar4 = (undefined4 *)0x0;
        puVar7 = (uchar *)0x0;
    }
    else
    {
        struct_1008_4c58((ushort *)CONCAT22(puVar6, puVar4));
    }
    *(undefined2 *)(iVar8 + 0x70) = puVar4;
    *(uchar **)(iVar8 + 0x72)     = puVar7;
    pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70), (ulong)puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10, puVar7);
    return;
}


void __stdcall16far pass1_1038_bddc(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_706 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x176, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_706 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x94                 = 0x0;
    iVar1->field_0x96                 = 0x0;
    iVar1->field_0x98                 = 0x0;
    iVar1->field_0x9a                 = 0x0;
    iVar1->field_0x9c                 = 0x0;
    *(undefined2 *)param_1            = 0xc436;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}


void __stdcall16far pass1_1038_c4a2(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_708 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x17c, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_708 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x96                 = 0x0;
    *(undefined2 *)param_1            = 0xc74c;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}


astruct_57 *__stdcall16far pass1_1038_c7b8(
  astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, uchar *param_6, ushort param_7)

{
    astruct_435 *iVar1;
    int          unaff_DI;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb8, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_435 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    *(undefined2 *)param_1            = 0xca6c;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x5, param_7, param_6, unaff_DI);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return param_1;
}


astruct_57 *__stdcall16far
pass1_1038_9f76(astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5)

{
    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfba, param_5);
    *(undefined2 *)param_1              = 0xa0b6;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return param_1;
}


ushort *__stdcall16far pass1_1038_a122(int param_1, ushort param_2, ushort param_3, ulong param_4, ulong param_5)

{
    get_sys_metrics_1040_7728(
      (astruct_57 *)CONCAT22(param_2, param_1), param_3, param_4, (ushort)param_5, (ushort)(param_5 >> 0x10));
    *(undefined2 *)(param_1 + 0x8e)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0xa2d0;
    *(undefined2 *)(param_1 + 0x2)        = (int)&PTR_LOOP_1050_1038;
    return (ushort *)CONCAT22(param_2, param_1);
}


astruct_57 *__stdcall16far pass1_1038_ab82(astruct_57 *param_1, ushort param_2)

{
    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfd3, param_2);
    *(undefined2 *)param_1              = 0xad72;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return param_1;
}


void __stdcall16far pass1_1038_9144(ushort *param_1, ushort param_2, ushort param_3)

{
    undefined4   uVar1;
    uint         uVar2;
    uchar       *in_DX;
    uchar       *puVar3;
    uchar       *puVar4;
    int          iVar5;
    int          iVar6;
    int          unaff_DI;
    undefined2   uVar7;
    undefined2   uVar8;
    ushort      *puVar9;
    int         *piStack8;
    astruct_432 *iVar8;

    struct_1040_b082((astruct_57 *)param_1, CONCAT22(param_2, 0xfaa));
    uVar7                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar5                         = (int)param_1;
    *(undefined2 *)(iVar5 + 0x94) = 0x0;
    *(undefined2 *)(iVar5 + 0x96) = 0x0;
    *(undefined4 *)(iVar5 + 0x98) = 0x0;
    *param_1                      = 0x99a2;
    *(undefined2 *)(iVar5 + 0x2)  = (int)&PTR_LOOP_1050_1038;
    *(undefined2 *)(iVar5 + 0x8a) = 0x27;
    puVar9                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x28, param_3, in_DX, unaff_DI);
    puVar3                        = (uchar *)((ulong)puVar9 >> 0x10);
    uVar2                         = (uint)puVar9;
    *(uint *)(iVar5 + 0x98)       = uVar2;
    *(uchar **)(iVar5 + 0x9a)     = puVar3;
    mem_op_1000_179c(0x18, puVar3, 0x1000);
    puVar4 = (uchar *)((uint)puVar3 | uVar2);
    if(puVar4 == (uchar *)0x0)
    {
        *(undefined4 *)(iVar5 + 0x90) = 0x0;
    }
    else
    {
        struct_1040_a598((ushort *)CONCAT22(puVar3, uVar2));
        *(uint *)(iVar5 + 0x90)   = uVar2;
        *(uchar **)(iVar5 + 0x92) = puVar4;
    }
    *(undefined2 *)*(undefined4 *)(iVar5 + 0x90) = 0x11;
    iVar6                                        = **(int **)(iVar5 + 0x90);
    uVar2                                        = iVar6 * 0xa + 0x2;
    mem_op_1000_179c(uVar2, puVar4, 0x1000);
    piStack8 = (int *)CONCAT22(puVar4, uVar2);
    if(((uint)puVar4 | uVar2) == 0x0)
    {
        uVar1                             = *(undefined4 *)(iVar5 + 0x90);
        *(undefined4 *)((int)uVar1 + 0x2) = 0x0;
    }
    else
    {
        *piStack8 = iVar6;
        pass1_1000_5586((uchar *)0xa564, (ushort)&PTR_LOOP_1050_1040, iVar6, 0xa, uVar2 + 0x2, (ushort)puVar4);
        uVar1                    = *(undefined4 *)(iVar5 + 0x90);
        uVar8                    = (undefined2)((ulong)uVar1 >> 0x10);
        iVar6                    = (int)uVar1;
        *(int *)(iVar6 + 0x2)    = uVar2 + 0x2;
        *(uchar **)(iVar6 + 0x4) = puVar4;
    }
    uVar1                              = *(undefined4 *)(iVar5 + 0x90);
    *(undefined2 *)((int)uVar1 + 0xa)  = 0x18;
    uVar1                              = *(undefined4 *)(iVar5 + 0x90);
    *(undefined2 *)((int)uVar1 + 0x12) = *(undefined2 *)(iVar5 + 0xa);
    return;
}


void __stdcall16far pass1_1038_78e2(ulong *param_1, uchar *param_2)

{
    uint         uVar1;
    uchar       *puVar2;
    uchar       *extraout_DX;
    undefined2   extraout_DX_00;
    undefined2   uVar3;
    astruct_431 *iVar4;
    undefined2   uVar4;

    uVar4                            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                            = (astruct_431 *)param_1;
    uVar1                            = 0x0;
    *param_1                         = 0x0;
    *(undefined4 *)&iVar4->field_0x4 = 0x0;
    _PTR_LOOP_1050_5a64              = param_1;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    puVar2 = (uchar *)((uint)param_2 | uVar1);
    if(puVar2 == (uchar *)0x0)
    {
        *param_1 = 0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(param_2, uVar1));
        *(uint *)param_1 = uVar1;
        iVar4->field_0x2 = extraout_DX;
        puVar2           = extraout_DX;
    }
    mem_op_1000_179c(0xc, puVar2, 0x1000);
    if(((uint)puVar2 | uVar1) == 0x0)
    {
        uVar1 = 0x0;
        uVar3 = 0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2, uVar1));
        uVar3 = extraout_DX_00;
    }
    iVar4->field_0x4 = uVar1;
    iVar4->field_0x6 = uVar3;
    return;
}


void __stdcall16far pass1_1038_79b2(ulong param_1, ulong param_2, uint param_3, uchar *param_4)

{
    code     **ppcVar1;
    uint       uVar2;
    undefined2 uVar3;
    undefined2 uVar4;

    uVar4 = 0x1000;
    mem_op_1000_179c(0x14, param_4, 0x1000);
    uVar2 = (uint)param_4 | param_3;
    if(uVar2 == 0x0)
    {
        param_3 = 0x0;
        uVar2   = 0x0;
    }
    else
    {
        uVar4 = 0x1030;
        pass1_1030_aefa((ushort *)CONCAT22(param_4, param_3), param_2);
    }
    uVar3   = (undefined2)(param_1 >> 0x10);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x4);
    (**ppcVar1)(uVar4, *(undefined4 *)((int)param_1 + 0x4), param_3, uVar2);
    return;
}


astruct_57 *__stdcall16far
pass1_1038_7d10(astruct_57 *param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5)

{
    astruct_703 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1853));
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_703 *)param_1;
    *(undefined4 *)&iVar1->field_0x94 = 0x0;
    *(undefined2 *)param_1            = 0x8876;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x40, param_5, param_3, param_4);
    iVar1->field_0x94                 = (int)puVar2;
    iVar1->field_0x96                 = (int)((ulong)puVar2 >> 0x10);
    return param_1;
}


ulong __stdcall16far pass1_1038_801a(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    ushort     uVar1;
    ushort     uVar2;
    undefined2 uVar3;
    ushort    *puVar4;
    char      *pcVar5;
    ulong      uVar6;

    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_4, param_2, param_3);
    uVar3  = (undefined2)(param_1 >> 0x10);
    uVar2  = (ushort)param_1;
    pcVar5 = (char *)pass1_1008_b340(*(ulong *)(uVar2 + 0x94));
    uVar1  = (uint)((ulong)pcVar5 >> 0x10) | (uint)pcVar5;
    uVar6  = (ulong)pcVar5 & 0xffff | (ulong)uVar1 << 0x10;
    if(pcVar5 != (char *)0x0)
    {
        pass1_1010_3770((ulong)puVar4, pcVar5, uVar1);
        uVar6 = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar2 + 0x6), 0x3, uVar1, uVar2, 0x1010, param_4);
    }
    return uVar6;
}


void __stdcall16far pass1_1038_6b88(
  ushort param_1, ushort param_2, ushort *param_3, ulong *param_4, uchar *param_5, int param_6, ushort param_7)

{
    ulong     *puVar1;
    undefined2 uVar2;
    ulong      local_12[0x2];
    long       lStack10;
    ushort    *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    uVar2    = (undefined2)((ulong)puStack6 >> 0x10);
    lStack10 = *(long *)((int)puStack6 + 0x20);
    puVar1   = local_12;
    pass1_1030_64ce(param_7, puVar1, uVar2, _PTR_LOOP_1050_5740, param_3, lStack10, (ulong *)CONCAT22(param_7, puVar1));
    *param_4 = *puVar1;
    return;
}


void __stdcall16far pass1_1038_354a(ulong param_1, uint param_2, uchar *param_3)

{
    uint         uVar1;
    astruct_424 *iVar1;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (astruct_424 *)param_1;
    if(*(long *)&iVar1->field_0x21a == 0x0)
    {
        mem_op_1000_179c(0xa, param_3, 0x1000);
        uVar1 = (uint)param_3 | param_2;
        if(uVar1 == 0x0)
        {
            *(undefined4 *)&iVar1->field_0x21a = 0x0;
        }
        else
        {
            pass1_1030_9ecc((ulong *)CONCAT22(param_3, param_2), param_1);
            *(uint *)&iVar1->field_0x21a = param_2;
            iVar1->field_0x21c           = uVar1;
        }
    }
    pass1_1030_9ef2(*(ulong **)&iVar1->field_0x21a);
    return;
}


void __stdcall16far pass1_1038_35a8(ulong param_1, ushort param_2, uint param_3, uchar *param_4)

{
    uint         uVar1;
    astruct_425 *iVar3;
    undefined2   uVar2;
    ushort       unaff_SS;
    uchar        in_AF;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_425 *)param_1;
    if(*(long *)&iVar3->field_0x21a == 0x0)
    {
        mem_op_1000_179c(0xa, param_4, 0x1000);
        uVar1 = (uint)param_4 | param_3;
        if(uVar1 == 0x0)
        {
            *(undefined4 *)&iVar3->field_0x21a = 0x0;
        }
        else
        {
            pass1_1030_9ecc((ulong *)CONCAT22(param_4, param_3), param_1);
            *(uint *)&iVar3->field_0x21a = param_3;
            iVar3->field_0x21c           = uVar1;
        }
    }
    pass1_1030_9f40(*(ulong *)&iVar3->field_0x21a, param_2, unaff_SS, in_AF);
    return;
}


void __stdcall16far pass1_1038_2944(ulong param_1, uint param_2, uchar *param_3)

{
    undefined4 *puVar1;
    undefined4 *puVar2;
    undefined4 *puVar3;
    int         iVar4;
    undefined4 *puVar5;
    undefined2  uVar6;
    undefined2 *puStack10;

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
        *puStack10                     = 0x29fe;
        *(undefined2 *)(param_2 + 0x2) = (int)&PTR_LOOP_1050_1038;
    }
    return;
}


void __stdcall16far pass1_1038_2b9a(ulong param_1, astruct_422 *param_2, uchar *param_3)

{
    undefined4  *puVar1;
    undefined4  *puVar2;
    int          iVar3;
    astruct_421 *iVar5;
    undefined4  *puVar4;
    undefined4  *puVar5;
    undefined2   uVar6;
    undefined2  *puStack10;

    mem_op_1000_179c(0x118, param_3, 0x1000);
    puStack10 = (undefined2 *)CONCAT22(param_3, param_2);
    iVar5     = (astruct_421 *)param_1;
    uVar6     = (undefined2)(param_1 >> 0x10);
    if(((uint)param_3 | (uint)param_2) != 0x0)
    {
        *puStack10         = 0x389a;
        param_2->field_0x2 = 0x1008;
        param_2->field_0x4 = iVar5->field_0x4;
        puVar4             = &iVar5->field_0x8;
        puVar5             = &param_2->field_0x8;
        for(iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1)
        {
            puVar2  = puVar5;
            puVar5  = puVar5 + 0x1;
            puVar1  = puVar4;
            puVar4  = puVar4 + 0x1;
            *puVar2 = *puVar1;
        }
        *puStack10           = 0x6ad2;
        param_2->field_0x2   = (int)&USHORT_1050_1028;
        param_2->field_0x108 = iVar5->field_0x108;
        param_2->field_0x10c = iVar5->field_0x10c;
        param_2->field_0x110 = iVar5->field_0x110;
        param_2->field_0x114 = iVar5->field_0x114;
        *puStack10           = 0x309a;
        param_2->field_0x2   = (int)&PTR_LOOP_1050_1038;
    }
    iVar5->field_0x114 = 0x0;
    iVar5->field_0x110 = 0x0;
    return;
}


void __stdcall16far pass1_1038_30aa(ushort *param_1, ushort param_2)

{
    uint        *puVar1;
    uchar       *puVar2;
    uchar       *puVar3;
    uint         uVar4;
    astruct_423 *iVar5;
    undefined2   uVar5;
    ushort      *puVar6;

    puVar6                             = struct_1030_17ce(param_1, 0x0, 0x0);
    puVar2                             = (uchar *)((ulong)puVar6 >> 0x10);
    uVar5                              = (undefined2)((ulong)param_1 >> 0x10);
    iVar5                              = (astruct_423 *)param_1;
    iVar5->field_0x10                  = 0x0;
    iVar5->field_0x14                  = 0x0;
    iVar5->field_0x18                  = 0x258;
    iVar5->field_0x1a                  = 0x258;
    iVar5->field_0x1c                  = 0x0;
    iVar5->field_0x1e                  = 0x0;
    iVar5->field_0x22                  = 0x0;
    iVar5->field_0x24                  = 0x32;
    *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
    iVar5->field_0x1fa                 = 0x0;
    iVar5->field_0x1fe                 = 0x0;
    iVar5->field_0x200                 = 0x8000001;
    iVar5->field_0x204                 = 0x0;
    iVar5->field_0x206                 = 0x0;
    iVar5->field_0x208                 = 0x1;
    iVar5->field_0x20a                 = 0x0;
    iVar5->field_0x20c                 = 0x0;
    iVar5->field_0x20e                 = 0x0;
    iVar5->field_0x210                 = 0x0;
    iVar5->field_0x214                 = 0x0;
    iVar5->field_0x216                 = 0x0;
    iVar5->field_0x21a                 = 0x0;
    *param_1                           = 0x6504;
    iVar5->field_0x2                   = (int)&PTR_LOOP_1050_1038;
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x26), (WNDCLASS16 *)0x0, 0x94);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0xba), (WNDCLASS16 *)0x0, 0x94);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x14e), (WNDCLASS16 *)0x0, 0x54);
    puVar1 = pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x1a2), (WNDCLASS16 *)0x0, 0x54);
    mem_op_1000_179c(0x1b0, puVar2, 0x1000);
    puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
    if(puVar3 == (uchar *)0x0)
    {
        *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
    }
    else
    {
        pass1_1030_314c((ushort *)CONCAT22(puVar2, puVar1), iVar5->field_0x4, puVar3, param_2);
        iVar5->field_0x1f6 = puVar1;
        iVar5->field_0x1f8 = puVar3;
    }
    mem_op_1000_179c(0x1e, puVar3, 0x1000);
    uVar4 = (uint)puVar3 | (uint)puVar1;
    if(uVar4 == 0x0)
    {
        puVar1 = (uint *)0x0;
        uVar4  = 0x0;
    }
    else
    {
        struct_1020_c444((astruct_75 *)CONCAT22(puVar3, puVar1), 0x64, 0xc8);
    }
    iVar5->field_0xc = puVar1;
    iVar5->field_0xe = uVar4;
    return;
}


void __stdcall16far pass1_1038_3222(
  ushort *param_1, ulong param_2, ulong param_3, uint param_4, uchar *param_5, uchar param_6, uchar *param_7)

{
    uint        *puVar1;
    uchar       *puVar2;
    uint         uVar3;
    uint         uVar4;
    astruct_363 *iVar5;
    undefined2   uVar5;
    ushort      *puVar6;
    uchar        local_16[0x14];

    puVar6                             = pass1_1030_183c(param_1, 0x0, 0x0, 0x4000000, param_3, param_4, param_5);
    puVar2                             = (uchar *)((ulong)puVar6 >> 0x10);
    uVar5                              = (undefined2)((ulong)param_1 >> 0x10);
    iVar5                              = (astruct_363 *)param_1;
    iVar5->field_0x10                  = param_2;
    iVar5->field_0x14                  = 0x0;
    iVar5->field_0x18                  = 0x258;
    iVar5->field_0x1a                  = 0x258;
    iVar5->field_0x1c                  = 0x0;
    iVar5->field_0x1e                  = 0x0;
    iVar5->field_0x22                  = 0x0;
    iVar5->field_0x24                  = 0x32;
    *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
    *(undefined4 *)&iVar5->field_0x1fa = 0x0;
    iVar5->field_0x1fe                 = 0x0;
    iVar5->field_0x200                 = 0x8000001;
    iVar5->field_0x204                 = 0x0;
    iVar5->field_0x206                 = 0x0;
    iVar5->field_0x208                 = 0x1;
    iVar5->field_0x20a                 = 0x0;
    iVar5->field_0x20c                 = 0x0;
    iVar5->field_0x20e                 = 0x0;
    iVar5->field_0x210                 = 0x0;
    iVar5->field_0x214                 = 0x0;
    iVar5->field_0x216                 = 0x0;
    iVar5->field_0x21a                 = 0x0;
    *param_1                           = 0x6504;
    iVar5->field_0x2                   = (int)&PTR_LOOP_1050_1038;
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x26), (WNDCLASS16 *)0x0, 0x94);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0xba), (WNDCLASS16 *)0x0, 0x94);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x14e), (WNDCLASS16 *)0x0, 0x54);
    puVar1 = pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x1a2), (WNDCLASS16 *)0x0, 0x54);
    mem_op_1000_179c(0x1b0, puVar2, 0x1000);
    uVar3 = (uint)puVar2 | (uint)puVar1;
    if(uVar3 == 0x0)
    {
        *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
    }
    else
    {
        pass1_1030_314c((ushort *)CONCAT22(puVar2, puVar1), *(ulong *)&iVar5->field_0x4, uVar3, param_7);
        iVar5->field_0x1f6 = puVar1;
        iVar5->field_0x1f8 = uVar3;
    }
    puVar2 = (uchar *)(iVar5->field_0x6 & 0xff);
    sys_1000_3f9c(local_16,
                  param_7,
                  0x5a1a,
                  (ushort)&USHORT_1050_1050,
                  (ushort) * (undefined4 *)&iVar5->field_0x4,
                  &stack0xfffe,
                  uVar5,
                  0x1000,
                  param_7,
                  param_6);
    uVar3              = str_op_1008_60e8((char *)CONCAT22(param_7, local_16), (ushort)puVar2);
    iVar5->field_0x1fa = uVar3;
    iVar5->field_0x1fc = puVar2;
    mem_op_1000_179c(0x1e, puVar2, 0x1000);
    uVar4 = (uint)puVar2 | uVar3;
    if(uVar4 == 0x0)
    {
        *(undefined4 *)&iVar5->field_0xc = 0x0;
    }
    else
    {
        struct_1020_c444((astruct_75 *)CONCAT22(puVar2, uVar3), 0x64, 0xc8);
        iVar5->field_0xc = uVar3;
        iVar5->field_0xe = uVar4;
    }
    return;
}


void __stdcall16far pass1_1038_19a0(ulong param_1, ulong *param_2, ulong param_3, ushort param_4, uchar param_5)

{
    code **ppcVar1;
    ulong  uVar2;
    uint   uVar3;
    uint   uVar4;
    uchar *puVar5;
    uint   extraout_DX;
    code **ppcVar6;
    ulong *puVar7;
    ulong *puStack10;

    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
    puVar5 = (uchar *)((ulong)puVar7 >> 0x10);
    uVar3  = (uint)puVar7;
    pass1_1038_4d6e(param_3, puVar7, uVar3, puVar5);
    puStack10 = (ulong *)CONCAT22(puVar5, uVar3);
    uVar2     = *puStack10;
    ppcVar6   = (code **)uVar2;
    ppcVar1   = ppcVar6 + 0x8;
    uVar4     = uVar3;
    (**ppcVar1)(0x1008, uVar3, puVar5);
    if((extraout_DX | uVar4) == 0x0)
    {
        vsprintf_op_1030_840a((ulong)s_mineToSmelter__no_mines_1050_59df, 0x1030, param_4, 0x0);
        if(puStack10 != (ulong *)0x0)
        {
            ppcVar1 = ppcVar6;
            (**ppcVar1)(0x1030, uVar3, (char)puVar5, 0x1);
            return;
        }
    }
    else
    {
        pass1_1038_16f2(param_1,
                        puStack10,
                        param_2,
                        extraout_DX | uVar4,
                        (ushort)ppcVar6,
                        (ushort)(uVar2 >> 0x10),
                        0x1008,
                        param_4,
                        param_5);
        if(puStack10 != (ulong *)0x0)
        {
            ppcVar1 = (code **)*puStack10;
            (**ppcVar1)(0x1008, uVar3, (char)puVar5, 0x1);
        }
    }
    return;
}


void __stdcall16far
pass1_1038_008e(ushort param_1, ushort param_2, ulong param_3, uchar *param_4, int param_5, ushort param_6)

{
    int        iVar1;
    undefined4 uVar2;
    uint       uVar3;
    uint       uVar4;
    uint       uVar5;
    uchar     *puVar7;
    uchar     *puVar8;
    int        iVar9;
    undefined2 uVar10;
    ushort    *puVar11;
    ushort    *puVar12;
    int        iStack32;
    int        iStack12;
    undefined4 uVar6;

    uVar10 = (undefined2)(param_3 >> 0x10);
    iVar9  = (int)param_3;
    if(*(long *)(iVar9 + 0x4) != 0x4000001)
    {
        return;
    }
    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2c, param_6, param_4, param_5);
    puVar7  = (uchar *)((ulong)puVar11 >> 0x10);
    uVar3   = (uint)puVar11;
    puVar8  = puVar7;
    uVar4   = uVar3;
    pass1_1008_612e(0x1, 0x64, uVar3);
    iStack12 = 0x0;
    iVar1    = *(int *)(uVar3 + 0xa);
    if(iVar1 == 0x1)
    {
        iStack12 = 0x15;
    }
    else
    {
        if(iVar1 != 0x2)
        {
            if(iVar1 == 0x3)
            {
                iStack12 = 0x16;
            }
            else
            {
                if(iVar1 == 0x4)
                {
                    if((int)uVar4 < 0x32)
                    {
                        iStack12 = 0x17;
                    }
                    else
                    {
                        iStack12 = 0x18;
                    }
                }
                else
                {
                    if(iVar1 == 0x5)
                    {
                        iStack12 = 0x19;
                    }
                }
            }
        }
    }
    if(iStack12 != 0x0)
    {
        puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar8, param_5);
        puVar8  = (uchar *)((ulong)puVar12 >> 0x10);
        pass1_1010_043a((ulong)puVar12 & 0xffff | ZEXT24(puVar8) << 0x10, *(long *)(iVar9 + 0x4), iStack12, param_6);
    }
    pass1_1008_eb74(puVar11, 0x0, puVar8, param_5, param_6);
    if(((*(uint *)(uVar3 + 0xe) | *(uint *)(uVar3 + 0xc)) == 0x0) && (*(int *)(iVar9 + 0x18) < 0xc9))
    {
        uVar2 = *_PTR_LOOP_1050_65e2;
        uVar4 = (uint)uVar2;
        uVar6 = uVar2;
        pass1_1008_612e(0x0, 0x8, uVar4);
        uVar5                 = (uint)uVar6;
        iStack32              = (int)((ulong)uVar2 >> 0x10);
        *(int *)(uVar3 + 0xc) = uVar5 + uVar4 + 0x1e;
        *(int *)(uVar3 + 0xe)
          = ((int)uVar5 >> 0xf) + iStack32 + (uint)CARRY2(uVar5, uVar4) + (uint)(0xffe1 < uVar5 + uVar4);
    }
    return;
}


astruct_100 *__stdcall16far pass1_1038_0ba6(astruct_100 *param_1, int param_2, ushort param_3, uchar param_4)

{
    uchar       *puVar1;
    astruct_701 *iVar2;
    undefined2   uVar2;
    astruct_100 *paVar3;
    ushort      *puVar4;

    paVar3                             = struct_op_1028_d1dc(param_3, param_4, param_1, 0x270f);
    puVar1                             = (uchar *)((ulong)paVar3 >> 0x10);
    uVar2                              = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                              = (astruct_701 *)param_1;
    *(undefined4 *)&iVar2->field_0x108 = 0x0;
    param_1->field_0x0                 = 0x1c2e;
    iVar2->field_0x2                   = (int)&PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x8), s_SCMove_1050_59d8);
    puVar4             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_3, puVar1, param_2);
    iVar2->field_0x108 = (int)puVar4;
    iVar2->field_0x10a = (int)((ulong)puVar4 >> 0x10);
    return param_1;
}


void __stdcall16far pass1_1038_0cf0(ulong param_1, uint param_2, uchar *param_3)

{
    undefined4 *puVar1;
    undefined4 *puVar2;
    undefined4 *puVar3;
    int         iVar4;
    int         iVar5;
    undefined4 *puVar6;
    undefined2  uVar7;
    undefined2 *puStack10;

    mem_op_1000_179c(0x10c, param_3, 0x1000);
    puStack10 = (undefined2 *)CONCAT22(param_3, param_2);
    if(((uint)param_3 | param_2) != 0x0)
    {
        *puStack10                     = 0x389a;
        *(undefined2 *)(param_2 + 0x2) = 0x1008;
        uVar7                          = (undefined2)(param_1 >> 0x10);
        iVar5                          = (int)param_1;
        *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)(iVar5 + 0x4);
        puVar3                         = (undefined4 *)(iVar5 + 0x8);
        puVar6                         = (undefined4 *)(param_2 + 0x8);
        for(iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
        {
            puVar2  = puVar6;
            puVar6  = puVar6 + 0x1;
            puVar1  = puVar3;
            puVar3  = puVar3 + 0x1;
            *puVar2 = *puVar1;
        }
        *puStack10                       = 0x6ad2;
        *(undefined2 *)(param_2 + 0x2)   = (int)&USHORT_1050_1028;
        *(undefined4 *)(param_2 + 0x108) = *(undefined4 *)(iVar5 + 0x108);
        *puStack10                       = 0x1c2e;
        *(undefined2 *)(param_2 + 0x2)   = (int)&PTR_LOOP_1050_1038;
    }
    return;
}


void __stdcall16far pass1_1030_e1f4(ulong param_1, uint param_2, uchar *param_3)

{
    undefined4 *puVar1;
    undefined4 *puVar2;
    undefined4 *puVar3;
    int         iVar4;
    undefined4 *puVar5;
    undefined2  uVar6;
    undefined2 *puStack10;

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
        *puStack10                     = 0xe2ae;
        *(undefined2 *)(param_2 + 0x2) = 0x1030;
    }
    return;
}
