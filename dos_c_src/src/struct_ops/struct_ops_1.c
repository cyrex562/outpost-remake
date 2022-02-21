ushort *__stdcall16far struct_1040_bf3e(ushort *param_1, ushort param_2)

{
    astruct_442 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_442 *)param_1;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    *param_1         = 0x3aa8;
    iVar1->field_0x2 = 0x1008;
    iVar1->field_0x4 = param_2;
    *param_1         = 0x3ab0;
    iVar1->field_0x2 = 0x1008;
    iVar1->field_0x6 = 0x0;
    *param_1         = 0xc53e;
    iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
    return param_1;
}


void __stdcall16far pass1_1040_b7ee(astruct_57 *param_1, long param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;
    undefined2 uVar3;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined4 *)(iVar1 + 0x94) = 0x0;
    *(undefined4 *)(iVar1 + 0x98) = 0x0;
    *(undefined4 *)(iVar1 + 0xb0) = 0x0;
    *(undefined2 *)(iVar1 + 0xb4) = 0x0;
    *(undefined2 *)(iVar1 + 0xb6) = 0x0;
    *(undefined2 *)param_1        = 0xbeba;
    *(undefined2 *)(iVar1 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    if(param_2 != 0x0)
    {
        uVar3                         = (undefined2)((ulong)param_2 >> 0x10);
        *(undefined4 *)(iVar1 + 0xb0) = *(undefined4 *)((int)param_2 + 0x6);
        *(undefined2 *)(iVar1 + 0xb4) = *(undefined2 *)((int)param_2 + 0x14);
    }
    return;
}


void __stdcall16far pass1_1040_a640(astruct_57 *param_1, ulong param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;

    struct_1040_b082(param_1, CONCAT22(param_3, 0x1f1));
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(ulong *)(iVar1 + 0x94)      = param_2;
    *(undefined2 *)(iVar1 + 0x98) = 0x0;
    *(undefined2 *)(iVar1 + 0xea) = 0x0;
    *(undefined2 *)param_1        = 0xac08;
    *(undefined2 *)(iVar1 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    return;
}


void __stdcall16far struct_1040_a598(ushort *param_1)

{
    astruct_259 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_259 *)param_1;
    *param_1          = 0x0;
    iVar1->field_0x2  = 0x0;
    iVar1->field_0x6  = 0x0;
    iVar1->field_0xa  = 0x0;
    iVar1->field_0xc  = 0x0;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x14 = 0x0;
    iVar1->field_0x16 = 0x0;
    return;
}


void __stdcall16far pass1_1040_a564(ulong *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0x0;
    *(undefined2 *)((int)param_1 + 0x4) = 0x0;
    *(undefined4 *)((int)param_1 + 0x6) = 0x0;
    return;
}


void __stdcall16far pass1_1040_9824(ulong *param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *param_1                      = 0x0;
    *(undefined2 *)(iVar1 + 0x4)  = 0x0;
    *(undefined4 *)(iVar1 + 0x56) = 0x0;
    *(undefined2 *)(iVar1 + 0x5a) = 0x0;
    *(undefined2 *)(iVar1 + 0x5c) = 0x0;
    *(undefined *)(iVar1 + 0x6)   = 0x0;
    return;
}


void __stdcall16far pass1_1040_4e94(astruct_57 *param_1, long param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;
    undefined2 uVar3;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined4 *)(iVar1 + 0x94) = 0x0;
    *(undefined4 *)(iVar1 + 0x98) = 0x0;
    *(undefined4 *)(iVar1 + 0xb0) = 0x0;
    *(undefined2 *)(iVar1 + 0xb4) = 0x0;
    *(undefined2 *)(iVar1 + 0xb6) = 0x0;
    *(undefined2 *)param_1        = 0x55a2;
    *(undefined2 *)(iVar1 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    if(param_2 != 0x0)
    {
        uVar3                         = (undefined2)((ulong)param_2 >> 0x10);
        *(undefined4 *)(iVar1 + 0xb0) = *(undefined4 *)((int)param_2 + 0x6);
        *(undefined2 *)(iVar1 + 0xb4) = *(undefined2 *)((int)param_2 + 0x14);
    }
    return;
}


ulong __stdcall16far pass1_1040_5d12(ulong param_1)

{
    uint         uVar1;
    uint         uVar2;
    undefined4   uVar3;
    astruct_440 *iVar4;
    undefined2   uVar4;
    ulong        uVar5;

    uVar3 = *(undefined4 *)((int)param_1 + 0x90);
    uVar4 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar4 = (astruct_440 *)uVar3;
    uVar1 = iVar4->field_0x6;
    uVar2 = iVar4->field_0x8;
    if((uVar2 | uVar1) != 0x0)
    {
        uVar5 = struct_op_1030_73a8(CONCAT22(uVar2, uVar1));
        return uVar5;
    }
    return 0x0;
}


void __stdcall16far pass1_1040_3966(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_722 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x185, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_722 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x96                 = 0x0;
    iVar1->field_0x9a                 = 0x0;
    iVar1->field_0x9c                 = 0x0;
    iVar1->field_0x9e                 = 0x0;
    iVar1->field_0xa0                 = 0x0;
    iVar1->field_0xa2                 = 0x0;
    iVar1->field_0xa4                 = 0x5;
    *(undefined2 *)param_1            = 0x3ffc;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1040;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}


astruct_57 *__stdcall16far
pas1_1040_29c2(astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5)

{
    int        iVar1;
    undefined2 uVar2;

    pass1_1040_b0bc(param_1, param_2, CONCAT22(param_3, 0x157));
    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *(undefined2 *)param_1       = 0x2e26;
    *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    *(ushort *)(iVar1 + 0x94) = param_4;
    *(ushort *)(iVar1 + 0x96) = param_5;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    *(ushort *)(iVar1 + 0x98) = param_4;
    *(ushort *)(iVar1 + 0x9a) = param_5;
    return param_1;
}


void __stdcall16far pass1_1040_2dac(ulong param_1)

{
    undefined4 uVar1;
    ulong      uVar2;
    int        iStack10;

    uVar1 = *(undefined4 *)((int)param_1 + 0x90);
    uVar2 = struct_op_1030_73a8(*(ulong *)((int)uVar1 + 0x6));
    for(iStack10 = 0x0; iStack10 < 0x5; iStack10 = iStack10 + 0x1)
    {
        pass1_1028_4ab2(
          uVar2, *(ushort *)((int)&PTR_LOOP_1050_5d04 + iStack10 * 0xc), *(int *)(iStack10 * 0xc + 0x5d02));
    }
    return;
}


void __stdcall16far pass1_1038_88f2(astruct_57 *param_1, ushort param_2)

{
    int        iVar1;
    undefined2 uVar2;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x184c));
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined4 *)(iVar1 + 0x94) = _PTR_LOOP_1050_5a68;
    *(undefined2 *)(iVar1 + 0x98) = 0x0;
    *(undefined2 *)(iVar1 + 0x9a) = 0x0;
    *(undefined2 *)(iVar1 + 0x9c) = 0x0;
    *(undefined2 *)(iVar1 + 0x9e) = 0x0;
    *(undefined2 *)param_1        = 0x8c2e;
    *(undefined2 *)(iVar1 + 0x2)  = (int)&PTR_LOOP_1050_1038;
    return;
}


astruct_57 *__stdcall16far
pass1_1038_8caa(astruct_57 *param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5)

{
    astruct_704 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_704 *)param_1;
    *(undefined4 *)&iVar1->field_0x94 = 0x0;
    *(undefined2 *)param_1            = 0x90c8;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3f, param_5, param_3, param_4);
    iVar1->field_0x94                 = (int)puVar2;
    iVar1->field_0x96                 = (int)((ulong)puVar2 >> 0x10);
    return param_1;
}


void __stdcall16far struct_1038_6520(ushort *param_1)

{
    astruct_308 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_308 *)param_1;
    *param_1          = 0x389a;
    iVar1->field_0x2  = 0x1008;
    iVar1->field_0x4  = 0x0;
    iVar1->field_0x8  = 0x0;
    iVar1->field_0xc  = 0x0;
    iVar1->field_0xe  = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x14 = 0x0;
    iVar1->field_0x16 = 0x0;
    pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x1a));
    iVar1->field_0x20 = 0x0;
    iVar1->field_0x24 = 0x0;
    iVar1->field_0x26 = 0x0;
    iVar1->field_0x28 = 0x0;
    *param_1          = 0x78de;
    iVar1->field_0x2  = (int)&PTR_LOOP_1050_1038;
    return;
}


void __stdcall16far
pass1_1038_6590(ushort *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, ulong param_6)

{
    ushort       uVar1;
    int          iVar2;
    astruct_410 *iVar3;
    undefined2   uVar3;
    ushort       unaff_SS;
    ushort      *puVar4;
    ulong        uVar5;

    uVar3                            = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                            = (astruct_410 *)param_1;
    *param_1                         = 0x389a;
    iVar3->field_0x2                 = 0x1008;
    *(undefined4 *)&iVar3->field_0x4 = 0x0;
    iVar3->field_0x8                 = param_6;
    iVar3->field_0xc                 = param_4;
    iVar3->field_0xe                 = 0x0;
    iVar3->field_0x12                = 0x0;
    iVar3->field_0x14                = 0x0;
    iVar3->field_0x16                = param_2;
    iVar3->field_0x18                = param_3;
    puVar4 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
    uVar1  = (ushort)((ulong)puVar4 >> 0x10);
    *(undefined4 *)&iVar3->field_0x20 = 0x0;
    iVar3->field_0x24                 = 0x0;
    iVar3->field_0x26                 = param_5;
    iVar3->field_0x28                 = 0x0;
    *param_1                          = 0x78de;
    iVar3->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_6, (uint)(param_6 >> 0x10));
    uVar5            = pass1_1030_6d4e(CONCAT22(uVar1, param_5), param_5, uVar1, unaff_SS);
    iVar2            = (int)(uVar5 >> 0x10);
    iVar3->field_0x4 = (int)uVar5;
    iVar3->field_0x6 = iVar2;
    puVar4           = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
    pass1_1008_3f62(puVar4, (ushort *)CONCAT22(uVar1, param_5 + 0xc));
    uVar1 = (ushort)puVar4;
    pass1_1010_8fba(*(ulong *)&iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = iVar2;
    return;
}


void __stdcall16far pass1_1038_666e(ushort *param_1, long *param_2, ushort param_3, ulong param_4)

{
    ushort       uVar1;
    undefined2   uVar2;
    astruct_420 *iVar3;
    undefined2   uVar3;
    ushort       unaff_SS;
    ushort      *puVar4;
    ulong        uVar5;

    uVar3             = (undefined2)((ulong)param_1 >> 0x10);
    iVar3             = (astruct_420 *)param_1;
    *param_1          = 0x389a;
    iVar3->field_0x2  = 0x1008;
    iVar3->field_0x4  = 0x0;
    iVar3->field_0x8  = param_4;
    iVar3->field_0xc  = 0x0;
    iVar3->field_0xe  = param_2;
    iVar3->field_0x12 = 0x0;
    iVar3->field_0x14 = 0x0;
    iVar3->field_0x18 = 0x0;
    iVar3->field_0x16 = 0x0;
    puVar4            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
    uVar1             = (ushort)((ulong)puVar4 >> 0x10);
    *(undefined4 *)&iVar3->field_0x20 = 0x0;
    iVar3->field_0x24                 = 0x0;
    iVar3->field_0x26                 = param_3;
    iVar3->field_0x28                 = 0x0;
    *param_1                          = 0x78de;
    iVar3->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
    uVar5                                         = pass1_1030_6d4e(CONCAT22(uVar1, param_3), param_3, uVar1, unaff_SS);
    uVar2                                         = (undefined2)(uVar5 >> 0x10);
    *(int *)&iVar3->field_0x4                     = (int)uVar5;
    *(undefined2 *)((int)&iVar3->field_0x4 + 0x2) = uVar2;
    puVar4 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
    pass1_1008_3f62(puVar4, (ushort *)CONCAT22(uVar1, param_3 + 0xc));
    uVar1 = (ushort)puVar4;
    pass1_1010_8fba(iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = uVar2;
    pass1_1020_ba94(param_2);
    iVar3->field_0x16 = uVar1;
    iVar3->field_0x18 = uVar2;
    return;
}


void __stdcall16far pass1_1038_675c(ushort *param_1, ulong param_2, ushort param_3, ushort param_4, ulong param_5)

{
    ushort       uVar1;
    undefined2   uVar2;
    astruct_414 *iVar3;
    undefined2   uVar3;
    ushort       unaff_SS;
    ushort      *puVar4;
    ulong        uVar5;

    uVar3             = (undefined2)((ulong)param_1 >> 0x10);
    iVar3             = (astruct_414 *)param_1;
    *param_1          = 0x389a;
    iVar3->field_0x2  = 0x1008;
    iVar3->field_0x4  = 0x0;
    iVar3->field_0x8  = param_5;
    iVar3->field_0xc  = 0x0;
    iVar3->field_0xe  = 0x0;
    iVar3->field_0x12 = param_3;
    iVar3->field_0x14 = 0x0;
    iVar3->field_0x16 = param_2;
    puVar4            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
    uVar1             = (ushort)((ulong)puVar4 >> 0x10);
    *(undefined4 *)&iVar3->field_0x20 = 0x0;
    iVar3->field_0x24                 = 0x0;
    iVar3->field_0x26                 = param_4;
    iVar3->field_0x28                 = 0x0;
    *param_1                          = 0x78de;
    iVar3->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_5, (uint)(param_5 >> 0x10));
    uVar5                                         = pass1_1030_6d4e(CONCAT22(uVar1, param_4), param_4, uVar1, unaff_SS);
    uVar2                                         = (undefined2)(uVar5 >> 0x10);
    *(int *)&iVar3->field_0x4                     = (int)uVar5;
    *(undefined2 *)((int)&iVar3->field_0x4 + 0x2) = uVar2;
    puVar4 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
    pass1_1008_3f62(puVar4, (ushort *)CONCAT22(uVar1, param_4 + 0xc));
    uVar1 = (ushort)puVar4;
    pass1_1010_8fba(iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = uVar2;
    return;
}


void __stdcall16far pass1_1038_6838(ushort *param_1, ulong param_2, ushort param_3, ushort param_4, ulong param_5)

{
    ushort       uVar1;
    undefined2   uVar2;
    astruct_415 *iVar3;
    undefined2   uVar3;
    ushort       unaff_SS;
    ushort      *puVar4;
    ulong        uVar5;

    uVar3             = (undefined2)((ulong)param_1 >> 0x10);
    iVar3             = (astruct_415 *)param_1;
    *param_1          = 0x389a;
    iVar3->field_0x2  = 0x1008;
    iVar3->field_0x4  = 0x0;
    iVar3->field_0x8  = param_5;
    iVar3->field_0xc  = 0x0;
    iVar3->field_0xe  = 0x0;
    iVar3->field_0x12 = 0x0;
    iVar3->field_0x14 = param_3;
    iVar3->field_0x16 = param_2;
    puVar4            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a));
    uVar1             = (ushort)((ulong)puVar4 >> 0x10);
    *(undefined4 *)&iVar3->field_0x20 = 0x0;
    iVar3->field_0x24                 = 0x0;
    iVar3->field_0x26                 = param_4;
    iVar3->field_0x28                 = 0x0;
    *param_1                          = 0x78de;
    iVar3->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_5, (uint)(param_5 >> 0x10));
    uVar5                                         = pass1_1030_6d4e(CONCAT22(uVar1, param_4), param_4, uVar1, unaff_SS);
    uVar2                                         = (undefined2)(uVar5 >> 0x10);
    *(int *)&iVar3->field_0x4                     = (int)uVar5;
    *(undefined2 *)((int)&iVar3->field_0x4 + 0x2) = uVar2;
    puVar4 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1a);
    pass1_1008_3f62(puVar4, (ushort *)CONCAT22(uVar1, param_4 + 0xc));
    uVar1 = (ushort)puVar4;
    pass1_1010_8fba(iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = uVar2;
    return;
}


void __stdcall16far pass1_1038_69fe(ulong param_1)

{
    *(undefined2 *)((int)param_1 + 0x28) = 0x0;
    return;
}


void __stdcall16far pass1_1038_6c68(
  ulong param_1, ushort *param_2, ulong *param_3, int param_4, uchar *param_5, int param_6, ushort param_7)

{
    int     iVar1;
    ushort  uVar2;
    uint    uVar3;
    ushort *puVar4;
    uint    uVar5;
    uint    uVar6;
    uchar  *puVar7;
    ushort  uVar8;
    ushort *puVar9;
    ulong   uVar10;
    int     iStack30;

    uVar2 = (ushort)param_1;
    pass1_1008_3f62(param_2, (ushort *)(param_1 & 0xffff0000 | (ulong)(uVar2 + 0x1a)));
    puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    uVar5  = (uint)((ulong)puVar9 >> 0x10);
    puVar4 = (ushort *)(param_1 & 0xffff0000 | (ulong)(uVar2 + 0x1a));
    pass1_1030_627e(param_7, uVar2 + 0x1a, uVar5, _PTR_LOOP_1050_5740, puVar4, *(long *)((int)puVar9 + 0x20));
    uVar3 = (uint)puVar4;
    uVar6 = uVar5 | uVar3;
    if(uVar6 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, uVar5);
        uVar10 = struct_op_1030_73a8(CONCAT22(uVar6, uVar3));
        puVar7 = (uchar *)(uVar10 >> 0x10);
        iVar1  = *(int *)((int)uVar10 + 0xc);
        if((iVar1 == 0x47) || (iVar1 == 0x6a))
        {
            uVar8    = (ushort)(param_1 >> 0x10);
            iStack30 = *(int *)(uVar2 + 0x1e);
            if(param_4 < 0x0)
            {
                iStack30 = iStack30 + -0x1;
            }
            else
            {
                iStack30 = iStack30 + 0x1;
            }
            *(int *)((int)param_2 + 0x4) = iStack30;
            pass1_1038_6b88(uVar2, uVar8, param_2, param_3, puVar7, param_6, param_7);
        }
    }
    return;
}


void __stdcall16far pass1_1038_709c(ulong param_1, ulong param_2, uchar *param_3, ushort param_4)

{
    ulong       *puVar1;
    long         lVar2;
    ushort       uVar7;
    uint         uVar8;
    uchar       *puVar9;
    astruct_618 *iVar8;
    int          iVar10;
    undefined2   uVar11;
    undefined2   uVar12;
    astruct_99  *paStack40;
    astruct_99  *paStack16;
    uint         uStack12;
    long         local_a;
    undefined2   local_6;
    uint         uStack4;
    astruct_617 *uVar3;
    astruct_619 *uVar4;
    astruct_620 *uVar5;
    astruct_621 *uVar6;

    uVar11 = (undefined2)(param_1 >> 0x10);
    iVar8  = (astruct_618 *)param_1;
    if((*(uint *)((int)&iVar8->field_0xe + 0x2) | *(uint *)&iVar8->field_0xe) == 0x0)
    {
        if(iVar8->field_0xc == 0x0)
        {
            if(iVar8->field_0x12 == 0x0)
            {
                if(iVar8->field_0x14 == 0x0)
                {
                    return;
                }
                paStack40 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                uVar8     = (uint)((ulong)paStack40 >> 0x10);
                uVar3     = (astruct_617 *)paStack40;
                if((uVar8 | (uint)uVar3) == 0x0)
                {
                    paStack40 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack40->field_0x0 = 0x389a;
                    uVar3->field_0x2     = 0x1008;
                    uVar3->field_0x4     = 0x0;
                    uVar3->field_0x6     = 0x0;
                    uVar3->field_0x8     = 0x0;
                    uVar3->field_0xa     = 0x0;
                    uVar3->field_0xc     = 0x0;
                    paStack40->field_0x0 = 0x56ce;
                    uVar3->field_0x2     = 0x1018;
                }
                uVar12                                = (undefined2)((ulong)paStack40 >> 0x10);
                *(int *)((int)paStack40 + 0x8)        = iVar8->field_0x14;
                *(undefined2 *)((int)paStack40 + 0xa) = *(undefined2 *)&iVar8->field_0x16;
                uVar8                                 = pass1_1020_c42e(iVar8->field_0x14);
            }
            else
            {
                pass1_1030_7c50(param_2, iVar8->field_0x16, iVar8->field_0x12, 0x0, param_3);
                paStack40 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                uVar8     = (uint)((ulong)paStack40 >> 0x10);
                uVar4     = (astruct_619 *)paStack40;
                if((uVar8 | (uint)uVar4) == 0x0)
                {
                    paStack40 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack40->field_0x0 = 0x389a;
                    uVar4->field_0x2     = 0x1008;
                    uVar4->field_0x4     = 0x0;
                    uVar4->field_0x6     = 0x0;
                    uVar4->field_0x8     = 0x0;
                    uVar4->field_0xa     = 0x0;
                    uVar4->field_0xc     = 0x0;
                    paStack40->field_0x0 = 0x56ce;
                    uVar4->field_0x2     = 0x1018;
                }
                uVar12                                = (undefined2)((ulong)paStack40 >> 0x10);
                *(ushort *)((int)paStack40 + 0x6)     = iVar8->field_0x12;
                *(undefined2 *)((int)paStack40 + 0xa) = *(undefined2 *)&iVar8->field_0x16;
                uVar8                                 = switch_1020_c3b4(iVar8->field_0x12);
            }
            uVar12 = (undefined2)((ulong)paStack40 >> 0x10);
            iVar10 = (int)paStack40;
            lVar2  = (ulong)uVar8 * (ulong) * (uint *)(iVar10 + 0xa);
            puVar9 = (uchar *)((ulong)lVar2 >> 0x10);
            uVar8  = (uint)lVar2;
        }
        else
        {
            paStack40 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
            uVar8     = (uint)((ulong)paStack40 >> 0x10);
            uVar5     = (astruct_620 *)paStack40;
            puVar9    = (uchar *)(uVar8 | (uint)uVar5);
            if(puVar9 == (uchar *)0x0)
            {
                paStack40 = (astruct_99 *)0x0;
            }
            else
            {
                paStack40->field_0x0 = 0x389a;
                uVar5->field_0x2     = 0x1008;
                uVar5->field_0x4     = 0x0;
                uVar5->field_0x6     = 0x0;
                uVar5->field_0x8     = 0x0;
                uVar5->field_0xa     = 0x0;
                uVar5->field_0xc     = 0x0;
                paStack40->field_0x0 = 0x56ce;
                uVar5->field_0x2     = 0x1018;
            }
            uVar12                  = (undefined2)((ulong)paStack40 >> 0x10);
            iVar10                  = (int)paStack40;
            *(int *)(iVar10 + 0x4)  = iVar8->field_0xc;
            uVar8                   = *(uint *)&iVar8->field_0x16;
            *(uint *)(iVar10 + 0xa) = uVar8;
        }
        *(uint *)(iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(param_2, CONCAT22(uVar12, iVar10), uVar8, puVar9, param_4);
    }
    else
    {
        puVar1  = iVar8->field_0xe;
        uStack4 = *(uint *)((int)puVar1 + 0x4);
        for(uStack12 = 0x0; uStack12 < uStack4; uStack12 = uStack12 + 0x1)
        {
            pass1_1020_bb16(
              iVar8->field_0xe, (ulong *)CONCAT22(param_4, &local_a), (ushort *)CONCAT22(param_4, &local_6), uStack12);
            if(local_a != 0x0)
            {
                paStack16 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                uVar8     = (uint)((ulong)paStack16 >> 0x10);
                uVar6     = (astruct_621 *)paStack16;
                if((uVar8 | (uint)uVar6) == 0x0)
                {
                    paStack16 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack16->field_0x0 = 0x389a;
                    uVar6->field_0x2     = 0x1008;
                    uVar6->field_0x4     = 0x0;
                    uVar6->field_0x6     = 0x0;
                    uVar6->field_0x8     = 0x0;
                    uVar6->field_0xa     = 0x0;
                    uVar6->field_0xc     = 0x0;
                    paStack16->field_0x0 = 0x56ce;
                    uVar6->field_0x2     = 0x1018;
                }
                uVar12                        = (undefined2)((ulong)paStack16 >> 0x10);
                iVar10                        = (int)paStack16;
                *(undefined2 *)(iVar10 + 0x4) = local_6;
                *(undefined2 *)(iVar10 + 0xa) = (undefined2)local_a;
                uVar7                         = pass1_1020_c3ae();
                lVar2                         = (ulong)uVar7 * (ulong) * (uint *)(iVar10 + 0xa);
                uVar8                         = (uint)lVar2;
                *(uint *)(iVar10 + 0xc)       = uVar8;
                pass1_1030_6a2c(param_2, (long)paStack16, uVar8, (uchar *)((ulong)lVar2 >> 0x10), param_4);
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_7356(ulong param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5)

{
    uchar      **ppuVar1;
    uint        *puVar2;
    undefined4   uVar3;
    astruct_18  *paVar4;
    long         lVar5;
    BOOL16       BVar6;
    uint         uVar7;
    uint         uVar9;
    uchar       *puVar10;
    uchar       *puVar11;
    astruct_615 *iVar9;
    int          iVar12;
    undefined2   uVar13;
    undefined2   uVar14;
    bool         bVar15;
    ulong        uVar16;
    ulong        uVar17;
    astruct_99  *paStack50;
    astruct_99  *paStack26;
    astruct_616 *uVar8;
    astruct_622 *uVar10;

    uVar16  = struct_op_1030_73a8(param_2);
    puVar10 = (uchar *)(uVar16 >> 0x10);
    uVar7   = (uint)uVar16;
    puVar11 = puVar10;
    BVar6   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(uVar7 + 0xc), 0x4);
    iVar9   = (astruct_615 *)param_1;
    uVar13  = (undefined2)(param_1 >> 0x10);
    if(BVar6 == 0x0)
    {
        uVar9 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(uVar7 + 0xc), 0x3);
        if(uVar9 == 0x0)
        {
        code_r0x10387545:
            pass1_1038_6f5a(param_1, param_2, uVar9, puVar11, param_4, param_5, param_3);
            goto LAB_1038_7549;
        }
        if((iVar9->field_0xc != 0x0) || (*(long *)&iVar9->field_0xe != 0x0))
        {
            uVar16  = pass1_1028_45e2(uVar16, uVar7, (int)puVar11, param_3);
            puVar11 = (uchar *)(uVar16 >> 0x10);
            uVar9   = (uint)uVar16;
            ppuVar1 = (uchar **)&iVar9->field_0x18;
            bVar15  = *ppuVar1 < puVar11;
            if((bVar15 || *ppuVar1 == puVar11)
               && ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9))))
                goto code_r0x10387545;
        }
    }
    else
    {
        uVar17  = pass1_1028_62c8(uVar16, param_3);
        puVar11 = (uchar *)(uVar17 >> 0x10);
        uVar9   = (uint)uVar17;
        ppuVar1 = (uchar **)&iVar9->field_0x18;
        bVar15  = *ppuVar1 < puVar11;
        if((bVar15 || *ppuVar1 == puVar11)
           && ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9))))
        {
            if(iVar9->field_0x12 == 0x0)
            {
                if(iVar9->field_0x14 == 0x0)
                    goto LAB_1038_74e0;
                paStack50 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                uVar7     = (uint)((ulong)paStack50 >> 0x10);
                uVar10    = (astruct_622 *)paStack50;
                if((uVar7 | (uint)uVar10) == 0x0)
                {
                    paStack50 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack50->field_0x0 = 0x389a;
                    uVar10->field_0x2    = 0x1008;
                    uVar10->field_0x4    = 0x0;
                    uVar10->field_0x6    = 0x0;
                    uVar10->field_0x8    = 0x0;
                    uVar10->field_0xa    = 0x0;
                    uVar10->field_0xc    = 0x0;
                    paStack50->field_0x0 = 0x56ce;
                    uVar10->field_0x2    = 0x1018;
                }
                uVar14                  = (undefined2)((ulong)paStack50 >> 0x10);
                iVar12                  = (int)paStack50;
                *(int *)(iVar12 + 0x8)  = iVar9->field_0x14;
                *(uint *)(iVar12 + 0xa) = iVar9->field_0x16;
                uVar7                   = pass1_1020_c42e(iVar9->field_0x14);
            }
            else
            {
                paStack26 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                uVar7     = (uint)((ulong)paStack26 >> 0x10);
                uVar8     = (astruct_616 *)paStack26;
                if((uVar7 | (uint)uVar8) == 0x0)
                {
                    paStack26 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack26->field_0x0 = 0x389a;
                    uVar8->field_0x2     = 0x1008;
                    uVar8->field_0x4     = 0x0;
                    uVar8->field_0x6     = 0x0;
                    uVar8->field_0x8     = 0x0;
                    uVar8->field_0xa     = 0x0;
                    uVar8->field_0xc     = 0x0;
                    paStack26->field_0x0 = 0x56ce;
                    uVar8->field_0x2     = 0x1018;
                }
                uVar14                    = (undefined2)((ulong)paStack26 >> 0x10);
                iVar12                    = (int)paStack26;
                *(ushort *)(iVar12 + 0x6) = iVar9->field_0x12;
                *(uint *)(iVar12 + 0xa)   = iVar9->field_0x16;
                uVar7                     = switch_1020_c3b4(iVar9->field_0x12);
            }
            lVar5                   = (ulong)uVar7 * (ulong) * (uint *)(iVar12 + 0xa);
            puVar11                 = (uchar *)((ulong)lVar5 >> 0x10);
            uVar9                   = (uint)lVar5;
            *(uint *)(iVar12 + 0xc) = uVar9;
            pass1_1028_6408(uVar16, (ulong *)CONCAT22(uVar14, iVar12), param_3);
            goto LAB_1038_7549;
        }
    }
LAB_1038_74e0:
    pass1_1038_709c(param_1, param_2, puVar11, param_3);
LAB_1038_7549:
    uVar3 = iVar9->field_0x8;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar3, (uint)((ulong)uVar3 >> 0x10));
    pass1_1030_6c4c(CONCAT22(puVar11, uVar9), *(int *)(uVar9 + 0x34) + iVar9->field_0x26);
    iVar9->field_0xc                  = 0x0;
    iVar9->field_0x12                 = 0x0;
    iVar9->field_0x14                 = 0x0;
    *(undefined4 *)&iVar9->field_0x16 = 0x0;
    paVar4                            = *(astruct_18 **)&iVar9->field_0xe;
    uVar7                             = iVar9->field_0x10;
    if((uVar7 | (uint)paVar4) != 0x0)
    {
        fn_ptr_1020_ba7e((ulong *)((ulong)paVar4 & 0xffff | (ulong)uVar7 << 0x10));
        fn_ptr_1000_17ce(paVar4, 0x1000);
    }
    *(undefined4 *)&iVar9->field_0xe = 0x0;
    return;
}


void __stdcall16far pass1_1038_58e6(
  ushort param_1, ushort param_2, ulong param_3, ulong param_4, ulong param_5, int param_6, ushort param_7)

{
    int         iVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    BOOL16      BVar4;
    undefined4 *puVar5;
    uint        uVar6;
    int         iVar7;
    undefined2  uVar8;
    undefined2  uVar9;
    ulong       uVar10;
    undefined4  local_12;
    int         iStack14;
    int         iStack12;
    ulong       uStack6;

    for(uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1)
    {
        uVar9 = (undefined2)(param_4 >> 0x10);
        iVar7 = (int)param_4;
        if((*(long *)((int)uStack6 * 0x4 + iVar7) != 0x0)
           && (uVar3 = *(undefined4 *)((int)uStack6 * 0x4 + iVar7),
               BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((int)uVar3 + 0xc), 0x2e),
               BVar4 != 0x0))
        {
            uVar8    = (undefined2)(param_5 >> 0x10);
            iVar1    = *(int *)((int)uStack6 * 0x4 + (int)param_5);
            uVar8    = *(undefined2 *)((int)uStack6 * 0x4 + (int)param_5 + 0x2);
            local_12 = *(undefined4 *)(iVar1 + 0xc);
            iStack12 = *(int *)(iVar1 + 0x10);
            iStack14 = iStack12;
            if(iStack12 == param_6)
            {
                iStack14 = iStack12 + -0x1;
                uVar10   = pass1_1028_bb24(*(ulong *)((int)uStack6 * 0x4 + iVar7));
                uVar6    = (uint)(uVar10 >> 0x10);
                puVar5   = &local_12;
                pass1_1030_627e(param_7,
                                (uint)puVar5,
                                uVar6,
                                _PTR_LOOP_1050_5740,
                                (ushort *)CONCAT22(param_7, puVar5),
                                uVar10 & 0xffff | (ulong)uVar6 << 0x10);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar5, uVar6);
                if((uVar6 | (uint)puVar5) != 0x0)
                {
                    uVar10 = struct_op_1030_73a8(CONCAT22(uVar6, puVar5));
                    uVar6  = *(uint *)((int)uVar10 + 0x1a);
                    if(((uVar6 & 0x2) != 0x0) && ((uVar6 & 0x1) != 0x0))
                    {
                        uVar3                              = *(undefined4 *)((int)uStack6 * 0x4 + iVar7);
                        *(undefined2 *)((int)uVar3 + 0x1a) = 0x3;
                        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack6 * 0x4 + iVar7) + 0x28);
                        (**ppcVar2)();
                    }
                }
            }
        }
    }
    return;
}


ushort __stdcall16far pass1_1038_5be8(
  undefined4 param_1, uint param_2, int param_3, ushort *param_4, uint param_5, uint param_6, ushort param_7)

{
    int        iVar1;
    undefined2 uVar2;
    int        iVar3;
    BOOL16     BVar4;
    uint       uVar5;
    ulong      uVar6;
    int        iStack14;
    ulong      uStack10;

    pass1_1030_627e(param_7, param_5, param_6, _PTR_LOOP_1050_5740, param_4, *(long *)((int)param_1 + 0x8));
    uVar5 = param_6 | param_5;
    if(uVar5 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, param_6);
        uStack10 = CONCAT22(uVar5, param_5);
        iStack14 = 0x7a;
        if(0x0 < *(int *)((int)param_4 + 0x4))
        {
            if(param_3 == 0x7b)
            {
                param_3 = 0x7e;
            }
            else
            {
                if(param_3 == 0x7c)
                {
                    param_3 = 0x7d;
                }
            }
            iStack14 = 0x7f;
        }
        uVar6 = struct_op_1030_73a8(uStack10);
        uVar2 = (undefined2)(uVar6 >> 0x10);
        iVar3 = (int)uVar6;
        if((((*(uint *)(iVar3 + 0x1a) & param_2) == 0x0)
            && (((iVar1 = *(int *)(iVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_3))
                 || (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x2b), BVar4 != 0x0))))
           && (*(int *)(iVar3 + 0x12) != 0x7))
        {
            *(uint *)(iVar3 + 0x1a) = *(uint *)(iVar3 + 0x1a) | param_2;
            return 0x1;
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1038_4b40(ulong param_1, ushort param_2, ushort param_3)

{
    code     **ppcVar1;
    uint       uVar2;
    ulong      uVar3;
    undefined2 extraout_DX;
    undefined2 uVar4;
    uint       extraout_DX_00;
    uint       uVar5;
    int        iVar6;
    undefined2 uVar7;
    ulong      uStack14;
    ulong      uStack10;

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if(*(long *)(iVar6 + 0xc) == 0x0)
    {
        param_2 = 0x0;
        uVar4   = 0x0;
    }
    else
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar4, param_2);
    for(uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x4);
        uVar3   = uStack10;
        (**ppcVar1)(param_3, *(undefined4 *)(iVar6 + 0xc));
        uVar2 = (uint)uVar3;
        uVar5 = extraout_DX_00 | uVar2;
        if(uVar5 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
            param_3 = 0x1030;
            struct_op_1030_73a8(CONCAT22(uVar5, uVar2));
        }
    }
    return;
}


void __stdcall16far pass1_1038_4cd0(ulong param_1, ulong param_2, ushort param_3)

{
    undefined2 uVar1;

    uVar1                            = (undefined2)(param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0x1c) = param_3;
    *(ulong *)((int)param_1 + 0x1e)  = param_2;
    return;
}


void __stdcall16far pass1_1038_4cea(ulong param_1, ulong *param_2, ushort *param_3)

{
    undefined2 uVar1;

    uVar1    = (undefined2)(param_1 >> 0x10);
    *param_3 = *(ushort *)((int)param_1 + 0x1c);
    *param_2 = *(ulong *)((int)param_1 + 0x1e);
    return;
}


void __stdcall16far pass1_1038_4d0e(ulong param_1, ushort param_2)

{
    astruct_686 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)(param_1 >> 0x10);
    iVar1             = (astruct_686 *)param_1;
    iVar1->field_0x1a = iVar1->field_0x18;
    iVar1->field_0x18 = param_2;
    return;
}


void __stdcall16far pass1_1038_4d6e(ulong param_1, ulong *param_2, uint param_3, uchar *param_4)

{
    int        *piVar1;
    code      **ppcVar2;
    uint        uVar3;
    ushort      uVar4;
    undefined2  extraout_DX;
    undefined2  extraout_DX_00;
    undefined2  uVar5;
    uint        extraout_DX_01;
    uint        uVar6;
    int         iVar7;
    undefined2  uVar8;
    ulong       uVar9;
    int         iStack30;
    ulong       uStack26;
    ulong       uStack14;
    ulong       uStack10;
    undefined4 *puStack6;

    mem_op_1000_179c(0x18, param_4, 0x1000);
    if(((uint)param_4 | param_3) == 0x0)
    {
        param_3 = 0x0;
        uVar8   = 0x0;
    }
    else
    {
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_4, param_3), 0x5, 0x5);
        uVar8 = extraout_DX;
    }
    puStack6 = (undefined4 *)CONCAT22(uVar8, param_3);
    uVar8    = (undefined2)(param_1 >> 0x10);
    iVar7    = (int)param_1;
    if(*(long *)(iVar7 + 0xc) == 0x0)
    {
        param_3 = 0x0;
        uVar5   = 0x0;
    }
    else
    {
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
        (**ppcVar2)();
        uVar5 = extraout_DX_00;
    }
    uStack10 = CONCAT22(uVar5, param_3);
    uStack14 = 0x0;
    do
    {
        if(uStack10 <= uStack14)
        {
            return;
        }
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x4);
        uVar9   = uStack10;
        (**ppcVar2)();
        uVar3 = (uint)uVar9;
        uVar6 = extraout_DX_01 | uVar3;
        if(uVar6 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_01);
            uStack26 = CONCAT22(uVar6, uVar3);
            uVar4    = pass1_1030_6fa0(CONCAT22(uVar6, uVar3));
            iStack30 = 0x0;
            while(true)
            {
                piVar1 = (int *)((int)param_2 + 0x4);
                if(*piVar1 == iStack30 || *piVar1 < iStack30)
                    break;
                if(*(ushort *)((int)*param_2 + iStack30 * 0x2) == uVar4)
                {
                    uVar9 = struct_op_1030_73a8(uStack26);
                    if(*(int *)((int)uVar9 + 0x12) == 0x5)
                    {
                        ppcVar2 = (code **)((int)*puStack6 + 0xc);
                        (**ppcVar2)();
                    }
                    break;
                }
                iStack30 = iStack30 + 0x1;
            }
        }
        uStack14 = uStack14 + 0x1;
    } while(true);
}


void __stdcall16far pass1_1038_4e78(uint param_1, uchar *param_2, ulong param_3, ulong *param_4)

{
    int        *piVar1;
    code      **ppcVar2;
    ushort      uVar3;
    ulong       uVar4;
    undefined2  extraout_DX;
    uint        extraout_DX_00;
    uint        uVar5;
    uint        extraout_DX_01;
    uint        uVar6;
    int         iVar7;
    undefined2  uVar8;
    int         iStack26;
    ulong       uStack14;
    ulong       uStack10;
    undefined4 *puStack6;

    mem_op_1000_179c(0x18, param_2, 0x1000);
    if(((uint)param_2 | param_1) == 0x0)
    {
        param_1 = 0x0;
        uVar8   = 0x0;
    }
    else
    {
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2, param_1), 0x5, 0x5);
        uVar8 = extraout_DX;
    }
    puStack6 = (undefined4 *)CONCAT22(uVar8, param_1);
    uVar8    = (undefined2)(param_3 >> 0x10);
    iVar7    = (int)param_3;
    if(*(long *)(iVar7 + 0xc) == 0x0)
    {
        param_1 = 0x0;
        uVar5   = 0x0;
    }
    else
    {
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
        (**ppcVar2)();
        uVar5 = extraout_DX_00;
    }
    uStack10 = CONCAT22(uVar5, param_1);
    uStack14 = 0x0;
    do
    {
        if(uStack10 <= uStack14)
        {
            return;
        }
        uVar4 = uStack10;
        pass1_1030_1d58(*(ulong *)(iVar7 + 0xc));
        uVar6 = uVar5 | (uint)uVar4;
        if(uVar6 != 0x0)
        {
            uVar3    = pass1_1030_6fa0(uVar4 & 0xffff | (ulong)uVar5 << 0x10);
            iStack26 = 0x0;
            while(true)
            {
                piVar1 = (int *)((int)param_4 + 0x4);
                if(*piVar1 == iStack26 || *piVar1 < iStack26)
                    break;
                if(*(ushort *)((int)*param_4 + iStack26 * 0x2) == uVar3)
                {
                    ppcVar2 = (code **)((int)*puStack6 + 0xc);
                    (**ppcVar2)();
                    uVar6 = extraout_DX_01;
                    break;
                }
                iStack26 = iStack26 + 0x1;
            }
        }
        uStack14 = uStack14 + 0x1;
        uVar5    = uVar6;
    } while(true);
}


astruct_100 *__stdcall16far pass1_1038_28d8(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x3a97);
    param_1->field_0x0                  = 0x29fe;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCRoboMove_1050_59f8);
    return param_1;
}


void __stdcall16far pass1_1038_2a0e(
  astruct_100 *param_1, ulong param_2, ulong param_3, ulong param_4, ulong param_5, ushort param_6, uchar param_7)

{
    int        iVar1;
    undefined2 uVar2;

    struct_op_1028_d1dc(param_6, param_7, param_1, 0x2af7);
    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *(ulong *)(iVar1 + 0x108)    = param_5;
    *(ulong *)(iVar1 + 0x10c)    = param_4;
    *(ulong *)(iVar1 + 0x110)    = param_3;
    *(ulong *)(iVar1 + 0x114)    = param_2;
    param_1->field_0x0           = 0x309a;
    *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return;
}


void __stdcall16far
pass1_1038_1c3e(ulong param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6)

{
    undefined2  uVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    ulong       uVar4;
    uint        uVar5;
    int         iVar6;
    BOOL16      BVar7;
    undefined4 *puVar8;
    uint        extraout_DX;
    uint        extraout_DX_00;
    uint        uVar9;
    undefined2  uVar10;
    ulong       uVar11;
    ushort      uVar12;
    ushort      uVar13;
    undefined2  uVar14;
    ulong       uStack26;
    ulong       uStack14;

    uVar10  = (undefined2)(param_2 >> 0x10);
    puVar2  = (undefined4 *)*(undefined4 *)((int)param_2 + 0xc);
    uVar10  = *(undefined2 *)((int)param_2 + 0xe);
    ppcVar3 = (code **)((int)*puVar2 + 0x10);
    puVar8  = puVar2;
    uVar14  = (int)puVar2;
    (**ppcVar3)();
    uVar4    = (ulong)puVar8 & 0xffff | (ulong)extraout_DX << 0x10;
    uStack14 = 0x0;
    do
    {
        if(uVar4 <= uStack14)
        {
            return;
        }
        ppcVar3 = (code **)((int)*puVar2 + 0x4);
        uVar11  = uVar4;
        (**ppcVar3)(param_5, (int)puVar2, (int)((ulong)puVar2 >> 0x10), uStack14, uVar14, uVar10);
        uVar5 = (uint)uVar11;
        uVar9 = extraout_DX_00 | uVar5;
        if(uVar9 != 0x0)
        {
            param_5 = (ushort)&USHORT_1050_1028;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
            uStack26 = CONCAT22(uVar9, uVar5);
            iVar6    = *(int *)(uVar5 + 0x34);
            if((iVar6 != 0x0) && (*(long *)(uVar5 + 0x36) != 0x0))
            {
                uVar12 = (ushort)param_1;
                uVar13 = (ushort)(param_1 >> 0x10);
                pass1_1038_201a(uVar12, uVar13, CONCAT22(uVar9, uVar5), iVar6, uVar9);
                if(iVar6 == 0x0)
                {
                    uVar11  = struct_op_1030_73a8(uStack26);
                    uVar1   = *(undefined2 *)((int)uVar11 + 0xc);
                    param_5 = 0x1008;
                    BVar7   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1);
                    if(BVar7 == 0x0)
                    {
                        param_5 = 0x1008;
                        BVar7   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x2);
                        if(BVar7 == 0x0)
                        {
                            BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x5);
                            if(BVar7 == 0x0)
                            {
                                param_5 = 0x1008;
                                BVar7   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x6);
                                if(BVar7 == 0x0)
                                    goto LAB_1038_1c76;
                            }
                            param_5 = 0x1008;
                            pass1_1038_2306(uVar12, uVar13, uStack26);
                        }
                        else
                        {
                            pass1_1038_26ee(uVar12, uVar13, uStack26, param_3, param_4, param_6);
                        }
                    }
                    else
                    {
                        pass1_1038_24e8(uVar12, uVar13, uStack26, param_3, param_4, param_6);
                    }
                }
            }
        }
    LAB_1038_1c76:
        uStack14 = uStack14 + 0x1;
    } while(true);
}


void __stdcall16far pass1_1038_1d68(ushort param_1,
                                    ushort param_2,
                                    ulong *param_3,
                                    ulong  param_4,
                                    ushort param_5,
                                    ushort param_6,
                                    ushort param_7,
                                    ulong  param_8)

{
    int        *piVar1;
    undefined2  uVar2;
    int         iVar3;
    undefined2  uVar4;
    uint        uVar5;
    code      **ppcVar6;
    ulong       uVar7;
    uint        uVar8;
    bool        bVar9;
    undefined  *puVar10;
    ulong       uVar11;
    uint        uVar12;
    uint        uVar13;
    int         iVar14;
    ulong      *puVar15;
    astruct_99 *paStack82;
    uint        uStack78;
    undefined4  uStack52;
    undefined   local_30[0x4];
    undefined4  uStack44;
    undefined4 *puStack40;
    undefined4  uStack36;
    undefined   local_20[0x4];
    undefined4 *puStack28;
    uint        uStack24;
    uint        uStack22;
    uint        uStack20;
    uint        uStack18;
    ulong       uStack16;
    ulong       uStack12;
    undefined2  uStack8;
    undefined4  uStack6;

    uStack6 = 0x64;
    uStack8 = 0x0;
    ppcVar6 = (code **)((int)*param_3 + 0x10);
    puVar15 = param_3;
    (**ppcVar6)();
    uStack12 = CONCAT22((int)param_8, param_5);
    uStack16 = 0x0;
    do
    {
        if(uStack12 <= uStack16)
        {
            return;
        }
        ppcVar6 = (code **)((int)*param_3 + 0x4);
        uVar11  = uStack12;
        uVar13  = (uint)param_8;
        (**ppcVar6)(param_6, param_3, uStack16, puVar15);
        uStack18 = uVar13;
        uVar12   = (uint)uVar11;
        uVar13   = uStack18 | uVar12;
        param_8  = (ulong)uVar13;
        uStack20 = uVar12;
        if(uVar13 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar12, uStack18);
            uStack22  = uVar13;
            param_6   = 0x1030;
            uStack24  = uVar12;
            puStack28 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uStack22, uVar12));
            param_8   = (ulong)puStack28 >> 0x10;
            puVar10   = local_20;
            ppcVar6   = (code **)((int)*puStack28 + 0x40);
            (**ppcVar6)(0x1030, (int)puStack28, (int)((ulong)puStack28 >> 0x10), puVar10, param_7);
            if(puVar10 == (undefined *)0x0)
            {
                uStack36  = pass1_1028_62c8((ulong)puStack28, param_7);
                uVar11    = uStack36 >> 0x10;
                uStack8   = 0x1;
                puStack40 = (undefined4 *)*(ulong *)((int)param_4 + 0x22);
                pass1_1008_5784((ulong *)CONCAT22(param_7, local_30), (ulong)puStack40);
                while(true)
                {
                    uVar13  = (uint)uVar11;
                    puVar10 = local_30;
                    param_6 = 0x1008;
                    pass1_1008_5b12(puVar10, param_7);
                    uStack52 = CONCAT22(uVar13, puVar10);
                    param_8  = (ulong)(uVar13 | (uint)puVar10);
                    if((uVar13 | (uint)puVar10) == 0x0)
                        break;
                    uVar2  = *(undefined2 *)(puVar10 + 0x4);
                    iVar3  = *(int *)(puVar10 + 0x6);
                    uVar4  = *(undefined2 *)(puVar10 + 0x8);
                    uVar12 = *(uint *)(puVar10 + 0xc);
                    uVar5  = *(uint *)(puVar10 + 0xa);
                    uVar8  = uVar12 / uVar5;
                    uVar11 = (ulong)uVar12 % (ulong)uVar5;
                    bVar9  = false;
                    if(((0x0 < iVar3) && (!SBORROW2(iVar3, 0x1)))
                       && ((iVar3 == 0x5 || iVar3 + -0x1 < 0x4 || (iVar3 == 0x8))))
                    {
                        bVar9 = true;
                    }
                    if(bVar9)
                    {
                        uVar11 = uStack36;
                        if(uStack6 < uStack36)
                        {
                            uVar11         = uStack6 & 0xffff;
                            uStack36._2_2_ = uStack6._2_2_;
                        }
                        uVar12  = uStack36._2_2_ | (uint)uVar11;
                        param_8 = (ulong)uVar12;
                        if(uVar12 == 0x0)
                            break;
                        uStack78 = (uint)((uVar11 & 0xffff | (ulong)uStack36._2_2_ << 0x10) / (ulong)uVar8);
                        if(uStack78 < uVar5)
                        {
                            piVar1  = (int *)(puVar10 + 0xc);
                            *piVar1 = *piVar1 - (uint)uVar11;
                            piVar1  = (int *)(puVar10 + 0xa);
                            *piVar1 = *piVar1 - uStack78;
                        }
                        else
                        {
                            ppcVar6 = (code **)((int)*puStack40 + 0xc);
                            (**ppcVar6)(0x1008, (int)puStack40, (int)((ulong)puStack40 >> 0x10), uStack52);
                            uStack44 = 0x0;
                            uStack78 = uVar5;
                        }
                        paStack82 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                        uVar12    = (uint)((ulong)paStack82 >> 0x10);
                        uVar13    = (uint)paStack82;
                        if((uVar12 | uVar13) == 0x0)
                        {
                            paStack82 = (astruct_99 *)0x0;
                        }
                        else
                        {
                            paStack82->field_0x0          = 0x389a;
                            *(undefined2 *)(uVar13 + 0x2) = 0x1008;
                            *(undefined2 *)(uVar13 + 0x4) = 0x0;
                            *(undefined2 *)(uVar13 + 0x6) = 0x0;
                            *(undefined2 *)(uVar13 + 0x8) = 0x0;
                            *(undefined2 *)(uVar13 + 0xa) = 0x0;
                            *(undefined2 *)(uVar13 + 0xc) = 0x0;
                            paStack82->field_0x0          = 0x56ce;
                            *(undefined2 *)(uVar13 + 0x2) = 0x1018;
                        }
                        uVar13                        = (uint)((ulong)paStack82 >> 0x10);
                        iVar14                        = (int)paStack82;
                        *(uint *)(iVar14 + 0xa)       = uStack78;
                        uVar7                         = (ulong)uStack78 * (ulong)uVar8;
                        uVar11                        = uVar7 >> 0x10;
                        *(undefined2 *)(iVar14 + 0xc) = (int)uVar7;
                        *(undefined2 *)(iVar14 + 0x4) = uVar2;
                        *(int *)(iVar14 + 0x6)        = iVar3;
                        *(undefined2 *)(iVar14 + 0x8) = uVar4;
                        pass1_1028_6408(
                          (ulong)puStack28, (ulong *)((ulong)paStack82 & 0xffff | (ulong)uVar13 << 0x10), param_7);
                    }
                }
            }
            else
            {
                ppcVar6 = (code **)((int)*param_3 + 0x8);
                (**ppcVar6)(0x1030, param_3, 0x0, uStack16);
            }
        }
        uStack16 = uStack16 + 0x1;
    } while(true);
}


void __stdcall16far
pass1_1038_1faa(ulong param_1, ulong *param_2, ulong *param_3, ushort param_4, ulong param_5, ushort param_6)

{
    code **ppcVar1;
    ushort uVar2;
    ulong  uVar3;
    ulong  uStack10;
    ulong  uStack6;

    ppcVar1 = (code **)((int)*param_3 + 0x10);
    (**ppcVar1)();
    uStack6  = CONCAT22((int)param_5, param_4);
    uStack10 = 0x0;
    while(true)
    {
        if(uStack6 <= uStack10)
        {
            return;
        }
        ppcVar1 = (code **)((int)*param_3 + 0x4);
        uVar3   = uStack6;
        (**ppcVar1)();
        uVar2 = (ushort)uVar3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uint)param_5);
        uVar3   = struct_op_1030_73a8(CONCAT22((int)param_5, uVar2));
        param_5 = param_5 & 0xffff0000 | uVar3 >> 0x10;
        uVar2   = (ushort)uVar3;
        pass1_1038_1d68((ushort)param_1, (ushort)(param_1 >> 0x10), param_2, uVar3, uVar2, 0x1030, param_6, param_5);
        if(uVar2 == 0x0)
            break;
        uStack10 = uStack10 + 0x1;
    }
    return;
}


void __stdcall16far
pass1_1038_201a(ushort param_1, ushort param_2, ulong param_3, undefined2 param_4, undefined2 param_5)

{
    uint        *puVar1;
    int          iVar2;
    code       **ppcVar3;
    long         lVar4;
    uint         uVar6;
    uint         uVar7;
    ushort       uVar8;
    ulong        uVar9;
    uchar       *puVar10;
    ulong        uVar11;
    ulong        uVar12;
    astruct_416 *iVar12;
    uint         uVar13;
    uchar       *puVar14;
    undefined2   uVar15;
    undefined4  *puVar16;
    undefined2   uVar17;
    long         lStack24;
    long         lStack20;
    uint         uStack10;
    astruct_413 *uVar5;

    uVar17  = (undefined2)(param_3 >> 0x10);
    uVar15  = 0x1030;
    puVar16 = (undefined4 *)pass1_1030_6b16(param_3);
    uVar6   = (uint)((ulong)puVar16 >> 0x10);
    uVar5   = (astruct_413 *)puVar16;
    if((uVar6 | (uint)uVar5) == 0x0)
    {
        return;
    }
    iVar12   = (astruct_416 *)param_3;
    iVar2    = iVar12->field_0x34;
    lVar4    = (long)iVar2;
    uVar12   = lVar4 * 0x64;
    puVar10  = (uchar *)(uVar12 >> 0x10);
    uVar7    = (uint)uVar12;
    uStack10 = 0x0;
    lStack20 = 0x0;
    if(uVar5->field_0x4 == 0x0)
    {
        if(uVar5->field_0x6 == 0x0)
        {
            if(uVar5->field_0x8 == 0x0)
                goto LAB_1038_2102;
            uVar8   = pass1_1020_c42e(uVar5->field_0x8);
            uVar11  = (ulong)uVar5->field_0xa * (ulong)uVar8;
            puVar14 = (uchar *)(uVar11 >> 0x10);
            if(uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11)
            {
                uVar11  = uVar12 & 0xffff;
                puVar14 = puVar10;
            }
            uVar12   = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar9    = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)(ulong)uVar8;
            puVar1   = &uVar5->field_0xa;
            *puVar1  = *puVar1 - (int)uVar9;
            uStack10 = (uint)((long)uVar12 / 0x64);
            uVar12   = (long)uVar12 % 0x64;
            uVar11   = uVar12;
            if(uVar12 != 0x0)
            {
                uStack10 = uStack10 + 0x1;
                uVar11   = (ulong)uStack10;
            }
            uVar7 = (uint)uVar11;
            mem_op_1000_179c(0x2a, (uchar *)uVar12, 0x1000);
            puVar10 = (uchar *)((uint)uVar12 | uVar7);
            if(puVar10 == (uchar *)0x0)
                goto LAB_1038_20fa;
            pass1_1038_6838(
              (ushort *)CONCAT22((uint)uVar12, uVar7), uVar9, uVar5->field_0x8, uStack10, iVar12->field_0x4);
        }
        else
        {
            uVar8   = switch_1020_c3b4(uVar5->field_0x6);
            uVar11  = (ulong)uVar5->field_0xa * (ulong)uVar8;
            puVar14 = (uchar *)(uVar11 >> 0x10);
            if(uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11)
            {
                uVar11  = uVar12 & 0xffff;
                puVar14 = puVar10;
            }
            uVar12   = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar9    = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)(ulong)uVar8;
            puVar1   = &uVar5->field_0xa;
            *puVar1  = *puVar1 - (int)uVar9;
            uStack10 = (uint)((long)uVar12 / 0x64);
            uVar12   = (long)uVar12 % 0x64;
            uVar11   = uVar12;
            if(uVar12 != 0x0)
            {
                uStack10 = uStack10 + 0x1;
                uVar11   = (ulong)uStack10;
            }
            uVar7 = (uint)uVar11;
            mem_op_1000_179c(0x2a, (uchar *)uVar12, 0x1000);
            puVar10 = (uchar *)((uint)uVar12 | uVar7);
            if(puVar10 == (uchar *)0x0)
                goto LAB_1038_20fa;
            pass1_1038_675c(
              (ushort *)CONCAT22((uint)uVar12, uVar7), uVar9, uVar5->field_0x6, uStack10, iVar12->field_0x4);
        }
    }
    else
    {
        uVar13  = uVar5->field_0xa;
        puVar14 = (uchar *)0x0;
        if(((int)puVar10 < 0x1) && (((uchar *)0x7fff < puVar10 || (uVar7 < uVar13))))
        {
            uVar13  = uVar7;
            puVar14 = puVar10;
        }
        lStack24 = CONCAT22(puVar14, uVar13);
        puVar1   = &uVar5->field_0xa;
        *puVar1  = *puVar1 - uVar13;
        uStack10 = (uint)(lStack24 / 0x64);
        uVar11   = lStack24 % 0x64;
        uVar12   = uVar11;
        if(uVar11 != 0x0)
        {
            uStack10 = uStack10 + 0x1;
            uVar12   = (ulong)uStack10;
        }
        uVar7 = (uint)uVar12;
        mem_op_1000_179c(0x2a, (uchar *)uVar11, 0x1000);
        puVar10 = (uchar *)((uint)uVar11 | uVar7);
        if(puVar10 == (uchar *)0x0)
        {
        LAB_1038_20fa:
            uVar15   = 0x1000;
            lStack20 = 0x0;
            goto LAB_1038_2102;
        }
        pass1_1038_6590((ushort *)CONCAT22((uint)uVar11, uVar7),
                        uVar13,
                        (ushort)puVar14,
                        uVar5->field_0x4,
                        uStack10,
                        iVar12->field_0x4);
    }
    uVar15   = 0x1000;
    lStack20 = CONCAT22(puVar10, uVar7);
LAB_1038_2102:
    if(lStack20 != 0x0)
    {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        uVar15 = 0x1030;
        uVar7  = uStack10;
        pass1_1030_6c4c(param_3, iVar2 - uStack10);
    }
    if(uVar5->field_0xa == 0x0)
    {
        if((uVar6 | (uint)uVar5) != 0x0)
        {
            ppcVar3 = (code **)*puVar16;
            (**ppcVar3)(uVar15, uVar5, uVar6, 0x1);
        }
    }
    else
    {
        pass1_1030_6c66(param_3, 0x0, (ulong)puVar16, uVar7, puVar10, 0x1030);
    }
    return;
}


void __stdcall16far pass1_1038_01c0(ushort param_1, ushort param_2, ulong param_3, ushort param_4)

{
    int         iVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    ulong       uVar4;
    ulong       uVar5;
    BOOL16      BVar6;
    undefined  *puVar7;
    undefined4 *puVar8;
    ulong       uVar9;
    uchar      *puVar10;
    undefined2  extraout_DX;
    uint        extraout_DX_00;
    uint        uVar11;
    uint        uVar12;
    undefined2  uVar13;
    undefined2  uVar14;
    ulong      *puVar15;
    ulong       uVar16;
    ulong       uVar17;
    undefined   uVar18;
    ulong       uStack50;
    ulong       uStack30;
    ulong       uStack18;
    undefined   local_e[0x2];
    undefined4 *puStack12;
    uint        uStack8;
    uchar      *puStack6;
    int         iStack4;

    iStack4  = 0x0;
    puVar15  = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x29);
    puVar10  = (uchar *)((ulong)puVar15 >> 0x10);
    uVar12   = (uint)puVar15;
    uStack8  = uVar12;
    puStack6 = puVar10;
    pass1_1038_4e78(uVar12, puVar10, param_3, puVar15);
    puStack12 = (undefined4 *)CONCAT22(puVar10, uVar12);
    uVar14    = 0x1030;
    uVar16    = pass1_1030_bcae((ushort)local_e, param_4);
    uVar13    = (undefined2)uVar16;
    ppcVar3   = (code **)((int)*puStack12 + 0x10);
    (**ppcVar3)(0x1030, (int)puStack12, (int)((ulong)puStack12 >> 0x10));
    uStack18 = CONCAT22(extraout_DX, uVar13);
    uVar13   = (undefined2)(param_3 >> 0x10);
    puVar2   = (undefined4 *)*(ulong *)((int)param_3 + 0xc);
    uVar13   = *(undefined2 *)((int)param_3 + 0xe);
    uVar18   = SUB41(puVar2, 0x0);
    ppcVar3  = (code **)((int)*puVar2 + 0x10);
    puVar8   = puVar2;
    (**ppcVar3)();
    uVar16   = (ulong)puVar8 & 0xffff | (ulong)extraout_DX_00 << 0x10;
    uStack30 = 0x0;
    uVar12   = extraout_DX_00;
    do
    {
        if(uStack18 <= uStack30)
        {
            if(puStack12 != (undefined4 *)0x0)
            {
                ppcVar3 = (code **)*puStack12;
                (**ppcVar3)(uVar14, (int)puStack12, (int)((ulong)puStack12 >> 0x10), 0x1, uVar18, uVar13);
            }
            return;
        }
        uVar14 = 0x1030;
        uVar9  = uStack18;
        pass1_1030_1d58((ulong)puStack12);
        uVar5  = (ulong)uVar12;
        iVar1  = *(int *)((int)uVar9 + 0x10);
        uVar11 = uVar12;
        for(uStack50 = 0x0; uVar12 = uVar11, uStack50 < uVar16; uStack50 = uStack50 + 0x1)
        {
            uVar14 = 0x1030;
            uVar17 = uVar16;
            pass1_1030_1d58((ulong)puVar2);
            uVar4  = uVar17 & 0xffff | (ulong)uVar11 << 0x10;
            uVar12 = uVar11 | (uint)uVar17;
            if((uVar12 != 0x0) && (uVar12 = uVar11, *(int *)((uint)uVar17 + 0x10) == iVar1))
            {
                uVar17 = struct_op_1030_73a8(uVar4);
                uVar12 = (uint)(uVar17 >> 0x10);
                uVar14 = 0x1008;
                BVar6  = pass1_1008_c6ae((ulong)_PTR_LOOP_1050_06e0, *(undefined2 *)((int)uVar17 + 0xc), 0x30);
                if(BVar6 == 0x0)
                {
                    puVar7 = local_e;
                    uVar14 = 0x1030;
                    pass1_1030_bd74((ushort)puVar7, param_4, uVar4, uVar9 & 0xffff | uVar5 << 0x10, param_4);
                    if((int)puVar7 < 0x6)
                    {
                        iStack4 = iStack4 + 0x1;
                        break;
                    }
                }
            }
            uVar11 = uVar12;
        }
        uStack30 = uStack30 + 0x1;
    } while(true);
}


void __stdcall16far pass1_1038_0e00(ulong param_1, ulong *param_2, ulong param_3, ushort param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined2 extraout_DX;
    uint       extraout_DX_00;
    uint       uVar2;
    uint       uVar3;
    ulong      uVar4;
    ulong      uStack10;
    ulong      uStack6;

    ppcVar1 = (code **)((int)*param_2 + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX, param_4);
    for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
    {
        ppcVar1 = (code **)((int)*param_2 + 0x4);
        uVar4   = uStack6;
        (**ppcVar1)();
        uVar3 = (uint)uVar4;
        uVar2 = extraout_DX_00 | uVar3;
        if(uVar2 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_00);
            uVar4 = struct_op_1030_73a8(CONCAT22(uVar2, uVar3));
            uVar3 = (uint)(uVar4 >> 0x10);
            if((uVar3 | (uint)uVar4) != 0x0)
            {
                pass1_1038_0d8e(
                  (ushort)param_1, (ushort)(param_1 >> 0x10), uVar4 & 0xffff | (ulong)uVar3 << 0x10, param_3, param_5);
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_0e78(ulong param_1, ulong param_2, ushort param_3)

{
    code      **ppcVar1;
    uint        uVar2;
    uint        uVar3;
    uint        uVar4;
    uchar      *puVar5;
    uint        extraout_DX;
    uchar      *puVar6;
    undefined2  extraout_DX_00;
    uint        extraout_DX_01;
    uint        uVar7;
    undefined2  uVar8;
    ulong      *puVar9;
    ulong       uVar10;
    ulong       uStack22;
    ulong       uStack18;
    undefined4 *puStack14;
    ulong      *puStack10;

    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
    puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
    uVar2  = (uint)puVar9;
    pass1_1038_4d6e(param_2, puVar9, uVar2, puVar5);
    puStack10 = (ulong *)CONCAT22(puVar5, uVar2);
    uVar10    = *puStack10;
    ppcVar1   = (code **)uVar10 + 0x8;
    uVar3     = uVar2;
    (**ppcVar1)(0x1008, uVar2, puVar5);
    if((extraout_DX | uVar3) == 0x0)
    {
        if(puStack10 != (ulong *)0x0)
        {
            ppcVar1 = (code **)uVar10;
            (**ppcVar1)(0x8, uVar2, (char)puVar5, 0x1);
            return;
        }
    }
    else
    {
        uVar8  = 0x1008;
        puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
        uVar3  = (uint)puVar9;
        pass1_1038_4d6e(param_2, puVar9, uVar3, puVar6);
        puStack14 = (undefined4 *)CONCAT22(puVar6, uVar3);
        ppcVar1   = (code **)((int)*puStack14 + 0x10);
        uVar4     = uVar3;
        (**ppcVar1)(0x1008, (char)uVar3, puVar6);
        uStack18 = CONCAT22(extraout_DX_00, uVar4);
        for(uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1)
        {
            ppcVar1 = (code **)((int)*puStack14 + 0x4);
            uVar10  = uStack18;
            (**ppcVar1)();
            uVar4 = (uint)uVar10;
            uVar7 = extraout_DX_01 | uVar4;
            if(uVar7 != 0x0)
            {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_01);
                uVar8  = 0x1030;
                uVar10 = struct_op_1030_73a8(CONCAT22(uVar7, uVar4));
                if(((uint)(uVar10 >> 0x10) | (uint)uVar10) != 0x0)
                {
                    pass1_1038_0e00(param_1, puStack10, uVar10, (uint)uVar10, param_3);
                }
            }
        }
        if(puStack10 != (ulong *)0x0)
        {
            ppcVar1 = (code **)*puStack10;
            (**ppcVar1)(uVar8, uVar2, (char)puVar5, 0x1);
        }
        if(puStack14 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack14;
            (**ppcVar1)(uVar8, uVar3, (char)puVar6, 0x1);
        }
    }
    return;
}


void __stdcall16far pass1_1038_0f8c(ushort param_1,
                                    ushort param_2,
                                    ulong *param_3,
                                    ulong  param_4,
                                    ushort param_5,
                                    ulong  param_6,
                                    ushort param_7,
                                    ushort param_8)

{
    int        *piVar1;
    undefined2  uVar2;
    undefined2  uVar3;
    undefined2  uVar4;
    code      **ppcVar5;
    ulong       uVar6;
    qword       qVar7;
    undefined  *puVar8;
    ulong       uVar9;
    uint        uVar10;
    uint        uVar11;
    uint        uVar12;
    int         iVar13;
    undefined2  uVar14;
    astruct_99 *paStack80;
    uint        uStack76;
    undefined   local_30[0x4];
    undefined4  uStack44;
    undefined4 *puStack40;
    undefined4  uStack36;
    undefined   local_20[0x4];
    undefined4 *puStack28;
    uint        uStack24;
    uint        uStack22;
    uint        uStack20;
    uint        uStack18;
    ulong       uStack16;
    ulong       uStack12;
    undefined2  uStack8;
    undefined4  uStack6;

    uStack6 = 0x64;
    uStack8 = 0x0;
    ppcVar5 = (code **)((int)*param_3 + 0x10);
    (**ppcVar5)(param_7, param_3);
    uStack12 = CONCAT22((int)param_6, param_5);
    uStack16 = 0x0;
    do
    {
        if(uStack12 <= uStack16)
        {
            return;
        }
        ppcVar5 = (code **)((int)*param_3 + 0x4);
        uVar9   = uStack12;
        uVar11  = (uint)param_6;
        (**ppcVar5)(param_7, (char)param_3, (int)((ulong)param_3 >> 0x10), (char)uStack16, (int)(uStack16 >> 0x10));
        uStack18 = uVar11;
        uVar12   = (uint)uVar9;
        uVar11   = uStack18 | uVar12;
        param_6  = (ulong)uVar11;
        uStack20 = uVar12;
        if(uVar11 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar12, uStack18);
            uStack22  = uVar11;
            param_7   = 0x1030;
            uStack24  = uVar12;
            puStack28 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uStack22, uVar12));
            param_6   = (ulong)puStack28 >> 0x10;
            puVar8    = local_20;
            ppcVar5   = (code **)((int)*puStack28 + 0x40);
            (**ppcVar5)(0x1030, (char)puStack28, (int)((ulong)puStack28 >> 0x10), (char)puVar8, param_8);
            if(puVar8 == (undefined *)0x0)
            {
                uStack36  = pass1_1028_62c8((ulong)puStack28, param_8);
                uVar9     = uStack36 >> 0x10;
                uStack8   = 0x1;
                puStack40 = (undefined4 *)*(ulong *)((int)param_4 + 0x22);
                pass1_1008_5784((ulong *)CONCAT22(param_8, local_30), (ulong)puStack40);
                while(true)
                {
                    uVar11  = (uint)uVar9;
                    puVar8  = local_30;
                    param_7 = 0x1008;
                    pass1_1008_5b12((char)puVar8, param_8);
                    param_6 = (ulong)(uVar11 | (uint)puVar8);
                    if((uVar11 | (uint)puVar8) == 0x0)
                        break;
                    uVar2  = *(undefined2 *)(puVar8 + 0x4);
                    uVar3  = *(undefined2 *)(puVar8 + 0x6);
                    uVar4  = *(undefined2 *)(puVar8 + 0x8);
                    uVar12 = *(uint *)(puVar8 + 0xa);
                    uVar6  = (ulong) * (uint *)(puVar8 + 0xc) / (ulong)uVar12;
                    uVar9  = uStack36;
                    if(uStack6 < uStack36)
                    {
                        uVar9          = uStack6 & 0xffff;
                        uStack36._2_2_ = uStack6._2_2_;
                    }
                    uVar10  = uStack36._2_2_ | (uint)uVar9;
                    param_6 = (ulong)uVar10;
                    if(uVar10 == 0x0)
                        break;
                    qVar7    = (qword)(uVar9 & 0xffff | (ulong)uStack36._2_2_ << 0x10) / (qword)uVar6;
                    param_6  = (ulong)qVar7 >> 0x10;
                    uStack76 = (uint)qVar7;
                    if(uStack76 == 0x0)
                        break;
                    if(uStack76 < uVar12)
                    {
                        piVar1  = (int *)(puVar8 + 0xc);
                        *piVar1 = *piVar1 - (uint)uVar9;
                        piVar1  = (int *)(puVar8 + 0xa);
                        *piVar1 = *piVar1 - uStack76;
                    }
                    else
                    {
                        ppcVar5 = (code **)((int)*puStack40 + 0xc);
                        (**ppcVar5)(0x1008, (char)puStack40, (int)((ulong)puStack40 >> 0x10), (char)puVar8, uVar11);
                        uStack44 = 0x0;
                        uStack76 = uVar12;
                    }
                    paStack80 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                    uVar12    = (uint)((ulong)paStack80 >> 0x10);
                    uVar11    = (uint)paStack80;
                    if((uVar12 | uVar11) == 0x0)
                    {
                        paStack80 = (astruct_99 *)0x0;
                    }
                    else
                    {
                        paStack80->field_0x0          = 0x389a;
                        *(undefined2 *)(uVar11 + 0x2) = 0x1008;
                        *(undefined2 *)(uVar11 + 0x4) = 0x0;
                        *(undefined2 *)(uVar11 + 0x6) = 0x0;
                        *(undefined2 *)(uVar11 + 0x8) = 0x0;
                        *(undefined2 *)(uVar11 + 0xa) = 0x0;
                        *(undefined2 *)(uVar11 + 0xc) = 0x0;
                        paStack80->field_0x0          = 0x56ce;
                        *(undefined2 *)(uVar11 + 0x2) = 0x1018;
                    }
                    uVar14                        = (undefined2)((ulong)paStack80 >> 0x10);
                    iVar13                        = (int)paStack80;
                    *(uint *)(iVar13 + 0xa)       = uStack76;
                    uVar6                         = uStack76 * uVar6;
                    uVar9                         = uVar6 >> 0x10;
                    *(undefined2 *)(iVar13 + 0xc) = (int)uVar6;
                    *(undefined2 *)(iVar13 + 0x4) = uVar2;
                    *(undefined2 *)(iVar13 + 0x6) = uVar3;
                    *(undefined2 *)(iVar13 + 0x8) = uVar4;
                    pass1_1028_6408((ulong)puStack28, (ulong *)paStack80, param_8);
                }
            }
            else
            {
                ppcVar5 = (code **)((int)*param_3 + 0x8);
                (**ppcVar5)(0x1030, param_3, 0x0, 0x0, (char)uStack16, (int)(uStack16 >> 0x10));
            }
        }
        uStack16 = uStack16 + 0x1;
    } while(true);
}


astruct_100 *__stdcall16far pass1_1030_e09e(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x2af7);
    param_1->field_0x0                  = 0xe2ae;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCAiInput_1050_5972);
    return param_1;
}


void __stdcall16far
struct_1030_e2be(astruct_100 *param_1, ushort param_2, ulong param_3, ulong param_4, ushort param_5, uchar param_6)

{
    astruct_217 *iVar1;
    undefined2   uVar1;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x2af7);
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    iVar1              = (astruct_217 *)param_1;
    iVar1->field_0x108 = param_4;
    iVar1->field_0x10c = param_3;
    iVar1->field_0x110 = param_2;
    param_1->field_0x0 = 0xe4ea;
    iVar1->field_0x2   = 0x1030;
    return;
}


astruct_100 *__stdcall16far pass1_1030_e63e(astruct_100 *param_1, ushort param_2, ushort param_3, uchar param_4)

{
    int        iVar1;
    undefined2 uVar2;

    iVar1 = (int)param_1;
    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    struct_op_1028_d1dc(param_3, param_4, param_1, 0xf9f);
    *(ushort *)(iVar1 + 0x108)   = param_2;
    param_1->field_0x0           = 0xe78a;
    *(undefined2 *)(iVar1 + 0x2) = 0x1030;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x8)), s_SCKillColony_1050_5990);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1030_e79a(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f);
    param_1->field_0x0                  = 0xe890;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),
                         s_SCKillRebelColony_1050_599e);
    return param_1;
}


void __stdcall16far pass1_1030_ea50(ulong param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5)

{
    undefined4 uVar1;
    BOOL16     BVar2;
    int        iVar3;
    undefined2 uVar4;
    ulong      uVar5;
    ulong      local_12;
    uint       local_e;
    int        iStack12;
    uint       uStack10;
    uint       uStack8;
    undefined4 uStack6;

    uStack6 = 0x1869f;
    uVar4   = (undefined2)(param_1 >> 0x10);
    iVar3   = (int)param_1;
    BVar2   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(iVar3 + 0x110), 0x3);
    if(BVar2 != 0x0)
    {
        uVar5    = struct_op_1030_73a8(param_2);
        iStack12 = (int)(uVar5 >> 0x10);
        local_e  = (uint)uVar5;
        uStack6  = pass1_1028_45e2(uVar5, local_e, iStack12, param_5);
    }
    uVar1    = *(undefined4 *)(iVar3 + 0x108);
    uStack8  = *(uint *)((int)uVar1 + 0x4);
    uStack10 = 0x0;
    while(true)
    {
        if(uStack8 <= uStack10)
        {
            return;
        }
        pass1_1020_bb16(*(ulong **)(iVar3 + 0x108),
                        (ulong *)CONCAT22(param_5, &local_12),
                        (ushort *)CONCAT22(param_5, &local_e),
                        uStack10);
        if(uStack6 < local_12)
        {
            pass1_1030_7ddc(param_2, uStack6, local_e, (uint)uStack6, uStack6._2_2_, param_3, param_4, param_5);
            uStack6 = 0x0;
        }
        else
        {
            uStack6 = uStack6 - local_12;
            pass1_1030_7ddc(param_2, local_12, local_e, (uint)local_12, uStack6._2_2_, param_3, param_4, param_5);
        }
        if(((uint)uStack6._2_2_ | (uint)uStack6) == 0x0)
            break;
        uStack10 = uStack10 + 0x1;
    }
    return;
}


astruct_100 *__stdcall16far pass1_1030_eb50(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x1f3f);
    param_1->field_0x0                  = 0xecb2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCMines_1050_59c6);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1030_ecc2(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f);
    param_1->field_0x0                  = 0xb96;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCMorale_1050_59ce);
    return param_1;
}


ushort *__stdcall16far struct_1030_d8f6(ushort *param_1)

{
    astruct_184 *iVar1;
    undefined2   uVar1;

    struct_1028_b354(param_1);
    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_184 *)param_1;
    *param_1         = 0xdc2e;
    iVar1->field_0x2 = 0x1030;
    if(iVar1->field_0xc == 0x4c)
    {
        iVar1->field_0xe = 0x43;
    }
    else
    {
        if(iVar1->field_0xc == 0x4d)
        {
            iVar1->field_0xe = 0x44;
        }
        else
        {
            iVar1->field_0xe = 0x45;
        }
    }
    return param_1;
}


ushort *__stdcall16far struct_1030_dc96(ushort *param_1)

{
    undefined2 uVar1;

    struct_1028_b354(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x20) = 0x0;
    *param_1                             = 0xe036;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1030;
    return param_1;
}
