use crate::gui;

pub fn pass1_1018_642e(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut i16,
    mut param_4: i16,
) {
    *param_3 = 0x64 - param_4 >> 0x1;
    return;
}

pub fn pass1_1018_659a(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u16,
) -> u32 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iStack18: i16;
    let mut local_6: i16;
    let mut local_4: i16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    piVar1 = &local_6;
    pass1_1008_3e94(
        param_4,
        CONCAT22(0x1050, piVar1),
        CONCAT22(0x1050, &local_4),
    );
    mem_op_1000_179c(0xc, paVar3);
    uVar2 = SUB42(paVar3, 0x0);
    for iStack18 in 0..0x3 {
        piVar1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4248) + local_4;
        piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x424a) + local_6;
    }
    return CONCAT22(uVar2, piVar1);
}

pub fn pass1_1018_6630(param_1: *mut c_char, pstruct_param_3: *mut astruct_2) {
    let mut paVar1: *mut astruct_812;
    let mut uVar2: u32;
    let mut dialog_id_5: u16;
    let mut uVar3: u16;
    let mut pstruct_2_1: *mut astruct_2;
    let mut uVar4: u16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut uStack8: u16;

    uVar4 = (pstruct_param_3 >> 0x10);
    pstruct_2_1 = pstruct_param_3;
    find_n_load_rsrc_1010_4e9e(pstruct_2_1.field6_0x6);
    if (param_1.is_null() == false) {
        // for (iStack12 = 0; iStack10 = (param_1 >> 0x10), uStack8 = SUB42(param_1,0x0), iStack12 < 0x9;
        // iStack12 += 1)
        iStack12 = 0;
        iStack10 = param_1 >> 0x10;
        uStack8 = SUB42(param_1, 0x0);
        while iStack12 < 0x9 {
            paVar1 = pstruct_2_1.field6_0x6;
            dialog_id_5 = pass1_1010_4f20(paVar1, (paVar1 >> 0x10), iStack12);
            uVar2 = pstruct_2_1.field7_0xa;
            set_window_text_1018_6066(
                uVar2,
                (uVar2 >> 0x10),
                CONCAT22(uStack8, iStack10),
                dialog_id_5,
            );
            uVar3 = str_op_1000_3da4(CONCAT22(uStack8, iStack10));
            param_1 = (param_1 & 0xffff | (iStack10 + uVar3 + 1) << 0x10);
            iStack12 += 1;
        }
    }
    return;
}



pub fn struct_1018_66cc(
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
    let mut uVar2: *mut astruct_20;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    unk_draw_op_1020_7f7a(param_2, 0xa, CONCAT22(param_4, param_3), param_5);
    uVar2 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field5_0xc = 0;
    iVar2[0x1].field7_0x10 = null_mut();
    param_2.offset_0x0 = 0x6880;
    iVar2.base_0x2 = 0x1018;
    (iVar2 + 1).offset_0x0 = 0x691c;
    iVar2[0x1].base_0x2 = 0x1018;
    puVar3 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0xb),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = (puVar3 >> 0x10);
    iVar2[0x1].field7_0x10 = puVar3;
    (&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
    iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
    (&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
    return;
}
pub fn pass1_1018_673c(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x6880;
    iVar1.address_offset_field_0x2 = 0x1018;
    iVar1.field_0xe2 = 0x691c;
    iVar1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}









pub fn pass1_1018_6924(mut param_1: u16, param_2: *mut StructA, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffca: u32;
    let mut in_stack_0000fff2: u16;
    let mut uVar2: u32;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    set_struct_op_1020_921c(param_1, param_2, param_3, in_stack_0000ffca);
    (param_2 + 0x14) = 0;
    param_2.field0_0x0 = 0x6a02;
    (param_2 + 0x2) = 0x1018;
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0xb),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar3 = (puVar6 >> 0x10);
    (param_2 + 0x14) = puVar6;
    (param_2 + 0x16) = uVar3;
    (param_2 + 0x6) = (param_2 + 0x14);
    (param_2 + 0x8) = uVar3;
    uVar2 = (param_2 + 0x14);
    iVar4 = param_2 + 0xa;
    ppcVar1 = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    (param_2 + 0x12) = iVar4;
    draw_op_1020_9364((param_2 & 0xffff | param_2 << 0x10));
    return;
}
pub fn pass1_1018_69ac(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x6a02;
    iVar1.address_offset_field_0x2 = 0x1018;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1dda(iVar1.field12_0x14);
    }
    palette_op_1020_92c4(param_1);
    return;
}

pub fn struct_op_1018_6a0e(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u32,
) -> *mut astruct_20 {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut in_stack_0000ffd6: u16;

    unk_draw_op_1008_61b2(in_stack_0000ffd6, param_1, param_3, param_6, param_7);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xea) = param_5;
    (iVar1 + 0xec) = param_4;
    (iVar1 + 0xee) = param_2;
    (iVar1 + 0xf0) = 0;
    param_1.offset_0x0 = 0x6c66;
    (iVar1 + 0x2) = 0x1018;
    (iVar1 + 0xe0) = 0x1;
    (iVar1 + 0xe2) = 0;
    (iVar1 + 0xe4) = 0;
    (iVar1 + 0xe6) = 0x1df027f;
    return param_1;
}


pub fn struct_1018_6d02(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0xb, 0x9c, 0x8b, param_2, param_3);
    param_1.offset_0x0 = 0xa27e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6d38(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0xc, 0x9d, 0xd0, param_2, param_3);
    param_1.offset_0x0 = 0xb562;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6d6e(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0xd, 0x9e, 0xd1, param_2, param_3);
    param_1.offset_0x0 = 0x9822;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6da4(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0xe, 0x9f, 0xd2, param_2, param_3);
    param_1.offset_0x0 = 0xab06;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6dda(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0xf, 0xa0, 0xd4, param_2, param_3);
    param_1.offset_0x0 = 0xbdea;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6e10(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x10, 0xa1, 0xda, param_2, param_3);
    param_1.offset_0x0 = 0xa0aa;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6e46(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x11, 0xa2, 0xdc, param_2, param_3);
    param_1.offset_0x0 = 0xb38e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6e7c(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x12, 0xa3, 0xd3, param_2, param_3);
    param_1.offset_0x0 = 0x964e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6eb2(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x13, 0xa4, 0xdb, param_2, param_3);
    param_1.offset_0x0 = 0xa932;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6ee8(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x14, 0xa5, 0xa5, param_2, param_3);
    param_1.offset_0x0 = 0xbc16;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6f1e(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x15, 0xa7, 0xb2, param_2, param_3);
    param_1.offset_0x0 = 0x9e3a;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6f54(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x16, 0xa8, 0x0, param_2, param_3);
    param_1.offset_0x0 = 0xb11e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6f8a(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x17, 0xaf, 0xc0, param_2, param_3);
    param_1.offset_0x0 = 0x93de;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6fc0(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x18, 0xb0, 0xc1, param_2, param_3);
    param_1.offset_0x0 = 0xa6c2;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn struct_1018_6ff6(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    struct_op_1018_6a0e(param_1, 0x0, 0x19, 0xb1, 0x80, param_2, param_3);
    param_1.offset_0x0 = 0xb9a6;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}
