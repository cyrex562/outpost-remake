use crate::unk::block_1010_e000::{pass1_1010_e58a, pass1_1010_e8d0, pass1_1010_e8f6, pass1_1010_ed22};

pub unsafe fn pass1_1010_c58as(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_20,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut pstructd_1_lo: *mut StructD;
    let mut pstructd_lo_11: u16;
    let mut uVar1: u32;
    let mut pstructd_1_hi: *mut StructD;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut paVar6: *mut Struct57;
    let mut pUStack26: *mut StructD;
    let mut HStack18: HDC16;
    let mut UStack16: u16;
    let mut ppuStack14: *mut *mut u8;
    let mut pstructd32_1: *mut u16;
    let mut paVar5: *mut Struct57;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    uVar1 = param_1;
    mem_op_1000_179c(0x18, paVar4);
    pstructd_1_lo = uVar1;
    pstructd_1_hi = (paVar4 | pstructd_1_lo);
    paVar6 = (paVar4 & 0xffff0000);
    paVar5 = (paVar6 | ZEXT24(pstructd_1_hi));
    if (pstructd_1_hi.is_null()) {
        pstructd_1_lo = null_mut();
    } else {
        struct_1040_a598((uVar1 & 0xffff | paVar4 << 0x10));
        paVar6 = paVar5;
    }
    uVar2 = paVar6;
    pstructd32_1 = CONCAT22(uVar2, pstructd_1_lo);
    paVar4 = (paVar6 & 0xffff0000 | (uVar2 | pstructd_1_lo));
    if ((uVar2 | pstructd_1_lo) == 0) {
        return;
    }
    ppuStack14 = null_mut();
    HStack18 = 0;
    UStack16 = 0;
    match param_4 {
        0x5 => {
            ppuStack14 = &u16_1050_352c;
            HStack18 = 0xfa4;
            UStack16 = 0x30;
        }
        // break;
        _ => {
            if (pstructd32_1.is_null()) {
                return;
            }
            pass1_1040_a5d0(CONCAT22(uVar2, pstructd_1_lo));
            fn_ptr_1000_17ce(pstructd32_1);
            return;
        }
        0xa => {
            ppuStack14 = &u16_1050_358c;
            HStack18 = 0xfb3;
            UStack16 = 0x51;
        }
        // break;
        0xb => {
            ppuStack14 = &u16_1050_358c;
            HStack18 = 0xfb4;
            UStack16 = 0x51;
        }
        // break;
        0xc => {
            ppuStack14 = &u16_1050_362e;
            HStack18 = 0xfb6;
            UStack16 = 0x30;
        }
        // break;
        0x10 => {
            ppuStack14 = &PTR_DAT_1050_1805_1050_368e;
            HStack18 = 0xfc4;
            UStack16 = 0x3c;
        }
        // break;
        0x11 => {
            ppuStack14 = &PTR_DAT_1050_1805_1050_3706;
            HStack18 = 0xfc1;
            UStack16 = 0x4b;
        }
        // break;
        0x12 => {
            ppuStack14 = &u16_1050_379c;
            HStack18 = 0xfc5;
            UStack16 = 0x8;
        }
        // break;
        0x13 => {
            pass1_1010_debe(
                param_3,
                param_4,
                CONCAT22(uVar2, &pstructd_1_lo.field_0x10),
                CONCAT22(uVar2, &pstructd_1_lo.field7_0xc),
                param_5,
            );
            ppuStack14 = &u16_1050_37ac;
            HStack18 = 0xfc6;
            UStack16 = 0x1;
            paVar4 = paVar6;
        }
        // break;
        0x15 => {
            pstructd_1_lo.field_0x6 = param_5;
            pstructd_1_lo.field6_0xa = param_4;
        }
        // break;
        0x16 => {
            ppuStack14 = &u16_1050_37ae;
            HStack18 = 0x157;
            UStack16 = 0x4;
        }
        // break;
        0x17 => {
            ppuStack14 = &u16_1050_37b6;
            UStack16 = 0x2c;
            HStack18 = 0xfd8;
        }
    };
    if (ppuStack14.is_null() == false) {
        *pstructd32_1 = UStack16;
        pstructd_lo_11 = UStack16 * 0xa + 2;
        mem_op_1000_179c(pstructd_lo_11, paVar4);
        uVar3 = paVar4;
        pUStack26 = CONCAT22(uVar3, pstructd_lo_11);
        if ((uVar3 | pstructd_lo_11) == 0) {
            pstructd_1_lo.address_offset_field_0x2 = 0;
        } else {
            pUStack26.address_offset_field_0x0 = UStack16;
            pass1_1000_5586(
                0xa564,
                &PTR_LOOP_1050_1040,
                UStack16,
                0xa,
                pstructd_lo_11 + 0x2,
                uVar3,
            );
            pstructd_1_lo.address_offset_field_0x2 = pstructd_lo_11 + 2;
            pstructd_1_lo.hfile_0x4 = uVar3;
        }
        pstructd_1_lo.field_0x6 = param_5;
        pstructd_1_lo.field6_0xa = param_4;
        pstructd_1_lo.field11_0x12 = HStack18;
        pass1_1010_a50c(param_3, ppuStack14, CONCAT22(uVar2, pstructd_1_lo));
    }
    return;
}

pub unsafe fn pass1_1010_debe(
    mut param_1: u32,
    mut param_2: u16,
    param_3: *mut u16,
    param_4: *mut u32,
    mut param_5: u32,
) {
    let mut bVar1: u8;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_EDX: u32;
    let mut paVar6: *mut Struct57;
    let mut iVar7: i16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut paVar9: *mut astruct_15;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut uVar11: u16;
    let mut iStack34: i16;
    let mut uStack30: u16;
    let mut iStack26: i16;
    let mut iStack24: i16;
    let mut iStack22: i16;
    let mut iStack20: i16;

    *param_4 = 0;
    *param_3 = 0;
    paVar9 = struct_op_1030_73a8(param_5, 0x0, in_EDX);
    paVar6 = (in_EDX & 0xffff0000 | paVar9 >> 0x10);
    iVar4 = (paVar9 + 0x12);
    uVar5 = param_1;
    uVar11 = (param_1 >> 0x10);
    uVar2 = pass1_1010_b028(uVar5, uVar11, paVar9);
    puVar10 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x35),
        in_stack_0000fe84,
        in_stack_0000ffa8,
        in_stack_0000ffae,
        in_stack_0000ffb2,
    );
    paVar6 = (paVar6 & 0xffff0000 | puVar10 >> 0x10);
    iVar7 = param_4;
    uVar8 = (param_4 >> 0x10);
    if (param_2 == 0x13) {
        iStack34 = 0;
        while (iStack34 += 0x1, iStack34 < 0x43) {
            param_2 = pass1_1010_ac62(param_2, paVar6, uVar5, uVar11, iStack34);
            if (param_2 != 0) {
                *param_3 = *param_3 + 1;
            }
        }
        iVar4 = *param_3 * 0x2;
        mem_op_1000_179c(iVar4, paVar6);
        param_4 = iVar4;
        (iVar7 + 0x2) = paVar6;
        if ((paVar6 | param_4) != 0) {
            iStack34 = 0;
            //   for (uStack30 = 0; uVar2 = uStack30, *param_3 != uStack30 && uStack30 <= *param_3; uStack30 += 1)
            uStack30 = 0;
            uVar2 = uStack30;
            while *param_3 != uStack30 && uStack30 <= *param_3 {
                loop {
                    iStack34 += 0x1;
                    //          if (0x42 < iStack34) goto LAB_1010_e0d4;
                    uVar2 = pass1_1010_ac62(uVar2, paVar6, uVar5, uVar11, iStack34);
                    if uVar2 != 0 {
                        break;
                    }
                }
                (uStack30 * 0x2 + *param_4) = iStack34; //
                // LAB_1010_e0d4:
                uStack30 += 1;
            }
        }
    } else if (param_2 < 0x14) {
        if (param_2 == '\x06') {
            if (((iVar4 == 0x5) || (iVar4 == 0x6)) || (iVar4 == 0x8)) {
                uVar3 = puVar10 + 0x11e;
                if (uVar2 == 0xf) {
                    iStack20 = 0xf;
                    iStack22 = 0x13;
                } else if (uVar2 == 0xe) {
                    iStack22 = 0x4;
                    iStack20 = 0x1;
                } else {
                    iStack22 = 0xe;
                    iStack20 = 0x1;
                }
                iVar4 = pass1_1010_e128(
                    uVar5,
                    uVar11,
                    iStack22,
                    iStack20,
                    puVar10 & 0xffff0000 | uVar3,
                );
                *param_3 = iVar4 + 1;
                if (iVar4 + 0x1 != 0) {
                    iVar4 = *param_3 * 0x2;
                    mem_op_1000_179c(iVar4, paVar6);
                    param_4 = iVar4;
                    (iVar7 + 0x2) = paVar6;
                    iStack24 = 0;
                    //   for (iStack26 = iStack20; iStack26 <= iStack22; iStack26 += 1)
                    for iStack26 in iStack20..iStack22 {
                        if ((iStack26 * 0x2 + uVar3) != 0) {
                            (*param_4 + iStack24 * 0x2) = iStack26;
                            iStack24 += 0x1;
                        }
                    }
                    (*param_4 + iStack24 * 0x2) = 0x14;
                    return;
                }
            }
        } else {
            bVar1 = param_2 - 0x7;
            if ((bVar1 == 0) && ((iVar4 == 0x5 || (iVar4 == 0x6)) || (iVar4 == 0x8))) {
                uVar2 = pass1_1010_ac62(
                    param_2 & 0xff00 | bVar1,
                    (puVar10 >> 0x10),
                    uVar5,
                    uVar11,
                    0x7,
                );
                uVar5 = -(uVar2 == 0) + 0x10;
                *param_3 = uVar5;
                iVar4 = uVar5 * 0x2;
                mem_op_1000_179c(iVar4, paVar6);
                param_4 = iVar4;
                (iVar7 + 0x2) = paVar6;
                if ((paVar6 | param_4) == 0) {
                    *param_3 = 0;
                    return;
                }
                // for (iStack26 = 0; iStack26 < (-(uVar2 == 0) + 0xf); iStack26 += 1)
                iStack26 = 0;
                while iStack26 < (-(uVar2 == 0) + 0xf) {
                    (iStack26 * 0x2 + *param_4) = iStack26 + 1;
                    iStack26 += 1;
                }
                (iStack26 * 0x2 + *param_4) = 0x10;
                return;
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_e128(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
    mut param_5: u32,
) -> i16 {
    let mut iStack6: i16;
    let mut iStack4: i16;

    iStack4 = 0;
    //   for (iStack6 = param_4; iStack6 <= param_3; iStack6 += 1)
    for iStack6 in param_4..param_3 {
        if ((iStack6 * 0x2 + param_5) != 0) {
            iStack4 += 0x1;
        }
    }
    return iStack4;
}

pub unsafe fn pass1_1010_e15e(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut puStack10: *mut u32;

    pass1_1010_afde(param_3, 0xc);
    puStack10 = CONCAT22(param_2, param_1);
    ppcVar1 = (*puStack10 + 0x10);
    uVar2 = param_1;
    uVar6 = param_1;
    uVar7 = param_2;
    (**ppcVar1)();
    uStack14 = CONCAT22(extraout_DX, uVar2);
    //   for (uStack18 = 0; uStack18 < uStack14; uStack18 += 1)
    for uStack18 in 0..uStack14 {
        ppcVar1 = (*puStack10 + 0x4);
        uVar4 = uStack14;
        (**ppcVar1)(unaff_CS, param_1, param_2, uStack18, (uStack18 >> 0x10));
        uVar3 = uVar4;
        uVar5 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4 & 0xffff | extraout_DX_00 << 0x10);
        unaff_CS = 0x1030;
        pass1_1030_7c28(
            uVar3,
            uVar5,
            CONCAT13((uVar5 >> 0x8), CONCAT12(uVar5, uVar3)),
            0x23,
        );
    }
    if (puStack10.is_null() == false) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(unaff_CS, param_1, param_2, 0x1, uVar6, uVar7);
    }
    return;
}

pub unsafe fn pass1_1010_e1f4(mut param_1: u16, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u16;
    let mut in_AX: u16;
    let mut BVar2: bool;
    let mut pcVar3: *mut c_char;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut in_buf_len_5: i16;
    let mut uVar7: u32;

    in_buf_len_5 = (param_2 >> 0x10);
    iVar6 = param_2;
    *(iVar6 + 0x13c) = 0;
    uVar7 = struct_op_1030_73a8(param_3, in_AX, param_1);
    uVar5 = (uVar7 >> 0x10);
    uVar1 = (uVar7 + 0xc);
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0xc);
    if ((((((((BVar2 == 0) && (
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x14),
        BVar2 == 0,
    )) && (
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0xa),
        BVar2 == 0,
    )) && ((
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x15),
        BVar2 == 0x0 && (
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0xb),
            BVar2 == 0,
        ),
    ))) && (
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x16),
        BVar2 == 0,
    )) && ((
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x17),
        BVar2 == 0x0 && (
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x21),
            BVar2 == 0,
        ),
    ) && ((
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x1c),
        BVar2 == 0x0 && ((
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x1d),
            BVar2 == 0x0 && (
                BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x18),
                BVar2 == 0,
            ),
        ) && (
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x19),
            BVar2 == 0,
        )),
    )))) && ((
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x4),
        BVar2 == 0x0 && (
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x3),
            BVar2 == 0,
        ),
    ))) && ((
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x1e),
        BVar2 == 0x0 && ((
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x23),
            BVar2 == 0x0 && (
                BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x1b),
                BVar2 == 0,
            ),
        ) && ((
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x1f),
            BVar2 == 0x0 && ((
                BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 1),
                BVar2 == 0x0 && (
                    BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x2),
                    BVar2 == 0,
                ),
            ) && (
                BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x13),
                BVar2 == 0,
            )),
        ))),
    ) && ((
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x20),
        BVar2 == 0x0 && (
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0xe),
            BVar2 == 0,
        ),
    ) && (
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x10),
        BVar2 == 0,
    )))) {
        pcVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x12);
        if ((pcVar3.is_null()) && (
            pcVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x11),
            pcVar3.is_null(),
        )) {
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x6);
            if (BVar2 == 0) {
                BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x5);
                if (BVar2 == 0) {
                    pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x40);
                    // TODO: goto LAB_1010_e241;
                }
                uVar4 = pass1_1030_7f98(param_3);
                pcVar3 = string_op_1020_c222(uVar4);
            } else {
                uVar4 = pass1_1030_7f5a(param_3);
                pcVar3 = string_op_1020_c2f8(uVar4);
            }
        } else {
            pass1_1010_e58a(uVar5, param_2, uVar7);
        }
        unk_str_op_1000_3d3e(
            (param_2 & 0xffff0000 | (iVar6 + 0x13c)),
            CONCAT22(uVar5, pcVar3),
        );
    } else {
        //
        // LAB_1010_e241:
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            (iVar6 + 0x13c),
            in_buf_len_5,
        );
    }
    return;
}

pub unsafe fn pass1_1010_e58a(mut param_1: u16, mut param_2: u32, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut puVar3: *mut u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar7: i16;
    let mut in_buf_len_5: i16;
    let mut uVar8: u16;
    let mut puVar9: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffee: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    in_buf_len_5 = (param_2 >> 0x10);
    iVar7 = param_2;
    *(iVar7 + 0x13c) = 0;
    uVar8 = (param_3 >> 0x10);
    puVar3 = (param_3 + 0x20);
    uVar8 = (param_3 + 0xc);
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar8, 0x11);
    if (BVar2 == 0) {
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uVar8, 0x12);
        if (BVar2 == 0) {
            return;
        }
        puVar9 = mixed_1010_20ba(
            paVar6,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffee, 0x31),
            in_stack_0000fe96,
            in_stack_0000ffba,
            in_stack_0000ffc0,
            in_stack_0000ffc4,
        );
        uVar4 = (puVar9 >> 0x10);
        ppcVar1 = (*puVar9 + 0x14);
        (**ppcVar1)(0x1008, puVar9, uVar4, puVar3, puVar3 >> 0xf);
        uVar5 = uVar4 | puVar3;
    } else {
        puVar9 = mixed_1010_20ba(
            paVar6,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffee, 0x41),
            in_stack_0000fe96,
            in_stack_0000ffba,
            in_stack_0000ffc0,
            in_stack_0000ffc4,
        );
        uVar4 = (puVar9 >> 0x10);
        ppcVar1 = (*puVar9 + 0x14);
        (**ppcVar1)(0x1008, puVar9, uVar4, puVar3, puVar3 >> 0xf);
        uVar5 = uVar4 | puVar3;
    }
    if (uVar5 == 0) {
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            (iVar7 + 0x13c),
            in_buf_len_5,
        );
    } else {
        ppcVar1 = (*puVar3 + 0x14);
        (**ppcVar1)(0x1008, puVar3, uVar4);
        unk_str_op_1000_3d3e(
            (param_2 & 0xffff0000 | (iVar7 + 0x13c)),
            CONCAT22(uVar5, puVar3),
        );
    }
    return;
}

pub unsafe fn pass1_1010_e682(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut uVar1: u16;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut in_buf_len_5: u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut local_1e: u16;
    let mut uStack28: u16;
    let mut local_1a: u16;
    let mut uStack24: u16;
    let mut uStack22: u16;
    let mut uStack20: u32;
    let mut uStack16: u32;
    let mut uStack12: u32;
    let mut uStack8: u16;
    let mut uStack6: u32;

    in_buf_len_5 = (param_3 >> 0x10);
    uVar5 = param_3;
    *(uVar5 + 0x13c) = 0;
    uStack6 = struct_op_1030_73a8(param_4, param_1, param_2);
    uVar3 = (uStack6 >> 0x10);
    uStack8 = (uStack6 + 0xc);
    uVar1 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 1);
    if (((uVar1 == 0) && (
        uVar1 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x13),
        uVar1 == 0,
    )) && (
        uVar1 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x2),
        uVar1 == 0,
    )) {
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0xe);
        if (BVar2 != 0) {
            uVar6 = (uVar5 + 0x138);
            uVar4 = (uVar6 + 0x24);
            uStack16 = uVar4;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4);
            uStack12 = uVar4 & 0xffff | uVar3 << 0x10;
            uStack20 = (uVar4 + 0x1f6);
            uVar3 = (uStack20 + 0x1a8);
            uVar7 = 0x3947;
            uStack22 = uVar3; //
            // LAB_1010_e76d:
            sys_1000_3f9c(
                (param_3 & 0xffff0000 | (uVar5 + 0x13c)),
                CONCAT22(0x1050, uVar7),
                uVar3,
            );
            return;
        }
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x5);
        if ((BVar2 == 0) && (
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x6),
            BVar2 == 0,
        )) {
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x10);
            if (BVar2 == 0) {
                local_1e = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0xc);
                if ((local_1e == 0) && (
                    local_1e = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x14),
                    local_1e == 0,
                )) {
                    BVar2 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0xa);
                    if (BVar2 == 0) {
                        uVar3 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x1e);
                        if (uVar3 == 0) {
                            load_string_1010_84e0(
                                _u16_1050_14cc,
                                (_u16_1050_14cc >> 0x10),
                                0x3ff,
                                (uVar5 + 0x13c),
                                in_buf_len_5,
                            );
                            return;
                        }
                        pass1_1030_6ddc(param_4);
                        uVar7 = 0x395c;
                        local_1e = uVar3;
                        // TODO: goto LAB_1010_e76d;
                    }
                    uVar6 = pass1_1030_7c28(BVar2, uVar3, param_4, 0x21);
                    uStack28 = (uVar6 >> 0x10);
                    uVar1 = uVar6;
                    uVar7 = 0x3958;
                    local_1e = uVar1;
                } else {
                    pass1_1010_e8f6(local_1e, uVar3, uVar5, in_buf_len_5, param_4);
                    uStack28 = uVar3;
                    uVar6 = pass1_1030_7c28(local_1e, uVar3, CONCAT22(uVar3, local_1e), 0x23);
                    uStack24 = (uVar6 >> 0x10);
                    uVar1 = uVar6;
                    uVar7 = 0x3954;
                    local_1a = uVar1;
                }
            } else {
                uVar6 = pass1_1030_7c28(BVar2, uVar3, param_4, 0x1e);
                uStack28 = (uVar6 >> 0x10);
                uVar1 = uVar6;
                uVar7 = 0x3950;
                local_1e = uVar1;
            }
        } else {
            local_1e = 0;
            local_1a = 0;
            pass1_1010_e8d0(
                &local_1e,
                uVar5,
                in_buf_len_5,
                CONCAT22(0x1050, &local_1a),
                CONCAT22(0x1050, &local_1e),
                param_4,
            );
            uVar7 = 0x394a;
            uVar1 = local_1e;
        }
    } else {
        pass1_1010_e8f6(uVar1, uVar3, uVar5, in_buf_len_5, param_4);
        uStack12 = CONCAT22(uVar3, uVar1);
        pass1_1030_70f4(CONCAT22(uVar3, uVar1));
        uStack16 = CONCAT22(uVar3, uVar1);
        uVar7 = 0x3943;
    }
    sys_1000_3f9c(
        (param_3 & 0xffff0000 | (uVar5 + 0x13c)),
        CONCAT22(0x1050, uVar7),
        uVar1,
    );
    return;
}

pub unsafe fn pass1_1010_e8d0(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u16,
    param_5: *mut u16,
    mut param_6: u32,
) {
    pass1_1030_7064(param_6);
    *param_5 = param_1;
    pass1_1030_70ac(param_6);
    *param_4 = param_1;
    return;
}

pub unsafe fn pass1_1010_e99a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
    pass1_1010_a478(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1010_e9a6(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1010_a478(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1010_eb66(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut puVar4: *mut u16;
    let mut iVar5: *mut StructD;
    let mut uVar5: u16;
    let mut puStack14: *mut u16;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.address_offset_field_0x0 = 0x558;
    iVar5.address_offset_field_0x2 = 0x1018;
    iVar5.field6_0xa = 0x568;
    iVar5.field7_0xc = 0x1018;
    puVar1 = iVar5.field8_0xe;
    uVar2 = &iVar5.field_0x10;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1018_04f2(param_1);
    fn_ptr_1000_17ce(iVar5.field29_0x2c);
    if (param_1.is_null()) {
        puVar4 = null_mut();
        uVar5 = 0;
    } else {
        puVar4 = &iVar5.field6_0xa;
    }
    puStack14 = CONCAT22(uVar5, puVar4);
    *puStack14 = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_ebf8(
    mut param_1: u32,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + param_4 * 0x4 + 0x34) = param_2;
    (param_1 + param_4 * 0x4 + 0x36) = param_3;
    return;
}

pub unsafe fn pass1_1010_ec18(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) -> u32 {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5);
    return CONCAT22((param_1 + 0x12), (param_1 + 0x10));
}

pub unsafe fn pass1_1010_ec68(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x28) = param_2;
    pass1_1010_1f62((param_1 & 0xffff | uVar1 << 0x10), 0x13);
    return;
}

pub unsafe fn pass1_1010_ec84(mut param_1: u32) {
    let mut local_10e: [u8; 0x10c] = [0; 0x10c];

    pass1_1010_1f62(param_1, 0x14);
    pass1_1030_532e(CONCAT22(0x1050, local_10e), (param_1 + 0x20));
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_10e));
    return;
}

pub unsafe fn pass1_1010_ecc6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
    param_5: i32,
) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    pass1_1030_627e(param_1, param_2, _PTR_LOOP_1050_5740, param_4, param_5);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(param_2, param_1));
    uVar1 = (param_1 + 0x2e);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    if ((iVar2 + 0x200) == 0x8000001) {
        pass1_1010_ed22(param_3, (iVar2 + 0x4));
    }
    return;
}
