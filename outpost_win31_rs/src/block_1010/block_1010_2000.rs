// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_2024(param_1: *mut StructD) -> *mut StructD {
    let mut struct_1_hi: u16;

    struct_1_hi = (param_1 >> 0x10);
    (param_1 + 0x124) = 0;
    _u16_1050_0ed0 = param_1;
    pass1_1000_4906((param_1 & 0xffff | struct_1_hi << 0x10), NULL, 0x124);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_2050(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;
    let mut iStack4: i16;

    pass1_1010_2816(param_1);
    iStack4 = 0;
    loop {
        uVar4 = (param_1 >> 0x10);
        puVar1 = (iStack4 * 0x4 + param_1);
        uVar2 = (iStack4 * 0x4 + param_1 + 2);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iStack4 += 0x1;
        if iStack4 >= 0x49 {
            break;
        }
    }
    _u16_1050_0ed0 = 0;
    return;
}
pub unsafe fn pass1_1010_209e(param_1: u32, mut param_2: u16) {
    pass1_1010_2816(param_1);
    (param_1 + 0x124) = param_2;
    return;
}

pub unsafe fn mixed_1010_20ba(
    param_1: *mut astruct_57,
    mut param_2: u32,
    param_3: u32,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) -> *mut u32 {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut paVar5: *mut astruct_638;
    let mut unaff_SI: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut puVar9: *mut u16;
    let mut uVar10: u32;
    let mut puVar11: *mut u16;
    let mut uVar12: u32;
    let mut paVar13: *mut astruct_19;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut puStack6: *mut u32;

    pass1_1010_2816(param_2);
    paVar5 = (param_3 * 0x4);
    uVar7 = (param_2 >> 0x10);
    iVar6 = param_2;
    puStack6 = (&paVar5.field_0x0 + iVar6);
    if (puStack6.is_null() == false) {
        return puStack6;
    }
    match param_3 {
        0x1 => {
            mem_op_1000_179c(0x18, param_1);
            puVar2 = (param_1 | paVar5);
            if (puVar2.is_null()) {
                //
                // LAB_1010_2126:
                paVar5 = null_mut();
                puVar2 = null_mut();
            } else {
                struct_1010_3b7a(CONCAT22(param_1, paVar5), param_3);
            }
        }
        // break;
        0x2 => {
            mem_op_1000_179c(0x84, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            win_sys_op_1010_5404(param_4, CONCAT22(param_1, paVar5), param_3);
        }
        // break;
        0x3 => {
            mem_op_1000_179c(0x53c, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_a1d8(CONCAT22(param_1, paVar5), param_3);
        }

        0x4 => {
            mem_op_1000_179c(0x18a, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1018_2b10(CONCAT22(param_1, paVar5), param_3);
        }

        0x5 => {
            mem_op_1000_179c(0x14, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            puVar11 = pass1_1008_eabc(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (puVar11 >> 0x10);
            paVar5 = puVar11;
        }

        0x6 => {
            mem_op_1000_179c(0x58, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_18b8(CONCAT22(param_1, paVar5), param_3);
        }

        0x7 => {
            mem_op_1000_179c(0xe, param_1);
            uVar4 = param_1 | paVar5;
            //     if (uVar4 == 0) goto LAB_1010_2126;
            uVar10 = pass1_1010_3d82(param_1 & 0xffff0000 | uVar4, paVar5, param_1, param_3);
            puVar2 = (uVar10 >> 0x10);
            paVar5 = uVar10;
        }

        0x8 => {
            mem_op_1000_179c(0x20, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_95aa(CONCAT22(param_1, paVar5), param_3);
        }

        0x9 => {
            mem_op_1000_179c(0x26, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_6326(CONCAT22(param_1, paVar5), param_3);
        }

        0xa => {
            mem_op_1000_179c(0x1c, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            paVar13 = pass1_1010_0eac(puVar2, CONCAT22(param_1, paVar5), param_3);
            puVar2 = (paVar13 >> 0x10);
            paVar5 = paVar13;
        }

        0xb => {
            mem_op_1000_179c(0x1c, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            uVar10 = pass1_1008_aefe(puVar2, CONCAT22(param_1, paVar5), param_3);
            puVar2 = (uVar10 >> 0x10);
            paVar5 = uVar10;
        }

        0xc | 0xd | 0xe | 0xf | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18
        | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 => {
            mem_op_1000_179c(0xaa, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1018_0570(CONCAT22(param_1, paVar5), param_3, param_5);
        }

        0x25 => {
            mem_op_1000_179c(0x1c, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_4aaa(puVar2, CONCAT22(param_1, paVar5), param_3);
        }

        0x26 => {
            mem_op_1000_179c(0x1c, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_d99e(puVar2, CONCAT22(param_1, paVar5), param_3);
        }

        0x27 => {
            mem_op_1000_179c(0x58, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_9d36(paVar5, param_1, param_3);
        }

        0x28 => {
            mem_op_1000_179c(0x2c, param_1);
            uVar4 = param_1 | paVar5;
            uVar10 = param_1 & 0xffff0000 | uVar4;
            //     if (uVar4 == 0) goto LAB_1010_2126;
            pass1_1010_28e6(uVar10, 0x1000, paVar5, param_1, param_3);
            puVar2 = uVar10;
        }

        0x29 => {
            mem_op_1000_179c(0x72, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1018_229c(puVar2, CONCAT22(param_1, paVar5), param_3);
        }

        0x2a => {
            mem_op_1000_179c(0x1c, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1010_503e(puVar2, CONCAT22(param_1, paVar5), param_3);
        }

        0x2b => {
            mem_op_1000_179c(0x1a, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_02e0(CONCAT22(param_1, paVar5), param_3);
        }

        0x2c => {
            mem_op_1000_179c(0x10, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_eb2a(CONCAT22(param_1, paVar5), param_3);
        }

        0x2d => {
            mem_op_1000_179c(0x80, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1010_3e3c(CONCAT22(param_1, paVar5), param_3, param_6);
        }

        0x2e => {
            mem_op_1000_179c(0x806, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            uVar10 = pass1_1018_1ff4(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (uVar10 >> 0x10);
            paVar5 = uVar10;
        }

        0x2f => {
            mem_op_1000_179c(0x58, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_e9e4(CONCAT22(param_1, paVar5), param_3);
        }

        0x30 => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            puVar9 = pass1_1010_3702(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (puVar9 >> 0x10);
            paVar5 = puVar9;
        }

        0x31 => {
            uVar8 = 0x1000;
            mem_op_1000_179c(0x60, param_1);
            uVar4 = param_1 | paVar5;
            if (uVar4 == 0) {
                //
                // LAB_1010_2680:
                uVar8 = 0x1000;
                paVar5 = null_mut();
                puVar2 = null_mut();
            } else {
                uVar10 =
                    pass1_1010_9298(CONCAT22(paVar5, uVar4), CONCAT22(param_1, paVar5), param_3);
                puVar2 = (uVar10 >> 0x10);
                paVar5 = uVar10;
            }
        }
        // TODO: goto LAB_1010_2683;
        0x32 => {
            mem_op_1000_179c(0x26, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1010_6abc(CONCAT22(param_1, paVar5), param_3, param_6);
        }

        0x33 => {
            mem_op_1000_179c(0x72, param_1);
            uVar4 = param_1 | paVar5;
            if (uVar4 == 0) {
                //
                // LAB_1010_25b8:
                uVar8 = 0;
                uVar3 = 0;
            } else {
                paVar13 = pass1_1010_195e(param_1 & 0xffff0000 | uVar4, paVar5, param_1, param_3);
                uVar3 = (paVar13 >> 0x10);
                uVar8 = SUB42(paVar13, 0x0);
            }
        }
        // TODO: goto LAB_1010_25bb;
        0x34 => {
            mem_op_1000_179c(0x72, param_1);
            uVar4 = param_1 | paVar5;
            //     if (uVar4 == 0) goto LAB_1010_25b8;
            paVar13 = pass1_1010_1b6e((param_1 & 0xffff0000 | uVar4), paVar5, param_1, param_3);
            uVar3 = (paVar13 >> 0x10);
            uVar8 = SUB42(paVar13, 0x0); //
                                         // LAB_1010_25bb:
            puStack6 = CONCAT22(uVar3, uVar8);
            pass1_1010_1146(CONCAT22(uVar3, uVar8), 0x0);
        }
        // TODO: goto switchD_1010_2765_caseD_38;
        0x35 => {
            mem_op_1000_179c(0x14a, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            paVar13 = pass1_1010_6700(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (paVar13 >> 0x10);
            paVar5 = paVar13;
        }

        0x36 => {
            mem_op_1000_179c(0x10, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_d790(paVar5, param_1, param_3);
        }

        0x37 => {
            mem_op_1000_179c(0x420, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1008_9fd2(CONCAT22(param_1, paVar5), param_3);
        }

        _ => {}
        // TODO: goto switchD_1010_2765_caseD_38;
        0x39 => {
            mem_op_1000_179c(0x36, param_1);
            uVar4 = param_1 | paVar5;
            uVar10 = param_1 & 0xffff0000 | uVar4;
            //     if (uVar4 == 0) goto LAB_1010_2126;
            pass1_1010_4a8a(
                uVar10,
                paVar5,
                param_1,
                param_3,
                param_7,
                in_stack_0000fe8a,
                in_stack_0000ffae,
                in_stack_0000ffb4,
                in_stack_0000ffb8,
            );
            puVar2 = uVar10;
        }

        0x3a => {
            mem_op_1000_179c(0xc, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            puVar9 = pass1_1008_d72e(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (puVar9 >> 0x10);
            paVar5 = puVar9;
        }

        0x3b => {
            mem_op_1000_179c(0x22, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1008_dd4e(CONCAT22(param_1, paVar5), param_3);
        }

        0x3c => {
            mem_op_1000_179c(0x146, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_331c(puVar2, paVar5, param_1, param_3);
        }

        0x3d => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            uVar10 = pass1_1010_8c32(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (uVar10 >> 0x10);
            paVar5 = uVar10;
        }
        0x3e => {
            mem_op_1000_179c(0x18, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1018_5070(CONCAT22(param_1, paVar5), param_3);
        }

        0x3f => {
            mem_op_1000_179c(0x12, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_c72a(CONCAT22(param_1, paVar5), param_3, unaff_SI);
        }

        0x40 => {
            mem_op_1000_179c(0x24, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            pass1_1008_af94(CONCAT22(param_1, paVar5), param_3, unaff_SI);
        }
        0x41 => {
            mem_op_1000_179c(0x60, param_1);
            uVar4 = param_1 | paVar5;
            //     if (uVar4 == 0) goto LAB_1010_2680;
            uVar8 = 0x1008;
            uVar12 = struct_1008_ecb2(paVar5, uVar4, CONCAT22(param_1, paVar5), param_3);
            puVar2 = (uVar12 >> 0x10);
            paVar5 = uVar12; //
                             // LAB_1010_2683:
            (param_3 * 0x4 + iVar6) = paVar5;
            (param_3 * 0x4 + iVar6 + 0x2) = puVar2;
            ppcVar1 = (paVar5 + 0x10);
            (**ppcVar1)(uVar8, paVar5, puVar2);
        }

        0x42 => {
            mem_op_1000_179c(0xc, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            puVar9 = pass1_1008_ec10(paVar5, param_1, param_3);
            puVar2 = (puVar9 >> 0x10);
            paVar5 = puVar9;
        }

        0x43 => {
            mem_op_1000_179c(0x12, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            puVar9 = pass1_1010_2bfc(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (puVar9 >> 0x10);
            paVar5 = puVar9;
        }
        0x45 => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            paVar13 = pass1_1010_0000(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (paVar13 >> 0x10);
            paVar5 = paVar13;
        }
        0x46 => {
            mem_op_1000_179c(0x1a, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            struct_1010_50b2(CONCAT22(param_1, paVar5), param_3);
        }
        0x47 => {
            mem_op_1000_179c(0xe, param_1);
            //     if ((param_1 | paVar5) == 0) goto LAB_1010_2126;
            puVar9 = pass1_1018_56e6(CONCAT22(param_1, paVar5), param_3);
            puVar2 = (puVar9 >> 0x10);
            paVar5 = puVar9;
        }

        0x48 => {
            mem_op_1000_179c(0x1c, param_1);
            puVar2 = (param_1 | paVar5);
            //     if (puVar2.is_null()) goto LAB_1010_2126;
            unk_draw_op_1008_da12(CONCAT22(param_1, paVar5), param_3);
        }
    };
    puStack6 = CONCAT22(puVar2, paVar5);
    // switchD_1010_2765_caseD_38:
    (param_3 * 0x4 + iVar6) = puStack6;
    return puStack6;
}
pub unsafe fn pass1_1010_2816(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x124) != 0) {
        iVar5 = (iVar4 + 0x124) * 0x4;
        puVar1 = (iVar5 + iVar4);
        uVar2 = (iVar5 + iVar4 + 2);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        ((iVar4 + 0x124) * 0x4 + iVar4) = 0;
        (iVar4 + 0x124) = 0;
    }
    return;
}

pub unsafe fn pass1_1010_286c() -> u16 {
    let mut puVar1: *mut u16;

    pass1_1008_3e54(&PTR_LOOP_1048_0000, 0x0, 0x5, 0x12c);
    pass1_1008_3e54((s__1050_65a0 + 0x6), 0x0, 0x9b, 0x20);
    pass1_1008_3e54(0x105065ac, 0x0, 0xf5, 0x3f);
    pass1_1008_3e54(0x105065b2, 0x0, 0x114, 0x4a);
    pass1_1008_3e54(0x105065b8, 0x0, 0x135, 0x45);
    pass1_1008_3e54(0x105065be, 0x0, 0xf5, 0x7b);
    puVar1 = pass1_1008_3e54(0x105065c4, 0x0, 0x117, 0x91);
    return puVar1;
}
pub unsafe fn pass1_1010_28e6(
    mut param_1: u32,
    mut param_2: u16,
    param_3: *mut astruct_19,
    param_4: *mut astruct_19,
    mut param_5: u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut puVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut unaff_DS: u16;
    let mut iStack6: i16;

    struct_op_1018_4cda(CONCAT22(param_4, param_3), param_5);
    uVar5 = 0;
    param_3.field15_0x1c = 0;
    param_3.field17_0x20 = 0;
    param_3.field18_0x22 = 0;
    param_3.field19_0x24 = 0;
    param_3.field20_0x26 = 0;
    // 0x2bdc
    CONCAT22(param_4, param_3) = 0x2be4;
    param_3.segment_0x2 = 0x1010;
    PTR_LOOP_1050_4230 = param_3;
    0x4232 = param_4;
    pass1_1018_4dce(param_1, CONCAT22(param_4, param_3), 0x56);
    uVar2 = FUN_1010_830a(uVar5, param_1, 0x1018, &u32_1048_14cc, 0x4);
    param_3.field15_0x1c = uVar2;
    param_3.field16_0x1e = param_1;
    if (0x5f2c == 0) {
        puVar2 = mem_op_1000_160a(param_1);
    } else {
        puVar2 = 0x5f2c;
        param_1 = param_1 & 0xffff0000 | 0x5f2e;
    }
    uVar4 = fn_ptr_op_1000_1708(0x40, 0x0, 0x1, puVar2, param_1);
    (param_3.field20_0x26 + 0x2) = uVar4;
    param_3.field21_0x2a = param_1;
    //   for (iStack6 = 0; iStack6 < 0x10; iStack6 += 1)
    for iStack6 in 0..0x10 {
        uVar5 = FUN_1010_830a(
            iStack6 + 0x56,
            param_1,
            0x1000,
            &u32_1048_14cc,
            iStack6 + 0x56,
        );
        uVar1 = (param_3.field20_0x26 + 2);
        uVar7 = (uVar1 >> 0x10);
        iVar6 = uVar1;
        (iVar6 + iStack6 * 0x4) = uVar5;
        (iVar6 + iStack6 * 0x4 + 0x2) = param_1;
    }
    return;
}
pub unsafe fn pass1_1010_29c6(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar5: *mut StructD;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.address_offset_field_0x0 = s_add16_wav_1050_2bdc + 0x8;
    iVar5.address_offset_field_0x2 = 0x1010;
    if (&iVar5.field_0x1c != 0) {
        puVar1 = &iVar5.field_0x1c;
        uVar2 = &iVar5.field_0x1e;
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iVar5.field_0x1c = 0;
        fn_ptr_1000_17ce(*&iVar5.field_0x28);
        iVar5.field_0x28 = 0;
    }
    clenaup_win_ui_1018_4d22(param_1);
    return;
}

// WARNING: Instruction at (ram,0x10104b2b) overlaps instruction at (ram,0x10104b2a)
//
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

pub unsafe fn FUN_1010_2a32(
    buffer_param_2: *mut u8,
    mut param_2: u32,
    hfile_param: *mut HFILE16,
    mut param_4: u16,
) -> u16 {
    let mut piVar1: *mut i16;
    let mut pcVar2: *mut c_char;
    let mut pbVar3: *mut u8;
    let mut bVar4: u8;
    let mut puVar5: *mut u32;
    let mut puVar6: *mut u32;
    let mut ppcVar7: *mut *mut code;
    let mut pcVar8: *mut code;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut HVar11: HDC16;
    let mut HVar12: HPALETTE16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut BVar15: bool;
    let mut iVar16: i16;
    let mut bVar17: u8;
    let mut puVar18: *mut u8;
    let mut puVar19: *mut u8;
    let mut uVar20: u16;
    let mut uVar21: u16;
    let mut in_EDX: u32;
    let mut in_BX: *mut u32;
    let mut unaff_BP: u16;
    let mut unaff_SI: i16;
    let mut iVar23: i16;
    let mut iVar24: i16;
    let mut unaff_ES: u16;
    let mut uVar25: u16;
    let mut bVar26: u8;
    let mut bVar27: bool;
    let mut init_data: *mut DEVMODEA;
    let mut in_stack_00000000: i16;
    let mut in_stack_00000002: u16;
    let mut uStack54: u16;
    let mut puStack46: *mut u16;
    let mut uStack42: u16;
    let mut read_buffer_38: *mut u8;
    let mut read_buffer_22: *mut u8;
    let mut HStack30: HGDIOBJ16;
    let mut HStack28: HGDIOBJ16;
    let mut in_stack_0000ffec: u8;
    let mut uVar28: u8;
    let mut in_stack_0000ffed: u8;
    let mut uVar29: u8;
    let mut uVar30: u8;
    let mut uVar31: u8;
    let mut uVar32: u8;
    let mut uVar33: u8;
    let mut uVar34: u8;
    let mut uVar35: u8;
    let mut uVar36: u8;
    let mut uVar37: u8;
    let mut uVar38: u8;
    let mut paVar22: *mut Struct57;

    uVar28 = unaff_BP;
    uVar29 = (unaff_BP >> 0x8);
    bVar26 = 0;
    uVar38 = 0;
    //   if ((param_2 + 0xec76 & 0x3) != 0) goto LAB_1010_2ad8;
    uVar10 = param_2 + 0xec76 >> 0x1;
    //   if (0x1c < uVar10) goto LAB_1010_2ad8;
    uVar14 = in_EDX;
    //   switch(uVar10)
    match uVar10 {
        //   _ =>
        _ => {}
        // TODO: goto switchD_1010_2ab5_caseD_0;
        0x1 | 0x3 | 0xb => {
            (uVar10 + 0xa) = in_BX;
        }
        0x9 | 0xf | 0x15 | 0x1b => {
            (uVar10 + 0xa) = in_BX;
            (uVar10 + 0x10) = in_BX;
            (uVar10 + 0xc) = in_BX;
            return uVar10;
        }
        0x5 => {
            BVar15 = write_to_file_1008_7e1c(
                param_2,
                ZEXT24(in_BX),
                CONCAT13(
                    (in_stack_00000000 >> 0x8),
                    CONCAT12(in_stack_00000000, unaff_BP),
                ),
                CONCAT11(in_stack_0000ffed, in_stack_0000ffec),
            );
            if (BVar15 != 0) {
                return 0x1;
            }
            u16_1050_0310 = 0x6d0;
            return 0x0;
        }
        0x6 => {
            bVar26 = 0;
        }
        // TODO: goto LAB_1010_2ad8;
        0x7 => {
            ppcVar7 = *in_BX;
            (**ppcVar7)();
            iVar16 = param_2 + 0x105;
            puVar18 = in_EDX;
            pass1_1010_8170(puVar18, _u16_1050_14cc, iVar16);
            iVar23 = param_2 * 0x4;
            (buffer_param_2 + iVar23 + 0x26) = iVar16;
            (buffer_param_2 + iVar23 + 0x28) = puVar18;
            uVar36 = 0x50;
            uVar37 = 0x10;
            uVar34 = 0x80;
            uVar35 = 0x13;
            uVar30 = 0;
            uVar31 = 0;
            uVar32 = 0;
            uVar33 = 0;
            uVar28 = 0;
            uVar29 = 0;
            init_data = pass1_1008_4772((buffer_param_2 + 0x26 + iVar23));
            uVar25 = (init_data >> 0x10);
            HVar11 = CreateDC16(
                init_data,
                CONCAT13(uVar29, CONCAT12(uVar28, uVar25)),
                CONCAT13(uVar33, CONCAT12(uVar32, CONCAT11(uVar31, uVar30))),
                CONCAT13(uVar37, CONCAT12(uVar36, CONCAT11(uVar35, uVar34))),
            );
            uVar28 = HVar11;
            uVar29 = (HVar11 >> 0x8);
            HVar12 = palette_op_1008_4e08(
                &stack0xffec,
                uVar25,
                (_PTR_LOOP_1050_4230 + 0xe),
                CONCAT13(0x10, CONCAT12(0x50, &stack0xffec)),
            );
            HStack28 = SelectObject16(CONCAT11(uVar38, bVar26), CONCAT11(uVar29, uVar28));
            HStack30 = SelectObject16(CONCAT11(uVar29, uVar28), CONCAT11(uVar29, uVar28));
            read_buffer_22 = 0;
            piVar1 = buffer_param_22 + 0x74;
            // for (read_buffer_22 = 0; piVar1 = (buffer_param_2 + 0x74),
            // *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1; read_buffer_22 += 1)
            while *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1 {
                iVar16 = (read_buffer_22 * 0x10 + param_2) * 0x8;
                uVar25 = (buffer_param_2 + 0x72);
                uVar13 = pass1_1000_484c(
                    CONCAT13(0x10, CONCAT12(0x50, &stack0xfff2)),
                    CONCAT13(
                        (uVar25 >> 0x8),
                        CONCAT12(uVar25, iVar16 + (buffer_param_2 + 0x70)),
                    ),
                    0x8,
                );
                if (uVar13 != 0) {
                    uVar9 = (buffer_param_2 + 0x70);
                    uVar25 = (uVar9 >> 0x10);
                    iVar23 = uVar9;
                    iVar24 = iVar16 + iVar23;
                    Rectangle16(
                        (iVar24 + 0x6),
                        (iVar24 + 0x4),
                        (iVar24 + 0x2),
                        (iVar23 + iVar16),
                        CONCAT11(uVar29, uVar28),
                    );
                }
                read_buffer_22 += 1;
            }
            HVar12 = SelectPalette16(0x0, HVar12, CONCAT11(uVar29, uVar28));
            DeleteObject16(HVar12);
            SelectObject16(HStack28, CONCAT11(uVar29, uVar28));
            SelectObject16(HStack30, CONCAT11(uVar29, uVar28));
            uVar38 = 0x38;
            uVar30 = 0x15;
            DeleteDC16(CONCAT11(uVar29, uVar28));
            BVar15 = DeleteObject16(CONCAT11(uVar30, uVar38));
            return BVar15;
        }
        0x8 => {
            bVar26 = 0x3;
        }
        // TODO: goto LAB_1010_2ad8;
        0xd => {
            pbVar3 = (uVar10 + unaff_SI);
            bVar26 = *pbVar3;
            bVar4 = *pbVar3 + in_EDX;
            *pbVar3 = bVar4 + (uVar10 < 0x1c);
            puVar5 = (CARRY1(bVar26, in_EDX) || CARRY1(bVar4, uVar10 < 0x1c));
            puVar6 = in_BX -0x404;
            bVar26 = in_BX < 0x1010 || puVar6 < puVar5;
            uVar21 = puVar6 - puVar5;
            pcVar8 = swi(0x4);
            if (SBORROW2(in_BX, 0x1010) != SBORROW2(puVar6, puVar5)) {
                (*pcVar8)();
            }
            puVar19 = in_EDX;
            bVar27 = uVar21 < 0x1010 || uVar21 + 0xeff0 < bVar26;
            pbVar3 = (uVar10 + unaff_SI);
            bVar26 = *pbVar3;
            bVar17 = in_EDX;
            bVar4 = *pbVar3;
            *pbVar3 = bVar4 + bVar17 + bVar27;
            pcVar2 = (uVar10 + unaff_SI);
            *pcVar2 = *pcVar2 + bVar17 + (CARRY1(bVar26, bVar17) || CARRY1(bVar4 + bVar17, bVar27));
            struct_op_1018_4cda(
                CONCAT13(
                    in_stack_00000000,
                    CONCAT12(uVar29, CONCAT11(uVar28, uVar38)),
                ),
                CONCAT11(in_stack_00000002, (in_stack_00000000 >> 0x8)),
            );
            iVar16 = CONCAT11(uVar28, uVar38);
            uVar30 = in_stack_00000000;
            piVar1 = CONCAT13(uVar30, CONCAT12(uVar29, iVar16));
            *piVar1 = s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 1;
            (iVar16 + 0x2) = 0x1010;
            pass1_1018_4dce(puVar19, CONCAT13(uVar30, CONCAT12(uVar29, iVar16)), 0x1b3);
            _PTR_LOOP_1050_4230 = CONCAT13(uVar30, CONCAT12(uVar29, CONCAT11(uVar28, uVar38)));
            return CONCAT11(uVar28, uVar38);
        }
        0xe => {
            (buffer_param_2 + 0x20) = param_2;
        }

        0x11 => {
            // loop {
            // // WARNING: Do nothing block with infinite loop
            // }
        }
        0x12 => {
            PTR_LOOP_1050_10c6 = (0x0 < param_2);
            PTR_LOOP_1050_1142 = (0x2 < param_2);
        }
        0x13 => {
            iVar16 = buffer_param_2 * 0x8 + in_stack_00000000;
            if ((((iVar16 + 0x22) != 0) || ((iVar16 + 0x24) != 0))
                || ((iVar16 + 0x26) != 0x0 || ((iVar16 + 0x28) != 0)))
            {
                HStack28 = 0x1010;
                HStack30 = 0x627c;
                sys_1000_3f9c(
                    *(in_stack_00000000 + 0xe),
                    s__d__d__d__d_1050_14ae,
                    (buffer_param_2 * 0x8 + in_stack_00000000 + 0x22),
                );
                HStack28 = 0x1000;
                HStack30 = 0x62a0;
                in_BX = WritePrivateProfileString16(
                    *(in_stack_00000000 + 0xa),
                    *(in_stack_00000000 + 0xe),
                    *(buffer_param_2 * 0x4 + 0x1446),
                    s_windows_1050_13b8,
                );
            }
            return in_BX;
        }
        0x14 => {
            (buffer_param_2 + 0x24) = param_2;
        }

        0x17 => {
            uVar21 = uVar14 - 0x1;
            paVar22 = (in_EDX & 0xffff0000 | uVar21);
            pbVar3 = (uVar10 + unaff_SI);
            *pbVar3 = *pbVar3 | uVar21;
            (buffer_param_2 + 0x12) = in_BX;
            (buffer_param_2 + 0x14) = uVar21;
            uStack42 = 0;
            loop {
                if (uStack54 <= uStack42) {
                    uVar38 = (buffer_param_2 >> 0x10);
                    BVar15 = read_file_1008_7dee(
                        param_2,
                        ((buffer_param_2 & 0xff00) << 0x10
                            | CONCAT12(uVar38, buffer_param_2 + 0x1a)),
                        0x2,
                    );
                    if (((BVar15 != 0)
                        && (
                            BVar15 = read_file_1008_7dee(
                                param_2,
                                ((buffer_param_2 & 0xff00) << 0x10
                                    | CONCAT12(uVar38, buffer_param_2 + 0x1c)),
                                0x2,
                            ),
                            BVar15 != 0,
                        ))
                        && (
                            BVar15 = read_file_1008_7dee(
                                param_2,
                                ((buffer_param_2 & 0xff00) << 0x10
                                    | CONCAT12(uVar38, buffer_param_2 + 0x1e)),
                                0x2,
                            ),
                            BVar15 != 0,
                        ))
                    {
                        return 0x1;
                    }
                    u16_1050_0310 = 0x6d2;
                    return 0x0;
                }
                uVar10 = uStack54;
                mem_op_1000_179c(0x8, paVar22);
                uVar21 = paVar22;
                puStack46 = CONCAT22(uVar21, uVar10);
                paVar22 = (paVar22 & 0xffff0000 | (uVar21 | uVar10));
                if ((uVar21 | uVar10) == 0) {
                    read_buffer_38 = null_mut();
                } else {
                    *puStack46 = 0x389a;
                    (uVar10 + 0x2) = 0x1008;
                    *puStack46 = 0xa1c4;
                    (uVar10 + 0x2) = 0x1010;
                    read_buffer_38 = puStack46;
                }
                BVar15 = read_file_1008_7dee(
                    param_2,
                    CONCAT13(0x10, CONCAT12(0x50, &read_buffer_22)),
                    0x2,
                );
                if ((BVar15 == 0)
                    || (
                        BVar15 = read_file_1008_7dee(
                            param_2,
                            (read_buffer_38 & 0xff000000
                                | CONCAT12((read_buffer_38 >> 0x10), read_buffer_38 + 0x6)),
                            0x2,
                        ),
                        BVar15 == 0,
                    ))
                {}
                iVar16 = switch_1008_73ea(param_2, param_2, read_buffer_22);
                (read_buffer_38 + 0x4) = iVar16;
                ppcVar7 = ((buffer_param_2 + 0x12) + 0x4);
                (**ppcVar7)();
                uStack42 += 0x1;
            }
            if (read_buffer_38.is_null()) {
                u16_1050_0310 = 0x6d2;
                return 0x0;
            }
            ppcVar7 = read_buffer_38;
            (**ppcVar7)();
            u16_1050_0310 = 0x6d2;
            return 0x0;
        }
        0x18 => {
            bVar26 = 0x2;
        }
        // TODO: goto LAB_1010_2ad8;
        0x19 => {
            uVar14 = pass1_1010_6ca2(uVar14, buffer_param_2, 0x8);
            uVar20 = in_EDX;
            if (uVar14 != 0) {
                pass1_1010_715c(uVar14, uVar20, CONCAT22(0x1a, buffer_param_2), 0x1a);
                buffer_param_2 = 0x1a001a;
            }
            if (param_2 == 0x2c) {
                pass1_1010_715c(uVar14, uVar20, CONCAT22(0x1d, buffer_param_2), 0x1d);
            }
            uVar14 = pass1_1010_6ca2(uVar20, 0x5a, 0x2);
            if (uVar14 != 0) {
                pass1_1010_715c(uVar14, uVar20, 0x1c005a, 0x1c);
            }
            return 0x1;
        }
        0x1a => {
            (buffer_param_2 + 0x26) = param_2;
        }
    };
    bVar26 = 0x1; //
                  // LAB_1010_2ad8:
    if ((bVar26 == 1) || (bVar26 == 0x2)) {
        if (bVar26 == 1) {
            param_2 = ((buffer_param_2 + 0x20)
                + (buffer_param_2 + 0x22)
                + (buffer_param_2 + 0x24)
                + (buffer_param_2 + 0x26));
        }
        if (param_2.is_null() == false) {
            uVar10 = param_2 / 0x2 + 1;
            if (0x5 < uVar10) {
                uVar10 = 0x5;
            }
            param_2 = uVar10;
            if ((bVar26 == 1) && (PTR_LOOP_1050_10c6.is_null() == false)) {
                if (0x4 < uVar10) {
                    uVar10 = 0x4;
                }
                param_2 = uVar10;
            }
        }
    }
    (bVar26 * 0x7c + 0xed6) = param_2;
    in_BX = param_2;
    pass1_1010_1f62(buffer_param_2, 0xc);
    // switchD_1010_2ab5_caseD_0:
    return in_BX;
}
pub unsafe fn pass1_1010_2b50(mut param_1: u16, mut param_2: u16, param_3: *mut u16) {
    pass1_1008_3f62(param_3, &PTR_LOOP_1048_0000);
    return;
}

pub unsafe fn pass1_1010_2b66(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x1e), (param_1 + 0x1c));
}
pub unsafe fn pass1_1010_2b78(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u32,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;

    puVar3 = (param_3 * 0x7c + 0xed4);
    puVar5 = param_4;
    //   for (iVar4 = 0x1f; iVar4 != 0; iVar4 += -1)
    for iVar4 in 0x1f..0 {
        puVar2 = puVar5;
        puVar5 = puVar5 + 1;
        puVar1 = puVar3;
        puVar3 = puVar3 + 1;
        *puVar2 = *puVar1;
    }
    return;
}

pub unsafe fn pass1_1010_2b98(mut param_1: u32, mut param_2: i16) -> *mut astruct_76 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar1 = (param_1 + 0x28);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22(
        (param_2 * 0x4 + iVar2 -0x156),
        (param_2 * 0x4 + iVar2 -0x158),
    );
}
pub unsafe fn pass1_1010_2bb9() {
    pass1_1010_286c();
    return;
}

pub unsafe fn pass1_1010_2bbe(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1010_29c6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_2bfc(param_1: *mut astruct_19, mut param_2: u16) -> *mut u16 {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xc) = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x10) = 0;
    param_1.offset_0x0 = 0x2cc2;
    (param_1 + 0x2) = 0x1010;
    return &param_1.offset_0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn unk_load_str_op_1010_2c34(param_1: u32) -> *mut c_char {
    let mut in_buffer_4: *mut c_char;
    let mut pUVar1: *mut u16;
    let mut in_buf_len_5: i16;
    let mut sVar2: i16;
    let mut in_EDX: *mut Struct57;
    let mut paVar3: *mut Struct57;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000fff6: u16;

    puVar4 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff6, 0x3),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    paVar3 = (in_EDX & 0xffff0000 | puVar4 >> 0x10);
    in_buffer_4 = puVar4;
    mem_op_1000_179c(0x80, paVar3);
    in_buf_len_5 = paVar3;
    sVar2 = in_buf_len_5;
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x80,
        in_buffer_4,
        in_buf_len_5,
    );
    pUVar1 = pass1_1000_3cea(CONCAT22(in_buf_len_5, in_buffer_4), s__1050_11c8);
    pass1_1010_e964(sVar2);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, in_buffer_4), CONCAT22(sVar2, pUVar1));
    return in_buffer_4;
}

pub unsafe fn pass1_1010_2c9c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn struct_1010_2cd2(param_1: *mut astruct_19, mut param_2: u16) {
    let mut in_EDX: u32;
    let mut uVar1: u16;
    let mut paVar2: *mut astruct_19;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe82: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut piVar6: *mut i16;
    let mut uVar7: u16;
    let mut local_6: i16;
    let mut local_4: i16;

    uVar1 = (in_EDX >> 0x10);
    paVar2 = struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x18) = 0;
    (param_1 + 0x22) = 0;
    (param_1 + 0x24) = 0;
    (param_1 + 0x26) = 0;
    (param_1 + 0x28) = 0;
    (param_1 + 0x52) = 0;
    (param_1 + 0x56) = 0;
    (param_1 + 0x5a) = 0;
    (param_1 + 0x5e) = 0;
    (param_1 + 0x5c) = 0;
    param_1.offset_0x0 = 0x36da;
    (param_1 + 0x2) = 0x1010;
    piVar6 = &local_4;
    uVar7 = SUB42(&DAT_1050_1050, 0x0);
    piVar4 = &local_6;
    uVar5 = SUB42(&DAT_1050_1050, 0x0);
    puVar3 = mixed_1010_20ba(
        CONCAT22(uVar1, (paVar2 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(piVar4, 0x48),
        in_stack_0000fe82,
        in_stack_0000ffa6,
        in_stack_0000ffac,
        in_stack_0000ffb0,
    );
    pass1_1008_3e94(
        (puVar3 & 0xffff0000 | (puVar3 + 0xe)),
        CONCAT22(uVar5, piVar4),
        CONCAT22(uVar7, piVar6),
    );
    (param_1 + 0xe) = 0x19001db;
    (param_1 + 0xa) = 0x140 - ((param_1 + 0xe) / 0x2 - local_4);
    (param_1 + 0xc) = 0xf0 - ((param_1 + 0x10) / 0x2 - local_6);
    (param_1 + 0x1a) = 0xa006e;
    (param_1 + 0x1e) = 0xa012c;
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x2a)), NULL, 0x28);
    return;
}
pub unsafe fn pass1_1010_2db2(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar5: *mut astruct_455;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.field0_0x0 = 0x36da;
    iVar5.field1_0x2 = 0x1010;
    puVar1 = iVar5[0xa].field3_0x6;
    uVar2 = (iVar5 + 0xb).field0_0x0;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*&iVar5[0xb].field2_0x4);
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Unable to use type for symbol uVar1

pub unsafe fn pass1_1010_2e02(mut param_1: u32, mut param_2: i16) -> u32 {
    let mut iVar2: *mut astruct_163;
    let mut uVar2: u16;
    let mut uVar1: u32;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0) {
        uVar1 = (param_1 + 0x5c);
        uVar2 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + param_2 * 0x4 + 0x2), (iVar2 + param_2 * 0x4));
    }
    return 0x0;
}
pub unsafe fn pass1_1010_2e30(mut param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: i16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0) {
        uVar1 = (param_1 + 0x5c);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        (iVar2 + param_4 * 0x4) = param_2;
        (iVar2 + param_4 * 0x4 + 0x2) = param_3;
    }
    return;
}
pub unsafe fn pass1_1010_2e5c(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uStack12: u32;

    uStack12 = param_3;
    if (param_3 == 0) {
        return;
    }
    //   for (; (uStack12 & 0xf) != 0; uStack12 >>= 0x4) {
    //   }
    while uStack12 & 0xf != 0 {
        uStack12 >>= 0x4;
    }
    return;
}
pub unsafe fn pass1_1010_2ee2(param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut extraout_DX: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut paStack6: *mut astruct_65;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x52) != 0) {
        return;
    }
    iVar2 = 0;
    (iVar3 + 0x28) = 0;
    uVar5 = *param_1;
    ppcVar1 = (uVar5 + 0x20);
    (**ppcVar1)();
    if (iVar2 == 0) {
        paStack6 = (iVar3 + 0x56);
    } else {
        ppcVar1 = (uVar5 + 0x14);
        (**ppcVar1)();
        paStack6 = CONCAT22(extraout_DX, iVar2);
        uVar5 = pass1_1010_2e02(param_1, (iVar2 + 0x12));
        pass1_1010_35a4((uVar5 >> 0x10), param_1, uVar5);
    }
    pass1_1010_32f4(param_1, paStack6);
    pass1_1010_1f62(param_1, 0x8);
    if ((iVar3 + 0x52) != 0) {
        return;
    }
    return;
}

// WARNING: Unable to use type for symbol uVar3
pub unsafe fn unk_destroy_win_op_1010_2fa0(param_1: *mut astruct_873) {
    astruct_872 * *ppaVar1;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_873;
    let mut uVar4: u16;
    let mut iStack4: *mut astruct_872;
    let mut uVar3: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar3.field39_0x28 = 0;
    iStack4 = null_mut();
    loop {
        ppaVar1 = &iVar3.field22_0x16;
        if (*ppaVar1 == iStack4 || *ppaVar1 < iStack4) {
            break;
        }
        uVar3 = (&iVar3.field_0x2a + iStack4 * 0x4);
        DestroyWindow16((uVar3 + 0x18));
        iStack4 = iStack4 + 1;
    }
    iVar3.field22_0x16 = null_mut();
    if ((iVar3.field82_0x54 | &iVar3.field_0x52) != 0) {
        iStack4 = null_mut();
        loop {
            uVar2 = &iVar3.field_0x52;
            if ((uVar2 + iStack4 * 0x4) != 0) {
                uVar2 = &iVar3.field_0x52;
                uVar2 = (uVar2 + iStack4 * 0x4);
                DestroyWindow16((uVar2 + 0x18));
                uVar2 = &iVar3.field_0x52;
                (uVar2 + iStack4 * 0x4) = 0;
            }
            iStack4 = iStack4 + 1;
            if iStack4 >= 0xa {
                break;
            }
        }
        fn_ptr_1000_17ce(*&iVar3.field_0x52);
        iVar3.field_0x52 = 0;
    }
    return;
}
