use crate::unk::block_1010_8000::pass1_1010_878c;
use crate::unk::block_1010_9000::pass1_1010_9f8c;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::sys_ops::set_err_mode_1010_8b14;
use crate::gui::msg_box::msg_box_op_1010_8bb4;

pub fn pass1_1010_866c(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u16,
) {
    let mut paVar1: *mut astruct_828;
    let mut cVar2: u8;
    let mut iVar3: i16;
    let mut bVar4: bool;

    if (param_5 < 0x28) {
        if ((param_5 < 0x25) && (param_5 != 0x23)) {
            if (0x23 < param_5) {
                return;
            }
            cVar2 = param_5;
            if ((cVar2 != '\x0f') && (cVar2 != '!')) {
                return;
            }
        }
    } else if (param_5 != 0x37) {
        if (param_5 < 0x38) {
            if (param_5 < 0x33) {
                return;
            }
            bVar4 = SBORROW2(param_5 - 0x33, 1);
            iVar3 = param_5 - 0x34;
        } else {
            //      if (param_5 == 0x49) goto LAB_1010_8691;
            if ((param_5 - 0x49) < 0x2a) {
                return;
            }
            bVar4 = SBORROW2(param_5 - 0x73, 0x5);
            iVar3 = param_5 - 0x78;
        }
        if (iVar3 != 0x0 && bVar4 == (iVar3 < 0x0)) {
            return;
        }
    } //
    // LAB_1010_8691:
    paVar1 = (param_5 * 0x4 + param_4);
    memcpy_op_1008_676e(paVar1, param_1, paVar1);
    return;
}

pub fn pass1_1010_86de(
    mut param_1: u16,
    mut param_2: u16,
    param_3: u8,
    param_4: *mut astruct_76,
) {
    let mut plVar1: *mut i32;
    let mut iVar2: i16;
    let mut bVar3: bool;
    let mut uVar4: u16;
    let mut lVar5: i32;
    let mut uVar6: u32;
    let mut lStack20: i32;
    let mut uStack10: u32;

    uVar6 = pass1_1008_4772(param_4);
    uVar4 = (uVar6 >> 0x10);
    uStack10 = 0;
    loop {
        plVar1 = (uVar6 + 0x8);
        if (*plVar1 == uStack10 || *plVar1 < uStack10) {
            return;
        }
        lVar5 = uStack10;
        pass1_1008_4544(param_4);
        iVar2 = lVar5;
        bVar3 = false;
        // for (lStack20 = 0; plVar1 = (uVar6 + 0x4), *plVar1 != lStack20 && lStack20 <= *plVar1;
        // lStack20 += 1)
        lStack20 = 0;
        plVar1 = uVar6 + 0x4;
        while *plVar1 != lStack20 && lStack20 <= *plVar1 {
            if (bVar3) {
                //
                // LAB_1010_86fc:
                if (bVar3) {
                    if (*(lStack20 + iVar2) == -1) {
                        *(lStack20 + iVar2) = param_3;
                        break;
                    }
                }
            } else {
                //        if (*(lStack20 + iVar2) == -1) goto LAB_1010_86fc;
                *(lStack20 + iVar2 - 1) = param_3;
                bVar3 = true;
            }
            lStack20 += 1;
        }
        uStack10 += 0x1;
    }
}

pub fn pass1_1010_887a(
    param_1: *mut *mut astruct_87,
    mut param_2: u32,
    mut param_3: i16,
    mut param_4: u32,
) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut bVar10: u8;
    let mut uVar11: u8;
    let mut local_26: [u8; 0x6] = [0; 0x6];
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut paStack28: *mut astruct_76;
    let mut uStack24: u32;
    let mut uStack20: u32;
    let mut uStack16: u32;
    let mut paStack12: *mut astruct_76;
    let mut paStack8: *mut astruct_76;
    let mut uStack4: u16;

    uStack4 = param_3 - 0xa;
    pass1_1010_878c(param_1, (uStack4 * 0xa + 0x3382));
    uVar7 = (param_1 >> 0x10);
    if ((param_1 + 0x67c) != 0) {
        iVar6 = uStack4 * 0xa;
        uVar1 = uStack4;
        pass1_1008_6562(
            (param_4 & 0xffff | uStack4 << 0x10),
            (param_1 + 0x67c),
            CONCAT22((iVar6 + 0x3388), (iVar6 + 0x338a)),
            (iVar6 + 0x3386),
        );
        paStack8 = CONCAT22(param_4, uVar1);
        uVar8 = (param_2 >> 0x10);
        paStack12 = (param_2 + 0x14);
        uStack16 = pass1_1008_4772(paStack12);
        uVar2 = param_4 & 0xffff0000;
        uStack20 = pass1_1008_4772(paStack8);
        paVar5 = (uVar2 & 0xffff0000 | uStack20 >> 0x10);
        uVar7 = (uStack20 >> 0x10);
        uVar2 = (uStack20 + 0x4);
        uVar9 = (uStack16 >> 0x10);
        iVar6 = uStack16;
        if (uVar2 < (iVar6 + 0x4)) {
            uVar2 = (iVar6 + 0x4);
        }
        uVar3 = (uStack20 + 0x8);
        if (uVar3 < (iVar6 + 0x8)) {
            uVar3 = (iVar6 + 0x8);
        }
        uVar1 = uVar3;
        uStack24 = uVar3 & 0xffff | uVar2 << 0x10;
        bVar10 = 0xff;
        uVar11 = 0;
        mem_op_1000_179c(0x1e, paVar5);
        uVar4 = paVar5 | uVar1;
        if (uVar4 == 0) {
            uVar1 = 0;
            uVar4 = 0;
        } else {
            struct_op_1008_6604(
                CONCAT22(paVar5, uVar1),
                uStack24,
                CONCAT13(uVar11, CONCAT12(bVar10, (uStack24 >> 0x10))),
            );
        }
        paStack28 = CONCAT22(uVar4, uVar1);
        pass1_1008_431c(CONCAT22(uVar4, uVar1), bVar10);
        uVar7 = (uStack16 >> 0x10);
        uStack30 = (uStack24 - (uStack16 + 0x4)) / 0x2;
        uStack32 = uStack24 - (uStack16 + 0x8);
        pass1_1008_3e54(CONCAT22(0x1050, local_26), 0x0, uStack32, uStack30);
        pass1_1008_4480(paStack28, CONCAT22(0x1050, local_26), paStack12);
        pass1_1008_3e76(CONCAT22(0x1050, local_26), 0x0, 0x0, 0x7);
        pass1_1008_4480(paStack28, CONCAT22(0x1050, local_26), paStack8);
        (param_3 * 0x4 + param_2) = paStack28;
    }
    return;
}


pub fn pass1_1010_89f0(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut puVar3: *mut u8;
    let mut in_EDX: u32;
    let mut paVar4: *mut Struct57;
    let mut iVar7: i16;
    let mut pcVar8: *mut c_char;
    let mut uStack22: u32;
    let mut pcStack12: *mut c_char;
    let mut uStack8: u16;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u16;

    uVar1 = (param_1 + 0x67c);
    uVar2 = (param_1 + 0x67e);
    uVar6 = (in_EDX >> 0x10);
    pcStack12 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1008_64a2(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(pcStack12);
    }
    pcVar8 = set_err_mode_1010_8b14(
        CONCAT22(param_2, param_1),
        ((param_1 + 0xe82) * 0x4 + 0x24be),
    );
    uVar2 = (pcVar8 >> 0x10);
    paVar4 = CONCAT22(uVar6, uVar2);
    uVar1 = pcVar8;
    iVar7 = (param_1 + 0xe82) * 0x4;
    if (((iVar7 + 0x24be) == uVar1) && ((iVar7 + 0x24c0) == uVar2)) {
        msg_box_op_1010_8bb4(param_1, param_2, pcVar8 & 0xffff | uVar2 << 0x10);
    }
    mem_op_1000_179c(0x8, paVar4);
    uVar2 = paVar4 | uVar1;
    paVar5 = (paVar4 & 0xffff0000 | uVar2);
    if (uVar2 == 0) {
        uVar1 = 0;
        paVar5 = null_mut();
    } else {
        file_1008_6414(paVar5, CONCAT22(paVar4, uVar1), pcVar8);
    }
    (param_1 + 0x67c) = uVar1;
    (param_1 + 0x67e) = paVar5;
    (param_1 + 0x680) = 0;
    if (((param_1 + 0x67e) | (param_1 + 0x67c)) != 0) {
        // for (uStack8 = 0x1; puVar3 = paVar5, uStack8 < 0xa; uStack8 += 1)
        uStack8 = 1;
        puVar3 = paVar5;
        while uStack8 < 0xa {
            iVar7 = uStack8 * 0xa;
            uVar1 = uStack8;
            pass1_1008_64c8(
                uStack8,
                puVar3,
                (param_1 + 0x67c),
                CONCAT22((iVar7 + 0x2558), (iVar7 + 0x255a)),
                (iVar7 + 0x2556),
            );
            uStack22 = CONCAT22(puVar3, uVar1);
            pass1_1010_86de(param_1, param_2, param_3, CONCAT22(puVar3, uVar1));
            paVar5 = ZEXT24(puVar3);
            (uStack8 * 0x4 + param_4) = uStack22;
            uStack8 += 1;
        }
    }
    return;
}

pub fn pass1_1010_8c78(param_1: *mut u16) {
    *param_1 = 0x8ee2;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_1d80(param_1);
    return;
}


pub fn unk_load_str_op_1010_8c96(
    param_1: *mut u8,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: u32,
) -> u32 {
    let mut uVar1: u32;
    let mut IVar2: i16;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut paVar9: *mut astruct_15;
    let mut uVar10: u32;
    let mut pcVar11: *mut c_char;
    WORD * valist;
    let mut in_buf_len_5: i16;
    let mut puVar12: *mut u8;
    let mut uStack46: u32;
    let mut local_10: u32;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut puStack8: *mut u8;
    let mut uStack6: u16;
    let mut uStack4: u16;

    uVar7 = (param_4 >> 0x10);
    iVar6 = param_4;
    uVar5 = (iVar6 + 0x6);
    uVar10 = uVar5;
    valist = param_3;
    in_buf_len_5 = (param_3 >> 0x10);
    if (uVar5 != 0) {
        uVar8 = (param_2 >> 0x10);
        if (uVar5 == 1) {
            uVar10 = param_4 & 0xffff;
            iVar4 = uVar10;
            match ((iVar6 + 0x4) - 1) {
                0x0 | 0x1 => {
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar6 + 0x8));
                    local_10 = (iVar4 + 0xc);
                    iStack12 = (iVar4 + 0x10);
                    iStack10 = iVar4;
                    puStack8 = param_1;
                    if (0x0 < iStack12) {
                        pcVar11 = load_string_1010_847e(_u16_1050_14cc, 0x437);
                        uStack4 = (pcVar11 >> 0x10);
                        uStack6 = SUB42(pcVar11, 0x0);
                        IVar2 = wsprintf16(
                            valist,
                            CONCAT22(uStack6, in_buf_len_5),
                            CONCAT22(iStack12, uStack4),
                        );
                        return CONCAT22(IVar2, uStack4);
                    }
                }
                // break;
                0x2 => {
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar6 + 0x8));
                    local_10 = (iVar4 + 0xc);
                    iStack12 = (iVar4 + 0x10);
                    if (0x0 < iStack12) {
                        iStack12 = 0;
                        paVar9 = struct_op_1030_73a8(CONCAT22(param_1, iVar4), &local_10, param_1);
                        uVar10 = pass1_1028_bb24(paVar9);
                        param_1 = (uVar10 >> 0x10);
                        iStack10 = uVar10;
                        puVar3 = &local_10;
                        puStack8 = param_1;
                        pass1_1030_627e(
                            puVar3,
                            param_1,
                            _PTR_LOOP_1050_5740,
                            CONCAT22(0x1050, puVar3),
                            uVar10,
                        );
                        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(param_1, puVar3));
                        uVar1 = (param_2 + 0xa);
                        pass1_1010_c3c2(
                            param_1,
                            uVar1,
                            (uVar1 >> 0x10),
                            0x0,
                            CONCAT22(param_1, puVar3),
                        );
                        uStack46 = CONCAT22(param_1, puVar3);
                        puVar12 = param_1;
                        pcVar11 = load_string_1010_847e(_u16_1050_14cc, 0x439);
                        uStack4 = (pcVar11 >> 0x10);
                        uStack6 = SUB42(pcVar11, 0x0);
                        wsprintf16(
                            valist,
                            CONCAT22(uStack6, in_buf_len_5),
                            CONCAT22(puVar3, uStack4),
                            puVar12,
                        );
                        // TODO: goto LAB_1010_8def;
                    }
                }
                // break;
                _ => {}
                // TODO: goto switchD_1010_8e11_caseD_4;
                0x4 | 0x7 | 0x8 | 0xa => {} // TODO: goto LAB_1010_8ea5;
            };
            uVar10 = ZEXT24(&local_10);
        } else {
            uVar10 = (uVar5 - 0x2);
            if (uVar5 - 0x2 == 0) {
                iVar4 = (iVar6 + 0x4);
                uVar5 = iVar4 - 0x4;
                if (uVar5 != 0) {
                    uVar5 = iVar4 - 0xc;
                    uVar10 = uVar5;
                    //          if (uVar5 != 0) goto LAB_1010_8ea5;
                }
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar6 + 0x8));
                uVar1 = (param_2 + 0xa);
                pass1_1010_c3c2(
                    param_1,
                    uVar1,
                    (uVar1 >> 0x10),
                    0x0,
                    CONCAT22(param_1, uVar5),
                );
                uStack46 = CONCAT22(param_1, uVar5);
                puVar12 = param_1;
                pcVar11 = load_string_1010_847e(_u16_1050_14cc, 0x43b);
                uStack4 = (pcVar11 >> 0x10);
                uStack6 = SUB42(pcVar11, 0x0);
                wsprintf16(
                    valist,
                    CONCAT22(uStack6, in_buf_len_5),
                    CONCAT22(uVar5, uStack4),
                    puVar12,
                ); //
                // LAB_1010_8def:
                fn_ptr_1000_17ce((uStack46 & 0xffff | ZEXT24(param_1) << 0x10));
                return CONCAT22(uStack46, param_1);
            }
        }
    } //
    // LAB_1010_8ea5:
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x3ff,
        valist,
        in_buf_len_5,
    );
    // switchD_1010_8e11_caseD_4:
    return CONCAT22(uVar10, param_1);
}

pub fn pass1_1010_8f78(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut StructD;
    let mut uVar4: *mut StructD;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.address_offset_field_0x0 = 0x9254;
    iVar4.address_offset_field_0x2 = 0x1010;
    puVar1 = iVar4.hfile_0x4;
    uVar2 = &iVar4.field_0x6;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    param_1.address_offset_field_0x0 = 0x389a;
    iVar4.address_offset_field_0x2 = 0x1008;
    return;
}

pub fn pass1_1010_8fba(mut param_1: u16, param_2: *mut astruct_411) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut iVar3: *mut astruct_411;
    let mut uVar3: u16;
    let mut uStack14: u32;
    let mut uStack10: u32;

    uVar3 = (param_2 >> 0x10);
    iVar3 = param_2;
    ppcVar1 = (*iVar3.field4_0x4 + 0x10);
    (**ppcVar1)();
    uStack10 = CONCAT22(extraout_DX, param_1);
    uStack14 = 0;
    loop {
        if (uStack10 <= uStack14) {
            return;
        }
        ppcVar1 = (*iVar3.field4_0x4 + 0x4);
        uVar2 = uStack10;
        (**ppcVar1)();
        if ((extraout_DX_00 | uVar2) != 0) {
            break;
        }
        uStack14 += 0x1;
    }
    ppcVar1 = (*iVar3.field4_0x4 + 0x8);
    (**ppcVar1)();
    return;
}

pub fn pass1_1010_9044(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x4) + 0x10);
    (**ppcVar1)();
    return;
}

pub fn fn_ptr_1010_905e(param_1: *mut astruct_169, mut param_2: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_169;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = &iVar4.field4_0x4;
    uVar2 = (&iVar4.field4_0x4 + 2);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    iVar4.field4_0x4 = param_2;
    return;
}

pub fn pass1_1010_9092(mut param_1: u16, mut param_2: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_EDX: *mut Struct57;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut in_stack_0000fe7c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffd4: u16;
    let mut uVar9: u32;
    let mut uStack14: u32;
    let mut uStack6: u32;
    let mut paVar6: *mut Struct57;

    uVar8 = (param_2 >> 0x10);
    iVar7 = param_2;
    uVar9 = (iVar7 + 0x4);
    ppcVar1 = ((iVar7 + 0x4) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(in_EDX, param_1);
    mem_op_1000_179c(0xc, in_EDX);
    uVar3 = in_EDX | param_1;
    paVar6 = (in_EDX & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
        param_1 = 0;
        paVar6 = null_mut();
    } else {
        pass1_1010_8ef2(
            paVar6,
            CONCAT22(in_EDX, param_1),
            in_stack_0000ffd4,
            in_stack_0000fe7c,
            in_stack_0000ffa0,
            in_stack_0000ffa6,
            in_stack_0000ffaa,
        );
    }
    uVar4 = SUB42(paVar6, 0x0);
    uStack14 = 0;
    loop {
        uVar3 = paVar6;
        if (uStack6 <= uStack14) {
            break;
        }
        ppcVar1 = ((iVar7 + 0x4) + 0x4);
        uVar2 = uStack6;
        (**ppcVar1)(0x1000, (iVar7 + 0x4), uStack14, uVar9);
        uVar5 = uVar3 | uVar2;
        paVar6 = uVar5;
        if (uVar5 != 0) {
            ppcVar1 = ((param_1 + 0x4) + 0xc);
            (**ppcVar1)(0x1000, (param_1 + 0x4), uVar2, uVar3);
        }
        uStack14 += 0x1;
    }
    return;
}


pub fn pass1_1010_9130(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u8,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_3 >> 0x10);
    pass1_1030_1d58((param_3 + 0x4));
    if ((param_2 | param_1).is_null() == false) {
        uVar1 = (param_3 + 0x8);
        pass1_1010_c3c2(
            (param_2 | param_1),
            uVar1,
            (uVar1 >> 0x10),
            param_4,
            CONCAT22(param_2, param_1),
        );
        return;
    }
    *param_4 = '\0';
    return;
}


pub fn struct_1010_9172(param_1: *mut astruct_249, mut param_2: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar4: *mut astruct_249;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar2 = iVar4.field4_0x4;
    uVar3 = iVar4.field5_0x6;
    paVar4 = (param_2 & 0xffff0000 | uVar3);
    if ((uVar3 | puVar2) != 0) {
        ppcVar1 = *puVar2;
        puVar2 = (**ppcVar1)();
    }
    mem_op_1000_179c(0x18, paVar4);
    uVar3 = paVar4 | puVar2;
    if (uVar3 == 0) {
        puVar2 = null_mut();
        uVar3 = 0;
    } else {
        puVar2 = struct_op_1030_1cd8(CONCAT22(paVar4, puVar2), 0x5, 0x5);
    }
    iVar4.field4_0x4 = puVar2;
    iVar4.field5_0x6 = uVar3;
    return;
}

pub fn pass1_1010_91cc(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut lVar3: i32;

    uVar2 = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 0x4) + 0x10);
    lVar3 = (**ppcVar1)();
    if (lVar3 != 0) {
        ppcVar1 = ((param_1 + 0x4) + 0x8);
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1010_9210(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x4) + 0xc);
    (**ppcVar1)();
    return;
}

pub fn pass1_1010_9258(param_1: *mut astruct_223) -> *mut astruct_223 {
    struct_1010_383a(param_1);
    param_1.field0_0x0 = 0x958e;
    (param_1 + 0x2) = 0x1010;
    return param_1;
}

pub fn pass1_1010_927a(param_1: *mut u16) {
    *param_1 = 0x958e;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_3880(param_1);
    return;
}

pub fn pass1_1010_92e6(param_1: *mut u16) {
    *param_1 = 0x9566;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_2db2(param_1);
    return;
}


pub fn pass1_1010_95f8(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field0_0x0 = 0xa1c8;
    iVar4.field1_0x2 = 0x1010;
    puVar1 = iVar4[0x1].field1_0x2;
    puVar2 = iVar4[0x1].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    puVar1 = iVar4[0x1].field3_0x6;
    uVar3 = (iVar4 + 0x2).field0_0x0;
    if ((uVar3 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    puVar1 = iVar4[0x2].field1_0x2;
    puVar2 = iVar4[0x2].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1010_9674(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0x12) = 0;
    return;
}

pub fn pass1_1010_96a8(mut param_1: u32, mut param_2: i16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x1e);
    *piVar1 = *piVar1 - param_2;
    if (*piVar1 < 0x0) {
        (param_1 + 0x1e) = 0;
    }
    return;
}

pub fn pass1_1010_96c2(mut param_1: u32) -> u16 {
    return (param_1 + 0x1e);
}


pub fn pass1_1010_96d0(param_1: *mut astruct_690) -> i16 {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut iVar3: *mut astruct_690;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut iStack8: i16;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field26_0x1a != 0) {
        if (0x0 < iVar3.field27_0x1c) {
            piVar1 = &iVar3.field27_0x1c;
            *piVar1 = *piVar1 - 0x1;
        }
        if ((iVar3.field27_0x1c == 0) && (iVar3.field28_0x1e != 0)) {
            iStack8 = 0x1;
            uVar4 = pass1_1030_8326();
            iVar2 = (uVar4 >> 0x10);
            if ((iVar2 != 0) || (0x32 < uVar4)) {
                iStack8 = 0x5;
            }
            if ((iVar2 != 0) || (0x3c < uVar4)) {
                iStack8 = 0xa;
            }
            if (iVar3.field28_0x1e < iStack8) {
                iStack8 = iVar3.field28_0x1e;
            }
            piVar1 = &iVar3.field28_0x1e;
            *piVar1 = *piVar1 - iStack8;
            if (*piVar1 < 0x0) {
                iVar3.field28_0x1e = 0;
            }
            if (0x0 < iVar3.field28_0x1e) {
                return iStack8;
            }
            return -0x1;
        }
    }
    return 0x0;
}

pub fn pass1_1010_9766(mut param_1: u16, mut param_2: u32) {
    let mut in_AX: i16;
    let mut uVar1: u16;

    uVar1 = (param_2 >> 0x10);
    (param_2 + 0x1a) = 0x1;
    pass1_1010_a0a0(param_1, param_2);
    pass1_1010_9f8c(param_2, 0x80);
    (param_2 + 0x1e) = in_AX * 0x32;
    return;
}

pub fn pass1_1010_9794(param_1: *mut astruct_250) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut pcVar3: *mut c_char;
    let mut uVar4: u16;
    let mut pchar_5: *mut c_char;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut in_EDX: u32;
    let mut uVar8: u32;
    let mut iVar9: *mut astruct_250;
    let mut uVar9: u16;
    let mut pcVar10: *mut c_char;
    let mut local_a: [u8; 0x8] = [0; 0x8];
    let mut paVar7: *mut Struct57;

    uVar9 = (param_1 >> 0x10);
    iVar9 = param_1;
    if (iVar9.field18_0x18 == 0) {
        iVar9.field18_0x18 = 0x1;
        puVar5 = iVar9.field11_0xe;
        uVar4 = (&iVar9.field11_0xe + 2);
        uVar6 = uVar4 | puVar5;
        paVar7 = (in_EDX & 0xffff0000 | uVar6);
        if (uVar6 != 0) {
            ppcVar2 = puVar5;
            (**ppcVar2)();
        }
        mem_op_1000_179c(0xc, paVar7);
        uVar4 = puVar5;
        uVar6 = paVar7 | uVar4;
        uVar8 = uVar6;
        if (uVar6 == 0) {
            uVar4 = 0;
            uVar8 = 0;
        } else {
            set_struct_1008_574a((puVar5 & 0xffff | paVar7 << 0x10));
        }
        iVar9.field11_0xe = uVar4;
        (&iVar9.field11_0xe + 0x2) = uVar8;
        pass1_1008_5784(CONCAT22(0x1050, local_a), iVar9.field10_0xa);
        loop {
            uVar4 = uVar8;
            pchar_5 = local_a;
            pass1_1008_5b12(CONCAT22(0x1050, pchar_5));
            uVar8 = (uVar4 | pchar_5);
            if ((uVar4 | pchar_5) == 0) {
                break;
            }
            iVar1 = (pchar_5 + 0x4);
            if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
                pcVar10 = iVar9.field10_0xa;
                (pcVar10 + 0xa) = 0;
                pcVar10 = iVar9.field10_0xa;
                ppcVar2 = (iVar9.field10_0xa + 0xc);
                (**ppcVar2)();
                pcVar3 = iVar9.field10_0xa;
                (pcVar3 + 0xa) = 0x1;
                local_a._4_4_ = 0;
                ppcVar2 = (*iVar9.field11_0xe + 0x4);
                (**ppcVar2)(0x1008, iVar9.field11_0xe, CONCAT22(uVar4, pchar_5), pcVar10);
            }
        }
    }
    return;
}


pub fn pass1_1010_988c(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut lVar8: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar6 + 0xe));
    loop {
        lVar8 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        uVar5 = (lVar8 >> 0x10);
        iVar3 = lVar8;
        if (lVar8 == 0) {
            return;
        }
        if !((iVar3 + 0x4) != param_2) {
            break;
        }
    }
    iVar4 = (iVar3 + 0x6) - 0x1;
    (iVar3 + 0x6) = iVar4;
    if ((iVar4 < 1) && (
        ppcVar1 = ((iVar6 + 0xe) + 0xc),
        (**ppcVar1)(0x1008, (iVar6 + 0xe), lVar8),
        uVar2 = (iVar6 + 0xe),
        (uVar2 + 0x8) == 0,
    )) {
        (iVar6 + 0x16) = 0x1;
    }
    return;
}
