
void __stdcall16far realize_palette_1020_2992(ULONG param_1, int param_2)

{
    code      **ppcVar1;
    undefined4 *puVar2;

    if(param_2 != 0x0)
    {
        puVar2  = (undefined4 *)pass1_1018_0a50(*(ulong *)((int)param_1 + 0xf2));
        ppcVar1 = (code **)((int)*puVar2 + 0x18);
        (**ppcVar1)(0x1018, (int)puVar2, (int)((ulong)puVar2 >> 0x10));
        UnrealizeObject16(0x1018);
        GetDC16((HWND16)s_tile2_bmp_1050_1538);
        RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    }
    return;
}

void __stdcall16far invalidate_rect_1020_2ae4(ulong *param_1, uint param_2, HWND16 param_3, ushort param_4)

{
    code      **ppcVar1;
    char        cVar2;
    int         iVar3;
    uchar      *in_DX;
    ushort      uVar4;
    ushort      uVar5;
    int         unaff_DI;
    ushort     *puVar6;
    ulong       uVar7;
    astruct_43 *paVar8;
    undefined2  uVar9;
    undefined2  uVar10;

    if(param_2 != 0x129)
    {
        uVar5 = (ushort)param_1;
        uVar9 = (undefined2)((ulong)param_1 >> 0x10);
        if(0x129 < param_2)
        {
            if(param_2 == 0x12a)
            {
                uVar9 = 0xf012;
            }
            else
            {
                if(param_2 == 0x12b)
                {
                    return;
                }
                if(param_2 == 0x12c)
                {
                    uVar9 = 0xf020;
                }
                else
                {
                    if(param_2 == 0x12d)
                    {
                        return;
                    }
                    if(param_2 != 0x12e)
                    {
                        return;
                    }
                    uVar9 = 0xf060;
                }
            }
            PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x112, uVar9));
            return;
        }
        if(param_2 == 0xfb)
        {
            puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_4, in_DX, unaff_DI);
            pass1_1010_375e((ulong)puVar6);
            ppcVar1 = (code **)((int)*param_1 + 0x14);
            (**ppcVar1)();
            uVar7 = pass1_1010_375e((ulong)puVar6);
            uVar4 = (ushort)(uVar7 >> 0x10);
            pass1_1018_181c(*(ulong *)(uVar5 + 0xf2), (char *)(uVar7 & 0xffff | (ulong)uVar4 << 0x10), (uchar)(uVar7 & 0xffff), uVar4);
            return;
        }
        if(param_2 < 0xfc)
        {
            cVar2 = (char)param_2;
            if(cVar2 == 'o')
            {
                paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1f8, param_4);
                WinHelp16(0x1010, (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0xa), 0x0, CONCAT22((int)paVar8, 0x1));
                return;
            }
            if(cVar2 == 'r')
            {
                iVar3  = uVar5 + 0xa;
                uVar10 = uVar9;
                puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_4, in_DX, unaff_DI);
                uVar4  = (ushort)((ulong)puVar6 >> 0x10);
                pass1_1010_3770((ulong)puVar6, (char *)CONCAT22(uVar10, iVar3), uVar4);
                pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar5 + 0x8), 0x3, uVar4, uVar5, (ushort)&PTR_LOOP_1050_1038, param_4);
                return;
            }
            if(cVar2 == 'u')
            {
                pass1_1018_0a76(*(ulong *)(uVar5 + 0xf2), param_4);
                InvalidateRect16(0x1018, (RECT16 *)0x0, 0x0);
                return;
            }
        }
    }
    return;
}

void __stdcall16far pass1_1020_0a52(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    ushort     uVar1;
    undefined2 uVar2;
    undefined4 uVar3;

    uVar2 = (undefined2)(param_1 >> 0x10);
    uVar1 = (ushort)param_1;
    unk_draw_op_1020_0c3e(*(undefined4 *)(uVar1 + 0xe2), param_3);
    if(*(int *)(uVar1 + 0xe6) == 0x0)
    {
        *(undefined2 *)(uVar1 + 0xe6)                    = 0x1;
        *(undefined2 *)((int)_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        uVar3                                            = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar1 + 0x8), 0x6, param_2, uVar1, (ushort)&PTR_LOOP_1050_1038, param_4);
        *(undefined2 *)(uVar1 + 0xe8)                    = (int)uVar3;
        *(undefined2 *)(uVar1 + 0xea)                    = (int)((ulong)uVar3 >> 0x10);
    }
    return;
}

void __stdcall16far unk_draw_op_1020_0c3e(undefined4 param_1, HWND16 param_2)

{
    undefined4   *puVar1;
    code        **ppcVar2;
    undefined4    uVar3;
    HDC16        *b_force_background;
    int           iVar4;
    int           iVar5;
    undefined2    uVar6;
    undefined2    uVar7;
    uint          uStack40;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    uVar6    = (undefined2)((ulong)param_1 >> 0x10);
    iVar4    = (int)param_1;
    local_24 = BeginPaint16(param_2, &local_22);
    uVar3    = *(undefined4 *)(iVar4 + 0x6);
    uVar7    = (undefined2)((ulong)uVar3 >> 0x10);
    iVar5    = (int)uVar3;
    puVar1   = (undefined4 *)*(undefined4 *)(iVar5 + 0xa);
    uStack40 = (uint)puVar1;
    if((*(uint *)(iVar5 + 0xc) | uStack40) != 0x0)
    {
        b_force_background = &local_24;
        uVar3              = *puVar1;
        ppcVar2            = (code **)((int)uVar3 + 0x8);
        (**ppcVar2)((int)s_tile2_bmp_1050_1538, uStack40, (int)((ulong)puVar1 >> 0x10), b_force_background);
        ppcVar2 = (code **)((int)uVar3 + 0x4);
        (**ppcVar2)((int)s_tile2_bmp_1050_1538, puVar1, *(undefined2 *)(iVar4 + 0xc), *(undefined2 *)(iVar4 + 0xa), &local_24);
        SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, (BOOL16)b_force_background);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

void __stdcall16far realize_palette_1020_0e46(ulong param_1, int param_2, HGDIOBJ16 param_3)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    int         iVar4;
    undefined2  uVar5;

    if(param_2 != 0x0)
    {
        uVar3   = *(undefined4 *)((int)param_1 + 0xf2);
        uVar5   = (undefined2)((ulong)uVar3 >> 0x10);
        iVar4   = (int)uVar3;
        puVar1  = (undefined4 *)*(undefined4 *)(iVar4 + 0x66);
        ppcVar2 = (code **)((int)*puVar1 + 0x18);
        (**ppcVar2)(param_3, (int)puVar1, *(undefined2 *)(iVar4 + 0x68));
        UnrealizeObject16(param_3);
        RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    }
    return;
}


void __stdcall16far pass1_1020_1022(ulong param_1, ushort param_2)

{
    draw_op_1020_15de(*(undefined4 *)((int)param_1 + 0xf6), param_2);
    return;
}

void __stdcall16far cleanup_ui_op_1020_1038(undefined4 param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;
    HICON16     unaff_CS;
    undefined2  uVar6;

    uVar5 = (undefined2)((ulong)param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar6 = *(undefined2 *)(iVar4 + 0xc2);
    DestroyIcon16(unaff_CS);
    *(undefined2 *)(iVar4 + 0xc2) = 0x0;
    *(undefined2 *)(iVar4 + 0x8)  = 0x0;
    puVar1                        = (undefined4 *)*(uint *)(iVar4 + 0xf6);
    uVar2                         = *(uint *)(iVar4 + 0xf8);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)((int)s_tile2_bmp_1050_1538, puVar1, uVar2, 0x1, uVar6);
    }
    *(undefined4 *)(iVar4 + 0xf6) = 0x0;
    pass1_1010_1dda(*(ulong *)(iVar4 + 0xf2));
    *(undefined4 *)(iVar4 + 0xf2) = 0x0;
    return;
}

void __stdcall16far invalidate_rect_1020_157c(ulong param_1, int param_2, HWND16 param_3)

{
    BOOL16 BVar1;
    RECT16 local_a;
    ushort uStack4;

    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x14) = 0x0;
        return;
    }
    if(param_2 == 0x2)
    {
        BVar1 = IsIconic16(param_3);
        if(BVar1 == 0x0)
        {
            GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_a);
            uStack4 = 0x9a;
            InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)0x0, (BOOL16)&local_a);
            return;
        }
    }
    return;
}


void __stdcall16far draw_op_1020_15de(ULONG param_1, HWND16 in_win_handle_2)

{
    ulong         uVar1;
    code        **ppcVar2;
    BOOL16        BVar3;
    uint          uVar4;
    int           iVar5;
    undefined2    uVar6;
    HWND16        hwnd;
    ushort        unaff_SS;
    ulong         uVar7;
    undefined2    uVar8;
    undefined2    uVar9;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    uVar6    = (undefined2)(param_1 >> 0x10);
    iVar5    = (int)param_1;
    uVar9    = *(undefined2 *)(iVar5 + 0x4);
    local_24 = BeginPaint16(in_win_handle_2, &local_22);
    uVar8    = *(undefined2 *)(iVar5 + 0x4);
    hwnd     = (HWND16)s_tile2_bmp_1050_1538;
    BVar3    = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
    if(BVar3 == 0x0)
    {
        hwnd  = 0x1010;
        uVar7 = pass1_1010_454a(*(ulong *)(iVar5 + 0x14));
        uVar4 = (uint)(uVar7 >> 0x10);
        if((uVar4 | (uint)uVar7) != 0x0)
        {
            uVar1 = *(ulong *)(iVar5 + 0x14);
            hwnd  = 0x1008;
            pass1_1008_4480(*(ulong *)(iVar5 + 0x18), (ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x76)), (astruct_76 *)(uVar7 & 0xffff | (ulong)uVar4 << 0x10), unaff_SS);
        }
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x18) + 0x4);
        (**ppcVar2)(hwnd, *(undefined4 *)(iVar5 + 0x18), 0x0, &local_24, unaff_SS, uVar8, uVar9);
    }
    else
    {
        draw_op_1020_1674(param_1, (int)s_tile2_bmp_1050_1538);
    }
    EndPaint16(hwnd, &local_22);
    return;
}


void __stdcall16far draw_op_1020_1674(ULONG param_1, INT16 param_2)

{
    code     **ppcVar1;
    undefined2 uVar2;
    undefined2 local_1a;
    undefined2 uStack24;
    int        iStack22;
    int        iStack20;
    int        iStack18;
    int        iStack16;
    RECT16     local_e;
    int        iStack10;
    int        iStack8;
    RECT16    *pRStack6;
    int        iStack4;

    if(PTR_LOOP_1050_0010 == (undefined *)0x0)
    {
        uVar2   = (undefined2)(param_1 >> 0x10);
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x14) + 0x2c);
        iStack4 = (**ppcVar1)(param_2, *(undefined4 *)((int)param_1 + 0x14));
        if(iStack4 != 0x0)
        {
            pRStack6 = (RECT16 *)GetStockObject16(param_2);
            GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_e);
            local_1a = 0x0;
            uStack24 = 0x0;
            iStack22 = (iStack10 - local_e.x) + -0x1;
            iStack20 = (iStack8 - local_e.y) + -0x1;
            iStack18 = iStack20;
            iStack16 = iStack22;
            FillRect16((HDC16)s_tile2_bmp_1050_1538, pRStack6, (HBRUSH16)&local_1a);
            DrawIcon16((HDC16)s_tile2_bmp_1050_1538, iStack4, 0x2, 0x2);
        }
    }
    return;
}

void __stdcall16far pass1_1018_e5dc(ushort param_1, astruct_20 *param_2, ushort param_3, ushort param_4)

{
    uchar      *extraout_DX;
    undefined2  uVar1;
    astruct_20 *iVar2;
    int         unaff_DI;
    undefined2  uVar2;
    ushort     *puVar3;

    unk_draw_op_1020_7f7a(param_2, 0x9, CONCAT22(param_4, param_3));
    uVar2                                              = (undefined2)((ulong)param_2 >> 0x10);
    iVar2                                              = (astruct_20 *)param_2;
    *(undefined4 *)&iVar2[0x1].field_0xc               = 0x0;
    iVar2[0x1].field_0x10                              = 0x0;
    param_2->field_0x0                                 = 0xe790;
    iVar2->field_0x2                                   = 0x1018;
    ((astruct_20 *)(iVar2 + 0x1))->field_0x0           = 0xe82c;
    iVar2[0x1].field_0x2                               = 0x1018;
    puVar3                                             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xa, param_1, extraout_DX, unaff_DI);
    uVar1                                              = (undefined2)((ulong)puVar3 >> 0x10);
    *(int *)&iVar2[0x1].field_0x10                     = (int)puVar3;
    *(undefined2 *)((int)&iVar2[0x1].field_0x10 + 0x2) = uVar1;
    *(undefined2 *)&iVar2[0x1].field_0x4               = *(undefined2 *)&iVar2[0x1].field_0x10;
    *(undefined2 *)((int)&iVar2[0x1].field_0x4 + 0x2)  = uVar1;
    return;
}

void __stdcall16far pass1_1018_e834(astruct_660 *param_1, ushort param_2, ushort param_3, int param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined4 uVar2;
    int        iVar3;
    uchar     *extraout_DX;
    undefined2 uVar4;
    ushort    *puVar5;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0x14       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xe912;
    param_1->field_0x2                        = 0x1018;
    puVar5                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xa, param_5, extraout_DX, param_4);
    uVar4                                     = (undefined2)((ulong)puVar5 >> 0x10);
    param_1->field_0x14                       = (int)puVar5;
    param_1->field_0x16                       = uVar4;
    param_1->field_0x6                        = param_1->field_0x14;
    param_1->field_0x8                        = uVar4;
    uVar2                                     = *(undefined4 *)&param_1->field_0x14;
    iVar3                                     = &param_1->field_0xa;
    ppcVar1                                   = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1->field_0x12 = iVar3;
    draw_op_1020_9364((astruct_7 *)CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}

void __stdcall16far pass1_1018_e8bc(ushort *param_1)

{
    astruct_577 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_577 *)param_1;
    *param_1         = 0xe912;
    iVar1->field_0x2 = 0x1018;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

void __stdcall16far pass1_1018_e91e(astruct_20 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    ulong       *puVar1;
    code       **ppcVar2;
    undefined2  *puVar3;
    uchar       *extraout_DX;
    uchar       *puVar4;
    undefined2   uVar5;
    int          unaff_DI;
    ushort      *puVar6;
    undefined2   uVar7;
    astruct_127 *iVar7;

    iVar7 = (astruct_127 *)param_1;
    uVar7 = (undefined2)((ulong)param_1 >> 0x10);
    unk_draw_op_1020_7f7a(param_1, 0x3, CONCAT22(param_3, param_2));
    iVar7->field_0xee                              = 0x0;
    *(undefined4 *)&iVar7->field_0xf2              = 0x0;
    iVar7->field_0xf6                              = (ulong *)0x0;
    param_1->field_0x0                             = 0xebd0;
    iVar7->field_0x2                               = 0x1018;
    iVar7->field_0xe2                              = 0xec6c;
    iVar7->field_0xe4                              = 0x1018;
    puVar6                                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x28, param_4, extraout_DX, unaff_DI);
    puVar4                                         = (uchar *)((ulong)puVar6 >> 0x10);
    iVar7->field_0xf2                              = (int)puVar6;
    iVar7->field_0xf4                              = puVar4;
    iVar7->field_0xe6                              = iVar7->field_0xf2;
    iVar7->field_0xe8                              = puVar4;
    puVar6                                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_4, puVar4, unaff_DI);
    *(int *)&iVar7->field_0xf6                     = (int)puVar6;
    *(undefined2 *)((int)&iVar7->field_0xf6 + 0x2) = (int)((ulong)puVar6 >> 0x10);
    if(param_1 == (astruct_20 *)0x0)
    {
        puVar3 = (undefined2 *)0x0;
        uVar5  = 0x0;
    }
    else
    {
        puVar3 = &iVar7->field_0xe2;
        uVar5  = uVar7;
    }
    puVar1  = iVar7->field_0xf6;
    ppcVar2 = (code **)((int)*iVar7->field_0xf6 + 0x4);
    (**ppcVar2)(0x1010, (int)puVar1, (int)((ulong)puVar1 >> 0x10), 0xb, puVar3, uVar5);
    return;
}

void __stdcall16far pass1_1018_ec74(astruct_661 *param_1, int param_2, ushort param_3, ushort param_4)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    undefined4   uVar3;
    undefined2   uVar4;
    uchar       *extraout_DX;
    uchar       *puVar5;
    ushort      *puVar6;
    ulong        uVar7;
    undefined2   uVar8;
    undefined2   uVar9;
    astruct_661 *paVar10;
    int          iVar11;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0x14 = (undefined4 *)0x0;
    pass1_1008_3e38((ushort *)CONCAT22(param_2, &param_1->field_0x18));
    puVar6                                           = pass1_1008_3e38((ushort *)CONCAT22(param_2, &param_1->field_0x1e));
    *(undefined4 *)&param_1->field_0x24              = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1)        = 0x1cc;
    param_1->field_0x2                               = 0x1020;
    puVar6                                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x28, param_4, (uchar *)((ulong)puVar6 >> 0x10), param_2);
    uVar4                                            = (undefined2)((ulong)puVar6 >> 0x10);
    *(int *)&param_1->field_0x14                     = (int)puVar6;
    *(undefined2 *)((int)&param_1->field_0x14 + 0x2) = uVar4;
    uVar9                                            = 0x0;
    uVar8                                            = *(undefined2 *)&param_1->field_0x14;
    ppcVar2                                          = (code **)((int)*param_1->field_0x14 + 0x4);
    paVar10                                          = param_1;
    iVar11                                           = param_2;
    (**ppcVar2)();
    param_1->field_0x6 = param_1->field_0x14;
    puVar1             = param_1->field_0x14;
    pass1_1010_2b50((ushort)puVar1, (ushort)((ulong)puVar1 >> 0x10), (ushort *)CONCAT22(param_2, &param_1->field_0x18));
    uVar7               = pass1_1010_2b66((ulong)param_1->field_0x14);
    param_1->field_0x24 = (int)uVar7;
    param_1->field_0x26 = (int)(uVar7 >> 0x10);
    puVar1              = param_1->field_0x14;
    puVar1              = (undefined4 *)*(undefined4 *)((int)puVar1 + 0xa);
    uVar3               = CONCAT22(param_2, &param_1->field_0xa);
    ppcVar2             = (code **)((int)*puVar1 + 0x8);
    (**ppcVar2)(0x1010, (int)puVar1, (int)((ulong)puVar1 >> 0x10), uVar3, uVar8, uVar4, uVar9, paVar10, iVar11);
    param_1->field_0x12 = (int)uVar3;
    puVar5              = extraout_DX;
    draw_op_1020_9364((astruct_7 *)CONCAT22(param_2, param_1), 0x1020, param_4);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, puVar5, param_2);
    pass1_1008_3f62((ushort *)CONCAT22(param_2, &param_1->field_0x1e), (ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)));
    pass1_1008_3f32((int *)CONCAT22(param_2, &param_1->field_0x18), (int *)CONCAT22(param_2, &param_1->field_0x1e));
    return;
}

void __stdcall16far pass1_1018_ed98(ushort *param_1, ushort param_2)

{
    astruct_579 *iVar1;
    uint         uVar1;

    uVar1            = (uint)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_579 *)param_1;
    *param_1         = 0x1cc;
    iVar1->field_0x2 = 0x1020;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1ea6(iVar1->field_0x14, (ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10, param_2);
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

void __stdcall16far invalidate_rect_1018_edd8(ulong param_1, int param_2, ushort param_3)

{
    int        iVar1;
    undefined2 uVar2;
    ulong      uVar3;
    int        local_16;
    int        iStack20;
    int        iStack18;
    int        iStack16;
    ulong      uStack14;
    undefined2 uStack10;
    uint       uStack8;
    int        local_6;
    int        local_4;

    iVar1 = (int)param_1;
    uVar2 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        *(undefined4 *)(iVar1 + 0x14) = 0x0;
        return;
    }
    if(param_2 != 0xc)
    {
        return;
    }
    pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0x18)), (ushort *)CONCAT22(param_3, &local_6), (ushort *)CONCAT22(param_3, &local_4));
    uVar3    = pass1_1010_2b66(*(ulong *)(iVar1 + 0x14));
    uStack8  = (uint)(uVar3 >> 0x10);
    uStack10 = (undefined2)uVar3;
    uStack14 = pass1_1008_4772((astruct_76 *)(uVar3 & 0xffff | (ulong)uStack8 << 0x10));
    uVar2    = (undefined2)(uStack14 >> 0x10);
    local_16 = local_4;
    iStack20 = local_6;
    iStack18 = local_4 + *(int *)((int)uStack14 + 0x4);
    iStack16 = local_6 + *(int *)((int)uStack14 + 0x8);
    InvalidateRect16(0x1008, (RECT16 *)0x0, (BOOL16)&local_16);
    return;
}

void __stdcall16far unk_draw_op_1020_0000(ulong param_1, HWND16 param_2, ushort param_3)

{
    code        **ppcVar1;
    undefined4    uVar2;
    int           iVar3;
    int           iVar4;
    int           iVar5;
    undefined2    uVar6;
    HWND16        hwnd;
    undefined2    uVar7;
    undefined     local_c4[0x6];
    undefined     local_be[0x2];
    int           local_b4;
    int           iStack178;
    int           aiStack176[0x3c];
    int           iStack56;
    int           iStack48;
    astruct_76   *paStack46;
    int           local_2a;
    int           local_28;
    undefined4   *puStack38;
    PAINTSTRUCT16 local_22;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    uVar7 = *(undefined2 *)(iVar5 + 0x4);
    BeginPaint16(param_2, &local_22);
    uVar2     = *(undefined4 *)(iVar5 + 0x14);
    puStack38 = (undefined4 *)*(undefined4 *)((int)uVar2 + 0xa);
    pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar5 + 0x18U)), (ushort *)CONCAT22(param_3, &local_2a), (ushort *)CONCAT22(param_3, &local_28));
    hwnd = 0x1008;
    pass1_1008_4480((ulong)puStack38, (ushort *)(param_1 & 0xffff0000 | (ulong)(iVar5 + 0x18U)), *(astruct_76 **)(iVar5 + 0x24), param_3);
    paStack46 = (astruct_76 *)0x0;
    for(iStack48 = 0x0; iStack48 < 0x6; iStack48 = iStack48 + 0x1)
    {
        uVar2 = *(undefined4 *)(iVar5 + 0x14);
        hwnd  = 0x1010;
        pass1_1010_2b78((ushort)uVar2, (ushort)((ulong)uVar2 >> 0x10), iStack48, CONCAT22(param_3, &local_b4));
        if(local_b4 == 0x0)
        {
            for(iStack56 = 0x0; iVar4 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 0x1)
            {
                iVar3 = iStack56 * 0x3;
                if(aiStack176[iStack56 * 0x3 + 0x2] != 0x0)
                {
                    paStack46 = (astruct_76 *)pass1_1010_2b98(*(ulong *)(iVar5 + 0x14), aiStack176[iStack56 * 0x3 + 0x2]);
                    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_be), 0x0, aiStack176[iVar4 * 0x3 + 0x1] + local_2a, aiStack176[iVar3] + local_28);
                    hwnd = 0x1008;
                    pass1_1008_4480((ulong)puStack38, (ushort *)CONCAT22(param_3, local_be), paStack46, param_3);
                }
            }
        }
        else
        {
            _local_be = (int *)CONCAT22(param_3, aiStack176 + iStack178 * 0x3);
            if(aiStack176[iStack178 * 0x3 + 0x2] != 0x0)
            {
                paStack46 = (astruct_76 *)pass1_1010_2b98(*(ulong *)(iVar5 + 0x14), aiStack176[iStack178 * 0x3 + 0x2]);
                pass1_1008_3e54((ushort *)CONCAT22(param_3, local_c4), 0x0, *(int *)((int)_local_be + 0x2) + local_2a, *_local_be + local_28);
                hwnd = 0x1008;
                pass1_1008_4480((ulong)puStack38, (ushort *)CONCAT22(param_3, local_c4), paStack46, param_3);
            }
        }
    }
    ppcVar1 = (code **)((int)*puStack38 + 0x4);
    (**ppcVar1)(hwnd, (int)puStack38, (int)((ulong)puStack38 >> 0x10), 0x0, 0x0, iVar5 + 0xa, uVar6, uVar7);
    EndPaint16(hwnd, &local_22);
    return;
}

void __stdcall16far pass1_1020_01d8(astruct_20 *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7, ulong param_8, ushort param_9)

{
    unk_draw_op_1008_61b2((astruct_20 *)CONCAT22(param_2, param_1), param_3, param_7, param_8, param_9);
    *(undefined4 *)(param_1 + 0x1)            = 0x0;
    param_1[0x1].field_0x4                    = 0x0;
    param_1[0x1].field_0x8                    = param_6;
    param_1[0x1].field_0xa                    = param_5;
    param_1[0x1].field_0xc                    = param_4;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x45a;
    param_1->field_0x2                        = 0x1020;
    return;
}

void __stdcall16far draw_op_1020_041e(ulong param_1, ushort param_2)

{
    fill_rect_1020_065e(*(undefined4 *)((int)param_1 + 0xe6), param_2);
    return;
}

void __stdcall16far fill_rect_1020_065e(undefined4 param_1, HWND16 in_win_handle_2)

{
    code        **ppcVar1;
    undefined4    uVar2;
    int           iVar3;
    undefined2    uVar4;
    undefined2    local_brush_handle;
    undefined2    uStack50;
    int           iStack48;
    int           iStack46;
    RECT16       *local_rect_1;
    HDC16        *pHStack42;
    undefined4   *puStack40;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    uVar4    = (undefined2)((ulong)param_1 >> 0x10);
    iVar3    = (int)param_1;
    local_24 = BeginPaint16(in_win_handle_2, &local_22);
    if(0x280 < *(int *)(iVar3 + 0xa))
    {
        local_rect_1       = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
        local_brush_handle = 0x0;
        uStack50           = 0x0;
        iStack48           = *(int *)(iVar3 + 0xa) + -0x1;
        iStack46           = *(int *)(iVar3 + 0xc) + -0x1;
        FillRect16((HDC16)s_tile2_bmp_1050_1538, local_rect_1, (HBRUSH16)&local_brush_handle);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
    uVar2     = *(undefined4 *)(iVar3 + 0x6);
    puStack40 = (undefined4 *)*(undefined4 *)((int)uVar2 + 0xe);
    pHStack42 = &local_24;
    uVar2     = *puStack40;
    ppcVar1   = (code **)((int)uVar2 + 0x8);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538, (int)puStack40, (int)((ulong)puStack40 >> 0x10), pHStack42);
    ppcVar1 = (code **)((int)uVar2 + 0x4);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538, puStack40, *(undefined2 *)(iVar3 + 0x10), *(undefined2 *)(iVar3 + 0xe), &local_24);
    pHStack42 = (HDC16 *)SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, (BOOL16)pHStack42);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

void __stdcall16far pass1_1020_07aa(ulong param_1, ushort param_2, int param_3, ushort param_4, ushort param_5)

{
    int        iVar1;
    undefined2 uVar2;
    undefined  local_16[0x14];

    draw_op_1020_041e(param_1, param_4);
    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0xf0) == 0x0)
    {
        *(undefined2 *)(iVar1 + 0xf0) = 0x1;
        pass1_1008_5bdc((astruct_79 *)CONCAT22(param_5, local_16), param_3, param_5);
        win_1008_5c9e(CONCAT22(param_5, local_16), (ulong *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0xf2)), local_16, param_2, param_5);
        pass1_1008_5c34((ushort *)CONCAT22(param_5, local_16));
    }
    return;
}

void __stdcall16far pass1_1018_dfd4(undefined4 param_1, uchar *param_2, int param_3, ushort param_4, ushort param_5)

{
    ushort     uVar1;
    undefined2 uVar2;
    ushort    *puVar3;

    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    uVar1 = (ushort)param_1;
    delete_palette_1018_e16c(*(ulong *)(uVar1 + 0xe2), param_4);
    if(*(int *)(uVar1 + 0xe6) == 0x0)
    {
        *(undefined2 *)(uVar1 + 0xe6) = 0x1;
        puVar3                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x36, param_5, param_2, param_3);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar1 + 0x8), 0x8, (ushort)((ulong)puVar3 >> 0x10), uVar1, (ushort)&PTR_LOOP_1050_1038, param_5);
    }
    return;
}

void __stdcall16far delete_palette_1018_e16c(ulong param_1, HWND16 param_2)

{
    undefined4   *puVar1;
    code        **ppcVar2;
    undefined4    uVar3;
    HDC16        *b_force_background;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    local_24           = BeginPaint16(param_2, &local_22);
    uVar3              = *(undefined4 *)((int)param_1 + 0x6);
    puVar1             = (undefined4 *)*(undefined4 *)((int)uVar3 + 0xa);
    b_force_background = &local_24;
    uVar3              = *puVar1;
    ppcVar2            = (code **)((int)uVar3 + 0x8);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)puVar1, (int)((ulong)puVar1 >> 0x10), b_force_background);
    ppcVar2 = (code **)((int)uVar3 + 0x4);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, puVar1, 0x0, &local_24);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, (BOOL16)b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

void __stdcall16far pass1_1018_e230(ushort param_1, astruct_20 *param_2, ushort param_3, ushort param_4)

{
    uchar       *extraout_DX;
    undefined2   uVar1;
    astruct_128 *iVar2;
    int          unaff_DI;
    undefined2   uVar2;
    ushort      *puVar3;

    unk_draw_op_1020_7f7a(param_2, 0x4, CONCAT22(param_4, param_3));
    uVar2                             = (undefined2)((ulong)param_2 >> 0x10);
    iVar2                             = (astruct_128 *)param_2;
    iVar2->field_0xee                 = 0x0;
    *(undefined4 *)&iVar2->field_0xf2 = 0x0;
    param_2->field_0x0                = 0xe44e;
    iVar2->field_0x2                  = 0x1018;
    iVar2->field_0xe2                 = 0xe4ea;
    iVar2->field_0xe4                 = 0x1018;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x26, param_1, extraout_DX, unaff_DI);
    uVar1                             = (undefined2)((ulong)puVar3 >> 0x10);
    iVar2->field_0xf2                 = (int)puVar3;
    iVar2->field_0xf4                 = uVar1;
    iVar2->field_0xe6                 = iVar2->field_0xf2;
    iVar2->field_0xe8                 = uVar1;
    return;
}

void __stdcall16far pass1_1018_e4f2(astruct_659 *param_1, ushort param_2, ushort param_3, int param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined4 uVar2;
    int        iVar3;
    uchar     *extraout_DX;
    undefined2 uVar4;
    ushort    *puVar5;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0x14       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xe5d0;
    param_1->field_0x2                        = 0x1018;
    puVar5                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x26, param_5, extraout_DX, param_4);
    uVar4                                     = (undefined2)((ulong)puVar5 >> 0x10);
    param_1->field_0x14                       = (int)puVar5;
    param_1->field_0x16                       = uVar4;
    param_1->field_0x6                        = param_1->field_0x14;
    param_1->field_0x8                        = uVar4;
    uVar2                                     = *(undefined4 *)&param_1->field_0x14;
    iVar3                                     = &param_1->field_0xa;
    ppcVar1                                   = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1->field_0x12 = iVar3;
    draw_op_1020_9364((astruct_7 *)CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}


void __stdcall16far pass1_1018_e57a(ushort *param_1)

{
    astruct_575 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_575 *)param_1;
    *param_1         = 0xe5d0;
    iVar1->field_0x2 = 0x1018;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

void __stdcall16far unk_draw_op_1018_c578(astruct_36 *param_1, ushort param_2)

{
    uint          uVar1;
    int           iVar2;
    int           iVar3;
    astruct_76   *paVar4;
    code        **ppcVar5;
    undefined4    uVar6;
    ushort        uVar7;
    HDC16        *b_force_background;
    int           iVar8;
    uchar        *in_DX;
    ushort        uVar9;
    undefined2    uVar10;
    undefined2    extraout_DX;
    int           iVar11;
    undefined2    uVar12;
    int           unaff_DI;
    undefined2    uVar13;
    undefined2    uVar14;
    HWND16        hwnd;
    ulong         uVar15;
    HDC16        *pHVar16;
    RECT16       *pRVar18;
    undefined4    uVar17;
    HDC16         HVar19;
    undefined4    local_34;
    int           iStack48;
    int           iStack46;
    RECT16       *pRStack44;
    HDC16         local_2a;
    ushort        uStack40;
    ushort       *puStack38;
    PAINTSTRUCT16 local_22;

    hwnd      = 0x1010;
    puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uVar9     = (ushort)((ulong)puStack38 >> 0x10);
    uVar7     = *(ushort *)((int)puStack38 + 0x20);
    iVar11    = (int)param_1;
    uVar13    = (undefined2)((ulong)param_1 >> 0x10);
    uStack40  = uVar7;
    if(uVar7 == 0x0)
    {
        BeginPaint16(0x1010, &local_22);
        EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
        PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, CONCAT22(0x111, *(undefined2 *)(iVar11 + 0xea)));
        return;
    }
    if((*(int *)(iVar11 + 0xf0) == 0x0) && (*(int *)(iVar11 + 0xf4) != 0x0))
    {
        *(undefined2 *)(iVar11 + 0xf0) = 0x1;
        uVar1                          = iVar11 + 0xf2;
        hwnd                           = 0x1008;
        win_1008_5c9e(_PTR_LOOP_1050_02a0, (ulong *)((ulong)param_1 & 0xffff0000 | (ulong)uVar1), uVar1, uVar9, param_2);
        uVar7 = uVar1;
    }
    if(*(int *)((int)_PTR_LOOP_1050_02a0 + 0x12) == 0x0)
    {
        hwnd = 0x1008;
        win_1008_5c5c((WNDCLASS16 *)param_2, uVar7, uVar9, _PTR_LOOP_1050_02a0, 0x1d3);
    }
    local_2a  = BeginPaint16(hwnd, &local_22);
    pRStack44 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    local_34  = 0x0;
    iStack48  = *(int *)(iVar11 + 0xf6) + -0x1;
    iStack46  = *(int *)(iVar11 + 0xf8) + -0x1;
    uVar7     = param_2;
    HVar19    = local_2a;
    FillRect16((HDC16)s_tile2_bmp_1050_1538, pRStack44, (HBRUSH16)&local_34);
    pRVar18 = pRStack44;
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    uVar6              = *(undefined4 *)(iVar11 + 0xe2);
    paVar4             = *(astruct_76 **)((int)uVar6 + 0xe);
    b_force_background = &local_2a;
    uVar17             = CONCAT22(pRVar18, param_2);
    uVar14             = (undefined2)((ulong)paVar4 >> 0x10);
    uVar12             = SUB42(paVar4, 0x0);
    uVar6              = *(undefined4 *)paVar4;
    ppcVar5            = (code **)((int)uVar6 + 0x8);
    pHVar16            = b_force_background;
    (**ppcVar5)();
    uVar15                   = pass1_1008_4772(paVar4);
    uVar10                   = (undefined2)(uVar15 >> 0x10);
    iVar2                    = *(int *)((int)uVar15 + 0x4);
    iVar3                    = *(int *)((int)uVar15 + 0x8);
    iVar8                    = (0x1e0 - iVar3) / 0x2;
    *(int *)(iVar11 + 0x10c) = iVar8 + iVar3 + *(int *)(iVar11 + 0x110);
    ppcVar5                  = (code **)((int)uVar6 + 0x4);
    (**ppcVar5)(0x1008, paVar4, *(int *)(iVar11 + 0xfc) + *(int *)(iVar11 + 0xfe) + iVar8, *(int *)(iVar11 + 0xfa) + (0x280 - iVar2) / 0x2, &local_2a, param_2, uVar12, uVar14, pHVar16, uVar17, uVar7, HVar19);
    draw_text_1018_c742(param_1, 0x1008, param_2, extraout_DX);
    SelectPalette16(0x1008, 0x0, (BOOL16)b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}


void __stdcall16far draw_text_1018_c742(astruct_36 *param_1, HDC16 param_2, RECT16 *param_3, undefined2 param_4)

{
    int        *piVar1;
    COLORREF    CVar2;
    int         iVar3;
    astruct_36 *iVar4;
    astruct_36 *uVar4;
    undefined2  local_1a;
    undefined2  uStack24;
    int         iStack22;
    int         iStack20;
    int         local_12;
    int         iStack16;
    int         iStack14;
    int         iStack12;
    COLORREF    CStack10;
    undefined2  uStack8;
    undefined4  uStack6;

    uVar4 = (astruct_36 *)((ulong)param_1 >> 0x10);
    iVar4 = (astruct_36 *)param_1;
    if((iVar4->field_0x108 != (char *)0x0) && (*iVar4->field_0x108 != '\0'))
    {
        CVar2    = SetTextColor16(param_2, 0x8000);
        uStack6  = CONCAT22(param_4, CVar2);
        CStack10 = SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
        uStack8  = param_4;
        if(iVar4->field_0x106 == 0x0)
        {
            local_1a = 0x0;
            uStack24 = 0x0;
            iStack22 = iVar4->field_0x10e;
            iStack20 = 0x0;
            DrawText16((HDC16)s_tile2_bmp_1050_1538, (LPCSTR)0x410, (INT16)&local_1a, param_3, 0xffff);
            iVar4->field_0x100 = (0x280 - iStack22) / 0x2;
            iVar4->field_0x102 = iVar4->field_0x10c;
            iVar4->field_0x104 = iVar4->field_0x100 + iStack22;
            iVar3              = iVar4->field_0x102 + iStack20;
            iVar4->field_0x106 = iVar3;
            piVar1             = &iVar4->field_0xf8;
            if(*piVar1 == iVar3 || *piVar1 < iVar3)
            {
                iVar3   = iVar3 - iVar4->field_0xf8;
                piVar1  = &iVar4->field_0x102;
                *piVar1 = *piVar1 - iVar3;
                piVar1  = &iVar4->field_0x106;
                *piVar1 = *piVar1 - iVar3;
            }
        }
        local_12 = iVar4->field_0xfa + iVar4->field_0x100;
        iStack16 = iVar4->field_0xfc + iVar4->field_0x102;
        iStack14 = iVar4->field_0xfa + iVar4->field_0x104;
        iStack12 = iVar4->field_0xfc + iVar4->field_0x106;
        DrawText16((HDC16)s_tile2_bmp_1050_1538, (LPCSTR)&PTR_LOOP_1050_0010, (INT16)&local_12, param_3, 0xffff);
        SetTextColor16((HDC16)s_tile2_bmp_1050_1538, (COLORREF)uStack6);
        SetBkColor16((HDC16)s_tile2_bmp_1050_1538, CStack10);
    }
    return;
}

void __stdcall16far pass1_1018_5b06(astruct_132 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    ulong       *puVar3;
    uint         uVar5;
    undefined4   uVar6;
    uchar       *puVar7;
    undefined2   uVar8;
    uchar       *puVar9;
    uint         uVar10;
    int          unaff_DI;
    ushort      *puVar11;
    astruct_76  *paVar12;
    ulong        uVar13;
    undefined2   uVar14;
    undefined2   uVar15;
    astruct_132 *paVar16;
    ushort       uVar17;
    undefined    local_c[0x6];
    ushort      *puStack6;
    uint         uVar4;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0x14                = (ulong *)0x0;
    param_1->field_0x18                = 0x0;
    puVar11                            = pass1_1008_3e38((ushort *)CONCAT22(param_2, &param_1->field_0x1c));
    *(int *)CONCAT22(param_2, param_1) = (int)&PTR_LOOP_1050_5e1a;
    param_1->field_0x2                 = 0x1018;
    puStack6                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, (uchar *)((ulong)puVar11 >> 0x10), unaff_DI);
    puVar11                            = pass1_1008_3e38((ushort *)CONCAT22(param_4, local_c));
    puVar7                             = (uchar *)((ulong)puVar11 >> 0x10);
    pass1_1008_3f62((ushort *)CONCAT22(param_4, local_c), (ushort *)((ulong)puStack6 & 0xffff0000 | (ulong)((int)puStack6 + 0xe)));
    puVar11                                          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x27, param_4, puVar7, unaff_DI);
    uVar8                                            = (undefined2)((ulong)puVar11 >> 0x10);
    *(int *)&param_1->field_0x14                     = (int)puVar11;
    *(undefined2 *)((int)&param_1->field_0x14 + 0x2) = uVar8;
    uVar15                                           = 0x0;
    uVar14                                           = *(undefined2 *)&param_1->field_0x14;
    ppcVar2                                          = (code **)((int)*param_1->field_0x14 + 0x4);
    paVar16                                          = param_1;
    uVar17                                           = param_2;
    (**ppcVar2)();
    param_1->field_0x6 = param_1->field_0x14;
    puVar3             = param_1->field_0x14;
    puVar1             = (undefined4 *)*(ulong *)((int)puVar3 + 0xa);
    uVar6              = CONCAT22(param_2, &param_1->field_0xa);
    ppcVar2            = (code **)((int)*puVar1 + 0x8);
    (**ppcVar2)(0x1010, (int)puVar1, (int)((ulong)puVar1 >> 0x10), uVar6, uVar14, uVar8, uVar15, paVar16, uVar17);
    param_1->field_0x12 = (int)uVar6;
    draw_op_1020_9364((astruct_7 *)CONCAT22(param_2, param_1), 0x1020, param_4);
    puVar3 = param_1->field_0x14;
    pass1_1008_3f62((ushort *)CONCAT22(param_2, &param_1->field_0x1c), (ushort *)((ulong)puVar3 & 0xffff0000 | (ulong)((int)puVar3 + 0x52)));
    pass1_1008_3f32((int *)CONCAT22(param_2, &param_1->field_0x1c), (int *)CONCAT22(param_4, local_c));
    paVar12 = (astruct_76 *)pass1_1008_9f48((ulong)param_1->field_0x14);
    uVar13  = pass1_1008_4772(paVar12);
    puVar9  = (uchar *)(uVar13 >> 0x10);
    uVar4   = (uint)uVar13;
    puVar7  = puVar9;
    uVar5   = uVar4;
    mem_op_1000_179c(0x14, puVar9, 0x1000);
    uVar10 = (uint)puVar7 | uVar5;
    if(uVar10 == 0x0)
    {
        param_1->field_0x18 = 0x0;
    }
    else
    {
        pass1_1008_50c2((astruct_110 *)CONCAT22(puVar7, uVar5), *(ulong *)(uVar4 + 0x8), *(ulong *)(uVar4 + 0x4), (uint *)CONCAT22(param_2, &param_1->field_0x1c), (ulong)puVar1);
        *(uint *)&param_1->field_0x18              = uVar5;
        *(uint *)((int)&param_1->field_0x18 + 0x2) = uVar10;
    }
    pass1_1008_5134(param_1->field_0x18);
    param_1->field_0x22 = param_1->field_0x1c;
    param_1->field_0x24 = param_1->field_0x1e;
    param_1->field_0x26 = *(int *)(uVar4 + 0x4) + param_1->field_0x22 + 0x1;
    param_1->field_0x28 = *(int *)(uVar4 + 0x8) + param_1->field_0x24 + 0x1;
    return;
}

void __stdcall16far pass1_1018_5cc8(ushort *param_1, ushort param_2)

{
    uint         uVar1;
    astruct_18  *paVar2;
    astruct_508 *iVar3;
    uint         uVar3;

    uVar3            = (uint)((ulong)param_1 >> 0x10);
    iVar3            = (astruct_508 *)param_1;
    *param_1         = (ushort)&PTR_LOOP_1050_5e1a;
    iVar3->field_0x2 = 0x1018;
    paVar2           = *(astruct_18 **)&iVar3->field_0x18;
    uVar1            = iVar3->field_0x1a;
    if((uVar1 | (uint)paVar2) != 0x0)
    {
        pass1_1008_5118((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10);
        fn_ptr_1000_17ce(paVar2, 0x1000);
    }
    if(iVar3->field_0x14 != 0x0)
    {
        pass1_1010_1ea6(iVar3->field_0x14, (ulong)param_1 & 0xffff | (ulong)uVar3 << 0x10, param_2);
        pass1_1010_1dda(iVar3->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}


void __stdcall16far invalidate_rect_1018_5d32(ulong param_1, int param_2, HWND16 param_3)

{
    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x14) = 0x0;
        return;
    }
    if(param_2 != 0x2)
    {
        return;
    }
    InvalidateRect16(param_3, (RECT16 *)0x0, (int)param_1 + 0x22);
    return;
}


void __stdcall16far misc_draw_op_1018_5d6c(ulong param_1, HWND16 param_2)

{
    undefined4   *puVar1;
    code        **ppcVar2;
    undefined4    uVar3;
    int           iVar4;
    undefined2    uVar5;
    ushort        unaff_SS;
    astruct_76   *paVar6;
    undefined2    uVar7;
    PAINTSTRUCT16 local_22;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar7 = *(undefined2 *)(iVar4 + 0x4);
    BeginPaint16(param_2, &local_22);
    uVar3  = *(undefined4 *)(iVar4 + 0x14);
    puVar1 = (undefined4 *)*(ulong *)((int)uVar3 + 0xa);
    paVar6 = (astruct_76 *)pass1_1008_9f48(*(ulong *)(iVar4 + 0x14));
    pass1_1008_5236(*(ulong *)(iVar4 + 0x18));
    pass1_1008_4480((ulong)puVar1, (ushort *)(param_1 & 0xffff0000 | (ulong)(iVar4 + 0x1c)), paVar6, unaff_SS);
    ppcVar2 = (code **)((int)*puVar1 + 0x4);
    (**ppcVar2)(0x1008, (int)puVar1, (int)((ulong)puVar1 >> 0x10), 0x0, param_1 & 0xffff0000 | (ulong)(iVar4 + 0xa), uVar7);
    EndPaint16(0x1008, &local_22);
    return;
}

void __stdcall16far set_window_text_1018_6066(UINT16 param_1, UINT16 param_2, SEGPTR in_win_text_3, UINT16 param_4, INT16 dialog_id_5, HWND16 in_hwnd_6)

{
    GetDlgItem16(in_hwnd_6, dialog_id_5);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, in_win_text_3);
    return;
}


void __stdcall16far set_window_text_1018_6086(ULONG param_1, LPSTR param_2, WORD *param_3)

{
    wsprintf16(param_2, &stack0xfff4, param_3);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1be);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)&stack0xfff4);
    wsprintf16((LPSTR)s_tile2_bmp_1050_1538, &stack0xfff4, param_3);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1bf);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)&stack0xfff4);
    return;
}

void __stdcall16far unk_draw_op_1018_623e(ulong param_1, HWND16 param_2, ushort param_3)

{
    int          *piVar1;
    code        **ppcVar2;
    undefined4    uVar3;
    undefined4   *puVar4;
    ushort        uVar5;
    HDC16        *pHVar6;
    int           iVar7;
    HPEN16        handle;
    HGDIOBJ16     HVar8;
    HBRUSH16      handle_00;
    uchar        *puVar9;
    ushort        uVar10;
    int           iVar11;
    int           iVar12;
    undefined    *puVar13;
    undefined2    uVar14;
    undefined2    uVar15;
    ulong         uVar16;
    int           iVar17;
    undefined     uVar18;
    undefined     uVar19;
    undefined     local_38[0x6];
    undefined2    local_32;
    undefined2    uStack48;
    undefined4    uStack46;
    undefined2    uStack42;
    undefined4   *puStack40;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    puVar13   = &stack0xfffe;
    uVar14    = (undefined2)(param_1 >> 0x10);
    iVar11    = (int)param_1;
    uVar15    = *(undefined2 *)(iVar11 + 0x4);
    local_24  = BeginPaint16(param_2, &local_22);
    puStack40 = (undefined4 *)pass1_1010_4c2c(*(ulong *)(iVar11 + 0x6));
    pHVar6    = &local_24;
    ppcVar2   = (code **)((int)*puStack40 + 0x8);
    (**ppcVar2)(0x1010, (int)puStack40, (int)((ulong)puStack40 >> 0x10), pHVar6, param_3, uVar15);
    *(HDC16 **)(iVar11 + 0x10) = pHVar6;
    uVar3                      = *(undefined4 *)(iVar11 + 0x6);
    uStack42                   = *(undefined2 *)((int)uVar3 + 0x30);
    uVar3                      = *(undefined4 *)(iVar11 + 0x6);
    uStack46                   = *(undefined4 *)*(undefined4 *)((int)uVar3 + 0x12);
    uStack48                   = 0x14;
    local_32                   = 0x0;
    pass1_1008_3e38((ushort *)CONCAT22(param_3, local_38));
    while(*(int *)(puVar13 + -0x38) < *(int *)(puVar13 + -0x28))
    {
        iVar11                       = *(int *)(puVar13 + -0x38) * 0x4;
        uVar3                        = *(undefined4 *)(puVar13 + -0x2c);
        uVar16                       = pass1_1008_4772(*(astruct_76 **)(iVar11 + (int)uVar3));
        puVar9                       = (uchar *)(uVar16 >> 0x10);
        *(int *)(puVar13 + -0x44)    = (int)uVar16;
        *(uchar **)(puVar13 + -0x42) = puVar9;
        uVar3                        = *(undefined4 *)(puVar13 + 0x6);
        pass1_1018_642e((ushort)uVar3, (ushort)((ulong)uVar3 >> 0x10), (int *)CONCAT13((char)(param_3 >> 0x8), CONCAT12((char)param_3, puVar13 + -0x30)), *(int *)((int)uVar16 + 0x8));
        uVar3 = *(undefined4 *)(puVar13 + -0x30);
        pass1_1008_3e76((ushort *)CONCAT22(param_3, puVar13 + -0x36), 0x0, (ushort)uVar3, (ushort)((ulong)uVar3 >> 0x10));
        uVar3 = *(undefined4 *)(puVar13 + -0x2c);
        pass1_1008_4480(*(ulong *)(puVar13 + -0x26), (ushort *)CONCAT22(param_3, puVar13 + -0x36), *(astruct_76 **)((int)uVar3 + iVar11), param_3);
        iVar11 = *(int *)(puVar13 + -0x38);
        uVar3  = *(undefined4 *)(puVar13 + -0x30);
        uVar14 = (undefined2)uVar3;
        uVar18 = (undefined)((ulong)uVar3 >> 0x10);
        uVar19 = (undefined)((ulong)uVar3 >> 0x18);
        uVar3  = *(undefined4 *)(puVar13 + -0x44);
        uVar15 = (undefined2)((ulong)uVar3 >> 0x10);
        iVar12 = (int)uVar3;
        iVar7  = *(int *)(iVar12 + 0x4) + *(int *)(puVar13 + -0x2e);
        iVar12 = *(int *)(iVar12 + 0x8) + *(int *)(puVar13 + -0x30);
        uVar3  = *(undefined4 *)(puVar13 + 0x6);
        uVar3  = *(undefined4 *)((int)uVar3 + 0x6);
        iVar17 = (int)uVar3;
        uVar15 = (undefined2)((ulong)uVar3 >> 0x10);
        if(*(long *)(iVar17 + 0x1a) == 0x0)
        {
            uVar5 = *(int *)(iVar17 + 0x30) << 0x3;
            mem_op_1000_179c(uVar5, puVar9, 0x1000);
            *(ushort *)(iVar17 + 0x1a) = uVar5;
            *(uchar **)(iVar17 + 0x1c) = puVar9;
        }
        uVar3                                      = *(undefined4 *)(iVar17 + 0x1a);
        iVar11                                     = iVar11 * 0x8;
        *(undefined2 *)((int)uVar3 + iVar11)       = CONCAT11(uVar19, uVar18);
        uVar3                                      = *(undefined4 *)(iVar17 + 0x1a);
        *(undefined2 *)((int)uVar3 + iVar11 + 0x2) = uVar14;
        uVar3                                      = *(undefined4 *)(iVar17 + 0x1a);
        *(int *)((int)uVar3 + iVar11 + 0x4)        = iVar7;
        uVar3                                      = *(undefined4 *)(iVar17 + 0x1a);
        *(int *)((int)uVar3 + iVar11 + 0x6)        = iVar12;
        uVar3                                      = *(undefined4 *)(puVar13 + -0x44);
        piVar1                                     = (int *)(puVar13 + -0x2e);
        *piVar1                                    = *piVar1 + (-(uint)(*(int *)(puVar13 + -0x38) == 0x0) & 0x5) + 0x14 + *(int *)((int)uVar3 + 0x4);
        piVar1                                     = (int *)(puVar13 + -0x38);
        *piVar1                                    = *piVar1 + 0x1;
    }
    puVar4  = *(undefined4 **)(puVar13 + -0x26);
    ppcVar2 = (code **)((int)*puVar4 + 0x4);
    (**ppcVar2)(0x1008, (int)puVar4, (int)((ulong)puVar4 >> 0x10), 0x0, 0x0, (char)puVar13 + -0x22, param_3);
    handle                          = CreatePen16(0x1008, 0x25, 0x100);
    *(HPEN16 *)(puVar13 + -0x3a)    = handle;
    HVar8                           = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    *(HGDIOBJ16 *)(puVar13 + -0x3c) = HVar8;
    handle_00                       = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    *(HBRUSH16 *)(puVar13 + -0x3e)  = handle_00;
    HVar8                           = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
    *(HGDIOBJ16 *)(puVar13 + -0x40) = HVar8;
    draw_line_1018_6444(*(ulong *)(puVar13 + 0x6), (int)s_tile2_bmp_1050_1538);
    uVar3  = *(undefined4 *)(puVar13 + 0x6);
    uVar16 = pass1_1010_4dc8(*(ulong *)((int)uVar3 + 0x6));
    uVar10 = (ushort)(uVar16 >> 0x10);
    uVar5  = (ushort)uVar16;
    draw_op_1018_6544(*(ulong *)(puVar13 + 0x6), (int *)(uVar16 & 0xffff | (ulong)uVar10 << 0x10), param_3);
    pass1_1018_6630(*(ulong *)(puVar13 + 0x6), uVar5, uVar10);
    uVar3 = *(undefined4 *)(puVar13 + 0x6);
    SelectPalette16(0x1010, 0x0, *(BOOL16 *)((int)uVar3 + 0x10));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar13 + -0x3c));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar13 + -0x40));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, (PAINTSTRUCT16 *)(puVar13 + -0x20));
    return;
}

void __stdcall16far draw_line_1018_6444(ulong param_1, HDC16 param_2)

{
    int        iVar1;
    INT16     *pIVar2;
    undefined4 uVar3;
    int        iVar4;
    int        iVar5;
    int       *piVar6;
    undefined2 uVar7;
    int        iStack10;

    uVar7  = (undefined2)(param_1 >> 0x10);
    uVar3  = *(undefined4 *)((int)param_1 + 0x6);
    iVar1  = *(int *)((int)uVar3 + 0x30);
    uVar3  = *(undefined4 *)((int)param_1 + 0x6);
    pIVar2 = *(INT16 **)((int)uVar3 + 0x1a);
    MoveTo16(param_2, 0x5, *pIVar2);
    uVar7 = (undefined2)((ulong)pIVar2 >> 0x10);
    iVar5 = (int)pIVar2;
    LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x5, *(INT16 *)(iVar5 + iVar1 * 0x8 + -0x4));
    for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
    {
        piVar6 = (int *)(iStack10 * 0x8 + iVar5);
        iVar4  = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
        MoveTo16((HDC16)s_tile2_bmp_1050_1538, 0x5, iVar4);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, 0xa, iVar4);
    }
    MoveTo16((HDC16)s_tile2_bmp_1050_1538, 0x5f, *pIVar2);
    LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x5f, *(INT16 *)(iVar5 + iVar1 * 0x8 + -0x4));
    for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
    {
        piVar6 = (int *)(iStack10 * 0x8 + iVar5);
        iVar4  = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
        MoveTo16((HDC16)s_tile2_bmp_1050_1538, 0x5f, iVar4);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x5a, iVar4);
    }
    return;
}


void __stdcall16far draw_op_1018_6544(ulong param_1, int *param_2, ushort param_3)

{
    ushort   *puVar1;
    ulong     uVar2;
    ushort    uVar3;
    undefined local_a[0x6];
    ushort    uStack4;

    if(param_2 != (int *)0x0)
    {
        uStack4 = (*(int *)((int)param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
        puVar1  = pass1_1008_3e54((ushort *)CONCAT22(param_3, local_a), 0x0, 0x57, uStack4);
        uVar3   = (ushort)(param_1 >> 0x10);
        uVar2   = pass1_1018_659a((ushort)param_1, uVar3, (ushort *)CONCAT22(param_3, local_a), (uchar *)((ulong)puVar1 >> 0x10), param_3);
        draw_polygon_1018_661c((ushort)param_1, uVar3, CONCAT22((int)uVar2, 0x3), 0x1008);
    }
    return;
}

void __stdcall16far draw_polygon_1018_661c(ushort param_1, ushort param_2, ulong param_3, HDC16 param_4)

{
    Polygon16(param_4, (POINT16 *)param_3, (INT16)(param_3 >> 0x10));
    return;
}

void __stdcall16far struct_1018_66cc(astruct_20 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uchar      *extraout_DX;
    undefined2  uVar1;
    astruct_20 *iVar2;
    int         unaff_DI;
    astruct_20 *uVar2;
    ushort     *puVar2;

    unk_draw_op_1020_7f7a(param_1, 0xa, CONCAT22(param_3, param_2));
    uVar2                                              = (astruct_20 *)((ulong)param_1 >> 0x10);
    iVar2                                              = (astruct_20 *)param_1;
    *(undefined4 *)&iVar2[0x1].field_0xc               = 0x0;
    iVar2[0x1].field_0x10                              = 0x0;
    param_1->field_0x0                                 = 0x6880;
    iVar2->field_0x2                                   = 0x1018;
    ((astruct_20 *)(iVar2 + 0x1))->field_0x0           = 0x691c;
    iVar2[0x1].field_0x2                               = 0x1018;
    puVar2                                             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xb, param_4, extraout_DX, unaff_DI);
    uVar1                                              = (undefined2)((ulong)puVar2 >> 0x10);
    *(int *)&iVar2[0x1].field_0x10                     = (int)puVar2;
    *(undefined2 *)((int)&iVar2[0x1].field_0x10 + 0x2) = uVar1;
    *(undefined2 *)&iVar2[0x1].field_0x4               = *(undefined2 *)&iVar2[0x1].field_0x10;
    *(undefined2 *)((int)&iVar2[0x1].field_0x4 + 0x2)  = uVar1;
    return;
}

void __stdcall16far pass1_1018_6924(astruct_658 *param_1, ushort param_2, ushort param_3, int param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined4 uVar2;
    int        iVar3;
    uchar     *extraout_DX;
    undefined2 uVar4;
    ushort    *puVar5;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0x14       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x6a02;
    param_1->field_0x2                        = 0x1018;
    puVar5                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xb, param_5, extraout_DX, param_4);
    uVar4                                     = (undefined2)((ulong)puVar5 >> 0x10);
    param_1->field_0x14                       = (int)puVar5;
    param_1->field_0x16                       = uVar4;
    param_1->field_0x6                        = param_1->field_0x14;
    param_1->field_0x8                        = uVar4;
    uVar2                                     = *(undefined4 *)&param_1->field_0x14;
    iVar3                                     = &param_1->field_0xa;
    ppcVar1                                   = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1->field_0x12 = iVar3;
    draw_op_1020_9364((astruct_7 *)CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}


void __stdcall16far pass1_1018_69ac(ushort *param_1)

{
    astruct_511 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_511 *)param_1;
    *param_1         = 0x6a02;
    iVar1->field_0x2 = 0x1018;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

void __stdcall16far mixed_draw_op_1018_6a7a(astruct_28 *param_1, ushort param_2)

{
    uchar        *in_DX;
    int           unaff_DI;
    ushort        unaff_SS;
    ushort       *puVar1;
    PAINTSTRUCT16 local_22;

    BeginPaint16(param_2, &local_22);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, unaff_SS, in_DX, unaff_DI);
    if(*(int *)((int)puVar1 + 0x20) == 0x0)
    {
        unk_destroy_window_op_1018_6bb6(param_1, 0x1010);
        return;
    }
    mix_ui_op_1018_6adc(param_1);
    return;
}

void __stdcall16far clenaup_win_ui_1018_4d22(astruct_11 *in_struct_1, HDC16 in_hdc_2)

{
    uint        uVar1;
    code      **ppcVar2;
    astruct_11 *local_struct_1;
    astruct_11 *uVar4;
    ushort      unaff_SS;
    ULONG      *puVar2;
    ULONG      *puVar1;

    uVar4                     = (astruct_11 *)((ulong)in_struct_1 >> 0x10);
    local_struct_1            = (astruct_11 *)in_struct_1;
    *(int *)in_struct_1       = (int)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
    local_struct_1->field_0x2 = 0x1018;
    if(local_struct_1->field_0x12 != 0x0)
    {
        SelectPalette16(in_hdc_2, 0x0, local_struct_1->field_0x1a);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        in_hdc_2 = (HDC16)s_tile2_bmp_1050_1538;
        DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
    }
    puVar1 = local_struct_1->field_0xa;
    uVar1  = local_struct_1->field_0xc;
    if((uVar1 | (uint)puVar1) != 0x0)
    {
        ppcVar2 = (code **)*puVar1;
        (**ppcVar2)(in_hdc_2, puVar1, uVar1, 0x1);
    }
    puVar2 = local_struct_1->field_0xe;
    uVar1  = local_struct_1->field_0x10;
    if((uVar1 | (uint)puVar2) != 0x0)
    {
        ppcVar2 = (code **)*puVar2;
        (**ppcVar2)(in_hdc_2, puVar2, uVar1, 0x1);
    }
    _PTR_LOOP_1050_4230 = 0x0;
    pass1_1010_1d80((ushort *)in_struct_1, unaff_SS);
    return;
}


void __stdcall16far get_dc_1018_4db0(ULONG param_1, ushort param_2, HWND16 param_3)

{
    HDC16      HVar1;
    undefined2 uVar2;

    uVar2                            = (undefined2)(param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0x16) = param_2;
    HVar1                            = GetDC16(param_3);
    *(HDC16 *)((int)param_1 + 0x14)  = HVar1;
    return;
}

void __stdcall16far create_dc_1018_4e04(astruct_8 **param_1, UINT16 param_2, int param_3, int param_4, LPCSTR in_string_5, undefined2 in_string_6)

{
    code     **ppcVar1;
    astruct_8 *paVar2;
    astruct_9 *iVar4;
    undefined2 uVar3;
    ulong      uVar4;
    int        iStack16;

    uVar3   = (undefined2)((ulong)param_1 >> 0x10);
    iVar4   = (astruct_9 *)param_1;
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
    uVar4 = pass1_1008_4772((astruct_76 *)iVar4->field_0xa);
    pass1_1008_43cc((ulong)iVar4->field_0xa);
    paVar2            = (astruct_8 *)CreateDC16((LPCSTR)0x1008, (LPCSTR)uVar4, (LPCSTR)(uVar4 >> 0x10), (DEVMODEA *)0x0);
    iVar4->field_0x12 = paVar2;
    paVar2            = (astruct_8 *)&iVar4->field_0x12;
    ppcVar1           = (code **)((int)*iVar4->field_0xa + 0x8);
    (**ppcVar1)();
    iVar4->field_0x1a = paVar2;
    if((DAT_1050_422e != 0x0) && (0x280 < param_4))
    {
        for(iStack16 = 0x0; iStack16 < DAT_1050_4216 + 0x1; iStack16 = iStack16 + 0x1)
        {
            *(undefined2 *)((int)&PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) = (int)(((long)*(int *)((int)&PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) * ((long)param_4 + 0x1)) / 0x280);
        }
        for(iStack16 = 0x0; iStack16 < DAT_1050_422c + 0x1; iStack16 = iStack16 + 0x1)
        {
            *(undefined2 *)((int)&DAT_1050_419a + iStack16 * 0x2) = (int)(((long)*(int *)((int)&DAT_1050_419a + iStack16 * 0x2) * ((long)param_3 + 0x1)) / 0x1e0);
        }
    }
    DAT_1050_422e = 0x0;
    return;
}

void __stdcall16far struct_1018_5840(astruct_20 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uchar       *extraout_DX;
    undefined2   uVar1;
    astruct_130 *iVar2;
    int          unaff_DI;
    undefined2   uVar2;
    ushort      *puVar3;

    unk_draw_op_1020_7f7a(param_1, 0x6, CONCAT22(param_3, param_2));
    uVar2                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                             = (astruct_130 *)param_1;
    iVar2->field_0xee                 = 0x0;
    *(undefined4 *)&iVar2->field_0xf2 = 0x0;
    iVar2->field_0xf6                 = 0x0;
    param_1->field_0x0                = (int)s_Alloc__s_1050_5a5b + 0x7;
    iVar2->field_0x2                  = 0x1018;
    iVar2->field_0xe2                 = 0x5afe;
    iVar2->field_0xe4                 = 0x1018;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x27, param_4, extraout_DX, unaff_DI);
    uVar1                             = (undefined2)((ulong)puVar3 >> 0x10);
    iVar2->field_0xf2                 = (int)puVar3;
    iVar2->field_0xf4                 = uVar1;
    iVar2->field_0xe6                 = iVar2->field_0xf2;
    iVar2->field_0xe8                 = uVar1;
    return;
}

void __stdcall16far invalidate_rect_1018_58e2(astruct_58 *param_1, int param_2, HWND16 param_3)

{
    int        *piVar1;
    astruct_58 *iVar2;
    ushort      uVar2;

    if(param_2 == 0x105)
    {
        uVar2   = (ushort)((ulong)param_1 >> 0x10);
        iVar2   = (astruct_58 *)param_1;
        piVar1  = &iVar2->field_0xf6;
        *piVar1 = *piVar1 + 0x1;
        if((int)PTR_DAT_1050_0004_1050_4240 <= iVar2->field_0xf6)
        {
            PostMessage16(param_3, 0x0, 0x0, 0x11100ca);
            return;
        }
        iVar2->field_0xea = 0x0;
        InvalidateRect16(param_3, (RECT16 *)0x0, 0x0);
    }
    return;
}

void __stdcall16far pass1_1010_4674(ulong param_1, int param_2, ushort param_3, UINT16 param_4)

{
    int   *piVar1;
    UINT32 UVar2;
    UINT16 UVar3;

    UVar2 = (UINT32)param_1;
    UVar3 = (UINT16)(param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        piVar1  = (int *)(UVar2 + 0x24);
        *piVar1 = *piVar1 + 0x1;
        if(0xf < *(int *)(UVar2 + 0x24))
        {
            *(undefined2 *)(UVar2 + 0x24) = 0x0;
        }
    LAB_1010_469a:
        draw_op_1010_47d0(UVar2, UVar3, *(UINT16 *)(UVar2 + 0x24), param_3, param_4);
    }
    else
    {
        if(param_2 != 0x2)
        {
            if(param_2 != 0x3)
            {
                if((*(int *)(UVar2 + 0x6a) != 0x0) && (*(int *)(UVar2 + 0x6a) != 0x4))
                {
                    pass1_1010_459e(param_1);
                }
                goto LAB_1010_46e8;
            }
            piVar1  = (int *)(UVar2 + 0x24);
            *piVar1 = *piVar1 + -0x1;
            if(*piVar1 < 0x0)
            {
                *(undefined2 *)(UVar2 + 0x24) = 0xf;
            }
            goto LAB_1010_469a;
        }
    }
    pass1_1010_1f62(param_4, param_1, 0x2);
    pass1_1010_45d6(param_1, param_3);
LAB_1010_46e8:
    *(int *)(UVar2 + 0x6a) = param_2;
    return;
}

void __stdcall16far draw_1010_47ae(ulong param_1, ushort param_2, UINT16 param_3)

{
    UINT16 UStack4;

    UStack4 = 0x0;
    do
    {
        draw_op_1010_47d0((UINT32)param_1, (UINT16)(param_1 >> 0x10), UStack4, param_2, param_3);
        UStack4 = UStack4 + 0x1;
    } while((int)UStack4 < 0x10);
    return;
}

void __stdcall16far draw_op_1010_47d0(UINT32 param_1, UINT16 param_2, UINT16 param_3, INT16 in_style_3, UINT16 param_5)

{
    int        *piVar1;
    undefined4 *puVar2;
    code      **ppcVar3;
    int         iVar4;
    HPALETTE16  b_force_background;
    HGDIOBJ16   handle;
    HGDIOBJ16   handle_00;
    uint        uVar5;
    uchar      *extraout_DX;
    uchar      *puVar6;
    LPCSTR      output;
    astruct_5  *iVar6;
    int         iVar7;
    astruct_4  *iVar9;
    undefined2  uVar8;
    HDC16       hdc;
    ulong       uVar9;
    DEVMODEA   *init_data;
    undefined4  uVar10;
    int         iStack32;
    HDC16       local_14;
    LPCSTR      pCStack18;
    LPCSTR      pCStack16;
    undefined2  local_e;
    undefined2  uStack12;
    undefined2  uStack10;
    undefined2  uStack8;
    HGDIOBJ16   stock_obj_handle;
    HPEN16      pen_handle;

    uVar10           = 0x1;
    pen_handle       = CreatePen16(in_style_3, -0x2805, 0x77);
    uVar8            = 0x5;
    stock_obj_handle = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    local_e          = 0x0;
    uStack12         = 0x0;
    uStack10         = 0x1;
    uStack8          = 0x1;
    puVar2           = (undefined4 *)*(uint *)(param_1 + 0x26 + param_3 * 0x4);
    puVar6           = *(uchar **)(param_1 + 0x26 + param_3 * 0x4 + 0x2);
    if(((uint)puVar6 | (uint)puVar2) != 0x0)
    {
        ppcVar3 = (code **)*puVar2;
        (**ppcVar3)((int)s_tile2_bmp_1050_1538, puVar2, puVar6, 0x1, uVar8, uVar10);
        puVar6 = extraout_DX;
    }
    iVar4 = param_3 + 0x105;
    pass1_1010_8170(_PTR_LOOP_1050_14cc, iVar4, puVar6, (ushort)s_tile2_bmp_1050_1538);
    iVar7                               = param_3 * 0x4;
    *(int *)(param_1 + iVar7 + 0x26)    = iVar4;
    *(uchar **)(param_1 + iVar7 + 0x28) = puVar6;
    init_data                           = (DEVMODEA *)0x0;
    uVar9                               = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x26 + iVar7));
    output                              = (LPCSTR)(uVar9 >> 0x10);
    pCStack18                           = (LPCSTR)uVar9;
    pCStack16                           = output;
    local_14                            = CreateDC16((LPCSTR)0x1008, pCStack18, output, init_data);
    b_force_background                  = palette_op_1008_4e08(*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe), &local_14, (ushort)output, 0x1008);
    handle                              = SelectObject16(0x1008, pen_handle);
    hdc                                 = (HDC16)s_tile2_bmp_1050_1538;
    handle_00                           = SelectObject16((HDC16)s_tile2_bmp_1050_1538, stock_obj_handle);
    iStack32                            = 0x0;
    while(true)
    {
        piVar1 = (int *)(param_1 + 0x74);
        if(*piVar1 == iStack32 || *piVar1 < iStack32)
            break;
        iVar4 = (iStack32 * 0x10 + param_3) * 0x8;
        hdc   = 0x1000;
        uVar5 = pass1_1000_484c(CONCAT22(param_5, &local_e), CONCAT22(*(undefined2 *)(param_1 + 0x72), iVar4 + *(int *)(param_1 + 0x70)), 0x8);
        if(uVar5 != 0x0)
        {
            uVar10 = *(undefined4 *)(param_1 + 0x70);
            uVar8  = (undefined2)((ulong)uVar10 >> 0x10);
            iVar7  = (int)uVar10;
            iVar9  = (astruct_4 *)(iVar4 + iVar7);
            hdc    = (HDC16)s_tile2_bmp_1050_1538;
            Rectangle16(0x1000, iVar9->field_0x6, iVar9->field_0x4, iVar9->field_0x2, *(INT16 *)(iVar7 + iVar4));
        }
        iStack32 = iStack32 + 0x1;
    }
    SelectPalette16(hdc, 0x0, b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
    DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    return;
}

void __stdcall16far pt_in_rect_1010_4e08(ulong param_1, ushort param_2, ushort param_3, RECT16 *param_4)

{
    int       *piVar1;
    bool       bVar2;
    BOOL16     BVar3;
    int        iVar4;
    undefined2 uVar5;
    int        iStack12;
    int        iStack10;
    POINT16    PStack8;

    PStack8                       = (POINT16)CONCAT22(param_2, param_3);
    uVar5                         = (undefined2)(param_1 >> 0x10);
    iVar4                         = (int)param_1;
    *(undefined2 *)(iVar4 + 0x22) = *(undefined2 *)(iVar4 + 0x20);
    bVar2                         = false;
    *(undefined2 *)(iVar4 + 0x24) = 0x0;
    iStack12                      = 0x0;
    iStack10                      = 0x0;
    do
    {
        piVar1 = (int *)(iVar4 + 0x30);
        if(*piVar1 == iStack12 || *piVar1 < iStack12)
        {
        LAB_1010_4e67:
            if(iStack10 != 0x0)
            {
                *(int *)(iVar4 + 0x20) = iStack10;
            }
            if(bVar2)
            {
                return;
            }
            return;
        }
        BVar3 = PtInRect16(param_4, PStack8);
        if(BVar3 != 0x0)
        {
            iStack10 = iStack12;
            bVar2    = true;
            goto LAB_1010_4e67;
        }
        iStack12 = iStack12 + 0x1;
        param_4  = (RECT16 *)s_tile2_bmp_1050_1538;
    } while(true);
}

int __stdcall16far pt_in_rect_1010_40f8(ulong param_1, POINT16 *param_2, RECT16 *param_3)

{
    int        *piVar1;
    code      **ppcVar2;
    BOOL16      BVar3;
    ushort      uVar4;
    ushort      uVar5;
    int         iVar6;
    uchar      *in_DX;
    uchar      *puVar7;
    uchar      *puVar8;
    int         unaff_DI;
    undefined2  uVar9;
    ushort      unaff_SS;
    ushort     *puVar10;
    undefined4 *puStack16;
    int         iStack6;
    ushort      uStack4;

    iStack6 = 0x0;
    uStack4 = 0x0;
    do
    {
        uVar9  = (undefined2)(param_1 >> 0x10);
        piVar1 = (int *)((int)param_1 + 0x74);
        if(*piVar1 == iStack6 || *piVar1 < iStack6)
        {
        LAB_1010_413e:
            if((uStack4 != 0x0) && (0x3 < (int)PTR_LOOP_1050_3960))
            {
                puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, iStack6 + 0xcU, unaff_SS, in_DX, unaff_DI);
                puVar7  = (uchar *)((ulong)puVar10 >> 0x10);
                uVar4   = pass1_1018_0afa((ulong)puVar10);
                if(uVar4 == 0x0)
                {
                    uVar9 = 0x1000;
                    uVar5 = uVar4;
                    mem_op_1000_179c(0xb4, puVar7, 0x1000);
                    puVar8 = (uchar *)((uint)puVar7 | uVar5);
                    if(puVar8 == (uchar *)0x0)
                    {
                        iVar6  = 0x0;
                        puVar8 = (uchar *)0x0;
                    }
                    else
                    {
                        uVar9 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                        iVar6 = string_1040_8520((astruct_57 *)CONCAT22(puVar7, uVar5), (ushort)PTR_LOOP_1050_0396, 0x30, 0x2, 0x643, 0x645, puVar8, unaff_SS);
                    }
                    puStack16 = (undefined4 *)CONCAT22(puVar8, iVar6);
                    ppcVar2   = (code **)((int)*puStack16 + 0x74);
                    (**ppcVar2)(uVar9, iVar6, puVar8);
                    pass1_1010_209e(_PTR_LOOP_1050_0ed0, iStack6 + 0xcU);
                    uStack4 = uVar4;
                }
            }
            if(uStack4 != 0x0)
            {
                return iStack6;
            }
            return -0x1;
        }
        in_DX = *(uchar **)((int)param_1 + 0x72);
        BVar3 = PtInRect16(param_3, *param_2);
        if(BVar3 != 0x0)
        {
            uStack4 = 0x1;
            goto LAB_1010_413e;
        }
        iStack6 = iStack6 + 0x1;
        param_3 = (RECT16 *)s_tile2_bmp_1050_1538;
    } while(true);
}

uint16_t __stdcall16far draw_fn_1010_2a32(uint16_t param_1, uint16_t param_2, uint16 *__return_storage_ptr__, int param_4, ushort param_5, ulong param_6, uint16_t param_7, uint16_t param_8, uint16_t param_9, uint16_t param_10)

{
    int        *piVar1;
    char       *pcVar2;
    byte       *pbVar3;
    undefined4  uVar4;
    byte        bVar5;
    uint        uVar6;
    uint        uVar7;
    code      **ppcVar8;
    code       *pcVar9;
    uint16     *puVar10;
    uint        uVar11;
    HPALETTE16  b_force_background;
    HGDIOBJ16   handle;
    uint        uVar12;
    ushort      uVar13;
    BOOL16      BVar14;
    int         iVar15;
    byte        bVar16;
    uchar      *extraout_DX;
    uchar      *extraout_DX_00;
    uchar      *puVar17;
    uchar      *extraout_DX_01;
    uchar      *extraout_DX_02;
    uchar      *puVar18;
    int         unaff_SI;
    int         iVar19;
    int         iVar20;
    ushort      unaff_DI;
    undefined2  uVar21;
    HDC16       hdc;
    ushort      unaff_SS;
    byte        bVar22;
    bool        bVar23;
    undefined1  in_AF;
    ulong       uVar24;
    uint        uStack54;
    undefined4 *puStack46;
    uint        uStack42;
    undefined4 *puStack38;
    int         local_22;
    int         iStack32;
    HGDIOBJ16   HStack30;
    undefined   uVar25;
    HGDIOBJ16   handle_00;
    undefined   uVar26;
    undefined   uVar27;
    undefined   uVar28;

    puVar10 = __return_storage_ptr__;
    uVar27  = (undefined)param_9;
    uVar28  = (undefined)(param_9 >> 0x8);
    bVar22  = 0x0;
    uVar26  = 0x0;
    uVar25  = (undefined)((uint)param_4 >> 0x8);
    if(((ushort)param_6 + 0xec76 & 0x3) != 0x0)
        goto LAB_1010_2ad8;
    uVar11 = (ushort)param_6 + 0xec76 >> 0x1;
    if(0x1c < uVar11)
        goto LAB_1010_2ad8;
    switch(uVar11)
    {
    default:
        goto switchD_1010_2ab5_caseD_0;
    case 0x1:
    case 0x3:
    case 0xb:
        *(uint16_t *)(uVar11 + 0xa) = param_8;
    case 0x9:
    case 0xf:
    case 0x15:
    case 0x1b:
        *(uint16_t *)(uVar11 + 0xa)  = param_8;
        *(uint16_t *)(uVar11 + 0x10) = param_8;
        *(uint16_t *)(uVar11 + 0xc)  = param_8;
        return (uint16_t)(uchar *)param_10;
    case 0x5:
        BVar14 = write_to_file_1008_7e1c(param_5, (ushort)param_6, param_8, 0x0, (char *)CONCAT13(param_1._1_1_, CONCAT12((undefined)param_1, param_9)), 0x1008);
        if(BVar14 != 0x0)
        {
            return param_7;
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return param_7;
    case 0x6:
        bVar22 = 0x0;
        goto LAB_1010_2ad8;
    case 0x7:
        ppcVar8 = (code **)*(undefined4 *)param_8;
        (**ppcVar8)();
        iVar15  = param_5 + 0x105;
        puVar17 = extraout_DX;
        pass1_1010_8170(_PTR_LOOP_1050_14cc, iVar15, extraout_DX, 0x1010);
        iVar19                                                   = param_5 * 0x4;
        *(int *)((int)__return_storage_ptr__ + iVar19 + 0x26)    = iVar15;
        *(uchar **)((int)__return_storage_ptr__ + iVar19 + 0x28) = puVar17;
        handle_00                                                = (HGDIOBJ16)&USHORT_1050_1050;
        uVar24                                                   = pass1_1008_4772(*(astruct_76 **)((int)__return_storage_ptr__ + iVar19 + 0x26));
        puVar17                                                  = (uchar *)(uVar24 >> 0x10);
        CreateDC16((LPCSTR)0x1008, (LPCSTR)uVar24, (LPCSTR)puVar17, (DEVMODEA *)puVar17);
        b_force_background = palette_op_1008_4e08(*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe), &stack0xffec, (ushort)puVar17, 0x1008);
        handle             = SelectObject16(0x1008, CONCAT11(uVar26, bVar22));
        hdc                = (HDC16)s_tile2_bmp_1050_1538;
        HStack30           = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
        for(iStack32 = 0x0; piVar1 = (int *)((int)__return_storage_ptr__ + 0x74), *piVar1 != iStack32 && iStack32 <= *piVar1; iStack32 = iStack32 + 0x1)
        {
            iVar15             = (iStack32 * 0x10 + param_5) * 0x8;
            puVar17            = *(uchar **)((int)__return_storage_ptr__ + 0x72);
            hdc                = 0x1000;
            b_force_background = 0x48e1;
            uVar11 = pass1_1000_484c(CONCAT13((char)(unaff_SS >> 0x8), CONCAT12((char)unaff_SS, &stack0xfff2)), CONCAT13((char)((uint)puVar17 >> 0x8), CONCAT12((char)puVar17, iVar15 + *(int *)(__return_storage_ptr__ + 0x7))), 0x8);
            if(uVar11 != 0x0)
            {
                uVar4              = *(undefined4 *)(__return_storage_ptr__ + 0x7);
                uVar21             = (undefined2)((ulong)uVar4 >> 0x10);
                iVar19             = (int)uVar4;
                iVar20             = iVar15 + iVar19;
                hdc                = (HDC16)s_tile2_bmp_1050_1538;
                b_force_background = (HPALETTE16)&PTR_LOOP_1050_4908;
                Rectangle16(0x1000, *(INT16 *)(iVar20 + 0x6), *(INT16 *)(iVar20 + 0x4), *(INT16 *)(iVar20 + 0x2), *(INT16 *)(iVar19 + iVar15));
            }
        }
        SelectPalette16(hdc, 0x0, b_force_background);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack30);
        DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        return (uint16_t)puVar17;
    case 0x8:
        bVar22 = 0x3;
        goto LAB_1010_2ad8;
    case 0xd:
        pbVar3  = (byte *)(uVar11 + unaff_SI);
        bVar22  = *pbVar3;
        bVar5   = *pbVar3 + (byte)param_7;
        *pbVar3 = bVar5 + (uVar11 < 0x1c);
        uVar6   = (uint)(CARRY1(bVar22, (byte)param_7) || CARRY1(bVar5, uVar11 < 0x1c));
        uVar7   = param_8 + 0xeff0;
        bVar22  = param_8 < 0x1010 || uVar7 < uVar6;
        uVar12  = uVar7 - uVar6;
        pcVar9  = (code *)swi(0x4);
        if(SBORROW2(param_8, 0x1010) != SBORROW2(uVar7, uVar6))
        {
            (*pcVar9)();
            param_7 = (uint16_t)extraout_DX_00;
        }
        bVar23  = uVar12 < 0x1010 || uVar12 + 0xeff0 < (uint)bVar22;
        pbVar3  = (byte *)(uVar11 + unaff_SI);
        bVar22  = *pbVar3;
        bVar16  = (byte)param_7;
        bVar5   = *pbVar3;
        *pbVar3 = bVar5 + bVar16 + bVar23;
        pcVar2  = (char *)(uVar11 + unaff_SI);
        *pcVar2 = *pcVar2 + bVar16 + (CARRY1(bVar22, bVar16) || CARRY1(bVar5 + bVar16, bVar23));
        struct_op_1018_4cda(CONCAT11(uVar27, uVar26), CONCAT11((undefined)param_1, uVar28), CONCAT11((undefined)param_2, param_1._1_1_));
        iVar15                        = CONCAT11(uVar27, uVar26);
        piVar1                        = (int *)CONCAT13((undefined)param_1, CONCAT12(uVar28, iVar15));
        *piVar1                       = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
        *(undefined2 *)(iVar15 + 0x2) = 0x1010;
        pass1_1018_4dce((ulong *)CONCAT13((undefined)param_1, CONCAT12(uVar28, iVar15)), 0x1b3, (uchar *)param_7, unaff_SS);
        _PTR_LOOP_1050_4230 = CONCAT13((undefined)param_1, CONCAT12(uVar28, CONCAT11(uVar27, uVar26)));
        return (uint16_t)(uchar *)CONCAT11((undefined)param_1, uVar28);
    case 0xe:
        *(ushort *)(__return_storage_ptr__ + 0x2) = param_5;
        break;
    case 0x11:
        do
        {
            // WARNING: Do nothing block with infinite loop
        } while(true);
    case 0x12:
        PTR_LOOP_1050_10c6 = (undefined *)(uint)(0x0 < (int)param_5);
        PTR_LOOP_1050_1142 = (undefined *)(uint)(0x2 < (int)param_5);
        break;
    case 0x13:
        iVar15 = (int)__return_storage_ptr__ * 0x8 + param_1;
        if(((*(int *)(iVar15 + 0x22) != 0x0) || (*(int *)(iVar15 + 0x24) != 0x0)) || ((*(int *)(iVar15 + 0x26) != 0x0 || (*(int *)(iVar15 + 0x28) != 0x0))))
        {
            uVar4    = *(undefined4 *)(param_1 + 0xe);
            HStack30 = 0x627c;
            sys_1000_3f9c((uchar *)uVar4,
                          (uchar *)((ulong)uVar4 >> 0x10),
                          (ushort)s__d__d__d__d_1050_14ae,
                          (ushort)&USHORT_1050_1050,
                          (ushort) * (undefined4 *)((int)__return_storage_ptr__ * 0x8 + param_1 + 0x22),
                          &stack0xfffa,
                          param_2,
                          0x1000,
                          unaff_SS,
                          in_AF);
            uVar4    = *(undefined4 *)(param_1 + 0xa);
            HStack30 = 0x62a0;
            WritePrivateProfileString16((LPCSTR)&PTR_LOOP_1050_1000, (LPCSTR)uVar4, (LPCSTR)((ulong)uVar4 >> 0x10), (LPCSTR) * (undefined4 *)(param_1 + 0xe));
        }
        return param_7;
    case 0x14:
        *(ushort *)((int)__return_storage_ptr__ + 0x24) = param_5;
        break;
    case 0x17:
        puVar17                                           = (uchar *)(param_7 - 0x1);
        pbVar3                                            = (byte *)(uVar11 + unaff_SI);
        *pbVar3                                           = *pbVar3 | (byte)puVar17;
        *(uint16_t *)((int)__return_storage_ptr__ + 0x12) = param_8;
        *(uchar **)((int)__return_storage_ptr__ + 0x14)   = puVar17;
        uStack42                                          = 0x0;
        while(true)
        {
            if(uStack54 <= uStack42)
            {
                BVar14 = read_file_1008_7dee(param_5, (ushort)param_6, (int)__return_storage_ptr__ + 0x1a, 0x0, param_4, 0x2, 0x1008);
                if(((BVar14 != 0x0) && (BVar14 = read_file_1008_7dee(param_5, (ushort)param_6, (int)__return_storage_ptr__ + 0x1c, 0x0, param_4, 0x2, 0x1008), BVar14 != 0x0))
                   && (BVar14 = read_file_1008_7dee(param_5, (ushort)param_6, (int)__return_storage_ptr__ + 0x1e, 0x0, param_4, 0x2, 0x1008), BVar14 != 0x0))
                {
                    return (uint16_t)puVar17;
                }
                PTR_LOOP_1050_0310 = (undefined *)0x6d2;
                return (uint16_t)puVar17;
            }
            uVar11 = uStack54;
            mem_op_1000_179c(0x8, puVar17, 0x1000);
            puStack46 = (undefined4 *)CONCAT22(puVar17, uVar11);
            puVar18   = (uchar *)((uint)puVar17 | uVar11);
            if(puVar18 == (uchar *)0x0)
            {
                puStack38 = (undefined4 *)0x0;
            }
            else
            {
                *(undefined2 *)puStack46      = 0x389a;
                *(undefined2 *)(uVar11 + 0x2) = 0x1008;
                *(undefined2 *)puStack46      = 0xa1c4;
                *(undefined2 *)(uVar11 + 0x2) = 0x1010;
                puStack38                     = puStack46;
            }
            BVar14 = read_file_1008_7dee(param_5, (ushort)param_6, (ushort)&local_22, 0x0, unaff_SS, 0x2, 0x1008);
            uVar13 = (ushort)((ulong)puStack38 >> 0x10);
            if((BVar14 == 0x0) || (BVar14 = read_file_1008_7dee(param_5, (ushort)param_6, (int)puStack38 + 0x6, 0x0, uVar13, 0x2, 0x1008), BVar14 == 0x0))
                break;
            iVar15                         = switch_1008_73ea(param_5, (ushort)param_6, local_22);
            *(int *)((int)puStack38 + 0x4) = iVar15;
            ppcVar8                        = (code **)((int)*(undefined4 *)*(undefined4 *)((int)__return_storage_ptr__ + 0x12) + 0x4);
            (**ppcVar8)();
            uStack42 = uStack42 + 0x1;
            puVar17  = extraout_DX_02;
        }
        if(puStack38 == (undefined4 *)0x0)
        {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return (uint16_t)puVar18;
        }
        ppcVar8 = (code **)*puStack38;
        (**ppcVar8)();
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return (uint16_t)extraout_DX_01;
    case 0x18:
        bVar22 = 0x2;
        goto LAB_1010_2ad8;
    case 0x19:
        uVar13 = pass1_1010_6ca2(CONCAT13(uVar25, CONCAT12((char)param_4, __return_storage_ptr__)), 0x8, param_7, unaff_SS);
        if(uVar13 != 0x0)
        {
            __return_storage_ptr__ = (uint16 *)((int)s_version__d__d_1050_0012 + 0x8);
            pass1_1010_715c(CONCAT22(0x1a, puVar10), 0x1a, uVar13, param_7, unaff_DI, unaff_SS);
        }
        if(param_5 == 0x2c)
        {
            pass1_1010_715c(CONCAT22(0x1d, __return_storage_ptr__), 0x1d, uVar13, param_7, unaff_DI, unaff_SS);
        }
        uVar13 = pass1_1010_6ca2(0x5a, 0x2, param_7, unaff_SS);
        if(uVar13 != 0x0)
        {
            pass1_1010_715c(0x1c005a, 0x1c, uVar13, param_7, unaff_DI, unaff_SS);
        }
        return param_7;
    case 0x1a:
        *(ushort *)((int)__return_storage_ptr__ + 0x26) = param_5;
    }
    bVar22 = 0x1;
LAB_1010_2ad8:
    if((bVar22 == 0x1) || (bVar22 == 0x2))
    {
        if(bVar22 == 0x1)
        {
            param_5 = *(int *)(__return_storage_ptr__ + 0x2) + *(int *)((int)__return_storage_ptr__ + 0x22) + *(int *)((int)__return_storage_ptr__ + 0x24) + *(int *)((int)__return_storage_ptr__ + 0x26);
        }
        if(param_5 != 0x0)
        {
            param_7 = (int)param_5 >> 0xf;
            param_5 = (int)param_5 / 0x2 + 0x1;
            if(0x5 < (int)param_5)
            {
                param_5 = 0x5;
            }
            if(((bVar22 == 0x1) && (PTR_LOOP_1050_10c6 != (undefined *)0x0)) && (0x4 < (int)param_5))
            {
                param_5 = 0x4;
            }
        }
    }
    *(ushort *)((uint)bVar22 * 0x7c + 0xed6) = param_5;
    pass1_1010_1f62(unaff_SS, CONCAT13(uVar25, CONCAT12((char)param_4, __return_storage_ptr__)), 0xc);
switchD_1010_2ab5_caseD_0:
    return param_7;
}
