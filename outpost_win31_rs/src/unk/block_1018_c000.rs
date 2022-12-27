
pub fn pass1_1018_c402(
    mut param_1: u16,
    param_2: *mut astruct_20,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u32,
    mut param_7: u32,
    mut param_8: u32,
    mut param_9: u32,
) {
    let mut iVar1: i16;
    let mut puVar2: *mut u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: *mut astruct_20;
    let mut uVar4: *mut astruct_20;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe8e: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffe6: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    struct_1020_0762(
        param_2,
        CONCAT22(param_6, param_5),
        CONCAT22(param_7, (param_6 >> 0x10)),
        (param_7 >> 0x10),
        param_8,
        param_9,
    );
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    iVar4[0x1].field8_0x14 = 0;
    iVar4[0x1].field9_0x16 = null_mut();
    iVar4[0x1].field10_0x18 = 0;
    iVar4[0x1].field11_0x1a = 0;
    iVar4[0x1].field12_0x1c = 0x2;
    iVar4[0x1].field15_0x26 = 0;
    iVar4[0x1].field16_0x2a = 0;
    iVar4[0x1].field17_0x2c = 0x1e0190;
    iVar4[0x1].field18_0x30 = 0;
    param_2.offset_0x0 = 0xc8bc;
    iVar4.base_0x2 = 0x1018;
    puVar2 = pass1_1000_4906(
        (param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field13_0x1e)),
        NULL,
        0x8,
    );
    if ((param_4 == 0) || (param_3 != 0)) {
        //    if ((param_3 & param_4) == 0) goto LAB_1018_c4bb;
        puVar2 = pass1_1008_5fd8(paVar3);
    } else {
        load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), param_4);
    }
    iVar4[0x1].field15_0x26 = puVar2;
    (&iVar4[0x1].field15_0x26 + 0x2) = paVar3; //
    // LAB_1018_c4bb:
    puVar4 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe6, 0x48),
        in_stack_0000fe8e,
        in_stack_0000ffb2,
        in_stack_0000ffb8,
        in_stack_0000ffbc,
    );
    iVar1 = puVar4;
    iVar4[0x1].field8_0x14 = (iVar1 + 0xa);
    iVar4[0x1].field9_0x16 = (iVar1 + 0xc);
    pass1_1008_3e94(
        (puVar4 & 0xffff0000 | (iVar1 + 0xe)),
        (param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field11_0x1a)),
        (param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field10_0x18)),
    );
    return;
}






pub fn pass1_1018_c958(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf1;
    uVar4 = 0x9a;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x8d);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x732,
        0x26,
        CONCAT22(puVar2, 0x1f40),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xdc5a;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_c9a6(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf2;
    uVar4 = 0xa0;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x8e);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x733,
        0x27,
        CONCAT22(puVar2, 0x1b58),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd6de;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_c9f4(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut uVar5: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf3;
    uVar5 = 0xa6;
    puVar4 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x8f);
    uVar2 = (puVar4 >> 0x10);
    pass1_1018_c402(
        uVar2,
        param_1,
        0x0,
        0x734,
        0x28,
        CONCAT22(puVar4, 0x32c8),
        CONCAT22(uVar3, uVar2),
        CONCAT22(param_2, uVar5),
        param_3,
    );
    uVar3 = (param_1 >> 0x10);
    param_1.offset_0x0 = 0xda86;
    (param_1 + 0x2) = 0x1018;
    piVar1 = (param_1 + 0x10e);
    *piVar1 = *piVar1 - 0x19;
    return param_1;
}

pub fn pass1_1018_ca48(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf4;
    uVar4 = 0xa1;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x90);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x735,
        0x29,
        CONCAT22(puVar2, 0x36b0),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd50a;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_ca96(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut uVar5: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf5;
    uVar5 = 0xbf;
    puVar4 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x92);
    uVar2 = (puVar4 >> 0x10);
    pass1_1018_c402(
        uVar2,
        param_1,
        0x737,
        0x736,
        0x2a,
        CONCAT22(puVar4, 0x6590),
        CONCAT22(uVar3, uVar2),
        CONCAT22(param_2, uVar5),
        param_3,
    );
    uVar3 = (param_1 >> 0x10);
    param_1.offset_0x0 = 0xd8b2;
    (param_1 + 0x2) = 0x1018;
    piVar1 = (param_1 + 0x10e);
    *piVar1 = *piVar1 + 0x64;
    return param_1;
}

pub fn pass1_1018_caea(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf6;
    uVar4 = 0x93;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x93);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x738,
        0x2b,
        CONCAT22(puVar2, 0x2328),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xdbbe;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cb38(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf7;
    uVar4 = 0x94;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x94);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x739,
        0x2c,
        CONCAT22(puVar2, 0x32c8),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd642;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cb86(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut uVar5: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf8;
    uVar5 = 0xc2;
    puVar4 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x96);
    uVar2 = (puVar4 >> 0x10);
    pass1_1018_c402(
        uVar2,
        param_1,
        0x0,
        0x73a,
        0x2d,
        CONCAT22(puVar4, 0x2328),
        CONCAT22(uVar3, uVar2),
        CONCAT22(param_2, uVar5),
        param_3,
    );
    uVar3 = (param_1 >> 0x10);
    param_1.offset_0x0 = 0xd9ea;
    (param_1 + 0x2) = 0x1018;
    piVar1 = (param_1 + 0x10e);
    *piVar1 = *piVar1 + 0x64;
    return param_1;
}

pub fn pass1_1018_cbda(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xf9;
    uVar4 = 0xc5;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x97);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x73b,
        0x2e,
        CONCAT22(puVar2, 0x2af8),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd46e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cc28(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];
    let mut uVar3: u16;
    let mut uVar4: u16;

    uVar3 = 0xfa;
    uVar4 = 0xa3;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x98);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x73c,
        0x2f,
        CONCAT22(puVar2, 0x2710),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd816;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cc76(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xfb;
    uVar4 = 0xa8;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x99);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x73e,
        0x73d,
        0x30,
        CONCAT22(puVar2, 0x61a8),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xdb22;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_ccc4(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xfc;
    uVar4 = 0xa9;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x9b);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x740,
        0x73f,
        0x31,
        CONCAT22(puVar2, 0x59d8),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd5a6;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cd12(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xfd;
    uVar4 = 0x7c;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x9c);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x741,
        0x32,
        CONCAT22(puVar2, 0x2710),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd94e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cd60(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xfe;
    uVar4 = 0xc9;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x0);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x0,
        0x33,
        CONCAT22(puVar2, 0x2710),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd3d2;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}






pub fn pass1_1018_cf74(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
) -> *mut astruct_20 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];

    uVar3 = 0xfe;
    uVar4 = 0xcf;
    puVar2 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x80);
    uVar1 = (puVar2 >> 0x10);
    pass1_1018_c402(
        uVar1,
        param_1,
        0x0,
        0x0,
        0x34,
        CONCAT22(puVar2, 0x2710),
        CONCAT22(uVar3, uVar1),
        CONCAT22(param_2, uVar4),
        param_3,
    );
    param_1.offset_0x0 = 0xd77a;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}
