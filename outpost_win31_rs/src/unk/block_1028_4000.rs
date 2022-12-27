pub fn struct_1028_406c(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0x42ec;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_408e(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0x42ec;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}



pub fn pass1_1028_40b8(mut param_1: u16, param_2: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut uStack54: u32;
    let mut local_2c: [u8; 0x6] = [0; 0x6];
    let mut puStack38: *mut u32;
    let mut uStack34: u32;
    let mut puStack26: *mut u32;
    let mut uStack24: u32;
    let mut local_14: u32;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    pass1_1028_b58e(param_2);
    local_14 = (param_1 + 0xc);
    iStack8 = (param_1 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34, &local_14);
    iStack16 = iStack8 + 1;
    puVar7 = CONCAT22(0x1050, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    uStack6 = param_1;
    uStack4 = extraout_DX;
    uVar6 = pass1_1028_bb24(param_2);
    uVar5 = (uVar6 >> 0x10);
    puVar2 = &local_14;
    pass1_1030_64ce(
        puVar2,
        uVar5,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        uVar6 & 0xffff | uVar5 << 0x10,
        puVar7,
    );
    uStack34 = *puVar2;
    uVar5 = (puVar2 + 2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    uVar3 = uStack54._3_1_;
    uStack24 = uStack34;
    if (uStack54._3_1_ != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack34 & 0xffff | uVar5 << 0x10);
        uStack54 = CONCAT22(uVar5, uVar3);
        uVar4 = pass1_1030_6fa0(CONCAT22(uVar5, uVar3));
        if (uVar4 == 0x64) {
            puStack38 = struct_op_1030_73a8(uStack54, 0x64, uVar5);
            ppcVar1 = (*puStack38 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b514(param_2);
    return;
}







pub fn struct_1028_4354(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0x446a;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_4376(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0x446a;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}




pub fn pass1_1028_43c2(
    param_1: *mut u8,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000fffa: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    pass1_1028_bd38(param_1, CONCAT22(param_3, param_2));
    if (param_4 == 0) {
        puVar2 = mixed_1010_20ba(
            paVar1,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fffa, 0x8),
            in_stack_0000fea2,
            in_stack_0000ffc6,
            in_stack_0000ffcc,
            in_stack_0000ffd0,
        );
        pass1_1010_988c(puVar2, (param_2 + 0xc));
    }
    return;
}





pub fn struct_1028_44d2(param_1: *mut astruct_180) -> *mut u16 {
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field0_0x0 = 0x4836;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_44fe(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    param_2.field0_0x0 = 0x4836;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}
pub fn pass1_1028_4530(param_1: *mut u16) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    *param_1 = 0x4836;
    (iVar4 + 0x2) = 0x1028;
    puVar1 = (iVar4 + 0x20);
    uVar2 = (iVar4 + 0x22);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1028_b418(param_1);
    return;
}



pub fn pass1_1028_45fe(mut param_1: i16, param_2: *mut astruct_15, mut param_3: u32) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_21;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut pstruct15_1: *mut astruct_15;
    let mut iVar5: *mut astruct_314;
    let mut pstruct15_2: *mut astruct_15;
    let mut uVar6: u16;
    let mut paStack44: *mut astruct_99;
    let mut local_28: i32;
    let mut paStack34: *mut astruct_21;
    let mut uStack32: u16;
    let mut paStack30: *mut astruct_99;
    let mut local_1a: [i16; 0x4] = [0; 0x4];
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut puStack10: *mut u32;
    let mut uStack6: u32;
    let mut uVar2: *mut astruct_313;

    pass1_1028_b58e(param_2);
    uStack6 = CONCAT22(param_3, param_1);
    puStack10 = (param_1 + 0x22);
    pstruct15_2 = (param_2 >> 0x10);
    pstruct15_1 = param_2;
    paVar3 = pstruct15_1.field24_0x20;
    uStack32 = pstruct15_1.field25_0x22;
    paVar5 = (param_3 & 0xffff0000 | uStack32);
    paStack30 = CONCAT22(uStack32, paVar3);
    paStack34 = paVar3;
    if ((uStack32 | paVar3) != 0) {
        ppcVar2 = paVar3;
        (**ppcVar2)();
    }
    mem_op_1000_179c(0xc, paVar5);
    uStack32 = paVar5;
    uVar4 = uStack32 | paVar3;
    paStack34 = paVar3;
    if (uVar4 == 0) {
        paVar3 = null_mut();
        uVar4 = 0;
    } else {
        set_struct_1008_574a(CONCAT22(uStack32, paVar3));
    }
    pstruct15_1.field24_0x20 = paVar3;
    pstruct15_1.field25_0x22 = uVar4;
    if (puStack10.is_null() == false) {
        uStack14 = (puStack10 + 0x4);
        for uStack18 in 0..uStack14 {
            pass1_1020_bb16(
                puStack10,
                CONCAT22(0x1050, &local_28),
                CONCAT22(0x1050, local_1a),
                uStack18,
            );
            if ((local_28 != 0) && (local_1a[0] != 0)) {
                paStack30 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                uVar4 = (paStack30 >> 0x10);
                uVar2 = paStack30;
                if ((uVar4 | uVar2) == 0) {
                    paStack44 = null_mut();
                } else {
                    paStack30.field0_0x0 = 0x389a;
                    uVar2.field2_0x2 = 0x1008;
                    uVar2.field3_0x4 = 0;
                    uVar2.field4_0x6 = 0;
                    uVar2.field5_0x8 = 0;
                    uVar2.field6_0xa = 0;
                    uVar2.field7_0xc = 0;
                    paStack30.field0_0x0 = 0x56ce;
                    uVar2.field2_0x2 = 0x1018;
                    paStack44 = paStack30;
                }
                uVar6 = (paStack44 >> 0x10);
                iVar5 = paStack44;
                iVar5.field4_0x4 = local_1a[0];
                iVar5.field9_0xa = local_28;
                iVar5.field10_0xc = local_28;
                uVar1 = &pstruct15_1.field24_0x20;
                ppcVar2 = (*&pstruct15_1.field24_0x20 + 0x8);
                (**ppcVar2)(0x0, uVar1, (uVar1 >> 0x10), iVar5, uVar6);
            }
        }
    }
    return;
}



pub fn pass1_1028_478a(mut param_1: i16, param_2: *mut astruct_15) {
    let mut extraout_DX: u16;
    let mut local_1e: i32;
    let mut local_1a: [i16; 0x4] = [0; 0x4];
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut lStack14: i32;
    let mut puStack10: *mut u32;
    let mut uStack6: u32;

    pass1_1028_b58e(param_2);
    uStack6 = CONCAT22(extraout_DX, param_1);
    puStack10 = (param_1 + 0x22);
    lStack14 = 0;
    if (((param_1 + 0x24) | puStack10) == 0) {
        return;
    }
    uStack16 = (puStack10 + 0x4);
    for uStack18 in 0..uStack16 {
        pass1_1020_bb16(
            puStack10,
            CONCAT22(0x1050, &local_1e),
            CONCAT22(0x1050, local_1a),
            uStack18,
        );
        if (0x0 < local_1a[0]) {
            lStack14 += local_1e;
        }
    }
    return;
}


pub fn struct_1028_489e(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = &PTR_LOOP_1050_4942;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_48c0(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = &PTR_LOOP_1050_4942;
    (param_2 + 0x2) = 0x1028;
    (param_2 + 0xe) = (param_2 + 0xc);
    (param_2 + 0x10) = (param_2 + 0xc);
    return &param_2.field0_0x0;
}


pub fn struct_1028_49aa(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0x4b1c;
    (param_1 + 0x2) = 0x1028;
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x20)), NULL, 0xa);
    return &param_1.field0_0x0;
}

pub fn pass1_1028_49de(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> u32 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0x4b1c;
    (param_2 + 0x2) = 0x1028;
    pass1_1000_4906((param_2 & 0xffff0000 | (param_2 + 0x20)), NULL, 0xa);
    return param_2;
}



pub fn pass1_1028_4a9a(mut param_1: u32, mut param_2: i16) -> u16 {
    return (param_1 + 0x20 + param_2 * 0x2);
}
pub fn pass1_1028_4ab2(mut param_1: u32, mut param_2: u16, mut param_3: i16) {
    (param_1 + param_3 * 0x2 + 0x20) = param_2;
    return;
}


pub fn pass1_1028_4aca(mut param_1: u16, mut param_2: u32) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut pSVar3: *mut StructD;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000fff6: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    if ((param_2 + 0x20) != 0) {
        puVar4 = mixed_1010_20ba(
            paVar2,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fff6, 0x2f),
            in_stack_0000fe9e,
            in_stack_0000ffc2,
            in_stack_0000ffc8,
            in_stack_0000ffcc,
        );
        pSVar3 = (paVar2 & 0xffff0000 | puVar4 >> 0x10);
        uVar1 = SUB42(puVar4, 0x0);
        pass1_1010_ed3e(puVar4);
        pass1_1030_2a2c(pSVar3, CONCAT22(pSVar3, uVar1));
    }
    return;
}


pub fn struct_1028_4b84(param_1: *mut astruct_180) -> *mut u16 {
    struct_1028_b354(param_1);
    // just 0x5070
    param_1.field0_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 1;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}

pub fn pass1_1028_4ba6(
    mut param_1: u16,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> *mut u16 {
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 1;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}














pub fn pass1_1028_4f62(param_1: *mut astruct_15) -> u16 {
    let mut uVar1: u32;

    uVar1 = pass1_1028_b58e(param_1);
    if ((uVar1 + 0x10) == 0) {
        return 0x1;
    }
    return 0x0;
}




pub fn pass1_1028_4faa(param_1: *mut astruct_15, mut param_2: u16) -> u32 {
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut uStack6: u32;

    uVar1 = pass1_1028_4f62(param_1);
    if (uVar1 != 0) {
        return param_1;
    }
    uStack6 = pass1_1028_b58e(param_1);
    local_c = (uStack6 + 0xc);
    uStack8 = 0;
    uVar4 = pass1_1028_bb24(param_1);
    uVar3 = (uVar4 >> 0x10);
    puVar2 = &local_c;
    pass1_1030_627e(
        puVar2,
        uVar3,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        uVar4 & 0xffff | uVar3 << 0x10,
    );
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(uVar3, puVar2));
    if ((uVar3 | puVar2) == 0) {
        return 0x0;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, puVar2), puVar2, uVar3 | puVar2);
    return uVar4;
}
