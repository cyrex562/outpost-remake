pub unsafe fn pass1_1008_9004(
    param_1: *mut astruct_78,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) {
    let mut puVar3: *mut u32;
    let mut puVar4: *mut u16;
    let mut pstruct78_var4: *mut astruct_78;
    let mut pstruct108_5_1: *mut astruct_108;
    let mut uVar4: *mut astruct_78;
    let mut uVar3: u16;
    let mut bVar5: bool;
    let mut puVar2: *mut u16;
    let mut puVar1: *mut u32;
    let mut pstruct108_5: *mut astruct_108;

    uVar4 = (param_1 >> 0x10);
    pstruct78_var4 = param_1;
    puVar1 = &pstruct78_var4.field4_0xa;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct78_var4.field3_0x6.is_null())) {
        puVar2 = (&pstruct78_var4.field6_0x12 + 2);
        if ((*puVar2 < param_4 || *puVar2 == param_4)
            && (*puVar2 < param_4
                || (
                    puVar3 = &pstruct78_var4.field6_0x12,
                    puVar3 < param_4 || puVar3 == param_4,
                )))
        {
            pass1_1008_909c((param_1 & 0xffff | ZEXT24(uVar4) << 0x10));
        }
        puVar3 = &pstruct78_var4.field6_0x12;
        if ((*puVar3 < param_4 || *puVar3 == param_4) || (pstruct78_var4.field3_0x6.is_null())) {
            return;
        }
        puVar4 = (&pstruct78_var4.field4_0xa + 2);
        bVar5 = *puVar4 < param_4;
        if ((bVar5 || *puVar4 == param_4)
            && (bVar5
                || (
                    puVar3 = &pstruct78_var4.field4_0xa,
                    puVar3 < param_4 || puVar3 == param_4,
                )))
        {
            pstruct78_var4.field4_0xa = (param_4 + 1);
            (pstruct78_var4.field4_0xa + 0x2) = (param_4 + 0x1 >> 0x10);
        }
    }
    pstruct108_5 = pstruct78_var4.field3_0x6;
    uVar3 = (pstruct108_5 >> 0x10);
    pstruct108_5_1 = pstruct108_5;
    (pstruct108_5_1 + param_4 * 0x4) = param_2;
    (pstruct108_5_1 + param_4 * 0x4 + 0x2) = param_3;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_909c(param_1: *mut astruct_78) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_108;
    let mut uVar6: u16;
    let mut uVar5: u16;
    let mut in_EDX: u32;
    let mut pSVar7: *mut StructD;
    let mut iVar5: *mut astruct_78;
    let mut uVar4: *mut astruct_78;
    let mut lVar8: i32;
    let mut paStack10: *mut astruct_108;
    let mut uStack6: u32;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (iVar5.field6_0x12 == 0) {
        uVar6 = &iVar5.field5_0xe;
        pSVar7 = (in_EDX & 0xffff0000 | (&iVar5.field5_0xe + 0x2));
    } else {
        uVar2 = &iVar5.field6_0x12;
        puVar1 = &iVar5.field7_0x16;
        uVar6 = uVar2 + puVar1;
        pSVar7 = (in_EDX & 0xffff0000
            | ((&iVar5.field6_0x12 + 0x2) + (&iVar5.field7_0x16 + 0x2) + CARRY2(uVar2, puVar1)));
    }
    uStack6 = CONCAT22(pSVar7, uVar6);
    if (iVar5.field3_0x6.is_null()) {
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
            PTR_LOOP_1050_5f2e = pSVar7;
        } else {
        }
        uVar5 = fn_ptr_op_1000_1708(
            uVar6 << 0x2,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            PTR_LOOP_1050_5f2e,
        );
    } else {
        paVar3 = iVar5.field3_0x6;
        lVar8 = pass1_1000_0ed4(
            0x1,
            uVar6 * 0x4,
            (pSVar7 * 0x2 + CARRY2(uVar6, uVar6)) * 0x2 + CARRY2(uVar6 * 0x2, uVar6 * 0x2),
            paVar3,
            (paVar3 >> 0x10),
        );
        PTR_LOOP_1050_5f2e = (lVar8 >> 0x10);
        uVar5 = lVar8;
    }
    paStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar5);
    if ((PTR_LOOP_1050_5f2e | uVar5) != 0) {
        iVar5.field6_0x12 = uStack6;
        iVar5.field3_0x6 = paStack10;
    }
    return;
}

pub fn pass1_1008_914a(param_1: *mut astruct_463, param_2: u8) -> *mut StructD {
    pass1_1008_8f24(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub fn struct_op_1008_9174(param_1: *mut astruct_57, mut param_2: u32, mut param_3: u32) {
    let mut iVar1: *mut astruct_88;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar1.field2_0x2 = 0x1008;
    iVar1.field3_0x4 = param_3;
    iVar1.field4_0x8 = param_2;
    iVar1.field5_0xc = param_2;
    iVar1.field6_0x10 = 0;
    param_1.field0_0x0 = 0x9412;
    iVar1.field2_0x2 = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_91ba(param_1: *mut astruct_3) -> *mut u16 {
    let mut UVar1: u16;
    let mut iVar2: *mut astruct_3;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar2.field1_0x2 = 0x1008;
    iVar2.field2_0x4 = 0;
    set_struct_1008_574a((param_1 & 0xffff0000 | ZEXT24(iVar2 + 1)));
    param_1.field0_0x0 = 0x9416;
    iVar2.field1_0x2 = 0x1008;
    _PTR_LOOP_1050_0388 = param_1;
    UVar1 = SetTimer16(NULL, 0x1, 0x1, HWND16_1050_0396);
    if (UVar1 == 0) {
        fn_ptr_op_1000_24cd(1);
    }
    PTR_LOOP_1050_038a = (_PTR_LOOP_1050_0388 >> 0x10);
    return &param_1.field0_0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn kill_timer_1008_921c(param_1: *mut u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x9416;
    (iVar1 + 0x2) = 0x1008;
    KillTimer16(0x1, HWND16_1050_0396);
    _PTR_LOOP_1050_0388 = 0;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0x6)));
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}
pub unsafe fn pass1_1008_9262(
    param_1: *mut astruct_57,
    param_2: *mut astruct_57,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    mem_op_1000_179c(0x12, param_2);
    uVar2 = param_2 | param_1;
    if (uVar2 == 0) {
        param_1 = NULL;
        uVar2 = 0;
    } else {
        struct_op_1008_9174(CONCAT22(param_2, param_1), param_5, param_6);
    }
    if ((uVar2 | param_1) != 0) {
        ppcVar1 = ((param_3 + 0x6) + 0x4);
        (**ppcVar1)();
    }
    return;
}
pub unsafe fn pass1_1008_92b2(mut param_1: u32, param_2: i32, param_3: i32) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut extraout_DX: u16;
    let mut local_c: [u8; 0x4] = [0; 0x4];
    let mut uStack8: u32;
    let mut uStack4: u16;

    uStack4 = 0;
    pass1_1008_57a4(
        CONCAT22(0x1050, local_c),
        param_1 & 0xffff0000 | (param_1 + 0x6),
    );
    loop {
        puVar2 = local_c;
        pass1_1008_5b12(CONCAT22(0x1050, puVar2));
        if ((extraout_DX | puVar2) == 0) {
            break;
        }
        if (((puVar2 + 0x4) == param_3) && ((puVar2 + 0x8) == param_2)) {
            uStack4 = 0x1;
            ppcVar1 = ((param_1 + 0x6) + 0xc);
            (**ppcVar1)();
            uStack8 = 0;
        }
    }
    return;
}
pub unsafe fn pass1_1008_932a(mut param_1: u32) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0x4) == 0) {
        (iVar5 + 0x4) = 0x1;
        pass1_1008_57a4(
            CONCAT22(0x1050, local_a),
            param_1 & 0xffff0000 | (iVar5 + 0x6),
        );
        loop {
            puVar3 = local_a;
            pass1_1008_5b12(CONCAT22(0x1050, puVar3));
            if ((extraout_DX | puVar3) == 0) {
                break;
            }
            uVar1 = (puVar3 + 0xc);
            iVar4 = (puVar3 + 0xe) - (uVar1 < 0x37);
            (puVar3 + 0xc) = uVar1 - 0x37;
            (puVar3 + 0xe) = iVar4;
            if ((iVar4 < 1) && ((iVar4 < 0x0 || ((puVar3 + 0xc) == 0)) && ((puVar3 + 0x10) == 0))) {
                ppcVar2 = (*(puVar3 + 0x4) + 0x4);
                (**ppcVar2)();
                (puVar3 + 0xc) = (puVar3 + 0x8);
            }
        }
        (iVar5 + 0x4) = 0;
    }
    return;
}

pub unsafe fn pass1_1008_93c0(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_93ec(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    kill_timer_1008_921c(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_941a(param_1: *mut u16, mut param_2: u16, mut param_3: u16) -> *mut u16 {
    *param_1 = param_2;
    (param_1 + 0x2) = param_3;
    return param_1;
}

pub unsafe fn pass1_1008_9436(param_1: *mut u16) -> *mut u16 {
    *param_1 = 0;
    (param_1 + 0x2) = 0;
    return param_1;
}
pub unsafe fn pass1_1008_944e(param_1: *mut u16, mut param_2: u16, mut param_3: u16) {
    (param_1 + 0x2) = param_3;
    *param_1 = param_2;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_9466(param_1: *mut u16) {
    *param_1 = 0x52a;
    (param_1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce(_PTR_LOOP_1050_0392);
    _PTR_LOOP_1050_0392 = NULL;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn win_msg_op_1008_9498() -> WPARAM16 {
    let mut BVar1: bool;
    let mut IVar2: i16;
    let mut local_msg_1: MSG16; //
                                //
                                // LAB_1008_949c:
    BVar1 = GetMessage16(0x0, 0x0, 0x0, &local_msg_1);
    if (BVar1 == 0) {
        return local_msg_1.wparam;
    }
    //   if ((_u16_1050_5bc8 + 0x8) != 0) goto code_r0x100894cd;
    //   goto LAB_1008_94dc;
    // code_r0x100894cd:
    BVar1 = IsDialogMessage16(&local_msg_1, &DAT_1050_1050);
    if (BVar1 == 0) {
        //
        // LAB_1008_94dc:
        if (PTR_LOOP_1050_0398.is_null() == false) {
            IVar2 = TranslateAccelerator16(
                &local_msg_1,
                (HACCEL16) & DAT_1050_1050,
                PTR_LOOP_1050_0398,
            );
            //   if (IVar2 != 0) goto LAB_1008_949c;
        }
        TranslateMessage16(&local_msg_1);
        DispatchMessage16(&local_msg_1);
    }
    //   goto LAB_1008_949c;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn unk_win_msg_op_1008_9510(param_1: *mut i16) {
    let mut has_message: bool;
    let mut IVar1: i16;
    let mut local_14: MSG16; //
                             //
                             // LAB_1008_9578:
    if (*param_1 != 0) {
        has_message = GetMessage16(0x0, 0x0, 0x0, &local_14);
        if (has_message != 0) {
            //   if ((_u16_1050_5bc8 + 0x8) != 0) goto code_r0x10089538;
            // TODO: goto LAB_1008_9547;
        }
    }
    return;
    // code_r0x10089538:
    has_message = IsDialogMessage16(&local_14, &DAT_1050_1050);
    if (has_message == 0) {
        //
        // LAB_1008_9547:
        if (PTR_LOOP_1050_0398.is_null() == false) {
            IVar1 =
                TranslateAccelerator16(&local_14, (HACCEL16) & DAT_1050_1050, PTR_LOOP_1050_0398);
            //   if (IVar1 != 0) goto LAB_1008_9578;
        }
        TranslateMessage16(&local_14);
        DispatchMessage16(&local_14);
    }
    //   goto LAB_1008_9578;
}
pub fn set_struct_op_1008_9584(param_1: *mut astruct_20, mut param_2: u32) {
    let mut iVar1: *mut astruct_20;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.offset_0x0 = 0x389a;
    iVar1.base_0x2 = 0x1008;
    iVar1.field2_0x4 = param_2;
    param_1.offset_0x0 = 0x9d2e;
    iVar1.base_0x2 = 0x1008;
    iVar1.field3_0x8 = 0;
    iVar1.field139_0xac = 0x2000000;
    iVar1.field140_0xb0 = 0;
    iVar1.field141_0xb4 = 0x8000;
    iVar1.field142_0xb6 = 0x8000;
    iVar1.field143_0xb8 = 0x8000;
    iVar1.field144_0xba = 0x8000;
    iVar1.field145_0xbc = 0;
    iVar1.field146_0xbe = 0;
    iVar1.field147_0xc2 = 0;
    iVar1.hcursor_field_0xc4 = 0;
    iVar1.hgdiobj_field_0xc6 = 0;
    iVar1.field150_0xc8 = 0x2008;
    iVar1.field151_0xca = 0;
    param_1.offset_0x0 = 0x380a;
    iVar1.base_0x2 = 0x1008;
    iVar1.field60_0x5b = '\0';
    *&iVar1.field4_0xa = 0;
    return;
}
pub fn pass1_1008_9628(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8) == 0) {
        (param_1 + 0x8) = param_2;
    }
    return;
}
pub fn send_msg_1008_9640(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        SendMessage16(0x0, param_2, 0x86, (param_1 + 0x8));
    }
    return;
}
pub fn set_win_text_1008_9664(mut param_1: u32, mut param_2: u16, param_3: *mut c_char) {
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)), param_3);
    SetWindowText16(param_1 & 0xffff0000 | (param_1 + 0xa), (param_1 + 0x8));
    return;
}
pub fn destroy_win_1008_9698(param_1: *mut astruct_871, mut param_2: u16) {
    DestroyWindow16(param_1.hwnd_0x8);
    return;
}

pub fn show_win_1008_96ae(mut param_1: u32, param_2: INT16) -> BOOL16 {
    let mut BVar1: bool;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        BVar1 = ShowWindow16(param_2, (param_1 + 0x8));
        return BVar1;
    }
    return 0x0;
}
pub fn win_ui_reg_class_1008_96d2(param_1: *mut StructA) {
    let mut BVar1: bool;
    let mut AVar2: ATOM;
    let mut wndclass: WNDCLASS16;

    wndclass.lpsz_class_name = param_1 + 0x5b;
    BVar1 = GetClassInfo16(
        &wndclass,
        CONCAT22(wndclass.lpsz_class_name, 0x1050),
        param_1,
    );
    if (BVar1 == 0) {
        wndclass.style = (param_1 + 0xc8);
        wndclass.lpfn_wnd_proc = 0x5632;
        wndclass.lpfn_wnd_proc = 0x1008;
        wndclass._6_4_ = 0x40000;
        wndclass.h_instance = HINSTANCE16_1050_038c;
        wndclass.h_icon = (param_1 + 0xc2);
        wndclass.h_cursor = (param_1 + 0xc4);
        wndclass.hbr_background = (param_1 + 0xc6);
        wndclass.lpsz_menu_name = 0;
        wndclass.lpsz_class_name = param_1;
        AVar2 = RegisterClass16(&wndclass);
        if (AVar2 == 0) {
            fn_ptr_op_1000_24cd(0x0);
        }
    }
    return;
}
pub fn create_window_ex_1008_9760(in_struct_1: *mut StructA) {
    let mut window_handle: HWND16;
    let struct_1: *mut StructA;
    let mut uVar1: u16;

    uVar1 = (in_struct_1 >> 0x10);
    struct_1 = in_struct_1;
    if (struct_1.field4_0x8 == 0) {
        window_handle = CreateWIndowEx16(
            (in_struct_1 & 0xffff | uVar1 << 0x10),
            HINSTANCE16_1050_038c,
            struct_1.field159_0xca,
            struct_1.field149_0xbc,
            struct_1.field148_0xba,
            struct_1.field147_0xb8,
            struct_1.field146_0xb6,
            struct_1.field145_0xb4,
            struct_1.field140_0xac,
            0x1050039e,
            (in_struct_1 & 0xffff0000 | ZEXT24(&struct_1.field60_0x5b)),
            &struct_1.field_0xb0,
        );
        struct_1.field4_0x8 = window_handle;
    }
    if (struct_1.field4_0x8 == 0) {
        fn_ptr_op_1000_24cd(0x0);
    }
    return;
}
pub fn begin_end_paint_1008_97c8(param_1: *mut astruct_837, mut param_2: u16) {
    BeginPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    EndPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn unk_win_op_1008_97f2(
    param_1: u32,
    param_2: *mut i16,
    param_3: WPARAM16,
    param_4: *mut u8,
    mut param_5: u16,
) -> u32 {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut paVar6: *mut astruct_864;
    let mut UVar7: u16;
    let mut unaff_CS: u8;
    let mut uVar8: u32;
    let mut uVar9: u8;
    let mut uVar10: u8;

    paVar6 = param_1;
    UVar7 = (param_1 >> 0x10);
    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2));
        } else {
            ppcVar1 = (*param_1 + 0x70);
            (**ppcVar1)();
        }
        uVar5 = 0x1;
        // TODO: goto LAB_1008_9a95;
    }
    uVar10 = (param_1 >> 0x10);
    uVar9 = SUB41(param_1, 0x0);
    if (param_5 < 0x2c) {
        unaff_CS = 0x8;
        // switch(param_5) {
        match param_5 {
            // case 0x1:
            1 => {}
            //   break;
            // case 0x2:
            2 => {
                ppcVar1 = (*param_1 + 0x3c);
                (**ppcVar1)(0x1008);
                SetWindowLong16(0x0, 0x0, paVar6.hwnd_0x8);
                BVar2 = IsWindow16(paVar6[0x12].hwnd_0x8);
                if (BVar2 != 0) {
                    PostMessage16(param_1, 0xc7, 0x111, paVar6[0x12].hwnd_0x8);
                }
            }
            //   break;
            // case 0x3:
            3 => {
                ppcVar1 = (*param_1 + 0x54);
                (**ppcVar1)(0x8, uVar9, UVar7, param_3, param_2);
            }
            //   break;
            // default:
            _ => {}
            // TODO: goto switchD_1008_9b30_caseD_4;
            // case 0x5:
            5 => {
                ppcVar1 = (*param_1 + 0x58);
                (**ppcVar1)(0x8, uVar9, uVar10, param_3, param_2, param_4);
            }
            //   break;
            // case 0x7:
            7 => {
                ppcVar1 = (*param_1 + 0x50);
                (**ppcVar1)(0x8, param_1, param_4);
            }
            //   break;
            // case 0x8:
            8 => {
                ppcVar1 = (*param_1 + 0x74);
                (**ppcVar1)(0x8, param_1, param_4);
            }
            //   break;
            // case 0xd:
            0xd => {
                ppcVar1 = (*param_1 + 0x84);
                iVar4 = (**ppcVar1)(
                    0x8,
                    uVar9,
                    uVar10,
                    param_2,
                    CONCAT12(param_4._0_1_, param_3),
                );
            }
            // TODO: goto LAB_1008_9ada;
            // case 0xf:
            0xf => {
                ppcVar1 = (*param_1 + 0x34);
                (**ppcVar1)(0x1008, param_1);
            }
            //   break;
            // case 0x10:
            0x10 => {
                ppcVar1 = (*param_1 + 0x38);
                uVar8 = (**ppcVar1)(0x1008, param_1);
                return uVar8;
            }
            // case 0x19:
            0x19 => {
                ppcVar1 = (*param_1 + 0x78);
                uVar3 = (**ppcVar1)(
                    0x8,
                    uVar9,
                    uVar10,
                    param_2,
                    CONCAT12(param_4._0_1_, param_3),
                );
                return CONCAT22(0x1050, uVar3);
            }
            // case 0x1c:
            0x1c => {
                ppcVar1 = (*param_1 + 0x30);
                (**ppcVar1)(0x8, param_1, param_4);
            }
        };
    } else if (param_5 == 0x112) {
        if ((PTR_LOOP_1050_039a.is_null())
            && (
                ppcVar1 = (*param_1 + 0x48),
                iVar4 = (**ppcVar1)(),
                iVar4 != 0,
            ))
        {
            make_def_wnd_proc_1008_9ce6(
                paVar6,
                UVar7,
                CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)),
                param_4,
                0x112,
            );
        }
    } else if (param_5 < 0x113) {
        if (param_5 == 0x86) {
            ppcVar1 = (*param_1 + 0x80);
            uVar8 = (**ppcVar1)();
            return uVar8;
        }
        if (param_5 < 0x87) {
            if (param_5 == 0x85) {
                ppcVar1 = (*param_1 + 0x7c);
                uVar8 = (**ppcVar1)();
                return uVar8;
            }
            if (param_5 < 0x86) {
                if (param_5 == '7') {
                    return &paVar6[0x13].field_0x4;
                }
                if (param_5 == 'A') {
                    ppcVar1 = (*param_1 + 0x2c);
                    (**ppcVar1)();
                    // TODO: goto switchD_1008_9b30_caseD_1;
                }
            }
            // switchD_1008_9b30_caseD_4:
            if ((param_5 < 0x400) || (0x7ffe < param_5)) {
                uVar8 = make_def_wnd_proc_1008_9ce6(
                    paVar6,
                    UVar7,
                    CONCAT22(param_3, param_2),
                    param_4,
                    param_5,
                );
                return uVar8;
            }
            ppcVar1 = (*param_1 + 0x28);
            (**ppcVar1)(
                unaff_CS,
                uVar9,
                uVar10,
                param_2,
                param_3,
                CONCAT22(param_5, param_4),
            );
        } else if (param_5 == 0x100) {
            if (PTR_LOOP_1050_039a.is_null()) {
                ppcVar1 = (*param_1 + 0x6c);
                (**ppcVar1)();
            }
        } else if (param_5 == 0x102) {
            if (PTR_LOOP_1050_039a.is_null()) {
                ppcVar1 = (*param_1 + 0x68);
                (**ppcVar1)();
            }
        } else {
            //   if (param_5 != 0x111) goto switchD_1008_9b30_caseD_4;
            if ((param_4 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a.is_null())) {
                if (param_2.is_null()) {
                    ppcVar1 = (*param_1 + 0x40);
                    (**ppcVar1)();
                } else {
                    ppcVar1 = (*param_1 + 0x44);
                    (**ppcVar1)();
                }
            }
        }
    } else if (param_5 == 0x204) {
        if (PTR_LOOP_1050_039a.is_null()) {
            ppcVar1 = (*param_1 + 0x60);
            (**ppcVar1)();
        }
    } else if (param_5 < 0x205) {
        if (param_5 == 0x113) {
            if (_PTR_LOOP_1050_0388 != 0) {
                pass1_1008_932a(_PTR_LOOP_1050_0388);
            }
        } else if (param_5 == 0x117) {
            if (param_3 == 0) {
                ppcVar1 = (*param_1 + 0x4c);
                (**ppcVar1)();
            } else {
                ppcVar1 = (*param_1 + 0x20);
                (**ppcVar1)();
            }
        } else {
            //   if (param_5 != 0x201) goto switchD_1008_9b30_caseD_4;
            if (PTR_LOOP_1050_039a.is_null()) {
                ppcVar1 = (*param_1 + 0x5c);
                (**ppcVar1)();
            }
        }
    } else if (param_5 == 0x210) {
        ppcVar1 = (*param_1 + 0x64);
        (**ppcVar1)();
    } else {
        if (param_5 == 0x30f) {
            //
            // LAB_1008_9af8:
            ppcVar1 = (*param_1 + 0x8c);
            iVar4 = (**ppcVar1)(); //
                                   // LAB_1008_9ada:
            return iVar4;
        }
        if (param_5 == 0x311) {
            ppcVar1 = (*param_1 + 0x88);
            iVar4 = (**ppcVar1)();
        //   if (iVar4 != 0) goto LAB_1008_9af8;
        } else {
            //   if (param_5 != 0x3b9) goto switchD_1008_9b30_caseD_4;
            ppcVar1 = (*param_1 + 0x24);
            (**ppcVar1)();
        }
    }
    // switchD_1008_9b30_caseD_1:
    uVar5 = 0; //
               // LAB_1008_9a95:
    return uVar5;
}

pub unsafe fn pass1_1008_9c16(mut param_1: u16, mut param_2: u32, mut param_3: u32) -> LRESULT {
    let mut LVar1: LRESULT;

    LVar1 = make_def_wnd_proc_1008_9ce6(
        param_1,
        param_2,
        CONCAT22(param_3, (param_2 >> 0x10)),
        (param_3 >> 0x10),
        0x85,
    );
    return LVar1;
}

pub unsafe fn pass1_1008_9c30(mut param_1: u16, mut param_2: u32, mut param_3: u32) -> LRESULT {
    let mut LVar1: LRESULT;

    LVar1 = make_def_wnd_proc_1008_9ce6(
        param_1,
        param_2,
        CONCAT22(param_3, (param_2 >> 0x10)),
        (param_3 >> 0x10),
        0x86,
    );
    return LVar1;
}
pub fn pass1_1008_9c4a() {
    return;
}
pub fn pass1_1008_9c4e() {
    return;
}
pub fn pass1_1008_9c52() {
    return;
}
pub fn get_stock_obj_1008_9c56() {
    GetStockObject16(HOLLOW_BRUSH);
    return;
}
pub unsafe fn pass1_1008_9c60(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut u32,
    mut param_4: i16,
) {
    let mut ppcVar1: *mut *mut code;

    if ((param_4 == 0xc7) && (param_3.is_null() == false)) {
        ppcVar1 = *param_3;
        (**ppcVar1)();
    }
    return;
}
pub fn pass1_1008_9c86(mut param_1: u32, param_2: *mut c_char, mut param_3: i16) {
    let mut uVar1: u16;

    uVar1 = str_op_1000_3da4((param_1 & 0xffff0000 | (param_1 + 0xa)));
    if (param_3 < uVar1) {
        uVar1 = param_3 - 0x1;
    }
    str_op_1000_3dbe(param_2, (param_1 & 0xffff0000 | (param_1 + 0xa)), uVar1);
    return;
}

pub fn pass1_1008_9cc4(mut param_1: u32, mut param_2: i16) -> BOOL16 {
    if ((param_1 + 0x8) != param_2) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn pass1_1008_9ce0() -> u16 {
    return 0x0;
}

pub unsafe fn make_def_wnd_proc_1008_9ce6(
    param_1: *mut astruct_864,
    mut param_2: u16,
    lparam_param_3: LPARAM,
    wparam_param_4: WPARAM16,
    msg_param_5: u16,
) -> LRESULT {
    let mut lvar1: LRESULT;
    lvar1 = DefWindowProc16(
        lparam_param_3,
        wparam_param_4,
        msg_param_5,
        param_1.hwnd_0x8,
    );
    return lvar1;
}

pub unsafe fn pass1_1008_9d02(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_9d36(
    param_1: *mut astruct_19,
    param_2: *mut astruct_19,
    mut param_3: u16,
) {
    let mut uVar1: u16;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut uVar4: u32;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u16;
    let mut uVar7: u32;
    let mut iStack4: i16;

    uVar3 = (in_EDX >> 0x10);
    struct_op_1018_4cda(CONCAT22(param_2, param_1), param_3);
    param_1.field15_0x1c = 0x389a;
    param_1.field16_0x1e = 0x1008;
    param_1.field15_0x1c = 0x3aa8;
    param_1.field16_0x1e = 0x1008;
    param_1.field17_0x20 = 0;
    puVar6 = pass1_1008_3e38(CONCAT22(param_2, &param_1.field44_0x52));
    uVar4 = CONCAT22(uVar3, (puVar6 >> 0x10));
    CONCAT22(param_2, param_1) = 0x9fb2;
    param_1.segment_0x2 = 0x1008;
    param_1.field15_0x1c = 0x9fca;
    param_1.field16_0x1e = 0x1008;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field18_0x22), NULL, 0x30);
    pass1_1018_4dce(uVar4, CONCAT22(param_2, param_1), 0x1c0);
    iStack4 = 0;
    uVar3 = 0x1018;
    loop {
        uVar1 = FUN_1010_830a(
            iStack4 + 0x1c0,
            uVar4,
            uVar3,
            _u16_1050_14cc,
            iStack4 + 0x1c0,
        );
        (&param_1.field18_0x22)[iStack4 * 0x2] = uVar1;
        (&param_1.field19_0x24)[iStack4 * 0x2] = uVar4;
        iStack4 += 0x1;
        uVar3 = 0x1010;
        if iStack4 >= 0xc {
            break;
        }
    }
    uVar7 = pass1_1008_4772(&param_1.field18_0x22);
    uVar4 &= 0xffff0000;
    uVar3 = (uVar7 >> 0x10);
    pass1_1008_3e76(
        CONCAT22(param_2, &param_1.field44_0x52),
        0x0,
        (0x1e0 - (uVar7 + 0x8)) / 0x2 - 0x32,
        (0x280 - (uVar7 + 0x4)) / 0x2,
    );
    if (CONCAT22(param_2, param_1) == 0) {
        paVar2 = NULL;
        paVar5 = (uVar4 & 0xffff0000);
    } else {
        paVar5 = (uVar4 & 0xffff0000 | ZEXT24(param_2));
        paVar2 = &param_1.field15_0x1c;
    }
    pass1_1008_9262(
        paVar2,
        paVar5,
        _PTR_LOOP_1050_0388,
        (_PTR_LOOP_1050_0388 >> 0x10),
        0x50,
        CONCAT22(paVar5, paVar2),
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_9e5a(structd_param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut puVar4: *mut u8;
    let mut uVar5: u16;
    let mut structd_5: *mut StructD;
    let mut uVar6: u16;
    let mut puStack8: *mut u16;
    let mut iStack4: i16;

    uVar6 = (structd_param_1 >> 0x10);
    structd_5 = structd_param_1;
    structd_param_1.address_offset_field_0x0 = 0x9fb2;
    structd_5.address_offset_field_0x2 = 0x1008;
    structd_5.field_0x1c = 0x9fca;
    structd_5.field_0x1e = 0x1008;
    if (_PTR_LOOP_1050_0388 != 0) {
        if (structd_param_1.is_null()) {
            puVar4 = NULL;
            uVar5 = 0;
        } else {
            puVar4 = &structd_5.field_0x1c;
            uVar5 = uVar6;
        }
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x50, CONCAT22(uVar5, puVar4));
    }
    iStack4 = 0;
    loop {
        puVar1 = (&structd_5.field20_0x22)[iStack4 * 0x2];
        uVar2 = (&structd_5.field_0x24 + iStack4 * 0x4);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iStack4 += 0x1;
        if iStac4 >= 0xc {
            break;
        }
    }
    if (structd_param_1.is_null()) {
        puVar4 = NULL;
        uVar6 = 0;
    } else {
        puVar4 = &structd_5.field_0x1c;
    }
    puStack8 = CONCAT22(uVar6, puVar4);
    *puStack8 = 0x389a;
    (puVar4 + 0x2) = 0x1008;
    clenaup_win_ui_1018_4d22(structd_param_1);
    return;
}
pub unsafe fn pass1_1008_9f18(mut param_1: i16, mut param_2: u16, mut param_3: i16) {
    if (param_3 == 0x2) {
        pass1_1008_9f64(CONCAT22(param_2, param_1 + -0x1c));
        pass1_1010_1f62(CONCAT22(param_2, param_1 + -0x1c), 0x2);
    }
    return;
}

pub unsafe fn pass1_1008_9f48(param_1: *mut astruct_134) -> *mut astruct_76 {
    let mut iVar1: *mut astruct_134;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar2 = iVar1.field32_0x20 * 0x4;
    return CONCAT22(
        (&iVar1[0x1].field_0x2 + iVar2),
        (&iVar1[0x1].field_0x0 + iVar2),
    );
}
pub unsafe fn pass1_1008_9f64(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    piVar1 = (iVar2 + 0x20);
    *piVar1 = *piVar1 + 1;
    if (0xb < (iVar2 + 0x20)) {
        (iVar2 + 0x20) = 0;
    }
    return;
}

pub unsafe fn pass1_1008_9f80(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x1c));
    pass1_1008_9e5a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn pass1_1008_9fb2(
    mut param_1: u16,
    mut param_2: i16,
    param_3: u8,
    param_4: u8,
    param_5: u8,
    mut param_6: u16,
    mut param_7: i16,
    mut param_8: u16,
    mut param_9: u16,
) {
    let mut pcVar1: *mut c_char;
    let mut pbVar2: *mut u8;
    let mut bVar3: u8;
    let mut pcVar4: *mut code;
    let mut bVar5: u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut bVar8: u8;
    let mut uVar9: u16;
    let mut in_register_00000009: u32;
    let mut uVar10: u32;
    let mut paVar11: *mut Struct57;
    let mut unaff_SI: i16;
    let mut bVar13: u8;
    let mut bVar14: bool;
    let mut bVar15: bool;
    let mut paVar16: *mut astruct_19;
    let mut paVar12: *mut Struct57;

    uVar10 = CONCAT31(in_register_00000009, param_5);
    (param_2 + 0x1008) = &DAT_1050_1050;
    uVar7 = param_4;
    uVar9 = param_1 + 0xeff0;
    bVar13 = param_1 < 0x1010 || uVar9 < uVar7;
    uVar6 = uVar9 - uVar7;
    pcVar4 = swi(0x4);
    if (SBORROW2(param_1, 0x1010) != SBORROW2(uVar9, uVar7)) {
        (*pcVar4)();
    }
    bVar5 = ((uVar6 + 0xeff0) - bVar13) % 0x1d;
    pcVar1 = (param_2 + unaff_SI);
    bVar8 = uVar10;
    *pcVar1 = *pcVar1 + bVar8 + (uVar6 < 0x1010 || uVar6 + 0xeff0 < bVar13);
    pbVar2 = (param_2 + unaff_SI);
    bVar14 = *pbVar2 < bVar8 || (*pbVar2 - bVar8) < (0xb1 < bVar5);
    *pbVar2 = (*pbVar2 - bVar8) - (0xb1 < bVar5);
    pbVar2 = (param_2 + 0x18);
    bVar15 = *pbVar2 < param_3 || (*pbVar2 - param_3) < bVar14;
    *pbVar2 = (*pbVar2 - param_3) - bVar14;
    pbVar2 = (param_2 + unaff_SI + 0x89f);
    bVar13 = *pbVar2;
    bVar3 = *pbVar2 + bVar5 + 0x4e;
    *pbVar2 = bVar3 + bVar15;
    pcVar1 = (param_2 + unaff_SI);
    *pcVar1 = *pcVar1 + param_2 + (CARRY1(bVar13, bVar5 + 0x4e) || CARRY1(bVar3, bVar15));
    pbVar2 = (param_2 + unaff_SI);
    *pbVar2 = *pbVar2 | bVar8;
    paVar16 = struct_op_1010_1d48(CONCAT22(param_8, param_7), param_9);
    paVar11 = (uVar10 & 0xffff0000 | paVar16 >> 0x10);
    uVar7 = 0;
    (param_7 + 0xa) = 0;
    (param_7 + 0x410) = 0;
    (param_7 + 0x414) = 0;
    (param_7 + 0x416) = 0;
    (param_7 + 0x418) = 0;
    (param_7 + 0x41a) = 0;
    (param_7 + 0x41c) = 0;
    (param_7 + 0x41e) = 0;
    CONCAT22(param_8, param_7) = 0xad92;
    (param_7 + 0x2) = 0x1008;
    mem_op_1000_179c(0xc, paVar11);
    uVar9 = paVar11 | uVar7;
    paVar12 = (paVar11 & 0xffff0000 | uVar9);
    if (uVar9 == 0) {
        (param_7 + 0xa) = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar11, uVar7));
        (param_7 + 0xa) = uVar7;
        (param_7 + 0xc) = paVar12;
    }
    mem_op_1000_179c(0xc, paVar12);
    uVar9 = paVar12 | uVar7;
    if (uVar9 == 0) {
        uVar7 = 0;
        uVar9 = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar12, uVar7));
    }
    (param_7 + 0x410) = uVar7;
    (param_7 + 0x412) = uVar9;
    return;
}
pub fn struct_1008_9fd2(param_1: *mut astruct_19, mut param_2: u16) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut paVar3: *mut Struct57;
    let mut paVar6: *mut astruct_19;
    let mut paVar4: *mut Struct57;

    uVar5 = (in_EDX >> 0x10);
    paVar6 = struct_op_1010_1d48(param_1, param_2);
    paVar3 = CONCAT22(uVar5, (paVar6 >> 0x10));
    uVar1 = 0;
    (param_1 + 0xa) = 0;
    (param_1 + 0x410) = 0;
    (param_1 + 0x414) = 0;
    (param_1 + 0x416) = 0;
    (param_1 + 0x418) = 0;
    (param_1 + 0x41a) = 0;
    (param_1 + 0x41c) = 0;
    (param_1 + 0x41e) = 0;
    param_1.offset_0x0 = 0xad92;
    (param_1 + 0x2) = 0x1008;
    mem_op_1000_179c(0xc, paVar3);
    uVar2 = paVar3 | uVar1;
    paVar4 = (paVar3 & 0xffff0000 | uVar2);
    if (uVar2 == 0) {
        (param_1 + 0xa) = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar3, uVar1));
        (param_1 + 0xa) = uVar1;
        (param_1 + 0xc) = paVar4;
    }
    mem_op_1000_179c(0xc, paVar4);
    uVar2 = paVar4 | uVar1;
    if (uVar2 == 0) {
        uVar1 = 0;
        uVar2 = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar4, uVar1));
    }
    (param_1 + 0x410) = uVar1;
    (param_1 + 0x412) = uVar2;
    return;
}
