
pub fn pass1_1028_3246(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut in_stack_0000ffd8: u16;
    let mut local_20: [u8; 0x6] = [0; 0x6];
    let mut puStack26: *mut u32;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut uStack6: u32;

    paVar3 = CONCAT22(in_register_0000000a, param_2);
    pass1_1028_b58e(param_3);
    uStack6 = CONCAT22(paVar3, param_1);
    uStack10 = (param_1 + 0x2e);
    uVar2 = (uStack10 + 0x10);
    uVar5 = 0;
    iVar6 = 0x1;
    uVar4 = 0x1;
    uStack14 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2);
    uVar1 = uVar2;
    uStack16 = SUB42(paVar3, 0x0);
    uStack18 = uVar1;
    pass1_1030_7c50(
        uVar1,
        paVar3,
        (uVar2 & 0xffff | paVar3 << 0x10),
        CONCAT22(uVar5, uVar4),
        iVar6,
    );
    pass1_1030_7c50(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x1, 0x2);
    pass1_1030_7c50(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x1, 0x3);
    pass1_1030_7c50(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x1, 0x5);
    puStack26 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffd8, 0x2),
        in_stack_0000fe80,
        in_stack_0000ffa4,
        in_stack_0000ffaa,
        in_stack_0000ffae,
    );
    paVar3 = (paVar3 & 0xffff0000 | puStack26 >> 0x10);
    uVar1 = puStack26;
    if ((uVar1 + 0x82) == 0) {
        pass1_1030_7c50(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x4, 0x4);
    }
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0xc8, 0x11);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0xc8, 0x12);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0xc8, 0x13);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0xc8, 0x14);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x14, 0x15);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x14, 0x16);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0xc8, 0x17);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0xc8, 0x18);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0xc8, 0x19);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x14, 0x1a);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x14, 0x1b);
    pass1_1030_7ddc(uVar1, paVar3, CONCAT22(uStack16, uStack18), 0x14, 0x1c);
    if ((uStack10 + 0x200) == 0x8000002) {
        pass1_1020_a43e(paVar3, CONCAT22(0x1050, local_20));
        pass1_1020_a89e(
            CONCAT22(0x1050, local_20),
            (uStack6 + 0xc),
            (uStack6 >> 0x10),
        );
    }
    return;
}


pub fn struct_1028_3484(param_1: *mut u16) -> *mut u16 {
    let mut in_EDX: *mut Struct57;

    struct_1028_0068(in_EDX, param_1);
    *param_1 = 0x34f6;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}

pub fn pass1_1028_34a6(
    param_1: *mut u8,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_00cc(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0x34f6;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}


pub fn struct_1028_355e(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0x3608;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_3580(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0x3608;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}


pub fn struct_1028_3670(param_1: *mut u8, param_2: *mut u16) -> *mut u16 {
    let mut in_register_0000000a: u16;

    struct_1028_37a6(CONCAT22(in_register_0000000a, param_1), param_2);
    *param_2 = 0x373e;
    (param_2 + 0x2) = 0x1028;
    return param_2;
}

pub fn pass1_1028_3692(
    param_1: *mut u8,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u32,
) -> *mut u16 {
    let mut in_register_0000000a: u16;

    pass1_1028_3816(
        CONCAT22(in_register_0000000a, param_1),
        CONCAT22(param_3, param_2),
        param_4,
        param_5,
    );
    CONCAT22(param_3, param_2) = 0x373e;
    (param_2 + 0x2) = 0x1028;
    return CONCAT22(param_3, param_2);
}


pub fn struct_1028_37a6(param_1: *mut Struct57, param_2: *mut astruct_180) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: *mut astruct_180;
    let mut uVar3: u16;

    struct_1028_b354(param_2);
    uVar3 = (param_2 >> 0x10);
    iVar3 = param_2;
    uVar1 = 0;
    (iVar3 + 1) = 0;
    iVar3[0x1].field_0x4 = 0;
    iVar3[0x1].field_0x8 = 0;
    param_2.field0_0x0 = 0x3e2c;
    iVar3.field1_0x2 = 0x1028;
    mem_op_1000_179c(0xa, param_1);
    uVar2 = param_1 | uVar1;
    if (uVar2 == 0) {
        iVar3[0x1].field_0x8 = 0;
    } else {
        pass1_1020_ba3e(CONCAT22(param_1, uVar1), 0x5, 0x5);
        iVar3[0x1].field_0x8 = uVar1;
        iVar3[0x1].field_0xa = uVar2;
    }
    return;
}

pub fn pass1_1028_3816(
    param_1: *mut Struct57,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;

    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    uVar1 = 0;
    (param_2 + 0x20) = 0;
    (param_2 + 0x24) = 0;
    (param_2 + 0x28) = 0;
    param_2.field0_0x0 = 0x3e2c;
    (param_2 + 0x2) = 0x1028;
    mem_op_1000_179c(0xa, param_1);
    uVar2 = param_1 | uVar1;
    if (uVar2 == 0) {
        (param_2 + 0x28) = 0;
    } else {
        pass1_1020_ba3e(CONCAT22(param_1, uVar1), 0x5, 0x5);
        (param_2 + 0x28) = uVar1;
        (param_2 + 0x2a) = uVar2;
    }
    return;
}

pub fn pass1_1028_388e(param_1: *mut u16) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    *param_1 = 0x3e2c;
    (iVar3 + 0x2) = 0x1028;
    pcVar2 = *(iVar3 + 0x28);
    uVar1 = (iVar3 + 0x2a);
    if ((uVar1 | pcVar2) != 0) {
        fn_ptr_1020_ba7e((pcVar2 & 0xffff | uVar1 << 0x10));
        fn_ptr_1000_17ce(pcVar2);
    }
    pass1_1028_b418(param_1);
    return;
}




pub fn pass1_1028_3c32(param_1: u32) -> u32 {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut local_6: u16;
    let mut iStack4: i16;

    ppcVar1 = (*param_1 + 0x40);
    iVar2 = (**ppcVar1)();
    if (iVar2 != 0) {
        return 0x0;
    }
    return CONCAT22(-(0x3e8 < local_6) - iStack4, 0x3e8 - local_6);
}

pub fn struct_1028_3e94(param_1: *mut astruct_180) -> *mut u16 {
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field0_0x0 = 0x4004;
    (param_1 + 0x2) = 0x1028;
    pass1_1028_3fa2((param_1 & 0xffff | uVar1 << 0x10));
    return &param_1.field0_0x0;
}

pub fn pass1_1028_3ec8(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> u32 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    param_2.field0_0x0 = 0x4004;
    (param_2 + 0x2) = 0x1028;
    pass1_1028_3fa2((param_2 & 0xffff | param_2 << 0x10));
    return param_2;
}

pub fn pass1_1028_3fa2(param_1: *mut astruct_180) {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xc) != 0x22) {
        if ((iVar2 + 0xc) == 0x23) {
            uVar1 = 0xa;
        } else {
            uVar1 = 0x5;
        }
        uVar1 >>= 0x1;
        pass1_1008_612e(uVar1, 0x0, uVar1);
        (iVar2 + 0x20) = uVar1;
        (iVar2 + 0x22) = uVar1 >> 0xf;
    }
    return;
}
