

pub unsafe fn struct_1018_e100(param_1: *mut u8, param_2: *mut u16, mut param_3: u16) -> *mut u16 {
    let mut in_register_0000000a: u16;
    let mut iVar1: *mut astruct_268;
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffec: u32;

    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    *param_2 = 0x389a;
    iVar1.field2_0x2 = 0x1008;
    *param_2 = 0x3aa8;
    iVar1.field2_0x2 = 0x1008;
    iVar1.field3_0x4 = param_3;
    *param_2 = 0x3ab0;
    iVar1.field2_0x2 = 0x1008;
    iVar1.field4_0x6 = 0;
    *param_2 = 0xe228;
    iVar1.field2_0x2 = 0x1018;
    puVar2 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22((in_stack_0000ffec >> 0x10), 0x36),
        in_stack_0000fe96,
        in_stack_0000ffba,
        in_stack_0000ffc0,
        in_stack_0000ffc4,
    );
    iVar1.field4_0x6 = puVar2;
    iVar1.field5_0x8 = (puVar2 >> 0x10);
    return param_2;
}


pub unsafe fn pass1_1018_e230(
    mut param_1: u16,
    param_2: *mut astruct_20,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut astruct_20;
    let mut unaff_BP: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    unk_draw_op_1020_7f7a(param_2, 0x4, CONCAT22(param_4, param_3), param_5);
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field5_0xc = 0;
    iVar2[0x1].field7_0x10 = null_mut();
    param_2.offset_0x0 = 0xe44e;
    iVar2.base_0x2 = 0x1018;
    (iVar2 + 1).offset_0x0 = 0xe4ea;
    iVar2[0x1].base_0x2 = 0x1018;
    puVar4 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x26),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = (puVar4 >> 0x10);
    iVar2[0x1].field7_0x10 = puVar4;
    (&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
    iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
    (&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
    return;
}

pub unsafe fn pass1_1018_e2a0(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0xe44e;
    iVar1.address_offset_field_0x2 = 0x1018;
    iVar1.field_0xe2 = 0xe4ea;
    iVar1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}






pub unsafe fn pass1_1018_e4f2(param_1: *mut astruct_659, param_2: *mut Struct57) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut in_EDX: *mut Struct57;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffca: u32;
    let mut uVar6: u16;
    let mut in_stack_0000fff2: u16;

    uVar6 = SUB42(param_2, 0x0);
    set_struct_op_1020_921c(
        in_EDX,
        CONCAT22(uVar6, param_1),
        (param_2 >> 0x10),
        in_stack_0000ffca,
    );
    param_1.field16_0x14 = 0;
    CONCAT22(uVar6, param_1) = 0xe5d0;
    param_1.field2_0x2 = 0x1018;
    puVar5 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x26),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar3 = (puVar5 >> 0x10);
    param_1.field16_0x14 = puVar5;
    param_1.field17_0x16 = uVar3;
    param_1.field5_0x6 = param_1.field16_0x14;
    param_1.field6_0x8 = uVar3;
    uVar2 = &param_1.field16_0x14;
    puVar4 = &param_1.field_0xa;
    ppcVar1 = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1.field15_0x12 = puVar4;
    draw_op_1020_9364(CONCAT22(uVar6, param_1));
    return;
}

pub unsafe fn pass1_1018_e57a(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0xe5d0;
    iVar1.address_offset_field_0x2 = 0x1018;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1dda(iVar1.field12_0x14);
    }
    palette_op_1020_92c4(param_1);
    return;
}



pub unsafe fn pass1_1018_e5dc(
    mut param_1: u16,
    param_2: *mut astruct_20,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut astruct_20;
    let mut unaff_BP: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    unk_draw_op_1020_7f7a(param_2, 0x9, CONCAT22(param_4, param_3), param_5);
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field5_0xc = 0;
    iVar2[0x1].field7_0x10 = null_mut();
    param_2.offset_0x0 = 0xe790;
    iVar2.base_0x2 = 0x1018;
    (iVar2 + 1).offset_0x0 = 0xe82c;
    iVar2[0x1].base_0x2 = 0x1018;
    puVar4 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0xa),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = (puVar4 >> 0x10);
    iVar2[0x1].field7_0x10 = puVar4;
    (&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
    iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
    (&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
    return;
}

pub unsafe fn pass1_1018_e64c(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0xe790;
    iVar1.address_offset_field_0x2 = 0x1018;
    iVar1.field_0xe2 = 0xe82c;
    iVar1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}




pub unsafe fn pass1_1018_e834(param_1: *mut StructA, mut param_2: u16, param_3: *mut Struct57) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffca: u32;
    let mut in_stack_0000fff2: u16;

    set_struct_op_1020_921c(param_3, param_1, param_2, in_stack_0000ffca);
    (param_1 + 0x14) = 0;
    param_1.field0_0x0 = 0xe912;
    (param_1 + 0x2) = 0x1018;
    puVar5 = mixed_1010_20ba(
        param_3,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0xa),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar3 = (puVar5 >> 0x10);
    (param_1 + 0x14) = puVar5;
    (param_1 + 0x16) = uVar3;
    (param_1 + 0x6) = (param_1 + 0x14);
    (param_1 + 0x8) = uVar3;
    uVar2 = (param_1 + 0x14);
    iVar4 = param_1 + 0xa;
    ppcVar1 = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    (param_1 + 0x12) = iVar4;
    draw_op_1020_9364((param_1 & 0xffff | param_1 << 0x10));
    return;
}

pub unsafe fn pass1_1018_e8bc(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0xe912;
    iVar1.address_offset_field_0x2 = 0x1018;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1dda(iVar1.field12_0x14);
    }
    palette_op_1020_92c4(param_1);
    return;
}


pub unsafe fn pass1_1018_e91e(
    mut param_1: u16,
    param_2: *mut astruct_20,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_20;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut uVar7: u16;
    let mut in_stack_0000fffa: u16;
    let mut iVar7: *mut astruct_20;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    iVar7 = param_2;
    uVar7 = (param_2 >> 0x10);
    unk_draw_op_1020_7f7a(param_2, 0x3, CONCAT22(param_4, param_3), param_5);
    iVar7[0x1].field5_0xc = 0;
    iVar7[0x1].field7_0x10 = null_mut();
    iVar7[0x1].field8_0x14 = 0;
    param_2.offset_0x0 = 0xebd0;
    iVar7.base_0x2 = 0x1018;
    (iVar7 + 1).offset_0x0 = 0xec6c;
    iVar7[0x1].base_0x2 = 0x1018;
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fffa, 0x28),
        in_stack_0000fea2,
        in_stack_0000ffc6,
        in_stack_0000ffcc,
        in_stack_0000ffd0,
    );
    iVar7[0x1].field7_0x10 = puVar6;
    uVar4 = (puVar6 >> 0x10);
    (&iVar7[0x1].field7_0x10 + 0x2) = uVar4;
    iVar7[0x1].field2_0x4 = &iVar7[0x1].field7_0x10;
    (&iVar7[0x1].field2_0x4 + 0x2) = uVar4;
    puVar6 = mixed_1010_20ba(
        (paVar5 & 0xffff0000 | puVar6 >> 0x10),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fffa, 0x32),
        in_stack_0000fea2,
        in_stack_0000ffc6,
        in_stack_0000ffcc,
        in_stack_0000ffd0,
    );
    iVar7[0x1].field8_0x14 = puVar6;
    iVar7[0x1].field9_0x16 = (puVar6 >> 0x10);
    if (param_2.is_null()) {
        paVar3 = null_mut();
        uVar4 = 0;
    } else {
        paVar3 = iVar7 + 1;
        uVar4 = uVar7;
    }
    uVar1 = &iVar7[0x1].field8_0x14;
    ppcVar2 = (*&iVar7[0x1].field8_0x14 + 0x4);
    (**ppcVar2)(0x1010, uVar1, (uVar1 >> 0x10), 0xb, paVar3, uVar4);
    return;
}

pub unsafe fn pass1_1018_e9de(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0xebd0;
    iVar1.address_offset_field_0x2 = 0x1018;
    iVar1.field_0xe2 = 0xec6c;
    iVar1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}




pub unsafe fn pass1_1018_ec74(
    mut param_1: u16,
    param_2: *mut astruct_661,
    mut param_3: i16,
    mut param_4: u16,
) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut unaff_SI: u16;
    let mut puVar8: *mut u16;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffba: u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut paVar13: *mut astruct_661;
    let mut iVar14: i16;

    set_struct_op_1020_921c(
        param_1,
        CONCAT22(param_3, param_2),
        param_4,
        in_stack_0000ffba,
    );
    param_2.field15_0x14 = null_mut();
    pass1_1008_3e38(CONCAT22(param_3, &param_2.field_0x18));
    puVar8 = pass1_1008_3e38(CONCAT22(param_3, &param_2.field_0x1e));
    paVar6 = CONCAT22(in_register_0000000a, (puVar8 >> 0x10));
    param_2.field28_0x24 = 0;
    CONCAT22(param_3, param_2) = 0x1cc;
    param_2.field2_0x2 = 0x1020;
    puVar9 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x28),
        in_stack_0000fe8a,
        in_stack_0000ffae,
        in_stack_0000ffb4,
        in_stack_0000ffb8,
    );
    uVar7 = (paVar6 >> 0x10);
    param_2.field15_0x14 = puVar9;
    uVar4 = (puVar9 >> 0x10);
    (&param_2.field15_0x14 + 0x2) = uVar4;
    uVar12 = 0;
    uVar11 = &param_2.field15_0x14;
    ppcVar2 = (*param_2.field15_0x14 + 0x4);
    paVar13 = param_2;
    iVar14 = param_3;
    (**ppcVar2)();
    param_2.field5_0x6 = param_2.field15_0x14;
    puVar1 = param_2.field15_0x14;
    pass1_1010_2b50(
        puVar1,
        (puVar1 >> 0x10),
        CONCAT22(param_3, &param_2.field_0x18),
    );
    uVar10 = pass1_1010_2b66(param_2.field15_0x14);
    iVar5 = (uVar10 >> 0x10);
    paVar6 = CONCAT22(uVar7, iVar5);
    param_2.field28_0x24 = uVar10;
    param_2.field29_0x26 = iVar5;
    puVar1 = param_2.field15_0x14;
    puVar1 = (puVar1 + 0xa);
    uVar3 = CONCAT22(param_3, &param_2.field_0xa);
    uVar7 = SUB42(puVar1, 0x0);
    ppcVar2 = (*puVar1 + 0x8);
    (**ppcVar2)(
        0x1010,
        uVar7,
        (puVar1 >> 0x10),
        uVar3,
        uVar11,
        uVar4,
        uVar12,
        paVar13,
        iVar14,
    );
    param_2.field14_0x12 = uVar3;
    draw_op_1020_9364(CONCAT22(param_3, param_2));
    puVar9 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(uVar7, 0x48),
        in_stack_0000fe78,
        in_stack_0000ff9c,
        in_stack_0000ffa2,
        in_stack_0000ffa6,
    );
    pass1_1008_3f62(
        CONCAT22(param_3, &param_2.field_0x1e),
        (puVar9 & 0xffff0000 | (puVar9 + 0xe)),
    );
    pass1_1008_3f32(
        CONCAT22(param_3, &param_2.field_0x18),
        CONCAT22(param_3, &param_2.field_0x1e),
    );
    return;
}

pub unsafe fn pass1_1018_ed98(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x1cc;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1ea6(iVar1.field12_0x14, (param_1 & 0xffff | uVar1 << 0x10));
        pass1_1010_1dda(iVar1.field12_0x14);
    }
    palette_op_1020_92c4(param_1);
    return;
}
