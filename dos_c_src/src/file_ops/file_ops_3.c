
void __stdcall16far pass1_1018_0000(ulong param_1, ulong param_2, int param_3, uchar *param_4, uint16_t param_5)

{
    int      *piVar1;
    int       iVar2;
    ulong     uVar3;
    ushort    uVar4;
    int       iVar5;
    BOOL16    BVar6;
    ushort    uVar7;
    ushort    uVar8;
    undefined local_20[0x10];
    int       iStack16;

    // Segment:    4
    // Offset:     00024460
    // Length:     ee6a
    // Min Alloc:  ee6a
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar8 = (ushort)param_2;
    uVar7 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe(uVar8, uVar7, 0x2, 0x1008, param_5);
    if(param_3 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d4;
    }
    else
    {
        iVar5 = (int)param_1;
        uVar4 = (ushort)(param_1 >> 0x10);
        BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x16, 0x0, uVar4, 0x4, 0x1008);
        if((((BVar6 != 0x0) && (BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x1a, 0x0, uVar4, 0x4, 0x1008), BVar6 != 0x0)) && (BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x20, 0x0, uVar4, 0x4, 0x1008), BVar6 != 0x0))
           && (((BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x24, 0x0, uVar4, 0x4, 0x1008), BVar6 != 0x0 && (BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x30, 0x0, uVar4, 0x2, 0x1008), BVar6 != 0x0))
                && (BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x32, 0x0, uVar4, 0x2, 0x1008), BVar6 != 0x0))))
        {
            if(*(int *)(iVar5 + 0x30) != 0x0)
            {
                iVar2 = *(int *)(iVar5 + 0x32);
                if(_PTR_LOOP_1050_5f2c == 0x0)
                {
                    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4, 0x1000);
                    PTR_LOOP_1050_5f2e = param_4;
                }
                else
                {
                }
                uVar7                     = fn_ptr_op_1000_1708(iVar2 * 0x6, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
                *(ushort *)(iVar5 + 0x2c) = uVar7;
                *(uchar **)(iVar5 + 0x2e) = PTR_LOOP_1050_5f2e;
                pass1_1008_3e38((ushort *)CONCAT22(param_5, local_20));
                for(iStack16 = 0x0; piVar1 = (int *)(iVar5 + 0x30), *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 = iStack16 + 0x1)
                {
                    BVar6 = read_file_1008_7bc8(param_2, (ushort *)CONCAT22(param_5, local_20), 0x1008, param_5);
                    if(BVar6 == 0x0)
                    {
                        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                        return;
                    }
                    uVar3 = *(ulong *)(iVar5 + 0x2c);
                    pass1_1008_3f62((ushort *)(uVar3 & 0xffff0000 | (ulong)(uint)((int)uVar3 + iStack16 * 0x6)), (ushort *)CONCAT22(param_5, local_20));
                }
            }
            return;
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}


void __stdcall16far pass1_1010_89f0(ushort param_1, ushort param_2, ushort param_3, ulong param_4, HINSTANCE16 param_5, ushort param_6)

{
    uint       uVar1;
    undefined2 uVar2;
    uint       uVar3;
    uchar     *puVar4;
    uchar     *puVar5;
    int        iVar6;
    ulong      uVar7;
    undefined4 uStack22;
    uint       uStack8;

    uVar3 = *(uint *)(param_1 + 0x67c);
    uVar1 = *(uint *)(param_1 + 0x67e);
    if((uVar1 | uVar3) != 0x0)
    {
        pass1_1008_64a2((uint *)CONCAT22(uVar1, uVar3));
        param_5 = 0x1000;
        fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar1, uVar3), 0x1000);
    }
    uVar7  = set_err_mode_1010_8b14(CONCAT22(param_2, param_1), *(ULONG *)(*(int *)(param_1 + 0xe82) * 0x4 + 0x24be), param_6);
    puVar4 = (uchar *)(uVar7 >> 0x10);
    uVar3  = (uint)uVar7;
    iVar6  = *(int *)(param_1 + 0xe82) * 0x4;
    if((*(uint *)(iVar6 + 0x24be) == uVar3) && (*(uchar **)(iVar6 + 0x24c0) == puVar4))
    {
        msg_box_op_1010_8bb4(param_1, param_2, uVar7, param_5, param_6);
    }
    mem_op_1000_179c(0x8, puVar4, 0x1000);
    puVar5 = (uchar *)((uint)puVar4 | uVar3);
    if(puVar5 == (uchar *)0x0)
    {
        uVar3  = 0x0;
        puVar5 = (uchar *)0x0;
    }
    else
    {
        file_1008_6414((ulong *)CONCAT13((char)((uint)puVar4 >> 0x8), CONCAT12((char)puVar4, uVar3)), uVar7, param_6, puVar5);
    }
    *(uint *)(param_1 + 0x67c)       = uVar3;
    *(uchar **)(param_1 + 0x67e)     = puVar5;
    *(undefined2 *)(param_1 + 0x680) = 0x0;
    if((*(uint *)(param_1 + 0x67e) | *(uint *)(param_1 + 0x67c)) != 0x0)
    {
        for(uStack8 = 0x1; (int)uStack8 < 0xa; uStack8 = uStack8 + 0x1)
        {
            iVar6 = uStack8 * 0xa;
            uVar2 = *(undefined2 *)(iVar6 + 0x2558);
            uVar3 = uStack8;
            pass1_1008_64c8(*(ulong **)(param_1 + 0x67c), CONCAT13((char)((uint)uVar2 >> 0x8), CONCAT12((char)uVar2, *(undefined2 *)(iVar6 + 0x255a))), *(int *)(iVar6 + 0x2556), uStack8, puVar5);
            uStack22 = CONCAT22(puVar5, uVar3);
            pass1_1010_86de(param_1, param_2, (uchar)param_3, CONCAT22(puVar5, uVar3));
            *(undefined4 *)(uStack8 * 0x4 + (int)param_4) = uStack22;
        }
    }
    return;
}

void __stdcall16far write_to_file_1010_6372(ulong param_1, ulong param_2, uint16_t param_3)

{
    BOOL16       BVar1;
    astruct_729 *iVar2;
    undefined2   uVar2;
    ushort       uVar3;
    ushort       uVar4;
    undefined4   local_10[0x2];
    undefined4   local_8;

    BVar1 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar1 != 0x0)
    {
        uVar2       = (undefined2)(param_1 >> 0x10);
        iVar2       = (astruct_729 *)param_1;
        local_10[0] = iVar2->field_0xa;
        uVar3       = (ushort)param_2;
        uVar4       = (ushort)(param_2 >> 0x10);
        BVar1       = write_to_file_1008_7e1c(uVar3, uVar4, (ushort)local_10, param_3, (char *)0x4, 0x1008);
        if(BVar1 != 0x0)
        {
            local_8 = iVar2->field_0xe;
            BVar1   = write_to_file_1008_7e1c(uVar3, uVar4, (ushort)&local_8, param_3, (char *)0x4, 0x1008);
            if(BVar1 != 0x0)
            {
                local_8 = iVar2->field_0x12;
                BVar1   = write_to_file_1008_7e1c(uVar3, uVar4, (ushort)&local_8, param_3, (char *)0x4, 0x1008);
                if(BVar1 != 0x0)
                {
                    local_8 = iVar2->field_0x16;
                    BVar1   = write_to_file_1008_7e1c(uVar3, uVar4, (ushort)&local_8, param_3, (char *)0x4, 0x1008);
                    if(BVar1 != 0x0)
                    {
                        local_8 = iVar2->field_0x1a;
                        BVar1   = write_to_file_1008_7e1c(uVar3, uVar4, (ushort)&local_8, param_3, (char *)0x4, 0x1008);
                        if(BVar1 != 0x0)
                        {
                            local_8 = iVar2->field_0x1e;
                            BVar1   = write_to_file_1008_7e1c(uVar3, uVar4, (ushort)&local_8, param_3, (char *)0x4, 0x1008);
                            if(BVar1 != 0x0)
                            {
                                local_8 = iVar2->field_0x22;
                                BVar1   = write_to_file_1008_7e1c(uVar3, uVar4, (ushort)&local_8, param_3, (char *)0x4, 0x1008);
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
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


void __stdcall16far pass1_1010_648a(ulong param_1, ulong param_2, int param_3, uint16_t param_4)

{
    ushort uVar1;
    int    iVar2;
    BOOL16 BVar3;
    ushort uVar4;
    ushort uVar5;

    uVar4 = (ushort)param_2;
    uVar5 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe(uVar4, uVar5, 0x7, 0x1008, param_4);
    if(param_3 != 0x0)
    {
        iVar2 = (int)param_1;
        uVar1 = (ushort)(param_1 >> 0x10);
        BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0xa, 0x0, uVar1, 0x4, 0x1008);
        if(BVar3 != 0x0)
        {
            BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0xe, 0x0, uVar1, 0x4, 0x1008);
            if(BVar3 != 0x0)
            {
                BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x12, 0x0, uVar1, 0x4, 0x1008);
                if(BVar3 != 0x0)
                {
                    BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x16, 0x0, uVar1, 0x4, 0x1008);
                    if(BVar3 != 0x0)
                    {
                        BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x1a, 0x0, uVar1, 0x4, 0x1008);
                        if(BVar3 != 0x0)
                        {
                            BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x1e, 0x0, uVar1, 0x4, 0x1008);
                            if(BVar3 != 0x0)
                            {
                                BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x22, 0x0, uVar1, 0x4, 0x1008);
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
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}

void __stdcall16far write_to_file_1010_6846(undefined4 param_1, undefined4 param_2, uint16_t param_3)

{
    ushort     uVar1;
    BOOL16     BVar2;
    int        iVar3;
    ushort     uVar4;
    ushort     uVar5;
    undefined2 local_c[0x5];

    BVar2 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar2 != 0x0)
    {
        iVar3 = (int)param_1;
        uVar1 = (ushort)((ulong)param_1 >> 0x10);
        uVar4 = (ushort)param_2;
        uVar5 = (ushort)((ulong)param_2 >> 0x10);
        BVar2 = write_to_file_1008_7e1c(uVar4, uVar5, iVar3 + 0xa, uVar1, (char *)0x114, 0x1008);
        if(BVar2 != 0x0)
        {
            BVar2 = write_to_file_1008_7e1c(uVar4, uVar5, iVar3 + 0x11e, uVar1, (char *)0x2a, 0x1008);
            if(BVar2 != 0x0)
            {
                local_c[0] = *(undefined2 *)(iVar3 + 0x148);
                BVar2      = write_to_file_1008_7e1c(uVar4, uVar5, (ushort)local_c, param_3, (char *)0x2, 0x1008);
                if(BVar2 != 0x0)
                {
                    return;
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


void __stdcall16far pass1_1010_68c6(ulong param_1, ulong param_2, uint param_3, uchar *param_4, uint16_t param_5)

{
    astruct_248 *iVar2;
    BOOL16       BVar1;
    int          iVar3;
    ushort       uVar4;
    ushort       uVar5;
    uchar       *puVar6;
    ushort       uVar7;
    ushort       uVar8;
    SEGPTR       SVar9;
    ushort       uVar10;
    astruct_18  *paStack18;
    astruct_18  *paStack10;
    int          local_6;
    uint         uStack4;

    uVar8  = (ushort)param_2;
    uVar10 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe(uVar8, uVar10, 0x3, 0x1008, param_5);
    if(param_3 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d4;
        return;
    }
    iVar2 = (astruct_248 *)param_1;
    uVar7 = (ushort)(param_1 >> 0x10);
    if((int)PTR_LOOP_1050_0312 < 0x2)
    {
        uVar4 = 0x102;
        SVar9 = 0x102;
        mem_op_1000_179c(0x102, param_4, 0x1000);
        paStack10 = (astruct_18 *)CONCAT22(param_4, param_3);
        puVar6    = param_4;
        BVar1     = read_file_1008_7dee(uVar8, uVar10, param_3, uVar4, (ushort)param_4, SVar9, 0x1008);
        paStack18 = paStack10;
        if(BVar1 == 0x0)
            goto LAB_1010_692c;
        uStack4 = 0x1;
        do
        {
            iVar3                                            = switch_1008_73ea(uVar8, uVar10, uStack4);
            *(undefined2 *)(&iVar2->field_0xa + iVar3 * 0x2) = *(undefined2 *)(uStack4 * 0x2 + param_3);
            uStack4                                          = uStack4 + 0x1;
        } while(uStack4 < 0x81);
        fn_ptr_1000_17ce(paStack10, 0x1000);
        uVar4   = (ushort)paStack10;
        param_4 = puVar6;
    }
    else
    {
        uVar4 = read_file_1008_7dee(uVar8, uVar10, &iVar2->field_0xa, 0x0, uVar7, 0x114, 0x1008);
        if(uVar4 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
    }
    if((int)PTR_LOOP_1050_0312 < 0x2)
    {
        uVar5 = 0x2a;
        SVar9 = 0x2a;
        mem_op_1000_179c(0x2a, param_4, 0x1000);
        paStack18 = (astruct_18 *)CONCAT22(param_4, uVar4);
        BVar1     = read_file_1008_7dee(uVar8, uVar10, uVar4, uVar5, (ushort)param_4, SVar9, 0x1008);
        if(BVar1 == 0x0)
        {
        LAB_1010_692c:
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            fn_ptr_1000_17ce((astruct_18 *)((ulong)paStack18 & 0xffff | ZEXT24(param_4) << 0x10), 0x1000);
            return;
        }
        uStack4 = 0x0;
        do
        {
            uVar5                                              = switch_1008_72bc(uVar8, uVar10, uStack4);
            *(undefined2 *)(&iVar2->field_0x11e + uVar5 * 0x2) = *(undefined2 *)(uStack4 * 0x2 + uVar4);
            uStack4                                            = uStack4 + 0x1;
        } while(uStack4 < 0x15);
        fn_ptr_1000_17ce(paStack18, 0x1000);
    }
    else
    {
        BVar1 = read_file_1008_7dee(uVar8, uVar10, &iVar2->field_0x11e, 0x0, uVar7, 0x2a, 0x1008);
        if(BVar1 == 0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
        }
    }
    BVar1 = read_file_1008_7dee(uVar8, uVar10, (ushort)&local_6, 0x0, param_5, 0x2, 0x1008);
    if(BVar1 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return;
    }
    BVar1              = switch_1008_73ea(uVar8, uVar10, local_6);
    iVar2->field_0x148 = BVar1;
    return;
}

undefined2 __stdcall16far pass1_1010_5dc6(ulong param_1, ulong param_2, uint16_t param_3)

{
    BOOL16     BVar1;
    int        iVar2;
    undefined2 uVar3;
    ushort     uVar4;
    undefined *local_c[0x3];
    undefined2 local_6[0x2];

    BVar1 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar1 != 0x0)
    {
        uVar3 = (undefined2)(param_1 >> 0x10);
        iVar2 = (int)param_1;
        BVar1 = pass1_1008_7c2a(param_2, *(char **)(iVar2 + 0x68), 0x1008);
        if(BVar1 != 0x0)
        {
            BVar1 = pass1_1008_7c2a(param_2, *(char **)(iVar2 + 0x6c), 0x1008);
            if(BVar1 != 0x0)
            {
                local_c[0] = PTR_LOOP_1050_13ae;
                uVar4      = (ushort)(param_2 >> 0x10);
                BVar1      = write_to_file_1008_7e1c((ushort)param_2, uVar4, (ushort)local_c, param_3, (char *)0x2, 0x1008);
                if(BVar1 != 0x0)
                {
                    local_6[0] = *(undefined2 *)(iVar2 + 0x82);
                    BVar1      = write_to_file_1008_7e1c((ushort)param_2, uVar4, (ushort)local_6, param_3, (char *)0x2, 0x1008);
                    if(BVar1 != 0x0)
                    {
                        return 0x1;
                    }
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return 0x0;
}


void __stdcall16far pass1_1010_5e56(ulong param_1, ulong param_2, int param_3, ushort param_4, uint16_t param_5)

{
    undefined *puVar1;
    ushort     uVar2;
    BOOL16     BVar3;
    int        iVar4;
    ushort     uVar5;
    ushort     uVar6;
    ushort     uVar7;
    undefined *local_404;
    undefined  local_402[0x400];

    uVar6 = (ushort)param_2;
    uVar7 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe(uVar6, uVar7, 0x4, 0x1008, param_5);
    if(param_3 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d4;
    }
    else
    {
        puVar1 = local_402;
        read_file_1008_7c6e(uVar6, uVar7, (char *)CONCAT22(param_5, puVar1), 0x1008);
        if(puVar1 != (undefined *)0x0)
        {
            uVar2                     = str_op_1008_60e8((char *)CONCAT22(param_5, local_402), param_4);
            uVar5                     = (ushort)(param_1 >> 0x10);
            iVar4                     = (int)param_1;
            *(ushort *)(iVar4 + 0x68) = uVar2;
            *(ushort *)(iVar4 + 0x6a) = param_4;
            puVar1                    = local_402;
            read_file_1008_7c6e(uVar6, uVar7, (char *)CONCAT22(param_5, puVar1), 0x1008);
            if(puVar1 != (undefined *)0x0)
            {
                uVar2                     = str_op_1008_60e8((char *)CONCAT22(param_5, local_402), param_4);
                *(ushort *)(iVar4 + 0x6c) = uVar2;
                *(ushort *)(iVar4 + 0x6e) = param_4;
                BVar3                     = read_file_1008_7dee(uVar6, uVar7, (ushort)&local_404, 0x0, param_5, 0x2, 0x1008);
                if(BVar3 != 0x0)
                {
                    PTR_LOOP_1050_13ae = local_404;
                    if((int)PTR_LOOP_1050_0312 < 0x2)
                    {
                        return;
                    }
                    BVar3 = read_file_1008_7dee(uVar6, uVar7, iVar4 + 0x82, 0x0, uVar5, 0x2, 0x1008);
                    if(BVar3 != 0x0)
                    {
                        return;
                    }
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}

void __stdcall16far find_n_load_rsrc_1010_4e9e(ulong param_1, HGLOBAL16 param_2)

{
    BOOL16    BVar1;
    HRSRC16   h_rsrc;
    int       iVar2;
    ushort    uVar3;
    HGLOBAL16 HVar3;

    uVar3 = (ushort)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(int *)(iVar2 + 0x20) != 0x0)
    {
        HVar3 = param_2;
        if(*(int *)(iVar2 + 0x2a) != 0x0)
        {
            HVar3 = (HGLOBAL16)s_tile2_bmp_1050_1538;
            BVar1 = GlobalUnlock16(param_2);
            if(BVar1 == 0x0)
            {
                HVar3 = (HGLOBAL16)s_tile2_bmp_1050_1538;
                FreeResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
            }
        }
        h_rsrc                       = FindResource16(HVar3, (LPCSTR)&PTR_LOOP_1050_000a, (LPCSTR)0x0);
        HVar3                        = LoadResource16((HMODULE16)s_tile2_bmp_1050_1538, h_rsrc);
        *(HGLOBAL16 *)(iVar2 + 0x2a) = HVar3;
        if(HVar3 != 0x0)
        {
            WIN16_LockResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
            return;
        }
    }
    return;
}

void __stdcall16far pass1_1010_404a(ulong param_1, ulong param_2, int param_3, uint16_t param_4)

{
    ushort     uVar1;
    int        iVar2;
    BOOL16     BVar3;
    ushort     uVar4;
    ushort     uVar5;
    undefined2 local_4;

    uVar4 = (ushort)param_2;
    uVar5 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe(uVar4, uVar5, 0x5, 0x1008, param_4);
    if(param_3 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d4;
    }
    else
    {
        iVar2 = (int)param_1;
        uVar1 = (ushort)(param_1 >> 0x10);
        BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x24, 0x0, uVar1, 0x2, 0x1008);
        if(BVar3 != 0x0)
        {
            BVar3 = read_file_1008_7dee(uVar4, uVar5, (ushort)&local_4, 0x0, param_4, 0x2, 0x1008);
            if(BVar3 != 0x0)
            {
                BVar3 = read_file_1008_7dee(uVar4, uVar5, iVar2 + 0x7e, 0x0, uVar1, 0x2, 0x1008);
                if(BVar3 != 0x0)
                {
                    *(undefined2 *)(iVar2 + 0x6a) = local_4;
                    return;
                }
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}

void __stdcall16far pass1_1010_0ad2(ulong param_1, ulong param_2, uint16_t param_3)

{
    undefined4 uVar1;
    BOOL16     BVar2;
    undefined *puVar3;
    uint       extraout_DX;
    int        iVar4;
    undefined2 uVar5;
    ushort     uVar6;
    ushort     uVar7;
    undefined4 local_2a[0x2];
    undefined2 local_22[0x2];
    undefined2 local_1e[0x3];
    undefined2 local_18[0x3];
    undefined4 uStack18;
    undefined  local_e[0x8];
    undefined2 uStack6;
    int        iStack4;

    BVar2 = write_to_file_1008_7cac(param_2, param_3);
    if(BVar2 == 0x0)
    {
        return;
    }
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0xa) == 0x0)
    {
        uStack6 = 0x0;
    }
    else
    {
        uVar1   = *(undefined4 *)(iVar4 + 0xa);
        uStack6 = *(undefined2 *)((int)uVar1 + 0x8);
    }
    local_1e[0] = uStack6;
    uVar6       = (ushort)param_2;
    uVar7       = (ushort)(param_2 >> 0x10);
    BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_1e, param_3, (char *)0x2, 0x1008);
    if(BVar2 != 0x0)
    {
        pass1_1008_5784((ulong *)CONCAT22(param_3, local_e), *(ulong *)(iVar4 + 0xa));
        do
        {
            puVar3 = local_e;
            pass1_1008_5b12(puVar3, param_3);
            uStack18 = CONCAT22(extraout_DX, puVar3);
            if((extraout_DX | (uint)puVar3) == 0x0)
            {
                local_22[0] = *(undefined2 *)(iVar4 + 0xe);
                BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_22, param_3, (char *)0x2, 0x1008);
                if(BVar2 == 0x0)
                {
                    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                    return;
                }
                local_22[0] = *(undefined2 *)(iVar4 + 0x10);
                BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_22, param_3, (char *)0x2, 0x1008);
                if(BVar2 == 0x0)
                {
                    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                    return;
                }
                if(*(int *)(iVar4 + 0x18) != 0x0)
                {
                    DAT_1050_0e28      = *(undefined2 *)(iVar4 + 0x12);
                    PTR_LOOP_1050_0e30 = (undefined *)*(undefined2 *)(iVar4 + 0x14);
                    PTR_LOOP_1050_0ea8 = (undefined *)*(undefined2 *)(iVar4 + 0x16);
                }
                iStack4 = 0x0;
                while(true)
                {
                    if(0x9 < iStack4)
                    {
                        iStack4 = 0x0;
                        while(true)
                        {
                            if(0x2 < iStack4)
                            {
                                if(*(int *)(iVar4 + 0x18) != 0x0)
                                {
                                    DAT_1050_0e28      = 0x0;
                                    PTR_LOOP_1050_0e30 = (undefined *)0x0;
                                    PTR_LOOP_1050_0ea8 = (undefined *)0x0;
                                }
                                return;
                            }
                            local_1e[0] = *(undefined2 *)(iStack4 * 0x8 + 0xea8);
                            BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_1e, param_3, (char *)0x2, 0x1008);
                            if(BVar2 == 0x0)
                                break;
                            iStack4 = iStack4 + 0x1;
                        }
                        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                        return;
                    }
                    local_1e[0] = *(undefined2 *)(iStack4 * 0x8 + 0xe28);
                    BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_1e, param_3, (char *)0x2, 0x1008);
                    if(BVar2 == 0x0)
                        break;
                    iStack4 = iStack4 + 0x1;
                }
                PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                return;
            }
            local_18[0] = *(undefined2 *)(puVar3 + 0x4);
            BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_18, param_3, (char *)0x2, 0x1008);
            if(BVar2 == 0x0)
            {
                PTR_LOOP_1050_0310 = (undefined *)0x6d0;
                return;
            }
            local_2a[0] = *(undefined4 *)((int)uStack18 + 0x6);
            BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_2a, param_3, (char *)0x4, 0x1008);
        } while(BVar2 != 0x0);
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return;
}

void __stdcall16far file_1010_0c7c(ulong param_1, ulong param_2, int param_3, uchar *param_4, uint16_t param_5)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    BOOL16       BVar3;
    astruct_229 *uVar4;
    ushort       uVar5;
    uchar       *extraout_DX;
    astruct_228 *iVar6;
    undefined2   uVar6;
    ushort       uVar7;
    ushort       uVar8;
    undefined2   local_2a[0x2];
    ushort       uStack38;
    undefined4  *puStack26;
    undefined4  *puStack22;
    undefined2   local_12[0x5];
    astruct_229 *paStack8;
    astruct_229 *local_6;
    ushort       uStack4;

    uVar7 = (ushort)param_2;
    uVar8 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe(uVar7, uVar8, 0x6, 0x1008, param_5);
    if(param_3 == 0x0)
    {
        PTR_LOOP_1050_0310 = (undefined *)0x6d4;
    }
    else
    {
        BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)&local_6, 0x0, param_5, 0x2, 0x1008);
        if(BVar3 != 0x0)
        {
            paStack8 = (astruct_229 *)0x0;
            while(true)
            {
                iVar6 = (astruct_228 *)param_1;
                uVar5 = (ushort)(param_1 >> 0x10);
                if(local_6 <= paStack8)
                    break;
                uVar4 = local_6;
                mem_op_1000_179c(0xa, param_4, 0x1000);
                puStack26 = (undefined4 *)CONCAT22(param_4, uVar4);
                if(((uint)param_4 | (uint)uVar4) == 0x0)
                {
                    puStack22 = (undefined4 *)0x0;
                }
                else
                {
                    *(undefined2 *)puStack26 = 0x389a;
                    uVar4->field_0x2         = 0x1008;
                    *(undefined2 *)puStack26 = 0xea8;
                    uVar4->field_0x2         = 0x1010;
                    puStack22                = puStack26;
                }
                BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)local_12, 0x0, param_5, 0x2, 0x1008);
                if((BVar3 == 0x0) || (BVar3 = read_file_1008_7dee(uVar7, uVar8, (int)puStack22 + 0x6, 0x0, (ushort)((ulong)puStack22 >> 0x10), 0x4, 0x1008), BVar3 == 0x0))
                {
                    puStack26 = puStack22;
                    if(puStack22 != (undefined4 *)0x0)
                    {
                        ppcVar2 = (code **)*puStack22;
                        (**ppcVar2)(0x1008, (int)puStack22, (int)((ulong)puStack22 >> 0x10), 0x1);
                    }
                    goto LAB_1010_0cb1;
                }
                uVar6                                 = (undefined2)((ulong)puStack22 >> 0x10);
                *(undefined2 *)((int)puStack22 + 0x4) = local_12[0];
                puVar1                                = iVar6->field_0xa;
                ppcVar2                               = (code **)((int)*iVar6->field_0xa + 0x8);
                (**ppcVar2)(0x8, (int)puVar1, (int)((ulong)puVar1 >> 0x10), (int)puStack22, uVar6);
                paStack8 = (astruct_229 *)&paStack8->field_0x1;
                param_4  = extraout_DX;
            }
            BVar3 = read_file_1008_7dee(uVar7, uVar8, &iVar6->field_0xe, 0x0, uVar5, 0x2, 0x1008);
            if((BVar3 != 0x0) && (BVar3 = read_file_1008_7dee(uVar7, uVar8, &iVar6->field_0x10, 0x0, uVar5, 0x2, 0x1008), BVar3 != 0x0))
            {
                for(uStack4 = 0x0; (int)uStack4 < 0xa; uStack4 = uStack4 + 0x1)
                {
                    BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)local_2a, 0x0, param_5, 0x2, 0x1008);
                    if(BVar3 == 0x0)
                        goto LAB_1010_0cb1;
                    uVar5 = uStack4;
                    if((int)PTR_LOOP_1050_0312 < 0x2)
                    {
                        uVar5 = pass1_1008_738c(uVar7, uVar8, uStack4);
                    }
                    *(undefined2 *)(uVar5 * 0x8 + 0xe28) = local_2a[0];
                    uStack38                             = uVar5;
                }
                if(0x2 < (int)PTR_LOOP_1050_0312)
                {
                    uStack4 = 0x0;
                    do
                    {
                        BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)local_2a, 0x0, param_5, 0x2, 0x1008);
                        if(BVar3 == 0x0)
                            goto LAB_1010_0cb1;
                        *(undefined2 *)(uStack4 * 0x8 + 0xea8) = local_2a[0];
                        uStack4                                = uStack4 + 0x1;
                    } while((int)uStack4 < 0x3);
                }
                return;
            }
        }
    LAB_1010_0cb1:
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}

void __stdcall16far pass1_1008_e5da(ulong param_1, ulong param_2, HFILE16 param_3, uint16_t param_4)

{
    undefined4 uVar1;
    BOOL16     BVar2;
    undefined *puVar3;
    uint       extraout_DX;
    int        iVar4;
    undefined2 uVar5;
    ushort     uVar6;
    ushort     uVar7;
    undefined4 local_30[0x2];
    undefined4 local_28;
    undefined4 local_24[0x2];
    undefined2 local_1c[0x3];
    undefined2 local_16[0x3];
    undefined4 uStack16;
    undefined  local_c[0x8];
    undefined2 uStack4;

    BVar2 = write_to_file_1008_7cac(param_2, param_4);
    if(BVar2 != 0x0)
    {
        uVar5 = (undefined2)(param_1 >> 0x10);
        iVar4 = (int)param_1;
        if(*(long *)(iVar4 + 0xa) == 0x0)
        {
            uStack4 = 0x0;
        }
        else
        {
            uVar1   = *(undefined4 *)(iVar4 + 0xa);
            uStack4 = *(undefined2 *)((int)uVar1 + 0x8);
        }
        local_1c[0] = uStack4;
        uVar6       = (ushort)param_2;
        uVar7       = (ushort)(param_2 >> 0x10);
        BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_1c, param_4, (char *)0x2, param_3);
        if(BVar2 != 0x0)
        {
            pass1_1008_5784((ulong *)CONCAT22(param_4, local_c), *(ulong *)(iVar4 + 0xa));
            do
            {
                puVar3 = local_c;
                pass1_1008_5b12(puVar3, param_4);
                uStack16 = CONCAT22(extraout_DX, puVar3);
                if((extraout_DX | (uint)puVar3) == 0x0)
                {
                    return;
                }
                local_24[0] = *(undefined4 *)(puVar3 + 0x4);
                BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_24, param_4, (char *)0x4, param_3);
                if(BVar2 == 0x0)
                    break;
                local_28 = *(undefined4 *)((int)uStack16 + 0x8);
                BVar2    = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)&local_28, param_4, (char *)0x4, param_3);
                if(BVar2 == 0x0)
                    break;
                local_16[0] = *(undefined2 *)((int)uStack16 + 0xc);
                BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_16, param_4, (char *)0x2, param_3);
                if(BVar2 == 0x0)
                    break;
                local_30[0] = *(undefined4 *)((int)uStack16 + 0xe);
                BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_30, param_4, (char *)0x4, param_3);
                if(BVar2 == 0x0)
                    break;
                local_16[0] = *(undefined2 *)((int)uStack16 + 0x12);
                BVar2       = write_to_file_1008_7e1c(uVar6, uVar7, (ushort)local_16, param_4, (char *)0x2, param_3);
            } while(BVar2 != 0x0);
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    }
    return;
}


void __stdcall16far file_1008_e70e(ulong param_1, ulong param_2, int param_3, uchar *param_4, uint16_t param_5, uint16_t param_6)

{
    undefined4  uVar1;
    code      **ppcVar2;
    BOOL16      BVar3;
    uint        uVar4;
    uchar      *extraout_DX;
    ushort      uVar5;
    undefined2  uVar6;
    ushort      uVar7;
    ushort      uVar8;
    undefined2  uVar9;
    undefined2  local_12[0x2];
    undefined4 *puStack14;
    uint        uStack10;
    uint        local_4;

    if((int)PTR_LOOP_1050_0312 < 0x2)
    {
        return;
    }
    uVar7 = (ushort)param_2;
    uVar8 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe(uVar7, uVar8, 0x14, param_5, param_6);
    if(param_3 != 0x0)
    {
        BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)&local_4, 0x0, param_6, 0x2, param_5);
        if(BVar3 != 0x0)
        {
            if(local_4 == 0x0)
            {
                return;
            }
            uStack10 = 0x0;
            while(true)
            {
                if(local_4 <= uStack10)
                {
                    return;
                }
                uVar9 = 0x14;
                uVar4 = local_4;
                mem_op_1000_179c(0x14, param_4, 0x1000);
                uVar5 = (uint)param_4 | uVar4;
                if(uVar5 == 0x0)
                {
                    uVar4 = 0x0;
                    uVar5 = 0x0;
                }
                else
                {
                    struct_1008_dcdc((ushort *)CONCAT22(param_4, uVar4));
                }
                puStack14 = (undefined4 *)CONCAT22(uVar5, uVar4);
                BVar3     = read_file_1008_7dee(uVar7, uVar8, uVar4 + 0x4, 0x0, uVar5, 0x4, 0x1000);
                if((((BVar3 == 0x0) || (BVar3 = read_file_1008_7dee(uVar7, uVar8, (int)puStack14 + 0x8, 0x0, (ushort)((ulong)puStack14 >> 0x10), 0x4, 0x1000), BVar3 == 0x0))
                    || (BVar3 = read_file_1008_7dee(uVar7, uVar8, (ushort)local_12, 0x0, param_6, 0x2, 0x1000), BVar3 == 0x0))
                   || ((BVar3 = read_file_1008_7dee(uVar7, uVar8, (int)puStack14 + 0xe, 0x0, (ushort)((ulong)puStack14 >> 0x10), 0x4, 0x1000),
                        BVar3 == 0x0 || (BVar3 = read_file_1008_7dee(uVar7, uVar8, (int)puStack14 + 0x12, 0x0, (ushort)((ulong)puStack14 >> 0x10), 0x2, 0x1000), BVar3 == 0x0))))
                    break;
                uVar9                                 = (undefined2)((ulong)puStack14 >> 0x10);
                *(undefined2 *)((int)puStack14 + 0xc) = local_12[0];
                uVar6                                 = (undefined2)(param_1 >> 0x10);
                uVar1                                 = *(undefined4 *)((int)param_1 + 0xa);
                ppcVar2                               = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xa) + 0x4);
                (**ppcVar2)(0x0, (int)uVar1, (int)((ulong)uVar1 >> 0x10), (int)puStack14, uVar9);
                uStack10 = uStack10 + 0x1;
                param_4  = extraout_DX;
            }
            if(puStack14 != (undefined4 *)0x0)
            {
                ppcVar2 = (code **)*puStack14;
                (**ppcVar2)(0x1000, (int)puStack14, (int)((ulong)puStack14 >> 0x10), 0x1, uVar9, puStack14);
            }
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    }
    return;
}
