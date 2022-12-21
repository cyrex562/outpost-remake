

// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub unsafe fn draw_op_1020_30be(struct_param_1: *mut astruct_762) {
    let mut is_iconic: bool;
    let mut iVar5: *mut astruct_762;
    let mut uVar5: *mut astruct_762;
    let mut pHVar1: *mut HDC16;
    let mut pHVar2: *mut HDC16;
    let mut rect_30: [RECT16; 2] = [RECT16::default(); 2];
    let mut hbrush_40: HGDIOBJ16;
    let mut hicon_38: HICON16;
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut IVar4: i16;
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (struct_param_1 >> 0x10);
    iVar5 = struct_param_1;
    local_24 = BeginPaint16(CONCAT22(0x1050, local_22), iVar5.field4_0x4);
    is_iconic = IsIconic16(iVar5.field4_0x4);
    if (is_iconic == 0) {
        pass1_1018_0a50(iVar5.field19_0x14);
        pHVar2 = &local_24;
        IVar4 = &DAT_1050_1050;
        fn_ptr_1 = (CONCAT22(0x1050, pHVar2) + 0x8);
        pHVar1 = pHVar2;
        (**fn_ptr_1)(0x1018, pHVar2, &DAT_1050_1050);
        uVar2 = iVar5.field19_0x14;
        if ((uVar2 + 0x84) == 1) {
            unk_draw_op_1020_320e(struct_param_1, local_24);
        }
        fn_ptr_1 = (CONCAT22(IVar4, pHVar2) + 0x4);
        (**fn_ptr_1)(
            0x1018,
            pHVar2,
            IVar4,
            0x0,
            0x0,
            &local_24,
            &DAT_1050_1050,
            pHVar1,
        );
        uVar1 = iVar5.field19_0x14;
        if ((uVar1 + 0x84) != 1) {
            unk_draw_op_1020_320e(struct_param_1, local_24);
        }
        draw_op_1020_3488(struct_param_1);
        fn_ptr_1 = (CONCAT22(IVar4, pHVar2) + 0xc);
        (**fn_ptr_1)(0x1018, pHVar2, IVar4, &local_24, &DAT_1050_1050);
    } else if (PTR_LOOP_1050_0010.is_null()) {
        fn_ptr_1 = (iVar5.field19_0x14 + 0x2c);
        hicon_38 = (**fn_ptr_1)(s_tile2_bmp_1050_1538);
        if (hicon_38 != 0) {
            hbrush_40 = GetStockObject16(BLACK_BRUSH);
            GetClientRect16(rect_30, &DAT_1050_1050);
            FillRect16(hbrush_40, &stack0xffc4, &DAT_1050_1050);
            DrawIcon16(hicon_38, 0x2, 0x2, local_24);
        }
    }
    EndPaint16(CONCAT22(0x1050, local_22), iVar5.field4_0x4);
    return;
}

// WARNING: Unable to use type for symbol uVar4
pub unsafe fn unk_draw_op_1020_320e(astruct762_param_1: *mut astruct_762, hdc_param_2: HDC16) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut obj: HPALETTE16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut struct_a: *mut astruct_762;
    let mut iVar7: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: *mut DEVMODEA;
    let mut device: *mut c_char;
    let mut driver: *mut c_char;
    let mut local_c: i16;
    let mut local_a: u32;
    let mut pHStack6: *mut HDC16;
    let mut hdc_var4: HDC16;
    let mut puVar2: *mut u32;
    let mut uVar4: u32;
    let mut uVar9: u8;
    let mut uVar10: u8;
    let mut uVar11: u16;

    hdc_var4 = hdc_param_2;
    uVar6 = (astruct762_param_1 >> 0x10);
    struct_a = astruct762_param_1;
    uVar4 = struct_a.field19_0x14;
    if ((uVar4 + 0x84) == 1) {
        uVar3 = struct_a.field19_0x14;
        uVar7 = (uVar3 >> 0x10);
        iVar7 = uVar3;
        puVar1 = (iVar7 + 0x24);
        driver = s_dib_1050_42c6;
        device = null_mut();
        uVar9 = '\0';
        uVar10 = '\0';
        uVar11 = 0;
        uVar8 = pass1_1008_4772((puVar1 & 0xffff | (iVar7 + 0x26) << 0x10));
        hdc_var4 = CreateDC16(
            uVar8,
            CONCAT22(uVar11, CONCAT11(uVar10, uVar9)),
            device,
            driver,
        );
        pHStack6 = &hdc_var4;
        ppcVar2 = (*puVar1 + 0x8);
        (**ppcVar2)(s_tile2_bmp_1050_1538);
        in_DX = extraout_DX;
    }
    pass1_1018_0d9a(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x6c),
        local_c,
        local_a,
        0x1,
        hdc_var4,
    );
    pass1_1018_1054(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x74),
        local_c,
        local_a,
        0x2,
        hdc_var4,
    );
    pass1_1018_1320(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x68),
        local_c,
        local_a,
        0x1,
        hdc_var4,
    );
    pass1_1018_15f6(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    if (local_c != 0) {
        uVar3 = struct_a.field19_0x14;
        draw_op_1020_33c0(
            in_DX,
            astruct762_param_1,
            *(uVar3 + 0x70),
            local_c,
            local_a,
            0x1,
            hdc_var4,
        );
    }
    pass1_1018_108c(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    if (local_c != 0) {
        uVar3 = struct_a.field19_0x14;
        draw_op_1020_33c0(
            in_DX,
            astruct762_param_1,
            *(uVar3 + 0x78),
            local_c,
            local_a,
            0x0,
            hdc_var4,
        );
    }
    uVar3 = struct_a.field19_0x14;
    if ((uVar3 + 0x84) == 1) {
        obj = SelectPalette16(0x0, pHStack6, hdc_var4);
        DeleteObject16(obj);
        DeleteDC16(hdc_var4);
    }
    return;
}
pub unsafe fn draw_op_1020_33c0(
    mut param_1: u16,
    mut param_2: u32,
    colorref_param_2: COLORREF,
    mut param_4: i16,
    mut param_5: u32,
    mut param_6: i16,
    hdc16_param_6: HDC16,
) {
    let mut pen_handle: HPEN16;
    let mut object_handle: HGDIOBJ16;
    let mut brush_handle: HBRUSH16;
    let mut obj_handle_2: HGDIOBJ16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_DX: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u16;
    let mut HVar7: HDC16;
    let mut iStack20: i16;
    let mut puStack14: *mut u16;
    let mut uVar5: u16;

    if (param_4 != 0) {
        HVar7 = hdc16_param_6;
        pen_handle = CreatePen16(colorref_param_2, 0x1, 0x0);
        object_handle = SelectObject16(pen_handle, HVar7);
        HVar7 = hdc16_param_6;
        brush_handle = CreateSolidBrush16(colorref_param_2);
        obj_handle_2 = SelectObject16(brush_handle, HVar7);
        puStack14 = param_5;
        for iStack20 in 0..param_4 {
            uVar6 = (param_2 >> 0x10);
            uVar1 = param_4;
            pass1_1020_3540(param_1, param_2, uVar6, param_6, puStack14);
            if (param_6 < 1) {
                uVar2 = 0x3;
            } else {
                uVar2 = 0x4;
            }
            uVar3 = param_1;
            draw_polygon_1020_3602(param_2, uVar6, uVar2, uVar1, param_1);
            fn_ptr_1000_17ce(CONCAT22(param_1, uVar1));
            puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x6));
            param_1 = uVar3;
        }
        obj_handle_2 = SelectObject16(obj_handle_2, hdc16_param_6);
        DeleteObject16(obj_handle_2);
        SelectObject16(object_handle, hdc16_param_6);
        DeleteObject16(pen_handle);
    }
    return;
}

// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar3
pub unsafe fn draw_op_1020_3488(param_1: *mut astruct_762) {
    let mut uVar6: u32;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut obj_handle_7: HGDIOBJ16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut in_stack_00000008: HDC16;
    let mut hdc: HDC16;
    let mut local_a: u32;
    let mut puStack6: *mut u16;
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut left: i16;
    let mut hdc16_ffe2: HDC16;

    uVar5 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x14);
    puStack6 = (uVar2 & 0xffff0000 | (uVar2 + 0x36));
    pass1_1008_3e94(
        puStack6,
        CONCAT22(0x1050, &local_a),
        CONCAT22(0x1050, &local_a + 0x2),
    );
    uVar4 = (local_a - 0x3) << 0x10;
    if ((local_a - 0x3) < 0x0) {
        uVar4 = 0;
    }
    uVar1 = local_a - 0x3;
    uVar6 = uVar1;
    if (uVar1 < 0x0) {
        uVar6 = 0;
    }
    local_a = uVar4 | uVar6;
    uVar3 = (param_1 + 0x14);
    hdc = in_stack_00000008;
    handle = CreatePen16(*(uVar3 + 0x64), 0x1, 0x0);
    handle_00 = SelectObject16(handle, hdc);
    obj_handle_7 = GetStockObject16(HOLLOW_BRUSH);
    obj_handle_7 = SelectObject16(obj_handle_7, hdc16_ffe2);
    left = (local_a >> 0x10);
    Rectangle16(local_a + 0x6, left + 0x6, local_a, left, in_stack_00000008);
    SelectObject16(handle_00, in_stack_00000008);
    SelectObject16(obj_handle_7, in_stack_00000008);
    DeleteObject16(handle);
    return;
}
pub unsafe fn pass1_1020_3540(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: *mut u16,
) {
    let mut iVar1: i16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut Struct279;
    let mut iStack18: i16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut local_6: i16;
    let mut local_4: i16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    pass1_1008_3e94(
        param_5,
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    if (param_4 == 0) {
        iStack12 = 0x3;
        iStack10 = 0x42a6;
    } else if (param_4 == 1) {
        iStack12 = 0x4;
        iStack10 = s_SITEICON_1050_428d + 0x9;
    } else {
        if (param_4 != 0x2) {
            return;
        }
        iStack12 = 0x4;
        iStack10 = 0x42b2;
    }
    iVar1 = iStack12 << 0x2;
    mem_op_1000_179c(iVar1, paVar2);
    for iStack18 in 0..iStack12 {
        iVar2 = (iStack18 * 0x4);
        (iVar2 + iVar1) = (iVar2 + iStack10) + local_4;
        (iVar2 + iVar1 + 0x2) = (iVar2 + iStack10 + 0x2) + local_6;
    }
    return;
}
pub unsafe fn draw_polygon_1020_3602(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    Polygon16(param_3, param_4, param_5);
    return;
}


pub unsafe fn struct_1020_3644(
    mut param_1: u32,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u16,
    mut param_6: u16,
) {
    let iVar2: *mut StructA;
    let mut in_buf_len_5: u16;
    let mut in_stack_0000fe52: u16;
    let mut in_stack_0000ff76: u16;
    let mut in_stack_0000ff7c: u16;
    let mut in_stack_0000ff80: u16;
    let mut iVar1: *mut Struct270;

    struct_1020_790e(&param_2.field0_0x0, NULL, param_3, param_4);
    in_buf_len_5 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field20_0x26 = 0x389a;
    iVar2[0x1].field21_0x28 = 0x1008;
    iVar2[0x1].field20_0x26 = 0x3aa8;
    iVar2[0x1].field21_0x28 = 0x1008;
    iVar2[0x1].field29_0x34 = 0;
    iVar2[0x1].field37_0x3e = 0;
    iVar2[0x1].field38_0x42 = 0;
    param_2.field0_0x0 = 0x3d08;
    iVar2.field1_0x2 = 0x1020;
    iVar2[0x1].field20_0x26 = 0x3d9c;
    iVar2[0x1].field21_0x28 = 0x1020;
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x3ff,
        &iVar2.field5_0xa,
        in_buf_len_5,
    );
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | ZEXT24(&iVar2.field60_0x5b)),
        s_VrMode_1050_42ca,
    );
    iVar2.field140_0xac = 0x44c00000;
    window_op_1020_38aa(
        param_1,
        (param_2 & 0xffff | in_buf_len_5 << 0x10),
        param_6,
        param_5,
        in_stack_0000ff7c,
        in_stack_0000ff80,
        in_stack_0000fe52,
        in_stack_0000ff76,
    );
    return;
}





pub unsafe fn window_op_1020_38aa(
    param_1: *mut Struct57,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
) {
    let mut hwnd: HWND16;
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut paVar3: *mut astruct_126;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut paVar9: *mut Struct57;
    let mut uVar11: u32;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut puVar13: *mut u32;
    let mut in_stack_0000fe08: u16;
    let mut in_stack_0000fe0c: u16;
    let mut in_stack_0000ff32: u16;
    let mut in_stack_0000ff36: u16;
    let mut in_stack_0000ff3a: u16;
    let mut uVar14: u16;
    let mut local_12: i16;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut local_a: [i16; 0x2] = [0; 2];
    let mut iStack6: i16;
    let mut iStack4: i16;
    let pstructa_hi: *mut StructA;
    let pstructa_1: *mut StructA;
    let mut paVar10: *mut Struct57;

    pstructa_1 = param_2;
    uVar14 = (param_2 >> 0x10);
    create_window_ex_1008_9760(param_2);
    puVar13 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x6),
        param_7,
        param_8,
        param_5,
        param_6,
    );
    paVar9 = (param_1 & 0xffff0000);
    pstructa_1[0x1].field25_0x2e = puVar13;
    iVar6 = (puVar13 >> 0x10);
    pstructa_1[0x1].field26_0x30 = iVar6;
    pstructa_1[0x1].field10_0x14 = pstructa_1[0x1].field25_0x2e;
    pstructa_1[0x1].field11_0x16 = iVar6;
    if (param_2.is_null()) {
        paVar3 = null_mut();
    } else {
        paVar9 = (paVar9 | uVar14);
        paVar3 = &pstructa_1[0x1].field20_0x26;
    }
    ppcVar1 = (*&pstructa_1[0x1].field25_0x2e + 0x4);
    (**ppcVar1)(0x1010, &pstructa_1[0x1].field25_0x2e, 0x0, paVar3, paVar9);
    pass1_1018_1a8e(paVar9, &pstructa_1[0x1].field25_0x2e);
    mem_op_1000_179c(0x20, paVar9);
    uVar7 = paVar9 | paVar3;
    paVar10 = (paVar9 & 0xffff0000 | uVar7);
    if (uVar7 == 0) {
        pstructa_1[0x1].field22_0x2a = 0;
    } else {
        unk_draw_op_1020_3da4(uVar7, param_3, CONCAT22(paVar9, paVar3), param_2);
        pstructa_1[0x1].field22_0x2a = paVar3;
        pstructa_1[0x1].field_0x2c = paVar10;
    }
    uVar5 = &pstructa_1[0x1].field22_0x2a;
    pstructa_1[0x1].field14_0x1c = uVar5;
    mem_op_1000_179c(0x42, paVar10);
    paVar4 = uVar5;
    uVar7 = paVar10 | paVar4;
    paVar9 = (paVar10 & 0xffff0000 | uVar7);
    if (uVar7 == 0) {
        pstructa_1[0x1].field_0x36 = 0;
    } else {
        pass1_1008_3bd6(
            paVar9,
            paVar4,
            paVar10,
            0x0,
            0xf1,
            0x0,
            0x1ac01ad,
            CONCAT22(pstructa_1.field4_0x8, 0xf4),
            param_4,
            in_stack_0000fe08,
            in_stack_0000fe0c,
            in_stack_0000ff32,
            in_stack_0000ff36,
            in_stack_0000ff3a,
        );
        pstructa_1[0x1].field_0x36 = paVar4;
        pstructa_1[0x1].field32_0x38 = paVar9;
    }
    uVar12 = 0x1000;
    mem_op_1000_179c(0x42, paVar9);
    uVar7 = paVar9 | paVar4;
    uVar11 = paVar9 & 0xffff0000 | uVar7;
    if (uVar7 == 0) {
        pstructa_1[0x1].field_0x3a = 0;
    } else {
        uVar12 = 0x1008;
        pass1_1008_3bd6(
            uVar11,
            paVar4,
            paVar9,
            0x0,
            0xf500f1,
            0x0,
            0x1ae01af,
            CONCAT22(pstructa_1.field4_0x8, 0xf5),
            param_4,
            in_stack_0000fe08,
            in_stack_0000fe0c,
            in_stack_0000ff32,
            in_stack_0000ff36,
            in_stack_0000ff3a,
        );
        pstructa_1[0x1].field_0x3a = paVar4;
        pstructa_1[0x1].field_0x3c = uVar11;
    }
    uVar5 = &pstructa_1[0x1].field25_0x2e;
    ppcVar1 = (*&pstructa_1[0x1].field25_0x2e + 0x10);
    (**ppcVar1)(uVar12, uVar5, (uVar5 >> 0x10));
    uVar12 = uVar11;
    uVar7 = paVar4.field1_0x2;
    paVar9 = (uVar11 & 0xffff0000 | uVar7);
    uVar7 = MoveWindow16(
        0x1,
        paVar4.field3_0x6,
        paVar4.field2_0x4,
        uVar7,
        paVar4.field0_0x0,
        pstructa_1.field4_0x8,
    );
    uVar12 = 0x1000;
    mem_op_1000_179c(0x8e, paVar9);
    uVar8 = paVar9 | uVar7;
    if (uVar8 == 0) {
        pstructa_1[0x1].field37_0x3e = 0;
    } else {
        uVar12 = SUB42(&PTR_LOOP_1050_1040, 0x0);
        get_sys_metrics_1040_7728(
            CONCAT22(paVar9, uVar7),
            0x1,
            0x0,
            0xfc0,
            pstructa_1.field4_0x8,
        );
        pstructa_1[0x1].field37_0x3e = uVar7;
        (&pstructa_1[0x1].field37_0x3e + 0x2) = uVar8;
    }
    uVar2 = pstructa_1[0x1].field37_0x3e;
    (uVar2 + 0x74) = 0x1;
    uVar2 = pstructa_1[0x1].field37_0x3e;
    ppcVar1 = (pstructa_1[0x1].field37_0x3e + 0x8);
    (**ppcVar1)(uVar12, uVar2, (uVar2 >> 0x10));
    if (((&pstructa_1[0x1].field37_0x3e + 0x2) | &pstructa_1[0x1].field37_0x3e) != 0) {
        uVar2 = pstructa_1[0x1].field37_0x3e;
        hwnd = (uVar2 + 0x6);
        GetWindowRect16(
            CONCAT13(0x10, CONCAT12(0x50, local_a)),
            pstructa_1.field4_0x8,
        );
        GetWindowRect16(CONCAT22(0x1050, &local_12), hwnd);
        iStack12 -= iStack16;
        iStack14 = iStack6 - local_a[0];
        local_12 = local_a[0];
        iStack16 = iStack4 + 0x3;
        SetWindowPos16(0x44, iStack12, iStack14, iStack16, local_a[0], 0x0, hwnd);
    }
    return;
}
pub unsafe fn destroy_window_1020_3b3e(param_1: *mut astruct_30) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut lVar4: i32;
    let mut puVar5: *mut u8;
    let mut paVar6: *mut astruct_30;
    let mut struct_5: *mut astruct_30;
    let mut struct_6: *mut astruct_30;
    let mut unaff_CS: u16;

    struct_6 = (param_1 >> 0x10);
    struct_5 = param_1;
    struct_5.field262_0x10e = 0;
    if (struct_5.field261_0x10a != 0) {
        lVar4 = struct_5.field261_0x10a;
        // 0x1538
        unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
        DestroyWindow16((lVar4 + 0x6));
    }
    puVar1 = struct_5.field246_0xf6;
    uVar2 = struct_5.field247_0xf8;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(unaff_CS, puVar1);
    }
    struct_5.field246_0xf6 = 0;
    if (struct_5.field248_0xfa != 0) {
        puVar5 = (struct_6 | struct_5);
        if (param_1.is_null()) {
            paVar6 = null_mut();
        } else {
            puVar5 = &struct_5.field_0xf2;
            paVar6 = struct_6;
        }
        pass1_1010_1ea6(struct_5.field248_0xfa, CONCAT22(paVar6, puVar5));
    }
    struct_5.field248_0xfa = 0;
    return;
}



pub unsafe fn pass1_1020_3c32(mut param_1: i16, mut param_2: u16, mut param_3: u16) {
    let mut cVar1: u8;
    let mut iVar2: i16;

    if (param_3 == 0xf5) {
        iVar2 = 0x1; //
                     // LAB_1020_3c52:
        pass1_1018_1b02((param_1 + 0xfa), iVar2);
        return;
    }
    if ((param_3 < 0xf6) && (cVar1 = param_3, cVar1 != '\0')) {
        if (cVar1 == '\x01' || cVar1 == '\x02') {
            return;
        }
        if (cVar1 == -0xc) {
            iVar2 = 0;
            // TODO: goto LAB_1020_3c52;
        }
    }
    pass1_1020_3c32(param_1, param_2, param_3);
    return;
}

pub unsafe fn pass1_1020_3c8c(mut param_1: u32, mut param_2: u32) {
    pt_in_rect_1018_1bda((param_1 + 0xfa), param_2, (param_2 >> 0x10));
    return;
}


// WARNING: Instruction at (ram,0x10203dab) overlaps instruction at (ram,0x10203da8)
//
// WARNING: Variable defined which should be unmapped: param_16



// WARNING: Unable to use type for symbol uVar4

pub unsafe fn unk_draw_op_1020_3da4(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_24,
    param_4: *mut StructA,
) {
    let mut puVar2: *mut u32;
    let mut pUVar3: *mut u32;
    let mut i16: i16;
    let mut white_pen_handle: HGDIOBJ16;
    let mut pHVar4: *mut HDC16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut pstruct24_1: *mut astruct_24;
    let mut pstruct_24_hi: u16;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut hdc: HDC16;
    let mut hdc_00: HDC16;
    let mut hdc_4: HDC16;
    let mut iVar9: *mut astruct_24;
    let mut uVar8: *mut astruct_24;
    let mut puVar1: *mut u32;
    let mut uVar4: *mut u32;
    let mut in_stack_0000ffea: u16;
    let mut fn_ptr_1: *mut *mut code;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    get_sys_metrics_1020_7c1a(param_3, param_4);
    pstruct_24_hi = (param_3 >> 0x10);
    pstruct24_1 = param_3;
    pstruct24_1.field17_0x14 = null_mut();
    param_3.offset_0x0 = 0x408a;
    pstruct24_1.base_0x2 = 0x1020;
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x6),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    hdc = (puVar6 >> 0x10);
    pstruct24_1.field17_0x14 = puVar6;
    *(&pstruct24_1.field17_0x14 + 0x2) = hdc;
    hdc_00 = 0;
    fn_ptr_1 = (*pstruct24_1.field17_0x14 + 0x4);
    (**fn_ptr_1)(0x1010, &pstruct24_1.field17_0x14, hdc, 0x0, param_3);
    hdc_4 = GetDC16(pstruct24_1.field2_0x4);
    i16 = SetMapMode16(0x1, hdc_4);
    pstruct24_1.field21_0x1e = i16;
    white_pen_handle = GetStockObject16(WHITE_BRUSH);
    white_pen_handle = SelectObject16(white_pen_handle, hdc);
    pstruct24_1.field18_0x18 = white_pen_handle;
    white_pen_handle = GetStockObject16(WHITE_PEN);
    white_pen_handle = SelectObject16(white_pen_handle, hdc_00);
    pstruct24_1.obj_handle_0x1a = white_pen_handle;
    uVar4 = pstruct24_1.field17_0x14;
    puVar2 = (uVar4 + 0x24);
    pHVar4 = &hdc_4;
    // just 0x1538
    fn_ptr_1 = (*puVar2 + 0x8);
    (**fn_ptr_1)(
        s_tile2_bmp_1050_1538,
        puVar2,
        (puVar2 >> 0x10),
        pHVar4,
        &DAT_1050_1050,
    );
    pstruct24_1.field20_0x1c = pHVar4;
    pUVar3 = pstruct24_1.field17_0x14;
    *(pUVar3 + 0x4c) = hdc_4;
    return;
}

// WARNING: Unable to use type for symbol uVar2
pub unsafe fn win_ui_palette_op_1020_3e84(param_1: *mut StructD) {
    let mut hdc: HDC16;
    let mut obj: HPALETTE16;
    let mut struct_v1: *mut StructD;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: i32;

    uVar1 = (param_1 >> 0x10);
    struct_v1 = param_1;
    param_1.address_offset_field_0x0 = 0x408a;
    struct_v1.address_offset_field_0x2 = 0x1020;
    pass1_1010_1ea6(struct_v1.field12_0x14, (param_1 & 0xffff | uVar1 << 0x10));
    uVar2 = struct_v1.field12_0x14;
    hdc = *(uVar2 + 0x4c);
    SelectObject16(struct_v1.field13_0x18, hdc);
    SelectObject16(struct_v1.field14_0x1a, hdc);
    obj = SelectPalette16(0x0, &struct_v1.field_0x1c, hdc);
    DeleteObject16(obj);
    SetMapMode16(&struct_v1.field_0x1e, hdc);
    param_1.address_offset_field_0x0 = 0x3ab0;
    struct_v1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    struct_v1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn mixed_draw_op_1020_3fa0(mut param_1: u32) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iStack56: i16;
    let mut uStack54: u32;
    let mut local_32: u32;
    let mut iStack46: i16;
    let mut uStack44: u32;
    let mut puStack40: *mut u32;
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    uVar3 = (iVar4 + 0x14);
    local_24 = *(uVar3 + 0x4c);
    uVar3 = (iVar4 + 0x14);
    puStack40 = (uVar3 + 0x24);
    uVar5 = puStack40;
    ppcVar2 = (*puStack40 + 0x4);
    (**ppcVar2)(
        s_tile2_bmp_1050_1538,
        uVar5,
        (puStack40 >> 0x10),
        0x0,
        &local_24,
        &DAT_1050_1050,
    );
    uVar3 = (iVar4 + 0x14);
    iStack46 = (uVar3 + 0x44);
    uVar3 = (iVar4 + 0x14);
    uStack44 = (uVar3 + 0x40);
    uVar1 = (iVar4 + 0x14);
    pass1_1008_3e94(
        (uVar1 & 0xffff0000 | (uVar1 + 0x3a)),
        CONCAT22(0x1050, &local_32),
        CONCAT22(0x1050, &local_32 + 0x2),
    );
    uStack54 = uStack44;
    for iStack56 in 0..iStack46 {
        draw_rect_1020_40ce(uStack54, local_32, (local_32 >> 0x10), local_24, uVar5);
        uStack54 = uStack54 & 0xffff0000 | (uStack54 + 0x18);
    }
    EndPaint16(CONCAT13(0x10, CONCAT12(0x50, local_22)), (iVar4 + 0x4));
    return;
}
