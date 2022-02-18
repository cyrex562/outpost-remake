
void __stdcall16far pass1_1040_5626(astruct_57 *param_1, ulong param_2, ushort param_3, uchar *param_4)

{
    int         *piVar1;
    uint         uVar2;
    uchar       *puVar3;
    int          iVar4;
    undefined2   uVar5;
    ulong        uVar6;
    undefined2   uVar7;
    int         *piStack12;
    astruct_441 *iVar8;
    astruct_396 *iVar7;
    astruct_439 *iVar6;

    iVar8 = (astruct_441 *)param_1;
    uVar7 = (undefined2)((ulong)param_1 >> 0x10);
    struct_1040_b082(param_1, CONCAT22(param_3, 0xfa3));
    uVar2                  = 0x0;
    iVar8->field_0x94      = 0x0;
    iVar8->field_0x96      = 0x0;
    iVar8->field_0x98      = 0x0;
    iVar8->field_0x9c      = 0x0;
    *(undefined2 *)param_1 = 0x6386;
    iVar8->field_0x2       = (int)&PTR_LOOP_1050_1040;
    mem_op_1000_179c(0x18, param_4, 0x1000);
    puVar3 = (uchar *)((uint)param_4 | uVar2);
    if(puVar3 == (uchar *)0x0)
    {
        iVar8->field_0x90 = (int *)0x0;
    }
    else
    {
        struct_1040_a598((ushort *)CONCAT22(param_4, uVar2));
        *(uint *)&iVar8->field_0x90                = uVar2;
        *(uchar **)((int)&iVar8->field_0x90 + 0x2) = puVar3;
    }
    *iVar8->field_0x90 = 0x6;
    iVar4              = *iVar8->field_0x90;
    uVar2              = iVar4 * 0xa + 0x2;
    mem_op_1000_179c(uVar2, puVar3, 0x1000);
    piStack12 = (int *)CONCAT22(puVar3, uVar2);
    if(((uint)puVar3 | uVar2) == 0x0)
    {
        piVar1                             = iVar8->field_0x90;
        *(undefined4 *)((int)piVar1 + 0x2) = 0x0;
    }
    else
    {
        *piStack12 = iVar4;
        pass1_1000_5586((uchar *)0xa564, (ushort)&PTR_LOOP_1050_1040, iVar4, 0xa, uVar2 + 0x2, (ushort)puVar3);
        piVar1                   = iVar8->field_0x90;
        uVar5                    = (undefined2)((ulong)piVar1 >> 0x10);
        iVar4                    = (int)piVar1;
        *(int *)(iVar4 + 0x2)    = uVar2 + 0x2;
        *(uchar **)(iVar4 + 0x4) = puVar3;
    }
    piVar1                              = iVar8->field_0x90;
    *(ulong *)((int)piVar1 + 0x6)       = param_2;
    piVar1                              = iVar8->field_0x90;
    *(undefined2 *)((int)piVar1 + 0xa)  = 0x4;
    piVar1                              = iVar8->field_0x90;
    *(undefined2 *)((int)piVar1 + 0x12) = iVar8->field_0xa;
    uVar6                               = pass1_1040_5d12((ulong)param_1);
    uVar2                               = (uint)(uVar6 >> 0x10);
    if((uVar2 | (uint)uVar6) == 0x0)
    {
        iVar8->field_0x9a = 0x0;
    }
    else
    {
        iVar8->field_0x9a = *(undefined2 *)((uint)uVar6 + 0x20);
    }
    return;
}


ushort __stdcall16far pass1_1040_5cd6(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;
    ulong      uVar3;

    uVar3 = pass1_1040_5d12(param_1);
    if(uVar3 != 0x0)
    {
        iVar1 = *(int *)((int)uVar3 + 0x20);
        uVar2 = (undefined2)(param_1 >> 0x10);
        if(*(int *)((int)param_1 + 0x9a) != iVar1)
        {
            *(int *)((int)param_1 + 0x9a) = iVar1;
            return 0x1;
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1040_5dc4(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    code       **ppcVar1;
    undefined4   uVar2;
    ushort       uVar3;
    uint         uVar4;
    uint         uVar5;
    uchar       *puVar6;
    uint         extraout_DX;
    astruct_724 *iVar7;
    undefined2   uVar7;
    ushort      *puVar8;
    undefined4  *puVar9;
    ushort       uVar10;
    int          iStack18;

    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, param_2, param_3);
    puVar6 = (uchar *)((ulong)puVar8 >> 0x10);
    uVar3  = (ushort)puVar8;
    uVar7  = (undefined2)(param_1 >> 0x10);
    iVar7  = (astruct_724 *)param_1;
    pass1_1010_a5ca(uVar3, (ushort)puVar6, iVar7->field_0x9a, uVar3, (ushort)puVar6);
    if(uVar3 == 0x0)
    {
        iVar7->field_0x94 = 0x0;
        iVar7->field_0x9c = 0x1;
    }
    if(-0x1 < (int)uVar3)
    {
        if((int)iVar7->field_0x9a < 0x72)
        {
            uVar10 = 0x31;
        }
        else
        {
            uVar10 = 0x41;
        }
        puVar9  = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, uVar10, param_4, puVar6, param_3);
        uVar4   = iVar7->field_0x9a;
        ppcVar1 = (code **)((int)*puVar9 + 0x14);
        (**ppcVar1)(0x1010, (int)puVar9, (int)((ulong)puVar9 >> 0x10), uVar4, (int)uVar4 >> 0xf);
        if((extraout_DX | uVar4) == 0x0)
        {
            iStack18 = 0x0;
        }
        else
        {
            uVar2    = *(undefined4 *)(uVar4 + 0x16);
            iStack18 = *(int *)((int)uVar2 + 0x4);
        }
        if((iStack18 != 0x0) && (uVar3 != 0x0))
        {
            uVar5             = (int)((iStack18 - uVar3) * 0x64) / iStack18;
            uVar4             = uVar5 / 0xa;
            iVar7->field_0x94 = uVar4;
            if(0x4 < uVar5 % 0xa)
            {
                iVar7->field_0x94 = uVar4 + 0x1;
            }
        }
    }
    return;
}


void __stdcall16far pass1_1040_288e(ulong param_1)

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
    puVar5  = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
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


ushort __stdcall16far pass1_1040_0d80(void)

{
    return 0x1;
}


ulong __stdcall16far pass1_1038_df5c(ulong param_1, ushort param_2, ushort param_3)

{
    ushort     uVar1;
    undefined2 uVar2;
    ulong      uVar3;

    uVar2 = (undefined2)(param_1 >> 0x10);
    uVar1 = (ushort)param_1;
    pass1_1010_038e(*(ulong *)(uVar1 + 0x92), 0x1, param_3);
    uVar3 = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar1 + 0x8), 0x20, param_2, uVar1, 0x1010, param_3);
    return uVar3;
}


void __stdcall16far pass1_1038_b6e0(ulong param_1, int param_2)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;
    uint       uStack4;

    uStack4 = 0x1;
    while(true)
    {
        if(0x2a < uStack4)
        {
            return;
        }
        uVar3 = (undefined2)(param_1 >> 0x10);
        iVar2 = (int)param_1;
        if(((*(uint *)(uStack4 * 0x4 + iVar2 + 0x2) | *(uint *)(uStack4 * 0x4 + iVar2)) != 0x0)
           && (uVar1 = *(undefined4 *)(uStack4 * 0x4 + iVar2), *(int *)((int)uVar1 + 0x6) == param_2))
            break;
        uStack4 = uStack4 + 0x1;
    }
    *(undefined4 *)(uStack4 * 0x4 + iVar2) = 0x0;
    return;
}
