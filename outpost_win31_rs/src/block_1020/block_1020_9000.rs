pub unsafe fn pass1_1020_9068(mut param_1: u32, param_2: *mut u32, mut param_3: u32) {
    let mut iVar1: i16;
    let mut paVar2: *mut astruct_76;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut paVar6: *mut astruct_76;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut iVar10: i16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut iStack10: i16;
    let mut uVar9: u32;

    uVar12 = (param_2 >> 0x10);
    iVar10 = param_2;
    uVar4 = (iVar10 + 0x16);
    paVar2 = (uVar4 + 0xa);
    paVar6 = paVar2;
    pass1_1018_280c((iVar10 + 0x16));
    (iVar10 + 0xaa) = paVar6;
    (iVar10 + 0xac) = param_1;
    uVar5 = param_1 | (iVar10 + 0xaa);
    if (uVar5 == 0) {
        pass1_1018_2862((iVar10 + 0x16));
        (iVar10 + 0xaa) = uVar5;
        (iVar10 + 0xac) = param_1;
    }
    if (((iVar10 + 0xac) | (iVar10 + 0xaa)) != 0) {
        pass1_1020_915a(param_1, (param_2 & 0xffff | uVar12 << 0x10), param_3);
        pass1_1008_4480(
            paVar2,
            (param_2 & 0xffff0000 | (iVar10 + 0xae)),
            (iVar10 + 0xb4),
        );
        ppcVar3 = (*param_2 + 0x10);
        (**ppcVar3)();
        uVar4 = (iVar10 + 0xaa);
        iVar1 = (uVar4 + 0xa);
        for iStack10 in 0..iVar1 {
            uVar7 = iStack10;
            empty_1008_8fc4();
            uVar5 = uVar7;
            uVar8 = param_1 | uVar5;
            uVar9 = param_1 & 0xffff0000 | uVar8;
            if (uVar8 != 0) {
                pass1_1008_8c4e(uVar7 & 0xffff | param_1 << 0x10, paVar2, uVar9);
                uVar4 = (iVar10 + 0xc);
                uVar13 = (uVar4 >> 0x10);
                iVar11 = uVar4;
                (iVar11 + iStack10 * 0x4) = uVar5;
                (iVar11 + iStack10 * 0x4 + 0x2) = uVar9;
            }
            param_1 = uVar9;
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_915a(param_1: *mut u8, param_2: *mut astruct_669, param_3: *mut *mut u8) {
    let mut iVar1: i16;
    let mut iVar3: i16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut iVar2: *mut astruct_669;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    param_3 = CONCAT22(param_3, 0x2f);
    puVar7 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        param_3,
        in_stack_0000fe9c,
        in_stack_0000ffc0,
        in_stack_0000ffc6,
        in_stack_0000ffca,
    );
    uVar5 = paVar4 & 0xffff0000 | puVar7 >> 0x10;
    iVar1 = (puVar7 + 0x1e);
    uVar6 = (param_2 >> 0x10);
    iVar2 = param_2;
    if (iVar2.field182_0xb8 != iVar1) {
        param_3 = 0x1ce;
        iVar3 = iVar1 + -0x1;
        if (iVar3 == 0) {
            param_3 = 0x1cf;
        } else {
            iVar3 = iVar1 + -0x2;
            if (iVar3 == 0) {
                param_3 = 0x1d0;
            } else {
                iVar3 = iVar1 + -0x3;
                if (iVar3 == 0) {
                    param_3 = 0x1d1;
                } else {
                    iVar3 = iVar1 + -0x4;
                    if (iVar3 == 0) {
                        param_3 = 0x1d2;
                    }
                }
            }
        }
        iVar3 = FUN_1010_830a(iVar3, uVar5, 0x1010, _u16_1050_14cc, param_3);
        iVar2.field180_0xb4 = iVar3;
        iVar2.field181_0xb6 = uVar5;
        iVar2.field182_0xb8 = iVar1;
    }
    return;
}

pub unsafe fn pass1_1020_91de(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8f74(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn set_struct_op_1020_921c(
    mut param_1: u16,
    pstructa_param_2: *mut StructA,
    mut param_3: u16,
    param_4: *mut *mut u8,
) {
    let mut uVar1: u16;
    let mut HVar2: HDC16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let iVar3: *mut StructA;
    let uVar3: *mut StructA;
    let mut pUVar3: *mut u16;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe2: u32;
    let mut uVar2: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    uVar3 = (pstructa_param_2 >> 0x10);
    iVar3 = pstructa_param_2;
    pstructa_param_2.field0_0x0 = 0x389a;
    iVar3.field1_0x2 = 0x1008;
    pstructa_param_2.field0_0x0 = 0x3aa8;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field2_0x4 = param_3;
    pstructa_param_2.field0_0x0 = 0x3ab0;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field3_0x6 = 0;
    iVar3.field5_0xa = 0;
    iVar3.field6_0xc = 0;
    iVar3.field7_0xe = 0;
    iVar3.field8_0x10 = 0;
    iVar3.field9_0x12 = 0;
    pstructa_param_2.field0_0x0 = 0x96c8;
    iVar3.field1_0x2 = 0x1020;
    HVar2 = GetDC16(iVar3.field2_0x4);
    iVar3.field5_0xa = HVar2;
    param_4 = CONCAT22(param_4, 0x48);
    pUVar3 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        param_4,
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    uVar1 = (pUVar3 >> 0x10);
    iVar3.field6_0xc = (pUVar3 + 0xa);
    iVar3.field7_0xe = (pUVar3 + 0xc);
    return;
}
pub unsafe fn palette_op_1020_92c4(struct_param_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct_1: *mut StructD;
    let mut uVar2: *mut StructD;

    uVar2 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    struct_param_1.address_offset_field_0x0 = 0x96c8;
    struct_1.address_offset_field_0x2 = 0x1020;
    if (struct_1.field11_0x12 != 0) {
        obj = SelectPalette16(0x0, struct_1.field11_0x12, struct_1.field6_0xa);
        DeleteObject16(obj);
    }
    struct_param_1.address_offset_field_0x0 = 0x3ab0;
    struct_1.address_offset_field_0x2 = 0x1008;
    struct_param_1.address_offset_field_0x0 = 0x389a;
    struct_1.address_offset_field_0x2 = 0x1008;
    return;
}
pub unsafe fn mix_draw_op_1020_9312(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut local_22: [u8; 0x20] = [0; 0x20];

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    uVar3 = (iVar4 + 0x6);
    puVar1 = (uVar3 + 0xa);
    ppcVar2 = (*puVar1 + 0x4);
    (**ppcVar2)(
        s_tile2_bmp_1050_1538,
        puVar1,
        (puVar1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | (iVar4 + 0xa),
    );
    EndPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    return;
}

// WARNING: Unable to use type for symbol uVar4
// WARNING: Could not reconcile some variable overlaps
pub unsafe fn draw_op_1020_9364(param_1: *mut StructA) {
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut brush_handle_var6: HBRUSH16;
    let local_struct_1: *mut StructA;
    let mut var7: u16;
    let mut uVar7: u16;
    let mut uVar3: u16;
    let mut uVar8: u16;
    let mut HVar3: HDC16;
    let mut iStack62: i16;
    let mut uStack58: *mut astruct_737;
    let mut POpoint16_38: INT16;
    let mut hgdiobj16_var_52: HGDIOBJ16;
    let mut HStack50: HPEN16;
    let mut hdc16_var_48: HDC16;
    let mut uStack46: u32;
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut uStack34: u32;
    let mut puStack30: *mut u32;
    let mut puStack26: *mut u16;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut rect16_var_12: RECT16;
    let mut uStack14: u32;
    let mut rect16_a: RECT16;
    let mut x_var_6: u32;
    let mut uVar2: u16;
    let mut iVar2: i16;
    let mut piVar2: *mut i16;
    let mut uVar4: u32;
    let mut iVar4: i16;
    let mut uVar10: u8;
    let mut uVar11: u8;

    var7 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    GetClientRect16(&rect16_a, &DAT_1050_1050);
    rect16_var_12 = rect16_a;
    uStack14 = x_var_6;
    uStack20 = DAT_1050_4216;
    iStack22 = DAT_1050_422c;
    puStack26 = PTR_u16_1050_4172_1050_4212;
    puStack30 = PTR_u16_1050_41b2_1050_4218;
    uStack34 = PTR_u16_1050_41da_1050_421c;
    uStack38 = PTR_u16_1050_4202_1050_4220;
    uStack42 = PTR_u16_1050_419a_1050_4224;
    uStack46 = PTR_u16_1050_41aa_1050_4228;
    uVar4 = &local_struct_1.field3_0x6;
    hdc16_var_48 = *(uVar4 + 0x12);
    uStack58 = (&u16_1050_0008 + 1);
    loop {
        HVar3 = hdc16_var_48;
        HStack50 = CreatePen16(*(uStack58 * 0x4 + uStack34), 0x0, 0x0);
        hgdiobj16_var_52 = SelectObject16(HStack50, HVar3);
        MoveToEx16(
            &point16_38,
            &DAT_1050_1050,
            (uStack58 * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16((puStack26 + uStack58 * 0x2), x_var_6, hdc16_var_48);
        iVar4 = (uStack20 - uStack58) * 0x2;
        MoveToEx16(&point16_38, &DAT_1050_1050, (iVar4 + puStack26), rect16_a.x);
        LineTo16((puStack26 + iVar4), x_var_6, hdc16_var_48);
        SelectObject16(hgdiobj16_var_52, hdc16_var_48);
        DeleteObject16(HStack50);
        uStack58 = (&uStack58[-0x1].field0_0x0 + 1);
        if uStack58 >= 0x8000 {
            break;
        }
    }
    brush_handle_var6 = CreateSolidBrush16(0x2000000);
    uVar7 = (puStack26 >> 0x10);
    rect16_a = CONCAT22((puStack26 + 0x12) + 0x1, rect16_a.x);
    uVar2 = (puStack26 + 0x14);
    uStack14 = uStack14 & 0xffff | uVar2 << 0x10;
    x_var_6 = CONCAT22(uVar2, x_var_6);
    FillRect16(brush_handle_var6, &rect16_a, &DAT_1050_1050);
    DeleteObject16(brush_handle_var6);
    iStack62 = 0x8;
    puVar2 = &PTR_LOOP_1050_0000;
    while (uStack58 = (puVar2 + 1), uStack58 < 0xa) {
        brush_handle_var6 = CreateSolidBrush16(*(puStack30 + iStack62 * 0x4 + 0x4));
        x_var_6 = x_var_6 & 0xffff | (rect16_a.y - 1) << 0x10;
        rect16_var_12 = (rect16_var_12 & 0xffff | (uStack14 + 1) << 0x10);
        uVar3 = (puStack26 >> 0x10);
        rect16_a = (rect16_a & 0xffff | ((iStack62 * 0x2 + puStack26) + 1) << 0x10);
        uStack14 = uStack14 & 0xffff | (uStack58 * 0x2 + puStack26 + 0x14) << 0x10;
        FillRect16(brush_handle_var6, &rect16_a, &DAT_1050_1050);
        FillRect16(brush_handle_var6, &rect16_var_12, &DAT_1050_1050);
        DeleteObject16(brush_handle_var6);
        iStack62 += -0x1;
        puVar2 = &uStack58.field0_0x0;
    }
    brush_handle_var6 = CreateSolidBrush16(*puStack30);
    rect16_a = (rect16_a & 0xffff);
    x_var_6 = x_var_6 & 0xffff | *puStack26 << 0x10;
    rect16_var_12 = (rect16_var_12 & 0xffff | ((uStack20 * 0x2 + puStack26) + 1) << 0x10);
    uStack14 = uStack14 & 0xffff | local_struct_1.field7_0xe << 0x10;
    FillRect16(brush_handle_var6, &rect16_a, &DAT_1050_1050);
    FillRect16(brush_handle_var6, &rect16_var_12, &DAT_1050_1050);
    DeleteObject16(brush_handle_var6);
    uStack58 = (&u16_1050_0002 + 1);
    loop {
        HVar3 = hdc16_var_48;
        HStack50 = CreatePen16(*(uStack58 * 0x4 + uStack38), 0x0, 0x0);
        hgdiobj16_var_52 = SelectObject16(HStack50, HVar3);
        iVar2 = uStack58 * 0x2;
        rect16_a.x = (iVar2 + uStack42) + rect16_a.x;
        uVar8 = (uStack46 >> 0x10);
        piVar1 = (iVar2 + uStack46);
        MoveToEx16(
            &point16_38,
            &DAT_1050_1050,
            ((iVar2 + uStack46) * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16(
            ((uStack20 - *piVar1) * 0x2 + puStack26),
            rect16_a.x,
            hdc16_var_48,
        );
        rect16_a.x = ((iStack22 - uStack58) * 0x2 + uStack42) + rect16_a.x;
        MoveToEx16(
            &point16_38,
            &DAT_1050_1050,
            (*piVar1 * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16(
            ((uStack20 - *piVar1) * 0x2 + puStack26),
            rect16_a.x,
            hdc16_var_48,
        );
        SelectObject16(hgdiobj16_var_52, hdc16_var_48);
        DeleteObject16(HStack50);
        uStack58 = (&uStack58[-0x1].field0_0x0 + 1);
        if uStack58 >= 0x8000 {
            break;
        }
    }

    local_struct_1.field8_0x10 = 0;
    return;
}

pub unsafe fn pass1_1020_96a2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    palette_op_1020_92c4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn init_globals_1020_96d4() {
    let mut puVar1: *mut u16;
    let mut iVar2: i16;
    let mut puVar3: *mut u16;

    _PTR_LOOP_1050_4514 = 0;
    _PTR_LOOP_1050_451a = 0;
    PTR_LOOP_1050_4520 = 0x4430;
    PTR_LOOP_1050_4522 = &DAT_1050_1050;
    PTR_LOOP_1050_4526 = 0x4430;
    PTR_LOOP_1050_4528 = &DAT_1050_1050;
    PTR_LOOP_1050_4524 = PTR_u16_1050_0002_1050_4434;
    PTR_LOOP_1050_452a = PTR_u16_1050_0002_1050_4434;
    PTR_LOOP_1050_452c = 0x4430;
    PTR_LOOP_1050_452e = &DAT_1050_1050;
    PTR_LOOP_1050_4530 = PTR_u16_1050_0002_1050_4434;
    PTR_LOOP_1050_4532 = 0x4430;
    PTR_LOOP_1050_4534 = &DAT_1050_1050;
    PTR_LOOP_1050_4536 = PTR_u16_1050_0002_1050_4434;
    _PTR_LOOP_1050_4538 = 0;
    _PTR_LOOP_1050_453e = 0;
    PTR_LOOP_1050_4544 = 0x4436;
    PTR_LOOP_1050_4546 = &DAT_1050_1050;
    PTR_LOOP_1050_454a = 0x4436;
    PTR_LOOP_1050_454c = &DAT_1050_1050;
    PTR_LOOP_1050_4548 = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_454e = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_4550 = 0x4436;
    PTR_LOOP_1050_4552 = &DAT_1050_1050;
    PTR_LOOP_1050_4554 = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_4512 = DAT_1050_4462;
    PTR_LOOP_1050_455a = DAT_1050_4462;
    PTR_LOOP_1050_4556 = 0x4454;
    PTR_LOOP_1050_4558 = &DAT_1050_1050;
    PTR_LOOP_1050_455c = 0x4454;
    PTR_LOOP_1050_455e = &DAT_1050_1050;
    PTR_LOOP_1050_4560 = DAT_1050_4462;
    PTR_LOOP_1050_4562 = 0x4454;
    PTR_LOOP_1050_4564 = &DAT_1050_1050;
    PTR_LOOP_1050_4566 = DAT_1050_4462;
    PTR_LOOP_1050_456a = null_mut();
    PTR_LOOP_1050_4568 = null_mut();
    PTR_LOOP_1050_456e = 0x443c;
    PTR_LOOP_1050_4570 = &DAT_1050_1050;
    PTR_LOOP_1050_4574 = 0x443c;
    PTR_LOOP_1050_4576 = &DAT_1050_1050;
    PTR_LOOP_1050_4572 = DAT_1050_4446;
    PTR_LOOP_1050_4578 = DAT_1050_4446;
    PTR_LOOP_1050_457a = 0x443c;
    PTR_LOOP_1050_457c = &DAT_1050_1050;
    PTR_LOOP_1050_457e = DAT_1050_4446;
    PTR_LOOP_1050_4580 = 0x443c;
    PTR_LOOP_1050_4582 = &DAT_1050_1050;
    PTR_LOOP_1050_4584 = DAT_1050_4446;
    PTR_LOOP_1050_4586 = 0x443c;
    PTR_LOOP_1050_4588 = &DAT_1050_1050;
    PTR_LOOP_1050_458a = DAT_1050_4446;
    PTR_LOOP_1050_458c = 0x443c;
    PTR_LOOP_1050_458e = &DAT_1050_1050;
    PTR_LOOP_1050_4590 = DAT_1050_4446;
    PTR_LOOP_1050_4592 = 0x4454;
    PTR_LOOP_1050_4594 = &DAT_1050_1050;
    PTR_LOOP_1050_4596 = DAT_1050_4462;
    PTR_LOOP_1050_4598 = 0x4454;
    PTR_LOOP_1050_459a = &DAT_1050_1050;
    PTR_LOOP_1050_459c = DAT_1050_4462;
    PTR_LOOP_1050_459e = 0x4436;
    PTR_LOOP_1050_45a0 = &DAT_1050_1050;
    PTR_LOOP_1050_45a2 = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_45a4 = 0x4436;
    PTR_LOOP_1050_45a6 = &DAT_1050_1050;
    PTR_LOOP_1050_45a8 = PTR_u16_1050_0002_1050_443a;
    _PTR_LOOP_1050_45aa = 0;
    _PTR_LOOP_1050_45b0 = 0;
    _PTR_LOOP_1050_45b6 = 0;
    PTR_LOOP_1050_45bc = 0x443c;
    PTR_LOOP_1050_45be = &DAT_1050_1050;
    PTR_LOOP_1050_45c0 = DAT_1050_4446;
    PTR_LOOP_1050_45c2 = 0x443c;
    PTR_LOOP_1050_45c4 = &DAT_1050_1050;
    PTR_LOOP_1050_45c6 = DAT_1050_4446;
    _PTR_LOOP_1050_45c8 = 0;
    _PTR_LOOP_1050_45ce = 0;
    _PTR_LOOP_1050_45d4 = 0;
    _PTR_LOOP_1050_45da = 0;
    PTR_LOOP_1050_45e0 = 0x443c;
    PTR_LOOP_1050_45e2 = &DAT_1050_1050;
    PTR_LOOP_1050_45e4 = DAT_1050_4446;
    PTR_LOOP_1050_45e6 = 0x443c;
    PTR_LOOP_1050_45e8 = &DAT_1050_1050;
    PTR_LOOP_1050_45ea = DAT_1050_4446;
    _PTR_LOOP_1050_45ec = 0;
    _PTR_LOOP_1050_45f2 = 0;
    _PTR_LOOP_1050_45f8 = 0;
    PTR_LOOP_1050_45fe = 0x443c;
    PTR_LOOP_1050_4600 = &DAT_1050_1050;
    PTR_LOOP_1050_4602 = DAT_1050_4446;
    PTR_LOOP_1050_4604 = 0x443c;
    PTR_LOOP_1050_4606 = &DAT_1050_1050;
    PTR_LOOP_1050_4608 = DAT_1050_4446;
    _PTR_LOOP_1050_460a = 0;
    _PTR_LOOP_1050_4610 = 0;
    PTR_LOOP_1050_451e = 0xffff;
    PTR_LOOP_1050_45ae = 0xffff;
    PTR_LOOP_1050_45b4 = 0xffff;
    PTR_LOOP_1050_45ba = 0xffff;
    PTR_LOOP_1050_45cc = 0xffff;
    PTR_LOOP_1050_45d2 = 0xffff;
    PTR_LOOP_1050_45f6 = 0xffff;
    PTR_LOOP_1050_45fc = 0xffff;
    PTR_LOOP_1050_460e = 0xffff;
    PTR_LOOP_1050_4614 = 0xffff;
    _PTR_LOOP_1050_4616 = 0;
    _PTR_LOOP_1050_461c = 0;
    _PTR_LOOP_1050_4622 = 0;
    _PTR_LOOP_1050_4628 = 0;
    _PTR_LOOP_1050_462e = 0;
    _PTR_LOOP_1050_4634 = 0;
    PTR_LOOP_1050_4518 = null_mut();
    PTR_LOOP_1050_453c = null_mut();
    PTR_LOOP_1050_4542 = null_mut();
    PTR_LOOP_1050_456c = null_mut();
    PTR_LOOP_1050_45d8 = null_mut();
    PTR_LOOP_1050_45de = null_mut();
    PTR_LOOP_1050_45f0 = null_mut();
    PTR_LOOP_1050_461a = null_mut();
    PTR_LOOP_1050_4620 = null_mut();
    PTR_LOOP_1050_4626 = null_mut();
    PTR_LOOP_1050_462c = null_mut();
    PTR_LOOP_1050_4632 = null_mut();
    PTR_LOOP_1050_4638 = null_mut();
    _PTR_LOOP_1050_463a = 0;
    _PTR_LOOP_1050_4640 = 0;
    _PTR_LOOP_1050_4646 = 0;
    _PTR_LOOP_1050_464c = 0;
    _PTR_LOOP_1050_4652 = 0;
    _PTR_LOOP_1050_4658 = 0;
    PTR_LOOP_1050_465e = 0x4448;
    PTR_LOOP_1050_4660 = &DAT_1050_1050;
    PTR_LOOP_1050_4664 = 0x4448;
    PTR_LOOP_1050_4666 = &DAT_1050_1050;
    PTR_LOOP_1050_4662 = DAT_1050_4452;
    PTR_LOOP_1050_4668 = DAT_1050_4452;
    PTR_LOOP_1050_466a = 0x4448;
    PTR_LOOP_1050_466c = &DAT_1050_1050;
    PTR_LOOP_1050_466e = DAT_1050_4452;
    PTR_LOOP_1050_4670 = 0x4454;
    PTR_LOOP_1050_4672 = &DAT_1050_1050;
    PTR_LOOP_1050_4676 = 0x4454;
    PTR_LOOP_1050_4678 = &DAT_1050_1050;
    PTR_LOOP_1050_4674 = DAT_1050_4462;
    PTR_LOOP_1050_467a = DAT_1050_4462;
    PTR_LOOP_1050_467c = 0x4454;
    PTR_LOOP_1050_467e = &DAT_1050_1050;
    PTR_LOOP_1050_4680 = DAT_1050_4462;
    PTR_LOOP_1050_4682 = 0x4454;
    PTR_LOOP_1050_4684 = &DAT_1050_1050;
    PTR_LOOP_1050_4686 = DAT_1050_4462;
    PTR_LOOP_1050_4688 = 0x4454;
    PTR_LOOP_1050_468a = &DAT_1050_1050;
    PTR_LOOP_1050_468c = DAT_1050_4462;
    PTR_LOOP_1050_468e = 0x4448;
    PTR_LOOP_1050_4690 = &DAT_1050_1050;
    PTR_LOOP_1050_4692 = DAT_1050_4452;
    PTR_LOOP_1050_4694 = 0x4448;
    PTR_LOOP_1050_4696 = &DAT_1050_1050;
    PTR_LOOP_1050_4698 = DAT_1050_4452;
    PTR_LOOP_1050_469a = 0x4448;
    PTR_LOOP_1050_469c = &DAT_1050_1050;
    PTR_LOOP_1050_469e = DAT_1050_4452;
    PTR_LOOP_1050_46a0 = 0x4448;
    PTR_LOOP_1050_46a2 = &DAT_1050_1050;
    PTR_LOOP_1050_46a4 = DAT_1050_4452;
    PTR_LOOP_1050_46a6 = 0x4454;
    PTR_LOOP_1050_46a8 = &DAT_1050_1050;
    PTR_LOOP_1050_46aa = DAT_1050_4462;
    PTR_LOOP_1050_46ac = 0x4454;
    PTR_LOOP_1050_46ae = &DAT_1050_1050;
    PTR_LOOP_1050_46b0 = DAT_1050_4462;
    PTR_LOOP_1050_46b2 = 0x4454;
    PTR_LOOP_1050_46b4 = &DAT_1050_1050;
    PTR_LOOP_1050_46b6 = DAT_1050_4462;
    PTR_LOOP_1050_46b8 = 0x4454;
    PTR_LOOP_1050_46ba = &DAT_1050_1050;
    PTR_LOOP_1050_46bc = DAT_1050_4462;
    PTR_LOOP_1050_46be = 0x4454;
    PTR_LOOP_1050_46c0 = &DAT_1050_1050;
    PTR_LOOP_1050_46c2 = DAT_1050_4462;
    PTR_LOOP_1050_46c6 = null_mut();
    PTR_LOOP_1050_46c4 = null_mut();
    PTR_LOOP_1050_46cc = null_mut();
    PTR_LOOP_1050_46ca = null_mut();
    PTR_LOOP_1050_46d2 = null_mut();
    PTR_LOOP_1050_46d0 = null_mut();
    PTR_LOOP_1050_46d8 = null_mut();
    PTR_LOOP_1050_46d6 = null_mut();
    PTR_LOOP_1050_46de = null_mut();
    PTR_LOOP_1050_46dc = null_mut();
    PTR_LOOP_1050_46e2 = 0x4454;
    PTR_LOOP_1050_46e4 = &DAT_1050_1050;
    PTR_LOOP_1050_46e6 = DAT_1050_4462;
    PTR_LOOP_1050_46e8 = 0x4448;
    PTR_LOOP_1050_46ea = &DAT_1050_1050;
    PTR_LOOP_1050_46ec = DAT_1050_4452;
    PTR_LOOP_1050_46ee = 0x4448;
    PTR_LOOP_1050_46f0 = &DAT_1050_1050;
    PTR_LOOP_1050_46f2 = DAT_1050_4452;
    _PTR_LOOP_1050_46f4 = 0;
    _PTR_LOOP_1050_46fa = 0;
    PTR_LOOP_1050_46f8 = 0xffff;
    PTR_LOOP_1050_46fe = 0xffff;
    _PTR_LOOP_1050_4700 = 0;
    _PTR_LOOP_1050_4706 = 0;
    PTR_LOOP_1050_470c = 0x4448;
    PTR_LOOP_1050_470e = &DAT_1050_1050;
    PTR_LOOP_1050_4710 = DAT_1050_4452;
    PTR_LOOP_1050_4712 = 0x4448;
    PTR_LOOP_1050_4714 = &DAT_1050_1050;
    PTR_LOOP_1050_4716 = DAT_1050_4452;
    _PTR_LOOP_1050_4718 = 0;
    _PTR_LOOP_1050_471e = 0;
    _PTR_LOOP_1050_4724 = 0;
    _PTR_LOOP_1050_472a = 0;
    _PTR_LOOP_1050_4730 = 0;
    _PTR_LOOP_1050_4736 = 0;
    _PTR_LOOP_1050_473c = 0;
    _PTR_LOOP_1050_4742 = 0;
    _PTR_LOOP_1050_4748 = 0;
    _PTR_LOOP_1050_474e = 0;
    _PTR_LOOP_1050_4754 = 0;
    _PTR_LOOP_1050_475a = 0;
    _PTR_LOOP_1050_4760 = 0;
    PTR_LOOP_1050_463e = null_mut();
    PTR_LOOP_1050_4644 = null_mut();
    PTR_LOOP_1050_464a = null_mut();
    PTR_LOOP_1050_4650 = null_mut();
    PTR_LOOP_1050_4656 = null_mut();
    PTR_LOOP_1050_465c = null_mut();
    PTR_LOOP_1050_46c8 = null_mut();
    PTR_LOOP_1050_46ce = null_mut();
    PTR_LOOP_1050_46d4 = null_mut();
    PTR_LOOP_1050_46da = null_mut();
    PTR_LOOP_1050_46e0 = null_mut();
    PTR_LOOP_1050_4704 = null_mut();
    PTR_LOOP_1050_470a = null_mut();
    PTR_LOOP_1050_471c = null_mut();
    PTR_LOOP_1050_4722 = null_mut();
    PTR_LOOP_1050_4728 = null_mut();
    PTR_LOOP_1050_472e = null_mut();
    PTR_LOOP_1050_4734 = null_mut();
    PTR_LOOP_1050_473a = null_mut();
    PTR_LOOP_1050_4740 = null_mut();
    PTR_LOOP_1050_4746 = null_mut();
    PTR_LOOP_1050_474c = null_mut();
    PTR_LOOP_1050_4752 = null_mut();
    PTR_LOOP_1050_4758 = null_mut();
    PTR_LOOP_1050_475e = null_mut();
    PTR_LOOP_1050_4764 = null_mut();
    _PTR_LOOP_1050_4766 = 0;
    _PTR_LOOP_1050_476c = 0;
    _PTR_LOOP_1050_4772 = 0;
    _PTR_LOOP_1050_4778 = 0;
    _PTR_LOOP_1050_477e = 0;
    _PTR_LOOP_1050_4784 = 0;
    _PTR_LOOP_1050_478a = 0;
    _PTR_LOOP_1050_4790 = 0;
    _PTR_LOOP_1050_4796 = 0;
    _PTR_LOOP_1050_479c = 0;
    _PTR_LOOP_1050_47a2 = 0;
    _PTR_LOOP_1050_47a8 = 0;
    _PTR_LOOP_1050_47ae = 0;
    _PTR_LOOP_1050_47b4 = 0;
    PTR_LOOP_1050_476a = null_mut();
    PTR_LOOP_1050_4770 = null_mut();
    PTR_LOOP_1050_4776 = null_mut();
    PTR_LOOP_1050_477c = null_mut();
    PTR_LOOP_1050_4782 = null_mut();
    PTR_LOOP_1050_4788 = null_mut();
    PTR_LOOP_1050_478e = null_mut();
    PTR_LOOP_1050_4794 = null_mut();
    PTR_LOOP_1050_479a = null_mut();
    PTR_LOOP_1050_47a0 = null_mut();
    PTR_LOOP_1050_47a6 = null_mut();
    PTR_LOOP_1050_47ac = null_mut();
    PTR_LOOP_1050_47b2 = null_mut();
    PTR_LOOP_1050_47b8 = null_mut();
    puVar3 = 0x47ba;
    //   for (iVar2 = 0x1b; iVar2 != 0; iVar2 += -1)
    for iVar2 in 0x1b..0 {
        puVar1 = puVar3;
        puVar3 = puVar3 + 1;
        *puVar1 = 0;
    }
    _PTR_LOOP_1050_4850 = 0;
    _PTR_LOOP_1050_4856 = 0;
    PTR_LOOP_1050_484e = PTR_u16_1050_0002_1050_4468;
    PTR_LOOP_1050_4860 = PTR_u16_1050_0002_1050_4468;
    PTR_LOOP_1050_485c = 0x4464;
    PTR_LOOP_1050_485e = &DAT_1050_1050;
    PTR_LOOP_1050_4862 = 0x4464;
    PTR_LOOP_1050_4864 = &DAT_1050_1050;
    PTR_LOOP_1050_4866 = PTR_u16_1050_0002_1050_4468;
    PTR_LOOP_1050_4868 = 0x4464;
    PTR_LOOP_1050_486a = &DAT_1050_1050;
    PTR_LOOP_1050_486c = PTR_u16_1050_0002_1050_4468;
    PTR_LOOP_1050_486e = 0x4464;
    PTR_LOOP_1050_4870 = &DAT_1050_1050;
    PTR_LOOP_1050_4872 = PTR_u16_1050_0002_1050_4468;
    _PTR_LOOP_1050_4874 = 0;
    _PTR_LOOP_1050_487a = 0;
    PTR_LOOP_1050_4880 = 0x4436;
    PTR_LOOP_1050_4882 = &DAT_1050_1050;
    PTR_LOOP_1050_4886 = 0x4436;
    PTR_LOOP_1050_4888 = &DAT_1050_1050;
    PTR_LOOP_1050_4884 = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_488a = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_488c = 0x4436;
    PTR_LOOP_1050_488e = &DAT_1050_1050;
    PTR_LOOP_1050_4890 = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_4892 = 0x4482;
    PTR_LOOP_1050_4894 = &DAT_1050_1050;
    PTR_LOOP_1050_4898 = 0x4482;
    PTR_LOOP_1050_489a = &DAT_1050_1050;
    PTR_LOOP_1050_4896 = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_489c = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_489e = 0x4482;
    PTR_LOOP_1050_48a0 = &DAT_1050_1050;
    PTR_LOOP_1050_48a2 = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_48a6 = null_mut();
    PTR_LOOP_1050_48a4 = null_mut();
    PTR_LOOP_1050_48aa = 0x4488;
    PTR_LOOP_1050_48ac = &DAT_1050_1050;
    PTR_LOOP_1050_48b0 = 0x4488;
    PTR_LOOP_1050_48b2 = &DAT_1050_1050;
    PTR_LOOP_1050_48ae = PTR_u16_1050_0002_1050_448c;
    PTR_LOOP_1050_48b4 = PTR_u16_1050_0002_1050_448c;
    PTR_LOOP_1050_48b6 = 0x4488;
    PTR_LOOP_1050_48b8 = &DAT_1050_1050;
    PTR_LOOP_1050_48ba = PTR_u16_1050_0002_1050_448c;
    PTR_LOOP_1050_48bc = 0x446a;
    PTR_LOOP_1050_48be = &DAT_1050_1050;
    PTR_LOOP_1050_48c2 = 0x446a;
    PTR_LOOP_1050_48c4 = &DAT_1050_1050;
    PTR_LOOP_1050_48c0 = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_48c6 = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_48c8 = 0x446a;
    PTR_LOOP_1050_48ca = &DAT_1050_1050;
    PTR_LOOP_1050_48cc = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_48ce = 0x447a;
    PTR_LOOP_1050_48d0 = &DAT_1050_1050;
    PTR_LOOP_1050_48d4 = 0x447a;
    PTR_LOOP_1050_48d6 = &DAT_1050_1050;
    PTR_LOOP_1050_48d2 = DAT_1050_4480;
    PTR_LOOP_1050_48d8 = DAT_1050_4480;
    PTR_LOOP_1050_48da = 0x4436;
    PTR_LOOP_1050_48dc = &DAT_1050_1050;
    PTR_LOOP_1050_48de = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_48e0 = 0x4436;
    PTR_LOOP_1050_48e2 = &DAT_1050_1050;
    PTR_LOOP_1050_48e4 = PTR_u16_1050_0002_1050_443a;
    PTR_LOOP_1050_48e6 = 0x447a;
    PTR_LOOP_1050_48e8 = &DAT_1050_1050;
    PTR_LOOP_1050_48ea = DAT_1050_4480;
    _PTR_LOOP_1050_48ec = 0;
    _PTR_LOOP_1050_48f2 = 0;
    PTR_LOOP_1050_48f8 = 0x447a;
    PTR_LOOP_1050_48fa = &DAT_1050_1050;
    PTR_LOOP_1050_48fc = DAT_1050_4480;
    PTR_LOOP_1050_48fe = 0x447a;
    PTR_LOOP_1050_4900 = &DAT_1050_1050;
    PTR_LOOP_1050_4902 = DAT_1050_4480;
    _PTR_LOOP_1050_4904 = 0;
    _PTR_LOOP_1050_490a = 0;
    PTR_LOOP_1050_485a = 0xffff;
    PTR_LOOP_1050_48f0 = 0xffff;
    PTR_LOOP_1050_48f6 = 0xffff;
    PTR_LOOP_1050_4908 = 0xffff;
    PTR_LOOP_1050_490e = 0xffff;
    _PTR_LOOP_1050_4910 = 0;
    _PTR_LOOP_1050_4916 = 0;
    PTR_LOOP_1050_4854 = null_mut();
    PTR_LOOP_1050_4878 = null_mut();
    PTR_LOOP_1050_487e = null_mut();
    PTR_LOOP_1050_48a8 = null_mut();
    PTR_LOOP_1050_4914 = null_mut();
    PTR_LOOP_1050_491a = null_mut();
    PTR_LOOP_1050_491c = 0x4488;
    PTR_LOOP_1050_491e = &DAT_1050_1050;
    PTR_LOOP_1050_4920 = PTR_u16_1050_0002_1050_448c;
    PTR_LOOP_1050_4922 = 0x4488;
    PTR_LOOP_1050_4924 = &DAT_1050_1050;
    PTR_LOOP_1050_4926 = PTR_u16_1050_0002_1050_448c;
    _PTR_LOOP_1050_4928 = 0;
    _PTR_LOOP_1050_492e = 0;
    _PTR_LOOP_1050_4934 = 0;
    PTR_LOOP_1050_493a = 0x446a;
    PTR_LOOP_1050_493c = &DAT_1050_1050;
    PTR_LOOP_1050_4940 = 0x446a;
    PTR_LOOP_1050_4942 = &DAT_1050_1050;
    PTR_LOOP_1050_493e = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_4944 = PTR_u16_1050_0002_1050_446e;
    _PTR_LOOP_1050_4946 = 0;
    _PTR_LOOP_1050_494c = 0;
    _PTR_LOOP_1050_4952 = 0;
    _PTR_LOOP_1050_4958 = 0;
    _PTR_LOOP_1050_495e = 0;
    _PTR_LOOP_1050_4964 = 0;
    _PTR_LOOP_1050_496a = 0;
    _PTR_LOOP_1050_4970 = 0;
    _PTR_LOOP_1050_4976 = 0;
    _PTR_LOOP_1050_497c = 0;
    _PTR_LOOP_1050_4982 = 0;
    _PTR_LOOP_1050_4988 = 0;
    _PTR_LOOP_1050_498e = 0;
    _PTR_LOOP_1050_4994 = 0;
    PTR_LOOP_1050_499a = 0x4448;
    PTR_LOOP_1050_499c = &DAT_1050_1050;
    PTR_LOOP_1050_49a0 = 0x4448;
    PTR_LOOP_1050_49a2 = &DAT_1050_1050;
    PTR_LOOP_1050_499e = DAT_1050_4452;
    PTR_LOOP_1050_49a4 = DAT_1050_4452;
    PTR_LOOP_1050_49a6 = 0x4448;
    PTR_LOOP_1050_49a8 = &DAT_1050_1050;
    PTR_LOOP_1050_49aa = DAT_1050_4452;
    PTR_LOOP_1050_49ac = 0x4470;
    PTR_LOOP_1050_49ae = &DAT_1050_1050;
    PTR_LOOP_1050_49b2 = 0x4470;
    PTR_LOOP_1050_49b4 = &DAT_1050_1050;
    PTR_LOOP_1050_49b0 = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_49b6 = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_49b8 = 0x4470;
    PTR_LOOP_1050_49ba = &DAT_1050_1050;
    PTR_LOOP_1050_49bc = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_49be = 0x4470;
    PTR_LOOP_1050_49c0 = &DAT_1050_1050;
    PTR_LOOP_1050_49c2 = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_49c4 = 0x4470;
    PTR_LOOP_1050_49c6 = &DAT_1050_1050;
    PTR_LOOP_1050_49c8 = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_49ca = 0x4448;
    PTR_LOOP_1050_49cc = &DAT_1050_1050;
    PTR_LOOP_1050_49ce = DAT_1050_4452;
    PTR_LOOP_1050_49d0 = 0x4448;
    PTR_LOOP_1050_49d2 = &DAT_1050_1050;
    PTR_LOOP_1050_49d4 = DAT_1050_4452;
    PTR_LOOP_1050_49d6 = 0x4448;
    PTR_LOOP_1050_49d8 = &DAT_1050_1050;
    PTR_LOOP_1050_49da = DAT_1050_4452;
    PTR_LOOP_1050_49dc = 0x4448;
    PTR_LOOP_1050_49de = &DAT_1050_1050;
    PTR_LOOP_1050_49e0 = DAT_1050_4452;
    PTR_LOOP_1050_49e2 = 0x4482;
    PTR_LOOP_1050_49e4 = &DAT_1050_1050;
    PTR_LOOP_1050_49e8 = 0x4482;
    PTR_LOOP_1050_49ea = &DAT_1050_1050;
    PTR_LOOP_1050_49e6 = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_49ec = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_49ee = 0x4470;
    PTR_LOOP_1050_49f0 = &DAT_1050_1050;
    PTR_LOOP_1050_49f2 = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_49f4 = 0x4470;
    PTR_LOOP_1050_49f6 = &DAT_1050_1050;
    PTR_LOOP_1050_49f8 = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_49fa = 0x4470;
    PTR_LOOP_1050_49fc = &DAT_1050_1050;
    PTR_LOOP_1050_49fe = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_4a02 = null_mut();
    PTR_LOOP_1050_4a00 = null_mut();
    PTR_LOOP_1050_4a08 = null_mut();
    PTR_LOOP_1050_4a06 = null_mut();
    PTR_LOOP_1050_4a0e = null_mut();
    PTR_LOOP_1050_4a0c = null_mut();
    PTR_LOOP_1050_4a14 = null_mut();
    PTR_LOOP_1050_4a12 = null_mut();
    PTR_LOOP_1050_4a1a = null_mut();
    PTR_LOOP_1050_4a18 = null_mut();
    PTR_LOOP_1050_4a1e = 0x4470;
    PTR_LOOP_1050_4a20 = &DAT_1050_1050;
    PTR_LOOP_1050_4a22 = PTR_u32_1050_0004_1050_4478;
    PTR_LOOP_1050_4a24 = 0x4448;
    PTR_LOOP_1050_4a26 = &DAT_1050_1050;
    PTR_LOOP_1050_4a28 = DAT_1050_4452;
    PTR_LOOP_1050_4a2a = 0x4448;
    PTR_LOOP_1050_4a2c = &DAT_1050_1050;
    PTR_LOOP_1050_4a2e = DAT_1050_4452;
    _PTR_LOOP_1050_4a30 = 0;
    _PTR_LOOP_1050_4a36 = 0;
    PTR_LOOP_1050_492c = 0xffff;
    PTR_LOOP_1050_4932 = 0xffff;
    PTR_LOOP_1050_4938 = 0xffff;
    PTR_LOOP_1050_494a = 0xffff;
    PTR_LOOP_1050_4950 = 0xffff;
    PTR_LOOP_1050_4a34 = 0xffff;
    PTR_LOOP_1050_4a3a = 0xffff;
    _PTR_LOOP_1050_4a3c = 0;
    _PTR_LOOP_1050_4a42 = 0;
    PTR_LOOP_1050_4956 = null_mut();
    PTR_LOOP_1050_495c = null_mut();
    PTR_LOOP_1050_4962 = null_mut();
    PTR_LOOP_1050_4968 = null_mut();
    PTR_LOOP_1050_496e = null_mut();
    PTR_LOOP_1050_4974 = null_mut();
    PTR_LOOP_1050_497a = null_mut();
    PTR_LOOP_1050_4980 = null_mut();
    PTR_LOOP_1050_4986 = null_mut();
    PTR_LOOP_1050_498c = null_mut();
    PTR_LOOP_1050_4992 = null_mut();
    PTR_LOOP_1050_4998 = null_mut();
    PTR_LOOP_1050_4a04 = null_mut();
    PTR_LOOP_1050_4a0a = null_mut();
    PTR_LOOP_1050_4a10 = null_mut();
    PTR_LOOP_1050_4a16 = null_mut();
    PTR_LOOP_1050_4a1c = null_mut();
    PTR_LOOP_1050_4a40 = null_mut();
    PTR_LOOP_1050_4a46 = null_mut();
    PTR_LOOP_1050_4a48 = 0x4448;
    PTR_LOOP_1050_4a4a = &DAT_1050_1050;
    PTR_LOOP_1050_4a4c = DAT_1050_4452;
    PTR_LOOP_1050_4a4e = 0x4448;
    PTR_LOOP_1050_4a50 = &DAT_1050_1050;
    PTR_LOOP_1050_4a52 = DAT_1050_4452;
    _PTR_LOOP_1050_4a54 = 0;
    _PTR_LOOP_1050_4a5a = 0;
    _PTR_LOOP_1050_4a60 = 0;
    _PTR_LOOP_1050_4a66 = 0;
    _PTR_LOOP_1050_4a6c = 0;
    _PTR_LOOP_1050_4a72 = 0;
    _PTR_LOOP_1050_4a78 = 0;
    _PTR_LOOP_1050_4a7e = 0;
    _PTR_LOOP_1050_4a84 = 0;
    _PTR_LOOP_1050_4a8a = 0;
    _PTR_LOOP_1050_4a90 = 0;
    _PTR_LOOP_1050_4a96 = 0;
    _PTR_LOOP_1050_4a9c = 0;
    _PTR_LOOP_1050_4aa2 = 0;
    _PTR_LOOP_1050_4aa8 = 0;
    _PTR_LOOP_1050_4aae = 0;
    _PTR_LOOP_1050_4ab4 = 0;
    _PTR_LOOP_1050_4aba = 0;
    _PTR_LOOP_1050_4ac0 = 0;
    _PTR_LOOP_1050_4ac6 = 0;
    _PTR_LOOP_1050_4acc = 0;
    _PTR_LOOP_1050_4ad2 = 0;
    _PTR_LOOP_1050_4ad8 = 0;
    _PTR_LOOP_1050_4ade = 0;
    _PTR_LOOP_1050_4ae4 = 0;
    _PTR_LOOP_1050_4aea = 0;
    _PTR_LOOP_1050_4af0 = 0;
    PTR_LOOP_1050_4a58 = null_mut();
    PTR_LOOP_1050_4a5e = null_mut();
    PTR_LOOP_1050_4a64 = null_mut();
    PTR_LOOP_1050_4a6a = null_mut();
    PTR_LOOP_1050_4a70 = null_mut();
    PTR_LOOP_1050_4a76 = null_mut();
    PTR_LOOP_1050_4a7c = null_mut();
    PTR_LOOP_1050_4a82 = null_mut();
    PTR_LOOP_1050_4a88 = null_mut();
    PTR_LOOP_1050_4a8e = null_mut();
    PTR_LOOP_1050_4a94 = null_mut();
    PTR_LOOP_1050_4a9a = null_mut();
    PTR_LOOP_1050_4aa0 = null_mut();
    PTR_LOOP_1050_4aa6 = null_mut();
    PTR_LOOP_1050_4aac = null_mut();
    PTR_LOOP_1050_4ab2 = null_mut();
    PTR_LOOP_1050_4ab8 = null_mut();
    PTR_LOOP_1050_4abe = null_mut();
    PTR_LOOP_1050_4ac4 = null_mut();
    PTR_LOOP_1050_4aca = null_mut();
    PTR_LOOP_1050_4ad0 = null_mut();
    PTR_LOOP_1050_4ad6 = null_mut();
    PTR_LOOP_1050_4adc = null_mut();
    PTR_LOOP_1050_4ae2 = null_mut();
    PTR_LOOP_1050_4ae8 = null_mut();
    PTR_LOOP_1050_4aee = null_mut();
    PTR_LOOP_1050_4af4 = null_mut();
    puVar3 = 0x4af6;
    //   for (iVar2 = 0x1b; iVar2 != 0; iVar2 += -1)
    for iVar2 in 0x1b..0 {
        puVar1 = puVar3;
        puVar3 = puVar3 + 1;
        *puVar1 = 0;
    }
    PTR_LOOP_1050_4b9c = PTR_u16_1050_0002_1050_4434;
    _PTR_LOOP_1050_4b9e = 0;
    _PTR_LOOP_1050_4ba4 = 0;
    _PTR_LOOP_1050_4baa = 0;
    PTR_LOOP_1050_4ba2 = 0xffff;
    PTR_LOOP_1050_4ba8 = 0xffff;
    PTR_LOOP_1050_4bae = 0xffff;
    _PTR_LOOP_1050_4bb0 = 0;
    _PTR_LOOP_1050_4bb6 = 0;
    PTR_LOOP_1050_4bbc = 0x448e;
    PTR_LOOP_1050_4bbe = &DAT_1050_1050;
    PTR_LOOP_1050_4bc2 = 0x448e;
    PTR_LOOP_1050_4bc4 = &DAT_1050_1050;
    PTR_LOOP_1050_4bc0 = DAT_1050_4494;
    PTR_LOOP_1050_4bc6 = DAT_1050_4494;
    PTR_LOOP_1050_4bc8 = 0x448e;
    PTR_LOOP_1050_4bca = &DAT_1050_1050;
    PTR_LOOP_1050_4bcc = DAT_1050_4494;
    PTR_LOOP_1050_4bce = 0x4482;
    PTR_LOOP_1050_4bd0 = &DAT_1050_1050;
    PTR_LOOP_1050_4bd4 = 0x4482;
    PTR_LOOP_1050_4bd6 = &DAT_1050_1050;
    PTR_LOOP_1050_4bd2 = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_4bd8 = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_4bda = 0x4482;
    PTR_LOOP_1050_4bdc = &DAT_1050_1050;
    PTR_LOOP_1050_4bde = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_4be2 = null_mut();
    PTR_LOOP_1050_4be0 = null_mut();
    PTR_LOOP_1050_4bb4 = null_mut();
    PTR_LOOP_1050_4bba = null_mut();
    PTR_LOOP_1050_4be4 = null_mut();
    PTR_LOOP_1050_4be6 = 0x44ac;
    PTR_LOOP_1050_4be8 = &DAT_1050_1050;
    PTR_LOOP_1050_4bec = 0x44ac;
    PTR_LOOP_1050_4bee = &DAT_1050_1050;
    PTR_LOOP_1050_4bea = DAT_1050_44b2;
    PTR_LOOP_1050_4bf0 = DAT_1050_44b2;
    PTR_LOOP_1050_4bf2 = 0x44ac;
    PTR_LOOP_1050_4bf4 = &DAT_1050_1050;
    PTR_LOOP_1050_4bf6 = DAT_1050_44b2;
    PTR_LOOP_1050_4bf8 = 0x446a;
    PTR_LOOP_1050_4bfa = &DAT_1050_1050;
    PTR_LOOP_1050_4bfe = 0x446a;
    PTR_LOOP_1050_4c00 = &DAT_1050_1050;
    PTR_LOOP_1050_4bfc = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_4c02 = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_4c04 = 0x446a;
    PTR_LOOP_1050_4c06 = &DAT_1050_1050;
    PTR_LOOP_1050_4c08 = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_4c0a = 0x448e;
    PTR_LOOP_1050_4c0c = &DAT_1050_1050;
    PTR_LOOP_1050_4c0e = DAT_1050_4494;
    PTR_LOOP_1050_4c10 = 0x448e;
    PTR_LOOP_1050_4c12 = &DAT_1050_1050;
    PTR_LOOP_1050_4c14 = DAT_1050_4494;
    PTR_LOOP_1050_4c16 = 0x44ac;
    PTR_LOOP_1050_4c18 = &DAT_1050_1050;
    PTR_LOOP_1050_4c1a = DAT_1050_44b2;
    PTR_LOOP_1050_4c22 = 0x448e;
    PTR_LOOP_1050_4c24 = &DAT_1050_1050;
    PTR_LOOP_1050_4c26 = DAT_1050_4494;
    _PTR_LOOP_1050_4c28 = 0;
    _PTR_LOOP_1050_4c2e = 0;
    _PTR_LOOP_1050_4c34 = 0;
    _PTR_LOOP_1050_4c3a = 0;
    _PTR_LOOP_1050_4c40 = 0;
    _PTR_LOOP_1050_4c46 = 0;
    _u16_1050_4c4c = 0;
    _PTR_LOOP_1050_4c52 = 0;
    PTR_LOOP_1050_4c1c = 0x44ac;
    PTR_LOOP_1050_4c1e = &DAT_1050_1050;
    PTR_LOOP_1050_4c58 = 0x44ac;
    PTR_LOOP_1050_4c5a = &DAT_1050_1050;
    PTR_LOOP_1050_4c20 = DAT_1050_44b2;
    PTR_LOOP_1050_4c5c = DAT_1050_44b2;
    PTR_LOOP_1050_4c5e = 0x44ac;
    PTR_LOOP_1050_4c60 = &DAT_1050_1050;
    PTR_LOOP_1050_4c62 = DAT_1050_44b2;
    _PTR_LOOP_1050_4c64 = 0;
    _PTR_LOOP_1050_4c6a = 0;
    _PTR_LOOP_1050_4c70 = 0;
    PTR_LOOP_1050_4c76 = 0x446a;
    PTR_LOOP_1050_4c78 = &DAT_1050_1050;
    PTR_LOOP_1050_4c7c = 0x446a;
    PTR_LOOP_1050_4c7e = &DAT_1050_1050;
    PTR_LOOP_1050_4c7a = PTR_u16_1050_0002_1050_446e;
    PTR_LOOP_1050_4c80 = PTR_u16_1050_0002_1050_446e;
    _PTR_LOOP_1050_4c82 = 0;
    _PTR_LOOP_1050_4c88 = 0;
    PTR_LOOP_1050_4c2c = 0xffff;
    PTR_LOOP_1050_4c32 = 0xffff;
    PTR_LOOP_1050_4c38 = 0xffff;
    PTR_LOOP_1050_4c3e = 0xffff;
    PTR_LOOP_1050_4c44 = 0xffff;
    PTR_LOOP_1050_4c4a = 0xffff;
    PTR_LOOP_1050_4c68 = 0xffff;
    PTR_LOOP_1050_4c6e = 0xffff;
    PTR_LOOP_1050_4c74 = 0xffff;
    PTR_LOOP_1050_4c86 = 0xffff;
    PTR_LOOP_1050_4c8c = 0xffff;
    _PTR_LOOP_1050_4c8e = 0;
    _PTR_LOOP_1050_4c94 = 0;
    _PTR_LOOP_1050_4c9a = 0;
    _PTR_LOOP_1050_4ca0 = 0;
    _PTR_LOOP_1050_4ca6 = 0;
    _PTR_LOOP_1050_4cac = 0;
    _PTR_LOOP_1050_4cb2 = 0;
    _PTR_LOOP_1050_4cb8 = 0;
    _PTR_LOOP_1050_4cbe = 0;
    _PTR_LOOP_1050_4cc4 = 0;
    _PTR_LOOP_1050_4cca = 0;
    _PTR_LOOP_1050_4cd0 = 0;
    PTR_LOOP_1050_4cd6 = 0x4496;
    PTR_LOOP_1050_4cd8 = &DAT_1050_1050;
    PTR_LOOP_1050_4cdc = 0x4496;
    PTR_LOOP_1050_4cde = &DAT_1050_1050;
    PTR_LOOP_1050_4cda = DAT_1050_44a2;
    PTR_LOOP_1050_4ce0 = DAT_1050_44a2;
    PTR_LOOP_1050_4ce2 = 0x4496;
    PTR_LOOP_1050_4ce4 = &DAT_1050_1050;
    PTR_LOOP_1050_4ce6 = DAT_1050_44a2;
    PTR_LOOP_1050_4ce8 = 0x4496;
    PTR_LOOP_1050_4cea = &DAT_1050_1050;
    PTR_LOOP_1050_4cec = DAT_1050_44a2;
    PTR_LOOP_1050_4cee = 0x4496;
    PTR_LOOP_1050_4cf0 = &DAT_1050_1050;
    PTR_LOOP_1050_4cf2 = DAT_1050_44a2;
    PTR_LOOP_1050_4cf4 = 0x44a4;
    PTR_LOOP_1050_4cf6 = &DAT_1050_1050;
    PTR_LOOP_1050_4cfa = 0x44a4;
    PTR_LOOP_1050_4cfc = &DAT_1050_1050;
    PTR_LOOP_1050_4cf8 = DAT_1050_44aa;
    PTR_LOOP_1050_4cfe = DAT_1050_44aa;
    PTR_LOOP_1050_4d00 = 0x44a4;
    PTR_LOOP_1050_4d02 = &DAT_1050_1050;
    PTR_LOOP_1050_4d04 = DAT_1050_44aa;
    PTR_LOOP_1050_4d06 = 0x4496;
    PTR_LOOP_1050_4d08 = &DAT_1050_1050;
    PTR_LOOP_1050_4d0a = DAT_1050_44a2;
    PTR_LOOP_1050_4d0c = 0x4496;
    PTR_LOOP_1050_4d0e = &DAT_1050_1050;
    PTR_LOOP_1050_4d10 = DAT_1050_44a2;
    PTR_LOOP_1050_4d12 = 0x4496;
    PTR_LOOP_1050_4d14 = &DAT_1050_1050;
    PTR_LOOP_1050_4d16 = DAT_1050_44a2;
    PTR_LOOP_1050_4d18 = 0x4496;
    PTR_LOOP_1050_4d1a = &DAT_1050_1050;
    PTR_LOOP_1050_4d1c = DAT_1050_44a2;
    PTR_LOOP_1050_4d1e = 0x4482;
    PTR_LOOP_1050_4d20 = &DAT_1050_1050;
    PTR_LOOP_1050_4d24 = 0x4482;
    PTR_LOOP_1050_4d26 = &DAT_1050_1050;
    PTR_LOOP_1050_4d22 = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_4d28 = PTR_u16_1050_0002_1050_4486;
    PTR_LOOP_1050_4d2a = 0x44a4;
    PTR_LOOP_1050_4d2c = &DAT_1050_1050;
    PTR_LOOP_1050_4d2e = DAT_1050_44aa;
    PTR_LOOP_1050_4d30 = 0x44a4;
    PTR_LOOP_1050_4d32 = &DAT_1050_1050;
    PTR_LOOP_1050_4d34 = DAT_1050_44aa;
    PTR_LOOP_1050_4d36 = 0x44a4;
    PTR_LOOP_1050_4d38 = &DAT_1050_1050;
    PTR_LOOP_1050_4d3a = DAT_1050_44aa;
    _PTR_LOOP_1050_4d3c = 0;
    _PTR_LOOP_1050_4d42 = 0;
    PTR_LOOP_1050_4c50 = null_mut();
    PTR_LOOP_1050_4c56 = null_mut();
    PTR_LOOP_1050_4c92 = null_mut();
    PTR_LOOP_1050_4c98 = null_mut();
    PTR_LOOP_1050_4c9e = null_mut();
    PTR_LOOP_1050_4ca4 = null_mut();
    PTR_LOOP_1050_4caa = null_mut();
    PTR_LOOP_1050_4cb0 = null_mut();
    PTR_LOOP_1050_4cb6 = null_mut();
    PTR_LOOP_1050_4cbc = null_mut();
    PTR_LOOP_1050_4cc2 = null_mut();
    PTR_LOOP_1050_4cc8 = null_mut();
    PTR_LOOP_1050_4cce = null_mut();
    PTR_LOOP_1050_4cd4 = null_mut();
    PTR_LOOP_1050_4d40 = null_mut();
    PTR_LOOP_1050_4d46 = null_mut();
    _PTR_LOOP_1050_4d48 = 0;
    _PTR_LOOP_1050_4d4e = 0;
    _PTR_LOOP_1050_4d54 = 0;
    PTR_LOOP_1050_4d5a = 0x44a4;
    PTR_LOOP_1050_4d5c = &DAT_1050_1050;
    PTR_LOOP_1050_4d5e = DAT_1050_44aa;
    PTR_LOOP_1050_4d60 = 0x4496;
    PTR_LOOP_1050_4d62 = &DAT_1050_1050;
    PTR_LOOP_1050_4d66 = 0x4496;
    PTR_LOOP_1050_4d68 = &DAT_1050_1050;
    PTR_LOOP_1050_4d64 = DAT_1050_44a2;
    PTR_LOOP_1050_4d6a = DAT_1050_44a2;
    _PTR_LOOP_1050_4d6c = 0;
    _PTR_LOOP_1050_4d72 = 0;
    PTR_LOOP_1050_4d70 = 0xffff;
    PTR_LOOP_1050_4d76 = 0xffff;
    _PTR_LOOP_1050_4d78 = 0;
    _PTR_LOOP_1050_4d7e = 0;
    PTR_LOOP_1050_4d84 = 0x4496;
    PTR_LOOP_1050_4d86 = &DAT_1050_1050;
    PTR_LOOP_1050_4d88 = DAT_1050_44a2;
    PTR_LOOP_1050_4d8a = 0x4496;
    PTR_LOOP_1050_4d8c = &DAT_1050_1050;
    PTR_LOOP_1050_4d8e = DAT_1050_44a2;
    _PTR_LOOP_1050_4d90 = 0;
    _PTR_LOOP_1050_4d96 = 0;
    _PTR_LOOP_1050_4d9c = 0;
    _PTR_LOOP_1050_4da2 = 0;
    _PTR_LOOP_1050_4da8 = 0;
    _PTR_LOOP_1050_4dae = 0;
    _PTR_LOOP_1050_4db4 = 0;
    _PTR_LOOP_1050_4dba = 0;
    _PTR_LOOP_1050_4dc0 = 0;
    _PTR_LOOP_1050_4dc6 = 0;
    _PTR_LOOP_1050_4dcc = 0;
    _PTR_LOOP_1050_4dd2 = 0;
    _PTR_LOOP_1050_4dd8 = 0;
    _PTR_LOOP_1050_4dde = 0;
    _PTR_LOOP_1050_4de4 = 0;
    _PTR_LOOP_1050_4dea = 0;
    _PTR_LOOP_1050_4df0 = 0;
    _PTR_LOOP_1050_4df6 = 0;
    _PTR_LOOP_1050_4dfc = 0;
    _PTR_LOOP_1050_4e02 = 0;
    _PTR_LOOP_1050_4e08 = 0;
    _PTR_LOOP_1050_4e0e = 0;
    _PTR_LOOP_1050_4e14 = 0;
    _PTR_LOOP_1050_4e1a = 0;
    _PTR_LOOP_1050_4e20 = 0;
    _PTR_LOOP_1050_4e26 = 0;
    _PTR_LOOP_1050_4e2c = 0;
    PTR_LOOP_1050_4d4c = null_mut();
    PTR_LOOP_1050_4d52 = null_mut();
    PTR_LOOP_1050_4d58 = null_mut();
    PTR_LOOP_1050_4d7c = null_mut();
    PTR_LOOP_1050_4d82 = null_mut();
    PTR_LOOP_1050_4d94 = null_mut();
    PTR_LOOP_1050_4d9a = null_mut();
    PTR_LOOP_1050_4da0 = null_mut();
    PTR_LOOP_1050_4da6 = null_mut();
    PTR_LOOP_1050_4dac = null_mut();
    PTR_LOOP_1050_4db2 = null_mut();
    PTR_LOOP_1050_4db8 = null_mut();
    PTR_LOOP_1050_4dbe = null_mut();
    PTR_LOOP_1050_4dc4 = null_mut();
    PTR_LOOP_1050_4dca = null_mut();
    PTR_LOOP_1050_4dd0 = null_mut();
    PTR_LOOP_1050_4dd6 = null_mut();
    PTR_LOOP_1050_4ddc = null_mut();
    PTR_LOOP_1050_4de2 = null_mut();
    PTR_LOOP_1050_4de8 = null_mut();
    PTR_LOOP_1050_4dee = null_mut();
    PTR_LOOP_1050_4df4 = null_mut();
    PTR_LOOP_1050_4dfa = null_mut();
    PTR_LOOP_1050_4e00 = null_mut();
    PTR_LOOP_1050_4e06 = null_mut();
    PTR_LOOP_1050_4e0c = null_mut();
    PTR_LOOP_1050_4e12 = null_mut();
    PTR_LOOP_1050_4e18 = null_mut();
    PTR_LOOP_1050_4e1e = null_mut();
    PTR_LOOP_1050_4e24 = null_mut();
    PTR_LOOP_1050_4e2a = null_mut();
    PTR_LOOP_1050_4e30 = null_mut();
    puVar3 = 0x4e32;
    //   for (iVar2 = 0x1b; iVar2 != 0; iVar2 += -1)
    for iVar2 in 0x1b..0 {
        puVar1 = puVar3;
        puVar3 = puVar3 + 1;
        *puVar1 = 0;
    }
    return;
}
