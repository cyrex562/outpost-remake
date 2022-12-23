use crate::block_1040::block_1040_6000::pass1_1040_6862;
use crate::block_1040::block_1040_7000::{create_window_1040_7620, dialog_ui_fn_1040_78e2};
use crate::block_1040::block_1040_8000::{create_window_1040_8bea, destroy_win_1040_8b7e, get_sys_metrics_1040_8c66};

pub unsafe fn pass1_1040_6cac(mut param_1: u32)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6((iVar4 + 0x94), (param_1 & 0xffff | uVar5 << 0x10));
    puVar1 = (iVar4 + 0x98);
    uVar2 = (iVar4 + 0x9a);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 1);
    }
    (iVar4 + 0x98) = 0;
    (iVar4 + 0x94) = 0;
    return;
}


pub unsafe fn pass1_1040_6cfa(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}


pub unsafe fn win_ui_op_1040_6d1a(param_1: *mut astruct_897, mut param_2: u16, mut param_3: u16, mut param_4: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut Struct27;
    let mut in_DX: *mut u8;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: *mut astruct_896;
    let mut iVar3: *mut astruct_895;

    match param_4 {
        0xfa => {
            ppcVar1 = (param_1.field144_0x94 + 0x18);
            (**ppcVar1)();
        }
        // break;
        _ => {
            pass1_1040_b54a(in_DX, CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)), param_3,
                            param_4);
            return;
        }
        0xfd => {
            if (DAT_1050_0ecc == 0) {
                return;
            }
            DAT_1050_0ecc = 0;
        }
// TODO: goto LAB_1040_6deb;
        0xfe => {
            if (DAT_1050_0ecc == 1) {
                return;
            }
            DAT_1050_0ecc = 0x1;
        }
// TODO: goto LAB_1040_6deb;
        0xff => {
            if (DAT_1050_0ecc == 0x2) {
                return;
            }
            DAT_1050_0ecc = 0x2;//
// LAB_1040_6deb:
            paVar2 = param_1.field144_0x94;
            ppcVar1 = (param_1.field144_0x94 + 0x10);
            (**ppcVar1)(&PTR_LOOP_1050_1040, paVar2, (paVar2 >> 0x10));
            pass1_1010_2ee2(param_1.field144_0x94);
            PostMessage16(0x0, 0x10a, 0x111, param_1.field6_0x6);
        }
        // break;
        0x107 => {
            iVar3 = null_mut();
        }
// TODO: goto LAB_1040_6e48;
        0x108 => {
            iVar3 = (&PTR_LOOP_1050_0000 + 1);//
// LAB_1040_6e48:
            win_ui_op_1010_3202(param_1.field144_0x94, iVar3);
        }
        // break;
        0x10a => {
            GetClientRect16(&local_a, &DAT_1050_1050);
            paVar2 = param_1.field144_0x94;
            local_a.y += 0x3;
            local_a.x = (paVar2 + 0x1a) - 0x9;
            iStack6 += -0x3;
            iStack4 = iStack4 - 0x3;
            InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
            unk_destroy_win_op_1010_2fa0(param_1.field144_0x94);
            pass1_1010_32c0(param_1.field144_0x94, 0x0);
            pass1_1010_2ee2(param_1.field144_0x94);
        }
        // break;
        0x10c => {
            DestroyWindow16(param_1.field6_0x6);
        }
    }
    return;
}


pub unsafe fn pass1_1040_6f0c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_6862(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn enable_win_1040_6ff2(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut iVar3: *mut astruct_926;
    let mut uVar3: u16;

    if (param_2 == 0x8) {
        uVar3 = (param_1 >> 0x10);
        iVar3 = param_1;
        HVar2 = GetDlgItem16(0x107, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, iVar3.field6_0x6);
        uVar1 = iVar3.field147_0x94;
        EnableWindow16((uVar1 + 0x26), HVar2);
    }
    return;
}


pub unsafe fn pass1_1040_7044(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> u16

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


pub unsafe fn pass1_1040_70a0(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}


pub unsafe fn mixed_win_ui_op_1040_70b4(param_1: *mut Struct57, mut param_2: u16, struct_b_param_1: *mut StructB, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut count: u16;
    let mut hwnd: *mut u32;
    let mut iVar3: *mut astruct_792;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut paVar5: *mut Struct57;
    let mut paVar7: *mut Struct57;
    let mut struct_b_5: *mut StructB;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut puVar10: *mut u32;
    let mut puVar11: *mut u16;
    let mut DVar11: u32;
    let mut DVar12: u32;
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
    let mut uVar11: u8;
    let mut uVar12: u8;
    let mut BVar13: bool;
    let mut uVar16: u16;
    let mut pcVar17: *mut c_char;
    let mut hdc: HDC16;
    let mut local_64: u32;
    let mut uStack96: u32;
    let mut HStack92: HWND16;
//   HMENlet mut HStack90: u16;
    let mut HStack90: HMENU16;
    let mut local_58: [u8; 0x50] = [0; 0x50];
    let mut HStack8: HDC16;
    let mut paStack6: *mut Struct57;
    let mut uStack4: u16;
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut in_stack_0000ff8a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar9: u32;
    let mut fn_ptr_1: *mut *mut code;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar10 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_2, 0x34), in_stack_0000fe32,
                              in_stack_0000ff56, in_stack_0000ff5c, in_stack_0000ff60);
    paVar5 = (param_1 & 0xffff0000 | puVar10 >> 0x10);
    paVar2 = puVar10;
    uVar6 = (struct_b_param_1 >> 0x10);
    struct_b_5 = struct_b_param_1;
    struct_b_5[0x7].max_count_field_0x10 = paVar2;
    struct_b_5[0x7].field5_0xa = (puVar10 >> 0x10);
    fn_ptr_1 = (*&struct_b_5[0x7].max_count_field_0x10 + 0x4);
    (**fn_ptr_1)(0x1010, struct_b_5[0x7].max_count_field_0x10, (puVar10 >> 0x10), 0x0, struct_b_param_1);
    mem_op_1000_179c(0xa, paVar5);
    uVar4 = paVar5 | paVar2;
    paVar6 = (paVar5 & 0xffff0000 | uVar4);
    if (uVar4 == 0) {
        struct_b_5[0x7].field6_0xc = 0;
    } else {
        puVar11 = struct_1040_bf3e(CONCAT13((paVar5 >> 0x8), CONCAT12(paVar5, paVar2)),
                                   struct_b_5.lpvoid_field_0x8);
        paVar6 = (paVar6 & 0xffff0000 | puVar11 >> 0x10);
        paVar2 = puVar11;
        struct_b_5[0x7].field6_0xc = paVar2;
        struct_b_5[0x7].field7_0xe = (puVar11 >> 0x10);
    }
    // WARNING: Load size is inaccurate
    pass1_1040_bfde(struct_b_5[0x7].field6_0xc, &struct_b_5[0x7].max_count_field_0x10);
    mem_op_1000_179c(0x42, paVar6);
    uVar4 = paVar6 | paVar2;
    paVar5 = (paVar6 & 0xffff0000 | uVar4);
    if (uVar4 != 0) {
        pass1_1008_3bd6(paVar5, paVar2, paVar6, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(struct_b_5.lpvoid_field_0x8, 0x10a), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    mem_op_1000_179c(0x42, paVar5);
    uVar4 = paVar5 | paVar2;
    paVar6 = (paVar5 & 0xffff0000 | uVar4);
    if (uVar4 != 0) {
        pass1_1008_3bd6(paVar6, paVar2, paVar5, 0x1, 0xa0028, 0x0, 0x820083,
                        CONCAT22(struct_b_5.lpvoid_field_0x8, 0x10c), param_5, in_stack_0000fdd6, in_stack_0000fdda,
                        in_stack_0000ff00, in_stack_0000ff04, in_stack_0000ff08);
    }
    BVar13 = 0;
    mem_op_1000_179c(0x42, paVar6);
    uVar4 = paVar6 | paVar2;
    paVar5 = (paVar6 & 0xffff0000);
    paVar7 = (paVar5 | uVar4);
    if (uVar4 == 0) {
        paVar2 = null_mut();
    } else {
        pvVar1 = struct_b_5.lpvoid_field_0x8;
        pass1_1008_3bd6(paVar7, paVar2, paVar6, 0x1, 0xa00aa, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x107)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        paVar5 = paVar7;
    }
    uStack4 = SUB42(paVar5, 0x0);
    paStack6 = paVar2;
    enable_win_1040_9234(CONCAT13((paVar5 >> 0x8), CONCAT12(paVar5, paVar2)), BVar13);
    BVar13 = 0;
    mem_op_1000_179c(0x42, paVar5);
    uVar5 = paVar5 | paVar2;
    uVar9 = paVar5 & 0xffff0000 | uVar5;
    if (uVar5 == 0) {
        paVar2 = null_mut();
        uStack4 = 0;
    } else {
        pvVar1 = struct_b_5.lpvoid_field_0x8;
        pass1_1008_3bd6(uVar9, paVar2, paVar5, 0x1, 0xa00c2, 0x101, 0xff0100,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0x108)), param_4, in_stack_0000fdd4,
                        in_stack_0000fdd8, in_stack_0000fefe, in_stack_0000ff02, in_stack_0000ff06);
        uStack4 = uVar9;
    }
    paStack6 = paVar2;
    enable_win_1040_9234(CONCAT13((uStack4 >> 0x8), CONCAT12(uStack4, paVar2)), BVar13);
    HStack8 = GetDC16(struct_b_5.lpvoid_field_0x8);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar16 = SUB42(&DAT_1050_1050, 0x0);
    uVar11 = SUB21(local_58, 0x0);
    uVar12 = (local_58 >> 0x8);
    hdc = HStack8;
    uVar3 = str_op_1000_3da4(CONCAT22(0x1050, local_58));
    DVar11 = GetTextExtent16(uVar3, CONCAT22(uVar16, CONCAT11(uVar12, uVar11)), hdc);
    HStack90 = (HMENU16)(DVar11 >> 0x10);
    HStack92 = DVar11;
    CreateWindow16(0x0, CONCAT22(0x7cd, HINSTANCE16_1050_038c), struct_b_5.lpvoid_field_0x8, HStack90,
                   HStack92, 0xad, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT13(0x10, CONCAT12(0x50, local_58)),
                   s_static_1050_5d9a);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x50, local_58, &DAT_1050_1050);
    uVar14 = HStack8;
    uVar15 = (HStack8 >> 0x8);
    pcVar17 = local_58;
    uVar16 = SUB42(&DAT_1050_1050, 0x0);
    count = str_op_1000_3da4(CONCAT13(0x10, CONCAT12(0x50, pcVar17)));
    DVar12 = GetTextExtent16(count, CONCAT22(uVar16, pcVar17), CONCAT11(uVar15, uVar14));
    HStack90 = (HMENU16)(DVar12 >> 0x10);
    HStack92 = DVar12;
    ReleaseDC16(HStack8, struct_b_5.lpvoid_field_0x8);
    CreateWindow16(0x0, CONCAT22(0x7ce, HINSTANCE16_1050_038c), struct_b_5.lpvoid_field_0x8, HStack90,
                   HStack92, 0xc5, 0x22, 0x0, s_Rebel_1050_4ffc + 0x4, CONCAT22(0x1050, local_58),
                   s_static_1050_5da1);
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    hwnd = &local_64;
    create_window_1040_7620(struct_b_param_1, 0x1, CONCAT22(0x1050, hwnd), 0x5eb, 0xfd);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_7620(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ed, 0xfe);
    local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
    create_window_1040_7620(struct_b_param_1, 0x0, CONCAT22(0x1050, &local_64), 0x5ef, 0xff);
    SendMessage16(0x0, 0x1, 0x401, hwnd);
    uVar1 = &struct_b_5[0x7].max_count_field_0x10;
    iVar3 = uVar1;
    iVar3 = &iVar3.field_0xa;
    uVar16 = ((uVar1 & 0xffff0000) >> 0x10);
    SetWindowPos16(0x40, iVar3.field14_0x10, iVar3.field13_0xe, iVar3.field12_0xc,
                   (uVar1 & 0xffff0000 | ZEXT24(iVar3)), 0x0, struct_b_5.lpvoid_field_0x8);
    DAT_1050_0ecc = 0;
    uVar2 = &struct_b_5[0x7].max_count_field_0x10;
    fn_ptr_1 = (*&struct_b_5[0x7].max_count_field_0x10 + 0x10);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10));
    pass1_1010_2ee2(&struct_b_5[0x7].max_count_field_0x10);
    PostMessage16(0x0, 0x10a, 0x111, struct_b_5.lpvoid_field_0x8);
    return;
}


pub unsafe fn pass1_1040_741e(mut param_1: u32)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6((iVar4 + 0x94), (param_1 & 0xffff | uVar5 << 0x10));
    puVar1 = (iVar4 + 0x98);
    uVar2 = (iVar4 + 0x9a);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 1);
    }
    (iVar4 + 0x98) = 0;
    (iVar4 + 0x94) = 0;
    return;
}


pub unsafe fn pass1_1040_746c(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}


pub unsafe fn win_cleanup_op_1040_748c(param_1: *mut u8, param_2: *mut astruct_898, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut puVar1: *mut u32;
    let mut iVar2: i16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut fn_ptr_1: *mut *mut code;

    match param_5 {
        0xfa => {
            fn_ptr_1 = (*param_2.field144_0x94 + 0x18);
            (**fn_ptr_1)();
        }
        // break;
        _ => {
            pass1_1040_b54a(param_1, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), param_4,
                            param_5);
            return;
        }
        0xfd => {
            if (DAT_1050_0ecc == 0) {
                return;
            }
            DAT_1050_0ecc = 0;
        }
// TODO: goto LAB_1040_755d;
        0xfe => {
            if (DAT_1050_0ecc == 1) {
                return;
            }
            DAT_1050_0ecc = 0x1;
        }
// TODO: goto LAB_1040_755d;
        0xff => {
            if (DAT_1050_0ecc == 0x2) {
                return;
            }
            DAT_1050_0ecc = 0x2;//
// LAB_1040_755d:
            puVar1 = param_2.field144_0x94;
            fn_ptr_1 = (*param_2.field144_0x94 + 0x10);
            (**fn_ptr_1)(&PTR_LOOP_1050_1040, puVar1, (puVar1 >> 0x10));
            pass1_1010_2ee2(param_2.field144_0x94);
            PostMessage16(0x0, 0x10a, 0x111, param_2.field6_0x6);
        }
        // break;
        0x107 => {
            iVar2 = 0;
        }
// TODO: goto LAB_1040_75ba;
        0x108 => {
            iVar2 = 0x1;//
// LAB_1040_75ba:
            win_ui_op_1010_3202(param_2.field144_0x94, iVar2);
        }
        // break;
        0x10a => {
            GetClientRect16(&local_a, &DAT_1050_1050);
            puVar1 = param_2.field144_0x94;
            local_a.y += 0x3;
            local_a.x = (puVar1 + 0x1a) - 0x9;
            iStack6 += -0x3;
            iStack4 += -0x3;
            InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
            unk_destroy_win_op_1010_2fa0(param_2.field144_0x94);
            pass1_1010_32c0(param_2.field144_0x94, 0x0);
            pass1_1010_2ee2(param_2.field144_0x94);
        }
        // break;
        0x10c => {
            DestroyWindow16(param_2.field6_0x6);
        }
    }
    return;
}


pub unsafe fn pass1_1040_767e(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn post_win_msg_1040_7f1c(mut param_1: u32, mut param_2: i16) -> BOOL16

{
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x78) != 0) {
        return 0x0;
    }
    if ((iVar1 + 0x60) != param_2) {
        (iVar1 + 0x60) = param_2;
        PostMessage16(0x0, 0x0, 0x85, (iVar1 + 0x6));
    }
    return 0x1;
}


pub unsafe fn post_win_msg_1040_7f56(mut param_1: u32, param_2: *mut c_char)

{
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x10)), param_2);
    PostMessage16(0x0, 0x0, 0x85, (param_1 + 0x6));
    return;
}


pub unsafe fn menu_ui_op_1040_7f86(param_1: *mut astruct_855)

{
//   HMENlet mut HVar1: u16;
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_855;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2.field104_0x6a.is_null() == false) && (iVar2.field103_0x68 == 0)) {
        HVar1 = LoadMenu16(iVar2.field104_0x6a, HINSTANCE16_1050_038c);
        iVar2.field103_0x68 = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field103_0x68);
        iVar2.field103_0x68 = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field6_0x6);
    HVar1 = iVar2.field103_0x68;
    TrackPopupMenu16(NULL, iVar2.field6_0x6, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}


pub unsafe fn win_help_1040_800c(mut param_1: u32)

{
    let mut in_AX: u16;
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut iVar4: i16;
    let mut w_command: u16;
    let mut hwnd: HWND16;

    uVar1 = FUN_1010_830a(in_AX, in_EDX, unaff_CS, _u16_1050_14cc, 0x1f8);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x8a) == 0) {
        hwnd = (iVar2 + 0x6);
        iVar4 = 0;
        w_command = 0x3;
        iVar2 = 0;
    } else {
        hwnd = (iVar2 + 0x6);
        w_command = 0x1;
        iVar2 = (iVar2 + 0x8a);
        iVar4 = iVar2 >> 0xf;
    }
    WinHelp16(CONCAT22(iVar4, iVar2), w_command, CONCAT22(in_EDX, uVar1), hwnd);
    return;
}

pub unsafe fn pass1_1040_8054() -> u16

{
    return 0x0;
}


pub unsafe fn pass1_1040_807e(param_1: *mut astruct_395, mut param_2: u16)

{
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: u16;
    let mut paVar2: *mut astruct_394;
    let mut paVar3: *mut astruct_394;
    let mut paVar4: *mut astruct_394;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u8;
    let mut in_EDX: u32;
    let mut paVar8: *mut Struct57;
    let mut iVar9: *mut astruct_395;
    let mut uVar9: *mut astruct_395;
    let mut unaff_CS: u16;
    let mut puStack6: *mut u32;

    if (param_2 == 1) {
        pass1_1040_805a(in_EDX);
        return;
    }
    paVar2 = FUN_1010_830a(in_AX, in_EDX, unaff_CS, _u16_1050_14cc, param_2);
    uVar5 = in_EDX;
    puStack6 = CONCAT22(uVar5, paVar2);
    paVar8 = (in_EDX & 0xffff0000 | (uVar5 | paVar2));
    if ((uVar5 | paVar2) != 0) {
        ppcVar1 = (*puStack6 + 0x14);
        paVar3 = paVar2;
        (**ppcVar1)(0x1010, paVar2, uVar5);
        uVar6 = SUB42(paVar8, 0x0);
        uVar9 = (param_1 >> 0x10);
        iVar9 = param_1;
        paVar4 = paVar3;
        if (iVar9.field112_0x70.is_null() == false) {
            paVar4 = &iVar9.field112_0x70;
            uVar5 = (&iVar9.field112_0x70 + 2);
            paVar8 = (paVar8 & 0xffff0000 | (uVar5 | paVar4));
            if ((uVar5 | paVar4) != 0) {
                ppcVar1 = paVar4;
                (**ppcVar1)(0x1010, paVar4, uVar5, 1);
            }
        }
        mem_op_1000_179c(0x14, paVar8);
        puVar7 = (paVar8 | paVar4);
        if (puVar7.is_null()) {
            paVar4 = null_mut();
            puVar7 = null_mut();
        } else {
            struct_1008_4c58(paVar4);
        }
        iVar9.field112_0x70 = paVar4;
        (&iVar9.field112_0x70 + 0x2) = puVar7;
        pass1_1008_4d84(puVar7, iVar9.field112_0x70, CONCAT22(uVar6, paVar3));
        if (puStack6.is_null() == false) {
            ppcVar1 = *puStack6;
            (**ppcVar1)(0x1008, paVar2, in_EDX, 1);
        }
        return;
    }
    return;
}


pub unsafe fn unk_win_ui_op_1040_8158(param_1: u32, POparam_2: INT16, mut param_3: i16)

{
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut iVar3: i16;
    let mut uVar4: u16;

    if (param_3 == 0x2) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        if ((iVar3 + 0x76) != 0) {
            ScreenToClient16(CONCAT22(0x1050, &stack0xfffa), (iVar3 + 0x6));
            GetSystemMetrics16(SM_CYCAPTION);
            BVar2 = PtInRect16((param_1 & 0xffff0000 | ZEXT24((iVar3 + 0x82))),
                               (iVar3 + 0x82));
            if (BVar2 != 0) {
                ppcVar1 = (*param_1 + 0x14);
                (**ppcVar1)(s_tile2_bmp_1050_1538, iVar3);
            }
        }
    }
    return;
}


pub unsafe fn check_dialog_msg_1040_81b6(mut param_1: u32)

{
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut local_14: MSG16;

    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x78) = 0x1;
    loop {
        BVar1 = IsWindow16((param_1 + 0x6));
        if (BVar1 == 0) {
            return;
        }
        local_14.hwnd = &DAT_1050_1050;
        BVar1 = GetMessage16(0x0, 0x0, 0x0, &local_14);
        if (BVar1 == 0) { break; }
        IsDialogMessage16(&local_14, &DAT_1050_1050);
    }
    return;
}


pub unsafe fn win_ui_op_1040_81fe(mut param_1: u32)

{
    SetSysModalWindow((param_1 + 0x6));
    return;
}

pub unsafe fn destroy_win_1040_8212(param_1: *mut astruct_899)

{
    let mut is_window: bool;
    let mut struct_1: *mut astruct_899;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    struct_1 = param_1;
    if (struct_1.field140_0x8c != 0) {
        is_window = IsWindow16(struct_1.field140_0x8c);
        if (is_window != 0) {
            DestroyWindow16(struct_1.field140_0x8c);
            struct_1.field140_0x8c = 0;
        }
    }
    return;
}


pub unsafe fn pass1_1040_824a(mut param_1: u32, mut param_2: i16) -> u16

{
    if ((param_1 + 0x6) != param_2) {
        return 0x1;
    }
    return 0x0;
}


pub unsafe fn FUN_1040_8260() -> u16

{
    return 0x0;
}

pub unsafe fn FUN_1040_8266() -> u16

{
    return 0x0;
}


pub unsafe fn win_ui_op_1040_8718(param_1: *mut Struct37) -> *mut u8

{
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_EDX: u32;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fd88: u16;
    let mut in_stack_0000fd8a: u16;
    let mut in_stack_0000feac: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb2: u16;
    let mut in_stack_0000feb4: u16;
    let mut in_stack_0000feb6: u16;
    let mut in_stack_0000feb8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut in_stack_0000fee0: u16;
    let mut in_stack_0000fee2: u16;
    let mut local_104: [i16; 0x80] = [0; 0x80];
    let mut uStack4: u16;
    let mut uVar8: u16;
    let mut paVar12: *mut Struct37;
    let mut uVar12: *mut Struct37;

    uVar5 = (in_EDX >> 0x10);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    paVar12 = param_1;
    uVar12 = (param_1 >> 0x10);
    dialog_ui_fn_1040_78e2(param_1);
    PTR_LOOP_1050_5df6 = (&paVar12.field1_0x4 + 2);
    if (&paVar12.field_0x94 != 0) {
        unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&paVar12.field_0x10)), *&paVar12.field_0x94);
    }
    get_sys_metrics_1040_8c66(param_1);
    uStack4 = paVar12.field138_0x98 & 0xf;
    if (uStack4 == 1) {
        iVar3 = &paVar12[0x1].field_0x8 - 0xc4;
        paVar6 = CONCAT22(uVar5, iVar3 >> 0xf);
        paVar12[0x1].field_0xc = iVar3 / 0x2;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              &DAT_1050_1050);
        uVar2 = &paVar12[0x1].field_0xc;
        create_window_1040_8bea(paVar12, uVar12, 0x1, 0x1, uVar2, (uVar2 >> 0x10), CONCAT22(0x1050, local_104));
        piVar1 = &paVar12[0x1].field_0xc;
        *piVar1 = *piVar1 + 0x6c;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              &DAT_1050_1050);
        uVar2 = &paVar12[0x1].field_0xc;
        uVar10 = uVar2;
        uVar11 = (uVar2 >> 0x10);
        uVar9 = 0x2;
    } else {
        if (uStack4 != 0x4) {
            iVar3 = &paVar12[0x1].field_0x8 - 0x58;
            paVar6 = CONCAT22(uVar5, iVar3 >> 0xf);
            paVar12[0x1].field_0xc = iVar3 / 0x2;
            load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                                  &DAT_1050_1050);
            uVar2 = &paVar12[0x1].field_0xc;
            uVar10 = uVar2;
            uVar11 = (uVar2 >> 0x10);
            uVar5 = 0x1;
            uVar9 = 0x1;
            // TODO: goto LAB_1040_88a5;
        }
        iVar3 = &paVar12[0x1].field_0x8 - 0xc4;
        paVar6 = CONCAT22(uVar5, iVar3 >> 0xf);
        paVar12[0x1].field_0xc = iVar3 / 0x2;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              &DAT_1050_1050);
        uVar2 = &paVar12[0x1].field_0xc;
        create_window_1040_8bea(paVar12, uVar12, 0x1, 0x6, uVar2, (uVar2 >> 0x10), CONCAT22(0x1050, local_104));
        piVar1 = &paVar12[0x1].field_0xc;
        *piVar1 = *piVar1 + 0x6c;
        load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0xff, local_104,
                              &DAT_1050_1050);
        uVar2 = &paVar12[0x1].field_0xc;
        uVar10 = uVar2;
        uVar11 = (uVar2 >> 0x10);
        uVar9 = 0x7;
    }
    uVar5 = 0;//
// LAB_1040_88a5:
    create_window_1040_8bea(paVar12, uVar12, uVar5, uVar9, uVar10, uVar11, CONCAT22(0x1050, local_104));
    puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000fee0, 0x48), in_stack_0000fd88,
                             in_stack_0000feac, in_stack_0000feb2, in_stack_0000feb6);
    uVar5 = (puVar7 >> 0x10);
    local_104[0] = (puVar7 + 0xa);
    uStack4 = (puVar7 + 0xc);
    iVar3 = uStack4 - &paVar12[0x1].field_0xa;
    paVar6 = (paVar6 & 0xffff0000 | (iVar3 >> 0xf));
    SetWindowPos16(0x40, &paVar12[0x1].field_0xa, &paVar12[0x1].field_0x8, iVar3 / 0x2,
                   (local_104[0] - &paVar12[0x1].field_0x8) / 0x2, 0x0, (&paVar12.field1_0x4 + 0x2),
    );
    PTR_LOOP_1050_5df4 = (&PTR_LOOP_1050_0000 + 1);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    destroy_win_1040_8b7e(param_1);
    PTR_LOOP_1050_5df6 = null_mut();
    if (&paVar12[0x1].field_0x10 != 0) {
        puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000fee2, 0x37), in_stack_0000fd8a,
                                 in_stack_0000feae, in_stack_0000feb4, in_stack_0000feb8);
        uVar4 = pass1_1008_ab54(puVar7);
        if (uVar4 != 0) {
            PostMessage16(0x0, 0xfc, 0x111, HWND16_1050_0396);
        }
    }
    return PTR_LOOP_1050_5df8;
}


pub unsafe fn pass1_1040_8978(mut param_1: u16, mut param_2: u16, param_3: *mut u32, mut param_4: u16)

{
    let mut ppcVar1: *mut *mut code;

    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    win_1008_5c5c(param_1, param_2, _u16_1050_02a0, param_4);
    ppcVar1 = (*param_3 + 0x74);
    (**ppcVar1)(0x1008, param_3);
    return;
}
