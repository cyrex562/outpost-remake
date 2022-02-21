
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


void __stdcall16far pass1_1028_5128(ushort param_1, ushort param_2, uchar *param_3, int param_4, ushort param_5, uchar param_6)

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
    pass1_1008_3eb4((ushort *)CONCAT22(param_5, &local_1c), (ushort *)CONCAT22(param_5, &local_22), (ushort *)CONCAT22(param_5, local_20), (ushort *)CONCAT22(param_5, local_1e));
    if(local_e < local_22)
    {
        pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
        pass1_1030_5b1c(CONCAT22(uStack16, piStack18), (ushort *)CONCAT22(param_5, &local_e), (ushort *)CONCAT22(param_5, &local_c));
    }
    uStack38 = *(undefined4 *)((int)uStack22 + 0x2e);
    uStack42 = *(ulong *)((int)uStack38 + 0x4);
    struct_op_1028_87f0(param_5, param_6, (astruct_97 *)CONCAT22(param_5, local_14e), 0x0, 0x0, 0x6f, &local_1c, param_5, uStack42, uStack10);
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


ulong __stdcall16far pass1_1028_58dc(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


ulong __stdcall16far pass1_1028_59be(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


astruct_18 *__stdcall16far pass1_1028_5bc6(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1028_5d12(ushort param_1, int param_2, ushort param_3, uchar param_4)

{
    ulong      uVar1;
    undefined4 uVar2;
    undefined2 extraout_DX;

    pass1_1028_b58e(*(ulong *)(param_2 + 0x6));
    *(ushort *)(param_2 + -0x4)     = param_1;
    *(undefined2 *)(param_2 + -0x2) = extraout_DX;
    uVar2                           = *(undefined4 *)(param_2 + -0x4);
    *(undefined4 *)(param_2 + -0x8) = *(undefined4 *)((int)uVar2 + 0x2e);
    uVar2                           = *(undefined4 *)(param_2 + -0x8);
    uVar1                           = *(ulong *)((int)uVar2 + 0x4);
    *(ulong *)(param_2 + -0xc)      = uVar1;
    pass1_1028_68de((astruct_100 *)CONCAT22(param_3, param_2 + -0x11a), 0x1, uVar1, param_4, param_3);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_3, param_2 + -0x11a));
    *(undefined2 *)(param_2 + -0x11a) = 0x389a;
    *(undefined2 *)(param_2 + -0x118) = 0x1008;
    return;
}


astruct_18 *__stdcall16far pass1_1028_5d68(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


ulong __stdcall16far pass1_1028_5e4e(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


astruct_18 *__stdcall16far pass1_1028_602e(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1028_6186(ushort *param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_603 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_603 *)param_1;
    *param_1         = 0x6876;
    iVar4->field_0x2 = (int)&USHORT_1050_1028;
    puVar1           = iVar4->field_0x20;
    uVar2            = iVar4->field_0x22;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1028_b418(param_1);
    return;
}


void __stdcall16far pass1_1028_61c4(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    uint         uVar4;
    astruct_21  *paVar5;
    undefined4   uVar6;
    undefined2   uVar7;
    astruct_315 *iVar7;

    iVar7 = (astruct_315 *)param_1;
    uVar7 = (undefined2)(param_1 >> 0x10);
    pass1_1028_b46e(param_1, param_2, param_3);
    puVar1 = iVar7->field_0x20;
    uVar2  = iVar7->field_0x22;
    uVar4  = uVar2 | (uint)puVar1;
    paVar5 = (astruct_21 *)CONCAT22(uVar4, puVar1);
    if(uVar4 != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        paVar5  = (astruct_21 *)(**ppcVar3)();
    }
    mem_op_1000_179c(0xc, (uchar *)((ulong)paVar5 >> 0x10), 0x1000);
    if(paVar5 == (astruct_21 *)0x0)
    {
        uVar6 = 0x0;
    }
    else
    {
        uVar6 = set_struct_1008_574a(paVar5);
    }
    iVar7->field_0x20 = (undefined4 *)uVar6;
    iVar7->field_0x22 = (uint)((ulong)uVar6 >> 0x10);
    return;
}


void __stdcall16far pass1_1028_6228(ulong param_1, uint param_2, int param_3, int param_4, ushort param_5)

{
    uint       uVar1;
    uint       uVar2;
    code     **ppcVar3;
    int        iVar4;
    undefined2 uVar5;
    int        iVar6;
    undefined2 uVar7;
    bool       bVar8;
    long       lVar9;
    undefined  local_a[0x4];
    undefined4 uStack6;

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    pass1_1008_5784((ulong *)CONCAT22(param_5, local_a), *(ulong *)(iVar6 + 0x20));
    while(true)
    {
        do
        {
            lVar9 = pass1_1008_5b12(local_a, param_5);
            uVar5 = (undefined2)((ulong)lVar9 >> 0x10);
            iVar4 = (int)lVar9;
            if(lVar9 == 0x0)
            {
                return;
            }
        } while(*(int *)(iVar4 + 0x6) != param_4);
        uVar1 = *(uint *)(iVar4 + 0xa);
        if((param_3 == 0x0) && (param_2 < uVar1))
            break;
        bVar8   = param_2 < uVar1;
        param_2 = param_2 - uVar1;
        param_3 = param_3 - (uint)bVar8;
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x20) + 0xc);
        (**ppcVar3)(0x1008, *(undefined4 *)(iVar6 + 0x20));
        uStack6 = 0x0;
    }
    uVar2                 = *(uint *)(iVar4 + 0xc);
    *(int *)(iVar4 + 0xa) = uVar1 - param_2;
    *(int *)(iVar4 + 0xc) = -(param_2 * (uVar2 / uVar1) - *(int *)(iVar4 + 0xc));
    return;
}


astruct_18 *__stdcall16far pass1_1028_3fde(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


ulong __stdcall16far pass1_1028_42ca(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


astruct_18 *__stdcall16far pass1_1028_4444(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_4810(astruct_18 *param_1, byte param_2)

{
    pass1_1028_4530(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


ulong __stdcall16far pass1_1028_4920(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


astruct_18 *__stdcall16far pass1_1028_4af6(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1028_2f18(ushort param_1, int param_2, uchar param_3, ulong param_4)

{
    int        iVar1;
    ulong     *puVar2;
    undefined2 extraout_DX;
    uint       uVar3;
    undefined2 uVar4;
    ushort    *puVar5;
    undefined  local_944[0x124];
    undefined  local_820[0x124];
    undefined  local_6fc[0x124];
    undefined  local_5d8[0x124];
    undefined  local_4b4[0x124];
    ulong      local_390;
    undefined  local_38a[0x124];
    undefined  local_266[0x124];
    undefined  local_142[0x124];
    undefined4 local_1e;
    ushort     local_1a;
    ulong      local_18;
    undefined2 uStack20;
    ulong      uStack18;
    undefined4 uStack14;
    undefined4 uStack10;
    ulong      uStack6;

    uStack6 = pass1_1028_bb24(param_4);
    iVar1   = (int)uStack6;
    pass1_1028_b58e(param_4);
    uStack10 = CONCAT22(extraout_DX, iVar1);
    uStack14 = *(undefined4 *)(iVar1 + 0x2e);
    uStack18 = *(ulong *)((int)uStack14 + 0x4);
    local_18 = *(ulong *)(iVar1 + 0xc);
    uStack20 = *(undefined2 *)(iVar1 + 0x10);
    pass1_1008_3eb4((ushort *)CONCAT22(param_1, &local_18), (ushort *)CONCAT22(param_1, &local_1e), (ushort *)CONCAT22(param_1, (int)&local_1e + 0x2), (ushort *)CONCAT22(param_1, &local_1a));
    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_18), (ushort)local_1e, local_1e._2_2_ - 0x1, local_1a - 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_142), 0x0, 0x0, 0xd, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_142));
    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_18), (ushort)local_1e, local_1e._2_2_ + 0x1, local_1a + 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_266), 0x0, 0x0, 0xc, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_266));
    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_18), (ushort)local_1e, local_1e._2_2_ + 0x1, local_1a - 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_38a), 0x0, 0x0, 0xe, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_38a));
    puVar5 = pass1_1008_3e54((ushort *)CONCAT22(param_1, &local_390), (ushort)local_1e, local_1e._2_2_ - 0x1, local_1a + 0x1);
    uVar3  = (uint)((ulong)puVar5 >> 0x10);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_4b4), 0x0, 0x0, 0xb, &local_390, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_4b4));
    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_18), (ushort)local_1e, local_1e._2_2_ - 0x1, local_1a);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_5d8), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_5d8));
    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_18), (ushort)local_1e, (ushort)((ulong)local_1e >> 0x10), local_1a + 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_6fc), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_6fc));
    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_18), (ushort)local_1e, local_1e._2_2_ + 0x1, local_1a);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_820), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_820));
    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_18), (ushort)local_1e, (ushort)((ulong)local_1e >> 0x10), local_1a - 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_944), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_944));
    puVar2 = &local_390;
    pass1_1030_627e(param_1, (uint)puVar2, uVar3, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_1, puVar2), uStack6);
    uVar4                             = (undefined2)((ulong)uStack14 >> 0x10);
    *(ulong **)((int)uStack14 + 0x10) = puVar2;
    *(uint *)((int)uStack14 + 0x12)   = uVar3;
    return;
}


astruct_18 *__stdcall16far pass1_1028_33f6(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_34d0(astruct_18 *param_1, byte param_2)

{
    pass1_1028_0138(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_35e2(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_3718(astruct_18 *param_1, byte param_2)

{
    pass1_1028_388e(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1028_388e(ushort *param_1)

{
    uint        uVar1;
    astruct_18 *paVar2;
    int         iVar3;
    undefined2  uVar4;

    uVar4                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                        = (int)param_1;
    *param_1                     = 0x3e2c;
    *(undefined2 *)(iVar3 + 0x2) = (int)&USHORT_1050_1028;
    paVar2                       = *(astruct_18 **)(iVar3 + 0x28);
    uVar1                        = *(uint *)(iVar3 + 0x2a);
    if((uVar1 | (uint)paVar2) != 0x0)
    {
        fn_ptr_1020_ba7e((ulong *)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10));
        fn_ptr_1000_17ce(paVar2, 0x1000);
    }
    pass1_1028_b418(param_1);
    return;
}


astruct_18 *__stdcall16far pass1_1028_3e06(astruct_18 *param_1, byte param_2)

{
    pass1_1028_388e(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1028_199a(ushort param_1, int param_2, uchar param_3, ulong param_4)

{
    int       *piVar1;
    ulong      uVar2;
    uchar     *extraout_DX;
    uchar     *puVar3;
    undefined2 uVar4;
    int       *piVar5;
    ushort     uVar6;
    ushort    *puVar7;
    ushort     uVar8;
    undefined2 local_15e;
    undefined2 uStack348;
    ulong     *puStack50;
    ulong      uStack42;
    undefined2 uStack38;
    int       *piStack36;
    int        local_22;
    ushort     local_20;
    ulong      uStack30;
    ushort    *puStack26;
    int        local_16;
    undefined4 local_14;
    ulong      local_10;
    undefined2 uStack12;
    ulong      uStack10;
    int        iStack6;
    uchar     *puStack4;

    piVar1  = (int *)((int)param_4 + 0x14);
    *piVar1 = *piVar1 + -0x1;
    if(*piVar1 == 0x0)
    {
        pass1_1028_b58e(param_4);
        uStack10 = *(ulong *)(param_2 + 0x2e);
        iStack6  = param_2;
        puStack4 = extraout_DX;
        pass1_1038_5804(uStack10, 0x1, 0x3);
        local_10  = *(ulong *)(iStack6 + 0xc);
        uStack12  = *(undefined2 *)(iStack6 + 0x10);
        puStack50 = &local_10;
        puVar3    = puStack4;
        pass1_1008_3eb4((ushort *)CONCAT22(param_1, &local_10), (ushort *)CONCAT22(param_1, &local_16), (ushort *)CONCAT22(param_1, &local_14), (ushort *)CONCAT22(param_1, (int)&local_14 + 0x2));
        puStack26 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_1, puVar3, (int)&uStack10);
        uVar2     = *(ulong *)((int)puStack26 + 0x20);
        puVar7    = &local_20;
        piStack36 = &local_22;
        piVar5    = piStack36;
        uVar6     = param_1;
        uVar8     = param_1;
        uStack30  = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)(uVar2 >> 0x10));
        uStack38 = (undefined2)uVar2;
        pass1_1030_5b1c(uVar2 & 0xffff | ZEXT24(piStack36) << 0x10, (ushort *)CONCAT22(uVar6, piVar5), (ushort *)CONCAT22(uVar8, puVar7));
        if(local_22 < local_16 + 0x1)
        {
            pass1_1030_5b3e(CONCAT22(piStack36, uStack38), local_16 + 0x1, local_20);
            pass1_1030_5b1c(CONCAT22(piStack36, uStack38), (ushort *)CONCAT22(param_1, &local_22), (ushort *)CONCAT22(param_1, &local_20));
        }
        uVar4    = (undefined2)(uStack10 >> 0x10);
        uStack42 = *(ulong *)((int)uStack10 + 0x4);
        struct_op_1028_87f0(
          param_1, param_3, (astruct_97 *)CONCAT22(param_1, &local_15e), 0x0, 0x0, (-(uint)(local_16 == 0x0) & 0xffd3) + 0x60, &local_10, param_1, uStack42 & 0xffff | (ulong) * (uint *)((int)uStack10 + 0x6) << 0x10, uStack30);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, &local_15e));
        local_15e = 0x389a;
        uStack348 = 0x1008;
        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_10), local_16 + 0x1, (ushort)local_14, (ushort)((ulong)local_14 >> 0x10));
        struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, &local_15e), 0x0, 0x0, 0x60, &local_10, param_1, uStack42, uStack30);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, &local_15e));
    }
    return;
}


astruct_18 *__stdcall16far pass1_1028_1b2e(astruct_18 *param_1, byte param_2, uint param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_1ec8(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_254c(astruct_18 *param_1, byte param_2, ushort param_3)

{
    pass1_1028_2042(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_2626(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_2762(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_2a6c(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_2b4e(astruct_18 *param_1, byte param_2, uint param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_0ab4(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1028_0b96(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1028_16fe(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1020_e868(astruct_18 *param_1, byte param_2)

{
    pass1_1020_e846(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1020_ee3a(ulong param_1, ushort param_2, int param_3, ushort param_4, uchar param_5)

{
    undefined2 extraout_DX;
    undefined  local_13c[0x124];
    ulong      uStack24;
    undefined4 uStack20;
    ulong      uStack16;
    ulong      local_c;
    undefined2 uStack8;
    int        iStack6;
    undefined2 uStack4;

    pass1_1028_b58e(param_1);
    local_c  = *(ulong *)(param_3 + 0xc);
    uStack8  = *(undefined2 *)(param_3 + 0x10);
    iStack6  = param_3;
    uStack4  = extraout_DX;
    uStack16 = pass1_1028_bb24(param_1);
    uStack20 = *(undefined4 *)(iStack6 + 0x2e);
    uStack24 = *(ulong *)((int)uStack20 + 0x4);
    struct_op_1028_87f0(param_4, param_5, (astruct_97 *)CONCAT22(param_4, local_13c), 0x0, 0x1, param_2, &local_c, param_4, uStack24, uStack16);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_4, local_13c));
    return;
}


astruct_18 *__stdcall16far pass1_1020_eed0(astruct_18 *param_1, byte param_2, uint param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1020_ef94(astruct_18 *param_1, byte param_2)

{
    pass1_1020_ef5e(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1028_0582(ulong *param_1, ulong *param_2, ushort param_3, ushort param_4, uchar param_5, ushort param_6)

{
    ulong     **ppuVar1;
    long       *plVar2;
    undefined4  uVar3;
    code      **ppcVar4;
    undefined  *puVar5;
    uint        uVar6;
    ushort      uVar7;
    ulong       uVar8;
    ushort      extraout_DX;
    uint        uVar9;
    ushort      extraout_DX_00;
    undefined2  extraout_DX_01;
    int         iVar10;
    int         iVar11;
    uint        uVar12;
    undefined2  uVar13;
    undefined2  uVar14;
    undefined   local_138[0x10e];
    ulong       local_2a;
    astruct_99 *paStack38;
    astruct_99 *paStack34;
    ulong       uStack30;
    ulong       uStack18;
    ulong       uStack14;
    undefined   local_a[0x4];
    ulong       uStack6;

    uVar12  = (uint)((ulong)param_1 >> 0x10);
    iVar10  = (int)param_1;
    uVar8   = *(ulong *)(iVar10 + 0x14);
    uVar13  = (undefined2)(uVar8 >> 0x10);
    iVar11  = (int)uVar8;
    uStack6 = uVar8 & 0xffff0000 | (ulong)(iVar11 + 0xa4);
    if((*(int *)(iVar11 + 0xa6) != 0x0) && (*(int *)(iVar11 + 0xac) != 0x0))
    {
        pass1_1028_081e((ulong)param_1, (int)param_2, param_6);
        param_2 = *(ulong **)(iVar10 + 0x20);
        ppuVar1 = (ulong **)((int)uStack6 + 0x8);
        if(*ppuVar1 < param_2 || *ppuVar1 == param_2)
        {
            puVar5  = local_a;
            ppcVar4 = (code **)((int)*param_1 + 0x40);
            (**ppcVar4)(param_3, param_1);
            uVar8   = ZEXT24(puVar5);
            param_6 = extraout_DX;
            if(puVar5 == (undefined *)0x0)
            {
                if(*(int *)((int)uStack6 + 0x2) == 0xc)
                {
                    uStack14 = pass1_1028_b4f2((ulong)param_1);
                    param_6  = (ushort)(uStack14 >> 0x10);
                    uVar8    = *(ulong *)((int)uStack14 + 0x1f6);
                    plVar2   = (long *)((int)uVar8 + 0x170);
                    *plVar2  = *plVar2 + 0x1;
                    uStack18 = uVar8;
                }
                else
                {
                    uStack18  = _PTR_LOOP_1050_68a2;
                    paStack38 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                    uVar9     = (uint)((ulong)paStack38 >> 0x10);
                    uVar6     = (uint)paStack38;
                    if((uVar9 | uVar6) == 0x0)
                    {
                        paStack34 = (astruct_99 *)0x0;
                    }
                    else
                    {
                        paStack38->field_0x0         = 0x389a;
                        *(undefined2 *)(uVar6 + 0x2) = 0x1008;
                        *(undefined2 *)(uVar6 + 0x4) = 0x0;
                        *(undefined2 *)(uVar6 + 0x6) = 0x0;
                        *(undefined2 *)(uVar6 + 0x8) = 0x0;
                        *(undefined2 *)(uVar6 + 0xa) = 0x0;
                        *(undefined2 *)(uVar6 + 0xc) = 0x0;
                        paStack38->field_0x0         = 0x56ce;
                        *(undefined2 *)(uVar6 + 0x2) = 0x1018;
                        paStack34                    = paStack38;
                    }
                    uVar13                                = (undefined2)(uStack6 >> 0x10);
                    iVar11                                = (int)uStack6;
                    uVar14                                = (undefined2)((ulong)paStack34 >> 0x10);
                    *(undefined2 *)((int)paStack34 + 0x6) = *(undefined2 *)(iVar11 + 0x2);
                    *(undefined2 *)((int)paStack34 + 0xa) = *(undefined2 *)(iVar11 + 0x6);
                    param_3                               = 0x1020;
                    uVar7                                 = switch_1020_c3b4(*(ushort *)(iVar11 + 0x2));
                    uVar6                                 = uVar7 * *(int *)((int)uStack6 + 0x6);
                    uVar8                                 = (ulong)uVar6;
                    *(uint *)((int)paStack34 + 0xc)       = uVar6;
                    uVar3                                 = *(undefined4 *)(iVar10 + 0x22);
                    ppcVar4                               = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar10 + 0x22) + 0x4);
                    (**ppcVar4)(0x1020, (int)uVar3, (int)((ulong)uVar3 >> 0x10));
                    param_6 = extraout_DX_00;
                }
            }
            param_2                        = (ulong *)uVar8;
            *(undefined2 *)(iVar10 + 0x20) = 0x0;
        }
    }
    uVar13 = (undefined2)(uStack6 >> 0x10);
    if((*(int *)((int)uStack6 + 0x4) != 0x0) && (*(int *)((int)uStack6 + 0x8) != 0x0))
    {
        pass1_1028_081e((ulong)param_1, (int)param_2, param_6);
        param_2 = *(ulong **)(iVar10 + 0x20);
        ppuVar1 = (ulong **)((int)uStack6 + 0x8);
        if(*ppuVar1 < param_2 || *ppuVar1 == param_2)
        {
            param_2 = &local_2a;
            ppcVar4 = (code **)((int)*param_1 + 0x40);
            (**ppcVar4)(param_3, param_1);
            if(param_2 == (ulong *)0x0)
            {
                uStack18  = _PTR_LOOP_1050_68a2;
                paStack38 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                uVar9     = (uint)((ulong)paStack38 >> 0x10);
                uVar6     = (uint)paStack38;
                if((uVar9 | uVar6) == 0x0)
                {
                    paStack34 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack38->field_0x0         = 0x389a;
                    *(undefined2 *)(uVar6 + 0x2) = 0x1008;
                    *(undefined2 *)(uVar6 + 0x4) = 0x0;
                    *(undefined2 *)(uVar6 + 0x6) = 0x0;
                    *(undefined2 *)(uVar6 + 0x8) = 0x0;
                    *(undefined2 *)(uVar6 + 0xa) = 0x0;
                    *(undefined2 *)(uVar6 + 0xc) = 0x0;
                    paStack38->field_0x0         = 0x56ce;
                    *(undefined2 *)(uVar6 + 0x2) = 0x1018;
                    paStack34                    = paStack38;
                }
                uVar13                                = (undefined2)(uStack6 >> 0x10);
                iVar11                                = (int)uStack6;
                uVar14                                = (undefined2)((ulong)paStack34 >> 0x10);
                *(undefined2 *)((int)paStack34 + 0x8) = *(undefined2 *)(iVar11 + 0x4);
                *(undefined2 *)((int)paStack34 + 0xa) = *(undefined2 *)(iVar11 + 0x6);
                uVar7                                 = pass1_1020_c42e(*(int *)(iVar11 + 0x4));
                param_2                               = (ulong *)(uVar7 * *(int *)((int)uStack6 + 0x6));
                *(ulong **)((int)paStack34 + 0xc)     = param_2;
                uVar3                                 = *(undefined4 *)(iVar10 + 0x22);
                ppcVar4                               = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar10 + 0x22) + 0x4);
                (**ppcVar4)(0x1020, (int)uVar3, (int)((ulong)uVar3 >> 0x10));
            }
            *(undefined2 *)(iVar10 + 0x20) = 0x0;
        }
    }
    if(*(int *)(iVar10 + 0xc) != 0xe)
    {
        pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar12 << 0x10);
        local_2a  = CONCAT22(extraout_DX_01, param_2);
        paStack34 = (astruct_99 *)*(undefined4 *)((int)param_2 + 0x2e);
        uStack30  = *(ulong *)((int)paStack34 + 0x4);
        pass1_1028_68de((astruct_100 *)CONCAT22(param_4, local_138), 0x1, uStack30, param_5, param_4);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_4, local_138));
    }
    return;
}


astruct_18 *__stdcall16far pass1_1028_08c6(astruct_18 *param_1, byte param_2)

{
    pass1_1028_0138(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_d7d8(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

undefined4 __stdcall16far pass1_1020_d8ca(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}

void __stdcall16far pass1_1020_e294(ulong param_1, ushort param_2, uchar param_3)

{
    undefined4 uVar1;
    ulong     *puVar2;
    ulong      uVar3;
    undefined2 extraout_DX;
    undefined2 uVar4;
    ushort     uVar5;
    ushort     uVar6;
    char       cStack347;
    undefined  local_150[0xc];
    ulong     *puStack324;
    undefined  local_140[0x124];
    ulong      uStack28;
    undefined4 uStack24;
    ulong      uStack20;
    ulong      local_10;
    undefined2 uStack12;
    int        iStack10;
    undefined2 uStack8;
    ulong      uStack6;

    uVar6 = (ushort)(param_1 >> 0x10);
    uVar5 = (ushort)param_1;
    if((0x1 < *(int *)(uVar5 + 0x24)) && (*(int *)(uVar5 + 0x24) < 0x6))
    {
        uVar1   = *(undefined4 *)(uVar5 + 0x28);
        uVar3   = *(ulong *)((int)uVar1 + 0x20);
        uStack6 = uVar3;
        pass1_1028_b58e(param_1);
        iStack10   = (int)uVar3;
        local_10   = *(ulong *)(iStack10 + 0xc);
        uStack12   = *(undefined2 *)(iStack10 + 0x10);
        puStack324 = &local_10;
        uVar4      = extraout_DX;
        uStack8    = extraout_DX;
        pass1_1028_c8ee(param_2, uVar5, uVar6, *(int *)(uVar5 + 0x24), (ushort *)CONCAT22(param_2, puStack324));
        puVar2 = &local_10;
        pass1_1028_c89c(param_1, (ushort *)CONCAT22(param_2, puVar2), (ulong *)CONCAT22(param_2, local_150), (int)puVar2, param_2);
        uStack20  = *puVar2;
        cStack347 = (char)(uStack20 >> 0x18);
        if((cStack347 == '\0') && ((int)uStack20 == 0x9))
        {
            *(undefined2 *)(uVar5 + 0x14) = 0x1;
        }
        uStack24 = *(undefined4 *)(iStack10 + 0x2e);
        uStack28 = *(ulong *)((int)uStack24 + 0x4);
        struct_op_1028_87f0(param_2, param_3, (astruct_97 *)CONCAT22(param_2, local_140), 0x0, *(int *)(uVar5 + 0x14) + 0x1, 0x79, &local_10, param_2, uStack28, uStack6);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_2, local_140));
    }
    *(undefined2 *)(uVar5 + 0x26) = 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_e39c(ulong param_1, ushort param_2, int param_3, ushort param_4, uchar param_5)

{
    undefined4 uVar1;
    undefined2 uVar2;
    undefined2 extraout_DX;
    undefined  local_13c[0x124];
    ulong      uStack24;
    undefined4 uStack20;
    ulong      uStack16;
    ulong      local_c;
    int        iStack8;
    undefined4 uStack6;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_DX, param_3);
    local_c = *(ulong *)(param_3 + 0xc);
    iStack8 = *(int *)(param_3 + 0x10);
    if(iStack8 < 0x1)
    {
        uVar2 = 0x5;
    }
    else
    {
        uVar2 = 0x6;
    }
    *(undefined2 *)(param_3 + 0x14) = uVar2;
    uVar1                           = *(undefined4 *)((int)param_1 + 0x28);
    uStack16                        = *(ulong *)((int)uVar1 + 0x20);
    uStack20                        = *(undefined4 *)(param_3 + 0x2e);
    uStack24                        = *(ulong *)((int)uStack20 + 0x4);
    struct_op_1028_87f0(param_4, param_5, (astruct_97 *)CONCAT22(param_4, local_13c), 0x0, 0x1, param_2, &local_c, param_4, uStack24, uStack16);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_4, local_13c));
    return;
}

astruct_18 *__stdcall16far pass1_1020_e76c(astruct_18 *param_1, byte param_2, uint param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1020_c80e(astruct_18 *param_1, byte param_2)

{
    pass1_1020_c47a(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_cc56(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_cd58(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_cfde(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_d2ee(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_d518(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1020_b872(ushort param_1, uchar param_2, ulong param_3, ulong param_4)

{
    ushort      uVar1;
    ushort      uVar2;
    ushort      uVar3;
    ulong      *puVar4;
    undefined  *puVar5;
    undefined4 *puVar6;
    ushort     *puVar7;
    ushort      uVar8;
    undefined   local_136[0x124];
    ulong       local_12;
    int         local_c;
    int         local_a;
    undefined4  local_8;
    undefined2  uStack4;

    uVar8   = (ushort)(param_4 >> 0x10);
    puVar6  = (undefined4 *)pass1_1030_5b5c((int)param_4, uVar8);
    local_8 = *puVar6;
    uStack4 = *(undefined2 *)((int)puVar6 + 0x4);
    pass1_1008_3e94((ushort *)CONCAT22(param_1, &local_8), (ushort *)CONCAT22(param_1, &local_c), (ushort *)CONCAT22(param_1, &local_a));
    uVar1 = local_a - 0xa;
    pass1_1008_612e(0xa, uVar1, uVar1);
    uVar2 = local_c - 0xa;
    pass1_1008_612e(0xa, uVar2, uVar2);
    puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_1, &local_12), 0x0, uVar2, uVar1);
    uVar1  = (ushort)((ulong)puVar7 >> 0x10);
    while(true)
    {
        puVar4 = &local_12;
        pass1_1020_b482(param_1, param_3, (undefined4 *)CONCAT22(param_1, puVar4), param_4);
        if(puVar4 != (ulong *)0x0)
            break;
        uVar2 = local_a - 0xa;
        pass1_1008_612e(0xa, uVar2, uVar2);
        uVar3 = local_c - 0xa;
        pass1_1008_612e(0xa, uVar3, uVar3);
        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_12), 0x0, uVar3, uVar2);
    }
    struct_op_1028_8888(param_1, param_2, (astruct_100 *)CONCAT22(param_1, local_136), 0x0, 0xa, &local_12, param_1, 0x8000002, 0x0, *(ulong *)((int)param_4 + 0x4));
    puVar5 = local_136;
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, puVar5));
    pass1_1020_b97e(param_1, puVar5, uVar1, (ushort)param_3, (ushort)(param_3 >> 0x10), 0x1);
    return;
}
void __stdcall16far fn_ptr_1020_ba7e(ulong *param_1)

{
    fn_ptr_1000_17ce((astruct_18 *)*param_1, 0x1000);
    return;
}

void __stdcall16far pass1_1020_bcc4(long *param_1, ushort param_2, ushort param_3)

{
    uint *puVar1;
    int   iVar2;
    uint  uVar3;
    int   iVar4;
    uint  uVar5;
    long  lVar6;
    long  lStack12;

    uVar5 = (uint)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(int *)(iVar4 + 0x4) == 0x0)
    {
        PTR_LOOP_1050_5f2e = (undefined *)0x0;
        iVar2              = *(int *)(iVar4 + 0x6);
    }
    else
    {
        uVar3              = *(uint *)(iVar4 + 0x4);
        puVar1             = (uint *)(iVar4 + 0x8);
        iVar2              = uVar3 + *puVar1;
        PTR_LOOP_1050_5f2e = (undefined *)(uint)CARRY2(uVar3, *puVar1);
    }
    if(PTR_LOOP_1050_5f2e == (undefined *)0x0)
    {
        if(*param_1 == 0x0)
        {
            if(_PTR_LOOP_1050_5f2c == 0x0)
            {
                PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0, 0x1000);
            }
            else
            {
            }
            uVar3 = fn_ptr_op_1000_1708(iVar2 * 0x6, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
        }
        else
        {
            lVar6              = pass1_1000_0ed4(0x1000, param_3, 0x1, iVar2 * 0x6, 0x0, (ushort *)*param_1, (ushort)((ulong)*param_1 >> 0x10));
            PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar6 >> 0x10);
            uVar3              = (uint)lVar6;
        }
        lStack12 = CONCAT22(PTR_LOOP_1050_5f2e, uVar3);
        if(((uint)PTR_LOOP_1050_5f2e | uVar3) != 0x0)
        {
            *(int *)(iVar4 + 0x4) = iVar2;
            *param_1              = lStack12;
            pass1_1020_bc72((uint *)((ulong)param_1 & 0xffff | (ulong)uVar5 << 0x10), param_2, param_3);
        }
    }
    return;
}

void __stdcall16far pass1_1020_c47a(ushort *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0xc834;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x18), 0x1000);
    pass1_1030_1d28(param_1);
    return;
}

void __stdcall16far pass1_1020_c73a(ulong param_1, ushort param_2)

{
    uint      *puVar1;
    uint       uVar2;
    undefined4 uVar3;
    uint       uVar4;
    int        iVar5;
    undefined2 uVar6;
    long       lVar7;
    undefined4 uStack10;
    undefined4 uStack6;

    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    if(*(long *)(iVar5 + 0x10) == 0x0)
    {
        uVar4              = *(uint *)(iVar5 + 0xc);
        PTR_LOOP_1050_5f2e = (undefined *)*(undefined2 *)(iVar5 + 0xe);
    }
    else
    {
        uVar2              = *(uint *)(iVar5 + 0x10);
        puVar1             = (uint *)(iVar5 + 0x14);
        uVar4              = uVar2 + *puVar1;
        PTR_LOOP_1050_5f2e = (undefined *)(*(int *)(iVar5 + 0x12) + *(int *)(iVar5 + 0x16) + (uint)CARRY2(uVar2, *puVar1));
    }
    uStack6 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    if(*(long *)(iVar5 + 0x18) == 0x0)
    {
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e, 0x1000);
        }
        else
        {
        }
        uVar4 = fn_ptr_op_1000_1708(uVar4 * 0x6, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    }
    else
    {
        uVar3 = *(undefined4 *)(iVar5 + 0x18);
        lVar7 = pass1_1000_0ed4(
          0x1000, param_2, 0x1, uVar4 * 0x6, ((int)PTR_LOOP_1050_5f2e * 0x3 + (uint)CARRY2(uVar4, uVar4) + (uint)CARRY2(uVar4 * 0x2, uVar4)) * 0x2 + (uint)CARRY2(uVar4 * 0x3, uVar4 * 0x3), (ushort *)uVar3, (ushort)((ulong)uVar3 >> 0x10));
        PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar7 >> 0x10);
        uVar4              = (uint)lVar7;
    }
    uStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    if(((uint)PTR_LOOP_1050_5f2e | uVar4) != 0x0)
    {
        *(undefined4 *)(iVar5 + 0x10) = uStack6;
        *(undefined4 *)(iVar5 + 0x18) = uStack10;
    }
    *(undefined2 *)(iVar5 + 0x1c) = 0x1;
    return;
}

astruct_18 *__stdcall16far pass1_1020_8784(astruct_18 *param_1, byte param_2)

{
    pass1_1020_8556(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_8a5e(astruct_18 *param_1, byte param_2)

{
    pass1_1020_8556(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_8e6c(astruct_18 *param_1, byte param_2)

{
    pass1_1020_8bae(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_91de(astruct_18 *param_1, byte param_2)

{
    pass1_1020_8f74(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_774c(astruct_18 *param_1, byte param_2)

{
    param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
    pass1_1020_75c4((ushort *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_78dc(astruct_18 *param_1, byte param_2, ushort param_3)

{
    pass1_1020_78ac(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_7f38(astruct_18 *param_1, byte param_2)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0x3ab0;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1020_8288(astruct_18 *param_1, byte param_2)

{
    param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
    pass1_1020_808e((ushort *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_843c(astruct_18 *param_1, byte param_2)

{
    pass1_1020_8556(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1020_8556(ushort *param_1)

{
    int         *piVar1;
    uint         uVar2;
    astruct_18  *paVar3;
    astruct_588 *iVar5;
    astruct_589 *iVar4;
    int          iVar6;
    undefined2   uVar7;
    undefined2   uVar8;
    int          iStack12;

    uVar7            = (undefined2)((ulong)param_1 >> 0x10);
    iVar5            = (astruct_588 *)param_1;
    *param_1         = 0x87aa;
    iVar5->field_0x2 = 0x1020;
    fn_ptr_1000_17ce(iVar5->field_0x8, 0x1000);
    if((*(uint *)((int)&iVar5->field_0xc + 0x2) | *(uint *)&iVar5->field_0xc) != 0x0)
    {
        iStack12 = 0x0;
        while(true)
        {
            piVar1 = &iVar5->field_0x6;
            if(*piVar1 == iStack12 || *piVar1 < iStack12)
                break;
            iVar6  = iStack12 * 0x4;
            paVar3 = iVar5->field_0xc;
            uVar8  = (undefined2)((ulong)paVar3 >> 0x10);
            iVar4  = (astruct_589 *)paVar3;
            if(*(long *)(iVar4 + iVar6) != 0x0)
            {
                paVar3 = *(astruct_18 **)(iVar4 + iVar6);
                uVar2  = *(uint *)(iVar4 + iVar6 + 0x2);
                if((uVar2 | (uint)paVar3) != 0x0)
                {
                    pass1_1008_5118((ulong)paVar3 & 0xffff | (ulong)uVar2 << 0x10);
                    fn_ptr_1000_17ce(paVar3, 0x1000);
                }
            }
            iStack12 = iStack12 + 0x1;
        }
        fn_ptr_1000_17ce(iVar5->field_0xc, 0x1000);
    }
    *param_1         = 0x389a;
    iVar5->field_0x2 = 0x1008;
    return;
}


void __stdcall16far pass1_1020_85f6(ulong param_1)

{
    int         *piVar1;
    uint         uVar2;
    astruct_18  *paVar3;
    undefined4   uVar4;
    int          iVar5;
    astruct_590 *iVar6;
    undefined2   uVar6;
    undefined2   uVar7;
    int          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar7  = (undefined2)(param_1 >> 0x10);
        iVar6  = (astruct_590 *)param_1;
        piVar1 = &iVar6->field_0x6;
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        uVar4  = iVar6->field_0xc;
        uVar6  = (undefined2)((ulong)uVar4 >> 0x10);
        iVar5  = (int)uVar4;
        paVar3 = *(astruct_18 **)(iVar5 + iStack4 * 0x4);
        uVar2  = *(uint *)(iVar5 + iStack4 * 0x4 + 0x2);
        if((uVar2 | (uint)paVar3) != 0x0)
        {
            pass1_1008_5118((ulong)paVar3 & 0xffff | (ulong)uVar2 << 0x10);
            fn_ptr_1000_17ce(paVar3, 0x1000);
        }
        uVar4                                       = iVar6->field_0xc;
        *(undefined4 *)((int)uVar4 + iStack4 * 0x4) = 0x0;
        iStack4                                     = iStack4 + 0x1;
    }
    return;
}


void __stdcall16far pass1_1020_865a(ulong param_1)

{
    int         *piVar1;
    uint         uVar2;
    astruct_18  *paVar3;
    undefined4   uVar4;
    int          iVar5;
    astruct_592 *iVar7;
    astruct_591 *iVar6;
    int          iVar8;
    undefined2   uVar9;
    undefined2   uVar10;
    int          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar9  = (undefined2)(param_1 >> 0x10);
        iVar5  = (int)param_1;
        piVar1 = (int *)(iVar5 + 0x6);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        iVar8  = iStack4 * 0x4;
        uVar4  = *(undefined4 *)(iVar5 + 0xc);
        uVar10 = (undefined2)((ulong)uVar4 >> 0x10);
        iVar7  = (astruct_592 *)uVar4;
        if(*(long *)(iVar7 + iVar8) != 0x0)
        {
            pass1_1008_5236(*(ulong *)(iVar7 + iVar8));
            uVar4  = *(undefined4 *)(iVar5 + 0xc);
            uVar10 = (undefined2)((ulong)uVar4 >> 0x10);
            iVar6  = (astruct_591 *)uVar4;
            paVar3 = *(astruct_18 **)(iVar6 + iVar8);
            uVar2  = *(uint *)(iVar6 + iVar8 + 0x2);
            if((uVar2 | (uint)paVar3) != 0x0)
            {
                pass1_1008_5118((ulong)paVar3 & 0xffff | (ulong)uVar2 << 0x10);
                fn_ptr_1000_17ce(paVar3, 0x1000);
            }
            uVar4                                       = *(undefined4 *)(iVar5 + 0xc);
            *(undefined4 *)((int)uVar4 + iStack4 * 0x4) = 0x0;
        }
        iStack4 = iStack4 + 0x1;
    }
    return;
}

astruct_18 *__stdcall16far pass1_1020_6208(astruct_18 *param_1, byte param_2, ushort param_3)

{
    param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
    destroy_cursor_1020_42f4((ushort *)param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}
astruct_18 *__stdcall16far pass1_1020_679c(astruct_18 *param_1, byte param_2, ushort param_3, ushort param_4)

{
    pass1_1020_6466(&param_1->field_0x0, param_3, param_4);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_3 *__stdcall16far pass1_1020_2e24(astruct_3 *param_1, byte param_2)

{
    ushort unaff_CS;

    pass1_1020_28fc(param_1, unaff_CS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_24f2(astruct_18 *param_1, byte param_2, ushort param_3)

{
    pass1_1020_1f74(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_26d8(astruct_18 *param_1, byte param_2)

{
    param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
    pass1_1020_2594((ushort *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_2868(astruct_18 *param_1, byte param_2, ushort param_3)

{
    pass1_1020_2838(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1020_0abc(ulong param_1)

{
    code     **ppcVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0xe6) != 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xe8) + 0x10);
        (**ppcVar1)();
    }
    return;
}

astruct_18 *__stdcall16far pass1_1020_0d82(astruct_18 *param_1, byte param_2)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0x3ab0;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1018_e5aa(astruct_18 *param_1, byte param_2)

{
    pass1_1018_e57a(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1018_e75c(astruct_18 *param_1, byte param_2)

{
    param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
    pass1_1018_e64c((ushort *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1018_e8ec(astruct_18 *param_1, byte param_2)

{
    pass1_1018_e8bc(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1018_eb9c(astruct_18 *param_1, byte param_2)

{
    param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
    pass1_1018_e9de((ushort *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_01a6(astruct_18 *param_1, byte param_2, ushort param_3)

{
    pass1_1018_ed98(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_0434(astruct_18 *param_1, byte param_2)

{
    pass1_1020_022c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_0734(astruct_18 *param_1, byte param_2, ushort param_3)

{
    pass1_1020_05d6(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_07f4(astruct_18 *param_1, byte param_2)

{
    pass1_1020_022c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1018_df10(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1018_e01c(astruct_18 *param_1, byte param_2)

{
    astruct_572 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_572 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}

astruct_18 *__stdcall16far pass1_1018_e1ee(astruct_18 *param_1, byte param_2)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0x3ab0;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1018_e41a(astruct_18 *param_1, byte param_2)

{
    param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
    pass1_1018_e2a0((ushort *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1018_8c46(astruct_18 *param_1, byte param_2)

{
    astruct_548 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_548 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8c8e(astruct_18 *param_1, byte param_2)

{
    astruct_549 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_549 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8cd6(astruct_18 *param_1, byte param_2)

{
    astruct_675 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_675 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8d1e(astruct_18 *param_1, byte param_2)

{
    astruct_550 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_550 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8d66(astruct_18 *param_1, byte param_2)

{
    astruct_551 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_551 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8dae(astruct_18 *param_1, byte param_2)

{
    astruct_552 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_552 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8df6(astruct_18 *param_1, byte param_2)

{
    astruct_553 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_553 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8e3e(astruct_18 *param_1, byte param_2)

{
    astruct_554 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_554 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8e86(astruct_18 *param_1, byte param_2)

{
    astruct_555 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_555 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8ece(astruct_18 *param_1, byte param_2)

{
    astruct_676 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_676 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8f16(astruct_18 *param_1, byte param_2)

{
    astruct_556 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_556 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8f5e(astruct_18 *param_1, byte param_2)

{
    astruct_677 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_677 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8fa6(astruct_18 *param_1, byte param_2)

{
    astruct_557 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_557 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_8fee(astruct_18 *param_1, byte param_2)

{
    astruct_558 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_558 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_9036(astruct_18 *param_1, byte param_2)

{
    astruct_559 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_559 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_907e(astruct_18 *param_1, byte param_2)

{
    astruct_560 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_560 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_90c6(astruct_18 *param_1, byte param_2)

{
    astruct_561 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_561 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_910e(astruct_18 *param_1, byte param_2)

{
    astruct_562 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_562 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_9156(astruct_18 *param_1, byte param_2)

{
    astruct_563 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_563 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_919e(astruct_18 *param_1, byte param_2)

{
    astruct_564 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_564 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_91e6(astruct_18 *param_1, byte param_2)

{
    astruct_565 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_565 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_922e(astruct_18 *param_1, byte param_2)

{
    astruct_566 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_566 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_9276(astruct_18 *param_1, byte param_2)

{
    astruct_567 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_567 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_92be(astruct_18 *param_1, byte param_2)

{
    astruct_568 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_568 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_9306(astruct_18 *param_1, byte param_2)

{
    astruct_569 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_569 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_934e(astruct_18 *param_1, byte param_2)

{
    astruct_570 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_570 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void __stdcall16far pass1_1018_9396(astruct_18 *param_1, byte param_2)

{
    astruct_571 *iVar1;
    undefined2   uVar1;

    iVar1 = (astruct_571 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}
