use crate::block_1040::block_1040_2000::pass1_1040_2a22;
use crate::block_1040::block_1040_3000::{dialog_item_ui_op_1040_3e08, pass1_1040_3506, send_dlg_item_msg_1040_3f12, send_msg_1040_323c, set_win_pos_1040_331a};

pub unsafe fn draw_ui_op_1040_27cc(param_1: *mut astruct_752, hwnd16_param_2: HWND16, mut param_3: u16, hdc_param_4: HDC16) -> u32

{
    let mut uVar1: u32;
    let mut brush_handle_var8: HBRUSH16;
    let mut IVar3: i16;
    let mut iVar3: *mut astruct_752;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut uVar7: u16;
    let mut uVar4: u32;
    let mut hdc: HDC16;
    let mut iVar2: *mut astruct_753;
    let mut uVar2: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut fn_ptr_1: *mut *mut code;

    uVar7 = SUB42(&PTR_LOOP_1050_1040, 0x0);
    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.brush_handle_field4_0x4 == 0) {
        uVar7 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        brush_handle_var8 = CreateSolidBrush16(WHITE_BRUSH);
        iVar3.brush_handle_field4_0x4 = brush_handle_var8;
    }
    if (_u16_1050_5cf8 == 0) {
        fn_ptr_1 = (param_1 + 0x68);
        uVar1 = (**fn_ptr_1)(uVar7, param_1, iVar3.field109_0x6e);
        uVar4 = pass1_1008_4d72(uVar1);
        uVar2 = (uVar4 >> 0x10);
        iVar2 = uVar4;
        _u16_1050_5cf8 = CONCAT22(CONCAT11(0x2, iVar2.field_0x94), CONCAT11(iVar2.field_0x95, iVar2.field_0x96));
    }
    if (0x5 < param_3) {
        if (param_3 != 0x6) {
            return 0x0;
        }
        uVar5 = 0x284a;
        IVar3 = GetDlgCtrlID16(hwnd16_param_2);
        if ((iVar3.field146_0x94 != 0) && (IVar3 == 0xfb2)) {
            uVar6 = 0xff;
            hdc = 0;
            // TODO: goto LAB_1040_286e;
        }
    }
    uVar5 = _u16_1050_5cf8;
    uVar6 = (_u16_1050_5cf8 >> 0x10);
    hdc = hdc_param_4;//
// LAB_1040_286e:
    SetTextColor16(CONCAT22(uVar6, uVar5), hdc);
    SetBkColor16(0x1000000, hdc_param_4);
    return CONCAT22(0x1050, iVar3.brush_handle_field4_0x4);
}


pub unsafe fn pass1_1040_288e(mut param_1: u32)

{
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut paVar5: *mut astruct_394;
    let mut puVar6: *mut u32;
    let mut uVar7: u16;
    let mut puVar8: *mut u8;
    let mut in_EDX: u32;
    let mut paVar9: *mut Struct57;
    let mut iVar10: i16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;

    uVar12 = (param_1 >> 0x10);
    iVar10 = param_1;
    uVar3 = (iVar10 + 0x8e);
    uVar13 = (uVar3 >> 0x10);
    iVar11 = uVar3;
    puVar6 = (iVar11 + 0x24);
    paVar9 = (in_EDX & 0xffff0000 | (iVar11 + 0x26));
    ppcVar2 = (*puVar6 + 0x14);
    (**ppcVar2)();
    paVar5 = puVar6;
    uVar4 = paVar9 << 0x10;
    if ((iVar10 + 0x70) != 0) {
        paVar5 = (iVar10 + 0x70);
        uVar1 = (iVar10 + 0x72);
        uVar7 = uVar1 | paVar5;
        paVar9 = (paVar9 & 0xffff0000 | uVar7);
        if (uVar7 != 0) {
            ppcVar2 = paVar5;
            (**ppcVar2)();
        }
    }
    mem_op_1000_179c(0x14, paVar9);
    puVar8 = (paVar9 | paVar5);
    if (puVar8.is_null()) {
        paVar5 = null_mut();
        puVar8 = null_mut();
    } else {
        struct_1008_4c58(paVar5);
    }
    (iVar10 + 0x70) = paVar5;
    (iVar10 + 0x72) = puVar8;
    pass1_1008_4d84(puVar8, (iVar10 + 0x70), puVar6 & 0xffff | uVar4);
    return;
}


pub unsafe fn pass1_1040_2930(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_2464(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pas1_1040_29c2(mut param_1: u16, mut param_2: u16, param_3: *mut Struct57, mut param_4: u32, mut param_5: u16) -> *mut Struct57

{
    let mut iVar1: *mut Struct57;
    let mut uVar1: *mut Struct57;

    pass1_1040_b0bc(param_3, param_4, CONCAT22(param_5, 0x157));
    uVar1 = (param_3 >> 0x10);
    iVar1 = param_3;
    param_3.field0_0x0 = 0x2e26;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x64b);
    iVar1[0x1].field3_0x6 = param_1;
    iVar1[0x1].field4_0x8 = param_2;
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x64a);
    iVar1[0x1].field5_0xa = param_1;
    iVar1[0x1].field6_0xc = param_2;
    return param_3;
}

pub unsafe fn dlg_ui_op_1040_2a64(mut param_1: u16, struct_b_param_1: *mut StructB)

{
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut struct_b_6: *mut StructB;
    let mut iVar8: *mut astruct_918;
    let mut uVar7: u16;
    let mut in_stack_0000fe30: u16;
    let mut in_stack_0000fe34: u16;
    let mut in_stack_0000ff5a: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ffa2: u16;
    let mut iVar9: i16;
    let mut local_16: RECT16;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut uStack12: u32;
    let mut uStack8: u32;
    let mut iStack4: i16;
    let mut paVar6: *mut Struct57;

    unk_win_ui_op_1040_b230(param_1, struct_b_param_1);
    iStack4 = 0x5;
    iVar9 = 0;
    uVar7 = (struct_b_param_1 >> 0x10);
    struct_b_6 = struct_b_param_1;
    uVar1 = &struct_b_6[0x7].hwnd_0x6;
    uStack12 = struct_op_1030_73a8((uVar1 + 0x6), in_AX, param_1);
    paVar5 = CONCAT22(in_register_0000000a, (uStack12 >> 0x10));
    PTR_LOOP_1050_5d04 = pass1_1028_4a9a(uStack12, iVar9);
    for iStack14 in 0..iStack4 {
        if (iStack14 != 0) {
            (&PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0;
        }
        iVar9 = iStack14 * 0xc;
        local_16.x = (iVar9 + 0x5cfc);
        local_16.y = (iVar9 + 0x5cfe);
        paVar2 = (&PTR_LOOP_1050_0000 + 1);
        uStack18 = 0x1;
        uStack16 = 0x1;
        MapDialogRect16(&local_16, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar5);
        uVar4 = paVar5 | paVar2;
        paVar6 = (paVar5 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            paVar2 = null_mut();
            paVar5 = (paVar5 & 0xffff0000);
        } else {
            pass1_1008_3bd6(paVar6, paVar2, paVar5, 0x1, CONCAT22(local_16.x, local_16.y), 0x101, 0xff0100,
                            CONCAT22(struct_b_6.lpvoid_field_0x8, (iVar9 + 0x5d00)), in_stack_0000ffa2,
                            in_stack_0000fe30, in_stack_0000fe34, in_stack_0000ff5a, in_stack_0000ff5e, in_stack_0000ff62);
            paVar5 = paVar6;
        }
        uVar4 = paVar5;
        uStack8 = CONCAT22(uVar4, paVar2);
        if (PTR_LOOP_1050_5d04.is_null()) {
            if ((iStack14 != 0) && ((uVar4 | paVar2) != 0)) {
                EnableWindow16(0x0, &paVar2.field11_0x18);
            }
        } else {
            iVar8 = (iStack14 * 0xc);
            uVar3 = pass1_1028_4a9a(uStack12, (iVar8 + 0x5d02));
            if (uVar3 != 0) {
                (iVar8 + 0x5d04) = 0x1;
                SetDlgItemText16(&struct_b_6[0x7].field6_0xc, (iVar8 + 0x5d06),
                                 struct_b_6.lpvoid_field_0x8);
            }
        }
    }
    return;
}


pub unsafe fn win_ui_op_1040_2bb2(param_1: *mut u8, pstruct_903_param_2: *mut Struct903, mut param_3: u16, mut param_4: u32)

{
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut HVar3: HWND16;
    let mut iVar4: *mut astruct_922;
    let mut iVar5: i16;
    let mut iVar3: *mut astruct_920;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut id: *mut u8;
    let mut iStack8: i16;
    let mut iStack4: i16;

    if (param_4 == 0x158) {
        PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04.is_null());
        if (PTR_LOOP_1050_5d04.is_null()) {
            //   for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 1)
            for iStack8 in 1..5 {
                iVar5 = iStack8 * 0xc;
                HVar3 = GetDlgItem16((iVar5 + 0x5d00), (pstruct_903_param_2 + 0x6));
                EnableWindow16(0x0, HVar3);
                (&PTR_LOOP_1050_5d04 + iVar5) = 0;
                SetDlgItemText16((pstruct_903_param_2 + 0x94),
                                 (&PTR_s_post_1050_015d_1050_5d06 + iVar5),
                                 (pstruct_903_param_2 + 0x6));
            }
            HVar3 = (pstruct_903_param_2 + 0x6);
            uVar1 = (pstruct_903_param_2 + 0x94);
            uVar6 = uVar1;
            uVar7 = (uVar1 >> 0x10);
            id = PTR_s_post_1050_015d_1050_5d06;
            // TODO: goto LAB_1040_2ccc;
        }
        // for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 1)
        for iStack8 in 1..5 {
            iVar3 = (iStack8 * 0xc);
            HVar3 = GetDlgItem16((iVar3 + 0x5d00), (pstruct_903_param_2 + 0x6));
            EnableWindow16(0x1, HVar3);
            (iVar3 + 0x5d04) = 0;
            SetDlgItemText16((pstruct_903_param_2 + 0x94), (iVar3 + 0x5d06),
                             (pstruct_903_param_2 + 0x6));
        }
        HVar3 = (pstruct_903_param_2 + 0x6);
        id = PTR_s_post_1050_015d_1050_5d06;
    } else {
        if (param_4 == 0x159) {
            iStack4 = 0x1;
        } else if (param_4 == 0x15a) {
            iStack4 = 0x2;
        } else if (param_4 == 0x15b) {
            iStack4 = 0x3;
        } else {
            if (param_4 != 0x15c) {
                pass1_1040_b54a(param_1, pstruct_903_param_2, param_3, param_4);
                return;
            }
            iStack4 = 0x4;
        }
        if (iStack4 == 0) {
            return;
        }
        iVar4 = (iStack4 * 0xc);
        uVar2 = ((iVar4 + 0x5d04) == 0);
        (iVar4 + 0x5d04) = uVar2;
        if (uVar2 == 0) {
            HVar3 = (pstruct_903_param_2 + 0x6);
            uVar1 = (pstruct_903_param_2 + 0x94);
            uVar6 = uVar1;
            uVar7 = (uVar1 >> 0x10);
            id = (iVar4 + 0x5d06);
            // TODO: goto LAB_1040_2ccc;
        }
        HVar3 = (pstruct_903_param_2 + 0x6);
        id = (&PTR_s_post_1050_015d_1050_5d06 + iStack4 * 0xc);
    }
    uVar1 = (pstruct_903_param_2 + 0x98);
    uVar6 = uVar1;
    uVar7 = (uVar1 >> 0x10);//
// LAB_1040_2ccc:
    SetDlgItemText16(CONCAT22(uVar7, uVar6), id, HVar3);
    return;
}

pub unsafe fn win_dlg_item_1040_2d48(mut param_1: u32)

{
    let mut UVar1: u16;
    let mut value: u16;
    let mut local_4: bool;

    pass1_1040_b45e(param_1);
    UVar1 = GetDlgItemInt16(0x1, &local_4, &DAT_1050_1050, 0x163);
    value = GetDlgItemInt16(0x1, &local_4, &DAT_1050_1050, 0x165);
    if (UVar1 != 0) {
        value /= UVar1;
    }
    SetDlgItemInt16(0x1, value, 0x165, (param_1 + 0x6));
    return;
}


pub unsafe fn pass1_1040_2dac(mut param_1: u32)

{
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar2: u32;
    let mut iStack10: i16;

    uVar1 = (param_1 + 0x90);
    uVar2 = struct_op_1030_73a8((uVar1 + 0x6), in_AX, in_DX);
    for iStack10 in 0..0x5 {
        pass1_1028_4ab2(uVar2, (&PTR_LOOP_1050_5d04 + iStack10 * 0xc), (iStack10 * 0xc + 0x5d02));
    }
    return;
}


pub unsafe fn pass1_1040_2e00(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_2a22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_2ea2(param_1: *mut StructD, param_2: *mut Struct57, mut param_3: u32, mut param_4: u16, mut param_5: u16, mut param_6: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut iVar1: *mut Struct57;
    let mut unaff_BP: u16;
    let mut uVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    get_sys_metrics_1040_7728(param_2, 0x1, param_3, 0x180, param_6);
    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    (iVar1 + 1).field0_0x0 = 0;
    iVar1[0x1].field1_0x2 = 0;
    iVar1[0x1].field2_0x4 = 0;
    iVar1[0x1].field3_0x6 = 0;
    iVar1[0x1].field4_0x8 = 0;
    param_2.field0_0x0 = 0x3436;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(paVar1, _u16_1050_0ed0, CONCAT22(unaff_BP, 0x3c), in_stack_0000fea6,
                             in_stack_0000ffca, in_stack_0000ffd0, in_stack_0000ffd4);
    iVar1[0x1].field4_0x8 = puVar2;
    iVar1[0x1].field5_0xa = (puVar2 >> 0x10);
    return;
}


pub unsafe fn pass1_1040_2f32(mut param_1: u16, mut param_2: u16, mut param_3: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    return;
}

pub unsafe fn show_win_1040_2f5a(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn win_dlg_op_1040_2f90(mut param_1: u16, mut param_2: u32)

{
    let mut uVar1: u16;
    let mut HVar2: HWND16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: *mut astruct_943;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut uVar6: u32;
    let mut l_param: *mut c_char;
    let mut in_stack_0000fd7a: u16;
    let mut in_stack_0000fd7c: u16;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000fea4: u16;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000fea8: u16;
    let mut in_stack_0000feaa: u16;
    let mut in_stack_0000fed2: u16;
    let mut in_stack_0000fed4: u16;
    let mut local_116: *mut u32;
    let mut local_112: *mut u32;
    let mut local_10e: [u16; 0x41] = [0; 0x41];
    let mut local_8c: [u8; 0x82] = [0; 0x82];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    puStack6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fed2, 0x2), in_stack_0000fd7a,
                               in_stack_0000fe9e, in_stack_0000fea4, in_stack_0000fea8);
    paVar3 = (paVar3 & 0xffff0000 | puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x68);
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    GetWindowText16(0x80, CONCAT22(0x1050, local_8c), iVar4.field6_0x6);
    wsprintf16(local_10e, CONCAT22(local_8c, 0x1050), CONCAT22(uStack10, 0x1050),
               (uStack10 >> 0x10));
    SetWindowText16(CONCAT22(0x1050, local_10e), iVar4.field6_0x6);
    HVar2 = GetDlgItem16(0x182, iVar4.field6_0x6);
    iVar4.field143_0x92 = HVar2;
    pass1_1018_3a94(iVar4.field145_0x96, CONCAT22(0x1050, &local_116), CONCAT22(0x1050, &local_112));
    send_msg_1040_3374(param_2, local_112, iVar4.field143_0x92);
    puVar5 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fed4, 0x2f), in_stack_0000fd7c,
                             in_stack_0000fea0, in_stack_0000fea6, in_stack_0000feaa);
    uVar1 = (puVar5 >> 0x10);
    uVar6 = (puVar5 + 0x24);
    uVar6 = pass1_1018_3a7a(uVar6, uVar1, iVar4.field145_0x96, uVar6);
    SendMessage16(uVar6, 0xffff, 0x40d, iVar4.field143_0x92);
    HVar2 = GetDlgItem16(0x183, iVar4.field6_0x6);
    iVar4.field144_0x94 = HVar2;
    send_msg_1040_3374(param_2, local_116, HVar2);
    l_param = load_string_1010_847e(_u16_1050_14cc, 0x531);
    SendDlgItemMessage16(l_param, 0x0, 0x403, 0x183, iVar4.field6_0x6);
    SendDlgItemMessage16(l_param, 0xffff, 0x40d, 0x183, iVar4.field6_0x6);
    HVar2 = GetDlgItem16(0x181, iVar4.field6_0x6);
    iVar4.field141_0x8e = HVar2;
    HVar2 = GetDlgItem16(0x184, iVar4.field6_0x6);
    iVar4.field142_0x90 = HVar2;
    return;
}


pub unsafe fn win_ui_op_1040_311a(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut in_EDX: u32;
    let mut uVar4: u16;
    let mut uVar3: u32;
    let mut LVar5: LRESULT;
    let mut paVar6: *mut Struct27;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut iVar7: i16;

    uVar4 = (in_EDX >> 0x10);
    send_msg_1040_323c(CONCAT22(param_2, param_1));
    load_string_1010_847e(_u16_1050_14cc, 0x531);
    if (param_4 == 0x181) {
        uVar3 = CONCAT22(uVar4, param_2);
        iVar1 = param_1 + 0x9a;
        iVar7 = iVar1;
        pass1_1018_3cda((param_1 + 0x96), CONCAT22(param_2, param_1 + 0x19a),
                        CONCAT22(param_2, iVar1));
        pass1_1018_3424(iVar7, uVar3, (param_1 + 0x96));
        if (iVar7 == 0) {
            iVar7 = 0x21;
        } else {
            pass1_1018_3a42(uVar3, (param_1 + 0x96), CONCAT22(param_2, iVar1));
            pass1_1030_8344(_u16_1050_5748, CONCAT22(uVar3, iVar7));
            uVar2 = (iVar7 + 0x10);
            pass1_1030_8344(_u16_1050_5748, uVar2);
            PTR_LOOP_1050_5f0c = uVar2;
            PTR_LOOP_1050_5f0e = uVar3;
            PTR_LOOP_1050_5f10 = (&PTR_LOOP_1050_0000 + 1);
            iVar7 = 0x25;
        }
        pass1_1038_af40(param_1, uVar3, _PTR_LOOP_1050_5b7c, (param_1 + 0x8), iVar7);
        uVar4 = (uVar3 >> 0x10);
        LVar5 = SendMessage16(0x0, 0x2, 0x111, (param_1 + 0x6));
        iVar7 = 0x1;
        paVar6 = mixed_1010_20ba(CONCAT22(uVar4, (LVar5 >> 0x10)), _u16_1050_0ed0,
                                 0x1002b, in_stack_0000fe8e, in_stack_0000ffb2, in_stack_0000ffb8,
                                 in_stack_0000ffbc);
        pass1_1010_038e(paVar6, iVar7);
    } else {
        if ((param_4 == 0x181) || (0x1 < param_4 - 0x182)) {
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4);
            return;
        }
        set_win_pos_1040_331a(CONCAT22(param_2, param_1), param_3, param_4);
    }
    return;
}


pub unsafe fn pass1_1040_3410(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_2f06(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_3532(param_1: *mut u8, mut param_2: u16, mut param_3: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    return;
}


pub unsafe fn show_win_1040_355a(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn set_win_text_1040_3590(mut param_1: u16, param_2: *mut astruct_923)

{
    let mut HVar1: HWND16;
    let mut HVar2: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut iVar5: *mut astruct_923;
    let mut uVar5: u16;
    let mut in_stack_0000f8f8: u16;
    let mut in_stack_0000fa1c: u16;
    let mut in_stack_0000fa22: u16;
    let mut in_stack_0000fa26: u16;
    let mut uVar6: u8;
    let mut in_stack_0000fa50: u16;
    let mut local_59a: u32;
    let mut local_596: u32;
    let mut BStack1426: bool;
    let mut uStack1424: u16;
    let mut local_58e: [u16; 0x41] = [0; 0x41];
    let mut local_50c: [u16; 0x80] = [0; 0x80];
    let mut uStack1036: u32;
    let mut puStack1032: *mut u32;
    let mut local_404: [u8; 0x402] = [0; 0x402];

    puStack1032 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0,
                                  CONCAT22(in_stack_0000fa50, 0x2), in_stack_0000f8f8, in_stack_0000fa1c,
                                  in_stack_0000fa22, in_stack_0000fa26);
    uVar4 = (puStack1032 >> 0x10);
    uStack1036 = (puStack1032 + 0x68);
    uVar5 = (param_2 >> 0x10);
    iVar5 = param_2;
    GetWindowText16(0x80, CONCAT22(0x1050, local_50c), iVar5.field6_0x6);
    uVar6 = SUB21(local_50c, 0x0);
    wsprintf16(local_58e, CONCAT13((local_50c >> 0x8), CONCAT12(uVar6, 0x1050)), uVar6,
               CONCAT22(uStack1036, 0x1050), (uStack1036 >> 0x10));
    BStack1426 = SetWindowText16(CONCAT22(0x1050, local_58e), iVar5.field6_0x6);
    sprintf_op_1018_34b6(BStack1426, uVar4, iVar5.field141_0x8e);
    uStack1424 = uVar4;
    pass1_1018_3d44(iVar5.field141_0x8e, CONCAT22(0x1050, &local_59a), CONCAT22(0x1050, &local_596));
    HVar1 = GetDlgItem16(0x193, iVar5.field6_0x6);
    iVar5.field148_0x98 = HVar1;
    EnableWindow16(0x1, HVar1);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
    wsprintf16(local_50c, 0x50, CONCAT22(local_404, 0x1050), CONCAT22(local_596, 0x1050),
               (local_596 >> 0x10));
    HVar1 = GetDlgItem16(0x195, iVar5.field6_0x6);
    SetWindowText16(CONCAT22(0x1050, local_50c), HVar1);
    HVar2 = GetDlgItem16(0x196, iVar5.field6_0x6);
    HVar1 = HVar2;
    sprintf_op_1018_34b6(HVar2, uVar4, iVar5.field141_0x8e);
    SetWindowText16(CONCAT22(uVar4, HVar2), HVar1);
    HVar1 = GetDlgItem16(0x197, iVar5.field6_0x6);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
    SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
    wsprintf16(local_50c, CONCAT22(local_404, 0x1050), CONCAT22(local_59a, 0x1050),
               (local_59a >> 0x10));
    HVar1 = GetDlgItem16(0x198, iVar5.field6_0x6);
    SetWindowText16(CONCAT22(0x1050, local_50c), HVar1);
    uVar3 = GetDlgItem16(0x199, iVar5.field6_0x6);
    HVar1 = uVar3;
    unk_str_op_1018_35b0(uVar3, iVar5.field141_0x8e);
    if ((uVar4 | uVar3) == 0) {
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
        SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
        GetDlgItem16(0x19a, iVar5.field6_0x6);
        HVar1 = _u16_1050_14cc;
        load_string_1010_84e0(HVar1, (_u16_1050_14cc >> 0x10), 0x3ff, local_404, &DAT_1050_1050);
        SetWindowText16(CONCAT22(0x1050, local_404), HVar1);
        EnableWindow16(0x0, iVar5.field148_0x98);
        return;
    }
    SetWindowText16(CONCAT22(uVar4, uVar3), HVar1);
    return;
}


pub unsafe fn message_box_op_1040_37f0(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut uVar4: u16;
    let mut LVar5: LRESULT;
    let mut in_stack_0000fa94: u16;
    let mut in_stack_0000fbb8: u16;
    let mut in_stack_0000fbbe: u16;
    let mut in_stack_0000fbc2: u16;
    let mut in_stack_0000fbec: u16;
    let mut iVar6: i16;
    let mut local_40c: [u8; 0x402] = [0; 0x402];
    let mut pcStack10: *mut c_char;
    let mut paStack6: *mut Struct27;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 == 0x193) {
        paStack6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fbec, 0x2), in_stack_0000fa94,
                                   in_stack_0000fbb8, in_stack_0000fbbe, in_stack_0000fbc2);
        uVar2 = (paStack6 >> 0x10);
        pcStack10 = *(paStack6 + 0x68);
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_40c, &DAT_1050_1050);
        uVar1 = MessageBox16(0x30, pcStack10, CONCAT22(0x1050, local_40c), (param_2 + 0x6));
        pass1_1018_3710(uVar1, uVar2, (param_2 + 0x8e));
        PostMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
    } else {
        if (param_5 != 0x194) {
            post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
            return;
        }
        pass1_1038_af40(param_2, param_1, _PTR_LOOP_1050_5b7c, (param_2 + 0x8), 0x21);
        uVar4 = (paVar3 >> 0x10);
        LVar5 = SendMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
        iVar6 = 0x1;
        paStack6 = mixed_1010_20ba(CONCAT22(uVar4, (LVar5 >> 0x10)), _u16_1050_0ed0,
                                   0x1002b, in_stack_0000fa94, in_stack_0000fbb8, in_stack_0000fbbe,
                                   in_stack_0000fbc2);
        pass1_1010_038e(paStack6, iVar6);
    }
    return;
}


pub unsafe fn pass1_1040_38d4(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_3506(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_3a0e(param_1: *mut u8, mut param_2: u16, mut param_3: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    return;
}


pub unsafe fn enable_win_1040_3a36(param_1: *mut astruct_924, mut param_2: u16, mut param_3: u16, mut param_4: i16) -> u16

{
    let mut puVar1: *mut u16;
    let mut bVar2: bool;
    let mut iVar3: *mut astruct_924;
    let mut uVar3: u16;

    bVar2 = false;
    iVar3 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_4 == 0) {
//    if (iVar3.field155_0x9e <= iVar3.field154_0x9c) goto LAB_1040_3a79;
        puVar1 = &iVar3.field154_0x9c;
        *puVar1 = *puVar1 + 1;
    } else {
//    if (param_4 != 1) goto LAB_1040_3a79;
//    if (iVar3.field154_0x9c == 0) goto LAB_1040_3a79;
        puVar1 = &iVar3.field154_0x9c;
        *puVar1 = *puVar1 - 0x1;
    }
    bVar2 = true;//
// LAB_1040_3a79:
    if (bVar2) {
        SetDlgItemInt16(0x0, iVar3.field154_0x9c, 0x18e, iVar3.field6_0x6);
    }
    if ((iVar3.field154_0x9c != 0) && (iVar3.field158_0xa2 == 0)) {
        iVar3.field158_0xa2 = 0x1;
        EnableWindow16(0x1, iVar3.field153_0x9a);
    }
    if ((iVar3.field154_0x9c == 0) && (iVar3.field158_0xa2 != 0)) {
        iVar3.field158_0xa2 = 0;
        EnableWindow16(0x0, iVar3.field153_0x9a);
    }
    return 0x0;
}


pub unsafe fn show_win_1040_3ae8(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn win_ui_op_1040_3b1e(mut param_1: u16, struct_c_param_1: *mut StructC)

{
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut HVar3: HWND16;
    let mut pSVar4: *mut StructC;
    let mut in_register_0000000a: u16;
    let mut struct_c_4: *mut StructC;
    let mut unaff_SI: u16;
    let mut struct_c_param_2: *mut StructC;
    let mut uVar5: u32;
    let mut in_stack_0000fd8a: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb4: u16;
    let mut in_stack_0000feb8: u16;
    let mut puStack282: *mut u32;
    let mut local_10e: [u16; 0x41] = [0; 0x41];
    let mut local_8c: [u16; 0x41] = [0; 0x41];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    puStack6 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0,
                               CONCAT22(unaff_SI, 0x2), in_stack_0000fd8a, in_stack_0000feae, in_stack_0000feb4,
                               in_stack_0000feb8);
    uStack10 = (puStack6 + 0x68);
    struct_c_param_2 = (struct_c_param_1 >> 0x10);
    struct_c_4 = struct_c_param_1;
    GetWindowText16(0x80, CONCAT22(0x1050, local_8c), struct_c_4.field6_0x6);
    wsprintf16(local_10e, CONCAT22(local_8c, 0x1050), CONCAT22(uStack10, 0x1050),
               (uStack10 >> 0x10));
    SetWindowText16(CONCAT22(0x1050, local_10e), struct_c_4.field6_0x6);
    puStack282 = (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field_0x96));
    pSVar4 = struct_c_param_2;
    pass1_1018_3d44(struct_c_4.field141_0x8e,
                    (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field142_0x92)),
                    (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field_0x96)));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x80, local_10e, &DAT_1050_1050,
    );
    uVar1 = struct_c_4.field142_0x92;
    wsprintf16(local_8c, CONCAT22(local_10e, 0x1050), CONCAT22(*puStack282, 0x1050),
               (*puStack282 >> 0x10), uVar1, (uVar1 >> 0x10));
    SetDlgItemText16(CONCAT22(0x1050, local_8c), 0x187, struct_c_4.field6_0x6);
    BVar2 = CheckRadioButton16(0x188, 0x18d, 0x188, struct_c_4.field6_0x6);
    struct_c_4.field149_0xa0 = 0x188;
    uVar5 = switch_1018_3b9e(BVar2, pSVar4, struct_c_4.field141_0x8e, struct_c_4.field149_0xa0);
    send_dlg_item_msg_1040_3f12(struct_c_4, struct_c_param_2, uVar5);
    dialog_item_ui_op_1040_3e08(struct_c_param_1);
    HVar3 = GetDlgItem16(0x186, struct_c_4.field6_0x6);
    struct_c_4.field146_0x9a = HVar3;
    return;
}
