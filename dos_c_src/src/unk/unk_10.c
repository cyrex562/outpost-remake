

ushort *__stdcall16far pass1_1020_d3a4(ushort *param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    undefined2 uVar1;

    pass1_1028_b39e(param_1, param_3, param_4, param_5);
    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0x20)    = param_2;
    *param_1                            = 0xd53e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

ushort __stdcall16far pass1_1020_d460(ulong *param_1, ushort *param_2, ulong param_3, ulong param_4, int param_5, ushort param_6, int param_7, undefined8 param_8)

{
    ushort     uVar1;
    uchar     *puVar2;
    ushort     unaff_SS;
    undefined4 uVar3;
    ushort    *puVar4;

    uVar1 = pass1_1028_bc90(param_1, param_2, param_3, param_4, param_5, param_6, unaff_SS);
    if(uVar1 != 0x0)
    {
        uVar3              = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)((ushort)_PTR_LOOP_1050_4230 + 0x16), 0x11, param_6, (ushort)_PTR_LOOP_1050_4230, (ushort)&PTR_LOOP_1050_1038, unaff_SS);
        puVar2             = (uchar *)((ulong)uVar3 >> 0x10);
        PTR_LOOP_1050_5b80 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
        unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5b80, 0x1008, unaff_SS);
        puVar4                               = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3a, unaff_SS, puVar2, param_7);
        *(undefined2 *)((int)param_1 + 0x20) = *(undefined2 *)((int)puVar4 + 0xa);
        uVar1                                = 0x1;
    }
    return uVar1;
}


void __stdcall16far pass1_1020_d4ca(ulong param_1, int param_2)

{
    BOOL16 BVar1;
    ulong  uVar2;
    uint   extraout_DX;
    uint   uVar3;
    int    iVar4;

    if(*(int *)((int)param_1 + 0x20) == 0x2)
    {
        return;
    }
    pass1_1028_b58e(param_1);
    uVar2 = *(ulong *)(param_2 + 0x2e);
    iVar4 = 0x63;
    uVar3 = extraout_DX;
    pass1_1038_3fb0(uVar2);
    BVar1 = pass1_1030_25b2(uVar2 & 0xffff | (ulong)uVar3 << 0x10, iVar4);
    if(BVar1 != 0x0)
    {
        return;
    }
    return;
}

ushort *__stdcall16far pass1_1020_d5c8(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xd7fe;
    *(undefined2 *)(param_1 + 0x2)        = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_d5f2(ulong param_1, ulong param_2, int param_3, ushort param_4)

{
    code      **ppcVar1;
    ushort      uVar2;
    undefined4 *puVar3;
    ushort      extraout_DX;
    uint        uVar4;
    ulong       uVar5;
    undefined4 *puVar6;
    ulong      *puVar7;
    byte        bStack55;
    undefined   local_32[0xa];
    undefined4  uStack40;
    undefined4  uStack36;
    undefined4 *puStack28;
    undefined4  local_1a;
    int         iStack22;
    ushort      uStack20;
    int         iStack18;
    ushort      uStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_b58e(param_1);
    local_c   = *(undefined4 *)(param_3 + 0xc);
    iStack18  = *(int *)(param_3 + 0x10);
    puStack28 = &local_c;
    uStack16  = extraout_DX;
    iStack14  = iStack18;
    iStack8   = iStack18;
    iStack6   = param_3;
    uStack4   = extraout_DX;
    pass1_1028_bab6(param_1, iStack18, extraout_DX);
    uVar2    = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    iStack22 = iStack8;
    uStack36 = CONCAT22(uStack36._2_2_, &local_1a);
    iStack14 = iStack14 + 0x1;
    uStack20 = uVar2;
    if(iStack14 < (int)uVar2)
    {
        puVar7   = (ulong *)CONCAT22(param_4, local_32);
        iStack22 = iStack14;
        uVar5    = pass1_1028_bb24(param_1);
        uVar4    = (uint)(uVar5 >> 0x10);
        puVar3   = &local_1a;
        pass1_1030_64ce(param_4, puVar3, uVar4, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_4, puVar3), uVar5 & 0xffff | (ulong)uVar4 << 0x10, puVar7);
        uStack40 = *puVar3;
        uVar4    = *(uint *)((int)puVar3 + 0x2);
        bStack55 = (byte)((ulong)uStack40 >> 0x18);
        uVar2    = (ushort)bStack55;
        uStack36 = uStack40;
        if(bStack55 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack40, uVar4);
            puVar6  = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar4, uVar2));
            uVar2   = (ushort)puVar6;
            ppcVar1 = (code **)((int)*puVar6 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(param_1, param_2, uVar2);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_d6e6(ulong param_1, int param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    uint        uVar3;
    ushort      extraout_DX;
    uint        uVar4;
    ulong       uVar5;
    undefined4 *puVar6;
    ulong      *puVar7;
    byte        bStack55;
    undefined   local_32[0xa];
    undefined4  uStack40;
    undefined4  uStack36;
    undefined4 *puStack28;
    undefined4  local_1a;
    int         iStack22;
    ushort      uStack20;
    int         iStack18;
    ushort      uStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_c   = *(undefined4 *)(param_2 + 0xc);
    iStack18  = *(int *)(param_2 + 0x10);
    puStack28 = &local_c;
    uStack16  = extraout_DX;
    iStack14  = iStack18;
    iStack8   = iStack18;
    iStack6   = param_2;
    uStack4   = extraout_DX;
    pass1_1028_bab6(param_1, iStack18, extraout_DX);
    uStack20 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    uStack36 = CONCAT22(uStack36._2_2_, &local_1a);
    iStack22 = iStack14 + 0x1;
    if(iStack22 < (int)uStack20)
    {
        puVar7   = (ulong *)CONCAT22(param_3, local_32);
        iStack14 = iStack22;
        uVar5    = pass1_1028_bb24(param_1);
        uVar4    = (uint)(uVar5 >> 0x10);
        puVar2   = &local_1a;
        pass1_1030_64ce(param_3, puVar2, uVar4, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_3, puVar2), uVar5 & 0xffff | (ulong)uVar4 << 0x10, puVar7);
        uStack40 = *puVar2;
        uVar4    = *(uint *)((int)puVar2 + 0x2);
        bStack55 = (byte)((ulong)uStack40 >> 0x18);
        uVar3    = (uint)bStack55;
        if(bStack55 != 0x0)
        {
            uStack36 = uStack40;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack40, uVar4);
            puVar6 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar4, uVar3));
            if(*(int *)((int)puVar6 + 0xc) == 0x6a)
            {
                ppcVar1 = (code **)((int)*puVar6 + 0x24);
                (**ppcVar1)();
            }
        }
    }
    return;
}

void __stdcall16far pass1_1020_b97e(ushort param_1, int param_2, ushort param_3, ushort param_4, ushort param_5, int param_6)

{
    undefined4 uVar1;
    int        local_e;
    ushort     local_c;
    int        iStack10;
    ushort     uStack8;
    undefined4 uStack6;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x2, 0x400);
    _PTR_LOOP_1050_4e70 = CONCAT22(param_3, param_2);
    uVar1               = *(undefined4 *)(param_2 + 0x10);
    uStack6             = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    iStack10 = (int)uVar1;
    uStack8  = param_3;
    pass1_1008_3f62((ushort *)&PTR_LOOP_1048_4230, (ushort *)CONCAT22(param_3, iStack10 + 0xc));
    pass1_1008_3e94((ushort *)&PTR_LOOP_1048_4230, (ushort *)CONCAT22(param_1, &local_e), (ushort *)CONCAT22(param_1, &local_c));
    if(param_6 == 0x0)
    {
        pass1_1008_3e76((ushort *)&PTR_LOOP_1048_4230, 0x0, local_e + 0x1, local_c - 0x1);
        pass1_1008_3e94((ushort *)&PTR_LOOP_1048_4230, (ushort *)CONCAT22(param_1, &local_e), (ushort *)CONCAT22(param_1, &local_c));
    }
    pass1_1008_3e76((ushort *)0x10484236, 0x1, local_e - 0x2, local_c);
    return;
}

void __cdecl16far pass1_1020_ba2b(void)

{
    init_globals_1020_96d4();
    pass1_1020_a426();
    return;
}

void __stdcall16far pass1_1020_ba3e(long *param_1, ushort param_2, int param_3, ushort param_4, ushort param_5)

{
    astruct_172 *iVar1;
    undefined2   uVar1;
    ushort       unaff_SS;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_172 *)param_1;
    *param_1         = 0x0;
    iVar1->field_0x4 = 0x0;
    iVar1->field_0x6 = param_3;
    iVar1->field_0x8 = param_2;
    if(iVar1->field_0x6 == 0x0)
    {
        iVar1->field_0x6 = 0x5;
    }
    pass1_1020_bcc4(param_1, param_4, unaff_SS);
    return;
}

void __stdcall16far pass1_1020_ba94(long *param_1)

{
    uint *puVar1;
    uint  uStack8;

    if(*param_1 == 0x0)
    {
        return;
    }
    uStack8 = 0x0;
    while(true)
    {
        puVar1 = (uint *)((int)param_1 + 0x4);
        if(*puVar1 < uStack8 || *puVar1 == uStack8)
            break;
        uStack8 = uStack8 + 0x1;
    }
    return;
}


undefined4 __stdcall16far pass1_1020_bae6(ushort param_1, ulong param_2, uint param_3, uint param_4, ushort param_5)

{
    undefined2 *puStack6;

    pass1_1020_bc92((uint *)CONCAT22((int)param_2, param_1), (ushort)(param_2 >> 0x10), param_5);
    puStack6 = (undefined2 *)CONCAT22(param_4, param_3);
    if((param_4 | param_3) != 0x0)
    {
        return CONCAT22(*(undefined2 *)(param_3 + 0x2), *puStack6);
    }
    return 0x0;
}


void __stdcall16far pass1_1020_bb16(ulong *param_1, ulong *param_2, ushort *param_3, uint param_4)

{
    if(*(uint *)((int)param_1 + 0x4) < param_4)
    {
        *param_3 = 0x0;
        *param_2 = 0x0;
        return;
    }
    *param_3 = *(ushort *)(param_4 * 0x6 + (int)*param_1 + 0x4);
    *param_2 = *(ulong *)((int)*param_1 + param_4 * 0x6);
    return;
}


void __stdcall16far pass1_1020_bb70(long *param_1, uint param_2, undefined4 param_3, ushort param_4, undefined2 param_5, ushort param_6)

{
    pass1_1020_bba4(param_1, 0x1, param_2, (int)param_3, (ushort)((ulong)param_3 >> 0x10), param_4, param_6);
    return;
}


void __stdcall16far pass1_1020_bb8a(long *param_1, uint param_2, undefined4 param_3, ushort param_4, ushort param_5)

{
    pass1_1020_bba4(param_1, 0x0, param_2, (int)param_3, (ushort)((ulong)param_3 >> 0x10), param_4, param_5);
    return;
}


BOOL16 __stdcall16far pass1_1020_bba4(long *param_1, int param_2, uint param_3, int param_4, ushort param_5, ushort param_6, ushort param_7)

{
    uint *in_AX;
    uint  in_DX;
    uint  uVar1;
    uint  uVar2;
    bool  bVar3;
    uint *puStack6;

    pass1_1020_bc92((uint *)param_1, param_5, param_7);
    puStack6 = (uint *)CONCAT22(in_DX, in_AX);
    uVar1    = in_DX | (uint)in_AX;
    if(uVar1 == 0x0)
    {
        pass1_1020_bc92((uint *)param_1, 0x0, param_7);
        uVar2 = uVar1 | (uint)in_AX;
        if(uVar2 == 0x0)
        {
            pass1_1020_bcc4(param_1, param_6, param_7);
            pass1_1020_bc92((uint *)param_1, 0x0, param_7);
            if((uVar2 | (uint)in_AX) == 0x0)
            {
                return 0x0;
            }
            in_AX[0x2] = param_5;
            uVar1      = uVar2;
        }
        else
        {
            in_AX[0x2] = param_5;
        }
        if(param_2 != 0x0)
        {
            uVar2   = *in_AX;
            bVar3   = CARRY2(uVar2, param_3);
            param_3 = uVar2 + param_3;
            param_4 = in_AX[0x1] + param_4 + (uint)bVar3;
        }
        *in_AX     = param_3;
        in_AX[0x1] = param_4;
        pass1_1020_bc72((uint *)param_1, param_6, param_7);
    }
    else
    {
        if(param_2 != 0x0)
        {
            bVar3   = CARRY2(*puStack6, param_3);
            param_3 = *puStack6 + param_3;
            param_4 = in_AX[0x1] + param_4 + (uint)bVar3;
        }
        *in_AX     = param_3;
        in_AX[0x1] = param_4;
    }
    return 0x1;
}


void __stdcall16far pass1_1020_bc72(uint *param_1, ushort param_2, ushort param_3)

{
    undefined4 uVar1;
    ushort     uVar2;

    uVar2 = (ushort)((ulong)param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x2);
    pass1_1000_4aea(*param_1, (uint)uVar1, (int)((ulong)uVar1 >> 0x10), 0x6, (uchar *)0xbd6c, (int)&stack0xfffe, param_2, uVar2, 0x1000, param_3);
    return;
}


void __stdcall16far pass1_1020_bc92(uint *param_1, ushort param_2, ushort param_3)

{
    undefined4 uVar1;
    undefined  local_c[0x4];
    ushort     uStack8;

    uStack8 = param_2;
    uVar1   = *(undefined4 *)((int)param_1 + 0x2);
    pass1_1000_49c6((ushort)local_c, param_3, *param_1, (uint)uVar1, (uint)((ulong)uVar1 >> 0x10), 0x6, (uchar *)0xbd6c, (int)&stack0xfffe);
    return;
}

int __cdecl16far pass1_1020_bd6c(ulong param_1, ulong param_2)

{
    return *(int *)((int)param_1 + 0x4) - *(int *)((int)param_2 + 0x4);
}

ushort __cdecl16far pass1_1020_c3ae(void)

{
    return 0x1;
}

astruct_20 *__stdcall16far pass1_1018_cbda(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xf9;
    uVar4  = 0xc5;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x97);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x73b, 0x2e, CONCAT22((int)puVar2, 0x2af8), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd46e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_cc28(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort    uVar1;
    ushort   *puVar2;
    undefined local_6[0x4];
    ushort    uVar3;
    ushort    uVar4;

    uVar3  = 0xfa;
    uVar4  = 0xa3;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x98);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x73c, 0x2f, CONCAT22((int)puVar2, 0x2710), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd816;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_cc76(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xfb;
    uVar4  = 0xa8;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x99);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x73e, 0x73d, 0x30, CONCAT22((int)puVar2, 0x61a8), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xdb22;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_ccc4(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xfc;
    uVar4  = 0xa9;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x9b);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x740, 0x73f, 0x31, CONCAT22((int)puVar2, 0x59d8), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd5a6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_cd12(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xfd;
    uVar4  = 0x7c;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x9c);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x741, 0x32, CONCAT22((int)puVar2, 0x2710), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd94e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_cd60(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xfe;
    uVar4  = 0xc9;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x0);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x0, 0x33, CONCAT22((int)puVar2, 0x2710), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd3d2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}

astruct_20 *__stdcall16far pass1_1018_cf74(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xfe;
    uVar4  = 0xcf;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x80);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x0, 0x34, CONCAT22((int)puVar2, 0x2710), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd77a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}

ushort __cdecl16far switch_1020_c3b4(ushort param_1)

{
    ushort uStack6;

    uStack6 = 0x1;
    switch(param_1)
    {
    case 0x1:
    case 0x2:
    case 0x3:
    case 0x5:
    case 0x8:
    case 0x9:
    case 0xa:
    case 0xb:
    case 0xc:
        uStack6 = 0x3;
        break;
    case 0x4:
        uStack6 = 0x6;
        break;
    case 0x6:
    case 0xf:
    case 0x10:
    case 0x11:
    case 0x12:
    case 0x13:
        uStack6 = 0xa;
        break;
    case 0x7:
        uStack6 = 0x2;
        break;
    case 0xd:
    case 0xe:
        uStack6 = 0x1;
    }
    return uStack6;
}


ushort __cdecl16far pass1_1020_c42e(int param_1)

{
    ushort uVar1;

    if(param_1 == 0xf)
    {
        uVar1 = 0x1;
    }
    else
    {
        uVar1 = 0x3;
    }
    return uVar1;
}

void __stdcall16far pass1_1020_c4a8(ulong param_1, ushort *param_2, ulong *param_3, int param_4, ushort param_5, ushort param_6)

{
    undefined4 uVar1;
    ulong     *puVar2;
    uint       uVar3;
    undefined2 uVar4;

    uVar3 = (uint)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x1c) != 0x0)
    {
        pass1_1020_c6a4(param_1 & 0xffff | (ulong)uVar3 << 0x10, param_5, param_6);
    }
    uVar1    = *(undefined4 *)((int)param_1 + 0x18);
    uVar4    = (undefined2)((ulong)uVar1 >> 0x10);
    puVar2   = (ulong *)((int)uVar1 + param_4 * 0x6);
    *param_3 = *puVar2;
    *param_2 = *(ushort *)(puVar2 + 0x1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_c4f4(ulong param_1, ushort param_2, ushort param_3, ulong param_4, astruct_361 *param_5, uint param_6)

{
    astruct_361 *paVar1;
    ushort       uVar2;
    uint         uVar3;

    pass1_1020_c6de(param_1, param_4);
    uVar3 = param_6 | (uint)param_5;
    if(uVar3 != 0x0)
    {
        paVar1 = param_5;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
        uVar2              = pass1_1030_6fa0(CONCAT22(uVar3, paVar1));
        param_5->field_0x4 = *(undefined2 *)(uVar2 * 0x2 + 0x4ea4);
    }
    return;
}


ulong __stdcall16far pass1_1020_c538(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x12), *(undefined2 *)((int)param_1 + 0x10));
}


void __stdcall16far pass1_1020_c54a(ulong param_1, int param_2, ushort param_3)

{
    uint uVar1;

    uVar1 = (uint)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x1c) != 0x0)
    {
        pass1_1020_c6a4(param_1 & 0xffff | (ulong)uVar1 << 0x10, param_2, param_3);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_c5b8(ushort param_1, uint param_2, int param_3, ushort param_4)

{
    long       *plVar1;
    undefined4  uVar2;
    code      **ppcVar3;
    undefined4 *puVar4;
    uint        uVar5;
    uint        extraout_DX;
    uint        uVar6;
    int         iVar7;
    undefined2  uVar8;

    uVar2 = *(undefined4 *)(param_3 + 0xa);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)((ulong)uVar2 >> 0x10));
    uVar5                       = pass1_1030_6fa0(CONCAT22(param_2, param_1));
    *(ushort *)(param_3 + -0x6) = uVar5;
    pass1_1020_c6de(*(ulong *)(param_3 + 0x6), 0x0);
    *(uint *)(param_3 + -0xa) = uVar5;
    *(uint *)(param_3 + -0x8) = param_2;
    if((param_2 | uVar5) == 0x0)
    {
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_3 + 0x6) + 0x20);
        (**ppcVar3)();
        uVar6 = extraout_DX;
        pass1_1020_c6de(*(ulong *)(param_3 + 0x6), 0x0);
        *(uint *)(param_3 + -0xa) = uVar5;
        *(uint *)(param_3 + -0x8) = uVar6;
        if((uVar6 | uVar5) == 0x0)
        {
            return;
        }
    }
    uVar2                              = *(undefined4 *)(param_3 + 0x6);
    uVar8                              = (undefined2)((ulong)uVar2 >> 0x10);
    iVar7                              = (int)uVar2;
    *(undefined2 *)(iVar7 + 0x1c)      = 0x1;
    plVar1                             = (long *)(iVar7 + 0x8);
    *plVar1                            = *plVar1 + 0x1;
    puVar4                             = (undefined4 *)*(undefined4 *)(param_3 + -0xa);
    *puVar4                            = *(undefined4 *)(param_3 + 0xa);
    *(undefined2 *)((int)puVar4 + 0x4) = *(undefined2 *)(*(int *)(param_3 + -0x6) * 0x2 + 0x4ea4);
    return;
}


void __stdcall16far pass1_1020_c644(ulong *param_1, ushort param_2, ulong param_3)

{
    long      *plVar1;
    undefined2 uVar2;
    code     **ppcVar3;
    int        iVar4;
    int        iVar5;
    undefined2 uVar6;
    ulong     *puStack6;

    uVar6 = (undefined2)((ulong)param_1 >> 0x10);
    iVar5 = (int)param_1;
    if(*(long *)(iVar5 + 0x18) == 0x0)
    {
        ppcVar3 = (code **)((int)*param_1 + 0x20);
        (**ppcVar3)();
    }
    iVar4                    = *(int *)(iVar5 + 0x8) * 0x6 + *(int *)(iVar5 + 0x18);
    uVar2                    = *(undefined2 *)(iVar5 + 0x1a);
    puStack6                 = (ulong *)CONCAT22(uVar2, iVar4);
    plVar1                   = (long *)(iVar5 + 0x8);
    *plVar1                  = *plVar1 + 0x1;
    *puStack6                = param_3;
    *(ushort *)(iVar4 + 0x4) = param_2;
    return;
}


void __stdcall16far pass1_1020_c694(ulong param_1, int param_2, ushort param_3)

{
    pass1_1020_c6a4(param_1, param_2, param_3);
    return;
}


void __stdcall16far pass1_1020_c6a4(ulong param_1, int param_2, ushort param_3)

{
    long         lVar1;
    astruct_359 *iVar2;
    ushort       uVar2;

    uVar2 = (ushort)(param_1 >> 0x10);
    iVar2 = (astruct_359 *)param_1;
    if((iVar2->field_0x18 != 0x0) && (iVar2->field_0x8 != 0x0))
    {
        lVar1 = iVar2->field_0x18;
        pass1_1000_4aea((uint)lVar1, (uint)((ulong)lVar1 >> 0x10), iVar2->field_0x10, 0x6, (uchar *)0xc7fa, (int)&stack0xfffe, param_2, uVar2, 0x1000, param_3);
        iVar2->field_0x1c = 0x0;
    }
    return;
}


void __stdcall16far pass1_1020_c6de(ulong param_1, long param_2)

{
    ulong       *puVar1;
    undefined4   uVar2;
    astruct_360 *iVar3;
    int          unaff_DI;
    uint         uVar3;
    ushort       unaff_SS;
    ulong        uStack6;

    uVar3 = (uint)(param_1 >> 0x10);
    iVar3 = (astruct_360 *)param_1;
    if(iVar3->field_0x1c != 0x0)
    {
        pass1_1020_c6a4(param_1 & 0xffff | (ulong)uVar3 << 0x10, unaff_DI, unaff_SS);
    }
    uStack6 = 0x0;
    while(true)
    {
        puVar1 = &iVar3->field_0x10;
        if(*puVar1 < uStack6 || *puVar1 == uStack6)
        {
            return;
        }
        uVar2 = iVar3->field_0x18;
        if(*(long *)((int)uVar2 + (int)uStack6 * 0x6) == param_2)
            break;
        uStack6 = uStack6 + 0x1;
    }
    return;
}

ushort __cdecl16far pass1_1020_a426(void)

{
    ushort *puVar1;

    pass1_1008_3e38((ushort *)&PTR_LOOP_1048_4230);
    puVar1 = pass1_1008_3e38((ushort *)0x10484236);
    return (ushort)puVar1;
}

void __stdcall16far pass1_1020_b0aa(ushort param_1, ushort param_2, int param_3, ushort param_4)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    int         iVar3;
    undefined4 *puVar4;
    uint        extraout_DX;
    uint        uVar5;
    uint        uVar6;
    undefined2  uVar7;
    ulong       uVar8;
    ulong       uStack20;

    uVar7 = (undefined2)((ulong)_PTR_LOOP_1050_4e74 >> 0x10);
    if(*(int *)((int)_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) == 0x0)
    {
        return;
    }
    if(*(int *)((int)_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) != -0x1)
    {
        if(PTR_LOOP_1050_4e78 == (undefined *)0x0)
        {
            iVar3 = param_3;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x2, 0x400);
            puVar1  = (undefined4 *)*(ulong *)(iVar3 + 0xc);
            ppcVar2 = (code **)((int)*puVar1 + 0x10);
            puVar4  = puVar1;
            (**ppcVar2)();
            uVar6 = extraout_DX;
            for(uStack20 = 0x0; uStack20 < ((ulong)puVar4 & 0xffff | (ulong)extraout_DX << 0x10); uStack20 = uStack20 + 0x1)
            {
                uVar8 = pass1_1030_1d7c((int)((ulong)puVar4 & 0xffff), uVar6, (ulong)puVar1);
                uVar5 = (uint)(uVar8 >> 0x10);
                uVar6 = uVar5 | (uint)uVar8;
                if((uVar6 != 0x0) && ((iVar3 = *(int *)((uint)uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b))))
                {
                    PTR_LOOP_1050_4e78 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
                    break;
                }
            }
            if(PTR_LOOP_1050_4e78 == (undefined *)0x0)
            {
                PTR_LOOP_1050_4e78 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
                return;
            }
        }
        iVar3 = *(int *)((int)_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) + -0x1;
        pass1_1008_612e(0x0, iVar3, iVar3);
    }
    return;
}

ushort __stdcall16far pass1_1020_b1ae(int param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, ushort *param_6, ulong param_7)

{
    undefined4 *puVar1;
    int         local_14;
    int         local_12;
    int         local_10;
    int         local_e;
    undefined4  local_c;
    undefined2  uStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_7, (uint)(param_7 >> 0x10));
    iStack6 = param_1;
    uStack4 = param_2;
    puVar1  = (undefined4 *)pass1_1030_5b5c(param_1, param_2);
    local_c = *puVar1;
    uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
    pass1_1008_3e94(param_6, (ushort *)CONCAT22(param_3, &local_10), (ushort *)CONCAT22(param_3, &local_e));
    pass1_1008_3e94((ushort *)CONCAT22(param_3, &local_c), (ushort *)CONCAT22(param_3, &local_14), (ushort *)CONCAT22(param_3, &local_12));
    if((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 + -0xb)) && (local_10 < local_14 + -0xb))
    {
        return 0x1;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_b240(ushort param_1, ushort param_2, ulong param_3, ulong param_4, ulong param_5)

{
    undefined4 *puVar1;
    uint        uVar2;
    ushort      uVar3;
    uint        uVar4;
    uint        uVar5;
    undefined2  uVar6;
    ulong       uVar7;
    byte        bStack31;
    undefined4  local_a;
    undefined4  uStack6;

    puVar1 = &local_a;
    uVar6  = (undefined2)(param_5 >> 0x10);
    pass1_1030_64ce(param_1, puVar1, param_2, _PTR_LOOP_1050_5740, (ushort *)param_4, *(long *)((int)param_5 + 0x4), (ulong *)CONCAT22(param_1, puVar1));
    uStack6  = *puVar1;
    uVar5    = *(uint *)((int)puVar1 + 0x2);
    bStack31 = (byte)((ulong)uStack6 >> 0x18);
    uVar2    = (uint)bStack31;
    if(bStack31 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack6, uVar5);
        uVar7 = struct_op_1030_73a8(CONCAT22(uVar5, uVar2));
        uVar4 = (uint)(uVar7 >> 0x10);
        uVar2 = (uint)uVar7;
        uVar5 = uVar4 | uVar2;
        if((uVar5 != 0x0) && (uVar2 = *(uint *)(uVar2 + 0xc), 0x9 < (int)uVar2))
        {
            return;
        }
    }
    uVar3 = pass1_1020_b1ae(uVar2, uVar5, param_1, (ushort)param_3, (ushort)(param_3 >> 0x10), (ushort *)param_4, *(ulong *)((int)param_5 + 0x4));
    if(uVar3 == 0x0)
    {
        return;
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1020_b2da(ushort param_1, ushort param_2, ushort param_3, int param_4, ushort *param_5, ulong param_6)

{
    int         iVar1;
    undefined2  uVar2;
    ushort      uVar3;
    undefined  *puVar4;
    ushort      uVar5;
    uchar       in_AF;
    ushort     *puVar6;
    undefined **ppuVar7;
    int         iStack28;
    undefined   local_1a[0x6];
    undefined2  uStack20;
    undefined2  uStack18;
    int        *piStack16;
    int        *piStack12;
    ushort      local_8;
    undefined4  local_6;

    if(param_4 == 0x0)
    {
        uVar2 = 0x4e6a;
    }
    else
    {
        uVar2 = 0x4e6e;
    }
    piStack12 = (int *)CONCAT22(0x1050, uVar2);
    if(param_4 == 0x0)
    {
        uStack20 = 0x4e68;
    }
    else
    {
        uStack20 = 0x4e6c;
    }
    uStack18  = SUB42(&USHORT_1050_1050, 0x0);
    piStack16 = (int *)CONCAT22(0x1050, uStack20);
    do
    {
        if(param_4 == 0x0)
        {
            ppuVar7 = &PTR_LOOP_1048_4230;
        }
        else
        {
            ppuVar7 = (undefined **)0x10484236;
        }
        pass1_1008_3eb4((ushort *)ppuVar7, (ushort *)CONCAT22(param_1, &local_8), (ushort *)CONCAT22(param_1, &local_6), (ushort *)CONCAT22(param_1, (int)&local_6 + 0x2));
        iVar1 = *piStack12;
        if(iVar1 == 0x0)
        {
            local_6 = CONCAT22(local_6._2_2_ + *piStack16, (int)local_6 + -0x1);
        }
        else
        {
            if(iVar1 == 0x1)
            {
                local_6 = CONCAT22(local_6._2_2_ + -0x1, (int)local_6 + *piStack16);
            }
            else
            {
                if(iVar1 == 0x2)
                {
                    local_6 = CONCAT22(local_6._2_2_ - *piStack16, (int)local_6 + -0x1);
                }
            }
        }
        puVar6 = pass1_1008_3e54((ushort *)CONCAT22(param_1, local_1a), local_8, (ushort)local_6, (ushort)(local_6 >> 0x10));
        uVar5  = (ushort)((ulong)puVar6 >> 0x10);
        uVar2  = (undefined2)(param_6 >> 0x10);
        uVar3  = pass1_1020_b1ae((int)local_1a, uVar5, param_1, param_2, param_3, (ushort *)CONCAT22(param_1, local_1a), *(ulong *)((int)param_6 + 0x4));
        if(uVar3 != 0x0)
        {
            puVar4 = local_1a;
            pass1_1020_b240(param_1, uVar5, CONCAT22(param_3, param_2), CONCAT22(param_1, puVar4), param_6);
            if(puVar4 != (undefined *)0x0)
            {
            LAB_1020_b46e:
                pass1_1008_3e76(param_5, local_8, (ushort)local_6, (ushort)(local_6 >> 0x10));
                return;
            }
        }
        iVar1 = *piStack12;
        if(iVar1 == 0x0)
        {
        LAB_1020_b45e:
            local_6 = local_6 & 0xffff0000 | (ulong)((int)local_6 + 0x2);
        }
        else
        {
            if(iVar1 == 0x1)
            {
                local_6 = local_6 & 0xffff | (ulong)(local_6._2_2_ + 0x2) << 0x10;
            }
            else
            {
                if(iVar1 == 0x2)
                    goto LAB_1020_b45e;
            }
        }
        pass1_1008_3e76((ushort *)CONCAT22(param_1, local_1a), local_8, (ushort)local_6, (ushort)(local_6 >> 0x10));
        uVar3 = pass1_1020_b1ae((int)local_1a, uVar5, param_1, param_2, param_3, (ushort *)CONCAT22(param_1, local_1a), *(ulong *)((int)param_6 + 0x4));
        if(uVar3 != 0x0)
        {
            puVar4 = local_1a;
            pass1_1020_b240(param_1, uVar5, CONCAT22(param_3, param_2), CONCAT22(param_1, puVar4), param_6);
            if(puVar4 != (undefined *)0x0)
                goto LAB_1020_b46e;
        }
        iStack28 = *piStack12 + 0x1;
        if(0x2 < iStack28)
        {
            iStack28   = 0x0;
            *piStack16 = *piStack16 + 0x1;
        }
        *piStack12 = iStack28;
        pass1_1020_ac6e(param_1, in_AF, CONCAT22(param_3, param_2), param_4, *piStack16, iStack28);
    } while(true);
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_b482(ushort param_1, ulong param_2, undefined4 *param_3, ulong param_4)

{
    undefined  *puVar1;
    undefined4 *puVar2;
    undefined4  uVar3;
    uint        uVar4;
    uint        uVar5;
    undefined4 *puVar6;
    ulong       uVar7;
    ushort      uVar8;
    ushort      uVar9;
    undefined4 *puVar10;
    int         iStack46;
    undefined4  local_2a;
    ushort      local_26;
    undefined4  local_24;
    undefined2  uStack32;
    long        lStack30;
    undefined4  uStack26;
    undefined   local_16[0x12];
    undefined   local_4[0x2];

    uVar7 = pass1_1030_bcae((ushort)local_4, param_1);
    uVar4 = (uint)(uVar7 >> 0x10);
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_1, local_16), 0x1, 0x0, 0x400);
    while(true)
    {
        puVar1 = local_16;
        pass1_1028_e4ec(CONCAT22(param_1, puVar1));
        uStack26 = CONCAT22(uVar4, puVar1);
        uVar5    = uVar4 | (uint)puVar1;
        if(uVar5 == 0x0)
        {
            pass1_1020_b240(param_1, 0x0, param_2, (ulong)param_3, param_4);
            if(puVar1 != (undefined *)0x0)
            {
                lStack30 = *(long *)((int)param_4 + 0x4);
                local_24 = *param_3;
                uStack32 = *(undefined2 *)((int)param_3 + 0x4);
                puVar6   = &local_2a;
                pass1_1008_3eb4((ushort *)CONCAT22(param_1, &local_24), (ushort *)CONCAT22(param_1, puVar6), (ushort *)CONCAT22(param_1, (int)&local_2a + 0x2), (ushort *)CONCAT22(param_1, &local_26));
                pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ - 0x1, local_26 - 0x1);
                puVar2 = &local_24;
                uVar8  = (ushort)param_2;
                uVar9  = (ushort)(param_2 >> 0x10);
                pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                if(puVar2 != (undefined4 *)0x0)
                {
                    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, (ushort)((ulong)local_2a >> 0x10), local_26 - 0x1);
                    puVar2 = &local_24;
                    pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                    if(puVar2 != (undefined4 *)0x0)
                    {
                        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ + 0x1, local_26 - 0x1);
                        puVar2 = &local_24;
                        pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                        if(puVar2 != (undefined4 *)0x0)
                        {
                            pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ - 0x1, local_26);
                            puVar2 = &local_24;
                            pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                            if(puVar2 != (undefined4 *)0x0)
                            {
                                pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ + 0x1, local_26);
                                puVar2 = &local_24;
                                pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                if(puVar2 != (undefined4 *)0x0)
                                {
                                    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ + 0x1, local_26 + 0x1);
                                    puVar2 = &local_24;
                                    pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                    if(puVar2 != (undefined4 *)0x0)
                                    {
                                        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, (ushort)((ulong)local_2a >> 0x10), local_26 + 0x1);
                                        puVar2 = &local_24;
                                        pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                        if(puVar2 != (undefined4 *)0x0)
                                        {
                                            pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ - 0x1, local_26 + 0x1);
                                            puVar2 = &local_24;
                                            pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                            if(puVar2 != (undefined4 *)0x0)
                                            {
                                                pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ - 0x2, local_26 - 0x2);
                                                puVar2 = &local_24;
                                                pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                                if(puVar2 != (undefined4 *)0x0)
                                                {
                                                    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ + 0x2, local_26 - 0x2);
                                                    puVar2 = &local_24;
                                                    pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                                    if(puVar2 != (undefined4 *)0x0)
                                                    {
                                                        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ - 0x2, local_26 + 0x2);
                                                        puVar2 = &local_24;
                                                        pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                                        if(puVar2 != (undefined4 *)0x0)
                                                        {
                                                            pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ + 0x2, local_26 + 0x2);
                                                            puVar2 = &local_24;
                                                            pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                                            if(puVar2 != (undefined4 *)0x0)
                                                            {
                                                                pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ - 0x1, local_26 + 0x2);
                                                                puVar2 = &local_24;
                                                                pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                                                if(puVar2 != (undefined4 *)0x0)
                                                                {
                                                                    pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), (ushort)local_2a, local_2a._2_2_ - 0x1, local_26 + 0x3);
                                                                    puVar2 = &local_24;
                                                                    pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                                                    if(puVar2 != (undefined4 *)0x0)
                                                                    {
                                                                        iStack46 = 0x3;
                                                                        while(true)
                                                                        {
                                                                            if(0x9 < iStack46)
                                                                            {
                                                                                return;
                                                                            }
                                                                            pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_24), 0x0, local_2a._2_2_ - iStack46, local_26);
                                                                            puVar2 = &local_24;
                                                                            pass1_1020_afc4(param_1, (ushort)puVar6, uVar8, uVar9, (ushort *)CONCAT22(param_1, puVar2), lStack30);
                                                                            if(puVar2 == (undefined4 *)0x0)
                                                                                break;
                                                                            iStack46 = iStack46 + 0x1;
                                                                        }
                                                                        return;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return;
        }
        uVar3   = *(undefined4 *)(puVar1 + 0x10);
        puVar10 = param_3;
        uVar7   = param_4;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar3, (uint)((ulong)uVar3 >> 0x10));
        puVar1 = local_4;
        pass1_1030_bcbc(param_1, (ushort)puVar1, CONCAT22((int)uVar3, param_1), CONCAT22((int)puVar10, uVar5), (ushort)((ulong)puVar10 >> 0x10), uVar7);
        if((int)puVar1 < 0x0)
            break;
        uVar4 = uVar5;
        if((int)puVar1 < 0x65)
        {
            return;
        }
    }
    return;
}

void __stdcall16far pass1_1020_86d8(ulong param_1)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 uVar4;
    int        iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar4  = (undefined2)(param_1 >> 0x10);
        piVar1 = (int *)((int)param_1 + 0x6);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        uVar2 = *(undefined4 *)((int)param_1 + 0xc);
        uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar3 = (int)uVar2;
        if(*(long *)(iVar3 + iStack4 * 0x4) != 0x0)
        {
            pass1_1008_5236(*(ulong *)(iVar3 + iStack4 * 0x4));
        }
        iStack4 = iStack4 + 0x1;
    }
    return;
}


void __stdcall16far pass1_1020_8712(ulong param_1, int *param_2, astruct_76 *param_3, ushort *param_4)

{
    undefined2 uVar1;
    ulong      uVar2;

    pass1_1008_3f32((int *)param_4, (int *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)));
    uVar2 = pass1_1008_4772(param_3);
    uVar1 = (undefined2)(uVar2 >> 0x10);
    pass1_1008_3e94(param_4, (ushort *)((ulong)param_2 & 0xffff0000 | ZEXT24((int *)((int)param_2 + 0x2))), (ushort *)((ulong)param_2 & 0xffff | (ulong)param_2._2_2_ << 0x10));
    *(int *)((int)param_2 + 0x4) = *(int *)((int)uVar2 + 0x4) + *param_2;
    *(int *)((int)param_2 + 0x6) = *(int *)((int)uVar2 + 0x8) + *(int *)((int)param_2 + 0x2);
    return;
}

void __stdcall16far pass1_1020_8bae(ushort *param_1)

{
    *param_1                            = 0x8e92;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    pass1_1020_8556(param_1);
    return;
}

void __stdcall16far pass1_1020_8f74(ushort *param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_593 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_593 *)param_1;
    *param_1         = 0x9204;
    iVar4->field_0x2 = 0x1020;
    puVar1           = iVar4->field_0xb4;
    uVar2            = iVar4->field_0xb6;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1020_8556(param_1);
    return;
}

void __stdcall16far pass1_1020_9068(ulong *param_1, uchar *param_2, int param_3, ushort param_4)

{
    int        iVar1;
    ulong      uVar2;
    code     **ppcVar3;
    undefined4 uVar4;
    uint       uVar5;
    ulong      uVar6;
    uint       extraout_DX;
    uint       uVar7;
    int        iVar8;
    int        iVar9;
    uint       uVar10;
    undefined2 uVar11;
    int        iStack10;

    uVar10 = (uint)((ulong)param_1 >> 0x10);
    iVar8  = (int)param_1;
    uVar4  = *(undefined4 *)(iVar8 + 0x16);
    uVar2  = *(ulong *)((int)uVar4 + 0xa);
    uVar6  = uVar2;
    pass1_1018_280c(*(ulong *)(iVar8 + 0x16));
    *(undefined2 *)(iVar8 + 0xaa) = (int)uVar6;
    *(uchar **)(iVar8 + 0xac)     = param_2;
    uVar5                         = (uint)param_2 | *(uint *)(iVar8 + 0xaa);
    if(uVar5 == 0x0)
    {
        pass1_1018_2862(*(ulong *)(iVar8 + 0x16));
        *(uint *)(iVar8 + 0xaa)   = uVar5;
        *(uchar **)(iVar8 + 0xac) = param_2;
    }
    if((*(uint *)(iVar8 + 0xac) | *(uint *)(iVar8 + 0xaa)) != 0x0)
    {
        pass1_1020_915a((ulong)param_1 & 0xffff | (ulong)uVar10 << 0x10, param_2, param_3, param_4);
        pass1_1008_4480(uVar2, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar8 + 0xae)), *(astruct_76 **)(iVar8 + 0xb4), param_4);
        ppcVar3 = (code **)((int)*param_1 + 0x10);
        (**ppcVar3)();
        uVar4 = *(undefined4 *)(iVar8 + 0xaa);
        iVar1 = *(int *)((int)uVar4 + 0xa);
        for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
        {
            uVar6 = SEXT24(iStack10);
            empty_1008_8fc4(*(undefined4 *)(iVar8 + 0xaa), uVar6);
            uVar5 = (uint)uVar6;
            uVar7 = extraout_DX | uVar5;
            if(uVar7 != 0x0)
            {
                pass1_1008_8c4e(uVar6 & 0xffff | (ulong)extraout_DX << 0x10, uVar2, param_4);
                uVar4                                   = *(undefined4 *)(iVar8 + 0xc);
                uVar11                                  = (undefined2)((ulong)uVar4 >> 0x10);
                iVar9                                   = (int)uVar4;
                *(uint *)(iVar9 + iStack10 * 0x4)       = uVar5;
                *(uint *)(iVar9 + iStack10 * 0x4 + 0x2) = uVar7;
            }
        }
    }
    return;
}

void __stdcall16far pass1_1020_75c4(ushort *param_1)

{
    astruct_586 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_586 *)param_1;
    *param_1          = 0x7780;
    iVar1->field_0x2  = 0x1020;
    iVar1->field_0xe2 = 0x781c;
    iVar1->field_0xe4 = 0x1020;
    pass1_1020_808e(param_1);
    return;
}

ushort __stdcall16far pass1_1020_79ae(void)

{
    return 0x1;
}

void __stdcall16far pass1_1020_808e(ushort *param_1)

{
    undefined2  *puVar1;
    undefined2   uVar2;
    astruct_574 *iVar3;
    undefined2   uVar3;
    undefined2  *puStack6;

    uVar3             = (undefined2)((ulong)param_1 >> 0x10);
    iVar3             = (astruct_574 *)param_1;
    *param_1          = 0x82bc;
    iVar3->field_0x2  = 0x1020;
    iVar3->field_0xe2 = 0x8358;
    iVar3->field_0xe4 = 0x1020;
    if(param_1 == (ushort *)0x0)
    {
        puVar1 = (undefined2 *)0x0;
        uVar2  = 0x0;
    }
    else
    {
        puVar1 = &iVar3->field_0xe2;
        uVar2  = uVar3;
    }
    puStack6    = (undefined2 *)CONCAT22(uVar2, puVar1);
    *puStack6   = 0x389a;
    puVar1[0x1] = 0x1008;
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0xd2));
    *param_1         = 0x380a;
    iVar3->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    iVar3->field_0x2 = 0x1008;
    return;
}


void __stdcall16far pass1_1020_8106(ulong param_1)

{
    code **ppcVar1;

    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x60);
    (**ppcVar1)();
    return;
}

void __stdcall16far pass1_1020_83f8(ulong param_1, ushort param_2)

{
    undefined4 uVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 uVar4;

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(int *)(iVar3 + 0x4) != 0x0)
    {
        uVar1 = *(undefined4 *)(iVar3 + 0x1c);
        uVar2 = *(undefined4 *)(iVar3 + 0x1c);
        pass1_1008_4480(*(ulong *)((int)uVar1 + 0xa), (ushort *)(param_1 & 0xffff0000 | (ulong)(iVar3 + 0x16)), *(astruct_76 **)((int)uVar2 + 0x2a), param_2);
    }
    return;
}

ulong __stdcall16far pass1_1020_6498(ulong param_1, int param_2)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x18 + param_2 * 0x4) != 0x0)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x18 + param_2 * 0x4);
        uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
        iVar2 = (int)uVar1;
        return CONCAT22(*(undefined2 *)(iVar2 + 0xa), *(undefined2 *)(iVar2 + 0x8));
    }
    return 0x0;
}


ushort __stdcall16far pass1_1020_64d4(ulong param_1, int param_2)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x18 + param_2 * 0x4) != 0x0)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x18 + param_2 * 0x4);
        return *(ushort *)((int)uVar1 + 0x4);
    }
    return 0x0;
}
void __stdcall16far pass1_1020_6746(ulong param_1, int param_2, int param_3)

{
    code     **ppcVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 uVar4;

    if(param_3 != 0x0)
    {
        uVar4 = (undefined2)(param_1 >> 0x10);
        iVar3 = (int)param_1;
        if(*(long *)(iVar3 + 0x18 + param_3 * 0x4) != 0x0)
        {
            uVar2                         = *(undefined4 *)(iVar3 + 0x18 + param_3 * 0x4);
            *(int *)((int)uVar2 + 0x4)    = param_2;
            *(undefined2 *)(iVar3 + 0x10) = 0x1;
            if(param_2 == 0x0)
            {
                ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x18 + param_3 * 0x4) + 0x14);
                (**ppcVar1)();
            }
        }
    }
    return;
}

ushort __stdcall16far pass1_1020_5d56(ulong *param_1, ulong param_2, uchar *param_3, int param_4, ushort param_5)

{
    code **ppcVar1;
    ushort uVar2;
    ushort uVar3;
    int    local_12[0x2];
    int    local_e;
    int    local_c;
    int    local_a[0x2];
    int    iStack6;

    iStack6 = *(int *)((int)param_2 + 0x2e);
    uVar2   = (ushort)param_1;
    uVar3   = (ushort)((ulong)param_1 >> 0x10);
    if(iStack6 == 0x47)
    {
        pass1_1020_61c4(uVar2, uVar3, CONCAT22(param_5, &local_c), (ushort *)CONCAT22(param_5, local_a), param_3, param_4, param_5);
        if(local_a[0] == 0x0)
            goto LAB_1020_5d8b;
        if(local_c <= local_a[0])
        {
            return 0x1;
        }
    }
    else
    {
        if(iStack6 != 0x6a)
        {
            return 0x0;
        }
        pass1_1020_61c4(uVar2, uVar3, CONCAT22(param_5, &local_e), (ushort *)CONCAT22(param_5, local_12), param_3, param_4, param_5);
        if(local_e <= local_12[0])
        {
        LAB_1020_5d8b:
            ppcVar1 = (code **)((int)*param_1 + 0x40);
            (**ppcVar1)();
            return 0x1;
        }
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar2 + 0x8), 0x9, (ushort)param_3, uVar2, (ushort)&PTR_LOOP_1050_1038, param_5);
    return 0x1;
}

ushort *__stdcall16far pass1_1020_4092(ushort *param_1)

{
    int        iVar1;
    undefined2 uVar2;

    pass1_1008_3e38(param_1);
    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *(undefined2 *)(iVar1 + 0x6) = 0x0;
    *(undefined2 *)(iVar1 + 0x8) = 0x0;
    *(undefined2 *)(iVar1 + 0xa) = 0x1;
    *(undefined2 *)(iVar1 + 0xc) = 0x0;
    *(undefined2 *)(iVar1 + 0xe) = 0x0;
    pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x10)));
    return param_1;
}

void __stdcall16far pass1_1020_44b0(ulong *param_1)

{
    code     **ppcVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(long *)(iVar2 + 0xf6) != 0x0)
    {
        ppcVar1 = (code **)((int)*param_1 + 0x98);
        (**ppcVar1)();
        *(undefined2 *)(iVar2 + 0x112) = 0x0;
        ppcVar1                        = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xf6) + 0x8);
        (**ppcVar1)();
    }
    return;
}

void __stdcall16far pass1_1020_3c32(int param_1, ushort param_2, uint param_3, ushort param_4)

{
    char cVar1;
    int  iVar2;

    if(param_3 == 0xf5)
    {
        iVar2 = 0x1;
    LAB_1020_3c52:
        pass1_1018_1b02(param_4, *(ulong *)(param_1 + 0xfa), iVar2);
        return;
    }
    if((param_3 < 0xf6) && (cVar1 = (char)param_3, cVar1 != '\0'))
    {
        if(cVar1 == '\x01' || cVar1 == '\x02')
        {
            return;
        }
        if(cVar1 == -0xc)
        {
            iVar2 = 0x0;
            goto LAB_1020_3c52;
        }
    }
    pass1_1020_3c32(param_1, param_2, param_3, param_4);
    return;
}


void __stdcall16far pass1_1020_3c74(ushort param_1, ulong param_2, ushort param_3, ushort param_4)

{
    pass1_1020_3c8c(CONCAT22((int)param_2, param_1), CONCAT22(param_3, (int)(param_2 >> 0x10)), param_4);
    return;
}

void __stdcall16far pass1_1020_1b68(ulong param_1, ushort param_2)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x92);
    uVar2  = *(uint *)(iVar4 + 0x94);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *(undefined4 *)(iVar4 + 0x92) = 0x0;
    pass1_1010_4f48(*(ulong *)(iVar4 + 0x8e), param_2);
    *(undefined4 *)(iVar4 + 0x8e) = 0x0;
    return;
}


ushort __stdcall16far pass1_1020_1bb6(ulong param_1)

{
    code **ppcVar1;

    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x92) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

ushort __stdcall16far pass1_1020_1da8(ulong param_1, int param_2, uint param_3, ushort param_4)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    uVar1 = *(undefined4 *)(iVar2 + 0x8e);
    if(*(int *)((int)uVar1 + 0x30) == 0x1)
    {
        return 0x1;
    }
    uVar1 = *(undefined4 *)(iVar2 + 0x8e);
    if((*(int *)((int)uVar1 + 0x30) < 0x3) && (pass1_1010_4df0(*(ulong *)(iVar2 + 0x8e), param_3, param_4), param_2 == 0x0))
    {
        return 0x1;
    }
    return 0x0;
}

void __stdcall16far pass1_1020_1f74(ushort *param_1, ushort param_2)

{
    astruct_582 *iVar1;
    uint         uVar1;

    uVar1            = (uint)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_582 *)param_1;
    *param_1         = 0x2518;
    iVar1->field_0x2 = 0x1020;
    pass1_1010_1ea6(iVar1->field_0x6, (ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10, param_2);
    *param_1         = 0x3ab0;
    iVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    return;
}

void __stdcall16far pass1_1020_2286(ushort param_1, ushort param_2, int *param_3, int param_4)

{
    *param_3 = 0x64 - param_4 >> 0x1;
    return;
}

void __stdcall16far pass1_1020_2594(ushort *param_1)

{
    astruct_583 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_583 *)param_1;
    *param_1          = 0x270c;
    iVar1->field_0x2  = 0x1020;
    iVar1->field_0xe2 = 0x27a8;
    iVar1->field_0xe4 = 0x1020;
    pass1_1020_808e(param_1);
    return;
}

void __stdcall16far pass1_1020_2936(void)

{
    pass1_1020_79ae();
    return;
}

void __stdcall16far pass1_1020_2a46(ushort param_1, ushort param_2, int param_3)

{
    pass1_1018_0ae8(*(ulong *)(param_1 + 0xf2), 0x1);
    pass1_1008_68c6(param_1, param_2, param_3, 0x1008);
    return;
}

void __stdcall16far pass1_1020_2a94(ulong param_1, ulong param_2, ushort param_3)

{
    pass1_1018_1662(*(ulong *)((int)param_1 + 0xf2), (int)param_2, (int)(param_2 >> 0x10), param_3);
    return;
}


void __stdcall16far pass1_1018_e64c(ushort *param_1)

{
    astruct_576 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_576 *)param_1;
    *param_1          = 0xe790;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xe2 = 0xe82c;
    iVar1->field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint __stdcall16far pass1_1018_e678(ulong param_1, ushort param_2, ushort param_3)

{
    code     **ppcVar1;
    uint       uVar2;
    ushort     uVar3;
    undefined2 uVar4;
    undefined4 uVar5;

    uVar4 = (undefined2)(param_1 >> 0x10);
    uVar3 = (ushort)param_1;
    uVar2 = *(uint *)(uVar3 + 0xf0) | *(uint *)(uVar3 + 0xee);
    if(uVar2 != 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar3 + 0xee) + 0x8);
        uVar5   = (**ppcVar1)();
        param_2 = (ushort)((ulong)uVar5 >> 0x10);
        uVar2   = (uint)uVar5;
    }
    if(*(int *)(uVar3 + 0xea) == 0x0)
    {
        *(undefined2 *)(uVar3 + 0xea) = 0x1;
        uVar5                         = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar3 + 0x8), 0x15, param_2, uVar3, (ushort)&PTR_LOOP_1050_1038, param_3);
        uVar2                         = (uint)uVar5;
    }
    return uVar2;
}

void __stdcall16far pass1_1018_e9de(ushort *param_1)

{
    astruct_578 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_578 *)param_1;
    *param_1          = 0xebd0;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xe2 = 0xec6c;
    iVar1->field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

void __stdcall16far pass1_1020_022c(ushort *param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_580 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_580 *)param_1;
    *param_1         = 0x45a;
    iVar4->field_0x2 = 0x1020;
    puVar1           = iVar4->field_0xe6;
    uVar2            = iVar4->field_0xe8;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0xd2));
    *param_1         = 0x380a;
    iVar4->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    iVar4->field_0x2 = 0x1008;
    return;
}

void __stdcall16far pass1_1020_028c(ushort param_1, ushort param_2, int param_3)

{
    pass1_1010_3c9e(*(long *)(param_1 + 0xe2));
    pass1_1008_68c6(param_1, param_2, param_3, 0x1008);
    return;
}

void __stdcall16far pass1_1020_05d6(ushort *param_1, ushort param_2)

{
    astruct_581 *iVar1;
    uint         uVar1;

    uVar1            = (uint)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_581 *)param_1;
    *param_1         = 0x75a;
    iVar1->field_0x2 = 0x1020;
    if(iVar1->field_0x6 != 0x0)
    {
        pass1_1010_1ea6(iVar1->field_0x6, (ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10, param_2);
    }
    *param_1         = 0x3ab0;
    iVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    return;
}

void __stdcall16far struct_1020_0762(astruct_20 *param_1, ulong param_2, ulong *param_3, ushort param_4, ulong param_5, ulong param_6, ushort param_7)

{
    astruct_20 *iVar1;
    astruct_20 *uVar1;
    astruct_20 *paVar1;
    ushort      uVar2;

    paVar1 = (astruct_20 *)param_1;
    uVar2  = (ushort)((ulong)param_1 >> 0x10);
    pass1_1020_01d8(paVar1, uVar2, (ushort)param_2, (ushort)(param_2 >> 0x10), param_4, (ushort)param_5, (ushort)(param_5 >> 0x10), param_6, param_7);
    paVar1[0x1].field_0xe  = 0x0;
    paVar1[0x1].field_0x10 = *param_3;
    param_1->field_0x0     = 0x81a;
    paVar1->field_0x2      = 0x1020;
    return;
}

ushort *__stdcall16far pass1_1018_dcf6(ushort *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    *param_1                            = 0xdf3c;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}

void __stdcall16far pass1_1018_dd7c(ushort param_1, ushort param_2, ulong param_3, ulong param_4, ushort param_5, ushort param_6)

{
    uint        uVar1;
    undefined4  uVar2;
    code      **ppcVar3;
    uint        uVar4;
    uint        uVar5;
    uint        uVar6;
    ushort      uVar7;
    uchar      *puVar8;
    uint        uVar9;
    uchar      *extraout_DX;
    uchar      *puVar10;
    int         unaff_DI;
    ushort     *puVar11;
    undefined4 *puVar12;
    int         iVar13;
    ushort      uVar14;
    long        lStack32;
    ushort      uStack20;
    uint        uStack12;

    uVar5  = (uint)param_4._3_1_;
    iVar13 = (int)(param_3 >> 0x10);
    if(param_4._3_1_ == 0x0)
    {
        puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, (uchar *)param_5, unaff_DI);
        puVar8  = (uchar *)((ulong)puVar11 >> 0x10);
        if(*(int *)((int)puVar11 + 0x1e) == 0x0)
        {
            uStack20 = (uint)param_4;
            uVar14   = (uint)param_4;
        }
        else
        {
            if((uint)param_4 - 0x7 == 0x0)
            {
                uStack20      = 0x6;
                param_4._0_2_ = (uint)param_4 - 0x7;
            }
            else
            {
                if((uint)param_4 - 0x8 == 0x0)
                {
                    uStack20      = 0x7;
                    param_4._0_2_ = (uint)param_4 - 0x8;
                }
                else
                {
                    uStack20      = 0x8;
                    param_4._0_2_ = (uint)param_4 - 0x9;
                }
            }
            uVar14 = 0x6;
        }
        pass1_1010_81f6(0x1010, param_6, _PTR_LOOP_1050_14cc, 0x0, uVar14);
        uVar5 = (uint)puVar8 | (uint)param_4;
        if((uVar5 != 0x0) && (puVar10 = puVar8, mem_op_1000_179c(0x46, puVar8, 0x1000), ((uint)puVar10 | uVar5) != 0x0))
        {
            pass1_1008_87cc((astruct_86 *)CONCAT22(puVar10, uVar5), (int)param_3, iVar13, uStack20, CONCAT13((char)((uint)puVar8 >> 0x8), CONCAT12((char)puVar8, (uint)param_4)), param_4, param_6);
        }
    }
    else
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (uint)param_4, (uint)(param_4 >> 0x10));
        puVar12 = (undefined4 *)struct_op_1030_73a8(CONCAT22(param_5, uVar5));
        uVar9   = (uint)((ulong)puVar12 >> 0x10);
        uVar4   = (uint)puVar12;
        if((uVar9 | uVar4) != 0x0)
        {
            uVar2    = *(undefined4 *)(uVar5 + 0x2e);
            uStack12 = (uint)uVar2;
            if((*(uint *)(uVar5 + 0x30) | uStack12) == 0x0)
            {
                lStack32 = 0x0;
            }
            else
            {
                lStack32 = *(long *)(uStack12 + 0x200);
            }
            uVar5 = *(uint *)(uVar4 + 0x1c);
            uVar1 = *(uint *)(uVar4 + 0x1e);
            uVar6 = uVar1 | uVar5;
            if(uVar6 != 0x0)
            {
                lStack32 = CONCAT22(uVar1, uVar5);
                uVar6    = uVar5;
            }
            ppcVar3 = (code **)((int)*puVar12 + 0x14);
            (**ppcVar3)(0x1030, uVar4, uVar9);
            puVar8 = extraout_DX;
            uVar7  = uVar6;
            pass1_1010_81f6(0x1010, param_6, _PTR_LOOP_1050_14cc, lStack32, uVar6);
            puVar10 = puVar8;
            uVar14  = uVar7;
            mem_op_1000_179c(0x46, puVar8, 0x1000);
            uVar5 = (uint)puVar10 | uVar14;
            if(uVar5 == 0x0)
            {
                uVar14 = 0x0;
                uVar5  = 0x0;
            }
            else
            {
                pass1_1008_87cc((astruct_86 *)CONCAT22(puVar10, uVar14), (int)param_3, iVar13, uVar6, CONCAT22(puVar8, uVar7), param_4, param_6);
            }
            pass1_1008_8bc6(param_6, uVar5, CONCAT13((char)(uVar5 >> 0x8), CONCAT12((char)uVar5, uVar14)));
        }
    }
    return;
}

void __stdcall16far pass1_1018_e2a0(ushort *param_1)

{
    astruct_573 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_573 *)param_1;
    *param_1          = 0xe44e;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xe2 = 0xe4ea;
    iVar1->field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

astruct_20 *__stdcall16far pass1_1018_c958(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xf1;
    uVar4  = 0x9a;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x8d);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x732, 0x26, CONCAT22((int)puVar2, 0x1f40), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xdc5a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_c9a6(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xf2;
    uVar4  = 0xa0;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x8e);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x733, 0x27, CONCAT22((int)puVar2, 0x1b58), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd6de;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_c9f4(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    int       *piVar1;
    ushort     uVar2;
    undefined2 uVar3;
    ushort    *puVar4;
    undefined2 uVar5;
    undefined  local_6[0x4];

    uVar3  = 0xf3;
    uVar5  = 0xa6;
    puVar4 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x8f);
    uVar2  = (ushort)((ulong)puVar4 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x734, 0x28, CONCAT22((int)puVar4, 0x32c8), CONCAT22(uVar3, uVar2), CONCAT22(param_2, uVar5), param_3, param_4, uVar2);
    uVar3                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xda86;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    piVar1                              = (int *)((int)param_1 + 0x10e);
    *piVar1                             = *piVar1 + -0x19;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_ca48(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xf4;
    uVar4  = 0xa1;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x90);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x735, 0x29, CONCAT22((int)puVar2, 0x36b0), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd50a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_ca96(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    int       *piVar1;
    ushort     uVar2;
    undefined2 uVar3;
    ushort    *puVar4;
    undefined2 uVar5;
    undefined  local_6[0x4];

    uVar3  = 0xf5;
    uVar5  = 0xbf;
    puVar4 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x92);
    uVar2  = (ushort)((ulong)puVar4 >> 0x10);
    pass1_1018_c402(param_1, 0x737, 0x736, 0x2a, CONCAT22((int)puVar4, 0x6590), CONCAT22(uVar3, uVar2), CONCAT22(param_2, uVar5), param_3, param_4, uVar2);
    uVar3                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xd8b2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    piVar1                              = (int *)((int)param_1 + 0x10e);
    *piVar1                             = *piVar1 + 0x64;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_caea(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xf6;
    uVar4  = 0x93;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x93);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x738, 0x2b, CONCAT22((int)puVar2, 0x2328), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xdbbe;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_cb38(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    ushort     uVar1;
    ushort    *puVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined  local_6[0x4];

    uVar3  = 0xf7;
    uVar4  = 0x94;
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x94);
    uVar1  = (ushort)((ulong)puVar2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x739, 0x2c, CONCAT22((int)puVar2, 0x32c8), CONCAT22(uVar3, uVar1), CONCAT22(param_2, uVar4), param_3, param_4, uVar1);
    param_1->field_0x0                  = 0xd642;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_cb86(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    int       *piVar1;
    ushort     uVar2;
    undefined2 uVar3;
    ushort    *puVar4;
    undefined2 uVar5;
    undefined  local_6[0x4];

    uVar3  = 0xf8;
    uVar5  = 0xc2;
    puVar4 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0x96);
    uVar2  = (ushort)((ulong)puVar4 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x73a, 0x2d, CONCAT22((int)puVar4, 0x2328), CONCAT22(uVar3, uVar2), CONCAT22(param_2, uVar5), param_3, param_4, uVar2);
    uVar3                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xd9ea;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    piVar1                              = (int *)((int)param_1 + 0x10e);
    *piVar1                             = *piVar1 + 0x64;
    return param_1;
}

void __stdcall16far pass1_1018_642e(ushort param_1, ushort param_2, int *param_3, int param_4)

{
    *param_3 = 0x64 - param_4 >> 0x1;
    return;
}

uint __stdcall16far pass1_1018_6768(ulong param_1, ushort param_2, ushort param_3)

{
    code     **ppcVar1;
    uint       uVar2;
    ushort     uVar3;
    undefined2 uVar4;
    undefined4 uVar5;

    uVar4 = (undefined2)(param_1 >> 0x10);
    uVar3 = (ushort)param_1;
    uVar2 = *(uint *)(uVar3 + 0xf0) | *(uint *)(uVar3 + 0xee);
    if(uVar2 != 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar3 + 0xee) + 0x8);
        uVar5   = (**ppcVar1)();
        param_2 = (ushort)((ulong)uVar5 >> 0x10);
        uVar2   = (uint)uVar5;
    }
    if(*(int *)(uVar3 + 0xea) == 0x0)
    {
        *(undefined2 *)(uVar3 + 0xea) = 0x1;
        uVar5                         = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar3 + 0x8), 0x16, param_2, uVar3, (ushort)&PTR_LOOP_1050_1038, param_3);
        uVar2                         = (uint)uVar5;
    }
    return uVar2;
}

void __stdcall16far pass1_1018_50ac(ushort *param_1, ushort param_2)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                        = (int)param_1;
    *param_1                     = 0x56d2;
    *(undefined2 *)(iVar4 + 0x2) = 0x1018;
    puVar1                       = (undefined4 *)*(uint *)(iVar4 + 0xe);
    uVar2                        = *(uint *)(iVar4 + 0x10);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_50ea(ulong param_1, ushort param_2, ulong param_3)

{
    int         iVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    uint        uVar4;
    uint        uVar5;
    uint        uVar6;
    uint        uVar7;
    int         iVar8;
    uint        uVar9;
    undefined2  uVar10;
    astruct_99 *paStack6;

    paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
    uVar6    = (uint)((ulong)paStack6 >> 0x10);
    uVar4    = (uint)paStack6;
    if((uchar *)(uVar6 | uVar4) == (uchar *)0x0)
    {
        paStack6 = (astruct_99 *)0x0;
    }
    else
    {
        paStack6->field_0x0          = 0x389a;
        *(undefined2 *)(uVar4 + 0x2) = 0x1008;
        *(undefined2 *)(uVar4 + 0x4) = 0x0;
        *(undefined2 *)(uVar4 + 0x6) = 0x0;
        *(undefined2 *)(uVar4 + 0x8) = 0x0;
        *(undefined2 *)(uVar4 + 0xa) = 0x0;
        *(undefined2 *)(uVar4 + 0xc) = 0x0;
        paStack6->field_0x0          = 0x56ce;
        *(undefined2 *)(uVar4 + 0x2) = 0x1018;
    }
    uVar9                    = (uint)((ulong)paStack6 >> 0x10);
    uVar7                    = (uint)paStack6;
    *(ushort *)(uVar7 + 0xa) = param_2;
    uVar10                   = (undefined2)(param_1 >> 0x10);
    iVar8                    = (int)param_1;
    uVar3                    = *(undefined4 *)(iVar8 + 0xa);
    iVar1                    = *(int *)((int)uVar3 + 0xc);
    if(iVar1 == 0x1)
    {
        uVar3                  = *(undefined4 *)(iVar8 + 0xa);
        uVar5                  = *(uint *)((int)uVar3 + 0xe);
        *(uint *)(uVar7 + 0x4) = uVar5;
    }
    else
    {
        if(iVar1 == 0x5)
        {
            uVar3                  = *(undefined4 *)(iVar8 + 0xa);
            uVar5                  = *(uint *)((int)uVar3 + 0xe);
            *(uint *)(uVar7 + 0x6) = uVar5;
        }
        else
        {
            if(iVar1 != 0x6)
            {
                if((uVar9 | uVar7) == 0x0)
                {
                    return;
                }
                ppcVar2 = (code **)*(undefined4 *)paStack6;
                (**ppcVar2)();
                return;
            }
            uVar3                  = *(undefined4 *)(iVar8 + 0xa);
            uVar5                  = *(uint *)((int)uVar3 + 0xe);
            *(uint *)(uVar7 + 0x8) = uVar5;
        }
    }
    pass1_1030_6c66(param_3, 0x1, (ulong)paStack6, uVar5, (uchar *)(uVar6 | uVar4), 0x1030);
    return;
}


void __stdcall16far pass1_1018_51d2(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe);
    uVar2  = *(uint *)(iVar4 + 0x10);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *(undefined4 *)(iVar4 + 0xe) = 0x0;
    return;
}


ulong __stdcall16far pass1_1018_5206(ulong param_1, ulong param_2, ushort param_3)

{
    int        iVar1;
    uint       uVar2;
    int        iVar3;
    undefined2 uVar4;
    undefined4 uVar5;
    undefined  local_a[0x8];

    uVar4                        = (undefined2)(param_1 >> 0x10);
    iVar3                        = (int)param_1;
    *(undefined4 *)(iVar3 + 0xa) = 0x0;
    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)(iVar3 + 0xe));
    do
    {
        uVar5                        = pass1_1008_5b12(local_a, param_3);
        uVar2                        = (uint)((ulong)uVar5 >> 0x10);
        *(undefined2 *)(iVar3 + 0xa) = (int)uVar5;
        *(uint *)(iVar3 + 0xc)       = uVar2;
        if((uVar2 | *(uint *)(iVar3 + 0xa)) == 0x0)
            break;
        uVar5 = *(undefined4 *)(iVar3 + 0xa);
        iVar1 = pass1_1000_3d7a(*(ulong *)((int)uVar5 + 0x4), param_2);
    } while(iVar1 != 0x0);
    return CONCAT22(*(undefined2 *)(iVar3 + 0xc), *(undefined2 *)(iVar3 + 0xa));
}


ulong __stdcall16far pass1_1018_526a(ulong param_1, ulong param_2, ushort param_3)

{
    int  iVar1;
    uint uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(long *)(iVar1 + 0xe) == 0x0)
    {
        pass1_1018_5292(param_1 & 0xffff | (ulong)uVar2 << 0x10, param_2, param_3);
    }
    return CONCAT22(*(undefined2 *)(iVar1 + 0x10), *(undefined2 *)(iVar1 + 0xe));
}

ushort *__stdcall16far pass1_1018_567c(ushort *param_1, byte param_2)

{
    ushort uVar1;

    uVar1                 = (ushort)((ulong)param_1 >> 0x10);
    *param_1              = 0x389a;
    ((int *)param_1)[0x1] = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        pass1_1000_093a((int *)param_1, uVar1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1018_5714(ushort *param_1, ushort param_2)

{
    *param_1                            = 0x5830;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    pass1_1010_1d80(param_1, param_2);
    return;
}


ulong __stdcall16far pass1_1018_5732(ushort param_1, ushort param_2, ulong param_3, ushort param_4, ushort param_5, ushort param_6)

{
    ulong uVar1;

    uVar1 = pass1_1030_6d4e(param_3, param_4, param_5, param_6);
    return uVar1;
}


void __stdcall16far pass1_1018_5742(ushort param_1, ushort param_2, ulong *param_3, ulong param_4)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    ulong       uVar3;
    bool        bVar4;
    undefined4 *puVar5;
    ulong       uVar6;
    uint        extraout_DX;
    uint        extraout_DX_00;
    ulong       uStack16;

    bVar4   = false;
    puVar1  = (undefined4 *)*(undefined4 *)((int)param_3 + 0x4);
    ppcVar2 = (code **)((int)*puVar1 + 0x10);
    puVar5  = puVar1;
    (**ppcVar2)();
    uVar3    = (ulong)puVar5 & 0xffff | (ulong)extraout_DX << 0x10;
    uStack16 = 0x0;
    do
    {
        if(uVar3 <= uStack16)
        {
        LAB_1018_579f:
            if(!bVar4)
            {
                if(param_3 != (ulong *)0x0)
                {
                    ppcVar2 = (code **)*param_3;
                    (**ppcVar2)();
                }
                param_3 = (ulong *)0x0;
            }
            pass1_1030_6d80(param_4, (ulong)param_3);
            return;
        }
        ppcVar2 = (code **)((int)*puVar1 + 0x4);
        uVar6   = uVar3;
        (**ppcVar2)();
        if((extraout_DX_00 | (uint)uVar6) != 0x0)
        {
            bVar4 = true;
            goto LAB_1018_579f;
        }
        uStack16 = uStack16 + 0x1;
    } while(true);
}


void __stdcall16far pass1_1018_57d2(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0xa) = param_2;
    return;
}
