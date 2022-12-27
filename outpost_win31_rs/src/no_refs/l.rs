use crate::unk::block_1010_9000::pass1_1010_9fa6;
use crate::unk::block_1010_a000::pass1_1010_a5ca;
use crate::unk::block_1010_b000::bad_1010_bf08;
use crate::unk::block_1010_c000::pass1_1010_c28a;

pub fn pass1_1010_9f72(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, uVar1, (param_1 + 0xe), param_2);
    return;
}

pub fn pass1_1010_9f8c(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, uVar1, (param_1 + 0xa), param_2);
    return;
}


pub fn pass1_1010_9fa6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: i16,
) -> u16 {
    let mut uVar1: u16;
    let mut lVar2: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    if (param_3 != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), param_3);
        loop {
            lVar2 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            uVar1 = (lVar2 >> 0x10);
            if (lVar2 == 0) {
                break;
            }
            if ((lVar2 + 0x4) == param_4) {
                return (lVar2 + 0x6);
            }
        }
    }
    return 0x0;
}

pub fn pass1_1010_a0a0(mut param_1: u16, param_2: *mut astruct_252) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut uVar9: u32;
    let mut bVar10: bool;
    let mut bVar11: bool;
    let mut puVar12: *mut u32;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe4: u16;
    let mut iStack12: i16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_2 + 0xa));
    iStack12 = 0x4;
    puVar12 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x2),
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    uVar9 = puVar12 >> 0x10;
    if ((PTR_LOOP_1050_13ae != &u16_1050_0002) && (PTR_LOOP_1050_13ae != (&PTR_LOOP_1050_0000 + 1))) {
        iStack12 = 0x2;
    }
    loop {
        loop {
            uVar6 = uVar9;
            uVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            uVar9 = (uVar6 | uVar4);
            if ((uVar6 | uVar4) == 0) {
                return;
            }
            iVar2 = (uVar4 + 0x4);
            if (iVar2 != 0x12) {
                break;
            }
            piVar1 = (uVar4 + 0x6);
            bVar11 = SBORROW2(*piVar1, 0x2);
            iVar3 = *piVar1 - 0x2;
            bVar10 = iVar3 == 0; //
            // LAB_1010_a151:
            if (!bVar10 && bVar11 == (iVar3 < 0x0)) {
                //
                // LAB_1010_a153:
                iVar2 = (uVar4 + 0x6);
                uVar9 = (iVar2 % iStack12);
                piVar1 = (uVar4 + 0x6);
                *piVar1 = *piVar1 - iVar2 / iStack12;
            }
        }
        if (((iVar2 != 0x3e) && (iVar2 != 0x41)) && (iVar2 != 0x80)) {
            if (iVar2 == 0x83) {
                piVar1 = (uVar4 + 0x6);
                bVar11 = SBORROW2(*piVar1, 1);
                iVar2 = *piVar1;
                iVar3 = iVar2 - 0x1;
                bVar10 = iVar2 == 0x1;
                // TODO: goto LAB_1010_a151;
            }
            // TODO: goto LAB_1010_a153;
        }
        iVar2 = (uVar4 + 0x6);
        uVar7 = iVar2 >> 0xf;
        uVar5 = iVar2 / 0x2;
        piVar1 = (uVar4 + 0x6);
        *piVar1 = *piVar1 - uVar5;
        if (uVar5 == 0) {
            uVar5 = 0x1;
        }
        uVar9 = uVar7;
        pass1_1010_9fee(CONCAT22(uVar5, uVar7), param_2, uVar5, (uVar4 + 0x4));
    }
}

pub fn pass1_1010_a568(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    pass1_1030_8344(_u16_1050_5748, 0x8000001);
    pass1_1030_2622(CONCAT22(param_2, param_1), param_5);
    return;
}

pub fn pass1_1010_a58a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    pass1_1030_8344(_u16_1050_5748, 0x8000001);
    pass1_1030_266c(param_1, CONCAT22(param_5, param_2));
    return;
}

pub fn pass1_1010_a5ac(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar1: u32;

    uVar1 = struct_op_1030_73a8(param_3, in_AX, in_DX);
    return (uVar1 + 0x20);
}

pub fn pass1_1010_a5ec(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut extraout_DX: u16;
    let mut puVar6: *mut u32;
    let mut uStack6: u32;

    uVar2 = param_5 | param_5;
    if (param_5 != 0) {
        pass1_1030_8344(_u16_1050_5748, 0x8000001);
        uStack6 = CONCAT22(param_1, uVar2);
        puVar6 = struct_op_1030_73a8(param_5, uVar2, param_1);
        uVar5 = (puVar6 >> 0x10);
        uVar4 = (puVar6 + 0x20);
        if (uVar4 != param_4) {
            uVar3 = param_4;
            pass1_1010_a5ca(param_4, uVar5, param_2, param_3, uVar4);
            if ((uVar4 != 0x70) && (uVar3 < 0x0)) {
                pass1_1030_25d8(CONCAT22(param_1, uVar2), uVar3 + 0x1, uVar4);
            }
            ppcVar1 = (*puVar6 + 0x8);
            uVar4 = param_4;
            (**ppcVar1)();
            if (param_4 != 0x70) {
                pass1_1010_a5ca(uVar4, extraout_DX, param_2, param_3, param_4);
                if (uVar4 < 0x0) {
                    pass1_1030_25d8(uStack6, uVar4 - 0x1, param_4);
                }
            }
        }
    }
    return;
}


pub fn load_string_1010_ac92(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
) -> *mut c_char {
    let mut pcVar1: *mut c_char;

    if ((0x0 < param_3) && (param_3 < 0x43)) {
        pcVar1 = load_string_1010_847e(_u16_1050_14cc, param_3 + 0x664);
        return pcVar1;
    }
    return NULL;
}


pub fn pass1_1010_acc0(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar1: u32;

    uVar1 = struct_op_1030_73a8(param_3, in_AX, in_DX);
    if ((uVar1 + 0x12) != 0x4) {
        return 0x1;
    }
    return 0x0;
}

pub fn pass1_1010_acec(mut param_1: u32, mut param_2: i16) {
    if (param_2 == 1) {
        (param_1 + 0x12e) = 0;
    } else if (param_2 != 0x5) {
        return;
    }
    pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 0xa)), param_2);
    return;
}

pub fn pass1_1010_ad22(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut uVar1: u32;
    let mut puVar2: *mut u16;

    uVar1 = (param_2 + 0x138);
    puVar2 = &param_4;
    pass1_1030_627e(
        puVar2,
        param_1,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        (uVar1 + 0x20),
    );
    if ((param_1 | puVar2) == 0) {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(param_1, puVar2));
    return;
}


pub fn pass1_1010_ad64(
    mut param_1: u32,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u32,
) {
    if (param_5 != 0) {
        param_1 = (param_5 + 0x2e);
        if ((param_1 + 0x200) == 0x8000002) {
            return;
        }
    }
    pass1_1010_c58as(
        param_1,
        param_2,
        CONCAT22(param_4, param_3),
        (param_4 >> 0x10),
        param_5,
    );
    return;
}


pub fn string_op_1010_ada6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: i16,
) -> *mut c_char {
    let mut pcVar1: *mut c_char;
    let mut pcStack6: *mut c_char;

    pcStack6 = null_mut();
    if (param_5 == 0x6) {
        //    if (param_4 == 0) goto LAB_1010_adee;
        pcVar1 = string_op_1020_c222(param_4);
    } else {
        if (param_5 != 0x7) {
            return NULL;
        }
        //    if (param_4 == 0) goto LAB_1010_adee;
        pcVar1 = string_op_1020_c2f8(param_4);
    }
    pcStack6 = CONCAT22(param_1, pcVar1); //
    // LAB_1010_adee:
    if (pcStack6.is_null()) {
        pcStack6 = load_string_1010_847e(_u16_1050_14cc, 0x531);
    }
    return pcStack6;
}


pub fn pass1_1010_ae12(
    param_1: *mut c_char,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut c_char,
    mut param_5: i16,
) -> u16 {
    let mut pcVar1: *mut c_char;
    let mut iVar2: i16;
    let mut uStack4: u16;

    if (param_5 == 0x6) {
        // for (uStack4 = 0; uStack4 < 0x15; uStack4 += 1)
        for uStack4 in 0..0x15 {
            pcVar1 = string_op_1020_c222(uStack4);
            iVar2 = pass1_1000_3d7a(param_4, CONCAT22(param_1, pcVar1));
            if (iVar2 == 0) {
                return uStack4;
            }
        }
    } else if (param_5 == 0x7) {
        // for (uStack4 = 0; uStack4 < 0x11; uStack4 += 1)
        for uStack4 in 0..0x11 {
            pcVar1 = string_op_1020_c2f8(uStack4);
            iVar2 = pass1_1000_3d7a(param_4, CONCAT22(param_1, pcVar1));
            if (iVar2 == 0) {
                return uStack4;
            }
        }
    }
    return 0xffff;
}

pub fn pass1_1010_ae92(
    mut param_1: u32,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u32,
) {
    let mut bVar1: u8;
    let mut uVar2: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut paVar6: *mut astruct_15;
    let mut uVar7: u32;
    let mut paVar8: *mut astruct_67;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut iVar11: i16;
    let mut uVar12: u8;
    let mut uVar13: u8;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut iVar16: i16;
    let mut uVar3: u16;

    if (param_3 == 0x15) {
        uVar7 = struct_op_1030_73a8(param_4, 0x15, param_5);
        uVar3 = (uVar7 >> 0x10);
        if ((uVar3 | uVar7) != 0) {
            (uVar7 + 0x20) = param_2;
            return;
        }
    } else if (param_3 < 0x16) {
        bVar1 = param_3 - 0x6;
        uVar3 = param_3 & 0xff00 | bVar1;
        if (bVar1 == 0) {
            pass1_1030_7f1a(param_4, param_2);
            paVar6 = struct_op_1030_73a8(param_4, uVar3, param_5);
            uVar5 = (param_5 >> 0x10);
            uVar2 = pass1_1010_b028(param_1, (param_1 >> 0x10), paVar6);
            uVar7 = pass1_1030_8326();
            iVar4 = (uVar7 >> 0x10);
            if (((uVar2 == 0xe) && (iVar4 != 0x0 || (0x32 < uVar7))) && (param_2 == 0x1 || ((param_2 == 0x2 || (param_2 == 0x4)) || (param_2 == 0x3)))) {
                uVar15 = 0;
                iVar16 = 0xb;
                uVar12 = 0x1;
                uVar13 = 0;
                uVar14 = 0;
                uVar10 = 0;
                iVar11 = 0;
                uVar9 = 0;
                paVar8 = mixed_1010_20ba(
                    CONCAT22(uVar5, iVar4),
                    _u16_1050_0ed0,
                    0x37,
                    in_stack_0000fe88,
                    in_stack_0000ffac,
                    in_stack_0000ffb2,
                    in_stack_0000ffb6,
                );
                post_win_msg_1008_a0e4(
                    paVar8,
                    CONCAT22(uVar10, uVar9),
                    iVar11,
                    CONCAT11(uVar13, uVar12),
                    CONCAT22(uVar15, uVar14),
                    iVar16,
                );
                return;
            }
        } else if (param_3 == '\x0a') {
            pass1_1030_7eda(param_4, param_2);
            return;
        }
    }
    return;
}

pub fn pass1_1010_af66(mut param_1: u16, mut param_2: u32, mut param_3: u16) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;

    uVar1 = (param_2 + 0x138);
    uVar2 = (uVar1 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2);
    uVar3 = param_1 | uVar2;
    if (uVar3 == 0) {
        return;
    }
    pass1_1038_5050(uVar2, uVar3, uVar2 & 0xffff | param_1 << 0x10, param_3);
    return;
}

pub fn pass1_1010_afa2(mut param_1: u16, mut param_2: u32, mut param_3: u16) {
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar1 = (param_2 + 0x138);
    uVar2 = (uVar1 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2);
    if ((param_1 | uVar2) == 0) {
        return;
    }
    pass1_1038_50e0(uVar2, uVar2 & 0xffff | param_1 << 0x10, param_3);
    return;
}

pub fn pass1_1010_bf1e(
    mut param_1: i16,
    param_2: *mut u8,
    mut param_3: u32,
    param_4: *mut i16,
) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut paVar4: *mut astruct_92;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u32;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uStack40: u32;
    let mut iStack36: i16;
    let mut puStack26: *mut u16;
    let mut local_16: *mut astruct_92;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    bad_1010_bf08(param_3, (param_3 >> 0x10), 0x1000000);
    iVar2 = param_1 - 0x1;
    *param_4 = iVar2;
    uVar3 = iVar2 * 0x18;
    mem_op_1000_179c(uVar3, paVar6);
    uVar5 = paVar6;
    uStack40 = CONCAT22(uVar5, uVar3);
    uVar7 = (uVar5 | uVar3);
    iVar8 = param_4;
    uVar9 = (param_4 >> 0x10);
    if ((uVar5 | uVar3) == 0) {
        (iVar8 + 0x2) = 0;
    } else {
        pass1_1000_5586(0x4092, 0x1020, iVar2, 0x18, uVar3, uVar5);
        (iVar8 + 0x2) = uStack40;
    }
    pass1_1028_dc52(CONCAT22(0x1050, &local_16), 0x1, 0x0, 0x100);
    puStack26 = (iVar8 + 2);
    iStack36 = 0;
    loop {
        uVar3 = uVar7;
        paVar4 = &local_16;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
        uVar7 = (uVar3 | paVar4);
        if ((uVar3 | paVar4) == 0) {
            break;
        }
        uVar1 = &paVar4.field6_0x10;
        if (uVar1 != 0) {
            uVar7 = uVar1 >> 0x10;
            pass1_1008_3f62(puStack26, (uVar1 & 0xffff0000 | (uVar1 + 0x4)));
            (puStack26 + 0xc) = iStack36;
            iStack36 += 0x1;
            puStack26 = (puStack26 & 0xffff0000 | (puStack26 + 0x18));
        }
    }
    return;
}

pub fn pass1_1010_bffa(mut param_1: i16, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar5: *mut astruct_257;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_register_0000000a: u16;
    let mut paVar9: *mut Struct57;
    let mut iVar6: *mut astruct_254;
    let mut iVar7: *mut astruct_255;
    let mut iVar8: *mut astruct_256;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut iStack34: i16;
    let mut iStack30: i16;
    let mut local_18: *mut astruct_92;
    let mut puStack6: *mut u32;

    paVar9 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0xa, paVar9);
    uVar7 = SUB42(paVar9, 0x0);
    puStack6 = CONCAT22(uVar7, param_1);
    iVar4 = param_1;
    bad_1010_bf08(param_3, (param_3 >> 0x10), 0x2000000);
    (param_1 + 0x8) = iVar4;
    if (iVar4 == 0) {
        (param_1 + 0x8) = 0x1;
    }
    uVar5 = (param_1 + 0x8) << 0x2;
    mem_op_1000_179c(uVar5, paVar9);
    puStack6 = uVar5;
    (param_1 + 0x2) = paVar9;
    if ((paVar9 | puStack6) == 0) {
        (param_1 + 0x8) = 0;
    } else {
        iVar4 = (param_1 + 0x8) * 0x2;
        mem_op_1000_179c(iVar4, paVar9);
        (param_1 + 0x4) = iVar4;
        (param_1 + 0x6) = paVar9;
        uVar5 = paVar9 | (param_1 + 0x4);
        if (uVar5 != 0) {
            uVar6 = FUN_1010_830a(uVar5, paVar9, 0x1000, _u16_1050_14cc, 0x1b4);
            puVar1 = *puStack6;
            *puVar1 = uVar6;
            (puVar1 + 0x2) = paVar9;
            (param_1 + 0x4) = 0;
            pass1_1028_dc52(CONCAT22(0x1050, &local_18), 0x1, 0x0, 0x200);
            iStack30 = 0x1;
            loop {
                puVar5 = &local_18;
                uVar6 = 0x1028;
                pass1_1028_e4ec(CONCAT22(0x1050, puVar5));
                uVar5 = paVar9;
                uVar8 = uVar5 | puVar5;
                paVar9 = (paVar9 & 0xffff0000 | uVar8);
                if (uVar8 == 0) {
                    break;
                }
                piVar2 = puVar5.field16_0x10;
                uVar8 = piVar2;
                iVar4 = *piVar2;
                iStack34 = 0;
                uVar5 = iVar4 - 0x1;
                if (uVar5 < 0xa) {
                    uVar6 = 0x1010;
                    match uVar5 {
                        0x0 => {
                            iStack34 = 0x1b5;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x1 => {
                            iStack34 = 0x1b6;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x2 => {
                            iStack34 = 0x1b7;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x3 => {
                            iStack34 = 0x1b8;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x4 => {
                            iStack34 = 0x1b9;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x5 => {
                            iStack34 = 0x1ba;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x6 => {
                            iStack34 = 0x1bb;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x7 => {
                            iStack34 = 0x1bc;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x8 => {
                            iStack34 = 0x1bd;
                            uVar5 = uVar8;
                        }
                        // break;
                        0x9 => {
                            iStack34 = 0x1be;
                            uVar5 = uVar8;
                        }
                    }
                }
                uVar6 = FUN_1010_830a(uVar5, paVar9, uVar6, _u16_1050_14cc, iStack34);
                uVar11 = (*puStack6 >> 0x10);
                iVar10 = *puStack6;
                (iVar10 + iStack30 * 0x4) = uVar6;
                (iVar10 + iStack30 * 0x4 + 0x2) = paVar9;
                uVar3 = (param_1 + 0x4);
                (uVar3 + iStack30 * 0x2) = iVar4;
                iStack30 += 0x1;
            }
            return;
        }
    }
    return;
}

pub fn pass1_1010_c1ba(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut paVar1: *mut astruct_92;
    let mut iStack28: i16;
    let mut local_16: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_16), 0x1, 0x0, 0x200);
    iStack28 = 0x1;
    loop {
        paVar1 = &local_16;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        param_1 |= paVar1;
        if ((param_1 == 0) || (iStack28 == param_4)) {
            break;
        }
        iStack28 += 0x1;
    }
    return;
}

pub fn pass1_1010_c234(mut param_1: u16, param_2: *mut u8) -> *mut c_char {
    let mut pcVar1: *mut c_char;

    pass1_1010_c28a(param_2);
    if ((param_2 | param_1) == 0) {
        return NULL;
    }
    pcVar1 = pass1_1038_4d28(CONCAT22(param_2, param_1));
    return pcVar1;
}

pub fn pass1_1010_c25e(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut c_char,
) {
    if (param_5.is_null() == false) {
        pass1_1010_c28a(param_2);
        if ((param_2 | param_1) != 0) {
            pass1_1038_4d3c(CONCAT22(param_2, param_1), param_5, param_2 | param_1);
        }
    }
    return;
}

pub fn pass1_1010_c28a(param_1: *mut u8) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffee: u16;

    puVar5 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffee, 0x2f),
        in_stack_0000fe96,
        in_stack_0000ffba,
        in_stack_0000ffc0,
        in_stack_0000ffc4,
    );
    uVar2 = (puVar5 >> 0x10);
    uVar1 = (puVar5 + 0x24);
    uVar4 = (puVar5 + 0x26);
    uVar3 = uVar4 | uVar1;
    if (uVar3 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(uVar4, uVar1));
        if ((uVar4 | uVar3) != 0) {
            return;
        }
    }
    return;
}

pub fn pass1_1010_c2d8(mut param_1: u16, mut param_2: u16, param_3: i32) {
    let mut uVar1: u16;
    let mut uStack6: u16;

    if ((param_3 != 0) && (
        uVar1 = (param_3 >> 0x10),
        uStack6 = (param_3 + 0x2e),
        ((param_3 + 0x30) | uStack6) != 0,
    )) {
        return;
    }
    return;
}

pub fn pass1_1010_c312() -> u32 {
    return CONCAT22((_PTR_LOOP_1050_65e2 + 0x2), *_PTR_LOOP_1050_65e2);
}

pub fn pass1_1010_c3c2(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u32,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut local_40c: [u8; 0x400] = [0; 0x400];
    let mut uStack12: u16;
    let mut uStack10: u32;
    let mut pcStack6: *mut c_char;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    pcStack6 = param_4;
    if (param_4 == 0) {
        mem_op_1000_179c(0x100, paVar3);
        pcStack6 = (param_4 & 0xffff | paVar3 << 0x10);
    }
    uStack10 = struct_op_1030_73a8(param_5, param_4, paVar3);
    uVar2 = (uStack10 >> 0x10);
    uStack12 = (uStack10 + 0xc);
    uVar1 = pass1_1020_bd80(uStack12);
    unk_str_op_1000_3d3e(CONCAT22(0x1050, local_40c), CONCAT22(uVar2, uVar1));
    pass1_1030_8086(param_5);
    sys_1000_3f9c(pcStack6, 0x105038c5, local_40c);
    return;
}

pub fn string_op_1010_c446(
    mut param_1: u16,
    mut param_2: u32,
    param_3: *mut c_char,
    param_4: *mut astruct_419,
) {
    let mut iVar1: i16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u32;
    let mut pcVar4: *mut c_char;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_buffer_4: *mut c_char;
    let mut in_buf_len_5: i16;
    let mut uStack22: u16;
    let mut pcStack6: *mut c_char;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    pcStack6 = param_3;
    if (param_3.is_null()) {
        mem_op_1000_179c(0x100, paVar2);
        pcStack6 = (param_3 & 0xffff | paVar2 << 0x10);
    }
    uVar3 = struct_op_1030_73a8(param_4, param_3, paVar2);
    struct_1010_dd5e(param_2, (param_2 >> 0x10), param_4);
    iVar1 = (uVar3 + 0x12);
    if (0x6 < iVar1 - 0x3) {
        return;
    }
    in_buffer_4 = pcStack6;
    in_buf_len_5 = (pcStack6 >> 0x10);
    uVar6 = _u16_1050_14cc;
    uVar7 = (_u16_1050_14cc >> 0x10);
    match iVar1 {
        _ => {}
        // break;
        0x6 => {
            load_string_1010_84e0(uVar6, uVar7, 0x3ff, in_buffer_4, in_buf_len_5);
            uStack22 = str_op_1000_3da4(pcStack6);
            pcVar4 = load_string_1010_847e(_u16_1050_14cc, 0x61e);
            uVar8 = pcVar4;
            uVar5 = SUB42(s_____s__lu_1050_38d7, 0x0);
        }
        // TODO: goto LAB_1010_c4f9;
        0x7 | 0x9 => {}
        // break;
        0x8 => {
            load_string_1010_84e0(uVar6, uVar7, 0x3ff, in_buffer_4, in_buf_len_5);
            uStack22 = str_op_1000_3da4(pcStack6);
            pcVar4 = load_string_1010_847e(_u16_1050_14cc, 0x61e);
            uVar8 = pcVar4;
            uVar5 = SUB42(s_____s__lu_1050_38cd, 0x0); //
            // LAB_1010_c4f9:
            sys_1000_3f9c(
                (pcStack6 & 0xffff0000 | ZEXT24(in_buffer_4 + uStack22)),
                CONCAT22(0x1050, uVar5),
                uVar8,
            );
            return;
        }
    };
    load_string_1010_84e0(uVar6, uVar7, 0x3ff, in_buffer_4, in_buf_len_5);
    return;
}
