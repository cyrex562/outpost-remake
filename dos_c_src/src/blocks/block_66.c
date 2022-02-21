


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


ulong __stdcall16far pass1_1028_58dc(int param_1, ushort param_2)

{
    pass1_1028_b418(*(undefined4 *)(param_1 + 0x6));
    if((*(byte *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x6), 0x1000);
    }
    return CONCAT22(*(undefined2 *)(param_1 + 0x8), *(undefined2 *)(param_1 + 0x6));
}


ushort *__stdcall16far struct_1028_5966(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = (int)s_mineToSmelter__no_mines_1050_59df + 0x1;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_5988(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (int)s_mineToSmelter__no_mines_1050_59df + 0x1;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
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


ushort *__stdcall16far struct_1028_5a48(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = (ushort)s_thisLo_1050_5bec;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_5a6a(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (ushort)s_thisLo_1050_5bec;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_5a98(uint param_1, int param_2, ushort param_3)

{
    long      *plVar1;
    int        iVar2;
    code     **ppcVar3;
    undefined4 uVar4;
    undefined2 uVar5;
    ushort     uVar6;
    uint       extraout_DX;
    uint       uVar7;
    uint       extraout_DX_00;
    ulong      uVar8;

    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_2 + 0xa) + 0x10);
    (**ppcVar3)();
    *(uint *)(param_2 + -0x4) = param_1;
    *(uint *)(param_2 + -0x2) = extraout_DX;
    if((extraout_DX | param_1) == 0x0)
    {
        return;
    }
    *(undefined2 *)(param_2 + -0x6) = 0x1;
    uVar8                           = pass1_1030_bcae(param_2 - 0x8, param_3);
    uVar7                           = (uint)(uVar8 >> 0x10);
    *(undefined4 *)(param_2 + -0xc) = 0x0;
    while(true)
    {
        uVar8 = *(ulong *)(param_2 + -0x4);
        if(uVar8 <= *(ulong *)(param_2 + -0xc))
        {
            return;
        }
        pass1_1030_1d58(*(ulong *)(param_2 + 0xa));
        uVar5                            = (undefined2)uVar8;
        *(undefined2 *)(param_2 + -0x10) = uVar5;
        *(uint *)(param_2 + -0xe)        = uVar7;
        uVar8                            = uVar8 & 0xffff | (ulong)uVar7 << 0x10;
        pass1_1028_b58e(*(ulong *)(param_2 + 0x6));
        uVar6 = param_2 - 0x8;
        uVar7 = extraout_DX_00;
        pass1_1030_bd74(uVar6, param_3, CONCAT22(extraout_DX_00, uVar5), uVar8, param_3);
        *(ushort *)(param_2 + -0x12) = uVar6;
        if((int)uVar6 < 0x5)
            break;
        plVar1  = (long *)(param_2 + -0xc);
        *plVar1 = *plVar1 + 0x1;
    }
    uVar8                            = struct_op_1030_73a8(*(ulong *)(param_2 + -0x10));
    *(undefined2 *)(param_2 + -0x16) = (int)uVar8;
    *(undefined2 *)(param_2 + -0x14) = (int)(uVar8 >> 0x10);
    uVar4                            = *(undefined4 *)(param_2 + -0x16);
    iVar2                            = *(int *)((int)uVar4 + 0x20);
    *(int *)(param_2 + -0x18)        = iVar2;
    if(iVar2 == 0x2)
    {
        *(undefined2 *)(param_2 + -0x6) = 0x2;
    }
    if(iVar2 != 0x1)
    {
        return;
    }
    *(undefined2 *)(param_2 + -0x6) = 0x3;
    return;
}


void __stdcall16far pass1_1028_5b42(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined2 uVar2;
    ulong      uVar3;
    int        iVar4;

    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) != 0x6)
    {
        return;
    }
    uVar3 = pass1_1028_b4f2((ulong)param_1);
    if(*(long *)((int)uVar3 + 0x200) != 0x8000002)
    {
        if(*(long *)((int)param_1 + 0x1c) == 0x8000002)
        {
            iVar4 = 0x6;
            goto code_r0x10285bbe;
        }
        ppcVar1 = (code **)((int)*param_1 + 0x64);
        iVar4   = (**ppcVar1)(param_4, param_1);
        if(iVar4 == 0x0)
        {
            return;
        }
        pass1_1028_c0f0((ulong)param_1, 0x2, iVar4, param_2, param_3, param_5);
        if(iVar4 == 0x0)
        {
            iVar4 = 0x6;
            goto code_r0x10285bbe;
        }
        pass1_1028_c952((ulong)param_1, param_2, param_3, param_5);
        pass1_1028_c00a((ulong)param_1, 0x2, iVar4, param_5);
    }
    iVar4 = 0x5;
code_r0x10285bbe:
    pass1_1028_bdac(param_1, iVar4, param_4);
    return;
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


ushort *__stdcall16far struct_1028_5c54(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = (int)s_static_1050_5d8b + 0x3;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_5c76(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (int)s_static_1050_5d8b + 0x3;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_5ca4(undefined2 param_1, int param_2, ushort param_3, uchar param_4)

{
    undefined4 uVar1;
    undefined4 uVar2;
    undefined2 extraout_DX;
    ulong      uVar3;

    pass1_1028_b58e(*(ulong *)(param_2 + 0x6));
    *(undefined2 *)(param_2 + -0x4) = param_1;
    *(undefined2 *)(param_2 + -0x2) = extraout_DX;
    uVar1                           = *(undefined4 *)(param_2 + -0x4);
    *(undefined4 *)(param_2 + -0x8) = *(undefined4 *)((int)uVar1 + 0x2e);
    uVar3                           = pass1_1028_bb24(*(ulong *)(param_2 + 0x6));
    uVar2                           = *(undefined4 *)(param_2 + -0x8);
    uVar1                           = *(undefined4 *)(param_2 + -0x4);
    struct_op_1028_87f0(param_3,
                        param_4,
                        (astruct_97 *)CONCAT22(param_3, param_2 + -0x12c),
                        0x0,
                        0x0,
                        0x65,
                        (ulong *)((int)uVar1 + 0xc),
                        (ushort)((ulong)uVar1 >> 0x10),
                        *(ulong *)((int)uVar2 + 0x4),
                        uVar3);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_3, param_2 + -0x12c));
    *(undefined2 *)(param_2 + -0x12c) = 0x389a;
    *(undefined2 *)(param_2 + -0x12a) = 0x1008;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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


ushort *__stdcall16far pass1_1028_5df6(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = (int)s_thisHi_1050_5e6f + 0x1;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_5e18(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (int)s_thisHi_1050_5e6f + 0x1;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
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


ushort *__stdcall16far struct_1028_5ed8(ushort *param_1)

{
    undefined2 uVar1;

    struct_1028_b354(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x20) = 0x0;
    *param_1                             = 0x6054;
    *(undefined2 *)((int)param_1 + 0x2)  = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_5f00(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)(param_1 + 0x20)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x6054;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far
pass1_1028_5f34(ushort param_1, int param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6)

{
    int       *piVar1;
    undefined4 uVar2;
    BOOL16     BVar3;
    ulong      uVar4;
    uint       extraout_DX;
    uint       uVar5;
    undefined2 uVar6;
    int        iVar7;

    pass1_1028_be9e(*(ulong **)(param_2 + 0x6), param_3, param_4, param_5, param_6);
    uVar4 = *(ulong *)(param_2 + 0x6);
    uVar6 = (undefined2)(uVar4 >> 0x10);
    if(*(int *)((int)uVar4 + 0x12) == 0x5)
    {
        *(undefined2 *)((int)uVar4 + 0x20) = 0x64;
        pass1_1028_b58e(uVar4);
        *(ushort *)(param_2 + -0x4) = param_1;
        *(uint *)(param_2 + -0x2)   = extraout_DX;
        uVar2                       = *(undefined4 *)(param_2 + -0x4);
        uVar4                       = *(ulong *)((int)uVar2 + 0x2e);
        iVar7                       = 0x61;
        uVar5                       = extraout_DX;
        pass1_1038_3fb0(uVar4);
        BVar3 = pass1_1030_25b2(uVar4 & 0xffff | (ulong)uVar5 << 0x10, iVar7);
        if(BVar3 != 0x0)
        {
            uVar2   = *(undefined4 *)(param_2 + 0x6);
            piVar1  = (int *)((int)uVar2 + 0x20);
            *piVar1 = *piVar1 + 0x64;
        }
    }
    return;
}


BOOL16 __stdcall16far write_to_file_1028_5f82(ulong param_1, ulong param_2, ushort param_3)

{
    BOOL16     BVar1;
    undefined2 local_c[0x5];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        local_c[0] = *(undefined2 *)((int)param_1 + 0x20);
        BVar1      = write_to_file_1008_7e1c(
          (ushort)param_2, (ushort)(param_2 >> 0x10), (ushort)local_c, param_3, (char *)0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}


void __stdcall16far pass1_1028_5fcc(int param_1, uchar *param_2, int param_3, ushort param_4)

{
    undefined4 uVar1;
    undefined4 uVar2;
    BOOL16     BVar3;

    file_1028_b81a(*(ulong *)(param_3 + 0x6), *(ulong *)(param_3 + 0xa), param_1, param_4, param_2);
    if((param_1 != 0x0)
       && (uVar1 = *(undefined4 *)(param_3 + 0x6),
           uVar2 = *(undefined4 *)(param_3 + 0xa),
           BVar3 = read_file_1008_7dee((ushort)uVar2,
                                       (ushort)((ulong)uVar2 >> 0x10),
                                       (int)uVar1 + 0x20,
                                       0x0,
                                       (ushort)((ulong)uVar1 >> 0x10),
                                       0x2,
                                       0x1008),
           BVar3 == 0x0))
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    return;
}


void __stdcall16far pass1_1028_6008(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    int       *piVar1;
    int        iVar2;
    undefined2 uVar3;

    pass1_1028_be2a(param_1, param_2, param_3, param_4, param_5);
    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    if((*(int *)(iVar2 + 0x12) == 0x5) && (0x0 < *(int *)(iVar2 + 0x20)))
    {
        piVar1  = (int *)(iVar2 + 0x20);
        *piVar1 = *piVar1 + -0x1;
    }
    return;
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


ushort *__stdcall16far struct_1028_60bc(ushort *param_1, uint param_2, uchar *param_3)

{
    undefined4   uVar1;
    undefined2   uVar2;
    astruct_187 *iVar2;

    iVar2 = (astruct_187 *)param_1;
    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    struct_1028_b354(param_1);
    *(undefined4 *)&iVar2->field_0x20 = 0x0;
    *param_1                          = 0x6876;
    iVar2->field_0x2                  = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_3, 0x1000);
    if(((uint)param_3 | param_2) == 0x0)
    {
        *(undefined4 *)&iVar2->field_0x20 = 0x0;
    }
    else
    {
        uVar1             = set_struct_1008_574a((astruct_21 *)CONCAT22(param_3, param_2));
        iVar2->field_0x20 = (int)uVar1;
        iVar2->field_0x22 = (int)((ulong)uVar1 >> 0x10);
    }
    return param_1;
}


ulong __stdcall16far
pass1_1028_611e(int param_1, ushort param_2, int param_3, ulong param_4, uint param_5, uchar *param_6)

{
    undefined4 uVar1;

    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, (ushort)param_6);
    *(undefined4 *)(param_1 + 0x20)           = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x6876;
    *(undefined2 *)(param_1 + 0x2)            = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_6, 0x1000);
    if(((uint)param_6 | param_5) == 0x0)
    {
        *(undefined4 *)(param_1 + 0x20) = 0x0;
    }
    else
    {
        uVar1                           = set_struct_1008_574a((astruct_21 *)CONCAT22(param_6, param_5));
        *(undefined2 *)(param_1 + 0x20) = (int)uVar1;
        *(undefined2 *)(param_1 + 0x22) = (int)((ulong)uVar1 >> 0x10);
    }
    return CONCAT22(param_2, param_1);
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


ulong __stdcall16far pass1_1028_62c8(ulong param_1, ushort param_2)

{
    uint  uVar1;
    ulong uVar2;

    uVar1 = (uint)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        uVar2 = pass1_1028_67d4(param_1 & 0xffff | (ulong)uVar1 << 0x10, param_2);
        uVar1 = (uint)uVar2;
        if(((int)(uVar2 >> 0x10) == 0x0) && (uVar1 < 0x64))
        {
            return CONCAT22(-(uint)(0x64 < uVar1), 0x64 - uVar1);
        }
    }
    return 0x0;
}


// WARNING: Could not reconcile some variable overlaps

ulong __stdcall16far pass1_1028_6302(ulong param_1, ushort param_2)

{
    uint       uVar1;
    undefined2 uVar2;
    long       lVar3;
    ulong      uStack18;
    undefined  local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_2, local_a), *(ulong *)((int)param_1 + 0x20));
    uStack18 = 0x0;
    while(true)
    {
        lVar3 = pass1_1008_5b12(local_a, param_2);
        uVar2 = (undefined2)((ulong)lVar3 >> 0x10);
        if(lVar3 == 0x0)
            break;
        if(*(int *)((int)lVar3 + 0x8) != 0x0)
        {
            uVar1    = *(uint *)((int)lVar3 + 0xa);
            uStack18 = CONCAT22((int)(uStack18 >> 0x10) + (uint)CARRY2((uint)uStack18, uVar1), (uint)uStack18 + uVar1);
        }
    }
    return uStack18;
}
