use crate::block_1000::block_1000_1000::fn_ptr_1000_17ce;
use crate::block_1018::block_1018_0000::pass1_1018_04de;
use crate::block_1018::block_1018_1000::pass1_1018_1a04;
use crate::block_1018::block_1018_2000::{pass1_1018_2076, pass1_1018_21f8, pass1_1018_2440, pass1_1018_2862, pass1_1018_2922};
use crate::block_1018::block_1018_4000::{pass1_1018_4760, pass1_1018_4882};
use crate::draw_ops::clenaup_win_ui_1018_4d22;

pub unsafe fn pass1_1010_ed22(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x24) = param_2;
    pass1_1010_1f62((param_1 & 0xffff | uVar1 << 0x10), 0x12);
    return;
}

pub unsafe fn pass1_1010_ed3e(mut param_1: u32) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_1 + 0x16));
    return;
}

pub unsafe fn write_to_file_1010_ed58(mut param_1: u32, mut param_2: u32) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut in_stack_0000ffc4: HFILE16;
    let mut local_22: u32;
    let mut uStack30: u16;
    let mut local_12: [u32; 0x2] = [0; 0x2];
    let mut local_a: u32;
    let mut iStack4: i16;

    BVar3 = write_to_file_1008_7cac(param_2);
    if (BVar3 != 0) {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        local_12[0] = (iVar6 + 0x16);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_12), 0x4, in_stack_0000ffc4);
        if (BVar3 != 0) {
            local_a = (iVar6 + 0x1a);
            BVar3 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_a),
                0x4,
                in_stack_0000ffc4,
            );
            if (BVar3 != 0) {
                local_a = (iVar6 + 0x20);
                BVar3 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, &local_a),
                    0x4,
                    in_stack_0000ffc4,
                );
                if (BVar3 != 0) {
                    local_a = (iVar6 + 0x24);
                    BVar3 = write_to_file_1008_7e1c(
                        param_2,
                        CONCAT22(0x1050, &local_a),
                        0x4,
                        in_stack_0000ffc4,
                    );
                    if (BVar3 != 0) {
                        local_a = local_a & 0xffff0000 | (iVar6 + 0x30);
                        BVar3 = write_to_file_1008_7e1c(
                            param_2,
                            CONCAT22(0x1050, &local_a),
                            0x2,
                            in_stack_0000ffc4,
                        );
                        if (BVar3 != 0) {
                            local_a = local_a & 0xffff0000 | (iVar6 + 0x32);
                            BVar3 = write_to_file_1008_7e1c(
                                param_2,
                                CONCAT22(0x1050, &local_a),
                                0x2,
                                in_stack_0000ffc4,
                            );
                            if (BVar3 != 0) {
                                iStack4 = 0;
                                loop {
                                    piVar1 = (iVar6 + 0x30);
                                    if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                                        return;
                                    }
                                    uVar2 = (iVar6 + 0x2e);
                                    puVar5 = ((iVar6 + 0x2c) + iStack4 * 0x6);
                                    local_22 = *puVar5;
                                    uStack30 = (puVar5 + 1);
                                    local_12[0] = local_12[0] & 0xffff0000 | ZEXT24(&local_22);
                                    iVar4 = write_to_file_1008_7b4c(
                                        param_2,
                                        CONCAT22(0x1050, &local_22),
                                    );
                                    if (iVar4 == 0) {
                                        break;
                                    }
                                    iStack4 += 0x1;
                                }
                            }
                        }
                    }
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn pass1_1018_0000(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut BVar5: bool;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut pSVar7: *mut StructD;
    let mut uVar8: u16;
    let mut local_20: [u8; 0x10] = [0; 0x10];
    let mut iStack16: i16;

    // Segment:    4
    // Offset:     00024460
    // Length:     ee6a
    // Min Alloc:  ee6a
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    pSVar7 = CONCAT22(in_register_0000000a, param_2);
    read_file_1008_7cfe(param_4, (param_4 >> 0x10), 0x2);
    if (param_1 == 0) {
        u16_1050_0310 = 0x6d4;
    } else {
        iVar4 = param_3;
        BVar5 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar4 + 0x16)), 0x4);
        if ((((BVar5 != 0) && (
            BVar5 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar4 + 0x1a)), 0x4),
            BVar5 != 0,
        )) && (
            BVar5 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar4 + 0x20)), 0x4),
            BVar5 != 0,
        )) && ((
            BVar5 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar4 + 0x24)), 0x4),
            BVar5 != 0x0 && (
                BVar5 = read_file_1008_7dee(
                    param_4,
                    (param_3 & 0xffff0000 | (iVar4 + 0x30)),
                    0x2,
                ),
                BVar5 != 0,
            ),
        ) && (
            BVar5 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar4 + 0x32)), 0x2),
            BVar5 != 0,
        ))) {
            uVar8 = (param_3 >> 0x10);
            if ((iVar4 + 0x30) != 0) {
                iVar2 = (iVar4 + 0x32);
                if (_PTR_LOOP_1050_5f2c == 0) {
                    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
                    PTR_LOOP_1050_5f2e = pSVar7;
                } else {}
                uVar6 = fn_ptr_op_1000_1708(
                    iVar2 * 0x6,
                    0x0,
                    0x1,
                    PTR_LOOP_1050_5f2c,
                    PTR_LOOP_1050_5f2e,
                );
                (iVar4 + 0x2c) = uVar6;
                (iVar4 + 0x2e) = PTR_LOOP_1050_5f2e;
                pass1_1008_3e38(CONCAT22(0x1050, local_20));
                // for (iStack16 = 0; piVar1 = (iVar4 + 0x30), *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 0x1)
                iStack16 = 0;
                piVar1 = (iVar4 + 0x30);
                while *piVar1 != iStack16 && iStack16 <= *piVar1 {
                    BVar5 = read_file_1008_7bc8(param_4, CONCAT22(0x1050, local_20));
                    if (BVar5 == 0) {
                        u16_1050_0310 = 0x6d0;
                        return;
                    }
                    uVar3 = (iVar4 + 0x2c);
                    pass1_1008_3f62(
                        (uVar3 & 0xffff0000 | (uVar3 + iStack16 * 0x6)),
                        CONCAT22(0x1050, local_20),
                    );
                    iStack16 += 1;
                }
            }
            return;
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn pass1_1018_03ea(mut param_1: u32, mut param_2: i16) {
    if (param_2 != 0x5) {
        return;
    }
    pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 0xa)), 0x5);
    return;
}

pub unsafe fn pass1_1018_0526(param_1: *mut u16, param_2: u8) -> *mut u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
    pass1_1010_eb66(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_0532(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1010_eb66(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1018_0aa0(mut param_1: u32, mut param_2: u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x14) = param_2;
    pass1_1018_04de((iVar1 + 0x2c), (iVar1 + 0x3c));
    return;
}

pub unsafe fn pass1_1018_0d76(mut param_1: u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x2c);
    if ((uVar1 + 0x20) == (param_1 + 0x3c)) {
        return;
    }
    return;
}

pub unsafe fn pass1_1018_1842(param_1: *mut u16, param_2: u8) -> *mut u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x20));
    pass1_1018_078e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_184e(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_078e(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1018_1f6a(
    mut param_1: u16,
    param_2: *mut StructD,
    param_3: u8,
) -> *mut StructD {
    param_2 = (param_2 & 0xffff0000 | (param_2 - 0x20));
    pass1_1018_1a04(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn FUN_1018_1f76() {
    return;
}

pub unsafe fn pass1_1018_1f7a(mut param_1: i16, mut param_2: u16) -> u32 {
    return CONCAT22(param_2, param_1 + 0x2a);
}

pub unsafe fn pass1_1018_1f8a(param_1: *mut StructD, param_2: u8) -> *mut u16 {
    pass1_1018_1a04(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return &param_1.address_offset_field_0x0;
}

pub unsafe fn pass1_1018_21c2(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1018_2076(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_289c(mut param_1: u16, mut param_2: u32, mut param_3: i16) {
    let mut uVar1: u16;

    if (param_3 == 1) {
        (param_2 + 0x4) = 0;
        return;
    }
    if (param_3 == 0x2) {
        pass1_1018_2922(param_2 & 0xffff0000 | (param_2 - 0x1c));
    } else {
        if ((((param_3 - 0x3 < 1) || (SBORROW2(param_3 - 0x3, 1))) || (0x1 < param_3 - 0x5)) || ((param_2 + 0x4) == 0)) {
            return;
        }
        uVar1 = param_2 - 0x1c;
        pass1_1018_2862((param_2 & 0xffff0000 | uVar1));
        if ((param_1 | uVar1) != 0) {
            (param_2 + 0x8) = 0x1;
        }
    }
    pass1_1010_1f62((param_2 & 0xffff0000 | (param_2 - 0x1c)), param_3);
    return;
}

pub unsafe fn win_op_1018_294a(
    mut param_1: u16,
    in_string_6: u16,
    param_3: *mut astruct_8,
    mut param_4: u16,
    mut param_5: u32,
) {
    if (((param_3 + 0x18) != 0) && (param_5 == 0x280)) {
        (param_3 + 0x18) = 0;
    }
    create_dc_1018_4e04(param_1, in_string_6, param_3, param_4, param_5, param_5);
    return;
}


pub unsafe fn pass1_1018_2aa3() {
    pass1_1018_21f8();
    return;
}


pub unsafe fn pass1_1018_2aa8(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x1c));
    pass1_1018_2440(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_2afa(param_1: u32) {
    fn_ptr_1000_17ce(*param_1);
    return;
}

pub unsafe fn pass1_1018_2ee4(param_1: *mut astruct_126, mut param_2: u16) {
    let mut uVar1: u32;
    let mut cVar2: u8;
    let mut uVar3: u16;

    if (param_2 != 0x12) {
        if (param_2 < 0x13) {
            cVar2 = param_2;
            if (cVar2 == '\x01') {
                (param_1 + 0x162) = 0;
                return;
            }
            //      if (('\x02' < (cVar2 -1)) && ((cVar2 -0x4) < '\x03')) goto LAB_1018_2f06;
        }
        return;
    }
    uVar1 = (param_1 + 0x162);
    (param_1 + 0x15a) = (uVar1 + 0x24); //
    // LAB_1018_2f06:
    uVar3 = param_1 - 0x20;
    pass1_1018_31fa(uVar3, param_1, (param_1 & 0xffff0000 | uVar3));
    pass1_1010_1f62((param_1 & 0xffff0000 | uVar3), param_2);
    return;
}

pub unsafe fn pass1_1018_32a6(param_1: *mut u16, param_2: u8) -> *mut u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x20));
    pass1_1018_2c60(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_32b2(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_2c60(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1018_3ea4(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar4: *mut astruct_455;

    pass1_1008_cac6(param_1);
    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = iVar4[0x26].field3_0x6;
    uVar2 = (iVar4 + 0x27).field0_0x0;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 1);
    }
    iVar4[0x26].field3_0x6 = 0;
    return;
}

pub unsafe fn pass1_1018_46e6(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1018_33b4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_495a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_4980(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_49a6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_49cc(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_49f2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4882(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_4a18(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_4a3e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_4a64(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_4ae0(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    clenaup_win_ui_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_4c2c(mut param_1: u32, param_2: *mut u32, mut param_3: u16) {
    let mut in_EDX: u32;
    let mut unaff_SI: u16;
    let mut puVar1: *mut u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;

    (param_1 + 0xa) = *param_2;
    (param_1 + 0xe) = param_2[0x1];
    puVar1 = mixed_1010_20ba(
        (in_EDX & 0xffff0000 | param_1),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    pass1_1010_5fb0(puVar1, 0x0, (param_1 + 0xa), param_1, (param_1 + 0x12));
    return;
}

pub unsafe fn pass1_1018_4c78(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn unk_win_ui_op_1018_4f18(
    mut param_1: u32,
    param_2: *mut Struct57,
    param_3: *mut astruct_39,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_394;
    let mut rect: *mut RECT16;
    let mut iVar4: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar9: *mut Struct57;
    let mut iVar6: *mut astruct_39;
    let mut uVar10: u16;
    let mut uVar11: u8;
    let mut unaff_CS: u16;
    let mut HVar12: HWND16;
    let mut local_12: RECT16;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut uStack10: u32;
    let mut puStack6: *mut u32;
    let mut uVar5: u32;
    let mut paVar8: *mut Struct57;

    uVar2 = FUN_1010_830a(param_1, param_2, unaff_CS, _u16_1050_14cc, param_4);
    uVar5 = uVar2;
    uVar6 = SUB42(param_2, 0x0);
    puStack6 = CONCAT22(uVar6, uVar2);
    ppcVar1 = (*puStack6 + 0x14);
    (**ppcVar1)();
    paVar3 = uVar5;
    uStack10 = uVar5 & 0xffff | param_2 << 0x10;
    uVar10 = (param_3 >> 0x10);
    iVar6 = param_3;
    if (&iVar6.field12_0xe != 0) {
        uVar7 = iVar6.field13_0x10;
        paVar3 = &iVar6.field12_0xe;
        param_2 = (param_2 & 0xffff0000 | (uVar7 | paVar3));
        if ((uVar7 | paVar3) != 0) {
            ppcVar1 = paVar3;
            (**ppcVar1)(0x1010, paVar3, uVar7, 0x1, uVar2, uVar6);
        }
    }
    mem_op_1000_179c(0x14, param_2);
    uVar7 = param_2 | paVar3;
    paVar9 = (param_2 & 0xffff0000);
    paVar8 = (paVar9 | uVar7);
    if (uVar7 == 0) {
        paVar3 = null_mut();
    } else {
        struct_1008_4c58(paVar3);
        paVar9 = paVar8;
    }
    iVar6.field12_0xe = paVar3;
    iVar6.field13_0x10 = paVar9;
    pass1_1008_4d84(paVar9, &iVar6.field12_0xe, uStack10);
    rect = &local_12;
    HVar12 = HWND16_1050_0396;
    GetClientRect16(rect, &DAT_1050_1050);
    uVar11 = 0;
    mem_op_1000_179c(0x1e, paVar9);
    uVar7 = paVar9 | rect;
    if (uVar7 == 0) {
        iVar6.field10_0xa = 0;
    } else {
        iVar4 = (iStack12 - local_12.y) + 1;
        uVar11 = 0x8;
        pass1_1008_405c(
            CONCAT22(paVar9, rect),
            &iVar6.field12_0xe,
            iVar4,
            (iStack14 - local_12.x) + 1,
        );
        iVar6.field10_0xa = iVar4;
        iVar6.field11_0xc = uVar7;
    }
    if (puStack6.is_null() == false) {
        ppcVar1 = *puStack6;
        (**ppcVar1)(
            uVar11,
            puStack6,
            (puStack6 >> 0x10),
            0x1,
            HVar12,
            uVar2,
            uVar6,
            puStack6,
            puStack6,
        );
    }
    return;
}

pub unsafe fn pass1_1018_5032(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    clenaup_win_ui_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_51d2(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0xe);
    uVar2 = (iVar4 + 0x10);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0xe) = 0;
    return;
}
