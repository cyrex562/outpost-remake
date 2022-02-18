
void __stdcall16far destroy_win_1038_ef3a(astruct_31 *param_1, HWND16 param_2)

{
    astruct_31 *iVar1;
    astruct_31 *uVar1;

    uVar1                  = (astruct_31 *)((ulong)param_1 >> 0x10);
    iVar1                  = (astruct_31 *)param_1;
    *(undefined2 *)param_1 = 0x67c;
    iVar1->field_0x2       = (int)&PTR_LOOP_1050_1040;
    if(iVar1->field_0x96 != 0x0)
    {
        DestroyWindow16(param_2);
        iVar1->field_0x96 = 0x0;
    }
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, iVar1->field_0x6);
    ui_cleanup_op_1040_782c((astruct_18 *)param_1, (int)&PTR_LOOP_1050_1040);
    return;
}

void __stdcall16far win_ui_op_1040_0000(astruct_1 *param_1, uchar *param_2, HWND16 param_3)

{
    astruct_160 *rect;
    uint         uVar1;
    undefined2   uVar2;
    ushort       uVar3;
    ushort       uVar4;
    ushort       uVar5;
    int          unaff_DI;
    undefined2   uVar6;
    WNDCLASS16  *unaff_SS;
    LRESULT      LVar7;
    undefined4   uVar8;
    undefined2   local_24;
    undefined2   uStack34;
    undefined2   uStack32;
    undefined2   uStack30;
    int          iStack28;
    RECT16       local_1a;
    int          iStack22;
    undefined4   uStack18;
    undefined2   uStack14;
    int          iStack12;
    int          iStack10;
    astruct_160 *paStack8;
    uint         uStack6;
    int          iStack4;

    // Segment:    9
    // Offset:     0006f820
    // Length:     d974
    // Min Alloc:  d974
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    dialog_ui_fn_1040_78e2(param_1, param_3);
    iStack4 = 0x8;
    for(iStack10 = 0x0; uVar5 = (ushort)param_1, uVar6 = (undefined2)((ulong)param_1 >> 0x10), iStack10 < iStack4;
        iStack10 = iStack10 + 0x1)
    {
        unaff_DI = iStack10 * 0xe;
        local_24 = *(undefined2 *)(unaff_DI + 0x5c60);
        uStack34 = *(undefined2 *)(unaff_DI + 0x5c62);
        uStack32 = 0x1;
        uStack30 = 0x1;
        rect     = (astruct_160 *)&local_24;
        MapDialogRect16(param_3, (RECT16 *)rect);
        param_3 = 0x1000;
        mem_op_1000_179c(0x42, param_2, 0x1000);
        uVar1 = (uint)param_2 | (uint)rect;
        if(uVar1 == 0x0)
        {
            rect  = (astruct_160 *)0x0;
            uVar1 = 0x0;
        }
        else
        {
            param_3 = 0x1008;
            pass1_1008_3bd6(rect,
                            (ushort)param_2,
                            0x1,
                            CONCAT22(local_24, uStack34),
                            0x104,
                            0x1020103,
                            CONCAT22(*(undefined2 *)(uVar5 + 0x6), *(undefined2 *)(unaff_DI + 0x5c64)),
                            uVar1,
                            (ushort)unaff_SS);
        }
        paStack8 = rect;
        uStack6  = uVar1;
        LVar7    = win_ui_op_1040_0558((ulong)param_1, iStack10, param_3);
        param_2  = (uchar *)((ulong)LVar7 >> 0x10);
    }
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, (ushort)unaff_SS, param_2, unaff_DI);
    uVar2    = (undefined2)((ulong)uStack18 >> 0x10);
    iStack12 = *(int *)((int)uStack18 + 0xa);
    uStack14 = *(undefined2 *)((int)uStack18 + 0xc);
    GetWindowRect16(0x1010, &local_1a);
    uVar3      = iStack12 >> 0xf;
    iStack28   = iStack22 - local_1a.x;
    local_1a.x = (iStack12 / 0x2 - iStack28) + -0x3;
    if(local_1a.x < 0x0)
    {
        local_1a.x = 0x0;
    }
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x41, 0x0, 0x0, local_1a.y, local_1a.x, 0x0);
    uVar8 = pass1_1038_af40(
      _PTR_LOOP_1050_5b7c, *(ushort *)(uVar5 + 0x6), 0x17, uVar3, uVar5, (ushort)&PTR_LOOP_1050_1038, (ushort)unaff_SS);
    uVar4                     = (ushort)((ulong)uVar8 >> 0x10);
    uVar3                     = (ushort)uVar8;
    *(ushort *)(uVar5 + 0x96) = uVar3;
    *(ushort *)(uVar5 + 0x98) = uVar4;
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x9e0001, unaff_SS, uVar3, uVar4);
    *(ushort *)(uVar5 + 0x8c) = uVar3;
    return;
}

void __stdcall16far
win_ui_op_1040_0170(int param_1, ushort param_2, ushort param_3, ulong param_4, ushort param_5, WNDCLASS16 *param_6)

{
    uint        uVar1;
    BOOL16      BVar2;
    int         iVar3;
    uchar      *in_DX;
    int         iVar4;
    uchar      *extraout_DX;
    int         unaff_DI;
    undefined2  uVar5;
    uchar       in_AF;
    char       *pcVar6;
    LRESULT     LVar7;
    WPARAM16    w_param;
    undefined2  uVar8;
    HCURSOR16  *pHVar9;
    WNDCLASS16 *pWVar10;
    HCURSOR16  *pHVar11;
    WNDCLASS16 *pWVar12;
    ulong      *local_12a[0x43];
    ushort     *puStack30;
    ushort      uStack26;
    HCURSOR16   local_18;
    HCURSOR16   local_16;
    ulong       uStack20;
    int         iStack16;
    int         iStack14;
    ushort     *puStack12;
    int         iStack8;
    int         iStack6;
    int         iStack4;

    iStack4 = 0x8;
    iStack6 = 0x0;
    switch(param_4._2_2_)
    {
    case 0x167:
        enable_win_1040_060e(CONCAT22(param_2, param_1), 0x3, (int)&PTR_LOOP_1050_1040, (ushort)param_6);
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1040, 0x16b);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        iStack4 = 0x0;
        break;
    case 0x168:
        enable_win_1040_060e(CONCAT22(param_2, param_1), 0x3, (int)&PTR_LOOP_1050_1040, (ushort)param_6);
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1040, 0x16b);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        iStack4 = 0x1;
        break;
    case 0x169:
        enable_win_1040_060e(CONCAT22(param_2, param_1), 0x3, (int)&PTR_LOOP_1050_1040, (ushort)param_6);
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1040, 0x16b);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        iStack4 = 0x2;
        break;
    case 0x16a:
        enable_win_1040_060e(CONCAT22(param_2, param_1), 0x3, (int)&PTR_LOOP_1050_1040, (ushort)param_6);
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1040, 0x16b);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        iStack4 = 0x3;
        break;
    case 0x16b:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1040, 0x16b);
        uVar5 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        BVar2 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        if(*(int *)(param_1 + 0x92) != 0x3)
        {
            uVar5 = 0x1008;
            win_1008_5c5c(param_6, BVar2, (ushort)in_DX, _PTR_LOOP_1050_02a0, 0x1de);
        }
        if(*(int *)(param_1 + 0x92) != 0x8)
        {
            iVar3   = *(int *)(param_1 + 0x92) * 0xe;
            iStack6 = *(int *)(iVar3 + 0x5c6c);
            uVar5   = 0x1010;
            pass1_1010_6604(*(ulong *)(param_1 + 0x8e), *(ushort *)(iVar3 + 0x5c66), (ushort)param_6);
            *(undefined2 *)(param_1 + 0x92) = 0x8;
        }
        for(iStack8 = 0x0; iStack8 < 0x4; iStack8 = iStack8 + 0x1)
        {
            LVar7 = win_ui_op_1040_0558(CONCAT22(param_2, param_1), iStack8, uVar5);
            in_DX = (uchar *)((ulong)LVar7 >> 0x10);
        }
        goto LAB_1040_04da;
    case 0x16c:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1040, 0x16d);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        iStack4                         = 0x5;
        *(undefined2 *)(param_1 + 0x94) = 0x5;
        goto LAB_1040_04da;
    case 0x16d:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1040, 0x16d);
        BVar2 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        uVar5 = 0x1008;
        win_1008_5c5c(param_6, BVar2, (ushort)in_DX, _PTR_LOOP_1050_02a0, 0x1de);
        if(*(int *)(param_1 + 0x94) != 0x8)
        {
            iVar3   = *(int *)(param_1 + 0x94) * 0xe;
            iStack6 = *(int *)(iVar3 + 0x5c6c);
            uVar5   = 0x1010;
            pass1_1010_6604(*(ulong *)(param_1 + 0x8e), *(ushort *)(iVar3 + 0x5c66), (ushort)param_6);
            *(undefined2 *)(param_1 + 0x94) = 0x8;
        }
        LVar7 = win_ui_op_1040_0558(CONCAT22(param_2, param_1), 0x5, uVar5);
        puStack12
          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, (ushort)param_6, (uchar *)((ulong)LVar7 >> 0x10), unaff_DI);
        iVar3    = *(int *)((int)puStack12 + 0x20);
        pHVar11  = &local_16;
        pHVar9   = &local_18;
        iVar4    = (iVar3 >> 0xf) + 0x200;
        pWVar10  = param_6;
        pWVar12  = param_6;
        iStack16 = iVar3;
        iStack14 = iVar4;
        iStack8  = iVar3;
        pass1_1030_8344(
          (ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), CONCAT22(iVar4, iVar3));
        uStack20 = CONCAT22(iVar4, iVar3);
        pass1_1030_2f1a(
          CONCAT22(iVar4, iVar3), (ushort *)CONCAT22(pWVar10, pHVar9), (ushort *)CONCAT22(pWVar12, pHVar11));
        in_DX    = (uchar *)((int)(local_18 - local_16) >> 0xf);
        local_16 = local_16 + (int)(local_18 - local_16) / 0x2;
        uStack26 = pass1_1030_2fac(uStack20);
        set_window_text_1018_6086(*(ULONG *)(param_1 + 0x96), 0x1018, param_6);
        goto LAB_1040_04da;
    case 0x16e:
        puStack30 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, (ushort)param_6, in_DX, unaff_DI);
        uStack26  = *(ushort *)((int)puStack30 + 0x20);
        local_18  = LoadCursor16(0x1010, (LPCSTR)0x7f02);
        local_16  = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
        pass1_1030_532e(
          (astruct_100 *)CONCAT22(param_6, local_12a), (long)(int)uStack26 + 0x2000000, (ushort)param_6, in_AF);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, local_12a));
        pass1_1030_838e((ulong *)_PTR_LOOP_1050_5748, (ushort)param_6, in_AF);
        pass1_1030_8334((int)_PTR_LOOP_1050_5748, (int)((ulong)_PTR_LOOP_1050_5748 >> 0x10));
        in_DX = extraout_DX;
        SetCursor16(0x1030);
        PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x111007e);
        DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
        local_12a[0] = &ULONG_1008_389a;
        goto LAB_1040_04da;
    default:
        post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2, param_1), param_3, (ushort)param_4, param_4._2_2_, param_5);
        return;
    }
    *(int *)(param_1 + 0x92) = iStack4;
LAB_1040_04da:
    if(iStack4 != 0x8)
    {
        uVar5   = *(undefined2 *)(iStack4 * 0xe + 0x5c68);
        w_param = 0x0;
        uVar8   = 0xc;
        pcVar6  = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        LVar7   = SendDlgItemMessage16(
          0x1010, (INT16)pcVar6, (UINT16)((ulong)pcVar6 >> 0x10), w_param, CONCAT22(uVar5, uVar8));
        in_DX = (uchar *)((ulong)LVar7 >> 0x10);
    }
    if(iStack6 != 0x0)
    {
        local_12a[0] = (ulong *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, (ushort)param_6, in_DX, unaff_DI);
        uVar1        = *(uint *)((int)local_12a[0] + 0x20);
        puStack30    = (ushort *)((ulong)puStack30 & 0xffff0000 | (ulong)uVar1);
        if(uVar1 != 0x0)
        {
            PostMessage16(0x1010, 0x0, 0x0, CONCAT22(0x111, iStack6));
        }
    }
    return;
}

LRESULT __stdcall16far win_ui_op_1040_0558(ulong param_1, int param_2, HWND16 param_3)

{
    int        iVar1;
    int        iVar2;
    ushort     unaff_SS;
    char      *pcVar3;
    LRESULT    LVar4;
    WPARAM16   w_param;
    undefined2 uVar5;
    undefined2 uVar6;

    iVar1 = param_2 * 0xe;
    GetDlgItem16(param_3, *(INT16 *)(iVar1 + 0x5c64));
    iVar2 = pass1_1010_659a(*(ulong *)((int)param_1 + 0x8e), *(ushort *)(iVar1 + 0x5c66), unaff_SS);
    if((iVar2 == 0x0) && (*(int *)(iVar1 + 0x5c66) != 0xa))
    {
        EnableWindow16(0x1010, 0x0);
        uVar6 = *(undefined2 *)(param_2 * 0xe + 0x5c68);
    }
    else
    {
        EnableWindow16(0x1010, 0x1);
        uVar6 = *(undefined2 *)(param_2 * 0xe + 0x5c68);
    }
    uVar5   = 0xc;
    w_param = 0x0;
    pcVar3  = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    LVar4
      = SendDlgItemMessage16(0x1010, (INT16)pcVar3, (UINT16)((ulong)pcVar3 >> 0x10), w_param, CONCAT22(uVar6, uVar5));
    return LVar4;
}

void __cdecl16far enable_win_1040_060e(ulong param_1, int param_2, HWND16 param_3, ushort param_4)

{
    INT16 *pIVar1;
    int    iStack10;
    int    iStack8;

    _iStack8 = (INT16 *)CONCAT22(param_4, &stack0x000a);
    iStack10 = param_2;
    while(true)
    {
        pIVar1 = _iStack8;
        if(iStack10 == 0x0)
            break;
        _iStack8 = (INT16 *)((ulong)_iStack8 & 0xffff0000 | (ulong)(iStack8 + 0x2));
        GetDlgItem16(param_3, *pIVar1);
        param_3 = (HWND16)s_tile2_bmp_1050_1538;
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        iStack10 = iStack10 + -0x1;
    }
    return;
}

void __stdcall16far pass1_1040_073a(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xb90;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1038);
    return;
}

void __stdcall16far show_win_1040_0766(astruct_1 *param_1, undefined2 param_2)

{
    uchar     *in_DX;
    uchar     *puVar1;
    int        unaff_DI;
    ushort     unaff_SS;
    ushort    *puVar2;
    int       *piVar3;
    int       *piVar4;
    ushort     uVar5;
    int        local_a;
    int        local_8;
    undefined4 uStack6;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    uStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, unaff_SS, in_DX, unaff_DI);
    puVar1  = (uchar *)((ulong)uStack6 >> 0x10);
    pass1_1010_6118((ulong)uStack6);
    piVar4 = &local_8;
    piVar3 = &local_a;
    uVar5  = unaff_SS;
    puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, puVar1, unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)),
                    (ushort *)CONCAT22(unaff_SS, piVar3),
                    (ushort *)CONCAT22(uVar5, piVar4));
    move_win_1040_826c(param_1, local_a + 0x8c, local_8 + 0xb9);
    ShowWindow16(0x1008, 0x5);
    return;
}

void __stdcall16far win_ui_op_1040_07dc(ushort param_1,
                                        ushort param_2,
                                        ushort param_3,
                                        ushort param_4,
                                        uint   param_5,
                                        ushort param_6,
                                        HWND16 param_7,
                                        ushort param_8)

{
    code      **ppcVar1;
    INT16       IVar2;
    BOOL16      BVar3;
    uchar      *puVar4;
    uchar      *puVar5;
    int         unaff_DI;
    undefined2  uVar6;
    ushort     *puVar7;
    undefined4 *puVar8;
    undefined   uVar9;
    undefined   uVar10;
    undefined4  uStack2060;
    char        local_806[0x400];
    undefined4  local_406[0x100];
    undefined4  uStack6;

    uStack6 = 0x0;
    if(param_5 == 0x73)
    {
        enable_window_1040_0acc(param_1, param_2, 0x0, param_7);
        puVar4     = pass1_1008_5fd8(param_8, (uchar *)param_6);
        uStack2060 = CONCAT22(param_6, puVar4);
        puVar5     = (uchar *)param_6;
        load_string_1010_84e0(
          0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_806, param_8);
        IVar2        = MessageBox16(0x1010, (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x14), local_806, param_8);
        local_406[0] = uStack2060;
        uVar6        = 0x1000;
        fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_6, puVar4), 0x1000);
        if(IVar2 == 0x6)
        {
            uVar6 = SUB42(s_tile2_bmp_1050_1538, 0x0);
            PostMessage16(0x1000, 0x0, 0x0, 0x11100cb);
            BVar3 = post_win_msg_1040_7b3c(
              (ulong *)CONCAT22(param_2, param_1), param_3, param_4, 0x1, (int)s_tile2_bmp_1050_1538);
            uStack6 = CONCAT22(puVar5, BVar3);
        }
    }
    else
    {
        uVar9 = (undefined)(param_2 >> 0x8);
        if(param_5 < 0x74)
        {
            if(param_5 == 0x6e)
            {
                *(undefined2 *)((int)_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
                puVar8                                           = (undefined4 *)pass1_1038_af40(_PTR_LOOP_1050_5b7c,
                                                       *(ushort *)(param_1 + 0x6),
                                                       0x2,
                                                       param_6,
                                                       param_1,
                                                       (ushort)&PTR_LOOP_1050_1038,
                                                       param_8);
                ppcVar1                                          = (code **)((int)*puVar8 + 0x3c);
                (**ppcVar1)((int)&PTR_LOOP_1050_1038, (int)puVar8, (int)((ulong)puVar8 >> 0x10));
                SetFocus16((HWND16)&PTR_LOOP_1050_1038);
                return;
            }
            if(0x6e < param_5)
            {
            LAB_1040_09f9:
                post_win_msg_1040_7b3c(
                  (ulong *)CONCAT13(uVar9, CONCAT12((char)param_2, param_1)), param_3, param_4, param_5, param_7);
                return;
            }
            if((char)param_5 == '\x02')
            {
            LAB_1040_09b4:
                post_win_msg_1040_7b3c(
                  (ulong *)CONCAT13(uVar9, CONCAT12((char)param_2, param_1)), 0x0, 0x0, 0x2, param_7);
                PostMessage16(param_7, 0x0, 0x0, 0x11100ee);
                return;
            }
            if((char)param_5 != 'd')
                goto LAB_1040_09f9;
            uVar9  = 0x0;
            uVar10 = 0x0;
            uVar6  = SUB42(s_tile2_bmp_1050_1538, 0x0);
            PostMessage16(param_7, 0x0, 0x0, 0x1110064);
            goto LAB_1040_0821;
        }
        if(param_5 != 0x74)
        {
            if(param_5 == 0xee)
                goto LAB_1040_09b4;
            if(param_5 == 0x13d)
            {
                enable_window_1040_0acc(param_1, param_2, 0x1, param_7);
                return;
            }
            goto LAB_1040_09f9;
        }
        enable_window_1040_0acc(param_1, param_2, 0x0, param_7);
        load_string_1010_84e0(0x1010,
                              (ushort)_PTR_LOOP_1050_14cc,
                              (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                              0x3ff,
                              (char *)local_406,
                              param_8);
        load_string_1010_84e0(
          0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_806, param_8);
        uVar6 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        IVar2
          = MessageBox16(0x1010, (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x14), (LPCSTR)local_406, param_8);
        if(IVar2 == 0x6)
        {
            PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x111007a);
            BVar3 = post_win_msg_1040_7b3c(
              (ulong *)CONCAT22(param_2, param_1), param_3, param_4, 0x1, (int)s_tile2_bmp_1050_1538);
            uStack6 = CONCAT22(param_6, BVar3);
            puVar7  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_8, (uchar *)param_6, unaff_DI);
            uVar6   = 0x1010;
            pass1_1010_60fa((ulong)puVar7);
        }
    }
    uVar9  = 0x1;
    uVar10 = 0x0;
LAB_1040_0821:
    enable_window_1040_0acc(param_1, param_2, CONCAT11(uVar10, uVar9), uVar6);
    return;
}

void __stdcall16far enable_window_1040_0acc(ushort param_1, ushort param_2, BOOL16 param_3, HWND16 param_4)

{
    BOOL16 BVar1;

    BVar1 = IsWindow16(param_4);
    if(BVar1 != 0x0)
    {
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x64);
        BVar1 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
        if(BVar1 != 0x0)
        {
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x74);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x73);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x6e);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0xee);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, param_3);
        }
    }
    return;
}

void __stdcall16far pass1_1040_0c54(astruct_18 *param_1, ushort param_2)

{
    undefined2 uVar1;

    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                   = 0xdb0;
    *(undefined2 *)((int)param_1 + 0x2)  = (int)&PTR_LOOP_1050_1040;
    *(undefined4 *)((int)param_1 + 0x8e) = 0x0;
    ui_cleanup_op_1040_782c(param_1, param_2);
    return;
}

void __stdcall16far show_win_1040_0c7c(astruct_1 *param_1, ushort param_2, ushort param_3)

{
    undefined4 uVar1;
    undefined4 local_6;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
    pass1_1010_4f30((ushort)uVar1,
                    (ushort)((ulong)uVar1 >> 0x10),
                    (ushort *)CONCAT22(param_3, &local_6),
                    (ushort *)CONCAT22(param_3, (int)&local_6 + 0x2));
    move_win_1040_826c(param_1, (INT16)local_6, (BOOL16)((ulong)local_6 >> 0x10));
    ShowWindow16(0x1010, 0x5);
    return;
}

void __stdcall16far pass1_1038_e03e(ulong param_1)

{
    undefined4 uVar1;
    ushort     uVar2;
    int        iVar3;
    undefined2 uVar4;
    undefined2 uVar5;
    ulong      uVar6;
    int        iStack6;

    uVar4 = (undefined2)(param_1 >> 0x10);
    uVar2 = pass1_1010_0886();
    for(iStack6 = 0x1; iStack6 <= (int)uVar2; iStack6 = iStack6 + 0x1)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x92);
        uVar6 = pass1_1010_08e2((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), iStack6);
        uVar1 = *(undefined4 *)((int)param_1 + 0x96);
        uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
        iVar3 = (int)uVar1;
        if(*(long *)(iVar3 + iStack6 * 0x4) != 0x0)
        {
            enable_win_1040_9234(
              *(ulong *)(iVar3 + iStack6 * 0x4), *(BOOL16 *)((int)uVar6 + 0x6), (int)&PTR_LOOP_1050_1040);
        }
    }
    return;
}

void __stdcall16far pass1_1038_e16e(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xe264;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}

void __stdcall16far check_radio_btn_show_win_1038_e19a(astruct_1 *param_1)

{
    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    CheckRadioButton16((HWND16)&PTR_LOOP_1050_1040, 0x1807, 0x1807, 0x1807);
    move_win_1040_826c(param_1, 0xc8, 0xc8);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    return;
}

void __stdcall16far pass1_1038_e308(astruct_18 *param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    param_1->field_0x0           = 0xe62e;
    *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)(iVar1 + 0x6));
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x8e), 0x1000);
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}

void __stdcall16far win_ui_op_1038_e348(astruct_1 *param_1)

{
    undefined4   uVar1;
    ushort       uVar2;
    astruct_160 *rect;
    uchar       *in_DX;
    uchar       *puVar3;
    uint         uVar4;
    int          iVar5;
    int          iVar6;
    int          unaff_DI;
    undefined2   uVar7;
    undefined2   uVar8;
    ushort       unaff_SS;
    undefined2   local_22;
    undefined2   uStack32;
    undefined2   uStack30;
    undefined2   uStack28;
    undefined2  *puStack26;
    int          iStack10;
    ushort       uStack8;
    ushort      *puStack6;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    puStack6           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, unaff_SS, in_DX, unaff_DI);
    PTR_LOOP_1050_5f2e = (undefined *)((ulong)puStack6 >> 0x10);
    uStack8            = pass1_1010_088c();
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e, 0x1000);
    }
    else
    {
    }
    puStack26 = (undefined2 *)CONCAT22(PTR_LOOP_1050_5f2e, PTR_LOOP_1050_5f2c);
    uVar2     = fn_ptr_op_1000_1708(
      (uStack8 + 0x2) * 0x4, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    uVar7                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar5                         = (int)param_1;
    *(ushort *)(iVar5 + 0x8e)     = uVar2;
    *(undefined2 *)(iVar5 + 0x90) = PTR_LOOP_1050_5f2e;
    for(iStack10 = 0x1; iStack10 <= (int)uStack8; iStack10 = iStack10 + 0x1)
    {
        puStack26 = (undefined2 *)pass1_1010_091e((ushort)puStack6, (ushort)((ulong)puStack6 >> 0x10), iStack10);
        puVar3    = (uchar *)((ulong)puStack26 >> 0x10);
        local_22  = *puStack26;
        uStack32  = *(undefined2 *)((int)puStack26 + 0x2);
        uStack30  = 0x1;
        uStack28  = 0x1;
        rect      = (astruct_160 *)&local_22;
        MapDialogRect16(0x1010, (RECT16 *)rect);
        mem_op_1000_179c(0x42, puVar3, 0x1000);
        uVar4 = (uint)puVar3 | (uint)rect;
        if(uVar4 == 0x0)
        {
            uVar1                                        = *(undefined4 *)(iVar5 + 0x8e);
            *(undefined4 *)((int)uVar1 + iStack10 * 0x4) = 0x0;
        }
        else
        {
            pass1_1008_3bd6(rect,
                            (ushort)puVar3,
                            0x0,
                            CONCAT22(local_22, uStack32),
                            0x101,
                            0xff0100,
                            CONCAT22(*(undefined2 *)(iVar5 + 0x6), *(undefined2 *)((int)puStack26 + 0x4)),
                            uVar4,
                            unaff_SS);
            uVar1                                     = *(undefined4 *)(iVar5 + 0x8e);
            uVar8                                     = (undefined2)((ulong)uVar1 >> 0x10);
            iVar6                                     = (int)uVar1;
            *(astruct_160 **)(iVar6 + iStack10 * 0x4) = rect;
            *(uint *)(iVar6 + iStack10 * 0x4 + 0x2)   = uVar4;
        }
        uVar1 = *(undefined4 *)(iVar5 + 0x8e);
        uVar8 = (undefined2)((ulong)uVar1 >> 0x10);
        iVar6 = (int)uVar1;
        if(*(long *)(iVar6 + iStack10 * 0x4) != 0x0)
        {
            enable_win_1040_9234(
              *(ulong *)(iVar6 + iStack10 * 0x4), *(BOOL16 *)((int)puStack26 + 0x6), (int)&PTR_LOOP_1050_1040);
        }
    }
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    return;
}

void __stdcall16far pass1_1038_e6f0(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xe92e;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}

void __stdcall16far unk_win_ui_op_1038_e71c(astruct_1 *param_1, UINT16 param_2)

{
    undefined2  extraout_DX;
    int         iVar1;
    undefined2  uVar2;
    undefined2  unaff_SS;
    astruct_18 *paStack6;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    iVar1 = (int)param_1;
    unk_load_str_op_1010_2c34(*(undefined4 *)(iVar1 + 0x8e));
    paStack6 = (astruct_18 *)CONCAT22(extraout_DX, param_2);
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x10)),
                         (char *)CONCAT22(extraout_DX, param_2));
    fn_ptr_1000_17ce(paStack6, 0x1000);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    *(undefined2 *)(iVar1 + 0x92) = 0x1;
    unk_win_msg_op_1008_9510((int *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x92)), 0x1008, unaff_SS);
    DestroyWindow16(0x1008);
    return;
}

void __stdcall16far chk_is_dlg_btn_checked_1038_e7a0(ulong param_1, int param_2)

{
    undefined4  uVar1;
    UINT16      UVar2;
    astruct_62 *iVar3;
    undefined2  uVar3;
    HWND16      unaff_CS;

    iVar3 = (astruct_62 *)param_1;
    uVar3 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        uVar1                              = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0x10) = 0x1;
        uVar1                              = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xa)  = 0x0;
        uVar1                              = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xc)  = 0x0;
        uVar1                              = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xe)  = 0x0;
    }
    else
    {
        UVar2 = IsDlgButtonChecked(unaff_CS, 0x1827);
        if(UVar2 == 0x0)
        {
            UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0x1828);
            if(UVar2 == 0x0)
            {
                uVar1                             = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
            }
            else
            {
                uVar1                             = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xa) = 0x2;
            }
        }
        else
        {
            uVar1                             = iVar3->field_0x8e;
            *(undefined2 *)((int)uVar1 + 0xa) = 0x1;
        }
        UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, (UINT16)s_vrpal_bmp_1050_183a);
        if(UVar2 == 0x0)
        {
            UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, (int)s_vrpal_bmp_1050_183a + 0x1);
            if(UVar2 == 0x0)
            {
                uVar1                             = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xc) = 0x0;
            }
            else
            {
                uVar1                             = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xc) = 0x2;
            }
        }
        else
        {
            uVar1                             = iVar3->field_0x8e;
            *(undefined2 *)((int)uVar1 + 0xc) = 0x1;
        }
        UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, (int)s_vrpal_bmp_1050_183a + 0x2);
        if(UVar2 == 0x0)
        {
            UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, (int)s_vrpal_bmp_1050_183a + 0x3);
            if(UVar2 == 0x0)
            {
                uVar1                             = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xe) = 0x0;
            }
            else
            {
                uVar1                             = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xe) = 0x2;
            }
        }
        else
        {
            uVar1                             = iVar3->field_0x8e;
            *(undefined2 *)((int)uVar1 + 0xe) = 0x1;
        }
        uVar1                              = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0x10) = 0x0;
    }
    iVar3->field_0x92 = 0x0;
    return;
}

void __stdcall16far pass1_1038_e9ec(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xeb32;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}

void __stdcall16far win_ui_op_1038_ea18(astruct_1 *param_1)

{
    INT16  IVar1;
    BOOL16 BVar2;
    RECT16 local_10[0x2];
    HWND16 HStack8;
    ulong  uStack6;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    uStack6 = pass1_1010_375e(*(ulong *)((int)param_1 + 0x8e));
    HStack8 = GetDlgItem16(0x1010, 0xfa5);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, (UINT16)uStack6, (WPARAM16)(uStack6 >> 0x10), 0xc0000);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, local_10);
    BVar2 = 0x4;
    IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    move_win_1040_826c(param_1, IVar1 + local_10[0].y + 0x5, BVar2);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    return;
}

void __stdcall16far win_ui_op_1038_eaa2(ulong param_1, int param_2, HWND16 param_3, WPARAM16 param_4)

{
    LRESULT   LVar1;
    undefined local_54[0x52];

    if(param_2 != 0x0)
    {
        GetDlgItem16(param_3, 0xfa5);
        LVar1 = SendMessage16((HWND16)s_tile2_bmp_1050_1538, (UINT16)local_54, param_4, 0xd0050);
        pass1_1010_3770(
          *(ulong *)((int)param_1 + 0x8e), (char *)CONCAT22(param_4, local_54), (ushort)((ulong)LVar1 >> 0x10));
        param_3 = (HWND16)s_tile2_bmp_1050_1538;
        PostMessage16(0x1010, 0x0, 0x0, 0x11100fb);
    }
    DestroyWindow16(param_3);
    return;
}

void __stdcall16far win_dlg_op_1038_c95e(ulong param_1, int param_2)

{
    undefined4 uVar1;
    UINT16     UVar2;
    int        iVar3;
    undefined2 uVar4;
    HWND16     unaff_CS;

    iVar3 = (int)param_1;
    uVar4 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        uVar1                             = *(undefined4 *)(iVar3 + 0x8e);
        *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
    }
    else
    {
        UVar2 = IsDlgButtonChecked(unaff_CS, 0xfac);
        if(UVar2 == 0x0)
        {
            unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
            UVar2    = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0xfad);
            if(UVar2 == 0x0)
            {
                unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
                UVar2    = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0xfae);
                if(UVar2 == 0x0)
                {
                    unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
                    UVar2    = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0xfaf);
                    if(UVar2 == 0x0)
                    {
                        unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
                        UVar2    = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0xfb0);
                        if(UVar2 != 0x0)
                        {
                            uVar1                             = *(undefined4 *)(iVar3 + 0x8e);
                            *(undefined2 *)((int)uVar1 + 0xa) = 0x5;
                        }
                    }
                    else
                    {
                        uVar1                             = *(undefined4 *)(iVar3 + 0x8e);
                        *(undefined2 *)((int)uVar1 + 0xa) = 0x4;
                    }
                }
                else
                {
                    uVar1                             = *(undefined4 *)(iVar3 + 0x8e);
                    *(undefined2 *)((int)uVar1 + 0xa) = 0x3;
                }
            }
            else
            {
                uVar1                             = *(undefined4 *)(iVar3 + 0x8e);
                *(undefined2 *)((int)uVar1 + 0xa) = 0x2;
            }
        }
        else
        {
            uVar1                             = *(undefined4 *)(iVar3 + 0x8e);
            *(undefined2 *)((int)uVar1 + 0xa) = 0x1;
            unaff_CS                          = (HWND16)s_tile2_bmp_1050_1538;
        }
    }
    DestroyWindow16(unaff_CS);
    PTR_LOOP_1050_5b80 = (undefined *)0x0;
    return;
}

void __stdcall16far pass1_1038_cb30(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xcc9a;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far show_win_1038_cb5c(astruct_1 *param_1)

{
    undefined4  uVar1;
    ushort      uVar2;
    ushort      in_DX;
    uchar      *puVar3;
    uint        uVar4;
    undefined2  uVar5;
    WNDCLASS16 *unaff_SS;
    undefined2 *puVar6;
    undefined2 *puVar7;
    int         iStack10;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    uVar2 = pass1_1008_eb6e();
    for(iStack10 = 0x0; iStack10 < (int)uVar2; iStack10 = iStack10 + 0x1)
    {
        uVar1  = *(undefined4 *)((int)param_1 + 0x8e);
        puVar6 = (undefined2 *)pass1_1008_eb5c((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), iStack10);
        puVar3 = (uchar *)((ulong)puVar6 >> 0x10);
        puVar7 = puVar6;
        mem_op_1000_179c(0x42, puVar3, 0x1000);
        uVar4 = (uint)((ulong)puVar7 >> 0x10);
        in_DX = uVar4 | (uint)(astruct_160 *)puVar7;
        if(puVar7 != (undefined2 *)0x0)
        {
            pass1_1008_3bd6((astruct_160 *)puVar7,
                            uVar4,
                            0x0,
                            CONCAT22(*puVar6, *(undefined2 *)((int)puVar6 + 0x2)),
                            0x101,
                            0xff0100,
                            CONCAT22(*(undefined2 *)((int)param_1 + 0x6), *(undefined2 *)((int)puVar6 + 0x4)),
                            in_DX,
                            (ushort)unaff_SS);
        }
    }
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x90001, unaff_SS, uVar2, in_DX);
    ShowWindow16(0x1008, 0x5);
    return;
}


void __stdcall16far destroy_window_1038_cc00(int param_1, UINT16 param_2, UINT16 param_3, ULONG param_4)

{
    ushort      uVar1;
    uchar      *in_DX;
    int         unaff_DI;
    WNDCLASS16 *unaff_SS;
    int         iVar2;

    uVar1 = param_4._2_2_ - 0x1cd;
    if(uVar1 == 0x0)
    {
        iVar2 = 0x1;
    }
    else
    {
        uVar1 = param_4._2_2_ - 0x1ce;
        if(uVar1 == 0x0)
        {
            iVar2 = 0x2;
        }
        else
        {
            uVar1 = param_4._2_2_ - 0x1cf;
            if(uVar1 == 0x0)
            {
                iVar2 = 0x3;
            }
            else
            {
                uVar1 = param_4._2_2_ - 0x1d0;
                if(uVar1 == 0x0)
                {
                    iVar2 = 0x4;
                }
                else
                {
                    uVar1 = param_4._2_2_ - 0x1d1;
                    if(uVar1 != 0x0)
                    {
                        post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2, param_1),
                                               param_3,
                                               (ushort)param_4,
                                               param_4._2_2_,
                                               (int)&PTR_LOOP_1050_1040);
                        return;
                    }
                    iVar2 = 0x5;
                }
            }
        }
    }
    pass1_1008_eb74(*(undefined4 *)(param_1 + 0x8e), iVar2, in_DX, unaff_DI, (ushort)unaff_SS);
    if(uVar1 != 0x0)
    {
        win_1008_5c7c(_PTR_LOOP_1050_02a0, CONCAT22(uVar1, 0x1), unaff_SS, uVar1, (ushort)in_DX);
        DestroyWindow16(0x1008);
    }
    return;
}


void __stdcall16far pass1_1038_cd5c(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xcf00;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far destroy_window_1038_cd88(astruct_1 *param_1)

{
    undefined2 unaff_SS;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    *(undefined2 *)((int)param_1 + 0x92) = 0x1;
    unk_win_msg_op_1008_9510((int *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x92)), 0x1008, unaff_SS);
    DestroyWindow16(0x1008);
    return;
}


void __stdcall16far check_dlg_btn_checked_1038_cdd6(ulong param_1, int param_2, HWND16 param_3)

{
    undefined4  uVar1;
    UINT16      UVar2;
    astruct_61 *iVar3;
    undefined2  uVar3;

    iVar3 = (astruct_61 *)param_1;
    uVar3 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        uVar1                             = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
    }
    else
    {
        UVar2 = IsDlgButtonChecked(param_3, 0x182e);
        if(UVar2 == 0x0)
        {
            UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0x182f);
            if(UVar2 == 0x0)
            {
                UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0x1829);
                if(UVar2 == 0x0)
                {
                    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0x182a);
                    if(UVar2 == 0x0)
                    {
                        UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0x182c);
                        if(UVar2 == 0x0)
                        {
                            UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, 0x182d);
                            if(UVar2 != 0x0)
                            {
                                uVar1                             = iVar3->field_0x8e;
                                *(undefined2 *)((int)uVar1 + 0xa) = 0x7;
                            }
                        }
                        else
                        {
                            uVar1                             = iVar3->field_0x8e;
                            *(undefined2 *)((int)uVar1 + 0xa) = 0x6;
                        }
                    }
                    else
                    {
                        uVar1                             = iVar3->field_0x8e;
                        *(undefined2 *)((int)uVar1 + 0xa) = 0x4;
                    }
                }
                else
                {
                    uVar1                             = iVar3->field_0x8e;
                    *(undefined2 *)((int)uVar1 + 0xa) = 0x3;
                }
            }
            else
            {
                uVar1                             = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xa) = 0x2;
            }
        }
        else
        {
            uVar1                             = iVar3->field_0x8e;
            *(undefined2 *)((int)uVar1 + 0xa) = 0x1;
        }
    }
    iVar3->field_0x92 = 0x0;
    return;
}


void __stdcall16far pass1_1038_d276(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xd6ea;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far win_ui_op_1038_d2a2(astruct_1 *param_1)

{
    astruct_160 *rect;
    int          iVar1;
    BOOL16       BVar2;
    uchar       *in_DX;
    uchar       *puVar3;
    ushort       uVar4;
    int          unaff_DI;
    undefined2   uVar5;
    HWND16       hwnd;
    HWND16       hwnd_00;
    WNDCLASS16  *unaff_SS;
    char        *pcVar6;
    LRESULT      LVar7;
    WPARAM16     w_param;
    undefined2   uVar8;
    undefined2   uVar9;
    ushort       uVar10;
    undefined2   local_16;
    undefined2   uStack20;
    undefined2   uStack18;
    undefined2   uStack16;
    undefined4   uStack14;
    int          iStack10;
    undefined4   uStack8;
    int          iStack4;

    hwnd = (HWND16)&PTR_LOOP_1050_1040;
    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    iStack4 = 0x7;
    for(iStack10 = 0x0; uVar5 = (undefined2)((ulong)param_1 >> 0x10), iStack10 < iStack4; iStack10 = iStack10 + 0x1)
    {
        unaff_DI = iStack10 * 0xc;
        local_16 = *(undefined2 *)(unaff_DI + 0x5c0c);
        uStack20 = *(undefined2 *)(unaff_DI + 0x5c0e);
        uStack18 = 0x1;
        uStack16 = 0x1;
        rect     = (astruct_160 *)&local_16;
        MapDialogRect16(hwnd, (RECT16 *)rect);
        hwnd_00 = 0x1000;
        mem_op_1000_179c(0x42, in_DX, 0x1000);
        puVar3 = (uchar *)((uint)in_DX | (uint)rect);
        if(puVar3 == (uchar *)0x0)
        {
            rect  = (astruct_160 *)0x0;
            in_DX = (uchar *)0x0;
        }
        else
        {
            hwnd_00 = 0x1008;
            pass1_1008_3bd6(rect,
                            (ushort)in_DX,
                            0x1,
                            CONCAT22(local_16, uStack20),
                            0x104,
                            0x1020103,
                            CONCAT22(*(undefined2 *)((int)param_1 + 0x6), *(undefined2 *)(unaff_DI + 0x5c10)),
                            (ushort)puVar3,
                            (ushort)unaff_SS);
            in_DX = puVar3;
        }
        uStack8 = CONCAT22(in_DX, rect);
        hwnd    = hwnd_00;
        if(*(int *)(iStack10 * 0xc + 0x5c12) == 0x0)
        {
            hwnd = (HWND16)s_tile2_bmp_1050_1538;
            EnableWindow16(hwnd_00, 0x0);
        }
    }
    uVar10   = 0x86;
    uStack14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, (ushort)unaff_SS, in_DX, unaff_DI);
    uVar4    = (ushort)((ulong)uStack14 >> 0x10);
    iVar1    = pass1_1010_659a((ulong)uStack14, uVar10, (ushort)unaff_SS);
    if(iVar1 == 0x0)
    {
        GetDlgItem16(0x1010, 0x14a);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        uVar8   = 0xc;
        uVar9   = 0x144;
        w_param = 0x0;
        pcVar6  = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        LVar7   = SendDlgItemMessage16(
          0x1010, (INT16)pcVar6, (UINT16)((ulong)pcVar6 >> 0x10), w_param, CONCAT22(uVar9, uVar8));
        uVar4 = (ushort)((ulong)LVar7 >> 0x10);
    }
    move_win_1040_826c(param_1, -0x1, 0xffff);
    BVar2 = ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x9a0001, unaff_SS, BVar2, uVar4);
    *(BOOL16 *)((int)param_1 + 0x8c) = BVar2;
    return;
}


void __stdcall16far unk_win_ui_op_1038_d400(UCHAR param_1, UINT16 param_2, UINT16 param_3, ULONG param_4)

{
    int         iVar1;
    ushort      uVar2;
    BOOL16      BVar3;
    ushort      in_DX;
    uchar      *puVar4;
    int         unaff_DI;
    HWND16      hwnd;
    HWND16      hwnd_00;
    WNDCLASS16 *unaff_SS;
    ushort     *puVar5;
    LRESULT     LVar6;
    char       *pcVar7;
    undefined   in_stack_00000005;
    WPARAM16    WVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    ushort      uVar11;
    undefined   local_c[0x4];
    int         iStack8;
    undefined4  uStack6;

    uStack6 = 0x0;
    iStack8 = 0x0;
    switch(param_4._2_2_)
    {
    case 0x145:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1038, 0x146);
        uVar2   = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        uStack6 = 0x13f0647;
        uVar11  = 0x1f1;
        goto LAB_1038_d490;
    case 0x146:
        uStack6 = 0x1400648;
        puVar5  = pass1_1008_941a((ushort *)CONCAT22(unaff_SS, local_c), 0x1, 0xc4);
        puVar4  = (uchar *)((ulong)puVar5 >> 0x10);
        win_1008_5c9e(_PTR_LOOP_1050_02a0, (ulong *)CONCAT22(unaff_SS, local_c), local_c, puVar4, unaff_SS);
        uVar2  = 0x86;
        puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, (ushort)unaff_SS, puVar4, unaff_DI);
        pass1_1010_6604((ulong)puVar5, uVar2, (ushort)unaff_SS);
        GetDlgItem16(0x1010, 0x145);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        uVar9  = 0xc;
        uVar10 = 0x13f;
        WVar8  = 0x0;
        pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        LVar6  = SendDlgItemMessage16(
          0x1010, (INT16)pcVar7, (UINT16)((ulong)pcVar7 >> 0x10), WVar8, CONCAT22(uVar10, uVar9));
        puVar4 = (uchar *)((ulong)LVar6 >> 0x10);
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x146);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        iVar1 = pass1_1010_659a((ulong)puVar5, 0x86, (ushort)unaff_SS);
        if(iVar1 == 0x0)
        {
            GetDlgItem16(0x1010, 0x14a);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
            uVar9  = 0xc;
            uVar10 = 0x144;
            WVar8  = 0x0;
            pcVar7
              = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            LVar6 = SendDlgItemMessage16(
              0x1010, (INT16)pcVar7, (UINT16)((ulong)pcVar7 >> 0x10), WVar8, CONCAT22(uVar10, uVar9));
            puVar4 = (uchar *)((ulong)LVar6 >> 0x10);
        }
        hwnd   = 0x1010;
        puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, (ushort)unaff_SS, puVar4, unaff_DI);
        if(*(int *)((int)puVar5 + 0x20) != 0x0)
        {
            hwnd = (HWND16)s_tile2_bmp_1050_1538;
            PostMessage16(0x1010, 0x0, 0x0, 0x11100af);
        }
        break;
    case 0x147:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1038, 0x148);
        uVar2   = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        uStack6 = 0x1410647;
        uVar11  = 0x1f5;
        goto LAB_1038_d490;
    case 0x148:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1038, 0x149);
        uVar2   = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        uStack6 = 0x1420647;
        uVar11  = 0x1f2;
    LAB_1038_d490:
        hwnd = 0x1008;
        win_1008_5c5c(unaff_SS, uVar2, in_DX, _PTR_LOOP_1050_02a0, uVar11);
        break;
    case 0x149:
        uStack6 = 0x1430648;
        PostMessage16((HWND16)&PTR_LOOP_1050_1038, 0x0, 0x0, 0x11100b8);
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
        break;
    case 0x14a:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1038, 0x145);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        uVar9  = 0xc;
        uVar10 = 0x140;
        WVar8  = 0x0;
        pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        hwnd   = (HWND16)s_tile2_bmp_1050_1538;
        SendDlgItemMessage16(0x1010, (INT16)pcVar7, (UINT16)((ulong)pcVar7 >> 0x10), WVar8, CONCAT22(uVar10, uVar9));
        break;
    case 0x14b:
        GetDlgItem16((HWND16)&PTR_LOOP_1050_1038, 0x147);
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        break;
    default:
        post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2, CONCAT11(in_stack_00000005, param_1)),
                               param_3,
                               (ushort)param_4,
                               param_4._2_2_,
                               (int)&PTR_LOOP_1050_1040);
        return;
    }
    hwnd_00 = hwnd;
    if((uStack6._2_2_ != 0x0) && ((int)uStack6 != 0x0))
    {
        hwnd_00 = (HWND16)s_tile2_bmp_1050_1538;
        BVar3   = IsWindow16(hwnd);
        if(BVar3 != 0x0)
        {
            WVar8 = 0x0;
            uVar9 = 0xc;
            pcVar7
              = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            hwnd_00 = (HWND16)s_tile2_bmp_1050_1538;
            SendDlgItemMessage16(
              0x1010, (INT16)pcVar7, (UINT16)((ulong)pcVar7 >> 0x10), WVar8, CONCAT22(uStack6._2_2_, uVar9));
        }
    }
    if(iStack8 != 0x0)
    {
        PostMessage16(hwnd_00, 0x0, 0x0, CONCAT22(0x111, iStack8));
    }
    return;
}


void __stdcall16far pass1_1038_d7d0(astruct_18 *param_1, ushort param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    param_1->field_0x0           = 0xe0d4;
    *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    if(*(int *)(iVar1 + 0x90) != 0x0)
    {
        pass1_1010_1ea6(_PTR_LOOP_1050_02a0, (long)param_1, param_2);
    }
    if(*(long *)(iVar1 + 0x92) != 0x0)
    {
        pass1_1010_1ea6(*(ulong *)(iVar1 + 0x92), (long)param_1, param_2);
    }
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)(iVar1 + 0x6));
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x96), 0x1000);
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far destroy_crsor_1038_d8b2(int param_1, HINSTANCE16 param_2, ushort param_3)

{
    int         *piVar1;
    undefined4   uVar2;
    undefined4   uVar3;
    HCURSOR16    HVar4;
    ushort       uVar5;
    astruct_160 *rect;
    uchar       *in_DX;
    uchar       *puVar6;
    int          iVar7;
    int          iVar8;
    int          unaff_DI;
    undefined2   uVar9;
    undefined2  *puVar10;
    ushort      *puVar11;

    HVar4                          = LoadCursor16(param_2, (LPCSTR)0x7f02);
    *(HCURSOR16 *)(param_1 + -0x2) = HVar4;
    HVar4                          = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    *(HCURSOR16 *)(param_1 + -0x4) = HVar4;
    dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_1 + 0x6), (int)&PTR_LOOP_1050_1040);
    uVar5                       = pass1_1010_0886();
    *(ushort *)(param_1 + -0x6) = uVar5;
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)in_DX, 0x1000);
        PTR_LOOP_1050_5f2e = in_DX;
    }
    else
    {
    }
    *(uchar **)(param_1 + -0x1c) = PTR_LOOP_1050_5f2c;
    *(uchar **)(param_1 + -0x1a) = PTR_LOOP_1050_5f2e;
    uVar5                        = fn_ptr_op_1000_1708(
      (*(int *)(param_1 + -0x6) + 0x2) * 0x4, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    uVar2                           = *(undefined4 *)(param_1 + 0x6);
    uVar9                           = (undefined2)((ulong)uVar2 >> 0x10);
    iVar7                           = (int)uVar2;
    *(ushort *)(iVar7 + 0x96)       = uVar5;
    *(uchar **)(iVar7 + 0x98)       = PTR_LOOP_1050_5f2e;
    *(undefined2 *)(param_1 + -0x8) = 0x1;
    while(iVar7 = *(int *)(param_1 + -0x6), piVar1 = (int *)(param_1 + -0x8), *piVar1 == iVar7 || *piVar1 < iVar7)
    {
        uVar2 = *(undefined4 *)(param_1 + 0x6);
        uVar2 = *(undefined4 *)((int)uVar2 + 0x92);
        puVar10
          = (undefined2 *)pass1_1010_08e2((ushort)uVar2, (ushort)((ulong)uVar2 >> 0x10), *(int *)(param_1 + -0x8));
        puVar6                           = (uchar *)((ulong)puVar10 >> 0x10);
        *(int *)(param_1 + -0x1c)        = (int)puVar10;
        *(uchar **)(param_1 + -0x1a)     = puVar6;
        *(undefined2 *)(param_1 + -0x24) = *puVar10;
        *(undefined2 *)(param_1 + -0x22) = *(undefined2 *)((int)puVar10 + 0x2);
        *(undefined2 *)(param_1 + -0x20) = 0x1;
        *(undefined2 *)(param_1 + -0x1e) = 0x1;
        rect                             = (astruct_160 *)(param_1 + -0x24);
        MapDialogRect16(0x1010, (RECT16 *)rect);
        mem_op_1000_179c(0x42, puVar6, 0x1000);
        *(astruct_160 **)(param_1 + -0x28) = rect;
        *(uchar **)(param_1 + -0x26)       = puVar6;
        PTR_LOOP_1050_5f2e                 = (undefined *)((uint)puVar6 | (uint)rect);
        if(PTR_LOOP_1050_5f2e == (uchar *)0x0)
        {
            uVar2                                                        = *(undefined4 *)(param_1 + 0x6);
            uVar2                                                        = *(undefined4 *)((int)uVar2 + 0x96);
            *(undefined4 *)((int)uVar2 + *(int *)(param_1 + -0x8) * 0x4) = 0x0;
        }
        else
        {
            uVar2 = *(undefined4 *)(param_1 + 0x6);
            uVar3 = *(undefined4 *)(param_1 + -0x1c);
            pass1_1008_3bd6(rect,
                            *(ushort *)(param_1 + -0x26),
                            0x0,
                            CONCAT22(*(undefined2 *)(param_1 + -0x24), *(undefined2 *)(param_1 + -0x22)),
                            0x101,
                            0xff0100,
                            CONCAT22(*(undefined2 *)((int)uVar2 + 0x6), *(undefined2 *)((int)uVar3 + 0x4)),
                            (ushort)PTR_LOOP_1050_5f2e,
                            param_3);
            uVar2                            = *(undefined4 *)(param_1 + 0x6);
            uVar2                            = *(undefined4 *)((int)uVar2 + 0x96);
            uVar9                            = (undefined2)((ulong)uVar2 >> 0x10);
            iVar7                            = (int)uVar2;
            iVar8                            = *(int *)(param_1 + -0x8) * 0x4;
            *(astruct_160 **)(iVar7 + iVar8) = rect;
            *(uchar **)(iVar7 + iVar8 + 0x2) = PTR_LOOP_1050_5f2e;
        }
        uVar2 = *(undefined4 *)(param_1 + 0x6);
        uVar2 = *(undefined4 *)((int)uVar2 + 0x96);
        uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar7 = (int)uVar2;
        iVar8 = *(int *)(param_1 + -0x8) * 0x4;
        if(*(long *)(iVar7 + iVar8) != 0x0)
        {
            uVar2                              = *(undefined4 *)(iVar7 + iVar8);
            *(undefined2 *)((int)uVar2 + 0x3e) = 0x1;
            uVar2                              = *(undefined4 *)(param_1 + -0x1c);
            uVar3                              = *(undefined4 *)(param_1 + 0x6);
            uVar3                              = *(undefined4 *)((int)uVar3 + 0x96);
            enable_win_1040_9234(*(ulong *)((int)uVar3 + *(int *)(param_1 + -0x8) * 0x4),
                                 *(BOOL16 *)((int)uVar2 + 0x6),
                                 (int)&PTR_LOOP_1050_1040);
        }
        piVar1  = (int *)(param_1 + -0x8);
        *piVar1 = *piVar1 + 0x1;
    }
    puVar11                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_3, PTR_LOOP_1050_5f2e, unaff_DI);
    *(undefined2 *)(param_1 + -0xc) = (int)puVar11;
    *(undefined2 *)(param_1 + -0xa) = (int)((ulong)puVar11 >> 0x10);
    uVar2                           = *(undefined4 *)(param_1 + -0xc);
    SetWindowText16(0x1010, (SEGPTR) * (undefined4 *)((int)uVar2 + 0x68));
    ShowWindow16((HWND16)s_tile2_bmp_1050_1538, 0x5);
    SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    return;
}


void __stdcall16far show_win_1038_b634(ulong param_1, HWND16 param_2)

{
    int        iVar1;
    undefined2 uVar2;
    HWND16     HVar3;
    uint       uStack4;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0xac) == 0x0)
    {
        *(undefined2 *)(iVar1 + 0xac) = 0x1;
        for(uStack4 = 0x1; uStack4 < 0x2b; uStack4 = uStack4 + 0x1)
        {
            HVar3 = param_2;
            if((*(uint *)(uStack4 * 0x4 + iVar1 + 0x2) | *(uint *)(uStack4 * 0x4 + iVar1)) != 0x0)
            {
                HVar3 = (HWND16)s_tile2_bmp_1050_1538;
                ShowWindow16(param_2, 0x0);
            }
            param_2 = HVar3;
        }
    }
    return;
}


void __stdcall16far show_win_1038_b68a(ulong param_1, HWND16 param_2)

{
    int        iVar1;
    undefined2 uVar2;
    HWND16     HVar3;
    uint       uStack4;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0xac) != 0x0)
    {
        *(undefined2 *)(iVar1 + 0xac) = 0x0;
        for(uStack4 = 0x1; uStack4 < 0x2b; uStack4 = uStack4 + 0x1)
        {
            HVar3 = param_2;
            if((*(uint *)(uStack4 * 0x4 + iVar1 + 0x2) | *(uint *)(uStack4 * 0x4 + iVar1)) != 0x0)
            {
                HVar3 = (HWND16)s_tile2_bmp_1050_1538;
                ShowWindow16(param_2, 0x1);
            }
            param_2 = HVar3;
        }
    }
    return;
}


BOOL16 __stdcall16far bring_win_to_top_1038_b72e(u32 param_1, i16 param_2, HWND16 in_win_handle_3)

{
    if(*(long *)(param_2 * 0x4 + (int)param_1) != 0x0)
    {
        SetFocus16(in_win_handle_3);
        BringWindowToTop16((HWND16)s_tile2_bmp_1050_1538);
        return 0x1;
    }
    return 0x0;
}


void __stdcall16far pass1_1038_b7f0(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xbd70;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far win_ui_op_1038_b81c(astruct_1 *param_1)

{
    ulong      uVar1;
    undefined4 uVar2;
    code     **ppcVar3;
    uint       uVar4;
    BOOL16     win_enabled;
    uchar     *in_DX;
    undefined2 extraout_DX;
    int        iVar6;
    int        unaff_DI;
    undefined2 uVar7;
    HWND16     HVar8;
    HWND16     hwnd_dlg;
    ushort     unaff_SS;
    ushort    *puVar9;
    int       *piStack16;
    UINT16     UStack12;
    int        iStack10;
    astruct_1 *iVar7;
    int       *piVar5;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    puVar9                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, unaff_SS, in_DX, unaff_DI);
    uVar7                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar6                         = (int)param_1;
    *(undefined2 *)(iVar6 + 0x92) = (int)puVar9;
    *(undefined2 *)(iVar6 + 0x94) = (int)((ulong)puVar9 >> 0x10);
    uVar1                         = *(ulong *)(iVar6 + 0x92);
    uVar4                         = (int)uVar1 + 0x4e;
    uVar1                         = uVar1 & 0xffff0000;
    piVar5                        = (int *)(uVar1 | uVar4);
    iStack10                      = 0x0;
    hwnd_dlg                      = 0x1010;
    for(UStack12 = 0x1a0; (int)UStack12 < 0x1b5; UStack12 = UStack12 + 0x1)
    {
        if(*(UINT16 *)(iStack10 * 0x2 + uVar4) == UStack12)
        {
            iStack10 = iStack10 + 0x1;
            HVar8    = hwnd_dlg;
        }
        else
        {
            HVar8 = (HWND16)s_tile2_bmp_1050_1538;
            CheckDlgButton16(hwnd_dlg, 0x2, UStack12);
        }
        hwnd_dlg = HVar8;
    }
    GetDlgItem16(hwnd_dlg, 0xfb1);
    win_enabled = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    uVar2       = *(undefined4 *)(iVar6 + 0x92);
    ppcVar3     = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x92) + 0x10);
    (**ppcVar3)((int)s_tile2_bmp_1050_1538, (int)uVar2, (int)((ulong)uVar2 >> 0x10));
    piStack16 = (int *)CONCAT22(extraout_DX, win_enabled);
    move_win_1040_826c(param_1, *(int *)(win_enabled + 0x2) + -0x2, *(int *)(win_enabled + 0x4) + *piStack16 + 0x3);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    pass1_1018_1c9a(*(ulong *)(iVar6 + 0x92), *piVar5);
    GetDlgItem16(0x1018, *piVar5);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    return;
}


ulong __stdcall16far
win_ui_op_1038_b922(ulong *param_1, ulong param_2, uint param_3, ushort param_4, HWND16 param_5, WNDCLASS16 *param_6)

{
    int        *piVar1;
    code      **ppcVar2;
    UINT16      UVar3;
    BOOL16      BVar4;
    uint        uVar5;
    uchar      *puVar6;
    int         iVar7;
    int         unaff_DI;
    undefined2  uVar8;
    undefined2  uVar9;
    LRESULT     LVar10;
    char       *pcVar11;
    astruct_57 *paVar12;
    undefined4  uVar13;
    CHAR       *pCVar14;
    WNDCLASS16 *pWVar15;
    undefined  *puVar16;
    uint        uStack1132;
    char        local_464[0x50];
    CHAR        local_414[0x400];
    ulong       uStack20;
    undefined  *puStack16;
    ushort     *puStack14;
    int         iStack10;
    HWND16      HStack8;
    BOOL16      BStack6;
    undefined2  uStack4;

    uVar13  = CONCAT22(param_4, HStack8);
    BStack6 = 0x0;
    uStack4 = 0x0;
    iVar7   = (int)param_1;
    uVar8   = (undefined2)((ulong)param_1 >> 0x10);
    if(param_3 < 0x1b5)
    {
        if(param_3 < 0x1a0)
        {
            uVar13 = CONCAT22(param_4, HStack8);
            if(param_3 != 0x2)
                goto LAB_1038_bbbf;
        }
        else
        {
            HStack8  = GetDlgItem16(param_5, param_3);
            LVar10   = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4000000);
            iStack10 = (int)LVar10;
            if(iStack10 == 0x2)
            {
                BStack6 = 0x0;
                uStack4 = 0x0;
                goto LAB_1038_bc26;
            }
            SendMessage16(
              (HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, CONCAT13(0x4, CONCAT12(0x1, (uint)(iStack10 == 0x0))));
            UVar3 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538, param_3);
            if(UVar3 == 0x0)
            {
                piVar1  = (int *)(iVar7 + 0x96);
                *piVar1 = *piVar1 + 0x1;
                if(*(int *)(iVar7 + 0x96) == 0x1)
                {
                    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0xfb1);
                    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
                }
            }
            else
            {
                piVar1  = (int *)(iVar7 + 0x96);
                *piVar1 = *piVar1 + -0x1;
                GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0xfb1);
                BVar4 = IsWindowEnabled16((HWND16)s_tile2_bmp_1050_1538);
                if(BVar4 == 0x0)
                {
                    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0xfb1);
                    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
                }
                if(*(int *)(iVar7 + 0x96) < 0x0)
                {
                    CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538, 0x0, *(UINT16 *)(iVar7 + 0x98));
                    *(undefined2 *)(iVar7 + 0x96) = 0x0;
                }
                *(uint *)(iVar7 + 0x98) = param_3;
                pass1_1018_1c9a(*(ulong *)(iVar7 + 0x92), param_3);
                puStack14 = (ushort *)pass1_1018_1e78(*(ulong *)(iVar7 + 0x92), -0x1);
                uVar5     = (uint)((ulong)puStack14 >> 0x10);
                if(puStack14 == (ushort *)0x0)
                {
                    puStack16 = (undefined *)0x0;
                }
                else
                {
                    puStack16 = (undefined *)*(ushort *)((uint)puStack14 + 0x1c);
                }
                win_1008_5c7c(
                  _PTR_LOOP_1050_02a0, CONCAT22(puStack16, 0x1), param_6, (ushort)puStack16, uVar5 | (uint)puStack14);
            }
        }
        BStack6 = 0x1;
        uStack4 = 0x0;
    }
    else
    {
        if(param_3 == 0xfb1)
        {
            for(uStack1132 = 0x1a0; uVar13 = CONCAT22(param_4, HStack8), uStack1132 < 0x1b5;
                uStack1132                 = uStack1132 + 0x1)
            {
                UVar3 = IsDlgButtonChecked(param_5, uStack1132);
                if(UVar3 == 0x1)
                {
                    pass1_1008_d818(*(ulong *)(iVar7 + 0x8e), uStack1132);
                    uVar13 = CONCAT22(param_4, HStack8);
                    goto LAB_1038_bba2;
                }
                param_5 = (HWND16)s_tile2_bmp_1050_1538;
            }
        }
        else
        {
            if(param_3 != 0xfbe)
                goto LAB_1038_bbbf;
            puStack14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, (ushort)param_6, (uchar *)param_4, unaff_DI);
            puStack16 = PTR_LOOP_1050_13ae;
            if(PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1))
            {
                puStack16 = (undefined *)&PTR_LOOP_1050_0002;
            }
            iStack10 = *(int *)((int)puStack16 * 0xc + 0x5b84) + -0x1;
            pass1_1008_612e(0x0, iStack10, iStack10);
            uStack20
              = pass1_1018_1e78(*(ulong *)(iVar7 + 0x92), *(int *)(((int)puStack16 * 0x6 + iStack10) * 0x2 + 0x5b86));
            load_string_1010_84e0(0x1010,
                                  (ushort)_PTR_LOOP_1050_14cc,
                                  (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                                  0x50,
                                  local_464,
                                  (short)param_6);
            pcVar11
              = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            puVar6 = (uchar *)((ulong)pcVar11 >> 0x10);
            uVar5  = wsprintf16((LPSTR)0x1010, local_414, &param_6->style);
            uVar9  = 0x1000;
            mem_op_1000_179c(0xb4, puVar6, 0x1000);
            if(((uint)puVar6 | uVar5) == 0x0)
            {
                paVar12 = (astruct_57 *)0x0;
            }
            else
            {
                pCVar14 = local_414;
                pWVar15 = param_6;
                puVar16 = PTR_LOOP_1050_0396;
                pcVar11 = load_string_1010_847e(
                  (int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
                uVar9   = SUB42(&PTR_LOOP_1050_1040, 0x0);
                paVar12 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar6, puVar16),
                                          0x41,
                                          pcVar11,
                                          (char *)CONCAT22(pWVar15, pCVar14),
                                          (ushort)puVar16,
                                          (ushort)((ulong)pcVar11 >> 0x10));
            }
            ppcVar2 = (code **)((int)*(undefined4 *)paVar12 + 0x74);
            uVar13  = (**ppcVar2)(uVar9, (int)paVar12);
            if((int)uVar13 != 0x1)
                goto LAB_1038_bc26;
            pass1_1008_d818(*(ulong *)(iVar7 + 0x8e), *(int *)((int)uStack20 + 0x1a));
            HStack8 = (HWND16)uVar13;
        LAB_1038_bba2:
            param_5 = 0x1008;
            win_ui_cursor_op_1038_bc30((ulong)param_1, 0x1008, (ushort)param_6);
            HStack8 = (HWND16)uVar13;
        }
        PostMessage16(param_5, 0x0, 0x0, 0x11100ce);
        HStack8 = (HWND16)uVar13;
        param_3 = 0x1;
    LAB_1038_bbbf:
        BStack6 = post_win_msg_1040_7b3c(
          param_1, (ushort)param_2, (ushort)(param_2 >> 0x10), param_3, (int)&PTR_LOOP_1050_1040);
        uStack4 = (undefined2)((ulong)uVar13 >> 0x10);
        HStack8 = (HWND16)uVar13;
    }
LAB_1038_bc26:
    return CONCAT22(uStack4, BStack6);
}


void __stdcall16far win_ui_cursor_op_1038_bc30(ulong param_1, HINSTANCE16 param_2, ushort param_3)

{
    undefined4 uVar1;
    uchar      in_AF;
    undefined2 local_112;
    undefined2 uStack272;
    HCURSOR16  HStack6;
    HCURSOR16  HStack4;

    HStack4 = LoadCursor16(param_2, (LPCSTR)0x7f02);
    HStack6 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    uVar1   = *(undefined4 *)((int)param_1 + 0x8e);
    pass1_1030_532e(
      (astruct_100 *)CONCAT22(param_3, &local_112), (long)*(int *)((int)uVar1 + 0xe) + 0x1000000, param_3, in_AF);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_3, &local_112));
    pass1_1030_838e((ulong *)_PTR_LOOP_1050_5748, param_3, in_AF);
    local_112 = 0x389a;
    uStack272 = 0x1008;
    pass1_1030_8334(_PTR_LOOP_1050_5748);
    SetCursor16(0x1030);
    return;
}


void __stdcall16far pass1_1038_be4a(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xc436;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far pass1_1038_be76(ushort param_1, ulong param_2, uchar *param_3, int param_4, ushort param_5)

{
    ushort *puVar1;
    int     iVar2;

    if(param_2._2_2_ == 0x0)
    {
        iVar2  = 0x0;
        puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
        pass1_1010_038e((ulong)puVar1, iVar2, param_5);
    }
    destroy_win_1040_7b98(CONCAT22((int)param_2, param_1), (int)&PTR_LOOP_1050_1040);
    return;
}
