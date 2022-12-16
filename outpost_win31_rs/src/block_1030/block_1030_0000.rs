pub unsafe fn switch_1030_0000(
    mut param_1: u16,
    param_2: *mut Struct57,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: i16,
) -> *mut u16 {
    let mut uVar1: u16;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut paVar5: *mut astruct_180;
    let mut puVar6: *mut u16;

    match param_5 -1 {
        0x0 | 0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0x8 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_489e(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x9 => {
            mem_op_1000_179c(0x22, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_2bdc(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0xa => {
            mem_op_1000_179c(0x26, param_2);
            uVar1 = param_2;
            uVar3 = uVar1 | param_1;
        }
        // TODO: goto joined_r0x103002a1;
        0xb => {
            mem_op_1000_179c(0x2c, param_2);
            puVar2 = (param_2 | param_1);
            if (puVar2.is_null() == false) {
                puVar6 = struct_1028_3670(puVar2, CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0xc => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_355e(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0xd => {
            mem_op_1000_179c(0x26, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_3484(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0xe => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_406c(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0xf | 0x32 | 0x33 | 0x5f | 0x60 => {
            mem_op_1000_179c(0x24, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_0c24(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x10 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_0b42(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x11 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_4354(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x12 | 0x13 | 0x14 | 0x61 | 0x62 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_4b84(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x15 | 0x16 | 0x17 => {
            mem_op_1000_179c(0x24, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_1bbc(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        _ => {
            mem_op_1000_179c(0x20, param_2);
            uVar3 = param_2 | param_1;
            if (uVar3 != 0) {
                struct_1028_b354(CONCAT22(param_2, param_1));
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x1a | 0x1b | 0x1c => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                paVar5 = set_fn_ptr_1030_be34(CONCAT22(param_2, param_1));
                uVar3 = (paVar5 >> 0x10);
                param_1 = paVar5;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x1d | 0x1e | 0x1f => {
            mem_op_1000_179c(0x26, param_2);
            uVar3 = param_2 | param_1;
            paVar4 = (param_2 & 0xffff0000 | uVar3);
            if (uVar3 != 0) {
                struct_1028_0068(paVar4, CONCAT22(param_2, param_1));
                uVar3 = paVar4;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x20 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_50d8(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x21 | 0x22 | 0x23 => {
            mem_op_1000_179c(0x24, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_3e94(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x24 | 0x25 | 0x26 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_d06c(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x27 | 0x28 | 0x5c | 0x5d | 0x5e => {
            mem_op_1000_179c(0x22, param_2);
            if ((param_2 | param_1) != 0) {
                paVar5 = struct_1030_c6f6(CONCAT22(param_2, param_1));
                uVar3 = (paVar5 >> 0x10);
                param_1 = paVar5;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x29 | 0x2a => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_cce4(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x2b => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_26b4(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x2c | 0x2d => {
            mem_op_1000_179c(0x2a, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_49aa(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x2e | 0x2f => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_e7fa(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x30 | 0x31 | 0x6b | 0x6c => {
            mem_op_1000_179c(0x22, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_d37c(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x34 | 0x35 => {
            mem_op_1000_179c(0x2c, param_2);
            uVar3 = param_2 | param_1;
            paVar4 = (param_2 & 0xffff0000 | uVar3);
            if (uVar3 != 0) {
                struct_1028_37a6(paVar4, CONCAT22(param_2, param_1));
                uVar3 = paVar4;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x36 => {
            mem_op_1000_179c(0x26, param_2);
            uVar1 = param_2;
            uVar3 = uVar1 | param_1;
            // joined_r0x103002a1:
            if (uVar3 != 0) {
                struct_1030_c06e(CONCAT22(uVar1, param_1));
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x37 | 0x38 => {
            mem_op_1000_179c(0x9a, param_2);
            if ((param_2 | param_1) != 0) {
                paVar5 = struct_1030_c9a8(CONCAT22(param_2, param_1));
                uVar3 = (paVar5 >> 0x10);
                param_1 = paVar5;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x39 | 0x3a => {
            mem_op_1000_179c(0x24, param_2);
            uVar3 = param_2 | param_1;
            if (uVar3 != 0) {
                puVar6 = struct_1028_60bc(
                    CONCAT22(param_2, param_1),
                    param_2 & 0xffff0000 | uVar3,
                    param_1,
                );
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x3b | 0x3c => {
            mem_op_1000_179c(0x24, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_44d2(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x3d => {
            mem_op_1000_179c(0x22, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_cde6(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x3e => {
            mem_op_1000_179c(0x26, param_2);
            uVar3 = param_2 | param_1;
            paVar4 = (param_2 & 0xffff0000 | uVar3);
            if (uVar3 != 0) {
                struct_1028_1f56(paVar4, CONCAT22(param_2, param_1));
                uVar3 = paVar4;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x3f => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_25da(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x40 => {
            mem_op_1000_179c(0x22, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_c9ea(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x46 | 0x69 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_d5a6(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x47 | 0x48 | 0x49 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_d866(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x4b | 0x4c | 0x4d => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                paVar5 = struct_1030_d8f6(CONCAT22(param_2, param_1));
                uVar3 = (paVar5 >> 0x10);
                param_1 = paVar5;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x4e | 0x4f | 0x50 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_5c54(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x51 | 0x52 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_5966(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x53 | 0x54 | 0x55 => {
            mem_op_1000_179c(0x22, param_2);
            if ((param_2 | param_1) != 0) {
                paVar5 = set_fn_ptr_1028_5ed8(CONCAT22(param_2, param_1));
                uVar3 = (paVar5 >> 0x10);
                param_1 = paVar5;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x56 | 0x57 | 0x58 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_53c6(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x59 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = pass1_1028_5884(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x5a | 0x5b => {
            mem_op_1000_179c(0x26, param_2);
            puVar2 = (param_2 | param_1);
            if (puVar2.is_null() == false) {
                puVar6 = pass1_1028_5524(puVar2, CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x63 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                paVar5 = set_fn_ptr_1028_5df6(CONCAT22(param_2, param_1));
                uVar3 = (paVar5 >> 0x10);
                param_1 = paVar5;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x64 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_5a48(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x65 | 0x66 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_52e8(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x67 | 0x68 => {
            mem_op_1000_179c(0x20, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_57a6(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x6d => {
            mem_op_1000_179c(0x20, param_2);
            uVar3 = param_2 | param_1;
            if (uVar3 != 0) {
                param_1 = struct_1028_5630(CONCAT22(param_2, param_1));
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x6f | 0x70 | 0x71 => {
            mem_op_1000_179c(0x20, param_2);
            uVar3 = param_2;
            param_2 = (param_2 & 0xffff0000);
            if ((uVar3 | param_1) == 0) {
                param_1 = 0;
            } else {
                puVar6 = struct_1020_d866(CONCAT22(uVar3, param_1));
                param_2 = (param_2 & 0xffff0000 | puVar6 >> 0x10);
                param_1 = puVar6;
            }
        }
        0x72 | 0x76 => {
            mem_op_1000_179c(0x26, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1020_e8f6(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x73 | 0x77 | 0x78 => {
            mem_op_1000_179c(0x2c, param_2);
            uVar3 = param_2 | param_1;
            if (uVar3 != 0) {
                struct_1020_d954(CONCAT22(param_2, param_1));
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x74 => {
            mem_op_1000_179c(0x24, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_178c(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x75 => {
            mem_op_1000_179c(0x24, param_2);
            if ((param_2 | param_1) != 0) {
                puVar6 = struct_1028_2afa(CONCAT22(param_2, param_1));
                uVar3 = (puVar6 >> 0x10);
                param_1 = puVar6;
                // TODO: goto LAB_1030_0058;
            }
        }
        // break;
        0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7e => {
            mem_op_1000_179c(0x20, param_2);
            uVar3 = param_2 | param_1;
            if (uVar3 != 0) {
                param_1 = struct_1028_27f0(CONCAT22(param_2, param_1));
                // TODO: goto LAB_1030_0058;
            }
        }
    };
    param_1 = 0;
    uVar3 = 0; //
               // LAB_1030_0058:
    return CONCAT22(uVar3, param_1);
}

pub unsafe fn switch_1030_07ac(
    param_1: *mut astruct_12,
    param_2: *mut astruct_12,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u32,
) -> *mut u16 {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut pSVar4: *mut StructD;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u32;
    let mut puVar8: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    match param_6 - 1 {
        0x0 | 0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0x8 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_48c0(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x9 => {
            mem_op_1000_179c(0x22, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_2bfe(uVar3, param_1, paVar5, param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0xa => {
            mem_op_1000_179c(0x26, paVar5);
            uVar3 = paVar5;
            pSVar4 = (uVar3 | param_1);
        }
        // TODO: goto joined_r0x10300adb;
        0xb => {
            mem_op_1000_179c(0x2c, paVar5);
            puVar1 = (paVar5 | param_1);
            if (puVar1.is_null() == false) {
                puVar8 = pass1_1028_3692(puVar1, param_1, paVar5, param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0xc => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_3580(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0xd => {
            mem_op_1000_179c(0x26, paVar5);
            puVar1 = (paVar5 | param_1);
            if (puVar1.is_null() == false) {
                puVar8 = pass1_1028_34a6(puVar1, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0xe => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_408e(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0xf | 0x32 | 0x33 | 0x5f | 0x60 => {
            mem_op_1000_179c(0x24, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_0c50(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x10 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_0b64(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x11 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_4376(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x12 | 0x13 | 0x14 | 0x61 | 0x62 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_4ba6(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x15 | 0x16 | 0x17 => {
            mem_op_1000_179c(0x24, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_1be8(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        _ => {
            mem_op_1000_179c(0x20, paVar5);
            pSVar4 = (paVar5 | param_1);
            if (pSVar4.is_null() == false) {
                pass1_1028_b39e(pSVar4, CONCAT22(paVar5, param_1), param_6, param_8);
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x1a | 0x1b | 0x1c => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1030_be56(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x1d | 0x1e | 0x1f => {
            mem_op_1000_179c(0x26, paVar5);
            pSVar4 = (paVar5 | param_1);
            if (pSVar4.is_null() == false) {
                pass1_1028_00cc(pSVar4, CONCAT22(paVar5, param_1), param_6, param_8);
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x20 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_50fa(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x21 | 0x22 | 0x23 => {
            mem_op_1000_179c(0x24, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                uVar7 = pass1_1028_3ec8(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (uVar7 >> 0x10);
                param_1 = uVar7;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x24 | 0x25 | 0x26 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_d08e(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x27 | 0x28 | 0x5c | 0x5d | 0x5e => {
            mem_op_1000_179c(0x22, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1030_c71e(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x29 | 0x2a => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_cd06(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x2b => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_26d6(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x2c | 0x2d => {
            mem_op_1000_179c(0x2a, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                uVar7 = pass1_1028_49de(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (uVar7 >> 0x10);
                param_1 = uVar7;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x2e | 0x2f => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_e81c(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x30 | 0x31 | 0x6b | 0x6c => {
            mem_op_1000_179c(0x22, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 =
                    pass1_1020_d3a4(uVar3, CONCAT22(paVar5, param_1), param_5, param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x34 | 0x35 => {
            mem_op_1000_179c(0x2c, paVar5);
            uVar3 = paVar5 | param_1;
            paVar6 = (paVar5 & 0xffff0000 | uVar3);
            if (uVar3 != 0) {
                pass1_1028_3816(paVar6, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = paVar6;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x36 => {
            mem_op_1000_179c(0x26, paVar5);
            uVar3 = paVar5;
            pSVar4 = (uVar3 | param_1);
            // joined_r0x10300adb:
            if (pSVar4.is_null() == false) {
                pass1_1030_c09c(pSVar4, CONCAT22(uVar3, param_1), param_6, param_8);
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x37 | 0x38 => {
            mem_op_1000_179c(0x9a, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                uVar7 = pass1_1030_c9e4(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (uVar7 >> 0x10);
                param_1 = uVar7;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x39 | 0x3a => {
            mem_op_1000_179c(0x24, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                uVar7 = pass1_1028_611e(
                    CONCAT22(param_1, uVar3),
                    CONCAT22(paVar5, param_1),
                    param_6,
                    param_8,
                );
                pSVar4 = (uVar7 >> 0x10);
                param_1 = uVar7;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x3b | 0x3c => {
            mem_op_1000_179c(0x24, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_44fe(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x3d => {
            mem_op_1000_179c(0x22, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_ce08(uVar3, param_1, paVar5, param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x3e => {
            mem_op_1000_179c(0x26, paVar5);
            pSVar4 = (paVar5 | param_1);
            if (pSVar4.is_null() == false) {
                pass1_1028_1fc8(pSVar4, CONCAT22(paVar5, param_1), param_6, param_8);
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x3f => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_25fc(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x40 => {
            mem_op_1000_179c(0x22, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_ca0c(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x46 | 0x69 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_d5c8(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x47 | 0x48 | 0x49 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_d888(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x4b | 0x4c | 0x4d => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                uVar7 = pass1_1030_d942(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (uVar7 >> 0x10);
                param_1 = uVar7;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x4e | 0x4f | 0x50 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_5c76(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x51 | 0x52 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_5988(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x53 | 0x54 | 0x55 => {
            mem_op_1000_179c(0x22, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_5f00(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x56 | 0x57 | 0x58 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_53e8(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x59 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_58a6(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x5a | 0x5b => {
            mem_op_1000_179c(0x26, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_5546(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x63 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_5e18(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x64 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_5a6a(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x65 | 0x66 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_530a(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x67 | 0x68 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_57c8(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x6d => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_5652(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x6f | 0x70 | 0x71 => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5;
            uVar2 = uVar3 | param_1;
            paVar5 = (paVar5 & 0xffff0000);
            if (uVar2 == 0) {
                param_1 = null_mut();
            } else {
                puVar8 = pass1_1020_d888(uVar2, CONCAT22(uVar3, param_1), param_6, param_8);
                paVar5 = (paVar5 & 0xffff0000 | puVar8 >> 0x10);
                param_1 = puVar8;
            }
        }
        0x72 | 0x76 => {
            mem_op_1000_179c(0x26, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1020_e91e(uVar3, param_1, paVar5, param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x73 | 0x77 | 0x78 => {
            mem_op_1000_179c(0x2c, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = struct_1020_d99e(
                    (paVar5 & 0xffff0000 | uVar3),
                    CONCAT22(paVar5, param_1),
                    param_5,
                    param_6,
                    param_8,
                );
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x74 => {
            mem_op_1000_179c(0x24, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_17ae(uVar3, param_1, paVar5, param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x75 => {
            mem_op_1000_179c(0x24, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_2b1c(uVar3, param_1, paVar5, param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
        // break;
        0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7e => {
            mem_op_1000_179c(0x20, paVar5);
            uVar3 = paVar5 | param_1;
            if (uVar3 != 0) {
                puVar8 = pass1_1028_2812(uVar3, CONCAT22(paVar5, param_1), param_6, param_8);
                pSVar4 = (puVar8 >> 0x10);
                param_1 = puVar8;
                // TODO: goto LAB_1030_0818;
            }
        }
    };
    param_1 = null_mut();
    pSVar4 = null_mut(); //
                         // LAB_1030_0818:
    return CONCAT22(pSVar4, param_1);
}
