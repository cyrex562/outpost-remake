
void  win_dlg_op_1038_bea4(param_1: u32, WORD *param_2)

{
    u32 uVar1;
    HWND16     HVar2;
    u8        *in_DX;
    u8        *puVar3;
    u16        uVar4;
    WPARAM16   wparam;
    i16        iVar5;
    i16        unaff_DI;
    u16        uVar6;
    u16       *puVar7;
    u32        uVar8;
    char      *pcVar9;
    LRESULT    LVar10;
    u32       *local_116;
    u32       *local_112;
    CHAR       local_10e[0x82];
    u8         local_8c[0x82];
    u32 uStack10;
    u16       *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    puVar3   = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x68);
    uVar6    = (param_1 >> 0x10);
    iVar5    = param_1;
    GetWindowText16(0x1010, 0x80, (u16)local_8c);
    wspri16f16(s_tile2_bmp_1050_1538, local_10e, param_2);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)local_10e);
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x179);
    *(HWND16 *)(iVar5 + 0x92) = HVar2;
    pass1_1008_e3ec(*(iVar5 + 0x8e), CONCAT22(param_2, &local_116), CONCAT22(param_2, &local_112), param_2);
    send_msg_1038_c374(param_1, local_112, (iVar5 + 0x92), 0x1008);
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_2, puVar3, unaff_DI);
    uVar4  = (puVar7 >> 0x10);
    uVar8  = *(puVar7 + 0x24);
    uVar1  = (iVar5 + 0x8e);
    uVar8  = string_1008_e586(uVar1, (uVar1 >> 0x10), uVar8, uVar8, uVar4);
    SendMessage16(0x1008, uVar8, (WPARAM16)(uVar8 >> 0x10), 0x40dffff);
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x17a);
    *(HWND16 *)(iVar5 + 0x94) = HVar2;
    send_msg_1038_c374(param_1, local_116, HVar2, s_tile2_bmp_1050_1538);
    pcVar9         = load_string_1010_847e(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    wparam         = (WPARAM16)(pcVar9 >> 0x10);
    LVar10         = SendMessage16(0x1010, pcVar9, wparam, 0x4030000);
    (iVar5 + 0x9c) = LVar10;
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, pcVar9, wparam, 0x40dffff);
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x178);
    *(HWND16 *)(iVar5 + 0x96) = HVar2;
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x177);
    *(HWND16 *)(iVar5 + 0x98) = HVar2;
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x184);
    *(HWND16 *)(iVar5 + 0x9a) = HVar2;
    return;
}


void  show_win_1038_c044(astruct_1 *param_1)

{
    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    return;
}


void  msg_box_op_1038_c07a(i16 param_1, u16 param_2, u16 param_3, param_4: u32, u16 param_5)

{
    i16        iVar1;
    u16        uVar2;
    u16        unaff_CS;
    HWND16     hwnd;
    u8         in_AF;
    u8         local_70c[0x200];
    char       local_50c[0x100];
    char       local_40c[0x402];
    u32        uStack10;
    u32 uStack6;

    send_msg_1038_c228(CONCAT22(param_2, param_1), unaff_CS);
    uStack6 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    if(param_4._2_2_ == 0x177)
    {
        pass1_1008_e05e(*(param_1 + 0x8e), 0x2, CONCAT22(param_2, param_1 + 0x19eU), CONCAT22(param_2, param_1 + 0x9e), param_5, in_AF);
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x200, local_40c, param_5);
        sys_1000_3f9c(local_70c, param_5, local_40c, param_5, param_1 + 0x19eU, &stack0xfffe, param_2, 0x1000, param_5, in_AF);
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_50c, param_5);
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x10), local_50c, param_5);
    }
    else
    {
        if(param_4._2_2_ != 0x178)
        {
            if((param_4._2_2_ != 0x178) && (param_4._2_2_ - 0x179U < 0x2))
            {
                set_win_pos_1038_c31a(CONCAT22(param_2, param_1), param_3, param_4, 0x1010);
                return;
            }
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4._2_2_, &PTR_LOOP_1050_1040);
            return;
        }
        uStack10 = CONCAT22(param_2, param_1 + 0x9e);
        uVar2    = param_2;
        iVar1    = pass1_1008_e10c(*(param_1 + 0x8e), CONCAT22(param_2, param_1 + 0x19e), CONCAT22(param_2, param_1 + 0x9e), param_2, param_5);
        if(iVar1 == 0x0)
        {
            load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_40c, param_5);
            load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_50c, param_5);
            MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x10), local_50c, param_5);
            return;
        }
        hwnd = 0x1008;
        pass1_1008_e01c(*(param_1 + 0x8e), CONCAT22(param_2, param_1 + 0x19e), uStack10);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, (param_1 + 0x8), 0x1f, uVar2, param_1, 0x1008, param_5);
    }
    PostMessage16(hwnd, 0x0, 0x0, 0x1110002);
    return;
}


void  enable_win_1038_c294(u32 param_1)

{
    SEGPTR lp_string;
    u16    uVar1;
    u16    unaff_SS;
    u32    uStack12;

    lp_string = param_1 + 0x9e;
    uStack12  = param_1 & 0xffff0000 | lp_string;
    pass1_1008_e320(*(astruct_102 **)(param_1 + 0x8e), param_1 & 0xffff0000 | (param_1 + 0x19eU), param_1 & 0xffff0000 | lp_string, unaff_SS);
    SetWindowText16(0x1008, lp_string);
    uVar1 = pass1_1008_e2a4(*(param_1 + 0x8e), param_1 & 0xffff0000 | (param_1 + 0x19eU), uStack12);
    EnableWindow16(0x1008, uVar1 & 0x1);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, uVar1 & 0x2);
    return;
}


BOOL16  set_win_pos_1038_c31a(param_1: u32, u16 param_2, i16 param_3, HWND16 param_4)

{
    RECT16 local_e;
    i16    iStack10;
    u16    uStack6;
    i16    iStack4;

    iStack4 = param_3;
    uStack6 = param_2;
    if(param_3 == 0x1)
    {
        enable_win_1038_c294(param_1);
    }
    else
    {
        if(param_3 != 0x7)
        {
            return 0x0;
        }
        GetWindowRect16(param_4, &local_e);
        iStack10 = iStack10 - local_e.x;
        SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x2, 0x50, iStack10, 0x0, 0x0, 0x0);
    }
    return 0x1;
}


void  pass1_1038_c4fe(Struct18 *param_1)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0xc74c;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &PTR_LOOP_1050_1040);
    return;
}


void  pass1_1038_c52a(u16 param_1, param_2: u32, u8 *param_3, i16 param_4, u16 param_5)

{
    u16 *puVar1;
    i16  iVar2;

    if(param_2._2_2_ == 0x0)
    {
        iVar2  = 0x0;
        puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
        pass1_1010_038e(puVar1, iVar2, param_5);
    }
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), &PTR_LOOP_1050_1040);
    return;
}


void  show_win_1038_c558(astruct_1 *param_1)

{
    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    return;
}


void  win_dlg_op_1038_c58e(param_1: u32, WORD *param_2)

{
    u8        *in_DX;
    i16        iVar1;
    i16        unaff_DI;
    CHAR       local_80e[0x402];
    CHAR       local_40c[0x402];
    u32 uStack10;
    u16       *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uStack10 = (puStack6 + 0x68);
    iVar1    = param_1;
    GetWindowText16(0x1010, 0x80, (u16)local_40c);
    wspri16f16(s_tile2_bmp_1050_1538, local_80e, param_2);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)local_80e);
    pass1_1008_e038(*(iVar1 + 0x8e), (param_1 & 0xffff0000 | (iVar1 + 0x92)), (param_1 & 0xffff0000 | (iVar1 + 0x96)));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x400, local_80e, (short)param_2);
    wspri16f16(0x1010, local_40c, param_2);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538, (u16)local_40c, (SEGPTR)param_2);
    return;
}


void  message_box_op_1038_c672(i16 param_1, u16 param_2, u16 param_3, param_4: u32, short param_5)

{
    u32 uVar1;
    HWND16     hwnd;
    u8         in_AF;
    u16        uVar2;
    char       local_404[0x402];

    uVar2 = (_PTR_LOOP_1050_14cc >> 0x10);
    if(param_4._2_2_ == 0x17d)
    {
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, uVar2, 0x3ff, local_404, param_5);
        uVar1 = (param_1 + 0x92);
        hwnd  = (HWND16)s_tile2_bmp_1050_1538;
        MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x10), uVar1, (uVar1 >> 0x10));
    }
    else
    {
        if(param_4._2_2_ != 0x17e)
        {
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4._2_2_, &PTR_LOOP_1050_1040);
            return;
        }
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, uVar2, 0x3ff, local_404, param_5);
        uVar1 = (param_1 + 0x92);
        MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x10), uVar1, (uVar1 >> 0x10));
        hwnd = 0x1008;
        pass1_1008_e164(*(param_1 + 0x8e), param_5, in_AF);
    }
    PostMessage16(hwnd, 0x0, 0x0, 0x1110002);
    return;
}


void  pass1_1038_c80a(Struct18 *param_1)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0xca6c;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &PTR_LOOP_1050_1040);
    return;
}


void  destroy_window_1038_c836(i16 param_1, param_2: u32, param_3: u32, u16 param_4)

{
    u32 uVar1;
    u16       *puVar2;
    u8         local_6[0x4];

    if(param_3._2_2_ == 0xfce)
    {
        puVar2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0xac);
        win_1008_5c9e(_PTR_LOOP_1050_02a0, CONCAT22(param_4, local_6), local_6, (puVar2 >> 0x10), param_4);
        uVar1         = (param_1 + 0x8e);
        (uVar1 + 0xa) = 0x6;
        DestroyWindow16(0x1008);
        globals->PTR_LOOP_1050_5b80 = 0x0;
        return;
    }
    post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3, param_3._2_2_, &PTR_LOOP_1050_1040);
    return;
}


void  win_ui_op_1038_c89c(astruct_1 *param_1)

{
    i16        iVar1;
    u32 uVar2;
    HWND16     HVar3;
    u16        uVar4;
    BOOL16     enable;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    uVar4 = (param_1 >> 0x10);
    CheckRadioButton16((HWND16)&PTR_LOOP_1050_1040, 0xfac, 0xfad, 0xfac);
    uVar2         = (param_1 + 0x8e);
    (uVar2 + 0xa) = 0x1;
    uVar2         = (param_1 + 0x8e);
    iVar1         = (uVar2 + 0x12);
    if(iVar1 == 0x4)
    {
    LAB_1038_c8da:
        HVar3 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0xfce);
        if(HVar3 != 0x0)
        {
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        }
        HVar3 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        if(HVar3 == 0x0)
            goto LAB_1038_c93c;
        enable = 0x0;
    }
    else
    {
        if((iVar1 + -0x5 < 0x1) || (SBORROW2(iVar1 + -0x5, 0x1)))
            goto LAB_1038_c93c;
        if(iVar1 != 0x8 && 0x0 < iVar1 + -0x7)
        {
            if(iVar1 != 0x9)
                goto LAB_1038_c93c;
            goto LAB_1038_c8da;
        }
        HVar3 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0xfce);
        if(HVar3 == 0x0)
            goto LAB_1038_c93c;
        enable = 0x1;
    }
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, enable);
LAB_1038_c93c:
    move_win_1040_826c(param_1, 0xc8, 0x0);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    return;
}


void  enable_window_1038_9cec(i16 param_1, u16 param_2, u16 param_3, u16 param_4, i16 param_5, HWND16 param_6)

{
    i16   *piVar1;
    i16    iVar2;
    u16    uVar3;
    i16    iVar4;
    HWND16 HVar5;
    u8    *in_DX;
    i16    unaff_DI;
    u16    unaff_SS;
    u16   *puVar6;
    i16    iStack12;

    if(param_5 == 0xeb)
    {
        pass1_1040_b54a(param_1, param_2, param_3, CONCAT22(0xeb, param_4), in_DX, &PTR_LOOP_1050_1040, unaff_SS);
        puVar6   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, unaff_SS, in_DX, unaff_DI);
        iVar4    = puVar6 + 0xa4;
        uVar3    = (puVar6 >> 0x10);
        iStack12 = 0x0;
        HVar5    = 0x1010;
        while(iVar2 = iStack12 * 0x2, (iVar2 + iVar4) != 0x0)
        {
            HVar5                               = GetDlgItem16(HVar5, (iVar2 + iVar4));
            *(HWND16 *)(param_1 + iVar2 + 0x94) = HVar5;
            iStack12                            = iStack12 + 0x1;
            piVar1                              = (param_1 + 0x128);
            *piVar1                             = *piVar1 + 0x1;
            HVar5                               = (HWND16)s_tile2_bmp_1050_1538;
        }
    }
    else
    {
        if(param_5 == 0xf8)
        {
            GetDlgItem16(param_6, 0x17d8);
            HVar5 = 0x1;
        }
        else
        {
            if(param_5 != 0x17d8)
            {
                pass1_1040_b54a(param_1, param_2, param_3, CONCAT22(param_5, param_4), in_DX, &PTR_LOOP_1050_1040, unaff_SS);
                return;
            }
            SetWindowPos16(param_6, 0x6, 0xed, 0x237, 0x0, 0x0, 0x0);
            HVar5 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x17d8);
        }
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, HVar5);
    }
    return;
}


void  pass1_1038_9fa4(Struct18 *param_1)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0xa0b6;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &PTR_LOOP_1050_1040);
    return;
}


void  show_win_1038_9fd0(astruct_1 *param_1)

{
    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    return;
}


void  destroy_window_1038_a072(u16 param_1, u16 param_2, i16 param_3, HWND16 param_4)

{
    if(param_3 != 0x0)
    {
        DestroyWindow16(param_4);
    }
    return;
}


void  pass1_1038_a156(Struct18 *param_1)

{
    param_1->field_0x0 = 0xa2d0;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    ui_cleanup_op_1040_782c(param_1, &PTR_LOOP_1050_1040);
    return;
}


void  unk_win_ui_op_1038_a18c(astruct_1 *param_1, u16 param_2)

{
    code      **ppcVar1;
    u16       IVar2;
    u8         *in_DX;
    i16         unaff_DI;
    u16         uVar3;
    i16        *piVar4;
    u8         *puVar5;
    u16         uVar6;
    u16         uVar7;
    RECT16      local_2c;
    i16         iStack40;
    u16        *puStack36;
    i16         iStack32;
    u16         uStack30;
    i16         local_1c;
    u8          local_1a[0x2];
    u32         uStack24;
    astruct_76 *paStack20;
    i16         local_10;
    BOOL16      local_e;
    u8          local_c[0x6];
    u16        *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x27, param_2, in_DX, unaff_DI);
    pass1_1008_3e38(CONCAT22(param_2, local_c));
    pass1_1008_3f62(CONCAT22(param_2, local_c), (puStack6 & 0xffff0000 | (puStack6 + 0x52)));
    pass1_1008_3e94(CONCAT22(param_2, local_c), CONCAT22(param_2, &local_10), CONCAT22(param_2, &local_e));
    paStack20 = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1c0, param_2);
    uStack24  = pass1_1008_4772(paStack20);
    puVar5    = local_1a;
    piVar4    = &local_1c;
    uVar7     = param_2;
    puStack36 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_2, (uStack24 >> 0x10), unaff_DI);
    pass1_1008_3e94((puStack36 & 0xffff0000 | (puStack36 + 0xe)), CONCAT22(param_2, piVar4), CONCAT22(uVar7, puVar5));
    uVar3    = (puStack36 >> 0x10);
    uStack30 = (puStack36 + 0xa);
    iStack32 = (puStack36 + 0xc);
    local_10 = local_10 + (iStack32 * 0xa) / 0x258 + (uStack24 + 0x8) + local_1c;
    uVar3    = (param_1 + 0x6);
    GetWindowRect16(0x1008, &local_2c);
    uVar6   = 0x0;
    IVar2   = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    local_e = (IVar2 - (iStack40 - local_2c.x)) / 0x2;
    move_win_1040_826c(param_1, local_10, local_e);
    if(paStack20 != (astruct_76 *)0x0)
    {
        ppcVar1 = paStack20;
        (**ppcVar1)(&PTR_LOOP_1050_1040, paStack20, (paStack20 >> 0x10), 0x1, uVar6, uVar3, paStack20, paStack20);
    }
    return;
}


void  show_win_1038_a396(astruct_1 *param_1, u16 param_2, u16 param_3)

{
    u16 in_AX;
    u16 in_DX;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    unk_win_ui_op_1038_a18c(param_1, param_3);
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x10001, param_3, in_AX, in_DX);
    (param_1 + 0x8c) = in_AX;
    ShowWindow16(0x1008, 0x5);
    return;
}


void  win_ui_op_1038_a4ee(astruct_1 *param_1, u16 param_2)

{
    u32  uVar1;
    u8         *in_DX;
    i16         unaff_DI;
    WNDCLASS16 *unaff_SS;
    u16        *puVar2;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x20001, unaff_SS, param_2, in_DX);
    (param_1 + 0x8c) = param_2;
    puVar2           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, unaff_SS, in_DX, unaff_DI);
    uVar1            = (puVar2 + 0x6c);
    GetDlgItem16(0x1010, 0x114);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)uVar1);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0xffff, 0x4010000);
    unk_win_ui_op_1038_a18c(param_1, unaff_SS);
    ShowWindow16((HWND16)s_tile2_bmp_1050_1538, 0x5);
    return;
}


void  win_ui_op_1038_a584(u16 param_1, i16 param_2, HWND16 param_3, u16 param_4)

{
    u16  uVar1;
    u8  *in_DX;
    i16  unaff_DI;
    u16 *puVar2;
    u16  in_stack_00000006;
    u8  *puVar3;
    u8   local_52[0x50];

    if(param_2 != 0x0)
    {
        GetDlgItem16(param_3, 0x114);
        GetWindowText16((HWND16)s_tile2_bmp_1050_1538, 0x50, (u16)local_52);
        uVar1 = str_op_1000_3da4(CONCAT22(param_4, local_52));
        if(uVar1 != 0x0)
        {
            puVar3 = local_52;
            puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, in_DX, unaff_DI);
            pass1_1010_6006(puVar2, CONCAT22(param_4, puVar3), (puVar2 >> 0x10));
            GetWindowWord16(0x1010, -0x8);
            PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110105);
            destroy_win_1040_7b98(CONCAT22(in_stack_00000006, param_1), &PTR_LOOP_1050_1040);
        }
    }
    return;
}


void  win_ui_op_1038_a6f4(astruct_1 *param_1)

{
    u32  uVar1;
    u16         uVar2;
    u8         *in_DX;
    u16         uVar3;
    i16         unaff_DI;
    WNDCLASS16 *unaff_SS;
    u16        *puVar4;
    LRESULT     LVar5;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, unaff_SS, in_DX, unaff_DI);
    uVar1  = (puVar4 + 0x68);
    GetDlgItem16(0x1010, 0x115);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)uVar1);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    LVar5 = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0xffff, 0x4010000);
    uVar3 = (LVar5 >> 0x10);
    uVar2 = LVar5;
    unk_win_ui_op_1038_a18c(param_1, unaff_SS);
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x30001, unaff_SS, uVar2, uVar3);
    (param_1 + 0x8c) = uVar2;
    ShowWindow16(0x1008, 0x5);
    return;
}


void  win_ui_op_1038_a788(param_1: u32, i16 param_2, HWND16 param_3, u16 param_4)

{
    u16  uVar1;
    u8  *in_DX;
    i16  unaff_DI;
    u16 *pUVar2;
    u8  *puVar2;
    u8   local_52[0x50];
    u8  *puVar3;

    if(param_2 != 0x0)
    {
        GetDlgItem16(param_3, 0x115);
        GetWindowText16((HWND16)s_tile2_bmp_1050_1538, 0x50, (u16)local_52);
        uVar1 = str_op_1000_3da4(CONCAT22(param_4, local_52));
        if(uVar1 != 0x0)
        {
            puVar2 = local_52;
            pUVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, in_DX, unaff_DI);
            pass1_1010_5fd8(pUVar2, CONCAT22(param_4, puVar2), (pUVar2 >> 0x10));
            GetWindowWord16(0x1010, -0x8);
            PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110105);
            destroy_win_1040_7b98(param_1, &PTR_LOOP_1050_1040);
        }
    }
    return;
}

void  enable_win_1038_a8f8(u16 param_1, u16 param_2, u16 param_3, TwoWords param_4, HWND16 in_hwnd_5)

{
    BOOL16 enable;

    if(param_4.b_0x2 == 0x116)
    {
        SendDlgItemMessage16(in_hwnd_5, 0x0, 0x0, 0x1, 0x11a0401);
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x11a);
        enable = 0x0;
    }
    else
    {
        if((param_4.b_0x2 == 0x116) || (0x2 < param_4.b_0x2 - 0x117))
        {
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4.b_0x2, &PTR_LOOP_1050_1040);
            return;
        }
        GetDlgItem16(in_hwnd_5, 0x11a);
        enable = 0x1;
    }
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, enable);
    return;
}

void  win_ui_op_1038_a972(astruct_1 *param_1)

{
    BOOL16      BVar1;
    u16         uVar2;
    WNDCLASS16 *unaff_SS;
    LRESULT     LVar3;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    SendDlgItemMessage16((HWND16)&PTR_LOOP_1050_1040, 0x0, 0x0, 0x1, 0x1160401);
    LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x11a0401);
    uVar2 = (LVar3 >> 0x10);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x11a);
    BVar1 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x40001, unaff_SS, BVar1, uVar2);
    *(BOOL16 *)(param_1 + 0x8c) = BVar1;
    unk_win_ui_op_1038_a18c(param_1, unaff_SS);
    ShowWindow16(0x1008, 0x5);
    return;
}


void  win_sys_op_1038_a9fa(param_1: u32, param_2: i16)

{
    u8     *in_DX;
    i16     unaff_DI;
    u16     unaff_SS;
    u16    *puVar1;
    LRESULT LVar2;

    if(param_2 != 0x0)
    {
        puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, unaff_SS, in_DX, unaff_DI);
        LVar2  = SendDlgItemMessage16(0x1010, 0x0, 0x0, 0x0, 0x1160400);
        if(LVar2 == 0x0)
        {
            LVar2 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1170400);
            if(LVar2 == 0x0)
            {
                LVar2 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1180400);
                if(LVar2 == 0x0)
                {
                    LVar2 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1190400);
                    if(LVar2 != 0x0)
                    {
                        globals->PTR_LOOP_1050_13ae = &DAT_1050_0004;
                    }
                }
                else
                {
                    globals->PTR_LOOP_1050_13ae = (&PTR_LOOP_1050_0002 + 0x1);
                }
            }
            else
            {
                globals->PTR_LOOP_1050_13ae = &PTR_LOOP_1050_0002;
            }
        }
        else
        {
            globals->PTR_LOOP_1050_13ae = (&PTR_LOOP_1050_0000 + 0x1);
        }
        LVar2           = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x11a0400);
        (puVar1 + 0x82) = LVar2;
        GetWindowWord16((HWND16)s_tile2_bmp_1050_1538, -0x8);
        PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110105);
        destroy_win_1040_7b98(param_1, &PTR_LOOP_1050_1040);
    }
    return;
}


void  pass1_1038_abb0(Struct18 *param_1)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0xad72;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &PTR_LOOP_1050_1040);
    return;
}


void  set_win_pos_1038_abdc(HWND16 param_1)

{
    RECT16 local_12[0x2];
    RECT16 local_a;
    i16    iStack6;
    i16    iStack4;

    GetWindowRect16(param_1, &local_a);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0xfd7);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, local_12);
    iStack6 = iStack6 - local_a.x;
    iStack4 = (local_12[0].y - local_a.y) + -0x2;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x6, iStack4, iStack6, 0x0, 0x0, 0x0);
    return;
}


astruct_20 * pass1_1038_aeca(astruct_20 *param_1, u16 param_2)

{
    u16 uVar1;
    u16 local_b6;
    u16 uStack180;
    u8  local_5c[0x5a];

    uVar1            = (param_1 >> 0x10);
    (param_1 + 0xac) = 0x0;
    (param_1 + 0xae) = 0x0;
    if(_PTR_LOOP_1050_5b7c == (astruct_20 *)0x0)
    {
        globals->_PTR_LOOP_1050_5b7c = param_1;
    }
    pass1_1000_4906(param_1, 0x0, 0xac);
    unk_draw_op_1008_80ee((astruct_23 *)CONCAT22(param_2, local_5c), param_2);
    unk_win_ui_op_1040_9854(CONCAT22(param_2, &local_b6), param_2);
    local_b6  = 0x389a;
    uStack180 = 0x1008;
    pass1_1008_8168(CONCAT22(param_2, local_5c));
    return param_1;
}


u16  pass1_1038_8966(param_1: u32, u16 param_2, u16 param_3, i16 param_4, HWND16 param_5)

{
    i16 *piVar1;
    bool bVar2;
    i16  iVar3;
    u16  uVar4;

    bVar2 = false;
    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if(param_4 == 0x0)
    {
        if((iVar3 + 0x98) < 0x1)
            goto LAB_1038_89af;
        piVar1  = (iVar3 + 0x9a);
        *piVar1 = *piVar1 + 0x1;
        piVar1  = (iVar3 + 0x98);
        *piVar1 = *piVar1 + -0x1;
    }
    else
    {
        if(param_4 != 0x1)
            goto LAB_1038_89af;
        if((iVar3 + 0x9a) < 0x1)
            goto LAB_1038_89af;
        piVar1  = (iVar3 + 0x9a);
        *piVar1 = *piVar1 + -0x1;
        piVar1  = (iVar3 + 0x98);
        *piVar1 = *piVar1 + 0x1;
    }
    bVar2 = true;
LAB_1038_89af:
    if(bVar2)
    {
        SetDlgItemi1616(param_5, 0x0, (iVar3 + 0x9a), s_dibtext_bmp_1050_1844 + 0x9);
        SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x0, (iVar3 + 0x98), s_dibtext_bmp_1050_1844 + 0xb);
    }
    return 0x0;
}


void  pass1_1038_89e8(param_1: u32, u16 param_2)

{
    send_dlg_item_msg_1038_8b58(param_1, param_2);
    return;
}


void  pass1_1038_89f8(i16 param_1, u16 param_2, u16 param_3, param_4: u32, u8 *param_5, u16 param_6)

{
    if(param_4._2_2_ == 0xeb)
    {
        send_dlg_item_msg_1038_8b58(CONCAT22(param_2, param_1), param_6);
    }
    else
    {
        if(param_4._2_2_ != s_vrpal_bmp_1050_183a + 0x7)
        {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, &PTR_LOOP_1050_1040, param_6);
            return;
        }
        msg_box_ui_op_1038_8a3a(CONCAT22(param_2, param_1), 0x0, param_5, param_6);
    }
    return;
}


void  msg_box_ui_op_1038_8a3a(param_1: u32, char *param_2, u8 *param_3, u16 param_4)

{
    char  local_20a[0x102];
    char *pcStack264;
    u8   *puStack262;
    char  local_104[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    pcStack264 = param_2;
    puStack262 = param_3;
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(puStack262, pcStack264), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(puStack262, pcStack264), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x101, local_20a, param_4);
    MessageBox16(0x1010, 0x0, local_20a, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(puStack262, pcStack264), 0x1000);
    return;
}


void  unk_win_ui_op_1038_8afe(astruct_50 *param_1, HWND16 param_2, BOOL16 param_3)

{
    u32  uVar1;
    u16         dlg_item;
    u16         in_DX;
    astruct_50 *iVar4;
    astruct_50 *uVar4;
    BOOL16      local_4;

    uVar4    = (astruct_50 *)(param_1 >> 0x10);
    iVar4    = (astruct_50 *)param_1;
    dlg_item = GetDlgItemi1616(param_2, 0x0, &local_4, param_3);
    pass1_1030_6c1a(iVar4->field_0x94, dlg_item);
    uVar1 = iVar4->field_0x94;
    pass1_1038_387e(*(uVar1 + 0x2e), dlg_item, iVar4->field_0x9c, iVar4->field_0x94, in_DX);
    return;
}


void  send_dlg_item_msg_1038_8b58(param_1: u32, u16 param_2)

{
    u32 uVar1;
    u32        uVar2;
    u8        *in_DX;
    u16        uVar3;
    u16        uVar4;
    i16        iVar5;
    i16        unaff_DI;
    u16        uVar6;
    u8         in_AF;
    LRESULT    LVar7;
    u8         local_106[0x100];
    u16       *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_2, in_DX, unaff_DI);
    uVar3    = (puStack6 >> 0x10);
    uVar6    = (param_1 >> 0x10);
    iVar5    = param_1;
    pass1_1010_c3c2(puStack6, uVar3, CONCAT22(param_2, local_106), *(iVar5 + 0x94), uVar3, in_AF, param_2);
    LVar7          = SendDlgItemMessage16(0x1010, (u16)local_106, param_2, 0x0, 0x1846000c);
    uVar4          = (LVar7 >> 0x10);
    uVar1          = (iVar5 + 0x94);
    (iVar5 + 0x9c) = (uVar1 + 0x32);
    (iVar5 + 0x9a) = (iVar5 + 0x9c);
    SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x0, (iVar5 + 0x9c), s_dibtext_bmp_1050_1844 + 0x9);
    uVar1 = (iVar5 + 0x94);
    uVar2 = *(uVar1 + 0x2e);
    pass1_1038_3aa6(uVar2, uVar2, uVar4);
    (iVar5 + 0x98) = uVar2;
    SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x0, uVar2, s_dibtext_bmp_1050_1844 + 0xb);
    return;
}


void  pass1_1038_8d98(i16 param_1, u16 param_2, u16 param_3, param_4: u32, u8 *param_5, u16 param_6, u16 param_7)

{
    if(param_4._2_2_ == 0xeb)
    {
        send_dlg_item_msg_1038_8f74(CONCAT22(param_2, param_1), param_6, param_7);
    }
    else
    {
        if(param_4._2_2_ != s_vrpal_bmp_1050_183a + 0x7)
        {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, &PTR_LOOP_1050_1040, param_7);
            return;
        }
        msg_box_op_1038_8dda(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
    }
    return;
}


void  msg_box_op_1038_8dda(param_1: u32, char *param_2, u8 *param_3, u16 param_4)

{
    char local_206[0x102];
    char local_104[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(param_3, param_2), 0x1000);
    return;
}


LRESULT  send_dlg_item_msg_1038_8f74(param_1: u32, HWND16 param_2, WORD *param_3)

{
    i16        iVar1;
    u16        uVar2;
    long       lVar3;
    LRESULT    LVar4;
    BOOL16     enable;
    CHAR       local_50c[0x100];
    u8         local_40c[0x8];
    u32 local_404;

    uVar2 = (param_1 >> 0x10);
    SendDlgItemMessage16(param_2, 0x0, 0x0, 0x0, 0x185b000b);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x185b0405);
    iVar1 = pass1_1008_c83a(*(param_1 + 0x94));
    if(iVar1 == 0x0)
    {
        local_404 = pass1_1008_c85e(*(param_1 + 0x94), param_3);
        pass1_1008_5784(CONCAT22(param_3, local_40c), local_404);
        while(true)
        {
            lVar3 = pass1_1008_5b12(local_40c, param_3);
            if(lVar3 == 0x0)
                break;
            wspri16f16(0x1008, local_50c, param_3);
            SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, (u16)local_50c, param_3, 0x0, 0x185b0401);
        }
        GetDlgItem16(0x1008, 0x1);
        enable = 0x1;
    }
    else
    {
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, &local_404, (short)param_3);
        SendDlgItemMessage16(0x1010, (u16)&local_404, param_3, 0x0, 0x185b0401);
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        enable = 0x0;
    }
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, enable);
    LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x185b000b);
    return LVar4;
}


void  win_dlg_op_1038_9294(astruct_1 *param_1, u16 param_2)

{
    u16         UVar1;
    u16         uVar2;
    u16         in_DX;
    u16         uVar3;
    WNDCLASS16 *unaff_SS;
    BOOL16      local_6;
    BOOL16      local_4;

    unk_win_ui_op_1040_b230(param_1, &PTR_LOOP_1050_1040, unaff_SS);
    uVar3            = (param_1 >> 0x10);
    UVar1            = GetDlgItemi1616((HWND16)&PTR_LOOP_1050_1040, 0x1, &local_4, (BOOL16)unaff_SS);
    (param_1 + 0x94) = UVar1;
    uVar2            = GetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x1, &local_6, (BOOL16)unaff_SS);
    (param_1 + 0x96) = uVar2;
    win_ui_dlg_op_1038_98b4((astruct_51 *)(param_1 & 0xffff | uVar3 << 0x10), s_tile2_bmp_1050_1538, unaff_SS);
    win_1008_5c7c(_PTR_LOOP_1050_02a0, 0x950001, unaff_SS, uVar2, in_DX);
    return;
}


BOOL16  send_dlg_item_i16_1038_94da(i16 param_1, u16 param_2, u16 param_3, u16 param_4, i16 param_5, HWND16 param_6, BOOL16 param_7)

{
    u16   *pUVar1;
    i16    iVar2;
    long   lVar3;
    BOOL16 local_c;
    u16    uStack10;
    i16    iStack8;
    u16    UStack6;
    i16    iStack4;

    iStack4 = 0x1;
    iStack8 = pass1_1038_993a(param_1, param_2, param_3);
    if((-0x1 < iStack8) && (UStack6 = GetDlgItemi1616(param_6, 0x1, &local_c, param_7), local_c != 0x0))
    {
        if(param_5 == 0x0)
        {
            UStack6 = UStack6 + 0x1;
        }
        else
        {
            iStack4 = -0x1;
            UStack6 = UStack6 - 0x1;
        }
        uStack10 = (UStack6 <= (iStack8 * 0xe + 0x5a7a));
        pUVar1   = (iStack8 * 0xe + 0x5a78);
        if(*pUVar1 != UStack6 && UStack6 <= *pUVar1)
        {
            uStack10 = 0x0;
        }
        iVar2 = iStack8 * 0xe;
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, (iVar2 + 0x5a72));
        SetFocus16((HWND16)s_tile2_bmp_1050_1538);
        if((uStack10 != 0x0) && (lVar3 = unk_win_ui_op_1038_9820((astruct_51 *)CONCAT22(param_2, param_1), 0x1, iStack4, s_tile2_bmp_1050_1538, param_7), lVar3 != 0x0))
        {
            SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x1, UStack6, *(BOOL16 *)(iVar2 + 0x5a72));
            SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x1, (param_1 + 0x94), 0xfa9);
            SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x1, (param_1 + 0x96), 0xfa8);
        }
    }
    return 0x1;
}


void  enable_win_1038_9a66(u16 param_1, u16 param_2, u16 in_b_enable_3, param_4: u32, HWND16 in_hwnd_5)

{
    u8 *in_DX;
    u16 unaff_SS;

    if(param_4._2_2_ == 0xf8)
    {
        GetDlgItem16(in_hwnd_5, 0x17d9);
        in_b_enable_3 = 0x1;
    }
    else
    {
        if(param_4._2_2_ != 0x17d9)
        {
            pass1_1040_b54a(param_1, param_2, in_b_enable_3, param_4, in_DX, &PTR_LOOP_1050_1040, unaff_SS);
            return;
        }
        SetWindowPos16(in_hwnd_5, 0x6, 0x1a0, 0x12c, 0x0, 0x0, 0x0);
    }
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, in_b_enable_3);
    return;
}


void  unk_win_ui_op_1038_9bc8(astruct_1 *param_1)

{
    i16       *piVar1;
    code     **ppcVar2;
    i16        iVar3;
    u16      IVar4;
    HDC16      hdc;
    i16        iVar5;
    HWND16     HVar6;
    u8        *in_DX;
    u8        *puVar7;
    i16        iVar8;
    i16        unaff_DI;
    u16        uVar9;
    u16        unaff_SS;
    u16       *puVar10;
    u16        uVar12;
    u32 uVar11;
    u16       *puVar13;
    i16        iStack36;
    RECT16     local_16;
    i16        iStack16;
    i16        iStack14;
    i16        iStack12;
    u32 uStack10;
    i16        local_6;
    i16        local_4;

    dialog_ui_fn_1040_78e2(param_1, &PTR_LOOP_1050_1040);
    if(PTR_LOOP_1050_5ef8 == (&DAT_1050_0004 + 0x1))
    {
        globals->PTR_LOOP_1050_5ef8 = 0x0;
    }
    puVar13  = CONCAT22(unaff_SS, &local_4);
    puVar10  = CONCAT22(unaff_SS, &local_6);
    uStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((uStack10 & 0xffff0000 | (uStack10 + 0xe)), puVar10, puVar13);
    IVar4              = GetSystemMetrics16(0x1008);
    puVar7             = (((long)IVar4 * (long)PTR_LOOP_1050_5ef8) >> 0x10);
    iStack12           = ((long)IVar4 * (long)PTR_LOOP_1050_5ef8) + 0xa;
    globals->PTR_LOOP_1050_5ef8 = globals->PTR_LOOP_1050_5ef8 + 0x1;
    iStack14           = iStack12 + local_6;
    iStack12           = iStack12 + local_4;
    uVar9              = (param_1 >> 0x10);
    iVar8              = param_1;
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_16);
    uVar12 = 0x0;
    hdc    = GetDC16((HWND16)s_tile2_bmp_1050_1538);
    IVar4  = GetDeviceCaps16((HDC16)s_tile2_bmp_1050_1538, 0xa);
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, hdc);
    if(IVar4 < iStack16)
    {
        iStack14 = (local_16.y - (iStack16 - IVar4)) + 0x1;
    }
    uVar11 = CONCAT22(uVar12, (iVar8 + 0x6));
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x1, 0x0, 0x0, iStack14, iStack12, 0x0);
    puVar10  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, unaff_SS, puVar7, unaff_DI);
    iVar5    = puVar10 + 0xa4;
    uVar12   = (puVar10 >> 0x10);
    iStack36 = 0x0;
    HVar6    = 0x1010;
    while(iVar3 = iStack36 * 0x2, (iVar3 + iVar5) != 0x0)
    {
        HVar6                             = GetDlgItem16(HVar6, (iVar3 + iVar5));
        *(HWND16 *)(iVar8 + iVar3 + 0x94) = HVar6;
        iStack36                          = iStack36 + 0x1;
        piVar1                            = (iVar8 + 0x128);
        *piVar1                           = *piVar1 + 0x1;
        HVar6                             = (HWND16)s_tile2_bmp_1050_1538;
    }
    ppcVar2 = (param_1->field_0x0 + 0x6c);
    (**ppcVar2)(HVar6, iVar8, uVar9, uVar11);
    return;
}


void  destroy_window_1038_7d88(param_1: u32, u16 param_2)

{
    u16 in_DX;

    pass1_1008_b544(*(param_1 + 0x94), param_2, in_DX, 0x1008);
    DestroyWindow16(0x1008);
    return;
}


LRESULT  pass1_1038_7dac(param_1: u32, u16 param_2)

{
    LRESULT LVar1;

    pass1_1040_78de(param_1);
    LVar1 = send_dlg_item_msg_1038_844a(param_1, &PTR_LOOP_1050_1040, param_2);
    return LVar1;
}


void  pass1_1038_7dc6(i16 param_1, u16 param_2, u16 param_3, param_4: u32, u8 *param_5, u16 param_6, u16 param_7, u16 param_8)

{
    bool bVar1;

    bVar1 = false;
    if(param_4._2_2_ == 0x1854)
    {
        if(param_4 != 0x1)
            goto LAB_1038_7e8c;
        send_dlg_item_msg_1038_8618(CONCAT22(param_2, param_1), param_8);
    }
    else
    {
        if(param_4 < 0x18550000)
        {
            if(param_4._2_2_ == 0xeb)
            {
                send_dlg_item_msg_1038_844a(CONCAT22(param_2, param_1), param_7, param_8);
            }
            else
            {
                if(param_4._2_2_ == 0xfb)
                {
                    send_dlg_item_msg_1038_7eac(CONCAT22(param_2, param_1));
                }
                else
                {
                    if(param_4._2_2_ != s_vrpal_bmp_1050_183a + 0x7)
                    {
                    LAB_1038_7e77:
                        pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, &PTR_LOOP_1050_1040, param_8);
                        return;
                    }
                    msg_box_op_1038_81be(CONCAT22(param_2, param_1), 0x0, param_5, param_8);
                }
            }
            goto LAB_1038_7e8c;
        }
        if(param_4._2_2_ == 0x1855)
        {
            if(param_4 != 0x1)
                goto LAB_1038_7e8c;
            send_dlg_item_msg_1038_87b2(CONCAT22(param_2, param_1), param_7, param_8);
        }
        else
        {
            if(param_4._2_2_ == 0x1856)
            {
                if(param_4 != 0x1)
                    goto LAB_1038_7e8c;
                pass1_1038_8810(CONCAT22(param_2, param_1), param_7, param_8);
            }
            else
            {
                if(param_4._2_2_ == 0x1858)
                {
                    send_dlg_item_msg_1038_7fae(CONCAT22(param_2, param_1));
                }
                else
                {
                    if(param_4._2_2_ != 0x1859)
                        goto LAB_1038_7e77;
                    pass1_1038_801a(CONCAT22(param_2, param_1), param_5, param_6, param_8);
                }
            }
        }
    }
    bVar1 = true;
LAB_1038_7e8c:
    if(bVar1)
    {
        set_win_text_1038_8358(CONCAT22(param_2, param_1), param_7, param_8);
        enable_win_1038_806a(CONCAT22(param_2, param_1), param_7);
    }
    return;
}


LRESULT  send_dlg_item_msg_1038_7eac(u32 param_1)

{
    u8     *in_DX;
    i16     unaff_DI;
    u16     unaff_SS;
    u16    *puVar1;
    char   *pcVar2;
    LRESULT LVar3;

    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, unaff_SS, in_DX, unaff_DI);
    pcVar2 = pass1_1010_375e(puVar1);
    pass1_1008_b1a6(*(param_1 + 0x94), pcVar2);
    SendDlgItemMessage16(0x1008, 0x0, 0x0, 0x0, 0x1854000b);
    LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18540409);
    if(LVar3 != -0x1)
    {
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, (WPARAM16)LVar3, 0x18540403);
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, (u16)pcVar2, (pcVar2 >> 0x10), 0x0, 0x18540401);
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0xffff, 0x18540407);
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18550405);
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18560405);
        enable_win_1038_806a(param_1, s_tile2_bmp_1050_1538);
    }
    LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1854000b);
    return LVar3;
}


void  send_dlg_item_msg_1038_7fae(u32 param_1)

{
    u16     in_AX;
    u16     in_DX;
    i16     iVar1;
    u16     uVar2;
    u16     unaff_SS;
    LRESULT LVar3;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    pass1_1008_b146(*(iVar1 + 0x94), in_AX, in_DX);
    SendDlgItemMessage16(0x1008, 0x0, 0x0, 0xffff, 0x18550407);
    LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0xffff, 0x18560407);
    pass1_1008_b61a(*(iVar1 + 0x94), 0x0, LVar3, (LVar3 >> 0x10), unaff_SS);
    pass1_1008_b63a(*(iVar1 + 0x94), 0x0);
    return;
}


void  enable_win_1038_806a(param_1: u32, HWND16 param_2)

{
    BOOL16 BVar1;
    u16    in_DX;
    i16    iVar2;
    u16    uVar3;
    HWND16 hwnd_dlg;
    u32    uVar4;
    u32    uVar5;
    u32    uVar6;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    GetDlgItem16(param_2, 0x1);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1858);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1859);
    BVar1 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    uVar4 = pass1_1008_b820(*(iVar2 + 0x94), BVar1, in_DX);
    if(uVar4 != 0x0)
    {
        uVar4    = pass1_1008_b340(*(iVar2 + 0x94));
        uVar5    = pass1_1008_b366(*(iVar2 + 0x94));
        hwnd_dlg = 0x1008;
        uVar6    = pass1_1008_b47a(*(iVar2 + 0x94));
        if(((uVar4 != 0x0) && (uVar5 != 0x0)) && (uVar6 != 0x0))
        {
            GetDlgItem16(0x1008, 0x1);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1858);
            hwnd_dlg = (HWND16)s_tile2_bmp_1050_1538;
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        }
        if(uVar4 != 0x0)
        {
            GetDlgItem16(hwnd_dlg, 0x1859);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        }
    }
    return;
}


u16  send_dlg_item_msg_1038_8164(u16 param_1, u16 param_2, u8 *param_3, u16 param_4, HWND16 param_5)

{
    LRESULT LVar1;

    *param_3 = '\0';
    LVar1    = SendDlgItemMessage16(param_5, 0x0, 0x0, 0x0, CONCAT22(param_4, 0x409));
    if((LVar1 != -0x1) && (LVar1 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, (u16)param_3, (param_3 >> 0x10), (WPARAM16)LVar1, CONCAT22(param_4, 0x40a)), LVar1 != -0x1))
    {
        return 0x1;
    }
    return 0x0;
}


void  msg_box_op_1038_81be(param_1: u32, char *param_2, u8 *param_3, u16 param_4)

{
    char local_206[0x102];
    char local_104[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(param_3, param_2), 0x1000);
    return;
}


void  set_win_text_1038_8358(param_1: u32, HWND16 param_2, u16 param_3)

{
    char      *lp_string;
    u16        uVar2;
    u16        in_DX;
    u16        uVar4;
    u16        uVar3;
    HWND16     hwnd;
    char       local_30a[0x102];
    CHAR       local_208[0x100];
    u8         local_108[0x100];
    u32 uStack8;
    HWND16     HStack4;
    u32        uVar1;

    uVar3   = (param_1 >> 0x10);
    uVar4   = param_1;
    HStack4 = GetDlgItem16(param_2, 0x1857);
    uStack8 = pass1_1008_b820(*(uVar4 + 0x94), HStack4, in_DX);
    if(uStack8 == 0x0)
    {
        hwnd = 0x1010;
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_30a, param_3);
        lp_string = local_30a;
    }
    else
    {
        uVar2 = send_dlg_item_msg_1038_8164(uVar4, uVar3, CONCAT22(param_3, local_108), 0x1854, 0x1008);
        if(uVar2 == 0x0)
        {
            hwnd = 0x1010;
            load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_208, param_3);
        }
        else
        {
            hwnd = 0x1008;
            load_string_1008_b65a(*(uVar4 + 0x94), local_208, CONCAT22(local_108, param_3), param_3);
        }
        lp_string = local_208;
    }
    SetWindowText16(hwnd, (SEGPTR)lp_string);
    return;
}


void  send_dlg_item_msg_1038_8400(u16 param_1, u16 param_2, param_3: u32, u16 param_4, u16 param_5)

{
    u32 uVar1;
    long       lVar2;
    u8         local_a[0x8];

    pass1_1008_5784(CONCAT22(param_5, local_a), param_3);
    while(true)
    {
        lVar2 = pass1_1008_5b12(local_a, param_5);
        if(lVar2 == 0x0)
            break;
        uVar1 = (lVar2 + 0x4);
        SendDlgItemMessage16(0x1008, (u16)uVar1, (uVar1 >> 0x10), 0x0, CONCAT22(param_4, 0x401));
    }
    return;
}


LRESULT  send_dlg_item_msg_1038_844a(param_1: u32, HWND16 param_2, u16 param_3)

{
    BOOL16     BVar1;
    u16        uVar2;
    u16        uVar3;
    LRESULT    LVar4;
    char       local_108[0x102];
    u32 uStack6;

    uVar3 = (param_1 >> 0x10);
    SendDlgItemMessage16(param_2, 0x0, 0x0, 0x0, 0x1854000b);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1855000b);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1856000b);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18540405);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18550405);
    LVar4   = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18560405);
    uStack6 = pass1_1008_b820(*(param_1 + 0x94), LVar4, (LVar4 >> 0x10));
    if(uStack6 == 0x0)
    {
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_108, param_3);
        SendDlgItemMessage16(0x1010, (u16)local_108, param_3, 0x0, 0x18540401);
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1854000b);
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1855000b);
        LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1856000b);
        uVar2 = (LVar4 >> 0x10);
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1857);
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_108, param_3);
        BVar1 = SetWindowText16(0x1010, (SEGPTR)local_108);
        return CONCAT22(uVar2, BVar1);
    }
    send_dlg_item_msg_1038_8400(param_1, uVar3, uStack6, 0x1854, param_3);
    set_win_text_1038_8358(param_1, 0x1008, param_3);
    SendDlgItemMessage16(0x1008, 0x0, 0x0, 0x1, 0x1854000b);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1855000b);
    LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1856000b);
    return LVar4;
}


u16  send_dlg_item_msg_1038_8618(param_1: u32, u16 param_2)

{
    i16        in_AX;
    u16        uVar1;
    u8        *puVar2;
    u16        in_DX;
    u8        *puVar3;
    u16        msg;
    u16        uVar4;
    u16        uVar5;
    HWND16     hwnd;
    LRESULT    LVar6;
    u32        uVar7;
    u32        uVar8;
    u8         local_106[0x100];
    u32 uStack6;

    uVar5   = (param_1 >> 0x10);
    uVar4   = param_1;
    uStack6 = pass1_1008_b820(*(uVar4 + 0x94), in_AX, in_DX);
    uVar1   = uStack6;
    if(uStack6 != 0x0)
    {
        uVar1 = send_dlg_item_msg_1038_8164(uVar4, uVar5, CONCAT22(param_2, local_106), 0x1854, 0x1008);
        if(uVar1 != 0x0)
        {
            SendDlgItemMessage16(0x1008, 0x0, 0x0, 0x0, 0x1855000b);
            SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1856000b);
            SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18550405);
            LVar6  = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18560405);
            puVar3 = (LVar6 >> 0x10);
            puVar2 = local_106;
            pass1_1008_b4a0(*(uVar4 + 0x94), CONCAT22(param_2, puVar2), puVar2, puVar3, param_2);
            pass1_1008_b200(*(uVar4 + 0x94), param_2);
            uVar8 = CONCAT22(puVar3 | puVar2, puVar2);
            if((puVar3 | puVar2) != 0x0)
            {
                send_dlg_item_msg_1038_8400(uVar4, uVar5, CONCAT22(puVar3, puVar2), 0x1855, param_2);
                uVar7 = pass1_1008_b366(*(uVar4 + 0x94));
                msg   = (uVar7 >> 0x10);
                uVar8 = uVar7 & 0xffff | (msg | uVar7) << 0x10;
                if(uVar7 != 0x0)
                {
                    uVar8 = SendDlgItemMessage16(0x1008, uVar7, msg, 0xffff, 0x1855040d);
                }
            }
            hwnd  = 0x1008;
            uVar8 = pass1_1008_b38c(*(uVar4 + 0x94), uVar8, (uVar8 >> 0x10));
            if(uVar8 != 0x0)
            {
                send_dlg_item_msg_1038_8400(uVar4, uVar5, uVar8, 0x1856, param_2);
                hwnd  = 0x1008;
                uVar8 = pass1_1008_b47a(*(uVar4 + 0x94));
                if(uVar8 != 0x0)
                {
                    hwnd = (HWND16)s_tile2_bmp_1050_1538;
                    SendDlgItemMessage16(0x1008, (u16)uVar8, (uVar8 >> 0x10), 0xffff, 0x1856040d);
                }
            }
            SendDlgItemMessage16(hwnd, 0x0, 0x0, 0x1, 0x1855000b);
            LVar6 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1856000b);
            uVar1 = LVar6;
        }
    }
    return uVar1;
}


u16  send_dlg_item_msg_1038_87b2(param_1: u32, u16 param_2, u16 param_3)

{
    u16        uVar1;
    u16        uVar2;
    u16        in_DX;
    u32 uVar3;
    LRESULT    LVar4;
    u16        uVar5;
    u8         local_102[0x100];

    uVar5 = param_1;
    uVar1 = (param_1 >> 0x10);
    uVar2 = send_dlg_item_msg_1038_8164(uVar5, uVar1, CONCAT22(param_3, local_102), 0x1855, param_2);
    if(uVar2 != 0x0)
    {
        pass1_1008_b61a(*(uVar5 + 0x94), CONCAT22(param_3, local_102), local_102, in_DX, param_3);
        uVar3 = (uVar5 + 0x94);
        uVar3 = load_string_1008_b1f0(uVar3, (uVar3 >> 0x10));
        LVar4 = SendDlgItemMessage16(0x1008, (u16)uVar3, (uVar3 >> 0x10), 0xffff, 0x1856040d);
        uVar2 = LVar4;
    }
    return uVar2;
}


void  pass1_1038_8810(param_1: u32, u16 param_2, u16 param_3)

{
    u16 uVar1;
    u16 uVar2;
    u8  local_102[0x100];

    uVar2 = (param_1 >> 0x10);
    uVar1 = send_dlg_item_msg_1038_8164(param_1, uVar2, CONCAT22(param_3, local_102), 0x1856, param_2);
    if(uVar1 != 0x0)
    {
        pass1_1008_b63a(*(param_1 + 0x94), CONCAT22(param_3, local_102));
    }
    return;
}

void  pass1_1020_de32(param_1: u32, u16 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    BOOL16 BVar1;
    u16    uVar2;
    u16    uVar3;
    u16    uVar4;
    u16   *puVar5;

    puVar5          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x5, param_5, param_3, param_4);
    uVar2           = (puVar5 >> 0x10);
    (puVar5 + 0x12) = param_2;
    uVar3           = uVar2;
    BVar1           = bring_win_to_top_1038_b72e(_PTR_LOOP_1050_5b7c, 0x4, (HWND16)&PTR_LOOP_1050_1038);
    if(BVar1 == 0x0)
    {
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, (_PTR_LOOP_1050_4230 + 0x16), 0x4, uVar3, globals->_PTR_LOOP_1050_4230, &PTR_LOOP_1050_1038, param_5);
    }
    globals->PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 0x1);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5b80, 0x1008, param_5);
    uVar4            = (param_1 >> 0x10);
    (param_1 + 0x24) = (puVar5 + 0xa);
    if((param_1 + 0x24) == 0x0)
    {
        globals->PTR_LOOP_1050_50ca = 0x6b2;
    }
    return;
}

astruct_29 * pass1_1018_d198(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d1be(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d1e4(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d20a(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d230(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d256(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d27c(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d2a2(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d2c8(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d2ee(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d314(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d33a(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d360(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d386(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_29 * pass1_1018_d3ac(astruct_29 *param_1, u8 param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1020_8bcc(param_1: u32, u16 param_2)

{
    u32   uVar1;
    u32         *puVar2;
    u16          uVar3;
    u16          uVar4;
    u16         *puVar5;
    u8          *puVar6;
    u16          uVar7;
    u16          extraout_DX;
    astruct_285 *iVar9;
    astruct_286 *iVar10;
    u16          uVar8;
    u16          uVar9;
    astruct_43  *paVar10;
    u8           local_58[0x1e];
    u8           local_3a[0x26];
    u32          uStack20;
    u16          uStack12;
    astruct_76  *paStack10;
    u32          uStack6;

    uVar8 = (param_1 >> 0x10);
    iVar9 = (astruct_285 *)param_1;
    if(iVar9->field_0x4 != 0x0)
    {
        uVar1     = iVar9->field_0x22;
        uStack6   = *(uVar1 + 0xa);
        paStack10 = (astruct_76 *)pass1_1018_268e(iVar9->field_0x22);
        uVar9     = (paStack10 >> 0x10);
        uVar1     = iVar9->field_0x22;
        uStack12  = (uVar1 + 0x16);
        if(*iVar9->field_0xc == 0x0)
        {
            uStack20 = pass1_1008_4772(paStack10);
            puVar6   = (uStack20 >> 0x10);
            uVar3    = uStack20;
            mem_op_1000_179c(0x14, puVar6, 0x1000);
            uVar7 = puVar6 | uVar3;
            if(uVar7 == 0x0)
            {
                *iVar9->field_0xc = 0x0;
            }
            else
            {
                puVar5 = (param_1 & 0xffff0000 | &iVar9->field_0x16);
                uVar9  = (uStack20 >> 0x10);
                pass1_1008_50c2((astruct_110 *)CONCAT22(puVar6, uVar3), *(uStack20 + 0x8), *(uStack20 + 0x4), puVar5, uStack6);
                puVar2         = iVar9->field_0xc;
                puVar2         = puVar5;
                (puVar2 + 0x2) = uVar7;
            }
            pass1_1008_5134(*iVar9->field_0xc);
            paVar10 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x2, param_2);
            struct_op_1008_48fe((astruct_81 *)CONCAT22(param_2, local_3a), 0x1, paVar10, (paVar10 >> 0x10));
            struct_op_1008_3f92((astruct_76 *)CONCAT22(param_2, local_58), (astruct_83 *)CONCAT22(param_2, local_3a));
            uStack20 = pass1_1008_4772((astruct_76 *)CONCAT22(param_2, local_58));
            puVar6   = (uStack20 >> 0x10);
            uVar3    = uStack20;
            mem_op_1000_179c(0x14, puVar6, 0x1000);
            uVar7 = puVar6 | uVar3;
            if(uVar7 == 0x0)
            {
                puVar2         = iVar9->field_0xc;
                (puVar2 + 0x4) = 0x0;
            }
            else
            {
                uVar4 = &iVar9->field_0x16;
                uVar9 = (uStack20 >> 0x10);
                pass1_1008_50c2((astruct_110 *)CONCAT22(puVar6, uVar3), *(uStack20 + 0x8), *(uStack20 + 0x4), (param_1 & 0xffff0000 | uVar4), uStack6);
                puVar2            = iVar9->field_0xc;
                uVar9             = (puVar2 >> 0x10);
                iVar10            = (astruct_286 *)puVar2;
                iVar10->field_0x4 = uVar4;
                iVar10->field_0x6 = uVar7;
            }
            puVar2 = iVar9->field_0xc;
            pass1_1008_5134(*(puVar2 + 0x4));
            pass1_1008_41bc(CONCAT22(param_2, local_58));
            close_file_1008_496c(local_3a, param_2);
            uVar9 = extraout_DX;
        }
        puVar2 = iVar9->field_0xc;
        pass1_1008_5236(*(puVar2 + 0x4));
        pass1_1008_5236(*iVar9->field_0xc);
        uVar3 = &iVar9->field_0x16;
        pass1_1008_4480(uStack6, (param_1 & 0xffff0000 | uVar3), paStack10, param_2);
        invalidate_rect_1020_8d90(param_1, uStack12, uStack6, uVar3, uVar9, param_2);
    }
    return;
}

void  invalidate_rect_1020_8d90(param_1: u32, u16 param_2, param_3: u32, u16 param_4, u16 param_5, u16 param_6)

{
    u32 uVar1;
    i16        iVar2;
    u16        uVar3;
    u8         in_AF;
    i16        local_48;
    i16        iStack70;
    i16        iStack68;
    i16        iStack66;
    i16        local_40;
    i16        local_3e;
    u32        uStack60;
    u8         local_38[0x28];
    u8         local_10[0xa];
    u16        uStack6;
    u16        uStack4;

    uVar3   = (param_1 >> 0x10);
    iVar2   = param_1;
    uStack6 = pass1_1018_266a(*(iVar2 + 0x22));
    if(uStack6 != 0x0)
    {
        pass1_1018_265c((iVar2 + 0x22));
        if((param_5 | uStack6) != 0x0)
        {
            uStack4 = param_5;
            sys_1000_3f9c(local_10, param_6, s__03ld_1050_442a, &USHORT_1050_1050, uStack6, &stack0xfffe, uVar3, 0x1000, param_6, in_AF);
            uVar1 = (iVar2 + 0x22);
            file_and_draw_op_1008_4f20(CONCAT22(param_6, local_38), *(uVar1 + 0xe), 0x25, CONCAT22(param_6, local_10), param_6);
            pass1_1008_4480(param_3, (param_1 & 0xffff0000 | (iVar2 + 0x1c)), (astruct_76 *)CONCAT22(param_6, local_38), param_6);
            uStack60 = pass1_1008_4772((astruct_76 *)CONCAT22(param_6, local_38));
            pass1_1008_3e94((param_1 & 0xffff0000 | (iVar2 + 0x1c)), CONCAT22(param_6, &local_40), CONCAT22(param_6, &local_3e));
            local_48 = local_3e;
            iStack70 = local_40;
            uVar3    = (uStack60 >> 0x10);
            iStack68 = local_3e + (uStack60 + 0x4);
            iStack66 = local_40 + (uStack60 + 0x8);
            InvalidateRect16(0x1008, (RECT16 *)0x0, (BOOL16)&local_48);
            pass1_1008_41bc(CONCAT22(param_6, local_38));
        }
    }
    return;
}

void  invalidate_rect_1020_8fb4(param_1: u32, u16 param_2)

{
    i16        iVar1;
    u32 uVar2;
    u16        erase;
    u32        uVar3;
    u16        in_DX;
    u16        extraout_DX;
    u16        uVar4;
    i16        iVar5;
    u16        uVar6;
    u16        unaff_SS;
    i16        iStack8;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    uVar2 = (iVar5 + 0xba);
    if((uVar2 + 0x1e) != 0x0)
    {
        pass1_1018_2862(*(iVar5 + 0x16));
        (iVar5 + 0xaa) = param_2;
        (iVar5 + 0xac) = in_DX;
        if((in_DX | (iVar5 + 0xaa)) != 0x0)
        {
            uVar2 = (iVar5 + 0xaa);
            iVar1 = (uVar2 + 0xa);
            for(iStack8 = 0x0; iStack8 < iVar1; iStack8 = iStack8 + 0x1)
            {
                uVar3 = SEXT24(iStack8);
                empty_1008_8fc4((iVar5 + 0xaa), uVar3);
                erase = uVar3;
                uVar4 = extraout_DX | erase;
                if(((uVar4 != 0x0) && (0x9 < (erase + 0x2e))) && (pass1_1008_8b20(uVar3 & 0xffff | extraout_DX << 0x10, unaff_SS), (uVar4 | erase) != 0x0))
                {
                    InvalidateRect16(0x1008, (RECT16 *)0x0, erase);
                }
            }
        }
    }
    return;
}

void  set_struct_op_1020_921c(astruct_42 *param_1, u16 param_2)

{
    HDC16       HVar1;
    u8         *in_DX;
    astruct_42 *iVar3;
    i16         unaff_DI;
    astruct_42 *uVar3;
    HWND16      unaff_CS;
    u16         unaff_SS;
    u16        *pUVar3;

    uVar3              = (param_1 >> 0x10);
    iVar3              = param_1;
    param_1->field_0x0 = 0x389a;
    iVar3->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x3aa8;
    iVar3->field_0x2   = 0x1008;
    iVar3->field_0x4   = param_2;
    param_1->field_0x0 = 0x3ab0;
    iVar3->field_0x2   = 0x1008;
    iVar3->field_0x6   = 0x0;
    iVar3->field_0xa   = 0x0;
    iVar3->field_0xc   = 0x0;
    iVar3->field_0xe   = 0x0;
    iVar3->field_0x10  = 0x0;
    iVar3->field_0x12  = 0x0;
    param_1->field_0x0 = 0x96c8;
    iVar3->field_0x2   = 0x1020;
    HVar1              = GetDC16(unaff_CS);
    iVar3->field_0xa   = HVar1;
    pUVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pUVar3._2_2_       = (pUVar3 >> 0x10);
    iVar3->field_0xc   = (pUVar3 + 0xa);
    iVar3->field_0xe   = (pUVar3 + 0xc);
    return;
}

void  pass1_1020_770e(u32 param_1)

{
    u32 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    uVar5  = (param_1 >> 0x10);
    iVar4  = param_1;
    puVar1 = (iVar4 + 0xee);
    uVar2  = (iVar4 + 0xf0);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0xee) = 0x0;
    destroy_win_1008_628e(param_1 & 0xffff | uVar5 << 0x10, 0x1008);
    return;
}

void  cleanup_menu_ui_op_1020_795c(astruct_3 *in_struct_1, HMENU16 in_menu_handle_2)

{
    astruct_3 *local_struct_1;
    astruct_3 *uVar1;

    uVar1                                    = (astruct_3 *)(in_struct_1 >> 0x10);
    local_struct_1                           = (astruct_3 *)in_struct_1;
    in_struct_1->address_offset_field_0x0    = 0x7b86;
    local_struct_1->address_offset_field_0x2 = 0x1020;
    if(local_struct_1->field_0xec != 0x0)
    {
        DestroyMenu16(in_menu_handle_2);
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | &local_struct_1->field_0xd2));
    in_struct_1->address_offset_field_0x0    = 0x380a;
    local_struct_1->address_offset_field_0x2 = 0x1008;
    in_struct_1->address_offset_field_0x0    = 0x389a;
    local_struct_1->address_offset_field_0x2 = 0x1008;
    return;
}

void  get_win_ui_info_op_1020_7a50(param_1: u32, HWND16 param_2)

{
    fn_ptr_1 *ppcVar1;
    BOOL16    b_var2;
    u16     IVar2;
    u16     IVar3;
    u16       var5;
    RECT16    local_a;
    i16       iStack6;
    i16       iStack4;

    local_a.x = 0x0;
    local_a.y = 0x0;
    iStack6   = 0x0;
    iStack4   = 0x0;
    var5      = (param_1 >> 0x10);
    b_var2    = IsIconic16(param_2);
    if(b_var2 == 0x0)
    {
        GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_a);
        iStack6   = iStack6 - local_a.x;
        iStack4   = iStack4 - local_a.y;
        IVar2     = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
        IVar3     = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
        local_a.x = local_a.x + IVar2 * 0x2;
        local_a.y = local_a.y + IVar3 * 0x2;
    }
    ppcVar1 = ((param_1 + 0xe0) + 0x14);
    (**ppcVar1)(s_tile2_bmp_1050_1538, (param_1 + 0xe0), &local_a);
    return;
}


void  win_ui_menu_op_1020_7ad2(param_1: u32, HWND16 param_2, RECT16 *param_3, HWND16 param_4)

{
    HMENU16 HVar1;
    i16     iVar2;
    u16     uVar3;
    POi1616 local_6;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if(((iVar2 + 0xee) != 0x0) && ((iVar2 + 0xec) == 0x0))
    {
        HVar1                      = LoadMenu16(param_4,  * (iVar2 + 0xee));
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
    local_6.x = (u16)param_3;
    local_6.y = param_2;
    ClientToScreen16(param_4, &local_6);
    TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0, 0x0, (iVar2 + 0x8), 0x0, local_6.y, (RECT16 *)local_6.x);
    return;
}


astruct_3 * pass1_1020_7b60(astruct_3 *param_1, u8 param_2, u16 param_3)

{
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void  destroy_window_1020_8250(param_1: u32, HWND16 param_2)

{
    BOOL16 BVar1;
    u16    uVar2;

    uVar2 = (param_1 >> 0x10);
    if((param_1 + 0xec) != 0x0)
    {
        BVar1 = IsWindow16(param_2);
        if(BVar1 != 0x0)
        {
            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
            (param_1 + 0xec) = 0x0;
        }
    }
    return;
}
