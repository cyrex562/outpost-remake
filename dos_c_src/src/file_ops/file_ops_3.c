
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

void __stdcall16far pass1_1008_c98e(ulong param_1,ulong param_2,HFILE16 param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  undefined4 local_10 [0x3];
  
  BVar1 = write_to_file_1008_7cac(param_2,param_4);
  if (BVar1 != 0x0) {
    local_10[0] = *(undefined4 *)((int)param_1 + 0xe);
    BVar1 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)local_10,param_4,(char *)0x4,param_3);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1008_c9d4(ulong param_1,ulong param_2,int param_3,uint16_t param_4,longlong param_5)

{
  BOOL16 BVar1;
  uint16_t unaff_SS;
  ushort uVar2;
  
  if (0x1 < (int)PTR_LOOP_1050_0312) {
    uVar2 = (ushort)(param_2 >> 0x10);
    read_file_1008_7cfe((ushort)param_2,uVar2,0x15,param_4,unaff_SS);
    if (param_3 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d4;
      return;
    }
    BVar1 = read_file_1008_7dee((ushort)param_2,uVar2,(int)param_1 + 0xe,0x0,(ushort)(param_1 >> 0x10),0x4,param_4);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
  }
  return;
}

void __stdcall16far pass1_1008_ba38(ulong param_1,ulong param_2,HFILE16 param_3,uint16_t param_4)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  undefined2 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined4 local_2a [0x3];
  undefined2 local_1e [0x5];
  undefined local_14 [0x8];
  undefined2 local_c;
  undefined4 uStack10;
  undefined2 local_6 [0x2];
  
  BVar2 = write_to_file_1008_7cac(param_2,param_4);
  if (BVar2 != 0x0) {
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    local_c = *(undefined2 *)(iVar4 + 0x22);
    uVar6 = (ushort)param_2;
    uVar7 = (ushort)(param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)&local_c,param_4,(char *)0x2,param_3);
    if (BVar2 != 0x0) {
      if (*(long *)(iVar4 + 0xa) == 0x0) {
        local_c = 0x0;
      }
      else {
        uVar1 = *(undefined4 *)(iVar4 + 0xa);
        local_c = *(undefined2 *)((int)uVar1 + 0x8);
      }
      local_1e[0] = local_c;
      BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_1e,param_4,(char *)0x2,param_3);
      if (BVar2 != 0x0) {
        pass1_1008_5784((ulong *)CONCAT22(param_4,local_14),*(ulong *)(iVar4 + 0xa));
        do {
          puVar3 = local_14;
          pass1_1008_5b12(puVar3,param_4);
          uStack10 = CONCAT22(extraout_DX,puVar3);
          if ((extraout_DX | (uint)puVar3) == 0x0) {
            return;
          }
          BVar2 = pass1_1008_7c2a(param_2,*(char **)(puVar3 + 0x4),param_3);
          if (BVar2 == 0x0) break;
          local_6[0] = *(undefined2 *)((int)uStack10 + 0x8);
          BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_6,param_4,(char *)0x2,param_3);
          if (BVar2 == 0x0) break;
          local_2a[0] = *(undefined4 *)((int)uStack10 + 0xa);
          BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_2a,param_4,(char *)0x4,param_3);
          if (BVar2 == 0x0) break;
          local_6[0] = *(undefined2 *)((int)uStack10 + 0xe);
          BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_6,param_4,(char *)0x2,param_3);
        } while (BVar2 != 0x0);
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far
file_1008_bb5e(ulong param_1,ulong param_2,int param_3,uchar *param_4,uint16_t param_5,uint16_t param_6)

{
  code **ppcVar1;
  ushort uVar2;
  astruct_199 *iVar3;
  BOOL16 BVar3;
  uint uVar5;
  astruct_200 *uVar4;
  undefined *puVar6;
  ushort uVar7;
  uchar *extraout_DX;
  uchar *puVar8;
  ushort uVar9;
  ushort uVar10;
  uchar *extraout_DX_00;
  ushort extraout_DX_01;
  ushort uVar11;
  ushort uVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  astruct_200 *paStack286;
  undefined4 *puStack284;
  undefined local_118 [0x100];
  undefined2 local_18 [0x2];
  undefined2 local_14 [0x2];
  astruct_200 *local_10 [0x4];
  undefined4 local_8;
  
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  uVar11 = (ushort)param_2;
  uVar12 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar11,uVar12,0x16,param_5,param_6);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
  }
  else {
    iVar3 = (astruct_199 *)param_1;
    iVar3 = (astruct_199 *)&iVar3->field_0x22;
    uVar2 = (ushort)(param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)iVar3,0x0,uVar2,0x2,param_5);
    if ((BVar3 != 0x0) &&
       (uVar5 = read_file_1008_7dee(uVar11,uVar12,(ushort)local_10,0x0,param_6,0x2,param_5), uVar5 != 0x0)) {
      if (local_10[0] == (astruct_200 *)0x0) {
        return;
      }
      uVar14 = 0xc;
      mem_op_1000_179c(0xc,param_4,0x1000);
      if (((uint)param_4 | uVar5) == 0x0) {
        uVar5 = 0x0;
        puVar8 = (uchar *)0x0;
      }
      else {
        set_struct_1008_574a((astruct_21 *)CONCAT22(param_4,uVar5));
        puVar8 = extraout_DX;
      }
      *(uint *)&iVar3->field_0xa = uVar5;
      *(uchar **)((int)&iVar3->field_0xa + 0x2) = puVar8;
      paStack286 = (astruct_200 *)0x0;
      while( true ) {
        if (local_10[0] <= paStack286) {
          return;
        }
        uVar13 = 0x12;
        uVar4 = local_10[0];
        mem_op_1000_179c(0x12,puVar8,0x1000);
        if (((uint)puVar8 | (uint)uVar4) == 0x0) {
          uVar4 = (astruct_200 *)0x0;
          uVar9 = 0x0;
        }
        else {
          set_stuct_1008_b0bc((astruct_26 *)CONCAT22(puVar8,uVar4));
          uVar9 = extraout_DX_01;
        }
        puStack284 = (undefined4 *)CONCAT22(uVar9,uVar4);
        puVar6 = local_118;
        uVar10 = uVar9;
        read_file_1008_7c6e(uVar11,uVar12,(char *)CONCAT22(param_6,puVar6),0x1000);
        if ((((puVar6 == (undefined *)0x0) ||
             (BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)local_14,0x0,param_6,0x2,0x1000), BVar3 == 0x0)) ||
            (BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)&local_8,0x0,param_6,0x4,0x1000), BVar3 == 0x0)) ||
           (BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)local_18,0x0,param_6,0x2,0x1000), BVar3 == 0x0)) break;
        uVar7 = str_op_1008_60e8((char *)CONCAT22(param_6,local_118),uVar10);
        uVar4->field_0x4 = uVar7;
        uVar4->field_0x6 = uVar10;
        uVar4->field_0x8 = local_14[0];
        uVar4->field_0xa = local_8;
        uVar4->field_0xe = local_18[0];
        ppcVar1 = (code **)((int)*iVar3->field_0xa + 0x8);
        (**ppcVar1)();
        paStack286 = (astruct_200 *)&paStack286->field_0x1;
        puVar8 = extraout_DX_00;
      }
      if (puStack284 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack284;
        (**ppcVar1)(0x1000,uVar4,uVar9,0x1,uVar13,uVar14,puStack284);
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}

void __stdcall16far file_1008_7548(undefined4 param_1,long *param_2,HFILE16 param_3,ushort param_4)

{
  code **ppcVar1;
  ushort uVar2;
  BOOL16 BVar3;
  uint uVar4;
  ulong uVar5;
  uint uVar6;
  ushort uVar7;
  undefined4 local_1c;
  undefined2 local_18 [0x5];
  ulong uStack14;
  ulong uStack10;
  undefined4 local_6;
  
  local_6 = 0x0;
  uVar7 = (ushort)param_1;
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  BVar3 = read_file_1008_7dee(uVar7,uVar2,(ushort)&local_6,0x0,param_4,0x4,param_3);
  if (BVar3 == 0x0) {
    return;
  }
  if (local_6 != 0x0) {
    uVar5 = local_6;
    if (local_6 < 0xc8) {
      local_6._2_2_ = (uchar *)0x0;
      uVar5 = 0xc8;
    }
    uVar4 = (uint)uVar5;
    uStack10 = uVar5 & 0xffff | ZEXT24(local_6._2_2_) << 0x10;
    if (*param_2 == 0x0) {
      param_3 = 0x1000;
      mem_op_1000_179c(0x1e,local_6._2_2_,0x1000);
      uVar6 = (uint)local_6._2_2_ | uVar4;
      if (uVar6 == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_3 = 0x1020;
        struct_1020_c444((astruct_75 *)CONCAT22(local_6._2_2_,uVar4),0x64,uStack10);
        *(uint *)param_2 = uVar4;
        *(uint *)((int)param_2 + 0x2) = uVar6;
      }
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x24);
    (**ppcVar1)(param_3,*param_2);
    for (uStack14 = 0x0; uStack14 < local_6; uStack14 = uStack14 + 0x1) {
      BVar3 = read_file_1008_7dee(uVar7,uVar2,(ushort)&local_1c,0x0,param_4,0x4,param_3);
      if ((BVar3 == 0x0) ||
         (BVar3 = read_file_1008_7dee(uVar7,uVar2,(ushort)local_18,0x0,param_4,0x2,param_3), BVar3 == 0x0)) {
        ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x1c);
        (**ppcVar1)(param_3,(char)*param_2,(int)((ulong)*param_2 >> 0x10));
        return;
      }
      ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x28);
      (**ppcVar1)(param_3,(int)*param_2,(char)((ulong)*param_2 >> 0x10),local_18[0],(char)local_1c,
                  (int)((ulong)local_1c >> 0x10));
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x1c);
    (**ppcVar1)(param_3,(char)*param_2,(int)((ulong)*param_2 >> 0x10));
  }
  return;
}


void __stdcall16far file_1008_76e4(ulong param_1,long *param_2,ushort param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  uint uVar2;
  BOOL16 BVar3;
  undefined2 extraout_DX;
  ushort uVar4;
  undefined local_18 [0xe];
  ulong uStack10;
  ulong local_6;
  
  local_6 = 0x0;
  uVar4 = (ushort)(param_1 >> 0x10);
  uVar2 = read_file_1008_7dee((ushort)param_1,uVar4,(ushort)&local_6,0x0,param_4,0x4,param_3);
  if (uVar2 == 0x0) {
    return;
  }
  if (local_6 != 0x0) {
    if (*param_2 == 0x0) {
      param_3 = 0x1000;
      mem_op_1000_179c(0x18,(uchar *)param_5,0x1000);
      if ((param_5 | uVar2) == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_3 = 0x1030;
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_5,uVar2),0x5,local_6);
        *(uint *)param_2 = uVar2;
        *(undefined2 *)((int)param_2 + 0x2) = extraout_DX;
      }
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x14);
    (**ppcVar1)(param_3,(int)*param_2,(int)((ulong)*param_2 >> 0x10),local_6);
    for (uStack10 = 0x0; uStack10 < local_6; uStack10 = uStack10 + 0x1) {
      BVar3 = read_file_1008_7dee((ushort)param_1,uVar4,(ushort)local_18,0x0,param_4,0x4,param_3);
      if (BVar3 == 0x0) {
        return;
      }
      ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x18);
      (**ppcVar1)();
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x1c);
    (**ppcVar1)();
  }
  return;
}



undefined2 __stdcall16far file_1008_77cc(ulong param_1,long *param_2,uchar *param_3,HFILE16 param_4,ushort param_5)

{
  uint uVar1;
  BOOL16 BVar2;
  uint uVar3;
  ushort unaff_SI;
  ushort unaff_DI;
  ushort uVar4;
  ushort uVar5;
  undefined2 local_14 [0x2];
  undefined4 local_10 [0x2];
  uint uStack6;
  uint local_4;
  
  local_4 = 0x0;
  uVar4 = (ushort)param_1;
  uVar5 = (ushort)(param_1 >> 0x10);
  uVar1 = read_file_1008_7dee(uVar4,uVar5,(ushort)&local_4,0x0,param_5,0x2,param_4);
  if (uVar1 == 0x0) {
    return 0x0;
  }
  if (local_4 != 0x0) {
    if (*param_2 == 0x0) {
      param_4 = 0x1000;
      mem_op_1000_179c(0xa,param_3,0x1000);
      uVar3 = (uint)param_3 | uVar1;
      if (uVar3 == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_4 = 0x1020;
        pass1_1020_ba3e((long *)CONCAT22(param_3,uVar1),0x5,0x5,unaff_DI,unaff_SI);
        *(uint *)param_2 = uVar1;
        *(uint *)((int)param_2 + 0x2) = uVar3;
      }
    }
    for (uStack6 = 0x0; uStack6 < local_4; uStack6 = uStack6 + 0x1) {
      BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)local_14,0x0,param_5,0x2,param_4);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)local_10,0x0,param_5,0x4,param_4);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      param_4 = 0x1020;
      pass1_1020_bb8a((long *)*param_2,(uint)local_10[0],CONCAT22(local_14[0],(int)((ulong)local_10[0] >> 0x10)),
                      unaff_DI,param_5);
    }
  }
  return 0x1;
}


void __stdcall16far
pass1_1008_7898(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,HFILE16 param_5,ushort param_6)

{
  code **ppcVar1;
  BOOL16 BVar2;
  undefined2 extraout_DX;
  undefined2 uVar3;
  ushort uVar4;
  ushort uVar5;
  undefined2 local_26;
  undefined4 local_24 [0x3];
  undefined4 local_18;
  undefined2 local_14 [0x5];
  ulong uStack10;
  ulong uStack6;
  
  if (param_2 == (ulong *)0x0) {
    param_3 = 0x0;
    uVar3 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*param_2 + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_3);
  local_18 = CONCAT22(uVar3,param_3);
  uVar4 = (ushort)param_1;
  uVar5 = (ushort)(param_1 >> 0x10);
  BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_18,param_6,(char *)0x4,param_5);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return;
      }
      pass1_1020_c4a8((ulong)param_2,(ushort *)CONCAT22(param_6,local_14),(ulong *)CONCAT22(param_6,&local_18),
                      (int)uStack10,param_4,param_6);
      local_24[0] = local_18;
      BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)local_24,param_6,(char *)0x4,0x1020);
      if (BVar2 == 0x0) break;
      local_26 = local_14[0];
      BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_26,param_6,(char *)0x2,0x1020);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
      }
      uStack10 = uStack10 + 0x1;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



uint16_t __stdcall16far
write_to_file_1008_7954(undefined4 param_1,undefined4 *param_2,uint16_t param_3,HFILE16 param_4,uint16_t param_5)

{
  code **ppcVar1;
  BOOL16 BVar2;
  ulong uVar3;
  uint16_t extraout_DX;
  uint16_t uVar4;
  uint16_t extraout_DX_00;
  ushort uVar5;
  uint16_t local_20;
  uint16_t uStack30;
  uint16_t local_18;
  uint16_t uStack22;
  ulong uStack10;
  ulong uStack6;
  
  if (param_2 == (undefined4 *)0x0) {
    param_3 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*param_2 + 0x10);
    (**ppcVar1)(param_4,param_2);
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_3);
  uVar5 = (ushort)((ulong)param_1 >> 0x10);
  local_18 = param_3;
  uStack22 = uVar4;
  BVar2 = write_to_file_1008_7e1c((ushort)param_1,uVar5,(ushort)&local_18,param_5,(char *)0x4,param_4);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return uVar4;
      }
      ppcVar1 = (code **)((int)*param_2 + 0x4);
      uVar3 = uStack6;
      (**ppcVar1)();
      local_20 = (uint16_t)uVar3;
      uVar4 = extraout_DX_00;
      uStack30 = extraout_DX_00;
      local_18 = local_20;
      uStack22 = extraout_DX_00;
      BVar2 = write_to_file_1008_7e1c((ushort)param_1,uVar5,(ushort)&local_20,param_5,(char *)0x4,param_4);
      if (BVar2 == 0x0) break;
      uStack10 = uStack10 + 0x1;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return uVar4;
}


void __stdcall16far pass1_1008_79f0(ulong param_1,long param_2,HFILE16 param_3,uint16_t param_4)

{
  uint16_t uVar1;
  undefined2 uVar2;
  undefined2 uStack4;
  
  if (param_2 == 0x0) {
    uVar1 = 0x0;
    uStack4 = 0x0;
  }
  else {
    uVar2 = (undefined2)((ulong)param_2 >> 0x10);
    uVar1 = *(uint16_t *)((int)param_2 + 0x4);
    uStack4 = *(undefined2 *)((int)param_2 + 0x6);
  }
  write_to_file_1008_7954(param_1,(undefined4 *)CONCAT22(uStack4,uVar1),uVar1,param_3,param_4);
  return;
}



void __stdcall16far write_to_file_1008_7a22(undefined4 param_1,long param_2,HFILE16 param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined4 local_24 [0x2];
  uint local_1c [0x5];
  uint local_12;
  undefined4 local_10;
  uint uStack10;
  uint uStack6;
  uint uStack4;
  
  if (param_2 == 0x0) {
    uStack4 = 0x0;
  }
  else {
    uStack4 = *(uint *)((int)param_2 + 0x4);
  }
  local_12 = uStack4;
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)&local_12,param_4,(char *)0x2,param_3);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  else {
    uStack6 = 0x0;
    while( true ) {
      if (uStack4 <= uStack6) {
        return;
      }
      pass1_1020_bb16((ulong *)param_2,(ulong *)CONCAT22(param_4,&local_10),(ushort *)CONCAT22(param_4,&local_12),
                      uStack6);
      uStack10 = local_12;
      local_1c[0] = local_12;
      BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_1c,param_4,(char *)0x2,0x1020);
      if (BVar1 == 0x0) break;
      local_24[0] = local_10;
      BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_24,param_4,(char *)0x4,0x1020);
      if (BVar1 == 0x0) {
        return;
      }
      uStack6 = uStack6 + 0x1;
    }
  }
  return;
}


ushort __stdcall16far pass1_1008_7ad4(ulong param_1,long *param_2,ushort param_3,HFILE16 param_4,ushort param_5)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined2 local_14 [0x2];
  undefined4 local_10 [0x2];
  uint uStack6;
  uint local_4;
  
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)&local_4,0x0,param_5,0x2,param_4);
  if (BVar1 != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      if (local_4 <= uStack6) {
        return 0x1;
      }
      BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_14,0x0,param_5,0x2,param_4);
      if ((BVar1 == 0x0) ||
         (BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_10,0x0,param_5,0x4,param_4), BVar1 == 0x0)) break;
      param_4 = 0x1020;
      pass1_1020_bb8a(param_2,(uint)local_10[0],CONCAT22(local_14[0],(int)((ulong)local_10[0] >> 0x10)),param_3,param_5)
      ;
      uStack6 = uStack6 + 0x1;
    }
  }
  return 0x0;
}



undefined2 __stdcall16far write_to_file_1008_7b4c(undefined4 param_1,ushort *param_2,HFILE16 param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined2 local_12 [0x3];
  undefined2 local_c [0x2];
  undefined2 local_8;
  undefined2 local_6;
  undefined2 local_4;
  
  pass1_1008_3eb4(param_2,(ushort *)CONCAT22(param_4,&local_8),(ushort *)CONCAT22(param_4,&local_6),
                  (ushort *)CONCAT22(param_4,&local_4));
  local_12[0] = local_4;
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_12,param_4,(char *)0x2,param_3);
  if (BVar1 != 0x0) {
    local_c[0] = local_6;
    BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_c,param_4,(char *)0x2,param_3);
    if (BVar1 != 0x0) {
      local_c[0] = local_8;
      BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_c,param_4,(char *)0x2,param_3);
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 __stdcall16far read_file_1008_7bc8(ulong param_1,ushort *param_2,HFILE16 param_3,ushort param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  ushort local_8;
  undefined4 local_6;
  
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  BVar1 = read_file_1008_7dee(uVar2,uVar3,(int)&local_6 + 0x2,0x0,param_4,0x2,param_3);
  if (BVar1 != 0x0) {
    BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)&local_6,0x0,param_4,0x2,param_3);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)&local_8,0x0,param_4,0x2,param_3);
      if (BVar1 != 0x0) {
        pass1_1008_3e76(param_2,local_8,(ushort)local_6,(ushort)((ulong)local_6 >> 0x10));
        return 0x1;
      }
    }
  }
  return 0x0;
}


void __stdcall16far read_file_1008_7c6e(ushort param_1,ushort param_2,char *param_3,HFILE16 param_4)

{
  char *pcVar1;
  char local_c [0xa];
  
  while( true ) {
    pcVar1 = param_3;
    WIN16_hread(param_4,0x1,ZEXT24(local_c) << 0x10);
    if (local_c[0] == '\0') break;
    param_3 = (char *)((ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x1));
    *pcVar1 = local_c[0];
    param_4 = (HFILE16)s_tile2_bmp_1050_1538;
  }
  *param_3 = '\0';
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far write_to_file_1008_7cac(undefined4 param_1,uint16_t param_2)

{
  uint uVar1;
  BOOL16 BVar2;
  undefined2 unaff_ES;
  undefined1 in_AF;
  uchar local_c [0xa];
  
  sys_1000_3f9c(local_c,(uchar *)param_2,0x340,(ushort)&USHORT_1050_1050,(ushort)_PTR_s_dcbSC_1050_0336_1050_033c,
                &stack0xfffe,unaff_ES,0x1000,param_2,in_AF);
  uVar1 = str_op_1000_3da4((char *)CONCAT22(param_2,local_c));
  BVar2 = write_to_file_1008_7e1c
                    ((ushort)param_1,(ushort)((ulong)param_1 >> 0x10),(ushort)local_c,param_2,(char *)(ulong)uVar1,
                     0x1000);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return BVar2;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
read_file_1008_7cfe(undefined2 param_1,undefined2 param_2,undefined2 param_3,uint16_t param_4,uint16_t param_5)

{
  bool bVar1;
  ushort uVar2;
  uchar in_AF;
  long lVar3;
  ushort in_stack_0000fbd2;
  uint16_t in_stack_0000fbd4;
  undefined4 uStack1040;
  char local_406 [0x400];
  undefined4 uStack6;
  
  uStack6 = 0x0;
  bVar1 = false;
  do {
    _llseek16(param_4,uStack6 << 0x10,(INT16)((ulong)uStack6 >> 0x10));
    param_4 = (uint16_t)s_tile2_bmp_1050_1538;
    lVar3 = WIN16_hread((HFILE16)s_tile2_bmp_1050_1538,0x400,ZEXT24(local_406) << 0x10);
    for (uStack1040 = 0x0; uStack1040 < lVar3; uStack1040 = uStack1040 + 0x1) {
      if (local_406[(uint)uStack1040] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
        if (!bVar1) {
          bVar1 = true;
          uStack6 = CONCAT22((int)((ulong)uStack6 >> 0x10) + uStack1040._2_2_ +
                             (uint)CARRY2((uint)uStack6,(uint)uStack1040),(uint)uStack6 + (uint)uStack1040);
          break;
        }
        bVar1 = false;
        uVar2 = pass1_1008_7e4a((ushort)((ulong)_PTR_s_dcbSC_1050_0336_1050_033c >> 0x10),(uchar *)param_5,in_AF,
                                (char *)CONCAT22(param_5,local_406 + (uint)uStack1040),in_stack_0000fbd2,
                                in_stack_0000fbd4);
        if (uVar2 != 0x0) {
          lVar3 = uStack1040 + uStack6 + 0x7;
          _llseek16((HFILE16)s_tile2_bmp_1050_1538,lVar3 * 0x10000,(INT16)((ulong)lVar3 >> 0x10));
          return;
        }
      }
    }
    if (!bVar1) {
      if (lVar3 < 0x400) {
        return;
      }
      uStack6._0_2_ = CONCAT11(uStack6._1_1_ + 0x4,(undefined)uStack6);
      uStack6 = CONCAT22((int)((ulong)uStack6 >> 0x10) + (uint)(0xfb < uStack6._1_1_),(uint)uStack6);
    }
  } while( true );
}



BOOL16 __stdcall16far
read_file_1008_7dee(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,SEGPTR param_6,
                   HFILE16 param_7)

{
  long lVar1;
  
  lVar1 = WIN16_hread(param_7,param_6,CONCAT22(param_3,param_4));
  if (lVar1 != CONCAT22(param_4,param_6)) {
    return 0x0;
  }
  return 0x1;
}



BOOL16 __stdcall16far
write_to_file_1008_7e1c
          (ushort param_1,ushort param_2,ushort param_3,ushort param_4,char *buf_to_write,HFILE16 file_handle)

{
  char *pcVar1;
  
  pcVar1 = (char *)_hwrite16(file_handle,(LPCSTR)buf_to_write,CONCAT22(param_3,(int)((ulong)buf_to_write >> 0x10)));
  if (pcVar1 != buf_to_write) {
    return 0x0;
  }
  return 0x1;
}


void __stdcall16far close_file_1008_6dd0(undefined4 *param_1,HFILE16 param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x4) != -0x1) {
    _lclose16(param_2);
    *(undefined2 *)((int)param_1 + 0x4) = 0xffff;
  }
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



BOOL16 __stdcall16far file_fn_1008_6e02(uint32_t *param_1,LPCSTR in_string,uint16_t param_3)

{
  int var1;
  BOOL16 var2;
  uchar *extraout_DX;
  int unaff_DI;
  ushort uVar1;
  undefined local_4 [0x2];
  
  PTR_LOOP_1050_0310 = (undefined *)0x0;
  var1 = write_to_file_1008_70a6((undefined4 *)param_1,in_string);
  if (var1 != 0x0) {
    uVar1 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1008_72a8();
    var1 = pass1_1008_7006((ushort)param_1,uVar1,CONCAT22(param_3,local_4),extraout_DX,unaff_DI,param_3);
    if ((var1 != 0x0) && (var1 = file_fn_1008_6eee(param_1,local_4,param_3), var1 != 0x0)) {
      var2 = file_fn_1008_726c((ushort)param_1,uVar1,(HFILE16)in_string);
      if (var2 == 0x0) {
        return 0x0;
      }
      return 0x1;
    }
    _lclose16((HFILE16)in_string);
  }
  return 0x0;
}



BOOL16 __stdcall16far read_file_1008_6e78(uint32_t param_1,uint16_t param_2,LPCSTR in_string,uint16_t param_4)

{
  BOOL16 b_var1;
  int i_var2;
  undefined *var3;
  uchar *extraout_DX;
  int unaff_DI;
  undefined local_4 [0x2];
  
  PTR_LOOP_1050_0310 = (undefined *)0x0;
  b_var1 = read_file_1008_7146(param_1,param_2,in_string,param_4);
  if (b_var1 != 0x0) {
    pass1_1008_72a8();
    i_var2 = pass1_1008_7056(param_1,param_2,CONCAT22(param_4,local_4),extraout_DX,unaff_DI,param_4);
    if (i_var2 != 0x0) {
      var3 = local_4;
      read_file_1008_6f7a(param_1,param_2,CONCAT22(param_4,var3),param_4);
      if (var3 != (undefined *)0x0) {
        b_var1 = file_fn_1008_726c(param_1,param_2,(HFILE16)in_string);
        if (b_var1 == 0x0) {
          return 0x0;
        }
        return 0x1;
      }
    }
    _lclose16((HFILE16)in_string);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_fn_1008_6eee(undefined2 param_1,undefined2 param_2,ulong param_3)

{
  BOOL16 BVar1;
  ushort uVar2;
  uchar *in_DX;
  ushort unaff_SS;
  undefined4 uVar3;
  undefined local_e [0x4];
  undefined4 uStack10;
  undefined4 *puStack6;
  
  puStack6 = (undefined4 *)*_PTR_LOOP_1050_5748;
  uStack10 = *puStack6;
  pass1_1020_a43e(unaff_SS,in_DX,(ushort *)CONCAT22(unaff_SS,local_e));
  BVar1 = pass1_1028_d7a0((ushort)uStack10,(ushort)((ulong)uStack10 >> 0x10),param_3,unaff_SS);
  if (BVar1 != 0x0) {
    BVar1 = pass1_1030_5c1a(_PTR_LOOP_1050_5736,param_3,unaff_SS);
    if (BVar1 != 0x0) {
      uVar3 = write_to_file_1028_dce2(_PTR_LOOP_1050_65e2,param_3,unaff_SS);
      if ((int)((ulong)uVar3 >> 0x10) != 0x0) {
        uVar2 = pass1_1038_7b20(_PTR_LOOP_1050_5a64,param_3,unaff_SS);
        if (uVar2 != 0x0) {
          BVar1 = pass1_1020_a644((ushort)local_e,unaff_SS,param_3,unaff_SS);
          if (BVar1 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far read_file_1008_6f7a(undefined2 param_1,undefined2 param_2,undefined4 param_3,uint16_t param_4)

{
  uint16_t var5;
  int i_var3;
  BOOL16 b_var4;
  uchar *in_DX;
  uint16_t uVar1;
  ushort *puVar2;
  undefined local_e [0x4];
  undefined4 uStack10;
  undefined4 *puStack6;
  
  puStack6 = (undefined4 *)*_PTR_LOOP_1050_5748;
  uStack10 = *puStack6;
  puVar2 = pass1_1020_a43e(param_4,in_DX,(ushort *)CONCAT22(param_4,local_e));
  uVar1 = (uint16_t)((ulong)puVar2 >> 0x10);
  var5 = read_file_1028_d7ba((int)uStack10,(int)((ulong)uStack10 >> 0x10),param_3,param_4,(uint16_t)puVar2);
  if (var5 != 0x0) {
    var5 = read_file_1030_5c52(_PTR_LOOP_1050_5736,param_3,var5,param_4);
    if (var5 != 0x0) {
      read_file_1028_def2(_PTR_LOOP_1050_65e2,param_3,param_4,var5);
      if (var5 != 0x0) {
        i_var3 = read_file_1038_7c02(_PTR_LOOP_1050_5a64,param_3,var5,uVar1);
        if (i_var3 != 0x0) {
          b_var4 = read_file_1020_a65e(CONCAT22(param_4,local_e),param_3,param_4,(uint16_t)local_e);
          if (b_var4 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}

undefined2 __stdcall16far write_to_file_1008_70a6(undefined4 *param_1,LPCSTR param_2)

{
  HFILE16 HVar1;
  int iVar2;
  undefined2 uVar3;
  LPCSTR pCVar4;
  undefined2 unaff_SS;
  undefined in_AF;
  long lVar5;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  pCVar4 = param_2;
  if (*(int *)(iVar2 + 0x4) != -0x1) {
    pCVar4 = (LPCSTR)s_tile2_bmp_1050_1538;
    _lclose16((HFILE16)param_2);
    *(undefined2 *)(iVar2 + 0x4) = 0xffff;
  }
  HVar1 = _lcreat16(pCVar4,0x0);
  *(HFILE16 *)(iVar2 + 0x4) = HVar1;
  if (HVar1 == 0xffff) {
    PTR_LOOP_1050_0310 = (undefined *)0x6cf;
  }
  else {
    PTR_LOOP_1050_0312 = (undefined *)&DAT_1050_0004;
    sys_1000_3f9c((uchar *)0x65a0,(uchar *)&USHORT_1050_1050,(ushort)_PTR_s_SC_03d_1050_0314_1050_031c,
                  (ushort)((ulong)_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),0x4,&stack0xfffe,uVar3,0x1000,unaff_SS,
                  in_AF);
    pCVar4 = (LPCSTR)str_op_1000_3da4((char *)0x105065a0);
    lVar5 = _hwrite16(0x1000,pCVar4,CONCAT22(0x65a0,(int)pCVar4 >> 0xf));
    if (lVar5 == (int)pCVar4) {
      return 0x1;
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



BOOL16 __stdcall16far read_file_1008_7146(int32_t param_1,uint16_t param_2,LPCSTR param_3,uint16_t param_4)

{
  HFILE16 HVar1;
  int iVar2;
  LPCSTR path;
  
  path = param_3;
  if (*(int *)(param_1 + 0x4) != -0x1) {
    path = (LPCSTR)s_tile2_bmp_1050_1538;
    _lclose16((HFILE16)param_3);
    *(undefined2 *)(param_1 + 0x4) = 0xffff;
  }
  HVar1 = _lopen16(path,0x0);
  *(HFILE16 *)(param_1 + 0x4) = HVar1;
  if (HVar1 == 0xffff) {
    PTR_LOOP_1050_0310 = (undefined *)0x6cf;
  }
  else {
    iVar2 = read_file_1008_71a0(CONCAT22(param_2,param_1),param_4);
    if (iVar2 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far read_file_1008_71a0(undefined4 param_1,uint16_t param_2)

{
  uint buffer;
  uint uVar1;
  undefined in_AF;
  long lVar2;
  int iStack26;
  int iStack24;
  int iStack22;
  char local_e [0x9];
  undefined uStack5;
  undefined2 uStack4;
  
  uStack4 = 0x1;
  buffer = str_op_1000_3da4((char *)0x105065a0);
  iStack22 = 0x0;
  lVar2 = WIN16_hread(0x1000,buffer,CONCAT22(local_e,(int)buffer >> 0xf));
  uVar1 = (uint)lVar2;
  if ((int)buffer < lVar2) {
    uVar1 = buffer;
  }
  iStack24 = uVar1 - 0x2;
  if (iStack24 < 0x0) {
    iStack24 = 0x0;
  }
  iStack26 = 0x2;
  while (iStack24 != 0x0) {
    iStack22 = iStack22 * 0xa + local_e[iStack26] + -0x30;
    iStack26 = iStack26 + 0x1;
    iStack24 = iStack24 + -0x1;
  }
  if (iStack22 == 0x1) {
    PTR_LOOP_1050_0312 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  }
  else {
    if (iStack22 == 0x4) {
      PTR_LOOP_1050_0312 = (undefined *)&DAT_1050_0004;
    }
    else {
      uStack5 = 0x0;
      PTR_LOOP_1050_0312 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
      uStack4 = 0x0;
    }
  }
  sys_1000_3f9c((uchar *)0x65a0,(uchar *)&USHORT_1050_1050,(ushort)_PTR_s_SC_03d_1050_0314_1050_031c,
                (ushort)((ulong)_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),(ushort)PTR_LOOP_1050_0312,&stack0xfffe,
                (int)((ulong)param_1 >> 0x10),0x1000,param_2,in_AF);
  return uStack4;
}



BOOL16 __stdcall16far file_fn_1008_726c(uint32_t param_1,uint16_t param_2,HFILE16 file_handle)

{
  HFILE16 HVar1;
  
  if (*(int *)(param_1 + 0x4) != -0x1) {
    HVar1 = _lclose16(file_handle);
    if (HVar1 == 0xffff) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d1;
      return 0x0;
    }
    *(undefined2 *)(param_1 + 0x4) = 0xffff;
    PTR_LOOP_1050_0310 = (undefined *)0x0;
  }
  return 0x1;
}


