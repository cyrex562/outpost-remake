use crate::defines::{astruct_642, POINT16};

pub fn draw_1018_623e(param_1: *mut astruct_604) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut HVar4: HDC16;
    let puVar5: *mut u16;
    let mut obj_handle: HPEN16;
    let mut HVar6: HGDIOBJ16;
    let mut obj_handle_00: HBRUSH16;
    let mut obj_handle_01: HGDIOBJ16;
    let mut h_gdi_obj: HPALETTE16;
    let mut extraout_DX: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: HWND16;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let mut uVar13: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 6];
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar9 = (param_1 >> 0x10);
    uVar8 = param_1;
    HVar4 = BeginPaint16(CONCAT22(unaff_SS, &local_22), (uVar8 + 4));
    local_24 = HVar4;
    pass1_1010_4c2c((uVar8 + 6));
    _local_28 = CONCAT22(extraout_DX, HVar4);
    puVar5 = &local_24;
    ppcVar1 = (*_local_28 + 8);
    (**ppcVar1)(0x1010, HVar4, extraout_DX, puVar5, unaff_SS);
    (uVar8 + 0x10) = puVar5;
    uVar2 = (uVar8 + 6);
    local_2a = (uVar2 + 0x30);
    uVar2 = (uVar8 + 6);
    local_2e = (uVar2 + 0x12);
    _local_32 = 0x140000;
    uVar10 = SUB42(&PTR_LOOP_1050_1008, 0);
    zero_list_1008_3e38(CONCAT22(unaff_SS, local_38));
    local_3a = 0;
    while (local_3a < local_2a) {
        uVar11 = process_struct_1008_4772((local_3a * 4 + local_2e));
        uVar7 = (uVar11 >> 0x10);
        iVar3 = uVar11;
        pass1_1018_642e(
            uVar8,
            uVar9,
            CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_32)),
            (iVar3 + 8),
        );
        pass1_1008_3e76(
            CONCAT22(unaff_SS, local_38),
            0,
            _local_32,
            (_local_32 >> 0x10),
        );
        pass1_1008_4480(
            _local_28,
            CONCAT22(unaff_SS, local_38),
            (local_2e + local_3a * 4),
        );
        uVar10 = 0x1010;
        process_struct_1010_4d5c(
            (uVar8 + 6),
            (iVar3 + 8) + _local_32,
            (iVar3 + 4) + local_30,
            _local_32,
            local_30,
            local_3a,
        );
        _local_32 =
            _local_32 & 0xffff | (local_30 + (-(local_3a == 0) & 5) + 0x14 + (iVar3 + 4)) << 0x10;
        local_3a = local_3a + 1;
    }
    ppcVar1 = (*_local_28 + 4);
    (**ppcVar1)(uVar10, _local_28, (_local_28 >> 0x10), 0, 0, 0xdc);
    obj_handle = CreatePen16(0x1000025, 1, 0);
    HVar6 = SelectObject16(obj_handle, local_24);
    obj_handle_00 = CreateSolidBrush16(0x1000025);
    obj_handle_01 = SelectObject16(obj_handle_00, local_24);
    draw_1018_6444(uVar8, (param_1 >> 0x10), local_24);
    uVar13 = local_24;
    uVar12 = pass1_1010_4dc8((uVar8 + 6));
    pass1_1018_6544(param_1, uVar12, uVar13);
    set_window_text_1018_6630(param_1);
    h_gdi_obj = SelectPalette16(0, (uVar8 + 0x10), local_24);
    DeleteObject16(h_gdi_obj);
    HVar6 = SelectObject16(HVar6, local_24);
    DeleteObject16(HVar6);
    HVar6 = SelectObject16(obj_handle_01, local_24);
    DeleteObject16(HVar6);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn polygon_1018_661c(param_1: u16, param_2: u16, in_count: u16, in_Paint16: *mut POINT16) {
    let mut hdc: HDC16;

    Polygon16(in_count, in_Paint16, hdc);
    return;
}

pub fn invalidate_rect_1018_5d32(param_1: u32, param_2: i32) {
    let mut hwnd: HWND16;

    hwnd = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 2) {
        return;
    }
    InvalidateRect16(0, (param_1 + 0x22), hwnd);
    return;
}

pub fn draw_1018_5d6c(param_1: u32) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: HWND16;
    let mut uVar6: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_22: PAINTSTRUCT16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar4 + 4));
    uVar3 = (iVar4 + 0x14);
    puVar1 = (uVar3 + 10);
    uVar6 = pass1_1008_9f48((iVar4 + 0x14));
    pass1_1008_5236((iVar4 + 0x18));
    pass1_1008_4480(puVar1, (param_1 & 0xffff0000 | (iVar4 + 0x1c)), uVar6);
    unsafe { ppcVar2 = (*puVar1 + 4) };
    ppcVar2(
        &PTR_LOOP_1050_1008,
        puVar1,
        (puVar1 >> 0x10),
        0,
        param_1 & 0xffff0000 | (iVar4 + 10),
    );
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_func_1018_4f18(param_1: u32, param_2: u16) {
    let mut uVar1: i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let puVar4: *mut u32;
    let puVar5: *mut u16;
    let mut iVar6: i32;
    let mut in_EAX: u32;
    let mut extraout_DX: i32;
    let extraout_DX_00: *mut astruct_199;
    let paVar7: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let mut extraout_DX_02: u16;
    let extraout_DX_03: *mut astruct_199;
    let mut extraout_DX_04: u16;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    mixed_fn_1010_830a(_g_struct_73_1050_14cc, param_2);
    _local_6 = (in_EAX & 0xffff | extraout_DX << 0x10);
    ppcVar3 = (*_local_6 + 0x14);
    (**ppcVar3)(0x1010, in_EAX, extraout_DX);
    puVar4 = in_EAX;
    _local_a = in_EAX & 0xffff | ZEXT24(extraout_DX_00) << 0x10;
    uVar9 = (param_1 >> 0x10);
    iVar8 = param_1;
    paVar7 = extraout_DX_00;
    if ((iVar8 + 0xe) != 0) {
        uVar1 = (iVar8 + 0x10);
        puVar4 = (iVar8 + 0xe);
        paVar7 = (uVar1 | puVar4);
        if (paVar7 != 0x0) {
            unsafe {
                ppcVar3 = *puVar4;
                (**ppcVar3)(0x10, puVar4, uVar1, 1);
            }
            paVar7 = extraout_DX_01;
        }
    }
    process_struct_1000_179c(0x14, paVar7);
    if ((paVar7 | puVar4) == 0) {
        puVar4 = 0x0;
        uVar10 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(paVar7, puVar4));
        uVar10 = extraout_DX_02;
    }
    (iVar8 + 0xe) = puVar4;
    (iVar8 + 0x10) = uVar10;
    pass1_1008_4d84((iVar8 + 0xe), _local_a);
    puVar5 = &local_12;
    paVar7 = extraout_DX_03;
    GetClientRect16(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, puVar5)),
        g_h_window,
    );
    uVar10 = 0x1000;
    process_struct_1000_179c(0x1e, paVar7);
    if ((paVar7 | puVar5) == 0) {
        (iVar8 + 10) = 0;
    } else {
        iVar6 = (local_c - local_10) + 1;
        uVar2 = (iVar8 + 0xe);
        uVar10 = SUB42(&PTR_LOOP_1050_1008, 0);
        pass1_1008_405c(
            puVar5,
            paVar7,
            uVar2,
            (uVar2 >> 0x10),
            iVar6,
            (local_e - local_12) + 1,
        );
        (iVar8 + 10) = iVar6;
        (iVar8 + 0xc) = extraout_DX_04;
    }
    if (_local_6 != 0x0) {
        ppcVar3 = *_local_6;
        (**ppcVar3)(uVar10, _local_6, (_local_6 >> 0x10), 1);
    }
    return;
}

pub fn pt_in_rect_1018_1bda(param_1: *mut astruct_318, param_2: u16, param_3: u16) {
    let puVar1: *mut u16;
    let mut iVar2: i32;
    let b_var3: bool;
    let paVar4: *mut astruct_199;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_e = 0;
    iVar2 = param_1;
    pass1_1008_3e94(
        (iVar2 + 0x3a),
        CONCAT22(unaff_SS, &local_14),
        CONCAT22(unaff_SS, &local_12),
    );
    _local_18 = CONCAT22(param_2, param_3);
    local_10 = 0;
    local_1a = 0;
    while (true) {
        uVar5 = (param_1 >> 0x10);
        puVar1 = (iVar2 + 0x44);
        let var = unsafe { *puVar1 };
        if (var == local_1a || var < local_1a) {
            return;
        }
        paVar4 = ((iVar2 + 0x40) + local_1a * 0x18);
        local_e = CONCAT22((iVar2 + 0x42), paVar4);
        pass1_1008_3e94(
            paVar4,
            CONCAT22(unaff_SS, &local_8),
            CONCAT22(unaff_SS, &local_a),
        );
        local_a = local_a + (local_12 - 6);
        local_6 = local_a + 0xc;
        local_8 = local_8 + (local_14 - 6);
        local_4 = local_8 + 0xc;
        b_var3 = PtInRect16(_local_18, &local_a);
        if (b_var3 != 0) {
            break;
        }
        local_1a = local_1a + 1;
    }
    pass1_1018_1eda(param_1, local_e);
    return;
}

pub fn get_gui_dc_1018_4db0(in_struct_1: *mut u8, in_win_handle: HWND16) -> *mut astruct_199 {
    let dev_ctx_handle: *mut astruct_199;
    let local_struct_1_hi: *mut void;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    (in_struct_1 + 0x16) = in_win_handle;
    dev_ctx_handle = GetDC16(in_win_handle);
    (in_struct_1 + 0x14) = dev_ctx_handle;
    return dev_ctx_handle;
}

pub fn create_drawing_dc_1018_4e04(
    param_1: *mut astruct_115,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) {
    let ppcVar1: fn();
    let mut dc_handle: u16;
    let mut hdc_ptr: u16;
    let local_BX_8: *mut astruct_115;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut func_ptr: u32;
    let mut inner_struct: u32;

    uVar2 = (param_1 >> 0x10);
    local_BX_8 = param_1;
    ppcVar1 = (param_1 + 0x14);
    (**ppcVar1)();
    uVar3 = process_struct_1008_4772(local_BX_8.field_0xa);
    pass1_1008_43cc(local_BX_8.field_0xa);
    dc_handle = CreateDC16(uVar3, (uVar3 >> 0x10), 0x0, 0x42340000);
    local_BX_8.dc_handle = dc_handle;
    hdc_ptr = &local_BX_8.dc_handle;
    inner_struct = local_BX_8.field_0xa;
    func_ptr = (local_BX_8.field_0xa + 8);
    (**func_ptr)(
        offset,
        inner_struct,
        (inner_struct >> 0x10),
        hdc_ptr,
        uVar2,
        param_1,
        param_2,
    );
    local_BX_8.field_0x1a = hdc_ptr;
    if ((WORD_1050_422e != 0) && (0x280 < param_4)) {
        local_10 = 0;
        while (local_10 < (u16_1050_4216 + 1)) {
            (&PTR_BYTE_1050_0009_1050_4172 + local_10 * 2) =
                (((&PTR_BYTE_1050_0009_1050_4172 + local_10 * 2) * (param_4 + 1)) / 0x280);
            local_10 = local_10 + 1;
        }
        local_10 = 0;
        while (local_10 < (WORD_1050_422c + 1)) {
            (&WORD_1050_419a + local_10 * 2) =
                (((&WORD_1050_419a + local_10 * 2) * (param_3 + 1)) / 0x1e0);
            local_10 = local_10 + 1;
        }
    }
    WORD_1050_422e = 0;
    return;
}

pub fn pt_in_rect_1010_40f8(param_1: u32, param_2: *mut POINT16) -> u16 {
    let pi_var1: *mut i32;
    let ppc_var2: fn();
    let b_var3: bool;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut i_var6: i32;
    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    let pu_var9: *mut u16;
    let mut in_stack_0000ffec: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    loop {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        pi_var1 = (iVar6 + 0x74);
        let pi_var1_val = unsafe { *pi_var1 };
        if (pi_var1_val == local_6 || pi_var1_val < local_6) {
            // LAB_1010_413e:
            if ((local_6._2_2_ != 0) && (3 < PTR_LOOP_1050_3960)) {
                ppVar8 = process_struct_1010_20ba(
                    _g_astruct_372_1050_0ed0,
                    CONCAT22(in_stack_0000ffec, local_6 + 0xc),
                );
                puVar9 = pass1_1018_0afa(ppVar8);
                if (puVar9 == 0) {
                    local_6 = local_6 & 0xffff;
                    uVar7 = 0x1000;
                    process_struct_1000_179c(0xb4, (puVar9 >> 0x10));
                    uVar5 = (puVar9 >> 0x10) | puVar9;
                    if (puVar9 == 0x0) {
                        uVar4 = 0;
                        uVar5 = 0;
                    } else {
                        uVar7 = SUB42(&PTR_LOOP_1050_1040, 0);
                        uVar4 = mixed_1040_8520(puVar9, g_h_window, 0x30, 2, 0x643, 0x645);
                    }
                    _local_10 = CONCAT22(uVar5, uVar4);
                    ppcVar2 = (*_local_10 + 0x74);
                    ppcVar2(uVar7, uVar4, uVar5);
                    pass1_1010_209e(_g_astruct_372_1050_0ed0, local_6 + 0xc);
                }
            }
            if (local_6._2_2_ != 0) {
                return local_6;
            }
            return 0xffff;
        }
        let param_2_val = unsafe { param_2 };
        b_var3 = PtInRect16(
            param_2_val,
            ((local_6 * 0x10 + (iVar6 + 0x24)) * 8 + (iVar6 + 0x70)),
        );
        if (b_var3 != 0) {
            local_6 = CONCAT22(1, local_6);
            // goto LAB_1010_413e;
        }
        local_6 = (local_6 + 1);
    }
}

pub fn draw_1010_47ae(param_1: u32) {
    let local_struct_1: *mut astruct_30;
    local_struct_1 = 0x0;
    while {
        draw_1010_47d0(param_1, local_struct_1);
        local_struct_1 = local_struct_1.field_0x1;
        local_struct_1 < 0x10
    } {}
}

pub fn draw_1010_47d0(param_1: u32, param_2: *mut astruct_30) {
    let pu_var1: *mut u16;
    let pu_var2: *mut u32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let ppc_var5: fn();
    let pa_var6: *mut astruct_30;
    let h_palette: *mut u16;
    let mut obj_handle: HGDIOBJ16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut i_var7: i32;
    let pu_var8: *mut u16;
    let mut h_gdi_obj: HPALETTE16;
    let mut extraout_d_x: u16;
    let local_b_x_49: *mut astruct_29;
    let mut i_var9: i32;
    let local_s_i_120: *mut astruct_31;
    let local_s_i_293: *mut astruct_28;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_s_s: u16;
    let mut u_var12: u32;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let mut u_var17: u16;
    let mut u_var18: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut h_dc: u16;
    let mut dev_mode: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut stock_object: u16;
    let mut pen: u16;

    pen = CreatePen16(0x77d7fb, 1, 0);
    stock_object = GetStockObject16(5);
    local_e = 0;
    local_c = 0;
    local_a = 1;
    local_8 = 1;
    u_var10 = (param_1 >> 0x10);
    local_b_x_49 = param_1;
    pu_var2 = (&local_b_x_49.field_0x26 + param_2 * 4);
    u_var3 = (&local_b_x_49.field_0x26 + param_2 * 4 + 2);
    if ((u_var3 | pu_var2) != 0) {
        unsafe {
            ppc_var5 = *pu_var2;
            (**ppc_var5)(offset, pu_var2, u_var3, 1);
        }
    }
    pa_var6 = param_2 + 1;
    win_gui_fn_1010_8170(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        pa_var6,
    );
    local_s_i_120 = (param_2 * 4);
    (local_b_x_49 + local_s_i_120 + 0x26) = pa_var6;
    (local_b_x_49 + local_s_i_120 + 0x28) = extraout_d_x;
    u_var18 = 0x1380;
    u_var11 = 0;
    u_var17 = 0;
    u_var13 = 0;
    u_var14 = 0;
    u_var15 = 0;
    u_var16 = 0;
    u_var12 = process_struct_1008_4772((local_s_i_120 + &local_b_x_49.field_0x26));
    local_10 = (u_var12 >> 0x10);
    dev_mode = u_var12;
    h_dc = CreateDC16(
        dev_mode,
        CONCAT13(u_var14, CONCAT12(u_var13, local_10)),
        CONCAT22(u_var11, CONCAT11(u_var16, u_var15)),
        CONCAT22(u_var18, u_var17),
    );
    u_var4 = (_PTR_LOOP_1050_4230 + 0xe);
    h_palette = &h_dc;
    realize_palette_1008_4e08(u_var4, (u_var4 >> 0x10), h_palette, unaff_s_s);
    obj_handle = SelectObject16(pen, h_dc);
    obj_handle_00 = SelectObject16(stock_object, h_dc);
    local_20 = 0;
    while (true) {
        pu_var1 = &local_b_x_49.field_0x74;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_20 || pu_var1_val < local_20) {
            break;
        }
        i_var7 = (&param_2.field_0x0 + local_20 * 0x10) * 8;
        pu_var8 = &local_e;
        pass1_fn_1000_484c(
            CONCAT22(unaff_s_s, pu_var8),
            CONCAT22(
                (&local_b_x_49.field_0x70 + 2),
                i_var7 + &local_b_x_49.field_0x70,
            ),
            8,
        );
        if (pu_var8 != 0x0) {
            u_var12 = local_b_x_49.field_0x70;
            u_var11 = (u_var12 >> 0x10);
            i_var9 = u_var12;
            local_s_i_293 = (i_var7 + i_var9);
            Rectangel16(
                local_s_i_293.field_0x6,
                local_s_i_293.field_0x4,
                local_s_i_293.field_0x2,
                (i_var9 + i_var7),
                h_dc,
            );
        }
        local_20 = local_20 + 1;
    }
    h_gdi_obj = SelectPalette16(0, h_palette, h_dc);
    DeleteObject16(h_gdi_obj);
    SelectObject16(obj_handle, h_dc);
    SelectObject16(obj_handle_00, h_dc);
    DeleteDC16(h_dc);
    DeleteObject16(pen);
    return;
}

pub fn pt_in_rect_1010_4e08(in_struct_1: *mut astruct_642, param_2: u16, param_3: u16) {
    let pi_var1: *mut i32;
    let mut b_var2: bool;
    let mut point_in_rect_result: bool;
    let local_struct_1: *mut astruct_642;
    let local_struct_1_hi: *mut astruct_642;
    let mut local_c: u32;
    let mut point_1: POINT16;
    let mut local_4: u16;

    point_1 = CONCAT22(param_2, param_3);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    &local_struct_1.field_0x22 = local_struct_1.field_0x20;
    b_var2 = false;
    local_struct_1.field_0x24 = 0;
    local_c = 0;
    loop {
        pi_var1 = local_struct_1.field_0x30;
        if (pi_var1 == local_c || pi_var1 < local_c) {
            // LAB_1010_4e67:
            if (local_c._2_2_ != 0) {
                local_struct_1.field_0x20 = local_c._2_2_;
            }
            if (bVar2) {
                return;
            }
            return;
        }
        point_in_rect_result = PtInRect16(Paint_1, (local_struct_1.field_0x1a + local_c * 8));
        if (point_in_rect_result != 0) {
            local_c = local_c << 0x10;
            bVar2 = true;
            // goto LAB_1010_4e67;
        }
        local_c = (local_c + 1);
    }
}

pub fn draw_1040_b372(param_1: u32, param_2: HWND16, uparam_3: i32, param_4: HDC16) {
    let mut iVar1: i32;
    let mut HVar2: HBRUSH16;
    let mut iVar3;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar4 = (param_1 >> 0x10);
    if ((param_1 + 0x8e) == 0) {
        HVar2 = CreateSolidBrush16(0);
        (param_1 + 0x8e) = HVar2;
    }
    if (_PTR_LOOP_1050_5efa == 0) {
        uVar5 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar4 = (uVar5 >> 0x10);
        iVar1 = uVar5;
        _PTR_LOOP_1050_5efa = CONCAT12(*(iVar1 + 0x94), CONCAT11(*(iVar1 + 0x95), *(iVar1 + 0x96)));
    }
    if (param_3 < 4) {
        // LAB_1040_b3ea:
        iVar3 = GetDlgCtrllID16(param_2);
        if (iVar3 == 0x14c) {
            uVar4 = 0xffff;
            uVar6 = 0;
            // goto LAB_1040_b41a;
        }
        if (iVar3 == 0x175) {
            uVar4 = 0xff;
            uVar6 = 0;
            // goto LAB_1040_b41a;
        }
    } else {
        if (param_3 != 4) {
            if ((param_3 == 4) || (1 < param_3 - 5)) {
                return;
            }
            // goto LAB_1040_b3ea;
        }
    }
    uVar4 = _PTR_LOOP_1050_5efa;
    uVar6 = (_PTR_LOOP_1050_5efa >> 0x10);
    // LAB_1040_b41a:
    SetTextColor16(CONCAT22(uVar6, uVar4), param_4);
    SetBkColor16(0x1000000, param_4);
    return;
}

pub fn draw_text_1040_c94a(param_1: *mut astruct_334, param_2: u16, param_3: u16) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000fff0: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    if (param_1.field_0x48 != 0) {
        ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff0, 3));
        uVar2 = param_1.field_0x42;
        uVar1 = (uVar2 + 0x12);
        iVar3 = pass1_1010_a5ca(ppVar4, (ppVar4 >> 0x10), uVar1);
        if (iVar3 == -1) {
            param_1.field_0x3c = 0xf9;
        } else {
            if (iVar3 == 0) {
                param_1.field_0x3c = 0x25;
                if ((uVar1 == 0x5b) || (uVar1 == 9)) {
                    param_1.field_0x3c = 0xfe;
                }
            } else {
                param_1.field_0x3c = 0xfb;
            }
        }
    }
    draw_text_1040_94fc(param_1, param_2, param_3);
    return;
}

pub fn draw_1040_c74c(param_1: *mut u32, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut h_gdi_obj: HGDIOBJ16;
    let mut h_gdi_obj_00: HPALETTE16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let unaff_SS: u8;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    uVar4 = (_PTR_LOOP_1050_4230 >> 0x10);
    local_6 = (_PTR_LOOP_1050_4230 + 0xe);
    local_8 = realize_palette_1008_4e08(
        local_6,
        (_PTR_LOOP_1050_4230 + 0x10),
        &param_2 + 2,
        unaff_SS,
    );
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    (iVar3 + 0x46) = 1;
    local_a = GetStockObject16(4);
    local_c = CreatePen16(0x1000002, 1, 0);
    local_e = SelectObject16(local_a, param_2._2_2_);
    local_10 = SelectObject16(local_c, param_2._2_2_);
    Rectangel16((iVar3 + 0x24), (iVar3 + 0x22), 0, 0, param_2._2_2_);
    MoveTo16(0, (iVar3 + 0x36) * 2 + (iVar3 + 0x2a), param_2._2_2_);
    LineTo16(
        (iVar3 + 0x24),
        (iVar3 + 0x36) * 2 + (iVar3 + 0x2a),
        param_2._2_2_,
    );
    SelectObject16(local_e, param_2._2_2_);
    h_gdi_obj = SelectObject16(local_10, param_2._2_2_);
    DeleteObject16(h_gdi_obj);
    uVar2 = unsafe { *param_1 };
    ppcVar1 = (uVar2 + 0x10);
    (**ppcVar1)(offset, iVar3, uVar4, param_2);
    ppcVar1 = (uVar2 + 0x14);
    (**ppcVar1)(offset, iVar3, (param_1 >> 0x10), param_2._2_2_);
    (iVar3 + 0x46) = 0;
    h_gdi_obj_00 = SelectPalette16(0, local_8, param_2._2_2_);
    DeleteObject16(h_gdi_obj_00);
    return;
}

pub fn draw_lines_1040_c38e(in_astruct_135: *mut astruct_135, in_hdc_2: u16) {
    let mut i_var1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let local_a_x_262: *mut astruct_137;
    let pi_var6: *mut u16;
    let mut extraout_d_x: u16;
    let local_astruct_135: *mut astruct_135;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut d_var9: u32;
    let mut curr_pos_10: u32;
    let mut local_1e: u16;
    let mut counter: u16;
    let mut local_18: u16;
    let mut x_coord: u16;
    let mut y_coord: u16;
    let mut local_10: u16;
    let local_e: *mut astruct_136;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (in_astruct_135 >> 0x10);
    local_astruct_135 = in_astruct_135;
    u_var2 = local_astruct_135.field_0x6;
    i_var1 = (u_var2 + 0x18);
    if ((i_var1 != 0) && (u_var2 = local_astruct_135.field_0x6, (u_var2 + 0x16) != 0)) {
        i_var4 = i_var1;
        pass1_1010_2ee2(local_astruct_135.field_0x6);
        counter = 0;
        while (counter < i_var1) {
            u_var2 = (counter * 4 + i_var4);
            i_var5 = u_var2;
            u_var2 = u_var2 & 0xffff0000;
            pi_var6 = (u_var2 | i_var5 + 0x1e);
            u_var8 = (u_var2 >> 0x10);
            i_var5 = (i_var5 + 0x24) / 2 + (i_var5 + 0x20);
            let pi_var6_val = unsafe { *pi_var6 };
            MoveTo16(i_var5, pi_var6_val, in_hdc_2);
            LineTo16(i_var5, pi_var6_val + -0xf, in_hdc_2);
            d_var9 = GetCurrentPosition16(in_hdc_2);
            y_coord = (d_var9 >> 0x10);
            x_coord = d_var9;
            if (counter == 0) {
                local_10 = x_coord;
                local_e = y_coord;
            }
            counter = counter + 1;
        }
        u_var2 = local_astruct_135.field_0x6;
        if ((u_var2 + 0x24) != 0) {
            local_e = local_e + -0xd;
        }
        u_var2 = local_astruct_135.field_0x6;
        if ((u_var2 + 0x26) != 0) {
            y_coord = y_coord + 0xd;
        }
        u_var2 = local_astruct_135.field_0x6;
        u_var3 = local_astruct_135.field_0x6;
        u_var2 = (u_var2 + (u_var3 + 0x16) * 4 + 0x26);
        local_a_x_262 = u_var2;
        local_a_x_262 = &local_a_x_262.field_0x1e;
        u_var2 = u_var2 & 0xffff0000;
        u_var7 = (u_var2 >> 0x10);
        MoveTo16(
            local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20,
            local_a_x_262.field_0x22 + (u_var2 | ZEXT24(local_a_x_262)),
            in_hdc_2,
        );
        LineTo16(
            local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20,
            local_10,
            in_hdc_2,
        );
        curr_pos_10 = GetCurrentPosition16(in_hdc_2);
        curr_pos_10._2_2_ = (curr_pos_10 >> 0x10);
        if (curr_pos_10._2_2_ < local_e) {
            local_e = curr_pos_10._2_2_;
        }
        if (y_coord < curr_pos_10._2_2_) {
            y_coord = curr_pos_10._2_2_;
        }
        MoveTo16(local_e, local_10, in_hdc_2);
        LineTo16(y_coord, x_coord, in_hdc_2);
    }
    return;
}

pub fn draw_lines_1040_c302(param_1: u32, in_hdc_2: HDC16) {
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut uVar5: i32;
    let local_AX_46: *mut astruct_138;
    let local_AX_100: *mut astruct_139;
    let mut uVar6: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    uVar4 = (param_1 + 6);
    iVar1 = (uVar4 + 0x16);
    if (1 < iVar1) {
        uVar2 = (param_1 + 6);
        uVar5 = uVar2 + 0x2a;
        uVar2 = uVar2 & 0xffff0000;
        uVar3 = (uVar2 | uVar5);
        local_AX_46 = uVar3;
        local_AX_46 = &local_AX_46.field_0x1e;
        uVar3 = uVar3 & 0xffff0000;
        uVar6 = (uVar3 >> 0x10);
        MoveTo16(
            local_AX_46.field_0x20 + local_AX_46.field_0x24,
            local_AX_46.field_0x22 / 2 + (uVar3 | ZEXT24(local_AX_46)),
            in_hdc_2,
        );
        uVar2 = (uVar5 + iVar1 * 4 + -4);
        local_AX_100 = uVar2;
        local_AX_100 = &local_AX_100.field_0x1e;
        uVar2 = uVar2 & 0xffff0000;
        uVar6 = (uVar2 >> 0x10);
        LineTo16(
            local_AX_100.field_0x20,
            local_AX_100.field_0x22 / 2 + (uVar2 | ZEXT24(local_AX_100)),
            in_hdc_2,
        );
    }
    return;
}

pub fn draw_1040_c226(param_1: *mut astruct_135) {
    let mut u_var1: u32;
    let mut obj_handle: HPEN16;
    let mut HVar2: HGDIOBJ16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: HWND16;
    let mut h_dc: u16;
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
    let mut local_22: PAINTSTRUCT16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    local_24 = BeginPaint16(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_22)),
        (iVar3 + 4),
    );
    local_26 = CreateSolidBrush16(0x8000);
    GetClientRect16(CONCAT22(unaff_SS, &local_32), (iVar3 + 4));
    uVar1 = (iVar3 + 6);
    local_28 = (uVar1 + 0x1a);
    uVar1 = (iVar3 + 6);
    local_2a = (uVar1 + 0x1c);
    local_30 = local_30 + 2;
    local_32 = local_28 - 10;
    local_2e = local_2e - 2;
    local_2c = local_2c - 2;
    FrameRect16(local_26, &local_32, unaff_SS);
    DeleteObject16(local_26);
    h_dc = local_24;
    obj_handle = CreatePen16(0x8080, 2, 0);
    HVar2 = SelectObject16(obj_handle, h_dc);
    draw_lines_1040_c302(iVar3, uVar4, local_24);
    draw_lines_1040_c38e(param_1, local_24);
    HVar2 = SelectObject16(HVar2, local_24);
    DeleteObject16(HVar2);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_1008_8288(param_1: HWND16, param_2: u32) {
    let mut HVar1: HGDIOBJ16;
    let mut obj_handle: HGDIOBJ16;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: HWND16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: PAINTSTRUCT16;
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
    let mut local_6: u16;
    let mut local_4: u16;

    local_4a = BeginPaint16(CONCAT22(unaff_SS, &local_3c), param_1);
    local_4 = 0;
    local_4c = CreatePen16(_PTR_LOOP_1050_0368, 1, 0);
    local_4e = CreatePen16(u32_1050_0364, 1, 0);
    local_50 = CreateSolidBrush16(u32_1050_0364);
    GetClientRect16(CONCAT22(unaff_SS, &local_58), param_1);
    local_3e = local_54;
    local_40 = local_52;
    local_42 = local_54 >> 1;
    local_44 = local_52 >> 1;
    local_46 = local_54 >> 2;
    local_48 = local_52 >> 2;
    HVar1 = GetStockObject16(7);
    HVar1 = SelectObject16(HVar1, local_5c);
    obj_handle = GetStockObject16(4);
    SelectObject16(obj_handle, HVar1);
    Rectangel16(local_52, local_54, local_56, local_58, local_4a);
    local_58 = local_4a;
    MoveTo16(local_44, 0, local_4a);
    _local_56 = CONCAT22(local_54, local_4a);
    local_58 = local_3e;
    local_5c = offset;
    LineTo16(local_44, local_3e, local_4a);
    uVar4 = (param_2 >> 0x10);
    if ((*(param_2 + 4) & 4) != 0) {
        local_4 = 1;
    }
    local_10 = local_42 + local_4;
    local_e = (local_48 + local_4) - 2;
    local_c = local_10 - 3;
    local_a = local_48 + local_4 + 1;
    local_8 = local_10 + 3;
    local_56 = local_4c;
    local_54 = local_4a;
    local_58 = offset;
    local_5a = 0x8395;
    local_6 = local_a;
    SelectObject16(local_4c, local_4a);
    if (local_4 == 0) {
        _local_56 = CONCAT22(local_4a, 1);
        local_58 = local_44 - 2;
        local_5c = 0x83b0;
        MoveTo16(local_58, 1, local_4a);
        local_52 = local_4a;
        _local_56 = 0x10001;
        local_58 = offset;
        local_5a = 0x83be;
        LineTo16(1, 1, local_4a);
        local_50 = local_4a;
        local_52 = local_3e - 1;
        local_54 = 1;
        local_56 = offset;
        local_58 = 0x83cd;
        LineTo16(1, local_52, local_4a);
    }
    local_4 = ((*(param_2 + 4) & 8) != 0);
    local_1c = local_42 + local_4;
    iVar3 = (local_40 - local_48) + local_4;
    local_1a = iVar3 + 1;
    local_18 = local_1c - 3;
    local_16 = iVar3 - 2;
    local_14 = local_1c + 3;
    local_12 = local_16;
    if (local_4 == 0) {
        local_4e = local_4a;
        local_50 = 1;
        local_52 = local_52 - 2;
        _local_56 = 0x15388429;
        MoveTo16(local_52, 1, local_4a);
        local_4c = local_4a;
        local_4e = 1;
        uVar2 = local_44 + 1;
        local_52 = offset;
        local_54 = 0x843a;
        local_50 = uVar2;
        LineTo16(uVar2, 1, local_4a);
        local_4c = local_3e - 1;
        local_50 = offset;
        local_52 = 0x8448;
        local_4e = uVar2;
        LineTo16(uVar2, local_4c, local_4a);
    }
    local_4e = local_4a;
    local_50 = local_4a;
    local_52 = offset;
    _local_56 = CONCAT22(0x8453, local_56);
    SelectObject16(local_4a, local_4a);
    local_4e = local_4a;
    local_52 = offset;
    _local_56 = CONCAT22(0x845e, local_56);
    SelectObject16(local_50, local_4a);
    local_4e = local_4a;
    local_52 = &local_10;
    _local_56 = 0x31538;
    local_58 = 0x846d;
    Polygon16(3, local_52, unaff_SS);
    local_4e = local_4a;
    local_52 = &local_1c;
    _local_56 = 0x31538;
    local_58 = 0x847c;
    Polygon16(3, local_52, unaff_SS);
    local_4e = local_4a;
    local_50 = local_5c;
    local_52 = offset;
    _local_56 = CONCAT22(0x8487, local_56);
    SelectObject16(local_5c, local_4a);
    local_4e = local_4a;
    local_50 = local_5a;
    local_52 = offset;
    _local_56 = CONCAT22(0x8492, local_56);
    SelectObject16(local_5a, local_4a);
    local_4e = local_4c;
    local_50 = offset;
    local_52 = 0x849a;
    DeleteObject16(local_4c);
    uVar2 = local_4e;
    _local_4c = CONCAT22(local_4a, local_4e);
    local_4e = offset;
    local_50 = 0x84a2;
    DeleteObject16(uVar2);
    _local_4c = CONCAT22(local_50, 0x1538);
    local_4e = 0x84aa;
    DeleteObject16(local_50);
    local_48 = param_1;
    _local_4c = CONCAT22(unaff_SS, &local_3c);
    local_4e = offset;
    local_50 = 0x84b7;
    EndPaint16(&local_3c, unaff_SS);
    return;
}

pub fn realize_palette_1008_46e4(param_1: u32, param_2: *mut HDC16) -> u16 {
    let mut bVar1: bool;
    let mut uVar2: u16;
    let mut HVar3: HPALETTE16;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 6) == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | uVar5 << 0x10));
    }
    if ((iVar4 + 6) == 0) {
        bVar1 = false;
    } else {
        if ((iVar4 + 10) == 0) {
            process_struct_1008_4834((param_1 & 0xffff | uVar5 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0;
    }
    uVar2 = create_palette_1008_4e38((iVar4 + 10));
    (iVar4 + 0xe) = uVar2;
    let param_2_val = unsafe { *param_2 };
    HVar3 = SelectPalette16(0, (iVar4 + 0xe), param_2_val);
    (iVar4 + 4) = HVar3;
    RealizePalette16(param_2_val);
    return (iVar4 + 4);
}

pub fn realize_palette_1008_4e08(param_1: u32, param_2: *mut HDC16) -> HDC16 {
    let mut h_palette: HPALETTE16;
    let mut HVar1: HDC16;
    let mut local_4: u16;

    unsafe { HVar1 = *param_2 };
    h_palette = create_palette_1008_4e38(param_1);
    SelectPalette16(0, h_palette, HVar1);
    unsafe { HVar1 = *param_2 };
    RealizePalette16(HVar1);
    return HVar1;
}

pub fn create_palette_1008_4e38(param_1: u32) {
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let in_DX: *mut astruct_199;
    let local_BX_4: *mut astruct_33;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar7 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar4 = (local_BX_4.field_0xc + 2) * 4;
    if (&local_BX_4.palette == 0) {
        process_struct_1000_179c(uVar4, in_DX);
        &local_BX_4.palette = uVar4;
        local_BX_4.field_0x10 = in_DX;
        *&local_BX_4.palette = 0x300;
        uVar3 = &local_BX_4.palette;
        (uVar3 + 2) = local_BX_4.field_0xc;
        uVar2 = &local_BX_4.palette;
        local_8 = uVar2 & 0xffff0000 | (uVar2 + 4);
        local_c = local_BX_4.field_0x4;
        local_e = 0;
        loop {
            puVar1 = &local_BX_4.field_0xc;
            let pu_var_1_val = unsafe { *puVar1 };
            if (pu_var_1_val == local_e || pu_var_1_val < local_e) {
                break;
            }
            uVar8 = (local_c >> 0x10);
            iVar5 = local_c;
            *local_8 = *(iVar5 + 2);
            uVar9 = (local_8 >> 0x10);
            iVar6 = local_8;
            *(iVar6 + 1) = *(iVar5 + 1);
            *(iVar6 + 2) = *local_c;
            *(iVar6 + 3) = 0;
            local_e = local_e + 1;
            local_8 = local_8 & 0xffff0000 | (iVar6 + 4);
            local_c = local_c & 0xffff0000 | (iVar5 + 4);
        }
    }
    CreatePalette16(&local_BX_4.palette);
    return;
}

pub fn draw_1008_4f20(param_1: *mut astruct_104, param_2: u32, param_3: u16, param_4: u32) {
    let mut palette: u16;
    let mut count: i32;
    let mut palette_1: u16;
    let mut local_EAX_147: u32;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut u_var1: u32;
    let mut color_ref: u32;
    let uVar2: u8;
    let uVar3: u8;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let paVar7: *mut astruct_102;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let a_struct_7_26: *mut astruct_7;

    paVar7 = param_1;
    uVar9 = (param_1 >> 0x10);
    set_struct_1008_4016(paVar7);
    (paVar7 + 1) = param_4;
    &paVar7[1].field_0x4 = param_3;
    paVar7[1].field_0x6 = param_2;
    param_1 = (s_SinternalPutBldg2_site_0x_08lx__1050_5099 + 9);
    paVar7.field_0x2 = &PTR_LOOP_1050_1008;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 2);
    process_struct_1008_48fe(
        CONCAT22(unaff_SS, &a_struct_7_26),
        1,
        param_2 & 0xffff | extraout_DX << 0x10,
    );
    read_from_file_1008_49e8(&a_struct_7_26);
    pass1_1008_5068(paVar7, uVar9, &a_struct_7_26, unaff_SS);
    process_struct_1008_47cc(param_1);
    process_struct_1008_4834(param_1);
    uVar8 = 0x27e;
    uVar5 = 0;
    uVar6 = 0;
    uVar2 = 0;
    uVar3 = 0;
    uVar4 = 0;
    uVar1 = process_struct_1008_4772(param_1);
    local_28 = (uVar1 >> 0x10);
    local_2a = uVar1;
    local_EAX_147._0_2_ = CreateDC16(
        local_2a,
        CONCAT13(uVar3, CONCAT12(uVar2, local_28)),
        CONCAT22(uVar5, uVar4),
        CONCAT22(uVar8, uVar6),
    );
    palette = &local_2c;
    realize_palette_1008_46e4(paVar7, uVar9, palette, unaff_SS);
    color_ref = SetBkColor16(0xffffff, local_EAX_147);
    SetTextColor16(CONCAT22(0x100, &paVar7[1].field_0x4), local_EAX_147);
    count = get_string_index_1000_3da4(*(paVar7 + 1));
    TextOut16(count, (paVar7 + 1), 0, 0, local_EAX_147);
    SetBkColor16(color_ref, local_EAX_147);
    SetTextColor16(CONCAT22(local_EAX_147, local_EAX_147), local_EAX_147);
    palette_1 = SelectPalette16(0, palette, local_EAX_147);
    DeleteObject16(palette_1);
    DeleteDC16(local_EAX_147);
    close_file_1008_496c(&a_struct_7_26);
    return;
}

pub fn set_palette_fn_1018_69ac(in_astruct_376: *mut astruct_376) {
    let local_astruct_376: *mut astruct_376;
    let mut u16_1: u16;

    u16_1 = (in_astruct_376 >> 0x10);
    local_astruct_376 = in_astruct_376;
    in_astruct_376.ptr_a_lo = 0x6a02;
    local_astruct_376.ptr_a_hi = 0x1018;
    if (&local_astruct_376.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_astruct_376.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(in_astruct_376);
    return;
}

pub fn set_palette_fn_1018_69dc(in_astruct_376: *mut astruct_376, param_2: u8) -> *mut astruct_376 {
    set_palette_fn_1018_69ac(in_astruct_376);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_astruct_376);
    }
    return in_astruct_376;
}

pub fn load_cursor_fn_1018_6a0e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: *mut libc::c_void,
) -> *mut astruct_65 {
    let local_BX_25: *mut astruct_65;
    let mut local_ES_25: u16;

    load_cursor_1008_61b2(param_1, param_3, param_6, param_7);
    local_ES_25 = (param_1 >> 0x10);
    local_BX_25 = param_1;
    local_BX_25.u16_xea = param_5;
    local_BX_25.u16_xec = param_4;
    local_BX_25.u16_xee = param_2;
    local_BX_25.u16_xf0 = 0;
    param_1.ptr_a_lo = 0x6c66;
    local_BX_25.ptr_a_hi = 0x1018;
    local_BX_25.u16_xe0 = 1;
    local_BX_25.ptr_b_lo = 0;
    local_BX_25.ptr_b_hi = 0;
    &local_BX_25.u16_xe6 = 0x1df027f;
    return param_1;
}

pub fn begin_end_Paint_1018_6a7a(param_1: *mut astruct_129) {
    let local_p_astruct_129: *mut astruct_129;
    let mut uVar1: u16;
    let mut unaff_SS: HWND16;
    let ppVar2: *mut pass1_struct_1;
    let mut HVar3: HWND16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut string_22: [u8; 32];

    uVar1 = (param_1 >> 0x10);
    local_p_astruct_129 = param_1;
    BeginPaint16(CONCAT22(unaff_SS, string_22), local_p_astruct_129.h_wnd_0x8);
    HVar3 = local_p_astruct_129.h_wnd_0x8;
    EndPaint16(string_22, unaff_SS);
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(HVar3, 2));
    if ((ppVar2 + 0x20) == 0) {
        win_func_1018_6bb6(param_1);
        return;
    }
    win_fn_1018_6adc(param_1);
    return;
}

pub fn load_cursor_fn_1018_6d02(
    in_struct_65: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(in_struct_65, 0, 0xb, 0x9c, 0x8b, param_2, param_3);
    in_struct_65.ptr_a_lo = 0xa27e;
    (in_struct_65 + 2) = 0x1018;
    return in_struct_65;
}

pub fn load_cursor_fn_1018_6d38(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xc, 0x9d, 0xd0, param_2, param_3);
    param_1.ptr_a_lo = 0xb562;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6d6e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xd, 0x9e, 0xd1, param_2, param_3);
    param_1.ptr_a_lo = 0x9822;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6da4(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xe, 0x9f, 0xd2, param_2, param_3);
    param_1.ptr_a_lo = 0xab06;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6dda(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xf, 0xa0, 0xd4, param_2, param_3);
    param_1.ptr_a_lo = 0xbdea;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e10(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x10, 0xa1, 0xda, param_2, param_3);
    param_1.ptr_a_lo = 0xa0aa;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e46(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x11, 0xa2, 0xdc, param_2, param_3);
    param_1.ptr_a_lo = 0xb38e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e7c(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x12, 0xa3, 0xd3, param_2, param_3);
    param_1.ptr_a_lo = 0x964e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6eb2(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x13, 0xa4, 0xdb, param_2, param_3);
    param_1.ptr_a_lo = 0xa932;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6ee8(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x14, 0xa5, 0xa5, param_2, param_3);
    param_1.ptr_a_lo = 0xbc16;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f1e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x15, 0xa7, 0xb2, param_2, param_3);
    param_1.ptr_a_lo = 0x9e3a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f54(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x16, 0xa8, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb11e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f8a(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x17, 0xaf, 0xc0, param_2, param_3);
    param_1.ptr_a_lo = 0x93de;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6fc0(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x18, 0xb0, 0xc1, param_2, param_3);
    param_1.ptr_a_lo = 0xa6c2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6ff6(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x19, 0xb1, 0x80, param_2, param_3);
    param_1.ptr_a_lo = 0xb9a6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_702c(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ec, 0x1a, 0xb2, 0xc3, param_2, param_3);
    param_1.ptr_a_lo = 0x9c66;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7062(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1b, 0xb3, 0xc4, param_2, param_3);
    param_1.ptr_a_lo = 0xaf4a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7098(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1c, 0xb4, 0xd8, param_2, param_3);
    param_1.ptr_a_lo = 0xc22e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_70ce(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1d, 0xb5, 0x7b, param_2, param_3);
    param_1.ptr_a_lo = 0xa4ee;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7104(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1e, 0xb6, 0xd9, param_2, param_3);
    param_1.ptr_a_lo = 0xb7d2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_713a(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1f, 0xb7, 0x7d, param_2, param_3);
    param_1.ptr_a_lo = 0x9a92;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7170(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x21, 0xb9, 0xdd, param_2, param_3);
    param_1.ptr_a_lo = 0xad76;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_71a6(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x23, 0xd3, 0xd6, param_2, param_3);
    param_1.ptr_a_lo = 0xb69a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_71dc(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ed, 0x24, 0xd4, 0xd7, param_2, param_3);
    param_1.ptr_a_lo = 0x995a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7212(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x25, 0xe9, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0xa452;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_7248(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 99, 0xa6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc05a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_727e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 100, 0xa9, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa31a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_72b4(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x65, 0xaa, 0xbb, param_2, param_3);
    param_1.ptr_a_lo = 0xb5fe;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_72ea(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x66, 0xab, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x98be;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7320(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x67, 0xac, 0xbd, param_2, param_3);
    param_1.ptr_a_lo = 0xaba2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7356(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x68, 0xad, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbe86;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_738c(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x69, 0xae, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xac3e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_73c2(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x35, 0xba, 0x81, param_2, param_3);
    param_1.ptr_a_lo = 0xbf22;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_73f8(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x39, 0xbb, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa146;
    (param_1 + 2) = 0x1018;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn drawing_and_sound_fn_1018_742e(in_struct_129: *mut astruct_129) {
    let mut iVar1: i32;

    begin_end_Paint_1018_6a7a(in_struct_129);
    if (PTR_LOOP_1050_4254 == 0x0) {
        iVar1 = mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1e9);
        if (iVar1 != 0) {
            PTR_LOOP_1050_4254 = (&PTR_LOOP_1050_0000 + 1);
        }
    }
    return;
}

pub fn load_cursor_fn_1018_745e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x22, 0xbc, 0xd5, param_2, param_3);
    param_1.ptr_a_lo = 0xb42a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7494(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x36, 0xbd, 0xcd, param_2, param_3);
    param_1.ptr_a_lo = 0x96ea;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_74ca(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x37, 0xbe, 0x83, param_2, param_3);
    param_1.ptr_a_lo = 0xa9ce;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7500(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x38, 0xbf, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbcb2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7536(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3a, 0xc0, 0x85, param_2, param_3);
    param_1.ptr_a_lo = 0x9f72;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_756c(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e2, 0x3b, 0xc1, 0x86, param_2, param_3);
    param_1.ptr_a_lo = 0xb256;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_75a2(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3c, 0xc2, 0x87, param_2, param_3);
    param_1.ptr_a_lo = 0x9516;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_75d8(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3d, 0xc3, 0x88, param_2, param_3);
    param_1.ptr_a_lo = 0xa7fa;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_760e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3e, 0xc4, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbade;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7644(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3f, 0xc5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9d02;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_767a(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x40, 0xc6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xafe6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_76b0(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x41, 199, 0x8d, param_2, param_3);
    param_1.ptr_a_lo = 0xc2ca;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_76e6(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x42, 200, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa58a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_771c(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x43, 0xc9, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb86e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7752(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x44, 0xcc, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9b2e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7788(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x45, 0xcd, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xae12;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_77be(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x46, 0xd1, 0x92, param_2, param_3);
    param_1.ptr_a_lo = 0xc0f6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_77f4(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x47, 0xd2, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa3b6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_782a(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x48, 0xd5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xacda;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x49, 0xd6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbfbe;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7896(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 500, 0x4a, 0xd7, 0x98, param_2, param_3);
    param_1.ptr_a_lo = 0xa1e2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_78cc(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4b, 0xd8, 0x99, param_2, param_3);
    param_1.ptr_a_lo = 0xb4c6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7902(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4c, 0xd9, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0x9786;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7938(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4d, 0xda, 0x9c, param_2, param_3);
    param_1.ptr_a_lo = 0xaa6a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_796e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4e, 0xdb, 0x9d, param_2, param_3);
    param_1.ptr_a_lo = 0xbd4e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_79a4(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4f, 0xdc, 0x9e, param_2, param_3);
    param_1.ptr_a_lo = 0xa00e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_79da(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x50, 0xdd, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb2f2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a10(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1d9, 0x51, 0xde, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x95b2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a46(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x52, 0xdf, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa896;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a7c(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x53, 0xe0, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbb7a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7ab2(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e4, 0x55, 0xe2, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb082;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7ae8(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e4, 0x56, 0xe3, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc366;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b1e(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1da, 0x57, 0xe4, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa626;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b54(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1d8, 0x58, 0xe5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb90a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b8a(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x59, 0xe6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9bca;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7bc0(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ef, 0x5a, 0xe7, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xaeae;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7bf6(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5b, 0xe8, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc192;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c2c(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5c, 0xea, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb736;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c62(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5d, 0xeb, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x99f6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c98(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e6, 0x5e, 0xec, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0xba42;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7cce(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1da, 0x5f, 0xed, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9ed6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d04(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x60, 0xee, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb1ba;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d3a(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1f0, 0x61, 0xef, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x947a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d70(
    param_1: *mut astruct_65,
    param_2: u16,
    param_3: *mut void,
) -> *mut astruct_65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1f7, 0x62, 0xf0, 0xcc, param_2, param_3);
    param_1.ptr_a_lo = 0xa75e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn draw_1018_c578(param_1: u32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let in_struct_104_ptr: *mut astruct_104;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let h_palette: *mut u16;
    let mut iVar5: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut uVar6: u16;
    let mut iVar7: i32;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut unaff_SS: HWND16;
    let mut uVar9: u32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    _local_26 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    local_28 = (_local_26 + 0x20);
    iVar7 = param_1;
    uVar8 = (param_1 >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar7 + 8));
        EndPaint16(&local_22, unaff_SS);
        PostMessage16(0, (iVar7 + 0xea), 0x111, g_h_window);
        return;
    }
    if (((iVar7 + 0xf0) == 0) && ((iVar7 + 0xf4) != 0)) {
        (iVar7 + 0xf0) = 1;
        mci_send_command_1008_5c9e(
            _g_struct_ptr_1050_02a0,
            (param_1 & 0xffff0000 | (iVar7 + 0xf2)),
        );
    }
    if ((_g_struct_ptr_1050_02a0 + 0x12) == 0) {
        mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1d3);
    }
    local_2a = BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar7 + 8));
    local_2c = CreateSolidBrush16(0x2000000);
    local_34 = 0;
    local_30 = (iVar7 + 0xf6) - 1;
    local_2e = (iVar7 + 0xf8) - 1;
    FillRect16(local_2c, &local_34, unaff_SS);
    DeleteObject16(local_2c);
    uVar4 = (iVar7 + 0xe2);
    in_struct_104_ptr = (uVar4 + 0xe);
    h_palette = &local_2a;
    uVar4 = in_struct_104_ptr;
    ppcVar3 = (uVar4 + 8);
    (**ppcVar3)(
        offset,
        in_struct_104_ptr,
        (in_struct_104_ptr >> 0x10),
        h_palette,
    );
    uVar9 = process_struct_1008_4772(in_struct_104_ptr);
    uVar6 = (uVar9 >> 0x10);
    iVar1 = (uVar9 + 4);
    iVar2 = (uVar9 + 8);
    iVar5 = (0x1e0 - iVar2) / 2;
    (iVar7 + 0x10c) = iVar5 + iVar2 + (iVar7 + 0x110);
    ppcVar3 = (uVar4 + 4);
    (**ppcVar3)(
        &PTR_LOOP_1050_1008,
        in_struct_104_ptr,
        (iVar7 + 0xfc) + (iVar7 + 0xfe) + iVar5,
        (iVar7 + 0xfa) + (0x280 - iVar1) / 2,
        0xd6,
    );
    draw_text_1018_c742(param_1, &local_2a, unaff_SS);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_SS);
    return;
}

// WARNING: Variable defined which should be unmapped: local_1c
// WARNING: Could not reconcile some variable overlaps

pub fn draw_text_1018_c742(param_1: u32, param_2: *mut HDC16) {
    let pi_var1: *mut i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let local_BX_4: *mut astruct_54;
    let mut uVar4: u16;
    let mut unaf_SS: u16;
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
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if ((local_BX_4.field_0x108 != 0) && (*local_BX_4.field_0x108 != '\0')) {
        let param_2_val = unsafe { *param_2 };
        let param_3_val = unsafe { *param_3 };
        _local_6 = SetTextColor16(0x8000, param_3_val);
        _local_a = SetBkColor16(0x2000000, param_2_val);
        if (local_BX_4.field_0x106 == 0) {
            local_18 = 0;
            local_16 = &local_BX_4.field_0x10e;
            local_14 = 0;
            local_1a = param_2_val;
            uVar2 = local_BX_4.field_0x108;
            local_1c = (uVar2 >> 0x10);
            DrawText16(
                0x410,
                &local_1a,
                unaff_SS,
                CONCAT22(uVar2, 0xffff),
                local_1c,
            );
            local_BX_4.field_0x100 = (0x280 - local_16) / 2;
            local_BX_4.field_0x102 = local_BX_4.field_0x10c;
            local_BX_4.field_0x104 = local_BX_4.field_0x100 + local_16;
            iVar3 = local_BX_4.field_0x102 + local_14;
            local_BX_4.field_0x106 = iVar3;
            pi_var1 = &local_BX_4.field_0xf8;
            let pi_var1_val = unsafe { *pi_var1 };
            if (pi_var1_val == iVar3 || pi_var1_val < iVar3) {
                iVar3 = iVar3 - local_BX_4.field_0xf8;
                pi_var1 = &local_BX_4.field_0x102;
                unsafe {
                    *pi_var1 = *pi_var1 - iVar3;
                    pi_var1 = &local_BX_4.field_0x106;
                    *pi_var1 = *pi_var1 - iVar3;
                }
            }
        }
        local_12 = local_BX_4.field_0xfa + local_BX_4.field_0x100;
        local_10 = local_BX_4.field_0xfc + local_BX_4.field_0x102;
        local_e = local_BX_4.field_0xfa + local_BX_4.field_0x104;
        local_c = local_BX_4.field_0xfc + local_BX_4.field_0x106;
        uVar2 = local_BX_4.field_0x108;
        DrawText16(
            0x10,
            &local_12,
            unaff_SS,
            CONCAT22(uVar2, 0xffff),
            (uVar2 >> 0x10),
        );
        SetTextColor16(_local_6, param_2_val);
        SetBkColor16(_local_a, param_2_val);
    }
    return;
}

pub fn draw_1018_cda8(param_1: u32) {
    let pi_var1: *mut i32;
    let mut iVar2: i32;
    let in_struct_104_ptr: *mut astruct_104;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let h_palette: *mut u16;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut uVar7: u16;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: HWND16;
    let mut uVar11: u32;
    let mut in_stack_0000ffb0: u32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    _local_26 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT22((in_stack_0000ffb0 >> 0x10), 2),
    );
    local_28 = (_local_26 + 0x20);
    iVar8 = param_1;
    uVar9 = (param_1 >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar8 + 8));
        EndPaint16(&local_22, unaff_SS);
        PostMessage16(0, (iVar8 + 0xea), 0x111, g_h_window);
        return;
    }
    if ((iVar8 + 0xf0) == 0) {
        (iVar8 + 0xf0) = 1;
        mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 499);
        if ((_g_struct_ptr_1050_02a0 + 0x12) == 0) {
            mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1d3);
        }
    }
    local_2a = BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar8 + 8));
    local_2c = CreateSolidBrush16(0x2000000);
    local_34 = 0;
    local_30 = (iVar8 + 0xf6) - 1;
    local_2e = (iVar8 + 0xf8) - 1;
    FillRect16(local_2c, &local_34, unaff_SS);
    DeleteObject16(local_2c);
    uVar4 = (iVar8 + 0xe2);
    in_struct_104_ptr = (uVar4 + 0xe);
    h_palette = &local_2a;
    uVar10 = (in_struct_104_ptr >> 0x10);
    ppcVar3 = (in_struct_104_ptr + 8);
    (**ppcVar3)(offset, in_struct_104_ptr, uVar10, h_palette);
    uVar11 = process_struct_1008_4772(in_struct_104_ptr);
    uVar7 = (uVar11 >> 0x10);
    iVar5 = (0x280 - (uVar11 + 4)) / 2;
    iVar2 = (uVar11 + 8);
    iVar6 = (0x1e0 - iVar2) / 2;
    (iVar8 + 0x10c) = iVar6 + iVar2 + (iVar8 + 0x110);
    if (((iVar8 + 0xfa) == 0) && (iVar5 == 0)) {
        pi_var1 = (iVar8 + 0xfa);
        unsafe { *pi_var1 = *pi_var1 + 2 };
    }
    ppcVar3 = (in_struct_104_ptr + 4);
    (**ppcVar3)(
        &PTR_LOOP_1050_1008,
        in_struct_104_ptr,
        uVar10,
        (iVar8 + 0xfc) + (iVar8 + 0xfe) + iVar6,
        (iVar8 + 0xfa) + iVar5,
        0xd6,
    );
    draw_text_1018_c742(param_1, &local_2a, unaff_SS);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_1018_cfc0(param_1: u32) {
    let pi_var1: *mut i32;
    let mut iVar2: i32;
    let in_struct_104_ptr: *mut astruct_104;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let h_palette: *mut u16;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut uVar7: u16;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: HWND16;
    let mut uVar11: u32;
    let mut in_stack_0000ffb0: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    _local_26 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffb0, 2));
    local_28 = (_local_26 + 0x20);
    iVar8 = param_1;
    uVar9 = (param_1 >> 0x10);
    if (local_28 == 0) {
        BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar8 + 8));
        EndPaint16(&local_22, unaff_SS);
        PostMessage16(0, (iVar8 + 0xea), 0x111, g_h_window);
        return;
    }
    if (((iVar8 + 0xf0) == 0) && ((iVar8 + 0xf4) != 0)) {
        (iVar8 + 0xf0) = 1;
        mci_send_command_1008_5c9e(
            _g_struct_ptr_1050_02a0,
            (param_1 & 0xffff0000 | (iVar8 + 0xf2)),
        );
        if ((_g_struct_ptr_1050_02a0 + 0x12) == 0) {
            mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1d3);
        }
    }
    local_2a = BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar8 + 8));
    local_2c = CreateSolidBrush16(0x2000000);
    local_34 = 0;
    local_30 = (iVar8 + 0xf6) - 1;
    local_2e = (iVar8 + 0xf8) - 1;
    FillRect16(local_2c, &local_34, unaff_SS);
    DeleteObject16(local_2c);
    uVar4 = (iVar8 + 0xe2);
    in_struct_104_ptr = (uVar4 + 0xe);
    h_palette = &local_2a;
    uVar10 = (in_struct_104_ptr >> 0x10);
    ppcVar3 = (in_struct_104_ptr + 8);
    (**ppcVar3)(offset, in_struct_104_ptr, uVar10, h_palette);
    uVar11 = process_struct_1008_4772(in_struct_104_ptr);
    uVar7 = (uVar11 >> 0x10);
    iVar5 = (0x280 - (uVar11 + 4)) / 2;
    iVar2 = (uVar11 + 8);
    iVar6 = (0x1e0 - iVar2) / 2;
    (iVar8 + 0x10c) = iVar6 + iVar2 + (iVar8 + 0x110);
    if (((iVar8 + 0xfa) == 0) && (iVar5 == 0)) {
        pi_var1 = (iVar8 + 0xfa);
        unsafe { *pi_var1 = *pi_var1 + 2 };
    }
    ppcVar3 = (in_struct_104_ptr + 4);
    (**ppcVar3)(
        &PTR_LOOP_1050_1008,
        in_struct_104_ptr,
        uVar10,
        (iVar8 + 0xfc) + (iVar8 + 0xfe) + iVar6,
        (iVar8 + 0xfa) + iVar5,
        0xd6,
    );
    draw_text_1018_c742(param_1, &local_2a, unaff_SS);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2a);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_fn_1018_dfd4(in_astruct_618_ptr_1: *mut astruct_618) -> u16 {
    let mut uVar1: u16;
    let local_astruct_618_ptr_1: *mut astruct_618;
    let local_astruct_618_ptr_1_hi: *mut astruct_618;
    let puVar2: *mut u8;
    let local_char_ptr_1: *mut char;
    let temp_5fe392f4b5: *mut void;

    local_astruct_618_ptr_1_hi = (in_astruct_618_ptr_1 >> 0x10);
    local_astruct_618_ptr_1 = in_astruct_618_ptr_1;
    temp_5fe392f4b5 = local_astruct_618_ptr_1.void_ptr_32_xe2;
    uVar1 = draw_1018_e16c(temp_5fe392f4b5, (temp_5fe392f4b5 >> 0x10));
    if (local_astruct_618_ptr_1.field_0xe6 == 0) {
        local_astruct_618_ptr_1.field_0xe6 = 1;
        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_char_ptr_1, 0x36));
        puVar2 = pass1_1038_af40(_g_astruct_112_a, local_astruct_618_ptr_1.field_0x8, 8);
        uVar1 = SUB42(puVar2, 0);
    }
    return uVar1;
}

pub fn draw_1018_e16c(param_1: u32) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let h_palette: *mut u16;
    let mut h_gdi_obj: HPALETTE16;
    let mut uVar4: u16;
    let mut unaff_SS: HWND16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar4 = (param_1 >> 0x10);
    local_24 = BeginPaint16(CONCAT22(unaff_SS, &local_22), (param_1 + 4));
    uVar3 = (param_1 + 6);
    puVar1 = (uVar3 + 10);
    h_palette = &local_24;
    uVar3 = unsafe { *puVar1 };
    ppcVar2 = (uVar3 + 8);
    ppcVar2(offset, puVar1, (puVar1 >> 0x10), h_palette);
    ppcVar2 = (uVar3 + 4);
    ppcVar2(offset, puVar1, 0, &local_24);
    h_gdi_obj = SelectPalette16(0, h_palette, local_24);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_fn_1018_e4f2(in_struct_622_ptr: *mut astruct_622, in_handle_ptr: *mut HANDLE16) {
    let mut local_AX__1: u16;
    let puVar1: *mut u16;
    let mut local_DX_46: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut local_handle_1: HANDLE16;
    let mut uStack16: u16;
    let local_char_ptr: *mut char;
    let mut local_6: u32;
    let temp_7e056a7c981: *mut u32;
    let mut temp_5fb7f3034b: u32;
    let fn_ptr_1: fn();

    local_handle_1 = in_handle_ptr;
    uStack16 = (in_handle_ptr >> 0x10);
    get_dc_1020_921c(CONCAT22(local_handle_1, in_struct_622_ptr), uStack16);
    in_struct_622_ptr.u32_0x14 = 0;
    CONCAT22(local_handle_1, in_struct_622_ptr) = 0xe5d0;
    in_struct_622_ptr.u16_0x2 = 0x1018;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_char_ptr, 0x26));
    local_DX_46 = (ppVar2 >> 0x10);
    &in_struct_622_ptr.u32_0x14 = ppVar2;
    (&in_struct_622_ptr.u32_0x14 + 2) = local_DX_46;
    in_struct_622_ptr.u16_0x6 = &in_struct_622_ptr.u32_0x14;
    in_struct_622_ptr.u16_0x8 = local_DX_46;
    temp_5fb7f3034b = in_struct_622_ptr.u32_0x14;
    puVar1 = &in_struct_622_ptr.field_0xa;
    fn_ptr_1 = ((temp_5fb7f3034b + 10) + 8);
    (**fn_ptr_1)();
    in_struct_622_ptr.i16_0x12 = puVar1;
    draw_1020_9364(CONCAT22(local_handle_1, in_struct_622_ptr));
    return;
}

pub fn select_and_delete_palette_fn_1018_e57a(in_struct_ptr_1: *mut astruct_44) {
    let local_struct_ptr_1: *mut astruct_376;
    let mut uVar1: u16;

    uVar1 = (in_struct_ptr_1 >> 0x10);
    local_struct_ptr_1 = in_struct_ptr_1;
    in_struct_ptr_1.ptr_a_lo = 0xe5d0;
    local_struct_ptr_1.ptr_a_hi = 0x1018;
    if (&local_struct_ptr_1.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_struct_ptr_1.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(in_struct_ptr_1);
    return;
}

pub fn draw_fn_1018_ec74(
    in_struct_1: *mut astruct_622,
    in_struct_1_hi: *mut astruct_622,
    param_3: u16,
) {
    let mut u_var1: u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut unaff_SI: u16;
    let ppVar7: *mut pass1_struct_1;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let paVar10: *mut astruct_622;
    let paVar11: *mut astruct_622;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    get_dc_1020_921c(CONCAT22(in_struct_1_hi, in_struct_1), param_3);
    in_struct_1.u32_0x14 = 0;
    zero_list_1008_3e38(CONCAT22(in_struct_1_hi, &in_struct_1.field_0x18));
    zero_list_1008_3e38(CONCAT22(in_struct_1_hi, &in_struct_1.field_0x1e));
    &in_struct_1.field_0x24 = 0;
    CONCAT22(in_struct_1_hi, in_struct_1) = 0x1cc;
    in_struct_1.u16_0x2 = 0x1020;
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x28));
    uVar5 = (ppVar7 >> 0x10);
    &in_struct_1.u32_0x14 = ppVar7;
    (&in_struct_1.u32_0x14 + 2) = uVar5;
    uVar9 = 0;
    uVar8 = &in_struct_1.u32_0x14;
    fn_ptr_1 = (in_struct_1.u32_0x14 + 4);
    paVar10 = in_struct_1;
    paVar11 = in_struct_1_hi;
    (**fn_ptr_1)();
    uVar3 = in_struct_1.u32_0x14;
    &in_struct_1.u16_0x6 = uVar3;
    uVar1 = in_struct_1.u32_0x14;
    modify_list_1010_2b50(
        uVar1,
        (uVar1 >> 0x10),
        CONCAT22(in_struct_1_hi, &in_struct_1.field_0x18),
    );
    uVar6 = uVar3;
    pass1_1010_2b66(in_struct_1.u32_0x14);
    in_struct_1.field_0x24 = uVar6;
    in_struct_1.field_0x26 = extraout_DX;
    uVar3 = in_struct_1.u32_0x14;
    puVar2 = (uVar3 + 10);
    uVar4 = CONCAT22(in_struct_1_hi, &in_struct_1.field_0xa);
    uVar6 = SUB42(puVar2, 0);
    unsafe {
        fn_ptr_1 = (*puVar2 + 8);
        (**fn_ptr_1)(
            0x1010,
            uVar6,
            (puVar2 >> 0x10),
            uVar4,
            uVar8,
            uVar5,
            uVar9,
            paVar10,
            paVar11,
        );
    }

    in_struct_1.i16_0x12 = uVar4;
    draw_1020_9364(CONCAT22(in_struct_1_hi, in_struct_1));
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar6, 0x48));
    modify_list_1008_3f62(
        CONCAT22(in_struct_1_hi, &in_struct_1.field_0x1e),
        ppVar7 & 0xffff0000 | (ppVar7 + 0xe),
    );
    pass1_1008_3f32(
        &in_struct_1.field_0x18,
        in_struct_1_hi,
        in_struct_1 + '\x1e',
        in_struct_1_hi,
    );
    return;
}

pub fn invalidate_rect_1018_edd8(param_1: u32, param_2: i32) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut unaff_SS: HWND16;
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

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xc) {
        return;
    }
    local_8 = iVar1 + 0x18;
    pass1_1008_3e94(
        local_8,
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    local_a = pass1_1010_2b66((iVar1 + 0x14));
    _local_e = process_struct_1008_4772(CONCAT22(local_8, local_a));
    uVar2 = (_local_e >> 0x10);
    local_16 = local_4;
    local_14 = local_6;
    local_12 = local_4 + (_local_e + 4);
    local_10 = local_6 + (_local_e + 8);
    InvalidateRect16(0, &local_16, unaff_SS);
    return;
}

// WARNING: Variable defined which should be unmapped: local_b6
// WARNING: Variable defined which should be unmapped: local_bc

pub fn draw_1020_0000(param_1: u32) {
    let puVar1: *mut u16;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut extraout_DX: i32;
    let mut uVar6: i32;
    let mut extraout_DX_00: i32;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: HWND16;
    let mut local_c4: [u8; 6];
    let mut local_be: u16;
    let mut local_bc: u16;
    let mut local_b8: u16;
    let mut local_b6: u16;
    let mut local_b4: u16;
    let mut local_b2: u16;
    let mut local_b0: [u16; 60];
    let mut local_38: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_22: PAINTSTRUCT16;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar7 + 4));
    uVar3 = (iVar7 + 0x14);
    local_26 = (uVar3 + 10);
    pass1_1008_3e94(
        (iVar7 + 0x18),
        CONCAT22(unaff_SS, &local_2a),
        CONCAT22(unaff_SS, &local_28),
    );
    uVar9 = SUB42(&PTR_LOOP_1050_1008, 0);
    pass1_1008_4480(
        local_26,
        (param_1 & 0xffff0000 | ZEXT24((iVar7 + 0x18))),
        (iVar7 + 0x24),
    );
    local_2e = 0;
    local_30 = 0;
    while (local_30 < 6) {
        uVar3 = (iVar7 + 0x14);
        uVar9 = 0x1010;
        pass1_1010_2b78(
            uVar3,
            (uVar3 >> 0x10),
            local_30,
            CONCAT22(unaff_SS, &local_b4),
        );
        if (local_b4 == 0) {
            local_38 = 0;
            uVar6 = extraout_DX_00;
            while (uVar4 = local_38, local_38 <= local_b2) {
                puVar1 = local_b0 + local_38 * 3;
                local_b8 = puVar1;
                if (local_b0[local_38 * 3 + 2] != 0) {
                    uVar5 = pass1_1010_2b98((iVar7 + 0x14), local_b0[local_38 * 3 + 2]);
                    local_2e = uVar5 & 0xffff | uVar6 << 0x10;
                    let pu_var1_val = unsafe { *puVar1 };
                    pass1_1008_3e54(
                        CONCAT22(unaff_SS, &local_be),
                        0,
                        local_b0[uVar4 * 3 + 1] + local_2a,
                        pu_var1_val + local_28,
                    );
                    uVar9 = SUB42(&PTR_LOOP_1050_1008, 0);
                    pass1_1008_4480(local_26, CONCAT22(unaff_SS, &local_be), local_2e);
                    uVar6 = extraout_DX;
                }
                local_38 = local_38 + 1;
            }
        } else {
            _local_be = CONCAT22(unaff_SS, local_b0 + local_b2 * 3);
            if (local_b0[local_b2 * 3 + 2] != 0) {
                uVar6 = extraout_DX_00;
                uVar5 = pass1_1010_2b98((iVar7 + 0x14), local_b0[local_b2 * 3 + 2]);
                local_2e = uVar5 & 0xffff | uVar6 << 0x10;
                pass1_1008_3e54(
                    CONCAT22(unaff_SS, local_c4),
                    0,
                    (_local_be + 2) + local_2a,
                    *_local_be + local_28,
                );
                uVar9 = SUB42(&PTR_LOOP_1050_1008, 0);
                pass1_1008_4480(local_26, CONCAT22(unaff_SS, local_c4), local_2e);
            }
        }
        local_30 = local_30 + 1;
    }
    ppcVar2 = (local_26 + 4);
    ppcVar2(uVar9, local_26, (local_26 >> 0x10), 0, 0, iVar7 + 10, uVar8);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn call_fill_rect_1020_041e(in_struct_1: *mut astruct_13) {
    fill_rect_1020_065e((in_struct_1 + 0xe6));
    return;
}

pub fn fill_rect_1020_065e(param_1: *mut astruct_13) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: u32;
    let local_BX_6: *mut astruct_13;
    let mut uVar4: u16;
    let mut h_dc: u16;
    let mut rectangle: RECT16;
    let mut brush: u16;
    let mut palette: u16;
    let mut local_28: u32;
    let mut result: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar4 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    result = BeginPaint16(CONCAT22(h_dc, &local_22), local_BX_6.field_0x4);
    if (0x280 < local_BX_6.field_0xa) {
        brush = CreateSolidBrush16(0x210070b);
        rectangle.left = 0;
        rectangle.top = 0;
        rectangle.right = local_BX_6.field_0xa + -1;
        rectangle.bottom = local_BX_6.field_0xc + -1;
        FillRect16(brush, &rectangle, h_dc);
        DeleteObject16(brush);
    }
    uVar2 = local_BX_6.field_0x6;
    local_28 = (uVar2 + 0xe);
    palette = &result;
    uVar3 = local_28;
    ppcVar1 = (uVar3 + 8);
    (**ppcVar1)(offset, local_28, (local_28 >> 0x10), palette);
    ppcVar1 = (uVar3 + 4);
    (**ppcVar1)(
        offset,
        local_28,
        local_BX_6.field_0x10,
        local_BX_6.field_0xe,
        &result,
    );
    palette = SelectPalette16(0, palette, result);
    DeleteObject16(palette);
    EndPaint16(&local_22, h_dc);
    return;
}

pub fn draw_fn_1020_0a52(param_1: *mut astruct_630) {
    let local_BX_4: *mut astruct_630;
    let mut uVar1: u16;
    let puVar2: *mut u8;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    draw_1020_0c3e(local_BX_4.field_0xe2);
    if (local_BX_4.field_0xe6 == 0) {
        local_BX_4.field_0xe6 = 1;
        (_g_astruct_112_a + 0xae) = 0x99;
        puVar2 = pass1_1038_af40(_g_astruct_112_a, local_BX_4.field_0x8, 6);
        local_BX_4.field_0xe8 = puVar2;
        local_BX_4.field_0xea = (puVar2 >> 0x10);
    }
    return;
}

pub fn palette_fn_1020_0aa6(param_1: *mut void) {
    palette_fn_1020_0cd2((param_1 + 0xe2));
    return;
}

pub fn draw_1020_0c3e(in_struct_1: *mut astruct_631) {
    let paVar1: *mut astruct_632;
    let h_palette: *mut HDC16;
    let mut h_gdi_obj: HPALETTE16;
    let struct_a: *mut astruct_631;
    let struct_b: *mut astruct_632;
    let struct_a_hi: *mut astruct_631;
    let struct_c_hi: *mut astruct_632;
    let mut window_handle_a: HWND16;
    let mut local_30: u16;
    let local_28: *mut astruct_632;
    let mut hdc1: HDC16;
    let mut paint_struct_a: PAINTSTRUCT16;
    let struct_c: *mut astruct_632;
    let temp_5f82dd1a72: *mut astruct_632;
    let fn_ptr_1: fn();

    struct_a_hi = (in_struct_1 >> 0x10);
    struct_a = in_struct_1;
    hdc1 = BeginPaint16(
        CONCAT22(window_handle_a, &paint_struct_a),
        struct_a.hwnd_0x4,
    );
    struct_c = struct_a.astruct_632_0x6;
    struct_c_hi = (struct_c >> 0x10);
    struct_b = struct_c;
    paVar1 = b.fn_ptr_0xa;
    local_28._0_2_ = paVar1;
    if ((struct_b.field_0xc | local_28) != 0) {
        h_palette = &hdc1;
        temp_5f82dd1a72 = paVar1;
        fn_ptr_1 = (temp_5f82dd1a72 + 8);
        (**fn_ptr_1)(offset, local_28, (paVar1 >> 0x10), h_palette);
        fn_ptr_1 = (temp_5f82dd1a72 + 4);
        (**fn_ptr_1)(
            offset,
            paVar1,
            &struct_a.field_0xc,
            struct_a.field_0xa,
            &hdc1,
        );
        h_gdi_obj = SelectPalette16(0, h_palette, hdc1);
        DeleteObject16(h_gdi_obj);
    }
    EndPaint16(&paint_struct_a, window_handle_a);
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn palette_fn_1020_0cd2(in_struct_1: *mut astruct_633) {
    let paVar1: *mut astruct_634;
    let mut uVar2: i32;
    let mut hdc: HDC16;
    let mut h_palette: HDC16;
    let mut h_palette_00: HPALETTE16;
    let mut uVar3: u16;
    let mut extraout_DX: i32;
    let local_struct_1: *mut astruct_633;
    let local_struct_2: *mut astruct_634;
    let local_struct_1_hi: *mut astruct_633;
    let local_struct_2_hi: *mut astruct_634;
    let mut h_dc: HDC16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let temp_5f7b277b00: *mut u32;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = local_struct_1.field_0x6;
    local_struct_2_hi = (paVar1 >> 0x10);
    local_struct_2 = paVar1;
    temp_5f7b277b00 = &local_struct_2.field_0xa;
    local_6._0_2_ = temp_5f7b277b00;
    uVar2 = &local_struct_2.field_0xc | local_6;
    if (uVar2 != 0) {
        let tmp_5f7b_val = unsafe { *temp_5f7b277b00 };
        fn_ptr_1 = (tmp_5f7b_val + 0x14);
        (**fn_ptr_1)();
        _local_a = CONCAT22(extraout_DX, uVar2);
        if ((extraout_DX | uVar2) != 0) {
            hdc = GetDC16(local_struct_1.win_handle_0x4);
            h_palette = hdc;
            h_dc = hdc;
            create_palette_1008_4e38(_local_a);
            h_palette_00 = SelectPalette16(0, h_palette, h_dc);
            uVar3 = RealizePalette16(hdc);
            SelectPalette16(1, h_palette_00, hdc);
            DeleteObject16(h_palette);
            if (0 < uVar3) {
                InvalidateRect16(1, 0x0, 0);
            }
            ReleaseDC16(hdc, local_struct_1.win_handle_0x4);
            return;
        }
    }
    return;
}

pub fn realize_palette_1020_0e46(param_1: u32, param_2: i32) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u32;
    let mut u_var4: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
        u_var4 = (param_1 >> 0x10);
        u_var2 = (param_1 + 0xf2);
        pu_var3 = (u_var2 + 0x66);
        let pu_var3_val = unsafe { *pu_var3 };
        ppcVar1 = (pu_var3_val + 0x18);
        (**ppcVar1)();
        UnrealizeObject16(pu_var3_val);
        u_var2 = (param_1 + 0xf2);
        RealizePalette16((u_var2 + 0x7c));
    }
    return;
}

pub fn call_Paint_fn_1020_1022(in_struct_1: *mut astruct_14) {
    Paint_func_1020_15de((in_struct_1 + 0xf6));
    return;
}

pub fn palette_func_1020_150e(in_struct_1: *mut astruct_376) {
    let mut u_var1: u32;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1_low: *mut astruct_376;
    let local_struct_1: *mut astruct_376;

    local_struct_1 = (in_struct_1 >> 0x10);
    local_struct_1_low = in_struct_1;
    in_struct_1.ptr_a_lo = 0x1730;
    local_struct_1_low.ptr_a_hi = 0x1020;
    if (&local_struct_1_low.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1ea6(
            local_struct_1_low.u8_ptr_x14,
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1) << 0x10),
        );
    }
    uVar1 = &local_struct_1_low.u8_ptr_x14;
    h_gdi_obj = SelectPalette16(0, &local_struct_1_low.field_0x1c, (uVar1 + 0x7c));
    &local_struct_1_low.field_0x1c = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    in_struct_1.ptr_a_lo = s_0_020_1050_3ab0;
    local_struct_1_low.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = s_1_1050_389a;
    local_struct_1_low.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn draw_1040_a67e(param_1: u32, param_2: i32, uparam_3: i32, in_h_dc: HDC16) {
    let puVar1: *mut u16;
    let mut bVar2: bool;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let mut brush: u16;
    let local_BX_5: *mut astruct_36;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (local_BX_5.brush == 0) {
        brush = CreateSolidBrush16(0);
        local_BX_5.brush = brush;
    }
    if (_PTR_LOOP_1050_5ee8 == 0) {
        uVar6 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar3 = (uVar6 >> 0x10);
        iVar4 = uVar6;
        _PTR_LOOP_1050_5ee8 = CONCAT12(*(iVar4 + 0x94), CONCAT11(*(iVar4 + 0x95), *(iVar4 + 0x96)));
        PTR_LOOP_1050_5eec = CONCAT11(*(iVar4 + 0x3e5), *(iVar4 + 0x3e6));
        PTR_LOOP_1050_5eee = *(iVar4 + 0x3e4);
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return;
        }
        bVar2 = false;
        local_e = 0;
        let puVar1_val = unsafe { *puVar1 };
        while (
            puVar1 = &local_BX_5.field_0xea,
            puVar1_val != local_e && local_e <= puVar1_val,
        ) {
            if ((&local_BX_5.field_0x9a + local_e * 2) == param_2) {
                bVar2 = true;
                break;
            }
            local_e = local_e + 1;
        }
        if (bVar2) {
            PTR_LOOP_1050_5ee8 = PTR_LOOP_1050_5eec;
            PTR_LOOP_1050_5eea = PTR_LOOP_1050_5eee;
        }
    }
    SetTextColor16(CONCAT22(PTR_LOOP_1050_5eea, PTR_LOOP_1050_5ee8), in_h_dc);
    SetBkColor16(0x1000000, in_h_dc);
    return;
}

pub fn draw_1040_9948(param_1: HWND16, param_2: u32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut hdc: HDC16;
    let mut mode: u16;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut obj_handle: HPEN16;
    let mut obj_handle_00: HPEN16;
    let mut obj_handle_01: HGDIOBJ16;
    let mut HVar5: HGDIOBJ16;
    let puVar6: *mut u8;
    let puVar7: *mut u8;
    let mut iVar8: i32;
    let mut hdc_00: HDC16;
    let mut unaff_SS: HWND16;
    let mut uVar9: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u32;
    let mut local_52: u32;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: PAINTSTRUCT16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = 1;
    local_4 = 1;
    local_a = 0;
    local_8 = 0;
    local_1c = 0;
    local_1e = 0;
    hdc_00 = (param_2 >> 0x10);
    iVar8 = param_2;
    local_20 = *(iVar8 + 4) & 8;
    local_22 = *(iVar8 + 0x56) & 1;
    hdc = BeginPaint16(CONCAT22(unaff_SS, &local_42), param_1);
    mode = SetMapMode16(1, hdc);
    GetClientRect16(CONCAT22(unaff_SS, &local_12), param_1);
    iVar2 = (_local_e >> 0x10);
    iVar1 = iVar2 + -1;
    _local_e = CONCAT22(iVar1, local_e - 1);
    if (local_22 != 0) {
        local_1a._0_2_ = local_12;
        local_1a._2_2_ = (local_12 >> 0x10);
        local_12 = CONCAT22(local_1a._2_2_ + 2, local_1a + 2);
        _local_e = CONCAT22(iVar2 + -3, local_e - 3);
        local_16._0_2_ = local_e - 1;
        local_16._2_2_ = iVar1;
    }
    if (*(iVar8 + 6) != '\0') {
        local_1c = 1;
        if ((iVar8 + 0x5a) != 0) {
            local_1e = SelectObject16((iVar8 + 0x5a), hdc);
        }
        uVar3 = iVar8 + 6;
        uVar4 = get_string_index_1000_3da4((param_2 & 0xffff0000 | uVar3));
        DrawText16(0x400, &local_a, unaff_SS, CONCAT22(uVar3, uVar4), hdc_00);
        local_8 = ((local_c - local_4) + local_8) / 2 + local_12._2_2_;
        local_4 = local_4 + local_8;
        local_a = ((local_e - local_6) + local_a) / 2 + local_12;
        local_6 = local_6 + local_a;
    }
    obj_handle = CreatePen16(u32_1050_5ec2, 1, 0);
    obj_handle_00 = CreatePen16(u32_1050_5ebe, 1, 0);
    obj_handle_01 = SelectObject16(obj_handle, hdc);
    if (local_22 != 0) {
        local_4e = 0;
        while {
            MoveTo16(local_16._2_2_, local_1a, hdc);
            LineTo16(local_16._2_2_, local_16, hdc);
            LineTo16(local_1a._2_2_, local_16, hdc);
            LineTo16(local_1a._2_2_, local_1a, hdc);
            LineTo16(local_16._2_2_, local_1a, hdc);
            local_1a._2_2_ = local_1a._2_2_ + 1;
            local_1a._0_2_ = local_1a + 1;
            local_16._0_2_ = local_16 + -1;
            local_16._2_2_ = local_16._2_2_ + -1;
            local_4e = local_4e + 1;
            local_4e < 1
        } {}
    }
    if ((*(iVar8 + 4) & 2) == 0) {
        local_56._2_2_ = (local_12 >> 0x10);
        local_52._0_2_ = _local_e;
        local_4e = 0;
        local_56._0_2_ = local_12;
        local_52._2_2_ = local_c;
        while {
            SelectObject16(obj_handle, hdc);
            MoveTo16(local_52._2_2_, local_56, hdc);
            LineTo16(local_52._2_2_, local_52, hdc);
            LineTo16(local_56._2_2_, local_52, hdc);
            while {
                SelectObject16(obj_handle_00, hdc);
                LineTo16(local_56._2_2_, local_56, hdc);
                LineTo16(local_52._2_2_, local_56, hdc);
                (hdc + 1) < 2
            } {}
            local_56._2_2_ = local_56._2_2_ + 1;
            local_56._0_2_ = local_56 + 1;
            local_52._0_2_ = local_52 + -1;
            local_52._2_2_ = local_52._2_2_ - 1;
            local_4e = local_4e + 1;
            local_4e < 2
        } {}
    } else {
        MoveTo16(local_c, local_12, hdc);
        LineTo16(local_12._2_2_, local_12, hdc);
        LineTo16(local_12._2_2_, local_e + 1, hdc);
        if (local_1c != 0) {
            local_8 = local_8 + 2;
            local_a = local_a + 2;
            local_6 = local_6 + 2;
            local_4 = local_4 + 2;
        }
    }
    MoveTo16(0, 0, hdc);
    if (local_1c != 0) {
        if ((*(iVar8 + 4) & 4) == 0) {
            puVar7 = u16_1050_5ec8;
            puVar6 = PTR_LOOP_1050_5ec6;
            if (local_20 != 0) {
                puVar7 = PTR_LOOP_1050_5ecc;
                puVar6 = u16_1050_5eca;
            }
            SetTextColor16(CONCAT22(puVar7, puVar6), hdc);
            SetBkColor16(0x1000000, hdc);
            uVar3 = get_string_index_1000_3da4((param_2 & 0xffff0000 | (iVar8 + 6)));
            DrawText16(1, &local_a, unaff_SS, CONCAT22(iVar8 + 6, uVar3), hdc_00);
            SetTextColor16(CONCAT22(hdc, hdc), hdc);
            SetBkColor16(CONCAT22(hdc, hdc), hdc);
        } else {
            HVar5 = GetStockObject16(1);
            uVar9 = 0;
            uVar3 = iVar8 + 6;
            uVar4 = get_string_index_1000_3da4((param_2 & 0xffff0000 | uVar3));
            GrayString16(
                0x1000,
                local_4 - local_8,
                local_6 - local_a,
                local_8,
                local_a,
                uVar4,
                uVar3,
                hdc_00,
                uVar9,
                0,
                HVar5,
                hdc,
            );
        }
        if (local_1e != 0) {
            SelectObject16(local_1e, hdc);
        }
    }
    SelectObject16(obj_handle_01, hdc);
    SetMapMode16(mode, hdc);
    DeleteObject16(obj_handle);
    DeleteObject16(obj_handle_00);
    EndPaint16(&local_42, unaff_SS);
    return;
}

pub fn draw_text_1040_9650(param_1: u32) {
    let mut u_var1: u32;
    let mut hdc: HDC16;
    let mut count: u16;
    let mut local_4: u16;

    hdc = GetDC16(0);
    count = (param_1 >> 0x10);
    uVar1 = (param_1 + 4);
    DrawText16(
        0x410,
        (param_1 + 0x2e),
        count,
        CONCAT22(uVar1, 0xffff),
        (uVar1 >> 0x10),
    );
    ReleaseDC16(hdc, 0);
    return;
}

pub fn draw_text_1040_94fc(param_1: u32, param_2: HDC16) -> COLORREF {
    let mut u_var1: u32;
    let local_BX_7: *mut astruct_56;
    let mut count: u16;
    let mut CVar2: COLORREF;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    count = (param_1 >> 0x10);
    local_BX_7 = param_1;
    CVar2 = SetBkColor16(CONCAT22(0x100, local_BX_7.field_0x3a), param_2);
    SetTextColor16(CONCAT22(0x100, local_BX_7.field_0x3c), param_2);
    uVar1 = local_BX_7.field_0x4;
    local_a = (uVar1 >> 0x10);
    DrawText16(
        0x10,
        &local_BX_7.field_0x2e,
        count,
        CONCAT22(uVar1, 0xffff),
        local_a,
    );
    SetBkColor16(CVar2, param_2);
    CVar2 = SetTextColor16(CONCAT22(param_2, param_2), param_2);
    return CVar2;
}

pub fn delete_palette_func_1040_9458(param_1: u32, param_2: u8, param_3: HDC16) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut h_gdi_obj: HPALETTE16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut uStack28: u16;
    let mut uStack26: u16;
    let puStack24: *mut u16;
    let mut uStack20: u16;
    let mut uStack18: u16;
    let puStack16: *mut u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 8) != 0) {
        local_6 = (iVar3 + 8);
        uStack18 = (iVar3 + 10);
        if ((((iVar3 + 0xe) | (iVar3 + 0xc)) != 0) && ((param_2 & 1) != 0)) {
            local_6 = (iVar3 + 0xc);
            uStack18 = (iVar3 + 0xe);
        }
        if (((iVar3 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_6 = (iVar3 + 0x10);
            uStack18 = (iVar3 + 0x12);
        }
        local_8 = &param_3;
        uStack20 = local_6;
        uVar2 = local_6;
        puStack24 = 0x94c5;
        ppcVar1 = (uVar2 + 8);
        puStack16 = local_8;
        (**ppcVar1)();
        puStack24 = &param_3;
        uStack26 = (iVar3 + 0x26);
        uStack28 = (iVar3 + 0x28);
        ppcVar1 = (uVar2 + 4);
        (**ppcVar1)();
        h_gdi_obj = SelectPalette16(0, local_8, param_3);
        DeleteObject16(h_gdi_obj);
    }
    return;
}

pub fn process_struct_1040_9252(param_1: *mut astruct_357) {
    let pi_var1: *mut i32;
    let mut iVar2: i32;
    let local_BX_3: *mut astruct_357;
    let mut uVar3: i32;

    uVar3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (&local_BX_3.field_0x4 != 0) {
        draw_text_1040_9650(param_1);
    }
    if (local_BX_3.field_0x8 != 0x0) {
        process_struct_1040_9618((param_1 & 0xffff | uVar3 << 0x10));
    }
    iVar2 = &local_BX_3[1].field_0x4;
    if (&local_BX_3.field_0x22 < iVar2) {
        &local_BX_3.field_0x22 = iVar2;
    }
    iVar2 = &local_BX_3[1].field_0x6;
    if (&local_BX_3.field_0x24 < iVar2) {
        &local_BX_3.field_0x24 = iVar2;
    }
    iVar2 = &local_BX_3.field_0x26 + local_BX_3.field_0x2a;
    if (&local_BX_3.field_0x22 < iVar2) {
        &local_BX_3.field_0x22 = iVar2;
    }
    iVar2 = &local_BX_3.field_0x28 + local_BX_3.field_0x2c;
    if (&local_BX_3.field_0x24 < iVar2) {
        &local_BX_3.field_0x24 = iVar2;
    }
    pi_var1 = &local_BX_3.field_0x22;
    unsafe {
        *pi_var1 = *pi_var1 + &local_BX_3[1].field_0x8;
        pi_var1 = &local_BX_3.field_0x24;
        *pi_var1 = *pi_var1 + &local_BX_3[1].field_0x8;
    }
    return;
}

pub fn get_sys_metrics_1040_8c66(param_1: u32) {
    let pi_var1: *mut i32;
    let mut bVar2: u8;
    let mut HVar3: HDC16;
    let mut iVar4: u16;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    HVar3 = GetDC16((iVar5 + 6));
    draw_text_1040_8d14(param_1, HVar3);
    (iVar5 + 0xa6) = (iVar5 + 0x9e);
    (iVar5 + 0xaa) = (iVar5 + 0xa2);
    iVar4 = GetSystemMetrics16(4);
    pi_var1 = (iVar5 + 0xac);
    unsafe { *pi_var1 = *pi_var1 + iVar4 };
    bVar2 = *(iVar5 + 0x98) & 0xf0;
    if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
        iVar4 = GetSystemMetrics16(0xc);
        if ((iVar5 + 0xac) < iVar4) {
            iVar4 = GetSystemMetrics16(0xc);
            (iVar5 + 0xac) = iVar4;
        }
    }
    pi_var1 = (iVar5 + 0xaa);
    unsafe {
        *pi_var1 = *pi_var1 + 0x14;
        pi_var1 = (iVar5 + 0xac);
        *pi_var1 = *pi_var1 + 10;
        (iVar5 + 0xb0) = (iVar5 + 0xac);
        pi_var1 = (iVar5 + 0xac);
        *pi_var1 = *pi_var1 + 0x30;
    }
    HVar3 = (iVar5 + 6);
    ReleaseDC16(HVar3, HVar3);
    return;
}

// WARNING: Variable defined which should be unmapped: local_8

pub fn draw_text_1040_8d14(param_1: u32, param_2: HGDIOBJ16) {
    let mut u_var1: u32;
    let mut bVar2: u8;
    let mut iVar3: u16;
    let mut obj_handle: HANDLE16;
    let local_BX_4: *mut astruct_55;
    let mut count: u16;
    let mut local_8: u16;

    count = (param_1 >> 0x10);
    local_BX_4 = param_1;
    bVar2 = local_BX_4.field_0x98 & 0xf0;
    if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
        local_BX_4.field_0xa0 = 10;
        iVar3 = GetSystemMetrics16(0xb);
        local_BX_4.field_0x9e = iVar3 + 0x28;
    } else {
        local_BX_4.field_0xa0 = 10;
        local_BX_4.field_0x9e = 0x14;
    }
    obj_handle = GetProp16(s_hfont_1050_5e0f, local_BX_4.field_0x6);
    if (obj_handle != 0) {
        SelectObject16(obj_handle, param_2);
    }
    uVar1 = local_BX_4.field_0x90;
    local_8 = (uVar1 >> 0x10);
    DrawText16(
        0x410,
        &local_BX_4.field_0x9e,
        count,
        CONCAT22(uVar1, 0xffff),
        local_8,
    );
    if (local_8 != 0) {
        SelectObject16(param_2, param_2);
    }
    return;
}

pub fn draw_1040_82ee(param_1: u32) {
    let local_BX_6: *mut astruct_35;
    let mut hdc: HDC16;
    let mut unaff_SS: HDC16;
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

    hdc = (param_1 >> 0x10);
    local_BX_6 = param_1;
    local_6 = (local_BX_6.field_0x80 - local_BX_6.field_0x7c) - 2;
    local_8 = (-(local_BX_6.field_0x60 == 0) & 0x1e) + 0x25;
    local_4 = local_6;
    local_a = CreateSolidBrush16(CONCAT22(0x100, local_8));
    if (local_BX_6.field_0x86 == 0) {
        _local_1a = CONCAT22(local_BX_6.field_0x66 + 2, local_BX_6.field_0x64 + 2);
        _local_16 = CONCAT22(local_4, local_6);
        &local_BX_6.field_0x82 = _local_1a;
        &local_BX_6.field_0x86 = _local_16;
    }
    local_12 = local_BX_6.field_0x82 + 2;
    local_10 = ((local_BX_6.field_0x88 - local_BX_6.field_0x84) / 2 + local_BX_6.field_0x84) - 2;
    local_e = local_BX_6.field_0x86 - 2;
    local_c = (local_BX_6.field_0x88 - local_BX_6.field_0x84) / 2 + local_BX_6.field_0x84 + 2;
    FrameRect16(local_a, &local_BX_6.field_0x82, hdc);
    FrameRect16(local_a, &local_12, unaff_SS);
    DeleteObject16(local_a);
    local_BX_6.field_0x7a = local_BX_6.field_0x86 + 2;
    return;
}

pub fn pt_in_rect_1040_8158(param_1: *mut astruct_69, param_2: u32, param_3: u16) {
    let ppcVar1: fn();
    let mut iVar2: u16;
    let b_var3: bool;
    let local_BX_10: *mut astruct_69;
    let mut unaff_CS: u16;
    let mut unaff_SS: HWND16;
    let mut local_6: u32;

    if (param_3 == 2) {
        local_BX_10 = param_1;
        if (local_BX_10.field_0x76 != 0) {
            local_6 = param_2;
            ScreenToClient16(CONCAT22(&local_6, unaff_CS), unaff_SS);
            iVar2 = GetSystemMetrics16(4);
            local_6 = local_6 & 0xffff | (local_6._2_2_ + iVar2) << 0x10;
            b_var3 = PtInRect16(local_6, (local_BX_10 + 1));
            if (b_var3 != 0) {
                ppcVar1 = (param_1 + 0x14);
                (**ppcVar1)(offset, param_1, 0);
            }
        }
    }
    return;
}

pub fn draw_fn_1040_7e5e(param_1: *mut u32, param_2: HWND16, uparam_3: i32, param_4: HDC16) {
    let ppcVar1: fn();
    let mut HVar2: HGDIOBJ16;
    let mut iVar3: i32;
    let mut iVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    HVar2 = GetStockObject16(4);
    if (_PTR_LOOP_1050_5df0 == 0) {
        uVar5 = (param_1 >> 0x10);
        let param_1_val = unsafe { *param_1 };
        ppcVar1 = (param_1_val + 0x68);
        uVar6 = (**ppcVar1)(offset, param_1, uVar5, (param_1 + 0x6e));
        if (uVar6 == 0) {
            return 0;
        }
        uVar6 = pass1_1008_4d72(uVar6);
        uVar5 = (uVar6 >> 0x10);
        iVar3 = uVar6;
        _PTR_LOOP_1050_5df0 = CONCAT22(
            CONCAT11(2, *(iVar3 + 0x94)),
            CONCAT11(*(iVar3 + 0x95), *(iVar3 + 0x96)),
        );
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return 0;
        }
        uVar5 = 0x7ed5;
        iVar4 = GetDlgCtrllID16(param_2);
        if (iVar4 == 0x14c) {
            uVar7 = 0xffff;
            local_c = 0;
            // goto LAB_1040_7f00;
        }
        if (iVar4 == 0x175) {
            uVar7 = 0xff;
            local_c = 0;
            // goto LAB_1040_7f00;
        }
    }
    local_c = param_4;
    uVar5 = _PTR_LOOP_1050_5df0;
    uVar7 = (_PTR_LOOP_1050_5df0 >> 0x10);
    // LAB_1040_7f00:
    SetTextColor16(CONCAT22(uVar7, uVar5), local_c);
    SetBkColor16(0x1000000, param_4);
    return CONCAT22(0x1050, HVar2);
}

pub fn draw_1040_7bb2(param_1: *mut u32) {
    let ppcVar1: fn();
    let mut hdc: u16;
    let BVar2: bool;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut obj_handle: HPEN16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut HVar6: HGDIOBJ16;
    let mut obj_handle_01: HANDLE16;
    let mut count: u16;
    let mut iVar7: i32;
    let rect: *mut RECT16;
    let mut unaff_SS: HWND16;
    let mut uVar8: u32;
    let mut dVar9: u32;
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

    rect = (param_1 >> 0x10);
    iVar7 = param_1;
    BVar2 = IsIconic16((iVar7 + 6));
    if (BVar2 == 0) {
        local_4 = GetWindowDC16((iVar7 + 6));
        let param_1_val = unsafe { *param_1 };
        ppcVar1 = (param_1_val + 0x68);
        _local_8 = (**ppcVar1)(offset, iVar7, rect, (iVar7 + 0x6e));
        if (_local_8 != 0) {
            local_a = realize_palette_1008_4e08(_local_8, 0xfc, unaff_SS);
            GetWindowRect16(CONCAT22(&local_12, 0x1008), unaff_SS);
            iVar3 = (local_e - local_12) + -1;
            iVar4 = (local_c - local_10) + -1;
            iVar5 = (-((iVar7 + 0x60) == 0) & 0x1e) + 0x25;
            obj_handle = CreatePen16(CONCAT22(0x100, iVar5), 0, 0);
            obj_handle_00 = SelectObject16(obj_handle, local_4);
            MoveTo16(0, 0, local_4);
            LineTo16(0, iVar3, local_4);
            LineTo16(iVar4, iVar3, local_4);
            LineTo16(iVar4, 0, local_4);
            LineTo16(0, 0, local_4);
            uVar8 = GetWindowLong16(-0x10, (iVar7 + 6));
            if (((uVar8 & 0x800000) != 0) && ((uVar8 & 0x400000) != 0)) {
                iVar4 = (iVar7 + 0x62) - (iVar7 + 0x66);
                MoveTo16(iVar4, 0, local_4);
                LineTo16(iVar4, iVar3, local_4);
                hdc = local_4;
                (iVar7 + 0x7a) = (iVar7 + 100);
                (iVar7 + 0x7c) = (iVar7 + 0x66);
                (iVar7 + 0x7e) = iVar3;
                (iVar7 + 0x80) = (iVar7 + 0x62) - (iVar7 + 0x66);
                HVar6 = GetStockObject16(4);
                FillRect16(HVar6, rect, hdc);
                if ((iVar7 + 0x76) != 0) {
                    draw_1040_82ee(iVar7, rect, local_4);
                }
                if (*(iVar7 + 0x10) != '\0') {
                    obj_handle_01 = GetProp16(s_hfont_1050_5de9, (iVar7 + 6));
                    if (obj_handle_01 != 0) {
                        SelectObject16(obj_handle_01, local_4);
                    }
                    SetBkColor16(0, local_4);
                    SetTextColor16(CONCAT22(0x100, iVar5), local_4);
                    count = lstrlen16((param_1 & 0xffff0000 | ZEXT24((iVar7 + 0x10))));
                    dVar9 = GetTextExtent16(count, (param_1 & 0xffff0000 | local_4), local_4);
                    iVar3 = (((iVar7 + 0x7e) - (iVar7 + 0x7a)) / 2 - dVar9 / 2) + (iVar7 + 0x7a);
                    HVar6 = ((iVar7 + 0x80) - (iVar7 + 0x7c)) / 2 - (dVar9 >> 0x10) / 2;
                    local_2c = ((param_1 & 0xffff0000) >> 0x10);
                    TextOut16(
                        local_2c,
                        (param_1 & 0xffff0000 | local_4),
                        HVar6,
                        iVar3,
                        local_4,
                    );
                    if (iVar3 != 0) {
                        SelectObject16(HVar6, local_4);
                    }
                }
            }
            ppcVar1 = (param_1_val + 100);
            (**ppcVar1)(offset, iVar7, rect, _local_8, local_4);
            local_a = SelectPalette16(0, local_a, local_4);
            DeleteObject16(local_a);
            SelectObject16(obj_handle_00, local_4);
            DeleteObject16(obj_handle);
        }
        ReleaseDC16(local_4, (iVar7 + 6));
        return;
    }
    return;
}

pub fn draw_1040_5a06(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let ppcVar4: fn();
    let h_palette: *mut u16;
    let mut iVar5: u16;
    let mut obj_handle: HPEN16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut HVar6: HGDIOBJ16;
    let mut y: i32;
    let mut y_00: i32;
    let mut x: i32;
    let mut h_gdi_obj: HPALETTE16;
    let mut uVar7: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut uVar8: u16;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut unaff_CS: u16;
    let mut uVar11: u16;
    let mut unaff_SS: HWND16;
    let puVar12: *mut u16;
    let mut local_62: u32;
    let mut local_5e: u32;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: PAINTSTRUCT16;
    let mut local_a: [u8; 8];

    uVar10 = (param_1 >> 0x10);
    iVar9 = param_1;
    GetWindowRect16(
        CONCAT13((local_a >> 8), CONCAT12(local_a, unaff_CS)),
        unaff_SS,
    );
    local_2c = BeginPaint16(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_2a)),
        (iVar9 + 6),
    );
    uVar2 = (_PTR_LOOP_1050_4230 + 0xe);
    h_palette = &local_2c;
    uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
    realize_palette_1008_4e08(uVar2, (uVar2 >> 0x10), h_palette, unaff_SS);
    uVar7 = 0;
    uVar8 = 0;
    local_36 = 0;
    if ((iVar9 + 0x98) != 0) {
        mixed_fn_1010_830a(_g_struct_73_1050_14cc, (iVar9 + 0x98));
        local_36 = CONCAT22(extraout_DX, uVar8);
        uVar7 = process_struct_1008_4772(CONCAT22(extraout_DX, uVar8));
        if (((uVar7 >> 0x10) | (uVar7 & 0xffff)) == 0) {
            uVar7 = uVar7 & 0xffff;
            if (local_36 != 0) {
                uVar7 = local_36;
                if (local_36 != 0) {
                    ppcVar4 = local_36;
                    (**ppcVar4)(&PTR_LOOP_1050_1008, uVar8, extraout_DX, 1);
                    uVar7 = local_36;
                }
            }
            uVar8 = uVar7;
            mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x4d);
            local_36 = CONCAT22(extraout_DX_00, uVar8);
        }
        puVar12 = &local_2c;
        uVar11 = SUB42(offset, 0);
        iVar5 = GetSystemMetrics16(4);
        uVar7 = -(iVar5 + -0x23);
        ppcVar4 = (local_36 + 4);
        (**ppcVar4)(
            0x38,
            local_36,
            (local_36 >> 0x10),
            -(iVar5 + -0x23),
            puVar12,
        );
    }
    if (local_36 != 0) {
        uVar7 = local_36;
        if (local_36 != 0) {
            ppcVar4 = local_36;
            (**ppcVar4)(uVar11, local_36, (local_36 >> 0x10), 1);
            uVar7 = local_36;
        }
    }
    uVar8 = uVar7;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, (iVar9 + 0x96));
    local_36 = CONCAT22(extraout_DX_01, uVar8);
    puVar12 = &local_2c;
    iVar5 = GetSystemMetrics16(4);
    uVar2 = local_36;
    ppcVar4 = uVar2 + 2;
    (**ppcVar4)(0x38, uVar8, extraout_DX_01, -(iVar5 + -0x23), puVar12);
    if (local_36 != 0) {
        if (local_36 != 0) {
            ppcVar4 = uVar2;
            (**ppcVar4)(offset, uVar8, extraout_DX_01, 1);
        }
    }
    obj_handle = CreatePen16(0x1000025, 0, 0);
    obj_handle_00 = SelectObject16(obj_handle, local_2c);
    HVar6 = obj_handle_00;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x4f);
    local_36 = CONCAT22(extraout_DX_02, HVar6);
    uVar7 = process_struct_1008_4772(CONCAT13(
        (extraout_DX_02 >> 8),
        CONCAT12(extraout_DX_02, HVar6),
    ));
    uVar8 = (uVar7 >> 0x10);
    uVar2 = (uVar7 + 4);
    uVar3 = (uVar7 + 8);
    iVar5 = GetSystemMetrics16(4);
    y = -(iVar5 + -0xc1);
    iVar5 = GetSystemMetrics16(4);
    local_48._0_2_ = uVar3;
    y_00 = 0xc5 - (iVar5 - local_48);
    MoveTo16(y, 0x82, local_2c);
    local_44._0_2_ = uVar2;
    x = local_44 * 10 + 0x85;
    LineTo16(y, x, local_2c);
    LineTo16(y_00, x, local_2c);
    LineTo16(y_00, 0x82, local_2c);
    LineTo16(y, 0x82, local_2c);
    local_52 = 0;
    let pu_var1_val = unsafe { *puVar1 };
    while (
        puVar1 = (iVar9 + 0x94),
        local_52 <= pu_var1_val && pu_var1_val != local_52,
    ) {
        puVar12 = &local_2c;
        iVar5 = GetSystemMetrics16(4);
        ppcVar4 = (local_36 + 4);
        (**ppcVar4)(offset, HVar6, extraout_DX_02, -(iVar5 + -0xc4), puVar12);
        local_52 = local_52 + 1;
    }
    if (local_36 != 0) {
        if (local_36 != 0) {
            ppcVar4 = local_36;
            (**ppcVar4)(offset, HVar6, extraout_DX_02, 1, local_36, local_36);
        }
    }
    SelectObject16(obj_handle_00, local_2c);
    DeleteObject16(obj_handle);
    h_gdi_obj = SelectPalette16(0, h_palette, local_2c);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_2a, unaff_SS);
    return;
}

pub fn draw_1040_27cc(param_1: *mut u32, param_2: HWND16, uparam_3: i32, param_4: HDC16) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut HVar3: HBRUSH16;
    let mut iVar4: u16;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut unaff_CS: u16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut hdc: HDC16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 4) == 0) {
        unaff_CS = SUB42(offset, 0);
        HVar3 = CreateSolidBrush16(0);
        (iVar5 + 4) = HVar3;
    }
    if (_PTR_LOOP_1050_5cf8 == 0) {
        let param_1_val = unsafe { *param_1 };
        ppcVar1 = (param_1_val + 0x68);
        uVar7 = (**ppcVar1)(unaff_CS, iVar5, uVar6, (iVar5 + 0x6e));
        uVar7 = pass1_1008_4d72(uVar7);
        uVar8 = (uVar7 >> 0x10);
        iVar2 = uVar7;
        _PTR_LOOP_1050_5cf8 = CONCAT22(
            CONCAT11(2, *(iVar2 + 0x94)),
            CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)),
        );
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return 0;
        }
        uVar8 = SUB42(s_fem67_wav_1050_2842 + 8, 0);
        iVar4 = GetDlgCtrllID16(param_2);
        if (((iVar5 + 0x94) != 0) && (iVar4 == 0xfb2)) {
            uVar9 = 0xff;
            hdc = 0;
            // goto LAB_1040_286e;
        }
    }
    uVar8 = _PTR_LOOP_1050_5cf8;
    uVar9 = (_PTR_LOOP_1050_5cf8 >> 0x10);
    hdc = param_4;
    // LAB_1040_286e:
    SetTextColor16(CONCAT22(uVar9, uVar8), hdc);
    SetBkColor16(0x1000000, param_4);
    return CONCAT22(0x1050, (iVar5 + 4));
}

pub fn draw_1040_21d6(param_1: u32) {
    let uVar1: u8;
    let uVar2: u8;
    let uVar3: u8;
    let mut uVar4: u32;
    let ppcVar5: fn();
    let mut uVar6: u16;
    let mut iVar7: i32;
    let h_palette: *mut u16;
    let mut obj_handle: HANDLE16;
    let mut h_gdi_obj: HPALETTE16;
    let mut iVar8: i32;
    let mut count: u16;
    let mut unaff_SS: HWND16;
    let mut uVar9: u32;
    let mut color: COLORREF;
    let mut color_00: COLORREF;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    count = (param_1 >> 0x10);
    iVar8 = param_1;
    local_24 = BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar8 + 6));
    uVar9 = (_PTR_LOOP_1050_4230 + 0xe);
    h_palette = &local_24;
    realize_palette_1008_4e08(uVar9, h_palette, unaff_SS);
    uVar4 = (iVar8 + 0x8e);
    ppcVar5 = ((iVar8 + 0x8e) + 4);
    (**ppcVar5)(
        &PTR_LOOP_1050_1008,
        uVar4,
        (uVar4 >> 0x10),
        0xffffffff,
        &local_24,
    );
    uVar9 = pass1_1008_4d72(uVar9);
    uVar6 = (uVar9 >> 0x10);
    iVar7 = uVar9;
    uVar1 = *(iVar7 + 0x3e5);
    uVar2 = *(iVar7 + 0x3e6);
    uVar3 = *(iVar7 + 0x3e4);
    color = SetBkColor16(0, local_24);
    color_00 = SetTextColor16(
        CONCAT22(CONCAT11(2, uVar3), CONCAT11(uVar1, uVar2)),
        local_24,
    );
    local_3e = 0;
    obj_handle = GetProp16(s_hfont_1050_5ced, (iVar8 + 6));
    if (obj_handle != 0) {
        local_3e = SelectObject16(obj_handle, local_24);
    }
    uVar4 = (iVar8 + 0xa2);
    DrawText16(
        0x10,
        (iVar8 + 0x92),
        count,
        CONCAT13((uVar4 >> 8), CONCAT12(uVar4, 0xffff)),
        (uVar4 >> 0x10),
    );
    SetTextColor16(
        CONCAT13(
            2,
            CONCAT12(*(iVar7 + 0x94), CONCAT11(*(iVar7 + 0x95), *(iVar7 + 0x96))),
        ),
        local_24,
    );
    uVar4 = (iVar8 + 0xa6);
    DrawText16(
        0x10,
        (iVar8 + 0x9a),
        count,
        CONCAT13((uVar4 >> 8), CONCAT12(uVar4, 0xffff)),
        (uVar4 >> 0x10),
    );
    if (obj_handle != 0) {
        SelectObject16(local_3e, local_24);
    }
    SetBkColor16(color, local_24);
    SetTextColor16(color_00, local_24);
    h_gdi_obj = SelectPalette16(0, h_palette, local_24);
    DeleteObject16(h_gdi_obj);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_1038_9dcc(param_1: u32, param_2: i32, uparam_3: i32, param_4: HDC16) {
    let puVar1: *mut u32;
    let mut bVar2: bool;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let mut solid_brush: u16;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0x8e) == 0) {
        solid_brush = CreateSolidBrush16(0);
        (iVar5 + 0x8e) = solid_brush;
    }
    if (_PTR_LOOP_1050_5b64 == 0) {
        uVar7 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar3 = (uVar7 >> 0x10);
        iVar4 = uVar7;
        _PTR_LOOP_1050_5b64 = CONCAT12(*(iVar4 + 0x94), CONCAT11(*(iVar4 + 0x95), *(iVar4 + 0x96)));
        PTR_LOOP_1050_5b68 = CONCAT11(*(iVar4 + 0x3e5), *(iVar4 + 0x3e6));
        PTR_LOOP_1050_5b6a = *(iVar4 + 0x3e4);
    }
    if (5 < param_3) {
        if (param_3 != 6) {
            return;
        }
        bVar2 = false;
        local_e = 0;
        let pu_var_1 = unsafe { *puVar1 };
        while (
            puVar1 = (iVar5 + 0x128),
            local_e <= pu_var_1 && pu_var_1 != local_e,
        ) {
            if ((iVar5 + 0x94 + local_e * 2) == param_2) {
                bVar2 = true;
                break;
            }
            local_e = local_e + 1;
        }
        if (bVar2) {
            PTR_LOOP_1050_5b64 = PTR_LOOP_1050_5b68;
            PTR_LOOP_1050_5b66 = PTR_LOOP_1050_5b6a;
        }
    }
    SetTextColor16(CONCAT22(PTR_LOOP_1050_5b66, PTR_LOOP_1050_5b64), param_4);
    SetBkColor16(0x1000000, param_4);
    return;
}

pub fn get_dc_1020_921c(param_1: *mut astruct_622, param_2: u16) {
    let local_AX_99: *mut astruct_670;
    let mut uVar1: u16;
    let local_struct_1: *mut astruct_622;
    let local_ES_4: *mut astruct_622;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffe2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (in_stack_0000ffe2 >> 0x10);
    local_ES_4 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.u16_0x0 = s_1_1050_389a;
    local_struct_1.u16_0x2 = &PTR_LOOP_1050_1008;
    param_1.u16_0x0 = (s_18_2_1050_3aa5 + 3);
    local_struct_1.u16_0x2 = &PTR_LOOP_1050_1008;
    local_struct_1.window_handle_0x4 = param_2;
    param_1.u16_0x0 = s_0_020_1050_3ab0;
    local_struct_1.u16_0x2 = &PTR_LOOP_1050_1008;
    &local_struct_1.u16_0x6 = 0;
    local_struct_1.field_0xa = 0;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0xe = 0;
    local_struct_1.field_0x10 = 0;
    local_struct_1.i16_0x12 = 0;
    param_1.u16_0x0 = 0x96c8;
    local_struct_1.u16_0x2 = 0x1020;
    local_AX_99 = GetDC16(local_struct_1.window_handle_0x4);
    local_struct_1.field_0xa = local_AX_99;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar1, 0x48));
    uVar1 = (ppVar2 >> 0x10);
    local_struct_1.field_0xc = (ppVar2 + 10);
    local_struct_1.field_0xe = (ppVar2 + 0xc);
    return;
}

pub fn select_and_delete_palette_1020_92c4(in_astruct_1: *mut astruct_44) {
    let mut h_gdi_obj: HPALETTE16;
    let local_astruct_376: *mut astruct_376;
    let local_struct_376_hi: *mut astruct_376;

    local_struct_376_hi = (in_astruct_1 >> 0x10);
    local_astruct_376 = in_astruct_1;
    in_astruct_1.ptr_a_lo = 0x96c8;
    local_astruct_376.ptr_a_hi = 0x1020;
    if (local_astruct_376.palette_handle_x12 != 0) {
        h_gdi_obj = SelectPalette16(
            0,
            local_astruct_376.palette_handle_x12,
            local_astruct_376.dc_handle_x0a,
        );
        DeleteObject16(h_gdi_obj);
    }
    in_astruct_1.ptr_a_lo = s_0_020_1050_3ab0;
    local_astruct_376.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_astruct_1.ptr_a_lo = s_1_1050_389a;
    local_astruct_376.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn draw_1020_9312(param_1: u32) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: HWND16;
    let mut local_26: u32;
    let mut local_22: PAINTSTRUCT16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar4 + 4));
    uVar3 = (iVar4 + 6);
    puVar1 = (uVar3 + 10);
    let pu_var1_val = unsafe { *puVar1 };
    ppcVar2 = (pu_var1_val + 4);
    ppcVar2(
        offset,
        puVar1,
        (puVar1 >> 0x10),
        0,
        param_1 & 0xffff0000 | (iVar4 + 10),
    );
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_1020_9364(param_1: *mut astruct_622) {
    let pi_var1: *mut i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let mut HVar4: HBRUSH16;
    let mut x: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut unaff_SS: HANDLE16;
    let mut uVar9: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_42: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 4];
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    GetClientRect16(CONCAT22(unaff_SS, &local_a), (iVar6 + 4));
    local_12 = local_a;
    local_e = local_6;
    local_14 = u16_1050_4216;
    local_16 = WORD_1050_422c;
    local_1a = _PTR_PTR_BYTE_1050_0009_1050_4172_1050_4212;
    local_1e = _PTR_PTR_s_ibr_1050_004f_1050_41b2_1050_4218;
    local_22 = _PTR_PTR_s_ew_failed_in_Op_Op_1050_0021_1050_41da_1050_421c;
    local_26 = _PTR_PTR_s_n_Op_Op__ResLibr_1050_0041_1050_4202_1050_4220;
    local_2a = _PTR_WORD_1050_419a_1050_4224;
    local_2e = _PTR_PTR_1050_4228;
    uVar3 = (iVar6 + 6);
    local_30 = (uVar3 + 0x12);
    local_3a = 9;
    while {
        uVar9 = local_30;
        local_32 = CreatePen16((local_3a * 4 + local_22), 0, 0);
        local_34 = SelectObject16(local_32, uVar9);
        MoveToEx16(
            CONCAT22(unaff_SS, local_38),
            (local_3a * 2 + local_1a),
            local_a,
            local_30,
        );
        LineTo16(local_1a + local_3a * 2, local_6, local_30);
        iVar5 = (local_14 - local_3a) * 2;
        MoveToEx16(
            CONCAT22(unaff_SS, local_38),
            (iVar5 + local_1a),
            local_a,
            local_30,
        );
        LineTo16(local_1a + iVar5, local_6, local_30);
        SelectObject16(local_34, local_30);
        DeleteObject16(local_32);
        local_3a = local_3a - 1;
        local_3a < 0x8000
    } {}
    HVar4 = CreateSolidBrush16(0x2000000);
    uVar8 = (local_1a >> 0x10);
    local_a = CONCAT22((local_1a + 0x12) + 1, local_a);
    uVar2 = (local_1a + 0x14);
    local_e = local_e & 0xffff | uVar2 << 0x10;
    local_6 = CONCAT22(uVar2, local_6);
    FillRect16(HVar4, &local_a, unaff_SS);
    DeleteObject16(HVar4);
    local_3a = 1;
    local_3e = 8;
    while (local_3a < 10) {
        HVar4 = CreateSolidBrush16((local_1e + local_3e * 4 + 4));
        local_6 = local_6 & 0xffff | (local_a._2_2_ - 1) << 0x10;
        local_12 = local_12 & 0xffff | (local_e._2_2_ + 1) << 0x10;
        uVar8 = (local_1a >> 0x10);
        local_a = local_a & 0xffff | ((local_3e * 2 + local_1a) + 1) << 0x10;
        local_e = local_e & 0xffff | *(local_3a * 2 + local_1a + 0x14) << 0x10;
        FillRect16(HVar4, &local_a, unaff_SS);
        FillRect16(HVar4, &local_12, unaff_SS);
        DeleteObject16(HVar4);
        local_3a = local_3a + 1;
        local_3e = local_3e - 1;
    }
    HVar4 = CreateSolidBrush16(local_1e);
    local_a = local_a & 0xffff;
    local_6 = local_6 & 0xffff | *local_1a << 0x10;
    local_12 = local_12 & 0xffff | ((local_14 * 2 + local_1a) + 1) << 0x10;
    local_e = local_e & 0xffff | *(iVar6 + 0xe) << 0x10;
    FillRect16(HVar4, &local_a, unaff_SS);
    FillRect16(HVar4, &local_12, unaff_SS);
    DeleteObject16(HVar4);
    local_3a = 3;
    while {
        uVar9 = local_30;
        local_32 = CreatePen16((local_3a * 4 + local_26), 0, 0);
        local_34 = SelectObject16(local_32, uVar9);
        iVar5 = local_3a * 2;
        x = (iVar5 + local_2a) + local_a;
        uVar8 = (local_2e >> 0x10);
        pi_var1 = (iVar5 + local_2e);
        MoveToEx16(
            CONCAT22(unaff_SS, local_38),
            ((iVar5 + local_2e) * 2 + local_1a),
            x,
            local_30,
        );
        let pi_var1_val = unsafe { *pi_var1 };
        LineTo16((local_14 - pi_var1_valv) * 2 + local_1a, x, local_30);
        iVar5 = ((local_16 - local_3a) * 2 + local_2a) + local_a;
        MoveToEx16(
            CONCAT22(unaff_SS, local_38),
            (pi_var1_val * 2 + local_1a),
            iVar5,
            local_30,
        );
        LineTo16((local_14 - pi_var1_val) * 2 + local_1a, iVar5, local_30);
        SelectObject16(local_34, local_30);
        DeleteObject16(local_32);
        local_3a = local_3a - 1;
        local_3a < 0x8000
    } {}
    (iVar6 + 0x10) = 0;
}

pub fn call_invalidate_rect_1020_8bcc(in_struct_1: *mut astruct_693) {
    let paVar1: *mut astruct_197;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let paVar4: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let local_struct_1: *mut astruct_693;
    let local_BX_310: *mut astruct_694;
    let local_struct_1_hi: *mut astruct_693;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 30];
    let mut local_3a: [u8; 38];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let local_struct_2: *mut astruct_538;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x4 != 0) {
        local_struct_2 = local_struct_1.struct538_ptr_0x22;
        local_6 = (local_struct_2 + 10);
        _local_a = pass1_1018_268e(local_struct_1.struct538_ptr_0x22);
        local_struct_2 = local_struct_1.struct538_ptr_0x22;
        local_c = (local_struct_2 + 0x16);
        if (local_struct_1.field_0xc == 0) {
            _local_14 = process_struct_1008_4772(_local_a);
            paVar4 = (_local_14 >> 0x10);
            uVar2 = _local_14;
            process_struct_1000_179c(0x14, paVar4);
            _local_5c = CONCAT22(paVar4, uVar2);
            if ((paVar4 | uVar2) == 0) {
                local_struct_1.field_0xc = 0;
            } else {
                uVar3 = in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16;
                uVar5 = (_local_14 >> 0x10);
                process_struct_1008_50c2(
                    _local_5c,
                    (_local_14 + 8),
                    (_local_14 + 4),
                    uVar3,
                    local_6,
                );
                paVar1 = local_struct_1.field_0xc;
                uVar2 = uVar3;
                paVar1 = uVar2;
                (paVar1 + 2) = extraout_DX;
            }
            pass1_1008_5134(local_struct_1.field_0xc);
            mixed_fn_1010_830a(_g_struct_73_1050_14cc, 2);
            process_struct_1008_48fe(
                CONCAT22(unaff_SS, local_3a),
                1,
                CONCAT22(extraout_DX_00, uVar2),
            );
            pass1_1008_3f92(CONCAT22(unaff_SS, local_58), CONCAT22(unaff_SS, local_3a));
            _local_14 = process_struct_1008_4772(CONCAT22(unaff_SS, local_58));
            paVar4 = (_local_14 >> 0x10);
            uVar2 = _local_14;
            process_struct_1000_179c(0x14, paVar4);
            _local_5c = CONCAT22(paVar4, uVar2);
            if ((paVar4 | uVar2) == 0) {
                paVar1 = local_struct_1.field_0xc;
                (paVar1 + 4) = 0;
            } else {
                uVar3 = in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16;
                uVar5 = (_local_14 >> 0x10);
                process_struct_1008_50c2(
                    _local_5c,
                    (_local_14 + 8),
                    (_local_14 + 4),
                    uVar3,
                    local_6,
                );
                paVar1 = local_struct_1.field_0xc;
                uVar5 = (paVar1 >> 0x10);
                local_BX_310 = paVar1;
                local_BX_310.field_0x4 = uVar3;
                local_BX_310.field_0x6 = extraout_DX_01;
            }
            paVar1 = local_struct_1.field_0xc;
            pass1_1008_5134((paVar1 + 4));
            process_struct_1008_41bc(CONCAT22(unaff_SS, local_58));
            close_file_1008_496c(local_3a);
        }
        paVar1 = local_struct_1.field_0xc;
        pass1_1008_5236((paVar1 + 4));
        pass1_1008_5236(local_struct_1.field_0xc);
        pass1_1008_4480(
            local_6,
            (in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16),
            _local_a,
        );
        invalidate_rect_1020_8d90(in_struct_1, local_c, local_6, (local_6 >> 0x10));
    }
    return;
}

pub fn invalidate_rect_1020_8d90(param_1: u32, param_2: u16, param_3: u32) {
    let mut u_var1: u32;
    let mut in_DX: i32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let unaff_SS: *mut char;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 40];
    let mut local_10: [u8; 10];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    local_6 = pass1_1018_266a((iVar2 + 0x22));
    if (local_6 != 0x0) {
        pass1_1018_265c((iVar2 + 0x22));
        if ((in_DX | local_6) != 0) {
            string_fn_1000_3f9c(
                local_10,
                unaff_SS,
                s__03ld_1050_442a,
                &g_alloc_addr_1050_1050,
                local_6,
            );
            uVar1 = (iVar2 + 0x22);
            uVar1 = (uVar1 + 0xe);
            draw_1008_4f20(
                local_38,
                unaff_SS,
                uVar1,
                (uVar1 >> 0x10),
                0x25,
                local_10,
                unaff_SS,
            );
            pass1_1008_4480(
                param_3,
                (param_1 & 0xffff0000 | (iVar2 + 0x1c)),
                CONCAT22(unaff_SS, local_38),
            );
            _local_3c = process_struct_1008_4772(CONCAT22(unaff_SS, local_38));
            pass1_1008_3e94(
                (iVar2 + 0x1c),
                CONCAT22(unaff_SS, &local_40),
                CONCAT22(unaff_SS, &local_3e),
            );
            local_48 = local_3e;
            local_46 = local_40;
            uVar3 = (_local_3c >> 0x10);
            local_44 = local_3e + (_local_3c + 4);
            local_42 = local_40 + (_local_3c + 8);
            InvalidateRect16(0, &local_48, unaff_SS);
            process_struct_1008_41bc(CONCAT22(unaff_SS, local_38));
        }
    }
    return;
}

pub fn realize_palette_1020_8128(param_1: u32, param_2: i32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u8;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut extraout_DX: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_22: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: [u8; 8];
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        uVar2 = (iVar6 + 0xe6);
        puVar5 = (uVar2 + 10);
        let pu_var5_val = unsafe { *puVar5 };
        ppcVar1 = (pu_var5_val + 0x18);
        local_6 = puVar5;
        (**ppcVar1)();
        local_8 = puVar5;
        UnrealizeObject16(local_8);
        uVar2 = (iVar6 + 0xe6);
        local_a = (uVar2 + 0x14);
        RealizePalette16(local_a);
        pass1_1008_57a4(
            CONCAT22(unaff_SS, local_12),
            param_1 & 0xffff0000 | (iVar6 + 0xd2),
        );
        loop {
            puVar3 = local_12;
            pass1_1008_5b12(CONCAT22(unaff_SS, puVar3));
            if ((extraout_DX | puVar3) == 0) {
                break;
            }
            uVar2 = (puVar3 + 4);
            uVar7 = (puVar3 + 6);
            puVar4 = uVar2;
            let pu_var4_1 = unsafe { *puVar4 };
            ppcVar1 = (pu_var4_1 + 0x90);
            (**ppcVar1)(&PTR_LOOP_1050_1008, puVar4, uVar7, 1, uVar2);
        }
    }
    return;
}

pub fn draw_func_1020_81c0(param_1: u32) {
    let mut u_var1: u32;
    let mut hdc: HDC16;
    let mut h_palette: HDC16;
    let mut h_palette_00: HPALETTE16;
    let mut uVar2: u16;
    let mut h_dc: HDC16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar2 = (_PTR_LOOP_1050_4230 >> 0x10);
    uVar1 = (_PTR_LOOP_1050_4230 + 0xe);
    local_6._0_2_ = uVar1;
    if (((_PTR_LOOP_1050_4230 + 0x10) | local_6) == 0) {
        return;
    }
    hdc = GetDC16((param_1 + 8));
    h_palette = hdc;
    h_dc = hdc;
    create_palette_1008_4e38(uVar1);
    h_palette_00 = SelectPalette16(0, h_palette, h_dc);
    RealizePalette16(hdc);
    SelectPalette16(1, h_palette_00, hdc);
    RealizePalette16(hdc);
    DeleteObject16(h_palette);
    if (0 < h_palette) {
        InvalidateRect16(1, 0x0, 0);
    }
    return;
}

pub fn destroy_win_1020_8250(param_1: u32) {
    let BVar1: bool;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xec) != 0) {
        BVar1 = IsWindow16((iVar2 + 0xec));
        if (BVar1 != 0) {
            DestroyWindow16((iVar2 + 0xec));
            (iVar2 + 0xec) = 0;
        }
    }
    return;
}

pub fn load_cursor_1020_7f7a(in_struct_65_1: *mut astruct_65, param_2: u32, param_3: u32) {
    let mut uVar1: u16;
    let mut h_gdi_obj: HGDIOBJ16;
    let mut h_cursor: HCURSOR16;
    let local_AX_192: *mut astruct_590;
    let struct_65_1: *mut astruct_65;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut uVar4: u16;
    let mut in_stack_0000ffe8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_2 >> 0x10);
    load_cursor_1008_61b2(in_struct_65_1, param_2, uVar4, param_3);
    uVar2 = (in_struct_65_1 >> 0x10);
    struct_65_1 = in_struct_65_1;
    struct_65_1.ptr_b_lo = s_1_1050_389a;
    struct_65_1.ptr_b_hi = &PTR_LOOP_1050_1008;
    struct_65_1.ptr_b_lo = (s_18_2_1050_3aa5 + 3);
    struct_65_1.ptr_b_hi = &PTR_LOOP_1050_1008;
    &struct_65_1.u16_xe6 = 0;
    struct_65_1.u16_xea = 0;
    struct_65_1.u16_xec = 0;
    in_struct_65_1.ptr_a_lo = 0x82bc;
    struct_65_1.ptr_a_hi = 0x1020;
    struct_65_1.ptr_b_lo = 0x8358;
    struct_65_1.ptr_b_hi = 0x1020;
    copy_string_1000_3d3e(
        (in_struct_65_1 & 0xffff0000 | ZEXT24(&struct_65_1.char_ptr_x5b)),
        s_VrMode_1050_4422,
    );
    h_gdi_obj = GetStockObject16(5);
    struct_65_1.h_gdi_obj_xc6 = h_gdi_obj;
    h_cursor = LoadCursor16(0x7f00, 0);
    struct_65_1.h_cursor_xc4 = h_cursor;
    struct_65_1.u16_xc8 = (s_576_bmp_1050_2027 + 1);
    struct_65_1.u32_xac = 0x47000000;
    struct_65_1.astruct_590_ptr_xbc = (param_3 + 8);
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe8, 0x48));
    uVar1 = (ppVar3 >> 0x10);
    struct_65_1.u16_xb4 = 0;
    struct_65_1.u16_xb6 = 0;
    struct_65_1.u16_xb8 = (ppVar3 + 10);
    struct_65_1.u16_xba = (ppVar3 + 0xc);
    &struct_65_1.u32_xca = uVar4;
    reg_class_1008_96d2(in_struct_65_1, in_stack_0000ffe8);
    return;
}

pub fn draw_1020_7cc8(param_1: *mut u8) {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut hdc: u16;
    let mut x: i32;
    let mut iVar4: i32;
    let mut h_brush: HGDIOBJ16;
    let mut width: u16;
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let pcVar5: *mut char;
    let mut iVar6: u16;
    let puVar7: *mut u32;
    let mut y: i32;
    let mut extraout_DX: u16;
    let local_BX_4: *mut astruct_34;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut h_wnd: u16;
    let mut dVar11: u32;
    let mut h_dc: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut uStack58: u16;
    let mut uStack56: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
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
    let mut local_a: u32;
    let mut window_dc: u16;
    let mut b_result: u16;

    uVar9 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    b_result = IsIconic16(local_BX_4.h_window);
    if ((b_result == 0) || (PTR_LOOP_1050_0010 != 0x0)) {
        window_dc = GetWindowDC16(local_BX_4.h_window);
        local_a = (_PTR_LOOP_1050_4230 + 0xe);
        local_c = &window_dc;
        realize_palette_1008_4e08(local_a, (local_a >> 0x10), local_c, h_wnd);
        GetWindowRect16(CONCAT22(&local_14, 0x1008), h_wnd);
        hdc = window_dc;
        x = (local_10 - local_14) + -1;
        y = (local_e - local_12) + -1;
        iVar1 = local_BX_4.field_0x12;
        iVar4 = y;
        if (b_result == 0) {
            iVar4 = local_BX_4.field_0xe - local_BX_4.field_0x12;
        }
        h_brush = GetStockObject16(4);
        FillRect16(h_brush, h_wnd, hdc);
        uVar3 = local_BX_4.field_0x6;
        uVar10 = (uVar3 >> 0x10);
        iVar8 = uVar3;
        puVar7 = (iVar8 + 0xe0);
        iVar6 = (iVar8 + 0xe2);
        width = puVar7;
        let pu_var7_val = unsafe { *puVar7 };
        ppcVar2 = (pu_var7_val + 0x24);
        ppcVar2(offset, width, iVar6, 0, 0);
        iVar8 = (-(puVar7 == 0) & 0x1e) + 0x25;
        pen = CreatePen16(CONCAT22(0x100, iVar8), width, iVar6);
        pen_obj_handle = SelectObject16(pen, window_dc);
        MoveTo16(0, 0, window_dc);
        LineTo16(0, x, window_dc);
        LineTo16(y, x, window_dc);
        LineTo16(y, 0, window_dc);
        pcVar5 = LineTo16(0, 0, window_dc);
        if (b_result == 0) {
            MoveTo16(local_BX_4.field_0xe - local_BX_4.field_0x12, 0, window_dc);
            pcVar5 = LineTo16(x, x, window_dc);
        }
        uVar3 = local_BX_4.field_0x6;
        uStack58 = uVar3;
        uStack56 = (uVar3 >> 0x10);
        ppcVar2 = (local_BX_4.field_0x6 + 0x18);
        ppcVar2(offset, uStack58, uStack56);
        let pc_var5_val = unsafe { *pcVar5 };
        if (pc_var5_val != '\0') {
            SetBkColor16(0, window_dc);
            SetTextColor16(CONCAT22(0x100, iVar8), window_dc);
            iVar6 = lstrlen16(CONCAT22(extraout_DX, pcVar5));
            dVar11 = GetTextExtent16(iVar6, CONCAT22(extraout_DX, pcVar5), window_dc);
            iVar8 = (dVar11 >> 0x10);
            if (b_result == 0) {
                local_42 = (iVar4 - iVar1) / 2 - iVar8 / 2;
                h_dc = x / 2 - dVar11 / 2;
            } else {
                local_42 = y / 2 - iVar8 / 2;
                h_dc = 2;
            }
            local_3e._0_1_ = extraout_DX;
            local_3e._1_1_ = (extraout_DX >> 8);
            TextOut16(
                local_42,
                CONCAT13(local_3e._1_1_, CONCAT12(local_3e, pcVar5)),
                local_42,
                h_dc,
                window_dc,
            );
        }
        local_c = SelectPalette16(0, local_c, window_dc);
        DeleteObject16(local_c);
        SelectObject16(pen_obj_handle, window_dc);
        DeleteObject16(pen);
        ReleaseDC16(window_dc, local_BX_4.h_window);
    }
    return;
}

pub fn get_sys_metrics_1020_7a50(param_1: *mut astruct_652) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let b_var3: bool;
    let mut iVar4: u16;
    let mut iVar5: u16;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut unaff_SS: HWND16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    b_var3 = IsIconic16((iVar6 + 8));
    if (b_var3 == 0) {
        GetWindowRect16(CONCAT22(&local_a, 0x1538), unaff_SS);
        local_6 = local_6 - local_a;
        local_4 = local_4 - local_8;
        iVar4 = GetSystemMetrics16(5);
        iVar5 = GetSystemMetrics16(6);
        local_a = local_a + iVar4 * 2;
        local_8 = local_8 + iVar5 * 2;
    }
    uVar1 = (iVar6 + 0xe0);
    ppcVar2 = ((iVar6 + 0xe0) + 0x14);
    ppcVar2(offset, uVar1, (uVar1 >> 0x10), &local_a);
    return;
}

pub fn track_popup_menu_1020_7ad2(param_1: u32, param_2: u16) {
    let mut HVar1: HMENU16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: HWND16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0xee) != 0) && ((iVar2 + 0xec) == 0)) {
        HVar1 = LoadMenu16((iVar2 + 0xee), g_h_instance_1050_038c);
        (iVar2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
        local_6 = (iVar2 + 0xec);
        unaff_CS = SUB42(offset, 0);
        HVar1 = GetSubMenu16(0, local_6);
        (iVar2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    local_4 = param_2;
    local_6 = (iVar2 + 8);
    ClientToScreen16(CONCAT22(&local_6, unaff_CS), unaff_SS);
    TrackPopupMenu16(0x0, 0, (iVar2 + 8), 0, local_4, local_6, 0);
    return;
}

pub fn call_draw_fn_1020_79b4(param_1: *mut astruct_674, param_2: u16, param_3: *mut char) {
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 10)), param_3);
    if (param_2 != 0) {
        draw_1020_7cc8(*(param_1 + 0xe8));
    }
    return;
}

pub fn call_draw_fn_1020_79e4(param_1: *mut u8) {
    draw_1020_7cc8(*(param_1 + 0xe8));
    return;
}

pub fn call_load_cursor_fn_1020_7554(param_1: *mut astruct_65, param_2: u16, param_3: u32) {
    let mut local_AX_24: u16;
    let mut local_DX_71: u16;
    let mut iVar1: i32;
    let mut local_BP__1: u16;
    let local_struct_1_hi: *mut astruct_65;
    let ppVar2: *mut pass1_struct_1;

    load_cursor_1020_7f7a(param_1, CONCAT22(param_2, 5), param_3);
    local_struct_1_hi = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xee) = 0;
    (iVar1 + 0xf2) = 0;
    param_1.ptr_a_lo = 0x7780;
    (iVar1 + 2) = 0x1020;
    (iVar1 + 0xe2) = 0x781c;
    (iVar1 + 0xe4) = 0x1020;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_BP__1, 0x25));
    local_DX_71 = (ppVar2 >> 0x10);
    (iVar1 + 0xf2) = ppVar2;
    (iVar1 + 0xf4) = local_DX_71;
    (iVar1 + 0xe6) = (iVar1 + 0xf2);
    (iVar1 + 0xe8) = local_DX_71;
    return;
}

pub fn call_palette_fn_1020_679c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    call_palette_fn_1020_6466(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn load_cursor_1020_67ce(param_1: *mut astruct_65, param_2: u16, param_3: u32) {
    let mut HVar1: HGDIOBJ16;
    let mut HVar2: HCURSOR16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut in_stack_0000fffc: u16;

    call_load_cursor_1020_790e(param_1, s_TPPOPMENU_1050_43fa, param_2, param_3);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    (iVar3 + 0xf2) = 0;
    (iVar3 + 0xf6) = 0;
    param_1.ptr_a_lo = 0x70e6;
    (iVar3 + 2) = 0x1020;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (iVar3 + 0x5b)), s_VrMode2_1050_4404);
    HVar1 = GetStockObject16(5);
    (iVar3 + 0xc6) = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (iVar3 + 0xc4) = HVar2;
    (iVar3 + 0xac) = 0x44c00000;
    (iVar3 + 200) = (s_575_bmp_1050_201f + 1);
    (iVar3 + 0xbc) = (param_3 + 8);
    (iVar3 + 0xca) = param_2;
    reg_class_1008_96d2(param_1, in_stack_0000fffc);
    update_window_1020_6c3a(iVar3, uVar4);
    return;
}

pub fn cleanup_fn_1020_687c(param_1: *mut astruct_652) {
    get_sys_metrics_1020_7a50(param_1);
    destroy_icon_1020_6bd2(param_1);
    return;
}

pub fn realize_palette_1020_6896(param_1: u32, param_2: i32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let mut uVar4: u16;
    let mut local_6: u32;

    if (param_2 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar2 = (param_1 + 0xf2);
        puVar3 = (uVar2 + 0x24);
        let pu_var3_val = unsafe { *puVar3 };
        ppcVar1 = (pu_var3_val + 0x18);
        (**ppcVar1)();
        UnrealizeObject16(puVar3);
        uVar2 = (param_1 + 0xf2);
        RealizePalette16((uVar2 + 0x178));
    }
    return;
}

pub fn call_invalidate_rect_1020_68de(param_1: *mut u8) {
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    if ((param_1 + 0xf6) != 0) {
        invalidate_rect_1020_735a((param_1 + 0xf6));
    }
    return;
}

pub fn pt_in_rect_1020_68fc(param_1: *mut u32, param_2: i32, param_3: u16) {
    let ppcVar1: fn();
    let BVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = CONCAT22(param_2, param_3);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    pass1_1018_31d0(*(iVar3 + 0xf2));
    if (param_2 != 0) {
        BVar2 = PtInRect16(_local_6, ((iVar3 + 0xf2) + 0x16c));
        if (BVar2 != 0) {
            let param_1_val = unsafe { *param_1 };
            ppcVar1 = (param_1_val + 0x40);
            (**ppcVar1)(offset, iVar3, uVar4, 0xef);
        }
    }
    return;
}

pub fn win_fn_1020_694c(param_1: *mut astruct_37, uparam_2: i32) -> u32 {
    let mut u_var1: u32;
    let mut lp_help_file: string;
    let mut uVar2: i32;
    let mut HVar3: HWND16;
    let local_BX_14: *mut astruct_5;
    let mut uVar4: u16;
    let uVar5: u8;

    uVar2 = param_2;
    if (param_2 != 299) {
        local_BX_14 = param_1;
        uVar4 = (param_1 >> 0x10);
        if (param_2 < 300) {
            if (param_2 == 0x6f) {
                lp_help_file = mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1f8);
                HVar3 = WinHelp16(0x29, 1, lp_help_file, local_BX_14.field_0x8);
                return HVar3;
            }
            if (param_2 == 0xeb) {
                uVar2 = GetDlgItem16(0x1797, local_BX_14.field_0x8);
                if (uVar2 != 0) {
                    // LAB_1020_6a6f:
                    HVar3 = create_win_1020_6e98(param_1);
                    return HVar3;
                }
            } else {
                if (param_2 == 0xef) {
                    pass1_1018_2e28(local_BX_14.field_0xf2);
                    uVar2 = pass1_1008_3e0e(param_1);
                } else {
                    uVar2 = param_2 - 0x129;
                    if ((uVar2 != 0) && (uVar2 = param_2 - 0x12a, uVar2 == 0)) {
                        HVar3 = local_BX_14.field_0x8;
                        uVar5 = 0x12;
                        // LAB_1020_69c3:
                        HVar3 = PostMessage16(0, CONCAT11(0xf0, uVar5), 0x112, HVar3);
                        return HVar3;
                    }
                }
            }
        } else {
            if (param_2 == 3000) {
                HVar3 = GetDlgItem16(0x1797, local_BX_14.field_0x8);
                if (HVar3 != 0) {
                    DestroyWindow16(HVar3);
                }
                uVar2 = pass1_1018_31d0(local_BX_14.field_0xf2);
                if (uVar2 != 0) {
                    uVar1 = local_BX_14.field_0xf2;
                    pass1_1018_2d9a(uVar1, (uVar1 >> 0x10));
                    // LAB_1020_6a0b:
                    uVar1 = local_BX_14.field_0xf6;
                    HVar3 = invalidate_rect_1020_735a(uVar1, (uVar1 >> 0x10));
                    return HVar3;
                }
            } else {
                if (param_2 < 0xbb9) {
                    if (param_2 == 300) {
                        HVar3 = local_BX_14.field_0x8;
                        uVar5 = 0x20;
                        // goto LAB_1020_69c3;
                    }
                    uVar2 = param_2 - 0x12d;
                    if (param_2 != 300) {
                        uVar2 = param_2 - 0x12e;
                    }
                } else {
                    if (param_2 == 0xbb9) {
                        HVar3 = GetDlgItem16(0x1797, local_BX_14.field_0x8);
                        if (HVar3 != 0) {
                            DestroyWindow16(HVar3);
                        }
                        uVar2 = pass1_1018_31d0(local_BX_14.field_0xf2);
                        if (uVar2 != 0) {
                            uVar1 = local_BX_14.field_0xf2;
                            pass1_1018_2dde(uVar1, (uVar1 >> 0x10));
                            // goto LAB_1020_6a0b;
                        }
                    } else {
                        uVar2 = param_2 - 0xbba;
                        if (uVar2 == 0) {
                            HVar3 = GetDlgItem16(0x1797, local_BX_14.field_0x8);
                            if (HVar3 != 0) {
                                HVar3 = DestroyWindow16(HVar3);
                                return HVar3;
                            }
                            // goto LAB_1020_6a6f;
                        }
                    }
                }
            }
        }
    }
    return uVar2;
}

pub fn destroy_win_1020_6ae6(param_1: *mut u32, param_2: u16_00, param_4: i32, param_3: i32) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3 == 0x1797) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        local_4 = GetDlgItem16(0x1797, (iVar3 + 8));
        if (local_4 != 0) {
            if (param_2 == 2) {
                _local_8 = SendMessage16(0, 0, 0x409, local_4);
                if (_local_8 != -1) {
                    SendMessage16(
                        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &stack0xffa8)),
                        _local_8,
                        0x40a,
                        local_4,
                    );
                    pass1_1018_30ca((iVar3 + 0xf2), CONCAT22(unaff_SS, &stack0xffa8));
                    iVar2 = pass1_1018_2fe8();
                    if (iVar2 != 0) {
                        invalidate_rect_1020_735a();
                        let param_1_val = unsafe { *param_1 };
                        ppcVar1 = (param_1_val + 0x40);
                        (**ppcVar1)(0x18, iVar3);
                    }
                }
            } else {
                if (param_2 != 3) {
                    return;
                }
            }
            DestroyWindow16(local_4);
        }
    }
    return;
}

pub fn enable_menu_item_1020_6b9a() {
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;

    if (in_stack_0000000a != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0, in_stack_0000000c);
    return;
}

pub fn call_win_fn_1020_6bbc(in_struct_1: *mut u8) {
    win_fn_1020_737a((in_struct_1 + 0xf6));
    return;
}

pub fn destroy_icon_1020_6bd2(param_1: u32) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    DestroyIcon16((iVar4 + 0xc2));
    (iVar4 + 0xc2) = 0;
    (iVar4 + 8) = 0;
    puVar1 = (iVar4 + 0xf6);
    uVar2 = (iVar4 + 0xf8);
    if ((uVar2 | puVar1) != 0) {
        let pu_var1_val = unsafe { *puVar1 };
        ppcVar3 = pu_var1_val;
        (**ppcVar3)(offset, puVar1, uVar2, 1);
    }
    (iVar4 + 0xf6) = 0;
    pass1_1010_1dda(*(iVar4 + 0xf2));
    (iVar4 + 0xf2) = 0;
    return;
}

pub fn update_window_1020_6c3a(param_1: *mut win_struct_42) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut uVar4: i32;
    let piVar5: *mut u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let struct_a: *mut astruct_199;
    let paVar8: *mut astruct_199;
    let struct_a_00: *mut astruct_199;
    let struct_a_01: *mut astruct_199;
    let struct_a_02: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut iVar9: i32;
    let mut unaff_SI: u16;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let ppVar11: *mut pass1_struct_1;
    let uVar12: u8;
    let uVar13: u8;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    create_win_1008_9760(param_1);
    ppVar11 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 4));
    uVar7 = (ppVar11 >> 0x10);
    uVar10 = (param_1 >> 0x10);
    iVar9 = param_1;
    (iVar9 + 0xf2) = ppVar11;
    (iVar9 + 0xf4) = uVar7;
    uVar3 = (iVar9 + 0xf2);
    (iVar9 + 0xe0) = uVar3;
    (iVar9 + 0xe2) = uVar7;
    LoadIcon16(
        0x1010,
        s_TILEICON_1050_440c,
        &g_alloc_addr_1050_1050,
        g_h_instance_1050_038c,
    );
    (iVar9 + 0xc2) = uVar3;
    uVar6 = (iVar9 + 0xf2);
    uVar7 = uVar6;
    ppcVar2 = ((iVar9 + 0xf2) + 0x30);
    ppcVar2(offset, uVar7, (uVar6 >> 0x10), uVar3);
    uVar4 = &local_6 + 2;
    uVar12 = unaff_SS;
    uVar13 = (unaff_SS >> 8);
    pass1_1018_2d22(
        (iVar9 + 0xf2),
        CONCAT22(unaff_SS, &local_6),
        CONCAT13(uVar13, CONCAT12(uVar12, uVar4)),
        3000,
    );
    paVar8 = struct_a;
    process_struct_1000_179c(0x42, struct_a);
    if ((paVar8 | uVar4) != 0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar8,
            0,
            local_6,
            0x7d0000,
            0xbb8007c,
            CONCAT22(uVar7, (iVar9 + 8)),
        );
    }
    uVar4 = &local_6 + 2;
    pass1_1018_2d22(
        (iVar9 + 0xf2),
        CONCAT22(unaff_SS, &local_6),
        CONCAT13(uVar13, CONCAT12(uVar12, uVar4)),
        0xbb9,
    );
    paVar8 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    if ((paVar8 | uVar4) != 0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar8,
            0,
            local_6,
            0x7f0000,
            0xbb9007e,
            CONCAT22(uVar7, (iVar9 + 8)),
        );
    }
    uVar4 = &local_6 + 2;
    pass1_1018_2d22(
        (iVar9 + 0xf2),
        CONCAT22(unaff_SS, &local_6),
        CONCAT13(uVar13, CONCAT12(uVar12, uVar4)),
        0xbba,
    );
    paVar8 = struct_a_01;
    process_struct_1000_179c(0x42, struct_a_01);
    struct_a_02 = (paVar8 | uVar4);
    if (struct_a_02 != 0x0) {
        win_fn_1008_3bd6(
            uVar4,
            paVar8,
            0,
            local_6,
            0x1b101b2,
            0xbba01b0,
            CONCAT22(uVar7, (iVar9 + 8)),
        );
        struct_a_02 = extraout_DX;
    }
    process_struct_1000_179c(0x22, struct_a_02);
    if ((struct_a_02 | uVar4) == 0) {
        (iVar9 + 0xf6) = 0;
    } else {
        get_dc_1020_717e();
        (iVar9 + 0xf6) = uVar4;
        (iVar9 + 0xf8) = extraout_DX_00;
    }
    uVar6 = (iVar9 + 0xf6);
    (iVar9 + 0xe8) = uVar6;
    uVar1 = (iVar9 + 0xf2);
    ppcVar2 = ((iVar9 + 0xf2) + 0x10);
    ppcVar2(0x1000, uVar1, (uVar1 >> 0x10));
    piVar5 = uVar6;
    let pi_var5_val = unsafe { *piVar5 };
    MoveWindow16(1, piVar5[3], piVar5[2], piVar5[1], pi_var5_val, (iVar9 + 8));
    uVar6 = param_1;
    ppcVar2 = (uVar6 + 0x94);
    ppcVar2(0x38, iVar9, uVar10, 0);
    ppcVar2 = (uVar6 + 0x10);
    ppcVar2(offset, iVar9, (param_1 >> 0x10), 1);
    UpdateWindow16((iVar9 + 8));
    return;
}

pub fn pass1_1020_6e52(param_1: *mut astruct_674, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let lVar2: u32;
    let pcVar3: *mut char;

    uVar1 = param_1.field_0xf2;
    lVar2 = pass1_1018_2e5e(uVar1, (uVar1 >> 0x10));
    if (lVar2 == 0) {
        pcVar3 = load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x5a1,
        );
    } else {
        pcVar3 = pass1_1018_2d84(param_1.field_0xf2);
    }
    call_draw_fn_1020_79b4(
        CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
        param_3,
        pcVar3,
    );
    return;
}

pub fn create_win_1020_6e98(param_1: *mut astruct_37) {
    let puVar1: *mut u16;
    let w_param: WPARAM16;
    let mut in_struct_1: u32;
    let mut h_var2: HWND16;
    let mut uvar3: u16;
    let mut i_var4: i32;
    let mut extraout_d_x: u16;
    let mut extraout_DX_00: u16;
    let local_BX_5: *mut astruct_37;
    let mut u_var5: u16;
    let mut unaff_SS: u16;
    let mut send_msg_result: u32;
    let mut l_var6: LRESULT;
    let mut local_24: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 4];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    GetClientRect16(CONCAT22(unaff_SS, local_a), local_BX_5.hwnd_dlg);
    local_e = 0;
    h_var2 = GetDlgItem16(0x1797, local_BX_5.hwnd_dlg);
    if (h_var2 != 0) {
        DestroyWindow16(h_var2);
    }
    pass1_1018_30fc(local_BX_5.field_0xf2, CONCAT22(unaff_SS, &local_e));
    if ((local_e._2_2_ | local_e) != 0) {
        h_var2 = CreateWindow16(
            local_e,
            g_h_instance_1050_038c,
            0x1797,
            local_BX_5.hwnd_dlg,
            local_4 - 0x19,
            local_6,
            0,
            0,
            0x40a00103,
            0x10504415,
            s_listbox_1050_4416,
        );
        in_struct_1 = local_e;
        if (h_var2 == 0) {
            if ((local_e._2_2_ | local_e) != 0) {
                pass1_1018_2afa(local_e);
                error_check_1000_17ce(in_struct_1);
                return;
            }
        } else {
            send_msg_result = SendMessage16(0, 0, 0xb, h_var2);
            send_msg_result._0_2_ = (send_msg_result & 0xffff);
            if ((local_e + 4) == 0) {
                load_string_1010_847e(
                    _g_struct_73_1050_14cc,
                    (_g_struct_73_1050_14cc >> 0x10),
                    0x531,
                );
                SendMessage16(CONCAT22(extraout_d_x, send_msg_result), 0, 0x401, h_var2);
            } else {
                local_24 = 0;
                i_var4 = (send_msg_result >> 0x10);
                while (true) {
                    puVar1 = (local_e + 4);
                    let pu_var1_val = unsafe { *puVar1 };
                    if (pu_var1_val == local_24 || pu_var1_val < local_24) {
                        break;
                    }
                    big_switch_statement_1020_bd80((local_e + local_24 * 2));
                    l_var6 = SendMessage16(CONCAT22(i_var4, local_24), 0, 0x401, h_var2);
                    local_24 = i_var4 + 1;
                    i_var4 = (l_var6 >> 0x10);
                }
            }
            l_var6 = SendMessage16(0, 1, 0xb, h_var2);
            uVar3 = l_var6;
            pass1_1018_2d84(local_BX_5.field_0xf2);
            l_var6 = SendMessage16(CONCAT22(extraout_DX_00, uVar3), 0xffff, 0x40d, h_var2);
            w_param = l_var6;
            if ((w_param != 0xffff) || ((l_var6 >> 0x10) != -1)) {
                SendMessage16(0, w_param, 0x407, h_var2);
                SendMessage16(0, w_param, 0x418, h_var2);
            }
            if (local_e != 0) {
                pass1_1018_2afa(local_e);
                error_check_1000_17ce(0x1538704d);
            }
            ShowWindow16(1, h_var2);
            SetFocus16(h_var2);
        }
    }
    return;
}

pub fn set_colors_1020_7070() {
    let mut in_DX: i32;
    let mut CVar1: COLORREF;
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: HDC16;
    let mut local_4: u16;

    GetStockObject16(4);
    if (_PTR_LOOP_1050_441e == 0) {
        _PTR_LOOP_1050_441e = 0x1000002;
    }
    if (6 < in_stack_0000000a) {
        return in_DX << 0x10;
    }
    SetTextColor16(_PTR_LOOP_1050_441e, in_stack_0000000c);
    CVar1 = SetBkColor16(0x1000000, in_stack_0000000c);
    return CVar1 & 0xffff0000 | 0x100;
}

pub fn cleanup_fn_1020_70c0(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn get_dc_1020_717e(param_1: *mut u16, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let puVar4: *mut u16;
    let puVar5: *mut u32;
    let mut extraout_DX: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let unaff_SS: u8;
    let ppVar8: *mut pass1_struct_1;
    let uVar9: u8;
    let uVar10: u8;
    let mut iVar11: i32;
    let mut uVar12: u16;
    let mut in_stack_0000ffdc: u16;
    let mut local_1c: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    iVar11 = param_1;
    uVar12 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(iVar11, uVar12, param_2, (param_2 >> 0x10));
    (iVar11 + 0x14) = 0;
    (iVar11 + 0x18) = param_2;
    (iVar11 + 0x1c) = 0;
    (iVar11 + 0x20) = 0;
    unsafe {
        *param_1 = 0x754c;
    }
    (iVar11 + 2) = 0x1020;
    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffdc, 4));
    uVar7 = (ppVar8 >> 0x10);
    (iVar11 + 0x1c) = ppVar8;
    (iVar11 + 0x1e) = uVar7;
    ppcVar1 = ((iVar11 + 0x1c) + 4);
    (**ppcVar1)(0x10, (iVar11 + 0x1c), uVar7, 0, iVar11, uVar12);
    local_4 = GetDC16((iVar11 + 4));
    uVar2 = (iVar11 + 0x1c);
    (uVar2 + 0x178) = local_4;
    uVar2 = (iVar11 + 0x1c);
    uVar7 = (uVar2 >> 0x10);
    iVar6 = uVar2;
    puVar5 = (iVar6 + 0x24);
    uVar9 = SUB41(puVar5, 0);
    uVar10 = (puVar5 >> 8);
    unsafe { ppcVar1 = (*puVar5 + 0x14) };
    (**ppcVar1)(0x38, uVar9, (iVar6 + 0x26));
    ppVar8 = process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT13(uVar10, CONCAT12(uVar9, 0x29)),
    );
    uVar2 = (ppVar8 + 0xe);
    puVar3 = uVar2;
    pass1_1008_4d84((puVar5 & 0xffff | extraout_DX << 0x10), puVar3);
    puVar4 = &local_4;
    realize_palette_1008_4e08(puVar3, (uVar2 >> 0x10), puVar4, unaff_SS);
    (iVar11 + 0x20) = puVar4;
    return;
}

pub fn win_fn_1020_7270(param_1: *mut astruct_44) {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1: *mut astruct_44;
    let local_struct_1_hi: *mut astruct_44;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut temp_5f4bfe04bb: u32;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x754c;
    local_struct_1.ptr_a_hi = 0x1020;
    if (&local_struct_1.field_0x1c != 0) {
        pass1_1010_1ea6(
            *&local_struct_1.field_0x1c,
            (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        );
    }
    uVar1 = &local_struct_1.field_0x14;
    uVar2 = (&local_struct_1.field_0x14 + 2);
    _local_8 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1008_5118(CONCAT22(uVar2, uVar1));
        error_check_1000_17ce(_local_8);
    }
    temp_5f4bfe04bb = &local_struct_1.field_0x1c;
    h_gdi_obj = SelectPalette16(0, local_struct_1.field_0x20, (temp_5f4bfe04bb + 0x178));
    local_struct_1.field_0x20 = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    param_1.ptr_a_lo = s_0_020_1050_3ab0;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn post_win_msg_1020_7308(param_1: u32, uparam_2: i32) {
    let mut cVar1: u8;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (param_2 != 0x12) {
        if (param_2 < 0x13) {
            cVar1 = param_2;
            if (cVar1 == 0x1) {
                (param_1 + 0x1c) = 0;
                return;
            }
            if (('\x03' < (cVar1 + -1)) && ((cVar1 + -5) < 0x2)) {}
            // goto LAB_1020_7310;
        }
        return;
    }
    // LAB_1020_7310:
    PostMessage16(0, 0xeb, 0x111, (param_1 + 4));
    invalidate_rect_1020_735a(param_1);
    return;
}

pub fn invalidate_rect_1020_735a(param_1: u32) {
    let mut u_var1: u32;

    uVar1 = (param_1 + 0x1c);
    InvalidateRect16(0, (uVar1 + 0x16c), (uVar1 >> 0x10));
    return;
}

pub fn win_fn_1020_737a(in_struct_1: *mut astruct_15) {
    let mut uVar1: i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut b_result: u16;
    let puVar4: *mut u8;
    let mut uVar5: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: u16;
    let local_BX_4: *mut astruct_15;
    let mut uVar6: u16;
    let mut unaff_SS: HWND16;
    let uVar7: u8;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut uStack74;
    let mut uStack68;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut h_dialog: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut b_result_2: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar6 = (in_struct_1 >> 0x10);
    local_BX_4 = in_struct_1;
    uVar7 = (unaff_SS >> 8);
    b_result_2 = BeginPaint16(
        CONCAT13(uVar7, CONCAT12(unaff_SS, &local_22)),
        local_BX_4.h_window,
    );
    b_result = IsIconic16(local_BX_4.h_window);
    if (b_result == 0) {
        uVar5 = local_BX_4.field_0x1c;
        uVar5 = (uVar5 + 0x24);
        local_3c = uVar5;
        local_3a = (uVar5 >> 0x10);
        uVar2 = local_BX_4.field_0x1c;
        pass1_1018_2e5e(uVar2, (uVar2 >> 0x10));
        _local_30 = (uVar5 & 0xffff | extraout_DX << 0x10);
        pass1_1008_3e54(
            CONCAT13(uVar7, CONCAT12(unaff_SS, &uStack68 + 2)),
            0,
            0x35,
            0xc,
        );
        if (&local_BX_4.field_0x14 != 0) {
            pass1_1008_5236(&local_BX_4.field_0x14);
        }
        if (_local_30 != 0x0) {
            puVar4 = local_BX_4.field_0x14;
            uVar1 = local_BX_4.field_0x16;
            if ((uVar1 | puVar4) != 0) {
                pass1_1008_5118(CONCAT22(uVar1, puVar4));
                error_check_1000_17ce(CONCAT22(uVar1, puVar4));
            }
            puVar4 = (&uStack68 + 2);
            pass1_1008_8ce4(
                _local_30,
                CONCAT22(unaff_SS, puVar4),
                CONCAT22(local_3a, local_3c),
            );
            local_BX_4.field_0x14 = puVar4;
            local_BX_4.field_0x16 = extraout_DX_00;
        }
        ppcVar3 = (CONCAT22(local_3a, local_3c) + 4);
        (**ppcVar3)(&PTR_LOOP_1050_1008, local_3c, local_3a, 0, 0, &b_result_2);
        ppcVar3 = (local_BX_4.field_0x18 + 0x94);
        (**ppcVar3)(&PTR_LOOP_1050_1008, local_BX_4.field_0x18, 1);
        h_dialog = GetDlgItem16(0x1797, local_BX_4.h_window);
        if (h_dialog != 0) {
            ShowWindow16(1, h_dialog);
        }
    } else {
        if (PTR_LOOP_1050_0010 == 0x0) {
            uVar5 = local_BX_4.field_0x1c;
            ppcVar3 = (local_BX_4.field_0x1c + 0x2c);
            (**ppcVar3)(offset, uVar5, (uVar5 >> 0x10));
            local_26 = b_result;
            if (b_result != 0) {
                local_28 = GetStockObject16(4);
                GetClientRect16(CONCAT22(unaff_SS, &local_30), local_BX_4.h_window);
                local_3c = 0;
                local_3a = 0;
                local_38 = (local_2c - local_30) - 1;
                local_36 = (local_2a - local_2e) - 1;
                h_dialog = local_36;
                local_32 = local_38;
                FillRect16(local_28, &local_3c, unaff_SS);
                DrawIcon16(
                    local_26,
                    CONCAT610(
                        uStack74,
                        CONCAT28(local_4c, CONCAT26(local_4e, CONCAT24(b_result_2, 0x20002))),
                    ),
                    CONCAT214(
                        local_36,
                        CONCAT212(local_38, CONCAT210(local_3a, CONCAT28(local_3c, uStack68))),
                    ),
                    h_dialog,
                );
            }
        }
    }
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn draw_1020_650c(param_1: *mut astruct_622) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: HWND16;
    let mut local_28: PAINTSTRUCT16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar2 = (iVar3 + 0x14);
    local_6 = (uVar2 + 10);
    if (((iVar3 + 0x10) != 0) || (uVar2 = (iVar3 + 0x14), (uVar2 + 0x24) != 0)) {
        draw_1020_9364(param_1);
        if ((iVar3 + 0x24) != 0) {
            ppcVar1 = ((iVar3 + 0x24) + 0x14);
            (**ppcVar1)();
        }
    }
    local_8 = 0;
    while {
        if ((iVar3 + 0x18 + local_8 * 4) != 0) {
            ppcVar1 = ((iVar3 + 0x18 + local_8 * 4) + 8);
            (**ppcVar1)();
        }
        local_8 = local_8 + 1;
        local_8 < 5
    } {}
    BeginPaint16(CONCAT22(unaff_SS, &local_28), (iVar3 + 4));
    ppcVar1 = (local_6 + 4);
    (**ppcVar1)(offset, local_6, (local_6 >> 0x10), 0, 0, iVar3 + 10, uVar4);
    EndPaint16(&local_28, unaff_SS);
    return;
}

pub fn send_win_msg_1020_65cc(param_1: *mut astruct_672, param_2: i32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let b_var3: bool;
    let mut uVar4: u16;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut local_4: u16;

    iVar6 = param_1;
    uVar8 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar6 + 0x14) = 0;
        return;
    }
    if (param_2 == 2) {
        local_4 = 0;
        while (local_4 < 5) {
            iVar5 = iVar6 + 0x18;
            iVar7 = local_4 * 4;
            if (((iVar5 + iVar7 + 2) | (iVar5 + iVar7)) != 0) {
                ppcVar1 = ((iVar5 + iVar7) + 4);
                (**ppcVar1)();
            }
            local_4 = local_4 + 1;
        }
    } else {
        if (((0 < param_2 + -3) && (!SBORROW2(param_2 + -3, 1))) && (param_2 + -4 < 4)) {
            b_var3 = IsIconic16(g_h_window);
            if (b_var3 == 0) {
                b_var3 = IsIconic16((iVar6 + 4));
                if ((b_var3 == 0) && (uVar2 = (iVar6 + 0x14), (uVar2 + 0x24) != 0)) {
                    InvalidateRect16(0, 0x0, 0);
                    pass1_1020_64d4(param_1, 2);
                    if (b_var3 == 0) {
                        call_fn_ptr_1_1020_6746(param_1, 1, 2);
                    }
                    pass1_1020_64d4(param_1, 3);
                    if (b_var3 == 0) {
                        call_fn_ptr_1_1020_6746(param_1, 1, 3);
                    }
                    uVar4 = pass1_1018_255e((iVar6 + 0x14));
                    if (uVar4 == 0) {
                        SendMessage16(0, 0x69, 0x111, (iVar6 + 4));
                    } else {
                        pass1_1020_64d4(param_1, 1);
                        if (uVar4 == 0) {
                            call_fn_ptr_1_1020_6746(param_1, 1, 1);
                        }
                    }
                    SendMessage16(0, 0xf0, 0x111, (iVar6 + 4));
                    uVar2 = (iVar6 + 0x2c);
                    if ((uVar2 + 0x7a) != 0) {
                        uVar2 = (iVar6 + 0x2c);
                        (uVar2 + 0x7a) = 0;
                        SendMessage16(0, 0x131, 0x111, (iVar6 + 4));
                        return;
                    }
                }
            }
        }
    }
    return;
}

pub fn call_palette_fn_1020_6466(in_struct_1: *mut astruct_44) {
    let local_struct_1: *mut astruct_44;
    let local_struct_1_hi: *mut astruct_44;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x67c2;
    local_struct_1.ptr_a_hi = 0x1020;
    if (local_struct_1.field_0x14 != 0) {
        pass1_1010_1ea6(
            local_struct_1.field_0x14,
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        );
    }
    select_and_delete_palette_1020_92c4(in_struct_1);
    return;
}

pub fn cleanup_fn_1020_6216(in_struct_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    destroy_win_1020_42f4(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn process_struct_1020_62e0(in_struct_1: *mut astruct_668, param_2: u32) {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let uVar3: u8;
    let mut uVar4: i32;
    let mut extraout_var;
    let mut uVar5: u32;
    let extraout_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let extraout_DX_01: *mut astruct_199;
    let extraout_DX_02: *mut astruct_199;
    let paVar6: *mut astruct_199;
    let mut extraout_DX_03: u16;
    let mut uVar7: u16;
    let ppVar8: *mut pass1_struct_1;
    // ppuVar9: *mut *mut u8;
    let uVar10: u8;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let paVar13: *mut astruct_668;
    let mut uVar14: u16;
    let mut in_stack_0000ffee: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    // fn_ptr_3: *mut *mut u8;
    // fn_ptr_1: *mut *mut u8;

    uVar12 = param_2;
    get_dc_1020_921c(CONCAT22(uVar12, in_struct_1), (param_2 >> 0x10));
    &in_struct_1.fn_ptr_1_0x14 = 0;
    &in_struct_1.field_0x2c = 0;
    CONCAT22(uVar12, in_struct_1) = 0x67c2;
    in_struct_1.field_0x2 = 0x1020;
    paVar6 = extraout_DX;
    uVar3 = pass1_1000_4906(CONCAT22(uVar12, &in_struct_1.field_0x18), 0, 0x14);
    uVar4 = CONCAT31(extraout_var, uVar3);
    process_struct_1000_179c(0x3c, paVar6);
    struct_a = (paVar6 | uVar4);
    if (struct_a == 0x0) {
        &in_struct_1.field_0x1c = 0;
    } else {
        pass1_1020_87c2(uVar4, paVar6);
        in_struct_1.field_0x1c = uVar4;
        in_struct_1.field_0x1e = extraout_DX_00;
        struct_a = extraout_DX_00;
    }
    process_struct_1000_179c(0x26, struct_a);
    if ((struct_a | uVar4) == 0) {
        uVar4 = 0;
        paVar6 = 0x0;
    } else {
        pass1_1020_8a9c(uVar4, struct_a);
        paVar6 = extraout_DX_01;
    }
    in_struct_1.field_0x20 = uVar4;
    in_struct_1.field_0x22 = paVar6;
    process_struct_1000_179c(0xbe, paVar6);
    if ((paVar6 | uVar4) == 0) {
        uVar4 = 0;
        paVar6 = 0x0;
    } else {
        pass1_1020_8eaa(uVar4, paVar6);
        paVar6 = extraout_DX_02;
    }
    in_struct_1.field_0x24 = uVar4;
    in_struct_1.field_0x26 = paVar6;
    process_struct_1000_179c(0x20, paVar6);
    if ((paVar6 | uVar4) == 0) {
        uVar4 = 0;
        uVar7 = 0;
    } else {
        process_struct_1020_8360(CONCAT22(paVar6, uVar4));
        uVar7 = extraout_DX_03;
    }
    in_struct_1.field_0x28 = uVar4;
    in_struct_1.field_0x2a = uVar7;
    call_fn_ptr_1_1020_6746(CONCAT22(uVar12, in_struct_1), 1, 4);
    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffee, 0x29));
    in_struct_1.fn_ptr_1_0x14 = ppVar8;
    &in_struct_1.field_0x16 = (ppVar8 >> 0x10);
    uVar11 = 0;
    uVar10 = (ppVar8 >> 0x10);
    ppuVar9 = in_struct_1.fn_ptr_1_0x14;
    fn_ptr_1 = (&in_struct_1.fn_ptr_1_0x14 + 4);
    paVar13 = in_struct_1;
    uVar14 = uVar12;
    (**fn_ptr_1)();
    in_struct_1.field_0x6 = &in_struct_1.fn_ptr_1_0x14;
    uVar2 = &in_struct_1.fn_ptr_1_0x14;
    puVar1 = (uVar2 + 10);
    uVar5 = param_2 << 0x10 | &in_struct_1.field_0xa;
    uVar7 = SUB42(puVar1, 0);
    let pu_var1_val = unsafe { *puVar1 };
    fn_ptr_3 = (pu_var1_val + 8);
    (**fn_ptr_3)(
        0x1010,
        uVar7,
        (puVar1 >> 0x10),
        uVar5,
        ppuVar9,
        uVar10,
        uVar11,
        paVar13,
        uVar14,
    );
    in_struct_1.field_0x12 = uVar5;
    in_struct_1.field_0x10 = 1;
    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar7, 2));
    in_struct_1.field_0x2c = ppVar8;
    in_struct_1.field_0x2e = (ppVar8 >> 0x10);
    return;
}

pub fn set_cursor_1020_5de8(
    in_struct_1: *mut astruct_664,
    param_2: u16,
    in_struct_2: *mut astruct_665,
) {
    let mut u_var1: u32;
    let local_struct_2_1: *mut astruct_665;
    let mut extraout_DX: i32;
    let local_struct_1: *mut astruct_664;
    let mut local_struct_1_hi: u16;
    let mut unaff_SS: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffe2: HWND16;
    let mut in_stack_0000ffe8: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_struct_2: *mut astruct_665;
    let mut local_4: u16;

    ReleaseCapture16(in_stack_0000ffe2);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    SetCursor16(local_struct_1.cursor_handle_0xee);
    local_struct_1.cursor_handle_0xee = 0;
    local_struct_1.field_0xf4 = 1;
    local_struct_2 = in_struct_2;
    local_4 = param_2;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe8, 0x47));
    local_struct_2_1 = &local_struct_2;
    pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_SS, local_struct_2_1));
    if ((extraout_DX | local_struct_2_1) != 0) {
        uVar1 = &local_struct_2_1.field_0x42;
        local_struct_1_hi = &local_struct_2_1.field_0x44;
        local_12._3_1_ = (uVar1 >> 0x18);
        if (local_12._3_1_ == '\x05') {
            local_12._0_2_ = uVar1;
            // goto LAB_1020_5e62;
        }
    }
    local_12._0_2_ = 0;
    local_struct_1_hi = 0;
    // LAB_1020_5e62:
    pass1_1018_57e6(ppVar2, CONCAT22(local_struct_1_hi, local_12));
    return;
}

pub fn win_fn_1020_5e76(in_struct_1: *mut astruct_663, param_2: u16, param_3: u16) {
    let puVar1: *mut u8;
    let ppcVar2: fn();
    let uVar3: u8;
    let puVar4: *mut u32;
    let extraout_var;
    let puVar5: *mut u16;
    let mut extraout_DX: i32;
    let struct_a: *mut astruct_199;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let puVar10: *mut u8;
    let mut uVar11: u16;
    let unaff_SS: *mut char;
    let ppVar12: *mut pass1_struct_1;
    let uVar13: u8;
    let mut local_2aa: u16;
    let mut local_2a8: u16;
    let mut uStack676: u16;
    let mut local_1aa: u16;
    let mut local_aa: [u8; 128];
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8; 16];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ReleaseCapture16(local_2aa);
    uVar9 = (in_struct_1 >> 0x10);
    iVar8 = in_struct_1;
    local_2a8 = (iVar8 + 0xee);
    local_2aa = offset;
    SetCursor16(local_2a8);
    (iVar8 + 0xee) = 0;
    (iVar8 + 0xf4) = 1;
    local_6 = param_3;
    local_4 = param_2;
    local_2a8 = &local_6;
    local_2aa = uVar9;
    pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_SS, local_2a8));
    _local_a = CONCAT22(extraout_DX, local_2a8);
    struct_a = (extraout_DX | local_2a8);
    if (struct_a == 0x0) {}
    // goto LAB_1020_6176;
    local_c = (local_2a8 + 0xc);
    local_2aa = (local_2a8 + 0xe);
    local_10 = 0;
    uVar11 = 0x1018;
    local_2a8 = local_c;
    local_e = local_2aa;
    puVar4 = pass1_1018_2580(
        (iVar8 + 0xfa),
        0,
        CONCAT22(local_c, local_2aa),
        (iVar8 + 0x10c),
    );
    if (puVar4 == 0x6b2) {}
    // goto LAB_1020_6176;
    local_12 = puVar4;
    if (puVar4 == 0x6b8) {
        local_2a8 = 0x1018;
        local_2aa = &PTR_LOOP_1050_5f06;
        process_struct_1000_179c(0xb4, struct_a);
        _local_2a = CONCAT22(struct_a, puVar4);
        uVar6 = struct_a | puVar4;
        if (uVar6 == 0) {
            puVar4 = 0x0;
            uVar6 = 0;
        } else {
            local_2a8 = 0x6b8;
            local_2aa = 2;
            mixed_1040_8520(
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, puVar4)),
                g_h_window,
                0x40,
                2,
                0x6b8,
                0x6ad,
            );
        }
        _local_26 = CONCAT22(uVar6, puVar4);
        uVar11 = 0xa5;
        // LAB_1020_5f84:
        local_2a8 = 1;
        pass1_1008_941a(CONCAT22(unaff_SS, local_22), 1, uVar11);
        puVar10 = (_local_26 >> 0x10);
        local_2aa = _local_26;
        local_2a8 = local_22;
    } else {
        if (puVar4 == 0x6b4) {
            local_2a8 = 0x1018;
            local_2aa = 0x5f4d;
            process_struct_1000_179c(0xb4, struct_a);
            _local_2a = CONCAT22(struct_a, puVar4);
            uVar6 = struct_a | puVar4;
            if (uVar6 == 0) {
                puVar4 = 0x0;
                uVar6 = 0;
            } else {
                local_2aa = 2;
                local_2a8 = 0x57b;
                mixed_1040_8520(
                    CONCAT13((struct_a >> 8), CONCAT12(struct_a, puVar4)),
                    g_h_window,
                    0x40,
                    2,
                    0x57b,
                    local_12,
                );
            }
            _local_26 = CONCAT22(uVar6, puVar4);
            uVar11 = 0xab;
            // goto LAB_1020_5f84;
        }
        if (puVar4 == 0x6b6) {
            local_2aa = local_aa;
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_SS, local_2aa),
                0x57b,
            );
            local_2aa = &local_1aa;
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_SS, local_2aa),
                0x6b6,
            );
            local_2aa = &local_1aa;
            uVar3 = string_fn_1000_3f9c(
                &local_2aa,
                unaff_SS,
                local_2aa,
                unaff_SS,
                PTR_LOOP_1050_50cc,
            );
            uVar6 = CONCAT31(extraout_var, uVar3);
            local_2a8 = 0x1000;
            uVar11 = 0x1000;
            local_2aa = 0x5fef;
            process_struct_1000_179c(0xb4, struct_a);
            _local_2a = CONCAT22(struct_a, uVar6);
            uVar7 = struct_a | uVar6;
            if (uVar7 == 0) {
                puVar4 = 0x0;
                uVar7 = 0;
            } else {
                local_2aa = &local_2aa;
                uVar11 = &PTR_LOOP_1050_1040;
                puVar5 = process_struct_1040_8478(
                    CONCAT22(struct_a, uVar6),
                    0x40,
                    CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, local_aa)),
                    CONCAT22(unaff_SS, local_2aa),
                    g_h_window,
                );
                puVar4 = puVar5;
            }
            _local_26 = CONCAT22(uVar7, puVar4);
            // LAB_1020_6027:
            let pu_var4_val = unsafe { *puVar4 };
            ppcVar2 = (pu_var4_val + 0x74);
            local_2aa = uVar11;
            local_2a8 = puVar4;
            ppcVar2();
            // goto LAB_1020_6176;
        }
        if (puVar4 < 0x6a7) {
            if (((iVar8 + 0x10c) == 0x78) || ((iVar8 + 0x10c) == 0x74)) {
                local_2aa = _g_astruct_372_1050_0ed0;
                local_2a8 = (_g_astruct_372_1050_0ed0 >> 0x10);
                uVar11 = 0x1010;
                ppVar12 =
                    process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uStack676, 5));
                if ((ppVar12 + 10) == 0) {
                    return;
                }
            }
            if (((((iVar8 + 0x10c) == 0x6c) || ((iVar8 + 0x10c) == 0x6d))
                || ((iVar8 + 0x10c) == 0x31))
                || ((iVar8 + 0x10c) == 0x32))
            {
                local_2aa = _g_astruct_372_1050_0ed0;
                local_2a8 = (_g_astruct_372_1050_0ed0 >> 0x10);
                uVar11 = 0x1010;
                ppVar12 =
                    process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uStack676, 0x3a));
                if ((ppVar12 + 10) == 0) {
                    return;
                }
            }
            puVar1 = *(iVar8 + 0xfe);
            local_2a8 = puVar1;
            local_2aa = uVar11;
            call_invalidate_rect_1020_68de(puVar1);
            // goto LAB_1020_6176;
        }
        if (0x6b1 < puVar4) {
            local_2a8 = 0x1018;
            uVar11 = 0x1000;
            local_2aa = 0x60cf;
            process_struct_1000_179c(0xb4, struct_a);
            _local_2a = CONCAT22(struct_a, puVar4);
            uVar7 = struct_a | puVar4;
            if (uVar7 == 0) {
                puVar4 = 0x0;
                uVar7 = 0;
            } else {
                local_2aa = 2;
                local_2a8 = 0x57b;
                uVar11 = &PTR_LOOP_1050_1040;
                mixed_1040_8520(
                    CONCAT13((struct_a >> 8), CONCAT12(struct_a, puVar4)),
                    g_h_window,
                    0x40,
                    2,
                    0x57b,
                    local_12,
                );
            }
            // goto LAB_1020_6027;
        }
        local_2a8 = 0x1018;
        local_2aa = 0x6116;
        process_struct_1000_179c(0xb4, struct_a);
        _local_2a = CONCAT22(struct_a, puVar4);
        if ((struct_a | puVar4) != 0) {
            local_2aa = 2;
            local_2a8 = 0x57b;
            mixed_1040_8520(
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, puVar4)),
                g_h_window,
                0x40,
                2,
                0x57b,
                local_12,
            );
        }
        local_1aa = local_12 - 0x608;
        local_2a8 = 1;
        local_2aa = unaff_SS;
        pass1_1008_941a(CONCAT22(unaff_SS, local_aa), 1, local_1aa);
        puVar10 = local_aa;
        local_2a8 = puVar10;
    }
    uVar13 = local_2aa;
    ppcVar2 = (local_2aa + 0x6c);
    local_2aa = puVar10;
    ppcVar2(&PTR_LOOP_1050_1008, uVar13);
    // LAB_1020_6176:
    (iVar8 + 0x10c) = 0;
    return;
}

pub fn set_capture_1020_6184(in_struct_1: *mut astruct_661, param_2: u16) {
    let mut cursor: HCURSOR16;
    let local_struct_1: *mut astruct_661;
    let local_struct_1_hi: *mut astruct_661;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0xf4 == 1) {
        cursor = SetCursor16(local_struct_1.cursor_handle_0xf0);
        local_struct_1.cursor_handle_0xee = cursor;
        local_struct_1.field_0x10c = param_2;
        SetCapture16(local_struct_1.window_handle_0x8);
        local_struct_1.field_0xf4 = 2;
    }
    return;
}

pub fn post_msg_1020_55b0(param_1: u32) {
    let ppcVar1: fn();
    let p_uvar2: *mut u16;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let puVar5: *mut u16;
    let mut extraout_DX: i32;
    let struct_a: *mut astruct_199;
    let mut uVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let ppVar9: *mut pass1_struct_1;
    let mut in_stack_0000fed8: u16;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u16;
    let mut local_114: u16;
    let mut local_112: [u8; 2];
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: [u8; 256];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fed8, 2));
    local_8 = (_local_6 + 0x20);
    _local_c =
        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fed8, 0x37));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, local_10c),
        0x59c,
    );
    pass1_1008_9436(CONCAT22(unaff_SS, local_112));
    puVar2 = &local_114;
    load_string_1008_a8f4(
        _local_c, puVar2, unaff_SS, 0xee, unaff_SS, &local_10e, unaff_SS,
    );
    _local_118 = CONCAT22(extraout_DX, puVar2);
    struct_a = (extraout_DX | puVar2);
    uVar7 = (param_1 >> 0x10);
    if ((struct_a != 0x0) && (*_local_118 != '\0')) {
        uVar8 = 0x1000;
        puVar3 = puVar2;
        process_struct_1000_179c(0xb4, struct_a);
        _local_120 = CONCAT22(struct_a, puVar3);
        uVar6 = struct_a | puVar3;
        if (uVar6 == 0) {
            uVar4 = 0;
            uVar6 = 0;
        } else {
            uVar8 = SUB42(&PTR_LOOP_1050_1040, 0);
            puVar5 = process_struct_1040_8478(
                _local_120,
                0,
                CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, local_10c)),
                CONCAT22(extraout_DX, puVar2),
                (param_1 + 8),
            );
            uVar4 = SUB42(puVar5, 0);
        }
        _local_11c = CONCAT22(uVar6, uVar4);
        if (local_110 == 0) {
            ppcVar1 = (*_local_11c + 0x74);
            (**ppcVar1)(uVar8, uVar4, uVar6);
        } else {
            ppcVar1 = (*_local_11c + 0x6c);
            (**ppcVar1)(uVar8, uVar4, uVar6, local_112);
        }
        if ((local_8 == 0) || (local_114 == 0)) {
            PostMessage16(0, 0xfc, 0x111, g_h_window);
        }
    }
    if ((local_8 != 0) && (local_114 != 0)) {
        SendMessage16(0, local_114, 0x111, g_h_window);
        (param_1 + 0x112) = 1;
        return;
    }
    if (local_10e == 6) {
        PostMessage16(0, 0xb0, 0x111, g_h_window);
        ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_126, 2));
        local_126 = ppVar9;
        (local_126 + 0x20) = 1;
    }
    if (local_10e == 0x15) {
        PostMessage16(0, 0x97, 0x111, g_h_window);
        ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_126, 2));
        (ppVar9 + 0x20) = 1;
    }
    return;
}

pub fn set_cursor_1020_5764(in_struct_1: *mut astruct_673, param_2: u16) {
    let mut uVar1: i32;
    let puVar2: *mut u8;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut h_cursor: HCURSOR16;
    let local_struct_1: *mut astruct_673;
    let local_struct_1_hi: *mut astruct_673;
    let mut unaff_SS: u16;
    let mut in_stack_0000ffe6: u16;
    let mut in_stack_0000ffe8: HWND16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 2];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 =
        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe6, 0x2f));
    uVar4 = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    uVar1 = (_local_6 + 0x22);
    if ((uVar1 | local_a) != 0) {
        pass1_1030_8308(
            _g_bool_1050_5748,
            (_g_bool_1050_5748 >> 0x10),
            CONCAT22(unaff_SS, &local_e),
            CONCAT22(unaff_SS, local_c),
            local_a & 0xffff | uVar1 << 0x10,
        );
        if (param_2 <= local_e) {
            local_struct_1_hi = (in_struct_1 >> 0x10);
            local_struct_1 = in_struct_1;
            if (local_struct_1.field_0xf4 != 1) {
                SetCursor16(local_struct_1.field_0xee);
                local_struct_1.field_0xee = 0;
                local_struct_1.field_0xf4 = 1;
                &local_struct_1.field_0x10c = 0;
                ReleaseCapture16(in_stack_0000ffe8);
            }
            h_cursor = LoadCursor16(0x7f02, 0);
            SetCursor16(h_cursor);
            local_12 = param_2;
            pass1_1018_017c(_local_6, param_2);
            puVar2 = local_struct_1.field_0xf6;
            (puVar2 + 0x10) = 1;
            if (&local_struct_1.field_0xfe != 0) {
                call_invalidate_rect_1020_68de(*&local_struct_1.field_0xfe);
                uVar3 = &local_struct_1.field_0xfe;
                local_12 = (uVar3 + 8);
                PostMessage16(0, 0xeb, 0x111, local_12);
            }
            SetCursor16(local_12);
        }
    }
    return;
}

pub fn pt_in_rect_1020_5856(in_struct_1: *mut astruct_664, in_struct_2: *mut astruct_665) {
    let puVar1: *mut u32;
    let mut in_a_x: i32;
    let b_var2: bool;
    let mut u_var3: u32;
    let mut extraout_d_x: i32;
    let mut extraout_d_x_00: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1018_2862((in_struct_1 + 0xfa));
    if ((extraout_d_x | in_a_x) != 0) {
        local_a = 0;
        while (true) {
            puVar1 = (in_a_x + 10);
            let pu_var1_val = unsafe { *puVar1 };
            if (pu_var1_val < local_a || pu_var1_val == local_a) {
                break;
            }

            u_var3 = local_a;
            bad_func_1008_8fc4(CONCAT22(extraout_d_x, in_a_x), local_a);
            if ((extraout_d_x_00 | u_var3) != 0) {
                b_var2 = PtInRect16(in_struct_2.field_0x0, (u_var3 + 0x14));
                if (b_var2 != 0) {
                    return;
                }
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub fn win_fn_1020_58ce(in_struct_1: *mut astruct_663, param_2: u16, param_3: u16, param_4: u8) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let rect: *mut RECT16;
    let puVar3: *mut u16;
    let BVar4: bool;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut w_param: WPARAM16;
    let mut in_stack_0000ffcc: u16;
    let mut local_2e: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_3;
    local_4 = param_2;
    local_8 = param_4 & 8;
    rect = (param_4 & 4);
    uVar9 = (in_struct_1 >> 0x10);
    iVar7 = in_struct_1;
    local_a = rect;
    pass1_1020_64d4(*(iVar7 + 0xf6), 2);
    if (rect == 0x0) {
        // LAB_1020_5942:
        pass1_1020_64d4(*(iVar7 + 0xf6), 4);
        if (rect == 0x0) {
            // LAB_1020_5a16:
            pass1_1020_64d4(*(iVar7 + 0xf6), 1);
            uVar6 = ZEXT24(rect);
            if (rect != 0x0) {
                pass1_1020_6498(*(iVar7 + 0xf6), 1);
                _local_1e = uVar6 & 0xffff | extraout_DX_01 << 0x10;
                local_1a = 0;
                loop {
                    rect = uVar6;
                    if (3 < local_1a) {
                        break;
                    }
                    BVar4 = PtInRect16(CONCAT22(local_4, local_6), (local_1a * 8 + local_1e));
                    uVar6 = BVar4;
                    if (BVar4 != 0) {
                        local_18 = 0;
                        local_14 = 0;
                        if (local_1a == 0) {
                            local_14 = (-(local_a == 0) & 4) - 5;
                        } else {
                            if (local_1a == 1) {
                                local_14 = (-(local_a == 0) & 0xfffc) + 5;
                            } else {
                                if (local_1a == 2) {
                                    local_18 = (-(local_a == 0) & 4) - 5;
                                } else {
                                    if (local_1a == 3) {
                                        local_18 = (-(local_a == 0) & 0xfffc) + 5;
                                    }
                                }
                            }
                        }
                        pass1_1020_2a94((iVar7 + 0xce), CONCAT22(local_18, local_14));
                        return;
                    }
                    local_1a = local_1a + 1;
                }
            }
            pass1_1020_64d4(*(iVar7 + 0xf6), 3);
            if (rect != 0x0) {
                local_1e = &local_6;
                pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_SS, local_1e));
                if ((extraout_DX_02 | local_1e) != 0) {
                    local_1a = (local_1e + 0x2e);
                    if (((local_8 == 0) || (local_a == 0)) && (local_a == 0)) {
                        local_18 = 1;
                    } else {
                        local_18 = 2;
                    }
                    local_14 = (local_1e + 0xc);
                    puVar5 = (local_1e + 0xe);
                    _local_12 = CONCAT22(local_10, puVar5);
                    local_1c = extraout_DX_02;
                    if ((local_1a == 0xb) || (local_1a == 0x37)) {
                        uVar2 = (iVar7 + 0xfa);
                        uVar10 = (uVar2 >> 0x10);
                        iVar8 = uVar2;
                        local_2e = (iVar8 + 0x20);
                        puVar5 = ((iVar8 + 0x22) | local_2e);
                        if (puVar5 != 0x0) {
                            zero_list_1008_3e38(CONCAT22(unaff_SS, &stack0xffcc));
                            pass1_1018_161c(
                                local_2e,
                                (local_2e >> 0x10),
                                &stack0xffcc,
                                unaff_SS,
                                local_12,
                                local_14,
                            );
                            _local_26 = process_struct_1010_20ba(
                                _g_astruct_372_1050_0ed0,
                                CONCAT22(in_stack_0000ffcc, 0x2f),
                            );
                            puVar5 = &stack0xffcc;
                            pass1_1010_ecc6(
                                _local_26,
                                CONCAT22(unaff_SS, puVar5),
                                (local_2e + 0x3c),
                            );
                        }
                    }
                    pass1_1018_25d2(
                        (iVar7 + 0xfa),
                        local_18,
                        _local_12 & 0xffff | local_14 << 0x10,
                    );
                    if (puVar5 != 0x0) {
                        return;
                    }
                    BVar4 = pass1_1020_5d56(in_struct_1, CONCAT22(local_1c, local_1e));
                    if (BVar4 != 0) {
                        return;
                    }
                }
            }
            return;
        }
        pass1_1020_6498(*(iVar7 + 0xf6), 4);
        local_e = rect;
        local_c = extraout_DX_00;
        rect = PtInRect16(CONCAT22(local_4, local_6), rect);
        if (rect == 0x0) {}
        // goto LAB_1020_5a16;
        _local_12 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffcc, 2));
        if ((_local_12 + 0x72) != 0) {
            (iVar7 + 0x116) = 1;
            if (in_struct_1 == 0x0) {
                iVar7 = 0;
                uVar9 = 0;
            } else {
                iVar7 = iVar7 + 0xe2;
            }
            _local_1e = CONCAT22(uVar9, iVar7);
            ppcVar1 = (*_g_struct_ptr_1050_02a0 + 4);
            (**ppcVar1)(
                0x1010,
                _g_struct_ptr_1050_02a0,
                (_g_struct_ptr_1050_02a0 >> 0x10),
                0x10,
                iVar7,
                uVar9,
            );
            pass1_1008_941a(CONCAT22(unaff_SS, &local_18), 1, 0x86);
            puVar3 = &local_18;
            mci_send_command_1008_5c9e(_g_struct_ptr_1050_02a0, CONCAT22(unaff_SS, puVar3));
            if (puVar3 != 0x0) {
                return;
            }
            w_param = 0xf6;
            local_1a = puVar3;
            // goto LAB_1020_5936;
        }
        w_param = 0xf6;
    } else {
        pass1_1020_6498(*(iVar7 + 0xf6), 2);
        local_e = rect;
        local_c = extraout_DX;
        rect = PtInRect16(CONCAT22(local_4, local_6), rect);
        if (rect == 0x0) {}
        // goto LAB_1020_5942;
        w_param = 0x68;
    }
    puVar3 = 0x0;
    // LAB_1020_5936:
    PostMessage16(CONCAT22(puVar3, puVar3), w_param, 0x111, g_h_window);
    return;
}

pub fn track_popup_menu_1020_5bf2(param_1: *mut astruct_26, param_2: u16, param_3: u16) -> bool {
    let mut iVar1: i32;
    let mut pt_in_rect: u16;
    let mut tile_menu_handle: u16;
    let mut HVar2: HMENU16;
    let mut uVar3: i32;
    let local_BX_18: *mut astruct_26;
    let mut uVar4: u16;
    let mut unaff_CS: u16;
    let mut h_window: u16;
    let mut uVar5: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut rect: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_3;
    local_4 = param_2;
    uVar4 = (param_1 >> 0x10);
    local_BX_18 = param_1;
    iVar1 = pass1_1020_64d4(local_BX_18.field_0xf6, 2);
    if (iVar1 != 0) {
        rect = pass1_1020_6498(local_BX_18.field_0xf6, 2);
        unaff_CS = SUB42(offset, 0);
        pt_in_rect = PtInRect16(CONCAT22(local_4, local_6), rect);
        if (pt_in_rect != 0) {
            PostMessage16(0, 0x131, 0x111, g_h_window);
            return 1;
        }
    }
    iVar1 = pass1_1020_64d4(local_BX_18.field_0xf6, 3);
    if (iVar1 == 0) {
        return 0;
    }
    uVar5 = pt_in_rect_1020_5856(param_1, CONCAT22(h_window, &local_6));
    uVar3 = (uVar5 >> 0x10);
    local_BX_18.field_0x108 = uVar5;
    &local_BX_18.field_0x10a = uVar3;
    if ((uVar3 | local_BX_18.field_0x108) == 0) {
        return 0;
    }
    if (local_BX_18.menu_handle == 0) {
        tile_menu_handle = LoadMenu16(tile_menu_name, g_h_instance_1050_038c);
        local_BX_18.menu_handle = tile_menu_handle;
        if (tile_menu_handle == 0) {
            return 1;
        }
        unaff_CS = SUB42(offset, 0);
        HVar2 = GetSubMenu16(0, local_BX_18.menu_handle);
        local_BX_18.menu_handle = HVar2;
        if (HVar2 == 0) {
            return 1;
        }
    }
    uVar5 = &local_BX_18.field_0x108;
    rect._0_2_ = (uVar5 + 0x2e);
    local_c = 0;
    if (rect == 0x42) {
        local_c = 0xc9;
    } else {
        if (((s_VrMode_1050_42ca + 8) + rect * 2) == 0) {
            local_c = 200;
        }
    }
    if (local_c != 0) {
        unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
        mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, CONCAT22(local_c, 1));
    }
    local_10 = param_3;
    local_e = param_2;
    ClientToScreen16(CONCAT22(&local_10, unaff_CS), h_window);
    TrackPopupMenu16(0x0, 0, local_BX_18.field_0x8, 0, local_e, local_10, 0);
    return 1;
}

pub fn enable_menu_item_1020_44ec(param_1: u32, param_2: u16_00, param_4: i32, param_3: u16) {
    let mut u_var1: u32;
    let uVar2: u8;
    let mut uVar3: i32;
    let ppVar4: *mut pass1_struct_2;
    let puVar5: *mut u16;
    let BVar6: bool;
    let extraout_var;
    let mut in_DX: i32;
    let mut uVar7: i32;
    let mut extraout_DX: u16;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let uVar12: u8;
    let mut unaff_SS: u16;
    let mut HVar13: HMENU16;
    let mut h_menu: u16;
    let mut local_138: u16;
    let ppStack310: *mut pass1_struct_2;
    let mut local_134: u16;
    let mut local_130: u16;
    let mut local_12e: u16;
    let mut local_12c: u32;
    let mut local_128: u32;
    let mut local_124: u32;
    let mut local_11e: u32;
    let mut local_116: u32;
    let mut local_10e: u16;
    let mut local_10c: u32;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar10 = (param_1 >> 0x10);
    iVar8 = param_1;
    if ((iVar8 + 0x106) != 0) {
        if (*(iVar8 + 0x106) == param_3) {
            _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_138, 3));
            uVar1 = (iVar8 + 0x108);
            local_8 = (uVar1 + 0x2e);
            uVar1 = (iVar8 + 0x108);
            uVar11 = (uVar1 >> 0x10);
            iVar9 = uVar1;
            local_128 = (iVar9 + 0x42);
            uVar7 = (iVar9 + 0x44);
            local_128._3_1_ = (local_128 >> 0x18);
            uVar3 = local_128._3_1_;
            local_11e = local_128;
            local_10c = local_128;
            if (local_128._3_1_ == 0) {
                big_switch_statement_1020_bd80(local_8);
                uVar11 = 0x1000;
                copy_string_1000_3d3e(CONCAT22(unaff_SS, &local_108), CONCAT22(uVar7, uVar3));
            } else {
                ppVar4 = pass1_1030_8344(
                    _g_bool_1050_5748,
                    (_g_bool_1050_5748 >> 0x10),
                    local_128 & 0xffff | uVar7 << 0x10,
                );
                local_128 = CONCAT22(uVar7, ppVar4);
                uVar11 = 0x1010;
                pass1_1010_c3c2(
                    _local_6,
                    (_local_6 >> 0x10),
                    CONCAT22(unaff_SS, &local_108),
                    CONCAT22(uVar7, ppVar4),
                );
            }
            puVar5 = &local_108;
            ModifyMenu16(uVar11, puVar5, unaff_SS, 0x76, 0, 0x76, (iVar8 + 0x106));
            local_10e = puVar5;
            GetMenuState16(offset, 0, 0x3c, (iVar8 + 0x106));
            if (puVar5 != 0xffff) {
                DeleteMenu16(0x38, 0, 0x13c, (iVar8 + 0x106));
            }
            BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_8, 0x20);
            if (BVar6 != 0) {
                load_string_1010_847e(
                    _g_struct_73_1050_14cc,
                    (_g_struct_73_1050_14cc >> 0x10),
                    0x74b,
                );
                local_128 = CONCAT22(extraout_DX, BVar6);
                InsertMenu16(
                    0x1010,
                    BVar6,
                    extraout_DX,
                    0x3c,
                    0x400,
                    0xffff,
                    (iVar8 + 0x106),
                );
            }
            if (((s_VrMode_1050_42ca + 8) + local_8 * 2) == 0) {
                local_128._0_2_ = *(iVar8 + 0x106);
                HVar13 = 1;
                h_menu = 0x77;
                // goto LAB_1020_464c;
            }
            local_128._0_2_ = *(iVar8 + 0x106);
            h_menu = 0x77;
        } else {
            HVar13 = GetSubMenu16(1, (iVar8 + 0x106));
            local_128 = local_128 & 0xffff0000 | HVar13;
            if (HVar13 != param_3) {}
            // goto LAB_1020_479e;
            EnableMenuItem16(1, 0x200, HVar13);
            local_138 = local_128;
            EnableMenuItem16(1, 0x201, local_128);
            ppStack310 = local_128;
            local_138 = 0x202;
            EnableMenuItem16(1, 0x202, local_128);
            uVar1 = (iVar8 + 0x108);
            local_124 = (uVar1 + 0x42);
            ppStack310 = local_124;
            local_134 = (local_124 >> 0x10);
            local_138 = (_g_bool_1050_5748 >> 0x10);
            ppStack310 = pass1_1030_8344(_g_bool_1050_5748, local_138, local_124);
            local_11e = CONCAT22(in_DX, ppStack310);
            if ((in_DX | ppStack310) == 0) {
                return;
            }
            local_116 = &ppStack310.field_0x2e;
            if ((&ppStack310.field_0x30 | local_116) == 0) {
                return;
            }
            local_10c = (local_116 + 0x200);
            local_138 = 0x1030;
            _local_108 = pass1_1030_73a8(CONCAT22(in_DX, ppStack310));
            uVar10 = (_local_108 >> 0x10);
            _local_6 = (_local_108 + 0x1c);
            uVar7 = (_local_108 + 0x1e);
            if ((uVar7 | local_6) != 0) {
                local_10c = _local_6 & 0xffff | uVar7 << 0x10;
            }
            if (local_10c != 1) {
                return;
            }
            if ((local_10c & 0xff0000) != 0) {
                return;
            }
            ppStack310 = local_11e;
            local_134 = (local_11e >> 0x10);
            local_138 = 0x1030;
            uVar2 = pass1_1030_6fa0(local_11e);
            ppStack310 = CONCAT31(extraout_var, uVar2);
            local_134 = 0x3f;
            local_138 = (_PTR_LOOP_1050_06e0 >> 0x10);
            BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, ppStack310, 0x3f);
            if (BVar6 != 0) {
                local_134 = local_128;
                local_138 = 0;
                ppStack310 = 0x201;
                BVar6 = EnableMenuItem16(0, 0x201, local_128);
            }
            if ((local_11e + 0x36) != 0) {
                BVar6 = EnableMenuItem16(0, 0x202, local_128);
            }
            pass1_1030_69cc(local_11e);
            if (BVar6 == 0) {
                return;
            }
            h_menu = 0x200;
        }
        HVar13 = 0;
        // goto LAB_1020_464c;
    }
    // LAB_1020_479e:
    iVar9 = param_2 + -1;
    if (iVar9 == 0) {
        uVar1 = (iVar8 + 0xfa);
        pass1_1018_2504(uVar1, (uVar1 >> 0x10));
        if (iVar9 == 0) {
            h_menu = 0;
            EnableMenuItem16(0x401, 0, param_3);
            local_138 = param_3;
            local_128._0_2_ = 1;
            // LAB_1020_47e3:
            HVar13 = 0x401;
            // goto LAB_1020_464c;
        }
        h_menu = 0;
        EnableMenuItem16(0x400, 0, param_3);
        local_138 = param_3;
        local_128._0_2_ = 1;
    } else {
        iVar9 = param_2 + -2;
        if (iVar9 == 0) {
            pass1_1020_64d4(*(iVar8 + 0xf6), 2);
            if (iVar9 == 0) {
                EnableMenuItem16(0x401, 0, param_3);
                h_menu = 0x401;
            } else {
                EnableMenuItem16(0x400, 0, param_3);
                h_menu = 0x400;
            }
            local_128._0_2_ = 1;
            local_138 = param_3;
            EnableMenuItem16(h_menu, 1, param_3);
            if ((PTR_LOOP_1050_0010 != 0x0) || ((iVar8 + 0x102) == 0)) {
                ppStack310 = param_3;
                local_138 = 5;
                // goto LAB_1020_47e3;
            }
            ppStack310 = param_3;
            local_138 = 5;
        } else {
            if (param_2 == 3) {
                local_138 = 0;
                local_134 = 0;
                uVar12 = 0x10;
                _local_130 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x2f);
                uVar10 = (_local_130 >> 0x10);
                local_12c = (_local_130 + 0x20);
                uVar7 = (_local_130 + 0x22);
                if ((uVar7 | local_12c) != 0) {
                    uVar12 = 0x30;
                    pass1_1030_8308(
                        _g_bool_1050_5748,
                        (_g_bool_1050_5748 >> 0x10),
                        CONCAT22(unaff_SS, &local_134),
                        CONCAT22(unaff_SS, &local_138),
                        local_12c & 0xffff | uVar7 << 0x10,
                    );
                    local_138 = (_local_130 + 0x1e);
                }
                local_128 = local_128 & 0xffff0000;
                while {
                    CheckMenuItem16(uVar12, 0x400, local_128, param_3);
                    uVar12 = 0x38;
                    EnableMenuItem16(0x401, local_128, param_3);
                    local_128 = local_128 & 0xffff0000 | (local_128 + 1);
                    (local_128 + 1) < 5
                } {}
                CheckMenuItem16(0x38, 0x408, local_138, param_3);
                local_128 = local_128 & 0xffff0000;
                while (local_128 <= local_134) {
                    EnableMenuItem16(0x400, local_128, param_3);
                    local_128 = local_128 & 0xffff0000 | (local_128 + 1);
                }
                return;
            }
            if (param_2 != 4) {
                return;
            }
            h_menu = 2;
            local_128._0_2_ = param_3;
        }
    }
    HVar13 = 0x400;
    // LAB_1020_464c:
    EnableMenuItem16(HVar13, h_menu, local_128);
    return;
}

pub fn win_fn_1020_493c(in_struct_1: *mut astruct_673, param_2: u16) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let in_bool_1: bool;
    let mut HVar3: HCURSOR16;
    let puVar4: *mut u32;
    let mut uVar5: i32;
    let mut iVar6: i32;
    let in_DX: *mut astruct_199;
    let puVar7: *mut u8;
    let mut extraout_DX: u16;
    let struct_a: *mut astruct_199;
    let mut uVar8: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: u16;
    let mut in_BX: i32;
    let local_struct_1: *mut astruct_673;
    let local_struct_1_hi: *mut astruct_673;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let ppVar10: *mut pass1_struct_1;
    let in_bool_2: bool;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut local_356: u16;
    let mut local_354: u16;
    let mut local_24e: u16;
    let mut local_24c: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u32;
    let mut local_13c: u16;
    let mut local_13a: u16;
    let mut local_2a: u16;
    let mut local_26: u16;
    let mut local_24: u16;
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
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u32;

    if (param_2 == 0xe9) {
        return;
    }
    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    if (param_2 < 0xea) {
        match param_2 {
            0x69 => uVar12 = 0,
            0x6a => uVar12 = 1,
            0x6b => uVar12 = 2,
            0x6c => uVar12 = 3,
            0x6d => uVar12 = 4,
            0x77 => {
                if ((&local_struct_1.field_0x10a | local_struct_1.field_0x108) == 0) {
                    return;
                }
                uVar2 = &local_struct_1.field_0x108;
                uVar8 = ((s_VrMode_1050_42ca + 8) + (uVar2 + 0x2e) * 2);
                local_1a = local_1a & 0xffff0000 | uVar8;
                if (uVar8 == 0) {
                    return;
                }
                mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1f8);
                _local_12 = CONCAT22(extraout_DX, uVar8);
                WinHelp16(
                    CONCAT13(
                        (local_1a >> 0xf),
                        CONCAT12((local_1a >> 0xf), local_1a & 0xff | (local_1a >> 8) << 8),
                    ),
                    1,
                    CONCAT22(extraout_DX, uVar8),
                    local_struct_1.field_0x8,
                );
                return;
            }
            0x78 => {
                ppVar10 =
                    process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_356, 0x45));
                local_24c = (ppVar10 >> 0x10);
                local_24e = ppVar10;
                enum_child_windows_1010_01be(local_24e, local_24c, local_struct_1.field_0x8);
                return;
            }
            _ => {
                return;
            }
        }
        set_cursor_1020_5764(in_struct_1, uVar12);
        return;
    }
    if (param_2 == 0x132) {
        pass1_1020_64d4(local_struct_1.field_0xf6, 3);
        if (param_2 == 0) {
            return;
        }
        uVar9 = 0xffff;
        // goto LAB_1020_4ef8;
    }
    if (param_2 < 0x133) {
        if (param_2 == 0x102) {
            uVar9 = 0x1000;
            process_struct_1000_179c(0xb4, in_DX);
            uVar8 = in_DX | param_2;
            local_22 = param_2;
            if (uVar8 == 0) {
                uVar5 = 0;
                uVar8 = 0;
            } else {
                uVar9 = SUB42(&PTR_LOOP_1050_1040, 0);
                uVar5 = param_2;
                mixed_1040_8520(
                    CONCAT13((in_DX >> 8), CONCAT12(in_DX, param_2)),
                    g_h_window,
                    0x31,
                    2,
                    0x57b,
                    0x62b,
                );
            }
            _local_144 = CONCAT22(uVar8, uVar5);
            ppcVar1 = (_local_144 + 0x74);
            (**ppcVar1)(uVar9, uVar5, uVar8);
            local_140 = CONCAT22(local_140._2_2_, uVar5);
            if (uVar5 != 1) {
                return;
            }
            pass1_1028_837e(&local_13c, unaff_SS);
            // LAB_1020_4b6c:
            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_13c));
            return;
        }
        if (param_2 < 0x103) {
            match param_2 {
                0xf0 => {
                    gui_window_func_1020_536e(in_struct_1, (in_struct_1 >> 0x10), 0, 0, 0xffff, 1);
                    return;
                }
                _ => {
                    return;
                }
                0xf6 => {
                    if (local_struct_1.field_0x116 != 0) {
                        if (in_struct_1 == 0x0) {
                            local_356 = 0;
                            local_354 = 0;
                        } else {
                            local_356 = &local_struct_1.field_0xe2;
                            local_354 = local_struct_1_hi;
                        }
                        pass1_1010_1ea6(_g_struct_ptr_1050_02a0, CONCAT22(local_354, local_356));
                        local_struct_1.field_0x116 = 0;
                    }
                    uVar12 = 0x12;
                }
                0xf7 => {
                    uVar2 = local_struct_1.field_0x10e;
                    window_msg_func_1010_7300(uVar2, (uVar2 >> 0x10), 0, 0, 9, 0, 0);
                    return;
                }
                0xfb => {
                    _local_144 =
                        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_356, 3));
                    local_140 = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(local_356, 0x30),
                    );
                    uVar9 = local_140;
                    pass1_1010_375e(local_140);
                    pass1_1010_c25e(_local_144, CONCAT22(extraout_DX_01, uVar9));
                    return;
                }
                0xfc => {
                    post_msg_1020_55b0(in_struct_1);
                    return;
                }
                0x101 => {
                    local_1a = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(local_356, 0x2f),
                    );
                    uVar9 = (local_1a >> 0x10);
                    _local_16 = (local_1a + 0x24);
                    local_20 = (local_1a + 0x26);
                    puVar4 = (local_20 | local_16);
                    if (puVar4 == 0x0) {
                        uVar9 = 0x1000;
                        process_struct_1000_179c(0xb4, local_20);
                        uVar8 = local_20 | puVar4;
                        local_22 = puVar4;
                        if (uVar8 == 0) {
                            puVar4 = 0x0;
                            uVar8 = 0;
                        } else {
                            uVar9 = SUB42(&PTR_LOOP_1050_1040, 0);
                            mixed_1040_8520(
                                CONCAT13((local_20 >> 8), CONCAT12(local_20, puVar4)),
                                g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x730,
                            );
                        }
                        _local_1e = CONCAT22(uVar8, puVar4);
                        // LAB_1020_4c5f:
                        let pu_var4_val = unsafe { *puVar4 };
                        ppcVar1 = (pu_var4_val + 0x74);
                        (**ppcVar1)(uVar9, puVar4, uVar8);
                        return;
                    }
                    pass1_1038_af40(_g_astruct_112_a, local_struct_1.field_0x8, 0xe);
                    _local_12 = process_struct_1010_20ba(
                        _g_astruct_372_1050_0ed0,
                        CONCAT22(local_356, 0x43),
                    );
                    uVar9 = (_local_12 >> 0x10);
                    iVar6 = _local_12;
                    local_e = (iVar6 + 10);
                    local_a = (iVar6 + 0xc);
                    uVar12 = (iVar6 + 0xe);
                    local_6 = CONCAT22(local_6._2_2_, uVar12);
                    if ((iVar6 + 0x10) != 0) {
                        return;
                    }
                    pass1_1028_84ca(
                        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_13c)),
                        _local_16,
                        uVar12,
                        local_a,
                        local_e,
                    );
                    // goto LAB_1020_4b6c;
                }
            }
        } else {
            if (param_2 != 0x106) {
                if (param_2 < 0x107) {
                    if (param_2 == 0x103) {
                        _local_144 = process_struct_1010_20ba(
                            _g_astruct_372_1050_0ed0,
                            CONCAT22(local_356, 0x2f),
                        );
                        uVar9 = (_local_144 >> 0x10);
                        local_140 = (_local_144 + 0x24);
                        local_20 = (_local_144 + 0x26);
                        local_22 = local_20 | local_140;
                        if (local_22 != 0x0) {
                            pass1_1038_af40(_g_astruct_112_a, local_struct_1.field_0x8, 0xf);
                            _local_13c = process_struct_1010_20ba(
                                _g_astruct_372_1050_0ed0,
                                CONCAT22(local_356, 0x42),
                            );
                            local_2a = (_local_13c + 10);
                            if (local_2a == 0) {
                                return;
                            }
                            pass1_1030_e63e(CONCAT22(unaff_SS, &local_24e), local_2a);
                            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_24e));
                            return;
                        }
                        uVar9 = 0x1000;
                        process_struct_1000_179c(0xb4, local_20);
                        uVar8 = local_20 | local_22;
                        if (uVar8 == 0) {
                            puVar4 = 0x0;
                            uVar8 = 0;
                        } else {
                            uVar9 = SUB42(&PTR_LOOP_1050_1040, 0);
                            puVar4 = local_22;
                            mixed_1040_8520(
                                CONCAT13((local_20 >> 8), CONCAT12(local_20, local_22)),
                                g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x730,
                            );
                        }
                        _local_26 = CONCAT22(uVar8, puVar4);
                    } else {
                        if (param_2 != 0x104) {
                            return;
                        }
                        ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x220003);
                        local_24c = (ppVar10 >> 0x10);
                        local_22 = ppVar10;
                        local_24e = local_22;
                        pass1_1010_af66(ppVar10);
                        _local_144 = CONCAT22(local_142, local_22);
                        local_20 = struct_a;
                        if (local_22 != 0x0) {
                            uVar9 = 0x1000;
                            process_struct_1000_179c(0xb4, struct_a);
                            uVar8 = local_20 | local_22;
                            if (uVar8 == 0) {
                                puVar4 = 0x0;
                                uVar8 = 0;
                            } else {
                                uVar9 = SUB42(&PTR_LOOP_1050_1040, 0);
                                puVar4 = local_22;
                                mixed_1040_8520(
                                    CONCAT13((local_20 >> 8), CONCAT12(local_20, local_22)),
                                    g_h_window,
                                    0x31,
                                    2,
                                    0x57b,
                                    0x62c,
                                );
                            }
                            local_140 = CONCAT22(uVar8, puVar4);
                            ppcVar1 = (local_140 + 0x74);
                            (**ppcVar1)(uVar9, puVar4, uVar8);
                            _local_13c = CONCAT22(local_13a, puVar4);
                            if (puVar4 != (&PTR_LOOP_1050_0000 + 1)) {
                                return;
                            }
                            pass1_1030_e79a(0xaa, unaff_SS);
                            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_356));
                            mci_send_cmd_1008_5c5c(_g_struct_ptr_1050_02a0, 0x1e6);
                            return;
                        }
                        uVar9 = 0x1000;
                        process_struct_1000_179c(0xb4, struct_a);
                        uVar8 = local_20 | local_22;
                        if (uVar8 == 0) {
                            puVar4 = 0x0;
                            uVar8 = 0;
                            local_356 = puVar4;
                            local_354 = uVar8;
                        } else {
                            uVar9 = SUB42(&PTR_LOOP_1050_1040, 0);
                            puVar4 = local_22;
                            mixed_1040_8520(
                                CONCAT13((local_20 >> 8), CONCAT12(local_20, local_22)),
                                g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x731,
                            );
                            local_356 = puVar4;
                            local_354 = uVar8;
                        }
                    }
                    // goto LAB_1020_4c5f;
                }
                if (param_2 == 0x12f) {
                    pass1_1020_61c4(
                        local_struct_1,
                        local_struct_1_hi,
                        CONCAT22(unaff_SS, &local_144),
                        CONCAT22(unaff_SS, &local_24e),
                    );
                    iVar6 = local_24e + 0x6a;
                } else {
                    if (param_2 != 0x130) {
                        iVar6 = param_2 - 0x131;
                        if (iVar6 != 0) {
                            return;
                        }
                        pass1_1020_64d4(local_struct_1.field_0xf6, 2);
                        if (iVar6 == 0) {
                            return;
                        }
                        uVar12 = 7;
                        // goto LAB_1020_49b7;
                    }
                    pass1_1020_61c4(
                        local_struct_1,
                        local_struct_1_hi,
                        CONCAT22(unaff_SS, &local_144),
                        CONCAT22(unaff_SS, &local_24e),
                    );
                    iVar6 = local_24e + 0x68;
                }
                local_140 = CONCAT22(local_140._2_2_, iVar6);
                if (0x6d < iVar6) {
                    return;
                }
                if (iVar6 < 0x69) {
                    return;
                }
                ppcVar1 = (in_struct_1 + 0x40);
                (**ppcVar1)();
                return;
            }
            uVar12 = 0x13;
        }
        // LAB_1020_49b7:
        pass1_1038_af40(_g_astruct_112_a, local_struct_1.field_0x8, uVar12);
        return;
    }
    if (param_2 == 0x1c8) {
        uVar2 = local_struct_1.field_0x102;
        SendMessage16(0, 0x72, 0x111, (uVar2 + 8));
        return;
    }
    if (0x1c8 < param_2) {
        if (param_2 == 0x1ca) {
            _local_144 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_356, 3));
            uVar8 = _local_144;
            pass1_1010_c234(_local_144);
            local_140 = CONCAT22(extraout_DX_00, uVar8);
            if ((extraout_DX_00 | uVar8) == 0) {
                return;
            }
            uVar5 = extraout_DX_00;
            _local_13c = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar8, 0x30));
            pass1_1010_3770(_local_13c, CONCAT22(uVar5, uVar8));
            uVar12 = 3;
        } else {
            in_bool_2 = (_g_bool_1050_5748 >> 0x10);
            in_bool_1 = _g_bool_1050_5748;
            if (param_2 == 0x200) {
                uVar2 = &local_struct_1.field_0x108;
                uVar9 = (uVar2 >> 0x10);
                iVar6 = uVar2;
                local_1a = (iVar6 + 0x42);
                local_c = (iVar6 + 0x44);
                local_1a._3_1_ = (local_1a >> 0x18);
                if (local_1a._3_1_ != '\x05') {
                    return;
                }
                PTR_LOOP_1050_5f0c =
                    pass1_1030_8344(in_bool_1, in_bool_2, local_1a & 0xffff | local_c << 0x10);
                uVar12 = 0x25;
                PTR_LOOP_1050_5f0e = local_c;
                local_e = PTR_LOOP_1050_5f0c;
            } else {
                if (param_2 == 0x201) {
                    uVar2 = &local_struct_1.field_0x108;
                    uVar9 = (uVar2 >> 0x10);
                    iVar6 = uVar2;
                    local_1a = (iVar6 + 0x42);
                    local_c = (iVar6 + 0x44);
                    local_1a._3_1_ = (local_1a >> 0x18);
                    if (local_1a._3_1_ != '\x05') {
                        return;
                    }
                    PTR_LOOP_1050_5f16 =
                        pass1_1030_8344(in_bool_1, in_bool_2, local_1a & 0xffff | local_c << 0x10);
                    uVar12 = 0x26;
                    PTR_LOOP_1050_5f18 = local_c;
                    local_e = PTR_LOOP_1050_5f16;
                } else {
                    if (param_2 != 0x202) {
                        if (param_2 != 0x203) {
                            return;
                        }
                        if (local_struct_1.field_0xf4 != 1) {
                            return;
                        }
                        HVar3 = SetCursor16(local_struct_1.field_0xf2);
                        local_struct_1.field_0xee = HVar3;
                        local_struct_1.field_0xf4 = 3;
                        local_356 = local_struct_1.field_0x8;
                        SetCapture16(local_356);
                        return;
                    }
                    uVar2 = &local_struct_1.field_0x108;
                    uVar9 = (uVar2 >> 0x10);
                    iVar6 = uVar2;
                    local_6 = (iVar6 + 0x42);
                    puVar7 = (iVar6 + 0x44);
                    local_6._3_1_ = (local_6 >> 0x18);
                    if (local_6._3_1_ != '\x05') {
                        return;
                    }
                    PTR_LOOP_1050_5a68 = pass1_1030_8344(
                        in_bool_1,
                        in_bool_2,
                        local_6 & 0xffff | ZEXT24(puVar7) << 0x10,
                    );
                    _local_16 = CONCAT22(puVar7, PTR_LOOP_1050_5a68);
                    uVar12 = 0x27;
                    PTR_LOOP_1050_5a6a = puVar7;
                }
            }
        }
        // goto LAB_1020_49b7;
    }
    match param_2 {
        0x133 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_BX == 0) {
                return;
            }
            uVar11 = 0xffff;
            uVar9 = 0;
        }
        0x134 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_BX == 0) {
                return;
            }
            uVar9 = 1;
            // goto LAB_1020_4ef8;
        }
        0x135 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_BX == 0) {
                return;
            }
            uVar11 = 1;
            uVar9 = 0;
        }
        0x136 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_BX == 0) {
                return;
            }
            uVar9 = 0xfffb;
            // goto LAB_1020_4ef8;
        }
        0x137 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_BX == 0) {
                return;
            }
            uVar11 = 0xfffb;
            uVar9 = 0;
        }
        0x138 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_BX == 0) {
                return;
            }
            uVar9 = 5;
            // LAB_1020_4ef8:
            uVar11 = 0;
        }
        0x139 => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 3);
            if (in_BX == 0) {
                return;
            }
            uVar11 = 5;
            uVar9 = 0;
        }
        // default:
        // goto switchD_1020_518a_caseD_13a;
        0x13c => {
            pass1_1020_64d4(local_struct_1.field_0xf6, 2);
            if (in_BX != 0) {
                uVar12 = 0x1a;
                // goto LAB_1020_49b7;
            }
            // goto switchD_1020_518a_caseD_13a;
        }
    }
    pass1_1020_2a94(local_struct_1.field_0xce, CONCAT22(uVar9, uVar11));
    // switchD_1020_518a_caseD_13a:
    return;
}

pub fn win_fn_1020_51c6(in_struct_1: *mut astruct_663, param_2: u16, param_3: u32) {
    let mut iVar1: i32;
    let local_struct_1: *mut astruct_663;
    let local_struct_1_hi: *mut astruct_663;
    let in_struct_2: *mut astruct_665;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    iVar1 = local_struct_1.field_0xf4;
    in_struct_2 = param_3;
    if (iVar1 == 2) {
        win_fn_1020_5e76(
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
            param_2,
            in_struct_2,
        );
        return;
    }
    if (iVar1 != 3) {
        iVar1 = win_fn_1020_58ce(
            (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
            param_2,
            in_struct_2,
            (param_3 >> 0x10),
        );
        if (iVar1 == 0) {
            fn_ptr_1 = (local_struct_1.field_0x4 + 0x5c);
            (**fn_ptr_1)();
        }
        return;
    }
    set_cursor_1020_5de8(
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        param_2,
        in_struct_2,
    );
    return;
}

pub fn win_func_1020_522e(param_1: u32, param_2: u16, param_3: u16) {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let b_var3: bool;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let ppVar6: *mut pass1_struct_1;
    let mut in_stack_0000fffc: HWND16;
    let mut uVar7: u16;
    let mut uVar8: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar1 = (iVar4 + 0xf4);
    if (iVar1 == 2) {
        SetCursor16((iVar4 + 0xee));
        (iVar4 + 0xee) = 0;
        (iVar4 + 0xf4) = 1;
        (iVar4 + 0x10c) = 0;
        ReleaseCapture16(in_stack_0000fffc);
        return;
    }
    if (iVar1 == 3) {
        SetCursor16((iVar4 + 0xee));
        (iVar4 + 0xee) = 0;
        (iVar4 + 0xf4) = 1;
        (iVar4 + 0x10c) = 0;
        ReleaseCapture16(in_stack_0000fffc);
        uVar7 = 0;
        uVar8 = 0;
        ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x47);
        pass1_1018_57e6(ppVar6, CONCAT22(uVar8, uVar7));
        return;
    }
    b_var3 = track_popup_menu_1020_5bf2((param_1 & 0xffff | uVar5 << 0x10), param_2, param_3);
    if (b_var3 == 0) {
        ppcVar2 = ((iVar4 + 4) + 0x60);
        ppcVar2();
    }
    return;
}

pub fn pass1_1020_52de(in_struct_1: *mut astruct_594) {
    // fn_ptr_2: *mut *mut u8;
    let local_struct_1_2: *mut astruct_594;
    let local_struct_1: *mut astruct_594;
    let local_struct_1_hi: *mut astruct_594;
    let mut in_stack_0000fff2: u16;
    // fn_ptr_1: *mut *mut u8;
    let temp_5f6e246310: *mut u32;
    let mut local_u16_2: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    temp_5f6e246310 = local_struct_1.field_0xf6;
    local_u16_2 = local_struct_1.field_0xf8;
    if ((local_u16_2 | temp_5f6e246310) != 0) {
        fn_ptr_1 = unsafe { *temp_5f6e246310 };
        (**fn_ptr_1)();
    }
    &local_struct_1.field_0xf6 = 0;
    if (local_struct_1.u8_ptr_32_0xfa != 0x0) {
        if ((local_struct_1_hi | local_struct_1) == 0) {
            fn_ptr_2 = 0x0;
            local_struct_1_2 = 0x0;
        } else {
            fn_ptr_2 = &local_struct_1.u32_xE2;
            local_struct_1_2 = local_struct_1_hi;
        }
        pass1_1010_1ea6(
            local_struct_1.u8_ptr_32_0xfa,
            CONCAT22(local_struct_1_2, fn_ptr_2),
        );
    }
    destroy_win_1008_628e(in_struct_1, in_stack_0000fff2);
    if (local_struct_1.u8_ptr_32_0xfa != 0x0) {
        pass1_1010_1dda(local_struct_1.u8_ptr_32_0xfa);
    }
    local_struct_1.u8_ptr_32_0xfa = 0x0;
    return;
}

pub fn gui_window_func_1020_536e(
    param_1: *mut astruct_675,
    param_2: u32,
    param_3: i32,
    param_4: i32,
) {
    let puVar1: *mut u16;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: i32;
    let in_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let puVar7: *mut u32;
    let local_BX_557: *mut astruct_18;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let ppVar9: *mut pass1_struct_1;
    let in_struct_1: *mut astruct_651;
    // ppuVar10: *mut *mut u8;
    let mut uVar11: u32;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
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

    uVar6 = param_4 - 1;
    uVar8 = (param_1 >> 0x10);
    local_BX_557 = param_1;
    if (uVar6 == 0) {
        if (&local_BX_557.field_0xfe == 0) {
            process_struct_1000_179c(0xfc, in_DX);
            if ((in_DX | uVar6) == 0) {
                &local_BX_557.field_0xfe = 0;
            } else {
                puVar1 = &local_BX_557.field_0xcc;
                unsafe { *puVar1 = *puVar1 + 1 };
                uVar11 = load_cursor_1020_67ce(
                    uVar6,
                    in_DX,
                    local_BX_557.field_0xcc,
                    local_BX_557,
                    uVar8,
                );
                &local_BX_557.field_0xfe = uVar11;
                local_BX_557.field_0x100 = (uVar11 >> 0x10);
            }
            pass1_1008_6978(param_1, 0, &local_BX_557.field_0xfe);
            uVar11 = &local_BX_557.field_0xfe;
            uVar12 = uVar11;
            uVar13 = (uVar11 >> 0x10);
            uVar11 = &local_BX_557.field_0xfe;
            uVar8 = (uVar11 >> 0x10);
            puVar7 = uVar11;
            // goto LAB_1020_53f3;
        }
    } else {
        if (param_4 == 2) {
            uVar4 = param_3 + 0xc;
            ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, uVar4));
            uVar11 = pass1_1018_0afa(ppVar9);
            struct_a = (uVar11 >> 0x10);
            if (uVar11 == 0) {
                puVar1 = &local_BX_557.field_0xcc;
                unsafe { *puVar1 = *puVar1 + 1 };
                uVar14 = local_BX_557.field_0xcc;
                uVar12 = param_2;
                uVar13 = (param_2 >> 0x10);
                uVar5 = uVar14;
                process_struct_1000_179c(0xfe, struct_a);
                if ((struct_a | uVar5) == 0) {
                    in_struct_1 = 0x0;
                } else {
                    in_struct_1 =
                        pass1_1020_289a(uVar5, struct_a, uVar14, param_1, (param_1 >> 0x10));
                }
                pass1_1020_294a(in_struct_1, CONCAT22(uVar13, uVar12), uVar4);
                uVar11 = in_struct_1;
                ppcVar2 = (uVar11 + 8);
                ppcVar2(0x1000, in_struct_1);
                pass1_1008_3e0e(in_struct_1);
                pass1_1008_6978(param_1, uVar14, in_struct_1);
                ppcVar2 = (uVar11 + 0xc);
                ppcVar2(8, in_struct_1, (in_struct_1 >> 0x10), 1);
            } else {
                ppuVar10 = pass1_1018_0ad4(ppVar9);
                pass1_1008_3e0e(ppuVar10);
            }
            pass1_1018_1662(ppVar9, (ppVar9 >> 0x10), 0, 0);
            uVar3 = local_BX_557.field_0xce;
            BringWindowToTop16((uVar3 + 8));
            uVar4 = 1;
            uVar14 = 4;
            ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
            pass1_1010_089e(ppVar9, uVar4, uVar14);
            pass1_1010_089e(ppVar9, 1, 3);
            return;
        }
        uVar6 = param_4 - 3;
        if ((uVar6 == 0) && (&local_BX_557.field_0x102 == 0)) {
            process_struct_1000_179c(0xfc, in_DX);
            if ((in_DX | uVar6) == 0) {
                &local_BX_557.field_0x102 = 0;
            } else {
                puVar1 = &local_BX_557.field_0xcc;
                unsafe { *puVar1 = *puVar1 + 1 };
                uVar11 = win_fn_1020_0dc4(
                    CONCAT13((in_DX >> 8), CONCAT12(in_DX, uVar6)),
                    local_BX_557.field_0xcc,
                    param_1,
                );
                local_BX_557.field_0x102 = uVar11;
                &local_BX_557.field_0x104 = (uVar11 >> 0x10);
            }
            pass1_1008_6978(param_1, 0, &local_BX_557.field_0x102);
            uVar11 = &local_BX_557.field_0x102;
            uVar12 = uVar11;
            uVar13 = (uVar11 >> 0x10);
            uVar11 = &local_BX_557.field_0x102;
            uVar8 = (uVar11 >> 0x10);
            puVar7 = uVar11;
            // LAB_1020_53f3:
            unsafe { ppcVar2 = (*puVar7 + 0xc) };
            ppcVar2(8, uVar12, uVar13, 5);
            return;
        }
    }
    return;
}

pub fn draw_1020_40ce(
    param_1: u32,
    param_2: *mut astruct_132,
    param_3: *mut astruct_134,
    in_hdc_4: u16,
) {
    let mut stock_obj_5: u16;
    let mut obj_handle: HPEN16;
    let mut stock_obj: u16;
    let mut HVar1: HGDIOBJ16;
    let mut rect_right: u16;
    let mut rect_bottom: u16;
    let mut unaff_SS: u16;
    let mut hdc_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut hdc_a: u16;
    let temp_5f8e606965: *mut astruct_133;

    pass1_1008_3e94(
        &(param_1)[1].field_0x4,
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    pass1_1008_3e94(
        param_1,
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    temp_5f8e606965 = (param_1).field_0xa;
    Ellipse16(
        param_2 + (temp_5f8e606965 + local_6),
        (param_3 + (temp_5f8e606965 + local_4)),
        (param_2 + (local_6 - (param_1).field_0xa)),
        (param_3 + (local_4 - (param_1).field_0xa)),
        in_hdc_4,
    );
    if ((*&(param_1)[1].field_0x2 & 1) != 0) {
        stock_obj_5 = GetStockObject16(5);
        SelectObject16(stock_obj_5, hdc_e);
        obj_handle = CreatePen16(0x10000f9, 1, 0);
        SelectObject16(obj_handle, in_hdc_4);
        rect_right = (param_3 + local_4 + 5);
        rect_bottom = (param_2 + local_6 + 5);
        Rectangel16(
            rect_bottom,
            rect_right,
            (param_2 + local_6 + -5),
            (param_3 + local_4 + -5),
            in_hdc_4,
        );
        stock_obj = GetStockObject16(0);
        SelectObject16(stock_obj, rect_right);
        HVar1 = GetStockObject16(6);
        HVar1 = SelectObject16(HVar1, rect_bottom);
        DeleteObject16(HVar1);
    }
    return;
}

pub fn pass1_1020_3c74(param_1: u16, param_2: u32, param_3: u32) {
    call_pt_in_rect_fn_1020_3c8c(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
    );
    return;
}

pub fn call_pt_in_rect_fn_1020_3c8c(param_1: *mut void, param_2: u32) {
    pt_in_rect_1018_1bda((param_1 + 0xfa), param_2);
    return;
}

pub fn cleanup_fn_1020_3cb8(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    let local_struct_1: *mut astruct_659;
    let mut local_a: u16;
    let mut local_8: u16;

    if (param_1 == 0x0) {
        local_struct_1 = 0x0;
        param_1._2_2_ = 0;
    } else {
        local_struct_1 = (param_1 + 0xf2);
    }
    _local_a = CONCAT22(param_1._2_2_, local_struct_1);
    *_local_a = s_1_1050_389a;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1008;
    destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn draw_1020_3da4(param_1: *mut u16, param_2: u32) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut iVar4: u16;
    let mut HVar5: HGDIOBJ16;
    let puVar6: *mut u16;
    let mut h_dc: HDC16;
    let ppVar7: *mut pass1_struct_1;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut h_dc_00: HDC16;
    let mut local_16: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    iVar8 = param_1;
    uVar9 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(iVar8, uVar9, param_2, (param_2 >> 0x10));
    (iVar8 + 0x14) = 0;
    unsafe { *param_1 = 0x408a };
    (iVar8 + 2) = 0x1020;
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_16, 6));
    h_dc = (ppVar7 >> 0x10);
    (iVar8 + 0x14) = ppVar7;
    (iVar8 + 0x16) = h_dc;
    h_dc_00 = 0;
    ppcVar2 = ((iVar8 + 0x14) + 4);
    ppcVar2(0x1010, (iVar8 + 0x14), h_dc, 0, iVar8, uVar9);
    local_4 = GetDC16((iVar8 + 4));
    iVar4 = SetMapMode16(1, local_4);
    (iVar8 + 0x1e) = iVar4;
    HVar5 = GetStockObject16(0);
    HVar5 = SelectObject16(HVar5, h_dc);
    (iVar8 + 0x18) = HVar5;
    HVar5 = GetStockObject16(6);
    HVar5 = SelectObject16(HVar5, h_dc_00);
    (iVar8 + 0x1a) = HVar5;
    uVar3 = (iVar8 + 0x14);
    puVar1 = (uVar3 + 0x24);
    puVar6 = &local_4;
    unsafe { ppcVar2 = (*puVar1 + 8) };
    ppcVar2(offset, puVar1, (puVar1 >> 0x10), puVar6);
    (iVar8 + 0x1c) = puVar6;
    uVar3 = (iVar8 + 0x14);
    (uVar3 + 0x4c) = local_4;
    return;
}

pub fn draw_1020_3e84(in_struct_1: *mut astruct_45) {
    let mut h_dc: HDC16;
    let puVar1: *mut u8;
    let mut h_gdi_obj: HPALETTE16;
    let local_struct_1: *mut astruct_45;
    let local_struct_1_hi: *mut astruct_45;
    let mut local_4: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.u16_0x0 = 0x408a;
    local_struct_1.u16_0x2 = 0x1020;
    pass1_1010_1ea6(
        local_struct_1.field_0x14,
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
    );
    puVar1 = local_struct_1.field_0x14;
    h_dc = (puVar1 + 0x4c);
    SelectObject16(local_struct_1.object_0x18, h_dc);
    SelectObject16(local_struct_1.object_0x1a, h_dc);
    h_gdi_obj = SelectPalette16(0, local_struct_1.palette, h_dc);
    DeleteObject16(h_gdi_obj);
    SetMapMode16(local_struct_1.mode, h_dc);
    in_struct_1.u16_0x0 = s_0_020_1050_3ab0;
    local_struct_1.u16_0x2 = &PTR_LOOP_1050_1008;
    in_struct_1.u16_0x0 = s_1_1050_389a;
    local_struct_1.u16_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn validate_rect_1020_3f12(param_1: u32, param_2: i32) {
    let mut h_wnd: u16;
    let mut rect: u32;
    let mut local_6: u32;

    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    rect = 0x8000e;
    local_6 = 0x1100116;
    InvalidateRect16(0, &rect, h_wnd);
    rect = 0xf10000;
    local_6 = 0x1220030;
    ValidateRect16(&rect, h_wnd);
    rect = 0xf100f5;
    local_6 = 0x1220127;
    ValidateRect16(&rect, h_wnd);
    return;
}

pub fn draw_1020_3fa0(param_1: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: HWND16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    BeginPaint16(CONCAT22(unaff_SS, &local_22), (iVar3 + 4));
    uVar2 = (iVar3 + 0x14);
    local_24 = (uVar2 + 0x4c);
    uVar2 = (iVar3 + 0x14);
    local_28 = (uVar2 + 0x24);
    ppcVar1 = (local_28 + 4);
    (**ppcVar1)(offset, local_28, (local_28 >> 0x10), 0, &local_24);
    uVar2 = (iVar3 + 0x14);
    local_2e = (uVar2 + 0x44);
    uVar2 = (iVar3 + 0x14);
    local_2c = (uVar2 + 0x40);
    pass1_1008_3e94(
        ((iVar3 + 0x14) + 0x3a),
        CONCAT22(unaff_SS, &local_32),
        CONCAT22(unaff_SS, &local_32 + 2),
    );
    local_36 = local_2c;
    local_38 = 0;
    while (local_38 < local_2e) {
        draw_1020_40ce(local_36, local_32, (local_32 >> 0x10), local_24);
        local_38 = local_38 + 1;
        local_36 = local_36 & 0xffff0000 | (local_36 + 0x18);
    }
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn call_draw_fn_1020_4064(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    draw_1020_3e84(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn polygon_1020_3602(param_1: u16_00, param_2: u16_00, param_3: u32, param_4: HDC16) {
    Polygon16(param_1, (param_1 >> 0x10), param_2);
    return;
}

pub fn cleanup_fn_1020_3898(param_1: *mut astruct_656) {
    destroy_win_1020_3b3e(param_1);
    return;
}

pub fn set_window_pos_1020_38aa(param_1: *mut win_struct_42) {
    let mut hwnd: HWND16;
    let ppcVar1: fn();
    let uVar2: u8;
    let mut uVar3: i32;
    let piVar4: *mut u16;
    let mut uVar5: i32;
    let mut uVar6: u32;
    let mut extraout_var;
    let struct_a: *mut astruct_199;
    let struct_a_00: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let paVar7: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: HWND16;
    let ppVar10: *mut pass1_struct_1;
    let mut uVar11: i32;
    let mut uVar12: i32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar11 = param_1;
    uVar12 = (param_1 >> 0x10);
    create_win_1008_9760(param_1);
    ppVar10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 6));
    uVar9 = (ppVar10 >> 0x10);
    (uVar11 + 0xfa) = ppVar10;
    (uVar11 + 0xfc) = uVar9;
    (uVar11 + 0xe0) = (uVar11 + 0xfa);
    (uVar11 + 0xe2) = uVar9;
    if ((uVar12 | uVar11) == 0) {
        uVar3 = 0;
        uVar5 = 0;
    } else {
        uVar3 = uVar11 + 0xf2;
        uVar5 = uVar12;
    }
    uVar6 = (uVar11 + 0xfa);
    uVar9 = uVar6;
    ppcVar1 = ((uVar11 + 0xfa) + 4);
    (**ppcVar1)(0x1010, uVar9, (uVar6 >> 0x10), 0, uVar3, uVar5);
    pass1_1018_1a8e((uVar11 + 0xfa));
    paVar7 = struct_a;
    process_struct_1000_179c(0x20, struct_a);
    struct_a_00 = (paVar7 | uVar3);
    if (struct_a_00 == 0x0) {
        (uVar11 + 0xf6) = 0;
    } else {
        draw_1020_3da4(uVar3, paVar7, param_1, uVar12);
        (uVar11 + 0xf6) = uVar3;
        (uVar11 + 0xf8) = extraout_DX;
        struct_a_00 = extraout_DX;
    }
    uVar6 = (uVar11 + 0xf6);
    (uVar11 + 0xe8) = uVar6;
    process_struct_1000_179c(0x42, struct_a_00);
    piVar4 = uVar6;
    paVar7 = (struct_a_00 | piVar4);
    if (paVar7 == 0x0) {
        (uVar11 + 0x102) = 0;
    } else {
        win_fn_1008_3bd6(
            piVar4,
            struct_a_00,
            0,
            0xf1,
            0x1ad0000,
            0xf401ac,
            CONCAT22(uVar9, (uVar11 + 8)),
        );
        (uVar11 + 0x102) = piVar4;
        (uVar11 + 0x104) = extraout_DX_00;
        paVar7 = extraout_DX_00;
    }
    uVar8 = 0x1000;
    process_struct_1000_179c(0x42, paVar7);
    if ((paVar7 | piVar4) == 0) {
        (uVar11 + 0x106) = 0;
    } else {
        uVar8 = SUB42(&PTR_LOOP_1050_1008, 0);
        win_fn_1008_3bd6(
            piVar4,
            paVar7,
            0,
            0xf500f1,
            0x1af0000,
            0xf501ae,
            CONCAT22(uVar9, (uVar11 + 8)),
        );
        (uVar11 + 0x106) = piVar4;
        (uVar11 + 0x108) = extraout_DX_01;
    }
    uVar6 = (uVar11 + 0xfa);
    ppcVar1 = ((uVar11 + 0xfa) + 0x10);
    (**ppcVar1)(uVar8, uVar6, (uVar6 >> 0x10));
    paVar7 = piVar4[1];
    let pi_var4_val = unsafe { *piVar4 };
    uVar5 = MoveWindow16(1, piVar4[3], piVar4[2], paVar7, pi_var4_val, (uVar11 + 8));
    uVar9 = 0x1000;
    process_struct_1000_179c(0x8e, paVar7);
    uVar3 = paVar7 | uVar5;
    if (uVar3 == 0) {
        (uVar11 + 0x10a) = 0;
    } else {
        uVar9 = SUB42(&PTR_LOOP_1050_1040, 0);
        uVar2 = process_struct_1040_7728(
            CONCAT22(paVar7, uVar5),
            (&PTR_LOOP_1050_0000 + 1),
            0,
            0xfc0,
            *(uVar11 + 8),
        );
        (uVar11 + 0x10a) = CONCAT31(extraout_var, uVar2);
        (uVar11 + 0x10c) = uVar3;
    }
    uVar6 = (uVar11 + 0x10a);
    (uVar6 + 0x74) = 1;
    uVar6 = (uVar11 + 0x10a);
    ppcVar1 = ((uVar11 + 0x10a) + 8);
    (**ppcVar1)(uVar9, uVar6, (uVar6 >> 0x10));
    if (((uVar11 + 0x10c) | (uVar11 + 0x10a)) != 0) {
        uVar6 = (uVar11 + 0x10a);
        hwnd = (uVar6 + 6);
        GetWindowRect16(CONCAT22(&local_a, uVar9), unaff_SS);
        GetWindowRect16(CONCAT22(&local_12, 0x1538), unaff_SS);
        local_c = local_c - local_10;
        local_e = local_6 - local_a;
        local_12 = local_a;
        local_10 = local_4 + 3;
        SetWindowPos16(0x44, local_c, local_e, local_10, local_a, 0, hwnd);
    }
    return;
}

pub fn destroy_win_1020_3b3e(in_struct_1: *mut astruct_656) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;
    let local_struct_1: *mut astruct_656;
    let mut local_ES_4: u16;
    let mut local_CS__1: u16;
    let mut temp_5f994c26c9: u32;
    let temp_8626ab7427f: *mut u32;

    local_ES_4 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x10e = 0;
    if (local_struct_1.field_0x10a != 0) {
        temp_5f994c26c9 = local_struct_1.field_0x10a;
        local_CS__1 = offset;
        DestroyWindow16((temp_5f994c26c9 + 6));
    }
    puVar1 = local_struct_1.field_0xf6;
    uVar2 = &local_struct_1.field_0xf8;
    if ((uVar2 | puVar1) != 0) {
        unsafe { ppcVar3 = *puVar1 };
        (**ppcVar3)(local_CS__1, puVar1);
    }
    &local_struct_1.field_0xf6 = 0;
    if (local_struct_1.field_0xfa != 0x0) {
        if (in_struct_1 == 0x0) {
            iVar4 = 0;
            uVar5 = 0;
        } else {
            iVar4 = &local_struct_1.field_0xf2;
            uVar5 = local_ES_4;
        }
        pass1_1010_1ea6(local_struct_1.field_0xfa, CONCAT22(uVar5, iVar4));
    }
    local_struct_1.field_0xfa = 0x0;
    return;
}

pub fn call_draw_fn_1020_27b0(param_1: *mut astruct_650, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut uVar6: u16;
    let mut in_stack_0000fff2: u16;
    let mut local_6: u32;

    uVar6 = param_2;
    get_dc_1020_921c(CONCAT22(uVar6, param_1), (param_2 >> 0x10));
    &param_1.field_0x14 = 0;
    CONCAT22(uVar6, param_1) = (s_fem74_wav_1050_2888 + 6);
    param_1.field_0x2 = 0x1020;
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff2, 0x2a));
    uVar4 = (ppVar5 >> 0x10);
    param_1.field_0x14 = ppVar5;
    &param_1.field_0x16 = uVar4;
    param_1.field_0x6 = param_1.field_0x14;
    param_1.field_0x8 = uVar4;
    uVar2 = &param_1.field_0x14;
    iVar3 = &param_1.field_0xa;
    ppcVar1 = ((uVar2 + 10) + 8);
    (**ppcVar1)();
    param_1.field_0x12 = iVar3;
    draw_1020_9364(CONCAT22(uVar6, param_1));
    return;
}

pub fn cleanup_fn_1020_2838(param_1: *mut astruct_44) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.ptr_a_lo = (s_fem74_wav_1050_2888 + 6);
    (iVar1 + 2) = 0x1020;
    if ((iVar1 + 0x14) != 0) {
        pass1_1010_1dda(*(iVar1 + 0x14));
    }
    select_and_delete_palette_1020_92c4(param_1);
    return;
}

pub fn call_cleanup_fn_1020_2868(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376 {
    cleanup_fn_1020_2838(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn polygon_1020_2474(
    param_1: u16,
    param_2: u16,
    count: *mut POINT16,
    device_ctx_handle: HDC16,
) {
    Polygon16(count >> 0x10, device_ctx_handle);
    return;
}

pub fn win_gui_fn_1020_2488(in_struct_1: *mut astruct_647) {
    let mut uVar1: u16;
    let mut uVar2: i32;
    let local_struct_1: *mut astruct_647;
    let local_struct_1_hi: *mut astruct_647;
    let mut uVar3: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_5f808f12c5: *mut astruct_60;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    uVar3 = load_rsrc_1010_4e9e(local_struct_1.struct_ptr_0x6);
    local_a = uVar3;
    if (uVar3 != 0) {
        local_c = 0;
        while (local_c < 9) {
            temp_5f808f12c5 = local_struct_1.struct_ptr_0x6;
            uVar1 = pass1_1010_4f20(temp_5f808f12c5, (temp_5f808f12c5 >> 0x10), local_c);
            set_window_text_1020_1d2a(local_struct_1.field_0xa, local_a, (uVar3 >> 0x10), uVar1);
            uVar2 = get_string_index_1000_3da4((uVar3 & 0xffff0000 | local_a));
            local_a = local_a + uVar2 + 1;
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn win_fn_1020_157c(param_1: u32, param_2: i32) {
    let BVar1: bool;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: HWND16;
    let mut iStack8: u16;
    let mut local_4: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar2 + 0x14) = 0;
        return;
    }
    if (param_2 == 2) {
        BVar1 = IsIconic16((iVar2 + 4));
        if (BVar1 == 0) {
            GetClientRect16(CONCAT22(unaff_SS, &stack0xfff6), (iVar2 + 4));
            local_4 = 0x9a;
            iStack8 = (iVar2 + 4);
            InvalidateRect16(0, &stack0xfff6, unaff_SS);
            return;
        }
    }
    return;
}

pub fn Paint_func_1020_15de(in_struct_1: *mut astruct_14) {
    let pvVar1: *mut void;
    let bool_result_1: bool;
    let mut uVar2: i32;
    let local_struct_1: *mut astruct_14;
    let local_struct_1_hi: *mut astruct_14;
    let mut uvar3: u16;
    let mut h_window: u16;
    let mut uVar4: u32;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut Paint_struct: PAINTSTRUCT;
    let fn_ptr_1: fn();
    let local_struct_2: *mut astruct_417;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    BeginPaint16(CONCAT22(h_window, Paint_struct), local_struct_1.h_wnd);
    bool_result_1 = IsIconic16(local_struct_1.h_wnd);
    if (bool_result_1 == 0) {
        uVar3 = 0x1010;
        uVar4 = pass1_1010_454a(local_struct_1.struct_ptr_0x14);
        uVar2 = (uVar4 >> 0x10);
        if ((uVar2 | uVar4) != 0) {
            local_struct_2 = local_struct_1.struct_ptr_0x14;
            uVar3 = SUB42(&PTR_LOOP_1050_1008, 0);
            pass1_1008_4480(
                local_struct_1.field_0x18,
                (local_struct_2 & 0xffff0000 | (local_struct_2 + 0x76)),
                uVar4 & 0xffff | uVar2 << 0x10,
            );
        }
        pvVar1 = local_struct_1.field_0x18;
        // WARNING: Load size is inaccurate
        fn_ptr_1 = (*local_struct_1.field_0x18 + 4);
        (**fn_ptr_1)(uVar3, pvVar1, (pvVar1 >> 0x10), 0, 0);
    } else {
        draw_icon_1020_1674(in_struct_1);
    }
    EndPaint16(Paint_struct, h_window);
    return;
}

pub fn draw_icon_1020_1674(in_struct_1: *mut astruct_14) {
    let mut icon_handle: HICON16;
    let mut h_brush: HGDIOBJ16;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let local_struct_1: *mut astruct_14;
    let local_struct_1_hi: *mut astruct_14;
    let mut unaff_SS: HANDLE16;
    let uStack34;
    let mut local_1c: u16;
    let mut rect16_2: RECT16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut rect16_1: RECT16;
    let mut local_6: u16;
    let mut local_4: u16;
    // fn_ptr_1: *mut *mut u8;
    let local_struct_2: *mut astruct_417;
    let mut uVar1: u16;

    if (PTR_LOOP_1050_0010 == 0x0) {
        local_struct_1_hi = (in_struct_1 >> 0x10);
        local_struct_1 = in_struct_1;
        fn_ptr_1 = (local_struct_1.struct_ptr_0x14 + 0x2c);
        icon_handle = (**fn_ptr_1)();
        if (icon_handle != 0) {
            h_brush = GetStockObject16(4);
            GetClientRect16(CONCAT22(unaff_SS, &rect16_1), local_struct_1.h_wnd);
            rect16_2.left = 0;
            rect16_2.top = 0;
            iVar2 = (rect16_1.right - rect16_1.left) + -1;
            iVar3 = (rect16_1.bottom - rect16_1.top) + -1;
            local_struct_2 = local_struct_1.struct_ptr_0x14;
            uVar1 = (local_struct_2 + 0x7c);
            rect16_2.right = iVar2;
            rect16_2.bottom = iVar3;
            FillRect16(h_brush, &rect16_2, unaff_SS);
            DrawIcon16(
                icon_handle,
                CONCAT214(
                    rect16_2.left,
                    CONCAT212(uVar1, CONCAT66(uStack34, CONCAT24(uVar1, 0x20002))),
                ),
                CONCAT214(
                    rect16_1.right,
                    CONCAT212(
                        rect16_1.top,
                        CONCAT210(
                            rect16_1.left,
                            CONCAT28(
                                iVar2,
                                CONCAT26(
                                    iVar3,
                                    CONCAT24(
                                        rect16_2.bottom,
                                        CONCAT22(rect16_2.right, rect16_2.top),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                rect16_1.bottom,
            );
        }
    }
    return;
}

pub fn call_palette_fn_1020_170a(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376 {
    palette_func_1020_150e(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn process_struct_1020_1738(in_struct_1: *mut astruct_68, param_2: u16, param_2_00: u32) {
    let local_struct_1: *mut astruct_68;
    let local_struct_1_hi: *mut astruct_68;

    process_struct_1040_7728(
        in_struct_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0xfcd,
        *(param_2_00 + 8),
    );
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x8e = 0;
    local_struct_1.field_0x92 = 0;
    local_struct_1.field_0x96 = 0;
    in_struct_1.field_0x0 = (s_512_bmp_1050_1e77 + 3);
    local_struct_1.field_0x2 = 0x1020;
    return;
}

pub fn call_destroy_win_fn_1020_1780(param_1: *mut astruct_53) {
    let fn_ptr_1: fn();

    fn_ptr_1 = (param_1 + 0x6c);
    (**fn_ptr_1)();
    destroy_window_1040_8212(param_1);
    return;
}

pub fn win_fn_1020_179c(param_1: *mut astruct_20) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: i32;
    let mut iVar5: u16;
    let puVar6: *mut u16;
    let BVar7: bool;
    let mut hwnd: HWND16;
    let struct_a: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut unaff_SS: HWND16;
    let ppVar13: *mut pass1_struct_1;
    let puVar14: *mut u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let uVar18: u8;
    let mut uVar19: u16;
    let mut in_stack_0000fe88: u16;
    let mut local_76: u32;
    let mut local_72: u32;
    let mut local_6e: u32;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_62: u16;
    let mut local_5e: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    uVar19 = 0x89;
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x890009);
    iVar8 = _local_6;
    pass1_1010_65d0(_local_6, uVar19);
    uVar4 = (iVar8 == 0);
    local_8 = uVar4;
    pass1_1010_65d0(_local_6, 0x86);
    if (uVar4 != 0) {
        local_8 = 0;
    }
    ppVar13 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fe88, 0x39));
    uVar12 = (ppVar13 >> 0x10);
    uVar4 = ppVar13;
    uVar10 = (param_1 >> 0x10);
    iVar8 = param_1;
    (iVar8 + 0x8e) = uVar4;
    (iVar8 + 0x90) = uVar12;
    uVar11 = (iVar8 + 0x8e);
    ppcVar2 = ((iVar8 + 0x8e) + 0x10);
    ppcVar2(0x1010, uVar11, uVar12, local_8);
    local_4c = struct_a;
    process_struct_1000_179c(0x12, struct_a);
    local_4e = uVar4;
    if ((local_4c | uVar4) == 0) {
        (iVar8 + 0x92) = 0;
    } else {
        process_struct_1020_1eea(CONCAT22(local_4c, uVar4), param_1, (iVar8 + 6));
        (iVar8 + 0x92) = uVar4;
        (iVar8 + 0x94) = extraout_DX;
    }
    uVar1 = (iVar8 + 0x8e);
    local_c = uVar1 & 0xffff0000 | (uVar1 + 10);
    _local_10 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar11, 0x48));
    uVar15 = unaff_SS;
    uVar17 = (unaff_SS >> 8);
    GetClientRect16(CONCAT13(uVar17, CONCAT12(uVar15, &local_1c)), (iVar8 + 6));
    iVar5 = GetSystemMetrics16(4);
    uVar11 = (local_c >> 0x10);
    iVar9 = local_c;
    (iVar9 + 6) = iVar5 + local_18._2_2_;
    uVar12 = (_local_10 >> 0x10);
    local_12 = (_local_10 + 10);
    local_14 = (_local_10 + 0xc);
    (iVar9 + 2) = (local_14 - (iVar9 + 6)) / 2;
    local_c = (local_12 - (iVar9 + 4)) / 2;
    pass1_1028_dc52(
        CONCAT13(uVar17, CONCAT12(uVar15, &local_2e)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x100,
    );
    local_38 = 0;
    while (true) {
        puVar6 = &local_2e;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar6));
        _local_32 = CONCAT22(extraout_DX_00, puVar6);
        if ((extraout_DX_00 | puVar6) == 0) {
            break;
        }
        local_36 = (puVar6 + 8);
        if (local_36 != 0x0) {
            process_string_1000_3cea((param_1 & 0xffff0000 | (iVar8 + 0x10)), *local_36);
        }
    }
    BVar7 = pass1_1020_1da8(param_1);
    (iVar8 + 0x96) = BVar7;
    pass1_1010_65d0(_local_6, 0x86);
    if ((BVar7 == 0) || ((iVar8 + 0x96) != 0)) {
        uVar3 = (iVar8 + 0x8e);
        (uVar3 + 0x2c) = 0;
        local_66 = GetDlgItem16(0x175, (iVar8 + 6));
        if (local_8 != 0) {
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x100,
                CONCAT22(unaff_SS, &stack0xfe88),
                0x72d,
            );
            SetWindowText16(CONCAT22(unaff_SS, &stack0xfe88), local_66);
        }
        _local_3c = MakeProcInstance16(offset, CONCAT22(0x1e1e, g_h_instance_1050_038c));
        GetWindowRect16(
            CONCAT13((&local_6e >> 8), CONCAT12(&local_6e, 0x1538)),
            unaff_SS,
        );
        local_72 = local_6a;
        local_62 = local_6a - local_6e;
        local_5e = local_6a._2_2_ - local_6e._2_2_;
        local_76 = local_6e & 0xffff0000 | (-(local_62 - (local_c + 4)) / 2);
        iVar5 = GetSystemMetrics16(4);
        uVar1 = local_76 & 0xffff;
        local_76 = uVar1 | (local_76._2_2_ - iVar5) << 0x10;
        local_76._0_2_ = uVar1;
        MoveWindow16(
            0,
            local_5e,
            local_62,
            local_76._2_2_ - iVar5,
            local_76,
            local_66,
        );
    } else {
        mci_send_command_1008_5c7c(_g_struct_ptr_1050_02a0, 0x9d0001);
        (iVar8 + 0x8c) = BVar7;
        _local_3c = MakeProcInstance16(
            &PTR_LOOP_1050_1008,
            CONCAT22(0x1dea, g_h_instance_1050_038c),
        );
    }
    EnumChildWindows16(0, _local_3c, (_local_3c >> 0x10));
    FreeProcInstance16(CONCAT13((_local_3c >> 8), CONCAT12(_local_3c, 0x1538)));
    local_46 = GetDlgItem16(1, (iVar8 + 6));
    GetWindowRect16(CONCAT22(&local_1c, 0x1538), unaff_SS);
    local_40 = local_18;
    iVar9 = local_18 - local_1c;
    _local_4a = CONCAT22(iVar9, local_18._2_2_ - local_1c._2_2_);
    local_44 = local_1c & 0xffff0000 | (-(iVar9 - (local_c + 4)) / 2);
    iVar5 = GetSystemMetrics16(4);
    local_44 = local_44 & 0xffff | (local_44._2_2_ - iVar5) << 0x10;
    if ((iVar8 + 0x96) == 0) {
        if (local_8 == 0) {}
        // goto LAB_1020_1b24;
        unaff_SS = 0x72e;
        puVar14 = &stack0xfe88;
        iVar5 = 0x100;
        uVar16 = uVar15;
        uVar18 = uVar17;
    } else {
        load_string_1010_84e0(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT13(uVar17, CONCAT12(uVar15, &stack0xfe88)),
            0x72f,
        );
        hwnd = GetDlgItem16(0x175, (iVar8 + 6));
        iVar5 = offset;
        SetWindowText16(CONCAT13(uVar17, CONCAT12(uVar15, &stack0xfe88)), hwnd);
        puVar14 = 0x3ff;
        uVar16 = &stack0xfe88;
        uVar18 = (&stack0xfe88 >> 8);
    }
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        iVar5,
        CONCAT13(uVar18, CONCAT12(uVar16, puVar14)),
        unaff_SS,
    );
    SetWindowText16(CONCAT13(uVar17, CONCAT12(uVar15, &stack0xfe88)), local_46);
    // LAB_1020_1b24:
    MoveWindow16(
        0,
        _local_4a,
        (_local_4a >> 0x10),
        local_44._2_2_,
        local_44,
        local_46,
    );
    uVar11 = (local_c >> 0x10);
    iVar9 = local_c;
    SetWindowPos16(
        0x44,
        (iVar9 + 6),
        (iVar9 + 4),
        (iVar9 + 2),
        local_c,
        0,
        (iVar8 + 6),
    );
    return;
}

pub fn pass1_1020_1b68(param_1: *mut astruct_409) {
    let puVar1: *mut u32;
    let pvVar2: *mut void;
    let local_struct_1: *mut astruct_641;
    let local_struct_1_hi: *mut astruct_409;
    let fn_ptr_1: fn();

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    puVar1 = local_struct_1.field_0x92;
    pvVar2 = local_struct_1.field_0x94;
    if ((pvVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
            (**fn_ptr_1)();
        }
    }
    &local_struct_1.field_0x92 = 0;
    pass1_1010_4f48(local_struct_1.field_0x8e);
    local_struct_1.field_0x8e = 0;
    return;
}

pub fn enable_window_1020_1bd4(param_1: i32, param_2: u16, param_3: u16_00, param_4: u16) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut b_var3: bool;
    let mut h_wnd: HWND16;
    let BVar4: bool;
    let mut uVar5: u16;
    let mut uVar6: i32;
    let mut uVar7: u16;
    let puVar8: *mut u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    b_var3 = false;
    puVar8 = call_pt_in_rect_fn_1020_1d8e(
        CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
        CONCAT22(param_3, param_3_00),
    );
    if (puVar8 != 0) {
        if (puVar8 < 2) {
            b_var3 = true;
        } else {
            h_wnd = GetDlgItem16(1, (param_1 + 6));
            uVar1 = (param_1 + 0x8e);
            pass1_1010_4e8c(uVar1, (uVar1 >> 0x10));
            EnableWindow16(1, h_wnd);
            puVar8 = pass1_1010_4df0((param_1 + 0x8e));
            uVar7 = (puVar8 >> 0x10);
            if ((puVar8 == 0) && (b_var3 = true, (param_1 + 0x96) == 0)) {
                BVar4 = EnableWindow16(0, h_wnd);
                puVar8 = CONCAT22(uVar7, BVar4);
            }
        }
    }
    if (b_var3) {
        uVar7 = 0x1000;
        process_struct_1000_179c(0xb4, (puVar8 >> 0x10));
        uVar6 = (puVar8 >> 0x10) | puVar8;
        if (puVar8 == 0x0) {
            uVar5 = 0;
            uVar6 = 0;
        } else {
            uVar7 = SUB42(&PTR_LOOP_1050_1040, 0);
            uVar5 = mixed_1040_8520(puVar8, (param_1 + 6), 0x30, 2, 0x57b, 0x62a);
        }
        _local_c = CONCAT22(uVar6, uVar5);
        ppcVar2 = (*_local_c + 0x74);
        ppcVar2(uVar7, uVar5, uVar6);
    }
    return;
}

pub fn set_window_text_1020_1d2a(param_1: u32, param_2: SEGPTR, param_3: u16) {
    let mut hwnd: HWND16;
    hwnd = GetDlgItem16(param_3, (param_1 + 6));
    SetWindowText16(param_2, hwnd);
    return;
}

pub fn win_func_1020_1d4a(param_1: u32, param_2: i32) {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    if (param_2 != 0) {
        iVar2 = param_1;
        uVar3 = (param_1 >> 0x10);
        iVar1 = post_win_msg_1020_1ca4(iVar2, uVar3);
        if (iVar1 != 0) {
            if ((iVar2 + 0x96) != 0) {
                PostMessage16(0, 0xee, 0x111, g_h_window);
            }
            DestroyWindow16((iVar2 + 6));
        }
    }
    return;
}

pub fn call_pt_in_rect_fn_1020_1d8e(param_1: *mut void, param_2: u32) {
    pt_in_rect_1010_4e08((param_1 + 0x8e), param_2, (param_2 >> 0x10));
    return;
}

pub fn pass1_1020_1da8(in_struct_1: *mut astruct_643) -> bool {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let local_struct_1: *mut astruct_643;
    let local_struct_1_hi: *mut astruct_643;
    let mut temp_5fb951b2a7: u32;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    temp_5fb951b2a7 = local_struct_1.field_0x8e;
    if ((temp_5fb951b2a7 + 0x30) == 1) {
        return 1;
    }
    uVar1 = local_struct_1.field_0x8e;
    if (((uVar1 + 0x30) < 3)
        && (
            iVar2 = pass1_1010_4df0(local_struct_1.field_0x8e),
            iVar2 == 0,
        ))
    {
        return 1;
    }
    return 0;
}

pub fn destroy_win_1020_1dea(param_1: u16_00, param_2: u16, param_3: HWND16) -> u16 {
    let BVar1: bool;
    let wVar2: u16;

    BVar1 = IsWindow16(param_1);
    if (BVar1 != 0) {
        wVar2 = GetWindowWord16(-0xc, param_1);
        if (wVar2 == 0x175) {
            DestroyWindow16(param_1);
            return 0;
        }
    }
    return 1;
}

pub fn destroy_win_1020_1e1e(param_1: u16_00, param_2: u16, 2: HWND16) -> u16 {
    let BVar1: bool;
    let wVar2: u16;
    let mut local_4: u16;

    BVar1 = IsWindow16(param_1);
    if (BVar1 != 0) {
        wVar2 = GetWindowWord16(-0xc, param_1);
        if ((wVar2 != 1) && (wVar2 != 0x175)) {
            DestroyWindow16(param_1);
        }
    }
    return 1;
}

pub fn call_cleanup_fn_1020_1e54(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    win_cleanup_func_1040_782c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_1eea(in_struct_1: *mut astruct_644, param_2: u32, param_3: u16) {
    let ppcVar1: fn();
    let mut local_AX_92: u16;
    let mut local_DX_92: u16;
    let local_BX_4: *mut astruct_644;
    let mut local_ES_4: u16;
    let ppVar2: *mut pass1_struct_1;
    let string_1: *mut char;

    local_ES_4 = (in_struct_1 >> 0x10);
    local_BX_4 = in_struct_1;
    in_struct_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    in_struct_1 = (s_18_2_1050_3aa5 + 3);
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = param_3;
    in_struct_1 = s_0_020_1050_3ab0;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    &local_BX_4.field_0x6 = 0;
    local_BX_4.field_0xa = param_2;
    in_struct_1 = (s_218_bmp_1050_2516 + 2);
    local_BX_4.field_0x2 = 0x1020;
    string_1 = CONCAT22(string_1._2_2_, 0x39);
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, string_1);
    local_DX_92 = (ppVar2 >> 0x10);
    local_BX_4.field_0x6 = ppVar2;
    &local_BX_4.field_0x8 = local_DX_92;
    ppcVar1 = (&local_BX_4.field_0x6 + 4);
    (**ppcVar1)(
        0x1010,
        local_BX_4.field_0x6,
        local_DX_92,
        0,
        in_struct_1,
        string_1._2_2_,
    );
    return;
}

pub fn pass1_1020_1f74(param_1: *mut astruct_376) {
    let local_struct_1: *mut astruct_376;
    let local_struct_1_hi: *mut astruct_376;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = (s_218_bmp_1050_2516 + 2);
    local_struct_1.ptr_a_hi = 0x1020;
    pass1_1010_1ea6(
        *&local_struct_1.u16_x6,
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
    );
    param_1.ptr_a_lo = s_0_020_1050_3ab0;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn invalidate_window_rect_1020_1fb2(param_1: u32, param_2: i32) {
    let mut unaff_CS: u16;
    let mut unaff_SS: HWND16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 1) {
        (param_1 + 6) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    GetWindowRect16(CONCAT22(&local_e, unaff_CS), unaff_SS);
    local_16 = 0;
    local_6 = 0x46;
    local_14 = 0x46;
    local_12 = local_a - local_e;
    local_4 = 0x5f;
    local_10 = 0x5f;
    InvalidateRect16(0, &local_16, unaff_SS);
    return;
}

pub fn draw_1020_2020(param_1: *mut astruct_647) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut HVar4: HDC16;
    let puVar5: *mut u16;
    let mut HVar6: HPEN16;
    let mut obj_handle: HGDIOBJ16;
    let mut HVar7: HBRUSH16;
    let mut obj_handle_00: HGDIOBJ16;
    let mut HVar8: HGDIOBJ16;
    let mut h_gdi_obj: HPALETTE16;
    let mut extraout_DX: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut unaff_SS: HWND16;
    let mut uVar13: u32;
    let mut uVar14: u32;
    let mut uVar15: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: [u8; 6];
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar11 = (param_1 >> 0x10);
    uVar10 = param_1;
    HVar4 = BeginPaint16(CONCAT22(unaff_SS, &local_22), (uVar10 + 4));
    local_24 = HVar4;
    pass1_1010_4c2c((uVar10 + 6));
    _local_28 = CONCAT22(extraout_DX, HVar4);
    puVar5 = &local_24;
    ppcVar1 = (*_local_28 + 8);
    (**ppcVar1)(0x1010, HVar4, extraout_DX, puVar5, unaff_SS);
    (uVar10 + 0x10) = puVar5;
    uVar2 = (uVar10 + 6);
    local_2a = (uVar2 + 0x30);
    uVar2 = (uVar10 + 6);
    local_2e = (uVar2 + 0x12);
    _local_32 = 0x140000;
    uVar12 = SUB42(&PTR_LOOP_1050_1008, 0);
    zero_list_1008_3e38(CONCAT22(unaff_SS, local_38));
    local_3a = 0;
    while (local_3a < local_2a) {
        uVar13 = process_struct_1008_4772((local_3a * 4 + local_2e));
        uVar9 = (uVar13 >> 0x10);
        iVar3 = uVar13;
        pass1_1020_2286(
            uVar10,
            uVar11,
            CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_32)),
            (iVar3 + 8),
        );
        pass1_1008_3e76(
            CONCAT22(unaff_SS, local_38),
            0,
            _local_32,
            (_local_32 >> 0x10),
        );
        pass1_1008_4480(
            _local_28,
            CONCAT22(unaff_SS, local_38),
            (local_2e + local_3a * 4),
        );
        uVar12 = 0x1010;
        process_struct_1010_4d5c(
            (uVar10 + 6),
            (iVar3 + 8) + _local_32,
            (iVar3 + 4) + local_30,
            _local_32,
            local_30,
            local_3a,
        );
        _local_32 =
            _local_32 & 0xffff | (local_30 + (-(local_3a == 0) & 5) + 0x14 + (iVar3 + 4)) << 0x10;
        local_3a = local_3a + 1;
    }
    ppcVar1 = (*_local_28 + 4);
    (**ppcVar1)(uVar12, _local_28, (_local_28 >> 0x10), 0, 0, 0xdc);
    HVar6 = CreatePen16(0x1000025, 1, 0);
    obj_handle = SelectObject16(HVar6, local_24);
    HVar7 = CreateSolidBrush16(0x1000025);
    obj_handle_00 = SelectObject16(HVar7, local_24);
    HVar8 = obj_handle_00;
    draw_1020_229c(uVar10, (param_1 >> 0x10), local_24);
    pass1_1010_4df0((uVar10 + 6));
    if (HVar8 == 0) {
        HVar8 = SelectObject16(obj_handle, local_24);
        DeleteObject16(HVar8);
        HVar8 = SelectObject16(obj_handle_00, local_24);
        DeleteObject16(HVar8);
        HVar7 = CreateSolidBrush16(0xff);
        HVar6 = CreatePen16(0xff, 1, 0);
        SelectObject16(HVar7, local_24);
        SelectObject16(HVar6, local_24);
    }
    uVar15 = local_24;
    uVar14 = pass1_1010_4dc8((uVar10 + 6));
    draw_fn_1020_239c(param_1, uVar14, uVar15);
    uVar2 = (uVar10 + 6);
    if ((uVar2 + 0x2c) != 0) {
        win_gui_fn_1020_2488(param_1);
    }
    h_gdi_obj = SelectPalette16(0, (uVar10 + 0x10), local_24);
    DeleteObject16(h_gdi_obj);
    HVar8 = SelectObject16(obj_handle, local_24);
    DeleteObject16(HVar8);
    HVar8 = SelectObject16(obj_handle_00, local_24);
    DeleteObject16(HVar8);
    EndPaint16(&local_22, unaff_SS);
    return;
}

pub fn pass1_1020_2286(param_1: u16, param_2: u16, param_1_00: *mut i32, param_2_00: i32) {
    unsafe {
        *param_1_00 = 100 - param_2_00 >> 1;
    }
    return;
}

pub fn draw_1020_229c(param_1: u32, param_2: HDC16) {
    let mut iVar1: i32;
    let piVar2: *mut u16;
    let mut uVar3: u32;
    let mut x: i32;
    let mut iVar4: i32;
    let piVar5: *mut i32;
    let mut uVar6: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    uVar3 = (param_1 + 6);
    iVar1 = (uVar3 + 0x30);
    uVar3 = (param_1 + 6);
    piVar2 = (uVar3 + 0x1a);
    let pi_var2_val = unsafe { *piVar2 };
    MoveTo16(5, pi_var2_val, param_2);
    uVar6 = (piVar2 >> 0x10);
    iVar4 = piVar2;
    LineTo16(5, (iVar4 + iVar1 * 8 + -4), param_2);
    local_a = 0;
    while (local_a < iVar1) {
        piVar5 = (local_a * 8 + iVar4);
        let pi_var_5_val = unsafe { *piVar5 };
        x = (piVar5[2] - pi_var_5_val >> 1) + pi_var_5_val;
        MoveTo16(5, x, param_2);
        LineTo16(10, x, param_2);
        local_a = local_a + 1;
    }
    let pi_var_2_val = unsafe { *piVar2 };
    MoveTo16(0x5f, pi_var_2_val, param_2);
    LineTo16(0x5f, (iVar4 + iVar1 * 8 + -4), param_2);
    local_a = 0;
    while (local_a < iVar1) {
        piVar5 = (local_a * 8 + iVar4);
        let pi_var5_val = unsafe { *piVar5 };
        MoveTo16(0x5f, (piVar5[2] - pi_var5_val >> 1) + pi_var5_val, param_2);
        LineTo16(0x5a, param_2, param_2);
        local_a = local_a + 1;
    }
    return;
}

pub fn draw_fn_1020_239c(param_1: u32, param_2: u32, param_3: u16) {
    let mut local_SS__1: u16;
    let mut local_DXAX_60: u32;
    let mut uVar1: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 6];
    let mut local_4: u16;

    if (param_2 != 0) {
        pass1_1008_3e54(
            CONCAT22(local_SS__1, local_a),
            0,
            0x57,
            ((param_2 + 4) - param_2 >> 1) + param_2,
        );
        uVar1 = (param_1 >> 0x10);
        local_DXAX_60 = pass1_1020_23f2(param_1, uVar1, CONCAT22(local_SS__1, local_a));
        polygon_1020_2474(
            param_1,
            uVar1,
            CONCAT22(local_DXAX_60, 3),
            (local_DXAX_60 >> 0x10),
        );
    }
    return;
}

pub fn pass1_1020_23f2(param_1: u16, param_2: u16, param_1_00: *mut u16) {
    let mut uVar1: u16;
    let local_struct_1: *mut astruct_645;
    let mut unaff_SS: u16;
    let mut uVar2: u32;
    let local_12: *mut astruct_646;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1008_3e94(
        param_1_00,
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    process_struct_1000_179c(0xc, (uVar2 >> 0x10));
    uVar1 = (uVar2 >> 0x10);
    local_12 = 0x0;
    while (local_12 < 3) {
        local_struct_1 = (local_12 * 4);
        (local_struct_1 + uVar2) = (local_struct_1 + 0x4270) + local_4;
        (local_struct_1 + uVar2 + 2) = (local_struct_1 + 0x4272) + local_6;
        local_12 = &local_12.field_0x1;
    }
    return uVar2;
}
