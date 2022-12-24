use crate::win_ui;


pub unsafe fn cleanup_ui_op_1020_1038(param_1: *mut astruct_868) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_868;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    DestroyIcon16(iVar4.hicon_0xc2);
    iVar4.hicon_0xc2 = 0;
    iVar4.field8_0x8 = 0;
    puVar1 = iVar4.field241_0xf6;
    uVar2 = iVar4.field242_0xf8;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(s_tile2_bmp_1050_1538, puVar1, uVar2, 1);
    }
    iVar4.field241_0xf6 = 0;
    pass1_1010_1dda(iVar4.field240_0xf2);
    iVar4.field240_0xf2 = 0;
    return;
}


pub unsafe fn struct_1020_1738(param_1: *mut Struct57, mut param_2: u16, mut param_3: u32) {
    let mut iVar1: *mut Struct57;
    let mut uVar1: *mut Struct57;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfcd, (param_3 + 0x8));
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 1) = 0;
    iVar1[0x1].field2_0x4 = 0;
    iVar1[0x1].field4_0x8 = 0;
    param_1.field0_0x0 = 0x1e7a;
    iVar1.field1_0x2 = 0x1020;
    return;
}


// WARNING: Unable to use type for symbol puVar2

pub unsafe fn pass1_1020_1d8e(mut param_1: u32, mut param_2: u32) {
    pt_in_rect_1010_4e08((param_1 + 0x8e), param_2, (param_2 >> 0x10));
    return;
}

// WARNING: Unable to use type for symbol uVar1

pub unsafe fn pass1_1020_1da8(mut param_1: i16, mut param_2: u16, param_3: *mut StructB) -> u16 {
    let mut uVar2: u32;
    let mut struct_b_1: *mut StructB;
    let mut uVar3: u16;
    let mut uVar1: u32;

    uVar3 = (param_3 >> 0x10);
    struct_b_1 = param_3;
    uVar1 = &struct_b_1[0x7].field1_0x2;
    if ((uVar1 + 0x30) == 1) {
        return 0x1;
    }
    uVar2 = &struct_b_1[0x7].field1_0x2;
    if (((uVar2 + 0x30) < 0x3) && (
        pass1_1010_4df0(param_2, &struct_b_1[0x7].field1_0x2),
        param_1 == 0,
    )) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn pass1_1020_1e54(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    ui_cleanup_op_1040_782c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1020_1eea(
    param_1: *mut u8,
    param_2: *mut astruct_663,
    param_3: *mut StructB,
    param_4: HWND16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut iVar3: *mut astruct_663;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffec: u32;
    let mut ppuVar5: *mut *mut u8;

    uVar3 = (param_2 >> 0x10);
    iVar3 = param_2;
    param_2 = 0x389a;
    iVar3.field2_0x2 = 0x1008;
    param_2 = 0x3aa8;
    iVar3.field2_0x2 = 0x1008;
    iVar3.field3_0x4 = param_4;
    param_2 = 0x3ab0;
    iVar3.field2_0x2 = 0x1008;
    iVar3.field4_0x6 = null_mut();
    iVar3.field5_0xa = param_3;
    param_2 = 0x2518;
    iVar3.field2_0x2 = 0x1020;
    ppuVar5 = CONCAT22((in_stack_0000ffec >> 0x10), 0x39);
    puVar4 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        ppuVar5,
        in_stack_0000fe96,
        in_stack_0000ffba,
        in_stack_0000ffc0,
        in_stack_0000ffc4,
    );
    uVar2 = (puVar4 >> 0x10);
    iVar3.field4_0x6 = puVar4;
    (&iVar3.field4_0x6 + 0x2) = uVar2;
    ppcVar1 = (*iVar3.field4_0x6 + 0x4);
    (**ppcVar1)(
        0x1010,
        &iVar3.field4_0x6,
        uVar2,
        0x0,
        param_2,
        (ppuVar5 >> 0x10),
    );
    return;
}

pub unsafe fn pass1_1020_1f74(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x2518;
    iVar1.address_offset_field_0x2 = 0x1020;
    pass1_1010_1ea6(&iVar1.field_0x6, (param_1 & 0xffff | uVar1 << 0x10));
    param_1.address_offset_field_0x0 = 0x3ab0;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    return;
}
