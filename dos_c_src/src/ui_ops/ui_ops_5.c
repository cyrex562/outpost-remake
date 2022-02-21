void __stdcall16far unk_win_op_1020_65cc(astruct_60 *param_1, int param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4  uVar2;
    BOOL16      BVar3;
    ushort      uVar4;
    astruct_59 *iVar4;
    astruct_60 *iVar5;
    int         iVar6;
    astruct_60 *uVar7;
    HWND16      hwnd;
    int         iStack4;

    iVar5 = (astruct_60 *)param_1;
    uVar7 = (astruct_60 *)((ulong)param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        iVar5->field_0x14 = 0x0;
        return;
    }
    if(param_2 == 0x2)
    {
        for(iStack4 = 0x0; iStack4 < 0x5; iStack4 = iStack4 + 0x1)
        {
            iVar4 = (astruct_59 *)&iVar5->field_0x18;
            iVar6 = iStack4 * 0x4;
            if((*(uint *)(iVar4 + iVar6 + 0x2) | *(uint *)(iVar4 + iVar6)) != 0x0)
            {
                ppcVar1 = (code **)((int)**(undefined4 **)(iVar4 + iVar6) + 0x4);
                (**ppcVar1)(param_3, *(undefined4 *)(iVar4 + iVar6));
            }
        }
    }
    else
    {
        if(((0x0 < param_2 + -0x3) && (!SBORROW2(param_2 + -0x3, 0x1))) && (param_2 + -0x4 < 0x4))
        {
            BVar3 = IsIconic16(param_3);
            if(BVar3 == 0x0)
            {
                BVar3 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
                if((BVar3 == 0x0) && (uVar2 = iVar5->field_0x14, *(int *)((int)uVar2 + 0x24) != 0x0))
                {
                    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)0x0, 0x0);
                    uVar4 = pass1_1020_64d4((ulong)param_1, 0x2);
                    if(uVar4 == 0x0)
                    {
                        pass1_1020_6746((ulong)param_1, 0x1, 0x2);
                    }
                    uVar4 = pass1_1020_64d4((ulong)param_1, 0x3);
                    if(uVar4 == 0x0)
                    {
                        pass1_1020_6746((ulong)param_1, 0x1, 0x3);
                    }
                    hwnd  = 0x1018;
                    uVar4 = pass1_1018_255e(iVar5->field_0x14);
                    if(uVar4 == 0x0)
                    {
                        hwnd = (HWND16)s_tile2_bmp_1050_1538;
                        SendMessage16(0x1018, 0x0, 0x0, 0x1110069);
                    }
                    else
                    {
                        uVar4 = pass1_1020_64d4((ulong)param_1, 0x1);
                        if(uVar4 == 0x0)
                        {
                            pass1_1020_6746((ulong)param_1, 0x1, 0x1);
                        }
                    }
                    SendMessage16(hwnd, 0x0, 0x0, 0x11100f0);
                    uVar2 = iVar5->field_0x2c;
                    if(*(int *)((int)uVar2 + 0x7a) != 0x0)
                    {
                        uVar2                              = iVar5->field_0x2c;
                        *(undefined2 *)((int)uVar2 + 0x7a) = 0x0;
                        SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110131);
                        return;
                    }
                }
            }
        }
    }
    return;
}
void __stdcall16far unk_win_ui_op_1020_67ce(astruct_20 *in_struct_1, UINT16 param_2, ULONG param_3)

{
    HGDIOBJ16   HVar1;
    HCURSOR16   HVar2;
    astruct_20 *iVar3;
    astruct_20 *uVar4;
    UINT16      unaff_SS;

    struct_1020_790e(&in_struct_1->field_0x0, (ulong)s_TPPOPMENU_1050_43fa, param_2, param_3, unaff_SS);
    uVar4                                 = (astruct_20 *)((ulong)in_struct_1 >> 0x10);
    iVar3                                 = (astruct_20 *)in_struct_1;
    iVar3[0x1].field_0x10                 = 0x0;
    *(undefined4 *)&iVar3[0x1].field_0x14 = 0x0;
    in_struct_1->field_0x0                = 0x70e6;
    iVar3->field_0x2                      = 0x1020;
    unk_str_op_1000_3d3e((char *)((ulong)in_struct_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x5b)), s_VrMode2_1050_4404);
    HVar1                     = GetStockObject16(0x1000);
    iVar3->hgdiobj_field_0xc6 = HVar1;
    HVar2                     = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538, (LPCSTR)0x7f00);
    iVar3->hcursor_field_0xc4 = HVar2;
    iVar3->field_0xac         = 0x44c00000;
    iVar3->field_0xc8         = 0x2020;
    iVar3->field_0xbc         = *(UINT16 *)((int)param_3 + 0x8);
    iVar3->field_0xca         = param_2;
    win_ui_reg_class_1008_96d2(in_struct_1, 0x1008, unaff_SS);
    window_op_1020_6c3a(in_struct_1);
    return;
}


void __stdcall16far pass1_1020_687c(ulong param_1, ushort param_2, ushort param_3)

{
    uchar uVar1;

    uVar1 = (uchar)param_2;
    get_win_ui_info_op_1020_7a50(param_1, param_3);
    destroy_icon_1020_6bd2(param_1, uVar1, param_3);
    return;
}
uint __stdcall16far unk_destroy_win_op_1020_694c(ULONG param_1, uint param_2, HWND16 param_3, ushort param_4)

{
    undefined4  uVar1;
    uint        uVar2;
    HWND16      HVar3;
    int         iVar4;
    astruct_43 *paVar5;
    undefined2  uVar6;

    uVar2 = param_2;
    if(param_2 != 0x12b)
    {
        iVar4 = (int)param_1;
        uVar6 = (undefined2)(param_1 >> 0x10);
        if(param_2 < 0x12c)
        {
            if(param_2 == 0x6f)
            {
                paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1f8, param_4);
                uVar2  = WinHelp16(0x1010, (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x9), 0x0, CONCAT22((int)paVar5, 0x1));
                return uVar2;
            }
            if(param_2 == 0xeb)
            {
                uVar2 = GetDlgItem16(param_3, 0x1797);
                if(uVar2 != 0x0)
                {
                LAB_1020_6a6f:
                    win_ui_fn_1020_6e98(param_1, (HWND16)s_tile2_bmp_1050_1538, param_4);
                    return uVar2;
                }
            }
            else
            {
                uVar2 = param_2 - 0xef;
                if(uVar2 == 0x0)
                {
                    pass1_1018_2e28(*(ulong *)(iVar4 + 0xf2));
                    pass1_1008_3e0e(param_1);
                }
                else
                {
                    uVar2 = param_2 - 0x129;
                    if((uVar2 != 0x0) && (uVar2 = param_2 - 0x12a, uVar2 == 0x0))
                    {
                        uVar6 = 0xf012;
                    LAB_1020_69c3:
                        uVar2 = PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x112, uVar6));
                        return uVar2;
                    }
                }
            }
        }
        else
        {
            if(param_2 == 0xbb8)
            {
                HVar3 = GetDlgItem16(param_3, 0x1797);
                if(HVar3 != 0x0)
                {
                    DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
                }
                uVar2 = pass1_1018_31d0(*(ulong *)(iVar4 + 0xf2));
                if(uVar2 != 0x0)
                {
                    uVar1 = *(undefined4 *)(iVar4 + 0xf2);
                    uVar2 = pass1_1018_2d9a((int)uVar1, (int)((ulong)uVar1 >> 0x10));
                LAB_1020_6a0b:
                    invalidate_rect_1020_735a(*(ulong *)(iVar4 + 0xf6), 0x1018);
                    return uVar2;
                }
            }
            else
            {
                if(param_2 < 0xbb9)
                {
                    if(param_2 == 0x12c)
                    {
                        uVar6 = 0xf020;
                        goto LAB_1020_69c3;
                    }
                    uVar2 = param_2 - 0x12d;
                    if(param_2 != 0x12c)
                    {
                        uVar2 = param_2 - 0x12e;
                    }
                }
                else
                {
                    if(param_2 == 0xbb9)
                    {
                        HVar3 = GetDlgItem16(param_3, 0x1797);
                        if(HVar3 != 0x0)
                        {
                            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
                        }
                        uVar2 = pass1_1018_31d0(*(ulong *)(iVar4 + 0xf2));
                        if(uVar2 != 0x0)
                        {
                            uVar1 = *(undefined4 *)(iVar4 + 0xf2);
                            uVar2 = pass1_1018_2dde((int)uVar1, (int)((ulong)uVar1 >> 0x10));
                            goto LAB_1020_6a0b;
                        }
                    }
                    else
                    {
                        uVar2 = param_2 - 0xbba;
                        if(uVar2 == 0x0)
                        {
                            uVar2 = GetDlgItem16(param_3, 0x1797);
                            if(uVar2 != 0x0)
                            {
                                uVar2 = DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
                                return uVar2;
                            }
                            goto LAB_1020_6a6f;
                        }
                    }
                }
            }
        }
    }
    return uVar2;
}

void __stdcall16far win_ui_op_1020_6ae6(ulong *param_1, ushort param_2, int param_3, int param_4, HWND16 param_5, WPARAM16 param_6)

{
    code     **ppcVar1;
    undefined *puVar2;
    int        iVar3;
    undefined2 uVar4;
    HWND16     hwnd;
    LRESULT    LVar5;
    uint16_t   in_stack_0000ff86;
    uint16_t   in_stack_0000ff88;
    HWND16     HVar6;
    undefined  local_58[0x50];
    undefined4 uStack8;
    HWND16     HStack4;

    if(param_4 == 0x1797)
    {
        uVar4   = (undefined2)((ulong)param_1 >> 0x10);
        iVar3   = (int)param_1;
        hwnd    = (HWND16)s_tile2_bmp_1050_1538;
        HStack4 = GetDlgItem16(param_5, 0x1797);
        if(HStack4 != 0x0)
        {
            if(param_3 == 0x2)
            {
                hwnd    = (HWND16)s_tile2_bmp_1050_1538;
                uStack8 = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4090000);
                if(uStack8 != -0x1)
                {
                    HVar6  = HStack4;
                    LVar5  = SendMessage16((HWND16)s_tile2_bmp_1050_1538, (UINT16)local_58, param_6, CONCAT22(0x40a, (int)uStack8));
                    puVar2 = local_58;
                    pass1_1018_30ca(*(ulong *)(iVar3 + 0xf2), (char *)CONCAT22(param_6, puVar2), (ushort)((ulong)LVar5 >> 0x10));
                    hwnd = 0x1018;
                    pass1_1018_2fe8(*(ulong *)(iVar3 + 0xf2), in_stack_0000ff86, in_stack_0000ff88);
                    if(puVar2 != (undefined *)0x0)
                    {
                        invalidate_rect_1020_735a(*(ulong *)(iVar3 + 0xf6), 0x1018);
                        ppcVar1 = (code **)((int)*param_1 + 0x40);
                        (**ppcVar1)(0x1018, param_1, 0xef, HVar6);
                    }
                }
            }
            else
            {
                if(param_3 != 0x3)
                {
                    return;
                }
            }
            DestroyWindow16(hwnd);
        }
    }
    return;
}


void __stdcall16far enable_menu_item_1020_6b9a(HMENU16 param_1, int param_2)

{
    if(param_2 != 0x0)
    {
        return;
    }
    EnableMenuItem16(param_1, 0x400, 0x0);
    return;
}


void __stdcall16far pass1_1020_6bbc(ulong param_1, ushort param_2, ushort param_3)

{
    win_ui_op_1020_737a(*(ULONG *)((int)param_1 + 0xf6), param_2, param_3);
    return;
}

void __stdcall16far win_ui_fn_1020_6e98(undefined4 param_1, HWND16 in_win_handle, UINT16 param_3)

{
    int        *piVar1;
    astruct_18 *paVar2;
    HWND16      window_handle;
    ushort      uVar3;
    undefined2  in_DX;
    WPARAM16    WVar4;
    int         iVar5;
    undefined2  uVar6;
    LRESULT     LVar7;
    char       *pcVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    int         iStack36;
    undefined4  window_name;
    RECT16      rectangle;
    INT16       IStack6;
    int         iStack4;

    uVar6 = (undefined2)((ulong)param_1 >> 0x10);
    iVar5 = (int)param_1;
    GetClientRect16(in_win_handle, &rectangle);
    window_name   = (astruct_18 *)0x0;
    window_handle = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1797);
    if(window_handle != 0x0)
    {
        DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
    }
    pass1_1018_30fc(*(ulong *)(iVar5 + 0xf2), (ushort **)CONCAT22(param_3, &window_name), in_DX);
    if((window_name._2_2_ | (uint)(LPCSTR)window_name) != 0x0)
    {
        window_handle = CreateWindow16((LPCSTR)0x1018, (LPCSTR)window_name, CONCAT22(PTR_LOOP_1050_038c, window_name._2_2_), 0x1797, *(INT16 *)(iVar5 + 0x8), iStack4 + -0x19, IStack6, 0x0, 0x0, 0x103, (LPVOID)0x40a0);
        paVar2        = window_name;
        if(window_handle == 0x0)
        {
            if((window_name._2_2_ | (uint)window_name) != 0x0)
            {
                pass1_1018_2afa((ulong *)window_name);
                fn_ptr_1000_17ce(paVar2, 0x1000);
                return;
            }
        }
        else
        {
            LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0xb0000);
            WVar4 = (WPARAM16)((ulong)LVar7 >> 0x10);
            if(*(int *)((int)window_name + 0x4) == 0x0)
            {
                uVar9  = 0x0;
                uVar10 = 0x401;
                pcVar8 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
                SendMessage16(0x1010, (UINT16)pcVar8, (WPARAM16)((ulong)pcVar8 >> 0x10), CONCAT22(uVar10, uVar9));
            }
            else
            {
                iStack36 = 0x0;
                while(true)
                {
                    piVar1 = (int *)((int)window_name + 0x4);
                    if(*piVar1 == iStack36 || *piVar1 < iStack36)
                        break;
                    uVar9    = 0x0;
                    uVar10   = 0x401;
                    uVar3    = pass1_1020_bd80(*(ushort *)((int)*(undefined4 *)window_name + iStack36 * 0x2));
                    LVar7    = SendMessage16((HWND16)s_tile2_bmp_1050_1538, uVar3, WVar4, CONCAT22(uVar10, uVar9));
                    WVar4    = (WPARAM16)((ulong)LVar7 >> 0x10);
                    iStack36 = iStack36 + 0x1;
                }
            }
            LVar7  = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0xb0001);
            WVar4  = (WPARAM16)((ulong)LVar7 >> 0x10);
            uVar3  = (ushort)LVar7;
            uVar9  = 0xffff;
            uVar10 = 0x40d;
            pass1_1018_2d84(uVar3, *(ulong *)(iVar5 + 0xf2));
            LVar7 = SendMessage16(0x1018, uVar3, WVar4, CONCAT22(uVar10, uVar9));
            iVar5 = (int)LVar7;
            if((iVar5 != -0x1) || ((int)((ulong)LVar7 >> 0x10) != -0x1))
            {
                SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, CONCAT22(0x407, iVar5));
                SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, CONCAT22(0x418, iVar5));
            }
            paVar2        = window_name;
            window_handle = (HWND16)s_tile2_bmp_1050_1538;
            if(window_name != (astruct_18 *)0x0)
            {
                pass1_1018_2afa((ulong *)window_name);
                window_handle = 0x1000;
                fn_ptr_1000_17ce(paVar2, 0x1000);
            }
            ShowWindow16(window_handle, 0x1);
            SetFocus16((HWND16)s_tile2_bmp_1050_1538);
        }
    }
    return;
}

astruct_3 *__stdcall16far pass1_1020_70c0(astruct_3 *param_1, byte param_2, ushort param_3)

{
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1020_717e(UINT16 *param_1, ULONG param_2, UINT16 param_3)

{
    astruct_13 *paVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    HPALETTE16  HVar4;
    undefined4 *puVar5;
    uchar      *in_DX;
    undefined2  uVar6;
    uchar      *extraout_DX;
    uchar      *puVar7;
    int         iVar8;
    int         iVar10;
    int         unaff_DI;
    undefined2  uVar11;
    undefined2  uVar12;
    undefined2  unaff_CS;
    ushort     *puVar13;
    HDC16       local_4;
    astruct_41 *iVar9;

    get_sys_metrics_1020_7c1a(param_1, param_2, unaff_CS);
    uVar11                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar8                         = (int)param_1;
    *(undefined4 *)(iVar8 + 0x14) = 0x0;
    *(ULONG *)(iVar8 + 0x18)      = param_2;
    *(undefined4 *)(iVar8 + 0x1c) = 0x0;
    *(undefined2 *)(iVar8 + 0x20) = 0x0;
    *param_1                      = 0x754c;
    *(undefined2 *)(iVar8 + 0x2)  = 0x1020;
    puVar13                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x4, param_3, in_DX, unaff_DI);
    uVar6                         = (undefined2)((ulong)puVar13 >> 0x10);
    *(undefined2 *)(iVar8 + 0x1c) = (int)puVar13;
    *(undefined2 *)(iVar8 + 0x1e) = uVar6;
    ppcVar2                       = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x1c) + 0x4);
    (**ppcVar2)(0x1010, *(undefined2 *)(iVar8 + 0x1c), uVar6, 0x0, param_1);
    uVar6                          = *(undefined2 *)(iVar8 + 0x4);
    local_4                        = GetDC16(0x1010);
    uVar3                          = *(undefined4 *)(iVar8 + 0x1c);
    *(HDC16 *)((int)uVar3 + 0x178) = local_4;
    uVar3                          = *(undefined4 *)(iVar8 + 0x1c);
    uVar12                         = (undefined2)((ulong)uVar3 >> 0x10);
    iVar10                         = (int)uVar3;
    puVar5                         = (undefined4 *)*(undefined4 *)(iVar10 + 0x24);
    ppcVar2                        = (code **)((int)*puVar5 + 0x14);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)puVar5, *(undefined2 *)(iVar10 + 0x26), uVar6);
    puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_3, extraout_DX, unaff_DI);
    puVar7  = (uchar *)((ulong)puVar13 >> 0x10);
    paVar1  = *(astruct_13 **)((int)puVar13 + 0xe);
    pass1_1008_4d84((astruct_90 *)((ulong)puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10), (ulong)paVar1, puVar7);
    HVar4                         = palette_op_1008_4e08(paVar1, &local_4, (ushort)puVar7, 0x1008);
    *(HPALETTE16 *)(iVar8 + 0x20) = HVar4;
    return;
}

void __stdcall16far pass1_1020_51c6(ulong param_1, ushort param_2, ulong param_3, ushort param_4, ushort param_5)

{
    code **ppcVar1;
    int    iVar2;
    int    iVar3;
    uint   uVar4;
    ushort uVar5;

    uVar4 = (uint)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    iVar2 = *(int *)(iVar3 + 0xf4);
    uVar5 = (ushort)param_3;
    if(iVar2 == 0x2)
    {
        win_ui_op_1020_5e76(param_1 & 0xffff | (ulong)uVar4 << 0x10, param_2, uVar5);
        return;
    }
    iVar2 = iVar2 + -0x3;
    if(iVar2 != 0x0)
    {
        pt_in_rect_op_1020_58ce(param_1 & 0xffff | (ulong)uVar4 << 0x10, param_2, uVar5, (byte)(param_3 >> 0x10), param_4, param_5);
        if(iVar2 == 0x0)
        {
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0x5c);
            (**ppcVar1)(param_4, *(undefined4 *)(iVar3 + 0x4), param_2, param_3);
        }
        return;
    }
    win_ui_op_1020_5de8(param_1 & 0xffff | (ulong)uVar4 << 0x10, param_2, uVar5, param_5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_cursor_op_1020_522e(astruct_52 *param_1, ushort param_2, ushort param_3)

{
    int        iVar1;
    code     **ppcVar2;
    BOOL16     BVar3;
    uchar     *in_DX;
    int        iVar4;
    int        unaff_DI;
    undefined2 uVar5;
    HCURSOR16  unaff_CS;
    ushort     unaff_SS;
    ushort    *puVar6;
    undefined  uVar7;
    undefined  uVar8;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    iVar1 = *(int *)(iVar4 + 0xf4);
    if(iVar1 == 0x2)
    {
        SetCursor16(unaff_CS);
        *(undefined2 *)(iVar4 + 0xee)  = 0x0;
        *(undefined2 *)(iVar4 + 0xf4)  = 0x1;
        *(undefined2 *)(iVar4 + 0x10c) = 0x0;
        ReleaseCapture16();
        return;
    }
    if(iVar1 == 0x3)
    {
        SetCursor16(unaff_CS);
        *(undefined2 *)(iVar4 + 0xee)  = 0x0;
        *(undefined2 *)(iVar4 + 0xf4)  = 0x1;
        *(undefined2 *)(iVar4 + 0x10c) = 0x0;
        ReleaseCapture16();
        uVar7  = 0x0;
        uVar8  = 0x0;
        uVar5  = 0x0;
        puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x47, unaff_SS, in_DX, unaff_DI);
        pass1_1018_57e6((ulong)puVar6, CONCAT22(uVar5, CONCAT11(uVar8, uVar7)), unaff_SS);
        return;
    }
    BVar3 = menu_ui_op_1020_5bf2(param_1, param_2, param_3);
    if(BVar3 == 0x0)
    {
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x4) + 0x60);
        (**ppcVar2)();
    }
    return;
}


void __stdcall16far pass1_1020_52de(ULONG param_1, ushort param_2)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;
    int         iVar6;
    undefined2  uVar7;

    uVar7  = (undefined2)(param_1 >> 0x10);
    iVar6  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar6 + 0xf6);
    uVar2  = *(uint *)(iVar6 + 0xf8);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *(undefined4 *)(iVar6 + 0xf6) = 0x0;
    if(*(long *)(iVar6 + 0xfa) != 0x0)
    {
        if(param_1 == 0x0)
        {
            iVar4 = 0x0;
            uVar5 = 0x0;
        }
        else
        {
            iVar4 = iVar6 + 0xe2;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6(*(ulong *)(iVar6 + 0xfa), CONCAT22(uVar5, iVar4), param_2);
    }
    destroy_win_1008_628e(param_1, 0x1008);
    if(*(long *)(iVar6 + 0xfa) != 0x0)
    {
        pass1_1010_1dda(*(ulong *)(iVar6 + 0xfa));
    }
    *(undefined4 *)(iVar6 + 0xfa) = 0x0;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far ui_op_1020_536e(ulong param_1, undefined4 param_2, int param_3, int param_4, uchar *param_5)

{
    int        *piVar1;
    UINT16      UVar2;
    code      **ppcVar3;
    ushort      uVar4;
    ushort      uVar5;
    UINT16      UVar6;
    uint        uVar7;
    uchar      *puVar8;
    uchar      *extraout_DX;
    uchar      *puVar9;
    int         iVar10;
    undefined4 *puVar11;
    int         unaff_DI;
    undefined2  uVar12;
    ushort      unaff_SS;
    ushort     *puVar13;
    undefined4  uVar14;
    ulong       uVar15;
    undefined   uVar16;
    undefined   uVar17;
    undefined2  uVar18;
    undefined2  uVar19;
    undefined4 *puStack16;

    uVar7  = param_4 - 0x1;
    uVar12 = (undefined2)(param_1 >> 0x10);
    iVar10 = (int)param_1;
    if(uVar7 == 0x0)
    {
        if(*(long *)(iVar10 + 0xfe) == 0x0)
        {
            mem_op_1000_179c(0xfc, param_5, 0x1000);
            uVar14 = CONCAT22((uint)param_5 | uVar7, uVar7);
            if(((uint)param_5 | uVar7) == 0x0)
            {
                *(undefined4 *)(iVar10 + 0xfe) = 0x0;
            }
            else
            {
                piVar1                          = (int *)(iVar10 + 0xcc);
                *piVar1                         = *piVar1 + 0x1;
                uVar14                          = unk_win_ui_op_1020_67ce(CONCAT13((char)((uint)param_5 >> 0x8), CONCAT12((char)param_5, uVar7)), *(UINT16 *)(iVar10 + 0xcc), param_1);
                *(undefined2 *)(iVar10 + 0xfe)  = (int)uVar14;
                *(undefined2 *)(iVar10 + 0x100) = (int)((ulong)uVar14 >> 0x10);
            }
            pass1_1008_6978(param_1, 0x0, *(ulong *)(iVar10 + 0xfe), (uint)uVar14, (uchar *)((ulong)uVar14 >> 0x10));
            uVar14  = *(undefined4 *)(iVar10 + 0xfe);
            uVar18  = (undefined2)uVar14;
            uVar19  = (undefined2)((ulong)uVar14 >> 0x10);
            uVar14  = *(undefined4 *)(iVar10 + 0xfe);
            uVar12  = (undefined2)((ulong)uVar14 >> 0x10);
            puVar11 = (undefined4 *)uVar14;
            goto LAB_1020_53f3;
        }
    }
    else
    {
        if(param_4 == 0x2)
        {
            uVar4   = param_3 + 0xc;
            puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, uVar4, unaff_SS, param_5, unaff_DI);
            puVar9  = (uchar *)((ulong)puVar13 >> 0x10);
            uVar5   = pass1_1018_0afa((ulong)puVar13);
            if(uVar5 == 0x0)
            {
                piVar1  = (int *)(iVar10 + 0xcc);
                *piVar1 = *piVar1 + 0x1;
                UVar2   = *(UINT16 *)(iVar10 + 0xcc);
                uVar12  = 0xfe;
                UVar6   = UVar2;
                mem_op_1000_179c(0xfe, puVar9, 0x1000);
                puVar8 = (uchar *)((uint)puVar9 | UVar6);
                if(puVar8 == (uchar *)0x0)
                {
                    UVar6  = 0x0;
                    puVar8 = (uchar *)0x0;
                }
                else
                {
                    pass1_1020_289a((ushort *)CONCAT13((char)((uint)puVar9 >> 0x8), CONCAT12((char)puVar9, UVar6)), UVar2, param_1, unaff_SS);
                }
                puStack16 = (undefined4 *)CONCAT22(puVar8, UVar6);
                uVar16    = SUB21(puVar8, 0x0);
                uVar17    = (undefined)((uint)puVar8 >> 0x8);
                pass1_1020_294a(CONCAT13(uVar17, CONCAT12(uVar16, UVar6)), CONCAT22((int)param_2, uVar12), (ushort)((ulong)param_2 >> 0x10), puVar8, unaff_DI, unaff_SS);
                unaff_DI = (int)((ulong)*puStack16 >> 0x10);
                iVar10   = (int)*puStack16;
                ppcVar3  = (code **)(iVar10 + 0x8);
                uVar14   = (**ppcVar3)(0x1000, UVar6, puVar8, uVar4);
                pass1_1008_3e0e(CONCAT13(uVar17, CONCAT12(uVar16, UVar6)));
                pass1_1008_6978(param_1, UVar2, CONCAT22(puVar8, UVar6), (uint)uVar14, (uchar *)((ulong)uVar14 >> 0x10));
                ppcVar3 = (code **)(iVar10 + 0xc);
                (**ppcVar3)(0x1008, (char)UVar6, uVar16, 0x1);
                puVar9 = extraout_DX;
            }
            else
            {
                uVar15 = pass1_1018_0ad4((ulong)puVar13);
                puVar9 = (uchar *)(uVar15 >> 0x10);
                pass1_1008_3e0e(uVar15);
            }
            pass1_1018_1662((ulong)puVar13, 0x0, 0x0, unaff_SS);
            BringWindowToTop16(0x1018);
            uVar4   = 0x1;
            iVar10  = 0x4;
            puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, unaff_SS, puVar9, unaff_DI);
            pass1_1010_089e(unaff_SS, (ulong)puVar13, uVar4, iVar10);
            pass1_1010_089e(unaff_SS, (ulong)puVar13, 0x1, 0x3);
            return;
        }
        uVar7 = param_4 - 0x3;
        if((uVar7 == 0x0) && (*(long *)(iVar10 + 0x102) == 0x0))
        {
            mem_op_1000_179c(0xfc, param_5, 0x1000);
            puVar9 = (uchar *)((uint)param_5 | uVar7);
            if(puVar9 == (uchar *)0x0)
            {
                *(undefined4 *)(iVar10 + 0x102) = 0x0;
            }
            else
            {
                piVar1  = (int *)(iVar10 + 0xcc);
                *piVar1 = *piVar1 + 0x1;
                pass1_1020_0dc4((ushort *)CONCAT13((char)((uint)param_5 >> 0x8), CONCAT12((char)param_5, uVar7)), *(UINT16 *)(iVar10 + 0xcc), param_1, unaff_SS);
                *(uint *)(iVar10 + 0x102)   = uVar7;
                *(uchar **)(iVar10 + 0x104) = puVar9;
            }
            pass1_1008_6978(param_1, 0x0, *(ulong *)(iVar10 + 0x102), uVar7, puVar9);
            uVar14  = *(undefined4 *)(iVar10 + 0x102);
            uVar18  = (undefined2)uVar14;
            uVar19  = (undefined2)((ulong)uVar14 >> 0x10);
            uVar14  = *(undefined4 *)(iVar10 + 0x102);
            uVar12  = (undefined2)((ulong)uVar14 >> 0x10);
            puVar11 = (undefined4 *)uVar14;
        LAB_1020_53f3:
            ppcVar3 = (code **)((int)*puVar11 + 0xc);
            (**ppcVar3)(0x8, uVar18, uVar19, 0x5);
            return;
        }
    }
    return;
}

void __stdcall16far set_cursor_1020_5764(ulong param_1, int param_2, ushort param_3)

{
    uint        uVar1;
    undefined4  uVar2;
    uchar      *in_DX;
    int         iVar3;
    int         unaff_DI;
    undefined2  uVar4;
    HINSTANCE16 h_instance;
    HCURSOR16   hcursor;
    int         local_e;
    undefined   local_c[0x2];
    ulong       uStack10;
    ushort     *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_3, in_DX, unaff_DI);
    uVar4    = (undefined2)((ulong)puStack6 >> 0x10);
    uStack10 = *(ulong *)((int)puStack6 + 0x20);
    uVar1    = *(uint *)((int)puStack6 + 0x22);
    if((uVar1 | (uint)uStack10) != 0x0)
    {
        h_instance = 0x1030;
        pass1_1030_8308((ushort)_PTR_LOOP_1050_5748,
                        (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                        (ushort *)CONCAT22(param_3, &local_e),
                        (ushort *)CONCAT13((char)(param_3 >> 0x8), CONCAT12((char)param_3, local_c)),
                        uStack10 & 0xffff | (ulong)uVar1 << 0x10,
                        (ushort)&local_e,
                        uVar1);
        if(param_2 <= local_e)
        {
            uVar4 = (undefined2)(param_1 >> 0x10);
            iVar3 = (int)param_1;
            if(*(int *)(iVar3 + 0xf4) != 0x1)
            {
                SetCursor16(0x1030);
                *(undefined2 *)(iVar3 + 0xee)  = 0x0;
                *(undefined2 *)(iVar3 + 0xf4)  = 0x1;
                *(undefined2 *)(iVar3 + 0x10c) = 0x0;
                h_instance                     = (HINSTANCE16)s_tile2_bmp_1050_1538;
                ReleaseCapture16();
            }
            LoadCursor16(h_instance, (LPCSTR)0x7f02);
            SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
            hcursor = 0x1018;
            pass1_1018_017c((ulong)puStack6, param_2, param_3);
            uVar2                              = *(undefined4 *)(iVar3 + 0xf6);
            *(undefined2 *)((int)uVar2 + 0x10) = 0x1;
            if(*(long *)(iVar3 + 0xfe) != 0x0)
            {
                pass1_1020_68de(*(ulong *)(iVar3 + 0xfe), 0x1018);
                hcursor = (HCURSOR16)s_tile2_bmp_1050_1538;
                PostMessage16(0x1018, 0x0, 0x0, 0x11100eb);
            }
            SetCursor16(hcursor);
        }
    }
    return;
}

void __stdcall16far pt_in_rect_1020_5856(ulong param_1, POINT16 *param_2, uint param_3)

{
    ulong *puVar1;
    BOOL16 BVar2;
    ulong  uVar3;
    uint   in_DX;
    uint   extraout_DX;
    ulong  uStack10;

    pass1_1018_2862(*(ulong *)((int)param_1 + 0xfa));
    if((in_DX | param_3) != 0x0)
    {
        uStack10 = 0x0;
        while(true)
        {
            puVar1 = (ulong *)(param_3 + 0xa);
            if(*puVar1 < uStack10 || *puVar1 == uStack10)
                break;
            uVar3 = uStack10;
            empty_1008_8fc4(param_3, in_DX, (int)uStack10, (int)(uStack10 >> 0x10));
            if((extraout_DX | (uint)uVar3) != 0x0)
            {
                BVar2 = PtInRect16((RECT16 *)0x1008, *param_2);
                if(BVar2 != 0x0)
                {
                    return;
                }
            }
            uStack10 = uStack10 + 0x1;
        }
    }
    return;
}

void __stdcall16far pt_in_rect_op_1020_58ce(ulong param_1, ushort param_2, ushort param_3, byte param_4, RECT16 *param_5, ushort param_6)

{
    code     **ppcVar1;
    undefined4 uVar2;
    ushort     uVar3;
    BOOL16     BVar4;
    ushort    *msg;
    uchar     *in_DX;
    uint       uVar5;
    uchar     *puVar6;
    int        iVar7;
    int        iVar8;
    int        unaff_DI;
    undefined2 uVar9;
    undefined2 uVar10;
    RECT16    *rect;
    RECT16    *rect_00;
    ulong      uVar11;
    ushort    *puVar12;
    undefined  local_34[0x6];
    ulong      uStack46;
    ushort    *puStack38;
    undefined4 uStack30;
    ushort    *puStack26;
    ushort     local_18[0x2];
    ushort     uStack20;
    undefined4 uStack18;
    undefined2 uStack14;
    uchar     *puStack12;
    uint       uStack10;
    uint       uStack8;
    ushort     local_6;
    ushort     uStack4;

    local_6        = param_3;
    uStack4        = param_2;
    uStack8        = param_4 & 0x8;
    uStack10       = param_4 & 0x4;
    uVar9          = (undefined2)(param_1 >> 0x10);
    iVar7          = (int)param_1;
    uVar3          = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6), 0x2);
    uStack30._2_2_ = in_DX;
    rect           = param_5;
    if(uVar3 == 0x0)
    {
    LAB_1020_5942:
        uVar3   = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6), 0x4);
        rect_00 = rect;
        if(uVar3 == 0x0)
        {
        LAB_1020_5a16:
            uVar3 = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6), 0x1);
            if(uVar3 != 0x0)
            {
                uStack30       = pass1_1020_6498(*(ulong *)(iVar7 + 0xf6), 0x1);
                uStack30._2_2_ = (uchar *)(uStack30 >> 0x10);
                for(puStack26 = (ushort *)0x0; (int)puStack26 < 0x4; puStack26 = (ushort *)((int)puStack26 + 0x1))
                {
                    BVar4 = PtInRect16(rect_00, (POINT16)CONCAT22(uStack4, local_6));
                    if(BVar4 != 0x0)
                    {
                        local_18[0] = 0x0;
                        uStack20    = 0x0;
                        if(puStack26 == (ushort *)0x0)
                        {
                            uStack20 = (-(uint)(uStack10 == 0x0) & 0x4) - 0x5;
                        }
                        else
                        {
                            if(puStack26 == (ushort *)((int)&PTR_LOOP_1050_0000 + 0x1))
                            {
                                uStack20 = (-(uint)(uStack10 == 0x0) & 0xfffc) + 0x5;
                            }
                            else
                            {
                                if(puStack26 == (ushort *)&PTR_LOOP_1050_0002)
                                {
                                    local_18[0] = (-(uint)(uStack10 == 0x0) & 0x4) - 0x5;
                                }
                                else
                                {
                                    if(puStack26 == (ushort *)((int)&PTR_LOOP_1050_0002 + 0x1))
                                    {
                                        local_18[0] = (-(uint)(uStack10 == 0x0) & 0xfffc) + 0x5;
                                    }
                                }
                            }
                        }
                        pass1_1020_2a94(*(ulong *)(iVar7 + 0xce), CONCAT22(local_18[0], uStack20), param_6);
                        return;
                    }
                    rect_00 = (RECT16 *)s_tile2_bmp_1050_1538;
                }
            }
            uVar3 = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6), 0x3);
            if(uVar3 != 0x0)
            {
                uStack30._0_2_ = &local_6;
                pt_in_rect_1020_5856(param_1, (POINT16 *)CONCAT22(param_6, (ushort *)uStack30), (ushort *)uStack30);
                uVar5 = (uint)uStack30._2_2_ | (uint)(ushort *)uStack30;
                if(uVar5 != 0x0)
                {
                    puStack26 = (ushort *)((ushort *)uStack30)[0x17];
                    if(((uStack8 == 0x0) || (uStack10 == 0x0)) && (uStack10 == 0x0))
                    {
                        local_18[0] = 0x1;
                    }
                    else
                    {
                        local_18[0] = 0x2;
                    }
                    uStack20 = ((ushort *)uStack30)[0x6];
                    uStack18 = (ushort *)CONCAT22(uStack18._2_2_, ((ushort *)uStack30)[0x7]);
                    if((puStack26 == (ushort *)0xb) || (puStack26 == (ushort *)0x37))
                    {
                        uVar2    = *(undefined4 *)(iVar7 + 0xfa);
                        uVar10   = (undefined2)((ulong)uVar2 >> 0x10);
                        iVar8    = (int)uVar2;
                        uStack46 = *(ulong *)(iVar8 + 0x20);
                        uVar5    = *(uint *)(iVar8 + 0x22);
                        if((uVar5 | (uint)uStack46) != 0x0)
                        {
                            puVar12 = pass1_1008_3e38((ushort *)CONCAT22(param_6, local_34));
                            puVar6  = (uchar *)((ulong)puVar12 >> 0x10);
                            pass1_1018_161c(param_6, uStack46, (ushort *)CONCAT22(param_6, local_34), (int)uStack18, uStack20);
                            puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, puVar6, unaff_DI);
                            uVar5     = (uint)((ulong)puStack38 >> 0x10);
                            pass1_1010_ecc6((ulong)puStack38, (ushort *)CONCAT22(param_6, local_34), *(long *)((int)uStack46 + 0x3c), (ushort)local_34, uVar5, param_6);
                        }
                    }
                    uVar3 = pass1_1018_25d2(*(ulong *)(iVar7 + 0xfa), local_18[0], (ulong)uStack18 & 0xffff | (ulong)uStack20 << 0x10, unaff_DI, param_6);
                    if(uVar3 != 0x0)
                    {
                        return;
                    }
                    uVar3 = pass1_1020_5d56((ulong *)param_1, CONCAT22(uStack30._2_2_, (ushort *)uStack30), uVar5, unaff_DI, param_6);
                    if(uVar3 != 0x0)
                    {
                        return;
                    }
                }
            }
            return;
        }
        uVar11         = pass1_1020_6498(*(ulong *)(iVar7 + 0xf6), 0x4);
        uStack30._2_2_ = (uchar *)(uVar11 >> 0x10);
        uVar10         = (undefined2)uVar11;
        rect_00        = (RECT16 *)s_tile2_bmp_1050_1538;
        puVar6         = uStack30._2_2_;
        uStack14       = uVar10;
        puStack12      = uStack30._2_2_;
        BVar4          = PtInRect16(rect, (POINT16)CONCAT22(uStack4, local_6));
        if(BVar4 == 0x0)
            goto LAB_1020_5a16;
        rect     = (RECT16 *)0x1010;
        uStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_6, uStack30._2_2_, unaff_DI);
        if(*(int *)((int)uStack18 + 0x72) != 0x0)
        {
            *(undefined2 *)(iVar7 + 0x116) = 0x1;
            if(param_1 == 0x0)
            {
                iVar7 = 0x0;
                uVar9 = 0x0;
            }
            else
            {
                iVar7 = iVar7 + 0xe2;
            }
            uStack30 = CONCAT22(uVar9, iVar7);
            ppcVar1  = (code **)((int)*_PTR_LOOP_1050_02a0 + 0x4);
            (**ppcVar1)(0x1010, (int)_PTR_LOOP_1050_02a0, (int)((ulong)_PTR_LOOP_1050_02a0 >> 0x10), 0x10, iVar7, uVar9, uVar10, puVar6);
            puVar12 = pass1_1008_941a((ushort *)CONCAT22(param_6, local_18), 0x1, 0x86);
            msg     = local_18;
            rect    = (RECT16 *)0x1008;
            win_1008_5c9e((ulong)_PTR_LOOP_1050_02a0, (ulong *)CONCAT22(param_6, msg), msg, (int)((ulong)puVar12 >> 0x10), param_6);
            if(msg != (ushort *)0x0)
            {
                return;
            }
            uVar9     = 0xf6;
            puStack26 = msg;
            goto LAB_1020_5936;
        }
        uVar9 = 0xf6;
    }
    else
    {
        uVar11         = pass1_1020_6498(*(ulong *)(iVar7 + 0xf6), 0x2);
        uStack30._2_2_ = (uchar *)(uVar11 >> 0x10);
        uStack14       = (undefined2)uVar11;
        rect           = (RECT16 *)s_tile2_bmp_1050_1538;
        puStack12      = uStack30._2_2_;
        BVar4          = PtInRect16(param_5, (POINT16)CONCAT22(uStack4, local_6));
        if(BVar4 == 0x0)
            goto LAB_1020_5942;
        uVar9 = 0x68;
    }
    msg = (ushort *)0x0;
LAB_1020_5936:
    PostMessage16((HWND16)rect, (UINT16)msg, (WPARAM16)msg, CONCAT22(0x111, uVar9));
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far menu_ui_op_1020_5bf2(astruct_52 *param_1, HWND16 param_2, RECT16 *param_3)

{
    undefined4  uVar1;
    ushort      uVar2;
    BOOL16      BVar3;
    RECT16    **ppRVar4;
    HMENU16     HVar5;
    uint        in_DX;
    uint        uVar6;
    astruct_52 *iVar5;
    undefined2  uVar7;
    RECT16     *unaff_CS;
    RECT16     *instance;
    WNDCLASS16 *unaff_SS;
    ulong       uVar8;
    POINT16     local_10;
    int         iStack12;
    undefined4  uStack10;
    RECT16     *local_6;
    HWND16      HStack4;

    local_6  = param_3;
    HStack4  = param_2;
    uVar7    = (undefined2)((ulong)param_1 >> 0x10);
    iVar5    = (astruct_52 *)param_1;
    uVar2    = pass1_1020_64d4(iVar5->field_0xf6, 0x2);
    uVar8    = (ulong)in_DX << 0x10;
    instance = unaff_CS;
    if(uVar2 != 0x0)
    {
        uStack10 = pass1_1020_6498(iVar5->field_0xf6, 0x2);
        instance = (RECT16 *)s_tile2_bmp_1050_1538;
        uVar8    = uStack10;
        BVar3    = PtInRect16(unaff_CS, (POINT16)CONCAT22(HStack4, local_6));
        if(BVar3 != 0x0)
        {
            PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110131);
            return 0x1;
        }
    }
    uVar2 = pass1_1020_64d4(iVar5->field_0xf6, 0x3);
    if(uVar2 == 0x0)
    {
        return 0x0;
    }
    ppRVar4 = &local_6;
    pt_in_rect_1020_5856((ulong)param_1, (POINT16 *)CONCAT22(unaff_SS, ppRVar4), ppRVar4);
    uVar6              = (uint)(uVar8 >> 0x10);
    iVar5->field_0x108 = (uint)ppRVar4;
    iVar5->field_0x10a = uVar6;
    if((uVar6 | iVar5->field_0x108) == 0x0)
    {
        return 0x0;
    }
    if(iVar5->field_0x106 == 0x0)
    {
        HVar5              = LoadMenu16((HINSTANCE16)instance, (LPCSTR)s_TILEMENU_1050_43f0);
        iVar5->field_0x106 = HVar5;
        if(HVar5 == 0x0)
        {
            return 0x1;
        }
        instance           = (RECT16 *)s_tile2_bmp_1050_1538;
        HVar5              = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0);
        iVar5->field_0x106 = HVar5;
        if(HVar5 == 0x0)
        {
            return 0x1;
        }
    }
    uVar1          = *(undefined4 *)&iVar5->field_0x108;
    uStack10._0_2_ = *(ushort *)((int)uVar1 + 0x2e);
    iStack12       = 0x0;
    if((ushort)uStack10 == 0x42)
    {
        iStack12 = 0xc9;
    }
    else
    {
        if(*(int *)((int)s_VrMode_1050_42ca + 0x8 + (ushort)uStack10 * 0x2) == 0x0)
        {
            iStack12 = 0xc8;
        }
    }
    if(iStack12 != 0x0)
    {
        instance = (RECT16 *)0x1008;
        win_1008_5c7c(_PTR_LOOP_1050_02a0, CONCAT22(iStack12, 0x1), unaff_SS, (ushort)uStack10, (ushort)(uVar8 >> 0x10));
    }
    local_10.x = (INT16)param_3;
    local_10.y = param_2;
    ClientToScreen16((HWND16)instance, &local_10);
    TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0, 0x0, iVar5->field_0x8, 0x0, local_10.y, (RECT16 *)local_10.x);
    return 0x1;
}

void __stdcall16far win_ui_op_1020_5de8(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    ushort     uVar1;
    undefined4 uVar2;
    ushort    *puVar3;
    uchar     *in_DX;
    uint       uVar4;
    int        unaff_DI;
    undefined2 uVar5;
    ushort    *puVar6;
    undefined  uVar7;
    undefined  uVar8;
    undefined2 uStack18;
    char       cStack15;
    ushort     local_6;
    ushort     uStack4;

    ReleaseCapture16();
    uVar5 = (undefined2)(param_1 >> 0x10);
    SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    *(undefined2 *)((int)param_1 + 0xee) = 0x0;
    *(undefined2 *)((int)param_1 + 0xf4) = 0x1;
    local_6                              = param_3;
    uStack4                              = param_2;
    puVar6                               = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x47, param_4, in_DX, unaff_DI);
    uVar4                                = (uint)((ulong)puVar6 >> 0x10);
    puVar3                               = &local_6;
    pt_in_rect_1020_5856(param_1, (POINT16 *)CONCAT13((char)(param_4 >> 0x8), CONCAT12((char)param_4, puVar3)), puVar3);
    if((uVar4 | (uint)puVar3) != 0x0)
    {
        uVar2    = *(undefined4 *)(puVar3 + 0x21);
        uVar1    = puVar3[0x22];
        cStack15 = (char)((ulong)uVar2 >> 0x18);
        if(cStack15 == '\x05')
        {
            uVar7    = (undefined)uVar1;
            uVar8    = (undefined)(uVar1 >> 0x8);
            uStack18 = (undefined2)uVar2;
            goto LAB_1020_5e62;
        }
    }
    uStack18 = 0x0;
    uVar7    = 0x0;
    uVar8    = 0x0;
LAB_1020_5e62:
    pass1_1018_57e6((ulong)puVar6, CONCAT13(uVar8, CONCAT12(uVar7, uStack18)), param_4);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1020_5e76(ulong param_1, ushort param_2, ushort param_3)

{
    code      **ppcVar1;
    astruct_57 *paVar2;
    ushort     *puVar3;
    uchar      *puVar4;
    int         iVar5;
    uint        uVar6;
    uint        in_DX;
    uchar      *puVar7;
    uchar      *puVar8;
    int         iVar9;
    undefined4 *puVar10;
    int         unaff_DI;
    undefined2  uVar11;
    undefined2  uVar12;
    ushort      uVar13;
    uchar      *unaff_SS;
    undefined   in_AF;
    char       *pcVar14;
    undefined   uVar15;
    ushort     *local_2aa[0x40];
    uchar      *local_1aa[0x80];
    char        local_aa[0x80];
    undefined4  uStack42;
    astruct_57 *paStack38;
    char        local_22[0x10];
    uchar      *puStack18;
    undefined2  uStack16;
    ushort      uStack14;
    ushort      uStack12;
    undefined4  uStack10;
    ushort      local_6;
    ushort      uStack4;

    ReleaseCapture16();
    uVar11 = (undefined2)(param_1 >> 0x10);
    iVar9  = (int)param_1;
    SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    *(undefined2 *)(iVar9 + 0xee) = 0x0;
    *(undefined2 *)(iVar9 + 0xf4) = 0x1;
    local_6                       = param_3;
    uStack4                       = param_2;
    puVar3                        = &local_6;
    uVar15                        = (undefined)((uint)unaff_SS >> 0x8);
    pt_in_rect_1020_5856(param_1, (POINT16 *)CONCAT13(uVar15, CONCAT12((char)unaff_SS, puVar3)), puVar3);
    uStack10 = CONCAT22(in_DX, puVar3);
    puVar7   = (uchar *)(in_DX | (uint)puVar3);
    if(puVar7 == (uchar *)0x0)
        goto LAB_1020_6176;
    uStack12 = puVar3[0x6];
    uStack14 = puVar3[0x7];
    uStack16 = 0x0;
    uVar13   = 0x1018;
    puVar4   = pass1_1018_2580(*(ulong *)(iVar9 + 0xfa), 0x0, CONCAT22(uStack12, uStack14), *(ushort *)(iVar9 + 0x10c), unaff_SS, in_AF);
    if(puVar4 == (uchar *)0x6b2)
        goto LAB_1020_6176;
    puStack18 = puVar4;
    if(puVar4 == (uchar *)0x6b8)
    {
        mem_op_1000_179c(0xb4, puVar7, 0x1000);
        uStack42 = CONCAT22(puVar7, puVar4);
        puVar8   = (uchar *)((uint)puVar7 | (uint)puVar4);
        if(puVar8 == (uchar *)0x0)
        {
            iVar5  = 0x0;
            puVar8 = (uchar *)0x0;
        }
        else
        {
            iVar5 = string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8), CONCAT12((char)puVar7, puVar4)), (ushort)PTR_LOOP_1050_0396, 0x40, 0x2, 0x6b8, 0x6ad, puVar8, (ushort)unaff_SS);
        }
        paStack38 = (astruct_57 *)CONCAT22(puVar8, iVar5);
        uVar13    = 0xa5;
    LAB_1020_5f84:
        pass1_1008_941a((ushort *)CONCAT22(unaff_SS, local_22), 0x1, uVar13);
        pcVar14 = local_22;
        uVar12  = (undefined2)((ulong)paStack38 >> 0x10);
        puVar10 = (undefined4 *)paStack38;
    }
    else
    {
        if(puVar4 == (uchar *)0x6b4)
        {
            mem_op_1000_179c(0xb4, puVar7, 0x1000);
            uStack42 = CONCAT22(puVar7, puVar4);
            puVar8   = (uchar *)((uint)puVar7 | (uint)puVar4);
            if(puVar8 == (uchar *)0x0)
            {
                iVar5  = 0x0;
                puVar8 = (uchar *)0x0;
            }
            else
            {
                iVar5 = string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8), CONCAT12((char)puVar7, puVar4)), (ushort)PTR_LOOP_1050_0396, 0x40, 0x2, 0x57b, puStack18, puVar8, (ushort)unaff_SS);
            }
            paStack38 = (astruct_57 *)CONCAT22(puVar8, iVar5);
            uVar13    = 0xab;
            goto LAB_1020_5f84;
        }
        if(puVar4 == (uchar *)0x6b6)
        {
            load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_aa, (short)unaff_SS);
            load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (char *)local_1aa, (short)unaff_SS);
            uVar6  = sys_1000_3f9c((uchar *)local_2aa, unaff_SS, (ushort)local_1aa, (ushort)unaff_SS, (ushort)PTR_LOOP_1050_50cc, &stack0xfffe, uVar11, 0x1000, unaff_SS, in_AF);
            uVar12 = 0x1000;
            mem_op_1000_179c(0xb4, puVar7, 0x1000);
            uStack42 = CONCAT22(puVar7, uVar6);
            if(((uint)puVar7 | uVar6) == 0x0)
            {
                paStack38 = (astruct_57 *)0x0;
            }
            else
            {
                uVar12    = SUB42(&PTR_LOOP_1050_1040, 0x0);
                paStack38 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar7, uVar6), 0x40, (char *)CONCAT13(uVar15, CONCAT12((char)unaff_SS, local_aa)), (char *)CONCAT22(unaff_SS, local_2aa), (ushort)PTR_LOOP_1050_0396, (uint)puVar7 | uVar6);
            }
            puVar8  = (uchar *)((ulong)paStack38 >> 0x10);
            puVar10 = (undefined4 *)paStack38;
            paVar2  = paStack38;
        LAB_1020_6027:
            ppcVar1 = (code **)((int)*puVar10 + 0x74);
            (**ppcVar1)(uVar12, paVar2);
            goto LAB_1020_6176;
        }
        if(puVar4 < (uchar *)0x6a7)
        {
            if((*(int *)(iVar9 + 0x10c) == 0x78) || (*(int *)(iVar9 + 0x10c) == 0x74))
            {
                uVar13       = 0x1010;
                local_2aa[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x5, (ushort)unaff_SS, puVar7, unaff_DI);
                puVar7       = (uchar *)((ulong)local_2aa[0] >> 0x10);
                if(*(int *)((int)local_2aa[0] + 0xa) == 0x0)
                {
                    return;
                }
            }
            if((((*(int *)(iVar9 + 0x10c) == 0x6c) || (*(int *)(iVar9 + 0x10c) == 0x6d)) || (*(int *)(iVar9 + 0x10c) == 0x31)) || (*(int *)(iVar9 + 0x10c) == 0x32))
            {
                uVar13       = 0x1010;
                local_2aa[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3a, (ushort)unaff_SS, puVar7, unaff_DI);
                if(*(int *)((int)local_2aa[0] + 0xa) == 0x0)
                {
                    return;
                }
            }
            pass1_1020_68de(*(ulong *)(iVar9 + 0xfe), uVar13);
            goto LAB_1020_6176;
        }
        if((uchar *)0x6b1 < puVar4)
        {
            uVar12 = 0x1000;
            mem_op_1000_179c(0xb4, puVar7, 0x1000);
            uStack42 = CONCAT22(puVar7, puVar4);
            puVar8   = (uchar *)((uint)puVar7 | (uint)puVar4);
            if(puVar8 == (uchar *)0x0)
            {
                puVar10 = (undefined4 *)0x0;
                puVar8  = (uchar *)0x0;
            }
            else
            {
                uVar12  = SUB42(&PTR_LOOP_1050_1040, 0x0);
                puVar10 = (undefined4 *)string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8), CONCAT12((char)puVar7, puVar4)), (ushort)PTR_LOOP_1050_0396, 0x40, 0x2, 0x57b, puStack18, puVar8, (ushort)unaff_SS);
            }
            local_2aa[0] = (ushort *)CONCAT22(puVar8, puVar10);
            paVar2       = (astruct_57 *)CONCAT22(puVar8, puVar10);
            goto LAB_1020_6027;
        }
        mem_op_1000_179c(0xb4, puVar7, 0x1000);
        uStack42 = CONCAT22(puVar7, puVar4);
        puVar8   = (uchar *)((uint)puVar7 | (uint)puVar4);
        if(puVar8 == (uchar *)0x0)
        {
            iVar5  = 0x0;
            puVar8 = (uchar *)0x0;
        }
        else
        {
            iVar5 = string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8), CONCAT12((char)puVar7, puVar4)), (ushort)PTR_LOOP_1050_0396, 0x40, 0x2, 0x57b, puStack18, puVar8, (ushort)unaff_SS);
        }
        local_2aa[0] = (ushort *)CONCAT22(puVar8, iVar5);
        local_1aa[0] = puStack18 + -0x608;
        pass1_1008_941a((ushort *)CONCAT22(unaff_SS, local_aa), 0x1, (ushort)local_1aa[0]);
        pcVar14 = local_aa;
        uVar12  = (undefined2)((ulong)local_2aa[0] >> 0x10);
        puVar10 = (undefined4 *)local_2aa[0];
    }
    ppcVar1 = (code **)((int)*puVar10 + 0x6c);
    (**ppcVar1)(0x1008, (char)puVar10, uVar12, pcVar14);
LAB_1020_6176:
    *(undefined2 *)(iVar9 + 0x10c) = 0x0;
    return;
}

void __stdcall16far pass1_1020_6184(ulong param_1, ushort param_2, ushort param_3)

{
    HCURSOR16  HVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(int *)(iVar2 + 0xf4) == 0x1)
    {
        HVar1                        = SetCursor16(param_3);
        *(HCURSOR16 *)(iVar2 + 0xee) = HVar1;
        *(ushort *)(iVar2 + 0x10c)   = param_2;
        SetCapture16((HWND16)s_tile2_bmp_1050_1538);
        *(undefined2 *)(iVar2 + 0xf4) = 0x2;
    }
    return;
}

void __stdcall16far pass1_1020_434c(int param_1, ushort param_2, ulong *param_3, ushort param_4, ushort param_5, int param_6, ushort param_7, ushort param_8)

{
    if(param_6 == 0x1)
    {
        pass1_1020_6184(CONCAT22(param_2, param_1), param_5, param_8);
        return;
    }
    if(param_6 == 0x2)
    {
        ui_op_1020_536e(CONCAT22(param_2, param_1), CONCAT22(param_4, param_3), param_5, 0x2, param_7);
        return;
    }
    pass1_1008_68ea(param_1, param_2, param_3, param_4, param_5, param_6);
    return;
}

void __stdcall16far mixed_menu_op_1020_44ec(ulong param_1, ushort param_2, int param_3, HMENU16 param_4, HMENU16 param_5, ushort param_6)

{
    undefined4 uVar1;
    ushort     uVar2;
    UINT16     UVar3;
    BOOL16     BVar4;
    HMENU16    HVar5;
    uint       uVar6;
    int        iVar7;
    ulong      uVar8;
    uchar     *in_DX;
    uchar     *puVar9;
    int        iVar10;
    int        unaff_DI;
    undefined2 uVar11;
    HMENU16    HVar12;
    uchar      in_AF;
    UINT16     local_138[0x2];
    ushort     local_134[0x2];
    ushort    *puStack304;
    ulong      uStack300;
    undefined4 uStack296;
    ulong      uStack292;
    char      *pcStack286;
    undefined4 uStack278;
    BOOL16     BStack270;
    undefined4 uStack268;
    ulong      local_108[0x40];
    ushort     uStack8;
    ushort    *puStack6;

    uVar11 = (undefined2)(param_1 >> 0x10);
    iVar10 = (int)param_1;
    HVar12 = param_5;
    if(*(int *)(iVar10 + 0x106) != 0x0)
    {
        if(*(HMENU16 *)(iVar10 + 0x106) == param_4)
        {
            puStack6        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
            uVar1           = *(undefined4 *)(iVar10 + 0x108);
            uStack8         = *(ushort *)((int)uVar1 + 0x2e);
            uVar1           = *(undefined4 *)(iVar10 + 0x108);
            uVar11          = (undefined2)((ulong)uVar1 >> 0x10);
            iVar10          = (int)uVar1;
            uStack296       = *(char **)(iVar10 + 0x42);
            puVar9          = *(uchar **)(iVar10 + 0x44);
            uStack296._3_1_ = (byte)((ulong)uStack296 >> 0x18);
            uVar6           = (uint)uStack296._3_1_;
            pcStack286      = uStack296;
            uStack268       = uStack296;
            if(uStack296._3_1_ == 0x0)
            {
                uVar2  = pass1_1020_bd80(uStack8);
                HVar12 = 0x1000;
                unk_str_op_1000_3d3e((char *)CONCAT22(param_6, local_108), (char *)CONCAT22(puVar9, uVar2));
            }
            else
            {
                pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), (ulong)uStack296 & 0xffff | ZEXT24(puVar9) << 0x10);
                uStack296 = (char *)CONCAT22(puVar9, uVar6);
                HVar12    = 0x1010;
                pass1_1010_c3c2((ushort)puStack6, (ushort)((ulong)puStack6 >> 0x10), CONCAT22(param_6, local_108), CONCAT22(puVar9, uVar6), puVar9, in_AF, param_6);
            }
            BStack270 = ModifyMenu16(HVar12, (UINT16)local_108, param_6, 0x76, 0x0);
            UVar3     = GetMenuState16((HMENU16)s_tile2_bmp_1050_1538, 0x0, 0x13c);
            if(UVar3 != 0xffff)
            {
                DeleteMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0, 0x13c);
            }
            HVar12 = 0x1008;
            BVar4  = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x20);
            if(BVar4 != 0x0)
            {
                uStack296 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
                HVar12    = (HMENU16)s_tile2_bmp_1050_1538;
                InsertMenu16(0x1010, (UINT16)uStack296, (UINT16)((ulong)uStack296 >> 0x10), 0x13c, 0x400);
            }
            if(*(int *)((int)s_VrMode_1050_42ca + 0x8 + uStack8 * 0x2) == 0x0)
            {
                UVar3   = 0x1;
                param_4 = 0x77;
                goto LAB_1020_464c;
            }
            param_4 = 0x77;
        }
        else
        {
            HVar12    = (HMENU16)s_tile2_bmp_1050_1538;
            HVar5     = GetSubMenu16(param_5, 0x1);
            uStack296 = (char *)((ulong)uStack296 & 0xffff0000 | (ulong)HVar5);
            if(HVar5 != param_4)
                goto LAB_1020_479e;
            EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538, 0x1, 0x200);
            EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538, 0x1, 0x201);
            EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538, 0x1, 0x202);
            uVar1     = *(undefined4 *)(iVar10 + 0x108);
            uVar8     = *(ulong *)((int)uVar1 + 0x42);
            uStack292 = uVar8;
            pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), uVar8);
            uVar6      = (uint)uVar8;
            pcStack286 = (char *)(uVar8 & 0xffff | ZEXT24(in_DX) << 0x10);
            if(((uint)in_DX | uVar6) == 0x0)
            {
                return;
            }
            uStack278 = *(undefined4 *)(uVar6 + 0x2e);
            if((*(uint *)(uVar6 + 0x30) | (uint)uStack278) == 0x0)
            {
                return;
            }
            uStack268    = *(char **)((uint)uStack278 + 0x200);
            local_108[0] = struct_op_1030_73a8(CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, uVar6)));
            uVar11       = (undefined2)(local_108[0] >> 0x10);
            puStack6     = *(ushort **)((int)local_108[0] + 0x1c);
            uVar6        = *(uint *)((int)local_108[0] + 0x1e);
            if((uVar6 | (uint)puStack6) != 0x0)
            {
                uStack268 = (char *)((ulong)puStack6 & 0xffff | (ulong)uVar6 << 0x10);
            }
            uStack268._2_2_ = uStack268._2_2_ & 0xff;
            if((int)uStack268 != 0x1)
            {
                return;
            }
            if(((ulong)uStack268 & 0xff0000) != 0x0)
            {
                return;
            }
            local_134[0] = pass1_1030_6fa0((ulong)pcStack286);
            HVar12       = 0x1008;
            BVar4        = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_134[0], 0x3f);
            if(BVar4 != 0x0)
            {
                HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
                BVar4  = EnableMenuItem16(0x1008, 0x0, 0x201);
            }
            if(*(long *)((int)pcStack286 + 0x36) != 0x0)
            {
                BVar4 = EnableMenuItem16(HVar12, 0x0, 0x202);
            }
            HVar12 = 0x1030;
            pass1_1030_69cc((ulong)pcStack286, BVar4, uStack268._2_2_, param_6);
            if(BVar4 == 0x0)
            {
                return;
            }
            param_4 = 0x200;
        }
        UVar3 = 0x0;
        goto LAB_1020_464c;
    }
LAB_1020_479e:
    iVar7 = param_3 + -0x1;
    if(iVar7 == 0x0)
    {
        pass1_1018_2504(0x0, in_DX);
        if(iVar7 == 0x0)
        {
            EnableMenuItem16(0x1018, 0x401, 0x0);
        LAB_1020_47e3:
            HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
            UVar3  = 0x401;
            goto LAB_1020_464c;
        }
        HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
        EnableMenuItem16(0x1018, 0x400, 0x0);
    }
    else
    {
        if(param_3 == 0x2)
        {
            uVar2 = pass1_1020_64d4(*(ulong *)(iVar10 + 0xf6), 0x2);
            if(uVar2 == 0x0)
            {
                EnableMenuItem16(HVar12, 0x401, 0x0);
            }
            else
            {
                EnableMenuItem16(HVar12, 0x400, 0x0);
            }
            HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
            EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538, CONCAT11(0x4, uVar2 == 0x0), 0x1);
            if((PTR_LOOP_1050_0010 != (undefined *)0x0) || (*(long *)(iVar10 + 0x102) == 0x0))
                goto LAB_1020_47e3;
        }
        else
        {
            if(param_3 == 0x3)
            {
                local_138[0] = 0x0;
                local_134[0] = 0x0;
                HVar12       = 0x1010;
                puStack304   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, in_DX, unaff_DI);
                uVar11       = (undefined2)((ulong)puStack304 >> 0x10);
                uStack300    = *(ulong *)((int)puStack304 + 0x20);
                uVar6        = *(uint *)((int)puStack304 + 0x22);
                if((uVar6 | (uint)uStack300) != 0x0)
                {
                    HVar12 = 0x1030;
                    pass1_1030_8308((ushort)_PTR_LOOP_1050_5748,
                                    (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                                    (ushort *)CONCAT22(param_6, local_134),
                                    (ushort *)CONCAT22(param_6, local_138),
                                    uStack300 & 0xffff | (ulong)uVar6 << 0x10,
                                    (ushort)local_134,
                                    uVar6);
                    local_138[0] = *(UINT16 *)((int)puStack304 + 0x1e);
                }
                uStack296 = (char *)((ulong)uStack296 & 0xffff0000);
                do
                {
                    CheckMenuItem16(HVar12, 0x400, (UINT16)uStack296);
                    HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
                    EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538, 0x401, (UINT16)uStack296);
                    uStack296 = (char *)((ulong)uStack296 & 0xffff0000 | (ulong)((UINT16)uStack296 + 0x1));
                } while((int)((UINT16)uStack296 + 0x1) < 0x5);
                CheckMenuItem16((HMENU16)s_tile2_bmp_1050_1538, 0x408, local_138[0]);
                for(uStack296 = (char *)((ulong)uStack296 & 0xffff0000); (int)(UINT16)uStack296 <= (int)local_134[0]; uStack296 = (char *)((ulong)uStack296 & 0xffff0000 | (ulong)((UINT16)uStack296 + 0x1)))
                {
                    EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538, 0x400, (UINT16)uStack296);
                }
                return;
            }
            if(param_3 != 0x4)
            {
                return;
            }
            param_4 = 0x2;
        }
    }
    UVar3 = 0x400;
LAB_1020_464c:
    EnableMenuItem16(HVar12, UVar3, param_4);
    return;
}

void __stdcall16far win_sys_op_1020_493c(ulong *param_1, uint param_2, uchar *param_3, undefined2 param_4, HCURSOR16 param_5, WNDCLASS16 *param_6)

{
    code       **ppcVar1;
    HCURSOR16    HVar2;
    undefined   *puVar3;
    int          iVar4;
    undefined4  *puVar5;
    ushort       uVar6;
    uchar       *puVar7;
    uchar       *puVar8;
    ushort       uVar9;
    uint         uVar10;
    int          unaff_DI;
    undefined2   uVar11;
    uchar        in_AF;
    undefined4   uVar12;
    ushort      *puVar13;
    astruct_100 *paVar14;
    char        *pcVar15;
    undefined    uVar16;
    ushort       uVar17;
    undefined2   uVar18;
    uint         uVar19;
    undefined4   local_356[0x42];
    uint         local_24e;
    uchar       *puStack588;
    undefined4   local_144;
    undefined4   uStack320;
    undefined4   local_13c;
    ushort       uStack42;
    undefined4   uStack38;
    uint         uStack34;
    uchar       *puStack32;
    undefined4   uStack30;
    undefined4   uStack26;
    ulong        uStack22;
    astruct_43  *paStack18;
    undefined   *puStack14;
    uchar       *puStack12;
    ushort       uStack10;
    undefined4   uStack6;

    if(param_2 == 0xe9)
    {
        return;
    }
    uVar9  = (ushort)param_1;
    puVar8 = (uchar *)((ulong)param_1 >> 0x10);
    if(param_2 < 0xea)
    {
        switch(param_2)
        {
        case 0x69:
            iVar4 = 0x0;
            break;
        case 0x6a:
            iVar4 = 0x1;
            break;
        case 0x6b:
            iVar4 = 0x2;
            break;
        case 0x6c:
            iVar4 = 0x3;
            break;
        case 0x6d:
            iVar4 = 0x4;
            break;
        default:
            return;
        case 0x77:
            if((*(uint *)(uVar9 + 0x10a) | *(uint *)(uVar9 + 0x108)) == 0x0)
            {
                return;
            }
            uVar12   = *(undefined4 *)(uVar9 + 0x108);
            uVar19   = *(uint *)((int)s_VrMode_1050_42ca + 0x8 + *(int *)((int)uVar12 + 0x2e) * 0x2);
            uStack26 = (ushort *)((ulong)uStack26 & 0xffff0000 | (ulong)uVar19);
            if(uVar19 == 0x0)
            {
                return;
            }
            paStack18 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1f8, (ushort)param_6);
            WinHelp16(0x1010, (LPCSTR)uStack26, CONCAT11((undefined)((int)(LPCSTR)uStack26 >> 0xf), (undefined)((int)(LPCSTR)uStack26 >> 0xf)), CONCAT22((int)paStack18, 0x1));
            return;
        case 0x78:
            puVar13    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x45, (ushort)param_6, param_3, unaff_DI);
            puStack588 = (uchar *)(int)((ulong)puVar13 >> 0x10);
            local_24e  = (uint)puVar13;
            enum_child_windows_1010_01be(0x1010);
            return;
        }
        set_cursor_1020_5764((ulong)param_1, iVar4, (ushort)param_6);
        return;
    }
    if(param_2 == 0x132)
    {
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar11 = 0xffff;
        goto LAB_1020_4ef8;
    }
    if(param_2 < 0x133)
    {
        if(param_2 == 0x102)
        {
            uVar16 = 0x0;
            mem_op_1000_179c(0xb4, param_3, 0x1000);
            puVar8    = (uchar *)((uint)param_3 | param_2);
            uStack34  = param_2;
            puStack32 = param_3;
            if(puVar8 == (uchar *)0x0)
            {
                iVar4  = 0x0;
                puVar8 = (uchar *)0x0;
            }
            else
            {
                uVar16 = 0x40;
                iVar4  = string_1040_8520((astruct_57 *)CONCAT22(param_3, param_2), (ushort)PTR_LOOP_1050_0396, 0x31, 0x2, 0x57b, 0x62b, puVar8, (ushort)param_6);
            }
            local_144 = (undefined4 *)CONCAT22(puVar8, iVar4);
            ppcVar1   = (code **)((int)*local_144 + 0x74);
            (**ppcVar1)(uVar16, iVar4, puVar8);
            uStack320 = (undefined4 *)CONCAT22(uStack320._2_2_, iVar4);
            if(iVar4 != 0x1)
            {
                return;
            }
            pass1_1028_837e((astruct_100 *)CONCAT22(param_6, &local_13c), param_6, in_AF);
        LAB_1020_4b6c:
            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, &local_13c));
            return;
        }
        if(param_2 < 0x103)
        {
            switch(param_2)
            {
            case 0xf0:
                ui_op_1020_536e(param_1, 0x0, -0x1, 0x1, param_3);
                return;
            default:
                return;
            case 0xf6:
                if(*(int *)(uVar9 + 0x116) != 0x0)
                {
                    if(param_1 == (ulong *)0x0)
                    {
                        iVar4   = 0x0;
                        param_3 = (uchar *)0x0;
                    }
                    else
                    {
                        iVar4   = uVar9 + 0xe2;
                        param_3 = puVar8;
                    }
                    local_356[0] = CONCAT22(param_3, iVar4);
                    pass1_1010_1ea6(_PTR_LOOP_1050_02a0, CONCAT22(param_3, iVar4), (ushort)param_6);
                    *(undefined2 *)(uVar9 + 0x116) = 0x0;
                }
                iVar4 = 0x12;
                break;
            case 0xf7:
                unk_win_op_1010_7300(*(ulong *)(uVar9 + 0x10e), 0x0, 0x9, 0x0);
                return;
            case 0xfb:
                local_144 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, (ushort)param_6, param_3, unaff_DI);
                uStack320 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, (ushort)param_6, (uchar *)((ulong)local_144 >> 0x10), unaff_DI);
                pcVar15   = (char *)pass1_1010_375e((ulong)uStack320);
                pass1_1010_c25e((ushort)local_144, (ushort)((ulong)local_144 >> 0x10), pcVar15, (uint)pcVar15, (uchar *)((ulong)pcVar15 >> 0x10), unaff_DI, (ushort)param_6);
                return;
            case 0xfc:
                post_msg_1020_55b0((ulong)param_1, param_6);
                return;
            case 0x101:
                uStack26       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, (ushort)param_6, param_3, unaff_DI);
                uVar11         = (undefined2)((ulong)uStack26 >> 0x10);
                uStack22       = *(ulong *)((int)uStack26 + 0x24);
                puVar7         = *(uchar **)((int)uStack26 + 0x26);
                uStack22._0_2_ = (uint)puVar7 | (uint)uStack22;
                if((uint)uStack22 == 0x0)
                {
                    uVar16 = 0x0;
                    mem_op_1000_179c(0xb4, puVar7, 0x1000);
                    puVar8    = (uchar *)((uint)puVar7 | (uint)uStack22);
                    uStack34  = (uint)uStack22;
                    puStack32 = puVar7;
                    if(puVar8 == (uchar *)0x0)
                    {
                        puVar5 = (undefined4 *)0x0;
                        puVar8 = (uchar *)0x0;
                    }
                    else
                    {
                        uVar16 = 0x40;
                        puVar5 = (undefined4 *)string_1040_8520((astruct_57 *)CONCAT22(puVar7, (uint)uStack22), (ushort)PTR_LOOP_1050_0396, 0x30, 0x2, 0x57b, 0x730, puVar8, (ushort)param_6);
                    }
                    uStack30 = CONCAT22(puVar8, puVar5);
                LAB_1020_4c5f:
                    ppcVar1 = (code **)((int)*puVar5 + 0x74);
                    (**ppcVar1)(uVar16, puVar5, puVar8);
                    return;
                }
                uVar12    = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar9 + 0x8), 0xe, (ushort)puVar7, uVar9, (ushort)&PTR_LOOP_1050_1038, (ushort)param_6);
                paStack18 = (astruct_43 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x43, (ushort)param_6, (uchar *)((ulong)uVar12 >> 0x10), unaff_DI);
                uVar11    = (undefined2)((ulong)paStack18 >> 0x10);
                iVar4     = (int)paStack18;
                puStack14 = (undefined *)*(ushort *)(iVar4 + 0xa);
                uStack10  = *(ushort *)(iVar4 + 0xc);
                uVar9     = *(ushort *)(iVar4 + 0xe);
                uStack6   = CONCAT22(uStack6._2_2_, uVar9);
                if(*(int *)(iVar4 + 0x10) != 0x0)
                {
                    return;
                }
                pass1_1028_84ca((astruct_100 *)CONCAT22(param_6, &local_13c), uStack22, uVar9, uStack10, (ushort)puStack14, (ushort)param_6, in_AF);
                goto LAB_1020_4b6c;
            }
        }
        else
        {
            if(param_2 != 0x106)
            {
                if(param_2 < 0x107)
                {
                    if(param_2 == 0x103)
                    {
                        local_144 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, (ushort)param_6, param_3, unaff_DI);
                        uVar11    = (undefined2)((ulong)local_144 >> 0x10);
                        uStack320 = (undefined4 *)*(char **)((int)local_144 + 0x24);
                        puStack32 = *(uchar **)((int)local_144 + 0x26);
                        uStack34  = (uint)puStack32 | (uint)uStack320;
                        if(uStack34 != 0x0)
                        {
                            uVar12    = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar9 + 0x8), 0xf, (ushort)puStack32, uVar9, (ushort)&PTR_LOOP_1050_1038, (ushort)param_6);
                            local_13c = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x42, (ushort)param_6, (uchar *)((ulong)uVar12 >> 0x10), unaff_DI);
                            uStack42  = *(ushort *)((int)local_13c + 0xa);
                            if(uStack42 == 0x0)
                            {
                                return;
                            }
                            pass1_1030_e63e((astruct_100 *)CONCAT22(param_6, &local_24e), uStack42, param_6, in_AF);
                            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, &local_24e));
                            return;
                        }
                        uVar16 = 0x0;
                        mem_op_1000_179c(0xb4, puStack32, 0x1000);
                        puVar8 = (uchar *)((uint)puStack32 | uStack34);
                        if(puVar8 == (uchar *)0x0)
                        {
                            puVar5 = (undefined4 *)0x0;
                            puVar8 = (uchar *)0x0;
                        }
                        else
                        {
                            uVar16 = 0x40;
                            puVar5 = (undefined4 *)string_1040_8520((astruct_57 *)CONCAT22(puStack32, uStack34), (ushort)PTR_LOOP_1050_0396, 0x30, 0x2, 0x57b, 0x730, puVar8, (ushort)param_6);
                        }
                        uStack38 = CONCAT22(puVar8, puVar5);
                    }
                    else
                    {
                        if(param_2 != 0x104)
                        {
                            return;
                        }
                        puVar13    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, (ushort)param_6, param_3, unaff_DI);
                        puStack32  = (uchar *)((ulong)puVar13 >> 0x10);
                        uStack34   = (uint)puVar13;
                        local_24e  = uStack34;
                        puStack588 = puStack32;
                        pass1_1010_af66((ulong)puVar13, (uint)puStack32);
                        local_144 = (undefined4 *)CONCAT22(local_144._2_2_, uStack34);
                        if(uStack34 != 0x0)
                        {
                            uVar16 = 0x0;
                            mem_op_1000_179c(0xb4, puStack32, 0x1000);
                            puVar8 = (uchar *)((uint)puStack32 | uStack34);
                            if(puVar8 == (uchar *)0x0)
                            {
                                iVar4  = 0x0;
                                puVar8 = (uchar *)0x0;
                            }
                            else
                            {
                                uVar16 = 0x40;
                                iVar4  = string_1040_8520((astruct_57 *)CONCAT22(puStack32, uStack34), (ushort)PTR_LOOP_1050_0396, 0x31, 0x2, 0x57b, 0x62c, puVar8, (ushort)param_6);
                            }
                            uStack320 = (undefined4 *)CONCAT22(puVar8, iVar4);
                            ppcVar1   = (code **)((int)*uStack320 + 0x74);
                            (**ppcVar1)(uVar16, iVar4, puVar8);
                            local_13c = (ushort *)CONCAT22(local_13c._2_2_, iVar4);
                            if(iVar4 != 0x1)
                            {
                                return;
                            }
                            uVar16  = (undefined)((uint)param_6 >> 0x8);
                            paVar14 = pass1_1030_e79a((astruct_100 *)CONCAT13(uVar16, CONCAT12((char)param_6, local_356)), param_6, in_AF);
                            uVar9   = (ushort)((ulong)paVar14 >> 0x10);
                            puVar5  = local_356;
                            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT13(uVar16, CONCAT12((char)param_6, puVar5)));
                            win_1008_5c5c(param_6, (ushort)puVar5, uVar9, _PTR_LOOP_1050_02a0, 0x1e6);
                            return;
                        }
                        uVar16 = 0x0;
                        mem_op_1000_179c(0xb4, puStack32, 0x1000);
                        puVar8 = (uchar *)((uint)puStack32 | uStack34);
                        if(puVar8 == (uchar *)0x0)
                        {
                            puVar5 = (undefined4 *)0x0;
                            puVar8 = (uchar *)0x0;
                        }
                        else
                        {
                            uVar16 = 0x40;
                            puVar5 = (undefined4 *)string_1040_8520((astruct_57 *)CONCAT22(puStack32, uStack34), (ushort)PTR_LOOP_1050_0396, 0x30, 0x2, 0x57b, 0x731, puVar8, (ushort)param_6);
                        }
                        local_356[0] = CONCAT22(puVar8, puVar5);
                    }
                    goto LAB_1020_4c5f;
                }
                if(param_2 == 0x12f)
                {
                    pass1_1020_61c4(uVar9, (ushort)puVar8, CONCAT22(param_6, &local_144), (ushort *)CONCAT22(param_6, &local_24e), param_3, unaff_DI, (ushort)param_6);
                    iVar4 = local_24e + 0x6a;
                }
                else
                {
                    if(param_2 != 0x130)
                    {
                        if(param_2 != 0x131)
                        {
                            return;
                        }
                        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x2);
                        if(uVar6 == 0x0)
                        {
                            return;
                        }
                        iVar4 = 0x7;
                        goto LAB_1020_49b7;
                    }
                    pass1_1020_61c4(uVar9, (ushort)puVar8, CONCAT22(param_6, &local_144), (ushort *)CONCAT22(param_6, &local_24e), param_3, unaff_DI, (ushort)param_6);
                    iVar4 = local_24e + 0x68;
                }
                uStack320 = (undefined4 *)CONCAT22(uStack320._2_2_, iVar4);
                if(0x6d < iVar4)
                {
                    return;
                }
                if(iVar4 < 0x69)
                {
                    return;
                }
                ppcVar1 = (code **)((int)*param_1 + 0x40);
                (**ppcVar1)();
                return;
            }
            iVar4 = 0x13;
        }
    LAB_1020_49b7:
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar9 + 0x8), iVar4, (ushort)param_3, uVar9, (ushort)&PTR_LOOP_1050_1038, (ushort)param_6);
        return;
    }
    if(param_2 == 0x1c8)
    {
        SendMessage16(param_5, 0x0, 0x0, 0x1110072);
        return;
    }
    if(0x1c8 < param_2)
    {
        if(param_2 == 0x1ca)
        {
            local_144 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, (ushort)param_6, param_3, unaff_DI);
            uStack320 = (undefined4 *)pass1_1010_c234((uint)local_144, (uchar *)((ulong)local_144 >> 0x10), unaff_DI, (ushort)param_6);
            uVar10    = (uint)((ulong)uStack320 >> 0x10);
            uVar19    = (uint)uStack320;
            if((uchar *)(uVar10 | uVar19) == (uchar *)0x0)
            {
                return;
            }
            local_13c = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, (ushort)param_6, (uchar *)(uVar10 | uVar19), unaff_DI);
            param_3   = (uchar *)((ulong)local_13c >> 0x10);
            pass1_1010_3770((ulong)local_13c, (char *)CONCAT22(uVar10, uVar19), (ushort)param_3);
            iVar4 = 0x3;
        }
        else
        {
            uVar17 = (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10);
            uVar6  = (ushort)_PTR_LOOP_1050_5748;
            if(param_2 == 0x200)
            {
                uVar12         = *(undefined4 *)(uVar9 + 0x108);
                uVar11         = (undefined2)((ulong)uVar12 >> 0x10);
                iVar4          = (int)uVar12;
                uStack26       = *(ushort **)(iVar4 + 0x42);
                param_3        = *(uchar **)(iVar4 + 0x44);
                uStack26._3_1_ = (byte)((ulong)uStack26 >> 0x18);
                puStack14      = (undefined *)(uint)uStack26._3_1_;
                if(uStack26._3_1_ != 0x5)
                {
                    return;
                }
                pass1_1030_8344(uVar6, uVar17, (ulong)uStack26 & 0xffff | ZEXT24(param_3) << 0x10);
                iVar4              = 0x25;
                PTR_LOOP_1050_5f0c = puStack14;
                PTR_LOOP_1050_5f0e = param_3;
                puStack12          = param_3;
            }
            else
            {
                if(param_2 == 0x201)
                {
                    uVar12         = *(undefined4 *)(uVar9 + 0x108);
                    uVar11         = (undefined2)((ulong)uVar12 >> 0x10);
                    iVar4          = (int)uVar12;
                    uStack26       = *(ushort **)(iVar4 + 0x42);
                    param_3        = *(uchar **)(iVar4 + 0x44);
                    uStack26._3_1_ = (byte)((ulong)uStack26 >> 0x18);
                    puStack14      = (undefined *)(uint)uStack26._3_1_;
                    if(uStack26._3_1_ != 0x5)
                    {
                        return;
                    }
                    pass1_1030_8344(uVar6, uVar17, (ulong)uStack26 & 0xffff | ZEXT24(param_3) << 0x10);
                    iVar4              = 0x26;
                    PTR_LOOP_1050_5f16 = puStack14;
                    PTR_LOOP_1050_5f18 = param_3;
                    puStack12          = param_3;
                }
                else
                {
                    if(param_2 != 0x202)
                    {
                        if(param_2 != 0x203)
                        {
                            return;
                        }
                        if(*(int *)(uVar9 + 0xf4) != 0x1)
                        {
                            return;
                        }
                        HVar2                         = SetCursor16(param_5);
                        *(HCURSOR16 *)(uVar9 + 0xee)  = HVar2;
                        *(undefined2 *)(uVar9 + 0xf4) = 0x3;
                        SetCapture16((HWND16)s_tile2_bmp_1050_1538);
                        return;
                    }
                    uVar12        = *(undefined4 *)(uVar9 + 0x108);
                    uVar11        = (undefined2)((ulong)uVar12 >> 0x10);
                    iVar4         = (int)uVar12;
                    uStack6       = *(ulong *)(iVar4 + 0x42);
                    param_3       = *(uchar **)(iVar4 + 0x44);
                    uStack6._3_1_ = (byte)(uStack6 >> 0x18);
                    puVar3        = (undefined *)(uint)uStack6._3_1_;
                    if(uStack6._3_1_ != 0x5)
                    {
                        return;
                    }
                    pass1_1030_8344(uVar6, uVar17, uStack6 & 0xffff | ZEXT24(param_3) << 0x10);
                    uStack22           = CONCAT22(param_3, puVar3);
                    iVar4              = 0x27;
                    PTR_LOOP_1050_5a68 = puVar3;
                    PTR_LOOP_1050_5a6a = param_3;
                }
            }
        }
        goto LAB_1020_49b7;
    }
    switch(param_2)
    {
    case 0x133:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar18 = 0xffff;
        uVar11 = 0x0;
        break;
    case 0x134:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar11 = 0x1;
        goto LAB_1020_4ef8;
    case 0x135:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar18 = 0x1;
        uVar11 = 0x0;
        break;
    case 0x136:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar11 = 0xfffb;
        goto LAB_1020_4ef8;
    case 0x137:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar18 = 0xfffb;
        uVar11 = 0x0;
        break;
    case 0x138:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar11 = 0x5;
    LAB_1020_4ef8:
        uVar18 = 0x0;
        break;
    case 0x139:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x3);
        if(uVar6 == 0x0)
        {
            return;
        }
        uVar18 = 0x5;
        uVar11 = 0x0;
        break;
    default:
        goto switchD_1020_518a_caseD_13a;
    case 0x13c:
        uVar6 = pass1_1020_64d4(*(ulong *)(uVar9 + 0xf6), 0x2);
        if(uVar6 != 0x0)
        {
            iVar4 = 0x1a;
            goto LAB_1020_49b7;
        }
        goto switchD_1020_518a_caseD_13a;
    }
    pass1_1020_2a94(*(ulong *)(uVar9 + 0xce), CONCAT22(uVar11, uVar18), (ushort)param_6);
switchD_1020_518a_caseD_13a:
    return;
}

BOOL16 __stdcall16far enable_menu_item_1020_2c2a(HMENU16 param_1, int param_2)

{
    BOOL16 BVar1;
    UINT16 w_item_id;

    if(param_2 != 0x0)
    {
        return param_2 - 0x1;
    }
    EnableMenuItem16(param_1, 0x400, 0x3);
    if((int)PTR_LOOP_1050_3960 < 0x2)
    {
        w_item_id = 0x401;
    }
    else
    {
        w_item_id = 0x400;
    }
    BVar1 = EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538, w_item_id, 0x5);
    return BVar1;
}

void __stdcall16far win_ui_op_1020_2cf0(astruct *param_1)

{
    undefined4 uVar1;
    code     **ppcVar2;
    uint       uVar3;
    BOOL16    *pBVar4;
    uchar     *in_DX;
    undefined2 uVar5;
    uchar     *extraout_DX;
    uchar     *puVar6;
    uint       uVar7;
    undefined2 extraout_DX_00;
    int        iVar8;
    int        unaff_DI;
    uint       uVar9;
    ushort     unaff_SS;
    ushort    *puVar10;
    ulong      uVar11;
    undefined *puVar12;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar9                         = (uint)((ulong)param_1 >> 0x10);
    iVar8                         = (int)param_1;
    puVar10                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, *(ushort *)(iVar8 + 0xfc), unaff_SS, in_DX, unaff_DI);
    uVar5                         = (undefined2)((ulong)puVar10 >> 0x10);
    *(undefined2 *)(iVar8 + 0xf2) = (int)puVar10;
    *(undefined2 *)(iVar8 + 0xf4) = uVar5;
    *(undefined2 *)(iVar8 + 0xe0) = *(undefined2 *)(iVar8 + 0xf2);
    *(undefined2 *)(iVar8 + 0xe2) = uVar5;
    puVar12                       = PTR_LOOP_1050_038c;
    uVar3                         = LoadIcon16(0x1010, (LPCSTR)s_SITEICON_1050_428d);
    *(HICON16 *)(iVar8 + 0xc2)    = uVar3;
    uVar1                         = *(undefined4 *)(iVar8 + 0xf2);
    ppcVar2                       = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xf2) + 0x30);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)uVar1, (int)((ulong)uVar1 >> 0x10), uVar3, puVar12);
    puVar6 = extraout_DX;
    mem_op_1000_179c(0x22, extraout_DX, 0x1000);
    uVar7 = (uint)puVar6 | uVar3;
    if(uVar7 == 0x0)
    {
        *(undefined4 *)(iVar8 + 0xf6) = 0x0;
    }
    else
    {
        load_draw_op_1020_2ede((ushort *)CONCAT22(puVar6, uVar3), (ulong)param_1, 0x1000);
        *(uint *)(iVar8 + 0xf6) = uVar3;
        *(uint *)(iVar8 + 0xf8) = uVar7;
    }
    *(undefined4 *)(iVar8 + 0xe8) = *(undefined4 *)(iVar8 + 0xf6);
    pass1_1018_0ac0(*(ulong *)(iVar8 + 0xf2), (ulong)param_1 & 0xffff | (ulong)uVar9 << 0x10);
    uVar11  = pass1_1018_0b08(*(ulong *)(iVar8 + 0xf2));
    pBVar4  = (BOOL16 *)uVar11;
    ppcVar2 = (code **)((int)*(undefined4 *)param_1 + 0x14);
    (**ppcVar2)();
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xf2) + 0x10);
    (**ppcVar2)();
    MoveWindow16(0x1018, 0x1, pBVar4[0x3], pBVar4[0x2], pBVar4[0x1], *pBVar4);
    pass1_1008_3e0e((ulong)param_1);
    UpdateWindow16(0x1008);
    return;
}

void __stdcall16far cleanup_win_ui_1020_2fea(astruct_12 *in_struct_1, HDC16 in_dc_handle_2)

{
    astruct_12 *iVar1;
    UINT16      var2;
    ushort      unaff_SS;

    var2                          = (UINT16)((ulong)in_struct_1 >> 0x10);
    iVar1                         = (astruct_12 *)in_struct_1;
    in_struct_1->offset_field_0x0 = 0x363c;
    iVar1->offset_field_0x2       = 0x1020;
    if(iVar1->field_0x14 != 0x0)
    {
        in_dc_handle_2 = 0x1010;
        pass1_1010_1ea6(iVar1->field_0x14, (ulong)in_struct_1 & 0xffff | (ulong)var2 << 0x10, unaff_SS);
    }
    SelectObject16(in_dc_handle_2, iVar1->field_0x1c);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, iVar1->field_0x1e);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, iVar1->field_0x20);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
    in_struct_1->offset_field_0x0 = 0x3ab0;
    iVar1->offset_field_0x2       = 0x1008;
    in_struct_1->offset_field_0x0 = 0x389a;
    iVar1->offset_field_0x2       = 0x1008;
    return;
}

astruct_18 *__stdcall16far pass1_1020_3616(astruct_18 *param_1, byte param_2, ushort param_3)

{
    cleanup_win_ui_1020_2fea((astruct_12 *)param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far win_ui_op_1020_36f6(ulong param_1, int param_2, short param_3)

{
    int        iVar1;
    code     **ppcVar2;
    undefined4 uVar3;
    char      *pcVar4;
    ushort     uVar5;
    undefined2 uVar6;
    SEGPTR     lp_string;
    int        iVar7;
    undefined2 uVar8;
    HWND16     hwnd;
    char      *pcVar9;
    INT16      id;
    undefined *puStack1034;
    char       local_406[0x400];
    ulong      uStack6;

    iVar7 = (int)param_1;
    uVar8 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        *(undefined4 *)(iVar7 + 0x8) = 0x0;
        return;
    }
    if(param_2 != 0xd)
    {
        return;
    }
    uStack6 = pass1_1018_1e78(*(ulong *)(iVar7 + 0x8), -0x1);
    uVar6   = (undefined2)(uStack6 >> 0x10);
    GetWindowText16(0x1018, 0x3ff, (INT16)local_406);
    pcVar4       = pass1_1000_472c(CONCAT22(param_3, local_406), ':');
    puStack1034  = (undefined *)CONCAT22(uVar6, pcVar4 + 0x2);
    *puStack1034 = 0x0;
    load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_406, param_3);
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0x18) + 0x18);
    (**ppcVar2)();
    uVar3 = *(undefined4 *)(iVar7 + 0x8);
    iVar1 = *(int *)((int)uVar3 + 0x4a);
    uVar3 = *(undefined4 *)((int)uStack6 + 0x2);
    SetDlgItemText16(0x1010, (INT16)uVar3, (SEGPTR)((ulong)uVar3 >> 0x10));
    uVar3 = *(undefined4 *)((int)uStack6 + 0xa);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538, (INT16)uVar3, (SEGPTR)((ulong)uVar3 >> 0x10));
    uVar3 = *(undefined4 *)((int)uStack6 + 0x12);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538, (INT16)uVar3, (SEGPTR)((ulong)uVar3 >> 0x10));
    uVar3 = *(undefined4 *)((int)uStack6 + 0xe);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538, (INT16)uVar3, (SEGPTR)((ulong)uVar3 >> 0x10));
    if(iVar1 != 0x0)
    {
        hwnd  = 0x1018;
        uVar5 = pass1_1018_1f1a(*(ulong *)(iVar7 + 0x8), *(int *)((int)uStack6 + 0x1a));
        if(uVar5 != 0x0)
        {
            uVar3     = *(undefined4 *)((int)uStack6 + 0x16);
            id        = (INT16)uVar3;
            lp_string = (SEGPTR)((ulong)uVar3 >> 0x10);
            goto LAB_1020_3846;
        }
    }
    hwnd      = 0x1010;
    pcVar9    = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    lp_string = (SEGPTR)((ulong)pcVar9 >> 0x10);
    id        = (INT16)pcVar9;
LAB_1020_3846:
    SetDlgItemText16(hwnd, id, lp_string);
    if(*(long *)(iVar7 + 0x1c) != 0x0)
    {
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, *(INT16 *)((int)uStack6 + 0x1a));
        SetFocus16((HWND16)s_tile2_bmp_1050_1538);
        return;
    }
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)0x0, 0x0);
    return;
}

void __stdcall16far pass1_1020_3898(astruct_30 *param_1, ushort param_2)

{
    destroy_window_1020_3b3e(param_1, param_2);
    return;
}

void __stdcall16far window_op_1020_38aa(astruct *param_1)

{
    code       **ppcVar1;
    uint         uVar2;
    astruct_160 *paVar3;
    undefined4   uVar4;
    uchar       *in_DX;
    undefined2   uVar5;
    uchar       *extraout_DX;
    uchar       *puVar6;
    uchar       *extraout_DX_00;
    uchar       *puVar7;
    uint         uVar8;
    undefined2   extraout_DX_01;
    int          unaff_DI;
    HWND16       HVar9;
    ushort       unaff_SS;
    ushort      *puVar10;
    uint         uVar11;
    uint         uVar12;
    RECT16       local_12;
    int          iStack14;
    int          iStack12;
    RECT16       local_a;
    int          iStack6;
    int          iStack4;

    uVar11 = (uint)param_1;
    uVar12 = (uint)((ulong)param_1 >> 0x10);
    create_window_ex_1008_9760(param_1, 0x1008);
    puVar10                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, unaff_SS, in_DX, unaff_DI);
    uVar5                          = (undefined2)((ulong)puVar10 >> 0x10);
    *(undefined2 *)(uVar11 + 0xfa) = (int)puVar10;
    *(undefined2 *)(uVar11 + 0xfc) = uVar5;
    *(undefined2 *)(uVar11 + 0xe0) = *(undefined2 *)(uVar11 + 0xfa);
    *(undefined2 *)(uVar11 + 0xe2) = uVar5;
    if((uVar12 | uVar11) == 0x0)
    {
        uVar2 = 0x0;
        uVar8 = 0x0;
    }
    else
    {
        uVar2 = uVar11 + 0xf2;
        uVar8 = uVar12;
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar11 + 0xfa) + 0x4);
    (**ppcVar1)(0x1010, *(undefined4 *)(uVar11 + 0xfa), 0x0, uVar2, uVar8);
    puVar7 = extraout_DX;
    pass1_1018_1a8e(*(ulong *)(uVar11 + 0xfa), extraout_DX, unaff_DI, unaff_SS);
    mem_op_1000_179c(0x20, puVar7, 0x1000);
    puVar6 = (uchar *)((uint)puVar7 | uVar2);
    if(puVar6 == (uchar *)0x0)
    {
        *(undefined4 *)(uVar11 + 0xf6) = 0x0;
    }
    else
    {
        unk_draw_op_1020_3da4((astruct_24 *)CONCAT22(puVar7, uVar2), (ULONG)param_1);
        *(uint *)(uVar11 + 0xf6)   = uVar2;
        *(uchar **)(uVar11 + 0xf8) = extraout_DX_00;
        puVar6                     = extraout_DX_00;
    }
    uVar4                          = *(undefined4 *)(uVar11 + 0xf6);
    *(undefined4 *)(uVar11 + 0xe8) = uVar4;
    mem_op_1000_179c(0x42, puVar6, 0x1000);
    paVar3 = (astruct_160 *)uVar4;
    puVar7 = (uchar *)((uint)puVar6 | (uint)paVar3);
    if(puVar7 == (uchar *)0x0)
    {
        *(undefined4 *)(uVar11 + 0x102) = 0x0;
    }
    else
    {
        pass1_1008_3bd6(paVar3, (ushort)puVar6, 0x0, 0xf1, 0x0, 0x1ac01ad, CONCAT22(*(undefined2 *)(uVar11 + 0x8), 0xf4), (ushort)puVar7, unaff_SS);
        *(astruct_160 **)(uVar11 + 0x102) = paVar3;
        *(uchar **)(uVar11 + 0x104)       = puVar7;
    }
    HVar9 = 0x1000;
    mem_op_1000_179c(0x42, puVar7, 0x1000);
    uVar8 = (uint)puVar7 | (uint)paVar3;
    if(uVar8 == 0x0)
    {
        *(undefined4 *)(uVar11 + 0x106) = 0x0;
    }
    else
    {
        HVar9 = 0x1008;
        pass1_1008_3bd6(paVar3, (ushort)puVar7, 0x0, 0xf500f1, 0x0, 0x1ae01af, CONCAT22(*(undefined2 *)(uVar11 + 0x8), 0xf5), uVar8, unaff_SS);
        *(astruct_160 **)(uVar11 + 0x106) = paVar3;
        *(uint *)(uVar11 + 0x108)         = uVar8;
    }
    uVar4   = *(undefined4 *)(uVar11 + 0xfa);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar11 + 0xfa) + 0x10);
    (**ppcVar1)(HVar9, (int)uVar4, (int)((ulong)uVar4 >> 0x10));
    puVar7 = (uchar *)paVar3->field_0x2;
    uVar8  = MoveWindow16(HVar9, 0x1, *(INT16 *)&paVar3->field_0x6, *(INT16 *)&paVar3->field_0x4, (INT16)puVar7, *(BOOL16 *)paVar3);
    HVar9  = 0x1000;
    mem_op_1000_179c(0x8e, puVar7, 0x1000);
    uVar2 = (uint)puVar7 | uVar8;
    if(uVar2 == 0x0)
    {
        *(undefined4 *)(uVar11 + 0x10a) = 0x0;
    }
    else
    {
        HVar9 = (HWND16)&PTR_LOOP_1050_1040;
        get_sys_metrics_1040_7728((astruct_57 *)CONCAT22(puVar7, uVar8), 0x1, 0x0, 0xfc0, *(ushort *)(uVar11 + 0x8));
        *(uint *)(uVar11 + 0x10a) = uVar8;
        *(uint *)(uVar11 + 0x10c) = uVar2;
    }
    uVar4                              = *(undefined4 *)(uVar11 + 0x10a);
    *(undefined2 *)((int)uVar4 + 0x74) = 0x1;
    uVar4                              = *(undefined4 *)(uVar11 + 0x10a);
    ppcVar1                            = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar11 + 0x10a) + 0x8);
    (**ppcVar1)(HVar9, (int)uVar4, (char)((ulong)uVar4 >> 0x10));
    if((*(uint *)(uVar11 + 0x10c) | *(uint *)(uVar11 + 0x10a)) != 0x0)
    {
        GetWindowRect16(HVar9, &local_a);
        GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_12);
        iStack12   = iStack12 - local_12.y;
        iStack14   = iStack6 - local_a.x;
        local_12.x = local_a.x;
        local_12.y = iStack4 + 0x3;
        SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x44, iStack12, iStack14, local_12.y, local_a.x, 0x0);
    }
    return;
}
