use crate::block_1020::block_1020_6000::{destroy_icon_1020_6bd2, pass1_1020_6466, pass1_1020_64d4, pass1_1020_6746};
use crate::block_1020::block_1020_7000::{cleanup_menu_ui_op_1020_795c, invalidate_rect_1020_735a, palette_op_1020_7270, pass1_1020_75c4, pass1_1020_7824, pass1_1020_78ac};
use crate::block_1020::block_1020_8000::{pass1_1020_808e, pass1_1020_8556};

pub unsafe fn unk_win_op_1020_65cc(param_1: *mut astruct_60, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut iVar4: *mut astruct_59;
    let mut iVar5: *mut astruct_60;
    let mut iVar6: i16;
    let mut uVar7: *mut astruct_60;
    let mut iStack4: i16;

    iVar5 = param_1;
    uVar7 = (param_1 >> 0x10);
    if (param_2 == 1) {
        iVar5.field20_0x14 = 0;
        return;
    }
    if (param_2 == 0x2) {
        for iStack4 in 0..0x5 {
            iVar6 = iStack4 * 0x4;
            if (((&iVar5.field_0x1a + iVar6) | (&iVar5.field_0x18 + iVar6)) != 0) {
                ppcVar1 = (*(&iVar5.field_0x18 + iVar6) + 0x4);
                (**ppcVar1)();
            }
        }
    } else if (((0x0 < param_2 - 0x3) && (!SBORROW2(param_2 - 0x3, 1))) && (param_2 - 0x4 < 0x4)) {
        BVar3 = IsIconic16(HWND16_1050_0396);
        if (BVar3 == 0) {
            BVar3 = IsIconic16(&iVar5.field_0x4);
            if ((BVar3 == 0) && (uVar2 = iVar5.field20_0x14, (uVar2 + 0x24) != 0)) {
                InvalidateRect16(0x0, NULL, 0x0);
                uVar4 = pass1_1020_64d4(param_1, 0x2);
                if (uVar4 == 0) {
                    pass1_1020_6746(param_1, 0x1, 0x2);
                }
                uVar4 = pass1_1020_64d4(param_1, 0x3);
                if (uVar4 == 0) {
                    pass1_1020_6746(param_1, 0x1, 0x3);
                }
                uVar4 = pass1_1018_255e(iVar5.field20_0x14);
                if (uVar4 == 0) {
                    SendMessage16(0x0, 0x69, 0x111, &iVar5.field_0x4);
                } else {
                    uVar4 = pass1_1020_64d4(param_1, 1);
                    if (uVar4 == 0) {
                        pass1_1020_6746(param_1, 0x1, 1);
                    }
                }
                SendMessage16(0x0, 0xf0, 0x111, &iVar5.field_0x4);
                uVar2 = iVar5.field41_0x2c;
                if ((uVar2 + 0x7a) != 0) {
                    uVar2 = iVar5.field41_0x2c;
                    (uVar2 + 0x7a) = 0;
                    SendMessage16(0x0, 0x131, 0x111, &iVar5.field_0x4);
                    return;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1020_679c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_6466(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_687c(param_1: *mut astruct_868) {
    let mut unaff_BP: u8;

    get_win_ui_info_op_1020_7a50(param_1);
    destroy_icon_1020_6bd2(param_1, unaff_BP);
    return;
}

pub unsafe fn realize_palette_1020_6896(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar3: u32;
    let mut puVar4: u32;
    let mut iVar4: *mut astruct_801;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;

    if (param_2 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar2 = (param_1 + 0xf2);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        puVar4 = iVar4.field36_0x24;
        ppcVar1 = (puVar4 + 0x18);
        (**ppcVar1)();
        UnrealizeObject16(puVar4);
        uVar3 = (param_1 + 0xf2);
        RealizePalette16(*(uVar3 + 0x178));
    }
    return;
}

pub unsafe fn pt_in_rect_1020_68fc(param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut POPStack6: INT16;

    PStack6 = CONCAT22(param_2, param_3);
    uVar4 = (param_1 >> 0x10);
    uVar2 = pass1_1018_31d0((param_1 + 0xf2));
    if (uVar2 != 0) {
        BVar3 = PtInRect16(PStack6, ((param_1 + 0xf2) + 0x16c));
        if (BVar3 != 0) {
            ppcVar1 = (*param_1 + 0x40);
            (**ppcVar1)(s_tile2_bmp_1050_1538, param_1, 0xef);
        }
    }
    return;
}

pub unsafe fn unk_destroy_win_op_1020_694c(
    mut param_1: u16,
    param_2: *mut StructA,
    mut param_3: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut HVar4: HWND16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut uVar6: u32;
    let iVar5: *mut StructA;
    let uVar4: *mut StructA;
    let mut unaff_CS: u16;
    let mut wparam: WPARAM16;

    uVar6 = CONCAT22(in_register_0000000a, param_1);
    uVar1 = param_3;
    if (param_3 != 0x12b) {
        iVar5 = param_2;
        uVar4 = (param_2 >> 0x10);
        if (param_3 < 0x12c) {
            if (param_3 == 0x6f) {
                uVar2 = FUN_1010_830a(0x0, uVar6, unaff_CS, _u16_1050_14cc, 0x1f8);
                BVar3 = WinHelp16(0x29, 0x1, CONCAT22(uVar6, uVar2), iVar5.field4_0x8);
                return BVar3;
            }
            if (param_3 == 0xeb) {
                uVar1 = GetDlgItem16(0x1797, iVar5.field4_0x8);
                uVar5 = uVar6;
                if (uVar1 != 0) {
                    //
                    // LAB_1020_6a6f:
                    win_ui_fn_1020_6e98(uVar5, param_2);
                    return uVar1;
                }
            } else {
                uVar1 = param_3 - 0xef;
                if (uVar1 == 0) {
                    pass1_1018_2e28(&iVar5[0x1].field20_0x26);
                    pass1_1008_3e0e(param_2);
                } else {
                    uVar1 = param_3 - 0x129;
                    if ((uVar1 != 0) && (uVar1 = param_3 - 0x12a, uVar1 == 0)) {
                        HVar4 = iVar5.field4_0x8;
                        wparam = 0xf012; //
                        // LAB_1020_69c3:
                        BVar3 = PostMessage16(0x0, wparam, 0x112, HVar4);
                        return BVar3;
                    }
                }
            }
        } else if (param_3 == 0xbb8) {
            HVar4 = GetDlgItem16(0x1797, iVar5.field4_0x8);
            if (HVar4 != 0) {
                DestroyWindow16(HVar4);
            }
            uVar1 = pass1_1018_31d0(&iVar5[0x1].field20_0x26);
            if (uVar1 != 0) {
                uVar1 = pass1_1018_2d9a(&iVar5[0x1].field20_0x26); //
                // LAB_1020_6a0b:
                invalidate_rect_1020_735a(&iVar5[0x1].field22_0x2a);
                return uVar1;
            }
        } else if (param_3 < 0xbb9) {
            if (param_3 == 0x12c) {
                HVar4 = iVar5.field4_0x8;
                wparam = 0xf020;
                // TODO: goto LAB_1020_69c3;
            }
            uVar1 = param_3 - 0x12d;
            if (param_3 != 0x12c) {
                uVar1 = param_3 - 0x12e;
            }
        } else if (param_3 == 0xbb9) {
            HVar4 = GetDlgItem16(0x1797, iVar5.field4_0x8);
            if (HVar4 != 0) {
                DestroyWindow16(HVar4);
            }
            uVar1 = pass1_1018_31d0(&iVar5[0x1].field20_0x26);
            if (uVar1 != 0) {
                uVar1 = pass1_1018_2dde(&iVar5[0x1].field20_0x26);
                // TODO: goto LAB_1020_6a0b;
            }
        } else {
            uVar1 = param_3 - 0xbba;
            if (uVar1 == 0) {
                uVar1 = GetDlgItem16(0x1797, iVar5.field4_0x8);
                uVar5 = uVar6;
                if (uVar1 != 0) {
                    BVar3 = DestroyWindow16(uVar1);
                    return BVar3;
                }
                // TODO: goto LAB_1020_6a6f;
            }
        }
    }
    return uVar1;
}


pub unsafe fn win_ui_op_1020_6ae6(
    param_1: *mut astruct_877,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
    param_5: u16,
    param_6: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut hwnd: HWND16;
    let mut puVar2: *mut u8;
    let mut iVar3: *mut astruct_877;
    let mut uVar3: u16;
    let mut LVar4: LRESULT;

    if (param_4 == 0x1797) {
        uVar3 = (param_1 >> 0x10);
        iVar3 = param_1;
        hwnd = GetDlgItem16(0x1797, iVar3.field8_0x8);
        if (hwnd != 0) {
            if (param_3 == 0x2) {
                LVar4 = SendMessage16(0x0, 0x0, 0x409, hwnd);
                if (LVar4 != -1) {
                    LVar4 = SendMessage16(
                        CONCAT13(0x10, CONCAT12(0x50, &stack0xffa8)),
                        LVar4,
                        0x40a,
                        hwnd,
                    );
                    puVar2 = &stack0xffa8;
                    pass1_1018_30ca(
                        (LVar4 >> 0x10),
                        iVar3.field241_0xf2,
                        CONCAT22(0x1050, puVar2),
                    );
                    pass1_1018_2fe8(iVar3.field241_0xf2, param_5, param_6);
                    if (puVar2.is_null() == false) {
                        invalidate_rect_1020_735a(iVar3.field242_0xf6);
                        ppcVar1 = (param_1 + 0x40);
                        (**ppcVar1)(0x1018, iVar3);
                    }
                }
            } else if (param_3 != 0x3) {
                return;
            }
            DestroyWindow16(hwnd);
        }
    }
    return;
}


pub unsafe fn enable_menu_item_1020_6b9a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: HMENU16,
) {
    if (param_4 != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0x0, param_5);
    return;
}

pub unsafe fn pass1_1020_6bbc(mut param_1: u32) {
    let mut in_DX: u16;

    win_ui_op_1020_737a(in_DX, (param_1 + 0xf6));
    return;
}

pub unsafe fn pass1_1020_6e52(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: i16,
) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;

    pass1_1018_2e5e(param_1, param_2, (param_3 + 0xf2));
    uVar1 = param_2 | param_1;
    if (uVar1 == 0) {
        pcVar2 = load_string_1010_847e(_u16_1050_14cc, 0x5a1);
    } else {
        pass1_1018_2d84(param_1, (param_3 + 0xf2));
        pcVar2 = CONCAT22(uVar1, param_1);
    }
    string_1020_79b4(CONCAT22(param_4, param_3), param_5, pcVar2);
    return;
}

pub unsafe fn draw_op_1020_7070(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    hdc_param_5: HDC16,
) -> u16 {
    let mut hgdi_obj: u16;

    GetStockObject16(BLACK_BRUSH);
    if (COLORREF_1050_441e == 0) {
        COLORREF_1050_441e = 0x1000002;
    }
    if (0x6 < param_4) {
        return 0x0;
    }
    SetTextColor16(COLORREF_1050_441e, hdc_param_5);
    hgdi_obj = 0x100;
    SetBkColor16(0x1000000, hdc_param_5);
    return hgdi_obj;
}


pub unsafe fn pass1_1020_70c0(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    cleanup_menu_ui_op_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn post_win_msg_1020_7308(mut param_1: u32, mut param_2: u16) {
    let mut cVar1: u8;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (param_2 != 0x12) {
        if (param_2 < 0x13) {
            cVar1 = param_2;
            if (cVar1 == '\x01') {
                (param_1 + 0x1c) = 0;
                return;
            }
            //      if (('\x03' < (cVar1 -1)) && ((cVar1 -0x5) < '\x02')) goto LAB_1020_7310;
        }
        return;
    } //
    // LAB_1020_7310:
    PostMessage16(0x0, 0xeb, 0x111, (param_1 + 0x4));
    invalidate_rect_1020_735a(param_1);
    return;
}

pub unsafe fn pass1_1020_7526(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    palette_op_1020_7270(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_1020_75f0(param_1: *mut astruct_283, mut param_2: u32) {
    let mut pUVar1: *mut u16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar8: u16;
    let mut iVar7: *mut astruct_283;
    let mut uVar9: u16;
    let mut puVar10: *mut u16;
    let mut in_stack_0000ff4c: u16;
    let mut in_stack_0000ff62: u16;
    let mut uStack10: u32;
    let mut local_6: [u8; 0x4] = [0; 0x4];
    let mut paVar6: *mut Struct57;
    let mut uVar7: u32;

    uVar8 = (param_2 >> 0x10);
    uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    if (iVar7.field235_0xee.is_null() == false) {
        ppcVar2 = (*iVar7.field235_0xee + 0x8);
        (**ppcVar2)();
    }
    if (iVar7.field233_0xea == 0) {
        iVar7.field233_0xea = 0x1;
        puVar10 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x91);
        uVar4 = (puVar10 >> 0x10);
        paVar6 = CONCAT22(uVar8, uVar4);
        uVar3 = ZEXT24(local_6);
        win_1008_5c9e(local_6, uVar4, _u16_1050_02a0, CONCAT22(0x1050, local_6));
        iVar7.field234_0xec = uVar3;
        mem_op_1000_179c(0x112, paVar6);
        uVar5 = paVar6 | uVar3;
        uVar7 = paVar6 & 0xffff0000 | uVar5;
        if (uVar5 == 0) {
            uVar9 = 0;
            uStack10 = null_mut();
        } else {
            pUVar1 = &iVar7.field204_0xcc;
            *pUVar1 = *pUVar1 + 1;
            struct_1020_3644(
                uVar7,
                CONCAT13((paVar6 >> 0x8), CONCAT12(paVar6, uVar3)),
                iVar7.field204_0xcc,
                param_1 & 0xffff | uVar9 << 0x10,
                in_stack_0000ff4c,
                in_stack_0000ff62,
            );
            uVar9 = uVar3;
            uStack10 = (uVar3 & 0xffff | uVar7 << 0x10);
        }
        pass1_1008_6978(uVar9, uVar7, param_1, 0x0, uStack10 & 0xffff0000 | uVar9);
        ppcVar2 = (*uStack10 + 0xc);
        (**ppcVar2)(0x8, uStack10, uStack10, 0x5);
    }
    return;
}


pub unsafe fn window_op_1020_76aa(param_1: *mut StructA, param_2: *mut astruct_666) {
    let mut uVar3: u16;
    let mut in_EDX: *mut Struct57;
    let iVar1: *mut StructA;
    let mut uVar2: u16;

    create_window_ex_1008_9760(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    get_dc_1018_4db0(&iVar1[0x1].field20_0x26, iVar1.field4_0x8);
    mem_op_1000_179c(0x18, in_EDX);
    uVar3 = in_EDX | param_2;
    if (uVar3 != 0) {
        pass1_1020_7824(uVar3, param_2, in_EDX, iVar1.field4_0x8);
        iVar1[0x1].field18_0x22 = param_2;
        iVar1[0x1].field19_0x24 = uVar3;
        return;
    }
    iVar1[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn pass1_1020_770e(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0xee);
    uVar2 = (iVar4 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0xee) = 0;
    destroy_win_1008_628e(param_1 & 0xffff | uVar5 << 0x10);
    return;
}


pub unsafe fn pass1_1020_774c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_75c4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_78dc(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_78ac(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_ui_menu_op_1020_7ad2(
    param_1: *mut astruct_854,
    param_2: HWND16,
    param_3: *mut RECT16,
) {
    //   HMENlet mut HVar1: u16;
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_854;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2.field236_0xee.is_null() == false) && (iVar2.field235_0xec == 0)) {
        HVar1 = LoadMenu16(iVar2.field236_0xee, HINSTANCE16_1050_038c);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field235_0xec);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field8_0x8);
    HVar1 = iVar2.field235_0xec;
    TrackPopupMenu16(NULL, iVar2.field8_0x8, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}

pub unsafe fn pass1_1020_7b60(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    cleanup_menu_ui_op_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_7f38(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x3ab0;
    (param_1 + 0x2) = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_8106(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x4) + 0x60);
    (**ppcVar1)();
    return;
}

pub unsafe fn realize_palette_1020_8128(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut extraout_DX: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut local_12: [u8; 0x8] = [0; 0x8];
    let mut hdc_10: HDC16;
    let mut HStack8: HGDIOBJ16;
    let mut puStack6: *mut u32;

    if (param_2 != 0) {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        uVar2 = (iVar6 + 0xe6);
        puVar5 = (uVar2 + 0xa);
        ppcVar1 = (*puVar5 + 0x18);
        puStack6 = puVar5;
        (**ppcVar1)();
        HStack8 = puVar5;
        UnrealizeObject16(HStack8);
        uVar2 = (iVar6 + 0xe6);
        hdc_10 = *(uVar2 + 0x14);
        RealizePalette16(hdc_10);
        pass1_1008_57a4(
            CONCAT22(0x1050, local_12),
            param_1 & 0xffff0000 | (iVar6 + 0xd2),
        );
        loop {
            puVar3 = local_12;
            pass1_1008_5b12(CONCAT22(0x1050, puVar3));
            if ((extraout_DX | puVar3) == 0) {
                break;
            }
            uVar2 = (puVar3 + 0x4);
            uVar7 = (puVar3 + 0x6);
            puVar4 = uVar2;
            ppcVar1 = (*puVar4 + 0x90);
            (**ppcVar1)(0x1008, puVar4, uVar7, 0x1, uVar2);
        }
    }
    return;
}

pub unsafe fn win_ui_palette_op_1020_81c0(mut param_1: u32) {
    let mut in_struct_1: *mut astruct_13;
    let mut hdc: HDC16;
    let mut hpal: HDC16;
    let mut hpal_00: HPALETTE16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut hdc_00: HDC16;
    let mut uStack6: u16;

    uVar2 = (_PTR_LOOP_1050_4230 >> 0x10);
    in_struct_1 = (_PTR_LOOP_1050_4230 + 0xe);
    uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
    uStack6 = in_struct_1;
    if ((uVar1 | uStack6) == 0) {
        return;
    }
    hdc = GetDC16((param_1 + 0x8));
    hpal = hdc;
    hdc_00 = hdc;
    create_palette_1008_4e38(in_struct_1, uVar1);
    hpal_00 = SelectPalette16(0x0, hpal, hdc_00);
    RealizePalette16(hdc);
    SelectPalette16(0x1, hpal_00, hdc);
    RealizePalette16(hdc);
    DeleteObject16(hpal);
    if (0x0 < hpal) {
        InvalidateRect16(0x1, NULL, 0x0);
    }
    return;
}

pub unsafe fn destroy_window_1020_8250(param_1: *mut astruct_879) {
    let mut BVar1: bool;
    let mut iVar2: *mut astruct_879;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field236_0xec != 0) {
        BVar1 = IsWindow16(iVar2.field236_0xec);
        if (BVar1 != 0) {
            DestroyWindow16(iVar2.field236_0xec);
            iVar2.field236_0xec = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1020_8288(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_808e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1020_8296(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1020_808e(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1020_83f8(mut param_1: u32) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x4) != 0) {
        uVar1 = (iVar3 + 0x1c);
        uVar2 = (iVar3 + 0x1c);
        pass1_1008_4480(
            (uVar1 + 0xa),
            (param_1 & 0xffff0000 | (iVar3 + 0x16)),
            (uVar2 + 0x2a),
        );
    }
    return;
}

pub unsafe fn FUN_1020_8438() {
    return;
}

pub unsafe fn pass1_1020_843c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_85f6(param_1: *mut astruct_590) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut pcVar3: *mut c_char;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar6: *mut astruct_590;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        piVar1 = &iVar6.field6_0x6;
        if (*piVar1 == iStack4 || *piVar1 < iStack4) {
            break;
        }
        uVar4 = iVar6.field11_0xc;
        uVar6 = (uVar4 >> 0x10);
        iVar5 = uVar4;
        pcVar3 = *(iVar5 + iStack4 * 0x4);
        uVar2 = (iVar5 + iStack4 * 0x4 + 2);
        if ((uVar2 | pcVar3) != 0) {
            pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
            fn_ptr_1000_17ce(pcVar3);
        }
        uVar4 = iVar6.field11_0xc;
        (uVar4 + iStack4 * 0x4) = 0;
        iStack4 += 0x1;
    }
    return;
}

pub unsafe fn pass1_1020_865a(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut pcVar3: *mut c_char;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar7: *mut astruct_592;
    let mut iVar6: *mut astruct_591;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        uVar9 = (param_1 >> 0x10);
        iVar5 = param_1;
        piVar1 = (iVar5 + 0x6);
        if (*piVar1 == iStack4 || *piVar1 < iStack4) {
            break;
        }
        iVar8 = iStack4 * 0x4;
        uVar4 = (iVar5 + 0xc);
        uVar10 = (uVar4 >> 0x10);
        iVar7 = uVar4;
        if ((iVar7 + iVar8) != 0) {
            pass1_1008_5236((iVar7 + iVar8));
            uVar4 = (iVar5 + 0xc);
            uVar10 = (uVar4 >> 0x10);
            iVar6 = uVar4;
            pcVar3 = *(iVar6 + iVar8);
            uVar2 = (iVar6 + iVar8 + 2);
            if ((uVar2 | pcVar3) != 0) {
                pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
                fn_ptr_1000_17ce(pcVar3);
            }
            uVar4 = (iVar5 + 0xc);
            (uVar4 + iStack4 * 0x4) = 0;
        }
        iStack4 += 0x1;
    }
    return;
}


pub unsafe fn pass1_1020_86d8(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        uVar4 = (param_1 >> 0x10);
        piVar1 = (param_1 + 0x6);
        if (*piVar1 == iStack4 || *piVar1 < iStack4) {
            break;
        }
        uVar2 = (param_1 + 0xc);
        uVar4 = (uVar2 >> 0x10);
        iVar3 = uVar2;
        if ((iVar3 + iStack4 * 0x4) != 0) {
            pass1_1008_5236((iVar3 + iStack4 * 0x4));
        }
        iStack4 += 0x1;
    }
    return;
}

pub unsafe fn FUN_1020_8780() {
    return;
}


pub unsafe fn pass1_1020_8784(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_8908(param_1: *mut astruct_284, mut param_2: u32, mut param_3: u32) {
    let mut paVar1: *mut astruct_76;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut pstruct284_8: *mut astruct_284;
    let mut iVar9: i16;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u32;
    let mut paStack28: *mut astruct_110;
    let mut iStack4: i16;
    let mut paVar8: *mut Struct57;

    //   for (iStack4 = 0; uVar12 = (param_3 >> 0x10), pstruct284_8 = param_1,
    //   uVar11 = (param_1 >> 0x10), iStack4 < 0x4; iStack4 += 1)
    iStack4 = 0;
    uVar12 = param_3 >> 0x10;
    pstruct284_8 = param_1;
    uVar11 = param_1 >> 0x10;
    while iStack4 < 4 {
        if (pstruct284_8.field4_0x4 == 0) {
            uVar2 = pstruct284_8.field11_0xc;
            uVar12 = (uVar2 >> 0x10);
            iVar10 = uVar2;
            iVar9 = iStack4 * 0x4;
            if (((iVar10 + iVar9 + 0x2) | (iVar10 + iVar9)) != 0) {
                pass1_1008_5236((iVar10 + iVar9));
            }
        } else {
            uVar2 = pstruct284_8.field42_0x2e;
            paVar1 = (uVar2 + 0x2e + iStack4 * 0x4);
            uVar13 = pass1_1008_4772(paVar1);
            uVar6 = (uVar13 >> 0x10);
            paVar8 = CONCAT22(uVar12, uVar6);
            uVar3 = uVar13;
            uVar2 = pstruct284_8.field11_0xc;
            iVar10 = iStack4 * 0x4;
            if ((uVar2 + iVar10) == 0) {
                uVar4 = uVar3;
                mem_op_1000_179c(0x14, paVar8);
                uVar7 = paVar8;
                paStack28 = CONCAT22(uVar7, uVar4);
                paVar8 = (paVar8 & 0xffff0000);
                if ((uVar7 | uVar4) == 0) {
                    uVar2 = pstruct284_8.field11_0xc;
                    (uVar2 + iStack4 * 0x4) = 0;
                } else {
                    paVar8 = (paVar8 | uVar11);
                    puVar5 = &pstruct284_8.field_0x16 + iStack4 * 0x6;
                    pass1_1008_50c2(
                        paStack28,
                        (uVar3 + 0x8),
                        (uVar3 + 0x4),
                        (param_1 & 0xffff0000 | ZEXT24(puVar5)),
                        param_2,
                    );
                    uVar2 = pstruct284_8.field11_0xc;
                    uVar12 = (uVar2 >> 0x10);
                    iVar9 = uVar2;
                    (iVar9 + iVar10) = puVar5;
                    (iVar9 + iVar10 + 0x2) = paVar8;
                }
                uVar2 = pstruct284_8.field11_0xc;
                pass1_1008_5134((uVar2 + iStack4 * 0x4));
            }
            uVar12 = (paVar8 >> 0x10);
            uVar2 = pstruct284_8.field11_0xc;
            pass1_1008_5236((uVar2 + iStack4 * 0x4));
            param_3 = CONCAT22(uVar12, uVar11);
            pass1_1008_4480(
                param_2,
                (param_1 & 0xffff0000 | ZEXT24(&pstruct284_8.field_0x16 + iStack4 * 0x6)),
                paVar1,
            );
        }
        iStack4 += 1;
    }
    if (pstruct284_8.field4_0x4 != 0) {
        pass1_1008_4480(
            param_2,
            (param_1 & 0xffff0000 | ZEXT24(&pstruct284_8.field_0x32)),
            pstruct284_8.field49_0x38,
        );
    }
    return;
}
