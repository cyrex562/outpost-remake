

use crate::draw_ops;
use crate::draw_ops::draw_a;


pub fn struct_1020_7554(
    mut param_1: u16,
    param_2: *mut Struct20,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut Struct20;
    let mut unaff_BP: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    draw_a::unk_draw_op_1020_7f7a(param_2, 0x5, CONCAT22(param_4, param_3), param_5);
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field5_0xc = 0;
    iVar2[0x1].field7_0x10 = null_mut();
    param_2.offset_0x0 = 0x7780;
    iVar2.base_0x2 = 0x1020;
    (iVar2 + 1).offset_0x0 = 0x781c;
    iVar2[0x1].base_0x2 = 0x1020;
    puVar4 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x25),
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

pub fn pass1_1020_75c4(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x7780;
    iVar1.address_offset_field_0x2 = 0x1020;
    iVar1.field_0xe2 = 0x781c;
    iVar1.field_0xe4 = 0x1020;
    pass1_1020_808e(param_1);
    return;
}





pub fn pass1_1020_7824(
    mut param_1: u16,
    param_2: *mut astruct_666,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffca: u32;
    let mut in_stack_0000fff2: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    set_struct_op_1020_921c(
        param_1,
        CONCAT22(param_3, param_2),
        param_4,
        in_stack_0000ffca,
    );
    param_2.field16_0x14 = 0;
    CONCAT22(param_3, param_2) = 0x7902;
    param_2.field2_0x2 = 0x1020;
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x25),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar3 = (puVar6 >> 0x10);
    param_2.field16_0x14 = puVar6;
    param_2.field17_0x16 = uVar3;
    param_2.field5_0x6 = param_2.field16_0x14;
    param_2.field6_0x8 = uVar3;
    uVar2 = &param_2.field16_0x14;
    puVar4 = &param_2.field_0xa;
    ppcVar1 = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_2.field15_0x12 = puVar4;
    draw_op_1020_9364(CONCAT22(param_3, param_2));
    return;
}

pub fn pass1_1020_78ac(param_1: *mut u16) {
    let mut iVar1: i16;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x7902;
    (iVar1 + 0x2) = 0x1020;
    if ((iVar1 + 0x14) != 0) {
        pass1_1010_1dda((iVar1 + 0x14));
    }
    palette_op_1020_92c4(param_1);
    return;
}


pub fn struct_1020_790e(
    param_1: *mut u16,
    param_2: *mut c_char,
    mut param_3: u16,
    mut param_4: u32,
) {
    let mut iVar1: *mut Struct271;
    let mut uVar1: u16;

    unk_draw_op_1008_7f62(param_1, param_3, param_4);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field223_0xe0 = 0;
    iVar1.field224_0xe4 = 0;
    iVar1.field225_0xe8 = 0;
    iVar1.field226_0xec = 0;
    iVar1.field227_0xee = param_2;
    *param_1 = 0x7b86;
    iVar1.field2_0x2 = 0x1020;
    return;
}

pub fn pass1_1020_79ae() -> u16 {
    return 0x1;
}

pub fn string_1020_79b4(mut param_1: u32, mut param_2: i16, param_3: *mut c_char) {
    let mut in_DX: u16;

    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)), param_3);
    if (param_2 != 0) {
        draw_a::draw_op_1020_7cc8(in_DX, (param_1 + 0xe8));
    }
    return;
}

pub fn pass1_1020_79e4(mut param_1: u32) {
    let mut in_DX: u16;

    draw_a::draw_op_1020_7cc8(in_DX, (param_1 + 0xe8));
    return;
}
