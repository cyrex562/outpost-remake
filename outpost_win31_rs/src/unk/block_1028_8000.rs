




pub fn pass1_1028_837e(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0xf9f);
    param_1.offset_0x0 = 0x84ba;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCFillResources_1050_500c,
    );
    return param_1;
}


pub fn pass1_1028_84ca(
    param_1: *mut astruct_97,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut offset: u16;
    let mut iVar2: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x3e7);
    uVar1 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar2.field259_0x108 = param_5;
    iVar2.field_0x10a = param_4;
    iVar2.field262_0x10c = param_3;
    iVar2.field263_0x10e = param_2;
    param_1.offset_0x0 = 0x8688;
    // just 0x1028
    iVar2.segment_0x2 = 0x1028;
    if (iVar2.field259_0x108 == 1) {
        // just 0x501c
        offset = s_max_1050_501c;
    } else {
        // just 0x5020
        offset = s_min_1050_5020;
    }
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar2.string_0x8)),
        s_SCForceMorale__s_for_colony__08l_1050_5024,
        offset,
    );
    return;
}




pub fn pass1_1028_8698(
    param_1: *mut astruct_97,
    mut param_2: u32,
    mut param_3: u32,
) -> *mut astruct_97 {
    pass1_1028_6af2(param_1, param_2, param_3);
    param_1.offset_0x0 = 0x87e0;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}


pub fn pass1_1028_86c2(param_1: *mut StructD, mut param_2: u32) {
    let mut paVar1: *mut astruct_67;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc2: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;

    uVar7 = 0;
    iVar8 = 0x1d;
    uVar5 = 0x1;
    uVar6 = 0;
    uVar3 = 0;
    iVar4 = 0;
    uVar2 = 0;
    paVar1 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        0x37,
        in_stack_0000fe94,
        in_stack_0000ffb8,
        in_stack_0000ffbe,
        in_stack_0000ffc2,
    );
    post_win_msg_1008_a0e4(
        paVar1,
        CONCAT22(uVar3, uVar2),
        iVar4,
        uVar5,
        CONCAT22(uVar7, uVar6),
        iVar8,
    );
    pass1_1028_6b2c(param_2);
    return;
}


pub fn struct_op_1028_87f0(
    param_1: *mut astruct_97,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut u32,
    mut param_6: u16,
    mut param_7: u32,
    mut param_8: u32,
) {
    let mut iVar1: *mut astruct_97;
    let mut puVar1: *mut astruct_97;

    struct_op_1028_d1dc(param_1, 0x3e8);
    puVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_8;
    iVar1.field262_0x10c = param_7;
    iVar1.field264_0x110 = 0;
    iVar1.field265_0x114 = *param_5;
    iVar1.field266_0x118 = (param_5 + 1);
    iVar1.field267_0x11a = param_4;
    iVar1.field268_0x11c = param_3;
    iVar1.field269_0x11e = param_2;
    iVar1.field271_0x122 = 0;
    iVar1.field270_0x120 = 0;
    param_1.offset_0x0 = 0x8d8e;
    iVar1.segment_0x2 = 0x1028;
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCInternalPutBldg_site_0x_08lx__b_1050_5046,
        param_8,
    );
    return;
}
pub fn struct_op_1028_8888(
    param_1: *mut astruct_97,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u32,
    mut param_5: u16,
    mut param_6: u32,
    mut param_7: u32,
    mut param_8: u32,
) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x3e8);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_8;
    iVar1.field262_0x10c = param_7;
    iVar1.field264_0x110 = param_6;
    iVar1.field265_0x114 = *param_4;
    iVar1.field266_0x118 = (param_4 + 1);
    iVar1.field267_0x11a = param_3;
    iVar1.field268_0x11c = 0;
    iVar1.field269_0x11e = param_2;
    iVar1.field271_0x122 = 0;
    iVar1.field270_0x120 = 0;
    param_1.offset_0x0 = 0x8d8e;
    // just 0x1028
    iVar1.segment_0x2 = 0x1028;
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCInternalPutBldg2_site_0x_08lx__1050_506f,
        param_8,
    );
    return;
}




pub fn pass1_1028_8d9e(
    param_1: *mut astruct_97,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x3e8);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_4;
    iVar1.field262_0x10c = param_3;
    iVar1.field264_0x110 = param_2;
    iVar1.field265_0x114 = 0;
    param_1.offset_0x0 = 0x8fb0;
    // just 0x1028
    iVar1.segment_0x2 = 0x1028;
    return;
}
pub fn pass1_1028_8dec(param_1: *mut u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x8fb0;
    (iVar1 + 0x2) = 0x1028;
    fn_ptr_1000_17ce(*(iVar1 + 0x114));
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}








pub fn pass1_1028_8fc0(
    param_1: *mut astruct_97,
    mut param_2: u32,
    mut param_3: u32,
) -> *mut astruct_97 {
    pass1_1028_6af2(param_1, param_2, param_3);
    param_1.offset_0x0 = 0x90d6;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}
