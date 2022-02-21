
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


ushort *__stdcall16far pass1_1028_5e18(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (int)s_thisHi_1050_5e6f + 0x1;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_5f00(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)(param_1 + 0x20)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x6054;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_5f34(ushort param_1, int param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6)

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


ushort *__stdcall16far pass1_1028_408e(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x42ec;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_40b8(ulong param_1, ushort param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    uint        uVar3;
    ushort      uVar4;
    undefined2  extraout_DX;
    uint        uVar5;
    ulong       uVar6;
    ulong      *puVar7;
    undefined4  uStack54;
    undefined   local_2c[0x6];
    undefined4 *puStack38;
    undefined4  uStack34;
    undefined4 *puStack26;
    undefined4  uStack24;
    undefined4  local_14;
    int         iStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    ushort      uStack6;
    undefined2  uStack4;

    pass1_1028_b58e(param_1);
    local_14  = *(undefined4 *)(param_2 + 0xc);
    iStack8   = *(int *)(param_2 + 0x10);
    puStack26 = &local_c;
    uStack34  = CONCAT22(uStack34._2_2_, &local_14);
    iStack16  = iStack8 + 0x1;
    puVar7    = (ulong *)CONCAT22(param_3, local_2c);
    iStack14  = iStack16;
    local_c   = local_14;
    uStack6   = param_2;
    uStack4   = extraout_DX;
    uVar6     = pass1_1028_bb24(param_1);
    uVar5     = (uint)(uVar6 >> 0x10);
    puVar2    = &local_14;
    pass1_1030_64ce(param_3, puVar2, uVar5, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_3, puVar2), uVar6 & 0xffff | (ulong)uVar5 << 0x10, puVar7);
    uStack34       = *puVar2;
    uVar5          = *(uint *)((int)puVar2 + 0x2);
    uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
    uVar3          = (uint)uStack54._3_1_;
    uStack24       = uStack34;
    if(uStack54._3_1_ != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack34, uVar5);
        uStack54 = CONCAT22(uVar5, uVar3);
        uVar4    = pass1_1030_6fa0(CONCAT22(uVar5, uVar3));
        if(uVar4 == 0x64)
        {
            puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
            ppcVar1   = (code **)((int)*puStack38 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b514(param_1);
    return;
}


void __stdcall16far pass1_1028_41ea(ulong param_1, int param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    uint        uVar3;
    ushort      uVar4;
    undefined2  extraout_DX;
    uint        uVar5;
    ulong       uVar6;
    ulong      *puVar7;
    undefined4  uStack54;
    undefined   local_2c[0x6];
    undefined4 *puStack38;
    undefined4  uStack34;
    undefined4 *puStack26;
    undefined4  uStack24;
    undefined4  local_14;
    int         iStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    int         iStack6;
    undefined2  uStack4;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_14  = *(undefined4 *)(param_2 + 0xc);
    iStack8   = *(int *)(param_2 + 0x10);
    puStack26 = &local_c;
    uStack34  = CONCAT22(uStack34._2_2_, &local_14);
    iStack16  = iStack8 + 0x1;
    puVar7    = (ulong *)CONCAT22(param_3, local_2c);
    iStack14  = iStack16;
    local_c   = local_14;
    iStack6   = param_2;
    uStack4   = extraout_DX;
    uVar6     = pass1_1028_bb24(param_1);
    uVar5     = (uint)(uVar6 >> 0x10);
    puVar2    = &local_14;
    pass1_1030_64ce(param_3, puVar2, uVar5, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_3, puVar2), uVar6 & 0xffff | (ulong)uVar5 << 0x10, puVar7);
    uStack34       = *puVar2;
    uVar5          = *(uint *)((int)puVar2 + 0x2);
    uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
    uVar3          = (uint)uStack54._3_1_;
    if(uStack54._3_1_ != 0x0)
    {
        uStack24 = uStack34;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack34, uVar5);
        uStack54 = CONCAT22(uVar5, uVar3);
        uVar4    = pass1_1030_6fa0(CONCAT22(uVar5, uVar3));
        if(uVar4 == 0x64)
        {
            puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
            ppcVar1   = (code **)((int)*puStack38 + 0x24);
            (**ppcVar1)();
        }
    }
    return;
}


ushort *__stdcall16far pass1_1028_4376(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x446a;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1028_43a0(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    if((*(int *)((int)param_1 + 0x12) != 0x6) && (*(int *)((int)param_1 + 0x12) != 0x5))
    {
        return 0x0;
    }
    return 0x1;
}


ushort *__stdcall16far pass1_1028_44fe(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined4 *)(param_1 + 0x20)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x4836;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_4530(ushort *param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                        = (int)param_1;
    *param_1                     = 0x4836;
    *(undefined2 *)(iVar4 + 0x2) = (int)&USHORT_1050_1028;
    puVar1                       = (undefined4 *)*(uint *)(iVar4 + 0x20);
    uVar2                        = *(uint *)(iVar4 + 0x22);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1028_b418(param_1);
    return;
}


void __stdcall16far pass1_1028_456e(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    pass1_1028_b46e(param_1, param_2, param_3);
    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x20);
    uVar2  = *(uint *)(iVar4 + 0x22);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *(undefined4 *)(iVar4 + 0x20) = 0x0;
    return;
}


void __stdcall16far pass1_1028_45b0(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    uint       uVar1;
    ulong      uVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    int        iVar5;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    uVar1 = (uint)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        uVar4 = 0x0;
        iVar5 = 0x4;
        uVar3 = 0x2;
        uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        pass1_1030_7c50(uVar2, CONCAT22(uVar4, uVar3), iVar5, (uint)uVar2, (uchar *)(uVar2 >> 0x10));
    }
    return;
}


ulong __stdcall16far pass1_1028_45e2(ulong param_1, uint param_2, int param_3, ushort param_4)

{
    pass1_1028_478a(param_1, param_2, param_4);
    return CONCAT22(-(uint)(0x3e8 < param_2) - param_3, 0x3e8 - param_2);
}


ushort __stdcall16far pass1_1028_4768(ulong param_1, uint param_2, int param_3, ushort param_4)

{
    pass1_1028_478a(param_1, param_2, param_4);
    if((param_3 == 0x0) && (param_2 < 0x3e8))
    {
        return 0x0;
    }
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1028_478a(ulong param_1, int param_2, ushort param_3)

{
    undefined2 extraout_DX;
    long       local_1e;
    int        local_1a[0x4];
    uint       uStack18;
    uint       uStack16;
    long       lStack14;
    ulong     *puStack10;
    undefined4 uStack6;

    pass1_1028_b58e(param_1);
    uStack6   = CONCAT22(extraout_DX, param_2);
    puStack10 = *(ulong **)(param_2 + 0x22);
    lStack14  = 0x0;
    if((*(uint *)(param_2 + 0x24) | (uint)puStack10) == 0x0)
    {
        return;
    }
    uStack16 = *(uint *)((uint)puStack10 + 0x4);
    for(uStack18 = 0x0; uStack18 < uStack16; uStack18 = uStack18 + 0x1)
    {
        pass1_1020_bb16(puStack10, (ulong *)CONCAT22(param_3, &local_1e), (ushort *)CONCAT22(param_3, local_1a), uStack18);
        if(0x0 < local_1a[0])
        {
            lStack14 = lStack14 + local_1e;
        }
    }
    return;
}


ushort *__stdcall16far pass1_1028_48c0(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (ushort)&PTR_LOOP_1050_4942;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    *(undefined2 *)(param_1 + 0xe)        = *(undefined2 *)(param_1 + 0xc);
    *(undefined2 *)(param_1 + 0x10)       = *(undefined2 *)(param_1 + 0xc);
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_48fa(ulong *param_1, ushort param_2)

{
    pass1_1028_bdac(param_1, 0x0, param_2);
    return;
}


ulong __stdcall16far pass1_1028_49de(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x4b1c;
    *(undefined2 *)(param_1 + 0x2)            = (int)&USHORT_1050_1028;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, param_1 + 0x20), (WNDCLASS16 *)0x0, 0xa);
    return CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1028_4a9a(ulong param_1, int param_2)

{
    return *(ushort *)((int)param_1 + 0x20 + param_2 * 0x2);
}


void __stdcall16far pass1_1028_4ab2(ulong param_1, ushort param_2, int param_3)

{
    *(ushort *)((int)param_1 + param_3 * 0x2 + 0x20) = param_2;
    return;
}


ushort *__stdcall16far pass1_1028_4ba6(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1028_4bd0(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    if((*(int *)((int)param_1 + 0x12) != 0x6) && (*(int *)((int)param_1 + 0x12) != 0x5))
    {
        return 0x0;
    }
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_4bf2(ulong param_1, ulong param_2, int param_3, ushort param_4)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    ushort      uVar3;
    undefined2  extraout_DX;
    uint        uVar4;
    ulong       uVar5;
    ulong      *puVar6;
    undefined4  uStack54;
    undefined   local_2c[0x6];
    undefined4 *puStack38;
    undefined4  uStack34;
    undefined4 *puStack26;
    undefined4  uStack24;
    undefined4  local_14;
    int         iStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    int         iStack6;
    undefined2  uStack4;

    pass1_1028_b58e(param_1);
    local_14  = *(undefined4 *)(param_3 + 0xc);
    iStack8   = *(int *)(param_3 + 0x10);
    puStack26 = &local_c;
    uStack34  = CONCAT22(uStack34._2_2_, &local_14);
    iStack16  = iStack8 + 0x1;
    puVar6    = (ulong *)CONCAT22(param_4, local_2c);
    iStack14  = iStack16;
    local_c   = local_14;
    iStack6   = param_3;
    uStack4   = extraout_DX;
    uVar5     = pass1_1028_bb24(param_1);
    uVar4     = (uint)(uVar5 >> 0x10);
    puVar2    = &local_14;
    pass1_1030_64ce(param_4, puVar2, uVar4, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_4, puVar2), uVar5 & 0xffff | (ulong)uVar4 << 0x10, puVar6);
    uStack34       = *puVar2;
    uVar4          = *(uint *)((int)puVar2 + 0x2);
    uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
    uVar3          = (ushort)uStack54._3_1_;
    uStack24       = uStack34;
    if(uStack54._3_1_ != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack34, uVar4);
        uStack54 = CONCAT22(uVar4, uVar3);
        uVar3    = pass1_1030_6fa0(CONCAT22(uVar4, uVar3));
        if((uVar3 == 0x62) || (uVar3 == 0x63))
        {
            puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
            uVar3     = (ushort)puStack38;
            ppcVar1   = (code **)((int)*puStack38 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(param_1, param_2, uVar3);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_4cd6(ulong param_1, int param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    uint        uVar3;
    ushort      uVar4;
    undefined2  extraout_DX;
    uint        uVar5;
    ulong       uVar6;
    ulong      *puVar7;
    undefined4  uStack54;
    undefined   local_2c[0x6];
    undefined4 *puStack38;
    undefined4  uStack34;
    undefined4 *puStack26;
    undefined4  uStack24;
    undefined4  local_14;
    int         iStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    int         iStack6;
    undefined2  uStack4;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_14  = *(undefined4 *)(param_2 + 0xc);
    iStack8   = *(int *)(param_2 + 0x10);
    puStack26 = &local_c;
    uStack34  = CONCAT22(uStack34._2_2_, &local_14);
    iStack16  = iStack8 + 0x1;
    puVar7    = (ulong *)CONCAT22(param_3, local_2c);
    iStack14  = iStack16;
    local_c   = local_14;
    iStack6   = param_2;
    uStack4   = extraout_DX;
    uVar6     = pass1_1028_bb24(param_1);
    uVar5     = (uint)(uVar6 >> 0x10);
    puVar2    = &local_14;
    pass1_1030_64ce(param_3, puVar2, uVar5, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_3, puVar2), uVar6 & 0xffff | (ulong)uVar5 << 0x10, puVar7);
    uStack34       = *puVar2;
    uVar5          = *(uint *)((int)puVar2 + 0x2);
    uStack54._3_1_ = (byte)((ulong)uStack34 >> 0x18);
    uVar3          = (uint)uStack54._3_1_;
    if(uStack54._3_1_ != 0x0)
    {
        uStack24 = uStack34;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack34, uVar5);
        uStack54 = CONCAT22(uVar5, uVar3);
        uVar4    = pass1_1030_6fa0(CONCAT22(uVar5, uVar3));
        if((uVar4 == 0x62) || (uVar4 == 0x63))
        {
            puStack38 = (undefined4 *)struct_op_1030_73a8(uStack54);
            ppcVar1   = (code **)((int)*puStack38 + 0x24);
            (**ppcVar1)();
        }
    }
    return;
}


void __stdcall16far pass1_1028_4f30(ulong param_1, int param_2, ushort param_3, ushort param_4, ushort param_5)

{
    ulong uVar1;
    uint  uVar2;

    uVar1 = pass1_1028_b58e(param_1);
    if(param_2 == 0x0)
    {
        uVar2 = 0x0;
    }
    else
    {
        uVar2 = 0x3e8;
    }
    pass1_1030_7d1c(uVar1, uVar2, 0x230000, (int)uVar1, (int)(uVar1 >> 0x10), param_3, param_4, param_5);
    return;
}


ushort __stdcall16far pass1_1028_4f62(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = pass1_1028_b58e(param_1);
    if(*(int *)((int)uVar1 + 0x10) == 0x0)
    {
        return 0x1;
    }
    return 0x0;
}


ushort *__stdcall16far pass1_1028_2bfe(astruct_179 *param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_0982(param_1, param_2, param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x341c;
    param_1->field_0x2                    = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1028_2c28(ushort param_1, ushort param_2, ushort param_3, ulong param_4, ulong *param_5, ulong param_6, ulong param_7)

{
    ulong     *puVar1;
    ushort    *puVar2;
    ushort     uVar3;
    ushort     uVar4;
    ushort     local_e;
    ushort     local_c;
    ushort     local_a;
    ulong      local_8;
    undefined2 uStack4;

    pass1_1028_09d4(param_1, param_2, param_3, param_4, (ushort *)param_5, param_6, param_7);
    if(param_2 != 0x0)
    {
        local_8 = *param_5;
        uStack4 = *(undefined2 *)((int)param_5 + 0x4);
        puVar2  = &local_e;
        pass1_1008_3eb4((ushort *)CONCAT22(param_1, &local_8), (ushort *)CONCAT22(param_1, puVar2), (ushort *)CONCAT22(param_1, &local_c), (ushort *)CONCAT22(param_1, &local_a));
        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c - 0x1, local_a - 0x1);
        puVar1 = &local_8;
        uVar3  = (ushort)param_4;
        uVar4  = (ushort)(param_4 >> 0x10);
        pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
        if(puVar1 != (ulong *)0x0)
        {
            pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c - 0x1, local_a);
            puVar1 = &local_8;
            pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
            if(puVar1 != (ulong *)0x0)
            {
                pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c - 0x1, local_a + 0x1);
                puVar1 = &local_8;
                pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
                if(puVar1 != (ulong *)0x0)
                {
                    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c, local_a - 0x1);
                    puVar1 = &local_8;
                    pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
                    if(puVar1 != (ulong *)0x0)
                    {
                        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c, local_a);
                        puVar1 = &local_8;
                        pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
                        if(puVar1 != (ulong *)0x0)
                        {
                            pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c, local_a + 0x1);
                            puVar1 = &local_8;
                            pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
                            if(puVar1 != (ulong *)0x0)
                            {
                                pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c + 0x1, local_a - 0x1);
                                puVar1 = &local_8;
                                pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
                                if(puVar1 != (ulong *)0x0)
                                {
                                    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c + 0x1, local_a);
                                    puVar1 = &local_8;
                                    pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
                                    if(puVar1 != (ulong *)0x0)
                                    {
                                        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_8), local_e, local_c + 0x1, local_a + 0x1);
                                        puVar1 = &local_8;
                                        pass1_1028_c7b6(param_1, (ushort)puVar2, uVar3, uVar4, (ushort *)CONCAT22(param_1, puVar1), param_7);
                                        if(puVar1 != (ulong *)0x0)
                                        {
                                            return 0x1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        PTR_LOOP_1050_50ca = (undefined *)0x6a8;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_2e40(ulong *param_1, int param_2, uchar *param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7, uchar param_8)

{
    uint       uVar1;
    ushort    *puVar2;
    undefined4 uVar3;

    pass1_1028_be9e(param_1, param_4, param_5, param_6, param_7);
    uVar1 = (uint)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        pass1_1028_2f18(param_7, param_2, param_8, (ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        pass1_1028_3246((ulong)param_1, param_2, (ushort)param_3, param_4, param_5, param_7);
        uVar3  = 0x50001;
        puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_7, param_3, param_5);
        pass1_1010_089e(param_7, (ulong)puVar2, (ushort)uVar3, (int)((ulong)uVar3 >> 0x10));
    }
    return;
}


void __stdcall16far pass1_1028_3246(ulong param_1, int param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6)

{
    uint       uVar1;
    ulong      uVar2;
    uchar     *extraout_DX;
    uchar     *puVar3;
    undefined2 uVar4;
    undefined2 uVar5;
    int        iVar6;
    undefined  local_20[0x6];
    ushort    *puStack26;
    uint       uStack18;
    uchar     *puStack16;
    ulong      uStack14;
    undefined4 uStack10;
    undefined4 uStack6;

    pass1_1028_b58e(param_1);
    uStack6  = CONCAT22(extraout_DX, param_2);
    uStack10 = *(undefined4 *)(param_2 + 0x2e);
    uVar2    = *(ulong *)((int)uStack10 + 0x10);
    uVar5    = 0x0;
    iVar6    = 0x1;
    uVar4    = 0x1;
    puVar3   = extraout_DX;
    uStack14 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)(uVar2 >> 0x10));
    uVar1     = (uint)uVar2;
    uStack18  = uVar1;
    puStack16 = puVar3;
    pass1_1030_7c50(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10, CONCAT22(uVar5, uVar4), iVar6, uVar1, puVar3);
    pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x1, 0x2, uVar1, puVar3);
    pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x1, 0x3, uVar1, puVar3);
    pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x1, 0x5, uVar1, puVar3);
    puStack26 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_6, puVar3, param_5);
    puVar3    = (uchar *)((ulong)puStack26 >> 0x10);
    uVar1     = (uint)puStack26;
    if(*(int *)(uVar1 + 0x82) == 0x0)
    {
        pass1_1030_7c50(CONCAT22(puStack16, uStack18), 0x4, 0x4, uVar1, puVar3);
    }
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0xc8, 0x11, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0xc8, 0x12, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0xc8, 0x13, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0xc8, 0x14, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0x14, 0x15, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0x14, 0x16, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0xc8, 0x17, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0xc8, 0x18, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0xc8, 0x19, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0x14, 0x1a, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0x14, 0x1b, uVar1, puVar3, param_4, param_5, param_6);
    pass1_1030_7ddc(CONCAT22(puStack16, uStack18), 0x14, 0x1c, uVar1, puVar3, param_4, param_5, param_6);
    if(*(long *)((int)uStack10 + 0x200) == 0x8000002)
    {
        pass1_1020_a43e(param_6, puVar3, (ushort *)CONCAT22(param_6, local_20));
        pass1_1020_a89e(param_6, CONCAT22(param_6, local_20), (ulong *)((int)uStack6 + 0xc), (ushort)((ulong)uStack6 >> 0x10));
    }
    return;
}


ushort *__stdcall16far pass1_1028_34a6(int param_1, ushort param_2, int param_3, ulong param_4, uchar *param_5)

{
    pass1_1028_00cc(param_1, param_2, param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x34f6;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_3580(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x3608;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1028_35aa(void)

{
    return 0x1;
}


void __stdcall16far pass1_1028_35b0(ulong param_1, int param_2, ushort param_3, ushort param_4, ushort param_5)

{
    ulong uVar1;
    uint  uVar2;

    uVar1 = pass1_1028_b58e(param_1);
    if(param_2 == 0x0)
    {
        uVar2 = 0x0;
    }
    else
    {
        uVar2 = 0x32;
    }
    pass1_1030_7d1c(uVar1, uVar2, 0x230000, (int)uVar1, (int)(uVar1 >> 0x10), param_3, param_4, param_5);
    return;
}


ushort *__stdcall16far pass1_1028_3692(int param_1, ushort param_2, int param_3, ulong param_4, uchar *param_5, ushort param_6, ushort param_7)

{
    pass1_1028_3816(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
    *(ushort *)CONCAT22(param_2, param_1) = 0x373e;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1028_36bc(ulong param_1, ulong *param_2, ushort param_3, ushort param_4, ushort param_5)

{
    int       *piVar1;
    uint       uVar2;
    undefined4 uVar3;
    undefined2 uVar4;
    undefined4 uVar5;
    int        iStack4;

    uVar5    = CONCAT22(param_4, param_3);
    *param_2 = 0x0;
    uVar4    = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x28) != 0x0)
    {
        iStack4 = 0x4;
        while(true)
        {
            if(0x1c < iStack4)
                break;
            uVar3            = *(undefined4 *)((int)param_1 + 0x28);
            uVar5            = pass1_1020_bae6((ushort)uVar3, CONCAT22(iStack4, (int)((ulong)uVar3 >> 0x10)), (uint)uVar5, (uint)((ulong)uVar5 >> 0x10), param_5);
            uVar2            = *(uint *)param_2;
            *(uint *)param_2 = *(int *)param_2 + (uint)uVar5;
            piVar1           = (int *)((int)param_2 + 0x2);
            *piVar1          = *piVar1 + (int)((ulong)uVar5 >> 0x10) + (uint)CARRY2(uVar2, (uint)uVar5);
            if(0xf9 < *param_2)
            {
                return 0x1;
            }
            iStack4 = iStack4 + 0x1;
        }
    }
    return 0x0;
}


ushort __stdcall16far pass1_1028_38d4(ulong *param_1, ushort *param_2, ulong param_3, ulong param_4, int param_5, ushort param_6, ushort param_7)

{
    code     **ppcVar1;
    BOOL16     BVar2;
    undefined4 uVar3;
    ushort     uVar4;
    ushort     uVar5;

    uVar4 = (ushort)param_1;
    uVar5 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1028_c7b6(param_7, param_6, uVar4, uVar5, param_2, param_4);
    if((param_5 == 0x5) || (param_5 == 0x6))
    {
        ppcVar1 = (code **)((int)*param_1 + 0x60);
        uVar3   = (**ppcVar1)();
        if((uint)uVar3 != 0x0)
        {
            pass1_1028_c23e(uVar4, uVar5, param_2, param_3, param_4, (uint)uVar3, (uint)((ulong)uVar3 >> 0x10), param_7);
            if((int)uVar3 != 0x0)
            {
                BVar2 = pass1_1028_c314(param_7, (int)uVar3, (ushort)((ulong)uVar3 >> 0x10), uVar4, uVar5, param_2, (ushort)param_3, (ushort)(param_3 >> 0x10), param_4);
                if(BVar2 != 0x0)
                {
                    return 0x1;
                }
            }
        }
    }
    else
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6a8;
    }
    return 0x0;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_3958(ulong param_1, int param_2, undefined2 param_3, int param_4, ushort param_5, ushort param_6)

{
    long      *plVar1;
    qword      qVar2;
    uint       uVar3;
    uint       uVar4;
    uint       uVar5;
    ulong      uVar6;
    int        iVar7;
    undefined2 uVar8;
    undefined2 uVar9;
    undefined4 uStack52;
    uint       local_2c[0x2];
    undefined4 local_28;
    int        iStack36;
    ulong      uStack34;
    undefined4 uStack30;
    uint       uStack22;
    uint       uStack20;
    undefined4 uStack18;
    ulong      uStack14;
    ulong     *puStack10;
    int        iStack6;
    undefined2 uStack4;

    pass1_1028_b58e(param_1);
    puStack10 = *(ulong **)(param_2 + 0x22);
    uVar5     = *(uint *)(param_2 + 0x24);
    uVar6     = (ulong)uVar5;
    if((uVar5 | (uint)puStack10) != 0x0)
    {
        iStack6 = param_2;
        uStack4 = param_3;
        if(PTR_LOOP_1050_574c != (undefined *)0x0)
        {
            uStack30 = (ulong) * (uint *)((uint)puStack10 + 0x4);
            for(uStack34 = 0x0; uStack34 < uStack30; uStack34 = uStack34 + 0x1)
            {
                pass1_1020_bb16(puStack10, (ulong *)CONCAT22(param_6, local_2c), (ushort *)CONCAT22(param_6, &local_28), (uint)uStack34);
            }
        }
        uStack14 = *(ulong *)(iStack6 + 0x2e);
        uStack18 = *_PTR_LOOP_1050_65e2;
        uStack20 = (uint)uStack18 & 0x1;
        for(uStack22 = 0x4; (int)uStack22 < 0xe; uStack22 = uStack22 + 0x1)
        {
            local_2c[0] = uStack22;
            local_28    = pass1_1020_bae6((ushort)puStack10, CONCAT22(uStack22, (int)((ulong)puStack10 >> 0x10)), uStack22, (uint)uVar6, param_6);
            uVar5       = (uint)(local_28 >> 0x10) | (uint)local_28;
            uVar6       = (ulong)uVar5;
            if(uVar5 != 0x0)
            {
                pass1_1020_bb8a((long *)puStack10, 0x0, (ulong)local_2c[0] << 0x10, param_5, param_6);
                uVar5    = -(local_28._2_2_ + (uint)((int)local_28 != 0x0));
                uVar6    = (ulong)uVar5;
                uStack34 = CONCAT22(uVar5, -(int)local_28);
                pass1_1038_5694(uStack14, CONCAT22(uVar5, -(int)local_28), local_2c[0]);
                uStack30 = 0x0;
                iStack36 = 0x0;
                iVar7    = (int)param_1;
                uVar8    = (undefined2)(param_1 >> 0x10);
                switch(uStack22)
                {
                case 0x4:
                    uStack30 = local_28 >> 0x1;
                    if((uStack30 == 0x0) && (uStack20 != 0x0))
                    {
                        uStack30 = 0x1;
                    }
                    iStack36 = 0x11;
                    break;
                case 0x5:
                    uStack30 = local_28 >> 0x1;
                    if((uStack30 == 0x0) && (uStack20 != 0x0))
                    {
                        uStack30 = 0x1;
                    }
                    iStack36 = 0x12;
                    break;
                case 0x6:
                    uStack30 = local_28 >> 0x1;
                    if((uStack30 == 0x0) && (uStack20 != 0x0))
                    {
                        uStack30 = 0x1;
                    }
                    iStack36 = 0x13;
                    break;
                case 0x7:
                    uStack30 = local_28 >> 0x1;
                    if((uStack30 == 0x0) && (uStack20 != 0x0))
                    {
                        uStack30 = 0x1;
                    }
                    iStack36 = 0x14;
                    break;
                case 0x8:
                    uStack30 = local_28;
                    iStack36 = 0x1a;
                    break;
                case 0x9:
                    uStack30 = local_28;
                    iStack36 = 0x1b;
                    break;
                case 0xa:
                    uStack30 = local_28;
                    iStack36 = 0x1c;
                    break;
                case 0xb:
                    uStack30 = local_28;
                    iStack36 = 0x17;
                    break;
                case 0xc:
                    iStack36               = 0x18;
                    uStack30               = local_28;
                    plVar1                 = (long *)(iVar7 + 0x20);
                    *plVar1                = *plVar1 + local_28;
                    uVar5                  = *(uint *)(iVar7 + 0x20);
                    uVar3                  = *(uint *)(iVar7 + 0x22);
                    uVar4                  = uVar5 >> 0x1 | (uint)((uVar3 & 0x1) != 0x0) << 0xf;
                    uStack52               = CONCAT22(uVar3 >> 0x1, uVar4);
                    uVar4                  = (uVar3 & 0xfffe) + (uint)CARRY2(uVar4, uVar4);
                    uVar6                  = (ulong)uVar4;
                    param_4                = (uVar3 - uVar4) - (uint)(uVar5 < (uVar5 & 0xfffe));
                    *(int *)(iVar7 + 0x20) = uVar5 - (uVar5 & 0xfffe);
                    *(int *)(iVar7 + 0x22) = param_4;
                    if(uStack52 != 0x0)
                    {
                        uVar9 = 0x15;
                    LAB_1028_3b14:
                        uStack30 = local_28;
                        pass1_1020_bb8a(*(long **)(iVar7 + 0x28), (uint)uStack52, CONCAT22(uVar9, (int)((ulong)uStack52 >> 0x10)), param_5, param_6);
                    }
                    break;
                case 0xd:
                    iStack36                  = 0x19;
                    uStack30                  = local_28;
                    plVar1                    = (long *)(iVar7 + 0x24);
                    *plVar1                   = *plVar1 + local_28;
                    uVar5                     = *(uint *)(iVar7 + 0x24);
                    qVar2                     = (qword)(local_28 & 0xffff0000 | (ulong)uVar5) / 0x3;
                    uStack52                  = (long)qVar2;
                    uStack52._2_2_            = (int)(qVar2 >> 0x10);
                    uVar3                     = (uint)qVar2;
                    uVar4                     = uStack52._2_2_ * 0x3 + (uint)CARRY2(uVar3, uVar3) + (uint)CARRY2(uVar3 * 0x2, uVar3);
                    uVar6                     = (ulong)uVar4;
                    param_4                   = uVar5 + uVar3 * -0x3;
                    param_5                   = (*(int *)(iVar7 + 0x26) - uVar4) - (uint)(uVar5 < uVar3 * 0x3);
                    *(int *)(iVar7 + 0x24)    = param_4;
                    *(ushort *)(iVar7 + 0x26) = param_5;
                    if(uStack52 != 0x0)
                    {
                        uVar9 = 0x16;
                        goto LAB_1028_3b14;
                    }
                }
                if(((uStack30._2_2_ | (uint)uStack30) != 0x0) && (iStack36 != 0x0))
                {
                    pass1_1020_bb70(*(long **)(iVar7 + 0x28), (uint)uStack30, CONCAT22(iStack36, uStack30._2_2_), param_5, param_4, param_6);
                }
            }
        }
    }
    return;
}


ulong __stdcall16far pass1_1028_3c32(ulong *param_1)

{
    code **ppcVar1;
    int    iVar2;
    uint   local_6;
    int    iStack4;

    ppcVar1 = (code **)((int)*param_1 + 0x40);
    iVar2   = (**ppcVar1)();
    if(iVar2 != 0x0)
    {
        return 0x0;
    }
    return CONCAT22(-(uint)(0x3e8 < local_6) - iStack4, 0x3e8 - local_6);
}


void __stdcall16far pass1_1028_3c60(undefined4 param_1, ulong *param_2, ushort param_3, ushort param_4, ushort param_5)

{
    int       *piVar1;
    uint       uVar2;
    undefined4 uVar3;
    int        iVar4;
    undefined2 uVar5;
    undefined4 uVar6;
    long       local_10;
    undefined  local_c[0x4];
    int        iStack8;
    uint       uStack6;
    uint       uStack4;

    uVar6    = CONCAT22(param_4, param_3);
    *param_2 = 0x0;
    uVar5    = (undefined2)((ulong)param_1 >> 0x10);
    iVar4    = (int)param_1;
    if(*(long *)(iVar4 + 0x28) != 0x0)
    {
        iStack8 = 0x4;
        while(true)
        {
            if(0x1c < iStack8)
                break;
            uVar3            = *(undefined4 *)(iVar4 + 0x28);
            uVar6            = pass1_1020_bae6((ushort)uVar3, CONCAT22(iStack8, (int)((ulong)uVar3 >> 0x10)), (uint)uVar6, (uint)((ulong)uVar6 >> 0x10), param_5);
            uVar2            = *(uint *)param_2;
            *(uint *)param_2 = *(int *)param_2 + (uint)uVar6;
            piVar1           = (int *)((int)param_2 + 0x2);
            *piVar1          = *piVar1 + (int)((ulong)uVar6 >> 0x10) + (uint)CARRY2(uVar2, (uint)uVar6);
            if(0x3e7 < *param_2)
            {
                return;
            }
            iStack8 = iStack8 + 0x1;
        }
    }
    uVar6   = *(undefined4 *)(iVar4 + 0x28);
    uStack4 = *(uint *)((int)uVar6 + 0x4);
    uStack6 = 0x0;
    while(true)
    {
        if(uStack4 <= uStack6)
        {
            return;
        }
        pass1_1020_bb16(*(ulong **)(iVar4 + 0x28), (ulong *)CONCAT22(param_5, &local_10), (ushort *)CONCAT22(param_5, local_c), uStack6);
        *param_2 = *param_2 + local_10;
        if(0x3e7 < *param_2)
            break;
        uStack6 = uStack6 + 0x1;
    }
    return;
}


ulong __stdcall16far pass1_1028_3ec8(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined4 *)(param_1 + 0x20)           = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x4004;
    *(undefined2 *)(param_1 + 0x2)            = (int)&USHORT_1050_1028;
    pass1_1028_3fa2(CONCAT22(param_2, param_1));
    return CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_3f04(ulong param_1, uint param_2, uint param_3, ushort param_4, ushort param_5, ushort param_6)

{
    uint       uVar1;
    ulong      uVar2;
    uchar     *puVar3;
    int        iVar4;
    undefined2 uVar5;
    ushort     uVar6;
    ulong      uStack14;
    ulong      uStack10;
    ulong      uStack6;

    uVar6 = 0x1f;
    pass1_1028_b58e(param_1);
    uStack6  = CONCAT22(param_3, param_2);
    uStack10 = pass1_1030_7c28(CONCAT22(param_3, param_2), uVar6, param_2, param_3, param_6);
    puVar3   = (uchar *)(uStack10 >> 0x10);
    uVar2    = uStack10 & 0xffff;
    pass1_1030_7d1c(uStack6, 0x0, 0x1f0000, (int)uVar2, puVar3, param_4, param_5, param_6);
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(int *)(iVar4 + 0xc) != 0x22)
    {
        if(*(int *)(iVar4 + 0xc) == 0x23)
        {
            uVar1 = 0xa;
        }
        else
        {
            uVar1 = 0x5;
        }
        uStack14                 = (ulong)uVar1;
        uStack10                 = uStack10 + *(long *)(iVar4 + 0x20);
        *(ulong *)(iVar4 + 0x20) = uStack10 % (ulong)uVar1;
        uVar2                    = uStack10 / uStack14;
        puVar3                   = (uchar *)(uStack10 % uStack14);
        uStack10                 = uStack10 + uVar2;
    }
    pass1_1030_7ddc(uStack6, uStack10, 0x21, (uint)uVar2, puVar3, param_4, param_5, param_6);
    return;
}


void __stdcall16far pass1_1028_3fa2(ulong param_1)

{
    uint       uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(int *)(iVar2 + 0xc) != 0x22)
    {
        if(*(int *)(iVar2 + 0xc) == 0x23)
        {
            uVar1 = 0xa;
        }
        else
        {
            uVar1 = 0x5;
        }
        uVar1 = uVar1 >> 0x1;
        pass1_1008_612e(0x0, uVar1, uVar1);
        *(uint *)(iVar2 + 0x20) = uVar1;
        *(int *)(iVar2 + 0x22)  = (int)uVar1 >> 0xf;
    }
    return;
}


void __stdcall16far pass1_1028_1b1e(ulong param_1)

{
    *(undefined2 *)((int)param_1 + 0x14) = 0x7;
    return;
}


ushort *__stdcall16far pass1_1028_1be8(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)(param_1 + 0x20)       = 0x0;
    *(undefined2 *)(param_1 + 0x22)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x1eee;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1028_1c1c(void)

{
    return 0x0;
}


ushort __stdcall16far pass1_1028_1c22(ulong param_1)

{
    ushort uVar1;
    int    iVar2;
    uint   uVar3;

    uVar3 = (uint)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if((*(int *)(iVar2 + 0x20) != 0x0) && ((*(int *)(iVar2 + 0x12) == 0x5 || (*(int *)(iVar2 + 0x12) == 0x6))))
    {
        if(*(int *)(iVar2 + 0xc) == 0x16)
        {
            return 0x19;
        }
        if(*(int *)(iVar2 + 0xc) == 0x17)
        {
            return 0x1a;
        }
    }
    uVar1 = pass1_1028_bc1c(param_1 & 0xffff | (ulong)uVar3 << 0x10);
    return uVar1;
}


void __stdcall16far pass1_1028_1c66(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    code **ppcVar1;
    int    iVar2;
    ulong  uVar3;

    if(*(int *)((int)param_1 + 0x12) != 0x6)
    {
        return;
    }
    uVar3 = pass1_1028_b4f2((ulong)param_1);
    if(*(long *)((int)uVar3 + 0x200) != 0x8000002)
    {
        ppcVar1 = (code **)((int)*param_1 + 0x64);
        iVar2   = (**ppcVar1)(param_4, param_1);
        if(iVar2 == 0x0)
        {
            return;
        }
        pass1_1028_cb04((ulong)param_1, param_2, param_3, param_5);
        if(iVar2 == 0x0)
        {
            iVar2 = 0x6;
            goto LAB_1028_1cbd;
        }
        pass1_1028_c952((ulong)param_1, param_2, param_3, param_5);
    }
    iVar2 = 0x5;
LAB_1028_1cbd:
    pass1_1028_bdac(param_1, iVar2, param_4);
    return;
}


// WARNING: Could not reconcile some variable overlaps

ushort __stdcall16far pass1_1028_1cca(ulong param_1, ulong *param_2, uint param_3, ushort param_4, ushort param_5, long param_6, ushort param_7)

{
    ushort     uVar1;
    ushort     uVar2;
    ushort     uVar3;
    undefined  local_e[0x2];
    int        local_c;
    int        local_a;
    undefined4 local_8;
    undefined2 uStack4;

    local_8 = *param_2;
    uStack4 = *(undefined2 *)(param_2 + 0x1);
    pass1_1008_3eb4((ushort *)CONCAT22(param_7, &local_8), (ushort *)CONCAT22(param_7, local_e), (ushort *)CONCAT22(param_7, &local_c), (ushort *)CONCAT22(param_7, &local_a));
    local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
    uVar2   = (ushort)param_1;
    uVar3   = (ushort)(param_1 >> 0x10);
    uVar1   = pass1_1028_1e14(uVar2, uVar3, 0x16, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
    if(uVar1 == 0x0)
    {
        local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
        uVar1   = pass1_1028_1e14(uVar2, uVar3, 0x16, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
        if(uVar1 == 0x0)
        {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            uVar1         = pass1_1028_1e14(uVar2, uVar3, 0x17, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
            if(uVar1 == 0x0)
            {
                local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                uVar1   = pass1_1028_1e14(uVar2, uVar3, 0x17, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
                if(uVar1 == 0x0)
                {
                    return uVar1;
                }
            }
        }
    }
    return 0x1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_1da4(ushort param_1, ushort param_2, ushort *param_3, ulong param_4, long param_5, ushort param_6)

{
    int        iVar1;
    undefined *puVar2;
    uint       uVar3;
    ulong      uVar4;
    undefined  local_4[0x2];

    uVar4 = pass1_1030_bcae((ushort)local_4, param_6);
    uVar3 = (uint)(uVar4 >> 0x10);
    iVar1 = (int)uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
    uVar4 = *(ulong *)(iVar1 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar4, (uint)(uVar4 >> 0x10));
    puVar2 = local_4;
    pass1_1030_bcde(param_6, (ushort)puVar2, param_6, uVar4 & 0xffff | (ulong)uVar3 << 0x10, param_3, param_5);
    if((int)puVar2 < 0x0)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
        return;
    }
    return;
}


ushort __stdcall16far pass1_1028_1e8a(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uint       uVar1;
    ulong      uVar2;
    uint       uVar3;
    undefined2 uVar4;
    undefined2 uVar5;

    uVar1 = (uint)((ulong)param_1 >> 0x10);
    if((*(byte *)((int)param_1 + 0x1a) & 0x2) == 0x0)
    {
        uVar4 = 0x0;
        uVar5 = 0x23;
        uVar3 = 0x1;
        uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        pass1_1030_7d7c(uVar2, uVar3, CONCAT22(uVar5, uVar4), (uint)uVar2, (uchar *)(uVar2 >> 0x10), param_2, param_3, param_4);
        pass1_1028_bdac(param_1, 0x6, 0x1030);
        return 0x0;
    }
    return 0x1;
}


void __stdcall16far pass1_1028_2042(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    undefined4   uVar4;
    astruct_602 *iVar5;
    undefined2   uVar5;
    ulong        uVar6;

    uVar5                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar5                             = (astruct_602 *)param_1;
    *param_1                          = 0x2572;
    iVar5->field_0x2                  = (int)&USHORT_1050_1028;
    uVar4                             = *(undefined4 *)&iVar5->field_0x20;
    *(undefined2 *)((int)uVar4 + 0xa) = 0x1;
    puVar1                            = iVar5->field_0x20;
    uVar2                             = iVar5->field_0x22;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    if((_PTR_LOOP_1050_65e2 != 0x0) && (_PTR_LOOP_1050_5a64 != 0x0))
    {
        uVar6 = pass1_1028_b58e((ulong)param_1);
        pass1_1038_79f2(_PTR_LOOP_1050_5a64, uVar6, param_2);
    }
    pass1_1028_b418(param_1);
    return;
}


ushort __stdcall16far pass1_1028_20b0(void)

{
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_20b6(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    uint       uVar1;
    uchar     *puVar2;
    ushort     uVar3;
    uint       uVar4;
    undefined  auStack22[0x2];
    int        iStack20;
    int        iStack18;
    ulong      uStack16;
    undefined2 uStack12;
    ulong      uStack10;
    ulong      uStack6;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    uVar4 = (uint)((ulong)param_1 >> 0x10);
    uVar3 = (ushort)param_1;
    if(*(int *)(uVar3 + 0x12) == 0x5)
    {
        uStack6  = pass1_1028_bb24((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10);
        uStack10 = pass1_1028_b58e((ulong)param_1);
        puVar2   = (uchar *)(uStack10 >> 0x10);
        uStack16 = *(ulong *)((int)uStack10 + 0xc);
        uStack12 = *(undefined2 *)((int)uStack10 + 0x10);
        pass1_1008_3eb4((ushort *)CONCAT22(param_5, &uStack16), (ushort *)CONCAT22(param_5, auStack22), (ushort *)CONCAT22(param_5, &iStack20), (ushort *)CONCAT22(param_5, &iStack18));
        uStack16 = uStack16 & 0xffff | (ulong)(iStack20 - 0x1) << 0x10;
        uVar1    = pass1_1028_21ba(uVar3, uVar4, (ushort *)CONCAT22(param_5, &uStack16), uStack6, (uint)&uStack16, (uint)puVar2, param_5);
        if(uVar1 == 0x0)
        {
            uStack16 = uStack16 & 0xffff | (ulong)(iStack20 + 0x1) << 0x10;
            uVar1    = pass1_1028_21ba(uVar3, uVar4, (ushort *)CONCAT22(param_5, &uStack16), uStack6, (uint)&uStack16, (uint)puVar2, param_5);
            if(uVar1 == 0x0)
            {
                uStack16 = CONCAT22(iStack20, iStack18 + -0x1);
                uVar1    = pass1_1028_21ba(uVar3, uVar4, (ushort *)CONCAT22(param_5, &uStack16), uStack6, (uint)&uStack16, (uint)puVar2, param_5);
                if(uVar1 == 0x0)
                {
                    uStack16 = uStack16 & 0xffff0000 | (ulong)(iStack18 + 0x1);
                    uVar1    = pass1_1028_21ba(uVar3, uVar4, (ushort *)CONCAT22(param_5, &uStack16), uStack6, (uint)&uStack16, (uint)puVar2, param_5);
                    if(uVar1 == 0x0)
                    {
                        return;
                    }
                }
            }
        }
        pass1_1038_79b2(_PTR_LOOP_1050_5a64, uStack10, uVar1, puVar2);
    }
    return;
}


int __stdcall16far pass1_1028_2290(ulong param_1, ulong *param_2, ushort param_3, ushort param_4, ushort param_5, long param_6, ushort param_7)

{
    int        iVar1;
    undefined2 uVar2;
    undefined2 uVar3;
    undefined  local_e[0x2];
    int        local_c;
    int        local_a;
    undefined4 local_8;
    undefined2 uStack4;

    local_8 = *param_2;
    uStack4 = *(undefined2 *)(param_2 + 0x1);
    pass1_1008_3eb4((ushort *)CONCAT22(param_7, &local_8), (ushort *)CONCAT22(param_7, local_e), (ushort *)CONCAT22(param_7, &local_c), (ushort *)CONCAT22(param_7, &local_a));
    local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
    uVar2   = (undefined2)param_1;
    uVar3   = (undefined2)(param_1 >> 0x10);
    iVar1   = pass1_1028_2220(uVar2, uVar3, 0x16, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
    if(iVar1 == 0x0)
    {
        local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
        iVar1   = pass1_1028_2220(uVar2, uVar3, 0x16, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
        if(iVar1 == 0x0)
        {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            iVar1         = pass1_1028_2220(uVar2, uVar3, 0x17, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
            if(iVar1 == 0x0)
            {
                local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                iVar1   = pass1_1028_2220(uVar2, uVar3, 0x17, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
                if(iVar1 == 0x0)
                {
                    return iVar1;
                }
            }
        }
    }
    return 0x1;
}


ushort __stdcall16far pass1_1028_236a(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uint       uVar1;
    ulong      uVar2;
    uint       uVar3;
    undefined2 uVar4;
    undefined2 uVar5;

    uVar1 = (uint)((ulong)param_1 >> 0x10);
    if((*(byte *)((int)param_1 + 0x1a) & 0x2) == 0x0)
    {
        uVar4 = 0x0;
        uVar5 = 0x23;
        uVar3 = 0x1;
        uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        pass1_1030_7d7c(uVar2, uVar3, CONCAT22(uVar5, uVar4), (uint)uVar2, (uchar *)(uVar2 >> 0x10), param_2, param_3, param_4);
        pass1_1028_bdac(param_1, 0x6, 0x1030);
        return 0x0;
    }
    return 0x1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_23a8(ushort param_1, ushort param_2, ushort *param_3, ulong param_4, long param_5, ushort param_6)

{
    int        iVar1;
    undefined *puVar2;
    uint       uVar3;
    ulong      uVar4;
    undefined  local_4[0x2];

    uVar4 = pass1_1030_bcae((ushort)local_4, param_6);
    uVar3 = (uint)(uVar4 >> 0x10);
    iVar1 = (int)uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
    uVar4 = *(ulong *)(iVar1 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar4, (uint)(uVar4 >> 0x10));
    puVar2 = local_4;
    pass1_1030_bcde(param_6, (ushort)puVar2, param_6, uVar4 & 0xffff | (ulong)uVar3 << 0x10, param_3, param_5);
    if((int)puVar2 < 0x0)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
        return;
    }
    return;
}


ushort *__stdcall16far pass1_1028_25fc(int param_1, undefined2 param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = (int)s_fem16_wav_1050_2644 + 0x8;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_26d6(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x2788;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_2700(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    uint  uVar1;
    ulong uVar2;

    pass1_1028_be2a(param_1, param_2, param_3, param_4, param_5);
    uVar1 = (uint)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        uVar2                               = pass1_1028_b4f2((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        *(undefined2 *)((int)uVar2 + 0x204) = 0x1;
    }
    return;
}


void __stdcall16far pass1_1028_272e(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    ulong uVar1;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    uVar1 = pass1_1028_b4f2((ulong)param_1);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        *(undefined2 *)((int)uVar1 + 0x204) = 0x1;
    }
    return;
}


ushort *__stdcall16far pass1_1028_2812(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x2a92;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    *(undefined2 *)(param_1 + 0xe)        = *(undefined2 *)(param_1 + 0xc);
    return (ushort *)CONCAT22(param_2, param_1);
}


// WARNING: Could not reconcile some variable overlaps

ushort __stdcall16far pass1_1028_2844(ulong param_1, ulong *param_2, uint param_3, ushort param_4, ushort param_5, long param_6, ushort param_7)

{
    BOOL16     BVar1;
    ushort     uVar2;
    ushort     uVar3;
    ushort     uVar4;
    undefined  local_e[0x2];
    int        local_c;
    int        local_a;
    undefined4 local_8;
    undefined2 uStack4;

    local_8 = *param_2;
    uStack4 = *(undefined2 *)(param_2 + 0x1);
    pass1_1008_3eb4((ushort *)CONCAT22(param_7, &local_8), (ushort *)CONCAT22(param_7, local_e), (ushort *)CONCAT22(param_7, &local_c), (ushort *)CONCAT22(param_7, &local_a));
    local_8 = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
    uVar3   = (ushort)param_1;
    uVar4   = (ushort)(param_1 >> 0x10);
    BVar1   = pass1_1028_c5a6(uVar3, uVar4, 0x7b, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
    if(BVar1 == 0x0)
    {
        uVar2 = pass1_1028_297c(param_1, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
        if(uVar2 == 0x0)
        {
            local_8 = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
            BVar1   = pass1_1028_c5a6(uVar3, uVar4, 0x7b, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
            if(BVar1 == 0x0)
            {
                uVar2 = pass1_1028_297c(param_1, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
                if(uVar2 == 0x0)
                {
                    local_8._0_2_ = local_a + -0x1;
                    local_8._2_2_ = local_c;
                    BVar1         = pass1_1028_c5a6(uVar3, uVar4, 0x7c, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
                    if(BVar1 == 0x0)
                    {
                        uVar2 = pass1_1028_297c(param_1, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
                        if(uVar2 == 0x0)
                        {
                            local_8 = CONCAT22(local_8._2_2_, local_a + 0x1);
                            BVar1   = pass1_1028_c5a6(uVar3, uVar4, 0x7c, (ushort *)CONCAT22(param_7, &local_8), param_6, (uint)&local_8, param_3, param_7);
                            if(BVar1 == 0x0)
                            {
                                uVar3 = pass1_1028_297c(param_1, (ushort *)CONCAT22(param_7, &local_8), param_6, &local_8, param_3, param_7);
                                if(uVar3 == 0x0)
                                {
                                    return uVar3;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0x1;
}


ushort __stdcall16far pass1_1028_297c(ulong param_1, ushort *param_2, long param_3, uint param_4, uint param_5, ushort param_6)

{
    char  cVar1;
    uint  uVar2;
    ulong uVar3;

    pass1_1028_c7b6(param_6, param_5, (ushort)param_1, (ushort)(param_1 >> 0x10), param_2, param_3);
    if(param_4 == 0x0)
    {
        pass1_1030_627e(param_6, 0x0, param_5, _PTR_LOOP_1050_5740, param_2, param_3);
        uVar2 = param_5 | param_4;
        if(uVar2 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4, param_5);
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_4));
            uVar2 = *(uint *)((int)uVar3 + 0xc);
            if(0x4a < (int)uVar2)
            {
                switch(uVar2)
                {
                case 0x4c:
                case 0x4d:
                case 0x4e:
                case 0x60:
                case 0x61:
                case 0x62:
                case 0x63:
                case 0x6e:
                case 0x73:
                case 0x74:
                case 0x75:
                case 0x76:
                case 0x77:
                case 0x78:
                case 0x79:
                    goto switchD_1028_2a0b_caseD_4c;
                default:
                    goto switchD_1028_2a0b_caseD_4f;
                }
            }
            if(((int)uVar2 < 0x48) && (uVar2 != 0x41))
            {
                if(uVar2 < 0x42)
                {
                    cVar1 = (char)uVar2;
                    if(cVar1 < '5')
                    {
                        if('2' < cVar1)
                        {
                            return 0x0;
                        }
                        cVar1 = cVar1 + -0x10;
                    }
                    else
                    {
                        cVar1 = cVar1 + -0x3e;
                    }
                    if(cVar1 == '\0')
                    {
                        return 0x0;
                    }
                }
            switchD_1028_2a0b_caseD_4f:
                return 0x1;
            }
        }
    }
switchD_1028_2a0b_caseD_4c:
    return 0x0;
}


ushort *__stdcall16far pass1_1028_2b1c(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1030_dcc2(param_1, param_2, param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x2b74;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort *__stdcall16far pass1_1028_0982(astruct_179 *param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    param_1->field_0x20                   = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0xada;
    param_1->field_0x2                    = (int)&USHORT_1050_1028;
    param_1->field_0xe                    = 0x4b;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_09b8(ulong param_1)

{
    undefined4 uVar1;

    uVar1                              = pass1_1028_b58e(param_1);
    *(undefined2 *)((int)uVar1 + 0x14) = 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_09d4(ushort param_1, int param_2, ushort param_3, ulong param_4, ushort *param_5, ulong param_6, long param_7)

{
    int        iVar1;
    undefined *puVar2;
    uint       uVar3;
    ulong      uVar4;
    ushort     uVar5;
    ushort     uVar6;
    uint       uVar7;
    undefined  local_6[0x2];
    BOOL16     BStack4;

    uVar5   = (ushort)param_4;
    uVar6   = (ushort)(param_4 >> 0x10);
    uVar7   = (uint)(param_6 >> 0x10);
    BStack4 = pass1_1028_c314(param_1, param_2, param_3, uVar5, uVar6, param_5, (ushort)param_6, uVar7, param_7);
    if(BStack4 == 0x0)
    {
        return;
    }
    pass1_1028_c7b6(param_1, param_3, uVar5, uVar6, param_5, param_7);
    if((BStack4 != 0x0) && (BStack4 != 0x4))
    {
        if(((int)(BStack4 - 0x5) < 0x1) || ((SBORROW2(BStack4 - 0x5, 0x1) || (0x3 < (int)(BStack4 - 0x6)))))
        {
            if((*(int *)(uVar5 + 0xc) != 0x3e) && (*(int *)(uVar5 + 0xc) != 0x41))
            {
                return;
            }
            uVar4 = pass1_1030_bcae((ushort)local_6, param_1);
            uVar3 = (uint)(uVar4 >> 0x10);
            iVar1 = (int)uVar4;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_6, uVar7);
            uVar4 = *(ulong *)(iVar1 + 0x10);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar4, (uint)(uVar4 >> 0x10));
            puVar2 = local_6;
            pass1_1030_bcde(param_1, (ushort)puVar2, param_1, uVar4 & 0xffff | (ulong)uVar3 << 0x10, param_5, param_7);
            if((int)puVar2 < 0x0)
            {
                PTR_LOOP_1050_50ca = (undefined *)0x6af;
                return;
            }
            if(0x5 < (int)puVar2)
            {
                PTR_LOOP_1050_50ca = (undefined *)0x6b5;
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = (undefined *)0x6a8;
    return;
}
