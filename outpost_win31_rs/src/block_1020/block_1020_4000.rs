


pub unsafe fn draw_rect_1020_40ce(
    mut param_1: u32,
    mut param_2: i16,
    mut param_3: i16,
    hdc16_param_4: HDC16,
    mut param_5: u16,
) {
    let mut pen_handle: HPEN16;
    let mut brush_handle_1: HGDIOBJ16;
    let mut right: i16;
    let mut bottom: i16;
    let mut unaff_SS: u16;
    let mut hdc: HDC16;
    let mut local_6: i16;
    let mut local_4: i16;
    let mut hdc16_var_fff2: HDC16;
    let mut iVar1: i16;

    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (param_1 + 0x10)),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    pass1_1008_3e94(
        param_1,
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    iVar1 = (param_1 + 0xa);
    Ellipse16(
        iVar1 + local_6 + param_2,
        iVar1 + local_4 + param_3,
        (local_6 - (param_1 + 0xa)) + param_2,
        (local_4 - (param_1 + 0xa)) + param_3,
        hdc16_param_4,
    );
    if ((*(param_1 + 0xe) & 1) != 0) {
        brush_handle_1 = GetStockObject16(HOLLOW_BRUSH);
        SelectObject16(brush_handle_1, hdc16_var_fff2);
        hdc = hdc16_param_4;
        pen_handle = CreatePen16(0x10000f9, 0x1, 0x0);
        SelectObject16(pen_handle, hdc);
        right = local_4 + param_3 + 0x5;
        bottom = local_6 + param_2 + 0x5;
        Rectangle16(
            bottom,
            right,
            local_6 + param_2 -0x5,
            local_4 + param_3 -0x5,
            hdc16_param_4,
        );
        brush_handle_1 = GetStockObject16(WHITE_BRUSH);
        SelectObject16(brush_handle_1, right);
        brush_handle_1 = GetStockObject16(WHITE_PEN);
        brush_handle_1 = SelectObject16(brush_handle_1, bottom);
        DeleteObject16(brush_handle_1);
    }
    return;
}

/*
Unable to decompile 'unk_draw_op_1020_41c8'
Cause:
Low-level Error: Symbol $$undef0000000d extends beyond the end of the address space
*/
pub unsafe fn destroy_cursor_1020_42f4(param_1: *mut StructD) {
    let mut struct_1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    struct_1 = param_1;
    param_1.address_offset_field_0x0 = 0x623c;
    struct_1.address_offset_field_0x2 = 0x1020;
    struct_1.field_0xe2 = 0x62d8;
    struct_1.field_0xe4 = 0x1020;
    if (struct_1[0x1].field13_0x18 != 0) {
        DestroyMenu16(struct_1[0x1].field13_0x18);
    }
    DestroyCursor16(struct_1[0x1].address_offset_field_0x2);
    DestroyCursor16(struct_1[0x1].hfile_0x4);
    pass1_1020_808e(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
