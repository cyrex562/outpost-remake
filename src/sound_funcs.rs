pub fn sound_fn_1008_53ae(param_1: u32, param_2: u16) {
    let mut unaff_SS: u16;
    let dVar1: u32;
    let mut local_432: [u8; 1024];
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    local_1e = 0;
    local_16 = 0x28c;
    local_14 = &g_alloc_addr_1050_1050;
    local_12 = param_1;
    local_e = 0;
    local_a = 0;
    local_8 = 0x4000;
    local_6 = param_2;
    dVar1 = mciSendCommand16(CONCAT22(unaff_SS, &local_1e), 0x30200, 0x803, 0);
    local_20 = (dVar1 >> 0x10);
    local_22 = dVar1;
    if (local_1a != 0) {
        if ((local_20 | local_22) != 0) {
            mciGetErrorString16(offset, 0x400, local_432);
        }
        pass1_1000_4906(CONCAT22(unaff_SS, &local_2e), 0, 0xc);
        local_2e = param_2;
        local_2c = 0;
        dVar1 = mciSendCommand16(CONCAT22(unaff_SS, &local_2e), 2, 0x806, local_1a);
        local_20 = (dVar1 >> 0x10);
        local_22 = dVar1;
        if ((local_20 | local_22) != 0) {
            mciGetErrorString16(offset, 0x400, local_432);
        }
        local_32 = param_2;
        local_30 = 0;
        dVar1 = mciSendCommand16(CONCAT22(unaff_SS, &local_32), 1, 0x804, local_1a);
        local_20 = (dVar1 >> 0x10);
        local_22 = dVar1;
        if ((local_20 | local_22) != 0) {
            mciGetErrorString16(offset, 0x400, local_432);
        }
    }
    return;
}

pub fn mci_send_cmd_1008_5c5c(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let mut local_DXAX_12: u32;

    uVar1 = pass1_1010_84f8(_g_struct_73_1050_14cc, param_2);
    mci_send_command_1008_5cfe(param_1, uVar1);
    return;
}

pub fn mci_send_command_1008_5c7c(in_char_buff: *mut u8, in_struct_1: *mut Struct13) {
    let mut u_var1: u32;

    uVar1 = set_error_mode_1010_85be(_g_struct_73_1050_14cc, in_struct_1, (in_struct_1 >> 0x10));
    mci_send_command_1008_5cfe(in_char_buff, uVar1);
    return;
}

pub fn mci_send_command_1008_5c9e(in_char_buff: *mut u8, in_struct_1: *mut Struct13) {
    mci_send_command_1008_5c7c(in_char_buff, in_struct_1);
    return;
}

pub fn mci_send_command_1008_5cb6(param_1: *mut Struct318, dev_id: u16) {
    let local_BX_23: *mut Struct75;
    let mut uVar1: u16;
    let mut uVar2: u16;

    mciSendCommand16(0, 0, 0x804, dev_id);
    uVar1 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    if ((local_BX_23.field_0xa == 0) || (local_BX_23.field_0xa != dev_id)) {
        local_BX_23.field_0x12 = 0;
        uVar2 = 0x11;
    } else {
        local_BX_23.field_0x10 = 0;
        uVar2 = 0x10;
    }
    pass1_1010_1f62(param_1, uVar2);
    return;
}

// WARNING: Variable defined which should be unmapped: local_132

// WARNING: Could not reconcile some variable overlaps

pub fn mci_send_command_1008_5cfe(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_ES_31: u16;
    let mut local_SS__1: u16;
    let dVar4: u32;
    let mut local_132: u16;
    let mut local_12c: u16;
    let mut local_12a: u16;
    let mut local_120: u16;
    let local_11e: u8;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut temp_5f24728895: u32;

    pass1_1000_4906(CONCAT22(local_SS__1, local_16), 0, 0x14);
    local_a = param_2;
    local_ES_31 = (param_1 >> 0x10);
    iVar3 = param_1;
    temp_5f24728895 = (iVar3 + 0xc);
    local_18 = (temp_5f24728895 + 0x72);
    local_1a = 1;
    _local_1e = s_waveaudio_1050_02a4;
    process_string_1000_4d58(param_2, 0, 0, 0, 0, &local_11e);
    iVar2 = process_string_1000_475e(CONCAT22(local_SS__1, &local_11e), s__mid_1050_02ae);
    if (iVar2 == 0) {
        uVar1 = (iVar3 + 0xc);
        local_18 = (uVar1 + 0x74);
        _local_1e = s_sequencer_1050_02b3;
        local_1a = 0;
    }
    if (local_18 != 0) {
        if ((local_1a != 0) && ((iVar3 + 0x10) != 0)) {
            return;
        }
        if ((local_1a == 0) && ((iVar3 + 0x12) != 0)) {
            return;
        }
        local_e = _local_1e;
        dVar4 = mciSendCommand16(CONCAT22(local_SS__1, local_16), 0x2200, 0x803, 0);
        local_120 = dVar4;
        if (((dVar4 >> 0x10) | local_120) == 0) {
            if (local_1a == 0) {
                (iVar3 + 0x12) = 1;
            } else {
                (iVar3 + 10) = local_12;
                (iVar3 + 0x10) = 1;
                local_120 = local_12;
            }
            create_win_1008_5e7e(param_1);
            if (local_120 == 0) {
                mci_send_command_1008_5cb6(param_1, local_12);
                return;
            }
            pass1_1000_4906(CONCAT22(local_SS__1, &local_12c), 0, 0xc);
            local_12c = local_120;
            local_12a = 0;
            mciSendCommand16(CONCAT22(local_SS__1, &local_12c), 1, 0x806, local_12);
            SetWindowWord16(0x38, local_12, 0, local_120);
            return;
        }
    }
    if (local_1a == 0) {
        local_132 = 0x11;
    } else {
        local_132 = 0x10;
    }
    pass1_1010_1f62(param_1, local_132);
    return;
}

pub fn mci_fn_1018_e2cc(in_Struct620_ptr_1: *mut Struct620) {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let struct_a: *mut Struct199;
    let paVar4: *mut Struct199;
    let mut extraout_DX: i32;
    let local_Struct620_ptr_1: *mut Struct620;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 4];
    let fn_ptr_1: fn();

    uVar5 = (in_Struct620_ptr_1 >> 0x10);
    local_Struct620_ptr_1 = in_Struct620_ptr_1;
    if (local_Struct620_ptr_1.field_0xee != 0) {
        fn_ptr_1 = (local_Struct620_ptr_1.field_0xee + 8);
        (**fn_ptr_1)();
    }
    if (local_Struct620_ptr_1.field_0xea == 0) {
        local_Struct620_ptr_1.field_0xea = 1;
        pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x7a);
        uVar3 = ZEXT24(local_6);
        mci_send_command_1008_5c9e(_g_struct_ptr_1050_02a0, CONCAT22(unaff_SS, local_6));
        local_Struct620_ptr_1.field_0xec = uVar3;
        paVar4 = struct_a;
        process_struct_1000_179c(0x112, struct_a);
        if ((paVar4 | uVar3) == 0) {
            uVar2 = 0;
            _local_a = 0x0;
        } else {
            piVar1 = &local_Struct620_ptr_1.field_0xcc;
            unsafe {
                *piVar1 = *piVar1 + 1;
            }
            win_gui_fn_1020_3644(
                uVar3,
                paVar4,
                local_Struct620_ptr_1.field_0xcc,
                in_Struct620_ptr_1,
            );
            uVar2 = uVar3;
            _local_a = (uVar3 & 0xffff | extraout_DX << 0x10);
        }
        pass1_1008_6978(in_Struct620_ptr_1, 0, _local_a & 0xffff0000 | uVar2);
        fn_ptr_1 = (*_local_a + 0xc);
        (**fn_ptr_1)(8, _local_a, local_8, 5);
    }
    return;
}

pub fn win_and_mci_fn_1018_ea66(param_1: *mut Struct626) {
    let mut local_AX_77: u16;
    let local_struct_1: *mut Struct626;
    let mut local_struct_1_hi: u16;
    let mut local_SS__1: u16;
    let mut char_buf_1: [u8; 4];
    // fn_ptr_1: *mut *mut u8;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0xee != 0) {
        fn_ptr_1 = (local_struct_1.field_0xee + 8);
        (**fn_ptr_1)();
    }
    if (local_struct_1.bool_xEA == 0) {
        local_struct_1.bool_xEA = 1;
        pass1_1008_941a(CONCAT22(local_SS__1, char_buf_1), 1, 0x95);
        local_AX_77 =
            mci_send_command_1008_5c9e(_g_struct_ptr_1050_02a0, CONCAT22(local_SS__1, char_buf_1));
        local_struct_1.field_0xec = local_AX_77;
        window_msg_func_1010_7300(local_struct_1.field_0xf6, 0, 0x80000, 0);
    }
    return;
}

pub fn mci_cmd_fn_1020_07aa(in_struct_1: *mut Struct13) {
    let local_struct_1: *mut Struct13;
    let local_struct_1_hi: *mut Struct13;
    let mut unaff_SS: u16;
    let mut char_buff_1: [u8; 20];

    call_fill_rect_1020_041e(in_struct_1);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0xf0 == 0) {
        local_struct_1.field_0xf0 = 1;
        modify_u16_list_1008_5bdc(CONCAT22(unaff_SS, char_buff_1));
        mci_send_command_1008_5c9e(
            CONCAT22(unaff_SS, char_buff_1),
            (in_struct_1 & 0xffff0000 | ZEXT24(local_struct_1 + 1)),
        );
        modify_u16_list_1008_5c34(CONCAT22(unaff_SS, char_buff_1));
    }
    return;
}

pub fn mci_fn_1020_08b6(param_1: *mut Struct65, param_2: u16, param_3: *mut void) {
    let mut iVar1: i32;
    let mut local_ES_21: u16;

    load_cursor_1008_61b2(param_1, 1, param_2, param_3);
    local_ES_21 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xe6) = 0;
    (iVar1 + 0xe8) = 0;
    param_1.ptr_a_lo = 0xb0e;
    (iVar1 + 2) = 0x1020;
    mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1d4);
    return;
}

pub fn sound_fn_1040_8978(param_1: *mut u32, param_2: u16) {
    let fn_ptr_1: fn();

    process_win_msg_1008_9510();
    mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, param_2);
    unsafe {
        fn_ptr_1 = (*param_1 + 0x74);
        (**fn_ptr_1)(&PTR_LOOP_1050_1008, param_1);
    }
    return;
}
