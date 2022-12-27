

pub fn pass1_1028_90e6(param_1: *mut astruct_97, mut param_2: u16) -> *mut astruct_97 {
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x108) = param_2;
    param_1.offset_0x0 = 0x932c;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}




pub fn struct_op_1028_933c(
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
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x3e8);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_8;
    iVar1.field262_0x10c = param_7;
    iVar1.field264_0x110 = 0;
    iVar1.field265_0x114 = *param_5;
    iVar1.field266_0x118 = (param_5 + 1);
    iVar1.field267_0x11a = param_4;
    iVar1.field268_0x11c = param_2;
    iVar1.field270_0x120 = 0;
    iVar1.field269_0x11e = 0;
    iVar1.field271_0x122 = param_3;
    param_1.offset_0x0 = 0x9934;
    iVar1.segment_0x2 = 0x1028;
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,
        param_8,
    );
    return;
}













pub fn struct_1028_9c62(param_1: *mut astruct_97, mut param_2: u16) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, param_2);
    (param_1 + 0x108) = param_2;
    param_1.offset_0x0 = 0x9eb6;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}
