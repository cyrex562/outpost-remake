
ushort *__stdcall16far struct_1030_be34(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0xc006;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
    return param_1;
}


void __stdcall16far struct_1030_c06e(ushort *param_1)

{
    astruct_188 *iVar1;
    undefined2   uVar1;

    struct_1028_b354(param_1);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_188 *)param_1;
    iVar1->field_0x20 = 0x0;
    iVar1->field_0x24 = 0x0;
    *param_1          = 0xc68e;
    iVar1->field_0x2  = 0x1030;
    return;
}


ushort *__stdcall16far struct_1030_c6f6(ushort *param_1)

{
    undefined2 uVar1;

    struct_1028_b354(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x20) = 0x0;
    *param_1                             = 0xc940;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1030;
    return param_1;
}


ushort *__stdcall16far struct_1030_c9a8(ushort *param_1)

{
    int        iVar1;
    undefined2 uVar2;

    struct_1028_b354(param_1);
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined2 *)(iVar1 + 0x98) = 0x1;
    *param_1                      = 0xd88e;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1030;
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x20)), (WNDCLASS16 *)0x0, 0x78);
    return param_1;
}


BOOL16 __stdcall16far pass1_1030_acbe(
  ushort param_1, ushort param_2, ushort *param_3, long param_4, uint param_5, uint param_6, ushort param_7)

{
    int   iVar1;
    uint  uVar2;
    ulong uVar3;

    pass1_1030_627e(param_7, param_5, param_6, _PTR_LOOP_1050_5740, param_3, param_4);
    uVar2 = param_6 | param_5;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, param_6);
        if((uVar2 | param_5) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_5));
            if((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9))))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1030_afa6(ushort *param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    undefined4   uVar4;
    astruct_614 *iVar5;
    undefined2   uVar5;

    uVar5            = (undefined2)((ulong)param_1 >> 0x10);
    iVar5            = (astruct_614 *)param_1;
    *param_1         = 0xb932;
    iVar5->field_0x2 = 0x1030;
    if(*(long *)&iVar5->field_0x10 != 0x0)
    {
        uVar4                             = *(undefined4 *)&iVar5->field_0x10;
        *(undefined2 *)((int)uVar4 + 0xa) = 0x1;
    }
    puVar1 = (undefined4 *)*(uint *)&iVar5->field_0x10;
    uVar2  = iVar5->field_0x12;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *param_1         = 0x389a;
    iVar5->field_0x2 = 0x1008;
    return;
}


void __stdcall16far pass1_1030_affc(ulong param_1, int param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;
    uint       uVar3;
    BOOL16     BVar4;
    uint       uVar5;
    uint       uVar6;
    ulong      uVar7;
    ulong      uVar8;
    int        iStack12;
    undefined4 uStack10;
    undefined4 local_6;

    uVar8 = ZEXT24(&local_6);
    pass1_1030_b718((ushort)param_1,
                    param_1._2_2_,
                    (ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                    (ulong *)CONCAT22(param_3, &local_6),
                    (uchar *)param_1._2_2_,
                    param_2,
                    param_3);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)local_6, (uint)((ulong)local_6 >> 0x10));
    uStack10 = uVar8 & 0xffff | (ulong)param_1._2_2_ << 0x10;
    uVar5    = param_1._2_2_ | (uint)uVar8;
    if(uVar5 != 0x0)
    {
        uVar7 = struct_op_1030_73a8(uVar8 & 0xffff | (ulong)param_1._2_2_ << 0x10);
        uVar5 = (uint)(uVar7 >> 0x10);
        iVar1 = *(int *)((int)uVar7 + 0xc);
        uVar8 = (ulong)(iVar1 - 0x16U);
        if((0x15 < iVar1) && (!SBORROW2(iVar1, 0x16)))
        {
            uVar8 = (ulong)(iVar1 - 0x17U);
            if(iVar1 - 0x17U != 0x0 && 0x0 < (int)(iVar1 - 0x16U))
            {
                uVar8 = (ulong)(iVar1 - 0x19U);
                if((iVar1 + -0x18 < 0x1)
                   || (uVar8 = (ulong)(iVar1 - 0x1aU), iVar1 - 0x1aU != 0x0 && 0x0 < (int)(iVar1 - 0x19U)))
                    goto LAB_1030_b064;
            }
            *(undefined2 *)((int)uVar7 + 0x20) = 0x0;
        }
    }
LAB_1030_b064:
    iStack12 = 0x6;
    do
    {
        uVar3 = (uint)uVar8;
        if(iStack12 == 0x0)
        {
        LAB_1030_b0fc:
            if((uStack10._2_2_ | (uint)uStack10) != 0x0)
            {
                uVar8 = struct_op_1030_73a8(uStack10);
                uVar2 = (undefined2)(uVar8 >> 0x10);
                iVar1 = *(int *)((int)uVar8 + 0xc);
                if(((0x15 < iVar1) && (!SBORROW2(iVar1, 0x16)))
                   && ((iVar1 == 0x17 || iVar1 + -0x16 < 0x1 || ((0x0 < iVar1 + -0x18 && (iVar1 + -0x19 < 0x2))))))
                {
                    *(undefined2 *)((int)uVar8 + 0x20) = 0x1;
                }
            }
            return;
        }
        pass1_1030_b578(param_1, param_2, param_3);
        if((uVar5 | uVar3) == 0x0)
            goto LAB_1030_b0fc;
        uStack10 = CONCAT22(uVar5, uVar3);
        uVar8    = struct_op_1030_73a8(CONCAT22(uVar5, uVar3));
        uVar6    = (uint)(uVar8 >> 0x10);
        iVar1    = *(int *)((int)uVar8 + 0xc);
        pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                        (ushort *)CONCAT22(uVar5, uVar3 + 0xc));
        if((iVar1 == 0x18) || (uVar5 = uVar6, iVar1 == 0x3f))
        {
            pass1_1030_b142(param_1, uStack10);
            uVar5 = uVar6;
        }
        BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x40);
        uVar8 = (ulong)BVar4;
        if(BVar4 != 0x0)
        {
            pass1_1030_b454(param_1, uStack10, param_3);
            goto LAB_1030_b0fc;
        }
        iStack12 = iStack12 + -0x1;
    } while(true);
}


void __stdcall16far pass1_1030_b2aa(ulong param_1, ushort *param_2, uchar *param_3, int param_4, ushort param_5)

{
    uint       uVar1;
    BOOL16     BVar2;
    ulong      uVar3;
    byte       bStack23;
    undefined4 local_6;

    pass1_1030_b718((ushort)param_1,
                    (ushort)(param_1 >> 0x10),
                    param_2,
                    (ulong *)CONCAT22(param_5, &local_6),
                    param_3,
                    param_4,
                    param_5);
    bStack23 = (byte)((ulong)local_6 >> 0x18);
    uVar1    = (uint)bStack23;
    if(bStack23 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)local_6, local_6._2_2_);
        if((local_6._2_2_ | uVar1) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(local_6._2_2_, uVar1));
            BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((int)uVar3 + 0xc), 0x42);
            if(BVar2 != 0x0)
            {
                pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                                (ushort *)CONCAT22(local_6._2_2_, uVar1 + 0xc));
                return;
            }
        }
    }
    return;
}


ulong __stdcall16far pass1_1030_b344(ulong param_1, ushort param_2)

{
    uchar      *puVar1;
    undefined4 *puStack18;
    uchar      *puStack16;
    undefined   local_e[0x2];
    int         local_c;
    int         local_a;
    undefined4  local_8;
    undefined2  uStack4;

    local_8 = *(ulong *)((int)param_1 + 0x8);
    uStack4 = *(undefined2 *)((int)param_1 + 0xc);
    puVar1  = param_1._2_2_;
    pass1_1008_3eb4((ushort *)CONCAT22(param_2, &local_8),
                    (ushort *)CONCAT22(param_2, local_e),
                    (ushort *)CONCAT22(param_2, &local_c),
                    (ushort *)CONCAT22(param_2, &local_a));
    local_8   = local_8 & 0xffff | (ulong)(local_c - 0x1) << 0x10;
    puStack18 = &local_8;
    pass1_1030_b2aa(param_1, (ushort *)CONCAT22(param_2, puStack18), puVar1, (int)&stack0xfffe, param_2);
    puStack16 = (uchar *)((uint)puVar1 | (uint)puStack18);
    if(puStack16 == (uchar *)0x0)
    {
        local_8   = local_8 & 0xffff | (ulong)(local_c + 0x1) << 0x10;
        puStack18 = &local_8;
        pass1_1030_b2aa(param_1, (ushort *)CONCAT22(param_2, puStack18), (uchar *)0x0, (int)&stack0xfffe, param_2);
        puVar1 = (uchar *)((uint)puStack16 | (uint)puStack18);
        if(puVar1 == (uchar *)0x0)
        {
            local_8._0_2_ = local_a + -0x1;
            local_8._2_2_ = local_c;
            puStack18     = &local_8;
            pass1_1030_b2aa(param_1, (ushort *)CONCAT22(param_2, puStack18), (uchar *)0x0, (int)&stack0xfffe, param_2);
            puStack16 = (uchar *)((uint)puVar1 | (uint)puStack18);
            if(puStack16 == (uchar *)0x0)
            {
                local_8   = CONCAT22(local_8._2_2_, local_a + 0x1);
                puStack18 = &local_8;
                pass1_1030_b2aa(
                  param_1, (ushort *)CONCAT22(param_2, puStack18), (uchar *)0x0, (int)&stack0xfffe, param_2);
                if(((uint)puStack16 | (uint)puStack18) == 0x0)
                {
                    return 0x0;
                }
                *(undefined2 *)((int)param_1 + 0xe) = 0x2;
            }
            else
            {
                *(undefined2 *)((int)param_1 + 0xe) = 0x4;
                puStack16                           = puVar1;
            }
        }
        else
        {
            *(undefined2 *)((int)param_1 + 0xe) = 0x3;
        }
    }
    else
    {
        *(undefined2 *)((int)param_1 + 0xe) = 0x1;
        puStack16                           = puVar1;
    }
    return CONCAT22(puStack16, puStack18);
}


void __stdcall16far pass1_1030_b454(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    undefined  *puVar3;
    uint        extraout_DX;
    int         iVar4;
    uint        extraout_DX_00;
    uint        uVar5;
    int         iVar6;
    undefined2  uVar7;
    ulong       uVar8;
    undefined4  uVar9;
    long        lStack38;
    undefined4  uStack30;
    undefined   local_12[0x4];
    undefined4  uStack14;
    ulong       uStack10;
    long        lStack6;

    lStack6 = *(long *)((int)param_2 + 0x4);
    uVar7   = (undefined2)(param_1 >> 0x10);
    iVar6   = (int)param_1;
    pass1_1008_5784((ulong *)CONCAT22(param_3, local_12), *(ulong *)(iVar6 + 0x10));
    while(true)
    {
        puVar3 = local_12;
        pass1_1008_5b12(puVar3, param_3);
        uStack10 = CONCAT22(extraout_DX, puVar3);
        if((extraout_DX | (uint)puVar3) == 0x0)
            break;
        if(*(long *)(puVar3 + 0x20) == lStack6)
        {
            ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x10) + 0xc);
            (**ppcVar2)();
            uStack14 = 0x0;
            pass1_1038_69fe(uStack10);
        }
    }
    uVar8  = struct_op_1030_73a8(param_2);
    iVar4  = (int)(uVar8 >> 0x10);
    puVar1 = (undefined4 *)*(ulong *)((int)uVar8 + 0x20);
    puVar3 = local_12;
    pass1_1008_5784((ulong *)CONCAT22(param_3, puVar3), (ulong)puVar1);
    pass1_1030_b13c();
    uStack30
      = CONCAT22(-(uint)((undefined *)((int)s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2aU) < puVar3) - iVar4,
                 0x1f4 - (int)puVar3);
    do
    {
        puVar3 = local_12;
        pass1_1008_5b12(puVar3, param_3);
        uStack10 = CONCAT22(extraout_DX_00, puVar3);
        uVar5    = extraout_DX_00 | (uint)puVar3;
        if(uVar5 == 0x0)
        {
            return;
        }
        pass1_1038_6984(CONCAT22(extraout_DX_00, puVar3));
        lStack38 = CONCAT22(uVar5, puVar3);
        if(((int)uVar5 <= uStack30._2_2_) && (((int)uVar5 < uStack30._2_2_ || (puVar3 <= (undefined *)uStack30))))
        {
            uVar9   = *(undefined4 *)(iVar6 + 0x10);
            ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x10) + 0x8);
            (**ppcVar2)();
            uStack30 = uStack30 - lStack38;
            ppcVar2  = (code **)((int)*puVar1 + 0xc);
            (**ppcVar2)((int)&PTR_LOOP_1050_1038, (int)puVar1, (int)((ulong)puVar1 >> 0x10), uStack10, uVar9);
            uStack14 = 0x0;
        }
    } while(0x0 < uStack30);
    return;
}


void __stdcall16far pass1_1030_b578(ulong param_1, int param_2, ushort param_3)

{
    int        iVar1;
    ulong     *puVar2;
    uint       uVar3;
    uchar     *puVar4;
    bool       bVar5;
    ulong      uVar6;
    undefined4 uStack48;
    undefined  local_1c[0x2];
    int        local_1a;
    int        local_18;
    ulong      local_16;
    undefined2 uStack18;
    undefined2 uStack16;
    ulong      uStack14;
    uint       uStack10;
    uint       uStack8;
    undefined4 local_6;

    pass1_1030_b718((ushort)param_1,
                    (ushort)param_1._2_2_,
                    (ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                    (ulong *)CONCAT22(param_3, &local_6),
                    param_1._2_2_,
                    param_2,
                    param_3);
    uStack48._3_1_ = (byte)((ulong)local_6 >> 0x18);
    uStack10       = (uint)uStack48._3_1_;
    if(uStack48._3_1_ == 0x0)
    {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)local_6, local_6._2_2_);
    uStack8  = local_6._2_2_;
    uStack14 = struct_op_1030_73a8(CONCAT22(local_6._2_2_, uStack10));
    uStack16 = *(undefined2 *)((int)uStack14 + 0xc);
    local_16 = *(ulong *)((ushort)param_1 + 0x8);
    uStack18 = *(undefined2 *)((ushort)param_1 + 0xc);
    puVar4   = param_1._2_2_;
    pass1_1008_3eb4((ushort *)CONCAT22(param_3, &local_16),
                    (ushort *)CONCAT22(param_3, local_1c),
                    (ushort *)CONCAT22(param_3, &local_1a),
                    (ushort *)CONCAT22(param_3, &local_18));
    iVar1 = *(int *)((ushort)param_1 + 0xe);
    if(iVar1 == 0x0)
    {
        pass1_1030_b344(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10, param_3);
        return;
    }
    if(iVar1 == 0x1)
    {
        uVar3 = local_1a - 0x1;
    LAB_1030_b63e:
        local_16 = local_16 & 0xffff | (ulong)uVar3 << 0x10;
        puVar2   = &local_16;
        pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,
                        (ushort *)CONCAT22(param_3, puVar2),
                        puVar4,
                        (int)&uStack16,
                        param_3);
        uStack48 = CONCAT22(puVar4, puVar2);
        if(((uint)puVar4 | (uint)puVar2) == 0x0)
        {
            return;
        }
        uVar6 = struct_op_1030_73a8(CONCAT22(puVar4, puVar2));
        uVar3 = *(uint *)((int)uVar6 + 0xc);
        if(uVar3 == 0x3f)
            goto LAB_1030_b6e0;
        if(0x3f < uVar3)
        {
            return;
        }
        if((char)uVar3 == '\x16')
            goto LAB_1030_b6e0;
        bVar5 = (char)uVar3 == '\x18';
    }
    else
    {
        if(iVar1 == 0x2)
        {
            uVar3 = local_18 + 0x1;
        }
        else
        {
            if(iVar1 == 0x3)
            {
                uVar3 = local_1a + 0x1;
                goto LAB_1030_b63e;
            }
            if(iVar1 != 0x4)
            {
                return;
            }
            uVar3 = local_18 - 0x1;
        }
        local_16 = local_16 & 0xffff0000 | (ulong)uVar3;
        puVar2   = &local_16;
        pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,
                        (ushort *)CONCAT22(param_3, puVar2),
                        puVar4,
                        (int)&uStack16,
                        param_3);
        uStack48 = CONCAT22(puVar4, puVar2);
        if(((uint)puVar4 | (uint)puVar2) == 0x0)
        {
            return;
        }
        uVar6 = struct_op_1030_73a8(CONCAT22(puVar4, puVar2));
        iVar1 = *(int *)((int)uVar6 + 0xc);
        if(iVar1 < 0x17)
        {
            return;
        }
        if(SBORROW2(iVar1, 0x17))
        {
            return;
        }
        if(iVar1 == 0x18 || iVar1 + -0x17 < 0x1)
            goto LAB_1030_b6e0;
        bVar5 = iVar1 == 0x3f;
    }
    if(!bVar5)
    {
        return;
    }
LAB_1030_b6e0:
    pass1_1008_3f62((ushort *)(param_1 & 0xffff0000 | (ulong)((ushort)param_1 + 0x8)),
                    (ushort *)(uStack48 & 0xffff0000 | (ulong)((int)uStack48 + 0xc)));
    return;
}


void __stdcall16far pass1_1030_b936(astruct_365 *param_1, ushort param_2, ushort param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b22c((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    param_1->field_0xe                        = 0x0;
    param_1->field_0x12                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xbc0c;
    param_1->field_0x2                        = 0x1030;
    return;
}


void __stdcall16far
pass1_1030_9adc(ushort param_1, ushort param_2, ulong *param_3, ulong param_4, uint param_5, uint param_6)

{
    code       **ppcVar1;
    astruct_99  *paVar2;
    uint         uVar4;
    uint         extraout_DX;
    uint         extraout_DX_00;
    astruct_121 *iVar7;
    astruct_119 *iVar6;
    astruct_99  *paStack6;
    astruct_120 *uVar3;

    pass1_1038_53ba(param_4, 0x1);
    uVar4 = param_6 | param_5;
    if(uVar4 != 0x0)
    {
        paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        uVar4    = (uint)((ulong)paStack6 >> 0x10);
        paVar2   = (astruct_99 *)((ulong)paStack6 & 0xffff);
        if((uVar4 | (uint)paVar2) == 0x0)
        {
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            iVar7               = (astruct_121 *)paStack6;
            paStack6->field_0x0 = 0x389a;
            iVar7->field_0x2    = 0x1008;
            iVar7->field_0x4    = 0x77;
            paStack6->field_0x0 = 0x9ec8;
            iVar7->field_0x2    = 0x1030;
            paVar2              = paStack6;
        }
        param_5 = (uint)paVar2;
        ppcVar1 = (code **)((int)*param_3 + 0x4);
        (**ppcVar1)(0x1000, param_3, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
        uVar4 = extraout_DX;
    }
    pass1_1038_53ba(param_4, 0x2);
    uVar4 = uVar4 | param_5;
    if(uVar4 != 0x0)
    {
        paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        uVar4    = (uint)((ulong)paStack6 >> 0x10);
        paVar2   = (astruct_99 *)((ulong)paStack6 & 0xffff);
        if((uVar4 | (uint)paVar2) == 0x0)
        {
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            iVar6               = (astruct_119 *)paStack6;
            paStack6->field_0x0 = 0x389a;
            iVar6->field_0x2    = 0x1008;
            iVar6->field_0x4    = 0x78;
            paStack6->field_0x0 = 0x9ec8;
            iVar6->field_0x2    = 0x1030;
            paVar2              = paStack6;
        }
        param_5 = (uint)paVar2;
        ppcVar1 = (code **)((int)*param_3 + 0x8);
        (**ppcVar1)(0x1000, param_3, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
        uVar4 = extraout_DX_00;
    }
    pass1_1038_53ba(param_4, 0x3);
    if((uVar4 | param_5) != 0x0)
    {
        paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        uVar4    = (uint)((ulong)paStack6 >> 0x10);
        uVar3    = (astruct_120 *)paStack6;
        if((uVar4 | (uint)uVar3) == 0x0)
        {
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            paStack6->field_0x0 = 0x389a;
            uVar3->field_0x2    = 0x1008;
            uVar3->field_0x4    = 0x75;
            paStack6->field_0x0 = 0x9ec8;
            uVar3->field_0x2    = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_3 + 0x8);
        (**ppcVar1)(0x1000, param_3, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
    }
    return;
}


ushort __stdcall16far pass1_1030_9ef2(ulong *param_1)

{
    int        iVar1;
    undefined2 uVar2;
    ulong      uVar3;

    if(*param_1 != 0x0)
    {
        uVar3 = struct_op_1030_73a8(*param_1);
        uVar2 = (undefined2)(uVar3 >> 0x10);
        iVar1 = *(int *)((int)uVar3 + 0xc);
        if(((iVar1 != 0x5) && (iVar1 != 0x9)) && (*(int *)((int)uVar3 + 0x12) < 0x5))
        {
            return 0x0;
        }
        pass1_1030_9f64(param_1);
    }
    return 0x1;
}


BOOL16 __stdcall16far pass1_1030_8fe4(
  ushort param_1, uint param_2, uint param_3, ushort param_4, ushort param_5, ushort *param_6, long param_7)

{
    int   iVar1;
    uint  uVar2;
    ulong uVar3;

    pass1_1030_627e(param_1, param_2, param_3, _PTR_LOOP_1050_5740, param_6, param_7);
    uVar2 = param_3 | param_2;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2, param_3);
        if((uVar2 | param_2) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_2));
            if((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9))))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1030_9048(ushort param_1, ulong param_2, int param_3, ulong param_4)

{
    int         iVar1;
    ulong       uVar2;
    code      **ppcVar3;
    BOOL16      BVar4;
    ushort      uVar5;
    ulong       uVar6;
    uchar      *puVar7;
    uint        extraout_DX;
    uint        extraout_DX_00;
    uint        uVar8;
    undefined2  uVar9;
    undefined4 *puVar10;
    undefined2  uVar11;
    undefined2  uVar12;
    ulong      *puVar13;
    ulong       uVar14;
    ushort      uVar15;
    undefined   uVar16;
    ulong       uStack36;
    undefined   local_18[0x2];
    int         local_16;
    int         local_14;
    int         local_12;
    int         iStack16;
    undefined4  uStack12;
    uint        uStack8;
    uchar      *puStack6;
    int         iStack4;

    iStack4  = 0x8 - (uint)(param_3 == 0x0);
    puVar13  = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, iStack4);
    puVar7   = (uchar *)((ulong)puVar13 >> 0x10);
    uVar8    = (uint)puVar13;
    uStack8  = uVar8;
    puStack6 = puVar7;
    pass1_1038_4e78(uVar8, puVar7, param_4, puVar13);
    uStack12 = (undefined4 *)CONCAT22(puVar7, uVar8);
    uVar12   = 0x1008;
    pass1_1008_3e38((ushort *)CONCAT22(param_1, &local_12));
    uVar2   = *(ulong *)((int)param_4 + 0x8);
    uVar11  = (undefined2)((ulong)uStack12 >> 0x10);
    uVar9   = SUB42(uStack12, 0x0);
    ppcVar3 = (code **)((int)*uStack12 + 0x10);
    uVar6   = uVar2;
    (**ppcVar3)(0x1008, uVar9, uVar11);
    uVar6    = uVar6 & 0xffff | (ulong)extraout_DX << 0x10;
    uStack36 = 0x0;
    while(true)
    {
        if(uVar6 <= uStack36)
        {
            if(uStack12 != (undefined4 *)0x0)
            {
                ppcVar3 = (code **)*uStack12;
                (**ppcVar3)(
                  uVar12, (int)uStack12, (char)((ulong)uStack12 >> 0x10), 0x1, uVar9, uVar11, uStack12, uStack12);
            }
            return;
        }
        ppcVar3 = (code **)((int)*uStack12 + 0x4);
        uVar14  = uVar6;
        (**ppcVar3)();
        uVar5 = (ushort)uVar14;
        uVar8 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
        pass1_1008_3f62((ushort *)CONCAT22(param_1, &local_12), (ushort *)CONCAT22(uVar8, uVar5 + 0xc));
        uVar12 = 0x1008;
        pass1_1008_3eb4((ushort *)CONCAT22(param_1, &local_12),
                        (ushort *)CONCAT22(param_1, local_18),
                        (ushort *)CONCAT22(param_1, &local_16),
                        (ushort *)CONCAT22(param_1, &local_14));
        uVar14 = struct_op_1030_73a8(CONCAT22(uVar8, uVar5));
        uVar8  = (uint)(uVar14 >> 0x10);
        iVar1  = *(int *)((int)uVar14 + 0xc);
        if(iVar1 - 0x7aU < 0x6)
            break;
    LAB_1030_91fa:
        uStack36 = uStack36 + 0x1;
    }
    uVar12 = 0x1030;
    uVar5  = (ushort)param_2;
    uVar15 = (ushort)(param_2 >> 0x10);
    switch(iVar1)
    {
    default:
        iStack16 = local_16 + -0x1;
        BVar4    = pass1_1030_8fe4(
          param_1, (uint)&local_12, uVar8, uVar5, uVar15, (ushort *)CONCAT22(param_1, &local_12), uVar2);
        if(BVar4 != 0x0)
            goto LAB_1030_91cb;
        iStack16 = local_16 + 0x1;
        BVar4    = pass1_1030_8fe4(
          param_1, (uint)&local_12, uVar8, uVar5, uVar15, (ushort *)CONCAT22(param_1, &local_12), uVar2);
        if(BVar4 == 0x0)
        {
            iStack16 = local_16;
            local_12 = local_14 + -0x1;
            BVar4    = pass1_1030_8fe4(
              param_1, (uint)&local_12, uVar8, uVar5, uVar15, (ushort *)CONCAT22(param_1, &local_12), uVar2);
            goto joined_r0x1030911e;
        }
    LAB_1030_9144:
        break;
    case 0x7b:
    case 0x7e:
        iStack16 = local_16 + -0x1;
        BVar4    = pass1_1030_8fe4(
          param_1, (uint)&local_12, uVar8, uVar5, uVar15, (ushort *)CONCAT22(param_1, &local_12), uVar2);
        if(BVar4 == 0x0)
        {
            iStack16 = local_16 + 0x1;
            goto LAB_1030_912c;
        }
        if(uStack12 == (undefined4 *)0x0)
        {
            return;
        }
        uVar12  = (undefined2)((ulong)uStack12 >> 0x10);
        puVar10 = (undefined4 *)uStack12;
        uVar16  = (undefined)((ulong)uStack12 >> 0x10);
        goto LAB_1030_90e6;
    case 0x7c:
    case 0x7d:
        local_12 = local_14 + -0x1;
        BVar4    = pass1_1030_8fe4(
          param_1, (uint)&local_12, uVar8, uVar5, uVar15, (ushort *)CONCAT22(param_1, &local_12), uVar2);
    joined_r0x1030911e:
        if(BVar4 == 0x0)
        {
            local_12 = local_14 + 0x1;
        LAB_1030_912c:
            BVar4 = pass1_1030_8fe4(
              param_1, (uint)&local_12, uVar8, uVar5, uVar15, (ushort *)CONCAT22(param_1, &local_12), uVar2);
            if(BVar4 != 0x0)
                goto LAB_1030_9144;
            goto LAB_1030_91fa;
        }
    LAB_1030_91cb:
    }
    puVar10 = (undefined4 *)uStack12;
    if((uStack12._2_2_ | (uint)puVar10) != 0x0)
    {
        uVar12 = (undefined2)((ulong)uStack12 >> 0x10);
        uVar16 = (undefined)((ulong)uStack12 >> 0x10);
    LAB_1030_90e6:
        ppcVar3 = (code **)*puVar10;
        (**ppcVar3)(0x1030, puVar10, uVar16, 0x1, uVar9, uVar11, uStack12, uStack12);
    }
    return;
}


void __stdcall16far
pass1_1030_9296(ulong param_1, ulong *param_2, ulong param_3, ushort param_4, uint param_5, uint param_6)

{
    code       **ppcVar1;
    undefined2   uVar2;
    undefined   *puVar3;
    undefined2   in_register_00000002;
    astruct_99  *paVar4;
    ulong        uVar6;
    uint         uVar7;
    uint         extraout_DX;
    uint         extraout_DX_00;
    uchar       *puVar8;
    uchar       *extraout_DX_01;
    uchar       *extraout_DX_02;
    uint         extraout_DX_03;
    uint         uVar9;
    astruct_116 *iVar11;
    astruct_115 *iVar10;
    astruct_114 *iVar9;
    int          unaff_DI;
    undefined2   uVar10;
    undefined    uVar11;
    undefined    local_3a[0xc];
    undefined4   uStack46;
    ulong        uStack36;
    ulong        uStack30;
    undefined2   uStack26;
    astruct_99  *paStack18;
    ulong        uStack14;
    ushort      *puStack10;
    astruct_99  *paStack6;
    astruct_113 *uVar5;

    paVar4 = (astruct_99 *)CONCAT22(in_register_00000002, param_5);
    pass1_1038_53ba(param_3, 0x1);
    uVar7  = param_6 | (uint)paVar4;
    uVar10 = (undefined2)((ulong)param_2 >> 0x10);
    uVar11 = SUB41(param_2, 0x0);
    if(uVar7 != 0x0)
    {
        uStack30  = _PTR_LOOP_1050_5768;
        uVar6     = _PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        uVar7     = (uint)((ulong)paStack18 >> 0x10);
        paVar4    = (astruct_99 *)(uVar6 & 0xffff0000 | (ulong)paStack18 & 0xffff);
        if((uVar7 | (uint)((ulong)paStack18 & 0xffff)) == 0x0)
        {
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            iVar11               = (astruct_116 *)paStack18;
            paStack18->field_0x0 = 0x389a;
            iVar11->field_0x2    = 0x1008;
            iVar11->field_0x4    = 0x73;
            paStack18->field_0x0 = 0x9ec8;
            iVar11->field_0x2    = 0x1030;
            paVar4               = paStack18;
            paStack6             = paStack18;
        }
        ppcVar1 = (code **)((int)*param_2 + 0x4);
        (**ppcVar1)(0x1000, uVar11, uVar10, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
        uVar7 = extraout_DX;
    }
    pass1_1038_53ba(param_3, 0x2);
    uVar7 = uVar7 | (uint)paVar4;
    if(uVar7 != 0x0)
    {
        uStack30  = _PTR_LOOP_1050_5768;
        uVar6     = _PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        uVar7     = (uint)((ulong)paStack18 >> 0x10);
        paVar4    = (astruct_99 *)(uVar6 & 0xffff0000 | (ulong)paStack18 & 0xffff);
        if((uVar7 | (uint)((ulong)paStack18 & 0xffff)) == 0x0)
        {
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            iVar10               = (astruct_115 *)paStack18;
            paStack18->field_0x0 = 0x389a;
            iVar10->field_0x2    = 0x1008;
            iVar10->field_0x4    = 0x74;
            paStack18->field_0x0 = 0x9ec8;
            iVar10->field_0x2    = 0x1030;
            paVar4               = paStack18;
            paStack6             = paStack18;
        }
        ppcVar1 = (code **)((int)*param_2 + 0x8);
        (**ppcVar1)(0x1000, uVar11, uVar10, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
        uVar7 = extraout_DX_00;
    }
    pass1_1038_53ba(param_3, 0x3);
    puVar8 = (uchar *)(uVar7 | (uint)paVar4);
    if(puVar8 != (uchar *)0x0)
    {
        uStack36  = _PTR_LOOP_1050_5768;
        uVar6     = _PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        uVar7     = (uint)((ulong)paStack18 >> 0x10);
        paVar4    = (astruct_99 *)(uVar6 & 0xffff0000 | (ulong)paStack18 & 0xffff);
        if((uVar7 | (uint)((ulong)paStack18 & 0xffff)) == 0x0)
        {
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            iVar9                = (astruct_114 *)paStack18;
            paStack18->field_0x0 = 0x389a;
            iVar9->field_0x2     = 0x1008;
            iVar9->field_0x4     = 0x75;
            paStack18->field_0x0 = 0x9ec8;
            iVar9->field_0x2     = 0x1030;
            paVar4               = paStack18;
            paStack6             = paStack18;
        }
        ppcVar1 = (code **)((int)*param_2 + 0x8);
        (**ppcVar1)(0x1000, uVar11, uVar10, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
        puVar8 = extraout_DX_01;
    }
    pass1_1030_8f04((ushort)param_1, (ushort)(param_1 >> 0x10), param_3, (ulong)paVar4, (uint)puVar8);
    if((int)paVar4 != 0x0)
    {
        uStack36  = _PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
        uVar7     = (uint)((ulong)paStack18 >> 0x10);
        uVar5     = (astruct_113 *)paStack18;
        if((uVar7 | (uint)uVar5) == 0x0)
        {
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            paStack18->field_0x0 = 0x389a;
            uVar5->field_0x2     = 0x1008;
            uVar5->field_0x4     = 0x37;
            paStack18->field_0x0 = 0x9ec8;
            uVar5->field_0x2     = 0x1030;
            paStack6             = paStack18;
        }
        ppcVar1 = (code **)((int)*param_2 + 0x8);
        (**ppcVar1)(0x1000, uVar11, uVar10, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
        puVar8 = extraout_DX_02;
    }
    puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_4, puVar8, unaff_DI);
    uVar2     = (undefined2)((ulong)puStack10 >> 0x10);
    uStack14  = *(ulong *)((int)puStack10 + 0xe);
    uVar7     = *(uint *)((int)puStack10 + 0x10);
    if((uVar7 | (uint)uStack14) != 0x0)
    {
        pass1_1008_5784((ulong *)CONCAT22(param_4, local_3a), uStack14 & 0xffff | (ulong)uVar7 << 0x10);
        while(true)
        {
            puVar3 = local_3a;
            pass1_1008_5b12(puVar3, param_4);
            uStack46 = CONCAT22(extraout_DX_03, puVar3);
            if((extraout_DX_03 | (uint)puVar3) == 0x0)
                break;
            if((*(int *)(puVar3 + 0x4) == 0x3e) || (*(int *)(puVar3 + 0x4) == 0x41))
            {
                uStack30  = _PTR_LOOP_1050_5768;
                paStack18 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_5768);
                uVar9     = (uint)((ulong)paStack18 >> 0x10);
                uVar7     = (uint)paStack18;
                if((uVar9 | uVar7) == 0x0)
                {
                    paStack6 = (astruct_99 *)0x0;
                }
                else
                {
                    uStack26                     = *(undefined2 *)((int)uStack46 + 0x4);
                    paStack18->field_0x0         = 0x389a;
                    *(undefined2 *)(uVar7 + 0x2) = 0x1008;
                    *(undefined2 *)(uVar7 + 0x4) = uStack26;
                    paStack18->field_0x0         = 0x9ec8;
                    *(undefined2 *)(uVar7 + 0x2) = 0x1030;
                    paStack6                     = paStack18;
                }
                ppcVar1 = (code **)((int)*param_2 + 0x8);
                (**ppcVar1)(0x1000, uVar11, uVar10, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_951a(ushort param_1, ushort param_2, ulong param_3, ulong *param_4, ulong param_5)

{
    code       **ppcVar1;
    uint         uVar6;
    ushort      *puVar7;
    ushort       uVar8;
    uchar       *puVar9;
    uint         extraout_DX;
    uint         uVar10;
    undefined2   extraout_DX_00;
    uint         extraout_DX_01;
    int          iVar11;
    undefined2  *puVar12;
    int          unaff_DI;
    undefined2   uVar13;
    undefined2   uVar14;
    undefined    uVar15;
    ulong       *puVar16;
    ulong        uVar17;
    undefined    uVar18;
    undefined    uVar19;
    undefined    uVar20;
    undefined4  *puStack76;
    ulong        uStack70;
    ulong        uStack62;
    astruct_99  *paStack58;
    ushort       local_36;
    uint         uStack52;
    undefined4   uStack46;
    undefined2   uStack42;
    uint         uStack40;
    int          iStack38;
    ushort      *puStack36;
    ushort      *puStack32;
    int          iStack28;
    int          iStack20;
    ulong        uStack18;
    ulong        uStack14;
    ushort      *puStack10;
    astruct_99  *paStack6;
    astruct_122 *uVar2;
    astruct_123 *uVar3;
    astruct_124 *uVar4;
    astruct_125 *uVar5;

    puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x35, param_1, (uchar *)param_2, unaff_DI);
    puVar9    = (uchar *)((ulong)puStack10 >> 0x10);
    uVar6     = (int)puStack10 + 0xa;
    uStack14  = (ulong)puStack10 & 0xffff0000 | (ulong)uVar6;
    pass1_1030_9048(param_1, param_3, 0x0, param_5);
    uVar13 = (undefined2)((ulong)param_4 >> 0x10);
    uVar20 = SUB41(param_4, 0x0);
    if(uVar6 != 0x0)
    {
        iStack28  = 0x0;
        puStack32 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_1, puVar9, unaff_DI);
        uVar14    = (undefined2)((ulong)puStack32 >> 0x10);
        puStack36 = *(ushort **)((int)puStack32 + 0xe);
        uVar6     = *(uint *)((int)puStack32 + 0x10);
        if((uVar6 | (uint)puStack36) != 0x0)
        {
            pass1_1008_5784((ulong *)CONCAT22(param_1, &local_36), (ulong)puStack36 & 0xffff | (ulong)uVar6 << 0x10);
            while(true)
            {
                puVar7 = &local_36;
                pass1_1008_5b12(puVar7, param_1);
                uStack46 = CONCAT22(extraout_DX, puVar7);
                if((extraout_DX | (uint)puVar7) == 0x0)
                    break;
                if((puVar7[0x2] != 0x3e) && (puVar7[0x2] != 0x41))
                {
                    paStack6 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
                    uVar10   = (uint)((ulong)paStack6 >> 0x10);
                    uVar6    = (uint)paStack6;
                    if((uVar10 | uVar6) == 0x0)
                    {
                        paStack6 = (astruct_99 *)0x0;
                    }
                    else
                    {
                        uVar14                       = *(undefined2 *)((int)uStack46 + 0x4);
                        paStack6->field_0x0          = 0x389a;
                        *(undefined2 *)(uVar6 + 0x2) = 0x1008;
                        *(undefined2 *)(uVar6 + 0x4) = uVar14;
                        paStack6->field_0x0          = 0x9ec8;
                        *(undefined2 *)(uVar6 + 0x2) = 0x1030;
                    }
                    ppcVar1 = (code **)((int)*param_4 + 0x8);
                    (**ppcVar1)(0x0, uVar20, uVar13, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
                    if(*(int *)((int)uStack46 + 0x4) == 0x13)
                    {
                        iStack28 = 0x1;
                    }
                }
            }
        }
        for(iStack38 = 0xa; iStack38 < 0x41; iStack38 = iStack38 + 0x1)
        {
            if((((((iStack38 != 0x37) && (iStack38 != 0x35)) && (iStack38 != 0x36))
                 && ((iStack38 != 0x25 && (iStack38 != 0x26))))
                && ((iStack38 != 0x27 && ((*(int *)(iStack38 * 0x2 + (int)uStack14) != 0x0 && (iStack38 != 0x13))))))
               && ((iStack38 != 0x14 || (iStack28 == 0x0))))
            {
                paStack6 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
                uVar10   = (uint)((ulong)paStack6 >> 0x10);
                uVar6    = (uint)paStack6;
                if((uVar10 | uVar6) == 0x0)
                {
                    paStack6 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack6->field_0x0          = 0x389a;
                    *(undefined2 *)(uVar6 + 0x2) = 0x1008;
                    *(int *)(uVar6 + 0x4)        = iStack38;
                    paStack6->field_0x0          = 0x9ec8;
                    *(undefined2 *)(uVar6 + 0x2) = 0x1030;
                }
                ppcVar1 = (code **)((int)*param_4 + 0x8);
                (**ppcVar1)(0x0, uVar20, uVar13, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
            }
        }
    }
    uVar14 = (undefined2)(uStack14 >> 0x10);
    if(*(int *)((int)uStack14 + 0x6a) == 0x0)
    {
        if(*(int *)((int)uStack14 + 0x6c) != 0x0)
        {
            paStack58 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
            uVar6     = (uint)((ulong)paStack58 >> 0x10);
            puVar12   = (undefined2 *)paStack58;
            if((uVar6 | (uint)puVar12) == 0x0)
                goto LAB_1030_973e;
            paStack58->field_0x0 = 0x389a;
            puVar12[0x1]         = 0x1008;
            puVar12[0x2]         = 0x36;
            goto LAB_1030_9728;
        }
    }
    else
    {
        paStack58 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
        uVar6     = (uint)((ulong)paStack58 >> 0x10);
        puVar12   = (undefined2 *)paStack58;
        if((uVar6 | (uint)puVar12) == 0x0)
        {
        LAB_1030_973e:
            paStack6 = (astruct_99 *)0x0;
        }
        else
        {
            paStack58->field_0x0 = 0x389a;
            puVar12[0x1]         = 0x1008;
            puVar12[0x2]         = 0x35;
        LAB_1030_9728:
            *puVar12     = 0x9ec8;
            puVar12[0x1] = 0x1030;
            paStack6     = paStack58;
        }
        ppcVar1 = (code **)((int)*param_4 + 0x8);
        (**ppcVar1)(0x0, uVar20, uVar13, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
    }
    uVar14 = (undefined2)(uStack14 >> 0x10);
    iVar11 = (int)uStack14;
    if(*(int *)(iVar11 + 0x4a) == 0x0)
    {
        if(*(int *)(iVar11 + 0x4c) == 0x0)
        {
            if(*(int *)(iVar11 + 0x4e) == 0x0)
                goto LAB_1030_97e8;
            paStack58 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
            uVar6     = (uint)((ulong)paStack58 >> 0x10);
            puVar12   = (undefined2 *)paStack58;
            if((uVar6 | (uint)puVar12) != 0x0)
            {
                paStack58->field_0x0 = 0x389a;
                puVar12[0x1]         = 0x1008;
                puVar12[0x2]         = 0x27;
                goto LAB_1030_9879;
            }
        }
        else
        {
            paStack58 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
            uVar6     = (uint)((ulong)paStack58 >> 0x10);
            puVar12   = (undefined2 *)paStack58;
            if((uVar6 | (uint)puVar12) != 0x0)
            {
                paStack58->field_0x0 = 0x389a;
                puVar12[0x1]         = 0x1008;
                puVar12[0x2]         = 0x26;
                goto LAB_1030_9879;
            }
        }
    LAB_1030_97d0:
        paStack6 = (astruct_99 *)0x0;
    }
    else
    {
        paStack58 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
        uVar6     = (uint)((ulong)paStack58 >> 0x10);
        puVar12   = (undefined2 *)paStack58;
        if((uVar6 | (uint)puVar12) == 0x0)
            goto LAB_1030_97d0;
        paStack58->field_0x0 = 0x389a;
        puVar12[0x1]         = 0x1008;
        puVar12[0x2]         = 0x25;
    LAB_1030_9879:
        *puVar12     = 0x9ec8;
        puVar12[0x1] = 0x1030;
        paStack6     = paStack58;
    }
    ppcVar1 = (code **)((int)*param_4 + 0x8);
    (**ppcVar1)(0x0, uVar20, uVar13, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
LAB_1030_97e8:
    uStack18 = (ulong)puStack10 & 0xffff0000 | (ulong)((int)puStack10 + 0x11e);
    if(*(int *)((int)puStack10 + 0x138) != 0x0)
    {
        puVar16 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
        puVar9  = (uchar *)((ulong)puVar16 >> 0x10);
        uVar6   = (uint)puVar16;
        uVar15  = 0x38;
        pass1_1038_4d6e(param_5, puVar16, uVar6, puVar9);
        puStack76 = (undefined4 *)CONCAT22(puVar9, uVar6);
        ppcVar1   = (code **)((int)*puStack76 + 0x10);
        uVar10    = uVar6;
        (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar6, puVar9);
        uStack70 = CONCAT22(extraout_DX_00, uVar10);
        for(uStack62 = 0x0; uStack62 < uStack70; uStack62 = uStack62 + 0x1)
        {
            ppcVar1 = (code **)((int)*puStack76 + 0x4);
            uVar17  = uStack70;
            (**ppcVar1)(uVar15, (char)uVar6, puVar9, (int)uStack62, (int)(uStack62 >> 0x10));
            uVar8    = (ushort)uVar17;
            iVar11   = 0xd;
            uVar10   = extraout_DX_01;
            local_36 = uVar8;
            uStack52 = extraout_DX_01;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar8, extraout_DX_01);
            uStack46 = CONCAT22(uVar10, uVar8);
            uVar17   = struct_op_1030_73a8(CONCAT22(uVar10, uVar8));
            uVar10   = (uint)(uVar17 >> 0x10);
            uStack42 = (undefined2)uVar17;
            uVar15   = 0x28;
            uStack40 = uVar10;
            uVar8    = pass1_1028_6744(param_1, uVar17, iVar11);
            if((uVar10 | uVar8) != 0x0)
            {
                puStack32 = _PTR_LOOP_1050_5768;
                paStack6  = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
                uVar10    = (uint)((ulong)paStack6 >> 0x10);
                uVar5     = (astruct_125 *)paStack6;
                if((uVar10 | (uint)uVar5) == 0x0)
                {
                    paStack6 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack6->field_0x0 = 0x389a;
                    uVar5->field_0x2    = 0x1008;
                    uVar5->field_0x4    = 0x4c;
                    paStack6->field_0x0 = 0x9ec8;
                    uVar5->field_0x2    = 0x1030;
                }
                ppcVar1 = (code **)((int)*param_4 + 0x8);
                (**ppcVar1)(0x0, uVar20, uVar13, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
                puStack36 = _PTR_LOOP_1050_5768;
                paStack6  = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
                uVar10    = (uint)((ulong)paStack6 >> 0x10);
                uVar4     = (astruct_124 *)paStack6;
                if((uVar10 | (uint)uVar4) == 0x0)
                {
                    paStack6 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack6->field_0x0 = 0x389a;
                    uVar4->field_0x2    = 0x1008;
                    uVar4->field_0x4    = 0x4d;
                    paStack6->field_0x0 = 0x9ec8;
                    uVar4->field_0x2    = 0x1030;
                }
                uVar18  = SUB41(paStack6, 0x0);
                uVar19  = (undefined)((ulong)paStack6 >> 0x10);
                ppcVar1 = (code **)((int)*param_4 + 0x8);
                puVar16 = param_4;
                (**ppcVar1)();
                puStack36 = _PTR_LOOP_1050_5768;
                uVar15    = 0x0;
                paStack6  = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
                uVar10    = (uint)((ulong)paStack6 >> 0x10);
                uVar3     = (astruct_123 *)paStack6;
                if((uVar10 | (uint)uVar3) == 0x0)
                {
                    paStack6 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack6->field_0x0 = 0x389a;
                    uVar3->field_0x2    = 0x1008;
                    uVar3->field_0x4    = 0x4e;
                    paStack6->field_0x0 = 0x9ec8;
                    uVar3->field_0x2    = 0x1030;
                }
                ppcVar1 = (code **)((int)*param_4 + 0x8);
                (**ppcVar1)(0x1000, param_4, paStack6, puVar16, uVar18, uVar19);
                break;
            }
        }
        if(puStack76 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack76;
            (**ppcVar1)(uVar15, uVar6, puVar9, 0x1);
        }
    }
    for(iStack20 = 0x7a; iStack20 < 0x7d; iStack20 = iStack20 + 0x1)
    {
        if(*(int *)(iStack20 * 0x2 + (int)uStack14) != 0x0)
        {
            paStack6 = pass1_1000_07fc(0x1000, (ulong)_PTR_LOOP_1050_5768);
            uVar6    = (uint)((ulong)paStack6 >> 0x10);
            uVar2    = (astruct_122 *)paStack6;
            if((uVar6 | (uint)uVar2) == 0x0)
            {
                paStack6 = (astruct_99 *)0x0;
            }
            else
            {
                paStack6->field_0x0 = 0x389a;
                uVar2->field_0x2    = 0x1008;
                uVar2->field_0x4    = iStack20;
                paStack6->field_0x0 = 0x9ec8;
                uVar2->field_0x2    = 0x1030;
            }
            ppcVar1 = (code **)((int)*param_4 + 0x8);
            (**ppcVar1)(0x0, uVar20, uVar13, (int)paStack6, (int)((ulong)paStack6 >> 0x10));
        }
    }
    return;
}


ushort __stdcall16far pass1_1030_7bee(ulong param_1)

{
    code **ppcVar1;
    ushort uVar2;
    int    iVar3;
    uint   uVar4;

    uVar4 = (uint)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0x16) == 0x0)
    {
        return 0x0;
    }
    if(*(long *)(iVar3 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar4 << 0x10);
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x1a) + 0x44);
    uVar2   = (**ppcVar1)();
    return uVar2;
}


void __stdcall16far pass1_1030_7e5a(ulong param_1, ulong param_2, uint param_3)

{
    astruct_358 *iVar1;
    uint         uVar1;

    uVar1             = (uint)(param_1 >> 0x10);
    iVar1             = (astruct_358 *)param_1;
    iVar1->field_0x16 = param_2;
    iVar1->field_0x1a = 0x0;
    pass1_1030_6fa0(param_1 & 0xffff | (ulong)uVar1 << 0x10);
    if(iVar1->field_0x2e != 0x0)
    {
        pass1_1038_4b20(iVar1->field_0x2e, iVar1->field_0x16, iVar1->field_0x4, param_3);
    }
    return;
}


void __stdcall16far pass1_1030_7eda(ulong param_1, ushort param_2, ushort param_3)

{
    uint       uVar1;
    undefined2 local_c;
    undefined2 uStack10;
    ushort     uStack8;
    undefined2 uStack6;
    undefined2 uStack4;

    local_c  = 0x0;
    uStack10 = 0x0;
    uStack6  = 0x0;
    uStack4  = 0x0;
    uStack8  = param_2;
    uVar1    = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
    }
    pass1_1028_bb96(*(ulong *)((int)param_1 + 0x1a), (ulong *)&local_c, param_3);
    return;
}


void __stdcall16far pass1_1030_7f1a(ulong param_1, ushort param_2, ushort param_3)

{
    uint       uVar1;
    undefined2 local_c;
    ushort     uStack10;
    undefined2 uStack8;
    undefined2 uStack6;
    undefined2 uStack4;

    local_c  = 0x0;
    uStack8  = 0x0;
    uStack6  = 0x0;
    uStack4  = 0x0;
    uStack10 = param_2;
    uVar1    = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
    }
    pass1_1028_bb96(*(ulong *)((int)param_1 + 0x1a), (ulong *)&local_c, param_3);
    return;
}


ushort __stdcall16far pass1_1030_7f5a(ulong param_1)

{
    uint  uVar1;
    ulong uVar2;

    uVar1 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
    }
    uVar2 = pass1_1028_bb6a(*(ulong *)((int)param_1 + 0x1a));
    if(uVar2 != 0x0)
    {
        return *(ushort *)((int)uVar2 + 0x4);
    }
    return 0x0;
}


ushort __stdcall16far pass1_1030_7f98(ulong param_1)

{
    uint  uVar1;
    ulong uVar2;

    uVar1 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
    }
    uVar2 = pass1_1028_bb6a(*(ulong *)((int)param_1 + 0x1a));
    if(uVar2 != 0x0)
    {
        return *(ushort *)((int)uVar2 + 0x2);
    }
    return 0x0;
}


void __stdcall16far pass1_1030_7fd6(ulong param_1, ushort param_2, ushort param_3)

{
    int        iVar1;
    undefined4 uVar2;
    int        iVar3;
    uint       uVar4;
    ulong      uVar5;

    uVar4 = (uint)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0x1a) == 0x0)
    {
        uVar5   = struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar4 << 0x10);
        param_2 = (ushort)(uVar5 >> 0x10);
    }
    uVar2 = *(undefined4 *)(iVar3 + 0x1a);
    iVar1 = *(int *)((int)uVar2 + 0xc);
    if(((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
       && ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2))))))
    {
        pass1_1028_1416(*(ulong *)(iVar3 + 0x1a), param_2, param_3);
    }
    return;
}


void __stdcall16far pass1_1030_8030(ulong param_1, ushort param_2, ushort param_3)

{
    int        iVar1;
    undefined4 uVar2;
    int        iVar3;
    uint       uVar4;
    ulong      uVar5;

    uVar4 = (uint)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0x1a) == 0x0)
    {
        uVar5   = struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar4 << 0x10);
        param_2 = (ushort)(uVar5 >> 0x10);
    }
    uVar2 = *(undefined4 *)(iVar3 + 0x1a);
    iVar1 = *(int *)((int)uVar2 + 0xc);
    if(((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
       && ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2))))))
    {
        uVar5 = *(ulong *)(iVar3 + 0x1a);
        pass1_1028_1106(uVar5, (int)uVar5, param_2, param_3);
    }
    return;
}


void __stdcall16far pass1_1030_809c(ulong param_1)

{
    uint uVar1;

    uVar1 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar1 << 0x10);
    }
    return;
}


void __stdcall16far struct_1030_8544(ushort *param_1, ushort *param_2)

{
    astruct_356 *iVar1;
    astruct_355 *iVar2;
    undefined2   uVar1;
    undefined2   uVar2;

    *param_1         = *param_2;
    uVar1            = (undefined2)((ulong)param_2 >> 0x10);
    iVar1            = (astruct_356 *)param_2;
    uVar2            = (undefined2)((ulong)param_1 >> 0x10);
    iVar2            = (astruct_355 *)param_1;
    iVar2->field_0x4 = iVar1->field_0x4;
    pass1_1008_3f62((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x8),
                    (ushort *)((ulong)param_2 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x8));
    iVar2->field_0xe  = iVar1->field_0xe;
    iVar2->field_0x12 = iVar1->field_0x12;
    iVar2->field_0x16 = iVar1->field_0x16;
    iVar2->field_0x1a = iVar1->field_0x1a;
    iVar2->field_0x1e = 0x0;
    return;
}


void __stdcall16far pass1_1030_85be(long *param_1, ushort param_2, int param_3, int param_4, ushort param_5)

{
    astruct_357 *iVar1;
    ushort       uVar1;

    uVar1            = (ushort)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_357 *)param_1;
    *param_1         = 0x0;
    iVar1->field_0x4 = 0x0;
    iVar1->field_0x6 = param_3;
    iVar1->field_0x8 = param_2;
    iVar1->field_0xe = 0x0;
    if(iVar1->field_0x6 == 0x0)
    {
        iVar1->field_0x6 = 0x5;
    }
    pass1_1030_878c(param_1, param_4, param_5);
    return;
}


void __stdcall16far pass1_1030_86ec(astruct_18 **param_1, ushort param_2)

{
    astruct_612 *iVar1;
    undefined2   uVar1;

    fn_ptr_1000_17ce(*param_1, 0x1000);
    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_612 *)param_1;
    *param_1         = (astruct_18 *)0x0;
    iVar1->field_0x4 = 0x0;
    iVar1->field_0x6 = param_2;
    iVar1->field_0xe = 0x0;
    return;
}


void __stdcall16far pass1_1030_6a2c(ulong param_1, long param_2, uint param_3, uchar *param_4, ushort param_5)

{
    code       **ppcVar1;
    astruct_384 *iVar2;
    undefined2   uVar2;
    astruct_382 *iVar4;
    astruct_383 *iVar5;
    undefined2   uVar3;
    undefined2   uVar4;
    undefined4   uVar5;
    long         lVar6;
    undefined    local_a[0x8];

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar4 = (astruct_382 *)param_1;
    if(iVar4->field_0x3e == (undefined4 *)0x0)
    {
        mem_op_1000_179c(0xc, param_4, 0x1000);
        if(((uint)param_4 | param_3) == 0x0)
        {
            iVar4->field_0x3e = (undefined4 *)0x0;
        }
        else
        {
            uVar5                      = set_struct_1008_574a((astruct_21 *)CONCAT22(param_4, param_3));
            *(int *)&iVar4->field_0x3e = (int)uVar5;
            *(undefined2 *)((int)&iVar4->field_0x3e + 0x2) = (int)((ulong)uVar5 >> 0x10);
        }
    }
    pass1_1008_5784((ulong *)CONCAT22(param_5, local_a), (ulong)iVar4->field_0x3e);
    do
    {
        do
        {
            lVar6 = pass1_1008_5b12(local_a, param_5);
            uVar2 = (undefined2)((ulong)lVar6 >> 0x10);
            iVar2 = (astruct_384 *)lVar6;
            if(lVar6 == 0x0)
                goto LAB_1030_6af4;
            uVar4 = (undefined2)((ulong)param_2 >> 0x10);
            iVar5 = (astruct_383 *)param_2;
        } while((iVar2->field_0x6 != iVar5->field_0x6) || (iVar2->field_0x4 != iVar5->field_0x4));
    } while(iVar2->field_0x8 != iVar5->field_0x8);
    iVar2->field_0xa = iVar2->field_0xa + iVar5->field_0xa;
    iVar2->field_0xc = iVar2->field_0xc + iVar5->field_0xc;
    param_2          = 0x0;
LAB_1030_6af4:
    if(param_2 != 0x0)
    {
        ppcVar1 = (code **)((int)*iVar4->field_0x3e + 0x8);
        (**ppcVar1)(0x1008, iVar4->field_0x3e, param_2);
    }
    return;
}


ulong __stdcall16far pass1_1030_6b16(ulong param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    undefined4   uVar4;
    astruct_412 *iVar5;
    undefined2   uVar5;
    ulong        uVar6;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_412 *)param_1;
    if(*(long *)&iVar5->field_0x3a == 0x0)
    {
        return 0x0;
    }
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)&iVar5->field_0x3a + 0x10);
    uVar6   = (**ppcVar3)();
    uVar4   = *(undefined4 *)&iVar5->field_0x3a;
    if(*(int *)((int)uVar4 + 0x8) == 0x0)
    {
        puVar1 = (undefined4 *)*(uint *)&iVar5->field_0x3a;
        uVar2  = iVar5->field_0x3c;
        if((uVar2 | (uint)puVar1) != 0x0)
        {
            ppcVar3 = (code **)*puVar1;
            (**ppcVar3)();
        }
        *(undefined4 *)&iVar5->field_0x3a = 0x0;
    }
    return uVar6;
}


void __stdcall16far
pass1_1030_6c66(ulong param_1, int param_2, ulong param_3, uint param_4, uchar *param_5, ushort param_6)

{
    uint         uVar1;
    code       **ppcVar2;
    uint         uVar3;
    ushort       uVar4;
    BOOL16       BVar5;
    undefined2   extraout_DX;
    uchar       *puVar6;
    astruct_386 *iVar7;
    astruct_385 *iVar6;
    ushort       unaff_SI;
    ushort       unaff_DI;
    undefined2   uVar7;
    undefined2   uVar8;
    ushort       unaff_SS;

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar7 = (astruct_386 *)param_1;
    if(iVar7->field_0x3a == (undefined4 *)0x0)
    {
        param_6 = 0x1000;
        mem_op_1000_179c(0xc, param_5, 0x1000);
        if(((uint)param_5 | param_4) == 0x0)
        {
            iVar7->field_0x3a = (undefined4 *)0x0;
        }
        else
        {
            param_6 = 0x1008;
            set_struct_1008_574a((astruct_21 *)CONCAT22(param_5, param_4));
            *(uint *)&iVar7->field_0x3a                    = param_4;
            *(undefined2 *)((int)&iVar7->field_0x3a + 0x2) = extraout_DX;
        }
    }
    ppcVar2 = (code **)((int)*iVar7->field_0x3a + 0x8);
    (**ppcVar2)(param_6, iVar7->field_0x3a, param_3);
    if(param_2 != 0x0)
    {
        uVar8 = (undefined2)(param_3 >> 0x10);
        iVar6 = (astruct_385 *)param_3;
        if(iVar6->field_0x6 != 0x0)
        {
            pass1_1030_6e9c(param_1, (ulong)iVar6->field_0xa, iVar6->field_0x6);
            return;
        }
        if(iVar6->field_0x4 != 0x0)
        {
            uVar1  = iVar6->field_0xa;
            uVar3  = -uVar1;
            puVar6 = (uchar *)-(uint)(uVar1 != 0x0);
            pass1_1030_7ddc(param_1,
                            CONCAT13((char)((uint)puVar6 >> 0x8), CONCAT12((char)puVar6, uVar3)),
                            iVar6->field_0x4,
                            uVar3,
                            puVar6,
                            unaff_SI,
                            unaff_DI,
                            unaff_SS);
            return;
        }
        if(iVar6->field_0x8 != 0x0)
        {
            uVar4 = pass1_1030_6fa0(param_1);
            BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar4, 0x4);
            if(BVar5 != 0x0)
            {
                pass1_1028_6356(iVar7->field_0x1a, 0x0, iVar6->field_0xa, 0x0, unaff_SS);
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_6e4c(ulong param_1)

{
    undefined4 uVar1;
    int        iVar2;
    uint       uVar3;

    uVar3 = (uint)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(long *)(iVar2 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar3 << 0x10);
    }
    if((*(long *)(iVar2 + 0x1a) != 0x0) && (uVar1 = *(undefined4 *)(iVar2 + 0x1a), *(int *)((int)uVar1 + 0x12) == 0x4))
    {
        return;
    }
    return;
}


ushort __stdcall16far pass1_1030_6fa0(ulong param_1)

{
    undefined4 uVar1;
    int        iVar2;
    uint       uVar3;

    uVar3 = (uint)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(long *)(iVar2 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar3 << 0x10);
    }
    if(*(long *)(iVar2 + 0x1a) != 0x0)
    {
        uVar1 = *(undefined4 *)(iVar2 + 0x1a);
        return *(ushort *)((int)uVar1 + 0xc);
    }
    return 0x0;
}


void __stdcall16far pass1_1030_6fd4(ulong param_1)

{
    undefined4 uVar1;
    uint       uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    if(*(int *)((int)uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


void __stdcall16far pass1_1030_701c(ulong param_1)

{
    undefined4 uVar1;
    uint       uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    if(*(int *)((int)uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


void __stdcall16far pass1_1030_7064(ulong param_1)

{
    undefined4 uVar1;
    uint       uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    if(*(int *)((int)uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


void __stdcall16far pass1_1030_70ac(ulong param_1)

{
    undefined4 uVar1;
    uint       uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    if(*(int *)((int)uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_70f4(ulong param_1)

{
    undefined2 uVar1;
    undefined4 uVar2;
    BOOL16     BVar3;
    int        iVar4;
    uint       uVar5;
    long      *plVar6;

    uVar5 = (uint)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar5 << 0x10);
    }
    uVar2 = *(undefined4 *)(iVar4 + 0x1a);
    uVar1 = *(undefined2 *)((int)uVar2 + 0xc);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1);
    if(BVar3 == 0x0)
    {
        BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x2);
        if((BVar3 == 0x0) || (*(long *)(iVar4 + 0x22) == 0x0))
        {
            return;
        }
        plVar6 = *(long **)(iVar4 + 0x22);
    }
    else
    {
        uVar2  = *(undefined4 *)(iVar4 + 0x1a);
        plVar6 = *(long **)((int)uVar2 + 0x28);
    }
    pass1_1020_ba94(plVar6);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_7176(ulong param_1, ushort param_2)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;
    long       local_1a;
    int        local_16[0x2];
    uint       uStack18;
    uint       uStack14;
    BOOL16     BStack10;
    undefined2 uStack8;
    long       lStack6;

    lStack6 = 0x0;
    uVar3   = (undefined2)(param_1 >> 0x10);
    iVar2   = (int)param_1;
    if(*(long *)(iVar2 + 0x22) == 0x0)
    {
        return;
    }
    if(*(long *)(iVar2 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1);
    }
    uVar1    = *(undefined4 *)(iVar2 + 0x1a);
    uStack8  = *(undefined2 *)((int)uVar1 + 0xc);
    BStack10 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x3);
    if((BStack10 != 0x0) && (uVar1 = *(undefined4 *)(iVar2 + 0x1a), *(int *)((int)uVar1 + 0x12) == 0x5))
    {
        uVar1    = *(undefined4 *)(iVar2 + 0x22);
        uStack14 = *(uint *)((int)uVar1 + 0x4);
        for(uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1)
        {
            pass1_1020_bb16(*(ulong **)(iVar2 + 0x22),
                            (ulong *)CONCAT22(param_2, &local_1a),
                            (ushort *)CONCAT22(param_2, local_16),
                            uStack18);
            if(0x0 < local_16[0])
            {
                lStack6 = lStack6 + local_1a;
            }
        }
    }
    return;
}
