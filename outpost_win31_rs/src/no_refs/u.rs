pub fn FUN_1020_3cb4() {
    return;
}

pub fn pass1_1020_4064(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    win_ui_palette_op_1020_3e84(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_4092(param_1: *mut u16) -> *mut u16 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    pass1_1008_3e38(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x6) = 0;
    (iVar1 + 0x8) = 0;
    (iVar1 + 0xa) = 0x1;
    (iVar1 + 0xc) = 0;
    (iVar1 + 0xe) = 0;
    pass1_1008_3e38((param_1 & 0xffff0000 | (iVar1 + 0x10)));
    return param_1;
}

pub fn pass1_1020_434c(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    param_4: *mut u32,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: i16,
) {
    if (param_7 == 1) {
        pass1_1020_6184(CONCAT22(param_3, param_2), param_6);
        return;
    }
    if (param_7 == 0x2) {
        ui_op_1020_536e(
            param_1,
            CONCAT22(param_3, param_2),
            CONCAT22(param_5, param_4),
            param_6,
            0x2,
        );
        return;
    }
    pass1_1008_68ea(param_2, param_3, param_4, param_5, param_6, param_7);
    return;
}

pub fn win_1020_43f6(
    param_1: *mut Struct57,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_160;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let struct_a_1: *mut StructA;

    struct_a_1 = param_2;
    uVar6 = (param_2 >> 0x10);
    create_window_ex_1008_9760(param_2);
    get_dc_1018_4db0(&struct_a_1[0x1].field25_0x2e, struct_a_1.field4_0x8);
    puVar5 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_7, 0x32),
        param_3,
        param_4,
        param_5,
        param_6,
    );
    paVar4 = (param_1 & 0xffff0000);
    struct_a_1[0x1].field38_0x42 = puVar5;
    (&struct_a_1[0x1].field38_0x42 + 0x2) = (puVar5 >> 0x10);
    if (param_2.is_null() == false) {
        paVar4 = (paVar4 | uVar6);
    }
    ppcVar1 = (struct_a_1[0x1].field38_0x42 + 0x4);
    paVar2 = (**ppcVar1)();
    mem_op_1000_179c(0x30, paVar4);
    uVar3 = paVar4 | paVar2;
    if (uVar3 == 0) {
        struct_a_1[0x1].field22_0x2a = 0;
    } else {
        pass1_1020_62e0(paVar2, paVar4, struct_a_1.field4_0x8);
        struct_a_1[0x1].field22_0x2a = paVar2;
        struct_a_1[0x1].field_0x2c = uVar3;
    }
    ui_op_1020_536e(uVar3, param_2, 0x0, -0x1, 0x3);
    return;
}


pub fn pass1_1020_44b0(param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xf6) != 0) {
        ppcVar1 = (*param_1 + 0x98);
        (**ppcVar1)();
        (iVar2 + 0x112) = 0;
        ppcVar1 = ((iVar2 + 0xf6) + 0x8);
        (**ppcVar1)();
    }
    return;
}
