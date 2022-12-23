pub unsafe fn FUN_1020_3cb4() {
    return;
}

pub unsafe fn pass1_1020_3d08(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
    mut param_9: u16,
    mut param_15: u32,
    mut param_16: u16,
    mut param_17: u16,
    mut param_18: u16,
    mut param_19: u16,
) {
    let mut pcVar1: *mut c_char;
    let mut pbVar2: *mut u8;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut ppcVar5: *mut *mut code;
    let mut puVar6: *mut u16;
    let mut uVar7: u32;
    let mut puVar8: *mut u32;
    let mut hdc: HDC16;
    let pSVar9: *mut StructA;
    let mut bVar10: u8;
    let mut bVar12: u8;
    let mut iVar13: i16;
    let mut bVar17: u8;
    let mut cVar18: u8;
    let mut hdc_00: HDC16;
    let mut iVar14: i16;
    let mut HVar15: HGDIOBJ16;
    let mut puVar16: *mut u8;
    let mut uVar19: u16;
    let mut bVar20: u8;
    let mut bVar21: u8;
    let mut uVar22: u16;
    let mut in_register_0000000a: u16;
    let mut bVar24: u8;
    let mut pcVar25: *mut c_char;
    let mut uVar26: u16;
    let mut uVar27: u16;
    let mut bVar28: bool;
    let mut bVar29: bool;
    let mut puVar30: *mut u32;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut uStack30: u16;
    let mut pcStack4: *mut code;
    let mut bVar11: u8;
    let mut paVar23: *mut Struct57;

    pSVar9 = CONCAT22(param_19, param_18);
    bVar20 = param_2 + (param_1 >> 0x8) + param_10;
    *param_6 = 0x3c;
    paVar23 = CONCAT22(
        in_register_0000000a,
        CONCAT11(
            (param_2 >> 0x8) + '<' + (*(param_3 + param_5) < 0x20),
            bVar20,
        ),
    );
    pcStack4 = caseD_a7;
    iVar13 = 0x203d;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 & bVar20;
    pcVar25 = CONCAT11(0x79, param_5 - 0x37);
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    bVar10 = (param_6 + 2);
    bVar12 = 0x9 < (bVar10 & 0xf) | param_11;
    bVar11 = bVar10 + bVar12 * '\x06' & 0xf;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    bVar10 = 0x9 < bVar11 | bVar12;
    uVar19 = CONCAT11(
        (param_6 + 0x2 >> 0x8) + bVar12 + bVar10,
        bVar11 + bVar10 * '\x06',
    ) & 0xff0f;
    loop {
        pbVar2 = (param_3 + iVar13);
        bVar21 = paVar23;
        *pbVar2 = *pbVar2 | bVar21;
        bVar12 = (uVar19 - 1);
        bVar3 = 0x9 < (bVar12 & 0xf);
        bVar20 = bVar3 | bVar10;
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar21;
        bVar4 = 0x9 < (bVar12 + bVar20 * '\x06' & 0xf);
        bVar17 = (uVar19 - 0x1 >> 0x8) + bVar20 + (bVar4 | bVar20);
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar21;
        uVar19 = 0;
        bVar28 = &pcStack4 < (param_3 + iVar13);
        pbVar2 = (param_3 + iVar13 + 0x896);
        bVar24 = param_3;
        bVar29 = CARRY1(*pbVar2, bVar24) || CARRY1(*pbVar2 + bVar24, bVar28);
        *pbVar2 = *pbVar2 + bVar24 + bVar28;
        pbVar2 = (param_3 + iVar13 + 0x2038);
        bVar12 = *pbVar2;
        bVar11 = *pbVar2;
        *pbVar2 = bVar11 + bVar24 + bVar29;
        pcVar1 = (param_4 + iVar13);
        *pcVar1 = *pcVar1 + (paVar23 >> 0x8) + (CARRY1(bVar12, bVar24) || CARRY1(bVar11 + bVar24, bVar29));
        pcVar1 = (param_3 + iVar13 - 0x64);
        *pcVar1 = *pcVar1 + bVar17 + '\x01';
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar21;
        pcVar1 = pcVar25;
        pcVar25 = pcVar25 + 1;
        bVar28 = *pcVar1 != '\0';
        if (-*pcVar1 < '\0') {
            pcVar1 = (param_4 + 0x37);
            *pcVar1 = *pcVar1 + bVar24 + bVar28;
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar21;
            iVar13 += -0x1;
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar21;
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar21;
            uVar22 = paVar23 - 0x1;
            paVar23 = (paVar23 & 0xffff0000 | uVar22);
            pbVar2 = (param_3 + iVar13);
            bVar12 = uVar22;
            *pbVar2 = *pbVar2 | bVar12;
            if (*pbVar2 == 0) {
                pbVar2 = (param_3 + iVar13);
                *pbVar2 = *pbVar2 & bVar12;
                // code_r0x10203d96:
                pbVar2 = (param_3 + iVar13);
                bVar12 = paVar23;
                *pbVar2 = *pbVar2 | bVar12;
                pbVar2 = (param_3 + iVar13);
                *pbVar2 = *pbVar2 & bVar12;
                paVar23 = (paVar23 & 0xffff0000 | CONCAT11((paVar23 >> 0x8) * '\x02' + (uVar19 < 0x20), bVar12));
                pbVar2 = (param_3 + iVar13 + 1);
                *pbVar2 = *pbVar2 & bVar12;
                param_4 = &stack0xfff6;
                param_16 = pcStack4;
                param_17 = (pcStack4 >> 0x10);
                pSVar9 = param_15;
                // code_r0x10203db1:
                get_sys_metrics_1020_7c1a(CONCAT22(param_17, param_16), pSVar9);
                puVar6 = (param_4 + 0x6);
                uVar26 = (puVar6 >> 0x10);
                (puVar6 + 0x14) = 0;
                *puVar6 = 0x408a;
                (puVar6 + 0x2) = 0x1020;
                puVar30 = mixed_1010_20ba(
                    paVar23,
                    _u16_1050_0ed0,
                    CONCAT22(uStack30, 0x6),
                    in_stack_0000fe8a,
                    in_stack_0000ffae,
                    in_stack_0000ffb4,
                    in_stack_0000ffb8,
                );
                hdc = (puVar30 >> 0x10);
                uVar7 = (param_4 + 0x6);
                uVar26 = (uVar7 >> 0x10);
                iVar13 = uVar7;
                (iVar13 + 0x14) = puVar30;
                *(iVar13 + 0x16) = hdc;
                ppcVar5 = ((iVar13 + 0x14) + 0x4);
                (**ppcVar5)(0x1010, (iVar13 + 0x14), hdc, 0x0, iVar13, uVar26);
                uVar7 = (param_4 + 0x6);
                hdc_00 = GetDC16((uVar7 + 0x4));
                *(param_4 - 0x2) = hdc_00;
                iVar14 = SetMapMode16(0x1, hdc_00);
                uVar7 = (param_4 + 0x6);
                (uVar7 + 0x1e) = iVar14;
                HVar15 = GetStockObject16(0x0);
                HVar15 = SelectObject16(HVar15, hdc);
                uVar7 = (param_4 + 0x6);
                (uVar7 + 0x18) = HVar15;
                HVar15 = GetStockObject16(0x6);
                HVar15 = SelectObject16(HVar15, 0x0);
                uVar7 = (param_4 + 0x6);
                (uVar7 + 0x1a) = HVar15;
                uVar7 = (param_4 + 0x6);
                uVar7 = (uVar7 + 0x14);
                (param_4 - 0x6) = (uVar7 + 0x24);
                puVar16 = (param_4 - 0x2);
                puVar8 = (param_4 - 0x6);
                ppcVar5 = (*puVar8 + 0x8);
                (**ppcVar5)(
                    s_tile2_bmp_1050_1538,
                    puVar8,
                    (puVar8 >> 0x10),
                    puVar16,
                    &DAT_1050_1050,
                );
                uVar7 = (param_4 + 0x6);
                uVar27 = (uVar7 >> 0x10);
                iVar13 = uVar7;
                (iVar13 + 0x1c) = puVar16;
                uVar26 = (param_4 - 0x2);
                (param_4 - 0x14) = uVar26;
                uVar7 = (iVar13 + 0x14);
                (uVar7 + 0x4c) = uVar26;
                return;
            }
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 & bVar12;
            pcVar1 = (param_4 + iVar13 + 0x20);
            bVar11 = param_1 & 0x1f;
            cVar18 = *pcVar1;
            *pcVar1 = *pcVar1 >> bVar11;
            pcVar1 = (param_4 + iVar13 + 0x6a);
            *pcVar1 = *pcVar1 + param_1 + ((param_1 & 0x1f) != 0x0 && (cVar18 >> bVar11 - 0x1 & 1) != 0);
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar12;
            uVar19 = (param_3 + iVar13) * 0x10;
            pbVar2 = (pcVar25 + param_4 + 0x8);
            bVar12 = (uVar19 >> 0x8);
            uVar22 = uVar19 & 0xff | (bVar12 + *pbVar2) << 0x8;
            pcVar1 = (param_4 + iVar13 + 0x37);
            *pcVar1 = *pcVar1 + (param_3 >> 0x8) + CARRY1(bVar12, *pbVar2);
        } else {
            pcVar1 = (param_4 + iVar13);
            *pcVar1 = *pcVar1 + bVar28;
            uVar22 = (param_3 + iVar13) * 0x10;
            //      if ((POPCOUNT(*pcVar1) & 1) == 0) goto code_r0x10203db1;
        }
        pbVar2 = (param_3 + iVar13);
        bVar11 = paVar23;
        *pbVar2 = *pbVar2 | bVar11;
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar11;
        param_16 = (param_14 & 1) * 0x4000 | (param_13 & 1) * 0x200 | (param_12 & 1) * 0x100 | (*pbVar2 < '\0') * 0x80 | (*pbVar2 == 0) * 0x40 | (bVar4 | bVar3 | bVar10 & 1) * 0x10 | ((POPCOUNT(*pbVar2) & 1) == 0) * 0x4;
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar11;
        // TODO: bVar12 = in(0x79);
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar11;
        if (-0x1 < *pbVar2) {
            pSVar9 = CONCAT22(param_19, param_18);
            if ((bVar17 | *(param_4 - 1)) == 0) {
                param_16 = param_7;
                pSVar9 = CONCAT22(param_19, param_18);
            }
            // TODO: goto code_r0x10203db1;
        }
        pbVar2 = (param_4 + 0x89c);
        bVar28 = CARRY1(*pbVar2, bVar12);
        *pbVar2 = *pbVar2 + bVar12;
        bVar21 = bVar17 + bVar12;
        cVar18 = bVar21 + bVar28;
        uVar19 = CONCAT11(cVar18, bVar12);
        pcStack4._0_3_ = CONCAT21(
            (param_14 & 1) * 0x4000 | (SCARRY1(bVar17, bVar12) != SCARRY1(bVar21, bVar28)) * 0x800 | (param_13 & 1) * 0x200 | (param_12 & 1) * 0x100 | (cVar18 < '\0') * 0x80 | (cVar18 == '\0') * 0x40 | (bVar4 | bVar3 | bVar10 & 1) * 0x10 | ((POPCOUNT(cVar18) & 1) == 0) * 0x4 | (CARRY1(bVar17, bVar12) || CARRY1(bVar21, bVar28)),
            pcStack4._0_1_,
        );
        pcStack4 = (pcStack4 & 0xff000000 | pcStack4);
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar11;
        param_1 = uVar22 - 0x1;
        bVar10 = bVar4 | bVar20;
        //    if (param_1 == 0x0 || *pbVar2 == 0) goto code_r0x10203d96;
    }
}

pub unsafe fn validate_rect_1020_3f12(mut param_1: u32, mut param_2: i16) {
    let mut local_a: RECT16;
    let mut uStack6: u32;

    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    local_a = 0x8000e;
    uStack6 = 0x1100116;
    InvalidateRect16(0x0, &local_a, &DAT_1050_1050);
    local_a = 0xf10000;
    uStack6 = 0x1220030;
    ValidateRect16(&local_a, &DAT_1050_1050);
    local_a = 0xf100f5;
    uStack6 = 0x1220127;
    ValidateRect16(&local_a, &DAT_1050_1050);
    return;
}

pub unsafe fn pass1_1020_4064(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    win_ui_palette_op_1020_3e84(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_4092(param_1: *mut u16) -> *mut u16 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    pass1_1008_3e38(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x6) = 0;
    (iVar1 + 0x8) = 0;
    (iVar1 + 0xa) = 0x1;
    (iVar1 + 0xc) = 0;
    (iVar1 + 0xe) = 0;
    pass1_1008_3e38((param_1 & 0xffff0000 | (iVar1 + 0x10)));
    return param_1;
}

pub unsafe fn pass1_1020_434c(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    param_4: *mut u32,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: i16,
) {
    if (param_7 == 1) {
        pass1_1020_6184(CONCAT22(param_3, param_2), param_6);
        return;
    }
    if (param_7 == 0x2) {
        ui_op_1020_536e(
            param_1,
            CONCAT22(param_3, param_2),
            CONCAT22(param_5, param_4),
            param_6,
            0x2,
        );
        return;
    }
    pass1_1008_68ea(param_2, param_3, param_4, param_5, param_6, param_7);
    return;
}

pub unsafe fn post_msg_1020_4394(mut param_1: u32, mut param_2: u16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if ((iVar2 + 0x34) != 0) {
            PostMessage16(0x0, 0xf6, 0x111, HWND16_1050_0396);
            return;
        }
    } else if (param_2 < 0x11) {
        if (param_2 == '\x01') {
            (iVar2 + 0x18) = 0;
            return;
        }
        // if (param_2 == '\v') {
        //   uVar1 = (iVar2 + 0x2c);
        //   (uVar1 + 0xe) = (iVar2 -0xda);
        //   return;
        // }
    }
    return;
}

pub unsafe fn win_1020_43f6(
    param_1: *mut Struct57,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_160;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let struct_a_1: *mut StructA;

    struct_a_1 = param_2;
    uVar6 = (param_2 >> 0x10);
    create_window_ex_1008_9760(param_2);
    get_dc_1018_4db0(&struct_a_1[0x1].field25_0x2e, struct_a_1.field4_0x8);
    puVar5 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_7, 0x32),
        param_3,
        param_4,
        param_5,
        param_6,
    );
    paVar4 = (param_1 & 0xffff0000);
    struct_a_1[0x1].field38_0x42 = puVar5;
    (&struct_a_1[0x1].field38_0x42 + 0x2) = (puVar5 >> 0x10);
    if (param_2.is_null() == false) {
        paVar4 = (paVar4 | uVar6);
    }
    ppcVar1 = (struct_a_1[0x1].field38_0x42 + 0x4);
    paVar2 = (**ppcVar1)();
    mem_op_1000_179c(0x30, paVar4);
    uVar3 = paVar4 | paVar2;
    if (uVar3 == 0) {
        struct_a_1[0x1].field22_0x2a = 0;
    } else {
        pass1_1020_62e0(paVar2, paVar4, struct_a_1.field4_0x8);
        struct_a_1[0x1].field22_0x2a = paVar2;
        struct_a_1[0x1].field_0x2c = uVar3;
    }
    ui_op_1020_536e(uVar3, param_2, 0x0, -0x1, 0x3);
    return;
}


pub unsafe fn pass1_1020_44b0(param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xf6) != 0) {
        ppcVar1 = (*param_1 + 0x98);
        (**ppcVar1)();
        (iVar2 + 0x112) = 0;
        ppcVar1 = ((iVar2 + 0xf6) + 0x8);
        (**ppcVar1)();
    }
    return;
}

pub unsafe fn mixed_menu_op_1020_44ec(
    param_1: *mut astruct_850,
    mut param_2: u16,
    mut param_3: i16,
    param_4: HMENU16,
    mut param_5: u16,
    param_6: u8,
) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut UVar4: u16;
    let mut BVar5: bool;
    //   HMENlet mut HVar6: u16;
    let mut HVar6: HMENU16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut puVar9: *mut u8;
    let mut uVar10: u16;
    let mut in_register_0000000a: u16;
    let mut paVar11: *mut Struct57;
    let mut iVar9: *mut astruct_850;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut data: *mut c_char;
    let mut puVar15: *mut u32;
    let mut in_stack_0000fd70: u16;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000fe9e: u16;
    let mut w_flags: u16;
    let mut in_stack_0000fec8: u16;
    //   HMENlet mut w_item_id: u16;
    let mut w_ite_id: HMENU16;
    let mut uStack300: u16;
    let mut bStack293: u8;
    let mut uStack278: u16;
    let mut uStack268: u32;
    let mut local_108: [u32; 0x40] = [0; 0x40];
    let mut uStack8: u16;
    let mut puStack6: *mut u32;

    paVar11 = CONCAT22(in_register_0000000a, param_5);
    uVar13 = (param_1 >> 0x10);
    iVar9 = param_1;
    if (iVar9.hmenu_0x106 != 0) {
        if (iVar9.hmenu_0x106 == param_4) {
            puStack6 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000fec8, 0x3),
                in_stack_0000fd70,
                in_stack_0000fe94,
                in_stack_0000fe9a,
                in_stack_0000fe9e,
            );
            uVar2 = iVar9.field257_0x108;
            uStack8 = (uVar2 + 0x2e);
            uVar2 = iVar9.field257_0x108;
            uVar14 = (uVar2 >> 0x10);
            iVar12 = uVar2;
            uVar1 = (iVar12 + 0x42);
            puVar9 = (iVar12 + 0x44);
            bStack293 = (uVar1 >> 0x18);
            uVar7 = bStack293;
            if (bStack293 == 0) {
                uVar3 = pass1_1020_bd80(uStack8);
                unk_str_op_1000_3d3e(CONCAT22(0x1050, local_108), CONCAT22(puVar9, uVar3));
            } else {
                pass1_1030_8344(_u16_1050_5748, uVar1 & 0xffff | ZEXT24(puVar9) << 0x10);
                pass1_1010_c3c2(
                    puVar9,
                    puStack6,
                    (puStack6 >> 0x10),
                    CONCAT22(0x1050, local_108),
                    CONCAT22(puVar9, uVar7),
                );
            }
            ModifyMenu16(
                CONCAT22(0x1050, local_108),
                0x76,
                0x0,
                0x76,
                iVar9.hmenu_0x106,
            );
            UVar4 = GetMenuState16(0x0, 0x13c, iVar9.hmenu_0x106);
            if (UVar4 != 0xffff) {
                DeleteMenu16(0x0, 0x13c, iVar9.hmenu_0x106);
            }
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x20);
            if (BVar5 != 0) {
                data = load_string_1010_847e(_u16_1050_14cc, 0x74b);
                InsertMenu16(data, 0x13c, 0x400, 0xffff, iVar9.hmenu_0x106);
            }
            if ((s_VrMode_1050_42ca + 0x8 + uStack8 * 0x2) == 0) {
                HVar6 = iVar9.hmenu_0x106;
                w_flags = 0x1;
                UVar4 = 0x77;
                // TODO: goto LAB_1020_464c;
            }
            HVar6 = iVar9.hmenu_0x106;
            UVar4 = 0x77;
        } else {
            HVar6 = GetSubMenu16(0x1, iVar9.hmenu_0x106);
            //      if (HVar6 != param_4) goto LAB_1020_479e;
            EnableMenuItem16(0x1, 0x200, HVar6);
            uVar10 = paVar11;
            EnableMenuItem16(0x1, 0x201, HVar6);
            EnableMenuItem16(0x1, 0x202, HVar6);
            uVar2 = iVar9.field257_0x108;
            uVar8 = (uVar2 + 0x42);
            pass1_1030_8344(_u16_1050_5748, uVar8);
            uVar7 = uVar8;
            if ((uVar10 | uVar7) == 0) {
                return;
            }
            uVar2 = (uVar7 + 0x2e);
            uVar7 = (uVar7 + 0x30);
            uStack278 = uVar2;
            if ((uVar7 | uStack278) == 0) {
                return;
            }
            uStack268 = (uStack278 + 0x200);
            local_108[0] = struct_op_1030_73a8((uVar8 & 0xffff | uVar10 << 0x10), uStack268, uVar7);
            uVar13 = (local_108[0] >> 0x10);
            puStack6 = (local_108[0] + 0x1c);
            uVar7 = (local_108[0] + 0x1e);
            if ((uVar7 | puStack6) != 0) {
                uStack268 = puStack6 & 0xffff | uVar7 << 0x10;
            }
            uStack268 &= 0xff;
            if (uStack268 != 1) {
                return;
            }
            if ((uStack268 & 0xff0000) != 0) {
                return;
            }
            uVar3 = pass1_1030_6fa0(uVar8 & 0xffff | uVar10 << 0x10);
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar3, 0x3f);
            if (BVar5 != 0) {
                BVar5 = EnableMenuItem16(0x0, 0x201, HVar6);
            }
            if (((uVar8 & 0xffff) + 0x36) != 0) {
                BVar5 = EnableMenuItem16(0x0, 0x202, HVar6);
            }
            pass1_1030_69cc(BVar5, uStack268, uVar8 & 0xffff | uVar10 << 0x10);
            if (BVar5 == 0) {
                return;
            }
            UVar4 = 0x200;
        }
        w_flags = 0;
        // TODO: goto LAB_1020_464c;
    } //
    // LAB_1020_479e:
    iVar12 = param_3 - 0x1;
    if (iVar12 == 0) {
        pass1_1018_2504(0x0, paVar11);
        if (iVar12 == 0) {
            UVar4 = 0;
            EnableMenuItem16(0x401, 0x0, param_4);
            HVar6 = 0x1; //
            // LAB_1020_47e3:
            w_flags = 0x401;
            // TODO: goto LAB_1020_464c;
        }
        UVar4 = 0;
        EnableMenuItem16(0x400, 0x0, param_4);
        HVar6 = 0x1;
    } else if (param_3 == 0x2) {
        uVar3 = pass1_1020_64d4(iVar9.field246_0xf6, 0x2);
        if (uVar3 == 0) {
            EnableMenuItem16(0x401, 0x0, param_4);
            UVar4 = 0x401;
        } else {
            EnableMenuItem16(0x400, 0x0, param_4);
            UVar4 = 0x400;
        }
        HVar6 = 0x1;
        EnableMenuItem16(UVar4, 0x1, param_4);
        //    if ((PTR_LOOP_1050_0010.is_null() == false) || (iVar9.field255_0x102 == 0)) goto LAB_1020_47e3;
    } else {
        if (param_3 == 0x3) {
            HVar6 = 0;
            puVar15 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                0x2f,
                in_stack_0000fd70,
                in_stack_0000fe94,
                in_stack_0000fe9a,
                in_stack_0000fe9e,
            );
            uVar13 = (puVar15 >> 0x10);
            uVar1 = (puVar15 + 0x20);
            uVar7 = (puVar15 + 0x22);
            uStack300 = uVar1;
            if ((uVar7 | uStack300) != 0) {
                pass1_1030_8308(
                    &stack0xfecc,
                    uVar7,
                    _u16_1050_5748,
                    (_u16_1050_5748 >> 0x10),
                    CONCAT22(0x1050, &stack0xfecc),
                    CONCAT22(0x1050, &stack0xfec8),
                    uVar1 & 0xffff | uVar7 << 0x10,
                );
            }
            UVar4 = 0;
            loop {
                CheckMenuItem16(0x400, UVar4, param_4);
                w_item_id = param_4;
                EnableMenuItem16(0x401, UVar4, param_4);
                UVar4 += 0x1;
                if UVar4 >= 5 {
                    break;
                }
            }
            CheckMenuItem16(0x408, w_item_id, param_4);
            //   for (UVar4 = 0; UVar4 <= HVar6; UVar4 += 1)
            for UVar4 in 0..HVar6 {
                HVar6 = param_4;
                EnableMenuItem16(0x400, UVar4, param_4);
            }
            return;
        }
        if (param_3 != 0x4) {
            return;
        }
        UVar4 = 0x2;
        HVar6 = param_4;
    }
    w_flags = 0x400; //
    // LAB_1020_464c:
    EnableMenuItem16(w_flags, UVar4, HVar6);
    return;
}
