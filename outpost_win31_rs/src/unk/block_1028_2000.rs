
pub fn pass1_1028_2042(param_1: *mut astruct_15) {
    let mut paVar1: *mut astruct_21;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut iVar5: *mut astruct_15;
    let mut uVar5: *mut astruct_15;
    let mut uVar6: u32;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.field0_0x0 = 0x2572;
    iVar5.field1_0x2 = 0x1028;
    uVar4 = &iVar5.field24_0x20;
    (uVar4 + 0xa) = 0x1;
    paVar1 = iVar5.field24_0x20;
    uVar2 = iVar5.field25_0x22;
    if ((uVar2 | paVar1) != 0) {
        ppcVar3 = paVar1;
        (**ppcVar3)();
    }
    if ((_PTR_LOOP_1050_65e2 != 0) && (_PTR_LOOP_1050_5a64 != 0)) {
        uVar6 = pass1_1028_b58e(param_1);
        pass1_1038_79f2(_PTR_LOOP_1050_5a64, uVar6, 0x1050);
    }
    pass1_1028_b418(&param_1.field0_0x0);
    return;
}

pub fn pass1_1028_20b0() -> u16 {
    return 0x0;
}





pub fn pass1_1028_21ba(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut u16,
    param_6: i32,
) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u32;

    pass1_1030_627e(param_1, param_2, _PTR_LOOP_1050_5740, param_5, param_6);
    uVar1 = param_2 | param_1;
    if (uVar1 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(param_2, param_1));
        if ((uVar1 | param_1) != 0) {
            uVar2 = struct_op_1030_73a8(CONCAT22(uVar1, param_1), uVar1 | param_1, uVar1);
            if ((uVar2 != 0) && ((uVar2 + 0xc) == 0x40)) {
                return 0x1;
            }
        }
    }
    return 0x0;
}



pub fn pass1_1028_2220(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: i16,
    param_6: *mut u16,
    param_7: i32,
) -> u16 {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u32;

    pass1_1030_627e(param_1, param_2, _PTR_LOOP_1050_5740, param_6, param_7);
    uVar2 = param_2 | param_1;
    if (uVar2 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(param_2, param_1));
        if ((uVar2 | param_1) != 0) {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_1), uVar2 | param_1, uVar2);
            if ((uVar3 != 0) && ((iVar1 = (uVar3 + 0xc), iVar1 == 0x40 || (iVar1 == param_5)))) {
                return 0x1;
            }
        }
    }
    return 0x0;
}







pub fn struct_1028_25da(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    // just 0x264c
    param_1.field0_0x0 = s_fem16_wav_1050_2644 + 0x8;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_25fc(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = s_fem16_wav_1050_2644 + 0x8;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}


pub fn struct_1028_26b4(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0x2788;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_26d6(
    param_1: *mut StructD,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0x2788;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}


pub fn struct_1028_27f0(param_1: *mut astruct_180) -> *mut astruct_180 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0x2a92;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}

pub fn pass1_1028_2812(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0x2a92;
    (param_2 + 0x2) = 0x1028;
    (param_2 + 0xe) = (param_2 + 0xc);
    return &param_2.field0_0x0;
}






pub fn pass1_1028_297c(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
    param_5: i32,
) -> u16 {
    let mut cVar1: u8;
    let mut uVar2: u16;
    let mut uVar3: u32;

    pass1_1028_c7b6(param_2, param_3, (param_3 >> 0x10), param_4, param_5);
    if (param_1 == 0) {
        pass1_1030_627e(0x0, param_2, _PTR_LOOP_1050_5740, param_4, param_5);
        uVar2 = param_2 | param_1;
        if (uVar2 != 0) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(param_2, param_1));
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_1), param_1, uVar2);
            uVar2 = (uVar3 + 0xc);
            if (0x4a < uVar2) {
                match uVar2 {
                    0x4c | 0x4d | 0x4e | 0x60 | 0x61 | 0x62 | 0x63 | 0x6e | 0x73 | 0x74 | 0x75
                    | 0x76 | 0x77 | 0x78 | 0x79 => {}
                    // TODO: goto switchD_1028_2a0b_caseD_4c;
                    _ => {} // TODO: goto switchD_1028_2a0b_caseD_4f;
                };
            }
            if ((uVar2 < 0x48) && (uVar2 != 0x41)) {
                if (uVar2 < 0x42) {
                    cVar1 = uVar2;
                    if (cVar1 < '5') {
                        if ('2' < cVar1) {
                            return 0x0;
                        }
                        cVar1 += -0x10;
                    } else {
                        cVar1 += -0x3e;
                    }
                    if (cVar1 == '\0') {
                        return 0x0;
                    }
                }
                // switchD_1028_2a0b_caseD_4f:
                return 0x1;
            }
        }
    }
    // switchD_1028_2a0b_caseD_4c:
    return 0x0;
}


pub fn struct_1028_2afa(param_1: *mut u16) -> *mut u16 {
    struct_1030_dc96(param_1);
    *param_1 = 0x2b74;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}

pub fn pass1_1028_2b1c(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u32,
) -> *mut u16 {
    pass1_1030_dcc2(param_1, CONCAT22(param_3, param_2), param_4, param_5);
    CONCAT22(param_3, param_2) = 0x2b74;
    (param_2 + 0x2) = 0x1028;
    return CONCAT22(param_3, param_2);
}


pub fn struct_1028_2bdc(param_1: *mut u16) -> *mut u16 {
    struct_1028_0954(param_1);
    *param_1 = 0x341c;
    (param_1 + 0x2) = 0x1028;
    return param_1;
}

pub fn pass1_1028_2bfe(
    mut param_1: u16,
    param_2: *mut astruct_179,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: u32,
) -> *mut u16 {
    pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_4, param_5);
    CONCAT22(param_3, param_2) = 0x341c;
    param_2.field2_0x2 = 0x1028;
    return CONCAT22(param_3, param_2);
}







pub fn pass1_1028_2f18(param_1: u8, mut param_2: i16, param_3: *mut astruct_15) {
    let mut iVar1: i16;
    let mut puVar2: *mut u32;
    let mut extraout_DX: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;
    let mut local_944: [u8; 0x124] = [0; 0x124];
    let mut local_820: [u8; 0x124] = [0; 0x124];
    let mut local_6fc: [u8; 0x124] = [0; 0x124];
    let mut local_5d8: [u8; 0x124] = [0; 0x124];
    let mut local_4b4: [u8; 0x124] = [0; 0x124];
    let mut local_390: u32;
    let mut local_38a: [u8; 0x124] = [0; 0x124];
    let mut local_266: [u8; 0x124] = [0; 0x124];
    let mut local_142: [u8; 0x124] = [0; 0x124];
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut uStack20: u16;
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut uStack6: u32;

    uStack6 = pass1_1028_bb24(param_3);
    iVar1 = uStack6;
    pass1_1028_b58e(param_3);
    uStack10 = CONCAT22(extraout_DX, iVar1);
    uStack14 = (iVar1 + 0x2e);
    uStack18 = (uStack14 + 0x4);
    local_18 = (iVar1 + 0xc);
    uStack20 = (iVar1 + 0x10);
    pass1_1008_3eb4(
        CONCAT22(0x1050, &local_18),
        CONCAT22(0x1050, &local_1e),
        CONCAT22(0x1050, &local_1e + 0x2),
        CONCAT22(0x1050, &local_1a),
    );
    pass1_1008_3e76(
        CONCAT22(0x1050, &local_18),
        local_1e,
        local_1e - 0x1,
        local_1a - 1,
    );
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_142),
        0x0,
        0x0,
        0xd,
        &local_18,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_142));
    pass1_1008_3e76(
        CONCAT22(0x1050, &local_18),
        local_1e,
        local_1e + 0x1,
        local_1a + 1,
    );
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_266),
        0x0,
        0x0,
        0xc,
        &local_18,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_266));
    pass1_1008_3e76(
        CONCAT22(0x1050, &local_18),
        local_1e,
        local_1e + 0x1,
        local_1a - 1,
    );
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_38a),
        0x0,
        0x0,
        0xe,
        &local_18,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_38a));
    puVar5 = pass1_1008_3e54(
        CONCAT22(0x1050, &local_390),
        local_1e,
        local_1e - 0x1,
        local_1a + 1,
    );
    uVar3 = (puVar5 >> 0x10);
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_4b4),
        0x0,
        0x0,
        0xb,
        &local_390,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_4b4));
    pass1_1008_3e76(
        CONCAT22(0x1050, &local_18),
        local_1e,
        local_1e - 0x1,
        local_1a,
    );
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_5d8),
        0x0,
        0x0,
        0x7a,
        &local_18,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_5d8));
    pass1_1008_3e76(
        CONCAT22(0x1050, &local_18),
        local_1e,
        (local_1e >> 0x10),
        local_1a + 1,
    );
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_6fc),
        0x0,
        0x0,
        0x7a,
        &local_18,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_6fc));
    pass1_1008_3e76(
        CONCAT22(0x1050, &local_18),
        local_1e,
        local_1e + 0x1,
        local_1a,
    );
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_820),
        0x0,
        0x0,
        0x7a,
        &local_18,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_820));
    pass1_1008_3e76(
        CONCAT22(0x1050, &local_18),
        local_1e,
        (local_1e >> 0x10),
        local_1a - 1,
    );
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_944),
        0x0,
        0x0,
        0x7a,
        &local_18,
        0x1050,
        uStack18,
        uStack6,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_944));
    puVar2 = &local_390;
    pass1_1030_627e(
        puVar2,
        uVar3,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        uStack6,
    );
    uVar4 = (uStack14 >> 0x10);
    (uStack14 + 0x10) = puVar2;
    (uStack14 + 0x12) = uVar3;
    return;
}
