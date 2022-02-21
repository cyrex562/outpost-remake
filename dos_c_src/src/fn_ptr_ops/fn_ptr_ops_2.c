
astruct_18 *__stdcall16far pass1_1038_3074(astruct_18 *param_1, byte param_2)

{
    pass1_1038_2a5c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1038_33f8(ushort *param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                        = (int)param_1;
    *param_1                     = 0x6504;
    *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1038;
    puVar1                       = (undefined4 *)*(uint *)(iVar4 + 0x14);
    uVar2                        = *(uint *)(iVar4 + 0x16);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x1f6);
    uVar2  = *(uint *)(iVar4 + 0x1f8);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x1fa), 0x1000);
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x210);
    uVar2  = *(uint *)(iVar4 + 0x212);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(0x1000, puVar1, uVar2, 0x1);
    }
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x21a), 0x1000);
    pass1_1030_18b2(param_1);
    return;
}


void __stdcall16far
pass1_1038_11b0(ulong param_1, ulong *param_2, ulong *param_3, ushort param_4, ulong param_5, ushort param_6)

{
    code **ppcVar1;
    ushort uVar2;
    ulong  uVar3;
    ulong  uStack10;
    ulong  uStack6;

    ppcVar1 = (code **)((int)*param_3 + 0x10);
    (**ppcVar1)();
    uStack6  = CONCAT22((int)param_5, param_4);
    uStack10 = 0x0;
    while(true)
    {
        if(uStack6 <= uStack10)
        {
            return;
        }
        ppcVar1 = (code **)((int)*param_3 + 0x4);
        uVar3   = uStack6;
        (**ppcVar1)();
        uVar2 = (ushort)uVar3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uint)param_5);
        uVar3   = struct_op_1030_73a8(CONCAT22((int)param_5, uVar2));
        param_5 = param_5 & 0xffff0000 | uVar3 >> 0x10;
        uVar2   = (ushort)uVar3;
        pass1_1038_0f8c((ushort)param_1, (ushort)(param_1 >> 0x10), param_2, uVar3, uVar2, param_5, 0x1030, param_6);
        if(uVar2 == 0x0)
            break;
        uStack10 = uStack10 + 0x1;
    }
    return;
}


void __stdcall16far pass1_1038_1220(ulong param_1, ulong param_2, ulong param_3, ushort param_4)

{
    code      **ppcVar1;
    undefined4  uVar2;
    uint        uVar3;
    uint        uVar4;
    uint        uVar5;
    uchar      *puVar6;
    uchar      *puVar7;
    uchar      *puVar8;
    undefined2  uVar10;
    ulong       uVar9;
    ulong      *puVar11;
    undefined   uVar12;
    undefined4 *puStack14;
    undefined4 *puStack10;

    uVar10  = (undefined2)(param_3 >> 0x10);
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
    puVar6  = (uchar *)((ulong)puVar11 >> 0x10);
    uVar3   = (uint)puVar11;
    pass1_1038_4d6e(param_2, puVar11, uVar3, puVar6);
    puStack10 = (undefined4 *)CONCAT22(puVar6, uVar3);
    ppcVar1   = (code **)((int)*puStack10 + 0x10);
    puVar7    = puVar6;
    uVar4     = uVar3;
    (**ppcVar1)(0x1008, uVar3, puVar6);
    if((puVar7 != (uchar *)0x0) || (uVar4 != 0x0))
    {
        puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x5);
        puVar8  = (uchar *)((ulong)puVar11 >> 0x10);
        uVar4   = (uint)puVar11;
        pass1_1038_4d6e(param_2, puVar11, uVar4, puVar8);
        puStack14 = (undefined4 *)CONCAT22(puVar8, uVar4);
        uVar12    = (undefined)uVar4;
        uVar2     = *puStack14;
        ppcVar1   = (code **)uVar2 + 0x8;
        puVar7    = puVar8;
        uVar5     = uVar4;
        (**ppcVar1)(0x1008, uVar12, puVar8);
        uVar9 = CONCAT22(uVar10, puVar7);
        if(((puVar7 != (uchar *)0x0) || (uVar5 != 0x0))
           && (pass1_1038_11b0(param_1,
                               (ulong *)CONCAT13((char)((uint)puVar6 >> 0x8), CONCAT12((char)puVar6, uVar3)),
                               (ulong *)CONCAT22(puVar8, uVar4),
                               uVar5,
                               uVar9,
                               param_4),
               uVar5 == 0x0))
        {
            if(puStack14 == (undefined4 *)0x0)
            {
                return;
            }
            ppcVar1 = (code **)uVar2;
            (**ppcVar1)(0x8, uVar12, (char)puVar8, 0x1);
            return;
        }
        uVar10 = (undefined2)(uVar9 >> 0x10);
        if(puStack14 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack14;
            (**ppcVar1)(0x8, uVar12, (char)puVar8, 0x1);
        }
        puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x6);
        puVar8  = (uchar *)((ulong)puVar11 >> 0x10);
        uVar4   = (uint)puVar11;
        pass1_1038_4d6e(param_2, puVar11, uVar4, puVar8);
        puStack14 = (undefined4 *)CONCAT22(puVar8, uVar4);
        ppcVar1   = (code **)((int)*puStack14 + 0x10);
        puVar7    = puVar8;
        uVar5     = uVar4;
        (**ppcVar1)(0x8, (char)uVar4, puVar8);
        if((puVar7 != (uchar *)0x0) || (uVar5 != 0x0))
        {
            pass1_1038_11b0(param_1,
                            (ulong *)CONCAT22(puVar6, uVar3),
                            (ulong *)CONCAT22(puVar8, uVar4),
                            uVar5,
                            CONCAT22(uVar10, puVar7),
                            param_4);
        }
        if(puStack14 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack14;
            (**ppcVar1)(0x8, uVar4, (char)puVar8, 0x1);
        }
    }
    if(puStack10 != (undefined4 *)0x0)
    {
        ppcVar1 = (code **)*puStack10;
        (**ppcVar1)(0x8, uVar3, (char)puVar6, 0x1);
    }
    return;
}


void __stdcall16far pass1_1038_134a(
  ushort param_1, ushort param_2, ulong *param_3, ulong *param_4, ulong *param_5, undefined2 param_6, ushort param_7)

{
    code     **ppcVar1;
    ushort     uVar2;
    undefined2 extraout_DX;
    uint       extraout_DX_00;
    uint       uVar3;
    ushort     unaff_SS;
    ulong      uVar4;
    ulong     *puVar5;
    ulong      uStack6;

    ppcVar1 = (code **)((int)*param_5 + 0x10);
    puVar5  = param_5;
    (**ppcVar1)();
    uStack6  = CONCAT22(extraout_DX, param_6);
    *param_3 = 0x0;
    do
    {
        if(uStack6 <= *param_4)
        {
            return;
        }
        uVar4    = *param_4;
        *param_4 = *param_4 + 0x1;
        ppcVar1  = (code **)((int)*param_5 + 0x4);
        (**ppcVar1)(param_7, param_5, uVar4, puVar5);
        uVar2 = (ushort)uVar4;
        uVar3 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
        uVar4           = struct_op_1030_73a8(CONCAT22(uVar3, uVar2));
        uVar3           = (uint)(uVar4 >> 0x10);
        param_7         = (ushort)&USHORT_1050_1028;
        uVar4           = pass1_1028_45e2(uVar4 & 0xffff | (ulong)uVar3 << 0x10, (uint)uVar4, uVar3, unaff_SS);
        uVar3           = (uint)(uVar4 >> 0x10);
        *(int *)param_3 = (int)uVar4;
        *(uint *)((int)param_3 + 0x2) = uVar3;
    } while((uVar3 | *(uint *)param_3) == 0x0);
    return;
}


void __stdcall16far pass1_1038_13da(
  ushort param_1, ushort param_2, ulong *param_3, ulong *param_4, ulong *param_5, ushort param_6, ushort param_7)

{
    code     **ppcVar1;
    uint       uVar2;
    undefined2 extraout_DX;
    uint       extraout_DX_00;
    uint       uVar3;
    ulong      uVar4;
    ulong     *puVar5;
    ulong      uStack6;

    ppcVar1 = (code **)((int)*param_5 + 0x10);
    puVar5  = param_5;
    (**ppcVar1)();
    uStack6  = CONCAT22(extraout_DX, param_6);
    *param_3 = 0x0;
    do
    {
        if(uStack6 <= *param_4)
        {
            return;
        }
        uVar4    = *param_4;
        *param_4 = *param_4 + 0x1;
        ppcVar1  = (code **)((int)*param_5 + 0x4);
        (**ppcVar1)(param_7, param_5, uVar4, puVar5);
        uVar2 = (uint)uVar4;
        uVar3 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
        if((uVar3 | uVar2) == 0x0)
        {
            return;
        }
        uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, uVar2));
        uVar3 = (uint)(uVar4 >> 0x10);
        if((uVar3 | (uint)uVar4) == 0x0)
        {
            return;
        }
        param_7                       = (ushort)&USHORT_1050_1028;
        uVar4                         = pass1_1028_3c32((ulong *)(uVar4 & 0xffff | (ulong)uVar3 << 0x10));
        uVar3                         = (uint)(uVar4 >> 0x10);
        *(int *)param_3               = (int)uVar4;
        *(uint *)((int)param_3 + 0x2) = uVar3;
    } while((uVar3 | *(uint *)param_3) == 0x0);
    return;
}


void __stdcall16far pass1_1038_1482(ulong  param_1,
                                    ulong *param_2,
                                    ulong *param_3,
                                    ushort param_4,
                                    ushort param_5,
                                    ushort param_6,
                                    ushort param_7,
                                    ushort param_8)

{
    code      **ppcVar1;
    sqword      sVar2;
    ushort      uVar3;
    undefined4 *puVar4;
    uint        uVar5;
    ushort      uVar6;
    ulong       uVar7;
    uchar      *puVar8;
    uchar      *puVar9;
    uint        uVar10;
    ushort      uVar11;
    undefined   uVar12;
    undefined   uVar13;
    undefined2  uVar14;
    long        lStack74;
    undefined4  local_46;
    int         local_42[0x4];
    uint        uStack58;
    uint        uStack56;
    ulong      *puStack54;
    ulong      *puStack50;
    ulong       uStack46;
    ushort      uStack42;
    uint        uStack40;
    ushort      uStack38;
    uint        uStack36;
    undefined4  uStack34;
    uint        uStack30;
    uint        uStack28;
    uint        uStack26;
    uint        uStack24;
    ulong       uStack22;
    undefined4  uStack18;
    undefined4  uStack14;
    undefined4  local_a;
    undefined4  local_6;

    local_6 = 0x0;
    local_a = 0x0;
    puVar4  = &local_a;
    uVar11  = (ushort)(param_1 >> 0x10);
    uVar3   = (ushort)param_1;
    pass1_1038_134a(uVar3,
                    uVar11,
                    (ulong *)CONCAT22(param_6, puVar4),
                    (ulong *)CONCAT22(param_6, &local_6),
                    param_3,
                    puVar4,
                    param_4);
    uStack14 = CONCAT22(param_5, puVar4);
    ppcVar1  = (code **)((int)*param_2 + 0x10);
    (**ppcVar1)(param_4, param_2);
    uStack18 = CONCAT22(param_5, puVar4);
    uStack22 = 0x0;
    do
    {
        if(uStack18 <= uStack22)
        {
            return;
        }
        uStack14._2_2_ = uStack14._2_2_ | (uint)uStack14;
        if(uStack14._2_2_ == 0x0)
        {
            return;
        }
        pass1_1028_b58e(uStack14);
        uStack26 = uStack14._2_2_;
        uStack24 = uStack18._2_2_;
        pass1_1038_1a30(uVar3, uVar11, CONCAT22(uStack18._2_2_, uStack14._2_2_), (ushort)&USHORT_1050_1028);
        uStack30 = uStack14._2_2_;
        uStack28 = uStack18._2_2_;
        if((uStack18._2_2_ | uStack14._2_2_) != 0x0)
        {
            sVar2    = (qword)CONCAT22(uStack18._2_2_, uStack14._2_2_) * 0x64;
            uVar5    = (uint)((qword)sVar2 >> 0x20);
            uVar7    = (ulong)sVar2 >> 0x1;
            ppcVar1  = (code **)((int)*param_2 + 0x4);
            uStack34 = uVar7;
            (**ppcVar1)((int)&USHORT_1050_1028, param_2, (char)uStack22, (int)(uStack22 >> 0x10));
            uVar6    = (ushort)uVar7;
            uStack38 = uVar6;
            uStack36 = uVar5;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6, uVar5);
            uStack42  = uVar6;
            uStack40  = uVar5;
            uStack46  = struct_op_1030_73a8(CONCAT22(uVar5, uVar6));
            puStack50 = *(ulong **)((int)uStack46 + 0x28);
            puStack54 = (ulong *)0x0;
            uStack56  = *(uint *)((int)puStack50 + 0x4);
            for(uStack58 = 0x0; uVar5 = uStack56, uStack58 < uStack56; uStack58 = uStack58 + 0x1)
            {
                pass1_1020_bb16(puStack50,
                                (ulong *)CONCAT22(param_6, &local_46),
                                (ushort *)CONCAT13((char)(param_6 >> 0x8), CONCAT12((char)param_6, local_42)),
                                uStack58);
                if(((local_46 != 0x0) && (0xd < local_42[0])) && (local_42[0] < 0x1d))
                {
                    uVar7 = local_46;
                    if(uStack34 < local_46)
                    {
                        uVar7          = uStack34 & 0xffff;
                        local_46._2_2_ = uStack34._2_2_;
                    }
                    uVar5 = (uint)uVar7;
                    if((local_a._2_2_ <= local_46._2_2_)
                       && ((local_a._2_2_ < local_46._2_2_ || ((uint)local_a < uVar5))))
                    {
                        uVar5          = (uint)local_a;
                        local_46._2_2_ = local_a._2_2_;
                    }
                    lStack74 = CONCAT22(local_46._2_2_, uVar5);
                    uStack34 = CONCAT22(uStack34._2_2_ + (-(uint)((uint)uStack34 < uVar5) - (int)local_46._2_2_),
                                        (uint)uStack34 - uVar5);
                    local_a  = CONCAT22(local_a._2_2_ + (-(uint)((uint)local_a < uVar5) - (int)local_46._2_2_),
                                       (uint)local_a - uVar5);
                    puVar9   = local_46._2_2_;
                    if(puStack54 == (ulong *)0x0)
                    {
                        puVar8 = local_46._2_2_;
                        uVar10 = uVar5;
                        mem_op_1000_179c(0xa, local_46._2_2_, 0x1000);
                        puVar9 = (uchar *)((uint)puVar8 | uVar10);
                        if(puVar9 == (uchar *)0x0)
                        {
                            uVar10 = 0x0;
                            puVar9 = (uchar *)0x0;
                        }
                        else
                        {
                            pass1_1020_ba3e((long *)CONCAT22(puVar8, uVar10), 0x5, 0x5, param_8, param_7);
                        }
                        puStack54 = (ulong *)CONCAT22(puVar9, uVar10);
                    }
                    pass1_1020_bb8a((long *)puStack54, uVar5, CONCAT22(local_42[0], local_46._2_2_), param_8, param_6);
                    uVar7 = local_46 - lStack74;
                    pass1_1020_bb8a(
                      (long *)puStack50, (uint)uVar7, CONCAT22(local_42[0], (int)(uVar7 >> 0x10)), param_8, param_6);
                    if(local_a == 0x0)
                    {
                        pass1_1038_1b3a(uVar3, uVar11, uStack14, puStack54, param_6, (ushort)uVar7, param_7, param_8);
                        puStack54 = (ulong *)0x0;
                        uVar7     = ZEXT24(&local_a);
                        pass1_1038_134a(uVar3,
                                        uVar11,
                                        (ulong *)CONCAT22(param_6, &local_a),
                                        (ulong *)CONCAT22(param_6, &local_6),
                                        param_3,
                                        &local_a,
                                        0x1020);
                        uVar5    = (uint)uVar7;
                        uStack14 = uVar7 & 0xffff | ZEXT24(puVar9) << 0x10;
                        uVar10   = (uint)puVar9 | uVar5;
                        if(uVar10 != 0x0)
                        {
                            uVar12 = 0x64;
                            uVar13 = 0x0;
                            uVar14 = 0x0;
                            pass1_1028_b58e(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10);
                            uStack26 = uVar5;
                            uStack24 = uVar10;
                            pass1_1038_1a30(uVar3, uVar11, CONCAT22(uVar10, uVar5), (ushort)&USHORT_1050_1028);
                            uVar7
                              = (ulong)(CONCAT22(uVar10, uVar5) * CONCAT22(uVar14, CONCAT11(uVar13, uVar12))) >> 0x1;
                            uStack34 = uVar7;
                            uStack30 = uVar5;
                            uStack28 = uVar10;
                        }
                    }
                    uVar5 = (uint)uVar7;
                    if((uStack34 == 0x0) || (local_a == 0x0))
                        break;
                }
            }
            if(puStack54 != (ulong *)0x0)
            {
                pass1_1038_1b3a(uVar3, uVar11, uStack14, puStack54, param_6, uVar5, param_7, param_8);
                puStack54 = (ulong *)0x0;
            }
        }
        uStack22 = uStack22 + 0x1;
    } while(true);
}


void __stdcall16far pass1_1038_16f2(ulong  param_1,
                                    ulong *param_2,
                                    ulong *param_3,
                                    uint   param_4,
                                    ushort param_5,
                                    ushort param_6,
                                    ushort param_7,
                                    ushort param_8,
                                    uchar  param_9)

{
    long       *plVar1;
    code      **ppcVar2;
    ushort      uVar3;
    undefined4 *puVar4;
    ushort      uVar5;
    undefined4 *puVar6;
    undefined4 *puVar7;
    uint        uVar8;
    uint        uVar9;
    uint        uVar10;
    uint        uVar11;
    uchar      *puVar12;
    ulong       uVar13;
    ulong       uVar14;
    ulong       uVar15;
    long        lVar16;
    ushort      uVar17;
    long        lStack68;
    undefined4 *puStack56;
    undefined4 *puStack52;
    long       *plStack50;
    uint        uStack46;
    undefined4  uStack42;
    ulong       uStack22;
    ulong       uStack18;
    ulong       uStack14;
    undefined4  local_a;
    undefined4  local_6;

    local_6 = 0x0;
    local_a = 0x0;
    puVar6  = &local_a;
    uVar17  = (ushort)(param_1 >> 0x10);
    uVar3   = (ushort)param_1;
    pass1_1038_13da(uVar3,
                    uVar17,
                    (ulong *)CONCAT22(param_8, puVar6),
                    (ulong *)CONCAT22(param_8, &local_6),
                    param_3,
                    (ushort)puVar6,
                    param_7);
    uStack14 = CONCAT22(param_4, puVar6);
    uVar8    = param_4 | (uint)puVar6;
    if(uVar8 != 0x0)
    {
        ppcVar2 = (code **)((int)*param_2 + 0x10);
        (**ppcVar2)(param_7, param_2);
        uStack18 = CONCAT22(uVar8, puVar6);
        for(uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1)
        {
            ppcVar2 = (code **)((int)*param_2 + 0x4);
            uVar15  = uStack18;
            uVar10  = uVar8;
            (**ppcVar2)(param_7, param_2, (char)uStack22, (int)(uStack22 >> 0x10));
            uVar5 = (ushort)uVar15;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, uVar10);
            param_7 = 0x1030;
            uVar15  = struct_op_1030_73a8(CONCAT22(uVar10, uVar5));
            uVar11  = (uint)(uVar15 >> 0x10);
            uVar9   = (uint)uVar15;
            pass1_1038_1a30(uVar3, uVar17, CONCAT22(uVar10, uVar5), 0x1030);
            if((uVar11 | uVar9) != 0x0)
            {
                uStack42 = (ulong)(CONCAT22(uVar11, uVar9) * 0x64) >> 0x1;
                plVar1   = *(long **)(uVar5 + 0x22);
                uVar9    = *(uint *)(uVar5 + 0x24);
                uVar13   = (ulong)uVar9;
                uStack46 = (uint)plVar1;
                if((uVar9 | uStack46) != 0x0)
                {
                    plStack50 = (long *)0x0;
                    puVar6    = (undefined4 *)pass1_1028_0d80(uVar15);
                    puStack56 = (undefined4 *)0x0;
                    puStack52 = puVar6;
                    while(true)
                    {
                        lVar16 = pass1_1020_bae6(uStack46,
                                                 CONCAT22(puStack52, (int)((ulong)plVar1 >> 0x10)),
                                                 (uint)puStack52,
                                                 (uint)uVar13,
                                                 param_8);
                        uVar9  = (uint)((ulong)lVar16 >> 0x10);
                        puVar7 = (undefined4 *)lVar16;
                        uVar13 = (ulong)(uVar9 | (uint)puVar7);
                        if((uVar9 | (uint)puVar7) != 0x0)
                        {
                            uVar14 = (ulong)uVar9;
                            if((uStack42._2_2_ <= uVar9)
                               && ((uStack42._2_2_ < uVar9 || ((undefined4 *)uStack42 < puVar7))))
                            {
                                uVar14 = (ulong)uStack42._2_2_;
                                puVar7 = (undefined4 *)uStack42;
                            }
                            if((local_a._2_2_ <= (uint)uVar14)
                               && ((local_a._2_2_ < (uint)uVar14 || ((undefined4 *)local_a < puVar7))))
                            {
                                uVar14 = (ulong)local_a._2_2_;
                                puVar7 = (undefined4 *)local_a;
                            }
                            puVar12  = (uchar *)uVar14;
                            lStack68 = CONCAT22(puVar12, puVar7);
                            uStack42
                              = CONCAT22((uStack42._2_2_ - (int)puVar12) - (uint)((undefined4 *)uStack42 < puVar7),
                                         (int)(undefined4 *)uStack42 - (int)puVar7);
                            local_a = CONCAT22((local_a._2_2_ - (int)puVar12) - (uint)((undefined4 *)local_a < puVar7),
                                               (int)(undefined4 *)local_a - (int)puVar7);
                            uVar13  = uVar14;
                            if(plStack50 == (long *)0x0)
                            {
                                puVar4 = puVar7;
                                mem_op_1000_179c(0xa, puVar12, 0x1000);
                                uVar13 = (ulong)((uint)puVar12 | (uint)puVar4);
                                if(((uint)puVar12 | (uint)puVar4) == 0x0)
                                {
                                    puVar4 = (undefined4 *)0x0;
                                    uVar13 = 0x0;
                                }
                                else
                                {
                                    pass1_1020_ba3e((long *)CONCAT22(puVar12, puVar4), 0x5, 0x5, param_6, param_5);
                                }
                                plStack50 = (long *)CONCAT22((int)uVar13, puVar4);
                            }
                            pass1_1020_bb8a(
                              plStack50, (uint)puVar7, uVar14 | ZEXT24(puStack52) << 0x10, param_6, param_8);
                            pass1_1020_bb8a(plVar1,
                                            (uint)(lVar16 - lStack68),
                                            CONCAT22(puStack52, (int)((ulong)(lVar16 - lStack68) >> 0x10)),
                                            param_6,
                                            param_8);
                            uVar9     = (uint)uVar13;
                            puStack56 = puStack52;
                            puVar7    = puStack52;
                            if(local_a == 0x0)
                            {
                                pass1_1038_1ac6(
                                  uVar3, uVar17, uStack14, (ulong)plStack50, (int)puStack52, param_8, param_9);
                                plStack50 = (long *)0x0;
                                puVar7    = &local_a;
                                pass1_1038_13da(uVar3,
                                                uVar17,
                                                (ulong *)CONCAT22(param_8, puVar7),
                                                (ulong *)CONCAT22(param_8, &local_6),
                                                param_3,
                                                (ushort)puVar7,
                                                0x1020);
                                uStack14 = CONCAT22(uVar9, puVar7);
                                uVar13   = (ulong)(uVar9 | (uint)puVar7);
                                if((uVar9 | (uint)puVar7) == 0x0)
                                {
                                    return;
                                }
                            }
                        }
                        param_7 = 0x1020;
                        if((uStack42 == 0x0) || (local_a == 0x0))
                            break;
                        param_7 = (ushort)&USHORT_1050_1028;
                        puVar7  = (undefined4 *)pass1_1028_0d80(uVar15);
                        if((puVar7 == puStack56)
                           || ((puStack52 = puVar7, puStack56 == (undefined4 *)0x0 && (puVar7 == puVar6))))
                            break;
                    }
                    if(plStack50 != (long *)0x0)
                    {
                        pass1_1038_1ac6(uVar3, uVar17, uStack14, (ulong)plStack50, (int)puVar7, param_8, param_9);
                    }
                }
            }
        }
    }
    return;
}


void __stdcall16far
pass1_1038_1940(ulong param_1, ulong *param_2, ulong param_3, ushort param_4, ushort param_5, ushort param_6)

{
    code **ppcVar1;
    uint   uVar2;
    uint   uVar3;
    uchar *puVar4;
    uint   extraout_DX;
    ulong *puVar5;
    ulong *puStack10;

    puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x3);
    puVar4 = (uchar *)((ulong)puVar5 >> 0x10);
    uVar2  = (uint)puVar5;
    pass1_1038_4d6e(param_3, puVar5, uVar2, puVar4);
    puStack10 = (ulong *)CONCAT22(puVar4, uVar2);
    ppcVar1   = (code **)((int)*puStack10 + 0x10);
    uVar3     = uVar2;
    (**ppcVar1)(0x1008, uVar2, puVar4);
    if((extraout_DX | uVar3) != 0x0)
    {
        pass1_1038_1482(param_1, param_2, puStack10, 0x1008, extraout_DX | uVar3, param_6, param_4, param_5);
    }
    if(puStack10 != (ulong *)0x0)
    {
        ppcVar1 = (code **)*puStack10;
        (**ppcVar1)(0x1008, uVar2, (char)puVar4, 0x1);
    }
    return;
}


void __stdcall16far pass1_1038_1b3a(ushort param_1,
                                    ushort param_2,
                                    ulong  param_3,
                                    ulong *param_4,
                                    ushort param_5,
                                    ushort param_6,
                                    ushort param_7,
                                    ushort param_8)

{
    int        extraout_DX;
    ulong      local_1a;
    ushort     local_16[0x2];
    uint       uStack18;
    uint       uStack16;
    undefined4 uStack14;
    ulong      uStack10;
    ulong      uStack6;

    pass1_1028_b58e(param_3);
    uStack6  = CONCAT22(extraout_DX, param_6);
    uStack10 = param_3;
    uStack14 = pass1_1028_45e2(param_3, (uint)param_3, extraout_DX, param_5);
    uStack16 = *(uint *)((int)param_4 + 0x4);
    for(uStack18 = 0x0; uStack18 < uStack16; uStack18 = uStack18 + 0x1)
    {
        pass1_1020_bb16(
          param_4, (ulong *)CONCAT22(param_5, &local_1a), (ushort *)CONCAT22(param_5, local_16), uStack18);
        if(uStack14 < local_1a)
        {
            pass1_1030_7ddc(uStack6, uStack14, local_16[0], (uint)uStack14, uStack14._2_2_, param_7, param_8, param_5);
            uStack14 = 0x0;
        }
        else
        {
            uStack14 = uStack14 - local_1a;
            pass1_1030_7ddc(uStack6, local_1a, local_16[0], (uint)local_1a, uStack14._2_2_, param_7, param_8, param_5);
        }
        if(uStack14 == 0x0)
            break;
    }
    if(param_4 != (ulong *)0x0)
    {
        fn_ptr_1020_ba7e(param_4);
        fn_ptr_1000_17ce((astruct_18 *)param_4, 0x1000);
    }
    return;
}


astruct_18 *__stdcall16far pass1_1038_1c02(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far
pass1_1038_0340(ushort param_1, ushort param_2, int param_3, ulong param_4, uint param_5, ushort param_6, uchar param_7)

{
    uint       uVar1;
    ulong      uVar2;
    int        iVar3;
    undefined2 uVar4;
    undefined  local_13a[0x11c];
    undefined4 uStack30;
    ulong      uStack26;
    ulong      uStack22;
    uint       local_12;
    uint       uStack16;
    int        local_e;
    ushort     uStack12;
    undefined4 uStack10;
    ulong      uStack6;

    uStack6  = *_PTR_LOOP_1050_65e2;
    uStack10 = 0x0;
    uStack12 = 0x0;
    iVar3    = (int)param_4;
    uVar4    = (undefined2)(param_4 >> 0x10);
    pass1_1038_4cea(param_4, (ulong *)CONCAT22(param_6, &local_12), (ushort *)CONCAT22(param_6, &local_e));
    uVar2    = *(ulong *)(iVar3 + 0x1f6);
    uStack22 = uVar2;
    pass1_1030_38b8();
    uVar1    = (uint)uVar2;
    uStack26 = uVar2 & 0xffff | (ulong)param_5 << 0x10;
    if(param_3 == 0x0)
    {
        if(local_e != 0x8)
        {
            uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0x4;
            uStack12 = 0x8;
            goto LAB_1038_054b;
        }
    }
    else
    {
        if(param_3 < 0xb)
        {
            if(local_e != 0x7)
            {
                uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0xa;
                uStack12 = 0x7;
                goto LAB_1038_054b;
            }
        }
        else
        {
            if(param_3 < 0x1a)
            {
                if(local_e != 0x6)
                {
                    uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0x14;
                    uStack12 = 0x6;
                    goto LAB_1038_054b;
                }
            }
            else
            {
                if(param_3 < 0x33)
                {
                    if(local_e != 0x5)
                    {
                        uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0x64;
                        uStack12 = 0x5;
                        goto LAB_1038_054b;
                    }
                }
                else
                {
                    if(param_3 < 0x4c)
                    {
                        if(uStack6 % 0x3 != 0x0)
                            goto LAB_1038_054b;
                        if(local_e != 0x4)
                        {
                            uStack10 = (long)uStack26 / 0x64;
                            uStack12 = 0x4;
                            goto LAB_1038_054b;
                        }
                    }
                    else
                    {
                        if(param_3 < 0x65)
                        {
                            if(uStack6 % 0x5 != 0x0)
                                goto LAB_1038_054b;
                            if(local_e != 0x3)
                            {
                                uStack10 = (long)uStack26 / 0x64;
                                uStack12 = 0x3;
                                goto LAB_1038_054b;
                            }
                        }
                        else
                        {
                            if(param_3 < 0x97)
                            {
                                if(uStack6 % 0xa != 0x0)
                                    goto LAB_1038_054b;
                                if(local_e != 0x2)
                                {
                                    uStack10 = (long)uStack26 / 0x64;
                                    uStack12 = 0x2;
                                    goto LAB_1038_054b;
                                }
                            }
                            else
                            {
                                if((0xc8 < param_3) || (uStack6 % 0x14 != 0x0))
                                    goto LAB_1038_054b;
                                if(local_e != 0x1)
                                {
                                    uStack10 = (long)uStack26 / 0x64;
                                    uStack12 = 0x1;
                                    goto LAB_1038_054b;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if(((int)uStack16 <= (int)param_5) && (((int)uStack16 < (int)param_5 || (local_12 < uVar1))))
    {
        uVar1   = local_12;
        param_5 = uStack16;
    }
    uStack10 = CONCAT22(param_5, uVar1);
LAB_1038_054b:
    if(uStack12 != 0x0)
    {
        if((uStack26 != 0x0) && (uStack10 == 0x0))
        {
            uStack10 = 0x1;
        }
        pass1_1038_4cd0(param_4, uStack10, uStack12);
    }
    if((uStack10._2_2_ | (uint)uStack10) != 0x0)
    {
        if(*(long *)(iVar3 + 0x200) == 0x8000001)
        {
            uStack30._0_2_ = 0x2;
        }
        else
        {
            uStack30._0_2_ = 0x1;
        }
        uStack30 = CONCAT22(0x400, (undefined2)uStack30);
        pass1_1028_9944((astruct_100 *)CONCAT22(param_6, local_13a),
                        uStack10,
                        CONCAT22(0x400, (undefined2)uStack30),
                        *(ulong *)(iVar3 + 0x4),
                        param_6,
                        param_7);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, local_13a));
        pass1_1028_9992((ushort *)CONCAT22(param_6, local_13a));
    }
    return;
}


void __stdcall16far pass1_1038_05d8(
  ushort param_1, ushort param_2, int param_3, ulong param_4, ulong param_5, ushort param_6, uchar param_7)

{
    undefined2 *puVar1;
    ulong       uVar2;
    uint        uVar3;
    uint        uVar4;
    undefined2  uVar5;
    undefined   local_158[0x118];
    ulong       uStack64;
    undefined2  local_34;
    undefined2  uStack50;
    undefined4  uStack34;
    undefined4  uStack30;
    ulong       uStack26;
    undefined4  uStack22;
    undefined4  local_12;
    int         local_e;
    ushort      uStack12;
    undefined4  uStack10;
    ulong       uStack6;

    uStack6  = *_PTR_LOOP_1050_65e2;
    uStack10 = 0x0;
    uStack12 = 0x0;
    pass1_1038_4cea(param_4, (ulong *)CONCAT22(param_6, &local_12), (ushort *)CONCAT22(param_6, &local_e));
    uStack22 = 0x0;
    uStack26 = 0x0;
    uStack30 = 0x0;
    pass1_1028_dc52(
      (astruct_92 *)CONCAT13((char)(param_6 >> 0x8), CONCAT12((char)param_6, &local_34)), 0x1, 0x0, 0x400);
    do
    {
        do
        {
            uVar3  = (uint)param_5;
            puVar1 = &local_34;
            pass1_1028_e4ec(CONCAT22(param_6, puVar1));
            uStack34 = CONCAT22(uVar3, puVar1);
            uVar4    = uVar3 | (uint)puVar1;
            param_5  = (ulong)uVar4;
            if(uVar4 == 0x0)
                goto LAB_1038_0668;
        } while(*(long *)(puVar1 + 0x100) != 0x8000002);
        uStack22 = CONCAT22(uVar3, puVar1);
        uVar2    = *(ulong *)(puVar1 + 0xfb);
        uStack26 = uVar2;
        pass1_1030_38b8();
        uStack30 = uVar2 & 0xffff | (ulong)uVar4 << 0x10;
        uVar4    = uVar4 | (uint)uVar2;
        param_5  = (ulong)uVar4;
    } while(uVar4 == 0x0);
LAB_1038_0668:
    local_34 = 0x389a;
    uStack50 = 0x1008;
    if((uStack22._2_2_ | (uint)uStack22) == 0x0)
    {
        return;
    }
    if(param_3 == 0x3e8)
    {
        if(local_e != 0x10)
        {
            uStack10 = (long)uStack30 / 0x4;
            uStack12 = 0x10;
            goto LAB_1038_0841;
        }
    }
    else
    {
        if(param_3 < 0x3de)
        {
            if(param_3 < 0x3cf)
            {
                if(param_3 < 0x3b6)
                {
                    if(param_3 < 0x39d)
                    {
                        if(param_3 < 0x384)
                        {
                            if(param_3 < 0x352)
                            {
                                if((param_3 < 0x320) || (uStack6 % 0x14 != 0x0))
                                    goto LAB_1038_0841;
                                if(local_e != 0x9)
                                {
                                    uStack10 = (long)uStack30 / 0x64;
                                    uStack12 = 0x9;
                                    goto LAB_1038_0841;
                                }
                            }
                            else
                            {
                                if(uStack6 % 0xa != 0x0)
                                    goto LAB_1038_0841;
                                if(local_e != 0xa)
                                {
                                    uStack10 = (long)uStack30 / 0x64;
                                    uStack12 = 0xa;
                                    goto LAB_1038_0841;
                                }
                            }
                        }
                        else
                        {
                            if(uStack6 % 0x5 != 0x0)
                                goto LAB_1038_0841;
                            if(local_e != 0xb)
                            {
                                uStack10 = (long)uStack30 / 0x64;
                                uStack12 = 0xb;
                                goto LAB_1038_0841;
                            }
                        }
                    }
                    else
                    {
                        if(uStack6 % 0x3 != 0x0)
                            goto LAB_1038_0841;
                        if(local_e != 0xc)
                        {
                            uStack10 = (long)uStack30 / 0x64;
                            uStack12 = 0xc;
                            goto LAB_1038_0841;
                        }
                    }
                }
                else
                {
                    if(local_e != 0xd)
                    {
                        uStack10 = (long)uStack30 / 0x64;
                        uStack12 = 0xd;
                        goto LAB_1038_0841;
                    }
                }
            }
            else
            {
                if(local_e != 0xe)
                {
                    uStack10 = (long)uStack30 / 0x14;
                    uStack12 = 0xe;
                    goto LAB_1038_0841;
                }
            }
        }
        else
        {
            if(local_e != 0xf)
            {
                uStack10 = (long)uStack30 / 0xa;
                uStack12 = 0xf;
                goto LAB_1038_0841;
            }
        }
    }
    uVar2 = uStack30;
    if((long)local_12 < (long)uStack30)
    {
        uVar2          = local_12 & 0xffff;
        uStack30._2_2_ = local_12._2_2_;
    }
    uStack10 = uVar2 & 0xffff | (ulong)uStack30._2_2_ << 0x10;
LAB_1038_0841:
    if(uStack12 != 0x0)
    {
        if((uStack30 != 0x0) && (uStack10 == 0x0))
        {
            uStack10 = 0x1;
        }
        pass1_1038_4cd0(param_4, uStack10, uStack12);
    }
    if((uStack10._2_2_ | (uint)uStack10) != 0x0)
    {
        uVar5 = (undefined2)(param_4 >> 0x10);
        if(*(long *)((int)param_4 + 0x200) == 0x8000001)
        {
            uStack64 = *(ulong *)((int)uStack22 + 0x4);
        }
        else
        {
            uStack64 = 0x4000001;
        }
        pass1_1028_9944((astruct_100 *)CONCAT22(param_6, local_158),
                        uStack10,
                        *(ulong *)((int)param_4 + 0x4),
                        uStack64,
                        param_6,
                        param_7);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, local_158));
        pass1_1028_9992((ushort *)CONCAT22(param_6, local_158));
    }
    return;
}


astruct_18 *__stdcall16far pass1_1038_0b6a(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_e010(astruct_18 *param_1, byte param_2)

{
    uint in_AX;

    pass1_1030_dcf4(&param_1->field_0x0, in_AX);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_e282(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_e4be(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_e602(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_e75e(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_e864(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


ushort __stdcall16far
pass1_1030_e8f8(ulong param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6)

{
    uint        uVar1;
    uint        uVar2;
    undefined4  uVar3;
    int         iVar4;
    undefined2  uVar5;
    ulong       uVar6;
    astruct_18 *paStack20;
    ulong       uStack6;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0x108) != 0x0)
    {
        uVar3 = *(undefined4 *)(iVar4 + 0x10c);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar3, (uint)((ulong)uVar3 >> 0x10));
        uStack6 = CONCAT22(param_3, param_2);
        uVar6   = struct_op_1030_73a8(CONCAT22(param_3, param_2));
        if(*(int *)((int)uVar6 + 0xc) == *(int *)(iVar4 + 0x110))
        {
            pass1_1030_ea50(param_1, uStack6, param_4, param_5, param_6);
        }
        uVar1     = *(uint *)(iVar4 + 0x108);
        uVar2     = *(uint *)(iVar4 + 0x10a);
        paStack20 = (astruct_18 *)CONCAT22(uVar2, uVar1);
        if((uVar2 | uVar1) != 0x0)
        {
            fn_ptr_1020_ba7e((ulong *)CONCAT22(uVar2, uVar1));
            fn_ptr_1000_17ce(paStack20, 0x1000);
        }
        *(undefined4 *)(iVar4 + 0x108) = 0x0;
    }
    return 0x1;
}


astruct_18 *__stdcall16far pass1_1030_eb14(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_ec86(astruct_18 *param_1, byte param_2)

{
    param_1->field_0x0                  = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_d868(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_dc08(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_bbe6(astruct_18 *param_1, byte param_2)

{
    pass1_1030_b96c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_bc70(astruct_18 *param_1, byte param_2)

{
    pass1_1030_bc4e(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_bfe0(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_c668(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_c91a(astruct_18 *param_1, byte param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1030_b90c(astruct_18 *param_1, byte param_2)

{
    pass1_1030_afa6(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far pass1_1030_b96c(ushort *param_1)

{
    uint        uVar1;
    astruct_18 *paVar2;
    int         iVar3;
    undefined2  uVar4;

    uVar4                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                        = (int)param_1;
    *param_1                     = 0xbc0c;
    *(undefined2 *)(iVar3 + 0x2) = 0x1030;
    paVar2                       = *(astruct_18 **)(iVar3 + 0xe);
    uVar1                        = *(uint *)(iVar3 + 0x10);
    if((uVar1 | (uint)paVar2) != 0x0)
    {
        fn_ptr_1020_ba7e((ulong *)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10));
        fn_ptr_1000_17ce(paVar2, 0x1000);
    }
    pass1_1028_b260(param_1);
    return;
}


void __stdcall16far
pass1_1030_bb0e(ulong param_1, ulong param_2, uint param_3, ushort param_4, ushort param_5, ushort param_6)

{
    astruct_18 *paVar1;
    ulong       uVar2;
    ushort      uVar3;
    ulong       uVar4;
    uint        in_DX;
    uchar      *puVar5;
    ulong       uVar6;
    ushort      uStack8;

    uVar3 = pass1_1030_7bee(param_2);
    uVar4 = (ulong)uVar3;
    if(uVar3 != 0x0)
    {
        return;
    }
    pass1_1030_b9b2(param_1);
    uVar2  = uVar4 & 0xffff;
    paVar1 = (astruct_18 *)(uVar2 | (ulong)in_DX << 0x10);
    puVar5 = (uchar *)(in_DX | (uint)uVar4);
    if(puVar5 != (uchar *)0x0)
    {
        for(uStack8 = 0x4; (int)uStack8 < 0x25; uStack8 = uStack8 + 0x1)
        {
            uVar6  = pass1_1020_bae6((ushort)uVar2, CONCAT22(uStack8, in_DX), (uint)uVar4, (uint)puVar5, param_6);
            uVar4  = uVar6 & 0xffff;
            puVar5 = (uchar *)((uint)(uVar6 >> 0x10) | (uint)uVar4);
            if(puVar5 != (uchar *)0x0)
            {
                pass1_1030_7ddc(param_2, uVar6, uStack8, (uint)uVar4, puVar5, param_4, param_5, param_6);
                uVar3 = pass1_1030_7bee(param_2);
                uVar4 = (ulong)uVar3;
                if(uVar3 != 0x0)
                {
                    return;
                }
                string_1020_c0ca(uStack8);
                vsprintf_op_1030_840a(
                  (ulong)s_truck_0x_08lx_unloaded__ld_of__s_1050_5798, 0x1020, param_6, (ushort)puVar5);
                pass1_1020_bb8a((long *)paVar1, 0x0, (ulong)uStack8 << 0x10, param_5, param_6);
            }
        }
        if(paVar1 != (astruct_18 *)0x0)
        {
            fn_ptr_1020_ba7e((ulong *)paVar1);
            fn_ptr_1000_17ce(paVar1, 0x1000);
        }
    }
    return;
}


void __stdcall16far pass1_1030_9f7a(ushort *param_1, ushort param_2, ushort param_3, uchar param_4)

{
    undefined4 uVar1;
    BOOL16     BVar2;
    ulong     *puVar3;
    ushort     extraout_DX;
    ushort     uVar4;
    ushort     uVar5;
    undefined  local_130[0x120];
    ulong      uStack16;
    ulong      uStack12;
    ulong      local_8;
    int        iStack4;

    pass1_1008_3e38((ushort *)CONCAT22(param_3, &local_8));
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, param_2, 0x28);
    if(BVar2 != 0x0)
    {
        iStack4 = 0x1;
    }
    puVar3 = &local_8;
    pass1_1030_a278(param_1, (ushort *)CONCAT22(param_3, puVar3), (ushort)puVar3, param_3, param_4);
    if(puVar3 != (ulong *)0x0)
    {
        uVar5    = (ushort)((ulong)param_1 >> 0x10);
        uVar4    = (ushort)param_1;
        uVar1    = *(undefined4 *)(uVar4 + 0x4);
        uStack12 = *(ulong *)((int)uVar1 + 0x8);
        uVar1    = *(undefined4 *)(uVar4 + 0x4);
        struct_op_1028_87f0(param_3,
                            param_4,
                            (astruct_97 *)CONCAT22(param_3, local_130),
                            0x0,
                            0x0,
                            param_2,
                            &local_8,
                            param_3,
                            *(ulong *)((int)uVar1 + 0x4),
                            uStack12);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_3, local_130));
        pass1_1028_b58e(uStack16);
        *param_1                 = (ushort)uStack16;
        *(ushort *)(uVar4 + 0x2) = extraout_DX;
        if(0x0 < iStack4)
        {
            pass1_1030_a044(
              param_3, extraout_DX, param_4, uVar4, uVar5, (ushort *)CONCAT22(param_3, &local_8), uStack12);
        }
    }
    return;
}


void __stdcall16far pass1_1030_a044(
  ushort param_1, ushort param_2, uchar param_3, ushort param_4, ushort param_5, ushort *param_6, ulong param_7)

{
    code      **ppcVar1;
    ushort     *puVar2;
    undefined  *puVar3;
    int         iVar4;
    undefined4  uVar5;
    uint        uVar6;
    undefined2  extraout_DX;
    undefined2  uVar7;
    undefined4 *puVar8;
    ushort      uVar9;
    undefined2  uVar10;
    undefined2  uVar11;
    undefined2  local_17e;
    undefined2  uStack380;
    int         iStack90;
    undefined4 *puStack78;
    ushort      uStack70;
    int         iStack68;
    undefined4  uStack66;
    undefined4 *puStack62;
    undefined   local_3a[0xc];
    undefined4  local_2e;
    undefined2  uStack42;
    int         iStack40;
    ushort      uStack38;
    int         local_24;
    int         local_22;
    undefined4  uStack32;
    undefined4  uStack28;
    undefined4  uStack24;
    ushort     *puStack20;
    uint        uStack18;
    int         iStack16;
    int         iStack14;
    undefined4  uStack12;
    ushort      local_8;
    int         local_6;
    int         local_4;

    puVar2 = &local_8;
    pass1_1008_3eb4(param_6,
                    (ushort *)CONCAT22(param_1, puVar2),
                    (ushort *)CONCAT22(param_1, &local_6),
                    (ushort *)CONCAT22(param_1, &local_4));
    pass1_1030_627e(param_1, (uint)puVar2, param_2, (ulong)_PTR_LOOP_1050_5740, param_6, param_7);
    puStack20 = puVar2;
    uStack18  = param_2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, param_2);
    uStack24 = CONCAT22(param_2, puVar2);
    uStack28 = *(undefined4 *)(puVar2 + 0x17);
    uVar5    = *(undefined4 *)((int)uStack28 + 0x4);
    uStack32 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_7, (uint)(param_7 >> 0x10));
    iStack40  = (int)uVar5;
    uStack38  = param_2;
    puVar8    = (undefined4 *)pass1_1030_5b5c(iStack40, param_2);
    uVar6     = (uint)((ulong)puVar8 >> 0x10);
    local_2e  = *puVar8;
    uStack42  = *(undefined2 *)((int)puVar8 + 0x4);
    puStack78 = &local_2e;
    pass1_1008_3e94((ushort *)CONCAT22(param_1, &local_2e),
                    (ushort *)CONCAT22(param_1, &local_24),
                    (ushort *)CONCAT22(param_1, &local_22));
    iStack14 = local_4 + 0x1;
    uStack12 = CONCAT22(local_4 + -0x1, local_6 - 0x1U);
    iStack16 = local_6 + 0x1;
    if(local_4 + -0x1 < 0x0)
    {
        uStack12 = (ulong)(local_6 - 0x1U);
    }
    if(local_22 <= iStack14)
    {
        iStack14 = local_22 + -0x1;
    }
    if((int)(ushort)uStack12 < 0x0)
    {
        uStack12 = uStack12 & 0xffff0000;
    }
    if(local_24 <= iStack16)
    {
        iStack16 = local_24 + -0x1;
    }
    pass1_1008_6c90((ushort *)CONCAT22(param_1, local_3a));
    uVar7 = 0x1008;
    pass1_1008_6cec((ushort *)CONCAT22(param_1, local_3a), local_8, CONCAT22(iStack14, iStack16), local_8, uStack12);
    puVar3 = local_3a;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(param_1, puVar3), param_7, param_1);
    puStack62 = (undefined4 *)CONCAT22(uVar6, puVar3);
    if((uVar6 | (uint)puVar3) != 0x0)
    {
        uStack66 = 0x0;
        iStack68 = 0x0;
        for(uStack70 = (ushort)uStack12; (int)uStack70 <= iStack16; uStack70 = uStack70 + 0x1)
        {
            for(puStack78 = (undefined4 *)uStack12._2_2_; (int)puStack78 <= iStack14;
                puStack78 = (undefined4 *)((int)puStack78 + 0x1))
            {
                ppcVar1  = (code **)((int)*puStack62 + 0x4);
                iVar4    = iStack68;
                iStack68 = iStack68 + 0x1;
                (**ppcVar1)(uVar7, (int)puStack62, (int)((ulong)puStack62 >> 0x10));
                uStack66       = CONCAT22(extraout_DX, iVar4);
                uStack66._3_1_ = (char)((uint)extraout_DX >> 0x8);
                if(uStack66._3_1_ == '\0')
                {
                    iStack90 = iVar4;
                    if(iVar4 == 0x7)
                    {
                        pass1_1008_3e76(param_6, local_8, uStack70, (ushort)puStack78);
                        uVar10 = (undefined2)uStack32;
                        uVar11 = (undefined2)((ulong)uStack32 >> 0x10);
                        uVar9  = 0x6;
                    }
                    else
                    {
                        if(iVar4 == 0x8)
                        {
                            pass1_1008_3e76(param_6, local_8, uStack70, (ushort)puStack78);
                            uVar10 = (undefined2)uStack32;
                            uVar11 = (undefined2)((ulong)uStack32 >> 0x10);
                            uVar9  = 0x7;
                        }
                        else
                        {
                            if(iVar4 != 0x9)
                                goto LAB_1030_a1d0;
                            pass1_1008_3e76(param_6, local_8, uStack70, (ushort)puStack78);
                            uVar10 = (undefined2)uStack32;
                            uVar11 = (undefined2)((ulong)uStack32 >> 0x10);
                            uVar9  = 0x8;
                        }
                    }
                    uVar7 = SUB42(&USHORT_1050_1028, 0x0);
                    struct_op_1028_87f0(param_1,
                                        param_3,
                                        (astruct_97 *)CONCAT22(param_1, &local_17e),
                                        0x0,
                                        0x0,
                                        uVar9,
                                        (ulong *)param_6,
                                        (ushort)((ulong)param_6 >> 0x10),
                                        CONCAT22(uVar11, uVar10),
                                        param_7);
                    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, &local_17e));
                    local_17e = 0x389a;
                    uStack380 = 0x1008;
                }
            LAB_1030_a1d0:
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_a278(ushort *param_1, ushort *param_2, ushort param_3, ushort param_4, uchar param_5)

{
    int        iVar1;
    undefined4 uVar2;
    int        in_DX;
    ushort     extraout_DX;
    ulong     *puVar3;
    ushort     uVar4;
    uint       uVar5;
    ushort     uVar6;
    undefined  local_134[0x120];
    ulong      uStack20;
    ulong      uStack16;
    ulong      uStack12;
    ushort     uStack6;
    undefined2 uStack4;

    uStack4 = 0x1;
    pass1_1030_a39a((ulong)param_1, param_2, param_4);
    if(param_3 != 0x0)
    {
        return;
    }
    uStack6 = param_3;
    pass1_1030_a3ae((ulong)param_1, param_2, param_4);
    puVar3 = (ulong *)param_2;
    uVar5  = (uint)((ulong)param_2 >> 0x10);
    if(param_3 == 0x0)
    {
        pass1_1030_a57e((ulong)param_1, param_2, 0x0, in_DX, param_4);
        if(param_3 == 0x0)
        {
            pass1_1030_a844((ulong)param_1, param_2, 0x0, in_DX, param_4);
            if(param_3 == 0x0)
            {
                uStack4 = 0x0;
                goto LAB_1030_a305;
            }
            iVar1 = *(int *)(puVar3 + 0x1);
        }
        else
        {
            iVar1 = *(int *)(puVar3 + 0x1);
        }
        if(iVar1 < 0x1)
        {
            uStack6 = 0x73;
        }
        else
        {
            uStack6 = 0x77;
        }
    }
    else
    {
        if(*(int *)(puVar3 + 0x1) < 0x1)
        {
            uStack6 = 0x7a;
        }
        else
        {
            uStack6 = 0x7f;
        }
    }
LAB_1030_a305:
    if(uStack6 != 0x0)
    {
        uVar6    = (ushort)((ulong)param_1 >> 0x10);
        uVar4    = (ushort)param_1;
        uVar2    = *(undefined4 *)(uVar4 + 0x4);
        uStack16 = *(ulong *)((int)uVar2 + 0x8);
        uVar2    = *(undefined4 *)(uVar4 + 0x4);
        struct_op_1028_87f0(param_4,
                            param_5,
                            (astruct_97 *)CONCAT22(param_4, local_134),
                            0x0,
                            0x0,
                            uStack6,
                            puVar3,
                            uVar5,
                            *(ulong *)((int)uVar2 + 0x4),
                            uStack16);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_4, local_134));
        uStack12 = uStack20;
        pass1_1028_b58e(uStack20);
        *param_1                 = (ushort)uStack20;
        *(ushort *)(uVar4 + 0x2) = extraout_DX;
        if(0x0 < *(int *)(puVar3 + 0x1))
        {
            pass1_1030_a044(param_4,
                            extraout_DX,
                            param_5,
                            uVar4,
                            uVar6,
                            (ushort *)((ulong)param_2 & 0xffff | (ulong)uVar5 << 0x10),
                            uStack16);
        }
    }
    return;
}


void __stdcall16far pass1_1030_a844(ulong param_1, ushort *param_2, int param_3, int param_4, ushort param_5)

{
    int          iVar1;
    undefined4   uVar2;
    code       **ppcVar3;
    uint         uVar4;
    ushort       uVar5;
    int         *piVar6;
    undefined4  *puVar7;
    uint         extraout_DX;
    uint         uVar9;
    uint         uVar10;
    astruct_426 *uVar8;
    astruct_427 *iVar8;
    int          iVar11;
    ushort       uVar12;
    undefined2   uVar13;
    ushort      *puVar14;
    ulong        uVar15;
    ulong        uStack34;
    int          local_1c;
    int          local_1a;
    int          local_18;
    int          local_16;
    int          iStack20;
    undefined2   uStack16;
    long         lStack14;
    ulong        uStack10;
    undefined4  *puStack6;

    uVar12 = (ushort)(param_1 >> 0x10);
    uVar8  = (astruct_426 *)param_1;
    pass1_1038_53ba(uVar8->field_0x4, 0x1);
    if((param_4 != 0x0) || (param_3 != 0x0))
    {
        uVar15   = uVar8->field_0x4;
        uVar13   = (undefined2)(uVar15 >> 0x10);
        iVar8    = (astruct_427 *)uVar15;
        puVar7   = iVar8->field_0xc;
        ppcVar3  = (code **)((int)*puVar7 + 0x10);
        puStack6 = puVar7;
        (**ppcVar3)((int)&PTR_LOOP_1050_1038, (int)puVar7, *(undefined2 *)((int)&iVar8->field_0xc + 0x2));
        uStack10 = (ulong)puVar7 & 0xffff | (ulong)extraout_DX << 0x10;
        uVar15   = uVar8->field_0x4;
        lStack14 = *(long *)((int)uVar15 + 0x8);
        uStack16 = 0x0;
        puVar14  = pass1_1008_3e38((ushort *)CONCAT22(param_5, &local_16));
        uVar9    = (uint)((ulong)puVar14 >> 0x10);
        iVar1    = *(int *)((int)param_2 + 0x4);
        for(uStack34 = 0x0; uStack34 < uStack10; uStack34 = uStack34 + 0x1)
        {
            uVar15 = pass1_1030_1d7c((int)uStack10, uVar9, (ulong)puStack6);
            uVar4  = (uint)(uVar15 >> 0x10);
            uVar10 = uVar4 | (uint)uVar15;
            uVar9  = uVar10;
            if((uVar10 != 0x0)
               && (uVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((uint)uVar15 + 0xc), 0x46),
                   uVar9 = uVar10,
                   uVar4 != 0x0))
            {
                pass1_1030_1d58((ulong)puStack6);
                uVar9 = uVar10 | uVar4;
                if((uVar10 | uVar4) != 0x0)
                {
                    pass1_1008_3f62((ushort *)CONCAT22(param_5, &local_16), (ushort *)CONCAT22(uVar10, uVar4 + 0xc));
                    pass1_1008_3eb4((ushort *)CONCAT22(param_5, &local_16),
                                    (ushort *)CONCAT22(param_5, &local_1c),
                                    (ushort *)CONCAT22(param_5, &local_1a),
                                    (ushort *)CONCAT22(param_5, &local_18));
                    uVar9 = uVar10;
                    if((iVar1 == local_1c)
                       && (uVar15 = uVar8->field_0x4,
                           uVar13 = (undefined2)(uVar15 >> 0x10),
                           iVar11 = (int)uVar15,
                           uVar2  = *(undefined4 *)(iVar11 + 0x4),
                           uVar5  = pass1_1030_addc((ushort)uVar8,
                                                   uVar12,
                                                   (ushort *)CONCAT22(param_5, &local_16),
                                                   (ushort)uVar2,
                                                   (ushort)((ulong)uVar2 >> 0x10),
                                                   *(ulong *)(iVar11 + 0x8),
                                                   (int)&local_16,
                                                   uVar10,
                                                   param_5),
                           uVar9  = uVar10,
                           uVar5 != 0x0))
                    {
                        iStack20 = local_1a + -0x1;
                        piVar6   = &local_16;
                        pass1_1030_ad86(
                          (ushort)uVar8, uVar12, (ushort *)CONCAT22(param_5, piVar6), lStack14, param_5, uVar10);
                        if(piVar6 != (int *)0x0)
                        {
                        LAB_1030_a98e:
                            pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_5, &local_16));
                            return;
                        }
                        iStack20 = local_1a + 0x1;
                        piVar6   = &local_16;
                        pass1_1030_ad86(
                          (ushort)uVar8, uVar12, (ushort *)CONCAT22(param_5, piVar6), lStack14, param_5, uVar10);
                        if(piVar6 != (int *)0x0)
                            goto LAB_1030_a98e;
                        iStack20 = local_1a;
                        local_16 = local_18 + -0x1;
                        piVar6   = &local_16;
                        pass1_1030_ad86(
                          (ushort)uVar8, uVar12, (ushort *)CONCAT22(param_5, piVar6), lStack14, param_5, uVar10);
                        if(piVar6 != (int *)0x0)
                            goto LAB_1030_a98e;
                        local_16 = local_18 + 0x1;
                        piVar6   = &local_16;
                        pass1_1030_ad86(
                          (ushort)uVar8, uVar12, (ushort *)CONCAT22(param_5, piVar6), lStack14, param_5, uVar10);
                        uVar9 = uVar10;
                        if(piVar6 != (int *)0x0)
                            goto LAB_1030_a98e;
                    }
                }
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_aa18(ulong param_1, ushort *param_2, ushort param_3)

{
    ulong       uVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    ushort      uVar4;
    BOOL16      BVar5;
    ulong       uVar6;
    uchar      *puVar7;
    uint        extraout_DX;
    uint        uVar8;
    uint        uVar9;
    ushort      uVar10;
    undefined2  uVar11;
    int         iVar12;
    undefined4 *puVar13;
    ushort      uVar14;
    undefined2  uVar15;
    undefined2  uVar16;
    undefined2  uVar17;
    ulong      *puVar18;
    ulong       uVar19;
    undefined   uVar20;
    ulong       uStack38;
    undefined   local_1a[0x2];
    int         local_18;
    int         local_16;
    int         local_14;
    int         iStack18;
    undefined4  uStack14;
    uint        uStack10;
    uchar      *puStack8;
    int         iStack6;
    int         iStack4;

    iStack4  = *(int *)((int)param_2 + 0x4);
    iStack6  = 0x8 - (uint)(iStack4 == 0x0);
    puVar18  = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, iStack6);
    puVar7   = (uchar *)((ulong)puVar18 >> 0x10);
    uVar8    = (uint)puVar18;
    uVar14   = (ushort)(param_1 >> 0x10);
    uVar10   = (ushort)param_1;
    uStack10 = uVar8;
    puStack8 = puVar7;
    pass1_1038_4e78(uVar8, puVar7, *(ulong *)(uVar10 + 0x4), puVar18);
    uStack14 = (undefined4 *)CONCAT22(puVar7, uVar8);
    uVar17   = 0x1008;
    pass1_1008_3e38((ushort *)CONCAT22(param_3, &local_14));
    uVar3   = *(undefined4 *)(uVar10 + 0x4);
    uVar1   = *(ulong *)((int)uVar3 + 0x8);
    uVar15  = (undefined2)((ulong)uStack14 >> 0x10);
    uVar11  = SUB42(uStack14, 0x0);
    ppcVar2 = (code **)((int)*uStack14 + 0x10);
    uVar6   = uVar1;
    (**ppcVar2)(0x1008, uVar11, uVar15);
    uVar6    = uVar6 & 0xffff | (ulong)extraout_DX << 0x10;
    uStack38 = 0x0;
    uVar8    = extraout_DX;
    while(true)
    {
        if(uVar6 <= uStack38)
        {
            if(uStack14 != (undefined4 *)0x0)
            {
                ppcVar2 = (code **)*uStack14;
                (**ppcVar2)(
                  uVar17, (int)uStack14, (char)((ulong)uStack14 >> 0x10), 0x1, uVar11, uVar15, uStack14, uStack14);
            }
            return;
        }
        uVar19 = uVar6;
        pass1_1030_1d58((ulong)uStack14);
        uVar9 = uVar8 | (uint)uVar19;
        if(uVar9 != 0x0)
            break;
    LAB_1030_aadc:
        uStack38 = uStack38 + 0x1;
        uVar8    = uVar9;
    }
    uVar9 = uVar8;
    pass1_1008_3f62((ushort *)CONCAT22(param_3, &local_14), (ushort *)CONCAT22(uVar8, (uint)uVar19 + 0xc));
    uVar17 = 0x1008;
    pass1_1008_3eb4((ushort *)CONCAT22(param_3, &local_14),
                    (ushort *)CONCAT22(param_3, local_1a),
                    (ushort *)CONCAT22(param_3, &local_18),
                    (ushort *)CONCAT22(param_3, &local_16));
    uVar3  = *(undefined4 *)(uVar10 + 0x4);
    uVar16 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar12 = (int)uVar3;
    uVar3  = *(undefined4 *)(iVar12 + 0x4);
    uVar4  = pass1_1030_addc(uVar10,
                            uVar14,
                            (ushort *)CONCAT22(param_3, &local_14),
                            (ushort)uVar3,
                            (ushort)((ulong)uVar3 >> 0x10),
                            *(ulong *)(iVar12 + 0x8),
                            (int)&local_14,
                            uVar9,
                            param_3);
    if(uVar4 == 0x0)
        goto LAB_1030_aadc;
    uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | (ulong)uVar8 << 0x10);
    uVar9  = (uint)(uVar19 >> 0x10);
    iVar12 = *(int *)((int)uVar19 + 0xc);
    if(0x5 < iVar12 - 0x7aU)
        goto LAB_1030_aadc;
    uVar17 = 0x1030;
    switch(iVar12)
    {
    default:
        iStack18 = local_18 + -0x1;
        BVar5    = pass1_1030_acbe(
          uVar10, uVar14, (ushort *)CONCAT22(param_3, &local_14), uVar1, (uint)&local_14, uVar9, param_3);
        if(BVar5 != 0x0)
            goto LAB_1030_ac5b;
        iStack18 = local_18 + 0x1;
        BVar5    = pass1_1030_acbe(
          uVar10, uVar14, (ushort *)CONCAT22(param_3, &local_14), uVar1, (uint)&local_14, uVar9, param_3);
        if(BVar5 == 0x0)
        {
            iStack18 = local_18;
            local_14 = local_16 + -0x1;
            BVar5    = pass1_1030_acbe(
              uVar10, uVar14, (ushort *)CONCAT22(param_3, &local_14), uVar1, (uint)&local_14, uVar9, param_3);
            goto joined_r0x1030ab9e;
        }
    LAB_1030_abc4:
        pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_3, &local_14));
        break;
    case 0x7b:
    case 0x7e:
        iStack18 = local_18 + -0x1;
        BVar5    = pass1_1030_acbe(
          uVar10, uVar14, (ushort *)CONCAT22(param_3, &local_14), uVar1, (uint)&local_14, uVar9, param_3);
        if(BVar5 == 0x0)
        {
            iStack18 = local_18 + 0x1;
            goto LAB_1030_abac;
        }
        pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_3, &local_14));
        if(uStack14 == (undefined4 *)0x0)
        {
            return;
        }
        uVar17  = (undefined2)((ulong)uStack14 >> 0x10);
        puVar13 = (undefined4 *)uStack14;
        uVar20  = (undefined)((ulong)uStack14 >> 0x10);
        goto LAB_1030_ab66;
    case 0x7c:
    case 0x7d:
        local_14 = local_16 + -0x1;
        BVar5    = pass1_1030_acbe(
          uVar10, uVar14, (ushort *)CONCAT22(param_3, &local_14), uVar1, (uint)&local_14, uVar9, param_3);
    joined_r0x1030ab9e:
        if(BVar5 == 0x0)
        {
            local_14 = local_16 + 0x1;
        LAB_1030_abac:
            BVar5 = pass1_1030_acbe(
              uVar10, uVar14, (ushort *)CONCAT22(param_3, &local_14), uVar1, (uint)&local_14, uVar9, param_3);
            if(BVar5 != 0x0)
                goto LAB_1030_abc4;
            goto LAB_1030_aadc;
        }
    LAB_1030_ac5b:
        pass1_1008_3f62(param_2, (ushort *)CONCAT22(param_3, &local_14));
    }
    puVar13 = (undefined4 *)uStack14;
    if((uStack14._2_2_ | (uint)puVar13) != 0x0)
    {
        uVar17 = (undefined2)((ulong)uStack14 >> 0x10);
        uVar20 = (undefined)((ulong)uStack14 >> 0x10);
    LAB_1030_ab66:
        ppcVar2 = (code **)*puVar13;
        (**ppcVar2)(0x1008, puVar13, uVar20, 0x1, uVar11, uVar15, uStack14, uStack14);
    }
    return;
}
