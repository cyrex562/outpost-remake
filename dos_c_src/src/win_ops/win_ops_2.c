
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
