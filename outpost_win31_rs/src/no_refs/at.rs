use crate::block_1040::block_1040_3000::{dialog_item_ui_op_1040_3e08, pass1_1040_39e2, send_dlg_item_msg_1040_3f12};
use crate::block_1040::block_1040_4000::{pass1_1040_40e2, pass1_1040_4d7e, pass1_1040_4dcc};

pub unsafe fn unk_win_ui_op_1040_3c64(mut param_1: u16, struct_c_param_1: *mut StructC, struct_c_param_2: *mut StructC, mut param_4: u16, mut param_5: u32)

{
    let mut UVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut uVar3: u32;
    let mut LVar4: LRESULT;
    let mut paVar5: *mut Struct27;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut iVar6: i16;

    if (param_5 == 0x186) {
        LVar4 = SendDlgItemMessage16(0x0, 0x0, 0x409, 0x190, struct_c_param_1.field6_0x6);
        uVar2 = (LVar4 >> 0x10);
        UVar1 = GetDlgItemInt16(0x0, NULL, 0x0, 0x18e);
        pass1_1018_36e6(struct_c_param_1.field141_0x8e, UVar1, LVar4, struct_c_param_1.field149_0xa0);
        pass1_1038_af40(struct_c_param_1, uVar2, _PTR_LOOP_1050_5b7c, &struct_c_param_1.field_0x8, 0x22);
        LVar4 = SendMessage16(0x0, 0x2, 0x111, struct_c_param_1.field6_0x6);
        iVar6 = 0x1;
        paVar5 = mixed_1010_20ba(CONCAT22(in_register_0000000a, (LVar4 >> 0x10)), _u16_1050_0ed0,
                                 0x1002b, in_stack_0000fe9a, in_stack_0000ffbe, in_stack_0000ffc4,
                                 in_stack_0000ffc8);
        pass1_1010_038e(paVar5, iVar6);
    } else {
        if (param_5 - 0x186 < 0x2) {//
// LAB_1040_3c7f:
            post_win_msg_1040_7b3c(CONCAT22(struct_c_param_2, struct_c_param_1), param_4, param_5, param_5);
            return;
        }
        if (param_5 - 0x188 < 0x5 || param_5 == 0x18d) {
            struct_c_param_1.field149_0xa0 = param_5;
            uVar3 = switch_1018_3b9e(param_5, param_1, struct_c_param_1.field141_0x8e, param_5);
            send_dlg_item_msg_1040_3f12(struct_c_param_1, struct_c_param_2, uVar3);
        } else {
//      if (param_5 - 0x188 != 0x8) goto LAB_1040_3c7f;
            if (param_5 != 1) {
                return;
            }
        }
        dialog_item_ui_op_1040_3e08(CONCAT22(struct_c_param_2, struct_c_param_1));
    }
    return;
}


pub unsafe fn get_dc_op_1040_3d5e(param_1: *mut astruct_1) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut iVar4: *mut astruct_1;
    let mut iVar3: *mut astruct_934;
    let mut uVar3: u16;
    let mut puStack8: *mut u32;
    let mut local_4: HDC16;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    local_4 = GetDC16(iVar4.field6_0x6);
    uVar2 = FUN_1010_830a(local_4, in_EDX, s_tile2_bmp_1050_1538, _u16_1050_14cc, iVar4.field163_0xa4);
    puStack8 = CONCAT22(in_EDX, uVar2);
    iVar3 = *puStack8;
    ppcVar1 = &iVar3.field6_0x8;
    (**ppcVar1)(0x1010, uVar2, in_EDX, &local_4, &DAT_1050_1050);
    ppcVar1 = &iVar3.field4_0x4;
    (**ppcVar1)(0x1010, puStack8, 0x50078, &local_4, &DAT_1050_1050);
    ppcVar1 = &iVar3.field8_0xc;
    (**ppcVar1)(0x1010, puStack8, &local_4, &DAT_1050_1050);
    ReleaseDC16(local_4, iVar4.field6_0x6);
    return 0x0;
}


pub unsafe fn pass1_1040_3fd6(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_39e2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn win_ui_op_1040_410e(param_1: u8, struct_b_param_1: *mut StructB)

{
    let mut uVar1: u32;
    let mut in_EDX: u32;
    let mut paVar2: *mut Struct57;
    let mut struct_b_3: *mut StructB;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe52: u16;
    let mut in_stack_0000ff76: u16;
    let mut in_stack_0000ff7c: u16;
    let mut in_stack_0000ff80: u16;
    let mut piVar6: *mut i16;
    let mut uVar7: u16;
    let mut pcVar8: *mut c_char;
    let mut local_36: i16;
    let mut local_34: i16;
    let mut local_32: i16;
    let mut local_30: [u8; 0x6] = [0; 0x6];
    let mut local_2a: [i16; 0x4] = [0; 0x4];
    let mut uStack34: u32;
    let mut local_1e: u32;
    let mut uStack26: u32;
    let mut local_16: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut HStack14: HWND16;
    let mut local_c: [u8; 0xa] = [0; 0xa];

    uVar7 = (in_EDX >> 0x10);
    dialog_ui_fn_1040_78e2(struct_b_param_1);
    pass1_1000_4906(CONCAT22(0x1050, local_c), NULL, 0xa);
    uVar3 = (struct_b_param_1 >> 0x10);
    struct_b_3 = struct_b_param_1;
    uVar1 = &struct_b_3[0x7].field1_0x2;
    sys_1000_3f9c(CONCAT22(0x1050, local_c), s__lu_1050_5d38, (uVar1 + 0x76));
    HStack14 = GetDlgItem16(0xfb5, struct_b_3.lpvoid_field_0x8);
    SendMessage16(CONCAT22(0x1050, local_c), 0x0, 0xc, HStack14);
    SetFocus16(HStack14);
    SendMessage16(-0x10000, 0x0, 0x401, HStack14);
    GetWindowRect16(CONCAT22(0x1050, &local_16), struct_b_3.lpvoid_field_0x8);
    pass1_1000_4906(CONCAT22(0x1050, &local_1e), NULL, 0x8);
    uVar1 = &struct_b_3[0x7].field1_0x2;
    uStack34 = pass1_1010_5f7a(uVar1, (uVar1 >> 0x10), 0x0, 0x7);
    if (uStack34.is_null() == false) {
        local_1e = *uStack34;
        uStack26 = (uStack34 + 0x4);
    }
    if ((local_1e == 0) && (local_1e == 0)) {
        puVar4 = pass1_1008_3e38(CONCAT22(0x1050, local_30));
        paVar2 = CONCAT22(uVar7, (puVar4 >> 0x10));
        uVar1 = &struct_b_3[0x7].field5_0xa;
        pass1_1018_2678(uVar1, (uVar1 >> 0x10), CONCAT22(0x1050, local_30));
        pass1_1008_3e94(CONCAT22(0x1050, local_30), CONCAT22(0x1050, &local_32), CONCAT22(0x1050, local_2a),
        );
        pcVar8 = CONCAT22(0x1050, &local_34);
        piVar6 = &local_36;
        uVar7 = SUB42(&DAT_1050_1050, 0x0);
        puVar5 = mixed_1010_20ba(paVar2, _u16_1050_0ed0, CONCAT22(piVar6, 0x48), in_stack_0000fe52,
                                 in_stack_0000ff76, in_stack_0000ff7c, in_stack_0000ff80);
        pass1_1008_3e94((puVar5 & 0xffff0000 | (puVar5 + 0xe)), CONCAT22(uVar7, piVar6),
                        pcVar8);
        uStack26 = CONCAT22(iStack16 - iStack20, iStack18 - local_16);
        local_1e = CONCAT22((((puVar5 + 0xc) * -0x14) / 0x258 - (iStack16 - iStack20)) + local_36 + local_32,
                            local_34 + local_2a[0]);
    }
    move_win_1040_826c(struct_b_param_1, local_1e, local_1e);
    ShowWindow16(0x5, struct_b_3.lpvoid_field_0x8);
    return;
}


pub unsafe fn win_ui_op_1040_42b2(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut iVar5: *mut astruct_893;
    let mut uVar5: u16;
    let mut LVar6: LRESULT;
    let mut local_54: [u16; 0x29] = [0; 0x29];

    iVar5 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        iVar5.field147_0x9a = 0x1;
        DestroyWindow16(iVar5.field6_0x6);
        return;
    }
    pass1_1000_4906(CONCAT22(0x1050, local_54), NULL, 0x51);
    HVar2 = GetDlgItem16(0xfb5, iVar5.field6_0x6);
    LVar6 = SendMessage16(CONCAT22(0x1050, local_54), 0x51, 0xd, HVar2);
    uVar4 = (LVar6 >> 0x10);
    uVar3 = pass1_1000_3e2c(CONCAT22(0x1050, local_54));
    if ((uVar4 | uVar3) != 0) {
        iVar5.field142_0x92 = uVar3;
        (&iVar5.field142_0x92 + 0x2) = uVar4;
    }
    if (uVar4 < 0x0) {
        uVar1 = iVar5.field141_0x8e;
        uVar1 = (uVar1 + 0x76);
        wsprintf16(local_54, 0x5d3c1050, CONCAT22(uVar1, 0x1050), (uVar1 >> 0x10));
        SendMessage16(CONCAT22(0x1050, local_54), 0x0, 0xc, HVar2);
        SetFocus16(HVar2);
        SendMessage16(-0x10000, 0x0, 0x401, HVar2);
        return;
    }
    HVar2 = GetDlgItem16(0x1, iVar5.field6_0x6);
    EnableWindow16(0x0, HVar2);
    uVar1 = iVar5.field141_0x8e;
    (uVar1 + 0x76) = iVar5.field142_0x92;
    PostMessage16(iVar5.field142_0x92, 0x0, 0x400, HWND16_1050_0396);
    HVar2 = GetDlgItem16(0x1, iVar5.field6_0x6);
    EnableWindow16(0x1, HVar2);
    return;
}


pub unsafe fn get_win_rect_1040_43ea(mut param_1: i16, mut param_2: u16)

{
    let mut uVar1: u32;
    let mut local_a: u32;
    let mut iStack6: i16;
    let mut iStack4: i16;

    GetWindowRect16(CONCAT22(0x1050, &local_a), (param_1 + 0x6));
    iStack6 -= local_a;
    iStack4 -= local_a;
    pass1_1010_5fb0((param_1 + 0x8e), 0x0, &local_a, &DAT_1050_1050, 0x7);
    uVar1 = (param_1 + 0x8e);
    (uVar1 + 0x7a) = ((param_1 + 0x9a) == 0);
    return;
}


pub unsafe fn pass1_1040_4440(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_40e2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_45e8(param_1: *mut u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut pSVar1: *mut StructD;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut pSVar5: *mut StructD;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut paVar9: *mut Struct57;
    let mut iVar10: i16;
    let mut unaff_SI: u16;
    let mut uVar11: u16;
    let mut paVar12: *mut astruct_20;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut piStack16: *mut i16;
    let mut paVar8: *mut Struct57;

    paVar7 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 != 0xeb) {
        pass1_1040_b54a(param_1, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), param_4,
                        param_5);
        return;
    }
    paVar12 = mixed_1010_20ba(paVar7, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x3), in_stack_0000fe88,
                              in_stack_0000ffac, in_stack_0000ffb2, in_stack_0000ffb6);
    paVar7 = (paVar7 & 0xffff0000 | paVar12 >> 0x10);
    pSVar1 = (param_2 + 0x90);
    if (pSVar1.is_null() == false) {
        pSVar5 = pSVar1;
        mem_op_1000_179c(0x18, paVar7);
        uVar4 = pSVar5;
        uVar6 = paVar7 | uVar4;
        paVar9 = (paVar7 & 0xffff0000);
        paVar8 = (paVar9 | uVar6);
        if (uVar6 == 0) {
            uVar4 = 0;
        } else {
            struct_1040_a598((pSVar5 & 0xffff | paVar7 << 0x10));
            paVar9 = paVar8;
        }
        (param_2 + 0x90) = uVar4;
        (param_2 + 0x92) = paVar9;
        (param_2 + 0x90) = 0x14;
        iVar10 = *(param_2 + 0x90);
        uVar4 = iVar10 * 0xa + 2;
        mem_op_1000_179c(uVar4, paVar9);
        uVar6 = paVar9;
        piStack16 = CONCAT22(uVar6, uVar4);
        if ((uVar6 | uVar4) == 0) {
            uVar3 = (param_2 + 0x90);
            (uVar3 + 0x2) = 0;
        } else {
            *piStack16 = iVar10;
            pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar10, 0xa, uVar4 + 0x2, uVar6);
            uVar3 = (param_2 + 0x90);
            uVar11 = (uVar3 >> 0x10);
            iVar10 = uVar3;
            (iVar10 + 0x2) = uVar4 + 2;
            (iVar10 + 0x4) = uVar6;
        }
        uVar3 = (param_2 + 0x90);
        (uVar3 + 0x6) = (pSVar1 + 0x6);
        uVar3 = (param_2 + 0x90);
        (uVar3 + 0xa) = 0x1;
        uVar3 = (param_2 + 0x90);
        (uVar3 + 0x12) = (param_2 + 0xa);
        uVar11 = 0x1010;
        pass1_1010_a50c(paVar12, 0x10505d40, (param_2 + 0x90));
        if (pSVar1.is_null() == false) {
            pass1_1040_a5d0(pSVar1);
            uVar11 = 0x1000;
            fn_ptr_1000_17ce(pSVar1);
        }
        ppcVar2 = (CONCAT22(param_3, param_2) + 0x70);
        (**ppcVar2)(uVar11, param_2, param_3);
    }
    return;
}


pub unsafe fn pass1_1040_477e(mut param_1: u16, param_2: *mut StructB)

{
    let mut puVar1: *mut u8;
    let mut pUVar2: *mut u16;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_stack_0000ffee: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    unk_win_ui_op_1040_b230(param_1, param_2);
    puVar6 = mixed_1010_20ba(paVar5, _u16_1050_0ed0, CONCAT22(in_stack_0000ffee, 0x3), in_stack_0000fe96,
                             in_stack_0000ffba, in_stack_0000ffc0, in_stack_0000ffc4);
    puVar3 = (puVar6 >> 0x10);
    uVar8 = SUB42(&DAT_1050_1050, 0x0);
    uVar7 = 0x5d68;
    puVar1 = pass1_1008_5fd8(puVar3);
    puVar4 = puVar3;
    pUVar2 = pass1_1000_3cea(CONCAT22(puVar3, puVar1), CONCAT22(uVar8, uVar7));
    pass1_1010_e964(puVar4);
    pass1_1000_3cea(CONCAT22(puVar3, puVar1), CONCAT22(puVar4, pUVar2));
    unk_str_op_1000_3d3e((param_2 & 0xffff0000 | (param_2 + 0x10)), CONCAT22(puVar3, puVar1));
    fn_ptr_1000_17ce(CONCAT22(puVar3, puVar1));
    return;
}


pub unsafe fn pass1_1040_47fe(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn send_win_msg_1040_4a0a(struct_param_1: *mut astruct_48) -> LRESULT

{
    WPARAM16 * pWVar1;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut dlg_item: HWND16;
    let mut uVar5: u16;
    let mut iVar5: *mut astruct_48;
    let mut uVar6: *mut astruct_48;
    let mut lresult_6: LRESULT;
    let mut pcVar6: *mut c_char;
    let mut wparam: WPARAM16;
    let mut UVar7: u16;
    let mut HVar8: HWND16;
    let mut WStack10: WPARAM16;

    uVar6 = (struct_param_1 >> 0x10);
    iVar5 = struct_param_1;
    ppcVar2 = (struct_param_1 + 0x74);
    (**ppcVar2)();
    dlg_item = GetDlgItem16(0x1770, iVar5.hwnd_0x6);
    SendMessage16(0x0, 0x0, 0x40b, dlg_item);
    lresult_6 = SendMessage16(0x0, 0x0, 0xb, dlg_item);
    uVar5 = (lresult_6 >> 0x10);
//   for (WStack10 = 0; uVar3 = iVar5.field143_0x90, pWVar1 = (uVar3 + 0x10),
//       *pWVar1 != WStack10 && WStack10 <= *pWVar1; WStack10 += 1)
    WStack10 = 0;
    uVar3 = iVar5.field143_0x90;
    pWVar1 = uVar3 + 0x10;
    while *pWVar1 != WStack10 && WStack10 <= *pWVar1

    {
        wparam = 0;
        UVar7 = 0x403;
        uVar3 = iVar5.field143_0x90;
        uVar3 = (uVar3 + 0xc);
        HVar8 = dlg_item;
        pcVar6 = pass1_1040_4dcc(uVar5, struct_param_1, (uVar3 + WStack10 * 0x2));
        lresult_6 = SendMessage16(pcVar6, wparam, UVar7, HVar8);
        uVar5 = (lresult_6 >> 0x10);
    }
    pass1_1040_4d7e(struct_param_1);
    if (WStack10 == 0) {
        UVar7 = 0x40a;
        uVar4 = iVar5.field143_0x90;
        uVar3 = iVar5.field144_0x94;
        HVar8 = dlg_item;
        pcVar6 = string_op_1010_ada6(uVar5, uVar3, (uVar3 >> 0x10), 0x0, (uVar4 + 0xa));
        SendMessage16(pcVar6, WStack10, UVar7, HVar8);
    }
    lresult_6 = SendMessage16(0x0, 0x1, 0xb, dlg_item);
    return lresult_6;
}


pub unsafe fn set_win_pos_1040_4ae4(mut param_1: i16, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut pSVar4: *mut StructD;
    let mut uVar5: u16;
    let mut in_EDX: *mut Struct57;
    let mut paVar6: *mut Struct57;
    let mut paVar8: *mut Struct57;
    let mut iVar9: i16;
    let mut unaff_SI: u16;
    let mut uVar10: u16;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut local_24: [i16; 0x2] = [0; 0x2];
    let mut iStack32: i16;
    let mut pSStack20: *mut StructD;
    let mut pSStack16: *mut StructD;
    let mut iStack12: i16;
    let mut pSStack10: *mut StructD;
    let mut paStack6: *mut astruct_20;
    let mut paVar7: *mut Struct57;

    if (param_4 == 0xeb) {
        paStack6 = mixed_1010_20ba(in_EDX, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x3), in_stack_0000fe80,
                                   in_stack_0000ffa4, in_stack_0000ffaa, in_stack_0000ffae);
        paVar6 = (in_EDX & 0xffff0000 | paStack6 >> 0x10);
        pSVar4 = (param_1 + 0x90);
        if (pSVar4.is_null() == false) {
            pSStack10 = pSVar4;
            mem_op_1000_179c(0x18, paVar6);
            uVar3 = pSVar4;
            pSStack16 = (pSVar4 & 0xffff | paVar6 << 0x10);
            uVar5 = paVar6 | uVar3;
            paVar8 = (paVar6 & 0xffff0000);
            paVar7 = (paVar8 | uVar5);
            if (uVar5 == 0) {
                uVar3 = 0;
            } else {
                struct_1040_a598((pSVar4 & 0xffff | paVar6 << 0x10));
                paVar8 = paVar7;
            }
            (param_1 + 0x90) = uVar3;
            (param_1 + 0x92) = paVar8;
            (param_1 + 0x90) = 0x7;
            iStack12 = *(param_1 + 0x90);
            uVar3 = iStack12 * 0xa + 2;
            mem_op_1000_179c(uVar3, paVar8);
            uVar5 = paVar8;
            pSStack16 = CONCAT22(uVar5, uVar3);
            if ((uVar5 | uVar3) == 0) {
                uVar2 = (param_1 + 0x90);
                (uVar2 + 0x2) = 0;
            } else {
                pSStack16 = iStack12;
                pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iStack12, 0xa, uVar3 + 0x2, uVar5);
                uVar2 = (param_1 + 0x90);
                uVar10 = (uVar2 >> 0x10);
                iVar9 = uVar2;
                (iVar9 + 0x2) = uVar3 + 2;
                (iVar9 + 0x4) = uVar5;
            }
            uVar10 = (pSStack10 >> 0x10);
            iVar9 = pSStack10;
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x6) = (iVar9 + 0x6);
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0xa) = (iVar9 + 0xa);
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x12) = (iVar9 + 0x12);
            uVar10 = 0x1010;
            pass1_1010_a50c(paStack6, 0x10505d6a, (param_1 + 0x90));
            pSStack20 = pSStack10;
            pSStack16 = pSStack10;
            if (pSStack10.is_null() == false) {
                pass1_1040_a5d0(pSStack10);
                uVar10 = 0x1000;
                fn_ptr_1000_17ce(pSStack10);
            }
            ppcVar1 = (CONCAT22(param_2, param_1) + 0x70);
            (**ppcVar1)(uVar10, param_1, param_2);
        }
    } else {
        if (param_4 != 0x1770) {
            pass1_1040_b54a(in_EDX, CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)),
                            param_3, param_4);
            return;
        }
        if (param_4 == 0x7) {
            GetWindowRect16(CONCAT22(0x1050, local_24), param_3);
            iStack32 -= local_24[0];
            SetWindowPos16(0x2, 0x50, iStack32, 0x0, 0x0, 0x0, param_3);
        }
    }
    return;
}


pub unsafe fn send_msg_1040_4cb2(mut param_1: u16, mut param_2: u32) -> LRESULT

{
    let mut uVar1: u8;
    let mut HVar1: HWND16;
    let mut uVar2: u32;
    let mut LVar2: LRESULT;
    let mut hwnd: HWND16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    pass1_1040_b45e(param_2);
    HVar1 = GetDlgItem16(0x1770, (param_2 + 0x6));
    uVar3 = 0xffff;
    uVar4 = 0x40d;
    hwnd = HVar1;
    pass1_1040_4d7e(param_2);
    uVar2 = pass1_1040_4dcc(param_1, param_2, HVar1);
    LVar2 = SendMessage16(uVar2, uVar3, uVar4, hwnd);
    return LVar2;
}


pub unsafe fn pass1_1040_4cf4(mut param_1: u32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut hwnd: HWND16;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut LVar9: LRESULT;
    let mut uVar10: u32;
    let mut local_52: [u8; 0x50] = [0; 0x50];

    uVar8 = (in_EDX >> 0x10);
    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    hwnd = GetDlgItem16(0x1770, (iVar5 + 0x6));
    LVar9 = SendMessage16(0x0, 0x0, 0x407, hwnd);
    uVar4 = CONCAT22(uVar8, (LVar9 >> 0x10));
    if (LVar9 != 0xffff) {
        uVar10 = SendMessage16(CONCAT22(0x1050, local_52), LVar9, 0x408, hwnd);
        uVar4 = uVar4 & 0xffff0000 | uVar10 >> 0x10;
    }
    uVar2 = (iVar5 + 0x90);
    uVar1 = (iVar5 + 0x94);
    uVar3 = pass1_1010_ae12(uVar4, uVar1, (uVar1 >> 0x10), CONCAT22(0x1050, local_52),
                            (uVar2 + 0xa));
    if (uVar3 != 0xffff) {
        uVar1 = (iVar5 + 0x90);
        uVar8 = (uVar1 >> 0x10);
        iVar6 = uVar1;
        uVar10 = (iVar6 + 0x6);
        pass1_1010_ae92((iVar5 + 0x94), uVar10, (iVar6 + 0xa), uVar10, uVar4);
    }
    return;
}


pub unsafe fn pass1_1040_4df2(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_4f28(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16, mut param_6: u16) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2));
        }
    } else {
        if (param_5 != 0x111) {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return uVar2;
        }
        ppcVar1 = (*param_1 + 0x7c);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}


pub unsafe fn pass1_1040_4f82(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}


pub unsafe fn set_win_pos_1040_4f96(param_1: *mut StructD, struct_b_param_1: *mut StructB, mut param_3: u16, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut paVar12: *mut Struct57;
    let mut paVar14: *mut Struct57;
    let mut struct_b_11: *mut StructB;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut puVar17: *mut u32;
    let mut puVar18: *mut u16;
    let mut in_stack_0000fe34: u16;
    let mut in_stack_0000fe36: u16;
    let mut in_stack_0000fe38: u16;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff60: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff66: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut uVar19: u8;
    let mut uVar20: u8;
    let mut BVar21: bool;
    let mut puVar22: *mut u8;
    let mut paVar13: *mut Struct57;
    let mut fn_ptr_1: *mut *mut code;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar17 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_5, 0x41), in_stack_0000fe8c,
                              in_stack_0000ffb0, in_stack_0000ffb6, in_stack_0000ffba);
    paVar12 = (param_1 & 0xffff0000 | puVar17 >> 0x10);
    paVar5 = puVar17;
    uVar15 = (struct_b_param_1 >> 0x10);
    struct_b_11 = struct_b_param_1;
    struct_b_11[0x7].field6_0xc = paVar5;
    uVar9 = (puVar17 >> 0x10);
    struct_b_11[0x7].field7_0xe = uVar9;
    puVar22 = struct_b_11[0x7].field6_0xc;
    fn_ptr_1 = (*&struct_b_11[0x7].field6_0xc + 0x10);
    (**fn_ptr_1)(0x1010, puVar22, uVar9);
    mem_op_1000_179c(0xa, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 == 0) {
        struct_b_11[0x7].max_count_field_0x10 = 0;
    } else {
        puVar18 = struct_1040_bf3e(CONCAT13((paVar12 >> 0x8), CONCAT12(paVar12, paVar5)),
                                   struct_b_11.lpvoid_field_0x8);
        paVar13 = (paVar13 & 0xffff0000 | puVar18 >> 0x10);
        paVar5 = puVar18;
        struct_b_11[0x7].max_count_field_0x10 = paVar5;
        struct_b_11[0x7].field5_0xa = (puVar18 >> 0x10);
    }
    pass1_1040_bfde(&struct_b_11[0x7].max_count_field_0x10, &struct_b_11[0x7].field6_0xc);
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar5, paVar13, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10a), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar13, paVar5, paVar12, 0x1, 0xa0028, 0x0, 0x840085,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10b), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar5, paVar13, 0x1, 0xa0046, 0x0, 0x860087,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10d), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar13, paVar5, paVar12, 0x1, 0xa0064, 0x0, 0x880089,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10e), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar5, paVar13, 0x1, 0xa0082, 0x0, 0x820083,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10c), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar13, paVar5, paVar12, 0x1, 0xa00d2, 0x0, 0x8a008b,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0xbbb), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    BVar21 = 0;
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000);
    paVar14 = (paVar12 | uVar10);
    if (uVar10 == 0) {
        paVar5 = null_mut();
    } else {
        pvVar1 = struct_b_11.lpvoid_field_0x8;
        pass1_1008_3bd6(paVar14, paVar5, paVar13, 0x1, 0xa00a0, 0x8e, 0x8c008d,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0xbbc)), param_3, in_stack_0000fe34,
                        in_stack_0000fe38, in_stack_0000ff5e, in_stack_0000ff62, in_stack_0000ff66);
        paVar12 = paVar14;
    }
    uVar19 = SUB41(paVar12, 0x0);
    uVar20 = (paVar12 >> 0x8);
    enable_win_1040_9234(CONCAT13(uVar20, CONCAT12(uVar19, paVar5)), BVar21);
    puVar17 = mixed_1010_20ba(paVar12, _u16_1050_0ed0, CONCAT22(puVar22, 0x3), in_stack_0000fe88,
                              in_stack_0000ffac, in_stack_0000ffb2, in_stack_0000ffb6);
    uVar4 = (puVar17 >> 0x10);
    uVar3 = puVar17;
    uVar11 = uVar4;
    uVar6 = pass1_1010_a5ac(uVar3, uVar4, struct_b_11[0x8].field8_0x10);
    uVar7 = pass1_1010_ac62(uVar6, uVar11, uVar3, uVar4, 0x1e);
    if (uVar7 != 0) {
        pass1_1010_a5ca(uVar7, uVar11, uVar3, uVar4, uVar6);
        if (0x0 < uVar7) {
            pass1_1010_a58a(uVar7, uVar11, uVar3, uVar4, uVar6);
            if (uVar7 == 0) {
                enable_win_1040_9234(CONCAT13(uVar20, CONCAT12(uVar19, paVar5)), 1);
            }
        }
    }
    uVar2 = &struct_b_11[0x7].field6_0xc;
    iVar8 = uVar2;
    uVar2 &= 0xffff0000;
    uVar16 = (uVar2 >> 0x10);
    SetWindowPos16(0x40, (iVar8 + 0x10), (iVar8 + 0xe), (iVar8 + 0xc),
                   (uVar2 | iVar8 + 0xa), 0x0, struct_b_11.lpvoid_field_0x8);
    return;
}

pub unsafe fn pass1_1040_5238(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}


pub unsafe fn destroy_win_1040_5256(param_1: *mut astruct_34)

{
    let mut pUVar1: *mut u32;
    let mut bool3: bool;
    let mut pstruct34_5: *mut astruct_34;
    let mut pstruct34_hi: *mut astruct_34;
    let mut unaff_CS: u16;
    let mut uVar2: u16;
    let mut fn_ptr_1: *mut *mut code;

    pstruct34_hi = (param_1 >> 0x10);
    pstruct34_5 = param_1;
    if (pstruct34_5.hwnd_0xb6 != 0) {
        // 0x1538
        unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
        bool3 = IsWindow16(pstruct34_5.hwnd_0xb6);
        if (bool3 != 0) {
            // 0x1538
            unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
            DestroyWindow16(pstruct34_5.hwnd_0xb6);
        }
    }
    pstruct34_5.hwnd_0xb6 = 0;
    pUVar1 = pstruct34_5.field148_0x94;
    uVar2 = pstruct34_5.field149_0x96;
    if ((uVar2 | pUVar1) != 0) {
        fn_ptr_1 = *pUVar1;
        (**fn_ptr_1)(unaff_CS, pUVar1, uVar2, 1);
    }
    pstruct34_5.field148_0x94 = 0;
    pstruct34_5.field150_0x98 = 0;
    return;
}
