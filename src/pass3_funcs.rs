pub fn pass1_1028_525a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_52e8(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x535e;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_530a(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = "";
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5338(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_53c6(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x54bc;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_53e8(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = 0x54bc;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5412(param_1: *mut *mut u8) {
    let ppcVar1: fn();
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 6) {
        return;
    }
    uVar3 = pass1_1028_b4f2(param_1);
    if ((uVar3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            iVar4 = 6;
            // goto code_r0x1028548e;
        }
        ppcVar1 = (param_1 + 100);
        iVar4 = (**ppcVar1)();
        if (iVar4 == 0) {
            return;
        }
        iVar4 = pass1_1028_c0f0(param_1, uVar2, 1, 0);
        if (iVar4 == 0) {
            iVar4 = 6;
            // goto code_r0x1028548e;
        }
        pass1_1028_c952(param_1);
        pass1_1028_c00a(param_1, uVar2, 1, 0);
    }
    iVar4 = 5;
    // code_r0x1028548e:
    pass1_1028_bdac(param_1, iVar4);
    return;
}

pub fn pass1_1028_5496(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5524(param_1: *mut astruct_727) -> *mut astruct_763 {
    pass1_1028_0068(param_1);
    param_1 = 0x55c8;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5546(
    param_1: *mut astruct_728,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut astruct_728 {
    pass1_1028_00cc(param_1, CONCAT22(param_3, param_2), param_3_00);
    *CONCAT22(param_2, param_1) = 0x55c8;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5570(param_1: u32) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    pass1_1028_0550(param_1);
    if ((param_1 + 0x12) == 5) {
        uVar4 = 0;
        uVar6 = 4;
        uVar3 = 1;
        uVar2 = extraout_DX;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar5 << 0x10));
        pass1_1030_7c50(
            CONCAT22(uVar2, CONCAT11(extraout_AH, uVar1)),
            CONCAT22(uVar4, uVar3),
            uVar6,
        );
    }
    return;
}

pub fn pass1_1028_55a2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_0138(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5630(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x56ac;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5652(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = 0x56ac;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5686(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5718(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_57a6(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x581c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_57c8(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = 0x581c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_57f6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5884(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x58fe;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_58a6(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = 0x58fe;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_58d8(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5966(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_mineToSmelter__no_mines_1050_59df + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5988(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = (s_mineToSmelter__no_mines_1050_59df + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_59ba(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5a48(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = s_thisLo_1050_5bec;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5a6a(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = s_thisLo_1050_5bec;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5a94(param_1: *mut astruct_44, param_2: *mut u32) {
    let ppcVar1: fn();
    let uVar2: u8;
    let mut in_AX: i32;
    let puVar3: *mut u8;
    let mut uVar4: u32;
    let extraout_var: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar5: i32;
    let mut unaff_SS: u16;
    let paVar6: *mut astruct_967;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 2];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    unsafe {
        ppcVar1 = (*param_2 + 0x10);
        (**ppcVar1)();
    }
    _local_6 = CONCAT22(extraout_DX, in_AX);
    if ((extraout_DX | in_AX) == 0) {
        return;
    }
    local_8 = 1;
    pass1_1030_bcae(local_a, unaff_SS);
    local_e = 0;
    while (true) {
        if (_local_6 <= local_e) {
            return;
        }
        uVar4 = _local_6;
        pass1_1030_1d58(param_2);
        paVar6 = (uVar4 & 0xffff | extraout_DX_00 << 0x10);
        uVar5 = extraout_DX_00;
        uVar2 = pass1_1028_b58e(param_1);
        puVar3 = local_a;
        pass1_1030_bd74(
            puVar3,
            unaff_SS,
            (CONCAT31(extraout_var, uVar2) & 0xffff | uVar5 << 0x10),
            paVar6,
        );
        if (puVar3 < 5) {
            break;
        }
        local_e = local_e + 1;
    }
    pass1_1030_73a8((uVar4 & 0xffff | extraout_DX_00 << 0x10));
    return;
}

pub fn pass1_1028_5b42(param_1: *mut u32) {
    let ppcVar1: fn();
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 6) {
        return;
    }
    uVar3 = pass1_1028_b4f2(param_1);
    if ((uVar3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            iVar4 = 6;
            // goto code_r0x10285bbe;
        }
        unsafe { ppcVar1 = (*param_1 + 100) };
        iVar4 = (**ppcVar1)();
        if (iVar4 == 0) {
            return;
        }
        iVar4 = pass1_1028_c0f0(param_1, uVar2, 2, 0);
        if (iVar4 == 0) {
            iVar4 = 6;
            // goto code_r0x10285bbe;
        }
        pass1_1028_c952(param_1);
        pass1_1028_c00a(param_1, uVar2, 2, 0);
    }
    iVar4 = 5;
    // code_r0x10285bbe:
    pass1_1028_bdac(param_1, iVar4);
    return;
}

pub fn pass1_1028_5bc6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5c54(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_static_1050_5d8b + 3);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5c76(param_1: u16, param_2: u32, param_3: u32) {
    let mut uVar1: u16;

    uVar1 = param_2;
    pass1_1028_b39e(CONCAT22(uVar1, param_1), (param_2 >> 0x10), param_3);
    CONCAT22(uVar1, param_1) = (s_static_1050_5d8b + 3);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5ca0(param_1: u32) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut uVar2: u32;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10;
    local_a = (CONCAT31(extraout_var, uVar1) + 0x2e);
    uVar2 = pass1_1028_bb24(param_1);
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_12e),
        0,
        0,
        0x65,
        (_local_6 + 0xc),
        (_local_6 >> 0x10),
        (local_a + 4),
        uVar2,
    );
    uVar1 = &local_12e;
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_12e));
    return uVar1;
}

pub fn pass1_1028_5d0e(param_1: *mut astruct_44) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10;
    local_a = (CONCAT31(extraout_var, uVar1) + 0x2e);
    local_e = (local_a + 4);
    pass1_1028_68de(CONCAT22(unaff_SS, &local_11c), 1, local_e);
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_11c));
    return;
}

pub fn pass1_1028_5d68(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5df6(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_thisHi_1050_5e6f + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5e18(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = (s_thisHi_1050_5e6f + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5e4a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_5ed8(param_1: *mut astruct_763) -> *mut astruct_763 {
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0x6054;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_5f00(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    (param_1 + 0x20) = 0;
    CONCAT22(param_2, param_1) = 0x6054;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5f30(param_1: *mut astruct_44) {
    let piVar1: *mut i32;
    let uVar2: u8;
    let mut uvar3: u16;
    let extraout_var: u32;
    let puVar4: *mut u8;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut extraout_DX_00: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut iVar8: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar6 = param_1;
    uVar7 = (param_1 >> 0x10);
    pass1_1028_be9e(iVar6, uVar7);
    if ((iVar6 + 0x12) == 5) {
        (iVar6 + 0x20) = 100;
        uVar5 = extraout_DX;
        uVar2 = pass1_1028_b58e(param_1);
        puVar4 = *(CONCAT31(extraout_var, uVar2) + 0x2e);
        iVar8 = 0x61;
        pass1_1038_3fb0(puVar4);
        uVar3 = pass1_1030_25b2(puVar4 & 0xffff | extraout_DX_00 << 0x10, iVar8);
        if (uVar3 != 0) {
            piVar1 = (iVar6 + 0x20);
            unsafe { *piVar1 = *piVar1 + 100 };
        }
    }
    return;
}

pub fn pass1_1028_3e06(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_388e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_3e94(param_1: *mut astruct_763) -> *mut astruct_763 {
    let local_struct_1: *mut astruct_763;
    let mut uVar1: i32;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    local_struct_1.field_0x20 = 0;
    param_1.field_0x0 = 0x4004;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1028;
    pass1_1028_3fa2((param_1 & 0xffff | uVar1 << 0x10));
    return param_1;
}

pub fn pass1_1028_3ec8(
    param_1: *mut astruct_763,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut astruct_763 {
    pass1_1028_b39e(param_1, param_2, CONCAT22(param_3_00, param_3));
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0x4004;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    pass1_1028_3fa2((param_1 & 0xffff | param_1._2_2_ << 0x10));
    return param_1;
}

pub fn pass1_1028_3f04(param_1: *mut astruct_44) {
    let mut u_var1: u32;
    let uVar2: u8;
    let mut uVar3: i32;
    let extraout_var: u32;
    let mut uVar5: u16;
    let in_EDX: u32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut uVar4: u32;

    uVar8 = 0x1f;
    uVar2 = pass1_1028_b58e(param_1);
    uVar4 = CONCAT31(extraout_var, uVar2);
    uVar7 = uVar4;
    uVar5 = in_EDX;
    uVar1 = uVar4 & 0xffff | in_EDX << 0x10;
    pass1_1030_7c28(uVar4 & 0xffff | in_EDX << 0x10, uVar8);
    _local_a = CONCAT22(uVar5, uVar7);
    pass1_1030_7d1c(uVar1, 0, 0x1f0000);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xc) != 0x22) {
        if ((iVar6 + 0xc) == 0x23) {
            uVar3 = 10;
        } else {
            uVar3 = 5;
        }
        _local_e = uVar3;
        uVar4 = _local_a + (iVar6 + 0x20);
        (iVar6 + 0x20) = uVar4 % uVar3;
        _local_a = uVar4 + uVar4 / _local_e;
    }
    pass1_1030_7ddc(uVar1, _local_a, 0x21);
    return;
}

pub fn pass1_1028_3fa2(in_struct_1: *mut astruct_763) {
    let mut uVar1: i32;
    let mut iVar2: i32;
    let local_BX_4: *mut astruct_763;
    let mut uvar3: u16;

    uVar3 = (in_struct_1 >> 0x10);
    local_BX_4 = in_struct_1;
    if (&local_BX_4.field_0xc != 0x22) {
        if (&local_BX_4.field_0xc == 0x23) {
            uVar1 = 10;
        } else {
            uVar1 = 5;
        }
        iVar2 = pass1_fn_1008_612e(0, uVar1 >> 1);
        &local_BX_4.field_0x20 = iVar2;
        (&local_BX_4.field_0x20 + 2) = iVar2 >> 0xf;
    }
    return;
}

pub fn pass1_1028_3fde(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_406c(in_struct_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(in_struct_1);
    in_struct_1.field_0x0 = 0x42ec;
    (in_struct_1 + 2) = &PTR_LOOP_1050_1028;
    return in_struct_1;
}

pub fn pass1_1028_408e(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0x42ec;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_40b8(param_1: *mut astruct_44, param_2: u32) {
    let ppcVar1: fn();
    let uVar2: u8;
    let puVar3: *mut u32;
    let paVar4: *mut astruct_493;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_2c: [u8; 6];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar2);
    local_14 = (local_6 + 0xc);
    local_8 = (local_6 + 0x10);
    local_1a = &local_c;
    local_22 = CONCAT22(local_22._2_2_, &local_14);
    local_10 = local_8 + 1;
    uVar7 = CONCAT22(unaff_SS, local_2c);
    local_e = local_10;
    local_c = local_14;
    uVar6 = pass1_1028_bb24(param_1);
    puVar3 = &local_14;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        puVar3,
        unaff_SS,
        uVar6,
        (uVar6 >> 0x10),
        uVar7,
    );
    unsafe { local_22 = *puVar3 };
    uVar5 = (puVar3 + 2);
    local_36._3_1_ = (local_22 >> 0x18);
    local_18 = local_22;
    if (local_36._3_1_ != '\0') {
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_22, uVar5);
        local_36 = CONCAT22(uVar5, paVar4);
        uVar2 = pass1_1030_6fa0(CONCAT22(uVar5, paVar4));
        if (CONCAT31(extraout_var_00, uVar2) == 100) {
            _local_26 = pass1_1030_73a8(local_36);
            ppcVar1 = (*_local_26 + 0x58);
            (**ppcVar1)(0x1030, _local_26, (_local_26 >> 0x10), param_2);
        }
    }
    pass1_1028_b514(param_1);
    return;
}

pub fn pass1_1028_4194(param_1: u32) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    uVar2 = pass1_1028_b4f2(param_1);
    uVar1 = (uVar2 >> 0x10);
    if (((uVar2 + 0x200) != 0x8000002) && ((param_1 + 0x12) == 5)) {
        ppVar3 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffea, 0x2b));
        pass1_1010_043a(ppVar3, (uVar2 + 4), 0xe);
    }
    return;
}

pub fn pass1_1028_42c6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_4354(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x446a;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_4376(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0x446a;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_43a0(param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (((param_1 + 0x12) != 6) && ((param_1 + 0x12) != 5)) {
        return 0;
    }
    return 1;
}

pub fn pass1_1028_43c2(param_1: u16, param_2: u16, param_3: u16) {
    let ppVar1: *mut pass1_struct_1;
    let mut in_stack_0000fffa: u16;

    pass1_1028_bd38(CONCAT22(param_2, param_1));
    if (param_3 == 0) {
        ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fffa, 8));
        pass1_1010_988c(ppVar1, (param_1 + 0xc));
    }
    return;
}

pub fn pass1_1028_43f6(param_1: *mut astruct_44, param_2: i32) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut uVar4: u32;
    let mut uVar5: u16;

    uVar5 = 0x83;
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x830009);
    uVar4 = pass1_1010_65d0(ppVar3, uVar5);
    uVar2 = (uVar4 >> 0x10);
    if (0 < uVar4) {
        uVar1 = pass1_1028_b58e(param_1);
        if (param_2 == 0) {
            uVar5 = 0;
        } else {
            uVar5 = 1000;
        }
        pass1_1030_7d1c(
            CONCAT22(uVar2, CONCAT11(extraout_AH, uVar1)),
            uVar5,
            0x230000,
        );
    }
    return;
}

pub fn pass1_1028_4444(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_44d2(param_1: *mut astruct_763) -> *mut astruct_763 {
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0x4836;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_44fe(param_1: i32, param_2: *mut u8, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    (param_1 + 0x20) = 0;
    *CONCAT22(param_2, param_1) = 0x4836;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_4530(param_1: *mut astruct_44) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_struct_1: *mut astruct_44;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x4836;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1028;
    puVar1 = local_struct_1.field_0x20;
    uVar2 = &local_struct_1.field_0x22;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_456e(param_1: *mut astruct_781, param_2: *mut u8) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_BX_18: *mut astruct_781;
    let mut uvar3: u16;
    let fn_ptr_1: fn();

    pass1_1028_b46e(param_1, param_2);
    uVar3 = (param_1 >> 0x10);
    local_BX_18 = param_1;
    puVar1 = local_BX_18.field_0x20;
    uVar2 = &local_BX_18.field_0x22;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
            (**fn_ptr_1)()
        };
    }
    &local_BX_18.field_0x20 = 0;
    return;
}

pub fn pass1_1028_45b0(param_1: u32) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    pass1_1028_be9e(param_1, uVar5);
    if ((param_1 + 0x12) == 5) {
        uVar4 = 0;
        uVar6 = 4;
        uVar3 = 2;
        uVar2 = extraout_DX;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar5 << 0x10));
        pass1_1030_7c50(
            CONCAT22(uVar2, CONCAT11(extraout_AH, uVar1)),
            CONCAT22(uVar4, uVar3),
            uVar6,
        );
    }
    return;
}

pub fn pass1_1028_45e2(param_1: u32) {
    let mut u_var1: u32;

    uVar1 = pass1_1028_478a(param_1);
    return CONCAT22(-(1000 < uVar1) - (uVar1 >> 0x10), 1000 - uVar1);
}

pub fn pass1_1028_45fe(param_1: *mut astruct_44) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let uVar3: u8;
    let puVar4: *mut u32;
    let paVar5: *mut astruct_199;
    let local_AX_178: *mut astruct_779;
    let extraout_var: u32;
    let mut in_DX: i32;
    let extraout_DX: *mut astruct_199;
    let mut uVar7: i32;
    let mut extraout_DX_00: i32;
    let mut iVar8: i32;
    let local_BX_265: *mut astruct_780;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut local_38: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let puVar6: *mut u8;

    uVar3 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, uVar3) & 0xffff | in_DX << 0x10;
    local_a = (CONCAT31(extraout_var, uVar3) + 0x22);
    uVar9 = (param_1 >> 0x10);
    iVar8 = param_1;
    puVar4 = (iVar8 + 0x20);
    paVar5 = (iVar8 + 0x22);
    _local_1e = CONCAT22(paVar5, puVar4);
    local_22 = puVar4;
    local_20 = paVar5;
    if ((paVar5 | puVar4) != 0) {
        unsafe {
            ppcVar2 = *puVar4;
            ppcVar2();
        }
        paVar5 = extraout_DX;
    }
    process_struct_1000_179c(0xc, paVar5);
    uVar7 = paVar5 | puVar4;
    local_22 = puVar4;
    local_20 = paVar5;
    if (uVar7 == 0) {
        paVar5 = 0x0;
        uVar7 = 0;
    } else {
        paVar5 = process_struct_1008_574a(CONCAT22(paVar5, puVar4));
    }
    (iVar8 + 0x20) = paVar5;
    (iVar8 + 0x22) = uVar7;
    if (local_a != 0) {
        _local_e = *(local_a + 4);
        local_12 = 0;
        while (local_12 < _local_e) {
            pass1_1020_bb16(
                local_a,
                CONCAT22(unaff_SS, &local_28),
                CONCAT22(unaff_SS, &local_1a),
                local_12,
            );
            if ((local_28 != 0) && (local_1a != 0)) {
                puVar6 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                uVar7 = puVar6;
                _local_1e = (puVar6 & 0xffff | extraout_DX_00 << 0x10);
                if ((extraout_DX_00 | uVar7) == 0) {
                    local_2c = 0;
                } else {
                    *_local_1e = s_1_1050_389a;
                    (uVar7 + 2) = &PTR_LOOP_1050_1008;
                    (uVar7 + 4) = 0;
                    (uVar7 + 6) = 0;
                    (uVar7 + 8) = 0;
                    (uVar7 + 10) = 0;
                    (uVar7 + 0xc) = 0;
                    *_local_1e = 0x56ce;
                    (uVar7 + 2) = 0x1018;
                    local_2c = _local_1e;
                }
                uVar10 = (local_2c >> 0x10);
                local_BX_265 = local_2c;
                local_BX_265.field_0x4 = local_1a;
                &local_BX_265.field_0xa = local_28;
                (&local_BX_265.field_0xa + 2) = local_28;
                uVar1 = (iVar8 + 0x20);
                ppcVar2 = ((iVar8 + 0x20) + 8);
                ppcVar2(0, uVar1, (uVar1 >> 0x10), local_BX_265, uVar10);
            }
            local_12 = local_12 + 1;
        }
    }
    return;
}

pub fn pass1_1028_4768(param_1: u32) {
    let mut u_var1: u32;

    uVar1 = pass1_1028_478a(param_1);
    if (((uVar1 >> 0x10) == 0) && (uVar1 < 1000)) {
        return 0;
    }
    return 1;
}

pub fn pass1_1028_478a(param_1: *mut astruct_44) {
    let mut iVar1: i32;
    let uVar2: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1028_b58e(param_1);
    iVar1 = CONCAT31(extraout_var, uVar2);
    _local_6 = CONCAT31(extraout_var, uVar2) & 0xffff | in_DX << 0x10;
    local_a = (iVar1 + 0x22);
    local_e = 0;
    if (((iVar1 + 0x24) | local_a) == 0) {
        return;
    }
    local_10 = (local_a + 4);
    local_12 = 0;
    while (local_12 < local_10) {
        pass1_1020_bb16(
            local_a,
            CONCAT22(unaff_SS, &local_1e),
            CONCAT22(unaff_SS, &local_1a),
            local_12,
        );
        if (0 < local_1a) {
            local_e = local_e + local_1e;
        }
        local_12 = local_12 + 1;
    }
    return;
}

pub fn pass1_1028_4810(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_4530(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_489e(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = &PTR_LOOP_1050_4942;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_48c0(param_1: *mut u8, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = &PTR_LOOP_1050_4942;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    (param_1 + 0xe) = (param_1 + 0xc);
    (param_1 + 0x10) = (param_1 + 0xc);
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_48fa(param_1: u32) {
    pass1_1028_bdac(param_1, 0);
    return;
}

pub fn pass1_1028_491c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_49aa(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x4b1c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x20)), 0, 10);
    return param_1;
}

pub fn pass1_1028_49de(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0x4b1c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    pass1_1000_4906(CONCAT22(param_2, param_1 + 0x20), 0, 10);
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_254c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_2042(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_25da(param_1: *mut *mut u8) -> *mut *mut u8 {
    pass1_1028_b354(param_1);
    unsafe {
        *param_1 = (s_fem16_wav_1050_2644 + 8);
        (param_1 + 2) = &PTR_LOOP_1050_1028;
    }
    return param_1;
}

pub fn pass1_1028_25fc(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = (s_fem16_wav_1050_2644 + 8);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2626(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_26b4(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_fem48_wav_1050_2784 + 4);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_26d6(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = (s_fem48_wav_1050_2784 + 4);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2700(param_1: u32) {
    let mut uVar1: i32;
    let mut local_DXAX_27: u32;

    pass1_1028_be2a(param_1);
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 5) {
        local_DXAX_27 = pass1_1028_b4f2((param_1 & 0xffff | uVar1 << 0x10));
        (local_DXAX_27 + 0x204) = 1;
    }
    return;
}

pub fn pass1_1028_272e(param_1: u32) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    uVar1 = pass1_1028_b4f2(param_1);
    if ((param_1 + 0x12) == 5) {
        (uVar1 + 0x204) = 1;
    }
    return;
}

pub fn pass1_1028_2762(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_27f0(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_fem123_wav_1050_2a89 + 9);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_2812(
    param_1: *mut astruct_759,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> u32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = (s_fem123_wav_1050_2a89 + 9);
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    param_1.field_0xe = param_1.field_0xc;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2844(
    param_1: u32,
    param_2: *mut u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_4_00: u32,
) -> i32 {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    unsafe { _local_8 = *param_2 };
    uStack4 = (param_2 + 1);
    pass1_1008_3eb4(
        CONCAT22(unaff_SS, &local_8),
        CONCAT22(unaff_SS, local_e),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    uVar1 = pass1_1028_c5a6(uVar2, uVar3, 0x7b, CONCAT22(unaff_SS, &local_8), param_4_00);
    if (uVar1 == 0) {
        uVar1 = switch_fn_1028_297c(param_1, CONCAT22(unaff_SS, &local_8), param_4_00);
        if (uVar1 == 0) {
            _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
            uVar1 = pass1_1028_c5a6(uVar2, uVar3, 0x7b, CONCAT22(unaff_SS, &local_8), param_4_00);
            if (uVar1 == 0) {
                uVar1 = switch_fn_1028_297c(param_1, CONCAT22(unaff_SS, &local_8), param_4_00);
                if (uVar1 == 0) {
                    local_8 = local_a - 1;
                    local_6 = local_c;
                    uVar1 = pass1_1028_c5a6(
                        uVar2,
                        uVar3,
                        0x7c,
                        CONCAT22(unaff_SS, &local_8),
                        param_4_00,
                    );
                    if (uVar1 == 0) {
                        uVar1 =
                            switch_fn_1028_297c(param_1, CONCAT22(unaff_SS, &local_8), param_4_00);
                        if (uVar1 == 0) {
                            _local_8 = CONCAT22(local_6, local_a + 1);
                            uVar1 = pass1_1028_c5a6(
                                uVar2,
                                uVar3,
                                0x7c,
                                CONCAT22(unaff_SS, &local_8),
                                param_4_00,
                            );
                            if (uVar1 == 0) {
                                uVar1 = switch_fn_1028_297c(
                                    param_1,
                                    CONCAT22(unaff_SS, &local_8),
                                    param_4_00,
                                );
                                if (uVar1 == 0) {
                                    return uVar1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 1;
}

pub fn switch_fn_1028_297c(param_1: u32, param_2: u32, param_3: u32) {
    let mut cVar1: u8;
    let mut iVar2: i32;
    let paVar3: *mut astruct_493;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let lVar6: u32;
    let mut uVar7: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_6: u16;

    iVar2 = pass1_1028_c7b6(param_1, param_2, param_3);
    if (iVar2 == 0) {
        lVar6 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_2, param_3);
        uVar4 = (lVar6 >> 0x10);
        uVar5 = uVar4 | lVar6;
        if (lVar6 != 0) {
            paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar6, uVar4);
            uVar7 = pass1_1030_73a8(CONCAT22(uVar5, paVar3));
            uVar4 = (uVar7 + 0xc);
            if (0x4a < uVar4) {
                todo!();
                //                 switch (uVar4)
                //                 {
                //                 0x4c =>{
                //                 0x4d =>{
                //                 0x4e =>{
                //                 0x60 =>{
                //                 0x61 =>{
                //                 0x62 =>{
                //                 99 =>{
                //                 0x6e =>{
                //                 0x73 =>{
                //                 0x74 =>{
                //                 0x75 =>{
                //                 0x76 =>{
                //                 0x77 =>{
                //                 0x78 =>{
                //                 0x79 =>{
                //                   // goto switchD_1028_2a0b_caseD_4c;
                // // default:
                //                   // goto switchD_1028_2a0b_caseD_4f;
                //                 }
            }
            if ((uVar4 < 0x48) && (uVar4 != 0x41)) {
                if (uVar4 < 0x42) {
                    cVar1 = uVar4;
                    if (cVar1 < '5') {
                        if ('2' < cVar1) {
                            return 0;
                        }
                        cVar1 = cVar1 + -0x10;
                    } else {
                        cVar1 = cVar1 + -0x3e;
                    }
                    if (cVar1 == '\0') {
                        return 0;
                    }
                }
                // switchD_1028_2a0b_caseD_4f:
                return 1;
            }
        }
    }
    // switchD_1028_2a0b_caseD_4c:
    return 0;
}

pub fn pass1_1028_2afa(param_1: *mut *mut u8) -> *mut *mut u8 {
    pass1_1030_dc96(param_1);
    unsafe {
        *param_1 = (s_add5_wav_1050_2b73 + 1);
        (param_1 + 2) = &PTR_LOOP_1050_1028;
    }
    return param_1;
}

pub fn pass1_1028_2b1c(
    param_1: *mut astruct_760,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> u32 {
    pass1_1030_dcc2(param_1, param_2, param_3, param_3_00);
    *CONCAT22(param_2, param_1) = (s_add5_wav_1050_2b73 + 1);
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_2b4e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_2bdc(param_1: *mut astruct_742) -> *mut astruct_742 {
    pass1_1028_0954(param_1);
    param_1 = 0x341c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_2bfe(param_1: *mut astruct_743, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_3_00);
    CONCAT22(param_2, param_1) = 0x341c;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_2c28(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let mut iVar1: i32;
    let mut unaff_SS: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    iVar1 = pass1_1028_09d4(param_1, param_2, param_3, param_4);
    if (iVar1 != 0) {
        local_8 = param_2;
        uStack4 = (param_2 + 4);
        pass1_1008_3eb4(
            CONCAT22(unaff_SS, &local_8),
            CONCAT22(unaff_SS, &local_e),
            CONCAT22(unaff_SS, &local_c),
            CONCAT22(unaff_SS, &local_a),
        );
        pass1_1008_3e76(
            CONCAT22(unaff_SS, &local_8),
            local_e,
            local_c - 1,
            local_a - 1,
        );
        iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
        if (iVar1 != 0) {
            pass1_1008_3e76(CONCAT22(unaff_SS, &local_8), local_e, local_c - 1, local_a);
            iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
            if (iVar1 != 0) {
                pass1_1008_3e76(
                    CONCAT22(unaff_SS, &local_8),
                    local_e,
                    local_c - 1,
                    local_a + 1,
                );
                iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
                if (iVar1 != 0) {
                    pass1_1008_3e76(CONCAT22(unaff_SS, &local_8), local_e, local_c, local_a - 1);
                    iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
                    if (iVar1 != 0) {
                        pass1_1008_3e76(CONCAT22(unaff_SS, &local_8), local_e, local_c, local_a);
                        iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
                        if (iVar1 != 0) {
                            pass1_1008_3e76(
                                CONCAT22(unaff_SS, &local_8),
                                local_e,
                                local_c,
                                local_a + 1,
                            );
                            iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
                            if (iVar1 != 0) {
                                pass1_1008_3e76(
                                    CONCAT22(unaff_SS, &local_8),
                                    local_e,
                                    local_c + 1,
                                    local_a - 1,
                                );
                                iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
                                if (iVar1 != 0) {
                                    pass1_1008_3e76(
                                        CONCAT22(unaff_SS, &local_8),
                                        local_e,
                                        local_c + 1,
                                        local_a,
                                    );
                                    iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
                                    if (iVar1 != 0) {
                                        pass1_1008_3e76(
                                            CONCAT22(unaff_SS, &local_8),
                                            local_e,
                                            local_c + 1,
                                            local_a + 1,
                                        );
                                        iVar1 =
                                            pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
                                        if (iVar1 != 0) {
                                            return 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1028_2e84(param_1: u16, param_2: u32) {
    let ppVar1: *mut pass1_struct_1;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let uVar4: u8;
    let uVar5: u8;
    let uVar6: u8;
    let uVar7: u8;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut in_stack_0000ffee: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    pass1_1028_09b8(param_1, param_2);
    if ((param_2 >> 0x10) == 0) {
        uVar9 = 0;
        uVar10 = 8;
        uVar6 = 1;
        uVar7 = 0;
        uVar8 = 0;
        uVar3 = 0;
        uVar4 = 0;
        uVar5 = 0;
        uVar2 = 0;
        ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(
            ppVar1,
            CONCAT22(uVar3, uVar2),
            CONCAT11(uVar5, uVar4),
            CONCAT11(uVar7, uVar6),
            CONCAT22(uVar9, uVar8),
            uVar10,
        );
        uVar3 = 0x400;
        uVar10 = 3;
        uVar2 = 1;
        ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
        pass1_1010_043a(ppVar1, CONCAT22(uVar3, uVar2), uVar10);
        pass1_1010_043a(ppVar1, 0x4000001, 4);
        ppVar1 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffee, 0x2f));
        pass1_1010_ec84(ppVar1);
        ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffee, 8));
        pass1_1010_9766(ppVar1);
    }
    return;
}

pub fn pass1_1028_2f18(param_1: *mut astruct_44) {
    let mut iVar1: i32;
    let uVar2: u8;
    let puVar3: *mut u32;
    let extraout_var: u32;
    let mut uVar4: i32;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_950: u16;
    let mut local_948: u16;
    let mut local_946: u16;
    let mut local_944: u16;
    let mut local_942: u16;
    let mut local_820: u16;
    let mut local_81e: u16;
    let mut local_6fc: u16;
    let mut local_6fa: u16;
    let mut local_5d8: u16;
    let mut local_5d6: u16;
    let mut local_4b4: u16;
    let mut local_4b2: u16;
    let mut local_390: u32;
    let mut local_38a: u16;
    let mut local_388: u16;
    let mut local_266: u16;
    let mut local_264: u16;
    let mut local_142: u16;
    let mut local_140: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut uStack20: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = pass1_1028_bb24(param_1);
    uVar4 = (_local_6 >> 0x10);
    uVar2 = pass1_1028_b58e(param_1);
    iVar1 = CONCAT31(extraout_var, uVar2);
    _local_a = CONCAT31(extraout_var, uVar2) & 0xffff | uVar4 << 0x10;
    local_e = (iVar1 + 0x2e);
    local_12 = (local_e + 4);
    local_18 = (iVar1 + 0xc);
    uStack20 = (iVar1 + 0x10);
    pass1_1008_3eb4(
        CONCAT22(unaff_SS, &local_18),
        CONCAT22(unaff_SS, &local_1e),
        CONCAT22(unaff_SS, &local_1c),
        CONCAT22(unaff_SS, &local_1a),
    );
    pass1_1008_3e76(
        CONCAT22(unaff_SS, &local_18),
        local_1e,
        local_1c - 1,
        local_1a - 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_142),
        0,
        0,
        0xd,
        &local_18,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_142));
    pass1_1008_3e76(
        CONCAT22(unaff_SS, &local_18),
        local_1e,
        local_1c + 1,
        local_1a + 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_266),
        0,
        0,
        0xc,
        &local_18,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_266));
    pass1_1008_3e76(
        CONCAT22(unaff_SS, &local_18),
        local_1e,
        local_1c + 1,
        local_1a - 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_38a),
        0,
        0,
        0xe,
        &local_18,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_38a));
    pass1_1008_3e54(
        CONCAT22(unaff_SS, &local_390),
        local_1e,
        local_1c - 1,
        local_1a + 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_4b4),
        0,
        0,
        0xb,
        &local_390,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_4b4));
    pass1_1008_3e76(
        CONCAT22(unaff_SS, &local_18),
        local_1e,
        local_1c - 1,
        local_1a,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_5d8),
        0,
        0,
        0x7a,
        &local_18,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_5d8));
    pass1_1008_3e76(
        CONCAT22(unaff_SS, &local_18),
        _local_1e,
        (_local_1e >> 0x10),
        local_1a + 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_6fc),
        0,
        0,
        0x7a,
        &local_18,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_6fc));
    pass1_1008_3e76(
        CONCAT22(unaff_SS, &local_18),
        local_1e,
        local_1c + 1,
        local_1a,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_820),
        0,
        0,
        0x7a,
        &local_18,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_820));
    pass1_1008_3e76(
        CONCAT22(unaff_SS, &local_18),
        _local_1e,
        (_local_1e >> 0x10),
        local_1a - 1,
    );
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_944),
        0,
        0,
        0x7a,
        &local_18,
        unaff_SS,
        local_12,
        _local_6,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_944));
    puVar3 = &local_390;
    pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, puVar3), _local_6);
    uVar5 = (local_e >> 0x10);
    (local_e + 0x10) = puVar3;
    (local_e + 0x12) = extraout_DX;
    return;
}

pub fn pass1_1028_3246(param_1: *mut astruct_44) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut in_stack_0000ffd8: u16;
    let mut local_20: [u8; 6];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10;
    local_a = (CONCAT31(extraout_var, uVar1) + 0x2e);
    local_e = (local_a + 0x10);
    uVar3 = 0;
    uVar4 = 1;
    uVar2 = 1;
    local_12 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_e, (local_e >> 0x10));
    local_10 = in_DX;
    pass1_1030_7c50(CONCAT22(in_DX, local_12), CONCAT22(uVar3, uVar2), uVar4);
    pass1_1030_7c50(CONCAT22(local_10, local_12), 1, 2);
    pass1_1030_7c50(CONCAT22(local_10, local_12), 1, 3);
    pass1_1030_7c50(CONCAT22(local_10, local_12), 1, 5);
    _local_1a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffd8, 2));
    if ((_local_1a + 0x82) == 0) {
        pass1_1030_7c50(CONCAT22(local_10, local_12), 4, 4);
    }
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 200, 0x11);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 200, 0x12);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 200, 0x13);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 200, 0x14);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 0x14, 0x15);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 0x14, 0x16);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 200, 0x17);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 200, 0x18);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 200, 0x19);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 0x14, 0x1a);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 0x14, 0x1b);
    pass1_1030_7ddc(CONCAT22(local_10, local_12), 0x14, 0x1c);
    if ((local_a + 0x200) == 0x8000002) {
        pass1_1020_a43e(CONCAT22(unaff_SS, local_20));
        pass1_1020_a89e(
            CONCAT22(unaff_SS, local_20),
            (_local_6 + 0xc),
            (_local_6 >> 0x10),
        );
    }
    return;
}

pub fn pass1_1028_33f6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_3484(param_1: *mut astruct_727) -> *mut astruct_763 {
    pass1_1028_0068(param_1);
    param_1 = 0x34f6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_34a6(
    param_1: *mut astruct_728,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut astruct_728 {
    pass1_1028_00cc(
        param_1,
        CONCAT22(param_2, param_1._2_2_),
        CONCAT22(param_3_00, param_3),
    );
    param_1.field_0x0 = 0x34f6;
    (param_1).field_0x2 = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_34d0(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_0138(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_355e(param_1: *mut astruct_763) -> *mut astruct_763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x3608;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_3580(
    param_1: *mut astruct_761,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut astruct_761 {
    pass1_1028_b39e(param_1, param_2, CONCAT22(param_3_00, param_3));
    param_1.field_0x0 = 0x3608;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_35b0(param_1: *mut astruct_44, param_2: i32) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut uVar2: u16;

    uVar1 = pass1_1028_b58e(param_1);
    if (param_2 == 0) {
        uVar2 = 0;
    } else {
        uVar2 = 0x32;
    }
    pass1_1030_7d1c(
        CONCAT22(in_DX, CONCAT11(extraout_AH, uVar1)),
        uVar2,
        0x230000,
    );
    return;
}

pub fn pass1_1028_35e2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_3670(param_1: *mut *mut u8) -> *mut *mut u8 {
    pass1_1028_37a6(param_1);
    unsafe {
        *param_1 = 0x373e;
        (param_1 + 2) = &PTR_LOOP_1050_1028;
    }
    return param_1;
}

pub fn pass1_1028_3692(
    param_1: *mut astruct_762,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> *mut astruct_762 {
    pass1_1028_3816(
        param_1,
        CONCAT22(param_2, param_1._2_2_),
        CONCAT22(param_3_00, param_3),
    );
    param_1 = 0x373e;
    (param_1).field_0x2 = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_36bc(param_1: u32, param_2: u32) {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut in_DX: i32;
    let mut uVar5: u16;
    let mut local_4: u16;

    param_2 = 0;
    uVar5 = (param_1 >> 0x10);
    if ((param_1 + 0x28) != 0) {
        local_4 = 4;
        while (local_4 < 0x1d) {
            uVar3 = (param_1 + 0x28);
            uVar4 = pass1_1020_bae6(uVar3, CONCAT22(local_4, (uVar3 >> 0x10)));
            uVar2 = param_2;
            param_2 = param_2 + uVar4;
            piVar1 = (param_2 + 2);
            unsafe {
                *piVar1 = *piVar1 + in_DX + CARRY2(uVar2, uVar4);
            }
            if (0xf9 < param_2) {
                return;
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn pass1_1028_3718(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_388e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_37a6(param_1: *mut astruct_763) {
    let mut uVar1: i32;
    let struct_a: *mut astruct_199;
    let paVar2: *mut astruct_199;
    let mut extraout_DX: u16;
    let local_BX_13: *mut astruct_763;
    let mut uvar3: u16;
    let mut local_4: u16;

    pass1_1028_b354(param_1);
    uVar3 = (param_1 >> 0x10);
    local_BX_13 = param_1;
    uVar1 = 0;
    local_BX_13.field_0x20 = 0;
    local_BX_13.field_0x24 = 0;
    &local_BX_13.field_0x28 = 0;
    param_1.field_0x0 = 0x3e2c;
    local_BX_13.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar2 | uVar1) == 0) {
        &local_BX_13.field_0x28 = 0;
    } else {
        pass1_1020_ba3e(CONCAT22(paVar2, uVar1), 5, 5);
        local_BX_13.field_0x28 = uVar1;
        local_BX_13.field_0x2a = extraout_DX;
    }
    return;
}

pub fn pass1_1028_3816(param_1: *mut astruct_764, param_2: u32, param_3: u32) {
    let mut uVar1: i32;
    let struct_a: *mut astruct_199;
    let paVar2: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = param_2;
    pass1_1028_b39e(CONCAT22(uVar3, param_1), (param_2 >> 0x10), param_3);
    uVar1 = 0;
    param_1.field_0x20 = 0;
    param_1.field_0x24 = 0;
    &param_1.field_0x28 = 0;
    CONCAT22(uVar3, param_1) = 0x3e2c;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar2 | uVar1) == 0) {
        &param_1.field_0x28 = 0;
    } else {
        pass1_1020_ba3e(CONCAT22(paVar2, uVar1), 5, 5);
        param_1.field_0x28 = uVar1;
        param_1.field_0x2a = extraout_DX;
    }
    return;
}

pub fn pass1_1028_388e(param_1: *mut astruct_44) {
    let mut uVar1: i32;
    let in_struct_1: *mut astruct_44;
    let local_BX_4: *mut astruct_44;
    let mut uVar2: u16;
    let mut local_6: u32;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.ptr_a_lo = 0x3e2c;
    local_BX_4.ptr_a_hi = &PTR_LOOP_1050_1028;
    in_struct_1 = &local_BX_4.field_0x28;
    uVar1 = &local_BX_4.field_0x2a;
    if ((uVar1 | in_struct_1) != 0) {
        pass1_1020_ba7e((in_struct_1 & 0xffff | uVar1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_38d4(param_1: *mut u8, param_2: *mut u8, param_3: u32, param_4: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let fn_ptr_1: fn();

    iVar1 = pass1_1028_c7b6(param_1, param_2, param_4);
    if ((iVar1 == 5) || (iVar1 == 6)) {
        uVar2 = (param_1 >> 0x10);
        fn_ptr_1 = (param_1 + 0x60);
        iVar1 = (**fn_ptr_1)();
        if (iVar1 != 0) {
            pass1_1028_c23e(param_1, uVar2, param_2, param_3, param_4);
            if (iVar1 != 0) {
                uVar2 =
                    pass1_1028_c314(param_1, uVar2, param_2, param_3, (param_3 >> 0x10), param_4);
                if (uVar2 != 0) {
                    return 1;
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1028_3958(param_1: *mut astruct_44) {
    let plVar1: *mut long;
    let mut iVar2: i32;
    let Var3: u16;
    let uVar4: u8;
    let mut uVar5: i32;
    let extraout_var: u32;
    let mut uVar6: u32;
    let mut in_DX: u16;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut uVar11: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_2c: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff6159465a: *mut astruct_768;

    local_4 = in_DX;
    uVar4 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar4);
    local_a = (local_6 + 0x22);
    uVar8 = (local_6 + 0x24);
    uVar6 = uVar8;
    if ((uVar8 | local_a) != 0) {
        if (PTR_LOOP_1050_574c != 0x0) {
            _local_1e = *(local_a + 4);
            local_22 = 0;
            while (local_22 < _local_1e) {
                pass1_1020_bb16(
                    local_a,
                    CONCAT22(unaff_SS, &local_2c),
                    CONCAT22(unaff_SS, &local_28),
                    local_22,
                );
                local_22 = local_22 + 1;
            }
        }
        local_e = (local_6 + 0x2e);
        local_12 = *_PTR_LOOP_1050_65e2;
        local_14 = local_12 & 1;
        local_16 = 4;
        while (uVar8 = uVar6, local_16 < 0xe) {
            local_2c = local_16;
            uVar6 = pass1_1020_bae6(local_a, CONCAT22(local_16, (local_a >> 0x10)));
            local_28 = uVar6 & 0xffff | uVar8 << 0x10;
            uVar8 = uVar8 | uVar6;
            uVar6 = uVar8;
            if (uVar8 != 0) {
                pass1_1020_bb8a(local_a, 0, 0, local_2c);
                uVar8 = -(local_28._2_2_ + (local_28 != 0));
                uVar6 = uVar8;
                local_22 = CONCAT22(uVar8, -local_28);
                pass1_1038_5694(local_e, CONCAT22(uVar8, -local_28), local_2c);
                _local_1e = 0;
                local_24 = 0;
                iVar9 = param_1;
                uVar10 = (param_1 >> 0x10);
                match (local_16) {
                    4 => {
                        _local_1e = local_28 >> 1;
                        if ((_local_1e == 0) && (local_14 != 0)) {
                            _local_1e = 1;
                        }
                        local_24 = 0x11;
                    }
                    5 => {
                        _local_1e = local_28 >> 1;
                        if ((_local_1e == 0) && (local_14 != 0)) {
                            _local_1e = 1;
                        }
                        local_24 = 0x12;
                    }
                    6 => {
                        _local_1e = local_28 >> 1;
                        if ((_local_1e == 0) && (local_14 != 0)) {
                            _local_1e = 1;
                        }
                        local_24 = 0x13;
                    }
                    7 => {
                        _local_1e = local_28 >> 1;
                        if ((_local_1e == 0) && (local_14 != 0)) {
                            _local_1e = 1;
                        }
                        local_24 = 0x14;
                    }
                    8 => {
                        _local_1e = local_28;
                        local_24 = 0x1a;
                    }
                    9 => {
                        _local_1e = local_28;
                        local_24 = 0x1b;
                    }
                    10 => {
                        _local_1e = local_28;
                        local_24 = 0x1c;
                    }
                    0xb => {
                        _local_1e = local_28;
                        local_24 = 0x17;
                    }
                    0xc => {
                        local_24 = 0x18;
                        _local_1e = local_28;
                        unsafe {
                            plVar1 = (iVar9 + 0x20);
                            *plVar1 = *plVar1 + local_28;
                        }
                        uVar8 = (iVar9 + 0x20);
                        uVar5 = (iVar9 + 0x22);
                        uVar7 = uVar8 >> 1 | ((uVar5 & 1) != 0) << 0xf;
                        _local_34 = CONCAT22(uVar5 >> 1, uVar7);
                        uVar7 = (uVar5 & 0xfffe) + CARRY2(uVar7, uVar7);
                        uVar6 = uVar7;
                        (iVar9 + 0x20) = uVar8 - (uVar8 & 0xfffe);
                        (iVar9 + 0x22) = (uVar5 - uVar7) - (uVar8 < (uVar8 & 0xfffe));
                        if (_local_34 != 0) {
                            uVar11 = 0x15;
                            // LAB_1028_3b14:
                            _local_1e = local_28;
                            pass1_1020_bb8a((iVar9 + 0x28), _local_34, (_local_34 >> 0x10), uVar11);
                        }
                    }
                    0xd => {
                        local_24 = 0x19;
                        _local_1e = local_28;
                        unsafe {
                            plVar1 = (iVar9 + 0x24);
                            *plVar1 = *plVar1 + local_28;
                        }
                        uVar8 = (iVar9 + 0x24);
                        iVar2 = (iVar9 + 0x26);
                        qVar3 = (local_28 & 0xffff0000 | uVar8) / 3;
                        _local_34 = qVar3;
                        local_32 = (qVar3 >> 0x10);
                        uVar5 = qVar3;
                        uVar7 = local_32 * 3 + CARRY2(uVar5, uVar5) + CARRY2(uVar5 * 2, uVar5);
                        uVar6 = uVar7;
                        (iVar9 + 0x24) = uVar8 + uVar5 * -3;
                        (iVar9 + 0x26) = (iVar2 - uVar7) - (uVar8 < uVar5 * 3);
                        if (_local_34 != 0) {
                            uVar11 = 0x16;
                            // goto LAB_1028_3b14;
                        }
                        if (((local_1c | local_1e) != 0) && (local_24 != 0)) {
                            pass1_1020_bb70(
                                *(iVar9 + 0x28),
                                local_1e,
                                CONCAT22(local_24, local_1c),
                            );
                        }
                    }
                }
                local_16 = local_16 + 1;
            }
        }
        return;
    }
}

pub fn pass1_1028_3c32(param_1: *mut *mut u8) {
    let mut iVar1: i32;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    fn_ptr_1 = (param_1 + 0x40);
    iVar1 = (**fn_ptr_1)();
    if (iVar1 != 0) {
        return 0x0;
    }
    return CONCAT22(-(1000 < local_6) - local_4, 1000 - local_6);
}

pub fn pass1_1028_3c60(param_1: *mut astruct_770, param_2: u32) {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    // ppuVar3: *mut *mut u8;
    let mut uVar4: u32;
    let mut in_DX: i32;
    let local_BX_15: *mut astruct_770;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_10: u32;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    param_2 = 0;
    uVar5 = (param_1 >> 0x10);
    local_BX_15 = param_1;
    if (local_BX_15.field_0x28 != 0x0) {
        local_8 = 4;
        while (local_8 < 0x1d) {
            ppuVar3 = local_BX_15.field_0x28;
            uVar4 = pass1_1020_bae6(ppuVar3, CONCAT22(local_8, (ppuVar3 >> 0x10)));
            uVar2 = param_2;
            param_2 = param_2 + uVar4;
            piVar1 = (param_2 + 2);
            unsafe { *piVar1 = *piVar1 + in_DX + CARRY2(uVar2, uVar4) };
            if (999 < param_2) {
                return;
            }
            local_8 = local_8 + 1;
        }
    }
    ppuVar3 = local_BX_15.field_0x28;
    local_4 = (ppuVar3 + 4);
    local_6 = 0;
    while (true) {
        if (local_4 <= local_6) {
            return;
        }
        pass1_1020_bb16(
            local_BX_15.field_0x28,
            CONCAT22(unaff_SS, &local_10),
            CONCAT22(unaff_SS, local_c),
            local_6,
        );
        param_2 = param_2 + local_10;
        if (999 < param_2) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn pass1_1028_1556(param_1: u32) -> bool {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let uVar3: u8;
    let mut in_AX: u16;
    let extraout_AH: u8;
    let paVar4: *mut astruct_493;
    let BVar5: bool;
    let mut in_DX: u16;
    let mut uVar6: i32;
    let mut unaff_SS: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_bab6(param_1);
    local_8 = pass1_1030_2fac(CONCAT22(in_DX, in_AX));
    _local_c = pass1_1028_bb24(param_1);
    local_e = (_local_c >> 0x10);
    uVar3 = pass1_1028_b58e(param_1);
    local_10 = CONCAT11(extraout_AH, uVar3);
    local_16 = (local_10 + 0xc);
    local_1a = 1;
    while (true) {
        if (local_8 < local_1a) {
            return 0;
        }
        local_12 = local_1a;
        uVar7 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, &local_16), _local_c);
        uVar6 = (uVar7 >> 0x10);
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7, uVar6);
        if ((uVar6 | paVar4) == 0) {
            return 0;
        }
        uVar8 = pass1_1030_73a8(CONCAT22(uVar6, paVar4));
        uVar2 = (uVar8 >> 0x10);
        if (uVar8 == 0) {
            return 0;
        }
        iVar1 = (uVar8 + 0xc);
        BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x13);
        if ((BVar5 == 0) && (iVar1 != 0x75)) {
            break;
        }
        if ((uVar8 + 0x12) != 9) {
            return 1;
        }
        local_1a = local_1a + 1;
    }
    return 0;
}

pub fn pass1_1028_1646(param_1: *mut astruct_750) -> *mut astruct_750 {
    let paVar1: *mut astruct_750;
    let local_BX_3: *mut astruct_750;
    let local_ES_3: *mut astruct_750;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    paVar1 = (local_BX_3.field_0x20 - 4);
    if (paVar1 < &BYTE_1050_0009) {
        match (paVar1) {
            0x0 => {
                local_BX_3.field_0x20 = 5;
            }
            0x1 => {
                local_BX_3.field_0x20 = 6;
            }
            0x2 => {
                local_BX_3.field_0x20 = 7;
            }
            0x3 => {
                local_BX_3.field_0x20 = 8;
            }
            0x4 => {
                local_BX_3.field_0x20 = 9;
            }
            0x5 => {
                local_BX_3.field_0x20 = 10;
                return local_BX_3;
            }
            0x6 => {
                local_BX_3.field_0x20 = 0xb;
                return local_BX_3;
            }
            0x7 => {
                local_BX_3.field_0x20 = 0xc;
                return local_BX_3;
            }
            0x8 => {
                local_BX_3.field_0x20 = 0xd;
                return local_BX_3;
            }
        }
        return local_BX_3;
    }
    local_BX_3.field_0x20 = 4;
    return paVar1;
}

pub fn pass1_1028_16fe(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_178c(param_1: *mut u16) {
    pass1_1030_dc96(param_1);
    unsafe { *param_1 = s_42_flc_1050_1b54 };
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_17ae(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1030_dcc2(param_1, param_2, param_3, param_3_00);
    CONCAT22(param_2, param_1) = s_42_flc_1050_1b54;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_17d8(param_1: u16, param_2: u16, param_3: u16) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_df0c(param_1, param_2, param_3);
    uVar2 = extraout_DX;
    uVar1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    pass1_1038_57dc(
        *(CONCAT31(extraout_var, uVar1) + 0x2e),
        0x1,
        (&dos_alloc_addr_1050_0002 + 1),
    );
    return;
}

pub fn pass1_1028_1812(param_1: u32) {
    pass1_1028_bdac(param_1, 2);
    return;
}

pub fn pass1_1028_1824(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let uVar1: u8;
    let mut in_AX: i32;
    let mut uVar2: u16;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_30: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar2 = (param_1 >> 0x10);
    pass1_1028_c3aa(param_1, uVar2, param_2, param_3, param_4);
    if (in_AX == 0) {
        return;
    }
    uVar2 = pass1_1028_c314(param_1, uVar2, param_2, param_3, (param_3 >> 0x10), param_4);
    if (uVar2 == 0) {
        return;
    }
    puVar3 = &local_c;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_4, puVar3, unaff_SS);
    unsafe { local_6 = *puVar3 };
    local_1e = (puVar3 + 2);
    local_8 = (param_2 + 4);
    _local_16 = (local_6 & 0xffff | local_1e << 0x10);
    local_20 = local_6;
    local_1e._1_1_ = (local_1e >> 8);
    if (local_1e._1_1_ != '\0') {
        local_20 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, local_1e);
        uVar1 = pass1_1030_6fa0(CONCAT22(local_1e, local_20));
        local_1c = CONCAT31(extraout_var, uVar1);
        if (local_1c == 0x10) {
            if (local_8 != 0) {
                PTR_LOOP_1050_50ca = 0x6ab;
                return;
            }
            return;
        }
        if ((local_1c == 0x60) || (local_1c == 0x61)) {
            _local_16 =
                process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
            uVar5 = pass1_1018_04b8(_local_16);
            local_22 = (uVar5 >> 0x10);
            local_10 = uVar5;
            local_18 = (_local_16 + 0x1e);
            local_e = local_22;
            local_24 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_10, local_22);
            uVar2 = pass1_1030_2fac(CONCAT22(local_22, local_24));
            if (uVar2 <= local_18) {
                PTR_LOOP_1050_50ca = 0x6ac;
                return;
            }
            local_2a = param_2;
            local_26 = local_8 + 1;
            puVar4 = &local_2a;
            pass1_1028_c7b6(param_1, puVar4, unaff_SS, param_4);
            if (puVar4 == 0x0) {
                return;
            }
            if (puVar4 == (&PTR_DAT_0005_0000_1050_0004 + 2)) {
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = 0x6aa;
    return;
}

pub fn pass1_1028_199a(param_1: *mut astruct_44) {
    let piVar1: *mut i32;
    let uVar2: u8;
    let local_AX__1: *mut astruct_751;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut unaff_SI: u16;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let puVar4: *mut u16;
    let mut uVar5: u16;
    let puVar6: *mut u16;
    let mut uVar7: u16;
    let mut local_15e: u16;
    let mut local_15c: u16;
    let mut local_32: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    piVar1 = (param_1 + 0x14);
    unsafe { *piVar1 = *piVar1 + -1 };
    let pi_var1_val = unsafe { *piVar1 };
    if (pi_var1_val == 0) {
        uVar2 = pass1_1028_b58e(param_1);
        local_6 = CONCAT31(extraout_var, uVar2);
        local_a = *(local_6 + 0x2e);
        local_4 = in_DX;
        pass1_1038_5804(local_a, 1, (&dos_alloc_addr_1050_0002 + 1));
        local_10 = (local_6 + 0xc);
        uStack12 = (local_6 + 0x10);
        local_32 = &local_10;
        pass1_1008_3eb4(
            CONCAT22(unaff_SS, &local_10),
            CONCAT22(unaff_SS, &local_16),
            CONCAT22(unaff_SS, &local_14),
            CONCAT22(unaff_SS, &local_14 + 2),
        );
        _local_1a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
        local_1e = (_local_1a + 0x20);
        puVar6 = &local_20;
        local_24 = &local_22;
        puVar4 = local_24;
        uVar5 = unaff_SS;
        uVar7 = unaff_SS;
        local_26 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_1e, (local_1e >> 0x10));
        pass1_1030_5b1c(
            CONCAT22(local_24, local_26),
            CONCAT22(uVar5, puVar4),
            CONCAT22(uVar7, puVar6),
        );
        if (local_22 < (local_16 + 1)) {
            pass1_1030_5b3e(CONCAT22(local_24, local_26), local_16 + 1, local_20);
            pass1_1030_5b1c(
                CONCAT22(local_24, local_26),
                CONCAT22(unaff_SS, &local_22),
                CONCAT22(unaff_SS, &local_20),
            );
        }
        uVar3 = (local_a >> 0x10);
        local_2a = (local_a + 4);
        pass1_1028_87f0(
            CONCAT22(unaff_SS, &local_15e),
            0,
            0,
            (-(local_16 == 0) & 0xffd3) + 0x60,
            &local_10,
            unaff_SS,
            local_2a & 0xffff | *(local_a + 6) << 0x10,
            local_1e,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_15e));
        local_15e = s_1_1050_389a;
        local_15c = &PTR_LOOP_1050_1008;
        pass1_1008_3e76(
            CONCAT22(unaff_SS, &local_10),
            local_16 + 1,
            local_14,
            (local_14 >> 0x10),
        );
        pass1_1028_87f0(
            CONCAT22(unaff_SS, &local_15e),
            0,
            0,
            0x60,
            &local_10,
            unaff_SS,
            local_2a,
            local_1e,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_15e));
    }
    return;
}

pub fn pass1_1028_1b1e(param_1: u32) {
    (param_1 + 0x14) = 7;
    return;
}

pub fn pass1_1028_1b2e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_1bbc(param_1: *mut astruct_752) -> *mut astruct_752 {
    let local_BX_14: *mut astruct_752;
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_14 = param_1;
    local_BX_14.field_0x20 = 0;
    local_BX_14.field_0x22 = 0;
    param_1 = (s_526_bmp_1050_1ee7 + 7);
    local_BX_14.field_0x2 = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_1be8(param_1: *mut astruct_753, param_2: u32, param_3: u32) {
    let mut uVar1: u16;

    uVar1 = param_2;
    pass1_1028_b39e(CONCAT22(uVar1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0x20 = 0;
    param_1.field_0x22 = 0;
    CONCAT22(uVar1, param_1) = (s_526_bmp_1050_1ee7 + 7);
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(uVar1, param_1);
}

pub fn return_false_1028_1c1c() -> bool {
    return 0;
}

pub fn pass1_1028_1c22(param_1: *mut astruct_754) {
    let mut uVar1: u16;
    let local_BX_3: *mut astruct_754;
    let mut uVar2: i32;

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if ((local_BX_3.field_0x20 != 0)
        && (local_BX_3.field_0x12 == 5 || (local_BX_3.field_0x12 == 6)))
    {
        if (local_BX_3.field_0xc == 0x16) {
            return 0x19;
        }
        if (local_BX_3.field_0xc == 0x17) {
            return 0x1a;
        }
    }
    uVar1 = pass1_1028_bc1c((param_1 & 0xffff | uVar2 << 0x10));
    return uVar1;
}

pub fn pass1_1028_1c66(param_1: *mut *mut astruct_758) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let local_BX_4: *mut astruct_755;
    let mut uVar3: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_BX_4 = param_1;
    if (local_BX_4.field_0x12 != 6) {
        return;
    }
    uVar3 = pass1_1028_b4f2(param_1);
    if ((uVar3 + 0x200) != 0x8000002) {
        ppcVar1 = (param_1 + 100);
        iVar2 = (**ppcVar1)();
        if (iVar2 == 0) {
            return;
        }
        iVar2 = pass1_1028_cb04(param_1);
        if (iVar2 == 0) {
            iVar2 = 6;
            // goto LAB_1028_1cbd;
        }
        pass1_1028_c952(param_1);
    }
    iVar2 = 5;
    // LAB_1028_1cbd:
    pass1_1028_bdac(param_1, iVar2);
    return;
}

pub fn pass1_1028_1cca(
    param_1: u32,
    param_2: *mut u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_4_00: u32,
) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    unsafe { _local_8 = *param_2 };
    uStack4 = (param_2 + 1);
    pass1_1008_3eb4(
        CONCAT22(unaff_SS, &local_8),
        CONCAT22(unaff_SS, local_e),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    BVar1 = pass1_1028_1e14(uVar2, uVar3, 0x16, CONCAT22(unaff_SS, &local_8), param_4_00);
    if (BVar1 == 0) {
        _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
        BVar1 = pass1_1028_1e14(uVar2, uVar3, 0x16, CONCAT22(unaff_SS, &local_8), param_4_00);
        if (BVar1 == 0) {
            local_8 = local_a - 1;
            local_6 = local_c;
            BVar1 = pass1_1028_1e14(uVar2, uVar3, 0x17, CONCAT22(unaff_SS, &local_8), param_4_00);
            if (BVar1 == 0) {
                _local_8 = CONCAT22(local_6, local_a + 1);
                BVar1 =
                    pass1_1028_1e14(uVar2, uVar3, 0x17, CONCAT22(unaff_SS, &local_8), param_4_00);
                if (BVar1 == 0) {
                    return BVar1;
                }
            }
        }
    }
    return 1;
}

pub fn pass1_1028_1da4(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    uVar5 = pass1_1030_bcae(local_4, unaff_SS);
    uVar4 = (uVar5 >> 0x10);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    uVar1 = &paVar2.field_0x10;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    puVar3 = local_4;
    pass1_1030_bcde(
        puVar3,
        unaff_SS,
        CONCAT22(uVar4, paVar2),
        param_1_00,
        param_5,
    );
    if (puVar3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

pub fn pass1_1028_1e14(
    param_1: u16,
    param_2: u16,
    param_1_00: i32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut iVar1: i32;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let lVar5: u32;
    let mut uVar6: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_2_00, param_5);
    uVar3 = (lVar5 >> 0x10);
    uVar4 = uVar3 | lVar5;
    if (lVar5 != 0) {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar5, uVar3);
        if ((uVar4 | paVar2) != 0) {
            uVar6 = pass1_1030_73a8(CONCAT22(uVar4, paVar2));
            if (uVar6 != 0) {
                iVar1 = (uVar6 + 0xc);
                if (((iVar1 == 0x18) || (iVar1 == 0x3f)) || (iVar1 == param_1_00)) {
                    return 1;
                }
            }
        }
    }
    return 0;
}

pub fn pass1_1028_1e8a(param_1: u32) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;

    uVar2 = (param_1 >> 0x10);
    if ((*(param_1 + 0x1a) & 2) == 0) {
        uVar4 = 0;
        uVar5 = 0x23;
        uVar3 = 1;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_DX, CONCAT11(extraout_AH, uVar1)),
            uVar3,
            CONCAT22(uVar5, uVar4),
        );
        pass1_1028_bdac(param_1, 6);
        return 0;
    }
    return 1;
}

pub fn pass1_1028_1ec8(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: This function may have set the stack p i32er

pub fn pass1_1028_1eee(param_1: u32, param_2: u8) {
    let pcVar1: *mut char;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut bVar5: u8;
    let mut cVar6: u8;
    let puVar7: *mut u8;
    let mut bVar8: u8;
    let mut in_DX: u16;
    let mut bVar9: u8;
    let puVar11: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_DI: i32;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut bVar12: bool;
    let mut bVar13: bool;
    let mut iVar10: i32;

    puVar11 = &stack0xfffe;
    cVar6 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar11 = puVar11 + -1;
        unsafe { *puVar11 = *unaff_BP };
        puVar7 = PTR_LOOP_1050_1028;
        cVar6 = cVar6 + -1;
        '\0' < cVar6
    } {}
    pbVar2 = unaff_SI + 0x1028;
    bVar8 = in_DX;
    unsafe {
        *pbVar2 = *pbVar2 ^ bVar8;
        _in(in_DX);
        pcVar1 = &stack0xfffe + unaff_SI;
        *pcVar1 = *pcVar1 + '(';
        bVar9 = *unaff_SI & 0x28;
        iVar10 = CONCAT11(0x10, bVar9);
        pbVar2 = unaff_SI + iVar10;
        *pbVar2 = *pbVar2 - bVar8;
        if ('\x10' < *(&PTR_LOOP_1050_1028 + unaff_DI)) {
            pbVar2 = unaff_SI + iVar10;
            bVar3 = *pbVar2;
            *pbVar2 = *pbVar2 - bVar8;
            pbVar2 = unaff_SI + iVar10;
            bVar4 = *pbVar2;
            *pbVar2 = *pbVar2 - bVar8;
            bVar5 = (bVar3 < bVar8) + 0xb5;
            bVar12 = ((bVar3 < bVar8) - 0x23) < 0x28 || bVar5 < (bVar4 < bVar8);
            bVar13 = CARRY1(s_fem79_wav_1050_28ba[5], bVar8);
            bVar3 = s_fem79_wav_1050_28ba[5] + bVar8;
            s_fem79_wav_1050_28ba[5] = bVar3 + bVar12;
            pcVar1 = &stack0xfffe + unaff_SI;
            *pcVar1 = *pcVar1 + bVar9 + (bVar13 || CARRY1(bVar3, bVar12));
            0x1026 = &g_alloc_addr_1050_1050;
            LOCK();
            pcVar1 = (&PTR_LOOP_1050_1028 + CONCAT11(s_501a_bmp_1050_2050[0] + '\x10', 0x28));
            *pcVar1 = *pcVar1 - bVar8;
            return CONCAT22(in_DX, CONCAT11(0x10, bVar5 - (bVar4 < bVar8)));
        }
    }
    0x1024 = param_1;
    0x1022 = unaff_CS;
    0x1020 = (s_523_bmp_1050_1ecf + 6);
    pass1_1028_b418(paRam00001024);
    if ((param_2 & 1) != 0) {
        0x1024 = param_1;
        0x1022 = unaff_CS;
        0x1020 = (s_525_bmp_1050_1edf + 5);
        error_check_1000_17ce(paRam00001024);
    }
    return param_1;
}

pub fn pass1_1028_1f56(param_1: *mut astruct_756) {
    let mut u_var1: u32;
    let mut uVar2: i32;
    let paVar3: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut uVar4: i32;
    let local_BX_13: *mut astruct_756;
    let mut uVar5: u16;
    let mut local_4: u16;

    pass1_1028_b354(param_1);
    uVar5 = (param_1 >> 0x10);
    local_BX_13 = param_1;
    uVar2 = 0;
    &local_BX_13.field_0x20 = 0;
    local_BX_13.field_0x24 = 0;
    param_1 = 0x2572;
    local_BX_13.field_0x2 = &PTR_LOOP_1050_1028;
    paVar3 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    uVar4 = paVar3 | uVar2;
    if (uVar4 == 0) {
        &local_BX_13.field_0x20 = 0;
    } else {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, uVar2));
        local_BX_13.field_0x20 = paVar3;
        &local_BX_13.field_0x22 = uVar4;
    }
    uVar1 = &local_BX_13.field_0x20;
    (uVar1 + 10) = 0;
    return;
}

pub fn pass1_1028_1fc8(param_1: *mut astruct_757, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut uVar2: i32;
    let paVar3: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut uVar4: i32;
    let mut local_4: u16;

    pass1_1028_b39e(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3);
    uVar2 = 0;
    &param_1.field_0x20 = 0;
    param_1.field_0x24 = 0;
    CONCAT22(param_2, param_1) = 0x2572;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    paVar3 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    uVar4 = paVar3 | uVar2;
    if (uVar4 == 0) {
        &param_1.field_0x20 = 0;
    } else {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, uVar2));
        param_1.field_0x20 = paVar3;
        &param_1.field_0x22 = uVar4;
    }
    uVar1 = &param_1.field_0x20;
    (uVar1 + 10) = 0;
    return;
}

pub fn pass1_1028_2042(param_1: *mut astruct_44) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut extraout_DX: i32;
    let local_BX_4: *mut astruct_44;
    let mut local_ES_4: u16;
    let mut local_DXAX_81: u32;
    let fn_ptr_1: *mut u32;
    let mut temp_5f32db2c2f: u32;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.ptr_a_lo = 0x2572;
    local_BX_4.ptr_a_hi = &PTR_LOOP_1050_1028;
    temp_5f32db2c2f = &local_BX_4.field_0x20;
    (temp_5f32db2c2f + 10) = 1;
    puVar1 = local_BX_4.field_0x20;
    local_DXAX_81._2_2_ = &local_BX_4.field_0x22;
    if ((local_DXAX_81._2_2_ | puVar1) != 0) {
        unsafe {
            ppcVar2 = *puVar1;
            ppcVar2();
        }
        local_DXAX_81._2_2_ = extraout_DX;
    }
    if ((_PTR_LOOP_1050_65e2 != 0) && (_PTR_LOOP_1050_5a64 != 0x0)) {
        local_DXAX_81._0_1_ = pass1_1028_b58e(param_1);
        pass1_1038_79f2(
            _PTR_LOOP_1050_5a64,
            CONCAT22(
                local_DXAX_81._2_2_,
                CONCAT11(local_DXAX_81._1_1_, local_DXAX_81),
            ),
        );
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn return_false_1028_20b0() -> bool {
    return 0;
}

pub fn pass1_1028_20b6(param_1: *mut astruct_44) {
    let uVar1: u8;
    let local_AH_46: u8;
    let BVar3: bool;
    let mut local_DX_31: u16;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let mut local_SS__1: u16;
    let mut local_16: [u8; 2];
    let mut iStack20: i32;
    let mut iStack18: i32;
    let mut uStack16: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut iVar2: i32;

    pass1_1028_be9e(param_1);
    uVar5 = (param_1 >> 0x10);
    uVar4 = param_1;
    if ((uVar4 + 0x12) == 5) {
        _local_6 = pass1_1028_bb24((param_1 & 0xffff | uVar5 << 0x10));
        local_DX_31 = (_local_6 >> 0x10);
        uVar1 = pass1_1028_b58e(param_1);
        iVar2 = CONCAT11(local_AH_46, uVar1);
        _local_a = CONCAT22(local_DX_31, iVar2);
        uStack16 = (iVar2 + 0xc);
        local_c = (iVar2 + 0x10);
        pass1_1008_3eb4(
            CONCAT22(local_SS__1, &uStack16),
            CONCAT22(local_SS__1, local_16),
            CONCAT22(local_SS__1, &iStack20),
            CONCAT22(local_SS__1, &iStack18),
        );
        uStack16 = uStack16 & 0xffff | (iStack20 - 1) << 0x10;
        BVar3 = pass1_1028_21ba(uVar4, uVar5, CONCAT22(local_SS__1, &uStack16), _local_6);
        if (BVar3 == 0) {
            uStack16 = uStack16 & 0xffff | (iStack20 + 1) << 0x10;
            BVar3 = pass1_1028_21ba(uVar4, uVar5, CONCAT22(local_SS__1, &uStack16), _local_6);
            if (BVar3 == 0) {
                uStack16 = CONCAT22(iStack20, iStack18 + -1);
                BVar3 = pass1_1028_21ba(uVar4, uVar5, CONCAT22(local_SS__1, &uStack16), _local_6);
                if (BVar3 == 0) {
                    uStack16 = uStack16 & 0xffff0000 | (iStack18 + 1);
                    BVar3 =
                        pass1_1028_21ba(uVar4, uVar5, CONCAT22(local_SS__1, &uStack16), _local_6);
                    if (BVar3 == 0) {
                        return;
                    }
                }
            }
        }
        pass1_1038_79b2(_PTR_LOOP_1050_5a64, _local_a);
    }
    return;
}

pub fn pass1_1028_21ba(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) -> bool {
    let paVar1: *mut astruct_493;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let lVar4: u32;
    let mut uVar5: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    lVar4 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    uVar2 = (lVar4 >> 0x10);
    uVar3 = uVar2 | lVar4;
    if (lVar4 != 0) {
        paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar4, uVar2);
        if ((uVar3 | paVar1) != 0) {
            uVar5 = pass1_1030_73a8(CONCAT22(uVar3, paVar1));
            if ((uVar5 != 0) && ((uVar5 + 0xc) == 0x40)) {
                return 1;
            }
        }
    }
    return 0;
}

pub fn pass1_1028_2220(
    param_1: u16,
    param_2: u16,
    param_1_00: i32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut iVar1: i32;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let lVar5: u32;
    let mut uVar6: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_2_00, param_5);
    uVar3 = (lVar5 >> 0x10);
    uVar4 = uVar3 | lVar5;
    if (lVar5 != 0) {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar5, uVar3);
        if ((uVar4 | paVar2) != 0) {
            uVar6 = pass1_1030_73a8(CONCAT22(uVar4, paVar2));
            if ((uVar6 != 0)
                && ((
                    iVar1 = (uVar6 + 0xc),
                    iVar1 == 0x40 || (iVar1 == param_1_00),
                )))
            {
                return 1;
            }
        }
    }
    return 0;
}

pub fn pass1_1028_2290(
    param_1: u32,
    param_2: *mut u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_4_00: u32,
) -> i32 {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    unsafe { _local_8 = *param_2 };
    uStack4 = (param_2 + 1);
    pass1_1008_3eb4(
        CONCAT22(unaff_SS, &local_8),
        CONCAT22(unaff_SS, local_e),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    uVar1 = pass1_1028_2220(uVar2, uVar3, 0x16, CONCAT22(unaff_SS, &local_8), param_4_00);
    if (uVar1 == 0) {
        _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
        uVar1 = pass1_1028_2220(uVar2, uVar3, 0x16, CONCAT22(unaff_SS, &local_8), param_4_00);
        if (uVar1 == 0) {
            local_8 = local_a - 1;
            local_6 = local_c;
            uVar1 = pass1_1028_2220(uVar2, uVar3, 0x17, CONCAT22(unaff_SS, &local_8), param_4_00);
            if (uVar1 == 0) {
                _local_8 = CONCAT22(local_6, local_a + 1);
                uVar1 =
                    pass1_1028_2220(uVar2, uVar3, 0x17, CONCAT22(unaff_SS, &local_8), param_4_00);
                if (uVar1 == 0) {
                    return uVar1;
                }
            }
        }
    }
    return 1;
}

pub fn pass1_1028_236a(param_1: *mut astruct_44) -> bool {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;

    uVar2 = (param_1 >> 0x10);
    if ((*(param_1 + 0x1a) & 2) == 0) {
        uVar4 = 0;
        uVar5 = 0x23;
        uVar3 = 1;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_DX, CONCAT11(extraout_AH, uVar1)),
            uVar3,
            CONCAT22(uVar5, uVar4),
        );
        pass1_1028_bdac(param_1, 6);
        return 0;
    }
    return 1;
}

pub fn pass1_1028_23a8(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    uVar5 = pass1_1030_bcae(local_4, unaff_SS);
    uVar4 = (uVar5 >> 0x10);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    uVar1 = &paVar2.field_0x10;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    puVar3 = local_4;
    pass1_1030_bcde(
        puVar3,
        unaff_SS,
        CONCAT22(uVar4, paVar2),
        param_1_00,
        param_5,
    );
    if (puVar3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

pub fn pass1_1028_04ee(param_1: u32, param_2: u32) {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let lVar5: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    param_2 = 0;
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x22));
    while {
        lVar5 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        if (lVar5 == 0) {
            return 0;
        }
        uVar2 = (lVar5 + 0xc);
        uVar4 = (param_2 >> 0x10);
        uVar3 = param_2;
        param_2 = param_2 + uVar2;
        piVar1 = (param_2 + 2);
        unsafe { *piVar1 = *piVar1 + CARRY2(uVar3, uVar2) };
        ((param_2 + 2) == 0) && (param_2 < 0x1e)
    } {}
    return 1;
}

pub fn pass1_1028_0550(param_1: u32) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    pass1_1028_be9e(param_1, uVar5);
    if ((param_1 + 0x12) == 5) {
        uVar4 = 0;
        uVar6 = 4;
        uVar3 = 1;
        uVar2 = extraout_DX;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar5 << 0x10));
        pass1_1030_7c50(
            CONCAT22(uVar2, CONCAT11(extraout_AH, uVar1)),
            CONCAT22(uVar4, uVar3),
            uVar6,
        );
    }
    return;
}

pub fn pass1_1028_0582(param_1: *mut astruct_734) {
    let puVar1: *mut u32;
    let plVar2: *mut long;
    let mut uVar3: u32;
    let ppcVar4: fn();
    let mut uVar5: u32;
    let uVar6: u8;
    let local_AX__1: *mut astruct_738;
    let puVar7: *mut u8;
    let local_AX_150: *mut astruct_733;
    let puVar8: *mut u16;
    let local_AX_402: *mut astruct_736;
    let mut uVar9: i32;
    let mut uVar10: u16;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let mut extraout_DX_05: i32;
    let mut extraout_DX_06: i32;
    let local_BX_4: *mut astruct_734;
    let mut iVar12: i32;
    let local_BX_486: *mut astruct_739;
    let local_BX_493: *mut astruct_737;
    let mut uVar13: i32;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut local_138: u16;
    let mut local_136: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let local_6: *mut astruct_735;
    let mut local_4: u16;
    let puVar11: *mut u8;

    uVar13 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar5 = local_BX_4.field_0x14;
    uVar14 = (uVar5 >> 0x10);
    iVar12 = uVar5;
    _local_6 = uVar5 & 0xffff0000 | (iVar12 + 0xa4);
    if (((iVar12 + 0xa6) != 0) && ((iVar12 + 0xac) != 0)) {
        pass1_1028_081e(param_1);
        uVar9 = local_BX_4.field_0x20;
        puVar1 = (_local_6 + 8);
        in_DX = extraout_DX;
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val < uVar9 || pu_var1_val == uVar9) {
            puVar7 = local_a;
            ppcVar4 = (param_1 + 0x40);
            (**ppcVar4)();
            in_DX = extraout_DX_00;
            if (puVar7 == 0x0) {
                if ((_local_6 + 2) == 0xc) {
                    _local_e = pass1_1028_b4f2(param_1);
                    in_DX = (_local_e >> 0x10);
                    local_12 = (_local_e + 0x1f6);
                    plVar2 = (local_12 + 0x170);
                    unsafe { *plVar2 = *plVar2 + 1 };
                } else {
                    local_12 = _PTR_LOOP_1050_68a2;
                    puVar11 = _PTR_LOOP_1050_68a2;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                    uVar9 = puVar11;
                    _local_26 = (puVar11 & 0xffff | extraout_DX_01 << 0x10);
                    if ((extraout_DX_01 | uVar9) == 0) {
                        local_22 = 0;
                    } else {
                        *_local_26 = s_1_1050_389a;
                        (uVar9 + 2) = &PTR_LOOP_1050_1008;
                        (uVar9 + 4) = 0;
                        (uVar9 + 6) = 0;
                        (uVar9 + 8) = 0;
                        (uVar9 + 10) = 0;
                        (uVar9 + 0xc) = 0;
                        *_local_26 = 0x56ce;
                        (uVar9 + 2) = 0x1018;
                        local_22 = _local_26;
                    }
                    uVar14 = (_local_6 >> 0x10);
                    iVar12 = _local_6;
                    uVar15 = (local_22 >> 0x10);
                    (local_22 + 6) = (iVar12 + 2);
                    (local_22 + 10) = (iVar12 + 6);
                    unaff_CS = 0x1020;
                    uVar10 = switch_statement_1020_c3b4((iVar12 + 2));
                    (local_22 + 0xc) = uVar10 * (_local_6 + 6);
                    uVar3 = local_BX_4.field_0x22;
                    ppcVar4 = (local_BX_4.field_0x22 + 4);
                    (**ppcVar4)(0x1020, uVar3, (uVar3 >> 0x10));
                    in_DX = extraout_DX_02;
                }
            }
            local_BX_4.field_0x20 = 0;
        }
    }
    uVar14 = (_local_6 >> 0x10);
    if (((_local_6 + 4) != 0) && ((_local_6 + 8) != 0)) {
        pass1_1028_081e(param_1);
        uVar9 = local_BX_4.field_0x20;
        puVar1 = (_local_6 + 8);
        in_DX = extraout_DX_03;
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val < uVar9 || pu_var1_val == uVar9) {
            puVar8 = &local_2a;
            ppcVar4 = (param_1 + 0x40);
            (**ppcVar4)(unaff_CS, param_1, uVar13);
            in_DX = extraout_DX_04;
            if (puVar8 == 0x0) {
                local_12 = _PTR_LOOP_1050_68a2;
                puVar11 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                uVar9 = puVar11;
                _local_26 = (puVar11 & 0xffff | extraout_DX_05 << 0x10);
                if ((extraout_DX_05 | uVar9) == 0) {
                    local_22 = 0;
                } else {
                    *_local_26 = s_1_1050_389a;
                    (uVar9 + 2) = &PTR_LOOP_1050_1008;
                    (uVar9 + 4) = 0;
                    (uVar9 + 6) = 0;
                    (uVar9 + 8) = 0;
                    (uVar9 + 10) = 0;
                    (uVar9 + 0xc) = 0;
                    *_local_26 = 0x56ce;
                    (uVar9 + 2) = 0x1018;
                    local_22 = _local_26;
                }
                uVar14 = (_local_6 >> 0x10);
                local_BX_486 = _local_6;
                uVar15 = (local_22 >> 0x10);
                local_BX_493 = local_22;
                local_BX_493.field_0x8 = local_BX_486.field_0x4;
                local_BX_493.field_0xa = local_BX_486.field_0x6;
                uVar10 = pass1_1020_c42e(local_BX_486.field_0x4);
                (local_22 + 0xc) = uVar10 * (_local_6 + 6);
                uVar3 = local_BX_4.field_0x22;
                ppcVar4 = (local_BX_4.field_0x22 + 4);
                (**ppcVar4)(0x1020, uVar3, (uVar3 >> 0x10));
                in_DX = extraout_DX_06;
            }
            local_BX_4.field_0x20 = 0;
        }
    }
    if (local_BX_4.field_0xc != 0xe) {
        uVar6 = pass1_1028_b58e((param_1 & 0xffff | uVar13 << 0x10));
        _local_2a = CONCAT31(extraout_var, uVar6) & 0xffff | in_DX << 0x10;
        local_22 = (CONCAT31(extraout_var, uVar6) + 0x2e);
        local_1e = (local_22 + 4);
        pass1_1028_68de(
            CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_138)),
            1,
            local_1e,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_138));
    }
    return;
}

pub fn pass1_1028_081e(param_1: *mut astruct_44) {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u32;
    let uVar5: u8;
    let local_AX__1: *mut astruct_740;
    let mut uVar6: i32;
    let extraout_var: u32;
    let mut in_DX: u16;
    let local_BX_41: *mut astruct_741;
    let mut uVar7: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = pass1_1028_b58e(param_1);
    uVar4 = (CONCAT31(extraout_var, uVar5) + 0x2e);
    iVar2 = (uVar4 + 0x18);
    uVar7 = (param_1 >> 0x10);
    local_BX_41 = param_1;
    piVar1 = &local_BX_41.field_0x20;
    unsafe { *piVar1 = *piVar1 + 1 };
    uVar6 = *_PTR_LOOP_1050_65e2;
    uVar3 = (_PTR_LOOP_1050_65e2 + 2);
    if (iVar2 < 0xfa) {
        uVar6 = uVar6 & 1;
    } else {
        if (0x1c1 < iVar2) {
            if (iVar2 < 0x226) {
                return;
            }
            if ((iVar2 < 0x2ee) && (CONCAT22(uVar3, uVar6) % 3 != 0)) {
                return;
            }
            piVar1 = &local_BX_41.field_0x20;
            unsafe { *piVar1 = *piVar1 + 1 };
            return;
        }
        uVar6 = (CONCAT22(uVar3, uVar6) % 3);
    }
    if (uVar6 != 0) {
        return;
    }
    piVar1 = &local_BX_41.field_0x20;
    unsafe { *piVar1 = *piVar1 + -1 };
    return;
}

pub fn pass1_1028_08c6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_0138(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0954(param_1: *mut astruct_742) -> *mut astruct_742 {
    let local_BX_12: *mut astruct_742;
    let local_ES_12: *mut astruct_742;

    pass1_1028_b354(param_1);
    local_ES_12 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    local_BX_12.field_0x20 = 0;
    param_1 = 0xada;
    local_BX_12.field_0x2 = &PTR_LOOP_1050_1028;
    local_BX_12.field_0xe = 0x4b;
    return param_1;
}

pub fn pass1_1028_0982(param_1: *mut astruct_743, param_2: u32, param_3: u32) {
    let mut local_EAX__1: u32;
    let mut local_register2: u16;
    let mut uVar1: u16;

    local_register2 = (local_EAX__1 >> 0x10);
    uVar1 = param_2;
    pass1_1028_b39e(CONCAT22(uVar1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0x20 = 0;
    CONCAT22(uVar1, param_1) = 0xada;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    param_1.field_0xe = 0x4b;
    return CONCAT22(local_register2, param_1);
}

pub fn pass1_1028_09b8(param_1: *mut astruct_44) {
    let mut local_DXAX_10: u32;

    local_DXAX_10._0_1_ = pass1_1028_b58e(param_1);
    (CONCAT11(local_DXAX_10._1_1_, local_DXAX_10) + 0x14) = 1;
    return;
}

pub fn pass1_1028_09d4(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 2];
    let mut local_4: u16;

    uVar6 = param_1;
    uVar7 = (param_1 >> 0x10);
    uVar8 = (param_3 >> 0x10);
    local_4 = pass1_1028_c314(uVar6, uVar7, param_2, param_3, uVar8, param_4);
    if (local_4 == 0) {
        return;
    }
    pass1_1028_c7b6(param_1, param_2, param_4);
    if ((local_4 != 0) && (local_4 != 4)) {
        if (((local_4 - 5) < 1) || (SBORROW2(local_4 - 5, 1) || (3 < (local_4 - 6)))) {
            if (((uVar6 + 0xc) != 0x3e) && ((uVar6 + 0xc) != 0x41)) {
                return;
            }
            uVar5 = pass1_1030_bcae(local_6, unaff_SS);
            uVar4 = (uVar5 >> 0x10);
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3, uVar8);
            uVar1 = &paVar2.field_0x10;
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
            puVar3 = local_6;
            pass1_1030_bcde(puVar3, unaff_SS, CONCAT22(uVar4, paVar2), param_2, param_4);
            if (puVar3 < 0) {
                PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            if (5 < puVar3) {
                PTR_LOOP_1050_50ca = 0x6b5;
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = 0x6a8;
    return;
}

pub fn pass1_1028_0ab4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0b42(param_1: *mut u16) {
    pass1_1028_b354(param_1);
    unsafe { *param_1 = 0xbbc };
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_0b64(param_1: *mut astruct_744, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xbbc;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_0b96(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0c24(param_1: *mut astruct_745) -> *mut astruct_745 {
    let local_BX_14: *mut astruct_745;
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_14 = param_1;
    local_BX_14.field_0x20 = 0;
    local_BX_14.field_0x22 = 0;
    param_1 = (s_480_bmp_1050_1721 + 3);
    local_BX_14.field_0x2 = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_0c50(param_1: *mut astruct_746, param_2: u32, param_3: u32) {
    let mut in_EAX: u32;
    let mut uVar1: u16;

    uVar1 = param_2;
    pass1_1028_b39e(CONCAT22(uVar1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0x20 = 0;
    param_1.field_0x22 = 0;
    CONCAT22(uVar1, param_1) = (s_480_bmp_1050_1721 + 3);
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return (ZEXT24(param_1) | in_EAX & 0xffff0000);
}

pub fn pass1_1028_0c84(param_1: *mut astruct_44, param_2: *mut u8) {
    let ppcVar1: fn();
    let uVar2: u8;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let paVar5: *mut astruct_493;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut uVar7: u32;
    let puVar8: *mut u32;
    let mut uVar9: u32;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: [u8; 10];
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1028_b58e(param_1);
    iVar3 = CONCAT31(extraout_var, uVar2);
    _local_6 = (CONCAT31(extraout_var, uVar2) & 0xffff | in_DX << 0x10);
    local_c = (iVar3 + 0xc);
    local_12 = (iVar3 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_DX, local_12));
    local_1a = local_c;
    local_16 = local_8;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_e = local_e + 1;
    if (local_e <= local_14) {
        uVar9 = CONCAT22(unaff_SS, local_32);
        local_16 = local_e;
        uVar7 = pass1_1028_bb24(param_1);
        puVar4 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            puVar4,
            unaff_SS,
            uVar7,
            (uVar7 >> 0x10),
            uVar9,
        );
        unsafe { local_28 = *puVar4 };
        uVar6 = (puVar4 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        local_24 = local_28;
        if (local_3a._3_1_ != '\0') {
            paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_28, uVar6);
            puVar8 = pass1_1030_73a8(CONCAT22(uVar6, paVar5));
            unsafe {
                ppcVar1 = (*puVar8 + 0x58);
                (**ppcVar1)(0x1030, puVar8, (puVar8 >> 0x10), param_2);
            }
        }
    }
    pass1_1028_b46e(param_1, param_2);
    pass1_1030_7296(_local_6);
    return;
}

pub fn pass1_1028_0d80(param_1: u32) {
    let mut uVar1: u16;
    let mut uVar2: i32;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x20);
    pass1_1028_1646((param_1 & 0xffff | uVar2 << 0x10));
    return uVar1;
}

pub fn pass1_1028_0d9c(param_1: *mut astruct_44) {
    let ppcVar1: fn();
    let uVar2: u8;
    let local_AX__1: *mut astruct_747;
    let puVar3: *mut u32;
    let paVar4: *mut astruct_493;
    let BVar5: bool;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_32: [u8; 6];
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_b514(param_1);
    uVar2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_DX, local_12));
    local_1a = local_c;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_16 = local_e + 1;
    if (local_16 <= local_14) {
        uVar8 = CONCAT22(unaff_SS, local_32);
        local_e = local_16;
        uVar7 = pass1_1028_bb24(param_1);
        puVar3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            puVar3,
            unaff_SS,
            uVar7,
            (uVar7 >> 0x10),
            uVar8,
        );
        unsafe { local_28 = *puVar3 };
        uVar6 = (puVar3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        if (local_3a._3_1_ != '\0') {
            local_24 = local_28;
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_28, uVar6);
            local_3a = CONCAT22(uVar6, paVar4);
            uVar2 = pass1_1030_6fa0(CONCAT22(uVar6, paVar4));
            BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var_00, uVar2), 0x13);
            if (BVar5 != 0) {
                _local_2c = pass1_1030_73a8(local_3a);
                ppcVar1 = (*_local_2c + 0x24);
                (**ppcVar1)();
            }
        }
    }
    return;
}

pub fn pass1_1028_0ea6(param_1: u32) {
    let piVar1: *mut i32;
    let BVar2: bool;
    let local_BX_6: *mut astruct_748;
    let mut uVar3: i32;
    let mut local_12: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar3 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    if (local_BX_6.field_0xc != 0x10) {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_BX_6.field_0xc, 0x13);
        if (BVar2 == 0) {
            BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_BX_6.field_0xc, 2);
            if (((BVar2 != 0) && (local_BX_6.field_0x12 != 7)) && (local_BX_6.field_0x12 != 4)) {
                BVar2 = pass1_1028_1556(param_1 & 0xffff | uVar3 << 0x10);
                if (BVar2 == 0) {}
                // goto LAB_1028_0f0a;
                if (local_BX_6.field_0x12 == 9) {
                    local_BX_6.field_0x12 = 5;
                }
            }
        } else {
            if (local_BX_6.field_0x22 < 1) {
                if ((local_BX_6.field_0x12 != 5) && (local_BX_6.field_0x12 != 6)) {
                    return;
                }
                error_check_1000_17ce(local_BX_6.field_0x14);
                local_BX_6.field_0x14 = 0;
                // LAB_1028_0f0a:
                local_BX_6.field_0x12 = 9;
                return;
            }
        }
        pass1_1028_be2a(param_1);
        if (local_BX_6.field_0x12 == 5) {
            unsafe {
                piVar1 = &local_BX_6.field_0x22;
                *piVar1 = *piVar1 + -1
            };
            pass1_1028_b58e((param_1 & 0xffff | uVar3 << 0x10));
        }
    }
    return;
}

pub fn pass1_1028_0fa4(param_1: *mut astruct_749) {
    let mut iVar1: i32;
    let BVar2: bool;
    let mut local_AX_105: u16;
    let local_DL_105: u8;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_12: u16;
    let local_10: *mut astruct_749;
    let local_e: *mut astruct_749;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_10 = param_1;
    local_e = (param_1 >> 0x10);
    pass1_1028_be9e(param_1);
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_c, 2));
    uVar3 = (ppVar4 >> 0x10);
    iVar1 = (ppVar4 + 0x82);
    if (local_10.field_0x12 == 5) {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_10.field_0xc, 2);
        if ((BVar2 != 0) && (PTR_LOOP_1050_4fbc == 0x0 || (iVar1 != 0))) {
            PTR_LOOP_1050_4fbc = (&PTR_LOOP_1050_0000 + 1);
            local_AX_105._0_1_ = pass1_1028_b58e(param_1);
            local_AX_105 = CONCAT11(local_AX_105._1_1_, local_AX_105);
            pass1_1030_7c50(CONCAT13((uVar3 >> 8), CONCAT12(uVar3, local_AX_105)), 1, 4);
        }
        local_10.field_0x22 = 100;
    }
    return;
}

pub fn pass1_1028_1024(param_1: *mut astruct_44) -> i32 {
    let uVar1: u8;
    let mut in_AX: u16;
    let extraout_AH: u8;
    let BVar2: bool;
    let paVar3: *mut astruct_493;
    let mut in_DX: u16;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let mut local_2c: u32;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_bab6(param_1);
    local_8 = pass1_1030_2fac(CONCAT22(in_DX, in_AX));
    _local_c = pass1_1028_bb24(param_1);
    local_e = (_local_c >> 0x10);
    uVar1 = pass1_1028_b58e(param_1);
    local_10 = CONCAT11(extraout_AH, uVar1);
    local_16 = (local_10 + 0xc);
    local_1a = 0;
    while (true) {
        if (local_8 < local_1a) {
            return local_1a._2_2_;
        }
        local_12 = local_1a;
        uVar6 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, &local_16), _local_c);
        uVar4 = (uVar6 >> 0x10);
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6, uVar4);
        if ((uVar4 | paVar3) == 0) {
            break;
        }
        uVar7 = pass1_1030_73a8(CONCAT22(uVar4, paVar3));
        uVar5 = (uVar7 >> 0x10);
        if (uVar7 == 0) {
            return local_1a._2_2_;
        }
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar7 + 0xc), 0x13);
        if ((BVar2 != 0) && ((uVar7 + 0x12) == 5)) {
            local_1a = local_1a & 0xffff | (local_1a._2_2_ + 1) << 0x10;
        }
        local_1a = local_1a & 0xffff0000 | (local_1a + 1);
    }
    return local_1a._2_2_;
}

pub fn pass1_1028_1106(param_1: *mut astruct_44) -> i32 {
    let uVar1: u8;
    let mut in_AX: u16;
    let extraout_AH: u8;
    let BVar2: bool;
    let paVar3: *mut astruct_493;
    let mut in_DX: u16;
    let mut uVar4: i32;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut local_2c: u32;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_bab6(param_1);
    local_8 = pass1_1030_2fac(CONCAT22(in_DX, in_AX));
    _local_c = pass1_1028_bb24(param_1);
    local_e = (_local_c >> 0x10);
    uVar1 = pass1_1028_b58e(param_1);
    local_10 = CONCAT11(extraout_AH, uVar1);
    local_16 = (local_10 + 0xc);
    local_1a = 0;
    while (true) {
        if (local_8 < local_1a) {
            return local_1a._2_2_;
        }
        local_12 = local_1a;
        uVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, &local_16), _local_c);
        uVar4 = (uVar5 >> 0x10);
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, uVar4);
        if ((uVar4 | paVar3) == 0) {
            break;
        }
        uVar6 = pass1_1030_73a8(CONCAT22(uVar4, paVar3));
        if (uVar6 == 0) {
            return local_1a._2_2_;
        }
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar6 + 0xc), 0x13);
        if (BVar2 != 0) {
            local_1a = local_1a & 0xffff | (local_1a._2_2_ + 1) << 0x10;
        }
        local_1a = local_1a & 0xffff0000 | (local_1a + 1);
    }
    return local_1a._2_2_;
}

pub fn pass1_1028_11de(param_1: *mut astruct_44) -> bool {
    let mut local_DXAX_12: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_DXAX_12._0_1_ = pass1_1028_b58e(param_1);
    return (CONCAT11(local_DXAX_12._1_1_, local_DXAX_12) + 0x10) == 0;
}

pub fn pass1_1028_121e(param_1: *mut astruct_44) -> *mut astruct_44 {
    let uVar1: u8;
    let extraout_AH: u8;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let mut unaff_SS: u16;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let paVar6: *mut astruct_44;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = pass1_1028_11de(param_1);
    local_4 = (uVar4 >> 0x10);
    if (uVar4 != 0) {
        return param_1;
    }
    uVar1 = pass1_1028_b58e(param_1);
    local_6 = CONCAT11(extraout_AH, uVar1);
    local_c = (local_6 + 0xc);
    local_8 = 0;
    uVar5 = pass1_1028_bb24(param_1);
    uVar4 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, &local_c), uVar5);
    uVar3 = (uVar4 >> 0x10);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, uVar3);
    if ((uVar3 | paVar2) == 0) {
        return 0x0;
    }
    paVar6 = pass1_1030_73a8(CONCAT22(uVar3, paVar2));
    return paVar6;
}

pub fn pass1_1028_12be(param_1: u32, param_2: u32) -> bool {
    let piVar1: *mut i32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let uVar4: u8;
    let BVar5: bool;
    let extraout_AH: u8;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar9 = pass1_1028_11de(param_1);
    iVar7 = (uVar9 >> 0x10);
    if (uVar9 == 0) {
        _local_c = pass1_1028_121e(param_1);
        ppcVar3 = (*_local_c + 0x40);
        BVar5 = (**ppcVar3)();
        return BVar5;
    }
    param_2 = 0;
    uVar4 = pass1_1028_b58e(param_1);
    local_8 = 4;
    uVar6 = CONCAT11(extraout_AH, uVar4);
    iVar8 = iVar7;
    while {
        pass1_1030_7c28(CONCAT22(iVar7, CONCAT11(extraout_AH, uVar4)), local_8);
        uVar2 = param_2;
        param_2 = param_2 + uVar6;
        piVar1 = (param_2 + 2);
        unsafe { *piVar1 = *piVar1 + iVar8 + CARRY2(uVar2, uVar6) };
        local_8 = local_8 + 1;
        local_8 < 0xe
    } {}
    if (500 < param_2) {
        return 1;
    }
    return 0;
}

pub fn pass1_1028_134a(param_1: *mut u32) {
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let uVar3: u8;
    let BVar4: bool;
    let puVar5: *mut u32;
    let mut uVar6: u32;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (iVar7 + 0xc), 0x13);
    if (BVar4 != 0) {
        puVar5 = &local_6;
        unsafe { ppcVar2 = (*param_1 + 0x40) };
        ppcVar2(&PTR_LOOP_1050_1008, iVar7, uVar8, puVar5);
        if (puVar5 != 0x0) {
            piVar1 = (iVar7 + 0x22);
            unsafe { *piVar1 = *piVar1 + 1 };
            return;
        }
        local_a = 500 - local_6;
        uVar6 = local_a;
        pass1_1028_121e(param_1, uVar8);
        uVar3 = pass1_1028_b58e((uVar6 & 0xffff | in_DX << 0x10));
        local_16 = 0;
        while (local_16 < 10) {
            local_a._0_2_ = (local_16 * 2 + 0x4fbe);
            local_a._2_2_ = local_a >> 0xf;
            if (local_a < local_a) {}
            _local_1a = CONCAT22(local_a._2_2_, local_a);
            pass1_1030_7ddc(
                CONCAT31(extraout_var, uVar3) & 0xffff | in_DX << 0x10,
                CONCAT13((local_a._2_2_ >> 8), CONCAT12(local_a._2_2_, local_a)),
                local_16 + 4,
            );
            local_a = local_a - _local_1a;
            if (local_a < 1) {
                return;
            }
            local_16 = local_16 + 1;
        }
    }
    return;
}

pub fn pass1_1028_1416(param_1: u32) -> i32 {
    let mut iVar1: i32;
    let mut uVar2: u32;

    iVar1 = pass1_1028_11de(param_1);
    if (iVar1 == 0) {
        uVar2 = pass1_1028_121e(param_1);
        iVar1 = pass1_1028_1416(uVar2);
        return iVar1;
    }
    iVar1 = pass1_1028_1024(param_1);
    return iVar1 * 0xf;
}

pub fn pass1_1020_e9d4(param_1: u16, param_2: u16, param_3: u16) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_df0c(param_1, param_2, param_3);
    uVar2 = extraout_DX;
    uVar1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    pass1_1038_57dc(
        *(CONCAT31(extraout_var, uVar1) + 0x2e),
        0x1,
        (&PTR_LOOP_1050_0000 + 1),
    );
    return;
}

pub fn pass1_1020_ea0e(param_1: u32) {
    pass1_1028_bdac(param_1, 1);
    return;
}

pub fn pass1_1020_ea20(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let ppcVar1: fn();
    let mut uVar2: u16;
    let mut cVar3: u8;
    let mut in_AX: i32;
    let puVar4: *mut u32;
    let paVar5: *mut astruct_493;
    let mut uVar6: u16;
    let mut extraout_DX: u16;
    let mut uVar7: i32;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let mut uVar9: u16;
    let mut uVar10: i32;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_22: u32;
    let mut local_1c: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar9 = (param_1 >> 0x10);
    uVar2 = param_1;
    pass1_1028_c3aa(uVar2, uVar9, param_2, param_3, param_4);
    if (in_AX == 0) {
        return;
    }
    pass1_1028_c23e(uVar2, uVar9, param_2, param_3, param_4);
    if (in_AX == 0) {
        return;
    }
    local_4 = pass1_1028_c314(uVar2, uVar9, param_2, param_3, (param_3 >> 0x10), param_4);
    if (local_4 == 0) {
        return;
    }
    uVar8 = (param_2 >> 0x10);
    pass1_1028_c7b6(param_1, uVar9, param_2, uVar8, param_4);
    if ((((local_4 == 5) || (local_4 == 4)) || (local_4 == 6)) || (local_4 == 9)) {
        PTR_LOOP_1050_50ca = 0x6a8;
        return;
    }
    if (local_4 != 0) {
        return;
    }
    puVar4 = &local_12;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        (_PTR_LOOP_1050_5740 >> 0x10),
        param_2,
        uVar8,
        param_4,
        (param_4 >> 0x10),
        puVar4,
        unaff_SS,
    );
    unsafe { local_26 = *puVar4 };
    local_38 = (puVar4 + 2);
    local_26._3_1_ = (local_26 >> 0x18);
    local_3a = local_26._3_1_;
    local_1c = local_26;
    local_8 = local_26;
    if (local_26._3_1_ == 0) {}
    // goto LAB_1020_eb4e;
    local_8._0_2_ = local_26;
    paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_8, local_38);
    local_26 = CONCAT22(local_38, paVar5);
    local_22 = &paVar5[1].field_0x10;
    if ((local_22 + 4) != param_3) {
        PTR_LOOP_1050_50ca = 0x6b7;
        return;
    }
    local_1c = pass1_1030_73a8(CONCAT22(local_38, paVar5));
    local_38 = (local_1c >> 0x10);
    uVar7 = (local_1c + 0xc);
    local_3a = uVar7;
    if (uVar7 != 0x41) {
        if (0x41 < uVar7) {
            if (uVar7 == 0x6b) {
                PTR_LOOP_1050_50ca = 0x6b1;
                return;
            }
            if (uVar7 < 0x6c) {
                if (uVar7 == 0x42) {
                    PTR_LOOP_1050_50ca = 0x6b1;
                    return;
                }
                local_3a = uVar7 - 0x4b;
                if (local_3a == 0) {
                    PTR_LOOP_1050_50ca = 0x6b1;
                    return;
                }
            } else {
                if (uVar7 == 0x6e) {
                    return;
                }
                local_3a = uVar7 - 0x73;
                if ((4 < (uVar7 - 0x6e))
                    && (local_3a = uVar7 - 0x79, local_3a == 0 || (uVar7 - 0x73) < 6))
                {
                    PTR_LOOP_1050_50ca = 0x6b0;
                    return;
                }
            }
            // goto LAB_1020_eb4e;
        }
        if (uVar7 != 0x3e) {
            if (uVar7 < 0x3f) {
                cVar3 = uVar7;
                if (cVar3 != 0xb) {
                    if (cVar3 == 0x10) {
                        return;
                    }
                    local_3a = uVar7 & 0xff00 | (cVar3 - 0x37);
                    if ((cVar3 - 0x37) != 0) {}
                    // goto LAB_1020_eb4e;
                }
                PTR_LOOP_1050_50ca = 0x6b4;
                return;
            }
            // goto LAB_1020_eb4e;
        }
    }
    if ((local_1c + 0x12) == 4) {
        PTR_LOOP_1050_50ca = 0x6b1;
        return;
    }
    // LAB_1020_eb4e:
    uVar8 = 0x1000;
    process_struct_1000_179c(0xb4, local_38);
    uVar7 = local_38 | local_3a;
    if (uVar7 == 0) {
        local_e = 0;
        uVar7 = 0;
    } else {
        uVar8 = SUB42(&PTR_LOOP_1050_1040, 0);
        local_e = local_3a;
        mixed_1040_8520(
            CONCAT13((local_38 >> 8), CONCAT12(local_38, local_3a)),
            g_h_window,
            0x24,
            2,
            0x57b,
            0x5e8,
        );
    }
    _local_c = CONCAT22(uVar7, local_e);
    ppcVar1 = (*_local_c + 0x74);
    uVar10 = local_e;
    (**ppcVar1)(uVar8, local_e, uVar7);
    if (local_e != 7) {
        local_2e = local_8;
        local_22 = local_8;
        local_22._3_1_ = (local_8 >> 0x18);
        if (local_22._3_1_ != '\0') {
            paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_8, local_8._2_2_);
            _local_32 = CONCAT22(local_8._2_2_, paVar5);
            pass1_1030_7296(CONCAT22(local_8._2_2_, paVar5));
            pass1_1030_730a(_local_32);
            local_2e = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar10, 0x2f));
            pass1_1010_ec68(local_2e, _local_32);
            _local_2a = pass1_1030_73a8(_local_32);
            ppcVar1 = (*_local_2a + 0x24);
            (**ppcVar1)(0x1030, _local_2a, (_local_2a >> 0x10));
            uVar6 = pass1_1028_bc4a(_local_2a);
            (uVar2 + 0x24) = uVar6;
            pass1_1030_e4fa(CONCAT22(unaff_SS, &local_146), (_local_32 + 0x16));
            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_146));
        }
        return;
    }
    PTR_LOOP_1050_50ca = (&PTR_LOOP_1050_0000 + 1);
    return;
}

pub fn pass1_1020_ecb0(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut in_DX: u16;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = (iVar3 + 8);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    if ((iVar3 + 0x12) == 1) {
        match (&paVar2.field_0x14) {
            2 | 7 => {
                local_8 = 2;
            }
            3 | 8 => {
                local_8 = 3;
            }
            // default:
            _ => {
                local_8 = &paVar2.field_0x14;
            }
            5 | 6 => {
                local_8 = 1;
            }
        }
        (iVar3 + 0x14) = local_8;
        return;
    }
    pass1_1028_bf22((param_1 & 0xffff | uVar4 << 0x10));
    return;
}

pub fn pass1_1020_ed3c(param_1: *mut astruct_44) {
    let piVar1: *mut i32;
    let uVar2: u8;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_DX: u16;
    let mut extraout_DX: i32;
    let mut uVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_138: u16;
    let mut local_136: u16;
    let mut local_26: u32;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 2];
    let mut local_e: [u8; 2];
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    piVar1 = (iVar4 + 0x14);
    unsafe { *piVar1 = *piVar1 + -1 };
    let pi_var1_val = unsafe { *piVar1 };
    if (pi_var1_val == 0) {
        (iVar4 + 0x12) = 0;
        uVar2 = pass1_1028_b58e(param_1);
        local_6 = CONCAT31(extraout_var, uVar2);
        local_c = (local_6 + 0xc);
        uStack8 = (local_6 + 0x10);
        local_1e = &local_c;
        local_4 = in_DX;
        pass1_1008_3eb4(
            CONCAT22(unaff_SS, &local_c),
            CONCAT22(unaff_SS, &local_12),
            CONCAT22(unaff_SS, local_10),
            CONCAT22(unaff_SS, local_e),
        );
        if (local_12 < 1) {
            local_1e = 5;
        } else {
            local_1e = 6;
        }
        (local_6 + 0x14) = local_1e;
        if (local_12 < 1) {
            local_14 = 5;
        } else {
            local_14 = 9;
        }
        pass1_1020_ee3a(param_1, local_14);
        uVar3 = extraout_DX;
        uVar2 = pass1_1028_b58e(param_1);
        _local_18 = CONCAT31(extraout_var_00, uVar2) & 0xffff | uVar3 << 0x10;
        local_1c = *(CONCAT31(extraout_var_00, uVar2) + 0x2e);
        pass1_1038_5804(local_1c, 1, (&PTR_LOOP_1050_0000 + 1));
        if (0 < (iVar4 + 0x24)) {
            local_26 = (local_1c + 4);
            pass1_1028_68de(CONCAT22(unaff_SS, &local_138), (iVar4 + 0x24), local_26);
            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_138));
        }
    }
    return;
}

pub fn pass1_1020_ee3a(param_1: u32, param_2: u16) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut unaff_SS: u16;
    let mut local_13e: u16;
    let mut local_13c: u16;
    let mut local_13a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar1);
    local_c = (local_6 + 0xc);
    uStack8 = (local_6 + 0x10);
    local_4 = in_DX;
    _local_10 = pass1_1028_bb24(param_1);
    local_14 = (local_6 + 0x2e);
    local_18 = (local_14 + 4);
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_13c),
        0,
        1,
        param_2,
        &local_c,
        unaff_SS,
        local_18,
        _local_10,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_13c));
    return;
}

pub fn pass1_1020_eed0(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_ef5e(param_1: *mut astruct_44) {
    param_1.ptr_a_lo = 0;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1020_ef94(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_ef5e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0068(param_1: *mut astruct_727) {
    let mut uVar1: i32;
    let paVar2: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut uVar3: i32;
    let local_BX_13: *mut astruct_727;
    let local_ES_13: *mut astruct_727;
    let mut local_4: u16;

    pass1_1028_b354(param_1);
    local_ES_13 = (param_1 >> 0x10);
    local_BX_13 = param_1;
    uVar1 = 0;
    local_BX_13.field_0x20 = 0;
    &local_BX_13.field_0x22 = 0;
    param_1 = 0x8ec;
    local_BX_13.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    uVar3 = paVar2 | uVar1;
    if (uVar3 == 0) {
        &local_BX_13.field_0x22 = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, uVar1));
        local_BX_13.field_0x22 = paVar2;
        local_BX_13.field_0x24 = uVar3;
    }
    return;
}

pub fn pass1_1028_00cc(param_1: *mut astruct_728, param_2: u32, param_3: u32) {
    let mut uVar1: i32;
    let paVar2: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut local_4: u16;

    uVar4 = param_2;
    pass1_1028_b39e(CONCAT22(uVar4, param_1), (param_2 >> 0x10), param_3);
    uVar1 = 0;
    param_1.field_0x20 = 0;
    &param_1.field_0x22 = 0;
    CONCAT22(uVar4, param_1) = 0x8ec;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    uVar3 = paVar2 | uVar1;
    if (uVar3 == 0) {
        &param_1.field_0x22 = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, uVar1));
        param_1.field_0x22 = paVar2;
        param_1.field_0x24 = uVar3;
    }
    return;
}

pub fn pass1_1028_0138(param_1: *mut astruct_44) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.ptr_a_lo = 0x8ec;
    (iVar4 + 2) = &PTR_LOOP_1050_1028;
    puVar1 = (iVar4 + 0x22);
    uVar2 = (iVar4 + 0x24);
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)()
        };
    }
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_0176(param_1: *mut astruct_44, param_2: *mut u8) {
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let uVar3: u8;
    let paVar4: *mut astruct_199;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let paVar7: *mut astruct_199;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut local_c: u16;

    iVar8 = param_1;
    uVar9 = (param_1 >> 0x10);
    pass1_1028_b46e(param_1, param_2);
    puVar1 = (iVar8 + 0x22);
    uVar6 = (iVar8 + 0x24);
    uVar5 = uVar6 | puVar1;
    paVar7 = CONCAT22(uVar5, puVar1);
    if (uVar5 != 0) {
        unsafe {
            ppcVar2 = *puVar1;
            paVar7 = ppcVar2()
        };
    }
    process_struct_1000_179c(0xc, (paVar7 >> 0x10));
    uVar6 = (paVar7 >> 0x10) | paVar7;
    if (paVar7 == 0x0) {
        paVar4 = 0x0;
        uVar6 = 0;
    } else {
        paVar4 = process_struct_1008_574a(paVar7);
    }
    (iVar8 + 0x22) = paVar4;
    (iVar8 + 0x24) = uVar6;
    uVar9 = 0x14;
    uVar3 = pass1_1028_b58e(param_1);
    pass1_1030_7f1a(uVar3, uVar6, uVar9);
    return;
}

pub fn pass1_1028_01ec(param_1: *mut astruct_729) {
    let mut u_var1: u32;
    let local_BX_4: *mut astruct_729;
    let local_BX_21: *mut astruct_730;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if ((local_BX_4.field_0x12 == 6) || (local_BX_4.field_0x12 == 5)) {
        uVar1 = local_BX_4.field_0x14;
        uVar2 = (uVar1 >> 0x10);
        local_BX_21 = uVar1;
        if ((local_BX_21.field_0xa6 == 0x14) || (local_BX_21.field_0xa8 == 0x10)) {
            pass1_1028_bdac(param_1, 6);
            return;
        }
        pass1_1028_be2a(param_1);
    }
    return;
}

pub fn pass1_1020_e76c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_e7fa(param_1: *mut u16) {
    pass1_1028_b354(param_1);
    unsafe {
        *param_1 = 0xe88e;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_e81c(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xe88e;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_e846(param_1: *mut astruct_44) {
    param_1.ptr_a_lo = 0xe88e;
    (param_1 + 2) = 0x1020;
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1020_e868(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_e846(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_e8f6(param_1: *mut u16) {
    let mut uVar1: u16;

    pass1_1030_dc96(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x24) = 0;
    unsafe {
        *param_1 = 0xeef6;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_e91e(param_1: *mut astruct_1012, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1030_dcc2(param_1, param_2, param_3, param_3_00);
    (param_1 + 1) = 0;
    CONCAT22(param_2, param_1) = 0xeef6;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d4ca(param_1: *mut astruct_44) {
    let uVar1: u8;
    let mut uVar2: u16;
    let extraout_var: u32;
    let puVar3: *mut u8;
    let mut in_DX: u16;
    let mut extraout_DX: i32;
    let mut iVar4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x20) == 2) {
        return;
    }
    uVar1 = pass1_1028_b58e(param_1);
    puVar3 = *(CONCAT31(extraout_var, uVar1) + 0x2e);
    iVar4 = 99;
    pass1_1038_3fb0(puVar3);
    uVar2 = pass1_1030_25b2(puVar3 & 0xffff | extraout_DX << 0x10, iVar4);
    if (uVar2 != 0) {
        return;
    }
    return;
}

pub fn pass1_1020_d518(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d5a6(param_1: *mut u8) {
    pass1_1028_b354(param_1);
    param_1 = 0xd7fe;
    (param_1 + 2) = 0x1020;
    return param_1;
}

pub fn pass1_1020_d5c8(param_1: i32, param_2: u16, param_3: u16, param_3_00: *mut u8) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xd7fe;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d5f2(param_1: *mut u8, param_2: *mut u8) {
    let ppcVar1: fn();
    let uVar2: u8;
    let puVar3: *mut u32;
    let paVar4: *mut astruct_493;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let mut uVar8: u32;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: [u8; 10];
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_DX, local_12));
    local_1a = local_c;
    local_16 = local_8;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_e = local_e + 1;
    if (local_e < local_14) {
        uVar8 = CONCAT22(unaff_SS, local_32);
        local_16 = local_e;
        uVar6 = pass1_1028_bb24(param_1);
        puVar3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            puVar3,
            unaff_SS,
            uVar6,
            (uVar6 >> 0x10),
            uVar8,
        );
        unsafe { local_28 = *puVar3 };
        uVar5 = (puVar3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        local_24 = local_28;
        if (local_3a._3_1_ != '\0') {
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_28, uVar5);
            puVar7 = pass1_1030_73a8(CONCAT22(uVar5, paVar4));
            unsafe { ppcVar1 = (*puVar7 + 0x58) };
            (**ppcVar1)(0x1030, puVar7, (puVar7 >> 0x10), param_2);
        }
    }
    pass1_1028_b46e(param_1, param_2);
    return;
}

pub fn pass1_1020_d6e6(param_1: *mut astruct_44) {
    let ppcVar1: fn();
    let uVar2: u8;
    let puVar3: *mut u32;
    let paVar4: *mut astruct_493;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let mut uVar8: u32;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: [u8; 10];
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_b514(param_1);
    uVar2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, uVar2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_DX, local_12));
    local_1a = local_c;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_16 = local_e + 1;
    if (local_16 < local_14) {
        uVar8 = CONCAT22(unaff_SS, local_32);
        local_e = local_16;
        uVar6 = pass1_1028_bb24(param_1);
        puVar3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            puVar3,
            unaff_SS,
            uVar6,
            (uVar6 >> 0x10),
            uVar8,
        );
        unsafe { local_28 = *puVar3 };
        uVar5 = (puVar3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        if (local_3a._3_1_ != '\0') {
            local_24 = local_28;
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_28, uVar5);
            puVar7 = pass1_1030_73a8(CONCAT22(uVar5, paVar4));
            if ((puVar7 + 0xc) == 0x6a) {
                unsafe {
                    ppcVar1 = (*puVar7 + 0x24);
                    (**ppcVar1)()
                };
            }
        }
    }
    return;
}

pub fn pass1_1020_d7d8(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d866(param_1: *mut u16) {
    pass1_1028_b354(param_1);
    unsafe {
        *param_1 = 0xd8ec;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_d888(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) -> i32 {
    let mut in_stack_0000000c: u16;

    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, _param_3_00);
    *CONCAT22(param_2, param_1) = 0xd8ec;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d8c6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d954(param_1: *mut astruct_723) {
    let mut unaff_BP: u16;
    let ppVar1: *mut pass1_struct_1;
    let local_6: *mut astruct_723;
    let local_4: *mut astruct_723;

    local_6 = param_1;
    local_4 = (param_1 >> 0x10);
    pass1_1030_dc96(param_1);
    local_6.field_0x24 = 0;
    local_6.field_0x26 = 0;
    &local_6.field_0x28 = 0;
    param_1.field_0x0 = 0xe792;
    local_6.field_0x2 = 0x1020;
    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x2f));
    local_6.field_0x28 = ppVar1;
    local_6.field_0x2a = (ppVar1 >> 0x10);
    return;
}

pub fn pass1_1020_d99e(
    param_1: *mut astruct_724,
    param_2: u16,
    param_3: u16,
    param_4: u32,
) -> *mut astruct_724 {
    let local_BX_22: *mut astruct_724;
    let mut unaff_BP: u16;
    let ppVar1: *mut pass1_struct_1;
    let paVar2: *mut astruct_1012;
    let mut uvar3: u16;

    paVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    pass1_1030_dcc2(paVar2, uVar3, param_3, param_4);
    (paVar2 + 1) = param_2;
    paVar2[1].field_0x2 = 0;
    &paVar2[1].field_0x4 = 0;
    param_1 = 0xe792;
    paVar2.field_0x2 = 0x1020;
    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x2f));
    &paVar2[1].field_0x4 = ppVar1;
    &paVar2[1].field_0x6 = (ppVar1 >> 0x10);
    &paVar2.field_0x10 = 0x49;
    return param_1;
}

pub fn pass1_1020_d9fa(param_1: *mut u8, param_2: u16) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0xc) != 0x79) {
        pass1_1030_df0c(param_1, param_2);
        uVar2 = extraout_DX;
        uVar1 = pass1_1028_b58e(param_1);
        pass1_1038_57dc(
            *(CONCAT31(extraout_var, uVar1) + 0x2e),
            0x1,
            &dos_alloc_addr_1050_0002,
        );
    }
    return;
}

pub fn pass1_1020_da3c(param_1: u32) {
    pass1_1028_bdac(param_1, 2);
    return;
}

pub fn pass1_1020_da4e(param_1: *mut u32, param_2: u32, param_3: u32, param_4: u32) {
    let ppcVar1: fn();
    let puVar2: *mut u32;
    let paVar3: *mut astruct_493;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut unaff_SS: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut uVar9: u32;
    let mut uVar10: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    puVar2 = &local_e;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_4, puVar2, unaff_SS);
    unsafe { local_6 = *puVar2 };
    uVar4 = (puVar2 + 2);
    local_22._3_1_ = (local_6 >> 0x18);
    uVar6 = local_22._3_1_;
    if (local_22._3_1_ != 0) {
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, uVar4);
        uVar7 = pass1_1030_73a8(CONCAT22(uVar4, paVar3));
        uVar6 = uVar7;
        if ((uVar6 + 0xc) == 0x10) {
            PTR_LOOP_1050_50ca = 0x6a9;
            return;
        }
    }
    pass1_1028_c7b6(param_1, param_2, param_4);
    uVar5 = (param_1 >> 0x10);
    uVar8 = param_1 & 0xffff | uVar5 << 0x10;
    unsafe { ppcVar1 = (*param_1 + 0x60) };
    uVar7 = param_2;
    uVar9 = param_3;
    uVar10 = param_4;
    local_8 = uVar6;
    (**ppcVar1)();
    if (((uVar6 != 0)
        && (
            pass1_1028_c23e(param_1, uVar5, param_2, param_3, param_4),
            uVar6 != 0,
        ))
        && (
            uVar4 = pass1_1028_c314(param_1, uVar5, param_2, param_3, (param_3 >> 0x10), param_4),
            uVar4 != 0,
        ))
    {
        uVar6 = (param_2 >> 0x10);
        if ((((param_2 + 4) == 0) && (local_8 != 4))
            && (
                unsafe { ppcVar1 = (*param_1 + 0x5c) },
                (**ppcVar1)(
                    &PTR_LOOP_1050_1028,
                    param_1,
                    param_2,
                    param_3,
                    param_4,
                    uVar8,
                    uVar7,
                    uVar9,
                    uVar10,
                ),
                uVar4 == 0,
            ))
        {
            return;
        }
        local_a = (param_2 + 4);
        if (local_a != 0) {
            win_fn_1020_df10(param_1, param_2 & 0xffff | uVar6 << 0x10, param_4);
            return;
        }
        pass1_1020_deac(param_1, param_2 & 0xffff | uVar6 << 0x10, param_4);
        return;
    }
    return;
}

pub fn pass1_1020_db86(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    uVar5 = pass1_1030_bcae(local_4, unaff_SS);
    uVar4 = (uVar5 >> 0x10);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    uVar1 = &paVar2.field_0x10;
    uVar5 = param_1_00;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    puVar3 = local_4;
    pass1_1030_bcde(puVar3, unaff_SS, CONCAT22(uVar4, paVar2), uVar5, param_5);
    if (puVar3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if ((puVar3 < 0x1f) || ((param_1_00 + 4) < 1)) {
            return;
        }
        PTR_LOOP_1050_50ca = 0x6b6;
        PTR_LOOP_1050_50cc = puVar3 + -0x1e;
    }
    return;
}

pub fn pass1_1020_dc1c(param_1: u32, param_2: u32) {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let puVar3: *mut u32;
    let paVar4: *mut astruct_493;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let mut uVar8: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u32;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

    uVar8 = CONCAT22(unaff_SS, local_a);
    uVar6 = pass1_1028_bb24(param_1);
    puVar3 = uVar6;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, puVar3, (uVar6 >> 0x10), uVar8);
    unsafe { local_6 = *puVar3 };
    uVar5 = (puVar3 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ != '\0') {
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, uVar5);
        puVar7 = pass1_1030_73a8(CONCAT22(uVar5, paVar4));
        iVar1 = (puVar7 + 0xc);
        if (((iVar1 < 1) || (SBORROW2(iVar1, 1)))
            || (iVar1 != 9 && 7 < iVar1 + -1 && (iVar1 + -9 < 0x6a || (6 < iVar1 + -0x73))))
        {
            unsafe {
                ppcVar2 = (*puVar7 + 0x24);
                ppcVar2()
            };
        }
    }
    return;
}

pub fn pass1_1020_dca8(param_1: *mut astruct_44) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut extraout_DX: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_SS__1: u16;
    let mut local_DXAX_29: u32;
    let mut local_2e: [u8; 14];
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 2];
    let mut local_4: u16;

    pass1_1028_c1f8(
        param_1,
        (param_1 >> 0x10),
        local_6,
        local_SS__1,
        &local_4,
        local_SS__1,
    );
    local_DXAX_29._2_2_ = extraout_DX;
    local_DXAX_29._0_1_ = pass1_1028_b58e(param_1);
    uVar2 = (CONCAT11(local_DXAX_29._1_1_, local_DXAX_29) + 0xc);
    uVar1 = (CONCAT11(local_DXAX_29._1_1_, local_DXAX_29) + 0x10);
    local_10 = uVar2;
    uVar3 = (uVar2 >> 0x10);
    uVar4 = local_10 - 1;
    if (uVar4 < 0) {
        uVar4 = 0;
    }
    uVar7 = local_4 - 1;
    uVar5 = local_10 + 1;
    if (uVar7 < (local_10 + 1)) {
        uVar5 = uVar7;
    }
    uVar6 = uVar3 - 1;
    if (uVar6 < 0) {
        uVar6 = 0;
    }
    uVar8 = uVar3 + 1;
    if (uVar7 < (uVar3 + 1)) {
        uVar8 = uVar7;
    }
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar6, uVar4);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar6, local_10);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar6, uVar5);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar3, uVar4);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar3, uVar5);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar8, uVar4);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar8, local_10);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    pass1_1008_3e54(CONCAT22(local_SS__1, local_2e), uVar1, uVar8, uVar5);
    pass1_1020_dc1c(param_1, CONCAT22(local_SS__1, local_2e));
    return;
}

pub fn win_fn_1020_de32(param_1: u32, param_2: u16) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000ffee: u32;
    let in_string_1: *mut char;
    let mut local_6: u16;
    let mut local_4: u16;

    in_string_1 = CONCAT22((in_stack_0000ffee >> 0x10), 5);
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, in_string_1);
    uVar2 = (ppVar4 >> 0x10);
    (ppVar4 + 0x12) = param_2;
    uVar3 = (in_string_1 >> 0x10);
    iVar1 = gui_window_func_1038_b72e(_g_astruct_112_a, (_g_astruct_112_a >> 0x10), 4);
    if (iVar1 == 0) {
        pass1_1038_af40(_g_astruct_112_a, *(_PTR_LOOP_1050_4230 + 0x16), 4);
    }
    PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 1);
    process_win_msg_1008_9510(&PTR_LOOP_1050_5b80, &g_alloc_addr_1050_1050, uVar3);
    uVar3 = (param_1 >> 0x10);
    (param_1 + 0x24) = (ppVar4 + 10);
    if ((param_1 + 0x24) == 0) {
        PTR_LOOP_1050_50ca = 0x6b2;
    }
    return;
}

pub fn pass1_1020_deac(param_1: u32, param_2: u32, param_3: u32) -> bool {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar1 = pass1_1028_c7b6(param_1, param_2, param_3);
    if (iVar1 < 1) {
        return 0;
    }
    if (SBORROW2(iVar1, 1)) {
        return 0;
    }
    uVar2 = (param_1 >> 0x10);
    if (iVar1 != 3 && 0 < iVar1 + -2) {
        if (iVar1 == 4) {
            win_fn_1020_de32(param_1, 4);
            if ((param_1 + 0x24) == 6) {
                return 1;
            }
            return 0;
        }
        if (iVar1 != 5) {
            return 0;
        }
    }
    (param_1 + 0x24) = 1;
    return 1;
}

pub fn win_fn_1020_df10(param_1: u32, param_2: u32, param_3: u32) {
    let mut in_AX: u16;
    let puVar1: *mut u32;
    let paVar2: *mut astruct_493;
    let mut uvar3: u16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1028_c7b6(param_1, param_2, param_3);
    uVar4 = (param_1 >> 0x10);
    if (in_AX == 0) {
        puVar1 = &local_e;
        pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_3, puVar1, unaff_SS);
        unsafe { local_a = *puVar1 };
        uVar3 = (puVar1 + 2);
        local_22._3_1_ = (local_a >> 0x18);
        if (local_22._3_1_ != '\0') {
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_a, uVar3);
            uVar5 = pass1_1030_73a8(CONCAT22(uVar3, paVar2));
            if ((uVar5 + 0xc) == 0x6a) {
                uVar3 = pass1_1020_e044(param_1);
                if (uVar3 == 0) {
                    (param_1 + 0x24) = 1;
                } else {
                    PTR_LOOP_1050_50ca = 0x6ac;
                }
            }
        }
    } else {
        if (((5 < in_AX) && (!SBORROW2(in_AX, 6))) && ((in_AX - 6) < 4)) {
            win_fn_1020_de32(param_1, in_AX);
            match (param_1 + 0x24) {
                1 => {
                    uVar3 = pass1_1020_e044(param_1);
                    if (uVar3 != 0) {
                        PTR_LOOP_1050_50ca = 0x6ac;
                    }
                }
                2 | 3 | 4 | 5 => {
                    pass1_1020_e652(param_1, param_2, (param_2 >> 0x10), param_3);
                }
            }
        }
    }
    return;
}

pub fn pass1_1020_e044(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_e: u16;
    let mut local_c: u16;

    uVar4 = (param_1 >> 0x10);
    uVar5 = pass1_1018_04b8((param_1 + 0x28));
    uVar3 = (uVar5 >> 0x10);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, uVar3);
    uVar3 = pass1_1030_2fac(CONCAT22(uVar3, paVar2));
    uVar1 = (param_1 + 0x28);
    if (uVar3 <= (uVar1 + 0x1e)) {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_e08e(param_1: *mut astruct_44) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let uVar3: u8;
    let mut iVar4: i32;
    let paVar5: *mut astruct_493;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut local_DX_47: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut local_DX_314: u16;
    let local_BX_6: *mut astruct_725;
    let mut local_ES_6: u16;
    let mut local_SS__1: u16;
    let local_16c: u8;
    let uStack363: u8;
    let mut local_16a: u16;
    let puVar6: *mut u16;
    let mut local_166: u16;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_32: u32;
    let mut local_2a: u16;
    let mut local_22: u16;
    let mut local_20: [u8; 2];
    let mut local_1e: [u8; 2];
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5ffa9150db: u32;
    let mut temp_5f0d0368a6: u32;

    local_ES_6 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    iVar1 = local_BX_6.field_0xc;
    if (iVar1 == 0x74) {
        iVar1 = local_BX_6.field_0x24;
        if (iVar1 == 1) {}
        // goto LAB_1020_e0ae;
        if (iVar1 != 6) {}
        // goto LAB_1020_e0b9;
        local_166 = 1;
    } else {
        if (iVar1 == 0x78) {
            iVar1 = local_BX_6.field_0x24;
            iVar4 = iVar1 + -1;
            if (iVar4 != 0) {
                if ((0 < iVar4) && (!SBORROW2(iVar4, 1))) {
                    if (iVar1 == 5 || iVar1 + -2 < 3) {
                        pass1_1020_e49a(local_BX_6, local_ES_6);
                        local_DX_47 = extraout_DX_01;
                    } else {
                        if (iVar1 == 6) {
                            pass1_1020_e39c(param_1, 6);
                            pass1_1020_dca8(local_BX_6, local_ES_6);
                            local_DX_47 = extraout_DX_00;
                        }
                    }
                }
                // goto LAB_1020_e0b9;
            }
            local_166 = 0x6a;
        } else {
            if (iVar1 == 0x79) {
                pass1_1020_e49a(local_BX_6, local_ES_6);
                return;
            }
            // LAB_1020_e0ae:
            local_166 = 0x47;
        }
    }
    pass1_1020_e39c(param_1, local_166);
    local_DX_47 = extraout_DX;
    // LAB_1020_e0b9:
    uVar3 = pass1_1028_b58e(param_1);
    iVar1 = CONCAT31(extraout_var, uVar3);
    _local_6 = CONCAT31(extraout_var, uVar3) & 0xffff | local_DX_47 << 0x10;
    local_a = (iVar1 + 0x2e);
    uVar2 = (iVar1 + 0x30);
    if (local_BX_6.field_0xc != 0x79) {
        local_16c = uVar2;
        uStack363 = (uVar2 >> 8);
        pass1_1038_5804(
            CONCAT13(uStack363, CONCAT12(local_16c, local_a)),
            1,
            &dos_alloc_addr_1050_0002,
        );
    }
    if (local_BX_6.field_0x24 == 6) {
        local_16a = (local_a >> 0x10);
        pass1_1038_43cc(local_a, CONCAT22(1, local_16a), 2);
    }
    local_10 = (local_6 + 0xc);
    local_c = (local_6 + 0x10);
    local_2a = &local_10;
    if ((local_BX_6.field_0x24 == 6) && (local_c == 0)) {
        return;
    }
    temp_5ffa9150db = local_BX_6.field_0x28;
    local_14 = (temp_5ffa9150db + 0x20);
    puVar6 = &local_16;
    local_DX_314 = &local_18;
    paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, (local_14 >> 0x10));
    pass1_1030_5b1c(
        CONCAT22(local_DX_314, paVar5),
        CONCAT22(local_SS__1, &local_18),
        CONCAT22(local_SS__1, puVar6),
    );
    pass1_1028_c8ee(
        local_BX_6,
        local_ES_6,
        local_BX_6.field_0x24,
        CONCAT22(local_SS__1, &local_10),
    );
    pass1_1008_3eb4(
        CONCAT22(local_SS__1, &local_10),
        CONCAT22(local_SS__1, &local_22),
        CONCAT22(local_SS__1, local_20),
        CONCAT22(local_SS__1, local_1e),
    );
    if (local_BX_6.field_0x24 == 1) {
        if (local_18 < local_22) {
            pass1_1030_5b3e(CONCAT22(local_DX_314, paVar5), local_22, local_16);
            pass1_1030_5b1c(
                CONCAT22(local_DX_314, paVar5),
                CONCAT22(local_SS__1, &local_18),
                CONCAT22(local_SS__1, &local_16),
            );
        }
        local_32 = (local_a + 4);
        pass1_1028_87f0(
            CONCAT22(local_SS__1, &local_158),
            0,
            0,
            0x6a,
            &local_10,
            local_SS__1,
            local_32,
            local_14,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(local_SS__1, &local_158));
        local_158 = s_1_1050_389a;
        local_156 = &PTR_LOOP_1050_1008;
    }
    pass1_1028_ccd0(param_1, CONCAT22(local_SS__1, &local_10));
    return;
}

pub fn pass1_1020_e294(param_1: *mut astruct_44) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut local_EAX_110: u32;
    let mut local_DX_44: u16;
    let mut local_DX_110: u16;
    let mut uVar2: u16;
    let mut local_ES_6: u16;
    let mut local_SS__1: u16;
    let mut local_15e: u32;
    let mut local_154: u32;
    let mut local_150: [u8; 12];
    let mut local_144: u16;
    let mut local_140: u16;
    let mut local_13e: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fc08ad9c9: u32;
    let mut temp_5f52d5638b: u32;

    local_ES_6 = (param_1 >> 0x10);
    uVar2 = param_1;
    if ((1 < (uVar2 + 0x24)) && ((uVar2 + 0x24) < 6)) {
        temp_5f52d5638b = (uVar2 + 0x28);
        local_6 = (temp_5f52d5638b + 0x20);
        uVar1 = pass1_1028_b58e(param_1);
        local_a = CONCAT31(extraout_var, uVar1);
        local_10 = (local_a + 0xc);
        uStack12 = (local_a + 0x10);
        local_8 = local_DX_44;
        pass1_1028_c8ee(
            uVar2,
            local_ES_6,
            (uVar2 + 0x24),
            CONCAT22(local_SS__1, &local_10),
        );
        local_EAX_110._0_2_ = &local_10;
        local_DX_110 = local_DX_44;
        pass1_1028_c89c(
            param_1,
            CONCAT22(local_SS__1, local_EAX_110),
            CONCAT22(local_SS__1, local_150),
        );
        local_14 = local_EAX_110;
        local_15e._3_1_ = (local_14 >> 0x18);
        if ((local_15e._3_1_ == '\0') && (local_14 == 9)) {
            (uVar2 + 0x14) = 1;
        }
        local_18 = (local_a + 0x2e);
        local_1c = (local_18 + 4);
        pass1_1028_87f0(
            CONCAT22(local_SS__1, &local_140),
            0,
            (uVar2 + 0x14) + 1,
            0x79,
            &local_10,
            local_SS__1,
            local_1c,
            local_6,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(local_SS__1, &local_140));
    }
    (uVar2 + 0x26) = 1;
    return;
}

pub fn pass1_1020_e39c(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let uVar2: u8;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_14e: u16;
    let mut local_13e: u16;
    let mut local_13c: u16;
    let mut local_13a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1028_b58e(param_1);
    iVar3 = CONCAT31(extraout_var, uVar2);
    _local_6 = CONCAT31(extraout_var, uVar2) & 0xffff | in_DX << 0x10;
    local_c = (iVar3 + 0xc);
    local_8 = (iVar3 + 0x10);
    if (local_8 < 1) {
        uVar4 = 5;
    } else {
        uVar4 = 6;
    }
    (iVar3 + 0x14) = uVar4;
    uVar1 = (param_1 + 0x28);
    local_10 = (uVar1 + 0x20);
    local_14 = (iVar3 + 0x2e);
    local_18 = (local_14 + 4);
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_13c),
        0,
        1,
        param_2,
        &local_c,
        unaff_SS,
        local_18,
        local_10,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_13c));
    return;
}

pub fn pass1_1020_e44c(param_1: *mut astruct_726) {
    let piVar1: *mut i32;
    let local_BX_3: *mut astruct_726;
    let local_ES_3: *mut astruct_726;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x12 == 2) {
        piVar1 = &local_BX_3.field_0x14;
        unsafe { *piVar1 = *piVar1 + -1 };
        if ((local_BX_3.field_0x26 == 0) && (local_BX_3.field_0xc == 0x78)) {
            pass1_1020_e294(param_1);
        }
        if (local_BX_3.field_0x14 == 0) {
            pass1_1020_e08e(param_1);
            return;
        }
        if (local_BX_3.field_0x24 == 6) {
            local_BX_3.field_0xe = 0x49;
        }
    }
    return;
}

pub fn pass1_1020_e49a(param_1: *mut astruct_44) {
    let mut iVar1: i32;
    let uVar2: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1028_b58e(param_1);
    iVar1 = (CONCAT11(extraout_AH, uVar2) + 0x14);
    local_a = 0;
    if (iVar1 == 6) {
        local_a = 9;
    } else {
        if (iVar1 == 7) {
            local_a = 6;
        } else {
            if (iVar1 == 8) {
                local_a = 7;
            } else {
                if (iVar1 == 9) {
                    local_a = 8;
                }
            }
        }
    }
    pass1_1020_e39c(param_1, local_a);
    return;
}

pub fn pass1_1020_e4fa(param_1: u32, param_2: u16) {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2) {
        2 | 5 | 6 | 7 => local_4 = 4,
        3 | 8 => local_4 = 5,
        // default:
        _ => {
            uVar1 = pass1_1028_b58e(param_1);
            local_4 = (CONCAT11(extraout_AH, uVar1) + 0x14) + 2;
        }
    }
    return local_4;
}

pub fn pass1_1020_e558(param_1: *mut astruct_44) {
    let uVar1: u8;
    let puVar2: *mut u32;
    let paVar3: *mut astruct_493;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut local_30: u32;
    let mut local_24: [u8; 12];
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    uVar5 = param_1;
    if ((uVar5 + 0xc) == 0x79) {
        (uVar5 + 0x14) = (uVar5 + 0x24);
        (uVar5 + 0x24) = 0;
    }
    if ((uVar5 + 0x24) != 6) {
        uVar1 = pass1_1028_b58e(param_1);
        local_6 = CONCAT31(extraout_var, uVar1);
        local_a = (local_6 + 0x14);
        local_8 = local_a;
        local_4 = in_DX;
        pass1_1020_e4fa(param_1, local_a);
        local_10 = (local_6 + 0xc);
        uStack12 = (local_6 + 0x10);
        local_18 = CONCAT22(local_18._2_2_, &local_10);
        pass1_1028_c8ee(uVar5, uVar6, (uVar5 + 0x24), CONCAT22(unaff_SS, &local_10));
        puVar2 = &local_10;
        pass1_1028_c89c(
            param_1,
            CONCAT22(unaff_SS, puVar2),
            CONCAT22(unaff_SS, local_24),
        );
        unsafe { local_18 = *puVar2 };
        uVar4 = (puVar2 + 2);
        local_30._3_1_ = (local_18 >> 0x18);
        local_14._0_2_ = local_18;
        local_14 = local_18;
        if (local_30._3_1_ != '\0') {
            paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, uVar4);
            local_14._0_2_ = &paVar3.field_0x14;
        }
        local_8 = local_14;
        pass1_1020_e4fa(param_1, local_14);
        (uVar5 + 0x14) = local_a + local_14;
        return;
    }
    (uVar5 + 0x14) = 1;
    return;
}

pub fn pass1_1020_e652(param_1: u32, param_2: u16, param_3: u16, param_4: u32) -> i32 {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    local_8 = param_2;
    uStack4 = (param_2 + 4);
    uVar2 = (param_1 >> 0x10);
    pass1_1028_c8ee(
        param_1,
        uVar2,
        (param_1 + 0x24),
        CONCAT22(unaff_SS, &local_8),
    );
    iVar1 = pass1_1028_c7b6(param_1, &local_8, unaff_SS, param_4);
    if (iVar1 != 0) {
        iVar1 = 1;
    }
    return iVar1;
}

pub fn ret_one_1020_c3ae() -> u32 {
    return 1;
}

pub fn switch_statement_1020_c3b4(param_1: u16) -> u32 {
    let mut local_6: u32;

    local_6._0_2_ = 1;
    match (param_1) {
        1 | 2 | 3 | 5 | 8 | 9 | 10 | 0xb | 0xc => local_6._0_2_ = 3,
        4 => local_6._0_2_ = 6,
        6 | 0xf | 0x10 | 0x11 | 0x12 | 0x13 => local_6._0_2_ = 10,
        7 => local_6._0_2_ = 2,
        0xd | 0xe => {
            local_6._0_2_ = 1;
        }
    }
    return local_6;
}

pub fn pass1_1020_c42e(param_1: u16) {
    let mut uVar1: u16;

    if (param_1 == 0xf) {
        uVar1 = 1;
    } else {
        uVar1 = 3;
    }
    return uVar1;
}

pub fn pass1_1020_c444(param_1: *mut *mut astruct_706, param_2: u32, param_3: u32) {
    let local_BX_20: *mut astruct_706;
    let mut uVar1: u16;

    pass1_1030_1cd8(param_1, param_2, param_3);
    uVar1 = (param_1 >> 0x10);
    local_BX_20 = param_1;
    local_BX_20.field_0x18 = 0;
    local_BX_20.field_0x1c = 0;
    unsafe { *param_1 = 0xc834 };
    local_BX_20.field_0x2 = 0x1020;
    return;
}

pub fn pass1_1020_c47a(param_1: *mut astruct_44) {
    astruct_44 * *local_ES_4;

    local_ES_4 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0xc834;
    (param_1 + 2) = 0x1020;
    error_check_1000_17ce((param_1 + 0x18));
    pass1_1030_1d28(param_1);
    return;
}

pub fn pass1_1020_c4a8(param_1: *mut astruct_709, param_2: u32, param_3: u32, param_4: u16) {
    let mut u_var1: u32;
    let local_BX_4: *mut astruct_709;
    let local_BX_39: *mut astruct_710;
    let local_ES_4: *mut astruct_709;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x1c != 0) {
        pass1_1020_c6a4((param_1 & 0xffff | ZEXT24(local_ES_4) << 0x10));
    }
    uVar1 = local_BX_4.field_0x18;
    uVar2 = (uVar1 >> 0x10);
    local_BX_39 = (uVar1 + param_4 * 6);
    param_3 = (local_BX_39).field_0x0;
    param_2 = local_BX_39.field_0x4;
    return;
}

pub fn pass1_1020_c4f4(param_1: u32, param_2: u16, param_3: u16, param_2_00: u32) {
    let paVar1: *mut astruct_493;
    let local_AX_43: *mut astruct_712;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let lVar4: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    lVar4 = pass1_1020_c6de(param_1, param_2_00);
    uVar2 = (lVar4 >> 0x10);
    uVar3 = uVar2 | lVar4;
    if (lVar4 != 0) {
        paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
        local_AX_43._0_1_ = pass1_1030_6fa0(CONCAT22(uVar3, paVar1));
        local_AX_43 = CONCAT11(local_AX_43._1_1_, local_AX_43);
        (lVar4 + 4) = (local_AX_43 * 2 + 0x4ea4);
    }
    return;
}

pub fn pass1_1020_c538(param_1: *mut u8) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x12), (param_1 + 0x10));
}

pub fn pass1_1020_c54a(param_1: *mut astruct_709) {
    let local_ES_5: *mut astruct_709;
    let mut local_6: u32;

    local_ES_5 = (param_1 >> 0x10);
    if ((param_1 + 0x1c) != 0) {
        pass1_1020_c6a4((param_1 & 0xffff | ZEXT24(local_ES_5) << 0x10));
    }
    return;
}

pub fn pass1_1020_c5b4(param_1: *mut u32, param_2: u32) {
    let plVar1: *mut long;
    let ppcVar2: fn();
    let uVar3: u8;
    let paVar4: *mut astruct_493;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    uVar3 = pass1_1030_6fa0(CONCAT22(in_DX, paVar4));
    uVar5 = CONCAT31(extraout_var, uVar3);
    uVar6 = uVar5;
    pass1_1020_c6de(param_1, 0);
    _local_c = CONCAT22(extraout_DX, uVar6);
    uVar7 = (param_1 >> 0x10);
    if ((extraout_DX | uVar6) == 0) {
        unsafe {
            ppcVar2 = (*param_1 + 0x20);
            ppcVar2()
        };
        pass1_1020_c6de(param_1, 0);
        _local_c = CONCAT22(extraout_DX_00, uVar6);
        if ((extraout_DX_00 | uVar6) == 0) {
            return;
        }
    }
    (param_1 + 0x1c) = 1;
    plVar1 = (param_1 + 8);
    unsafe {
        *plVar1 = *plVar1 + 1;
        *_local_c = param_2;
        (_local_c + 4) = (uVar5 * 2 + 0x4ea4);
    }
    return;
}

pub fn pass1_1020_c644(in_struct_1: *mut astruct_713, param_2: u16, param_3: u32) {
    let plVar1: *mut long;
    let mut uVar2: u16;
    let local_AX_39: *mut astruct_714;
    let local_struct_1: *mut astruct_713;
    let local_struct_1_hi: *mut astruct_713;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (&local_struct_1.field_0x18 == 0) {
        fn_ptr_1 = (in_struct_1 + 0x20);
        (**fn_ptr_1)();
    }
    local_AX_39 = (&local_struct_1.field_0x8 * 6 + local_struct_1.field_0x18);
    uVar2 = local_struct_1.field_0x1a;
    _local_6 = CONCAT22(uVar2, local_AX_39);
    plVar1 = &local_struct_1.field_0x8;
    unsafe {
        *plVar1 = *plVar1 + 1;
        *_local_6 = param_3
    };
    local_AX_39.field_0x4 = param_2;
    return;
}

pub fn pass1_1020_c694(param_1: *mut astruct_709) {
    pass1_1020_c6a4(param_1);
    return;
}

pub fn pass1_1020_c6a4(param_1: *mut astruct_709) {
    let local_struct_1: *mut astruct_709;
    let local_ES_3: *mut astruct_709;

    local_ES_3 = (param_1 >> 0x10);
    local_struct_1._0_2_ = param_1;
    if (((local_struct_1 + 0x18) != 0) && ((local_struct_1 + 8) != 0)) {
        pass1_1000_4aea(
            (local_struct_1 + 0x18),
            (local_struct_1 + 0x10),
            6,
            0xc7fa,
            0x1020,
        );
        (local_struct_1 + 0x1c) = 0;
    }
    return;
}

pub fn pass1_1020_c6de(in_struct_1: *mut astruct_715, param_2: u32) {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let local_struct_1: *mut astruct_715;
    let local_struct_1_hi: *mut astruct_715;
    let mut local_6: u32;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x1c != 0) {
        pass1_1020_c6a4((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
    }
    local_6 = 0;
    while (true) {
        puVar1 = &local_struct_1.field_0x10;
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val < local_6 || pu_var1_val == local_6) {
            return;
        }
        uVar2 = local_struct_1.field_0x18;
        if ((uVar2 + local_6 * 6) == param_2) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn pass1_1020_c73a(in_struct_1: *mut astruct_716) {
    let puVar1: *mut u32;
    let paVar2: *mut astruct_94;
    let mut uVar3: i32;
    let extraout_DX: *mut u16;
    let local_struct_1: *mut astruct_716;
    let local_struct_1_hi: *mut astruct_716;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5ffbaa3a02: *mut astruct_717;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (&local_struct_1.field_0x10 == 0) {
        paVar2 = local_struct_1.field_0xc;
        g_u16_ptr_1050_5f2e = local_struct_1.field_0xe;
    } else {
        temp_5ffbaa3a02 = local_struct_1.field_0x10;
        puVar1 = &local_struct_1.field_0x14;
        let pu_var1_val = unsafe { *puVar1 };
        paVar2 = (temp_5ffbaa3a02 + pu_var1_val);
        g_u16_ptr_1050_5f2e = (local_struct_1.field_0x12
            + local_struct_1.field_0x16
            + CARRY2(temp_5ffbaa3a02, pu_var1_val));
    }
    _local_6 = CONCAT22(g_u16_ptr_1050_5f2e, paVar2);
    if (&local_struct_1.field_0x18 == 0) {
        if (__g_astruct_94_ptr_1 == 0) {
            _g_astruct_94_ptr_1 = paVar2;
            struct_fn_1000_160a();
        } else {
        }
        uVar3 = paVar2 * 6;
        alloc_mem_1000_1708(uVar3, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
    } else {
        uVar3 = paVar2 * 6;
        alloc_mem_1000_0ed4(
            1,
            uVar3,
            (g_u16_ptr_1050_5f2e * 3 + CARRY2(paVar2, paVar2) + CARRY2(paVar2 * 2, paVar2)) * 2
                + CARRY2(paVar2 * 3, paVar2 * 3),
            &local_struct_1.field_0x18,
        );
        g_u16_ptr_1050_5f2e = extraout_DX;
    }
    _local_a = CONCAT22(g_u16_ptr_1050_5f2e, uVar3);
    if ((g_u16_ptr_1050_5f2e | uVar3) != 0) {
        &local_struct_1.field_0x10 = _local_6;
        &local_struct_1.field_0x18 = _local_a;
    }
    local_struct_1.field_0x1c = 1;
    return;
}

pub fn pass1_1020_c7fa(param_1: *mut u8, param_2: u32) -> i32 {
    return (param_1 + 4) - (param_2 + 4);
}

pub fn pass1_1020_c80e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_c47a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_c835() {
    let pbVar1: *mut byte;
    let pcVar2: *mut char;
    let mut bVar3: u8;
    let mut cVar5: u8;
    let mut uVar6: u32;
    let mut in_CH: u8;
    let mut in_DL: u8;
    let mut in_DH: u8;
    let mut in_BX: i32;
    let puVar7: *mut u16;
    let mut iVar8: i32;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_SS: u16;
    let mut uVar9: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut in_stack_000020c3: u8;
    let mut bVar4: u8;

    puVar7 = &stack0xfffe;
    cVar5 = '\t';
    while {
        unaff_BP = unaff_BP + -1;
        puVar7 = puVar7 + -1;
        unsafe { *puVar7 = *unaff_BP };
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    uVar9 = (*(in_BX + unaff_SI) >> 0x10);
    bVar10 = CARRY1(in_stack_000020c3, in_CH) || CARRY1(in_stack_000020c3 + in_CH, in_CF);
    pbVar1 = (s_514a_bmp_1050_20c5 + unaff_SI);
    unsafe {
        bVar3 = *pbVar1;
        bVar4 = *pbVar1;
        *pbVar1 = bVar4 + in_DH + bVar10;
        pcVar2 = (in_BX + unaff_SI);
        *pcVar2 = *pcVar2 + (in_BX >> 8) + (CARRY1(bVar3, in_DH) || CARRY1(bVar4 + in_DH, bVar10));
        uVar6 = (in_BX + unaff_SI);
        uVar9 = (uVar6 >> 0x10);
        iVar8 = uVar6;
        (iVar8 + -2) = uVar9;
        pbVar1 = (in_BX + unaff_SI);
        *pbVar1 = *pbVar1 ^ in_DL;
        (iVar8 + -4) = unaff_SS;
        pbVar1 = (in_BX + unaff_SI);
        *pbVar1 = *pbVar1 ^ in_DL;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn pass1_1020_c860(param_1: *mut u8) {
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 6), (param_1 + 4));
}

pub fn pass1_1020_c872(param_1: u32, param_2: u32, param_3: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let piVar3: *mut i32;
    let local_struct_1: *mut astruct_501;
    let mut uVar4: i32;
    let mut extraout_DX: i32;
    let local_BX_126: *mut astruct_502;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut bVar10: bool;
    let mut local_2a: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;
    let puVar5: *mut u8;

    puVar5 = _PTR_LOOP_1050_4fb8;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_4fb8);
    uVar4 = puVar5;
    local_6 = puVar5 & 0xffff | extraout_DX << 0x10;
    if ((extraout_DX | uVar4) == 0) {
        local_6 = 0;
    } else {
        local_6 = s_1_1050_389a;
        (uVar4 + 2) = &PTR_LOOP_1050_1008;
        (uVar4 + 4) = 0;
        (uVar4 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (uVar4 + 2) = &PTR_LOOP_1050_1008;
        (uVar4 + 0xe) = 0;
        (uVar4 + 0xc) = 0;
        local_6 = 0xc9e6;
        (uVar4 + 2) = 0x1020;
    }
    if (local_6 == 0) {
        return;
    }
    uVar7 = (local_6 >> 0x10);
    local_BX_126 = local_6;
    local_BX_126.field_0x8 = param_3;
    local_BX_126.field_0xc = param_2;
    uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    local_e = (iVar6 + 4);
    uVar9 = (iVar6 + 6);
    if ((iVar6 + 8) == 0) {
        // LAB_1020_c92d:
        local_BX_126.field_0x4 = (iVar6 + 4);
    } else {
        puVar1 = (local_e + 0xe);
        unsafe {
            bVar10 = *puVar1 < param_2._2_2_;
            if ((bVar10 || *puVar1 == param_2._2_2_)
                && (bVar10
                    || (
                        puVar1 = (local_e + 0xc),
                        *puVar1 < param_2 || *puVar1 == param_2,
                    )))
            {}
        }
        // goto LAB_1020_c92d;
        bVar10 = false;
        while (true) {
            if (local_e == 0) {
                break;
            }
            uVar9 = (local_e >> 0x10);
            puVar2 = (local_e + 0xc);
            let pu_var2_val = unsafe { *puVar2 };
            if (pu_var2_val < param_2 || pu_var2_val == param_2) {
                bVar10 = true;
                local_BX_126.field_0x4 = local_e;
                (local_a + 4) = local_6;
                break;
            }
            local_a = local_e;
            local_e = (local_e + 4);
        }
        param_1 = local_a;
        if (bVar10) {}
        // goto LAB_1020_c9ab;
    }
    uVar9 = (param_1 >> 0x10);
    (param_1 + 4) = local_BX_126;
    (param_1 + 6) = uVar7;
    // LAB_1020_c9ab:
    unsafe {
        piVar3 = (iVar6 + 8);
        *piVar3 = *piVar3 + 1;
    }
    return;
}

pub fn call_free_mem_1020_c9ba(param_1: *mut u16, param_2: u8) {
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        free_mem_1000_093a(param_1);
    }
    return param_1;
}

pub fn pass1_1020_c9ea(param_1: *mut u16) {
    pass1_1028_0954(param_1);
    unsafe {
        *param_1 = 0xcc7c;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_ca0c(param_1: *mut astruct_743, param_2: u16, param_3: u16, param_3_00: *mut u8) {
    pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_3_00);
    CONCAT22(param_2, param_1) = 0xcc7c;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_ca36(param_1: i32, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let local_struct_2: *mut pass1_struct_1;
    let mut in_stack_0000ffee: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_09b8(param_1, param_2, param_3);
    uVar1 = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    if ((uVar1 + 0x200) != 0x8000002) {
        local_struct_2 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffee, 8));
        pass1_1010_988c(local_struct_2, (param_1 + 0xc));
    }
    return;
}

pub fn pass1_1020_ca82(param_1: *mut u8) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    uVar1 = pass1_1028_b4f2(param_1);
    if ((uVar1 + 0x200) != 0x8000002) {
        if ((param_1 + 0x12) == 5) {
            pass1_1020_cac2(param_1);
        }
    }
    return;
}

pub fn pass1_1020_cac2(param_1: *mut astruct_44) {
    let puVar1: *mut u16;
    let mut iVar2: i32;
    let ppcVar3: fn();
    let uVar4: u8;
    let local_AX_96: *mut astruct_718;
    let mut uVar5: u16;
    let extraout_var: u32;
    let mut extraout_DX: i32;
    let mut uVar6: i32;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let ppVar8: *mut pass1_struct_1;
    let mut in_stack_0000ffca: u16;
    let mut local_34: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffca, 2));
    local_4 = (ppVar8 >> 0x10);
    local_6 = ppVar8;
    local_8 = u16_1050_13ae;
    if (u16_1050_13ae == 1) {
        local_8 = 2;
    }
    _local_c = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffca, 8));
    uVar7 = (_local_c >> 0x10);
    local_10 = (_local_c + 10);
    local_14 = (_local_c + 0xe);
    pass1_1008_5784(CONCAT22(unaff_SS, &stack0xffe4), local_10);
    loop {
        while {
            loop {
                while {
                    local_AX_96 = &stack0xffe4;
                    pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_96));
                    uVar6 = extraout_DX | local_AX_96;
                    if (uVar6 == 0) {
                        return;
                    }
                    iVar2 = local_AX_96.field_0x4;
                    (iVar2 < 0x12) || (SBORROW2(iVar2, 0x12))
                } {}
                if (iVar2 != 0x13 && 0 < iVar2 + -0x12) {
                    break;
                }
                local_34 = 0;
                if (local_8 == 3) {
                    local_34 = local_AX_96.field_0x6 / 2;
                } else {
                    if (local_8 == 4) {
                        iVar2 = local_AX_96.field_0x6 * 3;
                        local_34 = (iVar2 + (iVar2 >> 0xf & 3)) >> 2;
                    }
                }
                puVar1 = &local_AX_96.field_0x6;
                unsafe { *puVar1 = *puVar1 - local_34 };
                uVar7 = (local_10 >> 0x10);
                (local_10 + 10) = 0;
                ppcVar3 = (local_10 + 0xc);
                (**ppcVar3)(
                    &PTR_LOOP_1050_1008,
                    local_10,
                    uVar7,
                    local_AX_96,
                    extraout_DX,
                );
                (local_10 + 10) = 1;
                local_18 = 0;
                ppcVar3 = (local_14 + 4);
                (**ppcVar3)(
                    &PTR_LOOP_1050_1008,
                    local_14,
                    (local_14 >> 0x10),
                    local_AX_96,
                    extraout_DX,
                );
            }
            iVar2 != 0x81
        } {}
        local_24 = 0;
        if (local_8 == 2) {
            uVar5 = local_AX_96.field_0x6;
            // LAB_1020_cba7:
            uVar6 = uVar5 >> 0xf & 3;
            local_24 = (uVar5 + uVar6) >> 2;
        } else {
            if (local_8 == 3) {
                uVar5 = local_AX_96.field_0x6;
                uVar6 = uVar5 >> 0xf;
                local_24 = uVar5 / 2;
            } else {
                if (local_8 == 4) {
                    uVar5 = local_AX_96.field_0x6 * 3;
                    // goto LAB_1020_cba7;
                }
            }
        }
        uVar4 = pass1_1028_b58e(param_1);
        pass1_1030_7ddc(
            CONCAT31(extraout_var, uVar4) & 0xffff | uVar6 << 0x10,
            (local_AX_96.field_0x6 - local_24),
            0x1e,
        );
        ppcVar3 = (local_10 + 0xc);
        (**ppcVar3)(
            0x1030,
            local_10,
            (local_10 >> 0x10),
            local_AX_96,
            extraout_DX,
        );
        local_18 = 0;
    }
}

pub fn pass1_1020_cc56(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_cce4(param_1: *mut u16) {
    pass1_1028_b354(param_1);
    unsafe {
        *param_1 = 0xcd7e;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_cd06(param_1: i32, param_2: u16, param_3: u16, param_3_00: *mut u8) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xcd7e;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_cd30(param_1: *mut astruct_719) {
    let local_BX_3: *mut astruct_719;
    let local_ES_3: *mut astruct_719;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (((local_BX_3.field_0x12 == 6) || (local_BX_3.field_0x12 == 5))
        && ((local_BX_3.field_0x1a & 2) != 0))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_cd58(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_cde6(param_1: *mut u8) {
    pass1_1028_0954(param_1);
    param_1 = 0xd004;
    (param_1 + 2) = 0x1020;
    return param_1;
}

pub fn pass1_1020_ce08(param_1: *mut astruct_743, param_2: u16, param_3: u16, param_3_00: *mut u8) {
    pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_3_00);
    CONCAT22(param_2, param_1) = 0xd004;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_ce32(param_1: i32, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let ppVar2: *mut pass1_struct_1;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let uVar5: u8;
    let uVar6: u8;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_09b8(param_1, param_2, param_3);
    uVar1 = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    if ((uVar1 + 0x200) != 0x8000002) {
        ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffea, 8));
        pass1_1010_988c(ppVar2, (param_1 + 0xc));
        uVar9 = 0;
        uVar10 = 9;
        uVar7 = 1;
        uVar8 = 0;
        uVar4 = 0;
        uVar5 = 0;
        uVar6 = 0;
        uVar3 = 0;
        ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(
            ppVar2,
            CONCAT22(uVar4, uVar3),
            CONCAT11(uVar6, uVar5),
            uVar7,
            CONCAT22(uVar9, uVar8),
            uVar10,
        );
    }
    return;
}

pub fn pass1_1020_ce9e(param_1: *mut u8) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    if ((param_1 + 0x12) == 5) {
        pass1_1020_cefc(param_1);
        uVar2 = pass1_1028_b4f2(param_1);
        uVar1 = (uVar2 >> 0x10);
        if ((uVar2 + 0x200) != 0x8000002) {
            ppVar3 = process_struct_1010_20ba(
                _g_astruct_372_1050_0ed0,
                CONCAT22(in_stack_0000ffea, 0x2b),
            );
            pass1_1010_043a(ppVar3, (uVar2 + 4), 5);
        }
    }
    return;
}

pub fn pass1_1020_cefc(param_1: *mut u8) {
    let puVar1: *mut u8;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    puVar1 = pass1_1028_b4f2(param_1);
    if ((puVar1 + 0x200) == 0x8000002) {
        local_c = 0x32;
    } else {
        ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffec, 8));
        local_c = pass1_1010_96c2(ppVar2);
        if (0x32 < local_c) {
            local_c = 0x32;
        }
        pass1_1010_96a8(ppVar2, local_c);
    }
    pass1_1020_cf6c(param_1, (param_1 >> 0x10), local_c, puVar1);
    return;
}

pub fn pass1_1020_cf6c(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: *mut u8) {
    astruct_721 * *ppaVar1;
    let puVar2: *mut u32;
    astruct_722 * *ppaVar3;
    let paVar4: *mut astruct_721;
    let mut uVar5: i32;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let local_AX_14: *mut astruct_720;
    let mut uVar8: i32;
    let local_AX_42: *mut astruct_721;
    let local_DX_60: *mut astruct_722;
    let mut uVar9: u16;
    let mut local_a: u32;

    uVar9 = (param_2_00 >> 0x10);
    uVar6 = (param_2_00 + 0x1f6);
    local_AX_14 = uVar6;
    uVar7 = (uVar6 >> 0x10);
    uVar8 = param_1_00 / 5;
    local_AX_42 = (uVar8 * -4 + param_1_00);
    ppaVar1 = &local_AX_14.field_0x50;
    paVar4 = *ppaVar1;
    *ppaVar1 = local_AX_42 + *ppaVar1;
    puVar2 = &local_AX_14.field_0x52;
    unsafe {
        *puVar2 = *puVar2 + (local_AX_42 >> 0xf) + CARRY2(paVar4, local_AX_42);
        local_DX_60 = (uVar8 >> 0xf);
        puVar2 = &local_AX_14.field_0x78;
        uVar5 = *puVar2;
        *puVar2 = *puVar2 + uVar8;
        ppaVar3 = &local_AX_14.field_0x7a;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(uVar5, uVar8));
        puVar2 = &local_AX_14.field_0xa0;
        uVar5 = *puVar2;
        *puVar2 = *puVar2 + uVar8;
        ppaVar3 = &local_AX_14.field_0xa2;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(uVar5, uVar8));
        puVar2 = &local_AX_14.field_0xc8;
        uVar5 = *puVar2;
        *puVar2 = *puVar2 + uVar8;
        ppaVar3 = &local_AX_14.field_0xca;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(uVar5, uVar8));
        puVar2 = &local_AX_14.field_0xf0;
        uVar5 = *puVar2;
        *puVar2 = *puVar2 + uVar8;
        ppaVar3 = &local_AX_14.field_0xf2;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(uVar5, uVar8));
        (param_2_00 + 0x1fe) = 1;
    }
    return;
}

pub fn pass1_1020_cfde(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d06c(param_1: *mut u8) {
    pass1_1028_b354(param_1);
    param_1 = 0xd314;
    (param_1 + 2) = 0x1020;
    return param_1;
}

pub fn pass1_1020_d08e(param_1: i32, param_2: u16, param_3: u16, param_3_00: *mut u8) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xd314;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d0b8(param_1: *mut u8) {
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x12) != 6) {
        return;
    }
    uVar2 = pass1_1028_b4f2(param_1);
    if ((uVar2 + 0x200) != 0x8000002) {
        iVar1 = pass1_1028_cb04(param_1);
        if ((iVar1 == 0) || (iVar1 = pass1_1020_d194(param_1), iVar1 == 0)) {
            iVar1 = 6;
            // goto LAB_1020_d10b;
        }
        pass1_1028_c952(param_1);
    }
    iVar1 = 5;
    // LAB_1020_d10b:
    pass1_1028_bdac(param_1, iVar1);
    return;
}

pub fn pass1_1020_d118(param_1: *mut u8, param_2: u32, param_3: u32, param_4: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uvar3: u16;

    iVar1 = pass1_1028_c7b6(param_1, param_2, param_4);
    if (iVar1 == 5) {
        uVar2 = param_1;
        uVar3 = (param_1 >> 0x10);
        pass1_1028_c23e(uVar2, uVar3, param_2, param_3, param_4);
        if (iVar1 != 0) {
            pass1_1028_c3aa(uVar2, uVar3, param_2, param_3, param_4);
            if (iVar1 != 0) {
                uVar2 = pass1_1028_c314(uVar2, uVar3, param_2, param_3, (param_3 >> 0x10), param_4);
                if (uVar2 != 0) {
                    return 1;
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1020_d194(param_1: *mut astruct_44) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let uVar4: u8;
    let paVar5: *mut astruct_493;
    let puVar6: *mut u8;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let extraout_var: u32;
    let puVar10: *mut u8;
    let mut uVar11: u32;
    let mut uVar12: i32;
    let mut uVar13: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let mut uVar14: u16;
    let mut unaff_SS: u16;
    let mut uVar15: u32;
    let ppVar16: *mut pass1_struct_1;
    let uVar17: u8;
    let uVar18: u8;
    let mut uVar19: u16;
    let mut uVar20: u16;
    let mut uVar21: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    pass1_1030_bcae(local_4, unaff_SS);
    uVar15 = pass1_1028_b4f2(param_1);
    uVar12 = (uVar15 >> 0x10);
    uVar1 = (uVar15 + 0x10);
    paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    uVar13 = uVar12;
    uVar4 = pass1_1028_b58e(param_1);
    uVar3 = CONCAT31(extraout_var, uVar4) & 0xffff;
    puVar6 = local_4;
    pass1_1030_bd74(
        puVar6,
        unaff_SS,
        CONCAT22(uVar12, paVar5),
        (CONCAT31(extraout_var, uVar4) & 0xffff | uVar13 << 0x10),
    );
    if (puVar6 < 0) {
        return;
    }
    if (0x1e < puVar6) {
        uVar20 = 0x87;
        ppVar16 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x870009);
        iVar7 = ppVar16;
        pass1_1010_65d0(ppVar16, uVar20);
        if (iVar7 == 0) {
            uVar12 = extraout_DX;
            puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x15);
            uVar8 = SUB42(puVar10, 0);
            uVar14 = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_4e78(uVar15, puVar10 & 0xffff | uVar12 << 0x10);
            _local_22 = CONCAT22(extraout_DX_00, uVar8);
            ppcVar2 = (*_local_22 + 0x10);
            uVar9 = uVar8;
            uVar19 = uVar8;
            uVar21 = extraout_DX_00;
            ppcVar2(&PTR_LOOP_1050_1038, uVar8, extraout_DX_00);
            _local_26 = CONCAT22(extraout_DX_01, uVar9);
            local_2a = 0;
            while (true) {
                if (_local_26 <= local_2a) {
                    if (_local_22 == 0x0) {
                        return;
                    }
                    ppcVar2 = *_local_22;
                    ppcVar2(
                        uVar14,
                        uVar8,
                        extraout_DX_00,
                        1,
                        uVar19,
                        uVar21,
                        _local_22,
                        _local_22,
                    );
                    return;
                }
                uVar17 = uVar3;
                uVar18 = (uVar3 >> 8);
                uVar11 = _local_26;
                uVar12 = uVar13;
                pass1_1030_1d58(_local_22);
                puVar6 = local_4;
                uVar14 = 0x1030;
                pass1_1030_bd74(
                    puVar6,
                    unaff_SS,
                    (uVar11 & 0xffff | extraout_DX_02 << 0x10),
                    CONCAT22(uVar12, CONCAT11(uVar18, uVar17)),
                );
                if ((0 < puVar6) && (puVar6 < 0x1f)) {
                    break;
                }
                local_2a = local_2a + 1;
            }
            if (_local_22 == 0x0) {
                return;
            }
            ppcVar2 = *_local_22;
            ppcVar2(
                0x1030,
                uVar8,
                extraout_DX_00,
                1,
                uVar19,
                uVar21,
                _local_22,
                _local_22,
                uVar11,
                extraout_DX_02,
            );
            return;
        }
    }
    return;
}

pub fn pass1_1020_d2ee(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d37c(param_1: *mut u16) {
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    unsafe {
        (param_1 + 0x20) = 0;
        *param_1 = 0xd53e;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_d3a4(param_1: *mut u16, param_2: u16, param_3: u16, param_4: u32) {
    let mut uVar1: u16;

    pass1_1028_b39e(param_1, param_3, param_4);
    uVar1 = (param_1 >> 0x10);
    unsafe {
        (param_1 + 0x20) = param_2;
        *param_1 = 0xd53e;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_a6ee(param_1: *mut u8, param_2: u16) {
    let local_AX__1: *mut astruct_698;
    let paVar1: *mut astruct_493;
    let local_AX_84: *mut astruct_699;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut in_stack_0000fea0: u16;
    let mut local_142: u32;
    let mut local_13e: u16;
    let mut local_13c: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
    _local_6 = CONCAT22(in_DX, paVar1);
    if (((in_DX | paVar1) == 0) || (&paVar1[0x11].field_0x2 == 0x8000002)) {
        _local_a =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fea0, 0x2f));
        local_10 = (_local_a >> 0x10);
        local_e = (_local_a + 0x20);
        local_AX_84 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_e, (local_e >> 0x10));
        local_12 = local_AX_84;
        zero_list_1008_3e38(CONCAT22(unaff_SS, &local_18));
        local_1a = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, param_2, 0x28);
        if (local_1a != 0) {
            local_14 = 1;
        }
        pass1_1020_b2da(
            param_1,
            (local_1a != 0),
            CONCAT22(unaff_SS, &local_18),
            CONCAT22(local_10, local_12),
        );
        pass1_1028_87f0(
            CONCAT22(unaff_SS, &local_13e),
            0,
            0,
            param_2,
            &local_18,
            unaff_SS,
            (_PTR_LOOP_1050_4e70 + 4),
            (local_12 + 4),
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_13e));
        if (local_1a != 0) {
            pass1_1020_ad90(CONCAT22(unaff_SS, &local_18), (local_12 + 4));
        }
        (local_1e + 0x1c) = 0x8000001;
    }
    return;
}

pub fn pass1_1020_a80e(param_1: u16, param_2: u16, param_2_00: i32) {
    let mut u_var1: u32;
    let local_AX__1: *mut astruct_700;
    let paVar2: *mut astruct_493;
    let paVar3: *mut astruct_493;
    let mut in_DX: i32;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
    if (((in_DX | paVar2) == 0) || (&paVar2[0x11].field_0x2 == 0x8000002)) {
        ppVar5 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe4, 0x2f));
        uVar4 = (ppVar5 >> 0x10);
        uVar1 = (ppVar5 + 0x20);
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        if (param_2_00 == 10) {
            call_infinite_loop_1020_b872(param_1, param_2, paVar2, uVar4);
            return;
        }
        paVar3 = paVar2;
        pass1_1020_b0aa(param_1, param_2, param_2_00);
        if (paVar3 != 0x0) {
            pass1_1020_abc0(param_1, param_2, paVar3, paVar2, uVar4);
        }
    }
    return;
}

pub fn pass1_1020_a89e(param_1: *mut u8, param_2: *mut u32, param_3: u16) {
    let piVar1: *mut i32;
    let local_AX__1: *mut astruct_701;
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut extraout_DX: i32;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let mut local_5ee: u16;
    let mut local_5ec: u16;
    let mut local_4c2: u16;
    let mut local_4be: u16;
    let mut local_4bc: u16;
    let mut local_4ba: u16;
    let mut local_4b8: [u8; 8];
    let mut local_4b0: u32;
    let mut local_4ac: u16;
    let mut local_4aa: u16;
    let mut local_4a8: u16;
    let mut local_4a6: u16;
    let mut local_384: u16;
    let mut local_382: u16;
    let mut local_260: u16;
    let mut local_25e: u16;
    let mut local_13c: u16;
    let mut local_13a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 6];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
    uVar6 = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    _local_e = CONCAT22(uVar6, paVar2);
    unsafe {
        local_14._0_4_ = *param_2;
        local_14._4_2_ = (param_2 + 1);
    }
    local_4c2 = local_14;
    pass1_1008_3e94(
        local_14,
        CONCAT22(unaff_SS, &local_18),
        CONCAT22(unaff_SS, &local_16),
    );
    pass1_1008_3e76(CONCAT22(unaff_SS, local_14), 0, local_18, local_16 + 2);
    pass1_1028_8888(
        CONCAT22(unaff_SS, &local_13c),
        0,
        0x7a,
        local_14,
        unaff_SS,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_13c));
    pass1_1008_3e76(CONCAT22(unaff_SS, local_14), 0, local_18 - 2, local_16);
    pass1_1028_8888(
        CONCAT22(unaff_SS, &local_260),
        0,
        0x47,
        local_14,
        unaff_SS,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_260));
    pass1_1008_3e76(CONCAT22(unaff_SS, local_14), 1, local_18 - 2, local_16);
    pass1_1028_8888(
        CONCAT22(unaff_SS, &local_384),
        0,
        0x6a,
        local_14,
        unaff_SS,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_384));
    pass1_1020_ad90(CONCAT22(unaff_SS, local_14), local_a);
    pass1_1008_3e76(CONCAT22(unaff_SS, local_14), 1, local_18 - 2, local_16 + 1);
    pass1_1028_8888(
        CONCAT22(unaff_SS, &local_4a8),
        0,
        0x7f,
        local_14,
        unaff_SS,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_4a8));
    pass1_1020_ad90(CONCAT22(unaff_SS, local_14), local_a);
    _local_4ac = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 8));
    local_4b0 = (_local_4ac + 0x12);
    pass1_1008_5784(CONCAT22(unaff_SS, local_4b8), local_4b0);
    local_4be = 0;
    loop {
        while {
            puVar3 = local_4b8;
            pass1_1008_5b12(CONCAT22(unaff_SS, puVar3));
            _local_4bc = CONCAT22(extraout_DX, puVar3);
            if ((extraout_DX | puVar3) == 0) {
                pass1_1010_9674(_local_4ac);
                return;
            }
            ((puVar3 + 4) != 0x3e) && ((puVar3 + 4) != 0x41)
        } {}
        while (0 < (_local_4bc + 6)) {
            if (local_4be == 0) {
                uVar5 = local_16 - 2;
                // LAB_1020_ab4a:
                uVar4 = local_18 - 2;
                // LAB_1020_ab51:
                local_4be = local_4be + 1;
                pass1_1008_3e76(CONCAT22(unaff_SS, local_14), 0, uVar4, uVar5);
            } else {
                if (local_4be == 1) {
                    uVar5 = local_16 + 2;
                    // goto LAB_1020_ab4a;
                }
                if (local_4be == 2) {
                    uVar5 = local_16 + 2;
                    // LAB_1020_ab6e:
                    uVar4 = local_18 + 2;
                    // goto LAB_1020_ab51;
                }
                if (local_4be == 3) {
                    uVar5 = local_16 - 2;
                    // goto LAB_1020_ab6e;
                }
                local_4be = local_4be + 1;
                pass1_1020_b2da(param_1, 0, CONCAT22(unaff_SS, local_14), _local_e);
            }
            pass1_1028_8888(
                CONCAT22(unaff_SS, &local_5ee),
                0,
                (_local_4bc + 4),
                local_14,
                unaff_SS,
                0x8000002,
                0x4000002,
                local_a,
            );
            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_5ee));
            piVar1 = (_local_4bc + 6);
            unsafe {
                *piVar1 = *piVar1 + -1;
            }
            local_5ee = s_1_1050_389a;
            local_5ec = &PTR_LOOP_1050_1008;
        }
    }
}

pub fn pass1_1020_abc0(param_1: *mut u8, param_2: u16, param_3: u32) {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    zero_list_1008_3e38(CONCAT22(unaff_SS, &local_8));
    local_a = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, param_2, 0x28);
    if (local_a != 0) {
        local_4 = 1;
    }
    pass1_1020_b2da(
        param_1,
        (local_a != 0),
        CONCAT22(unaff_SS, &local_8),
        param_3,
    );
    uVar1 = (param_3 >> 0x10);
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_12e),
        0,
        0,
        param_2,
        &local_8,
        unaff_SS,
        (_PTR_LOOP_1050_4e70 + 4),
        (param_3 + 4),
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_12e));
    if (local_a != 0) {
        pass1_1020_ad90(CONCAT22(unaff_SS, &local_8), (param_3 + 4));
    }
    return;
}

pub fn pass1_1020_ac6e(param_1: *mut u8, param_2: u16, param_3: u16, param_4: u16) {
    let BVar1: bool;
    let puVar2: *mut u8;
    let mut local_DX_124: u16;
    let mut local_DX_154: u16;
    let mut local_SS__1: u16;
    let local_162: u8;
    let puStack350: *mut u16;
    let puStack342: *mut u16;
    let mut local_152: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 6];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        _local_162 = SUB42(&PTR_LOOP_1050_4230, 0);
    } else {
        _local_162 = SUB42(s_dib_1050_4234 + 2, 0);
    }
    puStack342 = &local_4;
    puStack350 = &local_8;
    pass1_1008_3eb4(
        CONCAT22(0x1048, _local_162),
        CONCAT22(local_SS__1, puStack350),
        CONCAT22(local_SS__1, &local_6),
        CONCAT22(local_SS__1, puStack342),
    );
    if (param_4 == 0) {
        _local_6 = _local_6 & 0xffff | (local_4 + param_3) << 0x10;
    } else {
        if (param_4 == 1) {
            _local_6 = _local_6 & 0xffff0000 | (local_6 + param_3);
        } else {
            if (param_4 == 2) {
                _local_6 = _local_6 & 0xffff | (local_4 - param_3) << 0x10;
            }
        }
    }
    puStack342 = _local_6;
    pass1_1008_3e54(
        CONCAT22(local_SS__1, local_e),
        local_8,
        puStack342,
        (_local_6 >> 0x10),
    );
    _local_12 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_152, 0x2f));
    local_DX_124 = (_local_12 >> 0x10);
    local_16 = (_local_12 + 0x20);
    puStack342 = local_16;
    local_DX_154 = local_DX_124;
    local_1a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puStack342, (local_16 >> 0x10));
    local_18 = local_DX_154;
    BVar1 = pass1_1020_b1ae(param_1, CONCAT22(local_SS__1, local_e), *(local_1a + 4));
    if (BVar1 != 0) {
        puVar2 = local_e;
        pass1_1020_b240(
            param_1,
            CONCAT22(local_SS__1, puVar2),
            CONCAT22(local_18, local_1a),
        );
        if (puVar2 != 0x0) {
            pass1_1028_87f0(
                CONCAT22(local_SS__1, &local_146),
                0,
                0,
                (-(param_2 == 0) & 0xfffb) + 0x7f,
                local_e,
                local_SS__1,
                (_PTR_LOOP_1050_4e70 + 4),
                local_16,
            );
            pass1_1030_835a(_g_bool_1050_5748, CONCAT22(local_SS__1, &local_146));
        }
    }
    return;
}

pub fn pass1_1020_ad90(param_1: *mut u8, param_2: u32) {
    let ppcVar1: fn();
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar6: i32;
    let mut extraout_DX_01: u16;
    let mut unaff_SS: u16;
    let puVar7: *mut u32;
    let mut uVar8: u16;
    let uVar9: u8;
    let uVar10: u8;
    let mut uVar11: u16;
    let mut local_17e: u16;
    let mut local_17c: u16;
    let mut local_5a: u16;
    let mut local_4e: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: [u8; 12];
    let mut local_2e: [u8; 6];
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_14 = &local_8;
    pass1_1008_3eb4(
        param_1,
        CONCAT22(unaff_SS, local_14),
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    pass1_1030_627e(_PTR_LOOP_1050_5740, param_1, param_2);
    local_26 = extraout_DX;
    local_12 = extraout_DX;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, extraout_DX);
    _local_18 = CONCAT22(local_26, paVar2);
    local_1c = &paVar2[1].field_0x10;
    local_20 = (local_1c + 4);
    local_28 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    puVar7 = pass1_1030_5b5c(local_28, local_26);
    unsafe { local_2e._0_4_ = *puVar7 };
    local_2e._4_2_ = (puVar7 + 4);
    local_4e = local_2e;
    pass1_1008_3e94(
        local_2e,
        CONCAT22(unaff_SS, &local_24),
        CONCAT22(unaff_SS, &local_22),
    );
    iVar4 = local_4 + 1;
    _local_c = CONCAT22(local_4 - 1, local_6 - 1);
    iVar5 = local_6 + 1;
    if ((local_4 - 1) < 0) {
        _local_c = (local_6 - 1);
    }
    if (local_22 <= iVar4) {
        iVar4 = local_22 - 1;
    }
    if (local_c < 0) {
        _local_c = _local_c & 0xffff0000;
    }
    if (local_24 <= iVar5) {
        iVar5 = local_24 - 1;
    }
    _local_10 = CONCAT22(iVar4, iVar5);
    zero_list_1008_6c90(local_3a, unaff_SS);
    pass1_1008_6cec(
        CONCAT22(unaff_SS, local_3a),
        local_8,
        _local_10,
        local_8,
        _local_c,
    );
    puVar3 = local_3a;
    uVar6 = extraout_DX_00;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, puVar3), param_2);
    _local_3e = CONCAT22(uVar6, puVar3);
    if ((uVar6 | puVar3) != 0) {
        local_42 = 0;
        local_44 = 0;
        local_46 = local_c;
        while (local_46 <= local_10) {
            local_4e = local_a;
            while (local_4e <= local_e) {
                ppcVar1 = (*_local_3e + 4);
                uVar8 = local_44;
                local_44 = local_44 + 1;
                (**ppcVar1)(0x1030, _local_3e, (_local_3e >> 0x10));
                local_42 = CONCAT22(extraout_DX_01, uVar8);
                local_42._3_1_ = (extraout_DX_01 >> 8);
                if (local_42._3_1_ == '\0') {
                    local_5a = uVar8;
                    if (uVar8 == 7) {
                        pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                        uVar9 = local_20;
                        uVar10 = (local_20 >> 8);
                        uVar11 = (local_20 >> 0x10);
                        uVar8 = 6;
                    } else {
                        if (uVar8 == 8) {
                            pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                            uVar9 = local_20;
                            uVar10 = (local_20 >> 8);
                            uVar11 = (local_20 >> 0x10);
                            uVar8 = 7;
                        } else {
                            if (uVar8 != 9) {}
                            // goto LAB_1020_af1c;
                            pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                            uVar9 = local_20;
                            uVar10 = (local_20 >> 8);
                            uVar11 = (local_20 >> 0x10);
                            uVar8 = 8;
                        }
                    }
                    pass1_1028_87f0(
                        CONCAT22(unaff_SS, &local_17e),
                        0,
                        0,
                        uVar8,
                        param_1,
                        (param_1 >> 0x10),
                        CONCAT22(uVar11, CONCAT11(uVar10, uVar9)),
                        param_2,
                    );
                    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_17e));
                    local_17e = s_1_1050_389a;
                    local_17c = &PTR_LOOP_1050_1008;
                }
                // LAB_1020_af1c:
                local_4e = local_4e + 1;
            }
            local_46 = local_46 + 1;
        }
    }
    return;
}

pub fn pass1_1020_afc4(param_1: u16, param_2: u16, param_1_00: *mut u8, param_2_00: *mut u8) {
    let paVar1: *mut astruct_493;
    let local_EAX_22: *mut u8;
    let mut local_DX_22: u16;
    let mut local_DX_71: u16;
    let mut uVar2: i32;
    let mut local_SS__1: u16;
    let mut uVar3: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    local_EAX_22._0_2_ = &local_a;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        param_2_00,
        local_EAX_22,
        local_SS__1,
    );
    local_6 = local_EAX_22;
    local_DX_71 = (local_EAX_22 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ == '\0') {
        return;
    }
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, local_DX_71);
    uVar3 = pass1_1030_73a8(CONCAT22(local_DX_71, paVar1));
    uVar2 = (uVar3 >> 0x10);
    if ((uVar2 | uVar3) != 0) {
        match (uVar3 + 0xc) {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => return,
            8 => return,
            9 => return,

            _ => return,
        }
    }
    return;
}

pub fn pass1_1020_b0aa(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let local_AX_71: *mut astruct_702;
    let local_AX_192: *mut astruct_703;
    let puVar5: *mut u32;
    let mut in_DX: u16;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar7: u16;
    let mut local_1c: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uVar6: u32;

    uVar7 = (_PTR_LOOP_1050_4e74 >> 0x10);
    if ((_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) == 0) {
        return;
    }
    if ((_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) != -1) {
        if (PTR_LOOP_1050_4e78 == 0x0) {
            local_AX_71 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
            puVar2 = local_AX_71.field_0xc;
            unsafe { ppcVar3 = (*puVar2 + 0x10) };
            puVar5 = puVar2;
            (**ppcVar3)();
            uVar4 = puVar5 & 0xffff | extraout_DX << 0x10;
            local_14 = 0;
            while (local_14 < uVar4) {
                uVar6 = uVar4;
                pass1_1030_1d7c(puVar2, local_14);
                if (((extraout_DX_00 | uVar6) != 0)
                    && ((iVar1 = (uVar6 + 0xc), iVar1 == 0x2a || (iVar1 == 0x2b))))
                {
                    PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 1);
                    break;
                }
                local_14 = local_14 + 1;
            }
            if (PTR_LOOP_1050_4e78 == 0x0) {
                PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 1);
                return;
            }
        }
        pass1_fn_1008_612e(0, (_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) + -1);
    }
    return;
}

pub fn pass1_1020_b1ae(param_1: *mut u8, param_1_00: *mut u8, param_2_00: *mut u8) -> bool {
    let mut in_DX: u16;
    let mut unaff_SS: u16;
    let puVar1: *mut u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    puVar1 = pass1_1030_5b5c(local_6, in_DX);
    unsafe { local_c = *puVar1 };
    uStack8 = (puVar1 + 4);
    pass1_1008_3e94(
        param_1_00,
        CONCAT22(unaff_SS, &local_10),
        CONCAT22(unaff_SS, &local_e),
    );
    pass1_1008_3e94(
        &local_c,
        CONCAT22(unaff_SS, &local_14),
        CONCAT22(unaff_SS, &local_12),
    );
    if ((((0xb < local_e) && (0xb < local_10)) && (local_e < (local_12 - 0xb)))
        && (local_10 < (local_14 - 0xb)))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_b240(param_1: *mut u8, param_2: *mut u8, param_3: *mut u8) {
    let puVar1: *mut u32;
    let paVar2: *mut astruct_493;
    let BVar3: bool;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut uVar7: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    puVar1 = &local_a;
    uVar6 = (param_3 >> 0x10);
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_2,
        (param_3 + 4),
        puVar1,
        unaff_SS,
    );
    unsafe { local_6 = *puVar1 };
    uVar4 = (puVar1 + 2);
    local_22._3_1_ = (local_6 >> 0x18);
    if (local_22._3_1_ != '\0') {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, uVar4);
        uVar7 = pass1_1030_73a8(CONCAT22(uVar4, paVar2));
        uVar5 = (uVar7 >> 0x10);
        if (((uVar5 | uVar7) != 0) && (9 < (uVar7 + 0xc))) {
            return;
        }
    }
    BVar3 = pass1_1020_b1ae(param_1, param_2, *(param_3 + 4));
    if (BVar3 == 0) {
        return;
    }
    return;
}

pub fn pass1_1020_b2da(param_1: *mut u8, param_2: u16, param_3: u32, param_4: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let BVar3: bool;
    let mut iVar4: i32;
    let mut unaff_SS: u16;
    word * *ppwVar5;
    let mut local_1c: u16;
    let mut local_1a: [u8; 6];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        uVar2 = 0x4e6a;
    } else {
        uVar2 = 0x4e6e;
    }
    _local_c = CONCAT22(0x1050, uVar2);
    if (param_2 == 0) {
        local_14 = 0x4e68;
    } else {
        local_14 = 0x4e6c;
    }
    local_12 = &g_alloc_addr_1050_1050;
    _local_10 = CONCAT22(0x1050, local_14);
    loop {
        if (param_2 == 0) {
            ppwVar5 = &PTR_LOOP_1048_4230;
        } else {
            ppwVar5 = 0x10484236;
        }
        pass1_1008_3eb4(
            ppwVar5,
            CONCAT22(unaff_SS, &local_8),
            CONCAT22(unaff_SS, &local_6),
            CONCAT22(unaff_SS, &local_4),
        );
        uVar1 = *_local_c;
        if (uVar1 == 0) {
            _local_6 = CONCAT22(local_4 + *_local_10, local_6 - 1);
        } else {
            if (uVar1 == 1) {
                _local_6 = CONCAT22(local_4 - 1, local_6 + *_local_10);
            } else {
                if (uVar1 == 2) {
                    _local_6 = CONCAT22(local_4 - *_local_10, local_6 - 1);
                }
            }
        }
        pass1_1008_3e54(
            CONCAT22(unaff_SS, local_1a),
            local_8,
            _local_6,
            (_local_6 >> 0x10),
        );
        uVar2 = (param_4 >> 0x10);
        BVar3 = pass1_1020_b1ae(param_1, CONCAT22(unaff_SS, local_1a), *(param_4 + 4));
        if (BVar3 != 0) {
            iVar4 = pass1_1020_b240(param_1, CONCAT22(unaff_SS, local_1a), param_4);
            if (iVar4 != 0) {
                // LAB_1020_b46e:
                pass1_1008_3e76(param_3, local_8, _local_6, (_local_6 >> 0x10));
                return;
            }
        }
        uVar1 = *_local_c;
        if (uVar1 == 0) {
            // LAB_1020_b45e:
            _local_6 = _local_6 & 0xffff0000 | (local_6 + 2);
        } else {
            if (uVar1 == 1) {
                _local_6 = _local_6 & 0xffff | (local_4 + 2) << 0x10;
            } else {
                if (uVar1 == 2) {}
                // goto LAB_1020_b45e;
            }
        }
        pass1_1008_3e76(
            CONCAT22(unaff_SS, local_1a),
            local_8,
            _local_6,
            (_local_6 >> 0x10),
        );
        BVar3 = pass1_1020_b1ae(param_1, CONCAT22(unaff_SS, local_1a), *(param_4 + 4));
        if (BVar3 != 0) {
            iVar4 = pass1_1020_b240(param_1, CONCAT22(unaff_SS, local_1a), param_4);
            if (iVar4 != 0) {}
            // goto LAB_1020_b46e;
        }
        local_1c = *_local_c + 1;
        if (2 < local_1c) {
            local_1c = 0;
            *_local_10 = *_local_10 + 1;
        }
        *_local_c = local_1c;
        pass1_1020_ac6e(param_1, param_2, *_local_10, local_1c);
    }
}

pub fn infinite_loop_1020_b482(param_1: *mut u8, param_2: *mut *mut u8, param_3: *mut u8) {
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let puVar4: *mut u16;
    let puVar5: *mut u32;
    let mut extraout_DX: i32;
    let mut uVar6: i32;
    let mut unaff_SS: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    // ppuVar9: *mut *mut u8;
    let puVar10: *mut u8;
    let mut local_42: u16;
    let mut local_3e: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut uStack32: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: [u8; 2];

    pass1_1030_bcae(local_4, unaff_SS);
    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_16),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        puVar4 = &local_16;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar4));
        _local_1a = CONCAT22(extraout_DX, puVar4);
        uVar6 = extraout_DX | puVar4;
        if (uVar6 == 0) {
            pass1_1020_b240(param_1, param_2, param_3);
            if (puVar4 != 0x0) {
                local_1e = (param_3 + 4);
                local_24 = param_2;
                uStack32 = (param_2 + 4);
                pass1_1008_3eb4(
                    CONCAT22(unaff_SS, &local_24),
                    CONCAT22(unaff_SS, &local_2a),
                    CONCAT22(unaff_SS, &local_28),
                    CONCAT22(unaff_SS, &local_26),
                );
                pass1_1008_3e76(
                    CONCAT22(unaff_SS, &local_24),
                    local_2a,
                    local_28 - 1,
                    local_26 - 1,
                );
                puVar5 = &local_24;
                uVar7 = param_1;
                uVar8 = (param_1 >> 0x10);
                pass1_1020_afc4(uVar7, uVar8, CONCAT22(unaff_SS, puVar5), local_1e);
                if (puVar5 != 0x0) {
                    pass1_1008_3e76(
                        CONCAT22(unaff_SS, &local_24),
                        _local_2a,
                        (_local_2a >> 0x10),
                        local_26 - 1,
                    );
                    puVar5 = &local_24;
                    pass1_1020_afc4(uVar7, uVar8, CONCAT22(unaff_SS, puVar5), local_1e);
                    if (puVar5 != 0x0) {
                        pass1_1008_3e76(
                            CONCAT22(unaff_SS, &local_24),
                            local_2a,
                            local_28 + 1,
                            local_26 - 1,
                        );
                        puVar5 = &local_24;
                        pass1_1020_afc4(uVar7, uVar8, CONCAT22(unaff_SS, puVar5), local_1e);
                        if (puVar5 != 0x0) {
                            pass1_1008_3e76(
                                CONCAT22(unaff_SS, &local_24),
                                local_2a,
                                local_28 - 1,
                                local_26,
                            );
                            puVar5 = &local_24;
                            pass1_1020_afc4(uVar7, uVar8, CONCAT22(unaff_SS, puVar5), local_1e);
                            if (puVar5 != 0x0) {
                                pass1_1008_3e76(
                                    CONCAT22(unaff_SS, &local_24),
                                    local_2a,
                                    local_28 + 1,
                                    local_26,
                                );
                                puVar5 = &local_24;
                                pass1_1020_afc4(uVar7, uVar8, CONCAT22(unaff_SS, puVar5), local_1e);
                                if (puVar5 != 0x0) {
                                    pass1_1008_3e76(
                                        CONCAT22(unaff_SS, &local_24),
                                        local_2a,
                                        local_28 + 1,
                                        local_26 + 1,
                                    );
                                    puVar5 = &local_24;
                                    pass1_1020_afc4(
                                        uVar7,
                                        uVar8,
                                        CONCAT22(unaff_SS, puVar5),
                                        local_1e,
                                    );
                                    if (puVar5 != 0x0) {
                                        pass1_1008_3e76(
                                            CONCAT22(unaff_SS, &local_24),
                                            _local_2a,
                                            (_local_2a >> 0x10),
                                            local_26 + 1,
                                        );
                                        puVar5 = &local_24;
                                        pass1_1020_afc4(
                                            uVar7,
                                            uVar8,
                                            CONCAT22(unaff_SS, puVar5),
                                            local_1e,
                                        );
                                        if (puVar5 != 0x0) {
                                            pass1_1008_3e76(
                                                CONCAT22(unaff_SS, &local_24),
                                                local_2a,
                                                local_28 - 1,
                                                local_26 + 1,
                                            );
                                            puVar5 = &local_24;
                                            pass1_1020_afc4(
                                                uVar7,
                                                uVar8,
                                                CONCAT22(unaff_SS, puVar5),
                                                local_1e,
                                            );
                                            if (puVar5 != 0x0) {
                                                pass1_1008_3e76(
                                                    CONCAT22(unaff_SS, &local_24),
                                                    local_2a,
                                                    local_28 - 2,
                                                    local_26 - 2,
                                                );
                                                puVar5 = &local_24;
                                                pass1_1020_afc4(
                                                    uVar7,
                                                    uVar8,
                                                    CONCAT22(unaff_SS, puVar5),
                                                    local_1e,
                                                );
                                                if (puVar5 != 0x0) {
                                                    pass1_1008_3e76(
                                                        CONCAT22(unaff_SS, &local_24),
                                                        local_2a,
                                                        local_28 + 2,
                                                        local_26 - 2,
                                                    );
                                                    puVar5 = &local_24;
                                                    pass1_1020_afc4(
                                                        uVar7,
                                                        uVar8,
                                                        CONCAT22(unaff_SS, puVar5),
                                                        local_1e,
                                                    );
                                                    if (puVar5 != 0x0) {
                                                        pass1_1008_3e76(
                                                            CONCAT22(unaff_SS, &local_24),
                                                            local_2a,
                                                            local_28 - 2,
                                                            local_26 + 2,
                                                        );
                                                        puVar5 = &local_24;
                                                        pass1_1020_afc4(
                                                            uVar7,
                                                            uVar8,
                                                            CONCAT22(unaff_SS, puVar5),
                                                            local_1e,
                                                        );
                                                        if (puVar5 != 0x0) {
                                                            pass1_1008_3e76(
                                                                CONCAT22(unaff_SS, &local_24),
                                                                local_2a,
                                                                local_28 + 2,
                                                                local_26 + 2,
                                                            );
                                                            puVar5 = &local_24;
                                                            pass1_1020_afc4(
                                                                uVar7,
                                                                uVar8,
                                                                CONCAT22(unaff_SS, puVar5),
                                                                local_1e,
                                                            );
                                                            if (puVar5 != 0x0) {
                                                                pass1_1008_3e76(
                                                                    CONCAT22(unaff_SS, &local_24),
                                                                    local_2a,
                                                                    local_28 - 1,
                                                                    local_26 + 2,
                                                                );
                                                                puVar5 = &local_24;
                                                                pass1_1020_afc4(
                                                                    uVar7,
                                                                    uVar8,
                                                                    CONCAT22(unaff_SS, puVar5),
                                                                    local_1e,
                                                                );
                                                                if (puVar5 != 0x0) {
                                                                    pass1_1008_3e76(
                                                                        CONCAT22(
                                                                            unaff_SS, &local_24,
                                                                        ),
                                                                        local_2a,
                                                                        local_28 - 1,
                                                                        local_26 + 3,
                                                                    );
                                                                    puVar5 = &local_24;
                                                                    pass1_1020_afc4(
                                                                        uVar7,
                                                                        uVar8,
                                                                        CONCAT22(unaff_SS, puVar5),
                                                                        local_1e,
                                                                    );
                                                                    if (puVar5 != 0x0) {
                                                                        local_2e = 3;
                                                                        while (true) {
                                                                            if (9 < local_2e) {
                                                                                return;
                                                                            }
                                                                            pass1_1008_3e76(
                                                                                CONCAT22(
                                                                                    unaff_SS,
                                                                                    &local_24,
                                                                                ),
                                                                                0,
                                                                                local_28 - local_2e,
                                                                                local_26,
                                                                            );
                                                                            puVar5 = &local_24;
                                                                            pass1_1020_afc4(
                                                                                uVar7,
                                                                                uVar8,
                                                                                CONCAT22(
                                                                                    unaff_SS,
                                                                                    puVar5,
                                                                                ),
                                                                                local_1e,
                                                                            );
                                                                            if (puVar5 == 0x0) {
                                                                                break;
                                                                            }
                                                                            local_2e = local_2e + 1;
                                                                        }
                                                                        return;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return;
        }
        uVar1 = (puVar4 + 8);
        ppuVar9 = param_2;
        puVar10 = param_3;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        puVar3 = local_4;
        pass1_1030_bcbc(
            CONCAT22(unaff_SS, puVar3),
            CONCAT22(uVar6, paVar2),
            ppuVar9,
            puVar10,
        );
        if (puVar3 < 0) {
            break;
        }
        if (puVar3 < 0x65) {
            return;
        }
    }
    return;
}

pub fn call_infinite_loop_1020_b872(param_1: *mut u8, param_2: *mut u8) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut unaff_SS: u16;
    let puVar4: *mut u32;
    let uVar5: u8;
    let mut uVar6: u16;
    let mut local_13a: u16;
    let mut local_138: u16;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    uVar6 = (param_2 >> 0x10);
    puVar4 = pass1_1030_5b5c(param_2, uVar6);
    unsafe { local_8 = *puVar4 };
    uStack4 = (puVar4 + 4);
    uVar5 = (unaff_SS >> 8);
    pass1_1008_3e94(
        &local_8,
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    uVar1 = pass1_fn_1008_612e(10, local_a - 10);
    uVar2 = pass1_fn_1008_612e(10, local_c - 10);
    pass1_1008_3e54(
        CONCAT13(uVar5, CONCAT12(unaff_SS, &local_12)),
        0,
        uVar2,
        uVar1,
    );
    while (true) {
        iVar3 = infinite_loop_1020_b482(param_1, CONCAT22(unaff_SS, &local_12), param_2);
        if (iVar3 != 0) {
            break;
        }
        uVar1 = pass1_fn_1008_612e(10, local_a - 10);
        uVar2 = pass1_fn_1008_612e(10, local_c - 10);
        pass1_1008_3e76(
            CONCAT13(uVar5, CONCAT12(unaff_SS, &local_12)),
            0,
            uVar2,
            uVar1,
        );
    }
    pass1_1028_8888(
        CONCAT22(unaff_SS, &local_136),
        0,
        10,
        &local_12,
        unaff_SS,
        0x8000002,
        0,
        (param_2 + 4),
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_136));
    pass1_1020_b97e(param_1, (param_1 >> 0x10), 1);
    return;
}

pub fn pass1_1020_b97e(param_1: u16, param_2: u16, param_1_00: i32) {
    let paVar1: *mut astruct_493;
    let mut in_DX: u16;
    let mut unaff_SS: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
    _PTR_LOOP_1050_4e70 = CONCAT22(in_DX, paVar1);
    local_6 = &paVar1.field_0x10;
    local_a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, (local_6 >> 0x10));
    modify_list_1008_3f62(&PTR_LOOP_1048_4230, CONCAT22(in_DX, local_a + 0xc));
    pass1_1008_3e94(
        &PTR_LOOP_1050_4230,
        CONCAT22(unaff_SS, &local_e),
        CONCAT22(unaff_SS, &local_c),
    );
    if (param_1_00 == 0) {
        pass1_1008_3e76(&PTR_LOOP_1048_4230, 0, local_e + 1, local_c - 1);
        pass1_1008_3e94(
            &PTR_LOOP_1050_4230,
            CONCAT22(unaff_SS, &local_e),
            CONCAT22(unaff_SS, &local_c),
        );
    }
    pass1_1008_3e76(0x10484236, 1, local_e - 2, local_c);
    return;
}

pub fn pass1_1020_ba3e(param_1: *mut astruct_704, param_2: u16, param_3: u16) {
    let local_BX_3: *mut astruct_704;
    let local_ES_3: *mut astruct_704;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    param_1 = 0;
    local_BX_3.field_0x4 = 0;
    local_BX_3.field_0x6 = param_3;
    local_BX_3.field_0x8 = param_2;
    if (local_BX_3.field_0x6 == 0) {
        local_BX_3.field_0x6 = 5;
    }
    call_alloc_mem_fn_1020_bcc4(param_1);
    return;
}

pub fn pass1_1020_ba7e(param_1: *mut *mut astruct_44) {
    error_check_1000_17ce(param_1);
    return;
}

pub fn infinite_loop_1020_ba94(param_1: *mut long) {
    let puVar1: *mut u32;
    let mut local_8: u16;
    let mut local_6: u32;

    let param_1_val = unsafe { *param_1 };
    if (param_1_val == 0) {
        return;
    }
    local_8 = 0;
    while (true) {
        puVar1 = (param_1 + 4);
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val < local_8 || pu_var1_val == local_8) {
            break;
        }
        local_8 = local_8 + 1;
    }
    return;
}

pub fn pass1_1020_bae6(param_1: u16, param_2: u32) {
    let mut in_EAX: u32;
    let local_DXAX_13: *mut u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1020_bc92(param_1, param_2);
    if ((local_DXAX_13._2_2_ | in_EAX) != 0) {
        return in_EAX & 0xffff0000 | *(in_EAX & 0xffff | local_DXAX_13._2_2_ << 0x10);
    }
    return in_EAX & 0xffff0000;
}

pub fn pass1_1020_bb16(param_1: *mut *mut u8, param_2: u32, param_3: u32, param_4: u16) {
    if ((param_1 + 4) < param_4) {
        param_3 = 0;
        param_2 = 0;
        return;
    }
    param_3 = (param_4 * 6 + param_1 + 4);
    param_2 = (param_1 + param_4 * 6);
    return;
}

pub fn pass1_1020_bb70(param_1: *mut u8, param_2: u16, param_3: *mut u8) {
    pass1_1020_bba4(param_1, 1, param_2, param_3);
    return;
}

pub fn pass1_1020_bb8a(param_1: *mut u8, param_2: u16, param_3: *mut u8) {
    pass1_1020_bba4(param_1, 0, param_2, param_3);
    return;
}

pub fn pass1_1020_bba4(param_1: *mut u8, param_2: u16, param_3: u16, param_4: *mut u8) {
    let mut uVar1: u16;
    let mut bVar2: bool;
    let puVar3: *mut u32;
    let mut local_6: u16;
    let mut local_4: u16;

    puVar3 = pass1_1020_bc92(param_1, param_4._2_2_);
    uVar1 = (puVar3 >> 0x10);
    if (puVar3 == 0x0) {
        puVar3 = pass1_1020_bc92(param_1, 0);
        if (puVar3 == 0x0) {
            call_alloc_mem_fn_1020_bcc4(param_1);
            puVar3 = pass1_1020_bc92(param_1, 0);
            if (puVar3 == 0x0) {
                return 0;
            }
            (puVar3 + 4) = param_4._2_2_;
        } else {
            (puVar3 + 4) = param_4._2_2_;
        }
        uVar1 = (puVar3 >> 0x10);
        if (param_2 != 0) {
            unsafe {
                bVar2 = CARRY2(*puVar3, param_3);
                param_3 = *puVar3 + param_3;
            }
            param_4._0_2_ = (puVar3 + 2) + param_4 + bVar2;
        }
        unsafe {
            *puVar3 = param_3;
            (puVar3 + 2) = param_4
        };
        pass1_1020_bc72(param_1);
    } else {
        if (param_2 != 0) {
            unsafe {
                bVar2 = CARRY2(*puVar3, param_3);
                param_3 = *puVar3 + param_3;
            }
            param_4._0_2_ = (puVar3 + 2) + param_4 + bVar2;
        }
        unsafe {
            *puVar3 = param_3;
            (puVar3 + 2) = param_4;
        }
    }
    return 1;
}

pub fn pass1_1020_bc72(param_1: *mut u16) {
    let param_1_val = unsafe { *param_1 };
    pass1_1000_4aea(param_1_val, (param_1 + 2), 6, 0xbd6c, 0x1020);
    return;
}

pub fn pass1_1020_bc92(param_1: *mut u16, param_2: u16) {
    let mut u_var1: u32;
    let mut unaff_SS: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_8 = param_2;
    uVar1 = (param_1 + 2);
    let param_1_val = unsafe { *param_1 };
    pass1_1000_49c6(
        local_c,
        unaff_SS,
        param_1_val,
        uVar1,
        (uVar1 >> 0x10),
        6,
        0xbd6c,
    );
    return;
}

pub fn call_alloc_mem_fn_1020_bcc4(in_struct_1: *mut astruct_705) {
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut uVar3: i32;
    let extraout_DX: *mut u16;
    let local_struct_1: *mut astruct_705;
    let local_struct_1_hi: *mut astruct_705;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x4 == 0) {
        g_u16_ptr_1050_5f2e = 0x0;
        iVar2 = local_struct_1.field_0x6;
    } else {
        uVar3 = local_struct_1.field_0x4;
        puVar1 = &local_struct_1.field_0x8;
        unsafe {
            iVar2 = uVar3 + *puVar1;
            g_u16_ptr_1050_5f2e = CARRY2(uVar3, *puVar1)
        };
    }
    if (g_u16_ptr_1050_5f2e == 0x0) {
        if (in_struct_1 == 0) {
            if (__g_astruct_94_ptr_1 == 0) {
                struct_fn_1000_160a();
            } else {
            }
            uVar3 = iVar2 * 6;
            alloc_mem_1000_1708(uVar3, 0, 1);
        } else {
            uVar3 = iVar2 * 6;
            alloc_mem_1000_0ed4(1, uVar3, 0, in_struct_1);
            g_u16_ptr_1050_5f2e = extraout_DX;
        }
        _local_c = CONCAT22(g_u16_ptr_1050_5f2e, uVar3);
        if ((g_u16_ptr_1050_5f2e | uVar3) != 0) {
            local_struct_1.field_0x4 = iVar2;
            in_struct_1 = _local_c;
            pass1_1020_bc72((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
        }
    }
    return;
}

pub fn pass1_1020_bd6c(param_1: *mut u8, param_2: *mut u8) -> i32 {
    return (param_1 + 4) - (param_2 + 4);
}

pub fn pass1_1020_a43e(param_1: *mut u16) {
    let mut in_stack_0000fff2: u32;

    unsafe {
        *param_1 = 0xba36;
        (param_1 + 2) = 0x1020;
    }
    if (_PTR_LOOP_1050_4e74 != 0) {
        return param_1;
    }
    process_struct_1010_20ba(
        _g_astruct_372_1050_0ed0,
        CONCAT22((in_stack_0000fff2 >> 0x10), 2),
    );
    if ((0 < u16_1050_13ae) && (!SBORROW2(u16_1050_13ae, 1))) {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
            PTR_LOOP_1050_4e74 = 0x44b4;
            // goto LAB_1020_a482;
        }
        if (u16_1050_13ae == 4) {
            PTR_LOOP_1050_4e74 = 0x4b2c;
            // goto LAB_1020_a482;
        }
    }
    PTR_LOOP_1050_4e74 = 0x47f0;
    // LAB_1020_a482:
    _PTR_LOOP_1050_4e74 = CONCAT22(0x1050, PTR_LOOP_1050_4e74);
    return param_1;
}

pub fn pass1_1020_a49a(param_1: *mut u16, param_2: u32, param_3: u16) {
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut in_stack_0000feba: u16;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 =
        process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000feba, 0x2f));
    local_c = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    local_e = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    if (param_2 != 0) {
        uVar1 = (param_2 >> 0x10);
        if ((param_2 + 1) == 0) {
            uVar2 = SUB42(&PTR_LOOP_1050_4230, 0);
        } else {
            uVar2 = SUB42(s_dib_1050_4234 + 2, 0);
        }
        pass1_1008_3f32(param_2, uVar2, &PTR_LOOP_1050_1048);
        pass1_1028_87f0(
            CONCAT22(unaff_SS, &local_136),
            0,
            0,
            param_3,
            param_2,
            uVar1,
            (_PTR_LOOP_1050_4e70 + 4),
            local_a,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_136));
        return;
    }
    pass1_1020_abc0(param_1, param_3, local_e, local_c);
    return;
}

pub fn pass1_1020_a54c(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut u_var1: u32;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_140: u16;
    let mut local_13e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 6];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
    local_c = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    local_e = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    local_14._0_4_ = _PTR_LOOP_1048_4230;
    local_14._4_2_ = PTR_LOOP_1048_4234;
    local_1c = local_14;
    pass1_1008_3e94(
        local_14,
        CONCAT22(unaff_SS, &local_18),
        CONCAT22(unaff_SS, &local_16),
    );
    if ((param_1_00 < 0) || (5 < param_1_00)) {
        pass1_1008_3e76(CONCAT22(unaff_SS, local_14), 0, local_18 - 9, local_16);
        uVar5 = local_a;
        uVar6 = (local_a >> 0x10);
        uVar1 = (_PTR_LOOP_1050_4e70 + 4);
        uVar3 = uVar1;
        uVar4 = (uVar1 >> 0x10);
        uVar2 = 0x14;
    } else {
        pass1_1008_3e76(
            CONCAT22(unaff_SS, local_14),
            0,
            (local_18 - param_1_00) - 3,
            local_16,
        );
        uVar5 = local_a;
        uVar6 = (local_a >> 0x10);
        uVar1 = (_PTR_LOOP_1050_4e70 + 4);
        uVar3 = uVar1;
        uVar4 = (uVar1 >> 0x10);
        uVar2 = 0x7b;
    }
    pass1_1028_87f0(
        CONCAT22(unaff_SS, &local_140),
        0,
        0,
        uVar2,
        local_14,
        unaff_SS,
        CONCAT22(uVar4, uVar3),
        CONCAT22(uVar6, uVar5),
    );
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_140));
    return;
}

pub fn pass1_1020_8e6c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_8bae(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_8eaa(param_1: *mut astruct_393) {
    let mut uVar1: i32;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut extraout_DX: u16;
    let local_BX_17: *mut astruct_695;
    let mut unaff_SI: u16;
    let mut uVar4: i32;
    let mut unaff_SS: u16;
    let ppVar5: *mut pass1_struct_1;

    process_struct_1020_847a(param_1, 0x25);
    uVar4 = (param_1 >> 0x10);
    local_BX_17 = param_1;
    &local_BX_17.field_0x16 = 0;
    local_BX_17.field_0xaa = 0;
    uVar1 = &local_BX_17.field_0xae;
    zero_list_1008_3e38((param_1 & 0xffff0000 | uVar1));
    &local_BX_17.field_0xb4 = 0;
    local_BX_17.field_0xb8 = 0xffff;
    &local_BX_17.field_0xba = 0;
    param_1.field_0x0 = 0x9204;
    local_BX_17.field_0x2 = 0x1020;
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x29));
    uVar3 = (ppVar5 >> 0x10);
    uVar2 = ppVar5;
    local_BX_17.field_0x16 = uVar2;
    local_BX_17.field_0x18 = uVar3;
    pass1_1018_2646(local_BX_17.field_0x16, uVar3, param_1 & 0xffff0000 | uVar1);
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1ce);
    local_BX_17.field_0xb4 = uVar2;
    local_BX_17.field_0xb6 = extraout_DX;
    pass1_1020_8712(
        (param_1 & 0xffff | uVar4 << 0x10),
        &stack0xfff6,
        unaff_SS,
        CONCAT22(extraout_DX, local_BX_17.field_0xb4),
        param_1 & 0xffff0000 | uVar1,
    );
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    local_BX_17.field_0xba = ppVar5;
    local_BX_17.field_0xbc = (ppVar5 >> 0x10);
    return;
}

pub fn pass1_1020_8f74(param_1: *mut astruct_44) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_BX_5: *mut astruct_44;
    let local_ES_5: *mut astruct_44;
    let fn_ptr_1: fn();

    local_ES_5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = 0x9204;
    local_BX_5.ptr_a_hi = 0x1020;
    puVar1 = local_BX_5.field_0xb4;
    uVar2 = local_BX_5.field_0xb6;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
            (**fn_ptr_1)()
        };
    }
    pass1_1020_8556(param_1);
    return;
}

pub fn invalidate_rect_1020_8fb4(param_1: u32) {
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut in_AX: u16;
    let rect: *mut RECT16;
    let mut uVar3: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut hwnd: i32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_16: u16;
    let mut local_e: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar2 = (iVar4 + 0xba);
    if ((uVar2 + 0x1e) != 0) {
        pass1_1018_2862((iVar4 + 0x16));
        (iVar4 + 0xaa) = in_AX;
        (iVar4 + 0xac) = extraout_DX;
        if ((extraout_DX | (iVar4 + 0xaa)) != 0) {
            uVar2 = (iVar4 + 0xaa);
            iVar1 = (uVar2 + 10);
            local_8 = 0;
            while (local_8 < iVar1) {
                uVar3 = SEXT24(local_8);
                bad_func_1008_8fc4(*(iVar4 + 0xaa), uVar3);
                rect = uVar3;
                if ((((extraout_DX_00 | rect) != 0) && (9 < rect[5].bottom))
                    && (
                        pass1_1008_8b20((uVar3 & 0xffff | extraout_DX_00 << 0x10)),
                        (hwnd | rect) != 0,
                    ))
                {
                    InvalidateRect16(0, rect, hwnd);
                }
                local_8 = local_8 + 1;
            }
        }
    }
    return;
}

pub fn pass1_1020_9068(in_struct_1: *mut astruct_696) {
    let mut iVar1: i32;
    let mut uVar2: u32;
    let paVar3: *mut astruct_318;
    let mut uVar4: u32;
    let mut uVar5: i32;
    let mut uVar6: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: u16;
    let local_struct_1: *mut astruct_696;
    let mut iVar7: i32;
    let local_struct_1_hi: *mut astruct_696;
    let mut uVar8: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar3 = local_struct_1.struct318_ptr_0x16;
    uVar2 = (paVar3 + 10);
    uVar6 = uVar2;
    pass1_1018_280c(local_struct_1.struct318_ptr_0x16);
    local_struct_1.field_0xaa = uVar6;
    &local_struct_1.field_0xac = extraout_DX;
    uVar5 = extraout_DX | local_struct_1.field_0xaa;
    if (uVar5 == 0) {
        pass1_1018_2862(local_struct_1.struct318_ptr_0x16);
        local_struct_1.field_0xaa = uVar5;
        &local_struct_1.field_0xac = extraout_DX_00;
    }
    if ((&local_struct_1.field_0xac | local_struct_1.field_0xaa) != 0) {
        pass1_1020_915a((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
        pass1_1008_4480(
            uVar2,
            (in_struct_1 & 0xffff0000 | &local_struct_1.field_0xae),
            local_struct_1.field_0xb4,
        );
        fn_ptr_1 = (in_struct_1 + 0x10);
        (**fn_ptr_1)();
        uVar4 = &local_struct_1.field_0xaa;
        iVar1 = (uVar4 + 10);
        local_a = 0;
        while (local_a < iVar1) {
            uVar6 = SEXT24(local_a);
            bad_func_1008_8fc4(*&local_struct_1.field_0xaa, uVar6);
            uVar5 = uVar6;
            if ((extraout_DX_01 | uVar5) != 0) {
                pass1_1008_8c4e((uVar6 & 0xffff | extraout_DX_01 << 0x10), uVar2);
                uVar4 = local_struct_1.field_0xc;
                uVar8 = (uVar4 >> 0x10);
                iVar7 = uVar4;
                (iVar7 + local_a * 4) = uVar5;
                (iVar7 + local_a * 4 + 2) = extraout_DX_02;
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub fn pass1_1020_915a(param_1: *mut astruct_697) {
    let mut iVar1: i32;
    let local_struct_1: *mut astruct_697;
    let local_struct_1_hi: *mut astruct_697;
    let ppVar2: *mut pass1_struct_1;
    let mut uVar3: u32;
    let mut local_c: u16;
    let pcStack14: *mut char;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pcStack14 = CONCAT22(local_c, 0x2f);
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, pcStack14);
    iVar1 = (ppVar2 + 0x1e);
    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0xb8 != iVar1) {
        local_c = 0x1ce;
        if (iVar1 == 1) {
            local_c = 0x1cf;
        } else {
            if (iVar1 == 2) {
                local_c = 0x1d0;
            } else {
                if (iVar1 == 3) {
                    local_c = 0x1d1;
                } else {
                    if (iVar1 == 4) {
                        local_c = 0x1d2;
                    }
                }
            }
        }
        uVar3 = mixed_fn_1010_830a(_g_struct_73_1050_14cc, local_c);
        local_struct_1.field_0xb4 = uVar3;
        local_struct_1.field_0xb6 = (uVar3 >> 0x10);
        local_struct_1.field_0xb8 = iVar1;
    }
    return;
}

pub fn pass1_1020_91de(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_8f74(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_8296(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    process_struct_1020_808e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_8360(in_struct_1: *mut astruct_680) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut uvar3: u16;
    let mut unaff_SI: u16;
    let ppVar4: *mut pass1_struct_1;
    let local_struct_1: *mut astruct_680;
    let local_struct_1_hi: *mut astruct_680;
    let mut local_6: u32;

    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    process_struct_1020_847a(in_struct_1, 1);
    zero_list_1008_3e38((in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16));
    &local_struct_1.field_0x1c = 0;
    in_struct_1.field_0x0 = 0x8462;
    local_struct_1.field_0x2 = 0x1020;
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x29));
    uVar3 = (ppVar4 >> 0x10);
    local_struct_1.field_0x1c = ppVar4;
    &local_struct_1.field_0x1e = uVar3;
    pass1_1018_26f8(
        local_struct_1.field_0x1c,
        uVar3,
        in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16,
    );
    uVar2 = &local_struct_1.field_0x1c;
    uVar1 = local_struct_1.field_0x8;
    pass1_1020_8712(
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        uVar1,
        (uVar1 >> 0x10),
        (uVar2 + 0x2a),
        in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16,
    );
    return;
}

pub fn pass1_1020_83f8(param_1: *mut astruct_417) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let local_struct_1: *mut astruct_417;
    let local_struct_1_hi: *mut astruct_417;
    let mut local_6: u32;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (&local_struct_1.field_0x4 != 0) {
        uVar1 = &local_struct_1.field_0x1c;
        uVar2 = &local_struct_1.field_0x1c;
        pass1_1008_4480(
            (uVar1 + 10),
            (param_1 & 0xffff0000 | &local_struct_1.field_0x16),
            (uVar2 + 0x2a),
        );
    }
    return;
}

pub fn pass1_1020_843c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_847a(param_1: *mut astruct_393, param_2: u16) {
    let mut uVar1: u16;
    let struct_a: *mut astruct_199;
    let struct_a_00: *mut astruct_199;
    let local_struct_1: *mut astruct_393;
    let mut unaff_SI: u16;
    let local_struct_1_hi: *mut astruct_393;
    let ppVar2: *mut pass1_struct_1;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1._0_2_ = param_1;
    param_1.field_0x0 = s_1_1050_389a;
    (local_struct_1 + 2) = &PTR_LOOP_1050_1008;
    (local_struct_1 + 4) = 0;
    (local_struct_1 + 6) = param_2;
    (local_struct_1 + 8) = 0;
    (local_struct_1 + 0xc) = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | (local_struct_1 + 0x10)));
    param_1.field_0x0 = 0x87aa;
    (local_struct_1 + 2) = 0x1020;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x48));
    modify_list_1008_3f62(
        (param_1 & 0xffff0000 | (local_struct_1 + 0x10)),
        ppVar2 & 0xffff0000 | (ppVar2 + 0xe),
    );
    uVar1 = (local_struct_1 + 6) << 3;
    struct_a_00 = struct_a;
    process_struct_1000_179c(uVar1, struct_a);
    (local_struct_1 + 8) = uVar1;
    (local_struct_1 + 10) = struct_a_00;
    uVar1 = (local_struct_1 + 6) << 2;
    process_struct_1000_179c(uVar1, struct_a_00);
    (local_struct_1 + 0xc) = uVar1;
    (local_struct_1 + 0xe) = struct_a_00;
    pass1_1000_4906((local_struct_1 + 8), 0, (local_struct_1 + 6) << 3);
    pass1_1000_4906((local_struct_1 + 0xc), 0, (local_struct_1 + 6) << 2);
    return;
}

pub fn pass1_1020_8556(param_1: *mut astruct_44) {
    let puVar1: *mut u16;
    let mut uVar2: i32;
    let in_struct_1: *mut astruct_44;
    let mut uVar3: u32;
    let local_BX_5: *mut astruct_684;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_c: u16;
    let mut local_6: u32;
    let local_struct_1: *mut astruct_44;

    uVar6 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = 0x87aa;
    local_BX_5.field_0x2 = 0x1020;
    error_check_1000_17ce(local_BX_5.field_0x8);
    if ((&local_BX_5.field_0xe | local_BX_5.field_0xc) != 0) {
        local_c = 0;
        while (true) {
            puVar1 = &local_BX_5.field_0x6;
            let pu_var1_val = unsafe { *puVar1 };
            if (pu_var1_val == local_c || pu_var1_val < local_c) {
                break;
            }
            iVar5 = local_c * 4;
            uVar3 = &local_BX_5.field_0xc;
            uVar7 = (uVar3 >> 0x10);
            iVar4 = uVar3;
            if ((iVar4 + iVar5) != 0) {
                in_struct_1 = (iVar4 + iVar5);
                uVar2 = (iVar4 + iVar5 + 2);
                if ((uVar2 | in_struct_1) != 0) {
                    pass1_1008_5118((in_struct_1 & 0xffff | uVar2 << 0x10));
                    error_check_1000_17ce(in_struct_1);
                }
            }
            local_c = local_c + 1;
        }
        error_check_1000_17ce(&local_BX_5.field_0xc);
    }
    param_1.ptr_a_lo = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1020_85f6(param_1: *mut u8) {
    let puVar1: *mut u16;
    let mut uVar2: i32;
    let in_struct_1: *mut astruct_44;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let local_BX_85: *mut u8;
    let mut uVar5: u16;
    let local_ES_85: *mut u8;
    let mut local_8: u32;
    let mut local_4: u16;
    let temp_5f20445f9b: *mut astruct_44;

    local_4 = 0;
    while (true) {
        local_ES_85 = (param_1 >> 0x10);
        local_BX_85 = param_1;
        puVar1 = (local_BX_85 + 6);
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        uVar3 = (local_BX_85 + 0xc);
        uVar5 = (uVar3 >> 0x10);
        iVar4 = uVar3;
        in_struct_1 = (iVar4 + local_4 * 4);
        uVar2 = (iVar4 + local_4 * 4 + 2);
        if ((uVar2 | in_struct_1) != 0) {
            pass1_1008_5118((in_struct_1 & 0xffff | uVar2 << 0x10));
            error_check_1000_17ce(in_struct_1);
        }
        uVar3 = (local_BX_85 + 0xc);
        (uVar3 + local_4 * 4) = 0;
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_865a(param_1: *mut astruct_681) {
    let puVar1: *mut u16;
    let in_struct_1: *mut astruct_44;
    let mut uVar2: u32;
    let local_BX_39: *mut astruct_681;
    let local_BX_53: *mut astruct_682;
    let mut iVar3: i32;
    let local_SI_50: *mut astruct_683;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut local_8: u32;
    let mut local_4: u16;
    let local_struct_1: *mut astruct_44;
    let local_struct_1_1: *mut astruct_44;

    local_4 = 0;
    while (true) {
        uVar4 = (param_1 >> 0x10);
        local_BX_39 = param_1;
        puVar1 = &local_BX_39.field_0x6;
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        local_SI_50 = (local_4 * 4);
        uVar2 = local_BX_39.field_0xc;
        uVar5 = (uVar2 >> 0x10);
        local_BX_53 = uVar2;
        if ((local_BX_53 + local_SI_50) != 0) {
            pass1_1008_5236((local_BX_53 + local_SI_50));
            uVar2 = local_BX_39.field_0xc;
            uVar5 = (uVar2 >> 0x10);
            iVar3 = uVar2;
            in_struct_1 = (local_SI_50 + iVar3);
            local_struct_1 = (local_SI_50 + iVar3 + 2);
            if ((local_struct_1 | in_struct_1) != 0) {
                pass1_1008_5118((in_struct_1 & 0xffff | ZEXT24(local_struct_1) << 0x10));
                error_check_1000_17ce(in_struct_1);
            }
            uVar2 = local_BX_39.field_0xc;
            (uVar2 + local_4 * 4) = 0;
        }
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_86d8(param_1: *mut u8) {
    let puVar1: *mut u16;
    let local_BX_17: *mut astruct_685;
    let mut uVar2: u16;
    let mut local_4: u16;
    let mut temp_5f84f21f47: u32;

    local_4 = 0;
    while (true) {
        uVar2 = (param_1 >> 0x10);
        puVar1 = (param_1 + 6);
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        temp_5f84f21f47 = (param_1 + 0xc);
        uVar2 = (temp_5f84f21f47 >> 0x10);
        local_BX_17 = temp_5f84f21f47;
        if ((local_BX_17 + local_4 * 4) != 0) {
            pass1_1008_5236((local_BX_17 + local_4 * 4));
        }
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_8712(
    param_1: *mut astruct_393,
    param_2: *mut astruct_393,
    param_3: u16,
    param_4: u32,
    param_5: u32,
) -> i32 {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3f32(param_5, param_1 & 0xffff0000 | (param_1 + 0x10));
    uVar2 = process_struct_1008_4772(param_4);
    uVar1 = (uVar2 >> 0x10);
    pass1_1008_3e94(
        param_5,
        CONCAT22(param_3, &param_2.u16_0x2),
        CONCAT22(param_3, param_2),
    );
    param_2.u16_0x4 = (uVar2 + 4) + *_param_2;
    param_2.u16_0x6 = (uVar2 + 8) + param_2.u16_0x2;
    return;
}

pub fn pass1_1020_8784(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_87c2(param_1: *mut astruct_393) {
    let mut u_var1: u32;
    let local_AX_25: *mut astruct_393;
    let mut unaff_SI: u16;
    let mut uVar2: i32;
    let mut unaff_SS: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut local_16: u32;
    let mut local_a: u16;
    let local_8: *mut astruct_393;
    let mut local_4: u16;

    process_struct_1020_847a(param_1, 4);
    local_4 = 4;
    local_AX_25 = param_1;
    local_AX_25 = (&local_AX_25.field_0x14 + 2);
    local_8 = (param_1 & 0xffff0000 | ZEXT24(local_AX_25));
    while {
        zero_list_1008_3e38(local_8);
        local_8 = (local_8 & 0xffff0000 | (local_8 + 6));
        local_4 = local_4 - 1;
        local_4 != 0
    } {}
    uVar2 = (param_1 >> 0x10);
    &local_AX_25.field_0x2e = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_AX_25.field_0x32));
    &local_AX_25.field_0x38 = 0;
    param_1.field_0x0 = 0x8a84;
    local_AX_25.u16_0x2 = 0x1020;
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x29));
    &local_AX_25.field_0x2e = ppVar3;
    &local_AX_25.field_0x30 = (ppVar3 >> 0x10);
    local_a = 0;
    while {
        uVar1 = &local_AX_25.field_0x2e;
        pass1_1018_26d8(
            uVar1,
            (uVar1 >> 0x10),
            local_a,
            param_1 & 0xffff0000 | (&local_AX_25.field_0x14 + local_a * 6 + 2),
        );
        uVar1 = &local_AX_25.field_0x2e;
        pass1_1020_8712(
            (param_1 & 0xffff | uVar2 << 0x10),
            (&local_AX_25.field_0x8 + local_a * 8),
            &local_AX_25.field_0xa,
            (uVar1 + 0x2e + local_a * 4),
            param_1 & 0xffff0000 | (&local_AX_25.field_0x14 + local_a * 6 + 2),
        );
        local_a = local_a + 1;
        local_a < 4
    } {}
    uVar1 = &local_AX_25.field_0x2e;
    pass1_1018_2548(
        uVar1,
        (uVar1 >> 0x10),
        param_1 & 0xffff0000 | &local_AX_25.field_0x32,
    );
    uVar1 = &local_AX_25.field_0x2e;
    &local_AX_25.field_0x38 = (uVar1 + 0x6e);
    pass1_1020_8712(
        (param_1 & 0xffff | uVar2 << 0x10),
        &stack0xffee,
        unaff_SS,
        &local_AX_25.field_0x38,
        param_1 & 0xffff0000 | &local_AX_25.field_0x32,
    );
    return;
}

pub fn pass1_1020_8908(param_1: *mut astruct_690, param_2: u32) {
    let in_struct_104_ptr: *mut astruct_104;
    let mut u_var1: u32;
    let mut uVar2: i32;
    let local_DX_188: *mut astruct_692;
    let paVar3: *mut astruct_692;
    let mut extraout_DX: u16;
    let mut iVar4: i32;
    let local_BX_151: *mut astruct_690;
    let local_BX_294: *mut astruct_688;
    let mut iVar5: i32;
    let local_SI_209: *mut astruct_689;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let local_4: *mut astruct_687;
    let mut temp_7ffdca9fab4: i32;

    local_4 = 0x0;
    while (
        local_BX_151 = param_1,
        uVar6 = (param_1 >> 0x10),
        local_4 < 4,
    ) {
        if (local_BX_151.field_0x4 == 0) {
            uVar1 = local_BX_151.field_0xc;
            uVar6 = (uVar1 >> 0x10);
            iVar4 = uVar1;
            iVar5 = local_4 * 4;
            if (((iVar4 + iVar5 + 2) | (iVar4 + iVar5)) != 0) {
                pass1_1008_5236((iVar4 + iVar5));
            }
        } else {
            uVar1 = local_BX_151.field_0x2e;
            in_struct_104_ptr = (uVar1 + 0x2e + local_4 * 4);
            uVar8 = process_struct_1008_4772(in_struct_104_ptr);
            local_DX_188 = (uVar8 >> 0x10);
            temp_7ffdca9fab4 = uVar8;
            uVar1 = local_BX_151.field_0xc;
            local_SI_209 = (local_4 * 4);
            if ((&local_SI_209.field_0x0 + uVar1) == 0) {
                paVar3 = local_DX_188;
                uVar2 = temp_7ffdca9fab4;
                process_struct_1000_179c(0x14, local_DX_188);
                _local_1c = CONCAT22(paVar3, uVar2);
                if ((paVar3 | uVar2) == 0) {
                    uVar1 = local_BX_151.field_0xc;
                    (uVar1 + local_4 * 4) = 0;
                } else {
                    uVar2 = &local_BX_151.field_0x16 + local_4 * 6;
                    process_struct_1008_50c2(
                        _local_1c,
                        (temp_7ffdca9fab4 + 8),
                        (temp_7ffdca9fab4 + 4),
                        param_1 & 0xffff0000 | uVar2,
                        param_2,
                    );
                    uVar1 = local_BX_151.field_0xc;
                    uVar7 = (uVar1 >> 0x10);
                    local_BX_294 = uVar1;
                    (local_BX_294 + local_SI_209) = uVar2;
                    (local_BX_294 + local_SI_209 + 2) = extraout_DX;
                }
                uVar1 = local_BX_151.field_0xc;
                pass1_1008_5134((uVar1 + local_4 * 4));
            }
            uVar1 = local_BX_151.field_0xc;
            pass1_1008_5236((uVar1 + local_4 * 4));
            pass1_1008_4480(
                param_2,
                (param_1 & 0xffff0000 | (&local_BX_151.field_0x16 + local_4 * 6)),
                in_struct_104_ptr,
            );
        }
        local_4 = &local_4.field_0x1;
    }
    if (local_BX_151.field_0x4 != 0) {
        pass1_1008_4480(
            param_2,
            (param_1 & 0xffff0000 | &local_BX_151.field_0x32),
            local_BX_151.field_0x38,
        );
    }
    return;
}

pub fn pass1_1020_8a5e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_8bae(param_1: *mut astruct_44) {
    param_1.ptr_a_lo = 0x8e92;
    (param_1 + 2) = 0x1020;
    pass1_1020_8556(param_1);
    return;
}

pub fn pass1_1020_6498(param_1: *mut u8, param_2: u16) {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 4) != 0) {
        uVar1 = (param_1 + 0x18 + param_2 * 4);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 10), (iVar2 + 8));
    }
    return 0;
}

pub fn pass1_1020_64d4(param_1: *mut u8, param_2: u16) {
    let mut local_ES_5: u16;
    let mut temp_5ff6edc30e: u32;

    local_ES_5 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 4) != 0) {
        temp_5ff6edc30e = (param_1 + 0x18 + param_2 * 4);
        return (temp_5ff6edc30e + 4);
    }
    return 0;
}

pub fn pass1_1020_61c4(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: *mut u16) {
    let mut uVar1: u16;
    let local_AX__1: *mut astruct_667;
    let mut local_DX_11: u16;
    let ppVar2: *mut pass1_struct_1;
    let local_string_1: *mut char;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f50cbac33: u32;

    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_string_1, 0x2f));
    uVar1 = (ppVar2 >> 0x10);
    pass1_1030_8308(
        _g_bool_1050_5748,
        (_g_bool_1050_5748 >> 0x10),
        param_1_00,
        param_2_00,
        (ppVar2 + 0x20),
    );
    unsafe { *param_2_00 = (ppVar2 + 0x1e) };
    return;
}

pub fn pass1_1020_5d56(param_1: *mut u32, param_2: u32) -> bool {
    let ppcVar1: fn();
    let mut unaff_SS: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = (param_2 + 0x2e);
    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (local_6 == 0x47) {
        pass1_1020_61c4(
            uVar2,
            uVar3,
            CONCAT22(unaff_SS, &local_c),
            CONCAT22(unaff_SS, &local_a),
        );
        if (local_a == 0) {}
        // goto LAB_1020_5d8b;
        if (local_c <= local_a) {
            return 1;
        }
    } else {
        if (local_6 != 0x6a) {
            return 0;
        }
        pass1_1020_61c4(
            uVar2,
            uVar3,
            CONCAT22(unaff_SS, &local_e),
            CONCAT22(unaff_SS, &local_12),
        );
        if (local_e <= local_12) {
            // LAB_1020_5d8b:
            unsafe {
                ppcVar1 = (*param_1 + 0x40);
                (**ppcVar1)();
            }
            return 1;
        }
    }
    pass1_1038_af40(_g_astruct_112_a, *(uVar2 + 8), 9);
    return 1;
}

pub fn call_draw_fn_1020_3bd6(in_struct_1: *mut astruct_657) {
    let mut u_var1: u32;
    let mut uVar2: u16;
    let local_struct_1: *mut astruct_657;
    let local_struct_1_hi: *mut astruct_657;
    let puVar3: *mut u8;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    draw_1020_3fa0(local_struct_1.field_0xf6);
    if (local_struct_1.field_0x100 == 0) {
        local_struct_1.field_0x100 = 1;
        uVar1 = local_struct_1.field_0xfa;
        if ((uVar1 + 0x56) == 0) {
            uVar2 = 5;
        } else {
            uVar2 = 8;
        }
        puVar3 = pass1_1038_af40(_g_astruct_112_a, local_struct_1.field_0x8, uVar2);
        local_struct_1.field_0x10e = puVar3;
        local_struct_1.field_0x110 = (puVar3 >> 0x10);
    }
    return;
}

pub fn pass1_1020_3c32(param_1: *mut astruct_658, param_2: u16, uparam_2_00: i32) {
    let mut u_var1: u32;
    let mut cVar2: u8;
    let mut uvar3: u16;

    if (param_2_00 == 0xf5) {
        uVar3 = 1;
        // LAB_1020_3c52:
        uVar1 = param_1.field_0xfa;
        pass1_1018_1b02(uVar1, (uVar1 >> 0x10), uVar3);
        return;
    }
    if ((param_2_00 < 0xf6) && (cVar2 = param_2_00, cVar2 != '\0')) {
        if (cVar2 == 0x1 || cVar2 == 0x2) {
            return;
        }
        if (cVar2 == -0xc) {
            uVar3 = 0;
            // goto LAB_1020_3c52;
        }
    }
    pass1_1020_3c32(param_1, param_2, param_2_00);
    return;
}

pub fn pass1_1020_3540(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let local_struct_2: *mut astruct_655;
    let struct_a: *mut astruct_199;
    let paVar1: *mut astruct_199;
    let local_struct_1: *mut astruct_654;
    let mut unaff_SS: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94(
        param_2_00,
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_4),
    );
    if (param_1_00 == 0) {
        local_c = 3;
        local_a._0_2_ = 0x42a6;
    } else {
        if (param_1_00 == 1) {
            local_c = 4;
            local_a._0_2_ = (s_SITEICON_1050_428d + 9);
        } else {
            if (param_1_00 != 2) {
                return;
            }
            local_c = 4;
            local_a._0_2_ = 0x42b2;
        }
    }
    local_struct_2 = (local_c << 2);
    paVar1 = struct_a;
    process_struct_1000_179c(local_struct_2, struct_a);
    local_12 = 0;
    while (local_12 < local_c) {
        local_struct_1 = (local_12 * 4);
        (local_struct_1 + local_struct_2) = (local_struct_1 + local_a) + local_4;
        (local_struct_2 + local_struct_1 + 2) = (local_struct_1 + local_a + 2) + local_6;
        local_12 = local_12 + 1;
    }
    return;
}

pub fn pass1_1020_2a94(param_1: *mut void, param_2: u32) {
    pass1_1018_1662((param_1 + 0xf2), param_2);
    return;
}

pub fn pass1_1020_2936(param_1: u16, param_2: u32) -> u8 {
    let mut uVar1: u16;

    uVar1 = return_1_1020_79ae();
    return uVar1;
}

pub fn pass1_1020_294a(in_struct_1: *mut astruct_651, param_2: u32, param_3: u16) {
    let mut uVar1: u16;
    let local_struct_1: *mut astruct_651;
    let mut unaff_BP: u16;
    let local_struct_1_hi: *mut astruct_651;
    let ppVar2: *mut pass1_struct_1;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0xfc = param_3;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, param_3));
    uVar1 = (ppVar2 >> 0x10);
    local_struct_1.field_0xf2 = ppVar2;
    &local_struct_1.field_0xf4 = uVar1;
    local_struct_1.field_0xe0 = local_struct_1.field_0xf2;
    local_struct_1.field_0xe2 = uVar1;
    pass1_1018_0902(&local_struct_1.field_0xf2, param_2);
    return;
}

pub fn process_struct_1020_26e6(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376 {
    process_struct_1020_2594(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}
