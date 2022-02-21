
void __stdcall16far send_msg_1038_ed8a(ushort param_1, ulong param_2, ulong param_3, HWND16 param_4)

{
    uint       uVar1;
    undefined2 uVar2;
    uint       uVar3;
    ushort     uVar4;
    uchar     *in_DX;
    uint       uVar5;
    uchar     *puVar6;
    int        unaff_DI;
    ushort     unaff_SS;
    ushort    *puVar7;
    ulong      uVar8;

    if(param_3._2_2_ != 0x1c8)
    {
        if(param_3._2_2_ == 0x1c9)
        {
            puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, unaff_SS, in_DX, unaff_DI);
            uVar2  = (undefined2)((ulong)puVar7 >> 0x10);
            uVar1  = *(uint *)((int)puVar7 + 0x20);
            uVar5  = *(uint *)((int)puVar7 + 0x22);
            uVar3  = uVar5 | uVar1;
            if(uVar3 == 0x0)
            {
                return;
            }
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, uVar5);
            puVar6 = (uchar *)(uVar5 | uVar3);
            if(puVar6 == (uchar *)0x0)
            {
                return;
            }
            uVar4  = pass1_1030_5b00(CONCAT22(uVar5, uVar3));
            puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, uVar4, unaff_SS, puVar6, unaff_DI);
            if(puVar7 == (ushort *)0x0)
            {
                return;
            }
            param_4 = 0x1018;
            uVar8   = pass1_1018_0ad4((ulong)puVar7);
            if(uVar8 == 0x0)
            {
                return;
            }
            param_3._2_2_ = 0x72;
        }
        else
        {
            if(param_3._2_2_ != 0x1ca)
            {
                post_win_msg_1040_7b3c((ulong *)CONCAT22((int)param_2, param_1),
                                       (ushort)(param_2 >> 0x10),
                                       (ushort)param_3,
                                       param_3._2_2_,
                                       (int)&PTR_LOOP_1050_1040);
                return;
            }
        }
    }
    SendMessage16(param_4, 0x0, 0x0, CONCAT22(0x111, param_3._2_2_));
    return;
}


void __stdcall16far post_win_msg_1040_0d5e(ushort param_1, ushort param_2, int param_3, HWND16 param_4)

{
    if(param_3 != 0x0)
    {
        PostMessage16(param_4, 0x0, 0x0, 0x1110001);
    }
    return;
}


void __stdcall16far unk_win_sys_op_1038_da68(
  int param_1, ushort param_2, ushort param_3, ulong param_4, WNDCLASS16 *param_5, uchar *param_6)

{
    code      **ppcVar1;
    uint        uVar2;
    uchar      *puVar3;
    uchar      *extraout_DX;
    undefined2  in_BX;
    int         unaff_DI;
    ushort      unaff_CS;
    undefined2  uVar4;
    ulong       uVar5;
    undefined4  uVar6;
    int         iVar7;
    undefined   local_16[0x4];
    uint        uStack18;
    uchar      *puStack16;
    astruct_43 *paStack14;
    undefined2  uStack10;
    ushort      uStack8;
    int         iStack6;
    int         iStack4;

    if(param_3 == 0x204)
    {
        pass1_1038_de20(CONCAT22(param_2, param_1), 0x204, (ushort)param_4, param_4._2_2_, param_6, in_BX, param_5);
        return;
    }
    iStack6 = 0x0;
    iStack4 = 0x0;
    uStack8 = 0x0;
    if(param_4._2_2_ == 0x121)
    {
        uStack10 = 0x1;
        iStack6  = 0x0;
        iStack4  = 0x6ec;
        uStack8  = 0x15;
    }
    else
    {
        if(param_4 < 0x1220000)
        {
            uVar2 = param_4._2_2_ - 0x100;
            if(uVar2 == 0x0)
            {
                param_4._2_2_ = uVar2;
                if(*(int *)(param_1 + 0x8e) == 0x0)
                {
                    pass1_1010_1ea6((ulong)_PTR_LOOP_1050_02a0, CONCAT22(param_2, param_1), (ushort)param_5);
                    *(undefined2 *)(param_1 + 0x90) = 0x0;
                }
                iStack4 = 0x72c;
                uStack8 = 0x48;
            }
            else
            {
                if(param_4._2_2_ - 0x11c == 0x0)
                {
                    param_4._2_2_ = param_4._2_2_ - 0x11c;
                    pass1_1038_df86(CONCAT22(param_2, param_1), param_6, unaff_DI, (ushort)param_5);
                }
                else
                {
                    if(param_4._2_2_ != 0x11d)
                    {
                        if(param_4._2_2_ == 0x11e)
                        {
                            iVar7 = 0x1d;
                        }
                        else
                        {
                            if(param_4._2_2_ != 0x120)
                            {
                            LAB_1038_dc20:
                                post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2, param_1),
                                                       param_3,
                                                       (ushort)param_4,
                                                       param_4._2_2_,
                                                       (int)&PTR_LOOP_1050_1040);
                                return;
                            }
                            iVar7 = 0x1c;
                        }
                        goto LAB_1038_db1c;
                    }
                    uVar5         = pass1_1038_df5c(CONCAT22(param_2, param_1), param_6, param_5);
                    param_6       = (uchar *)(uVar5 >> 0x10);
                    param_4._2_2_ = (uint)uVar5;
                }
            }
        }
        else
        {
            if(param_4._2_2_ == 0x122)
            {
                iVar7 = 0x14;
            }
            else
            {
                if(param_4._2_2_ != 0x123)
                {
                    if(param_4._2_2_ - 0x125 == 0x0)
                    {
                        ppcVar1       = (code **)((int)*_PTR_LOOP_1050_02a0 + 0x4);
                        param_4._2_2_ = param_4._2_2_ - 0x125;
                        (**ppcVar1)();
                        *(undefined2 *)(param_1 + 0x90) = 0x1;
                        param_6                         = extraout_DX;
                        win_1008_5c5c(param_5, param_4._2_2_, (ushort)extraout_DX, (ulong)_PTR_LOOP_1050_02a0, 0x1db);
                        *(undefined2 *)(param_1 + 0x8e) = 0x100;
                    }
                    else
                    {
                        if(param_4._2_2_ == 0x126)
                        {
                            *(undefined2 *)(param_1 + 0x8e) = 0x0;
                            win_1008_5c7c((ulong)_PTR_LOOP_1050_02a0, 0xcb0001, param_5, 0x0, (ushort)param_6);
                            paStack14     = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1f8, (ushort)param_5);
                            param_6       = (uchar *)((ulong)paStack14 >> 0x10);
                            param_4._2_2_ = WinHelp16(0x1010, (LPCSTR)0x0, 0x0, CONCAT22((int)paStack14, 0x3));
                        }
                        else
                        {
                            if(param_4._2_2_ - 0x127 != 0x0)
                                goto LAB_1038_dc20;
                            param_4._2_2_ = param_4._2_2_ - 0x127;
                            post_win_msg_1038_dcb0(CONCAT22(param_2, param_1), 0x0, param_6, param_5);
                        }
                    }
                    goto LAB_1038_dac3;
                }
                iVar7 = 0x28;
            }
        LAB_1038_db1c:
            uVar6         = pass1_1038_af40(_PTR_LOOP_1050_5b7c,
                                    *(ushort *)(param_1 + 0x8),
                                    iVar7,
                                    (ushort)param_6,
                                    param_1,
                                    unaff_CS,
                                    (ushort)param_5);
            param_6       = (uchar *)((ulong)uVar6 >> 0x10);
            param_4._2_2_ = (uint)uVar6;
        }
    }
LAB_1038_dac3:
    if(iStack4 == 0x0)
    {
        return;
    }
    if(iStack6 == 0x0)
    {
        mem_op_1000_179c(0xb4, param_6, 0x1000);
        puVar3    = (uchar *)((uint)param_6 | param_4._2_2_);
        uStack18  = param_4._2_2_;
        puStack16 = param_6;
        if(puVar3 != (uchar *)0x0)
        {
            uVar4 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar7 = string_1040_8520(
              (astruct_57 *)CONCAT13((char)((uint)param_6 >> 0x8), CONCAT12((char)param_6, param_4._2_2_)),
              *(ushort *)(param_1 + 0x6),
              0x0,
              0x2,
              0x634,
              iStack4,
              puVar3,
              (ushort)param_5);
            goto LAB_1038_dc37;
        }
    }
    else
    {
        mem_op_1000_179c(0xb4, param_6, 0x1000);
        puVar3    = (uchar *)((uint)param_6 | param_4._2_2_);
        uStack18  = param_4._2_2_;
        puStack16 = param_6;
        if(puVar3 != (uchar *)0x0)
        {
            uVar4 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar7 = string_1040_8520((astruct_57 *)CONCAT22(param_6, param_4._2_2_),
                                     *(ushort *)(param_1 + 0x6),
                                     0x0,
                                     0x3,
                                     0x634,
                                     iStack4,
                                     puVar3,
                                     (ushort)param_5);
            goto LAB_1038_dc37;
        }
    }
    uVar4  = 0x1000;
    iVar7  = 0x0;
    puVar3 = (uchar *)0x0;
LAB_1038_dc37:
    paStack14 = (astruct_43 *)CONCAT22(puVar3, iVar7);
    if(uStack8 == 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)paStack14 + 0x74);
        (**ppcVar1)(uVar4, iVar7, puVar3);
    }
    else
    {
        pass1_1008_941a((ushort *)CONCAT22(param_5, local_16), 0x1, uStack8);
        ppcVar1 = (code **)((int)*(undefined4 *)paStack14 + 0x6c);
        (**ppcVar1)(0x1008, (char)paStack14, (int)((ulong)paStack14 >> 0x10), local_16, param_5);
    }
    return;
}


void __stdcall16far post_win_msg_1038_dcb0(ulong param_1, uint param_2, uchar *param_3, ushort param_4)

{
    code      **ppcVar1;
    int         iVar2;
    int         iVar3;
    uchar      *puVar4;
    uchar      *extraout_DX;
    int         unaff_DI;
    undefined2  uVar5;
    undefined2  uVar6;
    ushort      uVar7;
    undefined2  uVar8;
    undefined   uVar9;
    undefined   uVar10;
    undefined4  local_18;
    undefined   local_14[0x4];
    undefined4  uStack16;
    int         iStack12;
    undefined   local_a[0x4];
    undefined4 *puStack6;

    mem_op_1000_179c(0xb4, param_3, 0x1000);
    puVar4         = (uchar *)((uint)param_3 | param_2);
    iVar3          = (int)param_1;
    uVar5          = (undefined2)(param_1 >> 0x10);
    uStack16._0_2_ = param_2;
    uStack16._2_2_ = param_3;
    if(puVar4 == (uchar *)0x0)
    {
        iVar2  = 0x0;
        puVar4 = (uchar *)0x0;
    }
    else
    {
        iVar2 = string_1040_8520(
          (astruct_57 *)CONCAT22(param_3, param_2), *(ushort *)(iVar3 + 0x6), 0x4, 0x3, 0x634, 0x726, puVar4, param_4);
    }
    puStack6 = (undefined4 *)CONCAT22(puVar4, iVar2);
    pass1_1008_941a((ushort *)CONCAT22(param_4, local_a), 0x1, 0x49);
    ppcVar1  = (code **)((int)*puStack6 + 0x6c);
    uStack16 = (astruct_57 *)(**ppcVar1)(0x1008, (int)puStack6, (int)((ulong)puStack6 >> 0x10), local_a, param_4);
    puVar4   = (uchar *)((ulong)uStack16 >> 0x10);
    iStack12 = (int)uStack16;
    if(iStack12 == 0x6)
    {
        mem_op_1000_179c(0xb4, puVar4, 0x1000);
        puVar4 = (uchar *)((uint)((ulong)uStack16 >> 0x10) | (uint)uStack16);
        if(uStack16 == (astruct_57 *)0x0)
        {
            iVar3  = 0x0;
            puVar4 = (uchar *)0x0;
        }
        else
        {
            iVar3 = string_1040_8520(uStack16, *(ushort *)(iVar3 + 0x6), 0x0, 0x2, 0x634, 0x728, puVar4, param_4);
        }
        puStack6 = (undefined4 *)CONCAT22(puVar4, iVar3);
        pass1_1008_941a((ushort *)CONCAT22(param_4, local_14), 0x1, 0x4a);
        ppcVar1 = (code **)((int)*puStack6 + 0x6c);
        (**ppcVar1)(0x1008, (int)puStack6, (int)((ulong)puStack6 >> 0x10), local_14);
        uVar9    = 0x0;
        uVar10   = 0x0;
        iVar2    = 0x15;
        uVar7    = 0x1;
        uVar8    = 0x0;
        uVar6    = 0x0;
        iVar3    = 0x0;
        uVar5    = 0x0;
        local_18 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, extraout_DX, unaff_DI);
        post_win_msg_1008_a0e4(local_18,
                               CONCAT22(uVar6, uVar5),
                               iVar3,
                               uVar7,
                               CONCAT13(uVar10, CONCAT12(uVar9, uVar8)),
                               iVar2,
                               0x1008,
                               param_4);
        PostMessage16(0x1008, 0x0, 0x0, 0x11100fc);
        return;
    }
    mem_op_1000_179c(0xb4, puVar4, 0x1000);
    puVar4 = (uchar *)((uint)((ulong)uStack16 >> 0x10) | (uint)uStack16);
    if(uStack16 == (astruct_57 *)0x0)
    {
        iVar3  = 0x0;
        puVar4 = (uchar *)0x0;
    }
    else
    {
        iVar3 = string_1040_8520(uStack16, *(ushort *)(iVar3 + 0x6), 0x0, 0x2, 0x634, 0x729, puVar4, param_4);
    }
    puStack6 = (undefined4 *)CONCAT22(puVar4, iVar3);
    pass1_1008_941a((ushort *)CONCAT22(param_4, &local_18), 0x1, 0x4b);
    ppcVar1 = (code **)((int)*puStack6 + 0x6c);
    (**ppcVar1)(0x1008, (int)puStack6, (int)((ulong)puStack6 >> 0x10), &local_18);
    return;
}


void __stdcall16far destroy_win_1038_e1dc(UINT16 param_1, UINT16 param_2, int param_3, HWND16 param_4)

{
    UINT16 UVar1;
    LPARAM lparam;

    if(param_3 != 0x0)
    {
        UVar1 = IsDlgButtonChecked(param_4, 0x1807);
        if(UVar1 == 0x0)
        {
            param_4 = (HWND16)s_tile2_bmp_1050_1538;
            UVar1   = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0x1806);
            if(UVar1 == 0x0)
                goto LAB_1038_e229;
            lparam = 0x1110130;
        }
        else
        {
            lparam = 0x111012f;
        }
        param_4 = (HWND16)s_tile2_bmp_1050_1538;
        SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, lparam);
    }
LAB_1038_e229:
    DestroyWindow16(param_4);
    return;
}


void __stdcall16far
pass1_1038_e4bc(ushort param_1, ulong param_2, ulong param_3, uchar *param_4, int param_5, ushort param_6)

{
    code      **ppcVar1;
    undefined4  uVar2;
    uint        uVar3;
    uint        uVar4;
    uint        uVar5;
    uchar      *puVar6;
    uint        extraout_DX;
    uchar      *extraout_DX_00;
    uchar      *puVar7;
    code      **ppcVar8;
    ulong      *puVar9;
    ushort     *puVar10;
    undefined2  uVar11;
    undefined   uVar12;
    undefined   uVar13;
    undefined2  uVar14;
    undefined2  uVar15;
    undefined2  uVar16;
    undefined4 *puStack22;

    if(param_3._2_2_ == 0x1c4)
    {
        puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, param_4, param_5);
        uVar14  = (undefined2)((ulong)puVar10 >> 0x10);
        uVar4   = *(uint *)((int)puVar10 + 0x24);
        uVar5   = *(uint *)((int)puVar10 + 0x26);
        uVar3   = uVar5 | uVar4;
        if(uVar3 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, uVar5);
            if((uVar5 | uVar3) != 0x0)
            {
                puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x20);
                puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
                uVar4  = (uint)puVar9;
                pass1_1038_4e78(uVar4, puVar6, CONCAT22(uVar5, uVar3), puVar9);
                puStack22 = (undefined4 *)CONCAT22(puVar6, uVar4);
                uVar2     = *puStack22;
                ppcVar8   = (code **)uVar2;
                ppcVar1   = ppcVar8 + 0x8;
                uVar5     = uVar4;
                (**ppcVar1)(0x1008, uVar4, puVar6);
                if((extraout_DX | uVar5) == 0x0)
                {
                    if(puStack22 != (undefined4 *)0x0)
                    {
                        ppcVar1 = ppcVar8;
                        (**ppcVar1)(0x1008, uVar4, (char)puVar6, 0x1);
                    }
                }
                else
                {
                    ppcVar1 = (code **)((int)*puStack22 + 0x4);
                    (**ppcVar1)(0x8, uVar4, puVar6, 0x0, 0x0);
                    puVar7 = extraout_DX_00;
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, (uint)extraout_DX_00);
                    puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_6, puVar7, (int)((ulong)uVar2 >> 0x10));
                    pass1_1010_71d6(
                      (ulong)puVar10,
                      0x1,
                      (ushort *)((ZEXT24(puVar7) & 0xff00) << 0x10 | (ulong)CONCAT12((char)puVar7, uVar5 + 0xc)),
                      uVar5 + 0xc,
                      (uint)((ulong)puVar10 >> 0x10),
                      param_6);
                    if(puStack22 != (undefined4 *)0x0)
                    {
                        ppcVar1 = (code **)*puStack22;
                        (**ppcVar1)(0x1010, uVar4, (char)puVar6, 0x1);
                    }
                }
            }
        }
    }
    else
    {
        if(param_3._2_2_ == 0x1c5)
        {
            uVar14 = 0xe;
        }
        else
        {
            if(param_3._2_2_ != 0x1c6)
            {
                post_win_msg_1040_7b3c((ulong *)CONCAT13((char)(param_2 >> 0x8), CONCAT12((char)param_2, param_1)),
                                       (ushort)(param_2 >> 0x10),
                                       (ushort)param_3,
                                       param_3._2_2_,
                                       (int)&PTR_LOOP_1050_1040);
                return;
            }
            uVar14 = 0xd;
        }
        uVar16  = 0x0;
        uVar15  = 0x0;
        uVar11  = 0x0;
        uVar12  = 0x0;
        uVar13  = 0x0;
        puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_6, param_4, param_5);
        unk_win_op_1010_7300(
          (ulong)puVar10, CONCAT13(uVar13, CONCAT12(uVar12, uVar11)), uVar14, CONCAT22(uVar16, uVar15));
    }
    return;
}


long __stdcall16far
call_win_proc_1038_d020(HWND16 win_handle_1, u32 param_2, LPARAM l_param, u16 param_4, HWND16 win_handle_2)

{
    HANDLE16   handle_1;
    HANDLE16   handle_2;
    uint       var1;
    LRESULT    lresult;
    i32        var5;
    u32       *var6;
    long       var7;
    undefined2 var8;
    code     **fn_ptr_1;
    undefined2 var2;
    undefined2 var3;
    undefined2 var4;
    WPARAM16   w_param;

    var4     = SUB42(&USHORT_1050_1050, 0x0);
    var3     = l_param._2_2_;
    handle_1 = GetProp16(win_handle_2, (LPCSTR)s_procHi_1050_5bd7);
    var2     = l_param._2_2_;
    handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)s_procLo_1050_5bd0);
    var7     = CONCAT22(handle_1, handle_2);
    var8     = l_param._2_2_;
    handle_1 = GetProp16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)s_thisHi_1050_5be5);
    handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)s_thisLo_1050_5bde);
    var6     = (u32 *)CONCAT22(handle_1, handle_2);
    w_param  = (WPARAM16)(param_2 >> 0x10);
    if((handle_1 | handle_2) != 0x0)
    {
        var5 = 0x0;
        if((int)l_param == 0x19)
        {
            fn_ptr_1 = (code **)((int)*var6 + 0x34);
            var5     = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,
                                (char)handle_2,
                                handle_1,
                                win_handle_1,
                                param_2,
                                l_param._2_2_,
                                var8,
                                var2,
                                var3,
                                var4);
        }
        else
        {
            if((int)l_param == 0x86)
            {
                fn_ptr_1 = (code **)((int)*var6 + 0x20);
                var1     = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538, handle_2, handle_1, w_param);
                goto LAB_1038_d10e;
            }
            if(((int)l_param == 0x112) && ((w_param & 0xfff0) == 0xf140))
            {
                lresult = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x112f140);
                var1    = (uint)((int)lresult == 0x0);
                goto LAB_1038_d10e;
            }
        }
        if(var5 != 0x0)
        {
            return var5;
        }
    }
    if(var7 != 0x0)
    {
        lresult = CallWindowProc16((LPVOID)s_tile2_bmp_1050_1538, win_handle_1, (UINT16)param_2, w_param, l_param);
        return lresult;
    }
    var1 = 0x0;
LAB_1038_d10e:
    return (long)(int)var1;
}


void __stdcall16far win_prop_op_1038_d118(ulong param_1, ulong param_2, ushort param_3, ushort param_4, HWND16 param_5)

{
    code      **ppcVar1;
    undefined4  uVar2;
    char        cVar3;
    HANDLE16    HVar4;
    HANDLE16    HVar5;
    ushort      uVar6;
    ushort      uVar7;
    undefined2  uVar8;
    undefined4 *puStack6;

    uVar8    = SUB42(&USHORT_1050_1050, 0x0);
    uVar7    = param_3;
    HVar4    = GetProp16(param_5, (LPCSTR)s_thisHi_1050_5bf3);
    uVar6    = param_3;
    HVar5    = GetProp16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)s_thisLo_1050_5bec);
    puStack6 = (undefined4 *)CONCAT22(HVar4, HVar5);
    if(param_2._2_2_ == 0x30)
    {
        if((LPCSTR)param_2 == (LPCSTR)0x0)
        {
            return;
        }
        SetProp16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)param_2, 0x5c06);
        return;
    }
    if(param_2 < 0x310000)
    {
        cVar3 = (char)(param_2 >> 0x10);
        if(cVar3 == '\x02')
        {
            if((HVar4 | HVar5) != 0x0)
            {
                uVar2   = *puStack6;
                ppcVar1 = (code **)uVar2 + 0x6;
                (**ppcVar1)((int)s_tile2_bmp_1050_1538, HVar5, HVar4, param_1, param_2, uVar6, uVar7, uVar8);
                if(puStack6 != (undefined4 *)0x0)
                {
                    ppcVar1 = (code **)uVar2;
                    (**ppcVar1)((int)s_tile2_bmp_1050_1538, HVar5, HVar4, 0x1);
                }
            }
            HVar4 = GetProp16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)0x5bfa);
            if(HVar4 == 0x0)
            {
                return;
            }
            DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
            RemoveProp16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)0x5c00);
            return;
        }
        if(cVar3 == '\x06')
        {
            if(((LPCSTR)param_2 != (LPCSTR)((int)&PTR_LOOP_1050_0000 + 0x1))
               && ((LPCSTR)param_2 != (LPCSTR)&PTR_LOOP_1050_0002))
            {
                uVar2                             = *(undefined4 *)&PTR_LOOP_1050_5bc8;
                *(undefined2 *)((int)uVar2 + 0x8) = 0x0;
                return;
            }
            uVar2                         = *(undefined4 *)&PTR_LOOP_1050_5bc8;
            *(ushort *)((int)uVar2 + 0x8) = param_3;
            return;
        }
    }
    if((HVar4 | HVar5) != 0x0)
    {
        ppcVar1 = (code **)((int)*puStack6 + 0xc);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538, HVar5, HVar4, param_1, param_2);
    }
    return;
}


void __stdcall16far post_win_msg_1038_d840(astruct_70 *param_1, uint param_2, HWND16 param_3)

{
    astruct_70 *iVar1;
    astruct_70 *uVar1;

    iVar1 = (astruct_70 *)param_1;
    uVar1 = (astruct_70 *)((ulong)param_1 >> 0x10);
    if(param_2 == 0x10)
    {
        if(iVar1->field_0x8e != 0x0)
        {
            PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x111, iVar1->field_0x8e));
            iVar1->field_0x8e = 0x0;
            return;
        }
    }
    else
    {
        if(param_2 < 0x11)
        {
            if((char)param_2 == '\x01')
            {
                iVar1->field_0x90 = 0x0;
                iVar1->field_0x92 = 0x0;
                return;
            }
            if((char)param_2 == '\x03')
            {
                pass1_1038_e03e((ulong)param_1);
                return;
            }
        }
    }
    return;
}


LRESULT __stdcall16far send_msg_1038_c228(ulong param_1, HWND16 param_2)

{
    WPARAM16 wparam;
    LRESULT  LVar1;
    LRESULT  LVar2;

    wparam = (WPARAM16)(param_1 >> 0x10);
    LVar1  = SendMessage16(param_2, 0x0, 0x0, 0x4070000);
    LVar2  = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4070000);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, (int)param_1 + 0x9e, wparam, CONCAT22(0x408, (int)LVar1));
    LVar1 = SendMessage16((HWND16)s_tile2_bmp_1050_1538, (int)param_1 + 0x19e, wparam, CONCAT22(0x408, (int)LVar2));
    return LVar1;
}


void __stdcall16far send_msg_1038_c374(ulong param_1, ulong *param_2, uint param_3, HWND16 param_4)

{
    undefined4  uVar1;
    code      **ppcVar2;
    undefined2  uVar3;
    ulong       uVar4;
    undefined2  extraout_DX;
    uint        extraout_DX_00;
    uint        uVar5;
    undefined2  uVar6;
    LRESULT     LVar7;
    astruct_18 *paVar8;
    uint        uVar9;
    ulong       uStack10;
    ulong       uStack6;

    uVar6   = SUB42(s_tile2_bmp_1050_1538, 0x0);
    uVar9   = param_3;
    LVar7   = SendMessage16(param_4, 0x0, 0x0, 0x40b0000);
    uVar3   = (undefined2)LVar7;
    uVar5   = (uint)param_2;
    ppcVar2 = (code **)((int)*param_2 + 0x10);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, param_2, uVar9);
    uStack6  = CONCAT22(extraout_DX, uVar3);
    uStack10 = 0x0;
    while(true)
    {
        if(uStack6 <= uStack10)
        {
            return;
        }
        ppcVar2 = (code **)((int)*param_2 + 0x4);
        uVar4   = uStack6;
        (**ppcVar2)(uVar6, param_2, (char)uStack10, (int)(uStack10 >> 0x10), uVar5);
        uVar1  = *(undefined4 *)((int)param_1 + 0x8e);
        paVar8 = (astruct_18 *)string_1008_e586((ushort)uVar1,
                                                (ushort)((ulong)uVar1 >> 0x10),
                                                uVar4 & 0xffff | (ulong)extraout_DX_00 << 0x10,
                                                (uint)uVar4,
                                                extraout_DX_00);
        uVar5  = param_3;
        LVar7  = SendMessage16(0x1008, (UINT16)paVar8, (WPARAM16)((ulong)paVar8 >> 0x10), 0x4030000);
        uVar6  = 0x1000;
        fn_ptr_1000_17ce(paVar8, 0x1000);
        if(LVar7 == -0x1)
            break;
        if(LVar7 == -0x2)
        {
            return;
        }
        uStack10 = uStack10 + 0x1;
    }
    return;
}


void __stdcall16far destroy_win_1038_a3d2(ULONG param_1, HWND16 param_2)

{
    GetWindowWord16(param_2, -0x8);
    PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110105);
    destroy_win_1040_7b98(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far send_dlg_item_msg_1038_8d22(ulong param_1, HWND16 param_2, UINT16 param_3)

{
    undefined2 unaff_DI;
    undefined  in_AF;
    LRESULT    LVar1;
    undefined  local_106[0x100];
    WPARAM16   WStack6;
    int        iStack4;

    LVar1   = SendDlgItemMessage16(param_2, 0x0, 0x0, 0x0, 0x185b0409);
    WStack6 = (WPARAM16)LVar1;
    iStack4 = (int)WStack6 >> 0xf;
    if(WStack6 != 0xffff)
    {
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, (INT16)local_106, param_3, WStack6, 0x185b040a);
        pass1_1008_c79a(*(ulong *)((int)param_1 + 0x94), CONCAT22(param_3, local_106), unaff_DI, param_3, in_AF);
    }
    return;
}


LRESULT __stdcall16far pass1_1038_8d7e(ulong param_1, ushort param_2)

{
    LRESULT LVar1;

    pass1_1040_78de(param_1);
    LVar1 = send_dlg_item_msg_1038_8f74(param_1, (int)&PTR_LOOP_1050_1040, param_2);
    return LVar1;
}


void __stdcall16far win_msg_op_1038_95fc(ulong param_1, ushort param_2)

{
    code      **ppcVar1;
    uint        uVar2;
    UINT16      UVar3;
    UINT16      UVar4;
    uchar      *in_DX;
    uchar      *extraout_DX;
    uchar      *puVar5;
    uchar      *extraout_DX_00;
    int         iVar6;
    int         unaff_DI;
    undefined2  uVar7;
    HWND16      hwnd;
    HWND16      HVar8;
    ushort      uVar9;
    ushort      uVar10;
    undefined2  uVar11;
    undefined2  uVar12;
    undefined2 *puStack30;
    undefined2 *puStack24;
    int         iStack20;
    BOOL16      local_10;
    undefined4 *puStack14;
    ushort     *puStack10;
    ushort     *puStack6;

    puStack6  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_2, in_DX, unaff_DI);
    puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, param_2, (uchar *)((ulong)puStack6 >> 0x10), unaff_DI);
    puVar5    = (uchar *)((ulong)puStack10 >> 0x10);
    uVar2     = (uint)puStack10;
    hwnd      = 0x1000;
    mem_op_1000_179c(0xc, puVar5, 0x1000);
    if(((uint)puVar5 | uVar2) == 0x0)
    {
        uVar2  = 0x0;
        puVar5 = (uchar *)0x0;
    }
    else
    {
        hwnd = 0x1008;
        set_struct_1008_574a((astruct_21 *)CONCAT22(puVar5, uVar2));
        puVar5 = extraout_DX;
    }
    puStack14 = (undefined4 *)CONCAT22(puVar5, uVar2);
    for(iStack20 = 0x0; iStack20 < 0xf; iStack20 = iStack20 + 0x1)
    {
        uVar12 = *(undefined2 *)((int)param_1 + 0x6);
        HVar8  = (HWND16)s_tile2_bmp_1050_1538;
        UVar3  = GetDlgItemInt16(hwnd, 0x1, &local_10, param_2);
        if(UVar3 != 0x0)
        {
            if(*(int *)(iStack20 * 0xe + 0x5a7c) < 0x83)
            {
                uVar11 = 0x8;
                HVar8  = 0x1000;
                UVar4  = UVar3;
                mem_op_1000_179c(0x8, puVar5, 0x1000);
                puStack24 = (undefined2 *)CONCAT22(puVar5, UVar4);
                if(((uint)puVar5 | UVar4) == 0x0)
                {
                    puStack30 = (undefined2 *)0x0;
                }
                else
                {
                    *puStack24                   = 0x389a;
                    *(undefined2 *)(UVar4 + 0x2) = 0x1008;
                    *puStack24                   = 0xa1c4;
                    *(undefined2 *)(UVar4 + 0x2) = 0x1010;
                    puStack30                    = puStack24;
                }
                uVar7                        = (undefined2)((ulong)puStack30 >> 0x10);
                iVar6                        = (int)puStack30;
                *(UINT16 *)(iVar6 + 0x6)     = UVar3;
                *(undefined2 *)(iVar6 + 0x4) = *(undefined2 *)(iStack20 * 0xe + 0x5a7c);
                ppcVar1                      = (code **)((int)*puStack14 + 0x4);
                (**ppcVar1)(0x1000, (int)puStack14, (int)((ulong)puStack14 >> 0x10), iVar6, uVar7, uVar11, uVar12);
                puVar5 = extraout_DX_00;
            }
            else
            {
                if(*(int *)(iStack20 * 0xe + 0x5a7c) == 0x89)
                {
                    uVar10 = *(ushort *)(iStack20 * 0xe + 0x5a7c);
                    uVar9  = UVar3;
                }
                else
                {
                    uVar10 = *(ushort *)(iStack20 * 0xe + 0x5a7c);
                    uVar9  = 0x0;
                }
                HVar8 = 0x1010;
                pass1_1010_6566((ulong)puStack10, uVar9, UVar3, uVar10, param_2);
            }
        }
        hwnd = HVar8;
    }
    *(undefined4 *)((int)puStack6 + 0xa) = puStack14;
    PostMessage16(hwnd, 0x0, 0x0, 0x11100ed);
    return;
}


void __stdcall16far
win_ui_op_1038_977a(int param_1, ushort param_2, int param_3, uchar *param_4, HWND16 param_5, ushort param_6)

{
    code      **ppcVar1;
    uint        uVar2;
    int         iVar3;
    uchar      *puVar4;
    undefined2  uVar5;
    undefined2  uVar6;
    undefined   local_10[0x4];
    undefined4 *puStack12;
    int         iStack8;
    UINT16      UStack6;
    BOOL16      local_4;

    iStack8 = 0x0;
    uVar6   = *(undefined2 *)(param_1 + 0x6);
    uVar2   = GetDlgItemInt16(param_5, 0x1, &local_4, param_6);
    UStack6 = uVar2;
    if(uVar2 != 0x0)
    {
        uVar5 = 0xb4;
        mem_op_1000_179c(0xb4, param_4, 0x1000);
        puVar4 = (uchar *)((uint)param_4 | uVar2);
        if(puVar4 == (uchar *)0x0)
        {
            iVar3  = 0x0;
            puVar4 = (uchar *)0x0;
        }
        else
        {
            iVar3 = string_1040_8520((astruct_57 *)CONCAT22(param_4, uVar2),
                                     *(ushort *)(param_1 + 0x6),
                                     0x41,
                                     0x2,
                                     0x5db,
                                     0x5da,
                                     puVar4,
                                     param_6);
        }
        puStack12 = (undefined4 *)CONCAT22(puVar4, iVar3);
        pass1_1008_941a((ushort *)CONCAT22(param_6, local_10), 0x1, 0xc3);
        ppcVar1 = (code **)((int)*puStack12 + 0x6c);
        iStack8 = (**ppcVar1)(
          0x1008, (int)puStack12, (int)((ulong)puStack12 >> 0x10), local_10, param_6, uVar5, uVar6, uVar2);
    }
    if((iStack8 == 0x1) || (UStack6 == 0x0))
    {
        destroy_window_1040_b726((ULONG *)CONCAT22(param_2, param_1), param_3, (int)&PTR_LOOP_1050_1040);
    }
    return;
}


long __stdcall16far
unk_win_ui_op_1038_9820(astruct_51 *param_1, int param_2, int param_3, HWND16 param_4, BOOL16 param_5)

{
    int        *piVar1;
    long        lVar2;
    UINT16      UVar3;
    int         iVar4;
    int         iVar5;
    uint        uVar6;
    astruct_51 *iVar7;
    astruct_51 *uVar7;
    BOOL16      local_6;
    BOOL16      local_4;

    uVar7 = (astruct_51 *)((ulong)param_1 >> 0x10);
    iVar7 = (astruct_51 *)param_1;
    UVar3 = GetDlgItemInt16(param_4, 0x1, &local_4, param_5);
    iVar4 = UVar3 * param_2 * param_3;
    UVar3 = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538, 0x1, &local_6, param_5);
    lVar2 = (long)(int)(UVar3 * param_2) * (long)param_3;
    uVar6 = (uint)((ulong)lVar2 >> 0x10);
    iVar5 = (int)lVar2;
    if((iVar4 - iVar7->field_0x94 < 0x1) && (-0x1 < iVar7->field_0x96 - iVar5))
    {
        piVar1  = &iVar7->field_0x94;
        *piVar1 = *piVar1 - iVar4;
        piVar1  = &iVar7->field_0x96;
        *piVar1 = *piVar1 - iVar5;
        return CONCAT22(uVar6, 0x1);
    }
    return (long)((ulong)uVar6 << 0x10);
}


void __stdcall16far win_ui_dlg_op_1038_98b4(astruct_51 *param_1, HWND16 param_2, BOOL16 param_3)

{
    UINT16     UVar1;
    undefined2 uVar2;
    int        iVar3;
    int        iStack8;
    BOOL16     local_4;

    local_4 = 0x0;
    for(iStack8 = 0x0; iVar3 = (int)param_1, uVar2 = (undefined2)((ulong)param_1 >> 0x10), iStack8 < 0xf;
        iStack8 = iStack8 + 0x1)
    {
        iVar3 = *(int *)(iVar3 + 0x6);
        UVar1 = GetDlgItemInt16(param_2, 0x1, &local_4, param_3);
        unk_win_ui_op_1038_9820(param_1, UVar1, iVar3, (int)s_tile2_bmp_1050_1538, param_3);
        param_2 = (HWND16)s_tile2_bmp_1050_1538;
    }
    SetDlgItemInt16(param_2, 0x1, *(UINT16 *)(iVar3 + 0x94), 0xfa9);
    SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538, 0x1, *(UINT16 *)(iVar3 + 0x96), 0xfa8);
    return;
}


void __stdcall16far pass1_1038_362e(ulong param_1)

{
    ushort      in_AX;
    uchar      *in_DX;
    int         iVar1;
    int         unaff_DI;
    uint        uVar2;
    ushort      unaff_SS;
    astruct_67 *paVar3;

    uVar2 = (uint)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0x214) == 0x0)
    {
        pass1_1038_4f54(param_1 & 0xffff | (ulong)uVar2 << 0x10, 0x1f, in_AX);
        if(in_AX == 0x0)
        {
            *(undefined2 *)(iVar1 + 0x214) = 0x14;
        }
        else
        {
            *(undefined2 *)(iVar1 + 0x214) = 0x28;
        }
        paVar3 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, unaff_SS, in_DX, unaff_DI);
        post_win_msg_1008_a0e4(paVar3, 0x0, 0x0, 0x1, *(ulong *)(iVar1 + 0x4), 0x38, 0x1008, unaff_SS);
        *(undefined4 *)(iVar1 + 0x216) = 0x0;
    }
    return;
}


void __stdcall16far
pass1_1038_095e(ushort param_1, ushort param_2, int param_3, ulong param_4, uchar *param_5, int param_6, ushort param_7)

{
    code      **ppcVar1;
    bool        bVar2;
    uint        uVar3;
    undefined  *puVar4;
    ulong       uVar5;
    ulong       uVar6;
    uchar      *puVar7;
    undefined2  uVar8;
    undefined   uVar9;
    ulong      *puVar10;
    ulong       uVar11;
    int         iVar12;
    ulong       uStack58;
    ulong       uStack54;
    undefined   local_28[0x2];
    ulong       uStack38;
    ulong       uStack34;
    undefined4 *puStack30;
    uint        uStack26;
    uchar      *puStack24;
    undefined4 *puStack22;
    ulong       uStack18;
    int         iStack14;
    int         iStack12;
    ulong       uStack10;
    astruct_67 *paStack6;

    paStack6 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_7, param_5, param_6);
    uStack10 = *_PTR_LOOP_1050_65e2;
    uVar8    = (undefined2)(param_4 >> 0x10);
    if(uStack10 % 0xa == 0x0)
    {
        if(param_3 < 0xc9)
        {
            iVar12 = 0x3f;
        }
        else
        {
            if(param_3 < 0x320)
                goto LAB_1038_09c3;
            iVar12 = 0x3e;
        }
        post_win_msg_1008_a0e4(paStack6, 0x0, 0x0, 0x1, *(ulong *)((int)param_4 + 0x4), iVar12, 0x1008, param_7);
    }
LAB_1038_09c3:
    iStack12 = *(int *)((int)param_4 + 0x22);
    iStack14 = 0x0;
    uStack18 = *_PTR_LOOP_1050_65e2;
    if(iStack12 < 0x4b)
    {
        if(iStack12 < 0x3c)
        {
            if(iStack12 < 0x32)
                goto LAB_1038_0a1c;
            uVar3 = 0x1e;
        }
        else
        {
            uVar3 = 0xf;
        }
    }
    else
    {
        uVar3 = 0x5;
    }
    if((uStack18 & 0xffff | (ulong) * (uint *)((int)_PTR_LOOP_1050_65e2 + 0x2) << 0x10) % (ulong)uVar3 == 0x0)
    {
        iStack14 = 0x1;
    }
LAB_1038_0a1c:
    if(iStack14 != 0x0)
    {
        puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0xf);
        puVar7  = (uchar *)((ulong)puVar10 >> 0x10);
        uVar3   = (uint)puVar10;
        pass1_1038_4e78(uVar3, puVar7, param_4, puVar10);
        puStack22 = (undefined4 *)CONCAT22(puVar7, uVar3);
        puVar10   = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1a);
        puVar7    = (uchar *)((ulong)puVar10 >> 0x10);
        uVar3     = (uint)puVar10;
        uStack26  = uVar3;
        puStack24 = puVar7;
        pass1_1038_4d6e(param_4, puVar10, uVar3, puVar7);
        puStack30 = (undefined4 *)CONCAT22(puVar7, uVar3);
        ppcVar1   = (code **)((int)*puStack22 + 0x10);
        (**ppcVar1)(0x1008, (int)puStack22, (int)((ulong)puStack22 >> 0x10));
        uStack34 = CONCAT22(puVar7, uVar3);
        ppcVar1  = (code **)((int)*puStack30 + 0x10);
        (**ppcVar1)(0x1008, (char)puStack30, (int)((ulong)puStack30 >> 0x10));
        uStack38 = CONCAT22(puVar7, uVar3);
        uVar11   = pass1_1030_bcae((ushort)local_28, param_7);
        uStack54 = 0x0;
        while(true)
        {
            uVar11 = uVar11 >> 0x10;
            uVar9  = 0x30;
            if(uStack34 <= uStack54)
                break;
            uVar6 = uStack34;
            pass1_1030_1d58((ulong)puStack22);
            uVar6 = uVar6 & 0xffff | uVar11 << 0x10;
            bVar2 = false;
            for(uStack58 = 0x0; uStack58 < uStack38; uStack58 = uStack58 + 0x1)
            {
                uVar5 = uStack38;
                pass1_1030_1d58((ulong)puStack30);
                puVar4 = local_28;
                pass1_1030_bd74((ushort)puVar4, param_7, uVar6, uVar5 & 0xffff | uVar11 << 0x10, param_7);
                if((int)puVar4 < 0x6)
                {
                    bVar2 = true;
                    break;
                }
            }
            uVar11 = struct_op_1030_73a8(uVar6);
            if(!bVar2)
            {
                uVar9 = 0x28;
                func_0x10285ca0(0x1030, (char)uVar11, (int)(uVar11 >> 0x10));
                break;
            }
            uStack54 = uStack54 + 0x1;
        }
        if(puStack22 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack22;
            (**ppcVar1)(uVar9, (int)puStack22, (char)((ulong)puStack22 >> 0x10), 0x1);
        }
        if(puStack30 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack30;
            (**ppcVar1)(uVar9, (int)puStack30, (char)((ulong)puStack30 >> 0x10), 0x1);
        }
    }
    return;
}


ushort __stdcall16far pass1_1030_e67c(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    ushort      uVar1;
    astruct_67 *paVar2;

    paVar2 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, param_2, param_3);
    uVar1  = pass1_1008_aaa8((ushort)paVar2, (ushort)((ulong)paVar2 >> 0x10), *(ushort *)((int)param_1 + 0x108));
    if(uVar1 != 0x0)
    {
        post_win_msg_1008_a0e4(paVar2, 0x0, 0x0, 0x1, 0x0, uVar1, 0x1008, param_4);
    }
    return 0x1;
}


void __stdcall16far pass1_1030_838e(ulong *param_1, ushort param_2, uchar param_3)

{
    struct_1028_d2b0((ulong *)*param_1, param_2, param_3);
    pass1_1028_d01a(*(ulong **)((int)param_1 + 0x4));
    send_msg_1028_e242(_PTR_LOOP_1050_65e2, 0x1, (int)&USHORT_1050_1028);
    return;
}


void __stdcall16far pass1_1030_83ba(ulong **param_1, long param_2, ushort param_3, uchar param_4)

{
    long lVar1;

    while(lVar1 = param_2 + -0x1, param_2 != 0x0)
    {
        struct_1028_d2b0(*param_1, param_3, param_4);
        pass1_1028_d01a(*(ulong **)((int)param_1 + 0x4));
        param_2 = lVar1;
        if(lVar1 != 0x0)
        {
            send_msg_1028_e242(_PTR_LOOP_1050_65e2, 0x0, (int)&USHORT_1050_1028);
        }
    }
    send_msg_1028_e242(_PTR_LOOP_1050_65e2, 0x1, (int)&USHORT_1050_1028);
    return;
}


void __stdcall16far send_msg_1028_e242(ulong *param_1, int param_2, HWND16 param_3)

{
    uchar  *puVar1;
    int     unaff_DI;
    ushort  unaff_SS;
    LRESULT LVar2;

    puVar1 = (uchar *)(*param_1 % 0x64);
    if(*param_1 % 0x64 == 0x0)
    {
        LVar2  = SendMessage16(param_3, 0x0, 0x0, 0x410000);
        puVar1 = (uchar *)((ulong)LVar2 >> 0x10);
    }
    *param_1 = *param_1 + 0x1;
    if(param_2 != 0x0)
    {
        pass1_1028_e28a(puVar1, unaff_DI, unaff_SS);
    }
    return;
}


void __stdcall16far pass1_1028_9a02(ulong param_1, int param_2, ushort param_3, ushort param_4, int param_5)

{
    undefined4  uVar1;
    undefined  *puVar2;
    ulong       uVar3;
    uint        uVar4;
    uint        uVar5;
    int         iVar6;
    undefined2  uVar7;
    ushort     *puVar8;
    astruct_67 *paVar9;
    undefined2  uVar10;
    undefined   uVar11;
    undefined   uVar12;
    undefined2  uVar13;
    undefined2  uVar14;
    int         iVar15;
    undefined   local_30[0x12];
    int         iStack30;
    undefined2  uStack26;
    undefined2  uStack22;
    undefined2  uStack20;
    long        lStack18;
    ulong       uStack10;
    ulong       uStack6;

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    uVar1 = *(undefined4 *)(iVar6 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    uStack6  = CONCAT22(param_3, param_2);
    uVar3    = *(ulong *)(param_2 + 0x1f6);
    uStack10 = uVar3;
    pass1_1030_3694(uVar3, 0x0, *(long *)(iVar6 + 0x110), (uchar *)param_3, 0x1030, param_4);
    uVar4                      = (uint)uVar3;
    *(uint *)(iVar6 + 0x114)   = uVar4;
    *(uchar **)(iVar6 + 0x116) = (uchar *)param_3;
    pass1_1030_38b8();
    if((param_3 | uVar4) == 0x0)
    {
        lStack18 = *(long *)((int)uStack6 + 0x200);
        puVar8   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_4, (uchar *)0x0, param_5);
        uStack20 = (undefined2)((ulong)puVar8 >> 0x10);
        uStack22 = SUB42(puVar8, 0x0);
        if(lStack18 == 0x8000002)
        {
            iVar6 = 0x1f;
        }
        else
        {
            iVar6 = 0xb;
        }
        pass1_1010_043a((ulong)puVar8, *(long *)((int)uStack6 + 0x4), iVar6, param_4);
        if(lStack18 == 0x8000001)
        {
            uVar7 = 0x2;
        }
        else
        {
            uVar7 = 0x1;
        }
        uVar4    = 0x800;
        lStack18 = CONCAT22(0x800, uVar7);
        pass1_1038_349e(uStack6, CONCAT22(0x800, uVar7));
        iStack30 = 0x0;
        uStack26 = 0x0;
        pass1_1028_dc52(
          (astruct_92 *)CONCAT13((char)(param_4 >> 0x8), CONCAT12((char)param_4, local_30)), 0x1, 0x0, 0x400);
        while(true)
        {
            puVar2 = local_30;
            pass1_1028_e4ec(CONCAT22(param_4, puVar2));
            uStack6 = CONCAT22(uVar4, puVar2);
            uVar5   = uVar4 | (uint)puVar2;
            if(uVar5 == 0x0)
                break;
            if(*(long *)(puVar2 + 0x200) == 0x8000002)
            {
                uStack26 = 0x1;
                uVar4    = uVar5;
            }
            else
            {
                iStack30 = 0x1;
                uVar4    = uVar5;
            }
        }
        if(iStack30 == 0x0)
        {
            uVar14 = 0x0;
            iVar15 = 0x3c;
            uVar11 = 0x1;
            uVar12 = 0x0;
            uVar13 = 0x0;
            uVar10 = 0x0;
            iVar6  = 0x0;
            uVar7  = 0x0;
            paVar9 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, (uchar *)0x0, param_5);
            post_win_msg_1008_a0e4(paVar9,
                                   CONCAT22(uVar10, uVar7),
                                   iVar6,
                                   CONCAT11(uVar12, uVar11),
                                   CONCAT22(uVar14, uVar13),
                                   iVar15,
                                   0x1008,
                                   param_4);
        }
    }
    return;
}


void __stdcall16far
pass1_1028_a188(ushort param_1, ushort param_2, int param_3, int param_4, ulong param_5, ushort param_6)

{
    ulong      uVar1;
    long       lVar2;
    ulong      uVar3;
    undefined2 uVar4;
    uint       uVar5;
    uint       uVar6;
    ulong      uVar7;
    long       lVar8;
    long       lVar9;
    uint       uVar10;
    int        iVar11;
    int        iVar12;
    uchar     *puVar13;
    undefined2 uVar14;
    ushort    *puVar15;
    uint       uStack18;
    uint       uStack16;
    uint       uStack14;
    int        iStack12;

    uVar14 = (undefined2)(param_5 >> 0x10);
    iVar11 = (int)param_5;
    uVar1  = *(ulong *)(iVar11 + 0x1f6);
    uVar6  = *(uint *)(iVar11 + 0x1f8);
    uVar5  = (int)uVar1 + 0x18c;
    uVar4  = (undefined2)(uVar1 >> 0x10);
    uVar7  = (ulong)uVar5;
    pass1_1030_38f2(uVar1 & 0xffff | (ulong)uVar6 << 0x10, param_4, param_6);
    uVar3                             = 0x64 / (long)param_3;
    uVar10                            = (int)uVar3 >> 0xf;
    iVar12                            = param_4 * 0x4;
    lVar2                             = (uVar7 & 0xffff | (ulong)uVar6 << 0x10) + *(long *)(iVar12 + uVar5);
    lVar8                             = lVar2 / (long)(uVar3 & 0xffff | (ulong)uVar10 << 0x10);
    lVar9                             = lVar8 * (uVar3 & 0xffff | (ulong)uVar10 << 0x10);
    uStack14                          = (uint)lVar2;
    iStack12                          = (int)((ulong)lVar2 >> 0x10);
    uVar6                             = (uint)lVar9;
    puVar13                           = (uchar *)((iStack12 - (int)((ulong)lVar9 >> 0x10)) - (uint)(uStack14 < uVar6));
    *(int *)(uVar5 + iVar12)          = uStack14 - uVar6;
    *(uchar **)(uVar5 + iVar12 + 0x2) = puVar13;
    uStack16                          = (uint)((ulong)lVar8 >> 0x10);
    uStack18                          = (uint)lVar8;
    if((uStack16 | uStack18) != 0x0)
    {
        pass1_1030_375a(uVar1, param_4, lVar8, param_6);
        if(*(long *)(iVar11 + 0x200) != 0x8000002)
        {
            puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_6, puVar13, iVar12);
            puVar13 = (uchar *)((ulong)puVar15 >> 0x10);
            post_win_msg_1008_a0e4((astruct_67 *)((ulong)puVar15 & 0xffff | ZEXT24(puVar13) << 0x10),
                                   0x0,
                                   uStack18,
                                   *(ushort *)(iVar11 + 0x208),
                                   *(ulong *)(iVar11 + 0x4),
                                   0x2,
                                   0x1008,
                                   param_6);
            puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar13, iVar12);
            pass1_1010_043a((ulong)puVar15, *(long *)(iVar11 + 0x4), 0xd, param_6);
        }
    }
    return;
}


void __stdcall16far pass1_1028_86c2(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    astruct_67 *paVar1;
    undefined2  uVar2;
    undefined2  uVar3;
    int         iVar4;
    ushort      uVar5;
    undefined2  uVar6;
    undefined2  uVar7;
    int         iVar8;

    uVar7  = 0x0;
    iVar8  = 0x1d;
    uVar5  = 0x1;
    uVar6  = 0x0;
    uVar3  = 0x0;
    iVar4  = 0x0;
    uVar2  = 0x0;
    paVar1 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, param_2, param_3);
    post_win_msg_1008_a0e4(
      paVar1, CONCAT22(uVar3, uVar2), iVar4, uVar5, CONCAT22(uVar7, uVar6), iVar8, 0x1008, param_4);
    pass1_1028_6b2c(param_1, param_4);
    return;
}


void __stdcall16far pass1_1028_9114(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    undefined2  uVar1;
    uchar      *puVar2;
    uint        uVar3;
    astruct_67 *paVar4;
    ushort     *puVar5;
    ushort      uVar6;
    uchar      *puVar7;
    int         iVar8;
    uint        uVar9;
    int         iVar10;
    uint        uStack10;

    paVar4 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, param_2, param_3);
    uVar3  = (uint)param_1;
    iVar10 = *(int *)(uVar3 + 0x108);
    if(iVar10 - 0x1U < 0x8)
    {
        uStack10 = (uint)*_PTR_LOOP_1050_65e2;
        iVar8    = (int)((ulong)*_PTR_LOOP_1050_65e2 >> 0x10);
        switch(iVar10)
        {
        case 0x1:
            iVar10 = 0x16;
            break;
        case 0x2:
            iVar10 = 0x17;
            break;
        case 0x3:
            iVar10 = 0x18;
            break;
        case 0x4:
            iVar10 = 0x1b;
            break;
        case 0x5:
            iVar10 = 0x1f;
            break;
        case 0x6:
            iVar10 = 0x24;
            break;
        case 0x7:
            pass1_1008_612e(0x0, 0x14, uVar3);
            puVar2 = (uchar *)(((int)uVar3 >> 0xf) + (uint)(0xff91 < uVar3));
            uVar6  = uStack10 + uVar3 + 0x6e;
            puVar7 = puVar2 + (uint)CARRY2(uStack10, uVar3 + 0x6e) + iVar8;
            iVar10 = 0x7;
            puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, puVar2, param_3);
            uVar1  = (undefined2)((ulong)puVar5 >> 0x10);
            uVar3  = (uint)puVar5;
            pass1_1010_ebf8((ulong)puVar5, uVar6, (ushort)puVar7, iVar10);
            pass1_1008_612e(0x1, 0x64, uVar3);
            if(0x32 < (int)uVar3)
            {
                return;
            }
            pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2, 0x1, 0x400);
            pass1_1038_4900(CONCAT22(uVar1, uVar3));
            iVar10 = 0x2c;
            break;
        case 0x8:
            pass1_1008_612e(0x0, 0x14, uVar3);
            puVar2 = (uchar *)(((int)uVar3 >> 0xf) + (uint)(0xff9b < uVar3));
            uVar6  = uStack10 + uVar3 + 0x64;
            puVar7 = puVar2 + (uint)CARRY2(uStack10, uVar3 + 0x64) + iVar8;
            iVar8  = 0x8;
            puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, puVar2, param_3);
            uVar1  = (undefined2)((ulong)puVar5 >> 0x10);
            iVar10 = (int)puVar5;
            pass1_1010_ebf8((ulong)puVar5, uVar6, (ushort)puVar7, iVar8);
            if(0x19 < (int)uVar3)
            {
                return;
            }
            uVar3 = 0x1;
            uVar9 = 0x2;
            pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2, 0x1, 0x400);
            pass1_1038_43cc(iVar10, uVar1, uVar3, uVar9, iVar10, uVar1);
            iVar10 = 0x2d;
        }
        post_win_msg_1008_a0e4(paVar4, 0x0, 0x0, 0x1, 0x0, iVar10, 0x1008, param_4);
    }
    return;
}

void __stdcall16far post_msg_1028_76da(void)

{
    long       lVar1;
    undefined2 uVar2;
    uchar     *in_DX;
    int        unaff_DI;
    ushort     unaff_SS;
    ushort    *puVar3;
    uint       uStack10;
    uint       uStack8;

    puVar3   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2c, unaff_SS, in_DX, unaff_DI);
    uVar2    = (undefined2)((ulong)puVar3 >> 0x10);
    lVar1    = *(long *)((int)puVar3 + 0xc);
    uStack8  = (uint)((ulong)lVar1 >> 0x10);
    uStack10 = (uint)lVar1;
    if(((uStack8 | uStack10) != 0x0) && (*_PTR_LOOP_1050_65e2 == lVar1))
    {
        PostMessage16(0x1010, 0x0, 0x0, 0x1110106);
        *(undefined4 *)((int)puVar3 + 0xc) = 0x0;
    }
    return;
}


void __stdcall16far pass1_1028_6ff6(ulong param_1, uint param_2, int param_3, ushort param_4)

{
    long       *plVar1;
    undefined  *puVar2;
    astruct_67 *paVar3;
    uchar      *puVar4;
    uchar      *puVar5;
    uint        uVar6;
    uint        uVar7;
    ushort     *puVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    int         iVar11;
    undefined   uVar12;
    undefined   uVar13;
    undefined   uVar14;
    undefined2  uVar15;
    ushort      uVar16;
    undefined2  uVar17;
    int         iVar18;
    undefined   local_46[0x12];
    undefined4  uStack52;
    int         iStack48;
    uchar      *puStack46;
    astruct_67 *paStack38;
    undefined  *puStack34;
    uchar      *puStack32;
    int         iStack30;
    int         iStack28;
    int         iStack26;
    ulong       uStack24;
    undefined   local_14[0x8];
    undefined2  uStack12;
    uchar      *puStack10;
    undefined2  uStack8;
    uchar      *puStack6;
    int         iStack4;

    uVar13 = (undefined)(param_4 >> 0x8);
    pass1_1028_dc52((astruct_92 *)CONCAT13(uVar13, CONCAT12((char)param_4, local_14)), 0x1, 0x0, 0x400);
    iStack26 = 0x1;
    iStack28 = 0x0;
    do
    {
        do
        {
            uVar7  = param_2;
            puVar2 = local_14;
            pass1_1028_e4ec(CONCAT22(param_4, puVar2));
            uStack24 = CONCAT22(uVar7, puVar2);
            param_2  = uVar7 | (uint)puVar2;
            if(param_2 == 0x0)
                goto LAB_1028_7066;
        } while((*(int *)(puVar2 + 0x1fe) == 0x0) || (*(long *)(puVar2 + 0x200) == 0x8000002));
        iStack28  = 0x1;
        paVar3    = *(astruct_67 **)(puVar2 + 0x1f6);
        paStack38 = paVar3;
        pass1_1030_38b8();
    } while(((int)param_2 < 0x0) || (((int)param_2 < 0x1 && ((int)paVar3 == 0x0))));
    iStack26 = 0x0;
LAB_1028_7066:
    puStack10 = puStack6;
    uStack12  = uStack8;
    if(iStack4 != 0x0)
    {
        puStack10 = (uchar *)0x0;
        uStack12  = 0x1;
    }
    iStack30 = 0x0;
    puVar4   = puStack10;
    while(true)
    {
        puVar2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar2));
        uStack24  = CONCAT22(puVar4, puVar2);
        puStack32 = (uchar *)((uint)puVar4 | (uint)puVar2);
        if(puStack32 == (uchar *)0x0)
            break;
        plVar1 = (long *)(puVar2 + 0x200);
        puVar4 = puStack32;
        if(*plVar1 == 0x8000001)
        {
            iStack30 = 0x1;
        }
    }
    if(iStack30 == 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x1, 0x400);
        uStack24  = CONCAT22(puStack32, puVar2);
        puStack32 = (uchar *)((uint)puStack32 | (uint)puVar2);
        if(puStack32 != (uchar *)0x0)
        {
            PTR_LOOP_1050_4fe8 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
            uVar16             = 0x0;
            iVar11             = 0x1;
            uStack52           = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_4, puStack32, param_3);
            puStack32          = (uchar *)((ulong)uStack52 >> 0x10);
            puVar2             = (undefined *)uStack52;
            pass1_1010_089e(param_4, (ulong)uStack52, uVar16, iVar11);
            pass1_1010_089e(param_4, (ulong)uStack52, 0x0, 0x2);
            pass1_1010_089e(param_4, (ulong)uStack52, 0x0, 0x3);
            pass1_1010_089e(param_4, (ulong)uStack52, 0x0, 0x4);
            pass1_1010_089e(param_4, (ulong)uStack52, 0x0, 0x5);
            pass1_1010_089e(param_4, (ulong)uStack52, 0x0, 0x7);
            pass1_1010_089e(param_4, (ulong)uStack52, 0x0, 0x8);
            pass1_1010_089e(param_4, (ulong)uStack52, 0x0, 0xa);
        }
    }
    if((iStack28 != 0x0) && (iStack26 != 0x0))
    {
        uVar17    = 0x0;
        iVar18    = 0x6;
        uVar12    = 0x1;
        uVar14    = 0x0;
        uVar15    = 0x0;
        uVar10    = 0x0;
        iVar11    = 0x0;
        uVar9     = 0x0;
        uStack52  = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, puStack32, param_3);
        puStack32 = (uchar *)((ulong)uStack52 >> 0x10);
        puVar2    = (undefined *)uStack52;
        post_win_msg_1008_a0e4(uStack52,
                               CONCAT22(uVar10, uVar9),
                               iVar11,
                               CONCAT11(uVar14, uVar12),
                               CONCAT22(uVar17, uVar15),
                               iVar18,
                               0x1008,
                               param_4);
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x1, 0x800);
    puVar4    = (uchar *)((uint)puStack32 | (uint)puVar2);
    puStack34 = puVar2;
    if(((((puVar4 != (uchar *)0x0)
          && (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32, puVar2), 0x4), puVar2 == (undefined *)0x0))
         && (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x2a), puVar2 == (undefined *)0x0))
        && ((puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x4b),
             puVar2 == (undefined *)0x0
               && (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x54),
                   puVar2 == (undefined *)0x0))))
       && ((puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x2c),
            puVar2 == (undefined *)0x0
              && ((puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x3c),
                   puVar2 == (undefined *)0x0
                     && (puVar2 = (undefined *)pass1_1030_2242(CONCAT22(puStack32, puStack34), 0x3d),
                         puVar2 == (undefined *)0x0))))))
    {
        if(iStack4 != 0x0)
        {
            uStack8  = 0x1;
            puStack6 = (uchar *)0x0;
        }
        uStack52  = (astruct_67 *)((ulong)uStack52 & 0xffff0000);
        iStack48  = 0x0;
        uStack12  = uStack8;
        puStack10 = puStack6;
        do
        {
            do
            {
                puVar5 = puStack6;
                puVar2 = local_14;
                pass1_1028_e4ec(CONCAT22(param_4, puVar2));
                uStack24 = CONCAT22(puVar5, puVar2);
                puVar4   = (uchar *)((uint)puVar5 | (uint)puVar2);
                if(puVar4 == (uchar *)0x0)
                    goto LAB_1028_72d3;
                puStack6 = puVar4;
            } while(*(long *)(puVar2 + 0x200) == 0x8000002);
            uVar16 = (ushort)(param_1 >> 0x10);
            if(((int)uStack52 == 0x0)
               && (pass1_1028_740c((ushort)param_1, uVar16, 0x22, CONCAT22(puVar5, puVar2)),
                   puVar2 != (undefined *)0x0))
            {
                uStack52 = (astruct_67 *)CONCAT22(uStack52._2_2_, 0x1);
            }
            if((iStack48 == 0x0)
               && (pass1_1028_740c((ushort)param_1, uVar16, 0x24, uStack24), puVar2 != (undefined *)0x0))
            {
                iStack48 = 0x1;
            }
            puStack6 = puVar4;
        } while(((int)uStack52 == 0x0) || (iStack48 == 0x0));
        uVar17    = 0x0;
        iVar18    = 0x14;
        uVar12    = 0x1;
        uVar14    = 0x0;
        uVar15    = 0x0;
        uVar10    = 0x0;
        iVar11    = 0x0;
        uVar9     = 0x0;
        paStack38 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, puVar4, param_3);
        puVar4    = (uchar *)((ulong)paStack38 >> 0x10);
        puVar2    = (undefined *)paStack38;
        post_win_msg_1008_a0e4(paStack38,
                               CONCAT22(uVar10, uVar9),
                               iVar11,
                               CONCAT11(uVar14, uVar12),
                               CONCAT22(uVar17, uVar15),
                               iVar18,
                               0x1008,
                               param_4);
    }
LAB_1028_72d3:
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x1, 0x400);
    uStack24 = CONCAT22(puVar4, puVar2);
    if((uchar *)((uint)puVar4 | (uint)puVar2) != (uchar *)0x0)
    {
        puVar8   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_4, (uchar *)((uint)puVar4 | (uint)puVar2), param_3);
        puVar4   = (uchar *)((ulong)puVar8 >> 0x10);
        iStack48 = (int)puVar8;
        puStack46 = puVar4;
        pass1_1008_df4a((ulong)puVar8, param_3, param_4);
        puVar8    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3c, param_4, puVar4, param_3);
        uVar7     = (uint)((ulong)puVar8 >> 0x10);
        iStack48  = (int)puVar8;
        puStack46 = (uchar *)uVar7;
        pass1_1018_34a6((ulong)puVar8);
        pass1_1028_dc52((astruct_92 *)CONCAT13(uVar13, CONCAT12((char)param_4, local_46)), 0x1, 0x0, 0x400);
        while(true)
        {
            uVar6  = uVar7;
            puVar2 = local_46;
            pass1_1028_e4ec(CONCAT22(param_4, puVar2));
            uStack52 = (astruct_67 *)CONCAT22(uVar6, puVar2);
            uVar7    = uVar6 | (uint)puVar2;
            if(uVar7 == 0x0)
                break;
            if(*(long *)(puVar2 + 0x200) != 0x8000002)
            {
                pass1_1038_3ba0(CONCAT22(uVar6, puVar2));
            }
        }
    }
    return;
}
