pub fn pass1_1028_ed2c(param_1: u32, param_2: u16, param_2: u16_00, param_4: u16, param_5: u32) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut in_AX: u16;
    let mut uvar3: u16;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x1e, in_DX);
    if ((in_DX | in_AX) == 0) {
        in_AX = 0;
        uVar4 = 0;
    } else {
        pass1_1030_565a(CONCAT22(in_DX, in_AX), param_5);
        uVar4 = extraout_DX;
    }
    uVar6 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x52);
    uVar5 = uVar4;
    uVar3 = in_AX;
    pass1_1030_4782(uVar1, (uVar1 >> 0x10), 1, 1, param_2_00);
    pass1_1030_5a80(CONCAT22(uVar4, in_AX), CONCAT22(uVar5, uVar3));
    uVar2 = (in_AX + 4);
    pass1_1030_6222(_PTR_LOOP_1050_5740, 1, CONCAT22(uVar5, uVar3), uVar2);
    pass1_1030_1358((param_1 + 0x16), in_AX, uVar4, uVar2 & 0xffffff);
    return;
}

pub fn pass1_1028_edc4(param_1: u32, param_2: u16, param_2_00: u32, param_4: u32) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let struct_a: *mut astruct_199;
    let paVar3: *mut astruct_199;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: [u8; 4];
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = param_2_00;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2_00, param_4, local_1a, unaff_SS);
    uVar2 = param_2_00;
    paVar3 = struct_a;
    local_e = uVar2;
    local_a = uVar2;
    process_struct_1000_179c(0x21e, struct_a);
    uVar1 = uVar2;
    uVar4 = paVar3 | uVar1;
    if (uVar4 == 0) {
        uVar1 = 0;
        uVar4 = 0;
    } else {
        pass1_1038_3222((uVar2 & 0xffff | ZEXT24(paVar3) << 0x10), local_e, param_4);
    }
    _local_12 = CONCAT22(uVar4, uVar1);
    local_16 = (uVar1 + 4);
    pass1_1030_1358(
        (param_1 + 0x1a),
        uVar1,
        uVar4,
        local_16 & 0xffff | ((uVar1 + 6) & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1028_ee54(param_1: u32, param_2: u32, param_3: u32) {
    let puVar1: *mut u8;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut unaff_SS: u16;
    let mut local_26: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = param_2;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_3, local_16, unaff_SS);
    local_a = param_2;
    puVar1 = _PTR_LOOP_1050_5744;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_5744);
    local_e = puVar1;
    local_c = extraout_DX_00 | local_e;
    if (local_c == 0) {
        local_e = 0;
        local_c = 0;
    } else {
        pass1_1030_684c(
            (puVar1 & 0xffff | extraout_DX_00 << 0x10),
            local_6,
            (local_6 >> 0x10),
            local_a,
            (local_a >> 0x10),
            param_3,
        );
    }
    local_12 = (local_e + 4);
    pass1_1030_61fe(_PTR_LOOP_1050_5740, local_12, local_6, param_3);
    pass1_1030_1358(
        (param_1 + 0x1e),
        local_e,
        local_c,
        local_12 & 0xffff | (local_12._2_2_ & 0xff) << 0x10,
    );
    return local_12;
}

pub fn pass1_1028_ef00(param_1: u32, param_2: u16, uparam_2_00: i32, param_4: u16, param_5: u32) {
    let mut uVar1: i32;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_2_00 == 4) {
        process_struct_1000_179c(0x16, in_DX);
        if ((in_DX | param_2_00) != 0) {
            pass1_1030_b936(param_2_00, CONCAT22(4, in_DX), param_5);
            uVar2 = extraout_DX;
            // goto LAB_1028_ef8b;
        }
    } else {
        if (param_2_00 == 0xc) {
            process_struct_1000_179c(0xe, in_DX);
            if ((in_DX | param_2_00) != 0) {
                puVar3 = pass1_1030_bc24(param_2_00, in_DX, 0xc, param_5);
                uVar2 = (puVar3 >> 0x10);
                param_2_00 = puVar3;
                // goto LAB_1028_ef8b;
            }
        } else {
            uVar1 = param_2_00;
            process_struct_1000_179c(0xe, in_DX);
            if ((in_DX | uVar1) != 0) {
                puVar3 = pass1_1028_b22c(CONCAT22(in_DX, uVar1), param_2_00, param_5);
                uVar2 = (puVar3 >> 0x10);
                param_2_00 = puVar3;
                // goto LAB_1028_ef8b;
            }
        }
    }
    param_2_00 = 0;
    uVar2 = 0;
    // LAB_1028_ef8b:
    pass1_1030_1358(
        (param_1 + 0x22),
        param_2_00,
        uVar2,
        (param_2_00 + 4) & 0xffff | ((param_2_00 + 6) & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1030_0000(param_1: u16, param_2: u16, param_1: u16_00) -> *mut astruct_763 {
    let in_DX: *mut astruct_199;
    let mut uVar1: i32;
    let mut in_BX: i32;
    let paVar2: *mut astruct_763;
    let puVar3: *mut u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    // Segment:    7
    // Offset:     000516c0
    // Length:     ef76
    // Min Alloc:  ef76
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    puVar3 = CONCAT22(in_DX, in_BX);
    match (param_1_00 - 1) {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_489e(in_BX, in_DX);
                return paVar2;
            }
        }
        9 => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_2bdc(in_BX, in_DX);
                return paVar2;
            }
        }
        10 => {
            process_struct_1000_179c(0x26, in_DX);
            uVar1 = in_DX | in_BX;
            // goto joined_r0x103002a1;
        }
        0xb => {
            process_struct_1000_179c(0x2c, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_3670(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0xc => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_355e(in_BX, in_DX);
                return paVar2;
            }
        }
        0xd => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_3484(in_BX, in_DX);
                return paVar2;
            }
        }
        0xe => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_406c(in_BX, in_DX);
                return paVar2;
            }
        }
        0xf | 0x32 | 0x33 | 0x5f | 0x60 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_0c24(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x10 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_0b42(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x11 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_4354(in_BX, in_DX);
                return paVar2;
            }
        }
        0x12 | 0x13 | 0x14 | 0x61 | 0x62 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_4b84(in_BX, in_DX);
                return paVar2;
            }
        }
        0x15 | 0x16 | 0x17 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_1bbc(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        _ => {
            // default:
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_b354(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x1a | 0x1b | 0x1c => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1030_be34(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x1d | 0x1e | 0x1f => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_0068(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x20 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_50d8(in_BX, in_DX);
                return paVar2;
            }
        }
        0x21 | 0x22 | 0x23 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_3e94(in_BX, in_DX);
                return paVar2;
            }
        }
        0x24 | 0x25 | 0x26 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_d06c(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x27 | 0x28 | 0x5c | 0x5d | 0x5e => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1030_c6f6(in_BX, in_DX);
                return paVar2;
            }
        }
        0x29 | 0x2a => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_cce4(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x2b => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_26b4(in_BX, in_DX);
                return paVar2;
            }
        }
        0x2c | 0x2d => {
            process_struct_1000_179c(0x2a, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_49aa(in_BX, in_DX);
                return paVar2;
            }
        }
        0x2e | 0x2f => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_e7fa(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x30 | 0x31 | 0x6b | 0x6c => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_d37c(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x34 | 0x35 => {
            process_struct_1000_179c(0x2c, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_37a6(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x36 => {
            process_struct_1000_179c(0x26, in_DX);
            uVar1 = in_DX | in_BX;
            // joined_r0x103002a1:
            if (uVar1 != 0) {
                pass1_1030_c06e(CONCAT22(in_DX, in_BX));
                return CONCAT22(uVar1, in_BX);
            }
        }
        0x37 | 0x38 => {
            process_struct_1000_179c(0x9a, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1030_c9a8(in_BX, in_DX);
                return paVar2;
            }
        }
        0x39 | 0x3a => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_60bc(in_BX, in_DX);
                return paVar2;
            }
        }
        0x3b | 0x3c => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_44d2(in_BX, in_DX);
                return paVar2;
            }
        }
        0x3d => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_cde6(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x3e => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_1f56(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x3f => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_25da(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x40 => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_c9ea(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x46 | 0x69 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_d5a6(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x47 | 0x48 | 0x49 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_d866(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x4b | 0x4c | 0x4d => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1030_d8f6(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x4e | 0x4f | 0x50 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5c54(in_BX, in_DX);
                return paVar2;
            }
        }
        0x51 | 0x52 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5966(in_BX, in_DX);
                return paVar2;
            }
        }
        0x53 | 0x54 | 0x55 => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5ed8(in_BX, in_DX);
                return paVar2;
            }
        }
        0x56 | 0x57 | 0x58 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_53c6(in_BX, in_DX);
                return paVar2;
            }
        }
        0x59 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5884(in_BX, in_DX);
                return paVar2;
            }
        }
        0x5a | 0x5b => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5524(in_BX, in_DX);
                return paVar2;
            }
        }
        99 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5df6(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        100 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5a48(in_BX, in_DX);
                return paVar2;
            }
        }
        0x65 | 0x66 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_52e8(in_BX, in_DX);
                return paVar2;
            }
        }
        0x67 | 0x68 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_57a6(in_BX, in_DX);
                return paVar2;
            }
        }
        0x6d => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_5630(in_BX, in_DX);
                return paVar2;
            }
        }
        0x6f | 0x70 | 0x71 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) == 0) {
                puVar3 = 0x0;
            } else {
                puVar3 = pass1_1020_d866(CONCAT22(in_DX, in_BX));
            }
        }
        0x72 | 0x76 => {
            process_struct_1000_179c(0x26, (puVar3 >> 0x10));
            if (puVar3 != 0x0) {
                paVar2 = pass1_1020_e8f6(puVar3);
                return paVar2;
            }
        }
        0x73 | 0x77 | 0x78 => {
            process_struct_1000_179c(0x2c, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1020_d954(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x74 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_178c(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x75 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_2afa(CONCAT22(in_DX, in_BX));
                return paVar2;
            }
        }
        0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7e => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                paVar2 = pass1_1028_27f0(in_BX, in_DX);
                return paVar2;
            }
        }
    }
    return 0x0;
}

pub fn pass1_1030_07ac(
    param_1: u16,
    param_2: u16,
    param_1: u16_00,
    param_2: u16_00,
    param_5: u16,
    param_6: *mut u8,
) -> i32 {
    let in_DX: *mut astruct_764;
    let mut uVar1: i32;
    let in_BX: *mut astruct_764;
    let puVar2: *mut u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut in_stack_0000fff4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = param_6;
    uVar4 = (param_6 >> 0x10);
    match (param_2_00 - 1) {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_48c0(in_BX, in_DX, param_2_00, param_6);
            }
        }
        9 => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_2bfe(in_BX, in_DX, param_2_00, param_6);
            }
        }
        10 => {
            process_struct_1000_179c(0x26, in_DX);
            uVar1 = in_DX | in_BX;
            // goto joined_r0x10300adb; },
        }
        0xb => {
            process_struct_1000_179c(0x2c, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_3692(
                    CONCAT22(in_DX, in_BX),
                    param_2_00,
                    uVar3,
                    CONCAT22(in_stack_0000fff4, uVar4),
                );
            }
        }
        0xc => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_3580(
                    CONCAT22(in_DX, in_BX),
                    param_2_00,
                    uVar3,
                    CONCAT22(in_stack_0000fff4, uVar4),
                );
            }
        }
        0xd => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_34a6(
                    CONCAT22(in_DX, in_BX),
                    param_2_00,
                    uVar3,
                    CONCAT22(in_stack_0000fff4, uVar4),
                );
            }
        }
        0xe => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_408e(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0xf | 0x32 | 0x33 | 0x5f | 0x60 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_0c50(in_BX, CONCAT22(param_2_00, in_DX), param_6);
            }
        }
        0x10 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_0b64(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x11 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_4376(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x12 | 0x13 | 0x14 | 0x61 | 0x62 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_4ba6(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x15 | 0x16 | 0x17 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_1be8(in_BX, CONCAT22(param_2_00, in_DX), param_6);
            }
        }
        _ => {
            // default:
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_b39e(CONCAT22(in_DX, in_BX), param_2_00, param_6);
            }
        }
        0x1a | 0x1b | 0x1c => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1030_be56(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x1d | 0x1e | 0x1f => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_00cc(in_BX, CONCAT22(param_2_00, in_DX), param_6);
            }
        }
        0x20 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_50fa(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x21 | 0x22 | 0x23 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_3ec8(
                    CONCAT22(in_DX, in_BX),
                    param_2_00,
                    uVar3,
                    CONCAT22(in_stack_0000fff4, uVar4),
                );
            }
        }
        0x24 | 0x25 | 0x26 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_d08e(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x27 | 0x28 | 0x5c | 0x5d | 0x5e => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1030_c71e(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x29 | 0x2a => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_cd06(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x2b => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_26d6(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x2c | 0x2d => {
            process_struct_1000_179c(0x2a, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_49de(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x2e | 0x2f => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_e81c(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x30 | 0x31 | 0x6b | 0x6c => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_d3a4(CONCAT22(in_DX, in_BX), param_1_00, param_2_00, param_6);
            }
        }
        0x34 | 0x35 => {
            process_struct_1000_179c(0x2c, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_3816(in_BX, CONCAT22(param_2_00, in_DX), param_6);
            }
        }
        0x36 => {
            process_struct_1000_179c(0x26, in_DX);
            uVar1 = in_DX | in_BX;
            // joined_r0x10300adb:
            if (uVar1 != 0) {
                pass1_1030_c09c(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x37 | 0x38 => {
            process_struct_1000_179c(0x9a, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1030_c9e4(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x39 | 0x3a => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_611e(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x3b | 0x3c => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_44fe(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x3d => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_ce08(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x3e => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_1fc8(in_BX, CONCAT22(param_2_00, in_DX), param_6);
            }
        }
        0x3f => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_25fc(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x40 => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_ca0c(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x46 | 0x69 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_d5c8(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x47 | 0x48 | 0x49 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_d888(in_BX, in_DX, param_2_00, uVar3);
            }
        }
        0x4b | 0x4c | 0x4d => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1030_d942(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x4e | 0x4f | 0x50 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_5c76(in_BX, CONCAT22(param_2_00, in_DX), param_6);
            }
        }
        0x51 | 0x52 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_5988(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x53 | 0x54 | 0x55 => {
            process_struct_1000_179c(0x22, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_5f00(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x56 | 0x57 | 0x58 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_53e8(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x59 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_58a6(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x5a | 0x5b => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_5546(in_BX, in_DX, param_2_00, param_6);
            }
        }
        99 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_5e18(in_BX, in_DX, param_2_00, param_6);
            }
        }
        100 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_5a6a(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x65 | 0x66 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_530a(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x67 | 0x68 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_57c8(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x6d => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_5652(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x6f | 0x70 | 0x71 => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) == 0) {
                in_BX = 0x0;
                in_DX = 0x0;
            } else {
                puVar2 = pass1_1020_d888(in_BX, in_DX, param_2_00, uVar3);
                in_DX = (puVar2 >> 0x10);
                in_BX = puVar2;
            }
        }
        0x72 | 0x76 => {
            process_struct_1000_179c(0x26, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_e91e(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x73 | 0x77 | 0x78 => {
            process_struct_1000_179c(0x2c, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1020_d99e(CONCAT22(in_DX, in_BX), param_1_00, param_2_00, param_6);
            }
        }
        0x74 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_17ae(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x75 => {
            process_struct_1000_179c(0x24, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_2b1c(in_BX, in_DX, param_2_00, param_6);
            }
        }
        0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7e => {
            process_struct_1000_179c(0x20, in_DX);
            if ((in_DX | in_BX) != 0) {
                pass1_1028_2812(in_BX, in_DX, param_2_00, param_6);
            }
        }
    }
    return;
}

pub fn pass1_1030_10b0(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
    param_5: *mut u8,
) {
    let mut u_var1: u32;
    let mut in_AX: u16;
    let paVar2: *mut astruct_493;
    let mut in_DX: u16;
    let mut uvar3: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_07ac(
        param_1,
        param_2,
        param_3,
        param_3_00,
        (param_3_00 >> 0x10),
        param_5,
    );
    uVar1 = (in_AX + 4);
    uVar3 = in_DX;
    paVar2 = pass1_1028_e1ec(CONCAT22(param_2, param_1), param_5, (param_5 >> 0x10));
    if ((uVar3 | paVar2) != 0) {
        pass1_1030_7e5a(CONCAT22(uVar3, paVar2), uVar1);
    }
    local_a._2_2_ = (uVar1 >> 0x10);
    pass1_1030_1358(
        (param_1 + 0x26),
        in_AX,
        in_DX,
        uVar1 & 0xffff | (local_a._2_2_ & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1030_1120(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut uVar1: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x3b2, in_DX);
    if ((in_DX | in_AX) == 0) {
        in_AX = 0;
        uVar1 = 0;
    } else {
        pass1_1030_2112(in_AX, in_DX, 0);
        uVar1 = extraout_DX;
    }
    pass1_1030_1358(
        (param_1 + 0x2a),
        in_AX,
        uVar1,
        (in_AX + 4) & 0xffff | ((in_AX + 6) & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1030_117a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_11aa(param_1: *mut astruct_846, param_2: u32, param_3: u32) {
    let local_BX_4: *mut astruct_843;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = 0;
    local_BX_4.field_0x6 = 0;
    local_BX_4.field_0xa = 0;
    local_BX_4.field_0xe = param_3;
    local_BX_4.field_0x12 = 0;
    local_BX_4.field_0x16 = param_2;
    local_BX_4.field_0x1a = 1;
    param_1 = (s_462_bmp_1050_1620 + 4);
    local_BX_4.field_0x2 = 0x1030;
    if (local_BX_4.field_0xe == 0) {
        local_BX_4.field_0xe = 5;
    }
    if (local_BX_4.field_0x16 == 0) {
        local_BX_4.field_0x16 = 5;
    }
    pass1_1030_1550(param_1);
    local_BX_4.field_0x6 = 0;
    return;
}

pub fn pass1_1030_1244(param_1: *mut u16) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uVar3: i32;
    let ppcVar4: fn();
    let mut uVar5: u32;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_6: u32;

    uVar9 = (param_1 >> 0x10);
    iVar6 = param_1;
    unsafe { *param_1 = (s_462_bmp_1050_1620 + 4) };
    (iVar6 + 2) = 0x1030;
    if ((iVar6 + 0x1a) != 0) {
        local_6 = 1;
        while (true) {
            puVar1 = (iVar6 + 10);
            let pu_var1_val = unsafe { *puVar1 };
            if (pu_var1_val < local_6 || pu_var1_val == local_6) {
                break;
            }
            iVar8 = local_6 * 4;
            uVar5 = (iVar6 + 6);
            uVar10 = (uVar5 >> 0x10);
            iVar7 = uVar5;
            puVar2 = (iVar7 + iVar8);
            uVar3 = (iVar7 + iVar8 + 2);
            if ((uVar3 | puVar2) != 0) {
                unsafe {
                    ppcVar4 = *puVar2;
                    (**ppcVar4)();
                }
            }
            local_6 = local_6 + 1;
        }
    }
    error_check_1000_17ce((iVar6 + 6));
    unsafe {
        *param_1 = s_1_1050_389a;
        (iVar6 + 2) = &PTR_LOOP_1050_1008;
    }
    return;
}

pub fn pass1_1030_12ca(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_6: u32;

    local_6 = 1;
    while (true) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        puVar1 = (iVar3 + 10);
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val < local_6 || pu_var1_val == local_6) {
            (iVar3 + 4) = 0;
            return;
        }
        uVar2 = (iVar3 + 6);
        if ((uVar2 + local_6 * 4) == 0) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn pass1_1030_1312(a: u16, b: u16, c: u16) {
    let mut local_6: u32;

    return;
}

pub fn pass1_1030_1358(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut uVar6: u16;
    let mut bVar7: bool;

    if (param_4 == 0) {
        return;
    }
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 10);
    unsafe {
        if ((*puVar1 < param_4 || *puVar1 == param_4) || ((iVar4 + 6) == 0)) {
            puVar2 = (iVar4 + 0x14);
            bVar7 = *puVar2 < param_4._2_2_;
            if ((bVar7 || *puVar2 == param_4._2_2_)
                && (bVar7
                    || (
                        puVar2 = (iVar4 + 0x12),
                        *puVar2 < param_4 || *puVar2 == param_4,
                    )))
            {
                pass1_1030_1550((param_1 & 0xffff | uVar5 << 0x10));
            }
            puVar1 = (iVar4 + 0x12);
            if (*puVar1 < param_4 || *puVar1 == param_4) {
                return;
            }
            if ((iVar4 + 6) == 0) {
                return;
            }
            puVar2 = (iVar4 + 0xc);
            bVar7 = *puVar2 < param_4._2_2_;
            if ((bVar7 || *puVar2 == param_4._2_2_)
                && (bVar7
                    || (
                        puVar2 = (iVar4 + 10),
                        *puVar2 < param_4 || *puVar2 == param_4,
                    )))
            {
                (iVar4 + 10) = (param_4 + 1);
                (iVar4 + 0xc) = (param_4 + 1 >> 0x10);
            }
        }
    }
    uVar3 = (iVar4 + 6);
    uVar6 = (uVar3 >> 0x10);
    iVar4 = uVar3;
    (iVar4 + param_4 * 4) = param_2;
    (iVar4 + param_4 * 4 + 2) = param_3;
    return;
}

pub fn pass1_1030_13f6(param_1: u32, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u16;
    let puVar3: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    puVar3 = pass1_1030_1312(param_1, param_2);
    if (puVar3 != 0x0) {
        local_4 = 1;
        uVar2 = (param_1 >> 0x10);
        if (((param_1 + 0x1a) != 0) && (puVar3 != 0x0)) {
            unsafe {
                ppcVar1 = *puVar3;
                (**ppcVar1)();
            }
        }
        pass1_1030_1358(param_1, 0, 0, param_2);
        (param_1 + 4) = 1;
    }
    return local_4;
}

pub fn pass1_1030_145a(param_1: *mut astruct_844, param_2: libc::c_long) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let local_BX_4: *mut astruct_844;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    error_check_1000_17ce(local_BX_4.field_0x6);
    local_BX_4.field_0x6 = 0;
    local_BX_4.field_0xa = 0;
    uVar1 = local_BX_4.field_0x16 + param_2;
    uVar2 = (uVar1 >> 0x10);
    if (uVar1 < &local_BX_4.field_0xe) {
        uVar1 = local_BX_4.field_0xe;
        uVar2 = local_BX_4.field_0x10;
    }
    local_BX_4.field_0xe = uVar1;
    local_BX_4.field_0x10 = uVar2;
    local_BX_4.field_0x12 = 0;
    return;
}

pub fn pass1_1030_14b4(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let local_BX_11: *mut astruct_845;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut uVar6: u16;
    let mut bVar7: bool;

    uVar5 = (param_1 >> 0x10);
    local_BX_11 = param_1;
    puVar1 = &local_BX_11.field_0xa;
    unsafe {
        if ((*puVar1 < param_4 || *puVar1 == param_4) || (local_BX_11.field_0x6 == 0)) {
            puVar2 = &local_BX_11.field_0x14;
            bVar7 = *puVar2 < param_4._2_2_;
            if ((bVar7 || *puVar2 == param_4._2_2_)
                && (bVar7
                    || (
                        puVar2 = &local_BX_11.field_0x12,
                        *puVar2 < param_4 || *puVar2 == param_4,
                    )))
            {
                pass1_1030_1550((param_1 & 0xffff | uVar5 << 0x10));
            }
            puVar1 = &local_BX_11.field_0x12;
            if ((*puVar1 < param_4 || *puVar1 == param_4) || (local_BX_11.field_0x6 == 0)) {
                return;
            }
            puVar2 = &local_BX_11.field_0xc;
            bVar7 = *puVar2 < param_4._2_2_;
            if ((bVar7 || *puVar2 == param_4._2_2_)
                && (bVar7
                    || (
                        puVar2 = &local_BX_11.field_0xa,
                        *puVar2 < param_4 || *puVar2 == param_4,
                    )))
            {
                local_BX_11.field_0xa = (param_4 + 1);
                local_BX_11.field_0xc = (param_4 + 1 >> 0x10);
            }
        }
    }
    uVar3 = local_BX_11.field_0x6;
    uVar6 = (uVar3 >> 0x10);
    iVar4 = uVar3;
    (iVar4 + param_4 * 4) = param_2;
    (iVar4 + param_4 * 4 + 2) = param_3;
    return;
}

pub fn ret_1030_154c() {
    return;
}

pub fn pass1_1030_1550(param_1: *mut astruct_846) {
    let puVar1: *mut u32;
    let paVar2: *mut astruct_94;
    let mut uVar3: i32;
    let extraout_DX: *mut u16;
    let local_BX_4: *mut astruct_846;
    let mut uVar4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x12 == 0) {
        paVar2 = local_BX_4.field_0xe;
        g_u16_ptr_1050_5f2e = local_BX_4.field_0x10;
    } else {
        uVar3 = local_BX_4.field_0x12;
        puVar1 = &local_BX_4.field_0x16;
        unsafe {
            paVar2 = (uVar3 + *puVar1);
            g_u16_ptr_1050_5f2e =
                (local_BX_4.field_0x14 + local_BX_4.field_0x18 + CARRY2(uVar3, *puVar1));
        }
    }
    _local_6 = CONCAT22(g_u16_ptr_1050_5f2e, paVar2);
    if (local_BX_4.field_0x6 == 0) {
        if (__g_astruct_94_ptr_1 == 0) {
            _g_astruct_94_ptr_1 = paVar2;
            struct_fn_1000_160a();
        } else {
        }
        uVar3 = paVar2 << 2;
        alloc_mem_1000_1708(uVar3, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
    } else {
        uVar3 = paVar2 * 4;
        alloc_mem_1000_0ed4(
            1,
            uVar3,
            (g_u16_ptr_1050_5f2e * 2 + CARRY2(paVar2, paVar2)) * 2 + CARRY2(paVar2 * 2, paVar2 * 2),
            local_BX_4.field_0x6,
        );
        g_u16_ptr_1050_5f2e = extraout_DX;
    }
    local_a = CONCAT22(g_u16_ptr_1050_5f2e, uVar3);
    if ((g_u16_ptr_1050_5f2e | uVar3) != 0) {
        &local_BX_4.field_0x12 = _local_6;
        local_BX_4.field_0x6 = local_a;
    }
    return;
}

pub fn pass1_1030_15fe(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1030_1244(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_1628(param_1: *mut astruct_1) {
    let local_BX_4: *mut astruct_847;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.field_0x0 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = 0;
    local_BX_4.field_0x8 = 0;
    param_1.field_0x0 = 0x17ba;
    local_BX_4.field_0x2 = 0x1030;
    return param_1;
}

pub fn pass1_1030_165e(param_1: *mut astruct_848, param_2: u32, param_3: u32) {
    let mut extraout_DX: u16;
    let local_BX_4: *mut astruct_848;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.field_0x0 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    &local_BX_4.field_0x4 = 0;
    local_BX_4.field_0x8 = param_3;
    param_1.field_0x0 = 0x17ba;
    local_BX_4.field_0x2 = 0x1030;
    pass1_1030_5c8a(_PTR_LOOP_1050_5736, param_2);
    local_BX_4.field_0x4 = param_3;
    local_BX_4.field_0x6 = extraout_DX;
    return;
}

pub fn pass1_1030_16b2(param_1: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    unsafe {
        *param_1 = 0x17ba;
        (param_1 + 2) = 0x1030;
        *param_1 = s_1_1050_389a;
        (param_1 + 2) = &PTR_LOOP_1050_1008;
    }
    return;
}

pub fn pass1_1028_e2ac(param_1: u32) {
    let mut in_stack_00000011: u8;
    let mut local_6: u32;

    local_6 = (param_1 + in_stack_00000011 * 4 + 0x2e);
    (*local_6)();
    return;
}

pub fn pass1_1028_e2e0(param_1: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut in_stack_00000011: u8;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    local_4 = in_stack_00000011;
    if (local_4 == 0xff) {
        pass1_1028_ebee(param_1);
        return;
    }
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1 + 0x2e;
    local_a = (iVar1 + local_4 * 4 + 2);
    (**(iVar1 + local_4 * 4))();
    return;
}

pub fn pass1_1028_e332(param_1: u32, param_2: u16, uparam_3: i32) {
    let mut local_8: u32;

    if ((param_3._1_1_ != 0) && (param_3._1_1_ < 10)) {
        pass1_1030_13f6(
            (param_1 + 10 + param_3._1_1_ * 4),
            CONCAT22(param_3, param_2) & 0xffffff,
        );
    }
    return;
}

pub fn pass1_1028_e372(param_1: u32, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let ppcVar4: fn();
    let paVar5: *mut astruct_493;
    let mut uVar6: u32;
    let mut uVar7: i32;
    let mut extraout_DX: i32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u32;

    if (param_3 >> 8 != 0xff) {
        uVar1 = (param_1 + 10 + (param_3 >> 8) * 4);
        uVar2 = (uVar1 + 10);
        uVar7 = param_3 & 0xff;
        _local_10 = CONCAT22(param_3, param_2) & 0xffffff;
        paVar5 = pass1_1028_e1ec(param_1, param_2, param_3);
        uVar3 = &paVar5.field_0x8;
        paVar5 = pass1_1028_e1ec(param_1, uVar3, (uVar3 >> 0x10));
        _local_1c = CONCAT22(uVar7, paVar5);
        local_20 = 1;
        while (local_20 < uVar2) {
            if (local_20 != _local_10) {
                uVar6 = _local_10;
                pass1_1030_1312(uVar1, local_20);
                if ((extraout_DX | uVar6) != 0) {
                    uVar3 = (uVar6 + 4);
                    pass1_1030_13f6(uVar1, local_20);
                    ppcVar4 = (*_local_1c + 0x18);
                    (**ppcVar4)(0x1030, paVar5, uVar7, uVar3);
                }
            }
            local_20 = local_20 + 1;
        }
    }
    return;
}

pub fn pass1_1028_e44a(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: i32;
    let mut uVar5: u32;
    let mut extraout_DX: i32;
    let mut uVar6: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    pass1_1028_e372(param_1, param_2, (param_2 >> 0x10));
    uVar6 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x26);
    uVar2 = (param_1 + 0x1e);
    uVar3 = (uVar2 + 10);
    local_12 = 1;
    while (local_12 < uVar3) {
        uVar5 = uVar3;
        pass1_1030_1312(uVar2, local_12);
        uVar4 = uVar5;
        if (((extraout_DX | uVar4) != 0) && ((uVar4 + 8) != param_2)) {
            pass1_1030_13f6(uVar1, CONCAT22((uVar4 + 0x18), (uVar4 + 0x16)) & 0xffffff);
            pass1_1030_13f6(uVar2, local_12);
        }
        local_12 = local_12 + 1;
    }
    return;
}

pub fn pass1_1028_e4ec(param_1: *mut astruct_514) {
    let puVar1: *mut u32;
    let plVar2: *mut long;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let lVar5: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let local_BX_11: *mut astruct_514;
    let mut uVar6: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar4 = 0;
    uVar6 = (param_1 >> 0x10);
    local_BX_11 = param_1;
    if (local_BX_11.field_0x10 == 0) {
        while {
            if (local_BX_11.field_0x8 == 0) {
                return;
            }
            unsafe {
                plVar2 = &local_BX_11.field_0x8;
                *plVar2 = *plVar2 + -1;
            }
            pass1_1030_1312(local_BX_11.field_0x4, local_BX_11.field_0x8);
            (extraout_DX_00 | uVar4) == 0
        } {}
    } else {
        while {
            uVar3 = local_BX_11.field_0xc;
            puVar1 = &local_BX_11.field_0x8;
            let pu_var1_val = unsafe { *puVar1 };
            if (uVar3 <= pu_var1_val && pu_var1_val != uVar3) {
                return;
            }
            lVar5 = local_BX_11.field_0x8;
            plVar2 = &local_BX_11.field_0x8;
            unsafe { *plVar2 = *plVar2 + 1 };
            pass1_1030_1312(local_BX_11.field_0x4, lVar5);
            (extraout_DX | lVar5) == 0
        } {}
    }
    return;
}

pub fn pass1_1028_e0a0(param_1: u32, param_2: u32) {
    let mut u_var1: u32;

    uVar1 = (param_1 + 0x52);
    pass1_1030_4782(uVar1, (uVar1 >> 0x10), 1, param_2, (param_2 >> 0x10));
    return;
}

pub fn pass1_1028_e0bc(param_1: u32, param_2: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let in_AX: *mut u32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let puVar6: *mut u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x98, in_DX);
    puVar3 = in_AX;
    paVar5 = in_DX;
    pass1_1030_4bbe((param_1 + 0x52), param_2);
    iVar4 = 0x26;
    puVar6 = in_AX;
    while (iVar4 != 0) {
        iVar4 = iVar4 + -1;
        puVar2 = puVar6;
        puVar6 = puVar6 + 1;
        puVar1 = puVar3;
        puVar3 = puVar3 + 1;
        unsafe { *puVar2 = *puVar1 };
    }
    return CONCAT22(in_DX, in_AX);
}

pub fn pass1_1028_e100(param_1: u32, param_2: u16) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uVar3: i32;
    let mut in_EAX: u32;
    let mut iVar4: i32;
    let in_DX: *mut u16;
    let mut uVar5: i32;
    let puVar6: *mut u32;
    let puVar7: *mut u32;
    let mut uVar8: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (__g_astruct_94_ptr_1 == 0) {
        struct_fn_1000_160a();
        g_u16_ptr_1050_5f2e = in_DX;
    } else {
        in_EAX = __g_astruct_94_ptr_1 & 0xffff;
    }
    alloc_mem_1000_1708(0xae, 0, 1, in_EAX, g_u16_ptr_1050_5f2e);
    uVar3 = in_EAX;
    local_6 = in_EAX & 0xffff | ZEXT24(g_u16_ptr_1050_5f2e) << 0x10;
    uVar5 = g_u16_ptr_1050_5f2e | uVar3;
    if (uVar5 == 0) {
        local_6 = 0;
    } else {
        (uVar3 + 0xa4) = 0;
        (uVar3 + 0xa8) = 0;
        (uVar3 + 0xac) = 0;
        in_EAX = local_6;
    }
    puVar6 = in_EAX;
    pass1_1030_4c06((param_1 + 0x52), param_2);
    uVar8 = (local_6 >> 0x10);
    puVar7 = local_6;
    iVar4 = 0x2b;
    while (iVar4 != 0) {
        iVar4 = iVar4 + -1;
        puVar2 = puVar7;
        puVar7 = puVar7 + 1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 1;
        unsafe { *puVar2 = *puVar1 };
    }
    puVar7 = puVar6;
    return;
}

pub fn pass1_1028_e198(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let paVar1: *mut astruct_493;
    let mut in_DX: u16;

    paVar1 = pass1_1028_e1ec(param_1, param_4, (param_4 >> 0x10));
    pass1_1030_5b1c(CONCAT22(in_DX, paVar1), param_2, param_3);
    return;
}

pub fn pass1_1028_e1bc() {
    let mut local_6: u32;

    return;
}

pub fn pass1_1028_e1ec(param_1: u32, param_2: u16, param_3: u16) -> *mut astruct_493 {
    let mut u_var1: u32;
    let mut local_8: u16;
    let mut local_4: u16;

    if (param_3._1_1_ == 0x0) {
        return param_3._1_1_;
    }
    if (param_3._1_1_ == 0xff) {
        return PTR_LOOP_1050_5166;
    }
    uVar1 = (param_1 + 10 + param_3._1_1_ * 4);
    pass1_1030_1312(uVar1, param_2, param_3 & 0xff);
    return uVar1;
}

pub fn pass1_1028_d7de(param_1: *mut astruct_215, param_2: u8) -> *mut astruct_215 {
    pass1_1008_57c4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_d81c(param_1: *mut astruct_841, param_2: u32) {
    let uVar1: u8;
    let mut uVar2: i32;
    let extraout_var: u32;
    let in_DX: *mut astruct_199;
    let paVar3: *mut astruct_199;
    let paVar4: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let extraout_DX_02: *mut astruct_199;
    let extraout_DX_03: *mut astruct_199;
    let extraout_DX_04: *mut astruct_199;
    let extraout_DX_05: *mut astruct_199;
    let extraout_DX_06: *mut astruct_199;
    let mut extraout_DX_07: u16;
    let mut uVar5: u16;
    let local_BX_4: *mut astruct_841;
    let mut uVar6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = 0;
    local_BX_4.field_0x4 = param_2;
    &local_BX_4.field_0x52 = 0;
    _PTR_LOOP_1050_65e2 = param_1;
    local_BX_4.field_0x32 = 0xec36;
    local_BX_4.field_0x34 = &PTR_LOOP_1050_1028;
    local_BX_4.field_0x36 = 0xecac;
    local_BX_4.field_0x38 = &PTR_LOOP_1050_1028;
    local_BX_4.field_0x3a = 0xed2c;
    local_BX_4.field_0x3c = &PTR_LOOP_1050_1028;
    local_BX_4.field_0x3e = 0xedc4;
    local_BX_4.field_0x40 = &PTR_LOOP_1050_1028;
    local_BX_4.field_0x42 = 0xee54;
    local_BX_4.field_0x44 = &PTR_LOOP_1050_1028;
    local_BX_4.field_0x46 = 0xef00;
    local_BX_4.field_0x48 = &PTR_LOOP_1050_1028;
    local_BX_4.field_0x4a = 0x10b0;
    local_BX_4.field_0x4c = 0x1030;
    local_BX_4.field_0x4e = 0x1120;
    local_BX_4.field_0x50 = 0x1030;
    process_struct_1000_179c(8, in_DX);
    uVar2 = param_2;
    paVar3 = (in_DX | uVar2);
    if (paVar3 != 0x0) {
        pass1_1030_615a((param_2 & 0xffff | ZEXT24(in_DX) << 0x10));
    }
    process_struct_1000_179c(0x56c, paVar3);
    paVar4 = (paVar3 | uVar2);
    if (paVar4 == 0x0) {
        uVar2 = 0;
        paVar4 = 0x0;
    } else {
        pass1_1030_44be(CONCAT22(paVar3, uVar2));
    }
    local_BX_4.field_0x52 = uVar2;
    local_BX_4.field_0x54 = paVar4;
    process_struct_1000_179c(4, paVar4);
    paVar3 = (paVar4 | uVar2);
    if (paVar3 != 0x0) {
        pass1_1008_bde0(CONCAT22(paVar4, uVar2));
        paVar3 = extraout_DX;
    }
    uVar1 = pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_4.field_0xa), 0, 0x24);
    uVar2 = CONCAT31(extraout_var, uVar1);
    process_struct_1000_179c(0x1c, paVar3);
    paVar4 = (paVar3 | uVar2);
    if (paVar4 == 0x0) {
        &local_BX_4.field_0xe = 0;
    } else {
        pass1_1030_11aa(uVar2, paVar3, 5, 0x15);
        local_BX_4.field_0xe = uVar2;
        local_BX_4.field_0x10 = extraout_DX_00;
        paVar4 = extraout_DX_00;
    }
    process_struct_1000_179c(0x1c, paVar4);
    if ((paVar4 | uVar2) == 0) {
        uVar2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(uVar2, paVar4, 5, 10);
        paVar3 = extraout_DX_01;
    }
    local_BX_4.field_0x12 = uVar2;
    local_BX_4.field_0x14 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | uVar2) == 0) {
        uVar2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(uVar2, paVar3, 5, 0x19);
        paVar3 = extraout_DX_02;
    }
    local_BX_4.field_0x16 = uVar2;
    local_BX_4.field_0x18 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | uVar2) == 0) {
        uVar2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(uVar2, paVar3, 5, 10);
        paVar3 = extraout_DX_03;
    }
    local_BX_4.field_0x1a = uVar2;
    local_BX_4.field_0x1c = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | uVar2) == 0) {
        uVar2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(uVar2, paVar3, 100, 500);
        paVar3 = extraout_DX_04;
    }
    local_BX_4.field_0x1e = uVar2;
    local_BX_4.field_0x20 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | uVar2) == 0) {
        uVar2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(uVar2, paVar3, 0x19, 100);
        paVar3 = extraout_DX_05;
    }
    local_BX_4.field_0x22 = uVar2;
    local_BX_4.field_0x24 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | uVar2) == 0) {
        uVar2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(uVar2, paVar3, 100, 500);
        paVar3 = extraout_DX_06;
    }
    local_BX_4.field_0x26 = uVar2;
    local_BX_4.field_0x28 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | uVar2) == 0) {
        uVar2 = 0;
        uVar5 = 0;
    } else {
        pass1_1030_11aa(uVar2, paVar3, 5, 2);
        uVar5 = extraout_DX_07;
    }
    local_BX_4.field_0x2a = uVar2;
    local_BX_4.field_0x2c = uVar5;
    return;
}

pub fn pass1_1028_daba(param_1: *mut astruct_842) {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let puVar3: *mut u32;
    let ppcVar4: fn();
    let paVar5: *mut astruct_44;
    let local_BX_43: *mut astruct_842;
    let mut uVar6: u16;
    let mut unaff_CS: u16;
    let mut local_e: u32;
    let mut local_a: u32;

    paVar5 = _PTR_LOOP_1050_5740;
    if (_PTR_LOOP_1050_5740 != 0x0) {
        pass1_1030_61b0(_PTR_LOOP_1050_5740, (_PTR_LOOP_1050_5740 >> 0x10));
        unaff_CS = 0x1000;
        error_check_1000_17ce(paVar5);
    }
    uVar6 = (param_1 >> 0x10);
    local_BX_43 = param_1;
    uVar1 = local_BX_43.field_0x52;
    uVar2 = local_BX_43.field_0x54;
    local_e = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1030_4538(uVar1, uVar2);
        unaff_CS = 0x1000;
        error_check_1000_17ce(local_e);
    }
    if (_PTR_LOOP_1050_5166 != 0x0) {
        ppcVar4 = *_PTR_LOOP_1050_5166;
        (**ppcVar4)(unaff_CS, _PTR_LOOP_1050_5166);
    }
    paVar5 = _PTR_LOOP_1050_06e0;
    _PTR_LOOP_1050_65e2 = 0;
    if (_PTR_LOOP_1050_06e0 != 0x0) {
        pass1_1008_c626(_PTR_LOOP_1050_06e0);
        unaff_CS = 0x1000;
        error_check_1000_17ce(paVar5);
    }
    puVar3 = local_BX_43.field_0xe;
    uVar1 = local_BX_43.field_0x10;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    puVar3 = local_BX_43.field_0x12;
    uVar1 = local_BX_43.field_0x14;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    puVar3 = local_BX_43.field_0x16;
    uVar1 = local_BX_43.field_0x18;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    puVar3 = local_BX_43.field_0x1a;
    uVar1 = local_BX_43.field_0x1c;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    puVar3 = local_BX_43.field_0x1e;
    uVar1 = local_BX_43.field_0x20;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    puVar3 = local_BX_43.field_0x22;
    uVar1 = local_BX_43.field_0x24;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    puVar3 = local_BX_43.field_0x26;
    uVar1 = local_BX_43.field_0x28;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    puVar3 = local_BX_43.field_0x2a;
    uVar1 = local_BX_43.field_0x2c;
    if ((uVar1 | puVar3) != 0) {
        unsafe {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
    }
    return;
}

pub fn pass1_1028_dc52(param_1: *mut astruct_374, param_2: *mut u8, param_3: u16, param_4: u16) {
    let local_BX_5: *mut astruct_374;
    let mut local_ES_5: u16;
    let mut temp_5f1f4e4cff: u32;

    local_ES_5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1 = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_5.field_0x4 = (_PTR_LOOP_1050_65e2 + (param_4 >> 8) * 4 + 10);
    local_BX_5.field_0x8 = 1;
    local_BX_5.field_0x10 = param_2;
    param_1 = 0x11a6;
    local_BX_5.field_0x2 = 0x1030;
    temp_5f1f4e4cff = local_BX_5.field_0x4;
    local_BX_5.field_0xc = (temp_5f1f4e4cff + 10);
    if (param_2 == 0x0) {
        local_BX_5.field_0x8 = local_BX_5.field_0xc;
    } else {
        local_BX_5.field_0x8 = 1;
    }
    return param_1;
}

pub fn pass1_1028_bab6(param_1: u32) {
    let mut u_var1: u32;

    uVar1 = pass1_1028_bad4(param_1);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    return;
}

pub fn pass1_1028_bad4(param_1: u32) {
    let mut in_AX: i32;
    let mut in_DX: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_baf6(param_1);
    return CONCAT22((in_AX + 10), (in_AX + 8));
}

pub fn pass1_1028_baf6(param_1: *mut u8) {
    let mut u_var1: u32;
    let mut local_4: u16;

    uVar1 = pass1_1028_bb24(param_1);
    if (uVar1 == 0) {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    return;
}

pub fn pass1_1028_bb24(param_1: *mut u8) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut uVar2: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 8) == 0) {
        return 0;
    }
    uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar2 << 0x10));
    return CONCAT22(
        (CONCAT11(extraout_AH, uVar1) + 10),
        (CONCAT11(extraout_AH, uVar1) + 8),
    );
}

pub fn pass1_1028_bb56(param_1: u32, param_2: u32) {
    pass1_1030_177a(param_1, param_2);
    return;
}

pub fn pass1_1028_bb6a(param_1: *mut astruct_830) {
    let local_BX_3: *mut astruct_830;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if ((local_BX_3.field_0x12 != 5) && (local_BX_3.field_0x12 != 6)) {
        return 0;
    }
    return CONCAT22(local_BX_3.field_0x16, local_BX_3.field_0x14 + 0xa4);
}

pub fn pass1_1028_bb96(param_1: u32, param_2: *mut u32, param_3: u16) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let local_BX_5: *mut astruct_831;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut uVar7: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if ((local_BX_5.field_0x12 == 5) || (local_BX_5.field_0x12 == 6)) {
        uVar3 = local_BX_5.field_0x14;
        uVar7 = (uVar3 >> 0x10);
        puVar5 = (uVar3 + 0xa4);
        iVar4 = 2;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = param_2;
            param_2 = param_2 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        puVar5 = param_2;
        pass1_1028_c724(param_1);
        uVar3 = local_BX_5.field_0x14;
        uVar6 = (uVar3 >> 0x10);
        iVar4 = uVar3;
        if ((iVar4 + 0xaa) == 0) {
            (iVar4 + 0xaa) = 1;
        }
    }
    return;
}

pub fn pass1_1028_bbf0(param_1: u16, param_2: u16, param_1_00: *mut u32) {
    unsafe { *param_1_00 = 0 };
    return;
}

pub fn pass1_1028_bc02(param_1: *mut u32) {
    let ppcVar1: fn();

    unsafe {
        ppcVar1 = (*param_1 + 0x40);
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1028_bc1c(param_1: *mut astruct_832) {
    let local_BX_3: *mut astruct_832;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x12 == 4) {
        return local_BX_3.field_0xe;
    }
    if (local_BX_3.field_0x12 == 7) {
        return local_BX_3.field_0x10;
    }
    return local_BX_3.field_0xc;
}

pub fn pass1_1028_bc4a(param_1: u32) {
    let mut uVar1: u16;
    let in_struct_1: *mut astruct_44;
    let mut in_stack_0000fff0: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    in_struct_1 = pass1_1028_e0bc(
        _PTR_LOOP_1050_65e2,
        CONCAT22(in_stack_0000fff0, (param_1 + 0xc)),
    );
    uVar1 = (in_struct_1 + 0x96);
    error_check_1000_17ce(in_struct_1);
    return uVar1;
}

pub fn pass1_1028_bc7e(param_1: u32) {
    pass1_1028_bdac(param_1, 4);
    return;
}

pub fn pass1_1028_bc90(param_1: *mut u32, param_2: *mut u8, param_3: u32, param_4: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;

    iVar3 = pass1_1028_c7b6(param_1, param_2, param_4);
    if ((iVar3 == 5) || (iVar3 == 6)) {
        uVar4 = (param_1 >> 0x10);
        unsafe { uVar2 = *param_1 };
        ppcVar1 = (uVar2 + 0x60);
        iVar3 = (**ppcVar1)();
        if (iVar3 != 0) {
            ppcVar1 = (uVar2 + 0x5c);
            iVar3 = (**ppcVar1)();
            if (iVar3 != 0) {
                pass1_1028_c23e(param_1, uVar4, param_2, param_3, param_4);
                if (iVar3 != 0) {
                    uVar4 = pass1_1028_c314(
                        param_1,
                        uVar4,
                        param_2,
                        param_3,
                        (param_3 >> 0x10),
                        param_4,
                    );
                    if (uVar4 != 0) {
                        return 1;
                    }
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1028_bd38(param_1: *mut astruct_44) {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let puVar3: *mut u8;
    let uVar4: u8;
    let mut uVar5: u32;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut uVar6: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar5 = (_PTR_LOOP_1050_65e2 + 0x52);
    pass1_1030_4bbe(uVar5, (param_1 + 0xc));
    uVar6 = in_DX;
    uVar4 = pass1_1028_b58e(param_1);
    puVar3 = *(CONCAT31(extraout_var, uVar4) + 0x2e);
    local_14 = 0x11;
    while {
        uVar1 = (local_14 * 4 + uVar5);
        uVar2 = (local_14 * 4 + uVar5 + 2);
        if ((uVar2 | uVar1) != 0) {
            pass1_1038_5770(puVar3, CONCAT22(uVar2, uVar1), local_14);
        }
        local_14 = local_14 + 1;
        local_14 < 0x25
    } {}
    return;
}

pub fn pass1_1028_bdac(param_1: *mut astruct_833, param_2: i32) {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let local_BX_7: *mut astruct_833;
    let mut uvar3: u16;
    let mut unaff_CS: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_7 = param_1;
    if (local_BX_7.field_0x12 != param_2) {
        if (local_BX_7.field_0x12 == 6) {
            if (local_BX_7.field_0x18 == param_2) {
                local_BX_7.field_0x12 = local_BX_7.field_0x18;
                local_BX_7.field_0x18 = 0;
                return;
            }
        } else {
            if (param_2 != 6) {
                iVar1 = local_BX_7.field_0x12;
                if ((iVar1 == 4) || (iVar1 == 5)) {
                    unaff_CS = 0x1000;
                    error_check_1000_17ce(local_BX_7.field_0x14);
                }
                local_BX_7.field_0x12 = param_2;
                ppcVar2 = (param_1 + 0x3c);
                ppcVar2(unaff_CS, param_1);
                return;
            }
            local_BX_7.field_0x18 = local_BX_7.field_0x12;
            local_BX_7.field_0x12 = 6;
        }
    }
    return;
}

pub fn pass1_1028_be2a(param_1: *mut astruct_44) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x12) != 6) {
        return;
    }
    uVar5 = pass1_1028_b4f2(param_1);
    if ((uVar5 + 0x200) != 0x8000002) {
        if ((iVar3 + 0x1c) == 0x8000002) {
            iVar3 = 6;
            // goto code_r0x1028be96;
        }
        ppcVar1 = (param_1 + 100);
        iVar2 = (**ppcVar1)();
        if (iVar2 == 0) {
            return;
        }
        iVar3 = pass1_1028_cb04(iVar3, uVar4);
        if (iVar3 == 0) {
            iVar3 = 6;
            // goto code_r0x1028be96;
        }
        pass1_1028_c952(param_1);
    }
    iVar3 = 5;
    // code_r0x1028be96:
    pass1_1028_bdac(param_1, iVar3);
    return;
}

pub fn pass1_1028_be9e(param_1: *mut astruct_44) {
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let local_BX_4: *mut astruct_834;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x12 == 4) {
        uVar5 = pass1_1028_b4f2(param_1);
        if ((uVar5 + 0x200) == 0x8000002) {
            uVar2 = local_BX_4.field_0x14;
            piVar1 = (uVar2 + 0x94);
            unsafe { *piVar1 = *piVar1 + -1 };
        } else {
            iVar3 = pass1_1028_cb04(local_BX_4, uVar4);
            if (iVar3 == 0) {
                return;
            }
            uVar2 = local_BX_4.field_0x14;
            piVar1 = (uVar2 + 0x94);
            unsafe { *piVar1 = *piVar1 + -1 };
            pass1_1028_c952(param_1);
        }
        uVar2 = local_BX_4.field_0x14;
        if ((uVar2 + 0x94) < 1) {
            pass1_1028_bdac(param_1, 5);
        }
    }
    return;
}

pub fn pass1_1028_bf22(param_1: *mut astruct_835) {
    let mut iVar1: i32;
    let mut in_DX: u16;
    let local_BX_3: *mut astruct_835;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let mut uVar3: u32;

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    iVar1 = local_BX_3.field_0x12;
    if (iVar1 == 4) {
        uVar3 = pass1_1028_e0bc(
            _PTR_LOOP_1050_65e2,
            CONCAT22(unaff_BP, local_BX_3.field_0xc),
        );
    } else {
        iVar1 = iVar1 + -5;
        if (iVar1 != 0) {
            if (iVar1 != 1) {
                &local_BX_3.field_0x14 = 0;
            }
            return;
        }
        pass1_1028_e100(_PTR_LOOP_1050_65e2, local_BX_3.field_0xc);
        uVar3 = CONCAT22(in_DX, iVar1);
    }
    local_BX_3.field_0x14 = uVar3;
    &local_BX_3.field_0x16 = (uVar3 >> 0x10);
    return;
}

pub fn pass1_1028_bf76(param_1: *mut astruct_764) {
    let mut iVar1: i32;
    let BVar2: bool;
    let local_BX_23: *mut astruct_764;
    let mut uvar3: u16;
    let mut local_4: u16;

    iVar1 = pass1_fn_1008_612e();
    uVar3 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, &local_BX_23.field_0xc, 0x28);
    if (BVar2 == 0) {
        if (iVar1 == 1) {
            &local_BX_23.field_0x10 = 0x48;
            return;
        }
        if (iVar1 != 2) {
            &local_BX_23.field_0x10 = 0x4a;
            return;
        }
        &local_BX_23.field_0x10 = 0x49;
        return;
    }
    if (iVar1 == 1) {
        &local_BX_23.field_0x10 = 0x70;
        return;
    }
    if (iVar1 != 2) {
        &local_BX_23.field_0x10 = 0x72;
        return;
    }
    &local_BX_23.field_0x10 = 0x71;
    return;
}

pub fn pass1_1028_c00a(param_1: *mut astruct_44, param_2: libc::c_long) {
    let paVar1: *mut astruct_1115;
    let ppcVar2: fn(a: u16, b: u16, c: u16);
    let uVar3: u8;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let paVar6: *mut astruct_493;
    let extraout_var: u32;
    let puVar7: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut uVar8: u16;
    let mut uVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = pass1_1028_b58e(param_1);
    paVar1 = (CONCAT31(extraout_var, uVar3) + 0x2e);
    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
    uVar4 = SUB42(puVar7, 0);
    uVar10 = SUB42(&PTR_LOOP_1050_1038, 0);
    pass1_1038_4d6e(paVar1, puVar7 & 0xffff | in_DX << 0x10);
    _local_12 = CONCAT22(extraout_DX, uVar4);
    ppcVar2 = (*_local_12 + 0x10);
    uVar5 = uVar4;
    ppcVar2(&PTR_LOOP_1050_1038, uVar4, extraout_DX);
    _local_16 = CONCAT22(extraout_DX_00, uVar5);
    local_1a = 0;
    loop {
        if (_local_16 <= local_1a) {
            // LAB_1028_c0d6:
            if (_local_12 != 0x0) {
                ppcVar2 = *_local_12;
                ppcVar2(uVar10, uVar4, extraout_DX, 1);
            }
            return;
        }
        ppcVar2 = (*_local_12 + 4);
        uVar11 = _local_16;
        ppcVar2(uVar10, uVar4, extraout_DX, local_1a, (local_1a >> 0x10));
        uVar8 = extraout_DX_01;
        paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar11, extraout_DX_01);
        uVar10 = 0x1030;
        uVar11 = pass1_1030_73a8(CONCAT22(uVar8, paVar6));
        uVar12 = pass1_1028_6302(uVar11);
        uVar9 = (uVar12 >> 0x10);
        if ((param_2._2_2_ <= uVar9) && (param_2._2_2_ < uVar9 || (param_2 <= uVar12))) {
            pass1_1028_6356(uVar11, 0, param_2, param_2._2_2_);
            // goto LAB_1028_c0d6;
        }
        pass1_1028_6356(uVar11, 0, uVar12, uVar9);
        param_2 = param_2 - uVar12;
        local_1a = local_1a + 1;
    }
}

pub fn pass1_1028_c0f0(param_1: *mut astruct_44, param_2: libc::c_long) {
    let paVar1: *mut astruct_1115;
    let ppcVar2: fn();
    let uVar3: u8;
    let mut uVar4: u16;
    let paVar5: *mut astruct_493;
    let extraout_var: u32;
    let puVar6: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut uVar7: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut uVar8: u16;
    let mut uVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut uVar13: u32;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = pass1_1028_b58e(param_1);
    paVar1 = (CONCAT31(extraout_var, uVar3) + 0x2e);
    pass1_1028_cb04(param_1, (param_1 >> 0x10));
    uVar10 = (paVar1 >> 0x10);
    if (((paVar1 + 0x204) == 0) && ((paVar1 + 0x206) == 0)) {
        uVar7 = extraout_DX;
        puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
        uVar4 = SUB42(puVar6, 0);
        uVar11 = SUB42(&PTR_LOOP_1050_1038, 0);
        pass1_1038_4d6e(paVar1, puVar6 & 0xffff | uVar7 << 0x10);
        _local_14 = CONCAT22(extraout_DX_00, uVar4);
        ppcVar2 = (*_local_14 + 0x10);
        uVar10 = uVar4;
        ppcVar2(&PTR_LOOP_1050_1038, uVar4, extraout_DX_00);
        _local_18 = CONCAT22(extraout_DX_01, uVar10);
        local_1c = 0;
        while (local_1c < _local_18) {
            ppcVar2 = (*_local_14 + 4);
            uVar12 = _local_18;
            ppcVar2(uVar11, uVar4, extraout_DX_00, local_1c, (local_1c >> 0x10));
            uVar8 = extraout_DX_02;
            paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar12, extraout_DX_02);
            uVar11 = 0x1030;
            uVar12 = pass1_1030_73a8(CONCAT22(uVar8, paVar5));
            uVar13 = pass1_1028_6302(uVar12);
            uVar9 = (uVar13 >> 0x10);
            uVar7 = uVar13;
            if ((param_2._2_2_ <= uVar9) && (param_2._2_2_ < uVar9 || (param_2 <= uVar7))) {
                param_2 = 0;
                break;
            }
            param_2 = CONCAT22((param_2._2_2_ - uVar9) - (param_2 < uVar7), param_2 - uVar7);
            local_1c = local_1c + 1;
        }
        if (_local_14 != 0x0) {
            ppcVar2 = *_local_14;
            ppcVar2(uVar11, uVar4, extraout_DX_00, 1);
        }
        if (param_2 != 0) {
            pass1_1030_7d7c(
                (CONCAT31(extraout_var, uVar3) & 0xffff | in_DX << 0x10),
                param_2,
                CONCAT22(0x1d, (param_2 >> 0x10)),
            );
        }
    }
    return;
}

pub fn pass1_1028_c1f8(param_1: *mut u8, param_2: *mut u8, param_3: *mut u8) {
    let mut in_AX: i32;
    let mut in_DX: u16;
    let puVar1: *mut u32;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_baf6(param_1);
    puVar1 = pass1_1030_5b5c(in_AX, in_DX);
    unsafe { local_c = *puVar1 };
    uStack8 = (puVar1 + 4);
    pass1_1008_3e94(&local_c, param_2, param_3);
    return;
}

pub fn pass1_1028_c23e(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: libc::c_long,
    param_5: u32,
) -> i32 {
    let lVar1: u32;
    let ppcVar2: fn();
    let mut in_AX: i32;
    let paVar3: *mut astruct_493;
    let paVar4: *mut astruct_493;
    let mut extraout_DX: i32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_5);
    _local_6 = CONCAT22(extraout_DX, in_AX);
    uVar5 = extraout_DX | in_AX;
    if (uVar5 != 0) {
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, in_AX, extraout_DX);
        _local_a = CONCAT22(uVar5, paVar3);
        lVar1 = &paVar3[1].field_0xc;
        if (lVar1 != param_2_00) {
            paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar1, (lVar1 >> 0x10));
            _local_12 = CONCAT22(uVar5, paVar3);
            uVar6 = uVar5;
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
            _local_16 = CONCAT22(uVar6, paVar4);
            if (((_local_12 == 0x0) || ((uVar6 | paVar4) == 0))
                || (&paVar4[0x11].field_0x2 != &paVar3[0x11].field_0x2))
            {
                return;
            }
            ppcVar2 = (*_local_12 + 0x18);
            ppcVar2(0x1030, paVar3, uVar5, _local_6);
            ppcVar2 = (*_local_16 + 8);
            ppcVar2();
            pass1_1030_73ee(_local_a, param_2_00);
        }
    }
    return;
}

pub fn pass1_1028_c314(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u16,
    param_4: u16,
    param_5: u16,
    param_2_00: u32,
) -> i32 {
    let mut in_DX: u16;
    let mut unaff_SS: u16;
    let puVar1: *mut u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    puVar1 = pass1_1030_5b5c(local_6, in_DX);
    unsafe { local_c = *puVar1 };
    uStack8 = (puVar1 + 4);
    pass1_1008_3e94(
        param_1_00,
        CONCAT22(unaff_SS, &local_10),
        CONCAT22(unaff_SS, &local_e),
    );
    pass1_1008_3e94(
        &local_c,
        CONCAT22(unaff_SS, &local_14),
        CONCAT22(unaff_SS, &local_12),
    );
    if ((((1 < local_e) && (1 < local_10)) && (local_e < (local_12 - 1)))
        && (local_10 < (local_14 - 1)))
    {
        return 1;
    }
    PTR_LOOP_1050_50ca = 0x6b8;
    return 0;
}

pub fn pass1_1028_c3aa(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let paVar3: *mut astruct_493;
    let paVar4: *mut astruct_493;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let puVar7: *mut u8;
    let puVar8: *mut u8;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut extraout_DX: i32;
    let mut uVar12: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let mut uVar13: u16;
    let mut unaff_SS: u16;
    let mut uVar14: u32;
    let ppVar15: *mut pass1_struct_1;
    let uVar16: u8;
    let uVar17: u8;
    let mut uVar18: u16;
    let mut uVar19: u16;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
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
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    uVar14 = pass1_1030_bcae(local_4, unaff_SS);
    uVar10 = (uVar14 >> 0x10);
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    uVar1 = &paVar3.field_0x10;
    uVar16 = param_1_00;
    uVar17 = (param_1_00 >> 8);
    uVar18 = (param_1_00 >> 0x10);
    uVar11 = uVar10;
    uVar14 = param_5;
    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    local_18 = local_4;
    pass1_1030_bcde(
        local_18,
        unaff_SS,
        CONCAT22(uVar11, paVar4),
        CONCAT22(uVar18, CONCAT11(uVar17, uVar16)),
        uVar14,
    );
    if (local_18 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    if (0x1e < local_18) {
        uVar19 = 0x87;
        ppVar15 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x870009);
        iVar5 = ppVar15;
        pass1_1010_65d0(ppVar15, uVar19);
        if (iVar5 == 0) {
            uVar12 = extraout_DX;
            puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x15);
            uVar6 = SUB42(puVar8, 0);
            uVar13 = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_4d6e(CONCAT22(uVar10, paVar3), puVar8 & 0xffff | uVar12 << 0x10);
            _local_20 = CONCAT22(extraout_DX_00, uVar6);
            ppcVar2 = (*_local_20 + 0x10);
            uVar11 = uVar6;
            uVar10 = uVar6;
            uVar18 = extraout_DX_00;
            ppcVar2(&PTR_LOOP_1050_1038, uVar6, extraout_DX_00);
            _local_24 = CONCAT22(extraout_DX_01, uVar11);
            local_28 = 0;
            while (true) {
                if (_local_24 <= local_28) {
                    if (_local_20 != 0x0) {
                        ppcVar2 = *_local_20;
                        ppcVar2(
                            uVar13,
                            uVar6,
                            extraout_DX_00,
                            1,
                            uVar10,
                            uVar18,
                            _local_20,
                            _local_20,
                        );
                    }
                    PTR_LOOP_1050_50ca = 0x6b6;
                    PTR_LOOP_1050_50cc = (local_18 - 0x1e);
                    return;
                }
                uVar16 = param_5;
                uVar17 = (param_5 >> 8);
                uVar9 = _local_24;
                uVar14 = param_1_00;
                uVar11 = (param_5 >> 0x10);
                pass1_1030_1d58(_local_20);
                puVar7 = local_4;
                uVar13 = 0x1030;
                pass1_1030_bcde(
                    puVar7,
                    unaff_SS,
                    (uVar9 & 0xffff | extraout_DX_02 << 0x10),
                    uVar14,
                    CONCAT22(uVar11, CONCAT11(uVar17, uVar16)),
                );
                if ((0 < puVar7) && (puVar7 < 0x1f)) {
                    break;
                }
                if (puVar7 < local_18) {
                    local_18 = puVar7;
                }
                local_28 = local_28 + 1;
            }
            if (_local_20 == 0x0) {
                return;
            }
            ppcVar2 = *_local_20;
            ppcVar2(
                0x1030,
                uVar6,
                extraout_DX_00,
                1,
                uVar10,
                uVar18,
                _local_20,
                _local_20,
                uVar9,
                extraout_DX_02,
            );
            return;
        }
    }
    return;
}

pub fn pass1_1028_c522(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let local_AX_25: *mut astruct_836;
    let paVar2: *mut astruct_493;
    let local_AX_79: *mut astruct_836;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut uVar4: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    uVar4 = pass1_1030_bcae(local_4, unaff_SS);
    uVar3 = (uVar4 >> 0x10);
    local_AX_25 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    uVar1 = local_AX_25.field_0x10;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    local_AX_79 = local_4;
    pass1_1030_bcde(
        local_AX_79,
        unaff_SS,
        CONCAT22(uVar3, paVar2),
        param_1_00,
        param_5,
    );
    if (local_AX_79 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if (local_AX_79 < 0x1f) {
            return;
        }
        PTR_LOOP_1050_50ca = 0x6b6;
        PTR_LOOP_1050_50cc = &local_AX_79[-2].field_0xa;
    }
    return;
}

pub fn pass1_1028_c5a6(
    param_1: u16,
    param_2: u16,
    param_1: u16_00,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut uVar1: u16;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let lVar5: u32;
    let mut uVar6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_2_00, param_5);
    uVar3 = (lVar5 >> 0x10);
    uVar4 = uVar3 | lVar5;
    if (lVar5 != 0) {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar5, uVar3);
        _local_a = CONCAT22(uVar4, paVar2);
        local_e = 0x7a;
        if (0 < (param_2_00 + 4)) {
            if (param_1_00 == 0x7b) {
                param_1_00 = 0x7e;
            } else {
                if (param_1_00 == 0x7c) {
                    param_1_00 = 0x7d;
                }
            }
            local_e = 0x7f;
        }
        if (_local_a != 0x0) {
            uVar6 = pass1_1030_73a8(_local_a);
            if ((uVar6 != 0)
                && ((
                    uVar1 = (uVar6 + 0xc),
                    uVar1 == local_e || (uVar1 == param_1_00),
                )))
            {
                return 1;
            }
        }
    }
    return 0;
}

pub fn pass1_1028_c64a(
    param_1: u32,
    param_2: *mut u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_4_00: u32,
) -> i32 {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    unsafe { _local_8 = *param_2 };
    uStack4 = (param_2 + 1);
    pass1_1008_3eb4(
        CONCAT22(unaff_SS, &local_8),
        CONCAT22(unaff_SS, local_e),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    uVar1 = pass1_1028_c5a6(uVar2, uVar3, 0x7b, CONCAT22(unaff_SS, &local_8), param_4_00);
    if (uVar1 == 0) {
        _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
        uVar1 = pass1_1028_c5a6(uVar2, uVar3, 0x7b, CONCAT22(unaff_SS, &local_8), param_4_00);
        if (uVar1 == 0) {
            local_8 = local_a - 1;
            local_6 = local_c;
            uVar1 = pass1_1028_c5a6(uVar2, uVar3, 0x7c, CONCAT22(unaff_SS, &local_8), param_4_00);
            if (uVar1 == 0) {
                _local_8 = CONCAT22(local_6, local_a + 1);
                uVar1 =
                    pass1_1028_c5a6(uVar2, uVar3, 0x7c, CONCAT22(unaff_SS, &local_8), param_4_00);
                if (uVar1 == 0) {
                    return uVar1;
                }
            }
        }
    }
    return 1;
}

pub fn pass1_1028_c724(param_1: u32) {
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar2 = (iVar3 + 0x14);
    if ((uVar2 + 0xac) != 0) {
        return;
    }
    uVar2 = (iVar3 + 0x14);
    uVar1 = (uVar2 + 0xa6);
    if (uVar1 == 0xd) {
        uVar2 = (iVar3 + 0x14);
        (uVar2 + 0xac) = 1;
        // goto LAB_1028_c770;
    }
    if (uVar1 < 0xe) {
        if (uVar1 == 0) {}
        // goto LAB_1028_c770;
        if (uVar1 == 0x7) {
            uVar2 = (iVar3 + 0x14);
            (uVar2 + 0xac) = 10;
            // goto LAB_1028_c770;
        }
    }
    uVar2 = (iVar3 + 0x14);
    (uVar2 + 0xac) = 5;
    // LAB_1028_c770:
    uVar2 = (iVar3 + 0x14);
    if ((uVar2 + 0xac) == 0) {
        uVar2 = (iVar3 + 0x14);
        if ((uVar2 + 0xa8) != 0) {
            uVar2 = (iVar3 + 0x14);
            (uVar2 + 0xac) = 1;
        }
        return;
    }
    return;
}

pub fn pass1_1028_c7b6(param_1: u16_00, param_2: u16_00, param_1: u32, param_2: u32) -> i32 {
    let puVar1: *mut u32;
    let paVar2: *mut astruct_493;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    puVar1 = &local_a;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_1, param_2, puVar1, unaff_SS);
    unsafe { local_6 = *puVar1 };
    uVar3 = (puVar1 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ == '\0') {
        return;
    }
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, uVar3);
    uVar5 = pass1_1030_73a8(CONCAT22(uVar3, paVar2));
    uVar4 = (uVar5 >> 0x10);
    if ((uVar4 | uVar5) != 0) {
        match (uVar5 + 0xc) {
            1 | 2 | 3 | 4 | 5 | 6 => {}
            7 | 8 | 9 => {
                return;
            }
        }
        return;
    }
    return;
}

pub fn pass1_1028_c89c(param_1: *mut astruct_44, param_2: u32, param_3: *mut u32) {
    let uVar1: u8;
    let puVar2: *mut u32;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut unaff_SS: u16;
    let mut local_16: [u32; 2];
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10;
    local_a = (CONCAT31(extraout_var, uVar1) + 8);
    puVar2 = local_16;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, local_a, puVar2, unaff_SS);
    unsafe { *param_3 = *puVar2 };
    return;
}

pub fn pass1_1028_c8ee(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: *mut u16) {
    let mut unaff_SS: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3eb4(
        param_2_00,
        CONCAT22(unaff_SS, &local_8),
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    if (param_1_00 == 1) {
        local_8 = local_8 + 1;
    } else {
        if (param_1_00 == 2) {
            _local_6 = _local_6 & 0xffff0000 | (local_6 - 1);
        } else {
            if (param_1_00 == 3) {
                _local_6 = _local_6 & 0xffff0000 | (local_6 + 1);
            } else {
                if (param_1_00 == 4) {
                    _local_6 = _local_6 & 0xffff | (local_4 + 1) << 0x10;
                } else {
                    if (param_1_00 == 5) {
                        _local_6 = _local_6 & 0xffff | (local_4 - 1) << 0x10;
                    }
                }
            }
        }
    }
    pass1_1008_3e76(param_2_00, local_8, _local_6, (_local_6 >> 0x10));
    return;
}

pub fn pass1_1028_c952(param_1: *mut astruct_44) {
    let paVar1: *mut astruct_1121;
    let uVar2: u8;
    let local_AX_22: *mut astruct_837;
    let BVar3: bool;
    let mut uVar4: i32;
    let extraout_var: u32;
    let mut uVar5: u32;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let mut uVar8: i32;
    let local_BX_6: *mut astruct_44;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_1e: u32;
    let mut local_18: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar9 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    uVar5 = local_BX_6.field_0x14;
    local_AX_22 = uVar5;
    uVar6 = (&local_BX_6.field_0x14 + 2) | local_AX_22;
    if (uVar6 != 0) {
        uVar2 = pass1_1028_b58e(param_1);
        iVar7 = CONCAT31(extraout_var, uVar2);
        paVar1 = (iVar7 + 0x2e);
        local_e._0_2_ = paVar1;
        if ((((iVar7 + 0x30) | local_e) != 0)
            && (uVar10 = (paVar1 >> 0x10), (local_e + 0x206) == 0))
        {
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, &local_BX_6.field_0xc, 0x32);
            if (BVar3 == 0) {
                BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, &local_BX_6.field_0xc, 0x33);
                if ((BVar3 != 0) && ((*_PTR_LOOP_1050_65e2 % 5) == 0)) {
                    return;
                }
            } else {
                if ((*_PTR_LOOP_1050_65e2 % 10) == 0) {
                    return;
                }
            }
            uVar9 = (uVar5 >> 0x10);
            if ((local_e + 0x204) == 0) {
                local_10 = 0;
                while (local_10 < 0x25) {
                    local_1e = (&local_AX_22.field_0x0 + local_10 * 4);
                    uVar6 = local_1e;
                    uVar8 = (&local_AX_22.field_0x2 + local_10 * 4) | uVar6;
                    if (uVar8 != 0) {
                        uVar5 = local_1e;
                        pass1_1038_540a(local_e, uVar10, local_10);
                        local_1e._2_2_ = (local_1e >> 0x10);
                        if ((uVar5 & 0xffff | uVar8 << 0x10) < local_1e) {
                            uVar4 = uVar6 - uVar5;
                            iVar7 = (local_1e._2_2_ - uVar8) - (uVar6 < uVar5);
                            pass1_1038_52b8(paVar1, CONCAT22(iVar7, uVar4), 0x21);
                            local_1e =
                                CONCAT22((local_1e._2_2_ - iVar7) - (uVar6 < uVar4), uVar6 - uVar4);
                        }
                        if ((local_1e._2_2_ | local_1e) != 0) {
                            pass1_1038_52b8(paVar1, local_1e, local_10);
                        }
                    }
                    local_10 = local_10 + 1;
                }
            } else {
                uVar6 = local_AX_22.field_0x8c;
                uVar8 = local_AX_22.field_0x8e;
                if ((uVar8 | uVar6) != 0) {
                    pass1_1038_52b8(paVar1, CONCAT22(uVar8, uVar6), 0x23);
                }
                uVar6 = local_AX_22.field_0x90;
                uVar8 = local_AX_22.field_0x92;
                if ((uVar8 | uVar6) != 0) {
                    pass1_1038_52b8(paVar1, CONCAT22(uVar8, uVar6), 0x24);
                    return;
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_cb04(param_1: *mut astruct_44) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let paVar3: *mut astruct_939;
    let uVar4: u8;
    let mut uVar5: i32;
    let extraout_var: u32;
    let mut uVar6: u32;
    let lVar7: u32;
    let mut uVar8: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let local_BX_92: *mut astruct_839;
    let mut unaff_SI: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;
    let ppVar12: *mut pass1_struct_1;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_26: u32;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let temp_7ffdd6893f6: *mut astruct_840;

    lVar7 = (param_1 + 0x14);
    if (lVar7 != 0) {
        uVar4 = pass1_1028_b58e(param_1);
        temp_7ffdd6893f6 = CONCAT31(extraout_var, uVar4);
        paVar3 = (CONCAT31(extraout_var, uVar4) & 0xffff | in_DX << 0x10);
        uVar1 = &temp_7ffdd6893f6.field_0x2e;
        uVar5 = temp_7ffdd6893f6.field_0x30;
        local_e._0_2_ = uVar1;
        local_12 = uVar5 | local_e;
        if (local_12 != 0) {
            uVar9 = (uVar1 >> 0x10);
            if ((local_e + 0x206) != 0) {
                return;
            }
            local_BX_92 = lVar7;
            uVar10 = (lVar7 >> 0x10);
            if ((local_e + 0x204) != 0) {
                uVar1 = local_BX_92.field_0x8c;
                uVar6 = uVar1;
                pass1_1038_540a(local_e, uVar9, 0x23);
                local_26._2_2_ = (uVar1 >> 0x10);
                if ((extraout_DX <= local_26._2_2_)
                    && ((
                        uVar5 = uVar6,
                        local_26._0_2_ = uVar1,
                        extraout_DX < local_26._2_2_ || (uVar5 < local_26),
                    )))
                {
                    pass1_1030_7d7c(
                        paVar3,
                        local_26 - uVar5,
                        CONCAT22(0x23, (local_26._2_2_ - extraout_DX) - (local_26 < uVar5)),
                    );
                    ppVar12 = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(unaff_SI, 0x2b),
                    );
                    pass1_1010_043a(ppVar12, (local_e + 4), 0x12);
                }
                uVar1 = local_BX_92.field_0x90;
                uVar6 = uVar1;
                pass1_1038_540a(local_e, uVar9, 0x24);
                local_26._2_2_ = (uVar1 >> 0x10);
                if ((extraout_DX_00 <= local_26._2_2_)
                    && ((
                        uVar5 = uVar6,
                        local_26._0_2_ = uVar1,
                        extraout_DX_00 < local_26._2_2_ || (uVar5 < local_26),
                    )))
                {
                    pass1_1030_7d7c(
                        paVar3,
                        local_26 - uVar5,
                        CONCAT22(0x24, (local_26._2_2_ - extraout_DX_00) - (local_26 < uVar5)),
                    );
                }
                return;
            }
            pass1_1038_540a(local_e, uVar5, 0x21);
            local_16 = 0x11;
            local_10 = extraout_DX_01;
            while (local_16 < 0x25) {
                uVar2 = (&local_BX_92.field_0x0 + local_16 * 4);
                uVar8 = uVar2;
                pass1_1038_540a(local_e, uVar9, local_16);
                uVar8 = uVar8 & 0xffff | extraout_DX_02 << 0x10;
                local_36._2_2_ = (uVar2 >> 0x10);
                if (uVar8 < uVar2) {
                    if ((((local_16 == 0x23) || (local_16 == 0x24)) || (local_10 < local_36._2_2_))
                        || ((
                            uVar5 = uVar2,
                            local_10 <= local_36._2_2_ && (local_12 < uVar5),
                        )))
                    {
                        lVar7 = uVar2 - uVar8;
                        pass1_1030_7d7c(paVar3, lVar7, CONCAT22(local_16, (lVar7 >> 0x10)));
                        if (local_16 == 0x23) {
                            ppVar12 = process_struct_1010_20ba(
                                _g_astruct_372_1050_0ed0,
                                CONCAT22(unaff_SI, 0x2b),
                            );
                            pass1_1010_043a(ppVar12, (local_e + 4), 0x12);
                        }
                    } else {
                        bVar11 = local_12 < uVar5;
                        local_12 = local_12 - uVar5;
                        local_10 = (local_10 - local_36._2_2_) - bVar11;
                    }
                }
                local_16 = local_16 + 1;
            }
            return;
        }
    }
    return;
}

pub fn pass1_1028_ccd0(param_1: *mut astruct_44, param_2: *mut u16) {
    let ppcVar1: fn();
    let uVar2: u8;
    let puVar3: *mut u8;
    let extraout_var: u32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut extraout_DX: i32;
    let mut uVar6: i32;
    let mut extraout_DX_00: u16;
    let mut unaff_SS: u16;
    let mut uVar7: u16;
    let uVar8: u8;
    let uVar9: u8;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut in_stack_0000fe80: u16;
    let mut local_178: u16;
    let mut local_176: u16;
    let mut local_54: u16;
    let mut local_48: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: [u8; 12];
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3eb4(
        param_2,
        CONCAT22(unaff_SS, &local_8),
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    uVar6 = extraout_DX;
    uVar2 = pass1_1028_b58e(param_1);
    _local_14 = CONCAT31(extraout_var, uVar2) & 0xffff | uVar6 << 0x10;
    local_18 = (CONCAT31(extraout_var, uVar2) + 0x2e);
    local_1c = (local_18 + 4);
    pass1_1028_c1f8(
        param_1,
        (param_1 >> 0x10),
        0xe0,
        unaff_SS,
        &local_1e,
        unaff_SS,
    );
    local_a = local_4 - 1;
    iVar4 = local_4 + 1;
    local_c = local_6 - 1;
    iVar5 = local_6 + 1;
    if (local_a < 0) {
        local_a = 0;
    }
    if (local_1e <= iVar4) {
        iVar4 = local_1e - 1;
    }
    if (local_c < 0) {
        local_c = 0;
    }
    if (local_20 <= iVar5) {
        iVar5 = local_20 - 1;
    }
    _local_10 = CONCAT22(iVar4, iVar5);
    zero_list_1008_6c90(local_2c, unaff_SS);
    pass1_1008_6cec(
        CONCAT22(unaff_SS, local_2c),
        local_8,
        _local_10,
        local_8,
        CONCAT22(local_a, local_c),
    );
    _local_30 =
        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fe80, 0x2f));
    uVar6 = (_local_30 >> 0x10);
    local_34 = (_local_30 + 0x20);
    puVar3 = local_2c;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, puVar3), local_34);
    _local_38 = CONCAT22(uVar6, puVar3);
    if ((uVar6 | puVar3) != 0) {
        local_3c = 0;
        local_3e = 0;
        local_40 = local_c;
        while (local_40 <= local_10) {
            local_48 = local_a;
            while (uVar7 = local_3e, local_48 <= local_e) {
                iVar4 = local_3e >> 0xf;
                ppcVar1 = (*_local_38 + 4);
                local_3e = local_3e + 1;
                (**ppcVar1)(0x1030, _local_38, (_local_38 >> 0x10), uVar7, iVar4);
                local_3c = CONCAT22(extraout_DX_00, uVar7);
                local_3c._3_1_ = (extraout_DX_00 >> 8);
                if (local_3c._3_1_ == '\0') {
                    local_54 = uVar7;
                    if (uVar7 == 7) {
                        pass1_1008_3e76(param_2, local_8, local_40, local_48);
                        uVar11 = local_34;
                        uVar12 = (local_34 >> 0x10);
                        uVar8 = local_1c;
                        uVar9 = (local_1c >> 8);
                        uVar10 = (local_1c >> 0x10);
                        uVar7 = 6;
                    } else {
                        if (uVar7 == 8) {
                            pass1_1008_3e76(param_2, local_8, local_40, local_48);
                            uVar11 = local_34;
                            uVar12 = (local_34 >> 0x10);
                            uVar8 = local_1c;
                            uVar9 = (local_1c >> 8);
                            uVar10 = (local_1c >> 0x10);
                            uVar7 = 7;
                        } else {
                            if (uVar7 != 9) {}
                            // goto LAB_1028_ce2c;
                            pass1_1008_3e76(param_2, local_8, local_40, local_48);
                            uVar11 = local_34;
                            uVar12 = (local_34 >> 0x10);
                            uVar8 = local_1c;
                            uVar9 = (local_1c >> 8);
                            uVar10 = (local_1c >> 0x10);
                            uVar7 = 8;
                        }
                    }
                    pass1_1028_87f0(
                        CONCAT22(unaff_SS, &local_178),
                        0,
                        0,
                        uVar7,
                        param_2,
                        (param_2 >> 0x10),
                        CONCAT22(uVar10, CONCAT11(uVar9, uVar8)),
                        CONCAT22(uVar12, uVar11),
                    );
                    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_178));
                    local_178 = s_1_1050_389a;
                    local_176 = &PTR_LOOP_1050_1008;
                }
                // LAB_1028_ce2c:
                local_48 = local_48 + 1;
            }
            local_40 = local_40 + 1;
        }
    }
    return;
}

pub fn pass1_1028_ced2(param_1: *mut astruct_833) {
    let uVar1: u8;
    let extraout_AH: u8;
    let extraout_AH_00: u8;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar2: i32;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    bVar3 = (*(param_1 + 0x1a) & 2) == 0;
    if (bVar3) {
        uVar6 = 0;
        uVar7 = 0x23;
        uVar5 = 1;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_DX, CONCAT11(extraout_AH, uVar1)),
            uVar5,
            CONCAT22(uVar7, uVar6),
        );
        in_DX = extraout_DX;
    }
    bVar4 = (*(param_1 + 0x1a) & 1) == 0;
    if (bVar4) {
        uVar6 = 0;
        uVar7 = 0xe;
        uVar5 = 1;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_DX, CONCAT11(extraout_AH_00, uVar1)),
            uVar5,
            CONCAT22(uVar7, uVar6),
        );
    }
    if (bVar4 || bVar3) {
        pass1_1028_bdac(param_1, 6);
        return 0;
    }
    return 1;
}

pub fn pass1_1028_cf44(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_cfd2(param_1: *mut u32, param_2: u32) {
    unsafe { *param_1 = param_2 };
    (param_1 + 4) = 0;
    return;
}

pub fn pass1_1028_cff2(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 4);
    uVar2 = (param_1 + 6);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    return;
}

pub fn pass1_1028_d01a(param_1: *mut u32) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: i32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    unsafe { puVar1 = *param_1 };
    _local_e = puVar1;
    while (true) {
        uVar4 = _local_e;
        pass1_1028_d728(puVar1);
        _local_e = CONCAT22(in_DX, uVar4);
        if ((in_DX | uVar4) == 0) {
            break;
        }
        uVar3 = *_local_e;
        ppcVar2 = uVar3 + 2;
        ppcVar2();
        in_DX = extraout_DX;
        if (_local_e != 0x0) {
            ppcVar2 = uVar3;
            ppcVar2();
            in_DX = extraout_DX_00;
        }
    }
    return;
}

pub fn pass1_1028_d078(param_1: u32, param_2: u32) {
    let ppcVar1: fn();
    let puVar2: *mut u32;
    let extraout_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
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

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    puVar2 = (iVar3 + 4);
    struct_a = (iVar3 + 6);
    _local_e = CONCAT22(struct_a, puVar2);
    local_12 = puVar2;
    local_10 = struct_a;
    if ((struct_a | puVar2) != 0) {
        unsafe {
            ppcVar1 = *puVar2;
            (**ppcVar1)();
        }
        struct_a = extraout_DX;
    }
    process_struct_1000_179c(0x1c, struct_a);
    local_12 = puVar2;
    local_10 = struct_a;
    if ((struct_a | puVar2) == 0) {
        puVar2 = 0x0;
        local_4 = 0;
    } else {
        process_struct_1008_8e9e(CONCAT22(struct_a, puVar2), 6, 0x24);
        local_4 = extraout_DX_00;
    }
    (iVar3 + 4) = puVar2;
    (iVar3 + 6) = local_4;
    local_a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    local_6 = local_a;
    if ((local_4 | local_a) == 0) {
        pass1_1018_dcf6(CONCAT22(unaff_SS, &local_16));
        uVar5 = pass1_1018_dd1e(&local_16, unaff_SS, 0, 0xa0000);
        pass1_1008_8faa((iVar3 + 4), uVar5);
        return;
    }
    pass1_1038_565e(CONCAT22(local_4, local_a));
    if ((extraout_DX_01 | local_a) != 0) {
        local_8 = extraout_DX_01;
        pass1_1028_d172(param_1, CONCAT22(extraout_DX_01, local_a));
    }
    return;
}

pub fn pass1_1028_d172(param_1: u32, param_2: u32) {
    let mut unaff_SS: u16;
    let lVar1: u32;
    let mut uVar2: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 8];
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1018_dcf6(CONCAT22(unaff_SS, &local_6));
    pass1_1008_5784(CONCAT22(unaff_SS, local_e), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_SS, local_e));
        if (lVar1 == 0) {
            break;
        }
        uVar2 = pass1_1018_dd1e(&local_6, unaff_SS, 0, *(lVar1 + 4) << 0x10);
        pass1_1008_8faa((param_1 + 4), uVar2);
    }
    return;
}

pub fn pass1_1028_d1dc(struct_a: *mut astruct_500, string_a: *mut char) {
    let local_struct_1: *mut astruct_500;
    let pcVar1: *mut char;
    let in_stack_0000fffa: *mut char;

    pcVar1 = (struct_a >> 0x10);
    local_struct_1 = struct_a;
    struct_a.a = s_1_1050_389a;
    local_struct_1.b = &PTR_LOOP_1050_1008;
    local_struct_1.c = string_a;
    local_struct_1.d = 0;
    struct_a.a = 0x6ad2;
    local_struct_1.b = &PTR_LOOP_1050_1028;
    string_fn_1000_3f9c(
        &local_struct_1.field_0x8,
        pcVar1,
        s_ctor_1050_5160,
        &g_alloc_addr_1050_1050,
        in_stack_0000fffa,
    );
    return struct_a;
}

pub fn pass1_1028_d22e(param_1: *mut u32, param_2: u32) {
    let mut uVar1: i32;
    let in_DX: *mut astruct_199;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    unsafe { *param_1 = 0 };
    (param_1 + 4) = param_2;
    process_struct_1000_179c(0xc, in_DX);
    uVar1 = param_2;
    uVar2 = in_DX | uVar1;
    if (uVar2 == 0) {
        unsafe { *param_1 = 0 };
    } else {
        pass1_1028_d59c((param_2 & 0xffff | ZEXT24(in_DX) << 0x10));
        param_1 = uVar1;
        (param_1 + 2) = uVar2;
    }
    return;
}

pub fn pass1_1028_d282(param_1: *mut u32) {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    unsafe { uVar1 = *param_1 };
    uVar2 = (param_1 + 2);
    _local_6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1028_d658(CONCAT22(uVar2, uVar1));
        error_check_1000_17ce(_local_6);
    }
    return;
}

pub fn pass1_1028_d2b0(param_1: *mut u32) {
    let mut unaff_SS: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;

    pass1_1028_9c62(&local_10c, unaff_SS, 16000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, (s_0_023_1050_3a93 + 5));
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, 14000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, 13000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(
        &local_10c,
        unaff_SS,
        (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0xab),
    );
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, (s_fem133_wav_1050_2af7 + 1));
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, (s_fem36_wav_1050_270c + 4));
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, (s_noth_bmp_1050_2321 + 7));
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, (s_547_bmp_1050_1f3f + 1));
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, (s_42_flc_1050_1b54 + 4));
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, 6000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, 5000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, 4000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, 3000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    pass1_1028_9c62(&local_10c, unaff_SS, 1000);
    pass1_1028_d566(param_1, CONCAT22(unaff_SS, &local_10c));
    local_10c = s_1_1050_389a;
    local_10a = &PTR_LOOP_1050_1008;
    unsafe { pass1_1028_d6b2(*param_1) };
    return;
}

pub fn pass1_1028_d52c(param_1: *mut u32, param_2: u32, param_3: *mut u32) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut uvar3: u16;

    unsafe { ppcVar1 = (*param_3 + 8) };
    iVar2 = (**ppcVar1)();
    if (iVar2 != 0) {
        unsafe { uVar3 = pass1_1028_d776(*param_1, param_2, param_3) };
        if (uVar3 != 0) {
            return 1;
        }
    }
    return 0;
}

pub fn pass1_1028_d566(param_1: *mut u32, param_2: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let fn_ptr_1: fn();

    fn_ptr_1 = (param_2 + 8);
    iVar1 = (**fn_ptr_1)();
    if (iVar1 != 0) {
        unsafe { uVar2 = pass1_1028_d742(*param_1, param_2) };
        if (uVar2 != 0) {
            return 1;
        }
    }
    return 0;
}

pub fn pass1_1028_d59c(param_1: *mut u32) {
    let puVar1: *mut u16;
    let mut uVar2: i32;
    let puVar3: *mut u16;
    let in_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    unsafe { *param_1 = 0 };
    (iVar4 + 4) = 0;
    (iVar4 + 8) = 0;
    puVar3 = *_g_bool_1050_5748;
    unsafe { *param_1 = puVar3 };
    process_struct_1000_179c(0xc, in_DX);
    puVar1 = (puVar3 & 0xffff | ZEXT24(in_DX) << 0x10);
    struct_a = (in_DX | puVar3);
    if (struct_a == 0x0) {
        (iVar4 + 4) = 0;
    } else {
        process_struct_1008_574a((puVar3 & 0xffff | ZEXT24(in_DX) << 0x10));
        unsafe { *puVar1 = 0xd804 };
        (puVar3 + 2) = &PTR_LOOP_1050_1028;
        (iVar4 + 4) = puVar1;
        puVar3 = puVar1;
    }
    uVar2 = puVar3;
    process_struct_1000_179c(0xc, struct_a);
    _local_e = CONCAT22(struct_a, uVar2);
    if ((struct_a | uVar2) == 0) {
        (iVar4 + 8) = 0;
    } else {
        process_struct_1008_574a(CONCAT22(struct_a, uVar2));
        *_local_e = 0xd804;
        (uVar2 + 2) = &PTR_LOOP_1050_1028;
        (iVar4 + 8) = _local_e;
    }
    return;
}

pub fn pass1_1028_d658(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 4);
    uVar2 = (iVar4 + 6);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    puVar1 = (iVar4 + 8);
    uVar2 = (iVar4 + 10);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    return;
}

pub fn pass1_1028_d69e(param_1: u32) {
    let mut u_var1: u32;

    uVar1 = (param_1 + 4);
    return (uVar1 + 8);
}

pub fn pass1_1028_d6b2(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let ppcVar3: fn();
    let mut uVar4: i32;
    let puVar5: *mut u32;
    let puVar6: *mut u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar7: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    puVar2 = *_PTR_LOOP_1050_65e2;
    puVar6 = puVar2;
    while (true) {
        uVar4 = puVar6;
        uVar7 = (param_1 >> 0x10);
        pass1_1020_c860(*(param_1 + 8));
        let pu_var1_val = unsafe { *puVar1 };
        if (((extraout_DX | uVar4) == 0)
            || (
                puVar1 = (uVar4 + 0xc),
                puVar2 <= pu_var1_val && pu_var1_val != puVar2,
            ))
        {
            break;
        }
        ppcVar3 = ((param_1 + 8) + 0x10);
        puVar5 = puVar2;
        (**ppcVar3)();
        puVar6 = (puVar5 & 0xffff | extraout_DX_00 << 0x10);
        pass1_1028_d742(param_1, puVar5 & 0xffff | extraout_DX_00 << 0x10);
        if (puVar6 != 0x0) {
            unsafe {
                ppcVar3 = *puVar6;
                (**ppcVar3)(0x1020, puVar5, extraout_DX_00, 1);
            }
        }
    }
    return;
}

pub fn pass1_1028_d728(param_1: u32) {
    let ppcVar1: fn();

    ppcVar1 = ((param_1 + 4) + 0x10);
    (**ppcVar1)();
    return;
}

pub fn pass1_1028_d742(param_1: u32, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (param_2 + 0xc);
    uVar2 = (**ppcVar1)();
    pass1_1020_c872((param_1 + 4), (uVar2 + 4), uVar2);
    return 1;
}

pub fn pass1_1028_d776(param_1: u32, param_2: u32, param_3: *mut u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;

    unsafe { ppcVar1 = (*param_3 + 0xc) };
    uVar2 = (**ppcVar1)();
    pass1_1020_c872((param_1 + 8), param_2, uVar2);
    return 1;
}

pub fn pass1_1028_b316(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b260(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_b34c() {
    let pcVar1: *mut char;
    let mut cVar2: u8;
    let mut in_EAX: u32;
    let mut in_DL: u8;
    let mut in_BX: i32;
    let local_BX_23: *mut astruct_829;
    let puVar3: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_ES: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut in_ZF: bool;
    let mut in_SF: u8;
    let mut in_OF: u8;
    let in_stack_0000d730: *mut astruct_1;

    puVar3 = &stack0xfffe;
    cVar2 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar3 = puVar3 + -1;
        unsafe { *puVar3 = *unaff_BP };
        cVar2 = cVar2 + -1;
        '\0' < cVar2
    } {}
    if (!in_ZF && in_OF == in_SF) {
        pcVar1 = (in_BX + unaff_SI);
        unsafe { *pcVar1 = *pcVar1 - in_DL };
        pass1_1030_1628(in_stack_0000d730);
        uVar4 = (in_stack_0000d730 >> 0x10);
        local_BX_23 = in_stack_0000d730;
        local_BX_23.field_0xc = 0;
        local_BX_23.field_0xe = 0;
        local_BX_23.field_0x10 = 0;
        local_BX_23.field_0x12 = 0;
        local_BX_23.field_0x18 = 0;
        local_BX_23.field_0x1a = 0;
        local_BX_23.field_0x1c = 0;
        in_stack_0000d730.field_0x0 = 0xcf6a;
        local_BX_23.field_0x2 = &PTR_LOOP_1050_1028;
        local_BX_23.field_0x16 = 0;
        local_BX_23.field_0x14 = 0;
        return;
    }
    0x872 = unaff_ES;
    (in_BX + 0xc) = CONCAT11((in_EAX >> 8) + in_EAX + in_CF, in_EAX);
    return;
}

pub fn pass1_1028_b354(param_1: *mut astruct_763) {
    let local_BX_15: *mut astruct_763;
    let mut uVar1: u16;

    pass1_1030_1628(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_15 = param_1;
    &local_BX_15.field_0xc = 0;
    &local_BX_15.field_0xe = 0;
    &local_BX_15.field_0x10 = 0;
    &local_BX_15.field_0x12 = 0;
    &local_BX_15.field_0x18 = 0;
    &local_BX_15.field_0x1a = 0;
    &local_BX_15.field_0x1c = 0;
    param_1.field_0x0 = 0xcf6a;
    local_BX_15.field_0x2 = &PTR_LOOP_1050_1028;
    &local_BX_15.field_0x16 = 0;
    &local_BX_15.field_0x14 = 0;
    return;
}

pub fn pass1_1028_b39e(param_1: *mut astruct_764, param_2: u16, param_3: u32) {
    let local_BX_25: *mut astruct_764;
    let mut uVar1: i32;

    pass1_1030_165e(param_1, 0x7000000, param_3);
    uVar1 = (param_1 >> 0x10);
    local_BX_25 = param_1;
    &local_BX_25.field_0xc = param_2;
    &local_BX_25.field_0xe = 0x42;
    &local_BX_25.field_0x10 = 0;
    &local_BX_25.field_0x12 = 0;
    &local_BX_25.field_0x18 = 0;
    &local_BX_25.field_0x1a = 0;
    &local_BX_25.field_0x1c = 0;
    param_1.field_0x0 = 0xcf6a;
    local_BX_25.field_0x2 = &PTR_LOOP_1050_1028;
    pass1_1028_bf76((param_1 & 0xffff | uVar1 << 0x10));
    &local_BX_25.field_0x14 = 0;
    if ((0x4e < &local_BX_25.field_0xc) && (&local_BX_25.field_0xc < 0x70)) {
        &local_BX_25.field_0xe = 0x6b;
    }
    return;
}

pub fn pass1_1028_b418(param_1: *mut astruct_44) {
    let mut iVar1: i32;
    let mut bVar2: u8;
    let uVar3: u8;
    let mut uVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.ptr_a_lo = 0xcf6a;
    (iVar5 + 2) = &PTR_LOOP_1050_1028;
    iVar1 = (iVar5 + 0x12);
    if (((iVar1 == 4) || (iVar1 == 5))
        || ((
            uVar4 = iVar1 - 6,
            uVar4 == 0
                && ((
                    iVar1 = (iVar5 + 0x18),
                    iVar1 == 4 || (uVar4 = iVar1 - 5, uVar4 == 0),
                )),
        )))
    {
        bVar2 = error_check_1000_17ce((iVar5 + 0x14));
        uVar4 = bVar2;
    }
    uVar3 = uVar4;
    pass1_1030_16b2(param_1);
    return uVar3;
}

pub fn pass1_1028_b46e(param_1: *mut astruct_781, param_2: *mut u8) {
    let mut uVar1: u16;
    let uVar2: u8;
    let extraout_var: u32;
    let mut uVar4: i32;
    let paVar5: *mut astruct_1095;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut uVar3: u32;

    paVar5 = pass1_1028_b4f2(param_1);
    uVar4 = (paVar5 >> 0x10);
    uVar6 = 0;
    uVar7 = 0;
    uVar2 = pass1_1028_b58e(param_1);
    uVar3 = CONCAT31(extraout_var, uVar2);
    pass1_1030_6d80(uVar3 & 0xffff | uVar4 << 0x10, CONCAT22(uVar7, uVar6));
    uVar1 = (uVar3 + 0x32);
    if (uVar1 != 0) {
        pass1_1030_6c4c(uVar3 & 0xffff | uVar4 << 0x10, 0);
        pass1_1038_387e(paVar5, 0, uVar1, uVar3 & 0xffff | uVar4 << 0x10);
    }
    pass1_1030_7296((uVar3 & 0xffff | uVar4 << 0x10));
    (param_1 + 0x1c) = (param_2 + 0x200);
    return;
}

pub fn pass1_1028_b4f2(param_1: *mut astruct_44) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1);
    return CONCAT22(
        (CONCAT11(extraout_AH, uVar1) + 0x30),
        (CONCAT11(extraout_AH, uVar1) + 0x2e),
    );
}

pub fn pass1_1028_b514(param_1: u32) {
    let mut iVar1: i32;
    let uVar2: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar1 = (iVar3 + 0x12);
    if (((iVar1 == 4) || (iVar1 == 5))
        || (iVar1 == 6 && ((iVar1 = (iVar3 + 0x18), iVar1 == 4 || (iVar1 == 5)))))
    {
        error_check_1000_17ce((iVar3 + 0x14));
    }
    (iVar3 + 0x14) = 0;
    (iVar3 + 0x12) = 7;
    uVar2 = pass1_1028_b58e((param_1 & 0xffff | uVar4 << 0x10));
    _local_6 = CONCAT22(in_DX, CONCAT11(extraout_AH, uVar2));
    pass1_1030_7296(CONCAT22(in_DX, CONCAT11(extraout_AH, uVar2)));
    pass1_1030_72d0(_local_6);
    pass1_1030_730a(_local_6);
    return;
}

pub fn pass1_1028_b58e(param_1: *mut astruct_44) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;

    uVar1 = (param_1 + 8);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    return paVar2;
}

pub fn pass1_1028_b5a8(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 5) {
        return 0;
    }
    uVar1 = (param_1 + 0x14);
    return (uVar1 + 0x94);
}

pub fn pass1_1028_b5ca(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 5) {
        return 0;
    }
    uVar1 = (param_1 + 0x14);
    return (uVar1 + 0x9c);
}

pub fn pass1_1028_afce(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_825;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_43: *mut astruct_826;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x116, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_43 = param_1;
        local_AX__1.field_0x4 = local_BX_43.field_0x4;
        puVar4 = &local_BX_43.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_43.field_0x108;
        local_AX__1.field_0x10c = local_BX_43.field_0x10c;
        local_AX__1.field_0x110 = local_BX_43.field_0x110;
        local_AX__1.field_0x114 = local_BX_43.field_0x114;
        *_local_a = 0xb0ce;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_b0a2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_b0de(param_1: *mut astruct_500, param_2: u32, param_3: u32) -> *mut astruct_500 {
    pass1_1028_6af2(param_1, param_2, param_3);
    param_1.a = 0xb1f4;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_b108(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_827;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_43: *mut astruct_828;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x110, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_43 = param_1;
        local_AX__1.field_0x4 = local_BX_43.field_0x4;
        puVar4 = &local_BX_43.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_43.field_0x108;
        local_AX__1.field_0x10c = local_BX_43.field_0x10c;
        *_local_a = 0x6e50;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xb1f4;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_b1c8(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_b1f4() -> *mut u16 {
    let pcVar1: *mut char;
    let puVar2: *mut u8;
    let pbVar3: *mut byte;
    let mut cVar4: u8;
    let mut in_DX: u16;
    let mut in_BX: i32;
    let puVar5: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_DI: i32;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let in_stack_0000d731: *mut astruct_1;

    puVar5 = &stack0xfffe;
    cVar4 = '\x0f';
    unsafe {
        while {
            unaff_BP = unaff_BP + -1;
            puVar5 = puVar5 + -1;
            *puVar5 = *unaff_BP;
            cVar4 = cVar4 + -1;
            '\0' < cVar4
        } {}
        pcVar1 = (in_BX + unaff_SI);
        *pcVar1 = *pcVar1 - in_DX;
        puVar2 = (in_BX + unaff_SI);
        *puVar2 = *puVar2;
        pbVar3 = (&PTR_LOOP_1050_1028 + in_BX + unaff_DI);
        *pbVar3 = *pbVar3 | (in_DX >> 8);
    }
    pass1_1030_1628(in_stack_0000d731);
    uVar6 = (in_stack_0000d731 >> 0x10);
    (in_stack_0000d731 + 0xc) = 0;
    in_stack_0000d731.field_0x0 = 0xb33c;
    (in_stack_0000d731 + 2) = &PTR_LOOP_1050_1028;
    return in_stack_0000d731;
}

pub fn pass1_1028_b204(param_1: *mut u16) {
    let mut uVar1: u16;

    pass1_1030_1628(param_1);
    uVar1 = (param_1 >> 0x10);
    unsafe {
        (param_1 + 0xc) = 0;
        *param_1 = 0xb33c;
    }
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_b22c(param_1: *mut u16, param_2: u16, param_3: u32) {
    let mut uVar1: u16;

    pass1_1030_165e(param_1, 0x6000000, param_3);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0xc) = param_2;
    unsafe { *param_1 = 0xb33c };
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_b260(param_1: u32) {
    let in_AL: u8;

    param_1 = 0xb33c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    pass1_1030_16b2(param_1);
    return in_AL;
}

pub fn pass1_1028_aec0(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let paVar3: *mut astruct_493;
    let mut in_DX: u16;
    let mut uVar4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x108);
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    uVar2 = paVar3[0x10].field_0x16;
    pass1_1030_375a(uVar2, (uVar2 >> 0x10), 0, (param_1 + 0x114));
    return;
}

pub fn pass1_1028_ad9c(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_822;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = &local_AX__1.field_0x8;
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xae56;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_ae2a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_ab68() {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let BVar3: bool;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let puVar5: *mut u32;
    let mut local_24: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_14)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    // LAB_1028_ab7e:
    puVar5 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
    uVar4 = (puVar5 >> 0x10);
    if (puVar5 == 0x0) {
        return 1;
    }
    iVar1 = (puVar5 + 0xc);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x11);
    if (BVar3 == 0) {}
    // goto code_r0x1028abad;
    // goto LAB_1028_abc0;
    // code_r0x1028abad:
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x12);
    if (BVar3 != 0) {
        // LAB_1028_abc0:
        if ((puVar5 + 0x12) == 5) {
            unsafe { ppcVar2 = (*puVar5 + 0x30) };
            ppcVar2(&PTR_LOOP_1050_1008);
        }
    }
    // goto LAB_1028_ab7e;
}

pub fn pass1_1028_abec(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) != 0) {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        (in_AX + 4) = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0xaca6;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_ac7a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a9f4() {
    let ppcVar1: fn();
    let BVar2: bool;
    let mut unaff_SS: u16;
    let puVar3: *mut u32;
    let mut local_24: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    while (true) {
        puVar3 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        if (puVar3 == 0x0) {
            break;
        }
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (puVar3 + 0xc), 0xc);
        if (BVar2 != 0) {
            unsafe { ppcVar1 = (*puVar3 + 0x34) };
            (**ppcVar1)(&PTR_LOOP_1050_1008, puVar3);
        }
    }
    return 1;
}

pub fn pass1_1028_aa68(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_821;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = &local_AX__1.field_0x8;
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xab22;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_aaf6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a89c() {
    let mut unaff_SS: u16;
    let mut u_var1: u32;
    let mut local_22: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        uVar1 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        if (uVar1 == 0) {
            break;
        }
        if ((uVar1 + 0x200) != 0x8000002) {
            pass1_1038_3fca(uVar1);
        }
    }
    return 1;
}

pub fn pass1_1028_a8f4(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_820;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = &local_AX__1.field_0x8;
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xa9ae;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_a982(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a73c() {
    let mut unaff_SS: u16;
    let paVar1: *mut astruct_1120;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        paVar1 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        if (paVar1 == 0x0) {
            break;
        }
        pass1_1038_5464(paVar1);
        pass1_1038_56d6(paVar1, 0);
        pass1_1038_518c(paVar1);
    }
    return 1;
}

pub fn pass1_1028_a79c(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_819;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = &local_AX__1.field_0x8;
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xa856;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_a82a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a28a(param_1: u16, param_2: u16, param_1_00: *mut astruct_817) {
    let ppcVar1: fn();
    let mut uVar2: u16;
    let mut uvar3: u16;
    let puVar4: *mut u8;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut in_DX: i32;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let local_BX_33: *mut astruct_817;
    let mut uVar9: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    puVar4 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0xe);
    uVar2 = SUB42(puVar4, 0);
    pass1_1038_4d6e(param_1_00, puVar4 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(in_DX, uVar2);
    uVar9 = (param_1_00 >> 0x10);
    local_BX_33 = param_1_00;
    uVar6 = local_BX_33.field_0x1f6;
    ppcVar1 = (*_local_a + 0x10);
    uVar5 = uVar6;
    uVar7 = in_DX;
    (**ppcVar1)(&PTR_LOOP_1050_1038, uVar2, in_DX);
    uVar3 = uVar5;
    uVar8 = uVar7;
    pass1_1030_38b8(uVar6, (uVar6 >> 0x10));
    if ((uVar5 & 0xffff | uVar7 << 0x10) == 0) {
        uVar6 = 100;
        uVar8 = 0;
    } else {
        uVar6 = CONCAT22(uVar8, uVar3) / (uVar5 & 0xffff | uVar7 << 0x10);
        uVar8 = (uVar6 >> 0x10);
    }
    uVar6 = uVar6 & 0xffff | uVar8 << 0x10;
    if (_local_a != 0x0) {
        ppcVar1 = *_local_a;
        (**ppcVar1)(0x1030, uVar2, in_DX, 1);
    }
    if (uVar6 < 100) {
        if (uVar6 < 0x55) {
            if (uVar6 < 0x4b) {
                if (uVar6 < 0x32) {
                    if (uVar6 < 0x19) {
                        local_BX_33.field_0x20a = 1;
                        local_BX_33.field_0x20c = 0xffff;
                        return;
                    }
                    local_BX_33.field_0x20a = 0;
                    local_BX_33.field_0x20c = 0;
                    return;
                }
                local_BX_33.field_0x20a = 0xfffb;
            } else {
                local_BX_33.field_0x20a = 0xfff6;
            }
        } else {
            local_BX_33.field_0x20a = 0xfff1;
        }
    } else {
        local_BX_33.field_0x20a = 0xffec;
    }
    local_BX_33.field_0x20c = 1;
    return;
}

pub fn pass1_1028_a3ae(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut in_AX: u16;
    let mut uVar1: u16;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: i32;
    let in_EDX: u32;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar6 = param_1_00;
    uVar7 = (param_1_00 >> 0x10);
    pass1_1038_3fb0(param_1_00);
    local_4 = in_EDX;
    if (((iVar6 + 0x204) != 0)
        && (
            uVar1 = pass1_1030_25b2(CONCAT22(local_4, in_AX), 0x82),
            uVar1 != 0,
        ))
    {
        return;
    }
    uVar3 = (iVar6 + 0x1f6);
    local_a = uVar3;
    pass1_1030_38b8(uVar3, (uVar3 >> 0x10));
    uVar2 = uVar3;
    local_10 = in_EDX;
    _local_e = uVar3 & 0xffff | in_EDX << 0x10;
    pass1_1038_540a(param_1_00, 0x1e);
    uVar4 = local_10 | uVar2;
    local_12 = uVar2;
    if ((((uVar4 == 0) && ((iVar6 + 0x200) != 0x8000002))
        && (pass1_1030_38b8(local_a, (local_a >> 0x10)), -1 < uVar4))
        && (0 < uVar4 || (uVar2 != 0)))
    {
        ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2b));
        local_1e = (ppVar5 >> 0x10);
        local_20 = ppVar5;
        pass1_1010_043a(ppVar5, (iVar6 + 4), 0x11);
    }
    local_1a = _local_e;
    uVar2 = local_12 * 10;
    uVar4 = (local_10 * 5
        + CARRY2(local_12, local_12) * 2
        + CARRY2(local_12 * 2, local_12 * 2)
        + CARRY2(local_12 * 4, local_12))
        * 2
        + CARRY2(local_12 * 5, local_12 * 5);
    _local_16 = CONCAT22(uVar4, uVar2);
    if ((uVar4 <= local_c) && (uVar4 < local_c || (uVar2 < _local_e))) {
        pass1_1028_ae66(
            CONCAT22(unaff_SS, &local_146),
            _local_e,
            CONCAT22(uVar4, uVar2),
            (iVar6 + 4),
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_146));
        local_1a = _local_16;
        local_146 = s_1_1050_389a;
        local_144 = &PTR_LOOP_1050_1008;
    }
    local_1a = local_1a + 9;
    pass1_1038_52b8(param_1_00, local_1a / 10, 0x1e);
    return;
}

pub fn pass1_1028_a4ee(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: i32;
    let puVar7: *mut u8;
    let mut uVar8: u32;
    let mut uVar9: i32;
    let mut uVar10: i32;
    let mut uVar11: u16;
    let in_EDX: u32;
    let mut uVar12: u16;
    let uVar13: u8;
    let mut uVar14: u16;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_22: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar12 = (param_2 >> 0x10);
    uVar1 = (param_2 + 0x1f6);
    uVar8 = *_PTR_LOOP_1050_65e2;
    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x26);
    uVar9 = in_EDX;
    uVar3 = puVar7;
    uVar13 = 0x38;
    pass1_1038_4d6e(param_2, puVar7 & 0xffff | in_EDX << 0x10);
    _local_12 = CONCAT22(uVar9, uVar3);
    ppcVar2 = (*_local_12 + 0x10);
    uVar6 = uVar3;
    uVar10 = uVar9;
    ppcVar2(&PTR_LOOP_1050_1038, uVar3, uVar9);
    if ((uVar10 | uVar6) != 0) {
        uVar13 = 0x30;
        pass1_1030_3548(uVar1, CONCAT22(uVar10, uVar6));
    }
    if (_local_12 != 0x0) {
        ppcVar2 = *_local_12;
        ppcVar2(uVar13, uVar3, uVar9, 1);
    }
    uVar11 = (uVar8 % 0xc);
    uVar14 = (param_1 >> 0x10);
    uVar4 = uVar11;
    if (uVar8 % 0xc == 0) {
        pass1_1030_387c(uVar1);
        pass1_1028_a61e(param_1, uVar14, uVar1, param_2);
    }
    pass1_1038_3fb0(param_2);
    if (((param_2 + 0x204) != 0)
        && (
            uVar5 = pass1_1030_25b2(CONCAT13((uVar11 >> 8), CONCAT12(uVar11, uVar4)), 0x80),
            uVar5 != 0,
        ))
    {
        return;
    }
    uVar12 = (uVar1 >> 0x10);
    uVar6 = uVar1 + 0x180;
    uVar8 = uVar6;
    local_32 = 1;
    while {
        if ((local_32 * 2 + uVar6) != 0) {
            pass1_fn_1008_612e(1, 100);
            if (uVar8 <= (local_32 * 2 + uVar6)) {
                pass1_1028_a188(
                    param_1,
                    uVar14,
                    (local_32 * 2 + uVar1 + 0x174),
                    local_32,
                    param_2,
                );
            }
        }
        local_32 = local_32 + 1;
        local_32 < 6
    } {}
    return;
}

pub fn pass1_1028_a61e(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let mut in_AX: i32;
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut in_DX: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let local_28: *mut astruct_818;
    let mut uStack38: u16;
    let mut uStack36: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    local_28 = param_1_00;
    uStack38 = (param_1_00 >> 0x10);
    pass1_1030_38b8(param_1_00);
    if ((in_DX < 0x3fff) || (in_DX < 0x4000 && (in_AX != 0xffff))) {
        pass1_1030_38f2(param_1_00, 3);
        uVar1 = in_AX;
        iVar3 = in_DX;
        pass1_1030_38f2(param_1_00, 4);
        _local_e = CONCAT22(in_DX + iVar3 + CARRY2(in_AX, uVar1), in_AX + uVar1);
        local_10 = local_28.field_0x1a8;
        if (local_10 == 0) {
            local_10 = 5;
        }
        uVar2 = _local_e / local_10;
        local_c = (uVar2 >> 0x10);
        if (((local_c | uVar2) != 0)
            && (
                uVar4 = (param_2_00 >> 0x10),
                (param_2_00 + 0x200) != 0x8000002,
            ))
        {
            ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uStack36, 0x2b));
            pass1_1010_043a(ppVar5, (param_2_00 + 4), 0xc);
            pass1_1030_3534(param_1_00, uVar2);
        }
    }
    return;
}

pub fn pass1_1028_a6ca(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_9efc(param_1: u32) {
    let lVar1: u32;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let mut uVar4: i32;
    let local_AX_291: *mut astruct_814;
    let lVar5: u32;
    let mut uVar6: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar7: u16;
    let mut extraout_DX_01: i32;
    let mut unaff_SS: u16;
    let ppVar8: *mut pass1_struct_1;
    let mut uVar9: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
    if ((in_DX | local_6) != 0) {
        local_4 = in_DX;
        pass1_1028_dc52(
            CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_18)),
            (&PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            puVar3 = &local_18;
            pass1_1028_e4ec(CONCAT22(unaff_SS, puVar3));
            local_6 = puVar3;
            local_4 = extraout_DX;
            if ((extraout_DX | puVar3) == 0) {
                break;
            }
            lVar1 = (puVar3 + 0x100);
            uVar7 = puVar3[0x101];
            if (puVar3[0xff] != 0) {
                uVar9 = (param_1 >> 0x10);
                lVar5 = lVar1;
                if ((lVar1 != 2) || (uVar7 != 0x800)) {
                    pass1_1028_a3ae(param_1, uVar9, CONCAT22(extraout_DX, puVar3));
                }
                uVar4 = lVar5;
                pass1_1028_a28a(param_1, uVar9, CONCAT22(local_4, local_6));
                if ((uVar7 < 1) && (uVar7 < 0 || (uVar4 < 100))) {
                    pass1_1028_a4ee(param_1, CONCAT22(local_4, local_6));
                }
                if (lVar1 != 0x8000002) {
                    pass1_1038_42cc(CONCAT22(local_4, local_6));
                    if ((extraout_DX_00 | uVar4) != 0) {
                        ppVar8 = process_struct_1010_20ba(
                            _g_astruct_372_1050_0ed0,
                            CONCAT22(local_3a, 0x37),
                        );
                        post_win_msg_1008_a0e4(
                            ppVar8,
                            0,
                            uVar4,
                            (local_6 + 0x208),
                            (local_6 + 4),
                            2,
                        );
                    }
                }
            }
        }
        local_18 = s_1_1050_389a;
        local_16 = &PTR_LOOP_1050_1008;
        ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_3a, 8));
        uVar7 = (ppVar8 >> 0x10);
        uVar2 = SUB42(ppVar8, 0);
        local_AX_291 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
        local_6 = local_AX_291;
        local_4 = uVar7;
        pass1_1010_9f72(ppVar8, 0x3e);
        if (local_AX_291 != 0x0) {
            pass1_1010_96d0(ppVar8);
            if (local_AX_291 < 1) {
                if (local_AX_291 < 0) {
                    uVar6 = (local_6 + 0x1f6);
                    pass1_1030_38b8(uVar6, (uVar6 >> 0x10));
                    if ((extraout_DX_01 < 1) && (extraout_DX_01 < 0 || (uVar6 == 0))) {
                        ppVar8 = process_struct_1010_20ba(
                            _g_astruct_372_1050_0ed0,
                            CONCAT22(uVar2, 0x37),
                        );
                        post_win_msg_1008_a0e4(ppVar8, 0, 0, 1, (local_6 + 4), 6);
                    }
                }
            } else {
                ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar2, 0x37));
                post_win_msg_1008_a0e4(ppVar8, 0, local_AX_291, (local_6 + 0x208), 0x4000001, 2);
                ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar2, 0x2b));
                pass1_1010_043a(ppVar8, (local_6 + 4), 0x14);
            }
        }
    }
    return;
}

pub fn pass1_1028_a0fa(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_815;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = &local_AX__1.field_0x8;
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xa6f6;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9b48(param_1: *mut astruct_808) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_809;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_25: *mut astruct_808;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x118, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    local_BX_25 = param_1;
    uVar6 = (param_1 >> 0x10);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        local_AX__1.field_0x4 = local_BX_25.field_0x4;
        puVar4 = &local_BX_25.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_25.field_0x108;
        local_AX__1.field_0x10c = local_BX_25.field_0x10c;
        local_AX__1.field_0x110 = local_BX_25.field_0x110;
        local_AX__1.field_0x114 = local_BX_25.field_0x114;
        *_local_a = 0x9c52;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    local_BX_25.field_0x114 = 0;
    return;
}

pub fn pass1_1028_9c2c(param_1: *mut astruct_805, param_2: u8) -> *mut astruct_805 {
    pass1_1028_9992(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_9c62(param_1: *mut astruct_811, param_2: u16, param_3: u16) {
    pass1_1028_d1dc(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x108 = param_3;
    CONCAT22(param_2, param_1) = 0x9eb6;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_9c90(param_1: u32) {
    let mut uVar1: i32;
    let mut uVar2: u16;

    uVar1 = (param_1 + 0x108) - 1000;
    if ((uVar1 < s_K1_1050_3a99) && (uVar1 % 1000 == 0)) {
        // WARNING: Could not recover jumptable at 0x10289dc0. Too many branches
        // WARNING: Treating indirect jump as call
        uVar2 = (**((uVar1 / 1000) * 2 + -0x623a))();
        return uVar2;
    }
    return 1;
}

pub fn pass1_1028_9dee(param_1: *mut astruct_812) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_813;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_41: *mut astruct_812;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10a, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_41 = param_1;
        local_AX__1.field_0x4 = local_BX_41.field_0x4;
        puVar4 = &local_BX_41.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_41.field_0x108;
        *_local_a = 0x9eb6;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9e8a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_93d4(param_1: u32) {
    let ppcVar1: fn();
    let paVar2: *mut astruct_493;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut in_DX: i32;
    let local_BX_20: *mut astruct_801;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_114: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    PTR_LOOP_1050_50ca = 0x0;
    PTR_LOOP_1050_50cc = 0x0;
    uVar5 = (param_1 >> 0x10);
    local_BX_20 = param_1;
    uVar4 = SEXT24(local_BX_20.field_0x11a);
    pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
    _local_6 = uVar4 & 0xffff | in_DX << 0x10;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, in_DX);
    &local_BX_20.field_0x11e = paVar2;
    local_BX_20.field_0x120 = in_DX;
    iVar3 = &local_BX_20.field_0x114;
    ppcVar1 = (&local_BX_20.field_0x11e + 0x1c);
    (**ppcVar1)();
    if (iVar3 != 0) {
        pass1_1028_9624(param_1);
        ppcVar1 = (&local_BX_20.field_0x11e + 0x20);
        (**ppcVar1)();
        ppcVar1 = (&local_BX_20.field_0x11e + 0x18);
        (**ppcVar1)();
        pass1_1028_9600(param_1);
        return;
    }
    &local_BX_20.field_0x11e = 0;
    pass1_1030_e4fa(CONCAT22(unaff_SS, &local_112), _local_6);
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_112));
    if (PTR_LOOP_1050_50ca == 0x0) {
        PTR_LOOP_1050_50ca = 0x6ad;
    }
    return;
}

pub fn pass1_1028_94e4(param_1: *mut astruct_803) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_802;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_43: *mut astruct_803;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x124, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_43 = param_1;
        local_AX__1.field_0x4 = local_BX_43.field_0x4;
        puVar4 = &local_BX_43.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_43.field_0x108;
        local_AX__1.field_0x10c = local_BX_43.field_0x10c;
        local_AX__1.field_0x110 = local_BX_43.field_0x110;
        local_AX__1.field_0x114 = local_BX_43.field_0x114;
        local_AX__1.field_0x118 = local_BX_43.field_0x118;
        local_AX__1.field_0x11a = local_BX_43.field_0x11a;
        local_AX__1.field_0x11c = local_BX_43.field_0x11c;
        local_AX__1.field_0x11e = local_BX_43.field_0x11e;
        local_AX__1.field_0x122 = local_BX_43.field_0x122;
        *_local_a = 0x9934;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9600(param_1: u32) {
    let mut unaff_SS: u16;
    let mut local_6: [u8; 4];

    pass1_1020_a43e(CONCAT22(unaff_SS, local_6));
    pass1_1020_a80e(local_6, unaff_SS, (param_1 + 0x11a));
    return;
}

pub fn pass1_1028_9624(param_1: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let paVar3: *mut astruct_493;
    let puVar4: *mut u32;
    let mut uVar5: u16;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let BVar8: bool;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: u16;
    let mut iVar9: i32;
    let mut unaff_SI: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut uStack332: u16;
    let mut uStack330: u16;
    let mut uStack64: u16;
    let mut uStack62: u32;
    let mut iStack58: i32;
    let mut local_38: u32;
    let mut local_2e: u32;
    let mut uStack42: u32;
    let mut local_26: [u8; 4];
    let paStack34: *mut astruct_493;
    let mut uStack32: u16;
    let mut uStack30: u32;
    let mut uStack26: u32;
    let mut local_16: u32;
    let mut local_12: [u8; 2];
    let mut local_10: [u8; 2];
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar10 = (param_1 >> 0x10);
    iVar9 = param_1;
    uVar1 = (iVar9 + 0x10c);
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    (iVar9 + 0x110) = paVar3;
    (iVar9 + 0x112) = in_DX;
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
    uVar1 = (iVar9 + 0x108);
    puVar4 = (iVar9 + 0x114);
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        (_PTR_LOOP_1050_5740 >> 0x10),
        puVar4,
        uVar10,
        uVar1,
        (uVar1 >> 0x10),
        local_26,
        unaff_SS,
    );
    unsafe { local_38 = *puVar4 };
    local_38._3_1_ = (local_38 >> 0x18);
    local_c = (local_38._3_1_ != '\0');
    uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
    local_2e = local_38;
    local_a = local_38;
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | (iVar9 + 0x114)),
        CONCAT22(unaff_SS, local_12),
        CONCAT22(unaff_SS, local_10),
        CONCAT22(unaff_SS, local_e),
    );
    if (local_c == 0) {
        local_a._2_2_ = extraout_DX_00;
        uVar6 = iVar9 + 0x114;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2);
        local_16 = CONCAT22(local_a._2_2_, uVar6);
        uVar11 = 0x1030;
        pass1_1030_61fe(
            _PTR_LOOP_1050_5740,
            CONCAT22(local_a._2_2_, uVar6),
            param_1 & 0xffff0000 | (iVar9 + 0x114),
            (iVar9 + 0x108),
        );
        if (((iVar9 + 0x11a) == 10) || ((iVar9 + 0x11a) == 0x37)) {
            if ((iVar9 + 0x11a) == 0x37) {
                local_38 = (iVar9 + 0x11e);
                local_a._2_2_ = (iVar9 + 0x120);
                uStack42 = (iVar9 + 0x10c);
                (local_38 + 0x20) = uStack42;
            }
            iVar7 = iVar9 + 0x114;
            pass1_1028_e2ac(_PTR_LOOP_1050_65e2);
            (iVar9 + 0x10c) = iVar7;
            (iVar9 + 0x10e) = local_a._2_2_;
            uVar11 = 0x1018;
            pass1_1018_0196(
                _local_6,
                CONCAT22(local_a._2_2_, (iVar9 + 0x10c)),
                (iVar9 + 0x108),
            );
            local_a._2_2_ = extraout_DX_01;
            if ((iVar9 + 0x11a) == 10) {
                uVar11 = 0x1010;
                pass1_1010_ed22(_local_6, (iVar9 + 0x10c));
                local_a._2_2_ = extraout_DX_02;
            }
        }
        uVar1 = (iVar9 + 0x10c);
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        (iVar9 + 0x110) = paVar3;
        (iVar9 + 0x112) = local_a._2_2_;
        if ((local_a._2_2_ | (iVar9 + 0x110)) == 0) {}
        // goto LAB_1028_9807;
        uVar5 = local_16;
        uVar6 = (local_16 >> 0x10);
    } else {
        local_16 = local_a;
        if ((iVar9 + 0x11a) != 0x75) {}
        // goto LAB_1028_9807;
        uVar5 = local_a;
        uVar6 = local_a._2_2_;
        local_a._2_2_ = (iVar9 + 0x112);
    }
    ppcVar2 = ((iVar9 + 0x110) + 8);
    ppcVar2(uVar11, (iVar9 + 0x110), local_a._2_2_, 0, uVar5, uVar6, 0);
    local_a._2_2_ = extraout_DX_03;
    // LAB_1028_9807:
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_16, (local_16 >> 0x10));
    uStack26 = CONCAT22(local_a._2_2_, paVar3);
    pass1_1030_73ee(CONCAT22(local_a._2_2_, paVar3), (iVar9 + 0x10c));
    uStack32 = extraout_DX_04;
    BVar8 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (iVar9 + 0x11a), 0x31);
    if ((BVar8 == 0) && ((iVar9 + 0x122) == 0)) {
        uStack62 = (uStack26 + 0xc);
        iStack58 = (uStack26 + 0x10);
        local_38 = local_38 & 0xffff0000 | ZEXT24(&uStack62);
        if (iStack58 < 1) {
            uStack64 = 5;
        } else {
            uStack64 = 6;
        }
        (uStack26 + 0x14) = uStack64;
        uStack32 = uStack26._2_2_;
    }
    uStack30 = (uStack26 + 0x16);
    paStack34 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack30, (uStack30 >> 0x10));
    if (uStack30 != 0) {
        pass1_1030_e4fa(CONCAT22(unaff_SS, &uStack332), uStack30);
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &uStack332));
        uStack332 = SUB42(s_1_1050_389a, 0);
        uStack330 = SUB42(&PTR_LOOP_1050_1008, 0);
    }
    ppcVar2 = ((iVar9 + 0x11e) + 4);
    ppcVar2();
    uVar1 = (iVar9 + 0x11e);
    pass1_1030_7e5a(uStack26, (uVar1 + 4));
    return;
}

pub fn pass1_1028_9908(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_9944(param_1: *mut astruct_500, param_2: u32, param_3: u32, param_4: u32) {
    let local_BX_19: *mut astruct_500;
    let mut uVar1: u16;

    pass1_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_4;
    local_BX_19.field_0x10c = param_3;
    local_BX_19.field_0x110 = param_2;
    local_BX_19.field_0x114 = 0;
    param_1.a = 0x9c52;
    local_BX_19.b = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_9992(param_1: *mut astruct_805) {
    let local_BX_4: *mut astruct_805;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = 0x9c52;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1028;
    error_check_1000_17ce(local_BX_4.field_0x114);
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1028_99c4(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut in_DX: u16;
    let mut uvar3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x10c);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    pass1_1030_355c(paVar2[0x10].field_0x16, (param_1 + 0x114));
    return;
}

pub fn pass1_1028_8920(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let puVar4: *mut u32;
    let BVar5: bool;
    let paVar6: *mut astruct_493;
    let mut uVar7: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut uVar8: i32;
    let mut iVar9: i32;
    let mut iVar10: i32;
    let mut unaff_SI: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut unaff_SS: u16;
    let mut uVar13: u16;
    let mut local_156: u16;
    let mut local_154: u16;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_38: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: [u8; 4];
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 2];
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u32;

    uVar11 = (param_1 >> 0x10);
    iVar9 = param_1;
    uVar2 = (iVar9 + 0x108);
    puVar1 = (iVar9 + 0x114);
    puVar4 = puVar1;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        (_PTR_LOOP_1050_5740 >> 0x10),
        puVar1,
        uVar11,
        uVar2,
        (uVar2 >> 0x10),
        local_26,
        unaff_SS,
    );
    unsafe { local_6 = *puVar4 };
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | ZEXT24(puVar1)),
        CONCAT22(unaff_SS, local_c),
        CONCAT22(unaff_SS, local_a),
        CONCAT22(unaff_SS, local_8),
    );
    local_2e = local_6;
    local_38 = local_6;
    local_38._3_1_ = (local_6 >> 0x18);
    local_e = (local_38._3_1_ != '\0');
    if (local_e == 0) {
        uVar8 = iVar9 + 0x114;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2);
        _local_12 = CONCAT22(local_6._2_2_, uVar8);
        uVar12 = 0x1030;
        pass1_1030_61fe(
            _PTR_LOOP_1050_5740,
            CONCAT22(local_6._2_2_, uVar8),
            param_1 & 0xffff0000 | (iVar9 + 0x114),
            (iVar9 + 0x108),
        );
        local_38 = 0;
        if (((iVar9 + 0x11a) == 10) || ((iVar9 + 0x11a) == 0x37)) {
            if ((iVar9 + 0x11a) == 0x37) {
                local_38 = (iVar9 + 0x10c);
            }
            iVar10 = iVar9 + 0x114;
            pass1_1028_e2ac(_PTR_LOOP_1050_65e2);
            (iVar9 + 0x10c) = iVar10;
            (iVar9 + 0x10e) = local_6._2_2_;
            local_2e = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
            uVar12 = 0x1018;
            pass1_1018_0196(local_2e, (iVar9 + 0x10c), (iVar9 + 0x108));
            local_6._2_2_ = extraout_DX_00;
            if ((iVar9 + 0x110) != 0) {
                uVar2 = (iVar9 + 0x10c);
                paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
                _local_2a = CONCAT22(local_6._2_2_, paVar6);
                local_44 = (iVar9 + 0x110);
                &paVar6[0x11].field_0x2 = local_44;
            }
        }
        uVar2 = (iVar9 + 0x10c);
        paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
        _local_34 = CONCAT22(local_6._2_2_, paVar6);
        local_14 = local_6._2_2_ | paVar6;
        if (local_14 != 0) {
            ppcVar3 = (*_local_34 + 8);
            (**ppcVar3)(
                uVar12,
                paVar6,
                local_6._2_2_,
                0,
                _local_12,
                (_local_12 >> 0x10),
                0,
            );
            local_14 = extraout_DX_01;
        }
    } else {
        _local_12 = local_6;
        local_14 = local_6._2_2_;
    }
    local_16 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, _local_12, (_local_12 >> 0x10));
    pass1_1030_73ee(CONCAT22(local_14, local_16), (iVar9 + 0x10c));
    BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (iVar9 + 0x11a), 0x31);
    if ((BVar5 == 0) && ((iVar9 + 0x11c) == 0)) {
        local_4a = (local_16 + 0xc);
        local_46 = (local_16 + 0x10);
        local_44 = local_44 & 0xffff0000 | ZEXT24(&local_4a);
        if (local_46 < 1) {
            local_30 = 5;
        } else {
            local_30 = 6;
        }
        (local_16 + 0x14) = local_30;
    }
    local_1a = (local_16 + 0x16);
    local_1c = (local_16 + 0x18);
    if ((local_1c | local_1a) != 0) {
        pass1_1030_e4fa(
            CONCAT22(unaff_SS, &local_156),
            local_1a & 0xffff | local_1c << 0x10,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_156));
        _local_156 = 0x1008389a;
        local_1c = extraout_DX_02;
    }
    uVar7 = SEXT24((iVar9 + 0x11a));
    pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
    local_1e = uVar7;
    uVar8 = local_1c | local_1e;
    if (uVar8 == 0) {
        return;
    }
    pass1_1030_7e5a(
        CONCAT22(local_14, local_16),
        uVar7 & 0xffff | local_1c << 0x10,
    );
    paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_1e, local_1c);
    _local_22 = CONCAT22(uVar8, paVar6);
    uVar12 = _local_12;
    uVar13 = (_local_12 >> 0x10);
    iVar10 = *_local_22;
    ppcVar3 = (iVar10 + 4);
    (**ppcVar3)();
    ppcVar3 = (iVar10 + 0x20);
    (**ppcVar3)(0x1030, _local_22, paVar6, uVar8, uVar12, uVar13);
    ppcVar3 = (iVar10 + 0x18);
    (**ppcVar3)(0x1030, _local_22, (_local_22 >> 0x10), 1);
    if ((iVar9 + 0x11a) == 0x37) {
        (_local_22 + 0x20) = (iVar9 + 0x10c);
    }
    (iVar9 + 0x120) = _local_22;
    return;
}

pub fn pass1_1028_8c46(param_1: *mut astruct_794) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_793;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_43: *mut astruct_794;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x124, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_43 = param_1;
        local_AX__1.field_0x4 = local_BX_43.field_0x4;
        puVar4 = &local_BX_43.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_43.field_0x108;
        local_AX__1.field_0x10c = local_BX_43.field_0x10c;
        local_AX__1.field_0x110 = local_BX_43.field_0x110;
        local_AX__1.field_0x114 = local_BX_43.field_0x114;
        local_AX__1.field_0x118 = local_BX_43.field_0x118;
        local_AX__1.field_0x11a = local_BX_43.field_0x11a;
        local_AX__1.field_0x11c = local_BX_43.field_0x11c;
        local_AX__1.field_0x11e = local_BX_43.field_0x11e;
        local_AX__1.field_0x120 = local_BX_43.field_0x120;
        *_local_a = 0x8d8e;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_8d62(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_8d9e(param_1: *mut astruct_500, param_2: u32, param_3: u32, param_4: u32) {
    let local_BX_19: *mut astruct_795;
    let mut uVar1: u16;

    pass1_1028_d1dc(param_1, 0x3e8);
    uVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_4;
    local_BX_19.field_0x10c = param_3;
    local_BX_19.field_0x110 = param_2;
    local_BX_19.field_0x114 = 0;
    param_1.a = 0x8fb0;
    local_BX_19.field_0x2 = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_8dec(param_1: *mut astruct_796) {
    let local_BX_4: *mut astruct_796;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = 0x8fb0;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1028;
    error_check_1000_17ce(local_BX_4.field_0x114);
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1028_8e1e(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut in_DX: u16;
    let mut uvar3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x10c);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    pass1_1030_355c(paVar2[0x10].field_0x16, (param_1 + 0x114));
    return;
}

pub fn pass1_1028_8e5c(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let struct_a: *mut astruct_393;
    let mut in_DX: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = (iVar3 + 0x108);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    struct_a = paVar2[0x10].field_0x16;
    pass1_1030_35a4(struct_a, (iVar3 + 0x110));
    (iVar3 + 0x114) = struct_a;
    (iVar3 + 0x116) = in_DX;
    return;
}

pub fn pass1_1028_8ea6(param_1: *mut astruct_798) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_797;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_25: *mut astruct_798;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x118, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    local_BX_25 = param_1;
    uVar6 = (param_1 >> 0x10);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        local_AX__1.field_0x4 = local_BX_25.field_0x4;
        puVar4 = &local_BX_25.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_25.field_0x108;
        local_AX__1.field_0x10c = local_BX_25.field_0x10c;
        local_AX__1.field_0x110 = local_BX_25.field_0x110;
        local_AX__1.field_0x114 = local_BX_25.field_0x114;
        *_local_a = 0x8fb0;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    local_BX_25.field_0x114 = 0;
    return;
}

pub fn pass1_1028_8f8a(param_1: u32, param_2: u8) {
    pass1_1028_8dec(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_8fc0(param_1: *mut astruct_500, param_2: u32, param_3: u32) -> *mut astruct_500 {
    pass1_1028_6af2(param_1, param_2, param_3);
    param_1.a = 0x90d6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_8fea(param_1: *mut astruct_800) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_799;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_43: *mut astruct_800;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x110, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_43 = param_1;
        local_AX__1.field_0x4 = local_BX_43.field_0x4;
        puVar4 = &local_BX_43.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_43.field_0x108;
        local_AX__1.field_0x10c = local_BX_43.field_0x10c;
        *_local_a = 0x6e50;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0x90d6;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_90aa(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_90e6(param_1: *mut astruct_500, param_2: u16) -> *mut astruct_500 {
    let mut uVar1: u16;

    pass1_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x108) = param_2;
    param_1.a = 0x932c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_9114(param_1: u32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let paVar3: *mut astruct_493;
    let paVar4: *mut astruct_1106;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar5: u16;
    let mut uVar6: i32;
    let ppVar7: *mut pass1_struct_1;
    let ppVar8: *mut pass1_struct_1;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut local_16: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_16, 0x37));
    uVar6 = param_1;
    iVar2 = (uVar6 + 0x108);
    if (iVar2 - 1 < 8) {
        local_a._0_2_ = *_PTR_LOOP_1050_65e2;
        iVar1 = (*_PTR_LOOP_1050_65e2 >> 0x10);
        match (iVar2) {
            1 => {
                uVar12 = 0x16;
            }
            2 => {
                uVar12 = 0x17;
            }
            3 => {
                uVar12 = 0x18;
            }
            4 => {
                uVar12 = 0x1b;
            }
            5 => {
                uVar12 = 0x1f;
            }
            6 => {
                uVar12 = 0x24;
            }
            7 => {
                pass1_fn_1008_612e(0, 0x14);
                uVar12 = local_a + uVar6 + 0x6e;
                uVar9 = iVar1 + (uVar6 >> 0xf) + (0xff91 < uVar6) + CARRY2(local_a, uVar6 + 0x6e);
                uVar11 = 7;
                ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar12, 0x2f));
                iVar2 = ppVar8;
                pass1_1010_ebf8(ppVar8, uVar12, uVar9, uVar11);
                uVar5 = extraout_DX;
                pass1_fn_1008_612e(1, 100);
                if (0x32 < iVar2) {
                    return;
                }
                paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
                pass1_1038_4900(CONCAT22(uVar5, paVar3));
                uVar12 = 0x2c;
            }
            8 => {
                pass1_fn_1008_612e(0, 0x14);
                uVar12 = local_a + uVar6 + 100;
                uVar9 = iVar1 + (uVar6 >> 0xf) + (0xff9b < uVar6) + CARRY2(local_a, uVar6 + 100);
                uVar11 = 8;
                ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar12, 0x2f));
                pass1_1010_ebf8(ppVar8, uVar12, uVar9, uVar11);
                if (0x19 < uVar6) {
                    return;
                }
                uVar10 = 1;
                uVar12 = 2;
                uVar5 = extraout_DX_00;
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
                pass1_1038_43cc(paVar4, CONCAT22(uVar10, uVar5), uVar12);
                uVar12 = 0x2d;
            }
        }
        post_win_msg_1008_a0e4(ppVar7, 0, 0, 1, 0, uVar12);
    }
    return;
}

pub fn pass1_1028_9264(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let mut iVar5: i32;
    let puVar6: *mut u32;
    let mut uVar7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10a, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) != 0) {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar7 = (param_1 >> 0x10);
        iVar5 = param_1;
        (in_AX + 4) = (iVar5 + 4);
        puVar3 = (iVar5 + 8);
        puVar6 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar6;
            puVar6 = puVar6 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        (in_AX + 0x108) = (iVar5 + 0x108);
        *_local_a = 0x932c;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9300(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_83b4() {
    let mut unaff_SS: u16;
    let lVar1: u32;
    let mut local_22: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        lVar1 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        if (lVar1 == 0) {
            break;
        }
        (lVar1 + 0x206) = 1;
    }
    return 1;
}

pub fn pass1_1028_8400(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) != 0) {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        (in_AX + 4) = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x84ba;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_848e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_84ca(
    param_1: *mut astruct_500,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> i32 {
    let pcVar1: *mut char;
    let mut iVar2: i32;
    let pcVar3: *mut char;

    pass1_1028_d1dc(param_1, 0x3e7);
    pcVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + 0x108) = param_5;
    (iVar2 + 0x10a) = param_4;
    (iVar2 + 0x10c) = param_3;
    (iVar2 + 0x10e) = param_2;
    param_1.a = 0x8688;
    (iVar2 + 2) = &PTR_LOOP_1050_1028;
    if ((iVar2 + 0x108) == 1) {
        pcVar1 = s_max_1050_501c;
    } else {
        pcVar1 = s_min_1050_5020;
    }
    string_fn_1000_3f9c(
        (iVar2 + 8),
        pcVar3,
        s_SCForceMorale__s_for_colony__08l_1050_5024,
        &g_alloc_addr_1050_1050,
        pcVar1,
    );
    return;
}

pub fn pass1_1028_853e(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut uvar3: u16;
    let mut in_DX: u16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_6: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x108) == 0) {
        return 0;
    }
    uVar1 = (iVar4 + 0x10e);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    if ((iVar4 + 0x108) == 1) {
        uVar3 = 1000;
    } else {
        uVar3 = 0;
    }
    pass1_1038_4d0e(CONCAT22(in_DX, paVar2), uVar3);
    return 1;
}

pub fn pass1_1028_858c(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let mut iVar5: i32;
    let puVar6: *mut u32;
    let mut uVar7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x112, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) != 0) {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar7 = (param_1 >> 0x10);
        iVar5 = param_1;
        (in_AX + 4) = (iVar5 + 4);
        puVar3 = (iVar5 + 8);
        puVar6 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar6;
            puVar6 = puVar6 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        (in_AX + 0x108) = (iVar5 + 0x108);
        (in_AX + 0x10a) = (iVar5 + 0x10a);
        (in_AX + 0x10c) = (iVar5 + 0x10c);
        (in_AX + 0x10e) = (iVar5 + 0x10e);
        *_local_a = 0x8688;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_865c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_8698(param_1: *mut astruct_500, param_2: u32, param_3: u32) -> *mut astruct_500 {
    pass1_1028_6af2(param_1, param_2, param_3);
    param_1.a = 0x87e0;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_86c2(param_1: u32) -> u8 {
    let mut uVar1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;

    uVar7 = 0;
    uVar8 = 0x1d;
    uVar5 = 1;
    uVar6 = 0;
    uVar4 = 0;
    uVar1 = 0;
    uVar3 = 0;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x37);
    post_win_msg_1008_a0e4(
        ppVar2,
        CONCAT22(uVar4, uVar3),
        uVar1,
        uVar5,
        CONCAT22(uVar7, uVar6),
        uVar8,
    );
    uVar1 = pass1_1028_6b2c(param_1);
    return uVar1;
}

pub fn pass1_1028_86f4(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let mut iVar5: i32;
    let puVar6: *mut u32;
    let mut uVar7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x110, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) == 0) {
        local_6._0_2_ = 0;
    } else {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar7 = (param_1 >> 0x10);
        iVar5 = param_1;
        (in_AX + 4) = (iVar5 + 4);
        puVar3 = (iVar5 + 8);
        puVar6 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar6;
            puVar6 = puVar6 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        (in_AX + 0x108) = (iVar5 + 0x108);
        (in_AX + 0x10c) = (iVar5 + 0x10c);
        *_local_a = 0x6e50;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x87e0;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        local_6._0_2_ = in_AX;
    }
    return local_6;
}

pub fn pass1_1028_87b4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_81e0() {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let puVar4: *mut u32;
    let mut local_24: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    // switchD_1028_8225_caseD_0:
    while {
        loop {
            puVar4 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
            uVar3 = (puVar4 >> 0x10);
            if (puVar4 == 0x0) {
                return 1;
            }
            iVar1 = (puVar4 + 0xc);
            if (iVar1 < 0x35) {}
            // goto code_r0x10288222;
            if (0x61 < iVar1) {
                break;
            }
            if ((iVar1 < 0x5d) && (iVar1 != 0x37 && (iVar1 != 0x47))) {}
            // goto switchD_1028_8225_caseD_1;
        }
        (iVar1 == 0x6a)
            || (8 < iVar1 + -0x6a
                && (iVar1 == 0x75
                    || iVar1 + -0x74 < 1
                    || (0 < iVar1 + -0x76 && (iVar1 + -0x78 < 2))))
    } {}
    // goto switchD_1028_8225_caseD_1;
    // code_r0x10288222:
    unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
    match (iVar1) {
        1 | 2 | 3 | 4 | 6 | 7 | 8 | 10 | 0xb | 0xc | 0xd | 0xe | 0xf | 0x11 => {
            // switchD_1028_8225_caseD_1:
            if ((puVar4 + 0x12) == 5) {
                unsafe {
                    ppcVar2 = (*puVar4 + 0x30);
                    ppcVar2(unaff_CS);
                }
            }
        } // goto switchD_1028_8225_caseD_0;
    }
}

pub fn pass1_1028_82b4(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) != 0) {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        (in_AX + 4) = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x836e;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_8342(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn post_msg_1028_76da() {
    let lVar1: u32;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000ffe4: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar3 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x2c),
    );
    uVar2 = (ppVar3 >> 0x10);
    lVar1 = (ppVar3 + 0xc);
    local_a._2_2_ = (lVar1 >> 0x10);
    local_a._0_2_ = lVar1;
    if (((local_a._2_2_ | local_a) != 0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
        PostMessage16(0, 0x106, 0x111, g_h_window);
        (ppVar3 + 0xc) = 0;
    }
    return;
}

pub fn pass1_1028_7742(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: *mut astruct_44) {
    let ppcVar1: fn();
    let uVar2: u8;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let puVar5: *mut u8;
    let puVar6: *mut u8;
    let mut uVar7: u32;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut uVar8: i32;
    let mut unaff_SS: u16;
    let paVar9: *mut astruct_1115;
    let uVar10: u8;
    let uVar11: u8;
    let mut uVar12: i32;
    let mut uVar13: u16;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: [u8; 2];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x18);
    uVar3 = puVar6;
    local_6 = uVar3;
    paVar9 = pass1_1028_b4f2(param_2_00);
    local_8 = (paVar9 >> 0x10);
    uVar4 = paVar9;
    local_a = uVar4;
    pass1_1038_4d6e(paVar9, CONCAT22(in_DX, uVar3));
    _local_e = CONCAT22(extraout_DX, uVar4);
    local_10 = 0;
    ppcVar1 = (*_local_e + 0x10);
    uVar3 = uVar4;
    uVar13 = extraout_DX;
    (**ppcVar1)(&PTR_LOOP_1050_1038, uVar4, extraout_DX);
    _local_14 = CONCAT22(extraout_DX_00, uVar4);
    pass1_1030_bcae(local_16, unaff_SS);
    local_1a = 0;
    loop {
        if (_local_14 <= local_1a) {
            // LAB_1028_77e7:
            if (_local_e != 0x0) {
                ppcVar1 = *_local_e;
                (**ppcVar1)(
                    0x1030,
                    _local_e,
                    (_local_e >> 0x10),
                    1,
                    uVar3,
                    uVar13,
                    _local_e,
                    _local_e,
                );
            }
            return;
        }
        uVar7 = _local_14;
        pass1_1030_1d58(_local_e);
        uVar10 = uVar7;
        uVar11 = (uVar7 >> 8);
        uVar8 = extraout_DX_01;
        uVar12 = extraout_DX_01;
        uVar2 = pass1_1028_b58e(param_2_00);
        puVar5 = local_16;
        pass1_1030_bd74(
            puVar5,
            unaff_SS,
            (CONCAT31(extraout_var, uVar2) & 0xffff | uVar8 << 0x10),
            CONCAT22(uVar12, CONCAT11(uVar11, uVar10)),
        );
        if (puVar5 <= param_1_00) {
            local_10 = 1;
            // goto LAB_1028_77e7;
        }
        local_1a = local_1a + 1;
    }
}

pub fn pass1_1028_780c(param_1: u16, param_2: u16, param_1_00: u32) {
    let ppcVar1: fn();
    let mut uVar2: i32;
    let mut uVar3: i32;
    let paVar4: *mut astruct_493;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let puVar9: *mut u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x25);
    uVar2 = puVar5;
    uVar8 = SUB42(&PTR_LOOP_1050_1038, 0);
    pass1_1038_4e78(param_1_00, puVar5 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, uVar2);
    ppcVar1 = (*_local_a + 0x10);
    uVar3 = uVar2;
    (**ppcVar1)(&PTR_LOOP_1050_1038, uVar2, extraout_DX);
    _local_e = CONCAT22(extraout_DX_00, uVar3);
    if ((extraout_DX_00 | uVar3) == 0) {
        return;
    }
    local_12 = 0;
    while (local_12 < _local_e) {
        ppcVar1 = (*_local_a + 4);
        uVar6 = _local_e;
        (**ppcVar1)();
        uVar7 = extraout_DX_01;
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6, extraout_DX_01);
        uVar8 = 0x1030;
        puVar9 = pass1_1030_73a8(CONCAT22(uVar7, paVar4));
        unsafe { ppcVar1 = (*puVar9 + 0x24) };
        (**ppcVar1)();
        local_12 = local_12 + 1;
    }
    if (_local_a != 0x0) {
        ppcVar1 = *_local_a;
        (**ppcVar1)(uVar8, uVar2, extraout_DX, 1);
    }
    return;
}

pub fn pass1_1028_78b8(param_1: u32) {
    let paVar1: *mut astruct_493;
    let puVar2: *mut u32;
    let paVar3: *mut astruct_1106;
    let puVar4: *mut u16;
    let puVar5: *mut u16;
    let puVar6: *mut u16;
    let mut in_DX: u16;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let mut iVar9: i32;
    let mut uVar10: u32;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let ppVar13: *mut pass1_struct_1;
    let uVar14: u8;
    let uVar15: u8;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut uVar18: u16;
    let mut uVar19: u16;
    let mut uStack340: u16;
    let mut uStack338: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut uStack70: u16;
    let mut uStack68: i32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut local_6: u32;

    local_6 = *_PTR_LOOP_1050_65e2;
    uVar14 = unaff_SS;
    uVar15 = (unaff_SS >> 8);
    if (local_6 == 0x98) {
        paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
        _local_10 = CONCAT22(in_DX, paVar1);
        if (&paVar1[0x11].field_0x2 == 0x8000002) {
            pass1_1020_a43e(CONCAT22(unaff_SS, &local_18));
            zero_list_1008_3e38(CONCAT22(unaff_SS, &local_1e));
            puVar2 = &local_18;
            pass1_1020_a49a(
                CONCAT13(uVar15, CONCAT12(uVar14, puVar2)),
                CONCAT22(unaff_SS, &local_1e),
                0x7a,
            );
            pass1_1038_4f54(_local_10, 1);
            if (puVar2 == 0x0) {
                pass1_1020_a49a(CONCAT13(uVar15, CONCAT12(uVar14, &local_18)), 0, 0x35);
            }
        }
    }
    if ((0xe < local_6) && (local_6 < 0x16)) {
        pass1_1020_a43e(CONCAT22(unaff_SS, &local_1e));
        local_18 = local_6 - 0xf;
        pass1_1020_a54c(&local_1e, unaff_SS, local_18);
    }
    uVar10 = local_6 % 0x7d;
    uVar8 = uVar10;
    if (uVar10 == 0) {
        local_1e = uVar8;
        pass1_fn_1008_612e(1, 100);
        uVar8 = uVar10;
        if (local_1e < 0x1a) {
            pass1_1028_dc52(
                CONCAT13(uVar15, CONCAT12(uVar14, &local_30)),
                (&PTR_LOOP_1050_0000 + 1),
                0,
                0x400,
            );
            while {
                uVar7 = uVar10;
                paVar3 = &local_30;
                pass1_1028_e4ec(CONCAT22(unaff_SS, paVar3));
                local_18 = CONCAT22(uVar7, paVar3);
                uVar8 = uVar7 | paVar3;
                uVar10 = uVar8;
                if (uVar8 == 0) {}
                // goto LAB_1028_79d6;
                &paVar3[1].field_0xb0 == 0x8000002
            } {}
            pass1_1038_43cc(paVar3, CONCAT22(1, uVar7), 4);
            // LAB_1028_79d6:
            local_30 = s_1_1050_389a;
            local_2e = &PTR_LOOP_1050_1008;
        }
    }
    if (local_6 == 5) {
        uVar17 = SUB42(&g_alloc_addr_1050_1050, 0);
        uVar16 = SUB42(s_Rebel_1050_4ffc, 0);
        local_30 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
        local_2e = uVar8;
        pass1_1038_4d3c(CONCAT22(uVar8, local_30), CONCAT22(uVar17, uVar16));
    }
    if (local_6 == 300) {
        uVar17 = 0x400;
        uVar19 = 0xf;
        uVar16 = 1;
        ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
        uVar8 = (ppVar13 >> 0x10);
        local_30 = ppVar13;
        local_2e = uVar8;
        pass1_1010_043a(ppVar13, CONCAT22(uVar17, uVar16), uVar19);
    }
    if (local_6 == 0x3d) {
        ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
        uVar10 = ppVar13 >> 0x10;
        local_30 = ppVar13;
        uVar8 = (ppVar13 >> 0x10);
        local_1e = u16_1050_13ae;
        local_2e = uVar8;
        if (u16_1050_13ae != 1) {
            pass1_1028_dc52(
                CONCAT13(uVar15, CONCAT12(uVar14, &local_42)),
                (&PTR_LOOP_1050_0000 + 1),
                0,
                0x400,
            );
            while (true) {
                uVar8 = uVar10;
                puVar4 = &local_42;
                pass1_1028_e4ec(CONCAT22(unaff_SS, puVar4));
                local_18 = CONCAT22(uVar8, puVar4);
                uVar10 = (uVar8 | puVar4);
                if ((uVar8 | puVar4) == 0) {
                    break;
                }
                _local_10 = (puVar4 + 0xfb);
                pass1_1030_34da(_local_10);
            }
            uVar17 = 0x400;
            uVar19 = 0x10;
            uVar16 = 1;
            ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
            uVar8 = (ppVar13 >> 0x10);
            local_14 = ppVar13;
            local_12 = uVar8;
            pass1_1010_043a(ppVar13, CONCAT22(uVar17, uVar16), uVar19);
            _local_42 = 0x1008389a;
        }
    }
    if (local_6 == 0x96) {
        paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
        _local_4a = CONCAT22(uVar8, paVar1);
        uVar19 = (param_1 >> 0x10);
        pass1_1028_780c(param_1, uVar19, CONCAT22(uVar8, paVar1));
        if (paVar1 != 0x0) {
            uVar17 = 0x400;
            uVar18 = 0x1d;
            uVar16 = 1;
            ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
            uVar8 = (ppVar13 >> 0x10);
            uStack70 = ppVar13;
            uStack68 = uVar8;
            pass1_1010_043a(ppVar13, CONCAT22(uVar17, uVar16), uVar18);
        }
        paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
        _local_4a = CONCAT22(uVar8, paVar1);
        pass1_1028_780c(param_1, uVar19, CONCAT22(uVar8, paVar1));
    }
    ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    uStack8 = (ppVar13 >> 0x10);
    uStack10 = SUB42(ppVar13, 0);
    uStack12 = u16_1050_13ae;
    if (2 < u16_1050_13ae) {
        _local_4a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
        uStack70 = 1;
        while (uStack70 < 9) {
            _local_42 = (_local_4a + 0x34 + uStack70 * 4);
            if (_local_42 == local_6) {
                uVar19 = 1;
                local_30 = 1;
                pass1_fn_1008_612e(1, 100);
                puVar6 = (uStack70 - 7);
                if (puVar6 == 0x0) {
                    bVar12 = SBORROW2(uVar19, 0x32);
                    iVar9 = uVar19 - 0x32;
                    bVar11 = uVar19 == 0x32;
                    // LAB_1028_7b74:
                    if ((!bVar11 && bVar12) == (iVar9 < 0)) {
                        local_30 = 0;
                    }
                } else {
                    puVar6 = (uStack70 - 8);
                    if (puVar6 == 0x0) {
                        bVar12 = SBORROW2(uVar19, 0x19);
                        iVar9 = uVar19 - 0x19;
                        bVar11 = iVar9 == 0;
                        // goto LAB_1028_7b74;
                    }
                }
                local_1e = uVar19;
                if (local_30 != 0) {
                    pass1_1028_90e6(CONCAT22(unaff_SS, &uStack340), uStack70);
                    puVar6 = &uStack340;
                    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, puVar6));
                    uStack340 = SUB42(s_1_1050_389a, 0);
                    uStack338 = SUB42(&PTR_LOOP_1050_1008, 0);
                }
                pass1_fn_1008_612e(0, 10);
                local_18 = local_18 & 0xffff0000 | ZEXT24(puVar6);
                if (uStack70 == 7) {
                    uVar19 = 7;
                    puVar5 = puVar6 + 0x37;
                    iVar9 = puVar5 >> 0xf;
                } else {
                    if (uStack70 != 8) {}
                    // goto LAB_1028_7ba0;
                    uVar19 = 8;
                    puVar5 = puVar6 + 0x32;
                    iVar9 = (puVar6 >> 0xf) + (0xff9b < puVar6);
                }
                uVar18 = (_local_42 >> 0x10) + iVar9 + CARRY2(local_42, puVar5);
                _local_42 = CONCAT22(uVar18, local_42 + puVar5);
                pass1_1010_ebf8(_local_4a, local_42 + puVar5, uVar18, uVar19);
            }
            // LAB_1028_7ba0:
            uStack70 = uStack70 + 1;
        }
    }
    return;
}

pub fn pass1_1028_7c4e(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let puVar4: *mut u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let puVar7: *mut u8;
    let mut uVar8: u32;
    let mut uVar9: i32;
    let mut uVar10: u32;
    let uVar11: u8;
    let mut unaff_SS: u16;
    let ppVar12: *mut pass1_struct_1;
    let mut uVar13: u16;
    let mut local_176: u32;
    let mut local_172: u32;
    let mut local_156: u16;
    let mut local_154: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u32;
    let mut local_38: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar12 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_176, 2));
    local_4 = (ppVar12 >> 0x10);
    local_6 = ppVar12;
    local_8 = u16_1050_13ae;
    if (2 < u16_1050_13ae) {
        local_c = *_PTR_LOOP_1050_65e2;
        local_c._2_2_ = (local_c >> 0x10);
        if (2 < local_c) {
            local_10 = local_c - 2;
            local_e = local_c._2_2_ - (local_c < 2);
            uVar10 = CONCAT22(local_e, local_10) % 0x14;
            if (uVar10 == 0) {
                pass1_1028_dc52(
                    CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_22)),
                    (&PTR_LOOP_1050_0000 + 1),
                    0,
                    0x400,
                );
                while (true) {
                    uVar9 = uVar10;
                    puVar4 = &local_22;
                    pass1_1028_e4ec(CONCAT22(unaff_SS, puVar4));
                    _local_26 = CONCAT22(uVar9, puVar4);
                    uVar10 = (uVar9 | puVar4);
                    if ((uVar9 | puVar4) == 0) {
                        break;
                    }
                    if ((puVar4 + 0x100) != 0x8000002) {
                        puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2a);
                        uVar5 = puVar7;
                        local_28 = uVar10;
                        uVar11 = 0x38;
                        local_2a = uVar5;
                        pass1_1038_4d6e(_local_26, puVar7 & 0xffff | uVar10 << 0x10);
                        _local_2e = CONCAT22(uVar10, uVar5);
                        ppcVar3 = (*_local_2e + 0x10);
                        (**ppcVar3)(&PTR_LOOP_1050_1038, uVar5, uVar10);
                        _local_32 = CONCAT22(uVar10, uVar5);
                        if (local_8 == 3) {
                            local_34 = 6;
                        } else {
                            local_34 = 0xc;
                        }
                        local_38 = 0;
                        while (uVar13 = (_local_2e >> 0x10), local_38 < _local_32) {
                            uVar8 = _local_32;
                            pass1_1030_1d7c(_local_2e, uVar13, local_38, (local_38 >> 0x10));
                            uVar6 = uVar8;
                            local_40 = uVar8 & 0xffff | uVar10 << 0x10;
                            pass1_1028_7742(
                                param_1,
                                (param_1 >> 0x10),
                                4,
                                (uVar8 & 0xffff | uVar10 << 0x10),
                            );
                            uVar5 = local_34;
                            if (uVar6 == 0) {
                                uVar5 = 0x19;
                            }
                            uVar11 = 8;
                            local_44 = uVar5;
                            local_42 = uVar6;
                            pass1_fn_1008_612e(1, 100);
                            local_46 = uVar5;
                            if (uVar5 <= local_44) {
                                uVar1 = (_local_26 + 4);
                                uVar2 = (local_40 + 4);
                                pass1_1028_8fc0(
                                    &local_156,
                                    unaff_SS,
                                    uVar2,
                                    (uVar2 >> 0x10),
                                    uVar1,
                                    (uVar1 >> 0x10),
                                );
                                uVar11 = 0x30;
                                pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_156));
                                local_156 = s_1_1050_389a;
                                local_154 = &PTR_LOOP_1050_1008;
                            }
                            local_38 = local_38 + 1;
                        }
                        local_176._0_2_ = SUB42(_local_2e, 0);
                        if (_local_2e != 0x0) {
                            ppcVar3 = *_local_2e;
                            (**ppcVar3)(uVar11, local_176, uVar13, 1, local_176, uVar13, _local_2e);
                        }
                    }
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_7dfc(param_1: u32) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let puVar4: *mut u16;
    let mut uVar5: u16;
    let puVar6: *mut u8;
    let mut uVar7: u32;
    let mut uVar8: i32;
    let mut uVar9: i32;
    let mut uVar10: u32;
    let uVar11: u8;
    let mut unaff_SS: u16;
    let ppVar12: *mut pass1_struct_1;
    let mut uVar13: u16;
    let mut local_178: u32;
    let mut local_174: u32;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_3c: u32;
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
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar12 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_178, 2));
    local_4 = (ppVar12 >> 0x10);
    local_6 = ppVar12;
    local_8 = u16_1050_13ae;
    if (2 < u16_1050_13ae) {
        local_c = *_PTR_LOOP_1050_65e2;
        local_c._2_2_ = (local_c >> 0x10);
        if (3 < local_c) {
            local_10 = local_c - 3;
            local_e = local_c._2_2_ - (local_c < 3);
            uVar10 = local_c % 0x14;
            if (uVar10 == 0) {
                pass1_1028_dc52(
                    CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_22)),
                    (&PTR_LOOP_1050_0000 + 1),
                    0,
                    0x400,
                );
                while (true) {
                    uVar8 = uVar10;
                    puVar4 = &local_22;
                    pass1_1028_e4ec(CONCAT22(unaff_SS, puVar4));
                    _local_26 = CONCAT22(uVar8, puVar4);
                    uVar9 = uVar8 | puVar4;
                    uVar10 = uVar9;
                    if (uVar9 == 0) {
                        break;
                    }
                    if ((puVar4 + 0x100) != 0x8000002) {
                        puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x29);
                        uVar5 = puVar6;
                        local_2a = uVar5;
                        local_28 = uVar9;
                        pass1_1038_4d6e(_local_26, puVar6 & 0xffff | uVar9 << 0x10);
                        _local_2e = CONCAT22(uVar9, uVar5);
                        ppcVar3 = (*_local_2e + 0x10);
                        (**ppcVar3)(&PTR_LOOP_1050_1038, uVar5, uVar9);
                        _local_32 = CONCAT22(uVar9, uVar5);
                        uVar11 = 0x10;
                        ppVar12 = process_struct_1010_20ba(
                            _g_astruct_372_1050_0ed0,
                            CONCAT22(local_178, 2),
                        );
                        uVar10 = ppVar12 >> 0x10;
                        local_38 = ppVar12;
                        local_36 = (ppVar12 >> 0x10);
                        if (local_8 == 3) {
                            local_34 = 5;
                        } else {
                            local_34 = 0x1e;
                        }
                        local_3c = 0;
                        while (uVar13 = (_local_2e >> 0x10), local_3c < _local_32) {
                            uVar7 = _local_32;
                            pass1_1030_1d7c(_local_2e, uVar13, local_3c, (local_3c >> 0x10));
                            uVar5 = uVar7;
                            _local_44 = (uVar7 & 0xffff | uVar10 << 0x10);
                            uVar11 = 8;
                            pass1_fn_1008_612e(1, 100);
                            local_46 = uVar5;
                            if ((uVar5 <= local_34)
                                && (
                                    pass1_1028_7742(param_1, (param_1 >> 0x10), 4, _local_44),
                                    local_48 = uVar5,
                                    uVar5 == 0,
                                ))
                            {
                                uVar1 = (_local_26 + 4);
                                uVar2 = (_local_44 + 4);
                                pass1_1028_b0de(
                                    &local_158,
                                    unaff_SS,
                                    uVar2,
                                    (uVar2 >> 0x10),
                                    uVar1,
                                    (uVar1 >> 0x10),
                                );
                                uVar11 = 0x30;
                                pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_158));
                                local_158 = s_1_1050_389a;
                                local_156 = &PTR_LOOP_1050_1008;
                            }
                            local_3c = local_3c + 1;
                        }
                        local_178._0_2_ = SUB42(_local_2e, 0);
                        if (_local_2e != 0x0) {
                            ppcVar3 = *_local_2e;
                            (**ppcVar3)(uVar11, local_178, uVar13, 1, local_178, uVar13, _local_2e);
                        }
                    }
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_7fb6(param_1: u32) {
    let ppcVar1: fn();
    let p_uvar2: *mut u16;
    let mut uvar3: u16;
    let puVar4: *mut u8;
    let mut uVar5: u32;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let mut uVar8: u32;
    let uVar9: u8;
    let mut unaff_SS: u16;
    let ppVar10: *mut pass1_struct_1;
    let paVar11: *mut astruct_500;
    let uVar12: u8;
    let mut uVar13: u16;
    let mut local_178: u32;
    let mut local_174: u32;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_48: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u32;
    let mut local_38: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = *_PTR_LOOP_1050_65e2;
    local_6._2_2_ = (local_6 >> 0x10);
    if (0xb < local_6) {
        local_a = local_6 - 0xb;
        local_8 = local_6._2_2_ - (local_6 < 0xb);
        uVar8 = local_6 % 0x32;
        if (uVar8 == 0) {
            uVar12 = (unaff_SS >> 8);
            pass1_1028_dc52(
                CONCAT13(uVar12, CONCAT12(unaff_SS, &local_1c)),
                (&PTR_LOOP_1050_0000 + 1),
                0,
                0x400,
            );
            while (true) {
                uVar6 = uVar8;
                puVar2 = &local_1c;
                pass1_1028_e4ec(CONCAT22(unaff_SS, puVar2));
                _local_20 = CONCAT22(uVar6, puVar2);
                uVar7 = uVar6 | puVar2;
                uVar8 = uVar7;
                if (uVar7 == 0) {
                    break;
                }
                if ((puVar2 + 0x100) != 0x8000002) {
                    puVar4 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x11);
                    uVar3 = puVar4;
                    local_24 = uVar3;
                    local_22 = uVar7;
                    pass1_1038_4d6e(_local_20, puVar4 & 0xffff | uVar7 << 0x10);
                    _local_28 = CONCAT22(uVar7, uVar3);
                    ppcVar1 = (*_local_28 + 0x10);
                    (**ppcVar1)(&PTR_LOOP_1050_1038, uVar3, uVar7);
                    _local_2c = CONCAT22(uVar7, uVar3);
                    uVar9 = 0x10;
                    ppVar10 =
                        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_178, 2));
                    uVar8 = ppVar10 >> 0x10;
                    local_30 = ppVar10;
                    local_2e = (ppVar10 >> 0x10);
                    local_32 = u16_1050_13ae;
                    if (u16_1050_13ae < 3) {
                        local_34 = 5;
                    } else {
                        local_34 = 0x14;
                    }
                    local_38 = 0;
                    while (uVar13 = (_local_28 >> 0x10), local_38 < _local_2c) {
                        uVar9 = 0x30;
                        uVar5 = _local_2c;
                        pass1_1030_1d7c(_local_28, uVar13, local_38, (local_38 >> 0x10));
                        local_40 = uVar5 & 0xffff | uVar8 << 0x10;
                        uVar3 = (uVar5 + 0x20);
                        local_42 = uVar3;
                        if (((uVar3 != 0) && (uVar3 != 0x70)) && (uVar3 != 0x71)) {
                            uVar9 = 8;
                            pass1_fn_1008_612e(1, 100);
                            local_44 = uVar3;
                            if ((uVar3 <= local_34)
                                && (
                                    pass1_1028_7742(param_1, (param_1 >> 0x10), 4, local_40),
                                    local_48 = uVar3,
                                    uVar3 == 0,
                                ))
                            {
                                paVar11 = pass1_1028_8698(
                                    CONCAT13(uVar12, CONCAT12(unaff_SS, &local_158)),
                                    (local_40 + 4),
                                    (_local_20 + 4),
                                );
                                uVar8 = paVar11 >> 0x10;
                                uVar9 = 0x30;
                                pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_158));
                                local_158 = s_1_1050_389a;
                                local_156 = &PTR_LOOP_1050_1008;
                            }
                        }
                        local_38 = local_38 + 1;
                    }
                    local_178._0_2_ = SUB42(_local_28, 0);
                    if (_local_28 != 0x0) {
                        ppcVar1 = *_local_28;
                        (**ppcVar1)(uVar9, local_178, uVar13, 1, local_178, uVar13, _local_28);
                    }
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_816e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_752e(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_791;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = &local_AX__1.field_0x8;
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0x819a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_75bc() {
    let mut uVar1: i32;
    let p_uvar2: *mut u16;
    let mut uVar3: u32;
    let mut unaff_SS: u16;
    let mut local_20: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = *_PTR_LOOP_1050_65e2;
    uVar1 = (local_6 % 0x7b);
    uVar3 = uVar1;
    if ((uVar1 == 0) && (0x95 < local_6)) {
        pass1_1028_dc52(
            CONCAT22(unaff_SS, &local_18),
            (&PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            uVar1 = uVar3;
            puVar2 = &local_18;
            pass1_1028_e4ec(CONCAT22(unaff_SS, puVar2));
            _local_1c = CONCAT22(uVar1, puVar2);
            uVar3 = (uVar1 | puVar2);
            if ((uVar1 | puVar2) == 0) {
                break;
            }
            pass1_fn_1008_612e();
            if (puVar2 < 6) {
                win_fn_1038_362e(_local_1c);
            }
        }
        if (local_8 != 0) {
            local_c = 1;
            local_a = 0;
        }
        uVar3 = local_a;
        local_10 = local_c;
        local_e = local_a;
        while (true) {
            uVar1 = uVar3;
            puVar2 = &local_18;
            pass1_1028_e4ec(CONCAT22(unaff_SS, puVar2));
            uVar3 = (uVar1 | puVar2);
            if ((uVar1 | puVar2) == 0) {
                break;
            }
            pass1_1038_3698(CONCAT22(uVar1, puVar2));
        }
    }
    return;
}

pub fn pass1_1028_737e(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) != 0) {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        (in_AX + 4) = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x749e;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_740c(param_1: u16, param_2: u16, param_1: u16_00, param_2_00: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let puVar5: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, param_1_00);
    uVar3 = SUB42(puVar5, 0);
    pass1_1038_4d6e(param_2_00, puVar5 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, uVar3);
    uVar2 = *_local_a;
    ppcVar1 = uVar2 + 8;
    uVar4 = uVar3;
    (**ppcVar1)(&PTR_LOOP_1050_1038, uVar3, extraout_DX);
    _local_e = CONCAT22(extraout_DX_00, uVar4);
    if (_local_a != 0x0) {
        ppcVar1 = uVar2;
        (**ppcVar1)(&PTR_LOOP_1050_1038, uVar3, extraout_DX, 1);
    }
    if (_local_e != 0) {
        return;
    }
    return;
}

pub fn pass1_1028_7472(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6e96() {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let mut unaff_SS: u16;
    let puVar3: *mut u32;
    let mut local_20: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    while (true) {
        puVar3 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        if (puVar3 == 0x0) {
            break;
        }
        iVar1 = (puVar3 + 0x12);
        if (((0 < iVar1) && (!SBORROW2(iVar1, 1))) && (iVar1 + -1 < 4)) {
            unsafe {
                ppcVar2 = (*puVar3 + 0x38);
                ppcVar2();
            }
        }
    }
    return 1;
}

pub fn pass1_1028_6ef6(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_AX: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_DX: *mut astruct_199;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_DX);
    _local_a = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) != 0) {
        *_local_a = s_1_1050_389a;
        (in_AX + 2) = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        (in_AX + 4) = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar5 = (in_AX + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x6fb0;
        (in_AX + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_6f84(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6926(param_1: u32) {
    let mut iVar1: i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let paVar4: *mut astruct_493;
    let mut uVar5: i32;
    let puVar6: *mut u8;
    let mut in_DX: i32;
    let mut uVar7: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar9 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x108);
    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
    uVar7 = in_DX;
    puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 10);
    uVar5 = puVar6;
    uVar10 = SUB42(&PTR_LOOP_1050_1038, 0);
    pass1_1038_4d6e(CONCAT22(in_DX, paVar4), puVar6 & 0xffff | uVar7 << 0x10);
    _local_e = CONCAT22(extraout_DX, uVar5);
    uVar2 = *_local_e;
    ppcVar3 = (uVar2 + 0x10);
    uVar7 = uVar5;
    (**ppcVar3)(&PTR_LOOP_1050_1038, uVar5, extraout_DX);
    if ((extraout_DX_00 | uVar7) != 0) {
        ppcVar3 = (uVar2 + 4);
        (**ppcVar3)(0x38, uVar5, extraout_DX, 0, 0);
        uVar8 = extraout_DX_01;
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7, extraout_DX_01);
        iVar1 = (param_1 + 0x10c);
        uVar10 = 0x1030;
        pass1_1030_7ddc(
            CONCAT22(uVar8, paVar4),
            CONCAT13(iVar1 >> 0xf, iVar10, 0x1f),
        );
    }
    if (_local_e != 0x0) {
        ppcVar3 = *_local_e;
        (**ppcVar3)(uVar10, uVar5, extraout_DX, 1);
    }
    return;
}

pub fn pass1_1028_69cc(param_1: *mut astruct_788) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_787;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let local_BX_41: *mut astruct_788;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10e, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_41 = param_1;
        local_AX__1.field_0x4 = local_BX_41.field_0x4;
        puVar4 = &local_BX_41.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe { *puVar2 = *puVar1 };
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_41.field_0x108;
        local_AX__1.field_0x10c = local_BX_41.field_0x10c;
        *_local_a = 0x6ae2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_6a7a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6aa6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6af2(param_1: *mut astruct_500, param_2: u32, param_3: u32) {
    let local_BX_19: *mut astruct_500;
    let mut uVar1: u16;

    pass1_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_3;
    local_BX_19.field_0x10c = param_2;
    param_1.a = 0x6e50;
    local_BX_19.b = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_6b2c(param_1: u32) {
    pass1_1028_6b40(param_1);
    return 1;
}

pub fn pass1_1028_6b40(param_1: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let uVar3: u8;
    let paVar4: *mut astruct_493;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_36: [u8; 14];
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
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 2];
    let mut local_4: u16;

    pass1_1028_6daa(
        param_1,
        CONCAT22(unaff_SS, local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    uVar7 = (param_1 >> 0x10);
    uVar6 = param_1;
    uVar1 = (uVar6 + 0x10c);
    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    _local_a = CONCAT22(in_DX, paVar4);
    ppcVar2 = (_local_a + 0x24);
    ppcVar2();
    local_c = extraout_DX;
    uVar3 = pass1_1028_b58e(_local_a);
    local_e = CONCAT11(extraout_AH, uVar3);
    _local_12 = pass1_1028_bb24(_local_a);
    _local_18 = (local_e + 0xc);
    local_14 = (local_e + 0x10);
    local_28 = &local_18;
    local_1a = local_18;
    local_1c = (_local_18 >> 0x10);
    local_20 = local_18 - 1;
    if (local_20 < 0) {
        local_20 = 0;
    }
    uVar5 = local_4 - 1;
    local_22 = local_18 + 1;
    if (uVar5 < (local_18 + 1)) {
        local_22 = uVar5;
    }
    local_24 = local_1c - 1;
    if (local_24 < 0) {
        local_24 = 0;
    }
    local_26 = local_1c + 1;
    if (uVar5 < (local_1c + 1)) {
        local_26 = uVar5;
    }
    local_1e = local_14;
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_14, local_24, local_20);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_1e, local_24, local_1a);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_1e, local_24, local_22);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_1e, local_1c, local_20);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_1e, local_1c, local_22);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_1e, local_26, local_20);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_1e, local_26, local_1a);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_36), local_1e, local_26, local_22);
    pass1_1028_6d24(uVar6, uVar7, CONCAT22(unaff_SS, local_36), _local_12);
    return;
}

pub fn pass1_1028_6d24(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let ppcVar1: fn();
    let puVar2: *mut u32;
    let paVar3: *mut astruct_493;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let puVar5: *mut u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;
    let temp_5f5b5c35b8: *mut astruct_790;

    puVar2 = &local_a;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        param_2_00,
        puVar2,
        unaff_SS,
    );
    unsafe { local_6 = *puVar2 };
    uVar4 = (puVar2 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ != '\0') {
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, uVar4);
        puVar5 = pass1_1030_73a8(CONCAT22(uVar4, paVar3));
        temp_5f5b5c35b8 = (puVar5 + 0xc);
        if (((temp_5f5b5c35b8 < 1) || (SBORROW2(temp_5f5b5c35b8, 1)))
            || (temp_5f5b5c35b8 != &BYTE_1050_0009
                && 7 < (temp_5f5b5c35b8 + -1)
                && ((temp_5f5b5c35b8 + -9) < 0x6a || (6 < (temp_5f5b5c35b8 + -0x73)))))
        {
            unsafe {
                ppcVar1 = (*puVar5 + 0x24);
                (**ppcVar1)()
            };
        }
    }
    return;
}

pub fn pass1_1028_6daa(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut in_DX: u16;
    let puVar2: *mut u32;
    let mut local_1c: u16;
    let mut local_18: u32;
    let mut uStack20: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_1 + 0x10c);
    local_6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    _local_a = pass1_1028_b4f2(CONCAT22(in_DX, local_6));
    local_10 = (_local_a >> 0x10);
    local_e = (_local_a + 8);
    local_12 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_e, (local_e >> 0x10));
    puVar2 = pass1_1030_5b5c(local_12, local_10);
    unsafe { local_18 = *puVar2 };
    uStack20 = (puVar2 + 4);
    pass1_1008_3e94(&local_18, param_2, param_3);
    return;
}

pub fn pass1_1028_6e24(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6744(param_1: u32, param_2: u16) {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let lVar2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x20));
    while {
        lVar2 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        uVar1 = (lVar2 >> 0x10);
        if (lVar2 == 0) {
            return 0;
        }
        (lVar2 + 6) != param_2
    } {}
    return *(lVar2 + 10);
}

pub fn pass1_1028_678c(param_1: u32, param_2: u16) {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let lVar2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x20));
    while {
        lVar2 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        uVar1 = (lVar2 >> 0x10);
        if (lVar2 == 0) {
            return 0;
        }
        (lVar2 + 8) != param_2
    } {}
    return *(lVar2 + 10);
}

pub fn pass1_1028_67d4(param_1: u32) {
    let mut uVar1: i32;
    let mut unaff_SS: u16;
    let lVar2: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x20));
    local_12 = 0;
    while (true) {
        lVar2 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        if (lVar2 == 0) {
            break;
        }
        uVar1 = (lVar2 + 0xc);
        local_12 = CONCAT22(
            (local_12 >> 0x10) + CARRY2(local_12, uVar1),
            local_12 + uVar1,
        );
    }
    return local_12;
}

pub fn pass1_1028_6822(param_1: u32, param_2: *mut u32) {
    let mut iVar1: i32;
    let mut uVar2: u32;

    uVar2 = pass1_1028_67d4(param_1);
    iVar1 = (uVar2 >> 0x10);
    unsafe {
        *param_2 = uVar2;
        (param_2 + 2) = iVar1;
        if ((iVar1 == 0) && (*param_2 < 100)) {
            return 0;
        }
    }
    return 1;
}

pub fn pass1_1028_6850(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_6186(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6008(param_1: u32) {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    pass1_1028_be2a(param_1);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x12) == 5) && (0 < (iVar2 + 0x20))) {
        piVar1 = (iVar2 + 0x20);
        unsafe { *piVar1 = *piVar1 + -1 };
    }
    return;
}

pub fn pass1_1028_602e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: This function may have set the stack p i32er

pub fn pass1_1028_6064(param_1: u32, param_2: u8) {
    let pcVar1: *mut char;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut in_AX: i32;
    let mut bVar4: u8;
    let mut bVar5: u8;
    let mut in_DX: u16;
    let mut in_BX: i32;
    let mut cVar6: u8;
    let puVar7: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_DI: i32;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar8: bool;

    puVar7 = &stack0xfffe;
    cVar6 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar7 = puVar7 + -1;
        unsafe { *puVar7 = *unaff_BP };
        cVar6 = cVar6 + -1;
        '\0' < cVar6
    } {}
    pcVar1 = (in_BX + unaff_SI);
    bVar5 = in_DX;
    unsafe { *pcVar1 = *pcVar1 - bVar5 };
    pcVar1 = (&PTR_LOOP_1050_1028 + unaff_DI);
    cVar6 = (in_BX >> 8);
    unsafe {
        if (*pcVar1 != cVar6 && cVar6 <= *pcVar1) {
            pbVar2 = (in_BX + unaff_SI);
            bVar3 = *pbVar2;
            *pbVar2 = *pbVar2 - bVar5;
            pcVar1 = (in_BX + unaff_SI);
            *pcVar1 = *pcVar1 - bVar5;
            pbVar2 = (in_BX + unaff_SI + 0x28);
            bVar4 = ((in_AX & 0xff00) >> 8);
            *pbVar2 = *pbVar2 | bVar4;
            bVar8 = CARRY1(s_fem79_wav_1050_28ba[5], bVar5);
            s_fem79_wav_1050_28ba[5] = s_fem79_wav_1050_28ba[5] + bVar5;
            pcVar1 = &stack0xfffe + unaff_SI;
            *pcVar1 = *pcVar1 + in_BX + bVar8;
            0x1026 = &g_alloc_addr_1050_1050;
            pbVar2 = (in_BX + 0x28);
            *pbVar2 = *pbVar2 ^ in_BX;
            pcVar1 = &stack0xfffe + unaff_SI;
            *pcVar1 = *pcVar1 + bVar4;
            LOCK();
            pcVar1 = (CONCAT11(*(&PTR_LOOP_1050_1028 + unaff_SI) + '\x10', 0x28) + unaff_SI);
            *pcVar1 = *pcVar1 - bVar5;
            return CONCAT22(
                in_DX,
                in_AX & 0xff00 | ((in_AX - in_CF) + -7 + (bVar3 < bVar5)),
            );
        }
    }
    0x1024 = param_1;
    0x1022 = unaff_CS;
    0x1020 = 0x603b;
    pass1_1028_b418(paRam00001024);
    if ((param_2 & 1) != 0) {
        0x1024 = param_1;
        0x1022 = unaff_CS;
        0x1020 = 0x604a;
        error_check_1000_17ce(paRam00001024);
    }
    return param_1;
}

pub fn pass1_1028_60bc(param_1: *mut astruct_763) -> *mut astruct_763 {
    let paVar1: *mut astruct_199;
    let mut uVar2: i32;
    let local_BX_13: *mut astruct_784;
    let mut uvar3: u16;
    let paVar4: *mut astruct_199;
    let mut local_4: u16;

    paVar4 = pass1_1028_b354(param_1);
    uVar3 = (param_1 >> 0x10);
    local_BX_13 = param_1;
    &local_BX_13.field_0x20 = 0;
    param_1.field_0x0 = 0x6876;
    local_BX_13.field_0x2 = &PTR_LOOP_1050_1028;
    process_struct_1000_179c(0xc, (paVar4 >> 0x10));
    uVar2 = (paVar4 >> 0x10) | paVar4;
    if (paVar4 == 0x0) {
        &local_BX_13.field_0x20 = 0;
    } else {
        paVar1 = process_struct_1008_574a(paVar4);
        local_BX_13.field_0x20 = paVar1;
        local_BX_13.field_0x22 = uVar2;
    }
    return param_1;
}

pub fn pass1_1028_611e(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    let paVar1: *mut astruct_199;
    let mut uVar2: i32;
    let paVar3: *mut astruct_199;
    let mut local_4: u16;

    paVar3 = pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    (param_1 + 0x20) = 0;
    CONCAT22(param_2, param_1) = 0x6876;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    process_struct_1000_179c(0xc, (paVar3 >> 0x10));
    uVar2 = (paVar3 >> 0x10) | paVar3;
    if (paVar3 == 0x0) {
        (param_1 + 0x20) = 0;
    } else {
        paVar1 = process_struct_1008_574a(paVar3);
        (param_1 + 0x20) = paVar1;
        (param_1 + 0x22) = uVar2;
    }
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_6186(param_1: *mut astruct_44) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.ptr_a_lo = 0x6876;
    (iVar4 + 2) = &PTR_LOOP_1050_1028;
    puVar1 = (iVar4 + 0x20);
    uVar2 = (iVar4 + 0x22);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_61c4(param_1: *mut astruct_781, param_2: *mut u8) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let paVar3: *mut astruct_199;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let paVar8: *mut astruct_199;
    let mut local_8: u16;

    pass1_1028_b46e(param_1, param_2);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    puVar1 = (iVar6 + 0x20);
    uVar5 = (iVar6 + 0x22);
    uVar4 = uVar5 | puVar1;
    paVar8 = CONCAT22(uVar4, puVar1);
    if (uVar4 != 0) {
        unsafe {
            ppcVar2 = *puVar1;
            paVar8 = ppcVar2();
        }
    }
    process_struct_1000_179c(0xc, (paVar8 >> 0x10));
    uVar5 = (paVar8 >> 0x10) | paVar8;
    if (paVar8 == 0x0) {
        paVar3 = 0x0;
        uVar5 = 0;
    } else {
        paVar3 = process_struct_1008_574a(paVar8);
    }
    (iVar6 + 0x20) = paVar3;
    (iVar6 + 0x22) = uVar5;
    return;
}

pub fn pass1_1028_6228(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut bVar8: bool;
    let lVar9: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar6 + 0x20));
    loop {
        while {
            lVar9 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
            uVar5 = (lVar9 >> 0x10);
            iVar4 = lVar9;
            if (lVar9 == 0) {
                return;
            }
            (iVar4 + 6) != param_4
        } {}
        uVar1 = (iVar4 + 10);
        if ((param_3 == 0) && (param_2 < uVar1)) {
            break;
        }
        bVar8 = param_2 < uVar1;
        param_2 = param_2 - uVar1;
        param_3 = param_3 - bVar8;
        ppcVar3 = ((iVar6 + 0x20) + 0xc);
        (**ppcVar3)(&PTR_LOOP_1050_1008, (iVar6 + 0x20));
        local_6 = 0;
    }
    uVar2 = (iVar4 + 0xc);
    (iVar4 + 10) = uVar1 - param_2;
    (iVar4 + 0xc) = -(param_2 * (uVar2 / uVar1) - (iVar4 + 0xc));
    return;
}

pub fn pass1_1028_62c8(param_1: u32) {
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 5) {
        uVar2 = pass1_1028_67d4(param_1 & 0xffff | uVar1 << 0x10);
        uVar1 = uVar2;
        if (((uVar2 >> 0x10) == 0) && (uVar1 < 100)) {
            return CONCAT22(-(100 < uVar1), 100 - uVar1);
        }
    }
    return 0;
}

pub fn pass1_1028_6302(param_1: u32) {
    let mut uVar1: i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let lVar3: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x20));
    local_12 = 0;
    loop {
        lVar3 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        uVar2 = (lVar3 >> 0x10);
        if (lVar3 == 0) {
            break;
        }
        if ((lVar3 + 8) != 0) {
            uVar1 = (lVar3 + 10);
            local_12 = CONCAT22(
                (local_12 >> 0x10) + CARRY2(local_12, uVar1),
                local_12 + uVar1,
            );
        }
    }
    return local_12;
}

pub fn pass1_1028_6356(param_1: u32, param_2: i32, uparam_3: i32, param_4: i32) {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let ppcVar4: fn();
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let mut bVar9: bool;
    let lVar10: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar7 + 0x20));
    loop {
        while {
            lVar10 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
            uVar6 = (lVar10 >> 0x10);
            iVar5 = lVar10;
            if (lVar10 == 0) {
                return;
            }
            (((iVar5 + 8) == 0) || (param_2 != 0 && ((iVar5 + 8) != param_2)))
                || ((iVar5 + 8) == 0xf && (param_2 != 0xf))
        } {}
        uVar2 = (iVar5 + 10);
        if ((param_4 == 0) && (param_3 < uVar2)) {
            break;
        }
        bVar9 = param_3 < uVar2;
        param_3 = param_3 - uVar2;
        param_4 = param_4 - bVar9;
        ppcVar4 = ((iVar7 + 0x20) + 0xc);
        (**ppcVar4)(&PTR_LOOP_1050_1008, (iVar7 + 0x20));
        local_6 = 0;
    }
    uVar3 = (iVar5 + 0xc);
    piVar1 = (iVar5 + 10);
    unsafe {
        *piVar1 = *piVar1 - param_3;
        piVar1 = (iVar5 + 0xc);
        *piVar1 = *piVar1 - param_3 * (uVar3 / uVar2);
    }
    return;
}

pub fn pass1_1028_6408(param_1: u32, param_2: *mut u32) {
    let ppcVar1: fn();
    let mut bVar2: bool;
    let puVar3: *mut u8;
    let mut extraout_DX: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar4 + 0x20));
    bVar2 = false;
    loop {
        puVar3 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar3));
        iVar5 = param_2;
        uVar7 = (param_2 >> 0x10);
        if ((extraout_DX | puVar3) == 0) {
            break;
        }
        if (((puVar3 + 4) == (iVar5 + 4)) && ((puVar3 + 6) == (iVar5 + 6))) {
            if ((puVar3 + 8) == (iVar5 + 8)) {
                bVar2 = true;
                (puVar3 + 10) = (puVar3 + 10) + (iVar5 + 10);
                (puVar3 + 0xc) = (puVar3 + 0xc) + (iVar5 + 0xc);
            }
        }
    }
    if (bVar2) {
        if (param_2 != 0x0) {
            unsafe {
                ppcVar1 = *param_2;
                (**ppcVar1)(&PTR_LOOP_1050_1008, param_2, 1, param_2, param_2);
            }
            return;
        }
    } else {
        ppcVar1 = ((iVar4 + 0x20) + 4);
        (**ppcVar1)(&PTR_LOOP_1050_1008, (iVar4 + 0x20), param_2);
    }
    return;
}

pub fn pass1_1028_4a9a(param_1: u32, param_2: i32) {
    return (param_1 + 0x20 + param_2 * 2);
}

pub fn pass1_1028_4ab2(param_1: u32, param_2: u16, param_3: i32) {
    (param_1 + param_3 * 2 + 0x20) = param_2;
    return;
}

pub fn pass1_1028_4aca(param_1: u32) {
    let paVar1: *mut astruct_865;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000fff6: u16;

    if ((param_1 + 0x20) != 0) {
        ppVar2 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff6, 0x2f));
        paVar1 = pass1_1010_ed3e(ppVar2);
        pass1_1030_2a2c(paVar1);
    }
    return;
}

pub fn pass1_1028_4af6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_4b84(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_SinternalPutBldg2_site_0x_08lx__1050_506f + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_4ba6(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = (s_SinternalPutBldg2_site_0x_08lx__1050_506f + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_4bd0(param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (((param_1 + 0x12) != 6) && ((param_1 + 0x12) != 5)) {
        return 0;
    }
    return 1;
}

pub fn pass1_1028_4bf2(param_1: *mut astruct_44, param_2: *mut u8) {
    let ppcVar1: fn();
    let uVar2: u8;
    let puVar3: *mut u32;
    let paVar4: *mut astruct_493;
    let mut iVar5: i32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_2c: [u8; 6];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar2);
    local_14 = (local_6 + 0xc);
    local_8 = (local_6 + 0x10);
    local_1a = &local_c;
    local_22 = CONCAT22(local_22._2_2_, &local_14);
    local_10 = local_8 + 1;
    uVar8 = CONCAT22(unaff_SS, local_2c);
    local_e = local_10;
    local_c = local_14;
    uVar7 = pass1_1028_bb24(param_1);
    puVar3 = &local_14;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        puVar3,
        unaff_SS,
        uVar7,
        (uVar7 >> 0x10),
        uVar8,
    );
    unsafe { local_22 = *puVar3 };
    uVar6 = (puVar3 + 2);
    local_36._3_1_ = (local_22 >> 0x18);
    local_18 = local_22;
    if (local_36._3_1_ != '\0') {
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_22, uVar6);
        local_36 = CONCAT22(uVar6, paVar4);
        uVar2 = pass1_1030_6fa0(CONCAT22(uVar6, paVar4));
        iVar5 = CONCAT31(extraout_var_00, uVar2);
        if ((iVar5 == 0x62) || (iVar5 == 99)) {
            _local_26 = pass1_1030_73a8(local_36);
            ppcVar1 = (*_local_26 + 0x58);
            (**ppcVar1)(0x1030, _local_26, (_local_26 >> 0x10), param_2);
        }
    }
    pass1_1028_b46e(param_1, param_2);
    return;
}

pub fn pass1_1028_4cd6(param_1: *mut astruct_44) {
    let ppcVar1: fn();
    let uVar2: u8;
    let puVar3: *mut u32;
    let paVar4: *mut astruct_493;
    let mut iVar5: i32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_2c: [u8; 6];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_b514(param_1);
    uVar2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar2);
    local_14 = (local_6 + 0xc);
    local_8 = (local_6 + 0x10);
    local_1a = &local_c;
    local_22 = CONCAT22(local_22._2_2_, &local_14);
    local_10 = local_8 + 1;
    uVar8 = CONCAT22(unaff_SS, local_2c);
    local_e = local_10;
    local_c = local_14;
    uVar7 = pass1_1028_bb24(param_1);
    puVar3 = &local_14;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        puVar3,
        unaff_SS,
        uVar7,
        (uVar7 >> 0x10),
        uVar8,
    );
    unsafe { local_22 = *puVar3 };
    uVar6 = (puVar3 + 2);
    local_36._3_1_ = (local_22 >> 0x18);
    if (local_36._3_1_ != '\0') {
        local_18 = local_22;
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_22, uVar6);
        local_36 = CONCAT22(uVar6, paVar4);
        uVar2 = pass1_1030_6fa0(CONCAT22(uVar6, paVar4));
        iVar5 = CONCAT31(extraout_var_00, uVar2);
        if ((iVar5 == 0x62) || (iVar5 == 99)) {
            _local_26 = pass1_1030_73a8(local_36);
            ppcVar1 = (*_local_26 + 0x24);
            (**ppcVar1)();
        }
    }
    return;
}

pub fn pass1_1028_4db2(param_1: i32, param_2: u16, param_2_00: i32) {
    let uVar1: u8;
    let BVar2: bool;
    let mut iVar3: i32;
    let extraout_var: u32;
    let mut uVar4: i32;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let ppVar5: *mut pass1_struct_1;
    let puVar6: *mut u16;
    let mut uVar7: u16;
    let puVar8: *mut u16;
    let mut uVar9: u16;
    let mut local_152: u16;
    let mut local_150: u16;
    let mut local_14e: u16;
    let mut local_14c: u16;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: [u8; 2];
    let mut local_1e: [u8; 2];
    let mut local_1c: u32;
    let mut uStack24: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (param_1 + 0xc), 0x29);
    if (BVar2 != 0) {
        pass1_1028_bd38(CONCAT22(param_2, param_1));
        if ((param_2_00 == 0) && ((param_1 + 0xc) == 0x13)) {
            ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 8));
            pass1_1010_988c(ppVar5, (param_1 + 0xc));
        }
        _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
        uVar4 = (_local_6 >> 0x10);
        local_a = (_local_6 + 0x20);
        puVar8 = &local_c;
        puVar6 = &local_e;
        uVar7 = unaff_SS;
        uVar9 = unaff_SS;
        local_12 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
        local_10 = uVar4;
        pass1_1030_5b1c(
            CONCAT22(uVar4, local_12),
            CONCAT22(uVar7, puVar6),
            CONCAT22(uVar9, puVar8),
        );
        uVar1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
        iVar3 = CONCAT31(extraout_var, uVar1);
        _local_16 = CONCAT31(extraout_var, uVar1) & 0xffff | uVar4 << 0x10;
        local_1c = (iVar3 + 0xc);
        uStack24 = (iVar3 + 0x10);
        pass1_1028_c8ee(param_1, param_2, 1, CONCAT22(unaff_SS, &local_1c));
        pass1_1008_3eb4(
            CONCAT22(unaff_SS, &local_1c),
            CONCAT22(unaff_SS, &local_22),
            CONCAT22(unaff_SS, local_20),
            CONCAT22(unaff_SS, local_1e),
        );
        if (local_e < local_22) {
            pass1_1030_5b3e(CONCAT22(local_10, local_12), local_22, local_c);
            pass1_1030_5b1c(
                CONCAT22(local_10, local_12),
                CONCAT22(unaff_SS, &local_e),
                CONCAT22(unaff_SS, &local_c),
            );
        }
        local_26 = (_local_16 + 0x2e);
        local_2a = (local_26 + 4);
        pass1_1028_87f0(
            CONCAT22(unaff_SS, &local_14e),
            0,
            0,
            0x62,
            &local_1c,
            unaff_SS,
            local_2a,
            local_a,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_14e));
        pass1_1028_ccd0(CONCAT22(param_2, param_1), CONCAT22(unaff_SS, &local_1c));
    }
    return;
}

pub fn pass1_1028_4f30(param_1: *mut astruct_44, param_2: i32) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut uVar2: u16;

    uVar1 = pass1_1028_b58e(param_1);
    if (param_2 == 0) {
        uVar2 = 0;
    } else {
        uVar2 = 1000;
    }
    pass1_1030_7d1c(
        CONCAT22(in_DX, CONCAT11(extraout_AH, uVar1)),
        uVar2,
        0x230000,
    );
    return;
}

pub fn pass1_1028_4f62(param_1: u32) -> bool {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1);
    if ((CONCAT11(extraout_AH, uVar1) + 0x10) == 0) {
        return 1;
    }
    return 0;
}

pub fn pass1_1028_4faa(param_1: u32) {
    let uVar1: u8;
    let BVar2: bool;
    let extraout_AH: u8;
    let paVar3: *mut astruct_493;
    let mut in_DX: u16;
    let mut uVar4: i32;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    BVar2 = pass1_1028_4f62(param_1);
    if (BVar2 != 0) {
        return param_1;
    }
    uVar1 = pass1_1028_b58e(param_1);
    local_6 = CONCAT11(extraout_AH, uVar1);
    local_c = (local_6 + 0xc);
    local_8 = 0;
    uVar5 = pass1_1028_bb24(param_1);
    uVar6 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, &local_c), uVar5);
    uVar4 = (uVar6 >> 0x10);
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6, uVar4);
    if ((uVar4 | paVar3) == 0) {
        return 0;
    }
    uVar5 = pass1_1030_73a8(CONCAT22(uVar4, paVar3));
    return uVar5;
}

pub fn pass1_1028_504a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_50d8(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x5280;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_50fa(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = "\x02";
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5128(param_1: u16, param_2: u16, param_3: u16) {
    let uVar1: u8;
    let mut iVar2: i32;
    let extraout_var: u32;
    let mut uVar3: i32;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let puVar4: *mut u16;
    let mut uVar5: u16;
    let puVar6: *mut u16;
    let mut uVar7: u16;
    let mut local_152: u16;
    let mut local_14e: u16;
    let mut local_14c: u16;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: [u8; 2];
    let mut local_1e: [u8; 2];
    let mut local_1c: u32;
    let mut uStack24: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_bd38(CONCAT22(param_2, param_1));
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
    uVar3 = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    puVar6 = &local_c;
    puVar4 = &local_e;
    uVar5 = unaff_SS;
    uVar7 = unaff_SS;
    local_12 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    local_10 = uVar3;
    pass1_1030_5b1c(
        CONCAT22(uVar3, local_12),
        CONCAT22(uVar5, puVar4),
        CONCAT22(uVar7, puVar6),
    );
    uVar1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    iVar2 = CONCAT31(extraout_var, uVar1);
    _local_16 = CONCAT31(extraout_var, uVar1) & 0xffff | uVar3 << 0x10;
    local_1c = (iVar2 + 0xc);
    uStack24 = (iVar2 + 0x10);
    pass1_1028_c8ee(param_1, param_2, 1, CONCAT22(unaff_SS, &local_1c));
    pass1_1008_3eb4(
        CONCAT22(unaff_SS, &local_1c),
        CONCAT22(unaff_SS, &local_22),
        CONCAT22(unaff_SS, local_20),
        CONCAT22(unaff_SS, local_1e),
    );
    if (local_e < local_22) {
        pass1_1030_5b3e(CONCAT22(local_10, local_12), local_22, local_c);
        pass1_1030_5b1c(
            CONCAT22(local_10, local_12),
            CONCAT22(unaff_SS, &local_e),
            CONCAT22(unaff_SS, &local_c),
        );
    }
    local_26 = (_local_16 + 0x2e);
    local_2a = (local_26 + 4);
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_14e),
        0,
        0,
        0x6f,
        &local_1c,
        unaff_SS,
        local_2a,
        local_a,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_14e));
    pass1_1028_ccd0(CONCAT22(param_2, param_1), CONCAT22(unaff_SS, &local_1c));
    return;
}
