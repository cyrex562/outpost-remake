use crate::defines::{COLORREF, LRESULT, RECT16, WPARAM16};

pub fn msg_box_1000_1f24(param_1: i32, param_2: u16) -> u16 {
    let piVar1: *mut i32;
    let mut in_AX: i32;

    if (in_AX < (param_1 + 0xc)) {
        msg_box_1000_214c(0, 0, 0xd940, &PTR_LOOP_1050_1040);
        return 1;
    }
    piVar1 = (param_1 + 0xc);
    unsafe {
        *piVar1 = *piVar1 + 1;
    }
    return 0;
}

// WARNING: Removing unreachable block (ram,0x10001f92)
pub fn out_of_mem_msg_box_1000_1f7e(param_1: &[u32]) -> u16 {
    // let mut uVar1: i32;
    let mut uVar1: u32;
    // let mut cVar2: u8;
    let mut cVar2: i8;
    // let mut uvar3: u16;
    let mut uVar3: u16;
    // let mut uVar4: u32;
    let mut uVar4: u32;

    uVar1 = *param_1;
    if (uVar1 == 0xf) {
        // LAB_1000_1fb6:
        uVar3 = 1;
    } else {
        if (uVar1 < 0x10) {
            cVar2 = uVar1;
            if (cVar2 == 0x2) {}
            // goto LAB_1000_1fb6;
            if ((0 < (cVar2 + -2)) && ((cVar2 + -3) < 0xc)) {
                uVar3 = 0;
                // goto LAB_1000_1fbe;
            }
        }
        uVar3 = 0;
    }
    // LAB_1000_1fbe:
    uVar4 = out_of_mem_msg_1000_1fd2();
    uVar3 = msg_box_1000_214c(0, uVar3, uVar4);
    return uVar3;
}

// : *mut char out_of_mem_msg_1000_1fd2()
pub fn out_of_mem_msg_1000_1fd2() -> String {
    // let mut in_AX: i32;
    let mut in_AX: i32;

    if (in_AX == 2) {
        return "Out of memory.  Please free some memory, then choose retry.".to_string();
    }
    return CONCAT22(0x1000, (s_242_flc_1050_1c76 + 4) + in_AX * 0x17);
}

pub fn msg_box_1000_214c(param_1: u16_00, param_4: i32, param_2: u16, param_3: u16) -> u16 {
    let iVar1: u16;
    let mut iVar2: i32;
    let mut _type: i32;

    _type = 2 - (param_1 == 0) | 0x2110;
    MessageBeep16(0);
    loop {
        iVar1 = MessageBox16(_type, "SmartHeap Library", CONCAT22(param_3, param_2), 0);
        iVar2 = iVar1 + -1;
        if (iVar2 == 0) {
            return 0;
        }
        if ((0 < iVar2) && (!SBORROW2(iVar2, 1))) {
            if (iVar1 == 3 || iVar1 + -2 < 1) {
                fatal_app_exit_1000_3e9e();
                return 0;
            }
            if (iVar1 == 4) {
                return 1;
            }
            if (iVar1 == 5) {
                return 0;
            }
        }
        if ((_type & 0x2000) == 0) {
            return 0;
        }
        _type = _type & 0xdfef | 0x1010;
    }
}

pub fn gui_fn_1008_2c4e(param_1: *mut astruct_16, param_2: u16, param_3: u16) {
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut cursor: u16;
    let h_cursor: HCURSOR16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_6: u16;

    cursor = LoadCursor16(0x7f02, 0);
    h_cursor = SetCursor16(cursor);
    piVar1 = &param_1.field_0xf2;
    unsafe {
        *piVar1 = *piVar1 + 1;
    }
    if (&param_1.field_0xee != 0) {
        uVar5 = &param_1.field_0xee;
        ppcVar3 = (&param_1.field_0xee + 0x90);
        (**ppcVar3)(offset, uVar5, (uVar5 >> 0x10));
    }
    uVar5 = big_fn_1008_15d4(CONCAT22(param_2, param_1), param_3);
    uVar4 = (uVar5 >> 0x10);
    &param_1.field_0xee = uVar5;
    param_1.field_0xf0 = uVar4;
    ppcVar3 = (&param_1.field_0xee + 8);
    (**ppcVar3)(offset, &param_1.field_0xee, uVar4);
    if (param_1.field_0xe8 != 0) {
        uVar2 = param_1.field_0xe8;
        ppcVar3 = (param_1.field_0xe8 + 0xc);
        (**ppcVar3)(offset, uVar2, (uVar2 >> 0x10), 0);
    }
    show_window_1038_b634(_g_astruct_112_a, (_g_astruct_112_a >> 0x10));
    uVar2 = param_1.field_0xf4;
    show_window_1010_7a76(uVar2, (uVar2 >> 0x10));
    uVar5 = &param_1.field_0xee;
    ppcVar3 = (&param_1.field_0xee + 0xc);
    (**ppcVar3)(0x1010, uVar5, (uVar5 >> 0x10), 5);
    uVar5 = &param_1.field_0xee;
    BringWindowToTop16((uVar5 + 8));
    SetCursor16(h_cursor);
    return;
}

pub fn set_cursor_1008_2dcc(param_1: *mut astruct_16, param_2: u16_00, param_3: u16) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let HVar3: HCURSOR16;
    let h_cursor: HCURSOR16;
    let mut extraout_DX: u16;
    let mut local_6: u16;

    HVar3 = LoadCursor16(0x7f02, 0);
    h_cursor = SetCursor16(HVar3);
    HVar3 = h_cursor;
    if (param_1.field_0xe8 != 0) {
        uVar1 = param_1.field_0xe8;
        ppcVar2 = (param_1.field_0xe8 + 0x90);
        ppcVar2(offset, uVar1, (uVar1 >> 0x10));
    }
    big_fn_1008_15d4(CONCAT22(param_2_00, param_1), param_2);
    &param_1.field_0xe8 = HVar3;
    (&param_1.field_0xe8 + 2) = extraout_DX;
    uVar1 = param_1.field_0xe8;
    if ((uVar1 + 0xe0) == 0) {
        uVar1 = param_1.field_0xe8;
        ppcVar2 = (param_1.field_0xe8 + 8);
        ppcVar2(offset, uVar1, (uVar1 >> 0x10));
        uVar1 = param_1.field_0xe8;
        ppcVar2 = (param_1.field_0xe8 + 0xc);
        ppcVar2(offset, uVar1, (uVar1 >> 0x10), 3);
        &param_1.field_0xce = param_1.field_0xe8;
    } else {
        param_1.field_0xe8 = 0;
        gui_fn_1008_2c4e(param_1, param_2_00, param_2);
        &param_1.field_0xce = 0;
    }
    SetCursor16(h_cursor);
    return;
}

pub fn set_cursor_1008_2e9a(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut uVar3: i32;
    let mut extraout_DX: i32;
    let mut iVar4: i32;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let uVar5: u8;
    let uVar6: u8;
    let mut uVar7: u16;
    let mut in_stack_0000fdd2: u8;
    let mut local_224: [u8; 264];
    let mut local_11c: u16;
    let mut local_11a: u32;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u32;
    let mut local_10a: u32;
    let mut local_106: u16;
    let mut local_104: u16;
    let local_102: u8;
    let local_101: u8;

    local_102 = '\0';
    _local_106 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    uVar2 = (_local_106 >> 0x10);
    iVar4 = _local_106;
    local_10a = (iVar4 + 0x16);
    uVar3 = (iVar4 + 0x18) | local_10a;
    uVar5 = unaff_SS;
    uVar6 = (unaff_SS >> 8);
    uVar7 = (param_1 >> 0x10);
    if (uVar3 == 0) {
        open_save_fn_1008_3178(param_1, uVar7, 1);
        local_10a = CONCAT22(extraout_DX, uVar3);
        if ((extraout_DX | uVar3) == 0) {
            PostMessage16(0, 0x13d, 0x111, g_h_window);
            return;
        }
        copy_string_1000_3d3e(
            CONCAT13(uVar6, CONCAT12(uVar5, &local_102)),
            CONCAT22(extraout_DX, uVar3),
        );
        process_string_1000_4d58(&local_102);
        if (in_stack_0000fdd2 != '\0') {
            process_string_1000_3cea(
                CONCAT13(uVar6, CONCAT12(uVar5, local_224)),
                CONCAT22(unaff_SS, &stack0xfdd2),
            );
        }
        pass1_1010_5f1e(_local_106, CONCAT22(unaff_SS, local_224));
    } else {
        local_11a = *(iVar4 + 0x1a);
        copy_string_1000_3d3e(CONCAT13(uVar6, CONCAT12(uVar5, &local_102)), local_11a);
        local_11c = get_string_index_1000_3da4(CONCAT22(unaff_SS, &local_102));
        if (*(&local_104 + local_11c + 1) != '\\') {
            (&local_102)[local_11c] = '\\';
            (&local_101)[local_11c] = '\0';
        }
        process_string_1000_3cea(CONCAT13(uVar6, CONCAT12(uVar5, &local_102)), local_10a);
    }
    if (local_102 != '\0') {
        iVar4 = param_1;
        local_10e = (iVar4 + 0xe8);
        send_win_msg_1020_097e(local_10e, (local_10e >> 0x10));
        uVar1 = (iVar4 + 0xe8);
        UpdateWindow16((uVar1 + 8));
        local_110 = LoadCursor16(0x7f02, 0);
        local_112 = SetCursor16(local_110);
        win_fn_1008_1414(iVar4, uVar7);
        SetCursor16(local_112);
    }
    return;
}

pub fn open_save_1008_30cc(param_1: u32) {
    let mut local_SS__1: i32;
    let in_string_2: *mut char;
    let mut local_DXAX_125: u32;
    let mut local_218: i32;
    let mut local_216: i32;
    let mut local_214: u16;
    let mut local_212: u16;
    let mut string_1: [u8; 10];
    let mut string_2: [u8; 256];
    let mut local_106: u16;
    let mut local_104: u16;
    let mut string_3: [u8; 256];

    string_3[0] = '\0';
    in_string_2 = open_save_fn_1008_3178(param_1, 2);
    if (in_string_2 != 0x0) {
        copy_string_1000_3d3e(CONCAT22(local_SS__1, string_3), in_string_2);
        process_string_1000_4d58(string_3);
        if (string_1[0] != '\0') {
            process_string_1000_3cea(
                CONCAT22(local_SS__1, string_2),
                CONCAT22(local_SS__1, string_1),
            );
        }
        local_DXAX_125 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(string_2, 2));
        pass1_1010_5f4c(local_DXAX_125, CONCAT22(local_SS__1, string_2));
        if (string_3[0] != '\0') {
            win_fn_1008_12dc(param_1, string_3, local_SS__1);
        }
    }
    return;
}

pub fn open_save_fn_1008_3178(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let puVar3: *mut u8;
    let mut iVar4: i32;
    let puVar5: *mut u32;
    let mut uVar6: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut uVar7: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: u16;
    let mut uVar8: u16;
    let mut extraout_DX_04: u16;
    let mut extraout_DX_05: u16;
    let mut unaff_SI: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut uVar10: i32;
    let mut local_786: u32;
    let mut local_782: [u8; 260];
    let mut local_67e: u16;
    let mut local_676: u16;
    let mut local_674: u16;
    let mut local_672: u16;
    let mut local_670: u16;
    let mut local_66e: u32;
    let local_666: u8;
    let mut local_566: u32;
    let mut local_562: u32;
    let mut local_55e: u16;
    let mut local_55a: u16;
    let mut local_558: u16;
    let mut local_54a: u16;
    let mut local_548: u16;
    let mut local_546: u32;
    let mut local_542: u16;
    let mut local_540: u16;
    let mut local_53e: u32;
    let mut local_53a: u16;
    let mut local_538: u16;
    let mut local_536: u16;
    let mut local_534: u16;
    let mut local_532: u32;
    let mut local_52e: u16;
    let mut local_52a: u16;
    let mut local_528: u16;
    let local_51a: u8;
    let local_519: u8;
    let local_518: u8;
    let mut local_418: u16;
    let local_416: u8;
    let mut local_415: [u8; 7];
    let mut local_40e: u16;
    let mut local_40c: [u8; 258];
    let mut local_30a: u32;
    let mut local_306: u16;
    let mut local_304: u16;
    let local_302: u8;
    let local_202: u8;
    let local_103: u8;
    let local_102: u8;

    local_102 = '\0';
    local_302 = '\0';
    local_202 = '\0';
    _local_306 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    uVar9 = (_local_306 >> 0x10);
    iVar4 = _local_306;
    local_30a = (iVar4 + 0x1a);
    uVar10 = (iVar4 + 0x1c);
    if ((uVar10 | local_30a) == 0) {
        local_66e = (iVar4 + 100);
        uVar10 = (iVar4 + 0x66);
        if ((uVar10 | local_66e) != 0) {
            pass1_1008_5784(
                CONCAT22(unaff_SS, &local_67e),
                local_66e & 0xffff | uVar10 << 0x10,
            );
            puVar2 = &local_67e;
            pass1_1008_5b12(CONCAT22(unaff_SS, puVar2));
            _local_676 = CONCAT22(extraout_DX, puVar2);
            if ((extraout_DX | puVar2) != 0) {
                uVar1 = (puVar2 + 2);
                local_30a._0_2_ = uVar1;
                uVar10 = (uVar1 >> 0x10);
                // goto LAB_1008_3206;
            }
        }
    } else {
        // LAB_1008_3206:
        copy_string_1000_3d3e(CONCAT22(unaff_SS, &local_102), CONCAT22(uVar10, local_30a));
    }
    pass1_fn_1000_5008(local_40c);
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_SS, local_40c));
    if (local_40c[local_40e - 1] == '\\') {
        local_40c[local_40e - 1] = 0;
    }
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_SS, &local_102));
    if ((&local_103)[local_40e] == '\\') {
        (&local_103)[local_40e] = '\0';
    }
    dos3_call_1000_4f2e(&local_102);
    uVar9 = (_local_306 >> 0x10);
    local_30a = (_local_306 + 0x12);
    uVar10 = (_local_306 + 0x14);
    puVar3 = (uVar10 | local_30a);
    if (puVar3 != 0x0) {
        puVar3 = &local_202;
        copy_string_1000_3d3e(
            CONCAT22(unaff_SS, puVar3),
            (local_30a & 0xffff | uVar10 << 0x10),
        );
    }
    local_416 = '\0';
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x579,
    );
    copy_string_1000_3d3e(
        CONCAT22(unaff_SS, &local_416),
        CONCAT22(extraout_DX_00, puVar3),
    );
    local_418 = get_string_index_1000_3da4(CONCAT22(unaff_SS, &local_416));
    uVar6 = local_418;
    local_40e = local_418;
    while (uVar9 = uVar6, -1 < local_418) {
        if ((&local_416)[local_418] == '.') {
            copy_string_1000_3d3e(
                CONCAT22(unaff_SS, &local_67e),
                CONCAT22(unaff_SS, local_415 + local_418),
            );
            uVar6 = ZEXT24(&local_416);
            copy_string_1000_3d3e(
                CONCAT22(unaff_SS, &local_416),
                CONCAT22(unaff_SS, &local_67e),
            );
        }
        local_418 = local_418 - 1;
    }
    local_518 = '\0';
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x74c,
    );
    uVar7 = copy_string_1000_3d3e(
        CONCAT22(unaff_SS, &local_518),
        CONCAT22(extraout_DX_01, uVar9),
    );
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_SS, &local_518));
    local_51a = (&local_519)[local_40e];
    local_418 = 0;
    while ((&local_518)[local_418] != '\0') {
        if ((&local_518)[local_418] == local_51a) {
            (&local_518)[local_418] = '\0';
        }
        local_418 = local_418 + 1;
    }
    pass1_1000_4906(CONCAT22(unaff_SS, &local_562), 0, 0x48);
    local_562 = 0x48;
    uVar9 = (param_1 >> 0x10);
    local_55e = (param_1 + 8);
    local_55a = &local_518;
    _local_54a = CONCAT22(unaff_SS, &local_202);
    local_542 = &local_302;
    local_546 = 0x100;
    local_53e = 0x100;
    local_53a = &local_102;
    local_52a = &local_416;
    local_566 = 0;
    local_666 = '\0';
    iVar4 = param_2 + -1;
    uVar8 = (_g_struct_73_1050_14cc >> 0x10);
    if (iVar4 == 0) {
        local_532 = 0x1804;
        load_string_1010_847e(_g_struct_73_1050_14cc, uVar8, 0x74d);
        copy_string_1000_3d3e(
            CONCAT22(unaff_SS, &local_666),
            CONCAT22(extraout_DX_02, iVar4),
        );
        local_536 = &local_666;
        puVar5 = &local_562;
        GetOpenFileName16(0x1000, puVar5);
    } else {
        param_2 = param_2 + -2;
        if (param_2 != 0) {
            fn_1008_6048(
                s_Unsupported_FileStructType_in_Op_1050_01ca,
                uVar7,
                SUB21(param_2, 0),
            );
            // goto LAB_1008_3461;
        }
        local_532 = 6;
        load_string_1010_847e(_g_struct_73_1050_14cc, uVar8, 0x74e);
        copy_string_1000_3d3e(
            CONCAT22(unaff_SS, &local_666),
            CONCAT22(extraout_DX_05, param_2),
        );
        local_536 = &local_666;
        puVar5 = &local_562;
        GetSaveFileName16(0x1000, puVar5);
    }
    if (puVar5 != 0x0) {
        local_566 = _local_54a;
    }
    // LAB_1008_3461:
    if (local_566 != 0) {
        local_67e = local_52e;
        if (local_52e < 0) {
            load_string_1010_847e(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x3fd,
            );
            _local_676 = CONCAT22(extraout_DX_03, local_52e);
            uVar8 = extraout_DX_03;
            pass1_fn_1008_60e8(local_52e, extraout_DX_03);
            _local_676 = CONCAT22(uVar8, local_52e);
            load_string_1010_847e(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x57b,
            );
            local_670 = extraout_DX_04;
            MessageBox16(
                0x10,
                CONCAT13((extraout_DX_04 >> 8), CONCAT12(extraout_DX_04, local_52e)),
                _local_676,
                (param_1 + 8),
            );
            local_566 = 0;
            local_66e = _local_676;
            error_check_1000_17ce(_local_676);
        } else {
            process_string_1000_3dbe(
                CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, local_782)),
                _local_54a,
                local_52e,
            );
            local_782[local_52e] = '\0';
            if (local_782[0] != '\0') {
                pass1_1010_60cc(_local_306, CONCAT22(unaff_SS, local_782));
            }
        }
    }
    dos3_call_1000_4f2e(local_40c);
    return;
}

pub fn sys_color_func_1008_357e(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let local_BX_221: *mut astruct_67;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let CVar6: COLORREF;
    let mut color_ref: u32;
    let puStack140: *mut u32;
    let mut local_84: u16;
    let mut local_82: u16;
    let mut local_80: u32;
    let mut local_7c: u16;
    let mut local_7a: u16;
    let mut local_78: u16;
    let mut local_76: u16;
    let mut local_74: u16;
    let mut local_72: u16;
    let mut local_70: u32;
    let mut local_6c: u32;
    let mut local_68: u16;
    let mut local_66: u16;
    let mut local_64: u16;
    let mut local_62: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut count: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    count = 0x70004;
    local_28 = 0xf0000;
    local_24 = 0x100014;
    local_20 = 0xd0012;
    local_1c = 0x6000e;
    local_18 = 0x80005;
    local_14 = 0x10011;
    local_10 = 0x30002;
    local_c = 0xa0009;
    local_8 = 0xc000b;
    local_4 = 0x13;
    local_80 = 0;
    local_6c = 0x808080;
    uVar3 = 0x100;
    local_74 = 0;
    local_72 = 0x100;
    local_64 = 0;
    local_62 = 0x100;
    local_60 = 0xffff;
    local_5e = 0;
    local_7c = 2;
    local_7a = 0x100;
    local_78 = 2;
    local_76 = 0x100;
    local_68 = 2;
    local_66 = 0x100;
    local_5c = 2;
    local_5a = 0x100;
    local_58 = 0;
    local_50 = 0xc0c0;
    local_4e = 0;
    local_4c = 0;
    local_48 = 0;
    local_44 = 0;
    local_34 = 0;
    uVar2 = 0x8000;
    local_70 = 0x8000;
    local_54 = 0x8000;
    local_40 = 0x8000;
    local_3c = 0x8000;
    local_38 = 0x8000;
    local_30 = 0x8000;
    uVar5 = (param_1 >> 0x10);
    local_BX_221 = param_1;
    if (&local_BX_221.field_0xf8 == 0) {
        process_struct_1000_179c(0x54, (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21));
        local_BX_221.field_0xf8 = uVar2;
        &local_BX_221.field_0xfa = uVar3;
        local_84 = 0;
        while (local_84 < 0x15) {
            CVar6 = GetSysColor16(&count + local_84 * 2);
            uVar1 = &local_BX_221.field_0xf8;
            uVar3 = (uVar1 >> 0x10);
            iVar4 = uVar1;
            (iVar4 + local_84 * 4) = CVar6;
            (iVar4 + local_84 * 4 + 2) = (CVar6 >> 0x10);
            local_84 = local_84 + 1;
        }
    }
    if (param_2 != 0) {
        CVar6 = GetSysColor16(count);
        if ((CVar6 == local_80) && ((CVar6 >> 0x10) == local_80._2_2_)) {
            return;
        }
    }
    if (PTR_LOOP_1050_0010 == 0x0) {
        local_70 = 0xc0c0c0;
    }
    if (param_2 == 0) {
        color_ref = &local_BX_221.field_0xf8;
    } else {
        color_ref = CONCAT22(unaff_SS, &local_80);
    }
    puStack140 = &count;
    SetSysColors16(color_ref, color_ref._2_2_, puStack140);
    return;
}

pub fn fill_client_window_1008_39ac(in_struct_12: *mut astruct_12) {
    let local_BX_4: *mut astruct_12;
    let unaff_SS: HWND16;
    let mut in_stack_00000006: u16;
    let HStack44: HWND16;
    let mut brush: u16;
    let mut b_result: u16;
    let paint_struct: [u8; 32];

    b_result = BeginPaint(CONCAT22(unaff_SS, paint_struct), in_struct_12.h_window);
    brush = CreateSolidBrush16(0x210070b);
    GetClientRect16(CONCAT22(unaff_SS, &stack0xffd2), in_struct_12.h_window);
    HStack44 = b_result;
    FillRect16(brush, &stack0xffd2, unaff_SS);
    HStack44 = in_struct_12.h_window;
    EndPaint(paint_struct, unaff_SS);
    DeleteObject16(brush);
    return;
}

pub fn set_di_bits_to_dev_1008_45d6(param_1: *mut astruct_103) {
    let mut bVar1: bool;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut local_6: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 6) == 0) {
        process_struct_1008_47cc(param_1);
    }
    if (((iVar2 + 8) | (iVar2 + 6)) == 0) {
        bVar1 = false;
    } else {
        if (((iVar2 + 0xc) | (iVar2 + 10)) == 0) {
            process_struct_1008_4834(param_1);
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return;
    }
    SetDIBitsToDevice();
    return;
}

pub fn stretch_di_bits_1008_465a(param_1: *mut astruct_103, param_2: HDC16) {
    let x_src: u16;
    let y_src: u16;
    let mut u_var1: u32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let info: *mut BITMAPINFO;
    let mut uVar4: u16;
    let bits: *mut void;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 6) == 0) {
        process_struct_1008_47cc(param_1);
    }
    if (((iVar3 + 8) | (iVar3 + 6)) == 0) {
        bVar2 = false;
    } else {
        if (((iVar3 + 0xc) | (iVar3 + 10)) == 0) {
            process_struct_1008_4834(param_1);
        }
        bVar2 = true;
    }
    if (!bVar2) {
        return;
    }
    uVar1 = (iVar3 + 0x10);
    bits = (uVar1 >> 0x10);
    info = uVar1;
    x_src = &(info.bmi_header).bi_width;
    y_src = &(info.bmi_header).bi_height;
    uVar1 = (iVar3 + 0x14);
    StretchDIBits16(
        0xcc0020,
        0,
        info,
        bits,
        uVar1,
        (uVar1 >> 0x10),
        y_src,
        x_src,
        0,
        0,
        y_src,
        x_src,
        param_2,
    );
    return;
}

pub fn set_win_1008_5634(
    param_1: *mut u32,
    param_2: u16,
    param_3_00: WPARAM16,
    param_3: u16,
    uparam_4: i32,
) {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let lVar2: u32;
    let mut uvar3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = SUB42(&g_alloc_addr_1050_1050, 0);
    lVar2 = param_4 << 0x10;
    _local_6 = GetWindowLong16(0, param_4);
    if (((_local_6 >> 0x10) | _local_6) == 0) {
        if (param_3 != 1) {
            DefWindowProc16(CONCAT22(param_2, param_1), param_3_00, param_3, param_4);
            return;
        }
        unsafe {
            _local_6 = *param_1;
        }
        lVar2 = param_4 << 0x10;
        SetWindowLong16(_local_6, (_local_6 >> 0x10));
        pass1_1008_9628(_local_6, param_4);
    }
    ppcVar1 = (*_local_6 + 0x1c);
    (**ppcVar1)(
        offset,
        _local_6,
        (_local_6 >> 0x10),
        param_1,
        param_2,
        param_3_00,
        param_3,
        lVar2,
        uVar3,
    );
    return;
}

pub fn cleanup_palette_1008_56e2(param_1: u32, param_2: *mut HDC16) -> u16 {
    let h_gdi_obj: HPALETTE16;
    let mut uVar1: u16;
    uVar1 = (param_1 >> 0x10);
    h_gdi_obj = SelectPalette16(0, (param_1 + 4), unsafe { *param_2 });
    (param_1 + 4) = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    return 1;
}

pub fn create_win_1008_5e7e() -> u16 {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let BVar3: bool;
    let AVar4: ATOM;
    let mut window_handle: u16;
    let mut iVar5: i32;
    let mut local_ESI_9: string;
    let puVar6: *mut u32;
    let unaff_SS: HINSTANCE16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut stock_obj: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let local_12: [u32; 4];

    puVar6 = local_12;
    local_ESI_9._0_2_ = s_MciSoundWindow_1050_02bd;
    iVar5 = 3;
    while (iVar5 != 0) {
        iVar5 = iVar5 + -1;
        puVar2 = puVar6;
        puVar6 = puVar6 + 1;
        puVar1 = local_ESI_9;
        local_ESI_9._0_2_ = local_ESI_9 + 1;
        unsafe {
            *puVar2 = *puVar1;
        }
    }
    puVar6 = local_ESI_9;
    *(puVar6 + 2) = *(local_ESI_9 + 2);
    local_2c = 0x2000;
    local_2a = &u16_1050_5f44;
    local_28 = &PTR_LOOP_1050_1008;
    local_24 = 2;
    local_22 = g_h_instance_1050_038c;
    local_20 = 0;
    local_1e = 0;
    local_26 = 0;
    stock_obj = GetStockObject16(0);
    local_1a = 0;
    local_16 = local_12;
    BVar3 = GetClassInfo16(&local_2c, CONCAT22(local_16, unaff_SS), unaff_SS);
    if (BVar3 == 0) {
        AVar4 = RegisterClass16(&local_2c);
        if (AVar4 == 0) {
            OutputDebugString16(s_MciSound_registerClass_failed_1050_02cc);
            return 0;
        }
    }
    window_handle = CreateWindow16(
        0,
        g_h_instance_1050_038c,
        0,
        g_h_window,
        1,
        1,
        -0x8000,
        -0x8000,
        0xcf0000,
        s_MciSound_registerClass_failed_1050_02cc + 0x1e,
        CONCAT22(unaff_SS, local_12),
    );
    return window_handle;
}

pub fn load_cursor_1008_61b2(
    in_astruct_65_ptr: *mut astruct_65,
    in_u16_2: u16,
    param_3: u16,
    param_4: *mut void,
) -> *mut astruct_65 {
    let mut uVar1: u16;
    let HVar2: HGDIOBJ16;
    let HVar3: HCURSOR16;
    let local_struct_65_ptr_1: *mut astruct_65;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000ffe8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_687a(in_astruct_65_ptr, param_4);
    uVar4 = (in_astruct_65_ptr >> 0x10);
    local_struct_65_ptr_1 = in_astruct_65_ptr;
    local_struct_65_ptr_1.u16_xde = in_u16_2;
    local_struct_65_ptr_1.u16_xe0 = 0;
    in_astruct_65_ptr.ptr_a_lo = 0x6378;
    local_struct_65_ptr_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (in_astruct_65_ptr & 0xffff0000 | ZEXT24(&local_struct_65_ptr_1.char_ptr_x5b)),
        s_DanBrotherton_1050_0302,
    );
    HVar2 = GetStockObject16(4);
    local_struct_65_ptr_1.h_gdi_obj_xc6 = HVar2;
    HVar3 = LoadCursor16(0x7f00, 0);
    local_struct_65_ptr_1.h_cursor_xc4 = HVar3;
    local_struct_65_ptr_1.u16_xc8 = (s_572_bmp_1050_2007 + 4);
    local_struct_65_ptr_1.u32_xac = 0x45000000;
    local_struct_65_ptr_1.astruct_590_ptr_xbc = (param_4 + 8);
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe8, 0x48));
    uVar1 = (ppVar5 >> 0x10);
    local_struct_65_ptr_1.u16_xb4 = 0;
    local_struct_65_ptr_1.u16_xb6 = 0;
    local_struct_65_ptr_1.u16_xb8 = (ppVar5 + 10);
    local_struct_65_ptr_1.u16_xba = (ppVar5 + 0xc);
    &local_struct_65_ptr_1.u32_xca = param_3;
    reg_class_1008_96d2(in_astruct_65_ptr, in_stack_0000ffe8);
    return in_astruct_65_ptr;
}

pub fn destroy_win_1008_628e(param_1: *mut astruct_594, param_2: u16) {
    let ppcVar1: fn();

    ppcVar1 = ((param_1 + 0xd2) + 0x14);
    (**ppcVar1)();
    DestroyWindow16((param_1 + 8));
    (param_1 + 8) = 0;
    return;
}

pub fn fill_rect_1008_62c0(param_1: u32) {
    let mut uVar1: u16;
    let unaff_SS: HWND16;
    let local_2e: RECT16;
    let mut local_26: u16;
    let mut local_24: u16;
    let local_22: PAINTSTRUCT16;

    uVar1 = (param_1 >> 0x10);
    local_24 = BeginPaint16(CONCAT22(unaff_SS, &local_22), (param_1 + 8));
    local_26 = CreateSolidBrush16(0x210070b);
    GetClientRect16(CONCAT22(unaff_SS, &local_2e), (param_1 + 8));
    FillRect16(local_26, &local_2e, unaff_SS);
    EndPaint(&local_22, unaff_SS);
    DeleteObject16(local_26);
    return;
}

pub fn show_window_1008_68c6(param_1: *mut astruct_628, param_2: u16, param_3: u16) -> u16 {
    let mut local_AX_13: u16;
    let mut local_4: u16;

    local_AX_13 = show_window_1008_96ae(CONCAT22(param_2, param_1), param_3);
    pass1_1008_6a04(param_1, param_2);
    return local_AX_13;
}

pub fn load_cursor_1008_7f62(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: u32,
) -> *mut astruct_65 {
    let HVar1: HGDIOBJ16;
    let HVar2: HCURSOR16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut in_stack_0000fffc: u16;

    pass1_1008_687a(param_1, param_3);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    (iVar3 + 0xde) = param_2;
    param_1.ptr_a_lo = 0x8042;
    (iVar3 + 2) = &PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (iVar3 + 0x5b)),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(5);
    (iVar3 + 0xc6) = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (iVar3 + 0xc4) = HVar2;
    (iVar3 + 200) = (s_572_bmp_1050_2007 + 1);
    (iVar3 + 0xac) = 0x44000000;
    (iVar3 + 0xbc) = (param_3 + 8);
    (iVar3 + 0xca) = (iVar3 + 0xde);
    reg_class_1008_96d2(param_1, in_stack_0000fffc);
    return param_1;
}

pub fn load_cursor_1008_80ee(param_1: *mut u16) -> *mut u16 {
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let mut iVar3: i32;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    (iVar3 + 2) = &PTR_LOOP_1050_1008;
    (iVar3 + 0x54) = 0;
    (iVar3 + 0x56) = 0;
    (iVar3 + 0x58) = 0;
    unsafe {
        *param_1 = 0x87c8;
    }
    (iVar3 + 2) = &PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (iVar3 + 4)),
        s_MicroSpinControl_1050_0370,
    );
    (iVar3 + 0x54) = 3;
    HVar1 = LoadCursor16(0x7f00, 0);
    (iVar3 + 0x58) = HVar1;
    HVar2 = GetStockObject16(4);
    (iVar3 + 0x56) = HVar2;
    reg_class_1008_818c(iVar3, uVar4);
    return param_1;
}

pub fn set_window_text_1008_9664(param_1: u32, param_2: u16, param_3: *mut char) {
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 10)), param_3);
    SetWindowText16(param_1 & 0xffff0000 | (param_1 + 10), (param_1 + 8));
    return;
}

pub fn destroy_wiin_1008_9698(param_1: u32) {
    DestroyWindow16((param_1 + 8));
    return;
}

pub fn show_window_1008_96ae(param_1: u32, param_2: u16) -> bool {
    let BVar1: bool;
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    if ((param_1 + 8) != 0) {
        BVar1 = ShowWindow16(param_2, (param_1 + 8));
        return BVar1;
    }
    return 0;
}

pub fn gui_get_info_func_1008_da12(param_1: *mut astruct_61, param_2: u32) {
    let puVar1: *mut u16;
    let mut bVar2: u8;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let hdc: HDC16;
    let iVar5: u16;
    let mut uVar6: i32;
    let local_AX_193: *mut astruct_62;
    let paVar7: *mut astruct_94;
    let paVar8: *mut astruct_94;
    let mut count: u16;
    let extraout_DX: *mut u16;
    let puVar9: *mut u16;
    let mut iVar10: i32;
    let mut uVar11: u16;
    let mut local_20: u32;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar11 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar11, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xc = 0;
    zero_list_1008_3e38(CONCAT13(
        (param_2 >> 8),
        CONCAT12(param_2, &param_1.field_0xe),
    ));
    param_1.field_0x14 = 0;
    param_1.field_0x16 = 0;
    &param_1.field_0x18 = 0;
    CONCAT22(uVar11, param_1) = 0xdc80;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    hdc = GetDC16(0);
    iVar5 = GetDeviceCaps16(8, hdc);
    param_1.field_0xa = iVar5;
    iVar5 = GetDeviceCaps16(10, hdc);
    param_1.field_0xc = iVar5;
    pass1_1008_3e76(
        CONCAT22(uVar11, &param_1.field_0xe),
        0,
        (param_1.field_0xc + -0x1e0) / 2,
        (param_1.field_0xa + -0x280) / 2,
    );
    puVar9 = extraout_DX;
    uVar6 = GetDeviceCaps16(0x26, hdc);
    if ((uVar6 & 0x100) != 0) {
        iVar5 = GetDeviceCaps16(0x68, hdc);
        param_1.field_0x14 = iVar5;
        local_AX_193 = GetDeviceCaps16(0x6a, hdc);
        param_1.field_0x16 = local_AX_193;
        if (__g_astruct_94_ptr_1 == 0) {
            paVar7 = ((local_AX_193 + 1) * 4);
            struct_fn_1000_160a();
        } else {
            puVar9 = g_u16_ptr_1050_5f2e;
            paVar7 = _g_astruct_94_ptr_1;
        }
        alloc_mem_1000_1708(((local_AX_193 + 1) * 4), 0, 1, paVar7, puVar9);
        _local_8 = CONCAT22(puVar9, paVar7);
        paVar8 = ((param_1.field_0x16 + 1) * 4);
        if (__g_astruct_94_ptr_1 == 0) {
            g_u16_ptr_1050_5f2e = puVar9;
            _g_astruct_94_ptr_1 = paVar8;
            struct_fn_1000_160a();
        } else {
        }
        alloc_mem_1000_1708(paVar8, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
        param_1.field_0x18 = _g_astruct_94_ptr_1;
        param_1.field_0x1a = g_u16_ptr_1050_5f2e;
        if (_local_8 != 0x0) {
            if (&param_1.field_0x18 != 0) {
                count = param_1.field_0x16 / 2;
                GetSystemPalette16(CONCAT22(puVar9, paVar7), count, 0, hdc);
                GetSystemPalette16(
                    CONCAT22(puVar9, &paVar7.field_0x0 + count * 2),
                    count,
                    param_1.field_0x14 - count,
                    hdc,
                );
                local_20 = &param_1.field_0x18;
                local_10 = 0;
                while (
                    uVar4 = local_20,
                    puVar1 = &param_1.field_0x16,
                    unsafe { *puVar1 != local_10 } && unsafe { local_10 <= *puVar1 },
                ) {
                    bVar2 = *(&paVar7.field_0x0 + local_10 * 2);
                    uVar3 = local_20 >> 0x10;
                    iVar10 = local_20;
                    local_20 = local_20 & 0xffff0000 | (iVar10 + 4);
                    uVar4 = CONCAT11(
                        *((&paVar7.field_0x0 + local_10 * 2) + 1),
                        *(&paVar7.field_0x0 + local_10 * 2 + 1),
                    );
                    (iVar10 + 2) = bVar2;
                    local_10 = local_10 + 1;
                }
            }
        }
        error_check_1000_17ce(_local_8);
    }
    ReleaseDC16(hdc, 0);
    return;
}

pub fn send_dialog_item_msg_1040_d79c(param_1: u32) {
    let lVar1: u32;
    let mut uVar2: u16;
    let mut extraout_DX: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let LVar5: LRESULT;
    let mut local_10e: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_10e, 3));
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    pass1_1010_c3c2(
        _local_6,
        (_local_6 >> 0x10),
        CONCAT22(unaff_SS, local_106),
        (iVar3 + 0x98),
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_SS, local_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        (iVar3 + 6),
    );
    SendDlgItemMessage16(0, 0, 0xb, (s_dibtext_bmp_1050_1844 + 3), (iVar3 + 6));
    LVar5 = SendDlgItemMessage16(0, 0, 0x405, (s_dibtext_bmp_1050_1844 + 3), (iVar3 + 6));
    uVar2 = LVar5;
    pass1_1010_9044((iVar3 + 0x9c));
    _local_10a = CONCAT22(extraout_DX, uVar2);
    local_10e._0_2_ = 0;
    local_10e._2_2_ = 0;
    while (CONCAT22(local_10e._2_2_, local_10e) < _local_10a) {
        pass1_1010_9130((iVar3 + 0x9c), CONCAT22(unaff_SS, local_106), local_10e);
        if (local_106[0] != '\0') {
            SendDlgItemMessage16(
                CONCAT22(unaff_SS, local_106),
                0,
                0x401,
                (s_dibtext_bmp_1050_1844 + 3),
                (iVar3 + 6),
            );
        }
        lVar1 = CONCAT22(local_10e._2_2_, local_10e) + 1;
        local_10e._0_2_ = lVar1;
        local_10e._2_2_ = (lVar1 >> 0x10);
    }
    SendDlgItemMessage16(0, 1, 0xb, (s_dibtext_bmp_1050_1844 + 3), (iVar3 + 6));
    return;
}

pub fn enable_window_1040_d6be(param_1: u32) {
    let HVar1: HWND16;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    HVar1 = GetDlgItem16(1, (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16(2, (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_vrpal_bmp_1050_183a + 7), (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 4), (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 5), (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 6), (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 7), (iVar2 + 6));
    EnableWindow16(0, HVar1);
    (iVar2 + 0xa0) = 1;
    return;
}

pub fn enable_window_1040_d60e(in_astruct_588_ptr_1: *mut astruct_588) -> u8 {
    let HVar1: HWND16;
    let BVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u16;

    uVar4 = (in_astruct_588_ptr_1 >> 0x10);
    iVar3 = in_astruct_588_ptr_1;
    HVar1 = GetDlgItem16(1, (iVar3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16(2, (iVar3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_vrpal_bmp_1050_183a + 7), (iVar3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 4), (iVar3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 5), (iVar3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 6), (iVar3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 7), (iVar3 + 6));
    BVar2 = EnableWindow16(1, HVar1);
    (iVar3 + 0xa0) = 0;
    return BVar2;
}

pub fn msg_box_1040_d3d0(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x7da,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7db,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7dc,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7dd,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7de,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7df,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    uVar1 = (param_1 >> 0x10);
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x7e0,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7e1,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7e2,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7e3,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7e4,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7e5,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_DX, in_AX));
    return;
}

pub fn send_dialog_item_msg_1040_d20c(in_struct_588_ptr_1: *mut astruct_588, param_2: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_DX: u16;
    let struct_588_ptr_2: *mut astruct_588;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let ppVar3: *mut pass1_struct_1;
    let puVar4: *mut u8;
    let mut uVar5: u16;
    let struct_588_ptr_1: *mut astruct_588;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        enable_window_1040_d60e(in_struct_588_ptr_1);
        return;
    }
    uVar2 = (in_struct_588_ptr_1 >> 0x10);
    struct_588_ptr_2 = in_struct_588_ptr_1;
    if (struct_588_ptr_2.field_0xa0 != 0) {
        pass1_1010_9210(struct_588_ptr_2.field_0x9c);
        enable_window_1040_d60e(in_struct_588_ptr_1);
        ppVar1 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), param_2);
        puVar4 = local_106;
        uVar5 = unaff_SS;
        local_6 = ppVar1;
        ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar4, 3));
        pass1_1010_c3c2(
            ppVar3,
            (ppVar3 >> 0x10),
            CONCAT22(uVar5, puVar4),
            CONCAT22(in_DX, ppVar1),
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_SS, local_106),
            0,
            0x401,
            (s_dibtext_bmp_1050_1844 + 3),
            struct_588_ptr_2.field_0x6,
        );
    }
    return;
}

pub fn send_dlg_item_msg_1040_d29c(param_1: u32) {
    send_dialog_item_msg_1040_d79c(param_1);
    return;
}

pub fn enable_window_1040_cf1c(in_astruct_123: *mut astruct_123) -> LRESULT {
    let h_wnd: HWND16;
    let local_astruct_123: *mut astruct_123;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut uVar3: u32;
    let LVar4: LRESULT;
    let enable: bool;
    let mut uVar5: u16;
    let mut buffer_50c: [u8; 1026];
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut buffer_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(buffer_50c._0_2_, 3));
    uVar1 = (in_astruct_123 >> 0x10);
    local_astruct_123 = in_astruct_123;
    pass1_1010_c3c2(
        ppVar2,
        (ppVar2 >> 0x10),
        CONCAT22(unaff_SS, buffer_106),
        local_astruct_123.field_0x94,
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_SS, buffer_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        local_astruct_123.h_wnd_0x6,
    );
    // msg = WM_SETREDRAW
    SendDlgItemMessage16(
        0,
        0,
        0xb,
        (s_vrpal_bmp_1050_183a + 8),
        local_astruct_123.h_wnd_0x6,
    );
    // WM_PSD_ENVSTAMPRECT
    SendDlgItemMessage16(
        0,
        0,
        0x405,
        (s_vrpal_bmp_1050_183a + 8),
        local_astruct_123.h_wnd_0x6,
    );
    uVar5 = SUB42(s_vrpal_bmp_1050_183a + 8, 0);
    uVar3 = pass1_1018_526a(local_astruct_123.field_0x98, local_astruct_123.field_0x94);
    send_dialog_item_msg_1040_ce12(in_astruct_123, uVar3, uVar5);
    // 040c   1036   SB_GETTEXTLENGTHW
    // 040c   1036   TB_ISBUTTONHIDDEN
    LVar4 = SendDlgItemMessage16(
        0,
        0,
        0x40c,
        (s_vrpal_bmp_1050_183a + 8),
        local_astruct_123.h_wnd_0x6,
    );
    if (((LVar4 >> 0x10) != 0 && -1 < LVar4) || (-1 < LVar4 && (LVar4 != 0))) {
        h_wnd = GetDlgItem16(1, local_astruct_123.h_wnd_0x6);
        enable = 1;
    } else {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_SS, buffer_50c),
            0x44a,
        );
        // 0401   1025   DDM_DRAW
        // 0401   1025   HKM_SETHOTKEY
        // 0401   1025   TB_ENABLEBUTTON
        // 0401   1025   WM_CHOOSEFONT_GETLOGFONT
        // 0401   1025   WM_PSD_FULLPAGERECT
        SendDlgItemMessage16(
            CONCAT22(unaff_SS, buffer_50c),
            0,
            0x401,
            (s_vrpal_bmp_1050_183a + 8),
            local_astruct_123.h_wnd_0x6,
        );
        h_wnd = GetDlgItem16(1, local_astruct_123.h_wnd_0x6);
        enable = 0;
    }
    EnableWindow16(enable, h_wnd);
    // WM_SETREDRAW
    LVar4 = SendDlgItemMessage16(
        0,
        1,
        0xb,
        (s_vrpal_bmp_1050_183a + 8),
        local_astruct_123.h_wnd_0x6,
    );
    return LVar4;
}

pub fn enable_window_1040_cc66(param_1: *mut astruct_123) {
    let ppcVar1: fn();

    ppcVar1 = ((param_1 + 0x98) + 0x10);
    (**ppcVar1)();
    enable_window_1040_cf1c(param_1);
    return;
}

pub fn win_gui_fn_1040_cc8c(param_1: *mut astruct_124, param_2: u16, param_3: u16_00, param4: u32) {
    if (param_3._2_2_ == 0xeb) {
        enable_window_1040_cf1c(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ == (s_vrpal_bmp_1050_183a + 7)) {
            display_msg_box_1040_cce4(CONCAT22(param_2, param_1));
        } else {
            if (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 8)) {
                win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
                return;
            }
            if (param_3 == 1) {
                send_dlg_item_msg_1040_ce76(CONCAT22(param_2, param_1));
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn display_msg_box_1040_cce4(param_1: *mut void) {
    let msg_box_text: *mut char;
    let in_DX: *mut astruct_199;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut msg_box_title: [u8; 258];
    let mut string_3: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, msg_box_title),
        0x57b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, msg_box_text),
        0x7e9,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, string_3),
        0x7ea,
    );
    process_string_1000_3cea(CONCAT22(in_DX, msg_box_text), CONCAT22(unaff_SS, string_3));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, string_3),
        0x7eb,
    );
    process_string_1000_3cea(CONCAT22(in_DX, msg_box_text), CONCAT22(unaff_SS, string_3));
    // type =  MB_OK 0x00000000L The message box contains one push button: OK. This
    // is the default.
    MessageBox16(
        0,
        CONCAT22(unaff_SS, msg_box_title),
        CONCAT22(in_DX, msg_box_text),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_DX, msg_box_text));
    return;
}

pub fn send_dlg_item_int_1040_cdac(
    param_1: u32,
    param_2: u16_00,
    param_3: u16,
    param_4: i32,
) -> u16 {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut bVar3: bool;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_4: u16;

    bVar3 = false;
    iVar4 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        iVar2 = (iVar4 + 0x9e);
        piVar1 = (iVar4 + 0x9c);
        unsafe {
            if (*piVar1 == iVar2 || *piVar1 < iVar2) {}
            // goto LAB_1040_cdef;
            piVar1 = (iVar4 + 0x9e);
            *piVar1 = *piVar1 + 1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1040_cdef;
        if ((iVar4 + 0x9e) < 1) {}
        // goto LAB_1040_cdef;
        piVar1 = (iVar4 + 0x9e);
        unsafe {
            *piVar1 = *piVar1 + -1;
        }
    }
    bVar3 = true;
    // LAB_1040_cdef:
    if (bVar3) {
        (0, *(iVar4 + 0x9e), 0x18e, (iVar4 + 6));
    }
    return 0;
}

// WARNING: Variable defined which should be unmapped: local_10e
// WARNING: Variable defined which should be unmapped: local_10c
// WARNING: Could not reconcile some variable overlaps

pub fn send_dialog_item_msg_1040_ce12(param_1: u32, param_2: u32, param_3: u16) {
    let mut unaff_SS: u16;
    let lVar1: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u32;
    let mut iStack262: i32;
    let HStack260: HWND16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        if (lVar1 == 0) {
            break;
        }
        wsprintf16(
            &local_10a,
            CONCAT22(0x5f12, unaff_SS),
            CONCAT22((lVar1 + 4), 0x1050),
        );
        HStack260 = (param_1 + 6);
        iStack262 = param_3;
        local_10a = 0x4010000;
        _local_10e = CONCAT22(unaff_SS, &local_10a);
        SendDlgItemMessage16(_local_10e, 0, 0x401, param_3, HStack260);
    }
    return;
}

pub fn send_dlg_item_msg_1040_ce76(param_1: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let LVar3: LRESULT;
    let mut uVar4: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, (s_vrpal_bmp_1050_183a + 8), (iVar1 + 6));
    local_6 = LVar3;
    local_4 = local_6 >> 0xf;
    if (local_6 != 0xffff) {
        SendDlgItemMessage16(
            CONCAT22(unaff_SS, local_106),
            local_6,
            0x40a,
            (s_vrpal_bmp_1050_183a + 8),
            (iVar1 + 6),
        );
        uVar4 = pass1_1018_5206((iVar1 + 0x98));
        if (uVar4 != 0) {
            (iVar1 + 0x9c) = (uVar4 + 8);
            (iVar1 + 0x9e) = 0;
            SetDlgItemint16(0, 0, 0x18e, (iVar1 + 6));
            SetDlgItemint16(0, *(iVar1 + 0x9c), 0x191, (iVar1 + 6));
        }
    }
    return;
}

pub fn destroy_win_1040_bf92(param_1: *mut astruct_339) {
    let local_BX_4: *mut astruct_339;
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = 0xc53e;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1040;
    pass1_1010_1ea6(local_BX_4.field_0x6, (param_1 & 0xffff | uVar1 << 0x10));
    destroy_win_1010_2fa0(local_BX_4.field_0x6);
    param_1 = s_0_020_1050_3ab0;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn win_fn_1040_bbe2(param_1: *mut astruct_124, param_2: u16_00, param_3: u16, param_4: u32) {
    let mut uVar1: u16;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut iVar4: i32;
    let BVar5: bool;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let h_wnd: HWND16;
    let mut uVar8: u16;
    let unaff_SS: HWND16;
    let ppVar9: *mut pass1_struct_1;
    let puVar10: *mut u8;
    let uVar11: u8;
    let uVar12: u8;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ != 0x10c) {
        if (param_2._2_2_ < 0x10d) {
            if (param_2._2_2_ == 0xfa) {
                ppcVar2 = (&param_1[1].field_0x4 + 0x18);
                ppcVar2();
                return;
            }
            if (param_2._2_2_ == 0x10a) {
                GetClientRect16(CONCAT22(unaff_SS, &local_a), &param_1.field_0x6);
                uVar3 = &param_1[1].field_0x4;
                local_8 = local_8 + 3;
                local_a = (uVar3 + 0x1a) - 9;
                local_6 = local_6 - 3;
                local_4 = local_4 - 3;
                InvalidateRect16(1, &local_a, unaff_SS);
                destroy_win_1010_2fa0(&param_1[1].field_0x4);
                pass1_1010_32c0(&param_1[1].field_0x4, 0);
                uVar3 = &param_1[1].field_0x4;
                local_22 = (uVar3 >> 0x10);
                pass1_1010_2ee2(uVar3, local_22);
                return;
            }
            if (param_2._2_2_ != 0x10b) {
                // LAB_1040_be78:
                win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                return;
            }
            uVar3 = &param_1[1].field_0x4;
            uVar6 = (uVar3 + 0x12);
            uVar7 = uVar6;
            ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar6, 3));
            uVar8 = (ppVar9 >> 0x10);
            iVar4 = pass1_1010_a5ca(ppVar9, uVar8, uVar7);
            if ((uVar6 != 0x70) && (iVar4 == 0)) {
                return;
            }
            uVar3 = &param_1[1].field_0x1c;
            uVar13 = uVar3;
            uVar14 = (uVar3 >> 0x10);
            uVar3 = &param_1[1].field_0x4;
            uVar1 = (uVar3 + 0x12);
            uVar11 = uVar1;
            uVar12 = (uVar1 >> 8);
        } else {
            if (param_2._2_2_ != 0x10d) {
                if (param_2._2_2_ == 0x10e) {
                    ppVar9 = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(local_22, 0x32),
                    );
                    iVar4 = win_gui_fn_1010_79aa(ppVar9, 0xfc6, &param_1[1].field_0x1c);
                    if (iVar4 != 0) {
                        return;
                    }
                    uVar3 = &param_1[1].field_0x1c;
                    window_msg_func_1010_7300(ppVar9, 0, 0, 0x13, uVar3, (uVar3 >> 0x10));
                    return;
                }
                if (param_2._2_2_ != 0xbbb) {
                    if (param_2._2_2_ == 0xbbc) {
                        ppVar9 = process_struct_1010_20ba(
                            _g_astruct_372_1050_0ed0,
                            CONCAT22(local_22, 3),
                        );
                        uVar8 = (ppVar9 >> 0x10);
                        uVar6 = ppVar9;
                        uVar7 = pass1_1010_a5ac(uVar6, uVar8, &param_1[1].field_0x1c);
                        iVar4 = pass1_1010_a58a(uVar6, uVar8, uVar7);
                        if (iVar4 == 0) {
                            pass1_1010_a568(uVar6, uVar8, uVar7);
                        }
                        h_wnd = GetDlgItem16(0xbbc, &param_1.field_0x6);
                        EnableWindow16(0, h_wnd);
                        return;
                    }
                    // goto LAB_1040_be78;
                }
                if ((&param_1[1].field_0x22 == 0)
                    || (BVar5 = IsWindow16(&param_1[1].field_0x22), BVar5 == 0))
                {
                    puVar10 = pass1_1038_af40(_g_astruct_112_a, *&param_1.field_0x6, 0x1b);
                    &param_1[1].field_0x22 = (puVar10 + 6);
                    ShowWindow16(1, &param_1[1].field_0x22);
                    return;
                }
                local_22 = &param_1[1].field_0x22;
                // goto LAB_1040_bd39;
            }
            ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_22, 3));
            uVar8 = (ppVar9 >> 0x10);
            uVar3 = &param_1[1].field_0x1c;
            uVar13 = uVar3;
            uVar14 = (uVar3 >> 0x10);
            uVar11 = 0x71;
            uVar12 = 0;
        }
        local_1e = ppVar9;
        pass1_1010_a5ec(
            local_1e,
            uVar8,
            CONCAT11(uVar12, uVar11),
            CONCAT22(uVar14, uVar13),
        );
        if ((&param_1[1].field_0x20 != 0)
            && (BVar5 = IsWindow16(&param_1[1].field_0x20), BVar5 != 0))
        {
            SendMessage16(0, 0xeb, 0x111, &param_1[1].field_0x20);
        }
    }
    local_22 = &param_1.field_0x6;
    // LAB_1040_bd39:
    DestroyWindow16(local_22);
    return;
}

pub fn destroy_window_1040_bb78(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let BVar4: bool;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut unaff_CS: u16;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0xb6) != 0) {
        unaff_CS = SUB42(offset, 0);
        BVar4 = IsWindow16((iVar5 + 0xb6));
        if (BVar4 != 0) {
            unaff_CS = SUB42(offset, 0);
            DestroyWindow16((iVar5 + 0xb6));
        }
    }
    (iVar5 + 0xb6) = 0;
    puVar1 = (iVar5 + 0x94);
    uVar2 = (iVar5 + 0x96);
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    (iVar5 + 0x94) = 0;
    (iVar5 + 0x98) = 0;
    return;
}

pub fn set_window_pos_1040_b8d2(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i32;
    let mut extraout_DX: u16;
    let struct_a: *mut astruct_199;
    let paVar7: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let paVar8: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let extraout_DX_02: *mut astruct_199;
    let extraout_DX_03: *mut astruct_199;
    let extraout_DX_04: *mut astruct_199;
    let extraout_DX_05: *mut astruct_199;
    let mut extraout_DX_06: u16;
    let mut uVar9: u16;
    let mut iVar10: i32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let ppVar13: *mut pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe4, 0x31));
    paVar8 = (ppVar13 >> 0x10);
    uVar3 = ppVar13;
    uVar11 = (param_1 >> 0x10);
    iVar10 = param_1;
    (iVar10 + 0x98) = uVar3;
    (iVar10 + 0x9a) = paVar8;
    process_struct_1000_179c(10, paVar8);
    if ((paVar8 | uVar3) == 0) {
        (iVar10 + 0x94) = 0;
    } else {
        process_struct_1040_bf3e(CONCAT22(paVar8, uVar3), (iVar10 + 6));
        (iVar10 + 0x94) = uVar3;
        (iVar10 + 0x96) = extraout_DX;
    }
    pass1_1040_bfde((iVar10 + 0x94), (iVar10 + 0x98));
    paVar8 = struct_a;
    process_struct_1000_179c(0x42, struct_a);
    paVar7 = (paVar8 | uVar3);
    if (paVar7 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar8,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(in_stack_0000ffe4, (iVar10 + 6)),
        );
        paVar7 = extraout_DX_00;
    }
    process_struct_1000_179c(0x42, paVar7);
    paVar8 = (paVar7 | uVar3);
    if (paVar8 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar7,
            1,
            0xa0028,
            0x850000,
            0x10b0084,
            CONCAT22(in_stack_0000ffe4, (iVar10 + 6)),
        );
        paVar8 = extraout_DX_01;
    }
    process_struct_1000_179c(0x42, paVar8);
    paVar7 = (paVar8 | uVar3);
    if (paVar7 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar8,
            1,
            0xa0046,
            0x870000,
            0x10d0086,
            CONCAT22(in_stack_0000ffe4, (iVar10 + 6)),
        );
        paVar7 = extraout_DX_02;
    }
    process_struct_1000_179c(0x42, paVar7);
    paVar8 = (paVar7 | uVar3);
    if (paVar8 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar7,
            1,
            0xa0064,
            0x890000,
            0x10e0088,
            CONCAT22(in_stack_0000ffe4, (iVar10 + 6)),
        );
        paVar8 = extraout_DX_03;
    }
    process_struct_1000_179c(0x42, paVar8);
    paVar7 = (paVar8 | uVar3);
    if (paVar7 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar8,
            1,
            0xa0082,
            0x830000,
            0x10c0082,
            CONCAT22(in_stack_0000ffe4, (iVar10 + 6)),
        );
        paVar7 = extraout_DX_04;
    }
    process_struct_1000_179c(0x42, paVar7);
    paVar8 = (paVar7 | uVar3);
    if (paVar8 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar7,
            1,
            0xa00d2,
            0x8b0000,
            0xbbb008a,
            CONCAT22(in_stack_0000ffe4, (iVar10 + 6)),
        );
        paVar8 = extraout_DX_05;
    }
    process_struct_1000_179c(0x42, paVar8);
    if ((paVar8 | uVar3) == 0) {
        uVar3 = 0;
        uVar12 = 0;
    } else {
        win_fn_1008_3bd6(
            uVar3,
            paVar8,
            1,
            0xa00a0,
            0x8d008e,
            0xbbc008c,
            CONCAT22(in_stack_0000ffe4, (iVar10 + 6)),
        );
        uVar12 = extraout_DX_06;
    }
    ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe4, 3));
    uVar9 = (ppVar13 >> 0x10);
    uVar2 = ppVar13;
    uVar4 = pass1_1010_a5ac(uVar2, uVar9, (iVar10 + 0xb0));
    uVar5 = pass1_1010_ac62(uVar2, uVar9, 0x1e);
    if (uVar5 != 0) {
        pass1_1010_a5ca(uVar2, uVar9, uVar4);
        if (0 < uVar5) {
            pass1_1010_a58a(uVar2, uVar9, uVar4);
            if (uVar5 == 0) {}
            // goto LAB_1040_bb26;
        }
    }
    enable_window_1040_9234(uVar3, uVar12, 0);
    // LAB_1040_bb26:
    uVar1 = (iVar10 + 0x98);
    iVar6 = uVar1;
    uVar1 = uVar1 & 0xffff0000;
    uVar12 = (uVar1 >> 0x10);
    SetWindowPos16(
        0x40,
        (iVar6 + 0x10),
        (iVar6 + 0xe),
        (iVar6 + 0xc),
        (uVar1 | iVar6 + 10),
        0,
        (iVar10 + 6),
    );
    return;
}

pub fn destroy_window_1040_b726(param_1: *mut u32, param_2: i32) {
    let ppcVar1: fn();

    if (param_2 != 0) {
        unsafe {
            ppcVar1 = (*param_1 + 0x78);
        }
        (**ppcVar1)();
    }
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn win_gui_fn_1040_b54a(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16,
    param_2_00: u32,
) {
    let in_struct_1: *mut astruct_44;
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let paVar5: *mut astruct_44;
    let mut extraout_DX: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let ppVar8: *mut pass1_struct_1;
    let uVar9: u8;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut fn_ptr_2: u32;

    if (param_2_00._2_2_ == 0xea) {
        fn_ptr_2 = (CONCAT22(param_2, param_1) + 0x5c);
        (**fn_ptr_2)();
    } else {
        if (param_2_00._2_2_ == 0xeb) {
            ppVar8 =
                process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe6, 3));
            in_struct_1 = &param_1.field_0x90;
            if (in_struct_1 != 0x0) {
                uVar7 = (in_struct_1 >> 0x10);
                uVar10 = 0x1010;
                paVar5 = in_struct_1;
                pass1_1010_ad64(
                    ppVar8,
                    CONCAT22((in_struct_1 + 10), (ppVar8 >> 0x10)),
                    (in_struct_1 + 6),
                );
                param_1.field_0x90 = paVar5;
                &param_1.field_0x92 = extraout_DX;
                if ((extraout_DX | param_1.field_0x90) == 0) {
                    &param_1.field_0x90 = in_struct_1;
                } else {
                    if (in_struct_1 != 0x0) {
                        pass1_1040_a5d0(in_struct_1);
                        uVar10 = 0x1000;
                        error_check_1000_17ce(in_struct_1);
                    }
                    ppcVar1 = (CONCAT22(param_2, param_1) + 0x70);
                    (**ppcVar1)(uVar10, param_1, param_2);
                }
            }
        } else {
            if (param_2_00._2_2_ == 0x1790) {
                ppVar8 = process_struct_1010_20ba(
                    _g_astruct_372_1050_0ed0,
                    CONCAT22(in_stack_0000ffe6, 0x32),
                );
                uVar3 = ppVar8;
                uVar2 = &param_1.field_0x90;
                uVar2 = (uVar2 + 6);
                pass1_1010_7d38(uVar3, (ppVar8 >> 0x10), uVar2, (uVar2 >> 0x10));
                uVar4 = uVar3;
                win_gui_fn_1010_79aa(ppVar8, 0xfab, 0);
                if (uVar4 != 0) {
                    return;
                }
                if (uVar3 == 0) {
                    uVar2 = &param_1.field_0x90;
                    uVar7 = (uVar2 >> 0x10);
                    iVar6 = uVar2;
                    uVar2 = (iVar6 + 6);
                    uVar11 = uVar2;
                    uVar12 = (uVar2 >> 0x10);
                    uVar10 = 0x14;
                } else {
                    uVar2 = &param_1.field_0x90;
                    uVar7 = (uVar2 >> 0x10);
                    iVar6 = uVar2;
                    uVar2 = (iVar6 + 6);
                    uVar11 = uVar2;
                    uVar12 = (uVar2 >> 0x10);
                    uVar10 = 9;
                }
                uVar9 = uVar7;
            } else {
                if (param_2_00._2_2_ == 0x1824) {
                    ppVar8 = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(in_stack_0000ffe6, 0x32),
                    );
                    iVar6 = ppVar8;
                    uVar2 = &param_1.field_0x90;
                    win_gui_fn_1010_79aa(ppVar8, 0xfc5, (uVar2 + 6));
                    if (iVar6 != 0) {
                        return;
                    }
                    uVar2 = &param_1.field_0x90;
                    uVar2 = (uVar2 + 6);
                    uVar11 = uVar2;
                    uVar12 = (uVar2 >> 0x10);
                    uVar10 = 0x12;
                    uVar9 = 0;
                } else {
                    if (param_2_00._2_2_ != 0x1830) {
                        post_win_msg_1040_7b3c(param_1, param_2, param_3, param_2_00);
                        return;
                    }
                    ppVar8 = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(in_stack_0000ffe6, 0x32),
                    );
                    iVar6 = ppVar8;
                    uVar2 = &param_1.field_0x90;
                    win_gui_fn_1010_79aa(ppVar8, 0xfb6, (uVar2 + 6));
                    if (iVar6 != 0) {
                        return;
                    }
                    uVar2 = &param_1.field_0x90;
                    uVar2 = (uVar2 + 6);
                    uVar11 = uVar2;
                    uVar12 = (uVar2 >> 0x10);
                    uVar10 = 0xc;
                    uVar9 = 0;
                }
            }
            window_msg_func_1010_7300(ppVar8, iVar6, uVar9, uVar10, uVar11, uVar12);
        }
    }
    return;
}

pub fn show_win_1040_b43c(param_1: *mut u32) {
    let ppcVar1: fn();

    unsafe {
        ppcVar1 = (*param_1 + 0x70);
    }
    (**ppcVar1)();
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn set_dlg_item_txt_1040_b45e(param_1: u32) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x90) != 0) {
        uVar1 = (iVar3 + 0x90);
        (uVar1 + 0x14) = (iVar3 + 6);
        uVar1 = (iVar3 + 0x90);
        local_6 = (uVar1 + 2);
        local_8 = 0;
        while (
            puVar2 = (iVar3 + 0x90),
            *puVar2 != local_8 && local_8 <= *puVar2,
        ) {
            uVar1 = (local_6 + 2);
            SetDlgItemText16(CONCAT22(uVar1, local_6), (uVar1 >> 0x10), (iVar3 + 6));
            local_8 = local_8 + 1;
            local_6 = local_6 & 0xffff0000 | (local_6 + 10);
        }
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_gui_fn_1040_b4c8(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x90) != 0) {
        ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_6, 0x32));
        uVar1 = (param_1 + 0x90);
        iVar2 = (uVar1 + 10);
        if (iVar2 == 4) {
            iVar2 = win_gui_fn_1010_79aa(ppVar4, 0xfd9, 0);
            if (iVar2 == 0) {
                uVar3 = 0xe;
                // LAB_1040_b50f:
                window_msg_func_1010_7300(ppVar4, iVar2, iVar2, uVar3, iVar2, iVar2);
                return;
            }
        } else {
            if (((0 < iVar2 + -5) && (!SBORROW2(iVar2 + -5, 1))) && (iVar2 + -6 < 2)) {
                iVar2 = win_gui_fn_1010_79aa(ppVar4, 0xfda, 0);
                if (iVar2 == 0) {
                    uVar3 = 0xd;
                    // goto LAB_1040_b50f;
                }
            }
        }
    }
    return;
}

pub fn set_window_pos_1040_b230(param_1: *mut astruct_20) {
    let ppcVar1: fn();
    let iVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let unaff_SS: HWND16;
    let puVar5: *mut u16;
    let uVar6: u8;
    let uVar7: u8;
    let puVar8: *mut u16;
    let HVar9: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    if (PTR_LOOP_1050_5ef8 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        PTR_LOOP_1050_5ef8 = 0x0;
    }
    puVar8 = &local_4;
    puVar5 = &local_6;
    uVar6 = unaff_SS;
    uVar7 = (unaff_SS >> 8);
    HVar9 = unaff_SS;
    _local_a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar5, 0x48));
    pass1_1008_3e94(
        (_local_a + 0xe),
        CONCAT13(uVar7, CONCAT12(uVar6, puVar5)),
        CONCAT22(HVar9, puVar8),
    );
    uVar4 = (_local_a >> 0x10);
    local_c = (_local_a + 10);
    local_e = (_local_a + 0xc);
    iVar2 = GetSystemMetrics16(4);
    iVar3 = iVar2 * PTR_LOOP_1050_5ef8 + 10;
    PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
    _local_12 = CONCAT22(iVar3 + local_4, iVar3 + local_6);
    uVar4 = (param_1 >> 0x10);
    GetWindowRect16(CONCAT22(&local_1a, 0x1538), unaff_SS);
    if (local_e < ((local_14 - local_18) + local_12)) {
        _local_12 = _local_12 & 0xffff0000 | (0xfffe - ((local_14 - local_18) - local_e));
    }
    if (local_c < ((local_16 - local_1a) + local_10)) {
        _local_12 = _local_12 & 0xffff | (0xfffe - ((local_16 - local_1a) - local_c)) << 0x10;
    }
    SetWindowPos16(1, 0, 0, _local_12, (_local_12 >> 0x10), 0, (param_1 + 6));
    ppcVar1 = (param_1 + 0x6c);
    (**ppcVar1)(offset, param_1, uVar4);
    return;
}

pub fn set_dialog_item_txt_1040_ad14(in_struct_1: *mut astruct_347) {
    set_dialog_item_text_1040_ae04(in_struct_1);
    return;
}

pub fn win_gui_fn_1040_ad24(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16_00,
    param_3: u32,
) {
    if (param_3._2_2_ == 0xeb) {
        set_dialog_item_text_1040_ae04(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ != 0x1f0) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        msg_box_1040_ad66(CONCAT22(param_2, param_1));
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn msg_box_1040_ad66(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x7f3,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7f4,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_DX, in_AX));
    return;
}

// WARNING: Variable defined which should be unmapped: local_124
// WARNING: Variable defined which should be unmapped: local_11e
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_dialog_item_text_1040_ae04(param_1: *mut astruct_347) {
    let mut u_var1: u32;
    let mut bVar2: bool;
    let mut uvar3: u16;
    let string_2: *mut char;
    let string_3: *mut char;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let mut local_AX_349: u16;
    let lVar6: u32;
    let local_DL_13: u8;
    let mut local_DX_81: u16;
    let mut local_DX_110: u16;
    let mut extraout_DX: u16;
    let mut local_DX_373: u16;
    let local_struct_1: *mut astruct_347;
    let plVar7: *mut long;
    let mut local_SI__1: u16;
    let mut local_DI__1: u16;
    let mut local_ES_26: u16;
    let mut local_SS__1: u16;
    let ppVar8: *mut pass1_struct_1;
    let mut uVar9: u32;
    let mut uVar10: u32;
    let mut local_124: u32;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut string_1: [u8; 256];
    let mut temp_5f97b0c4d6: u32;

    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_SI__1, 3));
    local_ES_26 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    pass1_1010_c3c2(
        ppVar8,
        (ppVar8 >> 0x10),
        CONCAT22(local_SS__1, string_1),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(local_SS__1, string_1),
        0x1778,
        local_struct_1.window_handle,
    );
    uVar9 = pass1_1030_73a8(local_struct_1.field_0x94);
    string_3 = (uVar9 + 0x20);
    uVar3 = (uVar9 >> 0x10);
    uVar10 = pass1_1030_8326();
    local_DX_110 = (uVar10 >> 0x10);
    uVar9 = 0;
    local_116 = 0;
    bVar2 = false;
    local_118 = 0;
    while (true) {
        local_AX_349 = uVar9;
        if (9 < local_118) {
            break;
        }
        uVar4 = (string_3 + local_118 * 0xc + 2) | (string_3 + local_118 * 0xc);
        uVar9 = uVar4;
        if (uVar4 != 0) {
            plVar7 = (string_3 + local_118 * 0xc);
            uVar5 = big_switch_statement_1020_c222((plVar7 + 1));
            SetDlgItemText16(
                CONCAT22(local_DX_110, uVar5),
                local_116 + 0x1d2,
                local_struct_1.window_handle,
            );
            unsafe {
                lVar6 = *plVar7 - (uVar10 & 0xffff);
            }
            wsprintf16(
                string_1,
                CONCAT13(0x5e, CONCAT12(0xf4, local_SS__1)),
                CONCAT13((lVar6 >> 8), CONCAT12(lVar6, 0x1050)),
            );
            _local_120 = CONCAT22(local_SS__1, string_1);
            SetDlgItemText16(_local_120, local_116 + 0x1dc, local_struct_1.window_handle);
            uVar1 = local_struct_1.field_0x98;
            local_124._0_2_ = uVar1;
            local_124._2_2_ = (uVar1 >> 0x10);
            wsprintf_1010_8c96(
                local_124,
                local_124._2_2_,
                string_1,
                local_SS__1,
                plVar7,
                uVar3,
            );
            uVar9 = ZEXT24(string_1);
            _local_120 = CONCAT22(local_SS__1, string_1);
            local_DX_110 = extraout_DX;
            SetDlgItemText16(_local_120, local_116 + 0x1e6, local_struct_1.window_handle);
            local_116 = CONCAT22(1, local_116 + 1);
            bVar2 = true;
        }
        local_118 = local_118 + 1;
    }
    if (!bVar2) {
        load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        SetDlgItemText16(
            CONCAT22(local_DX_373, local_AX_349),
            0x1d2,
            local_struct_1.window_handle,
        );
    }
    return;
}

pub fn set_win_placement_1010_0070(param_1: u32, param_2: i32, param_3: HWND16) {
    let ppcVar1: fn();
    let mut uVar2: u16;
    let puVar3: *mut u32;
    let mut unaff_SS: u16;
    let lVar4: u32;
    let puVar5: *mut u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_18 = 0x16;
    _local_16 = 0;
    local_12 = 0;
    local_10 = 0;
    local_e = 0;
    local_c = 0;
    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    puVar5 = &local_18;
    GetWindowPlacement16();
    if ((local_10 == 0xffff) || (param_2 != 0)) {
        _local_16 = 0x50001;
        lVar4 = GetWindowLong16(0, param_3);
        uVar2 = (lVar4 >> 0x10);
        puVar3 = (lVar4 + 0xe0);
        unsafe {
            ppcVar1 = (*puVar3 + 0x38);
        }
        (**ppcVar1)(offset, puVar3, (lVar4 + 0xe2), puVar5);
        pass1_1010_01f8(param_1, CONCAT22(unaff_SS, &local_18), puVar3);
        SetWindowPlacement16();
    }
    return;
}

pub fn set_win_placement_1010_010e(param_1: u16_00, param_2: u16, param_1: HWND16) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let puVar5: *mut u32;
    let mut extraout_DX: i32;
    let lVar6: u32;
    let puVar7: *mut u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_18 = 0x16;
    local_16 = 0;
    local_14 = 0;
    local_12 = 0;
    local_10 = 0;
    local_e = 0;
    local_c = 0;
    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    puVar7 = &local_18;
    GetWindowPlacement16();
    if (local_a == 0xffff) {
        lVar6 = GetWindowLong16(0, param_1);
        uVar4 = (lVar6 >> 0x10);
        puVar5 = (lVar6 + 0xe0);
        unsafe {
            ppcVar1 = (*puVar5 + 0x1c);
        }
        (**ppcVar1)(offset, puVar5, (lVar6 + 0xe2), puVar7);
        iVar2 = puVar5;
        puVar3 = (puVar5 & 0xffff | extraout_DX << 0x10);
        local_14 = 9;
        unsafe {
            local_a = *puVar3;
        }
        local_8 = (iVar2 + 2);
        unsafe {
            local_6 = (iVar2 + 4) + *puVar3;
        }
        local_4 = (iVar2 + 2) + (iVar2 + 6);
        SetWindowPlacement16(offset, &local_18);
    }
    return;
}

pub fn enum_child_windows_1010_01be() {
    let unaff_CS: HANDLE16;
    let pvVar1: *mut void;
    let mut local_6: u16;
    let mut local_4: u16;

    if (PTR_LOOP_1050_0010 == 0x0) {
        pvVar1 = MakeProcInstance16(unaff_CS, CONCAT22(0x240, g_h_instance_1050_038c));
        EnumChildWindows16(0, pvVar1, (pvVar1 >> 0x10));
        FreeProcInstance16(CONCAT22(pvVar1, 0x1538));
    }
    return;
}

pub fn win_gui_fn_1010_2a32(param_1: i32, uparam_2_00: i32, param_2: *mut HFILE16) {
    let piVar1: *mut i32;
    let pcVar2: *mut char;
    let pbVar3: *mut byte;
    let left: u16;
    let top: u16;
    let right: u16;
    let bottom: u16;
    let in_struct_104_ptr: *mut astruct_104;
    let mut uVar4: u32;
    let mut entry: string;
    let mut string: string;
    let mut filename: string;
    let mut uVar5: u32;
    let mut bVar6: u8;
    let puVar7: *mut u32;
    let puVar8: *mut u32;
    let ppcVar9: fn();
    let pcVar10: *mut code;
    let h_gdi_obj: HGDIOBJ16;
    let puVar11: *mut u16;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let uVar18: u8;
    let uVar19: u8;
    let uVar20: u8;
    let uVar21: u8;
    let uVar22: u8;
    let uVar23: u8;
    let uVar24: u8;
    let uVar25: u8;
    let local_AX_123: *mut astruct_382;
    let mut uVar26: i32;
    let HVar27: HDC16;
    let puVar28: *mut u8;
    let h_gdi_obj_00: HPALETTE16;
    let puVar29: *mut u32;
    let mut uVar30: u16;
    let BVar31: bool;
    let mut bVar32: u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let struct_a: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let in_BX: *mut u32;
    let mut iVar33: i32;
    let mut unaff_BP: u16;
    let mut unaff_SI: i32;
    let mut iVar34: i32;
    let mut unaff_DI: u16;
    let mut unaff_ES: u16;
    let mut unaff_CS: u16;
    let mut uVar35: u16;
    let mut unaff_SS: u16;
    let mut bVar36: bool;
    let mut bVar37: u8;
    let mut uVar38: u32;
    let mut in_stack_00000000: i32;
    let mut in_stack_00000002: u16;
    let mut local_36: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut iStack32: i32;
    let HStack30: HGDIOBJ16;
    let HStack28: HGDIOBJ16;
    let pcStack26: *mut char;
    let pcStack24: *mut char;
    let mut local_16: u32;
    let uStack18: u8;
    let uStack17: u8;
    let uStack16: u8;
    let uStack15: u8;
    let uStack14: u8;
    let uStack13: u8;
    let uStack12: u8;
    let uStack11: u8;
    let uStack10: u8;
    let uStack9: u8;
    let mut bStack8: u8;
    let uStack7: u8;
    let uStack6: u8;
    let uStack5: u8;
    let mut local_4: u16;
    let uStack2: u8;
    let uStack1: u8;

    iVar33 = param_1;
    uStack9 = 0xfe;
    uVar13 = uStack9;
    uStack6 = 0xfe;
    uVar21 = uStack6;
    uStack2 = unaff_BP;
    uStack1 = (unaff_BP >> 8);
    local_4._0_1_ = 0;
    local_4._1_1_ = 0;
    uStack10 = param_1;
    uVar12 = uStack10;
    uStack9 = (param_1 >> 8);
    uVar15 = uStack9;
    bStack8 = param_2_00;
    uVar18 = bStack8;
    uStack7 = (param_2_00 >> 8);
    uVar17 = uStack7;
    if ((param_2._2_2_ + 0xec76 & 3) != 0) {}
    // goto LAB_1010_2ad8;
    local_AX_123 = (param_2._2_2_ + 0xec76 >> 1);
    bVar36 = local_AX_123 < (s_version__d__d_1050_0012 + 10);
    if ((s_version__d__d_1050_0012 + 10) < local_AX_123) {}
    // goto LAB_1010_2ad8;
    unaff_CS = 0x1010;
    uStack6 = SUB21(in_BX, 0);
    uVar22 = uStack6;
    uStack5 = (in_BX >> 8);
    uVar24 = uStack5;
    uStack6 = unaff_SS;
    uVar23 = uStack6;
    uStack5 = (unaff_SS >> 8);
    uVar25 = uStack5;
    uStack10 = SUB41(param_2, 0);
    uVar14 = uStack10;
    uStack9 = (param_2 >> 8);
    uVar16 = uStack9;
    bStack8 = (param_2 >> 0x10);
    uVar19 = bStack8;
    uStack7 = (param_2 >> 0x18);
    uVar20 = uStack7;
    uStack6 = uVar22;
    uStack5 = uVar24;
    match (local_AX_123) {
        // default:
        // goto switchD_1010_2ab5_caseD_0;
        0x1 | 0x3 | 0xb => {
            local_AX_123.field_0xa = in_BX;
        }
        0x9 | 0xf | 0x15 | 0x1b => {
            local_AX_123.field_0xa = in_BX;
            local_AX_123.field_0x10 = in_BX;
            local_AX_123.field_0xc = in_BX;
            return;
        }
        0x5 => {
            uStack12 = 0x10;
            uStack11 = 0x10;
            uStack14 = 0x35;
            uStack13 = 0x40;
            BVar31 = write_to_file_1008_7e1c(
                param_2,
                ZEXT24(in_BX),
                CONCAT13(
                    (in_stack_00000000 >> 8),
                    CONCAT12(in_stack_00000000, unaff_BP),
                ),
            );
            if (BVar31 != 0) {
                return;
            }
            g_u16_1050_0310 = 0x6d0;
            return;
        }
        0x6 => {
            local_4._0_1_ = 0;
            // goto LAB_1010_2ad8;
        }
        0x7 => {
            uStack6 = 0x10;
            uStack5 = 0x10;
            bStack8 = 0x34;
            uStack7 = 0x48;
            unsafe {
                ppcVar9 = *in_BX;
            }
            (**ppcVar9)();
            iVar33 = param_2 + 0x105;
            uStack6 = iVar33;
            uStack5 = (iVar33 >> 8);
            uStack10 = _g_struct_73_1050_14cc;
            uStack9 = (_g_struct_73_1050_14cc >> 8);
            bStack8 = (_g_struct_73_1050_14cc >> 0x10);
            uStack7 = (_g_struct_73_1050_14cc >> 0x18);
            uStack12 = 0x10;
            uStack11 = 0x10;
            uStack14 = 0x45;
            uStack13 = 0x48;
            win_gui_fn_1010_8170();
            iVar34 = param_2 * 4;
            (param_1 + iVar34 + 0x26) = iVar33;
            (param_1 + iVar34 + 0x28) = extraout_DX;
            uStack6 = 0x50;
            uStack5 = 0x10;
            bStack8 = 0x80;
            uStack7 = 0x13;
            uStack12 = 0;
            uStack11 = 0;
            uStack10 = 0;
            uStack9 = 0;
            uStack16 = 0;
            uStack15 = 0;
            uStack14 = 0;
            uStack13 = 0;
            in_struct_104_ptr = (param_1 + 0x26 + iVar34);
            local_16._2_1_ = SUB41(in_struct_104_ptr, 0);
            local_16._3_1_ = (in_struct_104_ptr >> 8);
            uStack18 = (in_struct_104_ptr >> 0x10);
            uStack17 = (in_struct_104_ptr >> 0x18);
            local_16._0_2_ = 0x1010;
            pcStack24 = &PTR_LOOP_1050_486c;
            uVar38 = process_struct_1008_4772(in_struct_104_ptr);
            uStack18 = uVar38;
            uStack17 = (uVar38 >> 8);
            uStack16 = (uVar38 >> 0x10);
            uStack15 = (uVar38 >> 0x18);
            local_16._0_2_ = &PTR_LOOP_1050_1008;
            pcStack24 = 0x4879;
            local_16._2_1_ = uStack18;
            local_16._3_1_ = uStack17;
            uStack18 = uStack16;
            uStack17 = uStack15;
            HVar27 = CreateDC16(
                uVar38,
                (uVar38 & 0xff000000 | CONCAT12(uStack16, (uVar38 >> 0x10))),
                CONCAT13(uStack11, CONCAT12(uStack12, CONCAT11(uStack13, uStack14))),
                CONCAT13(uStack7, CONCAT12(bStack8, CONCAT11(uStack9, uStack10))),
            );
            local_16._2_1_ = HVar27;
            local_16._3_1_ = (HVar27 >> 8);
            uVar5 = (_PTR_LOOP_1050_4230 + 0xe);
            pcStack24 = uVar5;
            local_16._0_2_ = (uVar5 >> 0x10);
            pcStack26 = (&local_16 + 2);
            bStack8 = pcStack26;
            uStack7 = (pcStack26 >> 8);
            uStack12 = uVar5;
            uStack11 = (uVar5 >> 8);
            uStack10 = (uVar5 >> 0x10);
            uStack9 = (uVar5 >> 0x18);
            uStack14 = 0x38;
            uStack13 = 0x15;
            uStack16 = 0x97;
            uStack15 = 0x48;
            uStack6 = uVar23;
            uStack5 = uVar25;
            realize_palette_1008_4e08();
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = local_4;
            uStack7 = local_4._1_1_;
            uStack10 = 8;
            uStack9 = 0x10;
            uStack12 = 0xa5;
            uStack11 = 0x48;
            HStack28 = SelectObject16(
                CONCAT11(local_4._1_1_, local_4),
                CONCAT11(local_16._3_1_, local_16._2_1_),
            );
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = local_16._2_1_;
            uStack7 = local_16._3_1_;
            uStack10 = 0x38;
            uStack9 = 0x15;
            uVar35 = SUB42(offset, 0);
            uStack12 = 0xb3;
            uStack11 = 0x48;
            HStack30 = SelectObject16(
                CONCAT11(local_16._3_1_, local_16._2_1_),
                CONCAT11(local_16._3_1_, local_16._2_1_),
            );
            iStack32 = 0;
            while (true) {
                uVar13 = uStack15;
                uVar12 = uStack16;
                piVar1 = (param_1 + 0x74);
                uStack16 = uVar35;
                uStack15 = (uVar35 >> 8);
                if (unsafe { *piVar1 == iStack32 } || unsafe { *piVar1 < iStack32 }) {
                    break;
                }
                uStack6 = 8;
                uStack5 = 0;
                iVar33 = (iStack32 * 0x10 + param_2) * 8;
                iVar34 = iVar33 + (param_1 + 0x70);
                uVar35 = (param_1 + 0x72);
                bStack8 = uVar35;
                uStack7 = (uVar35 >> 8);
                uStack10 = iVar34;
                uStack9 = (iVar34 >> 8);
                puVar28 = &uStack14;
                uStack14 = SUB21(puVar28, 0);
                uStack13 = (puVar28 >> 8);
                uVar35 = 0x1000;
                uStack18 = 0xe1;
                uStack17 = 0x48;
                uStack12 = uVar23;
                uStack11 = uVar25;
                pass1_fn_1000_484c(
                    CONCAT13(uVar25, CONCAT12(uVar23, puVar28)),
                    CONCAT13(uStack7, CONCAT12(bStack8, iVar34)),
                    8,
                );
                if (puVar28 != 0x0) {
                    uStack6 = local_16._2_1_;
                    uStack5 = local_16._3_1_;
                    uVar5 = (param_1 + 0x70);
                    uVar35 = (uVar5 >> 0x10);
                    iVar34 = uVar5;
                    left = (iVar34 + iVar33);
                    bStack8 = left;
                    uStack7 = (left >> 8);
                    iVar33 = iVar33 + iVar34;
                    top = (iVar33 + 2);
                    uStack10 = top;
                    uStack9 = (top >> 8);
                    right = (iVar33 + 4);
                    uStack12 = right;
                    uStack11 = (right >> 8);
                    bottom = (iVar33 + 6);
                    uStack14 = bottom;
                    uStack13 = (bottom >> 8);
                    uStack16 = 0;
                    uStack15 = 0x10;
                    uVar35 = SUB42(offset, 0);
                    uStack18 = 8;
                    uStack17 = 0x49;
                    Rectangel16(
                        bottom,
                        right,
                        top,
                        left,
                        CONCAT11(local_16._3_1_, local_16._2_1_),
                    );
                }
                iStack32 = iStack32 + 1;
            }
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = pcStack26;
            uStack7 = (pcStack26 >> 8);
            uStack10 = 0;
            uStack9 = 0;
            uStack14 = 0x24;
            uStack13 = 0x49;
            uStack12 = uStack16;
            uStack16 = uVar12;
            uStack11 = uStack15;
            uStack15 = uVar13;
            h_gdi_obj_00 = SelectPalette16(0, pcStack26, CONCAT11(local_16._3_1_, local_16._2_1_));
            local_4._0_1_ = h_gdi_obj_00;
            local_4._1_1_ = (h_gdi_obj_00 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x2a;
            uStack7 = 0x49;
            DeleteObject16(h_gdi_obj_00);
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = HStack28;
            local_4._1_1_ = (HStack28 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x35;
            uStack7 = 0x49;
            SelectObject16(HStack28, CONCAT11(local_16._3_1_, local_16._2_1_));
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = HStack30;
            local_4._1_1_ = (HStack30 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x40;
            uStack7 = 0x49;
            SelectObject16(HStack30, CONCAT11(local_16._3_1_, local_16._2_1_));
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = 0x38;
            local_4._1_1_ = 0x15;
            uStack6 = 0x48;
            uStack5 = 0x49;
            DeleteDC16(CONCAT11(local_16._3_1_, local_16._2_1_));
            h_gdi_obj = CONCAT11(local_4._1_1_, local_4);
            uStack2 = 0x38;
            uStack1 = 0x15;
            local_4._0_1_ = 0x50;
            local_4._1_1_ = 0x49;
            DeleteObject16(h_gdi_obj);
            return;
        }
        0x8 => {
            local_4._0_1_ = 3;
            // goto LAB_1010_2ad8;
        }
        0xd => {
            pbVar3 = (&local_AX_123.field_0x0 + unaff_SI);
            unsafe {
                bVar37 = *pbVar3;
                bVar6 = *pbVar3 + in_DX;
                *pbVar3 = bVar6 + bVar36;
            }
            puVar7 = (CARRY1(bVar37, in_DX) || CARRY1(bVar6, bVar36));
            puVar8 = in_BX + -0x404;
            bVar37 = in_BX < 0x1010 || puVar8 < puVar7;
            puVar29 = (puVar8 - puVar7);
            pcVar10 = swi(4);
            if (SBORROW2(in_BX, 0x1010) != SBORROW2(puVar8, puVar7)) {
                unsafe {
                    (*pcVar10)();
                }
                in_DX = extraout_DX_00;
            }
            bVar36 = puVar29 < 0x1010 || puVar29 + -0x404 < bVar37;
            pbVar3 = (&local_AX_123.field_0x0 + unaff_SI);
            unsafe {
                bVar37 = *pbVar3;
                bVar32 = in_DX;
                bVar6 = *pbVar3;
                *pbVar3 = bVar6 + bVar32 + bVar36;
                pcVar2 = (&local_AX_123.field_0x0 + unaff_SI);
                *pcVar2 =
                    *pcVar2 + bVar32 + (CARRY1(bVar37, bVar32) || CARRY1(bVar6 + bVar32, bVar36));
            }
            bStack8 = (&uStack2 >> 8);
            uStack12 = in_stack_00000000;
            uStack11 = (in_stack_00000000 >> 8);
            uStack10 = in_stack_00000002;
            uStack13 = uStack1;
            uStack15 = local_4._1_1_;
            uStack14 = uStack2;
            uStack17 = 0x10;
            uStack16 = 0x10;
            local_16._3_1_ = 0x4d;
            uStack18 = 0x50;
            uStack9 = uVar13;
            pass1_1018_4cda(
                CONCAT11(uStack2, local_4._1_1_),
                CONCAT13(uStack10, CONCAT12(uStack11, CONCAT11(uStack12, uStack1))),
            );
            iVar33 = CONCAT11(uStack2, local_4._1_1_);
            uStack12 = in_stack_00000000;
            puVar11 = CONCAT13(uStack12, CONCAT12(uStack1, iVar33));
            unsafe {
                *puVar11 = (s_SinternalPutBldg2_site_0x_08lx__1050_5099 + 1);
            }
            (iVar33 + 2) = 0x1010;
            uStack11 = 0xb3;
            uStack10 = 1;
            uStack13 = uStack1;
            uStack15 = local_4._1_1_;
            uStack14 = uStack2;
            uStack17 = 0x18;
            uStack16 = 0x10;
            local_16._3_1_ = 0x65;
            uStack18 = 0x50;
            pass1_1018_4dce(CONCAT13(uStack12, CONCAT12(uStack1, iVar33)), 0x1b3);
            _PTR_LOOP_1050_4230 = CONCAT13(
                uStack12,
                CONCAT12(uStack1, CONCAT11(uStack2, local_4._1_1_)),
            );
            return;
        }
        0xe => (param_1 + 0x20) = param_2,
        0x11 => {
            loop {
                // WARNING: Do nothing block with infinite loop
            }
        }
        0x12 => {
            PTR_LOOP_1050_10c6 = (0 < param_2);
            PTR_LOOP_1050_1142 = (2 < param_2)
        }
        0x13 => {
            uStack5 = (&uStack2 >> 8);
            iVar33 = param_1 * 8 + in_stack_00000000;
            if (((((iVar33 + 0x22) != 0) || ((iVar33 + 0x24) != 0)) || ((iVar33 + 0x26) != 0))
                || ((iVar33 + 0x28) != 0))
            {
                iVar33 = param_1 * 8 + in_stack_00000000;
                uVar5 = (iVar33 + 0x22);
                uVar4 = (iVar33 + 0x26);
                uStack14 = uVar4;
                uStack13 = (uVar4 >> 8);
                uStack12 = (uVar4 >> 0x10);
                uStack11 = (uVar4 >> 0x18);
                uStack18 = uVar5;
                uStack17 = (uVar5 >> 8);
                uStack16 = (uVar5 >> 0x10);
                uStack15 = (uVar5 >> 0x18);
                local_16._2_1_ = 0x50;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = s__d__d__d__d_1050_14ae;
                uVar4 = (in_stack_00000000 + 0xe);
                pcStack26 = uVar4;
                pcStack24 = (uVar4 >> 0x10);
                HStack28 = 0x1010;
                HStack30 = 0x627c;
                uStack6 = uVar21;
                string_fn_1000_3f9c(
                    pcStack26,
                    pcStack24,
                    s__d__d__d__d_1050_14ae,
                    &g_alloc_addr_1050_1050,
                    uVar5,
                );
                uStack12 = 0x50;
                uStack11 = 0x10;
                uStack14 = 0xb8;
                uStack13 = 0x13;
                entry = (param_1 * 4 + 0x1446);
                uStack18 = SUB41(entry, 0);
                uStack17 = (entry >> 8);
                uStack16 = (entry >> 0x10);
                uStack15 = (entry >> 0x18);
                string = (in_stack_00000000 + 0xe);
                local_16._0_2_ = string;
                local_16._2_1_ = (string >> 0x10);
                local_16._3_1_ = (string >> 0x18);
                filename = (in_stack_00000000 + 10);
                pcStack26 = filename;
                pcStack24 = (filename >> 0x10);
                HStack28 = 0x1000;
                HStack30 = 0x62a0;
                WritePrivateProfileString(filename, string, entry, s_windows_1050_13b8);
            }
            return;
        }
        0x14 => (param_1 + 0x24) = param_2,
        0x17 => {
            struct_a = (in_DX + -1);
            uStack6 = unaff_DI;
            uStack5 = (unaff_DI >> 8);
            pbVar3 = (&local_AX_123.field_0x0 + unaff_SI);
            unsafe {
                *pbVar3 = *pbVar3 | struct_a;
            }
            (param_1 + 0x12) = in_BX;
            (param_1 + 0x14) = struct_a;
            local_2a = 0;
            loop {
                uStack10 = unaff_CS;
                uVar12 = uStack10;
                uStack9 = (unaff_CS >> 8);
                uVar13 = uStack9;
                if (local_36 <= local_2a) {
                    uStack10 = 2;
                    uStack9 = 0;
                    bStack8 = 0;
                    uStack7 = 0;
                    uVar26 = param_1 + 0x1a;
                    uStack14 = uVar26;
                    uStack13 = (uVar26 >> 8);
                    uStack11 = ((param_2_00 << 0x10) >> 0x18);
                    local_16._0_2_ = 0x9f2f;
                    local_16._2_1_ = uVar12;
                    local_16._3_1_ = uVar13;
                    uStack18 = uVar14;
                    uStack17 = uVar16;
                    uStack16 = uVar19;
                    uStack15 = uVar20;
                    uStack12 = uVar18;
                    BVar31 = read_file_1008_7dee(
                        param_2,
                        ((param_2_00 & 0xff00) << 0x10 | CONCAT12(uVar18, uVar26)),
                        2,
                    );
                    if (BVar31 != 0) {
                        uStack10 = 2;
                        uStack9 = 0;
                        bStack8 = 0;
                        uStack7 = 0;
                        uVar26 = param_1 + 0x1c;
                        uStack14 = uVar26;
                        uStack13 = (uVar26 >> 8);
                        uStack11 = ((param_2_00 << 0x10) >> 0x18);
                        local_16._2_1_ = 8;
                        local_16._3_1_ = 0x10;
                        local_16._0_2_ = 0x9f4a;
                        uStack18 = uVar14;
                        uStack17 = uVar16;
                        uStack16 = uVar19;
                        uStack15 = uVar20;
                        uStack12 = uVar18;
                        BVar31 = read_file_1008_7dee(
                            param_2,
                            ((param_2_00 & 0xff00) << 0x10 | CONCAT12(uVar18, uVar26)),
                            2,
                        );
                        if (BVar31 != 0) {
                            uStack10 = 2;
                            uStack9 = 0;
                            bStack8 = 0;
                            uStack7 = 0;
                            uVar26 = param_1 + 0x1e;
                            uStack14 = uVar26;
                            uStack13 = (uVar26 >> 8);
                            uStack11 = ((param_2_00 << 0x10) >> 0x18);
                            local_16._2_1_ = 8;
                            local_16._3_1_ = 0x10;
                            local_16._0_2_ = 0x9f65;
                            uStack18 = uVar14;
                            uStack17 = uVar16;
                            uStack16 = uVar19;
                            uStack15 = uVar20;
                            uStack12 = uVar18;
                            BVar31 = read_file_1008_7dee(
                                param_2,
                                ((param_2_00 & 0xff00) << 0x10 | CONCAT12(uVar18, uVar26)),
                                2,
                            );
                            if (BVar31 != 0) {
                                return;
                            }
                        }
                    }
                    g_u16_1050_0310 = 0x6d2;
                    return;
                }
                bStack8 = 8;
                uStack7 = 0;
                uStack12 = 0xe4;
                uStack11 = 0x9e;
                uVar30 = local_36;
                process_struct_1000_179c(8, struct_a);
                local_2e = CONCAT22(struct_a, uVar30);
                if ((struct_a | uVar30) == 0) {
                    local_26 = 0;
                } else {
                    local_2e = s_1_1050_389a;
                    (uVar30 + 2) = &PTR_LOOP_1050_1008;
                    local_2e = 0xa1c4;
                    (uVar30 + 2) = 0x1010;
                    local_26 = local_2e;
                }
                uStack10 = 2;
                uStack9 = 0;
                bStack8 = 0;
                uStack7 = 0;
                uStack14 = SUB21(&local_22, 0);
                uStack13 = (&local_22 >> 8);
                local_16._2_1_ = 0;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = 0x9e69;
                uStack18 = uVar14;
                uStack17 = uVar16;
                uStack16 = uVar19;
                uStack15 = uVar20;
                uStack12 = uVar23;
                uStack11 = uVar25;
                BVar31 =
                    read_file_1008_7dee(param_2, CONCAT13(uVar25, CONCAT12(uVar23, &local_22)), 2);
                iVar33 = local_26;
                uStack10 = (local_26 >> 0x10);
                uVar15 = uStack10;
                uStack9 = (local_26 >> 0x18);
                uVar17 = uStack9;
                uStack12 = local_26;
                uVar12 = uStack12;
                uStack11 = (local_26 >> 8);
                uVar13 = uStack11;
                if (BVar31 == 0) {
                    break;
                }
                uStack10 = 2;
                uStack9 = 0;
                bStack8 = 0;
                uStack7 = 0;
                uVar26 = iVar33 + 6;
                uStack14 = uVar26;
                uStack13 = (uVar26 >> 8);
                uStack12 = ((local_26 & 0xffff0000) >> 0x10);
                uStack11 = ((local_26 & 0xffff0000) >> 0x18);
                local_16._2_1_ = 8;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = 0x9e82;
                uStack18 = uVar14;
                uStack17 = uVar16;
                uStack16 = uVar19;
                uStack15 = uVar20;
                BVar31 = read_file_1008_7dee(
                    param_2,
                    (local_26 & 0xff000000 | CONCAT12(uStack12, uVar26)),
                    2,
                );
                if (BVar31 == 0) {
                    break;
                }
                bStack8 = local_22;
                uStack7 = (local_22 >> 8);
                uStack14 = 8;
                uStack13 = 0x10;
                unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                uStack16 = 0xb6;
                uStack15 = 0x9e;
                uStack12 = uVar14;
                uStack11 = uVar16;
                uStack10 = uVar19;
                uStack9 = uVar20;
                switch_statement_1008_73ea(param_2, param_2._2_2_, local_22);
                (iVar33 + 4) = BVar31;
                uVar5 = (param_1 + 0x12);
                uStack14 = uVar5;
                uStack13 = (uVar5 >> 8);
                uStack12 = (uVar5 >> 0x10);
                uStack11 = (uVar5 >> 0x18);
                uStack16 = 8;
                uStack15 = 0x10;
                uStack18 = 0xd2;
                uStack17 = 0x9e;
                ppcVar9 = ((param_1 + 0x12) + 4);
                uStack10 = uVar12;
                uStack9 = uVar13;
                bStack8 = uVar15;
                uStack7 = uVar17;
                (**ppcVar9)();
                local_2a = local_2a + 1;
                struct_a = extraout_DX_01;
            }
            if (local_26 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            bStack8 = 1;
            uStack7 = 0;
            uStack14 = 8;
            uStack13 = 0x10;
            uStack16 = 0xa6;
            uStack15 = 0x9e;
            ppcVar9 = local_26;
            local_16._0_2_ = iVar33;
            local_16._2_1_ = uVar15;
            local_16._3_1_ = uVar17;
            uStack12 = uVar12;
            uStack11 = uVar13;
            uStack10 = uVar15;
            uStack9 = uVar17;
            (**ppcVar9)();
            g_u16_1050_0310 = 0x6d2;
            return;
        }
        0x18 => {
            local_4._0_1_ = 2;
            // goto LAB_1010_2ad8;
        }
        0x19 => {
            uStack6 = 0x3c;
            uStack5 = 0;
            bStack8 = 0x2a;
            uStack7 = 0;
            uStack10 = 8;
            uStack9 = 0;
            uStack16 = 0x10;
            uStack15 = 0x10;
            uStack18 = 0x40;
            uStack17 = 0x6e;
            uStack14 = uVar12;
            uStack13 = uVar15;
            uStack12 = uVar18;
            uStack11 = uVar17;
            uVar30 = pass1_1010_6ca2(CONCAT13(uVar17, CONCAT12(uVar18, param_1)), 8);
            if (uVar30 != 0) {
                param_1 = 0x1a;
                uStack2 = 0x52;
                uStack1 = 0x6e;
                pass1_1010_715c(CONCAT22(0x1a, iVar33), 0x1a);
            }
            if (param_2 == 0x2c) {
                uStack2 = 99;
                uStack1 = 0x6e;
                pass1_1010_715c(CONCAT22(0x1d, param_1), 0x1d);
            }
            uStack2 = 0x5a;
            uStack1 = 0;
            local_4._0_1_ = 0x10;
            local_4._1_1_ = 0x10;
            uStack6 = 0x74;
            uStack5 = 0x6e;
            uVar30 = pass1_1010_6ca2(0x5a, 2);
            if (uVar30 != 0) {
                uStack2 = 0x27;
                uStack1 = 0x6d;
                pass1_1010_715c(0x1c005a, 0x1c);
            }
            return;
        }
        0x1a => {
            (param_1 + 0x26) = param_2;
        }
    }
    local_4._0_1_ = 1;
    // LAB_1010_2ad8:
    local_4._1_1_ = 0;
    if ((local_4 == 1) || (local_4 == 2)) {
        if (local_4 == 1) {
            param_2 = ((param_1 + 0x20) + (param_1 + 0x22) + (param_1 + 0x24) + (param_1 + 0x26));
        }
        if (param_2 != 0) {
            uVar26 = param_2 / 2 + 1;
            if (5 < uVar26) {
                uVar26 = 5;
            }
            param_2 = uVar26;
            if ((local_4 == 1) && (PTR_LOOP_1050_10c6 != 0x0)) {
                if (4 < uVar26) {
                    uVar26 = 4;
                }
                param_2 = uVar26;
            }
        }
    }
    (local_4 * 0x7c + 0xed6) = param_2;
    uStack6 = 0xc;
    uStack5 = 0;
    uStack12 = unaff_CS;
    uStack11 = (unaff_CS >> 8);
    uStack14 = 0x4b;
    uStack13 = 0x2b;
    uStack10 = uVar12;
    uStack9 = uVar15;
    bStack8 = uVar18;
    uStack7 = uVar17;
    pass1_1010_1f62(CONCAT13(uVar17, CONCAT12(uVar18, param_1)), 0xc);
    // switchD_1010_2ab5_caseD_0:
    return;
}

pub fn destroy_win_1010_2fa0(param_1: *mut astruct_340) {
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let local_BX_7: *mut astruct_340;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_7 = param_1;
    local_BX_7.field_0x28 = 0;
    local_4 = 0;
    loop {
        puVar1 = &local_BX_7.field_0x16;
        if (unsafe { *puVar1 == local_4 } || unsafe { *puVar1 < local_4 }) {
            break;
        }
        uVar2 = (&local_BX_7.field_0x2a + local_4 * 4);
        DestroyWindow16((uVar2 + 0x18));
        local_4 = local_4 + 1;
    }
    local_BX_7.field_0x16 = 0;
    if ((&local_BX_7.field_0x54 | local_BX_7.field_0x52) != 0) {
        local_4 = 0;
        while {
            uVar2 = &local_BX_7.field_0x52;
            if ((uVar2 + local_4 * 4) != 0) {
                uVar2 = &local_BX_7.field_0x52;
                uVar2 = (uVar2 + local_4 * 4);
                DestroyWindow16((uVar2 + 0x18));
                uVar2 = &local_BX_7.field_0x52;
                (uVar2 + local_4 * 4) = 0;
            }
            local_4 = local_4 + 1;
            local_4 < 10
        } {}
        error_check_1000_17ce(&local_BX_7.field_0x52);
        &local_BX_7.field_0x52 = 0;
    }
    return;
}

pub fn win_gui_fn_1010_32f4(param_1: *mut astruct_387, param_2: *mut u32) {
    let puVar1: *mut u16;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let extraout_DX: *mut u16;
    let mut extraout_DX_00: u16;
    let local_BX_5: *mut astruct_387;
    let mut iVar9: i32;
    let mut iVar10: i32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut unaff_CS: u16;
    let mut uVar13: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let temp_86224844142: *mut u32;
    let mut temp_5f9d4a48c3: u32;
    let mut temp_5fd8766d76: u32;
    let fn_ptr_1: fn();

    uVar11 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (&local_BX_5.field_0x52 != 0) {
        unaff_CS = 0x1000;
        error_check_1000_17ce(&local_BX_5.field_0x52);
        &local_BX_5.field_0x52 = 0;
        local_BX_5.field_0x18 = 0;
    }
    uVar6 = param_2._2_2_ | param_2;
    if ((param_2 != 0x0)
        && (
            fn_ptr_1 = (param_1 + 0x24),
            (**fn_ptr_1)(unaff_CS, param_1, param_2),
            uVar6 != 0,
        ))
    {
        unsafe {
            fn_ptr_1 = (*param_2 + 4);
        }
        (**fn_ptr_1)(unaff_CS, param_2);
        local_BX_5.field_0x18 = uVar6;
        if (uVar6 != 0) {
            local_BX_5.field_0x24 = 0;
            local_BX_5.field_0x26 = 0;
            _g_astruct_94_ptr_1 = local_BX_5.field_0x28;
            puVar1 = &local_BX_5.field_0x18;
            unsafe {
                *puVar1 = *puVar1 - _g_astruct_94_ptr_1;
            }
            if (10 < local_BX_5.field_0x18) {
                local_BX_5.field_0x26 = 1;
                local_BX_5.field_0x18 = 10;
            }
            if (1 < local_BX_5.field_0x28) {
                local_BX_5.field_0x24 = 1;
            }
            if (__g_astruct_94_ptr_1 == 0) {
                g_u16_ptr_1050_5f2e = extraout_DX;
                struct_fn_1000_160a();
            } else {
            }
            uVar13 = 0x1000;
            alloc_mem_1000_1708(0x28, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
            local_BX_5.field_0x52 = _g_astruct_94_ptr_1;
            &local_BX_5.field_0x54 = g_u16_ptr_1050_5f2e;
            if ((&local_BX_5.field_0x54 | local_BX_5.field_0x52) != 0) {
                uVar3 = (param_2 + 8);
                iVar7 = local_BX_5.field_0x10 + -10;
                local_c = 0;
                local_10 = 0;
                while (
                    puVar1 = &local_BX_5.field_0x18,
                    unsafe { *puVar1 != local_10 } && unsafe { local_10 <= *puVar1 },
                ) {
                    uVar4 = &local_BX_5.field_0x52;
                    uVar6 = uVar4 + local_10 * 4;
                    uVar4 = uVar4 & 0xffff0000;
                    _local_30 = (uVar4 | uVar6);
                    temp_5fd8766d76 = ((local_BX_5.field_0x28 + local_10) * 4 + uVar3);
                    fn_ptr_1 = (param_1 + 0x1c);
                    uVar8 = local_10;
                    (**fn_ptr_1)(
                        uVar13,
                        param_1,
                        temp_5fd8766d76,
                        (temp_5fd8766d76 >> 0x10),
                        local_BX_5.field_0x22,
                    );
                    *_local_30 = uVar8;
                    *(uVar6 + 2) = extraout_DX_00;
                    uVar5 = &local_BX_5.field_0x52;
                    uVar5 = (uVar5 + local_10 * 4);
                    local_c = local_c + (uVar5 + 0x24) + 8;
                    if (iVar7 < local_c) {
                        uVar13 = SUB42(&PTR_LOOP_1050_1008, 0);
                        fn_1008_6048(
                            s_overflow_on_node__d_1050_11ca,
                            extraout_DX_00,
                            SUB21(iVar7, 0),
                        );
                        local_BX_5.field_0x18 = local_10 - 1;
                        local_BX_5.field_0x26 = 1;
                        uVar5 = &local_BX_5.field_0x52;
                        uVar12 = (uVar5 >> 0x10);
                        iVar10 = uVar5;
                        puVar2 = (iVar10 + local_10 * 4);
                        uVar6 = (iVar10 + local_10 * 4 + 2);
                        if ((uVar6 | puVar2) != 0) {
                            unsafe {
                                fn_ptr_1 = *puVar2;
                            }
                            (**fn_ptr_1)(&PTR_LOOP_1050_1008, puVar2, uVar6, 1);
                        }
                        uVar5 = &local_BX_5.field_0x52;
                        iVar10 = local_10 * 4;
                        (uVar5 + iVar10) = 0;
                        if (0 < local_10) {
                            uVar5 = &local_BX_5.field_0x52;
                            uVar12 = (uVar5 >> 0x10);
                            iVar9 = uVar5;
                            puVar2 = (iVar9 + iVar10 + -4);
                            uVar6 = (iVar9 + iVar10 + -2);
                            if ((uVar6 | puVar2) != 0) {
                                unsafe {
                                    fn_ptr_1 = *puVar2;
                                }
                                (**fn_ptr_1)(&PTR_LOOP_1050_1008, puVar2, uVar6, 1);
                            }
                            uVar5 = &local_BX_5.field_0x52;
                            (local_10 * 4 + uVar5 + -4) = 0;
                        }
                    }
                    local_10 = local_10 + 1;
                }
                local_BX_5.field_0x20 = 10;
                uVar13 = local_BX_5.field_0x1e;
                uVar3 = &local_BX_5.field_0x52;
                update_window_1040_93aa(uVar3, (uVar3 >> 0x10), 10, uVar13);
                local_10 = 1;
                while (
                    puVar1 = &local_BX_5.field_0x18,
                    unsafe { *puVar1 != local_10 } && unsafe { local_10 <= *puVar1 },
                ) {
                    uVar3 = &local_BX_5.field_0x52;
                    uVar3 = (local_10 * 4 + uVar3 + -4);
                    iVar7 = uVar3;
                    uVar12 = (uVar3 >> 0x10);
                    uVar3 = &local_BX_5.field_0x52;
                    uVar3 = (uVar3 + local_10 * 4);
                    update_window_1040_93aa(
                        uVar3,
                        (uVar3 >> 0x10),
                        (iVar7 + 0x20) + (iVar7 + 0x24) + 0x8,
                        uVar13,
                    );
                    local_10 = local_10 + 1;
                }
            }
        }
    }
    return;
}

pub fn win_gui_fn_1010_79aa(param_1: *mut void, param_2: u16, param_3: u32) {
    let hwnd: HWND16;
    let mut u_var1: u32;
    let mut uVar2: u32;
    let local_AX_66: *mut astruct_17;
    let mut extraout_DX: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_1c: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    uVar3 = (param_1 >> 0x10);
    if (((param_1 + 0x1c) != 0) && (param_3 != 0 || (param_2 != 0))) {
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x1c));
        local_12 = 0;
        while {
            local_AX_66 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_66));
            local_e = CONCAT22(extraout_DX, local_AX_66);
            if ((extraout_DX | local_AX_66) == 0) {}
            // goto LAB_1010_7a49;
            if (((param_2 == 0) && (local_AX_66.field_0x4 == param_3))
                || (param_3 == 0 && (uVar1 = local_AX_66.field_0x8, (uVar1 + 10) == param_2)))
            {
                break;
            }
            (local_AX_66.field_0x4 != param_3)
                || (uVar1 = local_AX_66.field_0x8, (uVar1 + 10) != param_2)
        } {}
        local_12 = local_e;
        // LAB_1010_7a49:
        if (local_12 != 0) {
            uVar2 = (local_12 + 8);
            hwnd = (uVar2 + 6);
            SetFocus16(hwnd);
            BringWindowToTop16(hwnd);
            return;
        }
    }
    return;
}

pub fn show_window_1010_7a76(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x20) == 0) {
        (iVar2 + 0x20) = 1;
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
            if (lVar4 == 0) {
                break;
            }
            uVar1 = (lVar4 + 8);
            ShowWindow16(0, (uVar1 + 6));
        }
    }
    return;
}

pub fn show_window_1010_7ace(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x20) != 0) {
        (iVar2 + 0x20) = 0;
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
            if (lVar4 == 0) {
                break;
            }
            uVar1 = (lVar4 + 8);
            ShowWindow16(1, (uVar1 + 6));
        }
    }
    return;
}

pub fn destroy_win_1010_7b26(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;
    let puVar2: *mut u8;
    let mut extraout_DX: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (((iVar3 + 0x1e) | (iVar3 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar3 + 0x1c));
        while {
            puVar2 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_SS, puVar2));
            if ((extraout_DX | puVar2) == 0) {
                break;
            }
            (puVar2 + 4) != param_2
        } {}
        if ((extraout_DX | puVar2) != 0) {
            uVar1 = (puVar2 + 8);
            DestroyWindow16((uVar1 + 6));
        }
    }
    return;
}

pub fn win_gui_fn_1010_8096(param_1: *mut u32, param_2: u16) {
    let in_struct_1: *mut astruct_44;
    let uVar1: u8;
    let pcVar2: *mut char;
    let extraout_var: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: i32;
    let string_b: *mut char;
    let mut local_312: u32;
    let mut local_30e: u32;
    let mut local_30a: u16;
    let mut local_308: u16;
    let mut string_1: [u8; 256];
    let mut string_2: [u8; 256];
    let mut string_3: [u8; 260];
    let mut uvar3: u16;

    uVar5 = (param_1 >> 0x10);
    process_string_1000_4d58(((param_1 + 0xe82) * 4 + 0x2526), 0, 0);
    copy_string_1000_3d3e(CONCAT22(unaff_SS, string_3), CONCAT22(unaff_SS, string_2));
    if (param_2 == 2) {
        string_b = "b";
    } else {
        string_b = "a";
    }
    process_string_1000_3cea(CONCAT22(unaff_SS, string_3), string_b);
    process_string_1000_3cea(CONCAT22(unaff_SS, string_3), CONCAT22(unaff_SS, string_1));
    pcVar2 = string_3;
    set_error_mode_1010_8b14(param_1, pcVar2, unaff_SS);
    _local_30a = CONCAT22(extraout_DX, pcVar2);
    iVar4 = extraout_DX;
    if ((pcVar2 == string_3) && (extraout_DX == unaff_SS)) {
        msg_box_1010_8bb4(param_1, pcVar2, extraout_DX);
        iVar4 = extraout_DX_00;
    }
    unsafe {
        in_struct_1 = *param_1;
    }
    uVar1 = error_check_1000_17ce(in_struct_1);
    uVar3 = CONCAT31(extraout_var, uVar1);
    pass1_fn_1008_60e8(_local_30a, in_struct_1, in_struct_1);
    param_1 = uVar3;
    (param_1 + 2) = iVar4;
    return;
}

pub fn win_gui_fn_1010_8170(param_1: *mut u32, param_2: i32) {
    let mut uVar1: i32;
    let in_DX: *mut u16;
    let mut iVar2: i32;
    let local_BX_20: *mut astruct_447;
    let mut uvar3: u16;
    let mut uVar4: u32;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0x680);
    local_BX_20 = (param_2 * 0x10);
    if (local_BX_20.field_0x16 != uVar1) {
        win_gui_fn_1010_8096(param_1, local_BX_20.field_0x16);
        uVar4 = pass1_1010_878c(iVar2, uVar3, local_BX_20.field_0x16);
        in_DX = (uVar4 >> 0x10);
        uVar1 = uVar4;
        if ((iVar2 + 0x67c) == 0) {
            return;
        }
    }
    param_2 = param_2 * 0x10;
    pass1_1008_6562(
        (iVar2 + 0x67c),
        CONCAT22((param_2 + 0x1c), (param_2 + 0x1e)),
        (param_2 + 0x1a),
        uVar1,
        in_DX,
    );
    return;
}

pub fn msg_box_1010_8bb4(param_1: u16_00, param_2: u16, param_1: *mut char) {
    let mut title: string;
    let in_string_2: *mut char;
    let mut unaff_SS: u16;
    let mut w_param: u32;
    let mut local_402: [u8; 1024];

    in_string_2 = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3fa,
    );
    copy_string_1000_3d3e(CONCAT22(unaff_SS, local_402), in_string_2);
    process_string_1000_3cea(CONCAT22(unaff_SS, local_402), param_1);
    title = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x57b,
    );
    MessageBox16(0x1010, title, CONCAT22(unaff_SS, local_402), g_h_window);
    PostMessage16(0, 0xee, 0x111, g_h_window);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn load_cursor_1018_5840(param_1: *mut astruct_65, param_2: u16, param_3: u32) {
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut unaff_BP: u16;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;

    load_cursor_1020_7f7a(param_1, CONCAT22(param_2, 6), param_3);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + 0xee) = 0;
    (iVar2 + 0xf2) = 0;
    (iVar2 + 0xf6) = 0;
    param_1.ptr_a_lo = (s_Alloc__s_1050_5a5b + 7);
    (iVar2 + 2) = 0x1018;
    (iVar2 + 0xe2) = 0x5afe;
    (iVar2 + 0xe4) = 0x1018;
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x27));
    uVar1 = (ppVar4 >> 0x10);
    (iVar2 + 0xf2) = ppVar4;
    (iVar2 + 0xf4) = uVar1;
    (iVar2 + 0xe6) = (iVar2 + 0xf2);
    (iVar2 + 0xe8) = uVar1;
    return;
}

pub fn set_window_text_1018_6066(param_1: u32, in_window_text: u32, in_dlg_item_id: u16) {
    let hwnd: HWND16;

    hwnd = GetDlgItem16(in_dlg_item_id, (param_1 + 6));
    SetWindowText16(in_window_text, hwnd);
    return;
}

pub fn set_window_text_1018_6630(in_struct_604_ptr: *mut astruct_604) {
    let mut in_dlg_item_id: u16;
    let mut uVar1: i32;
    let struct_604_ptr_1: *mut astruct_604;
    let struct_60_ptr_hi: *mut astruct_604;
    let mut local_c: u16;
    let window_text: SEGPTR;
    let mut local_4: u16;
    let struct_60_ptr_1: *mut astruct_60;

    struct_60_ptr_hi = (in_struct_604_ptr >> 0x10);
    struct_604_ptr_1 = in_struct_604_ptr;
    window_text = load_rsrc_1010_4e9e(struct_604_ptr_1.astruct_60_ptr);
    if (window_text != 0) {
        local_c = 0;
        while (local_c < 9) {
            struct_60_ptr_1 = struct_604_ptr_1.astruct_60_ptr;
            in_dlg_item_id = pass1_1010_4f20(struct_60_ptr_1, (struct_60_ptr_1 >> 0x10), local_c);
            set_window_text_1018_6066(struct_604_ptr_1.field_0xa, window_text, in_dlg_item_id);
            uVar1 = get_string_index_1000_3da4(window_text);
            window_text = window_text & 0xffff0000 | (window_text + uVar1 + 1);
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn destroy_win_1018_c518(in_struct_376_1: *mut astruct_376) {
    let b_rc: bool;
    let struct_a_1: *mut astruct_376;
    let struct_a_2: *mut astruct_376;
    let temp_5fa5d31fd0: *mut astruct_376;

    struct_a_2 = (in_struct_376_1 >> 0x10);
    struct_a_1 = in_struct_376_1;
    in_struct_376_1.ptr_a_lo = 0xc8bc;
    struct_a_1.ptr_a_hi = 0x1018;
    error_check_1000_17ce(&struct_a_1.struct_44_a);
    if (struct_a_1.window_handle_a != 0) {
        b_rc = IsWindow16(struct_a_1.window_handle_a);
        if (b_rc != 0) {
            DestroyWindow16(struct_a_1.window_handle_a);
            struct_a_1.window_handle_a = 0;
        }
    }
    pass1_1020_022c(in_struct_376_1);
    return;
}

pub fn destroy_win_fn_1018_c896(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376 {
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn create_win_fn_1018_df40(in_struct_42_ptr_1: *mut win_struct_42) {
    let local_struct_42_1: *mut win_struct_42;
    let mut local_struct_42_hi_1: u16;
    let create_win_result: *mut astruct_199;
    let mut local_u32_43: u32;
    let mut local_4: u16;

    create_win_result = create_win_1008_9760(in_struct_42_ptr_1);
    process_struct_1000_179c(10, (create_win_result >> 0x10));
    local_struct_42_1 = in_struct_42_ptr_1;
    local_struct_42_hi_1 = (in_struct_42_ptr_1 >> 0x10);
    if (create_win_result != 0x0) {
        local_u32_43 = pass1_1018_e100(create_win_result, local_struct_42_1.win_handle_0x8);
        local_struct_42_1.u16_xe2 = local_u32_43;
        local_struct_42_1.u16_xe4 = (local_u32_43 >> 0x10);
        return;
    }
    &local_struct_42_1.u16_xe2 = 0;
    return;
}

pub fn destroy_win_fn_1018_df92(param_1: *mut astruct_594) {
    let puVar1: *mut u32;
    let pvVar2: *mut void;
    let local_struct_594_ptr_1: *mut astruct_594;
    let mut unaff_SI: u16;
    let mut uvar3: u16;
    let temp_862fc4681f0: *mut u32;
    let local_fn_ptr_1: fn();

    destroy_win_1008_628e(param_1, unaff_SI);
    uVar3 = (param_1 >> 0x10);
    local_struct_594_ptr_1 = param_1;
    puVar1 = local_struct_594_ptr_1.u32_xE2;
    pvVar2 = local_struct_594_ptr_1.vptr_xE4;
    if ((pvVar2 | puVar1) != 0) {
        unsafe {
            local_fn_ptr_1 = *puVar1;
        }
        (**local_fn_ptr_1)(&PTR_LOOP_1050_1008, puVar1, pvVar2, 1);
    }
    &local_struct_594_ptr_1.u32_xE2 = 0;
    return;
}

pub fn destroy_win_fn_1018_e72a(param_1: *mut astruct_594) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut in_stack_0000fff6: u16;
    astruct_594 * *temp_86277dbc59c;
    let fn_ptr_1: fn();

    uVar3 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
        }
        (**fn_ptr_1)();
    }
    destroy_win_1008_628e(param_1, in_stack_0000fff6);
    return;
}

pub fn win_gui_fn_1018_e8bc(param_1: *mut astruct_44) {
    let local_BX_3: *mut astruct_376;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    param_1.ptr_a_lo = 0xe912;
    local_BX_3.ptr_a_hi = 0x1018;
    if (&local_BX_3.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_BX_3.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(param_1);
    return;
}

pub fn win_gui_fn_1018_e8ec(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376 {
    win_gui_fn_1018_e8bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// NG: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_gui_fn_1018_eada(param_1: *mut win_struct_42) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let paVar1: *mut astruct_199;
    // local_DXAX_28: i32;
    let mut local_DXAX_61: u32;
    let mut local_4: u16;

    paVar1 = create_win_1008_9760(param_1);
    local_DXAX_28._2_2_ = (paVar1 >> 0x10);
    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    local_DXAX_28._0_2_ =
        get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x28, local_DXAX_28._2_2_);
    if ((local_DXAX_28._2_2_ | local_DXAX_28) != 0) {
        local_DXAX_61 = draw_fn_1018_ec74(
            local_DXAX_28,
            local_DXAX_28._2_2_,
            local_struct_1.win_handle_0x8,
        );
        local_struct_1.char_ptr_16_0xee = local_DXAX_61;
        local_struct_1.field_0xf0 = (local_DXAX_61 >> 0x10);
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub fn win_gui_fn_1018_eb3e(in_struct_1: *mut astruct_594) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    // ppuVar4: *mut *mut u8;
    let paVar5: *mut astruct_594;
    let local_struct_1: *mut astruct_594;
    let local_struct_1_hi: *mut astruct_594;
    let mut in_stack_0000fff2: u16;
    let temp_862c2f7bda0: *mut u8;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    puVar1 = local_struct_1.u8_ptr_16_xee;
    uVar2 = local_struct_1.field_0xf0;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
        }
        (**ppcVar3)();
    }
    if (&local_struct_1.field_0xf6 != 0) {
        if ((local_struct_1_hi | local_struct_1) == 0) {
            ppuVar4 = 0x0;
            paVar5 = 0x0;
        } else {
            ppuVar4 = &local_struct_1.u32_xE2;
            paVar5 = local_struct_1_hi;
        }
        pass1_1010_1ea6(*&local_struct_1.field_0xf6, CONCAT22(paVar5, ppuVar4));
    }
    destroy_win_1008_628e(in_struct_1, in_stack_0000fff2);
    return;
}

pub fn win_gui_fn_1018_ed98(in_struct_1: *mut astruct_44) {
    let local_BX_3: *mut astruct_376;
    let mut uVar1: i32;

    uVar1 = (in_struct_1 >> 0x10);
    local_BX_3 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x1cc;
    local_BX_3.ptr_a_hi = 0x1020;
    if (&local_BX_3.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1ea6(
            local_BX_3.u8_ptr_x14,
            (in_struct_1 & 0xffff | uVar1 << 0x10),
        );
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_BX_3.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(in_struct_1);
    return;
}

pub fn win_gui_fn_1020_01a6(in_struct_1: *mut astruct_376, in_byte_1: u8) -> *mut astruct_376 {
    win_gui_fn_1018_ed98(in_struct_1);
    if ((in_byte_1 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn load_cursor_fn_1020_01d8(
    in_struct_1: *mut astruct_65,
    in_struct_1_hi: *mut astruct_65,
    param_3: u16,
    param_3: u16_00,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: *mut void,
) {
    load_cursor_1008_61b2(
        CONCAT22(in_struct_1_hi, in_struct_1),
        param_3,
        param_7,
        param_8,
    );
    &in_struct_1.ptr_b_lo = 0;
    &in_struct_1.u16_xe6 = 0;
    in_struct_1.u16_xea = param_6;
    in_struct_1.u16_xec = param_5;
    in_struct_1.u16_xee = param_3_00;
    CONCAT22(in_struct_1_hi, in_struct_1) = 0x45a;
    in_struct_1.ptr_a_hi = 0x1020;
    return;
}

pub fn win_gui_fn_1020_028c(param_1: *mut astruct_628, param_2: u16, param_3: u16) {
    pass1_1010_3c9e(param_1.field_0xe2);
    show_window_1008_68c6(param_1, param_2, param_3);
    return;
}

pub fn pass1_1020_02ae(in_struct_1: *mut astruct_594) {
    let local_struct_1: *mut astruct_594;
    let mut unaff_SI: u16;
    let local_struct_1_hi: *mut astruct_594;
    let temp_5f77ded944: *mut u8;
    let temp_5f51233cf1: *mut u8;
    // fn_ptr_1: *mut *mut u8;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pass1_1010_3cd0(&local_struct_1.u32_xE2);
    destroy_win_1008_628e(in_struct_1, unaff_SI);
    // WARNING: Load size is inaccurate
    temp_5f77ded944 = local_struct_1.u8_ptr_32_xE6;
    temp_5f51233cf1 = *(&local_struct_1.u8_ptr_32_xE6 + 2);
    if ((temp_5f51233cf1 | temp_5f77ded944) != 0) {
        fn_ptr_1 = temp_5f77ded944;
        (**fn_ptr_1)(&PTR_LOOP_1050_1008, temp_5f77ded944, temp_5f51233cf1, 1);
    }
    local_struct_1.u8_ptr_32_xE6 = 0x0;
    // WARNING: Load size is inaccurate
    pass1_1010_1dda(local_struct_1.u32_xE2);
    &local_struct_1.u32_xE2 = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn create_win_fn_1020_0316(in_struct_1: *mut win_struct_42) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let ppVar1: *mut pass1_struct_1;
    let in_struct_1_00: *mut astruct_629;
    let mut uVar2: u32;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;

    create_win_1008_9760(in_struct_1);
    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff2, 1));
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.u16_xe2 = ppVar1;
    local_struct_1.u16_xe4 = (ppVar1 >> 0x10);
    uVar2 = &local_struct_1.u16_xe2;
    (uVar2 + 0x16) = &local_struct_1.field_0xea;
    uVar2 = &local_struct_1.u16_xe2;
    *(uVar2 + 0x12) = local_struct_1.char_ptr_16_0xee;
    in_struct_1_00 = pass1_1010_3c52(&local_struct_1.u16_xe2, &local_struct_1.field_0xec);
    process_struct_1000_179c(0x12, (in_struct_1_00 >> 0x10));
    if (in_struct_1_00 != 0x0) {
        uVar2 = process_struct_1020_04f6(in_struct_1_00, local_struct_1.win_handle_0x8);
        &local_struct_1.field_0xe6 = uVar2;
        &local_struct_1.field_0xe8 = (uVar2 >> 0x10);
        return;
    }
    &local_struct_1.field_0xe6 = 0;
    return;
}

pub fn win_gui_fn_1020_0762(
    param_1: *mut astruct_65,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: u32,
    param_6: u32,
) {
    let in_struct_1: *mut astruct_65;
    let in_struct_1_hi: *mut astruct_65;

    in_struct_1 = param_1;
    in_struct_1_hi = (param_1 >> 0x10);
    load_cursor_fn_1020_01d8(
        in_struct_1,
        in_struct_1_hi,
        param_2,
        (param_2 >> 0x10),
        param_4,
        param_5,
        (param_5 >> 0x10),
        param_6,
    );
    in_struct_1.u16_xf0 = 0;
    &in_struct_1.u16_xf2 = param_3;
    param_1.ptr_a_lo = 0x81a;
    in_struct_1.ptr_a_hi = 0x1020;
    return;
}

pub fn enable_menu_item_1020_1000() {
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;

    if (in_stack_0000000a != 0) {
        return;
    }
    EnableMenuItem16(0x400, 3, in_stack_0000000c);
    return;
}

pub fn destroy_icon_func_1020_1038(in_struct_1: *mut astruct_48) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_struct_1: *mut astruct_48;
    let local_struct_1_hi: *mut astruct_48;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    DestroyIcon16(local_struct_1.handle_0xc2);
    local_struct_1.handle_0xc2 = 0;
    local_struct_1.field_0x8 = 0;
    puVar1 = local_struct_1.fn_ptr_0xf6;
    uVar2 = &local_struct_1.field_0xf8;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
        }
        (**fn_ptr_1)(offset, puVar1, uVar2, 1);
    }
    &local_struct_1.fn_ptr_0xf6 = 0;
    pass1_1010_1dda(local_struct_1.field_0xf2);
    local_struct_1.field_0xf2 = 0;
    return;
}

pub fn update_window_1020_10a0(param_1: *mut win_struct_42) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: i32;
    let piVar4: *mut u16;
    let paVar5: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let paVar6: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut iVar7: i32;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let paVar9: *mut astruct_199;
    let ppVar10: *mut pass1_struct_1;
    let mut uVar11: u32;
    let mut uVar12: u16;
    let local_3a: *mut astruct_71;
    let mut local_8: u16;

    paVar9 = create_win_1008_9760(param_1);
    paVar6 = (paVar9 >> 0x10);
    uVar3 = paVar9;
    process_struct_1000_179c(0x42, paVar6);
    paVar5 = (paVar6 | uVar3);
    iVar7 = param_1;
    uVar8 = (param_1 >> 0x10);
    if (paVar5 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar6,
            0,
            0x1f009b,
            0x750000,
            0xf10074,
            CONCAT22(unaff_SI, (iVar7 + 8)),
        );
        paVar5 = extraout_DX;
    }
    process_struct_1000_179c(0x42, paVar5);
    paVar6 = (paVar5 | uVar3);
    if (paVar6 != 0x0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar5,
            0,
            0x31009b,
            0x770000,
            0xf20076,
            CONCAT22(unaff_SI, (iVar7 + 8)),
        );
        paVar6 = extraout_DX_00;
    }
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | uVar3) != 0) {
        win_fn_1008_3bd6(
            uVar3,
            paVar6,
            0,
            0x77009b,
            0x790000,
            0xf30078,
            CONCAT22(unaff_SI, (iVar7 + 8)),
        );
    }
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2d));
    uVar12 = (ppVar10 >> 0x10);
    (iVar7 + 0xf2) = ppVar10;
    (iVar7 + 0xf4) = uVar12;
    uVar3 = (iVar7 + 0xf2);
    (iVar7 + 0xe0) = uVar3;
    (iVar7 + 0xe2) = uVar12;
    LoadIcon16(
        0x1010,
        s_PLNTICON_1050_4267,
        &g_alloc_addr_1050_1050,
        g_h_instance_1050_038c,
    );
    (iVar7 + 0xc2) = uVar3;
    uVar1 = (iVar7 + 0xf2);
    uVar12 = uVar1;
    ppcVar2 = ((iVar7 + 0xf2) + 0x30);
    ppcVar2(offset, uVar12, (uVar1 >> 0x10), uVar3);
    paVar5 = struct_a;
    process_struct_1000_179c(0x24, struct_a);
    if ((paVar5 | uVar3) == 0) {
        (iVar7 + 0xf6) = 0;
    } else {
        get_dc_1020_1418(uVar3, paVar5, param_1, uVar8);
        (iVar7 + 0xf6) = uVar3;
        (iVar7 + 0xf8) = extraout_DX_01;
    }
    (iVar7 + 0xe8) = (iVar7 + 0xf6);
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar12, 0x2f));
    uVar11 = pass1_1018_04b8(ppVar10);
    pass1_1010_41d6((iVar7 + 0xf2), uVar11);
    uVar11 = pass1_1010_451a((iVar7 + 0xf2));
    piVar4 = uVar11;
    uVar1 = param_1;
    ppcVar2 = (uVar1 + 0x14);
    ppcVar2(0x1010, iVar7, uVar8, 0, uVar11, (uVar11 >> 0x10));
    uVar12 = 1;
    ppcVar2 = (uVar1 + 0x10);
    ppcVar2();
    pass1_1010_459e((iVar7 + 0xf2));
    ppcVar2 = ((iVar7 + 0xf2) + 0x10);
    ppcVar2(0x1010, (iVar7 + 0xf2), param_1, uVar12);
    MoveWindow16(
        1,
        piVar4[3],
        piVar4[2],
        piVar4[1],
        unsafe { *piVar4 },
        (iVar7 + 8),
    );
    UpdateWindow16((iVar7 + 8));
    return;
}

pub fn win_fn_1020_1294(param_1: *mut astruct_637, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let ppVar4: *mut pass1_struct_2;
    let paVar5: *mut astruct_199;
    let h_cursor: *mut u16;
    let h_cursor_00: HCURSOR16;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar4 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), 0x4000001);
    if ((in_DX | ppVar4) == 0) {
        local_6 = param_3;
        local_4 = param_2;
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        pass1_1010_40cc((iVar6 + 0xf2));
        _local_a = CONCAT22(extraout_DX, param_2);
        uVar1 = (iVar6 + 0xf2);
        paVar5 = (uVar1 + 0x76);
        local_e = uVar1 & 0xffff0000 | ZEXT24(paVar5);
        pass1_1008_3e94(
            paVar5,
            CONCAT22(unaff_SS, &local_12),
            CONCAT22(unaff_SS, &local_10),
        );
        local_6 = local_6 - local_10;
        local_4 = local_4 - local_12;
        h_cursor = &local_6;
        uVar2 = (iVar6 + 0xf2);
        pt_in_rect_1010_40f8(uVar2, (uVar2 >> 0x10), h_cursor, unaff_SS);
        if (h_cursor != 0xffff) {
            h_cursor_00 = LoadCursor16(0x7f02, 0);
            SetCursor16(h_cursor_00);
            ppcVar3 = (*_local_a + 4);
            (**ppcVar3)();
            pass1_1008_3e0e(param_1);
            SetCursor16(h_cursor);
        }
    }
    return;
}

pub fn call_destroy_menu_fn_1020_135e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn get_dc_1020_1418(param_1: *mut u16, param_2: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let puVar3: *mut u16;
    let mut uVar4: u32;
    let mut extraout_DX: i32;
    let local_BX_17: *mut astruct_63;
    let mut unaff_SS: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut in_stack_0000ffdc: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar6 = param_1;
    uVar7 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(iVar6, uVar7, param_2, (param_2 >> 0x10));
    (iVar6 + 0x14) = 0;
    (iVar6 + 0x18) = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | (iVar6 + 0x1e)));
    unsafe {
        *param_1 = 0x1730;
    }
    (iVar6 + 2) = 0x1020;
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffdc, 0x2d));
    (iVar6 + 0x14) = ppVar5;
    (iVar6 + 0x16) = (ppVar5 >> 0x10);
    _local_6 =
        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffdc, 0x29));
    uVar1 = (iVar6 + 0x14);
    ppcVar2 = ((iVar6 + 0x14) + 4);
    ppcVar2(0x10, uVar1, (uVar1 >> 0x10), 0, iVar6, uVar7);
    local_8 = GetDC16((iVar6 + 4));
    uVar1 = (iVar6 + 0x14);
    (uVar1 + 0x7c) = local_8;
    uVar1 = (iVar6 + 0x14);
    uVar4 = (uVar1 + 0x66);
    (iVar6 + 0x18) = uVar4;
    ppcVar2 = ((iVar6 + 0x18) + 0x14);
    ppcVar2();
    uVar1 = (_local_6 + 0xe);
    pass1_1008_4d84((uVar4 & 0xffff | extraout_DX << 0x10), uVar1);
    puVar3 = &local_8;
    realize_palette_1008_4e08(uVar1, puVar3, unaff_SS);
    (iVar6 + 0x1c) = puVar3;
    return;
}

pub fn set_dialog_item_1040_a94a(in_struct_1: *mut astruct_351) {
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let lVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let value: *mut u8;
    let mut uVar6: u16;
    let mut uVar7: i32;
    let HVar8: HWND16;
    let mut value_00: u16;
    let mut extraout_DX: i32;
    let mut uVar10: i32;
    let local_struct_1: *mut astruct_351;
    let mut iVar11: i32;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut unaff_SS: u16;
    let mut bVar14: bool;
    let ppVar15: *mut pass1_struct_1;
    let mut local_128: u16;
    let mut local_126: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u32;
    let mut local_116: u16;
    let mut local_114: u16;
    let mut local_112: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: [u8; 256];
    let mut uVar9: u32;

    ppVar15 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
    uVar5 = (ppVar15 >> 0x10);
    uVar4 = ppVar15;
    uVar12 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pass1_1010_c3c2(
        uVar4,
        uVar5,
        CONCAT22(unaff_SS, local_102),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(unaff_SS, local_102),
        0x1f2,
        local_struct_1.field_0x6,
    );
    pass1_1010_c320(
        uVar4,
        uVar5,
        CONCAT22(unaff_SS, local_102),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(unaff_SS, local_102),
        0x1774,
        local_struct_1.field_0x6,
    );
    str_fn_1010_c446(
        ppVar15,
        CONCAT22(unaff_SS, local_102),
        local_struct_1.field_0x94,
    );
    value = local_102;
    SetDlgItemText16(CONCAT22(unaff_SS, value), 0x1773, local_struct_1.field_0x6);
    uVar2 = local_struct_1.field_0x94;
    pass1_1030_6ddc(uVar2, (uVar2 >> 0x10));
    (0, value, 0x1f5, local_struct_1.field_0x6);
    pass1_1030_6e14(local_struct_1.field_0x94);
    (0, value, 0x1f6, local_struct_1.field_0x6);
    if (local_struct_1.field_0x98 != 0) {
        pass1_1010_dd5e(uVar4, uVar5, local_struct_1.field_0x94);
        if ((extraout_DX | value) != 0) {
            uVar2 = local_struct_1.field_0x94;
            uVar13 = (uVar2 >> 0x10);
            iVar11 = uVar2;
            lVar3 = (iVar11 + 0x26);
            uVar10 = (iVar11 + 0x28);
            local_114 = 0x1798;
            local_116 = 0x17c3;
            local_struct_1.field_0xea = 0;
            local_120 = 1;
            while (local_120 < 0x25) {
                if (lVar3 == 0) {
                    value_00 = 0;
                    uVar7 = 0;
                } else {
                    uVar9 = pass1_1020_bae6(lVar3, CONCAT22(local_120, (lVar3 >> 0x10)));
                    value_00 = uVar9;
                    uVar7 = uVar10;
                }
                bVar14 = (value + local_120 * 4) != 0;
                uVar10 = uVar7;
                if (bVar14) {
                    uVar6 = value_00;
                    big_switch_statement_1020_c0d8(local_120);
                    SetDlgItemText16(CONCAT22(uVar10, uVar6), local_114, local_struct_1.field_0x6);
                    SetDlgItemInt16(
                        0,
                        *(value + local_120 * 4),
                        local_116,
                        local_struct_1.field_0x6,
                    );
                }
                uVar7 = uVar7 | value_00;
                if (uVar7 != 0) {
                    if (!bVar14) {
                        big_switch_statement_1020_c0d8(local_120);
                        SetDlgItemText16(
                            CONCAT22(uVar10, uVar7),
                            local_114,
                            local_struct_1.field_0x6,
                        );
                    }
                    SetDlgItemInt16(0, value_00, local_116, local_struct_1.field_0x6);
                    iVar11 = local_struct_1.field_0xea;
                    HVar8 = GetDlgItem16(local_114, local_struct_1.field_0x6);
                    (&local_struct_1.field_0x9a + iVar11 * 2) = HVar8;
                    piVar1 = &local_struct_1.field_0xea;
                    unsafe {
                        *piVar1 = *piVar1 + 1;
                    }
                    iVar11 = local_struct_1.field_0xea;
                    HVar8 = GetDlgItem16(local_116, local_struct_1.field_0x6);
                    (&local_struct_1.field_0x9a + iVar11 * 2) = HVar8;
                    piVar1 = &local_struct_1.field_0xea;
                    unsafe {
                        *piVar1 = *piVar1 + 1;
                    }
                    bVar14 = true;
                }
                if (bVar14) {
                    local_114 = local_114 + 1;
                    local_116 = local_116 + 1;
                }
                local_120 = local_120 + 1;
            }
        }
    }
    return;
}

pub fn msg_box_1040_a85a(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x7ef,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7f0,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7f1,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7f2,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_DX, in_AX));
    return;
}

pub fn set_dialog_item_1040_a84a(param_1: *mut astruct_351) {
    set_dialog_item_1040_a94a(param_1);
    return;
}

pub fn win_fn_1040_a784(
    param_1: *mut astruct_124,
    param_2: *mut astruct_124,
    param_3: u16_00,
    param_3: u32,
) {
    let h_wnd: HWND16;
    let paVar1: *mut astruct_124;

    paVar1 = param_1;
    if (param_3._2_2_ != 0xeb) {
        if (param_3._2_2_ == 500) {
            msg_box_1040_a85a(param_1, param_2);
            return;
        }
        if (param_3._2_2_ == 0x1f7) {
            _PTR_LOOP_1050_5ef0 = (param_1 + 1);
            pass1_1038_af40(_g_astruct_112_a, *&param_1.field_0x8, 0x23);
            PostMessage16(0, 2, 0x111, &param_1.field_0x6);
            return;
        }
        if (param_3._2_2_ != 0x17d8) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        SetWindowPos16(6, 0xed, 0x237, 0, 0, 0, &param_1.field_0x6);
        h_wnd = GetDlgItem16(0x17d8, &param_1.field_0x6);
        paVar1 = offset;
        EnableWindow16(0, h_wnd);
        &param_1[1].field_0x4 = 1;
        param_2 = param_1;
    }
    set_dialog_item_1040_a94a(CONCAT22(param_2, paVar1));
    return;
}

pub fn win_fn_1020_0dc4(in_struct_1: *mut win_struct_42, param_2: u16, param_3: u32) {
    let mut iVar1: i32;
    let local_struct_1_hi: *mut win_struct_42;

    call_load_cursor_1020_790e(in_struct_1, s_PCPOPMENU_1050_4256, param_2, param_3);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    iVar1 = in_struct_1;
    (iVar1 + 0xf2) = 0;
    (iVar1 + 0xf6) = 0;
    (iVar1 + 0xfa) = 0;
    in_struct_1.u16_x0 = 0x1384;
    (iVar1 + 2) = 0x1020;
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | (iVar1 + 0x5b)),
        s_VrMode_1050_4260,
    );
    (iVar1 + 0xac) = 0x44c00000;
    update_window_1020_10a0(in_struct_1);
    return;
}

pub fn win_fn_1020_09ba(in_struct_1: *mut win_struct_42) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let in_struct_1_00: *mut astruct_636;
    let mut u_var1: u32;
    let mut local_4: u16;

    in_struct_1_00 = create_win_1008_9760(in_struct_1);
    process_struct_1000_179c(0xe, (in_struct_1_00 >> 0x10));
    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    if (in_struct_1_00 != 0x0) {
        uVar1 = process_struct_1020_0baa(in_struct_1_00, local_struct_1.win_handle_0x8);
        local_struct_1.u16_xe2 = uVar1;
        local_struct_1.u16_xe4 = (uVar1 >> 0x10);
        return;
    }
    &local_struct_1.u16_xe2 = 0;
    return;
}

pub fn win_fn_1018_e6c6(in_struct_a: *mut win_struct_42) {
    let struct_d: *mut astruct_199;
    let string_base_a: *mut char;
    let struct_a: *mut astruct_199;
    let local_DX_44: *mut u8;
    let struct_c_lo: *mut win_struct_42;
    let struct_c_hi: *mut win_struct_42;
    let struct_b: *mut astruct_199;
    let local_4: *mut u8;

    struct_b = create_win_1008_9760(in_struct_a);
    struct_a = (struct_b >> 0x10);
    struct_c_hi = (in_struct_a >> 0x10);
    struct_c_lo = in_struct_a;
    struct_d = get_gui_dc_1018_4db0(*&struct_c_lo.u32_xf2, struct_c_lo.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    local_DX_44 = (struct_a | struct_d);
    if (local_DX_44 != 0x0) {
        string_base_a._0_1_ =
            pass1_1018_e834(struct_d, CONCAT22(struct_c_lo.win_handle_0x8, struct_a));
        string_base_a._0_2_ = CONCAT11(string_base_a._1_1_, string_base_a);
        struct_c_lo.char_ptr_16_0xee = string_base_a;
        struct_c_lo.field_0xf0 = local_DX_44;
        return;
    }
    &struct_c_lo.char_ptr_16_0xee = 0;
    return;
}

pub fn win_fn_1018_e384(in_win_struct_42_ptr: *mut win_struct_42) {
    let in_struct_622_ptr: *mut astruct_622;
    let struct_a: *mut astruct_199;
    let local_win_struct_42_ptr_1: *mut win_struct_42;
    let mut local_win_struct_42_ptr_1_hi: u16;
    let paVar1: *mut astruct_199;
    let mut local_DXAX_61: u32;
    let mut local_4: u16;

    paVar1 = create_win_1008_9760(in_win_struct_42_ptr);
    struct_a = (paVar1 >> 0x10);
    local_win_struct_42_ptr_1_hi = (in_win_struct_42_ptr >> 0x10);
    local_win_struct_42_ptr_1 = in_win_struct_42_ptr;
    in_struct_622_ptr = get_gui_dc_1018_4db0(
        *&local_win_struct_42_ptr_1.u32_xf2,
        local_win_struct_42_ptr_1.win_handle_0x8,
    );
    process_struct_1000_179c(0x18, struct_a);
    if ((struct_a | in_struct_622_ptr) != 0) {
        local_DXAX_61 = draw_fn_1018_e4f2(
            in_struct_622_ptr,
            CONCAT22(local_win_struct_42_ptr_1.win_handle_0x8, struct_a),
        );
        local_win_struct_42_ptr_1.char_ptr_16_0xee = local_DXAX_61;
        local_win_struct_42_ptr_1.field_0xf0 = (local_DXAX_61 >> 0x10);
        return;
    }
    &local_win_struct_42_ptr_1.char_ptr_16_0xee = 0;
    return;
}

pub fn win_fn_1018_6adc(param_1: u32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let uVar3: u8;
    let mut uVar4: u16;
    let extraout_DL: u8;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let ppVar7: *mut pass1_struct_1;
    let mut in_stack_0000ffdc: u32;
    let in_string_1: *mut char;
    let mut local_6: u16;
    let mut local_4: u16;

    in_string_1 = CONCAT22((in_stack_0000ffdc >> 0x10), 0x48);
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, in_string_1);
    uVar6 = (ppVar7 >> 0x10);
    iVar1 = (ppVar7 + 10);
    iVar2 = (ppVar7 + 0xc);
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (1 < iVar1 - (iVar5 + 0xe6)) {
        (iVar5 + 0xe2) = iVar1 / 2 - ((iVar5 + 0xe6) + 1) / 2;
    }
    if (1 < iVar2 - (iVar5 + 0xe8)) {
        (iVar5 + 0xe4) = iVar2 / 2 - ((iVar5 + 0xe8) + 1) / 2;
    }
    ShowCursor16(0x1010, 0, (in_string_1 >> 0x10));
    if ((iVar5 + 0xee) != 0) {
        uVar4 = mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, (iVar5 + 0xee));
        (iVar5 + 0xf0) = uVar4;
    }
    uVar3 = mixed_fn_1010_830a(_g_struct_73_1050_14cc, (iVar5 + 0xec));
    sound_fn_1008_53ae(uVar3, extraout_DL, (iVar5 + 8));
    ShowCursor16(8, 1);
    win_func_1018_6bb6(param_1, (param_1 >> 0x10));
    return;
}

pub fn win_fn_1018_5e9a(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    char * *ppcVar2;
    let mut uVar3: i32;
    let iVar4: u16;
    let puVar5: *mut u16;
    let struct_a: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let ppVar10: *mut pass1_struct_1;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 6];
    let mut local_8: u16;
    let mut local_6: u32;

    win_gui_func_1040_78e2(param_1);
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffb2, 0x39));
    struct_a = (ppVar10 >> 0x10);
    uVar3 = ppVar10;
    uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    (iVar6 + 0x8e) = uVar3;
    (iVar6 + 0x90) = struct_a;
    process_struct_1000_179c(0x12, struct_a);
    if ((struct_a | uVar3) == 0) {
        (iVar6 + 0x92) = 0;
    } else {
        pass1_1018_6198(CONCAT22(struct_a, uVar3), param_1, (iVar6 + 6));
        (iVar6 + 0x92) = uVar3;
        (iVar6 + 0x94) = extraout_DX;
    }
    uVar1 = (iVar6 + 0x8e);
    local_6 = uVar1 & 0xffff0000 | (uVar1 + 10);
    GetClientRect16(CONCAT22(unaff_SS, local_e), (iVar6 + 6));
    iVar4 = GetSystemMetrics16(4);
    (local_6 + 6) = iVar4 + local_8;
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffb6, 0x48));
    local_14 = (ppVar10 >> 0x10);
    local_16 = ppVar10;
    local_10 = (local_16 + 10);
    local_12 = (local_16 + 0xc);
    uVar9 = (local_6 >> 0x10);
    (local_6 + 2) = (local_12 - (local_6 + 6)) / 2;
    local_6 = local_10 / 2 + 3;
    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_28),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x100,
    );
    while (true) {
        puVar5 = &local_28;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar5));
        if ((extraout_DX_00 | puVar5) == 0) {
            break;
        }
        ppcVar2 = (puVar5 + 8);
        if (ppcVar2 != 0x0) {
            process_string_1000_3cea((param_1 & 0xffff0000 | (iVar6 + 0x10)), *ppcVar2);
        }
    }
    uVar9 = (local_6 >> 0x10);
    iVar7 = local_6;
    SetWindowPos16(
        0x44,
        (iVar7 + 6),
        (iVar7 + 4),
        (iVar7 + 2),
        local_6,
        0,
        (iVar6 + 6),
    );
    return;
}

pub fn win_cleanup_1018_4d22(param_1: *mut astruct_376) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let h_gdi_obj: HPALETTE16;
    let local_BX_4: *mut astruct_43;
    let mut uVar4: u16;
    let mut unaff_CS: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.ptr_a_lo = (s_SinternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12);
    local_BX_4.field_0x2 = 0x1018;
    if (local_BX_4.h_dc != 0) {
        h_gdi_obj = SelectPalette16(0, local_BX_4.palette, local_BX_4.h_dc);
        DeleteObject16(h_gdi_obj);
        unaff_CS = SUB42(offset, 0);
        DeleteDC16(local_BX_4.h_dc);
    }
    puVar1 = local_BX_4.field_0xa;
    uVar2 = local_BX_4.field_0xc;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
        }
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    puVar1 = local_BX_4.field_0xe;
    uVar2 = local_BX_4.field_0x10;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
        }
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    _PTR_LOOP_1050_4230 = 0;
    pass1_1010_1d80(param_1);
    return;
}

pub fn win_fn_1018_2978(param_1: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut in_AX: u16;
    let puVar3: *mut u8;
    let puVar4: *mut u8;
    let puVar5: *mut u16;
    let mut iVar6: i32;
    let mut extraout_DX: u16;
    let struct_a: *mut astruct_199;
    let paVar7: *mut astruct_199;
    let mut extraout_DX_00: u16;
    let mut uVar8: u16;
    let struct_a_00: *mut astruct_199;
    let mut extraout_DX_01: u16;
    let extraout_DX_02: *mut astruct_199;
    let mut extraout_DX_03: u16;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let uVar12: u8;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: [u8; 36];
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_fn_1010_8096(_g_struct_73_1050_14cc, 1);
    puVar3 = local_2a;
    uVar12 = (unaff_SS >> 8);
    local_4 = extraout_DX;
    process_struct_1008_48fe(
        CONCAT13(uVar12, CONCAT12(unaff_SS, puVar3)),
        1,
        CONCAT22(extraout_DX, in_AX),
    );
    uVar11 = 0x1000;
    paVar7 = struct_a;
    process_struct_1000_179c(0x1e, struct_a);
    if ((paVar7 | puVar3) == 0) {
        puVar4 = 0x0;
        uVar8 = 0;
    } else {
        puVar4 = local_2a;
        uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
        pass1_1008_3f92(CONCAT22(paVar7, puVar3), CONCAT22(unaff_SS, puVar4));
        uVar8 = extraout_DX_00;
    }
    _local_2e = CONCAT22(uVar8, puVar4);
    ppcVar2 = (*_local_2e + 0x14);
    ppcVar2(uVar11, puVar4, uVar8);
    _local_32 = CONCAT22(struct_a_00, puVar4);
    paVar7 = struct_a_00;
    process_struct_1000_179c(0x14, struct_a_00);
    if ((paVar7 | puVar4) == 0) {
        puVar4 = 0x0;
        uVar11 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(paVar7, puVar4));
        uVar11 = extraout_DX_01;
    }
    uVar10 = (param_1 >> 0x10);
    iVar9 = param_1;
    (iVar9 + 0xe) = puVar4;
    (iVar9 + 0x10) = uVar11;
    pass1_1008_4d84((iVar9 + 0xe), _local_32);
    puVar5 = &local_3a;
    paVar7 = extraout_DX_02;
    GetClientRect16(CONCAT13(uVar12, CONCAT12(unaff_SS, puVar5)), g_h_window);
    uVar11 = 0x1000;
    process_struct_1000_179c(0x1e, paVar7);
    if ((paVar7 | puVar5) == 0) {
        (iVar9 + 10) = 0;
    } else {
        iVar6 = (local_34 - local_38) + 1;
        uVar1 = (iVar9 + 0xe);
        uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
        pass1_1008_405c(
            puVar5,
            paVar7,
            uVar1,
            (uVar1 >> 0x10),
            iVar6,
            (local_36 - local_3a) + 1,
        );
        (iVar9 + 10) = iVar6;
        (iVar9 + 0xc) = extraout_DX_03;
    }
    if (_local_2e != 0x0) {
        ppcVar2 = *_local_2e;
        ppcVar2(
            uVar11,
            _local_2e,
            (_local_2e >> 0x10),
            1,
            uVar8,
            _local_2e,
            _local_2e,
        );
    }
    close_file_1008_496c(local_2a);
    return;
}

pub fn win_fn_1010_71d6(param_1: u32, param_2: u16, param_3: *mut u16) {
    let mut u_var1: u32;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut in_AX: i32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut extraout_DX: i32;
    let mut uVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar7 = (param_1 >> 0x10);
    pass1_1010_ad22((param_1 + 0x14));
    _local_6 = CONCAT22(extraout_DX, in_AX);
    if ((extraout_DX | in_AX) == 0) {
        return;
    }
    uVar8 = pass1_1030_73a8(CONCAT22(extraout_DX, in_AX));
    uVar6 = (uVar8 >> 0x10);
    uVar2 = uVar8;
    if (((uVar6 | uVar2) != 0) && ((uVar2 + 0x1c) == 0x8000002)) {
        return;
    }
    uVar1 = (in_AX + 0x2e);
    local_e._0_2_ = uVar1;
    if ((((in_AX + 0x30) | local_e) != 0) && ((local_e + 0x200) == 0x8000002)) {
        return;
    }
    uVar1 = (param_1 + 0x14);
    uVar4 = pass1_1010_b028(uVar1, (uVar1 >> 0x10), uVar8);
    uVar5 = (uVar2 + 0x12);
    uVar3 = uVar5;
    if ((uVar5 != 4) && (uVar3 = param_2, uVar5 == 7)) {
        param_2 = 5;
        uVar3 = param_2;
    }
    param_2 = uVar3;
    uVar5 = param_2 - 2;
    if (uVar5 != 0) {
        if (param_2 == 3) {
            uVar5 = uVar4 - 0xb;
            if ((uVar5 == 0) || (uVar5 = uVar4 - 0x37, uVar5 == 0)) {
                local_14 = 0xb;
            } else {
                local_14 = 10;
            }
            // goto LAB_1010_72a7;
        }
        uVar5 = param_2 - 4;
        if (uVar5 == 0) {
            local_14 = 0x17;
            // goto LAB_1010_72a7;
        }
        uVar5 = param_2 - 5;
        if (uVar5 != 0) {
            pass1_1010_7818(param_1, uVar8);
            local_14 = uVar5;
            // goto LAB_1010_72a7;
        }
    }
    local_14 = 0xc;
    // LAB_1010_72a7:
    if (local_14 == 0) {
        return;
    }
    win_gui_fn_1010_79aa(param_1, 0, _local_6);
    if (uVar5 == 0) {
        window_msg_func_1010_7300(param_1, 0, 0, local_14, _local_6);
    }
    return;
}

pub fn win_fn_1010_7174(param_1: u32, uparam_2: i32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 0x13) {
        uVar1 = (iVar2 + 0x18);
        destroy_win_1010_7b26(param_1 & 0xffff0000 | (iVar2 - 10), (uVar1 + 0x28));
        return;
    }
    if (param_2 < 0x14) {
        if (param_2 == 0x1) {
            (iVar2 + 10) = 0;
            (iVar2 + 0x18) = 0;
            return;
        }
        if (param_2 == '\x05') {
            send_msg_1010_7c42(param_1 & 0xffff0000 | (iVar2 - 10));
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_fn_1040_c886(param_1: u32, param_2: u8, param_3: HDC16) {
    let ppcVar1: fn();
    let p_uvar2: *mut u16;
    let h_gdi_obj: HPALETTE16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (((iVar3 + 10) | (iVar3 + 8)) != 0) {
        local_4 = 0;
        if ((iVar3 + 0x46) == 0) {
            uVar5 = (_PTR_LOOP_1050_4230 >> 0x10);
            local_c = (_PTR_LOOP_1050_4230 + 0xe);
            puVar2 = &param_3;
            unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
            realize_palette_1008_4e08(local_c, (_PTR_LOOP_1050_4230 + 0x10), puVar2, unaff_SS);
            local_4 = puVar2;
        }
        local_8 = (iVar3 + 8);
        uVar5 = (iVar3 + 10);
        if ((((iVar3 + 0xe) | (iVar3 + 0xc)) != 0) && ((param_2 & 1) != 0)) {
            local_8 = (iVar3 + 0xc);
            uVar5 = (iVar3 + 0xe);
        }
        if (((iVar3 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_8 = (iVar3 + 0x10);
            uVar5 = (iVar3 + 0x12);
        }
        ppcVar1 = (local_8 + 4);
        (**ppcVar1)(
            unaff_CS,
            local_8,
            uVar5,
            (iVar3 + 0x28),
            (iVar3 + 0x26),
            &param_3,
        );
        if ((iVar3 + 0x46) == 0) {
            h_gdi_obj = SelectPalette16(0, local_4, param_3);
            DeleteObject16(h_gdi_obj);
        }
    }
    return;
}

pub fn win_fn_1040_c028(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let mut uVar9: u16;
    let unaff_CS: bool;
    let unaff_SS: *mut u16;
    let rect: *mut RECT16;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let puVar8: *mut u16;

    iVar7 = param_1;
    uVar9 = (param_1 >> 0x10);
    if (param_2 == 8) {
        GetClientRect16(CONCAT22(unaff_SS, &local_a), (iVar7 + 4));
        uVar1 = (iVar7 + 6);
        uVar3 = (iVar7 + 6);
        iVar5 = (uVar3 + 0x16);
        uVar3 = (iVar7 + 6);
        local_a = (uVar3 + 0x1a);
        uVar3 = (iVar7 + 6);
        local_8 = (uVar3 + 0x1c);
        if (iVar5 != 0) {
            if (iVar5 < 2) {
                iVar4 = 1;
            } else {
                iVar4 = 2;
            }
            uVar2 = ((iVar5 - iVar4) * 4 + uVar1 + 0x2a);
            iVar5 = uVar2;
            uVar2 = uVar2 & 0xffff0000;
            local_a = (iVar5 + 0x22) + (uVar2 | iVar5 + 0x1e);
        }
        uVar1 = (iVar7 + 6);
        local_6 = (uVar1 + 0x1e);
        local_4 = local_4 - 5;
    } else {
        if (param_2 != 9) {
            if (param_2 != 10) {
                return;
            }
            uVar1 = (iVar7 + 6);
            uVar6 = uVar1 + 0x2a;
            if (((iVar7 + 8) | uVar6) == 0) {
                return;
            }
            uVar3 = (iVar7 + 6);
            uVar2 = (((uVar3 + 0x16) + -1) * 4 + uVar6);
            iVar7 = uVar2;
            uVar2 = uVar2 & 0xffff0000;
            puVar8 = (uVar2 | iVar7 + 0x1e);
            uVar9 = (uVar2 >> 0x10);
            local_8 = (iVar7 + 0x20) - 8;
            unsafe {
                local_a = *puVar8;
            }
            unsafe {
                local_6 = (iVar7 + 0x22) + *puVar8;
            }
            local_4 = (iVar7 + 0x20);
            rect = &local_a;
            unaff_CS = 0;
            // goto LAB_1040_c19d;
        }
        local_a = 0;
        local_8 = 0;
        local_6 = 0;
        local_4 = 0;
        GetClientRect16(CONCAT22(unaff_SS, &local_a), (iVar7 + 4));
        uVar1 = (iVar7 + 6);
        local_a = (uVar1 + 0x1a);
        uVar1 = (iVar7 + 6);
        local_6 = (uVar1 + 0x1e);
        local_4 = local_4 - 5;
        uVar1 = (iVar7 + 6);
        uVar3 = (iVar7 + 6);
        iVar7 = (uVar3 + 0x16);
        if (0 < iVar7) {
            uVar1 = (uVar1 + iVar7 * 4 + 0x26);
            iVar7 = uVar1;
            uVar9 = (uVar1 >> 0x10);
            local_8 = (iVar7 + 0x20) + (iVar7 + 0x24);
        }
    }
    unaff_SS = &local_a;
    rect = (&PTR_LOOP_1050_0000 + 1);
    // LAB_1040_c19d:
    InvalidateRect16(unaff_CS, rect, unaff_SS);
    return;
}

pub fn post_win_msg_1040_d2ac(
    param_1: *mut astruct_124,
    param_2: u16_00,
    param_3: u16,
    param_2: u32,
) {
    let LVar1: LRESULT;

    if (param_2._2_2_ == (s_dibtext_bmp_1050_1844 + 4)) {
        SendDlgItemMessage16(
            0,
            0,
            0x405,
            (s_dibtext_bmp_1050_1844 + 3),
            &param_1.field_0x6,
        );
        pass1_1010_9172(&param_1[1].field_0x8);
    } else {
        if ((s_dibtext_bmp_1050_1844 + 4) < param_2._2_2_) {
            if (param_2._2_2_ == (s_dibtext_bmp_1050_1844 + 5)) {
                LVar1 = SendDlgItemMessage16(
                    0,
                    0,
                    0x40c,
                    (s_dibtext_bmp_1050_1844 + 3),
                    &param_1.field_0x6,
                );
                if ((LVar1 != -1) || ((LVar1 >> 0x10) != -1)) {
                    SendDlgItemMessage16(
                        0,
                        LVar1 - 1,
                        0x403,
                        (s_dibtext_bmp_1050_1844 + 3),
                        &param_1.field_0x6,
                    );
                    pass1_1010_91cc(&param_1[1].field_0x8);
                }
            } else {
                if (param_2._2_2_ == (s_dibtext_bmp_1050_1844 + 6)) {
                    enable_window_1040_d6be(param_1, param_2_00);
                    pass1_1018_57d2((param_1 + 1), CONCAT22(param_2_00, param_1));
                    PostMessage16(0, 0x203, 0x111, g_h_window);
                } else {
                    if (param_2._2_2_ != (s_dibtext_bmp_1050_1844 + 7)) {}
                    // goto LAB_1040_d3b3;
                    _PTR_LOOP_1050_5a68 = &param_1[1].field_0x4;
                    pass1_1038_af40(_g_astruct_112_a, *&param_1.field_0x6, 0x27);
                }
            }
        } else {
            if (param_2._2_2_ == 0xeb) {
                send_dialog_item_msg_1040_d79c(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
                    // LAB_1040_d3b3:
                    win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                    return;
                }
                msg_box_1040_d3d0(param_1, param_2_00);
            }
        }
    }
    return;
}

pub fn win_cleanup_1040_d1bc(param_1: *mut astruct_44) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.ptr_a_lo = 0xd8c4;
    (iVar4 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(iVar4 + 6));
    puVar1 = (iVar4 + 0x9c);
    uVar2 = (iVar4 + 0x9e);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
        }
        (**ppcVar3)(&PTR_LOOP_1050_1038, puVar1, uVar2, 1);
    }
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn win_fn_1040_cace(param_1: u32) {
    let mut u_var1: u32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let iVar4: u16;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let unaff_SS: u16;
    let mut bVar7: bool;
    let mut b: u16;
    let mut uVar8: u16;
    let mut local_218: u16;
    let mut local_214: u16;
    let mut local_20c: u32;
    let mut local_208: [u8; 256];
    let mut local_108: [u8; 256];
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    local_6 = GetDlgItemInt16(0, &local_4, unaff_SS, 0x18e);
    local_8 = GetDlgItemInt16(0, &local_4, unaff_SS, 0x191);
    if (local_6 == 0) {
        return;
    }
    pass1_1018_50ea((iVar5 + 0x98), local_6, (iVar5 + 0x94));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_208),
        0x57b,
    );
    uVar1 = (iVar5 + 0x94);
    b = (_g_struct_73_1050_14cc >> 0x10);
    if ((uVar1 + 0x36) == 0) {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_SS, local_108),
            0x7ed,
        );
        iVar3 = MessageBox16(
            0x34,
            CONCAT22(unaff_SS, local_208),
            CONCAT22(unaff_SS, local_108),
            (iVar5 + 8),
        );
    } else {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_SS, local_108),
            0x7ec,
        );
        iVar3 = MessageBox16(
            0x34,
            CONCAT22(unaff_SS, local_208),
            CONCAT22(unaff_SS, local_108),
            (iVar5 + 8),
        );
    }
    bVar2 = iVar3 == 6;
    bVar7 = false;
    if ((!bVar2) && (uVar1 = (iVar5 + 0x94), (uVar1 + 0x34) < 1)) {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_SS, local_108),
            0x7ee,
        );
        iVar4 = MessageBox16(
            0x34,
            CONCAT22(unaff_SS, local_208),
            CONCAT22(unaff_SS, local_108),
            (iVar5 + 8),
        );
        bVar7 = iVar4 == 6;
        bVar2 = false;
    }
    if (bVar2) {
        _PTR_LOOP_1050_5f16 = (iVar5 + 0x94);
        uVar8 = 0x26;
    } else {
        if (!bVar7) {
            return;
        }
        _PTR_LOOP_1050_5a68 = (iVar5 + 0x94);
        uVar8 = 0x27;
    }
    pass1_1038_af40(_g_astruct_112_a, *(iVar5 + 8), uVar8);
    return;
}

pub fn destroy_win_1040_caa6(param_1: u16, param_2: u32) {
    let paVar1: *mut astruct_318;
    let BVar2: bool;

    BVar2 = 0;
    paVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x2b);
    pass1_1010_038e(paVar1, BVar2);
    destroy_window_1040_b726(param_1, param_2);
    return;
}

pub fn win_fn_1008_84f4(param_1: u16, uparam_2_00: i32, param_2: WPARAM16, param_3: u32) {
    let pbVar1: *mut byte;
    let mut uVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut cVar5: u8;
    let BVar6: bool;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let in_struct_1: *mut astruct_44;
    let uVar8: u8;
    let HVar9: HWND16;
    let hwnd: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;

    hwnd = &g_alloc_addr_1050_1050;
    HVar9 = param_3._2_2_;
    in_struct_1 = GetWindowLong16(0, param_3._2_2_);
    uVar7 = (in_struct_1 >> 0x10);
    iVar4 = in_struct_1;
    if (param_3 == 0x1f) {
        (iVar4 + 4) = 0;
        KillTimer16(0xfa6, param_3._2_2_);
        KillTimer16(0xfa7, param_3._2_2_);
        ReleaseCapture16(hwnd);
    } else {
        cVar5 = param_3;
        uVar8 = (param_3 >> 0x10);
        if (param_3 < 0x20) {
            if (param_3 != 0x14) {
                if (0x14 < param_3) {}
                // goto LAB_1008_8771;
                if (cVar5 == 0x1) {
                    // LAB_1008_8560:
                    win_fn_1008_8214(uVar8, cVar5, param_2, param_1, param_2_00);
                    return;
                }
                if (cVar5 == 0x2) {
                    error_check_1000_17ce(in_struct_1);
                } else {
                    if (cVar5 != 0xc) {
                        if (cVar5 != 0xf) {}
                        // goto LAB_1008_8771;
                        draw_1008_8288(param_3._2_2_, iVar4, uVar7);
                    }
                }
            }
        } else {
            if (param_3 == 0x200) {
                if ((*(iVar4 + 4) & 1) != 0) {
                    GetClientRect16(CONCAT22(unaff_SS, &stack0xfff6), param_3._2_2_);
                    iVar3 = (iVar4 + 4);
                    pbVar1 = (iVar4 + 4);
                    unsafe {
                        *pbVar1 = *pbVar1 & 0xf3;
                    }
                    BVar6 = PtInRect16(CONCAT22(param_2_00, param_1), &stack0xfff6);
                    if (BVar6 == 0) {
                        pbVar1 = (iVar4 + 4);
                        unsafe {
                            *pbVar1 = *pbVar1 | 2;
                        }
                    } else {
                        if (param_2_00 < local_4 >> 1) {
                            pbVar1 = (iVar4 + 4);
                            unsafe {
                                *pbVar1 = *pbVar1 | 4;
                            }
                        } else {
                            pbVar1 = (iVar4 + 4);
                            unsafe {
                                *pbVar1 = *pbVar1 | 8;
                            }
                        }
                        pbVar1 = (iVar4 + 4);
                        unsafe {
                            *pbVar1 = *pbVar1 & 0xfd;
                        }
                    }
                    if ((iVar4 + 4) != iVar3) {
                        InvalidateRect16(1, 0x0, 0);
                        UpdateWindow16(param_3._2_2_);
                    }
                }
            } else {
                if (param_3 < 0x201) {
                    if (param_3 == 0x81) {}
                    // goto LAB_1008_8560;
                    if (param_3 != 0x113) {
                        // LAB_1008_8771:
                        DefWindowProc16(
                            CONCAT13((param_2_00 >> 8), CONCAT12(param_2_00, param_1)),
                            param_2,
                            param_3,
                            param_3._2_2_,
                        );
                        return;
                    }
                    if (param_2 == 0xfa6) {
                        KillTimer16(0xfa6, param_3._2_2_);
                        SetTimer16(0, 0, 1, 0xfa7);
                        HVar9 = param_3._2_2_;
                    }
                    if ((*(iVar4 + 4) & 2) == 0) {
                        send_win_msg_1008_84ba(uVar8, iVar4, uVar7, HVar9);
                    }
                } else {
                    if (param_3 != 0x201) {
                        if (param_3 == 0x202) {
                            KillTimer16(0xfa6, param_3._2_2_);
                            KillTimer16(0xfa7, param_3._2_2_);
                            ReleaseCapture16(hwnd);
                            uVar2 = (iVar4 + 4);
                            if (((uVar2 & 1) != 0) && ((uVar2 & 0xfffd) != 0)) {
                                pbVar1 = (iVar4 + 4);
                                unsafe {
                                    *pbVar1 = *pbVar1 & 0xf2;
                                }
                                InvalidateRect16(1, 0x0, 0);
                                UpdateWindow16(param_3._2_2_);
                            }
                            SendMessage16(*(iVar4 + 2), 0xf9, 0x111, in_struct_1.ptr_a_lo);
                            return;
                        }
                        if (param_3 != 0x203) {}
                        // goto LAB_1008_8771;
                    }
                    pbVar1 = (iVar4 + 4);
                    unsafe {
                        *pbVar1 = *pbVar1 | 1;
                    }
                    GetClientRect16(CONCAT22(unaff_SS, &stack0xfff6), param_3._2_2_);
                    if (param_2_00 < (local_4 >> 1)) {
                        pbVar1 = (iVar4 + 4);
                        unsafe {
                            *pbVar1 = *pbVar1 | 4;
                        }
                    } else {
                        pbVar1 = (iVar4 + 4);
                        unsafe {
                            *pbVar1 = *pbVar1 | 8;
                        }
                    }
                    send_win_msg_1008_84ba(param_3._2_2_, iVar4, uVar7);
                    SetTimer16(0, 0, 300, 0xfa6);
                    InvalidateRect16(1, 0x0, 0);
                    UpdateWindow16(param_3._2_2_);
                    SetCapture16(param_3._2_2_);
                }
            }
        }
    }
    return;
}

pub fn win_fn_1008_8214(param_1: HWND16, param_2: i32, param_3: u16_00, param_3: u32) -> u16 {
    let mut in_AX: i32;
    let iVar1: u16;
    let in_DX: *mut astruct_199;
    let mut uVar2: u32;
    let puVar3: *mut u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x81) {
        process_struct_1000_179c(6, in_DX);
        if ((in_DX | in_AX) == 0) {
            uVar2 = 0;
        } else {
            uVar2 = clear_list_elements_1008_80d2(CONCAT22(in_DX, in_AX));
        }
        SetWindowLong16(uVar2, (uVar2 >> 0x10));
    }
    if (param_2 == 1) {
        puVar3 = GetWindowLong16(0, param_1);
        unsafe {
            *puVar3 = (param_3 + 8);
        }
        iVar1 = GetDlgCtrllID16(param_1);
        (puVar3 + 2) = iVar1;
    }
    return 1;
}

pub fn win_fn_1008_5f44(param_1: u16, param_2: u32, param_3: u32) -> LRESULT {
    let wVar1: u16;
    let LVar2: LRESULT;
    let paVar3: *mut astruct_219;
    let mut in_stack_0000fff8: u16;

    if (param_3 == 2) {
        wVar1 = GetWindowWord16(0, param_3._2_2_);
        mci_send_command_1008_5cb6(
            _g_struct_ptr_1050_02a0,
            (_g_struct_ptr_1050_02a0 >> 0x10),
            wVar1,
        );
        paVar3 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff8, 0x37));
        pass1_1008_aa28(paVar3, paVar3);
    } else {
        if (param_3 != 0x3b9) {
            LVar2 = DefWindowProc16(
                CONCAT22(param_2, param_1),
                (param_2 >> 0x10),
                param_3,
                param_3._2_2_,
            );
            return LVar2;
        }
        DestroyWindow16(param_3._2_2_);
    }
    return 0;
}

pub fn win_fn_1008_3bd6(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) {
    make_proc_inst_1040_8fb8(
        param_1,
        param_3,
        0,
        param_5,
        param_5._2_2_,
        param_6,
        param_6._2_2_,
        param_7,
    );
    CONCAT22(param_2, param_1) = 0x3cfc;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    (param_1 + 0x36) = 0;
    (param_1 + 0x26) = 0;
    process_struct_1040_9252(CONCAT22(param_2, param_1));
    create_win_1040_92dc(CONCAT22(param_2, param_1));
    update_window_1040_93aa(CONCAT22(param_2, param_1), param_3_00);
    return;
}

pub fn win_func_1008_3c34(param_1: u32, param_2: u8, param_3: HDC16) {
    let ppcVar1: fn();
    let h_gdi_obj: HPALETTE16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u32;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 10) | (iVar2 + 8)) != 0) {
        local_6 = (iVar2 + 8);
        if (((iVar2 + 0xc) != 0) && ((param_2 & 1) != 0)) {
            local_6 = (iVar2 + 0xc);
        }
        if (((iVar2 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_6 = (iVar2 + 0x10);
        }
        uVar3 = (_PTR_LOOP_1050_4230 >> 0x10);
        local_a = (_PTR_LOOP_1050_4230 + 0xe);
        local_c = &param_3;
        realize_palette_1008_4e08(local_a, (_PTR_LOOP_1050_4230 + 0x10), local_c, unaff_SS);
        ppcVar1 = (local_6 + 4);
        (**ppcVar1)();
        h_gdi_obj = SelectPalette16(0, local_c, param_3);
        DeleteObject16(h_gdi_obj);
    }
    return;
}

pub fn win_fn_1008_016e(param_1: u32) {
    let ppcVar1: fn();
    let p_uvar2: *mut u16;
    let mut cVar3: u8;
    let mut in_AX: i32;
    let puVar4: *mut u8;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let extraout_var_03: u32;
    let struct_a: *mut astruct_199;
    let mut uVar8: i32;
    let extraout_DX: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let paVar9: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let paVar10: *mut astruct_199;
    let extraout_DX_02: *mut astruct_199;
    let extraout_DX_03: *mut astruct_199;
    let extraout_DX_04: *mut astruct_199;
    let extraout_DX_05: *mut astruct_199;
    let mut extraout_DX_06: u16;
    let mut uVar11: u16;
    let mut iVar12: i32;
    let mut uVar13: u16;
    let mut unaff_SS: u16;
    let uVar14: u8;
    let uVar15: u8;
    let mut in_stack_0000fe46: u16;
    let mut local_13e: [u8; 172];
    let mut local_92: [u8; 128];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    GetVersion16();
    local_8 = in_AX & 0xff;
    uVar6 = local_8;
    local_a = in_AX >> 8;
    local_4 = struct_a;
    if ((local_8 < 3) || ((paVar9 = struct_a, local_8 == 3 && (local_a < 10)))) {
        uVar13 = 0x1000;
        local_10 = struct_a;
        process_struct_1000_179c(0xb4, struct_a);
        local_12 = uVar6;
        uVar8 = local_10 | local_12;
        if (uVar8 == 0) {
            uVar6 = 0;
            uVar8 = 0;
        } else {
            uVar13 = SUB42(&PTR_LOOP_1050_1040, 0);
            mixed_1040_8520(
                (uVar6 & 0xffff | local_10 << 0x10),
                0,
                0x10,
                2,
                0x5de,
                0x5dd,
            );
        }
        _local_e = (uVar6 & 0xffff | uVar8 << 0x10);
        ppcVar1 = (*_local_e + 0x74);
        (**ppcVar1)(uVar13, uVar6, uVar8);
        paVar9 = extraout_DX;
        call_fn_ptr_1000_24cd(1);
    }
    fn_1008_6048(s_version__d__d_1050_0012, paVar9, SUB41(uVar6, 0));
    if ((local_8 == 3) && (0xb < local_a)) {
        PTR_LOOP_1050_0010 = (&PTR_LOOP_1050_0000 + 1);
    }
    LoadString16(
        0x80,
        CONCAT22(unaff_SS, local_92),
        0x578,
        g_h_instance_1050_038c,
    );
    puVar4 = local_92;
    dos3_call_1000_51aa(puVar4);
    if (puVar4 != 0x0) {
        uVar14 = unaff_SS;
        uVar15 = (unaff_SS >> 8);
        LoadString16(
            0x80,
            CONCAT13(uVar15, CONCAT12(uVar14, local_13e)),
            0x57b,
            g_h_instance_1050_038c,
        );
        LoadString16(
            0x80,
            CONCAT13(uVar15, CONCAT12(uVar14, &stack0xfe42)),
            0x62e,
            g_h_instance_1050_038c,
        );
        puVar4 = MessageBox16(
            0x10,
            CONCAT13(uVar15, CONCAT12(uVar14, local_13e)),
            CONCAT22(unaff_SS, &stack0xfe42),
            0,
        );
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(4, paVar9);
    if ((paVar9 | puVar4) == 0) {
        puVar5 = 0x0;
        paVar10 = 0x0;
        local_12 = puVar4;
        local_10 = paVar9;
    } else {
        puVar5 = puVar4;
        local_12 = puVar4;
        local_10 = paVar9;
        zero_array_val_1008_5394(CONCAT22(paVar9, puVar4));
        paVar10 = extraout_DX_00;
    }
    uVar13 = (param_1 >> 0x10);
    iVar12 = param_1;
    (iVar12 + 8) = puVar5;
    (iVar12 + 10) = paVar10;
    uVar7 = (iVar12 + 8);
    puVar2 = (iVar12 + 8);
    _PTR_LOOP_1050_0298 = uVar7;
    *puVar2 = 0x70;
    (puVar2 + 2) = offset;
    process_struct_1000_179c(0x126, paVar10);
    uVar8 = uVar7;
    paVar9 = (paVar10 | uVar8);
    local_12 = uVar8;
    local_10 = paVar10;
    if (paVar9 != 0x0) {
        pass1_1010_2024();
        paVar9 = extraout_DX_01;
    }
    if (_g_astruct_372_1050_0ed0 == 0) {
        cVar3 = fn_1008_6048(s_New_failed_in_Op__Op_1050_0020, paVar9, SUB21(uVar8, 0));
        uVar8 = CONCAT31(extraout_var, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xe8c, paVar9);
    paVar10 = (paVar9 | uVar8);
    local_12 = uVar8;
    local_10 = paVar9;
    if (paVar10 != 0x0) {
        pass1_1010_7e40(CONCAT22(paVar9, uVar8));
        paVar10 = extraout_DX_02;
    }
    if (_g_struct_73_1050_14cc == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__ResLibr_1050_0035,
            paVar10,
            SUB21(uVar8, 0),
        );
        uVar8 = CONCAT31(extraout_var_00, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xb0, paVar10);
    paVar9 = (paVar10 | uVar8);
    local_12 = uVar8;
    local_10 = paVar10;
    if (paVar9 != 0x0) {
        pass1_1038_aeca();
        paVar9 = extraout_DX_03;
    }
    if (_g_astruct_112_a == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__DialogCtr_1050_0053,
            paVar9,
            SUB21(uVar8, 0),
        );
        uVar8 = CONCAT31(extraout_var_01, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(10, paVar9);
    paVar10 = (paVar9 | uVar8);
    local_12 = uVar8;
    local_10 = paVar9;
    if (paVar10 != 0x0) {
        make_proc_inst_1038_cf6c();
        paVar10 = extraout_DX_04;
    }
    if (_PTR_LOOP_1050_5bc8 == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__DialogHand_1050_0073,
            paVar10,
            SUB21(uVar8, 0),
        );
        uVar8 = CONCAT31(extraout_var_02, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0x14, paVar10);
    paVar9 = (paVar10 | uVar8);
    local_12 = uVar8;
    local_10 = paVar10;
    if (paVar9 != 0x0) {
        modify_u16_list_1008_5bdc(CONCAT22(paVar10, uVar8));
        paVar9 = extraout_DX_05;
    }
    if (_g_struct_ptr_1050_02a0 == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__Simulator_1050_0097,
            paVar9,
            SUB21(uVar8, 0),
        );
        uVar8 = CONCAT31(extraout_var_03, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xfc, paVar9);
    local_10 = paVar9;
    local_12 = uVar8;
    if ((paVar9 | uVar8) == 0) {
        uVar8 = 0;
        uVar11 = 0;
    } else {
        win_fn_1008_0536();
        uVar11 = extraout_DX_06;
    }
    (iVar12 + 4) = uVar8;
    *(iVar12 + 6) = uVar11;
    if ((iVar12 + 4) == 0) {
        fn_1008_6048(s_New_failed_in_Op__Op_1050_00b7, uVar11, SUB21(uVar8, 0));
        call_fn_ptr_1000_24cd(1);
    }
    reg_class_1008_96d2((iVar12 + 4), in_stack_0000fe46);
    ppcVar1 = ((iVar12 + 4) + 8);
    (**ppcVar1)(0x1000);
    uVar7 = (iVar12 + 4);
    g_h_window = (uVar7 + 8);
    uVar7 = (iVar12 + 4);
    ppcVar1 = ((iVar12 + 4) + 0xc);
    (**ppcVar1)(0x1000, uVar7, (uVar7 >> 0x10), 3);
    uVar7 = (iVar12 + 4);
    UpdateWindow16((uVar7 + 8));
    return;
}

pub fn win_fn_1008_0536(param_1: *mut astruct_180) {
    let mut uVar1: u16;
    let HVar2: HCURSOR16;
    let HVar3: HGDIOBJ16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let ppVar6: *mut pass1_struct_1;
    let HVar7: HINSTANCE16;

    process_struct_1008_3ab8(param_1);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar1 = 0;
    (iVar4 + 0xe0) = 0;
    (iVar4 + 0xe4) = 0;
    (iVar4 + 0xe8) = 0;
    (iVar4 + 0xec) = 0;
    (iVar4 + 0xee) = 0;
    (iVar4 + 0xf2) = 0;
    (iVar4 + 0xf4) = 0;
    (iVar4 + 0xf8) = 0;
    param_1 = s_0_1050_389e;
    (iVar4 + 2) = &PTR_LOOP_1050_1008;
    (iVar4 + 200) = (s_572_bmp_1050_2007 + 1);
    (iVar4 + 0xac) = 0;
    (iVar4 + 0xae) = 0x8700;
    HVar7 = g_h_instance_1050_038c;
    LoadIcon16();
    (iVar4 + 0xc2) = uVar1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (iVar4 + 0xc4) = HVar2;
    HVar3 = GetStockObject16(4);
    (iVar4 + 0xc6) = HVar3;
    process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(HVar7, 0x48));
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (iVar4 + 10)), s_Outpost_1050_00d7);
    ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(HVar7, 0x32));
    (iVar4 + 0xf4) = ppVar6;
    (iVar4 + 0xf6) = (ppVar6 >> 0x10);
    sys_color_func_1008_357e(iVar4, uVar5, 1);
    return;
}

pub fn win_cleanup_1008_0618(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let uVar4: u8;
    let extraout_var: u32;
    let mut uVar5: i32;
    let mut extraout_DX: i32;
    let local_BX_5: *mut astruct_47;
    let mut uVar6: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1 = s_0_1050_389e;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    sys_color_func_1008_357e(param_1, 0);
    error_check_1000_17ce(local_BX_5.field_0xf8);
    if (local_BX_5.field_0xec != 0) {
        DestroyMenu16(0x1000);
    }
    DestroyIcon16(local_BX_5.field_0xc2);
    local_BX_5.field_0xc2 = 0;
    puVar1 = local_BX_5.field_0xe0;
    uVar2 = local_BX_5.field_0xe2;
    uVar5 = uVar2 | puVar1;
    if (uVar5 != 0) {
        unsafe {
            ppcVar3 = *puVar1;
        }
        (**ppcVar3)(offset, puVar1, uVar2, 1);
        uVar5 = extraout_DX;
    }
    uVar4 = pass1_1008_57c4((param_1 & 0xffff0000 | &local_BX_5.field_0xd2));
    param_1 = 0x380a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    param_1 = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return CONCAT31(extraout_var, uVar4) & 0xffff | uVar5 << 0x10;
}

pub fn track_popup_menu_1008_09ba(param_1: u32, param_2: u16) {
    let mut HVar1: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xec) == 0) {
        HVar1 = LoadMenu16(s_OPPOPMENU_1050_0150, g_h_instance_1050_038c);
        (iVar2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
        local_6 = (iVar2 + 0xec);
        unaff_CS = SUB42(offset, 0);
        HVar1 = GetSubMenu16(0, local_6);
        (iVar2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    local_4 = param_2;
    local_6 = (iVar2 + 8);
    ClientToScreen16(CONCAT22(&local_6, unaff_CS), unaff_SS);
    TrackPopupMenu16(0x0, 0, g_h_window, 0, local_4, local_6, 0);
    return;
}

pub fn send_window_msg_1008_0a3c(param_1: u32, uparam_2: i32) -> u16 {
    let BVar1: bool;
    let mut iVar2: i32;
    let mut uvar3: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if ((param_2 & 0xfff0) == 0xf140) {
        return (iVar2 + 0xde);
    }
    if ((param_2 & 0xfff0) == 0xf060) {
        BVar1 = IsIconic16((iVar2 + 8));
        if (BVar1 == 0) {
            PostMessage16(0, 0x67, 0x111, (iVar2 + 8));
        }
        return 0;
    }
    return 1;
}

pub fn create_window_1008_0af8(in_win_struct: *mut win_struct_42) -> u16 {
    let puVar1: *mut u16;
    let ppcVar2: fn();
    let local_win_handle: HANDLE16;
    let struct_a: *mut astruct_199;
    let paVar3: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let mut local_DX_85: i32;
    let struct_a_00: *mut astruct_199;
    let mut local_DX_175: i32;
    let mut extraout_DX_00: u16;
    let local_win_struct_42: *mut astruct_119;
    let mut local_ES_13: u16;
    let mut local_CS_138: u16;
    let paVar4: *mut astruct_199;
    let local_16: u32;
    let mut local_6: u16;
    let mut temp_5fdbebe016: i32;
    char * *fn_ptr_1;
    let mut temp_5fc4833446: u32;

    paVar4 = create_win_1008_9760(in_win_struct);
    struct_a = (paVar4 >> 0x10);
    local_ES_13 = (in_win_struct >> 0x10);
    local_win_struct_42 = in_win_struct;
    local_win_handle = local_win_struct_42.win_handle_0x8;
    g_h_window = local_win_handle;
    process_struct_1000_179c(0x12, struct_a);
    paVar3 = (struct_a | local_win_handle);
    if (paVar3 != 0x0) {
        set_timer_1008_91ba(local_win_handle, struct_a);
        paVar3 = extraout_DX;
    }
    process_struct_1000_179c(6, paVar3);
    if ((paVar3 | local_win_handle) == 0) {
        &local_win_struct_42.field_0xe0 = 0;
    } else {
        pass1_1008_392e(
            CONCAT22(paVar3, local_win_handle),
            local_win_struct_42.win_handle_0x8,
        );
        local_win_struct_42.field_0xe0 = local_win_handle;
        local_win_struct_42.field_0xe2 = local_DX_85;
    }
    fn_ptr_1 = (in_win_struct + 0x14);
    (**fn_ptr_1)(0x1000, in_win_struct, 0, 0x15a, &g_alloc_addr_1050_1050);
    local_CS_138 = 0x1000;
    paVar3 = struct_a_00;
    process_struct_1000_179c(0xec, struct_a_00);
    _local_6 = CONCAT22(paVar3, local_win_handle);
    if ((paVar3 | local_win_handle) == 0) {
        &local_win_struct_42.field_0xe4 = 0;
    } else {
        puVar1 = &local_win_struct_42.field_0xcc;
        unsafe {
            *puVar1 = *puVar1 + 1;
        }
        local_CS_138 = 0x1020;
        mci_fn_1020_08b6(_local_6, local_win_struct_42.field_0xcc, in_win_struct);
        local_win_struct_42.field_0xe4 = local_win_handle;
        local_win_struct_42.field_0xe6 = local_DX_175;
    }
    if (local_win_struct_42.field_0xce != 0) {
        ppcVar2 = (local_win_struct_42.field_0xce + 0x10);
        ppcVar2();
    }
    local_win_struct_42.field_0xce = &local_win_struct_42.field_0xe4;
    temp_5fc4833446 = &local_win_struct_42.field_0xe4;
    ppcVar2 = (&local_win_struct_42.field_0xe4 + 0x10);
    ppcVar2();
    temp_5fdbebe016 = local_win_struct_42.field_0xe6;
    local_win_struct_42.field_0xe8 = &local_win_struct_42.field_0xe4;
    ppcVar2 = (local_win_struct_42.field_0xe8 + 8);
    ppcVar2(
        local_CS_138,
        &local_win_struct_42.field_0xe8,
        temp_5fdbebe016,
        temp_5fc4833446,
        1,
    );
    ppcVar2 = (local_win_struct_42.field_0xe8 + 0xc);
    ppcVar2();
    pass1_1008_6978(
        (in_win_struct & 0xffff | local_ES_13 << 0x10),
        0,
        local_win_struct_42.field_0xe8,
    );
    return extraout_DX_00;
}

pub fn call_fill_client_window_1008_1230(param_1: *mut void) {
    fill_client_window_1008_39ac((param_1 + 0xe0));
    return;
}

pub fn win_fn_1008_12dc(param_1: u32, param_2: u32) {
    let puVar1: *mut u8;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let HVar4: HCURSOR16;
    let mut extraout_DX: i32;
    let mut hwnd: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    pass1_1008_6d8a(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, local_c)),
        param_2,
    );
    puVar1 = local_c;
    write_to_file_1008_6e02(puVar1, unaff_SS);
    uVar5 = (param_1 >> 0x10);
    if (puVar1 == 0x0) {
        SetCursor16(local_6);
        local_28._0_2_ = _g_struct_73_1050_14cc;
        local_28._2_2_ = (_g_struct_73_1050_14cc >> 0x10);
        uVar2 = g_u16_1050_0310;
        load_string_1010_847e(local_28, local_28._2_2_, g_u16_1050_0310);
        hwnd = extraout_DX;
        pass1_fn_1008_60e8(uVar2, extraout_DX, 0);
        _local_10 = CONCAT22(hwnd, uVar2);
        local_28._0_2_ = _g_struct_73_1050_14cc;
        local_28._2_2_ = (_g_struct_73_1050_14cc >> 0x10);
        uVar3 = uVar2;
        load_string_1010_847e(local_28, local_28._2_2_, 0x57b);
        MessageBeep16(0x10);
        MessageBox16(
            0x10,
            CONCAT22(extraout_DX_00, uVar3),
            CONCAT22(hwnd, uVar2),
            (param_1 + 8),
        );
    } else {
        (_g_bool_1050_5748 + 8) = 0;
        HVar4 = SetCursor16(local_6);
        local_28._0_2_ = _g_struct_73_1050_14cc;
        local_28._2_2_ = (_g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e(local_28, local_28._2_2_, 0x6d3);
        pass1_fn_1008_60e8(HVar4, extraout_DX_01, extraout_DX_01);
        local_28._0_2_ = _g_struct_73_1050_14cc;
        local_28._2_2_ = (_g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e(local_28, local_28._2_2_, 0x57b);
        MessageBeep16(0);
        hwnd = (param_1 + 8);
        MessageBox16(0x40, CONCAT22(extraout_DX_02, HVar4), (hwnd << 0x10), hwnd);
        _local_10 = CONCAT22(hwnd, hwnd);
    }
    error_check_1000_17ce((_local_10 & 0xffff | hwnd << 0x10));
    close_file_1008_6dd0(CONCAT22(unaff_SS, local_c));
    return;
}

pub fn win_fn_1008_1414(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let BVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let ppVar6: *mut pass1_struct_2;
    let mut iVar7: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar8: u16;
    let mut extraout_DX_01: u16;
    let mut unaff_DI: u16;
    let mut unaff_SS: u16;
    let ppVar9: *mut pass1_struct_1;
    let mut uVar10: u16;
    let uVar11: u8;
    let uVar12: u8;
    let mut local_50: u32;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut uStack38: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: [u8; 6];

    pass1_1008_6d8a(CONCAT22(unaff_SS, local_8), param_2);
    BVar3 = close_file_1008_6e78(CONCAT22(unaff_SS, local_8));
    iVar7 = param_1;
    uVar10 = (param_1 >> 0x10);
    if (BVar3 == 0) {
        if (g_u16_1050_0310 == 0) {
            g_u16_1050_0310 = 0x6d4;
        }
        uVar4 = g_u16_1050_0310;
        load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            g_u16_1050_0310,
        );
        uVar8 = extraout_DX;
        pass1_fn_1008_60e8(uVar4, extraout_DX);
        uVar5 = uVar4;
        load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x57b,
        );
        MessageBeep16(0x10);
        MessageBox16(
            0x10,
            CONCAT22(extraout_DX_00, uVar5),
            CONCAT22(uVar8, uVar4),
            (iVar7 + 8),
        );
        error_check_1000_17ce(CONCAT22(uVar8, uVar4));
        call_fn_ptr_1000_24cd(1);
    }
    set_cursor_1008_2dcc(iVar7, uVar10, 8);
    _local_c = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_DI, 0x2f));
    uVar8 = (_local_c >> 0x10);
    local_10 = (_local_c + 0x20);
    ppVar6 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), local_10);
    _local_14 = CONCAT22(uVar8, ppVar6);
    local_18 = &ppVar6.field_0x10;
    uVar1 = (iVar7 + 0xe8);
    ppcVar2 = ((iVar7 + 0xe8) + 4);
    ppcVar2(
        0x1030,
        uVar1,
        (uVar1 >> 0x10),
        local_10,
        (local_10 >> 0x10),
        (local_18 + 2) + -1,
        2,
    );
    local_22 = extraout_DX_01;
    ppVar6 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), 0x4000001);
    _local_1c = CONCAT22(local_22, ppVar6);
    local_20 = &ppVar6.field_0x10;
    local_24 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), local_20);
    local_2a = (local_24 + 0xc);
    uStack38 = (local_24 + 0x10);
    iVar7 = pass1_1030_5b00(_local_14);
    uVar11 = SUB21(&local_2a, 0);
    uVar12 = (&local_2a >> 8);
    uVar10 = unaff_SS;
    ppVar9 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT13(uVar12, CONCAT12(uVar11, iVar7)),
    );
    pass1_1018_179e(ppVar9, CONCAT22(uVar10, CONCAT11(uVar12, uVar11)));
    uVar11 = 0;
    uVar12 = 4;
    uVar5 = 0x1b;
    uVar10 = 1;
    ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
    pass1_1010_043a(ppVar9, CONCAT13(uVar12, CONCAT12(uVar11, uVar10)), uVar5);
    close_file_1008_6dd0(CONCAT22(unaff_SS, local_8));
    return;
}

// pub fn win_fn_1008_1414(param_1: u32,param_2: u32)

// {
//     let mut u_var1: u32;
//     let ppcVar2: fn();
//     let BVar3: bool;
//     let mut uVar4: u16;
//     let mut uVar5: u16;
//     let ppVar6: *mut pass1_struct_2;
//     let mut iVar7: i32;
//     let mut extraout_DX: u16;
//     let mut extraout_DX_00: u16;
//     let mut uVar8: u16;
//     let mut extraout_DX_01: u16;
//     let mut unaff_DI: u16;
//     let mut unaff_SS: u16;
//     let ppVar9: *mut pass1_struct_1;
//     let mut uVar10: u16;
//     let uVar11: u8;
//     let uVar12: u8;
//     let mut local_50: u32;
//     let mut local_4c: u16;
//     let mut local_4a: u16;
//     let mut local_3c: u16;
//     let mut local_3a: u16;
//     let mut local_38: u16;
//     let mut local_36: u16;
//     let mut local_34: u16;
//     let mut local_32: u16;
//     let mut local_30: u16;
//     let mut local_2e: u16;
//     let mut local_2c: u16;
//     let mut local_2a: u32;
//     let mut uStack38: u16;
//     let mut local_24: u16;
//     let mut local_22: u16;
//     let mut local_20: u32;
//     let mut local_1c: u16;
//     let mut local_1a: u16;
//     let mut local_18: u32;
//     let mut local_14: u16;
//     let mut local_12: u16;
//     let mut local_10: u32;
//     let mut local_c: u16;
//     let mut local_a: u16;
//     let mut local_8: [u8;6];

//     pass1_1008_6d8a(CONCAT22(unaff_SS, local_8), param_2);
//     BVar3 = close_file_1008_6e78(CONCAT22(unaff_SS, local_8));
//     iVar7 = param_1;
//     uVar10 = (param_1 >> 0x10);
//     if (BVar3 == 0)
//     {
//         if (g_u16_1050_0310 == 0)
//         {
//             g_u16_1050_0310 = 0x6d4;
//         }
//         uVar4 = g_u16_1050_0310;
//         load_string_1010_847e(_g_struct_73_1050_14cc, (_g_struct_73_1050_14cc >> 0x10),
//                               g_u16_1050_0310);
//         uVar8 = extraout_DX;
//         pass1_fn_1008_60e8(uVar4, extraout_DX);
//         uVar5 = uVar4;
//         load_string_1010_847e(_g_struct_73_1050_14cc, (_g_struct_73_1050_14cc >> 0x10), 0x57b);
//         MessageBeep16(0x10);
//         MessageBox16(0x10, CONCAT22(extraout_DX_00, uVar5), CONCAT22(uVar8, uVar4),
//                      (iVar7 + 8));
//         error_check_1000_17ce(CONCAT22(uVar8, uVar4));
//         call_fn_ptr_1000_24cd(1);
//     }
//     set_cursor_1008_2dcc(iVar7, uVar10, 8);
//     _local_c = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_DI, 0x2f));
//     uVar8 = (_local_c >> 0x10);
//     local_10 = (_local_c + 0x20);
//     ppVar6 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10),
//                              local_10);
//     _local_14 = CONCAT22(uVar8, ppVar6);
//     local_18 = &ppVar6.field_0x10;
//     uVar1 = (iVar7 + 0xe8);
//     ppcVar2 = ((iVar7 + 0xe8) + 4);
//     ppcVar2(0x1030, uVar1, (uVar1 >> 0x10), local_10, (local_10 >> 0x10),
//                 (local_18 + 2) + -1, 2);
//     local_22 = extraout_DX_01;
//     ppVar6 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10),
//                              0x4000001);
//     _local_1c = CONCAT22(local_22, ppVar6);
//     local_20 = &ppVar6.field_0x10;
//     local_24 = pass1_1030_8344(_g_bool_1050_5748,
//                                     (_g_bool_1050_5748 >> 0x10), local_20);
//     local_2a = (local_24 + 0xc);
//     uStack38 = (local_24 + 0x10);
//     iVar7 = pass1_1030_5b00(_local_14);
//     uVar11 = SUB21(&local_2a, 0);
//     uVar12 = (&local_2a >> 8);
//     uVar10 = unaff_SS;
//     ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT13(uVar12, CONCAT12(uVar11, iVar7)));
//     pass1_1018_179e(ppVar9, CONCAT22(uVar10, CONCAT11(uVar12, uVar11)));
//     uVar11 = 0;
//     uVar12 = 4;
//     uVar5 = 0x1b;
//     uVar10 = 1;
//     ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
//     pass1_1010_043a(ppVar9, CONCAT13(uVar12, CONCAT12(uVar11, uVar10)), uVar5);
//     close_file_1008_6dd0(CONCAT22(unaff_SS, local_8));
//     return;
// }

pub fn win_fn_1008_2b54(param_1: u32) -> u16 {
    let ppcVar1: fn();
    let pcVar2: *mut char;
    let mut in_AX: i32;
    let mut uvar3: u16;
    let in_DX: *mut astruct_199;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_aa: u16;
    let mut local_a8: u16;
    let mut local_a6: u16;
    let mut local_a4: u16;
    let mut local_56: [u8; 80];
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    if (_PTR_LOOP_1050_4230 == 0) {
        pcVar2 = load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x5f2,
        );
        copy_string_1000_3d3e(CONCAT22(unaff_SS, local_56), pcVar2);
        pcVar2 = load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x57b,
        );
        copy_string_1000_3d3e(CONCAT22(unaff_SS, &local_a6), pcVar2);
        local_4 = MessageBox16(
            0x21,
            CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_a6)),
            CONCAT22(unaff_SS, local_56),
            g_h_window,
        );
    } else {
        uVar5 = 0x1000;
        process_struct_1000_179c(0xb4, in_DX);
        uVar4 = in_DX | in_AX;
        if (uVar4 == 0) {
            uVar3 = 0;
            uVar4 = 0;
        } else {
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            uVar3 = mixed_1040_8520(CONCAT22(in_DX, in_AX), g_h_window, 0x21, 2, 0x57b, 0x5f2);
        }
        _local_a6 = CONCAT22(uVar4, uVar3);
        ppcVar1 = (*_local_a6 + 0x74);
        local_4 = (**ppcVar1)(uVar5, uVar3, uVar4, in_AX);
    }
    local_6 = local_4;
    if (local_4 != 1) {
        local_6 = 0;
    }
    if (((local_6 != 0) && (_g_bool_1050_5748 != 0))
        && (
            uVar4 = (_g_bool_1050_5748 + 8),
            _local_a6 = (_local_a6 & 0xffff0000 | uVar4),
            uVar4 != 0,
        ))
    {
        PostMessage16(0, 0xb4, 0x111, (param_1 + 8));
        local_6 = 0;
    }
    return local_6;
}

pub fn win_fn_1008_2d22(param_1: u32) {
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (((iVar4 + 0xee) != 0)
        && (
            piVar1 = (iVar4 + 0xf2),
            unsafe { *piVar1 = *piVar1 + -1 },
            (iVar4 + 0xf2) < 1,
        ))
    {
        uVar8 = (iVar4 + 0xee);
        ppcVar3 = ((iVar4 + 0xee) + 0x90);
        (**ppcVar3)();
        (iVar4 + 0xee) = 0;
        (iVar4 + 0xf2) = 0;
        if ((iVar4 + 0xe8) != 0) {
            uVar7 = 3;
            uVar6 = (iVar4 + 0xe8);
            ppcVar3 = ((iVar4 + 0xe8) + 0xc);
            (**ppcVar3)();
            show_win_1038_b68a(_g_astruct_112_a, (_g_astruct_112_a >> 0x10));
            uVar2 = (iVar4 + 0xf4);
            show_window_1010_7ace(uVar2, (uVar2 >> 0x10));
            uVar2 = (iVar4 + 0xe8);
            ppcVar3 = ((iVar4 + 0xe8) + 0x98);
            (**ppcVar3)(0x1010, uVar2, (uVar2 >> 0x10), 1, uVar6, uVar7, uVar8);
            PostMessage16(0, 0xfc, 0x111, g_h_window);
        }
    }
    return;
}

pub fn win_fn_1008_3018(param_1: u32) {
    let string_b: *mut char;
    let mut uVar1: u16;
    let mut iVar2: i32;
    let local_AX__1: *mut astruct_179;
    let mut str_index: u16;
    let mut local_DX_17: i32;
    let mut unaff_SI: u16;
    let mut local_SS__1: i32;
    let mut local_114: u16;
    let mut local_112: u32;
    let mut local_10a: u32;
    let mut local_106: u32;
    let mut local_str: [u8; 256];

    local_str[0] = '\0';
    local_106 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    uVar1 = (local_106 >> 0x10);
    iVar2 = local_106;
    string_b = *(iVar2 + 0x12);
    local_10a._0_2_ = string_b;
    if (((iVar2 + 0x14) | local_10a) == 0) {
        open_save_1008_30cc(param_1);
    } else {
        copy_string_1000_3d3e(CONCAT22(local_SS__1, local_str), *(iVar2 + 0x1a));
        str_index = get_string_index_1000_3da4(CONCAT22(local_SS__1, local_str));
        if (local_str[str_index - 1] != '\\') {
            local_str[str_index] = '\\';
            local_str[str_index + 1] = '\0';
        }
        process_string_1000_3cea(CONCAT22(local_SS__1, local_str), string_b);
        if (local_str[0] != '\0') {
            win_fn_1008_12dc(param_1, local_str, local_SS__1);
            return;
        }
    }
    return;
}

pub fn post_quit_msg_1008_3af4() {
    PostQuitMessage16();
    return;
}

pub fn win_cleanup_fn_1040_a294(param_1: *mut astruct_44) {
    let mut local_CS__1: u16;

    param_1.ptr_a_lo = 0xa4e8;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + -1;
    if (PTR_LOOP_1050_5eda == 0x0) {
        FreeProcInstance16(CONCAT22(_PTR_LOOP_1050_5edc, local_CS__1));
        _PTR_LOOP_1050_5edc = 0;
    }
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn win_gui_fn_1040_a2cc(param_1: *mut astruct_124, param_2: u32, param_3: u32) {
    let mut uVar1: i32;
    let mut uVar2: u32;

    if (param_3._2_2_ == 0x1826) {
        if ((param_3 == 1) || (1 < param_3 - 1 && (param_3 - 3 < 2))) {
            uVar1 = 1;
        } else {
            uVar1 = 0;
        }
        return uVar1;
    }
    uVar2 = win_gui_fn_1040_b54a(param_1, param_2, (param_2 >> 0x10), param_3);
    return uVar2;
}

pub fn make_proc_inst_1040_a234(param_1: *mut u8, param_2: *mut u8, param_3: u16_00, param_3: u32) {
    let unaff_CS: HANDLE16;

    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, param_3_00),
        (param_3 >> 0x10),
    );
    CONCAT22(param_2, param_1) = 0xa4e8;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    if (_PTR_LOOP_1050_5edc == 0) {
        _PTR_LOOP_1050_5edc =
            MakeProcInstance16(unaff_CS, CONCAT22(0xa40e, g_h_instance_1050_038c));
    }
    (param_1 + 0xc) = _PTR_LOOP_1050_5edc;
    PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 1;
    PTR_LOOP_1050_5ee0 = param_1;
    PTR_LOOP_1050_5ee2 = param_2;
    return;
}

pub fn win_fn_1040_9ce0(param_1: i32, param_2: u16_00, param_2: HWND16, param_3: u32) {
    let pbVar1: *mut byte;
    let mut iVar2: i32;
    let mut id: u16;
    let mut iVar3: i32;
    let mut bVar4: u8;
    let mut in_AX: u16;
    let WVar5: WPARAM16;
    let BVar6: bool;
    let mut offset: i32;
    let struct_a: *mut astruct_199;
    let extraout_DX: HWND16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let in_struct_1: *mut astruct_44;
    let pWVar8: *mut WPARAM16;
    let LVar9: LRESULT;
    let mut uVar10: u32;
    let mut uVar11: u16;
    let HVar12: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let local_a: RECT16;

    HVar12 = &g_alloc_addr_1050_1050;
    in_struct_1 = GetWindowLong16(0, param_3._2_2_);
    struct_a = (in_struct_1 >> 0x10);
    iVar3 = in_struct_1;
    uVar7 = ((in_struct_1 & 0xffff0000) >> 0x10);
    if (param_3 == 0x30) {
        (iVar3 + 0x5a) = param_2;
    } else {
        if (param_3 < 0x31) {
            if (param_3 == 0x1f) {
                (iVar3 + 4) = 0;
                ReleaseCapture16(HVar12);
                return;
            }
            if (0x1f < param_3) {}
            // goto LAB_1040_a1ae;
            bVar4 = param_3;
            if (bVar4 == 8) {
                pbVar1 = (iVar3 + 4);
                unsafe {
                    *pbVar1 = *pbVar1 & 0xf7;
                }
                local_1e._0_1_ = false;
                BVar6 = IsWindow16(param_2);
                if (BVar6 != 0) {
                    uVar10 = SendMessage16(0, 0, 0x87, param_2);
                    local_1e._0_1_ = (uVar10 & 0x20) == 0;
                }
                (iVar3 + 0x56) = 0;
                SendMessage16(0, (iVar3 + 0x5c), 0x401, (iVar3 + 2));
                if (((iVar3 + 0x5c) != 0) && ((iVar3 + 0x5c) != in_struct_1.ptr_a_lo)) {
                    SendDlgItemMessage16(local_1e, 1, 0x404, (iVar3 + 0x5c), (iVar3 + 2));
                }
                (iVar3 + 0x5c) = 0;
            } else {
                if (bVar4 < 9) {
                    if (bVar4 == 1) {
                        pWVar8 = GetWindowLong16(0, param_3._2_2_);
                        iVar3 = pWVar8;
                        uVar7 = ((pWVar8 & 0xffff0000) >> 0x10);
                        (iVar3 + 2) = (param_1 + 8);
                        WVar5 = GetDlgCtrllID16(param_3._2_2_);
                        unsafe {
                            *pWVar8 = WVar5;
                        }
                        (iVar3 + 0x56) = (param_1 + 0x12);
                        copy_string_1000_3d3e(
                            (pWVar8 & 0xffff0000 | (iVar3 + 6)),
                            *(param_1 + 0x16),
                        );
                        if ((*(param_1 + 0x12) & 1) != 0) {
                            SendMessage16(0, unsafe { *pWVar8 }, 0x401, (param_1 + 8));
                        }
                        if (((param_1 + 0x14) & 0x800) == 0) {
                            return;
                        }
                        pbVar1 = (iVar3 + 4);
                        unsafe {
                            *pbVar1 = *pbVar1 | 4;
                        }
                        return;
                    }
                    if (bVar4 == 2) {
                        error_check_1000_17ce(in_struct_1);
                        SetWindowLong16(0, 0);
                        return;
                    }
                    if (bVar4 != 7) {}
                    // goto LAB_1040_a1ae;
                    pbVar1 = (iVar3 + 4);
                    unsafe {
                        *pbVar1 = *pbVar1 | 8;
                    }
                    LVar9 = SendMessage16(0, 0, 0x400, (iVar3 + 2));
                    id = LVar9;
                    if (((LVar9 >> 0x10) == 0x534b)
                        && ((iVar3 + 0x5c) = id, id != in_struct_1.ptr_a_lo))
                    {
                        SendDlgItemMessage16(1, 0, 0x404, id, (iVar3 + 2));
                    }
                    SendMessage16(0, in_struct_1.ptr_a_lo, 0x401, (iVar3 + 2));
                    (iVar3 + 0x56) = 1;
                } else {
                    if (bVar4 == 10) {
                        pbVar1 = (iVar3 + 4);
                        unsafe {
                            *pbVar1 = *pbVar1 & 0xfb;
                        }
                        if (param_2 == 0) {
                            pbVar1 = (iVar3 + 4);
                            unsafe {
                                *pbVar1 = *pbVar1 | 4;
                            }
                        }
                    } else {
                        if (bVar4 != 0xc) {
                            if (bVar4 == 0xf) {
                                draw_1040_9948(param_3._2_2_, iVar3, uVar7);
                                return;
                            }
                            // goto LAB_1040_a1ae;
                        }
                        if (CONCAT22(param_2_00, param_1) != 0) {
                            copy_string_1000_3d3e(
                                (in_struct_1 & 0xffff0000 | (iVar3 + 6)),
                                CONCAT22(param_2_00, param_1),
                            );
                        }
                    }
                }
            }
            // goto LAB_1040_9e20;
        }
        if (param_3 == 0x200) {
            if ((*(iVar3 + 4) & 1) == 0) {
                return;
            }
            GetClientRect16(CONCAT22(unaff_SS, &local_a), param_3._2_2_);
            iVar2 = (iVar3 + 4);
            BVar6 = PtInRect16(CONCAT22(param_2_00, param_1), &local_a);
            if (BVar6 == 0) {
                pbVar1 = (iVar3 + 4);
                unsafe {
                    *pbVar1 = *pbVar1 & 0xfd;
                }
            } else {
                pbVar1 = (iVar3 + 4);
                unsafe {
                    *pbVar1 = *pbVar1 | 2;
                }
            }
            param_1 = (iVar3 + 4) - iVar2;
        } else {
            if (param_3 < 0x201) {
                offset = param_3 - 0x81;
                if (offset == 0) {
                    process_struct_1000_179c(0x5e, struct_a);
                    if ((struct_a | offset) == 0) {
                        offset = 0;
                        HVar12 = 0;
                    } else {
                        process_struct_1040_9824(CONCAT22(struct_a, offset));
                        HVar12 = extraout_DX;
                    }
                    SetWindowLong16(offset, HVar12);
                    return;
                }
                if (param_3 == 0x87) {
                    return;
                }
                HVar12 = param_3 - 0x100;
                if (HVar12 == 0) {
                    if ((param_2 == 0x26) || (param_2 == 0x25)) {
                        uVar7 = (iVar3 + 2);
                        uVar11 = 1;
                    } else {
                        if ((param_2 != 0x28) && (param_2 != 0x27)) {
                            if (((param_2 == 0x20) || (param_2 == 0xd))
                                && (&PTR_LOOP_1050_5ed8 == 0))
                            {
                                &PTR_LOOP_1050_5ed8 = 1;
                                pbVar1 = (iVar3 + 4);
                                unsafe {
                                    *pbVar1 = *pbVar1 | 2;
                                }
                                // goto LAB_1040_9e20;
                            }
                            // goto LAB_1040_a1ae;
                        }
                        uVar7 = (iVar3 + 2);
                        uVar11 = 0;
                    }
                    GetNextDlgTabItem16(::offset, uVar11, param_3._2_2_, uVar7);
                    SetFocus16(HVar12);
                    return;
                }
                if ((param_3 == 0x101) && (&PTR_LOOP_1050_5ed8 != 0)) {
                    &PTR_LOOP_1050_5ed8 = 0;
                    pbVar1 = (iVar3 + 4);
                    unsafe {
                        *pbVar1 = *pbVar1 & 0xfd;
                    }
                    InvalidateRect16(1, 0x0, 0);
                    UpdateWindow16(param_3._2_2_);
                    SendMessage16(0, in_struct_1.ptr_a_lo, 0x111, (iVar3 + 2));
                    return;
                }
                // LAB_1040_a1ae:
                DefWindowProc16(
                    CONCAT22(param_2_00, param_1),
                    param_2,
                    param_3,
                    param_3._2_2_,
                );
                return;
            }
            if (param_3 == 0x201) {
                // LAB_1040_9e74:
                SetFocus16(param_3._2_2_);
                pbVar1 = (iVar3 + 4);
                unsafe {
                    *pbVar1 = *pbVar1 | 3;
                }
                InvalidateRect16(1, 0x0, 0);
                UpdateWindow16(param_3._2_2_);
                SetCapture16(param_3._2_2_);
                return;
            }
            if (param_3 == 0x202) {
                ReleaseCapture16(HVar12);
                GetClientRect16(CONCAT22(unaff_SS, &local_a), param_3._2_2_);
                if ((*(iVar3 + 4) & 1) == 0) {
                    return;
                }
                pbVar1 = (iVar3 + 4);
                unsafe {
                    *pbVar1 = *pbVar1 & 0xfc;
                }
                InvalidateRect16(1, 0x0, 0);
                UpdateWindow16(param_3._2_2_);
                local_1e._0_1_ = param_2_00;
                local_1e._1_1_ = (param_2_00 >> 8);
                BVar6 = PtInRect16(
                    CONCAT13(local_1e._1_1_, CONCAT12(local_1e, param_1)),
                    &local_a,
                );
                if (BVar6 == 0) {
                    return;
                }
                PostMessage16(0, in_struct_1.ptr_a_lo, 0x111, (iVar3 + 2));
                return;
            }
            if (param_3 == 0x203) {}
            // goto LAB_1040_9e74;
            if (param_3 != 0x404) {}
            // goto LAB_1040_a1ae;
            if (param_2 == 1) {
                (iVar3 + 0x56) = 1;
            } else {
                (iVar3 + 0x56) = 0;
            }
        }
    }
    if (param_1 == 0) {
        return;
    }
    // LAB_1040_9e20:
    InvalidateRect16(1, 0x0, 0);
    UpdateWindow16(param_3._2_2_);
    return;
}

pub fn load_cursor_1040_9854(param_1: *mut u16) -> *mut u16 {
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let mut iVar3: i32;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    (iVar3 + 2) = &PTR_LOOP_1050_1008;
    unsafe {
        *param_1 = 0xa230;
    }
    (iVar3 + 2) = &PTR_LOOP_1050_1040;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (iVar3 + 4)), s_OPButton_1050_5ece);
    (iVar3 + 0x54) = 3;
    HVar1 = LoadCursor16(0x7f00, 0);
    (iVar3 + 0x58) = HVar1;
    HVar2 = GetStockObject16(4);
    (iVar3 + 0x56) = HVar2;
    reg_class_1040_98c0(iVar3, uVar4);
    return param_1;
}

pub fn call_win_proc_1040_9686(param_1: u16, param_2: u16, param_3_00: WPARAM16, param_3: u32) {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let HVar2: HANDLE16;
    let HVar3: HANDLE16;
    let BVar4: bool;
    let mut in_DX: u16;
    let mut unaff_SS: u16;
    let uVar5: u8;
    let local_1a: RECT16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (in_AX >> 8);
    HVar2 = GetProp16(CONCAT13(uVar5, CONCAT12(in_AX, 0x5e7d)), param_3._2_2_);
    HVar3 = GetProp16(CONCAT13(uVar5, CONCAT12(in_AX, 0x5e76)), param_3._2_2_);
    _local_6 = CONCAT22(HVar2, HVar3);
    HVar2 = GetProp16(CONCAT22(in_AX, 0x5e8b), param_3._2_2_);
    HVar3 = GetProp16(CONCAT22(in_AX, 0x5e84), param_3._2_2_);
    _local_a = CONCAT22(HVar2, HVar3);
    if ((HVar2 | HVar3) != 0) {
        if (param_3 == 2) {
            local_12 = _local_a;
            local_e = _local_a;
            if (_local_a != 0x0) {
                ppcVar1 = *_local_a;
                (**ppcVar1)(offset, HVar3, HVar2, 1);
            }
        } else {
            if (param_3 == 0x201) {
                HVar2 = GetProp16(CONCAT22(in_AX, 0x5e92), param_3._2_2_);
                if (HVar2 == 0) {
                    GetClientRect16(CONCAT22(unaff_SS, &local_1a), (_local_a + 0x18));
                    BVar4 = PtInRect16(CONCAT22(param_2, param_1), &local_1a);
                    if (BVar4 == 0) {
                        return;
                    }
                    fn_1008_6048(CONCAT22(in_AX, 0x5e98), in_DX, SUB21(BVar4, 0));
                    ppcVar1 = (*_local_a + 0x1c);
                    (**ppcVar1)(
                        &PTR_LOOP_1050_1008,
                        _local_a,
                        (_local_a >> 0x10),
                        param_2,
                        param_1,
                        param_3_00,
                    );
                    return;
                }
            } else {
                if (param_3 == 0x204) {
                    GetClientRect16(CONCAT22(unaff_SS, &local_1a), (HVar3 + 0x18));
                    BVar4 = PtInRect16(CONCAT22(param_2, param_1), &local_1a);
                    if (BVar4 == 0) {
                        return;
                    }
                    fn_1008_6048(CONCAT22(in_AX, 0x5eab), in_DX, SUB21(BVar4, 0));
                    ppcVar1 = (*_local_a + 0x20);
                    (**ppcVar1)(
                        8,
                        _local_a,
                        (_local_a >> 0x10),
                        param_2,
                        param_1,
                        param_3_00,
                    );
                    return;
                }
            }
        }
    }
    if (_local_6 != 0) {
        CallWindowProc16(
            CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
            param_3_00,
            param_3,
            param_3._2_2_,
            _local_6,
        );
    }
    return;
}

pub fn update_window_1040_93aa(param_1: u32, param_2: u16, param_3: u16) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x1e) = param_3;
    (iVar1 + 0x20) = param_2;
    MoveWindow16(
        1,
        (iVar1 + 0x24),
        (iVar1 + 0x22),
        param_2,
        (iVar1 + 0x1e),
        (iVar1 + 0x18),
    );
    UpdateWindow16((iVar1 + 0x18));
    return;
}

pub fn create_win_1040_92dc(param_1: *mut astruct_41) {
    let mut window: u16;
    let handle: HANDLE16;
    let local_BX_4: *mut astruct_41;
    let handle_00: HANDLE16;
    let lVar1: u32;

    handle_00 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x18 == 0) {
        window = CreateWindow16(
            0,
            g_h_instance_1050_038c,
            local_BX_4.field_0x1c,
            local_BX_4.field_0x1a,
            0,
            0,
            local_BX_4.field_0x20,
            local_BX_4.field_0x1e,
            0x4000000b,
            s__1050_5e3e,
            s_button_1050_5e3f,
        );
        local_BX_4.field_0x18 = window;
        lVar1 = SetWindowLong16(_g_proc_inst_1050_5e18, (_g_proc_inst_1050_5e18 >> 0x10));
        handle = (lVar1 >> 0x10);
        local_BX_4.field_0x14 = lVar1;
        local_BX_4.field_0x16 = handle;
        SetProp16(handle, s_procHi_1050_5e46, local_BX_4.field_0x18);
        SetProp16(
            local_BX_4.field_0x14,
            s_procLo_1050_5e4d,
            local_BX_4.field_0x18,
        );
        SetProp16(handle_00, s_thisHi_1050_5e54, local_BX_4.field_0x18);
        SetProp16(local_BX_4, s_thisLo_1050_5e5b, local_BX_4.field_0x18);
        if (local_BX_4.field_0x40 != 0) {
            SetProp16(1, s_IsDlg_1050_5e62, local_BX_4.field_0x18);
        }
        ShowWindow16(5, local_BX_4.field_0x18);
    }
    return;
}

pub fn enable_window_1040_9234(param_1: u32, param_2: bool) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x18) != 0) {
        EnableWindow16(param_2, (param_1 + 0x18));
    }
    return;
}

pub fn free_proc_inst_1040_911e(in_struct_1: *mut astruct_356) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let ppcVar4: fn();
    let local_struct_1: *mut astruct_356;
    let mut uVar5: u16;

    uVar5 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1 = 0x9800;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1040;
    if (local_struct_1.field_0x38 != 0) {
        puVar1 = local_struct_1.field_0x8;
        uVar2 = local_struct_1.field_0xa;
        if ((uVar2 | puVar1) != 0) {
            unsafe {
                ppcVar4 = *puVar1;
            }
            (**ppcVar4)();
        }
        puVar1 = local_struct_1.field_0xc;
        uVar2 = local_struct_1.field_0xe;
        if ((uVar2 | puVar1) != 0) {
            unsafe {
                ppcVar4 = *puVar1;
            }
            (**ppcVar4)();
        }
        puVar1 = local_struct_1.field_0x10;
        uVar2 = local_struct_1.field_0x12;
        if ((uVar2 | puVar1) != 0) {
            unsafe {
                ppcVar4 = *puVar1;
            }
            (**ppcVar4)();
        }
    }
    error_check_1000_17ce(local_struct_1.field_0x4);
    uVar3 = local_struct_1.field_0x14;
    SetWindowLong16(uVar3, (uVar3 >> 0x10));
    RemoveProp16(s_thisLo_1050_5e1c, local_struct_1.field_0x18);
    RemoveProp16(s_thisHi_1050_5e23, local_struct_1.field_0x18);
    RemoveProp16(s_procLo_1050_5e2a, local_struct_1.field_0x18);
    RemoveProp16(s_procHi_1050_5e31, local_struct_1.field_0x18);
    RemoveProp16(s_IsDlg_1050_5e38, local_struct_1.field_0x18);
    PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -1;
    if (PTR_LOOP_1050_5e16 == 0x0) {
        FreeProcInstance16(CONCAT22(_g_proc_inst_1050_5e18, 0x1538));
        _g_proc_inst_1050_5e18 = 0;
    }
    in_struct_1 = s_1_1050_389a;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn enable_window_1040_8ea0(param_1: *mut astruct_59, param_2: u32, param_3: u32) {
    let enable: bool;
    let h_wnd: HWND16;

    if (param_3._2_2_ == 0xf8) {
        h_wnd = GetDlgItem16(0x17d8, param_1.field_0x6);
        enable = 1;
    } else {
        if (param_3._2_2_ != 0x17d8) {
            win_gui_fn_1040_b54a(param_1, param_2, (param_2 >> 0x10), param_3);
            return;
        }
        SetWindowPos16(6, 0xf6, 0x269, 0, 0, 0, param_1.field_0x6);
        enable = offset;
        GetDlgItem16(0x17d8, param_1.field_0x6);
        h_wnd = 0;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn draw_1040_8a06(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut h_dc: u16;
    let dev_ctx_handle: *mut u16;
    let obj_handle: HANDLE16;
    let h_gdi_obj: HPALETTE16;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let count: u16;
    let unaff_SS: HWND16;
    let mut uVar5: u32;
    let CVar6: COLORREF;
    let color: COLORREF;
    let uStack68: u8;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let local_22: *mut PAINTSTRUCT16;
    let iStack30: u16;

    count = (param_1 >> 0x10);
    iVar4 = param_1;
    local_24 = BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar4 + 6));
    uVar5 = (_PTR_LOOP_1050_4230 + 0xe);
    local_28._0_2_ = uVar5;
    local_28._2_2_ = (uVar5 >> 0x10);
    dev_ctx_handle = &local_24;
    realize_palette_1008_4e08(local_28, local_28._2_2_, dev_ctx_handle, unaff_SS);
    uVar5 = pass1_1008_4d72(uVar5);
    uVar3 = (uVar5 >> 0x10);
    iVar2 = uVar5;
    DrawIcon16(
        (iVar4 + 0x8e),
        CONCAT214(
            local_3c,
            CONCAT212(local_3e, CONCAT66(uStack68, CONCAT24(local_24, 0x14000a))),
        ),
        CONCAT214(
            uVar3,
            CONCAT212(
                iVar2,
                CONCAT210(
                    CONCAT11(2, *(iVar2 + 0x94)),
                    CONCAT28(
                        CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)),
                        CONCAT26(local_34, CONCAT24(local_36, CONCAT22(local_38, local_3a))),
                    ),
                ),
            ),
        ),
        dev_ctx_handle,
    );
    CVar6 = SetBkColor16(0, local_24);
    color = SetTextColor16(0x15388a8c, local_24);
    local_3e = 0;
    obj_handle = GetProp16(s_hfont_1050_5dfa, (iVar4 + 6));
    if (obj_handle != 0) {
        local_3e = SelectObject16(obj_handle, local_24);
    }
    uVar1 = (iVar4 + 0x90);
    local_28._0_2_ = uVar1;
    local_28._2_2_ = (uVar1 >> 0x10);
    DrawText16(
        0x10,
        (iVar4 + 0x9e),
        count,
        CONCAT22(local_28, 0xffff),
        local_28._2_2_,
    );
    if (obj_handle != 0) {
        SelectObject16(local_3e, local_24);
    }
    SetBkColor16(CONCAT22(0x8ae7, CVar6), local_24);
    SetTextColor16(color, local_24);
    h_dc = local_24;
    local_22 = local_24;
    local_24 = offset;
    h_gdi_obj = SelectPalette16(0, offset, h_dc);
    local_22 = offset;
    local_24 = 0x8b23;
    DeleteObject16(h_gdi_obj);
    iStack30 = (iVar4 + 6);
    local_22 = &local_22;
    local_24 = offset;
    EndPaint(local_22, unaff_SS);
    return;
}

pub fn win_fn_1040_8b3c(param_1: u16, param_2: u32, param_3: u32) {
    if ((param_3._2_2_ != 0x0)
        && (param_3._2_2_ == (&PTR_LOOP_1050_0000 + 1)
            || param_3._2_2_ == &dos_alloc_addr_1050_0002
            || ((&dos_alloc_addr_1050_0002 + 1) < param_3._2_2_ + -2
                && (param_3._2_2_ + -6 < &dos_alloc_addr_1050_0002))))
    {
        PTR_LOOP_1050_5df4 = 0x0;
        PTR_LOOP_1050_5df8 = param_3._2_2_;
        return;
    }
    post_win_msg_1040_7b3c(param_1, param_2, param_3);
    return;
}

pub fn destroy_window_1040_8b7e(param_1: u32) {
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn win_fn_1040_8b92(param_1: u32) {
    let mut bVar1: u8;
    let mut uVar2: u16;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    bVar1 = *(param_1 + 0x98) & 0xf0;
    if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x10))
        || ((bVar1 == 0x40 || (bVar1 == 0x40)) || (bVar1 == 0x20)))
    {
        uVar2 = LoadIcon16();
        (param_1 + 0x8e) = uVar2;
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_8

pub fn create_win_1040_8bea(
    param_1: *mut astruct_40,
    param_2: u16,
    menu: u16,
    x_coord: u16,
    y_coord: u16,
    window_name: u32,
) -> HANDLE16 {
    let mut window_1: u16;
    let w_param: HANDLE16;
    let local_BX_26: *mut astruct_40;
    let mut uVar1: u16;
    let LVar2: LRESULT;
    let mut local_8: u16;
    let mut style: u32;

    style = 0x50010000;
    if (param_2 != 0) {
        style = 0x50010001;
    }
    uVar1 = (param_1 >> 0x10);
    local_BX_26 = param_1;
    if (local_BX_26.field_0x74 != 0) {
        style = style | 0x8000000;
    }
    window_1 = CreateWindow16(
        0,
        g_h_instance_1050_038c,
        menu,
        local_BX_26.parent,
        0x17,
        0x58,
        y_coord,
        x_coord,
        style,
        window_name,
        s_OPButton_1050_5e00,
    );
    w_param = GetProp16(s_hfont_1050_5e09, local_BX_26.parent);
    if (w_param != 0) {
        LVar2 = SendMessage16(1, w_param, 0x30, window_1);
        w_param = LVar2;
    }
    return w_param;
}

pub fn win_fn_1040_89a4(param_1: *mut u32, param_2: *mut u16) {
    let mut uVar1: u16;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000fff0: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &g_alloc_addr_1050_1050);
    uVar3 = (param_2 + 2);
    unsafe {
        uVar1 = *param_2;
    }
    uVar4 = 0x1010;
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff0, 2));
    if ((ppVar5 + 0x72) != 0) {
        uVar4 = SUB42(&PTR_LOOP_1050_1008, 0);
        uVar3 = mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, CONCAT22(uVar3, uVar1));
        (param_1 + 0x8c) = uVar3;
    }
    unsafe {
        ppcVar2 = (*param_1 + 0x74);
    }
    ppcVar2(uVar4, param_1);
    return;
}

pub fn win_fn_1040_8718(param_1: *mut astruct_20) -> *mut u8 {
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut uVar6: u16;
    let mut menu: u16;
    let uVar7: u8;
    let uVar8: u8;
    let uVar9: u8;
    let uVar10: u8;
    let mut uVar11: i32;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_4: u16;

    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &g_alloc_addr_1050_1050);
    iVar3 = param_1;
    uVar11 = (param_1 >> 0x10);
    win_gui_func_1040_78e2(param_1);
    PTR_LOOP_1050_5df6 = (iVar3 + 6);
    if ((iVar3 + 0x94) != 0) {
        copy_string_1000_3d3e(
            (param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), iVar3 + 0x10)),
            *(iVar3 + 0x94),
        );
    }
    get_sys_metrics_1040_8c66(iVar3, uVar11);
    local_4 = *(iVar3 + 0x98) & 0xf;
    if (local_4 == 1) {
        (iVar3 + 0xae) = ((iVar3 + 0xaa) + -0xc4) / 2;
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_SS, &local_104),
            0x592,
        );
        uVar2 = (iVar3 + 0xae);
        create_win_1040_8bea(
            (param_1 & 0xffff | uVar11 << 0x10),
            1,
            1,
            uVar2,
            (uVar2 >> 0x10),
            CONCAT22(unaff_SS, &local_104),
        );
        piVar1 = (iVar3 + 0xae);
        unsafe {
            *piVar1 = *piVar1 + 0x6c;
        }
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_SS, &local_104),
            0x7d8,
        );
        uVar2 = (iVar3 + 0xae);
        uVar7 = uVar2;
        uVar8 = (uVar2 >> 8);
        uVar9 = (uVar2 >> 0x10);
        uVar10 = (uVar2 >> 0x18);
        menu = 2;
    } else {
        if (local_4 != 4) {
            (iVar3 + 0xae) = ((iVar3 + 0xaa) + -0x58) / 2;
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0xff,
                CONCAT22(unaff_SS, &local_104),
                0x592,
            );
            uVar2 = (iVar3 + 0xae);
            uVar7 = uVar2;
            uVar8 = (uVar2 >> 8);
            uVar9 = (uVar2 >> 0x10);
            uVar10 = (uVar2 >> 0x18);
            uVar6 = 1;
            menu = 1;
            // goto LAB_1040_88a5;
        }
        (iVar3 + 0xae) = ((iVar3 + 0xaa) + -0xc4) / 2;
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_SS, &local_104),
            0x650,
        );
        uVar2 = (iVar3 + 0xae);
        create_win_1040_8bea(
            (param_1 & 0xffff | uVar11 << 0x10),
            1,
            6,
            uVar2,
            (uVar2 >> 0x10),
            CONCAT22(unaff_SS, &local_104),
        );
        piVar1 = (iVar3 + 0xae);
        unsafe {
            *piVar1 = *piVar1 + 0x6c;
        }
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_SS, &local_104),
            0x651,
        );
        uVar2 = (iVar3 + 0xae);
        uVar7 = uVar2;
        uVar8 = (uVar2 >> 8);
        uVar9 = (uVar2 >> 0x10);
        uVar10 = (uVar2 >> 0x18);
        menu = 7;
    }
    uVar6 = 0;
    // LAB_1040_88a5:
    create_win_1040_8bea(
        (param_1 & 0xffff | uVar11 << 0x10),
        uVar6,
        menu,
        CONCAT11(uVar8, uVar7),
        CONCAT11(uVar10, uVar9),
        CONCAT22(unaff_SS, &local_104),
    );
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_120, 0x48));
    uVar4 = (ppVar5 >> 0x10);
    local_104 = (ppVar5 + 10);
    local_4 = (ppVar5 + 0xc);
    SetWindowPos16(
        0x40,
        (iVar3 + 0xac),
        (iVar3 + 0xaa),
        (local_4 - (iVar3 + 0xac)) / 2,
        (local_104 - (iVar3 + 0xaa)) / 2,
        0,
        (iVar3 + 6),
    );
    PTR_LOOP_1050_5df4 = (&PTR_LOOP_1050_0000 + 1);
    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &g_alloc_addr_1050_1050);
    destroy_window_1040_8b7e(iVar3, uVar11);
    PTR_LOOP_1050_5df6 = 0x0;
    if ((iVar3 + 0xb2) != 0) {
        ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_11e, 0x37));
        iVar3 = pass1_1008_ab54(ppVar5);
        if (iVar3 != 0) {
            PostMessage16(0, 0xfc, 0x111, g_h_window);
        }
    }
    return PTR_LOOP_1050_5df8;
}

pub fn enable_window_1040_86dc(param_1: u32) {
    let HVar1: HWND16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    HVar1 = GetDlgItem16(1, (param_1 + 6));
    if (HVar1 != 0) {
        EnableWindow16(1, HVar1);
        HVar1 = GetDlgItem16(2, (param_1 + 6));
        if (HVar1 != 0) {
            EnableWindow16(1, HVar1);
        }
    }
    return;
}

pub fn mixed_1040_8520(
    param_1: *mut u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> *mut astruct_362 {
    let mut out_buffer: string;
    let paVar1: *mut astruct_362;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let mut uVar4: i32;
    let in_DX: *mut astruct_199;
    let mut unaff_CS: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let in_string_1: *mut char;
    let pcVar7: *mut char;
    let mut uStack54: u16;
    let local_32: *mut astruct_362;
    let mut uStack48: u16;
    let uStack46: u8;
    let uStack45: u8;
    let uStack44: u8;
    let uStack43: u8;
    let paStack42: *mut astruct_73;
    let mut uStack40: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uStack40._2_2_ = param_2;
    paStack42 = 0x0;
    uStack40._0_2_ = 0xfc3;
    uStack46 = 1;
    uStack45 = 0;
    uStack44 = 0;
    uStack43 = 0;
    local_32 = param_1;
    paVar1 = local_32;
    uStack48 = (param_1 >> 0x10);
    uVar2 = uStack48;
    uStack54 = 0x853b;
    _local_32 = param_1;
    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), 0, 0xfc3, param_2);
    paVar1.field_0x8e = 0;
    paVar1.field_0x98 = param_3;
    paVar1.field_0x9a = 0;
    paVar1.field_0xb2 = 0;
    unsafe {
        *param_1 = 0x8ddc;
    }
    paVar1.field_0x2 = &PTR_LOOP_1050_1040;
    _local_a = 0;
    _local_6 = 300;
    paVar1.field_0x9e = 0;
    paVar1.field_0xa2 = 300;
    _local_e = CONCAT22(unaff_SS, &param_5);
    local_10 = param_4;
    if (param_4 != 0) {
        _local_e = CONCAT22(unaff_SS, &param_6);
        local_12 = param_5;
        uStack40._2_2_ = param_5;
        paStack42 = _g_struct_73_1050_14cc;
        uStack40._0_2_ = (_g_struct_73_1050_14cc >> 0x10);
        uStack44 = unaff_CS;
        uStack43 = (unaff_CS >> 8);
        unaff_CS = 0x1010;
        uStack46 = 0xa7;
        uStack45 = 0x85;
        uVar6 = load_str_1010_84ac(paStack42, uStack40, param_5);
        in_DX = (uVar6 >> 0x10);
        paVar1.field_0x94 = uVar6;
        paVar1.field_0x96 = in_DX;
        local_10 = local_10 - 1;
    }
    local_16 = 0;
    while (puVar3 = _local_e, local_10 != 0) {
        _local_e = (_local_e & 0xffff0000 | (local_e + 2));
        unsafe {
            uStack40._2_2_ = *puVar3;
        }
        paStack42 = _g_struct_73_1050_14cc;
        uStack40._0_2_ = (_g_struct_73_1050_14cc >> 0x10);
        uStack44 = unaff_CS;
        uStack43 = (unaff_CS >> 8);
        uStack46 = 0xd6;
        uStack45 = 0x85;
        local_14 = uStack40._2_2_;
        local_10 = local_10 - 1;
        in_string_1 = load_string_1010_847e(paStack42, uStack40, uStack40._2_2_);
        local_1a = (in_string_1 >> 0x10);
        paStack42 = 0x1010;
        unaff_CS = 0x1000;
        uStack44 = 0xe3;
        uStack43 = 0x85;
        pcVar7 = in_string_1;
        uVar4 = get_string_index_1000_3da4(in_string_1);
        uStack40._2_2_ = (pcVar7 >> 0x10);
        local_1c = pcVar7;
        in_DX = (in_string_1 >> 0x10);
        uStack40._0_2_ = in_string_1;
        local_16 = local_16 + uVar4;
    }
    uStack40._2_2_ = (local_16 + 1);
    uVar5 = 0x1000;
    paStack42 = 0x85fd;
    uStack40._0_2_ = unaff_CS;
    local_10 = local_10 - 1;
    process_struct_1000_179c(uStack40._2_2_, in_DX);
    paVar1.field_0x90 = uStack40._2_2_;
    &paVar1.field_0x92 = in_DX;
    _local_e = CONCAT22(unaff_SS, &param_6);
    local_10 = param_4 - 1;
    if (local_10 != 0) {
        _local_e = CONCAT22(unaff_SS, &stack0x0012);
        local_14 = param_6;
        uStack40._2_2_ = param_6;
        out_buffer = &paVar1.field_0x90;
        paStack42 = out_buffer;
        uStack40._0_2_ = (out_buffer >> 0x10);
        uStack44 = 0xff;
        uStack43 = 3;
        uStack48 = _g_struct_73_1050_14cc;
        uStack46 = (_g_struct_73_1050_14cc >> 0x10);
        uStack45 = (_g_struct_73_1050_14cc >> 0x18);
        local_32 = &PTR_LOOP_1050_1000;
        uVar5 = 0x1010;
        load_string_1010_84e0(
            uStack48,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            out_buffer,
            param_6,
        );
        local_10 = local_10 - 1;
    }
    while (puVar3 = _local_e, local_10 != 0) {
        _local_e = (_local_e & 0xffff0000 | (local_e + 2));
        unsafe {
            uStack40._2_2_ = *puVar3;
        }
        paStack42 = _g_struct_73_1050_14cc;
        uStack40._0_2_ = (_g_struct_73_1050_14cc >> 0x10);
        uStack44 = uVar5;
        uStack43 = (uVar5 >> 8);
        uStack46 = 0x5e;
        uStack45 = 0x86;
        local_14 = uStack40._2_2_;
        local_10 = local_10 - 1;
        uStack40 = load_string_1010_847e(paStack42, uStack40, uStack40._2_2_);
        pcVar7 = *&paVar1.field_0x90;
        uStack44 = SUB41(pcVar7, 0);
        uStack43 = (pcVar7 >> 8);
        paStack42 = (pcVar7 >> 0x10);
        uStack46 = 0x10;
        uStack45 = 0x10;
        uVar5 = 0x1000;
        uStack48 = 0x8674;
        _local_20 = uStack40;
        process_string_1000_3cea(pcVar7, uStack40);
    }
    uStack44 = 0x8a;
    uStack43 = 0x86;
    paStack42 = uVar5;
    local_10 = local_10 - 1;
    uStack40 = param_1;
    win_fn_1040_8b92(param_1);
    PTR_LOOP_1050_5df8 = 0x0;
    return paVar1;
}

pub fn move_window_1040_826c(param_1: u32, param_2: u16, param_3: u16) {
    let iVar1: u16;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    GetWindowRect16(CONCAT22(&local_e, unaff_CS), unaff_SS);
    if ((param_3 == 0xffff) || (param_2 == 0xffff)) {
        iVar1 = GetSystemMetrics16(0);
        local_4 = (iVar1 - (local_a - local_e)) / 2;
        iVar1 = GetSystemMetrics16(1);
        param_2 = (iVar1 - (local_8 - local_c)) / 2;
    } else {
        local_4 = param_3;
    }
    local_6 = param_2;
    MoveWindow16(
        0,
        local_8 - local_c,
        local_a - local_e,
        param_2,
        local_4,
        (param_1 + 6),
    );
    return;
}

pub fn set_sys_modal_window_1040_81fe(param_1: u32) {
    SetSysModalWindow16((param_1 + 6));
    return;
}

pub fn get_message_1040_81b6(param_1: u32) {
    let BVar1: bool;
    let mut uVar2: u16;
    let unaff_SS: HWND16;

    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x78) = 1;
    while (true) {
        BVar1 = IsWindow16((param_1 + 6));
        if (BVar1 == 0) {
            return;
        }
        BVar1 = GetMessage16(0, 0, 0, &stack0xffec);
        if (BVar1 == 0) {
            break;
        }
        IsDialogMessage16(&stack0xffec, unaff_SS);
    }
    return;
}

pub fn win_fn_1040_800c(param_1: u32) {
    let mut in_AX: u16;
    let mut extraout_DX: u16;
    let local_BX_17: *mut astruct_6;
    let mut uVar1: u16;
    let mut iStack18: i32;
    let mut local_f: u8;
    let mut iStack16: i32;
    let mut w_command: u16;
    let lp_help_file: *mut char;
    let mut h_wnd: u16;

    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1f8);
    uVar1 = (param_1 >> 0x10);
    local_BX_17 = param_1;
    if (local_BX_17.field_0x8a == 0) {
        h_wnd = local_BX_17.field_0x6;
        iStack16 = 0;
        w_command = 3;
        iStack18 = 0;
    } else {
        h_wnd = local_BX_17.field_0x6;
        w_command = 1;
        iStack18 = local_BX_17.field_0x8a;
        iStack16 = iStack18 >> 0xf;
    }
    _lp_help_file = CONCAT22(extraout_DX, in_AX);
    WinHelp16(
        CONCAT22(iStack16, iStack18),
        w_command,
        _lp_help_file,
        h_wnd,
    );
    return;
}

pub fn track_popup_menu_1040_7f86(param_1: u32, param_2: u16) {
    let mut menu_handle: u16;
    let mut HVar1: u16;
    let local_BX_4: *mut astruct_27;
    let mut uVar2: u16;
    let mut unaff_CS: u16;
    let mut window_handle: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if ((local_BX_4.menu_name != 0x0) && (local_BX_4.menu_handle_2 == 0)) {
        menu_handle = LoadMenu16(local_BX_4.menu_name, g_h_instance_1050_038c);
        local_BX_4.menu_handle_2 = menu_handle;
        if (menu_handle == 0) {
            return;
        }
        local_6 = local_BX_4.menu_handle_2;
        unaff_CS = SUB42(offset, 0);
        HVar1 = GetSubMenu16(0, local_6);
        local_BX_4.menu_handle_2 = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    local_4 = param_2;
    local_6 = local_BX_4.field_0x6;
    ClientToScreen16(CONCAT22(&local_6, unaff_CS), window_handle);
    TrackPopupMenu16(0x0, 0, local_BX_4.field_0x6, 0, local_4, local_6, 0);
    return;
}

pub fn destroy_win_1040_7b98(param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x74) == 0) {
        DestroyWindow16((param_1 + 6));
    }
    return;
}

pub fn win_gui_func_1040_78e2(param_1: *mut astruct_20) {
    let ppcVar1: fn();
    let dlg_proc: DLGPROC16;
    let HVar2: HWND16;
    let local_BX_5: *mut astruct_32;
    let handle: HANDLE16;
    let mut uvar3: u16;
    let lVar4: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    handle = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (&local_BX_5.field_0xc == 0) {
        uVar3 = (_PTR_LOOP_1050_5bc8 >> 0x10);
        dlg_proc = (_PTR_LOOP_1050_5bc8 + 4);
        HVar2 = (_PTR_LOOP_1050_5bc8 + 6);
    } else {
        dlg_proc = local_BX_5.field_0xc;
        HVar2 = local_BX_5.field_0xe;
    }
    HVar2 = CreateDialog16(
        dlg_proc,
        HVar2,
        CONCAT22(local_BX_5.field_0xa, local_BX_5.h_instance),
        0,
    );
    local_BX_5.h_window = HVar2;
    GetWindowText16(0x50, param_1 & 0xffff0000 | ZEXT24(local_BX_5 + 1), HVar2);
    lVar4 = GetWindowLong16(-4, local_BX_5.h_window);
    SetWindowLong16(_PTR_LOOP_1050_5bcc, (_PTR_LOOP_1050_5bcc >> 0x10));
    SetProp16(local_BX_5, s_thisLo_1050_5dcd, local_BX_5.h_window);
    SetProp16(handle, s_thisHi_1050_5dd4, local_BX_5.h_window);
    local_a = lVar4;
    SetProp16(local_a, s_procLo_1050_5ddb, local_BX_5.h_window);
    local_8 = (lVar4 >> 0x10);
    SetProp16(local_8, s_procHi_1050_5de2, local_BX_5.h_window);
    ppcVar1 = (param_1 + 0x50);
    (**ppcVar1)(offset, param_1);
    return;
}

pub fn win_fn_1040_748c(param_1: *mut astruct_124, param_2: u16_00, param_3: u16, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let unaff_SS: HWND16;
    let mut uvar3: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2._2_2_) {
        0xfa => {
            ppcVar1 = ((param_1 + 1) + 0x18);
            (**ppcVar1)()
        }
        _ => {
            win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        0xfd => {
            if (u16_1050_0ecc == 0) {
                return;
            }
            u16_1050_0ecc = 0;
            // goto LAB_1040_755d;
        }
        0xfe => {
            if (u16_1050_0ecc == 1) {
                return;
            }
            u16_1050_0ecc = 1;
            // goto LAB_1040_755d;
        }
        0xff => {
            if (u16_1050_0ecc == 2) {
                return;
            }
            u16_1050_0ecc = 2;
            // LAB_1040_755d:
            uVar2 = (param_1 + 1);
            ppcVar1 = ((param_1 + 1) + 0x10);
            (**ppcVar1)(0x40, uVar2, (uVar2 >> 0x10));
            uVar2 = (param_1 + 1);
            pass1_1010_2ee2(uVar2, (uVar2 >> 0x10));
            PostMessage16(0, 0x10a, 0x111, &param_1.field_0x6)
        }
        0x107 => {
            uVar3 = 0;
            // goto LAB_1040_75ba;
        }
        0x108 => {
            uVar3 = 1;
            // LAB_1040_75ba:
            uVar2 = (param_1 + 1);
            destroy_win_1010_3202(uVar2, (uVar2 >> 0x10), uVar3)
        }
        0x10a => {
            GetClientRect16(CONCAT22(unaff_SS, &local_a), &param_1.field_0x6);
            uVar2 = (param_1 + 1);
            local_8 = local_8 + 3;
            local_a = (uVar2 + 0x1a) - 9;
            local_6 = local_6 - 3;
            local_4 = local_4 - 3;
            InvalidateRect16(1, &local_a, unaff_SS);
            destroy_win_1010_2fa0((param_1 + 1));
            pass1_1010_32c0((param_1 + 1), 0);
            uVar2 = (param_1 + 1);
            pass1_1010_2ee2(uVar2, (uVar2 >> 0x10))
        }
        0x10c => {
            DestroyWindow16(&param_1.field_0x6);
        }
    }
    return;
}

pub fn create_win_1040_7620(
    param_1: u32,
    param_2: i32,
    param_3: *mut u16,
    param_4: u16,
    HMENmenu: u16,
) {
    let mut window_name: string;
    let local_BX_49: *mut astruct_39;
    let mut uVar1: u16;
    let mut local_a: u32;

    window_name = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        param_4,
    );
    local_a = 0x50000009;
    if (param_2 != 0) {
        local_a = 0x50020009;
    }
    uVar1 = (param_3 >> 0x10);
    local_BX_49 = param_3;
    CreateWindow16(
        0,
        g_h_instance_1050_038c,
        menu,
        (param_1 + 6),
        local_BX_49.field_0x6,
        local_BX_49.field_0x4,
        local_BX_49.field_0x2,
        unsafe { *param_3 },
        local_a,
        window_name,
        s_button_1050_5da8,
    );
    return;
}

pub fn create_win_1040_70b4(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut uVar4: i32;
    let h_wnd: *mut u32;
    let mut iVar5: i32;
    let struct_a: *mut astruct_199;
    let mut extraout_DX: u16;
    let struct_a_00: *mut astruct_199;
    let struct_a_01: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let paVar6: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let mut extraout_DX_02: u16;
    let struct_a_02: *mut astruct_199;
    let mut extraout_DX_03: u16;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let ppVar10: *mut pass1_struct_1;
    let dVar11: u32;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let mut hdc: u16;
    let mut in_stack_0000ff8a: u16;
    let mut local_6e: u16;
    let mut local_6c: u16;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 80];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ff8a, 0x34));
    uVar4 = ppVar10;
    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    (iVar7 + 0x94) = uVar4;
    (iVar7 + 0x96) = (ppVar10 >> 0x10);
    uVar9 = (iVar7 + 0x94);
    ppcVar3 = ((iVar7 + 0x94) + 4);
    (**ppcVar3)(0x10, uVar9, (ppVar10 >> 0x10), 0, iVar7, uVar8);
    paVar6 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar6 | uVar4) == 0) {
        (iVar7 + 0x98) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((paVar6 >> 8), CONCAT12(paVar6, uVar4)),
            (iVar7 + 6),
        );
        (iVar7 + 0x98) = uVar4;
        (iVar7 + 0x9a) = extraout_DX;
    }
    pass1_1040_bfde((iVar7 + 0x98), (iVar7 + 0x94));
    paVar6 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    struct_a_01 = (paVar6 | uVar4);
    if (struct_a_01 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar6,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(uVar9, (iVar7 + 6)),
        );
        struct_a_01 = extraout_DX_00;
    }
    process_struct_1000_179c(0x42, struct_a_01);
    paVar6 = (struct_a_01 | uVar4);
    if (paVar6 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            struct_a_01,
            1,
            0xa0028,
            0x830000,
            0x10c0082,
            CONCAT22(uVar9, (iVar7 + 6)),
        );
        paVar6 = extraout_DX_01;
    }
    uVar12 = 0;
    uVar16 = 0;
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | uVar4) == 0) {
        uVar4 = 0;
        local_4 = 0;
    } else {
        win_fn_1008_3bd6(
            uVar4,
            paVar6,
            1,
            0xa00aa,
            0x1000101,
            0x10700ff,
            CONCAT13(uVar16, CONCAT12(uVar12, (iVar7 + 6))),
        );
        local_4 = extraout_DX_02;
    }
    local_6 = uVar4;
    enable_window_1040_9234(uVar4, local_4, uVar12);
    uVar12 = 0;
    uVar16 = 0;
    paVar6 = struct_a_02;
    process_struct_1000_179c(0x42, struct_a_02);
    if ((paVar6 | uVar4) == 0) {
        uVar4 = 0;
        local_4 = 0;
    } else {
        win_fn_1008_3bd6(
            uVar4,
            paVar6,
            1,
            0xa00c2,
            0x1000101,
            0x10800ff,
            CONCAT13(uVar16, CONCAT12(uVar12, (iVar7 + 6))),
        );
        local_4 = extraout_DX_03;
    }
    local_6 = uVar4;
    enable_window_1040_9234(uVar4, local_4, uVar12);
    local_8 = GetDC16((iVar7 + 6));
    uVar15 = unaff_SS;
    uVar17 = (unaff_SS >> 8);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT13(uVar17, CONCAT12(uVar15, local_58)),
        0x7cd,
    );
    uVar13 = SUB21(local_58, 0);
    uVar14 = (local_58 >> 8);
    uVar12 = uVar15;
    uVar16 = uVar17;
    hdc = local_8;
    uVar4 = get_string_index_1000_3da4(CONCAT22(unaff_SS, local_58));
    dVar11 = GetTextExtent16(
        uVar4,
        CONCAT13(uVar16, CONCAT12(uVar12, CONCAT11(uVar14, uVar13))),
        hdc,
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    CreateWindow16(
        0,
        g_h_instance_1050_038c,
        0x7cd,
        (iVar7 + 6),
        local_5a,
        local_5c,
        0xad,
        0x22,
        0x50000000,
        CONCAT13(uVar17, CONCAT12(uVar15, local_58)),
        s_static_1050_5d9a,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT22(unaff_SS, local_58),
        0x7ce,
    );
    uVar13 = local_8;
    uVar14 = (local_8 >> 8);
    uVar12 = SUB21(local_58, 0);
    uVar16 = (local_58 >> 8);
    uVar4 = get_string_index_1000_3da4(CONCAT13(uVar17, CONCAT12(uVar15, local_58)));
    dVar11 = GetTextExtent16(
        uVar4,
        CONCAT22(unaff_SS, CONCAT11(uVar16, uVar12)),
        CONCAT11(uVar14, uVar13),
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    ReleaseDC16(local_8, (iVar7 + 6));
    CreateWindow16(
        0,
        g_h_instance_1050_038c,
        0x7ce,
        (iVar7 + 6),
        local_5a,
        local_5c,
        0xc5,
        0x22,
        0x50000000,
        CONCAT13(uVar17, CONCAT12(uVar15, local_58)),
        s_static_1050_5da1,
    );
    local_64 = 0x5a000a;
    local_60 = 0x140050;
    h_wnd = &local_64;
    uVar12 = SUB41(param_1, 0);
    create_win_1040_7620(uVar12, uVar8, 1, h_wnd, uVar15, 0x5eb, 0xfd, (iVar7 + 6));
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_7620(uVar12, uVar8, 0, 0x9c, uVar15, 0x5ed, 0xfe, (iVar7 + 6));
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_7620(uVar12, uVar8, 0, 0x9c, uVar15, 0x5ef, 0xff, (iVar7 + 6));
    SendMessage16(0, 1, 0x401, h_wnd);
    uVar1 = (iVar7 + 0x94);
    iVar5 = uVar1;
    uVar1 = uVar1 & 0xffff0000;
    uVar9 = (uVar1 >> 0x10);
    SetWindowPos16(
        0x40,
        (iVar5 + 0x10),
        (iVar5 + 0xe),
        (iVar5 + 0xc),
        (uVar1 | iVar5 + 10),
        0,
        (iVar7 + 6),
    );
    u16_1050_0ecc = 0;
    uVar2 = (iVar7 + 0x94);
    ppcVar3 = ((iVar7 + 0x94) + 0x10);
    (**ppcVar3)(offset, uVar2, (uVar2 >> 0x10));
    uVar2 = (iVar7 + 0x94);
    pass1_1010_2ee2(uVar2, (uVar2 >> 0x10));
    PostMessage16(0, 0x10a, 0x111, (iVar7 + 6));
    return;
}

pub fn enable_window_1040_6ff2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut iVar3: i32;
    let mut uVar4: u16;

    if (param_2 == 8) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar2 = GetDlgItem16(0x107, (iVar3 + 6));
        uVar1 = (iVar3 + 0x94);
        EnableWindow16((uVar1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, (iVar3 + 6));
        uVar1 = (iVar3 + 0x94);
        EnableWindow16((uVar1 + 0x26), HVar2);
    }
    return;
}

pub fn create_win_1040_6eae(
    param_1: u32,
    param_2: u16,
    param_3: *mut astruct_38,
    param_4: u16,
    menu: u16,
) {
    let mut window_name: string;
    let local_BX_49: *mut astruct_38;
    let mut in_stack_0000000c: u16;
    let mut local_a: u32;

    window_name = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        param_4,
    );
    local_a = 0x50000009;
    if (param_2 != 0) {
        local_a = 0x50020009;
    }
    CreateWindow16(
        0,
        g_h_instance_1050_038c,
        menu,
        (param_1 + 6),
        param_3.field_0x6,
        param_3.field_0x4,
        param_3.field_0x2,
        *_param_3,
        local_a,
        window_name,
        s_button_1050_5d92,
    );
    return;
}

pub fn destroy_win_1040_6d1a(
    param_1: *mut astruct_124,
    param_2: u16_00,
    param_3: u16,
    param_2: u32,
) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let unaff_SS: HWND16;
    let mut uvar3: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2._2_2_) {
        0xfa => {
            ppcVar1 = ((param_1 + 1) + 0x18);
            (**ppcVar1)()
        }
        _ => {
            win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        0xfd => {
            if (u16_1050_0ecc == 0) {
                return;
            }
            u16_1050_0ecc = 0;
            // goto LAB_1040_6deb;
        }
        0xfe => {
            if (u16_1050_0ecc == 1) {
                return;
            }
            u16_1050_0ecc = 1;
            // goto LAB_1040_6deb;
        }
        0xff => {
            if (u16_1050_0ecc == 2) {
                return;
            }
            u16_1050_0ecc = 2;
            // LAB_1040_6deb:
            uVar2 = (param_1 + 1);
            ppcVar1 = ((param_1 + 1) + 0x10);
            (**ppcVar1)(0x40, uVar2, (uVar2 >> 0x10));
            uVar2 = (param_1 + 1);
            pass1_1010_2ee2(uVar2, (uVar2 >> 0x10));
            PostMessage16(0, 0x10a, 0x111, &param_1.field_0x6)
        }
        0x107 => {
            uVar3 = 0;
            // goto LAB_1040_6e48;
        }
        0x108 => {
            uVar3 = 1;
            // LAB_1040_6e48:
            uVar2 = (param_1 + 1);
            destroy_win_1010_3202(uVar2, (uVar2 >> 0x10), uVar3)
        }
        0x10a => {
            GetClientRect16(CONCAT22(unaff_SS, &local_a), &param_1.field_0x6);
            uVar2 = (param_1 + 1);
            local_8 = local_8 + 3;
            local_a = (uVar2 + 0x1a) - 9;
            local_6 = local_6 - 3;
            local_4 = local_4 - 3;
            InvalidateRect16(1, &local_a, unaff_SS);
            destroy_win_1010_2fa0((param_1 + 1));
            pass1_1010_32c0((param_1 + 1), 0);
            uVar2 = (param_1 + 1);
            pass1_1010_2ee2(uVar2, (uVar2 >> 0x10))
        }
        0x10c => {
            DestroyWindow16(&param_1.field_0x6);
        }
    }
    return;
}

pub fn create_win_1040_6942(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut uVar4: i32;
    let h_wnd: *mut astruct_38;
    let mut iVar5: i32;
    let struct_a: *mut astruct_199;
    let mut extraout_DX: u16;
    let struct_a_00: *mut astruct_199;
    let struct_a_01: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let paVar6: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let mut extraout_DX_02: u16;
    let struct_a_02: *mut astruct_199;
    let mut extraout_DX_03: u16;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let ppVar10: *mut pass1_struct_1;
    let dVar11: u32;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let mut hdc: u16;
    let mut in_stack_0000ff8a: u16;
    let mut local_6e: u16;
    let mut local_6c: u16;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 80];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ff8a, 0x33));
    uVar4 = ppVar10;
    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    (iVar7 + 0x94) = uVar4;
    (iVar7 + 0x96) = (ppVar10 >> 0x10);
    uVar9 = (iVar7 + 0x94);
    ppcVar3 = ((iVar7 + 0x94) + 4);
    (**ppcVar3)(0x10, uVar9, (ppVar10 >> 0x10), 0, iVar7, uVar8);
    paVar6 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar6 | uVar4) == 0) {
        (iVar7 + 0x98) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((paVar6 >> 8), CONCAT12(paVar6, uVar4)),
            (iVar7 + 6),
        );
        (iVar7 + 0x98) = uVar4;
        (iVar7 + 0x9a) = extraout_DX;
    }
    pass1_1040_bfde((iVar7 + 0x98), (iVar7 + 0x94));
    paVar6 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    struct_a_01 = (paVar6 | uVar4);
    if (struct_a_01 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar6,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(uVar9, (iVar7 + 6)),
        );
        struct_a_01 = extraout_DX_00;
    }
    process_struct_1000_179c(0x42, struct_a_01);
    paVar6 = (struct_a_01 | uVar4);
    if (paVar6 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            struct_a_01,
            1,
            0xa0028,
            0x830000,
            0x10c0082,
            CONCAT22(uVar9, (iVar7 + 6)),
        );
        paVar6 = extraout_DX_01;
    }
    uVar14 = 0;
    uVar16 = 0;
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | uVar4) == 0) {
        uVar4 = 0;
        local_4 = 0;
    } else {
        win_fn_1008_3bd6(
            uVar4,
            paVar6,
            1,
            0xa00aa,
            0x1000101,
            0x10700ff,
            CONCAT13(uVar16, CONCAT12(uVar14, (iVar7 + 6))),
        );
        local_4 = extraout_DX_02;
    }
    local_6 = uVar4;
    enable_window_1040_9234(uVar4, local_4, uVar14);
    uVar14 = 0;
    uVar16 = 0;
    paVar6 = struct_a_02;
    process_struct_1000_179c(0x42, struct_a_02);
    if ((paVar6 | uVar4) == 0) {
        uVar4 = 0;
        local_4 = 0;
    } else {
        win_fn_1008_3bd6(
            uVar4,
            paVar6,
            1,
            0xa00c2,
            0x1000101,
            0x10800ff,
            CONCAT13(uVar16, CONCAT12(uVar14, (iVar7 + 6))),
        );
        local_4 = extraout_DX_03;
    }
    local_6 = uVar4;
    enable_window_1040_9234(uVar4, local_4, uVar14);
    local_8 = GetDC16((iVar7 + 6));
    uVar15 = unaff_SS;
    uVar17 = (unaff_SS >> 8);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT13(uVar17, CONCAT12(uVar15, local_58)),
        0x7cd,
    );
    uVar12 = SUB21(local_58, 0);
    uVar13 = (local_58 >> 8);
    uVar14 = uVar15;
    uVar16 = uVar17;
    hdc = local_8;
    uVar4 = get_string_index_1000_3da4(CONCAT22(unaff_SS, local_58));
    dVar11 = GetTextExtent16(
        uVar4,
        CONCAT13(uVar16, CONCAT12(uVar14, CONCAT11(uVar13, uVar12))),
        hdc,
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    CreateWindow16(
        0,
        g_h_instance_1050_038c,
        0x7cd,
        (iVar7 + 6),
        local_5a,
        local_5c,
        0xad,
        0x22,
        0x50000000,
        CONCAT13(uVar17, CONCAT12(uVar15, local_58)),
        s_static_1050_5d84,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT22(unaff_SS, local_58),
        0x7ce,
    );
    uVar12 = local_8;
    uVar13 = (local_8 >> 8);
    uVar14 = SUB21(local_58, 0);
    uVar16 = (local_58 >> 8);
    uVar4 = get_string_index_1000_3da4(CONCAT13(uVar17, CONCAT12(uVar15, local_58)));
    dVar11 = GetTextExtent16(
        uVar4,
        CONCAT22(unaff_SS, CONCAT11(uVar16, uVar14)),
        CONCAT11(uVar13, uVar12),
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    ReleaseDC16(local_8, (iVar7 + 6));
    CreateWindow16(
        0,
        g_h_instance_1050_038c,
        0x7ce,
        (iVar7 + 6),
        local_5a,
        local_5c,
        0xc5,
        0x22,
        0x50000000,
        CONCAT13(uVar17, CONCAT12(uVar15, local_58)),
        s_static_1050_5d8b,
    );
    local_64 = 0x5a000a;
    local_60 = 0x140050;
    h_wnd = &local_64;
    create_win_1040_6eae(param_1, 1, h_wnd, 0x5eb, 0xfd);
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_6eae(param_1, 0, &local_64, 0x5ec, 0xfe);
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_6eae(param_1, 0, &local_64, 0x5ee, 0xff);
    SendMessage16(0, 1, 0x401, h_wnd);
    uVar1 = (iVar7 + 0x94);
    iVar5 = uVar1;
    uVar1 = uVar1 & 0xffff0000;
    uVar9 = (uVar1 >> 0x10);
    SetWindowPos16(
        0x40,
        (iVar5 + 0x10),
        (iVar5 + 0xe),
        (iVar5 + 0xc),
        (uVar1 | iVar5 + 10),
        0,
        (iVar7 + 6),
    );
    u16_1050_0ecc = 0;
    uVar2 = (iVar7 + 0x94);
    ppcVar3 = ((iVar7 + 0x94) + 0x10);
    (**ppcVar3)(offset, uVar2, (uVar2 >> 0x10));
    uVar2 = (iVar7 + 0x94);
    pass1_1010_2ee2(uVar2, (uVar2 >> 0x10));
    PostMessage16(0, 0x10a, 0x111, (iVar7 + 6));
    return;
}

pub fn enable_window_1040_6880(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut iVar3: i32;
    let mut uVar4: u16;

    if (param_2 == 8) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar2 = GetDlgItem16(0x107, (iVar3 + 6));
        uVar1 = (iVar3 + 0x94);
        EnableWindow16((uVar1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, (iVar3 + 6));
        uVar1 = (iVar3 + 0x94);
        EnableWindow16((uVar1 + 0x26), HVar2);
    }
    return;
}

pub fn msg_box_1040_64ca(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x7ff,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x800,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x801,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x802,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_DX, in_AX));
    return;
}

pub fn win_fn_1040_65ba(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let struct_a: *mut astruct_199;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut unaff_SI: u16;
    let mut uVar6: u16;
    let unaff_SS: HWND16;
    let mut uVar7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2b));
    g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_0898();
    if (__g_astruct_94_ptr_1 == 0) {
        _g_astruct_94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
    uVar2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_astruct_94_ptr_1,
        g_u16_ptr_1050_5f2e,
    );
    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    (iVar4 + 0x8e) = uVar2;
    (iVar4 + 0x90) = g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        _local_1a = pass1_1010_0946(_local_6, (_local_6 >> 0x10), local_a);
        struct_a = (_local_1a >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        uVar2 = (iVar4 + 6);
        puVar3 = &local_22;
        MapDialogRect16(CONCAT22(puVar3, 0x1010), unaff_SS);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | puVar3) == 0) {
            uVar1 = (iVar4 + 0x8e);
            (uVar1 + local_a * 4) = 0;
        } else {
            uVar7 = win_fn_1008_3bd6(
                puVar3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(uVar2, (iVar4 + 6)),
            );
            uVar1 = (iVar4 + 0x8e);
            uVar2 = (uVar1 >> 0x10);
            iVar5 = uVar1;
            (iVar5 + local_a * 4) = uVar7;
            (iVar5 + local_a * 4 + 2) = (uVar7 >> 0x10);
        }
        uVar1 = (iVar4 + 0x8e);
        uVar2 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        if ((iVar5 + local_a * 4) != 0) {
            uVar1 = (iVar5 + local_a * 4);
            enable_window_1040_9234(uVar1, (uVar1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    move_window_1040_826c(param_1, uVar6, 0xff, 0xffff);
    ShowWindow16(5, (iVar4 + 6));
    return;
}

pub fn win_fn_1040_5800(param_1: *mut astruct_124, param_2: u16_00, param_3: u16, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: i32;
    let puVar4: *mut u8;
    let h_wnd: HWND16;
    let mut uVar5: u32;
    let extraout_DX: *mut astruct_199;
    let paVar6: *mut astruct_199;
    let mut iVar7: i32;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let unaff_SS: HWND16;
    let mut local_24: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 8];
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ == 0xeb) {
        _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
        paVar6 = (_local_6 >> 0x10);
        uVar5 = &param_1.field_0x90;
        if (uVar5 != 0) {
            local_a = uVar5;
            process_struct_1000_179c(0x18, paVar6);
            uVar3 = uVar5;
            if ((paVar6 | uVar3) == 0) {
                uVar3 = 0;
                paVar6 = 0x0;
            } else {
                process_struct_1040_a598((uVar5 & 0xffff | ZEXT24(paVar6) << 0x10));
                paVar6 = extraout_DX;
            }
            param_1.field_0x90 = uVar3;
            &param_1.field_0x92 = paVar6;
            *&param_1.field_0x90 = 6;
            local_c = *&param_1.field_0x90;
            uVar3 = local_c * 10 + 2;
            process_struct_1000_179c(uVar3, paVar6);
            _local_18 = CONCAT22(paVar6, uVar3);
            if ((paVar6 | uVar3) == 0) {
                uVar2 = &param_1.field_0x90;
                (uVar2 + 2) = 0;
            } else {
                *_local_18 = local_c;
                call_fn_ptr_1000_5586(0xa564, &PTR_LOOP_1050_1040, local_c, 10, uVar3 + 2, paVar6);
                uVar2 = &param_1.field_0x90;
                uVar8 = (uVar2 >> 0x10);
                iVar7 = uVar2;
                (iVar7 + 2) = uVar3 + 2;
                (iVar7 + 4) = paVar6;
            }
            uVar2 = &param_1.field_0x90;
            (uVar2 + 6) = (local_a + 6);
            uVar2 = &param_1.field_0x90;
            (uVar2 + 10) = 4;
            uVar2 = &param_1.field_0x90;
            (uVar2 + 0x12) = &param_1.field_0xa;
            uVar8 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505d78, &param_1.field_0x90);
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                uVar8 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            ppcVar1 = (CONCAT22(param_2_00, param_1) + 0x70);
            (**ppcVar1)(uVar8, param_1, param_2_00);
            puVar4 = pass1_1040_5cd6(CONCAT22(param_2_00, param_1));
            if (puVar4 != 0x0) {
                pass1_1040_5eaa(CONCAT22(param_2_00, param_1));
                (param_1 + 1) = 0;
            }
            pass1_1040_5dc4(CONCAT22(param_2_00, param_1));
            GetWindowRect16(CONCAT22(local_14, uVar8), unaff_SS);
            InvalidateRect16(&param_1[1].field_0x8, 0x0, 0);
            if (&param_1[1].field_0x8 != 0) {
                &param_1[1].field_0x8 = 0;
            }
        }
    } else {
        if (param_2._2_2_ != 0x13b) {
            win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        h_wnd = GetDlgItem16(0x1790, &param_1.field_0x6);
        EnableWindow16(1, h_wnd);
    }
    return;
}

pub fn enable_window_1040_5780(param_1: *mut u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uvar3: u16;
    let h_wnd: HWND16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut iVar7: i32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    unsafe {
        ppcVar1 = (*param_1 + 0x74);
    }
    iVar7 = iVar4;
    (**ppcVar1)();
    ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(iVar7, 3));
    uVar2 = (iVar4 + 0x90);
    uVar3 = pass1_1010_acc0(ppVar6, (ppVar6 >> 0x10), (uVar2 + 6));
    if (uVar3 != 0) {
        h_wnd = GetDlgItem16(0x1790, (iVar4 + 6));
        EnableWindow16(1, h_wnd);
    }
    return;
}

pub fn pass1_1040_57d4(param_1: *mut u8) {
    pass1_1040_5d42(param_1);
    pass1_1040_5eaa(param_1);
    pass1_1040_5dc4(param_1);
    set_window_pos_1040_b230(param_1);
    return;
}

pub fn set_window_pos_1040_4f96(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iVar7: i32;
    let extraout_DL: u8;
    let uVar8: u8;
    let mut uVar9: u16;
    let struct_a: *mut astruct_199;
    let mut extraout_DX: u16;
    let struct_a_00: *mut astruct_199;
    let paVar10: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let paVar11: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let extraout_DX_02: *mut astruct_199;
    let extraout_DX_03: *mut astruct_199;
    let extraout_DX_04: *mut astruct_199;
    let extraout_DX_05: *mut astruct_199;
    let mut uVar12: u16;
    let mut iVar13: i32;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let ppVar16: *mut pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    ppVar16 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe4, 0x41));
    uVar9 = (ppVar16 >> 0x10);
    uVar4 = ppVar16;
    uVar14 = (param_1 >> 0x10);
    iVar13 = param_1;
    (iVar13 + 0x98) = uVar4;
    (iVar13 + 0x9a) = uVar9;
    uVar15 = (iVar13 + 0x98);
    ppcVar2 = ((iVar13 + 0x98) + 0x10);
    ppcVar2(0x1010, uVar15, uVar9);
    paVar11 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar11 | uVar4) == 0) {
        (iVar13 + 0x94) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((paVar11 >> 8), CONCAT12(paVar11, uVar4)),
            (iVar13 + 6),
        );
        (iVar13 + 0x94) = uVar4;
        (iVar13 + 0x96) = extraout_DX;
    }
    pass1_1040_bfde((iVar13 + 0x94), (iVar13 + 0x98));
    paVar11 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    paVar10 = (paVar11 | uVar4);
    if (paVar10 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar11,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(uVar15, (iVar13 + 6)),
        );
        paVar10 = extraout_DX_00;
    }
    process_struct_1000_179c(0x42, paVar10);
    paVar11 = (paVar10 | uVar4);
    if (paVar11 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar10,
            1,
            0xa0028,
            0x850000,
            0x10b0084,
            CONCAT22(uVar15, (iVar13 + 6)),
        );
        paVar11 = extraout_DX_01;
    }
    process_struct_1000_179c(0x42, paVar11);
    paVar10 = (paVar11 | uVar4);
    if (paVar10 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar11,
            1,
            0xa0046,
            0x870000,
            0x10d0086,
            CONCAT22(uVar15, (iVar13 + 6)),
        );
        paVar10 = extraout_DX_02;
    }
    process_struct_1000_179c(0x42, paVar10);
    paVar11 = (paVar10 | uVar4);
    if (paVar11 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar10,
            1,
            0xa0064,
            0x890000,
            0x10e0088,
            CONCAT22(uVar15, (iVar13 + 6)),
        );
        paVar11 = extraout_DX_03;
    }
    process_struct_1000_179c(0x42, paVar11);
    paVar10 = (paVar11 | uVar4);
    if (paVar10 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar11,
            1,
            0xa0082,
            0x830000,
            0x10c0082,
            CONCAT22(uVar15, (iVar13 + 6)),
        );
        paVar10 = extraout_DX_04;
    }
    process_struct_1000_179c(0x42, paVar10);
    paVar11 = (paVar10 | uVar4);
    if (paVar11 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar10,
            1,
            0xa00d2,
            0x8b0000,
            0xbbb008a,
            CONCAT22(uVar15, (iVar13 + 6)),
        );
        paVar11 = extraout_DX_05;
    }
    uVar9 = 0;
    process_struct_1000_179c(0x42, paVar11);
    if ((paVar11 | uVar4) == 0) {
        uVar4 = 0;
        uVar8 = 0;
    } else {
        win_fn_1008_3bd6(
            uVar4,
            paVar11,
            1,
            0xa00a0,
            0x8d008e,
            0xbbc008c,
            CONCAT22(uVar9, (iVar13 + 6)),
        );
        uVar8 = extraout_DL;
    }
    enable_window_1040_9234(uVar4, uVar8, uVar9);
    ppVar16 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar15, 3));
    uVar12 = (ppVar16 >> 0x10);
    uVar3 = ppVar16;
    uVar5 = pass1_1010_a5ac(uVar3, uVar12, (iVar13 + 0xb0));
    uVar6 = pass1_1010_ac62(uVar3, uVar12, 0x1e);
    if (uVar6 != 0) {
        pass1_1010_a5ca(uVar3, uVar12, uVar5);
        if (0 < uVar6) {
            pass1_1010_a58a(uVar3, uVar12, uVar5);
            if (uVar6 == 0) {
                enable_window_1040_9234(uVar4, uVar8, 1);
            }
        }
    }
    uVar1 = (iVar13 + 0x98);
    iVar7 = uVar1;
    uVar1 = uVar1 & 0xffff0000;
    uVar15 = (uVar1 >> 0x10);
    SetWindowPos16(
        0x40,
        (iVar7 + 0x10),
        (iVar7 + 0xe),
        (iVar7 + 0xc),
        (uVar1 | iVar7 + 10),
        0,
        (iVar13 + 6),
    );
    return;
}

pub fn pass1_1040_5238(param_1: *mut u8) -> *mut u8 {
    let ppcVar1: fn();

    ppcVar1 = ((param_1 + 0x94) + 8);
    (**ppcVar1)();
    return 0x0;
}

pub fn send_win_msg_1040_4a0a(param_1: *mut u32) -> LRESULT {
    let puVar1: *mut u16;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u32;
    let h_wnd: HWND16;
    let mut uVar5: u16;
    let WVar6: WPARAM;
    let mut uVar7: u16;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let LVar10: LRESULT;
    let l_param: LPARAM;
    let mut uVar11: u16;
    let HVar12: HWND16;
    let mut local_a: u16;
    let mut local_4: u16;

    uVar9 = (param_1 >> 0x10);
    iVar8 = param_1;
    unsafe {
        ppcVar2 = (*param_1 + 0x74);
    }
    ppcVar2();
    h_wnd = GetDlgItem16(6000, (iVar8 + 6));
    SendMessage16(0, 0, 0x40b, h_wnd);
    LVar10 = SendMessage16(0, 0, 0xb, h_wnd);
    uVar7 = (LVar10 >> 0x10);
    local_a = 0;
    while (
        uVar3 = (iVar8 + 0x90),
        puVar1 = (uVar3 + 0x10),
        unsafe { *puVar1 != local_a } && unsafe { local_a <= *puVar1 },
    ) {
        WVar6 = 0;
        uVar11 = 0x403;
        uVar3 = (iVar8 + 0x90);
        uVar3 = (uVar3 + 0xc);
        uVar5 = local_a;
        HVar12 = h_wnd;
        pass1_1040_4dcc(param_1, *(uVar3 + local_a * 2));
        LVar10 = SendMessage16(
            CONCAT13((uVar7 >> 8), CONCAT12(uVar7, uVar5)),
            WVar6,
            uVar11,
            HVar12,
        );
        uVar7 = (LVar10 >> 0x10);
        local_a = local_a + 1;
    }
    WVar6 = pass1_1040_4d7e(param_1);
    if (WVar6 == 0) {
        uVar11 = 0x40a;
        uVar4 = (iVar8 + 0x90);
        uVar3 = (iVar8 + 0x94);
        HVar12 = h_wnd;
        l_param = pass1_1010_ada6(uVar3, (uVar3 >> 0x10), 0, (uVar4 + 10));
        SendMessage16(l_param, WVar6, uVar11, HVar12);
    }
    LVar10 = SendMessage16(0, 1, 0xb, h_wnd);
    return LVar10;
}

pub fn set_win_pos_1040_4ae4(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16_00,
    param_3: u32,
) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: i32;
    let mut uVar4: u32;
    let extraout_DX: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let mut iVar6: i32;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0xeb) {
        _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
        paVar5 = (_local_6 >> 0x10);
        uVar4 = &param_1.field_0x90;
        if (uVar4 != 0) {
            local_a = uVar4;
            process_struct_1000_179c(0x18, paVar5);
            uVar3 = uVar4;
            _local_10 = (uVar4 & 0xffff | ZEXT24(paVar5) << 0x10);
            if ((paVar5 | uVar3) == 0) {
                uVar3 = 0;
                paVar5 = 0x0;
            } else {
                process_struct_1040_a598((uVar4 & 0xffff | ZEXT24(paVar5) << 0x10));
                paVar5 = extraout_DX;
            }
            param_1.field_0x90 = uVar3;
            &param_1.field_0x92 = paVar5;
            *&param_1.field_0x90 = 7;
            local_c = *&param_1.field_0x90;
            uVar3 = local_c * 10 + 2;
            process_struct_1000_179c(uVar3, paVar5);
            _local_10 = CONCAT22(paVar5, uVar3);
            if ((paVar5 | uVar3) == 0) {
                uVar2 = &param_1.field_0x90;
                (uVar2 + 2) = 0;
            } else {
                *_local_10 = local_c;
                call_fn_ptr_1000_5586(0xa564, &PTR_LOOP_1050_1040, local_c, 10, uVar3 + 2, paVar5);
                uVar2 = &param_1.field_0x90;
                uVar7 = (uVar2 >> 0x10);
                iVar6 = uVar2;
                (iVar6 + 2) = uVar3 + 2;
                (iVar6 + 4) = paVar5;
            }
            uVar7 = (local_a >> 0x10);
            iVar6 = local_a;
            uVar2 = &param_1.field_0x90;
            (uVar2 + 6) = (iVar6 + 6);
            uVar2 = &param_1.field_0x90;
            (uVar2 + 10) = (iVar6 + 10);
            uVar2 = &param_1.field_0x90;
            (uVar2 + 0x12) = (iVar6 + 0x12);
            uVar7 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505d6a, &param_1.field_0x90);
            local_14 = local_a;
            _local_10 = local_a;
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                uVar7 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            ppcVar1 = (CONCAT22(param_2, param_1) + 0x70);
            (**ppcVar1)(uVar7, param_1, param_2);
        }
    } else {
        if (param_3._2_2_ != 6000) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        if (param_3 == 7) {
            GetWindowRect16(CONCAT22(&local_24, unaff_CS), unaff_SS);
            local_20 = local_20 - local_24;
            SetWindowPos16(2, 0x50, local_20, 0, 0, 0, param_3_00);
        }
    }
    return;
}

pub fn send_win_fn_1040_4cb2(param_1: *mut u8) -> LRESULT {
    let h_wnd: HWND16;
    let l_param: LPARAM;
    let LVar1: LRESULT;
    let w_param: WPARAM16;
    let mut msg: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    set_dlg_item_txt_1040_b45e(param_1, uVar2);
    h_wnd = GetDlgItem16(6000, (param_1 + 6));
    w_param = 0xffff;
    msg = 0x40d;
    l_param = pass1_1040_4d7e(param_1);
    pass1_1040_4dcc(param_1, l_param);
    LVar1 = SendMessage16(l_param, w_param, msg, h_wnd);
    return LVar1;
}

pub fn win_fn_1040_477e(param_1: u32) {
    let uVar1: u8;
    let mut uVar2: u16;
    let extraout_var: u32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut extraout_DX: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_stack_0000ffee: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut uVar3: u32;

    set_window_pos_1040_b230(param_1, (param_1 >> 0x10));
    ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffee, 3));
    uVar4 = (ppVar6 >> 0x10);
    uVar8 = SUB42(&g_alloc_addr_1050_1050, 0);
    uVar7 = 0x5d68;
    uVar5 = uVar4;
    uVar1 = string_fn_1008_5fd8(0x7d3, -0x15);
    uVar3 = CONCAT31(extraout_var, uVar1);
    uVar2 = uVar3;
    process_string_1000_3cea((uVar3 & 0xffff | uVar5 << 0x10), CONCAT22(uVar8, uVar7));
    pass1_1010_e964(ppVar6, uVar4);
    process_string_1000_3cea(
        (uVar3 & 0xffff | uVar5 << 0x10),
        CONCAT22(extraout_DX, uVar2),
    );
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x10)),
        (uVar3 & 0xffff | uVar5 << 0x10),
    );
    error_check_1000_17ce((uVar3 & 0xffff | uVar5 << 0x10));
    return;
}

pub fn win_gui_fn_1040_45e8(
    param_1: *mut astruct_124,
    param_2: u16_00,
    param_3: u16,
    param_2: u32,
) {
    let in_struct_1: *mut astruct_44;
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: i32;
    let paVar4: *mut astruct_44;
    let extraout_DX: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let mut iVar6: i32;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let ppVar8: *mut pass1_struct_1;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ != 0xeb) {
        win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
        return;
    }
    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
    paVar5 = (ppVar8 >> 0x10);
    in_struct_1 = &param_1.field_0x90;
    if (in_struct_1 != 0x0) {
        paVar4 = in_struct_1;
        process_struct_1000_179c(0x18, paVar5);
        uVar3 = paVar4;
        if ((paVar5 | uVar3) == 0) {
            uVar3 = 0;
            paVar5 = 0x0;
        } else {
            process_struct_1040_a598((paVar4 & 0xffff | ZEXT24(paVar5) << 0x10));
            paVar5 = extraout_DX;
        }
        param_1.field_0x90 = uVar3;
        &param_1.field_0x92 = paVar5;
        *&param_1.field_0x90 = 0x14;
        iVar6 = &param_1.field_0x90;
        uVar3 = iVar6 * 10 + 2;
        process_struct_1000_179c(uVar3, paVar5);
        _local_10 = CONCAT22(paVar5, uVar3);
        if ((paVar5 | uVar3) == 0) {
            uVar2 = &param_1.field_0x90;
            (uVar2 + 2) = 0;
        } else {
            *_local_10 = iVar6;
            call_fn_ptr_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar6, 10, uVar3 + 2, paVar5);
            uVar2 = &param_1.field_0x90;
            uVar7 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            (iVar6 + 2) = uVar3 + 2;
            (iVar6 + 4) = paVar5;
        }
        uVar2 = &param_1.field_0x90;
        (uVar2 + 6) = (in_struct_1 + 6);
        uVar2 = &param_1.field_0x90;
        (uVar2 + 10) = 1;
        uVar2 = &param_1.field_0x90;
        (uVar2 + 0x12) = &param_1.field_0xa;
        uVar7 = 0x1010;
        pass1_1010_a50c(ppVar8, 0x10505d40, &param_1.field_0x90);
        if (in_struct_1 != 0x0) {
            pass1_1040_a5d0(in_struct_1);
            uVar7 = 0x1000;
            error_check_1000_17ce(in_struct_1);
        }
        ppcVar1 = (CONCAT22(param_2_00, param_1) + 0x70);
        (**ppcVar1)(uVar7, param_1, param_2_00);
    }
    return;
}

pub fn win_fn_1040_410e(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let unaff_SS: *mut char;
    let ppVar4: *mut pass1_struct_1;
    let puVar5: *mut u16;
    let puVar6: *mut u16;
    let pcVar7: *mut char;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: [u8; 6];
    let mut local_2a: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 10];

    win_gui_func_1040_78e2(param_1);
    pass1_1000_4906(CONCAT22(unaff_SS, local_c), 0, 10);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0x8e);
    string_fn_1000_3f9c(
        local_c,
        unaff_SS,
        0x5d38,
        &g_alloc_addr_1050_1050,
        (uVar1 + 0x76),
    );
    local_e = GetDlgItem16(0xfb5, (iVar2 + 6));
    SendMessage16(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, local_c)),
        0,
        0xc,
        local_e,
    );
    SetFocus16(local_e);
    SendMessage16(-0x10000, 0, 0x401, local_e);
    GetWindowRect16(CONCAT22(&local_16, 0x1538), unaff_SS);
    pass1_1000_4906(CONCAT22(unaff_SS, &local_1e), 0, 8);
    uVar1 = (iVar2 + 0x8e);
    _local_22 = pass1_1010_5f7a(uVar1, (uVar1 >> 0x10), 0, 7);
    if (_local_22 != 0x0) {
        _local_1e = *_local_22;
        _local_1a = (_local_22 + 4);
    }
    if ((local_1c == 0) && (local_1e == 0)) {
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_30));
        uVar1 = (iVar2 + 0x96);
        pass1_1018_2678(uVar1, (uVar1 >> 0x10), CONCAT22(unaff_SS, local_30));
        pass1_1008_3e94(
            local_30,
            CONCAT22(unaff_SS, &local_32),
            CONCAT22(unaff_SS, &local_2a),
        );
        puVar6 = &local_34;
        puVar5 = &local_36;
        pcVar7 = unaff_SS;
        ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar5, 0x48));
        pass1_1008_3e94(
            (ppVar4 + 0xe),
            CONCAT22(unaff_SS, puVar5),
            CONCAT22(pcVar7, puVar6),
        );
        _local_1a = CONCAT22(local_10 - local_14, local_12 - local_16);
        _local_1e = CONCAT22(
            (((ppVar4 + 0xc) * -0x14) / 600 - (local_10 - local_14)) + local_36 + local_32,
            local_34 + local_2a,
        );
    }
    move_window_1040_826c(iVar2, uVar3, local_1c, local_1e);
    ShowWindow16(5, (iVar2 + 6));
    return;
}

pub fn enable_win_1040_42b2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let puVar3: *mut u8;
    let mut uVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let LVar7: LRESULT;
    let mut local_66: u32;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    iVar5 = param_1;
    uVar6 = (param_1 >> 0x10);
    if (param_2 == 0) {
        (iVar5 + 0x9a) = 1;
        DestroyWindow16((iVar5 + 6));
        return;
    }
    pass1_1000_4906(CONCAT22(unaff_SS, local_54), 0, 0x51);
    HVar2 = GetDlgItem16(0xfb5, (iVar5 + 6));
    LVar7 = SendMessage16(CONCAT22(unaff_SS, local_54), 0x51, 0xd, HVar2);
    uVar4 = (LVar7 >> 0x10);
    puVar3 = local_54;
    pass1_fn_1000_3e2c(CONCAT22(unaff_SS, puVar3));
    if ((uVar4 | puVar3) != 0) {
        (iVar5 + 0x92) = puVar3;
        (iVar5 + 0x94) = uVar4;
    }
    if (uVar4 < 0) {
        uVar1 = (iVar5 + 0x8e);
        wsprintf16(
            local_54,
            CONCAT22(0x5d3c, unaff_SS),
            CONCAT22((uVar1 + 0x76), 0x1050),
        );
        SendMessage16(CONCAT22(unaff_SS, local_54), 0, 0xc, HVar2);
        SetFocus16(HVar2);
        SendMessage16(-0x10000, 0, 0x401, HVar2);
        return;
    }
    HVar2 = GetDlgItem16(1, (iVar5 + 6));
    EnableWindow16(0, HVar2);
    uVar1 = (iVar5 + 0x8e);
    (uVar1 + 0x76) = (iVar5 + 0x92);
    PostMessage16((iVar5 + 0x92), 0, 0x400, g_h_window);
    HVar2 = GetDlgItem16(1, (iVar5 + 6));
    EnableWindow16(1, HVar2);
    return;
}

pub fn get_win_rect_1040_43ea(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut local_12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    GetWindowRect16(CONCAT22(&local_a, unaff_CS), unaff_SS);
    local_6 = local_6 - local_a;
    local_4 = local_4 - local_8;
    pass1_1010_5fb0((iVar2 + 0x8e), 0, &local_a, unaff_SS, 7);
    uVar1 = (iVar2 + 0x8e);
    (uVar1 + 0x7a) = ((iVar2 + 0x9a) == 0);
    return;
}

pub fn send_dialog_item_msg_1040_3f12(param_1: u32, param_2: u32) {
    let puVar1: *mut u8;
    let mut extraout_DX: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let LVar5: LRESULT;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, 400, (iVar3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, 400, (iVar3 + 6));
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), param_2);
    while (true) {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            break;
        }
        LVar5 = SendDlgItemMessage16((puVar1 + 4), 0, 0x401, 400, (iVar3 + 6));
        iVar2 = (LVar5 >> 0x10);
        if (((LVar5 == -1) && (iVar2 == -1)) || (LVar5 == -2 && (iVar2 == -1))) {
            break;
        }
    }
    SendDlgItemMessage16(0, 0, 0x407, 400, (iVar3 + 6));
    SendDlgItemMessage16(0, 1, 0xb, 400, (iVar3 + 6));
    return;
}

pub fn win_gui_dialog_func_1040_3e08(param_1: *mut astruct_25) {
    let mut uVar1: u16;
    let local_BX_4: *mut astruct_25;
    let mut uVar2: u16;
    let LVar3: LRESULT;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    CheckRadioButton16(local_BX_4.check_id, 0x18d, 0x188, local_BX_4.h_wnd);
    local_BX_4.dlg_item_id = 0;
    local_BX_4.dlg_item_id_2 = 0;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, 400, local_BX_4.h_wnd);
    if (LVar3 != -1) {
        uVar1 = pass1_1018_3ab2(local_BX_4.field_0x8e, LVar3, local_BX_4.check_id);
        local_BX_4.dlg_item_id_2 = uVar1;
    }
    SetDlgItemInt16(0, local_BX_4.dlg_item_id, 0x18e, local_BX_4.h_wnd);
    SetDlgItemInt16(0, local_BX_4.dlg_item_id_2, 0x191, local_BX_4.h_wnd);
    match (local_BX_4.check_id) {
        0x188 => local_BX_4.field_0xa4 = 5,
        0x189 => local_BX_4.field_0xa4 = 6,
        0x18a => local_BX_4.field_0xa4 = 7,
        0x18b => local_BX_4.field_0xa4 = 8,
        0x18c => local_BX_4.field_0xa4 = 9,
        0x18d => {
            local_BX_4.field_0xa4 = 10;
        }
    }
    invalidate_rect_1040_3ddc(param_1);
    return;
}

pub fn invalidate_rect_1040_3ddc() {
    let unaff_SS: HWND16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_a = 0x780005;
    local_6 = 0xdc0069;
    InvalidateRect16(0, &local_a, unaff_SS);
    return;
}

pub fn release_dc_1040_3d5e(param_1: u32) -> u16 {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let puVar5: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar2 = param_1;
    local_4 = GetDC16((iVar2 + 6));
    puVar5 = mixed_fn_1010_830a(_g_struct_73_1050_14cc, (iVar2 + 0xa4));
    unsafe {
        iVar3 = *puVar5;
    }
    ppcVar1 = (iVar3 + 8);
    unsafe {
        (**ppcVar1)(0x1010, puVar5, &local_4);
    }
    ppcVar1 = (iVar3 + 4);
    unsafe {
        (**ppcVar1)(0x1010, puVar5, 0x50078, &local_4);
    }
    ppcVar1 = (iVar3 + 0xc);
    (**ppcVar1)(0x1010, puVar5, &local_4);
    ReleaseDC16(local_4, (iVar2 + 6));
    return 0;
}

pub fn win_func_1040_3c64(param_1: i32, param_2: u16, param_3: u16_00, param_3: u32) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let LVar3: LRESULT;
    let paVar4: *mut astruct_318;
    let BVar5: bool;
    let mut local_8: u16;

    if (param_3._2_2_ == (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x14)) {
        LVar3 = SendDlgItemMessage16(0, 0, 0x409, 400, (param_1 + 6));
        uVar1 = GetDlgItemInt16(0, 0x0, 0, 0x18e);
        set_struct_1018_36e6((param_1 + 0x8e), uVar1, LVar3, (param_1 + 0xa0));
        pass1_1038_af40(_g_astruct_112_a, *(param_1 + 8), 0x22);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        BVar5 = 1;
        paVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
        pass1_1010_038e(paVar4, BVar5);
    } else {
        if (param_3._2_2_ + -0xc3 < &dos_alloc_addr_1050_0002) {
            // LAB_1040_3c7f:
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        if (param_3._2_2_ + -0xc4 < (&PTR_DAT_0005_0000_1050_0004 + 1)
            || param_3._2_2_ == (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x1b))
        {
            (param_1 + 0xa0) = param_3._2_2_;
            uVar2 = string_fn_1018_3b9e((param_1 + 0x8e), param_3._2_2_);
            send_dialog_item_msg_1040_3f12(param_1, param_2, uVar2);
        } else {
            if (param_3._2_2_ + -0xc4 != &BYTE_1050_0008) {}
            // goto LAB_1040_3c7f;
            if (param_3 != 1) {
                return;
            }
        }
        win_gui_dialog_func_1040_3e08(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn gui_win_fn_1040_3b1e(param_1: *mut astruct_25) {
    let in_struct_a: *mut astruct_298;
    let BVar1: bool;
    let HVar2: HWND16;
    let mut extraout_DX: u16;
    let mut iVar3: i32;
    let mut unaff_SI: u16;
    let mut uVar4: i32;
    let mut unaff_SS: u16;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let puStack268: *mut u8;
    let BStack264: bool;
    let paStack262: *mut astruct_566;
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    local_a = (_local_6 + 0x68);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_SS, local_8c), (iVar3 + 6));
    wsprintf16(
        &local_10e,
        CONCAT22(local_8c, unaff_SS),
        CONCAT22(local_a, unaff_SS),
    );
    SetWindowText16(CONCAT22(unaff_SS, &local_10e), (iVar3 + 6));
    pass1_1018_3d44(
        ((iVar3 + 0x8e) & 0xffff | (iVar3 + 0x96) << 0x10),
        param_1 & 0xffff0000 | uVar4,
        param_1 & 0xffff0000 | (iVar3 + 0x96),
    );
    local_11a = (_g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        local_11a,
        0x80,
        CONCAT22(unaff_SS, &local_10e),
        0x7d7,
    );
    local_118 = *CONCAT22(0x80, local_11a);
    wsprintf16(
        local_8c,
        CONCAT22(&local_10e, unaff_SS),
        CONCAT22(local_118, unaff_SS),
    );
    paStack262 = (iVar3 + 6);
    BStack264 = 0x187;
    puStack268 = local_8c;
    local_10e = SUB42(offset, 0);
    SetDlgItemText16(CONCAT22(unaff_SS, puStack268), 0x187, paStack262);
    paStack262 = (iVar3 + 6);
    BStack264 = 0x188;
    puStack268 = 0x188;
    local_10e = SUB42(offset, 0);
    BVar1 = CheckRadioButton16(0x188, 0x18d, 0x188, paStack262);
    (iVar3 + 0xa0) = 0x188;
    paStack262 = (iVar3 + 0xa0);
    in_struct_a = (iVar3 + 0x8e);
    BStack264 = (in_struct_a >> 0x10);
    puStack268 = offset;
    local_10e = 0x3c2b;
    string_fn_1018_3b9e(in_struct_a, paStack262);
    local_10e = 0x1018;
    puStack268 = iVar3;
    BStack264 = BVar1;
    paStack262 = extraout_DX;
    send_dialog_item_msg_1040_3f12();
    puStack268 = 0x3c47;
    BStack264 = iVar3;
    paStack262 = uVar4;
    win_gui_dialog_func_1040_3e08(param_1);
    paStack262 = (iVar3 + 6);
    BStack264 = 0x186;
    puStack268 = 0x3c56;
    HVar2 = GetDlgItem16(0x186, paStack262);
    (iVar3 + 0x9a) = HVar2;
    return;
}

pub fn enable_window_1040_3a36(param_1: u32, param_2: u16_00, param_3: u16, param_2: i32) -> u16 {
    let piVar1: *mut i32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_4: u16;

    bVar2 = false;
    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        if ((iVar3 + 0x9e) <= (iVar3 + 0x9c)) {}
        // goto LAB_1040_3a79;
        piVar1 = (iVar3 + 0x9c);
        unsafe {
            *piVar1 = *piVar1 + 1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1040_3a79;
        if ((iVar3 + 0x9c) == 0) {}
        // goto LAB_1040_3a79;
        piVar1 = (iVar3 + 0x9c);
        unsafe {
            *piVar1 = *piVar1 + -1;
        }
    }
    bVar2 = true;
    // LAB_1040_3a79:
    if (bVar2) {
        SetDlgItemInt16(0, *(iVar3 + 0x9c), 0x18e, (iVar3 + 6));
    }
    if (((iVar3 + 0x9c) != 0) && ((iVar3 + 0xa2) == 0)) {
        (iVar3 + 0xa2) = 1;
        EnableWindow16(1, (iVar3 + 0x9a));
    }
    if (((iVar3 + 0x9c) == 0) && ((iVar3 + 0xa2) != 0)) {
        (iVar3 + 0xa2) = 0;
        EnableWindow16(0, (iVar3 + 0x9a));
    }
    return 0;
}

pub fn win_fn_1040_3ae8(param_1: *mut astruct_20) {
    let mut uVar1: u16;

    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1040_37f0(param_1: i32, param_2: u16, param_3: u16_00, param_3: u32) {
    let mut unaff_SS: u16;
    let mut in_stack_0000fbec: u16;
    let BVar1: bool;
    let mut local_40c: [u8; 1026];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0x193) {
        _local_6 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fbec, 2));
        local_a = (_local_6 + 0x68);
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_SS, local_40c),
            0x44f,
        );
        MessageBox16(0x30, local_a, CONCAT22(unaff_SS, local_40c), (param_1 + 6));
        pass1_1018_3710((param_1 + 0x8e));
        PostMessage16(0, 2, 0x111, (param_1 + 6));
    } else {
        if (param_3._2_2_ != 0x194) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        pass1_1038_af40(_g_astruct_112_a, *(param_1 + 8), 0x21);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        BVar1 = 1;
        _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
        pass1_1010_038e(_local_6, BVar1);
    }
    return;
}

pub fn set_focus_1040_355a(param_1: *mut astruct_20) {
    let mut uVar1: u16;

    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1040_3590(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let hwnd: HWND16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_d_x_01: i32;
    let mut i_var3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_5b0: u16;
    let mut uStack1454: u16;
    let uStack1450: u8;
    let uStack1449: u8;
    let mut local_5a8: u16;
    let mut local_5a6: u16;
    let mut local_5a4: u16;
    let mut local_5a2: u16;
    let mut local_5a0: u16;
    let mut local_59e: u16;
    let mut local_59c: u16;
    let mut local_59a: u32;
    let mut local_596: u32;
    let mut local_592: u16;
    let mut local_590: u16;
    let mut local_58e: u16;
    let mut uStack1420: u16;
    let uStack1418: u8;
    let uStack1417: u8;
    let HStack1410: HWND16;
    let HStack1408: HWND16;
    let mut local_50c: [u8; 256];
    let mut local_40c: u32;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: [u8; 1026];

    _local_408 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_5b0, 2));
    local_40c = (_local_408 + 0x68);
    uVar4 = (param_1 >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_SS, local_50c), (i_var3 + 6));
    wsprintf16(
        &local_58e,
        CONCAT22(local_50c, unaff_SS),
        CONCAT22(local_40c, unaff_SS),
    );
    local_592 = SetWindowText16(CONCAT22(unaff_SS, &local_58e), (i_var3 + 6));
    uVar1 = (i_var3 + 0x8e);
    local_5a6 = uVar1;
    local_5a4 = (uVar1 >> 0x10);
    wsprintf_1018_34b6(local_5a6, local_5a4);
    local_590 = extraout_DX;
    pass1_1018_3d44(
        *(i_var3 + 0x8e),
        CONCAT22(unaff_SS, &local_59a),
        CONCAT22(unaff_SS, &local_596),
    );
    HVar2 = GetDlgItem16(0x193, (i_var3 + 6));
    (i_var3 + 0x98) = HVar2;
    EnableWindow16(1, HVar2);
    uStack1454 = _g_struct_73_1050_14cc;
    load_string_1010_84e0(
        uStack1454,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_404),
        1099,
    );
    uStack1450 = SUB21(local_404, 0);
    uStack1449 = (local_404 >> 8);
    wsprintf16(
        local_50c,
        CONCAT13(uStack1449, CONCAT12(uStack1450, unaff_SS)),
        CONCAT22(local_596, unaff_SS),
    );
    local_59a._2_2_ = (i_var3 + 6);
    local_59a._0_2_ = 0x195;
    local_59a._2_2_ = GetDlgItem16(0x195, local_59a._2_2_);
    SetWindowText16(CONCAT22(unaff_SS, local_50c), local_59a._2_2_);
    local_596._0_2_ = (i_var3 + 6);
    local_59a._2_2_ = 0x196;
    local_59a._0_2_ = offset;
    HVar2 = GetDlgItem16(0x196, local_596);
    uVar1 = (i_var3 + 0x8e);
    local_59a._0_2_ = uVar1;
    local_59a._2_2_ = (uVar1 >> 0x10);
    local_596._0_2_ = HVar2;
    wsprintf_1018_34b6();
    local_59a._0_2_ = HVar2;
    local_59a._2_2_ = extraout_DX_00;
    SetWindowText16(CONCAT22(extraout_DX_00, HVar2), local_596);
    local_596._2_2_ = (i_var3 + 6);
    local_596._0_2_ = 0x197;
    local_59a._2_2_ = offset;
    local_59a._0_2_ = 0x36cb;
    GetDlgItem16(0x197, local_596._2_2_);
    local_596._2_2_ = 0x443;
    local_59a._2_2_ = local_404;
    local_59a._0_2_ = 0x3ff;
    local_59c = (_g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        local_59c,
        0x3ff,
        CONCAT22(unaff_SS, local_59a._2_2_),
        0x443,
    );
    local_596._2_2_ = offset;
    local_59a._2_2_ = local_404;
    local_59a._0_2_ = 0x1010;
    SetWindowText16(CONCAT22(unaff_SS, local_59a._2_2_), offset);
    local_592 = 0x44c;
    local_596._0_2_ = local_404;
    local_59a._2_2_ = 0x3ff;
    local_59c = _g_struct_73_1050_14cc;
    local_59a._0_2_ = (_g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        local_59c,
        local_59a,
        0x3ff,
        CONCAT22(unaff_SS, local_596),
        0x44c,
    );
    local_596._2_2_ = local_59a;
    local_592 = local_59a._2_2_;
    local_59a._2_2_ = local_404;
    wsprintf16(
        local_50c,
        CONCAT22(local_59a._2_2_, unaff_SS),
        CONCAT22(local_59a, unaff_SS),
    );
    uStack1418 = 0x38;
    uStack1417 = 0x15;
    uStack1420 = 0x3732;
    HVar2 = GetDlgItem16(0x198, (i_var3 + 6));
    uStack1418 = SUB21(local_50c, 0);
    uStack1417 = (local_50c >> 8);
    uStack1420 = offset;
    local_58e = 0x3742;
    SetWindowText16(CONCAT22(unaff_SS, local_50c), HVar2);
    uStack1418 = 0x51;
    uStack1417 = 0x37;
    hwnd = GetDlgItem16(0x199, (i_var3 + 6));
    uStack1418 = 99;
    uStack1417 = 0x37;
    HVar2 = hwnd;
    wsprintf_1018_35b0();
    if ((extraout_d_x_01 | HVar2) == 0) {
        uStack1418 = 0xff;
        uStack1417 = 3;
        local_58e = _g_struct_73_1050_14cc;
        uStack1420 = (_g_struct_73_1050_14cc >> 0x10);
        local_590 = 0x1018;
        local_592 = 0x3785;
        load_string_1010_84e0(
            local_58e,
            uStack1420,
            0x3ff,
            CONCAT22(unaff_SS, local_404),
            0x44d,
        );
        uStack1418 = 0x10;
        uStack1417 = 0x10;
        uStack1420 = 0x3794;
        SetWindowText16(CONCAT22(unaff_SS, local_404), hwnd);
        HStack1410 = (i_var3 + 6);
        HVar2 = GetDlgItem16(0x19a, HStack1410);
        HStack1410 = 0x44e;
        uStack1420 = _g_struct_73_1050_14cc;
        uStack1418 = (_g_struct_73_1050_14cc >> 0x10);
        uStack1417 = (_g_struct_73_1050_14cc >> 0x18);
        local_58e = offset;
        local_590 = 0x37bd;
        load_string_1010_84e0(
            uStack1420,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_SS, local_404),
            0x44e,
        );
        uStack1418 = 0xcc;
        uStack1417 = 0x37;
        HStack1410 = HVar2;
        SetWindowText16(CONCAT22(unaff_SS, local_404), HVar2);
        HStack1408 = (i_var3 + 0x98);
        HStack1410 = 0;
        EnableWindow16(0, HStack1408);
        return;
    }
    uStack1418 = 0x18;
    uStack1417 = 0x10;
    uStack1420 = 0x37eb;
    SetWindowText16(CONCAT22(extraout_d_x_01, HVar2), hwnd);
    return;
}

pub fn enable_window_1040_32a8(param_1: *mut void) {
    let lp_string: SEGPTR;
    let BVar1: bool;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    _local_c = param_1 & 0xffff0000 | (param_1 + 0x19a);
    lp_string = pass1_1018_3a5c(
        (param_1 + 0x96),
        param_1 & 0xffff0000 | (param_1 + 0x9a),
        param_1 & 0xffff0000 | (param_1 + 0x19a),
    );
    SetWindowText16(lp_string, (param_1 + 0x90));
    BVar1 = pass1_1018_39d8(
        (param_1 + 0x96),
        param_1 & 0xffff0000 | (param_1 + 0x9a),
        _local_c,
    );
    EnableWindow16(BVar1 & 1, (param_1 + 0x8e));
    return;
}

pub fn set_window_pos_1040_331a(param_1: *mut void, param_2: u16, param_3: u16) -> u16 {
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_3;
    local_6 = param_2;
    if (param_3 == 1) {
        enable_window_1040_32a8(param_1);
    } else {
        if (param_3 != 7) {
            return 0;
        }
        GetWindowRect16(CONCAT22(&local_e, unaff_CS), unaff_SS);
        local_a = local_a - local_e;
        SetWindowPos16(2, 0x50, local_a, 0, 0, 0, local_6);
    }
    return 1;
}

pub fn win_fn_1040_311a(param_1: i32, param_2: u16_00, param_3: u16, param_2: u32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let ppVar3: *mut pass1_struct_2;
    let extraout_DX: *mut u8;
    let puVar4: *mut u8;
    let paVar5: *mut astruct_318;
    let mut uVar6: u16;
    let BVar7: bool;
    let mut local_12: u16;
    let mut local_10: u16;

    send_msg_1040_323c(param_1, param_2_00);
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    if (param_2._2_2_ == 0x181) {
        iVar1 = param_1 + 0x9a;
        iVar2 = iVar1;
        pass1_1018_3cda(
            (param_1 + 0x96),
            CONCAT22(param_2_00, param_1 + 0x19a),
            CONCAT22(param_2_00, iVar1),
        );
        pass1_1018_3424((param_1 + 0x96));
        if (iVar2 == 0) {
            uVar6 = 0x21;
        } else {
            pass1_1018_3a42((param_1 + 0x96), CONCAT22(param_2_00, iVar1));
            puVar4 = extraout_DX;
            ppVar3 = pass1_1030_8344(
                _g_bool_1050_5748,
                (_g_bool_1050_5748 >> 0x10),
                CONCAT22(extraout_DX, iVar2),
            );
            PTR_LOOP_1050_5f0c = pass1_1030_8344(
                _g_bool_1050_5748,
                (_g_bool_1050_5748 >> 0x10),
                &ppVar3.field_0x10,
            );
            PTR_LOOP_1050_5f10 = (&PTR_LOOP_1050_0000 + 1);
            uVar6 = 0x25;
            PTR_LOOP_1050_5f0e = puVar4;
        }
        pass1_1038_af40(_g_astruct_112_a, *(param_1 + 8), uVar6);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        BVar7 = 1;
        paVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
        pass1_1010_038e(paVar5, BVar7);
    } else {
        if ((param_2._2_2_ == 0x181) || (1 < param_2._2_2_ - 0x182)) {
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2._2_2_);
            return;
        }
        set_window_pos_1040_331a(param_1, param_2_00, param_3, param_2, param_2._2_2_);
    }
    return;
}

pub fn set_focus_1040_2f5a(param_1: *mut astruct_20) {
    let mut uVar1: u16;

    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1040_2f90(param_1: u32) {
    let HVar1: HWND16;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let ppVar5: *mut pass1_struct_1;
    let in_stack_0000fed2: u8;
    let in_stack_0000fed3: u8;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u32;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: [u8; 130];
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT13(in_stack_0000fed3, CONCAT12(in_stack_0000fed2, 2)),
    );
    local_a = (_local_6 + 0x68);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_SS, local_8c), (iVar3 + 6));
    wsprintf16(
        local_10e,
        CONCAT22(local_8c, unaff_SS),
        CONCAT22(local_a, unaff_SS),
    );
    SetWindowText16(CONCAT22(unaff_SS, local_10e), (iVar3 + 6));
    HVar1 = GetDlgItem16(0x182, (iVar3 + 6));
    (iVar3 + 0x92) = HVar1;
    pass1_1018_3a94(
        *(iVar3 + 0x96),
        CONCAT22(unaff_SS, &local_116),
        CONCAT22(unaff_SS, &local_112),
    );
    local_126 = local_112;
    local_124 = (local_112 >> 0x10);
    win_fn_1040_3374(
        iVar3,
        (param_1 >> 0x10),
        local_126,
        local_124,
        (iVar3 + 0x92),
    );
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_120, 0x2f));
    uVar2 = (ppVar5 + 0x24);
    pass1_1018_3a7a(*(iVar3 + 0x96), uVar2);
    SendMessage16(
        CONCAT13((extraout_DX >> 8), CONCAT12(extraout_DX, uVar2)),
        0xffff,
        0x40d,
        (iVar3 + 0x92),
    );
    HVar1 = GetDlgItem16(0x183, (iVar3 + 6));
    (iVar3 + 0x94) = HVar1;
    local_124 = local_116;
    local_122 = (local_116 >> 0x10);
    win_fn_1040_3374(param_1, uVar4, local_124, local_122, HVar1);
    local_124 = _g_struct_73_1050_14cc;
    local_122 = (_g_struct_73_1050_14cc >> 0x10);
    load_string_1010_847e(local_124, local_122, 0x531);
    SendDlgItemMessage16(
        CONCAT13((extraout_DX_00 >> 8), CONCAT12(extraout_DX_00, HVar1)),
        0,
        0x403,
        0x183,
        (iVar3 + 6),
    );
    SendDlgItemMessage16(0x40dffff, 0xffff, 0x40d, 0x183, (iVar3 + 6));
    HVar1 = GetDlgItem16(0x181, (iVar3 + 6));
    (iVar3 + 0x8e) = HVar1;
    HVar1 = GetDlgItem16(0x184, (iVar3 + 6));
    (iVar3 + 0x90) = HVar1;
    return;
}

pub fn enable_window_1040_2a64(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut uVar3: i32;
    let struct_a: *mut astruct_199;
    let mut iVar4: i32;
    let local_BX_215: *mut astruct_58;
    let mut uVar5: u16;
    let mut h_wnd: u16;
    let local_28: *mut astruct_57;
    let mut uStack38: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_28 = param_1;
    uStack38 = (param_1 >> 0x10);
    set_window_pos_1040_b230(local_28, uStack38);
    local_4 = 5;
    uVar1 = local_28.field_0x90;
    _local_c = pass1_1030_73a8((uVar1 + 6));
    struct_a = (_local_c >> 0x10);
    uVar5 = SUB42(&PTR_LOOP_1050_1028, 0);
    PTR_LOOP_1050_5d04 = pass1_1028_4a9a(_local_c, 0);
    local_e = 0;
    while (local_e < local_4) {
        if (local_e != 0) {
            (&PTR_LOOP_1050_5d04 + local_e * 0xc) = 0;
        }
        local_BX_215 = (local_e * 0xc);
        local_16 = (local_BX_215 + 0x5cfc);
        local_14 = (local_BX_215 + 0x5cfe);
        uVar3 = 1;
        local_12 = 1;
        local_10 = 1;
        uVar2 = local_28.field_0x6;
        MapDialogRect16(CONCAT22(&local_16, uVar5), h_wnd);
        uVar5 = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | uVar3) == 0) {
            _local_8 = 0;
        } else {
            uVar5 = SUB42(&PTR_LOOP_1050_1008, 0);
            _local_8 = win_fn_1008_3bd6(
                uVar3,
                struct_a,
                1,
                CONCAT22(local_16, local_14),
                0x1000101,
                CONCAT22((local_BX_215 + 0x5d00), 0xff),
                CONCAT22(uVar2, local_28.field_0x6),
            );
        }
        struct_a = (_local_8 >> 0x10);
        if (PTR_LOOP_1050_5d04 == 0x0) {
            if ((local_e != 0) && (_local_8 != 0)) {
                uVar5 = SUB42(offset, 0);
                EnableWindow16(0, (_local_8 + 0x18));
            }
        } else {
            iVar4 = local_e * 0xc;
            uVar5 = SUB42(&PTR_LOOP_1050_1028, 0);
            uVar2 = pass1_1028_4a9a(_local_c, (iVar4 + 0x5d02));
            if (uVar2 != 0) {
                (&PTR_LOOP_1050_5d04 + iVar4) = 1;
                uVar5 = SUB42(offset, 0);
                SetDlgItemText16(
                    local_28.field_0x98,
                    (&PTR_s_post_1050_015d_1050_5d06 + iVar4),
                    local_28.field_0x6,
                );
            }
        }
        local_e = local_e + 1;
    }
    return;
}

pub fn enable_window_1040_2bb2(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16_00,
    param_3: u32,
) {
    let mut uVar1: i32;
    let HVar2: HWND16;
    let mut iVar3: i32;
    let lp_string: SEGPTR;
    let id: *mut u8;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0x158) {
        PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04 == 0x0);
        if (PTR_LOOP_1050_5d04 == 0x0) {
            local_8 = 1;
            while (local_8 < 5) {
                iVar3 = local_8 * 0xc;
                HVar2 = GetDlgItem16((iVar3 + 0x5d00), &param_1.field_0x6);
                EnableWindow16(0, HVar2);
                (&PTR_LOOP_1050_5d04 + iVar3) = 0;
                SetDlgItemText16(
                    (param_1 + 1),
                    (&PTR_s_post_1050_015d_1050_5d06 + iVar3),
                    &param_1.field_0x6,
                );
                local_8 = local_8 + 1;
            }
            HVar2 = &param_1.field_0x6;
            lp_string = (param_1 + 1);
            id = PTR_s_post_1050_015d_1050_5d06;
            // goto LAB_1040_2ccc;
        }
        local_8 = 1;
        while (local_8 < 5) {
            iVar3 = local_8 * 0xc;
            HVar2 = GetDlgItem16((iVar3 + 0x5d00), &param_1.field_0x6);
            EnableWindow16(1, HVar2);
            (&PTR_LOOP_1050_5d04 + iVar3) = 0;
            SetDlgItemText16(
                (param_1 + 1),
                (&PTR_s_post_1050_015d_1050_5d06 + iVar3),
                &param_1.field_0x6,
            );
            local_8 = local_8 + 1;
        }
        HVar2 = &param_1.field_0x6;
        id = PTR_s_post_1050_015d_1050_5d06;
    } else {
        if (param_3._2_2_ == 0x159) {
            local_4 = 1;
        } else {
            if (param_3._2_2_ == 0x15a) {
                local_4 = 2;
            } else {
                if (param_3._2_2_ == 0x15b) {
                    local_4 = 3;
                } else {
                    if (param_3._2_2_ != 0x15c) {
                        win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
                        return;
                    }
                    local_4 = 4;
                }
            }
        }
        if (local_4 == 0) {
            return;
        }
        iVar3 = local_4 * 0xc;
        uVar1 = ((&PTR_LOOP_1050_5d04 + iVar3) == 0);
        (&PTR_LOOP_1050_5d04 + iVar3) = uVar1;
        if (uVar1 == 0) {
            HVar2 = &param_1.field_0x6;
            lp_string = (param_1 + 1);
            id = (&PTR_s_post_1050_015d_1050_5d06 + iVar3);
            // goto LAB_1040_2ccc;
        }
        HVar2 = &param_1.field_0x6;
        id = (&PTR_s_post_1050_015d_1050_5d06 + local_4 * 0xc);
    }
    lp_string = &param_1[1].field_0x4;
    // LAB_1040_2ccc:
    SetDlgItemText16(lp_string, id, HVar2);
    return;
}

pub fn win_fn_1040_2d48(param_1: u32) {
    let mut uVar1: u16;
    let mut value: u16;
    let unaff_SS: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    set_dlg_item_txt_1040_b45e(param_1);
    uVar1 = GetDlgItemInt16(1, &local_4, unaff_SS, 0x163);
    value = GetDlgItemInt16(1, &local_4, unaff_SS, 0x165);
    if (uVar1 != 0) {
        value = value / uVar1;
    }
    SetDlgItemInt16(1, value, 0x165, (param_1 + 6));
    return;
}

pub fn enable_window_1040_2490(param_1: *mut astruct_20) {
    let ppcVar1: fn();
    let h_wnd: HWND16;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let piVar5: *mut i32;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    h_wnd = GetDlgItem16(0xfb1, (iVar3 + 6));
    EnableWindow16(0, h_wnd);
    ppcVar1 = ((iVar3 + 0x8e) + 0x10);
    piVar5 = (**ppcVar1)(offset, (iVar3 + 0x8e));
    uVar2 = (piVar5 >> 0x10);
    move_window_1040_826c(
        iVar3,
        uVar4,
        (piVar5 + 2) + -2,
        (piVar5 + 4) + unsafe { *piVar5 } + 3,
    );
    ShowWindow16(5, (iVar3 + 6));
    pass1_1018_1c9a(*(iVar3 + 0x8e), 0x1a0);
    return;
}

pub fn win_fn_1040_2512(param_1: u32, param_2: u32, param_3: u16) {
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: i32;
    let HVar5: HWND16;
    let puVar6: *mut u8;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: i32;
    let mut uVar9: i32;
    let mut iVar10: i32;
    let mut iVar11: i32;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let unaff_SS: *mut char;
    let uVar14: u8;
    let in_stack_0000ffdc: *mut char;
    let mut local_20: u16;
    let mut local_1e: [u8; 4];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_3 != 2) {
        iVar10 = param_1;
        if (0x19d < param_3 - 2) {
            uVar12 = (param_1 >> 0x10);
            uVar14 = (unaff_SS >> 8);
            if (param_3 - 0x1a0 < 0x14 || param_3 == 0x1b4) {
                uVar7 = IsDlgButtonChecked16(param_3, (iVar10 + 6));
                if (uVar7 == 0) {
                    piVar1 = (iVar10 + 0x92);
                    unsafe {
                        *piVar1 = *piVar1 + 1;
                    }
                    if (0 < (iVar10 + 0x92)) {
                        (iVar10 + 0x94) = 0;
                    }
                    uVar3 = (iVar10 + 0x8e);
                    if ((uVar3 + 0x28) == (iVar10 + 0x92)) {
                        HVar5 = GetDlgItem16(0xfb1, (iVar10 + 6));
                        EnableWindow16(0, HVar5);
                    }
                } else {
                    piVar1 = (iVar10 + 0x92);
                    unsafe {
                        *piVar1 = *piVar1 + -1;
                    }
                    HVar5 = GetDlgItem16(0xfb1, (iVar10 + 6));
                    IsWindowEnabled16(offset, HVar5);
                    uVar4 = extraout_DX;
                    if (HVar5 == 0) {
                        HVar5 = GetDlgItem16(0xfb1, (iVar10 + 6));
                        EnableWindow16(1, HVar5);
                    }
                    if ((iVar10 + 0x92) < 1) {
                        (iVar10 + 0x94) = 1;
                    }
                    pass1_1018_1c9a(*(iVar10 + 0x8e), param_3);
                    uVar8 = pass1_1018_1e78((iVar10 + 0x8e), 0xffff);
                    _local_a = (uVar8 & 0xffff | uVar4 << 0x10);
                    if ((uVar4 | uVar8) == 0) {
                        local_c = 0;
                    } else {
                        local_c = (uVar8 + 0x1c);
                    }
                    mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, CONCAT22(local_c, 1));
                }
                if ((iVar10 + 0x92) < 0) {
                    return;
                }
                uVar3 = (iVar10 + 0x8e);
                if ((uVar3 + 0x28) < (iVar10 + 0x92)) {
                    return;
                }
                string_fn_1000_3f9c(
                    &local_16,
                    unaff_SS,
                    s__d_1050_5cf4,
                    &g_alloc_addr_1050_1050,
                    *(iVar10 + 0x92),
                );
                SetDlgItemText16(
                    CONCAT13(uVar14, CONCAT12(unaff_SS, &local_16)),
                    0xfb2,
                    (iVar10 + 6),
                );
                return;
            }
            uVar4 = param_3 - 0xfb1;
            if (uVar4 == 0) {
                if ((iVar10 + 0x92) < 0) {
                    process_struct_1000_179c(0xb4, in_DX);
                    uVar9 = in_DX | uVar4;
                    local_1a = uVar4;
                    if (uVar9 == 0) {
                        uVar4 = 0;
                        uVar9 = 0;
                    } else {
                        mixed_1040_8520(
                            CONCAT13((in_DX >> 8), CONCAT12(in_DX, uVar4)),
                            g_h_window,
                            0x30,
                            2,
                            0x57b,
                            0x57c,
                        );
                    }
                    _local_a = CONCAT22(uVar9, uVar4);
                    ppcVar2 = (*_local_a + 0x74);
                    ppcVar2(0, uVar4, uVar9);
                    return;
                }
                if (0 < (iVar10 + 0x92)) {
                    process_struct_1000_179c(0xb4, in_DX);
                    uVar9 = in_DX | uVar4;
                    local_1a = uVar4;
                    if (uVar9 == 0) {
                        uVar4 = 0;
                        uVar9 = 0;
                    } else {
                        mixed_1040_8520(
                            CONCAT13((in_DX >> 8), CONCAT12(in_DX, uVar4)),
                            g_h_window,
                            0x21,
                            2,
                            0x57b,
                            0x57d,
                        );
                    }
                    _local_a = CONCAT22(uVar9, uVar4);
                    pass1_1008_941a(CONCAT13(uVar14, CONCAT12(unaff_SS, local_1e)), 1, 0xc2);
                    puVar6 = local_1e;
                    ppcVar2 = (*_local_a + 0x6c);
                    ppcVar2(&PTR_LOOP_1050_1008, _local_a, (_local_a >> 0x10), puVar6);
                    in_stack_0000ffdc = unaff_SS;
                    if (puVar6 == &dos_alloc_addr_1050_0002) {
                        return;
                    }
                }
                _local_16 = process_struct_1010_20ba(
                    _g_astruct_372_1050_0ed0,
                    CONCAT22(in_stack_0000ffdc, 6),
                );
                local_c = 0x1a0;
                while {
                    uVar7 = IsDlgButtonChecked16(local_c, (iVar10 + 6));
                    if (uVar7 == 1) {
                        uVar13 = (_local_16 >> 0x10);
                        iVar11 = _local_16;
                        (iVar11 + (iVar11 + 0x56) * 2 + 0x4e) = local_c;
                        piVar1 = (iVar11 + 0x56);
                        unsafe {
                            *piVar1 = *piVar1 + 1;
                        }
                    }
                    local_c = local_c + 1;
                    local_c < 0x1b5
                } {}
                uVar4 = (iVar10 + 0x92);
                _local_a = (_local_a & 0xffff0000 | uVar4);
                uVar3 = (iVar10 + 0x8e);
                (uVar3 + 0x28) = uVar4;
                PostMessage16(0, 200, 0x111, g_h_window);
                param_3 = 1;
            }
        }
        post_win_msg_1040_7b3c(
            iVar10,
            (param_1 >> 0x10),
            param_2,
            (param_2 >> 0x10),
            param_3,
        );
    }
    return;
}

pub fn create_win_1040_20d4(param_1: *mut astruct_20) {
    let mut cx: i32;
    let puVar1: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000ffca: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: [u8; 4];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffca, 0x48));
    local_c = (ppVar4 >> 0x10);
    local_e = ppVar4;
    local_8 = (local_e + 10);
    local_a = (local_e + 0xc);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    _local_12 = process_struct_1008_4772((iVar2 + 0x8e));
    cx = (_local_12 + 4);
    local_4 = (local_8 - cx) / 2;
    local_6 = 5;
    SetWindowPos16(6, 0x1d6, cx, 5, local_4, 0, (iVar2 + 6));
    puVar1 = local_1e;
    GetClientRect16(CONCAT22(unaff_SS, puVar1), (iVar2 + 6));
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x592,
    );
    local_16 = 0x50010001;
    CreateWindow16(
        0,
        g_h_instance_1050_038c,
        1,
        (iVar2 + 6),
        0x19,
        0x58,
        local_18 - 0x28,
        (local_1a - 0x58) / 2,
        0x50010001,
        CONCAT22(extraout_DX, puVar1),
        s_OPButton_1050_5ce4,
    );
    SetWindowPos16(
        0x45,
        local_a - 10,
        (_local_12 + 4),
        5,
        local_4,
        0,
        (iVar2 + 6),
    );
    return;
}

pub fn show_window_1040_1d50(param_1: *mut astruct_20) {
    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn check_dialog_btn_1040_1d7a(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let local_BX_8: *mut astruct_51;
    let mut uvar3: u16;

    local_BX_8 = param_1;
    uVar3 = (param_1 >> 0x10);
    if ((param_2 != 0) && (uVar1 = local_BX_8.field_0x8e, (uVar1 + 0x72) != 0)) {
        uVar2 = IsDlgButtonChecked16(0xe1, local_BX_8.field_0x6);
        if (uVar2 != 0) {
            pass1_1008_a930(local_BX_8.field_0x92, 0x1d5);
        }
        uVar2 = IsDlgButtonChecked16(0xe2, local_BX_8.field_0x6);
        if (uVar2 != 0) {
            pass1_1008_a930(local_BX_8.field_0x92, 0x1d6);
        }
        uVar2 = IsDlgButtonChecked16(0xe3, local_BX_8.field_0x6);
        if (uVar2 != 0) {
            pass1_1008_a930(local_BX_8.field_0x92, 0x1d7);
        }
        uVar2 = IsDlgButtonChecked16(0xe5, local_BX_8.field_0x6);
        if (uVar2 != 0) {
            pass1_1008_a930(local_BX_8.field_0x92, 0x1d8);
        }
        uVar2 = IsDlgButtonChecked16(0xe6, local_BX_8.field_0x6);
        if (uVar2 != 0) {
            pass1_1008_a930(local_BX_8.field_0x92, 0x1e2);
        }
        uVar2 = IsDlgButtonChecked16(0xe7, local_BX_8.field_0x6);
        if (uVar2 != 0) {
            pass1_1008_a930(local_BX_8.field_0x92, 0x1dc);
        }
        return;
    }
    DestroyWindow16(local_BX_8.field_0x6);
    return;
}

pub fn pass1_1040_1ab0(param_1: i32, param_2: *mut u8, param_3: *mut u8, param_2_00: *mut u8) {
    let mut local_6: u32;

    local_6 = 0;
    if (param_2_00._2_2_ == 0x1831) {
        (param_1 + 0x92) = 1;
        (param_1 + 0x94) = 1;
        check_dialog_func_1040_1b8a(param_1, param_2);
    } else {
        local_6 = post_win_msg_1040_7b3c(param_1, CONCAT22(param_3, param_2), param_2_00);
    }
    return local_6;
}

pub fn check_dialog_func_1040_1afe(param_1: *mut astruct_20) {
    let mut check: u16;
    let mut check_00: u16;
    let mut check_01: u16;
    let mut u_var1: u32;
    let mut uVar2: u32;
    let local_astruct_20: *mut astruct_20;
    let mut local_ES_4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_astruct_20 = param_1;
    uVar1 = local_astruct_20.field_0x8e;
    uVar2 = local_astruct_20.field_0x8e;
    check = *(uVar2 + 0x20);
    uVar2 = local_astruct_20.field_0x8e;
    check_00 = *(uVar2 + 0x74);
    uVar2 = local_astruct_20.field_0x8e;
    check_01 = *(uVar2 + 0x72);
    CheckDlgButton16(*(uVar1 + 0x1e), 0xfdb, local_astruct_20.h_dialog_6);
    CheckDlgButton16(check_00, 0xfdd, local_astruct_20.h_dialog_6);
    CheckDlgButton16(check_01, 0xfde, local_astruct_20.h_dialog_6);
    CheckDlgButton16(check, 0xfdc, local_astruct_20.h_dialog_6);
    return;
}

pub fn check_dialog_func_1040_1b8a(param_1: u32) {
    let mut uVar1: u16;
    let mut check: u16;
    let local_BX_4: *mut astruct_21;
    let mut uVar2: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = return_1_1010_60b4(local_BX_4.field_0x8e);
    return_1_1010_60c6(local_BX_4.field_0x8e);
    check = return_1_1010_60c0();
    return_1_1010_60ba(local_BX_4.field_0x8e);
    CheckDlgButton16(uVar1, 0xfdb, local_BX_4.h_wnd);
    CheckDlgButton16(check, 0xfdd, local_BX_4.h_wnd);
    CheckDlgButton16(0xfde, 0xfde, local_BX_4.h_wnd);
    uVar1 = local_BX_4.h_wnd;
    CheckDlgButton16(uVar1, 0xfdc, uVar1);
    return;
}

pub fn set_win_pos_1040_162a(param_1: u16, param_2: u32, param_3: u32) {
    let mut uVar1: i32;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut uVar2: u32;
    let mut local_a: u16;
    let mut local_6: u16;

    if ((param_3._2_2_ != (s_vrpal_bmp_1050_183a + 5))
        && (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 4)))
    {
        uVar2 = post_win_msg_1040_7b3c(param_1, param_2, param_3);
        return uVar2;
    }
    if (param_3 == 7) {
        GetWindowRect16(CONCAT22(&local_a, unaff_CS), unaff_SS);
        local_6 = local_6 - local_a;
        SetWindowPos16(2, 0x50, local_6, 0, 0, 0, param_2._2_2_);
    } else {
        if ((param_3 != 9) && (param_3 != 10)) {
            uVar1 = 0;
            // goto LAB_1040_164d;
        }
    }
    uVar1 = 1;
    // LAB_1040_164d:
    return uVar1;
}

pub fn win_fn_1040_12bc(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let unaff_SS: *mut char;
    let l_param: LPARAM;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    win_gui_func_1040_78e2(param_1);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = (iVar3 + 0x8e);
    string_fn_1000_3f9c(
        local_54,
        unaff_SS,
        s__u_1050_5cd4,
        &g_alloc_addr_1050_1050,
        *(uVar1 + 10),
    );
    HVar2 = GetDlgItem16(0xfd2, (iVar3 + 6));
    SendMessage16(CONCAT22(unaff_SS, local_54), 0, 0xc, HVar2);
    SetFocus16(HVar2);
    SendMessage16(-0x10000, 0, 0x401, HVar2);
    move_window_1040_826c(param_1, uVar4, 0xffff, 0xffff);
    l_param = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    HVar2 = GetDlgItem16((s_vrpal_bmp_1050_183a + 5), (iVar3 + 6));
    send_msg_1040_1696(iVar3, uVar4, HVar2);
    SendMessage16(l_param, 0xffff, 0x40d, HVar2);
    HVar2 = GetDlgItem16((s_vrpal_bmp_1050_183a + 4), (iVar3 + 6));
    send_msg_1040_1696(iVar3, uVar4, HVar2);
    SendMessage16(l_param, 0xffff, 0x40d, HVar2);
    ShowWindow16(5, (iVar3 + 6));
    return;
}

pub fn pass1_1040_109c(param_1: i32, param_2: *mut u8, param_3: *mut u8, param_3_00: *mut u8) {
    let mut u_var1: u32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000fff2: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    bVar2 = false;
    if (param_3_00._2_2_ == 0x1c1) {
        (param_1 + 0x96) = 2;
        bVar2 = true;
    } else {
        if (param_3_00._2_2_ == 0x1c2) {
            (param_1 + 0x96) = 1;
            bVar2 = true;
        } else {
            if (param_3_00._2_2_ != 0x1830) {
                post_win_msg_1040_7b3c(param_1, param_2, param_3, param_3_00);
                return;
            }
            ppVar4 = process_struct_1010_20ba(
                _g_astruct_372_1050_0ed0,
                CONCAT22(in_stack_0000fff2, 0x32),
            );
            uVar1 = (param_1 + 0x92);
            iVar3 = win_gui_fn_1010_79aa(ppVar4, 0xfb6, (uVar1 + 6));
            if (iVar3 == 0) {
                uVar1 = (param_1 + 0x92);
                uVar1 = (uVar1 + 6);
                window_msg_func_1010_7300(ppVar4, 0, 0, 0xc, uVar1, (uVar1 >> 0x10));
            }
        }
    }
    if (bVar2) {
        uVar1 = (param_1 + 0x8e);
        (uVar1 + 10) = (param_1 + 0x96);
    }
    return;
}

pub fn set_win_pos_1040_0f0c(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut uVar6: u32;
    let id: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_2e: [u8; 2];
    let mut local_2c: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x98) == 0) {
        GetWindowRect16(
            CONCAT13((&local_26 >> 8), CONCAT12(&local_26, unaff_CS)),
            unaff_SS,
        );
        GetDlgItem16(0x1830, (iVar4 + 6));
        GetWindowRect16(CONCAT22(local_2e, 0x1538), unaff_SS);
        local_22 = local_22 - local_26;
        local_20 = (local_2c - local_24) - 2;
        SetWindowPos16(6, local_20, local_22, 0, 0, 0, (iVar4 + 6));
        CheckDlgButton16(1, 0x1c1, (iVar4 + 6));
        uVar1 = (iVar4 + 0x8e);
        (uVar1 + 10) = 2;
        HVar2 = GetDlgItem16(0x1830, (iVar4 + 6));
        EnableWindow16(0, HVar2);
    } else {
        uVar1 = (iVar4 + 0x92);
        uVar6 = pass1_1030_73a8((uVar1 + 6));
        if ((uVar6 + 0x20) == 2) {
            HVar2 = (iVar4 + 6);
            id = 0x1c1;
        } else {
            HVar2 = (iVar4 + 6);
            id = 0x1c2;
        }
        CheckDlgButton16(1, id, HVar2);
    }
    GetCursor16(offset, &local_6);
    uVar3 = (iVar4 + 6);
    GetWindowRect16(CONCAT22(&local_e, 0x1538), unaff_SS);
    local_14 = local_a - local_e;
    local_10 = -(local_14 / 2 - local_6);
    local_16 = local_8 - local_c;
    local_12 = -(local_16 / 2 - local_4);
    _local_1e = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar3, 0x48));
    uVar3 = (_local_1e >> 0x10);
    local_18 = (_local_1e + 10);
    local_1a = (_local_1e + 0xc);
    if (local_18 < (local_14 + local_10)) {
        local_10 = local_18 - local_14;
    }
    if (local_1a < (local_16 + local_12)) {
        local_12 = local_1a - local_16;
    }
    SetWindowPos16(0x45, 0, 0, local_12, local_10, 0, (iVar4 + 6));
    return;
}

pub fn set_colors_1040_0cc0(param_1: *mut u32, param_2: u16_00, uparam_2: i32, param_3: HDC16) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    local_4 = GetStockObject16(4);
    if (_PTR_LOOP_1050_5cd0 == 0) {
        uVar3 = (param_1 >> 0x10);
        unsafe {
            ppcVar1 = (*param_1 + 0x68);
        }
        uVar4 = (**ppcVar1)(offset, param_1, uVar3, (param_1 + 0x6e));
        uVar4 = pass1_1008_4d72(uVar4);
        uVar3 = (uVar4 >> 0x10);
        iVar2 = uVar4;
        _PTR_LOOP_1050_5cd0 = CONCAT22(
            CONCAT11(2, *(iVar2 + 0x94)),
            CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)),
        );
    }
    if (3 < param_2) {
        if (param_2 == 4) {
            local_4 = GetStockObject16(5);
        } else {
            if ((param_2 == 4) || (1 < param_2 - 5)) {
                return 0;
            }
        }
    }
    SetTextColor16(_PTR_LOOP_1050_5cd0, param_3);
    SetBkColor16(0x1000000, param_3);
    return CONCAT22(0x1050, local_4);
}

pub fn show_window_1040_0c7c(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_6: u32;

    win_gui_func_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x8e);
    pass1_1010_4f30(
        uVar1,
        (uVar1 >> 0x10),
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_6 + 2),
    );
    move_window_1040_826c(param_1, local_6);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn enable_win_1040_0acc(param_1: u32, param_2: bool) {
    let BVar1: bool;
    let HVar2: HWND16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    BVar1 = IsWindow16((iVar3 + 6));
    if (BVar1 != 0) {
        HVar2 = GetDlgItem16(100, (iVar3 + 6));
        BVar1 = IsWindow16(HVar2);
        if (BVar1 != 0) {
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x74, (iVar3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x73, (iVar3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x6e, (iVar3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0xee, (iVar3 + 6));
            EnableWindow16(param_2, HVar2);
        }
    }
    return;
}

pub fn win_fn_1040_07dc(param_1: i32, param_2: u16_00, param_2: u16, param_4: u16, uparam_3: i32) {
    let ppcVar1: fn();
    let uVar2: u8;
    let iVar3: u16;
    let BVar4: bool;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar6: i32;
    let mut extraout_DX_01: u16;
    let mut unaff_SS: u16;
    let paVar7: *mut astruct_434;
    let puVar8: *mut u32;
    let uVar9: u8;
    let uVar10: u8;
    let uVar12: u8;
    let mut local_810: u32;
    let mut local_80c: u16;
    let mut local_80a: u16;
    let mut local_808: u16;
    let mut local_806: [u8; 1024];
    let mut local_406: u32;
    let mut local_6: u32;
    let mut uVar5: u32;
    let uVar11: u8;

    local_6 = 0;
    uVar9 = param_1;
    uVar10 = param_2_00;
    uVar11 = unaff_SS;
    uVar12 = (unaff_SS >> 8);
    if (param_3 == 0x73) {
        enable_win_1040_0acc(param_1, uVar10, 0);
        uVar6 = extraout_DX_00;
        uVar2 = string_fn_1008_5fd8(0x7d0, -0x2f);
        uVar5 = CONCAT31(extraout_var, uVar2);
        local_406 = uVar5 & 0xffff | uVar6 << 0x10;
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(uVar12, CONCAT12(uVar11, local_806)),
            1999,
        );
        iVar3 = MessageBox16(
            0x34,
            CONCAT13(uVar12, CONCAT12(uVar11, local_806)),
            CONCAT13((uVar6 >> 8), CONCAT12(uVar6, uVar5)),
            g_h_window,
        );
        error_check_1000_17ce((uVar5 & 0xffff | uVar6 << 0x10));
        if (iVar3 == 6) {
            BVar4 = PostMessage16(0, 0xcb, 0x111, g_h_window);
            post_win_msg_1040_7b3c(uVar9, param_2_00, param_2, param_4, 1);
            local_6 = CONCAT22(extraout_DX_01, BVar4);
        }
    } else {
        if (param_3 < 0x74) {
            if (param_3 == 0x6e) {
                (_g_astruct_112_a + 0xae) = 0x99;
                puVar8 = pass1_1038_af40(_g_astruct_112_a, *(param_1 + 6), 2);
                unsafe {
                    ppcVar1 = (*puVar8 + 0x3c);
                }
                (**ppcVar1)(&PTR_LOOP_1050_1038, puVar8, (puVar8 >> 0x10));
                SetFocus16((param_1 + 6));
                return;
            }
            if (0x6e < param_3) {
                // LAB_1040_09f9:
                post_win_msg_1040_7b3c(uVar9, uVar10, param_2, param_4, param_3);
                return;
            }
            if (param_3 == 0x2) {
                // LAB_1040_09b4:
                post_win_msg_1040_7b3c(uVar9, uVar10, 0, 0, 2);
                PostMessage16(0, 0xee, 0x111, g_h_window);
                return;
            }
            if (param_3 != 'd') {}
            // goto LAB_1040_09f9;
            PostMessage16(0, 100, 0x111, g_h_window);
            local_810._0_2_ = 0;
            // goto LAB_1040_0821;
        }
        if (param_3 != 0x74) {
            if (param_3 == 0xee) {}
            // goto LAB_1040_09b4;
            if (param_3 == 0x13d) {
                enable_win_1040_0acc(param_1, uVar10, 1);
                return;
            }
            // goto LAB_1040_09f9;
        }
        enable_win_1040_0acc(param_1, uVar10, 0);
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(uVar12, CONCAT12(uVar11, &local_406)),
            0x756,
        );
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(uVar12, CONCAT12(uVar11, local_806)),
            0x757,
        );
        iVar3 = MessageBox16(
            0x34,
            CONCAT13(uVar12, CONCAT12(uVar11, &local_406)),
            CONCAT13(uVar12, CONCAT12(uVar11, local_806)),
            g_h_window,
        );
        if (iVar3 == 6) {
            BVar4 = PostMessage16(0, 0x7a, 0x111, g_h_window);
            post_win_msg_1040_7b3c(uVar9, param_2_00, param_2, param_4, 1);
            local_6 = CONCAT22(extraout_DX, BVar4);
            paVar7 =
                process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_810._2_2_, 2));
            process_struct_1010_60fa(paVar7);
        }
    }
    local_810._0_2_ = 1;
    // LAB_1040_0821:
    enable_win_1040_0acc(uVar9, param_2_00, local_810);
    return;
}

pub fn show_win_1040_0766(param_1: *mut astruct_20) {
    let mut unaff_SS: u16;
    let ppVar1: *mut pass1_struct_1;
    let p_uvar2: *mut u16;
    let mut uvar3: u16;
    let puVar4: *mut u16;
    let mut in_stack_0000ffde: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffde, 2));
    process_struct_1010_6118(_local_6);
    puVar4 = &local_8;
    puVar2 = &local_a;
    uVar3 = unaff_SS;
    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(unaff_SS, puVar2),
        CONCAT22(uVar3, puVar4),
    );
    uVar3 = (param_1 >> 0x10);
    move_window_1040_826c(param_1, uVar3, local_a + 0x8c, local_8 + 0xb9);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn enable_window_1040_060e(param_1: u32, param_2: u16) {
    let piVar1: *mut u16;
    let h_wnd: HWND16;
    let mut unaff_SS: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    _local_8 = CONCAT22(unaff_SS, &stack0x000a);
    local_a = param_2;
    while (true) {
        piVar1 = _local_8;
        if (local_a == 0) {
            break;
        }
        _local_8 = (_local_8 & 0xffff0000 | (local_8 + 2));
        local_a = local_a - 1;
        h_wnd = GetDlgItem16(unsafe { *piVar1 }, (param_1 + 6));
        EnableWindow16(0, h_wnd);
    }
    return;
}

pub fn enable_window_1040_0558(param_1: u32, param_2: i32) -> LRESULT {
    let mut iVar1: i32;
    let l_param: LPARAM;
    let HVar2: HWND16;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let LVar6: LRESULT;
    let w_param: WPARAM16;
    let mut msg: u16;
    let id: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar1 = param_2 * 0xe;
    HVar2 = GetDlgItem16((iVar1 + 0x5c64), (iVar4 + 6));
    iVar3 = pass1_1010_659a((iVar4 + 0x8e), (iVar1 + 0x5c66));
    if ((iVar3 == 0) && ((iVar1 + 0x5c66) != 10)) {
        EnableWindow16(0, HVar2);
        HVar2 = (iVar4 + 6);
        id = (param_2 * 0xe + 0x5c68);
        uVar5 = 0x531;
    } else {
        EnableWindow16(1, HVar2);
        HVar2 = (iVar4 + 6);
        id = (param_2 * 0xe + 0x5c68);
        uVar5 = 0x649;
    }
    msg = 0xc;
    w_param = 0;
    l_param = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        uVar5,
    );
    LVar6 = SendDlgItemMessage16(l_param, w_param, msg, id, HVar2);
    return LVar6;
}

pub fn destroy_win_1040_0170(param_1: i32, param_2: u16_00, param_3: u16, param_2: u32) {
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let HVar4: HWND16;
    let ppVar5: *mut pass1_struct_2;
    let mut uVar6: u16;
    let mut extraout_DX: u16;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let ppVar8: *mut pass1_struct_1;
    let uVar9: u8;
    let uVar10: u8;
    let uVar11: u8;
    let uVar12: u8;
    let mut uVar13: u16;
    let mut local_12a: u16;
    let mut local_128: u16;
    let HStack294: HWND16;
    let mut uStack292: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 8;
    local_6 = 0;
    match (param_2._2_2_) {
        0x167 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x168, 0x69, 0x16a);
            HVar4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, HVar4);
            local_4 = 0
        }
        0x168 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x69, 0x16a);
            HVar4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, HVar4);
            local_4 = 1
        }
        0x169 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x68, 0x16a);
            HVar4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, HVar4);
            local_4 = 2
        }
        0x16a => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x68, 0x169);
            HVar4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, HVar4);
            local_4 = 3
        }
        0x16b => {
            HVar4 = GetDlgItem16(0x16b, (param_1 + 6));
            uVar7 = offset;
            EnableWindow16(0, HVar4);
            if ((param_1 + 0x92) != 3) {
                uVar7 = &PTR_LOOP_1050_1008;
                mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1de);
            }
            if ((param_1 + 0x92) != 8) {
                iVar3 = (param_1 + 0x92) * 0xe;
                local_6 = (iVar3 + 0x5c6c);
                uVar7 = 0x1010;
                pass1_1010_6604((param_1 + 0x8e), (iVar3 + 0x5c66));
                (param_1 + 0x92) = 8;
            }
            local_8 = 0;
            while (local_8 < 4) {
                enable_window_1040_0558(param_1, param_2_00, local_8);
                local_8 = local_8 + 1;
            }
            // goto LAB_1040_04da;
        }
        0x16c => {
            HVar4 = GetDlgItem16(0x16d, (param_1 + 6));
            EnableWindow16(1, HVar4);
            local_4 = 5;
            (param_1 + 0x94) = 5;
            // goto LAB_1040_01bf;
        }
        0x16d => {
            HVar4 = GetDlgItem16(0x16d, (param_1 + 6));
            EnableWindow16(0, HVar4);
            mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1de);
            if ((param_1 + 0x94) != 8) {
                iVar3 = (param_1 + 0x94) * 0xe;
                local_6 = (iVar3 + 0x5c6c);
                pass1_1010_6604((param_1 + 0x8e), (iVar3 + 0x5c66));
                (param_1 + 0x94) = 8;
            }
            enable_window_1040_0558(param_1, param_2_00, 5);
            _local_c = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x39));
            local_10 = (_local_c + 0x20);
            uVar11 = SUB21(&local_16, 0);
            uVar12 = (&local_16 >> 8);
            uVar9 = SUB21(&local_18, 0);
            uVar10 = (&local_18 >> 8);
            uVar7 = (local_10 >> 0xf) + 0x200;
            uVar13 = unaff_SS;
            local_e = uVar7;
            local_8 = local_10;
            ppVar5 = pass1_1030_8344(
                _g_bool_1050_5748,
                (_g_bool_1050_5748 >> 0x10),
                CONCAT22(uVar7, local_10),
            );
            _local_14 = CONCAT22(uVar7, ppVar5);
            pass1_1030_2f1a(
                CONCAT22(uVar7, ppVar5),
                CONCAT22(unaff_SS, CONCAT11(uVar10, uVar9)),
                CONCAT22(uVar13, CONCAT11(uVar12, uVar11)),
            );
            local_16 = local_16 + (local_18 - local_16) / 2;
            local_1a = pass1_1030_2fac(_local_14);
            uVar2 = (param_1 + 0x96);
            uVar7 = 0x1018;
            win_fn_1018_6086(uVar2, (uVar2 >> 0x10), local_1a, local_16);
            // goto LAB_1040_04da;
        }
        0x16e => {
            _local_1e =
                process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x39));
            local_1a = (_local_1e + 0x20);
            local_18 = LoadCursor16(0x7f02, 0);
            local_16 = SetCursor16(local_18);
            local_12a = (local_1a + 0x2000000 >> 0x10);
            pass1_1030_532e(CONCAT22(unaff_SS, &local_12a), local_1a + 0x2000000);
            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_12a));
            local_12a = (_g_bool_1050_5748 >> 0x10);
            pass1_1030_838e(_g_bool_1050_5748);
            local_12a = (_g_bool_1050_5748 >> 0x10);
            pass1_1030_8334();
            local_12a = local_16;
            SetCursor16(local_16);
            local_128 = g_h_window;
            local_12a = 0x111;
            PostMessage16(0, 0x7e, 0x111, g_h_window);
            HStack294 = (param_1 + 6);
            local_128 = offset;
            uVar7 = offset;
            local_12a = 0x495;
            DestroyWindow16(HStack294);
            // goto LAB_1040_04da;
        }
        // default:
        _ => {
            post_win_msg_1040_7b3c();
            return;
        }
    }
    (param_1 + 0x92) = local_4;
    // LAB_1040_01bf:
    uVar7 = offset;
    // LAB_1040_04da:
    if (local_4 != 8) {
        uStack292 = (uStack292 & 0xffff0000 | *(param_1 + 6));
        HStack294 = (local_4 * 0xe + 0x5c68);
        local_12a = 0;
        local_128 = 0xc;
        uVar6 = local_4;
        load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            (local_4 * 0xe + 0x5c6a),
        );
        uVar7 = offset;
        SendDlgItemMessage16(
            CONCAT22(extraout_DX, uVar6),
            local_12a,
            local_128,
            HStack294,
            uStack292,
        );
    }
    if (local_6 != 0) {
        uStack292 = CONCAT22(uStack292._2_2_, 2);
        local_128 = _g_astruct_372_1050_0ed0;
        HStack294 = (_g_astruct_372_1050_0ed0 >> 0x10);
        local_12a = uVar7;
        ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, uStack292);
        uVar1 = (ppVar8 + 0x20);
        _local_1e = (_local_1e & 0xffff0000 | uVar1);
        if (uVar1 != 0) {
            uStack292 = (uStack292 & 0xffff0000 | g_h_window);
            HStack294 = 0x111;
            local_128 = local_6;
            local_12a = 0;
            PostMessage16(0, local_6, 0x111, g_h_window);
        }
    }
    return;
}

pub fn win_fn_1040_0000(param_1: *mut astruct_20) {
    let mut iVar1: i32;
    let p_uvar2: *mut u16;
    let mut uvar3: u16;
    let extraout_DX: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut unaff_SI: u16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut uVar6: u32;
    let puVar7: *mut u8;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    // Segment:    9
    // Offset:     0006f820
    // Length:     d974
    // Min Alloc:  d974
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    win_gui_func_1040_78e2(param_1);
    local_4 = 8;
    local_a = 0;
    struct_a = extraout_DX;
    while (
        iVar4 = param_1,
        uVar5 = (param_1 >> 0x10),
        local_a < local_4,
    ) {
        iVar1 = local_a * 0xe;
        local_24 = (iVar1 + 0x5c60);
        local_22 = (iVar1 + 0x5c62);
        local_20 = 1;
        local_1e = 1;
        uVar3 = (iVar4 + 6);
        puVar2 = &local_24;
        MapDialogRect16(CONCAT22(puVar2, unaff_CS), unaff_SS);
        unaff_CS = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | puVar2) == 0) {
            uVar6 = 0;
        } else {
            unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
            uVar6 = win_fn_1008_3bd6(
                puVar2,
                struct_a,
                1,
                CONCAT22(local_24, local_22),
                0x1030104,
                CONCAT22((iVar1 + 0x5c64), 0x102),
                CONCAT22(uVar3, (iVar4 + 6)),
            );
        }
        _local_8 = uVar6;
        enable_window_1040_0558(iVar4, uVar5, local_a);
        local_a = local_a + 1;
        struct_a = extraout_DX_00;
    }
    move_window_1040_826c(iVar4, uVar5, 0xffff, 0xffff);
    _local_12 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x48));
    uVar3 = (_local_12 >> 0x10);
    local_c = (_local_12 + 10);
    local_e = (_local_12 + 0xc);
    GetWindowRect16(CONCAT22(&local_1a, 0x1010), unaff_SS);
    local_1c = local_16 - local_1a;
    local_1a = (local_c / 2 - local_1c) - 3;
    if (local_1a < 0) {
        local_1a = 0;
    }
    SetWindowPos16(0x41, 0, 0, local_18, local_1a, 0, (iVar4 + 6));
    puVar7 = pass1_1038_af40(_g_astruct_112_a, *(iVar4 + 6), 0x17);
    (iVar4 + 0x96) = puVar7;
    (iVar4 + 0x98) = (puVar7 >> 0x10);
    uVar3 = mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x9e0001);
    (iVar4 + 0x8c) = uVar3;
    return;
}

pub fn win_fn_1038_ec16(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let struct_a: *mut astruct_199;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut unaff_SI: u16;
    let mut uVar6: u16;
    let unaff_SS: HWND16;
    let mut uVar7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2b));
    g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_0892();
    if (__g_astruct_94_ptr_1 == 0) {
        _g_astruct_94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
    uVar2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_astruct_94_ptr_1,
        g_u16_ptr_1050_5f2e,
    );
    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    (iVar4 + 0x8e) = uVar2;
    (iVar4 + 0x90) = g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        _local_1a = pass1_1010_0932(_local_6, (_local_6 >> 0x10), local_a);
        struct_a = (_local_1a >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        uVar2 = (iVar4 + 6);
        puVar3 = &local_22;
        MapDialogRect16(CONCAT22(puVar3, 0x1010), unaff_SS);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | puVar3) == 0) {
            uVar1 = (iVar4 + 0x8e);
            (uVar1 + local_a * 4) = 0;
        } else {
            uVar7 = win_fn_1008_3bd6(
                puVar3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(uVar2, (iVar4 + 6)),
            );
            uVar1 = (iVar4 + 0x8e);
            uVar2 = (uVar1 >> 0x10);
            iVar5 = uVar1;
            (iVar5 + local_a * 4) = uVar7;
            (iVar5 + local_a * 4 + 2) = (uVar7 >> 0x10);
        }
        uVar1 = (iVar4 + 0x8e);
        uVar2 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        if ((iVar5 + local_a * 4) != 0) {
            uVar1 = (iVar5 + local_a * 4);
            enable_window_1040_9234(uVar1, (uVar1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    move_window_1040_826c(param_1, uVar6, 0xff, 0xffff);
    ShowWindow16(5, (iVar4 + 6));
    return;
}

pub fn show_window_1038_ea18(param_1: *mut astruct_20) {
    let iVar1: u16;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let unaff_SS: HWND16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    _local_6 = pass1_1010_375e((iVar3 + 0x8e));
    local_8 = GetDlgItem16(0xfa5, (iVar3 + 6));
    SendMessage16(_local_6, 0, 0xc, local_8);
    GetWindowRect16(CONCAT22(&local_10, 0x1538), unaff_SS);
    iVar1 = GetSystemMetrics16(4);
    iVar2 = iVar1 + local_e + 5;
    move_window_1040_826c(iVar3, uVar4, iVar2, iVar2);
    ShowWindow16(5, (iVar3 + 6));
    return;
}

pub fn destroy_win_1038_eaa2(param_1: u32, param_2: i32) {
    let h_wnd: HWND16;
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_56: u16;
    let local_54: [HWND16; 41];

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 != 0) {
        h_wnd = GetDlgItem16(0xfa5, (iVar1 + 6));
        SendMessage16(CONCAT22(unaff_SS, local_54), 0x50, 0xd, h_wnd);
        pass1_1010_3770((iVar1 + 0x8e), CONCAT22(unaff_SS, local_54));
        PostMessage16(0, 0xfb, 0x111, (iVar1 + 8));
    }
    local_54[0] = (iVar1 + 6);
    DestroyWindow16(local_54[0]);
    return;
}

pub fn show_destroy_win_1038_e71c(param_1: *mut astruct_20) {
    let mut in_AX: u16;
    let mut extraout_DX: u16;
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    string_fn_1010_2c34((iVar1 + 0x8e));
    _local_6 = CONCAT22(extraout_DX, in_AX);
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (iVar1 + 0x10)),
        CONCAT22(extraout_DX, in_AX),
    );
    error_check_1000_17ce(_local_6);
    move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (iVar1 + 6));
    (iVar1 + 0x92) = 1;
    process_win_msg_1008_9510();
    DestroyWindow16((iVar1 + 6));
    return;
}

pub fn dialog_button_checked_1038_e7a0(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;

    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        uVar1 = (iVar3 + 0x8e);
        (uVar1 + 0x10) = 1;
        uVar1 = (iVar3 + 0x8e);
        (uVar1 + 10) = 0;
        uVar1 = (iVar3 + 0x8e);
        (uVar1 + 0xc) = 0;
        uVar1 = (iVar3 + 0x8e);
        (uVar1 + 0xe) = 0;
    } else {
        uVar2 = IsDlgButtonChecked16(0x1827, (iVar3 + 6));
        if (uVar2 == 0) {
            uVar2 = IsDlgButtonChecked16(0x1828, (iVar3 + 6));
            if (uVar2 == 0) {
                uVar1 = (iVar3 + 0x8e);
                (uVar1 + 10) = 0;
            } else {
                uVar1 = (iVar3 + 0x8e);
                (uVar1 + 10) = 2;
            }
        } else {
            uVar1 = (iVar3 + 0x8e);
            (uVar1 + 10) = 1;
        }
        uVar2 = IsDlgButtonChecked16(s_vrpal_bmp_1050_183a, (iVar3 + 6));
        if (uVar2 == 0) {
            uVar2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 1), (iVar3 + 6));
            if (uVar2 == 0) {
                uVar1 = (iVar3 + 0x8e);
                (uVar1 + 0xc) = 0;
            } else {
                uVar1 = (iVar3 + 0x8e);
                (uVar1 + 0xc) = 2;
            }
        } else {
            uVar1 = (iVar3 + 0x8e);
            (uVar1 + 0xc) = 1;
        }
        uVar2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 2), (iVar3 + 6));
        if (uVar2 == 0) {
            uVar2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 3), (iVar3 + 6));
            if (uVar2 == 0) {
                uVar1 = (iVar3 + 0x8e);
                (uVar1 + 0xe) = 0;
            } else {
                uVar1 = (iVar3 + 0x8e);
                (uVar1 + 0xe) = 2;
            }
        } else {
            uVar1 = (iVar3 + 0x8e);
            (uVar1 + 0xe) = 1;
        }
        uVar1 = (iVar3 + 0x8e);
        (uVar1 + 0x10) = 0;
    }
    (iVar3 + 0x92) = 0;
    return;
}

pub fn win_fn_1038_e348(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let struct_a: *mut astruct_199;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut unaff_SI: u16;
    let mut uVar6: u16;
    let unaff_SS: HWND16;
    let mut uVar7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2b));
    g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_088c();
    if (__g_astruct_94_ptr_1 == 0) {
        _g_astruct_94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
    uVar2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_astruct_94_ptr_1,
        g_u16_ptr_1050_5f2e,
    );
    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    (iVar4 + 0x8e) = uVar2;
    (iVar4 + 0x90) = g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        _local_1a = pass1_1010_091e(_local_6, (_local_6 >> 0x10), local_a);
        struct_a = (_local_1a >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        uVar2 = (iVar4 + 6);
        puVar3 = &local_22;
        MapDialogRect16(CONCAT22(puVar3, 0x1010), unaff_SS);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | puVar3) == 0) {
            uVar1 = (iVar4 + 0x8e);
            (uVar1 + local_a * 4) = 0;
        } else {
            uVar7 = win_fn_1008_3bd6(
                puVar3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(uVar2, (iVar4 + 6)),
            );
            uVar1 = (iVar4 + 0x8e);
            uVar2 = (uVar1 >> 0x10);
            iVar5 = uVar1;
            (iVar5 + local_a * 4) = uVar7;
            (iVar5 + local_a * 4 + 2) = (uVar7 >> 0x10);
        }
        uVar1 = (iVar4 + 0x8e);
        uVar2 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        if ((iVar5 + local_a * 4) != 0) {
            uVar1 = (iVar5 + local_a * 4);
            enable_window_1040_9234(uVar1, (uVar1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    move_window_1040_826c(param_1, uVar6, 0xff, 0xffff);
    ShowWindow16(5, (iVar4 + 6));
    return;
}

pub fn check_dlg_btn_1038_e1dc(in_struct_1: *mut astruct_49, param_2: u16) {
    let mut uVar1: u16;
    let local_BX_7: *mut astruct_49;
    let mut uVar2: u16;
    let mut w_param: u32;

    local_BX_7 = in_struct_1;
    uVar2 = (in_struct_1 >> 0x10);
    if (param_2 != 0) {
        uVar1 = IsDlgButtonChecked16(0x1807, local_BX_7.h_win);
        if (uVar1 == 0) {
            uVar1 = IsDlgButtonChecked16(0x1806, local_BX_7.h_win);
            if (uVar1 == 0) {}
            // goto LAB_1038_e229;
            w_param = 0x1110130;
        } else {
            w_param = 0x111012f;
        }
        SendMessage16(0, w_param, w_param._2_2_, g_h_window);
    }
    // LAB_1038_e229:
    DestroyWindow16(local_BX_7.h_win);
    return;
}

pub fn gui_window_func_1038_e19a(param_1: *mut astruct_20) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let local_6: *mut astruct_24;

    win_gui_func_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    CheckRadioButton16(0x1807, 0x1807, 0x1807, (iVar1 + 6));
    move_window_1040_826c(iVar1, uVar2, 200, 200);
    ShowWindow16(5, (iVar1 + 6));
    return;
}

pub fn pass1_1038_e03e(param_1: *mut u8) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    uVar2 = return_10_1010_0886();
    local_6 = 1;
    while (local_6 <= uVar2) {
        uVar1 = (param_1 + 0x92);
        uVar6 = pass1_1010_08e2(uVar1, (uVar1 >> 0x10), local_6);
        uVar1 = (param_1 + 0x96);
        uVar5 = (uVar1 >> 0x10);
        iVar3 = uVar1;
        if ((iVar3 + local_6 * 4) != 0) {
            uVar1 = (iVar3 + local_6 * 4);
            enable_window_1040_9234(uVar1, (uVar1 >> 0x10), (uVar6 + 6));
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn win_fn_1038_d8ae(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let extraout_DX: *mut u16;
    let struct_a: *mut astruct_199;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let unaff_SS: HWND16;
    let mut uVar7: u32;
    let mut in_stack_0000ffd0: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    win_gui_func_1040_78e2(param_1);
    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    g_u16_ptr_1050_5f2e = extraout_DX;
    local_8 = return_10_1010_0886();
    if (__g_astruct_94_ptr_1 == 0) {
        _g_astruct_94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1e = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
    uVar2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_astruct_94_ptr_1,
        g_u16_ptr_1050_5f2e,
    );
    (iVar4 + 0x96) = uVar2;
    (iVar4 + 0x98) = g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        uVar1 = (iVar4 + 0x92);
        _local_1e = pass1_1010_08e2(uVar1, (uVar1 >> 0x10), local_a);
        struct_a = (_local_1e >> 0x10);
        local_26 = *_local_1e;
        local_24 = (_local_1e + 2);
        local_22 = 1;
        local_20 = 1;
        uVar2 = (iVar4 + 6);
        puVar3 = &local_26;
        MapDialogRect16(CONCAT22(puVar3, 0x1010), unaff_SS);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | puVar3) == 0) {
            uVar1 = (iVar4 + 0x96);
            (uVar1 + local_a * 4) = 0;
        } else {
            uVar7 = win_fn_1008_3bd6(
                puVar3,
                struct_a,
                0,
                CONCAT22(local_26, local_24),
                0x1000101,
                CONCAT22((_local_1e + 4), 0xff),
                CONCAT22(uVar2, (iVar4 + 6)),
            );
            uVar1 = (iVar4 + 0x96);
            uVar2 = (uVar1 >> 0x10);
            iVar5 = uVar1;
            (iVar5 + local_a * 4) = uVar7;
            (iVar5 + local_a * 4 + 2) = (uVar7 >> 0x10);
        }
        uVar1 = (iVar4 + 0x96);
        uVar2 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        if ((iVar5 + local_a * 4) != 0) {
            uVar1 = (iVar5 + local_a * 4);
            (uVar1 + 0x3e) = 1;
            uVar1 = (iVar4 + 0x96);
            uVar1 = (uVar1 + local_a * 4);
            enable_window_1040_9234(uVar1, (uVar1 >> 0x10), (_local_1e + 6));
        }
        local_a = local_a + 1;
    }
    _local_e = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffd0, 2));
    SetWindowText16((_local_e + 0x68), (iVar4 + 6));
    ShowWindow16(5, (iVar4 + 6));
    SetCursor16(local_6);
    return;
}

pub fn win_fn_1038_d400(param_1: i32, param_2: u16_00, param_3: u16, param_2: u32) {
    let HVar1: HWND16;
    let BVar2: bool;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut unaff_SS: u16;
    let ppVar3: *mut pass1_struct_1;
    let WVar4: WPARAM16;
    let mut uVar5: u16;
    let iVar6: u16;
    let mut uVar7: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    local_8 = 0;
    match (param_2._2_2_) {
        0x145 => {
            HVar1 = GetDlgItem16(0x146, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x13f0647;
            uVar7 = 0x1f1;
            // goto LAB_1038_d490;
        }
        0x146 => {
            local_6 = 0x1400648;
            pass1_1008_941a(CONCAT22(unaff_SS, local_c), 1, 0xc4);
            mci_send_command_1008_5c9e(_g_struct_ptr_1050_02a0, CONCAT22(unaff_SS, local_c));
            uVar7 = 0x86;
            ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x860009);
            pass1_1010_6604(ppVar3, uVar7);
            HVar1 = GetDlgItem16(0x145, (param_1 + 6));
            BVar2 = EnableWindow16(0, HVar1);
            HVar1 = (param_1 + 6);
            uVar5 = 0xc;
            iVar6 = 0x13f;
            WVar4 = 0;
            load_string_1010_847e(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x649,
            );
            SendDlgItemMessage16(CONCAT22(extraout_DX_00, BVar2), WVar4, uVar5, iVar6, HVar1);
            HVar1 = GetDlgItem16(0x146, (param_1 + 6));
            BVar2 = EnableWindow16(0, HVar1);
            pass1_1010_659a(ppVar3, 0x86);
            if (BVar2 == 0) {
                HVar1 = GetDlgItem16(0x14a, (param_1 + 6));
                BVar2 = EnableWindow16(0, HVar1);
                HVar1 = (param_1 + 6);
                uVar5 = 0xc;
                iVar6 = 0x144;
                WVar4 = 0;
                load_string_1010_847e(
                    _g_struct_73_1050_14cc,
                    (_g_struct_73_1050_14cc >> 0x10),
                    0x531,
                );
                SendDlgItemMessage16(CONCAT22(extraout_DX_01, BVar2), WVar4, uVar5, iVar6, HVar1);
            }
            ppVar3 =
                process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe6, 2));
            if ((ppVar3 + 0x20) != 0) {
                PostMessage16(0, 0xaf, 0x111, g_h_window);
            }
        }
        0x147 => {
            HVar1 = GetDlgItem16(0x148, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x1410647;
            uVar7 = 0x1f5;
            // goto LAB_1038_d490;
        }
        0x148 => {
            HVar1 = GetDlgItem16(0x149, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x1420647;
            uVar7 = 0x1f2;
            // LAB_1038_d490:
            mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, uVar7)
        }
        0x149 => {
            local_6 = 0x1430648;
            PostMessage16(0, 0xb8, 0x111, g_h_window);
            DestroyWindow16((param_1 + 6))
        }
        0x14a => {
            HVar1 = GetDlgItem16(0x145, (param_1 + 6));
            BVar2 = EnableWindow16(1, HVar1);
            HVar1 = (param_1 + 6);
            uVar5 = 0xc;
            iVar6 = 0x140;
            WVar4 = 0;
            load_string_1010_847e(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x649,
            );
            SendDlgItemMessage16(CONCAT22(extraout_DX, BVar2), WVar4, uVar5, iVar6, HVar1)
        }
        0x14b => {
            HVar1 = GetDlgItem16(0x147, (param_1 + 6));
            EnableWindow16(1, HVar1)
        }
        _ => {
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2._2_2_);
            return;
        }
    }
    if (((local_6._2_2_ != 0) && (local_6 != 0)) && (BVar2 = IsWindow16((param_1 + 6)), BVar2 != 0))
    {
        HVar1 = (param_1 + 6);
        WVar4 = 0;
        uVar5 = 0xc;
        load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            local_6,
        );
        SendDlgItemMessage16(
            CONCAT22(extraout_DX_02, BVar2),
            WVar4,
            uVar5,
            local_6._2_2_,
            HVar1,
        );
    }
    if (local_8 != 0) {
        PostMessage16(0, local_8, 0x111, g_h_window);
    }
    return;
}

pub fn win_fn_1038_d2a2(param_1: *mut astruct_20) {
    let l_param: LPARAM;
    let puVar1: *mut u16;
    let HVar2: HWND16;
    let extraout_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let unaff_SS: HWND16;
    let w_param: WPARAM16;
    let mut msg: u16;
    let id: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = SUB42(&PTR_LOOP_1050_1040, 0);
    win_gui_func_1040_78e2(param_1);
    local_4 = 7;
    local_a = 0;
    struct_a = extraout_DX;
    while (
        iVar4 = param_1,
        uVar5 = (param_1 >> 0x10),
        local_a < local_4,
    ) {
        iVar3 = local_a * 0xc;
        local_16 = (iVar3 + 0x5c0c);
        local_14 = (iVar3 + 0x5c0e);
        local_12 = 1;
        local_10 = 1;
        uVar7 = (iVar4 + 6);
        puVar1 = &local_16;
        MapDialogRect16(CONCAT22(puVar1, uVar6), unaff_SS);
        uVar6 = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | puVar1) == 0) {
            _local_8 = 0;
        } else {
            uVar6 = SUB42(&PTR_LOOP_1050_1008, 0);
            _local_8 = win_fn_1008_3bd6(
                puVar1,
                struct_a,
                1,
                CONCAT22(local_16, local_14),
                0x1030104,
                CONCAT22((iVar3 + 0x5c10), 0x102),
                CONCAT22(uVar7, (iVar4 + 6)),
            );
        }
        struct_a = (_local_8 >> 0x10);
        if ((local_a * 0xc + 0x5c12) == 0) {
            uVar6 = SUB42(offset, 0);
            EnableWindow16(0, (_local_8 + 0x18));
        }
        local_a = local_a + 1;
    }
    uVar8 = 0x86;
    _local_e = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x860009);
    iVar3 = pass1_1010_659a(_local_e, uVar8);
    if (iVar3 == 0) {
        HVar2 = GetDlgItem16(0x14a, (iVar4 + 6));
        EnableWindow16(0, HVar2);
        HVar2 = (iVar4 + 6);
        msg = 0xc;
        id = 0x144;
        w_param = 0;
        l_param = load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        SendDlgItemMessage16(l_param, w_param, msg, id, HVar2);
    }
    move_window_1040_826c(iVar4, uVar5, 0xffff, 0xffff);
    ShowWindow16(5, (iVar4 + 6));
    uVar6 = mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x9a0001);
    (iVar4 + 0x8c) = uVar6;
    return;
}

pub fn dialog_button_checked_1038_cdd6(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let local_BX_8: *mut astruct_70;
    let mut uvar3: u16;

    local_BX_8 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 0) {
        uVar1 = local_BX_8.field_0x8e;
        (uVar1 + 10) = 0;
    } else {
        uVar2 = IsDlgButtonChecked16(0x182e, local_BX_8.field_0x6);
        if (uVar2 == 0) {
            uVar2 = IsDlgButtonChecked16(0x182f, local_BX_8.field_0x6);
            if (uVar2 == 0) {
                uVar2 = IsDlgButtonChecked16(0x1829, local_BX_8.field_0x6);
                if (uVar2 == 0) {
                    uVar2 = IsDlgButtonChecked16(0x182a, local_BX_8.field_0x6);
                    if (uVar2 == 0) {
                        uVar2 = IsDlgButtonChecked16(0x182c, local_BX_8.field_0x6);
                        if (uVar2 == 0) {
                            uVar2 = IsDlgButtonChecked16(0x182d, local_BX_8.field_0x6);
                            if (uVar2 != 0) {
                                uVar1 = local_BX_8.field_0x8e;
                                (uVar1 + 10) = 7;
                            }
                        } else {
                            uVar1 = local_BX_8.field_0x8e;
                            (uVar1 + 10) = 6;
                        }
                    } else {
                        uVar1 = local_BX_8.field_0x8e;
                        (uVar1 + 10) = 4;
                    }
                } else {
                    uVar1 = local_BX_8.field_0x8e;
                    (uVar1 + 10) = 3;
                }
            } else {
                uVar1 = local_BX_8.field_0x8e;
                (uVar1 + 10) = 2;
            }
        } else {
            uVar1 = local_BX_8.field_0x8e;
            (uVar1 + 10) = 1;
        }
    }
    local_BX_8.field_0x92 = 0;
    return;
}

pub fn win_func_1038_cd88(param_1: *mut astruct_20) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    ShowWindow16(5, (iVar1 + 6));
    (iVar1 + 0x92) = 1;
    process_win_msg_1008_9510();
    DestroyWindow16((iVar1 + 6));
    return;
}

pub fn show_window_1038_cb5c(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let struct_a: *mut astruct_199;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let puVar5: *mut u16;
    let puVar6: *mut u16;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar2 = ret_5_1008_eb6e();
    local_a = 0;
    while (local_a < uVar2) {
        uVar1 = (iVar3 + 0x8e);
        puVar5 = pass1_1008_eb5c(uVar1, (uVar1 >> 0x10), local_a);
        struct_a = (puVar5 >> 0x10);
        puVar6 = puVar5;
        process_struct_1000_179c(0x42, struct_a);
        if (puVar6 != 0x0) {
            win_fn_1008_3bd6(
                puVar6,
                (puVar6 >> 0x10),
                0,
                unsafe { CONCAT22(*puVar5, (puVar5 + 2)) },
                0x1000101,
                CONCAT22((puVar5 + 4), 0xff),
                CONCAT22(in_stack_0000fff2, (iVar3 + 6)),
            );
        }
        local_a = local_a + 1;
    }
    mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x90001);
    ShowWindow16(5, (iVar3 + 6));
    return;
}

pub fn destroy_win_1038_c95e(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;

    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        uVar1 = (iVar3 + 0x8e);
        (uVar1 + 10) = 0;
    } else {
        uVar2 = IsDlgButtonChecked16(0xfac, (iVar3 + 6));
        if (uVar2 == 0) {
            uVar2 = IsDlgButtonChecked16(0xfad, (iVar3 + 6));
            if (uVar2 == 0) {
                uVar2 = IsDlgButtonChecked16(0xfae, (iVar3 + 6));
                if (uVar2 == 0) {
                    uVar2 = IsDlgButtonChecked16(0xfaf, (iVar3 + 6));
                    if (uVar2 == 0) {
                        uVar2 = IsDlgButtonChecked16(0xfb0, (iVar3 + 6));
                        if (uVar2 != 0) {
                            uVar1 = (iVar3 + 0x8e);
                            (uVar1 + 10) = 5;
                        }
                    } else {
                        uVar1 = (iVar3 + 0x8e);
                        (uVar1 + 10) = 4;
                    }
                } else {
                    uVar1 = (iVar3 + 0x8e);
                    (uVar1 + 10) = 3;
                }
            } else {
                uVar1 = (iVar3 + 0x8e);
                (uVar1 + 10) = 2;
            }
        } else {
            uVar1 = (iVar3 + 0x8e);
            (uVar1 + 10) = 1;
        }
    }
    DestroyWindow16((iVar3 + 6));
    PTR_LOOP_1050_5b80 = 0x0;
    return;
}

pub fn gui_window_func_1038_c89c(param_1: *mut astruct_22) {
    let mut u_var1: u32;
    let h_wnd: HWND16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let local_10: *mut astruct_22;
    let mut h_wnd_2: u16;
    let temp_5f4a343e5a: *mut astruct_23;

    win_gui_func_1040_78e2(param_1);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    CheckRadioButton16(0xfac, 0xfad, 0xfac, (iVar2 + 6));
    uVar1 = (iVar2 + 0x8e);
    (uVar1 + 10) = 1;
    uVar1 = (iVar2 + 0x8e);
    temp_5f4a343e5a = (uVar1 + 0x12);
    if (temp_5f4a343e5a == &PTR_DAT_0005_0000_1050_0004) {
        // LAB_1038_c8da:
        h_wnd = GetDlgItem16(0xfce, (iVar2 + 6));
        if (h_wnd != 0) {
            EnableWindow16(1, h_wnd);
        }
        h_wnd_2 = GetDlgItem16(1, (iVar2 + 6));
        if (h_wnd_2 == 0) {}
        // goto LAB_1038_c93c;
        local_10 = 0x0;
    } else {
        if (((temp_5f4a343e5a + -5) < 1) || (SBORROW2((temp_5f4a343e5a + -5), 1))) {}
        // goto LAB_1038_c93c;
        if (temp_5f4a343e5a != &BYTE_1050_0008 && 0 < (temp_5f4a343e5a + -7)) {
            if (temp_5f4a343e5a != &BYTE_1050_0009) {}
            // goto LAB_1038_c93c;
            // goto LAB_1038_c8da;
        }
        h_wnd_2 = GetDlgItem16(0xfce, (iVar2 + 6));
        if (h_wnd_2 == 0) {}
        // goto LAB_1038_c93c;
        local_10 = (&PTR_LOOP_1050_0000 + 1);
    }
    EnableWindow16(local_10, h_wnd_2);
    // LAB_1038_c93c:
    move_window_1040_826c(param_1, 200, 0);
    ShowWindow16(5, (iVar2 + 6));
    return;
}

pub fn win_fn_1038_c672(param_1: i32, param_2: u16, param_3: u16_00, param_3: u32) {
    let mut unaff_SS: u16;
    let mut b: u16;
    let mut local_404: [u8; 1026];

    b = (_g_struct_73_1050_14cc >> 0x10);
    if (param_3._2_2_ == 0x17d) {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_SS, local_404),
            0x453,
        );
        MessageBox16(
            0x30,
            (param_1 + 0x92),
            CONCAT22(unaff_SS, local_404),
            (param_1 + 6),
        );
    } else {
        if (param_3._2_2_ != 0x17e) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_SS, local_404),
            0x454,
        );
        MessageBox16(
            0x30,
            (param_1 + 0x92),
            CONCAT22(unaff_SS, local_404),
            (param_1 + 6),
        );
        pass1_1008_e164((param_1 + 0x8e));
    }
    PostMessage16(0, 2, 0x111, (param_1 + 6));
    return;
}

pub fn win_fn_1038_c58e(param_1: u32) {
    let mut iVar1: i32;
    let mut unaff_SI: u16;
    let mut uVar2: i32;
    let mut unaff_SS: u16;
    let mut local_816: u16;
    let mut local_814: u16;
    let mut local_80e: i32;
    let mut uStack2060: u16;
    let mut uStack2058: u16;
    let mut uStack2052: u16;
    let HStack2050: HWND16;
    let mut local_40c: [u8; 1026];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    local_a = (_local_6 + 0x68);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_SS, local_40c), (iVar1 + 6));
    wsprintf16(
        &local_80e,
        CONCAT22(local_40c, unaff_SS),
        CONCAT22(local_a, unaff_SS),
    );
    SetWindowText16(CONCAT22(unaff_SS, &local_80e), (iVar1 + 6));
    local_80e = uVar2;
    pass1_1008_e038(
        (iVar1 + 0x8e) & 0xffff | (iVar1 + 0x96) << 0x10,
        param_1 & 0xffff0000 | uVar2,
        param_1 & 0xffff0000 | (iVar1 + 0x96),
    );
    local_80e = 0x7d6;
    local_816 = (_g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        local_816,
        0x400,
        CONCAT22(unaff_SS, &local_80e),
        0x7d6,
    );
    local_80e = (*(iVar1 + 0x92) >> 0x10);
    local_814 = *CONCAT22(0x400, local_816);
    wsprintf16(
        local_40c,
        CONCAT22(&local_80e, unaff_SS),
        CONCAT22(local_814, unaff_SS),
    );
    HStack2050 = (iVar1 + 6);
    uStack2052 = 0x17f;
    uStack2058 = SUB42(offset, 0);
    uStack2060 = 0xc66c;
    SetDlgItemText16(CONCAT22(unaff_SS, local_40c), 0x17f, HStack2050);
    return;
}

pub fn set_focus_1038_c558(param_1: *mut astruct_20) {
    let mut uVar1: u16;

    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

// pub fn set_focus_1038_c558(param_1: *mut astruct_20)

// {
//     let mut uVar1: u16;

//     win_gui_func_1040_78e2(param_1);
//     move_window_1040_826c(param_1, 0xffffffff);
//     uVar1 = (param_1 >> 0x10);
//     ShowWindow16(5, (param_1 + 6));
//     SetFocus16((param_1 + 6));
//     return;
// }

pub fn enable_window_1038_c294(param_1: u32) {
    let lp_string: SEGPTR;
    let mut uVar1: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    _local_c = param_1 & 0xffff0000 | (param_1 + 0x9e);
    lp_string = pass1_1008_e320(
        (param_1 + 0x8e),
        param_1 & 0xffff0000 | (param_1 + 0x19e),
        param_1 & 0xffff0000 | (param_1 + 0x9e),
    );
    SetWindowText16(lp_string, (param_1 + 0x9a));
    uVar1 = string_fn_1008_e2a4(
        (param_1 + 0x8e),
        param_1 & 0xffff0000 | (param_1 + 0x19e),
        _local_c,
    );
    EnableWindow16(uVar1 & 1, (param_1 + 0x96));
    EnableWindow16(uVar1 & 2, (param_1 + 0x98));
    return;
}

pub fn set_window_pos_1038_c31a(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_3;
    local_6 = param_2;
    if (param_3 == 1) {
        enable_window_1038_c294(param_1);
    } else {
        if (param_3 != 7) {
            return 0;
        }
        GetWindowRect16(CONCAT22(&local_e, unaff_CS), unaff_SS);
        local_a = local_a - local_e;
        SetWindowPos16(2, 0x50, local_a, 0, 0, 0, local_6);
    }
    return 1;
}

pub fn set_focus_1038_c044(param_1: *mut astruct_20) {
    let mut uVar1: u16;

    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1038_c07a(param_1: i32, param_2: u16_00, param_3: u16, param_2: u32) {
    let mut iVar1: i32;
    let unaff_SS: *mut char;
    let uVar2: u8;
    let mut local_70c: [u8; 512];
    let mut local_50c: [u8; 256];
    let mut local_40c: [u8; 1026];
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    send_win_msg_1038_c228(param_1, param_2_00);
    _local_6 = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    uVar2 = (unaff_SS >> 8);
    if (param_2._2_2_ == 0x177) {
        pass1_1008_e05e(
            (param_1 + 0x8e),
            2,
            CONCAT22(param_2_00, (param_1 + 0x19e)),
            CONCAT22(param_2_00, param_1 + 0x9e),
        );
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x200,
            CONCAT22(unaff_SS, local_40c),
            0x451,
        );
        string_fn_1000_3f9c(local_70c, unaff_SS, local_40c, unaff_SS, (param_1 + 0x19e));
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_SS, local_50c),
            0x57b,
        );
        MessageBox16(
            0x30,
            CONCAT13(uVar2, CONCAT12(unaff_SS, local_50c)),
            CONCAT22(unaff_SS, local_70c),
            (param_1 + 6),
        );
    } else {
        if (param_2._2_2_ != 0x178) {
            if ((param_2._2_2_ != 0x178) && (param_2._2_2_ - 0x179 < 2)) {
                set_window_pos_1038_c31a(param_1, param_2_00, param_3, param_2, param_2._2_2_);
                return;
            }
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2._2_2_);
            return;
        }
        _local_a = CONCAT22(param_2_00, param_1 + 0x9e);
        iVar1 = pass1_1008_e10c(
            (param_1 + 0x8e),
            CONCAT22(param_2_00, param_1 + 0x19e),
            CONCAT22(param_2_00, param_1 + 0x9e),
        );
        if (iVar1 == 0) {
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_SS, local_40c),
                0x450,
            );
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_SS, local_50c),
                0x57b,
            );
            MessageBox16(
                0x30,
                CONCAT13(uVar2, CONCAT12(unaff_SS, local_50c)),
                CONCAT22(unaff_SS, local_40c),
                (param_1 + 6),
            );
            return;
        }
        pass1_1008_e01c(
            (param_1 + 0x8e),
            CONCAT22(param_2_00, param_1 + 0x19e),
            _local_a,
        );
        pass1_1038_af40(_g_astruct_112_a, *(param_1 + 8), 0x1f);
    }
    PostMessage16(0, 2, 0x111, (param_1 + 6));
    return;
}

pub fn win_fn_1038_bea4(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut extraout_DX: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut l_param: u32;
    let LVar6: LRESULT;
    let in_stack_0000fed2: u8;
    let in_stack_0000fed3: u8;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u32;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: [u8; 130];
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT13(in_stack_0000fed3, CONCAT12(in_stack_0000fed2, 2)),
    );
    local_a = (_local_6 + 0x68);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_SS, local_8c), (iVar3 + 6));
    wsprintf16(
        local_10e,
        CONCAT22(local_8c, unaff_SS),
        CONCAT22(local_a, unaff_SS),
    );
    SetWindowText16(CONCAT22(unaff_SS, local_10e), (iVar3 + 6));
    HVar2 = GetDlgItem16(0x179, (iVar3 + 6));
    (iVar3 + 0x92) = HVar2;
    process_struct_1008_e3ec(
        (iVar3 + 0x8e),
        CONCAT22(unaff_SS, &local_116),
        CONCAT22(unaff_SS, &local_112),
    );
    local_126 = local_112;
    local_124 = (local_112 >> 0x10);
    win_fn_1038_c374(
        iVar3,
        (param_1 >> 0x10),
        local_126,
        local_124,
        (iVar3 + 0x92),
    );
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_120, 0x2f));
    uVar1 = (iVar3 + 0x8e);
    local_126 = (uVar1 >> 0x10);
    l_param = process_struct_1008_e586(uVar1, local_126, (ppVar5 + 0x24));
    SendMessage16(l_param, 0xffff, 0x40d, (iVar3 + 0x92));
    HVar2 = GetDlgItem16(0x17a, (iVar3 + 6));
    (iVar3 + 0x94) = HVar2;
    local_124 = local_116;
    local_122 = (local_116 >> 0x10);
    win_fn_1038_c374(param_1, uVar4, local_124, local_122, HVar2);
    local_124 = _g_struct_73_1050_14cc;
    local_122 = (_g_struct_73_1050_14cc >> 0x10);
    load_string_1010_847e(local_124, local_122, 0x531);
    LVar6 = SendMessage16(CONCAT22(extraout_DX, HVar2), 0, 0x403, (iVar3 + 0x94));
    (iVar3 + 0x9c) = LVar6;
    SendMessage16(extraout_DX, 0xffff, 0x40d, (iVar3 + 0x94));
    HVar2 = GetDlgItem16(0x178, (iVar3 + 6));
    (iVar3 + 0x96) = HVar2;
    HVar2 = GetDlgItem16(0x177, (iVar3 + 6));
    (iVar3 + 0x98) = HVar2;
    HVar2 = GetDlgItem16(0x184, (iVar3 + 6));
    (iVar3 + 0x9a) = HVar2;
    return;
}

pub fn set_cursor_1038_bc30(param_1: u32) {
    let mut u_var1: u32;
    let mut unaff_SS: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    uVar1 = (param_1 + 0x8e);
    pass1_1030_532e(CONCAT22(unaff_SS, &local_112), (uVar1 + 0xe) + 0x1000000);
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_112));
    pass1_1030_838e(_g_bool_1050_5748);
    local_112 = s_1_1050_389a;
    local_110 = &PTR_LOOP_1050_1008;
    pass1_1030_8334();
    SetCursor16(local_6);
    return;
}

pub fn gui_dialog_1038_b81c(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut uVar4: i32;
    let mut h_dialog: u16;
    let mut b_result: u16;
    let hwnd: HWND16;
    let mut extraout_DX: u16;
    let mut iVar6: i32;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let ppVar8: *mut pass1_struct_1;
    let local_1e: *mut astruct_19;
    let mut uStack28: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut check_id: u16;
    let mut local_a: u16;
    let mut local_6: u32;
    let puVar5: *mut u16;

    win_gui_func_1040_78e2(param_1);
    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 6));
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    (iVar6 + 0x92) = ppVar8;
    (iVar6 + 0x94) = (ppVar8 >> 0x10);
    uVar1 = (iVar6 + 0x92);
    uVar4 = uVar1 + 0x4e;
    uVar1 = uVar1 & 0xffff0000;
    puVar5 = (uVar1 | uVar4);
    local_a = 0;
    check_id = 0x1a0;
    while (check_id < 0x1b5) {
        if ((local_a * 2 + uVar4) == check_id) {
            local_a = local_a + 1;
        } else {
            CheckDlgButton16(2, check_id, (iVar6 + 6));
        }
        check_id = check_id + 1;
    }
    h_dialog = GetDlgItem16(0xfb1, (iVar6 + 6));
    b_result = EnableWindow16(0, h_dialog);
    uVar2 = (iVar6 + 0x92);
    local_1e = uVar2;
    uStack28 = (uVar2 >> 0x10);
    ppcVar3 = ((iVar6 + 0x92) + 0x10);
    (**ppcVar3)(offset, local_1e, uStack28);
    _local_10 = CONCAT22(extraout_DX, b_result);
    move_window_1040_826c(
        iVar6,
        uVar7,
        (b_result + 2) + -2,
        (b_result + 4) + *_local_10 + 3,
    );
    ShowWindow16(5, (iVar6 + 6));
    unsafe {
        pass1_1018_1c9a(*(iVar6 + 0x92), *puVar5);
    }
    unsafe {
        hwnd = GetDlgItem16(*puVar5, (iVar6 + 6));
    }
    SetFocus16(hwnd);
    return;
}

pub fn gui_window_func_1038_b72e(param_1: u32, param_2: i32) -> u16 {
    let hwnd: HWND16;
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_2 * 4 + param_1) != 0) {
        uVar1 = (param_2 * 4 + param_1);
        hwnd = (uVar1 + 6);
        SetFocus16(hwnd);
        BringWindowToTop16(hwnd);
        return 1;
    }
    return 0;
}

pub fn show_window_1038_b634(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xac) == 0) {
        (iVar2 + 0xac) = 1;
        local_4 = 1;
        while (local_4 < 0x2b) {
            if (((local_4 * 4 + iVar2 + 2) | (local_4 * 4 + iVar2)) != 0) {
                uVar1 = (local_4 * 4 + iVar2);
                ShowWindow16(0, (uVar1 + 6));
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn show_win_1038_b68a(param_1: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xac) != 0) {
        (iVar2 + 0xac) = 0;
        local_4 = 1;
        while (local_4 < 0x2b) {
            if (((local_4 * 4 + iVar2 + 2) | (local_4 * 4 + iVar2)) != 0) {
                uVar1 = (local_4 * 4 + iVar2);
                ShowWindow16(1, (uVar1 + 6));
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn pass1_1038_af34() {
    _g_astruct_112_a = 0;
    return;
}

pub fn pass1_1038_af40(param_1: *mut astruct_112, param_2: *mut u8, param_3: u16) -> *mut u16 {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let uVar3: u8;
    let extraout_AH: u8;
    let struct_a: *mut astruct_199;
    let mut uVar4: i32;
    let mut in_BX: i32;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let paVar7: *mut astruct_68;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut local_8: u16;

    iVar8 = param_1;
    uVar9 = (param_1 >> 0x10);
    uVar6 = gui_window_func_1038_b72e(param_1, param_3);
    struct_a = (uVar6 >> 0x10);
    if (uVar6 != 0x0) {}
    // goto LAB_1038_b61f;
    uVar5 = SUB42(&PTR_LOOP_1050_1038, 0);
    PTR_LOOP_1050_5b82 = uVar6;
    match (param_3) {
        1 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {
                // LAB_1038_afa0:
                uVar5 = 0x1000;
                paVar7 = 0x0;
            } else {
                paVar7 = pass1_1038_9f76((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            }
        }
        2 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_181c((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        3 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_e99a((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2)
        }
        4 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_c7b8((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2)
        }
        5 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_23ea((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        6 => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_06e8((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2)
        }
        7 => {
            process_struct_1000_179c(0x9c, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_4068((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        8 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_b772(in_BX, uVar4, 0, 0, 0, 0, param_2)
        }
        9 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_e140((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2)
        }
        10 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_a33c((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        0xb => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_a494((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        0xc => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_a69a((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        0xd => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_a89e((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        0xe => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x94, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_e69a((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0xf => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x94, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_cd06((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x10 => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_0bfc((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2)
        }
        0x11 => {
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_0e1c(
                (uVar6 & 0xff000000 | CONCAT12((uVar6 >> 0x10), in_BX)),
                0x0,
                0x0,
                param_2,
            );
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x12 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_d756(in_BX, uVar4, param_2)
        }
        0x13 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar3 = pass1_1038_cad8(uVar6 & 0xffff0000 | in_BX, param_2);
            paVar7 = CONCAT22(uVar4, CONCAT11(extraout_AH, uVar3))
        }
        0x14 => {
            process_struct_1000_179c(0xaa, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_1f5a(in_BX, uVar4, param_2)
        }
        0x15 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_d242(in_BX, uVar4, param_2)
        }
        0x16 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_eeda(in_BX, uVar4, param_2)
        }
        0x17 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = 0x1018;
            paVar7 = pass1_1018_5e26((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        // default:
        // goto switchD_1038_b581_caseD_18;
        0x19 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_1cb4((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x1a => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_123e((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2)
        }
        0x1b => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_ab82(in_BX, uVar4, param_2)
        }
        0x1c => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_e2d0(in_BX, uVar4, param_2)
        }
        0x1d => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_eb9e(in_BX, uVar4, param_2)
        }
        0x1e => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x29e, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_bddc((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x1f => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_c4a2((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x20 => {
            process_struct_1000_179c(0x29a, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_2ea2((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x21 => {
            process_struct_1000_179c(0xa6, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_3966((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x22 => {
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_34a2((uVar6 & 0xffff0000 | in_BX), 0, 0x0, 0x0, param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x23 => {
            process_struct_1000_179c(0x9c, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_ac84((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        0x25 => {
            process_struct_1000_179c(0xa0, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_ca16((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        0x26 => {
            process_struct_1000_179c(0xa2, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_d0f8((uVar6 & 0xffff0000 | in_BX), param_2)
        }
        0x27 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0xa0, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            uVar4 = (uVar6 >> 0x10) | in_BX;
            if (uVar4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_88f2((uVar6 & 0xffff0000 | in_BX), param_2);
            paVar7 = CONCAT22(uVar4, in_BX)
        }
        0x28 => {
            process_struct_1000_179c(0x96, struct_a);
            uVar4 = (uVar6 >> 0x10);
            PTR_LOOP_1050_5b82 = uVar6;
            if ((uVar4 | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            uVar5 = SUB42(&PTR_LOOP_1050_1040, 0);
            paVar7 = pass1_1040_6402(in_BX, uVar4, param_2)
        }
        0x29 => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x98, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_7d10(uVar6 & 0xffff0000 | in_BX, param_2)
        }
        0x2a => {
            uVar5 = 0x1000;
            process_struct_1000_179c(0x98, struct_a);
            PTR_LOOP_1050_5b82 = uVar6;
            if (((uVar6 >> 0x10) | in_BX) == 0) {}
            // goto LAB_1038_afa0;
            paVar7 = pass1_1038_8caa((uVar6 & 0xffff0000 | in_BX), param_2);
        }
    }
    (param_3 * 4 + iVar8) = paVar7;
    (param_3 * 4 + iVar8 + 2) = (paVar7 >> 0x10);
    // switchD_1038_b581_caseD_18:
    if ((param_3 * 4 + iVar8) != 0) {
        if ((iVar8 + 0xae) != 0) {
            uVar2 = (param_3 * 4 + iVar8);
            (uVar2 + 0x6e) = (iVar8 + 0xae);
        }
        (iVar8 + 0xae) = 0;
        uVar2 = (param_3 * 4 + iVar8);
        ppcVar1 = ((param_3 * 4 + iVar8) + 8);
        (**ppcVar1)(uVar5, uVar2, (uVar2 >> 0x10));
    }
    // LAB_1038_b61f:
    return CONCAT22((param_3 * 4 + iVar8 + 2), (param_3 * 4 + iVar8));
}

pub fn pass1_1038_aeca(param_1: *mut astruct_65) -> *mut astruct_65 {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut local_b6: u16;
    let mut local_b4: u16;
    let mut local_5c: [u8; 90];

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0xac) = 0;
    (param_1 + 0xae) = 0;
    if (_g_astruct_112_a == 0x0) {
        _g_astruct_112_a = param_1;
    }
    pass1_1000_4906(param_1, 0, 0xac);
    load_cursor_1008_80ee(local_5c, unaff_SS);
    load_cursor_1040_9854(&local_b6, unaff_SS);
    local_b6 = s_1_1050_389a;
    local_b4 = &PTR_LOOP_1050_1008;
    modify_list_1008_8168(CONCAT22(unaff_SS, local_5c));
    return param_1;
}

pub fn pass1_1038_aeaa(param_1: *mut astruct_44) -> *mut astruct_44 {
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut cVar3: u8;
    let mut bVar4: u8;
    let mut in_CL: u8;
    let mut in_DX: i32;
    let mut bVar5: u8;
    let mut in_BX: i32;
    let mut bVar6: u8;
    let puVar7: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar8: bool;
    let mut bVar9: bool;
    let mut uStack00aa: u16;
    let mut uStack00ac: u16;
    let paVar10: *mut astruct_65;
    let mut bStack78: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar7 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar7 = puVar7 + -1;
        unsafe {
            *puVar7 = *unaff_BP;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar6 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar6;
    bVar5 = ((in_DX + 1) >> 8);
    bVar8 = CARRY1(bVar5, bVar6) || CARRY1(bVar5 + bVar6, in_CF);
    bVar4 = (in_DX + 1);
    paVar10 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar1 = unaff_SI + in_BX;
    unsafe {
        bVar5 = *pbVar1;
        bVar2 = *pbVar1 - bVar4;
        bVar9 = *pbVar1 < bVar4 || bVar2 < bVar8;
        *pbVar1 = bVar2 - bVar8;
        if (*pbVar1 != 0 && (SBORROW1(bVar5, bVar4) != SBORROW1(bVar2, bVar8)) == (*pbVar1 < '\0'))
        {
            pbVar1 = unaff_SI;
            bVar8 = CARRY1(*pbVar1, bVar6) || CARRY1(*pbVar1 + bVar6, bVar9);
            *pbVar1 = *pbVar1 + bVar6 + bVar9;
            bVar5 = bStack78 + in_BX;
            bVar9 = CARRY1(bStack78, in_BX) || CARRY1(bVar5, bVar8);
            bStack78 = bVar5 + bVar8;
            pbVar1 = unaff_SI + -0x4f;
            bVar5 = *pbVar1;
            bVar2 = *pbVar1;
            *pbVar1 = bVar2 + bVar6 + bVar9;
            pbVar1 = unaff_SI + -0x78;
            *pbVar1 = *pbVar1 + in_CL + (CARRY1(bVar5, bVar6) || CARRY1(bVar2 + bVar6, bVar9));
            uStack00aa = 0;
            uStack00ac = 0;
            if (_g_astruct_112_a == 0) {
                _g_astruct_112_a = CONCAT22(&stack0xbf2a, &stack0xfffe);
            }
            puStack34 = &stack0xfffe;
            pass1_1000_4906(paVar10, 0, 0xac);
            load_cursor_1008_80ee();
            load_cursor_1040_9854();
            modify_list_1008_8168(CONCAT22(unaff_SS, &stack0xbebe));
            return paVar10;
        }
        if (*pbVar1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub fn pass1_1038_ae56(param_1: *mut astruct_44) -> *mut astruct_44 {
    let pbVar1: *mut byte;
    let puVar2: *mut u32;
    let pcVar3: *mut char;
    let mut bVar4: u8;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut cVar7: u8;
    let mut cVar8: u8;
    let mut in_CX: u16;
    let mut bVar9: u8;
    let mut bVar10: u8;
    let mut in_DX: i32;
    let mut bVar12: u8;
    let mut iVar11: i32;
    let mut bVar13: u8;
    let mut in_BX: u16;
    let mut iVar14: i32;
    let puVar15: *mut u8;
    let puVar16: *mut u16;
    let unaff_BP: *mut u16;
    let puVar17: *mut u8;
    let puVar18: *mut u8;
    let unaff_SI: *mut byte;
    let mut unaff_DI: i32;
    let unaff_ES: *mut u8;
    let puVar19: *mut u8;
    let mut unaff_SS: u16;
    let mut bVar20: bool;
    let mut bVar21: bool;
    let mut in_stack_0000007c: u16;
    let mut bStack007d: u8;
    let mut in_stack_0000c741: u32;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar16 = &stack0xfffe;
    puVar15 = &stack0xfffe;
    puVar17 = &stack0xfffe;
    cVar8 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar16 = puVar16 + -1;
        unsafe {
            *puVar16 = *unaff_BP;
        }
        cVar8 = cVar8 + -1;
        '\0' < cVar8
    } {}
    bVar9 = (in_CX >> 8);
    puVar19 = &stack0xc73f;
    bVar10 = in_DX;
    bVar20 = CARRY1(bStack007d, bVar10)
        || CARRY1(
            bStack007d + bVar10,
            unaff_SI[CONCAT11((in_BX >> 8), 0x40)] < bVar10,
        );
    pbVar1 = &stack0x4079 + unaff_SI;
    bVar12 = (in_DX >> 8);
    unsafe {
        bVar21 = CARRY1(*pbVar1, bVar12) || CARRY1(*pbVar1 + bVar12, bVar20);
        *pbVar1 = *pbVar1 + bVar12 + bVar20;
        pbVar1 = unaff_SI;
        bVar4 = *pbVar1;
        bVar13 = *pbVar1 + 0x40;
        bVar20 = 0xbf < *pbVar1 || CARRY1(bVar13, bVar21);
        *pbVar1 = bVar13 + bVar21;
        cVar8 = in_CX;
        if ((*pbVar1 == 0) || (SCARRY1(bVar4, '@') != SCARRY1(bVar13, bVar21)) != (*pbVar1 < '\0'))
        {
            pbVar1 = unaff_SI + 0x3fc8;
            *pbVar1 = *pbVar1 + cVar8 + bVar20;
            cVar7 = PTR_LOOP_1050_4080;
            PTR_LOOP_1050_4080._0_1_ = '@';
            iVar14 = CONCAT11(cVar7, 0x40);
            pbVar1 = unaff_SI;
            *pbVar1 = *pbVar1 + cVar8 + (unaff_SI[0x4040] < bVar10);
            pbVar1 = unaff_SI + iVar14 + 0x10;
            *pbVar1 = *pbVar1 + 0x54;
            pbVar1 = unaff_SI + iVar14 + 0x10;
            *pbVar1 = *pbVar1 - 0x34;
            puVar2 = (unaff_SI + iVar14 + 0x10);
            uVar5 = *puVar2;
            *puVar2 = *puVar2 + 0x81b6;
            puVar2 = (unaff_SI + iVar14 + 0x10);
            uVar6 = *puVar2;
            *puVar2 = *puVar2 + 0x60ea;
            pbVar1 = unaff_SI + iVar14;
            *pbVar1 = (*pbVar1 - (in_DX & 0xff)) - (0x9f15 < uVar6);
            iVar11 = (in_DX & 0xff | (bVar12 + cVar7 + (0x7e49 < uVar5)) << 8) - 1;
            pbVar1 = unaff_SI + iVar14 + 0x10;
            *pbVar1 = *pbVar1 + 0x66;
            pbVar1 = unaff_SI + iVar14 + 0x10;
            bVar4 = *pbVar1;
            *pbVar1 = *pbVar1 - 0x22;
            bVar13 = (iVar11 >> 8);
            if (-1 < *pbVar1) {
                pbVar1 = unaff_SI + iVar14;
                *pbVar1 = (*pbVar1 - iVar11)
                    - (CARRY1(bVar13, bVar9) || CARRY1(bVar13 + bVar9, 0x21 < bVar4));
                // do
                // {
                //     // WARNING: Do nothing block with infinite loop
                // } while (true);
            }
            pcVar3 = (unaff_DI + 8);
            *pcVar3 = *pcVar3 + bVar13;
            puVar15 = (in_stack_0000c741 >> 0x10);
            puVar19 = unaff_ES;
        } else {
            bVar20 = 0xbf < bVar12 || CARRY1(bVar12 + 0x40, bVar20);
            pbVar1 = unaff_SI + 0x4040;
            bVar4 = *pbVar1;
            bVar13 = *pbVar1 - bVar10;
            bVar21 = *pbVar1 < bVar10 || bVar13 < bVar20;
            *pbVar1 = bVar13 - bVar20;
            if ((*pbVar1 == 0)
                || (SBORROW1(bVar4, bVar10) != SBORROW1(bVar13, bVar20)) != (*pbVar1 < '\0'))
            {
                if (*pbVar1 != 0) {
                    error_check_1000_17ce(param_1);
                }
                return param_1;
            }
            pbVar1 = unaff_SI;
            bVar20 = 0xbf < *pbVar1 || CARRY1(*pbVar1 + 0x40, bVar21);
            *pbVar1 = *pbVar1 + 0x40 + bVar21;
            bVar21 = 0xbf < local_4e || CARRY1(local_4e + 0x40, bVar20);
            local_4e = local_4e + 0x40 + bVar20;
            pbVar1 = unaff_SI + -0x4f;
            bVar4 = *pbVar1;
            bVar13 = *pbVar1;
            *pbVar1 = bVar13 + 0x40 + bVar21;
            pbVar1 = unaff_SI + -0x78;
            *pbVar1 = *pbVar1 + cVar8 + (0xbf < bVar4 || CARRY1(bVar13 + 0x40, bVar21));
            puVar18 = &stack0xc72d;
            puVar17 = &stack0xc72d;
            if (_g_astruct_112_a != 0) {}
            // goto LAB_1038_aeed;
        }
    }
    _g_astruct_112_a = CONCAT22(puVar19, puVar15);
    puVar18 = puVar17;
    // LAB_1038_aeed:
    puStack34 = &stack0xfffe;
    pass1_1000_4906((puVar18 + 6), 0, 0xac);
    load_cursor_1008_80ee();
    load_cursor_1040_9854();
    (puVar18 + -0xb4) = s_1_1050_389a;
    (puVar18 + -0xb2) = &PTR_LOOP_1050_1008;
    modify_list_1008_8168(CONCAT22(unaff_SS, puVar18 + -0x5a));
    return CONCAT22((puVar18 + 8), (puVar18 + 6));
}

pub fn set_colors_1038_ac38(
    param_1: u16,
    param_2: u16,
    dialog_handle: HWND16,
    uparam_2_00: i32,
    param_5: HDC16,
) {
    let mut iVar1: i32;
    let local_AX_35: *mut astruct_66;
    let iVar2: u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    GetStockObject16(4);
    if (_PTR_LOOP_1050_5b78 == 0) {
        uVar3 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar4 = (uVar3 >> 0x10);
        iVar1 = uVar3;
        _PTR_LOOP_1050_5b6c = CONCAT12(
            *(iVar1 + 0x3ec),
            CONCAT11(*(iVar1 + 0x3ed), *(iVar1 + 0x3ee)),
        );
        _PTR_LOOP_1050_5b70 = CONCAT12(
            *(iVar1 + 0x3e4),
            CONCAT11(*(iVar1 + 0x3e5), *(iVar1 + 0x3e6)),
        );
        _PTR_LOOP_1050_5b74 = CONCAT12(
            *(iVar1 + 0x3f8),
            CONCAT11(*(iVar1 + 0x3f9), *(iVar1 + 0x3fa)),
        );
        _PTR_LOOP_1050_5b78 = CONCAT12(*(iVar1 + 0x94), CONCAT11(*(iVar1 + 0x95), *(iVar1 + 0x96)));
    }
    if (param_2_00 < 4) {
        // LAB_1038_acf0:
        iVar2 = GetDlgCtrllID16(dialog_handle);
        if (iVar2 == 0xfd4) {
            uVar4 = _PTR_LOOP_1050_5b70;
            uVar5 = (_PTR_LOOP_1050_5b70 >> 0x10);
            // goto LAB_1038_ad0e;
        }
        if (iVar2 != 0xfd5) {
            if (iVar2 == 0xfd6) {
                uVar4 = _PTR_LOOP_1050_5b6c;
                uVar5 = (_PTR_LOOP_1050_5b6c >> 0x10);
                // goto LAB_1038_ad0e;
            }
            if (iVar2 == 0xfd7) {
                uVar4 = _PTR_LOOP_1050_5b74;
                uVar5 = (_PTR_LOOP_1050_5b74 >> 0x10);
                // goto LAB_1038_ad0e;
            }
        }
    } else {
        if (param_2_00 != 4) {
            if ((param_2_00 == 4) || (1 < param_2_00 - 5)) {
                return;
            }
            // goto LAB_1038_acf0;
        }
    }
    uVar4 = _PTR_LOOP_1050_5b78;
    uVar5 = (_PTR_LOOP_1050_5b78 >> 0x10);
    // LAB_1038_ad0e:
    SetTextColor16(CONCAT22(uVar5, uVar4), param_5);
    SetBkColor16(0x1000000, param_5);
    return;
}

pub fn set_window_pos_1038_abdc(param_1: u32) {
    let mut uVar1: u16;
    let mut unaff_CS: u16;
    let unaff_SS: HWND16;
    let mut local_12: [u8; 2];
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_1 >> 0x10);
    GetWindowRect16(CONCAT22(&local_a, unaff_CS), unaff_SS);
    GetDlgItem16(0xfd7, (param_1 + 6));
    GetWindowRect16(CONCAT22(local_12, 0x1538), unaff_SS);
    local_6 = local_6 - local_a;
    local_4 = (local_10 - local_8) - 2;
    SetWindowPos16(6, local_4, local_6, 0, 0, 0, (param_1 + 6));
    return;
}

pub fn enable_window_1038_a8f8(param_1: i32, param_2: u16, param_3: u16_00, param_3: u32) {
    let h_wnd: HWND16;
    let enable: bool;

    if (param_3._2_2_ == 0x116) {
        SendDlgItemMessage16(0, 1, 0x401, 0x11a, (param_1 + 6));
        h_wnd = GetDlgItem16(0x11a, (param_1 + 6));
        enable = 0;
    } else {
        if ((param_3._2_2_ == 0x116) || (2 < param_3._2_2_ - 0x117)) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        h_wnd = GetDlgItem16(0x11a, (param_1 + 6));
        enable = 1;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn enable_window_1038_a972(param_1: *mut astruct_20) {
    let h_wnd: HWND16;
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;

    win_gui_func_1040_78e2(param_1);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    SendDlgItemMessage16(0, 1, 0x401, 0x116, (iVar2 + 6));
    SendDlgItemMessage16(0, 1, 0x401, 0x11a, (iVar2 + 6));
    h_wnd = GetDlgItem16(0x11a, (iVar2 + 6));
    EnableWindow16(0, h_wnd);
    uVar1 = mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x40001);
    (iVar2 + 0x8c) = uVar1;
    get_system_metrics_1038_a18c(iVar2, uVar3);
    ShowWindow16(5, (iVar2 + 6));
    return;
}

pub fn win_fn_1038_a788(param_1: u32, param_2: i32) {
    let hwnd: HWND16;
    let mut uVar1: i32;
    let h_wnd: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let ppVar4: *mut pass1_struct_1;
    let puVar5: *mut u8;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    if (param_2 != 0) {
        uVar3 = (param_1 >> 0x10);
        iVar2 = param_1;
        hwnd = GetDlgItem16(0x115, (iVar2 + 6));
        GetWindowText16(0x50, CONCAT22(unaff_SS, local_52), hwnd);
        uVar1 = get_string_index_1000_3da4(CONCAT22(unaff_SS, local_52));
        if (uVar1 != 0) {
            puVar5 = local_52;
            ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar5, 2));
            pass1_1010_5fd8(ppVar4, CONCAT22(unaff_SS, puVar5));
            h_wnd = GetWindowWord16(-8, (iVar2 + 6));
            PostMessage16(0, 0x105, 0x111, h_wnd);
            destroy_win_1040_7b98(iVar2, uVar3, 1);
        }
    }
    return;
}

pub fn show_window_1038_a6f4(param_1: *mut astruct_20) {
    let lp_string: SEGPTR;
    let hwnd: HWND16;
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;
    let LVar5: LRESULT;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffec, 2));
    lp_string = (ppVar4 + 0x68);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    hwnd = GetDlgItem16(0x115, (iVar2 + 6));
    SetWindowText16(lp_string, hwnd);
    SetFocus16(hwnd);
    LVar5 = SendMessage16(-0x10000, 0, 0x401, hwnd);
    uVar1 = LVar5;
    get_system_metrics_1038_a18c(param_1);
    mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x30001);
    (iVar2 + 0x8c) = uVar1;
    ShowWindow16(5, (iVar2 + 6));
    return;
}

pub fn show_window_1038_a4ee(param_1: *mut astruct_20) {
    let lp_string: SEGPTR;
    let mut in_AX: u16;
    let hwnd: HWND16;
    let mut iVar1: i32;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x20001);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8c) = in_AX;
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffec, 2));
    lp_string = (ppVar3 + 0x6c);
    hwnd = GetDlgItem16(0x114, (iVar1 + 6));
    SetWindowText16(lp_string, hwnd);
    SetFocus16(hwnd);
    SendMessage16(-0x10000, 0, 0x401, hwnd);
    get_system_metrics_1038_a18c(param_1);
    ShowWindow16(5, (iVar1 + 6));
    return;
}

pub fn win_fn_1038_a584(param_1: u32, param_2: i32) {
    let hwnd: HWND16;
    let mut uVar1: i32;
    let h_wnd: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let paVar4: *mut astruct_431;
    let puVar5: *mut u8;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    if (param_2 != 0) {
        uVar3 = (param_1 >> 0x10);
        iVar2 = param_1;
        hwnd = GetDlgItem16(0x114, (iVar2 + 6));
        GetWindowText16(0x50, CONCAT22(unaff_SS, local_52), hwnd);
        uVar1 = get_string_index_1000_3da4(CONCAT22(unaff_SS, local_52));
        if (uVar1 != 0) {
            puVar5 = local_52;
            paVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar5, 2));
            pass1_1010_6006(paVar4, CONCAT22(unaff_SS, puVar5));
            h_wnd = GetWindowWord16(-8, (iVar2 + 6));
            PostMessage16(0, 0x105, 0x111, h_wnd);
            destroy_win_1040_7b98(iVar2, uVar3, 1);
        }
    }
    return;
}

pub fn show_window_1038_a396(param_1: *mut astruct_20) {
    let mut uVar1: u16;
    let mut uVar2: u16;

    win_gui_func_1040_78e2(param_1);
    get_system_metrics_1038_a18c(param_1);
    uVar1 = mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x10001);
    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x8c) = uVar1;
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn post_msg_1038_a3d2(param_1: u32) {
    let h_wnd: u16;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    h_wnd = GetWindowWord16(-8, (param_1 + 6));
    PostMessage16(0, 0x105, 0x111, h_wnd);
    destroy_win_1040_7b98(param_1, uVar1, 1);
    return;
}

pub fn get_system_metrics_1038_a18c(param_1: u32) {
    let ppcVar1: fn();
    let paVar2: *mut astruct_199;
    let iVar3: u16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let unaff_SS: HWND16;
    let puVar5: *mut u16;
    let HVar6: HWND16;
    let puVar7: *mut u8;
    let HVar8: HWND16;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: [u8; 2];
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_4c, 0x27));
    zero_list_1008_3e38(CONCAT22(unaff_SS, local_c));
    modify_list_1008_3f62(
        CONCAT22(unaff_SS, local_c),
        _local_6 & 0xffff0000 | (_local_6 + 0x52),
    );
    paVar2 = local_c;
    pass1_1008_3e94(
        paVar2,
        CONCAT22(unaff_SS, &local_10),
        CONCAT22(unaff_SS, &local_e),
    );
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1c0);
    _local_14 = CONCAT22(extraout_DX, paVar2);
    _local_18 = process_struct_1008_4772(CONCAT22(extraout_DX, paVar2));
    puVar7 = local_1a;
    puVar5 = &local_1c;
    HVar6 = unaff_SS;
    HVar8 = unaff_SS;
    _local_24 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar5, 0x48));
    pass1_1008_3e94(
        (_local_24 + 0xe),
        CONCAT22(HVar6, puVar5),
        CONCAT22(HVar8, puVar7),
    );
    uVar4 = (_local_24 >> 0x10);
    local_1e = (_local_24 + 10);
    local_20 = (_local_24 + 0xc);
    local_10 = local_10 + (local_20 * 10) / 600 + (_local_18 + 8) + local_1c;
    GetWindowRect16(CONCAT22(&local_2c, 0x1008), unaff_SS);
    iVar3 = GetSystemMetrics16(0);
    local_e = (iVar3 - (local_28 - local_2c)) / 2;
    move_window_1040_826c(param_1, (param_1 >> 0x10), local_10, local_e);
    local_4c._0_2_ = SUB42(_local_14, 0);
    local_4c._2_2_ = (_local_14 >> 0x10);
    if (_local_14 != 0x0) {
        ppcVar1 = *_local_14;
        (**ppcVar1)(
            &PTR_LOOP_1050_1040,
            local_4c,
            local_4c._2_2_,
            1,
            local_4c,
            local_4c._2_2_,
            _local_14,
        );
    }
    return;
}

pub fn dc_func_1038_9ffa(param_1: u32) -> u16 {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut uvar3: u16;
    let puVar4: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_4 = GetDC16((param_1 + 6));
    puVar4 = mixed_fn_1010_830a(_g_struct_73_1050_14cc, 3);
    unsafe { iVar2 = *puVar4 };
    ppcVar1 = (iVar2 + 8);
    (**ppcVar1)(0x1010, puVar4, &local_4);
    ppcVar1 = (iVar2 + 4);
    (**ppcVar1)(0x1010, puVar4, 0x50005, &local_4);
    ppcVar1 = (iVar2 + 0xc);
    (**ppcVar1)(0x1010, puVar4, &local_4);
    ReleaseDC16(local_4, (param_1 + 6));
    return 0;
}

pub fn show_window_1038_9fd0(param_1: *mut astruct_20) {
    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn win_fn_1038_9bc8(param_1: *mut astruct_20) {
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let mut iVar3: i32;
    let iVar4: u16;
    let mut iVar5: i32;
    let hdc: HDC16;
    let mut iVar6: i32;
    let HVar7: HWND16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let unaff_SS: HWND16;
    let ppVar10: *mut pass1_struct_1;
    let puVar11: *mut u16;
    let uVar12: u8;
    let uVar13: u8;
    let HVar14: HWND16;
    let mut in_stack_0000ffc2: u32;
    let mut uVar15: u16;
    let in_string_1: *mut char;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 2];
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar15 = (in_stack_0000ffc2 >> 0x10);
    win_gui_func_1040_78e2(param_1);
    if (PTR_LOOP_1050_5ef8 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        PTR_LOOP_1050_5ef8 = 0x0;
    }
    uVar12 = SUB21(&local_4, 0);
    uVar13 = (&local_4 >> 8);
    puVar11 = &local_6;
    HVar7 = unaff_SS;
    HVar14 = unaff_SS;
    _local_a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar11, 0x48));
    pass1_1008_3e94(
        (_local_a + 0xe),
        CONCAT22(HVar7, puVar11),
        CONCAT22(HVar14, CONCAT11(uVar13, uVar12)),
    );
    iVar4 = GetSystemMetrics16(4);
    iVar5 = iVar4 * PTR_LOOP_1050_5ef8 + 10;
    PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
    _local_e = CONCAT22(iVar5 + local_4, iVar5 + local_6);
    uVar8 = (param_1 >> 0x10);
    iVar5 = param_1;
    GetWindowRect16(
        CONCAT13((local_16 >> 8), CONCAT12(local_16, 0x1538)),
        unaff_SS,
    );
    hdc = GetDC16(0);
    iVar4 = GetDeviceCaps16(10, hdc);
    ReleaseDC16(hdc, 0);
    if (iVar4 < local_10) {
        _local_e = _local_e & 0xffff0000 | ((local_14 - (local_10 - iVar4)) + 1);
    }
    SetWindowPos16(1, 0, 0, _local_e, (_local_e >> 0x10), 0, (iVar5 + 6));
    in_string_1 = CONCAT22(uVar15, 3);
    uVar9 = 0x1010;
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, in_string_1);
    iVar6 = ppVar10 + 0xa4;
    uVar15 = (ppVar10 >> 0x10);
    local_24 = 0;
    while (iVar3 = local_24 * 2, (iVar3 + iVar6) != 0) {
        in_string_1 = (in_string_1 & 0xffff0000);
        uVar9 = SUB42(offset, 0);
        HVar7 = GetDlgItem16((iVar3 + iVar6), (iVar5 + 6));
        (iVar5 + iVar3 + 0x94) = HVar7;
        local_24 = local_24 + 1;
        piVar1 = (iVar5 + 0x128);
        unsafe {
            *piVar1 = *piVar1 + 1;
        }
    }
    ppcVar2 = (param_1 + 0x6c);
    ppcVar2(uVar9, iVar5, uVar8, (in_string_1 >> 0x10));
    return;
}

pub fn enable_window_1038_9cec(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16_00,
    param_3: u16,
    param_5: i32,
) {
    let paVar1: *mut astruct_124;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let mut unaff_SI: u16;
    let ppVar5: *mut pass1_struct_1;
    let enable: bool;
    let HVar6: HWND16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    if (param_5 == 0xeb) {
        win_gui_fn_1040_b54a(param_1, param_2, param_3_00, CONCAT22(0xeb, param_3));
        ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
        iVar4 = ppVar5 + 0xa4;
        uVar3 = (ppVar5 >> 0x10);
        local_c = 0;
        while (iVar2 = local_c * 2, (iVar2 + iVar4) != 0) {
            HVar6 = GetDlgItem16((iVar2 + iVar4), &param_1.field_0x6);
            (&param_1[1].field_0x0 + iVar2) = HVar6;
            local_c = local_c + 1;
            paVar1 = (param_1 + 2);
            paVar1 = paVar1 + 1;
        }
    } else {
        if (param_5 == 0xf8) {
            HVar6 = GetDlgItem16(0x17d8, &param_1.field_0x6);
            enable = 1;
        } else {
            if (param_5 != 0x17d8) {
                win_gui_fn_1040_b54a(param_1, param_2, param_3_00, CONCAT22(param_5, param_3));
                return;
            }
            SetWindowPos16(6, 0xed, 0x237, 0, 0, 0, &param_1.field_0x6);
            enable = offset;
            GetDlgItem16(0x17d8, &param_1.field_0x6);
            HVar6 = 0;
        }
        EnableWindow16(enable, HVar6);
    }
    return;
}

pub fn enable_window_1038_9a66(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16_00,
    param_3: u32,
) {
    let enable: bool;
    let h_wnd: HWND16;

    if (param_3._2_2_ == 0xf8) {
        h_wnd = GetDlgItem16(0x17d9, &param_1.field_0x6);
        enable = 1;
    } else {
        if (param_3._2_2_ != 0x17d9) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        enable = 0;
        SetWindowPos16(6, 0x1a0, 300, 0, 0, 0, &param_1.field_0x6);
        h_wnd = 0;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn win_fn_1038_977a(param_1: i32, param_2: u16) {
    let ppcVar1: fn();
    let mut uVar2: i32;
    let mut uvar3: u16;
    let in_DX: *mut astruct_199;
    let mut uVar4: i32;
    let unaff_SS: u16;
    let mut uVar5: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 4];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    local_8 = 0;
    uVar5 = (param_1 + 6);
    uVar2 = GetDlgItemInt16(1, &local_4, unaff_SS, 0xfa8);
    local_6 = uVar2;
    if (uVar2 != 0) {
        process_struct_1000_179c(0xb4, in_DX);
        uVar4 = in_DX | uVar2;
        if (uVar4 == 0) {
            uVar3 = 0;
            uVar4 = 0;
        } else {
            uVar3 = mixed_1040_8520(CONCAT22(in_DX, uVar2), (param_1 + 6), 0x41, 2, 0x5db, 0x5da);
        }
        _local_c = CONCAT22(uVar4, uVar3);
        pass1_1008_941a(CONCAT22(unaff_SS, local_10), 1, 0xc3);
        ppcVar1 = (*_local_c + 0x6c);
        local_8 = (**ppcVar1)(
            &PTR_LOOP_1050_1008,
            _local_c,
            (_local_c >> 0x10),
            local_10,
            unaff_SS,
            uVar5,
            uVar2,
        );
    }
    if ((local_8 == 1) || (local_6 == 0)) {
        destroy_window_1040_b726();
    }
    return;
}

pub fn get_dialog_int_1038_9820(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> libc::c_long {
    let piVar1: *mut i32;
    let lVar2: u32;
    let mut uVar3: u16;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let unaff_SS: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_6: bool;
    let local_4: bool;

    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    uVar3 = GetDlgItemInt16(1, &local_4, unaff_SS, (param_4 * 0xe + 0x5a74));
    iVar4 = uVar3 * param_2 * param_3;
    uVar3 = GetDlgItemInt16(1, &local_6, unaff_SS, (param_4 * 0xe + 0x5a76));
    lVar2 = (uVar3 * param_2) * param_3;
    uVar6 = (lVar2 >> 0x10);
    iVar5 = lVar2;
    if ((iVar4 - (iVar7 + 0x94) < 1) && (-1 < (iVar7 + 0x96) - iVar5)) {
        piVar1 = (iVar7 + 0x94);
        unsafe {
            *piVar1 = *piVar1 - iVar4;
        }
        piVar1 = (iVar7 + 0x96);
        unsafe {
            *piVar1 = *piVar1 - iVar5;
        }
        return CONCAT22(uVar6, 1);
    }
    return uVar6 << 0x10;
}

pub fn dialog_item_func_1038_98b4(param_1: u32) {
    let mut uVar1: u16;
    let mut iVar2: i32;
    let unaff_SS: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    local_8 = 0;
    while (iVar2 = param_1, uVar3 = (param_1 >> 0x10), local_8 < 0xf) {
        uVar4 = 1;
        uVar3 = (iVar2 + 6);
        uVar1 = GetDlgItemInt16(1, &local_4, unaff_SS, (local_8 * 0xe + 0x5a72));
        get_dialog_int32_1038_9820(param_1, uVar1, uVar3, uVar4);
        local_8 = local_8 + 1;
    }
    SetDlgItemInt16(1, *(iVar2 + 0x94), 0xfa9, (iVar2 + 6));
    SetDlgItemInt16(1, *(iVar2 + 0x96), 0xfa8, (iVar2 + 6));
    return;
}

pub fn win_fn_1038_95fc(param_1: u32) {
    let ppcVar1: fn();
    let mut uVar2: i32;
    let paVar3: *mut astruct_199;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let struct_a: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let mut iVar6: i32;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let unaff_SS: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let local_10: bool;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 8));
    _local_a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 9));
    paVar3 = (_local_a >> 0x10);
    uVar2 = _local_a;
    process_struct_1000_179c(0xc, paVar3);
    struct_a = (paVar3 | uVar2);
    if (struct_a == 0x0) {
        paVar3 = 0x0;
        struct_a = 0x0;
    } else {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, uVar2));
    }
    _local_e = CONCAT22(struct_a, paVar3);
    local_14 = 0;
    while (local_14 < 0xf) {
        uVar10 = (param_1 + 6);
        uVar4 = GetDlgItemInt16(1, &local_10, unaff_SS, (local_14 * 0xe + 0x5a72));
        if (uVar4 != 0) {
            if ((local_14 * 0xe + 0x5a7c) < 0x83) {
                uVar5 = uVar4;
                process_struct_1000_179c(8, struct_a);
                _local_18 = CONCAT22(struct_a, uVar5);
                if ((struct_a | uVar5) == 0) {
                    local_1e = 0;
                } else {
                    *_local_18 = s_1_1050_389a;
                    (uVar5 + 2) = &PTR_LOOP_1050_1008;
                    *_local_18 = 0xa1c4;
                    (uVar5 + 2) = 0x1010;
                    local_1e = _local_18;
                }
                uVar7 = (local_1e >> 0x10);
                iVar6 = local_1e;
                *(iVar6 + 6) = uVar4;
                (iVar6 + 4) = (local_14 * 0xe + 0x5a7c);
                ppcVar1 = (*_local_e + 4);
                (**ppcVar1)(0x1000, _local_e, (_local_e >> 0x10), iVar6, uVar7, uVar10);
                struct_a = extraout_DX;
            } else {
                if ((local_14 * 0xe + 0x5a7c) == 0x89) {
                    uVar9 = (local_14 * 0xe + 0x5a7c);
                    uVar8 = uVar4;
                } else {
                    uVar9 = (local_14 * 0xe + 0x5a7c);
                    uVar8 = 0;
                }
                pass1_1010_6566(_local_a, uVar8, uVar4, uVar9);
                struct_a = extraout_DX_00;
            }
        }
        local_14 = local_14 + 1;
    }
    (_local_6 + 10) = _local_e;
    PostMessage16(0, 0xed, 0x111, g_h_window);
    return;
}

pub fn win_fn_1038_9294(param_1: u32) {
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let unaff_SS: u16;
    let local_6: bool;
    let local_4: bool;

    set_window_pos_1040_b230(param_1);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = GetDlgItemInt16(1, &local_4, unaff_SS, 0xfa9);
    *(iVar2 + 0x94) = uVar1;
    uVar1 = GetDlgItemInt16(1, &local_6, unaff_SS, 0xfa8);
    *(iVar2 + 0x96) = uVar1;
    dialog_item_func_1038_98b4(iVar2, uVar3);
    mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x950001);
    return;
}

pub fn win_gui_fn_1038_92f6(
    param_1: *mut astruct_124,
    param_2_00: *mut u8,
    param_3: u16,
    param_2: u32,
) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: i32;
    let iVar4: i16;
    let mut uVar5: u32;
    let extraout_DX: *mut astruct_199;
    let paVar6: *mut astruct_199;
    let mut iVar7: i32;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let unaff_SS: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ == 0xeb) {
        _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
        paVar6 = (_local_6 >> 0x10);
        uVar5 = &param_1.field_0x90;
        if (uVar5 != 0) {
            local_a = uVar5;
            process_struct_1000_179c(0x18, paVar6);
            uVar3 = uVar5;
            _local_10 = (uVar5 & 0xffff | ZEXT24(paVar6) << 0x10);
            if ((paVar6 | uVar3) == 0) {
                uVar3 = 0;
                paVar6 = 0x0;
            } else {
                process_struct_1040_a598((uVar5 & 0xffff | ZEXT24(paVar6) << 0x10));
                paVar6 = extraout_DX;
            }
            param_1.field_0x90 = uVar3;
            &param_1.field_0x92 = paVar6;
            *&param_1.field_0x90 = 0x11;
            local_c = *&param_1.field_0x90;
            uVar3 = local_c * 10 + 2;
            process_struct_1000_179c(uVar3, paVar6);
            _local_10 = CONCAT22(paVar6, uVar3);
            if ((paVar6 | uVar3) == 0) {
                uVar1 = &param_1.field_0x90;
                (uVar1 + 2) = 0;
            } else {
                *_local_10 = local_c;
                call_fn_ptr_1000_5586(0xa564, &PTR_LOOP_1050_1040, local_c, 10, uVar3 + 2, paVar6);
                uVar1 = &param_1.field_0x90;
                uVar8 = (uVar1 >> 0x10);
                iVar7 = uVar1;
                (iVar7 + 2) = uVar3 + 2;
                (iVar7 + 4) = paVar6;
            }
            uVar8 = (local_a >> 0x10);
            uVar1 = &param_1.field_0x90;
            (uVar1 + 6) = (local_a + 6);
            uVar1 = &param_1.field_0x90;
            (uVar1 + 10) = (local_a + 10);
            uVar1 = &param_1.field_0x90;
            (uVar1 + 0x12) = &param_1.field_0xa;
            uVar8 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505b42, &param_1.field_0x90);
            local_14 = local_a;
            _local_10 = local_a;
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                uVar8 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            ppcVar2 = (CONCAT22(param_2_00, param_1) + 0x70);
            ppcVar2(uVar8, param_1, param_2_00);
        }
    } else {
        if (param_2._2_2_ != 0xf9) {
            win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        iVar4 = pass1_1038_993a(param_1, param_2_00, param_3);
        if (-1 < iVar4) {
            local_16 = GetDlgItemInt16(1, &local_1a, unaff_SS, (iVar4 * 0xe + 0x5a72));
            if (local_1a != 0) {
                uVar1 = &param_1[1].field_0x4;
                win_gui_fn_1010_2a32(uVar1, (uVar1 >> 0x10), local_16, (iVar4 * 0xe + 0x5a72));
            }
        }
    }
    return;
}

pub fn dialog_fn_1038_94da(
    param_1: *mut u8,
    param_2: *mut u8,
    param_3: u16_00,
    param_4: u16,
    param_3: i32,
) -> u16 {
    let puVar1: *mut u16;
    let mut iVar2: i32;
    let mut uVar3: u16;
    let hwnd: HWND16;
    let mut iVar4: i32;
    let unaff_SS: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 1;
    local_8 = pass1_1038_993a(param_1, param_2, param_3_00);
    if ((-1 < local_8)
        && (
            uVar3 = GetDlgItemInt16(1, &local_c, unaff_SS, (local_8 * 0xe + 0x5a72)),
            local_c != 0,
        ))
    {
        if (param_3 == 0) {
            local_6 = uVar3 + 1;
        } else {
            local_4 = 0xffff;
            local_6 = uVar3 - 1;
        }
        local_a = (local_6 <= (local_8 * 0xe + 0x5a7a));
        puVar1 = (local_8 * 0xe + 0x5a78);
        if (unsafe { *puVar1 != local_6 } && unsafe { local_6 <= *puVar1 }) {
            local_a = 0;
        }
        iVar2 = local_8 * 0xe;
        hwnd = GetDlgItem16((iVar2 + 0x5a72), (param_1 + 6));
        SetFocus16(hwnd);
        if ((local_a != 0)
            && (
                iVar4 = get_dialog_int32_1038_9820(CONCAT22(param_2, param_1), 1, local_4, local_8),
                iVar4 != 0,
            ))
        {
            SetDlgItemInt16(1, local_6, (iVar2 + 0x5a72), (param_1 + 6));
            SetDlgItemInt16(1, *(param_1 + 0x94), 0xfa9, (param_1 + 6));
            SetDlgItemInt16(1, *(param_1 + 0x96), 0xfa8, (param_1 + 6));
        }
    }
    return 1;
}

pub fn win_gui_fn_1038_8d98(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16_00,
    param_3: u32,
) {
    if (param_3._2_2_ == 0xeb) {
        win_fn_1038_8f74(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        load_Str_1038_8dda(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn load_Str_1038_8dda(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x803,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x804,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x805,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x806,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x807,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x808,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    uVar1 = (param_1 >> 0x10);
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x809,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x80a,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_DX, in_AX));
    return;
}

pub fn win_fn_1038_8f74(param_1: *mut u8) -> u8 {
    let mut iVar1: i32;
    let h_wnd: HWND16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let lVar4: u32;
    let LVar5: LRESULT;
    let enable: bool;
    let mut local_510: u16;
    let mut local_50e: u16;
    let mut local_50c: u16;
    let mut uStack1290: u32;
    let HStack1286: HWND16;
    let mut local_40c: [u8; 8];
    let mut local_404: u16;
    let mut local_402: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, (s_650_bmp_1050_1859 + 2), (iVar2 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_650_bmp_1050_1859 + 2), (iVar2 + 6));
    iVar1 = pass1_1008_c83a((iVar2 + 0x94));
    if (iVar1 == 0) {
        _local_404 = pass1_1008_c85e((iVar2 + 0x94));
        pass1_1008_5784(CONCAT22(unaff_SS, local_40c), _local_404);
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_SS, local_40c));
            if (lVar4 == 0) {
                break;
            }
            wsprintf16(
                &local_50c,
                CONCAT22(0x5a6c, unaff_SS),
                CONCAT22((lVar4 + 4), 0x1050),
            );
            HStack1286 = (iVar2 + 6);
            uStack1290 = 0x185b0401;
            local_50c = 0;
            _local_510 = CONCAT22(unaff_SS, &local_50c);
            SendDlgItemMessage16(_local_510, 0, 0x401, (s_650_bmp_1050_1859 + 2), HStack1286);
        }
        h_wnd = GetDlgItem16(1, (iVar2 + 6));
        enable = 1;
    } else {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_SS, &local_404),
            0x452,
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_SS, &local_404),
            0,
            0x401,
            (s_650_bmp_1050_1859 + 2),
            (iVar2 + 6),
        );
        h_wnd = GetDlgItem16(1, (iVar2 + 6));
        enable = 0;
    }
    EnableWindow16(enable, h_wnd);
    LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_650_bmp_1050_1859 + 2), (iVar2 + 6));
    return LVar5;
}

pub fn send_dlg_item_msg_1038_8d22(param_1: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let LVar3: LRESULT;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, (s_650_bmp_1050_1859 + 2), (iVar1 + 6));
    local_6 = LVar3;
    local_4 = local_6 >> 0xf;
    if (local_6 != 0xffff) {
        SendDlgItemMessage16(
            CONCAT22(unaff_SS, local_106),
            local_6,
            0x40a,
            (s_650_bmp_1050_1859 + 2),
            (iVar1 + 6),
        );
        pass1_1008_c79a((iVar1 + 0x94), CONCAT22(unaff_SS, local_106));
    }
    return;
}

pub fn send_dlg_item_msg_1038_8b58(param_1: *mut u8) -> u8 {
    let mut u_var1: u32;
    let paVar2: *mut astruct_1096;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut in_stack_0000feee: u16;
    let mut local_10a: u32;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000feee, 3));
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    pass1_1010_c3c2(
        _local_6,
        (_local_6 >> 0x10),
        CONCAT22(unaff_SS, local_106),
        (iVar3 + 0x94),
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_SS, local_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        (iVar3 + 6),
    );
    uVar1 = (iVar3 + 0x94);
    (iVar3 + 0x9c) = (uVar1 + 0x32);
    (iVar3 + 0x9a) = (iVar3 + 0x9c);
    SetDlgItemInt16(
        0,
        *(iVar3 + 0x9c),
        (s_dibtext_bmp_1050_1844 + 9),
        (iVar3 + 6),
    );
    uVar1 = (iVar3 + 0x94);
    paVar2 = (uVar1 + 0x2e);
    pass1_1038_3aa6(paVar2);
    *(iVar3 + 0x98) = paVar2;
    SetDlgItemInt16(0, paVar2, (s_dibtext_bmp_1050_1844 + 0xb), (iVar3 + 6));
    return paVar2;
}

pub fn msg_box_1038_8a3a(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: [u8; 258];
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    _local_108 = CONCAT22(in_DX, in_AX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x7e6,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7e7,
    );
    process_string_1000_3cea(_local_108, CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x7e8,
    );
    process_string_1000_3cea(_local_108, CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x101,
        CONCAT22(unaff_SS, local_20a),
        0x57b,
    );
    MessageBox16(0, CONCAT22(unaff_SS, local_20a), _local_108, (param_1 + 6));
    error_check_1000_17ce(_local_108);
    return;
}

pub fn set_dialog_item_1038_8966(param_1: u32, param_2: u16_00, param_3: u16, param_2: i32) -> u16 {
    let piVar1: *mut i32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_4: u16;

    bVar2 = false;
    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        if ((iVar3 + 0x98) < 1) {}
        // goto LAB_1038_89af;
        piVar1 = (iVar3 + 0x9a);
        unsafe {
            *piVar1 = *piVar1 + 1;
        }
        piVar1 = (iVar3 + 0x98);
        unsafe {
            *piVar1 = *piVar1 + -1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1038_89af;
        if ((iVar3 + 0x9a) < 1) {}
        // goto LAB_1038_89af;
        piVar1 = (iVar3 + 0x9a);
        unsafe {
            *piVar1 = *piVar1 + -1;
        }
        piVar1 = (iVar3 + 0x98);
        unsafe {
            *piVar1 = *piVar1 + 1;
        }
    }
    bVar2 = true;
    // LAB_1038_89af:
    if (bVar2) {
        SetDlgItemInt16(
            0,
            *(iVar3 + 0x9a),
            (s_dibtext_bmp_1050_1844 + 9),
            (iVar3 + 6),
        );
        SetDlgItemInt16(
            0,
            *(iVar3 + 0x98),
            (s_dibtext_bmp_1050_1844 + 0xb),
            (iVar3 + 6),
        );
    }
    return 0;
}

pub fn pass1_1038_89e8(param_1: *mut u8) {
    send_dlg_item_msg_1038_8b58(param_1);
    return;
}

pub fn win_gui_fn_1038_89f8(
    param_1: *mut astruct_124,
    param_2: u16,
    param_3: u16_00,
    param_3: u32,
) {
    if (param_3._2_2_ == 0xeb) {
        send_dlg_item_msg_1038_8b58(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        msg_box_1038_8a3a(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn send_dialog_item_msg_1038_844a(param_1: u32) -> LRESULT {
    let hwnd: HWND16;
    let BVar1: bool;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let LVar5: LRESULT;
    let mut local_10a: u16;
    let mut local_108: [u8; 258];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 4), (iVar3 + 6));
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 5), (iVar3 + 6));
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 6), (iVar3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 4), (iVar3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (iVar3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (iVar3 + 6));
    _local_6 = pass1_1008_b820((iVar3 + 0x94));
    if (_local_6 == 0) {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_SS, local_108),
            0x448,
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_SS, local_108),
            0,
            0x401,
            (s_logo_bmp_1050_1850 + 4),
            (iVar3 + 6),
        );
        SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (iVar3 + 6));
        SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (iVar3 + 6));
        LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (iVar3 + 6));
        uVar2 = (LVar5 >> 0x10);
        hwnd = GetDlgItem16((s_logo_bmp_1050_1850 + 7), (iVar3 + 6));
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_SS, local_108),
            0x449,
        );
        BVar1 = SetWindowText16(CONCAT22(unaff_SS, local_108), hwnd);
        return CONCAT22(uVar2, BVar1);
    }
    send_dialog_item_msg_1038_8400(
        iVar3,
        (param_1 >> 0x10),
        _local_6,
        (s_logo_bmp_1050_1850 + 4),
    );
    set_window_text_1038_8358(iVar3, uVar4);
    SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (iVar3 + 6));
    SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (iVar3 + 6));
    LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (iVar3 + 6));
    return LVar5;
}

pub fn set_window_text_1038_8358(param_1: *mut u8) {
    let puVar1: *mut u8;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_30a: [u8; 258];
    let mut local_208: [u8; 256];
    let mut local_108: [u8; 256];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    local_4 = GetDlgItem16((s_logo_bmp_1050_1850 + 7), (iVar3 + 6));
    _local_8 = pass1_1008_b820((iVar3 + 0x94));
    if (_local_8 == 0) {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_SS, local_30a),
            0x449,
        );
        puVar1 = local_30a;
    } else {
        iVar2 = send_dlg_item_msg_1038_8164(
            param_1,
            CONCAT22(unaff_SS, local_108),
            (s_logo_bmp_1050_1850 + 4),
        );
        if (iVar2 == 0) {
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x100,
                CONCAT22(unaff_SS, local_208),
                0x447,
            );
        } else {
            load_string_1008_b65a(
                (iVar3 + 0x94),
                CONCAT22(unaff_SS, local_208),
                CONCAT22(unaff_SS, local_108),
            );
        }
        puVar1 = local_208;
    }
    SetWindowText16(CONCAT22(unaff_SS, puVar1), local_4);
    return;
}

pub fn enable_window_1038_806a(param_1: u32) {
    let HVar1: HWND16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    HVar1 = GetDlgItem16(1, (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_logo_bmp_1050_1850 + 8), (iVar2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16(s_650_bmp_1050_1859, (iVar2 + 6));
    EnableWindow16(0, HVar1);
    uVar4 = pass1_1008_b820((iVar2 + 0x94));
    if (uVar4 != 0) {
        uVar4 = pass1_1008_b340((iVar2 + 0x94));
        uVar5 = pass1_1008_b366((iVar2 + 0x94));
        uVar6 = pass1_1008_b47a((iVar2 + 0x94));
        if (((uVar4 != 0) && (uVar5 != 0)) && (uVar6 != 0)) {
            HVar1 = GetDlgItem16(1, (iVar2 + 6));
            EnableWindow16(1, HVar1);
            HVar1 = GetDlgItem16((s_logo_bmp_1050_1850 + 8), (iVar2 + 6));
            EnableWindow16(1, HVar1);
        }
        if (uVar4 != 0) {
            HVar1 = GetDlgItem16(s_650_bmp_1050_1859, (iVar2 + 6));
            EnableWindow16(1, HVar1);
        }
    }
    return;
}

pub fn send_dlg_item_msg_1038_8164(param_1: *mut u8, param_2: *mut u8, param_3: u16) -> u16 {
    let mut uVar1: u16;
    let LVar2: LRESULT;

    unsafe {
        *param_2 = '\0';
    }
    uVar1 = (param_1 >> 0x10);
    LVar2 = SendDlgItemMessage16(0, 0, 0x409, param_3, (param_1 + 6));
    if ((LVar2 != -1)
        && (
            LVar2 = SendDlgItemMessage16(param_2, LVar2, 0x40a, param_3, (param_1 + 6)),
            LVar2 != -1,
        ))
    {
        return 1;
    }
    return 0;
}

pub fn win_gui_fn_1038_7dc6(
    param_1: *mut astruct_124,
    param_2: u16_00,
    param_3: u16,
    param_2: u32,
) {
    let mut bVar1: bool;
    let mut local_4: u16;

    bVar1 = false;
    if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 4)) {
        if (param_2 != 1) {}
        // goto LAB_1038_7e8c;
        send_dlg_item_msg_1038_8618(CONCAT22(param_2_00, param_1));
    } else {
        if (param_2._2_2_ < (s_logo_bmp_1050_1850 + 5)) {
            if (param_2._2_2_ == 0xeb) {
                send_dialog_item_msg_1038_844a(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2._2_2_ == 0xfb) {
                    send_dlg_item_msg_1038_7eac(CONCAT22(param_2_00, param_1));
                } else {
                    if (param_2._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
                        // LAB_1038_7e77:
                        win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                        return;
                    }
                    load_str_1038_81be(CONCAT22(param_2_00, param_1));
                }
            }
            // goto LAB_1038_7e8c;
        }
        if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 5)) {
            if (param_2 != 1) {}
            // goto LAB_1038_7e8c;
            send_dlg_item_msg_1038_87b2(CONCAT22(param_2_00, param_1));
        } else {
            if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 6)) {
                if (param_2 != 1) {}
                // goto LAB_1038_7e8c;
                pass1_1038_8810(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 8)) {
                    send_dlg_item_msg_1038_7fae(CONCAT22(param_2_00, param_1));
                } else {
                    if (param_2._2_2_ != s_650_bmp_1050_1859) {}
                    // goto LAB_1038_7e77;
                    pass1_1038_801a(CONCAT22(param_2_00, param_1));
                }
            }
        }
    }
    bVar1 = true;
    // LAB_1038_7e8c:
    if (bVar1) {
        set_window_text_1038_8358(CONCAT22(param_2_00, param_1));
        enable_window_1038_806a(CONCAT22(param_2_00, param_1));
    }
    return;
}

pub fn send_dlg_item_msg_1038_7eac(param_1: u32) -> LRESULT {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut l_param: u32;
    let LVar4: LRESULT;
    let mut in_stack_0000fff2: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff2, 0x30));
    l_param = pass1_1010_375e(ppVar3);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    pass1_1008_b1a6((iVar1 + 0x94), l_param);
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 4), (iVar1 + 6));
    LVar4 = SendDlgItemMessage16(0, 0, 0x409, (s_logo_bmp_1050_1850 + 4), (iVar1 + 6));
    if (LVar4 != -1) {
        SendDlgItemMessage16(0, LVar4, 0x403, (s_logo_bmp_1050_1850 + 4), (iVar1 + 6));
        SendDlgItemMessage16(l_param, 0, 0x401, (s_logo_bmp_1050_1850 + 4), (iVar1 + 6));
        SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 4), (iVar1 + 6));
        SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (iVar1 + 6));
        SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (iVar1 + 6));
        enable_window_1038_806a(iVar1, uVar2);
    }
    LVar4 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (iVar1 + 6));
    return LVar4;
}

pub fn send_dlg_item_msg_1038_7fae(param_1: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    pass1_1008_b146((iVar1 + 0x94));
    SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 5), (iVar1 + 6));
    SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 6), (iVar1 + 6));
    pass1_1008_b61a((iVar1 + 0x94), 0);
    pass1_1008_b63a(*(iVar1 + 0x94), 0x0);
    return;
}

pub fn call_load_cursor_1020_790e(
    in_struct_1: *mut win_struct_42,
    param_2: *mut char,
    param_3: u16,
    param_4: u32,
) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;

    load_cursor_1008_7f62(in_struct_1, param_3, param_4);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    &local_struct_1.field_0xe0 = 0;
    &local_struct_1.u16_xe4 = 0;
    &local_struct_1.field_0xe8 = 0;
    &local_struct_1.field_0xec = 0;
    *&local_struct_1.char_ptr_16_0xee = param_2;
    in_struct_1.u16_x0 = 0x7b86;
    local_struct_1.u16_x2 = 0x1020;
    return;
}

pub fn destroy_menu_func_1020_795c(in_struct_1: *mut astruct_44) {
    let local_struct_1: *mut astruct_215;
    let local_struct_1_hi: *mut astruct_215;
    let mut menu_handle: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x7b86;
    local_struct_1.ptr_a_hi = 0x1020;
    if (local_struct_1.field_0xec != 0) {
        DestroyMenu16(menu_handle);
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = s_1_1050_389a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn win_gui_fn_1020_7824(param_1: *mut astruct_622, param_2: u16) {
    let mut local_AX__1: u16;
    let mut iVar1: i32;
    let mut local_DX_46: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut local_e: u16;
    let mut local_6: u32;
    let mut temp_5fa3752684: u32;
    let fn_ptr_1: fn();

    get_dc_1020_921c(param_1, param_2);
    (param_1 + 0x14) = 0;
    param_1.u16_0x0 = 0x7902;
    (param_1 + 2) = 0x1020;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_e, 0x25));
    local_DX_46 = (ppVar2 >> 0x10);
    (param_1 + 0x14) = ppVar2;
    (param_1 + 0x16) = local_DX_46;
    (param_1 + 6) = (param_1 + 0x14);
    (param_1 + 8) = local_DX_46;
    temp_5fa3752684 = (param_1 + 0x14);
    iVar1 = param_1 + 10;
    fn_ptr_1 = ((temp_5fa3752684 + 10) + 8);
    (**fn_ptr_1)();
    (param_1 + 0x12) = iVar1;
    draw_1020_9364((param_1 & 0xffff | param_1._2_2_ << 0x10));
    return;
}

pub fn win_gui_fn_1020_76aa(in_struct_1: *mut win_struct_42) {
    let paVar1: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut uVar2: i32;
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let paVar3: *mut astruct_199;
    let mut local_4: u16;

    paVar3 = create_win_1008_9760(in_struct_1);
    struct_a = (paVar3 >> 0x10);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    uVar2 = struct_a | paVar1;
    if (uVar2 != 0) {
        win_gui_fn_1020_7824(CONCAT22(struct_a, paVar1), local_struct_1.win_handle_0x8);
        local_struct_1.char_ptr_16_0xee = paVar1;
        local_struct_1.field_0xf0 = uVar2;
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub fn win_gui_fn_1020_75f0(param_1: *mut astruct_675) {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let struct_a: *mut astruct_199;
    let paVar4: *mut astruct_199;
    let mut extraout_DX: i32;
    let local_BX_4: *mut astruct_675;
    let local_ES_4: *mut astruct_675;
    let mut unaff_SS: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 4];
    let fn_ptr_1: fn();

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0xee != 0) {
        fn_ptr_1 = (local_BX_4.field_0xee + 8);
        (**fn_ptr_1)();
    }
    if (local_BX_4.field_0xea == 0) {
        local_BX_4.field_0xea = 1;
        pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x91);
        uVar3 = ZEXT24(local_6);
        mci_send_command_1008_5c9e(_g_struct_ptr_1050_02a0, CONCAT22(unaff_SS, local_6));
        local_BX_4.field_0xec = uVar3;
        paVar4 = struct_a;
        process_struct_1000_179c(0x112, struct_a);
        if ((paVar4 | uVar3) == 0) {
            uVar2 = 0;
            _local_a = 0x0;
        } else {
            piVar1 = &local_BX_4.field_0xcc;
            unsafe {
                *piVar1 = *piVar1 + 1;
            }
            win_gui_fn_1020_3644(uVar3, paVar4, local_BX_4.field_0xcc, param_1);
            uVar2 = uVar3;
            _local_a = (uVar3 & 0xffff | extraout_DX << 0x10);
        }
        pass1_1008_6978(param_1, 0, _local_a & 0xffff0000 | uVar2);
        fn_ptr_1 = (*_local_a + 0xc);
        (**fn_ptr_1)(8, _local_a, local_8, 5);
    }
    return;
}

pub fn win_gui_fn_1020_43f6(in_struct_1: *mut win_struct_42) {
    let ppVar1: *mut pass1_struct_1;
    let lVar2: u32;
    let mut uVar3: u32;
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let mut uStack10: u16;
    let mut local_8: u16;
    let fn_ptr_1: fn();

    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    create_win_1008_9760(in_struct_1);
    get_gui_dc_1018_4db0(local_struct_1.u32_xfa, local_struct_1.win_handle_0x8);
    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uStack10, 0x32));
    &local_struct_1.field_0x10e = ppVar1;
    (&local_struct_1.field_0x10e + 2) = (ppVar1 >> 0x10);
    fn_ptr_1 = (local_struct_1.field_0x10e + 4);
    lVar2 = (**fn_ptr_1)();
    process_struct_1000_179c(0x30, (lVar2 >> 0x10));
    if (lVar2 == 0) {
        local_struct_1.u32_xf6 = 0;
    } else {
        uVar3 = process_struct_1020_62e0(
            lVar2,
            CONCAT22(local_struct_1.win_handle_0x8, (lVar2 >> 0x10)),
        );
        &local_struct_1.u32_xf6 = uVar3;
        (&local_struct_1.u32_xf6 + 2) = (uVar3 >> 0x10);
    }
    gui_window_func_1020_536e(in_struct_1, 0, 0x3ffff);
    return;
}

pub fn destroy_win_1020_42f4(in_struct_1: *mut astruct_44) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut unaff_CS: u16;
    let mut HVar3: u16;

    uVar2 = (in_struct_1 >> 0x10);
    iVar1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x623c;
    (iVar1 + 2) = 0x1020;
    (iVar1 + 0xe2) = 0x62d8;
    (iVar1 + 0xe4) = 0x1020;
    HVar3 = unaff_CS;
    if ((iVar1 + 0x106) != 0) {
        HVar3 = offset;
        DestroyMenu16(unaff_CS);
    }
    DestroyCursor16(HVar3, (iVar1 + 0xf0));
    DestroyCursor16();
    process_struct_1020_808e(in_struct_1);
    return;
}

pub fn win_gui_fn_1020_434c(
    in_struct_1_1: *mut astruct_661,
    in_struct_1_2: *mut astruct_661,
    param_3: *mut u32,
    param_3: u16_00,
    param_5: u16,
    param_6: i32,
) {
    if (param_6 == 1) {
        set_capture_1020_6184(CONCAT22(in_struct_1_2, in_struct_1_1), param_5);
        return;
    }
    if (param_6 == 2) {
        gui_window_func_1020_536e(
            in_struct_1_1,
            in_struct_1_2,
            param_3,
            param_3_00,
            param_5,
            2,
        );
        return;
    }
    pass1_1008_68ea(
        in_struct_1_1,
        in_struct_1_2,
        param_3,
        param_3_00,
        param_5,
        param_6,
    );
    return;
}

pub fn bring_window_to_top_1020_2aae(param_1: *mut void, param_2: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1008_3e0e(param_1);
    BringWindowToTop16((param_1 + 8));
    pass1_1018_169e((param_1 + 0xf2), param_2);
    return;
}

pub fn win_fun_1020_2ae4(param_1: *mut u32, uparam_2: i32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut cVar3: u8;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let local_BX_14: *mut astruct_4;
    let mut uVar6: u16;
    let ppVar7: *mut pass1_struct_1;
    let w_param: WPARAM16;
    let mut iVar8: i32;
    let mut h_wnd: u16;
    let mut local_c: u16;
    let mut local_a: u16;

    if (param_2 != 0x129) {
        local_BX_14 = param_1;
        uVar6 = (param_1 >> 0x10);
        if (0x129 < param_2) {
            if (param_2 == 0x12a) {
                h_wnd = local_BX_14.field_0x8;
                w_param = 0xf012;
            } else {
                if (param_2 == 299) {
                    return;
                }
                if (param_2 == 300) {
                    h_wnd = local_BX_14.field_0x8;
                    w_param = 0xf020;
                } else {
                    if (param_2 == 0x12d) {
                        return;
                    }
                    if (param_2 != 0x12e) {
                        return;
                    }
                    h_wnd = local_BX_14.field_0x8;
                    w_param = 0xf060;
                }
            }
            PostMessage16(0, w_param, 0x112, h_wnd);
            return;
        }
        if (param_2 == 0xfb) {
            ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_c, 0x30));
            uVar4 = SUB42(ppVar7, 0);
            pass1_1010_375e(ppVar7);
            unsafe {
                ppcVar2 = (*param_1 + 0x14);
            }
            ppcVar2(
                0x1010,
                local_BX_14,
                (param_1 >> 0x10),
                1,
                uVar4,
                extraout_DX,
            );
            pass1_1010_375e(ppVar7);
            pass1_1018_181c(local_BX_14.field_0xf2, CONCAT22(extraout_DX_00, uVar4));
            return;
        }
        if (param_2 < 0xfc) {
            cVar3 = param_2;
            uVar5 = param_2 & 0xff00 | (cVar3 + 0x91);
            if ((cVar3 + 0x91) == 0) {
                mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1f8);
                WinHelp16(
                    0x2a,
                    1,
                    CONCAT22(extraout_DX_01, uVar5),
                    local_BX_14.field_0x8,
                );
                return;
            }
            if (cVar3 == 'r') {
                iVar8 = &local_BX_14.field_0xa;
                uVar4 = uVar6;
                ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(iVar8, 0x30));
                pass1_1010_3770(ppVar7, CONCAT22(uVar4, iVar8));
                pass1_1038_af40(_g_astruct_112_a, local_BX_14.field_0x8, 3);
                return;
            }
            if (cVar3 == 'u') {
                uVar1 = local_BX_14.field_0xf2;
                pass1_1018_0a76(uVar1, (uVar1 >> 0x10));
                InvalidateRect16(0, 0x0, 0);
                return;
            }
        }
    }
    return;
}

pub fn enable_menu_item_1020_2c2a() -> bool {
    let BVar1: bool;
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;
    let mut h_menu: u16;

    if (in_stack_0000000a != 0) {
        return in_stack_0000000a - 1;
    }
    EnableMenuItem16(0x400, 3, in_stack_0000000c);
    if (PTR_LOOP_1050_3960 < 2) {
        h_menu = 0x401;
    } else {
        h_menu = 0x400;
    }
    BVar1 = EnableMenuItem16(h_menu, 5, in_stack_0000000c);
    return BVar1;
}

pub fn call_draw_1020_2c72(param_1: *mut void) {
    draw_1020_30be((param_1 + 0xf6));
    return;
}

pub fn destroy_icon_1020_2c88(param_1: *mut astruct_652) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    DestroyIcon16((iVar4 + 0xc2));
    (iVar4 + 0xc2) = 0;
    (iVar4 + 8) = 0;
    puVar1 = (iVar4 + 0xf6);
    uVar2 = (iVar4 + 0xf8);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
        }
        (**ppcVar3)(offset, puVar1, uVar2, 1);
    }
    (iVar4 + 0xf6) = 0;
    pass1_1010_1dda(*(iVar4 + 0xf2));
    (iVar4 + 0xf2) = 0;
    return;
}

pub fn win_fn_1020_2cf0(param_1: *mut win_struct_42) {
    let ppcVar1: fn();
    let mut uVar2: i32;
    let piVar3: *mut u16;
    let mut uVar4: u16;
    let struct_a: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut iVar6: i32;
    let mut unaff_SI: u16;
    let mut uVar7: i32;
    let ppVar8: *mut pass1_struct_1;
    let mut uVar9: u32;
    let mut local_4: u16;

    create_win_1008_9760(param_1);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, (iVar6 + 0xfc)));
    uVar4 = (ppVar8 >> 0x10);
    (iVar6 + 0xf2) = ppVar8;
    (iVar6 + 0xf4) = uVar4;
    uVar2 = (iVar6 + 0xf2);
    (iVar6 + 0xe0) = uVar2;
    (iVar6 + 0xe2) = uVar4;
    LoadIcon16(
        0x1010,
        s_SITEICON_1050_428d,
        &g_alloc_addr_1050_1050,
        g_h_instance_1050_038c,
    );
    (iVar6 + 0xc2) = uVar2;
    ppcVar1 = ((iVar6 + 0xf2) + 0x30);
    (**ppcVar1)();
    paVar5 = struct_a;
    process_struct_1000_179c(0x22, struct_a);
    if ((paVar5 | uVar2) == 0) {
        (iVar6 + 0xf6) = 0;
    } else {
        gui_win_fn_1020_2ede(uVar2, paVar5, param_1);
        (iVar6 + 0xf6) = uVar2;
        (iVar6 + 0xf8) = extraout_DX;
    }
    (iVar6 + 0xe8) = (iVar6 + 0xf6);
    pass1_1018_0ac0((iVar6 + 0xf2), param_1 & 0xffff | uVar7 << 0x10);
    uVar9 = pass1_1018_0b08((iVar6 + 0xf2));
    piVar3 = uVar9;
    ppcVar1 = (param_1 + 0x14);
    (**ppcVar1)();
    ppcVar1 = ((iVar6 + 0xf2) + 0x10);
    (**ppcVar1)();
    MoveWindow16(
        1,
        piVar3[3],
        piVar3[2],
        piVar3[1],
        unsafe { *piVar3 },
        (iVar6 + 8),
    );
    pass1_1008_3e0e(param_1);
    UpdateWindow16((iVar6 + 8));
    return;
}

pub fn call_cleanup_fn_1020_2e24(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376 {
    cleanup_fn_1020_28fc(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn gui_win_fn_1020_2ede(param_1: *mut u16, param_2: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let HVar3: HDC16;
    let mut iVar4: i32;
    let obj_handle: HPEN16;
    let HVar5: HGDIOBJ16;
    let ppVar6: *mut pass1_struct_1;
    let in_struct_104_ptr: *mut astruct_104;
    let mut uVar7: u32;
    let uVar8: u8;
    let uVar9: u8;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let uVar13: u8;
    let uVar14: u8;
    let mut iVar15: i32;
    let mut uVar16: u16;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar10 = (param_2 >> 0x10);
    iVar15 = param_1;
    uVar16 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(iVar15, uVar16, param_2, uVar10);
    (iVar15 + 0x14) = 0;
    (iVar15 + 0x18) = 0;
    (iVar15 + 0x1a) = 0;
    (iVar15 + 0x1c) = 0;
    (iVar15 + 0x1e) = 0;
    (iVar15 + 0x20) = 0;
    unsafe {
        *param_1 = 0x363c;
    }
    (iVar15 + 2) = 0x1020;
    ppVar6 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffea, (param_2 + 0xfc)),
    );
    (iVar15 + 0x14) = ppVar6;
    (iVar15 + 0x16) = (ppVar6 >> 0x10);
    uVar1 = (iVar15 + 0x14);
    ppcVar2 = ((iVar15 + 0x14) + 4);
    ppcVar2(0x1010, uVar1, (uVar1 >> 0x10), 0, iVar15, uVar16);
    uVar13 = 0xc2;
    uVar14 = 0x42;
    uVar11 = 0;
    uVar12 = 0;
    uVar8 = 0;
    uVar9 = 0;
    uVar10 = 0;
    in_struct_104_ptr = pass1_1018_0a50((iVar15 + 0x14));
    uVar7 = process_struct_1008_4772(in_struct_104_ptr);
    HVar3 = CreateDC16(
        uVar7,
        CONCAT13(uVar9, CONCAT12(uVar8, (uVar7 >> 0x10))),
        CONCAT22(uVar11, uVar10),
        CONCAT13(uVar14, CONCAT12(uVar13, uVar12)),
    );
    (iVar15 + 0x18) = HVar3;
    iVar4 = iVar15 + 0x18;
    ppcVar2 = (in_struct_104_ptr + 8);
    ppcVar2(
        offset,
        in_struct_104_ptr,
        (in_struct_104_ptr >> 0x10),
        iVar4,
        uVar16,
    );
    (iVar15 + 0x20) = iVar4;
    uVar1 = (iVar15 + 0x14);
    obj_handle = CreatePen16((uVar1 + 100), 1, 0);
    (iVar15 + 0x1a) = obj_handle;
    HVar5 = SelectObject16(obj_handle, (iVar15 + 0x18));
    (iVar15 + 0x1c) = HVar5;
    HVar5 = GetStockObject16(5);
    HVar5 = SelectObject16(HVar5, (iVar15 + 0x18));
    (iVar15 + 0x1e) = HVar5;
    return;
}

pub fn win_cleanup_func_1020_2fea(param_1: *mut astruct_44) {
    let h_gdi_obj: HPALETTE16;
    let local_BX_4: *mut astruct_44;
    let mut in_stack_00000006: u16;

    *_param_1 = 0x363c;
    param_1.ptr_a_hi = 0x1020;
    if (param_1.field_0x14 != 0) {
        pass1_1010_1ea6(param_1.field_0x14, CONCAT22(in_stack_00000006, param_1));
    }
    SelectObject16(param_1.field_0x1c, param_1.field_0x18);
    SelectObject16(param_1.field_0x1e, param_1.field_0x18);
    DeleteObject16(param_1.field_0x1a);
    h_gdi_obj = SelectPalette16(0, param_1.field_0x20, param_1.field_0x18);
    DeleteObject16(h_gdi_obj);
    DeleteDC16(param_1.field_0x18);
    *_param_1 = s_0_020_1050_3ab0;
    param_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    *_param_1 = s_1_1050_389a;
    param_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn invalidate_rect_1020_3080(param_1: u32, param_2: i32) {
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if ((param_2 != 4) && (param_2 != 6)) {
        return;
    }
    InvalidateRect16(0, 0x0, 0);
    return;
}

pub fn draw_1020_30be(in_struct_1: *mut astruct_653) {
    let paVar1: *mut astruct_510;
    let is_iconic_result: bool;
    let local_struct_1: *mut astruct_653;
    let local_struct_1_hi: *mut astruct_653;
    let unaff_SS: HWND16;
    let local_struct_2_16: *mut astruct_510;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let local_struct_2_16_2: *mut astruct_510;
    let local_paint_struct_1: PAINTSTRUCT16;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_2_16_2 = BeginPaint16(
        CONCAT22(unaff_SS, &local_paint_struct_1),
        local_struct_1.win_handle_0x4,
    );
    local_struct_2_16 = local_struct_1.win_handle_0x4;
    is_iconic_result = IsIconic16(local_struct_2_16);
    if (is_iconic_result == 0) {
        paVar1 = local_struct_1.struct_ptr_0x14;
        local_struct_2_16 = paVar1;
        local_3a = (paVar1 >> 0x10);
        pass1_1018_0a50(paVar1);
        local_struct_2_16 = &local_struct_2_16_2;
        fn_ptr_1 = (CONCAT22(unaff_SS, local_struct_2_16) + 8);
        local_3a = unaff_SS;
        (**fn_ptr_1)(0x1018, local_struct_2_16, unaff_SS);
        paVar1 = local_struct_1.struct_ptr_0x14;
        if ((paVar1 + 0x84) == 1) {
            draw_1020_320e(in_struct_1, local_struct_2_16_2);
        }
        fn_ptr_1 = (CONCAT22(local_3a, local_struct_2_16) + 4);
        (**fn_ptr_1)(0x1018, local_struct_2_16, local_3a, 0, 0, 0xdc);
        paVar1 = local_struct_1.struct_ptr_0x14;
        if ((paVar1 + 0x84) != 1) {
            draw_1020_320e(in_struct_1, local_struct_2_16_2);
        }
        draw_rect_1020_3488(in_struct_1, local_struct_2_16_2);
        fn_ptr_1 = (CONCAT22(local_3a, local_struct_2_16) + 0xc);
        (**fn_ptr_1)(0x1018, local_struct_2_16, local_3a, &local_struct_2_16_2);
    } else {
        if (PTR_LOOP_1050_0010 == 0x0) {
            paVar1 = local_struct_1.struct_ptr_0x14;
            local_struct_2_16 = paVar1;
            local_3a = (paVar1 >> 0x10);
            fn_ptr_1 = (local_struct_1.struct_ptr_0x14 + 0x2c);
            local_26 = (**fn_ptr_1)(offset);
            if (local_26 != 0) {
                local_3a = 4;
                local_struct_2_16 = offset;
                local_28 = GetStockObject16(4);
                local_38 = local_struct_1.win_handle_0x4;
                local_struct_2_16 = &local_30;
                GetClientRect16(CONCAT22(unaff_SS, local_struct_2_16), local_38);
                local_32 = (local_2c - local_30) - 1;
                local_34 = (local_2a - local_2e) - 1;
                local_36 = local_struct_2_16_2;
                local_3a = &local_struct_2_16;
                local_struct_2_16 = local_28;
                FillRect16(local_28, local_3a, unaff_SS);
                local_36 = local_struct_2_16_2;
                local_3a = 2;
                local_38 = 2;
                local_struct_2_16 = local_26;
                DrawIcon16(
                    local_26,
                    CONCAT214(
                        local_2c,
                        CONCAT212(
                            local_2e,
                            CONCAT210(
                                local_30,
                                CONCAT28(
                                    local_32,
                                    CONCAT26(local_34, CONCAT24(local_struct_2_16_2, 0x20002)),
                                ),
                            ),
                        ),
                    ),
                    CONCAT88(
                        local_paint_struct_1._0_8_,
                        CONCAT26(
                            local_struct_2_16_2,
                            CONCAT24(local_26, CONCAT22(local_28, local_2a)),
                        ),
                    ),
                    local_paint_struct_1.rc_print.right,
                );
            }
        }
    }
    local_3a = local_struct_1.win_handle_0x4;
    EndPaint(&local_paint_struct_1, unaff_SS);
    return;
}

pub fn draw_1020_320e(param_1: *mut astruct_653, param_2: u16) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let h_gdi_obj: HPALETTE16;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut uVar8: u32;
    let uVar9: u8;
    let uVar10: u8;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_2;
    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar3 = (iVar4 + 0x14);
    if ((uVar3 + 0x84) == 1) {
        uVar3 = (iVar4 + 0x14);
        uVar7 = (uVar3 >> 0x10);
        iVar5 = uVar3;
        puVar1 = (iVar5 + 0x24);
        uVar14 = SUB42(s_dib_1050_42c6, 0);
        uVar12 = 0;
        uVar13 = 0;
        uVar9 = 0;
        uVar10 = 0;
        uVar11 = 0;
        uVar8 = process_struct_1008_4772((puVar1 & 0xffff | *(iVar5 + 0x26) << 0x10));
        local_4 = CreateDC16(
            uVar8,
            CONCAT13(uVar10, CONCAT12(uVar9, (uVar8 >> 0x10))),
            CONCAT22(uVar12, uVar11),
            CONCAT22(uVar14, uVar13),
        );
        local_6 = &local_4;
        unsafe {
            ppcVar2 = (*puVar1 + 8);
        }
        ppcVar2(offset, puVar1, (puVar1 >> 0x10), local_6);
    }
    pass1_1018_0d9a(
        (iVar4 + 0x14),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    uVar3 = (iVar4 + 0x14);
    uVar3 = (uVar3 + 0x6c);
    draw_1020_33c0(
        param_1,
        uVar3,
        (uVar3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        1,
        local_4,
    );
    pass1_1018_1054(
        (iVar4 + 0x14),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    uVar3 = (iVar4 + 0x14);
    uVar3 = (uVar3 + 0x74);
    draw_1020_33c0(
        param_1,
        uVar3,
        (uVar3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        2,
        local_4,
    );
    pass1_1018_1320(
        (iVar4 + 0x14),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    uVar3 = (iVar4 + 0x14);
    uVar3 = (uVar3 + 0x68);
    draw_1020_33c0(
        param_1,
        uVar3,
        (uVar3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        1,
        local_4,
    );
    pass1_1018_15f6(
        (iVar4 + 0x14),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    if (local_c != 0) {
        uVar3 = (iVar4 + 0x14);
        uVar3 = (uVar3 + 0x70);
        draw_1020_33c0(
            param_1,
            uVar3,
            (uVar3 >> 0x10),
            local_c,
            local_a,
            (local_a >> 0x10),
            1,
            local_4,
        );
    }
    pass1_1018_108c(
        (iVar4 + 0x14),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    if (local_c != 0) {
        uVar3 = (iVar4 + 0x14);
        uVar3 = (uVar3 + 0x78);
        draw_1020_33c0(
            param_1,
            uVar3,
            (uVar3 >> 0x10),
            local_c,
            local_a,
            (local_a >> 0x10),
            0,
            local_4,
        );
    }
    uVar3 = (iVar4 + 0x14);
    if ((uVar3 + 0x84) == 1) {
        h_gdi_obj = SelectPalette16(0, local_6, local_4);
        DeleteObject16(h_gdi_obj);
        DeleteDC16(local_4);
    }
    return;
}

pub fn draw_1020_33c0(
    param_1: u32,
    color_ref: COLORREF,
    param_3: i32,
    param_4: u32,
    param_5: i32,
    in_dc_handle: HDC16,
) {
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let mut solid_brush: u16;
    let mut solid_brush_obj_handle: u16;
    let mut iVar1: i32;
    let mut uVar2: u16;
    let h_gdi_obj: HGDIOBJ16;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let HVar4: HDC16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3 != 0) {
        HVar4 = in_dc_handle;
        pen = CreatePen16(color_ref, 1, 0);
        pen_obj_handle = SelectObject16(pen, HVar4);
        HVar4 = in_dc_handle;
        solid_brush = CreateSolidBrush16(color_ref);
        solid_brush_obj_handle = SelectObject16(solid_brush, HVar4);
        local_e = param_4;
        local_14 = 0;
        while (local_14 < param_3) {
            uVar3 = (param_1 >> 0x10);
            iVar1 = param_3;
            HVar4 = in_dc_handle;
            pass1_1020_3540(param_1, uVar3, param_5, local_e);
            if (param_5 < 1) {
                uVar2 = 3;
            } else {
                uVar2 = 4;
            }
            polygon_1020_3602(param_1, uVar3, uVar2, iVar1, extraout_DX, HVar4);
            error_check_1000_17ce(CONCAT22(extraout_DX, iVar1));
            local_14 = local_14 + 1;
            local_e = local_e & 0xffff0000 | (local_e + 6);
        }
        h_gdi_obj = SelectObject16(solid_brush_obj_handle, in_dc_handle);
        DeleteObject16(h_gdi_obj);
        SelectObject16(pen_obj_handle, in_dc_handle);
        DeleteObject16(pen);
    }
    return;
}

pub fn draw_rect_1020_3488(param_1: *mut astruct_653, in_h_dc: u16) {
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let paVar4: *mut astruct_199;
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let mut stock_obj_5: u16;
    let obj_handle: HGDIOBJ16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut h_dc: u16;
    let mut left: i32;
    let mut h_dc_2: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar5 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x14);
    paVar4 = (uVar2 + 0x36);
    local_6 = uVar2 & 0xffff0000 | ZEXT24(paVar4);
    pass1_1008_3e94(
        paVar4,
        CONCAT22(unaff_SS, &local_a),
        CONCAT22(unaff_SS, &local_8),
    );
    uVar2 = (local_8 - 3) << 0x10;
    if ((local_8 - 3) < 0) {
        uVar2 = 0;
    }
    uVar1 = _local_a - 3;
    _local_a = uVar1;
    if (uVar1 < 0) {
        _local_a = 0;
    }
    _local_a = uVar2 | _local_a;
    uVar3 = (param_1 + 0x14);
    h_dc = in_h_dc;
    pen = CreatePen16((uVar3 + 100), 1, 0);
    pen_obj_handle = SelectObject16(pen, h_dc);
    stock_obj_5 = GetStockObject16(5);
    obj_handle = SelectObject16(stock_obj_5, h_dc_2);
    left = (_local_a >> 0x10);
    Rectangel16(_local_a + 6, left + 6, _local_a, left, in_h_dc);
    SelectObject16(pen_obj_handle, in_h_dc);
    SelectObject16(obj_handle, in_h_dc);
    DeleteObject16(pen);
    return;
}

pub fn palette_func_1020_2992(param_1: u32, param_2: i32) {
    let ppcVar1: fn();
    let obj: HGDIOBJ16;
    let hdc: HDC16;
    let mut uVar2: u16;
    let puVar3: *mut u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != 0) {
        uVar2 = (param_1 >> 0x10);
        puVar3 = pass1_1018_0a50((param_1 + 0xf2));
        unsafe {
            ppcVar1 = (*puVar3 + 0x18);
        }
        obj = (**ppcVar1)(0x1018);
        UnrealizeObject16(obj);
        hdc = GetDC16((param_1 + 8));
        RealizePalette16(hdc);
    }
    return;
}

pub fn send_win_msg_1020_29d8(param_1: i32, param_2: u16, param_3: u16_00, param_3: u32) {
    let mut u_var1: u32;
    let ppVar2: *mut pass1_struct_1;
    let mut iVar3: i32;
    let mut in_stack_0000fff6: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar3 = (param_3 >> 0x10);
    uVar1 = post_win_msg_1020_79fc(param_1, param_2, param_3_00, param_3, iVar3);
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff6, 0x29));
    if (iVar3 == 0) {
        pass1_1018_270e(ppVar2, 1, (param_1 + 0xfc));
    } else {
        pass1_1018_270e(ppVar2, 0, (param_1 + 0xfc));
        SendMessage16(0, 0x69, 0x111, g_h_window);
    }
    return uVar1;
}

pub fn win_gui_fn_1020_2a46(in_struct_1: *mut astruct_628, param_2: u16, param_3: u16) {
    pass1_1018_0ae8(in_struct_1.field_0xf2, 1);
    show_window_1008_68c6(in_struct_1, param_2, param_3);
    return;
}

pub fn cleanup_fn_1020_2a6a(in_struct_1: *mut astruct_652) {
    let local_struct_1: *mut astruct_652;
    let local_struct_1_hi: *mut astruct_652;

    get_sys_metrics_1020_7a50(in_struct_1);
    pass1_1018_0ae8((in_struct_1 + 0xf2), 0);
    destroy_icon_1020_2c88(in_struct_1);
    return;
}

pub fn pass1_1020_289a(param_1: *mut win_struct_42, param_2: u16, param_3: u32) {
    let mut iVar1: i32;
    let local_struct_1_hi: *mut win_struct_42;

    call_load_cursor_1020_790e(param_1, s_SCPOPMENU_1050_427c, param_2, param_3);
    local_struct_1_hi = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xf2) = 0;
    (iVar1 + 0xf6) = 0;
    (iVar1 + 0xfa) = 0;
    (iVar1 + 0xfc) = 0;
    param_1.u16_x0 = (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0x15);
    (iVar1 + 2) = 0x1020;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (iVar1 + 0x5b)), s_VrMode_1050_4286);
    (iVar1 + 0xac) = 0x44c00000;
    return;
}

pub fn win_gui_fn_1020_2642(in_struct_1: *mut win_struct_42) {
    let paVar1: *mut astruct_650;
    let struct_a: *mut astruct_199;
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let paVar2: *mut astruct_199;
    let mut uVar3: u32;
    let mut local_4: u16;

    paVar2 = create_win_1008_9760(in_struct_1);
    struct_a = (paVar2 >> 0x10);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    if ((struct_a | paVar1) != 0) {
        uVar3 = call_draw_fn_1020_27b0(paVar1, CONCAT22(local_struct_1.win_handle_0x8, struct_a));
        local_struct_1.char_ptr_16_0xee = uVar3;
        local_struct_1.field_0xf0 = (uVar3 >> 0x10);
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub fn call_fn_ptr_1020_26a6(in_struct_1: *mut astruct_594) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut in_stack_0000fff6: u16;
    let fn_ptr_1: fn();

    uVar3 = (in_struct_1 >> 0x10);
    puVar1 = (in_struct_1 + 0xee);
    uVar2 = (in_struct_1 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
        }
        (**fn_ptr_1)();
    }
    destroy_win_1008_628e(in_struct_1, in_stack_0000fff6);
    return;
}

pub fn call_load_cursor_1020_2524(in_struct_1: *mut astruct_65, param_2: u16, param_3: u32) {
    let mut local_AX_24: u16;
    let mut local_DX_71: u16;
    let mut iVar1: i32;
    let mut local_BP__1: u16;
    let mut local_ES_21: u16;
    let ppVar2: *mut pass1_struct_1;

    load_cursor_1020_7f7a(in_struct_1, CONCAT22(param_2, 7), param_3);
    local_ES_21 = (in_struct_1 >> 0x10);
    iVar1 = in_struct_1;
    (iVar1 + 0xee) = 0;
    (iVar1 + 0xf2) = 0;
    in_struct_1.ptr_a_lo = s_fem36_wav_1050_270c;
    (iVar1 + 2) = 0x1020;
    (iVar1 + 0xe2) = (s_fem51_wav_1050_27a2 + 6);
    (iVar1 + 0xe4) = 0x1020;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_BP__1, 0x2a));
    local_DX_71 = (ppVar2 >> 0x10);
    (iVar1 + 0xf2) = ppVar2;
    (iVar1 + 0xf4) = local_DX_71;
    (iVar1 + 0xe6) = (iVar1 + 0xf2);
    (iVar1 + 0xe8) = local_DX_71;
    return;
}

pub fn win_gui_fn_1020_3644(in_struct_1: *mut win_struct_42, param_2: u16, param_3: u32) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;

    call_load_cursor_1020_790e(in_struct_1, 0x0, param_2, param_3);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.u32_xf2 = s_1_1050_389a;
    local_struct_1.field_0xf4 = &PTR_LOOP_1050_1008;
    local_struct_1.u32_xf2 = (s_18_2_1050_3aa5 + 3);
    local_struct_1.field_0xf4 = &PTR_LOOP_1050_1008;
    local_struct_1.field_0x100 = 0;
    local_struct_1.field_0x10a = 0;
    local_struct_1.field_0x10e = 0;
    in_struct_1.u16_x0 = 0x3d08;
    local_struct_1.u16_x2 = 0x1020;
    local_struct_1.u32_xf2 = 0x3d9c;
    local_struct_1.field_0xf4 = 0x1020;
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.u32_0xa)),
        0x5e9,
    );
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.class_name_0x5b)),
        s_VrMode_1050_42ca,
    );
    local_struct_1.style_0xac = 0x44c00000;
    set_window_pos_1020_38aa(local_struct_1, local_struct_1_hi);
    return;
}

pub fn win_fn_1020_36f6(param_1: u32, param_2: i32) {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let puVar4: *mut u8;
    let mut HVar5: HWND16;
    let mut HVar6: HWND16;
    let mut uVar7: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut uVar10: u16;
    let uVar11: u8;
    let uVar12: u8;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: [u8; 1024];
    let mut local_6: u16;
    let mut local_4: u16;

    iVar8 = param_1;
    uVar9 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar8 + 8) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    uVar7 = pass1_1018_1e78((iVar8 + 8), 0xffff);
    _local_6 = (uVar7 & 0xffff | in_DX << 0x10);
    uVar3 = (iVar8 + 0x18);
    GetWindowText16(0x3ff, CONCAT22(unaff_SS, local_406), (uVar3 + 6));
    puVar4 = local_406;
    process_string_1000_472c(CONCAT22(unaff_SS, puVar4), ':');
    _local_40a = CONCAT22(in_DX, puVar4 + 2);
    *_local_40a = 0;
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_406),
        *_local_6,
    );
    uVar3 = (iVar8 + 0x18);
    ppcVar2 = ((iVar8 + 0x18) + 0x18);
    ppcVar2(0x1010, uVar3, (uVar3 >> 0x10), local_406);
    uVar3 = (iVar8 + 8);
    iVar1 = (uVar3 + 0x4a);
    uVar3 = (iVar8 + 0x18);
    HVar6 = (uVar3 + 6);
    HVar5 = HVar6;
    SetDlgItemText16((_local_6 + 2), 0x10f, HVar6);
    SetDlgItemText16((_local_6 + 10), 0x110, HVar6);
    SetDlgItemText16((_local_6 + 0x12), 0x112, HVar6);
    SetDlgItemText16((_local_6 + 0xe), 0x113, HVar6);
    if (iVar1 != 0) {
        HVar5 = pass1_1018_1f1a((iVar8 + 8), (_local_6 + 0x1a));
        if (HVar5 != 0) {
            uVar11 = 0x11;
            uVar12 = 1;
            uVar3 = (_local_6 + 0x16);
            HVar5 = uVar3;
            uVar10 = (uVar3 >> 0x10);
            // goto LAB_1020_3846;
        }
    }
    uVar11 = 0x11;
    uVar12 = 1;
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x5d9,
    );
    uVar10 = extraout_DX;
    // LAB_1020_3846:
    SetDlgItemText16(CONCAT22(uVar10, HVar5), CONCAT11(uVar12, uVar11), HVar6);
    if ((iVar8 + 0x1c) != 0) {
        uVar3 = (iVar8 + 0x1c);
        HVar6 = GetDlgItem16((_local_6 + 0x1a), (uVar3 + 6));
        SetFocus16(HVar6);
        return;
    }
    InvalidateRect16(0, 0x0, 0);
    return;
}
