use crate::block_1018::block_1018_5000::{pass1_1018_50ac, pass1_1018_5714, pass1_1018_58b6, pass1_1018_5b06, pass1_1018_5cc8};
use crate::block_1018::block_1018_6000::{draw_line_1018_6444, draw_op_1018_6544, pass1_1018_642e, pass1_1018_6630, pass1_1018_673c, pass1_1018_6924, pass1_1018_69ac};

pub unsafe fn pass1_1018_567c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        pass1_1000_093a(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_56a8(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1018_50ac(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_580a(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1018_5714(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn invalidate_rect_1018_58e2(param_1: *mut astruct_58, mut param_2: i16) {
    let mut piVar1: *mut i16;
    let mut iVar2: *mut astruct_58;
    let mut uVar2: u16;

    if (param_2 == 0x105) {
        uVar2 = (param_1 >> 0x10);
        iVar2 = param_1;
        piVar1 = &iVar2.field245_0xf6;
        *piVar1 = *piVar1 + 1;
        if (u16_1050_4240 <= iVar2.field245_0xf6) {
            PostMessage16(0x0, 0xca, 0x111, HWND16_1050_0396);
            return;
        }
        iVar2.field234_0xea = 0;
        InvalidateRect16(0x0, NULL, 0x0);
    }
    return;
}


pub unsafe fn pass1_1018_5932(mut param_1: u16, mut param_2: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;

    uVar4 = (param_2 >> 0x10);
    uVar3 = param_2;
    uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
    if (uVar2 != 0) {
        ppcVar1 = ((uVar3 + 0xee) + 0x8);
        uVar5 = (**ppcVar1)();
        param_1 = (uVar5 >> 0x10);
        uVar2 = uVar5;
    }
    if ((uVar3 + 0xea) == 0) {
        (uVar3 + 0xea) = 0x1;
        uVar5 = pass1_1038_af40(
            uVar3,
            param_1,
            _PTR_LOOP_1050_5b7c,
            (uVar3 + 0x8),
            ((uVar3 + 0xf6) * 0x2 + 0x4238),
        );
        uVar2 = uVar5;
    }
    return uVar2;
}


pub unsafe fn win_1018_598c(
    mut param_1: u16,
    param_2: *mut Struct57,
    struct_param_1: *mut StructA,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u16;
    let struct_1: *mut StructA;
    let mut uVar3: u16;
    let mut in_stack_0000fe68: u16;
    let mut in_stack_0000ff8c: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff96: u16;
    let mut uVar2: u32;

    create_window_ex_1008_9760(struct_param_1);
    uVar3 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    get_dc_1018_4db0(&struct_1[0x1].field20_0x26, struct_1.field4_0x8);
    mem_op_1000_179c(0x2a, param_2);
    uVar1 = param_2 | param_1;
    uVar2 = param_2 & 0xffff0000 | uVar1;
    if (uVar1 != 0) {
        pass1_1018_5b06(
            uVar2,
            CONCAT22(param_2, param_1),
            struct_1.field4_0x8,
            param_4,
            param_5,
            in_stack_0000fe68,
            in_stack_0000ff8c,
            in_stack_0000ff92,
            in_stack_0000ff96,
        );
        struct_1[0x1].field18_0x22 = param_1;
        struct_1[0x1].field19_0x24 = uVar2;
        return;
    }
    struct_1[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn FUN_1018_59f0(mut param_1: u16, mut param_2: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    puVar1 = (iVar4 + 0xee);
    uVar2 = (iVar4 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0xee) = 0;
    destroy_win_1008_628e(param_2 & 0xffff | uVar5 << 0x10);
    return;
}


pub unsafe fn pass1_1018_5a2e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_58b6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn FUN_1018_5a3c(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_58b6(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn invalidate_rect_1018_5d32(mut param_1: u32, mut param_2: i16) {
    let mut hwnd: HWND16;

    hwnd = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0x2) {
        return;
    }
    InvalidateRect16(0x0, (param_1 + 0x22), hwnd);
    return;
}

pub unsafe fn misc_draw_op_1018_5d6c(param_1: *mut astruct_839) {
    let mut pa_var1: *mut astruct_76;
    let mut struct_4: *mut astruct_839;
    let mut u_var5: u16;
    let mut pa_var2: *mut astruct_76;
    let mut local_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut pu_var1: *mut u32;
    let mut u_var4: *mut astruct_134;
    let mut fn_ptr_1: *mut *mut code;

    u_var5 = (param_1 >> 0x10);
    struct_4 = param_1;
    BeginPaint16(CONCAT22(0x1050, &local_22), struct_4.field4_0x4);
    u_var4 = struct_4.pstruct134_0x14;
    pa_var1 = (u_var4 + 0xa);
    pa_var2 = pass1_1008_9f48(struct_4.pstruct134_0x14);
    pass1_1008_5236(struct_4.field20_0x18);
    pass1_1008_4480(
        pa_var1,
        (param_1 & 0xffff0000 | ZEXT24(struct_4 + 1)),
        pa_var2,
    );
    fn_ptr_1 = (pa_var1 + 0x4);
    (**fn_ptr_1)(
        0x1008,
        pa_var1,
        (pa_var1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | ZEXT24(&struct_4.field_0xa),
    );
    EndPaint16(CONCAT22(0x1050, &local_22), struct_4.field4_0x4);
    return;
}


pub unsafe fn pass1_1018_5df4(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1018_5cc8(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_5e86(param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x6c);
    (**ppcVar1)();
    return;
}

pub unsafe fn win_ui_op_1018_5e9a(mut param_1: u16, structb_param_1: *mut StructB) {
    let mut ppcVar1: *mut *mut c_char;
    let mut pvVar2: LPVOID = null_mut();
    let mut IVar3: i16;
    let mut ppaVar4: *mut *mut astruct_92 = null_mut();
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut uVar9: u32;
    let mut structb_9: *mut StructB;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut puVar13: *mut u32;
    let mut in_stack_0000fe5a: u16;
    let mut in_stack_0000ff7e: u16;
    let mut in_stack_0000ff84: u16;
    let mut in_stack_0000ff88: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_28: *mut astruct_92;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut local_e: RECT16;
    let mut iStack8: i16;
    let mut pIStack6: *mut INT16 = null_mut();
    let mut paVar8: *mut Struct57;

    paVar7 = CONCAT22(in_register_0000000a, param_1);
    dialog_ui_fn_1040_78e2(structb_param_1);
    puVar13 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffb2, 0x39),
        in_stack_0000fe5a,
        in_stack_0000ff7e,
        in_stack_0000ff84,
        in_stack_0000ff88,
    );
    paVar7 = (paVar7 & 0xffff0000 | puVar13 >> 0x10);
    pvVar2 = puVar13;
    uVar11 = (structb_param_1 >> 0x10);
    structb_9 = structb_param_1;
    structb_9[0x7].field1_0x2 = pvVar2;
    structb_9[0x7].hwnd_0x6 = (puVar13 >> 0x10);
    mem_op_1000_179c(0x12, paVar7);
    puVar5 = (paVar7 | pvVar2);
    paVar8 = (paVar7 & 0xffff0000 | ZEXT24(puVar5));
    if (puVar5.is_null()) {
        structb_9[0x7].lpvoid_field_0x8 = 0;
    } else {
        pass1_1018_6198(
            puVar5,
            CONCAT22(paVar7, pvVar2),
            structb_param_1,
            structb_9.lpvoid_field_0x8,
        );
        structb_9[0x7].lpvoid_field_0x8 = pvVar2;
        structb_9[0x7].max_count_field_0x10 = paVar8;
    }
    uVar9 = &structb_9[0x7].field1_0x2;
    pIStack6 = (uVar9 & 0xffff0000 | (uVar9 + 0xa));
    GetClientRect16(&local_e, &DAT_1050_1050);
    IVar3 = GetSystemMetrics16(SM_CYCAPTION);
    (pIStack6 + 0x6) = IVar3 + iStack8;
    puVar13 = mixed_1010_20ba(
        paVar8,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffb2, 0x48),
        in_stack_0000fe5a,
        in_stack_0000ff7e,
        in_stack_0000ff84,
        in_stack_0000ff88,
    );
    uStack20 = (puVar13 >> 0x10);
    iStack22 = puVar13;
    iStack16 = (iStack22 + 0xa);
    iStack18 = (iStack22 + 0xc);
    uVar12 = (pIStack6 >> 0x10);
    (pIStack6 + 0x2) = (iStack18 - (pIStack6 + 0x6)) / 0x2;
    uVar9 = (iStack16 >> 0xf);
    *pIStack6 = iStack16 / 0x2 + 0x3;
    pass1_1028_dc52(CONCAT22(0x1050, &local_28), 0x1, 0x0, 0x100);
    loop {
        uVar6 = uVar9;
        ppaVar4 = &local_28;
        pass1_1028_e4ec(CONCAT22(0x1050, ppaVar4));
        uVar9 = (uVar6 | ppaVar4);
        if ((uVar6 | ppaVar4) == 0) {
            break;
        }
        ppcVar1 = (ppaVar4 + 0x8);
        if (ppcVar1.is_null() == false) {
            pass1_1000_3cea(
                structb_param_1 & 0xffff0000 | ZEXT24(&structb_9.field8_0x10),
                *ppcVar1,
            );
        }
    }
    uVar12 = (pIStack6 >> 0x10);
    iVar10 = pIStack6;
    SetWindowPos16(
        0x44,
        (iVar10 + 0x6),
        (iVar10 + 0x4),
        (iVar10 + 0x2),
        *pIStack6,
        0x0,
        structb_9.lpvoid_field_0x8,
    );
    return;
}

pub unsafe fn pass1_1018_5ffa(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0x92);
    uVar2 = (iVar4 + 0x94);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0x92) = 0;
    pass1_1010_1dda((iVar4 + 0x8e));
    (iVar4 + 0x8e) = 0;
    return;
}

pub unsafe fn fn_xyz() {
    let mut piVar2: *mut i16;
    let mut uVar3_01: u32;
    let mut uVar6: u16;
    let mut pHVar14: *mut HDC16;
    let mut iVar7: i16;
    let mut iVar11: i16;
    let mut handle: HPEN16;
    let mut hgdiobj16_00: HBRUSH16;
    let mut uVar7: u16;
    let mut obj: HPALETTE16;
    let mut puVar7: *mut u8;
    let mut hgdiobj16_var7: HGDIOBJ16;
    let mut in_EDX: u32;
    let mut paVar25: *mut Struct57;
    let mut uVar27: u16;
    let mut uVar26: u32;
    let mut struct742_var8: *mut astruct_742;
    let mut iVar9: *mut astruct_755;
    let mut iVar10: *mut astruct_756;
    let mut puVar11: *mut astruct_734;
    let mut iVar12: i16;
    let mut uVar12: u16;
    let mut uVar14: u16;
    let mut uVar13: u16;
    let mut uVar28: u32;
    let mut piVar16: *mut i16;
    let mut iVar29: i16;
    let mut local_38: [u8; 0x6] = [0; 0x6];
    let mut local_32: u16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u32;
    let mut local_24: HDC16;
    let mut paintstruct16_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2_01: u32;
    let mut piVar1: *mut i16;
    let mut uVar15: u32;
    let mut uVar5: u32;
    let mut uVar8: u32;
    let mut uVar9: u32;
    let mut uVar10: u32;
    let mut iVar13: *mut astruct_758;
    let mut uVar11: u32;
    let mut uVar2: u32;
    let mut puVar4: *mut u32;
    let mut uVar4: u32;
    let mut uVar16: u8;
    let mut uVar17: u8;
    let mut uVar18: u16;
    let mut iVar16: *mut astruct_757;
    let mut uVar19: u16;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar3: u32;
    let mut uVar2_00: u32;
    let mut uVar3_00: u32;
    let mut uVar23: u32;
    let mut uVar24: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar22 = (in_EDX >> 0x10);
    puVar11 = &stack0xfffe;
    uVar12 = (param_1 >> 0x10);
    struct742_var8 = param_1;
    local_24 = BeginPaint16(
        CONCAT22(0x1050, paintstruct16_22),
        struct742_var8.field4_0x4,
    );
    puStack40 = pass1_1010_4c2c(struct742_var8.field5_0x6);
    pHVar14 = &local_24;
    fn_ptr_1 = (*puStack40 + 0x8);
    (**fn_ptr_1)(
        0x1010,
        puStack40,
        (puStack40 >> 0x10),
        pHVar14,
        &DAT_1050_1050,
    );
    struct742_var8.field12_0x10 = pHVar14;
    uVar2 = struct742_var8.field5_0x6;
    uStack42 = (uVar2 + 0x30);
    uVar28 = struct742_var8.field5_0x6;
    uStack46 = *(uVar28 + 0x12);
    uStack48 = 0x14;
    local_32 = 0;
    uVar13 = 0x1008;
    pass1_1008_3e38(CONCAT22(0x1050, local_38));
    paVar25 = (uVar22 << 0x10);
    while (
        uVar27 = (paVar25 >> 0x10),
        &puVar11[-0x6].field_0x4 < (puVar11 - 0x4),
    ) {
        iVar9 = (&puVar11[-0x6].field_0x4 * 0x4);
        uVar8 = puVar11[-0x5].field6_0x6;
        uVar28 = pass1_1008_4772((iVar9 + uVar8));
        puVar7 = (uVar28 >> 0x10);
        paVar25 = CONCAT22(uVar27, puVar7);
        puVar11[-0x7].field_0x2 = uVar28;
        puVar11[-0x7].field_0x4 = puVar7;
        uVar3 = puVar11.field6_0x6;
        pass1_1018_642e(
            uVar3,
            (uVar3 >> 0x10),
            CONCAT13(0x10, CONCAT12(0x50, &puVar11[-0x5].field_0x2)),
            (uVar28 + 0x8),
        );
        uVar9 = &puVar11[-0x5].field_0x2;
        pass1_1008_3e76(
            CONCAT22(0x1050, &puVar11[-0x6].field6_0x6),
            0x0,
            uVar9,
            (uVar9 >> 0x10),
        );
        uVar23 = puVar11[-0x5].field6_0x6;
        pass1_1008_4480(
            &puVar11[-0x4].field_0x2,
            CONCAT22(0x1050, &puVar11[-0x6].field6_0x6),
            (iVar9 + uVar23),
        );
        iVar29 = &puVar11[-0x6].field_0x4;
        uVar10 = &puVar11[-0x5].field_0x2;
        uVar19 = uVar10;
        uVar20 = (uVar10 >> 0x10);
        uVar21 = (uVar10 >> 0x18);
        uVar11 = &puVar11[-0x7].field_0x2;
        uVar14 = (uVar11 >> 0x10);
        iVar10 = uVar11;
        iVar7 = iVar10.field4_0x4 + &puVar11[-0x5].field_0x4;
        iVar11 = iVar10.field7_0x8 + &puVar11[-0x5].field_0x2;
        uVar26 = puVar11.field6_0x6;
        uVar2_00 = (uVar26 + 0x6);
        iVar16 = uVar2_00;
        uVar18 = (uVar2_00 >> 0x10);
        uVar16 = '\x0b';
        uVar17 = '\x10';
        if (&iVar16.field_0x1a == 0) {
            uVar6 = iVar16.field47_0x30 << 0x3;
            mem_op_1000_179c(uVar6, paVar25);
            iVar16.field_0x1a = uVar6;
            iVar16.field28_0x1c = paVar25;
        }
        uVar3_00 = &iVar16.field_0x1a;
        iVar13 = (iVar29 * 0x8);
        (iVar13 + uVar3_00) = CONCAT11(uVar21, uVar20);
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x2) = uVar19;
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x4) = iVar7;
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x6) = iVar11;
        uVar13 = CONCAT11(uVar17, uVar16);
        uVar2_01 = &puVar11[-0x7].field_0x2;
        piVar2 = &puVar11[-0x5].field_0x4;
        *piVar2 = *piVar2 + (-(&puVar11[-0x6].field_0x4 == 0) & 0x5) + 0x14 + (uVar2_01 + 0x4);
        piVar2 = &puVar11[-0x6].field_0x4;
        *piVar2 = *piVar2 + 1;
    }
    puVar4 = &puVar11[-0x4].field_0x2;
    fn_ptr_1 = (*puVar4 + 0x4);
    (**fn_ptr_1)(
        uVar13,
        puVar4,
        (puVar4 >> 0x10),
        0x0,
        0x0,
        puVar11 - 0x22,
        &DAT_1050_1050,
    );
    handle = CreatePen16(0x1000025, 0x1, 0x0);
    &puVar11[-0x6].field_0x2 = handle;
    hgdiobj16_var7 = SelectObject16(handle, *&puVar11[-0x4].field6_0x6);
    (puVar11 - 0x6) = hgdiobj16_var7;
    hgdiobj16_00 = CreateSolidBrush16(0x1000025);
    (&puVar11[-0x7].field6_0x6 + 0x2) = hgdiobj16_00;
    hgdiobj16_var7 = SelectObject16(hgdiobj16_00, *&puVar11[-0x4].field6_0x6);
    puVar11[-0x7].field6_0x6 = hgdiobj16_var7;
    draw_line_1018_6444(puVar11.field6_0x6, *&puVar11[-0x4].field6_0x6);
    uVar4 = puVar11.field6_0x6;
    piVar16 = pass1_1010_4dc8((uVar4 + 0x6));
    uVar26 = piVar16 >> 0x10;
    uVar24 = piVar16 & 0xffff;
    draw_op_1018_6544(
        puVar11.field6_0x6,
        (piVar16 & 0xff000000 | CONCAT12((piVar16 >> 0x10), uVar24)),
    );
    pass1_1018_6630((uVar26 & 0xffff | uVar24 << 0x10), puVar11.field6_0x6);
    uVar5 = puVar11.field6_0x6;
    obj = SelectPalette16(0x0, (uVar5 + 0x10), *&puVar11[-0x4].field6_0x6);
    DeleteObject16(obj);
    hgdiobj16_var7 = SelectObject16((puVar11 - 0x6), *&puVar11[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var7);
    hgdiobj16_var7 = SelectObject16(&puVar11[-0x7].field6_0x6, *&puVar11[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var7);
    uVar15 = puVar11.field6_0x6;
    EndPaint16(
        CONCAT22(0x1050, (&puVar11[-0x4].field6_0x6 + 0x2)),
        (uVar15 + 0x4),
    );
    return;
}

pub unsafe fn pass1_1018_669a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_620c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_6768(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;

    uVar4 = (param_3 >> 0x10);
    uVar3 = param_3;
    uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
    if (uVar2 != 0) {
        ppcVar1 = ((uVar3 + 0xee) + 0x8);
        uVar5 = (**ppcVar1)();
        param_1 = (uVar5 >> 0x10);
        uVar2 = uVar5;
    }
    if ((uVar3 + 0xea) == 0) {
        (uVar3 + 0xea) = 0x1;
        uVar5 = pass1_1038_af40(uVar3, param_1, _PTR_LOOP_1050_5b7c, (uVar3 + 0x8), 0x16);
        uVar2 = uVar5;
    }
    return uVar2;
}


pub unsafe fn window_op_1018_67b6(param_1: *mut StructD, param_2: *mut StructA) {
    let mut paVar1: *mut astruct_666;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let struct_1: *mut StructA;
    let mut uVar2: u16;

    paVar1 = (param_1 >> 0x10);
    paVar4 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    create_window_ex_1008_9760(param_2);
    uVar2 = (param_2 >> 0x10);
    struct_1 = param_2;
    get_dc_1018_4db0(&struct_1[0x1].field20_0x26, struct_1.field4_0x8);
    mem_op_1000_179c(0x18, paVar4);
    uVar3 = paVar4 | paVar1;
    if (uVar3 != 0) {
        pass1_1018_6924(uVar3, CONCAT22(paVar4, paVar1), struct_1.field4_0x8);
        struct_1[0x1].field18_0x22 = paVar1;
        struct_1[0x1].field19_0x24 = uVar3;
        return;
    }
    struct_1[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn pass1_1018_681a(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1);
    return;
}

pub unsafe fn pass1_1018_684c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_673c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_685a(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_673c(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn pass1_1018_69dc(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_69ac(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_6a76() {
    return;
}

pub unsafe fn pass1_1018_6c1e(param_1: *mut StructD, param_2: u8) {
    let mut uVar1: *mut StructD;
    let mut uVar2: u16;

    uVar1 = param_1;
    uVar1 = &uVar1.field192_0xd2;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x380a;
    uVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    uVar1.address_offset_field_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_6048(mut param_1: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x92) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

pub unsafe fn FUN_1018_60ea() {
    return;
}

pub unsafe fn FUN_1018_60ee() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1018_60f4() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1018_60fa() {
    return;
}

pub unsafe fn FUN_1018_60fe() {
    return;
}

pub unsafe fn pass1_1018_6102(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_5e5a(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn unk_draw_op_1018_623e(param_1: *mut astruct_742) {
    todo!()
}

pub unsafe fn FUN_1018_742e(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut astruct_28,
) {
    mixed_draw_op_1018_6a7a(param_2, param_3, param_5);
    if (PTR_LOOP_1050_4254.is_null()) {
        win_1008_5c5c(param_1, param_2, _u16_1050_02a0, 0x1e9);
        if (param_1 != 0) {
            PTR_LOOP_1050_4254 = (&PTR_LOOP_1050_0000 + 1);
        }
    }
    return;
}

pub unsafe fn pass1_1018_7da6(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7dee(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7e36(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7e7e(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7ec6(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7f0e(param_1: *mut StructD, param_2: u8) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    iVar1 = param_1;
    iVar1 = &iVar1.field192_0xd2;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x380a;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}


pub unsafe fn pass1_1018_7f56(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7f9e(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7fe6(param_1: *mut StructD, param_2: u8) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    iVar1 = param_1;
    iVar1 = &iVar1.field192_0xd2;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x380a;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_802e(param_1: *mut u16, param_2: u8)

{
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}
