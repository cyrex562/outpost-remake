
void  begin_end_pai16_1008_97c8(HWND16 param_1)

{
    PAi16STRUCT16 local_22;

    BeginPai1616(param_1, &local_22);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

void  get_stock_obj_1008_9c56(i1616 param_1)

{
    GetStockObject16(param_1);
    return;
}


astruct_23 * unk_draw_op_1008_80ee(astruct_23 *param_1, u16 param_2)

{
    HCURSOR16   HVar1;
    HGDIOBJ16   HVar2;
    astruct_23 *iVar3;
    astruct_23 *uVar3;

    uVar3              = (astruct_23 *)(param_1 >> 0x10);
    iVar3              = (astruct_23 *)param_1;
    param_1->field_0x0 = 0x389a;
    iVar3->field_0x2   = 0x1008;
    iVar3->field_0x54  = 0x0;
    iVar3->field_0x56  = 0x0;
    iVar3->field_0x58  = 0x0;
    param_1->field_0x0 = 0x87c8;
    iVar3->field_0x2   = 0x1008;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x4)), s_MicroSpinControl_1050_0370);
    iVar3->field_0x54 = 0x3;
    HVar1             = LoadCursor16(0x1000, 0x7f00);
    iVar3->field_0x58 = HVar1;
    HVar2             = GetStockObject16((i1616)s_tile2_bmp_1050_1538);
    iVar3->field_0x56 = HVar2;
    pass1_1008_818c((astruct_23 *)(param_1 & 0xffff | ZEXT24(uVar3) << 0x10), s_tile2_bmp_1050_1538, param_2);
    return param_1;
}


void  pass1_1008_818c(astruct_23 *param_1, HINSTANCE16 param_2, WNDCLASS16 *param_3)

{
    BOOL16     BVar1;
    ATOM       AVar2;
    u16        local_1c;
    u16        uStack26;
    u16        uStack24;
    u32 uStack22;
    u8        *puStack18;
    u16        uStack16;
    u16        uStack14;
    u16        uStack12;
    u32 uStack10;
    i16        iStack6;
    u16        uStack4;

    iStack6 = param_1 + 0x4;
    BVar1   = GetClassInfo16(param_2, (SEGPTR)&local_1c, param_3);
    if(BVar1 == 0x0)
    {
        local_1c  = (param_1 + 0x54);
        uStack26  = 0x84f2;
        uStack24  = 0x1008;
        uStack22  = 0x40000;
        puStack18 = globals->PTR_LOOP_1050_038c;
        uStack16  = 0x0;
        uStack14  = (param_1 + 0x58);
        uStack12  = (param_1 + 0x56);
        uStack10  = 0x0;
        uStack4   = param_1._2_2_;
        AVar2     = RegisterClass16(s_tile2_bmp_1050_1538);
        if(AVar2 == 0x0)
        {
            fn_ptr_op_1000_24cd(0x0, &stack0xfffe);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void draw_op_1008_8288(u16 param_1, u32 param_2, HWND16 param_3)

{
    HGDIOBJ16     HVar1;
    HGDIOBJ16     HVar2;
    i16           x;
    u16           uVar3;
    RECT16        local_58;
    u16           uStack84;
    u16           uStack82;
    HBRUSH16      HStack80;
    HPEN16        HStack78;
    HPEN16        HStack76;
    HDC16         HStack74;
    u16           uStack72;
    u16           uStack70;
    u16           uStack68;
    u16           uStack66;
    u16           uStack64;
    u16           uStack62;
    PAi16STRUCT16 local_3c;
    i16           local_1c;
    i16           iStack26;
    i16           iStack24;
    i16           iStack22;
    i16           iStack20;
    i16           iStack18;
    i16           local_10;
    i16           iStack14;
    i16           iStack12;
    i16           iStack10;
    i16           iStack8;
    i16           iStack6;
    u16           uStack4;

    HStack74 = BeginPai1616(param_3, &local_3c);
    uStack4  = 0x0;
    HStack76 = CreatePen16((i1616)s_tile2_bmp_1050_1538, (i1616)_PTR_LOOP_1050_0368, (COLORREF)(_PTR_LOOP_1050_0368 >> 0x10));
    HStack78 = CreatePen16((i1616)s_tile2_bmp_1050_1538, (i1616)DAT_1050_0364, (COLORREF)(DAT_1050_0364 >> 0x10));
    HStack80 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_58);
    uStack62 = uStack84;
    uStack64 = uStack82;
    uStack66 = uStack84 >> 0x1;
    uStack68 = uStack82 >> 0x1;
    uStack70 = uStack84 >> 0x2;
    uStack72 = uStack82 >> 0x2;
    HVar1    = GetStockObject16((i1616)s_tile2_bmp_1050_1538);
    HVar1    = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar1);
    HVar2    = GetStockObject16((i1616)s_tile2_bmp_1050_1538);
    HVar2    = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar2);
    Rectangle16((HDC16)s_tile2_bmp_1050_1538, uStack82, uStack84, local_58.y, local_58.x);
    MoveTo16((HDC16)s_tile2_bmp_1050_1538, uStack68, 0x0);
    LineTo16((HDC16)s_tile2_bmp_1050_1538, uStack68, uStack62);
    uVar3 = (param_2 >> 0x10);
    if((*(u8 *)(param_2 + 0x4) & 0x4) != 0x0)
    {
        uStack4 = 0x1;
    }
    local_10 = uStack66 + uStack4;
    iStack14 = uStack72 + uStack4 + -0x2;
    iStack12 = local_10 + -0x3;
    iStack10 = uStack72 + uStack4 + 0x1;
    iStack8  = local_10 + 0x3;
    iStack6  = iStack10;
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack76);
    if(uStack4 == 0x0)
    {
        MoveTo16((HDC16)s_tile2_bmp_1050_1538, uStack68 - 0x2, 0x1);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x1, 0x1);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x1, uStack62 - 0x1);
    }
    uStack4  = ((*(u8 *)(param_2 + 0x4) & 0x8) != 0x0);
    local_1c = uStack66 + uStack4;
    iStack22 = (uStack64 - uStack72) + uStack4;
    iStack26 = iStack22 + 0x1;
    iStack24 = local_1c + -0x3;
    iStack22 = iStack22 + -0x2;
    iStack20 = local_1c + 0x3;
    iStack18 = iStack22;
    if(uStack4 == 0x0)
    {
        MoveTo16((HDC16)s_tile2_bmp_1050_1538, uStack82 - 0x2, 0x1);
        x = uStack68 + 0x1;
        LineTo16((HDC16)s_tile2_bmp_1050_1538, x, 0x1);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, x, uStack62 - 0x1);
    }
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack78);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack80);
    Polygon16((HDC16)s_tile2_bmp_1050_1538, (POi1616 *)(&PTR_LOOP_1050_0002 + 0x1), (i1616)&local_10);
    Polygon16((HDC16)s_tile2_bmp_1050_1538, (POi1616 *)(&PTR_LOOP_1050_0002 + 0x1), (i1616)&local_1c);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar2);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar1);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_3c);
    return;
}


astruct_20 * unk_draw_op_1008_61b2(astruct_20 *param_1, u16 param_2, u16 param_3, u32 param_4, u16 param_5)

{
    HGDIOBJ16   l_hgdiobj_1;
    HCURSOR16   l_hcursor_1;
    u8         *extraout_DX;
    u8         *puVar1;
    i16         unaff_DI;
    u16        *l_struct_2;
    astruct_20 *uVar5;
    astruct_20 *iVar1;
    astruct_20 *iVar4;
    u16        *uVar1;

    iVar1 = (astruct_20 *)param_1;
    uVar5 = (astruct_20 *)(param_1 >> 0x10);
    set_struct_1008_687a(param_1, param_4);
    iVar1->field_0xde  = param_2;
    iVar1->field_0xe0  = 0x0;
    param_1->field_0x0 = 0x6378;
    iVar1->field_0x2   = 0x1008;
    puVar1             = extraout_DX;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0x5b)), s_DanBrotherton_1050_0302);
    l_hgdiobj_1               = GetStockObject16(0x1000);
    iVar1->hgdiobj_field_0xc6 = l_hgdiobj_1;
    l_hcursor_1               = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538, 0x7f00);
    iVar1->hcursor_field_0xc4 = l_hcursor_1;
    iVar1->field_0xc8         = 0x200b;
    iVar1->field_0xac         = 0x45000000;
    iVar1->field_0xbc         = (param_4 + 0x8);
    l_struct_2                = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_5, puVar1, unaff_DI);
    uVar1                     = (l_struct_2 >> 0x10);
    iVar1->field_0xb4         = 0x0;
    iVar1->field_0xb6         = 0x0;
    iVar1->field_0xb8         = (l_struct_2 + 0xa);
    iVar1->field_0xba         = (l_struct_2 + 0xc);
    iVar1->field_0xca         = param_3;
    win_ui_reg_class_1008_96d2(param_1, 0x1010, param_5);
    return param_1;
}


void  fill_rect_1008_62c0(HWND16 param_1)

{
    RECT16        local_2e[0x2];
    RECT16       *pRStack38;
    HDC16         HStack36;
    PAi16STRUCT16 local_22;

    HStack36  = BeginPai1616(param_1, &local_22);
    pRStack38 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    GetClientRect16((HWND16)s_tile2_bmp_1050_1538, local_2e);
    FillRect16((HDC16)s_tile2_bmp_1050_1538, pRStack38, (HBRUSH16)local_2e);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_22);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    return;
}


HPALETTE16  palette_op_1008_4e08(astruct_13 *param_1, BOOL16 param_2, u16 param_3, HDC16 param_4)

{
    HPALETTE16 HVar1;

    create_palette_1008_4e38(param_1, param_4, param_3);
    HVar1 = SelectPalette16(param_4, 0x0, param_2);
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    return HVar1;
}

// WARNING: Unable to use type for symbol uVar3

void  create_palette_1008_4e38(astruct_13 *in_struct_1, LOGPALETTE *in_log_palette_2, u8 *param_3)

{
    i16        *piVar1;
    u16        *puVar2;
    u16         uVar4;
    astruct_13 *local_struct_1;
    i16         iVar5;
    i16         iVar6;
    u16         uVar8;
    u16         uVar9;
    u16         uVar10;
    i16         iStack14;
    u8         *puStack12;
    u8         *puStack8;
    u16        *uVar3;

    uVar8          = (in_struct_1 >> 0x10);
    local_struct_1 = (astruct_13 *)in_struct_1;
    uVar4          = (local_struct_1->field_0xc + 0x2) * 0x4;
    if(local_struct_1->field_0xe == 0x0)
    {
        in_log_palette_2 = (LOGPALETTE *)&globals->PTR_LOOP_1050_1000;
        mem_op_1000_179c(uVar4, param_3, 0x1000);
        &local_struct_1->field_0xe         = uVar4;
        (&local_struct_1->field_0xe + 0x2) = param_3;
        *local_struct_1->field_0xe         = 0x300;
        uVar3                              = local_struct_1->field_0xe;
        (uVar3 + 0x2)                      = local_struct_1->field_0xc;
        puVar2                             = local_struct_1->field_0xe;
        puStack8                           = (puVar2 & 0xffff0000 | (puVar2 + 0x4));
        puStack12                          = local_struct_1->field_0x4;
        iStack14                           = 0x0;
        while(true)
        {
            piVar1 = &local_struct_1->field_0xc;
            if(*piVar1 == iStack14 || *piVar1 < iStack14)
                break;
            uVar9          = (puStack12 >> 0x10);
            iVar5          = puStack12;
            *puStack8      = *(iVar5 + 0x2);
            uVar10         = (puStack8 >> 0x10);
            iVar6          = puStack8;
            *(iVar6 + 0x1) = *(iVar5 + 0x1);
            *(iVar6 + 0x2) = *puStack12;
            *(iVar6 + 0x3) = 0x0;
            iStack14       = iStack14 + 0x1;
            puStack8       = (puStack8 & 0xffff0000 | (iVar6 + 0x4));
            puStack12      = (puStack12 & 0xffff0000 | (iVar5 + 0x4));
        }
    }
    CreatePalette16(in_log_palette_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  file_and_draw_op_1008_4f20(u16 *param_1, u32 param_2, u16 param_3, u32 param_4, u16 param_5)

{
    u32  uVar1;
    u16         b_force_background;
    COLORREF    color;
    COLORREF    color_00;
    u16         x;
    u16         uVar2;
    LPCSTR      output;
    i16         iVar3;
    u16         uVar4;
    astruct_43 *paVar5;
    u32         uVar6;
    DEVMODEA   *init_data;
    HDC16       local_2c;
    LPCSTR      pCStack42;
    LPCSTR      pCStack40;
    u8          local_26[0x24];

    pass1_1008_4016((astruct_76 *)param_1);
    uVar4           = (param_1 >> 0x10);
    iVar3           = param_1;
    *(iVar3 + 0x1e) = param_4;
    (iVar3 + 0x22)  = param_3;
    *(iVar3 + 0x24) = param_2;
    *param_1        = s_SCi16ernalPutBldg2_site_0x_08lx__1050_5099 + 0x9;
    (iVar3 + 0x2)   = 0x1008;
    paVar5          = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x2, param_5);
    uVar2           = (paVar5 >> 0x10);
    struct_op_1008_48fe((astruct_81 *)CONCAT22(param_5, local_26), 0x1, paVar5, uVar2);
    read_file_1008_49e8(CONCAT22(param_5, local_26), 0x1010, uVar2);
    pass1_1008_5068((astruct_76 *)param_1, (astruct_83 *)CONCAT22(param_5, local_26));
    pass1_1008_47cc((astruct_76 *)param_1);
    pass1_1008_4834((astruct_76 *)param_1);
    init_data          = (DEVMODEA *)0x0;
    uVar6              = pass1_1008_4772((astruct_76 *)param_1);
    output             = (uVar6 >> 0x10);
    pCStack42          = uVar6;
    pCStack40          = output;
    local_2c           = CreateDC16(0x1010, pCStack42, output, init_data);
    b_force_background = palette_op_1008_46e4(param_1, &local_2c, output, s_tile2_bmp_1050_1538);
    color              = SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0xffff);
    color_00           = SetTextColor16((HDC16)s_tile2_bmp_1050_1538, *(COLORREF *)(iVar3 + 0x22));
    x                  = str_op_1000_3da4((iVar3 + 0x1e));
    uVar1              = (iVar3 + 0x1e);
    TextOut16(0x1000, x, (i1616)uVar1, (uVar1 >> 0x10), 0x0);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, color);
    SetTextColor16((HDC16)s_tile2_bmp_1050_1538, color_00);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
    close_file_1008_496c(local_26, param_5);
    return;
}


BOOL16  cleanup_palette_1008_56e2(u32 param_1, HDC16 param_2)

{
    HPALETTE16 HVar1;
    u16        uVar2;

    uVar2                          = (param_1 >> 0x10);
    HVar1                          = SelectPalette16(param_2, 0x0, *(BOOL16 *)(param_1 + 0x4));
    *(HPALETTE16 *)(param_1 + 0x4) = HVar1;
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    return 0x1;
}


void  set_di_bits_to_device_1008_45d6(u32 param_1, HDC16 param_2)

{
    i1616      info;
    u32 uVar1;
    bool       bVar2;
    i16        iVar3;
    i16        y_dest;
    u16        uVar4;
    i1616      cx;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x6) == 0x0)
    {
        pass1_1008_47cc((astruct_76 *)param_1);
    }
    if(((iVar3 + 0x8) | (iVar3 + 0x6)) == 0x0)
    {
        bVar2 = false;
    }
    else
    {
        if(((iVar3 + 0xc) | (iVar3 + 0xa)) == 0x0)
        {
            pass1_1008_4834((astruct_76 *)param_1);
        }
        bVar2 = true;
    }
    if(!bVar2)
    {
        return;
    }
    uVar1  = (iVar3 + 0x10);
    cx     = (i1616)(uVar1 >> 0x10);
    y_dest = uVar1;
    info   = (y_dest + 0x8);
    uVar1  = (iVar3 + 0x14);
    SetDIBitsToDevice(param_2, 0x0, y_dest, cx, (i1616)uVar1, (i1616)(uVar1 >> 0x10), info, 0x0, 0x0, (LPCVOID)0x0, (BITMAPINFO *)info, (y_dest + 0x4));
    return;
}


void  stretch_di_bits_1008_465a(u32 param_1, HDC16 param_2)

{
    PVOID      bits;
    i1616      height_src;
    u32 uVar1;
    bool       bVar2;
    i16        iVar3;
    i16        height_dst;
    u16        uVar4;
    i1616      x_src;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x6) == 0x0)
    {
        pass1_1008_47cc((astruct_76 *)param_1);
    }
    if(((iVar3 + 0x8) | (iVar3 + 0x6)) == 0x0)
    {
        bVar2 = false;
    }
    else
    {
        if(((iVar3 + 0xc) | (iVar3 + 0xa)) == 0x0)
        {
            pass1_1008_4834((astruct_76 *)param_1);
        }
        bVar2 = true;
    }
    if(!bVar2)
    {
        return;
    }
    uVar1      = (iVar3 + 0x10);
    x_src      = (i1616)(uVar1 >> 0x10);
    height_dst = uVar1;
    bits       = *(PVOID *)(height_dst + 0x4);
    height_src = (height_dst + 0x8);
    uVar1      = (iVar3 + 0x14);
    StretchDIBits16(param_2, 0x20, 0xcc, 0x0, height_dst, x_src, (i1616)uVar1, (i1616)(uVar1 >> 0x10), height_src, bits, (BITMAPINFO *)0x0, 0x0, CONCAT22(bits, height_src));
    return;
}


u16  palette_op_1008_46e4(u32 param_1, u16 param_2, u16 param_3, HDC16 param_4)

{
    bool       bVar1;
    u16        uVar2;
    HPALETTE16 HVar2;
    i16        iVar3;
    u16        uVar4;
    u32        uVar5;
    u32 uVar6;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x6) == 0x0)
    {
        uVar5._0_2_ = param_2;
        uVar5._2_2_ = param_3;
        pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar4 << 0x10));
        param_2 = uVar5;
        param_3 = uVar5._2_2_;
    }
    uVar6 = CONCAT22(param_3, param_2);
    if((iVar3 + 0x6) == 0x0)
    {
        bVar1 = false;
    }
    else
    {
        if((iVar3 + 0xa) == 0x0)
        {
            uVar6 = pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar4 << 0x10));
        }
        bVar1 = true;
    }
    uVar2 = uVar6;
    if(!bVar1)
    {
        return 0x0;
    }
    create_palette_1008_4e38(*(astruct_13 **)(iVar3 + 0xa), param_4, (uVar6 >> 0x10));
    (iVar3 + 0xe)                = uVar2;
    HVar2                        = SelectPalette16(param_4, 0x0, *(BOOL16 *)(iVar3 + 0xe));
    *(HPALETTE16 *)(iVar3 + 0x4) = HVar2;
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
    return (iVar3 + 0x4);
}


void  set_sys_color_1008_357e(u32 param_1, i16 param_2, i1616 in_index_3, u16 param_4)

{
    u16         uVar1;
    COLORREF    colorref_var2;
    i16         iVar2;
    astruct_53 *iVar3;
    i16         iVar4;
    astruct_53 *uVar6;
    u16         uVar5;
    i1616       count;
    u32  uVar7;
    i16         iStack132;
    u32  local_80;
    u16         uStack124;
    u16         uStack122;
    u16         uStack120;
    u16         uStack118;
    u16         uStack116;
    u16         uStack114;
    u32  uStack112;
    u32  uStack108;
    u16         uStack104;
    u16         uStack102;
    u16         uStack100;
    u16         uStack98;
    u16         uStack96;
    u16         uStack94;
    u16         uStack92;
    u16         uStack90;
    u32  uStack88;
    u32  uStack84;
    u16         uStack80;
    u16         uStack78;
    u32  uStack76;
    u32  uStack72;
    u32  uStack68;
    u32  uStack64;
    u32  uStack60;
    u32  uStack56;
    u32  uStack52;
    u32  uStack48;
    u32  local_2c;
    u32  uStack40;
    u32  uStack36;
    u32  uStack32;
    u32  uStack28;
    u32  uStack24;
    u32  uStack20;
    u32  uStack16;
    u32  uStack12;
    u32  uStack8;
    u16         uStack4;

    local_2c  = 0x70004;
    uStack40  = 0xf0000;
    uStack36  = 0x100014;
    uStack32  = 0xd0012;
    uStack28  = 0x6000e;
    uStack24  = 0x80005;
    uStack20  = 0x10011;
    uStack16  = 0x30002;
    uStack12  = 0xa0009;
    uStack8   = 0xc000b;
    uStack4   = 0x13;
    local_80  = 0x0;
    uStack108 = 0x808080;
    iVar2     = 0x100;
    uStack116 = 0x0;
    uStack114 = 0x100;
    uStack100 = 0x0;
    uStack98  = 0x100;
    uStack96  = 0xffff;
    uStack94  = 0x0;
    uStack124 = 0x2;
    uStack122 = 0x100;
    uStack120 = 0x2;
    uStack118 = 0x100;
    uStack104 = 0x2;
    uStack102 = 0x100;
    uStack92  = 0x2;
    uStack90  = 0x100;
    uStack88  = 0x0;
    uStack80  = 0xc0c0;
    uStack78  = 0x0;
    uStack76  = 0x0;
    uStack72  = 0x0;
    uStack68  = 0x0;
    uStack52  = 0x0;
    uVar1     = 0x8000;
    uStack112 = 0x8000;
    uStack84  = 0x8000;
    uStack64  = 0x8000;
    uStack60  = 0x8000;
    uStack56  = 0x8000;
    uStack48  = 0x8000;
    uVar6     = (astruct_53 *)(param_1 >> 0x10);
    iVar3     = (astruct_53 *)param_1;
    if(&iVar3->field_0xf8 == 0x0)
    {
        mem_op_1000_179c(0x54, (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), 0x1000);
        iVar3->field_0xf8 = uVar1;
        iVar3->field_0xfa = iVar2;
        in_index_3        = 0x1000;
        for(iStack132 = 0x0; iStack132 < 0x15; iStack132 = iStack132 + 0x1)
        {
            colorref_var2                          = GetSysColor16(in_index_3);
            uVar7                                  = &iVar3->field_0xf8;
            uVar5                                  = (uVar7 >> 0x10);
            iVar4                                  = uVar7;
            *(COLORREF *)(iVar4 + iStack132 * 0x4) = colorref_var2;
            (iVar4 + iStack132 * 0x4 + 0x2)        = iVar2;
            in_index_3                             = (i1616)s_tile2_bmp_1050_1538;
        }
    }
    count = in_index_3;
    if(param_2 != 0x0)
    {
        count         = (i1616)s_tile2_bmp_1050_1538;
        colorref_var2 = GetSysColor16(in_index_3);
        if((colorref_var2 == (COLORREF)local_80) && (iVar2 == local_80._2_2_))
        {
            return;
        }
    }
    if(PTR_LOOP_1050_0010 == 0x0)
    {
        uStack112 = 0xc0c0c0;
    }
    if(param_2 == 0x0)
    {
        uVar7 = &iVar3->field_0xf8;
    }
    else
    {
        uVar7 = CONCAT22(param_4, &local_80);
    }
    SetSysColors16(count, (i1616 *)uVar7, (COLORREF *)(uVar7 >> 0x10));
    return;
}

void  fill_rect_1008_39ac(HWND16 in_win_handle_1)

{
    RECT16        local_brush_handle[0x2];
    RECT16       *local_brush_handle_2;
    HDC16         HStack36;
    PAi16STRUCT16 local_pai16_struct;

    HStack36             = BeginPai1616(in_win_handle_1, &local_pai16_struct);
    local_brush_handle_2 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    GetClientRect16((HWND16)s_tile2_bmp_1050_1538, local_brush_handle);
    FillRect16((HDC16)s_tile2_bmp_1050_1538, local_brush_handle_2, (HBRUSH16)local_brush_handle);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_pai16_struct);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    return;
}


void  pass1_1008_0984(i16 param_1, u16 param_2, i16 param_3, u16 param_4, u16 param_5)

{
    u32 uVar1;
    code     **ppcVar2;

    set_sys_color_1008_357e(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    if((param_1 + 0xe8) != 0x0)
    {
        uVar1   = (param_1 + 0xe8);
        ppcVar2 = ((param_1 + 0xe8) + 0x98);
        (**ppcVar2)(param_4, uVar1, (uVar1 >> 0x10), param_3);
    }
    return;
}


void  set_struct_op_1008_0536(u16 *param_1, HINSTANCE16 param_2, u16 param_3)

{
    HICON16     HVar1;
    HCURSOR16   HVar2;
    HGDIOBJ16   HVar3;
    u8         *puVar4;
    i16         iVar5;
    i16         unaff_DI;
    u16         uVar6;
    astruct_20 *paVar7;
    u16        *puVar8;

    paVar7                       = pass1_1008_3ab8((astruct_20 *)param_1);
    puVar4                       = (paVar7 >> 0x10);
    uVar6                        = (param_1 >> 0x10);
    iVar5                        = param_1;
    (iVar5 + 0xe0)               = 0x0;
    (iVar5 + 0xe4)               = 0x0;
    (iVar5 + 0xe8)               = 0x0;
    (iVar5 + 0xec)               = 0x0;
    (iVar5 + 0xee)               = 0x0;
    (iVar5 + 0xf2)               = 0x0;
    (iVar5 + 0xf4)               = 0x0;
    (iVar5 + 0xf8)               = 0x0;
    *param_1                     = 0x389e;
    (iVar5 + 0x2)                = 0x1008;
    (iVar5 + 0xc8)               = 0x2008;
    (iVar5 + 0xac)               = 0x0;
    (iVar5 + 0xae)               = 0x8700;
    HVar1                        = LoadIcon16(param_2, 0xd4);
    *(HICON16 *)(iVar5 + 0xc2)   = HVar1;
    HVar2                        = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538, 0x7f00);
    *(HCURSOR16 *)(iVar5 + 0xc4) = HVar2;
    HVar3                        = GetStockObject16((i1616)s_tile2_bmp_1050_1538);
    *(HGDIOBJ16 *)(iVar5 + 0xc6) = HVar3;
    puVar8                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_3, puVar4, unaff_DI);
    puVar4                       = (puVar8 >> 0x10);
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (iVar5 + 0xa)), s_Outpost_1050_00d7);
    puVar8         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_3, puVar4, unaff_DI);
    (iVar5 + 0xf4) = puVar8;
    (iVar5 + 0xf6) = (puVar8 >> 0x10);
    set_sys_color_1008_357e(param_1, 0x1, 0x1010, param_3);
    return;
}
