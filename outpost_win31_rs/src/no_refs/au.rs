use crate::block_1040::block_1040_5000::{pass1_1040_5cd6, pass1_1040_5d42, pass1_1040_5dc4, pass1_1040_5eaa};
use crate::block_1040::block_1040_6000::{create_window_1040_6eae, msg_box_ui_op_1040_64ca, pass1_1040_6470};

pub unsafe fn win_ui_op_1040_52c0(param_1: *mut u8, param_2: *mut astruct_894, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut is_window: bool;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut uVar9: u32;
    let mut puVar10: *mut u32;
    let mut paVar11: *mut astruct_940;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb4: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut in_stack_0000ffde: u16;
    let mut uVar15: u16;
    let mut uStack30: u16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut hwnd_8: HWND16;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 != 0x10c) {
        if (param_5 < 0x10d) {
            if (param_5 == 0xfa) {
                ppcVar1 = (*param_2.field148_0x98 + 0x18);
                (**ppcVar1)();
                return;
            }
            if (param_5 == 0x10a) {
                GetClientRect16(&local_a, &DAT_1050_1050);
                puVar2 = param_2.field148_0x98;
                local_a.y += 0x3;
                local_a.x = (puVar2 + 0x1a) - 0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
                unk_destroy_win_op_1010_2fa0(param_2.field148_0x98);
                pass1_1010_32c0(param_2.field148_0x98, 0x0);
                pass1_1010_2ee2(param_2.field148_0x98);
                return;
            }
            if (param_5 != 0x10b) {//
// LAB_1040_5560:
                pass1_1040_b54a(param_1, CONCAT22(param_3, param_2), param_4, param_5);
                return;
            }
            puVar2 = param_2.field148_0x98;
            uVar12 = (puVar2 + 0x12);
            uVar5 = uVar12;
            puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(uVar12, 0x3), in_stack_0000fe84,
                                      in_stack_0000ffa8, in_stack_0000ffae, in_stack_0000ffb2);
            uVar7 = (puVar10 >> 0x10);
            uStack30 = puVar10;
            uVar4 = uStack30;
            uVar6 = uVar7;
            pass1_1010_a5ca(uStack30, uVar7, uStack30, uVar7, uVar12);
            if ((uVar5 != 0x70) && (uVar4 == 0)) {
                return;
            }
            uVar9 = param_2.field169_0xb0;
            uVar13 = uVar9;
            uVar14 = (uVar9 >> 0x10);
            puVar2 = param_2.field148_0x98;
            uVar12 = (puVar2 + 0x12);
        } else {
            if (param_5 != 0x10d) {
                if (param_5 == 0x10e) {
                    puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x32),
                                              in_stack_0000fe86, in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                    uVar9 = paVar8 & 0xffff0000 | puVar10 >> 0x10;
                    uVar3 = puVar10;
                    uVar15 = uVar3;
                    ui_op_1010_79aa(puVar10, 0xfc6, param_2.field169_0xb0);
                    if (uVar3 != 0) {
                        return;
                    }
                    unk_win_op_1010_7300(uVar9, (puVar10 & 0xffff0000 | uVar15), 0x0, 0x13, param_2.field169_0xb0);
                    return;
                }
                if (param_5 != 0xbbb) {
                    if (param_5 == 0xbbc) {
                        puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x3),
                                                  in_stack_0000fe86, in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                        uVar12 = (puVar10 >> 0x10);
                        uVar4 = puVar10;
                        uVar7 = uVar12;
                        uVar5 = pass1_1010_a5ac(uVar4, uVar12, param_2.field169_0xb0);
                        uVar6 = uVar5;
                        pass1_1010_a58a(uVar5, uVar7, uVar4, uVar12, uVar5);
                        if (uVar6 == 0) {
                            pass1_1010_a568(0x0, uVar7, uVar4, uVar12, uVar5);
                        }
                        hwnd_8 = GetDlgItem16(0xbbc, param_2.hwnd_0x6);
                        EnableWindow16(0x0, hwnd_8);
                        return;
                    }
                    // TODO: goto LAB_1040_5560;
                }
                if ((param_2.field171_0xb6 == 0) || (is_window = IsWindow16(param_2.field171_0xb6), is_window == 0)) {
                    paVar11 = pass1_1038_af40(param_2, paVar8, _PTR_LOOP_1050_5b7c, param_2.hwnd_0x6, 0x1b);
                    param_2.field171_0xb6 = (paVar11 + 0x6);
                    set_win_pos_1038_abdc(paVar11);
                    ShowWindow16(0x1, param_2.field171_0xb6);
                    return;
                }
                hwnd_8 = param_2.field171_0xb6;
                // TODO: goto LAB_1040_5417;
            }
            puVar10 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(in_stack_0000ffde, 0x3), in_stack_0000fe86,
                                      in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
            uVar6 = (puVar10 >> 0x10);
            uStack30 = puVar10;
            uVar9 = param_2.field169_0xb0;
            uVar13 = uVar9;
            uVar14 = (uVar9 >> 0x10);
            uVar12 = 0x71;
            uVar7 = uVar6;
        }
        pass1_1010_a5ec(uVar6, uStack30, uVar7, uVar12, CONCAT22(uVar14, uVar13));
        if ((param_2.hwnd_0xb4 != 0) && (is_window = IsWindow16(param_2.hwnd_0xb4), is_window != 0)) {
            SendMessage16(0x0, 0xeb, 0x111, param_2.hwnd_0xb4);
        }
    }
    hwnd_8 = param_2.hwnd_0x6;//
// LAB_1040_5417:
    DestroyWindow16(hwnd_8);
    return;
}


pub unsafe fn pass1_1040_557c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_4f0a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn enable_win_1040_5780(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut hwnd: HWND16;
    let mut in_EDX: *mut Struct57;
    let mut iVar4: *mut astruct_945;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut paVar6: *mut astruct_945;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    ppcVar1 = (*param_1 + 0x74);
    paVar6 = iVar4;
    (**ppcVar1)();
    puVar5 = mixed_1010_20ba(in_EDX, _u16_1050_0ed0, CONCAT22(paVar6, 0x3), in_stack_0000fe98, in_stack_0000ffbc,
                             in_stack_0000ffc2, in_stack_0000ffc6);
    uVar2 = iVar4.field143_0x90;
    uVar3 = pass1_1010_acc0(puVar5, (puVar5 >> 0x10), (uVar2 + 0x6));
    if (uVar3 != 0) {
        hwnd = GetDlgItem16(0x1790, iVar4.field6_0x6);
        EnableWindow16(0x1, hwnd);
    }
    return;
}


pub unsafe fn pass1_1040_57d4(param_1: *mut u8, param_2: *mut StructB)

{
    pass1_1040_5d42(param_2);
    pass1_1040_5eaa(param_2);
    pass1_1040_5dc4(param_1, param_2);
    unk_win_ui_op_1040_b230(param_1, param_2);
    return;
}


pub unsafe fn win_ui_op_1040_5800(param_1: *mut u8, param_2: *mut astruct_18, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut paVar5: *mut astruct_18;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut hwnd: HWND16;
    let mut uVar6: u16;
    let mut puVar7: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut paVar10: *mut Struct57;
    let mut iVar11: i16;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut piStack24: *mut i16;
    let mut local_14: [u8; 0x8] = [0; 0x8];
    let mut iStack12: i16;
    let mut pSStack10: *mut StructD;
    let mut paStack6: *mut astruct_20;
    let mut pSVar5: *mut StructD;
    let mut paVar9: *mut Struct57;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 == 0xeb) {
        paStack6 = mixed_1010_20ba(paVar8, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x3), in_stack_0000fe80,
                                   in_stack_0000ffa4, in_stack_0000ffaa, in_stack_0000ffae);
        paVar8 = (paVar8 & 0xffff0000 | paStack6 >> 0x10);
        pSVar5 = &param_2.field138_0x90;
        if (pSVar5.is_null() == false) {
            pSStack10 = pSVar5;
            // 0x0018
            mem_op_1000_179c(0x18, paVar8);
            uVar3 = pSVar5;
            uVar6 = paVar8 | uVar3;
            paVar10 = (paVar8 & 0xffff0000);
            paVar9 = (paVar10 | uVar6);
            if (uVar6 == 0) {
                uVar3 = 0;
            } else {
                struct_1040_a598((pSVar5 & 0xffff | paVar8 << 0x10));
                paVar10 = paVar9;
            }
            param_2.field138_0x90 = uVar3;
            param_2.field139_0x92 = paVar10;
            *&param_2.field138_0x90 = 0x6;
            iStack12 = *&param_2.field138_0x90;
            uVar3 = iStack12 * 0xa + 2;
            mem_op_1000_179c(uVar3, paVar10);
            uVar6 = paVar10;
            piStack24 = CONCAT22(uVar6, uVar3);
            puVar7 = (uVar6 | uVar3);
            if (puVar7.is_null()) {
                uVar2 = &param_2.field138_0x90;
                (uVar2 + 0x2) = 0;
            } else {
                *piStack24 = iStack12;
                // &PTR_LOOP_1050_1040 actually 0x1040
                pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iStack12, 0xa, uVar3 + 0x2, uVar6);
                uVar2 = &param_2.field138_0x90;
                uVar12 = (uVar2 >> 0x10);
                iVar11 = uVar2;
                (iVar11 + 0x2) = uVar3 + 2;
                (iVar11 + 0x4) = uVar6;
            }
            uVar2 = &param_2.field138_0x90;
            (uVar2 + 0x6) = (pSStack10 + 0x6);
            uVar2 = &param_2.field138_0x90;
            (uVar2 + 0xa) = 0x4;
            uVar2 = &param_2.field138_0x90;
            (uVar2 + 0x12) = &param_2.field_0xa;
            uVar12 = 0x1010;
            pass1_1010_a50c(paStack6, &u32_1050_5d78, &param_2.field138_0x90);
            if (pSStack10.is_null() == false) {
                pass1_1040_a5d0(pSStack10);
                uVar12 = 0x1000;
                fn_ptr_1000_17ce(pSStack10);
            }
            ppcVar1 = (CONCAT22(param_3, param_2) + 0x70);
            (**ppcVar1)(uVar12, param_2, param_3);
            uVar4 = pass1_1040_5cd6(CONCAT22(param_3, param_2));
            if (uVar4 != 0) {
                pass1_1040_5eaa(CONCAT22(param_3, param_2));
                param_2.field_0x94 = 0;
            }
            pass1_1040_5dc4(puVar7, CONCAT22(param_3, param_2));
            GetWindowRect16(CONCAT13(0x10, CONCAT12(0x50, local_14)), param_2.hwnd_0x6);
            InvalidateRect16(param_2[0x1].base_0x0, NULL, 0x0);
            if (param_2[0x1].base_0x0 != 0) {
                param_2[0x1].base_0x0 = 0;
            }
        }
    } else {
        if (param_5 != 0x13b) {
            pass1_1040_b54a(param_1, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), param_4,
                            param_5);
            return;
        }
        hwnd = GetDlgItem16(0x1790, param_2.hwnd_0x6);
        EnableWindow16(0x1, hwnd);
    }
    return;
}


pub unsafe fn draw_op_1040_5a06(mut param_1: u32, struct741_param_1: *mut astruct_741)

{
    let mut uVar1: u16;
    let mut caption_height_px: i16;
    let mut IVar2: i16;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut IVar3: i16;
    let mut y: i16;
    let mut IVar4: i16;
    let mut y_00: i16;
    let mut x: i16;
    let mut IVar5: i16;
    let mut in_DX: u16;
    let mut uVar6: u16;
    let mut palette_handle_7: HPALETTE16;
    let mut puVar2: *mut u32;
    let mut uVar8: u32;
    let mut struct_1: *mut astruct_741;
    let mut uVar9: u16;
    let mut base_addr: u16;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let mut uVar10: u8;
    let mut uVar14: u16;
    let mut uStack82: u16;
    let mut iStack72: i16;
    let mut iStack68: i16;
    let mut puStack54: *mut u32;
    let mut hdc16_2c: HDC16;
    let mut paint_struct_2a: [u8; 0x20] = [0; 0x20];
    let mut rect_array_local_a: [u8; 0x8] = [0; 0x8];
    let mut uVar13: u16;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut uVar4: u32;
    let mut puVar1: *mut u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar7: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar9 = (struct741_param_1 >> 0x10);
    struct_1 = struct741_param_1;
    GetWindowRect16(CONCAT13(0x10, CONCAT12(0x50, rect_array_local_a)), struct_1.field6_0x6);
    hdc16_2c = BeginPaint16(CONCAT13(0x10, CONCAT12(0x50, paint_struct_2a)), struct_1.field6_0x6);
    base_addr = 0x1008;
    palette_handle_7 = palette_op_1008_4e08(&hdc16_2c, param_1, (_PTR_LOOP_1050_4230 + 0xe),
                                            CONCAT13(0x10, CONCAT12(0x50, &hdc16_2c)));
    puVar2 = null_mut();
    puStack54 = null_mut();
    uVar7 = param_1;
    if (struct_1.field149_0x98 != 0) {
        uVar1 = FUN_1010_830a(0x0, param_1, 0x1008, _u16_1050_14cc, struct_1.field149_0x98);
        uVar14 = param_1;
        puStack54 = CONCAT22(uVar14, uVar1);
        uVar7 = param_1;
        uVar11 = pass1_1008_4772(CONCAT22(uVar14, uVar1));
        uVar6 = (uVar11 >> 0x10) | (uVar11 & 0xffff);
        uVar7 = uVar7 & 0xffff0000 | uVar6;
        if (uVar6 == 0) {
            puVar2 = (uVar11 & 0xffff);
            if (puStack54.is_null() == false) {
                puVar2 = puStack54;
                if (puStack54.is_null() == false) {
                    fn_ptr_1 = *puStack54;
                    (**fn_ptr_1)(0x8, uVar1, param_1, 0x1, uVar14);
                    puVar2 = puStack54;
                }
            }
            uVar1 = FUN_1010_830a(puVar2, uVar7, 0x1008, _u16_1050_14cc, 0x4d);
            puStack54 = CONCAT22(uVar7, uVar1);
        }
        uVar13 = &DAT_1050_1050;
        uVar10 = SUB21(&hdc16_2c, 0x0);
        base_addr = s_tile2_bmp_1050_1538;
        caption_height_px = GetSystemMetrics16(SM_CYCAPTION);
        puVar2 = -(caption_height_px - 0x23);
        fn_ptr_1 = (*puStack54 + 0x4);
        (**fn_ptr_1)(0x38, puStack54, (puStack54 >> 0x10), -(caption_height_px - 0x23), uVar10, uVar13);
    }
    if (puStack54.is_null() == false) {
        uVar1 = (puStack54 >> 0x10);
        puVar2 = puStack54;
        if (puStack54.is_null() == false) {
            fn_ptr_1 = *puStack54;
            (**fn_ptr_1)(base_addr, puStack54, uVar1, 0x1, puStack54, uVar1);
            puVar2 = puStack54;
        }
    }
    uVar1 = FUN_1010_830a(puVar2, uVar7, base_addr, _u16_1050_14cc, struct_1.field148_0x96);
    puStack54 = CONCAT22(uVar7, uVar1);
    uVar14 = SUB42(&DAT_1050_1050, 0x0);
    uVar10 = SUB21(&hdc16_2c, 0x0);
    uVar8 = uVar7;
    IVar2 = GetSystemMetrics16(SM_CYCAPTION);
    uVar3 = *puStack54;
    fn_ptr_1 = uVar3 + 2;
    (**fn_ptr_1)(0x38, uVar1, uVar7, -(IVar2 - 0x23), uVar10, uVar14);
    if (puStack54.is_null() == false) {
        if (puStack54.is_null() == false) {
            fn_ptr_1 = uVar3;
            (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar1, uVar7, 1);
        }
    }
    handle = CreatePen16(0x1000025, 0x0, 0x0);
    handle_00 = SelectObject16(handle, hdc16_2c);
    uVar14 = FUN_1010_830a(handle_00, uVar8, s_tile2_bmp_1050_1538, _u16_1050_14cc, 0x4f);
    puStack54 = CONCAT22(uVar8, uVar14);
    uVar12 = pass1_1008_4772(CONCAT13((uVar8 >> 0x8), CONCAT12(uVar8, uVar14)));
    uVar1 = (uVar12 >> 0x10);
    uVar4 = (uVar12 + 0x4);
    uVar2 = (uVar12 + 0x8);
    IVar3 = GetSystemMetrics16(SM_CYCAPTION);
    y = -(IVar3 - 0xc1);
    IVar4 = GetSystemMetrics16(SM_CYCAPTION);
    iStack72 = uVar2;
    y_00 = 0xc5 - (IVar4 - iStack72);
    MoveTo16(y, 0x82, hdc16_2c);
    iStack68 = uVar4;
    x = iStack68 * 0xa + 0x85;
    LineTo16(y, x, hdc16_2c);
    LineTo16(y_00, x, hdc16_2c);
    LineTo16(y_00, 0x82, hdc16_2c);
    LineTo16(y, 0x82, hdc16_2c);
//   for (uStack82 = 0; puVar1 = &struct_1.field147_0x94, uStack82 <= *puVar1 && *puVar1 != uStack82; uStack82 += 1)
    uStack82 = 0;
    puVar1 = &struct_1.field147_0x94;
    while uStack82 <= *puVar1 && *puVar1 != uStack82 {
        IVar5 = GetSystemMetrics16(SM_CYCAPTION);
        fn_ptr_1 = (*puStack54 + 0x4);
        (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar14, uVar8, -(IVar5 - 0xc4));
        uStack82 += 1;
    }
    if (puStack54.is_null() == false) {
        if (puStack54.is_null() == false) {
            fn_ptr_1 = *puStack54;
            (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar14);
        }
    }
    SelectObject16(handle_00, hdc16_2c);
    DeleteObject16(handle);
    palette_handle_7 = SelectPalette16(0x0, palette_handle_7, hdc16_2c);
    DeleteObject16(palette_handle_7);
    EndPaint16(CONCAT22(0x1050, paint_struct_2a), struct_1.field6_0x6);
    return;
}


pub unsafe fn pass1_1040_6360(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn show_win_1040_65ba(param_1: *mut StructD, struct_b_param_1: *mut StructB, mut param_3: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut rect: *mut Struct57;
    let mut uVar4: u16;
    let mut uVar5: *mut StructD;
    let mut paVar5: *mut Struct57;
    let mut struct_b_4: *mut StructB;
    let mut iVar6: i16;
    let mut unaff_SI: u16;
    let mut unaff_DI: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_stack_0000fe2a: u16;
    let mut in_stack_0000fe2e: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ff54: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut local_22: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut puStack26: *mut u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut puStack6: *mut u32;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar5 = param_1;
    puStack6 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b),
                               in_stack_0000fe7e, in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    uVar5 = (uVar5 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_0898();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar5);
    } else {
        uVar5 = (uVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(uVar5, PTR_LOOP_1050_5f2c);
    uVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, uVar5);
    uVar7 = (struct_b_param_1 >> 0x10);
    struct_b_4 = struct_b_param_1;
    struct_b_4[0x7].field1_0x2 = uVar3;
    struct_b_4[0x7].hwnd_0x6 = uVar5;
//   for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1)
    for iStack10 in 1..uStack8 {
        puStack26 = pass1_1010_0946(puStack6, (puStack6 >> 0x10), iStack10, uVar5, unaff_DI,
                                    &DAT_1050_1050);
        paVar5 = (uVar5 & 0xffff0000 | puStack26 >> 0x10);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar5);
        uVar4 = paVar5 | rect;
        uVar5 = (paVar5 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar2 = &struct_b_4[0x7].field1_0x2;
            (uVar2 + iStack10 * 0x4) = 0;
        } else {
            pvVar1 = struct_b_4.lpvoid_field_0x8;
            pass1_1008_3bd6(uVar5, rect, paVar5, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, (puStack26 + 0x4))), param_3, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar2 = &struct_b_4[0x7].field1_0x2;
            uVar8 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            (iVar6 + iStack10 * 0x4) = rect;
            (iVar6 + iStack10 * 0x4 + 0x2) = uVar5;
        }
        uVar2 = &struct_b_4[0x7].field1_0x2;
        uVar8 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        if ((iVar6 + iStack10 * 0x4) != 0) {
            unaff_DI = puStack26;
            enable_win_1040_9234((iVar6 + iStack10 * 0x4), (unaff_DI + 0x6));
        }
    }
    move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
    ShowWindow16(0x5, struct_b_4.lpvoid_field_0x8);
    return;
}


pub unsafe fn post_win_msg_1040_672e(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut iVar1: i16;

    if (param_5 == s_vrpal_bmp_1050_183a + 0x7) {
        msg_box_ui_op_1040_64ca(0x0, param_1, CONCAT22(param_3, param_2));
    } else {
        if (param_5 == 0x1851) {
            iVar1 = 0x2a;
        } else {
            if (param_5 != 0x1852) {
                post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
                return;
            }
            iVar1 = 0x29;
        }
        pass1_1038_af40(param_2, param_1, _PTR_LOOP_1050_5b7c, (param_2 + 0x8), iVar1);
        PostMessage16(0x0, 0x2, 0x111, (param_2 + 0x6));
    }
    return;
}


pub unsafe fn pass1_1040_6794(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_6470(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn enable_win_1040_6880(param_1: *mut astruct_925, mut param_2: i16)

{
    let mut uVar2: u32;
    let mut HVar3: HWND16;
    let mut iVar3: *mut astruct_925;
    let mut uVar4: u16;
    let mut uVar1: u32;

    if (param_2 == 0x8) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar3 = GetDlgItem16(0x107, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x24), HVar3);
        HVar3 = GetDlgItem16(0x108, iVar3.field6_0x6);
        uVar2 = iVar3.field147_0x94;
        EnableWindow16((uVar2 + 0x26), HVar3);
    }
    return;
}


pub unsafe fn pass1_1040_68d2(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2));
        }
    } else {
        if (param_5 != 0x111) {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return uVar2;
        }
        ppcVar1 = (*param_1 + 0x80);
        (**ppcVar1)();
    }
    return 0x1;
}


pub unsafe fn pass1_1040_692e(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}


pub unsafe fn mixed_win_ui_op_1040_6942(param_1: *mut Struct57, mut param_2: u16, struct_b_param_1: *mut StructB, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut Struct57;
    let mut hwnd: *mut u32;
    let mut iVar3: *mut astruct_790;
    let mut uVar4: u16;
    let mut uVar10: u16;
    let mut uVar5: u16;
    let mut paVar13: *mut Struct57;
    let mut struct_b_6: *mut StructB;
    let mut uVar6: u16;
    let mut uVar9: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut puVar15: *mut u32;
    let mut puVar14: *mut u16;
    let mut DVar16: u32;
    let mut in_stack_0000fdd4: u16;
    let mut in_stack_0000fdd6: u16;
    let mut in_stack_0000fdd8: u16;
    let mut in_stack_0000fdda: u16;
    let mut in_stack_0000fe32: u16;
    let mut in_stack_0000fefe: u16;
    let mut in_stack_0000ff00: u16;
    let mut in_stack_0000ff02: u16;
    let mut in_stack_0000ff04: u16;
    let mut in_stack_0000ff06: u16;
    let mut in_stack_0000ff08: u16;
    let mut in_stack_0000ff56: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ff60: u16;
    let mut uVar17: u8;
    let mut uVar20: u8;
    let mut BVar21: bool;
    let mut uVar22: u16;
    let mut pcVar23: *mut c_char;
    let mut hdc: HDC16;
    let mut local_64: u32;
    let mut uStack96: u32;
    let mut HStack92: HWND16;
//   HMENlet mut HStack90: u16;
    let mut HStack90: HMENU16;
    let mut local_58: [u8; 0x50] = [0; 0x50];
    let mut hdc_8: HDC16;
    let mut paStack6: *mut Struct57;
    let mut uStack4: u16;
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar18: u8;
    let mut uVar19: u8;
    let mut in_stack_0000ff8a: u16;
    let mut paVar11: *mut Struct57;
    let mut paVar12: *mut Struct57;
    let mut uVar14: u32;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar15 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_2, 0x33), in_stack_0000fe32,
                              in_stack_0000ff56, in_stack_0000ff5c, in_stack_0000ff60);
    paVar11 = (param_1 & 0xffff0000 | puVar15 >> 0x10);
    paVar3 = puVar15;
    uVar6 = (struct_b_param_1 >> 0x10);
    struct_b_6 = struct_b_param_1;
    struct_b_6[0x7].max_count_field_0x10 = paVar3;
    struct_b_6[0x7].field5_0xa = (puVar15 >> 0x10);
    ppcVar2 = (*&struct_b_6[0x7].max_count_field_0x10 + 0x4);
    (**ppcVar2)(0x1010, struct_b_6[0x7].max_count_field_0x10, (puVar15 >> 0x10), 0x0, struct_b_param_1);
    mem_op_1000_179c(0xa, paVar11);
    uVar10 = paVar11 | paVar3;
    paVar12 = (paVar11 & 0xffff0000 | uVar10);
    if (uVar10 == 0) {
        struct_b_6[0x7].field6_0xc = 0;
    } else {
        puVar14 = struct_1040_bf3e(CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, paVar3)),
                                   struct_b_6.lpvoid_field_0x8);
        paVar12 = (paVar12 & 0xffff0000 | puVar14 >> 0x10);
        paVar3 = puVar14;
        struct_b_6[0x7].field6_0xc = paVar3;
        struct_b_6[0x7].field7_0xe = (puVar14 >> 0x10);
    }
    // WARNING: Load size is inaccurate
    pass1_1040_bfde(struct_b_6[0x7].field6_0xc, &struct_b_6[0x7].max_count_field_0x10);
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar3;
    paVar11 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar11, paVar3, paVar12, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(struct_b_6.lpvoid_field_0x8, 0x10a), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar10 = paVar11 | paVar3;
    paVar12 = (paVar11 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar3, paVar11, 0x1, 0xa0028, 0x0, 0x820083,
                        CONCAT22(struct_b_6.lpvoid_field_0x8, 0x10c), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    BVar21 = 0;
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar3;
    paVar11 = (paVar12 & 0xffff0000);
    paVar13 = (paVar11 | uVar10);
    if (uVar10 == 0) {
        paVar3 = null_mut();
    } else {
        pvVar1 = struct_b_6.lpvoid_field_0x8;
        pass1_1008_3bd6(paVar13, paVar3, paVar12, 0x1, 0xa00aa, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x107)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        paVar11 = paVar13;
    }
    uStack4 = SUB42(paVar11, 0x0);
    paStack6 = paVar3;
    enable_win_1040_9234(CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, paVar3)), BVar21);
    BVar21 = 0;
    mem_op_1000_179c(0x42, paVar11);
    uVar5 = paVar11 | paVar3;
    uVar14 = paVar11 & 0xffff0000 | uVar5;
    if (uVar5 == 0) {
        paVar3 = null_mut();
        uStack4 = 0;
    } else {
        pvVar1 = struct_b_6.lpvoid_field_0x8;
        pass1_1008_3bd6(uVar14, paVar3, paVar11, 0x1, 0xa00c2, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x108)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        uStack4 = uVar14;
    }
    paStack6 = paVar3;
    enable_win_1040_9234(CONCAT13((uStack4 >> 0x8), CONCAT12(uStack4, paVar3)), BVar21);
    hdc_8 = GetDC16(struct_b_6.lpvoid_field_0x8);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar22 = SUB42(&DAT_1050_1050, 0x0);
    uVar17 = SUB21(local_58, 0x0);
    uVar20 = (local_58 >> 0x8);
    hdc = hdc_8;
    uVar10 = str_op_1000_3da4(CONCAT22(0x1050, local_58));
    DVar16 = GetTextExtent16(uVar10, CONCAT22(uVar22, CONCAT11(uVar20, uVar17)), hdc);
    HStack90 = (HMENU16)(DVar16 >> 0x10);
    HStack92 = DVar16;
    CreateWindow16(0x0, CONCAT22(0x7cd, HINSTANCE16_1050_038c), struct_b_6.lpvoid_field_0x8, HStack90,
                   HStack92, 0xad, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT13(0x10, CONCAT12(0x50, local_58)),
                   s_static_1050_5d84);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar18 = hdc_8;
    uVar19 = (hdc_8 >> 0x8);
    pcVar23 = local_58;
    uVar22 = SUB42(&DAT_1050_1050, 0x0);
    uVar10 = str_op_1000_3da4(CONCAT13(0x10, CONCAT12(0x50, pcVar23)));
    DVar16 = GetTextExtent16(uVar10, CONCAT22(uVar22, pcVar23), CONCAT11(uVar19, uVar18));
    HStack90 = (HMENU16)(DVar16 >> 0x10);
    HStack92 = DVar16;
    ReleaseDC16(hdc_8, struct_b_6.lpvoid_field_0x8);
    CreateWindow16(0x0, CONCAT22(0x7ce, HINSTANCE16_1050_038c), struct_b_6.lpvoid_field_0x8, HStack90,
                   HStack92, 0xc5, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT22(0x1050, local_58),
                   s_static_1050_5d8b);
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    hwnd = &local_64;
    create_window_1040_6eae(struct_b_param_1, 0x1, CONCAT22(0x1050, hwnd), 0x5eb, 0xfd);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_6eae(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ec, 0xfe);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_6eae(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ee, 0xff);
    SendMessage16(0x0, 0x1, 0x401, hwnd);
    uVar1 = &struct_b_6[0x7].max_count_field_0x10;
    iVar3 = uVar1;
    iVar3 = &iVar3.field_0xa;
    uVar9 = ((uVar1 & 0xffff0000) >> 0x10);
    SetWindowPos16(0x40, iVar3.field14_0x10, iVar3.field13_0xe, iVar3.field12_0xc,
                   (uVar1 & 0xffff0000 | ZEXT24(iVar3)), 0x0, struct_b_6.lpvoid_field_0x8);
    DAT_1050_0ecc = 0;
    uVar2 = &struct_b_6[0x7].max_count_field_0x10;
    ppcVar2 = (*&struct_b_6[0x7].max_count_field_0x10 + 0x10);
    (**ppcVar2)(s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10));
    pass1_1010_2ee2(&struct_b_6[0x7].max_count_field_0x10);
    PostMessage16(0x0, 0x10a, 0x111, struct_b_6.lpvoid_field_0x8);
    return;
}
