use crate::draw_ops;
use crate::draw_ops::draw_e;

pub fn pass1_1020_01d8(
    param_1: *mut Struct20,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u32,
) {
    let mut in_stack_0000ffd6: u16;

    unk_draw_op_1008_61b2(in_stack_0000ffd6, param_1, param_2, param_6, param_7);
    (param_1 + 0xe2) = 0;
    (param_1 + 0xe6) = 0;
    (param_1 + 0xea) = param_5;
    (param_1 + 0xec) = param_4;
    (param_1 + 0xee) = param_3;
    param_1.offset_0x0 = 0x45a;
    (param_1 + 0x2) = 0x1020;
    return;
}

pub fn pass1_1020_022c(struct_param_1: *mut astruct_29) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_29;
    let mut uVar4: u16;

    uVar4 = (struct_param_1 >> 0x10);
    iVar4 = struct_param_1;
    struct_param_1.field0_0x0 = 0x45a;
    iVar4.field1_0x2 = 0x1020;
    puVar1 = &iVar4.field228_0xe6;
    uVar2 = (&iVar4.field228_0xe6 + 2);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1008_57c4((struct_param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
    struct_param_1.field0_0x0 = 0x380a;
    iVar4.field1_0x2 = 0x1008;
    struct_param_1.field0_0x0 = 0x389a;
    iVar4.field1_0x2 = 0x1008;
    return;
}




pub fn draw_op_1020_041e(mut param_1: u32) {
    draw_e::fill_rect_1020_065e((param_1 + 0xe6));
    return;
}



pub fn pass1_1020_04f6(param_1: *mut u8, param_2: *mut astruct_662, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar4: *mut astruct_662;
    let mut uVar5: u16;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut uVar7: u16;
    let mut in_stack_0000ffe0: u32;
    let mut ppuVar8: *mut *mut u8;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    param_2 = 0x389a;
    iVar4.field2_0x2 = 0x1008;
    param_2 = 0x3aa8;
    iVar4.field2_0x2 = 0x1008;
    iVar4.field3_0x4 = param_3;
    param_2 = 0x3ab0;
    iVar4.field2_0x2 = 0x1008;
    iVar4.field4_0x6 = null_mut();
    iVar4.field5_0xa = 0;
    iVar4.field6_0xc = 0;
    iVar4.field7_0xe = 0;
    iVar4.field8_0x10 = 0;
    param_2 = 0x75a;
    iVar4.field2_0x2 = 0x1020;
    ppuVar8 = CONCAT22((in_stack_0000ffe0 >> 0x10), 1);
    puVar6 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        ppuVar8,
        in_stack_0000fe8a,
        in_stack_0000ffae,
        in_stack_0000ffb4,
        in_stack_0000ffb8,
    );
    paVar4 = (paVar4 & 0xffff0000 | puVar6 >> 0x10);
    iVar4.field4_0x6 = puVar6;
    uVar3 = (puVar6 >> 0x10);
    (&iVar4.field4_0x6 + 0x2) = uVar3;
    uVar7 = &iVar4.field4_0x6;
    ppcVar1 = (*iVar4.field4_0x6 + 0x4);
    (**ppcVar1)(0x1010, uVar7, uVar3, 0x0, param_2, (ppuVar8 >> 0x10));
    puVar6 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        CONCAT22(uVar7, 0x48),
        in_stack_0000fe80,
        in_stack_0000ffa4,
        in_stack_0000ffaa,
        in_stack_0000ffae,
    );
    iVar2 = puVar6;
    iVar4.field5_0xa = (iVar2 + 0xa);
    iVar4.field6_0xc = (iVar2 + 0xc);
    pass1_1008_3e94(
        (puVar6 & 0xffff0000 | (iVar2 + 0xe)),
        (param_2 & 0xffff0000 | ZEXT24(&iVar4.field8_0x10)),
        (param_2 & 0xffff0000 | ZEXT24(&iVar4.field7_0xe)),
    );
    return;
}

pub fn pass1_1020_05d6(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x75a;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (&iVar1.field_0x6 != 0) {
        pass1_1010_1ea6(&iVar1.field_0x6, (param_1 & 0xffff | uVar1 << 0x10));
    }
    param_1.address_offset_field_0x0 = 0x3ab0;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    return;
}


pub fn pass1_1020_08b6(param_1: *mut Struct20, mut param_2: u16, mut param_3: u32) {
    let mut iVar1: *mut Struct20;
    let mut uVar1: u16;
    let mut paVar2: *mut Struct20;
    let mut in_stack_0000ffd6: u16;

    paVar2 = unk_draw_op_1008_61b2(in_stack_0000ffd6, param_1, 0x1, param_2, param_3);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1[0x1].field2_0x4 = 0;
    (&iVar1[0x1].field2_0x4 + 0x2) = 0;
    param_1.offset_0x0 = 0xb0e;
    iVar1.base_0x2 = 0x1020;
    win_1008_5c5c(0x0, (paVar2 >> 0x10), _u16_1050_02a0, 0x1d4);
    return;
}


pub fn struct_1020_0baa(param_1: *mut u8, param_2: *mut u16, mut param_3: u16) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut iVar2: *mut Struct276;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe82: u16;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u16;
    let mut uVar6: u16;
    let mut in_stack_0000ffe2: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    uVar2 = (param_2 >> 0x10);
    iVar2 = param_2;
    *param_2 = 0x389a;
    iVar2.field2_0x2 = 0x1008;
    *param_2 = 0x3aa8;
    iVar2.field2_0x2 = 0x1008;
    iVar2.field3_0x4 = param_3;
    *param_2 = 0x3ab0;
    iVar2.field2_0x2 = 0x1008;
    iVar2.field4_0x6 = 0;
    iVar2.field6_0xa = 0;
    iVar2.field7_0xc = 0;
    *param_2 = 0xdbc;
    iVar2.field2_0x2 = 0x1020;
    puVar3 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe2, 0x7),
        in_stack_0000fe8a,
        in_stack_0000ffae,
        in_stack_0000ffb4,
        in_stack_0000ffb8,
    );
    iVar2.field4_0x6 = puVar3;
    iVar2.field5_0x8 = (puVar3 >> 0x10);
    puVar5 = &iVar2.field6_0xa;
    puVar4 = &iVar2.field7_0xc;
    uVar6 = uVar2;
    puVar3 = mixed_1010_20ba(
        (paVar1 & 0xffff0000 | puVar3 >> 0x10),
        _u16_1050_0ed0,
        CONCAT22(puVar4, 0x48),
        in_stack_0000fe82,
        in_stack_0000ffa6,
        in_stack_0000ffac,
        in_stack_0000ffb0,
    );
    pass1_1008_3e94(
        (puVar3 & 0xffff0000 | (puVar3 + 0xe)),
        CONCAT22(uVar2, puVar4),
        CONCAT22(uVar6, puVar5),
    );
    return;
}

pub fn pass1_1020_0d82(param_1: *mut StructD, param_2: u8) -> *mut StructD {
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

pub fn pass1_1020_0dc4(
    param_1: *mut StructA,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut ppcVar1: *mut *mut c_char;
    let iVar1: *mut StructA;
    let mut uVar2: u16;
    let mut in_stack_0000fe16: u16;
    let mut in_stack_0000fe1a: u16;
    let mut in_stack_0000fe59: u16;
    let mut in_stack_0000fe64: u16;
    let mut in_stack_0000fe68: u16;
    let mut in_stack_0000ff40: u16;
    let mut in_stack_0000ff44: u16;
    let mut in_stack_0000ff48: u16;
    let mut in_stack_0000ff8e: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff96: u16;

    struct_1020_790e(&param_1.field0_0x0, s_PCPOPMENU_1050_4256, param_2, param_3);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1[0x1].field20_0x26 = 0;
    iVar1[0x1].field22_0x2a = 0;
    iVar1[0x1].field25_0x2e = 0;
    param_1.field0_0x0 = 0x1384;
    iVar1.field1_0x2 = 0x1020;
    ppcVar1 = &iVar1.field60_0x5b;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(ppcVar1)), s_VrMode_1050_4260);
    iVar1.field140_0xac = 0x44c00000;
    window_op_1020_10a0(
        ppcVar1,
        param_4,
        (param_1 & 0xffff | uVar2 << 0x10),
        param_5,
        param_6,
        in_stack_0000fe16,
        in_stack_0000fe1a,
        in_stack_0000fe59,
        in_stack_0000fe68,
        in_stack_0000ff40,
        in_stack_0000ff44,
        in_stack_0000ff48,
        in_stack_0000ff8e,
        in_stack_0000ff92,
        in_stack_0000ff96,
        in_stack_0000fe64,
    );
    return;
}
