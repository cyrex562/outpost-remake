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
