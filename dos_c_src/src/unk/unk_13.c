

void __stdcall16far pass1_1010_0f76(ushort *param_1, ushort param_2)

{
    uint uVar1;

    uVar1                               = (uint)((ulong)param_1 >> 0x10);
    *param_1                            = (int)s_648_bmp_1050_1919 + 0x1;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    pass1_1010_17c0((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1010_2db2(param_1, param_2);
    return;
}

void __stdcall16far pass1_1010_1146(ulong param_1, ushort param_2, int param_3, ushort param_4)

{
    undefined4 uVar1;
    ushort     uVar2;

    DAT_1050_0ecc = param_2;
    uVar2         = (ushort)(param_1 >> 0x10);
    uVar1         = *(undefined4 *)((int)param_1 + 0x66);
    pass1_1000_4aea(*(uint *)((int)param_1 + 0x64), (uint)uVar1, (int)((ulong)uVar1 >> 0x10), 0x4, (uchar *)((int)s_dibtext_bmp_1050_1844 + 0x6), (int)&stack0xfffe, param_3, uVar2, 0x1000, param_4);
    return;
}

void __stdcall16far pass1_1010_116c(ulong *param_1, int param_2, ushort param_3)

{
    code     **ppcVar1;
    int        iVar2;
    undefined2 uVar3;
    int        iVar4;
    undefined2 uVar5;
    undefined4 uVar6;
    ushort     uStack4;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0x56) != 0x0)
    {
        ppcVar1 = (code **)((int)*param_1 + 0x34);
        (**ppcVar1)();
    }
    ppcVar1 = (code **)((int)*param_1 + 0x28);
    uVar6   = (**ppcVar1)();
    uVar3   = (undefined2)((ulong)uVar6 >> 0x10);
    if((int)uVar6 != 0x0)
    {
        uStack4 = DAT_1050_0ecc;
        iVar2   = DAT_1050_0ecc + 0x1;
        if(iVar2 == 0x0)
        {
            uStack4 = 0x0;
        }
        pass1_1010_1146((ulong)param_1, uStack4, param_2, param_3);
        pass1_1010_11c6((ulong)param_1, iVar2, uVar3);
        *(int *)(iVar4 + 0x56)        = iVar2;
        *(undefined2 *)(iVar4 + 0x58) = uVar3;
    }
    return;
}

void __stdcall16far pass1_1008_e852(ushort param_1, ushort param_2, ulong param_3, ushort param_4, uint param_5)

{
    undefined *puVar1;
    int        iVar2;
    char      *pcVar3;
    undefined  local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
    do
    {
        puVar1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar1));
        if((param_5 | (uint)puVar1) == 0x0)
        {
            return;
        }
        pcVar3  = pass1_1038_4d28(CONCAT22(param_5, puVar1));
        param_5 = (uint)((ulong)pcVar3 >> 0x10);
        iVar2   = pass1_1000_3d7a(param_3, (ulong)pcVar3 & 0xffff | (ulong)param_5 << 0x10);
    } while(iVar2 != 0x0);
    return;
}

long __stdcall16far pass1_1008_e8cc(ushort param_1, ulong param_2, ulong param_3, ulong param_4)

{
    undefined4 uVar1;
    uint       uVar2;
    uint       uVar3;
    int        iVar4;
    uint       uVar5;
    uint       uVar6;
    long       lVar7;
    char      *pcVar8;
    char      *pcVar9;
    ulong      uStack22;
    ulong      uStack18;
    undefined  local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_1, local_a), *(ulong *)((int)param_2 + 0xa));
    while(true)
    {
        lVar7 = pass1_1008_5b12(local_a, param_1);
        uVar5 = (uint)((ulong)lVar7 >> 0x10);
        uVar2 = (uint)lVar7;
        uVar6 = uVar5 | uVar2;
        if(lVar7 == 0x0)
        {
            return 0x0;
        }
        uVar1 = *(undefined4 *)(uVar2 + 0x4);
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        uStack18 = CONCAT22(uVar6, uVar3);
        uVar1    = *(undefined4 *)(uVar2 + 0x8);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        uStack22 = CONCAT22(uVar6, uVar3);
        pcVar8   = pass1_1038_4d28(uStack18);
        pcVar9   = pass1_1038_4d28(uStack22);
        iVar4    = pass1_1000_3d7a(param_4, (ulong)pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3, (ulong)pcVar9), iVar4 == 0x0))
            break;
        iVar4 = pass1_1000_3d7a(param_3, (ulong)pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4, (ulong)pcVar9), iVar4 == 0x0))
        {
            return lVar7;
        }
    }
    return lVar7;
}


ulong __stdcall16far pass1_1008_eb5c(ushort param_1, ushort param_2, int param_3)

{
    return CONCAT22(0x1050, param_3 * 0x10 + 0xd0e);
}


ushort __stdcall16far pass1_1008_eb6e(void)

{
    return 0x5;
}

void __stdcall16far pass1_1008_ec94(ushort *param_1)

{
    *param_1                            = 0xefc4;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    pass1_1010_3880(param_1);
    return;
}

void __stdcall16far pass1_1008_ed00(ushort *param_1, ushort param_2)

{
    *param_1                            = 0xef9c;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    pass1_1010_2db2(param_1, param_2);
    return;
}


void __stdcall16far pass1_1008_ed62(ulong param_1, int param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                         = (undefined2)(param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(int *)(iVar1 + 0x16)        = param_2 * 0x8 + 0xd5e;
    *(undefined2 *)(iVar1 + 0x18) = (int)&USHORT_1050_1050;
    *(undefined2 *)(iVar1 + 0x12) = *(undefined2 *)(param_2 * 0x8 + 0xd64);
    return;
}
