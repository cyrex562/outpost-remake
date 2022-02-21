
void __stdcall16far pass1_1030_227a(ulong param_1, ulong param_2, uint16_t param_3, ushort param_4)

{
    ushort uVar1;
    int    iVar2;
    BOOL16 BVar3;
    ushort uVar4;
    ushort uVar5;

    uVar1 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if(uVar1 != 0x0)
    {
        iVar2 = (int)param_1;
        uVar1 = (ushort)(param_1 >> 0x10);
        uVar4 = (ushort)param_2;
        uVar5 = (ushort)(param_2 >> 0x10);
        BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x10, uVar1, (char *)0x106, 0x1008);
        if(BVar3 != 0x0)
        {
            BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x116, uVar1, (char *)0x86, 0x1008);
            if(BVar3 != 0x0)
            {
                BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x19c, uVar1, (char *)0xa, 0x1008);
                if(BVar3 != 0x0)
                {
                    BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x1a6, uVar1, (char *)0x106, 0x1008);
                    if(BVar3 != 0x0)
                    {
                        BVar3 = write_to_file_1008_7e1c(uVar4, uVar5, iVar2 + 0x2ac, uVar1, (char *)0x106, 0x1008);
                        if(BVar3 != 0x0)
                        {
                            return;
                        }
                    }
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


void __stdcall16far pass1_1030_232e(ulong param_1, ulong param_2, int param_3, undefined2 param_4, undefined2 param_5)

{
    ushort uVar1;
    int    iVar2;
    BOOL16 BVar3;
    ushort uVar4;
    ushort uVar5;

    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if(param_3 != 0x0)
    {
        iVar2 = (int)param_1;
        uVar1 = (ushort)(param_1 >> 0x10);
        uVar4 = (ushort)param_2;
        uVar5 = (ushort)(param_2 >> 0x10);
        BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x10, 0x0, uVar1, 0x106, 0x1008);
        if(BVar3 != 0x0)
        {
            BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x116, 0x0, uVar1, 0x86, 0x1008);
            if(BVar3 != 0x0)
            {
                BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x19c, 0x0, uVar1, 0xa, 0x1008);
                if(BVar3 != 0x0)
                {
                    BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x1a6, 0x0, uVar1, 0x106, 0x1008);
                    if(BVar3 != 0x0)
                    {
                        BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x2ac, 0x0, uVar1, 0x106, 0x1008);
                        if(BVar3 != 0x0)
                        {
                            return;
                        }
                    }
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}


void __stdcall16far pass1_1030_2aca(ulong param_1, ulong param_2, uint16_t param_3, ushort param_4)

{
    undefined4   uVar1;
    undefined2  *puVar2;
    ushort       uVar3;
    BOOL16       BVar4;
    int          iVar5;
    astruct_730 *iVar6;
    undefined2   uVar6;
    undefined2   uVar7;
    ushort       uVar8;
    undefined4   local_18[0x3];
    undefined2   local_c[0x3];
    undefined2   local_6[0x2];

    uVar3 = pass1_1030_1978(param_1, param_2, param_3, param_4);
    if(uVar3 == 0x0)
    {
        return;
    }
    uVar6      = (undefined2)(param_1 >> 0x10);
    iVar6      = (astruct_730 *)param_1;
    local_c[0] = *iVar6->field_0x10;
    uVar3      = (ushort)param_2;
    uVar8      = (ushort)(param_2 >> 0x10);
    BVar4      = write_to_file_1008_7e1c(uVar3, uVar8, (ushort)local_c, param_4, (char *)0x2, 0x1008);
    if(((BVar4 != 0x0)
        && (puVar2 = iVar6->field_0x10,
            BVar4  = pass1_1008_7c2a(param_2, *(char **)((int)puVar2 + 0x2), 0x1008),
            BVar4 != 0x0))
       && (puVar2 = iVar6->field_0x10,
           iVar5
           = write_to_file_1008_7b4c(param_2, (ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0x6), 0x1008, param_4),
           iVar5 != 0x0))
    {
        puVar2     = iVar6->field_0x10;
        local_6[0] = *(undefined2 *)((int)puVar2 + 0xc);
        BVar4      = write_to_file_1008_7e1c(uVar3, uVar8, (ushort)local_6, param_4, (char *)0x2, 0x1008);
        if(BVar4 != 0x0)
        {
            puVar2      = iVar6->field_0x10;
            local_18[0] = *(undefined4 *)((int)puVar2 + 0xe);
            BVar4       = write_to_file_1008_7e1c(uVar3, uVar8, (ushort)local_18, param_4, (char *)0x4, 0x1008);
            if((BVar4 != 0x0)
               && (puVar2 = iVar6->field_0x10,
                   BVar4  = write_to_file_1008_7e1c(
                     uVar3, uVar8, (int)puVar2 + 0x12, (ushort)((ulong)puVar2 >> 0x10), (char *)0x10, 0x1008),
                   BVar4 != 0x0))
            {
                puVar2     = iVar6->field_0x10;
                local_c[0] = *(undefined2 *)((int)puVar2 + 0x22);
                BVar4      = write_to_file_1008_7e1c(uVar3, uVar8, (ushort)local_c, param_4, (char *)0x2, 0x1008);
                if((BVar4 != 0x0)
                   && ((puVar2 = iVar6->field_0x10,
                        *(int *)((int)puVar2 + 0x22) == 0x0
                          || (puVar2 = iVar6->field_0x10,
                              uVar7  = (undefined2)((ulong)puVar2 >> 0x10),
                              iVar5  = (int)puVar2,
                              uVar1  = *(undefined4 *)(iVar5 + 0x24),
                              BVar4  = write_to_file_1008_7e1c(uVar3,
                                                              uVar8,
                                                              (ushort)uVar1,
                                                              (ushort)((ulong)uVar1 >> 0x10),
                                                              (char *)(ulong)(uint)(*(int *)(iVar5 + 0x22) * 0x2),
                                                              0x1008),
                              BVar4 != 0x0))))
                {
                    local_c[0] = iVar6->field_0x14;
                    BVar4      = write_to_file_1008_7e1c(uVar3, uVar8, (ushort)local_c, param_4, (char *)0x2, 0x1008);
                    if(BVar4 != 0x0)
                    {
                        local_c[0] = iVar6->field_0x16;
                        BVar4 = write_to_file_1008_7e1c(uVar3, uVar8, (ushort)local_c, param_4, (char *)0x2, 0x1008);
                        if(BVar4 != 0x0)
                        {
                            local_c[0] = iVar6->field_0x18;
                            BVar4
                              = write_to_file_1008_7e1c(uVar3, uVar8, (ushort)local_c, param_4, (char *)0x2, 0x1008);
                            if(BVar4 != 0x0)
                            {
                                local_c[0] = iVar6->field_0x1a;
                                BVar4      = write_to_file_1008_7e1c(
                                  uVar3, uVar8, (ushort)local_c, param_4, (char *)0x2, 0x1008);
                                if(BVar4 != 0x0)
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


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_2c8a(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    undefined4   uVar1;
    ushort       uVar2;
    BOOL16       BVar3;
    undefined   *puVar4;
    ushort       uVar5;
    uchar       *puVar6;
    astruct_374 *iVar7;
    astruct_371 *iVar8;
    astruct_372 *iVar9;
    int          unaff_DI;
    undefined2   uVar7;
    ushort      *puVar8;
    ushort       uVar9;
    ushort       uVar10;
    ushort      *puStack1038;
    undefined2   local_406;
    undefined2   local_404;
    undefined    local_402[0x400];
    astruct_373 *iVar14;

    iVar14 = (astruct_373 *)param_1;
    uVar10 = (ushort)(param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if(param_3 == 0x0)
    {
        return;
    }
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4, 0x1000);
        PTR_LOOP_1050_5f2e = param_4;
    }
    else
    {
    }
    uVar2       = fn_ptr_op_1000_1708(0x28, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    puStack1038 = (ushort *)CONCAT22(PTR_LOOP_1050_5f2e, uVar2);
    puVar6      = (uchar *)((uint)PTR_LOOP_1050_5f2e | uVar2);
    if(puVar6 == (uchar *)0x0)
    {
        iVar14->field_0x10 = (ushort *)0x0;
    }
    else
    {
        puVar8             = pass1_1008_3e38((ushort *)CONCAT22(PTR_LOOP_1050_5f2e, uVar2 + 0x6));
        puVar6             = (uchar *)((ulong)puVar8 >> 0x10);
        iVar14->field_0x10 = puStack1038;
    }
    puVar8 = iVar14->field_0x10;
    uVar2  = (ushort)param_2;
    uVar9  = (ushort)(param_2 >> 0x10);
    BVar3  = read_file_1008_7dee(uVar2, uVar9, (ushort)puVar8, 0x0, (ushort)((ulong)puVar8 >> 0x10), 0x2, 0x1008);
    if(BVar3 != 0x0)
    {
        puVar4 = local_402;
        read_file_1008_7c6e(uVar2, uVar9, (char *)CONCAT22(param_5, puVar4), 0x1008);
        if(puVar4 != (undefined *)0x0)
        {
            uVar5            = str_op_1008_60e8((char *)CONCAT22(param_5, local_402), (ushort)puVar6);
            puVar8           = iVar14->field_0x10;
            uVar7            = (undefined2)((ulong)puVar8 >> 0x10);
            iVar7            = (astruct_374 *)puVar8;
            iVar7->field_0x2 = uVar5;
            iVar7->field_0x4 = puVar6;
            puVar8           = iVar14->field_0x10;
            BVar3            = read_file_1008_7bc8(
              param_2, (ushort *)((ulong)puVar8 & 0xffff0000 | (ulong)((int)puVar8 + 0x6)), 0x1008, param_5);
            if((((BVar3 != 0x0)
                 && (puVar8 = iVar14->field_0x10,
                     BVar3  = read_file_1008_7dee(
                       uVar2, uVar9, (int)puVar8 + 0xc, 0x0, (ushort)((ulong)puVar8 >> 0x10), 0x2, 0x1008),
                     BVar3 != 0x0))
                && (puVar8 = iVar14->field_0x10,
                    BVar3  = read_file_1008_7dee(
                      uVar2, uVar9, (int)puVar8 + 0xe, 0x0, (ushort)((ulong)puVar8 >> 0x10), 0x4, 0x1008),
                    BVar3 != 0x0))
               && ((puVar8 = iVar14->field_0x10,
                    BVar3  = read_file_1008_7dee(
                      uVar2, uVar9, (int)puVar8 + 0x12, 0x0, (ushort)((ulong)puVar8 >> 0x10), 0x10, 0x1008),
                    BVar3 != 0x0
                      && (puVar8 = iVar14->field_0x10,
                          BVar3  = read_file_1008_7dee(
                            uVar2, uVar9, (int)puVar8 + 0x22, 0x0, (ushort)((ulong)puVar8 >> 0x10), 0x2, 0x1008),
                          BVar3 != 0x0))))
            {
                puVar8 = iVar14->field_0x10;
                if(*(int *)((int)puVar8 + 0x22) != 0x0)
                {
                    puVar8   = iVar14->field_0x10;
                    unaff_DI = (int)((ulong)puVar8 >> 0x10);
                    iVar8    = (astruct_371 *)puVar8;
                    uVar5    = iVar8->field_0x22 * 0x2;
                    mem_op_1000_179c(uVar5, puVar6, 0x1000);
                    iVar8->field_0x24 = uVar5;
                    iVar8->field_0x26 = puVar6;
                    puVar8            = iVar14->field_0x10;
                    uVar7             = (undefined2)((ulong)puVar8 >> 0x10);
                    iVar9             = (astruct_372 *)puVar8;
                    uVar1             = iVar9->field_0x24;
                    BVar3             = read_file_1008_7dee(uVar2,
                                                uVar9,
                                                (ushort)uVar1,
                                                0x0,
                                                (ushort)((ulong)uVar1 >> 0x10),
                                                iVar9->field_0x22 * 0x2,
                                                0x1008);
                    if(BVar3 == 0x0)
                    {
                        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                        return;
                    }
                }
                BVar3 = read_file_1008_7dee(uVar2, uVar9, &iVar14->field_0x14, 0x0, uVar10, 0x2, 0x1008);
                if(((BVar3 != 0x0)
                    && (BVar3 = read_file_1008_7dee(uVar2, uVar9, (ushort)&local_404, 0x0, param_5, 0x2, 0x1008),
                        BVar3 != 0x0))
                   && ((BVar3 = read_file_1008_7dee(uVar2, uVar9, &iVar14->field_0x18, 0x0, uVar10, 0x2, 0x1008),
                        BVar3 != 0x0
                          && (BVar3 = read_file_1008_7dee(uVar2, uVar9, (ushort)&local_406, 0x0, param_5, 0x2, 0x1008),
                              BVar3 != 0x0))))
                {
                    iVar14->field_0x16 = local_404;
                    iVar14->field_0x1a = local_406;
                    puVar8             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_5, puVar6, unaff_DI);
                    pass1_1018_04a4((ulong)puVar8, iVar14->field_0x4);
                    pass1_1010_82f8(_PTR_LOOP_1050_14cc, *iVar14->field_0x10);
                    return;
                }
            }
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
}


void __stdcall16far pass1_1030_16d6(ulong param_1, ulong param_2, ushort param_3)

{
    BOOL16     BVar1;
    undefined2 uVar2;
    ushort     uVar3;
    undefined4 local_10[0x2];
    undefined4 local_8;

    uVar2       = (undefined2)(param_1 >> 0x10);
    local_10[0] = *(undefined4 *)((int)param_1 + 0x4);
    uVar3       = (ushort)(param_2 >> 0x10);
    BVar1       = write_to_file_1008_7e1c((ushort)param_2, uVar3, (ushort)local_10, param_3, (char *)0x4, 0x1008);
    if(BVar1 != 0x0)
    {
        local_8 = *(undefined4 *)((int)param_1 + 0x8);
        BVar1   = write_to_file_1008_7e1c((ushort)param_2, uVar3, (ushort)&local_8, param_3, (char *)0x4, 0x1008);
        if(BVar1 != 0x0)
        {
            return;
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
}


void __stdcall16far file_1030_1730(ulong param_1, ulong param_2)

{
    ushort uVar1;
    BOOL16 BVar2;
    ushort uVar3;

    uVar1 = (ushort)(param_1 >> 0x10);
    uVar3 = (ushort)(param_2 >> 0x10);
    BVar2 = read_file_1008_7dee((ushort)param_2, uVar3, (int)param_1 + 0x4, 0x0, uVar1, 0x4, 0x1008);
    if(BVar2 != 0x0)
    {
        BVar2 = read_file_1008_7dee((ushort)param_2, uVar3, (int)param_1 + 0x8, 0x0, uVar1, 0x4, 0x1008);
        if(BVar2 != 0x0)
        {
            return;
        }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
}


ushort __stdcall16far pass1_1030_1978(ulong param_1, ulong param_2, uint16_t param_3, ushort param_4)

{
    pass1_1030_16d6(param_1, param_2, param_4);
    if(param_3 != 0x0)
    {
        write_to_file_1008_7954(param_2, (undefined4 *)*(undefined4 *)((int)param_1 + 0xc), param_3, 0x1008, param_4);
        if(param_3 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return param_3;
        }
        param_3 = 0x1;
    }
    return param_3;
}


void __stdcall16far file_1030_19b4(ulong param_1, ulong param_2, int param_3, ushort param_4, ushort param_5)

{
    long *plVar1;

    file_1030_1730(param_1, param_2);
    if(param_3 != 0x0)
    {
        plVar1 = (long *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0xc));
        file_1008_76e4(param_2, plVar1, 0x1008, param_5, param_4);
        if((int)plVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
    }
    return;
}


undefined2 __stdcall16far pass1_1030_1a9c(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4 uVar1;
    int       *piVar2;
    uint16_t   in_AX;
    ushort     uVar3;
    BOOL16     BVar4;
    int        iVar5;
    undefined2 uVar6;
    undefined2 local_c[0x5];

    uVar3 = pass1_1030_1978(param_1, param_2, in_AX, param_3);
    if(uVar3 != 0x0)
    {
        uVar6      = (undefined2)(param_1 >> 0x10);
        iVar5      = (int)param_1;
        local_c[0] = *(undefined2 *)*(undefined4 *)(iVar5 + 0x10);
        uVar3      = (ushort)(param_2 >> 0x10);
        BVar4      = write_to_file_1008_7e1c((ushort)param_2, uVar3, (ushort)local_c, param_3, (char *)0x2, 0x1008);
        if(BVar4 != 0x0)
        {
            if(**(int **)(iVar5 + 0x10) == 0x0)
            {
                return 0x1;
            }
            piVar2 = *(int **)(iVar5 + 0x10);
            uVar1  = *(undefined4 *)((int)piVar2 + 0x2);
            BVar4  = write_to_file_1008_7e1c((ushort)param_2,
                                            uVar3,
                                            (ushort)uVar1,
                                            (ushort)((ulong)uVar1 >> 0x10),
                                            (char *)(ulong)(uint)(*piVar2 * 0x2),
                                            0x1008);
            if(BVar4 != 0x0)
            {
                return 0x1;
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far
file_1030_1b18(ulong param_1, ulong param_2, int param_3, undefined *param_4, undefined2 param_5)

{
    undefined4   uVar1;
    int         *piVar2;
    ushort       uVar3;
    ushort       uVar4;
    BOOL16       BVar5;
    ushort       uVar6;
    uchar       *puVar7;
    astruct_368 *iVar7;
    undefined2   uVar8;
    undefined2   uVar9;
    astruct_370 *iVar10;
    astruct_369 *iVar9;

    iVar10 = (astruct_370 *)param_1;
    uVar9  = (undefined2)(param_1 >> 0x10);
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
        uVar4 = fn_ptr_op_1000_1708(0x6, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
        *(ushort *)&iVar10->field_0x10                  = uVar4;
        *(undefined2 *)((int)&iVar10->field_0x10 + 0x2) = PTR_LOOP_1050_5f2e;
        puVar7                                          = *(uchar **)((int)&iVar10->field_0x10 + 0x2);
        uVar4                                           = (ushort)(param_2 >> 0x10);
        BVar5                                           = read_file_1008_7dee(
          (ushort)param_2, uVar4, *(ushort *)&iVar10->field_0x10, 0x0, (ushort)puVar7, 0x2, 0x1008);
        if(BVar5 != 0x0)
        {
            piVar2 = iVar10->field_0x10;
            if(*piVar2 == 0x0)
            {
                return 0x1;
            }
            uVar3 = *piVar2 * 0x2;
            uVar6 = uVar3;
            mem_op_1000_179c(uVar3, puVar7, 0x1000);
            piVar2           = iVar10->field_0x10;
            uVar8            = (undefined2)((ulong)piVar2 >> 0x10);
            iVar7            = (astruct_368 *)piVar2;
            iVar7->field_0x2 = uVar6;
            iVar7->field_0x4 = puVar7;
            piVar2           = iVar10->field_0x10;
            uVar1            = *(undefined4 *)((int)piVar2 + 0x2);
            BVar5            = read_file_1008_7dee(
              (ushort)param_2, uVar4, (ushort)uVar1, 0x0, (ushort)((ulong)uVar1 >> 0x10), uVar3, 0x1008);
            if(BVar5 != 0x0)
            {
                return 0x1;
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return 0x0;
}


uint16_t __stdcall16far write_file_fn_1028_e56c(undefined2 param_1,
                                                undefined2 param_2,
                                                undefined4 param_3,
                                                uint16_t   param_4)

{
    code      **ppcVar1;
    undefined  *puVar2;
    BOOL16      BVar3;
    uint16_t    in_DX;
    uint16_t    extraout_DX;
    undefined4  in_stack_0000000c;
    ulong       local_2a[0x3];
    undefined4 *puStack28;
    ulong       uStack24;
    undefined   local_14[0x8];
    undefined2  uStack12;
    uint16_t    uStack10;
    undefined2  uStack8;
    uint16_t    uStack6;
    int         iStack4;

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_14),
                    0x1,
                    (ushort)in_stack_0000000c,
                    (uint)((ulong)in_stack_0000000c >> 0x10));
    uStack24 = 0x0;
    while(true)
    {
        puVar2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar2));
        puStack28 = (undefined4 *)CONCAT22(in_DX, puVar2);
        in_DX     = in_DX | (uint)puVar2;
        if(in_DX == 0x0)
            break;
        uStack24 = uStack24 + 0x1;
    }
    local_2a[0] = uStack24;
    BVar3       = write_to_file_1008_7e1c(
      (ushort)param_3, (ushort)((ulong)param_3 >> 0x10), (ushort)local_2a, param_4, (char *)0x4, 0x1008);
    if(BVar3 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    else
    {
        uStack12 = uStack8;
        uStack10 = uStack6;
        if(iStack4 != 0x0)
        {
            uStack12 = 0x1;
            uStack6  = 0x0;
            uStack10 = uStack6;
        }
        do
        {
            puVar2 = local_14;
            pass1_1028_e4ec(CONCAT22(param_4, puVar2));
            puStack28 = (undefined4 *)CONCAT22(uStack6, puVar2);
            if((uStack6 | (uint)puVar2) == 0x0)
            {
                return 0x0;
            }
            ppcVar1 = (code **)((int)*puStack28 + 0xc);
            (**ppcVar1)(0x1008, puVar2, uStack6);
            local_2a[0] = local_2a[0] & 0xffff0000 | ZEXT24(puVar2);
            uStack6     = extraout_DX;
            in_DX       = extraout_DX;
        } while(puVar2 != (undefined *)0x0);
    }
    return in_DX;
}


BOOL16 __stdcall16far pass1_1028_d7a0(ushort param_1, ushort param_2, ulong param_3, uint16_t param_4)

{
    BOOL16 BVar1;

    BVar1 = write_to_file_1008_7cac(param_3, param_4);
    if(BVar1 != 0x0)
    {
        BVar1 = 0x1;
    }
    return BVar1;
}


int __stdcall16far
read_file_1028_d7ba(undefined2 param_1, undefined2 param_2, undefined4 param_3, uint16_t param_4, uint16_t param_5)

{
    read_file_1008_7cfe((int)param_3, (int)((ulong)param_3 >> 0x10), 0x8, 0x1008, param_4);
    if(param_5 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d4;
        return param_5;
    }
    return 0x1;
}


ulong __stdcall16far write_to_file_1028_dce2(undefined4 *param_1, undefined4 param_2, uint16_t param_3)

{
    code     **ppcVar1;
    BOOL16     BVar2;
    undefined *puVar3;
    uint16_t   in_DX;
    uint       extraout_DX;
    uint       uVar4;
    int        iVar5;
    undefined2 uVar6;
    ushort     uVar7;
    undefined4 local_26[0x2];
    undefined2 local_1e[0x3];
    undefined4 uStack24;
    undefined  local_14[0x12];

    uVar7 = (ushort)((ulong)param_2 >> 0x10);
    BVar2 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar2 != 0x0)
    {
        local_26[0] = *param_1;
        BVar2       = write_to_file_1008_7e1c((ushort)param_2, uVar7, (ushort)local_26, param_3, (char *)0x4, 0x1008);
        if(BVar2 != 0x0)
        {
            uVar6       = (undefined2)((ulong)param_1 >> 0x10);
            iVar5       = (int)param_1;
            local_1e[0] = *(undefined2 *)(iVar5 + 0x8);
            BVar2 = write_to_file_1008_7e1c((ushort)param_2, uVar7, (ushort)local_1e, param_3, (char *)0x2, 0x1008);
            if(BVar2 != 0x0)
            {
                ppcVar1 = (code **)((int)*_PTR_LOOP_1050_5166 + 0xc);
                (**ppcVar1)(0x1008, (int)_PTR_LOOP_1050_5166, (int)((ulong)_PTR_LOOP_1050_5166 >> 0x10), param_2);
                in_DX = extraout_DX;
                if(BVar2 != 0x0)
                {
                    BVar2 = write_to_file_1008_7cac(param_2, param_3);
                    if(BVar2 != 0x0)
                    {
                        in_DX = write_file_fn_1028_e56c(iVar5, uVar6, param_2, param_3);
                        if(BVar2 != 0x0)
                        {
                            BVar2 = write_to_file_1008_7cac(param_2, param_3);
                            if(BVar2 != 0x0)
                            {
                                in_DX = write_file_fn_1028_e56c(iVar5, uVar6, param_2, param_3);
                                if(BVar2 != 0x0)
                                {
                                    BVar2 = write_to_file_1008_7cac(param_2, param_3);
                                    if(BVar2 != 0x0)
                                    {
                                        in_DX = write_file_fn_1028_e56c(iVar5, uVar6, param_2, param_3);
                                        if(BVar2 != 0x0)
                                        {
                                            BVar2 = write_to_file_1008_7cac(param_2, param_3);
                                            if(BVar2 != 0x0)
                                            {
                                                in_DX = write_file_fn_1028_e56c(iVar5, uVar6, param_2, param_3);
                                                if(BVar2 != 0x0)
                                                {
                                                    BVar2 = write_to_file_1008_7cac(param_2, param_3);
                                                    if(BVar2 != 0x0)
                                                    {
                                                        in_DX = write_file_fn_1028_e56c(iVar5, uVar6, param_2, param_3);
                                                        if(BVar2 != 0x0)
                                                        {
                                                            BVar2 = write_to_file_1008_7cac(param_2, param_3);
                                                            if(BVar2 != 0x0)
                                                            {
                                                                in_DX = write_file_fn_1028_e56c(
                                                                  iVar5, uVar6, param_2, param_3);
                                                                if(BVar2 != 0x0)
                                                                {
                                                                    BVar2 = write_to_file_1008_7cac(param_2, param_3);
                                                                    if(BVar2 != 0x0)
                                                                    {
                                                                        in_DX = write_file_fn_1028_e56c(
                                                                          iVar5, uVar6, param_2, param_3);
                                                                        if(BVar2 != 0x0)
                                                                        {
                                                                            BVar2 = write_to_file_1008_7cac(param_2,
                                                                                                            param_3);
                                                                            if(BVar2 != 0x0)
                                                                            {
                                                                                in_DX = write_file_fn_1028_e56c(
                                                                                  iVar5, uVar6, param_2, param_3);
                                                                                if(BVar2 != 0x0)
                                                                                {
                                                                                    pass1_1028_dc52(
                                                                                      (astruct_92 *)CONCAT22(param_3,
                                                                                                             local_14),
                                                                                      0x1,
                                                                                      0x0,
                                                                                      0x400);
                                                                                    while(true)
                                                                                    {
                                                                                        uVar4  = in_DX;
                                                                                        puVar3 = local_14;
                                                                                        pass1_1028_e4ec(
                                                                                          CONCAT22(param_3, puVar3));
                                                                                        uStack24
                                                                                          = CONCAT22(uVar4, puVar3);
                                                                                        in_DX = uVar4 | (uint)puVar3;
                                                                                        if(in_DX == 0x0)
                                                                                            break;
                                                                                        if(*(long *)(puVar3 + 0x200)
                                                                                           != 0x8000002)
                                                                                        {
                                                                                            pass1_1038_3ba0(
                                                                                              CONCAT22(uVar4, puVar3));
                                                                                        }
                                                                                    }
                                                                                    return 0x10000;
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
    return (ulong)in_DX;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far read_file_1028_def2(ulong param_1, undefined4 param_2, uint16_t param_3, uint16_t param_4)

{
    code **ppcVar1;
    BOOL16 BVar2;
    ushort unaff_SI;
    ushort unaff_DI;
    ushort uVar3;
    uchar  in_AF;
    ushort uVar4;
    ushort uVar5;

    uVar4 = (ushort)param_2;
    uVar5 = (ushort)((ulong)param_2 >> 0x10);
    read_file_1008_7cfe(uVar4, uVar5, 0xa, 0x1008, param_3);
    if(param_4 != 0x0)
    {
        uVar3 = (ushort)(param_1 >> 0x10);
        BVar2 = read_file_1008_7dee(uVar4, uVar5, (ushort)param_1, 0x0, uVar3, 0x4, 0x1008);
        if(BVar2 != 0x0)
        {
            BVar2 = read_file_1008_7dee(uVar4, uVar5, (ushort)param_1 + 0x8, 0x0, uVar3, 0x2, 0x1008);
            if(BVar2 != 0x0)
            {
                uVar3   = (ushort)((ulong)*_PTR_LOOP_1050_5166 >> 0x10);
                ppcVar1 = (code **)((int)*_PTR_LOOP_1050_5166 + 0x10);
                (**ppcVar1)(0x1008, (int)_PTR_LOOP_1050_5166, (int)((ulong)_PTR_LOOP_1050_5166 >> 0x10), param_2);
                if(BVar2 != 0x0)
                {
                    read_file_1008_7cfe(uVar4, uVar5, 0xc, 0x1008, param_3);
                    if(BVar2 != 0x0)
                    {
                        pass1_1028_e628(param_1, uVar4, uVar5, 0x0, 0x100, unaff_SI, unaff_DI, uVar3, param_3, in_AF);
                        if(BVar2 != 0x0)
                        {
                            read_file_1008_7cfe(uVar4, uVar5, 0xd, 0x1008, param_3);
                            if(BVar2 != 0x0)
                            {
                                pass1_1028_e628(
                                  param_1, uVar4, uVar5, 0x0, 0x200, unaff_SI, unaff_DI, uVar3, param_3, in_AF);
                                if(BVar2 != 0x0)
                                {
                                    read_file_1008_7cfe(uVar4, uVar5, 0xe, 0x1008, param_3);
                                    if(BVar2 != 0x0)
                                    {
                                        pass1_1028_e628(
                                          param_1, uVar4, uVar5, 0x0, 0x300, unaff_SI, unaff_DI, uVar3, param_3, in_AF);
                                        if(BVar2 != 0x0)
                                        {
                                            read_file_1008_7cfe(uVar4, uVar5, 0xf, 0x1008, param_3);
                                            if(BVar2 != 0x0)
                                            {
                                                pass1_1028_e628(param_1,
                                                                uVar4,
                                                                uVar5,
                                                                0x0,
                                                                0x400,
                                                                unaff_SI,
                                                                unaff_DI,
                                                                uVar3,
                                                                param_3,
                                                                in_AF);
                                                if(BVar2 != 0x0)
                                                {
                                                    read_file_1008_7cfe(uVar4, uVar5, 0x10, 0x1008, param_3);
                                                    if(BVar2 != 0x0)
                                                    {
                                                        pass1_1028_e628(param_1,
                                                                        uVar4,
                                                                        uVar5,
                                                                        0x0,
                                                                        0x500,
                                                                        unaff_SI,
                                                                        unaff_DI,
                                                                        uVar3,
                                                                        param_3,
                                                                        in_AF);
                                                        if(BVar2 != 0x0)
                                                        {
                                                            read_file_1008_7cfe(uVar4, uVar5, 0x11, 0x1008, param_3);
                                                            if(BVar2 != 0x0)
                                                            {
                                                                pass1_1028_e628(param_1,
                                                                                uVar4,
                                                                                uVar5,
                                                                                0x0,
                                                                                0x600,
                                                                                unaff_SI,
                                                                                unaff_DI,
                                                                                uVar3,
                                                                                param_3,
                                                                                in_AF);
                                                                if(BVar2 != 0x0)
                                                                {
                                                                    read_file_1008_7cfe(
                                                                      uVar4, uVar5, 0x12, 0x1008, param_3);
                                                                    if(BVar2 != 0x0)
                                                                    {
                                                                        pass1_1028_e628(param_1,
                                                                                        uVar4,
                                                                                        uVar5,
                                                                                        0x0,
                                                                                        0x700,
                                                                                        unaff_SI,
                                                                                        unaff_DI,
                                                                                        uVar3,
                                                                                        param_3,
                                                                                        in_AF);
                                                                        if(BVar2 != 0x0)
                                                                        {
                                                                            read_file_1008_7cfe(
                                                                              uVar4, uVar5, 0x13, 0x1008, param_3);
                                                                            if(BVar2 != 0x0)
                                                                            {
                                                                                pass1_1028_e628(param_1,
                                                                                                uVar4,
                                                                                                uVar5,
                                                                                                0x0,
                                                                                                0x800,
                                                                                                unaff_SI,
                                                                                                unaff_DI,
                                                                                                uVar3,
                                                                                                param_3,
                                                                                                in_AF);
                                                                                if(BVar2 != 0x0)
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
    return;
}


BOOL16 __stdcall16far write_to_file_1028_b5ec(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4 uVar1;
    BOOL16     BVar2;
    int        iVar3;
    undefined2 uVar4;
    ushort     uVar5;
    ushort     uVar6;
    undefined2 local_e[0x3];
    undefined2 local_8[0x2];
    int        iStack4;

    uVar4      = (undefined2)(param_1 >> 0x10);
    iVar3      = (int)param_1;
    local_e[0] = *(undefined2 *)(iVar3 + 0xc);
    uVar5      = (ushort)param_2;
    uVar6      = (ushort)(param_2 >> 0x10);
    BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_e, param_3, (char *)0x2, 0x1008);
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    pass1_1030_16d6(param_1, param_2, param_3);
    if(BVar2 == 0x0)
    {
        return 0x0;
    }
    local_8[0] = *(undefined2 *)(iVar3 + 0xc);
    BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    local_8[0] = *(undefined2 *)(iVar3 + 0xe);
    BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    local_8[0] = *(undefined2 *)(iVar3 + 0x10);
    BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    local_8[0] = *(undefined2 *)(iVar3 + 0x12);
    BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    local_8[0] = *(undefined2 *)(iVar3 + 0x18);
    BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    local_8[0] = *(undefined2 *)(iVar3 + 0x1a);
    BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    iStack4 = *(int *)(iVar3 + 0x12);
    if(iStack4 == 0x6)
    {
        iStack4 = *(int *)(iVar3 + 0x18);
    }
    if(iStack4 < 0x1)
    {
        return 0x1;
    }
    if(SBORROW2(iStack4, 0x1))
    {
        return 0x1;
    }
    if(iStack4 == 0x3 || iStack4 + -0x2 < 0x1)
    {
        local_8[0] = *(undefined2 *)(iVar3 + 0x14);
    }
    else
    {
        if(iStack4 == 0x4)
        {
            if(*(long *)(iVar3 + 0x14) == 0x0)
            {
                local_8[0] = 0x0;
                BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
                goto joined_r0x1028b766;
            }
            uVar1      = *(undefined4 *)(iVar3 + 0x14);
            local_8[0] = *(undefined2 *)((int)uVar1 + 0x94);
        }
        else
        {
            if(iStack4 != 0x5)
            {
                return 0x1;
            }
            uVar1      = *(undefined4 *)(iVar3 + 0x14);
            local_8[0] = *(undefined2 *)((int)uVar1 + 0xa4);
            BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                return 0x0;
            }
            uVar1      = *(undefined4 *)(iVar3 + 0x14);
            local_8[0] = *(undefined2 *)((int)uVar1 + 0xa6);
            BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                return 0x0;
            }
            uVar1      = *(undefined4 *)(iVar3 + 0x14);
            local_8[0] = *(undefined2 *)((int)uVar1 + 0xa8);
            BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                return 0x0;
            }
            uVar1      = *(undefined4 *)(iVar3 + 0x14);
            local_8[0] = *(undefined2 *)((int)uVar1 + 0xaa);
            BVar2      = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                return 0x0;
            }
            uVar1      = *(undefined4 *)(iVar3 + 0x14);
            local_8[0] = *(undefined2 *)((int)uVar1 + 0xac);
        }
    }
    BVar2 = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)local_8, param_3, (char *)0x2, 0x1008);
joined_r0x1028b766:
    if(BVar2 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return 0x0;
    }
    return 0x1;
}


// WARNING: Unable to use type for symbol puVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1028_b81a(ulong param_1, ulong param_2, int param_3, ushort param_4, uchar *param_5)

{
    BOOL16    BVar1;
    int       iVar2;
    ulong    *puVar4;
    ushort    uVar5;
    uint      uVar6;
    ulong     uVar7;
    ushort    uVar8;
    ushort    local_2a[0x2];
    undefined local_26[0x16];
    ulong    *puStack16;
    uchar    *puStack14;
    int       iStack10;
    int       local_8;
    int       local_6;
    int       local_4;
    ulong    *puVar3;

    puVar3 = (ulong *)param_1;
    uVar6  = (uint)(param_1 >> 0x10);
    file_1030_1730(param_1, param_2);
    if(param_3 == 0x0)
    {
        return;
    }
    uVar5 = (ushort)param_2;
    uVar8 = (ushort)(param_2 >> 0x10);
    BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)&local_4, 0x0, param_4, 0x2, 0x1008);
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)&local_6, 0x0, param_4, 0x2, 0x1008);
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)&local_8, 0x0, param_4, 0x2, 0x1008);
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    iVar2                       = switch_1008_73ea(uVar5, uVar8, local_4);
    *(int *)(puVar3 + 0x3)      = iVar2;
    iVar2                       = switch_1008_73ea(uVar5, uVar8, local_6);
    *(int *)((int)puVar3 + 0xe) = iVar2;
    iVar2                       = switch_1008_73ea(uVar5, uVar8, local_8);
    *(int *)(puVar3 + 0x4)      = iVar2;
    BVar1                       = read_file_1008_7dee(uVar5, uVar8, (ushort)&local_4, 0x0, param_4, 0x2, 0x1008);
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)&local_6, 0x0, param_4, 0x2, 0x1008);
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    BVar1 = read_file_1008_7dee(uVar5, uVar8, (int)puVar3 + 0x1a, 0x0, uVar6, 0x2, 0x1008);
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    *(int *)((int)puVar3 + 0x12) = local_4;
    *(int *)(puVar3 + 0x6)       = local_6;
    iStack10                     = *(int *)((int)puVar3 + 0x12);
    if(iStack10 == 0x6)
    {
        iStack10 = *(int *)(puVar3 + 0x6);
    }
    switch(iStack10)
    {
    case 0x1:
    case 0x2:
    case 0x3:
        puVar4 = puVar3 + 0x5;
    LAB_1028_b968:
        BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)puVar4, 0x0, uVar6, 0x2, 0x1008);
        break;
    case 0x4:
        uVar7                  = pass1_1028_e0bc(_PTR_LOOP_1050_65e2, *(int *)(puVar3 + 0x3), puVar3, param_5, param_4);
        puStack14              = (uchar *)(uint)(uVar7 >> 0x10);
        *(int *)(puVar3 + 0x5) = (int)uVar7;
        *(uint *)((int)puVar3 + 0x16) = (uint)puStack14;
        if(((uint)puStack14 | *(uint *)(puVar3 + 0x5)) != 0x0)
        {
            puVar4    = (ulong *)(*(int *)(puVar3 + 0x5) + 0x94);
            uVar6     = (uint)puStack14;
            puStack16 = puVar4;
            goto LAB_1028_b968;
        }
        BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)local_26, 0x0, param_4, 0x2, 0x1008);
        break;
    case 0x5:
        puVar4 = puVar3;
        pass1_1028_e100(_PTR_LOOP_1050_65e2, *(ushort *)(puVar3 + 0x3), param_5);
        *(ulong **)(puVar3 + 0x5)       = puVar4;
        *(uchar **)((int)puVar3 + 0x16) = param_5;
        puStack16                       = (ulong *)(*(int *)(puVar3 + 0x5) + 0xa4);
        puStack14                       = param_5;
        BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)puStack16, 0x0, (ushort)param_5, 0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
        BVar1 = read_file_1008_7dee(uVar5, uVar8, (ushort)local_2a, 0x0, param_4, 0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
        uVar7 = puVar3[0x5];
        BVar1 = read_file_1008_7dee(uVar5, uVar8, (int)uVar7 + 0xa8, 0x0, (ushort)(uVar7 >> 0x10), 0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
        uVar7 = puVar3[0x5];
        BVar1 = read_file_1008_7dee(uVar5, uVar8, (int)uVar7 + 0xaa, 0x0, (ushort)(uVar7 >> 0x10), 0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
        uVar7 = puVar3[0x5];
        BVar1 = read_file_1008_7dee(uVar5, uVar8, (int)uVar7 + 0xac, 0x0, (ushort)(uVar7 >> 0x10), 0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
        uVar5                          = switch_1008_72bc(uVar5, uVar8, local_2a[0]);
        uVar7                          = puVar3[0x5];
        *(ushort *)((int)uVar7 + 0xa6) = uVar5;
        return;
    default:
        goto switchD_1028_ba97_caseD_6;
    case 0x9:
        puVar4 = puVar3;
        pass1_1028_e100(_PTR_LOOP_1050_65e2, *(ushort *)(puVar3 + 0x3), param_5);
        *(ulong **)(puVar3 + 0x5)       = puVar4;
        *(uchar **)((int)puVar3 + 0x16) = param_5;
        goto switchD_1028_ba97_caseD_6;
    }
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
switchD_1028_ba97_caseD_6:
    return;
}


BOOL16 __stdcall16far pass1_1028_b2c8(ulong param_1, ulong param_2, BOOL16 param_3, ushort param_4)

{
    BOOL16 BVar1;
    ushort uVar2;
    ushort local_4;

    file_1030_1730(param_1, param_2);
    if(param_3 != 0x0)
    {
        uVar2 = (ushort)(param_2 >> 0x10);
        BVar1 = read_file_1008_7dee((ushort)param_2, uVar2, (ushort)&local_4, 0x0, param_4, 0x2, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return BVar1;
        }
        uVar2                           = switch_1008_72bc((ushort)param_2, uVar2, local_4);
        *(ushort *)((int)param_1 + 0xc) = uVar2;
        param_3                         = 0x1;
    }
    return param_3;
}


undefined2 __stdcall16far pass1_1028_64d6(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4  uVar1;
    BOOL16      BVar2;
    undefined2 *puVar3;
    undefined2  uVar4;
    ushort      uVar5;
    ushort      uVar6;
    undefined2  local_26;
    undefined2  local_24;
    undefined2  local_22;
    undefined2  local_20;
    undefined2  local_1e;
    undefined2  local_1c[0x6];
    undefined2  uStack16;
    long        lStack14;
    undefined   local_a[0x8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if(BVar2 != 0x0)
    {
        uVar4 = (undefined2)(param_1 >> 0x10);
        pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)((int)param_1 + 0x20));
        uVar1       = *(undefined4 *)((int)param_1 + 0x20);
        local_1c[0] = *(undefined2 *)((int)uVar1 + 0x8);
        puVar3      = local_1c;
        uStack16    = local_1c[0];
        while(true)
        {
            uVar5 = (ushort)param_2;
            uVar6 = (ushort)(param_2 >> 0x10);
            BVar2 = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)puVar3, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
                break;
            lStack14 = pass1_1008_5b12(local_a, param_3);
            if(lStack14 == 0x0)
            {
                return 0x1;
            }
            local_1e = *(undefined2 *)((int)lStack14 + 0x4);
            BVar2    = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_1e, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
                break;
            local_20 = *(undefined2 *)((int)lStack14 + 0x6);
            BVar2    = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_20, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
                break;
            local_22 = *(undefined2 *)((int)lStack14 + 0x8);
            BVar2    = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_22, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
                break;
            local_24 = *(undefined2 *)((int)lStack14 + 0xa);
            BVar2    = write_to_file_1008_7e1c(uVar5, uVar6, (ushort)&local_24, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
                break;
            local_26 = *(undefined2 *)((int)lStack14 + 0xc);
            puVar3   = &local_26;
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_65e2(ulong param_1, ulong param_2, int param_3, uchar *param_4, ushort param_5)

{
    code      **ppcVar1;
    uint        uVar2;
    BOOL16      BVar3;
    ushort      uVar4;
    uint        uVar5;
    undefined2  uVar6;
    ushort      uVar7;
    ushort      uVar8;
    undefined2  local_16;
    astruct_99 *paStack20;
    undefined2  local_10[0x2];
    ushort      local_c[0x3];
    uint        uStack6;
    uint        local_4;

    file_1028_b81a(param_1, param_2, param_3, param_5, param_4);
    if(param_3 != 0x0)
    {
        uVar7 = (ushort)param_2;
        uVar8 = (ushort)(param_2 >> 0x10);
        BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)&local_4, 0x0, param_5, 0x2, 0x1008);
        if(BVar3 != 0x0)
        {
            uStack6 = 0x0;
            while(true)
            {
                if(local_4 <= uStack6)
                {
                    return;
                }
                paStack20 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_68a2);
                uVar5     = (uint)((ulong)paStack20 >> 0x10);
                uVar2     = (uint)paStack20;
                if((uVar5 | uVar2) == 0x0)
                {
                    paStack20 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack20->field_0x0         = 0x389a;
                    *(undefined2 *)(uVar2 + 0x2) = 0x1008;
                    *(undefined2 *)(uVar2 + 0x4) = 0x0;
                    *(undefined2 *)(uVar2 + 0x6) = 0x0;
                    *(undefined2 *)(uVar2 + 0x8) = 0x0;
                    *(undefined2 *)(uVar2 + 0xa) = 0x0;
                    *(undefined2 *)(uVar2 + 0xc) = 0x0;
                    paStack20->field_0x0         = 0x56ce;
                    *(undefined2 *)(uVar2 + 0x2) = 0x1018;
                }
                BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)local_10, 0x0, param_5, 0x2, 0x1008);
                if(BVar3 == 0x0)
                    break;
                BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)local_c, 0x0, param_5, 0x2, 0x1008);
                if(BVar3 == 0x0)
                    break;
                BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)&local_16, 0x0, param_5, 0x2, 0x1008);
                if(BVar3 == 0x0)
                    break;
                BVar3 = read_file_1008_7dee(
                  uVar7, uVar8, (int)paStack20 + 0xa, 0x0, (ushort)((ulong)paStack20 >> 0x10), 0x2, 0x1008);
                if(BVar3 == 0x0)
                    break;
                BVar3 = read_file_1008_7dee(
                  uVar7, uVar8, (int)paStack20 + 0xc, 0x0, (ushort)((ulong)paStack20 >> 0x10), 0x2, 0x1008);
                if(BVar3 == 0x0)
                    break;
                *(undefined2 *)((int)paStack20 + 0x4) = local_10[0];
                uVar4                                 = switch_1008_72bc(uVar7, uVar8, local_c[0]);
                uVar6                                 = (undefined2)((ulong)paStack20 >> 0x10);
                *(ushort *)((int)paStack20 + 0x6)     = uVar4;
                *(undefined2 *)((int)paStack20 + 0x8) = local_16;
                ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x20) + 0x8);
                (**ppcVar1)();
                uStack6 = uStack6 + 0x1;
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}
