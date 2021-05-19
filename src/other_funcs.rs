use crate::defines::Struct1;

pub fn return_one_1000_2146() -> u16 {
    return 1;
}

pub fn return_1000_214a() {
    return;
}

pub fn xor_1000_49b2(param_1: i32) -> i32 {
    return (param_1 ^ param_1 >> 0xf) - (param_1 >> 0xf);
}

pub fn empty_fn_1000_55ac() {
    return;
}

pub fn zero_list_1008_3e38(param_1: *mut Struct1) -> u8 {
    let mut local_es_5: u16;

    local_es_5 = (param_1 >> 0x10);
    param_1.field_0x0 = 0;
    (param_1 + 2) = 0;
    (param_1 + 4) = 0;
    return param_1;
}

pub fn modify_list_1008_3f62(param_1: *mut u16, param_2: u32) {
    let mut local_es_15: u16;
    let mut local_es_22: u16;

    unsafe { *param_1 = param_2 };
    local_es_15 = (param_2 >> 0x10);
    local_es_22 = (param_1 >> 0x10);
    (param_1 + 2) = (param_2 + 2);
    (param_1 + 4) = (param_2 + 4);
    return;
}

pub fn zero_array_val_1008_5394(param_1: *mut u32) {
    unsafe { *param_1 = 0 };
    return param_1;
}

pub fn return_1008_53aa(a: u16, b: u16) {
    return;
}

pub fn set_param_3_with_switch_1008_72bc(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    if (PTR_LOOP_1050_0312 < 2) {
        if param_1 > 0x13 {
            return 0;
        }
    }
    return param_1_00;
}

pub fn switch_statement_1008_738c(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let mut local_AX_88: u16;

    match (param_1_00) {
        1 => {
            local_AX_88 = 3;
        }
        2 => {
            local_AX_88 = 4;
        }
        3 => return 5,
        4 => return 6,
        5 => return 8,
        6 => return 9,
        7 => return 10,
        _ => local_AX_88 = 0,
    }
    return local_AX_88;
}

pub fn switch_statement_1008_73ea(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let mut local_4: u16;

    local_4 = param_1_00;
    if (PTR_LOOP_1050_0312 < 2) {
        if param_1 >= 0x18 || param_1 <= 0x3e {
            local_4 = param_1_00 + 3;
        } else if param_1 >= 0x3f || param_1 <= 0x66 {
            local_4 = param_1_00 + 4;
        } else if param_1 >= 0x67 || param_1 <= 0x80 {
            local_4 = param_1_00 + 9;
        }
    }
    return local_4;
}

pub fn set_timer_1008_91ba(param_1: *mut u16) -> *mut u16 {
    let mut u_var1: u16;
    let local_bx_4: *mut Struct76;
    let pu_var2: Vec<u8>;

    pu_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = 0;
    process_struct_1008_574a((param_1 & 0xffff0000 | ZEXT24(local_bx_4 + 1)));
    unsafe { *param_1 = 0x9416 };
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    PTR_LOOP_1050_0388 = local_bx_4;
    PTR_LOOP_1050_038a = pu_var2;
    u_var1 = SetTimer16(0, 0, 1, 1);
    if (u_var1 == 0) {
        call_fn_ptr_1000_24cd(1);
    }
    return param_1;
}

pub fn kill_timer_1008_921c(param_1: *mut Struct215) {
    let local_bx_4: *mut Struct215;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0x9416;
    local_bx_4.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    KillTimer16(1, ctx.g_h_window);
    _PTR_LOOP_1050_0388 = 0;
    pass1_1008_57c4((param_1 & 0xffff0000 | &local_bx_4.field_0x6));
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_bx_4.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn kill_timer_1008_93ec(in_struct_1: *mut Struct215, param_2: u8) -> *mut Struct215 {
    kill_timer_1008_921c(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn switch_stmt_1008_aaa8(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let mut local_4: u16;

    local_4 = 0;
    match (param_1_00) {
        1 => local_4 = 0x24,
        2 => local_4 = 0x16,
        3 => local_4 = 0x17,
        4 => local_4 = 0x18,
        5 => local_4 = 0x1b,
        6 => local_4 = 0x1c,
        7 => local_4 = 0x1f,
        _ => local_4 = local_4,
    }
    return local_4;
}

pub fn switch_stmt_1008_ab80(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let mut local_4: u16;

    local_4 = 0;
    match (param_1_00) {
        8 => local_4 = 0x82,
        9 => local_4 = 0x7f,
        10 => local_4 = 0x80,
        0xb => local_4 = 0x84,
        0xc => local_4 = 0x89,
        0xd => local_4 = 0x8a,
        0xe => local_4 = 0x8c,
        0xf => local_4 = 0x8e,
        0x10 => local_4 = 0x8f,
        0x11 => local_4 = 0x90,
        0x12 => local_4 = 0x91,
        0x13 => local_4 = 0x95,
        0x14 => local_4 = 0x96,
        0x16 => local_4 = 0x9b,
        0x17 => local_4 = 0x9f,
        0x18 => local_4 = 0xa2,
        0x19 => local_4 = 0xa4,
        0x1b | 0x1c => local_4 = 0xa7,
        0x1d => local_4 = 0xaa,
        0x1e => local_4 = 0xac,
        0x1f => local_4 = 0xad,
        0x20 => local_4 = 0xae,
        0x21 => local_4 = 0xb1,
        0x22 => local_4 = 0xb3,
        0x23 => local_4 = 0xb4,
        0x24 => local_4 = 0xb5,
        0x25 => local_4 = 0xb6,
        0x26 => local_4 = 0xb7,
        0x27 => local_4 = 0xab,
        0x28 => local_4 = 0xb9,
        0x29 => local_4 = 0xba,
        0x2a => local_4 = 0xbc,
        0x2b => local_4 = 0xbe,
        0x2c => local_4 = 0xdf,
        0x2d => {
            local_4 = 0xe0;
        }
    }
    return local_4;
}

pub fn switch_stmt_1008_d818(in_struct_1: *mut Struct300, switch_param: u16) {
    let local_struct: *mut Struct300;
    let mut u_var1: u16;

    if (switch_param - 0x1a0 < 0x15) {
        local_struct = in_struct_1;
        u_var1 = (in_struct_1 >> 0x10);
        match (switch_param) {
            0x1a0 => local_struct.field_0xe = 0x14,
            0x1a1 => local_struct.field_0xe = 3,
            0x1a2 => local_struct.field_0xe = 2,
            0x1a3 => local_struct.field_0xe = 0xe,
            0x1a4 => local_struct.field_0xe = 0xc,
            0x1a5 => local_struct.field_0xe = 4,
            0x1a6 => local_struct.field_0xe = 0xb,
            0x1a7 => local_struct.field_0xe = 6,
            0x1a8 => local_struct.field_0xe = 10,
            0x1a9 => local_struct.field_0xe = 0xd,
            0x1aa => local_struct.field_0xe = 0x13,
            0x1ab => local_struct.field_0xe = 5,
            0x1ac => local_struct.field_0xe = 9,
            0x1ad => local_struct.field_0xe = 8,
            0x1ae => local_struct.field_0xe = 0x12,
            0x1af => local_struct.field_0xe = 0x11,
            0x1b0 => local_struct.field_0xe = 7,
            0x1b1 => local_struct.field_0xe = 0x10,
            0x1b2 => local_struct.field_0xe = 1,
            0x1b3 => local_struct.field_0xe = 0xf,
            0x1b4 => local_struct.field_0xe = 0x15,
        }
    }
    return;
}

pub fn return_10_1010_0886() -> u16 {
    return 10;
}

pub fn return3_1010_088c() -> u16 {
    return 3;
}

pub fn return3_1010_0892() -> u16 {
    return 3;
}

pub fn return3_1010_0898() -> u16 {
    return 3;
}

pub fn get_private_profile_str_1010_6132(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut in_dx: i32;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut unaff_cs: u16;
    let mut local_4: u16;

    u_var1 = (param_2 * 4 + 0x1446);
    u_var9 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var2 = (i_var8 + 0xe);
    u_var3 = (i_var8 + 10);
    GetPrivateProfileString16(
        CONCAT22(u_var3, unaff_cs),
        (u_var3 >> 0x10),
        CONCAT22(u_var2, 0x100),
        CONCAT22(0x149a, (u_var2 >> 0x10)),
        CONCAT22(u_var1, 0x1050),
        CONCAT22(0x13b8, (u_var1 >> 0x10)),
    );
    if (**(i_var8 + 0xe) != '\0') {
        u_var4 = pass1_fn_1000_47a4((i_var8 + 0xe), 0x105014a6);
        u_var6 = in_dx | u_var4;
        if (u_var6 != 0) {
            u_var5 = pass1_fn_1000_3e2c(CONCAT22(in_dx, u_var4));
            i_var8 = param_2 * 8 + i_var8;
            (i_var8 + 0x22) = u_var5;
            u_var4 = pass1_fn_1000_47a4(0, 0x105014a8);
            u_var7 = u_var6 | u_var4;
            if (u_var7 != 0) {
                u_var5 = pass1_fn_1000_3e2c(CONCAT22(u_var6, u_var4));
                (i_var8 + 0x24) = u_var5;
                u_var4 = pass1_fn_1000_47a4(0, 0x105014aa);
                u_var6 = u_var7 | u_var4;
                if (u_var6 != 0) {
                    u_var5 = pass1_fn_1000_3e2c(CONCAT22(u_var7, u_var4));
                    (i_var8 + 0x26) = u_var5;
                    u_var4 = pass1_fn_1000_47a4(0, 0x105014ac);
                    if ((u_var6 | u_var4) != 0) {
                        u_var5 = pass1_fn_1000_3e2c(CONCAT22(u_var6, u_var4));
                        (i_var8 + 0x28) = u_var5;
                    }
                }
            }
        }
    }
    return;
}

pub fn write_private_profile_str_1010_622a(param_1: u32, param_3: i32) {
    let mut u_var1: u32;
    let mut i_var2: i32;

    i_var2 = param_3 * 8 + param_1;
    if (((((i_var2 + 0x22) != 0) || ((i_var2 + 0x24) != 0)) || ((i_var2 + 0x26) != 0))
        || ((i_var2 + 0x28) != 0))
    {
        u_var1 = (param_1 + 0xe);
        string_fn_1000_3f9c(
            u_var1,
            (u_var1 >> 0x10),
            s__d__d__d__d_1050_14ae,
            &ctx.g_alloc_addr_1050_1050,
            (param_3 * 8 + param_1 + 0x22),
        );
        WritePrivateProfileString16(
            (param_1 + 10),
            (param_1 + 0xe),
            (param_3 * 4 + 0x1446),
            s_windows_1050_13b8,
        );
    }
    return;
}

pub fn process_switch_stmt_1010_6646(
    param_1: u16,
    param_2: u16,
    in_value_to_set: *mut u16,
    in_switch_param: u16,
) {
    unsafe {
        match (in_switch_param) {
            0x83 => *in_value_to_set = 10,
            0x84 => *in_value_to_set = 0xe,
            0x85 => *in_value_to_set = 0x12,
            0x86 => {
                *in_value_to_set = 0x16;
                return;
            }
            0x87 => {
                *in_value_to_set = 0x1a;
                return;
            }
            0x88 => {
                *in_value_to_set = 0x1e;
                return;
            }
            0x89 => {
                *in_value_to_set = 0x22;
                return;
            }
            _ => {
                *in_value_to_set = 0;
                return;
            }
        }
    }
    return;
}

pub fn process_switch_stmt_1010_6cf8(param_1: u16, param_2: u32, param_1_00: i32) -> u16 {
    let mut u_var1: u16;

    match (param_1_00) {
        1 => {
            pass1_1010_715c(param_2, 1);
            send_win_msg_1010_7c9e(param_2, 0x12);
            return 1;
        }
        // default:
        _ => return 0,
        4 => u_var1 = 2,
        5 => u_var1 = 3,
        6 => u_var1 = 4,
        7 => u_var1 = 5,
        9 => pass1_1010_715c(param_2, 6),
        0x2e => u_var1 = 0x38,
        10 | 0x80 => u_var1 = 0x2d,
        0xb => u_var1 = 7,
        0xc | 0x17 | 0x18 | 0x19 | 0x21 | 0x75 | 0x81 => {
            if (param_1_00 == 0x75) {
                pass1_1010_715c(param_2, 8);
                pass1_1010_715c(param_2, 9);
            }
            u_var1 = pass1_1010_6ca2(param_2, 7);
            if (u_var1 != 0) {
                pass1_1010_715c(param_2, 0x10);
            }
            u_var1 = pass1_1010_6ca2(param_2, 3);
            if (u_var1 != 0) {
                pass1_1010_715c(param_2, 0x11);
            }
            if (param_1_00 == 0x21) {
                pass1_1010_715c(param_2, 0x14);
            }
            if (param_1_00 != 0xc) {
                return 1;
            }
            u_var1 = 9;
            // goto code_r0x10106d4c;
        }
        0xe => {
            u_var1 = 0xc;
            // goto code_r0x10106d4c;
        }
        0x10 | 0x11 | 0x13 => u_var1 = 0xd,
        0x12 => u_var1 = 0xe,
        0x1b | 0x1f | 0x5b | 0x78 | 0x7e | 0x7f => {
            if (param_1_00 == 0x7e) {
                pass1_1010_715c(param_2, 0x2c);
            }
            if (param_1_00 == 0x5b) {
                pass1_1010_715c(param_2, 0x38);
            }
            if (param_1_00 == 0x1f) {
                pass1_1010_715c(param_2, 0x3f);
            }
            if (param_1_00 == 0x7f) {
                pass1_1010_715c(param_2, 0x42);
            }
            u_var1 = pass1_1010_6ca2(param_2, 5);
            if ((u_var1 == 0) && (u_var1 = pass1_1010_6ca2(param_2, 5), u_var1 == 0)) {
                return 1;
            }
            u_var1 = 0x37
        }
        0x1d | 0x2a | 0x2c | 0x3c | 0x3d | 0x4b | 0x53 | 0x54 | 0x55 | 0x5a => {
            u_var1 = pass1_1010_6ca2(param_2, 2);
            if (u_var1 != 0) {
                pass1_1010_715c(param_2, 0x12);
            }
            u_var1 = pass1_1010_6ca2(param_2, 8);
            if (u_var1 != 0) {
                pass1_1010_715c(param_2, 0x1a);
            }
            if (param_1_00 == 0x2c) {
                pass1_1010_715c(param_2, 0x1d);
            }
            u_var1 = pass1_1010_6ca2(param_2, 2);
            if (u_var1 == 0) {
                return 1;
            }
            u_var1 = 0x1c
        }
        0x22 => u_var1 = 0x15,
        0x25 => u_var1 = 0x16,
        0x26 => pass1_1010_715c(param_2, 0x17),
        0x1e => u_var1 = 0x13,
        0x27 => u_var1 = 0x18,
        0x29 => u_var1 = 0x19,
        0x2b => u_var1 = 0x1b,
        0x2f | 0x36 => {
            u_var1 = pass1_1010_6ca2(param_2, 2);
            if (u_var1 == 0) {
                return 0;
            }
            u_var1 = 0x1e
        }
        0x30 => u_var1 = 0x1f,
        0x31 => u_var1 = 0x35,
        0x33 => u_var1 = 0x21,
        0x34 => u_var1 = 0x22,
        0x35 => {
            pass1_1010_715c(param_2, 0x23);
        }
        0x65 | 0x66 | 0x6b | 0x6c | 0x6d | 0x6e | 0x6f => u_var1 = 0x34,
        0x38 => {
            pass1_1010_715c(param_2, 0x24);
            u_var1 = 0x3d
        }
        0x39 => u_var1 = 0x25,
        0x3e => {
            pass1_1010_715c(param_2, 0x26);
            pass1_1010_715c(param_2, 0x28);
            u_var1 = 0x27
        }
        0x40 => u_var1 = 0x2a,
        0x41 => u_var1 = 0x39,
        0x42 => u_var1 = 0x3a,
        0x44 => u_var1 = 0x36,
        0x45 => u_var1 = 0x3b,
        0x49 => u_var1 = 0x29,
        0x50 => u_var1 = 0x2b,
        0x56 => {
            pass1_1010_715c(param_2, 0x3c);
            u_var1 = 0x3e
        }
        0x5d => {
            pass1_1010_715c(param_2, 0x2f);
            u_var1 = 0x40
        }
        0x5e | 0x60 => u_var1 = 0x2f,
        0x5f => {
            pass1_1010_715c(param_2, 0x34);
            u_var1 = 0x41
        }
        0x61 => u_var1 = 0x30,
        99 => u_var1 = 0x31,
        100 => u_var1 = 0x24,
        0x68 => u_var1 = 0x32,
        0x69 => u_var1 = 0x33,
        0x76 => {
            u_var1 = 10;
            // code_r0x10106d4c:
            pass1_1010_715c(param_2, u_var1);
            u_var1 = 0xb;
        }
    }
    pass1_1010_715c(param_2, u_var1);
    return 1;
}

pub fn ret_1030_e4ba() {
    return;
}

pub fn return_1_1020_79ae() -> u16 {
    return 1;
}

pub fn big_fn_1010_b038(a: u32, b: u8) {
    todo!()
}
