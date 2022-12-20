

// WARNING: Globals starting with '_' overlap smaller symbols at the same address



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pt_in_rect_1010_40f8(
    param_1: *mut Struct57,
    mut param_2: u32,
    param_3: *mut POINT16,
    mut param_4: u16,
) -> i16 {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut puVar12: *mut u32;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc2: u16;
    let mut puStack16: *mut u32;
    let mut iStack6: i16;
    let mut uStack4: u16;
    let mut uVar9: u32;

    iStack6 = 0;
    uStack4 = 0;
    loop {
        uVar11 = (param_2 >> 0x10);
        iVar10 = param_2;
        piVar1 = (iVar10 + 0x74);
        if (*piVar1 == iStack6 || *piVar1 < iStack6) {
            //
            // LAB_1010_413e:
            if ((uStack4 != 0) && (0x3 < PTR_LOOP_1050_3960)) {
                puVar12 = mixed_1010_20ba(
                    param_1,
                    _u16_1050_0ed0,
                    CONCAT22(param_4, iStack6 + 0xc),
                    in_stack_0000fe94,
                    in_stack_0000ffb8,
                    in_stack_0000ffbe,
                    in_stack_0000ffc2,
                );
                paVar8 = (param_1 & 0xffff0000 | puVar12 >> 0x10);
                uVar4 = pass1_1018_0afa(puVar12);
                if (uVar4 == 0) {
                    uVar11 = 0x1000;
                    uVar5 = uVar4;
                    mem_op_1000_179c(0xb4, paVar8);
                    uVar6 = paVar8 | uVar5;
                    uVar9 = paVar8 & 0xffff0000 | uVar6;
                    if (uVar6 == 0) {
                        iVar10 = 0;
                        uVar7 = 0;
                    } else {
                        uVar11 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                        iVar10 = string_1040_8520(
                            uVar9,
                            CONCAT22(paVar8, uVar5),
                            HWND16_1050_0396,
                            0x20030,
                            0x6450643,
                        );
                        uVar7 = uVar9;
                    }
                    puStack16 = CONCAT22(uVar7, iVar10);
                    ppcVar2 = (*puStack16 + 0x74);
                    (**ppcVar2)(uVar11, iVar10, uVar7);
                    pass1_1010_209e(_u16_1050_0ed0, iStack6 + 0xc);
                    uStack4 = uVar4;
                }
            }
            if (uStack4 != 0) {
                return iStack6;
            }
            return -0x1;
        }
        param_1 = (param_1 & 0xffff0000 | (iVar10 + 0x72));
        BVar3 = PtInRect16(
            *param_3,
            ((iStack6 * 0x10 + (iVar10 + 0x24)) * 0x8 + (iVar10 + 0x70)),
        );
        if (BVar3 != 0) {
            uStack4 = 0x1;
            // TODO: goto LAB_1010_413e;
        }
        iStack6 += 0x1;
    }
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_41d6(
    param_1: *mut Struct57,
    param_2: *mut astruct_243,
    mut param_3: u32,
) {
    let mut puVar1: *mut u16;
    let mut piVar2: *mut i16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut puVar6: *mut u8;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut uVar9: u32;
    let mut iVar9: *mut astruct_243;
    let mut iVar11: i16;
    let mut iVar10: *mut astruct_244;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut puVar14: *mut u32;
    let mut in_stack_0000fe6e: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff98: u16;
    let mut in_stack_0000ff9c: u16;
    let mut iStack50: i16;
    let mut local_30: i16;
    let mut local_2e: *mut c_char;
    let mut iStack42: i16;
    let mut pcStack40: *mut c_char;
    let mut pcStack34: *mut c_char;
    let mut pcStack30: *mut c_char;
    let mut iStack26: i16;
    let mut uStack24: u16;
    let mut iStack22: i16;
    let mut pcStack20: *mut c_char;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut iStack6: i16;
    let mut uStack4: u16;

    uVar12 = (param_2 >> 0x10);
    iVar9 = param_2;
    iVar9.field108_0x6c = param_3;
    puVar14 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2f),
        in_stack_0000fe6e,
        in_stack_0000ff92,
        in_stack_0000ff98,
        in_stack_0000ff9c,
    );
    uVar9 = param_1 & 0xffff0000;
    iStack6 = puVar14;
    uStack4 = (puVar14 >> 0x10);
    uStack10 = pass1_1010_ec40(iStack6, uStack4, iStack6, uStack4, iVar9.field108_0x6c);
    paVar8 = (uVar9 & 0xffff0000 | uStack10 >> 0x10);
    iVar9.field112_0x74 = (uStack10 + 0x22);
    if (&iVar9.field_0x70 != 0) {
        pcStack34 = *&iVar9.field_0x70;
        pcStack30 = pcStack34;
        fn_ptr_1000_17ce(pcStack34);
        iVar9.field_0x70 = 0;
    }
    iVar4 = iVar9.field112_0x74 << 0x7;
    mem_op_1000_179c(iVar4, paVar8);
    iVar9.field_0x70 = iVar4;
    iVar9.field111_0x72 = paVar8;
    pass1_1030_8344(_u16_1050_5748, iVar9.field108_0x6c);
    uStack14 = CONCAT22(paVar8, iVar4);
    uStack16 = (*(iVar4 + 0x10) == 0x9);
    iStack22 = (uStack10 + 0x22);
    uVar7 = iStack22 * 0x6;
    mem_op_1000_179c(uVar7, paVar8);
    uVar5 = paVar8;
    pcStack30 = CONCAT22(uVar5, uVar7);
    uVar9 = (uVar5 | uVar7);
    if ((uVar5 | uVar7) == 0) {
        pcStack20 = null_mut();
    } else {
        pass1_1000_5586(0x3e38, 0x1008, iStack22, 0x6, uVar7, uVar5);
        pcStack20 = pcStack30;
    }
    uStack24 = 0;
    loop {
        puVar6 = uVar9;
        uVar13 = (uStack10 >> 0x10);
        puVar1 = (uStack10 + 0x22);
        if (*puVar1 < uStack24 || *puVar1 == uStack24) {
            break;
        }
        uVar3 = (uStack10 + 0x24);
        uVar7 = uStack24;
        pass1_1028_e0a0(
            puVar6,
            _PTR_LOOP_1050_65e2,
            (uVar3 + uStack24 * 0x2) << 0x10,
        );
        pcStack34 = CONCAT22(puVar6, uVar7);
        uVar9 = pcStack20 >> 0x10;
        pass1_1008_3f62(
            (pcStack20 & 0xffff0000 | (uStack24 * 0x6 + pcStack20)),
            CONCAT22(puVar6, uVar7 + 0x8),
        );
        pcStack40 = pcStack34;
        pcStack30 = pcStack34;
        if (pcStack34.is_null() == false) {
            fn_ptr_1030_84d0(pcStack34);
            fn_ptr_1000_17ce(pcStack34);
        }
        uStack24 += 0x1;
    }
    //   for (iStack26 = 0; piVar2 = &iVar9.field112_0x74, *piVar2 != iStack26 && iStack26 <= *piVar2; iStack26 += 1)
    iStack26 = 0;
    piVar2 = &iVar9.field112_0x74;
    while *piVar2 != iStack26 && iStack26 <= *piVar2 {
        pass1_1008_3e94(
            (pcStack20 & 0xffff0000 | (iStack26 * 0x6 + pcStack20)),
            CONCAT22(0x1050, &local_2e),
            CONCAT22(0x1050, &local_30),
        );
        iStack42 = pass1_1000_49b2(local_2e);
        iStack42 /= 0x5;
        if (0xc < iStack42) {
            iStack42 = 0xc;
            iVar4 = pass1_1000_49b2(local_2e);
            local_2e = (local_2e & 0xffff0000 | ((local_2e / iVar4) * 0x3c));
        }
        iVar4 = pass1_1000_49b2(local_2e);
        uVar7 = iVar4 % 0x5;
        pcStack34 = (pcStack34 & 0xffff0000 | uVar7);
        if (local_2e < 0x0) {
            if (0x2 < uVar7) {
                uVar7 -= 0x5;
            }
            local_2e = (local_2e & 0xffff0000 | (local_2e + uVar7));
        } else if (uVar7 < 0x3) {
            local_2e = (local_2e & 0xffff0000 | (local_2e - uVar7));
        } else {
            local_2e = (local_2e & 0xffff0000 | (local_2e + (0x5 - uVar7)));
        }
        iStack50 = local_30 / 0x16;
        // for (iVar4 = 0; iVar4 < 0x10; iVar4 += 1)
        for iVar4 in 0..0x10 {
            if (0xf < iStack50) {
                iStack50 = 0;
            }
            if (((uStack16 != 0) < iStack50) && (iStack50 < 0x8)) {
                iVar11 = ((iStack42 * 0x10 + iStack50) * 0x2 + 0x11e0);
                iVar10 = ((iStack26 * 0x10 + iVar4) * 0x8);
                uVar3 = &iVar9.field_0x70;
                (iVar10 + uVar3) = iVar11 + 0x49;
                uVar3 = &iVar9.field_0x70;
                (iVar10 + uVar3 + 0x2) = local_2e + 0x49;
                uVar3 = &iVar9.field_0x70;
                (iVar10 + uVar3 + 0x4) = iVar11 + 0x4e;
                uVar3 = &iVar9.field_0x70;
                (iVar10 + uVar3 + 0x6) = local_2e + 0x4e;
            } else {
                iVar11 = (iStack26 * 0x10 + iVar4) * 0x8;
                uVar3 = &iVar9.field_0x70;
                (iVar11 + uVar3) = 0;
                uVar3 = &iVar9.field_0x70;
                (uVar3 + iVar11 + 0x2) = 0;
                uVar3 = &iVar9.field_0x70;
                (uVar3 + iVar11 + 0x4) = 0x1;
                uVar3 = &iVar9.field_0x70;
                (uVar3 + iVar11 + 0x6) = 0x1;
            }
            iStack50 += 0x1;
        }
        iStack26 += 1;
    }
    pcStack40 = pcStack20;
    local_2e = pcStack20;
    fn_ptr_1000_17ce(pcStack20);
    draw_1010_47ae(param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_451a(param_1: *mut u8, mut param_2: u32) -> u32 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut puVar3: *mut u32;
    let mut uVar4: u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000fff6: u16;

    puVar3 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff6, 0x2f),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    uVar1 = (puVar3 >> 0x10);
    uVar4 = pass1_1010_ec40(puVar3, uVar1, puVar3, uVar1, (param_2 + 0x6c));
    uVar2 = (uVar4 >> 0x10);
    return CONCAT22((uVar4 + 0x4), (uVar4 + 0x2));
}

pub unsafe fn pass1_1010_454a(mut param_1: u32) -> u32 {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar2 = (iVar1 + 0x24) * 0x4;
    return CONCAT22((iVar1 + iVar2 + 0x28), (iVar1 + iVar2 + 0x26));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_459e(param_1: *mut Struct27) {
    let mut paVar1: *mut Struct57;
    let mut in_EDX: u32;
    let mut paVar2: *mut Struct57;

    if (param_1.is_null()) {
        paVar1 = null_mut();
        paVar2 = (in_EDX & 0xffff0000);
    } else {
        paVar2 = (in_EDX & 0xffff0000 | param_1);
        paVar1 = (param_1 + 0x20);
    }
    pass1_1008_9262(
        paVar1,
        paVar2,
        _PTR_LOOP_1050_0388,
        (_PTR_LOOP_1050_0388 >> 0x10),
        0x1f4,
        CONCAT22(paVar2, paVar1),
    );
    (param_1 + 0x7e) = 0x1;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_45d6(param_1: i32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let mut iStack4: i16;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0x7e) != 0) {
        if (_PTR_LOOP_1050_0388 != 0) {
            if (param_1 == 0) {
                iVar4 = 0;
                uVar5 = 0;
            } else {
                iVar4 = iVar6 + 0x20;
                uVar5 = uVar7;
            }
            unaff_CS = 0x1008;
            pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x1f4, CONCAT22(uVar5, iVar4));
        }
        // for (iStack4 = 0; iStack4 < 0x10; iStack4 += 1)
        for iStack4 in 0..0x10 {
            if ((iVar6 + 0x24) != iStack4) {
                puVar1 = (iVar6 + 0x26 + iStack4 * 0x4);
                uVar2 = (iVar6 + 0x26 + iStack4 * 0x4 + 2);
                if ((uVar2 | puVar1) != 0) {
                    ppcVar3 = *puVar1;
                    (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
                }
                (iVar6 + iStack4 * 0x4 + 0x26) = 0;
            }
        }
        (iVar6 + 0x7e) = 0;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn draw_1010_47ae(mut param_1: u32) {
    let mut uStack4: u16;

    uStack4 = 0;
    loop {
        draw_op_1010_47d0(param_1, (param_1 >> 0x10), uStack4);
        uStack4 += 0x1;
        uStack4 >= 0x10;
    }
    return;
}

// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn draw_op_1010_47d0(param_1: *mut Struct27, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut puVar3: *mut u32;
    let mut iVar5: i16;
    let mut hpalette16_var6: HPALETTE16;
    let mut handle: HGDIOBJ16;
    let mut hgdiobj16_00: HGDIOBJ16;
    let mut iVar4: *mut astruct_797;
    let mut uVar5: u16;
    let mut extraout_DX: *mut u8;
    let mut puVar5: *mut u8;
    let mut iVar7: i16;
    let mut iVar8: *mut astruct_739;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut iStack32: i16;
    let mut hdc16_var_1: HDC16;
    let mut devmodea_init_data: u16;
    let mut uStack16: u16;
    let mut local_e: u16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut stock_obj_handle: HGDIOBJ16;
    let mut pen_handle: HPEN16;
    let mut uVar4: u32;
    let mut puVar2: *mut u32;
    let mut uVar13: u16;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut offset_1: u16;
    let mut base_addr_1: u16;
    let mut fn_ptr_1: *mut *mut code;

    pen_handle = CreatePen16(0x77d7fb, 0x1, 0x0);
    stock_obj_handle = GetStockObject16(HOLLOW_BRUSH);
    local_e = 0;
    uStack12 = 0;
    uStack10 = 0x1;
    uStack8 = 0x1;
    puVar3 = (&param_1.field_0x26 + param_3 * 0x4);
    puVar5 = (&param_1.field33_0x28)[param_3 * 0x2];
    if ((puVar5 | puVar3) != 0) {
        fn_ptr_1 = *puVar3;
        (**fn_ptr_1)(s_tile2_bmp_1050_1538, puVar3, puVar5, 1);
        puVar5 = extraout_DX;
    }
    iVar5 = param_3 + 0x105;
    pass1_1010_8170(puVar5, _u16_1050_14cc, iVar5);
    iVar8 = (param_3 * 0x4);
    (iVar8 + (&param_1.field_0x0 + 0x26)) = iVar5;
    (iVar8 + (&param_1.field_0x0 + 0x28)) = puVar5;
    base_addr_1 = &DAT_1050_1050;
    offset_1 = 0x1380;
    uVar16 = 0;
    uVar17 = 0;
    uVar13 = 0;
    uVar14 = '\0';
    uVar15 = '\0';
    uVar12 = pass1_1008_4772((&param_1.field_0x26 + iVar8));
    uVar12 = (uVar12 >> 0x10);
    devmodea_init_data = uVar12;
    uStack16 = uVar12;
    hdc16_var_1 = CreateDC16(
        (uVar12 & 0xffff | uVar12 << 0x10),
        CONCAT13(uVar15, CONCAT12(uVar14, uVar13)),
        CONCAT22(uVar17, uVar16),
        CONCAT22(base_addr_1, offset_1),
    );
    hpalette16_var6 = palette_op_1008_4e08(
        &hdc16_var_1,
        uVar12,
        (_PTR_LOOP_1050_4230 + 0xe),
        CONCAT22(0x1050, &hdc16_var_1),
    );
    handle = SelectObject16(pen_handle, hdc16_var_1);
    hgdiobj16_00 = SelectObject16(stock_obj_handle, hdc16_var_1);
    iStack32 = 0;
    loop {
        piVar1 = &param_1[0x1].field_0x1e;
        if (*piVar1 == iStack32 || *piVar1 < iStack32) {
            break;
        }
        iVar4 = ((iStack32 * 0x10 + param_3) * 0x8);
        uVar5 = pass1_1000_484c(
            CONCAT22(0x1050, &local_e),
            CONCAT22(&param_1[0x1].field_0x1c, iVar4 + param_1[0x1].field23_0x1a),
            0x8,
        );
        if (uVar5 != 0) {
            uVar4 = &param_1[0x1].field23_0x1a;
            uVar11 = (uVar4 >> 0x10);
            iVar7 = uVar4;
            Rectangle16(
                (iVar4 + iVar7 + 0x6),
                (iVar4 + iVar7 + 0x4),
                (iVar4 + iVar7 + 0x2),
                (iVar4 + iVar7),
                hdc16_var_1,
            );
        }
        iStack32 += 0x1;
    }
    hpalette16_var6 = SelectPalette16(0x0, hpalette16_var6, hdc16_var_1);
    DeleteObject16(hpalette16_var6);
    SelectObject16(handle, hdc16_var_1);
    SelectObject16(hgdiobj16_00, hdc16_var_1);
    DeleteDC16(hdc16_var_1);
    DeleteObject16(pen_handle);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_4a8a(
    mut param_1: u32,
    param_2: *mut Struct19,
    param_3: *mut Struct19,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
    mut param_9: u16,
) {
    let mut IVar1: i16;
    let mut uVar3: u16;
    let mut paVar2: *mut Struct57;
    let mut unaff_CS: u16;
    let mut paVar4: *mut Struct19;
    let mut puVar5: *mut u32;
    let mut uStack4: u16;

    uVar3 = (param_1 >> 0x10);
    paVar4 = struct_op_1010_1d48(CONCAT22(param_3, param_2), param_4);
    paVar2 = CONCAT22(uVar3, (paVar4 >> 0x10));
    param_2.field11_0x16 = 0;
    param_2.field_0x1a = 0;
    param_2.field16_0x1e = 0;
    param_2.field17_0x20 = 0x1;
    param_2.field18_0x22 = 0;
    param_2.field19_0x24 = 0;
    param_2.field20_0x26 = null_mut();
    param_2.field21_0x2a = 0;
    param_2.field22_0x2c = 0x1;
    param_2.field23_0x2e = 0;
    param_2.field24_0x30 = 0;
    // just 0x5024
    param_2.field25_0x32 = 0;
    // just 0x5024
    CONCAT22(param_3, param_2) = s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
    param_2.segment_0x2 = 0x1010;
    IVar1 = FUN_1010_830a(0x0, paVar2, unaff_CS, _u16_1050_14cc, 0x1b3);
    param_2.field11_0x16 = IVar1;
    param_2.field12_0x18 = paVar2;
    puVar5 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(uStack4, 0x3),
        param_6,
        param_7,
        param_8,
        param_9,
    );
    param_2.field20_0x26 = puVar5;
    (param_2.field20_0x26 + 0x2) = (puVar5 >> 0x10);
    pass1_1008_4772(&param_2.field11_0x16);
    param_2.field_0xe = 0x13c;
    param_2.horiz_res_0xa = 0;
    param_2.field8_0x10 = 0;
    param_2.ver_res_0xc = 0;
    return;
}
