
astruct_18 *__stdcall16far pass1_1028_7472(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_816e(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_8342(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_6850(astruct_18 *param_1, byte param_2)

{
    pass1_1028_6186(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_6a7a(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_6aa6(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_6e24(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_6f84(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1028_504a(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far
pass1_1028_5128(ushort param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5, uchar param_6)

{
    int       *piVar1;
    undefined2 extraout_DX;
    int       *piVar2;
    ushort     uVar3;
    ushort    *puVar4;
    ushort     uVar5;
    undefined  local_14e[0x124];
    ulong      uStack42;
    undefined4 uStack38;
    int        local_22;
    undefined  local_20[0x2];
    undefined  local_1e[0x2];
    ulong      local_1c;
    int        iStack24;
    undefined4 uStack22;
    int       *piStack18;
    undefined2 uStack16;
    int        local_e;
    ushort     local_c;
    ulong      uStack10;
    ushort    *puStack6;

    pass1_1028_bd38(CONCAT22(param_2, param_1), (ushort)param_3, param_5);
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_5, param_3, param_4);
    uStack16 = (undefined2)((ulong)puStack6 >> 0x10);
    uStack10 = *(ulong *)((int)puStack6 + 0x20);
    puVar4   = &local_c;
    piVar1   = &local_e;
    piVar2   = piVar1;
    uVar3    = param_5;
    uVar5    = param_5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack10, (uint)(uStack10 >> 0x10));
    piStack18 = piVar1;
    pass1_1030_5b1c(CONCAT22(uStack16, piVar1), (ushort *)CONCAT22(uVar3, piVar2), (ushort *)CONCAT22(uVar5, puVar4));
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    uStack22 = CONCAT22(extraout_DX, piVar1);
    local_1c = *(ulong *)(piVar1 + 0x6);
    iStack24 = piVar1[0x8];
    pass1_1028_c8ee(param_5, param_1, param_2, 0x1, (ushort *)CONCAT22(param_5, &local_1c));
    pass1_1008_3eb4((ushort *)CONCAT22(param_5, &local_1c),
                    (ushort *)CONCAT22(param_5, &local_22),
                    (ushort *)CONCAT22(param_5, local_20),
                    (ushort *)CONCAT22(param_5, local_1e));
    if(local_e < local_22)
    {
        pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
        pass1_1030_5b1c(
          CONCAT22(uStack16, piStack18), (ushort *)CONCAT22(param_5, &local_e), (ushort *)CONCAT22(param_5, &local_c));
    }
    uStack38 = *(undefined4 *)((int)uStack22 + 0x2e);
    uStack42 = *(ulong *)((int)uStack38 + 0x4);
    struct_op_1028_87f0(param_5,
                        param_6,
                        (astruct_97 *)CONCAT22(param_5, local_14e),
                        0x0,
                        0x0,
                        0x6f,
                        &local_1c,
                        param_5,
                        uStack42,
                        uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_5, local_14e));
    pass1_1028_ccd0(param_6, param_5, CONCAT22(param_2, param_1), (ushort *)CONCAT22(param_5, &local_1c));
    return;
}


astruct_18 *__stdcall16far pass1_1028_525a(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


ulong __stdcall16far pass1_1028_533c(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


astruct_18 *__stdcall16far pass1_1028_5496(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_55a2(astruct_18 *param_1, byte param_2)

{
    pass1_1028_0138(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


ulong __stdcall16far pass1_1028_568a(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


ulong __stdcall16far pass1_1028_571c(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


ulong __stdcall16far pass1_1028_57fa(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}
