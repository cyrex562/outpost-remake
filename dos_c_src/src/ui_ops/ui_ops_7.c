
void  cursor_op_1008_2dcc(i16 param_1, u16 param_2, u16 param_3, HINSTANCE16 in_hinstance)

{
    undefined4 uVar1;
    code     **ppcVar2;
    HCURSOR16  cursor_handle;
    HCURSOR16  HVar3;
    u16        in_DX;
    u16        extraout_DX;
    u16        uVar4;
    u16        unaff_SS;
    u16        uVar5;

    uVar5         = 0x0;
    cursor_handle = LoadCursor16(in_hinstance, (LPCSTR)0x7f02);
    HVar3         = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    uVar4         = param_1;
    if((param_1 + 0xe8) != 0x0)
    {
        uVar1   = (param_1 + 0xe8);
        uVar4   = (param_1 + 0xe8);
        ppcVar2 = (uVar4 + 0x90);
        (**ppcVar2)(s_tile2_bmp_1050_1538, uVar1, (uVar1 >> 0x10), cursor_handle, uVar5);
        in_DX = extraout_DX;
    }
    big_switch_1008_15d4(uVar4, s_tile2_bmp_1050_1538, unaff_SS, CONCAT22(param_2, param_1), CONCAT22(cursor_handle, param_3));
    *(HCURSOR16 *)(param_1 + 0xe8) = HVar3;
    (param_1 + 0xea)               = in_DX;
    uVar1                          = (param_1 + 0xe8);
    if((uVar1 + 0xe0) == 0x0)
    {
        uVar1   = (param_1 + 0xe8);
        ppcVar2 = ((param_1 + 0xe8) + 0x8);
        (**ppcVar2)(s_tile2_bmp_1050_1538, uVar1, (uVar1 >> 0x10));
        ppcVar2 = ((param_1 + 0xe8) + 0xc);
        (**ppcVar2)(s_tile2_bmp_1050_1538, (param_1 + 0xe8), 0x3);
        (param_1 + 0xce) = (param_1 + 0xe8);
    }
    else
    {
        (param_1 + 0xe8) = 0x0;
        ui_op_1008_2c4e(param_1, param_2, param_3, (HINSTANCE16)s_tile2_bmp_1050_1538);
        (param_1 + 0xce) = 0x0;
    }
    SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    return;
}


void  win_ui_cursor_op_1008_2e9a(astruct_72 **param_1, u16 param_2)

{
    u16        uVar1;
    i16        iVar2;
    u8        *in_DX;
    u16        uVar3;
    u16        uVar4;
    i16        unaff_DI;
    u8         in_AF;
    char       local_22e[0xa];
    u8         local_224[0x108];
    u16        uStack284;
    char      *pcStack282;
    HCURSOR16  HStack274;
    HCURSOR16  HStack272;
    u32        uStack270;
    u32        UStack266;
    undefined4 uStack262;
    char       local_102[0x100];

    local_102[0]    = '\0';
    uStack262       = (astruct_73 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uVar1           = (uStack262 >> 0x10);
    iVar2           = uStack262;
    UStack266       = *(iVar2 + 0x16);
    uVar3           = (iVar2 + 0x18);
    UStack266._0_2_ = uVar3 | UStack266;
    if(UStack266 == 0x0)
    {
        save_file_1008_3178(param_1, 0x1, param_2);
        UStack266 = CONCAT22(uVar3, UStack266);
        uVar4     = uVar3 | UStack266;
        if(uVar4 == 0x0)
        {
            PostMessage16(0x1010, 0x0, 0x0, 0x111013d);
            return;
        }
        unk_str_op_1000_3d3e(CONCAT22(param_2, local_102), CONCAT22(uVar3, UStack266));
        str_1000_4d58(CONCAT22(param_2, local_102), 0x0, 0x0, CONCAT22(param_2, local_224), CONCAT22(param_2, local_22e));
        uVar3 = uVar4;
        if(local_22e[0] != '\0')
        {
            pass1_1000_3cea(CONCAT22(param_2, local_224), CONCAT22(param_2, local_22e));
            uVar3 = uVar4;
        }
        struct_1010_5f1e(uStack262, CONCAT22(param_2, local_224), uVar3);
    }
    else
    {
        pcStack282 = (iVar2 + 0x1a);
        unk_str_op_1000_3d3e(CONCAT22(param_2, local_102), pcStack282);
        uStack284 = str_op_1000_3da4(CONCAT22(param_2, local_102));
        if(local_102[uStack284 - 0x1] != '\\')
        {
            local_102[uStack284]       = '\\';
            local_102[uStack284 + 0x1] = '\0';
        }
        pass1_1000_3cea(CONCAT22(param_2, local_102), UStack266);
    }
    if(local_102[0] != '\0')
    {
        uStack270 = *(param_1 + 0xe8);
        send_msg_1020_097e(uStack270, 0x1020);
        UpdateWindow16(0x1020);
        HStack272 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538, (LPCSTR)0x7f02);
        HStack274 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
        win_ui_op_1008_1414(param_1, CONCAT22(param_2, local_102), (LPCSTR)s_tile2_bmp_1050_1538, param_2, in_AF, uVar3);
        SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    }
    return;
}

void  pass1_1008_3018(u32 param_1, u8 *param_2, i16 param_3, u16 param_4)

{
    u32        UVar1;
    u16        uVar2;
    i16        iVar3;
    u16        uVar4;
    u16        uStack266;
    undefined4 uStack262;
    char       local_102[0x100];

    local_102[0] = '\0';
    uStack262    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    uVar2        = (uStack262 >> 0x10);
    iVar3        = uStack262;
    UVar1        = *(iVar3 + 0x12);
    uVar4        = (iVar3 + 0x14);
    uStack266    = UVar1;
    if((uVar4 | uStack266) == 0x0)
    {
        pass1_1008_30cc(param_1, 0x0, uVar4, param_3, param_4);
    }
    else
    {
        unk_str_op_1000_3d3e(CONCAT22(param_4, local_102), (iVar3 + 0x1a));
        uVar4 = str_op_1000_3da4(CONCAT22(param_4, local_102));
        if(local_102[uVar4 - 0x1] != '\\')
        {
            local_102[uVar4]       = '\\';
            local_102[uVar4 + 0x1] = '\0';
        }
        pass1_1000_3cea(CONCAT22(param_4, local_102), UVar1);
        if(local_102[0] != '\0')
        {
            message_box_op_1008_12dc(param_1, CONCAT22(param_4, local_102), 0x1000, param_4);
            return;
        }
    }
    return;
}


void  menu_ui_op_1008_09ba(u32 param_1, HWND16 param_2, RECT16 *param_3, HWND16 param_4)

{
    HMENU16 HVar1;
    i16     iVar2;
    u16     uVar3;
    POi1616 local_6;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if((iVar2 + 0xec) == 0x0)
    {
        HVar1                      = LoadMenu16(param_4, (LPCSTR)s_OPPOPMENU_1050_0150);
        *(HMENU16 *)(iVar2 + 0xec) = HVar1;
        if(HVar1 == 0x0)
        {
            return;
        }
        param_4                    = (HWND16)s_tile2_bmp_1050_1538;
        HVar1                      = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0);
        *(HMENU16 *)(iVar2 + 0xec) = HVar1;
        if(HVar1 == 0x0)
        {
            return;
        }
    }
    local_6.x = (i1616)param_3;
    local_6.y = param_2;
    ClientToScreen16(param_4, &local_6);
    TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0, 0x0, (i1616)PTR_LOOP_1050_0396, 0x0, local_6.y, (RECT16 *)local_6.x);
    return;
}


void  switchD_1008: 1091 ::caseD_a7(void)

{
    undefined4  uVar1;
    i16         unaff_BP;
    HINSTANCE16 unaff_CS;
    u16         unaff_SS;

    uVar1 = (unaff_BP + 0x6);
    ui_op_1008_2c4e(uVar1, (uVar1 >> 0x10), 0x57, unaff_CS);
    return;
}


void  switchD_1008: 1091 ::caseD_aa(void)

{
    undefined4  uVar1;
    i16         unaff_BP;
    HINSTANCE16 unaff_CS;
    u16         unaff_SS;

    uVar1 = (unaff_BP + 0x6);
    ui_op_1008_2c4e(uVar1, (uVar1 >> 0x10), 0x58, unaff_CS);
    return;
}


void  switchD_1008: 1091 ::caseD_ac(void)

{
    undefined4  uVar1;
    i16         unaff_BP;
    HINSTANCE16 unaff_CS;
    u16         unaff_SS;

    uVar1 = (unaff_BP + 0x6);
    ui_op_1008_2c4e(uVar1, (uVar1 >> 0x10), 0x59, unaff_CS);
    return;
}


void  switchD_1008: 1091 ::caseD_ad(void)

{
    undefined4  uVar1;
    i16         unaff_BP;
    HINSTANCE16 unaff_CS;
    u16         unaff_SS;

    uVar1 = (unaff_BP + 0x6);
    ui_op_1008_2c4e(uVar1, (uVar1 >> 0x10), 0x5a, unaff_CS);
    return;
}


void  switchD_1008: 1091 ::caseD_ae(void)

{
    undefined4  uVar1;
    i16         unaff_BP;
    HINSTANCE16 unaff_CS;
    u16         unaff_SS;

    uVar1 = (unaff_BP + 0x6);
    ui_op_1008_2c4e(uVar1, (uVar1 >> 0x10), 0x5b, unaff_CS);
    return;
}


void  switchD_1008: 1091 ::caseD_b1(void)

{
    undefined4  uVar1;
    i16         unaff_BP;
    HINSTANCE16 unaff_CS;
    u16         unaff_SS;

    uVar1 = (unaff_BP + 0x6);
    ui_op_1008_2c4e(uVar1, (uVar1 >> 0x10), 0x5c, unaff_CS);
    return;
}


void  switchD_1008: 1091 ::caseD_b3(void)

{
    undefined4  uVar1;
    i16         unaff_BP;
    HINSTANCE16 unaff_CS;
    u16         unaff_SS;

    uVar1 = (unaff_BP + 0x6);
    ui_op_1008_2c4e(uVar1, (uVar1 >> 0x10), 0x5d, unaff_CS);
    return;
}


void  draw_op_1008_1230(HWND16 param_1)

{
    fill_rect_1008_39ac(param_1);
    return;
}


void  message_box_op_1008_12dc(u32 param_1, u32 param_2, HINSTANCE16 param_3, u16 param_4)

{
    BOOL16    BVar1;
    u16       uVar2;
    u16       in_DX;
    u16       uVar3;
    u8        in_AF;
    char     *pcVar4;
    u32       uStack36;
    u32       uStack16;
    u8        local_c[0x6];
    HCURSOR16 HStack6;
    HCURSOR16 HStack4;

    HStack4 = LoadCursor16(param_3, (LPCSTR)0x7f02);
    HStack6 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    str_1008_6d8a(CONCAT22(param_4, local_c), param_2, in_DX, param_4, in_AF);
    BVar1 = file_fn_1008_6e02((u1632_t *)CONCAT22(param_4, local_c), s_tile2_bmp_1050_1538, param_4);
    if(BVar1 == 0x0)
    {
        SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
        pcVar4   = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        uVar3    = (pcVar4 >> 0x10);
        uVar2    = str_op_1008_60e8(pcVar4, uVar3);
        uStack16 = CONCAT22(uVar3, uVar2);
        pcVar4   = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        MessageBeep16(0x1010);
        MessageBox16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)&PTR_LOOP_1050_0010, (LPCSTR)pcVar4, (pcVar4 >> 0x10));
    }
    else
    {
        (_PTR_LOOP_1050_5748 + 0x8) = 0x0;
        SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
        pcVar4   = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        uVar3    = (pcVar4 >> 0x10);
        uVar2    = str_op_1008_60e8(pcVar4, uVar3);
        uStack36 = CONCAT22(uVar3, uVar2);
        pcVar4   = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        MessageBeep16(0x1010);
        MessageBox16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)0x40, (LPCSTR)pcVar4, (pcVar4 >> 0x10));
        uStack16 = uStack36;
    }
    fn_ptr_1000_17ce((astruct_18 *)(uStack16 & 0xffff | uVar3 << 0x10), 0x1000);
    close_file_1008_6dd0(CONCAT22(param_4, local_c), 0x1000);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  win_ui_op_1008_1414(astruct_72 **param_1, u32 param_2, LPCSTR param_3, u16 param_4, u8 param_5, u16 param_6)

{
    code     **ppcVar1;
    BOOL16     BVar2;
    u16        uVar3;
    i16        iVar4;
    u32       *puVar5;
    u32        uVar5;
    u8        *puVar6;
    u16        uVar7;
    u8        *type;
    u16        uVar8;
    u16        extraout_DX;
    i16        unaff_DI;
    u16        uVar9;
    u32       *puVar10;
    char      *pcVar11;
    u16       *puVar12;
    u8         uVar13;
    u8         uVar14;
    i16        iVar15;
    undefined4 local_2a;
    u16        uStack38;
    i16        iStack36;
    u8        *puStack34;
    u32        uStack32;
    u32        uStack28;
    u32        uStack24;
    u32        uStack20;
    u32        uStack16;
    u16       *puStack12;
    u8         local_8[0x6];
    u16        uVar10;

    puVar10 = str_1008_6d8a(CONCAT22(param_4, local_8), param_2, param_6, param_4, param_5);
    puVar6  = (puVar10 >> 0x10);
    BVar2   = read_file_1008_6e78((u1632_t)local_8, param_4, param_3, param_4);
    iVar15  = param_1;
    uVar9   = (param_1 >> 0x10);
    if(BVar2 == 0x0)
    {
        if(PTR_LOOP_1050_0310 == 0x0)
        {
            globals->PTR_LOOP_1050_0310 = 0x6d4;
        }
        pcVar11 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        uVar7   = (pcVar11 >> 0x10);
        uVar3   = str_op_1008_60e8(pcVar11, uVar7);
        pcVar11 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        type    = (pcVar11 >> 0x10);
        puVar6  = type;
        MessageBeep16(0x1010);
        MessageBox16((HWND16)s_tile2_bmp_1050_1538, (LPCSTR)&PTR_LOOP_1050_0010, (LPCSTR)pcVar11, type);
        fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar7, uVar3), 0x1000);
        param_3 = (LPCSTR)&globals->PTR_LOOP_1050_1000;
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    cursor_op_1008_2dcc(iVar15, uVar9, 0x8, param_3);
    puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, puVar6, unaff_DI);
    uVar8     = (puStack12 >> 0x10);
    uVar5     = *(puStack12 + 0x20);
    uStack16  = uVar5;
    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), uVar5);
    uStack20 = uVar5 & 0xffff | uVar8 << 0x10;
    uStack24 = *(uVar5 + 0x10);
    iVar4    = (uStack24 + 0x2) + -0x1;
    ppcVar1  = ((iVar15 + 0xe8) + 0x4);
    (**ppcVar1)(0x1030, (iVar15 + 0xe8), uStack16, (uStack16 >> 0x10), iVar4, 0x2);
    puVar6 = extraout_DX;
    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), 0x4000001);
    uStack28 = CONCAT22(puVar6, iVar4);
    uVar5    = *(iVar4 + 0x10);
    uStack32 = uVar5;
    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), uVar5);
    iStack36  = uVar5;
    local_2a  = (iStack36 + 0xc);
    uStack38  = (iStack36 + 0x10);
    puStack34 = puVar6;
    puVar5    = pass1_1030_5b00(uStack20);
    uVar13    = SUB21(&local_2a, 0x0);
    uVar14    = (undefined)(&local_2a >> 0x8);
    uVar3     = param_4;
    puVar12   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, puVar5, param_4, puVar6, &iStack36);
    puVar6    = (puVar12 >> 0x10);
    pass1_1018_179e(puVar12, CONCAT22(uVar3, CONCAT11(uVar14, uVar13)), 0x1018, param_4);
    uVar13  = 0x0;
    uVar14  = 0x4;
    iVar15  = 0x1b;
    uVar10  = 0x1;
    puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_4, puVar6, &iStack36);
    pass1_1010_043a(puVar12, CONCAT13(uVar14, CONCAT12(uVar13, uVar10)), iVar15, param_4);
    close_file_1008_6dd0(CONCAT22(param_4, local_8), 0x1010);
    return;
}

void  cleanup_ui_op_1008_0618(u16 *param_1)

{
    undefined4 *puVar1;
    u16         uVar2;
    astruct_18 *paVar3;
    code      **ppcVar4;
    i16         iVar5;
    u16         uVar6;
    u16         unaff_CS;
    HICON16     h_icon;
    u16         unaff_SS;
    u16         uVar7;
    u16         uVar8;

    uVar6         = (param_1 >> 0x10);
    iVar5         = param_1;
    *param_1      = 0x389e;
    (iVar5 + 0x2) = 0x1008;
    set_sys_color_1008_357e(param_1, 0x0, unaff_CS, unaff_SS);
    paVar3 = *(astruct_18 **)(iVar5 + 0xf8);
    uVar8  = (paVar3 >> 0x10);
    h_icon = 0x1000;
    fn_ptr_1000_17ce(paVar3, 0x1000);
    if((iVar5 + 0xec) != 0x0)
    {
        uVar8  = (iVar5 + 0xec);
        h_icon = (HICON16)s_tile2_bmp_1050_1538;
        DestroyMenu16(0x1000);
    }
    uVar7 = (iVar5 + 0xc2);
    DestroyIcon16(h_icon);
    (iVar5 + 0xc2) = 0x0;
    puVar1         = (iVar5 + 0xe0);
    uVar2          = (iVar5 + 0xe2);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar4 = *puVar1;
        (**ppcVar4)(s_tile2_bmp_1050_1538, puVar1, uVar2, 0x1, uVar7, uVar8);
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar5 + 0xd2)));
    *param_1      = 0x380a;
    (iVar5 + 0x2) = 0x1008;
    *param_1      = 0x389a;
    (iVar5 + 0x2) = 0x1008;
    return;
}


void  win_ui_cursor_op_1008_06c0(u32 *param_1, u32 param_2, u16 param_3, i16 param_4)

{
    code     **ppcVar1;
    u16        in_AX;
    u16        in_DX;
    u8        *puVar2;
    u8        *extraout_DX;
    i16        unaff_DI;
    u16        uVar3;
    u8        *unaff_SS;
    u8         in_AF;
    char      *pcVar4;
    u16       *puVar5;
    u8         local_5a[0x50];
    undefined4 uStack10;
    HCURSOR16  HStack6;
    HCURSOR16  HStack4;

    if(param_4 == 0x400)
    {
        pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), 0x4000001);
        puVar2 = (in_DX | in_AX);
        if(puVar2 != 0x0)
        {
            if(PTR_LOOP_1050_4fe8 != 0x0)
            {
                pcVar4 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
                MessageBox16(0x1010, (LPCSTR)&PTR_LOOP_1050_0010, (LPCSTR)pcVar4, (pcVar4 >> 0x10));
                return;
            }
            HStack4 = LoadCursor16(0x1030, (LPCSTR)0x7f02);
            HStack6 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
            pass1_1030_83ba(_PTR_LOOP_1050_5748, param_2, unaff_SS, in_AF);
            uVar3                       = (_PTR_LOOP_1050_5748 >> 0x10);
            (_PTR_LOOP_1050_5748 + 0x8) = 0x1;
            uStack10                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, unaff_SS, puVar2, unaff_DI);
            pass1_1018_262e(uStack10);
            pass1_1030_8326();
            pcVar4 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (i1616)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            sys_1000_3f9c(local_5a, unaff_SS, 0x109, &USHORT_1050_1050, pcVar4, &stack0xfffe, uVar3, 0x1000, unaff_SS, in_AF);
            ppcVar1 = (*param_1 + 0x14);
            (**ppcVar1)(0x1000, param_1, (param_1 >> 0x10), 0x0, local_5a);
            puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, unaff_SS, extraout_DX, unaff_DI);
            pass1_1008_a9ec(puVar5);
            SetCursor16(0x1010);
            PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x11100fc);
        }
    }
    return;
}


BOOL16 msg_box_op_1000_1f24(i16 param_1, u16 param_2, u16 param_3, u16 param_4)

{
    i16 *piVar1;
    u16  unaff_CS;

    if(param_3 < (param_1 + 0xc))
    {
        msg_box_op_1000_214c(0x0, 0x0, 0xd940, &PTR_LOOP_1050_1040, param_4);
        return 0x1;
    }
    piVar1  = (param_1 + 0xc);
    *piVar1 = *piVar1 + 0x1;
    return 0x0;
}


BOOL16  pass1_1000_1f7e(u16 *param_1, u16 param_2)

{
    char   cVar1;
    BOOL16 BVar2;
    u16    uVar3;
    i16    iVar4;
    char  *pcVar5;

    uVar3 = *param_1;
    if(uVar3 == 0xf)
    {
    LAB_1000_1fb6:
        iVar4 = 0x1;
    }
    else
    {
        if(uVar3 < 0x10)
        {
            cVar1 = uVar3;
            if(cVar1 == '\x02')
                goto LAB_1000_1fb6;
            if(('\0' < (cVar1 + -0x2)) && ((cVar1 + -0x3) < '\f'))
            {
                iVar4 = 0x0;
                goto LAB_1000_1fbe;
            }
        }
        iVar4 = 0x0;
        uVar3 = 0x1;
    }
LAB_1000_1fbe:
    pcVar5 = pass1_1000_1fd2(uVar3);
    BVar2  = msg_box_op_1000_214c(0x0, iVar4, pcVar5, (pcVar5 >> 0x10), param_2);
    return BVar2;
}

BOOL16  msg_box_op_1000_214c(u16 param_1, i16 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    i1616  IVar1;
    i16    iVar2;
    LPCSTR text;

    text = (LPCSTR)(0x2 - (param_2 == 0x0) | 0x2110);
    MessageBeep16(param_5);
    do
    {
        IVar1 = MessageBox16((HWND16)s_tile2_bmp_1050_1538, text, (LPCSTR)0x1de8, 0x1000);
        iVar2 = IVar1 + -0x1;
        if(iVar2 == 0x0)
        {
            return 0x0;
        }
        if((0x0 < iVar2) && (!SBORROW2(iVar2, 0x1)))
        {
            if(IVar1 == 0x3 || IVar1 + -0x2 < 0x1)
            {
                fatal_app_exit_1000_3e9e(s_tile2_bmp_1050_1538);
                return 0x0;
            }
            if(IVar1 == 0x4)
            {
                return 0x1;
            }
            if(IVar1 == 0x5)
            {
                return 0x0;
            }
        }
        if((text & 0x2000) == 0x0)
        {
            return 0x0;
        }
        text = (LPCSTR)(text & 0xdfef | 0x1010);
    } while(true);
}


bool  mem_op_1000_21b6(u16 param_1, u16 param_2)

{
    BOOL16 BVar1;

    BVar1 = mem_op_1000_1dfa(0x0, 0x4, param_1, param_2);
    return BVar1 == 0x0;
}
