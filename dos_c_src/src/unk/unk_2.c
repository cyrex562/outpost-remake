
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

void __stdcall16far pass1_1038_a174(ulong param_1, int param_2)

{
    if(param_2 == 0x1)
    {
        *(undefined2 *)((int)param_1 + 0x8e) = 0x0;
    }
    return;
}


ushort *__stdcall16far pass1_1038_a33c(ushort *param_1, ushort param_2)

{
    ushort uVar1;

    uVar1 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1038_a122((int)param_1, uVar1, 0x1, 0x0, CONCAT22(param_2, 0xfc7));
    *param_1                            = 0xa428;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return param_1;
}


void __stdcall16far pass1_1038_a36a(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xa428;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}


ushort *__stdcall16far pass1_1038_a494(ushort *param_1, ushort param_2)

{
    ushort uVar1;

    uVar1 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1038_a122((int)param_1, uVar1, 0x1, 0x0, CONCAT22(param_2, 0xfc8));
    *param_1                            = 0xa62e;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return param_1;
}


void __stdcall16far pass1_1038_a4c2(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xa62e;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}


ushort *__stdcall16far pass1_1038_a69a(ushort *param_1, ushort param_2)

{
    ushort uVar1;

    uVar1 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1038_a122((int)param_1, uVar1, 0x1, 0x0, CONCAT22(param_2, 0xfc9));
    *param_1                            = 0xa832;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return param_1;
}


void __stdcall16far pass1_1038_a6c8(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xa832;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}


ushort *__stdcall16far pass1_1038_a89e(ushort *param_1, ushort param_2)

{
    ushort uVar1;

    uVar1 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1038_a122((int)param_1, uVar1, 0x1, 0x0, CONCAT22(param_2, 0xfca));
    *param_1                            = 0xab16;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    return param_1;
}


void __stdcall16far pass1_1038_a8cc(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xab16;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}


ushort *__stdcall16far pass1_1038_adde(int param_1, ushort param_2, ushort param_3, ulong param_4)

{
    pass1_1038_9b72(param_1, param_2, param_3, param_4);
    *(ushort *)CONCAT22(param_2, param_1) = 0xae4e;
    *(undefined2 *)(param_1 + 0x2)        = (int)&PTR_LOOP_1050_1038;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1038_af34(void)

{
    _PTR_LOOP_1050_5b7c = 0x0;
    return;
}


undefined4 __stdcall16far pass1_1038_af40(
  ulong param_1, ushort param_2, int param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7)

{
    code      **ppcVar1;
    undefined4  uVar2;
    undefined  *puVar3;
    uchar      *puVar4;
    uint        uVar5;
    int         iVar6;
    int         unaff_DI;
    undefined2  uVar7;
    undefined2  uVar8;
    astruct_57 *paVar9;

    puVar3 = (undefined *)bring_win_to_top_1038_b72e(param_1, param_3, param_6);
    iVar6  = (int)param_1;
    uVar7  = (undefined2)(param_1 >> 0x10);
    if(puVar3 != (undefined *)0x0)
        goto LAB_1038_b61f;
    uVar8              = SUB42(&PTR_LOOP_1050_1038, 0x0);
    PTR_LOOP_1050_5b82 = puVar3;
    switch(param_3)
    {
    case 0x1:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x8e, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
        {
        LAB_1038_afa0:
            uVar8  = 0x1000;
            paVar9 = (astruct_57 *)0x0;
        }
        else
        {
            paVar9 = pass1_1038_9f76((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2);
        }
        break;
    case 0x2:
        mem_op_1000_179c(0x96, (uchar *)param_4, 0x1000);
        uVar5 = param_4 | param_5;
        if(uVar5 == 0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_181c((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, uVar5, param_7);
        paVar9 = (astruct_57 *)CONCAT22(uVar5, param_5);
        break;
    case 0x3:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_e99a(
          (astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, (uchar *)(param_4 | param_5), param_7);
        break;
    case 0x4:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_c7b8(
          (astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, (uchar *)(param_4 | param_5), param_7);
        break;
    case 0x5:
        mem_op_1000_179c(0x96, (uchar *)param_4, 0x1000);
        uVar5 = param_4 | param_5;
        if(uVar5 == 0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_23ea((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, param_7, uVar5);
        paVar9 = (astruct_57 *)CONCAT22(uVar5, param_5);
        break;
    case 0x6:
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8  = SUB42(&PTR_LOOP_1050_1040, 0x0);
        paVar9 = pass1_1040_06e8(
          (astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, (uchar *)(param_4 | param_5), param_7);
        break;
    case 0x7:
        mem_op_1000_179c(0x9c, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_4068((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x8:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x9a, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        pass1_1038_b772((astruct_57 *)CONCAT22(param_4, param_5), puVar4, unaff_DI, param_7, param_2);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x9:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x8e, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_e140((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2);
        break;
    case 0xa:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x90, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = (astruct_57 *)pass1_1038_a33c((ushort *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0xb:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x90, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = (astruct_57 *)pass1_1038_a494((ushort *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0xc:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x90, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = (astruct_57 *)pass1_1038_a69a((ushort *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0xd:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x90, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = (astruct_57 *)pass1_1038_a89e((ushort *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0xe:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x94, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        pass1_1038_e69a((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0xf:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x94, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        pass1_1038_cd06((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x10:
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8  = SUB42(&PTR_LOOP_1050_1040, 0x0);
        paVar9 = pass1_1040_0bfc((astruct_57 *)CONCAT22(param_4, param_5),
                                 0x0,
                                 0x0,
                                 0x0,
                                 param_2,
                                 (uchar *)(param_4 | param_5),
                                 unaff_DI,
                                 param_7);
        break;
    case 0x11:
        mem_op_1000_179c(0x9a, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_0e1c((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x12:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x9a, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_d756(
          (astruct_57 *)CONCAT22(param_4, param_5), param_2, (uchar *)(param_4 | param_5), unaff_DI, param_7);
        break;
    case 0x13:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_cad8(
          (astruct_57 *)CONCAT22(param_4, param_5), param_2, (uchar *)(param_4 | param_5), unaff_DI, param_7);
        break;
    case 0x14:
        mem_op_1000_179c(0xaa, (uchar *)param_4, 0x1000);
        uVar5 = param_4 | param_5;
        if(uVar5 == 0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_1f5a((astruct_57 *)CONCAT22(param_4, param_5), param_2, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(uVar5, param_5);
        break;
    case 0x15:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x8e, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_d242((astruct_57 *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0x16:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x9a, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        pass1_1038_eeda((astruct_57 *)CONCAT22(param_4, param_5), param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x17:
        mem_op_1000_179c(0x96, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        uVar8  = 0x1018;
        paVar9 = pass1_1018_5e26((astruct_57 *)CONCAT22(param_4, param_5), param_2);
        break;
    default:
        goto switchD_1038_b581_caseD_18;
    case 0x19:
        mem_op_1000_179c(0x96, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_1cb4((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x1a:
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8  = SUB42(&PTR_LOOP_1050_1040, 0x0);
        paVar9 = pass1_1040_123e((astruct_57 *)CONCAT22(param_4, param_5),
                                 0x0,
                                 0x0,
                                 0x0,
                                 param_2,
                                 (uchar *)(param_4 | param_5),
                                 unaff_DI,
                                 param_7);
        break;
    case 0x1b:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x8e, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_ab82((astruct_57 *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0x1c:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_e2d0((astruct_57 *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0x1d:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x92, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_eb9e((astruct_57 *)CONCAT22(param_4, param_5), param_2);
        break;
    case 0x1e:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x29e, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        pass1_1038_bddc((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x1f:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x9a, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        pass1_1038_c4a2((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x20:
        mem_op_1000_179c(0x29a, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_2ea2((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x21:
        mem_op_1000_179c(0xa6, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_3966((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x22:
        mem_op_1000_179c(0x9a, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_34a2((astruct_57 *)CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x23:
        mem_op_1000_179c(0x9c, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_ac84((astruct_57 *)CONCAT22(param_4, param_5), param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x25:
        mem_op_1000_179c(0xa0, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_ca16((astruct_57 *)CONCAT22(param_4, param_5), param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x26:
        mem_op_1000_179c(0xa2, (uchar *)param_4, 0x1000);
        uVar5 = param_4 | param_5;
        if(uVar5 == 0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_d0f8((astruct_57 *)CONCAT22(param_4, param_5), param_2);
        paVar9 = (astruct_57 *)CONCAT22(uVar5, param_5);
        break;
    case 0x27:
        uVar8 = 0x1000;
        mem_op_1000_179c(0xa0, (uchar *)param_4, 0x1000);
        uVar5 = param_4 | param_5;
        if(uVar5 == 0x0)
            goto LAB_1038_afa0;
        pass1_1038_88f2((astruct_57 *)CONCAT22(param_4, param_5), param_2);
        paVar9 = (astruct_57 *)CONCAT22(uVar5, param_5);
        break;
    case 0x28:
        mem_op_1000_179c(0x96, (uchar *)param_4, 0x1000);
        puVar4 = (uchar *)(param_4 | param_5);
        if(puVar4 == (uchar *)0x0)
            goto LAB_1038_afa0;
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        pass1_1040_6402((astruct_57 *)CONCAT22(param_4, param_5), param_2, puVar4, unaff_DI, param_7);
        paVar9 = (astruct_57 *)CONCAT22(puVar4, param_5);
        break;
    case 0x29:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x98, (uchar *)param_4, 0x1000);
        if((param_4 | param_5) == 0x0)
            goto LAB_1038_afa0;
        paVar9
          = pass1_1038_7d10((astruct_57 *)CONCAT22(param_4, param_5), param_2, param_4 | param_5, unaff_DI, param_7);
        break;
    case 0x2a:
        uVar8 = 0x1000;
        mem_op_1000_179c(0x98, (uchar *)param_4, 0x1000);
        if((uchar *)(param_4 | param_5) == (uchar *)0x0)
            goto LAB_1038_afa0;
        paVar9 = pass1_1038_8caa(
          (astruct_57 *)CONCAT22(param_4, param_5), param_2, (uchar *)(param_4 | param_5), unaff_DI, param_7);
    }
    *(undefined2 *)(param_3 * 0x4 + iVar6)       = (int)paVar9;
    *(undefined2 *)(param_3 * 0x4 + iVar6 + 0x2) = (int)((ulong)paVar9 >> 0x10);
switchD_1038_b581_caseD_18:
    if(*(long *)(param_3 * 0x4 + iVar6) != 0x0)
    {
        if(*(int *)(iVar6 + 0xae) != 0x0)
        {
            uVar2                              = *(undefined4 *)(param_3 * 0x4 + iVar6);
            *(undefined2 *)((int)uVar2 + 0x6e) = *(undefined2 *)(iVar6 + 0xae);
        }
        *(undefined2 *)(iVar6 + 0xae) = 0x0;
        uVar2                         = *(undefined4 *)(param_3 * 0x4 + iVar6);
        ppcVar1                       = (code **)((int)*(undefined4 *)*(undefined4 *)(param_3 * 0x4 + iVar6) + 0x8);
        (**ppcVar1)(uVar8, (int)uVar2, (int)((ulong)uVar2 >> 0x10));
    }
LAB_1038_b61f:
    return CONCAT22(*(undefined2 *)(param_3 * 0x4 + iVar6 + 0x2), *(undefined2 *)(param_3 * 0x4 + iVar6));
}


int __stdcall16far pass1_1038_993a(ushort param_1, ushort param_2, int param_3)

{
    int iStack6;

    iStack6 = 0x0;
    while(true)
    {
        if(0xe < iStack6)
        {
            return -0x1;
        }
        if(*(int *)(iStack6 * 0xe + 0x5a70) == param_3)
            break;
        iStack6 = iStack6 + 0x1;
    }
    return iStack6;
}


ushort *__stdcall16far pass1_1038_9a1e(int param_1, ushort param_2, ushort param_3, ulong param_4)

{
    pass1_1040_b040(
      (astruct_57 *)CONCAT22(param_2, param_1), CONCAT22((int)param_4, param_3), (ushort)(param_4 >> 0x10));
    *(ushort *)CONCAT22(param_2, param_1) = 0x9af6;
    *(undefined2 *)(param_1 + 0x2)        = (int)&PTR_LOOP_1050_1038;
    return (ushort *)CONCAT22(param_2, param_1);
}


ulong __stdcall16far pass1_1038_9b72(int param_1, ushort param_2, ushort param_3, ulong param_4)

{
    int iStack4;

    pass1_1040_b040(
      (astruct_57 *)CONCAT22(param_2, param_1), CONCAT22((int)param_4, param_3), (ushort)(param_4 >> 0x10));
    *(undefined2 *)(param_1 + 0x128)          = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x9efa;
    *(undefined2 *)(param_1 + 0x2)            = (int)&PTR_LOOP_1050_1038;
    iStack4                                   = 0x0;
    do
    {
        *(undefined2 *)(param_1 + iStack4 * 0x2 + 0x94) = 0x0;
        iStack4                                         = iStack4 + 0x1;
    } while(iStack4 < 0x4a);
    return CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1038_79f2(ulong param_1, ulong param_2, ushort param_3)

{
    code     **ppcVar1;
    undefined *puVar2;
    uint       extraout_DX;
    int        iVar3;
    undefined2 uVar4;
    undefined  local_e[0x8];
    long       lStack6;

    lStack6 = *(long *)((int)param_2 + 0x4);
    uVar4   = (undefined2)(param_1 >> 0x10);
    iVar3   = (int)param_1;
    pass1_1008_5784((ulong *)CONCAT22(param_3, local_e), *(ulong *)(iVar3 + 0x4));
    do
    {
        puVar2 = local_e;
        pass1_1008_5b12(puVar2, param_3);
        if((extraout_DX | (uint)puVar2) == 0x0)
        {
            return;
        }
    } while(*(long *)(puVar2 + 0x4) != lStack6);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0xc);
    (**ppcVar1)(0x1008, *(undefined4 *)(iVar3 + 0x4), puVar2, extraout_DX);
    return;
}


void __stdcall16far pass1_1038_7a76(ulong *param_1, undefined2 param_2, int param_3, ushort param_4)

{
    code     **ppcVar1;
    ushort     uVar2;
    ulong      uVar3;
    undefined  local_a[0x4];
    undefined4 uStack6;

    pass1_1008_5784((ulong *)CONCAT22(param_4, local_a), *param_1);
    while(true)
    {
        uVar3 = pass1_1008_5b12(local_a, param_4);
        if(uVar3 == 0x0)
            break;
        pass1_1038_6a0e(uVar3, (uint)uVar3, (uint)(uVar3 >> 0x10) | (uint)uVar3, param_2, param_3, param_4);
    }
    do
    {
        uStack6 = 0x0;
        do
        {
            uVar3 = pass1_1008_5b12(local_a, param_4);
            if(uVar3 == 0x0)
            {
                pass1_1008_5784((ulong *)CONCAT22(param_4, local_a), *(ulong *)((int)param_1 + 0x4));
                while(true)
                {
                    uVar3 = pass1_1008_5b12(local_a, param_4);
                    if(uVar3 == 0x0)
                        break;
                    pass1_1030_affc(uVar3, param_3, param_4);
                }
                return;
            }
            uVar2 = pass1_1038_6b3c(uVar3);
        } while(uVar2 == 0x0);
        ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0xc);
        (**ppcVar1)(0x1008);
    } while(true);
}


void __stdcall16far pass1_1038_8848(void)

{
    return;
}


void __stdcall16far pass1_1038_884c(void)

{
    return;
}


void __stdcall16far
pass1_1038_6a0e(ulong param_1, undefined2 param_2, undefined2 param_3, ushort param_4, ushort param_5, ushort param_6)

{
    int       *piVar1;
    undefined4 uVar2;
    uint       uVar3;
    BOOL16     BVar4;
    ushort     uVar5;
    uint       uVar6;
    uint       uVar7;
    ushort     uVar8;
    ushort     uVar9;
    ushort    *puVar10;
    ulong      uVar11;
    ulong      uStack22;
    undefined  local_10[0x4];
    undefined  local_c[0x6];
    ulong      uStack6;

    uVar9 = (ushort)(param_1 >> 0x10);
    uVar8 = (ushort)param_1;
    if(*(int *)(uVar8 + 0x28) == 0x0)
    {
        uVar2 = *(undefined4 *)(uVar8 + 0x20);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)((ulong)uVar2 >> 0x10));
        uStack6 = CONCAT22(param_3, param_2);
        piVar1  = (int *)(uVar8 + 0x24);
        *piVar1 = *piVar1 + 0x3c;
        puVar10 = pass1_1008_3e38((ushort *)CONCAT22(param_6, local_c));
        uVar6   = (uint)((ulong)puVar10 >> 0x10);
        while(true)
        {
            uVar3 = pass1_1038_6d24(param_1,
                                    (ulong *)CONCAT22(param_6, local_10),
                                    (ushort *)CONCAT22(param_6, local_c),
                                    (int)uStack6,
                                    (ushort)(uStack6 >> 0x10),
                                    param_6);
            if(uVar3 == 0x0)
            {
                pass1_1010_8fba(*(ulong *)(uVar8 + 0x4), 0x0);
                uStack22 = CONCAT22(uVar6, uVar3);
                uVar7    = uVar6 | uVar3;
                if(uVar7 == 0x0)
                {
                    uVar2 = *(undefined4 *)(uVar8 + 0x20);
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)((ulong)uVar2 >> 0x10));
                    pass1_1038_7356(param_1, CONCAT22(uVar7, uVar3), param_6, param_4, param_5);
                    return;
                }
                uVar11 = struct_op_1030_73a8(uStack6);
                BVar4  = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((int)uVar11 + 0xc), 0x40);
                if(BVar4 != 0x0)
                {
                    *(undefined2 *)(uVar8 + 0x28) = 0x1;
                    *(ulong *)(uVar8 + 0x20)      = uStack22;
                    return;
                }
                *(ulong *)(uVar8 + 0x20) = uStack22;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, *(ushort *)(uVar8 + 0x20), uVar6);
                uStack6 = uStack22 & 0xffff | (ulong)uVar6 << 0x10;
            }
            uVar5 = pass1_1038_6e1a(uVar8, uVar9, (long *)CONCAT22(param_6, local_10));
            if(*(int *)(uVar8 + 0x24) < (int)uVar5)
                break;
            piVar1  = (int *)(uVar8 + 0x24);
            *piVar1 = *piVar1 - uVar5;
            pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)(uVar8 + 0x1a)),
                            (ushort *)CONCAT22(param_6, local_c));
        }
    }
    return;
}


ushort __stdcall16far pass1_1038_6b3c(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if((((*(int *)(iVar1 + 0xc) == 0x0) && (*(int *)(iVar1 + 0x12) == 0x0)) && (*(int *)(iVar1 + 0x14) == 0x0))
       && ((*(long *)(iVar1 + 0xe) == 0x0 && (*(long *)(iVar1 + 0x16) != 0x0))))
    {
        *(undefined4 *)(iVar1 + 0x16) = 0x0;
    }
    if(*(long *)(iVar1 + 0x16) == 0x0)
    {
        return 0x1;
    }
    return 0x0;
}


void __stdcall16far pass1_1038_6bd4(
  ulong param_1, ushort *param_2, ulong *param_3, int param_4, uchar *param_5, int param_6, ushort param_7)

{
    ushort uStack4;

    pass1_1008_3f62(param_2, (ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x1a)));
    if(param_4 < 0x0)
    {
        uStack4 = *param_2 - 0x1;
    }
    else
    {
        uStack4 = *param_2 + 0x1;
    }
    *param_2 = uStack4;
    pass1_1038_6b88((ushort)param_1, (ushort)(param_1 >> 0x10), param_2, param_3, param_5, param_6, param_7);
    return;
}


void __stdcall16far pass1_1038_6c1c(
  ulong param_1, ushort *param_2, ulong *param_3, int param_4, uchar *param_5, int param_6, ushort param_7)

{
    undefined2 uVar1;
    int        iStack4;

    pass1_1008_3f62(param_2, (ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x1a)));
    uVar1   = (undefined2)((ulong)param_2 >> 0x10);
    iStack4 = *(int *)((int)param_2 + 0x2);
    if(param_4 < 0x0)
    {
        iStack4 = iStack4 + -0x1;
    }
    else
    {
        iStack4 = iStack4 + 0x1;
    }
    *(int *)((int)param_2 + 0x2) = iStack4;
    pass1_1038_6b88((ushort)param_1, (ushort)(param_1 >> 0x10), param_2, param_3, param_5, param_6, param_7);
    return;
}


int __stdcall16far
pass1_1038_6d24(ulong param_1, ulong *param_2, ushort *param_3, int param_4, ushort param_5, ushort param_6)

{
    int        local_14;
    int        local_12;
    int        local_10;
    int        local_e;
    int        local_c;
    int        local_a;
    undefined4 local_8;
    undefined2 uStack4;

    *param_2 = 0x0;
    local_8  = *(undefined4 *)(param_4 + 0xc);
    uStack4  = *(undefined2 *)(param_4 + 0x10);
    pass1_1008_3eb4((ushort *)CONCAT22(param_6, &local_8),
                    (ushort *)CONCAT22(param_6, &local_e),
                    (ushort *)CONCAT22(param_6, &local_c),
                    (ushort *)CONCAT22(param_6, &local_a));
    pass1_1008_3eb4((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x1a)),
                    (ushort *)CONCAT22(param_6, &local_14),
                    (ushort *)CONCAT22(param_6, &local_12),
                    (ushort *)CONCAT22(param_6, &local_10));
    local_c = local_c - local_12;
    local_e = local_e - local_14;
    local_a = local_a - local_10;
    if(((local_a == 0x0) && (local_c == 0x0)) && (local_e == 0x0))
    {
        return 0x0;
    }
    if((local_c != 0x0) || (local_a == 0x0))
    {
        if((local_a == 0x0) && (local_c != 0x0))
        {
            pass1_1038_6c1c(param_1, param_3, param_2, local_c, (uchar *)0x0, (int)&stack0xfffe, param_6);
            return 0x1;
        }
        if(((local_a == 0x0) && (local_c == 0x0)) && (local_e != 0x0))
        {
            pass1_1038_6c68(param_1, param_3, param_2, local_e, (uchar *)0x0, (int)&stack0xfffe, param_6);
            if(local_c != 0x0)
            {
                return 0x1;
            }
            return local_c;
        }
    }
    pass1_1038_6bd4(param_1, param_3, param_2, local_a, (uchar *)local_a, (int)&stack0xfffe, param_6);
    return 0x1;
}


ushort __stdcall16far pass1_1038_6e1a(ushort param_1, ushort param_2, long *param_3)

{
    uint       uVar1;
    BOOL16     BVar2;
    ushort     uVar3;
    undefined2 uVar4;
    byte       bStack21;
    undefined2 uStack4;

    uStack4 = 0x0;
    if((*param_3 == 0x0) && (*(int *)param_3 == 0x0))
    {
        return 0x1;
    }
    uVar4    = *(undefined2 *)((int)param_3 + 0x2);
    bStack21 = (byte)((uint)uVar4 >> 0x8);
    uVar1    = (uint)bStack21;
    if(bStack21 == 0x0)
    {
        uStack4 = *(undefined2 *)param_3;
        goto switchD_1038_6eab_caseD_9;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)*param_3, (uint)((ulong)*param_3 >> 0x10));
    uVar3 = pass1_1030_6fa0(CONCAT22(uVar4, uVar1));
    if((int)uVar3 < 0xa)
    {
        switch(uVar3)
        {
        case 0x1:
            uStack4 = 0x1;
            break;
        case 0x2:
        case 0x6:
            uStack4 = 0x2;
            break;
        case 0x3:
        case 0x7:
            uStack4 = 0x3;
            break;
        case 0x4:
        case 0x8:
            uStack4 = 0x4;
            break;
        case 0x5:
        case 0x9:
            goto switchD_1038_6eab_caseD_5;
        }
    }
    else
    {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar3, 0x41);
        if(BVar2 != 0x0)
        {
            uStack4 = 0xa;
            goto switchD_1038_6eab_caseD_9;
        }
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar3, 0x42);
        if((BVar2 != 0x0) || (uVar3 == 0x3f))
        {
            uStack4 = 0xb;
            goto switchD_1038_6eab_caseD_9;
        }
    switchD_1038_6eab_caseD_5:
        uStack4 = 0x5;
    }
switchD_1038_6eab_caseD_9:
    switch(uStack4)
    {
    case 0x1:
        return 0x14;
    case 0x2:
    case 0x7:
        return 0x3c;
    case 0x3:
    case 0x8:
        return 0x78;
    case 0x4:
    case 0x9:
        return 0xf0;
    case 0x5:
    case 0x6:
        return 0xf;
    case 0xa:
        uVar3 = 0xc;
        break;
    case 0xb:
        uVar3 = 0xa;
        break;
    default:
        uVar3 = 0xffff;
    }
    return uVar3;
}


void __stdcall16far pass1_1038_6f5a(
  ulong param_1, ulong param_2, uint param_3, uchar *param_4, ushort param_5, ushort param_6, ushort param_7)

{
    undefined4   uVar1;
    long         lVar2;
    ushort      *puVar3;
    ushort       uVar4;
    uint         uVar5;
    int          iVar6;
    int          iVar7;
    undefined2   uVar8;
    undefined2   uVar9;
    astruct_99  *paStack16;
    uint         uStack12;
    undefined2   local_a;
    undefined2   uStack8;
    ushort       local_6;
    uint         uStack4;
    astruct_623 *uVar3;

    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if(*(long *)(iVar6 + 0xe) == 0x0)
    {
        if(*(int *)(iVar6 + 0xc) != 0x0)
        {
            pass1_1030_7ddc(
              param_2, *(long *)(iVar6 + 0x16), *(ushort *)(iVar6 + 0xc), param_3, param_4, param_5, param_6, param_7);
            return;
        }
        if(*(int *)(iVar6 + 0x12) != 0x0)
        {
            pass1_1030_7c50(param_2, *(long *)(iVar6 + 0x16), *(int *)(iVar6 + 0x12), param_3, param_4);
            return;
        }
        paStack16 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
        uVar5     = (uint)((ulong)paStack16 >> 0x10);
        uVar3     = (astruct_623 *)paStack16;
        if((uVar5 | (uint)uVar3) == 0x0)
        {
            paStack16 = (astruct_99 *)0x0;
        }
        else
        {
            paStack16->field_0x0 = 0x389a;
            uVar3->field_0x2     = 0x1008;
            uVar3->field_0x4     = 0x0;
            uVar3->field_0x6     = 0x0;
            uVar3->field_0x8     = 0x0;
            uVar3->field_0xa     = 0x0;
            uVar3->field_0xc     = 0x0;
            paStack16->field_0x0 = 0x56ce;
            uVar3->field_0x2     = 0x1018;
        }
        uVar9                        = (undefined2)((ulong)paStack16 >> 0x10);
        iVar7                        = (int)paStack16;
        *(undefined2 *)(iVar7 + 0x8) = *(undefined2 *)(iVar6 + 0x14);
        *(undefined2 *)(iVar7 + 0xa) = *(undefined2 *)(iVar6 + 0x16);
        uVar4                        = pass1_1020_c42e(*(int *)(iVar6 + 0x14));
        lVar2                        = (ulong)uVar4 * (ulong) * (uint *)(iVar7 + 0xa);
        uVar5                        = (uint)lVar2;
        *(uint *)(iVar7 + 0xc)       = uVar5;
        pass1_1030_6a2c(param_2, (long)paStack16, uVar5, (uchar *)((ulong)lVar2 >> 0x10), param_7);
    }
    else
    {
        uVar1   = *(undefined4 *)(iVar6 + 0xe);
        uStack4 = *(uint *)((int)uVar1 + 0x4);
        for(uStack12 = 0x0; uStack12 < uStack4; uStack12 = uStack12 + 0x1)
        {
            puVar3 = &local_6;
            pass1_1020_bb16(*(ulong **)(iVar6 + 0xe),
                            (ulong *)CONCAT22(param_7, &local_a),
                            (ushort *)CONCAT22(param_7, puVar3),
                            uStack12);
            if(CONCAT22(uStack8, local_a) != 0x0)
            {
                pass1_1030_7ddc(
                  param_2, CONCAT22(uStack8, local_a), local_6, (uint)puVar3, param_4, param_5, param_6, param_7);
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_50e0(ulong param_1, ushort param_2, ushort param_3)

{
    code     **ppcVar1;
    ushort     uVar2;
    BOOL16     BVar3;
    uint       extraout_DX;
    uint       uVar4;
    uint       uVar5;
    int        iVar6;
    undefined2 uVar7;
    ulong      uVar8;
    ulong      uStack14;
    ulong      uStack10;

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if(*(long *)(iVar6 + 0xc) == 0x0)
    {
        param_3 = 0x0;
        uVar4   = 0x0;
    }
    else
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar4, param_3);
    for(uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1)
    {
        uVar8 = uStack10;
        pass1_1030_1d58(*(ulong *)(iVar6 + 0xc));
        uVar5 = uVar4 | (uint)uVar8;
        if(uVar5 != 0x0)
        {
            uVar2 = pass1_1030_6fa0(uVar8 & 0xffff | (ulong)uVar4 << 0x10);
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, param_2);
            if(BVar3 != 0x0)
            {
                uVar8 = struct_op_1030_73a8(uVar8 & 0xffff | (ulong)uVar4 << 0x10);
                uVar5 = (uint)(uVar8 >> 0x10);
            }
        }
        uVar4 = uVar5;
    }
    return;
}


void __stdcall16far pass1_1038_518c(ulong param_1, ushort param_2, ushort param_3)

{
    uint      *puVar1;
    undefined4 uVar2;
    code     **ppcVar3;
    uint       uVar4;
    ulong      uVar5;
    undefined2 extraout_DX;
    uint       extraout_DX_00;
    uint       uVar6;
    int        iVar7;
    int        iVar8;
    int        iVar9;
    undefined2 uVar10;
    undefined2 uVar11;
    bool       bVar12;
    ulong      uVar13;
    int        iStack34;
    ulong      uStack32;
    ulong     *puStack28;
    ulong      uStack10;
    ulong      uStack6;

    uVar10 = (undefined2)(param_1 >> 0x10);
    iVar7  = (int)param_1;
    if(*(int *)(iVar7 + 0x206) == 0x0)
    {
        if(*(long *)(iVar7 + 0xc) == 0x0)
        {
            param_2 = 0x0;
            uVar11  = 0x0;
        }
        else
        {
            uVar2   = *(undefined4 *)(iVar7 + 0xc);
            ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
            (**ppcVar3)(param_3, (int)uVar2, (int)((ulong)uVar2 >> 0x10));
            uVar11 = extraout_DX;
        }
        uStack6 = CONCAT22(uVar11, param_2);
        for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
        {
            uVar2   = *(undefined4 *)(iVar7 + 0xc);
            ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x4);
            uVar5   = uStack6;
            (**ppcVar3)(
              (char)param_3, (char)uVar2, (int)((ulong)uVar2 >> 0x10), (int)uStack10, (int)(uStack10 >> 0x10));
            uVar4 = (uint)uVar5;
            uVar6 = extraout_DX_00 | uVar4;
            if(uVar6 != 0x0)
            {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
                param_3   = 0x1030;
                uVar13    = struct_op_1030_73a8(CONCAT22(uVar6, uVar4));
                uVar6     = (uint)(uVar13 >> 0x10);
                iVar8     = *(int *)((int)uVar13 + 0x12);
                uVar4     = (int)uVar13 + 0x14;
                uVar5     = (ulong)uVar4;
                puStack28 = (ulong *)(uVar13 & 0xffff0000 | (ulong)uVar4);
                uStack32  = 0x0;
                if((iVar8 == 0x4) || (iVar8 == 0x5))
                {
                    uVar5    = *puStack28;
                    uStack32 = uVar5;
                }
                if(uStack32 != 0x0)
                {
                    for(iStack34 = 0x11; iStack34 < 0x25; iStack34 = iStack34 + 0x1)
                    {
                        if(((*(int *)(iVar7 + 0x204) == 0x0) || (iStack34 == 0x23)) || (iStack34 == 0x24))
                        {
                            empty_1038_540a();
                            iVar8  = iStack34 * 0x4;
                            uVar11 = (undefined2)(uStack32 >> 0x10);
                            iVar9  = (int)uStack32;
                            puVar1 = (uint *)(iVar8 + iVar9 + 0x2);
                            bVar12 = *puVar1 < uVar6;
                            if((bVar12 || *puVar1 == uVar6)
                               && ((bVar12
                                    || (puVar1 = (uint *)(iVar8 + iVar9),
                                        *puVar1 < (uint)uVar5 || *puVar1 == (uint)uVar5))))
                            {
                                pass1_1038_5770(param_1, *(long *)(iVar8 + iVar9), iStack34);
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_52b8(
  ulong param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7)

{
    undefined4   uVar1;
    code       **ppcVar2;
    ulong        uVar3;
    int          iVar4;
    uint         uVar5;
    undefined2   extraout_DX;
    undefined2   uVar6;
    uint         extraout_DX_00;
    uint         uVar7;
    undefined2   uVar8;
    ulong        uVar9;
    int          iVar11;
    ushort       uVar12;
    uint         uStack26;
    int          iStack24;
    ulong        uStack22;
    ulong        uStack14;
    ulong        uStack10;
    astruct_601 *iVar10;

    iVar4  = -(int)param_2;
    iVar11 = (int)param_1;
    pass1_1038_5694(param_1, CONCAT22(-(param_2._2_2_ + (uint)((int)param_2 != 0x0)), iVar4), param_3);
    if(param_3 != 0x24)
    {
        uVar8 = (undefined2)(param_1 >> 0x10);
        if(*(long *)(iVar11 + 0xc) == 0x0)
        {
            iVar4 = 0x0;
            uVar6 = 0x0;
        }
        else
        {
            uVar1   = *(undefined4 *)(iVar11 + 0xc);
            ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xc) + 0x10);
            (**ppcVar2)(param_6, (int)uVar1, (int)((ulong)uVar1 >> 0x10));
            uVar6 = extraout_DX;
        }
        uStack10 = CONCAT22(uVar6, iVar4);
        for(uStack14 = 0x0; uVar3 = param_2, uStack14 < uStack10; uStack14 = uStack14 + 0x1)
        {
            uVar1   = *(undefined4 *)(iVar11 + 0xc);
            ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xc) + 0x4);
            uVar9   = uStack10;
            (**ppcVar2)(param_6, (char)uVar1, (int)((ulong)uVar1 >> 0x10), (int)uStack14, (int)(uStack14 >> 0x10));
            uVar5 = (uint)uVar9;
            uVar7 = extraout_DX_00 | uVar5;
            if(uVar7 != 0x0)
            {
                uVar12 = param_3;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
                uStack22 = CONCAT22(uVar7, uVar5);
                param_6  = 0x1030;
                uVar9    = pass1_1030_7c28(CONCAT22(uVar7, uVar5), uVar12, uVar5, uVar7, param_7);
                uVar7    = (uint)(uVar9 >> 0x10);
                uVar5    = (uint)uVar9;
                if((uVar7 | uVar5) != 0x0)
                {
                    if(uVar9 < param_2)
                    {
                        param_2  = param_2 - uVar9;
                        uStack26 = 0x0;
                        iStack24 = 0x0;
                    }
                    else
                    {
                        uStack26 = uVar5 - (uint)param_2;
                        iStack24 = (uVar7 - param_2._2_2_) - (uint)(uVar5 < (uint)param_2);
                        param_2  = 0x0;
                        uVar9    = uVar3;
                    }
                    param_6 = 0x1030;
                    pass1_1030_7d1c(uStack22,
                                    uStack26,
                                    CONCAT22(param_3, iStack24),
                                    (int)uVar9,
                                    param_2._2_2_,
                                    param_4,
                                    param_5,
                                    param_7);
                    if(param_2 == 0x0)
                    {
                        return;
                    }
                }
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_53ba(ulong param_1, int param_2)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    if(*(ulong *)((int)param_1 + 0x1a2 + param_2 * 0x4) < *(ulong *)((int)param_1 + 0x14e + param_2 * 0x4))
    {
        return;
    }
    return;
}


void __stdcall16far empty_1038_540a(void)

{
    return;
}


void __stdcall16far pass1_1038_5464(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    undefined4  uVar1;
    code      **ppcVar2;
    uint        uVar3;
    ulong       uVar4;
    uint        extraout_DX;
    uint        extraout_DX_00;
    uint        extraout_DX_01;
    uint        extraout_DX_02;
    uint        uVar5;
    int         iVar6;
    int         iVar7;
    undefined2  uVar8;
    undefined2  uVar9;
    undefined2  local_2e;
    undefined2  uStack44;
    uint        local_2a;
    uint        uStack40;
    undefined4 *puStack34;
    uint        uStack30;
    uint        uStack28;
    ulong      *puStack26;
    undefined4  uStack22;
    uint        uStack18;
    uint        uStack16;
    ulong       uStack14;
    ulong       uStack10;
    undefined4  uStack6;

    pass1_1038_56ba(param_1);
    pass1_1038_57c0(param_1);
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if(*(long *)(iVar6 + 0xc) == 0x0)
    {
        param_2 = 0x0;
        uVar5   = 0x0;
    }
    else
    {
        uVar1   = *(undefined4 *)(iVar6 + 0xc);
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
        (**ppcVar2)(param_3, (int)uVar1, (int)((ulong)uVar1 >> 0x10));
        uVar5 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar5, param_2);
    for(uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1)
    {
        uVar1   = *(undefined4 *)(iVar6 + 0xc);
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x4);
        uVar4   = uStack10;
        (**ppcVar2)(param_3, (char)uVar1, (int)((ulong)uVar1 >> 0x10), (int)uStack14, (int)(uStack14 >> 0x10));
        uVar3    = (uint)uVar4;
        uVar5    = extraout_DX_02 | uVar3;
        uStack18 = uVar3;
        uStack16 = extraout_DX_02;
        if(uVar5 != 0x0)
        {
            param_3 = (ushort)&USHORT_1050_1028;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_02);
            uStack22  = CONCAT22(uVar5, uVar3);
            puStack26 = *(ulong **)(uVar3 + 0x22);
            if((*(uint *)(uVar3 + 0x24) | (uint)puStack26) == 0x0)
            {
                uStack28 = 0x0;
            }
            else
            {
                uStack28 = *(uint *)((uint)puStack26 + 0x4);
            }
            for(uStack30 = 0x0; uStack30 < uStack28; uStack30 = uStack30 + 0x1)
            {
                param_3 = 0x1020;
                pass1_1020_bb16(puStack26,
                                (ulong *)CONCAT13((char)(param_4 >> 0x8), CONCAT12((char)param_4, &local_2e)),
                                (ushort *)CONCAT22(param_4, &local_2a),
                                uStack30);
                if(CONCAT22(uStack44, local_2e) != 0x0)
                {
                    pass1_1038_5694(param_1, CONCAT22(uStack44, local_2e), local_2a);
                }
            }
            uVar9     = (undefined2)((ulong)uStack22 >> 0x10);
            puStack34 = (undefined4 *)*(undefined4 *)((int)uStack22 + 0x1e);
            uVar5     = *(uint *)((int)uStack22 + 0x20);
            uVar3     = uVar5 | (uint)puStack34;
            if(uVar3 == 0x0)
            {
                uVar3 = 0x0;
            }
            else
            {
                ppcVar2 = (code **)((int)*puStack34 + 0x10);
                (**ppcVar2)(param_3, (uint)puStack34, uVar5);
                uVar5 = extraout_DX_00;
            }
            uStack28 = uVar3;
            for(uStack30 = 0x0; uStack30 < uStack28; uStack30 = uStack30 + 0x1)
            {
                ppcVar2 = (code **)((int)*puStack34 + 0x4);
                uVar3   = uStack28;
                (**ppcVar2)(param_3, (char)puStack34, (int)((ulong)puStack34 >> 0x10), uStack30, 0x0);
                uVar5    = extraout_DX_01 | uVar3;
                local_2a = uVar3;
                uStack40 = extraout_DX_01;
                if(uVar5 != 0x0)
                {
                    param_3 = (ushort)&USHORT_1050_1028;
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_01);
                    iVar7                            = *(int *)(uVar3 + 0xc) * 0x4;
                    *(long *)(iVar6 + iVar7 + 0x14e) = *(long *)(iVar6 + 0x14e + iVar7) + 0x1;
                }
            }
        }
    }
    uVar4 = uStack10;
    pass1_1030_38f2(*(ulong *)(iVar6 + 0x1f6), 0x3, param_4);
    uVar3         = (uint)uVar4;
    uStack6._0_2_ = uVar3;
    uStack6._2_2_ = uVar5;
    pass1_1030_38f2(*(ulong *)(iVar6 + 0x1f6), 0x4, param_4);
    uStack6 = CONCAT22(uStack6._2_2_ + uVar5 + (uint)CARRY2((uint)uStack6, uVar3), (uint)uStack6 + uVar3);
    if(uStack6 == 0x0)
    {
        pass1_1030_38f2(*(ulong *)(iVar6 + 0x1f6), 0x2, param_4);
        uStack6 = CONCAT22(uVar5, uVar3);
    }
    uVar1   = *(undefined4 *)(iVar6 + 0x1f6);
    uStack6 = uStack6 + *(long *)((int)uVar1 + 0x170);
    pass1_1038_5694(param_1, uStack6, 0x24);
    return;
}


ulong __stdcall16far pass1_1038_565e(ushort param_1, uchar *param_2, ulong param_3)

{
    int        iVar1;
    undefined2 uVar2;
    ulong      uVar3;
    undefined  local_4[0x2];

    uVar2 = (undefined2)(param_3 >> 0x10);
    iVar1 = (int)param_3;
    uVar3 = pass1_1030_8e3c(param_1, (uint)local_4, param_2, CONCAT22(param_1, local_4), *(ulong *)(iVar1 + 0x4));
    pass1_1038_582c(param_3, uVar3);
    return CONCAT22(*(undefined2 *)(iVar1 + 0x16), *(undefined2 *)(iVar1 + 0x14));
}


void __stdcall16far pass1_1038_5694(ulong param_1, long param_2, int param_3)

{
    undefined2 uVar1;

    uVar1                                          = (undefined2)(param_1 >> 0x10);
    *(long *)((int)param_1 + param_3 * 0x4 + 0x26) = *(long *)((int)param_1 + 0x26 + param_3 * 0x4) + param_2;
    return;
}


void __stdcall16far pass1_1038_56ba(ulong param_1)

{
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x26)), (WNDCLASS16 *)0x0, 0x94);
    return;
}


void __stdcall16far pass1_1038_56d6(ulong param_1, int param_2)

{
    code     **ppcVar1;
    int        iVar2;
    uint      *puVar3;
    uint       uVar4;
    ulong      uVar5;
    undefined2 extraout_DX;
    undefined2 uVar6;
    uint       extraout_DX_00;
    uint       uVar7;
    undefined2 uVar8;
    undefined2 uVar9;
    ulong      uStack10;
    ulong      uStack6;

    iVar2  = (int)param_1;
    uVar9  = 0x1000;
    puVar3 = pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)(iVar2 + 0xba)), (WNDCLASS16 *)0x0, 0x94);
    if(param_2 != 0x0)
    {
        uVar8 = (undefined2)(param_1 >> 0x10);
        if(*(long *)(iVar2 + 0xc) == 0x0)
        {
            puVar3 = (uint *)0x0;
            uVar6  = 0x0;
        }
        else
        {
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xc) + 0x10);
            (**ppcVar1)();
            uVar6 = extraout_DX;
        }
        uStack6 = CONCAT22(uVar6, puVar3);
        for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
        {
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xc) + 0x4);
            uVar5   = uStack6;
            (**ppcVar1)(uVar9, *(undefined4 *)(iVar2 + 0xc));
            uVar4 = (uint)uVar5;
            uVar7 = extraout_DX_00 | uVar4;
            if(uVar7 != 0x0)
            {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
                uVar9 = 0x1030;
                pass1_1030_72d0(CONCAT22(uVar7, uVar4));
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_5770(ulong param_1, long param_2, int param_3)

{
    undefined2 uVar1;

    uVar1                                          = (undefined2)(param_1 >> 0x10);
    *(long *)((int)param_1 + param_3 * 0x4 + 0xba) = *(long *)((int)param_1 + 0xba + param_3 * 0x4) + param_2;
    return;
}


void __stdcall16far pass1_1038_5798(ulong param_1, long param_2, int param_3)

{
    undefined2 uVar1;

    uVar1                                           = (undefined2)(param_1 >> 0x10);
    *(long *)((int)param_1 + param_3 * 0x4 + 0x14e) = *(long *)((int)param_1 + 0x14e + param_3 * 0x4) + param_2;
    return;
}


void __stdcall16far pass1_1038_57c0(ulong param_1)

{
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x14e)), (WNDCLASS16 *)0x0, 0x54);
    return;
}


void __stdcall16far pass1_1038_57dc(ulong param_1, long param_2, int param_3)

{
    undefined2 uVar1;

    uVar1                                           = (undefined2)(param_1 >> 0x10);
    *(long *)((int)param_1 + param_3 * 0x4 + 0x1a2) = *(long *)((int)param_1 + 0x1a2 + param_3 * 0x4) + param_2;
    return;
}


void __stdcall16far pass1_1038_5804(ulong param_1, long param_2, int param_3)

{
    ushort uVar1;

    uVar1                                           = (ushort)(param_1 >> 0x10);
    *(long *)((int)param_1 + param_3 * 0x4 + 0x1a2) = *(long *)((int)param_1 + 0x1a2 + param_3 * 0x4) - param_2;
    return;
}


void __stdcall16far
pass1_1038_5cc6(ulong param_1, ulong param_2, ulong param_3, ulong param_4, int param_5, uint param_6)

{
    ulong      uVar1;
    undefined4 uVar2;
    ushort     uVar3;
    uint       uVar4;
    undefined2 uVar5;
    ushort     unaff_SS;
    ushort    *puVar6;
    int        local_14;
    int        local_12;
    int        local_10;
    ulong      uStack14;
    int        local_a;
    int        iStack8;
    int        iStack4;

    puVar6 = pass1_1008_3e38((ushort *)CONCAT22(unaff_SS, &local_a));
    uVar4  = (uint)((ulong)puVar6 >> 0x10);
    do
    {
        iStack4 = 0x0;
        for(uStack14 = 0x0; uStack14 < param_2; uStack14 = uStack14 + 0x1)
        {
            uVar5 = (undefined2)(param_4 >> 0x10);
            if(*(long *)((int)uStack14 * 0x4 + (int)param_4) != 0x0)
            {
                uVar1 = *(ulong *)((int)uStack14 * 0x4 + (int)param_4);
                pass1_1008_3f62((ushort *)CONCAT22(unaff_SS, &local_a),
                                (ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0xc)));
                pass1_1008_3eb4((ushort *)CONCAT22(unaff_SS, &local_a),
                                (ushort *)CONCAT22(unaff_SS, &local_14),
                                (ushort *)CONCAT22(unaff_SS, &local_12),
                                (ushort *)CONCAT22(unaff_SS, &local_10));
                if(local_14 == param_5)
                {
                    uVar5 = (undefined2)(param_3 >> 0x10);
                    if((*(long *)((int)uStack14 * 0x4 + (int)param_3) != 0x0)
                       && (uVar2 = *(undefined4 *)((int)uStack14 * 0x4 + (int)param_3),
                           (*(uint *)((int)uVar2 + 0x1a) & param_6) != 0x0))
                    {
                        iStack8 = local_12 + -0x1;
                        uVar3   = pass1_1038_5be8(param_1,
                                                param_6,
                                                0x7b,
                                                (ushort *)CONCAT22(unaff_SS, &local_a),
                                                (uint)&local_a,
                                                uVar4,
                                                unaff_SS);
                        if(uVar3 != 0x0)
                        {
                            iStack4 = 0x1;
                        }
                        iStack8 = local_12 + 0x1;
                        uVar3   = pass1_1038_5be8(param_1,
                                                param_6,
                                                0x7b,
                                                (ushort *)CONCAT22(unaff_SS, &local_a),
                                                (uint)&local_a,
                                                uVar4,
                                                unaff_SS);
                        if(uVar3 != 0x0)
                        {
                            iStack4 = 0x1;
                        }
                        iStack8 = local_12;
                        local_a = local_10 + -0x1;
                        uVar3   = pass1_1038_5be8(param_1,
                                                param_6,
                                                0x7c,
                                                (ushort *)CONCAT22(unaff_SS, &local_a),
                                                (uint)&local_a,
                                                uVar4,
                                                unaff_SS);
                        if(uVar3 != 0x0)
                        {
                            iStack4 = 0x1;
                        }
                        local_a = local_10 + 0x1;
                        uVar3   = pass1_1038_5be8(param_1,
                                                param_6,
                                                0x7c,
                                                (ushort *)CONCAT22(unaff_SS, &local_a),
                                                (uint)&local_a,
                                                uVar4,
                                                unaff_SS);
                        if(uVar3 != 0x0)
                        {
                            iStack4 = 0x1;
                        }
                    }
                }
            }
        }
    } while(iStack4 != 0x0);
    return;
}


void __stdcall16far
pass1_1038_43cc(int param_1, undefined2 param_2, uint param_3, uint param_4, int param_5, int param_6)

{
    code      **ppcVar1;
    uint        uVar2;
    uint        uVar3;
    uint        uVar4;
    ulong       uVar5;
    uchar      *puVar6;
    undefined2  extraout_DX;
    undefined2  uVar7;
    int         iVar8;
    int         iVar9;
    undefined2  uVar10;
    ulong      *puVar11;
    ulong       uVar12;
    ulong       uStack22;
    ulong       uStack18;
    undefined4 *puStack14;

    if(param_4 == 0x5)
    {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_4);
    if((param_6 != 0x0) || (param_5 != 0x0))
    {
        iVar8 = param_4 * 0x4;
        uVar2 = *(uint *)(param_1 + iVar8 + 0x14e);
        iVar9 = (*(int *)(param_1 + iVar8 + 0x150) - ((int)param_3 >> 0xf)) - (uint)(uVar2 < param_3);
        *(int *)(param_1 + iVar8 + 0x14e) = uVar2 - param_3;
        *(int *)(param_1 + iVar8 + 0x150) = iVar9;
        if(iVar9 < 0x0)
        {
            *(undefined4 *)(param_1 + iVar8 + 0x14e) = 0x0;
        }
        uVar10  = 0x1008;
        puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        puVar6  = (uchar *)((ulong)puVar11 >> 0x10);
        uVar2   = (uint)puVar11;
        pass1_1038_4e78(uVar2, puVar6, CONCAT22(param_2, param_1), puVar11);
        puStack14 = (undefined4 *)CONCAT22(puVar6, uVar2);
        ppcVar1   = (code **)((int)*puStack14 + 0x10);
        uVar3     = uVar2;
        (**ppcVar1)(0x1008, uVar2, puVar6);
        uStack18 = CONCAT22(extraout_DX, uVar3);
        uVar7    = extraout_DX;
        for(uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1)
        {
            uVar12 = pass1_1030_1d7c(uVar3, uVar7, (ulong)puStack14);
            uVar7  = (undefined2)(uVar12 >> 0x10);
            uVar5  = uVar12 & 0xffff;
            for(; uVar4 = (uint)uVar5, param_3 != 0x0; param_3 = param_3 - 0x1)
            {
                pass1_1030_cf78(uVar12, param_4);
                uVar5 = (ulong)uVar4;
                if(uVar4 == 0x0)
                    break;
            }
            uVar10 = 0x1030;
            if(param_3 == 0x0)
                break;
        }
        if(puStack14 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack14;
            (**ppcVar1)(uVar10, uVar2, (char)puVar6, 0x1);
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1038_44d8(int param_1, ushort param_2, uint param_3, uint param_4, int param_5, int param_6)

{
    code       **ppcVar1;
    uint         uVar2;
    uint         uVar3;
    uint         uVar4;
    ulong        uVar5;
    uchar       *puVar6;
    ushort       extraout_DX;
    ushort       uVar7;
    ushort       uVar8;
    astruct_697 *iVar9;
    int          iVar10;
    undefined2   uVar11;
    ulong       *puVar12;
    ulong        uVar13;
    ulong        uStack22;
    ulong        uStack18;
    undefined4  *puStack14;

    if(param_4 == 0x5)
    {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_4);
    if((param_6 != 0x0) || (param_5 != 0x0))
    {
        iVar9  = (astruct_697 *)(param_4 * 0x4);
        uVar2  = *(uint *)(iVar9 + param_1 + 0x14e);
        iVar10 = (*(int *)(iVar9 + param_1 + 0x150) - ((int)param_3 >> 0xf)) - (uint)(uVar2 < param_3);
        *(uint *)(iVar9 + param_1 + 0x14e) = uVar2 - param_3;
        *(int *)(iVar9 + param_1 + 0x150)  = iVar10;
        if(iVar10 < 0x0)
        {
            *(undefined4 *)(iVar9 + param_1 + 0x14e) = 0x0;
        }
        uVar11  = 0x1008;
        puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        puVar6  = (uchar *)((ulong)puVar12 >> 0x10);
        uVar2   = (uint)puVar12;
        pass1_1038_4e78(uVar2, puVar6, CONCAT22(param_2, param_1), puVar12);
        puStack14 = (undefined4 *)CONCAT22(puVar6, uVar2);
        ppcVar1   = (code **)((int)*puStack14 + 0x10);
        uVar3     = uVar2;
        (**ppcVar1)(0x1008, uVar2, puVar6);
        uStack18 = CONCAT22(extraout_DX, uVar3);
        uVar7    = extraout_DX;
        for(uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1)
        {
            uVar13 = pass1_1030_1d7c(uVar3, uVar7, (ulong)puStack14);
            uVar8  = (ushort)(uVar13 >> 0x10);
            uVar5  = uVar13 & 0xffff;
            uVar7  = uVar8;
            for(; uVar4 = (uint)uVar5, param_3 != 0x0; param_3 = param_3 - 0x1)
            {
                pass1_1030_d00c((int)uVar13, uVar8, param_4);
                uVar5 = (ulong)uVar4;
                if(uVar4 == 0x0)
                    break;
            }
            uVar11 = 0x1030;
            if(param_3 == 0x0)
                break;
        }
        if(puStack14 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack14;
            (**ppcVar1)(uVar11, uVar2, (char)puVar6, 0x1);
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1038_45e4(ulong param_1, uint param_2, int param_3, ushort param_4)

{
    int        *piVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    uint        uVar4;
    uint        uVar5;
    uint        uVar6;
    uint        uVar7;
    int         iVar8;
    int         iVar9;
    int         iVar10;
    uchar      *puVar11;
    int         iVar12;
    uint        uVar13;
    undefined2  uVar14;
    bool        bVar15;
    ulong      *puVar16;
    ushort      uStack28;
    undefined4 *puStack22;

    uVar14 = (undefined2)(param_1 >> 0x10);
    iVar12 = (int)param_1;
    pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6), 0x2, param_4);
    iVar8 = param_3;
    uVar4 = param_2;
    pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6), 0x1, param_4);
    bVar15 = param_2 < uVar4;
    uVar13 = param_2 - uVar4;
    iVar10 = param_3 - iVar8;
    pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6), 0x4, param_4);
    iVar9 = iVar8;
    uVar5 = uVar4;
    pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6), 0x3, param_4);
    uVar7  = *(uint *)(iVar12 + 0x24);
    uVar6  = uVar7 + (uVar4 - uVar5);
    iVar10 = ((int)uVar7 >> 0xf) + ((iVar8 - iVar9) - (uint)(uVar4 < uVar5)) + (uint)CARRY2(uVar7, uVar4 - uVar5)
           + (iVar10 - (uint)bVar15) + (uint)CARRY2(uVar6, uVar13);
    if((iVar10 < 0x0) || ((iVar10 < 0x1 && (uVar6 + uVar13 == 0x0))))
    {
        iVar10 = -0x1;
    }
    else
    {
        iVar10 = 0x1;
    }
    piVar1  = (int *)(iVar12 + 0x24);
    *piVar1 = *piVar1 + iVar10;
    puVar16 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x16);
    puVar11 = (uchar *)((ulong)puVar16 >> 0x10);
    uVar7   = (uint)puVar16;
    pass1_1038_4d6e(param_1, puVar16, uVar7, puVar11);
    puStack22 = (undefined4 *)CONCAT22(puVar11, uVar7);
    uVar3     = *puStack22;
    ppcVar2   = (code **)uVar3 + 0x8;
    uVar5     = uVar7;
    (**ppcVar2)(0x1008, uVar7, puVar11);
    if(puStack22 != (undefined4 *)0x0)
    {
        ppcVar2 = (code **)uVar3;
        (**ppcVar2)(0x1008, uVar7, (char)puVar11, 0x1);
    }
    piVar1  = (int *)(iVar12 + 0x24);
    *piVar1 = *piVar1 + uVar5 * 0x2;
    iVar10  = *(int *)(iVar12 + 0x24);
    if(0x64 < iVar10)
    {
        iVar10 = 0x64;
    }
    *(int *)(iVar12 + 0x24) = iVar10;
    if(iVar10 < 0x0)
    {
        iVar10 = 0x0;
    }
    *(int *)(iVar12 + 0x24) = iVar10;
    iVar10                  = iVar10 / 0xa;
    uStack28                = 0x10;
    if(iVar10 < 0xb)
    {
        uStack28 = 0x14;
    }
    else
    {
        if(iVar10 < 0x15)
        {
            uStack28 = 0x13;
        }
        else
        {
            if(iVar10 < 0x1f)
            {
                uStack28 = 0x12;
            }
            else
            {
                if(iVar10 < 0x29)
                {
                    uStack28 = 0x11;
                }
                else
                {
                    if(iVar10 < 0x33)
                    {
                        uStack28 = 0x10;
                    }
                    else
                    {
                        if(iVar10 < 0x3d)
                        {
                            uStack28 = 0xf;
                        }
                        else
                        {
                            if(iVar10 < 0x47)
                            {
                                uStack28 = 0xe;
                            }
                            else
                            {
                                if(iVar10 < 0x51)
                                {
                                    uStack28 = 0xd;
                                }
                                else
                                {
                                    if(iVar10 < 0x5b)
                                    {
                                        uStack28 = 0xc;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pass1_1030_3258(*(ulong *)(iVar12 + 0x1f6), uStack28);
    return;
}


void __stdcall16far pass1_1038_4760(ulong param_1)

{
    uint        *puVar1;
    code       **ppcVar2;
    uint         uVar3;
    uint         uVar4;
    int          iVar5;
    uint         uVar6;
    uchar       *puVar7;
    uchar       *puVar8;
    uint         extraout_DX;
    undefined2   extraout_DX_00;
    undefined2   extraout_DX_01;
    undefined2   extraout_DX_02;
    undefined2   uVar9;
    undefined2   extraout_DX_03;
    undefined2   extraout_DX_04;
    astruct_700 *iVar10;
    undefined2   uVar10;
    undefined2   uVar11;
    undefined2   unaff_SS;
    ulong       *puVar12;
    ulong        uVar13;
    undefined    uVar14;
    uchar       *puVar15;
    ulong        uStack26;
    ulong        uStack22;
    undefined4  *puStack14;
    undefined4  *puStack10;

    uVar10  = (undefined2)(param_1 >> 0x10);
    iVar10  = (astruct_700 *)param_1;
    puVar1  = &iVar10->field_0x22;
    *puVar1 = *puVar1 + iVar10->field_0x20c;
    puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x26);
    puVar7  = (uchar *)((ulong)puVar12 >> 0x10);
    uVar6   = (uint)puVar12;
    pass1_1038_4d6e(param_1, puVar12, uVar6, puVar7);
    puStack10 = (undefined4 *)CONCAT22(puVar7, uVar6);
    uVar11    = 0x1008;
    puVar12   = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1a);
    puVar8    = (uchar *)((ulong)puVar12 >> 0x10);
    uVar3     = (uint)puVar12;
    pass1_1038_4d6e(param_1, puVar12, uVar3, puVar8);
    puStack14 = (undefined4 *)CONCAT22(puVar8, uVar3);
    ppcVar2   = (code **)((int)*puStack14 + 0x10);
    uVar4     = uVar3;
    (**ppcVar2)(0x1008, uVar3, puVar8);
    uVar14  = (undefined)uVar6;
    puVar15 = puVar7;
    if((extraout_DX | uVar4) == 0x0)
    {
        ppcVar2 = (code **)((int)*puStack10 + 0x10);
        (**ppcVar2)();
        puVar1  = &iVar10->field_0x22;
        *puVar1 = *puVar1 + uVar4;
        uVar9   = extraout_DX_00;
    }
    else
    {
        ppcVar2 = (code **)((int)*puStack10 + 0x10);
        (**ppcVar2)();
        uStack22 = CONCAT22(extraout_DX_03, uVar4);
        uVar9    = extraout_DX_03;
        for(uStack26 = 0x0; uStack26 < uStack22; uStack26 = uStack26 + 0x1)
        {
            uVar13 = pass1_1030_1d7c(uVar4, uVar9, (ulong)puStack10);
            iVar5  = (int)uVar13;
            uVar11 = SUB42(&USHORT_1050_1028, 0x0);
            func_0x10285a94();
            if(iVar5 == 0x2)
            {
                if((*_PTR_LOOP_1050_65e2 & 0x1) == 0x0)
                    goto LAB_1038_485e;
            }
            else
            {
                if(iVar5 != 0x3)
                {
                LAB_1038_485e:
                    puVar1  = &iVar10->field_0x22;
                    *puVar1 = *puVar1 + 0x1;
                }
            }
            uVar9 = extraout_DX_04;
        }
    }
    if(puStack10 != (undefined4 *)0x0)
    {
        ppcVar2 = (code **)*puStack10;
        (**ppcVar2)(uVar11, uVar6, puVar7, 0x1, uVar14, puVar15);
        uVar9 = extraout_DX_01;
    }
    if(puStack14 != (undefined4 *)0x0)
    {
        ppcVar2 = (code **)*puStack14;
        (**ppcVar2)(uVar11, uVar3, puVar8, 0x1);
        uVar9 = extraout_DX_02;
    }
    pass1_1038_45e4(param_1, (int)puStack14, uVar9, unaff_SS);
    if(0x32 < iVar10->field_0x24)
    {
        puVar1  = &iVar10->field_0x22;
        *puVar1 = *puVar1 - 0x1;
    }
    if(iVar10->field_0x24 < 0x32)
    {
        puVar1  = &iVar10->field_0x22;
        *puVar1 = *puVar1 + 0x1;
    }
    if(iVar10->field_0x18 < 0xfa)
    {
        puVar1  = &iVar10->field_0x22;
        *puVar1 = *puVar1 + 0x2;
    }
    else
    {
        if(iVar10->field_0x18 < 0x1c2)
        {
            puVar1  = &iVar10->field_0x22;
            *puVar1 = *puVar1 + 0x1;
        }
        else
        {
            if(0x225 < iVar10->field_0x18)
            {
                if(iVar10->field_0x18 < 0x2ee)
                {
                    puVar1  = &iVar10->field_0x22;
                    *puVar1 = *puVar1 - 0x1;
                }
                else
                {
                    puVar1  = &iVar10->field_0x22;
                    *puVar1 = *puVar1 - 0x2;
                }
            }
        }
    }
    uVar6 = iVar10->field_0x22;
    if(0x64 < (int)uVar6)
    {
        uVar6 = 0x64;
    }
    iVar10->field_0x22 = uVar6;
    if((int)uVar6 < 0x0)
    {
        uVar6 = 0x0;
    }
    iVar10->field_0x22 = uVar6;
    return;
}


void __stdcall16far pass1_1038_48e0(ulong param_1, int param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = *(int *)((int)param_1 + 0x20e) + param_2;
    if(0xa < iVar1)
    {
        iVar1 = 0xa;
    }
    *(int *)((int)param_1 + 0x20e) = iVar1;
    return;
}


void __stdcall16far pass1_1038_4900(ulong param_1)

{
    int       *piVar1;
    undefined2 uVar2;

    uVar2   = (undefined2)(param_1 >> 0x10);
    piVar1  = (int *)((int)param_1 + 0x20e);
    *piVar1 = *piVar1 + -0x1;
    if(*piVar1 < 0x0)
    {
        *(undefined2 *)((int)param_1 + 0x20e) = 0x0;
    }
    return;
}


void __stdcall16far pass1_1038_4b20(ulong param_1, ulong param_2, ulong param_3, uint param_4)

{
    ulong uVar1;

    uVar1 = *(ulong *)((int)param_1 + 0xc);
    pass1_1020_c4f4(uVar1, (ushort)param_2, (ushort)(param_2 >> 0x10), param_3, (astruct_361 *)uVar1, param_4);
    return;
}


void __stdcall16far pass1_1038_4c1a(ulong param_1, ushort param_2, ulong param_3, ushort param_4)

{
    code     **ppcVar1;
    uint       uVar2;
    ushort     uVar3;
    ulong      uVar4;
    uint       uVar5;
    int        iVar6;
    undefined2 uVar7;
    undefined4 uVar8;
    ulong      uStack14;
    ulong      uStack10;

    uVar7   = (undefined2)(param_1 >> 0x10);
    iVar6   = (int)param_1;
    uVar8   = *(undefined4 *)(iVar6 + 0xc);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uStack10 = CONCAT22((int)param_3, param_2);
    for(uStack14 = 0x0; uVar5 = (uint)param_3, uStack14 < uStack10; uStack14 = uStack14 + 0x1)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x4);
        uVar4   = uStack10;
        (**ppcVar1)(param_4, *(undefined4 *)(iVar6 + 0xc), uStack14, uVar8);
        uVar2   = (uint)uVar4;
        param_3 = (ulong)(uVar5 | uVar2);
        if((uVar5 | uVar2) != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, uVar5);
            uVar3   = pass1_1030_6fa0(CONCAT22((int)param_3, uVar2));
            param_4 = 0x1008;
            pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar3, 0xe);
        }
    }
    return;
}


void __stdcall16far pass1_1038_4cba(void)

{
    pass1_1030_38b8();
    return;
}


char *__stdcall16far pass1_1038_4d28(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return (char *)CONCAT22(*(undefined2 *)((int)param_1 + 0x1fc), *(undefined2 *)((int)param_1 + 0x1fa));
}


void __stdcall16far pass1_1038_4f54(ulong param_1, ushort param_2, ushort param_3)

{
    code     **ppcVar1;
    ushort     uVar2;
    BOOL16     BVar3;
    ulong      uVar4;
    uint       extraout_DX;
    uint       uVar5;
    uint       uVar6;
    int        iVar7;
    undefined2 uVar8;
    ulong      uStack10;
    ulong      uStack6;

    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar7 = (int)param_1;
    if(*(long *)(iVar7 + 0xc) == 0x0)
    {
        param_3 = 0x0;
        uVar5   = 0x0;
    }
    else
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
        (**ppcVar1)();
        uVar5 = extraout_DX;
    }
    uStack6  = CONCAT22(uVar5, param_3);
    uStack10 = 0x0;
    do
    {
        if(uStack6 <= uStack10)
        {
            return;
        }
        uVar4 = uStack6;
        pass1_1030_1d58(*(ulong *)(iVar7 + 0xc));
        uVar6 = uVar5 | (uint)uVar4;
        if(uVar6 != 0x0)
        {
            uVar2 = pass1_1030_6fa0(uVar4 & 0xffff | (ulong)uVar5 << 0x10);
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, param_2);
            if(BVar3 != 0x0)
            {
                return;
            }
        }
        uStack10 = uStack10 + 0x1;
        uVar5    = uVar6;
    } while(true);
}


void __stdcall16far pass1_1038_4fd8(ushort param_1, ulong param_2, ushort param_3)

{
    code     **ppcVar1;
    ushort     uVar2;
    ulong      uVar3;
    uint       extraout_DX;
    uint       uVar4;
    uint       uVar5;
    int        iVar6;
    undefined2 uVar7;
    ulong      uStack10;
    ulong      uStack6;

    uVar7 = (undefined2)(param_2 >> 0x10);
    iVar6 = (int)param_2;
    if(*(long *)(iVar6 + 0xc) == 0x0)
    {
        param_1 = 0x0;
        uVar4   = 0x0;
    }
    else
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    uStack6  = CONCAT22(uVar4, param_1);
    uStack10 = 0x0;
    do
    {
        if(uStack6 <= uStack10)
        {
            return;
        }
        uVar3 = uStack6;
        pass1_1030_1d58(*(ulong *)(iVar6 + 0xc));
        uVar5 = uVar4 | (uint)uVar3;
        if(uVar5 != 0x0)
        {
            uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | (ulong)uVar4 << 0x10);
            if(uVar2 == param_3)
            {
                return;
            }
        }
        uStack10 = uStack10 + 0x1;
        uVar4    = uVar5;
    } while(true);
}


void __stdcall16far pass1_1038_5050(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    code     **ppcVar1;
    ushort     uVar2;
    ulong      uVar3;
    uint       extraout_DX;
    uint       uVar4;
    uint       uVar5;
    int        iVar6;
    undefined2 uVar7;
    ulong      uStack14;
    ulong      uStack10;

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if(*(long *)(iVar6 + 0xc) == 0x0)
    {
        param_3 = 0x0;
        uVar4   = 0x0;
    }
    else
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar4, param_3);
    for(uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1)
    {
        uVar3 = uStack10;
        pass1_1030_1d58(*(ulong *)(iVar6 + 0xc));
        uVar5 = uVar4 | (uint)uVar3;
        if(uVar5 != 0x0)
        {
            uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | (ulong)uVar4 << 0x10);
            pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, param_2);
        }
        uVar4 = uVar5;
    }
    return;
}


void __stdcall16far pass1_1038_349e(ulong param_1, ulong param_2)

{
    code       **ppcVar1;
    uint         uVar2;
    undefined2   uVar3;
    uint         extraout_DX;
    uint         uVar4;
    uint         uVar5;
    uint         extraout_DX_00;
    astruct_685 *iVar7;
    undefined2   uVar6;
    undefined4  *puVar7;
    undefined2   uVar8;
    undefined2   uVar9;
    ulong        uStack10;
    ulong        uStack6;

    uVar6              = (undefined2)(param_1 >> 0x10);
    iVar7              = (astruct_685 *)param_1;
    iVar7->field_0x200 = param_2;
    pass1_1038_4d0e(param_1, 0x258);
    uVar3 = (undefined2)param_2;
    pass1_1038_4d0e(param_1, 0x258);
    iVar7->field_0x204 = 0x0;
    iVar7->field_0x206 = 0x0;
    puVar7             = iVar7->field_0xc;
    uVar8              = SUB42(puVar7, 0x0);
    uVar9              = (undefined2)((ulong)puVar7 >> 0x10);
    ppcVar1            = (code **)((int)*iVar7->field_0xc + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX, uVar3);
    uVar5   = extraout_DX;
    for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
    {
        puVar7 = (undefined4 *)pass1_1030_1d7c(uVar3, uVar5, (ulong)iVar7->field_0xc);
        uVar4  = (uint)((ulong)puVar7 >> 0x10);
        uVar2  = (uint)puVar7;
        uVar5  = uVar4 | uVar2;
        if(uVar5 != 0x0)
        {
            ppcVar1 = (code **)((int)*puVar7 + 0x58);
            (**ppcVar1)(0x1030, uVar2, uVar4, (char)param_1, uVar6, uVar8, uVar9);
            *(undefined4 *)(uVar2 + 0x1c) = 0x0;
            uVar5                         = extraout_DX_00;
        }
    }
    return;
}
