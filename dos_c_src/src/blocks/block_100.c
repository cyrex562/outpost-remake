


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


void __stdcall16far show_win_1038_c044(astruct_1 *param_1)

{
    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_op_1038_c07a(int param_1, ushort param_2, ushort param_3, ulong param_4, ushort param_5)

{
    int        iVar1;
    ushort     uVar2;
    undefined2 unaff_CS;
    HWND16     hwnd;
    uchar      in_AF;
    uchar      local_70c[0x200];
    char       local_50c[0x100];
    char       local_40c[0x402];
    ulong      uStack10;
    undefined4 uStack6;

    send_msg_1038_c228(CONCAT22(param_2, param_1), unaff_CS);
    uStack6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    if(param_4._2_2_ == 0x177)
    {
        pass1_1008_e05e(*(ulong *)(param_1 + 0x8e),
                        0x2,
                        CONCAT22(param_2, param_1 + 0x19eU),
                        CONCAT22(param_2, param_1 + 0x9e),
                        param_5,
                        in_AF);
        load_string_1010_84e0(
          0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x200, local_40c, param_5);
        sys_1000_3f9c(local_70c,
                      (uchar *)param_5,
                      (ushort)local_40c,
                      param_5,
                      param_1 + 0x19eU,
                      &stack0xfffe,
                      param_2,
                      0x1000,
                      param_5,
                      in_AF);
        load_string_1010_84e0(
          0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_50c, param_5);
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        MessageBox16(0x1010, (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10), local_50c, param_5);
    }
    else
    {
        if(param_4._2_2_ != 0x178)
        {
            if((param_4._2_2_ != 0x178) && (param_4._2_2_ - 0x179U < 0x2))
            {
                set_win_pos_1038_c31a(CONCAT22(param_2, param_1), param_3, (ushort)param_4, 0x1010);
                return;
            }
            post_win_msg_1040_7b3c(
              (ulong *)CONCAT22(param_2, param_1), param_3, (ushort)param_4, param_4._2_2_, (int)&PTR_LOOP_1050_1040);
            return;
        }
        uStack10 = CONCAT22(param_2, param_1 + 0x9e);
        uVar2    = param_2;
        iVar1    = pass1_1008_e10c(*(ulong *)(param_1 + 0x8e),
                                CONCAT22(param_2, param_1 + 0x19e),
                                CONCAT22(param_2, param_1 + 0x9e),
                                param_2,
                                param_5);
        if(iVar1 == 0x0)
        {
            load_string_1010_84e0(0x1010,
                                  (ushort)_PTR_LOOP_1050_14cc,
                                  (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                                  0x3ff,
                                  local_40c,
                                  param_5);
            load_string_1010_84e0(0x1010,
                                  (ushort)_PTR_LOOP_1050_14cc,
                                  (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                                  0x3ff,
                                  local_50c,
                                  param_5);
            MessageBox16(0x1010, (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10), local_50c, param_5);
            return;
        }
        hwnd = 0x1008;
        pass1_1008_e01c(*(ulong *)(param_1 + 0x8e), CONCAT22(param_2, param_1 + 0x19e), uStack10);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(param_1 + 0x8), 0x1f, uVar2, param_1, 0x1008, param_5);
    }
    PostMessage16(hwnd, 0x0, 0x0, 0x1110002);
    return;
}


LRESULT __stdcall16far send_msg_1038_c228(ulong param_1, HWND16 param_2)

{
    WPARAM16 wparam;
    LRESULT  LVar1;
    LRESULT  LVar2;

    wparam = (WPARAM16)(param_1 >> 0x10);
    LVar1  = SendMessage16(param_2, 0x0, 0x0, 0x4070000);
    LVar2  = SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4070000);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, (int)param_1 + 0x9e, wparam, CONCAT22(0x408, (int)LVar1));
    LVar1 = SendMessage16((HWND16)s_tile2_bmp_1050_1538, (int)param_1 + 0x19e, wparam, CONCAT22(0x408, (int)LVar2));
    return LVar1;
}


void __stdcall16far enable_win_1038_c294(ulong param_1)

{
    SEGPTR     lp_string;
    ushort     uVar1;
    undefined2 unaff_SS;
    ulong      uStack12;

    lp_string = (int)param_1 + 0x9e;
    uStack12  = param_1 & 0xffff0000 | (ulong)lp_string;
    pass1_1008_e320(*(astruct_102 **)((int)param_1 + 0x8e),
                    param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x19eU),
                    param_1 & 0xffff0000 | (ulong)lp_string,
                    unaff_SS);
    SetWindowText16(0x1008, lp_string);
    uVar1 = pass1_1008_e2a4(
      *(ulong *)((int)param_1 + 0x8e), param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x19eU), uStack12);
    EnableWindow16(0x1008, uVar1 & 0x1);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, uVar1 & 0x2);
    return;
}


BOOL16 __stdcall16far set_win_pos_1038_c31a(ulong param_1, ushort param_2, int param_3, HWND16 param_4)

{
    RECT16 local_e;
    int    iStack10;
    ushort uStack6;
    int    iStack4;

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


void __stdcall16far send_msg_1038_c374(ulong param_1, ulong *param_2, uint param_3, HWND16 param_4)

{
    undefined4  uVar1;
    code      **ppcVar2;
    undefined2  uVar3;
    ulong       uVar4;
    undefined2  extraout_DX;
    uint        extraout_DX_00;
    uint        uVar5;
    undefined2  uVar6;
    LRESULT     LVar7;
    astruct_18 *paVar8;
    uint        uVar9;
    ulong       uStack10;
    ulong       uStack6;

    uVar6   = SUB42(s_tile2_bmp_1050_1538, 0x0);
    uVar9   = param_3;
    LVar7   = SendMessage16(param_4, 0x0, 0x0, 0x40b0000);
    uVar3   = (undefined2)LVar7;
    uVar5   = (uint)param_2;
    ppcVar2 = (code **)((int)*param_2 + 0x10);
    (**ppcVar2)((int)s_tile2_bmp_1050_1538, param_2, uVar9);
    uStack6  = CONCAT22(extraout_DX, uVar3);
    uStack10 = 0x0;
    while(true)
    {
        if(uStack6 <= uStack10)
        {
            return;
        }
        ppcVar2 = (code **)((int)*param_2 + 0x4);
        uVar4   = uStack6;
        (**ppcVar2)(uVar6, param_2, (char)uStack10, (int)(uStack10 >> 0x10), uVar5);
        uVar1  = *(undefined4 *)((int)param_1 + 0x8e);
        paVar8 = (astruct_18 *)string_1008_e586((ushort)uVar1,
                                                (ushort)((ulong)uVar1 >> 0x10),
                                                uVar4 & 0xffff | (ulong)extraout_DX_00 << 0x10,
                                                (uint)uVar4,
                                                extraout_DX_00);
        uVar5  = param_3;
        LVar7  = SendMessage16(0x1008, (UINT16)paVar8, (WPARAM16)((ulong)paVar8 >> 0x10), 0x4030000);
        uVar6  = 0x1000;
        fn_ptr_1000_17ce(paVar8, 0x1000);
        if(LVar7 == -0x1)
            break;
        if(LVar7 == -0x2)
        {
            return;
        }
        uStack10 = uStack10 + 0x1;
    }
    return;
}


astruct_18 *__stdcall16far pass1_1038_c410(astruct_18 *param_1, byte param_2)

{
    pass1_1038_be4a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_c4a2(astruct_57 *param_1,
                                    ulong       param_2,
                                    ushort      param_3,
                                    ushort      param_4,
                                    ushort      param_5,
                                    uchar      *param_6,
                                    int         param_7,
                                    ushort      param_8)

{
    astruct_708 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x17c, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_708 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    iVar1->field_0x92                 = 0x0;
    iVar1->field_0x96                 = 0x0;
    *(undefined2 *)param_1            = 0xc74c;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_8, param_6, param_7);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_c4fe(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xc74c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_c52a(ushort param_1, ulong param_2, uchar *param_3, int param_4, ushort param_5)

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


void __stdcall16far show_win_1038_c558(astruct_1 *param_1)

{
    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16((HWND16)&PTR_LOOP_1050_1040, 0x5);
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_dlg_op_1038_c58e(ulong param_1, WORD *param_2)

{
    uchar     *in_DX;
    int        iVar1;
    int        unaff_DI;
    CHAR       local_80e[0x402];
    CHAR       local_40c[0x402];
    undefined4 uStack10;
    ushort    *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, (ushort)param_2, in_DX, unaff_DI);
    uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
    iVar1    = (int)param_1;
    GetWindowText16(0x1010, 0x80, (INT16)local_40c);
    wsprintf16((LPSTR)s_tile2_bmp_1050_1538, local_80e, param_2);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)local_80e);
    pass1_1008_e038(*(ulong *)(iVar1 + 0x8e),
                    (ulong *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0x92)),
                    (ulong *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0x96)));
    load_string_1010_84e0(0x1010,
                          (ushort)_PTR_LOOP_1050_14cc,
                          (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                          0x400,
                          local_80e,
                          (short)param_2);
    wsprintf16((LPSTR)0x1010, local_40c, param_2);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538, (INT16)local_40c, (SEGPTR)param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far message_box_op_1038_c672(int param_1, ushort param_2, ushort param_3, ulong param_4, short param_5)

{
    undefined4 uVar1;
    HWND16     hwnd;
    uchar      in_AF;
    ushort     uVar2;
    char       local_404[0x402];

    uVar2 = (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
    if(param_4._2_2_ == 0x17d)
    {
        load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, uVar2, 0x3ff, local_404, param_5);
        uVar1 = *(undefined4 *)(param_1 + 0x92);
        hwnd  = (HWND16)s_tile2_bmp_1050_1538;
        MessageBox16(0x1010,
                     (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10),
                     (LPCSTR)uVar1,
                     (UINT16)((ulong)uVar1 >> 0x10));
    }
    else
    {
        if(param_4._2_2_ != 0x17e)
        {
            post_win_msg_1040_7b3c(
              (ulong *)CONCAT22(param_2, param_1), param_3, (ushort)param_4, param_4._2_2_, (int)&PTR_LOOP_1050_1040);
            return;
        }
        load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, uVar2, 0x3ff, local_404, param_5);
        uVar1 = *(undefined4 *)(param_1 + 0x92);
        MessageBox16(0x1010,
                     (LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10),
                     (LPCSTR)uVar1,
                     (UINT16)((ulong)uVar1 >> 0x10));
        hwnd = 0x1008;
        pass1_1008_e164(*(ulong *)(param_1 + 0x8e), param_5, in_AF);
    }
    PostMessage16(hwnd, 0x0, 0x0, 0x1110002);
    return;
}


astruct_18 *__stdcall16far pass1_1038_c726(astruct_18 *param_1, byte param_2)

{
    pass1_1038_c4fe(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *__stdcall16far pass1_1038_c7b8(
  astruct_57 *param_1, ulong param_2, ushort param_3, ushort param_4, ushort param_5, uchar *param_6, ushort param_7)

{
    astruct_435 *iVar1;
    int          unaff_DI;
    undefined2   uVar1;
    ushort      *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb8, param_5);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_435 *)param_1;
    *(undefined4 *)&iVar1->field_0x8e = 0x0;
    *(undefined2 *)param_1            = 0xca6c;
    iVar1->field_0x2                  = (int)&PTR_LOOP_1050_1038;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x5, param_7, param_6, unaff_DI);
    iVar1->field_0x8e                 = (int)puVar2;
    iVar1->field_0x90                 = (int)((ulong)puVar2 >> 0x10);
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_c80a(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0xca6c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, (int)&PTR_LOOP_1050_1040);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far destroy_window_1038_c836(int param_1, ULONG param_2, ULONG param_3, UINT16 param_4)

{
    undefined4 uVar1;
    ushort    *puVar2;
    undefined  local_6[0x4];

    if(param_3._2_2_ == 0xfce)
    {
        puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4, local_6), 0x1, 0xac);
        win_1008_5c9e(
          _PTR_LOOP_1050_02a0, (ulong *)CONCAT22(param_4, local_6), local_6, (int)((ulong)puVar2 >> 0x10), param_4);
        uVar1                             = *(undefined4 *)(param_1 + 0x8e);
        *(undefined2 *)((int)uVar1 + 0xa) = 0x6;
        DestroyWindow16(0x1008);
        PTR_LOOP_1050_5b80 = (undefined *)0x0;
        return;
    }
    post_win_msg_1040_7b3c((ulong *)CONCAT22((undefined2)param_2, param_1),
                           (ushort)(param_2 >> 0x10),
                           (ushort)param_3,
                           param_3._2_2_,
                           (int)&PTR_LOOP_1050_1040);
    return;
}


void __stdcall16far win_ui_op_1038_c89c(astruct_1 *param_1)

{
    int        iVar1;
    undefined4 uVar2;
    HWND16     HVar3;
    undefined2 uVar4;
    BOOL16     enable;

    dialog_ui_fn_1040_78e2(param_1, (int)&PTR_LOOP_1050_1040);
    uVar4 = (undefined2)((ulong)param_1 >> 0x10);
    CheckRadioButton16((HWND16)&PTR_LOOP_1050_1040, 0xfac, 0xfad, 0xfac);
    uVar2                             = *(undefined4 *)((int)param_1 + 0x8e);
    *(undefined2 *)((int)uVar2 + 0xa) = 0x1;
    uVar2                             = *(undefined4 *)((int)param_1 + 0x8e);
    iVar1                             = *(int *)((int)uVar2 + 0x12);
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
