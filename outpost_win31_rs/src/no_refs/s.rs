use crate::block_1020::block_1020_0000::{draw_op_1020_041e, pass1_1020_022c, send_win_msg_1020_08fe, struct_1020_0baa, unk_draw_op_1020_0c3e, win_ui_palette_op_1020_0cd2};
use crate::block_1020::block_1020_1000::{destroy_win_1020_1dea, destroy_win_1020_1e1e, draw_op_1020_15de, pass1_1020_1d8e, pass1_1020_1da8, pass1_1020_1eea, unk_win_ui_op_1020_1418, win_ui_op_1020_150e};
use crate::block_1020::block_1020_2000::{draw_line_1020_229c, pass1_1020_2286, pass1_1020_239c, pass1_1020_2488};
use crate::utils::CONCAT22;
use crate::windef::{HDC16, HGDIOBJ16, HPEN16};

pub unsafe fn pass1_1020_07aa(mut param_1: u16, param_2: *mut Struct19) {
    let mut iVar1: *mut Struct19;
    let mut uVar2: *mut Struct19;
    let mut local_16: [u8; 0x14] = [0; 0x14];

    draw_op_1020_041e(param_2);
    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2;
    if (iVar1[0x2].field24_0x30 == 0) {
        iVar1[0x2].field24_0x30 = 0x1;
        pass1_1008_5bdc(CONCAT22(0x1050, local_16));
        win_1008_5c9e(
            local_16,
            param_1,
            CONCAT22(0x1050, local_16),
            (param_2 & 0xffff0000 | ZEXT24(&iVar1[0x2].field25_0x32)),
        );
        pass1_1008_5c34(CONCAT22(0x1050, local_16));
    }
    return;
}

pub unsafe fn pass1_1020_07f4(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29 {
    pass1_1020_022c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_1020_09ba(mut param_1: u16, mut param_2: u16, param_3: *mut StructA) {
    let mut puVar1: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let iVar1: *mut StructA;
    let mut uVar3: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_2);
    create_window_ex_1008_9760(param_3);
    mem_op_1000_179c(0xe, paVar2);
    puVar1 = (paVar2 | param_1);
    iVar1 = param_3;
    uVar3 = (param_3 >> 0x10);
    if (puVar1.is_null() == false) {
        struct_1020_0baa(puVar1, CONCAT22(paVar2, param_1), iVar1.field4_0x8);
        iVar1[0x1].field11_0x16 = param_1;
        iVar1[0x1].field12_0x18 = puVar1;
        return;
    }
    iVar1[0x1].field11_0x16 = 0;
    return;
}

pub unsafe fn pass1_1020_0a0c(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    destroy_win_1008_628e(param_1);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0xe2);
    uVar2 = (iVar4 + 0xe4);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 1);
    }
    (iVar4 + 0xe2) = 0;
    (iVar4 + 0xe6) = 0;
    return;
}

pub unsafe fn pass1_1020_0a52(mut param_1: u16, mut param_2: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u32;

    uVar2 = (param_2 >> 0x10);
    uVar1 = param_2;
    unk_draw_op_1020_0c3e((uVar1 + 0xe2));
    if ((uVar1 + 0xe6) == 0) {
        (uVar1 + 0xe6) = 0x1;
        (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        uVar3 = pass1_1038_af40(uVar1, param_1, _PTR_LOOP_1050_5b7c, (uVar1 + 0x8), 0x6);
        (uVar1 + 0xe8) = uVar3;
        (uVar1 + 0xea) = (uVar3 >> 0x10);
    }
    return;
}

pub unsafe fn pass1_1020_0aa6(mut param_1: u32) {
    win_ui_palette_op_1020_0cd2((param_1 + 0xe2));
    return;
}

pub unsafe fn pass1_1020_0abc(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe6) != 0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x10);
        (**ppcVar1)();
    }
    return;
}

pub unsafe fn pass1_1020_0ae8(param_1: *mut astruct_63, param_2: u8) -> *mut astruct_63 {
    send_win_msg_1020_08fe(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_0e2c(param_1: *mut astruct_868) {
    get_win_ui_info_op_1020_7a50(param_1);
    cleanup_ui_op_1020_1038(param_1);
    return;
}

pub unsafe fn realize_palette_1020_0e46(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut puVar4: HGDIOBJ16;
    let mut iVar4: *mut astruct_800;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut puVar2: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (param_2 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar2 = (param_1 + 0xf2);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        puVar2 = iVar4.field102_0x66;
        fn_ptr_1 = (*puVar2 + 0x18);
        (**fn_ptr_1)();
        UnrealizeObject16(puVar2);
        uVar1 = (param_1 + 0xf2);
        RealizePalette16(*(uVar1 + 0x7c));
    }
    return;
}

pub unsafe fn pass1_1020_0e8e(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: i16,
    mut param_6: i16,
) {
    let mut ppcVar1: *mut *mut code;

    win_ui_cursor_op_1020_1294(param_2, CONCAT22(param_4, param_3), param_5, param_6);
    if (param_1 == 0) {
        ppcVar1 = ((param_3 + 0x4) + 0x5c);
        (**ppcVar1)();
    }
    return;
}

pub unsafe fn win_help_op_1020_0ec4(mut param_1: u16, param_2: *mut u32, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut cVar2: u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let mut paVar8: *mut astruct_477;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut wparam: WPARAM16;
    let mut hwnd: HWND16;
    let mut iVar11: i16;
    let mut in_stack_0000fff4: u16;
    let mut uVar12: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    uVar7 = (param_2 >> 0x10);
    uVar3 = param_2;
    if (param_3 == 0xfb) {
        puVar9 = mixed_1010_20ba(
            paVar6,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fff4, 0x30),
            in_stack_0000fe9c,
            in_stack_0000ffc0,
            in_stack_0000ffc6,
            in_stack_0000ffca,
        );
        uVar12 = puVar9;
        pass1_1010_375e(puVar9);
        ppcVar1 = (*param_2 + 0x14);
        (**ppcVar1)();
        uVar10 = pass1_1010_375e(puVar9 & 0xffff0000 | uVar12);
        uVar12 = (uVar10 >> 0x10);
        pass1_1010_4788(
            uVar10,
            uVar12,
            (uVar3 + 0xf2),
            (uVar10 & 0xffff | uVar12 << 0x10),
        );
        return;
    }
    if (0xfb < param_3) {
        match param_3 {
            _ => {
                return;
            }
            0x12a => {
                hwnd = (uVar3 + 0x8);
                wparam = 0xf012;
            }
            //   break;
            0x12c => {
                hwnd = (uVar3 + 0x8);
                wparam = 0xf020;
            }
        }
        PostMessage16(0x0, wparam, 0x112, hwnd);
        return;
    }
    if (param_3 == 0xf3) {
        iVar11 = 0x3;
    } else {
        if (0xf3 < param_3) {
            return;
        }
        cVar2 = param_3;
        if ((cVar2 + 0x91) == 0) {
            uVar4 = FUN_1010_830a(
                param_3 & 0xff00 | (cVar2 + 0x91),
                paVar6,
                unaff_CS,
                _u16_1050_14cc,
                0x1f8,
            );
            WinHelp16(0x28, 0x1, CONCAT22(paVar6, uVar4), (uVar3 + 0x8));
            return;
        }
        if (cVar2 == 'r') {
            iVar11 = uVar3 + 0xa;
            uVar4 = uVar7;
            paVar8 = mixed_1010_20ba(
                paVar6,
                _u16_1050_0ed0,
                CONCAT22(iVar11, 0x30),
                in_stack_0000fe98,
                in_stack_0000ffbc,
                in_stack_0000ffc2,
                in_stack_0000ffc6,
            );
            uVar5 = (paVar8 >> 0x10);
            pass1_1010_3770(uVar5, paVar8, CONCAT22(uVar4, iVar11));
            pass1_1038_af40(uVar3, uVar5, _PTR_LOOP_1050_5b7c, (uVar3 + 0x8), 0x3);
            return;
        }
        if (cVar2 == -0xf) {
            iVar11 = 0x1;
        } else {
            if (cVar2 != -0xe) {
                return;
            }
            iVar11 = 0x2;
        }
    }
    pass1_1010_4674((uVar3 + 0xf2), iVar11, 0x1010, &DAT_1050_1050);
    return;
}

pub unsafe fn enable_menu_1020_1000(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: HMENU,
) {
    if (param_4 != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0x3, param_5);
    return;
}

pub unsafe fn FUN_1020_101f() {
    return;
}

pub unsafe fn pass1_1020_1022(mut param_1: u32) {
    draw_op_1020_15de((param_1 + 0xf6));
    return;
}

pub unsafe fn pass1_1020_135e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    cleanup_menu_ui_op_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_170a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    win_ui_op_1020_150e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_1780(param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x6c);
    (**ppcVar1)();
    destroy_win_1040_8212(param_1);
    return;
}

pub unsafe fn mixed_ui_op_1020_179c(
    mut param_1: u16,
    mut param_2: u16,
    structb_param_1: *mut StructB,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut pvVar1: LPVOID = null_mut();
    let mut IVar1: i16;
    let mut iVar2: i16;
    let mut puVar11: *mut astruct_816;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut x: i16;
    let mut y_offset: i16;
    let mut IVar2: i16;
    let mut HVar3: HWND16;
    let mut uVar8: u16;
    let mut puVar4: *mut u8;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut struct_9: *mut StructB;
    let mut iVar9: i16;
    let mut uVar10: *mut StructB;
    let mut uVar13: u16;
    let mut uVar11: u16;
    let mut uVar14: u16;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fd2a: u16;
    let mut in_stack_0000fd2e: u16;
    let mut in_stack_0000fd30: u16;
    let mut in_stack_0000fe4e: u16;
    let mut in_stack_0000fe52: u16;
    let mut in_stack_0000fe54: u16;
    let mut in_stack_0000fe58: u16;
    let mut in_stack_0000fe5a: u16;
    let mut in_stack_0000fe5c: u16;
    let mut in_stack_0000fe5e: u16;
    let mut uVar16: u16;
    let mut in_buffer_4: *mut c_char;
    let mut in_stack_0000fe88_00: u16;
    let mut in_buf_len_5: i16;
    let mut x_6e: i16;
    let mut y: i16;
    let mut x106: u16;
    let mut y4: i16;
    let mut hwnd_10: HWND16;
    let mut cx: i16;
    let mut cy: i16;
    let mut uStack78: u16;
    let mut uStack76: u16;
    let mut uStack74: u32;
    let mut HStack70: HWND16;
    let mut uStack68: u32;
    let mut uStack64: u32;
    // pub unsafe fn *pvStack60;
    let mut uStack56: u16;
    let mut ppcStack54: *mut *mut c_char;
    let mut uStack50: u32;
    let mut local_2e: *mut astruct_92;
    let mut local_1c: RECT16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut puStack16: *mut u32;
    let mut pIStack12: *mut INT16 = null_mut();
    let mut uStack8: u16;
    let mut puStack6: *mut u32;
    let mut puVar1: *mut u32;
    let mut puVar2: u32;
    let mut uVar12: u16;
    let mut in_resc_id_3: *mut u8;
    let mut uVar15: u16;
    let mut in_stack_0000fe88: u16;
    let mut uVar9: u32;
    let mut fn_ptr_1: *mut *mut code;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    dialog_ui_fn_1040_78e2(structb_param_1);
    uVar15 = 0x89;
    puStack6 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        0x890009,
        in_stack_0000fd2e,
        in_stack_0000fe52,
        in_stack_0000fe58,
        in_stack_0000fe5c,
    );
    paVar8 = (paVar8 & 0xffff0000 | puStack6 >> 0x10);
    uVar1 = pass1_1010_65d0(puStack6, uVar15);
    uStack8 = (uVar1 == 0);
    uVar2 = pass1_1010_65d0(puStack6, 0x86);
    if (uVar2 != 0) {
        uStack8 = 0;
    }
    puVar10 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fe88_00, 0x39),
        in_stack_0000fd30,
        in_stack_0000fe54,
        in_stack_0000fe5a,
        in_stack_0000fe5e,
    );
    paVar8 = (paVar8 & 0xffff0000 | puVar10 >> 0x10);
    pvVar1 = puVar10;
    uVar10 = (structb_param_1 >> 0x10);
    struct_9 = structb_param_1;
    struct_9[0x7].field1_0x2 = pvVar1;
    HVar3 = (puVar10 >> 0x10);
    struct_9[0x7].hwnd_0x6 = HVar3;
    uVar16 = struct_9[0x7].field1_0x2;
    fn_ptr_1 = (*&struct_9[0x7].field1_0x2 + 0x10);
    (**fn_ptr_1)(0x1010, uVar16, HVar3, uStack8);
    mem_op_1000_179c(0x12, paVar8);
    uStack76 = paVar8;
    puVar4 = (uStack76 | pvVar1);
    paVar8 = (paVar8 & 0xffff0000 | ZEXT24(puVar4));
    uStack78 = pvVar1;
    if (puVar4.is_null()) {
        struct_9[0x7].lpvoid_field_0x8 = 0;
    } else {
        pass1_1020_1eea(
            puVar4,
            CONCAT22(uStack76, pvVar1),
            structb_param_1,
            struct_9.lpvoid_field_0x8,
        );
        struct_9[0x7].lpvoid_field_0x8 = pvVar1;
        struct_9[0x7].max_count_field_0x10 = paVar8;
    }
    puVar1 = &struct_9[0x7].field1_0x2;
    pIStack12 = (puVar1 & 0xffff0000 | (puVar1 + 0xa));
    puStack16 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(uVar16, 0x48),
        in_stack_0000fd2a,
        in_stack_0000fe4e,
        in_stack_0000fe54,
        in_stack_0000fe58,
    );
    GetClientRect16(&local_1c, &DAT_1050_1050);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    uVar13 = (pIStack12 >> 0x10);
    iVar9 = pIStack12;
    (iVar9 + 0x6) = IVar1 + iStack22;
    uVar11 = (puStack16 >> 0x10);
    iStack18 = (puStack16 + 0xa);
    iStack20 = (puStack16 + 0xc);
    (iVar9 + 0x2) = (iStack20 - (iVar9 + 0x6)) / 0x2;
    iVar2 = iStack18 - (iVar9 + 0x4);
    uVar5 = iVar2 >> 0xf;
    uVar9 = uVar5;
    *pIStack12 = iVar2 / 0x2;
    pass1_1028_dc52(CONCAT22(0x1050, &local_2e), 0x1, 0x0, 0x100);
    uStack56 = 0;
    loop {
        uVar6 = uVar9;
        puVar11 = &local_2e;
        pass1_1028_e4ec(CONCAT22(0x1050, puVar11));
        uStack50 = CONCAT22(uVar6, puVar11);
        uVar7 = uVar6 | puVar11;
        uVar9 = uVar7;
        if (uVar7 == 0) {
            break;
        }
        ppcStack54 = puVar11.field16_0x10;
        if (ppcStack54.is_null() == false) {
            pass1_1000_3cea(
                structb_param_1 & 0xffff0000 | ZEXT24(&struct_9.field8_0x10),
                *ppcStack54,
            );
        }
    }
    uVar3 = pass1_1020_1da8(puVar11, 0x0, structb_param_1);
    struct_9[0x7].field5_0xa = uVar3;
    uVar4 = pass1_1010_65d0(puStack6, 0x86);
    if ((uVar4 == 0) || (struct_9[0x7].field5_0xa.is_null() == false)) {
        puVar2 = &struct_9[0x7].field1_0x2;
        (puVar2 + 0x2c) = 0;
        hwnd_10 = GetDlgItem16(0x175, struct_9.lpvoid_field_0x8);
        if (uStack8 != 0) {
            load_string_1010_84e0(
                _u16_1050_14cc,
                (_u16_1050_14cc >> 0x10),
                0x100,
                &stack0xfe88,
                &DAT_1050_1050,
            );
            SetWindowText16(CONCAT13(0x10, CONCAT12(0x50, &stack0xfe88)), hwnd_10);
        }
        pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c, destroy_win_1020_1e1e);
        GetWindowRect16(CONCAT22(0x1050, &x_6e), hwnd_10);
        cx = _x106 - x_6e;
        cy = y4 - y;
        x = cx - (pIStack12 + 0x4);
        y_offset = GetSystemMetrics16(SM_CYCAPTION);
        MoveWindow16(0x0, cy, cx, y - y_offset, -x / 0x2, hwnd_10);
    } else {
        win_1008_5c7c(uVar4, uVar7, _u16_1050_02a0, 0x9d0001);
        (struct_9 + 0x7).field0_0x0 = uVar4;
        pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c, destroy_win_1020_1dea);
    }
    EnumChildWindows1(0x0, pvStack60, struct_9.lpvoid_field_0x8);
    FreeProcInstance16(pvStack60);
    HStack70 = GetDlgItem16(0x1, struct_9.lpvoid_field_0x8);
    GetWindowRect16(CONCAT22(0x1050, &local_1c), HStack70);
    uStack64 = _param_2;
    local_1c.x = _param_2 - local_1c.x;
    uStack74 = CONCAT22(local_1c.x, iStack22 - local_1c.y);
    uStack68 = local_1c & 0xffff0000 | (-(local_1c.x - (pIStack12 + 0x4)) / 0x2);
    IVar2 = GetSystemMetrics16(SM_CYCAPTION);
    uStack68 = uStack68 & 0xffff | (uStack68 - IVar2) << 0x10;
    if (struct_9[0x7].field5_0xa.is_null()) {
        //    if (uStack8 == 0) goto LAB_1020_1b24;
        in_buf_len_5 = 0x72e;
        in_resc_id_3 = &stack0xfe88;
        in_buffer_4 = &DAT_1050_1050;
    } else {
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x100,
            &stack0xfe88,
            &DAT_1050_1050,
        );
        HVar3 = GetDlgItem16(0x175, struct_9.lpvoid_field_0x8);
        SetWindowText16(CONCAT22(0x1050, &stack0xfe88), HVar3);
        in_buffer_4 = &stack0xfe88;
        in_buf_len_5 = &DAT_1050_1050;
        in_resc_id_3 = 0x3ff;
    }
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        in_resc_id_3,
        in_buffer_4,
        in_buf_len_5,
    );
    SetWindowText16(CONCAT22(0x1050, &stack0xfe88), HStack70); //
    // LAB_1020_1b24:
    MoveWindow16(
        0x0,
        uStack74,
        (uStack74 >> 0x10),
        uStack68,
        uStack68,
        HStack70,
    );
    uVar14 = (pIStack12 >> 0x10);
    iVar2 = pIStack12;
    SetWindowPos16(
        0x44,
        (iVar2 + 0x6),
        (iVar2 + 0x4),
        (iVar2 + 0x2),
        *pIStack12,
        0x0,
        struct_9.lpvoid_field_0x8,
    );
    return;
}

pub unsafe fn pass1_1020_1b68(param_1: *mut astruct) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct;
    let mut uVar4: *mut astruct;

    uVar4 = param_1 >> 0x10;
    iVar4 = param_1;
    puVar1 = iVar4.field143_0x92;
    uVar2 = iVar4.field144_0x94;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    iVar4.field143_0x92 = 0;
    pass1_1010_4f48(iVar4.field142_0x8e);
    iVar4.field142_0x8e = null_mut();
    return;
}

pub unsafe fn pass1_1020_1bb6(mut param_1: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x92) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

pub unsafe fn enable_window_1020_1bd4(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_901,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut bVar2: bool;
    let mut hwnd: HWND16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar8: u16;
    let mut puStack12: *mut u32;
    let mut uVar7: u32;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    bVar2 = false;
    pass1_1020_1d8e(
        CONCAT13((param_4 >> 0x8), CONCAT12(param_4, param_3)),
        CONCAT22(param_6, param_5),
    );
    if (param_1 != 0) {
        if (param_1 < 0x2) {
            bVar2 = true;
        } else {
            hwnd = GetDlgItem16(0x1, param_3.field6_0x6);
            pass1_1010_4e8c(param_3.field141_0x8e);
            param_1 = EnableWindow16(0x1, hwnd);
            pass1_1010_4df0(paVar6, param_3.field141_0x8e);
            if ((param_1 == 0) && (bVar2 = true, param_3.field146_0x96 == 0)) {
                param_1 = EnableWindow16(0x0, hwnd);
            }
        }
    }
    if (bVar2) {
        uVar8 = 0x1000;
        mem_op_1000_179c(0xb4, paVar6);
        uVar4 = paVar6 | param_1;
        uVar7 = paVar6 & 0xffff0000 | uVar4;
        if (uVar4 == 0) {
            iVar3 = 0;
            uVar5 = 0;
        } else {
            uVar8 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar3 = string_1040_8520(
                uVar7,
                CONCAT22(paVar6, param_1),
                param_3.field6_0x6,
                0x20030,
                0x62a057b,
            );
            uVar5 = uVar7;
        }
        puStack12 = CONCAT22(uVar5, iVar3);
        ppcVar1 = (*puStack12 + 0x74);
        (**ppcVar1)(uVar8, iVar3, uVar5);
    }
    return;
}


pub unsafe fn post_win_msg_1020_1ca4(
    mut param_1: u32,
    param_2: *mut Struct57,
    mut param_3: u16,
) -> BOOL16 {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar6: u16;
    let mut puStack10: *mut u32;
    let mut uVar5: u32;

    uVar6 = (param_1 >> 0x10);
    if ((param_1 + 0x96) == 0) {
        pass1_1010_4df0(param_2, (param_1 + 0x8e));
        if (param_3 == 0) {
            uVar6 = 0x1000;
            mem_op_1000_179c(0xb4, param_2);
            uVar3 = param_2 | param_3;
            uVar5 = param_2 & 0xffff0000 | uVar3;
            if (uVar3 == 0) {
                iVar2 = 0;
                uVar4 = 0;
            } else {
                uVar6 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                iVar2 = string_1040_8520(
                    uVar5,
                    CONCAT22(param_2, param_3),
                    HWND16_1050_0396,
                    0x20030,
                    0x62a057b,
                );
                uVar4 = uVar5;
            }
            puStack10 = CONCAT22(uVar4, iVar2);
            ppcVar1 = (*puStack10 + 0x74);
            (**ppcVar1)(uVar6, iVar2, uVar4);
            return 0x0;
        }
        PostMessage16(0x0, 0xde, 0x111, HWND16_1050_0396);
    }
    return 0x1;
}

pub unsafe fn destroy_window_1020_1d4a(mut param_1: u32, mut param_2: i16) {
    let mut in_AX: u16;
    let mut BVar1: bool;
    let mut in_EDX: u32;
    let mut uVar2: u16;

    if (param_2 != 0) {
        BVar1 = post_win_msg_1020_1ca4(param_1, in_EDX, in_AX);
        if (BVar1 != 0) {
            uVar2 = (param_1 >> 0x10);
            if ((param_1 + 0x96) != 0) {
                PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
            }
            DestroyWindow16((param_1 + 0x6));
        }
    }
    return;
}

pub unsafe fn invalidate_rect_1020_1fb2(mut param_1: u32, mut param_2: i16) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut local_16: u16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut local_e: [i16; 0x2] = [0; 2];
    let mut iStack10: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar1 + 0x6) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    GetWindowRect16(CONCAT22(0x1050, local_e), (iVar1 + 0x4));
    uStack6 = 0x46;
    uStack20 = 0x46;
    iStack18 = iStack10 - local_e[0];
    uStack4 = 0x5f;
    uStack16 = 0x5f;
    local_16 = (iVar1 + 0x4);
    InvalidateRect16(0x0, &local_16, &DAT_1050_1050);
    return;
}

