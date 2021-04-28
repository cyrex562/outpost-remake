
pub fn call_big_fn_1040_b17c(in_pustruct_a: &mut astruct_345, in_u32_b: u32) {
    let mut u32_a: u32;
    let pu16_b: *mut u16;
    let mut u16_c: u16;
    let mut u16_d: u16;
    let pstruct_e: *mut astruct_345;
    let mut u16_f: u16;
    let mut u16_g: u16;
    let pustruct_h: *mut pass1_struct_1;
    let mut u32_j: u32;
    let mut u16_k: u16;
    let mut u16_m: u16;
    let mut u16_n: u16;

    u16_n = 0;
    loop {
        // uVar4 = (param_1 >> 0x10);
        let pustruct_p = in_pustruct_a;
        puVar2 = pustruct_p.field_0x90;
        if (*puVar2 == u16_n || *puVar2 < u16_n) {
            break;
        }
        uVar1 = (puVar2 + 2);
        (u16_n * 10 + uVar1 + 4) = (u16_n * 2 + in_u32_b);
        u16_n = u16_n + 1;
    }
    pustruct_h = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(u16_f, 3));
    puVar2 = pstruct_e.field_0x90;
    u32_j = (puVar2 + 2);
    u16_n = 0;
    while (
        puVar2 = pstruct_e.field_0x90,
        *puVar2 != u16_n && u16_n <= *puVar2,
    ) {
        puVar2 = pstruct_e.field_0x90;
        uVar3 = u16_n;
        big_fn_1010_b038(pustruct_h, (puVar2 + 6));
        pass1_1040_a626(u32_j, CONCAT22(u16_d, uVar3));
        u32_j = u32_j & 0xffff0000 | (u32_j + 10);
        u16_n = u16_n + 1;
    }
    return;
}


// WARNING: Removing unreachable block (ram,0x10004090)
// WARNING: Removing unreachable block (ram,0x1000409a)
// WARNING: Removing unreachable block (ram,0x10004311)
// WARNING: Removing unreachable block (ram,0x1000438a)
// WARNING: Removing unreachable block (ram,0x10004372)
// WARNING: Removing unreachable block (ram,0x100043aa)
// WARNING: Removing unreachable block (ram,0x10004f47)
// WARNING: Removing unreachable block (ram,0x10004fa9)
// WARNING: Removing unreachable block (ram,0x10004fd7)
// WARNING: Removing unreachable block (ram,0x10004feb)
// WARNING: Removing unreachable block (ram,0x10005167)
// WARNING: Removing unreachable block (ram,0x1000518c)
// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)
pub fn big_fn_1008_15d4(param_1: *mut astruct_657, param_2: u16) {
    let pi32_a: *mut i32;
    let mut u32_b: u32;
    let mut i32_c: i32;
    let mut string_d: *mut libc::c_char;
    let mut i32_e: i32;
    let pustruct_f: *mut astruct_199;
    let mut i32_g: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let mut extraout_DX_05: i32;
    let mut extraout_DX_06: i32;
    let mut extraout_DX_07: i32;
    let pustruct_h: *mut astruct_199;
    let mut extraout_DX_08: i32;
    let mut extraout_DX_09: i32;
    let mut extraout_DX_10: i32;
    let mut extraout_DX_11: i32;
    let mut extraout_DX_12: i32;
    let mut extraout_DX_13: i32;
    let mut extraout_DX_14: i32;
    let mut extraout_DX_15: i32;
    let mut extraout_DX_16: i32;
    let mut extraout_DX_17: i32;
    let mut extraout_DX_18: i32;
    let mut extraout_DX_19: i32;
    let mut extraout_DX_20: i32;
    let mut extraout_DX_21: i32;
    let mut extraout_DX_22: i32;
    let mut extraout_DX_23: i32;
    let mut extraout_DX_24: i32;
    let mut extraout_DX_25: i32;
    let mut extraout_DX_26: i32;
    let mut extraout_DX_27: i32;
    let mut extraout_DX_28: i32;
    let mut extraout_DX_29: i32;
    let mut extraout_DX_30: i32;
    let mut extraout_DX_31: i32;
    let mut extraout_DX_32: i32;
    let mut extraout_DX_33: i32;
    let mut extraout_DX_34: i32;
    let mut extraout_DX_35: i32;
    let mut extraout_DX_36: i32;
    let mut extraout_DX_37: i32;
    let mut extraout_DX_38: i32;
    let mut extraout_DX_39: i32;
    let mut extraout_DX_40: i32;
    let mut extraout_DX_41: i32;
    let mut extraout_DX_42: i32;
    let mut extraout_DX_43: i32;
    let mut extraout_DX_44: i32;
    let mut extraout_DX_45: i32;
    let mut extraout_DX_46: i32;
    let mut extraout_DX_47: i32;
    let mut extraout_DX_48: i32;
    let mut extraout_DX_49: i32;
    let mut extraout_DX_50: i32;
    let mut extraout_DX_51: i32;
    let mut extraout_DX_52: i32;
    let mut extraout_DX_53: i32;
    let mut extraout_DX_54: i32;
    let mut extraout_DX_55: i32;
    let mut extraout_DX_56: i32;
    let mut extraout_DX_57: i32;
    let mut extraout_DX_58: i32;
    let mut extraout_DX_59: i32;
    let mut extraout_DX_60: i32;
    let mut extraout_DX_61: i32;
    let mut extraout_DX_62: i32;
    let mut extraout_DX_63: i32;
    let mut extraout_DX_64: i32;
    let mut extraout_DX_65: i32;
    let mut extraout_DX_66: i32;
    let mut extraout_DX_67: i32;
    let mut extraout_DX_68: i32;
    let mut extraout_DX_69: i32;
    let mut extraout_DX_70: i32;
    let mut extraout_DX_71: i32;
    let mut extraout_DX_72: i32;
    let mut extraout_DX_73: i32;
    let mut extraout_DX_74: i32;
    let mut extraout_DX_75: i32;
    let mut extraout_DX_76: i32;
    let mut extraout_DX_77: i32;
    let mut extraout_DX_78: i32;
    let mut extraout_DX_79: i32;
    let mut extraout_DX_80: i32;
    let mut extraout_DX_81: i32;
    let mut extraout_DX_82: i32;
    let mut extraout_DX_83: i32;
    let mut extraout_DX_84: i32;
    let mut extraout_DX_85: i32;
    let mut extraout_DX_86: i32;
    let mut extraout_DX_87: i32;
    let mut extraout_DX_88: i32;
    let mut extraout_DX_89: i32;
    let mut extraout_DX_90: i32;
    let mut extraout_DX_91: i32;
    let mut extraout_DX_92: i32;
    let mut extraout_DX_93: i32;
    let mut i32_j: i32;
    let mut extraout_DX_94: i32;
    let mut extraout_DX_95: i32;
    let mut extraout_DX_96: i32;
    let mut extraout_DX_97: i32;
    let mut extraout_DX_98: i32;
    let mut extraout_DX_99: i32;
    let mut i32_k: i32;
    let mut i32_m: i32;
    let mut stack_seg_n: u16;
    let pustruct_p: *mut astruct_65;
    let mut param_1_hi: u16;
    let mut u32_r: u32;
    let mut u32_s: u32;
    let mut u16_t: u16;
    let mut u16_v: u16;
    let mut string_w: [u8; 8];
    let mut u32_x: u32;

    u32_x = 0;
    i32_c = param_1;
    pass1_1008_57a4(
        CONCAT22(stack_seg_n, string_w),
        param_1 & 0xffff0000 | (i32_c + 0xd2),
    );
    let mut broke = false;
    while {
        string_d = string_w;
        let in_1: *mut astruct_306 = CONCAT22(stack_seg_n, string_d);
        pass1_1008_5b12(in_1);
        pustruct_f = (i32_e | string_d);
        if (pustruct_f == 0) {
            broke = true;
            break;
        }
        u32_b = string_d + 4;
        pustruct_f = string_d + 6;
        i32_k = u32_b;
        (i32_k + 0xde) != param_2
    } {}
    if !broke {
        u32_x = u32_b & 0xffff | ZEXT24(pustruct_f) << 0x10;
    }

    if (u32_x != 0) {
        return;
    }
    param_1_hi = (param_1 >> 0x10);
    match param_2 - 1 {
        0 => {
            process_struct_1000_179c(0xec, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                let mut input_a: *mut astruct_675 = param_1;
                pass1_1008_6978(input_a, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe { *pi32_a = *pi32_a + 1 };
            mci_fn_1020_08b6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_07;
        }
        2 => {
            process_struct_1000_179c(0xfa, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                let mut input_a: *mut astruct_675 = param_1;
                pass1_1008_6978(input_a, 0, u32_x);
                return;
            }

            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            let input_a: *mut astruct_65 = CONCAT22(pustruct_f, i32_k);
            process_struct_1018_e91e(input_a, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_05;
        }
        3 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;

                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_e230(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_03;
        }
        4 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            call_load_cursor_fn_1020_7554(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_04;
        }
        5 => {
            process_struct_1000_179c(0xf8, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_5840(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_06;
        }
        6 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            call_load_cursor_1020_2524(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_02;
        }
        7 => {
            process_struct_1000_179c(0x118, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_accelerators_1020_41c8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_01;
        }
        8 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            process_struct_1018_e5dc(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = i32_g;
        }
        9 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_66cc(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_00;
        }
        10 => {
            mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1d3);
            pustruct_f = pustruct_h;
            process_struct_1000_179c(0xf2, pustruct_h);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6d02(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_08;
        }
        0xb => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6d38(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_09;
        }
        0xc => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6d6e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_10;
        }
        0xd => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6da4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_11;
        }
        0xe => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6dda(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_12;
        }
        0xf => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6e10(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_13;
        }
        0x10 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6e46(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_14;
        }
        0x11 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6e7c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_15;
        }
        0x12 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6eb2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_16;
        }
        0x13 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6ee8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_17;
        }
        0x14 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6f1e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_18;
        }
        0x15 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6f54(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_19;
        }
        0x16 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6f8a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_20;
        }
        0x17 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6fc0(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_21;
        }
        0x18 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6ff6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_22;
        }
        0x19 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_702c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_23;
        }
        0x1a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7062(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_24;
        }
        0x1b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7098(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_25;
        }
        0x1c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_70ce(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_26;
        }
        0x1d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7104(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_27;
        }
        0x1e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_713a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_28;
        }
        0x20 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7170(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_29;
        }
        0x21 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_745e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_31;
        }
        0x22 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_71a6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_32;
        }
        0x23 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_71dc(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_33;
        }
        0x24 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7212(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_34;
        }
        0x25 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }

            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_c958(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_86;
        }
        0x26 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_c9a6(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_87;
        }
        0x27 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_c9f4(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_88;
        }
        0x28 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_ca48(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_89;
        }
        _ => {
            fn_1008_6048(
                s_OpWnd__getKid__Unknown_target_mo_1050_01a3,
                pustruct_f,
                SUB21(i32_k, 0),
            );
            call_fn_ptr_1000_24cd(1);
            pass1_1008_6978(param_1, 0, u32_x);
            return;
        }

        0x29 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_ca96(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_90;
        }
        0x2a => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_caea(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_91;
        }
        0x2b => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cb38(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_92;
        }
        0x2c => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cb86(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_93;
        }
        0x2d => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pustruct_p = pass1_1018_cbda(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = (pustruct_p >> 0x10);
            i32_k = pustruct_p;
        }
        0x2e => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cc28(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_94;
        }
        0x2f => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cc76(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_95;
        }
        0x30 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_ccc4(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_96;
        }
        0x31 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cd12(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_97;
        }
        0x32 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cd60(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_98;
        }
        0x33 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cf74(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_99;
        }
        0x34 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_73c2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_30;
        }
        0x35 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7494(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_36;
        }
        0x36 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_74ca(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_37;
        }
        0x37 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7500(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_38;
        }
        0x38 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_73f8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_39;
        }
        0x39 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7536(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_40;
        }
        0x3a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_756c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_41;
        }
        0x3b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_75a2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_42;
        }
        0x3c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_75d8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_43;
        }
        0x3d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_760e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_44;
        }
        0x3e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7644(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_45;
        }
        0x3f => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_767a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_46;
        }
        0x40 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_76b0(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_47;
        }
        0x41 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_76e6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_48;
        }
        0x42 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_771c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_49;
        }
        0x43 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7752(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_50;
        }
        0x44 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7788(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_51;
        }
        0x45 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_77be(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_52;
        }
        0x46 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_77f4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_53;
        }
        0x47 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_782a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_54;
        }
        0x48 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_55;
        }
        0x49 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7896(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_56;
        }
        0x4a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_78cc(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_57;
        }
        0x4b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7902(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_58;
        }
        0x4c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7938(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_59;
        }
        0x4d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_796e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_60;
        }
        0x4e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_79a4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_61;
        }
        0x4f => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_79da(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_62;
        }
        0x50 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7a10(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_63;
        }
        0x51 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7a46(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_64;
        }
        0x52 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7a7c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_65;
        }
        0x54 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7ab2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_66;
        }
        0x55 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7ae8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_67;
        }
        0x56 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7b1e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_68;
        }
        0x57 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7b54(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_69;
        }
        0x58 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7b8a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_70;
        }
        0x59 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7bc0(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_71;
        }
        0x5a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7bf6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_72;
        }
        0x5b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7c2c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_73;
        }
        0x5c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7c62(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_74;
        }
        0x5d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7c98(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_35;
        }
        0x5e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7cce(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_75;
        }
        0x5f => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7d04(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_76;
        }
        0x60 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7d3a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_77;
        }
        0x61 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7d70(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_78;
        }
        0x62 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_7248(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_79;
        }
        99 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_727e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_80;
        }
        100 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_72b4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_81;
        }
        0x65 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_72ea(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_82;
        }
        0x66 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7320(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_83;
        }
        0x67 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7356(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_84;
        }
        0x68 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_738c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_DX_85;
        }
    };
    u32_x = CONCAT22(i32_j, i32_k);
    pass1_1008_6978(param_1, 0, u32_x);
    return;
}


pub fn call_big_fn_1010_1788(param_1: u16, param_2: u16, param_1_00: u32) {
    let uVar1: u8;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000fff4: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT22((in_stack_0000fff4 >> 0x10), 3),
    );
    uVar1 = pass1_1028_b58e(param_1_00);
    big_fn_1010_b038(ppVar2, uVar1);
    pass1_fn_1008_60e8();
    return;
}

pub fn call_big_fn_1010_1c16(param_1: u32, param_2: u32) {
    let uVar1: u8;

    uVar1 = pass1_1028_b58e(param_2);
    big_fn_1010_b038((param_1 + 0x6e), uVar1);
    pass1_fn_1008_60e8();
    return;
}


pub fn call_big_fn_1010_c7e2(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut extraout_DX: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        uVar4 = (param_3 >> 0x10);
        iVar3 = param_3;
        if (param_3 == local_4 || param_3 < local_4) {
            break;
        }
        uVar1 = (iVar3 + 2);
        (local_4 * 10 + uVar1 + 4) = (local_4 * 2 + param_2);
        local_4 = local_4 + 1;
    }
    local_8 = (iVar3 + 2);
    local_4 = 0;
    while (param_3 != local_4 && local_4 <= param_3) {
        uVar2 = local_4;
        big_fn_1010_b038(param_1, (iVar3 + 6));
        pass1_1040_a626(local_8, CONCAT22(extraout_DX, uVar2));
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
    }
    return;
}