
void destroy_window_1020_3b3e(astruct_30 *param_1, HWND16 param_2)

{
    u32 *puVar1;
    code      **ppcVar2;
     let mut uVar3: u16;
    astruct_30 *paVar4;
    astruct_30 *uVar5;
    astruct_30 *uVar6;
    HWND16      HVar5;
     let mut unaff_SS: u16;

    uVar6              = (astruct_30 *)(param_1 >> 0x10);
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
    if((uVar3 | puVar1) != 0x0)
    {
        ppcVar2 = *puVar1;
        (**ppcVar2)(HVar5, puVar1);
    }
    &uVar5->field_0xf6 = 0x0;
    if(uVar5->field_0xfa != 0x0)
    {
        uVar3 = uVar6 | uVar5;
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

void pass1_1020_3c8c(param_1: u32, param_2: u32, param_3: u16)

{
    pt_in_rect_1018_1bda(*(param_1 + 0xfa), param_2, (param_2 >> 0x10), param_3);
    return;
}

astruct_3 *pass1_1020_3ca6(astruct_3 *param_1, param_2: u8, param_3: u16)

{
    u32  uVar1;
    u16 *puStack10;

    uVar1         = param_1 & 0xffff0000;
    param_1       = (astruct_3 *)(uVar1 | param_1 - 0xf2);
    param_1._2_2_ = (uVar1 >> 0x10);
    if(param_1 == (astruct_3 *)0x0)
    {
        param_1._0_2_ = 0x0;
        param_1._2_2_ = 0x0;
    }
    puStack10       = CONCAT22(param_1._2_2_, param_1);
    *puStack10      = 0x389a;
    (param_1 + 0x2) = 0x1008;
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void enable_window_1020_1bd4(param_1: i16, param_2: u16, param_3: u16, param_4: u32, HWND16 param_5)

{
    code      **ppcVar1;
    bool        bVar2;
     let mut in_AX: u16;
    i16         iVar3;
     let mut in_DX: *mut u8;
     let mut puVar4: *mut u8;
     let mut uVar5: u16;
     let mut unaff_SS: u16;
    u32 *puStack12;

    bVar2 = false;
    pass1_1020_1d8e(CONCAT22(param_2, param_1), CONCAT22(param_4, param_3));
    if(in_AX != 0x0)
    {
        if(in_AX < 0x2)
        {
            bVar2 = true;
        }
        else
        {
            GetDlgItem16(param_5, 0x1);
            pass1_1010_4e8c(*(param_1 + 0x8e), unaff_SS);
            in_AX = EnableWindow16(0x1010, 0x1);
            pass1_1010_4df0(*(param_1 + 0x8e), in_DX, unaff_SS);
            if((in_AX == 0x0) && (bVar2 = true, (param_1 + 0x96) == 0x0))
            {
                in_AX = EnableWindow16(0x1010, 0x0);
            }
        }
    }
    if(bVar2)
    {
        uVar5 = 0x1000;
        mem_op_1000_179c(0xb4, in_DX, 0x1000);
        puVar4 = (in_DX | in_AX);
        if(puVar4 == 0x0)
        {
            iVar3  = 0x0;
            puVar4 = 0x0;
        }
        else
        {
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar3 = string_1040_8520(CONCAT22(in_DX, in_AX), (param_1 + 0x6), 0x30, 0x2, 0x57b, 0x62a, puVar4, unaff_SS);
        }
        puStack12 = CONCAT22(puVar4, iVar3);
        ppcVar1   = (*puStack12 + 0x74);
        (**ppcVar1)(uVar5, iVar3, puVar4);
    }
    return;
}

void set_win_tet_1020_1d2a(param_1: u16, param_2: u16, SEGPTR in_win_text_3, param_4: u16, u16 in_dlg_id_5, HWND16 in_hwnd_6)

{
    GetDlgItem16(in_hwnd_6, in_dlg_id_5);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, in_win_text_3);
    return;
}

void pass1_1020_1d8e(param_1: u32, param_2: u32)

{
    pt_in_rect_1010_4e08(*(param_1 + 0x8e), param_2, (param_2 >> 0x10), 0x1010);
    return;
}

BOOL16 destroy_win_1020_1dea(HWND16 param_1)

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


u16 destroy_win_1020_1e1e(HWND16 param_1)

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

Struct18 *pass1_1020_1e54(Struct18 *param_1, param_2: u8)

{
    ui_cleanup_op_1040_782c(param_1, &PTR_LOOP_1050_1040);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void window_op_1020_2642(astruct *param_1)

{
    astruct_664 *in_AX;
     let mut in_DX: *mut u8;
     let mut uVar1: u16;
    i16          iVar2;
    i16          unaff_DI;
     let mut uVar3: u16;
     let mut unaff_SS: u16;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    get_dc_1018_4db0(*(iVar2 + 0xf2), (iVar2 + 0x8), 0x1018);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = in_DX | in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1020_27b0(in_AX, in_DX, (iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_664 **)(iVar2 + 0xee) = in_AX;
        *(iVar2 + 0xf0)                 = uVar1;
        return;
    }
    (iVar2 + 0xee) = 0x0;
    return;
}


void pass1_1020_26a6(param_1: u32)

{
    u32 *puVar1;
     let mut uVar2: u16;
    code      **ppcVar3;
     let mut uVar4: u16;

    uVar4  = (param_1 >> 0x10);
    puVar1 = *(param_1 + 0xee);
    uVar2  = *(param_1 + 0xf0);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void pass1_1020_28fc(astruct_3 *param_1, param_2: u16)

{
    param_1->address_offset_field_0x0 = 0x2e4a;
    (param_1 + 0x2)                   = 0x1020;
    cleanup_menu_ui_op_1020_795c(param_1, param_2);
    return;
}

void pass1_1020_2a6a(param_1: u32, param_2: u16)

{
    get_win_ui_info_op_1020_7a50(param_1, param_2);
    pass1_1018_0ae8(*(param_1 + 0xf2), 0x0);
    destroy_icon_1020_2c88(param_1, 0x1018);
    return;
}

void bring_window_to_top_1020_2aae(param_1: u32, param_2: u32)

{
     let mut unaff_SS: u16;

    pass1_1008_3e0e(param_1);
    BringWindowToTop16(0x1008);
    pass1_1018_169e(*(param_1 + 0xf2), param_2, unaff_SS);
    return;
}

void pass1_1020_0aa6(param_1: u32, param_2: u16)

{
    win_ui_palette_op_1020_0cd2((param_1 + 0xe2), param_2);
    return;
}

void win_ui_palette_op_1020_0cd2(param_1: u32, HWND16 param_2)

{
     let mut uVar1: u16;
    u32 *puVar2;
    code      **ppcVar3;
    u32  uVar4;
     let mut uVar5: u16;
    HDC16       hdc;
    HDC16       b_force_background;
    HPALETTE16  b_force_background_00;
    u1616       UVar6;
     let mut extraout_DX: u16;
    i16         iVar7;
     let mut uVar8: u16;
    astruct_13 *paStack10;
     let mut uStack6: u16;

    uVar4   = (param_1 + 0x6);
    uVar8   = (uVar4 >> 0x10);
    iVar7   = uVar4;
    puVar2  = (iVar7 + 0xa);
    uVar1   = *(iVar7 + 0xc);
    uStack6 = puVar2;
    uVar5   = uVar1 | uStack6;
    if(uVar5 != 0x0)
    {
        ppcVar3 = (*puVar2 + 0x14);
        (**ppcVar3)(param_2, uStack6, uVar1);
        paStack10 = (astruct_13 *)CONCAT22(extraout_DX, uVar5);
        uVar5     = extraout_DX | uVar5;
        if(uVar5 != 0x0)
        {
            hdc                = GetDC16(param_2);
            b_force_background = hdc;
            create_palette_1008_4e38(paStack10, 0x1008, uVar5);
            b_force_background_00 = SelectPalette16(0x1008, 0x0, b_force_background);
            UVar6                 = RealizePalette16(s_tile2_bmp_1050_1538);
            SelectPalette16(s_tile2_bmp_1050_1538, 0x1, b_force_background_00);
            DeleteObject16(s_tile2_bmp_1050_1538);
            if(0x0 < UVar6)
            {
                InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (&PTR_LOOP_1050_0000 + 0x1), 0x0);
            }
            ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, hdc);
            return;
        }
    }
    return;
}

void pass1_1020_0e2c(param_1: u32, param_2: u16)

{
    get_win_ui_info_op_1020_7a50(param_1, param_2);
    cleanup_ui_op_1020_1038(param_1);
    return;
}

void pass1_1020_0e8e(param_1: i16, param_2: u16, param_3: i16, param_4: i16, param_5: i16, param_6: u16, param_7: u16)

{
    fn_ptr_1 *ppcVar1;

    win_ui_cursor_op_1020_1294(CONCAT22(param_2, param_1), param_3, param_4, param_6, param_7);
    if(param_5 == 0x0)
    {
        ppcVar1 = ((param_1 + 0x4) + 0x5c);
        (**ppcVar1)();
    }
    return;
}

void enable_menu_1020_1000(HMENparam_1: u16, param_2: i16)

{
    if(param_2 != 0x0)
    {
        return;
    }
    EnableMenuItem16(param_1, 0x400, 0x3);
    return;
}

void window_op_1020_10a0(astruct *param_1)

{
    u32   uVar1;
    code       **ppcVar2;
    astruct_160 *in_AX;
     let mut uVar3: u16;
    BOOL16      *pBVar4;
     let mut in_DX: *mut u8;
     let mut puVar5: *mut u8;
     let mut puVar6: *mut u8;
     let mut extraout_DX: *mut u8;
     let mut extraout_DX_00: u16;
    i16          unaff_DI;
     let mut unaff_SS: u16;
     let mut in_AF: u8;
    u16         *puVar7;
    u32          uVar8;
     let mut uVar9: u16;
     let mut puVar10: *mut u8;
    i16          iVar11;
     let mut uVar12: u16;

    iVar11 = param_1;
    uVar12 = (param_1 >> 0x10);
    create_window_ex_1008_9760(param_1, 0x1008);
    mem_op_1000_179c(0x42, in_DX, 0x1000);
    puVar5 = (in_DX | in_AX);
    if(puVar5 != 0x0)
    {
        pass1_1008_3bd6(in_AX, in_DX, 0x0, 0x1f009b, 0x0, 0x740075, CONCAT22((iVar11 + 0x8), 0xf1), puVar5, unaff_SS);
    }
    mem_op_1000_179c(0x42, puVar5, 0x1000);
    puVar6 = (puVar5 | in_AX);
    if(puVar6 != 0x0)
    {
        pass1_1008_3bd6(in_AX, puVar5, 0x0, 0x31009b, 0x0, 0x760077, CONCAT22((iVar11 + 0x8), 0xf2), puVar6, unaff_SS);
    }
    mem_op_1000_179c(0x42, puVar6, 0x1000);
    puVar5 = (puVar6 | in_AX);
    if(puVar5 != 0x0)
    {
        pass1_1008_3bd6(in_AX, puVar6, 0x0, 0x77009b, 0x0, 0x780079, CONCAT22((iVar11 + 0x8), 0xf3), puVar5, unaff_SS);
    }
    puVar7                      = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2d, unaff_SS, puVar5, unaff_DI);
    uVar9                       = (puVar7 >> 0x10);
    (iVar11 + 0xf2)             = puVar7;
    (iVar11 + 0xf4)             = uVar9;
    (iVar11 + 0xe0)             = (iVar11 + 0xf2);
    (iVar11 + 0xe2)             = uVar9;
    puVar10                     = globals.PTR_LOOP_1050_038c;
    uVar3                       = LoadIcon16(0x1010, s_PLNTICON_1050_4267);
    *(HICON16 *)(iVar11 + 0xc2) = uVar3;
    uVar1                       = (iVar11 + 0xf2);
    ppcVar2                     = ((iVar11 + 0xf2) + 0x30);
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar1, (uVar1 >> 0x10), uVar3, puVar10);
    puVar5 = extraout_DX;
    mem_op_1000_179c(0x24, extraout_DX, 0x1000);
    puVar6 = (puVar5 | uVar3);
    if(puVar6 == 0x0)
    {
        (iVar11 + 0xf6) = 0x0;
    }
    else
    {
        unk_win_ui_op_1020_1418((astruct_40 *)CONCAT22(puVar5, uVar3), param_1, unaff_SS);
        *(iVar11 + 0xf6) = uVar3;
        (iVar11 + 0xf8)  = puVar6;
    }
    (iVar11 + 0xe8) = (iVar11 + 0xf6);
    puVar7          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, unaff_SS, puVar6, unaff_DI);
    uVar8           = pass1_1018_04b8(puVar7);
    puVar5          = (uVar8 >> 0x10);
    pass1_1010_41d6(*(iVar11 + 0xf2), uVar8, puVar5, unaff_SS, in_AF);
    uVar8   = pass1_1010_451a(*(iVar11 + 0xf2), puVar5, unaff_DI, unaff_SS);
    pBVar4  = (BOOL16 *)uVar8;
    uVar1   = param_1;
    ppcVar2 = (uVar1 + 0x14);
    (**ppcVar2)(0x1010, iVar11, uVar12, 0x0, pBVar4, (uVar8 >> 0x10));
    uVar9   = 0x1;
    ppcVar2 = (uVar1 + 0x10);
    (**ppcVar2)();
    pass1_1010_459e((iVar11 + 0xf2));
    ppcVar2 = ((iVar11 + 0xf2) + 0x10);
    (**ppcVar2)(0x1010, (iVar11 + 0xf2), param_1, uVar9);
    MoveWindow16(0x1010, 0x1, pBVar4[0x3], pBVar4[0x2], pBVar4[0x1], *pBVar4);
    UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
    return;
}

void win_ui_cursor_op_1020_1294(param_1: u32, param_2: i16, param_3: i16, param_4: u16, param_5: u16)

{
    code      **ppcVar1;
     let mut in_AX: u16;
    HCURSOR16   HVar2;
    HCURSOR16   HVar3;
    i16         iVar4;
     let mut uVar5: u16;
    u32         uVar6;
    i16         local_12;
    i16         local_10;
    u16        *puStack14;
    u32 *puStack10;
    i16         local_6;
    i16         iStack4;

    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), 0x4000001);
    if((param_4 | in_AX) == 0x0)
    {
        local_6   = param_3;
        iStack4   = param_2;
        uVar5     = (param_1 >> 0x10);
        iVar4     = param_1;
        puStack10 = pass1_1010_40cc(*(iVar4 + 0xf2), param_2, 0x0);
        uVar6     = *(iVar4 + 0xf2);
        puStack14 = (uVar6 & 0xffff0000 | (uVar6 + 0x76));
        pass1_1008_3e94(puStack14, CONCAT22(param_5, &local_12), CONCAT22(param_5, &local_10));
        local_6 = local_6 - local_10;
        iStack4 = iStack4 - local_12;
        iVar4   = pt_in_rect_1010_40f8(*(iVar4 + 0xf2), (POINT16 *)CONCAT22(param_5, &local_6), 0x1010);
        if(iVar4 != -0x1)
        {
            uVar6   = 0x0;
            HVar2   = LoadCursor16(0x1010, 0x7f02);
            uVar6   = uVar6 & 0xffff0000 | HVar2;
            HVar3   = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
            ppcVar1 = (*puStack10 + 0x4);
            (**ppcVar1)(s_tile2_bmp_1050_1538, puStack10, (puStack10 >> 0x10), iVar4, iVar4 >> 0xf, iVar4, 0x2, uVar6, HVar3, HVar2);
            pass1_1008_3e0e(param_1);
            SetCursor16(0x1008);
        }
    }
    return;
}


astruct_3 *pass1_1020_135e(astruct_3 *param_1, param_2: u8, param_3: u16)

{
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1020_1418(astruct_40 *param_1, param_2: u32, u1616 param_3)

{
    u32  uVar1;
    astruct_13 *paVar2;
    code      **ppcVar3;
    HDC16      *pHVar4;
    u32 *puVar5;
     let mut puVar6: *mut u8;
     let mut extraout_DX: *mut u8;
    astruct_40 *iVar5;
    i16         unaff_DI;
     let mut uVar7: u16;
     let mut unaff_CS: u16;
    u16        *puVar8;
    HDC16       local_8;
    u16        *puStack6;

    get_sys_metrics_1020_7c1a(param_1, param_2, unaff_CS);
    uVar7              = (param_1 >> 0x10);
    iVar5              = (astruct_40 *)param_1;
    &iVar5->field_0x14 = 0x0;
    iVar5->field_0x18  = 0x0;
    puVar8             = pass1_1008_3e38((param_1 & 0xffff0000 | &iVar5->field_0x1e));
    param_1            = 0x1730;
    iVar5->field_0x2   = 0x1020;
    puVar8             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2d, param_3, (puVar8 >> 0x10), unaff_DI);
    puVar6             = (puVar8 >> 0x10);
    iVar5->field_0x14  = puVar8;
    &iVar5->field_0x16 = puVar6;
    puStack6           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_3, puVar6, unaff_DI);
    uVar1              = &iVar5->field_0x14;
    ppcVar3            = (**(u32 **)&iVar5->field_0x14 + 0x4);
    (**ppcVar3)(0x1010, uVar1, (uVar1 >> 0x10), 0x0, param_1);
    local_8                  = GetDC16(0x1010);
    uVar1                    = &iVar5->field_0x14;
    *(HDC16 *)(uVar1 + 0x7c) = local_8;
    uVar1                    = &iVar5->field_0x14;
    puVar5                   = *(uVar1 + 0x66);
    iVar5->field_0x18        = puVar5;
    ppcVar3                  = (*iVar5->field_0x18 + 0x14);
    (**ppcVar3)();
    paVar2 = *(astruct_13 **)(puStack6 + 0xe);
    puVar6 = extraout_DX;
    pass1_1008_4d84((astruct_90 *)(puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10), paVar2, extraout_DX);
    pHVar4            = (HDC16 *)palette_op_1008_4e08(paVar2, &local_8, puVar6, 0x1008);
    iVar5->field_0x1c_addr_base = pHVar4;
    return;
}


void win_ui_op_1020_150e(mut param_1: *mut u16, param_2: HDC16)

{
    HPALETTE16 HVar1;
    i16        iVar2;
     let mut uVar3: u16;
     let mut unaff_SS: u16;

    uVar3         = (param_1 >> 0x10);
    iVar2         = param_1;
    *param_1      = 0x1730;
    (iVar2 + 0x2) = 0x1020;
    if((iVar2 + 0x14) != 0x0)
    {
        param_2 = 0x1010;
        pass1_1010_1ea6(*(iVar2 + 0x14), param_1 & 0xffff | uVar3 << 0x10, unaff_SS);
    }
    HVar1                         = SelectPalette16(param_2, 0x0, (iVar2 + 0x1c));
    *(HPALETTE16 *)(iVar2 + 0x1c) = HVar1;
    DeleteObject16(s_tile2_bmp_1050_1538);
    *param_1      = 0x3ab0;
    (iVar2 + 0x2) = 0x1008;
    *param_1      = 0x389a;
    (iVar2 + 0x2) = 0x1008;
    return;
}

Struct18 *pass1_1020_170a(Struct18 *param_1, param_2: u8, param_3: u16)

{
    win_ui_op_1020_150e(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void pass1_1020_1780(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x6c);
    (**ppcVar1)();
    destroy_win_1040_8212(param_1, (HWND16)&PTR_LOOP_1050_1040);
    return;
}


void mixed_ui_op_1020_179c(astruct_1 *param_1)

{
    u32         uVar1;
    code      **ppcVar2;
    u32  uVar3;
     let mut uVar4: u16;
     let mut IVar5: u16;
     let mut puVar6: *mut u8;
     let mut in_DX: *mut u8;
     let mut extraout_DX: *mut u8;
     let mut puVar7: *mut u8;
     let mut uVar8: u16;
    i16         iVar9;
    i16         iVar10;
    i16         unaff_DI;
     let mut uVar11: u16;
     let mut uVar12: u16;
     let mut uVar13: u16;
    WNDCLASS16 *unaff_SS;
    u16        *puVar14;
    WNDCLASS16 *in_resc_id_3;
    WNDCLASS16 *in_buffer_4;
    WNDCLASS16  local_178[0xc];
    u32  uStack118;
    u32  uStack114;
    RECT16      local_6e;
    u32  uStack106;
    HWND16      HStack102;
    i16         iStack98;
    i16         iStack94;
     let mut uStack78: u16;
     let mut puStack76: *mut u8;
    u32  uStack74;
    HWND16      HStack70;
    u32  uStack68;
    u32  uStack64;
    LPVOID      pvStack60;
     let mut uStack58: u16;
     let mut uStack56: u16;
    u32        *pUStack54;
    u32  uStack50;
    u8          local_2e[0x12];
    RECT16      local_1c;
    u32  uStack24;
    i16         iStack20;
    i16         iStack18;
    u16        *puStack16;
    u16      *pIStack12;
     let mut uStack8: u16;
    u16        *puStack6;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    uVar4    = 0x89;
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, unaff_SS, in_DX, unaff_DI);
    puVar7   = (puStack6 >> 0x10);
    uVar4    = pass1_1010_65d0(unaff_SS, puStack6, uVar4);
    uStack8  = (uVar4 == 0x0);
    uVar4    = pass1_1010_65d0(unaff_SS, puStack6, 0x86);
    if(uVar4 != 0x0)
    {
        uStack8 = 0x0;
    }
    puVar14         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, unaff_SS, puVar7, unaff_DI);
    uVar12          = (puVar14 >> 0x10);
    uVar8           = puVar14;
    uVar11          = (param_1 >> 0x10);
    iVar9           = param_1;
    *(iVar9 + 0x8e) = uVar8;
    (iVar9 + 0x90)  = uVar12;
    ppcVar2         = ((iVar9 + 0x8e) + 0x10);
    (**ppcVar2)(0x1010, (iVar9 + 0x8e), uVar12, uStack8);
    puStack76 = extraout_DX;
    mem_op_1000_179c(0x12, extraout_DX, 0x1000);
    puVar7   = (puStack76 | uVar8);
    uStack78 = uVar8;
    if(puVar7 == 0x0)
    {
        (iVar9 + 0x92) = 0x0;
    }
    else
    {
        pass1_1020_1eea(CONCAT22(puStack76, uVar8), param_1, (iVar9 + 0x6), puVar7, unaff_DI, unaff_SS);
        *(iVar9 + 0x92) = uVar8;
        (iVar9 + 0x94)  = puVar7;
    }
    uVar1     = *(iVar9 + 0x8e);
    pIStack12 = (u16 *)(uVar1 & 0xffff0000 | (uVar1 + 0xa));
    puStack16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, puVar7, unaff_DI);
    GetClientRect16(0x1010, &local_1c);
    IVar5          = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    uVar12         = (pIStack12 >> 0x10);
    iVar10         = pIStack12;
    (iVar10 + 0x6) = IVar5 + uStack24._2_2_;
    uVar13         = (puStack16 >> 0x10);
    iStack18       = (puStack16 + 0xa);
    iStack20       = (puStack16 + 0xc);
    (iVar10 + 0x2) = (iStack20 - (iVar10 + 0x6)) / 0x2;
    iVar10         = iStack18 - (iVar10 + 0x4);
    uVar8          = iVar10 >> 0xf;
    *pIStack12     = iVar10 / 0x2;
    pass1_1028_dc52((astruct_92 *)CONCAT22(unaff_SS, local_2e), 0x1, 0x0, 0x100);
    uStack56 = 0x0;
    while(true)
    {
        puVar6 = local_2e;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar6));
        uStack50 = CONCAT22(uVar8, puVar6);
        uStack58 = uVar8 | puVar6;
        if(uStack58 == 0x0)
            break;
        pUStack54 = (puVar6 + 0x10);
        uVar8     = uStack58;
        if(pUStack54 != 0x0)
        {
            pass1_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10), *pUStack54);
            uVar8 = uStack58;
        }
    }
    uVar4          = pass1_1020_1da8(param_1, puVar6, 0x0, unaff_SS);
    (iVar9 + 0x96) = uVar4;
    uVar4          = pass1_1010_65d0(unaff_SS, puStack6, 0x86);
    if((uVar4 == 0x0) || ((iVar9 + 0x96) != 0x0))
    {
        uVar3          = (iVar9 + 0x8e);
        (uVar3 + 0x2c) = 0x0;
        HStack102      = GetDlgItem16(0x1010, 0x175);
        if(uStack8 != 0x0)
        {
            load_string_1010_84e0(0x1010, globals._PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_178, (short)unaff_SS);
            SetWindowText16(0x1010, (SEGPTR)local_178);
        }
        pvStack60 = MakeProcInstance16(s_tile2_bmp_1050_1538, globals.PTR_LOOP_1050_038c);
        GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_6e);
        uStack114       = uStack106;
        iStack98        = uStack106 - local_6e.x;
        iStack94        = uStack106._2_2_ - local_6e.y;
        uStack118       = local_6e & 0xffff0000 | (-(iStack98 - (pIStack12 + 0x4)) / 0x2);
        IVar5           = GetSystemMetrics16(s_tile2_bmp_1050_1538);
        uVar1           = uStack118 & 0xffff;
        uStack118       = uVar1 | (uStack118._2_2_ - IVar5) << 0x10;
        uStack118._0_2_ = (BOOL16)uVar1;
        MoveWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0, iStack94, iStack98, uStack118._2_2_ - IVar5, (BOOL16)uStack118);
    }
    else
    {
        win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x9d0001, unaff_SS, uVar4, uStack58);
        (iVar9 + 0x8c) = uVar4;
        pvStack60      = MakeProcInstance16(0x1008, globals.PTR_LOOP_1050_038c);
    }
    EnumChildWindows1((HWND16)s_tile2_bmp_1050_1538, 0x0, ZEXT24(pvStack60) << 0x10);
    FreeProcInstance16(s_tile2_bmp_1050_1538);
    HStack70 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_1c);
    uStack64   = uStack24;
    local_1c.x = uStack24 - local_1c.x;
    uStack74   = CONCAT22(local_1c.x, uStack24._2_2_ - local_1c.y);
    uStack68   = local_1c & 0xffff0000 | (-(local_1c.x - (pIStack12 + 0x4)) / 0x2);
    IVar5      = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    uStack68   = uStack68 & 0xffff | (uStack68._2_2_ - IVar5) << 0x10;
    if((iVar9 + 0x96) == 0x0)
    {
        if(uStack8 == 0x0)
            goto LAB_1020_1b24;
        in_buffer_4  = local_178;
        in_resc_id_3 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21);
    }
    else
    {
        load_string_1010_84e0(0x1010, globals._PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_178, (short)unaff_SS);
        GetDlgItem16(0x1010, 0x175);
        SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)local_178);
        in_resc_id_3 = local_178;
        in_buffer_4  = unaff_SS;
        unaff_SS     = 0x3fe;
    }
    load_string_1010_84e0(0x1010, globals._PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), in_resc_id_3, in_buffer_4, (short)unaff_SS);
    SetWindowText16(0x1010, (SEGPTR)local_178);
LAB_1020_1b24:
    MoveWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0, uStack74, (uStack74 >> 0x10), uStack68._2_2_, (BOOL16)uStack68);
    uVar12 = (pIStack12 >> 0x10);
    iVar9  = pIStack12;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x44, (iVar9 + 0x6), (iVar9 + 0x4), (iVar9 + 0x2), *pIStack12, 0x0);
    return;
}

void pass1_1018_5e5a(mut param_1: *mut u16)

{
     let mut uVar1: u16;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0x6128;
    (param_1 + 0x2) = 0x1018;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c((Struct18 *)param_1, &PTR_LOOP_1050_1040);
    return;
}

void win_ui_op_1018_5e9a(astruct_1 *param_1, param_2: u16)

{
    u32    uVar1;
    u32   *pUVar2;
     let mut IVar3: u16;
     let mut puVar4: *mut u8;
     let mut in_DX: *mut u8;
     let mut puVar5: *mut u8;
     let mut puVar6: *mut u8;
     let mut uVar7: u16;
     let mut uVar8: u16;
    i16    iVar9;
    i16    unaff_DI;
     let mut uVar10: u16;
    u16   *puVar11;
    u8     local_28[0x12];
    i16    iStack22;
     let mut uStack20: u16;
    i16    iStack18;
    i16    iStack16;
    RECT16 local_e;
    i16    iStack8;
    u16 *pIStack6;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    puVar11         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, param_2, in_DX, unaff_DI);
    puVar5          = (puVar11 >> 0x10);
    uVar7           = puVar11;
    uVar10          = (param_1 >> 0x10);
    iVar9           = param_1;
    *(iVar9 + 0x8e) = uVar7;
    (iVar9 + 0x90)  = puVar5;
    mem_op_1000_179c(0x12, puVar5, 0x1000);
    puVar6 = (puVar5 | uVar7);
    if(puVar6 == 0x0)
    {
        (iVar9 + 0x92) = 0x0;
    }
    else
    {
        pass1_1018_6198(CONCAT22(puVar5, uVar7), param_1, (iVar9 + 0x6), puVar6, unaff_DI, param_2);
        *(iVar9 + 0x92) = uVar7;
        (iVar9 + 0x94)  = puVar6;
    }
    uVar1    = *(iVar9 + 0x8e);
    pIStack6 = (u16 *)(uVar1 & 0xffff0000 | (uVar1 + 0xa));
    GetClientRect16(0x1000, &local_e);
    IVar3            = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    (pIStack6 + 0x6) = IVar3 + iStack8;
    puVar11          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_2, puVar6, unaff_DI);
    uStack20         = (puVar11 >> 0x10);
    iStack22         = puVar11;
    iStack16         = (iStack22 + 0xa);
    iStack18         = (iStack22 + 0xc);
    uVar10           = (pIStack6 >> 0x10);
    (pIStack6 + 0x2) = (iStack18 - (pIStack6 + 0x6)) / 0x2;
    uVar7            = iStack16 >> 0xf;
    *pIStack6        = iStack16 / 0x2 + 0x3;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_2, local_28), 0x1, 0x0, 0x100);
    while(true)
    {
        puVar4 = local_28;
        pass1_1028_e4ec(CONCAT22(param_2, puVar4));
        uVar8 = uVar7 | puVar4;
        if(uVar8 == 0x0)
            break;
        pUVar2 = (puVar4 + 0x10);
        uVar7  = uVar8;
        if(pUVar2 != 0x0)
        {
            pass1_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10), *pUVar2);
            uVar7 = uVar8;
        }
    }
    uVar10 = (pIStack6 >> 0x10);
    iVar9  = pIStack6;
    SetWindowPos16((HWND16)&USHORT_1050_1028, 0x44, (iVar9 + 0x6), (iVar9 + 0x4), (iVar9 + 0x2), *pIStack6, 0x0);
    return;
}

void mix_ui_op_1018_6adc(astruct_28 *param_1)

{
    i16         iVar1;
    i16         iVar2;
     let mut uVar3: u16;
     let mut in_DX: *mut u8;
     let mut uVar4: u16;
    i16         iVar5;
    i16         unaff_DI;
     let mut uVar6: u16;
    WNDCLASS16 *unaff_SS;
    u16        *puVar7;
    astruct_43 *paVar8;

    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    uVar4  = (puVar7 >> 0x10);
    iVar1  = (puVar7 + 0xa);
    iVar2  = (puVar7 + 0xc);
    uVar6  = (param_1 >> 0x10);
    iVar5  = param_1;
    if(0x1 < iVar1 - (iVar5 + 0xe6))
    {
        uVar4          = iVar1 >> 0xf;
        (iVar5 + 0xe2) = iVar1 / 0x2 - ((iVar5 + 0xe6) + 0x1) / 0x2;
    }
    if(0x1 < iVar2 - (iVar5 + 0xe8))
    {
        uVar4          = iVar2 >> 0xf;
        (iVar5 + 0xe4) = iVar2 / 0x2 - ((iVar5 + 0xe8) + 0x1) / 0x2;
    }
    uVar3 = ShowCursor16(0x1010);
    if((iVar5 + 0xee) != 0x0)
    {
        win_1008_5c5c(unaff_SS, uVar3, uVar4, globals._PTR_LOOP_1050_02a0, (iVar5 + 0xee));
        (iVar5 + 0xf0) = uVar3;
    }
    paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, (iVar5 + 0xec), unaff_SS);
    mci_send_command_1008_53ae(paVar8, (iVar5 + 0x8), 0x1008, unaff_SS);
    ShowCursor16(0x1008);
    unk_destroy_window_op_1018_6bb6(param_1, s_tile2_bmp_1050_1538);
    return;
}

astruct_11 *pass1_1018_4ae0(astruct_11 *param_1, param_2: u8, param_3: u16)

{
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void unk_win_ui_op_1018_4f18(astruct_39 *param_1, u1616 param_2, param_3: u32)

{
    code      **ppcVar1;
    u32 *puVar2;
    RECT16     *rect;
    i16         iVar3;
    u32         uVar4;
     let mut extraout_DX: *mut u8;
     let mut puVar5: *mut u8;
     let mut extraout_DX_00: *mut u8;
     let mut puVar6: *mut u8;
     let mut uVar7: u16;
    astruct_39 *iVar6;
     let mut uVar8: u16;
     let mut uVar9: u16;
     let mut unaff_SS: u16;
    astruct_76 *paStack22;
    RECT16      local_12;
    i16         iStack14;
    i16         iStack12;
    u32         uStack10;
    astruct_43 *paStack6;

    paStack6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, param_2, unaff_SS);
    uVar4    = paStack6 & 0xffff;
    ppcVar1  = (paStack6 + 0x14);
    (**ppcVar1)(0x1010, uVar4, (paStack6 >> 0x10));
    puVar2   = uVar4;
    uStack10 = uVar4 & 0xffff | ZEXT24(extraout_DX) << 0x10;
    uVar8    = (param_1 >> 0x10);
    iVar6    = (astruct_39 *)param_1;
    puVar5   = extraout_DX;
    if(&iVar6->field_0xe != 0x0)
    {
        uVar7  = iVar6->field_0x10;
        puVar2 = &iVar6->field_0xe;
        puVar5 = (uVar7 | puVar2);
        if(puVar5 != 0x0)
        {
            ppcVar1 = *puVar2;
            (**ppcVar1)();
            puVar5 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar5, 0x1000);
    puVar6 = (puVar5 | puVar2);
    if(puVar6 == 0x0)
    {
        puVar2 = 0x0;
        puVar6 = 0x0;
    }
    else
    {
        struct_1008_4c58(CONCAT22(puVar5, puVar2));
    }
    iVar6->field_0xe  = puVar2;
    iVar6->field_0x10 = puVar6;
    pass1_1008_4d84(&iVar6->field_0xe, uStack10, puVar6);
    rect = &local_12;
    GetClientRect16(0x1008, rect);
    uVar9 = 0x1000;
    mem_op_1000_179c(0x1e, puVar6, 0x1000);
    paStack22 = (astruct_76 *)CONCAT22(puVar6, rect);
    uVar7     = puVar6 | rect;
    if(uVar7 == 0x0)
    {
        &iVar6->field_0xa = 0x0;
    }
    else
    {
        iVar3 = (iStack12 - local_12.y) + 0x1;
        uVar9 = 0x1008;
        pass1_1008_405c(paStack22, *&iVar6->field_0xe, iVar3, (iStack14 - local_12.x) + 0x1);
        iVar6->field_0xa = iVar3;
        iVar6->field_0xc = uVar7;
    }
    if(paStack6 != (astruct_43 *)0x0)
    {
        ppcVar1 = paStack6;
        (**ppcVar1)(uVar9, paStack6, (paStack6 >> 0x10), 0x1);
    }
    return;
}


astruct_11 *pass1_1018_5032(astruct_11 *param_1, param_2: u8, param_3: u16)

{
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1018_57e6(param_1: u32, long param_2, param_3: u16)

{
     let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    send_dlg_item_msg_1040_d20c(*(param_1 + 0xa), param_2, &PTR_LOOP_1050_1040, param_3);
    (param_1 + 0xa) = 0x0;
    return;
}

void pt_in_rect_1018_1bda(param_1: u32, param_2: u16, param_3: u16, param_4: u16)

{
     let mut piVar1: *mut i16;
     let mut uVar2: u16;
    i16     iVar3;
    BOOL16  BVar4;
    i16     iVar5;
     let mut uVar6: u16;
    i16     iStack26;
    POINT16 PStack24;
    i16     local_14;
    i16     local_12;
     let mut uStack16: u16;
    u32     uStack14;
    i16     local_a;
    i16     local_8;
    i16     iStack6;
    i16     iStack4;

    uStack14 = 0x0;
    iVar3    = param_1;
    pass1_1008_3e94((param_1 & 0xffff0000 | (iVar3 + 0x3a)), CONCAT22(param_4, &local_14), CONCAT22(param_4, &local_12));
    PStack24 = (POINT16)CONCAT22(param_2, param_3);
    uStack16 = 0x0;
    iStack26 = 0x0;
    while(true)
    {
        uVar6  = (param_1 >> 0x10);
        piVar1 = (iVar3 + 0x44);
        if(*piVar1 == iStack26 || *piVar1 < iStack26)
        {
            return;
        }
        uVar2    = (iVar3 + 0x42);
        iVar5    = (iVar3 + 0x40) + iStack26 * 0x18;
        uStack14 = CONCAT22(uVar2, iVar5);
        pass1_1008_3e94(CONCAT22(uVar2, iVar5), CONCAT22(param_4, &local_8), CONCAT22(param_4, &local_a));
        local_a = local_a + local_12 + -0x6;
        iStack6 = local_a + 0xc;
        local_8 = local_8 + local_14 + -0x6;
        iStack4 = local_8 + 0xc;
        BVar4   = PtInRect16(0x1008, PStack24);
        if(BVar4 != 0x0)
            break;
        iStack26 = iStack26 + 0x1;
    }
    pass1_1018_1eda(param_1, uStack14, param_4);
    return;
}
void pass1_1018_2440(astruct_11 *param_1, param_2: u16)

{
    u32  *puVar1;
     let mut uVar2: u16;
    code       **ppcVar3;
     let mut piVar4: *mut i16;
     let mut uVar6: u16;
    astruct_502 *uVar5;
     let mut uVar7: u16;
    u16         *puStack6;

    uVar7             = (param_1 >> 0x10);
    uVar5             = (astruct_502 *)param_1;
    param_1           = 0x2ada;
    uVar5->field_0x2  = 0x1018;
    uVar5->field_0x1c_addr_base = s_fem132_wav_1050_2aec + 0x6;
    uVar5->field_0x1e = 0x1018;
    if(_PTR_LOOP_1050_0388 != 0x0)
    {
        if(param_1 == (astruct_11 *)0x0)
        {
            piVar4 = 0x0;
            uVar6  = 0x0;
        }
        else
        {
            piVar4 = &uVar5->field_0x1c_addr_base;
            uVar6  = uVar7;
        }
        param_2 = 0x1008;
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x73, CONCAT22(uVar6, piVar4));
    }
    puVar1 = uVar5->field_0x2a;
    uVar2  = uVar5->field_0x2c;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(param_2, puVar1, uVar2, 0x1);
    }
    puVar1 = uVar5->field_0x6e;
    uVar2  = uVar5->field_0x70;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(param_2, puVar1, uVar2, 0x1);
    }
    if(param_1 == (astruct_11 *)0x0)
    {
        piVar4 = 0x0;
        uVar7  = 0x0;
    }
    else
    {
        piVar4 = &uVar5->field_0x1c_addr_base;
    }
    puStack6    = CONCAT22(uVar7, piVar4);
    *puStack6   = 0x389a;
    piVar4[0x1] = 0x1008;
    clenaup_win_ui_1018_4d22(param_1, param_2);
    return;
}

void msg_box_op_1010_8bb4(param_1: u16, param_2: u16, param_3: u32, HINSTANCE16 param_4, param_5: u16)

{
     let mut pcVar1: String;
    u8    local_402[0x400];

    pcVar1 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), param_4);
    unk_str_op_1000_3d3e(CONCAT22(param_5, local_402), pcVar1);
    pass1_1000_3cea(CONCAT22(param_5, local_402), param_3);
    pcVar1 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x1000);
    MessageBox16(0x1000, 0x1010, pcVar1, (u1616)(pcVar1 >> 0x10));
    PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x11100ee);
    return;
}

void ui_op_1010_79aa(param_1: u32, param_2: i16, long param_3, param_4: u16)

{
    u32 uVar1;
     let mut puVar2: *mut u8;
     let mut extraout_DX: u16;
     let mut uVar3: u16;
    long       lStack18;
    long       lStack14;
    u8         local_a[0x8];

    uVar3 = (param_1 >> 0x10);
    if(((param_1 + 0x1c) != 0x0) && ((param_3 != 0x0 || (param_2 != 0x0))))
    {
        pass1_1008_5784(CONCAT22(param_4, local_a), *(param_1 + 0x1c));
        lStack18 = 0x0;
        do
        {
            puVar2 = local_a;
            pass1_1008_5b12(puVar2, param_4);
            lStack14 = CONCAT22(extraout_DX, puVar2);
            if((extraout_DX | puVar2) == 0x0)
                goto LAB_1010_7a49;
            if(((param_2 == 0x0) && ((puVar2 + 0x4) == param_3)) || ((param_3 == 0x0 && (uVar1 = (puVar2 + 0x8), (uVar1 + 0xa) == param_2))))
                break;
        } while(((puVar2 + 0x4) != param_3) || (uVar1 = (puVar2 + 0x8), (uVar1 + 0xa) != param_2));
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

void show_win_1010_7a76(param_1: u32, param_2: u16)

{
    i16  iVar1;
     let mut uVar2: u16;
     let mut unaff_SS: u16;
    long lVar3;
    u8   local_a[0x8];

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((iVar1 + 0x20) == 0x0)
    {
        (iVar1 + 0x20) = 0x1;
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), *(iVar1 + 0x1c));
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

void show_window_1010_7ace(param_1: u32, param_2: u16)

{
    i16  iVar1;
     let mut uVar2: u16;
    long lVar3;
    u8   local_a[0x8];

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((iVar1 + 0x20) != 0x0)
    {
        (iVar1 + 0x20) = 0x0;
        pass1_1008_5784(CONCAT22(param_2, local_a), *(iVar1 + 0x1c));
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


u32 destroy_window_1010_7b26(param_1: u32, long param_2, u1616 param_3, param_4: u16)

{
     let mut uVar1: u16;
     let mut puVar2: *mut u8;
     let mut extraout_DX: u16;
    i16   iVar2;
    u1616 uVar4;
    u8    local_a[0x8];

    uVar4 = (u1616)(param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = *(iVar2 + 0x1e) | *(iVar2 + 0x1c);
    if(uVar1 != 0x0)
    {
        pass1_1008_5784(CONCAT22(param_3, local_a), *(iVar2 + 0x1c));
        do
        {
            puVar2 = local_a;
            pass1_1008_5b12(puVar2, param_3);
            param_4 = extraout_DX | puVar2;
            if(param_4 == 0x0)
                break;
        } while((puVar2 + 0x4) != param_2);
        uVar1 = extraout_DX | puVar2;
        if(uVar1 != 0x0)
        {
            uVar1 = DestroyWindow16(0x1008);
        }
    }
    return CONCAT22(uVar1, param_4);
}

void pass1_1010_8096(u32 *param_1, param_2: i16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
     let mut uVar3: u16;
     let mut uVar4: u16;
     let mut unaff_SS: u16;
     let mut pcVar5: String;
    u16  *puVar6;
    u8    local_306[0x100];
    u8    local_206[0x100];
    u8    local_106[0x104];

    uVar4 = (param_1 >> 0x10);
    uVar3 = param_1;
    str_1000_4d58(((uVar3 + 0xe82) * 0x4 + 0x2526), 0x0, 0x0, CONCAT22(unaff_SS, local_206), CONCAT22(unaff_SS, local_306));
    unk_str_op_1000_3d3e(CONCAT22(unaff_SS, local_106), CONCAT22(unaff_SS, local_206));
    if(param_2 == 0x2)
    {
        puVar6 = &USHORT_1050_3194;
    }
    else
    {
        puVar6 = &USHORT_1050_3196;
    }
    pass1_1000_3cea(CONCAT22(unaff_SS, local_106), puVar6);
    pass1_1000_3cea(CONCAT22(unaff_SS, local_106), CONCAT22(unaff_SS, local_306));
    pcVar5 = set_err_mode_1010_8b14(param_1, CONCAT22(unaff_SS, local_106), unaff_SS);
    uVar2  = (pcVar5 >> 0x10);
    if((pcVar5 == local_106) && (uVar2 == unaff_SS))
    {
        msg_box_op_1010_8bb4(uVar3, uVar4, pcVar5 & 0xffff | uVar2 << 0x10, 0x1000, unaff_SS);
    }
    fn_ptr_1000_17ce((Struct18 *)*param_1, 0x1000);
    uVar1         = str_op_1008_60e8(pcVar5, uVar2);
    param_1       = uVar1;
    (uVar3 + 0x2) = uVar2;
    return;
}

astruct_43 *unk_io_op_1010_830a(param_1: u32, param_2: u16, param_3: u16)

{
     let mut in_AX: u16;
    u32 *puVar1;
    u32 *puVar2;
     let mut in_DX: *mut u8;
     let mut uVar3: u16;
    astruct_45 *iVar3;
    astruct_44 *iVar2;
    i16         iVar4;
    HINSTANCE16 unaff_CS;
     let mut uVar5: u16;
     let mut uVar6: u16;
    u32  local_2e;
    u32  uStack10;
    astruct_43 *paStack6;

    paStack6 = (astruct_43 *)0x0;
    iVar3    = (astruct_45 *)(param_2 * 0x10);
    uVar5    = param_1;
    uVar6    = (param_1 >> 0x10);
    if(iVar3->field_0x10 == 0x1)
    {
        uStack10       = set_err_mode_1010_8b14(param_1, *&iVar3->field_0x12, param_3);
        uStack10._2_2_ = (uStack10 >> 0x10);
        if((iVar3->field_0x12 == uStack10) && (iVar3->field_0x14 == uStack10._2_2_))
        {
            msg_box_op_1010_8bb4(uVar5, uVar6, uStack10, unaff_CS, param_3);
            return (astruct_43 *)0x0;
        }
        puVar1 = &local_2e;
        struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3, puVar1), 0x1, uStack10, uStack10._2_2_);
        mem_op_1000_179c(0x1e, (uStack10 >> 0x10), 0x1000);
        uVar3 = (uStack10 >> 0x10) | puVar1;
        if(uVar3 == 0x0)
        {
            puVar2 = 0x0;
            uVar3  = 0x0;
        }
        else
        {
            puVar2 = &local_2e;
            struct_op_1008_3f92((astruct_76 *)(uStack10 & 0xffff0000 | ZEXT24(puVar1)), (astruct_83 *)CONCAT22(param_3, puVar2));
        }
        paStack6 = (astruct_43 *)CONCAT22(uVar3, puVar2);
        close_file_1008_496c(&local_2e, param_3);
        local_2e = paStack6;
    }
    else
    {
        if((param_2 * 0x10 + 0x10) == 0x2)
        {
            pass1_1010_878c((astruct_87 **)param_1, (param_2 * 0x10 + 0x16), unaff_CS);
            if((uVar5 + 0x67c) == 0x0)
            {
                return (astruct_43 *)0x0;
            }
            iVar2 = (astruct_44 *)(param_2 * 0x10);
            pass1_1008_6562((uVar5 + 0x67c), CONCAT22(iVar2->field_0x1c_addr_base, iVar2->field_0x1e), iVar2->field_0x1a_addr_offset, in_AX, in_DX);
            local_2e = (astruct_43 *)CONCAT22(in_DX, in_AX);
        }
        else
        {
            iVar4 = param_2 * 0x10;
            if((iVar4 + 0x10) == 0x3)
            {
                local_2e = (astruct_43 *)set_err_mode_1010_8b14(param_1, *(iVar4 + 0x12), param_3);
                if(((iVar4 + 0x12) == local_2e) && ((iVar4 + 0x14) == (local_2e >> 0x10)))
                {
                    msg_box_op_1010_8bb4(uVar5, uVar6, local_2e, unaff_CS, param_3);
                    local_2e = local_2e;
                }
            }
            else
            {
                local_2e = paStack6;
                if((param_2 * 0x10 + 0x10) == 0x4)
                {
                    local_2e = (astruct_43 *)set_err_mode_1010_8b14(param_1, *(param_2 * 0x10 + 0x12), param_3);
                }
            }
        }
    }
    paStack6 = local_2e;
    return paStack6;
}

void pass1_1010_71d6(param_1: u32, param_2: i16, mut param_3: *mut u16, param_4: u16, param_5: u16, param_6: u16)

{
    i16        iVar1;
    u32 uVar2;
     let mut uVar3: u16;
    i16        iVar4;
     let mut uVar5: u16;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut uVar8: u16;
    u32        uVar9;
     let mut uStack20: u16;
     let mut uStack14: u16;
    u32        uStack6;

    uVar8 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x14);
    pass1_1010_ad22(uVar2, param_5, param_6, (uVar2 >> 0x10), *param_3);
    uStack6 = CONCAT22(param_5, param_4);
    if((param_5 | param_4) == 0x0)
    {
        return;
    }
    uVar9 = struct_op_1030_73a8(CONCAT22(param_5, param_4));
    uVar7 = (uVar9 >> 0x10);
    uVar3 = uVar9;
    if(((uVar7 | uVar3) != 0x0) && ((uVar3 + 0x1c) == 0x8000002))
    {
        return;
    }
    uVar2    = (param_4 + 0x2e);
    uStack14 = uVar2;
    if(((*(param_4 + 0x30) | uStack14) != 0x0) && ((uStack14 + 0x200) == 0x8000002))
    {
        return;
    }
    uVar2 = (param_1 + 0x14);
    uVar5 = pass1_1010_b028(uVar2, (uVar2 >> 0x10), uVar9);
    iVar1 = (uVar3 + 0x12);
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

astruct_11 *pass1_1010_5074(astruct_11 *param_1, param_2: u8)

{
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_29c6(astruct_11 *param_1)

{
    u32  *puVar1;
     let mut uVar2: u16;
    code       **ppcVar3;
    astruct_476 *iVar5;
     let mut uVar4: u16;

    uVar4            = (param_1 >> 0x10);
    iVar5            = (astruct_476 *)param_1;
    param_1          = s_add16_wav_1050_2bdc + 0x8;
    iVar5->field_0x2 = 0x1010;
    if(&iVar5->field_0x1c_addr_base != 0x0)
    {
        puVar1 = *&iVar5->field_0x1c_addr_base;
        uVar2  = iVar5->field_0x1e;
        if((uVar2 | puVar1) != 0x0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        &iVar5->field_0x1c_addr_base = 0x0;
        fn_ptr_1000_17ce((Struct18 *)iVar5->field_0x28, 0x1000);
        iVar5->field_0x28 = 0x0;
    }
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    return;
}

void win_ui_op_1010_3202(param_1: u32, param_2: i16, HWND16 param_3)

{
     let mut piVar1: *mut i16;
    u32 uVar2;
    i16        iVar3;
     let mut uVar4: u16;
    HWND16     hwnd;
     let mut unaff_SS: u16;
    i16        iStack4;

    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        piVar1  = (iVar3 + 0x28);
        *piVar1 = *piVar1 + -0xa;
        if(*piVar1 < 0x0)
        {
            (iVar3 + 0x28) = 0x0;
        }
    }
    else
    {
        piVar1  = (iVar3 + 0x28);
        *piVar1 = *piVar1 + (iVar3 + 0x18);
    }
    if((iVar3 + 0x52) != 0x0)
    {
        iStack4 = 0x0;
        hwnd    = param_3;
        do
        {
            uVar2   = (iVar3 + 0x52);
            param_3 = hwnd;
            if((uVar2 + iStack4 * 0x4) != 0x0)
            {
                param_3 = (HWND16)s_tile2_bmp_1050_1538;
                DestroyWindow16(hwnd);
                uVar2                   = (iVar3 + 0x52);
                (uVar2 + iStack4 * 0x4) = 0x0;
            }
            iStack4 = iStack4 + 0x1;
            hwnd    = param_3;
        } while(iStack4 < 0xa);
    }
    if((iVar3 + 0x16) == 0x0)
    {
        pass1_1010_32f4(param_1, (iVar3 + 0x56), unaff_SS, param_3);
    }
    else
    {
        pass1_1010_32da(param_1, *(iVar3 + (iVar3 + 0x16) * 0x4 + 0x26), param_3, unaff_SS);
    }
    pass1_1010_1f62(unaff_SS, param_1, 0x8);
    return;
}

astruct_11 *pass1_1010_0ee6(astruct_11 *param_1, param_2: u8)

{
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}
