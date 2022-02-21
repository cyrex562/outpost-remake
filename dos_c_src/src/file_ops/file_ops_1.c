
void __stdcall16far file_1038_774e(ulong param_1, ulong param_2, uchar *param_3, ushort param_4)

{
    ushort       uVar1;
    astruct_307 *iVar2;
    BOOL16       BVar2;
    int          iVar3;
    ushort       uVar4;
    ushort       uVar6;
    undefined2   local_8;
    ushort       local_6;
    undefined2   local_4;
    ulong       *puVar5;

    if((int)PTR_LOOP_1050_0312 < 0x2)
    {
        return;
    }
    iVar2  = (astruct_307 *)param_1;
    iVar2  = (astruct_307 *)&iVar2->field_0x4;
    puVar5 = (ulong *)(param_1 & 0xffff0000 | ZEXT24(iVar2));
    pass1_1008_766e(param_2, puVar5, param_4, 0x1008, param_3);
    if((int)puVar5 != 0x0)
    {
        uVar1 = (ushort)(param_1 >> 0x10);
        uVar4 = (ushort)param_2;
        uVar6 = (ushort)(param_2 >> 0x10);
        BVar2 = read_file_1008_7dee(uVar4, uVar6, &iVar2->field_0x8, 0x0, uVar1, 0x4, 0x1008);
        if((((((BVar2 != 0x0)
               && (iVar3 = file_1008_77cc(param_2,
                                          (long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0xe),
                                          param_3,
                                          0x1008,
                                          param_4),
                   iVar3 != 0x0))
              && (BVar2 = read_file_1008_7dee(uVar4, uVar6, (ushort)&local_4, 0x0, param_4, 0x2, 0x1008), BVar2 != 0x0))
             && ((BVar2 = read_file_1008_7dee(uVar4, uVar6, (ushort)&local_6, 0x0, param_4, 0x2, 0x1008),
                  BVar2 != 0x0
                    && (BVar2 = read_file_1008_7dee(uVar4, uVar6, (ushort)&local_8, 0x0, param_4, 0x2, 0x1008),
                        BVar2 != 0x0))))
            && ((BVar2 = read_file_1008_7dee(uVar4, uVar6, &iVar2->field_0x16, 0x0, uVar1, 0x4, 0x1008),
                 BVar2 != 0x0
                   && ((BVar2 = read_file_1008_7bc8(
                          param_2, (ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x1a), 0x1008, param_4),
                        BVar2 != 0x0
                          && (BVar2 = read_file_1008_7dee(uVar4, uVar6, &iVar2->field_0x20, 0x0, uVar1, 0x4, 0x1008),
                              BVar2 != 0x0))))))
           && ((BVar2 = read_file_1008_7dee(uVar4, uVar6, &iVar2->field_0x24, 0x0, uVar1, 0x2, 0x1008),
                BVar2 != 0x0
                  && ((BVar2 = read_file_1008_7dee(uVar4, uVar6, &iVar2->field_0x26, 0x0, uVar1, 0x2, 0x1008),
                       BVar2 != 0x0
                         && (BVar2 = read_file_1008_7dee(uVar4, uVar6, &iVar2->field_0x28, 0x0, uVar1, 0x2, 0x1008),
                             BVar2 != 0x0))))))
        {
            iVar2->field_0xc  = local_4;
            uVar4             = switch_1008_72bc(uVar4, uVar6, local_6);
            iVar2->field_0x12 = uVar4;
            iVar2->field_0x14 = local_8;
            return;
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
}


ushort __stdcall16far pass1_1038_7b20(ulong *param_1, ulong param_2, uint16_t param_3)

{
    undefined4 uVar1;
    BOOL16     BVar2;
    undefined2 uVar3;
    ulong      uVar4;
    ushort     uVar5;
    undefined2 local_1c;
    undefined2 uStack26;
    undefined2 uStack24;
    undefined4 uStack16;
    undefined  local_c[0x8];
    undefined2 local_4;

    BVar2 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar2 != 0x0)
    {
        local_1c = *(undefined2 *)((int)*param_1 + 0x8);
        uVar5    = (ushort)(param_2 >> 0x10);
        local_4  = local_1c;
        BVar2    = write_to_file_1008_7e1c((ushort)param_2, uVar5, (ushort)&local_1c, param_3, (char *)0x2, 0x1008);
        if(BVar2 != 0x0)
        {
            pass1_1008_5784((ulong *)CONCAT22(param_3, local_c), *param_1);
            do
            {
                uStack16 = pass1_1008_5b12(local_c, param_3);
                if(uStack16 == 0x0)
                {
                    uVar3    = (undefined2)((ulong)param_1 >> 0x10);
                    uVar1    = *(undefined4 *)((int)param_1 + 0x4);
                    local_1c = *(undefined2 *)((int)uVar1 + 0x8);
                    local_4  = local_1c;
                    BVar2
                      = write_to_file_1008_7e1c((ushort)param_2, uVar5, (ushort)&local_4, param_3, (char *)0x2, 0x1008);
                    if(BVar2 == 0x0)
                    {
                        return 0x0;
                    }
                    pass1_1008_5784((ulong *)CONCAT22(param_3, local_c), *(ulong *)((int)param_1 + 0x4));
                    do
                    {
                        uVar4    = pass1_1008_5b12(local_c, param_3);
                        uStack26 = (undefined2)uVar4;
                        if(uVar4 == 0x0)
                        {
                            return 0x1;
                        }
                        pass1_1030_b768(uVar4, param_2, param_3);
                        uStack24 = (undefined2)(uVar4 >> 0x10);
                    } while((int)uVar4 != 0x0);
                    return 0x0;
                }
                pass1_1038_75ca(uStack16, param_2, (int)uStack16, param_3);
                uStack16._2_2_ = (undefined2)(uStack16 >> 0x10);
            } while((int)uStack16 != 0x0);
        }
    }
    return 0x0;
}


undefined2 __stdcall16far read_file_1038_7c02(undefined4 *param_1,
                                              undefined4  param_2,
                                              uint16_t    param_3,
                                              uint16_t    param_4)

{
    code     **ppcVar1;
    BOOL16     BVar2;
    uint       uVar3;
    uint       uVar4;
    uchar     *extraout_DX;
    uchar     *puVar5;
    uchar     *extraout_DX_00;
    uint16_t   unaff_SS;
    ushort     uVar6;
    undefined2 uVar7;
    ushort     uVar8;
    undefined4 uVar9;
    undefined2 uVar10;
    uint       local_12[0x2];
    undefined4 uStack14;
    uint       local_4;

    if((int)PTR_LOOP_1050_0312 < 0x2)
    {
        return 0x1;
    }
    uVar6 = (ushort)param_2;
    uVar8 = (ushort)((ulong)param_2 >> 0x10);
    read_file_1008_7cfe(uVar6, uVar8, 0x17, 0x1008, unaff_SS);
    if((param_3 != 0x0)
       && (BVar2 = read_file_1008_7dee(uVar6, uVar8, (ushort)&local_4, 0x0, unaff_SS, 0x2, 0x1008), BVar2 != 0x0))
    {
        while(local_4 != 0x0)
        {
            uVar7   = 0x2a;
            uVar3   = local_4;
            local_4 = local_4 - 0x1;
            uVar9   = param_2;
            mem_op_1000_179c(0x2a, (uchar *)param_4, 0x1000);
            puVar5 = (uchar *)(param_4 | uVar3);
            if(puVar5 == (uchar *)0x0)
            {
                uVar3  = 0x0;
                puVar5 = (uchar *)0x0;
            }
            else
            {
                struct_1038_6520((ushort *)CONCAT22(param_4, uVar3));
            }
            uVar10   = (undefined2)((ulong)uVar9 >> 0x10);
            uStack14 = CONCAT22(puVar5, uVar3);
            file_1038_774e(CONCAT22(puVar5, uVar3), CONCAT22((int)uVar9, uVar7), puVar5, unaff_SS);
            if(uVar3 == 0x0)
            {
                return 0x0;
            }
            ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x4);
            (**ppcVar1)(0x1000,
                        (int)*param_1,
                        (int)((ulong)*param_1 >> 0x10),
                        (int)uStack14,
                        (int)((ulong)uStack14 >> 0x10),
                        uVar10);
            param_4 = (uint16_t)extraout_DX;
        }
        local_4 = local_4 - 0x1;
        BVar2   = read_file_1008_7dee(uVar6, uVar8, (ushort)local_12, 0x0, unaff_SS, 0x2, 0x1008);
        if(BVar2 != 0x0)
        {
            while(true)
            {
                if(local_12[0] == 0x0)
                {
                    return 0x1;
                }
                uVar7       = 0x14;
                uVar3       = local_12[0];
                local_12[0] = local_12[0] - 0x1;
                uVar9       = param_2;
                mem_op_1000_179c(0x14, (uchar *)param_4, 0x1000);
                puVar5 = (uchar *)(param_4 | uVar3);
                if(puVar5 == (uchar *)0x0)
                {
                    uVar3  = 0x0;
                    puVar5 = (uchar *)0x0;
                }
                else
                {
                    pass1_1030_ae6c((ushort *)CONCAT22(param_4, uVar3));
                }
                uVar10 = (undefined2)((ulong)uVar9 >> 0x10);
                uVar4  = uVar3;
                file_1030_b836(CONCAT22(puVar5, uVar3), CONCAT22((int)uVar9, uVar7), puVar5, unaff_SS);
                if(uVar4 == 0x0)
                    break;
                uVar7   = (undefined2)((ulong)param_1 >> 0x10);
                uVar9   = *(undefined4 *)((int)param_1 + 0x4);
                ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x4);
                (**ppcVar1)(0x1030, (int)uVar9, (int)((ulong)uVar9 >> 0x10), uVar3, puVar5, uVar10);
                param_4 = (uint16_t)extraout_DX_00;
            }
            return 0x0;
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1038_5e16(ulong param_1, ulong param_2, int param_3, ushort param_4, ushort param_5)

{
    BOOL16     BVar1;
    ulong     *puVar2;
    int        iVar3;
    ushort     uVar4;
    ushort     uVar5;
    ushort     uVar6;
    undefined4 local_14[0x2];
    undefined4 local_c;
    ulong     *puStack6;

    pass1_1030_16d6(param_1, param_2, param_5);
    if(param_3 != 0x0)
    {
        uVar4    = (ushort)(param_1 >> 0x10);
        iVar3    = (int)param_1;
        puVar2   = *(ulong **)(iVar3 + 0xc);
        puStack6 = puVar2;
        pass1_1008_7898(param_2, puVar2, (ushort)puVar2, param_4, 0x1008, param_5);
        if((int)puVar2 != 0x0)
        {
            local_14[0] = *(undefined4 *)(iVar3 + 0x10);
            uVar5       = (ushort)param_2;
            uVar6       = (ushort)(param_2 >> 0x10);
            BVar1       = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_14, param_5, (char *)0x4, 0x1008);
            if(BVar1 != 0x0)
            {
                local_c._0_2_ = *(undefined2 *)(iVar3 + 0x18);
                BVar1         = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_c, param_5, (char *)0x2, 0x1008);
                if(BVar1 != 0x0)
                {
                    local_c._0_2_ = *(undefined2 *)(iVar3 + 0x1a);
                    BVar1 = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_c, param_5, (char *)0x2, 0x1008);
                    if(BVar1 != 0x0)
                    {
                        local_c = CONCAT22(local_c._2_2_, *(undefined2 *)(iVar3 + 0x1c));
                        BVar1   = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_c, param_5, (char *)0x2, 0x1008);
                        if(BVar1 != 0x0)
                        {
                            local_c = *(ulong *)(iVar3 + 0x1e);
                            BVar1
                              = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_c, param_5, (char *)0x4, 0x1008);
                            if(BVar1 != 0x0)
                            {
                                local_c = local_c & 0xffff0000 | (ulong) * (uint *)(iVar3 + 0x22);
                                BVar1   = write_to_file_1008_7e1c(
                                  uVar5, uVar6, (ushort)&local_c, param_5, (char *)0x2, 0x1008);
                                if(BVar1 != 0x0)
                                {
                                    local_c = local_c & 0xffff0000 | (ulong) * (uint *)(iVar3 + 0x24);
                                    BVar1   = write_to_file_1008_7e1c(
                                      uVar5, uVar6, (ushort)&local_c, param_5, (char *)0x2, 0x1008);
                                    if(BVar1 != 0x0)
                                    {
                                        BVar1 = write_to_file_1008_7e1c(
                                          uVar5, uVar6, iVar3 + 0x26, uVar4, (char *)0x94, 0x1008);
                                        if(BVar1 != 0x0)
                                        {
                                            BVar1 = write_to_file_1008_7e1c(
                                              uVar5, uVar6, iVar3 + 0x14e, uVar4, (char *)0x54, 0x1008);
                                            if(BVar1 != 0x0)
                                            {
                                                BVar1 = write_to_file_1008_7e1c(
                                                  uVar5, uVar6, iVar3 + 0x1a2, uVar4, (char *)0x54, 0x1008);
                                                if(BVar1 != 0x0)
                                                {
                                                    write_to_file_1030_32e4(
                                                      *(ulong *)(iVar3 + 0x1f6), param_2, param_5);
                                                    BVar1 = pass1_1008_7c2a(param_2, *(char **)(iVar3 + 0x1fa), 0x1008);
                                                    if(BVar1 != 0x0)
                                                    {
                                                        local_c
                                                          = local_c & 0xffff0000 | (ulong) * (uint *)(iVar3 + 0x1fe);
                                                        BVar1 = write_to_file_1008_7e1c(
                                                          uVar5, uVar6, (ushort)&local_c, param_5, (char *)0x2, 0x1008);
                                                        if(BVar1 != 0x0)
                                                        {
                                                            local_c = *(ulong *)(iVar3 + 0x200);
                                                            BVar1   = write_to_file_1008_7e1c(uVar5,
                                                                                            uVar6,
                                                                                            (ushort)&local_c,
                                                                                            param_5,
                                                                                            (char *)0x4,
                                                                                            0x1008);
                                                            if(BVar1 != 0x0)
                                                            {
                                                                local_c = local_c & 0xffff0000
                                                                        | (ulong) * (uint *)(iVar3 + 0x204);
                                                                BVar1 = write_to_file_1008_7e1c(uVar5,
                                                                                                uVar6,
                                                                                                (ushort)&local_c,
                                                                                                param_5,
                                                                                                (char *)0x2,
                                                                                                0x1008);
                                                                if(BVar1 != 0x0)
                                                                {
                                                                    local_c = local_c & 0xffff0000
                                                                            | (ulong) * (uint *)(iVar3 + 0x206);
                                                                    BVar1 = write_to_file_1008_7e1c(uVar5,
                                                                                                    uVar6,
                                                                                                    (ushort)&local_c,
                                                                                                    param_5,
                                                                                                    (char *)0x2,
                                                                                                    0x1008);
                                                                    if(BVar1 != 0x0)
                                                                    {
                                                                        local_c = local_c & 0xffff0000
                                                                                | (ulong) * (uint *)(iVar3 + 0x208);
                                                                        BVar1
                                                                          = write_to_file_1008_7e1c(uVar5,
                                                                                                    uVar6,
                                                                                                    (ushort)&local_c,
                                                                                                    param_5,
                                                                                                    (char *)0x2,
                                                                                                    0x1008);
                                                                        if(BVar1 != 0x0)
                                                                        {
                                                                            local_c = local_c & 0xffff0000
                                                                                    | (ulong) * (uint *)(iVar3 + 0x20a);
                                                                            BVar1 = write_to_file_1008_7e1c(
                                                                              uVar5,
                                                                              uVar6,
                                                                              (ushort)&local_c,
                                                                              param_5,
                                                                              (char *)0x2,
                                                                              0x1008);
                                                                            if(BVar1 != 0x0)
                                                                            {
                                                                                local_c
                                                                                  = local_c & 0xffff0000
                                                                                  | (ulong) * (uint *)(iVar3 + 0x20c);
                                                                                BVar1 = write_to_file_1008_7e1c(
                                                                                  uVar5,
                                                                                  uVar6,
                                                                                  (ushort)&local_c,
                                                                                  param_5,
                                                                                  (char *)0x2,
                                                                                  0x1008);
                                                                                if(BVar1 != 0x0)
                                                                                {
                                                                                    local_c
                                                                                      = local_c & 0xffff0000
                                                                                      | (ulong)
                                                                                          * (uint *)(iVar3 + 0x20e);
                                                                                    BVar1 = write_to_file_1008_7e1c(
                                                                                      uVar5,
                                                                                      uVar6,
                                                                                      (ushort)&local_c,
                                                                                      param_5,
                                                                                      (char *)0x2,
                                                                                      0x1008);
                                                                                    if(BVar1 != 0x0)
                                                                                    {
                                                                                        local_c
                                                                                          = local_c & 0xffff0000
                                                                                          | (ulong)
                                                                                              * (uint *)(iVar3 + 0x214);
                                                                                        BVar1 = write_to_file_1008_7e1c(
                                                                                          uVar5,
                                                                                          uVar6,
                                                                                          (ushort)&local_c,
                                                                                          param_5,
                                                                                          (char *)0x2,
                                                                                          0x1008);
                                                                                        if(BVar1 != 0x0)
                                                                                        {
                                                                                            local_c = *(
                                                                                              undefined4 *)(iVar3
                                                                                                            + 0x216);
                                                                                            BVar1
                                                                                              = write_to_file_1008_7e1c(
                                                                                                uVar5,
                                                                                                uVar6,
                                                                                                (ushort)&local_c,
                                                                                                param_5,
                                                                                                (char *)0x4,
                                                                                                0x1008);
                                                                                            if(BVar1 != 0x0)
                                                                                            {
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
                                }
                            }
                        }
                    }
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


void __stdcall16far file_1038_6118(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    undefined2   uVar1;
    undefined4  *puVar2;
    BOOL16       BVar3;
    ushort       uVar4;
    ushort       uVar5;
    undefined   *puVar6;
    uchar       *puVar7;
    astruct_429 *iVar9;
    ushort       uVar8;
    ushort       uVar9;
    ushort       uVar10;
    SEGPTR       SVar11;
    astruct_18  *paStack1046;
    ushort       uStack1042;
    undefined    local_408[0x400];
    undefined2   local_8;
    undefined4   local_6;

    file_1030_1730(param_1, param_2);
    if(param_3 == 0x0)
    {
        return;
    }
    local_6 = 0x0;
    puVar2  = &local_6;
    file_1008_7548(param_2, (long *)CONCAT22(param_5, puVar2), 0x1008, param_5);
    if(puVar2 != (undefined4 *)0x0)
    {
        uVar8            = (ushort)(param_1 >> 0x10);
        iVar9            = (astruct_429 *)param_1;
        iVar9->field_0xc = local_6;
        uVar9            = (ushort)param_2;
        uVar10           = (ushort)(param_2 >> 0x10);
        BVar3            = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x10, 0x0, uVar8, 0x4, 0x1008);
        if(((((BVar3 != 0x0)
              && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x18, 0x0, uVar8, 0x2, 0x1008),
                  BVar3 != 0x0))
             && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x1a, 0x0, uVar8, 0x2, 0x1008), BVar3 != 0x0))
            && ((BVar3 = read_file_1008_7dee(uVar9, uVar10, (ushort)&local_8, 0x0, param_5, 0x2, 0x1008),
                 BVar3 != 0x0
                   && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x1e, 0x0, uVar8, 0x4, 0x1008),
                       BVar3 != 0x0))))
           && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x22, 0x0, uVar8, 0x2, 0x1008), BVar3 != 0x0))
        {
            iVar9->field_0x1c = local_8;
            BVar3             = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x24, 0x0, uVar8, 0x2, 0x1008);
            if((BVar3 != 0x0)
               && (uVar4 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x26, 0x0, uVar8, 0x94, 0x1008),
                   uVar4 != 0x0))
            {
                if((int)PTR_LOOP_1050_0312 < 0x2)
                {
                    uVar5  = 0x54;
                    SVar11 = 0x54;
                    mem_op_1000_179c(0x54, param_4, 0x1000);
                    paStack1046 = (astruct_18 *)CONCAT22(param_4, uVar4);
                    BVar3       = read_file_1008_7dee(uVar9, uVar10, uVar4, uVar5, (ushort)param_4, SVar11, 0x1008);
                    if(BVar3 == 0x0)
                    {
                    LAB_1038_626a:
                        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                        fn_ptr_1000_17ce(paStack1046, 0x1000);
                        return;
                    }
                    uStack1042 = 0x0;
                    do
                    {
                        uVar5 = switch_1008_72bc(uVar9, uVar10, uStack1042);
                        uVar1 = *(undefined2 *)(uStack1042 * 0x4 + uVar4 + 0x2);
                        *(undefined2 *)(&iVar9->field_0x14e + uVar5 * 0x4) = *(undefined2 *)(uStack1042 * 0x4 + uVar4);
                        *(undefined2 *)(&iVar9->field_0x150 + uVar5 * 0x4) = uVar1;
                        uStack1042                                         = uStack1042 + 0x1;
                    } while((int)uStack1042 < 0x15);
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, uVar4, 0x0, (ushort)param_4, 0x54, 0x1008);
                    if(BVar3 == 0x0)
                        goto LAB_1038_626a;
                    uStack1042 = 0x0;
                    do
                    {
                        uVar5  = switch_1008_72bc(uVar9, uVar10, uStack1042);
                        puVar7 = *(uchar **)(uStack1042 * 0x4 + uVar4 + 0x2);
                        *(undefined2 *)(&iVar9->field_0x1a2 + uVar5 * 0x4) = *(undefined2 *)(uStack1042 * 0x4 + uVar4);
                        *(uchar **)(&iVar9->field_0x1a4 + uVar5 * 0x4)     = puVar7;
                        uStack1042                                         = uStack1042 + 0x1;
                    } while((int)uStack1042 < 0x15);
                    fn_ptr_1000_17ce(paStack1046, 0x1000);
                    param_4 = puVar7;
                }
                else
                {
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x14e, 0x0, uVar8, 0x54, 0x1008);
                    if(BVar3 == 0x0)
                    {
                        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                        return;
                    }
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x1a2, 0x0, uVar8, 0x54, 0x1008);
                    if(BVar3 == 0x0)
                    {
                        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                        return;
                    }
                }
                read_file_1030_33f0(iVar9->field_0x1f6, param_2);
                puVar6 = local_408;
                read_file_1008_7c6e(uVar9, uVar10, (char *)CONCAT22(param_5, puVar6), 0x1008);
                if(puVar6 != (undefined *)0x0)
                {
                    uVar4              = str_op_1008_60e8((char *)CONCAT22(param_5, local_408), (ushort)param_4);
                    iVar9->field_0x1fa = uVar4;
                    iVar9->field_0x1fc = (ushort)param_4;
                    BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x1fe, 0x0, uVar8, 0x2, 0x1008);
                    if(((((BVar3 != 0x0)
                          && (BVar3 = read_file_1008_7dee(uVar9,
                                                          uVar10,
                                                          CONCAT11((char)(param_1 >> 0x8) + '\x02', (char)param_1),
                                                          0x0,
                                                          uVar8,
                                                          0x4,
                                                          0x1008),
                              BVar3 != 0x0))
                         && (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x204, 0x0, uVar8, 0x2, 0x1008),
                             BVar3 != 0x0))
                        && ((
                          (BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x206, 0x0, uVar8, 0x2, 0x1008),
                           BVar3 != 0x0
                             && (BVar3
                                 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x208, 0x0, uVar8, 0x2, 0x1008),
                                 BVar3 != 0x0))
                          && ((BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x20a, 0x0, uVar8, 0x2, 0x1008),
                               BVar3 != 0x0
                                 && ((BVar3 = read_file_1008_7dee(
                                        uVar9, uVar10, &iVar9->field_0x20c, 0x0, uVar8, 0x2, 0x1008),
                                      BVar3 != 0x0
                                        && (BVar3 = read_file_1008_7dee(
                                              uVar9, uVar10, &iVar9->field_0x20e, 0x0, uVar8, 0x2, 0x1008),
                                            BVar3 != 0x0))))))))
                       && ((
                         (int)PTR_LOOP_1050_0312 < 0x2
                         || ((BVar3 = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x214, 0x0, uVar8, 0x2, 0x1008),
                              BVar3 != 0x0
                                && (BVar3
                                    = read_file_1008_7dee(uVar9, uVar10, &iVar9->field_0x216, 0x0, uVar8, 0x4, 0x1008),
                                    BVar3 != 0x0))))))
                    {
                        return;
                    }
                    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                    return;
                }
            }
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
}


void __stdcall16far pass1_1030_de7c(ulong param_1, ulong param_2, ushort param_3)

{
    BOOL16     BVar1;
    undefined4 local_10[0x3];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        local_10[0] = *(undefined4 *)((int)param_1 + 0x20);
        BVar1       = write_to_file_1008_7e1c(
          (ushort)param_2, (ushort)(param_2 >> 0x10), (ushort)local_10, param_3, (char *)0x4, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1030_dec4(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    BOOL16 BVar1;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(((param_3 != 0x0) && (0x1 < (int)PTR_LOOP_1050_0312))
       && (BVar1 = read_file_1008_7dee((ushort)param_2,
                                       (ushort)(param_2 >> 0x10),
                                       (int)param_1 + 0x20,
                                       0x0,
                                       (ushort)(param_1 >> 0x10),
                                       0x4,
                                       0x1008),
           BVar1 == 0x0))
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    return;
}


void __stdcall16far pass1_1030_d61c(ulong param_1, ulong param_2, ushort param_3)

{
    BOOL16     BVar1;
    int        iVar2;
    undefined2 uVar3;
    ushort     uVar4;
    ushort     uVar5;
    undefined4 local_1a;
    undefined *local_16;
    undefined2 local_14;
    undefined4 local_12[0x3];
    int        iStack4;

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        for(iStack4 = 0x0; uVar4 = (ushort)param_2, uVar5 = (ushort)(param_2 >> 0x10), iStack4 < 0xa;
            iStack4 = iStack4 + 0x1)
        {
            uVar3       = (undefined2)(param_1 >> 0x10);
            iVar2       = (int)param_1;
            local_12[0] = *(undefined4 *)(iVar2 + iStack4 * 0xc + 0x20);
            BVar1       = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)local_12, param_3, (char *)0x4, 0x1008);
            if(BVar1 == 0x0)
                goto LAB_1030_d701;
            local_14 = *(undefined2 *)(iVar2 + iStack4 * 0xc + 0x24);
            BVar1    = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)&local_14, param_3, (char *)0x2, 0x1008);
            if(BVar1 == 0x0)
                goto LAB_1030_d701;
            local_16 = (undefined *)*(undefined2 *)(iVar2 + iStack4 * 0xc + 0x26);
            BVar1    = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)&local_16, param_3, (char *)0x2, 0x1008);
            if(BVar1 == 0x0)
                goto LAB_1030_d701;
            local_1a = *(undefined4 *)(iVar2 + iStack4 * 0xc + 0x28);
            BVar1    = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)&local_1a, param_3, (char *)0x4, 0x1008);
            if(BVar1 == 0x0)
                goto LAB_1030_d701;
        }
        local_16 = PTR_LOOP_1050_5812;
        BVar1    = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)&local_16, param_3, (char *)0x2, 0x1008);
        if(BVar1 != 0x0)
        {
            return;
        }
    LAB_1030_d701:
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


void __stdcall16far pass1_1030_d72e(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    ushort     uVar1;
    BOOL16     BVar2;
    int        iVar3;
    ushort     uVar4;
    ushort     uVar5;
    int        iStack10;
    undefined4 local_8;
    ushort     local_4;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(param_3 == 0x0)
    {
        return;
    }
    iStack10 = 0x0;
    while(true)
    {
        uVar4 = (ushort)param_2;
        uVar5 = (ushort)(param_2 >> 0x10);
        if(0x9 < iStack10)
        {
            if((0x3 < (int)PTR_LOOP_1050_0312)
               && (BVar2 = read_file_1008_7dee(
                     uVar4, uVar5, (ushort)&PTR_LOOP_1050_5812, 0x0, (ushort)&USHORT_1050_1050, 0x2, 0x1008),
                   BVar2 == 0x0))
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                return;
            }
            return;
        }
        BVar2 = read_file_1008_7dee(uVar4, uVar5, (ushort)&local_8, 0x0, param_5, 0x4, 0x1008);
        if(BVar2 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
        BVar2 = read_file_1008_7dee(uVar4, uVar5, (ushort)&local_4, 0x0, param_5, 0x2, 0x1008);
        if(BVar2 == 0x0)
            break;
        iVar3                         = iStack10 * 0xc + (int)param_1;
        *(undefined2 *)(iVar3 + 0x20) = (undefined2)local_8;
        *(undefined2 *)(iVar3 + 0x22) = local_8._2_2_;
        uVar1                         = switch_1008_72bc(uVar4, uVar5, local_4);
        *(ushort *)(iVar3 + 0x24)     = uVar1;
        if((int)PTR_LOOP_1050_0312 < 0x2)
        {
            iVar3                         = iStack10 * 0xc + (int)param_1;
            *(undefined2 *)(iVar3 + 0x26) = 0x3;
            *(undefined4 *)(iVar3 + 0x28) = 0x0;
        }
        else
        {
            BVar2 = read_file_1008_7dee(uVar4, uVar5, (ushort)&local_4, 0x0, param_5, 0x2, 0x1008);
            if(BVar2 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                return;
            }
            BVar2 = read_file_1008_7dee(uVar4, uVar5, (ushort)&local_8, 0x0, param_5, 0x4, 0x1008);
            if(BVar2 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                return;
            }
            iVar3                         = iStack10 * 0xc + (int)param_1;
            *(ushort *)(iVar3 + 0x26)     = local_4;
            *(undefined4 *)(iVar3 + 0x28) = local_8;
        }
        iStack10 = iStack10 + 0x1;
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
}


void __stdcall16far pass1_1030_c230(ulong param_1, ulong param_2, ushort param_3)

{
    BOOL16     BVar1;
    undefined2 uVar2;
    ushort     uVar3;
    undefined4 local_10[0x2];
    undefined2 local_8[0x3];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar1 != 0x0)
    {
        uVar2       = (undefined2)(param_1 >> 0x10);
        local_10[0] = *(undefined4 *)((int)param_1 + 0x20);
        uVar3       = (ushort)(param_2 >> 0x10);
        BVar1       = write_to_file_1008_7e1c((ushort)param_2, uVar3, (ushort)local_10, param_3, (char *)0x4, 0x1008);
        if(BVar1 != 0x0)
        {
            local_8[0] = *(undefined2 *)((int)param_1 + 0x24);
            BVar1      = write_to_file_1008_7e1c((ushort)param_2, uVar3, (ushort)local_8, param_3, (char *)0x2, 0x1008);
            if(BVar1 != 0x0)
            {
                return;
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


void __stdcall16far pass1_1030_c29c(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    ushort uVar1;
    BOOL16 BVar2;
    ushort uVar3;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(param_3 != 0x0)
    {
        uVar1 = (ushort)(param_1 >> 0x10);
        uVar3 = (ushort)(param_2 >> 0x10);
        BVar2 = read_file_1008_7dee((ushort)param_2, uVar3, (int)param_1 + 0x20, 0x0, uVar1, 0x4, 0x1008);
        if(BVar2 != 0x0)
        {
            BVar2 = read_file_1008_7dee((ushort)param_2, uVar3, (int)param_1 + 0x24, 0x0, uVar1, 0x2, 0x1008);
            if(BVar2 != 0x0)
            {
                return;
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}


BOOL16 __stdcall16far pass1_1030_c84e(ulong param_1, ulong param_2, ushort param_3)

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


BOOL16 __stdcall16far pass1_1030_c894(ulong param_1, ulong param_2, BOOL16 param_3, uchar *param_4, ushort param_5)

{
    BOOL16     BVar1;
    undefined2 local_4;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(param_3 != 0x0)
    {
        BVar1 = read_file_1008_7dee(
          (ushort)param_2, (ushort)(param_2 >> 0x10), (ushort)&local_4, 0x0, param_5, 0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return BVar1;
        }
        *(undefined2 *)((int)param_1 + 0x20) = local_4;
        param_3                              = 0x1;
    }
    return param_3;
}


void __stdcall16far pass1_1030_b768(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4 uVar1;
    BOOL16     BVar2;
    int        iVar3;
    undefined *puVar4;
    uint       extraout_DX;
    int        iVar5;
    undefined2 uVar6;
    ushort     uVar7;
    ushort     uVar8;
    uint       local_22[0x4];
    undefined  local_1a[0xa];
    ulong      local_10;
    undefined *puStack12;
    uint       uStack10;
    undefined2 local_8[0x3];

    uVar6    = (undefined2)(param_1 >> 0x10);
    iVar5    = (int)param_1;
    local_10 = *(ulong *)(iVar5 + 0x4);
    uVar7    = (ushort)param_2;
    uVar8    = (ushort)(param_2 >> 0x10);
    BVar2    = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_10, param_3, (char *)0x4, 0x1008);
    if((BVar2 != 0x0)
       && (iVar3 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | (ulong)(iVar5 + 0x8), 0x1008, param_3),
           iVar3 != 0x0))
    {
        local_8[0] = *(undefined2 *)(iVar5 + 0xe);
        BVar2      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_8, param_3, (char *)0x2, 0x1008);
        if(BVar2 != 0x0)
        {
            uVar1       = *(undefined4 *)(iVar5 + 0x10);
            local_22[0] = *(uint *)((int)uVar1 + 0x8);
            local_10    = local_10 & 0xffff0000 | (ulong)local_22[0];
            BVar2       = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_22, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
            {
                return;
            }
            pass1_1008_5784((ulong *)CONCAT22(param_3, local_1a), *(ulong *)(iVar5 + 0x10));
            do
            {
                puVar4 = local_1a;
                pass1_1008_5b12(puVar4, param_3);
                if((extraout_DX | (uint)puVar4) == 0x0)
                {
                    return;
                }
                puStack12 = puVar4;
                uStack10  = extraout_DX;
                pass1_1038_75ca(CONCAT22(extraout_DX, puVar4), param_2, (int)puVar4, param_3);
            } while(puVar4 != (undefined *)0x0);
            return;
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
}


void __stdcall16far file_1030_b836(ulong param_1, ulong param_2, uchar *param_3, ushort param_4)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    ushort       uVar3;
    astruct_401 *iVar4;
    BOOL16       BVar4;
    uint         uVar5;
    uint         uVar6;
    uint         uVar7;
    uchar       *puVar8;
    uchar       *extraout_DX;
    ushort       uVar9;
    ushort       uVar10;
    undefined2   uVar11;
    ulong        uVar12;
    undefined2   uVar13;
    uint         local_12[0x7];
    undefined2   local_4;

    iVar4  = (astruct_401 *)param_1;
    iVar4  = (astruct_401 *)&iVar4->field_0x4;
    uVar3  = (ushort)(param_1 >> 0x10);
    uVar9  = (ushort)param_2;
    uVar10 = (ushort)(param_2 >> 0x10);
    BVar4  = read_file_1008_7dee(uVar9, uVar10, (ushort)iVar4, 0x0, uVar3, 0x4, 0x1008);
    if(((BVar4 == 0x0)
        || (BVar4 = read_file_1008_7bc8(
              param_2, (ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x8), 0x1008, param_4),
            BVar4 == 0x0))
       || (BVar4 = read_file_1008_7dee(uVar9, uVar10, (ushort)&local_4, 0x0, param_4, 0x2, 0x1008), BVar4 == 0x0))
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    else
    {
        iVar4->field_0xe = local_4;
        BVar4            = read_file_1008_7dee(uVar9, uVar10, (ushort)local_12, 0x0, param_4, 0x2, 0x1008);
        if(BVar4 != 0x0)
        {
            while(true)
            {
                if(local_12[0] == 0x0)
                {
                    return;
                }
                uVar11      = 0x2a;
                uVar5       = local_12[0];
                local_12[0] = local_12[0] - 0x1;
                uVar12      = param_2;
                mem_op_1000_179c(0x2a, param_3, 0x1000);
                puVar8 = (uchar *)((uint)param_3 | uVar5);
                if(puVar8 == (uchar *)0x0)
                {
                    uVar6  = 0x0;
                    puVar8 = (uchar *)0x0;
                }
                else
                {
                    uVar6 = uVar5;
                    struct_1038_6520((ushort *)CONCAT22(param_3, uVar5));
                }
                uVar13 = (undefined2)(uVar12 >> 0x10);
                uVar7  = uVar6;
                file_1038_774e(CONCAT22(puVar8, uVar6), CONCAT22((int)uVar12, uVar11), puVar8, param_4);
                if(uVar7 == 0x0)
                    break;
                puVar1  = iVar4->field_0x10;
                ppcVar2 = (code **)((int)*iVar4->field_0x10 + 0x4);
                (**ppcVar2)(
                  (int)&PTR_LOOP_1050_1038, (int)puVar1, (int)((ulong)puVar1 >> 0x10), uVar6, puVar8, uVar13, uVar5);
                param_3 = extraout_DX;
            }
        }
    }
    return;
}


void __stdcall16far pass1_1030_7418(ulong param_1, ulong param_2, int param_3, ushort param_4)

{
    ulong        uVar1;
    astruct_731 *iVar2;
    int          iVar3;
    BOOL16       BVar4;
    undefined   *puVar5;
    uint         extraout_DX;
    uint         extraout_DX_00;
    undefined2   uVar6;
    ushort       uVar7;
    ushort       uVar8;
    undefined2   uStack62;
    undefined2   local_2a[0x2];
    undefined    local_26[0xe];
    ulong        local_18;
    ulong        local_14[0x2];
    undefined2   local_c;
    undefined4   local_a;
    undefined2   local_6[0x2];

    pass1_1030_16d6(param_1, param_2, param_4);
    if(param_3 == 0x0)
    {
        return;
    }
    iVar2 = (astruct_731 *)param_1;
    iVar2 = (astruct_731 *)&iVar2->field_0xc;
    iVar3 = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | ZEXT24(iVar2), 0x1008, param_4);
    if(iVar3 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    uVar6   = (undefined2)(param_1 >> 0x10);
    local_c = iVar2->field_0x12;
    uVar7   = (ushort)param_2;
    uVar8   = (ushort)(param_2 >> 0x10);
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_c, param_4, (char *)0x2, 0x1008);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    local_6[0] = iVar2->field_0x14;
    BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    local_18 = iVar2->field_0x16;
    BVar4    = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_18, param_4, (char *)0x4, 0x1008);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    write_to_file_1008_7954(param_2, iVar2->field_0x1e, BVar4, 0x1008, param_4);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_2, iVar2->field_0x22, 0x1008, param_4);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    write_to_file_1008_7a22(param_2, iVar2->field_0x26, 0x1008, param_4);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    local_a = iVar2->field_0x2a;
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_a, param_4, (char *)0x4, 0x1008);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    local_c = iVar2->field_0x32;
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_c, param_4, (char *)0x2, 0x1008);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    local_c = iVar2->field_0x34;
    BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_c, param_4, (char *)0x2, 0x1008);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    pass1_1008_79f0(param_2, iVar2->field_0x36, 0x1008, param_4);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    if(iVar2->field_0x3a == 0x0)
    {
        local_18 = local_18 & 0xffff0000;
    }
    else
    {
        uVar1    = iVar2->field_0x3a;
        local_18 = local_18 & 0xffff0000 | (ulong) * (uint *)((int)uVar1 + 0x8);
    }
    local_6[0] = (undefined2)local_18;
    BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
    if(BVar4 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
    }
    pass1_1008_5784((ulong *)CONCAT22(param_4, local_26), iVar2->field_0x3a);
    while(true)
    {
        puVar5 = local_26;
        pass1_1008_5b12(puVar5, param_4);
        local_14[0] = CONCAT22(extraout_DX, puVar5);
        if((extraout_DX | (uint)puVar5) == 0x0)
        {
            if(iVar2->field_0x3e == 0x0)
            {
                uStack62 = 0x0;
            }
            else
            {
                uVar1    = iVar2->field_0x3e;
                uStack62 = *(undefined2 *)((int)uVar1 + 0x8);
            }
            local_2a[0] = uStack62;
            BVar4       = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_2a, param_4, (char *)0x2, 0x1008);
            if(BVar4 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                return;
            }
            pass1_1008_5784((ulong *)CONCAT22(param_4, local_26), iVar2->field_0x3e);
            while(true)
            {
                puVar5 = local_26;
                pass1_1008_5b12(puVar5, param_4);
                if((extraout_DX_00 | (uint)puVar5) == 0x0)
                {
                    return;
                }
                local_18 = local_18 & 0xffff0000 | (ulong) * (uint *)(puVar5 + 0x4);
                BVar4    = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_18, param_4, (char *)0x2, 0x1008);
                if(BVar4 == 0x0)
                {
                    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                    return;
                }
                local_14[0] = local_14[0] & 0xffff0000 | (ulong) * (uint *)(puVar5 + 0x6);
                BVar4       = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_14, param_4, (char *)0x2, 0x1008);
                if(BVar4 == 0x0)
                {
                    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                    return;
                }
                local_c = *(undefined2 *)(puVar5 + 0x8);
                BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_c, param_4, (char *)0x2, 0x1008);
                if(BVar4 == 0x0)
                    break;
                local_c = *(undefined2 *)(puVar5 + 0xa);
                BVar4   = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)&local_c, param_4, (char *)0x2, 0x1008);
                if(BVar4 == 0x0)
                {
                    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                    return;
                }
                local_6[0] = *(undefined2 *)(puVar5 + 0xc);
                BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
                if(BVar4 == 0x0)
                {
                    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                    return;
                }
            }
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
        }
        local_6[0] = *(undefined2 *)(puVar5 + 0x4);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
        if(BVar4 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
        }
        local_6[0] = *(undefined2 *)((int)local_14[0] + 0x6);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
        if(BVar4 == 0x0)
            break;
        local_6[0] = *(undefined2 *)((int)local_14[0] + 0x8);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
        if(BVar4 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
        }
        local_6[0] = *(undefined2 *)((int)local_14[0] + 0xa);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
        if(BVar4 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
        }
        local_6[0] = *(undefined2 *)((int)local_14[0] + 0xc);
        BVar4      = write_to_file_1008_7e1c(uVar7, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
        if(BVar4 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1030_778c(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    long         lVar1;
    code       **ppcVar2;
    astruct_387 *iVar3;
    BOOL16       BVar3;
    int          iVar6;
    long        *plVar7;
    ulong       *puVar8;
    undefined2   extraout_DX;
    uint         uVar9;
    uchar       *puVar10;
    undefined2   extraout_DX_00;
    astruct_389 *iVar4;
    astruct_391 *iVar5;
    ushort       uVar11;
    undefined2   uVar12;
    ushort       uVar13;
    ushort       uVar14;
    uint         local_56[0x2];
    uint         uStack82;
    astruct_99  *paStack74;
    undefined2   local_46[0x2];
    undefined2   local_42[0x2];
    ulong        local_3e[0x3];
    astruct_99  *paStack50;
    uint         local_2e;
    astruct_99  *paStack44;
    undefined2   local_28[0x2];
    undefined2   local_24[0x2];
    uint         local_20[0x9];
    uint         uStack14;
    undefined2   local_4;
    astruct_388 *uVar5;
    astruct_390 *uVar8;

    file_1030_1730(param_1, param_2);
    if(param_3 != 0x0)
    {
        iVar3 = (astruct_387 *)param_1;
        iVar3 = (astruct_387 *)&iVar3->field_0xc;
        BVar3 = read_file_1008_7bc8(param_2, (ushort *)(param_1 & 0xffff0000 | ZEXT24(iVar3)), 0x1008, param_5);
        if(BVar3 != 0x0)
        {
            uVar13 = (ushort)param_2;
            uVar14 = (ushort)(param_2 >> 0x10);
            BVar3  = read_file_1008_7dee(uVar13, uVar14, (ushort)&local_4, 0x0, param_5, 0x2, 0x1008);
            if(BVar3 != 0x0)
            {
                uVar11            = (ushort)(param_1 >> 0x10);
                iVar3->field_0x12 = local_4;
                BVar3             = read_file_1008_7dee(uVar13, uVar14, (ushort)&local_4, 0x0, param_5, 0x2, 0x1008);
                if(BVar3 != 0x0)
                {
                    iVar3->field_0x14 = local_4;
                    BVar3 = read_file_1008_7dee(uVar13, uVar14, &iVar3->field_0x16, 0x0, uVar11, 0x4, 0x1008);
                    if(BVar3 != 0x0)
                    {
                        plVar7 = (long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x1e);
                        file_1008_76e4(param_2, plVar7, 0x1008, param_5, (ushort)param_4);
                        if(((((int)plVar7 != 0x0)
                             && (iVar6
                                 = file_1008_77cc(param_2,
                                                  (long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x22),
                                                  param_4,
                                                  0x1008,
                                                  param_5),
                                 iVar6 != 0x0))
                            && (iVar6 = file_1008_77cc(param_2,
                                                       (long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x26),
                                                       param_4,
                                                       0x1008,
                                                       param_5),
                                iVar6 != 0x0))
                           && (BVar3 = read_file_1008_7dee(
                                 uVar13, uVar14, (ushort)&iVar3->field_0x2a, 0x0, uVar11, 0x4, 0x1008),
                               BVar3 != 0x0))
                        {
                            if(iVar3->field_0x2a != 0x0)
                            {
                                lVar1 = iVar3->field_0x2a;
                                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)lVar1, (uint)((ulong)lVar1 >> 0x10));
                                iVar3->field_0x2e = BVar3;
                                iVar3->field_0x30 = param_4;
                            }
                            if((int)PTR_LOOP_1050_0312 < 0x2)
                            {
                                return;
                            }
                            BVar3 = read_file_1008_7dee(uVar13, uVar14, &iVar3->field_0x32, 0x0, uVar11, 0x2, 0x1008);
                            if((BVar3 != 0x0)
                               && (BVar3
                                   = read_file_1008_7dee(uVar13, uVar14, &iVar3->field_0x34, 0x0, uVar11, 0x2, 0x1008),
                                   BVar3 != 0x0))
                            {
                                puVar8 = (ulong *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x36);
                                pass1_1008_766e(param_2, puVar8, param_5, 0x1008, param_4);
                                if(((int)puVar8 != 0x0)
                                   && (BVar3 = read_file_1008_7dee(
                                         uVar13, uVar14, (ushort)local_20, 0x0, param_5, 0x2, 0x1008),
                                       BVar3 != 0x0))
                                {
                                    for(uStack14 = 0x0; uStack14 < local_20[0]; uStack14 = uStack14 + 0x1)
                                    {
                                        local_3e[0] = _PTR_LOOP_1050_68a2;
                                        paStack50   = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                                        uVar9       = (uint)((ulong)paStack50 >> 0x10);
                                        uVar5       = (astruct_388 *)paStack50;
                                        puVar10     = (uchar *)(uVar9 | (uint)uVar5);
                                        if(puVar10 == (uchar *)0x0)
                                        {
                                            paStack44 = (astruct_99 *)0x0;
                                        }
                                        else
                                        {
                                            paStack50->field_0x0 = 0x389a;
                                            uVar5->field_0x2     = 0x1008;
                                            uVar5->field_0x4     = 0x0;
                                            uVar5->field_0x6     = 0x0;
                                            uVar5->field_0x8     = 0x0;
                                            uVar5->field_0xa     = 0x0;
                                            uVar5->field_0xc     = 0x0;
                                            paStack50->field_0x0 = 0x56ce;
                                            uVar5->field_0x2     = 0x1018;
                                            paStack44            = paStack50;
                                        }
                                        BVar3 = read_file_1008_7dee(
                                          uVar13, uVar14, (ushort)local_28, 0x0, param_5, 0x2, 0x1008);
                                        if(((BVar3 == 0x0)
                                            || (BVar3 = read_file_1008_7dee(
                                                  uVar13, uVar14, (ushort)local_24, 0x0, param_5, 0x2, 0x1008),
                                                BVar3 == 0x0))
                                           || ((BVar3 = read_file_1008_7dee(
                                                  uVar13, uVar14, (ushort)&local_2e, 0x0, param_5, 0x2, 0x1008),
                                                BVar3 == 0x0
                                                  || ((BVar3 = read_file_1008_7dee(uVar13,
                                                                                   uVar14,
                                                                                   (int)paStack44 + 0xa,
                                                                                   0x0,
                                                                                   (ushort)((ulong)paStack44 >> 0x10),
                                                                                   0x2,
                                                                                   0x1008),
                                                       BVar3 == 0x0
                                                         || (BVar3
                                                             = read_file_1008_7dee(uVar13,
                                                                                   uVar14,
                                                                                   (int)paStack44 + 0xc,
                                                                                   0x0,
                                                                                   (ushort)((ulong)paStack44 >> 0x10),
                                                                                   0x2,
                                                                                   0x1008),
                                                             BVar3 == 0x0))))))
                                            goto LAB_1030_77be;
                                        uVar12           = (undefined2)((ulong)paStack44 >> 0x10);
                                        iVar4            = (astruct_389 *)paStack44;
                                        iVar4->field_0x4 = local_28[0];
                                        iVar4->field_0x6 = local_24[0];
                                        iVar4->field_0x8 = local_2e;
                                        if(iVar3->field_0x3a == (undefined4 *)0x0)
                                        {
                                            uVar9 = local_2e;
                                            mem_op_1000_179c(0xc, puVar10, 0x1000);
                                            paStack50 = (astruct_99 *)CONCAT22(puVar10, uVar9);
                                            if(((uint)puVar10 | uVar9) == 0x0)
                                            {
                                                iVar3->field_0x3a = (undefined4 *)0x0;
                                            }
                                            else
                                            {
                                                set_struct_1008_574a((astruct_21 *)CONCAT22(puVar10, uVar9));
                                                *(uint *)&iVar3->field_0x3a                    = uVar9;
                                                *(undefined2 *)((int)&iVar3->field_0x3a + 0x2) = extraout_DX;
                                            }
                                        }
                                        ppcVar2 = (code **)((int)*iVar3->field_0x3a + 0x8);
                                        (**ppcVar2)();
                                    }
                                    BVar3 = read_file_1008_7dee(
                                      uVar13, uVar14, (ushort)local_56, 0x0, param_5, 0x2, 0x1008);
                                    if(BVar3 != 0x0)
                                    {
                                        uStack82 = 0x0;
                                        while(true)
                                        {
                                            if(local_56[0] <= uStack82)
                                            {
                                                return;
                                            }
                                            paStack44 = (astruct_99 *)_PTR_LOOP_1050_68a2;
                                            paStack50 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                                            uVar9     = (uint)((ulong)paStack50 >> 0x10);
                                            uVar8     = (astruct_390 *)paStack50;
                                            puVar10   = (uchar *)(uVar9 | (uint)uVar8);
                                            if(puVar10 == (uchar *)0x0)
                                            {
                                                paStack74 = (astruct_99 *)0x0;
                                            }
                                            else
                                            {
                                                paStack50->field_0x0 = 0x389a;
                                                uVar8->field_0x2     = 0x1008;
                                                uVar8->field_0x4     = 0x0;
                                                uVar8->field_0x6     = 0x0;
                                                uVar8->field_0x8     = 0x0;
                                                uVar8->field_0xa     = 0x0;
                                                uVar8->field_0xc     = 0x0;
                                                paStack50->field_0x0 = 0x56ce;
                                                uVar8->field_0x2     = 0x1018;
                                                paStack74            = paStack50;
                                            }
                                            BVar3 = read_file_1008_7dee(
                                              uVar13, uVar14, (ushort)local_46, 0x0, param_5, 0x2, 0x1008);
                                            if((((BVar3 == 0x0)
                                                 || (BVar3 = read_file_1008_7dee(
                                                       uVar13, uVar14, (ushort)local_42, 0x0, param_5, 0x2, 0x1008),
                                                     BVar3 == 0x0))
                                                || (BVar3 = read_file_1008_7dee(
                                                      uVar13, uVar14, (ushort)local_3e, 0x0, param_5, 0x2, 0x1008),
                                                    BVar3 == 0x0))
                                               || ((
                                                 BVar3 = read_file_1008_7dee(uVar13,
                                                                             uVar14,
                                                                             (int)paStack74 + 0xa,
                                                                             0x0,
                                                                             (ushort)((ulong)paStack74 >> 0x10),
                                                                             0x2,
                                                                             0x1008),
                                                 BVar3 == 0x0
                                                   || (BVar3 = read_file_1008_7dee(uVar13,
                                                                                   uVar14,
                                                                                   (int)paStack74 + 0xc,
                                                                                   0x0,
                                                                                   (ushort)((ulong)paStack74 >> 0x10),
                                                                                   0x2,
                                                                                   0x1008),
                                                       BVar3 == 0x0))))
                                                break;
                                            uVar12           = (undefined2)((ulong)paStack74 >> 0x10);
                                            iVar5            = (astruct_391 *)paStack74;
                                            iVar5->field_0x4 = local_46[0];
                                            iVar5->field_0x6 = local_42[0];
                                            iVar5->field_0x8 = (uint)local_3e[0];
                                            if(iVar3->field_0x3e == (undefined4 *)0x0)
                                            {
                                                mem_op_1000_179c(0xc, puVar10, 0x1000);
                                                paStack50 = (astruct_99 *)CONCAT22(puVar10, (uint)local_3e[0]);
                                                if(((uint)puVar10 | (uint)local_3e[0]) == 0x0)
                                                {
                                                    iVar3->field_0x3e = (undefined4 *)0x0;
                                                }
                                                else
                                                {
                                                    set_struct_1008_574a(
                                                      (astruct_21 *)CONCAT22(puVar10, (uint)local_3e[0]));
                                                    *(uint *)&iVar3->field_0x3e                    = (uint)local_3e[0];
                                                    *(undefined2 *)((int)&iVar3->field_0x3e + 0x2) = extraout_DX_00;
                                                }
                                            }
                                            ppcVar2 = (code **)((int)*iVar3->field_0x3e + 0x8);
                                            (**ppcVar2)();
                                            uStack82 = uStack82 + 0x1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    LAB_1030_77be:
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}


BOOL16 __stdcall16far pass1_1030_5c1a(ulong param_1, ulong param_2, uint16_t param_3)

{
    BOOL16 BVar1;

    BVar1 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar1 != 0x0)
    {
        BVar1 = write_to_file_1008_7e1c(
          (ushort)param_2, (ushort)(param_2 >> 0x10), (ushort)param_1, (ushort)(param_1 >> 0x10), (char *)0x24, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}


BOOL16 __stdcall16far read_file_1030_5c52(undefined4 param_1, undefined4 param_2, uint16_t param_3, uint16_t param_4)

{
    BOOL16 BVar1;
    ushort uVar2;

    uVar2 = (ushort)((ulong)param_2 >> 0x10);
    read_file_1008_7cfe((ushort)param_2, uVar2, 0x9, 0x1008, param_4);
    if(param_3 != 0x0)
    {
        BVar1 = read_file_1008_7dee(
          (ushort)param_2, uVar2, (ushort)param_1, 0x0, (ushort)((ulong)param_1 >> 0x10), 0x24, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return BVar1;
        }
        param_3 = 0x1;
    }
    return param_3;
}


void __stdcall16far pass1_1030_5dbe(ulong param_1, ulong param_2, uint16_t param_3, ushort param_4)

{
    ulong      uVar1;
    undefined4 uVar2;
    ushort     uVar3;
    BOOL16     BVar4;
    int        iVar5;
    int        iVar6;
    undefined2 uVar7;
    undefined2 local_c[0x5];

    uVar3 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if(uVar3 != 0x0)
    {
        uVar7 = (undefined2)(param_1 >> 0x10);
        iVar6 = (int)param_1;
        BVar4 = pass1_1008_7c2a(param_2, *(char **)*(char **)(iVar6 + 0x10), 0x1008);
        if((BVar4 != 0x0)
           && (uVar1 = *(ulong *)(iVar6 + 0x10),
               iVar5
               = write_to_file_1008_7b4c(param_2, uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x4), 0x1008, param_4),
               iVar5 != 0x0))
        {
            uVar2      = *(undefined4 *)(iVar6 + 0x10);
            local_c[0] = *(undefined2 *)((int)uVar2 + 0xa);
            uVar3      = (ushort)(param_2 >> 0x10);
            BVar4      = write_to_file_1008_7e1c((ushort)param_2, uVar3, (ushort)local_c, param_4, (char *)0x2, 0x1008);
            if(BVar4 != 0x0)
            {
                uVar2 = *(undefined4 *)(iVar6 + 0x10);
                if(*(int *)((int)uVar2 + 0xa) == 0x0)
                {
                    return;
                }
                uVar2 = *(undefined4 *)(iVar6 + 0x10);
                uVar7 = (undefined2)((ulong)uVar2 >> 0x10);
                iVar6 = (int)uVar2;
                uVar2 = *(undefined4 *)(iVar6 + 0xc);
                BVar4 = write_to_file_1008_7e1c((ushort)param_2,
                                                uVar3,
                                                (ushort)uVar2,
                                                (ushort)((ulong)uVar2 >> 0x10),
                                                (char *)(ulong)(uint)(*(int *)(iVar6 + 0xa) * 0x2),
                                                0x1008);
                if(BVar4 != 0x0)
                {
                    return;
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1030_5e70(ulong param_1, ulong param_2, int param_3, undefined *param_4, ushort param_5)

{
    ulong      uVar1;
    undefined4 uVar2;
    ushort     uVar3;
    undefined *puVar4;
    ushort     uVar5;
    BOOL16     BVar6;
    ushort     uVar7;
    uchar     *puVar8;
    int        iVar9;
    int        unaff_DI;
    undefined2 uVar10;
    ushort    *puVar11;
    int        iVar12;
    undefined2 uVar13;
    ushort     uVar14;
    undefined4 uStack1034;
    undefined  local_402[0x400];

    iVar12 = (int)param_1;
    uVar13 = (undefined2)(param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if(param_3 != 0x0)
    {
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4, 0x1000);
            PTR_LOOP_1050_5f2e = param_4;
        }
        else
        {
        }
        uVar3      = fn_ptr_op_1000_1708(0x10, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
        uStack1034 = CONCAT22(PTR_LOOP_1050_5f2e, uVar3);
        puVar8     = (uchar *)((uint)PTR_LOOP_1050_5f2e | uVar3);
        if(puVar8 == (uchar *)0x0)
        {
            *(undefined4 *)(iVar12 + 0x10) = 0x0;
        }
        else
        {
            puVar11                        = pass1_1008_3e38((ushort *)CONCAT22(PTR_LOOP_1050_5f2e, uVar3 + 0x4));
            puVar8                         = (uchar *)((ulong)puVar11 >> 0x10);
            *(undefined4 *)(iVar12 + 0x10) = uStack1034;
        }
        puVar4 = local_402;
        uVar3  = (ushort)param_2;
        uVar14 = (ushort)(param_2 >> 0x10);
        read_file_1008_7c6e(uVar3, uVar14, (char *)CONCAT22(param_5, puVar4), 0x1008);
        if(puVar4 != (undefined *)0x0)
        {
            uVar5                           = str_op_1008_60e8((char *)CONCAT22(param_5, local_402), (ushort)puVar8);
            puVar11                         = *(ushort **)(iVar12 + 0x10);
            *puVar11                        = uVar5;
            *(uchar **)((int)puVar11 + 0x2) = puVar8;
            uVar1                           = *(ulong *)(iVar12 + 0x10);
            BVar6                           = read_file_1008_7bc8(
              param_2, (ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x4)), 0x1008, param_5);
            if(BVar6 != 0x0)
            {
                uVar2 = *(undefined4 *)(iVar12 + 0x10);
                BVar6 = read_file_1008_7dee(
                  uVar3, uVar14, (int)uVar2 + 0xa, 0x0, (ushort)((ulong)uVar2 >> 0x10), 0x2, 0x1008);
                if(BVar6 != 0x0)
                {
                    uVar2  = *(undefined4 *)(iVar12 + 0x10);
                    uVar10 = (undefined2)((ulong)uVar2 >> 0x10);
                    iVar9  = (int)uVar2;
                    if(*(int *)(iVar9 + 0xa) == 0x0)
                    {
                    LAB_1030_5fb7:
                        puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_5, puVar8, unaff_DI);
                        pass1_1018_04ca((ulong)puVar11, *(ulong *)(iVar12 + 0x4));
                        return;
                    }
                    uVar5 = *(int *)(iVar9 + 0xa) * 0x2;
                    uVar7 = uVar5;
                    mem_op_1000_179c(uVar5, puVar8, 0x1000);
                    uVar2                    = *(undefined4 *)(iVar12 + 0x10);
                    uVar10                   = (undefined2)((ulong)uVar2 >> 0x10);
                    iVar9                    = (int)uVar2;
                    *(ushort *)(iVar9 + 0xc) = uVar7;
                    *(uchar **)(iVar9 + 0xe) = puVar8;
                    uVar2                    = *(undefined4 *)(iVar12 + 0x10);
                    uVar2                    = *(undefined4 *)((int)uVar2 + 0xc);
                    BVar6                    = read_file_1008_7dee(
                      uVar3, uVar14, (ushort)uVar2, 0x0, (ushort)((ulong)uVar2 >> 0x10), uVar5, 0x1008);
                    if(BVar6 != 0x0)
                        goto LAB_1030_5fb7;
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}


uint16_t __stdcall16far
read_file_1030_4e70(ulong param_1, undefined4 *param_2, byte **param_3, long param_4, uint16_t param_5)

{
    uint       uVar1;
    HFILE16    HVar2;
    uint16_t   uVar3;
    ushort     unaff_SS;
    ulong      uVar4;
    long       lVar5;
    byte      *pbStack60;
    long       lStack56;
    undefined4 uStack20;

    *param_3 = (byte *)0x0;
    *param_2 = 0x0;
    if(param_4 != 0x0)
    {
        uVar4   = pass1_1030_5164(param_1, param_4, unaff_SS);
        param_5 = (uint16_t)(uVar4 >> 0x10);
        uVar1   = dos3_call_1000_51aa((ushort)&stack0xfffe);
        if(uVar1 == 0x0)
        {
            *param_2 = uStack20;
            HVar2    = _lopen16((LPCSTR)&PTR_LOOP_1050_1000, 0x0);
            if(HVar2 != 0xffff)
            {
                lVar5
                  = mem_op_1000_0a48(0x1, (uint)*param_2, (int)((ulong)*param_2 >> 0x10), _PTR_LOOP_1050_5f2c, 0x1000);
                param_5                       = (uint16_t)((ulong)lVar5 >> 0x10);
                *(int *)param_3               = (int)lVar5;
                *(uint *)((int)param_3 + 0x2) = param_5;
                if((param_5 | *(uint *)param_3) != 0x0)
                {
                    lStack56
                      = WIN16_hread(0x1000, (SEGPTR)*param_2, CONCAT22((int)*param_3, (int)((ulong)*param_2 >> 0x10)));
                    uVar3 = (uint16_t)((ulong)lStack56 >> 0x10);
                    _lclose16((HFILE16)s_tile2_bmp_1050_1538);
                    pbStack60 = *param_3;
                    while(lStack56 != 0x0)
                    {
                        if((*(byte *)(*pbStack60 + 0x608b) & 0x20) == 0x0)
                        {
                            *pbStack60 = *pbStack60 + 0x80;
                        }
                        pbStack60 = (byte *)((ulong)pbStack60 & 0xffff0000 | (ulong)((int)pbStack60 + 0x1));
                        lStack56  = lStack56 + -0x1;
                    }
                    return uVar3;
                }
            }
        }
    }
    return param_5;
}


void __stdcall16far pass1_1030_5044(ulong param_1, ushort param_2, uint16_t param_3)

{
    uint       uVar1;
    char      *pcVar2;
    long      *plVar3;
    uint       uVar4;
    ushort     uVar5;
    undefined2 uVar6;
    int        iVar7;
    char      *pcVar8;
    undefined2 extraout_DX;
    undefined2 extraout_DX_00;
    ushort     uVar10;
    ushort     uVar11;
    ulong      uStack28;
    uint       uStack24;
    ulong      uStack22;
    u16        uStack14;
    uint       uStack12;
    long       local_a;
    char      *local_6;
    ulong      uVar9;

    plVar3             = &local_a;
    PTR_LOOP_1050_5f2e = (undefined *)read_file_1030_4e70(param_1,
                                                          (undefined4 *)CONCAT22(param_2, plVar3),
                                                          (byte **)CONCAT22(param_2, &local_6),
                                                          (long)s_bldgops_dat_1050_5708,
                                                          param_3);
    pcVar2             = local_6;
    if(plVar3 != (long *)0x0)
    {
        uVar10 = (ushort)param_1;
        uVar11 = (ushort)(param_1 >> 0x10);
        pcVar8 = local_6;
        pass1_1030_4e34(uVar10, uVar11, local_a, local_6);
        uVar4 = (uint)pcVar8;
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e, 0x1000);
        }
        else
        {
        }
        uVar5 = fn_ptr_op_1000_1708(uVar4 * 0xae, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
        uVar9 = (ulong)uVar5;
        uStack28 = CONCAT22(PTR_LOOP_1050_5f2e, uVar5);
        if(((uint)PTR_LOOP_1050_5f2e | uVar5) == 0x0)
        {
            *(undefined4 *)(uVar10 + 0x15c) = 0x0;
        }
        else
        {
            pass1_1000_5586((uchar *)0x51f0, 0x1030, uVar4, 0xae, uVar5, (ushort)PTR_LOOP_1050_5f2e);
            *(ulong *)(uVar10 + 0x15c) = uStack28;
            uVar9                      = uStack28;
        }
        uVar6 = (undefined2)uVar9;
        pass1_1030_4dbc(param_1, (ulong)local_6, (ulong)pcVar8 & 0xffff);
        uStack22 = CONCAT22(extraout_DX, uVar6);
        for(uStack24 = 0x0; uStack24 < uVar4; uStack24 = uStack24 + 0x1)
        {
            uVar1 = *(uint *)(uVar10 + 0x15e);
            iVar7 = *(int *)(uVar10 + 0x15c) + uStack24 * 0xae;
            pass1_1030_4c52(uVar10, uVar11, CONCAT22(uVar1, iVar7), uStack22, uVar1, param_2);
            pass1_1030_4dbc(param_1, 0x0, 0x0);
            uStack22 = CONCAT22(extraout_DX_00, iVar7);
        }
        uStack12 = (uint)((ulong)pcVar2 >> 0x10);
        uStack14 = (u16)pcVar2;
        if((uStack12 | uStack14) != 0x0)
        {
            call_fn_ptr_1000_0dc6(uStack14, uStack12, 0x1000);
        }
    }
    return;
}


void __stdcall16far pass1_1030_56f6(ulong param_1, ulong param_2, uint16_t param_3, ushort param_4)

{
    int       *piVar1;
    ulong      uVar2;
    undefined4 uVar3;
    ushort     uVar4;
    BOOL16     BVar5;
    int        iVar6;
    int        iVar7;
    undefined2 uVar8;
    ushort     uVar9;
    undefined2 local_e[0x3];
    undefined2 local_8[0x2];
    int        iStack4;

    uVar4 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if(uVar4 != 0x0)
    {
        uVar8      = (undefined2)(param_1 >> 0x10);
        iVar7      = (int)param_1;
        local_e[0] = *(undefined2 *)*(undefined4 *)(iVar7 + 0x10);
        uVar4      = (ushort)param_2;
        uVar9      = (ushort)(param_2 >> 0x10);
        BVar5      = write_to_file_1008_7e1c(uVar4, uVar9, (ushort)local_e, param_4, (char *)0x2, 0x1008);
        if(BVar5 != 0x0)
        {
            uVar3      = *(undefined4 *)(iVar7 + 0x10);
            local_8[0] = *(undefined2 *)((int)uVar3 + 0x2);
            BVar5      = write_to_file_1008_7e1c(uVar4, uVar9, (ushort)local_8, param_4, (char *)0x2, 0x1008);
            if((BVar5 != 0x0)
               && (uVar3 = *(undefined4 *)(iVar7 + 0x10),
                   BVar5 = pass1_1008_7c2a(param_2, *(char **)((int)uVar3 + 0x4), 0x1008),
                   BVar5 != 0x0))
            {
                uVar3      = *(undefined4 *)(iVar7 + 0x10);
                local_8[0] = *(undefined2 *)((int)uVar3 + 0x1a);
                BVar5      = write_to_file_1008_7e1c(uVar4, uVar9, (ushort)local_8, param_4, (char *)0x2, 0x1008);
                if(BVar5 != 0x0)
                {
                    for(iStack4 = 0x0; uVar3 = *(undefined4 *)(iVar7 + 0x10),
                    piVar1                   = (int *)((int)uVar3 + 0x1a),
                    *piVar1 != iStack4 && iStack4 <= *piVar1;
                        iStack4 = iStack4 + 0x1)
                    {
                        uVar3 = *(undefined4 *)(iVar7 + 0x10);
                        uVar2 = *(ulong *)((int)uVar3 + 0x16);
                        iVar6 = write_to_file_1008_7b4c(
                          param_2, uVar2 & 0xffff0000 | (ulong)(uint)((int)uVar2 + iStack4 * 0x6), 0x1008, param_4);
                        if(iVar6 == 0x0)
                            goto LAB_1030_5734;
                    }
                    iVar6
                      = write_to_file_1008_7b4c(param_2, param_1 & 0xffff0000 | (ulong)(iVar7 + 0x14), 0x1008, param_4);
                    if(iVar6 != 0x0)
                    {
                        local_8[0] = *(undefined2 *)(iVar7 + 0x1c);
                        BVar5 = write_to_file_1008_7e1c(uVar4, uVar9, (ushort)local_8, param_4, (char *)0x2, 0x1008);
                        if(BVar5 != 0x0)
                        {
                            return;
                        }
                    }
                }
            }
        }
    LAB_1030_5734:
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1030_581e(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    int         *piVar1;
    int          iVar2;
    ulong        uVar3;
    ushort       uVar4;
    BOOL16       BVar5;
    undefined   *puVar6;
    ushort       uVar7;
    undefined4   uVar8;
    uchar       *puVar9;
    astruct_380 *iVar9;
    undefined2   uVar10;
    uchar        in_AF;
    ushort       uVar11;
    ushort       uVar12;
    undefined4   uStack1040;
    int          iStack1036;
    undefined    local_408[0x400];
    undefined4   uStack8;
    int          local_4;
    astruct_381 *iVar12;

    iVar12 = (astruct_381 *)param_1;
    uVar12 = (ushort)(param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if(param_3 != 0x0)
    {
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4, 0x1000);
            PTR_LOOP_1050_5f2e = param_4;
        }
        else
        {
        }
        uVar4  = fn_ptr_op_1000_1708(0x20, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
        puVar9 = (uchar *)((uint)PTR_LOOP_1050_5f2e | uVar4);
        if(puVar9 == (uchar *)0x0)
        {
            uVar4  = 0x0;
            puVar9 = (uchar *)0x0;
        }
        else
        {
            pass1_1030_84ae(CONCAT22(PTR_LOOP_1050_5f2e, uVar4));
        }
        iVar12->field_0x10 = uVar4;
        iVar12->field_0x12 = puVar9;
        uVar4              = (ushort)param_2;
        uVar11             = (ushort)(param_2 >> 0x10);
        BVar5              = read_file_1008_7dee(uVar4, uVar11, (ushort)&local_4, 0x0, param_5, 0x2, 0x1008);
        if(BVar5 != 0x0)
        {
            uVar8   = *(undefined4 *)((int)_PTR_LOOP_1050_65e2 + 0x52);
            uStack8 = uVar8;
            pass1_1030_4782(param_5, in_AF, puVar9, (ushort)uVar8, (ushort)((ulong)uVar8 >> 0x10), 0x0, 0x1, local_4);
            iVar12->field_0x10 = (ushort)uVar8;
            iVar12->field_0x12 = puVar9;
            BVar5 = read_file_1008_7dee(uVar4, uVar11, iVar12->field_0x10 + 0x2, 0x0, (ushort)puVar9, 0x2, 0x1008);
            if(BVar5 != 0x0)
            {
                puVar6 = local_408;
                read_file_1008_7c6e(uVar4, uVar11, (char *)CONCAT22(param_5, puVar6), 0x1008);
                if(puVar6 != (undefined *)0x0)
                {
                    uVar8 = *(undefined4 *)&iVar12->field_0x10;
                    fn_ptr_1000_17ce(*(astruct_18 **)((int)uVar8 + 0x4), 0x1000);
                    uVar7            = str_op_1008_60e8((char *)CONCAT22(param_5, local_408), (ushort)puVar9);
                    uVar8            = *(undefined4 *)&iVar12->field_0x10;
                    uVar10           = (undefined2)((ulong)uVar8 >> 0x10);
                    iVar9            = (astruct_380 *)uVar8;
                    iVar9->field_0x4 = uVar7;
                    iVar9->field_0x6 = puVar9;
                    uVar8            = *(undefined4 *)&iVar12->field_0x10;
                    BVar5            = read_file_1008_7dee(
                      uVar4, uVar11, (int)uVar8 + 0x1a, 0x0, (ushort)((ulong)uVar8 >> 0x10), 0x2, 0x1008);
                    if(BVar5 != 0x0)
                    {
                        uVar8 = *(undefined4 *)&iVar12->field_0x10;
                        iVar2 = *(int *)((int)uVar8 + 0x1a);
                        uVar7 = iVar2 * 0x6;
                        mem_op_1000_179c(uVar7, puVar9, 0x1000);
                        uStack1040 = CONCAT22(puVar9, uVar7);
                        if(((uint)puVar9 | uVar7) == 0x0)
                        {
                            uVar8                              = *(undefined4 *)&iVar12->field_0x10;
                            *(undefined4 *)((int)uVar8 + 0x16) = 0x0;
                        }
                        else
                        {
                            pass1_1000_5586((uchar *)0x3e38, 0x1008, iVar2, 0x6, uVar7, (ushort)puVar9);
                            uVar8                              = *(undefined4 *)&iVar12->field_0x10;
                            *(undefined4 *)((int)uVar8 + 0x16) = uStack1040;
                        }
                        for(iStack1036 = 0x0; uVar8 = *(undefined4 *)&iVar12->field_0x10,
                        piVar1                      = (int *)((int)uVar8 + 0x1a),
                        *piVar1 != iStack1036 && iStack1036 <= *piVar1;
                            iStack1036 = iStack1036 + 0x1)
                        {
                            uVar8 = *(undefined4 *)&iVar12->field_0x10;
                            uVar3 = *(ulong *)((int)uVar8 + 0x16);
                            BVar5 = read_file_1008_7bc8(
                              param_2,
                              (ushort *)(uVar3 & 0xffff0000 | (ulong)(uint)((int)uVar3 + iStack1036 * 0x6)),
                              0x1008,
                              param_5);
                            if(BVar5 == 0x0)
                                goto LAB_1030_58a7;
                        }
                        BVar5 = read_file_1008_7bc8(param_2,
                                                    (ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar12->field_0x14),
                                                    0x1008,
                                                    param_5);
                        if((BVar5 != 0x0)
                           && (BVar5
                               = read_file_1008_7dee(uVar4, uVar11, &iVar12->field_0x1c, 0x0, uVar12, 0x2, 0x1008),
                               BVar5 != 0x0))
                        {
                            return;
                        }
                    }
                }
            }
        }
    LAB_1030_58a7:
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}


void __stdcall16far write_to_file_1030_32e4(ulong param_1, ulong param_2, ushort param_3)

{
    ushort     uVar1;
    int        iVar2;
    BOOL16     BVar3;
    ushort     uVar4;
    ushort     uVar5;
    undefined4 local_16[0x2];
    undefined2 local_c;
    undefined4 local_a[0x2];

    iVar2 = (int)param_1;
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar4 = (ushort)param_2;
    uVar5 = (ushort)(param_2 >> 0x10);
    BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x4, uVar1, (char *)0x16c, 0x1008);
    if(BVar3 != 0x0)
    {
        BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x174, uVar1, &DAT_0000_000c, 0x1008);
        if(BVar3 != 0x0)
        {
            BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x180, uVar1, &DAT_0000_000c, 0x1008);
            if(BVar3 != 0x0)
            {
                BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x18c, uVar1, (char *)0x18, 0x1008);
                if(BVar3 != 0x0)
                {
                    local_c = *(undefined2 *)(iVar2 + 0x1a8);
                    BVar3   = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)&local_c, param_3, (char *)0x2, 0x1008);
                    if(BVar3 != 0x0)
                    {
                        local_16[0] = *(undefined4 *)(iVar2 + 0x1aa);
                        BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)local_16, param_3, (char *)0x4, 0x1008);
                        if(BVar3 != 0x0)
                        {
                            local_a[0] = *(undefined4 *)(iVar2 + 0x170);
                            BVar3
                              = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)local_a, param_3, (char *)0x4, 0x1008);
                            if(BVar3 != 0x0)
                            {
                                local_c = *(undefined2 *)(iVar2 + 0x1ae);
                                BVar3   = write_to_file_1008_7e1c(
                                  uVar4, uVar5, (ushort)&local_c, param_3, (char *)0x2, 0x1008);
                                if(BVar3 != 0x0)
                                {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
}


void __stdcall16far read_file_1030_33f0(ulong param_1, ulong param_2)

{
    ushort       uVar1;
    astruct_430 *iVar2;
    BOOL16       BVar2;
    ushort       uVar3;
    ushort       uVar4;

    iVar2 = (astruct_430 *)param_1;
    iVar2 = (astruct_430 *)&iVar2->field_0x4;
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar3 = (ushort)param_2;
    uVar4 = (ushort)(param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(uVar3, uVar4, (ushort)iVar2, 0x0, uVar1, 0x16c, 0x1008);
    if(((((BVar2 != 0x0)
          && (BVar2 = read_file_1008_7dee(uVar3, uVar4, &iVar2->field_0x174, 0x0, uVar1, 0xc, 0x1008), BVar2 != 0x0))
         && (BVar2 = read_file_1008_7dee(uVar3, uVar4, &iVar2->field_0x180, 0x0, uVar1, 0xc, 0x1008), BVar2 != 0x0))
        && ((BVar2 = read_file_1008_7dee(uVar3, uVar4, &iVar2->field_0x18c, 0x0, uVar1, 0x18, 0x1008),
             BVar2 != 0x0
               && (BVar2 = read_file_1008_7dee(uVar3, uVar4, &iVar2->field_0x1a8, 0x0, uVar1, 0x2, 0x1008),
                   BVar2 != 0x0))))
       && (BVar2 = read_file_1008_7dee(uVar3, uVar4, &iVar2->field_0x1aa, 0x0, uVar1, 0x4, 0x1008), BVar2 != 0x0))
    {
        if((int)PTR_LOOP_1050_0312 < 0x2)
        {
            return;
        }
        BVar2 = read_file_1008_7dee(uVar3, uVar4, &iVar2->field_0x170, 0x0, uVar1, 0x4, 0x1008);
        if((BVar2 != 0x0)
           && (BVar2 = read_file_1008_7dee(uVar3, uVar4, &iVar2->field_0x1ae, 0x0, uVar1, 0x2, 0x1008), BVar2 != 0x0))
        {
            return;
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
}
