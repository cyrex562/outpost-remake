
void __stdcall16far destroy_window_1020_3b3e(astruct_30 *param_1, HWND16 param_2)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    uint        uVar3;
    astruct_30 *paVar4;
    astruct_30 *uVar5;
    astruct_30 *uVar6;
    HWND16      HVar5;
    ushort      unaff_SS;

    uVar6              = (astruct_30 *)((ulong)param_1 >> 0x10);
    uVar5              = (astruct_30 *)param_1;
    uVar5->field_0x10e = 0x0;
    HVar5              = param_2;
    if(uVar5->field_0x10a != 0x0)
    {
        HVar5 = (HWND16)s_tile2_bmp_1050_1538;
        DestroyWindow16(param_2);
    }
    puVar1 = uVar5->field_0xf6;
    uVar3  = uVar5->field_0xf8;
    if((uVar3 | (uint)puVar1) != 0x0)
    {
        ppcVar2 = (code **)*puVar1;
        (**ppcVar2)(HVar5, puVar1);
    }
    *(undefined4 *)&uVar5->field_0xf6 = 0x0;
    if(uVar5->field_0xfa != 0x0)
    {
        uVar3 = (uint)uVar6 | (uint)uVar5;
        if(param_1 == (astruct_30 *)0x0)
        {
            paVar4 = (astruct_30 *)0x0;
        }
        else
        {
            uVar3  = &uVar5->field_0xf2;
            paVar4 = uVar6;
        }
        pass1_1010_1ea6(uVar5->field_0xfa, CONCAT22(paVar4, uVar3), unaff_SS);
    }
    uVar5->field_0xfa = 0x0;
    return;
}

void __stdcall16far pass1_1020_3c8c(ulong param_1, ulong param_2, ushort param_3)

{
    pt_in_rect_1018_1bda(*(ulong *)((int)param_1 + 0xfa), (ushort)param_2, (ushort)(param_2 >> 0x10), param_3);
    return;
}

astruct_3 *__stdcall16far pass1_1020_3ca6(astruct_3 *param_1, byte param_2, undefined2 param_3)

{
    ulong       uVar1;
    undefined2 *puStack10;

    uVar1         = (ulong)param_1 & 0xffff0000;
    param_1       = (astruct_3 *)(uVar1 | (int)param_1 - 0xf2);
    param_1._2_2_ = (undefined2)(uVar1 >> 0x10);
    if(param_1 == (astruct_3 *)0x0)
    {
        param_1._0_2_ = 0x0;
        param_1._2_2_ = 0x0;
    }
    puStack10                           = (undefined2 *)CONCAT22(param_1._2_2_, (int)param_1);
    *puStack10                          = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far enable_window_1020_1bd4(int param_1, ushort param_2, ushort param_3, ulong param_4, HWND16 param_5)

{
    code      **ppcVar1;
    bool        bVar2;
    uint        in_AX;
    int         iVar3;
    uchar      *in_DX;
    uchar      *puVar4;
    undefined2  uVar5;
    ushort      unaff_SS;
    undefined4 *puStack12;

    bVar2 = false;
    pass1_1020_1d8e(CONCAT22(param_2, param_1), CONCAT22((int)param_4, param_3));
    if(in_AX != 0x0)
    {
        if((int)in_AX < 0x2)
        {
            bVar2 = true;
        }
        else
        {
            GetDlgItem16(param_5, 0x1);
            pass1_1010_4e8c(*(ulong *)(param_1 + 0x8e), unaff_SS);
            in_AX = EnableWindow16(0x1010, 0x1);
            pass1_1010_4df0(*(ulong *)(param_1 + 0x8e), (uint)in_DX, unaff_SS);
            if((in_AX == 0x0) && (bVar2 = true, *(int *)(param_1 + 0x96) == 0x0))
            {
                in_AX = EnableWindow16(0x1010, 0x0);
            }
        }
    }
    if(bVar2)
    {
        uVar5 = 0x1000;
        mem_op_1000_179c(0xb4, in_DX, 0x1000);
        puVar4 = (uchar *)((uint)in_DX | in_AX);
        if(puVar4 == (uchar *)0x0)
        {
            iVar3  = 0x0;
            puVar4 = (uchar *)0x0;
        }
        else
        {
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar3 = string_1040_8520((astruct_57 *)CONCAT22(in_DX, in_AX), *(ushort *)(param_1 + 0x6), 0x30, 0x2, 0x57b, 0x62a, puVar4, unaff_SS);
        }
        puStack12 = (undefined4 *)CONCAT22(puVar4, iVar3);
        ppcVar1   = (code **)((int)*puStack12 + 0x74);
        (**ppcVar1)(uVar5, iVar3, puVar4);
    }
    return;
}

void __stdcall16far set_win_tet_1020_1d2a(ushort param_1, ushort param_2, SEGPTR in_win_text_3, ushort param_4, INT16 in_dlg_id_5, HWND16 in_hwnd_6)

{
    GetDlgItem16(in_hwnd_6, in_dlg_id_5);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, in_win_text_3);
    return;
}

void __stdcall16far pass1_1020_1d8e(ulong param_1, ulong param_2)

{
    pt_in_rect_1010_4e08(*(ulong *)((int)param_1 + 0x8e), (ushort)param_2, (ushort)(param_2 >> 0x10), 0x1010);
    return;
}

BOOL16 __stdcall16far destroy_win_1020_1dea(HWND16 param_1)

{
    BOOL16 BVar1;
    WORD   WVar2;

    BVar1 = IsWindow16(param_1);
    if(BVar1 != 0x0)
    {
        WVar2 = GetWindowWord16((HWND16)s_tile2_bmp_1050_1538, -0xc);
        if(WVar2 == 0x175)
        {
            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
            return 0x0;
        }
    }
    return 0x1;
}


ushort __stdcall16far destroy_win_1020_1e1e(HWND16 param_1)

{
    BOOL16 BVar1;
    WORD   WVar2;

    BVar1 = IsWindow16(param_1);
    if(BVar1 != 0x0)
    {
        WVar2 = GetWindowWord16((HWND16)s_tile2_bmp_1050_1538, -0xc);
        if((WVar2 != 0x1) && (WVar2 != 0x175))
        {
            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
        }
    }
    return 0x1;
}

astruct_18 *__stdcall16far pass1_1020_1e54(astruct_18 *param_1, byte param_2)

{
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far window_op_1020_2642(astruct *param_1)

{
    astruct_664 *in_AX;
    uchar       *in_DX;
    uint         uVar1;
    int          iVar2;
    int          unaff_DI;
    undefined2   uVar3;
    ushort       unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2), *(ushort *)(iVar2 + 0x8), 0x1018);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = (uint)in_DX | (uint)in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1020_27b0(in_AX, (ushort)in_DX, *(ushort *)(iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_664 **)(iVar2 + 0xee) = in_AX;
        *(uint *)(iVar2 + 0xf0)         = uVar1;
        return;
    }
    *(undefined4 *)(iVar2 + 0xee) = 0x0;
    return;
}


void __stdcall16far pass1_1020_26a6(ULONG param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    undefined2  uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    puVar1 = (undefined4 *)*(uint *)((int)param_1 + 0xee);
    uVar2  = *(uint *)((int)param_1 + 0xf0);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void __stdcall16far pass1_1020_28fc(astruct_3 *param_1, ushort param_2)

{
    param_1->address_offset_field_0x0   = 0x2e4a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    cleanup_menu_ui_op_1020_795c(param_1, param_2);
    return;
}

void __stdcall16far pass1_1020_2a6a(ulong param_1, ushort param_2)

{
    get_win_ui_info_op_1020_7a50(param_1, param_2);
    pass1_1018_0ae8(*(ulong *)((int)param_1 + 0xf2), 0x0);
    destroy_icon_1020_2c88(param_1, 0x1018);
    return;
}

void __stdcall16far bring_window_to_top_1020_2aae(ulong param_1, ulong param_2)

{
    u16 unaff_SS;

    pass1_1008_3e0e(param_1);
    BringWindowToTop16(0x1008);
    pass1_1018_169e(*(ulong *)((int)param_1 + 0xf2), param_2, unaff_SS);
    return;
}

void __stdcall16far pass1_1020_0aa6(ulong param_1, ushort param_2)

{
    win_ui_palette_op_1020_0cd2(*(undefined4 *)((int)param_1 + 0xe2), param_2);
    return;
}

void __stdcall16far win_ui_palette_op_1020_0cd2(undefined4 param_1, HWND16 param_2)

{
    uint        uVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    undefined4  uVar4;
    uint        uVar5;
    HDC16       hdc;
    HDC16       b_force_background;
    HPALETTE16  b_force_background_00;
    UINT16      UVar6;
    uint        extraout_DX;
    int         iVar7;
    undefined2  uVar8;
    astruct_13 *paStack10;
    uint        uStack6;

    uVar4   = *(undefined4 *)((int)param_1 + 0x6);
    uVar8   = (undefined2)((ulong)uVar4 >> 0x10);
    iVar7   = (int)uVar4;
    puVar2  = (undefined4 *)*(undefined4 *)(iVar7 + 0xa);
    uVar1   = *(uint *)(iVar7 + 0xc);
    uStack6 = (uint)puVar2;
    uVar5   = uVar1 | uStack6;
    if(uVar5 != 0x0)
    {
        ppcVar3 = (code **)((int)*puVar2 + 0x14);
        (**ppcVar3)(param_2, uStack6, uVar1);
        paStack10 = (astruct_13 *)CONCAT22(extraout_DX, uVar5);
        uVar5     = extraout_DX | uVar5;
        if(uVar5 != 0x0)
        {
            hdc                = GetDC16(param_2);
            b_force_background = hdc;
            create_palette_1008_4e38(paStack10, 0x1008, uVar5);
            b_force_background_00 = SelectPalette16(0x1008, 0x0, b_force_background);
            UVar6                 = RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
            SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x1, b_force_background_00);
            DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
            if(0x0 < (int)UVar6)
            {
                InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1), 0x0);
            }
            ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, hdc);
            return;
        }
    }
    return;
}

void __stdcall16far pass1_1020_0e2c(ulong param_1, ushort param_2)

{
    get_win_ui_info_op_1020_7a50(param_1, param_2);
    cleanup_ui_op_1020_1038(param_1);
    return;
}

void __stdcall16far pass1_1020_0e8e(int param_1, ushort param_2, int param_3, int param_4, int param_5, ushort param_6, ushort param_7)

{
    code **ppcVar1;

    win_ui_cursor_op_1020_1294(CONCAT22(param_2, param_1), param_3, param_4, param_6, param_7);
    if(param_5 == 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x4) + 0x5c);
        (**ppcVar1)();
    }
    return;
}

void __stdcall16far enable_menu_1020_1000(HMENU16 param_1, int param_2)

{
    if(param_2 != 0x0)
    {
        return;
    }
    EnableMenuItem16(param_1, 0x400, 0x3);
    return;
}

void __stdcall16far window_op_1020_10a0(astruct *param_1)

{
    undefined4   uVar1;
    code       **ppcVar2;
    astruct_160 *in_AX;
    uint         uVar3;
    BOOL16      *pBVar4;
    uchar       *in_DX;
    uchar       *puVar5;
    uchar       *puVar6;
    uchar       *extraout_DX;
    undefined2   extraout_DX_00;
    int          unaff_DI;
    ushort       unaff_SS;
    uchar        in_AF;
    ushort      *puVar7;
    ulong        uVar8;
    undefined2   uVar9;
    undefined   *puVar10;
    int          iVar11;
    undefined2   uVar12;

    iVar11 = (int)param_1;
    uVar12 = (undefined2)((ulong)param_1 >> 0x10);
    create_window_ex_1008_9760(param_1, 0x1008);
    mem_op_1000_179c(0x42, in_DX, 0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)in_AX);
    if(puVar5 != (uchar *)0x0)
    {
        pass1_1008_3bd6(in_AX, (ushort)in_DX, 0x0, 0x1f009b, 0x0, 0x740075, CONCAT22(*(undefined2 *)(iVar11 + 0x8), 0xf1), (ushort)puVar5, unaff_SS);
    }
    mem_op_1000_179c(0x42, puVar5, 0x1000);
    puVar6 = (uchar *)((uint)puVar5 | (uint)in_AX);
    if(puVar6 != (uchar *)0x0)
    {
        pass1_1008_3bd6(in_AX, (ushort)puVar5, 0x0, 0x31009b, 0x0, 0x760077, CONCAT22(*(undefined2 *)(iVar11 + 0x8), 0xf2), (ushort)puVar6, unaff_SS);
    }
    mem_op_1000_179c(0x42, puVar6, 0x1000);
    puVar5 = (uchar *)((uint)puVar6 | (uint)in_AX);
    if(puVar5 != (uchar *)0x0)
    {
        pass1_1008_3bd6(in_AX, (ushort)puVar6, 0x0, 0x77009b, 0x0, 0x780079, CONCAT22(*(undefined2 *)(iVar11 + 0x8), 0xf3), (ushort)puVar5, unaff_SS);
    }
    puVar7                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2d, unaff_SS, puVar5, unaff_DI);
    uVar9                          = (undefined2)((ulong)puVar7 >> 0x10);
    *(undefined2 *)(iVar11 + 0xf2) = (int)puVar7;
    *(undefined2 *)(iVar11 + 0xf4) = uVar9;
    *(undefined2 *)(iVar11 + 0xe0) = *(undefined2 *)(iVar11 + 0xf2);
    *(undefined2 *)(iVar11 + 0xe2) = uVar9;
    puVar10                        = PTR_LOOP_1050_038c;
    uVar3                          = LoadIcon16(0x1010, (LPCSTR)s_PLNTICON_1050_4267);
    *(HICON16 *)(iVar11 + 0xc2)    = uVar3;
    uVar1                          = *(undefined4 *)(iVar11 + 0xf2);
    ppcVar2                        = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xf2) + 0x30);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)uVar1, (int)((ulong)uVar1 >> 0x10), uVar3, puVar10);
    puVar5 = extraout_DX;
    mem_op_1000_179c(0x24, extraout_DX, 0x1000);
    puVar6 = (uchar *)((uint)puVar5 | uVar3);
    if(puVar6 == (uchar *)0x0)
    {
        *(undefined4 *)(iVar11 + 0xf6) = 0x0;
    }
    else
    {
        unk_win_ui_op_1020_1418((astruct_40 *)CONCAT22(puVar5, uVar3), (ULONG)param_1, unaff_SS);
        *(uint *)(iVar11 + 0xf6)   = uVar3;
        *(uchar **)(iVar11 + 0xf8) = puVar6;
    }
    *(undefined4 *)(iVar11 + 0xe8) = *(undefined4 *)(iVar11 + 0xf6);
    puVar7                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, unaff_SS, puVar6, unaff_DI);
    uVar8                          = pass1_1018_04b8((ulong)puVar7);
    puVar5                         = (uchar *)(uVar8 >> 0x10);
    pass1_1010_41d6(*(ulong *)(iVar11 + 0xf2), uVar8, puVar5, unaff_SS, in_AF);
    uVar8   = pass1_1010_451a(*(ulong *)(iVar11 + 0xf2), puVar5, unaff_DI, unaff_SS);
    pBVar4  = (BOOL16 *)uVar8;
    uVar1   = *(undefined4 *)param_1;
    ppcVar2 = (code **)((int)uVar1 + 0x14);
    (**ppcVar2)(0x1010, iVar11, uVar12, 0x0, pBVar4, (char)(uVar8 >> 0x10));
    uVar9   = 0x1;
    ppcVar2 = (code **)((int)uVar1 + 0x10);
    (**ppcVar2)();
    pass1_1010_459e(*(long *)(iVar11 + 0xf2));
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xf2) + 0x10);
    (**ppcVar2)(0x1010, *(undefined4 *)(iVar11 + 0xf2), param_1, uVar9);
    MoveWindow16(0x1010, 0x1, pBVar4[0x3], pBVar4[0x2], pBVar4[0x1], *pBVar4);
    UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
    return;
}

void __stdcall16far win_ui_cursor_op_1020_1294(ulong param_1, int param_2, int param_3, ushort param_4, ushort param_5)

{
    code      **ppcVar1;
    uint        in_AX;
    HCURSOR16   HVar2;
    HCURSOR16   HVar3;
    int         iVar4;
    undefined2  uVar5;
    ulong       uVar6;
    int         local_12;
    int         local_10;
    ushort     *puStack14;
    undefined4 *puStack10;
    int         local_6;
    int         iStack4;

    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x4000001);
    if((param_4 | in_AX) == 0x0)
    {
        local_6   = param_3;
        iStack4   = param_2;
        uVar5     = (undefined2)(param_1 >> 0x10);
        iVar4     = (int)param_1;
        puStack10 = (undefined4 *)pass1_1010_40cc(*(ulong *)(iVar4 + 0xf2), param_2, 0x0);
        uVar6     = *(ulong *)(iVar4 + 0xf2);
        puStack14 = (ushort *)(uVar6 & 0xffff0000 | (ulong)((int)uVar6 + 0x76));
        pass1_1008_3e94(puStack14, (ushort *)CONCAT22(param_5, &local_12), (ushort *)CONCAT22(param_5, &local_10));
        local_6 = local_6 - local_10;
        iStack4 = iStack4 - local_12;
        iVar4   = pt_in_rect_1010_40f8(*(ulong *)(iVar4 + 0xf2), (POINT16 *)CONCAT22(param_5, &local_6), 0x1010);
        if(iVar4 != -0x1)
        {
            uVar6   = 0x0;
            HVar2   = LoadCursor16(0x1010, (LPCSTR)0x7f02);
            uVar6   = uVar6 & 0xffff0000 | (ulong)HVar2;
            HVar3   = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
            ppcVar1 = (code **)((int)*puStack10 + 0x4);
            (**ppcVar1)((int)s_tile2_bmp_1050_1538, (int)puStack10, (int)((ulong)puStack10 >> 0x10), iVar4, iVar4 >> 0xf, iVar4, 0x2, uVar6, HVar3, HVar2);
            pass1_1008_3e0e(param_1);
            SetCursor16(0x1008);
        }
    }
    return;
}


astruct_3 *__stdcall16far pass1_1020_135e(astruct_3 *param_1, byte param_2, ushort param_3)

{
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1020_1418(astruct_40 *param_1, ULONG param_2, UINT16 param_3)

{
    undefined4  uVar1;
    astruct_13 *paVar2;
    code      **ppcVar3;
    HDC16      *pHVar4;
    undefined4 *puVar5;
    uchar      *puVar6;
    uchar      *extraout_DX;
    astruct_40 *iVar5;
    int         unaff_DI;
    undefined2  uVar7;
    undefined2  unaff_CS;
    ushort     *puVar8;
    HDC16       local_8;
    ushort     *puStack6;

    get_sys_metrics_1020_7c1a((ushort *)param_1, param_2, unaff_CS);
    uVar7                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar5                             = (astruct_40 *)param_1;
    *(undefined4 *)&iVar5->field_0x14 = 0x0;
    iVar5->field_0x18                 = (undefined4 *)0x0;
    puVar8                            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x1e));
    *(undefined2 *)param_1            = 0x1730;
    iVar5->field_0x2                  = 0x1020;
    puVar8                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2d, param_3, (uchar *)((ulong)puVar8 >> 0x10), unaff_DI);
    puVar6                            = (uchar *)((ulong)puVar8 >> 0x10);
    iVar5->field_0x14                 = (int)puVar8;
    *(uchar **)&iVar5->field_0x16     = puVar6;
    puStack6                          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_3, puVar6, unaff_DI);
    uVar1                             = *(undefined4 *)&iVar5->field_0x14;
    ppcVar3                           = (code **)((int)**(undefined4 **)&iVar5->field_0x14 + 0x4);
    (**ppcVar3)(0x1010, (int)uVar1, (int)((ulong)uVar1 >> 0x10), 0x0, param_1);
    local_8                       = GetDC16(0x1010);
    uVar1                         = *(undefined4 *)&iVar5->field_0x14;
    *(HDC16 *)((int)uVar1 + 0x7c) = local_8;
    uVar1                         = *(undefined4 *)&iVar5->field_0x14;
    puVar5                        = (undefined4 *)*(ulong *)((int)uVar1 + 0x66);
    iVar5->field_0x18             = puVar5;
    ppcVar3                       = (code **)((int)*iVar5->field_0x18 + 0x14);
    (**ppcVar3)();
    paVar2 = *(astruct_13 **)((int)puStack6 + 0xe);
    puVar6 = extraout_DX;
    pass1_1008_4d84((astruct_90 *)((ulong)puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10), (ulong)paVar2, extraout_DX);
    pHVar4            = (HDC16 *)palette_op_1008_4e08(paVar2, &local_8, (ushort)puVar6, 0x1008);
    iVar5->field_0x1c = pHVar4;
    return;
}


void __stdcall16far win_ui_op_1020_150e(undefined2 *param_1, HDC16 param_2)

{
    HPALETTE16 HVar1;
    int        iVar2;
    uint       uVar3;
    ushort     unaff_SS;

    uVar3                        = (uint)((ulong)param_1 >> 0x10);
    iVar2                        = (int)param_1;
    *param_1                     = 0x1730;
    *(undefined2 *)(iVar2 + 0x2) = 0x1020;
    if(*(long *)(iVar2 + 0x14) != 0x0)
    {
        param_2 = 0x1010;
        pass1_1010_1ea6(*(ulong *)(iVar2 + 0x14), (ulong)param_1 & 0xffff | (ulong)uVar3 << 0x10, unaff_SS);
    }
    HVar1                         = SelectPalette16(param_2, 0x0, *(BOOL16 *)(iVar2 + 0x1c));
    *(HPALETTE16 *)(iVar2 + 0x1c) = HVar1;
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    *param_1                     = 0x3ab0;
    *(undefined2 *)(iVar2 + 0x2) = 0x1008;
    *param_1                     = 0x389a;
    *(undefined2 *)(iVar2 + 0x2) = 0x1008;
    return;
}

astruct_18 *__stdcall16far pass1_1020_170a(astruct_18 *param_1, byte param_2, undefined2 param_3)

{
    win_ui_op_1020_150e(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1020_1780(ulong *param_1)

{
    code **ppcVar1;

    ppcVar1 = (code **)((int)*param_1 + 0x6c);
    (**ppcVar1)();
    destroy_win_1040_8212((ULONG)param_1, (HWND16)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far mixed_ui_op_1020_179c(astruct_1 *param_1)

{
    ulong       uVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    ushort      uVar4;
    INT16       IVar5;
    undefined  *puVar6;
    uchar      *in_DX;
    uchar      *extraout_DX;
    uchar      *puVar7;
    uint        uVar8;
    int         iVar9;
    int         iVar10;
    int         unaff_DI;
    undefined2  uVar11;
    undefined2  uVar12;
    undefined2  uVar13;
    WNDCLASS16 *unaff_SS;
    ushort     *puVar14;
    WNDCLASS16 *in_resc_id_3;
    WNDCLASS16 *in_buffer_4;
    WNDCLASS16  local_178[0xc];
    undefined4  uStack118;
    undefined4  uStack114;
    RECT16      local_6e;
    undefined4  uStack106;
    HWND16      HStack102;
    int         iStack98;
    int         iStack94;
    uint        uStack78;
    uchar      *puStack76;
    undefined4  uStack74;
    HWND16      HStack70;
    undefined4  uStack68;
    undefined4  uStack64;
    LPVOID      pvStack60;
    ushort      uStack58;
    undefined2  uStack56;
    ULONG      *pUStack54;
    undefined4  uStack50;
    undefined   local_2e[0x12];
    RECT16      local_1c;
    undefined4  uStack24;
    int         iStack20;
    int         iStack18;
    ushort     *puStack16;
    INT16      *pIStack12;
    uint        uStack8;
    ushort     *puStack6;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    uVar4    = 0x89;
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, (ushort)unaff_SS, in_DX, unaff_DI);
    puVar7   = (uchar *)((ulong)puStack6 >> 0x10);
    uVar4    = pass1_1010_65d0((ushort)unaff_SS, (ulong)puStack6, uVar4);
    uStack8  = (uint)(uVar4 == 0x0);
    uVar4    = pass1_1010_65d0((ushort)unaff_SS, (ulong)puStack6, 0x86);
    if(uVar4 != 0x0)
    {
        uStack8 = 0x0;
    }
    puVar14                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, (ushort)unaff_SS, puVar7, unaff_DI);
    uVar12                        = (undefined2)((ulong)puVar14 >> 0x10);
    uVar8                         = (uint)puVar14;
    uVar11                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar9                         = (int)param_1;
    *(uint *)(iVar9 + 0x8e)       = uVar8;
    *(undefined2 *)(iVar9 + 0x90) = uVar12;
    ppcVar2                       = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar9 + 0x8e) + 0x10);
    (**ppcVar2)(0x1010, *(undefined2 *)(iVar9 + 0x8e), uVar12, uStack8);
    puStack76 = extraout_DX;
    mem_op_1000_179c(0x12, extraout_DX, 0x1000);
    puVar7   = (uchar *)((uint)puStack76 | uVar8);
    uStack78 = uVar8;
    if(puVar7 == (uchar *)0x0)
    {
        *(undefined4 *)(iVar9 + 0x92) = 0x0;
    }
    else
    {
        pass1_1020_1eea((ushort *)CONCAT22(puStack76, uVar8), (ulong)param_1, *(ushort *)(iVar9 + 0x6), puVar7, unaff_DI, (ushort)unaff_SS);
        *(uint *)(iVar9 + 0x92)   = uVar8;
        *(uchar **)(iVar9 + 0x94) = puVar7;
    }
    uVar1     = *(ulong *)(iVar9 + 0x8e);
    pIStack12 = (INT16 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0xa));
    puStack16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, (ushort)unaff_SS, puVar7, unaff_DI);
    GetClientRect16(0x1010, &local_1c);
    IVar5                  = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    uVar12                 = (undefined2)((ulong)pIStack12 >> 0x10);
    iVar10                 = (int)pIStack12;
    *(int *)(iVar10 + 0x6) = IVar5 + uStack24._2_2_;
    uVar13                 = (undefined2)((ulong)puStack16 >> 0x10);
    iStack18               = *(int *)((int)puStack16 + 0xa);
    iStack20               = *(int *)((int)puStack16 + 0xc);
    *(int *)(iVar10 + 0x2) = (iStack20 - *(int *)(iVar10 + 0x6)) / 0x2;
    iVar10                 = iStack18 - *(int *)(iVar10 + 0x4);
    uVar8                  = iVar10 >> 0xf;
    *pIStack12             = iVar10 / 0x2;
    pass1_1028_dc52((astruct_92 *)CONCAT22(unaff_SS, local_2e), 0x1, 0x0, 0x100);
    uStack56 = 0x0;
    while(true)
    {
        puVar6 = local_2e;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar6));
        uStack50 = CONCAT22(uVar8, puVar6);
        uStack58 = uVar8 | (uint)puVar6;
        if(uStack58 == 0x0)
            break;
        pUStack54 = *(ULONG **)(puVar6 + 0x10);
        uVar8     = uStack58;
        if(pUStack54 != (ULONG *)0x0)
        {
            pass1_1000_3cea((ulong)param_1 & 0xffff0000 | (ulong)(iVar9 + 0x10), *pUStack54);
            uVar8 = uStack58;
        }
    }
    uVar4                     = pass1_1020_1da8((ulong)param_1, puVar6, 0x0, unaff_SS);
    *(ushort *)(iVar9 + 0x96) = uVar4;
    uVar4                     = pass1_1010_65d0((ushort)unaff_SS, (ulong)puStack6, 0x86);
    if((uVar4 == 0x0) || (*(int *)(iVar9 + 0x96) != 0x0))
    {
        uVar3                              = *(undefined4 *)(iVar9 + 0x8e);
        *(undefined2 *)((int)uVar3 + 0x2c) = 0x0;
        HStack102                          = GetDlgItem16(0x1010, 0x175);
        if(uStack8 != 0x0)
        {
            load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x100, (char *)local_178, (short)unaff_SS);
            SetWindowText16(0x1010, (SEGPTR)local_178);
        }
        pvStack60 = MakeProcInstance16((LPVOID)s_tile2_bmp_1050_1538, (HANDLE16)PTR_LOOP_1050_038c);
        GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_6e);
        uStack114       = uStack106;
        iStack98        = (int)uStack106 - local_6e.x;
        iStack94        = uStack106._2_2_ - local_6e.y;
        uStack118       = local_6e & 0xffff0000 | (ulong)(uint)(-(iStack98 - *(int *)((int)pIStack12 + 0x4)) / 0x2);
        IVar5           = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
        uVar1           = uStack118 & 0xffff;
        uStack118       = uVar1 | (ulong)(uint)(uStack118._2_2_ - IVar5) << 0x10;
        uStack118._0_2_ = (BOOL16)uVar1;
        MoveWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0, iStack94, iStack98, uStack118._2_2_ - IVar5, (BOOL16)uStack118);
    }
    else
    {
        win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x9d0001, unaff_SS, uVar4, uStack58);
        *(ushort *)(iVar9 + 0x8c) = uVar4;
        pvStack60                 = MakeProcInstance16((LPVOID)0x1008, (HANDLE16)PTR_LOOP_1050_038c);
    }
    EnumChildWindows1((HWND16)s_tile2_bmp_1050_1538, (LPVOID)0x0, ZEXT24(pvStack60) << 0x10);
    FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
    HStack70 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_1c);
    uStack64   = uStack24;
    local_1c.x = (int)uStack24 - local_1c.x;
    uStack74   = CONCAT22(local_1c.x, uStack24._2_2_ - local_1c.y);
    uStack68   = local_1c & 0xffff0000 | (ulong)(uint)(-(local_1c.x - *(int *)((int)pIStack12 + 0x4)) / 0x2);
    IVar5      = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    uStack68   = uStack68 & 0xffff | (ulong)(uint)(uStack68._2_2_ - IVar5) << 0x10;
    if(*(int *)(iVar9 + 0x96) == 0x0)
    {
        if(uStack8 == 0x0)
            goto LAB_1020_1b24;
        in_buffer_4  = local_178;
        in_resc_id_3 = (WNDCLASS16 *)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21);
    }
    else
    {
        load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x100, (char *)local_178, (short)unaff_SS);
        GetDlgItem16(0x1010, 0x175);
        SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)local_178);
        in_resc_id_3 = local_178;
        in_buffer_4  = unaff_SS;
        unaff_SS     = (WNDCLASS16 *)0x3fe;
    }
    load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), (ushort)in_resc_id_3, (char *)in_buffer_4, (short)unaff_SS);
    SetWindowText16(0x1010, (SEGPTR)local_178);
LAB_1020_1b24:
    MoveWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0, (INT16)uStack74, (INT16)((ulong)uStack74 >> 0x10), uStack68._2_2_, (BOOL16)uStack68);
    uVar12 = (undefined2)((ulong)pIStack12 >> 0x10);
    iVar9  = (int)pIStack12;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x44, *(INT16 *)(iVar9 + 0x6), *(INT16 *)(iVar9 + 0x4), *(INT16 *)(iVar9 + 0x2), *pIStack12, 0x0);
    return;
}

void __stdcall16far pass1_1018_5e5a(ushort *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0x6128;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c((astruct_18 *)param_1, (int)&PTR_LOOP_1050_1040);
    return;
}

void __stdcall16far win_ui_op_1018_5e9a(astruct_1 *param_1, ushort param_2)

{
    ulong      uVar1;
    ULONG     *pUVar2;
    INT16      IVar3;
    undefined *puVar4;
    uchar     *in_DX;
    uchar     *puVar5;
    uchar     *puVar6;
    uint       uVar7;
    uint       uVar8;
    int        iVar9;
    int        unaff_DI;
    undefined2 uVar10;
    ushort    *puVar11;
    undefined  local_28[0x12];
    int        iStack22;
    undefined2 uStack20;
    int        iStack18;
    int        iStack16;
    RECT16     local_e;
    int        iStack8;
    INT16     *pIStack6;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    puVar11                   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, param_2, in_DX, unaff_DI);
    puVar5                    = (uchar *)((ulong)puVar11 >> 0x10);
    uVar7                     = (uint)puVar11;
    uVar10                    = (undefined2)((ulong)param_1 >> 0x10);
    iVar9                     = (int)param_1;
    *(uint *)(iVar9 + 0x8e)   = uVar7;
    *(uchar **)(iVar9 + 0x90) = puVar5;
    mem_op_1000_179c(0x12, puVar5, 0x1000);
    puVar6 = (uchar *)((uint)puVar5 | uVar7);
    if(puVar6 == (uchar *)0x0)
    {
        *(undefined4 *)(iVar9 + 0x92) = 0x0;
    }
    else
    {
        pass1_1018_6198((ushort *)CONCAT22(puVar5, uVar7), (ulong)param_1, *(ushort *)(iVar9 + 0x6), puVar6, unaff_DI, param_2);
        *(uint *)(iVar9 + 0x92)   = uVar7;
        *(uchar **)(iVar9 + 0x94) = puVar6;
    }
    uVar1    = *(ulong *)(iVar9 + 0x8e);
    pIStack6 = (INT16 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0xa));
    GetClientRect16(0x1000, &local_e);
    IVar3                         = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    *(int *)((int)pIStack6 + 0x6) = IVar3 + iStack8;
    puVar11                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_2, puVar6, unaff_DI);
    uStack20                      = (undefined2)((ulong)puVar11 >> 0x10);
    iStack22                      = (int)puVar11;
    iStack16                      = *(int *)(iStack22 + 0xa);
    iStack18                      = *(int *)(iStack22 + 0xc);
    uVar10                        = (undefined2)((ulong)pIStack6 >> 0x10);
    *(int *)((int)pIStack6 + 0x2) = (iStack18 - *(int *)((int)pIStack6 + 0x6)) / 0x2;
    uVar7                         = iStack16 >> 0xf;
    *pIStack6                     = iStack16 / 0x2 + 0x3;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, local_28), 0x1, 0x0, 0x100);
    while(true)
    {
        puVar4 = local_28;
        pass1_1028_e4ec(CONCAT22(param_2, puVar4));
        uVar8 = uVar7 | (uint)puVar4;
        if(uVar8 == 0x0)
            break;
        pUVar2 = *(ULONG **)(puVar4 + 0x10);
        uVar7  = uVar8;
        if(pUVar2 != (ULONG *)0x0)
        {
            pass1_1000_3cea((ulong)param_1 & 0xffff0000 | (ulong)(iVar9 + 0x10), *pUVar2);
            uVar7 = uVar8;
        }
    }
    uVar10 = (undefined2)((ulong)pIStack6 >> 0x10);
    iVar9  = (int)pIStack6;
    SetWindowPos16((HWND16)&USHORT_1050_1028, 0x44, *(INT16 *)(iVar9 + 0x6), *(INT16 *)(iVar9 + 0x4), *(INT16 *)(iVar9 + 0x2), *pIStack6, 0x0);
    return;
}

void __stdcall16far mix_ui_op_1018_6adc(astruct_28 *param_1)

{
    int         iVar1;
    int         iVar2;
    ushort      uVar3;
    uchar      *in_DX;
    ushort      uVar4;
    int         iVar5;
    int         unaff_DI;
    undefined2  uVar6;
    WNDCLASS16 *unaff_SS;
    ushort     *puVar7;
    astruct_43 *paVar8;

    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, (ushort)unaff_SS, in_DX, unaff_DI);
    uVar4  = (ushort)((ulong)puVar7 >> 0x10);
    iVar1  = *(int *)((int)puVar7 + 0xa);
    iVar2  = *(int *)((int)puVar7 + 0xc);
    uVar6  = (undefined2)((ulong)param_1 >> 0x10);
    iVar5  = (int)param_1;
    if(0x1 < iVar1 - *(int *)(iVar5 + 0xe6))
    {
        uVar4                  = iVar1 >> 0xf;
        *(int *)(iVar5 + 0xe2) = iVar1 / 0x2 - (*(int *)(iVar5 + 0xe6) + 0x1) / 0x2;
    }
    if(0x1 < iVar2 - *(int *)(iVar5 + 0xe8))
    {
        uVar4                  = iVar2 >> 0xf;
        *(int *)(iVar5 + 0xe4) = iVar2 / 0x2 - (*(int *)(iVar5 + 0xe8) + 0x1) / 0x2;
    }
    uVar3 = ShowCursor16(0x1010);
    if(*(int *)(iVar5 + 0xee) != 0x0)
    {
        win_1008_5c5c(unaff_SS, uVar3, uVar4, _PTR_LOOP_1050_02a0, *(ushort *)(iVar5 + 0xee));
        *(ushort *)(iVar5 + 0xf0) = uVar3;
    }
    paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, *(ushort *)(iVar5 + 0xec), (ushort)unaff_SS);
    mci_send_command_1008_53ae((ulong)paVar8, *(ushort *)(iVar5 + 0x8), 0x1008, (ushort)unaff_SS);
    ShowCursor16(0x1008);
    unk_destroy_window_op_1018_6bb6(param_1, (int)s_tile2_bmp_1050_1538);
    return;
}

astruct_11 *__stdcall16far pass1_1018_4ae0(astruct_11 *param_1, byte param_2, ushort param_3)

{
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far unk_win_ui_op_1018_4f18(astruct_39 *param_1, UINT16 param_2, ulong param_3)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    RECT16     *rect;
    int         iVar3;
    ulong       uVar4;
    uchar      *extraout_DX;
    uchar      *puVar5;
    uchar      *extraout_DX_00;
    uchar      *puVar6;
    uint        uVar7;
    astruct_39 *iVar6;
    undefined2  uVar8;
    undefined2  uVar9;
    ushort      unaff_SS;
    astruct_76 *paStack22;
    RECT16      local_12;
    int         iStack14;
    int         iStack12;
    ulong       uStack10;
    astruct_43 *paStack6;

    paStack6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, param_2, unaff_SS);
    uVar4    = (ulong)paStack6 & 0xffff;
    ppcVar1  = (code **)((int)*(undefined4 *)paStack6 + 0x14);
    (**ppcVar1)(0x1010, (int)uVar4, (int)((ulong)paStack6 >> 0x10));
    puVar2   = (undefined4 *)uVar4;
    uStack10 = uVar4 & 0xffff | ZEXT24(extraout_DX) << 0x10;
    uVar8    = (undefined2)((ulong)param_1 >> 0x10);
    iVar6    = (astruct_39 *)param_1;
    puVar5   = extraout_DX;
    if(*(long *)&iVar6->field_0xe != 0x0)
    {
        uVar7  = iVar6->field_0x10;
        puVar2 = (undefined4 *)*(undefined4 *)&iVar6->field_0xe;
        puVar5 = (uchar *)(uVar7 | (uint)puVar2);
        if(puVar5 != (uchar *)0x0)
        {
            ppcVar1 = (code **)*puVar2;
            (**ppcVar1)();
            puVar5 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar5, 0x1000);
    puVar6 = (uchar *)((uint)puVar5 | (uint)puVar2);
    if(puVar6 == (uchar *)0x0)
    {
        puVar2 = (undefined4 *)0x0;
        puVar6 = (uchar *)0x0;
    }
    else
    {
        struct_1008_4c58((ushort *)CONCAT22(puVar5, puVar2));
    }
    iVar6->field_0xe  = puVar2;
    iVar6->field_0x10 = (uint)puVar6;
    pass1_1008_4d84(*(astruct_90 **)&iVar6->field_0xe, uStack10, puVar6);
    rect = &local_12;
    GetClientRect16(0x1008, rect);
    uVar9 = 0x1000;
    mem_op_1000_179c(0x1e, puVar6, 0x1000);
    paStack22 = (astruct_76 *)CONCAT22(puVar6, rect);
    uVar7     = (uint)puVar6 | (uint)rect;
    if(uVar7 == 0x0)
    {
        *(undefined4 *)&iVar6->field_0xa = 0x0;
    }
    else
    {
        iVar3 = (iStack12 - local_12.y) + 0x1;
        uVar9 = 0x1008;
        pass1_1008_405c(paStack22, *(ulong *)&iVar6->field_0xe, iVar3, (iStack14 - local_12.x) + 0x1);
        iVar6->field_0xa = iVar3;
        iVar6->field_0xc = uVar7;
    }
    if(paStack6 != (astruct_43 *)0x0)
    {
        ppcVar1 = (code **)*(undefined4 *)paStack6;
        (**ppcVar1)(uVar9, (int)paStack6, (int)((ulong)paStack6 >> 0x10), 0x1);
    }
    return;
}


astruct_11 *__stdcall16far pass1_1018_5032(astruct_11 *param_1, byte param_2, ushort param_3)

{
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1018_57e6(ulong param_1, long param_2, ushort param_3)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    send_dlg_item_msg_1040_d20c(*(ulong *)((int)param_1 + 0xa), param_2, (ushort)&PTR_LOOP_1050_1040, param_3);
    *(undefined4 *)((int)param_1 + 0xa) = 0x0;
    return;
}

void __stdcall16far pt_in_rect_1018_1bda(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int       *piVar1;
    undefined2 uVar2;
    int        iVar3;
    BOOL16     BVar4;
    int        iVar5;
    undefined2 uVar6;
    int        iStack26;
    POINT16    PStack24;
    int        local_14;
    int        local_12;
    undefined2 uStack16;
    ulong      uStack14;
    int        local_a;
    int        local_8;
    int        iStack6;
    int        iStack4;

    uStack14 = 0x0;
    iVar3    = (int)param_1;
    pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar3 + 0x3a)), (ushort *)CONCAT22(param_4, &local_14), (ushort *)CONCAT22(param_4, &local_12));
    PStack24 = (POINT16)CONCAT22(param_2, param_3);
    uStack16 = 0x0;
    iStack26 = 0x0;
    while(true)
    {
        uVar6  = (undefined2)(param_1 >> 0x10);
        piVar1 = (int *)(iVar3 + 0x44);
        if(*piVar1 == iStack26 || *piVar1 < iStack26)
        {
            return;
        }
        uVar2    = *(undefined2 *)(iVar3 + 0x42);
        iVar5    = *(int *)(iVar3 + 0x40) + iStack26 * 0x18;
        uStack14 = CONCAT22(uVar2, iVar5);
        pass1_1008_3e94((ushort *)CONCAT22(uVar2, iVar5), (ushort *)CONCAT22(param_4, &local_8), (ushort *)CONCAT22(param_4, &local_a));
        local_a = local_a + local_12 + -0x6;
        iStack6 = local_a + 0xc;
        local_8 = local_8 + local_14 + -0x6;
        iStack4 = local_8 + 0xc;
        BVar4   = PtInRect16((RECT16 *)0x1008, PStack24);
        if(BVar4 != 0x0)
            break;
        iStack26 = iStack26 + 0x1;
    }
    pass1_1018_1eda(param_1, uStack14, param_4);
    return;
}
void __stdcall16far pass1_1018_2440(astruct_11 *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    int         *piVar4;
    undefined2   uVar6;
    astruct_502 *uVar5;
    undefined2   uVar7;
    undefined2  *puStack6;

    uVar7                  = (undefined2)((ulong)param_1 >> 0x10);
    uVar5                  = (astruct_502 *)param_1;
    *(undefined2 *)param_1 = 0x2ada;
    uVar5->field_0x2       = 0x1018;
    uVar5->field_0x1c      = (int)s_fem132_wav_1050_2aec + 0x6;
    uVar5->field_0x1e      = 0x1018;
    if(_PTR_LOOP_1050_0388 != 0x0)
    {
        if(param_1 == (astruct_11 *)0x0)
        {
            piVar4 = (int *)0x0;
            uVar6  = 0x0;
        }
        else
        {
            piVar4 = &uVar5->field_0x1c;
            uVar6  = uVar7;
        }
        param_2 = 0x1008;
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x73, CONCAT22(uVar6, piVar4));
    }
    puVar1 = uVar5->field_0x2a;
    uVar2  = uVar5->field_0x2c;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(param_2, puVar1, uVar2, 0x1);
    }
    puVar1 = uVar5->field_0x6e;
    uVar2  = uVar5->field_0x70;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(param_2, puVar1, uVar2, 0x1);
    }
    if(param_1 == (astruct_11 *)0x0)
    {
        piVar4 = (int *)0x0;
        uVar7  = 0x0;
    }
    else
    {
        piVar4 = &uVar5->field_0x1c;
    }
    puStack6    = (undefined2 *)CONCAT22(uVar7, piVar4);
    *puStack6   = 0x389a;
    piVar4[0x1] = 0x1008;
    clenaup_win_ui_1018_4d22(param_1, param_2);
    return;
}

void __stdcall16far msg_box_op_1010_8bb4(ushort param_1, ushort param_2, ulong param_3, HINSTANCE16 param_4, ushort param_5)

{
    char     *pcVar1;
    undefined local_402[0x400];

    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), param_4);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_5, local_402), pcVar1);
    pass1_1000_3cea(CONCAT22(param_5, local_402), param_3);
    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1000);
    MessageBox16(0x1000, (LPCSTR)0x1010, (LPCSTR)pcVar1, (UINT16)((ulong)pcVar1 >> 0x10));
    PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x11100ee);
    return;
}

void __stdcall16far ui_op_1010_79aa(undefined4 param_1, int param_2, long param_3, undefined2 param_4)

{
    undefined4 uVar1;
    undefined *puVar2;
    uint       extraout_DX;
    undefined2 uVar3;
    long       lStack18;
    long       lStack14;
    undefined  local_a[0x8];

    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    if((*(long *)((int)param_1 + 0x1c) != 0x0) && ((param_3 != 0x0 || (param_2 != 0x0))))
    {
        pass1_1008_5784((ulong *)CONCAT22(param_4, local_a), *(ulong *)((int)param_1 + 0x1c));
        lStack18 = 0x0;
        do
        {
            puVar2 = local_a;
            pass1_1008_5b12(puVar2, param_4);
            lStack14 = CONCAT22(extraout_DX, puVar2);
            if((extraout_DX | (uint)puVar2) == 0x0)
                goto LAB_1010_7a49;
            if(((param_2 == 0x0) && (*(long *)(puVar2 + 0x4) == param_3)) || ((param_3 == 0x0 && (uVar1 = *(undefined4 *)(puVar2 + 0x8), *(int *)((int)uVar1 + 0xa) == param_2))))
                break;
        } while((*(long *)(puVar2 + 0x4) != param_3) || (uVar1 = *(undefined4 *)(puVar2 + 0x8), *(int *)((int)uVar1 + 0xa) != param_2));
        lStack18 = lStack14;
    LAB_1010_7a49:
        if(lStack18 != 0x0)
        {
            SetFocus16(0x1008);
            BringWindowToTop16((HWND16)s_tile2_bmp_1050_1538);
            return;
        }
    }
    return;
}

void __stdcall16far show_win_1010_7a76(ulong param_1, ushort param_2)

{
    int        iVar1;
    undefined2 uVar2;
    undefined2 unaff_SS;
    long       lVar3;
    undefined  local_a[0x8];

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0x20) == 0x0)
    {
        *(undefined2 *)(iVar1 + 0x20) = 0x1;
        pass1_1008_5784((ulong *)CONCAT22(unaff_SS, local_a), *(ulong *)(iVar1 + 0x1c));
        while(true)
        {
            lVar3 = pass1_1008_5b12(local_a, unaff_SS);
            if(lVar3 == 0x0)
                break;
            ShowWindow16(0x1008, 0x0);
        }
    }
    return;
}

void __stdcall16far show_window_1010_7ace(ulong param_1, ushort param_2)

{
    int        iVar1;
    undefined2 uVar2;
    long       lVar3;
    undefined  local_a[0x8];

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0x20) != 0x0)
    {
        *(undefined2 *)(iVar1 + 0x20) = 0x0;
        pass1_1008_5784((ulong *)CONCAT22(param_2, local_a), *(ulong *)(iVar1 + 0x1c));
        while(true)
        {
            lVar3 = pass1_1008_5b12(local_a, param_2);
            if(lVar3 == 0x0)
                break;
            ShowWindow16(0x1008, 0x1);
        }
    }
    return;
}


ulong __stdcall16far destroy_window_1010_7b26(ulong param_1, long param_2, UINT16 param_3, uint param_4)

{
    uint   uVar1;
    UCHAR *puVar2;
    uint   extraout_DX;
    int    iVar2;
    UINT16 uVar4;
    UCHAR  local_a[0x8];

    uVar4 = (UINT16)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    uVar1 = *(uint *)(iVar2 + 0x1e) | *(uint *)(iVar2 + 0x1c);
    if(uVar1 != 0x0)
    {
        pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)(iVar2 + 0x1c));
        do
        {
            puVar2 = local_a;
            pass1_1008_5b12(puVar2, param_3);
            param_4 = extraout_DX | (uint)puVar2;
            if(param_4 == 0x0)
                break;
        } while(*(long *)(puVar2 + 0x4) != param_2);
        uVar1 = extraout_DX | (uint)puVar2;
        if(uVar1 != 0x0)
        {
            uVar1 = DestroyWindow16(0x1008);
        }
    }
    return CONCAT22(uVar1, param_4);
}

void __stdcall16far pass1_1010_8096(ulong *param_1, int param_2)

{
    ushort    uVar1;
    ushort    uVar2;
    ushort    uVar3;
    ushort    uVar4;
    ushort    unaff_SS;
    char     *pcVar5;
    ushort   *puVar6;
    undefined local_306[0x100];
    undefined local_206[0x100];
    undefined local_106[0x104];

    uVar4 = (ushort)((ulong)param_1 >> 0x10);
    uVar3 = (ushort)param_1;
    str_1000_4d58(*(char **)(*(int *)(uVar3 + 0xe82) * 0x4 + 0x2526), (char *)0x0, 0x0, CONCAT22(unaff_SS, local_206), (WNDCLASS16 *)CONCAT22(unaff_SS, local_306));
    unk_str_op_1000_3d3e((char *)CONCAT22(unaff_SS, local_106), (char *)CONCAT22(unaff_SS, local_206));
    if(param_2 == 0x2)
    {
        puVar6 = &USHORT_1050_3194;
    }
    else
    {
        puVar6 = &USHORT_1050_3196;
    }
    pass1_1000_3cea(CONCAT22(unaff_SS, local_106), (ULONG)puVar6);
    pass1_1000_3cea(CONCAT22(unaff_SS, local_106), CONCAT22(unaff_SS, local_306));
    pcVar5 = (char *)set_err_mode_1010_8b14((ulong)param_1, CONCAT22(unaff_SS, local_106), unaff_SS);
    uVar2  = (ushort)((ulong)pcVar5 >> 0x10);
    if(((undefined *)pcVar5 == local_106) && (uVar2 == unaff_SS))
    {
        msg_box_op_1010_8bb4(uVar3, uVar4, (ulong)pcVar5 & 0xffff | (ulong)uVar2 << 0x10, 0x1000, unaff_SS);
    }
    fn_ptr_1000_17ce((astruct_18 *)*param_1, 0x1000);
    uVar1                    = str_op_1008_60e8(pcVar5, uVar2);
    *(ushort *)param_1       = uVar1;
    *(ushort *)(uVar3 + 0x2) = uVar2;
    return;
}

astruct_43 *__stdcall16far unk_io_op_1010_830a(ulong param_1, ushort param_2, ushort param_3)

{
    uint        in_AX;
    undefined4 *puVar1;
    undefined4 *puVar2;
    uchar      *in_DX;
    uint        uVar3;
    astruct_45 *iVar3;
    astruct_44 *iVar2;
    int         iVar4;
    HINSTANCE16 unaff_CS;
    ushort      uVar5;
    ushort      uVar6;
    undefined4  local_2e;
    undefined4  uStack10;
    astruct_43 *paStack6;

    paStack6 = (astruct_43 *)0x0;
    iVar3    = (astruct_45 *)(param_2 * 0x10);
    uVar5    = (ushort)param_1;
    uVar6    = (ushort)(param_1 >> 0x10);
    if(iVar3->field_0x10 == 0x1)
    {
        uStack10       = (char *)set_err_mode_1010_8b14(param_1, *(ULONG *)&iVar3->field_0x12, param_3);
        uStack10._2_2_ = (ushort)((ulong)uStack10 >> 0x10);
        if((iVar3->field_0x12 == (int)uStack10) && (iVar3->field_0x14 == uStack10._2_2_))
        {
            msg_box_op_1010_8bb4(uVar5, uVar6, (ulong)uStack10, unaff_CS, param_3);
            return (astruct_43 *)0x0;
        }
        puVar1 = &local_2e;
        struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3, puVar1), 0x1, uStack10, uStack10._2_2_);
        mem_op_1000_179c(0x1e, (uchar *)((ulong)uStack10 >> 0x10), 0x1000);
        uVar3 = (uint)((ulong)uStack10 >> 0x10) | (uint)puVar1;
        if(uVar3 == 0x0)
        {
            puVar2 = (undefined4 *)0x0;
            uVar3  = 0x0;
        }
        else
        {
            puVar2 = &local_2e;
            struct_op_1008_3f92((astruct_76 *)((ulong)uStack10 & 0xffff0000 | ZEXT24(puVar1)), (astruct_83 *)CONCAT22(param_3, puVar2));
        }
        paStack6 = (astruct_43 *)CONCAT22(uVar3, puVar2);
        close_file_1008_496c(&local_2e, param_3);
        local_2e = paStack6;
    }
    else
    {
        if(*(int *)(param_2 * 0x10 + 0x10) == 0x2)
        {
            pass1_1010_878c((astruct_87 **)param_1, *(int *)(param_2 * 0x10 + 0x16), unaff_CS);
            if(*(long *)(uVar5 + 0x67c) == 0x0)
            {
                return (astruct_43 *)0x0;
            }
            iVar2 = (astruct_44 *)(param_2 * 0x10);
            pass1_1008_6562(*(ulong **)(uVar5 + 0x67c), CONCAT22(iVar2->field_0x1c, iVar2->field_0x1e), iVar2->field_0x1a, in_AX, in_DX);
            local_2e = (astruct_43 *)CONCAT22(in_DX, in_AX);
        }
        else
        {
            iVar4 = param_2 * 0x10;
            if(*(int *)(iVar4 + 0x10) == 0x3)
            {
                local_2e = (astruct_43 *)set_err_mode_1010_8b14(param_1, *(ULONG *)(iVar4 + 0x12), param_3);
                if((*(int *)(iVar4 + 0x12) == (int)local_2e) && (*(int *)(iVar4 + 0x14) == (int)((ulong)local_2e >> 0x10)))
                {
                    msg_box_op_1010_8bb4(uVar5, uVar6, (ulong)local_2e, unaff_CS, param_3);
                    local_2e = local_2e;
                }
            }
            else
            {
                local_2e = paStack6;
                if(*(int *)(param_2 * 0x10 + 0x10) == 0x4)
                {
                    local_2e = (astruct_43 *)set_err_mode_1010_8b14(param_1, *(ULONG *)(param_2 * 0x10 + 0x12), param_3);
                }
            }
        }
    }
    paStack6 = local_2e;
    return paStack6;
}

void __stdcall16far pass1_1010_71d6(ulong param_1, int param_2, ushort *param_3, uint param_4, uint param_5, ushort param_6)

{
    int        iVar1;
    undefined4 uVar2;
    uint       uVar3;
    int        iVar4;
    ushort     uVar5;
    ushort     uVar6;
    uint       uVar7;
    undefined2 uVar8;
    ulong      uVar9;
    ushort     uStack20;
    uint       uStack14;
    ulong      uStack6;

    uVar8 = (undefined2)(param_1 >> 0x10);
    uVar2 = *(undefined4 *)((int)param_1 + 0x14);
    pass1_1010_ad22((ushort)uVar2, param_5, param_6, (ushort)((ulong)uVar2 >> 0x10), *param_3);
    uStack6 = CONCAT22(param_5, param_4);
    if((param_5 | param_4) == 0x0)
    {
        return;
    }
    uVar9 = struct_op_1030_73a8(CONCAT22(param_5, param_4));
    uVar7 = (uint)(uVar9 >> 0x10);
    uVar3 = (uint)uVar9;
    if(((uVar7 | uVar3) != 0x0) && (*(long *)(uVar3 + 0x1c) == 0x8000002))
    {
        return;
    }
    uVar2    = *(undefined4 *)(param_4 + 0x2e);
    uStack14 = (uint)uVar2;
    if(((*(uint *)(param_4 + 0x30) | uStack14) != 0x0) && (*(long *)(uStack14 + 0x200) == 0x8000002))
    {
        return;
    }
    uVar2 = *(undefined4 *)((int)param_1 + 0x14);
    uVar5 = pass1_1010_b028((ushort)uVar2, (ushort)((ulong)uVar2 >> 0x10), uVar9);
    iVar1 = *(int *)(uVar3 + 0x12);
    iVar4 = iVar1;
    if((iVar1 != 0x4) && (iVar4 = param_2, iVar1 == 0x7))
    {
        param_2 = 0x5;
        iVar4   = param_2;
    }
    param_2 = iVar4;
    uVar6   = param_2 - 0x2;
    if(uVar6 != 0x0)
    {
        if(param_2 == 0x3)
        {
            uVar6 = uVar5 - 0xb;
            if((uVar6 == 0x0) || (uVar6 = uVar5 - 0x37, uVar6 == 0x0))
            {
                uStack20 = 0xb;
            }
            else
            {
                uStack20 = 0xa;
            }
            goto LAB_1010_72a7;
        }
        uVar6 = param_2 - 0x4;
        if(uVar6 == 0x0)
        {
            uStack20 = 0x17;
            goto LAB_1010_72a7;
        }
        uVar6 = param_2 - 0x5;
        if(uVar6 != 0x0)
        {
            uVar6    = pass1_1010_7818(param_1, uVar9);
            uStack20 = uVar6;
            goto LAB_1010_72a7;
        }
    }
    uStack20 = 0xc;
LAB_1010_72a7:
    if(uStack20 == 0x0)
    {
        return;
    }
    ui_op_1010_79aa(param_1, 0x0, uStack6, param_6);
    if(uVar6 == 0x0)
    {
        unk_win_op_1010_7300(param_1, 0x0, uStack20, uStack6);
    }
    return;
}

astruct_11 *__stdcall16far pass1_1010_5074(astruct_11 *param_1, byte param_2)

{
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_29c6(astruct_11 *param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_476 *iVar5;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar5            = (astruct_476 *)param_1;
    *(int *)param_1  = (int)s_add16_wav_1050_2bdc + 0x8;
    iVar5->field_0x2 = 0x1010;
    if(*(long *)&iVar5->field_0x1c != 0x0)
    {
        puVar1 = (undefined4 *)*(uint *)&iVar5->field_0x1c;
        uVar2  = iVar5->field_0x1e;
        if((uVar2 | (uint)puVar1) != 0x0)
        {
            ppcVar3 = (code **)*puVar1;
            (**ppcVar3)();
        }
        *(undefined4 *)&iVar5->field_0x1c = 0x0;
        fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x28, 0x1000);
        iVar5->field_0x28 = 0x0;
    }
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    return;
}

void __stdcall16far win_ui_op_1010_3202(ulong param_1, int param_2, HWND16 param_3)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 uVar4;
    HWND16     hwnd;
    ushort     unaff_SS;
    int        iStack4;

    iVar3 = (int)param_1;
    uVar4 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        piVar1  = (int *)(iVar3 + 0x28);
        *piVar1 = *piVar1 + -0xa;
        if(*piVar1 < 0x0)
        {
            *(undefined2 *)(iVar3 + 0x28) = 0x0;
        }
    }
    else
    {
        piVar1  = (int *)(iVar3 + 0x28);
        *piVar1 = *piVar1 + *(int *)(iVar3 + 0x18);
    }
    if(*(long *)(iVar3 + 0x52) != 0x0)
    {
        iStack4 = 0x0;
        hwnd    = param_3;
        do
        {
            uVar2   = *(undefined4 *)(iVar3 + 0x52);
            param_3 = hwnd;
            if(*(long *)((int)uVar2 + iStack4 * 0x4) != 0x0)
            {
                param_3 = (HWND16)s_tile2_bmp_1050_1538;
                DestroyWindow16(hwnd);
                uVar2                                       = *(undefined4 *)(iVar3 + 0x52);
                *(undefined4 *)((int)uVar2 + iStack4 * 0x4) = 0x0;
            }
            iStack4 = iStack4 + 0x1;
            hwnd    = param_3;
        } while(iStack4 < 0xa);
    }
    if(*(int *)(iVar3 + 0x16) == 0x0)
    {
        pass1_1010_32f4((ulong *)param_1, *(ulong **)(iVar3 + 0x56), unaff_SS, param_3);
    }
    else
    {
        pass1_1010_32da((ulong *)param_1, *(ulong *)(iVar3 + *(int *)(iVar3 + 0x16) * 0x4 + 0x26), param_3, unaff_SS);
    }
    pass1_1010_1f62(unaff_SS, param_1, 0x8);
    return;
}

astruct_11 *__stdcall16far pass1_1010_0ee6(astruct_11 *param_1, byte param_2)

{
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}
