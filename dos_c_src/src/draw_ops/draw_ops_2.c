
void __stdcall16far pass1_1038_9a48(astruct_18 *param_1)

{
    param_1->field_0x0                  = 0x9af6;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void __stdcall16far pass1_1038_7d5c(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0x8876;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}

void __stdcall16far unk_draw_op_1018_cda8(astruct_36 *param_1, ushort param_2)

{
    int          *piVar1;
    int           iVar2;
    astruct_76   *paVar3;
    code        **ppcVar4;
    ushort        uVar5;
    HDC16        *b_force_background;
    int           iVar6;
    int           iVar7;
    uchar        *in_DX;
    ushort        uVar8;
    undefined2    uVar9;
    undefined2    extraout_DX;
    int           iVar10;
    int           unaff_DI;
    undefined2    uVar11;
    uint          uVar12;
    undefined2    uVar13;
    HWND16        hwnd;
    ulong         uVar14;
    undefined2    uVar15;
    undefined2    uVar16;
    HDC16        *pHVar17;
    RECT16       *pRVar19;
    undefined4    uVar18;
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
    uVar8     = (ushort)((ulong)puStack38 >> 0x10);
    uVar5     = *(ushort *)((int)puStack38 + 0x20);
    iVar10    = (int)param_1;
    uVar11    = (undefined2)((ulong)param_1 >> 0x10);
    uStack40  = uVar5;
    if(uVar5 == 0x0)
    {
        BeginPaint16(0x1010, &local_22);
        EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
        PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, CONCAT22(0x111, *(undefined2 *)(iVar10 + 0xea)));
        return;
    }
    if(*(int *)(iVar10 + 0xf0) == 0x0)
    {
        *(undefined2 *)(iVar10 + 0xf0) = 0x1;
        hwnd                           = 0x1008;
        win_1008_5c5c((WNDCLASS16 *)param_2, uVar5, uVar8, _PTR_LOOP_1050_02a0, 0x1f3);
        uVar12 = (uint)(_PTR_LOOP_1050_02a0 >> 0x10);
        if(*(int *)((int)_PTR_LOOP_1050_02a0 + 0x12) == 0x0)
        {
            hwnd = 0x1008;
            win_1008_5c5c((WNDCLASS16 *)param_2, uVar5, uVar8, _PTR_LOOP_1050_02a0 & 0xffff | (ulong)uVar12 << 0x10, 0x1d3);
        }
    }
    local_2a  = BeginPaint16(hwnd, &local_22);
    pRStack44 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    local_34  = 0x0;
    iStack48  = *(int *)(iVar10 + 0xf6) + -0x1;
    iStack46  = *(int *)(iVar10 + 0xf8) + -0x1;
    FillRect16((HDC16)s_tile2_bmp_1050_1538, pRStack44, (HBRUSH16)&local_34);
    pRVar19 = pRStack44;
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    uVar18             = *(undefined4 *)(iVar10 + 0xe2);
    paVar3             = *(astruct_76 **)((int)uVar18 + 0xe);
    b_force_background = &local_2a;
    uVar18             = CONCAT22(pRVar19, param_2);
    uVar13             = (undefined2)((ulong)paVar3 >> 0x10);
    ppcVar4            = (code **)((int)*(undefined4 *)paVar3 + 0x8);
    uVar15             = (int)paVar3;
    uVar16             = uVar13;
    pHVar17            = b_force_background;
    (**ppcVar4)();
    uVar14                   = pass1_1008_4772(paVar3);
    uVar9                    = (undefined2)(uVar14 >> 0x10);
    iVar6                    = (0x280 - *(int *)((int)uVar14 + 0x4)) / 0x2;
    iVar2                    = *(int *)((int)uVar14 + 0x8);
    iVar7                    = (0x1e0 - iVar2) / 0x2;
    *(int *)(iVar10 + 0x10c) = iVar7 + iVar2 + *(int *)(iVar10 + 0x110);
    if((*(int *)(iVar10 + 0xfa) == 0x0) && (iVar6 == 0x0))
    {
        piVar1  = (int *)(iVar10 + 0xfa);
        *piVar1 = *piVar1 + 0x2;
    }
    ppcVar4 = (code **)((int)*(undefined4 *)paVar3 + 0x4);
    (**ppcVar4)(0x1008, (int)paVar3, uVar13, *(int *)(iVar10 + 0xfc) + *(int *)(iVar10 + 0xfe) + iVar7, *(int *)(iVar10 + 0xfa) + iVar6, &local_2a, param_2, uVar15, uVar16, pHVar17, uVar18);
    draw_text_1018_c742(param_1, 0x1008, param_2, extraout_DX);
    SelectPalette16(0x1008, 0x0, (BOOL16)b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

void __stdcall16far unk_draw_op_1018_cfc0(astruct_36 *param_1, ushort param_2)

{
    int          *piVar1;
    int           iVar2;
    astruct_76   *paVar3;
    code        **ppcVar4;
    uint          uVar5;
    HDC16        *b_force_background;
    int           iVar6;
    int           iVar7;
    uchar        *in_DX;
    ushort        uVar8;
    undefined2    uVar9;
    undefined2    extraout_DX;
    int           iVar10;
    int           unaff_DI;
    undefined2    uVar11;
    undefined2    uVar12;
    HWND16        hwnd;
    ulong         uVar13;
    undefined2    uVar14;
    undefined2    uVar15;
    HDC16        *pHVar16;
    RECT16       *pRVar18;
    undefined4    uVar17;
    HDC16         HVar19;
    undefined4    local_34;
    int           iStack48;
    int           iStack46;
    RECT16       *pRStack44;
    HDC16         local_2a;
    int           iStack40;
    ushort       *puStack38;
    PAINTSTRUCT16 local_22;

    hwnd      = 0x1010;
    puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uVar8     = (ushort)((ulong)puStack38 >> 0x10);
    iStack40  = *(int *)((int)puStack38 + 0x20);
    iVar10    = (int)param_1;
    uVar11    = (undefined2)((ulong)param_1 >> 0x10);
    if(iStack40 == 0x0)
    {
        BeginPaint16(0x1010, &local_22);
        EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
        PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, CONCAT22(0x111, *(undefined2 *)(iVar10 + 0xea)));
        return;
    }
    if((*(int *)(iVar10 + 0xf0) == 0x0) && (*(int *)(iVar10 + 0xf4) != 0x0))
    {
        *(undefined2 *)(iVar10 + 0xf0) = 0x1;
        uVar5                          = iVar10 + 0xf2;
        hwnd                           = 0x1008;
        win_1008_5c9e(_PTR_LOOP_1050_02a0, (ulong *)((ulong)param_1 & 0xffff0000 | (ulong)uVar5), uVar5, uVar8, param_2);
        if(*(int *)((int)_PTR_LOOP_1050_02a0 + 0x12) == 0x0)
        {
            hwnd = 0x1008;
            win_1008_5c5c((WNDCLASS16 *)param_2, uVar5, uVar8, _PTR_LOOP_1050_02a0, 0x1d3);
        }
    }
    local_2a  = BeginPaint16(hwnd, &local_22);
    pRStack44 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    local_34  = 0x0;
    iStack48  = *(int *)(iVar10 + 0xf6) + -0x1;
    iStack46  = *(int *)(iVar10 + 0xf8) + -0x1;
    uVar8     = param_2;
    HVar19    = local_2a;
    FillRect16((HDC16)s_tile2_bmp_1050_1538, pRStack44, (HBRUSH16)&local_34);
    pRVar18 = pRStack44;
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    uVar17             = *(undefined4 *)(iVar10 + 0xe2);
    paVar3             = *(astruct_76 **)((int)uVar17 + 0xe);
    b_force_background = &local_2a;
    uVar17             = CONCAT22(pRVar18, param_2);
    uVar12             = (undefined2)((ulong)paVar3 >> 0x10);
    ppcVar4            = (code **)((int)*(undefined4 *)paVar3 + 0x8);
    uVar14             = (int)paVar3;
    uVar15             = uVar12;
    pHVar16            = b_force_background;
    (**ppcVar4)();
    uVar13                   = pass1_1008_4772(paVar3);
    uVar9                    = (undefined2)(uVar13 >> 0x10);
    iVar6                    = (0x280 - *(int *)((int)uVar13 + 0x4)) / 0x2;
    iVar2                    = *(int *)((int)uVar13 + 0x8);
    iVar7                    = (0x1e0 - iVar2) / 0x2;
    *(int *)(iVar10 + 0x10c) = iVar7 + iVar2 + *(int *)(iVar10 + 0x110);
    if((*(int *)(iVar10 + 0xfa) == 0x0) && (iVar6 == 0x0))
    {
        piVar1  = (int *)(iVar10 + 0xfa);
        *piVar1 = *piVar1 + 0x2;
    }
    ppcVar4 = (code **)((int)*(undefined4 *)paVar3 + 0x4);
    (**ppcVar4)(0x1008, (int)paVar3, uVar12, *(int *)(iVar10 + 0xfc) + *(int *)(iVar10 + 0xfe) + iVar7, *(int *)(iVar10 + 0xfa) + iVar6, &local_2a, param_2, uVar14, uVar15, pHVar16, uVar17, uVar8, HVar19);
    draw_text_1018_c742(param_1, 0x1008, param_2, extraout_DX);
    SelectPalette16(0x1008, 0x0, (BOOL16)b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

void __stdcall16far palette_op_1020_92c4(undefined2 *param_1, HDC16 param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *param_1                     = 0x96c8;
    *(undefined2 *)(iVar1 + 0x2) = 0x1020;
    if(*(int *)(iVar1 + 0x12) != 0x0)
    {
        SelectPalette16(param_2, 0x0, *(BOOL16 *)(iVar1 + 0x12));
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
    *param_1                     = 0x3ab0;
    *(undefined2 *)(iVar1 + 0x2) = 0x1008;
    *param_1                     = 0x389a;
    *(undefined2 *)(iVar1 + 0x2) = 0x1008;
    return;
}


void __stdcall16far mix_draw_op_1020_9312(ulong param_1, HWND16 param_2)

{
    undefined4   *puVar1;
    code        **ppcVar2;
    undefined4    uVar3;
    int           iVar4;
    undefined2    uVar5;
    undefined2    uVar6;
    PAINTSTRUCT16 local_22;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar6 = *(undefined2 *)(iVar4 + 0x4);
    BeginPaint16(param_2, &local_22);
    uVar3   = *(undefined4 *)(iVar4 + 0x6);
    puVar1  = (undefined4 *)*(undefined4 *)((int)uVar3 + 0xa);
    ppcVar2 = (code **)((int)*puVar1 + 0x4);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)puVar1, (int)((ulong)puVar1 >> 0x10), 0x0, param_1 & 0xffff0000 | (ulong)(iVar4 + 0xa), uVar6);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

void __stdcall16far draw_op_1020_9364(astruct_7 *param_1, HWND16 in_win_handle_2, INT16 param_3)

{
    int       *piVar1;
    uint       uVar2;
    int        iVar3;
    undefined4 uVar4;
    int        iVar5;
    RECT16    *pRVar6;
    astruct_7 *local_struct_1;
    undefined2 var7;
    undefined2 uVar7;
    int        iStack62;
    uint       uStack58;
    undefined  local_38[0x4];
    HGDIOBJ16  HStack52;
    HPEN16     HStack50;
    undefined2 uStack48;
    undefined4 uStack46;
    undefined4 uStack42;
    undefined4 uStack38;
    undefined4 uStack34;
    undefined4 uStack30;
    uint      *puStack26;
    int        iStack22;
    int        iStack20;
    ulong      local_12;
    undefined4 uStack14;
    RECT16     local_a;
    ulong      uStack6;

    var7           = (undefined2)((ulong)param_1 >> 0x10);
    local_struct_1 = (astruct_7 *)param_1;
    GetClientRect16(in_win_handle_2, &local_a);
    local_12  = local_a;
    uStack14  = uStack6;
    iStack20  = DAT_1050_4216;
    iStack22  = DAT_1050_422c;
    puStack26 = _PTR_PTR_DAT_1050_0009_1050_4172_1050_4212;
    uStack30  = _PTR_PTR_1050_4218;
    uStack34  = _PTR_PTR_s_ew_failed_in_Op_Op_1050_0021_1050_41da_1050_421c;
    uStack38  = _PTR_PTR_DAT_1050_0041_1050_4202_1050_4220;
    uStack42  = _PTR_DAT_1050_419a_1050_4224;
    uStack46  = _PTR_PTR_1050_4228;
    uVar4     = local_struct_1->field_0x6;
    uStack48  = *(undefined2 *)((int)uVar4 + 0x12);
    uStack58  = 0x9;
    do
    {
        uVar4    = *(undefined4 *)(uStack58 * 0x4 + (int)uStack34);
        HStack50 = CreatePen16((INT16)s_tile2_bmp_1050_1538, (INT16)uVar4, (COLORREF)((ulong)uVar4 >> 0x10));
        HStack52 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack50);
        MoveToEx16((HDC16)s_tile2_bmp_1050_1538, (INT16)local_38, param_3, *(POINT16 **)(uStack58 * 0x2 + (int)puStack26));
        LineTo16((HDC16)s_tile2_bmp_1050_1538, *(INT16 *)((int)puStack26 + uStack58 * 0x2), (INT16)uStack6);
        iVar3 = (iStack20 - uStack58) * 0x2;
        MoveToEx16((HDC16)s_tile2_bmp_1050_1538, (INT16)local_38, param_3, *(POINT16 **)(iVar3 + (int)puStack26));
        LineTo16((HDC16)s_tile2_bmp_1050_1538, *(INT16 *)((int)puStack26 + iVar3), (INT16)uStack6);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack52);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        uStack58 = uStack58 - 0x1;
    } while(uStack58 < 0x8000);
    pRVar6   = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    uVar7    = (undefined2)((ulong)puStack26 >> 0x10);
    local_a  = CONCAT22(*(int *)((int)puStack26 + 0x12) + 0x1, local_a.x);
    uVar2    = *(uint *)((int)puStack26 + 0x14);
    uStack14 = uStack14 & 0xffff | (ulong)uVar2 << 0x10;
    uStack6  = CONCAT22(uVar2, (INT16)uStack6);
    FillRect16((HDC16)s_tile2_bmp_1050_1538, pRVar6, (HBRUSH16)&local_a);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    iStack62 = 0x8;
    for(uStack58 = 0x1; (int)uStack58 < 0xa; uStack58 = uStack58 + 0x1)
    {
        pRVar6   = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
        uStack6  = uStack6 & 0xffff | (ulong)(local_a.y - 0x1) << 0x10;
        local_12 = local_12 & 0xffff | (ulong)(uStack14._2_2_ + 0x1) << 0x10;
        uVar7    = (undefined2)((ulong)puStack26 >> 0x10);
        local_a  = local_a & 0xffff | (ulong)(*(int *)(iStack62 * 0x2 + (int)puStack26) + 0x1) << 0x10;
        uStack14 = uStack14 & 0xffff | (ulong) * (uint *)(uStack58 * 0x2 + (int)puStack26 + 0x14) << 0x10;
        FillRect16((HDC16)s_tile2_bmp_1050_1538, pRVar6, (HBRUSH16)&local_a);
        FillRect16((HDC16)s_tile2_bmp_1050_1538, pRVar6, (HBRUSH16)&local_12);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        iStack62 = iStack62 + -0x1;
    }
    pRVar6   = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    local_a  = local_a & 0xffff;
    uStack6  = uStack6 & 0xffff | (ulong)*puStack26 << 0x10;
    local_12 = local_12 & 0xffff | (ulong)(*(int *)(iStack20 * 0x2 + (int)puStack26) + 0x1) << 0x10;
    uStack14 = uStack14 & 0xffff | (ulong)local_struct_1->field_0xe << 0x10;
    FillRect16((HDC16)s_tile2_bmp_1050_1538, pRVar6, (HBRUSH16)&local_a);
    FillRect16((HDC16)s_tile2_bmp_1050_1538, pRVar6, (HBRUSH16)&local_12);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    uStack58 = 0x3;
    do
    {
        uVar4    = *(undefined4 *)(uStack58 * 0x4 + (int)uStack38);
        HStack50 = CreatePen16((INT16)s_tile2_bmp_1050_1538, (INT16)uVar4, (COLORREF)((ulong)uVar4 >> 0x10));
        HStack52 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack50);
        iVar5    = uStack58 * 0x2;
        iVar3    = *(int *)(iVar5 + (int)uStack42);
        uVar7    = (undefined2)((ulong)uStack46 >> 0x10);
        piVar1   = (int *)(iVar5 + (int)uStack46);
        MoveToEx16((HDC16)s_tile2_bmp_1050_1538, (INT16)local_38, param_3, *(POINT16 **)(*(int *)(iVar5 + (int)uStack46) * 0x2 + (int)puStack26));
        LineTo16((HDC16)s_tile2_bmp_1050_1538, *(INT16 *)((iStack20 - *piVar1) * 0x2 + (int)puStack26), iVar3 + local_a.x);
        iVar3 = *(int *)((iStack22 - uStack58) * 0x2 + (int)uStack42);
        MoveToEx16((HDC16)s_tile2_bmp_1050_1538, (INT16)local_38, param_3, *(POINT16 **)(*piVar1 * 0x2 + (int)puStack26));
        LineTo16((HDC16)s_tile2_bmp_1050_1538, *(INT16 *)((iStack20 - *piVar1) * 0x2 + (int)puStack26), iVar3 + local_a.x);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack52);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        uStack58 = uStack58 - 0x1;
    } while(uStack58 < 0x8000);
    local_struct_1->field_0x10 = 0x0;
    return;
}


astruct_18 *__stdcall16far pass1_1020_96a2(astruct_18 *param_1, byte param_2, ushort param_3)

{
    palette_op_1020_92c4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_18 *__stdcall16far pass1_1020_7526(astruct_18 *param_1, byte param_2, ushort param_3)

{
    palette_op_1020_7270(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far struct_1020_7554(ushort param_1, astruct_20 *param_2, ushort param_3, ushort param_4)

{
    uchar       *extraout_DX;
    undefined2   uVar1;
    astruct_129 *iVar2;
    int          unaff_DI;
    undefined2   uVar2;
    ushort      *puVar3;

    unk_draw_op_1020_7f7a(param_2, 0x5, CONCAT22(param_4, param_3));
    uVar2                             = (undefined2)((ulong)param_2 >> 0x10);
    iVar2                             = (astruct_129 *)param_2;
    iVar2->field_0xee                 = 0x0;
    *(undefined4 *)&iVar2->field_0xf2 = 0x0;
    param_2->field_0x0                = 0x7780;
    iVar2->field_0x2                  = 0x1020;
    iVar2->field_0xe2                 = 0x781c;
    iVar2->field_0xe4                 = 0x1020;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x25, param_1, extraout_DX, unaff_DI);
    uVar1                             = (undefined2)((ulong)puVar3 >> 0x10);
    iVar2->field_0xf2                 = (int)puVar3;
    iVar2->field_0xf4                 = uVar1;
    iVar2->field_0xe6                 = iVar2->field_0xf2;
    iVar2->field_0xe8                 = uVar1;
    return;
}

void __stdcall16far pass1_1020_7824(astruct_666 *param_1, ushort param_2, ushort param_3, int param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined4 uVar2;
    int        iVar3;
    uchar     *extraout_DX;
    undefined2 uVar4;
    ushort    *puVar5;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0x14       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x7902;
    param_1->field_0x2                        = 0x1020;
    puVar5                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x25, param_5, extraout_DX, param_4);
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
    draw_op_1020_9364((astruct_7 *)CONCAT22(param_2, param_1), 0x1010, param_5);
    return;
}

void __stdcall16far pass1_1020_78ac(ushort *param_1, ushort param_2)

{
    astruct_587 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_587 *)param_1;
    *param_1         = 0x7902;
    iVar1->field_0x2 = 0x1020;
    if(iVar1->field_0x14 != 0x0)
    {
        param_2 = 0x1010;
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, param_2);
    return;
}

void __stdcall16far struct_1020_790e(ushort *param_1, ulong param_2, UINT16 param_3, ulong param_4, UINT16 param_5)

{
    astruct_271 *iVar1;
    ushort       uVar1;

    unk_draw_op_1008_7f62(param_1, param_3, param_4, param_5);
    uVar1             = (ushort)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_271 *)param_1;
    iVar1->field_0xe0 = 0x0;
    iVar1->field_0xe4 = 0x0;
    iVar1->field_0xe8 = 0x0;
    iVar1->field_0xec = 0x0;
    iVar1->field_0xee = param_2;
    *param_1          = 0x7b86;
    iVar1->field_0x2  = 0x1020;
    return;
}


void __stdcall16far string_1020_79b4(ushort param_1, ulong param_2, int param_3, char *param_4)

{
    unk_str_op_1000_3d3e((char *)(param_2 & 0xffff0000 | (ulong)((int)param_2 + 0xa)), param_4);
    if(param_3 != 0x0)
    {
        draw_op_1020_7cc8(*(ULONG *)((int)param_2 + 0xe8), 0x1000, param_1);
    }
    return;
}


void __stdcall16far pass1_1020_79e4(ulong param_1, ushort param_2, UINT16 param_3)

{
    draw_op_1020_7cc8(*(ULONG *)((int)param_1 + 0xe8), param_2, param_3);
    return;
}

void __stdcall16far draw_op_1020_7cc8(ULONG param_1, HWND16 in_win_handle_2, UINT16 param_3)

{
    code      **ppcVar1;
    RECT16     *rect;
    COLORREF    color;
    HPEN16      handle;
    HGDIOBJ16   handle_00;
    char       *count;
    LPCSTR      str;
    undefined4 *puVar2;
    ushort      in_DX;
    char       *str_00;
    astruct_6  *iVar4;
    int         iVar3;
    undefined2  uVar4;
    undefined2  uVar5;
    DWORD       DVar6;
    ulong       uVar7;
    ulong       uVar8;
    HBRUSH16    hbrush;
    undefined4  uVar9;
    HDC16       HVar10;
    undefined2  uVar11;
    int         iStack66;
    undefined2  local_20;
    int         iStack30;
    int         iStack28;
    int         iStack26;
    int         iStack24;
    int         iStack22;
    RECT16      local_rect_1;
    int         iStack16;
    int         iStack14;
    HPALETTE16  HStack12;
    astruct_13 *paStack10;
    HDC16       local_hdc_1;
    BOOL16      is_iconic;

    uVar4     = (undefined2)(param_1 >> 0x10);
    iVar4     = (astruct_6 *)param_1;
    is_iconic = IsIconic16(in_win_handle_2);
    if((is_iconic == 0x0) || (PTR_LOOP_1050_0010 != (undefined *)0x0))
    {
        local_hdc_1 = GetWindowDC16((HWND16)s_tile2_bmp_1050_1538);
        paStack10   = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
        HStack12    = palette_op_1008_4e08(paStack10, &local_hdc_1, in_DX, 0x1008);
        uVar11      = iVar4->field_0x4;
        GetWindowRect16(0x1008, &local_rect_1);
        iStack28 = (iStack16 - local_rect_1.x) + -0x1;
        iStack24 = (iStack14 - local_rect_1.y) + -0x1;
        local_20 = iVar4->field_0x10;
        iStack30 = iVar4->field_0x12;
        iStack26 = iStack24;
        if(is_iconic == 0x0)
        {
            iStack26 = iVar4->field_0xe - iVar4->field_0x12;
        }
        uVar9    = CONCAT22(param_3, &local_20);
        hbrush   = 0x4;
        HVar10   = local_hdc_1;
        iStack22 = iStack28;
        rect     = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        FillRect16((HDC16)s_tile2_bmp_1050_1538, rect, hbrush);
        puVar2  = iVar4->field_0x6;
        uVar5   = (undefined2)((ulong)puVar2 >> 0x10);
        iVar3   = (int)puVar2;
        puVar2  = (undefined4 *)*(undefined4 *)(iVar3 + 0xe0);
        ppcVar1 = (code **)((int)*puVar2 + 0x24);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538, (int)puVar2, *(undefined2 *)(iVar3 + 0xe2), 0x0, uVar9, HVar10, uVar11);
        color     = (-(uint)((int)puVar2 == 0x0) & 0x1e) + 0x25;
        handle    = CreatePen16((INT16)s_tile2_bmp_1050_1538, color, 0x100);
        handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
        MoveTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, 0x0);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, iStack22);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack24, iStack22);
        uVar7 = (ulong)local_hdc_1 << 0x10;
        LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack24, 0x0);
        uVar8 = uVar7 & 0xffff0000 | (ulong)local_hdc_1;
        uVar7 = 0x0;
        count = (char *)LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, 0x0);
        if(is_iconic == 0x0)
        {
            iVar3 = iVar4->field_0xe - iVar4->field_0x12;
            uVar7 = (ulong)local_hdc_1 << 0x10;
            MoveTo16((HDC16)s_tile2_bmp_1050_1538, iVar3, 0x0);
            uVar7 = uVar7 & 0xffff0000 | (ulong)local_hdc_1;
            count = (char *)LineTo16((HDC16)s_tile2_bmp_1050_1538, iVar3, iStack22);
        }
        ppcVar1 = (code **)((int)*iVar4->field_0x6 + 0x18);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538, iVar4->field_0x6, uVar7, uVar8);
        if(*count != '\0')
        {
            SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
            SetTextColor16((HDC16)s_tile2_bmp_1050_1538, color);
            str   = (LPCSTR)lstrlen16((LPCSTR)s_tile2_bmp_1050_1538);
            DVar6 = GetTextExtent16((HDC16)s_tile2_bmp_1050_1538, str, (INT16)count);
            iVar3 = (int)(DVar6 >> 0x10);
            if(is_iconic == 0x0)
            {
                iStack66 = (iStack26 - iStack30) / 0x2 - iVar3 / 0x2;
            }
            else
            {
                iStack66 = iStack24 / 0x2 - iVar3 / 0x2;
            }
            TextOut16((HDC16)s_tile2_bmp_1050_1538, (INT16)str, (INT16)count, str_00, iStack66);
        }
        HStack12 = SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, HStack12);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, local_hdc_1);
    }
    return;
}

void __stdcall16far unk_draw_op_1020_7f7a(astruct_20 *param_1, UINT16 param_2, ULONG param_3)

{
    undefined2  uVar1;
    HGDIOBJ16   HVar2;
    HCURSOR16   HVar3;
    uchar      *puVar4;
    astruct_20 *iVar4;
    int         unaff_DI;
    undefined2  uVar5;
    UINT16      unaff_SS;
    astruct_20 *paVar6;
    ushort     *puVar7;
    undefined2  in_stack_0000000e;

    paVar6                                   = unk_draw_op_1008_61b2(param_1, param_2, (UINT16)param_3, CONCAT22(in_stack_0000000e, param_3._2_2_), unaff_SS);
    puVar4                                   = (uchar *)((ulong)paVar6 >> 0x10);
    uVar5                                    = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                                    = (astruct_20 *)param_1;
    ((astruct_20 *)(iVar4 + 0x1))->field_0x0 = 0x389a;
    iVar4[0x1].field_0x2                     = 0x1008;
    ((astruct_20 *)(iVar4 + 0x1))->field_0x0 = 0x3aa8;
    iVar4[0x1].field_0x2                     = 0x1008;
    iVar4[0x1].field_0x4                     = 0x0;
    iVar4[0x1].field_0x8                     = 0x0;
    iVar4[0x1].field_0xa                     = 0x0;
    param_1->field_0x0                       = 0x82bc;
    iVar4->field_0x2                         = 0x1020;
    ((astruct_20 *)(iVar4 + 0x1))->field_0x0 = 0x8358;
    iVar4[0x1].field_0x2                     = 0x1020;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x5b)), s_VrMode_1050_4422);
    HVar2                     = GetStockObject16(0x1000);
    iVar4->hgdiobj_field_0xc6 = HVar2;
    HVar3                     = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538, (LPCSTR)0x7f00);
    iVar4->hcursor_field_0xc4 = HVar3;
    iVar4->field_0xc8         = 0x2028;
    iVar4->field_0xac         = 0x47000000;
    iVar4->field_0xbc         = *(UINT16 *)(param_3._2_2_ + 0x8);
    puVar7                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, puVar4, unaff_DI);
    uVar1                     = (undefined2)((ulong)puVar7 >> 0x10);
    iVar4->field_0xb4         = 0x0;
    iVar4->field_0xb6         = 0x0;
    iVar4->field_0xb8         = *(UINT16 *)((int)puVar7 + 0xa);
    iVar4->field_0xba         = *(UINT16 *)((int)puVar7 + 0xc);
    iVar4->field_0xca         = (UINT16)param_3;
    win_ui_reg_class_1008_96d2(param_1, 0x1008, unaff_SS);
    return;
}

void __stdcall16far realize_palette_1020_8128(ulong param_1, int param_2, HGDIOBJ16 param_3, ushort param_4)

{
    code      **ppcVar1;
    undefined4  uVar2;
    undefined  *puVar3;
    undefined4 *puVar4;
    undefined4 *puVar5;
    uint        extraout_DX;
    int         iVar6;
    int         iVar7;
    undefined2  uVar8;
    undefined2  uVar9;
    undefined   local_12[0x8];
    undefined2  uStack10;
    undefined2  uStack8;
    undefined4 *puStack6;

    if(param_2 != 0x0)
    {
        uVar8    = (undefined2)(param_1 >> 0x10);
        iVar6    = (int)param_1;
        uVar2    = *(undefined4 *)(iVar6 + 0xe6);
        uVar9    = (undefined2)((ulong)uVar2 >> 0x10);
        iVar7    = (int)uVar2;
        puVar5   = (undefined4 *)*(undefined4 *)(iVar7 + 0xa);
        ppcVar1  = (code **)((int)*puVar5 + 0x18);
        puStack6 = puVar5;
        (**ppcVar1)(param_3, (int)puVar5, *(undefined2 *)(iVar7 + 0xc));
        uStack8 = SUB42(puVar5, 0x0);
        UnrealizeObject16(param_3);
        uVar2    = *(undefined4 *)(iVar6 + 0xe6);
        uVar8    = *(undefined2 *)((int)uVar2 + 0x14);
        uStack10 = uVar8;
        RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
        pass1_1008_57a4((ulong *)CONCAT22(param_4, local_12), param_1 & 0xffff0000 | (ulong)(iVar6 + 0xd2));
        while(true)
        {
            puVar3 = local_12;
            pass1_1008_5b12(puVar3, param_4);
            if((extraout_DX | (uint)puVar3) == 0x0)
                break;
            uVar9   = *(undefined2 *)(puVar3 + 0x6);
            puVar4  = (undefined4 *)*(undefined4 *)(puVar3 + 0x4);
            ppcVar1 = (code **)((int)*puVar4 + 0x90);
            (**ppcVar1)(0x1008, puVar4, uVar9, 0x1, uVar8);
        }
    }
    return;
}

void __stdcall16far win_ui_palette_op_1020_81c0(HWND16 param_1)

{
    astruct_13 *in_struct_1;
    BOOL16      b_force_background;
    HPALETTE16  b_force_background_00;
    UINT16      UVar1;
    uint        uVar2;
    undefined2  uVar3;
    uint        uStack6;

    uVar3       = (undefined2)((ulong)_PTR_LOOP_1050_4230 >> 0x10);
    in_struct_1 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
    uVar2       = *(uint *)((int)_PTR_LOOP_1050_4230 + 0x10);
    uStack6     = (uint)in_struct_1;
    if((uVar2 | uStack6) == 0x0)
    {
        return;
    }
    b_force_background = GetDC16(param_1);
    create_palette_1008_4e38(in_struct_1, 0x1008, uVar2);
    b_force_background_00 = SelectPalette16(0x1008, 0x0, b_force_background);
    UVar1                 = RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x1, b_force_background_00);
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    if(0x0 < (int)UVar1)
    {
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1), 0x0);
    }
    return;
}

void __stdcall16far pass1_1020_6466(ushort *param_1, ushort param_2, ushort param_3)

{
    astruct_585 *iVar1;
    uint         uVar1;

    uVar1            = (uint)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_585 *)param_1;
    *param_1         = 0x67c2;
    iVar1->field_0x2 = 0x1020;
    if(iVar1->field_0x14 != 0x0)
    {
        param_2 = 0x1010;
        pass1_1010_1ea6(iVar1->field_0x14, (ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10, param_3);
    }
    palette_op_1020_92c4(param_1, param_2);
    return;
}
void __stdcall16far mix_draw_op_1020_650c(astruct_7 *param_1, HWND16 param_2, ushort param_3)

{
    code        **ppcVar1;
    undefined4    uVar2;
    int           iVar3;
    int           iVar4;
    int           iVar5;
    undefined2    uVar6;
    undefined2    uVar7;
    PAINTSTRUCT16 local_28;
    int           iStack8;
    undefined4   *puStack6;

    uVar6    = (undefined2)((ulong)param_1 >> 0x10);
    iVar3    = (int)param_1;
    uVar2    = *(undefined4 *)(iVar3 + 0x14);
    puStack6 = (undefined4 *)*(undefined4 *)((int)uVar2 + 0xa);
    if((*(int *)(iVar3 + 0x10) != 0x0) || (uVar2 = *(undefined4 *)(iVar3 + 0x14), *(int *)((int)uVar2 + 0x24) != 0x0))
    {
        draw_op_1020_9364(param_1, param_2, param_3);
        if(*(long *)(iVar3 + 0x24) != 0x0)
        {
            uVar2   = *(undefined4 *)(iVar3 + 0x24);
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x24) + 0x14);
            (**ppcVar1)(param_2, (int)uVar2, (int)((ulong)uVar2 >> 0x10));
        }
    }
    iStack8 = 0x0;
    do
    {
        iVar4 = iVar3 + 0x18;
        iVar5 = iStack8 * 0x4;
        if(*(long *)(iVar4 + iVar5) != 0x0)
        {
            uVar2   = *(undefined4 *)(iVar4 + iVar5);
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + iVar5) + 0x8);
            (**ppcVar1)(param_2, (char)uVar2, (int)((ulong)uVar2 >> 0x10), (int)puStack6, (int)((ulong)puStack6 >> 0x10));
        }
        iStack8 = iStack8 + 0x1;
    } while(iStack8 < 0x5);
    uVar7 = *(undefined2 *)(iVar3 + 0x4);
    BeginPaint16(param_2, &local_28);
    ppcVar1 = (code **)((int)*puStack6 + 0x4);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538, (int)puStack6, (int)((ulong)puStack6 >> 0x10), 0x0, 0x0, iVar3 + 0xa, uVar6, uVar7);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_28);
    return;
}
void __stdcall16far realize_palette_1020_6896(ulong param_1, int param_2, HGDIOBJ16 param_3)

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
        puVar1  = (undefined4 *)*(undefined4 *)(iVar4 + 0x24);
        ppcVar2 = (code **)((int)*puVar1 + 0x18);
        (**ppcVar2)(param_3, (int)puVar1, *(undefined2 *)(iVar4 + 0x26));
        UnrealizeObject16(param_3);
        RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    }
    return;
}


void __stdcall16far pass1_1020_68de(ulong param_1, ushort param_2)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0xf6) != 0x0)
    {
        invalidate_rect_1020_735a(*(ulong *)((int)param_1 + 0xf6), param_2);
    }
    return;
}


void __stdcall16far pt_in_rect_1020_68fc(ulong *param_1, undefined2 param_2, ushort param_3)

{
    code     **ppcVar1;
    ushort     uVar2;
    BOOL16     BVar3;
    ulong      uVar4;
    undefined2 uVar5;
    POINT16    PStack6;

    PStack6 = (POINT16)CONCAT22(param_2, param_3);
    uVar5   = (undefined2)((ulong)param_1 >> 0x10);
    uVar2   = pass1_1018_31d0(*(ulong *)((int)param_1 + 0xf2));
    if(uVar2 != 0x0)
    {
        uVar4 = *(ulong *)((int)param_1 + 0xf2);
        uVar4 = uVar4 & 0xffff0000 | (ulong)((int)uVar4 + 0x16c);
        BVar3 = PtInRect16((RECT16 *)0x1018, PStack6);
        if(BVar3 != 0x0)
        {
            ppcVar1 = (code **)((int)*param_1 + 0x40);
            (**ppcVar1)((int)s_tile2_bmp_1050_1538, param_1, 0xef, uVar4);
        }
    }
    return;
}

void __stdcall16far destroy_icon_1020_6bd2(ulong param_1, uchar param_2, HICON16 param_3)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;
    undefined2  uVar6;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar6 = *(undefined2 *)(iVar4 + 0xc2);
    DestroyIcon16(param_3);
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

HGDIOBJ16 __stdcall16far draw_op_1020_7070(INT16 param_1, uint param_2)

{
    HGDIOBJ16 HVar1;

    HVar1 = GetStockObject16(param_1);
    if(_PTR_LOOP_1050_441e == 0x0)
    {
        _PTR_LOOP_1050_441e = 0x1000002;
    }
    if(0x6 < param_2)
    {
        return 0x0;
    }
    SetTextColor16((HDC16)s_tile2_bmp_1050_1538, (COLORREF)_PTR_LOOP_1050_441e);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return HVar1;
}

void __stdcall16far palette_op_1020_7270(ushort *param_1, HDC16 param_2)

{
    uint        uVar1;
    uint        uVar2;
    HPALETTE16  HVar3;
    int         iVar4;
    uint        uVar5;
    ushort      unaff_SS;
    astruct_18 *paStack8;

    uVar5                        = (uint)((ulong)param_1 >> 0x10);
    iVar4                        = (int)param_1;
    *param_1                     = 0x754c;
    *(undefined2 *)(iVar4 + 0x2) = 0x1020;
    if(*(long *)(iVar4 + 0x1c) != 0x0)
    {
        param_2 = 0x1010;
        pass1_1010_1ea6(*(ulong *)(iVar4 + 0x1c), (ulong)param_1 & 0xffff | (ulong)uVar5 << 0x10, unaff_SS);
    }
    uVar1    = *(uint *)(iVar4 + 0x14);
    uVar2    = *(uint *)(iVar4 + 0x16);
    paStack8 = (astruct_18 *)CONCAT22(uVar2, uVar1);
    if((uVar2 | uVar1) != 0x0)
    {
        pass1_1008_5118(CONCAT22(uVar2, uVar1));
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paStack8, 0x1000);
    }
    HVar3                         = SelectPalette16(param_2, 0x0, *(BOOL16 *)(iVar4 + 0x20));
    *(HPALETTE16 *)(iVar4 + 0x20) = HVar3;
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    *param_1                     = 0x3ab0;
    *(undefined2 *)(iVar4 + 0x2) = 0x1008;
    *param_1                     = 0x389a;
    *(undefined2 *)(iVar4 + 0x2) = 0x1008;
    return;
}

void __stdcall16far invalidate_rect_1020_735a(ulong param_1, HWND16 param_2)

{
    InvalidateRect16(param_2, (RECT16 *)0x0, (int)*(undefined4 *)((int)param_1 + 0x1c) + 0x16c);
    return;
}


BOOL16 __stdcall16far win_ui_op_1020_737a(ULONG param_1, HWND16 param_2, UINT16 param_3)

{
    uint          uVar1;
    code        **ppcVar2;
    undefined4    uVar3;
    BOOL16        BVar4;
    undefined    *puVar5;
    undefined4   *puVar6;
    uint          in_DX;
    uint          uVar7;
    int           iVar8;
    undefined2    uVar9;
    undefined2    uVar10;
    undefined2    uVar11;
    astruct_18   *paStack78;
    undefined     local_42[0x6];
    undefined4   *local_brush_handle;
    int           iStack56;
    HWND16        HStack54;
    HWND16        HStack52;
    int           iStack50;
    RECT16        local_30;
    int           iStack44;
    int           iStack42;
    RECT16       *local_rect;
    BOOL16        BStack38;
    HDC16         local_24;
    PAINTSTRUCT16 local_paint_struct;

    uVar9    = (undefined2)(param_1 >> 0x10);
    iVar8    = (int)param_1;
    uVar11   = *(undefined2 *)(iVar8 + 0x4);
    local_24 = BeginPaint16(param_2, &local_paint_struct);
    uVar10   = *(undefined2 *)(iVar8 + 0x4);
    BVar4    = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
    if(BVar4 == 0x0)
    {
        uVar3              = *(undefined4 *)(iVar8 + 0x1c);
        puVar6             = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
        local_brush_handle = puVar6;
        pass1_1018_2e5e(param_3, (ushort)puVar6, in_DX, *(ulong *)(iVar8 + 0x1c));
        local_30 = (ulong)puVar6 & 0xffff | (ulong)in_DX << 0x10;
        pass1_1008_3e54((ushort *)CONCAT13((char)(param_3 >> 0x8), CONCAT12((char)param_3, local_42)), 0x0, 0x35, 0xc);
        if(*(long *)(iVar8 + 0x14) != 0x0)
        {
            pass1_1008_5236(*(ulong *)(iVar8 + 0x14));
        }
        if(local_30 != 0x0)
        {
            uVar1     = *(uint *)(iVar8 + 0x14);
            uVar7     = *(uint *)(iVar8 + 0x16);
            paStack78 = (astruct_18 *)CONCAT22(uVar7, uVar1);
            if((uVar7 | uVar1) != 0x0)
            {
                pass1_1008_5118(CONCAT22(uVar7, uVar1));
                fn_ptr_1000_17ce(paStack78, 0x1000);
            }
            puVar5 = local_42;
            pass1_1008_8ce4(local_30, (ushort *)CONCAT22(param_3, puVar5), (ulong)local_brush_handle, param_3);
            *(int *)(iVar8 + 0x14)  = (int)puVar5;
            *(uint *)(iVar8 + 0x16) = uVar7;
        }
        ppcVar2 = (code **)((int)*local_brush_handle + 0x4);
        (**ppcVar2)(0x1008, (int)local_brush_handle, (int)((ulong)local_brush_handle >> 0x10), 0x0, 0x0);
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x18) + 0x94);
        (**ppcVar2)(0x1008, *(undefined4 *)(iVar8 + 0x18), 0x1);
        HStack52 = GetDlgItem16(0x1008, 0x1797);
        if(HStack52 != 0x0)
        {
            ShowWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        }
    }
    else
    {
        if(PTR_LOOP_1050_0010 == (undefined *)0x0)
        {
            ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x1c) + 0x2c);
            (**ppcVar2)((int)s_tile2_bmp_1050_1538, *(undefined4 *)(iVar8 + 0x1c), uVar10, uVar11);
            BStack38 = BVar4;
            if(BVar4 != 0x0)
            {
                local_rect = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
                GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_30);
                local_brush_handle = (undefined4 *)0x0;
                iStack56           = (iStack44 - local_30.x) + -0x1;
                HStack54           = (iStack42 - local_30.y) - 0x1;
                HStack52           = HStack54;
                iStack50           = iStack56;
                FillRect16((HDC16)s_tile2_bmp_1050_1538, local_rect, (HBRUSH16)&local_brush_handle);
                DrawIcon16((HDC16)s_tile2_bmp_1050_1538, BStack38, 0x2, 0x2);
            }
        }
    }
    BVar4 = EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_paint_struct);
    return BVar4;
}

void __stdcall16far unk_draw_op_1020_3da4(astruct_24 *param_1, ULONG param_2)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    int16_t     iVar4;
    HGDIOBJ16   HVar5;
    HDC16      *pHVar6;
    uchar      *in_DX;
    undefined2  uVar7;
    int         iVar8;
    int         unaff_DI;
    undefined2  uVar9;
    undefined2  unaff_CS;
    ushort      unaff_SS;
    ushort     *puVar10;
    HDC16       local_4;
    astruct_24 *iVar9;
    astruct_24 *uVar8;

    get_sys_metrics_1020_7c1a(&param_1->field_0x0, param_2, unaff_CS);
    uVar9                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar8                         = (int)param_1;
    *(undefined4 *)(iVar8 + 0x14) = 0x0;
    param_1->field_0x0            = 0x408a;
    *(undefined2 *)(iVar8 + 0x2)  = 0x1020;
    puVar10                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, unaff_SS, in_DX, unaff_DI);
    uVar7                         = (undefined2)((ulong)puVar10 >> 0x10);
    *(undefined2 *)(iVar8 + 0x14) = (int)puVar10;
    *(undefined2 *)(iVar8 + 0x16) = uVar7;
    ppcVar2                       = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x14) + 0x4);
    (**ppcVar2)(0x1010, *(undefined2 *)(iVar8 + 0x14), uVar7, 0x0, param_1);
    local_4                      = GetDC16(0x1010);
    iVar4                        = SetMapMode16((HDC16)s_tile2_bmp_1050_1538, 0x1);
    *(int16_t *)(iVar8 + 0x1e)   = iVar4;
    HVar5                        = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    HVar5                        = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar5);
    *(HGDIOBJ16 *)(iVar8 + 0x18) = HVar5;
    HVar5                        = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    HVar5                        = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar5);
    *(HGDIOBJ16 *)(iVar8 + 0x1a) = HVar5;
    uVar3                        = *(undefined4 *)(iVar8 + 0x14);
    puVar1                       = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
    pHVar6                       = &local_4;
    ppcVar2                      = (code **)((int)*puVar1 + 0x8);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)puVar1, (int)((ulong)puVar1 >> 0x10), pHVar6);
    *(HDC16 **)(iVar8 + 0x1c)     = pHVar6;
    uVar3                         = *(undefined4 *)(iVar8 + 0x14);
    *(HDC16 *)((int)uVar3 + 0x4c) = local_4;
    return;
}

void __stdcall16far win_ui_palette_op_1020_3e84(astruct_16 *param_1)

{
    astruct_16 *iVar1;
    UINT16      uVar1;
    ushort      unaff_SS;

    uVar1                  = (UINT16)((ulong)param_1 >> 0x10);
    iVar1                  = (astruct_16 *)param_1;
    *(undefined2 *)param_1 = 0x408a;
    iVar1->field_0x2       = 0x1020;
    pass1_1010_1ea6(iVar1->field_0x14, (ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10, unaff_SS);
    SelectObject16(0x1010, iVar1->field_0x18);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, iVar1->field_0x1a);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, iVar1->field_0x1c);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SetMapMode16((HDC16)s_tile2_bmp_1050_1538, iVar1->field_0x1e);
    *(undefined2 *)param_1 = 0x3ab0;
    iVar1->field_0x2       = 0x1008;
    *(undefined2 *)param_1 = 0x389a;
    iVar1->field_0x2       = 0x1008;
    return;
}


void __stdcall16far validate_rect_1020_3f12(ulong param_1, int param_2, HWND16 param_3)

{
    RECT16     local_a;
    undefined4 uStack6;

    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x14) = 0x0;
        return;
    }
    if(param_2 != 0xd)
    {
        return;
    }
    local_a = (RECT16)0x8000e;
    uStack6 = 0x1100116;
    InvalidateRect16(param_3, (RECT16 *)0x0, (BOOL16)&local_a);
    local_a = (RECT16)0xf10000;
    uStack6 = 0x1220030;
    ValidateRect16((HWND16)s_tile2_bmp_1050_1538, &local_a);
    local_a = (RECT16)0xf100f5;
    uStack6 = 0x1220127;
    ValidateRect16((HWND16)s_tile2_bmp_1050_1538, &local_a);
    return;
}


void __stdcall16far mixed_draw_op_1020_3fa0(ulong param_1, HWND16 param_2, ushort param_3)

{
    ulong         uVar1;
    code        **ppcVar2;
    undefined4    uVar3;
    int           iVar4;
    undefined2    uVar5;
    undefined2    uVar6;
    int           iStack56;
    ulong         uStack54;
    undefined4    local_32;
    int           iStack46;
    ulong         uStack44;
    undefined4   *puStack40;
    undefined2    local_24;
    PAINTSTRUCT16 local_22;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar6 = *(undefined2 *)(iVar4 + 0x4);
    BeginPaint16(param_2, &local_22);
    uVar3     = *(undefined4 *)(iVar4 + 0x14);
    local_24  = *(undefined2 *)((int)uVar3 + 0x4c);
    uVar3     = *(undefined4 *)(iVar4 + 0x14);
    puStack40 = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
    ppcVar2   = (code **)((int)*puStack40 + 0x4);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)puStack40, (int)((ulong)puStack40 >> 0x10), 0x0, &local_24, param_3, uVar6);
    uVar3    = *(undefined4 *)(iVar4 + 0x14);
    iStack46 = *(int *)((int)uVar3 + 0x44);
    uVar3    = *(undefined4 *)(iVar4 + 0x14);
    uStack44 = *(ulong *)((int)uVar3 + 0x40);
    uVar1    = *(ulong *)(iVar4 + 0x14);
    pass1_1008_3e94((ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x3a)), (ushort *)CONCAT22(param_3, &local_32), (ushort *)CONCAT22(param_3, (int)&local_32 + 0x2));
    uStack54 = uStack44;
    for(iStack56 = 0x0; iStack56 < iStack46; iStack56 = iStack56 + 0x1)
    {
        draw_rect_1020_40ce(uStack54, (int)local_32, (int)((ulong)local_32 >> 0x10), param_3);
        uStack54 = uStack54 & 0xffff0000 | (ulong)((int)uStack54 + 0x18);
    }
    EndPaint16(0x1008, &local_22);
    return;
}


astruct_18 *__stdcall16far pass1_1020_4064(astruct_18 *param_1, byte param_2)

{
    win_ui_palette_op_1020_3e84((astruct_16 *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far draw_rect_1020_40ce(ulong param_1, int param_2, int param_3, ushort param_4)

{
    int       iVar1;
    HGDIOBJ16 HVar2;
    HPEN16    handle;
    int       local_6;
    int       local_4;

    pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)), (ushort *)CONCAT22(param_4, &local_6), (ushort *)CONCAT22(param_4, &local_4));
    pass1_1008_3e94((ushort *)param_1, (ushort *)CONCAT22(param_4, &local_6), (ushort *)CONCAT22(param_4, &local_4));
    iVar1 = *(int *)((int)param_1 + 0xa);
    Ellipse16(0x1008, iVar1 + local_6 + param_2, iVar1 + local_4 + param_3, (local_6 - *(int *)((int)param_1 + 0xa)) + param_2, (local_4 - *(int *)((int)param_1 + 0xa)) + param_3);
    if((*(byte *)((int)param_1 + 0xe) & 0x1) != 0x0)
    {
        HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar2);
        handle = CreatePen16((INT16)s_tile2_bmp_1050_1538, 0xf9, 0x100);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
        Rectangle16((HDC16)s_tile2_bmp_1050_1538, local_6 + param_2 + 0x5, local_4 + param_3 + 0x5, local_6 + param_2 + -0x5, local_4 + param_3 + -0x5);
        HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar2);
        HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar2);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_draw_op_1020_41c8(astruct_20 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    code      **ppcVar1;
    HCURSOR16   HVar2;
    undefined2 *puVar3;
    uchar      *extraout_DX;
    uchar      *puVar4;
    undefined2  uVar6;
    astruct_64 *uVar5;
    int         unaff_DI;
    undefined2  uVar7;
    ushort      unaff_SS;
    ushort     *puVar8;
    undefined  *puVar9;
    undefined  *puVar10;
    undefined  *puVar11;

    unk_draw_op_1020_7f7a(param_1, 0x8, CONCAT22(param_3, param_2));
    uVar7                                          = (undefined2)((ulong)param_1 >> 0x10);
    uVar5                                          = (astruct_64 *)param_1;
    uVar5->field_0xee                              = 0x0;
    uVar5->field_0xf0                              = 0x0;
    uVar5->field_0xf2                              = 0x0;
    uVar5->field_0xf4                              = 0x1;
    uVar5->field_0xf6                              = 0x0;
    uVar5->field_0xfa                              = (undefined4 *)0x0;
    uVar5->field_0xfe                              = 0x0;
    uVar5->field_0x102                             = 0x0;
    uVar5->field_0x106                             = 0x0;
    uVar5->field_0x10a                             = 0x0;
    uVar5->field_0x108                             = 0x0;
    uVar5->field_0x10c                             = 0x0;
    uVar5->field_0x110                             = 0x0;
    uVar5->field_0x10e                             = 0x0;
    uVar5->field_0x112                             = 0x0;
    uVar5->field_0x114                             = 0x0;
    uVar5->field_0x116                             = 0x0;
    param_1->field_0x0                             = 0x623c;
    uVar5->field_0x2                               = 0x1020;
    uVar5->field_0xe2                              = 0x62d8;
    uVar5->field_0xe4                              = 0x1020;
    puVar4                                         = extraout_DX;
    puVar11                                        = PTR_LOOP_1050_038c;
    HVar2                                          = LoadCursor16(param_4, (LPCSTR)((int)s__s__ld_1050_019c + 0x2));
    uVar5->field_0xf0                              = HVar2;
    puVar10                                        = PTR_LOOP_1050_038c;
    HVar2                                          = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538, (LPCSTR)((int)s__s__ld_1050_019c + 0x3));
    uVar5->field_0xf2                              = HVar2;
    puVar9                                         = PTR_LOOP_1050_038c;
    PTR_LOOP_1050_0398                             = (undefined *)LoadAccelerators16((HINSTANCE16)s_tile2_bmp_1050_1538, (LPCSTR)s_OpAccel_1050_43e8);
    puVar8                                         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, unaff_SS, puVar4, unaff_DI);
    *(int *)&uVar5->field_0xfa                     = (int)puVar8;
    *(undefined2 *)((int)&uVar5->field_0xfa + 0x2) = (int)((ulong)puVar8 >> 0x10);
    if(param_1 == (astruct_20 *)0x0)
    {
        puVar3 = (undefined2 *)0x0;
        uVar6  = 0x0;
    }
    else
    {
        puVar3 = &uVar5->field_0xe2;
        uVar6  = uVar7;
    }
    ppcVar1 = (code **)((int)*uVar5->field_0xfa + 0x4);
    (**ppcVar1)(0x1010, uVar5->field_0xfa, 0x0, puVar3, uVar6, puVar9, puVar10, puVar11);
    uVar5->field_0xe6 = uVar5->field_0xfa;
    return;
}


void __stdcall16far destroy_cursor_1020_42f4(ushort *param_1, HMENU16 param_2)

{
    int        iVar1;
    undefined2 uVar2;
    HMENU16    h_cursor;

    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *param_1                      = 0x623c;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1020;
    *(undefined2 *)(iVar1 + 0xe2) = 0x62d8;
    *(undefined2 *)(iVar1 + 0xe4) = 0x1020;
    h_cursor                      = param_2;
    if(*(int *)(iVar1 + 0x106) != 0x0)
    {
        h_cursor = (HMENU16)s_tile2_bmp_1050_1538;
        DestroyMenu16(param_2);
    }
    DestroyCursor16(h_cursor);
    DestroyCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    pass1_1020_808e(param_1);
    return;
}

void __stdcall16far pass1_1020_2c72(ulong param_1, ushort param_2, ushort param_3)

{
    draw_op_1020_30be(*(ulong *)((int)param_1 + 0xf6), param_2, param_3);
    return;
}

void __stdcall16far destroy_icon_1020_2c88(ulong param_1, HICON16 param_2)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;
    undefined2  uVar6;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar6 = *(undefined2 *)(iVar4 + 0xc2);
    DestroyIcon16(param_2);
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

void __stdcall16far load_draw_op_1020_2ede(ushort *param_1, ulong param_2, ushort param_3)

{
    undefined4  uVar1;
    code      **ppcVar2;
    HDC16       HVar3;
    int         iVar4;
    HPEN16      handle;
    HGDIOBJ16   HVar5;
    uchar      *in_DX;
    int         iVar6;
    int         unaff_DI;
    undefined2  uVar7;
    ushort      unaff_SS;
    ushort     *puVar8;
    astruct_76 *paVar9;
    ulong       uVar10;

    get_sys_metrics_1020_7c1a(param_1, param_2, param_3);
    uVar7                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar6                         = (int)param_1;
    *(undefined4 *)(iVar6 + 0x14) = 0x0;
    *(undefined2 *)(iVar6 + 0x18) = 0x0;
    *(undefined2 *)(iVar6 + 0x1a) = 0x0;
    *(undefined2 *)(iVar6 + 0x1c) = 0x0;
    *(undefined2 *)(iVar6 + 0x1e) = 0x0;
    *(undefined2 *)(iVar6 + 0x20) = 0x0;
    *param_1                      = 0x363c;
    *(undefined2 *)(iVar6 + 0x2)  = 0x1020;
    puVar8                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, *(ushort *)((int)param_2 + 0xfc), unaff_SS, in_DX, unaff_DI);
    *(undefined2 *)(iVar6 + 0x14) = (int)puVar8;
    *(undefined2 *)(iVar6 + 0x16) = (int)((ulong)puVar8 >> 0x10);
    uVar1                         = *(undefined4 *)(iVar6 + 0x14);
    ppcVar2                       = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x14) + 0x4);
    (**ppcVar2)(0x1010, (int)uVar1, (int)((ulong)uVar1 >> 0x10), 0x0, param_1);
    paVar9                   = (astruct_76 *)pass1_1018_0a50(*(ulong *)(iVar6 + 0x14));
    uVar10                   = pass1_1008_4772(paVar9);
    HVar3                    = CreateDC16((LPCSTR)0x1008, (LPCSTR)uVar10, (LPCSTR)(uVar10 >> 0x10), (DEVMODEA *)0x0);
    *(HDC16 *)(iVar6 + 0x18) = HVar3;
    iVar4                    = iVar6 + 0x18;
    ppcVar2                  = (code **)((int)*(undefined4 *)paVar9 + 0x8);
    (**ppcVar2)();
    *(int *)(iVar6 + 0x20)       = iVar4;
    uVar1                        = *(undefined4 *)(iVar6 + 0x14);
    uVar1                        = *(undefined4 *)((int)uVar1 + 0x64);
    handle                       = CreatePen16((INT16)s_tile2_bmp_1050_1538, (INT16)uVar1, (COLORREF)((ulong)uVar1 >> 0x10));
    *(HPEN16 *)(iVar6 + 0x1a)    = handle;
    HVar5                        = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    *(HGDIOBJ16 *)(iVar6 + 0x1c) = HVar5;
    HVar5                        = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    HVar5                        = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar5);
    *(HGDIOBJ16 *)(iVar6 + 0x1e) = HVar5;
    return;
}

void __stdcall16far invalidate_rect_1020_3080(ulong param_1, int param_2, HWND16 param_3)

{
    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x14) = 0x0;
        return;
    }
    if((param_2 != 0x4) && (param_2 != 0x6))
    {
        return;
    }
    InvalidateRect16(param_3, (RECT16 *)0x0, 0x0);
    return;
}

void __stdcall16far draw_op_1020_30be(ulong param_1, HWND16 param_2, ushort param_3)

{
    code        **ppcVar1;
    undefined4    uVar2;
    BOOL16        BVar3;
    int           iVar4;
    undefined2    uVar5;
    HWND16        hwnd;
    undefined2    uVar6;
    undefined2    uVar7;
    undefined4   *local_3c;
    int           iStack56;
    int           iStack54;
    int           iStack52;
    int           iStack50;
    RECT16        local_30;
    int           iStack44;
    int           iStack42;
    RECT16       *pRStack40;
    int           iStack38;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    uVar5    = (undefined2)(param_1 >> 0x10);
    iVar4    = (int)param_1;
    uVar7    = *(undefined2 *)(iVar4 + 0x4);
    local_24 = BeginPaint16(param_2, &local_22);
    uVar6    = *(undefined2 *)(iVar4 + 0x4);
    hwnd     = (HWND16)s_tile2_bmp_1050_1538;
    BVar3    = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
    if(BVar3 == 0x0)
    {
        hwnd     = 0x1018;
        local_3c = (undefined4 *)pass1_1018_0a50(*(ulong *)(iVar4 + 0x14));
        ppcVar1  = (code **)((int)*local_3c + 0x8);
        (**ppcVar1)(0x1018, (int)local_3c, (int)((ulong)local_3c >> 0x10), &local_24, param_3, uVar6, uVar7);
        uVar2 = *(undefined4 *)(iVar4 + 0x14);
        if(*(int *)((int)uVar2 + 0x84) == 0x1)
        {
            unk_draw_op_1020_320e(param_1, local_24, param_3);
        }
        ppcVar1 = (code **)((int)*local_3c + 0x4);
        (**ppcVar1)(0x1018, (int)local_3c, (int)((ulong)local_3c >> 0x10), 0x0, 0x0, 0xdc, param_3);
        uVar2 = *(undefined4 *)(iVar4 + 0x14);
        if(*(int *)((int)uVar2 + 0x84) != 0x1)
        {
            unk_draw_op_1020_320e(param_1, local_24, param_3);
        }
        draw_op_1020_3488(param_1);
        ppcVar1 = (code **)((int)*local_3c + 0xc);
        (**ppcVar1)(0x1018, (int)local_3c, (int)((ulong)local_3c >> 0x10), &local_24, param_3);
    }
    else
    {
        if(PTR_LOOP_1050_0010 == (undefined *)0x0)
        {
            ppcVar1  = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x14) + 0x2c);
            iStack38 = (**ppcVar1)((int)s_tile2_bmp_1050_1538);
            if(iStack38 != 0x0)
            {
                pRStack40 = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
                GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_30);
                local_3c = (undefined4 *)0x0;
                iStack56 = (iStack44 - local_30.x) + -0x1;
                iStack54 = (iStack42 - local_30.y) + -0x1;
                iStack52 = iStack54;
                iStack50 = iStack56;
                FillRect16((HDC16)s_tile2_bmp_1050_1538, pRStack40, (HBRUSH16)&local_3c);
                hwnd = (HWND16)s_tile2_bmp_1050_1538;
                DrawIcon16((HDC16)s_tile2_bmp_1050_1538, iStack38, 0x2, 0x2);
            }
        }
    }
    EndPaint16(hwnd, &local_22);
    return;
}


void __stdcall16far unk_draw_op_1020_320e(ulong param_1, HDC16 param_2, ushort param_3)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    undefined4  uVar3;
    int         iVar4;
    int         iVar5;
    undefined2  uVar6;
    undefined2  uVar7;
    ulong       uVar8;
    DEVMODEA   *init_data;
    int         local_c;
    ulong       local_a;
    HDC16      *pHStack6;
    HDC16       local_4;

    local_4 = param_2;
    uVar6   = (undefined2)(param_1 >> 0x10);
    iVar4   = (int)param_1;
    uVar3   = *(undefined4 *)(iVar4 + 0x14);
    if(*(int *)((int)uVar3 + 0x84) == 0x1)
    {
        uVar3     = *(undefined4 *)(iVar4 + 0x14);
        uVar7     = (undefined2)((ulong)uVar3 >> 0x10);
        iVar5     = (int)uVar3;
        puVar1    = (undefined4 *)*(ulong *)(iVar5 + 0x24);
        init_data = (DEVMODEA *)0x0;
        uVar8     = pass1_1008_4772((astruct_76 *)((ulong)puVar1 & 0xffff | (ulong) * (uint *)(iVar5 + 0x26) << 0x10));
        local_4   = CreateDC16((LPCSTR)0x1008, (LPCSTR)uVar8, (LPCSTR)(uVar8 >> 0x10), init_data);
        pHStack6  = &local_4;
        ppcVar2   = (code **)((int)*puVar1 + 0x8);
        (**ppcVar2)((int)s_tile2_bmp_1050_1538, (int)puVar1, (int)((ulong)puVar1 >> 0x10), pHStack6, param_3);
    }
    pass1_1018_0d9a(*(ulong *)(iVar4 + 0x14), (ushort *)CONCAT22(param_3, &local_c), (ulong *)CONCAT22(param_3, &local_a));
    uVar3 = *(undefined4 *)(iVar4 + 0x14);
    draw_op_1020_33c0(param_1, *(ulong *)((int)uVar3 + 0x6c), local_c, local_a, 0x1, local_4, 0x1018);
    pass1_1018_1054(*(ulong *)(iVar4 + 0x14), (ushort *)CONCAT22(param_3, &local_c), (ulong *)CONCAT22(param_3, &local_a), param_3);
    uVar3 = *(undefined4 *)(iVar4 + 0x14);
    draw_op_1020_33c0(param_1, *(ulong *)((int)uVar3 + 0x74), local_c, local_a, 0x2, local_4, 0x1018);
    pass1_1018_1320(*(ulong *)(iVar4 + 0x14), (ushort *)CONCAT22(param_3, &local_c), (ulong *)CONCAT22(param_3, &local_a));
    uVar3 = *(undefined4 *)(iVar4 + 0x14);
    draw_op_1020_33c0(param_1, *(ulong *)((int)uVar3 + 0x68), local_c, local_a, 0x1, local_4, 0x1018);
    pass1_1018_15f6(*(ulong *)(iVar4 + 0x14), (ushort *)CONCAT22(param_3, &local_c), (ulong *)CONCAT22(param_3, &local_a));
    if(local_c != 0x0)
    {
        uVar3 = *(undefined4 *)(iVar4 + 0x14);
        draw_op_1020_33c0(param_1, *(ulong *)((int)uVar3 + 0x70), local_c, local_a, 0x1, local_4, 0x1018);
    }
    pass1_1018_108c(*(ulong *)(iVar4 + 0x14), (ushort *)CONCAT22(param_3, &local_c), (ulong *)CONCAT22(param_3, &local_a), param_3);
    if(local_c != 0x0)
    {
        uVar3 = *(undefined4 *)(iVar4 + 0x14);
        draw_op_1020_33c0(param_1, *(ulong *)((int)uVar3 + 0x78), local_c, local_a, 0x0, local_4, 0x1018);
    }
    uVar3 = *(undefined4 *)(iVar4 + 0x14);
    if(*(int *)((int)uVar3 + 0x84) == 0x1)
    {
        SelectPalette16(0x1018, 0x0, (BOOL16)pHStack6);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
    }
    return;
}


void __stdcall16far draw_op_1020_33c0(ulong param_1, ulong param_2, int param_3, ulong param_4, int param_5, ushort param_6, ushort param_7)

{
    HPEN16     pen_handle;
    HGDIOBJ16  object_handle;
    HBRUSH16   brush_handle;
    HGDIOBJ16  obj_handle_2;
    int        iVar1;
    undefined2 uVar2;
    undefined2 in_DX;
    undefined2 uVar3;
    HDC16      hdc;
    undefined2 unaff_SS;
    ushort     uVar4;
    int        iStack20;
    ushort    *puStack14;

    if(param_3 != 0x0)
    {
        pen_handle    = CreatePen16(param_7, (INT16)param_2, (COLORREF)(param_2 >> 0x10));
        object_handle = SelectObject16((HDC16)s_tile2_bmp_1050_1538, pen_handle);
        brush_handle  = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
        hdc           = (HDC16)s_tile2_bmp_1050_1538;
        obj_handle_2  = SelectObject16((HDC16)s_tile2_bmp_1050_1538, brush_handle);
        puStack14     = (ushort *)param_4;
        for(iStack20 = 0x0; iStack20 < param_3; iStack20 = iStack20 + 0x1)
        {
            uVar4 = (ushort)(param_1 >> 0x10);
            iVar1 = param_3;
            pass1_1020_3540((ushort)param_1, uVar4, param_5, puStack14, in_DX, unaff_SS);
            if(param_5 < 0x1)
            {
                uVar2 = 0x3;
            }
            else
            {
                uVar2 = 0x4;
            }
            uVar3 = in_DX;
            draw_polygon_1020_3602((ushort)param_1, uVar4, CONCAT22(iVar1, uVar2), hdc);
            hdc = 0x1000;
            fn_ptr_1000_17ce((astruct_18 *)CONCAT22(in_DX, iVar1), 0x1000);
            puStack14 = (ushort *)((ulong)puStack14 & 0xffff0000 | (ulong)((int)puStack14 + 0x6));
            in_DX     = uVar3;
        }
        SelectObject16(hdc, obj_handle_2);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, object_handle);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

void __stdcall16far draw_op_1020_3488(ULONG param_1)

{
    uint       uVar1;
    ulong      uVar2;
    undefined4 uVar3;
    HPEN16     handle;
    HGDIOBJ16  handle_00;
    HGDIOBJ16  HVar4;
    undefined2 uVar5;
    undefined2 unaff_SS;
    int        bottom;
    undefined4 local_a;
    ushort    *puStack6;

    uVar5    = (undefined2)(param_1 >> 0x10);
    uVar2    = *(ulong *)((int)param_1 + 0x14);
    puStack6 = (ushort *)(uVar2 & 0xffff0000 | (ulong)((int)uVar2 + 0x36));
    pass1_1008_3e94(puStack6, (ushort *)CONCAT22(unaff_SS, &local_a), (ushort *)CONCAT22(unaff_SS, (int)&local_a + 0x2));
    uVar2 = (ulong)(local_a._2_2_ - 0x3U) << 0x10;
    if((int)(local_a._2_2_ - 0x3U) < 0x0)
    {
        uVar2 = 0x0;
    }
    uVar1   = (int)local_a - 0x3;
    local_a = (ulong)uVar1;
    if((int)uVar1 < 0x0)
    {
        local_a = 0x0;
    }
    local_a   = uVar2 | local_a;
    uVar3     = *(undefined4 *)((int)param_1 + 0x14);
    uVar3     = *(undefined4 *)((int)uVar3 + 0x64);
    handle    = CreatePen16(0x1008, (INT16)uVar3, (COLORREF)((ulong)uVar3 >> 0x10));
    handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    HVar4     = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    HVar4     = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar4);
    bottom    = (int)(local_a >> 0x10);
    Rectangle16((HDC16)s_tile2_bmp_1050_1538, (int)local_a + 0x6, bottom + 0x6, (int)local_a, bottom);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar4);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    return;
}

void __stdcall16far draw_polygon_1020_3602(ushort param_1, ushort param_2, ulong param_3, HDC16 param_4)

{
    Polygon16(param_4, (POINT16 *)param_3, (INT16)(param_3 >> 0x10));
    return;
}


void __stdcall16far pass1_1020_3bd6(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int        iVar1;
    ushort     uVar2;
    undefined2 uVar3;
    undefined4 uVar4;

    uVar3 = (undefined2)(param_1 >> 0x10);
    uVar2 = (ushort)param_1;
    mixed_draw_op_1020_3fa0(*(ulong *)(uVar2 + 0xf6), param_3, param_4);
    if(*(int *)(uVar2 + 0x100) == 0x0)
    {
        *(undefined2 *)(uVar2 + 0x100) = 0x1;
        uVar4                          = *(undefined4 *)(uVar2 + 0xfa);
        if(*(int *)((int)uVar4 + 0x56) == 0x0)
        {
            iVar1 = 0x5;
        }
        else
        {
            iVar1 = 0x8;
        }
        uVar4                          = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar2 + 0x8), iVar1, param_2, uVar2, (ushort)&PTR_LOOP_1050_1038, param_4);
        *(undefined2 *)(uVar2 + 0x10e) = (int)uVar4;
        *(undefined2 *)(uVar2 + 0x110) = (int)((ulong)uVar4 >> 0x10);
    }
    return;
}

void __stdcall16far pass1_1020_3d08(ushort param_1,
                                    ushort param_2,
                                    ushort param_3,
                                    ushort param_4,
                                    ushort param_5,
                                    ushort param_6,
                                    ushort param_7,
                                    ushort param_8,
                                    ushort param_9,
                                    uchar  param_10,
                                    uchar  param_11,
                                    uchar  param_12,
                                    uchar  param_13,
                                    uchar  param_14,
                                    ulong  param_15,
                                    ushort param_16,
                                    ushort param_17,
                                    ushort param_18,
                                    ushort param_19)

{
    char       *pcVar1;
    byte       *pbVar2;
    bool        bVar3;
    bool        bVar4;
    code      **ppcVar5;
    undefined2 *puVar6;
    undefined4  uVar7;
    undefined4 *puVar8;
    ulong       uVar9;
    byte        bVar10;
    byte        bVar12;
    int         iVar13;
    byte        bVar18;
    char        cVar19;
    HDC16       HVar14;
    int16_t     iVar15;
    HGDIOBJ16   HVar16;
    undefined  *puVar17;
    uint        uVar20;
    uint        uVar21;
    byte        bVar22;
    byte        bVar23;
    uchar      *puVar24;
    byte        bVar25;
    char       *pcVar26;
    char       *pcVar27;
    undefined2  uVar28;
    undefined2  uVar29;
    bool        bVar30;
    bool        bVar31;
    ushort     *puVar32;
    code       *pcStack4;
    byte        bVar11;

    uVar9                 = CONCAT22(param_19, param_18);
    bVar22                = (char)param_2 + (char)(param_1 >> 0x8) + param_10;
    *(undefined *)param_6 = 0x3c;
    puVar24               = (uchar *)CONCAT11((char)(param_2 >> 0x8) + '<' + (*(byte *)(param_3 + param_5) < 0x20), bVar22);
    pcStack4              = switchD_1008: 1091 ::caseD_a7;
    iVar13                = 0x203d;
    pbVar2                = (byte *)(param_3 + 0x203d);
    *pbVar2               = *pbVar2 | bVar22;
    pbVar2                = (byte *)(param_3 + 0x203d);
    *pbVar2               = *pbVar2 & bVar22;
    pbVar2                = (byte *)(param_3 + 0x203d);
    *pbVar2               = *pbVar2 | bVar22;
    pbVar2                = (byte *)(param_3 + 0x203d);
    *pbVar2               = *pbVar2 | bVar22;
    pbVar2                = (byte *)(param_3 + 0x203d);
    *pbVar2               = *pbVar2 | bVar22;
    bVar10                = (byte)(param_6 + 0x2);
    bVar12                = 0x9 < (bVar10 & 0xf) | param_11;
    bVar11                = bVar10 + bVar12 * '\x06' & 0xf;
    pbVar2                = (byte *)(param_3 + 0x203d);
    *pbVar2               = *pbVar2 | bVar22;
    bVar10                = 0x9 < bVar11 | bVar12;
    uVar20                = CONCAT11((char)(param_6 + 0x2 >> 0x8) + bVar12 + bVar10, bVar11 + bVar10 * '\x06') & 0xff0f;
    pcVar27               = (char *)CONCAT11(0x79, (char)param_5 + -0x37);
    do
    {
        pcVar26 = pcVar27;
        pbVar2  = (byte *)(param_3 + iVar13);
        bVar23  = (byte)puVar24;
        *pbVar2 = *pbVar2 | bVar23;
        bVar12  = (byte)(uVar20 - 0x1);
        bVar3   = 0x9 < (bVar12 & 0xf);
        bVar22  = bVar3 | bVar10;
        pbVar2  = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar23;
        bVar4   = 0x9 < (bVar12 + bVar22 * '\x06' & 0xf);
        bVar18  = (char)(uVar20 - 0x1 >> 0x8) + bVar22 + (bVar4 | bVar22);
        pbVar2  = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar23;
        uVar20  = 0x0;
        bVar30  = &pcStack4 < (undefined *)*(uint *)(param_3 + iVar13);
        pbVar2  = (byte *)(param_3 + iVar13 + 0x896);
        bVar25  = (byte)param_3;
        bVar31  = CARRY1(*pbVar2, bVar25) || CARRY1(*pbVar2 + bVar25, bVar30);
        *pbVar2 = *pbVar2 + bVar25 + bVar30;
        pbVar2  = (byte *)(param_3 + iVar13 + 0x2038);
        bVar12  = *pbVar2;
        bVar11  = *pbVar2;
        *pbVar2 = bVar11 + bVar25 + bVar31;
        pcVar1  = (char *)(param_4 + iVar13);
        *pcVar1 = *pcVar1 + (char)((uint)puVar24 >> 0x8) + (CARRY1(bVar12, bVar25) || CARRY1(bVar11 + bVar25, bVar31));
        pcVar1  = (char *)(param_3 + iVar13 + -0x64);
        *pcVar1 = *pcVar1 + bVar18 + '\x01';
        pbVar2  = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar23;
        pcVar1  = pcVar26;
        pcVar27 = pcVar26 + 0x1;
        bVar30  = *pcVar1 != '\0';
        if(-*pcVar1 < '\0')
        {
            pcVar1  = (char *)(param_4 + 0x37);
            *pcVar1 = *pcVar1 + bVar25 + bVar30;
            pbVar2  = (byte *)(param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar23;
            iVar13  = iVar13 + -0x1;
            pbVar2  = (byte *)(param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar23;
            pbVar2  = (byte *)(param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar23;
            puVar24 = puVar24 + -0x1;
            pbVar2  = (byte *)(param_3 + iVar13);
            bVar12  = (byte)puVar24;
            *pbVar2 = *pbVar2 | bVar12;
            if(*pbVar2 == 0x0)
            {
                pbVar2  = (byte *)(param_3 + iVar13);
                *pbVar2 = *pbVar2 & bVar12;
            code_r0x10203d96:
                pbVar2   = (byte *)(param_3 + iVar13);
                *pbVar2  = *pbVar2 | (byte)puVar24;
                pbVar2   = (byte *)(param_3 + iVar13);
                *pbVar2  = *pbVar2 & (byte)puVar24;
                pcVar27  = pcVar26 + 0x2;
                uVar21   = (uint)puVar24 & 0xff;
                puVar24  = (uchar *)(uVar21 | (uint)(byte)((char)((uint)puVar24 >> 0x8) * '\x02' + ((byte)uVar20 < 0x20)) << 0x8);
                pbVar2   = (byte *)(param_3 + iVar13 + 0x1);
                *pbVar2  = *pbVar2 & (byte)uVar21;
                param_4  = (ushort)&stack0xfff6;
                param_16 = (ushort)pcStack4;
                param_17 = (ushort)((ulong)pcStack4 >> 0x10);
                uVar9    = param_15;
            code_r0x10203db1:
                get_sys_metrics_1020_7c1a((ushort *)CONCAT22(param_17, param_16), uVar9, param_8);
                puVar6                              = (undefined2 *)*(undefined4 *)(param_4 + 0x6);
                uVar28                              = (undefined2)((ulong)puVar6 >> 0x10);
                *(undefined4 *)((int)puVar6 + 0x14) = 0x0;
                *puVar6                             = 0x408a;
                *(undefined2 *)((int)puVar6 + 0x2)  = 0x1020;
                puVar32                             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, param_9, puVar24, (int)pcVar27);
                uVar28                              = (undefined2)((ulong)puVar32 >> 0x10);
                uVar7                               = *(undefined4 *)(param_4 + 0x6);
                uVar29                              = (undefined2)((ulong)uVar7 >> 0x10);
                iVar13                              = (int)uVar7;
                *(undefined2 *)(iVar13 + 0x14)      = (int)puVar32;
                *(undefined2 *)(iVar13 + 0x16)      = uVar28;
                ppcVar5                             = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar13 + 0x14) + 0x4);
                (**ppcVar5)(0x1010, *(undefined2 *)(iVar13 + 0x14), uVar28, 0x0, iVar13, uVar29);
                HVar14                            = GetDC16(0x1010);
                *(HDC16 *)(param_4 + -0x2)        = HVar14;
                iVar15                            = SetMapMode16((HDC16)s_tile2_bmp_1050_1538, 0x1);
                uVar7                             = *(undefined4 *)(param_4 + 0x6);
                *(int16_t *)((int)uVar7 + 0x1e)   = iVar15;
                uVar28                            = *(undefined2 *)(param_4 + -0x2);
                HVar16                            = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
                HVar16                            = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar16);
                uVar7                             = *(undefined4 *)(param_4 + 0x6);
                *(HGDIOBJ16 *)((int)uVar7 + 0x18) = HVar16;
                uVar29                            = *(undefined2 *)(param_4 + -0x2);
                HVar16                            = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
                HVar16                            = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar16);
                uVar7                             = *(undefined4 *)(param_4 + 0x6);
                *(HGDIOBJ16 *)((int)uVar7 + 0x1a) = HVar16;
                uVar7                             = *(undefined4 *)(param_4 + 0x6);
                uVar7                             = *(undefined4 *)((int)uVar7 + 0x14);
                *(undefined4 *)(param_4 + -0x6)   = *(undefined4 *)((int)uVar7 + 0x24);
                puVar17                           = (undefined *)(param_4 + -0x2);
                puVar8                            = (undefined4 *)*(undefined4 *)(param_4 + -0x6);
                ppcVar5                           = (code **)((int)*puVar8 + 0x8);
                (**ppcVar5)((int)s_tile2_bmp_1050_1538, (int)puVar8, (int)((ulong)puVar8 >> 0x10), puVar17, param_9, uVar29, uVar28);
                uVar7                              = *(undefined4 *)(param_4 + 0x6);
                uVar29                             = (undefined2)((ulong)uVar7 >> 0x10);
                iVar13                             = (int)uVar7;
                *(int *)(iVar13 + 0x1c)            = (int)puVar17;
                uVar28                             = *(undefined2 *)(param_4 + -0x2);
                *(undefined2 *)(param_4 + -0x14)   = uVar28;
                uVar7                              = *(undefined4 *)(iVar13 + 0x14);
                *(undefined2 *)((int)uVar7 + 0x4c) = uVar28;
                return;
            }
            pbVar2  = (byte *)(param_3 + iVar13);
            *pbVar2 = *pbVar2 & bVar12;
            pcVar1  = (char *)(param_4 + iVar13 + 0x20);
            bVar11  = (byte)param_1 & 0x1f;
            cVar19  = *pcVar1;
            *pcVar1 = *pcVar1 >> bVar11;
            pcVar1  = (char *)(param_4 + iVar13 + 0x6a);
            *pcVar1 = *pcVar1 + (byte)param_1 + ((param_1 & 0x1f) != 0x0 && (cVar19 >> bVar11 - 0x1 & 0x1U) != 0x0);
            pbVar2  = (byte *)(param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar12;
            param_8 = 0x1020;
            uVar20  = *(int *)(param_3 + iVar13) * 0x10;
            pbVar2  = (byte *)(pcVar27 + param_4 + 0x8);
            bVar12  = (byte)(uVar20 >> 0x8);
            uVar21  = uVar20 & 0xff | (uint)(byte)(bVar12 + *pbVar2) << 0x8;
            pcVar1  = (char *)(param_4 + iVar13 + 0x37);
            *pcVar1 = *pcVar1 + (char)(param_3 >> 0x8) + CARRY1(bVar12, *pbVar2);
        }
        else
        {
            pcVar1  = (char *)(param_4 + iVar13);
            *pcVar1 = *pcVar1 + bVar30;
            uVar21  = *(int *)(param_3 + iVar13) * 0x10;
            if((POPCOUNT(*pcVar1) & 0x1U) == 0x0)
                goto code_r0x10203db1;
        }
        pbVar2   = (byte *)(param_3 + iVar13);
        bVar11   = (byte)puVar24;
        *pbVar2  = *pbVar2 | bVar11;
        pbVar2   = (byte *)(param_3 + iVar13);
        *pbVar2  = *pbVar2 | bVar11;
        param_16 = (uint)(param_14 & 0x1) * 0x4000 | (uint)(param_13 & 0x1) * 0x200 | (uint)(param_12 & 0x1) * 0x100 | (uint)((char)*pbVar2 < '\0') * 0x80 | (uint)(*pbVar2 == 0x0) * 0x40 | (uint)(byte)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10
                 | (uint)((POPCOUNT(*pbVar2) & 0x1U) == 0x0) * 0x4;
        pbVar2  = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar11;
        bVar12  = in(0x79);
        pbVar2  = (byte *)(param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar11;
        if(-0x1 < (char)*pbVar2)
        {
            uVar9 = CONCAT22(param_19, param_18);
            if((bVar18 | *(byte *)(param_4 - 0x1)) == 0x0)
            {
                param_16 = param_7;
                uVar9    = CONCAT22(param_19, param_18);
            }
            goto code_r0x10203db1;
        }
        pbVar2         = (byte *)(param_4 + 0x89c);
        bVar30         = CARRY1(*pbVar2, bVar12);
        *pbVar2        = *pbVar2 + bVar12;
        bVar23         = bVar18 + bVar12;
        cVar19         = bVar23 + bVar30;
        uVar20         = CONCAT11(cVar19, bVar12);
        pcStack4._0_3_ = CONCAT21((uint)(param_14 & 0x1) * 0x4000 | (uint)(SCARRY1(bVar18, bVar12) != SCARRY1(bVar23, bVar30)) * 0x800 | (uint)(param_13 & 0x1) * 0x200 | (uint)(param_12 & 0x1) * 0x100 | (uint)(cVar19 < '\0') * 0x80
                                    | (uint)(cVar19 == '\0') * 0x40 | (uint)(byte)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10 | (uint)((POPCOUNT(cVar19) & 0x1U) == 0x0) * 0x4 | (uint)(CARRY1(bVar18, bVar12) || CARRY1(bVar23, bVar30)),
                                  pcStack4._0_1_);
        pcStack4       = (code *)((ulong)pcStack4 & 0xff000000 | (ulong)(uint3)pcStack4);
        pbVar2         = (byte *)(param_3 + iVar13);
        *pbVar2        = *pbVar2 | bVar11;
        param_1        = uVar21 - 0x1;
        bVar10         = bVar4 | bVar22;
        if(param_1 == 0x0 || *pbVar2 == 0x0)
            goto code_r0x10203d96;
    } while(true);
}

void __stdcall16far invalidate_rect_1020_1fb2(ulong param_1, int param_2, HWND16 param_3)

{
    ushort local_16;
    ushort uStack20;
    int    iStack18;
    ushort uStack16;
    RECT16 local_e;
    int    iStack10;
    ushort uStack6;
    ushort uStack4;

    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x6) = 0x0;
        return;
    }
    if(param_2 != 0xd)
    {
        return;
    }
    GetWindowRect16(param_3, &local_e);
    local_16 = 0x0;
    uStack6  = 0x46;
    uStack20 = 0x46;
    iStack18 = iStack10 - local_e.x;
    uStack4  = 0x5f;
    uStack16 = 0x5f;
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)0x0, (BOOL16)&local_16);
    return;
}

void __stdcall16far unk_draw_op_1020_2020(ulong param_1, HWND16 param_2, ushort param_3)

{
    code        **ppcVar1;
    undefined4    uVar2;
    undefined4   *puVar3;
    ushort        uVar4;
    HDC16        *pHVar5;
    int           iVar6;
    HPEN16        HVar7;
    HGDIOBJ16     HVar8;
    HBRUSH16      HVar9;
    uchar        *puVar10;
    uint          extraout_DX;
    uint          uVar11;
    int           iVar12;
    int           iVar13;
    undefined    *puVar14;
    undefined2    uVar15;
    undefined2    uVar16;
    ulong         uVar17;
    int          *piVar18;
    int           iVar19;
    undefined     uVar20;
    undefined     uVar21;
    undefined     local_38[0x6];
    undefined2    local_32;
    undefined2    uStack48;
    undefined4    uStack46;
    undefined2    uStack42;
    undefined4   *puStack40;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    puVar14   = &stack0xfffe;
    uVar15    = (undefined2)(param_1 >> 0x10);
    iVar12    = (int)param_1;
    uVar16    = *(undefined2 *)(iVar12 + 0x4);
    local_24  = BeginPaint16(param_2, &local_22);
    puStack40 = (undefined4 *)pass1_1010_4c2c(*(ulong *)(iVar12 + 0x6));
    pHVar5    = &local_24;
    ppcVar1   = (code **)((int)*puStack40 + 0x8);
    (**ppcVar1)(0x1010, (int)puStack40, (int)((ulong)puStack40 >> 0x10), pHVar5, param_3, uVar16);
    *(HDC16 **)(iVar12 + 0x10) = pHVar5;
    uVar2                      = *(undefined4 *)(iVar12 + 0x6);
    uStack42                   = *(undefined2 *)((int)uVar2 + 0x30);
    uVar2                      = *(undefined4 *)(iVar12 + 0x6);
    uStack46                   = *(undefined4 *)*(undefined4 *)((int)uVar2 + 0x12);
    uStack48                   = 0x14;
    local_32                   = 0x0;
    pass1_1008_3e38((ushort *)CONCAT22(param_3, local_38));
    while(*(int *)(puVar14 + -0x38) < *(int *)(puVar14 + -0x28))
    {
        iVar12                       = *(int *)(puVar14 + -0x38) * 0x4;
        uVar2                        = *(undefined4 *)(puVar14 + -0x2c);
        uVar17                       = pass1_1008_4772(*(astruct_76 **)(iVar12 + (int)uVar2));
        puVar10                      = (uchar *)(uVar17 >> 0x10);
        *(int *)(puVar14 + -0x44)    = (int)uVar17;
        *(uchar **)(puVar14 + -0x42) = puVar10;
        uVar2                        = *(undefined4 *)(puVar14 + 0x6);
        pass1_1020_2286((ushort)uVar2, (ushort)((ulong)uVar2 >> 0x10), (int *)CONCAT13((char)(param_3 >> 0x8), CONCAT12((char)param_3, puVar14 + -0x30)), *(int *)((int)uVar17 + 0x8));
        uVar2 = *(undefined4 *)(puVar14 + -0x30);
        pass1_1008_3e76((ushort *)CONCAT22(param_3, puVar14 + -0x36), 0x0, (ushort)uVar2, (ushort)((ulong)uVar2 >> 0x10));
        uVar2 = *(undefined4 *)(puVar14 + -0x2c);
        pass1_1008_4480(*(ulong *)(puVar14 + -0x26), (ushort *)CONCAT22(param_3, puVar14 + -0x36), *(astruct_76 **)((int)uVar2 + iVar12), param_3);
        iVar12 = *(int *)(puVar14 + -0x38);
        uVar2  = *(undefined4 *)(puVar14 + -0x30);
        uVar15 = (undefined2)uVar2;
        uVar20 = (undefined)((ulong)uVar2 >> 0x10);
        uVar21 = (undefined)((ulong)uVar2 >> 0x18);
        uVar2  = *(undefined4 *)(puVar14 + -0x44);
        uVar16 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar13 = (int)uVar2;
        iVar6  = *(int *)(iVar13 + 0x4) + *(int *)(puVar14 + -0x2e);
        iVar13 = *(int *)(iVar13 + 0x8) + *(int *)(puVar14 + -0x30);
        uVar2  = *(undefined4 *)(puVar14 + 0x6);
        uVar2  = *(undefined4 *)((int)uVar2 + 0x6);
        iVar19 = (int)uVar2;
        uVar16 = (undefined2)((ulong)uVar2 >> 0x10);
        if(*(long *)(iVar19 + 0x1a) == 0x0)
        {
            uVar4 = *(int *)(iVar19 + 0x30) << 0x3;
            mem_op_1000_179c(uVar4, puVar10, 0x1000);
            *(ushort *)(iVar19 + 0x1a) = uVar4;
            *(uchar **)(iVar19 + 0x1c) = puVar10;
        }
        uVar2                                      = *(undefined4 *)(iVar19 + 0x1a);
        iVar12                                     = iVar12 * 0x8;
        *(undefined2 *)((int)uVar2 + iVar12)       = CONCAT11(uVar21, uVar20);
        uVar2                                      = *(undefined4 *)(iVar19 + 0x1a);
        *(undefined2 *)((int)uVar2 + iVar12 + 0x2) = uVar15;
        uVar2                                      = *(undefined4 *)(iVar19 + 0x1a);
        *(int *)((int)uVar2 + iVar12 + 0x4)        = iVar6;
        uVar2                                      = *(undefined4 *)(iVar19 + 0x1a);
        *(int *)((int)uVar2 + iVar12 + 0x6)        = iVar13;
        uVar2                                      = *(undefined4 *)(puVar14 + -0x44);
        piVar18                                    = (int *)(puVar14 + -0x2e);
        *piVar18                                   = *piVar18 + (-(uint)(*(int *)(puVar14 + -0x38) == 0x0) & 0x5) + 0x14 + *(int *)((int)uVar2 + 0x4);
        piVar18                                    = (int *)(puVar14 + -0x38);
        *piVar18                                   = *piVar18 + 0x1;
    }
    puVar3  = *(undefined4 **)(puVar14 + -0x26);
    ppcVar1 = (code **)((int)*puVar3 + 0x4);
    (**ppcVar1)(0x1008, (int)puVar3, (int)((ulong)puVar3 >> 0x10), 0x0, 0x0, (char)puVar14 + -0x22, param_3);
    uVar11                          = extraout_DX;
    HVar7                           = CreatePen16(0x1008, 0x25, 0x100);
    *(HPEN16 *)(puVar14 + -0x3a)    = HVar7;
    HVar8                           = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar7);
    *(HGDIOBJ16 *)(puVar14 + -0x3c) = HVar8;
    HVar9                           = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    *(HBRUSH16 *)(puVar14 + -0x3e)  = HVar9;
    HVar8                           = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar9);
    *(HGDIOBJ16 *)(puVar14 + -0x40) = HVar8;
    draw_line_1020_229c(*(ulong *)(puVar14 + 0x6), (int)s_tile2_bmp_1050_1538);
    uVar2 = *(undefined4 *)(puVar14 + 0x6);
    pass1_1010_4df0(*(ulong *)((int)uVar2 + 0x6), uVar11, param_3);
    if(HVar8 == 0x0)
    {
        SelectObject16(0x1010, *(HGDIOBJ16 *)(puVar14 + -0x3c));
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar14 + -0x40));
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        HVar9                          = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
        *(HBRUSH16 *)(puVar14 + -0x3e) = HVar9;
        HVar7                          = CreatePen16((INT16)s_tile2_bmp_1050_1538, 0xff, 0x0);
        *(HPEN16 *)(puVar14 + -0x3a)   = HVar7;
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar14 + -0x3e));
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar14 + -0x3a));
    }
    uVar2   = *(undefined4 *)(puVar14 + 0x6);
    piVar18 = (int *)pass1_1010_4dc8(*(ulong *)((int)uVar2 + 0x6));
    uVar15  = (undefined2)((ulong)piVar18 >> 0x10);
    uVar16  = SUB42(piVar18, 0x0);
    pass1_1020_239c(*(ulong *)(puVar14 + 0x6), piVar18, param_3);
    uVar2 = *(undefined4 *)(puVar14 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x6);
    if(*(int *)((int)uVar2 + 0x2c) != 0x0)
    {
        pass1_1020_2488(*(ulong *)(puVar14 + 0x6), uVar16, uVar15);
    }
    uVar2 = *(undefined4 *)(puVar14 + 0x6);
    SelectPalette16(0x1010, 0x0, *(BOOL16 *)((int)uVar2 + 0x10));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar14 + -0x3c));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar14 + -0x40));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, (PAINTSTRUCT16 *)(puVar14 + -0x20));
    return;
}

void __stdcall16far draw_line_1020_229c(ulong param_1, HDC16 param_2)

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

void __stdcall16far pass1_1020_239c(ulong param_1, int *param_2, ushort param_3)

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
        uVar2   = pass1_1020_23f2((ushort)param_1, uVar3, (ushort *)CONCAT22(param_3, local_a), (uchar *)((ulong)puVar1 >> 0x10), param_3);
        draw_polygon_1020_2474((ushort)param_1, uVar3, CONCAT22((int)uVar2, 0x3), 0x1008);
    }
    return;
}

void __stdcall16far draw_polygon_1020_2474(ushort param_1, ushort param_2, ulong param_3, HDC16 param_4)

{
    Polygon16(param_4, (POINT16 *)param_3, (INT16)(param_3 >> 0x10));
    return;
}

void __stdcall16far struct_1020_2524(astruct_20 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uchar      *extraout_DX;
    undefined2  uVar1;
    astruct_20 *iVar2;
    int         unaff_DI;
    undefined2  uVar2;
    ushort     *puVar3;

    unk_draw_op_1020_7f7a(param_1, 0x7, CONCAT22(param_3, param_2));
    uVar2                                              = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                                              = (astruct_20 *)param_1;
    *(undefined4 *)&iVar2[0x1].field_0xc               = 0x0;
    iVar2[0x1].field_0x10                              = 0x0;
    param_1->field_0x0                                 = 0x270c;
    iVar2->field_0x2                                   = 0x1020;
    ((astruct_20 *)(iVar2 + 0x1))->field_0x0           = 0x27a8;
    iVar2[0x1].field_0x2                               = 0x1020;
    puVar3                                             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2a, param_4, extraout_DX, unaff_DI);
    uVar1                                              = (undefined2)((ulong)puVar3 >> 0x10);
    *(int *)&iVar2[0x1].field_0x10                     = (int)puVar3;
    *(undefined2 *)((int)&iVar2[0x1].field_0x10 + 0x2) = uVar1;
    *(undefined2 *)&iVar2[0x1].field_0x4               = *(undefined2 *)&iVar2[0x1].field_0x10;
    *(undefined2 *)((int)&iVar2[0x1].field_0x4 + 0x2)  = uVar1;
    return;
}

void __stdcall16far pass1_1020_27b0(astruct_664 *param_1, ushort param_2, ushort param_3, int param_4, ushort param_5)

{
    code     **ppcVar1;
    undefined4 uVar2;
    int        iVar3;
    uchar     *extraout_DX;
    undefined2 uVar4;
    ushort    *puVar5;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0x14       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x288e;
    param_1->field_0x2                        = 0x1020;
    puVar5                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2a, param_5, extraout_DX, param_4);
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
    draw_op_1020_9364((astruct_7 *)CONCAT22(param_2, param_1), 0x1010, param_5);
    return;
}

void __stdcall16far pass1_1020_2838(ushort *param_1, ushort param_2)

{
    astruct_584 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_584 *)param_1;
    *param_1         = 0x288e;
    iVar1->field_0x2 = 0x1020;
    if(iVar1->field_0x14 != 0x0)
    {
        param_2 = 0x1010;
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, param_2);
    return;
}
